//! Pure compilation preview (`282:rul-compile-before-promote`).

use errorloom::{EditableFragment, RenderComponent, VariableDrop};

use crate::{
    CompiledFragment, CompiledSection, DorcEditableBaseline, DorcSectionEditRefusal, SectionKey,
    SectionVariableId, TemplateVariableName, compile_section_edits,
};

/// One hole the render stamped into a section that the compiled edit no longer interpolates.
///
/// Legal — omission IS the removal mechanism (`282` §13) — and silently destructive when it was not
/// meant, which is why a publish carrying one takes the confirmation path whatever the reason
/// (`30C:rul-any-hole-loss-confirms`). The two facts beside the name are the transport's own
/// evidence, never a second reading of the compiled bytes
/// (`28L:rul-editability-is-stamped-never-re-derived`); they say WHY, and the reasons are
/// independent of each other.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DroppedHole {
    /// The variable this register no longer interpolates.
    pub name: TemplateVariableName,
    /// Its rendered value NEWLY appears in the section's own literal text: the author most likely
    /// typed over the marker, freezing whatever this render happened to say.
    pub value_reappears_as_text: bool,
    /// Another occurrence in the section rendered the same bytes, so which one went is the reading
    /// the transport selected rather than something the edited bytes settle.
    pub value_shared_with_another_occurrence: bool,
}

/// One interpreted editable section.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SectionPreview {
    pub(crate) section: SectionKey,
    pub(crate) compiled: CompiledSection,
    pub(crate) used_bindings: Vec<(TemplateVariableName, String)>,
    pub(crate) dropped: Vec<DroppedHole>,
    pub(crate) stamped: String,
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

    /// The register as the render stamped it, in `{{name}}` spelling — the BEFORE of a publish diff.
    ///
    /// Read off the stamped fragment series rather than looked up in the registry: the render's
    /// literal runs and its `ParamValue` holes ARE the stored template, so the two sides of the
    /// diff come from one source and a registry lookup cannot drift out from under it.
    #[must_use]
    pub fn stamped_template(&self) -> &str {
        &self.stamped
    }

    /// The register this edit compiles to, in the same spelling — the AFTER.
    #[must_use]
    pub fn compiled_template(&self) -> String {
        self.compiled.template()
    }

    /// The holes this edit gives up, in render order.
    #[must_use]
    pub fn dropped(&self) -> &[DroppedHole] {
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
            let dropped = dropped_holes(baseline, edit.section(), &used_bindings, edit.drops());
            SectionPreview {
                section: edit.section().clone(),
                stamped: stamped_template(baseline, edit.section()),
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

/// The names the RENDER stamped into `section` that the compiled edit no longer carries, each
/// carrying the transport's evidence about why.
///
/// Read off the stamped fragments, never off the edited bytes: a value's text is ordinary words
/// once typed, and asking the bytes which of them used to be a variable would be exactly the
/// byte-shape re-derivation the arc outlawed (`28L:rul-editability-is-stamped-never-re-derived`).
///
/// The reduction from occurrences to NAMES is the load-bearing step. The transport's facts are
/// per-OCCURRENCE, so a section carrying `{{name}}` twice can lose one and keep the other and the
/// register still interpolates it; the question a publish is holding is whether the variable is
/// gone from the REGISTER, and any lost occurrence of a lost name carries its reason.
fn dropped_holes(
    baseline: &DorcEditableBaseline,
    section: &SectionKey,
    used: &[(TemplateVariableName, String)],
    drops: &[VariableDrop<SectionVariableId>],
) -> Vec<DroppedHole> {
    let mut dropped: Vec<DroppedHole> = Vec::new();
    for fragment in stamped_fragments(baseline, section) {
        let EditableFragment::Variable { id, .. } = fragment else {
            continue;
        };
        let name = &id.name;
        if used.iter().any(|(kept, _)| kept == name)
            || dropped.iter().any(|gone| gone.name == *name)
        {
            continue;
        }
        let lost = || drops.iter().filter(|drop| drop.id().name == *name);
        dropped.push(DroppedHole {
            name: name.clone(),
            value_reappears_as_text: lost().any(VariableDrop::value_reappears_as_text),
            value_shared_with_another_occurrence: lost()
                .any(VariableDrop::value_shared_with_another_occurrence),
        });
    }
    dropped
}

/// `section`'s fragments as the render stamped them.
fn stamped_fragments<'a>(
    baseline: &'a DorcEditableBaseline,
    section: &SectionKey,
) -> &'a [EditableFragment<SectionVariableId>] {
    baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            RenderComponent::EditableSection(stamped) if stamped.id() == section => {
                Some(stamped.fragments())
            }
            _ => None,
        })
        .unwrap_or_default()
}

/// The register as it is STORED, in `{{name}}` spelling.
///
/// Through the same normalization the edited side takes, which is the whole point: a render lays a
/// register out at some width, so the stamped bytes carry line breaks the ENTRY does not. Diffing
/// the two unnormalized made every wrapped register report a change it was not making, and buried
/// the hole movement this view exists for under it.
fn stamped_template(baseline: &DorcEditableBaseline, section: &SectionKey) -> String {
    crate::edit::normalize_register_prose(section.field, stamped_fragments(baseline, section))
        .iter()
        .map(|fragment| match fragment {
            EditableFragment::Text(text) => text.clone(),
            EditableFragment::Variable { id, .. } => format!("{{{{{}}}}}", id.name.0),
        })
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
