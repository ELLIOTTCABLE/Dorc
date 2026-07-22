//! Pure compilation preview (`282:rul-compile-before-promote`).

use errorloom::{EditableFragment, RenderComponent};

use crate::{
    CompiledFragment, CompiledSection, DorcEditableBaseline, DorcSectionEditRefusal, SectionKey,
    SectionVariableId, TemplateVariableName, compile_section_edits,
};

/// One interpreted editable section.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SectionPreview {
    section: SectionKey,
    compiled: CompiledSection,
    used_bindings: Vec<(TemplateVariableName, String)>,
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
}

/// The complete in-memory result of compiling dirty transcript sections.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompilePreview {
    sections: Vec<SectionPreview>,
    concrete: String,
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
            let used_bindings = compiled
                .used()
                .iter()
                .map(|name| (name.clone(), compiled.bindings()[name].clone()))
                .collect();
            SectionPreview {
                section: edit.section().clone(),
                compiled,
                used_bindings,
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
