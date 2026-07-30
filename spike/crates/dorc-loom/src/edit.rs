//! Dorc policy over generic editable-section transport (`282` §5 · §13).

use std::collections::BTreeSet;

use dorc_aid::catalog::{TemplatePart, TemplateRefusal, parse_template};
use errorloom::{
    EditRefusal, EditRefusalClass, EditTransport, EditableFragment, EditableRender,
    EditableSection, RenderComponent, SectionAddressRefusal, address_sections,
    transport_edit_allow_removal,
};

use crate::{
    CompileRefusal, CompiledSection, DorcEditableBaseline, SectionKey, SectionVariableId,
    TemplateVariableName, compile_fragments,
};

/// One Dorc-local edit interpretation.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DorcSectionEdit {
    section: SectionKey,
    compiled: CompiledSection,
}

impl DorcSectionEdit {
    /// The sole editable section selected by the render-stamped transport.
    #[must_use]
    pub fn section(&self) -> &SectionKey {
        &self.section
    }

    /// The compiled replacement for that section.
    #[must_use]
    pub fn compiled(&self) -> &CompiledSection {
        &self.compiled
    }
}

/// Why a dirty Dorc transcript cannot compile to one editable section.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DorcSectionEditRefusal {
    /// No transcript bytes changed.
    Unchanged,
    /// Generic transport could not attribute the edit safely.
    Transport(EditRefusal),
    /// A dirty double-brace marker is malformed.
    Template(TemplateRefusal),
    /// A dirty marker names no variable in its exact section.
    UnknownVariable(TemplateVariableName),
    /// A marker is not a standalone token, or compilation otherwise refused it.
    Compile(CompileRefusal),
    /// More than one immutable-boundary candidate compiled successfully.
    AmbiguousCandidate,
    /// A marker occurs outside the interior of an editable section.
    MarkerOutsideEditableSection,
    /// The generic transport selected a section other than the marker candidate.
    CandidateMismatch,
    /// The selected field is split around immutable render components.
    SplitEditableField(SectionKey),
    /// The edit laid out MORE lines than the render did.
    ///
    /// A catalog register and a chrome line hold WORDS; where those words BREAK is the renderer's
    /// (`282` §3, `28H` ruling 7). So a transcript line the render never emitted is not a longer
    /// sentence — it is a request for a register that does not exist yet, and absorbing it into the
    /// neighbouring one silently rewrote that register with somebody else's line
    /// (`_loom-final-map:fnd-added-help-is-silently-absorbed`). A whole-PAGE entry is exempt: its
    /// blank lines ARE the author's.
    AddedLine {
        /// The section the added line landed in.
        section: SectionKey,
        /// Line breaks the render laid out inside that section.
        laid_out: usize,
        /// Line breaks the compiled edit carries.
        edited: usize,
    },
}

/// Compile one dirty transcript edit through the generic transport.
///
/// Ordinary edits retain generic variable identities. Explicit `{{name}}` markers are
/// interpreted only within a single exact-boundary editable section (`282` §13), so a
/// marker can move, duplicate, or remove that section's existing named variables.
///
/// # Errors
/// Returns a structured refusal when attribution, marker parsing, or compilation fails.
pub fn compile_section_edit(
    baseline: &DorcEditableBaseline,
    dirty: &str,
) -> Result<DorcSectionEdit, DorcSectionEditRefusal> {
    if !contains_marker_sigil(dirty) {
        return compile_transport(
            baseline,
            transport_edit_allow_removal(baseline.render(), dirty),
        );
    }

    let mut successful = Vec::new();
    let mut refusals = Vec::new();
    let mut saw_ambiguity = false;
    let mut limit = None;

    let candidates = marker_candidates(baseline.render(), dirty);
    if candidates.is_empty() {
        return Err(DorcSectionEditRefusal::MarkerOutsideEditableSection);
    }
    for (index, section) in candidates {
        let transformed = match transform_marked_section(baseline, index, section, dirty) {
            Ok(render) => render,
            Err(refusal) => {
                refusals.push(refusal);
                continue;
            }
        };
        match transport_edit_allow_removal(&transformed, dirty) {
            Ok(EditTransport::Edited(edit)) if edit.section() == section.id() => {
                let values = available_values(baseline, section.id());
                let fragments = normalize_register_prose(section.id().field, edit.fragments());
                match compile_fragments(&fragments, &values) {
                    Ok(compiled) => {
                        refuse_split_field(baseline.render(), section.id())?;
                        refuse_added_lines(baseline.render(), section.id(), &compiled)?;
                        successful.push(DorcSectionEdit {
                            section: section.id().clone(),
                            compiled,
                        });
                    }
                    Err(refusal) => refusals.push(DorcSectionEditRefusal::Compile(refusal)),
                }
            }
            Ok(EditTransport::Edited(_)) => {
                refusals.push(DorcSectionEditRefusal::CandidateMismatch);
            }
            Ok(EditTransport::Unchanged) => refusals.push(DorcSectionEditRefusal::Unchanged),
            Err(refusal) if refusal.class() == EditRefusalClass::AlignmentLimitExceeded => {
                limit = Some(refusal);
            }
            Err(refusal) if refusal.class() == EditRefusalClass::AmbiguousAttribution => {
                saw_ambiguity = true;
            }
            Err(refusal) => refusals.push(DorcSectionEditRefusal::Transport(refusal)),
        }
    }

    if let Some(refusal) = limit {
        return Err(DorcSectionEditRefusal::Transport(refusal));
    }
    if saw_ambiguity || successful.len() > 1 {
        return Err(DorcSectionEditRefusal::AmbiguousCandidate);
    }
    if let Some(edit) = successful.pop() {
        return Ok(edit);
    }
    Err(refusals
        .into_iter()
        .next()
        .unwrap_or(DorcSectionEditRefusal::CandidateMismatch))
}

/// Compile every changed renderer-stamped section in one dirty render.
///
/// # Errors
/// Returns a refusal when immutable renderer output does not delimit one exact section edit.
pub fn compile_section_edits(
    baseline: &DorcEditableBaseline,
    dirty: &str,
) -> Result<Vec<DorcSectionEdit>, DorcSectionEditRefusal> {
    let ranges = address_sections(baseline.render(), dirty).map_err(|refusal| match refusal {
        SectionAddressRefusal::Ambiguous => DorcSectionEditRefusal::AmbiguousCandidate,
        _ => DorcSectionEditRefusal::MarkerOutsideEditableSection,
    })?;
    let sections = baseline
        .render()
        .components()
        .iter()
        .filter_map(|component| match component {
            RenderComponent::EditableSection(section) => Some(section),
            _ => None,
        });
    let mut edits = Vec::new();
    for (section, range) in sections.zip(ranges) {
        let Some(interior) = dirty.get(range) else {
            return Err(DorcSectionEditRefusal::MarkerOutsideEditableSection);
        };
        if interior == section_text(section) {
            continue;
        }
        let section_baseline = baseline
            .section_baseline(section.id())
            .ok_or(DorcSectionEditRefusal::CandidateMismatch)?;
        edits.push(compile_section_edit(&section_baseline, interior)?);
    }
    if edits.is_empty() {
        return Err(DorcSectionEditRefusal::Unchanged);
    }
    Ok(edits)
}

fn compile_transport(
    baseline: &DorcEditableBaseline,
    transport: Result<EditTransport<SectionKey, SectionVariableId>, EditRefusal>,
) -> Result<DorcSectionEdit, DorcSectionEditRefusal> {
    let EditTransport::Edited(edit) = transport.map_err(DorcSectionEditRefusal::Transport)? else {
        return Err(DorcSectionEditRefusal::Unchanged);
    };
    let values = available_values(baseline, edit.section());
    let fragments = normalize_register_prose(edit.section().field, edit.fragments());
    let compiled =
        compile_fragments(&fragments, &values).map_err(DorcSectionEditRefusal::Compile)?;
    refuse_split_field(baseline.render(), edit.section())?;
    refuse_added_lines(baseline.render(), edit.section(), &compiled)?;
    Ok(DorcSectionEdit {
        section: edit.section().clone(),
        compiled,
    })
}

/// Refuse an edit that carries more line breaks than the render laid out in that section
/// (see [`DorcSectionEditRefusal::AddedLine`]).
///
/// Both counts come from PROSE only — a value's own bytes are the render's account of the world and
/// belong to neither side of the arithmetic — so the comparison reads the render's stamped
/// fragments, never the shape of a rendered line
/// (`28L:rul-editability-is-stamped-never-re-derived`).
fn refuse_added_lines(
    render: &EditableRender<SectionKey, SectionVariableId>,
    selected: &SectionKey,
    compiled: &CompiledSection,
) -> Result<(), DorcSectionEditRefusal> {
    if selected.field == crate::ARRANGEMENT_FIELD {
        return Ok(());
    }
    let laid_out = render
        .components()
        .iter()
        .filter_map(|component| match component {
            RenderComponent::EditableSection(section) if section.id() == selected => Some(section),
            _ => None,
        })
        .flat_map(EditableSection::fragments)
        .filter_map(|fragment| match fragment {
            EditableFragment::Text(text) => Some(prose_line_breaks(text)),
            EditableFragment::Variable { .. } => None,
        })
        .sum();
    let edited = compiled
        .fragments()
        .iter()
        .filter_map(|fragment| match fragment {
            crate::CompiledFragment::Text(text) => Some(prose_line_breaks(text)),
            crate::CompiledFragment::Variable(_) => None,
        })
        .sum();
    if edited > laid_out {
        return Err(DorcSectionEditRefusal::AddedLine {
            section: selected.clone(),
            laid_out,
            edited,
        });
    }
    Ok(())
}

/// The ONE place this crate decides what a line break in prose is (`28H`'s
/// named-word-judgment law).
fn prose_line_breaks(text: &str) -> usize {
    text.matches('\n').count()
}

/// Read-in normalization for catalog register prose (`282` §3, routed by `28L:tc-catalog-prose-
/// is-not-normalized-at-read-in`): within a paragraph, every whitespace run — a single embedded
/// newline included — collapses to one space; a run of two-plus newlines canonicalizes to exactly
/// one paragraph break (`"\n\n"`, `282` §3: "nothing else exists"); trailing whitespace trims off
/// the section's own tail. Lands beside [`prose_line_breaks`] — the sibling judgment this crate
/// already names as its ONE seat for what a line break in prose is (`28H` standing law: no second
/// one) — and runs BEFORE `refuse_added_lines` counts the result, so a re-wrapped register can only
/// relax that check, never trip it: a genuinely added paragraph break still refuses. Idempotent —
/// re-normalizing normalized bytes is a no-op — which is what makes it a generator fixpoint under
/// the byte-identity gate.
///
/// Catalog fields only (`message`/`help`); arrangement chrome keeps its own, deliberately
/// different, always-single-space rule with no paragraph concept (`collapse_runs`, consumer.rs) —
/// a chrome LINE has no paragraphs to preserve.
fn normalize_register_prose(
    field: &'static str,
    fragments: &[EditableFragment<SectionVariableId>],
) -> Vec<EditableFragment<SectionVariableId>> {
    if !matches!(field, "message" | "help") {
        return fragments.to_vec();
    }
    // The section's own tail, never the last TEXT-typed fragment: a series that ends on a
    // Variable (an untouched value at the register's end) has no trailing prose to trim at all —
    // trimming the last Text fragment regardless of position ate the legitimate space in front of
    // it (caught by `omission_removes_variable_and_its_surrounding_backticks`).
    let trailing_index = fragments
        .len()
        .checked_sub(1)
        .filter(|&index| matches!(fragments.get(index), Some(EditableFragment::Text(_))));
    fragments
        .iter()
        .enumerate()
        .map(|(index, fragment)| match fragment {
            EditableFragment::Text(text) => {
                let collapsed = collapse_paragraph_whitespace(text);
                let text = if Some(index) == trailing_index {
                    collapsed.trim_end().to_owned()
                } else {
                    collapsed
                };
                EditableFragment::Text(text)
            }
            variable @ EditableFragment::Variable { .. } => variable.clone(),
        })
        .collect()
}

/// One whitespace run collapses to a single space, unless it carries two or more newlines — a
/// paragraph break — which canonicalizes to exactly `"\n\n"`.
fn collapse_paragraph_whitespace(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut run_newlines: Option<usize> = None;
    for character in text.chars() {
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

/// A catalog register split across several sections cannot be owned by one edit: rewriting one
/// segment would leave the rest of the register saying the old thing.
///
/// A chrome LINE is exempt, and the exemption is the point of its separate field: several
/// sections keyed to one line are several RENDERINGS of one registry entry — the same words
/// printed at two chain rows — and each is complete on its own, so an edit to either rewrites the
/// whole entry (`28H` ruling 3).
fn refuse_split_field(
    render: &EditableRender<SectionKey, SectionVariableId>,
    selected: &SectionKey,
) -> Result<(), DorcSectionEditRefusal> {
    if selected.field == crate::ARRANGEMENT_LINE_FIELD {
        return Ok(());
    }
    let segments = render
        .components()
        .iter()
        .filter_map(|component| match component {
            RenderComponent::EditableSection(section) => Some(section.id()),
            _ => None,
        })
        .filter(|section| {
            section.owner == selected.owner
                && section.field == selected.field
                && section.instance == selected.instance
        })
        .count();
    if segments > 1 {
        return Err(DorcSectionEditRefusal::SplitEditableField(selected.clone()));
    }
    Ok(())
}

fn marker_candidates<'a>(
    render: &'a EditableRender<SectionKey, SectionVariableId>,
    dirty: &str,
) -> Vec<(usize, &'a EditableSection<SectionKey, SectionVariableId>)> {
    render
        .components()
        .iter()
        .enumerate()
        .filter_map(|(index, component)| {
            let RenderComponent::EditableSection(section) = component else {
                return None;
            };
            let prefix: String = render
                .components()
                .get(..index)?
                .iter()
                .map(component_text)
                .collect();
            let suffix: String = render
                .components()
                .get(index.saturating_add(1)..)?
                .iter()
                .map(component_text)
                .collect();
            dirty
                .strip_prefix(&prefix)
                .and_then(|interior| interior.strip_suffix(&suffix))
                .map(|_| (index, section))
        })
        .collect()
}

fn transform_marked_section(
    baseline: &DorcEditableBaseline,
    index: usize,
    section: &EditableSection<SectionKey, SectionVariableId>,
    dirty: &str,
) -> Result<EditableRender<SectionKey, SectionVariableId>, DorcSectionEditRefusal> {
    let interior = section_interior(baseline.render(), index, dirty)
        .ok_or(DorcSectionEditRefusal::MarkerOutsideEditableSection)?;
    let names: BTreeSet<_> = parse_template(interior)
        .map_err(DorcSectionEditRefusal::Template)?
        .into_iter()
        .filter_map(|part| match part {
            TemplatePart::Hole(name) => Some(TemplateVariableName(name)),
            TemplatePart::Literal(_) => None,
        })
        .collect();
    let values = available_values(baseline, section.id());
    for name in &names {
        if !values.contains_key(name) {
            return Err(DorcSectionEditRefusal::UnknownVariable(name.clone()));
        }
    }
    let components = baseline
        .render()
        .components()
        .iter()
        .enumerate()
        .map(|(component_index, component)| {
            if component_index != index {
                return component.clone();
            }
            let fragments = section
                .fragments()
                .iter()
                .map(|fragment| match fragment {
                    EditableFragment::Variable { id, rendered } if names.contains(&id.name) => {
                        EditableFragment::Text(rendered.clone())
                    }
                    fragment => fragment.clone(),
                })
                .collect();
            RenderComponent::EditableSection(EditableSection::new(section.id().clone(), fragments))
        })
        .collect();
    Ok(EditableRender::new(components))
}

fn section_interior<'a>(
    render: &'a EditableRender<SectionKey, SectionVariableId>,
    index: usize,
    dirty: &'a str,
) -> Option<&'a str> {
    let prefix: String = render
        .components()
        .get(..index)?
        .iter()
        .map(component_text)
        .collect();
    let suffix: String = render
        .components()
        .get(index.saturating_add(1)..)?
        .iter()
        .map(component_text)
        .collect();
    dirty
        .strip_prefix(&prefix)
        .and_then(|interior| interior.strip_suffix(&suffix))
}

fn available_values(
    baseline: &DorcEditableBaseline,
    section: &SectionKey,
) -> std::collections::BTreeMap<TemplateVariableName, String> {
    let mut values = baseline.all_variables().clone();
    if let Some(rendered) = baseline.variables().get(section) {
        values.extend(rendered.clone());
    }
    values
}

fn section_text(section: &EditableSection<SectionKey, SectionVariableId>) -> String {
    section
        .fragments()
        .iter()
        .map(|fragment| match fragment {
            EditableFragment::Text(text) | EditableFragment::Variable { rendered: text, .. } => {
                text.as_str()
            }
        })
        .collect()
}

fn component_text(component: &RenderComponent<SectionKey, SectionVariableId>) -> String {
    match component {
        RenderComponent::Structure(text)
        | RenderComponent::FixedVariable { rendered: text, .. } => text.clone(),
        RenderComponent::EditableSection(section) => section
            .fragments()
            .iter()
            .map(|fragment| match fragment {
                EditableFragment::Text(text)
                | EditableFragment::Variable { rendered: text, .. } => text.clone(),
            })
            .collect(),
    }
}

fn contains_marker_sigil(text: &str) -> bool {
    text.contains("{{") || text.contains("}}")
}
