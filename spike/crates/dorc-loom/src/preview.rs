//! Pure compilation preview (`282:rul-compile-before-promote`).

use errorloom::{EditableFragment, RenderComponent};

use crate::{
    CompiledFragment, CompiledSection, DorcEditableBaseline, DorcSectionEditRefusal, SectionKey,
    SectionVariableId, TemplateVariableName, compile_section_edits,
};

/// One interpreted editable section.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SectionPreview {
    pub(crate) section: SectionKey,
    pub(crate) compiled: CompiledSection,
    pub(crate) used_bindings: Vec<(TemplateVariableName, String)>,
    pub(crate) dropped: Vec<TemplateVariableName>,
}

impl SectionPreview {
    /// The renderer-stamped section selected by compilation.
    #[must_use]
    pub fn section(&self) -> &SectionKey {
        &self.section
    }

    /// The interpreted text and variable fragments.
    #[must_use]
    pub fn fragments(&self) -> &[CompiledFragment] {
        self.compiled.fragments()
    }

    /// The exact bindings in compiled first-use order.
    #[must_use]
    pub fn used_bindings(&self) -> &[(TemplateVariableName, String)] {
        &self.used_bindings
    }

    /// Variables the render stamped into this section that the edit no longer interpolates.
    ///
    /// Legal — omission IS the removal mechanism (`282` §13) — and silently destructive when it
    /// was not meant: typing a value's TEXT where its marker stood bakes the current world into
    /// the register, and the register's `params` shrink to match. Disclosed rather than refused.
    #[must_use]
    pub fn dropped(&self) -> &[TemplateVariableName] {
        &self.dropped
    }
}

/// The complete in-memory result of compiling dirty transcript sections.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompilePreview {
    pub(crate) sections: Vec<SectionPreview>,
    pub(crate) concrete: String,
}

impl CompilePreview {
    /// Interpreted sections in renderer order.
    #[must_use]
    pub fn sections(&self) -> &[SectionPreview] {
        &self.sections
    }

    /// The full concrete transcript with the selected section substituted.
    #[must_use]
    pub fn concrete(&self) -> &str {
        &self.concrete
    }
}

/// Compile a dirty transcript and reconstruct its full concrete render.
///
/// The preview replaces all compiled sections and copies every other baseline component byte-for-byte.
///
/// # Errors
/// Returns the edit-attribution or section-compilation refusal.
pub fn compile_preview(
    baseline: &DorcEditableBaseline,
    dirty: &str,
) -> Result<CompilePreview, DorcSectionEditRefusal> {
    let sections: Vec<_> = compile_section_edits(baseline, dirty)?
        .into_iter()
        .map(|edit| {
            let compiled = edit.compiled().clone();
            let used_bindings: Vec<(TemplateVariableName, String)> = compiled
                .used()
                .iter()
                .map(|name| (name.clone(), compiled.bindings()[name].clone()))
                .collect();
            let dropped = dropped_variables(baseline, edit.section(), &used_bindings);
            SectionPreview {
                section: edit.section().clone(),
                compiled,
                used_bindings,
                dropped,
            }
        })
        .collect();
    let concrete = baseline
        .render()
        .components()
        .iter()
        .map(|component| component_text(component, &sections))
        .collect();
    Ok(CompilePreview { sections, concrete })
}

/// The names the RENDER stamped into `section` that the compiled edit no longer carries.
///
/// Read off the stamped fragments, never off the edited bytes: a value's text is ordinary words
/// once typed, and asking the bytes which of them used to be a variable would be exactly the
/// byte-shape re-derivation the arc outlawed (`28L:rul-editability-is-stamped-never-re-derived`).
fn dropped_variables(
    baseline: &DorcEditableBaseline,
    section: &SectionKey,
    used: &[(TemplateVariableName, String)],
) -> Vec<TemplateVariableName> {
    let mut dropped = Vec::new();
    for component in baseline.render().components() {
        let RenderComponent::EditableSection(stamped) = component else {
            continue;
        };
        if stamped.id() != section {
            continue;
        }
        for fragment in stamped.fragments() {
            let EditableFragment::Variable { id, .. } = fragment else {
                continue;
            };
            let name = &id.name;
            if !used.iter().any(|(kept, _)| kept == name) && !dropped.contains(name) {
                dropped.push(name.clone());
            }
        }
    }
    dropped
}

fn component_text(
    component: &RenderComponent<SectionKey, SectionVariableId>,
    previews: &[SectionPreview],
) -> String {
    match component {
        RenderComponent::Structure(text)
        | RenderComponent::FixedVariable { rendered: text, .. } => text.clone(),
        RenderComponent::EditableSection(section) => previews
            .iter()
            .find(|preview| preview.section == *section.id())
            .map_or_else(|| section_text(section), |preview| preview.compiled.text()),
    }
}

fn section_text(section: &errorloom::EditableSection<SectionKey, SectionVariableId>) -> String {
    section
        .fragments()
        .iter()
        .map(|fragment| match fragment {
            EditableFragment::Text(text) | EditableFragment::Variable { rendered: text, .. } => {
                text.clone()
            }
        })
        .collect()
}
