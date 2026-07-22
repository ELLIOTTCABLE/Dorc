//! Internal compile-preview inspection (`282:rul-compile-before-promote`).

use crate::{CompilePreview, CompiledFragment, SectionPreview};

/// Render a deterministic, deliberately blunt compilation inspection.
#[must_use]
pub fn render_compile_preview(preview: &CompilePreview) -> String {
    let sections = preview
        .sections()
        .iter()
        .map(section_text)
        .collect::<Vec<_>>()
        .join("\n");
    format!("{sections}\nconcrete:\n{}", preview.concrete())
}

fn section_text(preview: &SectionPreview) -> String {
    let fragments = preview
        .fragments()
        .iter()
        .map(fragment_text)
        .collect::<Vec<_>>()
        .join(" | ");
    let bindings = preview
        .used_bindings()
        .iter()
        .map(|(name, value)| format!("{{{{{}}}}} = {value:?}", name.0))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "section: {}.{}#{}:{}\ninterpreted: {fragments}\nbindings:\n{bindings}",
        preview.section().code,
        preview.section().field,
        preview.section().instance,
        preview.section().segment
    )
}

fn fragment_text(fragment: &CompiledFragment) -> String {
    match fragment {
        CompiledFragment::Text(text) => format!("Text({text:?})"),
        CompiledFragment::Variable(name) => format!("Variable({{{{{}}}}})", name.0),
    }
}
