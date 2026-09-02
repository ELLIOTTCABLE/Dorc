//! `dorc-cli` — the INTERNAL invocation surface, split out of the binary so a loom harness can
//! drive it (`289:rul-worldless-route-honest-trigger`; `291` §5a route W2).
//!
//! This is a **loom-harness seam, never a public API**. The crate is `publish = false`, and the
//! only reason it has a lib target at all is that a defining case for an invocation error must be
//! HONEST: `dorc-loom` runs [`parse_args_from`] over the case's own replay argv and refuses unless
//! the declared code really fires (`291:rule-worldless-route-refuses-on-mismatch`). Without that a
//! case's command would be decorative — nothing binding the argv it shows to the code it claims,
//! on the project's primary error-review surface (`288:rul-errors-human-authored-review-surface`).
//!
//! What lives here is exactly the PURE surface a harness can drive: usage text, the parsed shapes,
//! the parsers, and the DEGRADED `dorc why` render (`28H:prop-drifted-why-is-the-thin-driver`) —
//! the first why-surface report a loom case can carry editable prose for, since driving it needs
//! only a committed durable. Every I/O edge — reading files, resolving oracle dirs, the clock, the
//! git query, `std::env::args`, printing — stays in `main.rs` (`io-at-edges-only`); what crosses
//! the seam is already-resolved VALUES. Nothing here reads the world, which is what makes the
//! harness deterministic (`inv-determinism`).
//!
//! Do not grow this into a general-purpose library. If something here starts wanting a clock, a
//! file, or an environment read, it belongs on the other side of the seam.

// The dependency-graph fact the crypto crate carries, inherited by naming it as a PRODUCTION
// dependency: `age` reaches two major lines of several hashing crates through separate subtrees,
// which `-D warnings` then makes fatal. No version choice avoids it, and `deny.toml` sets
// `multiple-versions = "warn"` for the workspace. `expect`, so it warns once the duplication
// clears — the same shape `dorc-receipt-local` already carries for the same reason.
#![expect(
    clippy::multiple_crate_versions,
    reason = "a transitive-dependency fact; see the note above"
)]
#![forbid(unsafe_code)]

pub mod apply;
pub mod artifact;
pub mod bundle;
pub mod custody;
pub mod durable;
pub mod engine;
pub mod fixpoint;
pub mod kinds;
pub mod provenance;
pub mod receipt_edge;
pub mod recorded;
pub mod recorded_facts;
pub mod results;
pub mod snapshot;
pub mod source_comparison;
pub mod sourcing;
pub mod survival;
pub mod why;
pub mod why_json;
pub mod why_total;
pub mod world;

use dorc_aid::RenderCtx;
use dorc_aid::Severity;
use dorc_aid::arrangement::arrangement_text;
use dorc_aid::diag::{Diag, DiagCode};
use dorc_aid::said::{Said, WHY_VALUE_CAP};
use dorc_aid::weave::Face;
use weft::{Banner, Document, LabeledRow, Node, NodeKind, Paragraph};

use crate::artifact::ArtifactForm;

/// The invocation-error carrier (`288` §6). A plain [`Diag`]: the parsers hand the print seat the
/// same typed value every other surface carries, and boxing it would buy nothing but an indirection
/// on a path that runs at most once per process.
pub type InvocationError = Diag;

/// The arrangement slug of the one-line usage synopsis every invocation-error print seat
/// appends. Its words live in the arrangement registry, like every other user-facing string
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`); [`usage_text`] renders it.
pub const USAGE_ARRANGEMENT: &str = "cli-usage-synopsis";

/// The arrangement slug of the long help page `--help` prints
/// (`288:rul-help-text-is-loomable`). Its defining loom drives `$ dorc --help` and its
/// transcript IS the editing surface for the page's prose.
pub const HELP_ARRANGEMENT: &str = "cli-help-page";

/// The one-line usage synopsis, appended to argument errors by the print seats.
#[must_use]
pub fn usage_text(ctx: &RenderCtx<'_>) -> String {
    arrangement_text(ctx.arrangements(), USAGE_ARRANGEMENT, None)
}

/// One invocation error, WHOLE, as a stamped part stream: the `dorc: ` seat prefix, the rendered
/// diagnostic, and the usage synopsis beneath it.
///
/// The one seat, so the loom and the binary cannot disagree about the bytes an admin sees
/// (`28L:rul-editability-is-stamped-never-re-derived`). It also gives `cli-usage-synopsis` its
/// first FACE: the synopsis used to be read as plain text through [`usage_text`], which left the
/// one line every argument error prints with no editable home at all.
///
#[must_use]
pub fn invocation_error_parts(
    ctx: &RenderCtx<'_>,
    error: &InvocationError,
    interner: &dorc_core::Interner,
) -> dorc_aid::tagged::RenderParts {
    let mut parts = staged_invocation_parts("dorc", ctx, error, interner);
    for part in usage_parts(ctx).parts() {
        parts.push(part.clone());
    }
    parts
}

/// The `dorc-sh` shim's own invocation errors, under its terse `dorc-sh: ` framing.
///
/// No synopsis: the shim's whole error vocabulary is three lines and one of them IS its usage, so
/// appending `dorc`'s would name a command the reader did not run.
#[must_use]
pub fn shim_error_parts(
    ctx: &RenderCtx<'_>,
    error: &InvocationError,
    interner: &dorc_core::Interner,
) -> dorc_aid::tagged::RenderParts {
    staged_invocation_parts("dorc-sh", ctx, error, interner)
}

/// The diagnostic produced when `dorc-sh` receives no script.
#[must_use]
pub fn shim_usage_error() -> InvocationError {
    Diag::new_spanless_site(DiagCode::DorcShUsage(dorc_aid::diag::DorcShUsage))
}

/// `stage` is the seat's own prefix word — not a catalog register, and never editable.
fn staged_invocation_parts(
    stage: &str,
    ctx: &RenderCtx<'_>,
    error: &InvocationError,
    interner: &dorc_core::Interner,
) -> dorc_aid::tagged::RenderParts {
    dorc_aid::diag::render_staged_cli_parts(stage, ctx, error, "", "", interner)
}

/// One registry chrome LINE as a stamped part stream, its computed values interleaved.
///
/// Stamped but NOT laid out: these lines go to stderr exactly as long as they are, so the bytes
/// stay `arrangement_sentence`'s by construction and only the attribution is added
/// (`aid::arrangement::push_arrangement_sentence`). That is what lets the binary and a loom case
/// read the same seat without moving a single production byte.
#[must_use]
pub fn chrome_line_parts(
    ctx: &RenderCtx<'_>,
    slug: &'static str,
    values: &[&str],
) -> dorc_aid::tagged::RenderParts {
    let mut parts = dorc_aid::tagged::RenderParts::new();
    dorc_aid::arrangement::push_arrangement_sentence(
        &mut parts,
        ctx.arrangements(),
        slug,
        None,
        values,
    );
    parts
}

/// The plan route's stderr ENVELOPE: the why-pointer, the plan-summary yardstick, and the decision
/// digest, in the order the binary prints them.
///
/// A loom case opting into this (`envelope: stderr`) is the only way these three lines have an
/// editable home: they are stderr chrome around an artifact, so no diagnostic case ever reaches
/// them and they sat registry-homed but faceless.
#[must_use]
pub fn plan_envelope_parts(
    ctx: &RenderCtx<'_>,
    world: &world::WhyWorld,
    book_name: &str,
) -> dorc_aid::tagged::RenderParts {
    let counts = world.disposition_counts();
    let digest = world.presented_plan_hex();
    let numbers = [
        counts.sites.to_string(),
        counts.elide.to_string(),
        counts.omit.to_string(),
        counts.guard.to_string(),
        counts.run.to_string(),
        world.may_alias_fires().to_string(),
    ];
    let summary: Vec<&str> = numbers.iter().map(String::as_str).collect();
    let mut parts = dorc_aid::tagged::RenderParts::new();
    for (slug, values) in [
        ("cli-why-pointer-line", vec![book_name]),
        ("cli-plan-summary-line", summary),
        ("cli-decision-digest-line", vec![digest.as_str()]),
    ] {
        for part in chrome_line_parts(ctx, slug, &values).parts() {
            parts.push(part.clone());
        }
        parts.push(dorc_aid::tagged::RenderPart::Arrangement {
            text: String::from("\n"),
            slug: "cli-envelope-break",
        });
    }
    parts
}

/// The usage synopsis as its own laid-out paragraph, span-stamped so an edit lands on its registry
/// entry.
fn usage_parts(ctx: &RenderCtx<'_>) -> dorc_aid::tagged::RenderParts {
    why_parts(vec![registry_paragraph(ctx, USAGE_ARRANGEMENT)], 0)
}

/// The long help (ack-1 + the cheap help-is-success item): `--help` prints this to STDOUT
/// and exits 0 (a help request is a success, not a usage error).
#[must_use]
pub fn help_text(ctx: &RenderCtx<'_>) -> String {
    help_parts(ctx).text()
}

/// Tagged help: computed labels stay immutable while descriptions retain row provenance.
#[must_use]
pub fn help_parts(ctx: &RenderCtx<'_>) -> dorc_aid::tagged::RenderParts {
    let paragraph = |slug| registry_paragraph(ctx, slug);
    let table = |name: &str, rows: &[HelpRow]| {
        rows.iter()
            .map(|row| {
                Node::new(NodeKind::Labeled(LabeledRow {
                    table: Some(Face::Table(name.to_owned())),
                    label: vec![dorc_aid::weave::value(
                        row.label,
                        "cli-help-label",
                        WHY_VALUE_CAP,
                    )],
                    body: Said::words(row.slug, &[]).runs(ctx, row.slug),
                    attachments: Vec::new(),
                }))
            })
            .collect::<Vec<_>>()
    };
    let mut nodes = vec![
        paragraph(HELP_ARRANGEMENT),
        paragraph("cli-help-usage"),
        paragraph("cli-help-modes-heading"),
    ];
    nodes.extend(table("cli-help-modes", HELP_MODE_ROWS));
    nodes.push(paragraph("cli-help-options-heading"));
    nodes.extend(table("cli-help-options", HELP_OPTION_ROWS));
    nodes.push(paragraph("cli-help-receipts-heading"));
    nodes.extend(table("cli-help-receipts", HELP_RECEIPT_ROWS));
    nodes.push(paragraph("cli-help-stdin"));
    nodes.push(paragraph("cli-help-exit-codes-heading"));
    nodes.extend(table("cli-help-exit-codes", HELP_EXIT_ROWS));
    nodes.push(paragraph("cli-help-lint-exit-codes-heading"));
    nodes.extend(table("cli-help-lint-exit-codes", HELP_LINT_EXIT_ROWS));
    dorc_aid::weave::to_render_parts(&weft::render_framed(&Document::new(nodes), ctx.frame()))
}

struct HelpRow {
    label: &'static str,
    slug: &'static str,
}

const HELP_MODE_ROWS: &[HelpRow] = &[
    HelpRow {
        label: "bundle",
        slug: "cli-help-mode-bundle",
    },
    HelpRow {
        label: "probe",
        slug: "cli-help-mode-probe",
    },
    HelpRow {
        label: "plan",
        slug: "cli-help-mode-plan",
    },
    HelpRow {
        label: "apply",
        slug: "cli-help-mode-apply",
    },
    HelpRow {
        label: "why [<addr>]",
        slug: "cli-help-mode-why",
    },
    HelpRow {
        label: "strip <file>",
        slug: "cli-help-mode-strip",
    },
    HelpRow {
        label: "lint <files>",
        slug: "cli-help-mode-lint",
    },
    HelpRow {
        label: "(none)",
        slug: "cli-help-mode-none",
    },
];

const HELP_OPTION_ROWS: &[HelpRow] = &[
    HelpRow {
        label: "<book.sh>",
        slug: "cli-help-option-book",
    },
    HelpRow {
        label: "--pre-source <sh>",
        slug: "cli-help-option-pre-source",
    },
    HelpRow {
        label: "--oracle-dir <dir>",
        slug: "cli-help-option-oracle-dir",
    },
    HelpRow {
        label: "--results <file>",
        slug: "cli-help-option-results",
    },
    HelpRow {
        label: "--risk-faultless-skips",
        slug: "cli-help-option-risk-faultless-skips",
    },
    HelpRow {
        label: "--receipts <folder>",
        slug: "cli-help-option-receipts",
    },
    HelpRow {
        label: "--no-receipt",
        slug: "cli-help-option-no-receipt",
    },
    HelpRow {
        label: "--receipt-last",
        slug: "cli-help-option-receipt-last",
    },
    HelpRow {
        label: "--receipt-id <id>",
        slug: "cli-help-option-receipt-id",
    },
    HelpRow {
        label: "--receipt <file>",
        slug: "cli-help-option-receipt",
    },
    HelpRow {
        label: "--all",
        slug: "cli-help-option-all",
    },
    HelpRow {
        label: "--json",
        slug: "cli-help-option-json",
    },
    HelpRow {
        label: "--debug-argv",
        slug: "cli-help-option-debug-argv",
    },
    HelpRow {
        label: "--help",
        slug: "cli-help-option-help",
    },
    HelpRow {
        label: "--version",
        slug: "cli-help-option-version",
    },
];

const HELP_RECEIPT_ROWS: &[HelpRow] = &[
    HelpRow {
        label: "sm Holds:",
        slug: "cli-help-receipt-holds",
    },
    HelpRow {
        label: "sm Kept",
        slug: "cli-help-receipt-kept",
    },
];

const HELP_EXIT_ROWS: &[HelpRow] = &[
    HelpRow {
        label: "0",
        slug: "cli-help-exit-success",
    },
    HelpRow {
        label: "2",
        slug: "cli-help-exit-usage",
    },
    HelpRow {
        label: "10",
        slug: "cli-help-exit-parse",
    },
];

const HELP_LINT_EXIT_ROWS: &[HelpRow] = &[
    HelpRow {
        label: "0",
        slug: "cli-help-lint-exit-clean",
    },
    HelpRow {
        label: "1",
        slug: "cli-help-lint-exit-findings",
    },
    HelpRow {
        label: "3",
        slug: "cli-help-lint-exit-operational",
    },
];

/// `dorc lint --list-sources` as a stamped part stream: one row per registered source, its name
/// and rung computed, its one-line description read from the registry as WORDS.
///
/// It used to be a `println!` loop reading the same rows as flat text, which left all eight
/// `lint-source-*` entries with no editable home — the row a reader can see and not change
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`). The listing is a weft table, so the
/// column stop is measured rather than a hardcoded pad.
#[must_use]
pub fn lint_sources_parts(ctx: &RenderCtx<'_>) -> dorc_aid::tagged::RenderParts {
    let rows = dorc_lint::list_sources()
        .into_iter()
        .map(|source| {
            Node::new(NodeKind::Labeled(LabeledRow {
                table: Some(Face::Table(LINT_SOURCES_TABLE.to_owned())),
                label: vec![
                    dorc_aid::weave::value(source.name, "lint-source-name", WHY_VALUE_CAP),
                    dorc_aid::weave::mark(" [", "lint-source-rung-open"),
                    dorc_aid::weave::value(source.rung, "lint-source-rung", WHY_VALUE_CAP),
                    dorc_aid::weave::mark("]", "lint-source-rung-close"),
                ],
                body: Said::words(source.describe_arrangement, &[])
                    .runs(ctx, source.describe_arrangement),
                attachments: Vec::new(),
            }))
        })
        .collect();
    why_parts(rows, 0)
}

/// The table every `--list-sources` row joins, so the descriptions square up as a block.
const LINT_SOURCES_TABLE: &str = "lint-sources";

/// What the arg-parse resolved to: an analysis run, or a help/version request (both of which
/// are successes printed to stdout, ack-1 help-is-success — never a usage error).
#[derive(Debug)]
pub enum Invocation {
    /// A normal analysis run with the parsed [`Args`].
    ///
    /// Boxed because [`Args`] is one field per flag by design and dwarfs every other variant;
    /// this enum is built and matched once per process, so the indirection is free and the
    /// alternative is every invocation carrying the analysis surface's footprint.
    Analyze(Box<Args>),
    /// `--help`: print [`help_text`] to stdout, exit 0.
    Help,
    /// `--version`: print the version to stdout, exit 0.
    Version,
    /// `dorc strip <file>`: the off-ramp cleaner (`27D` rider-dorc-sh-unbuilt / `274` §13). A
    /// NON-analysis invocation (like help/version) — it erases every dialect construct from one
    /// file and prints runnable stock sh to stdout. The path is the sole positional.
    Strip(String),
    /// `dorc lint <files…>`: the oracle-author doctor/lint grab-bag (`27R`). A NON-analysis
    /// invocation over the `dorc-lint` crate; contacts no hosts, ships no probes (`dir-lint-never-probes`).
    Lint(LintArgs),
}
/// Which user-facing behavioral mode of the core to drive (ui-A — a fair-shape CLI over
/// the core invocation modes, NOT flag-complete; ru-25). Each maps to one of the engine's
/// distinct surfaces; `RoundTrip` is the legacy bare-flag invocation the e2e harness drives
/// (kept so the corpus stays green without a harness rewrite — the least-disruptive path).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// `dorc bundle …`: print the pure multipart oracle-bundle projection. It reads the same
    /// authored snapshot as analysis and performs no host contact or plan edit.
    Bundle,
    /// `dorc probe …`: emit ONLY the read-only probe artifact (round-trip phase 1). Reads no
    /// stdin (there are no results yet — this is what you ship to the host to GET them).
    Probe,
    /// `dorc plan …`: the human-facing PREVIEW (ru-20 ui-3 / DESIGN approach-3 "still as a
    /// simple shell-script"). Emits the eliding-apply to stdout AND doubly-emits the why-lens
    /// + diagnostics to stderr — the cited-sections render surface.
    Plan,
    /// `dorc apply …`: the byte-floored, receipt-free shippable artifact (rec-1). Emits the
    /// SAME apply bytes as `plan` to stdout, but the stderr render surface carries only the
    /// error floor + the decision-digest (no why-lens, no advisory notes).
    Apply,
    /// No mode token: the legacy round-trip (probe THEN apply on stdout, full disclosure on
    /// stderr). The exact shape the e2e runner drives — preserved verbatim (tc-subcommand-shape).
    RoundTrip,
    /// `dorc why [<address>] …`: the WHY-query surface (ack-2). NOT an artifact-producing
    /// invocation — its report goes to STDOUT (help/version/why are their own non-analysis
    /// invocations, per the fences). Runs the full pipeline (it reports on the CURRENT run's
    /// dispositions, so it consumes stdin results like `plan`), then prints a source-line-keyed
    /// report (rul24-lineno-identity) instead of an artifact: bare ⇒ the run's PROBLEMS; a
    /// `book.sh:N` / content address ⇒ that site's cause-chain. Emits no artifact, no digest.
    Why,
}

/// The parsed analysis invocation.
#[derive(Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "this struct IS the argv: one field per flag the parser accepts, and a flag is a bool. Bundling them behind newtypes or an enum would re-spell the command line one layer down without making any state less representable"
)]
pub struct Args {
    /// Which behavioral mode of the core to drive.
    pub mode: Mode,
    /// The ONE main book this target's plan is of — a positional (`dorc plan book.sh`, the day-one
    /// ergonomic) or `--book=PATH` (`30I:rul-one-main-book-per-target`). `-` names stdin.
    ///
    /// Exactly one, never a list: a second main-book operand is a separate PROGRAM, and reading it
    /// as source composition is the concatenation `30I` retired. Sharing one shell environment is
    /// spelled `--pre-source`.
    pub book: Option<String>,
    /// `--pre-source PATH`, repeatable: ordinary `.` preludes, run in CLI occurrence order
    /// immediately before the main book body (`30I:rul-pre-source-is-dot-prelude`). `-` names
    /// stdin.
    pub pre_sources: Vec<String>,
    /// `--oracle-dir DIR` (ack-6): pre-source every `*.oracle.sh` in DIR (glob-sorted,
    /// deterministic), repeatable — the bulk form.
    pub oracle_dirs: Vec<String>,
    /// `--results FILE`: read the probe results from FILE; `--results -` reads them from stdin.
    /// Absent, the run measures NOTHING and every site runs -- no flag acquires stdin implicitly
    /// (`30I:owed-no-flag-defaults-to-stdin`).
    pub results: Option<String>,
    /// `--debug-argv` (gate-5 / cm-2): emit the engine's per-site resolved argv to stderr,
    /// then proceed normally — a cli-edge readout the e2e argv-echo differential consumes.
    pub debug_argv: bool,
    /// `--risk-faultless-skips` (rul24-mode-gate): opt into the survival tier — a converged line
    /// may ELIDE past a RUNNING wall when the wall's authored `touches()` footprint is disjoint
    /// from the line's fact's backing (Stage 2, the golden hill). DEFAULT OFF; not recommended
    /// by hints/docs beyond noting availability. Honest framing (24A §1a-addendum): marketing at
    /// best (the admin chose the danger), theatre at worst (everyone enables it) — demanded
    /// anyway as the non-vacuous CYA. When off, the footprints are never even lifted (TC-1).
    pub risk_faultless_skips: bool,
    /// The optional `dorc why <address>` positional (ack-2): `book.sh:N` (a source line-address —
    /// rul24-lineno-identity), or free content to substring-match a command; `None` ⇒ the
    /// unargumented default (report the CURRENT run's problems). Only meaningful for [`Mode::Why`].
    pub why_address: Option<String>,
    /// The escalation dial (`27C` §1 axis 2 — the ternary admin surface): `--no-probe-escalation` /
    /// `--probe-escalation` (default) / `--escalate-any-probe`. Gates whether oracle code may
    /// context-shift under a wrapped site (`27C:rul-two-axis-escalation-consent`).
    pub dial: dorc_core::EscalationDial,
    /// The connection's mechanical capability (`27C` §1 axis 1) — a HOST FACT the cli edge would
    /// probe in reality (`hostsim`-injected in DST). `--probe-capability=root|nopasswd|degraded`
    /// stands in for that probe in the spike; defaults to `root`. The probe NEVER self-acquires.
    pub capability: dorc_core::Capability,
    /// `--receipts=FOLDER` (`30R:receipt-rooted-attention-and-cli`): the receipt STORE this run
    /// publishes into and `dorc why` looks up graph siblings in. Unset ⇒ the standard per-user
    /// store (`dorc_cli`'s caller resolves it), because the promise is zero-setup: `USER_STORY`
    /// has `dorc why` working "with nothing you had to set up beforehand", and a receipt nobody
    /// remembered to ask for is the only kind that exists on the bad morning.
    ///
    /// The folder is the store root EXACTLY — no `receipts-v1` component is appended beneath it —
    /// and it never moves the standard KEY root (`30Rd:controller-root-resolution`: V1 has no
    /// custom key-root surface). Orthogonal to the three root selectors below: it says WHICH
    /// store, they say which document within it.
    pub receipts: Option<String>,
    /// `--no-receipt`: write no receipt for this run.
    ///
    /// The escape hatch default-on owes: a receipt is host metadata written unprompted
    /// (`AID-NEEDS:law-receipts-are-sensitive`), so refusing one must be typeable. Per-invocation and
    /// subtractive-only, which is the shape `28D:pay-levers-are-subtractive` demands of anything in
    /// this family — there is no widening sibling and never will be.
    ///
    /// Refused outright under `dorc apply --host`, where the intent publication is the dispatch
    /// authority itself: the parser answers `apply-receipt-not-optional` rather than letting a
    /// subtractive lever ask for a bypass V1 does not have.
    pub no_receipt: bool,
    /// `--receipt-last`: derive the root from the store's newest recognized document.
    ///
    /// Also the no-selector DEFAULT for a receipt-reading `why`
    /// (`30Rd:store-enumeration-and-last-selection`); the flag survives as a spelling rather than a
    /// switch, because it is printed in committed transcripts and typed in muscle memory. A `why`
    /// flag, refusing on every other mode: a receipt answers what a PAST run decided, and feeding
    /// one into a mode that EMITS an artifact would let a recorded stream stand where a live
    /// measurement belongs.
    ///
    /// Derivation is cohort-then-collapse, never a tie-break: the maximum-order cohort is taken,
    /// members that are typed graph PREDECESSORS of another member collapse beneath it, and a sole
    /// surviving terminal root is the answer. Several incomparable terminals report ambiguity, and
    /// a damaged newest candidate never falls back to an older complete one.
    pub receipt_last: bool,
    /// `--receipt-id=ID`: the ONE document in the selected store carrying that identity.
    ///
    /// Retrieval, never a second ranking. The store offers exactly one DERIVED selection — its
    /// maximum-order cohort — and a second way to PREFER a candidate is what would reopen the
    /// fallback past a damaged newest one. An exact identity match prefers nothing: a document
    /// either carries it or it does not, and nothing is answered when none does.
    pub receipt_id: Option<String>,
    /// `--receipt=FILE`: one explicit report-only root document, named by path.
    ///
    /// Report-only in the strong sense (`30Ra:receipt-rooted-attention-and-selection`): naming a
    /// file mints no publication, trust, approval, or action, and grants no permission to discover
    /// a backend. `--receipts` stays orthogonal beside it, supplying the bounded store in which
    /// this root's typed siblings may be resolved.
    pub receipt_file: Option<String>,
    /// `--all`: the DEEPEST explanation register — every `dorc why` footer already points here, so
    /// the flag exists to make that pointer copy-paste-true (`28E` §7 held-placement-reread).
    ///
    /// DEPTH ONLY (`30R:receipt-rooted-attention-and-cli`). It never selects store entries: graph
    /// closure is automatic and question-directed, so there is no whole-store explanation mode and
    /// disconnected receipt DAGs never join one answer.
    ///
    /// What it carries today is the `[unnarrated: <class>]` census
    /// (`28E:prop-unnarrated-is-visible`): the aid plane fails toward narration, so a narrative
    /// class this run MINTED and no render CONSUMED is disclosed rather than silently omitted. The
    /// footer's fuller promise — every link, unselected, exhaustive — is not yet built, since the
    /// render does no link SELECTION to undo.
    pub all: bool,
    /// `--json`: the machine sibling of the receipt-rooted total surface (`30V` §5).
    ///
    /// A REGISTER rather than a different question: it serializes the same reconstruction and makes
    /// the same totality claim, with every withhold an explicit typed marker instead of an absent
    /// key. Version-unstable by open contract, which the envelope says in its own first field.
    ///
    /// Spelled `--json` rather than `--format=json` (`30Vd:tc-machine-format-flag-spelling`): the
    /// `dorc lint --format=jsonl` precedent is a lane with several machine formats, and this one has
    /// exactly two registers.
    pub json: bool,
    /// `--host DEST`: the ssh destination this run really contacts (`260` §6, at N=1).
    ///
    /// Absent, nothing is shipped anywhere and behaviour is exactly what it was before a
    /// transport existed — probe results arrive on stdin or via `--results`, and the emitted
    /// bytes are byte-identical. That is a standing regression fence, not a transitional state:
    /// the hostless path is what the golden corpus pins.
    ///
    /// Present, this is the sole source of the run's host identity. It is consumed VERBATIM as
    /// an ssh destination — an alias from the user's own ssh config is first-class, and Dorc
    /// never parses it (`260` §2).
    pub host: Option<String>,
    /// `--plan PATH`: the already-rendered artifact `apply --host` ships; `-` names stdin.
    /// Required for that mode -- there is no default (`30I:owed-no-flag-defaults-to-stdin`).
    ///
    /// Deliberately not the book positional. A remote apply consumes a PLAN the user has already
    /// read and consented to; letting it take a book would put build-and-apply in one breath,
    /// which is the one thing the plan→apply consent cut exists to prevent.
    pub plan: Option<String>,
    /// `--accept-new`: accept an unknown host key on first contact. Off by default; the default
    /// defers to OpenSSH's own `known_hosts` enforcement.
    pub accept_new: bool,
    /// `--ssh-config PATH`: ignore the user's ssh config and read only this file. Off by
    /// default, because bypassing their aliases, jump hosts and keys is the wrong default.
    pub ssh_config: Option<String>,
    /// `--connect-timeout SECS`: ceiling on establishing the connection (default 15).
    pub connect_timeout: Option<u64>,
    /// `--probe-timeout SECS`: wall-clock ceiling on a probe session (default 120). A probe is
    /// read-only, so bounding it costs nothing but a re-probe.
    pub probe_timeout: Option<u64>,
    /// `--apply-timeout SECS`: wall-clock ceiling on an apply session. UNSET by default and
    /// deliberately so — an apply is the user's real work, and killing one does not fail it, it
    /// mints Unknown. Opting in means accepting that outcome.
    pub apply_timeout: Option<u64>,
    /// `--artifact-dir DIR` (`30I` §7.5): the artifact set's own stream. Present, the run may
    /// materialize a dependency tree beside `plan.sh` and the emission planner is free to choose a
    /// multipart form; absent, stdout is the artifact stream and one stream means one flat plan
    /// (`30I:rul-piped-stdout-implies-one-flat-plan`).
    ///
    /// The directory is CONTROLLER-OWNED and is published atomically: a fresh staging directory
    /// receives every file, and only a complete set is moved into place, so a plan can never point
    /// at a sidecar from an earlier generation.
    pub artifact_dir: Option<String>,
    /// `--form <flattened|multipart|preserved-book-tree>` (`30I` §7.1): name a semantic emission
    /// form. Absent ⇒ `auto`, which picks the most flattened SAFE form for the stream posture and
    /// explains what it settled for. Named and unavailable ⇒ a pre-network REFUSAL: returning a
    /// different form than the one asked for is explicitly not builder latitude (`30I` §14).
    pub form: Option<ArtifactForm>,
    /// `--shim-dir=DIR` (`274` §5 / `27L` task-14 — the shim-materialization edge): DIR into which
    /// the entry-composed probe's per-run PATH shim files are written (the session-establishment I/O
    /// that lets a `sudo -n <inner-check>` resolve its guest across the exec boundary). A pure
    /// side-effect at the cli edge; stdout is unchanged. `None` ⇒ no materialization (a wrapper-free
    /// or already-answered run writes nothing — `empty-world-byte-identical`).
    pub shim_dir: Option<String>,
}

impl Args {
    /// Does this invocation answer from the stored receipt rather than from records handed to it?
    ///
    /// The surface fold (`28E:lean-why-is-whylog-reconciliation`, phased by `plans/28G` §1 W3):
    /// `dorc why` is receipt-reconciliation by DEFAULT -- "why did that happen" is the question
    /// people actually ask, and it is asked with nothing in hand. Records-from-argv survives as the
    /// harness/tooling posture, and it is now EXPLICIT: naming `--results` is what selects it.
    ///
    /// Deliberately not "is stdin a pipe": that would be an ambient read at a seat sworn off them
    /// (`io-at-edges-only`), it would make a CI `dorc why` silently answer a different question
    /// than an interactive one, and it would block on a terminal.
    #[must_use]
    pub const fn reads_the_receipt(&self) -> bool {
        reads_the_receipt(
            self.mode,
            self.names_a_receipt_root(),
            self.results.is_some(),
        )
    }

    /// Did this invocation name one of the three mutually-exclusive root selectors?
    #[must_use]
    pub const fn names_a_receipt_root(&self) -> bool {
        self.receipt_last || self.receipt_id.is_some() || self.receipt_file.is_some()
    }

    /// Which register the receipt-rooted answer is spelled in.
    #[must_use]
    pub const fn why_register(&self) -> engine::WhyRegister {
        if self.json {
            engine::WhyRegister::Json
        } else {
            engine::WhyRegister::Terminal
        }
    }

    /// Which root receipt this invocation's flags select.
    ///
    /// Three spellings, one root (`30R:receipt-rooted-attention-and-cli`). The graph closure the
    /// answer needs is derived from that root and is never a flag: `--all` changes explanation
    /// DEPTH and selects nothing, so there is no arm here for a whole-store union.
    #[must_use]
    pub fn receipt_root(&self) -> engine::ReceiptRoot<'_> {
        match (self.receipt_file.as_deref(), self.receipt_id.as_deref()) {
            (Some(path), _) => engine::ReceiptRoot::File(path),
            (None, Some(id)) => engine::ReceiptRoot::Id(id),
            // `--receipt-last` and the no-selector default are ONE answer, not two: a bare
            // `dorc why` asks about the last run, which is what the flag spells explicitly.
            (None, None) => engine::ReceiptRoot::Last,
        }
    }
}

/// Map parser-owned arguments and edge observations into parser-independent engine inputs.
#[must_use]
pub fn engine_options_from_args(
    args: &Args,
    stdout: artifact::StdoutPosture,
    artifact_directory: bool,
    durable: bool,
) -> engine::EngineOptions {
    engine::EngineOptions {
        mode: args.mode,
        analysis: engine::AnalysisOptions {
            survival: if args.risk_faultless_skips {
                engine::SurvivalPolicy::RiskAccepted
            } else {
                engine::SurvivalPolicy::HonestWalls
            },
            escalation: args.dial,
            capability: args.capability,
        },
        reporting: engine::ReportingOptions {
            why_address: args.why_address.clone(),
            why_depth: if args.all {
                engine::WhyDepth::All
            } else {
                engine::WhyDepth::Curated
            },
            argv_readout: if args.debug_argv {
                engine::ArgvReadout::Visible
            } else {
                engine::ArgvReadout::Hidden
            },
        },
        artifact: engine::ArtifactOptions {
            form: args.form,
            stdout,
            destination: engine::ArtifactDestinationShape::from_directory_requested(
                artifact_directory,
            ),
        },
        durable: if durable {
            engine::DurableOutput::Enabled
        } else {
            engine::DurableOutput::Disabled
        },
    }
}

/// [`Args::reads_the_receipt`] over the parts, so the parser can apply the same rule before it has
/// an `Args` to ask. Two spellings of this predicate would be two answers to "which surface am I".
///
/// `names_a_document` folds the three root selectors — `--receipt-last`, `--receipt-id` and
/// `--receipt` — because they answer the same question about the surface: an invocation naming any
/// of them is asking about something already written, whatever else it carries.
const fn reads_the_receipt(mode: Mode, names_a_document: bool, has_results: bool) -> bool {
    matches!(mode, Mode::Why) && (names_a_document || !has_results)
}

/// Every claimant this invocation puts on stdin, in argv order
/// (`30I` §2.5: stdin is a collapsed single resource).
///
/// EVERY claim is explicit, because `-` in filename position is the only way to acquire stdin
/// (`30I:rul-dash-is-stdin-in-any-filename-position` · `owed-no-flag-defaults-to-stdin`). The
/// records lane and `apply --host`'s artifact lane used to take it by DEFAULT and were declared
/// here so that a refusal could at least name them; retiring those defaults is what this list now
/// records, and it is why the mode, the receipt posture and the host no longer reach this seat.
fn stdin_claimants(
    book: Option<&str>,
    pre_sources: &[String],
    results: Option<&str>,
    plan: Option<&str>,
) -> Vec<&'static str> {
    let mut claims = Vec::new();
    if book == Some("-") {
        claims.push("the book");
    }
    if pre_sources.iter().any(|path| path == "-") {
        claims.push("--pre-source");
    }
    if results == Some("-") {
        claims.push("--results");
    }
    if plan == Some("-") {
        claims.push("--plan");
    }
    claims
}

#[expect(
    clippy::too_many_lines,
    reason = "one linear arg surface: the help/version pre-scan, the mode + why-address token, then the flag/positional loop with did-you-mean; splitting it would scatter the ONE parse"
)]
/// Resolve a whole `dorc` invocation from its argv (the process name already dropped). Total over
/// its input and free of I/O — the property the loom harness rides.
///
/// # Errors
/// Returns the typed invocation diagnostic a bad invocation produces (`288` §6).
#[expect(
    clippy::result_large_err,
    reason = "the Err is a full `Diag` because that is what the print seat renders; the whole family \n              fires at most once per process, so the cold-path size is not worth an indirection"
)]
pub fn parse_args_from(raw: Vec<String>) -> Result<Invocation, InvocationError> {
    // ack-1 help-is-success: `--help`/`--version` are stdout-and-exit-0 requests, not usage
    // errors, and they win even alongside a malformed flag (the conventional precedence).
    // Long-form only, like every other spike flag (`30I:rul-spike-has-no-short-options`).
    if raw.iter().any(|a| a == "--help") {
        return Ok(Invocation::Help);
    }
    if raw.iter().any(|a| a == "--version") {
        return Ok(Invocation::Version);
    }
    // `dorc strip <file>`: the off-ramp cleaner — a non-analysis invocation, handled before the
    // mode/flag machinery. Exactly one positional (the file to strip); no other flags apply.
    if raw.first().map(String::as_str) == Some("strip") {
        let path = raw.get(1).ok_or_else(|| {
            Diag::new_spanless_site(DiagCode::CliStripNeedsPath(
                dorc_aid::diag::CliStripNeedsPath,
            ))
        })?;
        if path.starts_with('-') {
            return Err(Diag::new_spanless_site(DiagCode::CliStripGotAFlag(
                dorc_aid::diag::CliStripGotAFlag { got: path.clone() },
            )));
        }
        return Ok(Invocation::Strip(path.clone()));
    }

    // `dorc lint`: a distinct arg surface (`27R` §5), handled before the analyze machinery like strip.
    if raw.first().map(String::as_str) == Some("lint") {
        return parse_lint_args(&raw);
    }

    let mut books: Vec<String> = Vec::new();
    let mut pre_sources = Vec::new();
    let mut oracle_dirs = Vec::new();
    let mut results: Option<String> = None;
    let mut debug_argv = false;
    let mut risk_faultless_skips = false;
    let mut why_address: Option<String> = None;
    let mut dial = dorc_core::EscalationDial::VouchedOnly;
    let mut capability = dorc_core::Capability::Root;
    let mut receipts: Option<String> = None;
    let mut receipt_last = false;
    let mut receipt_id: Option<String> = None;
    let mut receipt_file: Option<String> = None;
    let mut no_receipt = false;
    let mut all = false;
    let mut json = false;
    let mut shim_dir: Option<String> = None;
    let mut artifact_dir: Option<String> = None;
    let mut form: Option<ArtifactForm> = None;
    let mut host: Option<String> = None;
    let mut plan: Option<String> = None;
    let mut accept_new = false;
    let mut ssh_config: Option<String> = None;
    let mut connect_timeout: Option<u64> = None;
    let mut probe_timeout: Option<u64> = None;
    let mut apply_timeout: Option<u64> = None;
    let mut it = raw.into_iter().peekable();

    // A leading bare word (no `-` prefix) selects the mode. A near-miss (`pln`, `aply`) is a
    // did-you-mean, not a silent book (the recon's missing-suggestion hazard).
    let mode = match it.peek().map(String::as_str) {
        Some("probe") => {
            it.next();
            Mode::Probe
        }
        Some("bundle") => {
            it.next();
            Mode::Bundle
        }
        Some("plan") => {
            it.next();
            Mode::Plan
        }
        Some("apply") => {
            it.next();
            Mode::Apply
        }
        Some("why") => {
            it.next();
            // ack-2: `why` takes an OPTIONAL address positional — the next token, IF it is not a
            // flag (`book.sh:N` and content queries never start with `-`). Absent ⇒ the
            // unargumented default (the run's problems).
            if it.peek().is_some_and(|a| !a.starts_with('-')) {
                why_address = it.next();
            }
            Mode::Why
        }
        Some(w) if !w.starts_with('-') => {
            // A leading bare word that is NOT a known mode: if it is a NEAR-MISS of one, suggest it
            // (did-you-mean); otherwise it is a positional book (the round-trip default — the flag
            // loop below picks it up).
            if let Some(sugg) = nearest(w, &["probe", "plan", "apply", "why", "bundle", "strip"]) {
                return Err(Diag::new_spanless_site(DiagCode::CliUnknownMode(
                    dorc_aid::diag::CliUnknownMode {
                        mode: w.to_owned(),
                        suggestion: sugg.to_owned(),
                    },
                )));
            }
            Mode::RoundTrip
        }
        _ => Mode::RoundTrip,
    };

    while let Some(arg) = it.next() {
        if let Some(p) = arg.strip_prefix("--book=") {
            books.push(p.to_string());
        } else if arg == "--book" {
            books.push(
                it.next()
                    .ok_or_else(|| flag_needs_value("--book", "a path"))?,
            );
        } else if arg == "--pre-source" {
            pre_sources.push(
                it.next()
                    .ok_or_else(|| flag_needs_value("--pre-source", "a path"))?,
            );
        } else if let Some(p) = arg.strip_prefix("--pre-source=") {
            pre_sources.push(p.to_string());
        } else if let Some(p) = arg.strip_prefix("--oracle-dir=") {
            oracle_dirs.push(p.to_string());
        } else if arg == "--oracle-dir" {
            oracle_dirs.push(
                it.next()
                    .ok_or_else(|| flag_needs_value("--oracle-dir", "a directory"))?,
            );
        } else if let Some(p) = arg.strip_prefix("--results=") {
            results = Some(p.to_string());
        } else if arg == "--results" {
            results = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--results", "a path"))?,
            );
        } else if arg == "--debug-argv" {
            debug_argv = true;
        } else if arg == "--risk-faultless-skips" {
            risk_faultless_skips = true;
        } else if arg == "--no-probe-escalation" {
            dial = dorc_core::EscalationDial::NoEscalation;
        } else if arg == "--probe-escalation" {
            dial = dorc_core::EscalationDial::VouchedOnly;
        } else if arg == "--escalate-any-probe" {
            dial = dorc_core::EscalationDial::AnyProbe;
        } else if let Some(c) = arg.strip_prefix("--probe-capability=") {
            capability = match c {
                "root" => dorc_core::Capability::Root,
                "nopasswd" => dorc_core::Capability::NonRootNopasswd,
                "degraded" => dorc_core::Capability::Degraded,
                other => {
                    return Err(value_not_recognized(
                        "--probe-capability",
                        other,
                        "root|nopasswd|degraded",
                    ));
                }
            };
        } else if let Some(p) = arg.strip_prefix("--receipts=") {
            receipts = Some(p.to_string());
        } else if arg == "--receipts" {
            receipts = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--receipts", "a store folder"))?,
            );
        } else if arg == "--all" {
            all = true;
        } else if arg == "--json" {
            json = true;
        } else if arg == "--receipt-last" {
            receipt_last = true;
        } else if let Some(id) = arg.strip_prefix("--receipt-id=") {
            receipt_id = Some(id.to_owned());
        } else if arg == "--receipt-id" {
            receipt_id = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--receipt-id", "a receipt identity"))?,
            );
        } else if let Some(path) = arg.strip_prefix("--receipt=") {
            receipt_file = Some(path.to_owned());
        } else if arg == "--receipt" {
            receipt_file = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--receipt", "a receipt file"))?,
            );
        } else if arg == "--no-receipt" {
            no_receipt = true;
        } else if let Some(p) = arg.strip_prefix("--artifact-dir=") {
            artifact_dir = Some(p.to_string());
        } else if arg == "--artifact-dir" {
            artifact_dir = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--artifact-dir", "a directory"))?,
            );
        } else if let Some(f) = arg
            .strip_prefix("--form=")
            .map(ToOwned::to_owned)
            .map_or_else(|| (arg == "--form").then(|| it.next()).flatten(), Some)
        {
            form = Some(match f.as_str() {
                "flattened" => ArtifactForm::Flattened,
                "multipart" => ArtifactForm::Multipart,
                "mirrored-tree" => ArtifactForm::MirroredTree,
                "preserved-book-tree" => ArtifactForm::PreservedBookTree,
                other => {
                    return Err(value_not_recognized(
                        "--form",
                        other,
                        "flattened|multipart|mirrored-tree|preserved-book-tree",
                    ));
                }
            });
        } else if let Some(p) = arg.strip_prefix("--shim-dir=") {
            shim_dir = Some(p.to_string());
        } else if arg == "--shim-dir" {
            shim_dir = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--shim-dir", "a directory"))?,
            );
        } else if let Some(h) = arg.strip_prefix("--host=") {
            host = Some(h.to_string());
        } else if arg == "--host" {
            host = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--host", "an ssh destination"))?,
            );
        } else if let Some(p) = arg.strip_prefix("--plan=") {
            plan = Some(p.to_string());
        } else if arg == "--plan" {
            plan = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--plan", "a path"))?,
            );
        } else if arg == "--accept-new" {
            accept_new = true;
        } else if let Some(p) = arg.strip_prefix("--ssh-config=") {
            ssh_config = Some(p.to_string());
        } else if arg == "--ssh-config" {
            ssh_config = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--ssh-config", "a path"))?,
            );
        } else if let Some(v) = arg.strip_prefix("--connect-timeout=") {
            connect_timeout = Some(seconds_value("--connect-timeout", v)?);
        } else if arg == "--connect-timeout" {
            let v = it
                .next()
                .ok_or_else(|| flag_needs_value("--connect-timeout", "seconds"))?;
            connect_timeout = Some(seconds_value("--connect-timeout", &v)?);
        } else if let Some(v) = arg.strip_prefix("--probe-timeout=") {
            probe_timeout = Some(seconds_value("--probe-timeout", v)?);
        } else if arg == "--probe-timeout" {
            let v = it
                .next()
                .ok_or_else(|| flag_needs_value("--probe-timeout", "seconds"))?;
            probe_timeout = Some(seconds_value("--probe-timeout", &v)?);
        } else if let Some(v) = arg.strip_prefix("--apply-timeout=") {
            apply_timeout = Some(seconds_value("--apply-timeout", v)?);
        } else if arg == "--apply-timeout" {
            let v = it
                .next()
                .ok_or_else(|| flag_needs_value("--apply-timeout", "seconds"))?;
            apply_timeout = Some(seconds_value("--apply-timeout", &v)?);
        } else if arg.starts_with('-') && arg != "-" {
            // An unrecognized FLAG: suggest the nearest known one (did-you-mean) rather than a bare
            // "unexpected argument" (the recon's missing-suggestion hazard). A bare `-` is not a
            // flag at all — it names stdin in filename position
            // (`30I:rul-dash-is-stdin-in-any-filename-position`) and falls through below.
            let known = [
                "--book",
                "--pre-source",
                "--oracle-dir",
                "--results",
                "--debug-argv",
                "--risk-faultless-skips",
                "--no-probe-escalation",
                "--probe-escalation",
                "--escalate-any-probe",
                "--probe-capability",
                "--receipts",
                "--no-receipt",
                "--receipt-last",
                "--receipt-id",
                "--receipt",
                "--all",
                "--json",
                "--shim-dir",
                "--host",
                "--plan",
                "--accept-new",
                "--ssh-config",
                "--connect-timeout",
                "--probe-timeout",
                "--apply-timeout",
                "--help",
                "--version",
            ];
            return Err(match nearest(&arg, &known) {
                Some(sugg) => Diag::new_spanless_site(DiagCode::CliUnknownFlagDidYouMean(
                    dorc_aid::diag::CliUnknownFlagDidYouMean {
                        flag: arg.clone(),
                        suggestion: sugg.to_owned(),
                    },
                )),
                None => Diag::new_spanless_site(DiagCode::CliUnknownFlag(
                    dorc_aid::diag::CliUnknownFlag { flag: arg.clone() },
                )),
            });
        } else if mode == Mode::Why && why_address.is_none() {
            // `289:rider-why-last-address-order`: the address is the first bare word WHEREVER it
            // sits — taking it only when it leads answered the wrong surface at rc 0.
            why_address = Some(arg);
        } else {
            // A bare word, or `-`: this target's ONE main book (the day-one `dorc plan book.sh`
            // ergonomic). A second one is refused below, never merged.
            books.push(arg);
        }
    }
    // Reading a receipt is an EXPLAIN act, so the flags that select a root belong to the explain
    // surface and nowhere else. They used to replay on the plan-producing modes too, which turned a
    // stored record stream back into the inputs of an emitted artifact. Refused HERE, ahead of the
    // book check, because a mode that cannot use the flag should say so rather than complain about
    // a book it would not have needed. `--receipts` is deliberately absent: it names the store
    // plan and apply PUBLISH into, so it is legal in every mode.
    for (present, flag) in [
        (receipt_last, "--receipt-last"),
        (receipt_id.is_some(), "--receipt-id"),
        (receipt_file.is_some(), "--receipt"),
        // A REGISTER of the receipt-rooted surface: a plan or apply run has none, so accepting the
        // flag there would be an assertion the admin only believes they made.
        (json, "--json"),
    ] {
        if present && mode != Mode::Why {
            return Err(Diag::new_spanless_site(DiagCode::CliFlagRequiresMode(
                dorc_aid::diag::CliFlagRequiresMode {
                    flag,
                    mode: "dorc why",
                },
            )));
        }
    }
    // The three root selectors are MUTUALLY EXCLUSIVE (`30R:receipt-rooted-attention-and-cli`):
    // each names one attention root, and ranking two against each other would be inventing a
    // preference the design refuses to have. Reported as the first colliding pair in argv-independent
    // order, because naming all three would not tell the reader anything the pair does not.
    for (first, first_present, second, second_present) in [
        (
            "--receipt",
            receipt_file.is_some(),
            "--receipt-id",
            receipt_id.is_some(),
        ),
        (
            "--receipt",
            receipt_file.is_some(),
            "--receipt-last",
            receipt_last,
        ),
        (
            "--receipt-id",
            receipt_id.is_some(),
            "--receipt-last",
            receipt_last,
        ),
        // Naming records selects the LIVE route, which emits no machine register — so the pair is a
        // format nothing would print. Refused rather than quietly ignored.
        ("--json", json, "--results", results.is_some()),
    ] {
        if first_present && second_present {
            return Err(Diag::new_spanless_site(
                DiagCode::CliFlagsMutuallyExclusive(dorc_aid::diag::CliFlagsMutuallyExclusive {
                    first,
                    second,
                }),
            ));
        }
    }
    let ships_a_rendered_plan = mode == Mode::Apply && host.is_some();
    if let [first, second, ..] = &books[..] {
        return Err(Diag::new_spanless_site(DiagCode::CliSeveralMainBooks(
            dorc_aid::diag::CliSeveralMainBooks {
                first: first.clone(),
                second: second.clone(),
            },
        )));
    }
    let book = books.into_iter().next();
    if book.is_none()
        && mode != Mode::Bundle
        && !ships_a_rendered_plan
        && !reads_the_receipt(
            mode,
            receipt_last || receipt_id.is_some() || receipt_file.is_some(),
            results.is_some(),
        )
    {
        return Err(Diag::new_spanless_site(DiagCode::CliNoBookGiven(
            dorc_aid::diag::CliNoBookGiven,
        )));
    }
    // Stdin is ONE resource and every mode wanting it declares its claim (`30I` §2.5). Two
    // claimants refuse before network and NAME BOTH, rather than ranking them by a silent
    // precedence the reader would have to know.
    if let [first, second, ..] = &stdin_claimants(
        book.as_deref(),
        &pre_sources,
        results.as_deref(),
        plan.as_deref(),
    )[..]
    {
        return Err(Diag::new_spanless_site(DiagCode::CliStdinClaimedTwice(
            dorc_aid::diag::CliStdinClaimedTwice {
                first: (*first).to_owned(),
                second: (*second).to_owned(),
            },
        )));
    }
    if results.is_some() && host.is_some() {
        return Err(Diag::new_spanless_site(
            DiagCode::CliFlagsMutuallyExclusive(dorc_aid::diag::CliFlagsMutuallyExclusive {
                first: "--results",
                second: "--host",
            }),
        ));
    }
    if host.is_some() && !matches!(mode, Mode::Plan | Mode::Apply) {
        return Err(Diag::new_spanless_site(DiagCode::CliFlagRequiresMode(
            dorc_aid::diag::CliFlagRequiresMode {
                flag: "--host",
                mode: "dorc plan or dorc apply",
            },
        )));
    }
    // `owed-no-flag-defaults-to-stdin`: the artifact lane no longer falls back to stdin, so a
    // remote apply must NAME what it ships -- a path, or `-` for stdin. Refusing here is what
    // keeps "no flag acquires stdin implicitly" a rule rather than a preference.
    if ships_a_rendered_plan && plan.is_none() {
        return Err(Diag::new_spanless_site(DiagCode::CliModeNeedsFlag(
            dorc_aid::diag::CliModeNeedsFlag {
                mode: "dorc apply --host",
                flag: "--plan",
            },
        )));
    }
    if plan.is_some() && !ships_a_rendered_plan {
        return Err(Diag::new_spanless_site(DiagCode::CliFlagRequiresMode(
            dorc_aid::diag::CliFlagRequiresMode {
                flag: "--plan",
                mode: "dorc apply --host",
            },
        )));
    }
    // A remote apply's intent publication IS its dispatch authority (`30R:publication-and-dispatch-
    // boundary`, no bypass in V1), so honouring the flag would be the bypass and ignoring it wrote
    // rich receipts under a flag saying none would be. HERE, in the parser, because the refusal has
    // to precede the plan file, the keyset, the store, the clock and the transport.
    if ships_a_rendered_plan && no_receipt {
        return Err(Diag::new_spanless_site(DiagCode::ApplyReceiptNotOptional(
            dorc_aid::diag::ApplyReceiptNotOptional,
        )));
    }
    // `probe` is IN although it emits no plan: it is the round-trip's own first PHASE, and refusing
    // there would stop one invocation carrying one flag set across both halves.
    let plans_or_probes = matches!(
        mode,
        Mode::Plan | Mode::Apply | Mode::RoundTrip | Mode::Probe
    );
    for (named, flag) in [
        (artifact_dir.is_some(), "--artifact-dir"),
        (form.is_some(), "--form"),
    ] {
        if named && !plans_or_probes {
            return Err(Diag::new_spanless_site(DiagCode::CliFlagRequiresMode(
                dorc_aid::diag::CliFlagRequiresMode {
                    flag,
                    mode: "dorc plan, dorc apply or the round-trip",
                },
            )));
        }
    }
    Ok(Invocation::Analyze(Box::new(Args {
        mode,
        book,
        pre_sources,
        oracle_dirs,
        results,
        debug_argv,
        risk_faultless_skips,
        why_address,
        dial,
        capability,
        receipts,
        no_receipt,
        receipt_last,
        receipt_id,
        receipt_file,
        all,
        json,
        host,
        plan,
        accept_new,
        ssh_config,
        connect_timeout,
        probe_timeout,
        apply_timeout,
        artifact_dir,
        form,
        shim_dir,
    })))
}

/// A flag whose value is a whole number of seconds.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see parse_args_from"
)]
fn seconds_value(flag: &str, v: &str) -> Result<u64, InvocationError> {
    v.parse::<u64>().map_err(|_| {
        Diag::new_spanless_site(DiagCode::CliFlagValueNotANumber(
            dorc_aid::diag::CliFlagValueNotANumber {
                flag: flag.to_owned(),
                got: v.to_owned(),
            },
        ))
    })
}
/// A tiny did-you-mean: the nearest `candidate` to `word` within edit-distance 2 (a typo, not a
/// wholly different word), or `None`. Case-sensitive; ASCII. Used for mode + flag suggestions.
///
/// A candidate EQUAL to the word is refused: reaching here means the tables and the parse arms
/// disagree, and "did you mean `--receipts`?" for `--receipts` teaches nothing while hiding the gap.
fn nearest<'a>(word: &str, candidates: &[&'a str]) -> Option<&'a str> {
    // A word the table holds is spelled correctly. Skipping only distance-0 left the NEIGHBOUR
    // one edit away as best-remaining, which is how `--receipt` suggested `--receipts`.
    if candidates.contains(&word) {
        return None;
    }
    candidates
        .iter()
        .map(|c| (levenshtein(word, c), *c))
        .filter(|(d, _)| *d <= 2 && *d > 0)
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c)
}

/// Levenshtein edit-distance (the two-row DP), for [`nearest`]. Pure; small inputs (flag/mode
/// names), so the allocation is irrelevant.
#[expect(
    clippy::indexing_slicing,
    reason = "the two DP rows are sized `b_chars.len()+1`; `j` ranges `0..b_chars.len()`, so every `[j]`/`[j+1]` index is in-bounds by construction"
)]
fn levenshtein(a: &str, b: &str) -> usize {
    let b_chars: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    let mut cur = vec![0usize; b_chars.len().saturating_add(1)];
    for (i, ca) in a.chars().enumerate() {
        cur[0] = i.saturating_add(1);
        for (j, &cb) in b_chars.iter().enumerate() {
            let cost = usize::from(ca != cb);
            cur[j.saturating_add(1)] = (prev[j.saturating_add(1)].saturating_add(1))
                .min(cur[j].saturating_add(1))
                .min(prev[j].saturating_add(cost));
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b_chars.len()]
}
/// A HUMANE file-read error (the recon flagged raw OS phrasing leaking to the user): name what
/// we were reading and the path, and translate the common `io::ErrorKind`s to plain English
/// (a missing/permission-denied file, the two an admin actually hits) rather than the platform's
/// raw "The system cannot find the file specified. (os error 2)".
/// THREE SIBLING CODES, not one `{why}`-parameterized code
/// (`AID-NEEDS:law-codes-vary-by-world-not-grammar`): a missing file, a file you may not read, and
/// an unclassed OS failure are three states of the world with three different remediations. Only
/// the residual arm passes the platform's own words through.
#[must_use]
pub fn humane_read_error(kind: &str, path: &str, err: &std::io::Error) -> InvocationError {
    match err.kind() {
        std::io::ErrorKind::NotFound => {
            Diag::new_spanless_site(DiagCode::CliFileNotFound(dorc_aid::diag::CliFileNotFound {
                kind: kind.to_owned(),
                path: path.to_owned(),
            }))
        }
        std::io::ErrorKind::PermissionDenied => Diag::new_spanless_site(
            DiagCode::CliFilePermissionDenied(dorc_aid::diag::CliFilePermissionDenied {
                kind: kind.to_owned(),
                path: path.to_owned(),
            }),
        ),
        _ => Diag::new_spanless_site(DiagCode::CliFileUnreadable(
            dorc_aid::diag::CliFileUnreadable {
                kind: kind.to_owned(),
                path: path.to_owned(),
                detail: dorc_aid::ForeignBytes::from_os_error(err),
            },
        )),
    }
}

/// Map a shim-directory write failure at the production edge.
#[must_use]
pub fn shim_write_error(path: &str, error: &std::io::Error) -> InvocationError {
    Diag::new_spanless_site(DiagCode::CliShimDirUnwritable(
        dorc_aid::diag::CliShimDirUnwritable {
            path: path.to_owned(),
            detail: dorc_aid::ForeignBytes::from_os_error(error),
        },
    ))
}

/// Map a `dorc-sh` script read failure at the production edge.
#[must_use]
pub fn shim_script_read_error(path: &str, error: &std::io::Error) -> InvocationError {
    Diag::new_spanless_site(DiagCode::DorcShScriptUnreadable(
        dorc_aid::diag::DorcShScriptUnreadable {
            path: path.to_owned(),
            detail: dorc_aid::ForeignBytes::from_os_error(error),
        },
    ))
}

/// Map a `dorc-sh` process spawn failure at the production edge.
#[must_use]
pub fn shim_exec_error(error: &std::io::Error) -> InvocationError {
    Diag::new_spanless_site(DiagCode::DorcShExecFailed(
        dorc_aid::diag::DorcShExecFailed {
            detail: dorc_aid::ForeignBytes::from_os_error(error),
        },
    ))
}

/// Map carriage-return detection before transport.
#[must_use]
pub fn transport_crlf_error(which: &str, line: usize) -> InvocationError {
    Diag::new_spanless_site(DiagCode::TransportCrlfRefused(
        dorc_aid::diag::TransportCrlfRefused {
            which: which.to_owned(),
            line: line.to_string(),
        },
    ))
}

/// Map a transport session that exhausted its retries without completion.
#[must_use]
pub fn transport_session_lost(
    host: &str,
    attempts: u32,
    diagnosis: &dorc_transport::TransportDiagnosis,
) -> InvocationError {
    let diagnosis = match diagnosis {
        dorc_transport::TransportDiagnosis::TimedOut { after } => {
            format!("timed out after {}s", after.as_secs())
        }
        dorc_transport::TransportDiagnosis::ChildExited { status: Some(code) } => {
            format!("ssh exited {code}")
        }
        dorc_transport::TransportDiagnosis::ChildExited { status: None } => {
            "ssh exited on a signal".to_owned()
        }
        dorc_transport::TransportDiagnosis::ChildLost => {
            "the session ended without a status".to_owned()
        }
    };
    Diag::new_spanless_site(DiagCode::TransportSessionLost(
        dorc_aid::diag::TransportSessionLost {
            host: host.to_owned(),
            attempts: attempts.to_string(),
            diagnosis,
        },
    ))
}

/// Map a transport process spawn refusal.
#[must_use]
pub fn transport_spawn_refused(host: &str, detail: &str) -> InvocationError {
    Diag::new_spanless_site(DiagCode::TransportSpawnRefused(
        dorc_aid::diag::TransportSpawnRefused {
            host: host.to_owned(),
            detail: dorc_aid::ForeignBytes::from_io_edge(detail),
        },
    ))
}

/// Map a nonce that cannot form a transport marker.
#[must_use]
pub fn transport_marker_unusable(host: &str) -> InvocationError {
    Diag::new_spanless_site(DiagCode::TransportMarkerUnusable(
        dorc_aid::diag::TransportMarkerUnusable {
            host: host.to_owned(),
        },
    ))
}

/// Map a completed remote apply with a non-zero status.
#[must_use]
pub fn transport_apply_failed(host: &str, status: i32) -> InvocationError {
    Diag::new_spanless_site(DiagCode::TransportApplyFailed(
        dorc_aid::diag::TransportApplyFailed {
            host: host.to_owned(),
            status: status.to_string(),
        },
    ))
}

/// Construct the unloaded-sibling advisory from loaded and discovered paths.
#[must_use]
pub fn unloaded_sibling_oracle_diagnostics(
    loaded_paths: &[String],
    discovered_paths: &[String],
) -> Vec<Diag> {
    let loaded: std::collections::BTreeSet<String> = loaded_paths
        .iter()
        .map(|path| oracle_path_key(path))
        .collect();
    let mut unloaded: Vec<String> = discovered_paths
        .iter()
        .map(|path| path.replace('\\', "/"))
        .filter(|path| path.ends_with(".oracle.sh") && !loaded.contains(&oracle_path_key(path)))
        .collect();
    unloaded.sort();
    unloaded.dedup();
    if unloaded.is_empty() {
        return Vec::new();
    }
    let oracles = unloaded
        .iter()
        .map(|path| format!("`{path}`"))
        .collect::<Vec<_>>()
        .join(", ");
    vec![Diag::new_spanless_site(DiagCode::AidUnloadedSiblingOracle(
        dorc_aid::diag::AidUnloadedSiblingOracle { oracles },
    ))]
}

/// Normalize an oracle path for loaded-versus-discovered comparison without filesystem access.
#[must_use]
pub fn oracle_path_key(path: &str) -> String {
    use std::path::{Component, Path, PathBuf};

    let slash_separated = path.replace('\\', "/");
    let keyed: PathBuf = Path::new(&slash_separated)
        .components()
        .filter(|component| !matches!(component, Component::CurDir))
        .collect();
    let keyed = keyed.to_string_lossy().replace('\\', "/");
    if keyed.is_empty() {
        ".".to_owned()
    } else {
        keyed
    }
}

/// The parsed `dorc lint` invocation (`27R` §5). Files + oracle sources + the render/exit knobs.
#[derive(Debug)]
pub struct LintArgs {
    /// The lintable files given as positionals.
    pub files: Vec<String>,
    /// `--oracle PATH`: oracle sources to lint as oracles.
    pub oracles: Vec<String>,
    /// `--oracle-dir DIR`: every `*.oracle.sh` in DIR, glob-sorted.
    pub oracle_dirs: Vec<String>,
    /// `--format`: which of the two renders to emit.
    pub format: LintFormat,
    /// The `--fail-on` threshold as a severity, or `None` for `never` (`27R` §5). The one severity
    /// vocabulary is `core::Severity` (`27V` §3 rider-d); the `warn` wire token maps to `Warning`.
    pub fail_on: Option<Severity>,
    /// `--no-tools` clears this: whether external linters may run at all.
    pub tools_enabled: bool,
    /// `--require-tools`: an absent configured tool becomes operational, not advisory.
    pub require_tools: bool,
    /// `--expect-files N` (`27R` §8b): the exact lintable-file count CI asserts.
    pub expect_files: Option<usize>,
    /// `--list-sources`: enumerate the registry and exit instead of linting.
    pub list_sources: bool,
    /// `--source NAME` subset selection (`27R` §8 delta-named-sources-selectable); empty ⇒ all.
    pub sources: Vec<String>,
    /// The human render's density (`289:rul-lint-render-split-is-policy`). Default reproduces each
    /// finding's declared shape, so the surface only moves when the admin asks.
    pub verbosity: dorc_lint::render::Verbosity,
}

/// The lint invocation's operational scope failure, in production precedence order.
#[must_use]
pub fn lint_operational_diagnostic(
    args: &LintArgs,
    found_files: usize,
    report: &dorc_lint::LintReport,
) -> Option<Diag> {
    if found_files == 0 {
        return Some(Diag::new_spanless_site(DiagCode::LintNoLintableFiles(
            dorc_aid::diag::LintNoLintableFiles,
        )));
    }
    if let Some(expected) = args.expect_files
        && expected != found_files
    {
        return Some(Diag::new_spanless_site(DiagCode::LintFileCountDrift(
            dorc_aid::diag::LintFileCountDrift {
                expected,
                found: found_files,
            },
        )));
    }
    if args.require_tools {
        let absent = report
            .coverage
            .sources
            .iter()
            .filter(|source| source.status == dorc_lint::SourceStatus::Absent)
            .map(|source| source.name)
            .collect::<Vec<_>>();
        if !absent.is_empty() {
            return Some(Diag::new_spanless_site(DiagCode::LintRequiredToolsMissing(
                dorc_aid::diag::LintRequiredToolsMissing {
                    tools: absent.join(", "),
                },
            )));
        }
    }
    None
}

/// The `--format` choice (`27R` §5 dir-two-renders-one-model).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintFormat {
    /// The unstable-by-declaration human render.
    Human,
    /// The versioned additive-only machine envelope.
    Jsonl,
}

/// Parse `dorc lint <files…> [flags]` (`raw[0]` is `"lint"`). No config file, ever (`kOOB` redline —
/// flags only); no comment-directive suppression for dorc-native findings (the dialect marker stays
/// the one comment-parse). Files are positionals; `--source NAME` selects a subset (positionals are
/// taken by files, so subset selection needs a flag — deviation from brew's positional-checks, `27S`).
/// # Errors
/// Returns the rendered lint usage/argument error a bad invocation produces.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see parse_args_from"
)]
pub fn parse_lint_args(raw: &[String]) -> Result<Invocation, InvocationError> {
    let mut files = Vec::new();
    let mut oracles = Vec::new();
    let mut oracle_dirs = Vec::new();
    let mut format = LintFormat::Human;
    // tc-lint-fail-on-default: `error` (hot-loop mercy; CI tightens to `warn`) — `27R` §6, flagged.
    let mut fail_on = Some(Severity::Error);
    let mut tools_enabled = true;
    let mut require_tools = false;
    let mut expect_files = None;
    let mut list_sources = false;
    let mut sources = Vec::new();
    let mut verbosity = dorc_lint::render::Verbosity::default();
    let mut it = raw.iter().skip(1).cloned().peekable();
    while let Some(arg) = it.next() {
        if let Some(p) = arg.strip_prefix("--oracle-dir=") {
            oracle_dirs.push(p.to_owned());
        } else if arg == "--oracle-dir" {
            oracle_dirs.push(
                it.next()
                    .ok_or_else(|| flag_needs_value("--oracle-dir", "a directory"))?,
            );
        } else if arg == "--oracle" {
            oracles.push(
                it.next()
                    .ok_or_else(|| flag_needs_value("--oracle", "a path"))?,
            );
        } else if let Some(p) = arg.strip_prefix("--oracle=") {
            oracles.push(p.to_owned());
        } else if let Some(p) = arg.strip_prefix("--format=") {
            format = parse_lint_format(p)?;
        } else if arg == "--format" {
            format = parse_lint_format(
                &it.next()
                    .ok_or_else(|| flag_needs_value("--format", "a value"))?,
            )?;
        } else if let Some(p) = arg.strip_prefix("--fail-on=") {
            fail_on = parse_fail_on(p)?;
        } else if arg == "--fail-on" {
            fail_on = parse_fail_on(
                &it.next()
                    .ok_or_else(|| flag_needs_value("--fail-on", "a value"))?,
            )?;
        } else if arg == "--no-tools" {
            tools_enabled = false;
        } else if arg == "--require-tools" {
            require_tools = true;
        } else if let Some(p) = arg.strip_prefix("--expect-files=") {
            expect_files = Some(parse_expect_count(p)?);
        } else if arg == "--expect-files" {
            expect_files =
                Some(parse_expect_count(&it.next().ok_or_else(|| {
                    flag_needs_value("--expect-files", "a number")
                })?)?);
        } else if arg == "--terse" {
            verbosity = dorc_lint::render::Verbosity::Terse;
        } else if arg == "--verbose" {
            verbosity = dorc_lint::render::Verbosity::Verbose;
        } else if arg == "--list-sources" {
            list_sources = true;
        } else if let Some(p) = arg.strip_prefix("--source=") {
            sources.push(p.to_owned());
        } else if arg == "--source" {
            sources.push(
                it.next()
                    .ok_or_else(|| flag_needs_value("--source", "a name"))?,
            );
        } else if arg.starts_with('-') {
            return Err(Diag::new_spanless_site(DiagCode::CliUnknownFlag(
                dorc_aid::diag::CliUnknownFlag { flag: arg.clone() },
            )));
        } else {
            files.push(arg);
        }
    }
    Ok(Invocation::Lint(LintArgs {
        files,
        oracles,
        oracle_dirs,
        format,
        fail_on,
        tools_enabled,
        require_tools,
        expect_files,
        list_sources,
        sources,
        verbosity,
    }))
}

#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see parse_args_from"
)]
fn parse_lint_format(v: &str) -> Result<LintFormat, InvocationError> {
    match v {
        "human" => Ok(LintFormat::Human),
        "jsonl" => Ok(LintFormat::Jsonl),
        other => Err(value_not_recognized("--format", other, "human|jsonl")),
    }
}

/// `--fail-on` → the severity threshold, or `None` for `never`. `27R` §5: only `error`/`warn`
/// gate (`info` never does).
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see parse_args_from"
)]
fn parse_fail_on(v: &str) -> Result<Option<Severity>, InvocationError> {
    match v {
        "error" => Ok(Some(Severity::Error)),
        "warn" => Ok(Some(Severity::Warning)),
        "never" => Ok(None),
        other => Err(value_not_recognized("--fail-on", other, "error|warn|never")),
    }
}

#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see parse_args_from"
)]
fn parse_expect_count(v: &str) -> Result<usize, InvocationError> {
    v.parse::<usize>().map_err(|_| {
        Diag::new_spanless_site(DiagCode::CliFlagValueNotANumber(
            dorc_aid::diag::CliFlagValueNotANumber {
                flag: "--expect-files".to_owned(),
                got: v.to_owned(),
            },
        ))
    })
}

// ── the invocation-error mints (`288` §6) ────────────────────────────────────────────────────
//
// Spanless, and the payload variant is spelled LITERALLY at every mint — the allow-list gate is a
// lexical grep for exactly that shape.

/// A flag that takes a value, given without one — ONE code across every such flag.
fn flag_needs_value(flag: &str, wants: &'static str) -> InvocationError {
    Diag::new_spanless_site(DiagCode::CliFlagNeedsValue(
        dorc_aid::diag::CliFlagNeedsValue {
            flag: flag.to_owned(),
            wants,
        },
    ))
}

/// A flag value outside its closed vocabulary.
fn value_not_recognized(flag: &str, got: &str, expected: &'static str) -> InvocationError {
    Diag::new_spanless_site(DiagCode::CliFlagValueNotRecognized(
        dorc_aid::diag::CliFlagValueNotRecognized {
            flag: flag.to_owned(),
            got: got.to_owned(),
            expected,
        },
    ))
}

// ── the why-report render seam (`28H:prop-drifted-why-is-the-thin-driver`) ────────────────────

/// The consent flag as the BINARY spells it. The corpus names this lever
/// `--risk-faultless-skips` (`spike/CLAUDE.md` survive-license, `271:rul-flag-is-razor-residue`); the
/// cli implements `--risk-faultless-skips`. A why-surface pointer must be copy-paste-true (`28E` §7
/// held-placement-reread), so the render prints what the parser accepts and the rename is flagged
/// upward rather than papered over here.
pub const CONSENT_FLAG: &str = "--risk-faultless-skips";

/// The canonical render width. Layout is the RENDERER's, never the semantics engine's
/// (`28E` §8 `rul-renderer-owns-layout`): every seat below hands `weft` a MARKED tree, and weft
/// rules columns, wrapping and blocks. The doc-algebra reflow engine that will replace its filler
/// is still deferred (`28G` §2), so the surface renders at ONE fixed width and transcripts pin
/// there.
pub const WHY_WIDTH: usize = 92;

/// The table the receipt header's record lines join — one block, degrading as a unit.
pub const RECEIPT_TABLE: &str = "why-receipt";

/// The book is byte-identical to the same path at this commit. Pure data by the time it is held.
///
/// The nondeterminism (a subprocess, a filesystem, a repository that may not exist) is spent at the
/// cli edge (`main.rs`'s `source_match`) and never travels: what crosses into the render is this
/// string, in exactly the way `RunClock` spends a clock read and passes a `RunInstant` inward. It
/// sits on THIS side of the seam because it is the VALUE, not the query (`lib-target-is-a-loom-seam`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMatch {
    /// The short commit the book is at.
    pub commit: String,
}

/// Lay a marked tree out at `inset` and re-attribute it — the ONE seat where the why surface
/// becomes bytes, and the span map's production consumer.
///
/// The render happens ONCE and the span map is carried, not dropped: the printed bytes come out
/// of the part stream, so every transcript the corpus drives proves the bridge total and
/// byte-exact (`_w4-map-DRAFT:gap-span-map-unconsumed`). Nothing here is `dorc why`-specific — a
/// loom driver re-renders the same tree through the same seat and gets a stream it can attribute
/// an edit against.
#[must_use]
pub fn why_parts(nodes: Vec<Node<Face>>, inset: usize) -> dorc_aid::tagged::RenderParts {
    let frame = weft::Frame::of_width(weft::Width::new(WHY_WIDTH)).inset(inset);
    dorc_aid::weave::to_render_parts(&weft::render_framed(&Document::new(nodes), &frame))
}

/// One attributed fragment as a paragraph.
#[must_use]
pub fn paragraph(ctx: &RenderCtx<'_>, said: &Said, part: &'static str) -> Node<Face> {
    Node::new(NodeKind::Prose(Paragraph {
        runs: said.runs(ctx, part),
    }))
}

/// A whole registry line as a paragraph.
#[must_use]
pub fn registry_paragraph(ctx: &RenderCtx<'_>, slug: &'static str) -> Node<Face> {
    paragraph(ctx, &Said::words(slug, &[]), slug)
}

/// The invocation record the zero-argument `dorc why` opens with (`28D:need-exact-input-identity`;
/// `28G` strawman `a-fire-morning` lines 33–38): which run this is, on which host, over which
/// bytes, under which consent, and what it decided.
///
/// Every field is CONTROLLER-minted (`rul-attribution-is-controller-minted`) — the host contributes
/// none of it, including the instant (`28F:rul-probe-instants-host-says-no-times`, human-typed).
#[derive(Debug)]
pub struct Receipt {
    /// This invocation's start instant, or `None` when the edge had no clock.
    pub at: Option<dorc_core::RunInstant>,
    /// The session host id.
    pub host: String,
    /// The analyzed book's path.
    pub book: String,
    /// The analyzed book's content digest.
    pub book_digest: String,
    /// The commit the book sits at, when it sits at one exactly (`28E:lean-git-source-tracking-
    /// secondary`). Already-resolved pure data: the subprocess that answered it was spent at the
    /// edge, and a `None` here is indistinguishable from "no repository", by design.
    pub at_head: Option<SourceMatch>,
    /// The loaded oracles, in argv order.
    pub oracles: Vec<String>,
    /// The consent flag in force, or `None` for a flagless run.
    pub risk_profile: Option<&'static str>,
    /// What the run decided, as much of it as this receipt can honestly state.
    pub tally: dorc_plan::DispositionCounts,
    /// `--all`: the reader asked for the deepest pull tier.
    pub deepest_tier: bool,
}

/// The receipt header as one banner: the run's identity, then the indented record of what it read
/// and what it decided.
///
/// The plan tally counts the TYPED disposition, never the rendered word: the words are registry
/// prose meant to churn (`27V:rul-output-form-unwelded`), so a tally keyed on them would silently
/// go wrong the first time someone rewrote one.
#[must_use]
pub fn receipt_banner(ctx: &RenderCtx<'_>, receipt: &Receipt) -> Node<Face> {
    let when = match receipt.at {
        Some(at) => Said::words(
            "why-receipt-when-live",
            &[&dorc_aid::instant::date_time_text(at)],
        ),
        None => Said::words("why-receipt-when-undated", &[]),
    };
    let counts = receipt.tally;
    let tally = if counts.elide_by_trusted_claim == 0 {
        Said::words(
            "why-receipt-plan-tally-by-proof",
            &[
                &counts.run.to_string(),
                &counts.guard.to_string(),
                &counts.elide.to_string(),
                &counts.elide_by_proof.to_string(),
            ],
        )
    } else {
        Said::words(
            "why-receipt-plan-tally",
            &[
                &counts.run.to_string(),
                &counts.guard.to_string(),
                &counts.elide.to_string(),
                &counts.elide_by_proof.to_string(),
                &counts.elide_by_trusted_claim.to_string(),
            ],
        )
    };
    let risk = receipt.risk_profile.map_or_else(
        || Said::words("why-receipt-risk-profile-none", &[]),
        |profile| Said::Value(profile.to_owned()),
    );
    // Replaces the digest row rather than joining it: exact-or-absent, never a third shape.
    let book_row = match &receipt.at_head {
        Some(matched) => Said::words(
            "why-receipt-book-at-head",
            &[&receipt.book, &matched.commit],
        ),
        None => Said::words("why-receipt-book", &[&receipt.book, &receipt.book_digest]),
    };
    let mut body = vec![receipt_row(ctx, &book_row)];
    body.extend([
        receipt_row(
            ctx,
            &Said::words("why-receipt-oracles", &[&receipt.oracles.join(", ")]),
        ),
        receipt_row(
            ctx,
            &Said::sentence("why-receipt-risk-profile", None, vec![risk]),
        ),
        receipt_row(ctx, &tally),
        // `tc-apply-report-is-prediction`: no apply executor exists, so saying so IS the whole
        // replayed-voice obligation — never let a reader take a prediction for an outcome.
        receipt_row(ctx, &Said::words("why-receipt-dispositions-predicted", &[])),
        receipt_row(ctx, &Said::words("why-addressability-line", &[])),
    ]);
    Node::new(NodeKind::Banner(Banner {
        headline: Said::sentence(
            "why-receipt-header",
            None,
            vec![when, Said::Value(receipt.host.clone())],
        )
        .runs(ctx, "why-receipt"),
        body,
    }))
}

/// One line of the receipt header's indented record.
///
/// A labelled row rather than a paragraph, because the six lines are ONE block: weft keeps a run of
/// like rows tight and puts a blank line between unlike things, and a receipt broken up by blank
/// lines reads as six separate remarks rather than one identity.
#[must_use]
pub fn receipt_row(ctx: &RenderCtx<'_>, said: &Said) -> Node<Face> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table: Some(Face::Table(RECEIPT_TABLE.to_owned())),
        label: Vec::new(),
        body: said.runs(ctx, "why-receipt"),
        attachments: Vec::new(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(argv: &[&str]) -> Invocation {
        parse_args_from(argv.iter().map(|word| (*word).to_owned()).collect())
            .expect("invocation parses")
    }

    fn analyzed(words: &[&str]) -> Args {
        match parse(words) {
            Invocation::Analyze(args) => *args,
            other => panic!("expected an analysis invocation, got {other:?}"),
        }
    }

    /// The surface fold (`28E:lean-why-is-whylog-reconciliation`): which invocations answer from
    /// the stored receipt, and which from records handed in. Worth pinning as a table rather than
    /// trusting the one-line predicate, because getting it wrong is SILENT in the worst direction —
    /// a `why` that quietly analyses fresh records while the admin believes they are reading last
    /// night's receipt is the wrong-surface-at-rc-0 class `289:rider-why-last-address-order` cost
    /// us once already.
    #[test]
    fn only_an_unnamed_record_source_reads_the_receipt() {
        assert!(
            analyzed(&["why"]).reads_the_receipt(),
            "bare `dorc why` is the fold's whole point: no book, no records, read the receipt"
        );
        assert!(
            analyzed(&["why", "10"]).reads_the_receipt(),
            "an address narrows the question, it does not name a record source"
        );
        assert!(
            !analyzed(&["why", "--results", "r.txt", "--book=book.sh"]).reads_the_receipt(),
            "naming records is what selects the harness posture"
        );
        assert!(
            !analyzed(&["plan", "book.sh"]).reads_the_receipt(),
            "plan is a live analysis, untouched by the fold"
        );
    }

    /// `--last` is an EXPLAIN flag. Replaying a durable into a mode that emits an artifact let a
    /// stored record stream supply the inputs of a plan, so the flag refuses everywhere but `why`
    /// — including `bundle`, which used to accept and silently ignore it.
    #[test]
    fn asking_for_a_stored_receipt_refuses_outside_the_explain_surface() {
        for mode in ["plan", "apply", "probe", "round-trip", "bundle"] {
            let refusal = parse_args_from(vec![
                mode.to_owned(),
                "--receipt-last".to_owned(),
                "book.sh".to_owned(),
            ])
            .expect_err("--receipt-last belongs to dorc why");
            assert_eq!(
                refusal.code.slug(),
                "cli-flag-requires-mode",
                "{mode} --receipt-last must name the mode the flag belongs to"
            );
        }
        assert!(
            parse_args_from(vec!["why".to_owned(), "--receipt-last".to_owned()]).is_ok(),
            "the explain surface still takes it"
        );
    }

    /// `--json` names the receipt-rooted surface's machine REGISTER, so it belongs to `why` and
    /// nowhere else — and never beside the flag that selects the other route.
    ///
    /// Both directions, because a flag accepted where nothing emits it is exactly the silently
    /// ineffective assertion the closed-vocabulary discipline exists to refuse.
    #[test]
    fn the_machine_register_belongs_to_the_receipt_rooted_surface_alone() {
        let slug = |argv: &[&str]| -> Option<String> {
            parse_args_from(argv.iter().map(|word| (*word).to_owned()).collect())
                .err()
                .map(|diag| diag.code.slug().to_owned())
        };
        assert_eq!(
            slug(&["plan", "--json", "book.sh"]).as_deref(),
            Some("cli-flag-requires-mode"),
            "a plan has no register to spell"
        );
        assert_eq!(
            slug(&["why", "--json", "--results", "r.txt"]).as_deref(),
            Some("cli-flags-mutually-exclusive"),
            "naming records selects the live route, which emits no machine register"
        );
        assert!(
            parse_args_from(vec!["why".to_owned(), "--json".to_owned()]).is_ok(),
            "the receipt-rooted surface takes it"
        );
    }

    /// The three ROOT selectors are mutually exclusive and `--receipts` is orthogonal to all of
    /// them (`30R:receipt-rooted-attention-and-cli`).
    ///
    /// Every PAIR, not one sample: each selector names one attention root, so any two of them ask
    /// two questions, and a rule that caught two pairs of three would rank the survivors by
    /// accident. `--receipts` moves WHERE a root is looked up and never WHETHER one was named, so
    /// it must sit beside each selector and outside `why` entirely — the same distinction that made
    /// naming a store silently mean writing no receipt.
    #[test]
    fn the_root_selectors_exclude_each_other_and_the_store_is_orthogonal() {
        let refusal_of = |argv: &[&str]| -> Option<String> {
            parse_args_from(argv.iter().map(|word| (*word).to_owned()).collect())
                .err()
                .map(|diag| diag.code.slug().to_owned())
        };
        let selectors = [
            "--receipt=r.dorc-receipt",
            "--receipt-id=abc",
            "--receipt-last",
        ];
        for (left, right) in [(0, 1), (0, 2), (1, 2)] {
            assert_eq!(
                refusal_of(&["why", selectors[left], selectors[right]]).as_deref(),
                Some("cli-flags-mutually-exclusive"),
                "{} beside {} must refuse as a collision",
                selectors[left],
                selectors[right]
            );
        }
        for selector in selectors {
            assert_eq!(
                refusal_of(&["why", "--receipts=store", selector]),
                None,
                "--receipts sites the lookup for {selector}; it is not a fourth selector"
            );
        }
        assert_eq!(
            refusal_of(&["plan", "--receipts=store", "book.sh"]),
            None,
            "a plan PUBLISHES into the store it names, so the flag is legal outside `why`"
        );
    }

    /// Both spellings of a value-taking flag parse, and nothing in the table can suggest itself:
    /// a flag with only the `=` form answers "did you mean `--receipt`?" to `--receipt`.
    ///
    /// `--receipt` / `--receipts` is the sharpest pair to hold this over — one letter apart, both
    /// live, and meaning two different things — so a suggestion that fired on an exact match would
    /// land on the flag beside the one typed.
    #[test]
    fn a_flag_never_suggests_the_word_that_was_typed() {
        assert_eq!(
            analyzed(&["why", "--receipt", "run.dorc-receipt"])
                .receipt_file
                .as_deref(),
            Some("run.dorc-receipt")
        );
        assert_eq!(
            analyzed(&["why", "--receipt=run.dorc-receipt"])
                .receipt_file
                .as_deref(),
            Some("run.dorc-receipt")
        );
        assert_eq!(nearest("--receipt", &["--receipt", "--receipts"]), None);
        assert_eq!(
            nearest("--receip", &["--receipt", "--book"]),
            Some("--receipt")
        );
    }

    /// Plan-producing and record-reading modes require a book unless a receipt supplies one.
    /// Bundle mode may project invocation-named roots over an empty book.
    #[test]
    fn a_book_is_required_unless_a_receipt_supplies_one() {
        assert!(parse_args_from(vec!["why".to_owned()]).is_ok());
        assert!(
            parse_args_from(vec![
                "why".to_owned(),
                "--results".to_owned(),
                "r.txt".to_owned()
            ])
            .is_err(),
            "records without a book have nothing to be about"
        );
        assert!(
            parse_args_from(vec!["plan".to_owned()]).is_err(),
            "plan still demands its book"
        );
    }

    #[test]
    fn bundle_is_a_pure_analysis_mode_over_one_book() {
        let args = analyzed(&["bundle", "book.sh", "--pre-source", "entry.sh"]);
        assert_eq!(args.mode, Mode::Bundle);
        assert_eq!(args.book.as_deref(), Some("book.sh"));
        assert_eq!(args.pre_sources, ["entry.sh"]);
        assert!(!args.reads_the_receipt());
        assert!(
            analyzed(&["bundle", "--pre-source", "entry.sh"])
                .book
                .is_none()
        );
    }

    /// The did-you-mean helper: a near-miss (edit-distance ≤ 2) suggests, a wholly-different word
    /// does not (no misleading suggestion). Pins the mode + flag typo-suggestion behavior.
    #[test]
    fn nearest_suggests_within_edit_distance_two() {
        let modes = ["probe", "plan", "apply", "why"];
        assert_eq!(nearest("pln", &modes), Some("plan"), "one deletion");
        assert_eq!(nearest("aply", &modes), Some("apply"), "one deletion");
        assert_eq!(
            nearest("wanted", &modes),
            None,
            "a wholly-different word ⇒ no suggestion"
        );
        let flags = ["--risk-faultless-skips", "--debug-argv", "--book"];
        assert_eq!(
            nearest("--risk-faultless-skip", &flags),
            Some("--risk-faultless-skips")
        );
        assert_eq!(nearest("--boook", &flags), Some("--book"), "one insertion");
    }

    /// `289:rider-why-last-address-order`: in `why` mode the address is the first bare word wherever
    /// it sits. The old reading only took it when it LED, so `dorc why --receipt-last book.sh:9`
    /// filed the address as a positional book and answered the unargumented aggregate at rc 0 — the
    /// user asked about one line and silently got the whole-run surface, with nothing to notice.
    #[test]
    fn a_why_address_is_found_after_a_flag() {
        let args =
            |raw: &[&str]| match parse_args_from(raw.iter().map(|a| (*a).to_owned()).collect()) {
                Ok(Invocation::Analyze(args)) => args,
                other => panic!("expected an analyze invocation, got {other:?}"),
            };
        let leading = args(&["why", "book.sh:9", "--book=book.sh"]);
        assert_eq!(leading.why_address.as_deref(), Some("book.sh:9"));

        let after_last = args(&["why", "--receipt-last", "book.sh:9"]);
        assert_eq!(
            after_last.why_address.as_deref(),
            Some("book.sh:9"),
            "the address must survive a preceding flag"
        );
        assert!(
            after_last.book.is_none(),
            "and must not be mistaken for a positional book"
        );

        // The `why`-mode carve does not leak: every other mode still reads a bare word as a book.
        let planned = args(&["plan", "book.sh"]);
        assert_eq!(planned.book.as_deref(), Some("book.sh"));
    }

    /// Parse `raw`, returning the slug of whatever it refuses with — the shape every rule below
    /// asserts, since an invocation refusal's whole product is which code fires.
    fn refusal(raw: &[&str]) -> String {
        match parse_args_from(raw.iter().map(|a| (*a).to_owned()).collect()) {
            Err(diag) => diag.code.slug().to_owned(),
            other => panic!("expected a refusal, got {other:?}"),
        }
    }

    fn analyzed_args(raw: &[&str]) -> Box<Args> {
        match parse_args_from(raw.iter().map(|a| (*a).to_owned()).collect()) {
            Ok(Invocation::Analyze(args)) => args,
            other => panic!("expected an analyze invocation, got {other:?}"),
        }
    }

    /// `30I:rul-pre-source-is-dot-prelude` — repeated pre-sources keep their CLI occurrence order,
    /// because they compile to ordinary `.` commands and a shell's last declaration wins.
    #[test]
    fn pre_sources_keep_their_cli_order() {
        let args = analyzed_args(&[
            "plan",
            "book.sh",
            "--pre-source",
            "a.oracle.sh",
            "--pre-source=b.oracle.sh",
        ]);
        assert_eq!(args.pre_sources, vec!["a.oracle.sh", "b.oracle.sh"]);
    }

    /// `30I:rul-spike-has-no-short-options` — no spike feature carries a single-letter spelling, so
    /// the whole latin alphabet stays free for the post-spike CLI design. The retired `-o` is the
    /// one worth pinning: it was the most-typed flag in the corpus, so a quiet re-acceptance is the
    /// likeliest regression.
    #[test]
    fn no_spike_feature_claims_a_single_letter() {
        assert_eq!(
            refusal(&["plan", "book.sh", "-o", "x.oracle.sh"]),
            "cli-unknown-flag"
        );
        assert_eq!(refusal(&["plan", "book.sh", "-h"]), "cli-unknown-flag");
    }

    /// `30I:rul-one-main-book-per-target` — a second main book is a separate PROGRAM, refused
    /// rather than concatenated. Both spellings reach the same refusal, because the merge this
    /// closes was reachable through either.
    #[test]
    fn a_second_main_book_is_refused_not_merged() {
        assert_eq!(
            refusal(&["plan", "book.sh", "other.sh"]),
            "cli-several-main-books"
        );
        assert_eq!(
            refusal(&["plan", "--book=book.sh", "--book=other.sh"]),
            "cli-several-main-books"
        );
    }

    /// `30I:rul-dash-is-stdin-in-any-filename-position` — `-` names stdin wherever a filename goes,
    /// and is never read as a flag.
    #[test]
    fn a_dash_names_stdin_in_filename_position() {
        assert_eq!(
            analyzed_args(&["probe", "-"]).book.as_deref(),
            Some("-"),
            "a `-` book is that target's book, not an unknown flag"
        );
        assert_eq!(
            analyzed_args(&["probe", "book.sh", "--pre-source", "-"]).pre_sources,
            vec!["-"]
        );
    }

    /// `30I` §2.5 — stdin is ONE resource, so two claimants refuse before network. The records lane
    /// is a claimant too: it takes stdin by default on `plan`, which is exactly why `dorc plan -`
    /// cannot silently win it. `probe` reads no stdin, so the same `-` book is ordinary there.
    #[test]
    fn two_claimants_on_stdin_refuse() {
        assert_eq!(
            refusal(&["probe", "-", "--pre-source", "-"]),
            "cli-stdin-claimed-twice"
        );
        assert_eq!(
            refusal(&["plan", "-", "--results", "-"]),
            "cli-stdin-claimed-twice"
        );
        assert_eq!(
            analyzed_args(&["plan", "-"]).book.as_deref(),
            Some("-"),
            "THE PAYOFF of `owed-no-flag-defaults-to-stdin`: a `-` book no longer has to name \
             `--results FILE` beside it to free a stream nothing else asked for"
        );
        assert_eq!(
            analyzed_args(&["probe", "-"]).book.as_deref(),
            Some("-"),
            "and probe never wanted the stream at all"
        );
    }

    /// `owed-no-flag-defaults-to-stdin` from the other side: with no `-` anywhere, NOTHING claims
    /// stdin -- which is what makes `-` its only claimant rather than its loudest one.
    #[test]
    fn no_flag_acquires_stdin_implicitly() {
        assert!(analyzed_args(&["plan", "book.sh"]).results.is_none());
        assert_eq!(
            analyzed_args(&["plan", "book.sh", "--results", "-"])
                .results
                .as_deref(),
            Some("-"),
            "the records lane takes stdin only when told to"
        );
    }

    /// The artifact lane's half: `apply --host` used to fall back to stdin, so it could be spelled
    /// with no input at all. Now it must NAME one, and the refusal says which flag is missing.
    #[test]
    fn a_remote_apply_must_name_its_artifact() {
        assert_eq!(
            refusal(&["apply", "--host", "web1.example.net"]),
            "cli-mode-needs-flag"
        );
        assert_eq!(
            analyzed_args(&["apply", "--host", "web1.example.net", "--plan", "-"])
                .plan
                .as_deref(),
            Some("-")
        );
    }
}
