//! Internal compile-preview inspection (`282:rul-compile-before-promote`).

use crate::{CompilePreview, CompiledFragment, SectionPreview};

/// Render how each touched section was interpreted, in the template spelling.
///
/// This is the one question git cannot answer later. A post-promote diff shows the resulting
/// bytes; only this shows which of the author's words the compiler took as a `{{hole}}`, and under
/// `282`'s whole-token rule a hole in the wrong place silently changes every OTHER render that
/// binds the same variable. The concrete re-render is deliberately absent: it was the bulk of the
/// output and duplicated, worse, what `mise run loom:promote` word-diffs.
#[must_use]
pub fn render_compile_preview(preview: &CompilePreview) -> String {
    preview
        .sections()
        .iter()
        .map(section_text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn section_text(preview: &SectionPreview) -> String {
    let series: String = preview.fragments().iter().map(fragment_text).collect();
    let bindings = preview
        .used_bindings()
        .iter()
        .map(|(name, value)| format!("  {{{{{}}}}} = {}", name.0, visible(value)))
        .collect::<Vec<_>>();
    let section = preview.section();
    let mut lines = vec![
        format!(
            "section: {}.{}#{}:{}",
            section.owner, section.field, section.instance, section.segment
        ),
        format!("  {series}"),
    ];
    lines.extend(bindings);
    lines.join("\n")
}

fn fragment_text(fragment: &CompiledFragment) -> String {
    match fragment {
        CompiledFragment::Text(text) => visible(text),
        CompiledFragment::Variable(name) => format!("{{{{{}}}}}", name.0),
    }
}

/// Escape only what would break the line-per-section shape.
///
/// `{:?}` also escapes quotes and backslashes, and these transcripts are full of both (`"$@"`,
/// `%LOCALAPPDATA%\dorc`) — mangling them is what made the old render unreadable.
fn visible(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '\n' => "\\n".to_owned(),
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            character => character.to_string(),
        })
        .collect()
}
