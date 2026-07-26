//! The adapter seam between the describe plane and `weft`, the layout engine.
//!
//! Weft is generic over an opaque key and knows nothing about Dorc
//! (`28E:rul-tree-render-is-a-firewalled-crate`). This module supplies the key —
//! which is exactly this crate's render-part identity, since that is what the
//! spans have to name for a round-trip to mean anything — plus the three run
//! constructors a describe-plane render needs. It composes no document: the why
//! surface's composition needs `dorc_plan` types this crate may never depend on
//! (`aid-is-the-describe-plane`), so the tree is built at the cli edge out of
//! runs minted here.
//!
//! The division of labour is the one law that matters:
//! **weft self-mints WORDLESS geometry only** (`28F:rul-weft-geometry-vs-words`).
//! Section rules, gutter bars, quotes, indentation — weft's. Every English word,
//! including section headers, tier verbs, row labels and join connectives,
//! arrives as a run from here, backed by a registry row (`28G` §0). A `format!`
//! literal reaching a render is the failure this exists to prevent.

use weft::{Instance, Provenance, Run};

/// The identity a rendered run carries into weft's span map.
///
/// One namespace spanning every identity a describe-plane render needs, which is
/// the ordinary shape for a weft key: weft never inspects one, so keeping the
/// namespaces straight is the consumer's job.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Face {
    /// An arrangement-registry row, named by its slug.
    Row(&'static str),
    /// A named part of a row — the parameter a computed value filled.
    Part(&'static str),
    /// Bytes that are NOT ours, named by where they were taken from.
    Source(String),
    /// A table identity: what relates rows that are not siblings.
    Table(&'static str),
}

/// A whole registry-sourced line, as one run.
///
/// One span per chrome line, never word-spans fenced by computed ones
/// (`a-chrome-line-is-one-span`): the edit transport anchors sections on the
/// immutable text between them, and a bare digit is not an anchor.
#[must_use]
pub fn words(text: impl Into<String>, slug: &'static str) -> Run<Face> {
    Run::new(
        text,
        Provenance::Arrangement {
            key: Some(Face::Row(slug)),
        },
    )
}

/// A computed value: a coordinate, an address, a count. Never editable —
/// rewriting one would be lying about the world rather than rephrasing a
/// sentence.
#[must_use]
pub fn value(text: impl Into<String>, slug: &'static str, part: &'static str) -> Run<Face> {
    Run::param(text, Face::Row(slug), Face::Part(part), Instance(0))
}

/// Bytes taken from somebody else's file — an oracle arm, its author's comment,
/// a book line — escaped for display.
///
/// Two obligations ride this class and both are discharged here (`28G` §0's
/// foreign-text carve). The bytes are stamped [`Provenance::Foreign`], so a
/// round-trip can never mistake them for editable prose; and they are encoded
/// before they enter the render, because weft measures bytes as columns and does
/// not sanitise its input, so an unescaped control byte would corrupt the layout
/// as well as the terminal (`28D:must-encode-per-surface`).
#[must_use]
pub fn foreign(text: &str, source: impl Into<String>) -> Run<Face> {
    Run::foreign(escape_foreign(text), Face::Source(source.into()))
}

/// The display encoding for not-ours bytes: printable ASCII survives verbatim,
/// everything else becomes `\xNN`.
///
/// Deliberately narrow. The job is terminal safety and honest column arithmetic,
/// not quoting — a backslash is printable and stays a backslash, so an oracle's
/// `printf '%s\n'` still reads as the author wrote it rather than as `\\n`. The
/// cost of that choice is that an escaped byte and a source backslash-x are
/// spelled the same; the alternative doubles every backslash in every shell
/// excerpt on the surface, which is a worse lie about the source more often.
#[must_use]
pub fn escape_foreign(text: &str) -> String {
    use std::fmt::Write as _;
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

#[cfg(test)]
mod tests {
    use super::*;
    use weft::{
        CodeBlock, CodeCell, CodeLine, Document, Literalness, Node, NodeKind, Span, render,
    };

    #[test]
    fn printable_ascii_survives_escaping_verbatim() {
        let arm = r#"push) printf '%s\n' "$1"  : disturbs org.foob.Certs ;;"#;
        assert_eq!(
            escape_foreign(arm),
            arm,
            "an oracle arm is shown as its author wrote it; escaping quotes or backslashes \
             would be a lie about the source"
        );
    }

    #[test]
    fn a_control_or_non_ascii_byte_is_encoded_before_it_reaches_layout() {
        assert_eq!(escape_foreign("a\tb"), "a\\x09b");
        assert_eq!(escape_foreign("\u{1b}[31m"), "\\x1b[31m");
        assert_eq!(escape_foreign("caf\u{e9}"), "caf\\xc3\\xa9");
    }

    /// The tagging is load-bearing, not decorative: an edge must be able to find
    /// not-ours bytes in the span map, and every byte they contributed must
    /// already be safe to print.
    #[test]
    fn foreign_bytes_are_findable_in_the_span_map_and_already_encoded() {
        let document = Document::new(vec![Node::new(NodeKind::Code(CodeBlock {
            table: None,
            mode: Literalness::Literal,
            locus: None,
            lines: vec![CodeLine {
                gutter: Some(value("31", "excerpt", "line")),
                cells: vec![CodeCell::new(vec![foreign(
                    "push)\tprintf 'x'\u{7}",
                    "certsync.oracle.sh",
                )])],
            }],
        }))]);
        let rendered = render(&document, 80);
        let foreign_spans: Vec<&Span<Face>> = rendered
            .spans()
            .iter()
            .filter(|span| matches!(span.provenance, Provenance::Foreign { .. }))
            .collect();
        assert_eq!(foreign_spans.len(), 1, "the arm is one foreign run");
        assert!(
            matches!(
                &foreign_spans[0].provenance,
                Provenance::Foreign { key: Face::Source(file) } if file == "certsync.oracle.sh"
            ),
            "a foreign span names the file its bytes came from"
        );
        let bytes = rendered
            .text()
            .get(foreign_spans[0].start..foreign_spans[0].end())
            .expect("the span lies within the render");
        assert_eq!(bytes, "push)\\x09printf 'x'\\x07");
    }
}
