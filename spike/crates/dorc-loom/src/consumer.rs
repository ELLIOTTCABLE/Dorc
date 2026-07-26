//! The Dorc case renderer and compiled-edit applier (`282` §5 · §13), implemented against a mutable
//! owned-catalog mirror ([`dorc_aid::catalog::OwnedEntry`]).
//!
//! World-form dispatch (`283:dec-world-two-forms`): a `-- world --`-only case is WORLD-AS-PAYLOAD (a
//! canonical constructor keyed by slug — the phase-4 floor for the artificial/expensive-world codes);
//! a case carrying a materialized oracle/book section is WORLD-AS-PIPELINE (the real in-process kernel
//! fires the diagnostic — the marker pilot). Phase 4 lands the payload path; the pipeline arm is the
//! marker-version-unrecognized pilot.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;

use dorc_aid::Severity;
use dorc_aid::arrangement::{OwnedArrangement, OwnedWords, arrangement_parts, owned_arrangements};
use dorc_aid::catalog::{OwnedEntry, is_foreign_param, owned_catalog, parse_template};
use dorc_aid::diag::{
    AidUnloadedSiblingOracle, CarriedAcrossSubstrateAxis, CliFileNotFound, CliFilePermissionDenied,
    CliFileUnreadable, CliShimDirUnwritable, CmdsubOperandTop, CommandName, DanglingReference,
    Diag, DiagCode, DorcShExecFailed, DorcShScriptUnreadable, EscalationPolicy,
    HostEvidenceAdmissionRefused, HostEvidenceRefusalKind, LintFileCountDrift, LintNoLintableFiles,
    LintRequiredToolsMissing, LintToolAbsent, LintToolFailedWithoutFindings,
    LintToolOutputUnparsable, OperandPosition, RecordsFactTruncated, RenderHeredocRefused, SiteId,
    SiteUnresolvable, SyntaxUnsupported, WhylogUnwritten, WrapperPeelIncoherent, render_cli_parts,
    render_cli_with, render_staged_cli_parts,
};
use dorc_core::{Interner, LeafId, ProvArena, TopCause};
use errorloom::{
    Case, CaseRenderer, EditableFragment, EditableRender, RenderComponent, ReplayContext,
    ReplayDriver, ReplayInput, ReplayResult, RunEnv, RunError, drive_case, drive_case_with_inputs,
};

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
    /// An arrangement edit carried a `{{name}}` variable; registry entries store WORDS, never
    /// templates (`289` §2o: the registry never grows grammar machinery).
    ArrangementTakesNoVariables(String),
    /// The edited entry holds a WORD SEQUENCE, and nothing re-splits an edited string back into
    /// its words. Storage stays sequence-shaped for a future chain narration
    /// (`289:rider-arrangement-home-anticipates-chains`); the edit path refuses rather than
    /// guessing, because nothing chain-shaped is built yet.
    ArrangementIsSequenceStructured(String),
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
        }
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
    pub fn set_message(&mut self, slug: &str, message: Option<String>) {
        if let Some(e) = self.mirror.iter_mut().find(|e| e.slug == slug) {
            e.message = message;
        }
    }

    /// Overwrite an arrangement entry's words in the mirror (the [`Self::set_message`] twin: it
    /// models a raw registry hand-edit, and stages the word-sequence state nothing authors yet).
    pub fn set_arrangement_words(&mut self, slug: &str, words: OwnedWords) {
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

    /// Apply every compiled section of a preview to the mirror (the promote-publish edited mirror,
    /// `28A` §4): the edited templates the regenerated lock and affected cases are computed from.
    ///
    /// # Errors
    /// Returns [`DorcApplyRefusal`] for an absent code or non-prose field.
    pub fn apply_preview(
        &mut self,
        preview: &crate::CompilePreview,
    ) -> Result<(), DorcApplyRefusal> {
        for section in preview.sections() {
            self.apply_compiled_section(&section.section, &section.compiled)?;
        }
        Ok(())
    }

    fn apply_compiled_section(
        &mut self,
        key: &SectionKey,
        compiled: &crate::CompiledSection,
    ) -> Result<(), DorcApplyRefusal> {
        if key.field == crate::ARRANGEMENT_FIELD {
            return self.apply_arrangement_edit(key, compiled);
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
        if key.field == "message" {
            entry.message = Some(template);
        } else {
            entry.help = Some(template);
        }
        entry.params = entry
            .message
            .iter()
            .chain(entry.help.iter())
            .flat_map(|template| parse_template(template).unwrap_or_default())
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
        Ok(())
    }

    /// Apply one compiled arrangement section to the registry mirror — the chrome twin of the
    /// catalog path above, and the whole of "an arrangement-word edit in a transcript flows to
    /// its registry entry exactly as catalog prose does" (`289` §2o).
    ///
    /// The edit lands on the entry the RENDER read: the occurrence's own entry when it has one,
    /// else the whole-slug entry — so an edit to any one occurrence of shared chrome updates the
    /// shared words, which is the truth of a shared entry.
    fn apply_arrangement_edit(
        &mut self,
        key: &SectionKey,
        compiled: &crate::CompiledSection,
    ) -> Result<(), DorcApplyRefusal> {
        let mut words = Vec::new();
        for fragment in compiled.fragments() {
            match fragment {
                crate::CompiledFragment::Text(text) => words.push(text.clone()),
                crate::CompiledFragment::Variable(_) => {
                    return Err(DorcApplyRefusal::ArrangementTakesNoVariables(
                        key.owner.clone(),
                    ));
                }
            }
        }
        let index = arrangement_index(&self.arrangements, &key.owner, key.instance)
            .ok_or_else(|| DorcApplyRefusal::MissingArrangement(key.owner.clone()))?;
        let entry = self
            .arrangements
            .get_mut(index)
            .ok_or_else(|| DorcApplyRefusal::MissingArrangement(key.owner.clone()))?;
        if entry.words.words().is_some_and(|current| current.len() > 1) {
            return Err(DorcApplyRefusal::ArrangementIsSequenceStructured(
                key.owner.clone(),
            ));
        }
        entry.words = OwnedWords::Authored(vec![words.concat()]);
        Ok(())
    }

    /// Re-render a case corpus from the current in-memory mirror.
    ///
    /// # Errors
    /// Returns a case-world materialization refusal.
    pub fn render_cases(&self, cases: &[Case]) -> Result<Vec<String>, String> {
        cases.iter().map(|case| self.render_case(case)).collect()
    }

    /// Render one case through the core part stream and map it to editable sections.
    ///
    /// # Errors
    /// Returns the case-world materialization refusal.
    pub fn editable_baseline(&self, case: &Case) -> Result<DorcEditableBaseline, String> {
        if let Some(slug) = case.frontmatter().scalar("arrangement") {
            return self.arrangement_baseline(slug);
        }
        let (diag, src, filename) = Self::world_of(case)?;
        let interner = Interner::default();
        let parts = render_cli_parts(&self.mirror, &diag, &src, &filename, &interner);
        let render = to_editable_render(&parts);
        let variables = editable_variables(&render)?;
        let all_variables = dorc_aid::diag::params_of(&diag.code, &interner)
            .into_iter()
            .filter(|(name, _)| !is_foreign_param(name))
            .map(|(name, value)| (TemplateVariableName(String::from(name)), value))
            .collect();
        Ok(DorcEditableBaseline {
            render,
            variables,
            all_variables,
        })
    }

    /// Drive only direct invocations whose replay inputs and rendering are exact.
    #[must_use]
    pub fn replay(
        &self,
        case: &Case,
        command: &str,
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        let tokens = exact_words(command)?;
        if let Some(slug) = arrangement_page_slug(case, &tokens) {
            let parts = self.arrangement_page(slug).ok()?;
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if let ["dorc-loom", "vars", mode, path] = tokens.as_slice()
            && matches!(*mode, "--used" | "--all")
            && case_relative_path(path)
        {
            let target = Case::parse(context.materialized_input(path)?).ok()?;
            let baseline = self.editable_baseline(&target).ok()?;
            let mut output = String::new();
            output.push_str("case: ");
            output.push_str(path);
            output.push('\n');
            let values = if *mode == "--used" {
                baseline.used_variables()
            } else {
                baseline
                    .all_variables()
                    .iter()
                    .map(|(name, value)| (name.clone(), value.clone()))
                    .collect()
            };
            for (name, value) in values {
                let _ = writeln!(output, "{{{{{}}}}} = {value:?}", name.0);
            }
            return Some(ReplayResult::bytes(output));
        }
        if let Some(path) = parse_direct_why(&tokens) {
            let raw = context.materialized_input(path);
            let book = materialized_source(case, context, "book.sh");
            let inspected = dorc_plan::whylog::inspect(
                raw,
                path,
                book.as_deref()
                    .map(|book| dorc_plan::whylog::WhylogCurrent {
                        book: Some(book),
                        oracles: &[],
                    }),
            );
            let diag = inspected.diagnostics.into_iter().next()?;
            let interner = Interner::default();
            let parts = render_staged_cli_parts("whylog", &self.mirror, &diag, "", "", &interner);
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        if let Some(path) = parse_direct_lint(&tokens) {
            let source = materialized_source(case, context, path)?;
            let result = dorc_lint::lint_materialized_source(
                path.to_owned(),
                source,
                dorc_lint::SourcePolicy {
                    tools_enabled: false,
                },
            );
            return Some(ReplayResult::editable(to_editable_render(result.human())));
        }
        if let Some(diag) = fire_invocation_error(case, &tokens) {
            let interner = Interner::default();
            let parts = render_cli_parts(&self.mirror, &diag, "", "", &interner);
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        }
        let plan = parse_direct_plan(&tokens)?;
        let interner = Interner::default();
        // World-as-payload, the branch `render_direct_replay` has always had. Without it the
        // driver declined, so `compile`/`promote` never saw provenance for these cases.
        let Some(source) = materialized_source(case, context, plan.book) else {
            let diag = Self::world_of(case).ok()?.0;
            if plan.machine {
                return Some(ReplayResult::bytes(render_diag_jsonl(&diag)));
            }
            let parts = render_cli_parts(&self.mirror, &diag, "", "", &interner);
            return Some(ReplayResult::editable(to_editable_render(&parts)));
        };
        if let Some(input) = plan.input {
            let _ = materialized_input(case, context, input)?;
        }
        let (diag, _, filename) = Self::world_of_source(case, plan.book, &source).ok()?;
        if plan.machine {
            return Some(ReplayResult::bytes(render_diag_jsonl(&diag)));
        }
        let parts = render_cli_parts(&self.mirror, &diag, &source, &filename, &interner);
        let render = to_editable_render(&parts);
        Some(ReplayResult::editable(render))
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
        // An arrangement case has no diagnostic and no payload: registry entries store WORDS, so
        // the variable inventory is empty by construction rather than absent by failure.
        if case.frontmatter().scalar("arrangement").is_some() {
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
        let all_variables = dorc_aid::diag::params_of(&diag.code, &interner)
            .into_iter()
            .filter(|(name, _)| !is_foreign_param(name))
            .map(|(name, value)| (TemplateVariableName(String::from(name)), value))
            .collect();
        Ok(DorcEditableBaseline {
            render,
            variables,
            all_variables,
        })
    }

    /// One whole-page arrangement's part stream, resolved against the COMMITTED registry so the
    /// span carries a stable slug. A case naming a slug with no row yet renders nothing: its row
    /// arrives by promotion and the build sees it after a rebuild — the same generation lag the
    /// catalog has, and the same assertion.
    fn arrangement_page(&self, slug: &str) -> Result<dorc_aid::tagged::RenderParts, String> {
        let stable = dorc_aid::arrangement::ARRANGEMENTS
            .iter()
            .find(|entry| entry.slug == slug)
            .map(|entry| entry.slug)
            .ok_or_else(|| {
                format!(
                    "arrangement `{slug}` has no registry row yet -- promote the case, then rebuild"
                )
            })?;
        Ok(arrangement_parts(&self.arrangements, stable, None))
    }

    /// The editable baseline of a whole-page arrangement case — the registry's own render, with
    /// an empty payload inventory.
    fn arrangement_baseline(&self, slug: &str) -> Result<DorcEditableBaseline, String> {
        let render = to_editable_render(&self.arrangement_page(slug)?);
        let variables = editable_variables(&render)?;
        Ok(DorcEditableBaseline {
            render,
            variables,
            all_variables: BTreeMap::new(),
        })
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
        if let Some(section) = case
            .sections()
            .iter()
            .find(|s| s.name().ends_with("oracle.sh"))
        {
            return fire_lint_case(slug, section.name(), section.content());
        }
        if let Some(section) = case.sections().iter().find(|s| s.name() == "book.sh")
            && let Ok(world) = fire_book_analysis(slug, section.name(), section.content())
        {
            return Ok(world);
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
        let diag = canonical_payload(slug)
            .ok_or_else(|| format!("no canonical world for `{slug}` (world-as-payload)"))?;
        Ok((diag, String::new(), String::new()))
    }

    fn world_of_source(
        case: &Case,
        path: &str,
        source: &str,
    ) -> Result<(Diag, String, String), String> {
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        if path.ends_with("oracle.sh") {
            let (diag, _, filename) = fire_lint_case(slug, path, source)?;
            return Ok((diag, source.to_owned(), filename));
        }
        if path == "book.sh"
            && let Ok((diag, _, filename)) = fire_book_analysis(slug, path, source)
        {
            return Ok((diag, source.to_owned(), filename));
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
        let path =
            parse_direct_why(&words).ok_or_else(|| "unsupported whylog replay".to_owned())?;
        let raw = case
            .sections()
            .iter()
            .find(|section| section.name() == path)
            .map(errorloom::Section::content);
        let book = case
            .sections()
            .iter()
            .find(|section| section.name() == "book.sh")
            .map(errorloom::Section::content);
        dorc_plan::whylog::inspect(
            raw,
            path,
            book.map(|book| dorc_plan::whylog::WhylogCurrent {
                book: Some(book),
                oracles: &[],
            }),
        )
        .diagnostics
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

fn parse_direct_why<'a>(words: &[&'a str]) -> Option<&'a str> {
    let ["dorc", "why", "--last", whylog] = words else {
        return None;
    };
    let path = whylog.strip_prefix("--whylog=")?;
    case_relative_path(path).then_some(path)
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

fn parse_direct_lint<'a>(words: &[&'a str]) -> Option<&'a str> {
    let ["dorc", "lint", path, "--no-tools"] = words else {
        return None;
    };
    case_relative_path(path).then_some(path)
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
    input: Option<&'a str>,
    machine: bool,
}

fn parse_direct_plan<'a>(words: &[&'a str]) -> Option<DirectPlan<'a>> {
    if words.get(..2) != Some(["dorc", "plan"].as_slice()) {
        return None;
    }
    let mut book = None;
    let mut input = None;
    let mut verbose = false;
    let mut machine = false;
    let mut index = 2;
    while let Some(word) = words.get(index) {
        if let Some(path) = word.strip_prefix("--book=") {
            if book.replace(path).is_some() || !case_relative_path(path) {
                return None;
            }
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
        input,
        machine,
    })
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
}

impl<'a> DorcReplayDriver<'a> {
    /// Bind one case to its production-render consumer.
    #[must_use]
    pub fn new(consumer: &'a DorcConsumer, case: &'a Case) -> Self {
        Self { consumer, case }
    }
}

impl ReplayDriver<SectionKey, SectionVariableId> for DorcReplayDriver<'_> {
    fn drive(
        &self,
        command: &str,
        context: &ReplayContext<'_>,
    ) -> Option<ReplayResult<SectionKey, SectionVariableId>> {
        self.consumer.replay(self.case, command, context)
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
        Ok(regenerated.to_text())
    }
}

impl DorcConsumer {
    fn render_direct_replay(&self, case: &Case, command: &str) -> Result<String, String> {
        let words =
            exact_words(command).ok_or_else(|| format!("unsupported replay {command:?}"))?;
        if let Some(slug) = arrangement_page_slug(case, &words) {
            return Ok(self.arrangement_page(slug)?.text());
        }
        if let Some(path) = parse_direct_why(&words) {
            let raw = case
                .sections()
                .iter()
                .find(|section| section.name() == path)
                .map(errorloom::Section::content);
            let book = case
                .sections()
                .iter()
                .find(|section| section.name() == "book.sh")
                .map(errorloom::Section::content);
            let inspected = dorc_plan::whylog::inspect(
                raw,
                path,
                book.map(|book| dorc_plan::whylog::WhylogCurrent {
                    book: Some(book),
                    oracles: &[],
                }),
            );
            let diag = inspected
                .diagnostics
                .into_iter()
                .next()
                .ok_or_else(|| format!("unsupported replay {command:?}"))?;
            let interner = Interner::default();
            let parts = render_staged_cli_parts("whylog", &self.mirror, &diag, "", "", &interner);
            return Ok(format!("{}\n", reflow_to_canonical(&parts.text())));
        }
        if let Some(path) = parse_direct_lint(&words) {
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
                dorc_lint::SourcePolicy {
                    tools_enabled: false,
                },
            )
            .human()
            .text());
        }
        if let Some(diag) = fire_invocation_error(case, &words) {
            let interner = Interner::default();
            let parts = render_cli_parts(&self.mirror, &diag, "", "", &interner);
            return Ok(format!("{}\n", reflow_to_canonical(&parts.text())));
        }
        let plan =
            parse_direct_plan(&words).ok_or_else(|| format!("unsupported replay {command:?}"))?;
        let interner = Interner::default();
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
            return Ok(reflow_to_canonical(&render_cli_with(
                &self.mirror,
                &diag,
                "",
                "",
                &interner,
            )));
        };
        if let Some(input) = plan.input {
            let has_input = case_relative_path(input)
                && case
                    .sections()
                    .iter()
                    .any(|section| section.name() == input);
            if !has_input {
                return Err(format!("unsupported replay {command:?}"));
            }
        }
        let (diag, _, filename) = Self::world_of_source(case, plan.book, source)?;
        if plan.machine {
            return Ok(render_diag_jsonl(&diag));
        }
        Ok(reflow_to_canonical(&render_cli_with(
            &self.mirror,
            &diag,
            source,
            &filename,
            &interner,
        )))
    }
}

/// The corpus's pinned canonical render width (`282` §3): committed transcripts word-wrap HERE, not
/// at a terminal — a live surface may wrap adaptively, the corpus does not. The committed file's own
/// hard-wrapping is normalized away on read-in (the whitespace-collapsing prose tokenizer) and
/// regenerated at this width, so the on-disk layout is render-owned, never author-owned.
const CANONICAL_WIDTH: usize = 80;

/// Reflow a flat CLI diagnostic render to [`CANONICAL_WIDTH`] (`282` §3, item-6 layout): the title
/// line wraps under a 3-space hanging indent (the message body under the title) and each
/// `= help:` / `= note:` block wraps under a 6-space hanging indent, its `=` marker re-aligned to the
/// frame's gutter column; the caret-frame lines pass through verbatim. This corpus surface owns the
/// wrap so a committed file's editing-time layout never reaches the fixpoint assertion.
fn reflow_to_canonical(render: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for (i, line) in render.lines().enumerate() {
        if i == 0
            && let Some(cut) = line.find("]: ")
        {
            let (prefix, text) = line.split_at(cut.saturating_add(3));
            out.push(wrap_words(prefix, "   ", text));
            continue;
        }
        if let Some(rest) = line.trim_start().strip_prefix("= ")
            && let Some(cut) = rest.find(": ")
        {
            let marker = rest.get(..cut.saturating_add(2)).unwrap_or(rest);
            let text = rest.get(cut.saturating_add(2)..).unwrap_or("");
            out.push(wrap_words(&format!("   = {marker}"), "      ", text));
            continue;
        }
        out.push(line.to_owned());
    }
    out.join("\n")
}

/// Greedy word-wrap of `text` beneath `prefix` (the un-wrapped first-line lead-in) at
/// [`CANONICAL_WIDTH`], every continuation line carrying `cont_indent`. Column counting is by
/// `char`, so the one-column `—`/`…` glyphs the prose uses count as one. Pure; total.
fn wrap_words(prefix: &str, cont_indent: &str, text: &str) -> String {
    let mut out = String::from(prefix);
    let mut col = prefix.chars().count();
    let mut started = false;
    for word in text.split_whitespace() {
        let wlen = word.chars().count();
        if !started {
            out.push_str(word);
            col = col.saturating_add(wlen);
            started = true;
        } else if col.saturating_add(1).saturating_add(wlen) <= CANONICAL_WIDTH {
            out.push(' ');
            out.push_str(word);
            col = col.saturating_add(1).saturating_add(wlen);
        } else {
            out.push('\n');
            out.push_str(cont_indent);
            out.push_str(word);
            col = cont_indent.chars().count().saturating_add(wlen);
        }
    }
    out
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

/// The world-as-payload canonical constructors, keyed by slug (`283:dec-world-two-forms`). The
/// phase-5 backport (`283` §5.9) renders every non-pipeline covered code SPANLESS: a code may carry a
/// span in production, but its defining case pins the frame-less title+body prose registers (the
/// authoring surface), not the caret frame — that is the marker pilot's world-as-pipeline job.
#[expect(
    clippy::too_many_lines,
    reason = "one arm PER CODE, like the catalog registry it mirrors — merging arms would hide which codes have a constructed stand-in"
)]
fn canonical_payload(slug: &str) -> Option<Diag> {
    let code = match slug {
        // phase-5 backport: the covered give-up / records / mark-grammar codes.
        "cmdsub-operand-top" => DiagCode::CmdsubOperandTop(CmdsubOperandTop {
            site: SiteId::leaf(LeafId(3)),
            position: OperandPosition::Operand(1),
            cause: None,
            top_cause: TopCause::UnmodeledExpansion,
            command: CommandName::Literal("apt-get".to_owned()),
        }),
        "site-unresolvable" => DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: SiteId::leaf(LeafId(4)),
            detail: "2 sites run unprobed (no read-only check could be shipped): \
                     `make install`, `ldconfig`"
                .to_owned(),
        }),
        "render-heredoc-refused" => DiagCode::RenderHeredocRefused(RenderHeredocRefused {
            site: SiteId::leaf(LeafId(7)),
            verb: "elide",
            command: "cat <<EOF".to_owned(),
        }),
        "syntax-unsupported" => DiagCode::SyntaxUnsupported(SyntaxUnsupported {
            detail: "process substitution `<(...)` is not modeled".to_owned(),
        }),
        "records-fact-truncated" => DiagCode::RecordsFactTruncated(RecordsFactTruncated {
            received: 3,
            declared: 5,
            unseen: 2,
        }),
        "host-evidence-admission-refused" => {
            DiagCode::HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused {
                kind: HostEvidenceRefusalKind::Framing,
            })
        }
        "escalation-policy" => DiagCode::EscalationPolicy(EscalationPolicy {
            detail: "escalation policy: probe re-uses connection authority for \
                     `tolerates:`-vouched functions only (default)"
                .to_owned(),
        }),
        "carried-across-substrate-axis" => {
            DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                detail: "elision carried across the fs-view axis: backing kind `sm_dorc_File` \
                         vouches `invariant:fs-view`; the verdict body is read-set-closed"
                    .to_owned(),
            })
        }
        "wrapper-peel-incoherent" => DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
            detail: "wrapper `sudo`: __predict and __lend_map disagree on the peel tail \
                     position (predict reaches \"$@\" after 1 argv token(s), lend_map after 0)"
                .to_owned(),
        }),
        "aid-unloaded-sibling-oracle" => {
            DiagCode::AidUnloadedSiblingOracle(AidUnloadedSiblingOracle {
                detail: "1 sibling oracle exists on disk but was not loaded: `redis.oracle.sh`"
                    .to_owned(),
            })
        }
        "dangling-reference" => DiagCode::DanglingReference(DanglingReference {
            coord: "sm.dorc.Package:nginx".to_owned(),
        }),
        // The external-linter trio: world-as-payload by necessity — replay never runs a foreign
        // tool (`tools_enabled: false`).
        "lint-tool-absent" => DiagCode::LintToolAbsent(LintToolAbsent {
            tool: "shellcheck".to_owned(),
        }),
        "lint-tool-output-unparsable" => {
            DiagCode::LintToolOutputUnparsable(LintToolOutputUnparsable {
                tool: "checkbashisms".to_owned(),
                output: "possible bashism in - line 4 (should be '.'):".to_owned(),
            })
        }
        "lint-tool-failed-without-findings" => {
            DiagCode::LintToolFailedWithoutFindings(LintToolFailedWithoutFindings {
                tool: "shellcheck".to_owned(),
                rc: 2,
            })
        }
        // The invocation errors whose world is an I/O FAILURE, not an argv — an honest trigger
        // would need a real unreadable file, a full disk, or an absent `sh`.
        "cli-file-not-found" => DiagCode::CliFileNotFound(CliFileNotFound {
            kind: "book".to_owned(),
            path: "webhost.sh".to_owned(),
        }),
        "cli-file-permission-denied" => {
            DiagCode::CliFilePermissionDenied(CliFilePermissionDenied {
                kind: "oracle".to_owned(),
                path: "/etc/dorc/nginx.oracle.sh".to_owned(),
            })
        }
        "cli-file-unreadable" => DiagCode::CliFileUnreadable(CliFileUnreadable {
            kind: "results".to_owned(),
            path: "probe-results.txt".to_owned(),
            detail: "Is a directory (os error 21)".to_owned(),
        }),
        "cli-shim-dir-unwritable" => DiagCode::CliShimDirUnwritable(CliShimDirUnwritable {
            path: "/run/dorc/shims".to_owned(),
            detail: "Read-only file system (os error 30)".to_owned(),
        }),
        "whylog-unwritten" => DiagCode::WhylogUnwritten(WhylogUnwritten {
            dir: "/var/lib/dorc/whylog".to_owned(),
            reason: "directory".to_owned(),
        }),
        "lint-no-lintable-files" => DiagCode::LintNoLintableFiles(LintNoLintableFiles),
        "lint-file-count-drift" => DiagCode::LintFileCountDrift(LintFileCountDrift {
            expected: 12,
            found: 9,
        }),
        "lint-required-tools-missing" => {
            DiagCode::LintRequiredToolsMissing(LintRequiredToolsMissing {
                tools: "checkbashisms, shellcheck".to_owned(),
            })
        }
        "dorc-sh-script-unreadable" => DiagCode::DorcShScriptUnreadable(DorcShScriptUnreadable {
            path: "webhost.sh".to_owned(),
            detail: "No such file or directory (os error 2)".to_owned(),
        }),
        "dorc-sh-exec-failed" => DiagCode::DorcShExecFailed(DorcShExecFailed {
            detail: "No such file or directory (os error 2)".to_owned(),
        }),
        _ => return None,
    };
    Some(Diag::new_spanless_site(code))
}

/// World-as-pipeline for the marker pilot (`28A` §2n): fire the REAL in-process marker gate over the
/// materialized oracle `source`, returning its (spanned) diagnostic + the source the caret frame
/// resolves against. Refuses if the gate fired nothing or a different code than the case declares
/// (the honest-trigger coherence the world-as-pipeline form buys).
fn fire_lint_case(
    slug: &str,
    filename: &str,
    source: &str,
) -> Result<(Diag, String, String), String> {
    let result = dorc_lint::lint_materialized_source(
        filename.to_owned(),
        source.to_owned(),
        dorc_lint::SourcePolicy {
            tools_enabled: false,
        },
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
/// REAL pipeline (parse → cfg → value → classify with NO oracles loaded) over the materialized
/// `book.sh`, returning the (spanned) diagnostic whose slug matches the case's `code` + the source its
/// caret frame resolves against. The ⊤-operand disclosure fires before any oracle argparse, so an
/// empty [`dorc_oracle::KindIndex`] suffices; the whole path is kernel-pure (`inv-determinism`).
/// Refuses if the pipeline fired nothing matching the declared slug (honest-trigger coherence).
fn fire_book_analysis(
    slug: &str,
    filename: &str,
    source: &str,
) -> Result<(Diag, String, String), String> {
    let mut interner = Interner::default();
    let parsed = dorc_syntax::parse(source);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
    let idx = dorc_oracle::KindIndex::default();
    let mut arena = ProvArena::new();
    let diag = dorc_analysis::effect::classify(
        &cfg.value,
        &value,
        &parsed.value,
        &idx,
        &[],
        &BTreeSet::new(),
        &mut interner,
        &mut arena,
    )
    .diags
    .into_iter()
    .find(|d| d.code.slug() == slug)
    .ok_or_else(|| format!("world-as-pipeline `{slug}` fired no `{slug}` diagnostic"))?;
    Ok((diag, source.to_owned(), filename.to_owned()))
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
        CompileRefusal, DorcSectionEditRefusal, compile_fragments, compile_preview,
        compile_section_edit, render_compile_preview,
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
    fn malformed_unknown_and_attached_markers_refuse_structurally() {
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
        assert!(matches!(
            compile_section_edit(&baseline, "run ({{command}})"),
            Err(DorcSectionEditRefusal::Compile(
                CompileRefusal::AttachedMarker(_)
            ))
        ));
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
                vec![variable("command", 0, "")],
            )),
            RenderComponent::EditableSection(EditableSection::new(
                key(1),
                vec![variable("command", 0, "")],
            )),
        ]);
        let shared_boundary_result = compile_section_edit(&shared_boundary, "{{command}}");
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
    #[test]
    fn inspection_renders_only_the_touched_section_as_a_template() {
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

        let interpretation = render_compile_preview(&preview);
        assert_eq!(
            interpretation,
            "section: code.message#0:0\n  run {{command}} using {{path}}\n  {{command}} = \"$@\"\n  {{path}} = %LOCALAPPDATA%\\dorc"
        );
        // An untouched section and a fixed variable are another render's business; showing them
        // here would bury the one thing this view exists to expose.
        assert!(!interpretation.contains("hidden"));
        assert!(!interpretation.contains("foreign"));
    }

    /// A whole-page arrangement section is one hole-free span, so the first cut of this render
    /// escaped a sixty-line help page into a single line and buried the edit it was meant to
    /// expose. With no variables there is nothing here git's word-diff does not show better.
    #[test]
    fn a_hole_free_section_renders_as_one_line() {
        let page = key(0);
        let baseline = baseline(vec![RenderComponent::EditableSection(
            EditableSection::new(
                page,
                vec![EditableFragment::Text(String::from("first\nsecond\nthird"))],
            ),
        )]);
        let preview = compile_preview(&baseline, "first\nsecond changed\nthird")
            .unwrap_or_else(|error| panic!("{error:?}"));
        assert_eq!(
            render_compile_preview(&preview),
            "section: code.message#0:0\n  no variables"
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
                .and_then(|entry| entry.message.as_deref()),
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
