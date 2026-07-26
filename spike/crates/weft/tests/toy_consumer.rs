//! A toy consumer, and the goldens that make weft's output inspectable.
//!
//! The first document hand-builds the ANALYSIS and NEXT STEPS blocks of the
//! `28G` flagship strawman — the naked-trust case — using nothing but the
//! generic vocabulary. That is the point of the exercise: if the shapes a real
//! decision-report needs cannot be expressed without weft learning a single
//! domain word, the firewall is not real. Where the rendered layout diverges
//! from the strawman's hand-drawn one, the strawman is the design target and
//! the divergence is a flag, not a silent absorption — the strawman's own
//! header says its layout is illustrative and that the renderer owns layout.
//!
//! The second document exercises every remaining node kind, so no part of the
//! vocabulary is carried without ever having been rendered.
//!
//! Both are goldened at 80 columns and at 40. Forty is the acceptance
//! constraint: the surface must stay meaningful at a brutal width, degrading by
//! wrapping and by stacking columns, never by dropping content.

#![expect(
    clippy::expect_used,
    reason = "golden-file plumbing in a test helper: a missing or unwritable fixture must fail the trial loudly"
)]

use std::path::PathBuf;
use weft::{
    Banner, Branch, CodeBlock, CodeCell, CodeLine, Document, Instance, Join, LabeledRow,
    Literalness, Node, NodeKind, Paragraph, Payload, Placement, PointerLine, Quoting, Run, Section,
    SpeakerRow, Truncation, render,
};

/// One opaque key namespace covering every identity the consumer needs.
///
/// Weft never inspects these, so a single enum spanning rows, fields, params
/// and sources is the ordinary shape — keeping the namespaces apart is the
/// consumer's business, not the renderer's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Key {
    Row(&'static str),
    Field(&'static str),
    Param(&'static str),
    Source(&'static str),
}

fn prose_text(text: &str, row: &'static str, field: &'static str) -> Run<Key> {
    Run::template(text, Key::Row(row), Key::Field(field), Instance(0))
}

fn value(text: &str, row: &'static str, name: &'static str) -> Run<Key> {
    Run::param(text, Key::Row(row), Key::Param(name), Instance(0))
}

fn source(text: &str, file: &'static str) -> Run<Key> {
    Run::foreign(text, Key::Source(file))
}

fn word(text: &str, row: &'static str) -> Run<Key> {
    Run::arrangement(text, Key::Row(row))
}

fn prose(runs: Vec<Run<Key>>) -> Node<Key> {
    Node::new(NodeKind::Prose(Paragraph { runs }))
}

fn labeled(label: &'static str, body: Vec<Run<Key>>) -> Node<Key> {
    Node::new(NodeKind::Labeled(LabeledRow {
        label: vec![word(label, "label")],
        body,
        attachments: Vec::new(),
    }))
}

fn quoted_row(
    mark: &'static str,
    speaker: &'static str,
    verb: &'static str,
    said: &str,
    trailer: Vec<Run<Key>>,
    attachments: Vec<Node<Key>>,
) -> Node<Key> {
    Node::new(NodeKind::Speaker(SpeakerRow {
        gutter: Some(word(mark, "mark")),
        speaker: vec![value(speaker, "chain", "locus")],
        verb: Some(vec![word(verb, "tier")]),
        payload: Payload {
            quoting: Quoting::Quoted,
            runs: vec![value(said, "chain", "coordinate")],
            trailer,
        },
        attachments,
    }))
}

fn analysis_section() -> Node<Key> {
    let claim_attachments = vec![
        prose(vec![prose_text(
            "This speaks for everything it did NOT measure. Fact derived from as-written contract only; no run can back it:",
            "chain",
            "unmeasured-gloss",
        )]),
        Node::new(NodeKind::Code(CodeBlock {
            mode: Literalness::Literal,
            locus: Some(vec![value(
                "certsync.oracle.sh, as-written:",
                "excerpt",
                "locus",
            )]),
            lines: vec![
                CodeLine {
                    gutter: Some(value("30", "excerpt", "line")),
                    cells: vec![CodeCell::new(vec![source(
                        "# surveyed 2026-05: cert store only.",
                        "certsync.oracle.sh",
                    )])],
                },
                CodeLine {
                    gutter: Some(value("31", "excerpt", "line")),
                    cells: vec![CodeCell::new(vec![source(
                        "push) printf '%s\\n' \"$1\"  : disturbs org.foob.Certs ;;",
                        "certsync.oracle.sh",
                    )])],
                },
            ],
        })),
    ];

    Node::new(NodeKind::Section(Section {
        header: vec![word("ANALYSIS", "section.analysis")],
        counts: None,
        body: vec![
            prose(vec![
                value("9|systemctl", "opener", "site"),
                prose_text(
                    " being skipped stood on all of the following, together:",
                    "opener",
                    "message",
                ),
            ]),
            quoted_row(
                "*",
                "service.oracle.sh:12",
                "reported",
                "Package:nginx@{enabled,active}",
                vec![value(" (ran 01:59:52, rc 0)", "chain", "event")],
                Vec::new(),
            ),
            quoted_row(
                "*",
                "service.oracle.sh:9",
                "vouches",
                "re-running an already-@active unit adds nothing worth running",
                Vec::new(),
                Vec::new(),
            ),
            quoted_row(
                "!",
                "certsync.oracle.sh:31",
                "claims",
                "`certsync push` touches AT MOST @cert state",
                Vec::new(),
                claim_attachments,
            ),
            labeled(
                "so:",
                vec![
                    prose_text("nothing ", "closer", "message"),
                    value("8|certsync", "closer", "wall"),
                    prose_text(" claims to touch overlaps what ", "closer", "message"),
                    value("9|systemctl", "closer", "site"),
                    prose_text(
                        "'s skip was reported on; the skip stood.",
                        "closer",
                        "message",
                    ),
                ],
            ),
        ],
    }))
}

fn next_steps_section() -> Node<Key> {
    let alternatives = Node::new(NodeKind::Join(Join {
        branches: vec![
            Branch {
                connective: None,
                nodes: vec![labeled(
                    "fix:",
                    vec![
                        value("certsync.oracle.sh:31", "fix", "target"),
                        prose_text(" (widen the claim; every consumer heals)", "fix", "message"),
                    ],
                )],
            },
            Branch {
                connective: Some(vec![word("OR", "connective.alternative")]),
                nodes: vec![labeled(
                    "fix:",
                    vec![
                        value("`dorc plan web.sh web1`", "fix", "target"),
                        prose_text(
                            " (run without the risk flag, to distrust oracle-authored claims)",
                            "fix",
                            "message",
                        ),
                    ],
                )],
            },
        ],
        restatement: None,
    }));

    Node::new(NodeKind::Section(Section {
        header: vec![word("NEXT STEPS", "section.next-steps")],
        counts: None,
        body: vec![
            prose(vec![
                prose_text("if ", "steps", "opener"),
                value("9|systemctl", "steps", "site"),
                prose_text(" SHOULD have run:", "steps", "opener"),
            ]),
            labeled(
                "suspect:",
                vec![prose_text(
                    "the 8|certsync claim -- the only link speaking for unmeasured state. if `certsync push` also touches @enabled/@active, it is what wrongly kept 9|systemctl out.",
                    "suspect",
                    "message",
                )],
            ),
            alternatives,
            labeled(
                "verify:",
                vec![
                    value("`dorc plan --why web.sh web1`", "verify", "target"),
                    prose_text(
                        " (re-measures under plan's standing consent, and leaves you one step from applying.)",
                        "verify",
                        "message",
                    ),
                ],
            ),
            labeled(
                "review:",
                vec![
                    value("`dorc why web.sh:9 --all`", "review", "target"),
                    prose_text(" (every link, unselected, exhaustive)", "review", "message"),
                ],
            )
            .summarizable(),
        ],
    }))
}

fn fire_morning() -> Document<Key> {
    Document::new(vec![analysis_section(), next_steps_section()])
}

/// A second document whose only job is to render every node kind the first one
/// does not reach, so nothing in the vocabulary ships unrendered.
fn vocabulary_sampler() -> Document<Key> {
    Document::new(vec![receipt_banner(), improvements_section()])
}

fn receipt_banner() -> Node<Key> {
    Node::new(NodeKind::Banner(Banner {
        headline: vec![
            word("receipt:", "banner.receipt"),
            value(
                " apply 2026-07-25 02:00:37, host web1, trigger cron",
                "banner",
                "identity",
            ),
        ],
        body: vec![
            labeled(
                "oracles:",
                vec![value(
                    "certsync 1.2, service 2.0, + base library",
                    "banner",
                    "inventory",
                )],
            ),
            labeled(
                "plan:",
                vec![value("2 ran, 1 guarded, 5 skipped", "banner", "tally")],
            ),
        ],
    }))
}

/// An excerpt whose trailing comments form their own aligned column.
///
/// The box model has to reach *into* code, not stop at it: these comments are a
/// column that squares up across lines and will later carry its own styling.
/// Splitting the lines into cells is what licenses the alignment padding — the
/// single-cell excerpts elsewhere in this file stay byte-untouched.
fn annotated_excerpt() -> Node<Key> {
    Node::new(NodeKind::Code(CodeBlock {
        mode: Literalness::Literal,
        locus: None,
        lines: vec![
            CodeLine {
                gutter: Some(value("3", "excerpt", "line")),
                cells: vec![
                    CodeCell::new(vec![source("PORT=443", "web.sh")]),
                    CodeCell::new(vec![source("# the admin's own default", "web.sh")]),
                ],
            },
            CodeLine {
                gutter: Some(value("23", "excerpt", "line")),
                cells: vec![
                    CodeCell::new(vec![source("ufw allow \"$PORT\"/tcp", "web.sh")]),
                    CodeCell::new(vec![source("# guarded on every apply", "web.sh")]),
                ],
            },
        ],
    }))
}

fn improvements_section() -> Node<Key> {
    Node::new(NodeKind::Section(Section {
        header: vec![word("IMPROVEMENTS", "section.improvements")],
        counts: Some(vec![value("2", "section", "count")]),
        body: vec![
            Node::new(NodeKind::Code(CodeBlock {
                mode: Literalness::Descriptive,
                locus: Some(vec![value("web.sh, described:", "excerpt", "locus")]),
                lines: vec![CodeLine {
                    gutter: Some(value("12", "excerpt", "line")),
                    cells: vec![CodeCell::new(vec![source(
                        "hork tune --profile web ... and the rest of a very long invocation that must wrap",
                        "web.sh",
                    )])],
                }],
            })),
            Node::new(NodeKind::Code(CodeBlock {
                mode: Literalness::Formatted,
                locus: None,
                lines: vec![
                    CodeLine {
                        gutter: None,
                        cells: vec![CodeCell::new(vec![source(
                            "( ufw__is_converged allow \"$PORT\"/tcp ) \\",
                            "guard",
                        )])],
                    },
                    CodeLine {
                        gutter: None,
                        cells: vec![CodeCell::new(vec![source(
                            "   || ufw allow \"$PORT\"/tcp",
                            "guard",
                        )])],
                    },
                ],
            })),
            annotated_excerpt(),
            prose(vec![prose_text(
                "a convergence check for hork skips lines 12 and 13 whenever hork is converged",
                "improvement",
                "message",
            )]),
            Node::new(NodeKind::Pointer(PointerLine {
                placement: Placement::Trailing,
                target: vec![value("(dorc teach walls)", "pointer", "command")],
            })),
            Node::new(NodeKind::Join(Join {
                branches: vec![
                    Branch {
                        connective: Some(vec![word("both", "connective.join")]),
                        nodes: vec![prose(vec![prose_text(
                            "hork is undescribed",
                            "branch",
                            "message",
                        )])],
                    },
                    Branch {
                        connective: Some(vec![word("and", "connective.join")]),
                        nodes: vec![prose(vec![prose_text(
                            "corp-agent is undescribed",
                            "branch",
                            "message",
                        )])],
                    },
                ],
                restatement: Some(Paragraph {
                    runs: vec![prose_text(
                        "so both walls stand between the report and this line's turn.",
                        "restatement",
                        "message",
                    )],
                }),
            })),
            Node::new(NodeKind::Truncation(Truncation {
                note: vec![value("3 further links (--all)", "truncation", "note")],
            }))
            .summarizable(),
            Node::new(NodeKind::Pointer(PointerLine {
                placement: Placement::Standalone,
                target: vec![value("(dorc why web.sh:13)", "pointer", "command")],
            })),
        ],
    }))
}

fn check_golden(name: &str, rendered: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(name);
    if std::env::var_os("WEFT_BLESS").is_some() {
        let parent = path.parent().expect("golden path has a parent");
        std::fs::create_dir_all(parent).expect("create the golden directory");
        std::fs::write(&path, rendered).expect("write the golden");
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(
        rendered, expected,
        "golden `{name}` drifted; inspect the diff, then re-bless with WEFT_BLESS=1"
    );
}

#[test]
fn fire_morning_renders_at_eighty_columns() {
    check_golden("fire-morning-80.txt", render(&fire_morning(), 80).text());
}

#[test]
fn fire_morning_stays_meaningful_at_forty_columns() {
    check_golden("fire-morning-40.txt", render(&fire_morning(), 40).text());
}

#[test]
fn vocabulary_sampler_renders_at_eighty_columns() {
    check_golden("sampler-80.txt", render(&vocabulary_sampler(), 80).text());
}

#[test]
fn vocabulary_sampler_renders_at_forty_columns() {
    check_golden("sampler-40.txt", render(&vocabulary_sampler(), 40).text());
}
