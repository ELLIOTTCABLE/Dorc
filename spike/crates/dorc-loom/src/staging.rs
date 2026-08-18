//! The bounded, content-bound staged publication.
//!
//! A publish that gives up a hole writes nothing and stages what it computed instead
//! (`30C:rul-any-hole-loss-confirms`). `--verbatim` recomputes the whole interpretation and applies
//! it only when the fresh bytes equal the staged ones, so the confirmation binds the exact case
//! bytes, catalog input, and replay results the author was shown — never merely the fact that some
//! earlier run happened (`282:rul-promote-requires-fresh-compilation`, which outlives the verb
//! that named it).

use std::fmt;

use errorloom::{EditableFragment, EditableRender, RenderComponent};

use crate::{
    CompilePreview, CompiledFragment, SectionKey, SectionVariableId, StagingStore,
    StagingWriteOutcome, TemplateVariableName,
};

const STAGING_SCHEMA: u32 = 1;
const STAGING_SEMANTICS_EPOCH: u32 = 1;
/// Maximum accepted packet size at the persistence boundary.
pub const MAX_STAGING_BYTES: usize = 2 * 1024 * 1024;
/// Bounds the staged packet, not the corpus: `--all` selects EVERY committed case, so this has to
/// stay ahead of the collection or publishing stops working the moment one more case lands (it did,
/// at 65).
const MAX_STAGED_CASES: usize = 512;
const MAX_STAGED_REPLAYS: usize = 512;
const MAX_STAGED_FIELD_BYTES: usize = 256 * 1024;
const MAX_RENDER_COMPONENTS: usize = 4_096;
const MAX_EDITABLE_FRAGMENTS: usize = 4_096;
const MAX_COMPILED_SECTIONS: usize = 1_024;
const MAX_COMPILED_FRAGMENTS: usize = 4_096;
const MAX_BINDINGS: usize = 1_024;

/// The complete private inspection that promotion must recompute byte-for-byte.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StagedPublication {
    catalog: String,
    selected_cases: Vec<String>,
    touched_cases: Vec<String>,
    cases: Vec<CaseInspection>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct CaseInspection {
    path: String,
    text: String,
    touched: bool,
    replays: Vec<ReplayInspection>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct ReplayInspection {
    ordinal: usize,
    command: String,
    result: String,
    species: ReplaySpecies,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum ReplaySpecies {
    BytesOnly,
    Editable {
        render: StagedRender,
        sections: Vec<StagedSection>,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct StagedRender {
    components: Vec<StagedComponent>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum StagedComponent {
    Structure(String),
    FixedVariable {
        id: StagedVariableId,
        rendered: String,
    },
    EditableSection {
        id: StagedSectionId,
        fragments: Vec<StagedFragment>,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum StagedFragment {
    Text(String),
    Variable {
        id: StagedVariableId,
        rendered: String,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct StagedSection {
    id: StagedSectionId,
    fragments: Vec<StagedCompiledFragment>,
    bindings: Vec<(String, String)>,
    concrete: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum StagedCompiledFragment {
    Text(String),
    Variable(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct StagedSectionId {
    owner: String,
    field: String,
    instance: usize,
    segment: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct StagedVariableId {
    name: String,
    occurrence: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// A bounded refusal from the closed staging grammar.
pub enum StagingError {
    /// A bounded packet resource was exceeded.
    Limit(&'static str),
    /// A packet did not represent the exact typed schema.
    Malformed(&'static str),
}

impl fmt::Display for StagingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Limit(what) => write!(f, "staging limit exceeded: {what}"),
            Self::Malformed(what) => write!(f, "malformed staging: {what}"),
        }
    }
}

impl std::error::Error for StagingError {}

impl StagedPublication {
    /// Construct the inspection at the binary boundary while retaining private fields.
    ///
    /// # Errors
    ///
    /// Returns a refusal when the supplied canonical record is incomplete or invalid.
    pub fn new(
        catalog: String,
        mut selected_cases: Vec<String>,
        mut touched_cases: Vec<String>,
        cases: Vec<(String, String, bool, Vec<StagedReplay>)>,
    ) -> Result<Self, StagingError> {
        let mut cases: Vec<_> = cases
            .into_iter()
            .map(|(path, text, touched, replays)| CaseInspection {
                path,
                text,
                touched,
                replays: replays.into_iter().map(StagedReplay::into_inner).collect(),
            })
            .collect();
        selected_cases.sort();
        touched_cases.sort();
        cases.sort_by(|left, right| left.path.cmp(&right.path));
        let inspection = Self {
            catalog,
            selected_cases,
            touched_cases,
            cases,
        };
        validate(&inspection)?;
        Ok(inspection)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
/// One replay record built by the command adapter.
pub struct StagedReplay(ReplayInspection);

impl StagedReplay {
    /// Record a bytes-only result with no prose authority.
    #[must_use]
    pub fn bytes(ordinal: usize, command: String, result: String) -> Self {
        Self(ReplayInspection {
            ordinal,
            command,
            result,
            species: ReplaySpecies::BytesOnly,
        })
    }

    /// Record a result with its renderer-stamped typed provenance.
    pub fn editable(
        ordinal: usize,
        command: String,
        result: String,
        render: &EditableRender<SectionKey, SectionVariableId>,
        previews: &[CompilePreview],
    ) -> Self {
        let sections = previews
            .iter()
            .flat_map(CompilePreview::sections)
            .map(|section| StagedSection {
                id: section_id(section.section()),
                fragments: section.fragments().iter().map(compiled_fragment).collect(),
                bindings: section
                    .used_bindings()
                    .iter()
                    .map(|(name, value)| (name.0.clone(), value.clone()))
                    .collect(),
                concrete: section_concrete(section),
            })
            .collect();
        Self(ReplayInspection {
            ordinal,
            command,
            result,
            species: ReplaySpecies::Editable {
                render: staged_render(render),
                sections,
            },
        })
    }

    fn into_inner(self) -> ReplayInspection {
        self.0
    }
}

/// Encode the exact typed inspection into the versioned plain-text packet.
///
/// # Errors
///
/// Returns a refusal when the inspection exceeds a bound or violates canonical ordering.
pub fn encode(inspection: &StagedPublication) -> Result<Vec<u8>, StagingError> {
    validate(inspection)?;
    let mut out = Vec::new();
    out.extend_from_slice(
        format!(
            "dorc-loom-staging\nschema: {STAGING_SCHEMA}\nsemantics: {STAGING_SEMANTICS_EPOCH}\nidentity-mode: exact\n"
        )
        .as_bytes(),
    );
    field(&mut out, "catalog", &inspection.catalog)?;
    strings(&mut out, "selected", &inspection.selected_cases)?;
    strings(&mut out, "touched", &inspection.touched_cases)?;
    for case in &inspection.cases {
        field(&mut out, "case", "")?;
        field(&mut out, "path", &case.path)?;
        field(&mut out, "text", &case.text)?;
        field(&mut out, "touched", if case.touched { "1" } else { "0" })?;
        for replay in &case.replays {
            field(&mut out, "replay", "")?;
            field(&mut out, "ordinal", &replay.ordinal.to_string())?;
            field(&mut out, "command", &replay.command)?;
            field(&mut out, "result", &replay.result)?;
            match &replay.species {
                ReplaySpecies::BytesOnly => field(&mut out, "species", "bytes")?,
                ReplaySpecies::Editable { render, sections } => {
                    field(&mut out, "species", "editable")?;
                    encode_render(&mut out, render)?;
                    for section in sections {
                        field(&mut out, "compiled", "")?;
                        encode_section_id(&mut out, &section.id)?;
                        for fragment in &section.fragments {
                            match fragment {
                                StagedCompiledFragment::Text(text) => {
                                    field(&mut out, "ctext", text)?;
                                }
                                StagedCompiledFragment::Variable(name) => {
                                    field(&mut out, "cvariable", name)?;
                                }
                            }
                        }
                        for (name, value) in &section.bindings {
                            field(&mut out, "binding", "")?;
                            field(&mut out, "name", name)?;
                            field(&mut out, "value", value)?;
                        }
                        field(&mut out, "concrete", &section.concrete)?;
                        field(&mut out, "end-compiled", "")?;
                    }
                    field(&mut out, "end-editable", "")?;
                }
            }
            field(&mut out, "end-replay", "")?;
        }
        field(&mut out, "end-case", "")?;
    }
    if out.len() > MAX_STAGING_BYTES {
        return Err(StagingError::Limit("total bytes"));
    }
    Ok(out)
}

pub(crate) fn parse(packet: &[u8]) -> Result<StagedPublication, StagingError> {
    if packet.len() > MAX_STAGING_BYTES {
        return Err(StagingError::Limit("total bytes"));
    }
    let prefix = format!(
        "dorc-loom-staging\nschema: {STAGING_SCHEMA}\nsemantics: {STAGING_SEMANTICS_EPOCH}\nidentity-mode: exact\n"
    );
    let Some(rest) = packet.strip_prefix(prefix.as_bytes()) else {
        return Err(StagingError::Malformed("header"));
    };
    let mut frames = Frames::new(rest);
    let catalog = frames.required("catalog")?;
    let selected_cases = read_strings(&mut frames, "selected")?;
    let touched_cases = read_strings(&mut frames, "touched")?;
    let mut cases = Vec::new();
    while frames.peek_tag()? == Some("case") {
        frames.required("case")?;
        let path = frames.required("path")?;
        let text = frames.required("text")?;
        let touched = match frames.required("touched")?.as_str() {
            "0" => false,
            "1" => true,
            _ => return Err(StagingError::Malformed("touched enum")),
        };
        let mut replays = Vec::new();
        while frames.peek_tag()? == Some("replay") {
            frames.required("replay")?;
            let ordinal = number(&frames.required("ordinal")?)?;
            let command = frames.required("command")?;
            let result = frames.required("result")?;
            let species = match frames.required("species")?.as_str() {
                "bytes" => ReplaySpecies::BytesOnly,
                "editable" => {
                    let render = parse_render(&mut frames)?;
                    let mut sections = Vec::new();
                    while frames.peek_tag()? == Some("compiled") {
                        frames.required("compiled")?;
                        let id = parse_section_id(&mut frames)?;
                        let mut fragments = Vec::new();
                        while matches!(frames.peek_tag()?, Some("ctext" | "cvariable")) {
                            let (tag, value) =
                                frames.next()?.ok_or(StagingError::Malformed("compiled"))?;
                            fragments.push(if tag == "ctext" {
                                StagedCompiledFragment::Text(value)
                            } else {
                                StagedCompiledFragment::Variable(value)
                            });
                        }
                        let mut bindings = Vec::new();
                        while frames.peek_tag()? == Some("binding") {
                            frames.required("binding")?;
                            bindings.push((frames.required("name")?, frames.required("value")?));
                        }
                        let concrete = frames.required("concrete")?;
                        frames.required("end-compiled")?;
                        sections.push(StagedSection {
                            id,
                            fragments,
                            bindings,
                            concrete,
                        });
                    }
                    frames.required("end-editable")?;
                    ReplaySpecies::Editable { render, sections }
                }
                _ => return Err(StagingError::Malformed("species enum")),
            };
            frames.required("end-replay")?;
            replays.push(ReplayInspection {
                ordinal,
                command,
                result,
                species,
            });
        }
        frames.required("end-case")?;
        cases.push(CaseInspection {
            path,
            text,
            touched,
            replays,
        });
    }
    if frames.next()?.is_some() {
        return Err(StagingError::Malformed("trailing bytes"));
    }
    let inspection = StagedPublication {
        catalog,
        selected_cases,
        touched_cases,
        cases,
    };
    validate(&inspection)?;
    Ok(inspection)
}

/// Accept a stored packet only after parsing it AND recomputing its exact canonical bytes.
///
/// # Errors
///
/// Returns a refusal when the packet is invalid or was computed from different bytes.
pub fn validate_current(packet: &[u8], current: &StagedPublication) -> Result<(), StagingError> {
    let _ = parse(packet)?;
    if packet != encode(current)? {
        return Err(StagingError::Malformed(
            "the staged interpretation was computed from different bytes",
        ));
    }
    Ok(())
}

/// Hold one interpretation for a later `--verbatim` to confirm.
///
/// # Errors
///
/// Returns encoding or storage failures without changing any source file.
pub fn stage(
    store: &impl StagingStore,
    publication: &StagedPublication,
) -> Result<StagingWriteOutcome, String> {
    let packet = encode(publication).map_err(|error| error.to_string())?;
    store.publish(&packet)
}

/// Require the stored staging to match this fresh interpretation exactly.
///
/// # Errors
///
/// Returns a refusal, naming the plain re-run, without writing anything.
pub fn accept_staged(
    store: &impl StagingStore,
    publication: &StagedPublication,
    cases: &str,
) -> Result<(), String> {
    let packet = store
        .read()
        .map_err(|error| format!("read the staged publication: {error}"))?
        .ok_or_else(|| stale_staging("nothing is staged", cases))?;
    validate_current(&packet, publication).map_err(|error| stale_staging(&error.to_string(), cases))
}

/// One refusal for both ways a `--verbatim` can find no interpretation to apply, because a reader
/// hitting either needs the same next command (`28L:rul-refusals-name-the-next-command`): see the
/// loss, then confirm it.
fn stale_staging(why: &str, cases: &str) -> String {
    format!(
        "--verbatim applies an interpretation you have already been shown, and {why}. Run \
         `dorc-loom publish {cases}` to see what it gives up, then re-run with --verbatim."
    )
}

fn validate(inspection: &StagedPublication) -> Result<(), StagingError> {
    check(&inspection.catalog)?;
    ordered_paths(&inspection.selected_cases, "selected paths")?;
    ordered_paths(&inspection.touched_cases, "touched paths")?;
    if inspection.cases.is_empty() || inspection.cases.len() > MAX_STAGED_CASES {
        return Err(StagingError::Limit("case count"));
    }
    if inspection.selected_cases.len() != inspection.cases.len()
        || inspection
            .selected_cases
            .iter()
            .zip(&inspection.cases)
            .any(|(path, case)| path != &case.path)
    {
        return Err(StagingError::Malformed("selected case set"));
    }
    let expected_touched: Vec<_> = inspection
        .cases
        .iter()
        .filter(|case| case.touched)
        .map(|case| case.path.clone())
        .collect();
    if expected_touched != inspection.touched_cases {
        return Err(StagingError::Malformed("touched case set"));
    }
    let mut replay_count = 0usize;
    let mut prior_path: Option<String> = None;
    for case in &inspection.cases {
        if !safe_path(&case.path) || prior_path.as_ref().is_some_and(|path| path >= &case.path) {
            return Err(StagingError::Malformed("case ordering"));
        }
        prior_path = Some(case.path.clone());
        check(&case.path)?;
        check(&case.text)?;
        for (expected, replay) in case.replays.iter().enumerate() {
            if replay.ordinal != expected {
                return Err(StagingError::Malformed("replay ordinal"));
            }
            replay_count = replay_count
                .checked_add(1)
                .ok_or(StagingError::Limit("replay count"))?;
            check(&replay.command)?;
            check(&replay.result)?;
            validate_species(&replay.species)?;
        }
    }
    if replay_count > MAX_STAGED_REPLAYS {
        return Err(StagingError::Limit("replay count"));
    }
    Ok(())
}

fn validate_species(species: &ReplaySpecies) -> Result<(), StagingError> {
    let ReplaySpecies::Editable { render, sections } = species else {
        return Ok(());
    };
    if render.components.len() > MAX_RENDER_COMPONENTS {
        return Err(StagingError::Limit("render components"));
    }
    if sections.len() > MAX_COMPILED_SECTIONS {
        return Err(StagingError::Limit("compiled sections"));
    }
    let mut render_sections = std::collections::BTreeSet::new();
    for component in &render.components {
        match component {
            StagedComponent::Structure(text) => {
                check(text)?;
            }
            StagedComponent::FixedVariable { id, rendered } => {
                validate_variable_id(id)?;
                check(rendered)?;
            }
            StagedComponent::EditableSection { id, fragments } => {
                validate_section_id(id)?;
                if !render_sections.insert(section_identity(id)) {
                    return Err(StagingError::Malformed("duplicate editable section"));
                }
                if fragments.len() > MAX_EDITABLE_FRAGMENTS {
                    return Err(StagingError::Limit("editable fragments"));
                }
                for fragment in fragments {
                    validate_fragment(fragment)?;
                }
            }
        }
    }
    let mut compiled_sections = std::collections::BTreeSet::new();
    for section in sections {
        validate_section_id(&section.id)?;
        let identity = section_identity(&section.id);
        if !render_sections.contains(&identity) {
            return Err(StagingError::Malformed(
                "compiled section has no render section",
            ));
        }
        if !compiled_sections.insert(identity) {
            return Err(StagingError::Malformed("duplicate compiled section"));
        }
        if section.fragments.len() > MAX_COMPILED_FRAGMENTS {
            return Err(StagingError::Limit("compiled fragments"));
        }
        if section.bindings.len() > MAX_BINDINGS {
            return Err(StagingError::Limit("bindings"));
        }
        check(&section.concrete)?;
        for fragment in &section.fragments {
            match fragment {
                StagedCompiledFragment::Text(text) | StagedCompiledFragment::Variable(text) => {
                    check(text)?;
                }
            }
        }
        let mut names = std::collections::BTreeSet::new();
        for (name, value) in &section.bindings {
            check_nonempty(name, "binding name")?;
            if !names.insert(name) {
                return Err(StagingError::Malformed("duplicate binding"));
            }
            check(value)?;
        }
    }
    Ok(())
}

fn ordered_paths(paths: &[String], what: &'static str) -> Result<(), StagingError> {
    let mut prior = None;
    for path in paths {
        if !safe_path(path) || prior.is_some_and(|value: &String| value >= path) {
            return Err(StagingError::Malformed(what));
        }
        prior = Some(path);
        check(path)?;
    }
    Ok(())
}
fn validate_section_id(id: &StagedSectionId) -> Result<(), StagingError> {
    check_nonempty(&id.owner, "section owner")?;
    check_nonempty(&id.field, "section field")
}
fn validate_variable_id(id: &StagedVariableId) -> Result<(), StagingError> {
    check_nonempty(&id.name, "variable name")
}
fn validate_fragment(fragment: &StagedFragment) -> Result<(), StagingError> {
    match fragment {
        StagedFragment::Text(text) => check(text),
        StagedFragment::Variable { id, rendered } => {
            validate_variable_id(id)?;
            check(rendered)
        }
    }
}
fn check(value: &str) -> Result<(), StagingError> {
    if value.len() > MAX_STAGED_FIELD_BYTES {
        Err(StagingError::Limit("field bytes"))
    } else {
        Ok(())
    }
}
fn check_nonempty(value: &str, what: &'static str) -> Result<(), StagingError> {
    if value.is_empty() {
        Err(StagingError::Malformed(what))
    } else {
        check(value)
    }
}
fn section_identity(id: &StagedSectionId) -> (&str, &str, usize, usize) {
    (&id.owner, &id.field, id.instance, id.segment)
}
fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':', '\0'])
        && path.split('/').all(|part| !matches!(part, "" | "." | ".."))
}
fn number(value: &str) -> Result<usize, StagingError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(StagingError::Malformed("number"));
    }
    value
        .parse()
        .map_err(|_| StagingError::Malformed("number overflow"))
}

fn staged_render(render: &EditableRender<SectionKey, SectionVariableId>) -> StagedRender {
    StagedRender {
        components: render
            .components()
            .iter()
            .map(|component| match component {
                RenderComponent::Structure(text) => StagedComponent::Structure(text.clone()),
                RenderComponent::FixedVariable { id, rendered } => StagedComponent::FixedVariable {
                    id: variable_id(id),
                    rendered: rendered.clone(),
                },
                RenderComponent::EditableSection(section) => StagedComponent::EditableSection {
                    id: section_id(section.id()),
                    fragments: section.fragments().iter().map(editable_fragment).collect(),
                },
            })
            .collect(),
    }
}
fn editable_fragment(fragment: &EditableFragment<SectionVariableId>) -> StagedFragment {
    match fragment {
        EditableFragment::Text(text) => StagedFragment::Text(text.clone()),
        EditableFragment::Variable { id, rendered } => StagedFragment::Variable {
            id: variable_id(id),
            rendered: rendered.clone(),
        },
    }
}
fn compiled_fragment(fragment: &CompiledFragment) -> StagedCompiledFragment {
    match fragment {
        CompiledFragment::Text(text) => StagedCompiledFragment::Text(text.clone()),
        CompiledFragment::Variable(TemplateVariableName(name)) => {
            StagedCompiledFragment::Variable(name.clone())
        }
    }
}
fn section_id(id: &SectionKey) -> StagedSectionId {
    StagedSectionId {
        owner: id.owner.clone(),
        field: id.field.to_owned(),
        instance: id.instance,
        segment: id.segment,
    }
}
fn variable_id(id: &SectionVariableId) -> StagedVariableId {
    StagedVariableId {
        name: id.name.0.clone(),
        occurrence: id.occurrence,
    }
}
fn section_concrete(section: &crate::SectionPreview) -> String {
    section
        .fragments()
        .iter()
        .map(|fragment| match fragment {
            CompiledFragment::Text(text) => text.clone(),
            CompiledFragment::Variable(name) => section
                .used_bindings()
                .iter()
                .find(|(bound, _)| bound == name)
                .map_or_else(String::new, |(_, value)| value.clone()),
        })
        .collect()
}

fn strings(out: &mut Vec<u8>, tag: &str, values: &[String]) -> Result<(), StagingError> {
    for value in values {
        field(out, tag, value)?;
    }
    Ok(())
}
fn encode_render(out: &mut Vec<u8>, render: &StagedRender) -> Result<(), StagingError> {
    for component in &render.components {
        match component {
            StagedComponent::Structure(text) => field(out, "structure", text)?,
            StagedComponent::FixedVariable { id, rendered } => {
                field(out, "fixed", "")?;
                encode_variable_id(out, id)?;
                field(out, "rendered", rendered)?;
            }
            StagedComponent::EditableSection { id, fragments } => {
                field(out, "editable", "")?;
                encode_section_id(out, id)?;
                for fragment in fragments {
                    match fragment {
                        StagedFragment::Text(text) => field(out, "text", text)?,
                        StagedFragment::Variable { id, rendered } => {
                            field(out, "variable", "")?;
                            encode_variable_id(out, id)?;
                            field(out, "rendered", rendered)?;
                        }
                    }
                }
                field(out, "end-editable-section", "")?;
            }
        }
    }
    Ok(())
}
fn encode_section_id(out: &mut Vec<u8>, id: &StagedSectionId) -> Result<(), StagingError> {
    field(out, "owner", &id.owner)?;
    field(out, "field", &id.field)?;
    field(out, "instance", &id.instance.to_string())?;
    field(out, "segment", &id.segment.to_string())
}
fn encode_variable_id(out: &mut Vec<u8>, id: &StagedVariableId) -> Result<(), StagingError> {
    field(out, "name", &id.name)?;
    field(out, "occurrence", &id.occurrence.to_string())
}
fn parse_render(frames: &mut Frames<'_>) -> Result<StagedRender, StagingError> {
    let mut components = Vec::new();
    while matches!(frames.peek_tag()?, Some("structure" | "fixed" | "editable")) {
        match frames.peek_tag()? {
            Some("structure") => {
                components.push(StagedComponent::Structure(frames.required("structure")?));
            }
            Some("fixed") => {
                frames.required("fixed")?;
                let id = parse_variable_id(frames)?;
                let rendered = frames.required("rendered")?;
                components.push(StagedComponent::FixedVariable { id, rendered });
            }
            Some("editable") => {
                frames.required("editable")?;
                let id = parse_section_id(frames)?;
                let mut fragments = Vec::new();
                while matches!(frames.peek_tag()?, Some("text" | "variable")) {
                    if frames.peek_tag()? == Some("text") {
                        fragments.push(StagedFragment::Text(frames.required("text")?));
                    } else {
                        frames.required("variable")?;
                        let id = parse_variable_id(frames)?;
                        let rendered = frames.required("rendered")?;
                        fragments.push(StagedFragment::Variable { id, rendered });
                    }
                }
                frames.required("end-editable-section")?;
                components.push(StagedComponent::EditableSection { id, fragments });
            }
            _ => return Err(StagingError::Malformed("render tag")),
        }
    }
    Ok(StagedRender { components })
}
fn parse_section_id(frames: &mut Frames<'_>) -> Result<StagedSectionId, StagingError> {
    Ok(StagedSectionId {
        owner: frames.required("owner")?,
        field: frames.required("field")?,
        instance: number(&frames.required("instance")?)?,
        segment: number(&frames.required("segment")?)?,
    })
}
fn parse_variable_id(frames: &mut Frames<'_>) -> Result<StagedVariableId, StagingError> {
    Ok(StagedVariableId {
        name: frames.required("name")?,
        occurrence: number(&frames.required("occurrence")?)?,
    })
}

fn field(out: &mut Vec<u8>, tag: &str, value: &str) -> Result<(), StagingError> {
    check(value)?;
    let header = format!("{tag} {}\n", value.len());
    if out
        .len()
        .checked_add(header.len())
        .and_then(|length| length.checked_add(value.len()))
        .and_then(|length| length.checked_add(1))
        .is_none_or(|length| length > MAX_STAGING_BYTES)
    {
        return Err(StagingError::Limit("total bytes"));
    }
    out.extend_from_slice(header.as_bytes());
    out.extend_from_slice(value.as_bytes());
    out.push(b'\n');
    Ok(())
}

struct Frames<'a> {
    rest: &'a [u8],
}
impl<'a> Frames<'a> {
    fn new(rest: &'a [u8]) -> Self {
        Self { rest }
    }
    fn peek_tag(&self) -> Result<Option<&str>, StagingError> {
        let Some(line_end) = self.rest.iter().position(|byte| *byte == b'\n') else {
            return if self.rest.is_empty() {
                Ok(None)
            } else {
                Err(StagingError::Malformed("frame header"))
            };
        };
        let line = std::str::from_utf8(
            self.rest
                .get(..line_end)
                .ok_or(StagingError::Malformed("frame header"))?,
        )
        .map_err(|_| StagingError::Malformed("header UTF-8"))?;
        Ok(Some(
            line.split_once(' ')
                .ok_or(StagingError::Malformed("frame header"))?
                .0,
        ))
    }
    fn next(&mut self) -> Result<Option<(&str, String)>, StagingError> {
        if self.rest.is_empty() {
            return Ok(None);
        }
        let line_end = self
            .rest
            .iter()
            .position(|byte| *byte == b'\n')
            .ok_or(StagingError::Malformed("frame header"))?;
        let line = std::str::from_utf8(
            self.rest
                .get(..line_end)
                .ok_or(StagingError::Malformed("frame header"))?,
        )
        .map_err(|_| StagingError::Malformed("header UTF-8"))?;
        let (tag, length) = line
            .split_once(' ')
            .ok_or(StagingError::Malformed("frame header"))?;
        if tag.is_empty() || length.is_empty() || !length.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(StagingError::Malformed("frame header"));
        }
        let length: usize = length
            .parse()
            .map_err(|_| StagingError::Malformed("frame length overflow"))?;
        if length > MAX_STAGED_FIELD_BYTES {
            return Err(StagingError::Limit("field bytes"));
        }
        let start = line_end
            .checked_add(1)
            .ok_or(StagingError::Malformed("frame length overflow"))?;
        let end = start
            .checked_add(length)
            .ok_or(StagingError::Malformed("frame length overflow"))?;
        let next = end
            .checked_add(1)
            .ok_or(StagingError::Malformed("frame length overflow"))?;
        if next > self.rest.len() || self.rest.get(end) != Some(&b'\n') {
            return Err(StagingError::Malformed("truncated frame"));
        }
        let value = std::str::from_utf8(
            self.rest
                .get(start..end)
                .ok_or(StagingError::Malformed("truncated frame"))?,
        )
        .map_err(|_| StagingError::Malformed("field UTF-8"))?
        .to_owned();
        self.rest = self
            .rest
            .get(next..)
            .ok_or(StagingError::Malformed("truncated frame"))?;
        Ok(Some((tag, value)))
    }
    fn required(&mut self, wanted: &str) -> Result<String, StagingError> {
        match self.next()? {
            Some((tag, value)) if tag == wanted => Ok(value),
            _ => Err(StagingError::Malformed("field order or tag")),
        }
    }
}
fn read_strings(frames: &mut Frames<'_>, tag: &str) -> Result<Vec<String>, StagingError> {
    let mut values = Vec::new();
    while frames.peek_tag()? == Some(tag) {
        values.push(frames.required(tag)?);
    }
    Ok(values)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use errorloom::EditableSection;

    fn key(code: &str, segment: usize) -> SectionKey {
        SectionKey {
            owner: code.to_owned(),
            field: "message",
            instance: 0,
            segment,
        }
    }
    fn render(
        variable: &str,
        occurrence: usize,
        value: &str,
    ) -> EditableRender<SectionKey, SectionVariableId> {
        EditableRender::new(vec![
            RenderComponent::Structure("S".to_owned()),
            RenderComponent::FixedVariable {
                id: SectionVariableId {
                    name: TemplateVariableName("fixed".to_owned()),
                    occurrence: 0,
                },
                rendered: "F".to_owned(),
            },
            RenderComponent::EditableSection(EditableSection::new(
                key("code", 0),
                vec![
                    EditableFragment::Text("T".to_owned()),
                    EditableFragment::Variable {
                        id: SectionVariableId {
                            name: TemplateVariableName(variable.to_owned()),
                            occurrence,
                        },
                        rendered: value.to_owned(),
                    },
                ],
            )),
        ])
    }
    fn preview(render: &EditableRender<SectionKey, SectionVariableId>) -> CompilePreview {
        CompilePreview {
            sections: vec![],
            concrete: render
                .components()
                .iter()
                .map(|component| match component {
                    RenderComponent::Structure(text) => text.clone(),
                    RenderComponent::FixedVariable { rendered, .. } => rendered.clone(),
                    RenderComponent::EditableSection(section) => section
                        .fragments()
                        .iter()
                        .map(|fragment| match fragment {
                            EditableFragment::Text(text)
                            | EditableFragment::Variable { rendered: text, .. } => text.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
    pub(crate) fn inspection(value: &str) -> StagedPublication {
        let render = render("name", 0, value);
        StagedPublication::new(
            "catalog".to_owned(),
            vec!["cases/a.loom".to_owned()],
            vec!["cases/a.loom".to_owned()],
            vec![(
                "cases/a.loom".to_owned(),
                "case".to_owned(),
                true,
                vec![StagedReplay::editable(
                    0,
                    "dorc plan".to_owned(),
                    value.to_owned(),
                    &render,
                    &[preview(&render)],
                )],
            )],
        )
        .expect("valid")
    }

    pub(crate) fn inspection_mutations() -> (StagedPublication, Vec<StagedPublication>) {
        let mut original = inspection("same");
        let (_, sections) = editable_species(&mut original);
        sections.push(StagedSection {
            id: StagedSectionId {
                owner: "code".to_owned(),
                field: "message".to_owned(),
                instance: 0,
                segment: 0,
            },
            fragments: vec![
                StagedCompiledFragment::Text("compiled ".to_owned()),
                StagedCompiledFragment::Variable("name".to_owned()),
            ],
            bindings: vec![("name".to_owned(), "same".to_owned())],
            concrete: "compiled same".to_owned(),
        });
        let mut variants = Vec::new();

        let mut selected = original.clone();
        selected.selected_cases[0] = "cases/b.loom".to_owned();
        selected.cases[0].path = "cases/b.loom".to_owned();
        selected.touched_cases[0] = "cases/b.loom".to_owned();
        variants.push(selected);
        let mut touched = original.clone();
        touched.touched_cases.clear();
        touched.cases[0].touched = false;
        variants.push(touched);
        let mut text = original.clone();
        text.cases[0].text.push('!');
        variants.push(text);
        let mut catalog = original.clone();
        catalog.catalog.push('!');
        variants.push(catalog);
        let mut command = original.clone();
        command.cases[0].replays[0].command.push('!');
        variants.push(command);
        let mut result = original.clone();
        result.cases[0].replays[0].result.push('!');
        variants.push(result);
        let mut bytes = original.clone();
        bytes.cases[0].replays[0].species = ReplaySpecies::BytesOnly;
        variants.push(bytes);
        let mut structure = original.clone();
        let (render, _) = editable_species(&mut structure);
        render.components[0] = StagedComponent::Structure("changed".to_owned());
        variants.push(structure);
        let mut fixed = original.clone();
        let (render, _) = editable_species(&mut fixed);
        let StagedComponent::FixedVariable { id, rendered } = &mut render.components[1] else {
            panic!("fixed variable");
        };
        id.occurrence = 1;
        rendered.push('!');
        variants.push(fixed);
        let mut fragment = original.clone();
        let (render, _) = editable_species(&mut fragment);
        let StagedComponent::EditableSection { fragments, .. } = &mut render.components[2] else {
            panic!("editable section");
        };
        fragments[0] = StagedFragment::Text("changed".to_owned());
        variants.push(fragment);
        let mut compiled = original.clone();
        let (_, sections) = editable_species(&mut compiled);
        sections[0].fragments[0] = StagedCompiledFragment::Text("changed".to_owned());
        variants.push(compiled);
        let mut binding = original.clone();
        let (_, sections) = editable_species(&mut binding);
        sections[0].bindings[0].1.push('!');
        variants.push(binding);
        let mut concrete = original.clone();
        let (_, sections) = editable_species(&mut concrete);
        sections[0].concrete.push('!');
        variants.push(concrete);
        (original, variants)
    }
    #[test]
    fn typed_packet_round_trips_and_distinguishes_lossy_dimensions() {
        let original = inspection("\0 equal = unicode \u{2603}");
        let packet = encode(&original).expect("encode");
        assert_eq!(parse(&packet), Ok(original.clone()));
        assert!(validate_current(&packet, &original).is_ok());
        for changed in [inspection(""), inspection("\0 equal = unicode \u{2604}")] {
            assert_ne!(packet, encode(&changed).expect("encode"));
        }
    }

    #[test]
    fn exact_model_encoding_is_injective_across_every_bound_dimension() {
        let (original, variants) = inspection_mutations();
        let packet = encode(&original).expect("original packet");
        for variant in variants {
            let changed = encode(&variant).expect("valid changed inspection");
            assert_ne!(packet, changed);
            assert!(validate_current(&packet, &variant).is_err());
        }
    }
    #[test]
    fn packet_is_injective_over_provenance_and_compilation_identity() {
        let original = inspection("same");
        let packet = encode(&original).expect("encode");
        let mut variants = Vec::new();
        let mut bytes = original.clone();
        bytes.cases[0].replays[0].species = ReplaySpecies::BytesOnly;
        variants.push(bytes);
        let mut ordinal = original.clone();
        ordinal.cases[0].replays[0].ordinal = 1;
        assert!(
            encode(&ordinal).is_err(),
            "ordinals are validated rather than normalized"
        );
        let mut case = original.clone();
        case.cases[0].path = "cases/b.loom".to_owned();
        case.selected_cases[0] = "cases/b.loom".to_owned();
        case.touched_cases[0] = "cases/b.loom".to_owned();
        variants.push(case);
        let mut provenance = original.clone();
        let ReplaySpecies::Editable {
            render,
            sections: _,
        } = &mut provenance.cases[0].replays[0].species
        else {
            panic!("editable");
        };
        render.components[0] = StagedComponent::Structure("changed structure".to_owned());
        variants.push(provenance.clone());
        let ReplaySpecies::Editable { render, sections } =
            &mut provenance.cases[0].replays[0].species
        else {
            panic!("editable");
        };
        let StagedComponent::FixedVariable { id, .. } = &mut render.components[1] else {
            panic!("fixed");
        };
        id.occurrence = 1;
        render.components.push(StagedComponent::EditableSection {
            id: StagedSectionId {
                owner: "code".to_owned(),
                field: "help".to_owned(),
                instance: 0,
                segment: 1,
            },
            fragments: Vec::new(),
        });
        sections.push(StagedSection {
            id: StagedSectionId {
                owner: "code".to_owned(),
                field: "help".to_owned(),
                instance: 0,
                segment: 1,
            },
            fragments: vec![
                StagedCompiledFragment::Text("compiled".to_owned()),
                StagedCompiledFragment::Variable("name".to_owned()),
            ],
            bindings: vec![("name".to_owned(), "same".to_owned())],
            concrete: "compiledsame".to_owned(),
        });
        variants.push(provenance);
        for variant in variants {
            assert_ne!(packet, encode(&variant).expect("encode"));
        }
    }
    #[test]
    fn parser_refuses_nested_unknown_and_boundary_overflow() {
        let packet = encode(&inspection("x")).expect("encode");
        for suffix in [b"unknown 0\n\n".as_slice(), b"end-case 0\n\n".as_slice()] {
            let mut malformed = packet.clone();
            malformed.extend_from_slice(suffix);
            assert!(parse(&malformed).is_err());
        }
        assert!(parse(&vec![b'x'; MAX_STAGING_BYTES + 1]).is_err());
    }
    #[test]
    fn canonical_case_order_is_required() {
        let mut inspection = inspection("x");
        inspection.selected_cases = vec!["cases/b.loom".to_owned(), "cases/a.loom".to_owned()];
        assert!(encode(&inspection).is_err());
    }

    #[test]
    fn construction_canonicalizes_shuffled_case_selection_to_one_packet() {
        let ordered = StagedPublication::new(
            "catalog".to_owned(),
            vec!["cases/a.loom".to_owned(), "cases/b.loom".to_owned()],
            vec!["cases/a.loom".to_owned(), "cases/b.loom".to_owned()],
            vec![
                ("cases/a.loom".to_owned(), "a".to_owned(), true, Vec::new()),
                ("cases/b.loom".to_owned(), "b".to_owned(), true, Vec::new()),
            ],
        )
        .expect("ordered inspection");
        let shuffled = StagedPublication::new(
            "catalog".to_owned(),
            vec!["cases/b.loom".to_owned(), "cases/a.loom".to_owned()],
            vec!["cases/b.loom".to_owned(), "cases/a.loom".to_owned()],
            vec![
                ("cases/b.loom".to_owned(), "b".to_owned(), true, Vec::new()),
                ("cases/a.loom".to_owned(), "a".to_owned(), true, Vec::new()),
            ],
        )
        .expect("shuffled inspection");
        assert_eq!(encode(&ordered), encode(&shuffled));
    }

    fn bytes_replays(count: usize) -> StagedPublication {
        StagedPublication::new(
            "catalog".to_owned(),
            vec!["cases/a.loom".to_owned()],
            vec!["cases/a.loom".to_owned()],
            vec![(
                "cases/a.loom".to_owned(),
                "case".to_owned(),
                true,
                (0..count)
                    .map(|ordinal| StagedReplay::bytes(ordinal, "cmd".to_owned(), String::new()))
                    .collect(),
            )],
        )
        .expect("bounded inspection")
    }

    fn cases(count: usize) -> StagedPublication {
        let paths: Vec<_> = (0..count)
            .map(|index| format!("cases/{index:03}.loom"))
            .collect();
        StagedPublication::new(
            "catalog".to_owned(),
            paths.clone(),
            paths.clone(),
            paths
                .iter()
                .map(|path| (path.clone(), String::new(), true, Vec::new()))
                .collect(),
        )
        .expect("bounded inspection")
    }

    fn editable_species(
        inspection: &mut StagedPublication,
    ) -> (&mut StagedRender, &mut Vec<StagedSection>) {
        let ReplaySpecies::Editable { render, sections } =
            &mut inspection.cases[0].replays[0].species
        else {
            panic!("editable");
        };
        (render, sections)
    }

    #[test]
    fn bounded_collections_accept_the_limit_and_refuse_one_more() {
        assert!(encode(&cases(MAX_STAGED_CASES)).is_ok());
        assert!(
            StagedPublication::new(
                "catalog".to_owned(),
                (0..=MAX_STAGED_CASES)
                    .map(|index| format!("cases/{index:03}.loom"))
                    .collect(),
                (0..=MAX_STAGED_CASES)
                    .map(|index| format!("cases/{index:03}.loom"))
                    .collect(),
                (0..=MAX_STAGED_CASES)
                    .map(|index| (
                        format!("cases/{index:03}.loom"),
                        String::new(),
                        true,
                        Vec::new()
                    ))
                    .collect(),
            )
            .is_err()
        );
        assert!(encode(&bytes_replays(MAX_STAGED_REPLAYS)).is_ok());
        assert!(
            StagedPublication::new(
                "catalog".to_owned(),
                vec!["cases/a.loom".to_owned()],
                vec!["cases/a.loom".to_owned()],
                vec![(
                    "cases/a.loom".to_owned(),
                    String::new(),
                    true,
                    (0..=MAX_STAGED_REPLAYS)
                        .map(|ordinal| StagedReplay::bytes(
                            ordinal,
                            "cmd".to_owned(),
                            String::new()
                        ))
                        .collect()
                )],
            )
            .is_err()
        );

        let mut field = inspection("x");
        field.catalog = "x".repeat(MAX_STAGED_FIELD_BYTES);
        assert!(encode(&field).is_ok());
        field.catalog.push('x');
        assert!(encode(&field).is_err());

        let mut components = inspection("x");
        editable_species(&mut components).0.components =
            vec![StagedComponent::Structure(String::new()); MAX_RENDER_COMPONENTS];
        assert!(encode(&components).is_ok());
        editable_species(&mut components)
            .0
            .components
            .push(StagedComponent::Structure(String::new()));
        assert!(encode(&components).is_err());
    }

    #[test]
    fn nested_collections_accept_the_limit_and_refuse_one_more() {
        let mut editable_fragments = inspection("x");
        {
            let (render, _) = editable_species(&mut editable_fragments);
            let StagedComponent::EditableSection { fragments, .. } = &mut render.components[2]
            else {
                panic!("editable section");
            };
            *fragments = vec![StagedFragment::Text(String::new()); MAX_EDITABLE_FRAGMENTS];
        }
        assert!(encode(&editable_fragments).is_ok());
        let StagedComponent::EditableSection { fragments, .. } =
            &mut editable_species(&mut editable_fragments).0.components[2]
        else {
            panic!("editable section");
        };
        fragments.push(StagedFragment::Text(String::new()));
        assert!(encode(&editable_fragments).is_err());

        let mut compiled_fragments = inspection("x");
        editable_species(&mut compiled_fragments)
            .1
            .push(StagedSection {
                id: StagedSectionId {
                    owner: "code".to_owned(),
                    field: "message".to_owned(),
                    instance: 0,
                    segment: 0,
                },
                fragments: vec![
                    StagedCompiledFragment::Text(String::new());
                    MAX_COMPILED_FRAGMENTS
                ],
                bindings: Vec::new(),
                concrete: String::new(),
            });
        assert!(encode(&compiled_fragments).is_ok());
        editable_species(&mut compiled_fragments).1[0]
            .fragments
            .push(StagedCompiledFragment::Text(String::new()));
        assert!(encode(&compiled_fragments).is_err());

        let mut bindings = inspection("x");
        editable_species(&mut bindings).1.push(StagedSection {
            id: StagedSectionId {
                owner: "code".to_owned(),
                field: "message".to_owned(),
                instance: 0,
                segment: 0,
            },
            fragments: Vec::new(),
            bindings: (0..MAX_BINDINGS)
                .map(|index| (format!("name{index}"), String::new()))
                .collect(),
            concrete: String::new(),
        });
        assert!(encode(&bindings).is_ok());
        editable_species(&mut bindings).1[0]
            .bindings
            .push(("one-over".to_owned(), String::new()));
        assert!(encode(&bindings).is_err());

        let mut sections = inspection("x");
        editable_species(&mut sections).0.components = (0..MAX_COMPILED_SECTIONS)
            .map(|segment| StagedComponent::EditableSection {
                id: StagedSectionId {
                    owner: "code".to_owned(),
                    field: "message".to_owned(),
                    instance: 0,
                    segment,
                },
                fragments: Vec::new(),
            })
            .collect();
        *editable_species(&mut sections).1 = (0..MAX_COMPILED_SECTIONS)
            .map(|segment| StagedSection {
                id: StagedSectionId {
                    owner: "code".to_owned(),
                    field: "message".to_owned(),
                    instance: 0,
                    segment,
                },
                fragments: Vec::new(),
                bindings: Vec::new(),
                concrete: String::new(),
            })
            .collect();
        assert!(encode(&sections).is_ok());
        editable_species(&mut sections).1.push(StagedSection {
            id: StagedSectionId {
                owner: "code".to_owned(),
                field: "message".to_owned(),
                instance: 0,
                segment: MAX_COMPILED_SECTIONS,
            },
            fragments: Vec::new(),
            bindings: Vec::new(),
            concrete: String::new(),
        });
        assert!(encode(&sections).is_err());
    }

    #[test]
    fn packet_limit_and_decimal_length_arithmetic_are_checked() {
        let mut source = bytes_replays(7);
        source.catalog = "x".repeat(MAX_STAGED_FIELD_BYTES);
        for replay in &mut source.cases[0].replays[..6] {
            replay.result = "x".repeat(MAX_STAGED_FIELD_BYTES);
        }
        let mut low = 0usize;
        let mut high = MAX_STAGED_FIELD_BYTES;
        while low < high {
            let middle = low.saturating_add(high).saturating_add(1) / 2;
            source.cases[0].replays[6].result = "x".repeat(middle);
            if encode(&source).is_ok() {
                low = middle;
            } else {
                high = middle.saturating_sub(1);
            }
        }
        source.cases[0].replays[6].result = "x".repeat(low);
        let packet = encode(&source).expect("exact boundary packet");
        assert_eq!(packet.len(), MAX_STAGING_BYTES);
        source.cases[0].replays[6].result.push('x');
        assert!(matches!(
            encode(&source),
            Err(StagingError::Limit("total bytes"))
        ));

        let mut malformed = packet.clone();
        let header = b"result ";
        let offset = malformed
            .windows(header.len())
            .position(|window| window == header)
            .expect("catalog header")
            + header.len();
        malformed.splice(
            offset..offset + 6,
            b"999999999999999999999999999999".iter().copied(),
        );
        assert!(parse(&malformed).is_err());
    }

    #[test]
    fn parser_refuses_every_structural_frame_class() {
        let packet = encode(&inspection("unicode \u{2603}\n\0")).expect("packet");
        for prefix in [
            b"wrong\n".as_slice(),
            b"dorc-loom-staging\nschema: 9\nsemantics: 1\nidentity-mode: exact\n",
            b"dorc-loom-staging\nschema: 1\nsemantics: 9\nidentity-mode: exact\n",
            b"dorc-loom-staging\nschema: 1\nsemantics: 1\nidentity-mode: loose\n",
        ] {
            let mut changed = prefix.to_vec();
            changed.extend_from_slice(b"catalog 0\n\n");
            assert!(parse(&changed).is_err());
        }
        for suffix in [b"wat 0\n\n".as_slice(), b"case 0\n\n", b"catalog nope\n"] {
            let mut changed = packet.clone();
            changed.extend_from_slice(suffix);
            assert!(parse(&changed).is_err());
        }
        for end in packet
            .iter()
            .enumerate()
            .filter_map(|(index, byte)| (*byte == b'\n').then_some(index + 1))
            .filter(|end| *end < packet.len())
        {
            assert!(
                parse(&packet[..end]).is_err(),
                "accepted truncation at {end}"
            );
        }
        let mut bad_header_utf8 = packet.clone();
        bad_header_utf8[0] = 0xff;
        assert!(parse(&bad_header_utf8).is_err());
        let mut bad_value_utf8 = packet.clone();
        let value = bad_value_utf8
            .windows(b"catalog 7\n".len())
            .position(|window| window == b"catalog 7\n")
            .expect("catalog field")
            + b"catalog 7\n".len();
        bad_value_utf8[value] = 0xff;
        assert!(parse(&bad_value_utf8).is_err());
    }

    #[test]
    fn typed_model_rejects_impossible_nesting_and_identity() {
        let mut invalid = inspection("x");
        let (render, _) = editable_species(&mut invalid);
        let StagedComponent::EditableSection { id, .. } = &mut render.components[2] else {
            panic!("editable section");
        };
        id.owner.clear();
        assert!(encode(&invalid).is_err());

        let mut invalid = inspection("x");
        let (_, sections) = editable_species(&mut invalid);
        sections.push(StagedSection {
            id: StagedSectionId {
                owner: "other".to_owned(),
                field: "message".to_owned(),
                instance: 0,
                segment: 0,
            },
            fragments: Vec::new(),
            bindings: Vec::new(),
            concrete: String::new(),
        });
        assert!(encode(&invalid).is_err());

        let mut invalid = inspection("x");
        let (render, _) = editable_species(&mut invalid);
        let StagedComponent::EditableSection { fragments, .. } = &mut render.components[2] else {
            panic!("editable section");
        };
        fragments.push(StagedFragment::Variable {
            id: StagedVariableId {
                name: String::new(),
                occurrence: 0,
            },
            rendered: String::new(),
        });
        assert!(encode(&invalid).is_err());

        let mut invalid = inspection("x");
        let (_, sections) = editable_species(&mut invalid);
        sections.push(StagedSection {
            id: StagedSectionId {
                owner: "code".to_owned(),
                field: "message".to_owned(),
                instance: 0,
                segment: 0,
            },
            fragments: Vec::new(),
            bindings: vec![
                ("name".to_owned(), "one".to_owned()),
                ("name".to_owned(), "two".to_owned()),
            ],
            concrete: String::new(),
        });
        assert!(encode(&invalid).is_err());
    }
}
