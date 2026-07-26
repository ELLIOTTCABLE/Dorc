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
    /// A table identity: what relates members that are not siblings. Owned rather than static
    /// because some tables are per-instance — the two halves of one cut excerpt relate to each
    /// other and to nothing else.
    Table(String),
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
///
/// Encoded on the way in, at `cap` bytes. Many of these values are ENGINE-shaped and the encoding
/// is a no-op on them, but several are not — an oracle's coordinate text, a speaker's `file:line`,
/// a book site's `N|command` — and a value seat that encoded only the ones it believed were
/// foreign would be one audit away from wrong. The registry WORDS a value is interleaved with are
/// never encoded (see [`words`]); only the value is.
#[must_use]
pub fn value(
    text: impl AsRef<str>,
    slug: &'static str,
    part: &'static str,
    cap: usize,
) -> Run<Face> {
    Run::param(
        crate::display::encode_foreign(text.as_ref(), cap),
        Face::Row(slug),
        Face::Part(part),
        Instance(0),
    )
}

/// Structure the CONSUMER computed rather than weft: a rank glyph, a separator
/// the consumer owns, the space between two joined fragments.
///
/// Punctuation is not a word (`layout-is-not-a-word`), so this is deliberately
/// not a registry row — putting shape in an editable entry would weld the
/// arrangement that `27V:rul-output-form-unwelded` keeps free. Keyed by a
/// [`Face::Part`] rather than a [`Face::Row`], which is what tells it apart from
/// registry words in the span map.
#[must_use]
pub fn mark(text: impl Into<String>, part: &'static str) -> Run<Face> {
    Run::new(
        text,
        Provenance::Arrangement {
            key: Some(Face::Part(part)),
        },
    )
}

/// Bytes taken from somebody else's file — an oracle arm, its author's comment,
/// a book line — escaped for display.
///
/// Two obligations ride this class and both are discharged here (`28G` §0's
/// foreign-text carve). The bytes are stamped [`Provenance::Foreign`], so a
/// round-trip can never mistake them for editable prose; and they are encoded
/// before they enter the render, because weft measures bytes as columns and does
/// not sanitise its input, so an unescaped control byte would corrupt the layout
/// as well as the terminal (`28D:must-encode-per-surface`). The encoding itself is
/// [`crate::display::encode_foreign`], the shared seat both display destinations answer to.
///
/// The `cap` is the CALLER's, in bytes: a breadcrumb and a quoted source line want very different
/// budgets, and the seat has no way to tell which it is holding.
#[must_use]
pub fn foreign(text: &str, source: impl Into<String>, cap: usize) -> Run<Face> {
    Run::foreign(
        crate::display::encode_foreign(text, cap),
        Face::Source(source.into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use weft::{
        CodeBlock, CodeCell, CodeLine, Document, Literalness, Node, NodeKind, Span, render,
    };

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
                gutter: Some(value("31", "excerpt", "line", 32)),
                cells: vec![CodeCell::new(vec![foreign(
                    "push)\tprintf 'x'\u{7}",
                    "certsync.oracle.sh",
                    512,
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
