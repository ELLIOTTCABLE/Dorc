//! Internal compile-preview inspection (`282:rul-compile-before-promote`).

use crate::{CompilePreview, CompiledFragment};

/// Render a deterministic, deliberately blunt compilation inspection.
#[must_use]
pub fn render_compile_preview(preview: &CompilePreview) -> String {
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
        "interpreted: {fragments}\nbindings:\n{bindings}\nconcrete:\n{}",
        preview.concrete()
    )
}

fn fragment_text(fragment: &CompiledFragment) -> String {
    match fragment {
        CompiledFragment::Text(text) => format!("Text({text:?})"),
        CompiledFragment::Variable(name) => format!("Variable({{{{{}}}}})", name.0),
    }
}
