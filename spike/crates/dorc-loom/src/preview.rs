//! Pure compilation preview (`282:rul-compile-before-promote`).

use errorloom::{EditableFragment, RenderComponent};

use crate::{
    CompiledFragment, CompiledSection, DorcEditableBaseline, DorcSectionEditRefusal, SectionKey,
    SectionVariableId, TemplateVariableName, compile_section_edit,
};

/// The complete in-memory result of compiling one dirty transcript edit.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompilePreview {
    section: SectionKey,
    compiled: CompiledSection,
    used_bindings: Vec<(TemplateVariableName, String)>,
    concrete: String,
}

impl CompilePreview {
    /// The editable section selected by compilation.
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

    /// The full concrete transcript with the selected section substituted.
    #[must_use]
    pub fn concrete(&self) -> &str {
        &self.concrete
    }
}

/// Compile a dirty transcript and reconstruct its full concrete render.
///
/// The preview derives its inventory solely from the compiled section, and copies all
/// nonselected baseline components byte-for-byte (`282:rul-compile-before-promote`).
///
/// # Errors
/// Returns the edit-attribution or section-compilation refusal.
pub fn compile_preview(
    baseline: &DorcEditableBaseline,
    dirty: &str,
) -> Result<CompilePreview, DorcSectionEditRefusal> {
    let edit = compile_section_edit(baseline, dirty)?;
    let section = edit.section().clone();
    let compiled = edit.compiled().clone();
    let used_bindings = compiled
        .used()
        .iter()
        .map(|name| (name.clone(), compiled.bindings()[name].clone()))
        .collect();
    let concrete = baseline
        .render()
        .components()
        .iter()
        .map(|component| component_text(component, &section, &compiled))
        .collect();
    Ok(CompilePreview {
        section,
        compiled,
        used_bindings,
        concrete,
    })
}

fn component_text(
    component: &RenderComponent<SectionKey, SectionVariableId>,
    selected: &SectionKey,
    compiled: &CompiledSection,
) -> String {
    match component {
        RenderComponent::Structure(text)
        | RenderComponent::FixedVariable { rendered: text, .. } => text.clone(),
        RenderComponent::EditableSection(section) if section.id() == selected => compiled.text(),
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
