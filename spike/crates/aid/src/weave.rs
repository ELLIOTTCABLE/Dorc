//! The adapter seam between the describe plane and `weft`, the layout engine.
//!
//! Weft is generic over an opaque key and knows nothing about Dorc
//! (`28E:rul-tree-render-is-a-firewalled-crate`). This module supplies the key —
//! which is exactly this crate's render-part identity, since that is what the
//! spans have to name for a round-trip to mean anything — plus the run
//! constructors a describe-plane render needs, plus the bridge that turns weft's
//! span map back into a [`RenderParts`] stream the loom transport understands.
//! It composes no document: the why surface's composition needs `dorc_plan`
//! types this crate may never depend on (`aid-is-the-describe-plane`), so the
//! tree is built at the cli edge out of runs minted here.
//!
//! The division of labour is the one law that matters:
//! **weft self-mints WORDLESS geometry only** (`28F:rul-weft-geometry-vs-words`).
//! Section rules, gutter bars, quotes, indentation — weft's. Every English word,
//! including section headers, tier verbs, row labels and join connectives,
//! arrives as a run from here, backed by a registry row (`28G` §0). A `format!`
//! literal reaching a render is the failure this exists to prevent.

use weft::{Instance, Provenance, Rendered, Run, Span};

use crate::tagged::{Field, RenderPart, RenderParts};

/// The identity a rendered run carries into weft's span map.
///
/// One namespace spanning every identity a describe-plane render needs, which is
/// the ordinary shape for a weft key: weft never inspects one, so keeping the
/// namespaces straight is the consumer's job.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Face {
    /// An arrangement-registry row, named by its slug and by the occurrence the
    /// seat resolved it at.
    ///
    /// The occurrence rides HERE rather than being resolved and dropped at the
    /// seat, because a registry key is `(slug, occurrence)` and sixteen reached
    /// rows are occurrence-keyed: without it the transport would key an edit by
    /// RENDER POSITION and quietly rewrite a different entry
    /// (`_w4-map-DRAFT:gap-occurrence-lost-at-the-weave-seat`).
    Row {
        /// The registry slug.
        slug: &'static str,
        /// The occurrence the seat resolved, or `None` for the whole-slug entry.
        occurrence: Option<usize>,
    },
    /// A named part of a row — the parameter a computed value filled.
    Part(&'static str),
    /// Bytes that are NOT ours, named by where they were taken from.
    Source(String),
    /// A table identity: what relates members that are not siblings. Owned rather than static
    /// because some tables are per-instance — the two halves of one cut excerpt relate to each
    /// other and to nothing else.
    Table(String),
    /// A diagnostic code — the catalog row a prose register belongs to.
    Code(&'static str),
    /// Which prose register of a catalog row, and which paragraph of it.
    Register {
        /// The register.
        field: Field,
        /// Zero-based paragraph within the register.
        paragraph: usize,
    },
    /// A named interpolation hole in a catalog register.
    Hole {
        /// The register the hole sits in.
        field: Field,
        /// The declared param name.
        param: &'static str,
    },
}

/// The display budget for one interpolated value on a laid-out surface.
///
/// Generous — several of these carry an engine-authored sentence — and bounded, because some
/// carry text a book, not we, decided the length of.
pub const RENDER_VALUE_CAP: usize = 2048;

/// The display budget for one quoted line of somebody else's source on a laid-out surface.
pub const RENDER_SOURCE_CAP: usize = 4096;

/// The [`RenderPart::Arrangement`] slug for weft's own layout: indentation, padding, line
/// breaks, column separators. Renderer-computed and never an edit region.
pub const WEFT_LAYOUT: &str = "weft-layout";

/// The [`RenderPart::Arrangement`] slug for a span whose key shape the bridge does not model.
/// Unreachable from the why surface's own vocabulary; present so the bridge is total rather
/// than panicking on a key namespace somebody later adds.
pub const WEFT_UNKEYED: &str = "weft-unkeyed";

/// One run of a registry-sourced line.
///
/// A chrome line is ONE editable SECTION, never one span: the seat interleaves
/// its computed values between the entry's words and each piece arrives here
/// separately, so the transport can re-split an edited line at the boundaries
/// the render actually stamped (`28H` ruling 3).
#[must_use]
pub fn words(text: impl Into<String>, slug: &'static str, occurrence: Option<usize>) -> Run<Face> {
    Run::new(
        text,
        Provenance::Arrangement {
            key: Some(Face::Row { slug, occurrence }),
        },
    )
}

/// A computed value INTERLEAVED into a registry line, at its positional index.
///
/// Never editable — rewriting one would be lying about the world rather than rephrasing a
/// sentence — but it lives INSIDE its row's section, so an edit to the words around it keeps
/// its identity.
///
/// Encoded on the way in, at `cap` bytes. Many of these values are ENGINE-shaped and the
/// encoding is a no-op on them, but several are not — an oracle's coordinate text, a speaker's
/// `file:line`, a book site's `N|command` — and a value seat that encoded only the ones it
/// believed were foreign would be one audit away from wrong. The registry WORDS a value is
/// interleaved with are never encoded (see [`words`]); only the value is.
#[must_use]
pub fn sentence_value(
    text: impl AsRef<str>,
    slug: &'static str,
    occurrence: Option<usize>,
    index: usize,
    cap: usize,
) -> Run<Face> {
    Run::param(
        crate::display::encode_foreign(text.as_ref(), cap),
        Face::Row { slug, occurrence },
        Face::Part("value"),
        Instance(u32::try_from(index).unwrap_or(u32::MAX)),
    )
}

/// A computed value standing on its OWN — a speaker column, a gutter line number, an address.
///
/// Keyed by its seat rather than by a registry row, because there is no sentence for it to sit
/// inside; the transport therefore reads it as immutable structure.
#[must_use]
pub fn value(text: impl AsRef<str>, part: &'static str, cap: usize) -> Run<Face> {
    Run::param(
        crate::display::encode_foreign(text.as_ref(), cap),
        Face::Part(part),
        Face::Part("value"),
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

/// A run of catalog or registry prose, as the whitespace-absorption rule compares them.
///
/// Two stretches of ONE row are what the rule looks for: a break weft minted between them is that
/// row's own inter-word space wearing the renderer's clothes.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Row {
    Arrangement {
        slug: &'static str,
        occurrence: Option<usize>,
    },
    Register {
        code: &'static str,
        field: Field,
        instance: usize,
    },
}

/// What one stretch of rendered output IS, for transport purposes — the bridge's working
/// vocabulary between weft's span map and the loom's part stream.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Facet {
    Words {
        slug: &'static str,
        occurrence: Option<usize>,
    },
    Value {
        slug: &'static str,
        occurrence: Option<usize>,
        index: usize,
    },
    Template {
        code: &'static str,
        field: Field,
        paragraph: usize,
        instance: usize,
    },
    Param {
        code: &'static str,
        field: Field,
        param: &'static str,
        instance: usize,
    },
    Foreign(String),
    Computed(&'static str),
    Layout,
}

impl Facet {
    /// The prose row this stretch belongs to, if any.
    fn row(&self) -> Option<Row> {
        match self {
            Facet::Words { slug, occurrence }
            | Facet::Value {
                slug, occurrence, ..
            } => Some(Row::Arrangement {
                slug,
                occurrence: *occurrence,
            }),
            Facet::Template {
                code,
                field,
                instance,
                ..
            }
            | Facet::Param {
                code,
                field,
                instance,
                ..
            } => Some(Row::Register {
                code,
                field: *field,
                instance: *instance,
            }),
            _ => None,
        }
    }

    /// The row's own WORDS facet — what an absorbed inter-word space belongs to when the two
    /// stretches around it are different pieces of one row.
    fn words_of(row: &Row, paragraph: usize) -> Facet {
        match row {
            Row::Arrangement { slug, occurrence } => Facet::Words {
                slug,
                occurrence: *occurrence,
            },
            Row::Register {
                code,
                field,
                instance,
            } => Facet::Template {
                code,
                field: *field,
                paragraph,
                instance: *instance,
            },
        }
    }

    /// The paragraph a stretch names, for reconstructing an absorbed space's own facet.
    fn paragraph(&self) -> Option<usize> {
        match self {
            Facet::Template { paragraph, .. } => Some(*paragraph),
            _ => None,
        }
    }
}

fn facet_of(provenance: &Provenance<Face>) -> Facet {
    match provenance {
        Provenance::Arrangement {
            key: Some(Face::Row { slug, occurrence }),
        } => Facet::Words {
            slug,
            occurrence: *occurrence,
        },
        Provenance::Param {
            key: Face::Row { slug, occurrence },
            instance,
            ..
        } => Facet::Value {
            slug,
            occurrence: *occurrence,
            index: instance.0 as usize,
        },
        Provenance::Template {
            key: Face::Code(code),
            field: Face::Register { field, paragraph },
            instance,
        } => Facet::Template {
            code,
            field: *field,
            paragraph: *paragraph,
            instance: instance.0 as usize,
        },
        Provenance::Param {
            key: Face::Code(code),
            param: Face::Hole { field, param },
            instance,
        } => Facet::Param {
            code,
            field: *field,
            param,
            instance: instance.0 as usize,
        },
        Provenance::Foreign {
            key: Face::Source(source),
        } => Facet::Foreign(source.clone()),
        Provenance::Arrangement {
            key: Some(Face::Part(part)),
        }
        | Provenance::Param {
            key: Face::Part(part),
            ..
        } => Facet::Computed(part),
        Provenance::Arrangement { key: None } => Facet::Layout,
        _ => Facet::Computed(WEFT_UNKEYED),
    }
}

/// Map a part stream INTO weft's run vocabulary — the direction a seat composing a document needs.
///
/// The inverse of [`to_render_parts`], and the two must stay inverses: a run this mints has to
/// come back as the part it was born from, or an edit lands on the wrong register.
///
/// Values are encoded on the way in. A laid-out surface measures bytes as columns and does not
/// sanitise its input, so a control byte would corrupt the geometry as well as the terminal
/// (`28D:must-encode-per-surface`); our OWN words — catalog literals, registry words, computed
/// chrome — are never encoded, because encoding them twice would be a defect.
#[must_use]
pub fn to_runs(parts: &RenderParts) -> Vec<Run<Face>> {
    parts
        .parts()
        .iter()
        .map(|part| match part {
            RenderPart::TemplateLiteral {
                text,
                code,
                field,
                paragraph,
                instance,
            } => Run::template(
                text.clone(),
                Face::Code(code),
                Face::Register {
                    field: *field,
                    paragraph: *paragraph,
                },
                Instance(u32::try_from(*instance).unwrap_or(u32::MAX)),
            ),
            RenderPart::ParamValue {
                text,
                code,
                field,
                param,
                instance,
            } => Run::param(
                crate::display::encode_foreign(text, RENDER_VALUE_CAP),
                Face::Code(code),
                Face::Hole {
                    field: *field,
                    param,
                },
                Instance(u32::try_from(*instance).unwrap_or(u32::MAX)),
            ),
            RenderPart::ForeignText { text, source } => {
                foreign(text, source.clone(), RENDER_VALUE_CAP)
            }
            RenderPart::Arrangement { text, slug } | RenderPart::ArrangementPage { text, slug } => {
                mark(text.clone(), slug)
            }
            RenderPart::ArrangementWords {
                text,
                slug,
                occurrence,
            } => words(text.clone(), slug, *occurrence),
            RenderPart::ArrangementValue {
                text,
                slug,
                occurrence,
                index,
            } => sentence_value(text, slug, *occurrence, *index, RENDER_VALUE_CAP),
        })
        .collect()
}

/// Map a weft render back to the loom's part stream — the bridge that gives the why surface an
/// editable face (`_w4-map-DRAFT:gap-no-weft-to-parts-bridge`).
///
/// Two rules do the work beyond the flat key mapping, and both exist because weft WRAPS:
///
/// - a run is tokenized before wrapping, so one chrome line arrives as many spans sharing one
///   provenance; adjacent equal facets coalesce back into one part.
/// - at a break weft drops the whitespace token and mints its own newline+pad
///   (`Arrangement { key: None }`). A pure-whitespace layout stretch lying BETWEEN two stretches
///   of the SAME registry row is that row's own inter-word space wearing the renderer's clothes,
///   so it is absorbed into the row rather than splitting it. Compile-back collapses whitespace
///   runs to one space, which is what makes the absorption lossless (`28H` ruling 7).
#[must_use]
pub fn to_render_parts(rendered: &Rendered<Face>) -> RenderParts {
    let text = rendered.text();
    let spans = rendered.spans();
    let mut facets: Vec<Facet> = spans
        .iter()
        .map(|span| facet_of(&span.provenance))
        .collect();
    for index in 0..facets.len() {
        if !matches!(facets.get(index), Some(Facet::Layout))
            || !span_text(text, spans, index)
                .is_some_and(|bytes| !bytes.is_empty() && bytes.chars().all(char::is_whitespace))
        {
            continue;
        }
        let before = index.checked_sub(1).and_then(|before| facets.get(before));
        let after = facets.get(index.saturating_add(1));
        let (Some(before), Some(after)) = (before, after) else {
            continue;
        };
        let (Some(row), Some(next_row)) = (before.row(), after.row()) else {
            continue;
        };
        if row != next_row {
            continue;
        }
        // Inside ONE value the absorbed break belongs to that value; across a word/value
        // boundary it is the row's own inter-word space.
        let absorbed = if before == after {
            before.clone()
        } else {
            let paragraph = before
                .paragraph()
                .or_else(|| after.paragraph())
                .unwrap_or(0);
            Facet::words_of(&row, paragraph)
        };
        if let Some(slot) = facets.get_mut(index) {
            *slot = absorbed;
        }
    }

    let mut parts = RenderParts::new();
    let mut open: Option<(Facet, String)> = None;
    for (index, facet) in facets.into_iter().enumerate() {
        let Some(bytes) = span_text(text, spans, index) else {
            continue;
        };
        match &mut open {
            Some((current, accumulated)) if *current == facet => accumulated.push_str(bytes),
            _ => {
                if let Some((current, accumulated)) = open.take() {
                    parts.push(part_of(current, accumulated));
                }
                open = Some((facet, bytes.to_owned()));
            }
        }
    }
    if let Some((current, accumulated)) = open {
        parts.push(part_of(current, accumulated));
    }
    parts
}

fn span_text<'a>(text: &'a str, spans: &[Span<Face>], index: usize) -> Option<&'a str> {
    let span = spans.get(index)?;
    text.get(span.start..span.end())
}

fn part_of(facet: Facet, text: String) -> RenderPart {
    match facet {
        Facet::Words { slug, occurrence } => RenderPart::ArrangementWords {
            text,
            slug,
            occurrence,
        },
        Facet::Value {
            slug,
            occurrence,
            index,
        } => RenderPart::ArrangementValue {
            text,
            slug,
            occurrence,
            index,
        },
        Facet::Template {
            code,
            field,
            paragraph,
            instance,
        } => RenderPart::TemplateLiteral {
            text,
            code,
            field,
            paragraph,
            instance,
        },
        Facet::Param {
            code,
            field,
            param,
            instance,
        } => RenderPart::ParamValue {
            text,
            code,
            field,
            param,
            instance,
        },
        Facet::Foreign(source) => RenderPart::ForeignText { text, source },
        Facet::Computed(slug) => RenderPart::Arrangement { text, slug },
        Facet::Layout => RenderPart::Arrangement {
            text,
            slug: WEFT_LAYOUT,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weft::{
        CodeBlock, CodeCell, CodeLine, Document, Literalness, Node, NodeKind, Paragraph, render,
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
                gutter: Some(value("31", "excerpt-line", 32)),
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

    fn sentence(runs: Vec<Run<Face>>) -> Document<Face> {
        Document::new(vec![Node::new(NodeKind::Prose(Paragraph { runs }))])
    }

    /// The bridge's whole promise: the bytes it re-attributes are the bytes weft printed.
    #[test]
    fn the_part_stream_reproduces_the_render() {
        let document = sentence(vec![
            words("ran because operand ", "why-reason", Some(1)),
            sentence_value("3", "why-reason", Some(1), 0, 240),
            words(
                " is a command-substitution nothing could be said about",
                "why-reason",
                Some(1),
            ),
        ]);
        let rendered = render(&document, 40);
        assert_eq!(to_render_parts(&rendered).text(), rendered.text());
    }

    /// A wrapped chrome line is still ONE row: the newline+pad weft mints where it dropped the
    /// row's own space is absorbed back into the row, so the line does not fragment into
    /// sections the transport would have to re-anchor between.
    #[test]
    fn a_wrap_inside_a_chrome_line_stays_one_row() {
        let document = sentence(vec![
            words("ran because operand ", "why-reason", Some(1)),
            sentence_value("3", "why-reason", Some(1), 0, 240),
            words(
                " is a command-substitution nothing could be said about",
                "why-reason",
                Some(1),
            ),
        ]);
        let rendered = render(&document, 40);
        assert!(
            rendered.text().contains('\n'),
            "the fixture must actually wrap or it proves nothing"
        );
        let parts = to_render_parts(&rendered);
        let split = parts.parts().windows(2).any(|pair| {
            matches!(pair.first(), Some(RenderPart::Arrangement { slug, .. }) if *slug == WEFT_LAYOUT)
                && matches!(
                    pair.last(),
                    Some(RenderPart::ArrangementWords { .. } | RenderPart::ArrangementValue { .. })
                )
        });
        assert!(!split, "no layout byte splits the row: {parts:?}");
    }
}
