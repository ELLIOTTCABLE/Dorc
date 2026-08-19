//! The Dorc case renderer and compiled-edit applier (`282` §5 · §13), implemented against a mutable
//! owned-catalog mirror ([`dorc_aid::catalog::OwnedEntry`]).
//!
//! World-form dispatch (`283:dec-world-two-forms`): a `-- world --`-only case is WORLD-AS-PAYLOAD (a
//! canonical constructor keyed by slug — the phase-4 floor for the artificial/expensive-world codes);
//! a case carrying a materialized oracle/book section is WORLD-AS-PIPELINE (the real in-process kernel
//! fires the diagnostic — the marker pilot). Phase 4 lands the payload path; the pipeline arm is the
//! marker-version-unrecognized pilot.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;

use dorc_aid::arrangement::{OwnedArrangement, arrangement_parts, owned_arrangements};
use dorc_aid::catalog::{HelpRegister, OwnedEntry, owned_catalog, parse_template};
use dorc_aid::diag::{Diag, DiagCode, render_cli_parts, render_staged_cli_parts};
use dorc_aid::prose::{Mint, ProseTier};
use dorc_aid::{RenderCtx, Severity};
use dorc_core::{Interner, ProvArena};
use errorloom::{
    Case, CaseRenderer, EditableFragment, EditableRender, RenderComponent, ReplayContext,
    ReplayDriver, ReplayInput, ReplayResult, RunEnv, RunError, drive_case, drive_case_with_inputs,
};

use crate::invocation::{Breadth, Target, Verb};
use crate::usage::{self, PROGRAM, Reading};
use crate::{
    DorcSectionEdit, ENVELOPE_INVOCATION, ENVELOPE_KEY, ENVELOPE_STDERR, SectionKey,
    SectionVariableId, TemplateVariableName, to_editable_render,
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
    /// The selected section is neither a catalog prose field nor the arrangement register.
    IllegalField(&'static str),
    /// An edit to a whole-PAGE arrangement carried a `{{name}}` variable, or the entry it lands
    /// on holds a word SEQUENCE. A page is one entry, laid out by its author; neither a template
    /// hole nor a re-split has any meaning there (`289` §2o: the registry never grows grammar
    /// machinery).
    ArrangementTakesNoVariables(String),
    /// The edited entry holds a WORD SEQUENCE where the page path expects one word.
    ArrangementIsSequenceStructured(String),
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
        if key.field == crate::ARRANGEMENT_FIELD {
            return self.apply_arrangement_page_edit(key, compiled);
        }
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

    /// Apply one compiled whole-PAGE arrangement section to the registry mirror.
    ///
    /// A page is one entry laid out by its author, so it takes no values and its bytes survive
    /// verbatim: no whitespace normalization, no re-split (`28H` ruling 7).
    fn apply_arrangement_page_edit(
        &mut self,
        key: &SectionKey,
        compiled: &crate::CompiledSection,
    ) -> Result<(), DorcApplyRefusal> {
        if compiled
            .fragments()
            .iter()
            .any(|fragment| matches!(fragment, crate::CompiledFragment::Variable(_)))
        {
            return Err(DorcApplyRefusal::ArrangementTakesNoVariables(
                key.owner.clone(),
            ));
        }
        let words = page_words(compiled);
        let mint = self.mint;
        let entry = self.arrangement_entry(key)?;
        if entry
            .words
            .as_ref()
            .is_some_and(|current| current.text().len() > 1)
        {
            return Err(DorcApplyRefusal::ArrangementIsSequenceStructured(
                key.owner.clone(),
            ));
        }
        let demoted = mint.demotes(entry.words.as_ref());
        entry.words = Some(mint.tier(words));
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

    /// One diagnostic's part stream at the corpus's canonical width.
    ///
    /// The ONE seat both answers come from — the provenance answer [`Self::replay`] hands the
    /// transport, and the bytes [`Self::render_direct_replay`] commits. Two seats is how a
    /// transcript stops being what the renderer printed
    /// (`28L:rul-editability-is-stamped-never-re-derived`).
    fn cli_parts(&self, diag: &Diag, src: &str, filename: &str) -> dorc_aid::tagged::RenderParts {
        render_cli_parts(
            &self.render_ctx(),
            diag,
            src,
            filename,
            &Interner::default(),
        )
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

    /// The plan route's stderr ENVELOPE, when the case opted into it with `envelope: stderr`.
    ///
    /// The three lines a `dorc plan` closes with are chrome around an ARTIFACT — no diagnostic
    /// carries them, so no ordinary case ever renders them and their registry entries had no
    /// editable home. The key is what a case says to be handed the whole stderr envelope instead
    /// (`28L`, the X2a plan-stderr trio ruling). Oracles come from the case's own `*.oracle.sh`
    /// sections, in section order, exactly as an admin's `-o` list would.
    fn plan_envelope(&self, case: &Case, book: &str) -> Option<dorc_aid::tagged::RenderParts> {
        if case.frontmatter().scalar(ENVELOPE_KEY) != Some(ENVELOPE_STDERR) {
            return None;
        }
        let source = section_source(case, book)?;
        let oracles: Vec<(String, String)> = case
            .sections()
            .iter()
            .filter(|section| section.name().ends_with(".oracle.sh"))
            .map(|section| (section.name().to_owned(), section.content().to_owned()))
            .collect();
        let paths: Vec<String> = oracles.iter().map(|(path, _)| path.clone()).collect();
        let sources: Vec<String> = oracles.into_iter().map(|(_, source)| source).collect();
        // The case is a FLAT virtual directory: its section names are the paths, so the modeled
        // cwd is the root of that directory and every `./x.oracle.sh` names a section
        // (`30I:rul-dot-resolves-as-sh`). The e2e runner materializes the same case into a real
        // directory and runs the real binary there, so the two routes agree by construction.
        let world =
            dorc_cli::world::WhyWorld::analyze(&case_snapshot(book, source, paths, sources));
        Some(dorc_cli::plan_envelope_parts(
            &self.render_ctx(),
            &world,
            book,
        ))
    }

    /// The seat a case DECLARES, when the command shape would pick the wrong one. A code `run`
    /// returns as `Err` prints through the invocation seat whatever the invocation was, so a
    /// plan-shaped replay under-shows the prefix and the synopsis that come with it.
    fn declared_seat_parts(
        &self,
        case: &Case,
        diag: &Diag,
    ) -> Option<dorc_aid::tagged::RenderParts> {
        (case.frontmatter().scalar(ENVELOPE_KEY) == Some(ENVELOPE_INVOCATION))
            .then(|| self.invocation_parts(diag, "dorc"))
    }

    /// [`Self::cli_parts`] for a source-staged diagnostic.
    fn staged_cli_parts(&self, stage: &str, diag: &Diag) -> dorc_aid::tagged::RenderParts {
        render_staged_cli_parts(
            stage,
            &self.render_ctx(),
            diag,
            "",
            "",
            &Interner::default(),
        )
    }

    /// The editable baseline of a case's FIRST replay — what `dorc-loom vars` reports.
    ///
    /// It drives the case exactly as `publish` does rather than re-deriving a world of its own
    /// (`_loom-final-map` §2c): a second derivation answered only for the plain diagnostic shape, so
    /// a whylog, lint, or invocation-error case got a different render — or none — from the one an
    /// edit actually compiles against, and an inventory that disagrees with the compiler is worse
    /// than no inventory.
    ///
    /// # Errors
    /// Returns the replay refusal, or names the case whose first replay carries no editable prose.
    pub fn editable_baseline(&self, case: &Case) -> Result<DorcEditableBaseline, String> {
        // The generation lag, stated before the driver can only shrug about it: a case naming a
        // slug with no committed row renders nothing, and the honest answer names the repair.
        if let Some(slug) = case.frontmatter().scalar("arrangement") {
            Self::arrangement_row(slug)?;
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
    ) -> Option<String> {
        let invocation = match usage::read(tokens.get(1..)?) {
            Reading::Help(page) => return Some(format!("{page}\n")),
            Reading::Refused(refusal) => return Some(format!("{PROGRAM}: {refusal}\n")),
            Reading::Runs(invocation) => *invocation,
        };
        if invocation.root.is_some() {
            return None;
        }
        let wanted = match &invocation.verb {
            Verb::Vars(args) => Inventory::Vars(args.breadth()),
            Verb::Sections(_) => Inventory::Sections,
            _ => return None,
        };
        match invocation.target().ok()? {
            Target::This => {
                if self_reference == SelfReference::Forbidden {
                    return None;
                }
                self_slug(case)?;
                self.inventory(wanted, case)
            }
            Target::Named([one]) if case_relative_path(one) => {
                let target = Case::parse(&source_of(one)?).ok()?;
                self.inventory(wanted, &target)
            }
            Target::Named(_) => None,
        }
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
        self.replay_within(case, command, context, SelfReference::Allowed)
    }

    fn replay_within(
        &self,
        case: &Case,
        command: &str,
        context: &ReplayContext<'_>,
        self_reference: SelfReference,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        let tokens = exact_words(command)?;
        if let Some(slug) = arrangement_page_slug(case, &tokens) {
            let parts = self.arrangement_page(slug).ok()?;
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if tokens.first() == Some(&LOOM_COMMAND) {
            return self
                .loom_replay(case, &tokens, self_reference, &|target| {
                    context.materialized_input(target).map(str::to_owned)
                })
                .map(ReplayResult::bytes);
        }
        if tokens.as_slice() == ["dorc", "lint", "--list-sources"] {
            let parts = dorc_cli::lint_sources_parts(&self.render_ctx());
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if let Some(why) = parse_direct_why_report(&tokens) {
            let parts = live_why_parts(&self.render_ctx(), &why, |path| {
                materialized_source(case, context, path)
            })
            .ok()?;
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if let Some(why) = parse_direct_why(&tokens) {
            let raw = context.materialized_input(why.whylog);
            if let Some(parts) = raw.and_then(|whylog| {
                drifted_why_parts(&self.render_ctx(), whylog, why.address, |path| {
                    materialized_source(case, context, path)
                })
            }) {
                return Some(ReplayResult::editable(to_editable_render(&parts)));
            }
            if why.address.is_some() {
                return None;
            }
            let book = materialized_source(case, context, "book.sh");
            let inspected = dorc_plan::whylog::inspect(raw, why.whylog, book.as_deref(), |path| {
                materialized_source(case, context, path)
            });
            let diag = inspected.into_iter().next()?;
            let parts = self.staged_cli_parts("whylog", &diag);
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if let Some((path, tools_enabled)) = parse_direct_lint(&tokens) {
            let source = materialized_source(case, context, path)?;
            let result = dorc_lint::lint_materialized_source(
                path.to_owned(),
                source,
                dorc_lint::SourcePolicy { tools_enabled },
            );
            return Some(ReplayResult::editable(to_editable_render(
                &result.human(&self.render_ctx()),
            )));
        }
        if let Some(diag) = fire_invocation_error(case, &tokens) {
            let parts = self.invocation_parts(&diag, tokens.first().copied().unwrap_or_default());
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if parse_direct_remote_apply(&tokens) {
            let diag = Self::world_of(case).ok()?.0;
            let parts = self
                .declared_seat_parts(case, &diag)
                .unwrap_or_else(|| self.cli_parts(&diag, "", ""));
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        let plan = parse_direct_plan(&tokens)?;
        if let Some(parts) = self.plan_envelope(case, plan.book) {
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        self.replay_plan(case, context, &plan)
    }

    /// The `dorc plan` arm of [`Self::replay`], split out because the dispatch above is a table and
    /// this one arm carries the whole world derivation.
    fn replay_plan(
        &self,
        case: &Case,
        context: &ReplayContext<'_>,
        plan: &DirectPlan<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        // World-as-payload, the branch `render_direct_replay` has always had. Without it the
        // driver declined, so no publish ever saw provenance for these cases.
        let Some(source) = materialized_source(case, context, plan.book) else {
            let diag = Self::world_of(case).ok()?.0;
            if plan.machine {
                return Some(ReplayResult::bytes(render_diag_jsonl(&diag)));
            }
            let parts = self
                .declared_seat_parts(case, &diag)
                .unwrap_or_else(|| self.cli_parts(&diag, "", ""));
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        };
        let oracles: Vec<(String, String)> = plan
            .oracles
            .iter()
            .map(|path| {
                Some((
                    (*path).to_owned(),
                    materialized_source(case, context, path)?,
                ))
            })
            .collect::<Option<_>>()?;
        let results = match plan.input {
            Some(input) => Some(materialized_input(case, context, input)?),
            None => None,
        };
        // The FRAME source is the world's, not the book's: an oracle-side diagnostic's caret points
        // into the oracle file that raised it.
        let (diag, framed, filename) = Self::world_of_source(
            case,
            plan.book,
            &source,
            &oracles,
            plan.consented,
            results.as_deref(),
        )
        .ok()?;
        if plan.machine {
            return Some(ReplayResult::bytes(render_diag_jsonl(&diag)));
        }
        let parts = self.cli_parts(&diag, &framed, &filename);
        Some(ReplayResult::editable(to_editable_render(&parts)))
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
        let diag = Self::world_of(case)
            .map(|(diag, _, _)| diag)
            .or_else(|_| Self::whylog_diagnostic(case))?;
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

    /// The COMMITTED registry's own spelling of `slug`, so a span carries a stable one. A case
    /// naming a slug with no row yet gets the repair: its row arrives by promotion and the build
    /// sees it after a rebuild — the same generation lag the catalog has, and the same assertion.
    ///
    /// EXISTENCE only. A value-bearing chrome LINE has no whole-page render at all — laying one
    /// out passes zero values to a seat that interleaves several — so the lag check cannot go
    /// through [`Self::arrangement_page`].
    fn arrangement_row(slug: &str) -> Result<&'static str, String> {
        dorc_aid::arrangement::ARRANGEMENTS
            .iter()
            .find(|entry| entry.slug == slug)
            .map(|entry| entry.slug)
            .ok_or_else(|| {
                format!(
                    "arrangement `{slug}` has no registry row yet -- publish the case, then rebuild"
                )
            })
    }

    /// One whole-page arrangement's part stream, resolved against the COMMITTED registry.
    fn arrangement_page(&self, slug: &str) -> Result<dorc_aid::tagged::RenderParts, String> {
        Ok(arrangement_parts(
            &self.arrangements,
            Self::arrangement_row(slug)?,
            None,
        ))
    }

    /// The defining replay's typed diagnostic for a case — the payload the generated `example` field
    /// and the full inventory read (`28A` §4). World-as-payload/pipeline, with the whylog durable
    /// fallback for `dorc why --last` cases.
    ///
    /// # Errors
    /// Returns the case-world or whylog-provenance refusal.
    pub fn case_diag(&self, case: &Case) -> Result<Diag, String> {
        Self::world_of(case)
            .map(|(diag, _, _)| diag)
            .or_else(|_| Self::whylog_diagnostic(case))
    }

    /// The (diag, source, filename) a case materializes into (`283:dec-world-two-forms`). A case
    /// carrying a materialized `*.oracle.sh` section is WORLD-AS-PIPELINE: the REAL in-process marker
    /// gate fires the diagnostic over that source (the one real-fired proof, `28A` §2n) — a spanned
    /// diag whose caret frame points into it. Otherwise WORLD-AS-PAYLOAD: the canonical constructor
    /// keyed by the frontmatter `code` (spanless roster codes need no source).
    fn world_of(case: &Case) -> Result<(Diag, String, String), String> {
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        // The BOOK route first: a case carrying both a book and oracles is a `dorc plan` world,
        // and its oracle sections are that run's loaded set rather than a lint target. The
        // invocation's own flag and `< results` redirect are read off its first replay, so a
        // worldless derivation answers the same world the driven one does.
        let (consented, results) = declared_plan_shape(case);
        if let Some(section) = case.sections().iter().find(|s| s.name() == "book.sh")
            && let Ok(world) = fire_book_analysis(
                slug,
                section.name(),
                section.content(),
                &oracle_sections(case),
                consented,
                results,
            )
        {
            return Ok(world);
        }
        if let Some(section) = case
            .sections()
            .iter()
            .find(|s| s.name().ends_with("oracle.sh"))
        {
            return fire_lint_case(
                slug,
                section.name(),
                section.content(),
                declared_lint_tools(case),
            );
        }
        if let Some((book, results)) = declared_plan_inputs(case)
            && let Ok(diag) = fire_records_admission(slug, book, results)
        {
            return Ok((diag, String::new(), String::new()));
        }
        // Tried BEFORE the payload floor, so a code that can fire for real never settles for a
        // constructed stand-in (`289:rul-worldless-route-honest-trigger`).
        if let Some(diag) = case
            .replay()
            .blocks()
            .first()
            .and_then(|block| exact_words(block.command()))
            .and_then(|tokens| fire_invocation_error(case, &tokens))
        {
            return Ok((diag, String::new(), String::new()));
        }
        if let Ok(diag) = Self::whylog_diagnostic(case) {
            return Ok((diag, String::new(), String::new()));
        }
        let diag = canonical_payload(slug)
            .ok_or_else(|| format!("no canonical world for `{slug}` (world-as-payload)"))?;
        Ok((diag, String::new(), String::new()))
    }

    fn world_of_source(
        case: &Case,
        path: &str,
        source: &str,
        oracles: &[(String, String)],
        consented: bool,
        results: Option<&str>,
    ) -> Result<(Diag, String, String), String> {
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        if path.ends_with("oracle.sh") {
            let (diag, _, filename) =
                fire_lint_case(slug, path, source, declared_lint_tools(case))?;
            return Ok((diag, source.to_owned(), filename));
        }
        if path == "book.sh"
            && let Ok(world) = fire_book_analysis(slug, path, source, oracles, consented, results)
        {
            return Ok(world);
        }
        if let Some(results) = results
            && let Ok(diag) = fire_records_admission(slug, source, results)
        {
            return Ok((diag, String::new(), String::new()));
        }
        let diag = canonical_payload(slug)
            .ok_or_else(|| format!("no canonical world for `{slug}` (world-as-payload)"))?;
        Ok((diag, String::new(), String::new()))
    }

    fn whylog_diagnostic(case: &Case) -> Result<Diag, String> {
        let command = case
            .replay()
            .blocks()
            .first()
            .map(errorloom::ReplayBlock::command)
            .ok_or_else(|| "case has no replay".to_owned())?;
        let words = exact_words(command).ok_or_else(|| "unsupported whylog replay".to_owned())?;
        let why = parse_direct_why(&words).ok_or_else(|| "unsupported whylog replay".to_owned())?;
        let raw = case
            .sections()
            .iter()
            .find(|section| section.name() == why.whylog)
            .map(errorloom::Section::content);
        let book = case
            .sections()
            .iter()
            .find(|section| section.name() == "book.sh")
            .map(errorloom::Section::content);
        dorc_plan::whylog::inspect(raw, why.whylog, book, |path| {
            case.sections()
                .iter()
                .find(|section| section.name() == path)
                .map(|section| section.content().to_owned())
        })
        .into_iter()
        .next()
        .ok_or_else(|| "whylog replay produced no diagnostic".to_owned())
    }
}

/// The WHOLE-PAGE arrangement route: an invocation whose entire output is one registry entry
/// (`288:rul-help-text-is-loomable`; the `289` §2o help pilot). Both the driver and the
/// re-render seat go through this, so the transcript a human edits and the bytes the fixpoint
/// re-derives are the same registry read.
///
/// The declared-arrangement check is this family's honest trigger
/// (`289:rul-worldless-route-honest-trigger`): a page case whose command renders some OTHER
/// page is refused rather than quietly transcribing a page it does not claim.
fn arrangement_page_slug(case: &Case, words: &[&str]) -> Option<&'static str> {
    let slug = match words {
        ["dorc", "--help" | "-h"] => dorc_cli::HELP_ARRANGEMENT,
        _ => return None,
    };
    match case.frontmatter().scalar("arrangement") {
        Some(declared) if declared != slug => None,
        _ => Some(slug),
    }
}

/// The words a compiled arrangement section would STORE, or `None` for a catalog register.
///
/// The ONE derivation, shared by the appliers and by the divergence pre-pass, so the pre-pass can
/// never disagree with the write it is guarding about what an edit lands as.
fn stored_words(field: &str, compiled: &crate::CompiledSection) -> Option<Vec<String>> {
    if field == crate::ARRANGEMENT_LINE_FIELD {
        return Some(line_words(compiled));
    }
    (field == crate::ARRANGEMENT_FIELD).then(|| page_words(compiled))
}

/// A PAGE's one word: its author's bytes, verbatim (`28H` ruling 7).
fn page_words(compiled: &crate::CompiledSection) -> Vec<String> {
    vec![
        compiled
            .fragments()
            .iter()
            .filter_map(|fragment| match fragment {
                crate::CompiledFragment::Text(text) => Some(text.as_str()),
                crate::CompiledFragment::Variable(_) => None,
            })
            .collect(),
    ]
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

/// Whitespace runs to one space. A laid-out line's inter-word whitespace is the RENDERER's, so
/// storing the wrap it chose at one width would freeze that width into the entry (`282` §3's
/// read-in normalization). Collapse, never TRIM: a word's own leading or trailing space is what
/// separates it from the value beside it.
pub(crate) fn collapse_runs(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    let mut spacing = false;
    for character in word.chars() {
        if character.is_whitespace() {
            if !spacing {
                out.push(' ');
            }
            spacing = true;
        } else {
            out.push(character);
            spacing = false;
        }
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

/// One `dorc why --last --whylog=<file>` replay: which durable, and which question of it.
struct DirectWhy<'a> {
    /// The `book.sh:N` positional, when the case asked about a line rather than the whole run.
    /// LEADING only — the parser also takes it after a flag (`289:rider-why-last-address-order`),
    /// but a case's replay line is written by hand and the canonical spelling is the leading one.
    address: Option<&'a str>,
    /// The case-relative durable to replay.
    whylog: &'a str,
}

fn parse_direct_why<'a>(words: &[&'a str]) -> Option<DirectWhy<'a>> {
    let (address, whylog) = match words {
        ["dorc", "why", "--last", whylog] => (None, whylog),
        ["dorc", "why", address, "--last", whylog] if !address.starts_with('-') => {
            (Some(*address), whylog)
        }
        _ => return None,
    };
    let path = whylog.strip_prefix("--whylog=")?;
    case_relative_path(path).then_some(DirectWhy {
        address,
        whylog: path,
    })
}

/// The DEGRADED `dorc why --last` receipt, rendered in-process over a committed durable
/// (`28F:rul-drift-replay-d1`; `28H:prop-drifted-why-is-the-thin-driver`).
///
/// The ONE seat both replay chains go through, which is what keeps them from disagreeing: the two
/// differ only in where a case's bytes come from, so that is the only thing `source` supplies.
///
/// `None` — falling through to the refusal-diagnostic route — for anything that is not a drifted
/// v2 durable: an unadmissible durable, a durable naming a book the case does not carry, or a book
/// that still digests to what the run recorded. Drift is the ONLY state this route answers, because
/// it is the only one whose answer is a report rather than a diagnostic.
fn drifted_why_parts(
    ctx: &RenderCtx<'_>,
    whylog: &str,
    address: Option<&str>,
    source: impl Fn(&str) -> Option<String>,
) -> Option<dorc_aid::tagged::RenderParts> {
    let dorc_plan::records::Admission::Admitted(envelope) =
        dorc_plan::whylog::admit_unscoped_whylog(
            whylog.as_bytes(),
            dorc_plan::whylog::WhylogLimits::spike_default(),
        )
    else {
        return None;
    };
    let book = source(envelope.recorded_book_path().as_str())?;
    if dorc_plan::invocation::book_digest(&book) == envelope.claims().book_digest() {
        return None;
    }
    Some(dorc_cli::drifted_why_parts(
        ctx,
        address,
        &dorc_cli::drifted_receipt(&envelope),
    ))
}

/// One LIVE `dorc why [<address>] --book=<book> [-o <oracle>]… [--all]` replay: the full report
/// over a world the case materializes, rather than the degraded receipt a drifted durable yields.
///
/// This is what gives the `why-*` chrome an editable home (`28L:rul-full-driver-this-arc`): the
/// report is driven through the same `dorc_cli::why` seat the binary prints, so the transcript a
/// human edits IS the render.
struct DirectWhyReport<'a> {
    /// The `book.sh:N` / content positional, when the case asked about one site.
    address: Option<&'a str>,
    /// The case-relative book.
    book: &'a str,
    /// The case-relative oracle sources, in argv order.
    oracles: Vec<&'a str>,
    /// `--all`: the deepest pull tier.
    deepest: bool,
    /// The `< <path>` probe-results redirect — a MEASURED world, admitted through the real fixture
    /// intake. Absent ⇒ every fact ⊤ ⇒ every site runs.
    input: Option<&'a str>,
    /// Did the invocation consent to the survival tier (`--risk-faultless-skips`)? Read off the
    /// command rather than the frontmatter, so the world the transcript shows is the world its own
    /// committed invocation asks for.
    consented: bool,
}

fn parse_direct_why_report<'a>(words: &[&'a str]) -> Option<DirectWhyReport<'a>> {
    let mut rest = words.strip_prefix(&["dorc", "why"])?.iter().peekable();
    let leads = rest.peek().is_some_and(|word| !word.starts_with('-'));
    let address = leads.then(|| rest.next().copied()).flatten();
    let mut book = None;
    let mut oracles = Vec::new();
    let mut deepest = false;
    let mut input = None;
    let mut consented = false;
    while let Some(word) = rest.next() {
        if let Some(path) = word.strip_prefix("--book=") {
            book = Some(path);
        } else if *word == "--pre-source" {
            oracles.push(*rest.next()?);
        } else if *word == "--all" {
            deepest = true;
        } else if *word == dorc_cli::CONSENT_FLAG {
            if consented {
                return None;
            }
            consented = true;
        } else if *word == "<" {
            if input.replace(*rest.next()?).is_some() {
                return None;
            }
        } else {
            return None;
        }
    }
    let book = book?;
    (case_relative_path(book)
        && oracles.iter().copied().all(case_relative_path)
        && input.is_none_or(case_relative_path))
    .then_some(DirectWhyReport {
        address,
        book,
        oracles,
        deepest,
        input,
        consented,
    })
}

/// The LIVE `dorc why` report, rendered in-process over a world the case materializes.
///
/// The ONE seat both replay chains go through — they differ only in where a case's bytes come
/// from, so that is the only thing `source` supplies.
/// The load snapshot a CASE is: a flat virtual directory whose section NAMES are its paths.
///
/// The modeled cwd is that directory's root, so `. ./x.oracle.sh` in a case's book names the
/// case's own section (`30I:rul-dot-resolves-as-sh`). The e2e runner materializes the same case
/// into a real directory and runs the real binary standing in it, so the in-process route and the
/// executed one resolve identically by construction rather than by agreement.
fn case_snapshot(
    book_path: &str,
    book_src: &str,
    paths: Vec<String>,
    srcs: Vec<String>,
) -> dorc_cli::snapshot::StaticLoadSnapshot {
    let cwd = dorc_core::loadpath::Cwd::default();
    let book_sourced = dorc_cli::snapshot::book_reached(&cwd, &paths, &srcs, book_src);
    dorc_cli::snapshot::StaticLoadSnapshot::over(
        cwd,
        paths,
        srcs,
        &book_sourced,
        book_path,
        book_src,
    )
}

fn live_why_parts(
    ctx: &RenderCtx<'_>,
    why: &DirectWhyReport<'_>,
    source: impl Fn(&str) -> Option<String>,
) -> Result<dorc_aid::tagged::RenderParts, String> {
    // A `Result`, not an `Option`, so an intake REFUSAL reaches the author verbatim: collapsing it
    // to "unsupported replay" would hide the one message that names the header they must write
    // (`28L:rul-refusals-name-the-next-command`).
    let missing = |path: &str| format!("the case carries no `{path}` section");
    let book = source(why.book).ok_or_else(|| missing(why.book))?;
    let oracle_paths: Vec<String> = why.oracles.iter().map(|p| (*p).to_owned()).collect();
    let oracle_srcs = oracle_paths
        .iter()
        .map(|path| source(path).ok_or_else(|| missing(path)))
        .collect::<Result<Vec<String>, String>>()?;
    let results = match why.input {
        Some(path) => admitted_site_results(
            why.book,
            &book,
            &oracle_paths,
            &oracle_srcs,
            &source(path).ok_or_else(|| missing(path))?,
        )?,
        None => dorc_cli::results::SiteResults::default(),
    };
    let world = dorc_cli::world::WhyWorld::analyze_measured(
        &case_snapshot(why.book, &book, oracle_paths.clone(), oracle_srcs.clone()),
        &results,
        why.consented,
    );
    // Every field is controller-minted, exactly as the binary mints them on a hostless run: the
    // fixture framing supplies the host, the book supplies its own digest, and there is no clock
    // (`28F:rul-probe-instants-host-says-no-times` — an undated receipt says so rather than
    // inventing a moment).
    let framing = dorc_plan::records::Framing::spike(dorc_plan::invocation::book_digest(&book));
    let receipt = dorc_cli::Receipt {
        at: None,
        replayed: false,
        host: framing.host().to_owned(),
        book: why.book.to_owned(),
        book_digest: framing.book_digest().to_owned(),
        at_head: None,
        oracles: oracle_paths,
        risk_profile: why.consented.then_some(dorc_cli::CONSENT_FLAG),
        tally: dorc_cli::PlanTally::Derived(world.disposition_counts()),
        deepest_tier: why.deepest,
        narratable: true,
    };
    Ok(dorc_cli::why::why_report_parts(
        ctx,
        &world.report(why.address, &receipt),
    ))
}

/// The HONEST-TRIGGER invocation route (`289:rul-worldless-route-honest-trigger`; `291` §5a W2).
///
/// Runs the REAL argument parser over the case's own replay argv and returns the diagnostic it
/// actually produced — but only when that diagnostic's slug equals the case's declared `code`. The
/// refusal on mismatch is the whole value (`291:rule-worldless-route-refuses-on-mismatch`): without
/// it, a case's command would be decorative and could drift from its code forever, on the surface
/// humans review errors through. `$ dorc strip` IS the world for this family — no fixture needed.
///
/// `None` for anything that parses successfully or whose slug disagrees, so the caller falls
/// through to the plan/payload routes exactly as before.
fn fire_invocation_error(case: &Case, tokens: &[&str]) -> Option<Diag> {
    let slug = case.frontmatter().scalar("code")?;
    let argv: Vec<String> = match tokens.split_first()? {
        (&"dorc", rest) => rest.iter().map(|word| (*word).to_owned()).collect(),
        (&"dorc-sh", rest) => return fire_dorc_sh_error(slug, rest),
        _ => return None,
    };
    let diag = dorc_cli::parse_args_from(argv).err()?;
    (diag.code.slug() == slug).then_some(diag)
}

/// `dorc-sh`'s three errors have no parser to run — the bin decides them inline from its argv and
/// the filesystem. Only the ARGV-decidable one is honest here; the two I/O failures would need a
/// real unreadable file and a real missing shell, so their cases stay world-as-payload.
fn fire_dorc_sh_error(slug: &str, rest: &[&str]) -> Option<Diag> {
    (slug == "dorc-sh-usage" && rest.is_empty())
        .then(|| Diag::new_spanless_site(DiagCode::DorcShUsage(dorc_aid::diag::DorcShUsage)))
}

/// The `dorc lint` route, and whether the run leaves external tools ENABLED. The bare form is the
/// default invocation, and the injected runner answers every tool absent — a real world reached
/// with no PATH probe and no process, which is what lets the tool-absence findings replay their own
/// production surface instead of a `dorc plan` that never fires them.
fn parse_direct_lint<'a>(words: &[&'a str]) -> Option<(&'a str, bool)> {
    let (path, tools_enabled) = match words {
        ["dorc", "lint", path, "--no-tools"] => (path, false),
        ["dorc", "lint", path] => (path, true),
        _ => return None,
    };
    case_relative_path(path).then_some((path, tools_enabled))
}

fn exact_words(command: &str) -> Option<Vec<&str>> {
    if command.is_empty()
        || command.contains([
            '\'', '"', '`', '$', '|', ';', '&', '>', '(', ')', '\\', '\n', '\r',
        ])
    {
        return None;
    }
    let words: Vec<_> = command.split_ascii_whitespace().collect();
    if words.is_empty() {
        return None;
    }
    Some(words)
}

struct DirectPlan<'a> {
    book: &'a str,
    /// Did the invocation consent to the survival tier (`--risk-faultless-skips`)?
    consented: bool,
    /// The `--pre-source <path>` set, in the order the invocation names it — the same order
    /// `law-lineno-identity` keys oracle file indices by, so a threaded span frames into the
    /// section the author expects.
    oracles: Vec<&'a str>,
    input: Option<&'a str>,
    machine: bool,
}

/// `dorc apply --host <dest> --plan <path>`: recognized ONLY to reach the world-as-payload floor.
///
/// A remote apply's outcome has no in-process world to drive — this driver opens no sockets — so
/// there is nothing to execute, only prose to pin against a canonical payload.
fn parse_direct_remote_apply(words: &[&str]) -> bool {
    matches!(words, ["dorc", "apply", "--host", _, "--plan", _])
}

fn parse_direct_plan<'a>(words: &[&'a str]) -> Option<DirectPlan<'a>> {
    if words.get(..2) != Some(["dorc", "plan"].as_slice()) {
        return None;
    }
    let mut book = None;
    let mut oracles = Vec::new();
    let mut consented = false;
    let mut input = None;
    let mut verbose = false;
    let mut machine = false;
    let mut index = 2;
    while let Some(word) = words.get(index) {
        if let Some(path) = word.strip_prefix("--book=") {
            if book.replace(path).is_some() || !case_relative_path(path) {
                return None;
            }
        } else if *word == "--pre-source" {
            index = index.saturating_add(1);
            let path = *words.get(index)?;
            if !case_relative_path(path) || oracles.contains(&path) {
                return None;
            }
            oracles.push(path);
        } else if *word == "--host" {
            index = index.saturating_add(1);
            words.get(index)?;
        } else if *word == dorc_cli::CONSENT_FLAG {
            if consented {
                return None;
            }
            consented = true;
        } else if *word == "--verbose" {
            if verbose {
                return None;
            }
            verbose = true;
        } else if *word == "--format=jsonl" {
            if machine {
                return None;
            }
            machine = true;
        } else if *word == "<" {
            let path = *words.get(index.saturating_add(1))?;
            if input.replace(path).is_some() || !case_relative_path(path) {
                return None;
            }
            index = index.saturating_add(1);
        } else {
            return None;
        }
        index = index.saturating_add(1);
    }
    (!verbose || !machine).then_some(DirectPlan {
        book: book?,
        consented,
        oracles,
        input,
        machine,
    })
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

fn materialized_file(case: &Case, context: &ReplayContext<'_>, path: &str) -> bool {
    case_relative_path(path)
        && case.sections().iter().any(|section| section.name() == path)
        && context.cwd().join(path).is_file()
}

fn materialized_source(case: &Case, context: &ReplayContext<'_>, path: &str) -> Option<String> {
    if !materialized_file(case, context, path) {
        return None;
    }
    fs::read_to_string(context.cwd().join(path)).ok()
}

/// Every `*.oracle.sh` section a case carries, in section order — the loaded set for a world
/// derived without a command to name one ([`DorcConsumer::world_of`]).
fn oracle_sections(case: &Case) -> Vec<(String, String)> {
    case.sections()
        .iter()
        .filter(|section| section.name().ends_with("oracle.sh"))
        .map(|section| (section.name().to_owned(), section.content().to_owned()))
        .collect()
}

/// [`materialized_source`]'s twin for the re-render chain, which has no materialized directory:
/// the case's own section bytes, under the same case-relative path rule.
fn section_source<'a>(case: &'a Case, path: &str) -> Option<&'a str> {
    case_relative_path(path)
        .then(|| {
            case.sections()
                .iter()
                .find(|section| section.name() == path)
                .map(errorloom::Section::content)
        })
        .flatten()
}

fn materialized_input(case: &Case, context: &ReplayContext<'_>, path: &str) -> Option<String> {
    if !materialized_file(case, context, path) {
        return None;
    }
    fs::read_to_string(context.cwd().join(path)).ok()
}

/// Consumer-neutral replay dispatch is implemented by this exact-shape Dorc adapter.
#[derive(Debug)]
pub struct DorcReplayDriver<'a> {
    consumer: &'a DorcConsumer,
    case: &'a Case,
    self_reference: SelfReference,
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
    /// Bind one case to its production-render consumer.
    #[must_use]
    pub fn new(consumer: &'a DorcConsumer, case: &'a Case) -> Self {
        Self {
            consumer,
            case,
            self_reference: SelfReference::Allowed,
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
        command: &str,
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        self.consumer
            .replay_within(self.case, command, context, self.self_reference)
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
            None => fallback(command, context),
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
            None => fallback(command, context),
        }
    })
}

impl CaseRenderer for DorcConsumer {
    type Error = String;

    fn render_case(&self, case: &Case) -> Result<String, String> {
        let outputs = case
            .replay()
            .blocks()
            .iter()
            .map(|block| self.render_direct_replay(case, block.command()))
            .collect::<Result<Vec<_>, _>>()?;
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

impl DorcConsumer {
    /// The committed transcript for one replay block.
    ///
    /// Every arm here answers with the SAME seat's bytes as [`Self::replay`]'s provenance answer —
    /// there is one render form, so nothing has to undo a second one before an edit can be
    /// attributed (`28L:rul-editability-is-stamped-never-re-derived`).
    fn render_direct_replay(&self, case: &Case, command: &str) -> Result<String, String> {
        let words =
            exact_words(command).ok_or_else(|| format!("unsupported replay {command:?}"))?;
        if let Some(slug) = arrangement_page_slug(case, &words) {
            return Ok(self.arrangement_page(slug)?.text());
        }
        if words.first() == Some(&LOOM_COMMAND) {
            return self
                .loom_replay(case, &words, SelfReference::Allowed, &|target| {
                    section_source(case, target).map(str::to_owned)
                })
                .ok_or_else(|| format!("unsupported replay {command:?}"));
        }
        if words.as_slice() == ["dorc", "lint", "--list-sources"] {
            return Ok(dorc_cli::lint_sources_parts(&self.render_ctx()).text());
        }
        if let Some(why) = parse_direct_why_report(&words) {
            return live_why_parts(&self.render_ctx(), &why, |path| {
                section_source(case, path).map(str::to_owned)
            })
            .map(|parts| parts.text());
        }
        if let Some(why) = parse_direct_why(&words) {
            return self.render_direct_why(case, &why, command);
        }
        if let Some((path, tools_enabled)) = parse_direct_lint(&words) {
            let source = case
                .sections()
                .iter()
                .find(|section| section.name() == path)
                .filter(|_| case_relative_path(path))
                .map(errorloom::Section::content)
                .ok_or_else(|| format!("unsupported replay {command:?}"))?;
            return Ok(dorc_lint::lint_materialized_source(
                path.to_owned(),
                source.to_owned(),
                dorc_lint::SourcePolicy { tools_enabled },
            )
            .human(&self.render_ctx())
            .text());
        }
        if let Some(diag) = fire_invocation_error(case, &words) {
            return Ok(self
                .invocation_parts(&diag, words.first().copied().unwrap_or_default())
                .text());
        }
        if parse_direct_remote_apply(&words) {
            let diag = Self::world_of(case)?.0;
            return Ok(self
                .declared_seat_parts(case, &diag)
                .unwrap_or_else(|| self.cli_parts(&diag, "", ""))
                .text());
        }
        let plan =
            parse_direct_plan(&words).ok_or_else(|| format!("unsupported replay {command:?}"))?;
        if let Some(parts) = self.plan_envelope(case, plan.book) {
            return Ok(parts.text());
        }
        let Some(source) = case
            .sections()
            .iter()
            .find(|section| section.name() == plan.book)
            .filter(|_| case_relative_path(plan.book))
            .map(errorloom::Section::content)
        else {
            // World-as-payload: no materialized book ⇒ the canonical spanless diagnostic.
            let diag = Self::world_of(case)?.0;
            if plan.machine {
                return Ok(render_diag_jsonl(&diag));
            }
            return Ok(self
                .declared_seat_parts(case, &diag)
                .unwrap_or_else(|| self.cli_parts(&diag, "", ""))
                .text());
        };
        let oracles: Vec<(String, String)> = plan
            .oracles
            .iter()
            .map(|path| Some(((*path).to_owned(), section_source(case, path)?.to_owned())))
            .collect::<Option<_>>()
            .ok_or_else(|| format!("unsupported replay {command:?}"))?;
        let results = match plan.input {
            Some(input) => Some(
                section_source(case, input)
                    .ok_or_else(|| format!("unsupported replay {command:?}"))?,
            ),
            None => None,
        };
        let (diag, framed, filename) =
            Self::world_of_source(case, plan.book, source, &oracles, plan.consented, results)?;
        if plan.machine {
            return Ok(render_diag_jsonl(&diag));
        }
        Ok(self.cli_parts(&diag, &framed, &filename).text())
    }

    /// The re-render half of the `dorc why --last` route: the degraded RECEIPT when the case's
    /// durable drifted from its book, else the refusal diagnostic. Split out of
    /// [`Self::render_direct_replay`] so the two answers stay one arm rather than two.
    fn render_direct_why(
        &self,
        case: &Case,
        why: &DirectWhy<'_>,
        command: &str,
    ) -> Result<String, String> {
        let raw = section_source(case, why.whylog);
        if let Some(parts) = raw.and_then(|whylog| {
            drifted_why_parts(&self.render_ctx(), whylog, why.address, |path| {
                section_source(case, path).map(str::to_owned)
            })
        }) {
            return Ok(parts.text());
        }
        if why.address.is_some() {
            return Err(format!("unsupported replay {command:?}"));
        }
        let book = section_source(case, "book.sh");
        let diag = dorc_plan::whylog::inspect(raw, why.whylog, book, |path| {
            section_source(case, path).map(str::to_owned)
        })
        .into_iter()
        .next()
        .ok_or_else(|| format!("unsupported replay {command:?}"))?;
        let parts = self.staged_cli_parts("whylog", &diag);
        Ok(parts.text())
    }
}

/// The compact machine view of a single diagnostic for a `--format=jsonl` replay block (`282` §2
/// machine-format replay · `282:rul-multi-replay-per-case`): one JSON object carrying the code slug
/// (the same-slug coherence gate every replay must pass) and its registry severity word. Both are
/// bare identifiers — no user text, so no escaping is possible — and it is a tool-corpus surface, not
/// a product API (`27V:rul-output-form-unwelded`; the machine format is free to churn). Trailing LF so
/// the block round-trips through the container's `set_replay_outputs`/`to_text` unchanged.
fn render_diag_jsonl(diag: &Diag) -> String {
    let severity = match diag.severity() {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    };
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{severity}\"}}\n",
        diag.code.slug(),
    )
}

/// The world-as-payload floor (`283:dec-world-two-forms`), for a slug with no honest trigger. The
/// stand-in payloads themselves live beside the payload TYPES, in [`dorc_aid::fixture`] — a Rust
/// surface — so adding a payload field never compile-errors inside this crate
/// (`28L:rul-rust-and-loom-are-the-only-edit-surfaces`).
///
/// Every stand-in renders SPANLESS: a code may carry a span in production, but its defining case
/// pins the frame-less title+body prose registers (the authoring surface), not the caret frame —
/// that is the world-as-pipeline routes' job.
fn canonical_payload(slug: &str) -> Option<Diag> {
    dorc_aid::fixture::canonical_payload(slug).map(Diag::new_spanless_site)
}

/// World-as-pipeline for the marker pilot (`28A` §2n): fire the REAL in-process marker gate over the
/// materialized oracle `source`, returning its (spanned) diagnostic + the source the caret frame
/// resolves against. Refuses if the gate fired nothing or a different code than the case declares
/// (the honest-trigger coherence the world-as-pipeline form buys).
fn fire_lint_case(
    slug: &str,
    filename: &str,
    source: &str,
    tools_enabled: bool,
) -> Result<(Diag, String, String), String> {
    let result = dorc_lint::lint_materialized_source(
        filename.to_owned(),
        source.to_owned(),
        dorc_lint::SourcePolicy { tools_enabled },
    );
    let finding = result
        .report()
        .findings
        .iter()
        .find(|finding| finding.code == slug)
        .ok_or_else(|| format!("world-as-pipeline `{slug}` fired no diagnostic"))?;
    let provenance = finding
        .provenance
        .as_ref()
        .ok_or_else(|| format!("world-as-pipeline `{slug}` lost typed provenance"))?;
    Ok((
        provenance.diag.clone(),
        provenance.source.clone(),
        filename.to_owned(),
    ))
}

/// World-as-pipeline for the cmdsub flagship (`28A` §2n, extended to the analysis kernel): fire the
/// REAL pipeline over the materialized `book.sh` against the case's own `-o` oracle set, returning
/// the (spanned) diagnostic whose slug matches the case's `code` + the source its caret frame
/// resolves against. Refuses if the pipeline fired nothing matching the declared slug
/// (honest-trigger coherence). The whole path is kernel-pure (`inv-determinism`).
///
/// The stage sequence is the binary's own, in the binary's own order (`cli/src/main.rs`'s `run`):
/// lift the oracles into the effect map + the check sets + the verdict index, then
/// parse → marker → reserved → CFG → value-flow → classify. Every stage's diagnostics are
/// searched, because a run reports every stage: searching only the last left every parse/CFG code
/// unreachable, and loading no oracles left every oracle-dependent code unreachable, so both had to
/// settle for hand-built stand-ins (`289:rul-worldless-route-honest-trigger`).
fn fire_book_analysis(
    slug: &str,
    filename: &str,
    source: &str,
    oracles: &[(String, String)],
    consented: bool,
    results: Option<&str>,
) -> Result<(Diag, String, String), String> {
    let mut interner = Interner::default();
    let oracle_refs: Vec<&str> = oracles.iter().map(|(_, src)| src.as_str()).collect();
    let idx = dorc_oracle::lift(&mut interner, &oracle_refs).value;
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
        .collect();
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
        .collect();
    let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);

    let parsed = dorc_syntax::parse(source);
    let marker = dorc_oracle::marker::check_dialect_marker(&mut interner, source);
    let reserved = dorc_oracle::reserved::lint_book_reserved_names(&parsed.value);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
    let mut arena = ProvArena::new();
    let effect = dorc_analysis::effect::classify(
        &cfg.value,
        &value,
        &parsed.value,
        &idx,
        &checks,
        &verdicts,
        &mut interner,
        &mut arena,
    );
    // The oracle-side confusability lints are a run's own act over the SAME lifted sets
    // (`cli::kinds`), not a second implementation: a defining case for one of them therefore pins
    // what an author's `dorc plan` really prints. Their carets point into an ORACLE, so they carry
    // their own frame — resolving an oracle's span against the book's bytes drew a caret under
    // whatever line happened to share the offset.
    let framed = dorc_cli::kinds::confusability_diagnostics(&checks, &oracle_refs, &mut interner)
        .into_iter()
        .map(|(file, diag)| match file.and_then(|i| oracles.get(i)) {
            Some((name, src)) => (diag, src.clone(), name.clone()),
            None => (diag, String::new(), String::new()),
        });
    // The wrapped-site + survival lanes, over the SAME sources (`cli::survival`). Flag-gated
    // exactly as a run is: with no `--risk-faultless-skips` in the replay command the survival half
    // is absent, not quiet.
    let survival = dorc_cli::survival::survival_diagnostics(
        source,
        &oracles
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>(),
        &oracles
            .iter()
            .map(|(_, src)| src.clone())
            .collect::<Vec<_>>(),
        consented,
        dorc_core::EscalationDial::VouchedOnly,
        dorc_core::Capability::Root,
        &match results {
            Some(stream) => admitted_site_results(
                filename,
                source,
                &oracles
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect::<Vec<_>>(),
                &oracles
                    .iter()
                    .map(|(_, src)| src.clone())
                    .collect::<Vec<_>>(),
                stream,
            )?,
            None => dorc_cli::results::SiteResults::default(),
        },
    );
    parsed
        .diags
        .into_iter()
        .chain(marker)
        .chain(reserved)
        .chain(cfg.diags)
        .chain(effect.diags)
        .chain(survival)
        .map(|diag| (diag, source.to_owned(), filename.to_owned()))
        .chain(framed)
        .find(|(diag, _, _)| diag.code.slug() == slug)
        .ok_or_else(|| format!("world-as-pipeline `{slug}` fired no `{slug}` diagnostic"))
}

/// The case's declared `< results` bytes, through the REAL fixture intake
/// (`28L:rul-records-seam-approved`). All three admission outcomes are honoured, and a REFUSED
/// stream refuses the whole world rather than degrading to an empty one: a case whose measured
/// facts rest on a broken channel would render a world nothing measured
/// (`rul-admission-is-a-closed-outcome`).
fn admitted_site_results(
    filename: &str,
    source: &str,
    oracle_paths: &[String],
    oracle_srcs: &[String],
    stream: &str,
) -> Result<dorc_cli::results::SiteResults, String> {
    use dorc_plan::records::Admission;
    let sources = dorc_cli::results::RunSources {
        book_name: filename,
        book: source,
        oracle_paths,
        oracle_sources: oracle_srcs,
    };
    // No clock: a committed transcript must be a fixpoint, and a fixture stream carries no
    // instants of its own (`inv-determinism`; `seam-tolerated-nondeterminism-stops-at-the-run-log`
    // leaves a rendered surface no normalizer to hide behind).
    let mut clock = dorc_cli::results::RunClock::Absent;
    let mut interner = Interner::default();
    match dorc_cli::results::admit_fixture_records(
        &sources,
        stream.as_bytes(),
        &mut clock,
        &mut interner,
    ) {
        Admission::Admitted(admitted) => Ok(admitted.scoped.results().clone()),
        Admission::NoObservation => Ok(dorc_cli::results::SiteResults::default()),
        // The refusal NAMES the header the stream must carry
        // (`28L:rul-refusals-name-the-next-command`). A framing mismatch is nearly always a book
        // edit moving the digest, and "refused" alone leaves the author to recompute a hash by
        // hand; `sites=` stays theirs to count, so the text says what it counts — a census, never
        // an id — because the two coincide at one record and diverge at two.
        Admission::Refused(reason) => Err(format!(
            "the declared results stream was refused ({}) -- a measured world cannot rest on a \
             broken channel. Its first line must be:\n  {} sites=<N> {}\nwhere <N> counts the \
             `site` records that follow, NOT the largest site id among them",
            reason.spanless_diagnostic().code.slug(),
            dorc_plan::records::expected_header_prefix(&dorc_plan::records::Framing::spike(
                dorc_plan::invocation::book_digest(source)
            )),
            dorc_plan::records::TERMINAL_TOKEN,
        )),
    }
}

/// The `(book, results)` section bytes a case's own first replay declares through
/// `dorc plan --book=B < R`. The re-render chain has no materialized directory, so it reads the
/// case's sections; the driven chain reads the materialized files. Both must answer the same world.
fn declared_plan_inputs(case: &Case) -> Option<(&str, &str)> {
    let block = case.replay().blocks().first()?;
    let tokens = exact_words(block.command())?;
    let plan = parse_direct_plan(&tokens)?;
    Some((
        section_source(case, plan.book)?,
        section_source(case, plan.input?)?,
    ))
}

/// World-as-pipeline for the intake edge: run the REAL bounded host-evidence admission over the
/// case's declared `< results` bytes, framed exactly as a hostless `dorc plan` over `book` frames
/// them. The refusal's own spanless diagnostic IS the world, which is what makes the `< file` in a
/// replay command load-bearing rather than decorative.
fn fire_records_admission(slug: &str, book: &str, results: &str) -> Result<Diag, String> {
    use dorc_plan::records::{
        Admission, Framing, HostEvidenceLimits, admit_unscoped_host_records, read_host_evidence,
    };
    let framing = Framing::spike(dorc_plan::invocation::book_digest(book));
    let limits = HostEvidenceLimits::spike_default();
    let admitted = match read_host_evidence(results.as_bytes(), limits) {
        Admission::Admitted(bytes) => admit_unscoped_host_records(&bytes, &framing, limits),
        Admission::NoObservation => Admission::NoObservation,
        Admission::Refused(reason) => Admission::Refused(reason),
    };
    let Admission::Refused(reason) = admitted else {
        return Err(format!(
            "world-as-pipeline `{slug}`: the results stream was admitted, so intake fired nothing"
        ));
    };
    let diag = reason.spanless_diagnostic();
    if diag.code.slug() == slug {
        Ok(diag)
    } else {
        Err(format!(
            "world-as-pipeline `{slug}` fired `{}` instead",
            diag.code.slug()
        ))
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

/// The `(consented, results)` a case's own first replay declares — the survival flag and the
/// `< file` section bytes. Read off the invocation rather than the frontmatter so the world a
/// worldless derivation answers is the world the committed command really asks for.
/// Whether the case's own first replay leaves external tools enabled — the worldless lint route's
/// half of `declared_plan_shape`, so `world_of` answers the world the driven render does.
fn declared_lint_tools(case: &Case) -> bool {
    case.replay()
        .blocks()
        .first()
        .and_then(|block| exact_words(block.command()))
        .and_then(|tokens| parse_direct_lint(&tokens).map(|(_, tools)| tools))
        .unwrap_or(false)
}

fn declared_plan_shape(case: &Case) -> (bool, Option<&str>) {
    let Some(block) = case.replay().blocks().first() else {
        return (false, None);
    };
    let Some(tokens) = exact_words(block.command()) else {
        return (false, None);
    };
    let Some(plan) = parse_direct_plan(&tokens) else {
        return (false, None);
    };
    (
        plan.consented,
        plan.input.and_then(|path| section_source(case, path)),
    )
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

    /// Read-in normalization (`282` §3, `28L`): a single embedded newline inside a catalog
    /// register is a soft wrap, not a paragraph break, so it collapses to a space and the
    /// compiled template stores no literal `\n` — the relaxed half of the tension pinned by
    /// `added_help_line_refuses_and_names_the_command` (`editable_surface.rs`), which keeps a
    /// genuine two-newline paragraph break refusing.
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

    /// A whole-page arrangement section can be a sixty-line help page, and the hunk must be the
    /// LINE that moved rather than the page around it: identical leading and trailing lines are
    /// trimmed off both sides before anything is printed.
    #[test]
    fn a_multi_line_section_prints_only_the_line_that_moved() {
        let page = SectionKey {
            field: crate::ARRANGEMENT_FIELD,
            ..key(0)
        };
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                page,
                vec![EditableFragment::Text(String::from("first\nsecond\nthird"))],
            ),
        )]);
        let preview = compile_preview(&baseline, "first\nsecond changed\nthird")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(
            render_publish_diff(&preview),
            "section: code.arrangement#0:0\n  - second\n  + second changed"
        );
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
