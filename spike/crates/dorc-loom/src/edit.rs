//! Dorc policy over generic editable-section transport (`282` §5 · §13).

use std::collections::BTreeSet;

use dorc_core::catalog::{TemplatePart, TemplateRefusal, parse_template};
use errorloom::{
    EditRefusal, EditRefusalClass, EditTransport, EditableFragment, EditableRender,
    EditableSection, RenderComponent, transport_edit_allow_removal,
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
                match compile_fragments(edit.fragments(), section_values(baseline, section.id())) {
                    Ok(compiled) => successful.push(DorcSectionEdit {
                        section: section.id().clone(),
                        compiled,
                    }),
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

fn compile_transport(
    baseline: &DorcEditableBaseline,
    transport: Result<EditTransport<SectionKey, SectionVariableId>, EditRefusal>,
) -> Result<DorcSectionEdit, DorcSectionEditRefusal> {
    let EditTransport::Edited(edit) = transport.map_err(DorcSectionEditRefusal::Transport)? else {
        return Err(DorcSectionEditRefusal::Unchanged);
    };
    let compiled = compile_fragments(edit.fragments(), section_values(baseline, edit.section()))
        .map_err(DorcSectionEditRefusal::Compile)?;
    Ok(DorcSectionEdit {
        section: edit.section().clone(),
        compiled,
    })
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
    let values = section_values(baseline, section.id());
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

fn section_values<'a>(
    baseline: &'a DorcEditableBaseline,
    section: &SectionKey,
) -> &'a std::collections::BTreeMap<TemplateVariableName, String> {
    baseline.variables().get(section).unwrap_or(&EMPTY_VALUES)
}

static EMPTY_VALUES: std::collections::BTreeMap<TemplateVariableName, String> =
    std::collections::BTreeMap::new();

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
