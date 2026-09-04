//! The Dorc case renderer and compiled-edit applier (`282` §5 · §13), implemented against a mutable
//! owned-catalog mirror ([`dorc_aid::catalog::OwnedEntry`]).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Write;

use dorc_aid::RenderCtx;
use dorc_aid::arrangement::{OwnedArrangement, owned_arrangements};
use dorc_aid::catalog::{HelpRegister, OwnedEntry, owned_catalog, parse_template};
use dorc_aid::diag::{Diag, render_cli_parts};
use dorc_aid::prose::{Mint, ProseTier};
use dorc_core::Interner;
use errorloom::{
    Case, CaseRenderer, EditableFragment, EditableRender, RenderComponent, ReplayCommand,
    ReplayContext, ReplayDriver, ReplayEmission, ReplayInput, ReplayInputTarget, ReplayResult,
    ReplayStatus, RunEnv, RunError, drive_case, drive_case_with_inputs,
};

use crate::invocation::{Breadth, Target, Verb};
use crate::session_env::SessionEnv;
use crate::session_grammar::{ExportOp, OutputRouting, SessionHead, SessionReading};
use crate::usage::{self, PROGRAM, Reading};
use crate::{
    DorcSectionEdit, SectionKey, SectionVariableId, TemplateVariableName, to_editable_render,
};

/// Exact current values by editable section and semantic variable name.
pub type SectionVariables = BTreeMap<SectionKey, BTreeMap<TemplateVariableName, String>>;

/// A case render ready for generic editable transport.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DorcEditableBaseline {
    render: EditableRender<SectionKey, SectionVariableId>,
    variables: SectionVariables,
    all_variables: BTreeMap<TemplateVariableName, String>,
}

impl DorcEditableBaseline {
    /// The editable diagnostic render.
    #[must_use]
    pub fn render(&self) -> &EditableRender<SectionKey, SectionVariableId> {
        &self.render
    }

    /// Exact current variable values keyed by editable section.
    #[must_use]
    pub fn variables(&self) -> &SectionVariables {
        &self.variables
    }

    /// Ordinary typed payload values, including values not currently rendered.
    #[must_use]
    pub fn all_variables(&self) -> &BTreeMap<TemplateVariableName, String> {
        &self.all_variables
    }

    pub(crate) fn section_baseline(&self, section: &SectionKey) -> Option<Self> {
        let component = self.render.components().iter().find(|component| {
            matches!(component, RenderComponent::EditableSection(candidate) if candidate.id() == section)
        })?;
        Some(DorcEditableBaseline {
            render: EditableRender::new(vec![component.clone()]),
            variables: self
                .variables
                .get(section)
                .map(|values| BTreeMap::from([(section.clone(), values.clone())]))
                .unwrap_or_default(),
            all_variables: self.all_variables.clone(),
        })
    }

    /// Rendered editable variables in deterministic first-use order.
    #[must_use]
    pub fn used_variables(&self) -> Vec<(TemplateVariableName, String)> {
        let mut used = Vec::new();
        for component in self.render.components() {
            let RenderComponent::EditableSection(section) = component else {
                continue;
            };
            for fragment in section.fragments() {
                let EditableFragment::Variable { id, rendered } = fragment else {
                    continue;
                };
                if !used.iter().any(|(name, _)| name == &id.name) {
                    used.push((id.name.clone(), rendered.clone()));
                }
            }
        }
        used
    }
}

/// The Dorc case renderer and compiled-edit applier.
#[derive(Debug)]
pub struct DorcConsumer {
    mirror: Vec<OwnedEntry>,
    arrangements: Vec<OwnedArrangement>,
    mint: Mint,
    demoted: Vec<String>,
}

/// Why applying a compiled section to the in-memory mirror refused.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DorcApplyRefusal {
    /// The selected diagnostic code is absent from the mirror.
    MissingCode(String),
    /// The selected arrangement slug is absent from the registry.
    MissingArrangement(String),
    /// The selected section is neither a catalog prose field nor an arrangement line.
    IllegalField(&'static str),
    /// Two sections of ONE render edited the SAME registry entry to different words.
    ///
    /// Repeated chrome is one entry rendered twice (`28H` ruling 3), so either span is a complete
    /// rendering of it and an edit to either rewrites the whole entry. Two DIFFERENT edits are a
    /// contradiction, and applying them in order would silently keep the last: the author would
    /// see one of their two rewrites vanish with nothing said. Refusing is the only honest answer,
    /// because nothing here can know which one they meant.
    ArrangementEntryEditedTwice {
        /// The row both edits landed on.
        slug: String,
        /// The words the first section compiled to.
        first: Vec<String>,
        /// The words the second compiled to.
        second: Vec<String>,
    },
    /// A chrome-line edit moved, dropped or duplicated a value the render placed.
    ///
    /// The narrow half of what the old blanket sequence refusal covered: an edit may rephrase
    /// every word around a value, but the values are the render's own account of the world and
    /// their ORDER is the only thing that says which word goes where. `expected` and `found` are
    /// the positional variable sequences, so the refusal can say what moved.
    ArrangementValueSequenceChanged {
        /// The row the edit landed on.
        slug: String,
        /// The sequence the render stamped: `v0, v1, …`.
        expected: Vec<String>,
        /// The sequence the edit compiled to.
        found: Vec<String>,
        /// The entry's own stored words — the half of the line that IS the author's, carried so
        /// the refusal can point at it instead of leaving them to guess which word was computed.
        editable_words: Vec<String>,
    },
}

/// Why minting a prose register refused.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SeedRefusal {
    /// No catalog row carries that slug.
    MissingCode(String),
    /// The register is already there — unwritten or written.
    AlreadyPresent(String),
}

impl Default for DorcConsumer {
    fn default() -> Self {
        Self::new()
    }
}

impl DorcConsumer {
    /// A consumer seeded from the compiled-in catalog and arrangement registry (the
    /// carry-forward starting state for both tables).
    #[must_use]
    pub fn new() -> Self {
        DorcConsumer {
            mirror: owned_catalog(),
            arrangements: owned_arrangements(),
            mint: Mint::Slop,
            demoted: Vec::new(),
        }
    }

    /// The same consumer minting a different tier — `dorc-loom publish --human`'s one effect.
    #[must_use]
    pub fn minting(self, mint: Mint) -> Self {
        DorcConsumer { mint, ..self }
    }

    /// The slugs whose human-written register this consumer's edits re-marked as slop, in
    /// application order — what the CLI turns into a notice or a refusal (`spec-demotion-branches`).
    #[must_use]
    pub fn demoted(&self) -> &[String] {
        &self.demoted
    }

    /// The current catalog mirror (test/inspection surface).
    #[must_use]
    pub fn mirror(&self) -> &[OwnedEntry] {
        &self.mirror
    }

    /// The current arrangement mirror (test/inspection surface).
    #[must_use]
    pub fn arrangements(&self) -> &[OwnedArrangement] {
        &self.arrangements
    }

    /// Overwrite a code's message in the mirror (models a raw catalog hand-edit for the fixpoint gate).
    pub fn set_message(&mut self, slug: &str, message: Option<ProseTier<String>>) {
        if let Some(e) = self.mirror.iter_mut().find(|e| e.slug == slug) {
            e.message = message;
        }
    }

    /// Mint a code's HELP register in the mirror, unwritten (`28L:rul-help-affordance-is-scaffold`).
    ///
    /// The register, never its prose: after promotion the render grows a
    /// `= help: [unwritten: <slug>.help]` line and the ORDINARY transcript loop fills it. That is
    /// why this is an explicit act rather than something an added transcript line could imply —
    /// inferring the register from the shape of a typed line is the byte-shape re-detection the
    /// whole surface is built to avoid (`28L:rul-editability-is-stamped-never-re-derived`).
    ///
    /// # Errors
    /// Returns [`SeedRefusal`] when no row carries the slug, or the register is already there.
    pub fn seed_help_register(&mut self, slug: &str) -> Result<(), SeedRefusal> {
        let entry = self
            .mirror
            .iter_mut()
            .find(|entry| entry.slug == slug)
            .ok_or_else(|| SeedRefusal::MissingCode(slug.to_owned()))?;
        match entry.help {
            HelpRegister::Absent => {
                entry.help = HelpRegister::Unwritten;
                Ok(())
            }
            HelpRegister::Unwritten | HelpRegister::Written(_) => {
                Err(SeedRefusal::AlreadyPresent(slug.to_owned()))
            }
        }
    }

    /// Overwrite an arrangement entry's words in the mirror (the [`Self::set_message`] twin: it
    /// models a raw registry hand-edit, and stages the word-sequence state nothing authors yet).
    pub fn set_arrangement_words(&mut self, slug: &str, words: Option<ProseTier<Vec<String>>>) {
        if let Some(entry) = self.arrangements.iter_mut().find(|e| e.slug == slug) {
            entry.words = words;
        }
    }

    /// Apply one accepted compiled section to the in-memory catalog mirror.
    ///
    /// # Errors
    /// Returns [`DorcApplyRefusal`] for an absent code or non-prose field.
    pub fn apply_section_edit(&mut self, edit: &DorcSectionEdit) -> Result<(), DorcApplyRefusal> {
        self.apply_compiled_section(edit.section(), edit.compiled())
    }

    /// Apply every compiled section of a preview to the mirror (the publish-publish edited mirror,
    /// `28A` §4): the edited templates the regenerated lock and affected cases are computed from.
    ///
    /// # Errors
    /// Returns [`DorcApplyRefusal`] for an absent code or non-prose field.
    pub fn apply_preview(
        &mut self,
        preview: &crate::CompilePreview,
    ) -> Result<(), DorcApplyRefusal> {
        self.refuse_divergent_entry_edits(preview)?;
        for section in preview.sections() {
            self.apply_compiled_section(&section.section, &section.compiled)?;
        }
        Ok(())
    }

    /// Refuse BEFORE the first write when two sections of one preview land on ONE registry entry
    /// with different words (see [`DorcApplyRefusal::ArrangementEntryEditedTwice`]).
    ///
    /// A whole pre-pass rather than a check inside the applier, because the mirror must be
    /// untouched when it refuses: a partially-applied preview is a mirror nobody asked for. Two
    /// sections compiling to the SAME words are the ordinary case — a human rewriting repeated
    /// chrome consistently — and pass.
    fn refuse_divergent_entry_edits(
        &self,
        preview: &crate::CompilePreview,
    ) -> Result<(), DorcApplyRefusal> {
        let mut landed: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        for section in preview.sections() {
            let key = &section.section;
            let (Some(words), Some(index)) = (
                stored_words(key.field, &section.compiled),
                arrangement_index(&self.arrangements, &key.owner, key.instance),
            ) else {
                continue;
            };
            match landed.entry(index) {
                std::collections::btree_map::Entry::Vacant(slot) => {
                    slot.insert(words);
                }
                std::collections::btree_map::Entry::Occupied(seen) if *seen.get() != words => {
                    return Err(DorcApplyRefusal::ArrangementEntryEditedTwice {
                        slug: key.owner.clone(),
                        first: seen.get().clone(),
                        second: words,
                    });
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }
        Ok(())
    }

    fn apply_compiled_section(
        &mut self,
        key: &SectionKey,
        compiled: &crate::CompiledSection,
    ) -> Result<(), DorcApplyRefusal> {
        if key.field == crate::ARRANGEMENT_LINE_FIELD {
            return self.apply_arrangement_line_edit(key, compiled);
        }
        if !matches!(key.field, "message" | "help") {
            return Err(DorcApplyRefusal::IllegalField(key.field));
        }
        let entry = self
            .mirror
            .iter_mut()
            .find(|entry| entry.slug == key.owner)
            .ok_or_else(|| DorcApplyRefusal::MissingCode(key.owner.clone()))?;
        let template = compiled
            .fragments()
            .iter()
            .map(|fragment| match fragment {
                crate::CompiledFragment::Text(text) => text.clone(),
                crate::CompiledFragment::Variable(name) => format!("{{{{{}}}}}", name.0),
            })
            .collect();
        let mint = self.mint;
        let demoted = if key.field == "message" {
            let demoted = mint.demotes(entry.message.as_ref());
            entry.message = Some(mint.tier(template));
            demoted
        } else {
            let demoted = mint.demotes(entry.help.written());
            entry.help = HelpRegister::Written(mint.tier(template));
            demoted
        };
        entry.params = entry
            .message
            .iter()
            .chain(entry.help.written())
            .flat_map(|tier| parse_template(tier.text()).unwrap_or_default())
            .filter_map(|part| match part {
                dorc_aid::catalog::TemplatePart::Hole(name) => Some(name),
                dorc_aid::catalog::TemplatePart::Literal(_) => None,
            })
            .fold(Vec::new(), |mut params, name| {
                if !params.contains(&name) {
                    params.push(name);
                }
                params
            });
        if demoted {
            self.demoted.push(key.owner.clone());
        }
        Ok(())
    }

    /// Apply one compiled chrome-LINE section to the registry mirror — the whole of "a
    /// value-interleaved chrome line edits back from a transcript exactly as catalog prose
    /// does" (`_w4-map-DRAFT:prop-one-section-many-fragments`).
    ///
    /// The compiled fragment series IS the re-split: a `Text` fragment is a word, a `Variable`
    /// fragment is a boundary. Two things are checked rather than guessed. The variable sequence
    /// must be the one the render stamped, in order — reorder, drop or duplicate means the edit
    /// moved a value, which no rephrasing does. And a WRITTEN entry's arity must survive, because
    /// its seat interleaves a fixed number of values and a line that no longer accepts them
    /// renders as the unwritten placeholder instead.
    ///
    /// Whitespace runs collapse to one space on the way in: a laid-out line's inter-word
    /// whitespace is the RENDERER's — a wrap it chose at this width — so storing it would freeze
    /// one width into the entry (`282` §3's read-in normalization, and why the page path above is
    /// a separate function rather than a flag).
    fn apply_arrangement_line_edit(
        &mut self,
        key: &SectionKey,
        compiled: &crate::CompiledSection,
    ) -> Result<(), DorcApplyRefusal> {
        let words = line_words(compiled);
        let found: Vec<String> = compiled
            .fragments()
            .iter()
            .filter_map(|fragment| match fragment {
                crate::CompiledFragment::Variable(name) => Some(name.0.clone()),
                crate::CompiledFragment::Text(_) => None,
            })
            .collect();
        let mint = self.mint;
        let entry = self.arrangement_entry(key)?;
        let stored = entry.words.as_ref().map(ProseTier::text);
        let arity = stored.map_or(words.len(), Vec::len);
        let expected: Vec<String> = (0..arity.saturating_sub(1))
            .map(|index| crate::arrangement_variable(index).0)
            .collect();
        if found != expected {
            return Err(DorcApplyRefusal::ArrangementValueSequenceChanged {
                slug: key.owner.clone(),
                expected,
                found,
                editable_words: stored.cloned().unwrap_or_default(),
            });
        }
        let demoted = mint.demotes(entry.words.as_ref());
        entry.words = Some(mint.tier(words));
        if demoted {
            self.demoted.push(key.owner.clone());
        }
        Ok(())
    }

    /// The registry entry the RENDER read: the occurrence's own entry when it has one, else the
    /// whole-slug entry — so an edit to any one occurrence of shared chrome updates the shared
    /// words, which is the truth of a shared entry.
    fn arrangement_entry(
        &mut self,
        key: &SectionKey,
    ) -> Result<&mut OwnedArrangement, DorcApplyRefusal> {
        let index = arrangement_index(&self.arrangements, &key.owner, key.instance)
            .ok_or_else(|| DorcApplyRefusal::MissingArrangement(key.owner.clone()))?;
        self.arrangements
            .get_mut(index)
            .ok_or_else(|| DorcApplyRefusal::MissingArrangement(key.owner.clone()))
    }

    /// Re-render a case corpus from the current in-memory mirror.
    ///
    /// # Errors
    /// Returns a case-world materialization refusal.
    pub fn render_cases(&self, cases: &[Case]) -> Result<Vec<String>, String> {
        cases.iter().map(|case| self.render_case(case)).collect()
    }

    /// This consumer's own EDITABLE tables, at the corpus's canonical width — the seat that makes
    /// an edited row render before anyone rebuilds
    /// (`28H:finding-why-render-reads-the-const-not-the-mirror`).
    ///
    /// BOTH mirrors, always: a render that read the edited catalog and the compiled-in chrome would
    /// show an author half of their own edit, which is exactly the failure the one-context rule
    /// exists to make unrepresentable (`28L:rul-render-context-struct`).
    pub(crate) fn render_ctx(&self) -> RenderCtx<'_> {
        RenderCtx::new(&self.mirror, &self.arrangements)
    }

    /// One INVOCATION error, whole — prefix, diagnostic, usage synopsis — through the binary's own
    /// seat, so a case shows the bytes an admin really sees rather than the diagnostic alone.
    ///
    /// Which binary it is comes from the replay's own first word, never from the diagnostic: the
    /// shim prints a different framing and no synopsis.
    fn invocation_parts(&self, diag: &Diag, binary: &str) -> dorc_aid::tagged::RenderParts {
        let ctx = self.render_ctx();
        let interner = Interner::default();
        if binary == "dorc-sh" {
            return dorc_cli::shim_error_parts(&ctx, diag, &interner);
        }
        dorc_cli::invocation_error_parts(&ctx, diag, &interner)
    }

    /// The editable baseline of a case's FIRST replay — what `dorc-loom vars` reports.
    ///
    /// It drives the case exactly as `publish` does rather than re-deriving a world of its own
    /// (`_loom-final-map` §2c): a second derivation answered only for the plain diagnostic shape, so
    /// a receipt, lint, or invocation-error case got a different render — or none — from the one an
    /// edit actually compiles against, and an inventory that disagrees with the compiler is worse
    /// than no inventory.
    ///
    /// # Errors
    /// Returns the replay refusal, names the case whose first replay carries no editable prose, or
    /// declines a case whose bytes this tool does not produce.
    pub fn editable_baseline(&self, case: &Case) -> Result<DorcEditableBaseline, String> {
        // The generation lag, stated before the driver can only shrug about it: a case naming a
        // slug with no committed row renders nothing, and the honest answer names the repair.
        if let Some(slug) = case.frontmatter().scalar("arrangement") {
            self.require_arrangement_row(slug)?;
        }
        // Declining the case's own inventory block is what makes the block legal to write down.
        let driver = DorcReplayDriver::new(self, case).without_self_reference();
        let render = drive_case(case, &RunEnv::new(), |command, context| {
            Ok(driver
                .drive(command, context)
                .unwrap_or_else(|| ReplayResult::bytes(String::new())))
        })
        .map_err(|error: RunError| error.to_string())?
        .into_iter()
        .find_map(|result| result.editable_render().cloned())
        .ok_or_else(|| {
            "no replay of this case renders editable prose; `vars` reports the render an edit \
             compiles against, so a case whose replays are all bytes-only has no inventory"
                .to_owned()
        })?;
        self.baseline_from_render(case, render)
    }

    fn require_arrangement_row(&self, slug: &str) -> Result<(), String> {
        self.arrangements
            .iter()
            .any(|entry| entry.slug == slug)
            .then_some(())
            .ok_or_else(|| {
                format!(
                    "arrangement `{slug}` has no registry row yet -- publish the case, then rebuild"
                )
            })
    }

    /// The `dorc-loom vars` inventory for one case, as bytes.
    ///
    /// The ONE derivation the binary's verb, the driver, and the re-render seat all go through, so
    /// a committed inventory block is a generator fixpoint rather than a second opinion
    /// (`282:rul-used-inventory-is-committed`).
    ///
    /// The case NAMES ITSELF, from its own declared slug — never from how the caller reached it.
    /// A terminal holds a path and a replay line holds nothing, and labelling by what each happened
    /// to have made the same case print two different headers depending on which seat asked
    /// (`30C` item 1: only `--this` may behave differently inside a loom).
    ///
    /// An EMPTY string means the case has no variables at this breadth. The header rides the
    /// values, so a variable-less case contributes no row at either seat rather than a `case:` line
    /// with nothing under it (`30C` item 4).
    ///
    /// # Errors
    /// Returns the baseline refusal for a case whose replays render no editable prose.
    pub fn vars_inventory(&self, target: &Case, breadth: Breadth) -> Result<String, String> {
        let baseline = self.editable_baseline(target)?;
        let values = match breadth {
            Breadth::Used => baseline.used_variables(),
            Breadth::All => baseline
                .all_variables()
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
        };
        if values.is_empty() {
            return Ok(String::new());
        }
        let mut output = format!("case: {}\n", case_label(target));
        for (name, value) in values {
            let _ = writeln!(output, "{{{{{}}}}} = {value:?}", name.0);
        }
        Ok(output)
    }

    /// The `dorc-loom sections` listing for one case, as bytes — the same one-derivation shape as
    /// [`Self::vars_inventory`], for the other inventory.
    ///
    /// It drives with [`SelfReference::Forbidden`] for the reason the baseline does: a listing of
    /// every replay's sections is itself derived from driving every replay, so a `--this sections`
    /// block that answered here would ask its own question forever.
    ///
    /// # Errors
    /// Returns the replay refusal.
    pub fn sections_inventory(&self, case: &Case) -> Result<String, String> {
        let driver = DorcReplayDriver::new(self, case).without_self_reference();
        let results = drive_case(case, &RunEnv::new(), |command, context| {
            Ok(driver
                .drive(command, context)
                .unwrap_or_else(|| ReplayResult::bytes(String::new())))
        })
        .map_err(|error: RunError| error.to_string())?;
        let mut output = format!("case: {}\n", case_label(case));
        for (index, result) in results.iter().enumerate() {
            let Some(render) = result.editable_render() else {
                let _ = writeln!(output, "replay {index}: bytes-only");
                continue;
            };
            let _ = writeln!(output, "replay {index}:");
            for component in render.components() {
                write_component(&mut output, component);
            }
        }
        Ok(output)
    }

    /// The `dorc-loom …` arm of both replay chains.
    ///
    /// Both chains route here and both go through [`crate::usage`] rather than matching token
    /// slices, so the shapes a transcript may carry are exactly the shapes a terminal accepts, and
    /// a page or a refusal a case teaches is the one a reader will actually meet
    /// (`30C:rul-this-is-a-global-flag`). `source_of` is how the chain finds a NAMED case's bytes:
    /// a materialized input on the driven chain, a section on the re-render chain.
    ///
    /// `--this` is the ONE spelling that may answer differently here than at a terminal (`30C`
    /// item 1); `-C` is the mirror of that — a replay line has no world of its own to resolve
    /// against, so an invocation carrying one is declined rather than half-honoured.
    fn loom_replay(
        &self,
        case: &Case,
        tokens: &[&str],
        self_reference: SelfReference,
        source_of: &dyn Fn(&str) -> Option<String>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        let invocation = match usage::read(tokens.get(1..)?) {
            Reading::Help(page) => return Some(ReplayResult::bytes(format!("{page}\n"))),
            Reading::Refused(refusal) => {
                return Some(ReplayResult::bytes(format!("{PROGRAM}: {refusal}\n")));
            }
            Reading::Runs(invocation) => *invocation,
        };
        if invocation.root.is_some() {
            return None;
        }
        let wanted = match &invocation.verb {
            Verb::Vars(args) => Inventory::Vars(args.breadth()),
            Verb::Sections(_) => Inventory::Sections,
            Verb::Defect => {
                if invocation.target().ok()? != Target::This {
                    return None;
                }
                return self.defect_replay(case);
            }
            _ => return None,
        };
        match invocation.target().ok()? {
            Target::This => {
                if self_reference == SelfReference::Forbidden {
                    return None;
                }
                self_slug(case)?;
                self.inventory(wanted, case).map(ReplayResult::bytes)
            }
            Target::Named([one]) if case_relative_path(one) => {
                let target = Case::parse(&source_of(one)?).ok()?;
                self.inventory(wanted, &target).map(ReplayResult::bytes)
            }
            Target::Named(_) => None,
        }
    }

    fn defect_replay(&self, case: &Case) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        let scenario = crate::defect::DefectScenario::from_slug(self_slug(case)?)?;
        let diagnostic = scenario.diagnostic();
        // The seat comes from the REGISTRY: a bypass render has no invocation whose shape one
        // could be picked from, and a transcript at the wrong seat shows a render the product
        // never emits.
        let (parts, status) = match scenario.seat() {
            crate::defect::DefectSeat::Staged(stage) => {
                let event = dorc_cli::engine::diagnostic_event(
                    &self.render_ctx(),
                    stage,
                    &diagnostic,
                    "",
                    "",
                );
                (event.tagged_parts()?.clone(), ReplayStatus::SUCCESS)
            }
            crate::defect::DefectSeat::Invocation { status } => (
                self.invocation_parts(&diagnostic, "dorc"),
                ReplayStatus::new(status),
            ),
        };
        Some(ReplayResult::emitted(
            status,
            vec![ReplayEmission::editable(
                errorloom::ReplayChannel::Stderr,
                to_editable_render(&parts),
            )],
        ))
    }

    fn inventory(&self, wanted: Inventory, case: &Case) -> Option<String> {
        match wanted {
            Inventory::Vars(breadth) => self.vars_inventory(case, breadth).ok(),
            Inventory::Sections => self.sections_inventory(case).ok(),
        }
    }

    /// Drive only direct invocations whose replay inputs and rendering are exact.
    #[must_use]
    pub fn replay(
        &self,
        case: &Case,
        command: &str,
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        let command = ReplayCommand::parse(command).ok()?;
        self.replay_within(
            case,
            &command,
            context,
            SelfReference::Allowed,
            &mut LoomSession::new(),
        )
    }

    fn replay_within(
        &self,
        case: &Case,
        command: &ReplayCommand,
        context: &ReplayContext<'_>,
        self_reference: SelfReference,
        session: &mut LoomSession,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        let reading = SessionReading::read(command.original());
        if reading.disagrees_with(
            command.argv(),
            command.input(),
            command.stdout_is_terminal(),
        ) {
            return None;
        }
        let SessionReading::Line(line) = reading else {
            return None;
        };
        let output = line.output;
        match line.head {
            SessionHead::Export(ExportOp::Assign { name, value }) => {
                session.env.export_assign(&name, &value);
                return Some(ReplayResult::bytes(String::new()));
            }
            SessionHead::Export(ExportOp::Mark { name }) => {
                session.env.export_mark(&name);
                return Some(ReplayResult::bytes(String::new()));
            }
            SessionHead::Cd(target) => return replay_cd(session, &target),
            // Only a `dorc` invocation ticks the clock (`30Xa:Checkpoint C3`); `session.seams()` below reads this invocation's.
            SessionHead::Invocation => session.env.inject_invocation_clock(),
            SessionHead::EchoStatus | SessionHead::Cat(_) => {}
        }
        let tokens: Vec<&str> = command.argv().iter().map(String::as_str).collect();
        if is_help_case(case, &tokens) {
            let parts = dorc_cli::help_parts(&self.render_ctx());
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if tokens.first() == Some(&LOOM_COMMAND) {
            return self.loom_replay(case, &tokens, self_reference, &|target| {
                context.materialized_input(target).map(str::to_owned)
            });
        }
        if tokens.as_slice() == ["dorc", "lint", "--list-sources"] {
            let parts = dorc_cli::lint_sources_parts(&self.render_ctx());
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if tokens.first() == Some(&"dorc") {
            return self.replay_dorc(case, command, context, session, output);
        }
        if tokens.first() == Some(&"dorc-sh") {
            return self.replay_dorc_sh(case, &tokens, context);
        }
        None
    }

    fn replay_dorc(
        &self,
        case: &Case,
        command: &ReplayCommand,
        context: &ReplayContext<'_>,
        session: &mut LoomSession,
        output: OutputRouting,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        self.replay_dorc_outcome(case, command, context, session, output)
            .map(|replay| replay.result)
    }

    fn replay_dorc_outcome(
        &self,
        case: &Case,
        command: &ReplayCommand,
        context: &ReplayContext<'_>,
        session: &mut LoomSession,
        output: OutputRouting,
    ) -> Option<DorcEngineReplay> {
        let invocation_argv = command.argv().get(1..)?.to_vec();
        let invocation = match dorc_cli::parse_args_from(invocation_argv) {
            Ok(invocation) => invocation,
            Err(diagnostic) => return self.invocation_diagnostic(case, diagnostic, "dorc"),
        };
        match invocation {
            dorc_cli::Invocation::Help => {
                let words: Vec<&str> = command.argv().iter().map(String::as_str).collect();
                is_help_case(case, &words).then(|| {
                    DorcEngineReplay::without_diagnostic(ReplayResult::editable(
                        to_editable_render(&dorc_cli::help_parts(&self.render_ctx())),
                    ))
                })
            }
            dorc_cli::Invocation::Version => Some(DorcEngineReplay::without_diagnostic(
                ReplayResult::bytes("dorc 0.0.0\n".to_owned()),
            )),
            dorc_cli::Invocation::Lint(args) => self.run_lint(case, &args, context),
            dorc_cli::Invocation::Analyze(analysis_args) => {
                if analysis_args.mode == dorc_cli::Mode::Apply && analysis_args.host.is_some() {
                    self.run_apply(case, &analysis_args, context, session, output)
                } else if analysis_args.reads_the_receipt() {
                    self.run_receipt_store_why(case, &analysis_args, session, output)
                } else {
                    self.run_engine(case, &analysis_args, command, context, session, output)
                }
            }
            dorc_cli::Invocation::Strip(_) => None,
        }
    }

    fn replay_dorc_sh(
        &self,
        case: &Case,
        words: &[&str],
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        self.replay_dorc_sh_outcome(case, words, context)
            .map(|replay| replay.result)
    }

    fn replay_dorc_sh_outcome(
        &self,
        case: &Case,
        words: &[&str],
        context: &ReplayContext<'_>,
    ) -> Option<DorcEngineReplay> {
        let fault = crate::edge_fault::EdgeFault::from_case(case).ok().flatten();
        let diagnostic = match words.get(1..) {
            Some([]) => dorc_cli::shim_usage_error(),
            Some([path]) => {
                if let Some(failure) = fault.as_ref().and_then(|fault| fault.read_failure(path)) {
                    dorc_cli::shim_script_read_error(path, &failure.error())
                } else if context.read_file(path).is_none() {
                    return None;
                } else if let Some(crate::edge_fault::EdgeFault::ShimExec(failure)) = &fault {
                    dorc_cli::shim_exec_error(&failure.error())
                } else {
                    return None;
                }
            }
            _ => return None,
        };
        self.invocation_diagnostic(case, diagnostic, "dorc-sh")
    }

    fn invocation_diagnostic(
        &self,
        case: &Case,
        diagnostic: Diag,
        command: &str,
    ) -> Option<DorcEngineReplay> {
        if case.frontmatter().scalar("code") != Some(diagnostic.code.slug()) {
            return None;
        }
        let parts = self.invocation_parts(&diagnostic, command);
        Some(DorcEngineReplay {
            result: ReplayResult::emitted(
                ReplayStatus::new(2),
                vec![ReplayEmission::editable(
                    errorloom::ReplayChannel::Stderr,
                    to_editable_render(&parts),
                )],
            ),
            diagnostics: vec![diagnostic],
        })
    }

    /// The in-process `dorc apply --host` route (`30X` §11): the REAL `ship_consented_apply` over the
    /// session's model store, its transport answered by the scripted column. `run_remote_apply`'s
    /// hand-rolled `edge-fault` diagnostic table is GONE — its rows are now what the real apply route
    /// renders when the same fault is injected (`rul-strawman-formats-no-compat`; rip, never adapt).
    fn run_apply(
        &self,
        case: &Case,
        args: &dorc_cli::Args,
        context: &ReplayContext<'_>,
        session: &mut LoomSession,
        output: OutputRouting,
    ) -> Option<DorcEngineReplay> {
        let host = args.host.as_deref()?;
        let plan = args.plan.as_deref()?;
        let mut artifact = context.read_file(plan)?.into_bytes();
        let fault = crate::edge_fault::EdgeFault::from_case(case).ok().flatten();

        // A carriage return is a nonportable byte the LF-only case cannot carry, so the declaration
        // stands in for it: inject the CR and the real route's own check fires exactly as production.
        if let Some(crate::edge_fault::EdgeFault::Transport(
            crate::edge_fault::TransportFailure::Crlf { line },
        )) = &fault
        {
            artifact = inject_carriage_return(&artifact, *line);
        }

        let outcome_unwritable = matches!(fault, Some(crate::edge_fault::EdgeFault::ApplyOutcome));
        // The scripted session: the transport-fault half, the outcome-fault's own success transport,
        // or the `hosts/<name>/…` success half; a `--host` apply declaring none is a typed decline.
        let script = scripted_session(case, host, fault.as_ref(), outcome_unwritable)?;
        let seams = session.scripted_seams(script)?;
        let edge = session.edge.clone();

        let mut sink = LoomOutputSink {
            ctx: self.render_ctx(),
            actions: Vec::new(),
        };
        let routing = output;
        let result = if outcome_unwritable {
            drive_apply_outcome_unwritable(&edge, &seams, &mut sink, args, &artifact, host)
        } else {
            dorc_cli::compose::dispatch_and_report_apply(
                session.store.as_mut(),
                &edge,
                &seams,
                &mut sink,
                args,
                &artifact,
                host,
            )
        };
        match result {
            Ok(status) => Some(dorc_engine_replay(status, sink.actions, &routing)),
            Err(diagnostic) => self.invocation_diagnostic(case, diagnostic, "dorc"),
        }
    }

    fn run_lint(
        &self,
        case: &Case,
        args: &dorc_cli::LintArgs,
        context: &ReplayContext<'_>,
    ) -> Option<DorcEngineReplay> {
        if !args.oracle_dirs.is_empty() || !args.oracles.is_empty() {
            return None;
        }
        let fault = crate::edge_fault::EdgeFault::from_case(case).ok().flatten();
        let inputs = args
            .files
            .iter()
            .map(|path| {
                context.read_file(path).map(|source| dorc_lint::LintInput {
                    path: path.clone(),
                    src: source,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        let runner = LoomToolRunner {
            fault: fault.as_ref(),
        };
        let only = (!args.sources.is_empty()).then_some(args.sources.as_slice());
        let report = if let [input] = inputs.as_slice()
            && !args.tools_enabled
            && only.is_none()
        {
            dorc_lint::lint_materialized_source_with_runner(
                input.path.clone(),
                input.src.clone(),
                dorc_lint::SourcePolicy {
                    tools_enabled: false,
                },
                &runner,
            )
            .report()
            .clone()
        } else {
            dorc_lint::lint(
                &inputs,
                &[],
                dorc_lint::LintOptions {
                    tools_enabled: args.tools_enabled,
                },
                &runner,
                only,
            )
        };
        let operational = dorc_cli::lint_operational_diagnostic(args, inputs.len(), &report);
        let mut diagnostics = report
            .findings
            .iter()
            .filter_map(|finding| {
                finding
                    .provenance
                    .as_ref()
                    .map(|provenance| provenance.diag.clone())
            })
            .collect::<Vec<_>>();
        let mut emissions = Vec::new();
        if !inputs.is_empty() {
            emissions.push(match args.format {
                dorc_cli::LintFormat::Jsonl => ReplayEmission::bytes(
                    errorloom::ReplayChannel::Stdout,
                    dorc_lint::render::render_jsonl(&report),
                ),
                dorc_cli::LintFormat::Human => ReplayEmission::editable(
                    errorloom::ReplayChannel::Stdout,
                    to_editable_render(&dorc_lint::render::render_human_parts_at(
                        &self.render_ctx(),
                        &report,
                        args.verbosity,
                    )),
                ),
            });
        }
        let status = if let Some(diagnostic) = operational {
            if case.frontmatter().scalar("code") != Some(diagnostic.code.slug()) {
                return None;
            }
            let body = to_editable_render(&render_cli_parts(
                &self.render_ctx(),
                &diagnostic,
                "",
                "",
                &Interner::default(),
            ));
            let mut components = vec![RenderComponent::Structure("dorc: lint: ".to_owned())];
            components.extend(body.components().iter().cloned());
            emissions.push(ReplayEmission::editable(
                errorloom::ReplayChannel::Stderr,
                EditableRender::new(components),
            ));
            diagnostics.push(diagnostic);
            3
        } else {
            i32::from(report.count_at_or_above(args.fail_on) > 0)
        };
        Some(DorcEngineReplay {
            result: ReplayResult::emitted(ReplayStatus::new(status), emissions),
            diagnostics,
        })
    }

    /// Answer a `dorc why` that reads the receipt store, from the SESSION's own store — production's
    /// own read core and render seat over the deterministic model store
    /// (`dorc-replay-is-production-semantics`; `30Xa:rul-rootless-worlds-are-declared-faults`).
    ///
    /// An empty store answers as production does (`store-not-initialized`); the rootless refusal is
    /// reachable ONLY through a declared `receipt-read` edge-fault (the read-side twin of
    /// `receipt-publish`), so `no-controller-root` is a world the case DECLARES rather than a
    /// session-history flag. `--receipt <file>` (an explicit file root) and `--receipts` (a store
    /// override) are not expressible in-process and DECLINE (`None` routes the session to the
    /// process driver).
    ///
    /// The store LABEL is always the loom's honest one (`NO_STATE_ROOT`): its world has no per-user
    /// profile, and the synthetic model-store path is platform-shaped and meaningless to an
    /// operator, so it never renders; only the ANSWER changes with the world.
    fn run_receipt_store_why(
        &self,
        case: &Case,
        args: &dorc_cli::Args,
        session: &mut LoomSession,
        output: OutputRouting,
    ) -> Option<DorcEngineReplay> {
        if matches!(args.receipt_root(), dorc_cli::engine::ReceiptRoot::File(_))
            || args.receipts.is_some()
        {
            return None;
        }
        let routing = output;
        let fault = crate::edge_fault::EdgeFault::from_case(case).ok().flatten();
        let answer = if let Some(crate::edge_fault::EdgeFault::ReceiptRead(reason)) = &fault {
            dorc_cli::recorded::StoreAnswer::Unreadable(reason.clone())
        } else {
            dorc_cli::compose::read_rooted_receipt(&session.edge, session.store.as_mut(), args)
        };
        let mut sink = LoomOutputSink {
            ctx: self.render_ctx(),
            actions: Vec::new(),
        };
        // The source-comparison's CURRENT bytes come from the case's own sections, never real disk:
        // in-process the recorded book/oracle is unchanged, so it renders Matching exactly as the
        // shipped binary reading the materialized case dir does (`30Va:rul-source-comparison-is-one-cli-seat`).
        let current_sources: BTreeMap<String, Vec<u8>> = case
            .sections()
            .iter()
            .map(|section| {
                (
                    section.name().to_owned(),
                    section.content().as_bytes().to_vec(),
                )
            })
            .collect();
        let status = dorc_cli::engine::report_recorded_store(
            answer,
            args.why_register(),
            dorc_cli::engine::NO_STATE_ROOT,
            &mut sink,
            Some(&current_sources),
        );
        Some(dorc_engine_replay(status, sink.actions, &routing))
    }

    fn run_engine(
        &self,
        case: &Case,
        args: &dorc_cli::Args,
        command: &ReplayCommand,
        context: &ReplayContext<'_>,
        session: &mut LoomSession,
        output: OutputRouting,
    ) -> Option<DorcEngineReplay> {
        if args.plan.is_some() || !args.oracle_dirs.is_empty() || args.reads_the_receipt() {
            return None;
        }
        let fault = crate::edge_fault::EdgeFault::from_case(case).ok().flatten();
        let stdin = replay_stdin(command, context)?;
        let book_path = args.book.as_deref()?;
        let book = match replay_source("book", book_path, stdin.bytes(), context, fault.as_ref()) {
            Ok(book) => book,
            Err(diagnostic) => return self.invocation_diagnostic(case, *diagnostic, "dorc"),
        };
        let mut paths = Vec::new();
        let mut sources = Vec::new();
        for path in &args.pre_sources {
            let source = match replay_source("oracle", path, stdin.bytes(), context, fault.as_ref())
            {
                Ok(source) => source,
                Err(diagnostic) => return self.invocation_diagnostic(case, *diagnostic, "dorc"),
            };
            sources.push(source);
            paths.push(path.clone());
        }
        let ambient = paths.len();
        for section in case.sections() {
            if section.name() != book_path
                && std::path::Path::new(section.name())
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("sh"))
                && !paths.iter().any(|path| path == section.name())
            {
                paths.push(section.name().to_owned());
                sources.push(context.read_file(section.name())?);
            }
        }
        let snapshot = engine_snapshot(&session.cwd, book_path, &book, paths, sources, ambient);
        let raw_results = match args.results.as_deref() {
            Some(path) => {
                match replay_source("results", path, stdin.bytes(), context, fault.as_ref()) {
                    Ok(results) => Some(results),
                    Err(diagnostic) => {
                        return self.invocation_diagnostic(case, *diagnostic, "dorc");
                    }
                }
            }
            None => None,
        };
        let controller_results = matches!(
            command.input(),
            Some(ReplayInputTarget::File(path)) if path.ends_with("controller-results.txt")
        );
        let observation = loom_observation(raw_results, controller_results);
        // The admin.s REFUSAL, as `main.rs` gates it: naming a store moves WHERE a receipt lands,
        // never WHETHER one is written (`dorc-replay-is-production-semantics`).
        let options = dorc_cli::engine_options_from_args(
            args,
            replay_stdout_posture(command),
            args.artifact_dir.is_some(),
            !args.no_receipt,
        );
        let discovered_oracles = case
            .sections()
            .iter()
            .map(errorloom::Section::name)
            .filter(|path| path.ends_with(".oracle.sh"))
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let acquisition_diagnostics = dorc_cli::unloaded_sibling_oracle_diagnostics(
            snapshot.cwd(),
            snapshot.oracle_paths(),
            &discovered_oracles,
        );
        let routing = output;
        let seams = session.seams()?;
        let mut edges = LoomEngineEdges {
            observation: Some(observation),
            clock: seams.clock(),
            argv: command.argv().to_vec(),
            fault,
            shim_dir: args.shim_dir.clone(),
            receipt_label: dorc_cli::engine::NO_STATE_ROOT.to_owned(),
            host: args.host.clone(),
            edge: session.edge.clone(),
            seams,
            store: session.store.as_mut(),
        };
        let mut sink = LoomOutputSink {
            ctx: self.render_ctx(),
            actions: Vec::new(),
        };
        let result = dorc_cli::engine::run(
            &dorc_cli::engine::EngineRequest {
                snapshot: &snapshot,
                options: &options,
                acquisition_diagnostics: &acquisition_diagnostics,
            },
            &mut edges,
            &mut sink,
        );
        match result {
            Ok(result) => Some(dorc_engine_replay(result.status, sink.actions, &routing)),
            Err(diagnostic) => self.invocation_diagnostic(case, *diagnostic, "dorc"),
        }
    }

    /// Reattach the payload inventory to renderer-stamped exact provenance.
    ///
    /// # Errors
    /// Returns a case-world or renderer-provenance refusal.
    pub fn baseline_from_render(
        &self,
        case: &Case,
        render: EditableRender<SectionKey, SectionVariableId>,
    ) -> Result<DorcEditableBaseline, String> {
        let variables = editable_variables(&render)?;
        // A case that declares no diagnostic has no payload: an arrangement page and a chrome-line
        // REPORT (the drifted receipt) are both built out of registry entries, which store WORDS.
        // The inventory is empty by construction rather than absent by failure — and a report case
        // must not inherit one from whatever diagnostic its durable happens to also provoke.
        if case.frontmatter().scalar("arrangement").is_some()
            || case.frontmatter().scalar("code").is_none()
        {
            return Ok(DorcEditableBaseline {
                render,
                variables,
                all_variables: BTreeMap::new(),
            });
        }
        let diag = self.case_diag(case)?;
        let interner = Interner::default();
        let all_variables = dorc_aid::diag::params_of(&self.render_ctx(), &diag.code, &interner)
            .into_iter()
            .filter(|(_, value)| !value.is_foreign())
            .map(|(name, value)| {
                (
                    TemplateVariableName(String::from(name)),
                    value.text().to_owned(),
                )
            })
            .collect();
        Ok(DorcEditableBaseline {
            render,
            variables,
            all_variables,
        })
    }

    /// The defining replay's typed diagnostic, taken only from the invocation that emitted it.
    ///
    /// # Errors
    /// Returns when the case has no externally-triggered diagnostic matching its declared code.
    pub fn case_diag(&self, case: &Case) -> Result<Diag, String> {
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        if case.replay().blocks().is_empty() {
            return Err("case has no replay".to_owned());
        }
        for block in case.replay().blocks() {
            let command =
                ReplayCommand::parse(block.command()).map_err(|error| error.to_string())?;
            let words: Vec<&str> = command.argv().iter().map(String::as_str).collect();
            if words.first() == Some(&LOOM_COMMAND)
                && matches!(usage::read(words.get(1..).unwrap_or_default()), Reading::Runs(invocation) if matches!(invocation.verb, Verb::Defect))
            {
                return crate::defect::DefectScenario::from_slug(slug)
                    .map(crate::defect::DefectScenario::diagnostic)
                    .ok_or_else(|| format!("`{slug}` is not an authorized defect scenario"));
            }
        }
        let found = RefCell::new(None);
        // A throwaway session: a diagnostic hunt re-drives the case, and a re-drive that would
        // publish must root in its own store (`30Xa:inspection-redrives-carry-no-durable`).
        let session = RefCell::new(LoomSession::new());
        drive_case(case, &RunEnv::new(), |command, context| {
            let words = command
                .argv()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            let replay = match words.first() {
                Some(&"dorc") => self.replay_dorc_outcome(
                    case,
                    command,
                    context,
                    &mut session.borrow_mut(),
                    OutputRouting::default(),
                ),
                Some(&"dorc-sh") => self.replay_dorc_sh_outcome(case, &words, context),
                _ => None,
            };
            if let Some(replay) = replay {
                if found.borrow().is_none() {
                    *found.borrow_mut() = replay
                        .diagnostics
                        .iter()
                        .find(|diagnostic| diagnostic.code.slug() == slug)
                        .cloned();
                }
                return Ok(replay.result);
            }
            Ok(ReplayResult::bytes(String::new()))
        })
        .map_err(|error| error.to_string())?;
        if let Some(diagnostic) = found.into_inner() {
            return Ok(diagnostic);
        }
        Err(format!(
            "case `{slug}` has no externally-triggered diagnostic matching its declared code"
        ))
    }
}

fn is_help_case(case: &Case, words: &[&str]) -> bool {
    matches!(words, ["dorc", "--help" | "-h"])
        && case.frontmatter().scalar("arrangement") == Some(dorc_cli::HELP_ARRANGEMENT)
}

/// The words a compiled arrangement section would STORE, or `None` for a catalog register.
///
/// The ONE derivation, shared by the appliers and by the divergence pre-pass, so the pre-pass can
/// never disagree with the write it is guarding about what an edit lands as.
fn stored_words(field: &str, compiled: &crate::CompiledSection) -> Option<Vec<String>> {
    if field == crate::ARRANGEMENT_LINE_FIELD {
        return Some(line_words(compiled));
    }
    None
}

/// A chrome LINE's word sequence: the compiled fragment series IS the re-split — a `Text` fragment
/// extends the current word, a `Variable` fragment closes it and opens the next.
fn line_words(compiled: &crate::CompiledSection) -> Vec<String> {
    let mut words = vec![String::new()];
    for fragment in compiled.fragments() {
        match fragment {
            crate::CompiledFragment::Text(text) => {
                if let Some(last) = words.last_mut() {
                    last.push_str(text);
                }
            }
            crate::CompiledFragment::Variable(_) => words.push(String::new()),
        }
    }
    words.iter().map(|word| collapse_runs(word)).collect()
}

/// Collapses soft wraps but preserves `\n\n`; never trims edge spaces that may separate a value.
pub(crate) fn collapse_runs(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    let mut run_newlines: Option<usize> = None;
    for character in word.chars() {
        if character.is_whitespace() {
            let newlines = run_newlines.get_or_insert(0);
            if character == '\n' {
                *newlines = newlines.saturating_add(1);
            }
            continue;
        }
        if let Some(newlines) = run_newlines.take() {
            out.push_str(if newlines >= 2 { "\n\n" } else { " " });
        }
        out.push(character);
    }
    if let Some(newlines) = run_newlines {
        out.push_str(if newlines >= 2 { "\n\n" } else { " " });
    }
    out
}

/// The registry index serving `(slug, occurrence)`: the occurrence's own entry when it has one,
/// else the whole-slug entry. The mutable-mirror twin of
/// [`ArrangementLookup::words`](dorc_aid::arrangement::ArrangementLookup::words) — kept in step
/// with it, since an edit must land on the entry the render read.
fn arrangement_index(
    arrangements: &[OwnedArrangement],
    slug: &str,
    occurrence: usize,
) -> Option<usize> {
    arrangements
        .iter()
        .position(|entry| entry.slug == slug && entry.occurrence == Some(occurrence))
        .or_else(|| {
            arrangements
                .iter()
                .position(|entry| entry.slug == slug && entry.occurrence.is_none())
        })
}

fn engine_snapshot(
    cwd: &dorc_core::loadpath::Cwd,
    book_path: &str,
    book_src: &str,
    paths: Vec<String>,
    srcs: Vec<String>,
    ambient: usize,
) -> dorc_cli::snapshot::StaticLoadSnapshot {
    let cwd = cwd.clone();
    let book_sourced = dorc_cli::snapshot::book_reached(&cwd, &paths, &srcs, book_src);
    let dependencies = dorc_cli::snapshot::root_dependencies(&cwd, &paths, &srcs, ambient);
    let mut kept_paths = Vec::new();
    let mut kept_srcs = Vec::new();
    let mut kept_book_sourced = std::collections::BTreeSet::new();
    let mut kept_dependencies = std::collections::BTreeSet::new();
    for (index, (path, source)) in paths.into_iter().zip(srcs).enumerate() {
        if index >= ambient && !book_sourced.contains(&index) && !dependencies.contains(&index) {
            continue;
        }
        let kept = kept_paths.len();
        kept_paths.push(path);
        kept_srcs.push(source);
        if book_sourced.contains(&index) {
            kept_book_sourced.insert(kept);
        } else if index >= ambient {
            kept_dependencies.insert(kept);
        }
    }
    dorc_cli::snapshot::StaticLoadSnapshot::over(
        cwd,
        kept_paths,
        kept_srcs,
        &dorc_cli::snapshot::LoadPositions::book_sourced(kept_book_sourced)
            .with_dependencies(kept_dependencies),
        book_path,
        book_src,
    )
}

/// Route a `--results` block's bytes to the CONTROLLER intake for EVERY session line
/// (`30Xa:rul-in-process-sessions-take-the-controller-intake`), so a session's plan is
/// receipt-eligible and publishes into the store exactly as under the harness.
///
/// An already-framed stream (an authored `controller-results.txt`, or any `dorc-records/`-headed
/// bytes) passes straight through, bounded here and checked against `default_framing`. Raw inner
/// records — and the empty `/dev/null` target, an empty framed stream — carry to
/// [`LoomEngineEdges::observe`], which frames them against the engine's own probe stage.
fn loom_observation(raw_results: Option<String>, controller_results: bool) -> LoomObservation {
    let Some(raw) = raw_results else {
        return LoomObservation::Controller(dorc_plan::records::Admission::NoObservation);
    };
    if controller_results || raw.starts_with("dorc-records/") {
        let evidence = dorc_plan::records::read_host_evidence(
            std::io::Cursor::new(&raw),
            dorc_plan::records::HostEvidenceLimits::spike_default(),
        );
        return LoomObservation::Controller(evidence);
    }
    LoomObservation::ControllerToFrame(raw)
}

/// The program name whose invocations both replay chains answer in-process.
const LOOM_COMMAND: &str = "dorc-loom";

/// Which inventory a `dorc-loom` replay asked for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Inventory {
    Vars(Breadth),
    Sections,
}

/// What `--this` resolves to: the slug this case DEFINES, which is also the filename it must carry
/// (`288:rul-slug-decides-loom-placement`).
///
/// A case with neither key defines nothing, so there is no identity for a selector to name and the
/// block declines — which is the same answer the old target-naming form gave, without the rename
/// hazard of spelling the slug in the command.
fn self_slug(case: &Case) -> Option<&str> {
    case.frontmatter()
        .scalar("code")
        .or_else(|| case.frontmatter().scalar("arrangement"))
}

/// How an inventory names the case it is about: the slug the case DECLARES, at every seat.
///
/// A case that declares neither key is a corpus error every other gate already reports, so this
/// says exactly that rather than reaching for the filename — a second identity is what makes two
/// seats disagree.
fn case_label(case: &Case) -> &str {
    self_slug(case).unwrap_or("<declares neither code nor arrangement>")
}

/// One render component, as `dorc-loom sections` prints it.
fn write_component(out: &mut String, component: &RenderComponent<SectionKey, SectionVariableId>) {
    match component {
        RenderComponent::Structure(text) => {
            let _ = writeln!(out, "  computed: {text:?}");
        }
        // Every fixed value in a Dorc render is a `RenderPart::ForeignText` — the ONE thing that
        // mints this component (`passthrough-is-type-gated`). Saying so is the difference between
        // an author reading `{{detail}}` here and reaching for it, and knowing not to: the name
        // looks exactly like an insertable hole, `vars --all` deliberately omits it, and typing it
        // earns an `UnknownVariable` with nothing to point at.
        RenderComponent::FixedVariable { id, rendered } => {
            let _ = writeln!(
                out,
                "  computed {{{{{}}}}} = {rendered:?} — foreign passthrough: absent from `vars`, \
                 and typing the name is refused",
                id.name.0
            );
        }
        RenderComponent::EditableSection(section) => {
            let key = section.id();
            let _ = writeln!(
                out,
                "  section {}/{}#{} (segment {}):",
                key.owner, key.field, key.instance, key.segment,
            );
            for fragment in section.fragments() {
                match fragment {
                    EditableFragment::Text(text) => {
                        let _ = writeln!(out, "    text: {text:?}");
                    }
                    EditableFragment::Variable { id, rendered } => {
                        let _ = writeln!(out, "    var {{{{{}}}}} = {rendered:?}", id.name.0);
                    }
                }
            }
        }
    }
}

fn case_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains(['\\', ':'])
        && path
            .split('/')
            .all(|component| !matches!(component, "" | "." | ".."))
}

fn rebase_editable_instances(
    render: &EditableRender<SectionKey, SectionVariableId>,
    next: &mut BTreeMap<(String, &'static str), usize>,
) -> EditableRender<SectionKey, SectionVariableId> {
    let mut widths: BTreeMap<(String, &'static str), usize> = BTreeMap::new();
    let components = render
        .components()
        .iter()
        .cloned()
        .map(|component| match component {
            RenderComponent::EditableSection(section) => {
                let mut key = section.id().clone();
                let group = (key.owner.clone(), key.field);
                let base = next.get(&group).copied().unwrap_or_default();
                widths
                    .entry(group)
                    .and_modify(|width| *width = (*width).max(key.instance.saturating_add(1)))
                    .or_insert(key.instance.saturating_add(1));
                key.instance = key.instance.saturating_add(base);
                RenderComponent::EditableSection(errorloom::EditableSection::new(
                    key,
                    section.fragments().to_vec(),
                ))
            }
            other => other,
        })
        .collect();
    for (group, width) in widths {
        next.entry(group)
            .and_modify(|base| *base = base.saturating_add(width))
            .or_insert(width);
    }
    EditableRender::new(components)
}

enum ReplayStdin {
    Absent,
    Bytes(String),
}

impl ReplayStdin {
    fn bytes(&self) -> Option<&str> {
        match self {
            Self::Absent => None,
            Self::Bytes(bytes) => Some(bytes),
        }
    }
}

fn replay_stdin(command: &ReplayCommand, context: &ReplayContext<'_>) -> Option<ReplayStdin> {
    match command.input() {
        None => Some(ReplayStdin::Absent),
        Some(ReplayInputTarget::Null) => Some(ReplayStdin::Bytes(String::new())),
        Some(ReplayInputTarget::File(path)) => context.read_file(path).map(ReplayStdin::Bytes),
    }
}

fn replay_source(
    kind: &str,
    path: &str,
    stdin: Option<&str>,
    context: &ReplayContext<'_>,
    fault: Option<&crate::edge_fault::EdgeFault>,
) -> Result<String, Box<Diag>> {
    if let Some(failure) = fault.and_then(|fault| fault.read_failure(path)) {
        return Err(Box::new(dorc_cli::humane_read_error(
            kind,
            path,
            &failure.error(),
        )));
    }
    if path == "-" {
        return Ok(stdin.unwrap_or_default().to_owned());
    }
    if case_relative_path(path)
        && let Some(source) = context.read_file(path)
    {
        return Ok(source);
    }
    Err(Box::new(dorc_cli::humane_read_error(
        kind,
        path,
        &std::io::Error::new(std::io::ErrorKind::NotFound, "not found in replay sandbox"),
    )))
}

fn replay_stdout_posture(command: &ReplayCommand) -> dorc_cli::artifact::StdoutPosture {
    if command.stdout_is_terminal() {
        dorc_cli::artifact::StdoutPosture::Interactive
    } else {
        dorc_cli::artifact::StdoutPosture::NonInteractive
    }
}

/// A `cd <literal>` in the modelled session. Only a cd that STAYS at the case root (`cd .`) is a
/// no-op; a cd that would MOVE the cwd declines the whole session to the process driver. The section
/// names a case materializes are flat, so a moved cwd would silently misresolve every later relative
/// operand (`--book`, `<`, `cat`) — an honest decline beats that scaffold, and the cwd model that
/// would meet a moved cwd is a kernel-arc change (`30X:front-dogfood-ceiling`).
fn replay_cd(
    session: &LoomSession,
    target: &str,
) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
    let resolved = session.cwd.resolve_operand(target)?;
    resolved
        .is_empty()
        .then(|| ReplayResult::bytes(String::new()))
}

struct DorcEngineReplay {
    result: ReplayResult<SectionKey, SectionVariableId>,
    diagnostics: Vec<Diag>,
}

impl DorcEngineReplay {
    fn without_diagnostic(result: ReplayResult<SectionKey, SectionVariableId>) -> Self {
        Self {
            result,
            diagnostics: Vec::new(),
        }
    }
}

fn dorc_engine_replay(
    status: dorc_cli::engine::EngineStatus,
    actions: Vec<dorc_cli::engine::OutputAction>,
    routing: &OutputRouting,
) -> DorcEngineReplay {
    let diagnostics = actions
        .iter()
        .filter_map(|action| match action {
            dorc_cli::engine::OutputAction::Event(event) => event.diagnostic_payload().cloned(),
            dorc_cli::engine::OutputAction::Flush(_) => None,
        })
        .collect();
    let mut section_instances = BTreeMap::new();
    let emissions = actions
        .into_iter()
        .filter_map(|action| match action {
            dorc_cli::engine::OutputAction::Flush(_) => None,
            dorc_cli::engine::OutputAction::Event(event) => {
                let (channel, suppressed) = match event.channel() {
                    dorc_cli::engine::OutputChannel::Stdout => {
                        (errorloom::ReplayChannel::Stdout, routing.stdout_to_null)
                    }
                    dorc_cli::engine::OutputChannel::Stderr => {
                        (errorloom::ReplayChannel::Stderr, routing.stderr_to_null)
                    }
                };
                if suppressed {
                    return None;
                }
                Some(match event.tagged_parts() {
                    Some(parts) => ReplayEmission::editable(
                        channel,
                        rebase_editable_instances(
                            &to_editable_render(parts),
                            &mut section_instances,
                        ),
                    ),
                    None => ReplayEmission::bytes(channel, event.text()),
                })
            }
        })
        .collect();
    DorcEngineReplay {
        result: ReplayResult::emitted(ReplayStatus::new(i32::from(status.exit_code())), emissions),
        diagnostics,
    }
}

struct LoomOutputSink<'a> {
    ctx: RenderCtx<'a>,
    actions: Vec<dorc_cli::engine::OutputAction>,
}

struct LoomToolRunner<'a> {
    fault: Option<&'a crate::edge_fault::EdgeFault>,
}

impl dorc_lint::ExternalToolRunner for LoomToolRunner<'_> {
    fn available(&self, _tool: &str) -> bool {
        matches!(
            self.fault,
            Some(crate::edge_fault::EdgeFault::ToolRun { .. })
        )
    }

    fn run(&self, tool: &str, _args: &[&str], _stdin: &[u8]) -> dorc_lint::ToolRun {
        match self.fault {
            Some(crate::edge_fault::EdgeFault::ToolRun {
                tool: fault_tool,
                rc,
                stdout,
            }) if fault_tool == tool => dorc_lint::ToolRun {
                rc: *rc,
                stdout: stdout.clone(),
                stderr: Vec::new(),
            },
            _ => dorc_lint::ToolRun {
                rc: 0,
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
        }
    }
}

impl dorc_cli::engine::OutputSink for LoomOutputSink<'_> {
    fn render_ctx(&self) -> RenderCtx<'_> {
        self.ctx.clone()
    }

    fn emit(&mut self, event: dorc_cli::engine::OutputEvent) {
        self.actions
            .push(dorc_cli::engine::OutputAction::Event(event));
    }

    fn flush(&mut self, channel: dorc_cli::engine::OutputChannel) {
        self.actions
            .push(dorc_cli::engine::OutputAction::Flush(channel));
    }
}

struct LoomEngineEdges<'io> {
    observation: Option<LoomObservation>,
    clock: dorc_cli::results::RunClock,
    argv: Vec<String>,
    fault: Option<crate::edge_fault::EdgeFault>,
    shim_dir: Option<String>,
    receipt_label: String,
    host: Option<String>,
    /// This session's pinned edge and this block's seam selections, over a borrow of the SESSION
    /// store, so a publish lands in the one world later blocks read
    /// (`dorc-replay-is-production-semantics`).
    seams: dorc_cli::seam::Seams,
    edge: dorc_cli::durable::LocalReceiptEdgeV1,
    store: &'io mut dyn dorc_cli::durable::LocalIo,
}

enum LoomObservation {
    /// An already-framed controller stream, bounded here and checked against `default_framing`.
    Controller(dorc_plan::records::Admission<dorc_plan::records::BoundedHostBytes>),
    /// Raw inner records, framed against the engine's own probe stage in [`LoomEngineEdges::observe`].
    ControllerToFrame(String),
}

impl dorc_cli::engine::EngineEdges for LoomEngineEdges<'_> {
    fn materialize_shims(&mut self, files: &BTreeMap<String, String>) -> Result<(), Box<Diag>> {
        if files.is_empty() {
            return Ok(());
        }
        if let Some(crate::edge_fault::EdgeFault::ShimWrite { path, failure }) = &self.fault
            && self.shim_dir.as_deref() == Some(path)
        {
            return Err(Box::new(dorc_cli::shim_write_error(path, &failure.error())));
        }
        Ok(())
    }

    fn observe(
        &mut self,
        request: &dorc_cli::engine::ObservationRequest<'_>,
        render_probe: &dyn Fn(&dorc_plan::records::Framing) -> String,
    ) -> Result<dorc_cli::engine::Observation, Box<Diag>> {
        if let Some(crate::edge_fault::EdgeFault::HostEvidence(refusal)) = &self.fault {
            return Ok(dorc_cli::engine::Observation::Controller {
                framing: request.default_framing.clone(),
                evidence: dorc_plan::records::Admission::Refused(*refusal),
                stderr: Vec::new(),
            });
        }
        if let Some(host) = self.host.as_deref()
            && let Some(crate::edge_fault::EdgeFault::Transport(failure)) = &self.fault
        {
            let (status, diagnostic) = match failure {
                crate::edge_fault::TransportFailure::SessionLost => (
                    dorc_cli::engine::EngineStatus::SessionLost,
                    dorc_cli::transport_session_lost(
                        host,
                        3,
                        &dorc_transport::TransportDiagnosis::ChildLost,
                    ),
                ),
                crate::edge_fault::TransportFailure::SpawnRefused(detail) => (
                    dorc_cli::engine::EngineStatus::HostNotReached,
                    dorc_cli::transport_spawn_refused(host, detail),
                ),
                crate::edge_fault::TransportFailure::MarkerUnusable => (
                    dorc_cli::engine::EngineStatus::HostNotReached,
                    dorc_cli::transport_marker_unusable(host),
                ),
                crate::edge_fault::TransportFailure::Crlf { line } => (
                    dorc_cli::engine::EngineStatus::HostNotReached,
                    dorc_cli::transport_crlf_error("the plan", *line),
                ),
                crate::edge_fault::TransportFailure::ApplyFailed { .. } => {
                    return Err(Box::new(dorc_cli::transport_apply_failed(host, 2)));
                }
            };
            return Ok(dorc_cli::engine::Observation::Terminal { status, diagnostic });
        }
        Ok(
            match self
                .observation
                .take()
                .unwrap_or(LoomObservation::Controller(
                    dorc_plan::records::Admission::NoObservation,
                )) {
                LoomObservation::Controller(evidence) => {
                    dorc_cli::engine::Observation::Controller {
                        framing: request.default_framing.clone(),
                        evidence,
                        stderr: Vec::new(),
                    }
                }
                // Frame against the engine's OWN probe stage — the in-process twin of the process
                // driver's `dorc probe` (`inv-site-keyed-results`) — then admit through the controller.
                LoomObservation::ControllerToFrame(raw) => {
                    let probe = render_probe(request.default_framing);
                    let framed = crate::records_framing::frame_records(&probe, &raw);
                    dorc_cli::engine::Observation::Controller {
                        framing: request.default_framing.clone(),
                        evidence: dorc_plan::records::read_host_evidence(
                            std::io::Cursor::new(framed.into_bytes()),
                            dorc_plan::records::HostEvidenceLimits::spike_default(),
                        ),
                        stderr: Vec::new(),
                    }
                }
            },
        )
    }

    fn clock(&mut self) -> &mut dorc_cli::results::RunClock {
        &mut self.clock
    }

    fn source_match(&mut self, _book_name: &str) -> Option<dorc_cli::SourceMatch> {
        None
    }

    fn publish_artifact(
        &mut self,
        _artifact: &dorc_cli::artifact::ArtifactSet,
    ) -> Result<(), &'static str> {
        if let Some(crate::edge_fault::EdgeFault::ArtifactPublish(reason)) = &self.fault {
            return Err(*reason);
        }
        Ok(())
    }

    /// A loom drive publishes into the SESSION's own model store, through production's ONE publish
    /// core (`dorc-replay-is-production-semantics`), so a later block reads back what this one wrote.
    /// A declared `receipt-publish` fault is how a case exercises the refusal without touching it.
    fn publish_receipt(
        &mut self,
        request: &dorc_cli::engine::ReceiptPublicationRequest<'_>,
    ) -> Result<Option<dorc_cli::receipt_edge::PlacedDocument>, String> {
        if let Some(crate::edge_fault::EdgeFault::ReceiptPublish(reason)) = &self.fault {
            return Err(reason.clone());
        }
        dorc_cli::compose::publish_seamed_receipt(
            &mut *self.store,
            &self.edge,
            &self.seams,
            &mut self.clock,
            request,
        )
    }

    fn receipt_label(&self) -> &str {
        &self.receipt_label
    }

    fn invocation_record(
        &mut self,
        request: dorc_cli::engine::InvocationRecordRequest<'_>,
    ) -> dorc_core::spine::SpineInvocation {
        dorc_cli::receipt_edge::invocation_record(
            self.argv.clone(),
            request.framing,
            request.snapshot,
            request.started_at,
            request.account,
        )
    }
}

/// One in-process session's world: the deterministic model store, born empty at block 0, and the
/// pinned edge over it (`30X:tier-in-process-loom`). A session is ONE world across a case's blocks —
/// a block that publishes and a later block that reads share it — and it NEVER crosses a case, a
/// run, or an inspection re-drive (`receipts-not-a-cache`, `inv-recorded-values-stay-recorded`,
/// `30Xa:inspection-redrives-carry-no-durable`): a fresh [`DorcReplayDriver`] per drive means a
/// fresh throwaway store for free, so a re-drive that would publish cannot collide with the first.
struct LoomSession {
    store: Box<dyn dorc_cli::durable::LocalIo>,
    edge: dorc_cli::durable::LocalReceiptEdgeV1,
    /// The modelled environment (`30X:loom-seams-are-sh-lines`): seam selections spelled as `export`
    /// lines in the session, read through the ONE parser as the exported subset.
    env: SessionEnv,
    /// The modelled cwd (`dorc_core::loadpath::Cwd`): the session's own cwd model, moved by `cd`, and
    /// every path operand resolves through it before reaching the invocation or the framing seat.
    cwd: dorc_core::loadpath::Cwd,
}

impl LoomSession {
    fn new() -> Self {
        Self::with_env(SessionEnv::seeded())
    }

    /// A session seeded with an explicit run seed, so a blessing authority can re-render the same
    /// case under a SECOND seed (`30Xa:Checkpoint D1`, rider d).
    fn new_seeded(seed: u64) -> Self {
        Self::with_env(SessionEnv::seeded_with(seed))
    }

    #[expect(
        clippy::expect_used,
        reason = "the pinned synthetic root is a runner literal that always resolves; a refusal is a bug"
    )]
    fn with_env(env: SessionEnv) -> Self {
        let store: Box<dyn dorc_cli::durable::LocalIo> = Box::new(platform_model_io(
            dorc_cli::durable::FailureSchedule::intact(),
        ));
        let seams: dorc_cli::seam::Seams = dorc_cli::seam::HarnessSeams::from_env(&env)
            .expect("the runner's seeded defaults always parse")
            .into();
        let edge = dorc_cli::compose::production_receipt_edge_over(&seams, None, &LoomRootEnv)
            .expect("the pinned synthetic root always resolves");
        Self {
            store,
            edge,
            env,
            cwd: dorc_core::loadpath::Cwd::default(),
        }
    }

    /// This block's `Seams` from the modelled environment's exported subset, through the ONE parser
    /// both drivers use (`30X:loom-seams-are-sh-lines`). `None` when an authored `export` gave a seam
    /// a value the parser refuses — a decline, so the process driver proves that line.
    fn seams(&self) -> Option<dorc_cli::seam::Seams> {
        dorc_cli::seam::HarnessSeams::from_env(&self.env)
            .ok()
            .map(Into::into)
    }

    /// This block's `Seams` with the transport replaced by the scripted column
    /// (`30X:front-transport-scripted-column`): the `--host` apply route answers its session from the
    /// case's own declaration through the SAME `SessionDriver` the ssh and local drivers implement.
    fn scripted_seams(&self, script: dorc_transport::SimScript) -> Option<dorc_cli::seam::Seams> {
        dorc_cli::seam::HarnessSeams::from_env(&self.env)
            .ok()
            .map(|harness| harness.with_scripted_transport(script).into())
    }
}

/// The platform-shaped model store, forced by the store's baseline check: a publish whose
/// `directory_sync()` mismatches the roots' `host_platform()` is refused
/// (`store.rs meets_required_baseline`), and the pinned roots resolve against `host_platform()`.
/// Rendered bytes carry no platform path, so both legs agree with one transcript
/// (`30Xa:Checkpoint C1-map`, R2). The `schedule` is `intact()` for a session's own world and a
/// scheduled fault for the post-dispatch outcome-unwritable drive.
fn platform_model_io(schedule: dorc_cli::durable::FailureSchedule) -> dorc_cli::durable::ModelIo {
    if cfg!(windows) {
        dorc_cli::durable::ModelIo::windows_shaped(schedule)
    } else {
        dorc_cli::durable::ModelIo::new(schedule, dorc_cli::durable::DirectorySync::Synchronized)
    }
}

/// Put a carriage return at the end of the 1-based `line` of `bytes`, so the transport's own
/// `first_carriage_return` check reports that line — the CR the LF-only case format cannot carry.
fn inject_carriage_return(bytes: &[u8], line: usize) -> Vec<u8> {
    let text = String::from_utf8_lossy(bytes);
    let mut out = Vec::new();
    for (index, segment) in text.split_inclusive('\n').enumerate() {
        if line.checked_sub(1) != Some(index) {
            out.extend_from_slice(segment.as_bytes());
        } else if let Some(body) = segment.strip_suffix('\n') {
            out.extend_from_slice(body.as_bytes());
            out.extend_from_slice(b"\r\n");
        } else {
            out.extend_from_slice(segment.as_bytes());
            out.push(b'\r');
        }
    }
    out
}

/// The scripted session outcome a `--host` apply answers from: the transport-fault half (the
/// `SimScript` a real driver would produce), the outcome-fault's own success transport, or the
/// `hosts/<name>/apply-outcome` success declaration (`282` §2's reserved convention; spelling
/// strawman, `rul-strawman-formats-no-compat`). `None` is a typed decline: no scripted outcome for
/// host `<name>` (`LoomDecline::RemoteApply`).
fn scripted_session(
    case: &Case,
    host: &str,
    fault: Option<&crate::edge_fault::EdgeFault>,
    outcome_unwritable: bool,
) -> Option<dorc_transport::SimScript> {
    use crate::edge_fault::{EdgeFault, TransportFailure};
    use dorc_transport::SimScript;
    match fault {
        Some(EdgeFault::Transport(TransportFailure::ApplyFailed { status })) => {
            Some(SimScript::Completes {
                stdout: Vec::new(),
                status: *status,
            })
        }
        Some(EdgeFault::Transport(TransportFailure::SessionLost)) => {
            Some(SimScript::SeveredAfter { stdout: Vec::new() })
        }
        Some(EdgeFault::Transport(TransportFailure::SpawnRefused(detail))) => {
            Some(SimScript::NotSpawnable {
                reason: detail.clone(),
            })
        }
        // A CRLF refuses pre-dispatch, before the driver is built; the script is a harmless stand-in.
        Some(EdgeFault::Transport(TransportFailure::Crlf { .. })) => Some(SimScript::Completes {
            stdout: Vec::new(),
            status: 0,
        }),
        // The marker-unusable world is not a driver outcome (the seeded nonce is always marker-safe).
        Some(EdgeFault::Transport(TransportFailure::MarkerUnusable)) => None,
        // The outcome-unwritable world has a SUCCESS transport; the outcome write is what fails.
        _ if outcome_unwritable => Some(SimScript::Completes {
            stdout: Vec::new(),
            status: 0,
        }),
        _ => scripted_host_outcome(case, host),
    }
}

/// Parse a `hosts/<name>/apply-outcome` section into the success transport it declares: `status <n>`
/// on the head line, the captured stdout in the body (spelling strawman).
fn scripted_host_outcome(case: &Case, host: &str) -> Option<dorc_transport::SimScript> {
    let wanted = format!("hosts/{host}/apply-outcome");
    let section = case.sections().iter().find(|s| s.name() == wanted)?;
    let content = section.content();
    let (head, body) = content.split_once('\n').unwrap_or((content, ""));
    let status = match head.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
        ["status", value] => value.parse::<i32>().ok()?,
        _ => return None,
    };
    Some(dorc_transport::SimScript::Completes {
        stdout: body.as_bytes().to_vec(),
        status,
    })
}

/// Drive one apply whose OUTCOME document cannot be written — the loom's post-dispatch failure world
/// (`30X` §11). The outcome's own create is DISCOVERED on a throwaway store rather than named by a
/// fragile constant, then faulted, so the intent publishes and the outcome does not; the render is
/// the durable-failure diagnostic.
#[expect(
    clippy::result_large_err,
    reason = "the Err is production's full `Diag`, as on the apply route it drives"
)]
fn drive_apply_outcome_unwritable(
    edge: &dorc_cli::durable::LocalReceiptEdgeV1,
    seams: &dorc_cli::seam::Seams,
    sink: &mut LoomOutputSink<'_>,
    args: &dorc_cli::Args,
    artifact: &[u8],
    host: &str,
) -> Result<dorc_cli::engine::EngineStatus, Diag> {
    let mut probe = platform_model_io(dorc_cli::durable::FailureSchedule::intact());
    let mut discard = LoomOutputSink {
        ctx: sink.ctx.clone(),
        actions: Vec::new(),
    };
    let _ = dorc_cli::compose::dispatch_and_report_apply(
        &mut probe,
        edge,
        seams,
        &mut discard,
        args,
        artifact,
        host,
    );
    let occurrence = last_create_file_occurrence(probe.schedule());
    let mut store = platform_model_io(dorc_cli::durable::FailureSchedule::faulting_occurrence(
        dorc_cli::durable::Op::CreateFileExclusive,
        dorc_cli::durable::Side::Before,
        occurrence,
        dorc_cli::durable::IoFault::Denied,
    ));
    dorc_cli::compose::dispatch_and_report_apply(
        &mut store, edge, seams, sink, args, artifact, host,
    )
}

/// The 0-based occurrence of the LAST `CreateFileExclusive` a run reached — the outcome document's
/// own create in an apply (the keyset docs, the manifest, and the intent all precede it). A run that
/// created nothing (unreachable for an apply) yields 0.
fn last_create_file_occurrence(schedule: &dorc_cli::durable::FailureSchedule) -> usize {
    schedule
        .arrivals()
        .iter()
        .filter(|(op, side)| {
            *op == dorc_cli::durable::Op::CreateFileExclusive
                && *side == dorc_cli::durable::Side::Before
        })
        .count()
        .saturating_sub(1)
}

impl std::fmt::Debug for LoomSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoomSession")
            .field("edge", &self.edge)
            .finish_non_exhaustive()
    }
}

/// The loom's root-resolution query: it reads nothing, because the session's roots are `Pinned` (a
/// runner-owned literal), so no `APPDATA`/`HOME`/`XDG_*` is ever consulted.
struct LoomRootEnv;

impl dorc_cli::durable::RootEnvironment for LoomRootEnv {
    fn var(&self, _name: &str) -> Option<String> {
        None
    }
}

/// Consumer-neutral replay dispatch is implemented by this exact-shape Dorc adapter.
#[derive(Debug)]
pub struct DorcReplayDriver<'a> {
    consumer: &'a DorcConsumer,
    case: &'a Case,
    self_reference: SelfReference,
    /// This drive's one world (`30Xa:inspection-redrives-carry-no-durable`): interior-mutable so the
    /// shared-`&self` [`ReplayDriver::drive`] can thread it through the replay chain.
    session: RefCell<LoomSession>,
}

/// Whether a replay may answer a case's inventory of ITSELF.
///
/// A case's inventory is derived from the render an edit compiles against, and that render comes
/// from driving the case — so answering the inventory block while computing the inventory would ask
/// the same question forever. The baseline seat drives with [`SelfReference::Forbidden`] and the
/// block contributes nothing there; every other caller allows it, which is what lets a case carry
/// its own values without a section to read them from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SelfReference {
    Allowed,
    Forbidden,
}

impl<'a> DorcReplayDriver<'a> {
    /// Bind one case to its production-render consumer, with a fresh throwaway session
    /// (`30Xa:inspection-redrives-carry-no-durable`).
    #[must_use]
    pub fn new(consumer: &'a DorcConsumer, case: &'a Case) -> Self {
        Self {
            consumer,
            case,
            self_reference: SelfReference::Allowed,
            session: RefCell::new(LoomSession::new()),
        }
    }

    /// The same driver under an explicit run seed, so a second-seed re-render can prove a candidate
    /// reproduced before a blessing authority writes it (`30Xa:Checkpoint D1`, rider d).
    #[must_use]
    pub fn new_seeded(consumer: &'a DorcConsumer, case: &'a Case, seed: u64) -> Self {
        Self {
            consumer,
            case,
            self_reference: SelfReference::Allowed,
            session: RefCell::new(LoomSession::new_seeded(seed)),
        }
    }

    /// The same driver, declining the case's own inventory block (see [`SelfReference`]).
    fn without_self_reference(mut self) -> Self {
        self.self_reference = SelfReference::Forbidden;
        self
    }
}

impl ReplayDriver<SectionKey, SectionVariableId> for DorcReplayDriver<'_> {
    fn drive(
        &self,
        command: &ReplayCommand,
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        self.consumer.replay_within(
            self.case,
            command,
            context,
            self.self_reference,
            &mut self.session.borrow_mut(),
        )
    }
}

/// Drive a case through the exact Dorc adapter, leaving decline policy to the caller.
///
/// # Errors
/// Returns materialization or caller-supplied fallback failures.
pub fn replay_case<F>(
    case: &Case,
    consumer: &DorcConsumer,
    env: &RunEnv,
    mut fallback: F,
) -> Result<Vec<ReplayResult<SectionKey, SectionVariableId>>, RunError>
where
    F: FnMut(
        &str,
        &ReplayContext<'_>,
    ) -> Result<ReplayResult<SectionKey, SectionVariableId>, RunError>,
{
    let driver = DorcReplayDriver::new(consumer, case);
    drive_case(case, env, |command, context| {
        match driver.drive(command, context) {
            Some(result) => Ok(result),
            None => fallback(command.original(), context),
        }
    })
}

/// Drive a case with explicit bounded files available to both the adapter and any
/// configured generic fallback.
///
/// # Errors
/// Returns materialization or caller-supplied fallback failures.
pub fn replay_case_with_inputs<F>(
    case: &Case,
    consumer: &DorcConsumer,
    env: &RunEnv,
    inputs: &[ReplayInput],
    mut fallback: F,
) -> Result<Vec<ReplayResult<SectionKey, SectionVariableId>>, RunError>
where
    F: FnMut(
        &str,
        &ReplayContext<'_>,
    ) -> Result<ReplayResult<SectionKey, SectionVariableId>, RunError>,
{
    let driver = DorcReplayDriver::new(consumer, case);
    drive_case_with_inputs(case, env, inputs, |command, context| {
        match driver.drive(command, context) {
            Some(result) => Ok(result),
            None => fallback(command.original(), context),
        }
    })
}

/// Why the in-process driver cannot express a session (`30X:loom-in-process-driver-is-a-closed-grammar`).
///
/// One decline anywhere routes the WHOLE session to the process driver, because a session has state
/// and cannot change drivers midway. This enum IS the closed set — `30X` §9 forbids a roster or a
/// lexical meta-test that polices it — and it only ever SHRINKS: the expressible set is exactly what
/// the driver runs today plus the receipt-store arm.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LoomDecline {
    /// A `dorc apply --host` — a remote apply, driven only by the scripted transport table
    /// (`30X:front-transport-scripted-column`), never a real dispatch.
    RemoteApply,
    /// A `dorc apply` reading a plan file — no in-process engine run to express it.
    ApplyWithPlan,
    /// A `--oracle-dir` load — the in-process driver takes `--pre-source` files only.
    OracleDirs,
    /// A `--receipt <file>` out-of-store document root.
    ExplicitReceiptFile,
    /// A `--receipts <dir>` store override.
    ReceiptsOverride,
    /// A command whose first word is not one the driver recognizes — a shell builtin outside the
    /// modelled set, or an external tool (`export`, `cd`, `echo $?` and `cat` ARE modelled now).
    ShellOrExternal(String),
    /// Any other command the in-process driver does not express (`dorc strip`, a `dorc-sh` it
    /// cannot run, an unmodelled durable, a process exit): named so the trial output points at it.
    Unexpressible(String),
}

impl LoomDecline {
    /// One-line reason for the trial output (`30X:loom-driver-is-derived-and-reported`).
    #[must_use]
    pub fn reason(&self) -> String {
        match self {
            Self::RemoteApply => {
                "a remote apply (`--host`) — driven only by the process driver".to_owned()
            }
            Self::ApplyWithPlan => "an apply reading a plan file".to_owned(),
            Self::OracleDirs => {
                "a `--oracle-dir` load (the driver takes `--pre-source` only)".to_owned()
            }
            Self::ExplicitReceiptFile => {
                "a `--receipt <file>` out-of-store document root".to_owned()
            }
            Self::ReceiptsOverride => "a `--receipts <dir>` store override".to_owned(),
            Self::ShellOrExternal(command) => {
                format!("a shell construct or external tool: `{command}`")
            }
            Self::Unexpressible(command) => {
                format!("a command the in-process driver does not express: `{command}`")
            }
        }
    }
}

/// Why the SHELL driver cannot prove a case (`30Xa:rul-drivers-decline-symmetrically`).
///
/// The symmetric twin of [`LoomDecline`]: each driver owns a typed decline set, and the runner
/// reads both to derive which driver proves a case. The shell declines a nonportable outcome it
/// cannot make happen, and any block that invokes the in-process tooling's own binary — nothing
/// else. This set does not grow without a conductor ruling.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ShellDecline {
    /// The case declares an `edge-fault` section: an injected nonportable I/O or transport outcome
    /// the shell cannot reproduce (only the in-process driver scripts it).
    EdgeFault,
    /// A `dorc-loom` invocation — the in-process authoring tool has no shipped binary the shell
    /// session's shim provides, so it is in-process authority by content.
    LoomToolBlock(String),
}

impl ShellDecline {
    /// One-line reason for the trial's proof label (`30X:loom-driver-is-derived-and-reported`).
    #[must_use]
    pub fn reason(&self) -> String {
        match self {
            Self::EdgeFault => {
                "an `edge-fault` section — a nonportable outcome the shell cannot make happen"
                    .to_owned()
            }
            Self::LoomToolBlock(command) => {
                format!("a `dorc-loom` invocation (in-process authoring tool): `{command}`")
            }
        }
    }
}

/// Why the shell driver declines `case`, or `None` when the shell proves it
/// (`30Xa:rul-drivers-decline-symmetrically`). Read by the runner beside [`render_run_loom_in_process`]'s
/// [`LoomDecline`] so which driver proves the case is DERIVED, never declared.
#[must_use]
pub fn shell_decline(case: &Case) -> Option<ShellDecline> {
    if case.sections().iter().any(|s| s.name() == "edge-fault") {
        return Some(ShellDecline::EdgeFault);
    }
    case.replay()
        .blocks()
        .iter()
        .map(errorloom::ReplayBlock::command)
        .find(|command| {
            crate::session_grammar::block_argv(command)
                .first()
                .map(String::as_str)
                == Some("dorc-loom")
        })
        .map(|command| ShellDecline::LoomToolBlock(command.to_owned()))
}

/// The outcome of driving a loom in-process for `gate-two-drivers-agree`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TwoDriverOutcome {
    /// Every block rendered; the stamped both-streams bytes, one per block, in engine order.
    Rendered(Vec<String>),
    /// The session declined and routes to the process driver; its reason.
    Declined(LoomDecline),
}

/// Which decline a command the in-process driver could not express falls under.
///
/// Re-derived from the command's own argv through the SAME `parse_args_from` the driver used, so it
/// agrees with the driver's decline decisions rather than being a second, drifting policy.
fn classify_decline(command: &str) -> LoomDecline {
    let Ok(parsed) = ReplayCommand::parse(command) else {
        return LoomDecline::Unexpressible(command.to_owned());
    };
    match parsed.argv().first().map(String::as_str) {
        Some("dorc") => {
            match dorc_cli::parse_args_from(parsed.argv().get(1..).unwrap_or_default().to_vec()) {
                Ok(dorc_cli::Invocation::Analyze(args)) => {
                    if args.host.is_some() {
                        LoomDecline::RemoteApply
                    } else if args.plan.is_some() {
                        LoomDecline::ApplyWithPlan
                    } else if !args.oracle_dirs.is_empty() {
                        LoomDecline::OracleDirs
                    } else if matches!(args.receipt_root(), dorc_cli::engine::ReceiptRoot::File(_))
                    {
                        LoomDecline::ExplicitReceiptFile
                    } else if args.receipts.is_some() {
                        LoomDecline::ReceiptsOverride
                    } else {
                        LoomDecline::Unexpressible(command.to_owned())
                    }
                }
                _ => LoomDecline::Unexpressible(command.to_owned()),
            }
        }
        // A `dorc-sh` the driver cannot run, or a non-root `cd` (`30X:front-dogfood-ceiling`): unexpressible, distinct from a plain shell/external head.
        Some("dorc-sh" | "cd") => LoomDecline::Unexpressible(command.to_owned()),
        _ => LoomDecline::ShellOrExternal(command.to_owned()),
    }
}

/// Drive a committed `run:` loom in-process, the SECOND witness of every session the e2e runner
/// proves (`gate-two-drivers-agree`; `30X:loom-driver-is-derived-and-reported`).
///
/// If the in-process driver declines ANY block, the whole session is `Declined` with its reason;
/// otherwise every block's stamped both-streams render is returned in engine order
/// (`30X:loom-transcript-is-what-the-user-saw`), for the caller to compare byte-for-byte.
///
/// # Errors
/// Returns a case-materialization failure (never a decline — that is a value).
pub fn render_run_loom_in_process(
    consumer: &DorcConsumer,
    case: &Case,
) -> Result<TwoDriverOutcome, RunError> {
    let driver = DorcReplayDriver::new(consumer, case);
    let declined: RefCell<Option<LoomDecline>> = RefCell::new(None);
    let outcome = drive_case(case, &RunEnv::new(), |command, context| {
        if let Some(result) = driver.drive(command, context) {
            Ok(result)
        } else {
            if declined.borrow().is_none() {
                *declined.borrow_mut() = Some(classify_decline(command.original()));
            }
            // A placeholder: one decline routes the whole session, so this render is discarded.
            Ok(ReplayResult::bytes(String::new()))
        }
    });
    let results = match outcome {
        Ok(results) => results,
        // A block only the shell's grammar can express is a decline, never a gate failure
        // (`30Xa:tc-gate-runerror-should-decline`).
        Err(error @ RunError::UnsupportedReplayGrammar { .. }) => {
            return Ok(TwoDriverOutcome::Declined(LoomDecline::Unexpressible(
                error.to_string(),
            )));
        }
        Err(other) => return Err(other),
    };
    if let Some(reason) = declined.into_inner() {
        return Ok(TwoDriverOutcome::Declined(reason));
    }
    let bytes = results
        .iter()
        .map(|result| {
            result
                .editable_render()
                .map_or_else(|| result.output().to_owned(), EditableRender::text)
        })
        .collect();
    Ok(TwoDriverOutcome::Rendered(bytes))
}

/// This case's block outputs under one run seed (the stamped part stream where a block renders
/// editable prose, else its raw bytes). A declined session yields `None` — it is proven by the
/// shell, so a blessing authority has nothing to reproduce here.
fn block_outputs_under_seed(
    consumer: &DorcConsumer,
    case: &Case,
    seed: u64,
) -> Result<Option<Vec<String>>, RunError> {
    let driver = DorcReplayDriver::new_seeded(consumer, case, seed);
    let declined = RefCell::new(false);
    let outcome = drive_case(case, &RunEnv::new(), |command, context| {
        driver.drive(command, context).map_or_else(
            || {
                *declined.borrow_mut() = true;
                Ok(ReplayResult::bytes(String::new()))
            },
            Ok,
        )
    });
    match outcome {
        Ok(results) if !*declined.borrow() => Ok(Some(
            results
                .iter()
                .map(|result| {
                    result
                        .editable_render()
                        .map_or_else(|| result.output().to_owned(), EditableRender::text)
                })
                .collect(),
        )),
        Ok(_) | Err(RunError::UnsupportedReplayGrammar { .. }) => Ok(None),
        Err(other) => Err(other),
    }
}

/// Re-render `case` under a SECOND seed and refuse a divergence, so `dorc-loom publish` — the
/// in-process blessing authority — never writes a transcript that baked in nondeterminism
/// (`30X:seed-two-affordances`; `30Xa:Checkpoint D1`, rider d). The mirror of
/// `e2e.rs::second_seed_reproduction_refusal` and the `run_loom` bless path. Returns the first
/// differing block's command and the pin remedy, or `None` when every block reproduced (or the
/// session declines — a declined case is the shell's to prove).
///
/// # Errors
/// Returns a case-materialization failure (never a decline — that is a `None`).
pub fn second_seed_reproduction_refusal(
    consumer: &DorcConsumer,
    case: &Case,
) -> Result<Option<String>, RunError> {
    let seed = dorc_testbed::run_seed::run_seed();
    let (Some(primary), Some(second)) = (
        block_outputs_under_seed(consumer, case, seed)?,
        block_outputs_under_seed(consumer, case, dorc_testbed::run_seed::second_seed(seed))?,
    ) else {
        return Ok(None);
    };
    Ok(primary
        .iter()
        .zip(&second)
        .zip(case.replay().blocks())
        .find(|((a, b), _)| a != b)
        .map(|((_, _), block)| {
            format!(
                "`{}` did not reproduce under a second seed, so publishing it would bake nondeterminism into a golden — pin the case by adding `$ export DORC_SEED=<n>` as its first session line",
                block.command()
            )
        }))
}

impl CaseRenderer for DorcConsumer {
    type Error = String;

    fn render_case(&self, case: &Case) -> Result<String, String> {
        let outputs = replay_case(case, self, &RunEnv::new(), |command, context| {
            Err(RunError::DriverDeclined {
                block: context.block(),
                command: command.to_owned(),
            })
        })
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(|result| result.output().to_owned())
        .collect();
        let mut regenerated = case.clone();
        regenerated.set_replay_outputs(outputs);
        // A rendered line that reads back as a txtar header would silently re-parse into a
        // DIFFERENT case, and the container has no escape for one. Without this the failure still
        // happened — as a bare "render-level fixpoint failed", which names neither the line nor the
        // reason (`28L:residue-a-wrapped-line-can-look-like-a-txtar-header`).
        regenerated.check_hygiene(None).map_err(|error| {
            format!(
                "{error}; a rendered line cannot also be a section header and the container has no \
                 escape for one -- reword the prose, or change the value the render interpolates, \
                 so the line does not both begin `-- ` and end ` --`"
            )
        })?;
        Ok(regenerated.to_text())
    }
}

fn editable_variables(
    render: &EditableRender<SectionKey, SectionVariableId>,
) -> Result<SectionVariables, String> {
    let mut variables = SectionVariables::new();
    for component in render.components() {
        let RenderComponent::EditableSection(section) = component else {
            continue;
        };
        for fragment in section.fragments() {
            let EditableFragment::Variable { id, rendered } = fragment else {
                continue;
            };
            let values = variables.entry(section.id().clone()).or_default();
            match values.entry(id.name.clone()) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(rendered.clone());
                }
                std::collections::btree_map::Entry::Occupied(entry) if entry.get() != rendered => {
                    return Err(format!(
                        "section {:?} renders `{}` as both {:?} and {:?}",
                        section.id(),
                        id.name.0,
                        entry.get(),
                        rendered
                    ));
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }
    }
    Ok(variables)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DorcSectionEditRefusal, compile_fragments, compile_preview, compile_section_edit,
        render_publish_diff,
    };
    use errorloom::{EditableFragment, EditableSection, RenderComponent};

    fn key(segment: usize) -> SectionKey {
        SectionKey {
            owner: String::from("code"),
            field: "message",
            instance: 0,
            segment,
        }
    }

    /// `sections` names a computed span, an editable section's key, and its fragment series —
    /// structure, not prose bytes: real catalog wording is free to churn without retouching this.
    #[test]
    fn sections_prints_computed_spans_and_editable_fragment_series() {
        let components = vec![
            RenderComponent::Structure("error[".to_owned()),
            RenderComponent::FixedVariable {
                id: SectionVariableId {
                    name: TemplateVariableName("code".to_owned()),
                    occurrence: 0,
                },
                rendered: "some-code".to_owned(),
            },
            RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    owner: "some-code".to_owned(),
                    field: "message",
                    instance: 0,
                    segment: 0,
                },
                vec![
                    EditableFragment::Text("do the ".to_owned()),
                    variable("thing", 0, "widget"),
                    EditableFragment::Text(" now".to_owned()),
                ],
            )),
        ];
        let mut out = String::new();
        for component in &components {
            write_component(&mut out, component);
        }
        assert!(out.contains("computed: \"error[\""), "{out}");
        // The annotation is the whole point of naming a fixed value at all: it is the only place
        // an author learns that a `{{name}}` they can see is one they cannot type.
        assert!(
            out.contains("computed {{code}} = \"some-code\" — foreign passthrough"),
            "{out}"
        );
        assert!(
            out.contains("section some-code/message#0 (segment 0):"),
            "{out}"
        );
        assert!(out.contains("text: \"do the \""), "{out}");
        assert!(out.contains("var {{thing}} = \"widget\""), "{out}");
        assert!(out.contains("text: \" now\""), "{out}");
    }

    fn variable(
        name: &str,
        occurrence: usize,
        rendered: &str,
    ) -> EditableFragment<SectionVariableId> {
        EditableFragment::Variable {
            id: SectionVariableId {
                name: TemplateVariableName(String::from(name)),
                occurrence,
            },
            rendered: String::from(rendered),
        }
    }

    fn baseline(
        components: Vec<RenderComponent<SectionKey, SectionVariableId>>,
    ) -> DorcEditableBaseline {
        let render = EditableRender::new(components);
        let variables = editable_variables(&render).unwrap_or_else(|error| panic!("{error}"));
        DorcEditableBaseline {
            render,
            variables,
            all_variables: BTreeMap::new(),
        }
    }

    #[test]
    fn editable_variables_preserve_empty_values_and_refuse_disagreement() {
        let key = SectionKey {
            owner: String::from("code"),
            field: "message",
            instance: 0,
            segment: 0,
        };
        let empty = EditableRender::new(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key.clone(),
                vec![EditableFragment::Variable {
                    id: SectionVariableId {
                        name: TemplateVariableName(String::from("name")),
                        occurrence: 0,
                    },
                    rendered: String::new(),
                }],
            ),
        )]);
        assert_eq!(
            editable_variables(&empty),
            Ok(BTreeMap::from([(
                key.clone(),
                BTreeMap::from([(TemplateVariableName(String::from("name")), String::new())]),
            )]))
        );

        let conflicting = EditableRender::new(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key,
                vec![
                    EditableFragment::Variable {
                        id: SectionVariableId {
                            name: TemplateVariableName(String::from("name")),
                            occurrence: 0,
                        },
                        rendered: String::from("left"),
                    },
                    EditableFragment::Variable {
                        id: SectionVariableId {
                            name: TemplateVariableName(String::from("name")),
                            occurrence: 1,
                        },
                        rendered: String::from("right"),
                    },
                ],
            ),
        )]);
        assert!(editable_variables(&conflicting).is_err());
    }

    #[test]
    fn marker_moves_command_after_preserved_path_identity() {
        let section = key(0);
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                section.clone(),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("path", 0, "/x"),
                    EditableFragment::Text(String::from(" using ")),
                    variable("command", 0, "apt-get"),
                ],
            ),
        )]);

        let edit = compile_section_edit(&baseline, "run {{command}} using {{path}}")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(edit.section(), &section);
        assert_eq!(edit.compiled().text(), "run apt-get using /x");
        assert_eq!(
            edit.compiled().used(),
            &[
                TemplateVariableName(String::from("command")),
                TemplateVariableName(String::from("path"))
            ]
        );
    }

    #[test]
    fn omission_removes_variable_and_its_surrounding_backticks() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("run `")),
                    variable("command", 0, "apt-get"),
                    EditableFragment::Text(String::from("` using ")),
                    variable("path", 0, "/x"),
                ],
            ),
        )]);
        let edit = compile_section_edit(&baseline, "run using /x")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(edit.compiled().text(), "run using /x");
        assert_eq!(
            edit.compiled().used(),
            &[TemplateVariableName(String::from("path"))]
        );
    }

    /// A single newline is a soft wrap and stores as a space (`282` §3).
    #[test]
    fn a_wrapped_register_edit_stores_no_literal_newline() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![EditableFragment::Text(String::from("run the thing now"))],
            ),
        )]);
        let edit = compile_section_edit(&baseline, "run the\nthing now")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(edit.compiled().text(), "run the thing now");
    }

    /// Read-in normalization trims trailing whitespace/newline off a compiled register — layout
    /// is the renderer's, never the stored template's (`282` §3).
    #[test]
    fn trailing_whitespace_and_newline_trim_off_a_compiled_register() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![EditableFragment::Text(String::from("run the thing"))],
            ),
        )]);
        let edit = compile_section_edit(&baseline, "run the thing now  \n")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(edit.compiled().text(), "run the thing now");
    }

    #[test]
    fn explicit_markers_can_duplicate_and_replace_every_repeated_name() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("command", 0, "apt-get"),
                    EditableFragment::Text(String::from(" then ")),
                    variable("command", 1, "apt-get"),
                ],
            ),
        )]);
        let edit = compile_section_edit(
            &baseline,
            "run {{command}} then {{command}} again {{command}}",
        )
        .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(
            edit.compiled().text(),
            "run apt-get then apt-get again apt-get"
        );
        assert_eq!(
            edit.compiled().used(),
            &[TemplateVariableName(String::from("command"))]
        );
    }

    #[test]
    fn repeated_equal_values_keep_their_existing_identity_without_markers() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("from ")),
                    variable("left", 0, "same"),
                    EditableFragment::Text(String::from(" to ")),
                    variable("right", 0, "same"),
                ],
            ),
        )]);
        let edit = compile_section_edit(&baseline, "copy from same to same")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(
            edit.compiled().used(),
            &[
                TemplateVariableName(String::from("left")),
                TemplateVariableName(String::from("right"))
            ]
        );
    }

    #[test]
    fn empty_and_nul_values_survive_marker_interpretation() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("values ")),
                    variable("empty", 0, ""),
                    EditableFragment::Text(String::from(" ")),
                    variable("nul", 0, "\0"),
                ],
            ),
        )]);
        let edit = compile_section_edit(&baseline, "values {{empty}} {{nul}}")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(edit.compiled().text(), "values  \0");
    }

    #[test]
    fn markers_can_be_the_entire_first_or_last_section_content() {
        let entire = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(key(0), vec![variable("command", 0, "apt-get")]),
        )]);
        let first = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    variable("command", 0, "apt-get"),
                    EditableFragment::Text(String::from(" later")),
                ],
            ),
        )]);
        let last = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("earlier ")),
                    variable("command", 0, "apt-get"),
                ],
            ),
        )]);

        for (baseline, dirty) in [
            (entire, "{{command}}"),
            (first, "{{command}} later"),
            (last, "earlier {{command}}"),
        ] {
            let edit =
                compile_section_edit(&baseline, dirty).unwrap_or_else(|error| panic!("{error:?}"));
            assert_eq!(
                edit.compiled().text(),
                dirty.replace("{{command}}", "apt-get")
            );
        }
    }

    #[test]
    fn malformed_and_unknown_markers_refuse_structurally() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("command", 0, "apt-get"),
                ],
            ),
        )]);
        assert!(matches!(
            compile_section_edit(&baseline, "run {{command}"),
            Err(DorcSectionEditRefusal::Template(_))
        ));
        assert!(matches!(
            compile_section_edit(&baseline, "run {{unknown}}"),
            Err(DorcSectionEditRefusal::UnknownVariable(_))
        ));
        assert_eq!(
            compile_section_edit(&baseline, "run ({{command}})").map(|edit| edit.compiled().text()),
            Ok(String::from("run (apt-get)")),
            "a marker glued to punctuation is an ordinary marker"
        );
    }

    #[test]
    fn structure_markers_and_split_fields_do_not_license_an_edit() {
        let section = EditableSection::new(
            key(0),
            vec![EditableFragment::Text(String::from(" editable"))],
        );
        let structure = baseline(vec![
            RenderComponent::Structure(String::from("before structure")),
            RenderComponent::EditableSection(section),
        ]);
        let structure_result =
            compile_section_edit(&structure, "before {{name}} structure editable");
        assert!(
            matches!(
                structure_result,
                Err(DorcSectionEditRefusal::MarkerOutsideEditableSection)
            ),
            "{structure_result:?}"
        );

        let shared_boundary = baseline(vec![
            RenderComponent::EditableSection(EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("command", 0, "apt-get"),
                ],
            )),
            RenderComponent::EditableSection(EditableSection::new(
                key(1),
                vec![EditableFragment::Text(String::from(" always"))],
            )),
        ]);
        let shared_boundary_result =
            compile_section_edit(&shared_boundary, "run {{command}} always");
        assert!(
            matches!(
                shared_boundary_result,
                Err(DorcSectionEditRefusal::SplitEditableField(_))
            ),
            "{shared_boundary_result:?}"
        );
    }

    #[test]
    fn unchanged_transcripts_refuse_explicitly() {
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![EditableFragment::Text(String::from("unchanged"))],
            ),
        )]);
        assert_eq!(
            compile_section_edit(&baseline, "unchanged"),
            Err(DorcSectionEditRefusal::Unchanged)
        );
    }

    #[test]
    fn preview_replaces_each_changed_section_in_renderer_order() {
        let message = key(0);
        let baseline = baseline(vec![
            RenderComponent::Structure(String::from("message: ")),
            RenderComponent::EditableSection(EditableSection::new(
                message.clone(),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("path", 0, "/x"),
                    EditableFragment::Text(String::from(" using ")),
                    variable("command", 0, "apt-get"),
                ],
            )),
            RenderComponent::Structure(String::from("\nhelp: ")),
            RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    field: "help",
                    segment: 1,
                    ..message.clone()
                },
                vec![EditableFragment::Text(String::from("unchanged help"))],
            )),
            RenderComponent::FixedVariable {
                id: SectionVariableId {
                    name: TemplateVariableName(String::from("foreign")),
                    occurrence: 0,
                },
                rendered: String::from(" [foreign]"),
            },
        ]);

        let preview = compile_preview(
            &baseline,
            "message: run {{command}} using {{path}}\nhelp: changed help [foreign]",
        )
        .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(preview.sections().len(), 2);
        assert_eq!(preview.sections()[0].section(), &message);
        assert_eq!(preview.sections()[1].section().field, "help");
        assert_eq!(
            preview.concrete(),
            "message: run apt-get using /x\nhelp: changed help [foreign]"
        );
        assert!(!preview.concrete().contains("{{"));
        assert_eq!(
            preview.sections()[0].used_bindings(),
            &[
                (
                    TemplateVariableName(String::from("command")),
                    String::from("apt-get")
                ),
                (
                    TemplateVariableName(String::from("path")),
                    String::from("/x")
                ),
            ]
        );
    }

    #[test]
    fn preview_refuses_the_whole_render_when_later_section_compilation_fails() {
        let message = key(0);
        let baseline = baseline(vec![
            RenderComponent::EditableSection(EditableSection::new(
                message.clone(),
                vec![variable("command", 0, "apt-get")],
            )),
            RenderComponent::Structure(String::from("\nhelp: ")),
            RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    field: "help",
                    segment: 1,
                    ..message
                },
                vec![EditableFragment::Text(String::from("original"))],
            )),
        ]);

        assert!(matches!(
            compile_preview(&baseline, "{{command}}\nhelp: {{unknown}}"),
            Err(DorcSectionEditRefusal::UnknownVariable(_))
        ));
    }

    #[test]
    fn preview_keeps_exact_bindings_through_duplication_removal_and_nul() {
        let duplicate = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("command", 0, "apt-get"),
                    EditableFragment::Text(String::from(" using ")),
                    variable("path", 0, "/x"),
                ],
            ),
        )]);
        let duplicate = compile_preview(
            &duplicate,
            "run {{command}} then {{command}} using {{path}}",
        )
        .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(duplicate.concrete(), "run apt-get then apt-get using /x");

        let omitted = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("run `")),
                    variable("command", 0, "apt-get"),
                    EditableFragment::Text(String::from("` using ")),
                    variable("path", 0, "/x"),
                ],
            ),
        )]);
        let omitted =
            compile_preview(&omitted, "run using /x").unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(omitted.concrete(), "run using /x");
        assert_eq!(
            omitted.sections()[0].used_bindings(),
            &[(
                TemplateVariableName(String::from("path")),
                String::from("/x")
            )]
        );

        let exact = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                key(0),
                vec![
                    EditableFragment::Text(String::from("values ")),
                    variable("empty", 0, ""),
                    EditableFragment::Text(String::from(" ")),
                    variable("nul", 0, "\0"),
                ],
            ),
        )]);
        let exact = compile_preview(&exact, "values {{empty}} {{nul}}")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(exact.concrete(), "values  \0");
        assert_eq!(
            exact.sections()[0].used_bindings(),
            &[
                (TemplateVariableName(String::from("empty")), String::new()),
                (
                    TemplateVariableName(String::from("nul")),
                    String::from("\0")
                ),
            ]
        );
    }

    /// The backslash-and-quote path is the point: these transcripts are full of `"$@"` and
    /// `%LOCALAPPDATA%\dorc`, and a `{:?}` render doubles every one of them. Only a control
    /// character may be escaped, or the view stops being readable at exactly the corpus we have.
    ///
    /// And the diff is the whole reason this render exists: a hole that MOVED renders the same
    /// concrete bytes as one that DIED, so only the two template spellings, side by side, tell an
    /// author which of the two their edit did.
    #[test]
    fn the_diff_shows_only_the_touched_section_in_template_spelling() {
        let message = key(0);
        let baseline = baseline(vec![
            RenderComponent::Structure(String::from("message: ")),
            RenderComponent::EditableSection(EditableSection::new(
                message.clone(),
                vec![
                    EditableFragment::Text(String::from("run ")),
                    variable("path", 0, "%LOCALAPPDATA%\\dorc"),
                    EditableFragment::Text(String::from(" using ")),
                    variable("command", 0, "\"$@\""),
                ],
            )),
            RenderComponent::Structure(String::from("\nhelp: ")),
            RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    field: "help",
                    segment: 1,
                    ..message
                },
                vec![variable("unused", 0, "hidden")],
            )),
            RenderComponent::FixedVariable {
                id: SectionVariableId {
                    name: TemplateVariableName(String::from("foreign")),
                    occurrence: 0,
                },
                rendered: String::from(" [foreign]"),
            },
        ]);
        let preview = compile_preview(
            &baseline,
            "message: run {{command}} using {{path}}\nhelp: hidden [foreign]",
        )
        .unwrap_or_else(|error| panic!("{error:?}"));

        let interpretation = render_publish_diff(&preview);
        assert_eq!(
            interpretation,
            "section: code.message#0:0\n  - run {{path}} using {{command}}\n  + run {{command}} using {{path}}"
        );
        // The load-bearing negative: NO side of the diff spells a rendered value. Both templates
        // interpolate the same two, so a diff that flattened them would be two identical lines and
        // the reorder would be invisible — which is the whole failure this render exists to stop.
        assert!(!interpretation.contains("$@"), "{interpretation}");
        assert!(!interpretation.contains("LOCALAPPDATA"), "{interpretation}");
        // An untouched section and a fixed variable are another render's business; showing them
        // here would bury the one thing this view exists to expose.
        assert!(!interpretation.contains("hidden"));
        assert!(!interpretation.contains("foreign"));
    }

    #[test]
    fn applying_compiled_markers_preserves_duplicate_empty_and_nul_variables() {
        let section = SectionKey {
            owner: String::from("dangling-reference"),
            field: "message",
            instance: 0,
            segment: 0,
        };
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                section.clone(),
                vec![
                    variable("empty", 0, ""),
                    EditableFragment::Text(String::from(" ")),
                    variable("nul", 0, "\0"),
                    EditableFragment::Text(String::from(" remove-me ")),
                    variable("removed", 0, "gone"),
                ],
            ),
        )]);
        let edit = compile_section_edit(&baseline, "{{nul}} {{empty}} {{nul}}")
            .unwrap_or_else(|error| panic!("{error:?}"));
        let mut consumer = DorcConsumer::new();

        consumer
            .apply_section_edit(&edit)
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(
            consumer
                .mirror()
                .iter()
                .find(|entry| entry.slug == "dangling-reference")
                .and_then(|entry| entry.message.as_ref())
                .map(|tier| tier.text().as_str()),
            Some("{{nul}} {{empty}} {{nul}}")
        );
    }

    #[test]
    fn applying_missing_code_or_illegal_field_leaves_the_mirror_unchanged() {
        let mut consumer = DorcConsumer::new();
        let before = consumer.mirror().to_vec();
        let compiled =
            compile_fragments(&[], &BTreeMap::new()).unwrap_or_else(|error| panic!("{error:?}"));

        assert_eq!(
            consumer.apply_compiled_section(
                &SectionKey {
                    owner: String::from("missing-code"),
                    field: "message",
                    instance: 0,
                    segment: 0,
                },
                &compiled,
            ),
            Err(DorcApplyRefusal::MissingCode(String::from("missing-code")))
        );
        assert_eq!(consumer.mirror(), before);

        assert_eq!(
            consumer.apply_compiled_section(
                &SectionKey {
                    owner: String::from("dangling-reference"),
                    field: "when_fires",
                    instance: 0,
                    segment: 0,
                },
                &compiled,
            ),
            Err(DorcApplyRefusal::IllegalField("when_fires"))
        );
        assert_eq!(consumer.mirror(), before);
    }

    /// `28H` ruling 3's CONDITION, and the reason the split-field relaxation is safe for chrome
    /// lines: several sections may share one registry entry, so two edits in ONE transcript must
    /// agree or the apply refuses. Last-wins is the failure this pins against — it drops one of
    /// the author's two rewrites silently, and nothing downstream would ever say so. The
    /// same-words case is the ordinary one (repeated chrome, rewritten consistently) and must
    /// still land, or consistent editing would become impossible.
    #[test]
    fn two_edits_to_one_shared_entry_must_agree() {
        let shared = |segment: usize| SectionKey {
            owner: String::from("why-receipt-book-drifted"),
            field: crate::ARRANGEMENT_LINE_FIELD,
            instance: 0,
            segment,
        };
        let compiled = |text: &str| {
            compile_fragments(
                &[EditableFragment::Text(String::from(text))],
                &BTreeMap::new(),
            )
            .unwrap_or_else(|error| panic!("{error:?}"))
        };
        let preview = |first: &str, second: &str| crate::CompilePreview {
            sections: vec![
                crate::SectionPreview {
                    section: shared(0),
                    compiled: compiled(first),
                    used_bindings: Vec::new(),
                    dropped: Vec::new(),
                    stamped: String::new(),
                },
                crate::SectionPreview {
                    section: shared(4),
                    compiled: compiled(second),
                    used_bindings: Vec::new(),
                    dropped: Vec::new(),
                    stamped: String::new(),
                },
            ],
            concrete: String::new(),
        };

        let mut consumer = DorcConsumer::new();
        let before = consumer.arrangements().to_vec();
        assert_eq!(
            consumer.apply_preview(&preview("one thing", "another thing")),
            Err(DorcApplyRefusal::ArrangementEntryEditedTwice {
                slug: String::from("why-receipt-book-drifted"),
                first: vec![String::from("one thing")],
                second: vec![String::from("another thing")],
            })
        );
        assert_eq!(
            consumer.arrangements(),
            before,
            "a refusal writes nothing: the pre-pass runs before the first apply"
        );

        consumer
            .apply_preview(&preview("agreed words", "agreed words"))
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(
            consumer
                .arrangements()
                .iter()
                .find(|entry| entry.slug == "why-receipt-book-drifted")
                .map(|entry| entry.words.clone()),
            Some(Some(ProseTier::Slop(vec![String::from("agreed words")]))),
        );
    }

    #[test]
    fn two_wrapped_rows_compile_back_without_layout_whitespace() {
        use dorc_aid::weave::{Face, to_render_parts, words};
        use weft::{Document, Node, NodeKind, Paragraph, render};

        let row = |slug: &str, text: &str| OwnedArrangement {
            slug: slug.to_owned(),
            occurrence: None,
            when_used: "harness".to_owned(),
            why: "harness".to_owned(),
            words: Some(ProseTier::Migrated(vec![text.to_owned()])),
        };
        let mut consumer = DorcConsumer {
            mirror: Vec::new(),
            arrangements: vec![
                row("first-row", "first row has enough words to wrap"),
                row("second-row", "second row has enough words to wrap"),
            ],
            mint: Mint::Slop,
            demoted: Vec::new(),
        };
        let document = Document::new(vec![
            Node::new(NodeKind::Prose(Paragraph {
                runs: vec![words(
                    "first row has enough words to wrap",
                    "first-row",
                    None,
                )],
            })),
            Node::new(NodeKind::Prose(Paragraph {
                runs: vec![words(
                    "second row has enough words to wrap",
                    "second-row",
                    None,
                )],
            })),
        ]);
        let rendered = render::<Face>(&document, 24);
        assert!(rendered.text().matches('\n').count() > 2);
        let case = Case::parse("---\narrangement: first-row\n---\n-- replay --\n$ harness\n")
            .unwrap_or_else(|error| panic!("{error}"));
        let baseline = consumer
            .baseline_from_render(&case, to_editable_render(&to_render_parts(&rendered)))
            .unwrap_or_else(|error| panic!("{error}"));
        let dirty = rendered
            .text()
            .replace("first row", "edited first row")
            .replace("second row", "edited second row");
        let preview =
            compile_preview(&baseline, &dirty).unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(preview.sections().len(), 2);

        consumer
            .apply_preview(&preview)
            .unwrap_or_else(|error| panic!("{error:?}"));
        let stored: Vec<&str> = consumer
            .arrangements()
            .iter()
            .filter_map(|entry| entry.words.as_ref())
            .flat_map(|tier| tier.text().iter().map(String::as_str))
            .collect();
        assert_eq!(
            stored,
            [
                "edited first row has enough words to wrap",
                "edited second row has enough words to wrap",
            ]
        );
    }

    #[test]
    fn split_editable_fields_refuse_every_segment_without_conflating_other_fields() {
        let split = SectionKey {
            owner: String::from("code"),
            field: "message",
            instance: 0,
            segment: 0,
        };
        let split_tail = SectionKey {
            segment: 1,
            ..split.clone()
        };
        let split_baseline = baseline(vec![
            RenderComponent::EditableSection(EditableSection::new(
                split.clone(),
                vec![
                    EditableFragment::Text(String::from("prefix ")),
                    variable("left", 0, "left"),
                ],
            )),
            RenderComponent::FixedVariable {
                id: SectionVariableId {
                    name: TemplateVariableName(String::from("foreign")),
                    occurrence: 0,
                },
                rendered: String::from(" fixed "),
            },
            RenderComponent::EditableSection(EditableSection::new(
                split_tail.clone(),
                vec![
                    EditableFragment::Text(String::from("suffix ")),
                    variable("right", 0, "right"),
                ],
            )),
        ]);
        assert_eq!(
            compile_section_edit(&split_baseline, "changed left fixed suffix right"),
            Err(DorcSectionEditRefusal::SplitEditableField(split.clone()))
        );
        assert_eq!(
            compile_section_edit(&split_baseline, "{{left}} fixed suffix right"),
            Err(DorcSectionEditRefusal::SplitEditableField(split))
        );
        assert_eq!(
            compile_section_edit(&split_baseline, "prefix left fixed changed right"),
            Err(DorcSectionEditRefusal::SplitEditableField(split_tail))
        );

        let first = key(0);
        let second = SectionKey {
            field: "help",
            segment: 1,
            ..first.clone()
        };
        let other_instance = SectionKey {
            instance: 1,
            segment: 2,
            ..first.clone()
        };
        let unsplit = baseline(vec![
            RenderComponent::EditableSection(EditableSection::new(
                first,
                vec![EditableFragment::Text(String::from("first"))],
            )),
            RenderComponent::Structure(String::from("|")),
            RenderComponent::EditableSection(EditableSection::new(
                second,
                vec![EditableFragment::Text(String::from("help"))],
            )),
            RenderComponent::Structure(String::from("|")),
            RenderComponent::EditableSection(EditableSection::new(
                other_instance,
                vec![EditableFragment::Text(String::from("second"))],
            )),
        ]);
        assert!(compile_section_edit(&unsplit, "changed|help|second").is_ok());
    }
}
