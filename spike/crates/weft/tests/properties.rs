//! The properties that make weft trustworthy enough to build on.
//!
//! Goldens show what the output looks like; these pin what is true of it at
//! every width. Each test states the invariant it exists to defend, because an
//! invariant nobody can name is one nobody will preserve.

#![expect(
    clippy::expect_used,
    clippy::panic,
    clippy::arithmetic_side_effects,
    reason = "test assertions: panic-based checks and plain counter arithmetic over fixture-sized data"
)]

use weft::{
    CodeBlock, CodeLine, Document, Frame, Instance, LabeledRow, Literalness, Node, NodeKind,
    Paragraph, Payload, Provenance, Quoting, Rendered, Reservation, Run, Section, Side, SpeakerRow,
    Width, render, render_framed,
};

/// The width sweep every whole-document property runs over: narrow enough to
/// force column stacking, wide enough to leave everything on one line.
const SWEEP: std::ops::RangeInclusive<usize> = 12..=140;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Key {
    Row(&'static str),
    Part(&'static str),
}

fn text(text: &str) -> Run<Key> {
    Run::template(text, Key::Row("row"), Key::Part("message"), Instance(0))
}

fn value(text: &str) -> Run<Key> {
    Run::param(text, Key::Row("row"), Key::Part("value"), Instance(0))
}

fn source(text: &str) -> Run<Key> {
    Run::foreign(text, Key::Row("source"))
}

fn prose(runs: Vec<Run<Key>>) -> Node<Key> {
    Node::new(NodeKind::Prose(Paragraph { runs }))
}

/// Prose only, with every word short enough to fit the narrowest swept width —
/// so the wrapper has no excuse to overrun.
fn wrappable_document() -> Document<Key> {
    Document::new(vec![
        prose(vec![text(
            "the guard did its job and this is the fall through working not failing",
        )]),
        prose(vec![
            text("a report is a moment "),
            value("not"),
            text(" a promise and the live re check caught the drift"),
        ]),
    ])
}

/// Every node kind at once, including material that is allowed to overrun.
fn mixed_document() -> Document<Key> {
    Document::new(vec![Node::new(NodeKind::Section(Section {
        header: vec![value("ANALYSIS")],
        counts: Some(vec![value("3")]),
        body: vec![
            prose(vec![text("what was known, and why it was not enough:")]),
            Node::new(NodeKind::Speaker(SpeakerRow {
                gutter: Some(value("*")),
                speaker: vec![value("ufw.oracle.sh:44")],
                verb: Some(vec![value("reported")]),
                payload: Payload {
                    quoting: Quoting::Quoted,
                    runs: vec![value("Firewall:443/tcp@allowed")],
                    trailer: vec![text(" (ran 09:13:51, rc 0)")],
                },
                attachments: vec![Node::new(NodeKind::Code(CodeBlock {
                    mode: Literalness::Literal,
                    locus: Some(vec![value("ufw.oracle.sh, as-written:")]),
                    lines: vec![CodeLine {
                        gutter: Some(value("44")),
                        content: vec![source(
                            "ufw status verbose | grep -q \"$1\"  : org.ufw.Firewall:\"$1\"@allowed",
                        )],
                    }],
                }))],
            })),
            Node::new(NodeKind::Labeled(LabeledRow {
                label: vec![value("so:")],
                body: vec![text("the report was good when it ran")],
                attachments: Vec::new(),
            })),
        ],
    }))])
}

fn lines_of(rendered: &Rendered<Key>) -> Vec<&str> {
    rendered.text().lines().collect()
}

#[test]
fn spans_totally_cover_the_output() {
    for width in SWEEP {
        for document in [wrappable_document(), mixed_document()] {
            let rendered = render(&document, width);
            let mut offset = 0usize;
            let mut rebuilt = String::new();
            for span in rendered.spans() {
                assert_eq!(
                    span.start, offset,
                    "spans must be contiguous and in output order (width {width})"
                );
                assert!(span.len > 0, "an empty span attributes nothing");
                let slice = rendered
                    .text()
                    .get(span.start..span.end())
                    .expect("every span lies within the rendered text");
                rebuilt.push_str(slice);
                offset = span.end();
            }
            assert_eq!(
                offset,
                rendered.text().len(),
                "spans must reach the end of the output (width {width})"
            );
            assert_eq!(
                rebuilt,
                rendered.text(),
                "concatenated spans must reproduce the bytes exactly (width {width})"
            );
        }
    }
}

#[test]
fn renderer_minted_bytes_are_distinguishable_from_consumer_vocabulary() {
    let rendered = render(&mixed_document(), 80);
    let minted: String = rendered
        .spans()
        .iter()
        .filter(|span| matches!(span.provenance, Provenance::Arrangement { key: None }))
        .filter_map(|span| rendered.text().get(span.start..span.end()))
        .collect();
    assert!(
        !minted.is_empty(),
        "the renderer does mint layout, and it must be visible in the map"
    );
    assert!(
        minted
            .chars()
            .all(|character| character.is_ascii_whitespace() || "=|~[]\"().".contains(character)),
        "weft mints whitespace and structural glyphs only, never words — found {minted:?}"
    );
}

#[test]
fn output_is_printable_ascii() {
    for width in SWEEP {
        for document in [wrappable_document(), mixed_document()] {
            let rendered = render(&document, width);
            for byte in rendered.text().bytes() {
                assert!(
                    byte == b'\n' || (0x20..=0x7e).contains(&byte),
                    "output must be printable ASCII or a newline; found {byte:#04x} at width {width}"
                );
            }
        }
    }
}

#[test]
fn rendering_is_deterministic() {
    for width in [40usize, 80, 137] {
        let document = mixed_document();
        let first = render(&document, width);
        let second = render(&document, width);
        assert_eq!(
            first, second,
            "layout is a pure function of (tree, width): text and spans alike"
        );
    }
}

#[test]
fn wrappable_prose_never_exceeds_the_width() {
    for width in SWEEP {
        let rendered = render(&wrappable_document(), width);
        for line in lines_of(&rendered) {
            assert!(
                line.len() <= width,
                "prose whose every word fits must never overrun: {line:?} at width {width}"
            );
        }
    }
}

#[test]
fn wrapping_is_greedy_and_never_breaks_early() {
    for width in SWEEP {
        let rendered = render(&wrappable_document(), width);
        let lines = lines_of(&rendered);
        for (index, line) in lines.iter().enumerate() {
            let Some(next) = lines.get(index + 1) else {
                continue;
            };
            let Some(first_word) = next.split_whitespace().next() else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            assert!(
                line.len() + 1 + first_word.len() > width,
                "a break that could have been avoided is a wasted line: {line:?} then {first_word:?} at width {width}"
            );
        }
    }
}

#[test]
fn an_over_wide_word_overruns_rather_than_being_split() {
    let giant = "certsync.oracle.sh:31@org.foob.Certs:/etc/nginx/certs@synced";
    let document = Document::new(vec![prose(vec![text("and "), value(giant)])]);
    let rendered = render(&document, 20);
    assert!(
        rendered.text().contains(giant),
        "content is never dropped or split to satisfy a width; omission must be a visible act"
    );
    assert!(
        lines_of(&rendered).iter().any(|line| line.len() > 20),
        "the over-wide word is expected to overrun — that is the named exception"
    );
}

#[test]
fn literal_code_lines_stay_byte_honest() {
    let line = "ufw status verbose | grep -q \"$1\"  : org.ufw.Firewall:\"$1\"@allowed";
    let document = Document::new(vec![Node::new(NodeKind::Code(CodeBlock {
        mode: Literalness::Literal,
        locus: None,
        lines: vec![CodeLine {
            gutter: None,
            content: vec![source(line)],
        }],
    }))]);
    let rendered = render(&document, 30);
    assert_eq!(
        rendered.text().trim_end(),
        line,
        "a literal excerpt is never rewrapped: implying a break the source lacks would be a lie about the bytes"
    );
}

#[test]
fn a_descriptive_block_is_always_marked_non_runnable() {
    let document = Document::new(vec![Node::new(NodeKind::Code(CodeBlock {
        mode: Literalness::Descriptive,
        locus: None,
        lines: vec![CodeLine {
            gutter: None,
            content: vec![source("hork tune ... --profile web")],
        }],
    }))]);
    let rendered = render(&document, 80);
    assert!(
        rendered.text().starts_with("~ "),
        "display-sh must never masquerade as runnable, gutter or no gutter: {:?}",
        rendered.text()
    );
}

#[test]
fn a_right_reservation_narrows_only_the_lines_it_covers() {
    let document = Document::new(vec![prose(vec![text(
        "one two three four five six seven eight nine ten eleven twelve thirteen",
    )])]);
    let reserved_line = 1usize;
    let frame = Frame::of_width(Width::new(40)).reserving(Reservation {
        side: Side::Right,
        first_line: reserved_line,
        line_count: 1,
        columns: 20,
    });
    let rendered = render_framed(&document, &frame);
    for (index, line) in lines_of(&rendered).iter().enumerate() {
        let allowed = if index == reserved_line { 20 } else { 40 };
        assert!(
            line.len() <= allowed,
            "line {index} must respect its own reservation, not the box's full width: {line:?}"
        );
    }
    assert!(
        lines_of(&rendered).len() > reserved_line + 1,
        "the reservation must narrow a line in the middle, not the last one — otherwise it proves nothing about per-line geometry"
    );
}

#[test]
fn a_zero_width_box_still_renders() {
    let rendered = render(&wrappable_document(), 0);
    assert_eq!(Width::new(0).columns(), Width::MINIMUM);
    assert!(
        !rendered.text().is_empty(),
        "a degenerate width must clamp and render, never divide by zero or refuse"
    );
}
