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

#![forbid(unsafe_code)]

pub mod artifact;
pub mod bundle;
pub mod fixpoint;
pub mod kinds;
pub mod provenance;
pub mod results;
pub mod snapshot;
pub mod sourcing;
pub mod survival;
pub mod why;
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
    let digest = world.decision_digest();
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
    arrangement_text(ctx.arrangements(), HELP_ARRANGEMENT, None)
}

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
    /// `--whylog-dir=DIR` (`27V` Lane B): DIR the posthoc-why durable is written to (on a
    /// plan/apply/round-trip run) and read from (`dorc why`). Unset ⇒ the per-user state directory
    /// (`dorc_cli`'s caller resolves it), because the promise is zero-setup: `USER_STORY` has
    /// `dorc why` working "with nothing you had to set up beforehand", and a receipt nobody
    /// remembered to ask for is the only kind that exists on the bad morning.
    pub whylog_dir: Option<String>,
    /// `--no-whylog`: write no durable for this run.
    ///
    /// The escape hatch default-on owes: a receipt is host metadata written unprompted
    /// (`AID-NEEDS:law-whylog-is-sensitive`), so refusing one must be typeable. Per-invocation and
    /// subtractive-only, which is the shape `28D:pay-levers-are-subtractive` demands of anything in
    /// this family — there is no widening sibling and never will be.
    pub no_whylog: bool,
    /// `--whylog=FILE`: the exact durable to replay (`why --last` only).
    pub whylog: Option<String>,
    /// `--last` (`27V` Lane B): replay the most recent durable in `--whylog-dir` through the SAME
    /// kernel instead of the live pipeline (determinism is the replay license).
    ///
    /// Since `28E:lean-why-is-whylog-reconciliation` this is what `dorc why` does ANYWAY when no
    /// record source was named ([`Args::reads_the_receipt`]); the flag survives as a spelling
    /// rather than a switch, because it is printed in committed transcripts and typed in muscle
    /// memory, and it still means something on plan-producing modes. Bundle projection is the
    /// deliberate exception: it consumes authored-before-contact inputs only.
    pub last: bool,
    /// `--all`: the DEEPEST pull tier — every `dorc why` footer already points here, so the flag
    /// exists to make that pointer copy-paste-true (`28E` §7 held-placement-reread).
    ///
    /// What it carries today is the `[unnarrated: <class>]` census
    /// (`28E:prop-unnarrated-is-visible`): the aid plane fails toward narration, so a narrative
    /// class this run MINTED and no render CONSUMED is disclosed rather than silently omitted. The
    /// footer's fuller promise — every link, unselected, exhaustive — is not yet built, since the
    /// render does no link SELECTION to undo.
    pub all: bool,
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
    /// harness/tooling posture, and it is now EXPLICIT: naming `--results` (or `--whylog`, which
    /// names an exact durable) is what selects it.
    ///
    /// Deliberately not "is stdin a pipe": that would be an ambient read at a seat sworn off them
    /// (`io-at-edges-only`), it would make a CI `dorc why` silently answer a different question
    /// than an interactive one, and it would block on a terminal.
    #[must_use]
    pub const fn reads_the_receipt(&self) -> bool {
        reads_the_receipt(self.mode, self.last, self.results.is_some())
    }
}

/// [`Args::reads_the_receipt`] over the parts, so the parser can apply the same rule before it has
/// an `Args` to ask. Two spellings of this predicate would be two answers to "which surface am I".
const fn reads_the_receipt(mode: Mode, last: bool, has_results: bool) -> bool {
    (last && !matches!(mode, Mode::Bundle)) || (matches!(mode, Mode::Why) && !has_results)
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
    let mut whylog_dir: Option<String> = None;
    let mut whylog: Option<String> = None;
    let mut last = false;
    let mut no_whylog = false;
    let mut all = false;
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
        } else if let Some(p) = arg.strip_prefix("--whylog-dir=") {
            whylog_dir = Some(p.to_string());
        } else if arg == "--whylog-dir" {
            whylog_dir = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--whylog-dir", "a directory"))?,
            );
        } else if let Some(path) = arg.strip_prefix("--whylog=") {
            whylog = Some(path.to_owned());
        } else if arg == "--whylog" {
            whylog = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--whylog", "a durable path"))?,
            );
        } else if arg == "--all" {
            all = true;
        } else if arg == "--last" {
            last = true;
        } else if arg == "--no-whylog" {
            no_whylog = true;
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
                "preserved-book-tree" => ArtifactForm::PreservedBookTree,
                other => {
                    return Err(value_not_recognized(
                        "--form",
                        other,
                        "flattened|multipart|preserved-book-tree",
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
                "--whylog-dir",
                "--whylog",
                "--no-whylog",
                "--last",
                "--all",
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
        && !reads_the_receipt(mode, last, results.is_some())
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
    // The emission planner shapes a PLAN, so a mode whose product is something else entirely —
    // `bundle`'s inert archive, `why`'s report — is told so rather than handed an inert flag.
    //
    // `probe` is deliberately NOT in that set. It is the same run's earlier PHASE (the round-trip's
    // half one), analysing the same book from the same inputs and stopping before the plan exists;
    // a form flag there shapes something the phase does not reach, which is inert rather than
    // wrong. Refusing it would also mean an invocation could not carry one set of flags across both
    // phases, which is exactly how the round-trip is driven.
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
    if whylog.is_some() && whylog_dir.is_some() {
        return Err(Diag::new_spanless_site(
            DiagCode::CliFlagsMutuallyExclusive(dorc_aid::diag::CliFlagsMutuallyExclusive {
                first: "--whylog",
                second: "--whylog-dir",
            }),
        ));
    }
    if whylog.is_some() && mode != Mode::Why {
        return Err(Diag::new_spanless_site(DiagCode::CliFlagRequiresMode(
            dorc_aid::diag::CliFlagRequiresMode {
                flag: "--whylog",
                mode: "dorc why",
            },
        )));
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
        whylog_dir,
        no_whylog,
        whylog,
        last,
        all,
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
/// disagree, and "did you mean `--whylog`?" for `--whylog` teaches nothing while hiding the gap.
fn nearest<'a>(word: &str, candidates: &[&'a str]) -> Option<&'a str> {
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
    /// The durable's own start instant on a `--last` replay, this invocation's on a live one, and
    /// `None` when the edge had no clock. A replay carries the ORIGINAL run's instant, never this
    /// moment's — reading a replay's clock here would date the receipt to when it was read.
    pub at: Option<dorc_core::RunInstant>,
    /// Whether this report replays a durable rather than reporting the run that just happened.
    pub replayed: bool,
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
    pub tally: PlanTally,
    /// `--all`: the reader asked for the deepest pull tier.
    pub deepest_tier: bool,
    /// Whether the `[unnarrated:]` census may be asserted over this report at all — the version
    /// coupling (`28E:prop-unnarrated-is-visible`'s caveat). False when a replayed durable's
    /// record stream is not the one this binary's narrative plane was built against.
    pub narratable: bool,
}

/// How much of the plan tally a receipt can honestly state.
///
/// The skipped-count SPLIT is the line the reader needs most — an `elide_by_trusted_claim` skip
/// rests on an author's at-most claim rather than on anything measured, and the two carry different
/// risk — but it is a LICENSE-plane derivation, re-derived through the kernel from the book. A
/// drifted replay (`28F:rul-drift-replay-d1`) has no kernel run behind it: the thin durable stores
/// one disposition word per leaf and nothing else, so there the split is not zero, it is unknown,
/// and rendering the two alike would put a proof-claim on a receipt that holds none.
/// The two states are also the receipt's drift state, deliberately: the split is missing for
/// exactly the reason the chain is, so nothing can set one without the other. D2 — storing the
/// chain so a drifted receipt could still name lines — is REJECTED, which is what makes the
/// coupling permanent rather than a convenience.
#[derive(Debug, Clone, Copy)]
pub enum PlanTally {
    /// A live run or a clean replay: the counts the plan itself produced.
    Derived(dorc_plan::DispositionCounts),
    /// A drifted replay: the durable's own per-leaf dispositions, split unknown.
    DriftedUnsplit {
        /// Leaves the durable recorded as `run`.
        run: usize,
        /// Leaves the durable recorded as `guard`.
        guard: usize,
        /// Leaves the durable recorded as `replace`.
        elide: usize,
    },
}

impl PlanTally {
    /// Whether this receipt reports on a run whose book is no longer the file at its path.
    #[must_use]
    pub const fn is_drifted(self) -> bool {
        matches!(self, PlanTally::DriftedUnsplit { .. })
    }
}

/// The receipt header as one banner: the run's identity, then the indented record of what it read
/// and what it decided.
///
/// The plan tally counts the TYPED disposition, never the rendered word: the words are registry
/// prose meant to churn (`27V:rul-output-form-unwelded`), so a tally keyed on them would silently
/// go wrong the first time someone rewrote one.
#[must_use]
pub fn receipt_banner(ctx: &RenderCtx<'_>, receipt: &Receipt) -> Node<Face> {
    let when = match (receipt.at, receipt.replayed) {
        (Some(at), false) => Said::words(
            "why-receipt-when-live",
            &[&dorc_aid::instant::date_time_text(at)],
        ),
        (Some(at), true) => Said::words(
            "why-receipt-when-replayed",
            &[&dorc_aid::instant::date_time_text(at)],
        ),
        (None, _) => Said::words("why-receipt-when-undated", &[]),
    };
    let tally = match receipt.tally {
        PlanTally::Derived(counts) if counts.elide_by_trusted_claim == 0 => Said::words(
            "why-receipt-plan-tally-by-proof",
            &[
                &counts.run.to_string(),
                &counts.guard.to_string(),
                &counts.elide.to_string(),
                &counts.elide_by_proof.to_string(),
            ],
        ),
        PlanTally::Derived(counts) => Said::words(
            "why-receipt-plan-tally",
            &[
                &counts.run.to_string(),
                &counts.guard.to_string(),
                &counts.elide.to_string(),
                &counts.elide_by_proof.to_string(),
                &counts.elide_by_trusted_claim.to_string(),
            ],
        ),
        PlanTally::DriftedUnsplit { run, guard, elide } => Said::words(
            "why-receipt-plan-tally-unsplit",
            &[&run.to_string(), &guard.to_string(), &elide.to_string()],
        ),
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
    if receipt.tally.is_drifted() {
        // Adjacent to the row it qualifies: the digest above is the RUN's, not the file's now.
        body.push(receipt_row(
            ctx,
            &Said::words("why-receipt-book-drifted", &[]),
        ));
    }
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

/// Everything a drifted receipt renders, and NOTHING that came from the current filesystem.
///
/// `28F:rul-drift-replay-d1`. Once the recorded book digest disagrees with the file at the recorded
/// path, every downstream read of that file is a read of somebody else's book: the kernel would
/// re-derive a chain for lines the run never saw, and naming them would be `271:rul-sin-ordering`'s
/// pope-sin — a mis-attribution — rather than a missing answer. So the drifted path collects the
/// durable's own scalars HERE and the render is structurally unable to reach anything else.
///
/// D2 (storing the chain so a drifted receipt could still name lines) is REJECTED: the thin durable
/// never carries book structure, and the git line cannot rescue line-naming under the
/// annotation-tier fence.
#[derive(Debug)]
pub struct DriftedReceipt {
    /// The session host id the durable recorded.
    pub host: String,
    /// The book path the durable recorded.
    pub book_path: String,
    /// The digest the RUN was keyed on — not the file's digest now.
    pub book_digest: String,
    /// The oracle paths the durable recorded, in argv order.
    pub oracle_paths: Vec<String>,
    /// The consent flag as the ORIGINAL invocation spelled it, read back off the recorded argv.
    pub risk_profile: Option<&'static str>,
    /// The instant the recorded run started.
    pub started_at: Option<dorc_core::RunInstant>,
    /// The per-leaf predicted dispositions the durable stored, folded to a tally.
    pub tally: PlanTally,
}

/// Collect a drifted receipt out of an admitted durable, and out of NOTHING else.
///
/// The one constructor, shared by the live `dorc why --last` edge and the loom driver, so a
/// committed transcript proves the render the binary prints rather than a re-derivation of it.
#[must_use]
pub fn drifted_receipt(envelope: &dorc_plan::whylog::UnscopedWhylogEnvelope) -> DriftedReceipt {
    DriftedReceipt {
        host: envelope.claims().host().to_owned(),
        book_path: envelope.recorded_book_path().as_str().to_owned(),
        book_digest: envelope.claims().book_digest().to_owned(),
        oracle_paths: envelope
            .recorded_oracles()
            .iter()
            .map(|oracle| oracle.path().as_str().to_owned())
            .collect(),
        risk_profile: envelope
            .argv()
            .iter()
            .any(|word| word == CONSENT_FLAG)
            .then_some(CONSENT_FLAG),
        started_at: envelope.claims().started_at(),
        tally: recorded_tally(envelope.apply()),
    }
}

/// Fold a durable's recorded apply report into the tally a drifted receipt may state.
///
/// Keyed on the stored WORD, which is the only shape the durable has — the typed `Disposition` it
/// came from is unreachable here (re-deriving it needs the book). An unrecognized word is counted
/// nowhere rather than guessed into a bucket: the tally under-reports instead of mis-reporting
/// (`271:rul-sin-ordering`), and the parser's own `disposition_valid` already refuses the case.
#[must_use]
pub fn recorded_tally(apply: &[dorc_plan::whylog::ApplyLine]) -> PlanTally {
    let count = |tag: &str| apply.iter().filter(|line| line.disposition == tag).count();
    PlanTally::DriftedUnsplit {
        run: count("run"),
        guard: count("guard"),
        elide: count("replace"),
    }
}

/// The DEGRADED `dorc why` render for a replay whose book has drifted (`28F:rul-drift-replay-d1`).
///
/// The bad morning's worst case is the admin who edited the book before asking why, and the answer
/// they used to get was a refusal with nothing behind it. The receipt itself survives drift — it is
/// header, inventory and tally, all minted by the controller and stored — so it renders, with the
/// drift stated in it. What cannot survive is everything keyed to SOURCE: the chain re-derives
/// through the kernel from the book, and leaf-to-line needs the AST, so ANALYSIS and every chain
/// are suppressed and say so in their place.
///
/// An ADDRESSED query gets only the explanation, matching the un-drifted addressed form, which also
/// renders no banner: the reader asked about a line, and the honest answer names no line at all.
/// No footer either — its "filtered for presumed relevance" is a claim about selection, and nothing
/// here was selected.
#[must_use]
pub fn drifted_why_parts(
    ctx: &RenderCtx<'_>,
    address: Option<&str>,
    drifted: &DriftedReceipt,
) -> dorc_aid::tagged::RenderParts {
    if let Some(address) = address {
        return why_parts(
            vec![paragraph(
                ctx,
                &Said::words("why-drift-address-unanswerable", &[address]),
                "why-drift-address-unanswerable",
            )],
            0,
        );
    }
    let receipt = Receipt {
        at: drifted.started_at,
        replayed: true,
        host: drifted.host.clone(),
        book: drifted.book_path.clone(),
        book_digest: drifted.book_digest.clone(),
        // Never resolved under drift: the git line answers for the file on disk, and saying that
        // file matches HEAD would attach a provenance claim to bytes the run never read.
        at_head: None,
        oracles: drifted.oracle_paths.clone(),
        risk_profile: drifted.risk_profile,
        tally: drifted.tally,
        deepest_tier: false,
        narratable: false,
    };
    why_parts(
        vec![
            receipt_banner(ctx, &receipt),
            registry_paragraph(ctx, "why-drift-analysis-suppressed"),
        ],
        0,
    )
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
            analyzed(&["why", "--whylog=run.whylog"]).reads_the_receipt(),
            "naming an exact durable is still reading a receipt"
        );
        assert!(
            analyzed(&["plan", "--last", "book.sh"]).reads_the_receipt(),
            "`--last` survives as a spelling and still means replay on the other modes"
        );
        assert!(
            !analyzed(&["plan", "book.sh"]).reads_the_receipt(),
            "plan without --last is a live analysis, untouched by the fold"
        );
    }

    /// Both spellings of a value-taking flag parse, and nothing in the table can suggest itself:
    /// `--whylog` had only the `=` form, so the space form answered "did you mean `--whylog`?".
    #[test]
    fn a_flag_never_suggests_the_word_that_was_typed() {
        assert_eq!(
            analyzed(&["why", "--whylog", "run.whylog"])
                .whylog
                .as_deref(),
            Some("run.whylog")
        );
        assert_eq!(
            analyzed(&["why", "--whylog=run.whylog"]).whylog.as_deref(),
            Some("run.whylog")
        );
        assert_eq!(nearest("--whylog", &["--whylog", "--whylog-dir"]), None);
        assert_eq!(
            nearest("--whylo", &["--whylog", "--book"]),
            Some("--whylog")
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
        assert!(!analyzed(&["bundle", "book.sh", "--last"]).reads_the_receipt());
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
    /// it sits. The old reading only took it when it LED, so `dorc why --last book.sh:9` filed the
    /// address as a positional book and answered the unargumented aggregate at rc 0 — the user asked
    /// about one line and silently got the whole-run surface, with nothing to notice.
    #[test]
    fn a_why_address_is_found_after_a_flag() {
        let args =
            |raw: &[&str]| match parse_args_from(raw.iter().map(|a| (*a).to_owned()).collect()) {
                Ok(Invocation::Analyze(args)) => args,
                other => panic!("expected an analyze invocation, got {other:?}"),
            };
        let leading = args(&["why", "book.sh:9", "--book=book.sh"]);
        assert_eq!(leading.why_address.as_deref(), Some("book.sh:9"));

        let after_last = args(&["why", "--last", "book.sh:9"]);
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
