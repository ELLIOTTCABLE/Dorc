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
    let section = preview.section();
    let mut lines = vec![format!(
        "section: {}.{}#{}:{}",
        section.owner, section.field, section.instance, section.segment
    )];
    let series = series_lines(preview.fragments());
    if series.iter().any(|(_, holed)| *holed) {
        let mut elided = false;
        for (text, holed) in &series {
            if *holed {
                lines.push(format!("  {}", visible(text)));
                elided = false;
            } else if !elided {
                lines.push(String::from("  ..."));
                elided = true;
            }
        }
        lines.extend(
            preview
                .used_bindings()
                .iter()
                .map(|(name, value)| format!("  {{{{{}}}}} = {}", name.0, visible(value))),
        );
    } else {
        // Whole-page arrangement sections are routinely hole-free, and dumping one costs sixty
        // lines to say nothing this view is for. The word-diff carries the prose.
        lines.push(String::from("  no variables"));
    }
    // The one destructive edit that LOOKS like an ordinary reword: typing a value's text where its
    // marker stood is legal (omission is how a variable is removed) and silent, so it is disclosed
    // here rather than refused.
    if !preview.dropped().is_empty() {
        let names: Vec<String> = preview
            .dropped()
            .iter()
            .map(|name| format!("{{{{{}}}}}", name.0))
            .collect();
        lines.push(format!(
            "  DROPPED VARIABLES: {} — the value now appears only as literal text, frozen at what \
             this render happened to say. Re-type it as {} to keep it interpolated; leave it out \
             only if you meant to remove it.",
            names.join(", "),
            names.join(", "),
        ));
    }
    lines.join("\n")
}

/// The compiled series as display lines, each flagged with whether a `{{hole}}` landed in it.
///
/// Keyed on the FRAGMENT, never on the rendered text: searching the output for `{{` would also
/// match an author who wrote braces, and this view's whole job is to be trusted about holes.
fn series_lines(fragments: &[CompiledFragment]) -> Vec<(String, bool)> {
    let mut lines = vec![(String::new(), false)];
    for fragment in fragments {
        match fragment {
            CompiledFragment::Text(text) => {
                for (index, piece) in text.split('\n').enumerate() {
                    if index > 0 {
                        lines.push((String::new(), false));
                    }
                    if let Some(line) = lines.last_mut() {
                        line.0.push_str(piece);
                    }
                }
            }
            CompiledFragment::Variable(name) => {
                if let Some(line) = lines.last_mut() {
                    line.0.push_str("{{");
                    line.0.push_str(&name.0);
                    line.0.push_str("}}");
                    line.1 = true;
                }
            }
        }
    }
    lines
}

/// `{:?}` escapes quotes and backslashes too, and these transcripts are full of both (`"$@"`,
/// `%LOCALAPPDATA%\dorc`) — mangling them is what made the old render unreadable.
fn visible(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            character => character.to_string(),
        })
        .collect()
}
