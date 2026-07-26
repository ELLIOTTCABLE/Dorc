//! Honest first-divergence reporting between a committed transcript and a fresh one.
//!
//! A transcript check that fails owes its reader one thing: WHERE. The obvious
//! implementation — zip the two line lists and report the first index that differs —
//! lies twice. When the line counts change it pairs unrelated lines from that point
//! on, so a one-line insertion reads as an every-line rewrite. And when the only
//! difference is bytes that `lines()` discards (a trailing newline, a stray `\r`) it
//! finds no differing index at all and reports the first line, showing two lines that
//! look identical.
//!
//! Both are fixed the same way: split so that no byte is discarded
//! ([`str::split_inclusive`], so a final line without its newline is a DIFFERENT line
//! from one with it), and align the two sequences by longest common subsequence
//! instead of by position.

use std::fmt::Write as _;

/// Lines of aligned context printed on each side of the divergence.
const CONTEXT_LINES: usize = 3;

/// Maximum `-`/`+` lines emitted before the report truncates. A whole-transcript
/// rewrite has one useful fact — where it starts — and printing the rest restores the
/// haystack.
const MAX_REPORTED_LINES: usize = 24;

/// Maximum characters shown of any one line.
const MAX_LINE_CHARS: usize = 200;

/// Alignment table cells beyond which the report skips the alignment and describes
/// the divergence positionally. Transcripts are bounded (see
/// [`MAX_REPLAY_OUTPUT_BYTES`](crate::MAX_REPLAY_OUTPUT_BYTES)) so this is a floor
/// under pathology, not a working limit.
const MAX_ALIGNMENT_CELLS: usize = 4_000_000;

/// Describe the first place `got` diverges from `want`, or `None` when the two are
/// byte-identical.
///
/// The report is a multi-line block of `=`/`-`/`+` rows: `-` is committed, `+` is
/// fresh, `=` is aligned context. Every line is escaped, so a trailing-newline or
/// carriage-return difference is visible rather than invisible.
///
/// # Examples
/// ```
/// use errorloom::describe_divergence;
///
/// assert_eq!(describe_divergence("same\n", "same\n"), None);
/// let report = describe_divergence("done\n", "done").expect("differs");
/// assert!(report.contains("no trailing newline"), "{report}");
/// ```
#[must_use]
pub fn describe_divergence(want: &str, got: &str) -> Option<String> {
    if want == got {
        return None;
    }
    let left: Vec<&str> = want.split_inclusive('\n').collect();
    let right: Vec<&str> = got.split_inclusive('\n').collect();

    let mut out = format!(
        "{} committed lines, {} fresh{}\n",
        left.len(),
        right.len(),
        tail_note(want, got)
    );
    for row in aligned_rows(&left, &right) {
        let _ = writeln!(out, "{row}");
    }
    Some(out.trim_end().to_owned())
}

/// The one difference a line-oriented report cannot show by itself: whether the text
/// ends with a newline. Named explicitly, because two rendered lines that differ only
/// there are visually identical.
fn tail_note(want: &str, got: &str) -> &'static str {
    match (want.ends_with('\n'), got.ends_with('\n')) {
        (true, false) => " — the fresh text has no trailing newline; the committed one does",
        (false, true) => " — the fresh text has a trailing newline; the committed one does not",
        _ => "",
    }
}

/// The `=`/`-`/`+` rows around the first divergence.
fn aligned_rows(left: &[&str], right: &[&str]) -> Vec<String> {
    let script = align(left, right);
    let Some(first) = script
        .iter()
        .position(|step| !matches!(step, Step::Same(_)))
    else {
        return vec![String::from(
            "  (every line aligns; the difference is in the trailing bytes above)",
        )];
    };
    let mut rows = Vec::new();
    let mut emitted = 0usize;
    let mut trailing_context = 0usize;
    for step in script.iter().skip(first.saturating_sub(CONTEXT_LINES)) {
        match step {
            Step::Same(line) => {
                if emitted == 0 {
                    rows.push(format!("  = {}", show(line)));
                    continue;
                }
                trailing_context = trailing_context.saturating_add(1);
                if trailing_context > CONTEXT_LINES {
                    break;
                }
                rows.push(format!("  = {}", show(line)));
            }
            Step::Removed(line) | Step::Added(line) => {
                if emitted >= MAX_REPORTED_LINES {
                    rows.push(String::from("  … (further differences elided)"));
                    break;
                }
                let sign = if matches!(step, Step::Removed(_)) {
                    '-'
                } else {
                    '+'
                };
                rows.push(format!("  {sign} {}", show(line)));
                emitted = emitted.saturating_add(1);
                trailing_context = 0;
            }
        }
    }
    rows
}

fn show(line: &str) -> String {
    let clipped: String = line.chars().take(MAX_LINE_CHARS).collect();
    if clipped.chars().count() < line.chars().count() {
        format!("{clipped:?}…")
    } else {
        format!("{clipped:?}")
    }
}

enum Step<'a> {
    Same(&'a str),
    Removed(&'a str),
    Added(&'a str),
}

/// Longest-common-subsequence alignment. Over the bounded transcripts this crate
/// admits, the quadratic table is a few million cells at absolute worst and typically
/// a few thousand; past the ceiling the caller still gets a positional description
/// rather than nothing.
fn align<'a>(left: &[&'a str], right: &[&'a str]) -> Vec<Step<'a>> {
    let cells = left
        .len()
        .saturating_add(1)
        .saturating_mul(right.len().saturating_add(1));
    if cells > MAX_ALIGNMENT_CELLS {
        return positional(left, right);
    }
    let width = right.len().saturating_add(1);
    let mut table = vec![0usize; cells];
    for row in (0..left.len()).rev() {
        for column in (0..right.len()).rev() {
            let index = row.saturating_mul(width).saturating_add(column);
            let value = if left.get(row) == right.get(column) {
                table
                    .get(index.saturating_add(width).saturating_add(1))
                    .copied()
                    .unwrap_or(0)
                    .saturating_add(1)
            } else {
                let down = table.get(index.saturating_add(width)).copied().unwrap_or(0);
                let across = table.get(index.saturating_add(1)).copied().unwrap_or(0);
                down.max(across)
            };
            if let Some(cell) = table.get_mut(index) {
                *cell = value;
            }
        }
    }

    let mut script = Vec::new();
    let (mut row, mut column) = (0usize, 0usize);
    while row < left.len() && column < right.len() {
        let index = row.saturating_mul(width).saturating_add(column);
        if left.get(row) == right.get(column) {
            script.push(Step::Same(left.get(row).copied().unwrap_or_default()));
            row = row.saturating_add(1);
            column = column.saturating_add(1);
            continue;
        }
        let down = table.get(index.saturating_add(width)).copied().unwrap_or(0);
        let across = table.get(index.saturating_add(1)).copied().unwrap_or(0);
        if down >= across {
            script.push(Step::Removed(left.get(row).copied().unwrap_or_default()));
            row = row.saturating_add(1);
        } else {
            script.push(Step::Added(right.get(column).copied().unwrap_or_default()));
            column = column.saturating_add(1);
        }
    }
    script.extend(left.iter().skip(row).map(|line| Step::Removed(line)));
    script.extend(right.iter().skip(column).map(|line| Step::Added(line)));
    script
}

/// The over-ceiling fallback: pair by position, which is exactly the behaviour the
/// alignment exists to replace — acceptable only because it is announced.
fn positional<'a>(left: &[&'a str], right: &[&'a str]) -> Vec<Step<'a>> {
    let mut script = Vec::new();
    for index in 0..left.len().max(right.len()) {
        match (left.get(index), right.get(index)) {
            (Some(l), Some(r)) if l == r => script.push(Step::Same(l)),
            (l, r) => {
                script.extend(l.copied().map(Step::Removed));
                script.extend(r.copied().map(Step::Added));
            }
        }
    }
    script
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;

    use super::describe_divergence;

    /// The trap this module exists for: `lines()` discards the trailing newline, so the old
    /// reporter found no differing line and announced "first divergence at line 1" beneath two
    /// lines that rendered identically.
    #[test]
    fn a_trailing_newline_difference_is_named_not_mislocated() {
        let report = describe_divergence("a\nb\n", "a\nb").expect("bytes differ");
        assert!(report.contains("no trailing newline"), "{report}");
        assert!(report.contains("\"b\\n\""), "{report}");
        assert!(report.contains("\"b\""), "{report}");
    }

    /// The second lie: an inserted line shifted every later line by one, and positional pairing
    /// reported all of them as changed. Under alignment exactly the insertion is reported.
    #[test]
    fn an_insertion_reports_only_the_inserted_line() {
        let want = "one\ntwo\nthree\nfour\n";
        let got = "one\ntwo\nNEW\nthree\nfour\n";
        let report = describe_divergence(want, got).expect("bytes differ");
        assert_eq!(
            report.matches("\n  + ").count(),
            1,
            "exactly one added line: {report}"
        );
        assert_eq!(report.matches("\n  - ").count(), 0, "{report}");
        assert!(report.contains("NEW"), "{report}");
    }

    #[test]
    fn identical_text_has_nothing_to_report() {
        assert_eq!(describe_divergence("x\ny\n", "x\ny\n"), None);
    }

    /// A wholesale rewrite must stay bounded — the useful fact is where it starts.
    #[test]
    fn a_wholesale_rewrite_is_bounded() {
        let mut want = String::new();
        let mut got = String::new();
        for n in 0..500 {
            let _ = writeln!(want, "old {n}");
            let _ = writeln!(got, "new {n}");
        }
        let report = describe_divergence(&want, &got).expect("bytes differ");
        assert!(report.contains("elided"), "{report}");
        assert!(report.lines().count() < 40, "{report}");
    }
}
