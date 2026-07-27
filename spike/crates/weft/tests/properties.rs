//! The properties that make weft trustworthy enough to build on.
//!
//! Goldens show what the output looks like; these pin what is true of it at
//! every width. Each test states the invariant it exists to defend, because an
//! invariant nobody can name is one nobody will preserve.

use weft::{
    CodeBlock, CodeCell, CodeLine, Document, Frame, Instance, LabeledRow, Literalness, Node,
    NodeKind, Paragraph, Payload, Provenance, Quoting, Rendered, Reservation, Run, Section, Side,
    SpeakerRow, Width, render, render_framed,
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
                table: None,
                gutter: Some(value("*")),
                speaker: vec![value("ufw.oracle.sh:44")],
                verb: Some(vec![value("reported")]),
                payload: Payload {
                    quoting: Quoting::Quoted,
                    runs: vec![value("Firewall:443/tcp@allowed")],
                    trailer: vec![text(" (ran 09:13:51, rc 0)")],
                },
                attachments: vec![Node::new(NodeKind::Code(CodeBlock {
                    table: None,
                    mode: Literalness::Literal,
                    locus: Some(vec![value("ufw.oracle.sh, as-written:")]),
                    lines: vec![CodeLine {
                        gutter: Some(value("44")),
                        cells: vec![CodeCell::new(vec![source(
                            "ufw status verbose | grep -q \"$1\"  : org.ufw.Firewall:\"$1\"@allowed",
                        )])],
                    }],
                }))],
            })),
            Node::new(NodeKind::Labeled(LabeledRow {
                table: None,
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

fn labeled(table: Option<Key>, label: &str, body: &str) -> Node<Key> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table,
        label: vec![value(label)],
        body: vec![text(body)],
        attachments: Vec::new(),
    }))
}

/// A shallow row and two rows buried in the branches of a join, all naming one
/// table — the shape the named table exists for, and the one no structural rule
/// can relate.
fn table_across_a_join() -> Document<Key> {
    let steps = Some(Key::Row("steps"));
    Document::new(vec![
        labeled(steps, "suspect:", "the claim is the only unmeasured link"),
        Node::new(NodeKind::Join(weft::Join {
            branches: vec![
                weft::Branch {
                    connective: None,
                    nodes: vec![labeled(steps, "fix:", "widen the claim")],
                },
                weft::Branch {
                    connective: Some(vec![value("OR")]),
                    nodes: vec![labeled(steps, "fix:", "re plan without the flag")],
                },
            ],
            restatement: None,
        })),
        labeled(steps, "review:", "every link unselected"),
    ])
}

/// The column a labelled row's body starts in, for every line that carries one
/// of the table's labels.
fn body_columns(rendered: &Rendered<Key>, labels: &[&str]) -> Vec<usize> {
    lines_of(rendered)
        .iter()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let label = labels.iter().find(|label| trimmed.starts_with(**label))?;
            let indent = line.len().saturating_sub(trimmed.len());
            let rest = trimmed.get(label.len()..)?;
            let gap = rest.len().saturating_sub(rest.trim_start().len());
            (!rest.trim().is_empty())
                .then_some(indent.saturating_add(label.len()).saturating_add(gap))
        })
        .collect()
}

const STEP_LABELS: &[&str] = &["suspect:", "fix:", "review:"];

#[test]
fn a_named_table_squares_up_members_that_are_not_siblings() {
    let rendered = render(&table_across_a_join(), 80);
    let columns = body_columns(&rendered, STEP_LABELS);
    assert_eq!(
        columns.len(),
        4,
        "all four labelled rows must render on a line of their own: {:?}",
        rendered.text()
    );
    assert!(
        columns.windows(2).all(|pair| pair[0] == pair[1]),
        "a table shares its whole prefix, so every member's body starts in ONE column; got {columns:?} in\n{}",
        rendered.text()
    );
}

/// At 30 columns the deepest member cannot afford a hanging body, which is what
/// forces the whole table to stack (`28F:rul-table-degrades-whole`).
#[test]
fn a_table_degrades_as_one_unit() {
    let rendered = render(&table_across_a_join(), 30);
    let hanging = body_columns(&rendered, STEP_LABELS);
    assert!(
        hanging.is_empty(),
        "one narrow member stacks the WHOLE table: a table where some rows hang and \
         others stack reads as broken, not as adaptive; got\n{}",
        rendered.text()
    );
}

/// The measure walk and the layout walk must visit the same members, in the same
/// order, at the same insets. A debug assertion checks every member; this drives
/// it over every container the vocabulary has, at every swept width, so a walk
/// that stops mirroring its twin fails loudly instead of shifting a column.
#[test]
fn the_layout_walk_agrees_with_the_measure_walk_everywhere() {
    let nested = Document::new(vec![
        Node::new(NodeKind::Section(Section {
            header: vec![value("ANALYSIS")],
            counts: None,
            body: vec![Node::new(NodeKind::Banner(weft::Banner {
                headline: vec![value("receipt:")],
                body: vec![
                    mixed_document().nodes.remove(0),
                    table_across_a_join().nodes.remove(1),
                ],
            }))],
        })),
        table_across_a_join().nodes.remove(0),
    ]);
    for width in SWEEP {
        let rendered = render(&nested, width);
        assert!(
            !rendered.text().is_empty(),
            "the nested document must render at width {width}"
        );
    }
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

/// A word assembled from several runs is still ONE word. A consumer that
/// interleaves a computed value into a line of its own prose hands the wrapper
/// three runs where the reader sees one token, and a break at that seam would be
/// a line-break the consumer never wrote.
#[test]
fn a_word_assembled_from_several_runs_never_breaks_at_a_seam() {
    for width in SWEEP {
        let document = Document::new(vec![prose(vec![
            text("the site at "),
            text("book.sh:"),
            value("142"),
            text(":cmdsub"),
            text(" ran because nothing could be said about it"),
        ])]);
        let rendered = render(&document, width);
        assert!(
            rendered.text().contains("book.sh:142:cmdsub"),
            "a run boundary is not a break point at width {width}: {:?}",
            rendered.text()
        );
    }
}

#[test]
fn literal_code_lines_stay_byte_honest() {
    let line = "ufw status verbose | grep -q \"$1\"  : org.ufw.Firewall:\"$1\"@allowed";
    let document = Document::new(vec![Node::new(NodeKind::Code(CodeBlock {
        table: None,
        mode: Literalness::Literal,
        locus: None,
        lines: vec![CodeLine {
            gutter: None,
            cells: vec![CodeCell::new(vec![source(line)])],
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
        table: None,
        mode: Literalness::Descriptive,
        locus: None,
        lines: vec![CodeLine {
            gutter: None,
            cells: vec![CodeCell::new(vec![source("hork tune ... --profile web")])],
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
