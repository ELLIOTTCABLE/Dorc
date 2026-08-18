//! The publish diff: what a publication does to the registers, in `{{hole}}` spelling.

use crate::{CompilePreview, SectionPreview};

/// Rows of `-`/`+` emitted for one section before the hunk truncates. A wholesale rewrite has one
/// useful fact — that it is wholesale — and printing the rest restores the haystack.
const MAX_HUNK_LINES: usize = 40;

/// Render what publishing this interpretation changes, one hunk per touched register.
///
/// This is the one question git cannot answer afterwards. The committed transcript renders concrete
/// values by design, so a case-file diff is byte-identical whether a hole MOVED or DIED — both
/// spell `apt-get` where `{{command}}` used to stand. Only the template spelling separates them,
/// which makes this the author's sole surface for seeing it, and why it renders the same way
/// whatever the volume and whichever path the publish then takes.
///
/// A section whose register is unchanged prints nothing: the transcript moved (whitespace a chrome
/// line's renderer owns, say) but the stored words did not, and there is no change to review.
#[must_use]
pub fn render_publish_diff(preview: &CompilePreview) -> String {
    preview
        .sections()
        .iter()
        .filter_map(section_hunk)
        .collect::<Vec<_>>()
        .join("\n")
}

fn section_hunk(preview: &SectionPreview) -> Option<String> {
    let (before, after) = (preview.stamped_template(), preview.compiled_template());
    if before == after {
        return None;
    }
    let section = preview.section();
    let mut lines = vec![format!(
        "section: {}.{}#{}:{}",
        section.owner, section.field, section.instance, section.segment
    )];
    let (before, after) = changed_lines(before, &after);
    lines.extend(sided('-', &before));
    lines.extend(sided('+', &after));
    Some(lines.join("\n"))
}

/// The two sides with their identical leading and trailing LINES trimmed away.
///
/// Not a general alignment: a prose edit is one contiguous rewrite, so trimming what both sides
/// still share leaves exactly the run that moved — and on a sixty-line help page that is the
/// difference between three rows and a hundred and twenty.
fn changed_lines<'a>(before: &'a str, after: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let before: Vec<&str> = before.split('\n').collect();
    let after: Vec<&str> = after.split('\n').collect();
    let head = before
        .iter()
        .zip(&after)
        .take_while(|(left, right)| left == right)
        .count();
    let shortest = before.len().min(after.len()).saturating_sub(head);
    let tail = before
        .iter()
        .rev()
        .zip(after.iter().rev())
        .take(shortest)
        .take_while(|(left, right)| left == right)
        .count();
    let hunk = |lines: Vec<&'a str>| {
        let end = lines.len().saturating_sub(tail);
        lines.get(head..end).unwrap_or_default().to_vec()
    };
    (hunk(before), hunk(after))
}

fn sided(sign: char, lines: &[&str]) -> Vec<String> {
    let mut rows: Vec<String> = lines
        .iter()
        .take(MAX_HUNK_LINES)
        .map(|line| format!("  {sign} {}", visible(line)))
        .collect();
    if let Some(elided) = lines.len().checked_sub(MAX_HUNK_LINES).filter(|n| *n > 0) {
        rows.push(format!("  {sign} ... ({elided} further lines elided)"));
    }
    rows
}

/// `{:?}` escapes quotes and backslashes too, and these registers are full of both (`"$@"`,
/// `%LOCALAPPDATA%\dorc`) — mangling them is what made an earlier render unreadable.
fn visible(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            '\r' => "\\r".to_owned(),
            '\t' => "\\t".to_owned(),
            character => character.to_string(),
        })
        .collect()
}
