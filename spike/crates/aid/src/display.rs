//! The one seat where bytes we did not write are made safe to show a person.
//!
//! Two destinations, two encodings, one home. A rendered why-surface is measured in bytes and
//! laid out in columns, so anything reaching it must be printable ASCII or the geometry is a lie
//! as well as the terminal being at risk; a plain advisory line printed straight to stderr is
//! measured by nothing, so it keeps its own text and only loses the runs a terminal acts on.
//! Both live here so a new display route picks one deliberately instead of inventing a third.
//!
//! Making a byte safe to print says NOTHING about whether it is trustworthy or fit to retain.
//! Both are pure (`inv-determinism`) and both are IDEMPOTENT — which is what lets a sweep assert
//! a rendered surface is already clean without knowing which seat produced it.

use std::fmt::Write as _;

/// Unicode format (general category `Cf`) and explicit bidi-control code points, as ranges.
///
/// Hand-rolled against Unicode 15.1 because the crate carries no dependencies and this list is
/// closed and small. It exists for one reason: a comment that DISPLAYS differently than it lexes
/// defeats a surface whose whole purpose is showing a person the code they are judging. The
/// explicit isolate/override block (`U+202A..U+202E`, `U+2066..U+2069`) is inside `Cf` already and
/// is spelled out anyway, because it is the set that motivated the rule.
const FORMAT_AND_BIDI: &[(char, char)] = &[
    ('\u{00ad}', '\u{00ad}'),
    ('\u{0600}', '\u{0605}'),
    ('\u{061c}', '\u{061c}'),
    ('\u{06dd}', '\u{06dd}'),
    ('\u{070f}', '\u{070f}'),
    ('\u{0890}', '\u{0891}'),
    ('\u{08e2}', '\u{08e2}'),
    ('\u{180e}', '\u{180e}'),
    ('\u{200b}', '\u{200f}'),
    ('\u{202a}', '\u{202e}'),
    ('\u{2060}', '\u{2064}'),
    ('\u{2066}', '\u{206f}'),
    ('\u{feff}', '\u{feff}'),
    ('\u{fff9}', '\u{fffb}'),
    ('\u{110bd}', '\u{110bd}'),
    ('\u{110cd}', '\u{110cd}'),
    ('\u{13430}', '\u{1343f}'),
    ('\u{1bca0}', '\u{1bca3}'),
    ('\u{1d173}', '\u{1d17a}'),
    ('\u{e0001}', '\u{e0001}'),
    ('\u{e0020}', '\u{e007f}'),
];

/// Whether `c` is a format or bidi control — a character that changes how its NEIGHBOURS display
/// without displaying anything itself.
#[must_use]
pub fn is_format_or_bidi(c: char) -> bool {
    FORMAT_AND_BIDI.iter().any(|(lo, hi)| c >= *lo && c <= *hi)
}

/// Whether `c` may reach a display surface unencoded: anything that is neither a control character
/// nor a format/bidi control.
#[must_use]
pub fn is_display_safe(c: char) -> bool {
    !c.is_control() && !is_format_or_bidi(c)
}

/// The ASCII truncation marker. ASCII forever (`rul-ascii-output-forever`); a `…` here would be
/// multiplied across every capped surface.
const ELLIPSIS: &str = "...";

/// Encode a value for a PLAIN line — an advisory printed straight to a terminal, nothing measuring
/// it.
///
/// Control characters and format/bidi controls become a space; every other character survives as
/// the author wrote it. The result never exceeds `cap` bytes, truncation is at a character
/// boundary, and a truncated result ends in [`ELLIPSIS`] — capping the WHOLE result rather than
/// just the retained content is what makes the function idempotent, which a re-encoded value
/// depends on.
#[must_use]
pub fn encode_line(text: &str, cap: usize) -> String {
    let cleaned: String = text
        .chars()
        .map(|c| if is_display_safe(c) { c } else { ' ' })
        .collect();
    cap_to(&cleaned, cap)
}

/// Encode bytes taken from somebody else's file — an oracle arm, its author's comment, a book
/// line, a host's own words — for a MEASURED surface.
///
/// Printable ASCII survives verbatim; every other byte becomes `\xNN`, capped at `cap` bytes.
/// Escaping rather than blanking, because a reader deciding about code needs to see that
/// something was there, and the escaped form measures as the columns it occupies.
///
/// Deliberately narrow: this is terminal safety and column arithmetic, not quoting — a backslash
/// stays a backslash, so an oracle's `printf '%s\n'` reads as its author wrote it. The cost is
/// that an escaped byte and a source backslash-x are spelled the same; doubling every backslash
/// in every shell excerpt would be a worse lie about the source more often.
#[must_use]
pub fn encode_foreign(text: &str, cap: usize) -> String {
    cap_to(&encode_ascii_bytes(text), cap)
}

/// Encode source-derived text inside a generated POSIX-shell comment.
///
/// This is a distinct sink from terminal display even though v0 uses the same byte spelling:
/// printable ASCII survives, while every byte capable of ending or controlling the comment is
/// rendered as `\xNN`. The result is capped and contains no newline or control byte.
#[must_use]
pub fn encode_shell_comment(text: &str, cap: usize) -> String {
    cap_to(&encode_ascii_bytes(text), cap)
}

fn encode_ascii_bytes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        if (0x20..=0x7e).contains(&byte) {
            out.push(char::from(byte));
        } else {
            let _ = write!(out, "\\x{byte:02x}");
        }
    }
    out
}

/// Truncate `text` so the RESULT is at most `cap` bytes, ending in [`ELLIPSIS`] when anything was
/// dropped. A `cap` too small to hold the marker truncates bare rather than emitting a marker
/// longer than the budget.
fn cap_to(text: &str, cap: usize) -> String {
    if text.len() <= cap {
        return text.to_owned();
    }
    let budget = cap.saturating_sub(ELLIPSIS.len());
    let mut end = budget.min(text.len());
    while end > 0 && !text.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    let head = text.get(..end).unwrap_or_default();
    if cap < ELLIPSIS.len() {
        return head.to_owned();
    }
    format!("{head}{ELLIPSIS}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bidi/format rule is the reason this seat exists: an override inside a comment makes the
    /// displayed line disagree with the lexed one, on a surface whose whole job is showing a
    /// person the code they are deciding about.
    #[test]
    fn a_bidi_override_never_survives_either_encoding() {
        let hidden = "ok \u{202e}gnitucexe\u{202c} ok";
        let line = encode_line(hidden, 200);
        assert!(
            !line.chars().any(is_format_or_bidi),
            "a plain line blanks the override: {line:?}"
        );
        let foreign = encode_foreign(hidden, 200);
        assert!(
            foreign.is_ascii() && !foreign.contains('\u{202e}'),
            "a measured surface escapes it: {foreign:?}"
        );
    }

    #[test]
    fn control_characters_leave_both_encodings() {
        assert_eq!(encode_line("a\tb\u{7}c\u{7f}d", 200), "a b c d");
        assert_eq!(encode_foreign("a\tb", 200), "a\\x09b");
        assert_eq!(encode_foreign("\u{1b}[31m", 200), "\\x1b[31m");
    }

    #[test]
    fn shell_comment_encoding_cannot_open_another_line() {
        let encoded = encode_shell_comment("path\n# forged\r\u{202e}", 200);
        assert_eq!(encoded, "path\\x0a# forged\\x0d\\xe2\\x80\\xae");
        assert!(!encoded.contains(['\n', '\r']));
    }

    /// Printable ASCII is the author's own text and is never touched — the escape is for bytes
    /// that would otherwise lie about themselves, not for quoting.
    #[test]
    fn printable_ascii_survives_escaping_verbatim() {
        let arm = r#"push) printf '%s\n' "$1"  : disturbs org.foob.Certs ;;"#;
        assert_eq!(encode_foreign(arm, 4096), arm);
        assert_eq!(encode_line(arm, 4096), arm);
    }

    /// Idempotence is a REQUIREMENT, not an observation: the sweep that protects this surface
    /// asserts "already encoded" by encoding again and comparing, so a second pass that changed
    /// anything would make the sweep unable to tell a clean render from a dirty one.
    #[test]
    fn both_encodings_are_idempotent_including_at_the_cap() {
        let samples = [
            "plain ascii",
            "a\tb\u{7}c",
            "na\u{ef}ve \u{202e}flip\u{202c}",
            &"x".repeat(500),
            &format!("{}\u{ef}", "y".repeat(199)),
            "...",
            "short",
        ];
        for cap in [0_usize, 1, 2, 3, 4, 8, 200] {
            for sample in samples {
                let once = encode_line(sample, cap);
                assert_eq!(
                    encode_line(&once, cap),
                    once,
                    "encode_line(cap={cap}) is not idempotent on {sample:?}"
                );
                assert!(
                    once.len() <= cap,
                    "encode_line(cap={cap}) overran: {once:?}"
                );
                let once = encode_foreign(sample, cap);
                assert_eq!(
                    encode_foreign(&once, cap),
                    once,
                    "encode_foreign(cap={cap}) is not idempotent on {sample:?}"
                );
                assert!(
                    once.len() <= cap,
                    "encode_foreign(cap={cap}) overran: {once:?}"
                );
            }
        }
    }

    /// An author's text is never silently dropped: something non-empty always survives a cap, so
    /// a reader is never shown an empty quotation where a line existed.
    #[test]
    fn a_capped_value_stays_non_empty_when_there_is_room_for_anything() {
        for cap in [4_usize, 8, 32] {
            assert!(!encode_line(&"z".repeat(400), cap).is_empty());
            assert!(!encode_foreign(&"\u{1b}".repeat(400), cap).is_empty());
        }
    }

    /// The truncation marker is three ASCII dots, on both encodings and every caller
    /// (`rul-ascii-output-forever`): one shared seat is exactly what makes it one marker.
    #[test]
    fn the_truncation_marker_is_ascii() {
        assert!(ELLIPSIS.is_ascii() && ELLIPSIS == "...");
        for capped in [
            encode_line(&"z".repeat(400), 32),
            encode_foreign(&"z".repeat(400), 32),
        ] {
            assert!(capped.ends_with("...") && capped.is_ascii(), "{capped:?}");
        }
    }
}
