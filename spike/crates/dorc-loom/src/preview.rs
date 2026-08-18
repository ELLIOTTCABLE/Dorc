//! Pure compilation preview (`282:rul-compile-before-promote`).

use errorloom::{EditableFragment, RenderComponent, VariableDrop};

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
    pub(crate) baked: Vec<TemplateVariableName>,
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

    /// The dropped variables whose rendered value is STILL THERE, as literal text.
    ///
    /// The evidenced half of [`Self::dropped`], and the only half that can be warned about honestly
    /// (`30C` item 2): a variable removed with its value gone is an ordinary removal, but a variable
    /// removed while its exact rendered bytes sit in the section's new literal text is a world the
    /// author probably froze by accident. Never a refusal — the author may genuinely mean it, and
    /// nothing here can tell.
    ///
    /// The evidence is the transport's own, not a second reading of the compiled bytes: errorloom
    /// reports a NEW occurrence, counted against what the baseline section's literal text already
    /// carried. That distinction is load-bearing rather than fussy — prose that spells a value out
    /// beside its own variable is ordinary, and a `contains` test would call every genuine deletion
    /// of such a variable a frozen world.
    #[must_use]
    pub fn baked(&self) -> &[TemplateVariableName] {
        &self.baked
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
            let baked = baked_variables(&dropped, edit.drops());
            SectionPreview {
                section: edit.section().clone(),
                compiled,
                used_bindings,
                dropped: dropped.into_iter().map(|(name, _)| name).collect(),
                baked,
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
) -> Vec<(TemplateVariableName, String)> {
    let mut dropped: Vec<(TemplateVariableName, String)> = Vec::new();
    for component in baseline.render().components() {
        let RenderComponent::EditableSection(stamped) = component else {
            continue;
        };
        if stamped.id() != section {
            continue;
        }
        for fragment in stamped.fragments() {
            let EditableFragment::Variable { id, rendered } = fragment else {
                continue;
            };
            let name = &id.name;
            if !used.iter().any(|(kept, _)| kept == name)
                && !dropped.iter().any(|(gone, _)| gone == name)
            {
                dropped.push((name.clone(), rendered.clone()));
            }
        }
    }
    dropped
}

/// The dropped variables whose rendered value NEWLY appears in the edit's own literal text.
///
/// Both halves are needed and neither implies the other. The transport's reappearance fact is
/// per-OCCURRENCE, so a section carrying `{{name}}` twice can lose one and keep the other, and
/// the register still interpolates it — while [`dropped_variables`] is per-NAME and answers the
/// question the warning is actually about: is this variable gone from the register.
fn baked_variables(
    dropped: &[(TemplateVariableName, String)],
    drops: &[VariableDrop<SectionVariableId>],
) -> Vec<TemplateVariableName> {
    dropped
        .iter()
        .filter(|(name, _)| {
            drops
                .iter()
                .any(|drop| drop.id().name == *name && drop.value_reappears_as_text())
        })
        .map(|(name, _)| name.clone())
        .collect()
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
