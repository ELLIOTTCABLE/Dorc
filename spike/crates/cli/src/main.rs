//! `dorc` — the thin spike CLI: the apply-2 round-trip over real files, as a
//! multi-mode plan/apply surface (ui-A, ru-25 / ru-20 ui-3).
//!
//! Reads a book + oracle files, runs the pure analyzer kernel, and emits one of the
//! engine's distinct user-facing behavioral modes. No executor — it *compiles* a probe
//! and an apply; it runs neither. The simulated host's answers arrive on stdin (in a
//! real deployment those come from running the probe on the host).
//!
//! ```text
//! usage: dorc [<mode>] --book=<book.sh> [-o <oracle.sh>]... [--debug-argv]
//!   modes:
//!     probe      emit the read-only probe artifact (phase 1) to stdout; reads no stdin
//!     plan       PREVIEW (ru-20 ui-3): the eliding-apply to stdout, PLUS the why-lens +
//!                diagnostics doubly-emitted to stderr (the cited-sections render surface)
//!     apply      the byte-floored, receipt-free shippable apply artifact to stdout;
//!                stderr carries ONLY error-severity diagnostics + the decision-digest
//!     <none>     the legacy round-trip: probe THEN apply on stdout, full disclosure on
//!                stderr — the shape the e2e harness drives (kept verbatim, do not break)
//!   stdin : probe results (plan/apply/round-trip), one per line —
//!           `site <leafid> effect=<holds|absent|cant-tell> rc=<n>`
//!   stdout: the selected mode's artifact(s); stderr: diagnostics / why-lens / digest
//!           + the plan-summary (every plan-building mode; the yardstick's metric):
//!           `dorc: plan-summary sites=<N> elide=<E> omit=<O> guard=<G> run=<R>`
//!           where sites == elide+omit+guard+run; elide = provably-skipped lines,
//!           omit = fold-dead branches, guard = 0 until the Stage-3 guard tier, run
//!           = the rest. Stable grammar (a parse target — plans/240 Stage-1 yardstick).
//! ```
//!
//! rec-1 TWO SURFACES (ru-12 + ru-20, spike/CLAUDE.md): the shipped `.sh` artifact on
//! stdout is byte-floored and receipt-free — `plan` and `apply` emit BYTE-IDENTICAL
//! apply bytes. The only difference is the RENDER surface (stderr): `plan` overlays the
//! per-line why-lens + advisory disclosure there; `apply` (the off-ramp) suppresses the
//! advisory plane, keeping only the error floor + digest. The why-lens is never woven
//! into the artifact bytes in any mode.
//!
//! Round-20 task-D1 (the WIRE — `inv-site-keyed-results`): the probe is a real,
//! self-reporting artifact; its results-records are keyed by command **site** (the
//! stable `LeafId`), not by fact. The simulated host's answers (the e2e
//! `probe-results.txt`, a stand-in for running the rendered probe remotely) are now
//! the site-keyed records the probe itself emits.
//!
//! I/O edge: `inv-determinism` exempts `cli`; the analyzer kernel it calls is pure.
//! Diagnostics go to stderr so stdout stays the artifact. The mode dispatch is a thin
//! driver over ONE pipeline call ([`analyze`]) — no kernel logic moves here (the
//! thin-driver mandate, crates/cli/CLAUDE.md).

#![forbid(unsafe_code)]
// The cli is the sanctioned I/O edge (workspace Cargo.toml: "I/O-edge crates may
// `#[expect]` these at the crate root, with reason"): stdout carries the
// probe-then-apply artifact, stderr carries diagnostics. The kernel it drives
// stays print-free. Not a seeded-ratchet expect — this one is permanent for the
// binary's edge.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "cli is the I/O edge: probe/apply to stdout, diagnostics to stderr; the kernel stays print-free"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::process::ExitCode;

mod source_match;
mod whylog_store;

use dorc_aid::diag::{
    AidUnloadedSiblingOracle, CarriedAcrossSubstrateAxis, DanglingReference, DerivFamilyIncomplete,
    Diag, DiagCode, EscalationPolicy, FootprintIncoherent, ReachesConflict,
    ReachesProviderCollision, ResolverConflict, ResolverProviderCollision, TouchesEscalated,
    WrappedSiteAdoptionHint,
};
use dorc_aid::weave::Face;
use dorc_aid::{CollapseKind, CollapseNarrative, Knowability, Severity, SpeechAct};
use dorc_core::{Interner, Observable, OutBytes, Predicted, ProvArena, Rc, Symbol, Verdict};
use weft::{
    Banner, Branch, CodeBlock, CodeCell, CodeLine, Document, Join, LabeledRow, Literalness, Node,
    NodeKind, Paragraph, Payload, PointerLine, Quoting, Run, Section, SpeakerRow, Truncation,
};

// The invocation surface lives in the crate's INTERNAL lib target (`289:rul-worldless-route-
// honest-trigger`) so the loom harness can fire the real parser; this bin keeps every I/O edge.
use dorc_cli::{Args, Invocation, LintArgs, LintFormat, Mode, humane_read_error, parse_args_from};

/// A usage/argument error, or an unreadable input file (the classic getopt convention).
const EXIT_USAGE: u8 = 2;
/// A parse-error / unmodeled book (`inv-top-reject`): the book carries a construct dorc
/// cannot model. The artifact still ships, but the exit signals partial understanding
/// (ack-1). First of the reserved 10..=19 dorc-semantic fast-fail range.
const EXIT_BOOK_UNMODELED: u8 = 10;
/// A dual-peel incoherent wrapper oracle (`273` §5): a wrapper authoring BOTH `__predict`
/// (peeling) and `__lend_map` whose `"$@"` reach DIFFERENT tail positions — the
/// declarations-genuinely-contradict category, a genuine plan-time, pre-network fail-fast
/// (`rul-proven-mutation-fails-fast` posture). The artifact still ships (the fail-fast is loud,
/// not a crash); the exit stops a `dorc … && deploy` chain. Second of the 10..=19 range.
const EXIT_WRAPPER_INCOHERENT: u8 = 11;

/// `dorc lint`: findings AT OR ABOVE the `--fail-on` threshold were reported (`27R` §5 exit
/// trichotomy). Distinct from clean (0) and from operational (below); shares linter convention.
const EXIT_LINT_FINDINGS: u8 = 1;
/// `dorc lint`: an OPERATIONAL error — the lint itself is compromised, distinct from both clean and
/// findings (`27R` §5, §8b): zero lintable files, an `--expect-files` mismatch, or a `--require-tools`
/// absence. NOT in the 10..=19 dorc-semantic family (a ⊤-reject book is a FINDING for lint, `27R` §5).
/// Numbered 3 (tc-lint-operational-exit-code — golangci-lint uses 3=Failure, shellcheck 3=bad-invoke;
/// the conservative lean, flagged for the human).
const EXIT_LINT_OPERATIONAL: u8 = 3;

/// The outcome of a completed analysis run — the process exit code (ack-1). `Complete` is the
/// ordinary success; `BookUnmodeled` still emitted the artifact but the book carried an
/// `inv-top-reject` construct, so the process fast-fails with [`EXIT_BOOK_UNMODELED`].
enum RunOutcome {
    /// The analysis completed cleanly ⇒ exit 0.
    Complete,
    /// The book carried a parse/CFG ⊤-reject ⇒ the artifact shipped, but exit [`EXIT_BOOK_UNMODELED`].
    BookUnmodeled,
    /// A wrapper oracle's `__predict`/`__lend_map` peels are dual-peel incoherent (`273` §5) ⇒ the
    /// artifact shipped, but exit [`EXIT_WRAPPER_INCOHERENT`] (fail-fast).
    WrapperIncoherent,
    /// Host evidence failed admission before any decision artifact could be built.
    IngressRefused,
}

fn main() -> ExitCode {
    match parse_args() {
        Ok(Invocation::Help) => {
            print!("{}", dorc_cli::help_text());
            std::io::stdout().flush().ok();
            ExitCode::SUCCESS
        }
        Ok(Invocation::Version) => {
            println!("dorc {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(Invocation::Strip(path)) => match strip_command(&path) {
            Ok(()) => ExitCode::SUCCESS,
            Err(diag) => {
                report_invocation_error(&diag);
                ExitCode::from(EXIT_USAGE)
            }
        },
        Ok(Invocation::Lint(args)) => lint_command(&args),
        Ok(Invocation::Analyze(args)) => match run(&args, &mut RunClock::for_invocation()) {
            Ok(RunOutcome::Complete) => ExitCode::SUCCESS,
            Ok(RunOutcome::BookUnmodeled) => ExitCode::from(EXIT_BOOK_UNMODELED),
            Ok(RunOutcome::WrapperIncoherent) => ExitCode::from(EXIT_WRAPPER_INCOHERENT),
            Ok(RunOutcome::IngressRefused) => ExitCode::from(12),
            Err(diag) => {
                report_invocation_error(&diag);
                ExitCode::from(EXIT_USAGE)
            }
        },
        Err(diag) => {
            report_invocation_error(&diag);
            ExitCode::from(EXIT_USAGE)
        }
    }
}

/// Minimal hand-rolled parsing (no `clap` dep yet): resolve the whole invocation. `-h`/`--help`
/// and `--version` win unconditionally (a pre-scan — ack-1 help-is-success, so a help request
/// beats a malformed flag) and return the stdout-and-exit-0 variants. Otherwise: an OPTIONAL
/// leading mode token (`probe`/`plan`/`apply`; absent ⇒ [`Mode::RoundTrip`]), then `--book=PATH` /
/// `--book PATH`, `-o PATH` / `-oPATH` / `--oracle PATH` (repeatable), `--debug-argv`,
/// `--risk-faultless-skips`. The mode is positional-first ONLY (a bare word after flags is still an
/// error) so the legacy `dorc --book=… < results` invocation parses unchanged.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn parse_args() -> Result<Invocation, Diag> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    parse_args_from(raw)
}

/// The ONE print seat for an invocation error (`288` §6). Body-only: an argv has no span, so the
/// framed render would draw a caret at nothing. The `dorc: ` prefix and the usage synopsis are
/// CHROME the seat owns, never part of a catalog register — which is why 20 codes' prose does not
/// each carry a copy of the usage text (`291` §5d parks usage/help for the arrangement round).
/// The `--shim-dir` materialization edge's write failures. The one invocation-surface code the
/// `291` §5a inventory did not name — it fell out of giving `run` a single error type.
fn shim_dir_unwritable(path: &str, err: &std::io::Error) -> Diag {
    Diag::new_spanless_site(DiagCode::CliShimDirUnwritable(
        dorc_aid::diag::CliShimDirUnwritable {
            path: path.to_owned(),
            detail: err.to_string(),
        },
    ))
}

/// The lint OPERATIONAL print seat (`27R` §5 exit trichotomy): the lint itself is compromised, so
/// the message rides the `dorc: lint: ` chrome and the caller returns `EXIT_LINT_OPERATIONAL`.
fn report_lint_operational(diag: &Diag) {
    eprintln!(
        "dorc: lint: {}",
        dorc_aid::diag::render_body(diag, &Interner::default())
    );
}

/// One registry-sourced chrome line, its computed values interleaved between the entry's words
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`). These stderr lines have a registry
/// HOME but not yet an editable face: no case drives them, so their words are edited in the lock
/// until a page case exists for them.
fn chrome(slug: &str, values: &[&str]) -> String {
    dorc_aid::arrangement::arrangement_sentence(
        &dorc_aid::arrangement::CONST_ARRANGEMENTS,
        slug,
        None,
        values,
    )
}

fn report_invocation_error(diag: &Diag) {
    eprintln!(
        "dorc: {}",
        dorc_aid::diag::render_body(diag, &Interner::default())
    );
    eprintln!("{}", dorc_cli::usage_text());
}

/// `dorc strip <path>` (`27D` rider-dorc-sh-unbuilt / `274` §13): read the file, erase every dorc
/// dialect construct (parser-backed — [`dorc_oracle::strip_file`]), print runnable stock sh to
/// stdout. An unmarked file passes through byte-identical (idempotent). Pure — the strip carries no
/// diagnostics today, but any it grows are reported to stderr so stdout stays exactly the artifact.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn strip_command(path: &str) -> Result<(), Diag> {
    let src = std::fs::read_to_string(path).map_err(|e| humane_read_error("source", path, &e))?;
    let mut interner = Interner::default();
    let stripped = dorc_oracle::strip_file(&mut interner, &src);
    for d in &stripped.diags {
        eprintln!("dorc: strip: {}", dorc_aid::diag::render_body(d, &interner));
    }
    print!("{}", stripped.value);
    std::io::stdout().flush().ok();
    Ok(())
}

/// Read + CONCATENATE the book(s) into one analyzed unit (`\n`-joined so no two files' lines
/// merge — multi-book concatenation-as-one-unit). Humane per-file errors.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn read_books(books: &[String]) -> Result<String, Diag> {
    let mut out = String::new();
    for (i, path) in books.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(
            &std::fs::read_to_string(path).map_err(|e| humane_read_error("book", path, &e))?,
        );
    }
    Ok(out)
}

/// Resolve the oracle PATHS (ack-6): the explicit `-o` list first, then every `*.oracle.sh` in
/// each `--oracle-dir` (glob-sorted for determinism — the cli is the I/O edge, but the ORDER it
/// hands the kernel must be stable). A directory that cannot be read is a humane error.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn resolve_oracle_paths(oracles: &[String], oracle_dirs: &[String]) -> Result<Vec<String>, Diag> {
    let mut paths: Vec<String> = oracles.to_vec();
    for dir in oracle_dirs {
        let entries =
            std::fs::read_dir(dir).map_err(|e| humane_read_error("oracle directory", dir, &e))?;
        let mut found: Vec<String> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.to_string_lossy().ends_with(".oracle.sh"))
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        found.sort();
        paths.extend(found);
    }
    Ok(paths)
}

/// The REAL external-tool runner at the cli edge (`27R` §1 dir-runner-is-the-di-seam): the ONLY
/// non-hermetic part, kept out of the deterministic `dorc-lint` crate. Feeds the stripped bytes on the
/// tool's stdin (so the tool sees `-`/stdin, never a temp path — `dir-paths-stay-yours`).
struct SubprocessRunner;

impl dorc_lint::ExternalToolRunner for SubprocessRunner {
    fn available(&self, tool: &str) -> bool {
        tool_on_path(tool)
    }

    fn run(&self, tool: &str, args: &[&str], stdin: &[u8]) -> dorc_lint::ToolRun {
        use std::process::{Command, Stdio};
        let mut child = match Command::new(tool)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            // Vanished since the availability probe: rc 127, which the adapter turns into a finding.
            Err(e) => {
                return dorc_lint::ToolRun {
                    rc: 127,
                    stdout: Vec::new(),
                    stderr: e.to_string().into_bytes(),
                };
            }
        };
        if let Some(mut si) = child.stdin.take() {
            // A tool reading only a prefix of stdin closes early (BrokenPipe) — not an error here.
            let _ = si.write_all(stdin);
        }
        match child.wait_with_output() {
            Ok(out) => dorc_lint::ToolRun {
                rc: out.status.code().unwrap_or(-1),
                stdout: out.stdout,
                stderr: out.stderr,
            },
            Err(e) => dorc_lint::ToolRun {
                rc: -1,
                stdout: Vec::new(),
                stderr: e.to_string().into_bytes(),
            },
        }
    }
}

/// Is `tool` an executable on `PATH`? A `which`-style scan (no process spawn) — cross-platform via
/// `PATHEXT` on Windows (an extensionless script like a POSIX `shellcheck` won't be CreateProcess-able
/// on Windows, so a `.exe`/`.cmd` is required there; see `27S` for the e2e consequence).
fn tool_on_path(tool: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    for dir in std::env::split_paths(&path) {
        for ext in executable_exts() {
            if dir.join(format!("{tool}{ext}")).is_file() {
                return true;
            }
        }
    }
    false
}

/// The executable suffixes to try: just `""` on unix; `""` plus each `PATHEXT` entry on Windows.
fn executable_exts() -> Vec<String> {
    let mut exts = vec![String::new()];
    if cfg!(windows)
        && let Ok(pathext) = std::env::var("PATHEXT")
    {
        for e in pathext.split(';') {
            let e = e.trim();
            if !e.is_empty() {
                exts.push(e.to_owned());
            }
        }
    }
    exts
}

/// `dorc lint` driver (`27R` §5): resolve inputs, run `dorc-lint`, render, and compute the exit
/// trichotomy (0 clean / 1 findings-at-or-above / operational distinct from both). Operational checks
/// take precedence over the findings threshold (a compromised run must not read as a clean/findings
/// signal — `27R` §8 delta-exit-trichotomy-sharpened).
#[expect(
    clippy::too_many_lines,
    reason = "one linear exit-trichotomy driver: resolve inputs, run, render, then the operational checks in precedence order; splitting it would scatter the ONE precedence the exit codes encode"
)]
fn lint_command(args: &LintArgs) -> ExitCode {
    if args.list_sources {
        for s in dorc_lint::list_sources() {
            let describe = dorc_aid::arrangement::arrangement_text(
                &dorc_aid::arrangement::CONST_ARRANGEMENTS,
                s.describe_arrangement,
                None,
            );
            println!("{:<22} [{}]  {describe}", s.name, s.rung);
        }
        return ExitCode::SUCCESS;
    }
    let inputs = match read_lint_inputs("file", &args.files) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!(
                "dorc: lint: {}",
                dorc_aid::diag::render_body(&msg, &Interner::default())
            );
            return ExitCode::from(EXIT_LINT_OPERATIONAL);
        }
    };
    let oracle_paths = match resolve_oracle_paths(&args.oracles, &args.oracle_dirs) {
        Ok(p) => p,
        Err(diag) => {
            report_invocation_error(&diag);
            return ExitCode::from(EXIT_USAGE);
        }
    };
    let oracles = match read_lint_inputs("oracle", &oracle_paths) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!(
                "dorc: lint: {}",
                dorc_aid::diag::render_body(&msg, &Interner::default())
            );
            return ExitCode::from(EXIT_LINT_OPERATIONAL);
        }
    };
    let options = dorc_lint::LintOptions {
        tools_enabled: args.tools_enabled,
    };
    let only = (!args.sources.is_empty()).then_some(args.sources.as_slice());
    let report = if let [input] = inputs.as_slice()
        && !args.tools_enabled
        && oracles.is_empty()
        && only.is_none()
    {
        dorc_lint::lint_materialized_source(
            input.path.clone(),
            input.src.clone(),
            dorc_lint::SourcePolicy {
                tools_enabled: false,
            },
        )
        .report()
        .clone()
    } else {
        dorc_lint::lint(&inputs, &oracles, options, &SubprocessRunner, only)
    };

    // Zero lintable files is OPERATIONAL, never clean (`27R` §8b); the jsonl envelope still ships.
    if inputs.is_empty() {
        if args.format == LintFormat::Jsonl {
            print!("{}", dorc_lint::render::render_jsonl(&report));
            std::io::stdout().flush().ok();
        }
        report_lint_operational(&Diag::new_spanless_site(DiagCode::LintNoLintableFiles(
            dorc_aid::diag::LintNoLintableFiles,
        )));
        return ExitCode::from(EXIT_LINT_OPERATIONAL);
    }

    match args.format {
        LintFormat::Human => print!(
            "{}",
            dorc_lint::render::render_human_parts_at(&report, args.verbosity).text()
        ),
        LintFormat::Jsonl => print!("{}", dorc_lint::render::render_jsonl(&report)),
    }
    std::io::stdout().flush().ok();

    if let Some(want) = args.expect_files
        && inputs.len() != want
    {
        report_lint_operational(&Diag::new_spanless_site(DiagCode::LintFileCountDrift(
            dorc_aid::diag::LintFileCountDrift {
                expected: want,
                found: inputs.len(),
            },
        )));
        return ExitCode::from(EXIT_LINT_OPERATIONAL);
    }
    if args.require_tools {
        let absent: Vec<&str> = report
            .coverage
            .sources
            .iter()
            .filter(|s| s.status == dorc_lint::SourceStatus::Absent)
            .map(|s| s.name)
            .collect();
        if !absent.is_empty() {
            report_lint_operational(&Diag::new_spanless_site(
                DiagCode::LintRequiredToolsMissing(dorc_aid::diag::LintRequiredToolsMissing {
                    tools: absent.join(", "),
                }),
            ));
            return ExitCode::from(EXIT_LINT_OPERATIONAL);
        }
    }
    if report.count_at_or_above(args.fail_on) > 0 {
        return ExitCode::from(EXIT_LINT_FINDINGS);
    }
    ExitCode::SUCCESS
}

/// Read a set of paths into [`dorc_lint::LintInput`]s; an unreadable file is a hard error (the lint
/// cannot lint what it cannot read — an operational failure, `27R` §8b). `kind` labels the humane error.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn read_lint_inputs(kind: &str, paths: &[String]) -> Result<Vec<dorc_lint::LintInput>, Diag> {
    let mut inputs = Vec::new();
    for path in paths {
        let src = std::fs::read_to_string(path).map_err(|e| humane_read_error(kind, path, &e))?;
        inputs.push(dorc_lint::LintInput {
            path: path.clone(),
            src,
        });
    }
    Ok(inputs)
}

/// Materialize the per-run PATH shim files into `dir` (`274` §5 / `27L` task-14). `files` is the
/// deterministic kernel product ([`dorc_plan::ProbePlan::shim_files`]); this is the I/O half at the
/// cli edge (`io-at-edges-only`): create the dir, write each `(name, content)`, mark it executable so
/// a `sudo -n <inner-check>` can exec the guest across the wrapper boundary. On unix the executable
/// bit is set here; on other platforms (msys) the exec permission is supplied by the session harness
/// (the e2e runner `chmod +x`s them), so a plain write suffices and this stays cross-platform.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn materialize_shim_dir(dir: &str, files: &BTreeMap<String, String>) -> Result<(), Diag> {
    if files.is_empty() {
        return Ok(()); // wrapper-free / already-answered run — nothing to materialize.
    }
    std::fs::create_dir_all(dir).map_err(|e| shim_dir_unwritable(dir, &e))?;
    for (name, content) in files {
        let path = std::path::Path::new(dir).join(name);
        std::fs::write(&path, content)
            .map_err(|e| shim_dir_unwritable(&path.display().to_string(), &e))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            // Same edge, same world-state as the write above (the shim dir cannot be made
            // usable), so it carries the same code rather than minting a grammar-driven sibling
            // — `AID-NEEDS:law-codes-vary-by-world-not-grammar`. The io error rides `detail`.
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
                .map_err(|e| shim_dir_unwritable(&path.display().to_string(), &e))?;
        }
    }
    Ok(())
}

/// The run's instant source — the DI seam for wall clock (`io-at-edges-only`). It lives HERE, in
/// the binary, and nowhere else: the analyzer kernel owns no clock type at all, so no kernel
/// signature can accept one and no kernel path can "reach for a clock to help". Only
/// [`dorc_core::RunInstant`] values (already read) cross inward.
///
/// Nondeterminism enters ONCE, at [`system`](RunClock::system) — the single wall-clock read in the
/// product, exactly as `records::Nonce` is minted once at this edge and DI'd inward
/// (`inv-determinism`: nondeterminism is seeded and injected, never ambient). Everything after is a
/// deterministic tick, so a seeded DST clock and the production clock are the same code path.
///
/// [`Absent`](RunClock::Absent) is a first-class "no clock here", not a failure mode: a replayed
/// durable does not carry the original run's per-record observation times, and re-stamping them
/// from the REPLAY's clock would present this moment as the original measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
enum RunClock {
    /// Yields `at`, then advances by `step_millis`. Production reads a whole record stream in one
    /// slurp, so it ticks by zero — every record of one read genuinely shares one instant. A DST
    /// seed supplies a non-zero step to make per-record instants distinguishable.
    Ticking {
        at: dorc_core::RunInstant,
        step_millis: u64,
    },
    /// No clock: every read is `None`.
    Absent,
    /// The instants a durable RECORDED, keyed by the record ordinal they belong to.
    ///
    /// Not a clock at all, which is the point: a replay must date its records from the run that
    /// made them, and reading any live clock here would present the moment of reading as the
    /// moment of measurement. An ordinal the durable carries no instant for answers `None`.
    Recorded(BTreeMap<u64, dorc_core::RunInstant>),
}

/// The harness's clock pin (`rul-fixture-identity-never-production`) — Unix milliseconds, and the
/// ONE substitution point for the run's instant, exactly as `records::Framing::spike` is the one
/// substitution point for the run's nonce/host.
///
/// It exists because the why surface now DATES its output: a receipt header and a `reported` row's
/// run-instant are wall-clock values, so a committed transcript could otherwise never be a
/// fixpoint. A rendered-but-wrong timestamp was the alternative and is strictly worse — dating a
/// receipt wrongly is mis-attribution, the top of `271:rul-sin-ordering` — so the real clock stays
/// the default and the pin is something a harness must deliberately set.
const FIXTURE_CLOCK_ENV: &str = "DORC_FIXTURE_CLOCK_MS";

impl RunClock {
    /// The clock this invocation runs on: the harness pin when one is set, else the real one.
    /// Read at the process edge, once (`io-at-edges-only`).
    fn for_invocation() -> Self {
        match std::env::var(FIXTURE_CLOCK_ENV)
            .ok()
            .as_deref()
            .map(str::parse::<u64>)
        {
            Some(Ok(millis)) => Self::Ticking {
                at: dorc_core::RunInstant(millis),
                step_millis: 0,
            },
            Some(Err(_)) => Self::Absent,
            None => Self::system(),
        }
    }

    /// The ONE wall-clock read. A clock the platform cannot place after the epoch answers
    /// [`Absent`](RunClock::Absent) rather than saturating to a fabricated zero (`inv-no-throw`).
    fn system() -> Self {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .and_then(|d| u64::try_from(d.as_millis()).ok())
            .map_or(Self::Absent, |millis| Self::Ticking {
                at: dorc_core::RunInstant(millis),
                step_millis: 0,
            })
    }

    fn now(&mut self) -> Option<dorc_core::RunInstant> {
        match self {
            Self::Ticking { at, step_millis } => {
                let read = *at;
                *at = dorc_core::RunInstant(read.0.saturating_add(*step_millis));
                Some(read)
            }
            Self::Absent | Self::Recorded(_) => None,
        }
    }

    /// The instant belonging to the record at `ordinal`.
    ///
    /// A live run reads its own clock and ignores the ordinal — the reading IS the record's
    /// arrival. A replay looks the ordinal up, because its records arrived once, already, and the
    /// only honest answer is the one that run wrote down.
    fn at(&mut self, ordinal: u64) -> Option<dorc_core::RunInstant> {
        match self {
            Self::Recorded(instants) => instants.get(&ordinal).copied(),
            Self::Ticking { .. } | Self::Absent => self.now(),
        }
    }
}

#[expect(
    clippy::too_many_lines,
    clippy::result_large_err,
    reason = "the top-level pipeline driver: lift → analyze → probe → plan → render, one linear sequence with mode-routing; splitting it into sub-drivers would scatter the ONE call-shape the thin-driver mandate keeps here. The Err is a full `Diag` on a once-per-process path"
)]
fn run(args: &Args, clock: &mut RunClock) -> Result<RunOutcome, Diag> {
    let mut interner = Interner::default();
    let mode = args.mode;
    // rec-1 advisory routing: `plan` and the legacy round-trip overlay the FULL advisory plane
    // on stderr (warnings, notes, the why-lens, the unresolvable readout); `apply` (the
    // off-ramp shippable) suppresses it, keeping only the error floor + digest. `probe`'s
    // stage diagnostics are advisory-or-error like any analysis run. tc-apply-receipt-floor:
    // WHERE this line falls (advisory-suppressed but error-kept, digest-kept) is the
    // load-bearing surface judgment — flagged to the conductor, not silently settled.
    let advisory = !matches!(mode, Mode::Apply);

    let replay = if args.reads_the_receipt() {
        match load_whylog_replay(args, advisory)? {
            ReplayLoad::Admitted(replay) | ReplayLoad::NoObservation(replay) => Some(replay),
            ReplayLoad::Refused => return Ok(RunOutcome::IngressRefused),
        }
    } else {
        None
    };

    // ---- the shared, pure pipeline (one call-shape for every mode — the thin-driver
    // mandate: no mode branches the kernel; only the stdout/stderr ROUTING below differs) ----

    let oracle_paths = match &replay {
        Some(r) => r.oracle_paths.clone(),
        None => resolve_oracle_paths(&args.oracles, &args.oracle_dirs)?,
    };
    let oracle_srcs: Vec<String> = oracle_paths
        .iter()
        .map(|p| std::fs::read_to_string(p).map_err(|e| humane_read_error("oracle", p, &e)))
        .collect::<Result<_, _>>()?;
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();

    // The book-free oracle-side lints, factored into one entry the lint rung-oracle-solo lane also
    // uses (`27S:seam-oracle-validate-factoring`); `wrapper_incoherent` is the pre-network fail-fast.
    let validation = dorc_oracle::validate::validate(&mut interner, &oracle_refs);
    let wrapper_incoherent = validation.wrapper_incoherent;
    for stage in &validation.stages {
        let source = stage
            .file
            .and_then(|i| Some((oracle_paths.get(i)?.as_str(), oracle_srcs.get(i)?.as_str())));
        report_at(advisory, stage.stage, source, &stage.diags);
    }

    // The effect-map value (23D §1 — the check is the oracle); its diags were emitted by `validate`.
    let idx = dorc_oracle::lift(&mut interner, &oracle_refs).value;

    // The per-file PredictSets (the entity-resolution mechanism; shared interner — 204 seam #2). The
    // per-file `check`-dialect diags were emitted by `validate` above.
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
        .collect();

    // The typeless-floor verdict-provider set (`24L` §7 — THE kernel seam): the analyzer kernel is
    // verdict-unaware by design (`inv-determinism`), so the cli edge lifts which providers bear an
    // `is_converged` verdict function and threads that set INTO `classify` as DATA. The auto-cell
    // mint reads it to light up a markless verdict-only oracle's guard/elide tier (`24L` §2).
    let verdict_providers = dorc_oracle::verdict::verdict_providers(&mut interner, &oracle_refs);
    // Pre-lift each file's verdict funcdefs so the (immutable-interner) probe ship-closure can
    // strip the auto-cell's verdict body without a mutating re-lift (`24L` §2 probe emission). Diags
    // drop here — `build_vouches` re-lifts and surfaces them once for gate-3.
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
        .collect();

    // The escalation-POLICY disclosure (`27C:render-authority-disclosure`): one advisory line naming
    // the escalation posture (the dial × the connection capability) and the entry-capable wrappers
    // loaded. Consent legibility — the admin sees, once, what authority the probe re-uses.
    emit_escalation_policy(
        advisory,
        &mut interner,
        &oracle_refs,
        args.dial,
        args.capability,
    );

    // Parse + analyze the book (shared interner, so symbols match the oracles). Multiple books
    // CONCATENATE into one analyzed unit (`\n`-joined so no two files' lines merge). `book_name`
    // is the display path (the first book) — for a single book (the norm) the frame's line numbers
    // are exact source lines; a multi-book unit's line numbers are into the concatenation.
    let replay_books: Vec<String>;
    let books: &[String] = match &replay {
        Some(r) => {
            replay_books = vec![r.book_path.clone()];
            &replay_books
        }
        None => &args.books,
    };
    let book_src = read_books(books)?;
    let book_name = books.first().map_or("book.sh", String::as_str);
    // The unloaded-sibling-oracle hint (gap-5 / `24H` ack-6): a cli-edge, filesystem-reading disclosure.
    emit_unloaded_sibling_oracles(advisory, books, &oracle_paths);
    // `--last` desync guard (`22F` book-identity): re-read digests must match the durable's.
    // ack-8: the book-stage diags (parse/cfg/classify/probe/render) all span into `book_src`;
    // this pair feeds their file:line:col frames (rul24-lineno-identity — the SOURCE line space).
    let book_source = Some((book_name, book_src.as_str()));
    let parsed = dorc_syntax::parse(&book_src);
    report_at(advisory, "parse", book_source, &parsed.diags);
    // The marker gate also covers a BOOK that HOSTS oracle functions (share-a-file): an unmarked
    // book carrying a bind/mark errors, while a stripped off-ramp artifact (dialect erased) stays
    // marker-free and only warns on the reserved-name squat below (guard23-reingest-collision).
    report_at(
        advisory,
        "marker",
        book_source,
        &dorc_oracle::marker::check_dialect_marker(&mut interner, &book_src),
    );
    // The munge-reservation squat lint (24M rul24M-bare-dorcism-names): a book funcdef
    // coincidentally named `<x>__<role>` squats the reserved emitted namespace — surfaced LOUDLY
    // as a Warning (the loud-friend law; rul24-warnings-tune-high). The live corpus instance is
    // guard23-reingest-collision-verbatim's `apt_get__predict` book function. Warning-severity, so
    // it never fails a case (gate-3 keys on `error[`).
    report_at(
        advisory,
        "reserved",
        book_source,
        &dorc_oracle::reserved::lint_book_reserved_names(&parsed.value),
    );
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    report_at(advisory, "cfg", book_source, &cfg.diags);
    // ack-1 exit-code family: a book carrying a parse/CFG ⊤-reject (`inv-top-reject`) — a
    // syntax-unsupported/malformed construct, or its downstream CFG ⊤-node — leaves the analysis
    // built on partial understanding. The artifact still ships byte-identically (the stdout
    // fence), but the process fast-fails with EXIT_BOOK_UNMODELED so a `dorc … && deploy` chain
    // STOPS. Keyed on Error-severity from the parse OR cfg stage (the only Error codes there are
    // syntax-{unsupported,malformed} and cfg-top-node — the "book cannot be modeled" set); a
    // render-refusal / oracle-lift Error is kFAIL-safe and does NOT fast-fail (the artifact is
    // valid). Computed here (before the `probe` early-return) so every mode signals it.
    let book_unmodeled = parsed
        .diags
        .iter()
        .chain(cfg.diags.iter())
        .any(|d| d.severity() == Severity::Error);
    let book_outcome = if book_unmodeled {
        RunOutcome::BookUnmodeled
    } else if wrapper_incoherent {
        // Dual-peel incoherence is pre-network fail-fast (`273` §5), independent of the book;
        // ranked after book-unmodeled only because the exit codes are distinct signals.
        RunOutcome::WrapperIncoherent
    } else {
        RunOutcome::Complete
    };
    // Book-side value-flow: resolve each command-site's argv (constant/variable
    // propagation) — the input entity-resolution consumes (19H §1 / 202 §1).
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
    // The per-run receipts plane (arch-1): give-up causes (`Top(cause)`) and license
    // witnesses land here. EXEMPT — it informs no decision (the `plan::erasability` gate
    // proves the apply/probe artifacts are byte-identical with it stripped); the cli holds it
    // only to emit the decision-digest line and (future) the why-lens.
    let mut arena = ProvArena::new();
    // stage-3 (the why-lens): take the TYPED cmdsub-⊤ disclosures too — `report`/gate-3 consume the
    // LOWERED `diags` (cause-dropped), but the why-lens render reads the `cause` off the typed
    // `Diag`s (`to_legacy` drops it). The arena is shared (the typed diags' causes resolve in it).
    // `kills` (R3 / 24A §3): the kill-bearing leaf set the wall predicate cannot read off the
    // `MustRun` SkipClass alone. Threaded to `build_plan_walled` so a running `apt-get purge`
    // walls downstream, closing the kill gap fd10's establish-only wall left open. `kill_coords`
    // (24E §7): each single-kill node's killed coordinate — the kill-wall coherence comparand
    // (own-killed-coord ⊆ footprint), closing resid-kill-coherence.
    // `fact_backings` (`277` §5 backing-SETS): each establish fact's survival-backing provenance
    // — its minting family + observe-backing-widening selectors — threaded to `build_plan_walled`
    // so the survival tier builds each fact's backing SET (a widened backing GROWS kill-surface).
    // `27N` — peel wrapped BOOK sites into (inner command, composed context) + decide entry. Empty
    // for a wrapper-free run ⇒ the whole pipeline is byte-identical (`empty-world-byte-identical`).
    let wrapped_analysis = build_wrapped_analysis(
        &oracle_srcs,
        &oracle_refs,
        &oracle_paths,
        &checks,
        &verdict_sets,
        &parsed.value,
        &cfg.value,
        &value,
        args.dial,
        args.capability,
        &mut interner,
    );
    let peeled_sites = wrapped_analysis.peeled;
    let wrapped_probes = wrapped_analysis.wrapped;
    let carried_attribution = wrapped_analysis.carried;
    let entry_narrative = wrapped_analysis.collapse_narrative;
    report_at(advisory, "wrapped", book_source, &wrapped_analysis.hints);
    let (classified, why_diags, kills, kill_coords, fact_backings, classify_narrative) =
        dorc_analysis::effect::classify_with_why_diags(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &verdict_providers,
            &peeled_sites,
            &mut interner,
            &mut arena,
        );
    report_at(advisory, "classify", book_source, &classified.diags);
    let classes = classified.value;

    // The per-site guard VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c) —
    // ALWAYS-ON (guards are the un-flagged baseline; rul24-mode-gate governs only the survival
    // tier, NOT this). A vouched past-wall establish ships its read-only probe (the witness needs
    // the verdict) and, converged, mints a `Disposition::Guard`.
    let (mut vouches, decline_narrative) =
        build_vouches(&oracle_refs, &classes, &value, &mut interner, advisory);
    // `27N` — wrapped-entering sites vouch on the INNER verdict over the peeled argv (argv[0] is the
    // wrapper word, invisible to `build_vouches`). Disjoint nodes ⇒ a plain merge.
    vouches.extend(dorc_plan::build_wrapped_vouches(
        &oracle_refs,
        &classes,
        &wrapped_probes,
        &mut interner,
    ));

    // The CONNECTED check-pipes (`24J` §2, repaired — `271:rul-only-oracle-bytes-ship`): a simple
    // all-vouched-read-only pipeline `A | F [| F…]` ships as ONE composed probe keyed to its
    // governing (last) stage — each stage replaced by its oracle's stripped predict; the non-last
    // stages are subsumed. `connected_check_pipes` is the DECIDER: it resolves each stage + applies
    // the per-channel coverage rule (rider 1 — a non-last stage must produce REAL stdout), refusing
    // any compound whose stage can't be model-substituted (⇒ its stages run). Empty for a book with
    // no such pipe. Threaded into BOTH the probe compiler (ship the composed body) and the plan
    // builder (omit the subsumed members).
    let ship_stage = |p, a: &[Symbol]| ship_predict_stage(&oracle_srcs, &checks, &interner, p, a);
    let connected =
        dorc_plan::connected_check_pipes(&parsed.value, &cfg.value, &value, &classes, ship_stage);

    // The read-only, SELF-REPORTING, site-keyed probe (R3 / 23D §1 — the check IS the oracle):
    // each site ships its provider's stripped `<provider>__predict` invoked with the site's argv.
    // `is_vouched` closes strain-classify-coupling (24C): a vouched past-wall `EstablishWritten`
    // site ships its probe here (at HEAD it would be `unresolvable-no-probe`).
    let ship = |p, a: &[Symbol]| ship_predict_body(&oracle_srcs, &checks, &interner, p, a);
    // `24L` §2 — the typeless-floor auto-cell ships its stripped VERDICT body (the probe IS the
    // verdict). `Some` ONLY for an auto-cell fact (keyed on the reserved auto-kind), so `compile_probe`
    // reads a `Some` as the auto-cell signal. rul-only-oracle-bytes-ship: the shipped bytes are the
    // oracle's OWN authored `is_converged` funcdef, strip-only; the admin's argv flows as arguments.
    let ship_auto =
        |fact: dorc_core::FactKey, p: Symbol, _a: &[Symbol]| -> Option<dorc_plan::ShippedCheck> {
            if !dorc_core::is_auto_kind(&interner, fact.kind) {
                return None;
            }
            ship_verdict_body(&oracle_srcs, &verdict_sets, &interner, p)
        };
    let probe = dorc_plan::compile_probe(
        &parsed.value,
        &cfg.value,
        &value,
        &classes,
        &wrapped_probes,
        &connected,
        ship,
        ship_auto,
        |node| vouches.contains_site(node),
    );

    // Shim-materialization edge (`274` §5 / `27L` task-14): `--shim-dir` writes the entry-composed
    // probe's per-run PATH shim files (a pure side-effect at the cli edge; stdout unchanged).
    if let Some(dir) = &args.shim_dir {
        materialize_shim_dir(dir, &probe.shim_files())?;
    }

    // The DERIVATION-probe (24E §2 corr-§2 — the SECOND probe-shipping path, a NEW pipeline
    // stage): under `--risk-faultless-skips`, a wall-candidate whose `touches()` body ESCALATED (it
    // reached a host query the static `evaluate_touches` could not resolve) ships that body into
    // phase-1, runs read-only, and its stdout coord-lines are read back into a `Derived` footprint
    // (merged below, pre-`build_plan_walled`). Lifted for the derivation lane here; the authored
    // lane (`build_survival_footprints`) lifts its own — both pure + cheap, and a clean oracle
    // reports no touches diag either way (fork-s4-compile: a parallel compiler, NOT a `compile_probe`
    // extension — different site-set/body-source/readback, the convergence path left unperturbed).
    let touches_paired: Vec<(&str, dorc_oracle::touches::TouchesSet)> = if args.trust_footprints {
        oracle_refs
            .iter()
            .map(|src| {
                (
                    *src,
                    dorc_oracle::touches::TouchesSet::lift(&mut interner, src).value,
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let derivations = if args.trust_footprints {
        let derive = |p, a: &[Symbol]| ship_touches_body(&touches_paired, &interner, p, a);
        dorc_plan::compile_derivations(&parsed.value, &cfg.value, &value, &classes, &kills, derive)
    } else {
        dorc_plan::DerivationPlan::default()
    };

    // The RESOLVER-probe (24F §3 — the identity CANONICALIZATION lane, a THIRD phase-1 shipping path).
    // Lift the per-kind resolvers + enforce confusability ALWAYS (an oracle-authoring correctness
    // check, flag-independent). The round-trip (coord enumeration + probe compile) is flag-on: for
    // each resolver-bearing coordinate (footprint + backing sides) ship `<kind>.resolve()` to
    // canonicalize its entity; the `resolv` readback builds the `Resolutions` merged pre-survival-walk.
    // The RAW coordinate kinds — used to re-key the munged kind-keyed resolver/reaches maps
    // (`flag-forward-munge-keying`: funcdefs are named by the kind's forward-munge, coords carry the
    // raw dotted kind, so the two are bridged here once).
    let coord_kinds = {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_coord_kinds(&classes, &kills, &value, &touches_sets, &mut interner)
    };
    let kind_resolvers = build_kind_resolvers(
        &oracle_srcs,
        &oracle_paths,
        &checks,
        &touches_paired,
        &coord_kinds,
        &mut interner,
        advisory,
    );
    let resolver_kinds: BTreeSet<Symbol> = kind_resolvers.resolver_kinds().collect();
    let resolver_coords = if args.trust_footprints && !resolver_kinds.is_empty() {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_resolver_coords(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &resolver_kinds,
            &mut interner,
        )
    } else {
        BTreeSet::new()
    };
    let resolvers = compile_resolvers(&resolver_coords, &kind_resolvers, &oracle_srcs, &interner);

    // The REACH-probe (24G §4 — the reaches() EXPANSION lane, a FOURTH phase-1 shipping path). Lift
    // the per-kind reach-functions + enforce confusability ALWAYS (kind-keyed like the resolver). The
    // round-trip (dynamic-arm shipping) is flag-on: for each reach-bearing AUTHORED footprint coord,
    // ship each DYNAMIC arm strip-clean, invoked with the entity; the `reach` readback expands the
    // footprints (via `Footprint::add_reached`) before the survival walk. STATIC arms never ship.
    let kind_reaches = build_kind_reaches(
        &oracle_srcs,
        &oracle_paths,
        &checks,
        &touches_paired,
        &coord_kinds,
        &mut interner,
        advisory,
    );
    let reach_kinds: BTreeSet<Symbol> = kind_reaches.reach_kinds().collect();
    let reaches_plan = if args.trust_footprints && !reach_kinds.is_empty() {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_reach_probes(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &kind_reaches,
            &reach_kinds,
            &oracle_srcs,
            &mut interner,
        )
    } else {
        dorc_plan::ReachPlan::default()
    };

    // `probe` mode stops here: emit the probe artifact and return. It reads no stdin (no
    // results exist yet — this is phase 1, what you ship to GET them), builds no plan, and so
    // emits no apply, no why-lens, no digest (there is no plan/identity-plane to hash —
    // tc-probe-no-digest, flagged). Stage diagnostics above already routed to stderr.
    // `262` §2 framing: the run's records-lane keys, minted at THIS controller edge (the DI
    // seam — `inv-determinism`, no ambient RNG in the kernel). The nonce/host/attempt are the
    // spike fixed defaults (deterministic goldens; a real fleet mints per-attempt/per-host);
    // `book=` binds the stream to the exact analyzed book bytes. The end-sentinel follows the
    // final record lane (`records::sentinel_line`); the drain keys on it, never on EOF.
    let framing = dorc_plan::records::Framing::spike(book_digest(&book_src));
    if mode == Mode::Probe {
        print!("{}", probe.render_sh(&framing, &interner));
        print!("{}", derivations.render_sh(&framing.nonce, &interner)); // 24E §2: SAME phase-1 block
        print!("{}", resolvers.render_sh(&framing.nonce)); // 24F §3: SAME phase-1 block
        print!("{}", reaches_plan.render_sh(&framing.nonce)); // 24G §4: SAME phase-1 block
        print!("{}", dorc_plan::records::sentinel_line(&framing.nonce));
        std::io::stdout().flush().ok();
        return Ok(book_outcome);
    }

    // The round-trip emits the probe FIRST (phase 1 on stdout), then the apply (phase 2)
    // after stdin EOF — the e2e harness splits the two on the `#!/bin/sh` shebang. `plan`
    // and `apply` emit ONLY the apply artifact (the probe is an internal compile there).
    if mode == Mode::RoundTrip {
        print!("{}", probe.render_sh(&framing, &interner));
        print!("{}", derivations.render_sh(&framing.nonce, &interner)); // 24E §2: SAME phase-1 block
        print!("{}", resolvers.render_sh(&framing.nonce)); // 24F §3: SAME phase-1 block
        print!("{}", reaches_plan.render_sh(&framing.nonce)); // 24G §4: SAME phase-1 block
        print!("{}", dorc_plan::records::sentinel_line(&framing.nonce));
        std::io::stdout().flush().ok();
    }

    // read the (simulated) probe results — the site-keyed records the rendered probe would emit
    // when run remotely (the round-trip's return channel). From `--results FILE` when given, else
    // the default stdin (the harness pipes them in).
    let scope =
        WidthOneAttemptScope::new(&framing, book_name, &book_src, &oracle_paths, &oracle_srcs);
    let (admitted_records, scoped_results, whylog_eligible) = if let Some(r) = replay.as_ref() {
        let results = r.records.as_ref().map_or_else(
            || SiteResults {
                framed: true,
                ..SiteResults::default()
            },
            |records| {
                parse_admitted_results(
                    records,
                    &mut RunClock::Recorded(r.instants.clone()),
                    &mut interner,
                )
            },
        );
        (None, ScopedHostEvidence::new(scope, results), false)
    } else {
        let evidence = if let Some(path) = &args.results {
            let file =
                std::fs::File::open(path).map_err(|e| humane_read_error("results", path, &e))?;
            dorc_plan::records::read_host_evidence(
                file,
                dorc_plan::records::HostEvidenceLimits::spike_default(),
            )
        } else {
            dorc_plan::records::read_host_evidence(
                std::io::stdin(),
                dorc_plan::records::HostEvidenceLimits::spike_default(),
            )
        };
        let admitted = match evidence {
            dorc_plan::records::Admission::Admitted(bytes) => {
                dorc_plan::records::admit_unscoped_host_records(
                    &bytes,
                    &framing,
                    dorc_plan::records::HostEvidenceLimits::spike_default(),
                )
            }
            dorc_plan::records::Admission::NoObservation => {
                dorc_plan::records::Admission::NoObservation
            }
            dorc_plan::records::Admission::Refused(reason) => {
                dorc_plan::records::Admission::Refused(reason)
            }
        };
        match admitted {
            dorc_plan::records::Admission::Admitted(records) => {
                let parsed = parse_admitted_results(&records, clock, &mut interner);
                (Some(records), ScopedHostEvidence::new(scope, parsed), true)
            }
            dorc_plan::records::Admission::NoObservation => (
                None,
                ScopedHostEvidence::new(
                    scope,
                    SiteResults {
                        framed: true,
                        ..SiteResults::default()
                    },
                ),
                false,
            ),
            dorc_plan::records::Admission::Refused(reason) => {
                report_at(advisory, "records", None, &[reason.spanless_diagnostic()]);
                return Ok(RunOutcome::IngressRefused);
            }
        }
    };
    let _scope = scoped_results.scope();
    let results = scoped_results.borrow();

    // re-key the site-keyed records to the FactKey-keyed observations `build_plan`
    // consumes (its fold/elision machinery is fact-keyed; only this probe-answer
    // plumbing re-keys — `inv-site-keyed-results`). The probe's `checks` carry each
    // site's resolved fact + its `site_kind`, so a site-record maps site→fact AND the
    // firewall knows whether the rc is fold-usable. CRITICAL (the wrong-concrete
    // firewall, 202 §3 / task-D2): a record's `rc` feeds the fold's Status ONLY for a
    // VALID Query-class site (the guard's own rc); an establish site's rc is the PROBE
    // command's (dpkg-query's), NOT the mutator's, so it feeds the fold NOTHING.
    let (by_fact, merge_narrative) = facts_from_sites(&probe, results);
    let probe_origins = probe_origins(&probe, results, &mut arena);

    // The survival tier (Stage 2 / rul24-mode-gate, TC-1): footprints are lifted ONLY under
    // `--risk-faultless-skips` — off ⇒ `None` ⇒ the honest Stage-1 total wall, the data never exists.
    let survival = args.trust_footprints.then(|| {
        let mut fps = build_survival_footprints(
            &oracle_refs,
            &classes,
            &kills,
            &kill_coords,
            &value,
            &cfg.value,
            &parsed.value,
            &mut interner,
            advisory,
        );
        // 24E §2 corr-§2: merge the host-DERIVED footprints (read back from the phase-1
        // derivation-probe's `deriv` coord-records) into the authored set, before the survival
        // walk. An escalated site has NO authored footprint (its static trace ⊤'d), so the two
        // sets are disjoint by construction — no collision.
        // The escalated sites' book spans (`aid-caret-span-precision`), precomputed at the edge (the
        // merge runs interner-only): each derivation's `CfgNodeId`→AST span, total by construction.
        let derived_node_spans: BTreeMap<_, _> = derivations
            .derivations
            .iter()
            .map(|d| (d.node, parsed.value.node(cfg.value.node(d.node).ast).span))
            .collect();
        merge_derived_footprints(
            &mut fps,
            &derivations,
            results,
            &classes,
            &kill_coords,
            &derived_node_spans,
            &mut interner,
            book_source,
            advisory,
        );
        // 24G §4: EXPAND each reach-bearing footprint coord via reaches() — STATIC arms (cli-traced,
        // all coords) + DYNAMIC arms (the `reach` readback, authored coords only). Widening is
        // monotone-safe (`inv-kfail`); runs AFTER the authored/derived merge and BEFORE the walk, so
        // the wider footprint flows the EXISTING disjoint/canonicalize path (no new interplay code).
        expand_footprints_via_reaches(
            &mut fps,
            &kind_reaches,
            &reach_kinds,
            results,
            &mut interner,
        );
        fps
    });
    // 24F §3: build the identity-canonicalization map from the `resolv` readback (both footprint and
    // backing coords canonicalized in the survival walk). Flag-off / no-resolver ⇒ empty ⇒ the
    // token-equality floor (identical to today). §4: each DANGLING coordinate is a loud diagnostic.
    let mut resolutions =
        build_resolutions(&resolver_coords, &resolver_kinds, results, &mut interner);
    // fence-no-disjoint (`24L` §7): register every verdict-provider's auto-cell kind so the survival
    // tier reads an auto coordinate as may-touch (`survival::disjoint`). The plan is interner-free,
    // so this resolution happens here (the edge holds the interner) and rides the Resolutions the
    // walk already threads. Re-interning `dorc-auto:<provider>` returns the KindId classify minted.
    for provider in &verdict_providers {
        let name = interner.resolve(provider.0).to_owned();
        let kind = dorc_core::auto_fact(&mut interner, &name).kind;
        resolutions.add_auto_kind(kind);
    }
    report_at(
        advisory,
        "resolve",
        None, // dangling-reference notes are spanless (no book/oracle location)
        &dangling_diagnostics(&resolutions, &interner),
    );
    let plan = dorc_plan::build_plan_walled(
        &book_src,
        &parsed.value,
        &cfg.value,
        &classes,
        &kills,
        survival.as_ref(),
        args.trust_footprints.then_some(&resolutions),
        &dorc_oracle::build_dialect(&idx),
        &fact_backings,
        &vouches,
        &connected,
        &probe_origins,
        |f| {
            by_fact
                .get(&f)
                .copied()
                .unwrap_or(Observable::verdict_only(Verdict::Unknown))
        },
        &mut arena,
    );

    // q-2 (`dq-site-unresolvable`, the cli-edge readout): a `unresolvable-no-probe` comment lands
    // in the probe artifact, but nothing reached stderr (`219` q-1.f silent-3). Disclose each
    // probe-unresolvable site's source command as a Note — the apply runs it (`kFAIL-perform`).
    // ADVISORY (Note-severity): the off-ramp `apply` mode suppresses it; `plan`/round-trip show
    // it (the ui-3 cited-disclosure surface). The apply still RUNS the site either way, so no
    // correctness rides on this readout — it is purely the render surface (rec-1).
    report_at(
        advisory,
        "probe",
        book_source, // the unresolvable-site notes span into the book (file:line:col frame)
        &unresolvable_diagnostics(&probe, &plan, &parsed.value, &book_src),
    );

    // upcoming-firstwall-hint (USER_STORY stage 3): the FIRST poison wall formed by an UNMODELED
    // command, plus the counterfactual count of downstream sites an oracle for it would un-wall.
    // Computed once (cheap, pure over the built plan) and consumed by BOTH the advisory `hint:` nag
    // (below) and the `dorc why` detail (`emit_why_report`). `None` ⇒ no unmodeled wall ⇒ no hint
    // (a modeled-but-diverged wall is an honest wall, never this hint's subject).
    let wall_steps = collect_wall_steps(
        &plan,
        &probe,
        &classes,
        &cfg.value,
        &kills,
        &parsed.value,
        &book_src,
    );
    let first_wall = first_wall_hint(&wall_steps);

    // stage-3 (the why-lens, `22D` §1): the FIRST receipt-READER made user-visible. For each
    // forced-run (never-elided) command whose ⊤ has a wired cause, surface — on the RENDER surface
    // (stderr), at the decision point — "why did this run?", cause-derived + remediation-classed.
    // rec-1 WELD: this is the plan-render surface ONLY; it is NEVER woven into the byte-floored
    // `.sh` artifact on stdout (the artifact stays receipt-free). The off-ramp `apply` mode
    // suppresses it (advisory); `plan` + round-trip emit it (ru-20 ui-3: "doubly-emit cited
    // sections + their warnings to the console").
    // The `plan`/round-trip render surface keeps its per-line `why:` disclosures (the attribution
    // lanes are load-bearing correctness disclosures gate-7 pins). `why` mode SKIPS them — its
    // stdout report (below) is the detail surface, so a stderr echo would just double it.
    // `27W` §3 C3 pairing: fold each ingested tier-3 report record (recognized class + site)
    // into its site's `VerdictDecline` via `with_authored_reason` (idempotent — tier-2 static
    // wins). Then union the collapse-narratives onto the why-lens seam (d4 renders; decision-inert).
    let paired_declines = pair_authored_reasons(decline_narrative, &results.reports);
    let collapse_narrative: Vec<CollapseNarrative> = classify_narrative
        .iter()
        .cloned()
        .chain(paired_declines)
        .chain(entry_narrative.iter().cloned())
        .chain(merge_narrative.iter().cloned())
        .chain(plan.survival_report.collapse_narrative().iter().cloned())
        .chain(plan.render_refusal_narratives(&parsed.value))
        .collect();
    if advisory && mode != Mode::Why {
        emit_why_lens(&why_diags, &arena, &book_src, &collapse_narrative);
        // sigpipe-flap-class (`279f` §5): a probe record landing rc 141 (128+SIGPIPE) is the
        // NAMED early-exit-race nondeterminism class — a `pipefail`-off `A | grep -q` whose
        // consumer closed the pipe before an upstream stage finished writing. The landing is SAFE
        // (cant-tell ⇒ Unknown ⇒ run) and never flaps the verdict, so this is an advisory nudge,
        // not an error. (A `--exit-code`-like surface must source from divergence-of-world, never
        // this raw rc — see `dorc_plan::render::probe::record_scaffold`.)
        emit_sigpipe_race_notes(results);
        emit_report_lane_notes(results); // `27W` §2 tier-3 RUNTIME records; empty in-corpus
        // `27W` §3 tier-2 STATIC decline classes at plan time, with the emitting arm's file:line.
        emit_static_decline_notes(&collapse_narrative, &oracle_paths, &oracle_srcs);
        // Stage 2 co-primary (rul24-divergence-is-the-game / TC-3): every SURVIVED elision names,
        // on this same why-lens lane, which running walls it crossed and whose footprint licensed
        // each crossing. This is the attribution tether under the sharpest claim in the design —
        // a wrong footprint silently under-executes someone else's line, so the render surface
        // must always say whose footprint you trusted. Empty when unflagged (no survivals).
        emit_survival_attribution(&plan, &interner, &oracle_paths, &oracle_srcs);
        // 24G Part B: every converged elision a reaches() expansion DEMOTED names the reach-function
        // (the cross-author demote); empty when no reach expansion poisoned an elision.
        emit_reach_poisonings(&plan, &interner);
        // Stage 3 (rul-guard-license / X-why): every GUARDED site names, on the same lane, the
        // mechanism + its converged-vouch license + the vouching oracle (a render-REFUSED guard
        // discloses the refusal instead). Empty when no site guards.
        emit_guard_attribution(&plan, &parsed.value, &interner, &oracle_paths, &oracle_srcs);
        // `27C` §4(a): every pure-predicate-CARRY elision names its cross-context attribution chain
        // on this same lane (the crossed substrate axes, each backing kind's owner `invariant:` line,
        // the read-set-closure proof). Empty when no site carried.
        emit_carry_attribution(&plan, &carried_attribution);
        // upcoming-firstwall-hint (USER_STORY stage 3): the forward NAG — ONE aggregated line for
        // the FIRST unmodeled wall, naming the count an oracle for it would un-wall. `hint: ` prefix
        // (never `error[`), so the gate-3 stderr floor ignores it. rul24-warnings-tune-high: the
        // nag-loop drives the entire enhancement curve — this hint IS the product, not noise.
        if let Some(fw) = &first_wall {
            eprintln!("hint: {}", fw.body());
        }
        // ack-2 aggregate POINTER: the `plan` preview points the reader at the focused query
        // surface. (This pass keeps the per-line `why:` detail here too — gate-7 pins it; fully
        // moving the detail into `dorc why` is a sanctioned follow-on that churns the 13
        // expected-why needles + rewires gate-7, deferred to keep this pass green.)
        eprintln!("{}", chrome("cli-why-pointer-line", &[book_name]));
    }

    // gate-5 (cm-2 argv-echo differential): per-site resolved argv to stderr, behind the flag.
    // Independent of the advisory plane — it is a mechanized readout the harness consumes, not
    // human-facing disclosure, so it fires in any mode when asked (the round-trip is the only
    // caller in-corpus, but `plan --debug-argv` is a legitimate inspection).
    if args.debug_argv {
        emit_debug_argv(&plan, &cfg.value, &value, &interner);
    }

    // arch-1 d-6: the leaf-exact render refuses to elide a leaf whose span can't be safely
    // edited (a heredoc-bearing command — its span covers `<<EOF`, not the body), running it
    // verbatim instead (kFAIL-perform). Surface WHY on stderr (else a converged mutator
    // silently running is invisible); the gate-3 floor requires the case to declare it. These
    // are ERROR-severity, so they cross the floor in EVERY mode (incl. `apply`): the off-ramp
    // must never silently ship an artifact whose render had to refuse a licensed elision.
    let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
    report("render", book_source, &refusals);

    let identity_diags: Vec<Diag> = classified
        .diags
        .iter()
        .cloned()
        .chain(refusals.iter().cloned())
        .collect();
    let decision_digest = dorc_plan::erasability::decision_digest(
        &plan,
        &probe,
        &book_src,
        &parsed.value,
        &interner,
        &identity_diags,
    );

    // ack-2 `dorc why`: NOT an artifact-producing invocation. Emit the source-line-keyed report to
    // STDOUT (its own non-analysis output) and return — no artifact, no plan-summary, no digest.
    // It runs the full pipeline above so it reports on the CURRENT run's real dispositions.
    if mode == Mode::Why {
        // `--last` belt-and-suspenders: a diverged decision digest (same inputs) ⇒ refuse, not narrate.
        if let Some(r) = &replay
            && decision_digest != r.decision_digest
        {
            report_at(
                advisory,
                "whylog",
                None,
                &[Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                    dorc_aid::diag::WhylogBookDesync {
                        which: "decision-digest".to_owned(),
                    },
                ))],
            );
            return Ok(book_outcome);
        }
        let receipt = Receipt {
            at: replay
                .as_ref()
                .map_or_else(|| clock.now(), |r| r.started_at),
            replayed: replay.is_some(),
            host: framing.host.clone(),
            book: book_name.to_owned(),
            book_digest: book_digest(&book_src),
            at_head: source_match::resolve(
                &source_match::GitRepository,
                std::path::Path::new(book_name),
            ),
            oracles: oracle_paths.clone(),
            risk_profile: args.trust_footprints.then_some(CONSENT_FLAG),
            counts: plan.disposition_counts(),
            deepest_tier: args.all,
            // Only a replay can disagree, and it declares its stream rather than being assumed.
            narratable: replay
                .as_ref()
                .is_none_or(|r| r.record_stream_version == dorc_aid::narrative::PLANE_VERSION),
        };
        emit_why_report(
            args.why_address.as_deref(),
            &plan,
            &probe,
            first_wall.as_ref(),
            &wall_steps,
            &why_diags,
            &refusals,
            &arena,
            &parsed.value,
            &book_src,
            book_name,
            &interner,
            &oracle_paths,
            &oracle_srcs,
            &collapse_narrative,
            &receipt,
        );
        std::io::stdout().flush().ok();
        return Ok(book_outcome);
    }

    // rec-1 / ru-12 BYTE FLOOR: `plan` and `apply` emit BYTE-IDENTICAL apply bytes here — the
    // artifact is receipt-free in both; only the stderr disclosure above differed. The
    // round-trip emits the same bytes as its second shebang block.
    print!("{}", plan.render_apply(&book_src, &parsed.value));

    // plans/240 Stage-1 yardstick: the plan-summary on stderr, alongside the digest below.
    emit_plan_summary(&plan);

    eprintln!(
        "{}",
        chrome("cli-decision-digest-line", &[&decision_digest])
    );

    // Default-on: the receipt nobody asked for is the only kind that exists on the bad morning.
    if let Some(dir) = durable_destination(args) {
        let metadata = assemble_whylog_metadata(
            &framing,
            book_name,
            &book_src,
            &oracle_paths,
            &oracle_srcs,
            &decision_digest,
            &plan,
            clock.now(),
            results,
        );
        if whylog_eligible && let Some(records) = admitted_records.as_ref() {
            write_whylog(&dir, &metadata, records);
        }
    }
    Ok(book_outcome)
}

struct Replay {
    book_path: String,
    oracle_paths: Vec<String>,
    decision_digest: String,
    /// The instant the ORIGINAL run started, as the durable recorded it. The receipt dates itself
    /// by this and never by the replay's own clock — a replay reports on a moment that has already
    /// passed, and re-dating it to now would present a reading as a running.
    started_at: Option<dorc_core::RunInstant>,
    /// Which record-stream version this durable declared — the key the `[unnarrated:]` census is
    /// gated on, so it can never make a coverage claim about a stream this binary's narrative
    /// plane was not built against.
    record_stream_version: u32,
    /// The instants the ORIGINAL run recorded for its probe records, by arrival ordinal.
    instants: BTreeMap<u64, dorc_core::RunInstant>,
    records: Option<dorc_plan::records::AdmittedUnscopedHostRecords>,
}

enum ReplayLoad {
    Admitted(Replay),
    NoObservation(Replay),
    Refused,
}

/// Whylog retention: keep the newest [`WHYLOG_KEEP`] durables by run-index; cap each at
/// [`WHYLOG_CAP`] bytes. Deterministic (index order, no clock — `inv-determinism` at the edge).
///
/// INTERIM, and disclosed as such (`churn-avoidance-disclosure`). The real retention design —
/// what is durable, for how long, at what permissions, classified how — is ONE decision that
/// `28D:must-retention-is-one-decision` puts ahead of the whole forensic tier, and it is r30's.
/// These two numbers are not that decision; they are what keeps default-on honest until it lands.
///
/// Sized against the promise rather than a guess: `USER_STORY` says "ask tomorrow; ask next week",
/// and the shape that has to survive is nightly cron applies plus a firefighting day's re-runs.
/// The old keep-5 could not survive one bad morning; these can hold a week of them.
const WHYLOG_KEEP: usize = 64;
const WHYLOG_CAP: usize = 4_000_000;

#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
#[expect(
    clippy::too_many_lines,
    reason = "one linear admission ladder: select the durable, bound it, read back the book and oracles it names, check the framing, then admit the records. Every rung refuses on its own terms and splitting it would scatter the ONE place a replay's inputs are validated"
)]
fn load_whylog_replay(args: &Args, advisory: bool) -> Result<ReplayLoad, Diag> {
    // Exact-file `--whylog=` selection (the deterministic single-file corpus flag) feeds r29's
    // admission unchanged; otherwise fall back to newest-in-`--whylog-dir`.
    let path = if let Some(exact) = args.whylog.as_deref() {
        std::path::PathBuf::from(exact)
    } else {
        let dir = durable_destination(args).ok_or_else(|| {
            Diag::new_spanless_site(DiagCode::CliFlagRequiresMode(
                dorc_aid::diag::CliFlagRequiresMode {
                    flag: "--whylog-dir=DIR",
                    mode: "dorc why",
                },
            ))
        })?;
        let dir = dir.as_str();
        let Some(path) = whylog_store::newest(dir) else {
            report_at(
                advisory,
                "whylog",
                None,
                &[Diag::new_spanless_site(DiagCode::WhylogAbsent(
                    dorc_aid::diag::WhylogAbsent {
                        dir: dir.to_owned(),
                    },
                ))],
            );
            return Ok(ReplayLoad::Refused);
        };
        path
    };
    let Ok(file) = std::fs::File::open(&path) else {
        return Ok(refuse_replay(
            advisory,
            dorc_plan::records::AdmissionRefusal::Framing,
        ));
    };
    let envelope = match dorc_plan::whylog::admit_unscoped_whylog(
        file,
        dorc_plan::whylog::WhylogLimits::spike_default(),
    ) {
        dorc_plan::records::Admission::Admitted(envelope) => envelope,
        dorc_plan::records::Admission::NoObservation => {
            return Ok(refuse_replay(
                advisory,
                dorc_plan::records::AdmissionRefusal::Framing,
            ));
        }
        dorc_plan::records::Admission::Refused(reason) => {
            return Ok(refuse_replay(advisory, reason));
        }
    };
    let book_path = envelope.recorded_book_path().as_str().to_owned();
    let oracle_paths: Vec<String> = envelope
        .recorded_oracles()
        .iter()
        .map(|oracle| oracle.path().as_str().to_owned())
        .collect();
    let Ok(book) = read_replay_source(&book_path) else {
        return Ok(refuse_replay(
            advisory,
            dorc_plan::records::AdmissionRefusal::Framing,
        ));
    };
    let oracle_sources: Vec<String> = match oracle_paths.iter().map(read_replay_source).collect() {
        Ok(sources) => sources,
        Err(()) => {
            return Ok(refuse_replay(
                advisory,
                dorc_plan::records::AdmissionRefusal::Framing,
            ));
        }
    };
    let framing = dorc_plan::records::Framing::spike(book_digest(&book));
    let scope =
        WidthOneAttemptScope::new(&framing, &book_path, &book, &oracle_paths, &oracle_sources);
    // An edited book is the ordinary mismatch, so it is NAMED rather than reported as generic
    // framing. The refusal stands; the degraded drift-disclosed render is the owed follow-on.
    if envelope.claims().book_digest() != scope.book.1 {
        report_at(
            advisory,
            "whylog",
            None,
            &[Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                dorc_aid::diag::WhylogBookDesync {
                    which: "book".to_owned(),
                },
            ))],
        );
        return Ok(ReplayLoad::Refused);
    }
    if !replay_claims_match(&envelope, &scope) {
        return Ok(refuse_replay(
            advisory,
            dorc_plan::records::AdmissionRefusal::Framing,
        ));
    }
    let decision_digest = envelope.claims().decision_digest().to_owned();
    let started_at = envelope.claims().started_at();
    let record_stream_version = envelope.record_stream_version();
    let instants: BTreeMap<u64, dorc_core::RunInstant> =
        envelope.recorded_instants().iter().copied().collect();
    match dorc_plan::whylog::admit_unscoped_whylog_replay(
        envelope,
        &framing,
        dorc_plan::records::HostEvidenceLimits::spike_default(),
    ) {
        dorc_plan::records::Admission::Admitted(replay) => Ok(ReplayLoad::Admitted(Replay {
            book_path,
            oracle_paths,
            decision_digest,
            started_at,
            record_stream_version,
            instants: instants.clone(),
            records: Some(replay.records().clone()),
        })),
        dorc_plan::records::Admission::NoObservation => Ok(ReplayLoad::NoObservation(Replay {
            book_path,
            oracle_paths,
            decision_digest,
            started_at,
            record_stream_version,
            instants: instants.clone(),
            records: None,
        })),
        dorc_plan::records::Admission::Refused(reason) => Ok(refuse_replay(advisory, reason)),
    }
}

/// The ceiling on a source file a DURABLE named, as opposed to one the admin typed
/// (`rul-host-bytes-bounded-before-admission`: limits are injectable policy, not timeless truth —
/// this is the cli-local policy value, sibling to [`WHYLOG_CAP`]).
const REPLAY_SOURCE_CAP: u64 = 16 * 1024 * 1024;

/// Read one book or oracle a durable NAMED, bounded and regular-file-only.
///
/// `28F:rul-path-hint-must-match-its-doc`. `RecordedSourcePathHint` says it is never a
/// source-loading capability, and this is the one seat that comes closest to making it one: the
/// digest comparison that decides whether the named file is the run's file happens AFTER the read,
/// so the read itself has to be safe on its own terms. Two ways it was not:
///
/// * unbounded — `/dev/zero` or a huge file at the named path was slurped whole before any check;
/// * unfiltered — a FIFO at the named path blocks the replay forever, which no timeout would catch
///   because nothing here has one.
///
/// So: `symlink_metadata` (never a following stat) must say regular file, the size must be under
/// [`REPLAY_SOURCE_CAP`], and the read is `take`-bounded anyway rather than trusting the stat it
/// just did. The result is still only a CANDIDATE — `replay_claims_match` remains the thing that
/// decides it is the run's book, and a mismatch refuses.
fn read_replay_source(path: impl AsRef<std::path::Path>) -> Result<String, ()> {
    use std::io::Read as _;

    let path = path.as_ref();
    let metadata = std::fs::symlink_metadata(path).map_err(|_| ())?;
    if !metadata.is_file() || metadata.len() > REPLAY_SOURCE_CAP {
        return Err(());
    }
    let file = std::fs::File::open(path).map_err(|_| ())?;
    let mut source = String::new();
    let read = file
        .take(REPLAY_SOURCE_CAP.saturating_add(1))
        .read_to_string(&mut source)
        .map_err(|_| ())?;
    if read as u64 > REPLAY_SOURCE_CAP {
        return Err(());
    }
    Ok(source)
}

fn refuse_replay(advisory: bool, reason: dorc_plan::records::AdmissionRefusal) -> ReplayLoad {
    report_at(advisory, "whylog", None, &[reason.spanless_diagnostic()]);
    ReplayLoad::Refused
}

fn replay_claims_match(
    envelope: &dorc_plan::whylog::UnscopedWhylogEnvelope,
    scope: &WidthOneAttemptScope,
) -> bool {
    let claims = envelope.claims();
    claims.nonce() == scope.nonce
        && claims.attempt() == scope.attempt
        && claims.host() == scope.host
        && claims.target() == "width-one"
        && claims.generation() == "width-one"
        && envelope.mode() == "whylog-replay"
        && envelope.recorded_book_path().as_str() == scope.book.0
        && claims.book_digest() == scope.book.1
        && envelope.recorded_oracles().len() == scope.sources.len()
        && envelope
            .recorded_oracles()
            .iter()
            .zip(&scope.sources)
            .enumerate()
            .all(|(ordinal, (recorded, current))| {
                recorded.ordinal() == ordinal
                    && recorded.path().as_str() == current.0
                    && recorded.digest() == current.1
            })
}

/// Write the durable for a completed run (`27V` Lane B), through the hardened store.
///
/// Every refusal is REPORTED (`28F:rul-write-failure-is-error-floor`) — this used to swallow five
/// of them, which `28D:must-retention-is-one-decision` names as one of the whylog's five
/// each-looked-local decisions. The artifact on stdout is still untouched by any of this: the
/// durable is a postmortem aid, so a failure to keep one is loud, not fatal.
fn write_whylog(
    dir: &str,
    metadata: &dorc_plan::whylog::WhylogV2Metadata,
    records: &dorc_plan::records::AdmittedUnscopedHostRecords,
) {
    let write = dorc_plan::whylog::WhylogV2Write::new(metadata, records);
    let bytes = match dorc_plan::whylog::try_serialize_v2(
        &write,
        dorc_plan::whylog::WhylogLimits::spike_default(),
    ) {
        Ok(bytes) => bytes,
        Err(refusal) => return report_whylog_unwritten(dir, serialize_refusal_reason(refusal)),
    };
    if let Err(refusal) = whylog_store::publish(dir, &bytes, WHYLOG_CAP, WHYLOG_KEEP) {
        report_whylog_unwritten(dir, refusal.reason());
    }
}

/// Where this run's receipt goes: the admin's `--whylog-dir`, else the per-user state directory.
///
/// `None` on two very different grounds, and the difference is why this returns an Option rather
/// than a path: `--no-whylog` is a REFUSAL the admin typed, and an unresolvable state root is an
/// environment with nowhere to put anything. Neither is a persistence failure, so neither reports
/// one — the failures are what happens once a destination exists (`whylog-unwritten`).
fn durable_destination(args: &Args) -> Option<String> {
    if args.no_whylog {
        return None;
    }
    match &args.whylog_dir {
        Some(named) => Some(named.clone()),
        None => whylog_store::default_root().map(|root| root.to_string_lossy().into_owned()),
    }
}

/// Report a durable that did not land. Deliberately on [`report`], not `report_at`: the advisory
/// filter drops everything under `apply`, and an apply whose receipt vanished is precisely the run
/// an admin will come back asking about.
fn report_whylog_unwritten(dir: &str, reason: &str) {
    report(
        "whylog",
        None,
        &[Diag::new_spanless_site(DiagCode::WhylogUnwritten(
            dorc_aid::diag::WhylogUnwritten {
                dir: dir.to_owned(),
                reason: reason.to_owned(),
            },
        ))],
    );
}

/// The closed reason word a serializer refusal carries into the diagnostic.
const fn serialize_refusal_reason(refusal: dorc_plan::whylog::WhylogWriteRefusal) -> &'static str {
    use dorc_plan::whylog::WhylogWriteRefusal as R;
    match refusal {
        R::Limit => "limit",
        R::Grammar => "grammar",
        R::Numeric => "numeric",
        R::Digest => "digest",
        R::ArithmeticOverflow => "overflow",
    }
}

/// Assemble the thin durable from a completed run (`27V` §2). The apply report records the PREDICTED
/// per-leaf disposition (`predicted=true`) — the spike has no apply executor (`tc-apply-report-is-
/// prediction`); the field shape is additive so a real executor fills genuine outcomes later.
#[expect(
    clippy::too_many_arguments,
    reason = "the invocation record IS a wide tuple of independent invocation facts (framing/book/oracles/digest/plan/instant); bundling them behind a params struct would just re-spell this signature one layer down"
)]
fn assemble_whylog_metadata(
    framing: &dorc_plan::records::Framing,
    book_name: &str,
    book_src: &str,
    oracle_paths: &[String],
    oracle_srcs: &[String],
    decision_digest: &str,
    plan: &dorc_plan::Plan,
    started_at: Option<dorc_core::RunInstant>,
    results: &SiteResults,
) -> dorc_plan::whylog::WhylogV2Metadata {
    let apply = plan
        .steps
        .iter()
        .map(|s| dorc_plan::whylog::ApplyLine {
            leaf: s.leaf.0,
            disposition: disposition_tag(&s.disposition).to_owned(),
            predicted: true,
        })
        .collect();
    dorc_plan::whylog::WhylogV2Metadata {
        mode: "whylog-replay".to_owned(),
        argv: std::env::args().collect(),
        book: (book_name.to_owned(), book_digest(book_src)),
        oracles: oracle_paths
            .iter()
            .zip(oracle_srcs)
            .map(|(p, s)| (p.clone(), book_digest(s)))
            .collect(),
        nonce: framing.nonce.0.clone(),
        attempt: framing.attempt,
        host: framing.host.clone(),
        decision_digest: decision_digest.to_owned(),
        started_at,
        instants: results
            .records
            .values()
            .filter_map(|record| Some((record.stamp.ordinal, record.stamp.received_at?)))
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect(),
        apply,
    }
}

/// R3 (23D §1 — the check IS the oracle): resolve the stripped `<provider>__predict` funcdef
/// a probe site ships, given its resolved (provider-word, argv-after-word0). Re-runs the
/// SAME resolution [`dorc_analysis::effect`] used — the FIRST check, in oracle-file order,
/// whose provider matches (through the shared hyphen↔underscore
/// [`map_provider_name`](dorc_oracle::predict::map_provider_name) convention) AND whose own
/// argparse [`evaluate`](dorc_oracle::predict::evaluate)s this argv concretely — then
/// [`strip_predict`](dorc_oracle::predict::strip_predict)s it. Matching the analysis's resolution
/// is load-bearing: the shipped probe must check exactly the fact the analysis decided
/// (a provider with two checks — `apt-get` as `package` and `pkgindex` — resolves per argv,
/// `install …` ⇒ package, `update` ⇒ whichever resolves first). `None` ⇒ no check resolves
/// ⇒ the site is un-shippable ⇒ un-elidable (`kFAIL-perform`).
fn ship_predict_body(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::{Resolution, evaluate, map_provider_name, strip_predict};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    for (idx, (src, cs)) in oracle_srcs.iter().zip(checks).enumerate() {
        for cp in cs.providers() {
            if map_provider_name(interner.resolve(cp)) != want {
                continue;
            }
            let Some(check) = cs.get(cp) else { continue };
            if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                return Some(dorc_plan::ShippedCheck::predict(
                    strip_predict(src, check, interner),
                    Some((check.name_span, oracle_file_id(idx))),
                ));
            }
        }
    }
    None
}

/// The loaded-oracle index a threaded span belongs to (`law-lineno-identity`): the position in the
/// driver's ordered oracle-source list, which is the ONE disambiguator between two oracles'
/// line-number spaces. Saturating rather than panicking (`inv-no-throw`).
fn oracle_file_id(idx: usize) -> dorc_core::OracleFileId {
    dorc_core::OracleFileId(u32::try_from(idx).unwrap_or(u32::MAX))
}

/// Resolve a connected pipe STAGE's stripped `<provider>__predict` body PLUS its STDOUT coverage
/// (`271:rul-only-oracle-bytes-ship` rider 1 — the composed-probe repair). Mirrors
/// [`ship_predict_body`]'s check-resolution, then asks
/// [`predict_stage_stdout`](dorc_oracle::predict::predict_stage_stdout) whether the arm this argv
/// selects produces REAL (delegation-produced) stdout bytes — the coverage a downstream byte-consumer
/// requires. `None` ⇒ no check resolves ⇒ the stage is un-shippable ⇒ the compound refuses (⇒ runs).
fn ship_predict_stage(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<dorc_plan::StageShip> {
    use dorc_oracle::predict::{
        Resolution, StageStdout, evaluate, map_provider_name, predict_stage_stdout, strip_predict,
    };
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    for (src, cs) in oracle_srcs.iter().zip(checks) {
        for cp in cs.providers() {
            if map_provider_name(interner.resolve(cp)) != want {
                continue;
            }
            let Some(check) = cs.get(cp) else { continue };
            if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                return Some(dorc_plan::StageShip {
                    sh: strip_predict(src, check, interner),
                    produces_real_stdout: predict_stage_stdout(check, &arg_refs)
                        == StageStdout::RealBytes,
                });
            }
        }
    }
    None
}

/// `24L` §2 — resolve the stripped `<provider>__is_converged` verdict funcdef a typeless-floor
/// auto-cell probe ships. Mirrors [`ship_predict_body`] over the pre-lifted [`VerdictSet`]s but
/// keys on the verdict lane and needs no argv (the strip is argv-independent; the invocation adds
/// the site argv at render, so the host runs `<provider>__is_converged <argv>` and its rc maps to
/// the Effect verdict through the record scaffold's rc-partition). `None` ⇒ the provider authored
/// no verdict funcdef (should not happen for an auto-cell — its mint gated on exactly this — so a
/// `None` here safely folds the site to unresolvable ⇒ run).
fn ship_verdict_body(
    oracle_srcs: &[String],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &Interner,
    provider: Symbol,
) -> Option<dorc_plan::ShippedCheck> {
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    let want = map_provider_name(interner.resolve(provider));
    for (idx, (src, set)) in oracle_srcs.iter().zip(verdict_sets).enumerate() {
        for vp in set.providers() {
            if map_provider_name(interner.resolve(vp)) != want {
                continue;
            }
            let Some(verdict) = set.get(vp) else { continue };
            // `27W` §3 C4: pair the body with whether it emits report lines (gates the tier-3 drain).
            let emits_report = dorc_oracle::report::emits_report(verdict);
            return Some(dorc_plan::ShippedCheck::verdict(
                strip_verdict(src, verdict, interner),
                Some((verdict.name_span, oracle_file_id(idx))),
                emits_report,
            ));
        }
    }
    None
}

/// Lift the survival footprints (Stage 2 / rul24-mode-gate) — called ONLY on the
/// `--risk-faultless-skips` path (TC-1: the footprint data does not exist unflagged). For each
/// wall-candidate site (an establish-bearing class, or a kill) whose provider declares a
/// `touches()`, trace it over the site's resolved argv and record the emitted footprint —
/// after a **coherence check** (23M / the Stage-2 brief): the site's OWN establish coordinate
/// must be ⊆ its lifted footprint (at-least ⊆ at-most), else the footprint is a loud
/// contradiction and is REFUSED (⇒ the site walls). A ⊤/empty lift, a non-literal argv, or a
/// missing `touches()` all mean "no trustworthy footprint" ⇒ absence from the map ⇒ wall.
///
/// `inv-referent-agnostic`: emitted `kind:entity` fragments are interned into the SAME
/// vocabulary the book/predict analysis uses (one interner) — `package` here is the SAME
/// [`KindId`] a predict annotation minted — never a parallel string-typed universe (24A §1b).
#[expect(
    clippy::too_many_arguments,
    reason = "the cli-edge footprint lift threads the whole compiled context (oracles/classes/kills/kill-coords/value/cfg/ast/interner) + the advisory routing flag; each is a distinct pipeline output, not a bundle-able struct"
)]
fn build_survival_footprints(
    oracle_refs: &[&str],
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    value: &dorc_analysis::value::ValueFlow,
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    interner: &mut Interner,
    advisory: bool,
) -> dorc_plan::TrustedFootprints {
    use dorc_analysis::effect::SkipClass;
    let touches_sets: Vec<dorc_oracle::touches::TouchesSet> = oracle_refs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::touches::TouchesSet::lift(interner, src);
            report_at(advisory, "touches", None, &lifted.diags);
            lifted.value
        })
        .collect();

    let mut footprints = dorc_plan::TrustedFootprints::new();
    let mut diags = Vec::new();
    for (node, class) in classes {
        // A wall candidate: an establish-bearing class OR a kill. Both now carry their OWN effect
        // coordinate for the coherence check (24E §7: the kill's coord rides `kill_coords`).
        let establish = match class {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        };
        if establish.is_none() && !kills.contains(node) {
            continue; // not a wall candidate (a pure builtin, a Query, an opaque)
        }
        let Some((provider, coords_with_selectors, arm_span)) =
            resolve_touches_footprint(*node, value, &touches_sets, interner)
        else {
            continue; // no touches / non-literal argv / ⊤ / empty emission ⇒ no footprint ⇒ wall
        };
        let coords: Vec<dorc_plan::EntityCoord> =
            coords_with_selectors.iter().map(|(c, _)| *c).collect();
        let own = own_wall_coord(*node, classes, kill_coords);
        // Coherence CANARY (authored lane only, PRE-union — 24G §8 / 24E §7): the site's OWN effect
        // coordinate (its establish, or its killed cell) must be ⊆ the author's RAW `touches()`
        // emission (at-least ⊆ at-most). A violation is a cross-lane contradiction — the author's
        // touches() disagrees with their own establish/kill — ⇒ refuse ⇒ wall. Real teeth here, and
        // UNCHANGED. Closes resid-kill-coherence (a drifted kill footprint omitting the killed cell).
        if let Some(own_coord) = own
            && !coords.contains(&own_coord)
        {
            let span = ast.node(cfg.node(*node).ast).span;
            diags.push(Diag::new(
                DiagCode::FootprintIncoherent(FootprintIncoherent {
                    detail: "touches() footprint omits this command's own effect coordinate \
                             (at-least ⊄ at-most) — footprint refused, the site walls"
                        .to_string(),
                }),
                span,
            ));
            continue;
        }
        // 24G §8: UNION the site's own effect coordinate (engine-supplied provenance) into the
        // footprint. A no-op on the hit-surface HERE (the canary just proved own ∈ coords), but it
        // records own for the why-lens and keeps the two lanes uniform. Empty emission ⇒ None from
        // `authored` ⇒ `with_own` cannot resurrect it (anti-233).
        // `tc-disturbs-span-threading`: the MATCHED ARM over the funcdef, still the honest floor.
        let defining =
            arm_span.or_else(|| touches_defining_span(provider, &touches_sets, interner));
        if let Some(mut footprint) = dorc_plan::Footprint::authored(provider, coords)
            .map(|fp| fp.with_own(own).with_defining(defining))
        {
            // `277` §3: record each emission's `@selector` so a selector-bearing disturbs mark can
            // SPARE a sibling cell under the dialect. Whole-entity emissions (the corpus default,
            // `None`) record nothing ⇒ ⊤ ⇒ collide (empty-world-byte-identical).
            for (coord, selector) in coords_with_selectors {
                if let Some(sel) = selector {
                    footprint.set_selector(coord, sel);
                }
            }
            footprints.insert(*node, footprint);
        }
    }
    report_at(advisory, "footprint", None, &diags);
    footprints
}

/// Resolve a wall-candidate site's `touches()` footprint: split its resolved argv into
/// `(provider, operands)` (all must be literal — a ⊤ word ⇒ no footprint), find the provider's
/// touches funcdef (through the shared hyphen↔underscore convention, like the probe), trace it,
/// and intern the emitted coordinates. `None` ⇒ any of: non-literal argv, no matching
/// `touches()`, a ⊤ trace, or an EMPTY emission (no claim = wall).
/// One footprint coordinate plus its disturbs-emission selector (`277` §3): the entity-granular
/// [`dorc_plan::EntityCoord`] that drives canonicalization/render, and the `@selector` cell the
/// dialect consults (`None` ⇒ whole-entity ⊤).
type FootprintCoord = (dorc_plan::EntityCoord, Option<dorc_core::SelectorId>);

/// One resolved `disturbs` footprint: whose claim it is, the cells it names, and the arm that
/// emitted them (`tc-disturbs-span-threading`; `None` when the trace located no emitting line).
type ResolvedFootprint = (
    Symbol,
    Vec<FootprintCoord>,
    Option<(dorc_core::Span, dorc_core::OracleFileId)>,
);

fn resolve_touches_footprint(
    node: dorc_analysis::cfg::CfgNodeId,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
) -> Option<ResolvedFootprint> {
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::touches::{TouchesResolution, evaluate_touches_located};

    let argv = value.argv_values(node);
    let (first, rest) = argv.split_first()?;
    let ValueOf::Literal(provider) = first else {
        return None; // ⊤ command word
    };
    let mut arg_texts = Vec::with_capacity(rest.len());
    for w in rest {
        let ValueOf::Literal(s) = w else {
            return None; // a ⊤ operand ⇒ the argparse cannot resolve ⇒ no footprint
        };
        arg_texts.push(interner.resolve(*s).to_owned());
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    let want = map_provider_name(interner.resolve(*provider));
    let (coords, arm) = touches_sets.iter().enumerate().find_map(|(index, set)| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .and_then(
                |touches| match evaluate_touches_located(touches, &arg_refs) {
                    (TouchesResolution::Emitted(coords), arm) if !coords.is_empty() => Some((
                        coords,
                        arm.map(|span| {
                            (
                                span,
                                dorc_core::OracleFileId(u32::try_from(index).unwrap_or(u32::MAX)),
                            )
                        }),
                    )),
                    // Emitted(empty) = no claim = wall; Top = ⊤ = wall. Both ⇒ no footprint.
                    (TouchesResolution::Emitted(_) | TouchesResolution::Top(_), _) => None,
                },
            )
    })?;

    // Intern each opaque `kind:entity@selector` fragment into the shared vocabulary (the fence).
    // The selector rides alongside the entity-granular coord (`277` §3): absent ⇒ whole-entity ⊤.
    let entity_coords = coords
        .iter()
        .map(|c| {
            let kind = dorc_core::KindId(interner.intern(&c.kind));
            let entity = match &c.entity {
                Some(text) => {
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(text)))
                }
                None => dorc_core::EntityRef::Singleton,
            };
            let selector = c
                .selector
                .as_deref()
                .map(|s| dorc_core::SelectorId(interner.intern(s)));
            (dorc_plan::EntityCoord::new(kind, entity), selector)
        })
        .collect();
    Some((*provider, entity_coords, arm))
}

/// The `disturbs` funcdef's defining `(Span, OracleFileId)` for a provider (`tc-disturbs-span-
/// threading`; `27V:mech-minting-line-threading`) — a NAME-keyed lookup (no argv trace): the touches
/// funcdef's `name_span` is the leverage point a survival's `claimed` link points at ("the line to
/// widen"). The funcdef `name_span` is the honest coarsest-true span; per-arm precision is deferred.
/// `None` when the provider has no touches funcdef in the loaded set.
fn touches_defining_span(
    provider: Symbol,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &Interner,
) -> Option<(dorc_core::Span, dorc_core::OracleFileId)> {
    use dorc_oracle::predict::map_provider_name;
    let want = map_provider_name(interner.resolve(provider));
    touches_sets.iter().enumerate().find_map(|(idx, set)| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .map(|t| {
                (
                    t.name_span,
                    dorc_core::OracleFileId(u32::try_from(idx).unwrap_or(u32::MAX)),
                )
            })
    })
}

/// The derivation-probe seam (24E §2/§3 — fork-4A: the SAME self-vouch tier as `predict`, no new
/// trust edge): for a wall-candidate site's (provider-word, argv), find the provider's `touches()`
/// funcdef and trace it statically. `Some(DerivationShip)` iff the trace ESCALATED — it ⊤'d
/// specifically on a `NonPrintfCommand` (the body reached a host query the static tracer cannot
/// resolve, e.g. `dpkg -L`), the sanctioned escalation trigger (fork-4B). The body then ships
/// strip-only (`strip_touches`; the funcdef mangles to `<provider>__disturbs`), the SAME strip
/// discipline as the probe/guard lanes. `None` for: a statically-resolvable body (`Emitted` — the
/// authored-footprint lane owns it), any OTHER ⊤ (degrade-to-wall, fork-4B — the site runs), an
/// empty emission, or a provider with no touches funcdef. `inv-referent-agnostic`: the operands are
/// resolved for the trace/invocation, never decoded.
fn ship_touches_body(
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<dorc_plan::DerivationShip> {
    use dorc_oracle::predict::{map_provider_name, strip_touches};
    use dorc_oracle::touches::{TouchesResolution, TouchesTop, evaluate_touches};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    touches_paired.iter().find_map(|(src, set)| {
        let p = set
            .providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
        let touches = set.get(p)?;
        match evaluate_touches(touches, &arg_refs) {
            // The EXPECTED escalation (24E §4): the body reached a host query ⇒ ship it.
            TouchesResolution::Top(TouchesTop::NonPrintfCommand) => {
                Some(dorc_plan::DerivationShip {
                    // Display the BOOK command word (`apt-get`), not the munged funcdef segment
                    // (`apt_get`, the forward-munge key) — the why-lens reads better with the word
                    // the admin wrote (`24C:rul24-totalistic-munge` keeps the segment internal).
                    call: format!("{}.touches()", interner.resolve(provider)),
                    sh: strip_touches(src, touches, interner),
                })
            }
            // Static-resolvable, an OTHER ⊤ (degrade-to-wall), or empty ⇒ NOT a derivation.
            TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
        }
    })
}

/// Read back the host-DERIVED footprints (24E §2 corr-§2) and merge them into the survival set.
/// For each escalated [`dorc_plan::ProbeDerivation`], intern its readback `deriv` coordinate lines
/// into the SHARED vocabulary (the 24A §1b fence — `package` here is the SAME [`dorc_core::KindId`]
/// a predict annotation minted), build a `Derived` [`dorc_plan::Footprint`], and UNION the site's own
/// effect coordinate into it (24G §8 — the derived lane no longer REQUIRES own-membership; the
/// boilerplate `printf 'kind:%s' "$1"` that used to supply it was a decoy the coherence check tested
/// instead of the derivation). Insert keyed by the site's node. An escalated site with NO readback
/// records ⇒ empty ⇒ wall (silence = wall, kFAIL-safe).
///
/// ALL-OR-NOTHING (24E §4 / the static path's TC-4): a MALFORMED derived coordinate refuses the
/// WHOLE footprint (the site walls) — never silently dropped, because a footprint is an *at-most*
/// claim and dropping a coordinate NARROWS it (⇒ a downstream fact wrongly survives ⇒ under-execute).
///
/// SPIKE-ONLY (ru-26): the `touches-escalated` advisory below makes the static→dynamic boundary
/// visible in the render/differential; it must NOT leak into greenfield as a permanent
/// per-escalation requirement.
#[expect(
    clippy::too_many_arguments,
    reason = "the derived-footprint merge threads the compiled context (footprints/derivations/results/classes/kill-coords/node-spans/interner) + the book-source and advisory routing; each is a distinct pipeline output, not a bundle-able struct"
)]
fn merge_derived_footprints(
    footprints: &mut dorc_plan::TrustedFootprints,
    derivations: &dorc_plan::DerivationPlan,
    results: &SiteResults,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    node_spans: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::Span>,
    interner: &mut Interner,
    book_source: Option<(&str, &str)>,
    advisory: bool,
) {
    let mut diags = Vec::new();
    for d in &derivations.derivations {
        // The escalated book command's own span (`aid-caret-span-precision`): every diag this loop
        // emits points at the site that escalated (the same `CfgNodeId`→AST span the canary uses,
        // precomputed at the cli edge). Absent ⇒ wall this site silently (kFAIL-safe; never happens
        // in production, where the map covers every derivation node).
        let Some(&span) = node_spans.get(&d.node) else {
            continue;
        };
        diags.push(Diag::new(
            DiagCode::TouchesEscalated(TouchesEscalated {
                site: d.site.0,
                call: d.call.clone(),
            }),
            span,
        ));
        let Some(coord_strs) = results.derivations.get(&d.site) else {
            continue; // no readback records ⇒ empty derived footprint ⇒ wall (kFAIL-safe)
        };
        // `262` §2 / `26A` stop-1 — THE at-most family completeness gate. A deriv footprint is
        // an AT-MOST claim, so a mid-family cut SHRINKS it (⇒ more survivals — the
        // under-execution cardinal sin). The family MUST close with `deriv-end n=<K>` whose K
        // equals the received coord count; a missing end-record or a count mismatch ⇒ the
        // family is INCOMPLETE ⇒ refuse the footprint ⇒ the site walls TOTAL (never keep a
        // partial at-most family). This is the SAME wall-total path as the malformed-coord
        // refusal below.
        match results.derivation_ends.get(&d.site) {
            // Legacy (unframed) fixtures carry no `deriv-end`; they are trusted-complete, so the
            // gate is framed-only (the framed round-trip + DST enforce the real contract).
            _ if !results.framed => {}
            Some(&k) if k as usize == coord_strs.len() => {}
            reason => {
                diags.push(Diag::new(
                    DiagCode::DerivFamilyIncomplete(DerivFamilyIncomplete {
                        site: d.site.0,
                        reason: match reason {
                            Some(&k) => format!("declared n={k}, received {}", coord_strs.len()),
                            None => "no deriv-end close-record".to_string(),
                        },
                    }),
                    span,
                ));
                continue;
            }
        }
        let mut coords = Vec::with_capacity(coord_strs.len());
        let mut malformed = false;
        for line in coord_strs {
            if let Some(c) = intern_coordinate(line, interner) {
                coords.push(c);
            } else {
                malformed = true;
                break;
            }
        }
        if malformed {
            diags.push(Diag::new(
                DiagCode::FootprintIncoherent(FootprintIncoherent {
                    detail: "derived touches() emitted a malformed coordinate (not kind:entity) \
                             — footprint refused, the site walls (an at-most claim cannot be \
                             partial)"
                        .to_string(),
                }),
                span,
            ));
            continue;
        }
        // 24G §8: the DERIVED lane DROPS the own-membership requirement — the boilerplate
        // `printf 'kind:%s' "$1"` that satisfied it was a DECOY the check tested INSTEAD of the
        // derivation. UNION the site's own effect coordinate (its establish, or its killed cell from
        // `kill_coords`) into the footprint instead — engine-supplied provenance. An empty emission
        // still walls: `derived` returns None on empty coords ⇒ `with_own` cannot resurrect it (the
        // anti-233 boundary — the engine never manufactures a claim from silence).
        let own = own_wall_coord(d.node, classes, kill_coords);
        if let Some(fp) = dorc_plan::Footprint::derived(d.provider, coords, d.call.clone())
            .map(|fp| fp.with_own(own))
        {
            footprints.insert(d.node, fp);
        }
    }
    report_at(advisory, "derive", book_source, &diags);
}

/// Intern one readback `kind:entity` coordinate line into the shared vocabulary (24A §1b fence —
/// split on the FIRST `:`; an empty entity is the kind's singleton). `None` on a malformed line
/// (no `:` / empty kind) — the caller refuses the WHOLE footprint (all-or-nothing).
fn intern_coordinate(line: &str, interner: &mut Interner) -> Option<dorc_plan::EntityCoord> {
    let (kind, entity) = line.split_once(':')?;
    if kind.is_empty() {
        return None;
    }
    let kind = dorc_core::KindId(interner.intern(kind));
    let entity = if entity.is_empty() {
        dorc_core::EntityRef::Singleton
    } else {
        dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(entity)))
    };
    Some(dorc_plan::EntityCoord::new(kind, entity))
}

/// The establish fact a wall-candidate node establishes, if it is an establish class. A kill's
/// coordinate rides the `kill_coords` side-map instead (24E §7).
fn establish_fact_of(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    node: dorc_analysis::cfg::CfgNodeId,
) -> Option<dorc_core::FactKey> {
    use dorc_analysis::effect::SkipClass;
    classes.iter().find_map(|(n, c)| {
        if *n != node {
            return None;
        }
        match c {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        }
    })
}

/// The wall-candidate node's OWN effect coordinate — the coherence comparand (own ⊆ footprint,
/// 24E §7): its establish coordinate (an establish class) OR its killed coordinate (a kill node,
/// from `kill_coords`). `None` for a node with neither (nothing to check coherence against). This
/// unifies the establish-wall check (Stage 2) with the kill-wall check (24E §7) for BOTH the
/// authored and derived footprint lanes.
fn own_wall_coord(
    node: dorc_analysis::cfg::CfgNodeId,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
) -> Option<dorc_plan::EntityCoord> {
    establish_fact_of(classes, node)
        .or_else(|| kill_coords.get(&node).copied())
        .map(|f| dorc_plan::EntityCoord::new(f.kind, f.entity))
}

/// The per-KIND resolvers (24F §3, corr-kind-keying §10): `<kind>.resolve()` funcdefs lifted per
/// oracle file, combined with CONFUSABILITY enforcement. Resolvers are a SECOND family keyed by KIND
/// (the kind-owner holds the nouns — 23M contribution-vs-identity), NOT per-command role-siblings;
/// the engine looks one up by a coordinate's kind symbol, never its provider.
struct KindResolvers {
    /// Per-file resolver sets (indexed by `by_kind`).
    sets: Vec<dorc_oracle::resolve::ResolverSet>,
    /// The kept, non-conflicting map from a RAW coordinate kind to `(file-index, munged-base
    /// symbol)`. Kind-keyed funcdefs are NAMED by the kind's forward-munge (`sm_dorc_Package__resolve`),
    /// so the inner [`ResolverSet`] is keyed by the munged base; this map re-keys to the RAW kind a
    /// coordinate carries (`flag-forward-munge-keying`), so a lookup by `coord.kind()` finds the
    /// funcdef named by that kind's munge. A kind ABSENT here is resolver-LESS (the token-equality
    /// floor) — never declared, or REFUSED for a cross-file duplicate.
    by_kind: BTreeMap<Symbol, (usize, Symbol)>,
}

impl KindResolvers {
    /// The resolver-bearing RAW kinds (the engine marks each; a coordinate of such a kind that fails
    /// to resolve degrades to may-alias, §3a).
    fn resolver_kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.by_kind.keys().copied()
    }

    /// The `(file-index, resolver funcdef)` for a RAW coordinate kind, if it has a kept resolver.
    fn get(&self, kind: Symbol) -> Option<(usize, &dorc_oracle::predict::Predict)> {
        let (idx, base) = *self.by_kind.get(&kind)?;
        self.sets.get(idx)?.get(base).map(|p| (idx, p))
    }
}

/// Lift the per-kind resolvers + ENFORCE confusability (24F §3 / corr-kind-keying §10 — a LOUD
/// diagnostic, never a silent dud). Two checks: (1) at-most-one-resolver-per-kind — two files
/// declaring `<kind>.resolve()` for the SAME kind ⇒ REFUSE BOTH (the kind stays resolver-less) + an
/// error; (2) a resolver keyed to a name matching a known PROVIDER (a lifted predict/touches
/// command) ⇒ a WARNING (the exact mis-keying the brief itself made — `apt-get.resolve()` would mint
/// identity for a "kind" no coordinate uses). `inv-referent-agnostic`: names compared as interned
/// symbols/strings, never decoded.
fn build_kind_resolvers(
    oracle_srcs: &[String],
    oracle_paths: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    coord_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
    advisory: bool,
) -> KindResolvers {
    use dorc_oracle::resolve::ResolverSet;
    use dorc_oracle::to_funcname_segment;

    let sets: Vec<ResolverSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = ResolverSet::lift(interner, src);
            report_at(advisory, "resolve", None, &lifted.diags);
            lifted.value
        })
        .collect();

    // Every (kind, file-index) declaration, grouped by kind (the same kind in ≥2 files is a conflict).
    let mut per_kind: BTreeMap<Symbol, Vec<usize>> = BTreeMap::new();
    for (idx, set) in sets.iter().enumerate() {
        for kind in set.kinds() {
            per_kind.entry(kind).or_default().push(idx);
        }
    }

    // The known PROVIDER names, FORWARD-MUNGED into NAME space (`flag-forward-munge-keying`: a
    // kind-keyed resolver interns its base by the kind's forward-munge, so the collision compares in
    // the same NAME space the funcdefs live in) — a resolver whose kind munges to a provider's is the
    // mis-keying we warn on.
    let mut providers: BTreeSet<String> = BTreeSet::new();
    for cs in checks {
        for p in cs.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }
    for (_, ts) in touches_paired {
        for p in ts.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }

    let mut diags_by_file: BTreeMap<usize, Vec<Diag>> = BTreeMap::new();
    let mut base_to_idx: BTreeMap<Symbol, usize> = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        // The diagnostic points at the FIRST declaring file's `<kind>__resolve` funcdef name
        // (`aid-caret-span-precision`); the file index carries its `law-lineno-identity` space.
        let anchor = files
            .first()
            .and_then(|&idx| Some((idx, sets.get(idx)?.get(kind)?.name_span)));
        if files.len() > 1 {
            if let Some((idx, span)) = anchor {
                diags_by_file.entry(idx).or_default().push(Diag::new(
                    DiagCode::ResolverConflict(ResolverConflict {
                        kind: name.clone(),
                        count: files.len(),
                    }),
                    span,
                ));
            }
            continue; // refuse both ⇒ resolver-less
        }
        if providers.contains(&name)
            && let Some((idx, span)) = anchor
        {
            // Kept (it may legitimately match a kind of the same name); the warning surfaces the risk.
            diags_by_file.entry(idx).or_default().push(Diag::new(
                DiagCode::ResolverProviderCollision(ResolverProviderCollision {
                    name: name.clone(),
                }),
                span,
            ));
        }
        if let Some(&idx) = files.first() {
            base_to_idx.insert(kind, idx);
        }
    }
    report_by_oracle_file(
        advisory,
        "resolve",
        oracle_paths,
        oracle_srcs,
        &diags_by_file,
    );
    let by_kind = rekey_to_raw_kinds(&base_to_idx, coord_kinds, interner);
    KindResolvers { sets, by_kind }
}

/// Re-key a kind-keyed `munged-base → file-index` map to the RAW coordinate kinds
/// (`flag-forward-munge-keying`). A raw coord kind K maps to `(idx, munged-base)` iff its
/// forward-munge is a kept base. Shared by [`build_kind_resolvers`] and [`build_kind_reaches`].
fn rekey_to_raw_kinds(
    base_to_idx: &BTreeMap<Symbol, usize>,
    coord_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
) -> BTreeMap<Symbol, (usize, Symbol)> {
    let mut by_kind = BTreeMap::new();
    for &raw in coord_kinds {
        let munged_text = dorc_oracle::to_funcname_segment(interner.resolve(raw));
        let base = interner.intern(&munged_text);
        if let Some(&idx) = base_to_idx.get(&base) {
            by_kind.insert(raw, (idx, base));
        }
    }
    by_kind
}

/// Collect the coordinates that need canonicalization (24F §3): every establish/query BACKING coord
/// and every wall-candidate FOOTPRINT coord whose KIND is resolver-bearing. Deduplicated (resolution
/// is a pure function of `(kind, entity)`) and deterministic (`BTreeSet`). Derived-footprint coords
/// (escalated walls, resolved only post-results) are NOT covered — a resolver+derived combination is
/// a second round-trip, deferred (noted `resid-resolve-derived`).
fn collect_resolver_coords(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    resolver_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
) -> BTreeSet<dorc_plan::EntityCoord> {
    use dorc_analysis::effect::SkipClass;
    let mut coords = BTreeSet::new();
    let consider = |coord: dorc_plan::EntityCoord, coords: &mut BTreeSet<_>| {
        if resolver_kinds.contains(&coord.kind().0) {
            coords.insert(coord);
        }
    };
    for (node, class) in classes {
        // Backing coords: the cell each establish/query site is about.
        if let SkipClass::EstablishAmbient(f)
        | SkipClass::EstablishWritten(f)
        | SkipClass::QueryResolvable { fact: f, .. } = class
        {
            consider(dorc_plan::EntityCoord::new(f.kind, f.entity), &mut coords);
        }
        // Footprint coords: a wall-candidate's touches() emissions.
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
        ) || kills.contains(node);
        if is_wall_candidate
            && let Some((_, fp_coords, _)) =
                resolve_touches_footprint(*node, value, touches_sets, interner)
        {
            for (c, _selector) in fp_coords {
                consider(c, &mut coords);
            }
        }
    }
    coords
}

/// Collect the RAW coordinate kinds present in this analysis — every establish/query BACKING kind
/// plus every wall-candidate FOOTPRINT kind. Used to re-key the munged kind-keyed resolver/reaches
/// maps to the raw kinds coordinates carry (`flag-forward-munge-keying`; [`rekey_to_raw_kinds`]).
///
/// rider-resolver-coverage-watch (`277` §7b): this collected set is EXACTLY the population the
/// survival comparison ([`dorc_plan::survival::disjoint`]) ever canonicalizes — backings come from
/// converged-`Replace` licenses (establish/query classes) and footprints from wall candidates, so
/// every coordinate that reaches a resolver lookup has its kind collected here. The coverage is
/// therefore sound (no silent under-cover); it stays collection-based rather than structural because
/// the resolver-SHIPPING pipeline is a cli-edge concern the comparison-layer re-key does not subsume.
fn collect_coord_kinds(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
) -> BTreeSet<Symbol> {
    use dorc_analysis::effect::SkipClass;
    let mut kinds = BTreeSet::new();
    for (node, class) in classes {
        if let SkipClass::EstablishAmbient(f)
        | SkipClass::EstablishWritten(f)
        | SkipClass::QueryResolvable { fact: f, .. } = class
        {
            kinds.insert(f.kind.0);
        }
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
        ) || kills.contains(node);
        if is_wall_candidate
            && let Some((_, fp_coords, _)) =
                resolve_touches_footprint(*node, value, touches_sets, interner)
        {
            for (c, _selector) in fp_coords {
                kinds.insert(c.kind().0);
            }
        }
    }
    kinds
}

/// Compile the resolver-probe (24F §3): for each resolver-bearing coordinate, ship its kind's
/// stripped `<kind>__resolve` funcdef + a per-coordinate invocation with the entity. Deterministic
/// (coords arrive `BTreeSet`-ordered). A coord whose kind's resolver cannot be resolved/stripped is
/// dropped (defensive — the kind is in `resolver_kinds` by construction, so this is unreachable, but
/// dropping degrades it to may-alias at readback, the safe direction).
fn compile_resolvers(
    coords: &BTreeSet<dorc_plan::EntityCoord>,
    kind_resolvers: &KindResolvers,
    oracle_srcs: &[String],
    interner: &Interner,
) -> dorc_plan::ResolverPlan {
    use dorc_oracle::predict::strip_resolve;
    let mut probes = Vec::new();
    for coord in coords {
        let kind_sym = coord.kind().0;
        let Some((idx, resolver)) = kind_resolvers.get(kind_sym) else {
            continue;
        };
        let Some(src) = oracle_srcs.get(idx) else {
            continue;
        };
        let entity_text = match coord.entity() {
            dorc_core::EntityRef::Operand(tok) => interner.resolve(tok.0).to_owned(),
            dorc_core::EntityRef::Singleton => String::new(),
        };
        probes.push(dorc_plan::ResolverProbe {
            coord_label: render_coord(*coord, interner),
            kind_label: interner.resolve(kind_sym).to_owned(),
            kind_fn: format!(
                "{}__resolve",
                dorc_oracle::to_funcname_segment(interner.resolve(kind_sym))
            ),
            entity_text,
            sh: strip_resolve(src, resolver, interner),
        });
    }
    dorc_plan::ResolverPlan { probes }
}

/// Build the [`dorc_plan::Resolutions`] map (24F §3) from the resolver-probe readback: mark every
/// resolver-bearing kind, record each `canon`, and flag each `dangling`. A resolver-bearing coord
/// with NO readback record degrades to may-alias at canonicalization (§3a — the safe direction).
/// Interning the canonical form through the SHARED interner keeps it in the one vocabulary (the
/// fence); the engine compares canonical tokens as symbols, never decoding (`inv-referent-agnostic`).
fn build_resolutions(
    coords: &BTreeSet<dorc_plan::EntityCoord>,
    resolver_kinds: &BTreeSet<Symbol>,
    readback: &SiteResults,
    interner: &mut Interner,
) -> dorc_plan::Resolutions {
    let mut resolutions = dorc_plan::Resolutions::none();
    for kind in resolver_kinds {
        resolutions.add_resolver_kind(dorc_core::KindId(*kind));
    }
    for coord in coords {
        let label = render_coord(*coord, interner);
        match readback.resolutions.get(&label) {
            Some(ResolvOutcome::Canonical(canon_text)) => {
                let entity = if canon_text.is_empty() {
                    dorc_core::EntityRef::Singleton
                } else {
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(
                        interner.intern(canon_text),
                    ))
                };
                resolutions.record(*coord, entity);
            }
            // Dangling OR no record ⇒ leave unrecorded ⇒ may-alias at canonicalization (§3a). A
            // dangling is additionally flagged for the loud diagnostic (§4).
            Some(ResolvOutcome::Dangling) => resolutions.record_dangling(*coord),
            None => {}
        }
    }
    resolutions
}

/// The DANGLING-reference diagnostics (24F §4): one loud per-coordinate note for each coordinate the
/// resolver flagged dangling (a reference to a non-existent entity on an enumerable kind — the
/// resolver's natural `dpkg-query -W` non-zero). Turns the third-party-typo case from silent
/// value-loss into a pointed hint; the coordinate ALSO rides the may-alias degrade (§3a). ADVISORY —
/// the apply runs the affected site either way (fail toward run), so no correctness rides on this
/// readout; it is the render surface (rec-1). `inv-referent-agnostic`: the coord label is display.
fn dangling_diagnostics(resolutions: &dorc_plan::Resolutions, interner: &Interner) -> Vec<Diag> {
    resolutions
        .dangling()
        .map(|coord| {
            Diag::new_spanless_site(DiagCode::DanglingReference(DanglingReference {
                coord: render_coord(coord, interner),
            }))
        })
        .collect()
}

/// The per-KIND reach-functions (24G §4): `<kind>.reaches()` funcdefs lifted per oracle file, with
/// CONFUSABILITY enforcement — kind-keyed exactly like the resolvers ([`KindResolvers`], corr-kind-keying
/// §10). The engine expands a footprint coord through the reach-function keyed by the coord's kind.
struct KindReaches {
    /// Per-file reach sets (indexed by `by_kind`).
    sets: Vec<dorc_oracle::reaches::ReachesSet>,
    /// The kept, non-conflicting map from a RAW coordinate kind to `(file-index, munged-base symbol)`
    /// — re-keyed from the funcdef's munged base to the raw kind coords carry
    /// (`flag-forward-munge-keying`; see [`KindResolvers::by_kind`]). A kind ABSENT here is reach-LESS
    /// (its footprints never expand) — never declared, or REFUSED for a cross-file duplicate.
    by_kind: BTreeMap<Symbol, (usize, Symbol)>,
}

impl KindReaches {
    /// The reach-bearing RAW kinds (the engine expands every footprint coord of such a kind).
    fn reach_kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.by_kind.keys().copied()
    }

    /// The `(file-index, reaches funcdef)` for a RAW coordinate kind, if it has a kept reach-function.
    fn get(&self, kind: Symbol) -> Option<(usize, &dorc_oracle::predict::Predict)> {
        let (idx, base) = *self.by_kind.get(&kind)?;
        self.sets.get(idx)?.get(base).map(|p| (idx, p))
    }
}

/// Lift the per-kind reach-functions + ENFORCE confusability (24G §4, kind-keyed like the resolver —
/// a LOUD diagnostic, never a silent dud). Two checks, mirroring [`build_kind_resolvers`]: (1)
/// at-most-one-reaches-per-kind — two files declaring `<kind>.reaches()` for the SAME kind ⇒ REFUSE
/// BOTH (the kind stays reach-less) + an error; (2) a reaches keyed to a name matching a known
/// PROVIDER ⇒ a WARNING (the reaches is keyed by KIND, not command). `inv-referent-agnostic`: names
/// compared as interned strings, never decoded.
fn build_kind_reaches(
    oracle_srcs: &[String],
    oracle_paths: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    coord_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
    advisory: bool,
) -> KindReaches {
    use dorc_oracle::reaches::ReachesSet;
    use dorc_oracle::to_funcname_segment;

    let sets: Vec<ReachesSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = ReachesSet::lift(interner, src);
            report_at(advisory, "reaches", None, &lifted.diags);
            lifted.value
        })
        .collect();

    let mut per_kind: BTreeMap<Symbol, Vec<usize>> = BTreeMap::new();
    for (idx, set) in sets.iter().enumerate() {
        for kind in set.kinds() {
            per_kind.entry(kind).or_default().push(idx);
        }
    }

    let mut providers: BTreeSet<String> = BTreeSet::new();
    for cs in checks {
        for p in cs.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }
    for (_, ts) in touches_paired {
        for p in ts.providers() {
            providers.insert(to_funcname_segment(interner.resolve(p)));
        }
    }

    let mut diags_by_file: BTreeMap<usize, Vec<Diag>> = BTreeMap::new();
    let mut base_to_idx: BTreeMap<Symbol, usize> = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        // Point at the FIRST declaring file's `<kind>__reaches` funcdef name (`aid-caret-span-precision`).
        let anchor = files
            .first()
            .and_then(|&idx| Some((idx, sets.get(idx)?.get(kind)?.name_span)));
        if files.len() > 1 {
            if let Some((idx, span)) = anchor {
                diags_by_file.entry(idx).or_default().push(Diag::new(
                    DiagCode::ReachesConflict(ReachesConflict {
                        kind: name.clone(),
                        count: files.len(),
                    }),
                    span,
                ));
            }
            continue;
        }
        if providers.contains(&name)
            && let Some((idx, span)) = anchor
        {
            diags_by_file.entry(idx).or_default().push(Diag::new(
                DiagCode::ReachesProviderCollision(ReachesProviderCollision { name: name.clone() }),
                span,
            ));
        }
        if let Some(&idx) = files.first() {
            base_to_idx.insert(kind, idx);
        }
    }
    report_by_oracle_file(
        advisory,
        "reaches",
        oracle_paths,
        oracle_srcs,
        &diags_by_file,
    );
    let by_kind = rekey_to_raw_kinds(&base_to_idx, coord_kinds, interner);
    KindReaches { sets, by_kind }
}

/// The per-arm wrapper funcname a dynamic `reaches()` arm ships and is invoked under. Engine-
/// synthesized scaffolding, so def and invocation are one string by construction; the ROLE part is
/// taken from the shared suffix constant so the emitted namespace tracks the role's real spelling
/// (`289:rul-touches-mismatch-own-lane` — the half-landed respell left `__reaches_<n>` behind).
fn reach_arm_fn_name(kind_name: &str, arm_index: usize) -> String {
    format!(
        "{}{}_{arm_index}",
        dorc_oracle::to_funcname_segment(kind_name),
        dorc_oracle::reaches::DISTURBANCE_REACHES_ONLY_SUFFIX,
    )
}

/// Compile the reach-probe (24G §4): for each reach-bearing AUTHORED footprint coordinate, ship each
/// DYNAMIC `reaches()` arm's per-arm wrapper ([`reach_arm_fn_name`]`() { <arm bytes> ; }` — the arm
/// command's byte-exact span-slice, mark-free by construction) invoked with the entity; its stdout is
/// the RAW ENTITIES it drags. STATIC arms never ship (traced at expansion). Deduped by (coord, arm).
/// Dynamic arms apply to AUTHORED footprint coords only this pass (derived coords resolved only
/// post-results — the `resid-kindfn-derived` deferral, 24G §3). `inv-referent-agnostic`: the entity
/// text is resolved for the invocation, never decoded.
#[expect(
    clippy::too_many_arguments,
    reason = "the reach-probe compile threads the compiled context (classes/kills/value/touches/reaches/reach-kinds/oracle-srcs/interner); each is a distinct pipeline output, not a bundle-able struct"
)]
fn collect_reach_probes(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    reaches: &KindReaches,
    reach_kinds: &BTreeSet<Symbol>,
    oracle_srcs: &[String],
    interner: &mut Interner,
) -> dorc_plan::ReachPlan {
    use dorc_analysis::effect::SkipClass;
    use dorc_oracle::reaches::{ArmOutcome, evaluate_reaches};
    let mut probes: BTreeMap<(String, usize), dorc_plan::ReachProbe> = BTreeMap::new();
    for (node, class) in classes {
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
        ) || kills.contains(node);
        if !is_wall_candidate {
            continue;
        }
        let Some((_, fp_coords, _)) =
            resolve_touches_footprint(*node, value, touches_sets, interner)
        else {
            continue;
        };
        for (coord, _selector) in fp_coords {
            let kind_sym = coord.kind().0;
            if !reach_kinds.contains(&kind_sym) {
                continue;
            }
            let Some((idx, reaches_fn)) = reaches.get(kind_sym) else {
                continue;
            };
            let Some(src) = oracle_srcs.get(idx) else {
                continue;
            };
            let entity_text = entity_text_of(coord, interner);
            let coord_label = render_coord(coord, interner);
            let kind_name = interner.resolve(kind_sym).to_owned();
            let exp = evaluate_reaches(reaches_fn, &entity_text);
            for arm in &exp.arms {
                let ArmOutcome::Dynamic { cmd_span } = &arm.outcome else {
                    continue; // a STATIC arm ships nothing (traced at expansion)
                };
                let bytes = src
                    .get(cmd_span.lo.0 as usize..cmd_span.hi.0 as usize)
                    .unwrap_or_default()
                    .trim();
                let arm_fn = reach_arm_fn_name(&kind_name, arm.index);
                let arm_sh = format!("{arm_fn}() {{ {bytes} ; }}");
                probes
                    .entry((coord_label.clone(), arm.index))
                    .or_insert(dorc_plan::ReachProbe {
                        coord_label: coord_label.clone(),
                        kind_label: kind_name.clone(),
                        arm_fn,
                        arm_index: arm.index,
                        entity_text: entity_text.clone(),
                        arm_sh,
                    });
            }
        }
    }
    dorc_plan::ReachPlan {
        probes: probes.into_values().collect(),
    }
}

/// Expand every reach-bearing footprint coordinate via its kind's `reaches()` (24G §4 — the
/// compositional half; the cross-author widening). STATIC arms apply to ALL footprint coords
/// (authored + derived), traced here at the cli (no host); DYNAMIC arms apply to AUTHORED coords only
/// this pass (their entities come from the `reach` readback — derived coords are known only
/// post-results, the `resid-kindfn-derived` deferral, 24G §3). Each expanded coord is unioned into
/// the footprint via [`dorc_plan::Footprint::add_reached`] (attributed to the reach-function KIND),
/// flowing through the EXISTING `disjoint`/canonicalization path. `inv-referent-agnostic`: the engine
/// interns the annotated kind (fixed at LIFT — the vocabulary fence) + the raw entities, never
/// decoding them. `inv-kfail`: widening only ever HITs MORE (demotes toward run), the safe direction.
fn expand_footprints_via_reaches(
    footprints: &mut dorc_plan::TrustedFootprints,
    reaches: &KindReaches,
    reach_kinds: &BTreeSet<Symbol>,
    readback: &SiteResults,
    interner: &mut Interner,
) {
    use dorc_oracle::reaches::{ArmOutcome, evaluate_reaches};
    footprints.expand_reaches(|coord, origin| {
        let kind_sym = coord.kind().0;
        if !reach_kinds.contains(&kind_sym) {
            return Vec::new();
        }
        let Some((_, reaches_fn)) = reaches.get(kind_sym) else {
            return Vec::new();
        };
        let entity_text = entity_text_of(coord, interner);
        let coord_label = render_coord(coord, interner);
        let via = coord.kind();
        let exp = evaluate_reaches(reaches_fn, &entity_text);
        let mut out = Vec::new();
        for arm in &exp.arms {
            let arm_kind = dorc_core::KindId(interner.intern(&arm.kind));
            let entities: Vec<String> = match &arm.outcome {
                // STATIC arms apply to ALL footprint coords (24G §3) — the traced lines, no host.
                ArmOutcome::Static(lines) => lines.clone(),
                // DYNAMIC arms apply to AUTHORED coords only this pass (24G §3, resid-kindfn-derived).
                ArmOutcome::Dynamic { .. } => {
                    if matches!(origin, dorc_plan::FootprintOrigin::Authored) {
                        readback
                            .reaches
                            .get(&(coord_label.clone(), arm.index))
                            .cloned()
                            .unwrap_or_default()
                    } else {
                        Vec::new()
                    }
                }
            };
            for e in entities {
                if e.is_empty() {
                    continue; // a blank reached entity is not a coordinate
                }
                let ec = dorc_plan::EntityCoord::new(
                    arm_kind,
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(&e))),
                );
                out.push((ec, via));
            }
        }
        out
    });
}

/// The entity text of a coordinate for a reach/resolver invocation (an operand's text, or the empty
/// string for a Singleton). `inv-referent-agnostic`: resolved for the invocation, never decoded.
fn entity_text_of(coord: dorc_plan::EntityCoord, interner: &Interner) -> String {
    match coord.entity() {
        dorc_core::EntityRef::Operand(tok) => interner.resolve(tok.0).to_owned(),
        dorc_core::EntityRef::Singleton => String::new(),
    }
}

/// Lift the per-site GUARD VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c).
/// Called ALWAYS-ON — guards are the un-flagged baseline (rul24-mode-gate governs only the survival
/// tier, NOT this). For each establish-bearing site whose provider authored a verdict function
/// (`<provider>.is_converged`) that REACHES a vouching path over the site's resolved
/// argv (`evaluate_verdict` ⇒ `Vouched`), build a [`dorc_plan::Vouches`] entry: a
/// `ByVouch<VerdictVouch>` carrying the guard emitter's data (the mangled funcname, the strip-only
/// preamble, the invocation, the declared sense, the fact's kind label), keyed by the site's
/// `CfgNodeId`. A `Declined` (unhandled path — hz-refusepath: a refuse path that returns 0
/// vacuously never vouches) or ⊤ (P-topargv: an unpropagatable argv) resolution, or no verdict
/// function, ⇒ absence from the map ⇒ the site never guards (no vouch ⇒ run — the judgment tier the
/// map carries is exactly what [`dorc_plan::GuardLicense::mint`] DEMANDS, TC-tier-2).
///
/// Verdict-lift diagnostics are surfaced AS-IS (inv-top-reject: under-modeling is a loud
/// correctness boundary, never a silent degrade). The Part-A `tc-verdict-return` softening
/// (⊤-reject → warning) is REVERTED (find-return-vouches, 24C): the tracer now models a reached
/// `return N` as a DECLINE and the corpus arity-refuse is spelled in-dialect
/// (`if [ … ]; then return N; fi`), so no corpus verdict body ⊤-rejects — one that still does is
/// genuinely out of dialect and SHOULD fail loudly. A verdict function that fails to lift yields
/// no vouch (the site runs, kFAIL-perform) regardless.
///
/// `inv-referent-agnostic`: the kind label + operands are resolved for the invocation/attribution,
/// never decoded for meaning; the vouch travels the site's own value-flow (the 24A §1b fence).
fn build_vouches(
    oracle_refs: &[&str],
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    value: &dorc_analysis::value::ValueFlow,
    interner: &mut Interner,
    advisory: bool,
) -> (dorc_plan::Vouches, Vec<CollapseNarrative>) {
    // The composition lives in `dorc_plan::build_vouches` (the ONE home — the sweep/coverage DSTs
    // share it). This edge only ROUTES the lift diagnostics: surfaced AS-IS (inv-top-reject — the
    // tc-verdict-return softening is reverted, find-return-vouches 24C), so a genuinely
    // out-of-dialect verdict body fails gate-3's error-floor rather than degrading silently.
    let (lifted, decline_narrative) =
        dorc_plan::build_vouches(oracle_refs, classes, value, interner);
    report_at(advisory, "verdict", None, &lifted.diags);
    (lifted.value, decline_narrative)
}

/// gate-5 / cm-2 readout: per command site, emit `argv <leafid> <disposition> <word|TOP
/// per word>` on stderr (a resolved literal verbatim, an unresolved word `TOP`). The
/// leaf-ids are the plan's own ([`dorc_plan::Step::leaf`]) — the same span-sorted space the
/// probe records share (`inv-site-keyed-results`), so `argv N` keys to the same site as
/// `site N`. The argv is the book-side value-flow
/// ([`dorc_analysis::value::ValueFlow::argv_values`]), keyed by `CfgNodeId` (mapped back
/// from the leaf's `AstId`). Cli-edge only.
///
/// The `<disposition>` tag (task-O / `tc-gate5-omit`, strain-D3b-fold-vs-gate5): one of
/// `run`/`replace`/`omit`, so gate-5 can SKIP a site the plan does not run. An `Omit`ted or
/// `Replace`d site legitimately never appears in the bare book's argv log when a preceding
/// guard short-circuits it (e.g. a shimmed Query-guard fold) — asserting it ⊆ the log would
/// be a false failure, the exact structural exclusion that confined the fold/omit
/// demonstration to builtin guards (20G §5). Filtering on `run` removes that exclusion
/// without weakening the gate for the sites that DO run.
fn emit_debug_argv(
    plan: &dorc_plan::Plan,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    interner: &Interner,
) {
    use dorc_analysis::value::ValueOf;
    // AstId → CfgNodeId for Command nodes (argv_values is keyed by CfgNodeId; the plan
    // step carries the AstId). One CfgNode per command AstId in the modeled subset.
    let node_of_ast: BTreeMap<dorc_core::AstId, dorc_analysis::cfg::CfgNodeId> = cfg
        .iter()
        .filter(|(_, n)| n.kind == dorc_analysis::cfg::CfgNodeKind::Command)
        .map(|(id, n)| (n.ast, id))
        .collect();
    for step in &plan.steps {
        let Some(&node) = node_of_ast.get(&step.ast) else {
            continue;
        };
        let words: Vec<String> = value
            .argv_values(node)
            .into_iter()
            .map(|w| match w {
                ValueOf::Literal(sym) => interner.resolve(sym).to_string(),
                ValueOf::Top(_) => "TOP".to_string(),
            })
            .collect();
        eprintln!(
            "argv {} {} {}",
            step.leaf.0,
            disposition_tag(&step.disposition),
            words.join(" ")
        );
    }
    // gate-6 `guardcmd` attribution (23A §5): one line per DISTINCT check-command a GUARDED site's
    // verdict body runs (`guardcmd dpkg-query`). The widened dual-rail judge allowlists these as
    // legitimate apply-only lines (the guard's live check runs at apply, absent from the bare
    // book) — never an unrelated one (cf-5). Deterministic (`BTreeSet`, `inv-determinism`).
    let mut guard_cmds: BTreeSet<&str> = BTreeSet::new();
    for step in &plan.steps {
        if let dorc_plan::Disposition::Guard(license) = &step.disposition {
            for c in license.insert().check_cmds() {
                guard_cmds.insert(c.as_str());
            }
        }
    }
    for c in &guard_cmds {
        eprintln!("guardcmd {c}");
    }
}

/// q-2 (`dq-site-unresolvable`) + cheap-7 (the firehose fix): ONE aggregated Note for the
/// probe-unresolvable sites that are worth disclosing — never the per-site stanza-per-site
/// firehose the recon flagged (a 5,000-line book emitted 50,002 stderr lines). Two moves:
///
/// 1. **SUPPRESS the structurally-unprobeable** ([`is_structurally_unprobeable`]): a bare
///    assignment (`pkg=nginx`), `set -eu`, or a pure/no-target-state builtin (`:`/`echo`/`cd`/…,
///    the ENGINE's own [`dorc_analysis::effect::is_target_state_pure_builtin`] list) has NO probe
///    that could ever exist — "declare a read-only probe for `set -eu`" is actively wrong advice —
///    so it earns NO disclosure at all.
/// 2. **AGGREGATE the remainder** into ONE honest Note naming every real command (`make install`,
///    an un-oracled `apt-get …`) and pointing at `dorc why` for the per-site detail. The frame
///    points at the FIRST real site as a representative (a caret example), constant-size regardless
///    of how many sites run unprobed. rul-attention-honesty is intact: the artifact still RUNS every
///    one (this only collapses the stderr readout — no run is hidden).
///
/// Reuses the migrated `DiagCode::SiteUnresolvable` spine (the sanctioned emit for this slug —
/// tidy-gate reachability unchanged). The `unresolvable` [`LeafId`]s share the apply plan's
/// span-sorted site space (`inv-site-keyed-results`), so each maps to a [`dorc_plan::Step`]'s `ast`,
/// whose span resolves to the book source. A site with no matching step is ASSERTED-UNREACHABLE
/// (human ruling 22-q2: `unresolvable ⊆ plan.steps` by construction) then skipped — `debug_assert`
/// loud in debug/DST, safe-degrade (skip) in release (never-vouch: the reachability claim is ours).
fn unresolvable_diagnostics(
    probe: &dorc_plan::ProbePlan,
    plan: &dorc_plan::Plan,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
) -> Vec<Diag> {
    use dorc_aid::diag::{SiteId, SiteUnresolvable};
    let ast_of_leaf: BTreeMap<dorc_plan::LeafId, dorc_core::AstId> =
        plan.steps.iter().map(|s| (s.leaf, s.ast)).collect();

    // The REAL (worth-disclosing) unresolvable sites, in the probe's site order.
    let mut real: Vec<(dorc_plan::LeafId, dorc_core::Span, String)> = Vec::new();
    for &leaf in &probe.unresolvable {
        let Some(&id) = ast_of_leaf.get(&leaf) else {
            debug_assert!(
                false,
                "unresolvable site has no plan step — unresolvable ⊆ plan.steps by \
                 construction (f-7); a hit means the probe/plan site spaces diverged"
            );
            continue;
        };
        let span = ast.node(id).span;
        let text = book_src
            .get(span.lo.0 as usize..span.hi.0 as usize)
            .unwrap_or("<source unavailable>");
        if is_structurally_unprobeable(text) {
            continue; // cheap-7: no probe could ever exist for an assignment / pure builtin
        }
        real.push((leaf, span, flatten_ws(text)));
    }

    let Some((first_leaf, first_span, _)) = real.first().cloned() else {
        return Vec::new(); // nothing worth disclosing (only inert sites, or none)
    };

    // The aggregate: name every real command (backtick-wrapped), point at `dorc why`. The frame's
    // caret lands on the first as a representative (its source_excerpt is that first command).
    let names: Vec<String> = real.iter().map(|(_, _, t)| format!("`{t}`")).collect();
    let plural = if real.len() == 1 { "" } else { "s" };
    let label = format!(
        "{} site{plural} run unprobed (no read-only check could be shipped): {} -- \
         run `dorc why` for the per-site detail (the apply runs each anyway, to stay safe)",
        real.len(),
        names.join(", "),
    );
    let first_text = book_src
        .get(first_span.lo.0 as usize..first_span.hi.0 as usize)
        .unwrap_or("<source unavailable>");
    // PASSTHROUGH `detail` reproduces BOTH the aggregate label AND the old render_body
    // `\n  = note: site runs `{excerpt}`` continuation, folded into one string so the migrated
    // render stays byte-identical bar the `sm ` prefix (`27V`, conductor-ruled shape).
    let detail = format!("{label}\n  = note: site runs `{first_text}`");
    vec![Diag::new(
        DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: SiteId::leaf(first_leaf),
            detail,
        }),
        first_span,
    )]
}

/// cheap-7: is this command source text a STRUCTURALLY-UNPROBEABLE site — one for which no
/// read-only probe could ever be authored, so the firehose disclosure would be actively-wrong
/// advice? Two shapes: a bare assignment (`NAME=value`, no command), and a pure/no-target-state
/// builtin (the engine's OWN [`dorc_analysis::effect::is_target_state_pure_builtin`] list — never a
/// parallel notion). Everything else (a real un-oracled command like `make install`) is a genuine
/// "runs unprobed" the admin should see aggregated.
fn is_structurally_unprobeable(cmd_text: &str) -> bool {
    let first = cmd_text.split_whitespace().next().unwrap_or("");
    // A bare assignment: `NAME=…` where NAME is a valid sh name (no command word to probe).
    if let Some((name, _)) = first.split_once('=')
        && !name.is_empty()
        && name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return true;
    }
    dorc_analysis::effect::is_target_state_pure_builtin(first)
}

/// Collapse interior whitespace runs (incl. newlines) to single spaces for a ONE-LINE disclosure
/// of a possibly multi-line command — the aggregate Note stays one line per rul-attention-honesty's
/// compactness (the artifact still carries the verbatim command).
fn flatten_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// plans/240 Stage-1 yardstick: emit the plan-summary — a one-line, greppable, stable-grammar
/// readout of the per-disposition tally (the round's north-star metric, elision frequency) — on
/// stderr, the render surface. rec-1 TWO SURFACES: NEVER woven into the byte-floored `.sh`
/// artifact on stdout. The cli emits it in every plan-building mode (`probe` returns before any
/// plan exists, so it emits none). Shaped `dorc: plan-summary …`, never `<stage>: error[…]`, so
/// the e2e gate-3 stderr floor (keyed on the `error[` shape) ignores it. Counts derive from the
/// Plan value alone (`inv-determinism`).
fn emit_plan_summary(plan: &dorc_plan::Plan) {
    let counts = plan.disposition_counts();
    // 24F §3a: the may-alias fire-rate — converged elisions demoted because a same-kind pair could
    // not be canonicalized (the resolver ⊤'d / dangled / was absent). Surfaced so a SWAMPED count is
    // a finding to REPORT (the resolver is too weak/broken), never a license to silently flip the
    // may-alias default. 0 when no resolver-bearing kind participates (the token-equality floor).
    eprintln!(
        "{}",
        chrome(
            "cli-plan-summary-line",
            &[
                &counts.sites.to_string(),
                &counts.elide.to_string(),
                &counts.omit.to_string(),
                &counts.guard.to_string(),
                &counts.run.to_string(),
                &plan.survival_report.may_alias_fires().to_string(),
            ],
        )
    );
}

/// stage-3 (the why-lens render, `22D` §1): surface — on stderr, the RENDER surface — the
/// per-line "why did this command RUN (never elided)?" disclosure for each forced-run command
/// whose ⊤ carries a wired cause. The render + stage-4 dedup is [`why_lens_lines`] (pure,
/// unit-testable); this is just its stderr driver.
///
/// rec-1 WELD (two surfaces): this prints to STDERR only — the plan-render surface. It is NEVER
/// woven into the byte-floored `.sh` artifact on stdout (the artifact stays receipt-free). The
/// line is prefixed `why:` and never `error[`, so the e2e gate-3 stderr-floor (which keys on the
/// `<stage>: error[` shape) ignores it — the why-lens is additive, never a case-failing diagnostic.
///
/// `_collapse_narrative` is the C3/C4 decision-inert narrative seam (`27V` Lane A): the collapse
/// records the why-lens will render (d4). Carried through here and IGNORED for now — the render
/// arrangement is d4's, so surfacing it early would freeze `render-form-unwelded` output.
fn emit_why_lens(
    why_diags: &[Diag],
    arena: &ProvArena,
    src: &str,
    _collapse_narrative: &[CollapseNarrative],
) {
    for line in why_lens_lines(why_diags, arena, src) {
        eprintln!("why: {line}");
    }
}

/// Stage 2 attribution (TC-3 / rul24-divergence-is-the-game): emit, on the why-lens stderr
/// lane (the `why: ` prefix, alongside the run-cause disclosures — one lens, two directions:
/// why-a-line-runs and why-a-line-survived), one line per SURVIVED elision — naming the
/// surviving site, each running wall it crossed, whose footprint licensed the crossing (the
/// provider and its claimed coordinates), and the backing coordinate proven disjoint. Reads the
/// [`dorc_plan::SurvivalWitness`] the wall walk minted — NEVER recomputes disjointness (the
/// witness IS the attribution). rec-1 WELD: stderr render surface only; the byte-floored `.sh`
/// artifact stays receipt-free (a survived elision's artifact bytes are identical to any other
/// elision's). Never `error[`, so the gate-3 stderr floor ignores it; the `why: ` prefix lets
/// gate-7 (`expected-why`) pin the attribution end-to-end.
fn emit_survival_attribution(
    plan: &dorc_plan::Plan,
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) {
    for step in &plan.steps {
        let dorc_plan::Disposition::Replace(license, _) = &step.disposition else {
            continue;
        };
        let Some(witness) = &license.derivation().survival else {
            continue;
        };
        let crossings: Vec<String> = witness
            .crossings()
            .iter()
            .map(|c| {
                let provider = interner.resolve(c.provider());
                let coords: Vec<String> = c
                    .footprint()
                    .iter()
                    .map(|fc| render_coord(*fc, interner))
                    .collect();
                // 24E §9: name a host-DERIVED footprint's provenance ("DERIVED at probe from
                // <call>"); an authored (static) footprint carries no extra locus.
                let origin = match c.origin() {
                    dorc_plan::FootprintOrigin::Derived { call } => {
                        format!("; DERIVED at probe from {call}")
                    }
                    dorc_plan::FootprintOrigin::Authored => String::new(),
                };
                // 24F §6: name the resolver that canonicalized this crossing's coords ("disjoint
                // AFTER <kind>.resolve()"). The aliasing closure is the sharpest claim in the design,
                // so a survival it licensed must always name whose identity-judgment it trusted.
                let via = c.via_resolver().map_or_else(String::new, |k| {
                    format!(
                        "; disjoint AFTER {}.resolve() canonicalization",
                        interner.resolve(k.0)
                    )
                });
                // 24G §8: name the engine-supplied OWN-effect coordinate distinctly — present only
                // when the union WIDENED the footprint (the derived lane; the authored lane's canary
                // folds own into the `touches()` claim, so it is not repeated). Provenance: the site's
                // declared effect, NOT the author's claim.
                let own = c.own().map_or_else(String::new, |o| {
                    format!("; own-effect {}", render_coord(o, interner))
                });
                format!(
                    "wall site {} ({provider} touches {{{}}}{own}{origin}{via})",
                    c.wall_leaf().0,
                    coords.join(" ")
                )
            })
            .collect();
        let aggregate_loci: Vec<String> = license
            .derivation()
            .establish_vouches
            .iter()
            .map(|receipt| {
                let locus = oracle_locus(receipt.defining_span, oracle_paths, oracle_srcs)
                    .map(|value| format!(" at {value}"))
                    .unwrap_or_default();
                format!(
                    "site {} {} vouched{locus}",
                    receipt.site.0,
                    dorc_plan::fact_label(interner, receipt.fact)
                )
            })
            .collect();
        let locus = if aggregate_loci.is_empty() {
            oracle_locus(license.derivation().vouch_span, oracle_paths, oracle_srcs)
                .map(|value| format!("; vouched at {value}"))
                .unwrap_or_default()
        } else {
            format!("; {}", aggregate_loci.join(", "))
        };
        eprintln!(
            "why: site {} survives+elides past {} -- backing {} disjoint (trusted footprint){locus}",
            step.leaf.0,
            crossings.join(", "),
            render_coord(witness.backing(), interner),
        );
    }
}

/// The REACH-POISON why-lane (24G Part B): one `why:` line per converged elision that DEMOTED to run
/// because a `<kind>.reaches()` EXPANSION coordinate hit its backing — the cross-author demote the
/// reach mechanism exists for. Mirrors the resolver-attribution shape (the sharpest claims name whose
/// knowledge they trusted): here the demote names the reach-function whose widening caught the
/// otherwise-wrongly-surviving elision. rec-1 WELD: stderr render surface only. Never `error[`, so the
/// gate-3 floor ignores it; the `why: ` prefix lets the render surface pin it.
fn emit_reach_poisonings(plan: &dorc_plan::Plan, interner: &Interner) {
    for (leaf, kind) in plan.survival_report.reach_poisonings() {
        eprintln!(
            "why: site {} runs -- poisoned via {}.reaches() (a reach-expanded coordinate hit its \
             backing; the wall drags it cross-author)",
            leaf.0,
            interner.resolve(kind.0),
        );
    }
}

/// The GUARD why-lane (rul-guard-license / X-why): one `why:` line per guarded site, naming
/// (i) the mechanism (`guard`), (ii) the license (a converged-`vouch`), (iii) the vouching oracle
/// (the fact's kind) — the `guard23-why-attribution` conjoined pattern (`guard && vouch && <kind>`
/// in ONE line). Attribution is the guard-license's whole enforcement story ("we can't prevent, so
/// we attribute" — plans/233 §guard-license); rul-attention-honesty makes it load-bearing (a guard
/// the user can't trace to its licensor is hidden risk). rec-1 WELD: stderr render surface only —
/// the byte-floored artifact carries the inline `# dorc: guard …` comment; this is the disclosure.
/// Never `error[`, so the gate-3 floor ignores it; the `why: ` prefix lets gate-7 pin it.
fn emit_guard_attribution(
    plan: &dorc_plan::Plan,
    ast: &dorc_syntax::ast::Ast,
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) {
    // A render-REFUSED guard (heredoc / non-devnull output redirect) does NOT guard the site — the
    // mutator runs verbatim. rul-attention-honesty: never claim a skip that did not happen; disclose
    // the refusal (gate-7 `refus`) instead of the licensing line.
    let refused = plan.guard_refused_asts(ast);
    for step in &plan.steps {
        let dorc_plan::Disposition::Guard(license) = &step.disposition else {
            continue;
        };
        let kind = interner.resolve(license.fact().kind.0);
        if refused.contains(&step.ast) {
            eprintln!(
                "why: site {} guard refused -- the site's structurally-awkward form (a heredoc \
                 body, or a non-`/dev/null` output redirect) would corrupt the artifact or suppress \
                 an admin-spelled side-effect, so the original bytes RUN VERBATIM (to stay safe), \
                 the {kind} oracle's vouch that it is already satisfied notwithstanding",
                step.leaf.0,
            );
        } else {
            // C7: the vouch's defining `file:line` (the reached check arm), when the plan threaded it.
            let locus = oracle_locus(license.insert().defining_span(), oracle_paths, oracle_srcs)
                .map(|l| format!(" (at {l})"))
                .unwrap_or_default();
            eprintln!(
                "why: site {} guard [{kind}] -- licensed by the {kind} oracle's vouch{locus} that \
                 it is already satisfied; the original bytes survive and the check re-runs live at \
                 apply (to stay safe)",
                step.leaf.0,
            );
        }
    }
}

/// Resolve a threaded oracle `(Span, OracleFileId)` to a `path:line` locus (C7 `file:line`;
/// `law-lineno-identity` — the file id disambiguates WHICH oracle's line-number space, since a
/// bare span is file-ambiguous once >1 oracle is loaded). `None` when the vouch/claim was
/// unthreaded, or the id is out of range (the render omits the locus — never fabricates one).
fn oracle_locus(
    defining: Option<(dorc_core::Span, dorc_core::OracleFileId)>,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> Option<String> {
    let (span, file) = defining?;
    let i = file.0 as usize;
    let (path, src) = (oracle_paths.get(i)?, oracle_srcs.get(i)?);
    let (line, _col) = dorc_aid::diag::line_col(src, span.lo.0 as usize);
    Some(format!("{path}:{line}"))
}

/// A speaker's own source, inlined beneath their row: the file it came from, the numbered lines,
/// and whether a middle was cut out of them.
struct Excerpt {
    path: String,
    head: Vec<(usize, String)>,
    /// The retained tail, when a middle was cut. Empty when the excerpt is contiguous.
    tail: Vec<(usize, String)>,
    /// How many lines the cut dropped. Zero when the excerpt is contiguous.
    elided: usize,
}

/// The most lines of an author's arm the surface will inline before cutting a middle out of it.
const EXCERPT_LINES: usize = 8;
/// The most preceding comment lines attached to an arm.
const EXCERPT_COMMENT_LINES: usize = 4;

/// Slice an oracle's own source around a threaded span, for display beneath the row that quotes it.
///
/// The massaging is licensed and bounded (`27W:rul-report-surface-massaging`): the CONTRIBUTING
/// lines are the span's own, the author's ADJACENT comment block is attached because a comment
/// above an arm is the author explaining that arm, and a long middle is CUT with the cut shown.
/// Authorship-implying and repair-directing, never byte-obligated — and never runnable, which is
/// why the render marks any cut rather than quietly closing over it.
///
/// `None` when the span was unthreaded or its file is out of range: an absent excerpt is an
/// omission, never a fabrication.
fn oracle_excerpt(
    defining: Option<(dorc_core::Span, dorc_core::OracleFileId)>,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> Option<Excerpt> {
    let (span, file) = defining?;
    let index = file.0 as usize;
    let (path, src) = (oracle_paths.get(index)?, oracle_srcs.get(index)?);
    let source: Vec<&str> = src.lines().collect();
    // A span ending at end-of-file resolves PAST the last line, so both ends clamp.
    let first = dorc_aid::diag::line_col(src, span.lo.0 as usize)
        .0
        .min(source.len());
    let last = dorc_aid::diag::line_col(src, span.hi.0 as usize)
        .0
        .min(source.len())
        .max(first);

    let mut start = first;
    while start > 1
        && first.saturating_sub(start) < EXCERPT_COMMENT_LINES
        && source
            .get(start.saturating_sub(2))
            .is_some_and(|line| line.trim_start().starts_with('#'))
    {
        start = start.saturating_sub(1);
    }

    let numbered = |line: usize| {
        source
            .get(line.saturating_sub(1))
            .map(|text| (line, (*text).to_owned()))
    };
    let shown = last.saturating_sub(start).saturating_add(1);
    if shown <= EXCERPT_LINES {
        return Some(Excerpt {
            path: path.clone(),
            head: (start..=last).filter_map(numbered).collect(),
            tail: Vec::new(),
            elided: 0,
        });
    }
    // Keep the head and the last line, and say how much was dropped between them.
    let head_end = start.saturating_add(EXCERPT_LINES).saturating_sub(2);
    Some(Excerpt {
        path: path.clone(),
        head: (start..=head_end).filter_map(numbered).collect(),
        tail: std::iter::once(last).filter_map(numbered).collect(),
        elided: last.saturating_sub(head_end).saturating_sub(1),
    })
}

/// Every pure-predicate-CARRY elision names, on the why-lens lane, its cross-context attribution
/// chain (`27C` §9 / steering `pure-predicate-carry`): the crossed substrate axes, each backing
/// kind's owner `invariant:<axis>` line (vouch-species), and the engine read-set-closure proof. The
/// block acceptance demands this "from day one" — an UNFLAGGED cross-boundary answer resting on a
/// kind-owner's typed line + an engine proof MUST disclose whose line and what proof licensed it.
/// `carried` is keyed by the site's `AstId` (built at the carry decision); this re-keys to the plan's
/// per-site number. Empty when no site carried. Deterministic (plan step order).
fn emit_carry_attribution(plan: &dorc_plan::Plan, carried: &BTreeMap<dorc_core::AstId, String>) {
    for step in &plan.steps {
        if let Some(text) = carried.get(&step.ast) {
            eprintln!("why: site {} {text}", step.leaf.0);
        }
    }
}

/// upcoming-firstwall-hint (`USER_STORY` stage 3): the role a plan step plays in the poison-wall
/// walk, reduced for the first-wall hint. The wall the hint TARGETS is specifically an UNMODELED
/// (opaque) running command — the class an oracle could describe; a modeled-but-diverged wall is
/// honest and never the hint's subject ("the hint's whole point is 'an oracle would help HERE'").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallRole {
    /// A running UNMODELED (opaque) command — the poison wall. Its run ⊤-poisons every downstream
    /// fact in `classify` (⇒ `EstablishWritten` ⇒ guard-or-run), so no downstream elision survives
    /// it. Detected at the cli edge as a probe-UNRESOLVABLE real command — the same
    /// `probe.unresolvable` ∩ not-[`is_structurally_unprobeable`] set the firehose already discloses.
    Opaque,
    /// A running MODELED mutator (a diverged establish) or a kill — an HONEST wall. It BOUNDS the
    /// un-wall count (a downstream guard past it is walled by IT, not by the opaque wall), but is
    /// never the hint's subject: it is already described by an oracle.
    Honest,
    /// A converged-but-walled GUARDED site — it would upgrade guard→elide if the wall above it
    /// lifted ("an elided command casts no wall"). The un-wall count tallies exactly these, in the
    /// first opaque wall's own window.
    Guard,
    /// Transparent to the walk — an elision, an omit, or an inert running builtin: neither a wall
    /// nor an improvable guard.
    Transparent,
}

impl WallRole {
    /// The occurrence a wall row's payload is keyed by, so an UNDESCRIBED wall and a described
    /// running mutator never wear each other's words: one may touch anything because nobody said
    /// otherwise, the other has an author who said exactly what it touches.
    const fn occurrence(self) -> usize {
        match self {
            WallRole::Opaque => 0,
            WallRole::Honest | WallRole::Guard | WallRole::Transparent => 1,
        }
    }
}

/// One plan step reduced to (leaf, source line, command word, wall role) for [`first_wall_hint`].
/// `line` is the SOURCE line (rul24-lineno-identity); `word` is the command's first word — the
/// `'hork'` the hint names.
struct WallStep {
    leaf: dorc_plan::LeafId,
    line: usize,
    word: String,
    role: WallRole,
}

/// The first-wall hint payload (upcoming-firstwall-hint / `USER_STORY` stage 3): the FIRST opaque
/// wall in book order and the counterfactual un-wall count.
struct FirstWallHint {
    /// The wall site's leaf — the `dorc why` detail attaches to exactly this site.
    leaf: dorc_plan::LeafId,
    /// The wall's SOURCE line (rul24-lineno-identity: queryable as `dorc why book.sh:line`).
    line: usize,
    /// The wall command's first word (`'hork'`).
    word: String,
    /// `M` — the un-wall count: the number of converged-but-walled GUARD sites strictly between
    /// this wall and the next wall (opaque or honest). These are the sites that would upgrade
    /// guard→elide if this wall's command were modeled-and-converged ("an elided command casts no
    /// wall").
    ///
    /// CONSERVATIVE APPROXIMATION (flagged — the honest counterfactual is NOT a plan-level re-fold):
    /// an opaque wall's poison is applied in `classify` (⊤-reach ⇒ downstream `EstablishWritten`),
    /// NOT in the plan wall-walk, so "re-fold the plan with this wall treated as non-walling" cannot
    /// un-poison — the honest count needs a re-CLASSIFY with the command's effect forced Pure. This
    /// tally instead counts the walled guards in the wall's own window. It is EXACT in the common
    /// case and OVER-counts only when a downstream guard is `EstablishWritten` from a same-cell
    /// in-script write rather than from this wall (the `install X; hork; install X` shape), where
    /// lifting this wall alone would not recover it. Erring high on an advisory nag is acceptable.
    unwall: usize,
    /// Other opaque walls after the first — the trailing "N more unmodeled walls" pointer count.
    more_walls: usize,
}

impl FirstWallHint {
    /// The hint body (no `hint: ` prefix — the caller adds it, matching the `why:`/`dorc:` lanes).
    /// `USER_STORY` stage-3 register; plain English (24H ack-4 — no ⊤, no jargon; "unmodeled" is
    /// established vocabulary).
    fn body(&self) -> String {
        let unwall_clause = if self.unwall == 0 {
            String::new()
        } else {
            let sites = if self.unwall == 1 { "site" } else { "sites" };
            format!(", and un-wall {} downstream {sites}", self.unwall)
        };
        let more_clause = if self.more_walls == 0 {
            String::new()
        } else {
            let walls = if self.more_walls == 1 {
                "wall"
            } else {
                "walls"
            };
            format!("; {} more unmodeled {walls} -- dorc why", self.more_walls)
        };
        format!(
            "'{}' (line {}) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged{unwall_clause}{more_clause}",
            self.word, self.line
        )
    }

    /// The `dorc why` detail row for the wall's own site (the reasoning behind the plan-mode nag).
    /// Registry-homed like every other why-surface string (`28G` §0), and stated in admin-English:
    /// the engine's `elide` never reaches a render.
    fn why_detail(&self) -> String {
        let recovery = if self.unwall == 0 {
            String::new()
        } else {
            why_words("why-reason-first-wall-unwall", &[&self.unwall.to_string()])
        };
        why_words("why-reason-first-wall", &[&recovery])
    }
}

/// upcoming-firstwall-hint: the PURE first-wall computation. Find the first [`WallRole::Opaque`]
/// step in book order; tally the [`WallRole::Guard`] steps between it and the next wall (opaque or
/// honest) as the un-wall count; count the remaining opaque walls for the trailing pointer.
/// `None` ⇒ no opaque wall ⇒ no hint. Pure + total (`inv-determinism`); unit-tested over
/// hand-built scenarios.
fn first_wall_hint(steps: &[WallStep]) -> Option<FirstWallHint> {
    let w1 = steps.iter().position(|s| s.role == WallRole::Opaque)?;
    let wall = steps.get(w1)?;
    let after = steps.get(w1.saturating_add(1)..).unwrap_or(&[]);
    let mut unwall: usize = 0;
    for s in after {
        match s.role {
            WallRole::Guard => unwall = unwall.saturating_add(1),
            WallRole::Opaque | WallRole::Honest => break,
            WallRole::Transparent => {}
        }
    }
    let more_walls = after.iter().filter(|s| s.role == WallRole::Opaque).count();
    Some(FirstWallHint {
        leaf: wall.leaf,
        line: wall.line,
        word: wall.word.clone(),
        unwall,
        more_walls,
    })
}

/// Reduce each plan step to its [`WallStep`] (role + source line + command word) — the input
/// [`first_wall_hint`] consumes. The role classification is the load-bearing bit:
/// * a `Guard` disposition ⇒ [`WallRole::Guard`];
/// * a `Run` that is a KILL (`kills`) or a modeled establish ([`is_establish_bearing`]) ⇒
///   [`WallRole::Honest`] (a running mutator — it bounds the count);
/// * a `Run` that is probe-UNRESOLVABLE and a real command (not [`is_structurally_unprobeable`]) ⇒
///   [`WallRole::Opaque`] (the unmodeled poison wall — the same set the firehose discloses);
/// * everything else (elide / omit / inert builtin run) ⇒ [`WallRole::Transparent`].
///
/// The kill check PRECEDES the opaque check so a modeled kill is never mistaken for an unmodeled
/// wall. `by_ast` maps a step's `AstId` back to its `(CfgNodeId, SkipClass)` (steps ⊆ classified
/// leaves by construction; an unexpected miss degrades to `Transparent`, the safe non-claim).
fn collect_wall_steps(
    plan: &dorc_plan::Plan,
    probe: &dorc_plan::ProbePlan,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    cfg: &dorc_analysis::cfg::Cfg,
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
) -> Vec<WallStep> {
    let by_ast: BTreeMap<
        dorc_core::AstId,
        (
            dorc_analysis::cfg::CfgNodeId,
            &dorc_analysis::effect::SkipClass,
        ),
    > = classes
        .iter()
        .map(|(node, class)| (cfg.node(*node).ast, (*node, class)))
        .collect();
    plan.steps
        .iter()
        .map(|step| {
            let span = ast.node(step.ast).span;
            let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
            let line = dorc_aid::diag::line_col(book_src, lo).0;
            let text = book_src.get(lo..hi).unwrap_or("");
            let word = text.split_whitespace().next().unwrap_or("").to_owned();
            let role = match &step.disposition {
                dorc_plan::Disposition::Guard(_) => WallRole::Guard,
                dorc_plan::Disposition::Replace(..) | dorc_plan::Disposition::Omit { .. } => {
                    WallRole::Transparent
                }
                dorc_plan::Disposition::Run => {
                    let cls = by_ast.get(&step.ast);
                    if cls.is_some_and(|(node, _)| kills.contains(node)) {
                        WallRole::Honest
                    } else if probe.unresolvable.contains(&step.leaf)
                        && !is_structurally_unprobeable(text)
                    {
                        WallRole::Opaque
                    } else if cls.is_some_and(|(_, class)| is_establish_bearing(class)) {
                        WallRole::Honest
                    } else {
                        WallRole::Transparent
                    }
                }
            };
            WallStep {
                leaf: step.leaf,
                line,
                word,
                role,
            }
        })
        .collect()
}

/// Mirror of the plan crate's private `class_is_establish_bearing` (a running establish is a
/// mutator wall). Re-derived here rather than exported: a small, stable predicate, and the cli edge
/// already reaches into `SkipClass` variants for other readouts. Kept in step by the shared slug.
fn is_establish_bearing(class: &dorc_analysis::effect::SkipClass) -> bool {
    use dorc_analysis::effect::SkipClass as Sc;
    match class {
        Sc::EstablishAmbient(_) | Sc::EstablishWritten(_) | Sc::EstablishMembers { .. } => true,
        Sc::InlineCall { sites } => sites.iter().any(|s| {
            matches!(
                s.class,
                Sc::EstablishAmbient(_) | Sc::EstablishWritten(_) | Sc::EstablishMembers { .. }
            )
        }),
        Sc::QueryResolvable { .. } | Sc::MustRun => false,
    }
}

/// The ONE seat that renders a [`SpeechAct`] to a word (`law-trust-tier-is-syntax`;
/// `27V:mech-trust-tier-typed`): the chain walker below is the ONLY code that turns a typed tier into
/// prose, so a `claims` link can never wear a `reported`'s clothes (mis-attribution is the worst aid
/// failure — `271:rul-sin-ordering`).
///
/// The words are arrangement-registry rows keyed by the tier's ordinal, never literals (`28G` §0):
/// the tier SET is the law, the words ride `27V:rul-output-form-unwelded`. `28E` §8 fixes the
/// grammar they must obey — the tier word is the sentence's VERB, past tense for run events
/// (`reported`, `ran`) and present for standing text (`vouches`, `claims`, `derives`).
fn verb_word(tier: SpeechAct) -> String {
    let occurrence = match tier {
        SpeechAct::Measured => 0,
        SpeechAct::Vouched => 1,
        SpeechAct::Ran => 2,
        SpeechAct::Claimed => 3,
        SpeechAct::Derived => 4,
        SpeechAct::Consented => 5,
        SpeechAct::Declined => 6,
    };
    dorc_aid::arrangement::arrangement_text(
        &dorc_aid::arrangement::CONST_ARRANGEMENTS,
        "why-tier-word",
        Some(occurrence),
    )
}

/// What happened to a line, in the ADMIN's terms rather than the engine's — the typed twin of
/// [`outcome_word`], so counting and comparing never go through rendered prose.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OutcomeKind {
    Skipped,
    Guarded,
    Ran,
    Dropped,
}

impl OutcomeKind {
    /// The disposition, re-read in admin terms. The engine's own vocabulary stops here.
    fn of(disposition: &dorc_plan::Disposition) -> Self {
        match disposition {
            dorc_plan::Disposition::Replace(..) => OutcomeKind::Skipped,
            dorc_plan::Disposition::Guard(_) => OutcomeKind::Guarded,
            dorc_plan::Disposition::Run => OutcomeKind::Ran,
            dorc_plan::Disposition::Omit { .. } => OutcomeKind::Dropped,
        }
    }

    /// The other thing that could have happened to the line — what the contrastive OUTCOME sentence
    /// answers against (`28E` §7 adopt-contrastive-first: the foil is the line's other disposition,
    /// and it is free).
    const fn foil(self) -> Self {
        match self {
            OutcomeKind::Skipped | OutcomeKind::Ran => OutcomeKind::Guarded,
            OutcomeKind::Guarded | OutcomeKind::Dropped => OutcomeKind::Skipped,
        }
    }

    /// The admin-English word (`28E` §8, human-demonstrated). Registry-homed by ordinal like
    /// [`verb_word`]. The `skip`-ban is LLM-facing law over design and code layers
    /// (`271:rul-skip-ban-is-llm-facing`); this is the deliberate user-surface carve, and engine
    /// vocabulary (elide / replace / omit) never appears in a render.
    fn word(self) -> String {
        let occurrence = match self {
            OutcomeKind::Skipped => 0,
            OutcomeKind::Guarded => 1,
            OutcomeKind::Ran => 2,
            OutcomeKind::Dropped => 3,
        };
        dorc_aid::arrangement::arrangement_text(
            &dorc_aid::arrangement::CONST_ARRANGEMENTS,
            "why-outcome-word",
            Some(occurrence),
        )
    }
}

/// The admin-English disposition word for a plan step.
fn outcome_word(disposition: &dorc_plan::Disposition) -> String {
    OutcomeKind::of(disposition).word()
}

/// The word for a disposition's FOIL — a skip's is the guard it would otherwise have worn, a
/// guard's is the skip it could not earn, a run's is a guard.
fn foil_word(disposition: &dorc_plan::Disposition) -> String {
    OutcomeKind::of(disposition).foil().word()
}

/// One registry-sourced why-surface line, values interleaved between the entry's words. The
/// why-render twin of [`chrome`] (`289:rul-arrangement-home-is-registry-plus-transcripts`); every
/// user-facing string the triptych prints comes through here or through [`verb_word`] /
/// [`outcome_word`], never from a `format!` literal (`28G` §0).
fn why_words(slug: &str, values: &[&str]) -> String {
    why_words_at(slug, None, values)
}

/// [`why_words`] for a registry row whose words are keyed by occurrence.
///
/// This is the ONE seat that interleaves a computed value into a registry line, and therefore the
/// one place a value carrying bytes we did not write can enter our own words. The registry words
/// are never encoded — they are ours, and encoding them twice would be a defect — while every
/// value passes the display seat first (`sinv-sink-encoding`). A chrome line renders as ONE span
/// (`a-chrome-line-is-one-span`), so the value cannot carry its own foreign-text span here and
/// must instead arrive already safe.
fn why_words_at(slug: &str, occurrence: Option<usize>, values: &[&str]) -> String {
    let encoded: Vec<String> = values
        .iter()
        .map(|value| dorc_aid::display::encode_foreign(value, WHY_VALUE_CAP))
        .collect();
    let borrowed: Vec<&str> = encoded.iter().map(String::as_str).collect();
    dorc_aid::arrangement::arrangement_sentence(
        &dorc_aid::arrangement::CONST_ARRANGEMENTS,
        slug,
        occurrence,
        &borrowed,
    )
}

/// The display budget for one computed value on the why surface: a coordinate, an address, a
/// speaker, a `N|command` reference. Generous enough that nothing the corpus produces is touched,
/// bounded so a pathological book word cannot own the whole render.
const WHY_VALUE_CAP: usize = 240;

/// The display budget for one quoted line of somebody else's source. Wider than a value's because
/// a wrapped-off source line is a worse lie than a long one, and still bounded.
const WHY_SOURCE_CAP: usize = 512;

/// The gutter glyph a chain row wears in the DEFAULT render (`28E` §7
/// adapt-two-rank-default-render, sharpened by §8 `rul-danger-axis-is-completion-class`). ASCII
/// forever (`28E` §0 `rul-ascii-output-forever`, human-typed). The rank itself is the ordered
/// [`Knowability`] projection over the seven [`SpeechAct`] kinds, minted at the ONE derivation seat
/// (`SpeechAct::knowability`, `28F:rul-speechact-rename`) — this function only picks the glyph a
/// `Knowability` already decided, never re-derives one.
const fn rank_glyph(rank: Knowability) -> &'static str {
    match rank {
        Knowability::Witnessed => "*",
        Knowability::CoversUnmeasured => "!",
    }
}

/// One quoted-speakers row of a `dorc why <addr>` ANALYSIS panel (`28E` §8 quoted-speakers, ADOPTED):
/// speaker first, the tier word as the sentence's verb, the payload as the speaker's own quoted
/// words. Dorc asserts no world-fact in its own voice — it QUOTES speakers, and vouches only for the
/// run record and for its own derivations (which is why an engine row's payload is unquoted).
struct ChainLink {
    tier: SpeechAct,
    /// Who is speaking: an oracle `file:line`, a book site's `N|command`, or the engine. `None` when
    /// the model does not carry a locus for this speaker (rendered as an empty column, never faked).
    speaker: Option<String>,
    /// The payload's own words, and the registry row they came from — so the render can stamp the
    /// span with the entry an edit would rewrite rather than with the seat that assembled it.
    payload: Said,
    /// Whether the payload is the speaker's own words (quoted) or dorc's narration of them (bare).
    quoted: bool,
    /// Metadata about the SPEAKING rather than the thing said — when a check ran and what it
    /// exited with (`28G` strawman `a-fire-morning`'s `(ran 01:59:52, rc 0)`). It renders OUTSIDE
    /// the quotation, because attributing the circumstances to the speaker puts words in their
    /// mouth. `None` throughout today: the run clock and the stored rcs are the narration lane's,
    /// and a fabricated timestamp is worse than an absent one.
    event: Option<Said>,
    /// The indented paragraph carried below the quote — today only the at-most claim's
    /// covers-unmeasured disclosure.
    explanation: Option<Said>,
    /// The speaker's own source, inlined beneath the explanation: the arm plus the author's
    /// adjacent comment (`27W:rul-report-surface-massaging`). Not our bytes.
    excerpt: Option<Excerpt>,
}

/// A rendered fragment of the why surface, and where its bytes came from.
///
/// Carrying the origin past composition is what keeps `28G` §0 honest: the bytes reach weft already
/// interleaved (`a-chrome-line-is-one-span`), so without this the span map would name the seat that
/// assembled a line rather than the entry an edit has to rewrite. It also keeps the two classes
/// apart — registry words are rephrasable, a computed value is not, and rewriting one would be
/// lying about the world.
#[derive(Clone)]
enum Said {
    /// One registry-sourced line, with the arrangement slug it was composed from.
    Words(&'static str, String),
    /// A value the engine computed: a coordinate, an address, a count.
    Value(String),
    /// Prose the why-lens flattened to a string before this seat could see its parts — the standing
    /// `289:seam-whylens-render-seat`. It is not editable and cannot yet name the row it came from;
    /// giving it a real seat is `28G` Phase W4's.
    Lens(String),
}

impl Said {
    /// One registry line, its values interleaved.
    fn words(slug: &'static str, values: &[&str]) -> Self {
        Said::Words(slug, why_words(slug, values))
    }

    fn text(&self) -> &str {
        match self {
            Said::Words(_, text) | Said::Value(text) | Said::Lens(text) => text,
        }
    }

    /// The fragment as an attributed run. `part` names the seat for anything with no registry
    /// entry of its own to point at.
    fn run(&self, part: &'static str) -> Run<Face> {
        match self {
            Said::Words(slug, text) => dorc_aid::weave::words(text.clone(), slug),
            Said::Value(text) => dorc_aid::weave::value(text, part, "value", WHY_VALUE_CAP),
            Said::Lens(text) => dorc_aid::weave::value(text, "why-lens", "reason", WHY_SOURCE_CAP),
        }
    }
}

/// The label a NEXT STEPS row wears. Registry-homed by ordinal like [`verb_word`] — the label SET
/// is the structure, the words ride `27V:rul-output-form-unwelded`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum StepLabel {
    Suspect,
    Fix,
    Verify,
    Review,
    /// The improvements-not-repairs step: what to DESCRIBE so a guarded line can eventually skip
    /// (`28G` strawmen `b-wide-guarded` and `d-guard-fell-through` — a healthy guard has no repair,
    /// so its arc is a different verb).
    Describe,
}

impl StepLabel {
    fn word(self) -> String {
        let occurrence = match self {
            StepLabel::Suspect => 0,
            StepLabel::Fix => 1,
            StepLabel::Verify => 2,
            StepLabel::Review => 3,
            StepLabel::Describe => 4,
        };
        dorc_aid::arrangement::arrangement_text(
            &dorc_aid::arrangement::CONST_ARRANGEMENTS,
            "why-next-step-label",
            Some(occurrence),
        )
    }
}

/// One row of the remediation arc.
struct StepRow {
    label: StepLabel,
    body: Said,
    /// Whether this row is an ALTERNATIVE to the one before it — mutually-exclusive repairs the
    /// reader picks between, rather than a further step. A run of them renders as one join.
    alternative: bool,
}

/// The NEXT STEPS panel of the triptych (`28E` §8: the human's markup grew this from a two-line
/// epilogue into a labeled STRUCTURAL remediation arc — `lean-prose-down-one-step`, mechanical
/// explanation over flowing paragraphs). The panel is OMITTED entirely when it has no rows, which
/// is the triptych-collapse `28G` strawman `e-skipped-quiet` demonstrates.
struct NextSteps {
    /// The line that frames the rows. A suspected-wrong skip opens on the reader's doubt; a
    /// deliberate decline opens by saying there is nothing to repair (`28G` strawman
    /// `c-declined-unsound`), and reusing one opener for both would ask a reader to fix a
    /// correctly-behaving line.
    opener: Said,
    rows: Vec<StepRow>,
}

/// A `dorc why <addr>` triptych (`28G` Phase W1): the contrastive OUTCOME, the quoted-speakers
/// ANALYSIS, and the structural NEXT STEPS. Content + structure are the law; wording and arrangement
/// ride `27V:rul-output-form-unwelded` — transcripts re-bless freely on churn here.
struct ChainRender {
    /// The `N|command` references of every wall this line was kept past, and the provider whose
    /// at-most claim licensed that — what the aggregate's TRUST SPENT item names.
    crossed: String,
    claimant: String,
    outcome: Said,
    /// The ANALYSIS opener, then the speaker rows, then the numberless join restatement
    /// (`28E` §7 adapt-join-only-numbering: a linear chain carries no numbering at all).
    analysis_opener: Said,
    links: Vec<ChainLink>,
    /// Every book line this answer names, in source order — the participating-lines block
    /// (`28E` §8 presence-complete, density-selected).
    ///
    /// PRESENCE is the invariant: a participant the ANALYSIS mentions and this list omits would be
    /// a false provenance claim, so the block is complete over the closure it declares and the
    /// panels below select only how MUCH each one gets. The closure is the answer's own references
    /// — the asked line plus every wall and crossing the chain names. It is NOT the value closure:
    /// no reaching-definitions query is exposed, so a `PORT=443` feeding the asked line's argv
    /// (`28G` strawman `b-wide-guarded` line 29) does not appear, which is exactly why the block
    /// has to say which closure it is complete over rather than saying "participating lines" flat.
    participants: Vec<usize>,
    /// The guard dorc shipped in place of a skip, as sh — the answer to "so what DID it do"
    /// (`28G` strawman `b-wide-guarded`). Not our bytes: the oracle author wrote the check and the
    /// admin wrote the command it fronts.
    shipped: Option<String>,
    join: Option<Said>,
    next_steps: NextSteps,
}

/// The consent flag as the BINARY spells it. The corpus names this lever
/// `--risk-faultless-skips` (`spike/CLAUDE.md` survive-license, `271:rul-flag-is-razor-residue`); the
/// cli implements `--risk-faultless-skips`. A why-surface pointer must be copy-paste-true (`28E` §7
/// held-placement-reread), so the render prints what the parser accepts and the rename is flagged
/// upward rather than papered over here.
const CONSENT_FLAG: &str = "--risk-faultless-skips";

/// The engine's own name in the speaker column — the only row dorc speaks in its own voice, and it
/// speaks only about its own derivations (`28E` §8 quoted-speakers).
const ENGINE_SPEAKER: &str = "dorc";

/// Build the survived-elision triptych (`28G` Phase W1 over the `27V` §4 flagship). Pure over the
/// plan's [`dorc_plan::SurvivalWitness`] + display context. `None` when the step survived no wall (an
/// ordinary elision has no chain to walk).
///
/// The row set is `28G` strawman `a-fire-morning`'s exactly: the probe's REPORT, the site oracle's
/// standing VOUCH, each crossed wall's at-most CLAIM, and dorc's own disjointness DERIVATION. Two
/// links the as-built chain carried as rows are stated in the contrastive OUTCOME instead — the
/// wall's run and the admin's consent — because neither has a speaker to quote, and OUTCOME puts the
/// consent AHEAD of the chain rather than last in it.
///
/// The `suspect:` row's claim of UNIQUENESS is a model fact — a count of covers-unmeasured rows —
/// never one fragment knowing what another rendered (`28E` lean-start-without-mutual-awareness).
#[expect(
    clippy::too_many_arguments,
    reason = "the chain builder threads the display context it quotes (reference/address/disposition/license/wall-map/interner/oracle paths+sources); each is a distinct pipeline output, not a bundle-able struct"
)]
#[expect(
    clippy::too_many_lines,
    reason = "one linear row-by-row chain construction followed by its NEXT STEPS rows; splitting it would scatter the ONE place the strawman's row set is expressed"
)]
fn survival_chain(
    reference: &str,
    address: &str,
    disposition: &dorc_plan::Disposition,
    license: &dorc_plan::ReplaceLicense,
    walls: &BTreeMap<dorc_plan::LeafId, String>,
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> Option<ChainRender> {
    let witness = license.derivation().survival.as_ref()?;
    let backing = render_coord(witness.backing(), interner);
    let outcome = outcome_word(disposition);
    let reported = license.derivation().probe.and_then(|p| p.reported);
    let mut links = vec![ChainLink {
        tier: SpeechAct::Measured,
        speaker: reported_speaker(reference, reported, oracle_paths, oracle_srcs),
        payload: Said::Value(dorc_plan::fact_label(interner, license.fact())),
        quoted: true,
        event: reported.map(reported_event),
        explanation: None,
        excerpt: None,
    }];
    if license.derivation().establish_vouches.is_empty() {
        links.push(ChainLink {
            tier: SpeechAct::Vouched,
            speaker: oracle_locus(license.derivation().vouch_span, oracle_paths, oracle_srcs),
            payload: Said::words("why-vouch-payload-site", &[&backing]),
            quoted: true,
            event: None,
            explanation: None,
            excerpt: None,
        });
    } else {
        let mut by_speaker: Vec<(Option<String>, Vec<String>)> = Vec::new();
        for receipt in &license.derivation().establish_vouches {
            let speaker = oracle_locus(receipt.defining_span, oracle_paths, oracle_srcs);
            let label = dorc_plan::fact_label(interner, receipt.fact);
            match by_speaker.iter_mut().find(|(who, _)| *who == speaker) {
                Some((_, labels)) => labels.push(label),
                None => by_speaker.push((speaker, vec![label])),
            }
        }
        links.extend(by_speaker.into_iter().map(|(speaker, labels)| ChainLink {
            tier: SpeechAct::Vouched,
            speaker,
            payload: Said::words("why-vouch-payload-establish", &[&brace_selectors(&labels)]),
            quoted: true,
            event: None,
            explanation: None,
            excerpt: None,
        }));
    }
    let mut wall_refs: Vec<String> = Vec::new();
    let mut claimants: Vec<String> = Vec::new();
    let mut leverage: Option<String> = None;
    for c in witness.crossings() {
        let provider = interner.resolve(c.provider()).to_owned();
        claimants.push(provider.clone());
        wall_refs.push(
            walls
                .get(&c.wall_leaf())
                .cloned()
                .unwrap_or_else(|| provider.clone()),
        );
        let coords: Vec<String> = c
            .footprint()
            .iter()
            .map(|fc| render_coord(*fc, interner))
            .collect();
        let locus = oracle_locus(c.footprint_span(), oracle_paths, oracle_srcs);
        leverage = leverage.or_else(|| locus.clone());
        links.push(ChainLink {
            tier: SpeechAct::Claimed,
            speaker: locus,
            payload: Said::words("why-claims-payload", &[&provider, &coords.join(" ")]),
            quoted: true,
            event: None,
            explanation: Some(Said::words("why-claims-covers-unmeasured", &[])),
            excerpt: oracle_excerpt(c.footprint_span(), oracle_paths, oracle_srcs),
        });
    }
    links.push(ChainLink {
        tier: SpeechAct::Derived,
        speaker: Some(ENGINE_SPEAKER.to_owned()),
        payload: Said::words("why-derives-payload-disjoint", &[&backing]),
        quoted: false,
        event: None,
        explanation: None,
        excerpt: None,
    });

    let joined_walls = wall_refs.join(", ");
    let unmeasured = links
        .iter()
        .filter(|l| l.tier.knowability() == Knowability::CoversUnmeasured)
        .count();
    let mut rows = vec![StepRow {
        label: StepLabel::Suspect,
        body: if unmeasured == 1 {
            Said::words(
                "why-next-step-suspect-sole-claim",
                &[&joined_walls, &backing],
            )
        } else {
            Said::words(
                "why-next-step-suspect-several-claims",
                &[&unmeasured.to_string(), &backing],
            )
        },
        alternative: false,
    }];
    if let Some(lev) = &leverage {
        rows.push(StepRow {
            label: StepLabel::Fix,
            body: Said::words("why-next-step-fix-widen", &[lev]),
            alternative: false,
        });
    }
    rows.push(StepRow {
        label: StepLabel::Fix,
        body: Said::words("why-next-step-fix-replan", &[CONSENT_FLAG]),
        alternative: leverage.is_some(),
    });
    rows.push(StepRow {
        label: StepLabel::Verify,
        body: Said::words("why-next-step-verify", &[]),
        alternative: false,
    });
    rows.push(StepRow {
        label: StepLabel::Review,
        body: Said::words("why-next-step-review", &[address]),
        alternative: false,
    });
    Some(ChainRender {
        crossed: joined_walls.clone(),
        claimant: claimants.join(", "),
        outcome: Said::words(
            "why-outcome-contrastive",
            &[
                reference,
                &outcome,
                &foil_word(disposition),
                &why_words(
                    "why-outcome-because-survived",
                    &[&joined_walls, CONSENT_FLAG],
                ),
            ],
        ),
        analysis_opener: Said::words("why-analysis-opener", &[reference, &outcome]),
        links,
        participants: Vec::new(),
        shipped: None,
        join: Some(Said::words("why-analysis-join", &[&joined_walls, &backing])),
        next_steps: NextSteps {
            opener: Said::words("why-next-steps-opener", &[reference]),
            rows,
        },
    })
}

/// Build the HEALTHY-GUARD triptych (`28G` strawmen `b-wide-guarded` and `d-guard-fell-through`):
/// what dorc knew, why it was not enough to skip, and what it shipped instead.
///
/// The wall rows are the point (`289:fnd-guarded-chain-omits-the-wall`). A guarded line's whole
/// story is that a good report went stale — and until now the chain named the report and the vouch
/// but never the thing that came between them, leaving the reader with two links that ought to have
/// been sufficient and no account of why they were not.
#[expect(
    clippy::too_many_arguments,
    reason = "the guard chain quotes the same display context the survival chain does, plus the wall walk it names its walls from; each is a distinct pipeline output"
)]
fn guard_chain(
    reference: &str,
    address: &str,
    original: &str,
    license: &dorc_plan::GuardLicense,
    walls_above: &[&WallStep],
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> ChainRender {
    let backing = dorc_plan::fact_label(interner, license.fact());
    let reported = license.reported();
    let mut links = vec![
        ChainLink {
            tier: SpeechAct::Measured,
            speaker: reported_speaker(reference, reported, oracle_paths, oracle_srcs),
            payload: Said::Value(backing.clone()),
            quoted: true,
            event: reported.map(reported_event),
            explanation: None,
            excerpt: None,
        },
        ChainLink {
            tier: SpeechAct::Vouched,
            speaker: oracle_locus(license.insert().defining_span(), oracle_paths, oracle_srcs),
            payload: Said::words("why-vouch-payload-site", &[&backing]),
            quoted: true,
            event: None,
            explanation: None,
            excerpt: None,
        },
    ];
    links.extend(walls_above.iter().map(|wall| ChainLink {
        tier: SpeechAct::Ran,
        speaker: Some(format!("{}|{}", wall.line, wall.word)),
        payload: Said::Words(
            "why-wall-payload",
            dorc_aid::arrangement::arrangement_text(
                &dorc_aid::arrangement::CONST_ARRANGEMENTS,
                "why-wall-payload",
                Some(wall.role.occurrence()),
            ),
        ),
        quoted: false,
        event: None,
        explanation: None,
        excerpt: None,
    }));
    let wall_refs: Vec<String> = walls_above
        .iter()
        .map(|wall| format!("{}|{}", wall.line, wall.word))
        .collect();
    let joined_walls = wall_refs.join(", ");
    // ANALYSIS names every wall; `describe:` only the UNDESCRIBED ones, else the nag lands wrong.
    let describable: Vec<String> = walls_above
        .iter()
        .filter(|wall| wall.role == WallRole::Opaque)
        .map(|wall| format!("{}|{}", wall.line, wall.word))
        .collect();
    let rows = if describable.is_empty() {
        Vec::new()
    } else {
        vec![
            StepRow {
                label: StepLabel::Describe,
                body: Said::Words(
                    "why-next-step-describe-walls",
                    why_words_at(
                        "why-next-step-describe-walls",
                        Some(usize::from(describable.len() > 1)),
                        &[&describable.join(", ")],
                    ),
                ),
                alternative: false,
            },
            StepRow {
                label: StepLabel::Review,
                body: Said::words("why-next-step-review", &[address]),
                alternative: false,
            },
        ]
    };
    ChainRender {
        crossed: joined_walls.clone(),
        claimant: String::new(),
        outcome: Said::words(
            "why-outcome-contrastive",
            &[
                reference,
                &OutcomeKind::Guarded.word(),
                &OutcomeKind::Guarded.foil().word(),
                &why_words("why-outcome-because-guarded", &[&joined_walls]),
            ],
        ),
        analysis_opener: Said::words("why-analysis-opener-guarded", &[]),
        links,
        participants: Vec::new(),
        shipped: Some(license.insert().display_line(original)),
        join: Some(Said::words("why-analysis-join-guarded", &[reference])),
        next_steps: NextSteps {
            opener: Said::words("why-next-steps-opener-guarded", &[]),
            rows,
        },
    }
}

/// The narrative classes this run MINTED and no render CONSUMED, as greppable
/// `[unnarrated: <class>]` lines (`28E:prop-unnarrated-is-visible`).
///
/// The aid plane fails toward narration (`two-plane-aid-law`), and a class that mints without ever
/// being rendered fails toward SILENCE instead — the standing
/// `289:seam-narrative-render-unconsumed`. Deepest pull tier only: this is a maintainer's disclosure
/// about the surface's own coverage, and putting it on the default surface would spend the
/// firefighter's attention on dorc's gaps rather than on their host.
///
/// `narratable` carries the version coupling. On a replay it is false when the durable's record
/// stream and this binary's narrative plane disagree, because the census would then be a confident
/// claim about a run whose class set this binary never held.
fn unnarrated_lines(narrative: &[CollapseNarrative], narratable: bool) -> Vec<String> {
    if !narratable {
        return Vec::new();
    }
    let mut classes: Vec<&'static str> = Vec::new();
    for record in narrative {
        let rendered = matches!(
            record.kind(),
            CollapseKind::VerdictDecline {
                authored_reason: Some(_),
                ..
            }
        );
        let class = record.class_name();
        if !rendered && !classes.contains(&class) {
            classes.push(class);
        }
    }
    classes.sort_unstable();
    classes
        .into_iter()
        .map(|class| format!("[unnarrated: {class}]"))
        .collect()
}

/// Collapse coordinate labels sharing one `kind:entity` into the brace-alternation display
/// `kind:entity@{a,b}` (`28G` strawman `a-fire-morning` line 72; `281` §7's own spelling for a
/// multi-cell coordinate, reused rather than a second one invented).
///
/// DISPLAY only, and deliberately so: the model still holds N separate [`dorc_core::FactKey`]s,
/// each with its own selector, and every comparison the engine does still runs per-cell through
/// the `selector_covers` chokepoint (`selector-chokepoint`). Folding the model would make two cells
/// look like one to the algebra, which is exactly the collision the chokepoint exists to prevent.
/// Order is preserved and duplicates are kept out; a label with no `@` passes through untouched.
fn brace_selectors(labels: &[String]) -> String {
    let mut grouped: Vec<(String, Vec<String>)> = Vec::new();
    for label in labels {
        let (head, selector) = match label.rsplit_once('@') {
            Some((head, selector)) => (head.to_owned(), Some(selector.to_owned())),
            None => (label.clone(), None),
        };
        if !grouped.iter().any(|(seen, _)| *seen == head) {
            grouped.push((head.clone(), Vec::new()));
        }
        if let Some(entry) = grouped.iter_mut().find(|(seen, _)| *seen == head)
            && let Some(selector) = selector
            && !entry.1.contains(&selector)
        {
            entry.1.push(selector);
        }
    }
    grouped
        .into_iter()
        .map(|(head, selectors)| match selectors.len() {
            0 => head,
            1 => format!("{head}@{}", selectors.join("")),
            _ => format!("{head}@{{{}}}", selectors.join(",")),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The asked line plus every line the answer names, deduped and in source order.
///
/// Source order, not chain order: the block is a reading aid over the book, and the reader's eye
/// expects the file's own sequence. Chain ORDER stays the ANALYSIS panel's, where it means
/// something (`28E` lean-ordering-is-a-seam).
fn participants(asked: usize, named: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut lines: BTreeSet<usize> = named.collect();
    lines.insert(asked);
    lines.into_iter().collect()
}

/// The participating-lines block that opens an addressed answer (`28E` §8 presence-complete,
/// density-selected; `28G` strawman `a-fire-morning` lines 57–59).
///
/// The qualification row beneath it is not decoration. "Participating lines" read alone, at 03:40,
/// becomes "nothing else was involved" — a claim about the WORLD rather than about a closure, and
/// exactly the negative `28E:rul-never-a-dinna-do-it-layer` forbids Dorc from ever synthesizing.
/// The block therefore states which closure it is complete over.
fn participating_block(lines: &[usize], filename: &str, book_src: &str) -> Vec<Node<Face>> {
    let source: Vec<&str> = book_src.lines().collect();
    let rows: Vec<CodeLine<Face>> = lines
        .iter()
        .filter_map(|number| {
            let text = source.get(number.saturating_sub(1))?;
            Some(CodeLine {
                gutter: Some(dorc_aid::weave::value(
                    number.to_string(),
                    "why-participating-lines",
                    "line",
                    WHY_VALUE_CAP,
                )),
                cells: vec![CodeCell::new(vec![dorc_aid::weave::foreign(
                    text.trim_end(),
                    filename.to_owned(),
                    WHY_SOURCE_CAP,
                )])],
            })
        })
        .collect();
    if rows.is_empty() {
        return Vec::new();
    }
    vec![
        Node::new(NodeKind::Code(CodeBlock {
            table: Some(Face::Table(format!("participating:{filename}"))),
            mode: Literalness::Literal,
            locus: Some(vec![dorc_aid::weave::words(
                why_words("why-participating-lines-locus", &[filename]),
                "why-participating-lines-locus",
            )]),
            lines: rows,
        })),
        registry_paragraph("why-participating-lines-closure"),
    ]
}

/// The sites whose author DELIBERATELY declined, keyed by leaf — the pull surface's index into the
/// `VerdictDecline` narratives (`collapse-mints-narrative`).
///
/// Only a decline carrying an `authored_reason` counts: an unclassed decline is ordinary
/// control-flow (`rul-vouch-is-verdict-authoring` — no vouch ⇒ run) and says nothing a reader could
/// act on, while a CLASSED one is the author stating why. Reading a narrative for display is the
/// one direction the two planes allow (`two-plane-aid-law`); nothing here reaches a license.
fn authored_declines(
    narrative: &[CollapseNarrative],
) -> BTreeMap<dorc_plan::LeafId, dorc_aid::narrative::AuthoredReason> {
    let mut out = BTreeMap::new();
    for record in narrative {
        if let CollapseKind::VerdictDecline {
            site,
            authored_reason: Some(reason),
            ..
        } = record.kind()
        {
            out.entry(site.leaf).or_insert(*reason);
        }
    }
    out
}

/// Build the AUTHORED-REFUSAL triptych (`28G` strawman `c-declined-unsound`): the answer for a
/// line that runs because the person who knows the tool ruled the question unanswerable.
///
/// This is the pull-surface half of `27W`'s decline design, which until now existed only as a
/// stderr push line (`289:fnd-decline-class-is-push-only`): asking `dorc why` about a declined site
/// showed the generic ran-blind answer, which reads as a GAP in dorc's knowledge when it is the
/// opposite — a place the knowledge exists and says no.
///
/// The class drives everything class-specific through occurrence-keyed rows, so an `unmodeled`
/// decline (the author's "not yet") never wears an `unsound` decline's words (the author's "not
/// ever, by anyone").
fn decline_chain(
    reference: &str,
    address: &str,
    word: &str,
    reason: &dorc_aid::narrative::AuthoredReason,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> ChainRender {
    let class = reason.class;
    let occurrence = Some(class.occurrence());
    let arm = Some((reason.arm.0, reason.arm_file));
    let class_words = |slug: &str| {
        dorc_aid::arrangement::arrangement_text(
            &dorc_aid::arrangement::CONST_ARRANGEMENTS,
            slug,
            occurrence,
        )
    };
    let links = vec![
        ChainLink {
            tier: SpeechAct::Declined,
            speaker: oracle_locus(arm, oracle_paths, oracle_srcs),
            payload: Said::words("why-declines-payload", &[class.token()]),
            quoted: true,
            event: None,
            explanation: Some(Said::Words(
                "why-declines-explanation",
                class_words("why-declines-explanation"),
            )),
            excerpt: oracle_excerpt(arm, oracle_paths, oracle_srcs),
        },
        ChainLink {
            tier: SpeechAct::Derived,
            speaker: Some(ENGINE_SPEAKER.to_owned()),
            payload: Said::words("why-declines-derives-cannot-say-runs", &[]),
            quoted: false,
            event: None,
            explanation: None,
            excerpt: None,
        },
    ];
    ChainRender {
        crossed: String::new(),
        claimant: String::new(),
        outcome: Said::words(
            "why-outcome-contrastive",
            &[
                reference,
                &OutcomeKind::Ran.word(),
                &OutcomeKind::Ran.foil().word(),
                &why_words("why-outcome-because-declined", &[word]),
            ],
        ),
        analysis_opener: Said::words("why-analysis-opener-plain", &[reference]),
        links,
        participants: Vec::new(),
        shipped: None,
        join: Some(Said::Words(
            "why-declines-join",
            class_words("why-declines-join"),
        )),
        next_steps: NextSteps {
            opener: Said::Words(
                "why-declines-next-steps-opener",
                class_words("why-declines-next-steps-opener"),
            ),
            rows: vec![StepRow {
                label: StepLabel::Review,
                body: Said::words("why-next-step-review", &[address]),
                alternative: false,
            }],
        },
    }
}

/// The speaker of a `reported` row: the oracle line whose body produced the report
/// (`service.oracle.sh:12`, the strawmen's shape), from the reporting record's own threaded
/// predict-defining span.
///
/// Falls back to the shipped funcname when the span is honestly absent — an entry-composed or
/// connected-pipe body has no single defining funcdef, and no record reported at all when the
/// license was minted without a probe-attribution map. Naming a file we did not derive would be a
/// mis-attributed speaker, the worst class of aid failure (`271:rul-sin-ordering`).
fn reported_speaker(
    reference: &str,
    reported: Option<dorc_plan::ReportedObservation>,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> Option<String> {
    reported
        .and_then(|r| oracle_locus(r.predict_span, oracle_paths, oracle_srcs))
        .or_else(|| Some(predict_speaker(reference)))
}

/// The `reported` row's payload trailer: WHEN the controller took the report in, and what the
/// probe command exited with (`28G` strawman `a-fire-morning`'s `(ran 01:59:52, rc 0)` slot).
///
/// The instant is CONTROLLER-minted (`28F:rul-probe-instants-host-says-no-times`, human-typed: the
/// host says no times, ever), and the moment it names is the one this edge actually holds — when
/// the record was received, not when the check ran on the host. The word says so; a `ran` here
/// would date a host event we were never told about. `None` for the instant ⇒ the rc alone, never
/// a fabricated moment.
fn reported_event(reported: dorc_plan::ReportedObservation) -> Said {
    let rc = reported.tool_rc.0.to_string();
    match reported.stamp.received_at {
        Some(at) => Said::words(
            "why-chain-event-received",
            &[&dorc_aid::instant::time_text(at), &rc],
        ),
        None => Said::words("why-chain-event-rc-only", &[&rc]),
    }
}

/// The speaker of a `reported` row whose record carried no defining span: the funcname the probe
/// actually shipped and invoked (`<provider>__predict`) — exact, and claiming no file.
fn predict_speaker(reference: &str) -> String {
    let word = reference.split_once('|').map_or(reference, |(_, w)| w);
    format!(
        "{}__predict",
        dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(word))
    )
}

/// The canonical render width. Layout is the RENDERER's, never the semantics engine's
/// (`28E` §8 `rul-renderer-owns-layout`): every seat below hands `weft` a MARKED tree, and weft
/// rules columns, wrapping and blocks. The doc-algebra reflow engine that will replace its filler
/// is still deferred (`28G` §2), so the surface renders at ONE fixed width and transcripts pin
/// there.
const WHY_WIDTH: usize = 92;

/// The indent the whole `dorc why <addr>` triptych sits at.
const TRIPTYCH_INSET: usize = 3;

/// The table every NEXT STEPS row joins. Naming one relates the alternatives buried inside the
/// repair join to the steps around them, which no structural rule can do — and the table then
/// hangs or stacks as a unit (`28F:rul-table-degrades-whole`).
const STEPS_TABLE: &str = "why-next-steps";

/// The table the receipt header's record lines join — one block, degrading as a unit.
const RECEIPT_TABLE: &str = "why-receipt";

/// Render a marked tree at `inset` and print it. The ONE seat where the why surface becomes bytes.
fn print_document(nodes: Vec<Node<Face>>, inset: usize) {
    let frame = weft::Frame::of_width(weft::Width::new(WHY_WIDTH)).inset(inset);
    print!(
        "{}",
        weft::render_framed(&Document::new(nodes), &frame).text()
    );
}

/// One attributed fragment as a paragraph.
fn paragraph(said: &Said, part: &'static str) -> Node<Face> {
    Node::new(NodeKind::Prose(Paragraph {
        runs: vec![said.run(part)],
    }))
}

/// A whole registry line as a paragraph.
fn registry_paragraph(slug: &'static str) -> Node<Face> {
    paragraph(&Said::words(slug, &[]), slug)
}

/// A titled panel. The header WORDS come from the registry; weft mints the rule around them and
/// nothing else (`28F:rul-weft-geometry-vs-words`).
fn panel(header: &'static str, body: Vec<Node<Face>>) -> Node<Face> {
    Node::new(NodeKind::Section(Section {
        header: vec![dorc_aid::weave::words(why_words(header, &[]), header)],
        counts: None,
        body,
    }))
}

/// The ANALYSIS panel's quoted-speakers rows (`28E` §8): who spoke, the tier word as the sentence's
/// verb, and their own words quoted. The rows are adjacent siblings, so weft resolves them as one
/// table and every payload starts in one column — a `claims` row's covers-unmeasured paragraph and
/// its `as-written:` excerpt hang below the quote without breaking the run.
fn chain_rows(links: &[ChainLink]) -> Vec<Node<Face>> {
    links
        .iter()
        .map(|link| {
            let mut attachments: Vec<Node<Face>> = link
                .explanation
                .iter()
                .map(|said| paragraph(said, "why-chain-explanation"))
                .collect();
            attachments.extend(link.excerpt.iter().flat_map(excerpt_nodes));
            Node::new(NodeKind::Speaker(SpeakerRow {
                table: None,
                gutter: Some(dorc_aid::weave::mark(
                    rank_glyph(link.tier.knowability()),
                    "why-rank-mark",
                )),
                speaker: link
                    .speaker
                    .iter()
                    .map(|who| {
                        dorc_aid::weave::value(who, "why-chain-row", "speaker", WHY_VALUE_CAP)
                    })
                    .collect(),
                verb: Some(vec![dorc_aid::weave::words(
                    verb_word(link.tier),
                    "why-tier-word",
                )]),
                payload: Payload {
                    quoting: if link.quoted {
                        Quoting::Quoted
                    } else {
                        Quoting::Bare
                    },
                    runs: vec![link.payload.run("why-chain-row")],
                    trailer: link
                        .event
                        .iter()
                        .map(|event| event.run("why-chain-event"))
                        .collect(),
                },
                attachments,
            }))
        })
        .collect()
}

/// A speaker's source, inlined beneath their row (`28G` §0's foreign-text class; the strawman's
/// `as-written:` gutter). LITERAL mode: these bytes are byte-honest and never rewrapped, because a
/// break the source does not contain would be a lie about what the author wrote. A cut middle is
/// SHOWN, and the two halves name one table so their gutters stay in one column across it.
fn excerpt_nodes(excerpt: &Excerpt) -> Vec<Node<Face>> {
    let table = Some(Face::Table(format!("as-written:{}", excerpt.path)));
    let block = |lines: &[(usize, String)], locus: bool| {
        Node::new(NodeKind::Code(CodeBlock {
            table: table.clone(),
            mode: Literalness::Literal,
            locus: locus.then(|| {
                vec![dorc_aid::weave::words(
                    why_words("why-as-written-locus", &[&excerpt.path]),
                    "why-as-written-locus",
                )]
            }),
            lines: lines
                .iter()
                .map(|(number, text)| CodeLine {
                    gutter: Some(dorc_aid::weave::value(
                        number.to_string(),
                        "why-as-written",
                        "line",
                        WHY_VALUE_CAP,
                    )),
                    cells: vec![CodeCell::new(vec![dorc_aid::weave::foreign(
                        text,
                        excerpt.path.clone(),
                        WHY_SOURCE_CAP,
                    )])],
                })
                .collect(),
        }))
    };
    let mut out = vec![block(&excerpt.head, true)];
    if excerpt.elided > 0 {
        out.push(Node::new(NodeKind::Truncation(Truncation {
            note: vec![dorc_aid::weave::words(
                why_words("why-as-written-elided", &[&excerpt.elided.to_string()]),
                "why-as-written-elided",
            )],
        })));
        out.push(block(&excerpt.tail, false));
    }
    out
}

/// The guard as dorc shipped it, inlined beneath the ANALYSIS restatement.
///
/// Rides the same foreign-text class the `as-written:` excerpt does (`28G` §0), and for the same
/// reason: the check is the oracle author's invocation and the fallback is the admin's own line, so
/// nothing in the block is ours to rephrase. LITERAL mode — it is displayed sh, and a break the
/// shipped bytes do not contain would be a lie about what runs.
fn shipped_block(sh: &str) -> Node<Face> {
    Node::new(NodeKind::Code(CodeBlock {
        table: None,
        mode: Literalness::Literal,
        locus: None,
        lines: vec![CodeLine {
            gutter: None,
            cells: vec![CodeCell::new(vec![dorc_aid::weave::foreign(
                sh,
                "the shipped guard",
                WHY_SOURCE_CAP,
            )])],
        }],
    }))
}

/// One row of the remediation arc.
fn step_row(row: &StepRow) -> Node<Face> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table: Some(Face::Table(STEPS_TABLE.to_owned())),
        label: vec![dorc_aid::weave::words(
            row.label.word(),
            "why-next-step-label",
        )],
        body: vec![row.body.run("why-next-step")],
        attachments: Vec::new(),
    }))
}

/// The remediation arc. A row followed by ALTERNATIVES becomes one join under the consumer's own
/// connective, so the reader sees a choice rather than a to-do list; the shared table keeps the
/// branch rows squared up with the steps they sit between.
fn step_nodes(steps: &NextSteps) -> Vec<Node<Face>> {
    let mut out: Vec<Node<Face>> = Vec::new();
    let mut index = 0usize;
    while index < steps.rows.len() {
        let mut last = index;
        while steps
            .rows
            .get(last.saturating_add(1))
            .is_some_and(|next| next.alternative)
        {
            last = last.saturating_add(1);
        }
        if last == index {
            out.extend(steps.rows.get(index).map(step_row));
        } else {
            let branches = steps
                .rows
                .get(index..=last)
                .unwrap_or_default()
                .iter()
                .enumerate()
                .map(|(position, row)| Branch {
                    connective: (position > 0).then(|| {
                        vec![dorc_aid::weave::words(
                            why_words("why-alternative-connective", &[]),
                            "why-alternative-connective",
                        )]
                    }),
                    nodes: vec![step_row(row)],
                })
                .collect();
            out.push(Node::new(NodeKind::Join(Join {
                branches,
                restatement: None,
            })));
        }
        index = last.saturating_add(1);
    }
    out
}

/// A [`ChainRender`] as the three panels of the `dorc why <addr>` triptych (`28G` Phase W1).
///
/// The contrastive OUTCOME, the quoted-speakers ANALYSIS closed by its numberless join
/// restatement, and the structural NEXT STEPS — which is OMITTED whole when the question has no
/// next step, the question-relative floor `28G` strawman `e-skipped-quiet` demonstrates.
fn chain_nodes(chain: &ChainRender) -> Vec<Node<Face>> {
    let mut out = vec![panel(
        "why-outcome-heading",
        vec![paragraph(&chain.outcome, "why-outcome")],
    )];
    if chain.links.is_empty() {
        return out;
    }
    let mut analysis = vec![paragraph(&chain.analysis_opener, "why-analysis-opener")];
    analysis.extend(chain_rows(&chain.links));
    if chain
        .links
        .iter()
        .any(|link| link.tier.knowability() == Knowability::CoversUnmeasured)
    {
        analysis.push(registry_paragraph("why-mark-legend"));
    }
    analysis.extend(
        chain
            .join
            .iter()
            .map(|join| paragraph(join, "why-analysis-join")),
    );
    analysis.extend(chain.shipped.iter().map(|sh| shipped_block(sh)));
    out.push(panel("why-analysis-heading", analysis));

    if !chain.next_steps.rows.is_empty() {
        let mut arc = vec![paragraph(&chain.next_steps.opener, "why-next-steps-opener")];
        arc.extend(step_nodes(&chain.next_steps));
        out.push(panel("why-next-steps-heading", arc));
    }
    out
}
/// Which aggregate section a site belongs to in the zero-argument `dorc why` (`28E` §8, the
/// human-demonstrated three-way split; the PROBLEMS section name is RETIRED — genuine breakage
/// surfaces as a SURPRISE, and everything else dorc could do better about is an IMPROVEMENT).
#[derive(Clone, Copy, PartialEq, Eq)]
enum AggregateClass {
    /// Nothing to say about this site in the aggregate.
    Quiet,
    /// The world disagreed with the plan. Leads the aggregate when no trust was spent.
    Surprise,
    /// dorc could do better here, if the reader described more of their world.
    Improvement,
}

/// One site's WHY-record ([`emit_why_report`]): its SOURCE line (`rul24-lineno-identity`), the
/// one-line command, its admin-English outcome, its ANALYSIS rows, and which aggregate section it
/// belongs to.
struct WhySite {
    line: usize,
    /// The command's first word — the `certsync` of an `8|certsync` inline reference.
    word: String,
    command: String,
    outcome: String,
    foil: String,
    reasons: Vec<Said>,
    class: AggregateClass,
    /// The improvement's one-line reason, when this site is an [`AggregateClass::Improvement`].
    improvement: Option<Said>,
}

impl WhySite {
    /// The `N|command` inline reference (`28E` §8, human-demonstrated row shape): short enough to
    /// sit inside a sentence, and unambiguous because the line number is the SOURCE file's.
    fn reference(&self) -> String {
        format!("{}|{}", self.line, self.word)
    }

    /// The file-qualified address this site answers to — the exact bytes `dorc why` accepts back
    /// (`28E` §7 held-placement-reread: a pointer line must be copy-paste-true).
    fn address(&self, filename: &str) -> String {
        format!("{filename}:{}", self.line)
    }
}

/// ack-2 `dorc why`: the source-line-keyed WHY report — the focused query surface (the `plan`
/// preview points here). **rul24-lineno-identity** (a product invariant): the ONE line-number
/// space is the SOURCE file's, so a `file:N` this report PRINTS is exactly the `book.sh:N` a query
/// ACCEPTS — the mapping is 1:1 through [`dorc_aid::diag::line_col`]. Three addressing forms:
/// * `None` (unargumented) — the CURRENT run's PROBLEMS: every site that runs on a ⊤, runs
///   unprobed, or carries a guard / render-refusal (never a clean elide/omit) — "can't be typing
///   lines manually when you're already annoyed" (NO cross-run state; kSTATE stays parked).
/// * a `book.sh:N` / bare `N` line-address — the site(s) on that source line.
/// * free content — the site(s) whose command text contains it.
///
/// An ADDRESSED site renders the `28G` triptych (OUTCOME / ANALYSIS / NEXT STEPS); the unargumented
/// form renders the aggregate (TRUST SPENT first and uncapped, then SURPRISES, then IMPROVEMENTS).
#[expect(
    clippy::too_many_arguments,
    reason = "the why-report threads the compiled context it reports on (plan/probe/first-wall/why-diags/refusals/arena/ast/src/filename/interner); each is a distinct pipeline output, not a bundle-able struct"
)]
#[expect(
    clippy::too_many_lines,
    reason = "one linear per-disposition reason-derivation + the three addressing branches; splitting it would scatter the ONE report shape"
)]
fn emit_why_report(
    address: Option<&str>,
    plan: &dorc_plan::Plan,
    probe: &dorc_plan::ProbePlan,
    first_wall: Option<&FirstWallHint>,
    wall_steps: &[WallStep],
    why_diags: &[Diag],
    refusals: &[Diag],
    arena: &ProvArena,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
    filename: &str,
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
    narrative: &[CollapseNarrative],
    receipt: &Receipt,
) {
    use dorc_plan::Disposition;
    let declines = authored_declines(narrative);
    let unnarrated = if receipt.deepest_tier {
        unnarrated_lines(narrative, receipt.narratable)
    } else {
        Vec::new()
    };
    let mut sites: Vec<WhySite> = Vec::new();
    // A chain names the walls it crossed by `N|command`, never by internal site id (`28E` §8).
    let walls: BTreeMap<dorc_plan::LeafId, String> = plan
        .steps
        .iter()
        .map(|step| {
            let span = ast.node(step.ast).span;
            let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
            let line = dorc_aid::diag::line_col(book_src, lo).0;
            let text = book_src.get(lo..hi).unwrap_or("");
            let word = text.split_whitespace().next().unwrap_or("").to_owned();
            (step.leaf, format!("{line}|{word}"))
        })
        .collect();
    let lines_by_leaf: BTreeMap<dorc_plan::LeafId, usize> = plan
        .steps
        .iter()
        .map(|step| {
            let lo = ast.node(step.ast).span.lo.0 as usize;
            (step.leaf, dorc_aid::diag::line_col(book_src, lo).0)
        })
        .collect();
    let mut chains: Vec<(usize, ChainRender)> = Vec::new();
    for step in &plan.steps {
        let span = ast.node(step.ast).span;
        let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
        let line = dorc_aid::diag::line_col(book_src, lo).0;
        let raw = book_src.get(lo..hi).unwrap_or("<source unavailable>");
        let command = flatten_ws(raw);
        let word = raw.split_whitespace().next().unwrap_or("").to_owned();
        let reference = format!("{line}|{word}");
        if let Disposition::Replace(license, _) = &step.disposition
            && let Some(mut chain) = survival_chain(
                &reference,
                &format!("{filename}:{line}"),
                &step.disposition,
                license,
                &walls,
                interner,
                oracle_paths,
                oracle_srcs,
            )
        {
            let crossed = license
                .derivation()
                .survival
                .iter()
                .flat_map(dorc_plan::SurvivalWitness::crossings)
                .filter_map(|c| lines_by_leaf.get(&c.wall_leaf()).copied());
            chain.participants = participants(line, crossed);
            chains.push((line, chain));
        }
        if let Disposition::Guard(license) = &step.disposition {
            let above: Vec<&WallStep> = wall_steps
                .iter()
                .take_while(|wall| wall.leaf != step.leaf)
                .filter(|wall| matches!(wall.role, WallRole::Opaque | WallRole::Honest))
                .collect();
            let mut chain = guard_chain(
                &reference,
                &format!("{filename}:{line}"),
                &command,
                license,
                &above,
                interner,
                oracle_paths,
                oracle_srcs,
            );
            chain.participants = participants(line, above.iter().map(|wall| wall.line));
            chains.push((line, chain));
        }
        let authored_decline = declines.get(&step.leaf);
        if let Some(reason) = authored_decline {
            let mut chain = decline_chain(
                &reference,
                &format!("{filename}:{line}"),
                &word,
                reason,
                oracle_paths,
                oracle_srcs,
            );
            chain.participants = participants(line, std::iter::empty());
            chains.push((line, chain));
        }
        let refused = refusals.iter().any(|d| {
            d.primary
                .span()
                .is_some_and(|s| s.lo == span.lo && s.hi == span.hi)
        });
        let (reasons, class, improvement): (Vec<Said>, AggregateClass, Option<Said>) =
            match &step.disposition {
                Disposition::Run => {
                    if let Some(reason) = authored_decline {
                        (
                            vec![Said::words(
                                "why-reason-run-declined",
                                &[reason.class.token()],
                            )],
                            if reason.class.an_oracle_could_still_answer() {
                                AggregateClass::Improvement
                            } else {
                                AggregateClass::Quiet
                            },
                            reason.class.an_oracle_could_still_answer().then(|| {
                                Said::words("why-improvement-declined-unmodeled", &[&word])
                            }),
                        )
                    } else if let Some(reason) = top_run_reason(span, why_diags, arena, book_src) {
                        (vec![Said::Lens(reason)], AggregateClass::Quiet, None)
                    } else if probe.unresolvable.contains(&step.leaf)
                        && !is_structurally_unprobeable(&command)
                    {
                        let mut reasons = vec![Said::words("why-reason-run-unprobed", &[])];
                        // upcoming-firstwall-hint: the FIRST unmodeled wall carries the forward
                        // reasoning here — the pull detail behind the plan-mode `hint:` nag.
                        if let Some(fw) = first_wall.filter(|fw| fw.leaf == step.leaf) {
                            reasons.push(Said::Words("why-reason-first-wall", fw.why_detail()));
                        }
                        (
                            reasons,
                            AggregateClass::Improvement,
                            Some(Said::words("why-improvement-ran-blind", &[&word])),
                        )
                    } else {
                        (
                            vec![Said::words("why-reason-run-not-elidable", &[])],
                            AggregateClass::Quiet,
                            None,
                        )
                    }
                }
                Disposition::Replace(license, _) => {
                    let mut reasons = vec![Said::words(
                        "why-reason-skipped-converged",
                        &[&dorc_plan::fact_label(interner, license.fact())],
                    )];
                    if refused {
                        reasons.push(Said::words("why-reason-render-refused", &[]));
                        (reasons, AggregateClass::Surprise, None)
                    } else {
                        (reasons, AggregateClass::Quiet, None)
                    }
                }
                Disposition::Guard(license) => {
                    let kind = interner.resolve(license.fact().kind.0).to_owned();
                    if refused {
                        (
                            vec![Said::words("why-reason-guard-refused", &[&kind])],
                            AggregateClass::Surprise,
                            None,
                        )
                    } else {
                        // The leverage is the WALL, never the guarded line: an elided command
                        // casts no wall, so describing the wall is what frees this line.
                        let wall = first_wall.map(|fw| format!("{}|{}", fw.line, fw.word));
                        (
                            vec![Said::words("why-reason-guarded", &[&kind])],
                            if wall.is_some() {
                                AggregateClass::Improvement
                            } else {
                                AggregateClass::Quiet
                            },
                            wall.map(|w| Said::words("why-improvement-guarded-past-wall", &[&w])),
                        )
                    }
                }
                Disposition::Omit { .. } => (
                    vec![Said::words("why-reason-omitted", &[])],
                    AggregateClass::Quiet,
                    None,
                ),
            };
        sites.push(WhySite {
            line,
            word,
            command,

            outcome: outcome_word(&step.disposition),
            foil: foil_word(&step.disposition),
            reasons,
            class,
            improvement,
        });
    }

    if let Some(addr) = address {
        emit_why_triptych(addr, &sites, &chains, filename, book_src, &unnarrated);
    } else {
        emit_why_aggregate(&sites, &chains, filename, first_wall, receipt);
    }
}

/// The ADDRESSED pull answer (`28G` Phase W1): the triptych for every site the address matched.
///
/// Two addressing forms, both file-qualified on the way OUT (`rul24-lineno-identity`: the ONE
/// line-number space is the source file's, so a `file:N` this prints is exactly the address a query
/// accepts back): a `book.sh:N` / bare `N` line-address, or free content substring-matched against
/// the command text.
///
/// A survived elision already carries a fully-populated triptych; every other disposition gets the
/// same three panels built from its own ANALYSIS rows, so the surface has exactly ONE shape.
fn emit_why_triptych(
    address: &str,
    sites: &[WhySite],
    chains: &[(usize, ChainRender)],
    filename: &str,
    book_src: &str,
    unnarrated: &[String],
) {
    let matched: Vec<&WhySite> = match parse_line_address(address) {
        Some(n) if address_names_book(address, filename) => {
            sites.iter().filter(|s| s.line == n).collect()
        }
        Some(_) => Vec::new(),
        None => sites
            .iter()
            .filter(|s| s.command.contains(address))
            .collect(),
    };
    if matched.is_empty() {
        println!("{}", why_words("why-address-unmatched", &[address]));
        return;
    }
    let mut nodes: Vec<Node<Face>> = Vec::new();
    for site in matched {
        let built;
        let chain = if let Some((_, chain)) = chains.iter().find(|(l, _)| *l == site.line) {
            chain
        } else {
            built = plain_chain(site);
            &built
        };
        let participants = if chain.participants.is_empty() {
            vec![site.line]
        } else {
            chain.participants.clone()
        };
        nodes.extend(participating_block(&participants, filename, book_src));
        nodes.extend(chain_nodes(chain));
    }
    nodes.extend(
        unnarrated
            .iter()
            .map(|line| paragraph(&Said::Value(line.clone()), "why-unnarrated-class")),
    );
    print_document(nodes, TRIPTYCH_INSET);
    println!();
    print_document(vec![registry_paragraph("why-receipt-footer")], 0);
    println!();
}

/// The triptych for a site with no survival chain: the same three panels, with each of the site's
/// ANALYSIS rows spoken by dorc in its own voice — which is honest, because these rows ARE engine
/// derivations rather than quotations of any speaker (`28E` §8 quoted-speakers).
///
/// The site's LEADING reason becomes the contrastive because-clause and the rest become ANALYSIS
/// rows, so nothing is said twice: a one-reason site collapses to OUTCOME alone
/// (`28E` §7 adopt-question-relative-informativeness — demote what the asker's own question already
/// fixed). NEXT STEPS is likewise omitted, the triptych-collapse `28G` strawman `e-skipped-quiet`
/// demonstrates. The richer per-disposition panels — a guarded line naming its wall, a declined line
/// showing the author's arm — are the narration lane's.
fn plain_chain(site: &WhySite) -> ChainRender {
    let (because, rest) = site
        .reasons
        .split_first()
        .map_or((String::new(), &[][..]), |(head, tail)| {
            (head.text().to_owned(), tail)
        });
    ChainRender {
        crossed: String::new(),
        claimant: String::new(),
        outcome: Said::words(
            "why-outcome-contrastive",
            &[&site.reference(), &site.outcome, &site.foil, &because],
        ),
        analysis_opener: Said::words("why-analysis-opener-plain", &[&site.reference()]),
        links: rest
            .iter()
            .map(|reason| ChainLink {
                tier: SpeechAct::Derived,
                speaker: Some(ENGINE_SPEAKER.to_owned()),
                payload: reason.clone(),
                quoted: false,
                event: None,
                explanation: None,
                excerpt: None,
            })
            .collect(),
        participants: Vec::new(),
        shipped: None,
        join: None,
        next_steps: NextSteps {
            opener: Said::Value(String::new()),
            rows: Vec::new(),
        },
    }
}

/// The zero-argument `dorc why` aggregate (`28E` §8, human-demonstrated).
///
/// Section order is LAW, not taste: TRUST SPENT leads and is never capped
/// (`28E` §0 `rul-trust-spent-first-argless-why`, human-typed — danger in the user's face first),
/// SURPRISES follows and renders only when the world disagreed with the plan, and IMPROVEMENTS
/// closes, calm and quantified. The retired PROBLEMS section name appears nowhere.
///
/// The invocation record leads it ([`receipt_banner`]), because the reader arriving at 03:40 has to
/// know WHICH run they are reading before any item on it means anything.
fn emit_why_aggregate(
    sites: &[WhySite],
    chains: &[(usize, ChainRender)],
    filename: &str,
    first_wall: Option<&FirstWallHint>,
    receipt: &Receipt,
) {
    let surprises: Vec<&WhySite> = sites
        .iter()
        .filter(|s| s.class == AggregateClass::Surprise)
        .collect();
    let improvements: Vec<&WhySite> = sites
        .iter()
        .filter(|s| s.class == AggregateClass::Improvement)
        .collect();
    if chains.is_empty() && surprises.is_empty() && improvements.is_empty() {
        print_document(vec![receipt_banner(receipt)], 0);
        println!();
        println!("{}", why_words("why-nothing-to-report", &[filename]));
        return;
    }

    let mut nodes: Vec<Node<Face>> = vec![receipt_banner(receipt)];
    if !chains.is_empty() {
        let items = chains
            .iter()
            .filter_map(|(line, chain)| {
                let site = sites.iter().find(|s| s.line == *line)?;
                let reason = Said::words(
                    "why-trust-spent-item-reason",
                    &[&chain.crossed, &chain.claimant],
                );
                Some(aggregate_item(site, filename, &[&reason]))
            })
            .collect();
        nodes.push(panel("why-trust-spent-heading", items));
    }
    if !surprises.is_empty() {
        let items = surprises
            .iter()
            .map(|site| {
                let reasons: Vec<&Said> = site.reasons.iter().collect();
                aggregate_item(site, filename, &reasons)
            })
            .collect();
        nodes.push(panel("why-surprises-heading", items));
    }
    if !improvements.is_empty() {
        let mut items: Vec<Node<Face>> = improvements
            .iter()
            .map(|site| {
                let reasons: Vec<&Said> = site.improvement.iter().collect();
                aggregate_item(site, filename, &reasons)
            })
            .collect();
        if let Some(fw) = first_wall.filter(|fw| fw.unwall > 0) {
            items.push(paragraph(
                &Said::words(
                    "why-improvement-quantified",
                    &[&fw.word, &fw.unwall.to_string()],
                ),
                "why-improvement-quantified",
            ));
        }
        nodes.push(panel("why-improvements-heading", items));
    }
    nodes.push(registry_paragraph("why-receipt-footer"));
    print_document(nodes, 0);
}

/// One aggregate item: the `file:N | command` headline, its reason beneath, and the
/// `dorc why <addr>` pointer that turns it into the next question (`28E` §8 row shape).
///
/// The command is the ADMIN's own bytes, so it rides the foreign-text class rather than any
/// registry row — un-editable, and encoded on the way in (`28G` §0).
fn aggregate_item(site: &WhySite, filename: &str, reasons: &[&Said]) -> Node<Face> {
    let address = site.address(filename);
    let mut runs: Vec<Run<Face>> = Vec::new();
    for reason in reasons {
        if !runs.is_empty() {
            runs.push(dorc_aid::weave::mark(" ", "why-item-reason-gap"));
        }
        runs.push(reason.run("why-item-reason"));
    }
    Node::new(NodeKind::Banner(Banner {
        headline: vec![
            dorc_aid::weave::value(&address, "why-item", "address", WHY_VALUE_CAP),
            dorc_aid::weave::mark(" | ", "why-item-gutter"),
            dorc_aid::weave::foreign(&site.command, filename, WHY_SOURCE_CAP),
        ],
        body: vec![
            Node::new(NodeKind::Prose(Paragraph { runs })),
            Node::new(NodeKind::Pointer(PointerLine {
                placement: weft::Placement::Standalone,
                target: vec![dorc_aid::weave::words(
                    why_words("why-item-pointer", &[&address]),
                    "why-item-pointer",
                )],
            })),
        ],
    }))
}

/// The invocation record the zero-argument `dorc why` opens with (`28D:need-exact-input-identity`;
/// `28G` strawman `a-fire-morning` lines 33–38): which run this is, on which host, over which
/// bytes, under which consent, and what it decided.
///
/// Every field is CONTROLLER-minted (`rul-attribution-is-controller-minted`) — the host contributes
/// none of it, including the instant (`28F:rul-probe-instants-host-says-no-times`, human-typed).
struct Receipt {
    /// The durable's own start instant on a `--last` replay, this invocation's on a live one, and
    /// `None` when the edge had no clock. A replay carries the ORIGINAL run's instant, never this
    /// moment's — reading a replay's clock here would date the receipt to when it was read.
    at: Option<dorc_core::RunInstant>,
    /// Whether this report replays a durable rather than reporting the run that just happened.
    replayed: bool,
    host: String,
    book: String,
    book_digest: String,
    /// The commit the book sits at, when it sits at one exactly (`28E:lean-git-source-tracking-
    /// secondary`). Already-resolved pure data: the subprocess that answered it was spent at the
    /// edge, and a `None` here is indistinguishable from "no repository", by design.
    at_head: Option<source_match::SourceMatch>,
    /// The loaded oracles, in argv order.
    oracles: Vec<String>,
    /// The consent flag in force, or `None` for a flagless run.
    risk_profile: Option<&'static str>,
    counts: dorc_plan::DispositionCounts,
    /// `--all`: the reader asked for the deepest pull tier.
    deepest_tier: bool,
    /// Whether the `[unnarrated:]` census may be asserted over this report at all — the version
    /// coupling (`28E:prop-unnarrated-is-visible`'s caveat). False when a replayed durable's
    /// record stream is not the one this binary's narrative plane was built against.
    narratable: bool,
}

/// The receipt header as one banner: the run's identity, then the indented record of what it read
/// and what it decided.
///
/// The plan tally counts the TYPED disposition, never the rendered word: the words are registry
/// prose meant to churn (`27V:rul-output-form-unwelded`), so a tally keyed on them would silently
/// go wrong the first time someone rewrote one. Its skipped-count SPLIT is the line the reader
/// needs most — an `elide_by_trusted_claim` skip rests on an author's at-most claim rather than on
/// anything measured, and the two carry different risk.
fn receipt_banner(receipt: &Receipt) -> Node<Face> {
    let when = match (receipt.at, receipt.replayed) {
        (Some(at), false) => why_words(
            "why-receipt-when-live",
            &[&dorc_aid::instant::date_time_text(at)],
        ),
        (Some(at), true) => why_words(
            "why-receipt-when-replayed",
            &[&dorc_aid::instant::date_time_text(at)],
        ),
        (None, _) => why_words("why-receipt-when-undated", &[]),
    };
    let counts = receipt.counts;
    let tally = if counts.elide_by_trusted_claim == 0 {
        why_words(
            "why-receipt-plan-tally-by-proof",
            &[
                &counts.run.to_string(),
                &counts.guard.to_string(),
                &counts.elide.to_string(),
                &counts.elide_by_proof.to_string(),
            ],
        )
    } else {
        why_words(
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
        || why_words("why-receipt-risk-profile-none", &[]),
        str::to_owned,
    );
    // Replaces the digest row rather than joining it: exact-or-absent, never a third shape.
    let book_row = match &receipt.at_head {
        Some(matched) => Said::words(
            "why-receipt-book-at-head",
            &[&receipt.book, &matched.commit],
        ),
        None => Said::words("why-receipt-book", &[&receipt.book, &receipt.book_digest]),
    };
    let body = vec![
        receipt_row(&book_row),
        receipt_row(&Said::words(
            "why-receipt-oracles",
            &[&receipt.oracles.join(", ")],
        )),
        receipt_row(&Said::words("why-receipt-risk-profile", &[&risk])),
        receipt_row(&Said::Words("why-receipt-plan-tally", tally)),
        // `tc-apply-report-is-prediction`: no apply executor exists, so saying so IS the whole
        // replayed-voice obligation — never let a reader take a prediction for an outcome.
        receipt_row(&Said::words("why-receipt-dispositions-predicted", &[])),
        receipt_row(&Said::words("why-addressability-line", &[])),
    ];
    Node::new(NodeKind::Banner(Banner {
        headline: vec![dorc_aid::weave::words(
            why_words("why-receipt-header", &[&when, &receipt.host]),
            "why-receipt-header",
        )],
        body,
    }))
}

/// One line of the receipt header's indented record.
///
/// A labelled row rather than a paragraph, because the six lines are ONE block: weft keeps a run of
/// like rows tight and puts a blank line between unlike things, and a receipt broken up by blank
/// lines reads as six separate remarks rather than one identity.
fn receipt_row(said: &Said) -> Node<Face> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table: Some(Face::Table(RECEIPT_TABLE.to_owned())),
        label: Vec::new(),
        body: vec![said.run("why-receipt")],
        attachments: Vec::new(),
    }))
}

/// The ⊤-run cause for a Run site, if a `why_diags` disclosure covers it: the FIRST diag whose
/// primary span starts inside this command's span (the cmdsub-⊤ origin sits at/within the
/// command), rendered through the why-lens [`dorc_aid::diag::why`] (the same cause-chain the
/// `plan` render surfaces). `None` ⇒ no ⊤-cause (the caller falls to unprobed / not-elidable).
fn top_run_reason(
    span: dorc_core::Span,
    why_diags: &[Diag],
    arena: &ProvArena,
    book_src: &str,
) -> Option<String> {
    why_diags.iter().find_map(|d| {
        let psp = d.primary.span()?;
        (psp.lo.0 >= span.lo.0 && psp.lo.0 < span.hi.0)
            .then(|| dorc_aid::diag::why(d, arena, book_src).map(|e| e.reason))
            .flatten()
    })
}

/// Parse a `dorc why` address as a SOURCE line-number (rul24-lineno-identity): `book.sh:12` ⇒ 12,
/// bare `12` ⇒ 12 (the tail after the last `:` when numeric); a non-numeric tail ⇒ `None` ⇒ the
/// caller treats the address as free CONTENT to substring-match.
fn parse_line_address(addr: &str) -> Option<usize> {
    addr.rsplit(':')
        .next()
        .unwrap_or(addr)
        .parse::<usize>()
        .ok()
}

/// Does a file-QUALIFIED address name the book this run analyzed? A bare `12` names no file and
/// always matches; `web.sh:12` matches only `web.sh`, compared on the trailing path component so a
/// pasted `./web.sh:12` or an absolute path still resolves.
///
/// Load-bearing because the render now PRINTS file-qualified pointers: without the check, a
/// qualified address naming some other book silently answers for the analyzed one at rc 0 — the
/// same silent-wrong-surface class as `289:rider-why-last-address-order`.
fn address_names_book(addr: &str, book_name: &str) -> bool {
    let Some((file, _)) = addr.rsplit_once(':') else {
        return true;
    };
    if file.is_empty() {
        return true;
    }
    let tail = |path: &str| {
        path.rsplit(['/', '\\'])
            .next()
            .unwrap_or(path)
            .to_ascii_lowercase()
    };
    tail(file) == tail(book_name)
}

/// Render a [`dorc_plan::EntityCoord`] as `kind:entity` for the attribution surface (empty
/// entity ⇒ `kind:`, the singleton form). DISPLAY only — resolving an interned symbol for
/// provenance is explicitly permitted; the engine never DECODES it for meaning
/// (`inv-referent-agnostic`).
fn render_coord(coord: dorc_plan::EntityCoord, interner: &Interner) -> String {
    let kind = interner.resolve(coord.kind().0);
    let entity = match coord.entity() {
        dorc_core::EntityRef::Operand(token) => interner.resolve(token.0),
        dorc_core::EntityRef::Singleton => "",
    };
    format!("{kind}:{entity}")
}

/// The why-lens render + stage-4 dedup, factored PURE (the stderr side is [`emit_why_lens`]) so
/// the dedup is unit-testable (`x2-fd1`). For each caused-⊤ diag it renders the "why did this run"
/// line via [`dorc_aid::diag::why`], showing a given cause-SITE once.
///
/// stage-4 DEDUP KEY = `(cause, site)`, NOT the cause [`dorc_core::ProvId`] alone (`x2-fd1` fix,
/// `224` §10): under function inlining two call-sites splice the SAME body `AstId` (`inv-leaf-seam`)
/// ⇒ both `CmdsubOperandTop` diags hash-cons to ONE cause `ProvId`. Keying on cause alone collapsed
/// two GENUINELY INDEPENDENT forced runs (suppressing the 2nd `why:` — the over-suppression). They
/// differ by `site` (the stable `site N.M` leaf), so `(cause, site)` keeps them separately disclosed
/// while still deduping a true re-disclosure (same cause AND same site). Tracked in a `Vec` of
/// first-occurrences — `ProvId` is `!Ord` (no `BTreeSet`) and the diags arrive in node order, so
/// first-seen order is deterministic (`inv-determinism`). The only suppression built (no general
/// subsystem — `22D` §1 stage-4).
fn why_lens_lines(why_diags: &[Diag], arena: &ProvArena, src: &str) -> Vec<String> {
    let mut shown: Vec<(dorc_core::ProvId, dorc_aid::diag::SiteId)> = Vec::new();
    let mut lines = Vec::new();
    for diag in why_diags {
        if let Some(key) = cmdsub_cause_site(diag) {
            if shown.contains(&key) {
                continue; // stage-4: this (cause, site) was already explained — show it once
            }
            shown.push(key);
        }
        if let Some(explanation) = dorc_aid::diag::why(diag, arena, src) {
            lines.push(explanation.reason);
        }
    }
    lines
}

/// The stage-4 render-dedup key a why-lens diag carries, if any: `(⊤-cause, site)`. Only a
/// `CmdsubOperandTop` carries a cause at HEAD (stage-1); any other diag returns `None` (the why-lens
/// does not explain it anyway, fd-G), so it never participates in the dedup. The `site` half is what
/// separates two inlined call-sites sharing one cause `ProvId` (`x2-fd1`).
fn cmdsub_cause_site(diag: &Diag) -> Option<(dorc_core::ProvId, dorc_aid::diag::SiteId)> {
    match &diag.code {
        DiagCode::CmdsubOperandTop(p) => p.cause.map(|c| (c, p.site)),
        _ => None,
    }
}

#[cfg(test)]
mod brace_selector_tests {
    use super::brace_selectors;

    /// The whole point of the aggregation is that ONE entity reads as one thing. Two cells of one
    /// entity spelled as two full coordinates make a reader compare two long strings character by
    /// character to notice they differ only in the selector; `281` §7's brace-alternation puts the
    /// difference where the eye already is.
    #[test]
    fn cells_of_one_entity_collapse_to_one_braced_coordinate() {
        assert_eq!(
            brace_selectors(&[
                "sm.dorc.Package:nginx@enabled".to_owned(),
                "sm.dorc.Package:nginx@active".to_owned(),
            ]),
            "sm.dorc.Package:nginx@{enabled,active}"
        );
    }

    /// Grouping must never merge across entities: `nginx` and `redis` are different things, and a
    /// render that ran them together would be claiming a skip rested on cells it did not.
    #[test]
    fn different_entities_stay_separate_coordinates() {
        assert_eq!(
            brace_selectors(&[
                "sm.dorc.Package:nginx@installed".to_owned(),
                "sm.dorc.Package:redis@installed".to_owned(),
            ]),
            "sm.dorc.Package:nginx@installed sm.dorc.Package:redis@installed"
        );
    }

    /// A single cell keeps its plain spelling — braces around one token would suggest a set where
    /// there is one member — and a selector-less label passes through untouched, since the
    /// whole-entity form means something different from any braced set.
    #[test]
    fn a_lone_cell_and_a_selectorless_label_are_left_alone() {
        assert_eq!(
            brace_selectors(&["sm.dorc.Package:nginx@installed".to_owned()]),
            "sm.dorc.Package:nginx@installed"
        );
        assert_eq!(
            brace_selectors(&["sm.dorc.Package:nginx".to_owned()]),
            "sm.dorc.Package:nginx"
        );
    }

    /// A repeated cell is one cell. Two erased establishes of the same coordinate say the same
    /// thing, and `@{active,active}` would read as two distinct pieces of evidence.
    #[test]
    fn a_repeated_cell_is_not_listed_twice() {
        assert_eq!(
            brace_selectors(&[
                "sm.dorc.Service:nginx@active".to_owned(),
                "sm.dorc.Service:nginx@active".to_owned(),
            ]),
            "sm.dorc.Service:nginx@active"
        );
    }
}

#[cfg(test)]
mod why_lens_dedup_tests {
    //! `x2-fd1` (`22E`, `224` §10): the stage-4 render-dedup must key on `(cause, site)`, not the
    //! cause `ProvId` alone — else two inlined call-sites sharing one body-span cause collapse and
    //! the 2nd forced run's `why:` is wrongly suppressed. The arena hash-conses identical
    //! `(OriginKind, span)` origins (`core::prov` `hash_cons_shares_identical_origins`), so two
    //! `arena.leaf(TopCause, same_span)` calls reproduce the inlined-body cause collision.
    use dorc_aid::diag::{CmdsubOperandTop, CommandName, Diag, DiagCode, OperandPosition, SiteId};
    use dorc_core::{BytePos, LeafId, OriginKind, ProvArena, Span, TopCause};

    fn cmdsub_top(arena: &mut ProvArena, leaf: u32, body_span: Span) -> Diag {
        let cause = arena.leaf(OriginKind::TopCause, Some(body_span));
        Diag::new(
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: SiteId::leaf(LeafId(leaf)),
                position: OperandPosition::Operand(1),
                cause: Some(cause),
                top_cause: TopCause::UnmodeledExpansion,
                command: CommandName::Literal("apt-get".to_owned()),
            }),
            Span::new(BytePos(0), BytePos(20)),
        )
    }

    #[test]
    fn two_inlined_sites_sharing_one_cause_both_disclose() {
        // `apt_install "$(curl a)"; apt_install "$(curl b)"`: both calls inline ONE wrapper body ⇒
        // one shared cause ProvId, distinct call-site leaves. (cause, site) keeps BOTH `why:`s; the
        // old cause-alone key suppressed the 2nd (x2-fd1, disclosure-only over-suppression).
        let mut arena = ProvArena::new();
        let body = Span::new(BytePos(11), BytePos(20));
        let diags = [
            cmdsub_top(&mut arena, 3, body),
            cmdsub_top(&mut arena, 7, body),
        ];
        let lines = super::why_lens_lines(&diags, &arena, "apt_install \"$(curl a)\"");
        assert_eq!(
            lines.len(),
            2,
            "two inlined sites sharing one cause must BOTH disclose: {lines:?}"
        );
    }

    #[test]
    fn an_identical_cause_and_site_is_shown_once() {
        // The dedup still FIRES for a true duplicate (same cause AND same site) — the (cause, site)
        // key didn't neuter the stage-4 dedup into a no-op.
        let mut arena = ProvArena::new();
        let body = Span::new(BytePos(11), BytePos(20));
        let diags = [
            cmdsub_top(&mut arena, 3, body),
            cmdsub_top(&mut arena, 3, body),
        ];
        let lines = super::why_lens_lines(&diags, &arena, "apt-get install \"$(date)\"");
        assert_eq!(
            lines.len(),
            1,
            "an identical (cause, site) re-disclosure is shown once: {lines:?}"
        );
    }
}

/// The gate-5 disposition tag for a [`dorc_plan::Disposition`] — `run`/`replace`/`omit`.
/// gate-5 asserts the bare-book argv-echo ONLY for `run` sites: a `replace`d or `omit`ted
/// site is deliberately not in the apply run-set, and a guarded omit may be absent from the
/// BARE book too (a preceding guard short-circuits it), so it must not be asserted ⊆ the
/// log (task-O / strain-D3b-fold-vs-gate5).
fn disposition_tag(disposition: &dorc_plan::Disposition) -> &'static str {
    use dorc_plan::Disposition;
    match disposition {
        Disposition::Run => "run",
        Disposition::Replace(_, _) => "replace",
        Disposition::Omit { .. } => "omit",
        // A guard's ledger tag (gate-6's widened judge reads it — cf-5/cf-6): gate-5 skips it (a
        // guarded site's run-set argv is the check invocation, not the bare book's mutator argv).
        Disposition::Guard(_) => "guard",
    }
}

/// Re-key the site-keyed [`SiteResults`] to the `FactKey → Observable` map
/// [`dorc_plan::build_plan`] consumes (`inv-site-keyed-results`): for each resolvable
/// site the probe compiled, look up its reported [`Verdict`] (the Effect channel) and
/// — gated by the wrong-concrete firewall — its rc (the Status channel), keyed by the
/// site's resolved fact. A site with no reported record folds to `Unknown` ⇒ run
/// (`kFAIL-perform`).
///
/// THE WRONG-CONCRETE FIREWALL, Query-only (202 §3 / 20C §7 / task-D2 — the heart of
/// the task): a record's `rc` feeds the fold's Status channel ONLY for a Query-class
/// site that passed rule-query-validity. The asymmetry is load-bearing and
/// disaster-class if wrong:
/// * an **establish** site's record-rc is the PROBE command's rc (`dpkg-query`'s), NOT
///   the mutator's (`apt-get`'s) — feeding it would be a confidently-wrong concrete, so
///   its status stays `Predicted::Top` UNCONDITIONALLY (the check's rc is never the
///   mutator's rc);
/// * a **valid Query** site's record-rc IS the guard's own rc (`command -v`'s) — the
///   exact value the `&&`/`||`/`if`/errexit consumer reads — so it feeds Status;
/// * an **invalid Query** site (a mutator/opaque reached it from entry) has a stale
///   resting rc, so its status also stays `Predicted::Top` ⇒ the guard runs for real.
///
/// SAME-CELL CONFLICT FLOOR (20I find-6a / item-5): two sites mapping to the SAME cell
/// merge **conservatively** — a per-channel DISAGREEMENT degrades that channel to ⊤
/// (`Verdict::Unknown` for Effect, `Predicted::Top` for the others), NEVER last-write-wins.
/// Normally only one site per cell is resolvable (a same-command re-establish is
/// `EstablishWritten` ⇒ unresolvable ⇒ absent from `checks`, strain-D1-samecell), so this
/// is a defensive floor: it cannot be argued the two records "must agree" (a forged or
/// flaky host could disagree), and the conservative ⊤ folds to run (`kFAIL-perform`) — the
/// only safe resolution of a self-contradicting host. [`merge_observable`] does the join.
fn facts_from_sites(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
) -> (
    BTreeMap<dorc_core::FactKey, Observable>,
    Vec<CollapseNarrative>,
) {
    use dorc_plan::ProbeSiteKind;
    let mut by_fact: BTreeMap<dorc_core::FactKey, Observable> = BTreeMap::new();
    // C4 (`27V` Lane A): the `Measured` fact-merge narrative minted beside the ⊤-fold. `first_site`
    // remembers each cell's first establisher so a cross-site conflict names both operands.
    let mut collapse_narrative: Vec<CollapseNarrative> = Vec::new();
    let mut first_site: BTreeMap<dorc_core::FactKey, dorc_aid::diag::SiteId> = BTreeMap::new();
    for check in &probe.checks {
        let site_id = dorc_aid::diag::SiteId {
            leaf: check.site,
            member: check.member,
        };
        // Key the record by (site, member) — a member check (`site N.M`) reads its own
        // sub-record (task-L2 item-4); an ordinary check (`site N`) reads `member: None`.
        let record = results.records.get(&RecordKey {
            site: check.site,
            member: check.member,
        });
        let effect = record.map_or(Verdict::Unknown, |r| r.verdict);
        // The firewall: only a VALID Query site's rc is fold-usable as Status — and only when
        // the record is not a duplicate-meet CONFLICT (`262` §2: a conflicting rc is can't-tell,
        // so it must not substitute into the control-flow fold).
        let status = match check.site_kind {
            ProbeSiteKind::Query { valid: true } => record.map_or(Predicted::Top, |r| {
                if r.conflicted {
                    Predicted::Top
                } else {
                    Predicted::Value(r.rc)
                }
            }),
            // Establish site (check's rc, not the mutator's) OR an invalid Query
            // (stale resting rc) ⇒ withhold the rc, status stays ⊤.
            ProbeSiteKind::Establish | ProbeSiteKind::Query { valid: false } => Predicted::Top,
        };
        // The reserved Stdout/Stderr claims ride into the tuple verbatim (19F §3 shape).
        // INERT this round: nothing emits them, and `consumption_ok` blocks a consumed
        // stdout/stderr UNCONDITIONALLY (16F §3) — never reading the claim value — so a
        // (hypothetical) non-⊤ claim cannot relax that block. The slot is plumbed so a
        // future stdout-producing probe + vouch is a value change, not a representation one.
        let stdout = record.map_or(Predicted::Top, |r| r.stdout);
        let stderr = record.map_or(Predicted::Top, |r| r.stderr);
        let obs = Observable {
            effect,
            status,
            stdout,
            stderr,
        };
        // Source 1 — a WITHIN-site conflict: a valid Query whose parse-merged record contradicts
        // itself (`r.conflicted`), so its fold-usable rc is withheld to ⊤ above.
        if matches!(check.site_kind, ProbeSiteKind::Query { valid: true })
            && record.is_some_and(|r| r.conflicted)
        {
            collapse_narrative.push(measured_merge_disagreement(site_id, &[site_id]));
        }
        // C5 substitution refusal. tc-substitution-refusal-scope: minted ONLY for the invalid-Query
        // withhold (a genuine consumed-channel refusal), NOT the establish withhold (firewall-by-
        // design; it elides via Effect). Flagged UP — a scoping judgment (`inv-superposition`).
        if matches!(check.site_kind, ProbeSiteKind::Query { valid: false }) {
            collapse_narrative.push(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::SubstitutionRefusal {
                    site: site_id,
                    top_channel: dorc_core::Channel::StatusRelaxable,
                },
            ));
        }
        // Runtime EntryFailure (`27C` §3): entry-bearing ≥2 sink-landing, class-only + inert. rc 127
        // ⇒ missing deps; other ≥2 ⇒ in-context decline. Refused/Impossible unminted (SEAM: a marker).
        if check.entry.is_some()
            && let Some(rc) = record.map(|r| r.rc.0)
            && rc >= 2
        {
            let class = if rc == 127 {
                dorc_aid::narrative::EntryFailureTag::MissingDeps
            } else {
                dorc_aid::narrative::EntryFailureTag::InContextDecline
            };
            collapse_narrative.push(CollapseNarrative::new(
                SpeechAct::Measured,
                CollapseKind::EntryFailure {
                    site: site_id,
                    class,
                },
            ));
        }
        // Source 2 — a CROSS-site conflict: two sites on one cell disagree ⇒ the meet ⊤s the channel.
        if let Some(prior) = by_fact.get(&check.fact).copied() {
            if prior != obs {
                let prior_site = first_site.get(&check.fact).copied().unwrap_or(site_id);
                collapse_narrative
                    .push(measured_merge_disagreement(site_id, &[prior_site, site_id]));
            }
            by_fact.insert(check.fact, merge_observable(prior, obs));
        } else {
            first_site.insert(check.fact, site_id);
            by_fact.insert(check.fact, obs);
        }
    }
    (by_fact, collapse_narrative)
}

/// C6 (`27V` Lane A · `OriginKind::ProbeResult`): mint one probe-result origin per received record
/// and key it by the fact it establishes, so [`dorc_plan::build_plan_walled`] can attach it to a
/// licensing disposition's `Witness` — the why-chain's tie from "why THIS elision" back to the
/// exact record that measured it. The stamp is the record's stream ordinal (deterministic, no
/// clock — `inv-determinism`). A fact backed by two records JOINS their origins (two records are
/// two events). Runs at the cli edge where the arena lives (`io-at-edges-only`); the [`Observable`]
/// stays receipt-clean (the tc-c6-scope ruling: the receipt rides the record, not the value).
///
/// The origin NODE's source span stays `None`: an [`dorc_core::OriginNode`] carries a bare
/// [`dorc_core::Span`], which is file-ambiguous once >1 oracle is loaded (`law-lineno-identity`).
/// The file-qualified reporting span therefore rides the [`dorc_plan::ReportedObservation`] beside
/// the receipt, which is also where the tool-rc and the observation instant live.
///
/// A fact measured by SEVERAL records keeps the joined receipt but reports NO single observation:
/// two records are two events with no one speaker, instant, or rc, and inventing a winner would be
/// a fabricated measurement.
fn probe_origins(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
    arena: &mut ProvArena,
) -> BTreeMap<dorc_core::FactKey, dorc_plan::ProbeAttribution> {
    let mut origins: BTreeMap<dorc_core::FactKey, dorc_plan::ProbeAttribution> = BTreeMap::new();
    for check in &probe.checks {
        let Some(record) = results.records.get(&RecordKey {
            site: check.site,
            member: check.member,
        }) else {
            continue;
        };
        let origin = arena.leaf(dorc_core::OriginKind::ProbeResult(record.stamp), None);
        let reported = Some(dorc_plan::ReportedObservation {
            stamp: record.stamp,
            tool_rc: record.rc,
            predict_span: check.defining_span,
        });
        let attribution = match origins.get(&check.fact) {
            Some(prior) => dorc_plan::ProbeAttribution {
                origin: arena.join(None, &[prior.origin, origin]).unwrap_or(origin),
                reported: None,
            },
            None => dorc_plan::ProbeAttribution { origin, reported },
        };
        origins.insert(check.fact, attribution);
    }
    origins
}

/// Build the `Measured`-tier fact-merge narrative a probe-result disagreement mints (C4;
/// `27V` Lane A, `AID-NEEDS:law-collapse-mints-narrative`): a host self-contradiction at `cell`,
/// carrying the participating establisher sites as operands (`minting_line`/`shown` filled by d3).
/// Decision-inert (`two-plane-aid-law`): the conservative meet already folded the channel to ⊤
/// (`kFAIL-perform`, the only safe resolution of a self-contradicting host); this only narrates why.
fn measured_merge_disagreement(
    cell: dorc_aid::diag::SiteId,
    sites: &[dorc_aid::diag::SiteId],
) -> CollapseNarrative {
    let operands = dorc_aid::narrative::Operands::capped(
        sites
            .iter()
            .map(|&site| dorc_aid::narrative::ValueOperand {
                site,
                minting_line: None,
                shown: None,
            })
            .collect(),
    );
    CollapseNarrative::new(
        SpeechAct::Measured,
        CollapseKind::FactMergeDisagreement { cell, operands },
    )
}

/// Conservatively merge two [`Observable`]s reported for the SAME cell (20I find-6a /
/// item-5). Per channel: equal values pass through; ANY disagreement degrades the
/// channel to ⊤ (`Verdict::Unknown` for Effect, `Predicted::Top` for status/stdout/
/// stderr). This is the meet toward ⊤ — never last-write-wins — so a self-contradicting
/// host folds to run (`kFAIL-perform`), the only safe resolution. Order-independent
/// (commutative + idempotent): merging in any site order yields the same ⊤-on-conflict.
fn merge_observable(a: Observable, b: Observable) -> Observable {
    Observable {
        effect: if a.effect == b.effect {
            a.effect
        } else {
            Verdict::Unknown
        },
        status: if a.status == b.status {
            a.status
        } else {
            Predicted::Top
        },
        stdout: if a.stdout == b.stdout {
            a.stdout
        } else {
            Predicted::Top
        },
        stderr: if a.stderr == b.stderr {
            a.stderr
        } else {
            Predicted::Top
        },
    }
}

/// A record's key: the command **site** (the stable `LeafId`, `inv-site-keyed-results`)
/// plus an optional MEMBER index (task-L2 item-4): `None` for an ordinary single-fact
/// record (`site N`), `Some(m)` for member `m` of an in-loop Members family (`site N.M`).
/// The probe's [`dorc_plan::ProbePredict`] carries the same `(site, member)` pair, so the
/// bridge ([`facts_from_sites`]) keys a member record back to that member's cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RecordKey {
    site: dorc_plan::LeafId,
    member: Option<u32>,
}

/// Controller-owned width-one identity. Payload records never construct or refresh this scope.
#[derive(Debug)]
struct WidthOneAttemptScope {
    host: String,
    target: WidthOneLocalTargetId,
    nonce: String,
    attempt: u32,
    sources: Vec<(String, String)>,
    generation: InitialWidthOneGeneration,
    book: (String, String),
}

#[derive(Debug)]
struct WidthOneLocalTargetId;

#[derive(Debug)]
struct InitialWidthOneGeneration;

impl WidthOneAttemptScope {
    fn new(
        framing: &dorc_plan::records::Framing,
        book_name: &str,
        book: &str,
        paths: &[String],
        sources: &[String],
    ) -> Self {
        Self {
            host: framing.host.clone(),
            target: WidthOneLocalTargetId,
            nonce: framing.nonce.0.clone(),
            attempt: framing.attempt,
            sources: paths
                .iter()
                .zip(sources)
                .map(|(path, source)| (path.clone(), book_digest(source)))
                .collect(),
            generation: InitialWidthOneGeneration,
            book: (book_name.to_owned(), book_digest(book)),
        }
    }

    fn retain(&self) {
        let _ = (
            &self.host,
            &self.target,
            &self.nonce,
            self.attempt,
            &self.sources,
            &self.generation,
            &self.book,
        );
    }
}

/// Keeps controller attribution attached while live evidence participates in planning.
struct ScopedHostEvidence<T> {
    scope: WidthOneAttemptScope,
    value: T,
}

impl<T> ScopedHostEvidence<T> {
    fn new(scope: WidthOneAttemptScope, value: T) -> Self {
        Self { scope, value }
    }

    fn borrow(&self) -> &T {
        &self.value
    }

    fn scope(&self) -> &WidthOneAttemptScope {
        self.scope.retain();
        &self.scope
    }
}

/// The probe results parsed from stdin, keyed by [`RecordKey`] (site, optional member —
/// `inv-site-keyed-results` + task-L2 item-4). One record per (site, member): the reported
/// Effect [`Verdict`] plus the raw probe-command rc carried alongside it. Whether that rc
/// is fold-usable is the FIREWALL's decision ([`facts_from_sites`]), not the parser's —
/// the parser faithfully carries what the probe reported (`inv-superposition`: the wire
/// transports the observed rc; the phased caller decides which channel, if any, it feeds).
#[derive(Debug, Default)]
struct SiteResults {
    records: BTreeMap<RecordKey, SiteRecord>,
    /// The DERIVATION coord-blob lane (24E §5 / fork-s4-coordwire): per escalated wall-site, the
    /// raw `kind:entity` coordinate lines its host-run `touches()` printed (`deriv <leafid>
    /// coord=…`). Demuxed SEPARATELY from the `site` verdict records (a derivation-blob never
    /// collides with a site's `effect=`/`rc=` record — `inv-site-keyed-results`). Read back into a
    /// `Derived` [`dorc_plan::Footprint`] before the survival walk (24E §2 corr-§2).
    derivations: BTreeMap<dorc_plan::LeafId, Vec<String>>,
    /// The DERIV FAMILY end-records (`262` §2 / `26A` stop-1): per escalated wall-site, the `n=<K>`
    /// declared by its `deriv-end <leafid> n=<K>` close-record. THE SAFETY INVERSION: a deriv
    /// footprint is an AT-MOST claim, so a mid-family cut SHRINKS it (⇒ more survivals — the
    /// under-execution direction). The consumer ([`merge_derived_footprints`]) refuses a family
    /// whose received coord count ≠ this `K` (or that has no end-record) ⇒ wall-total. Absent key
    /// ⇒ the family never closed ⇒ refused.
    derivation_ends: BTreeMap<dorc_plan::LeafId, u32>,
    /// The RESOLVER canonicalization lane (24F §3): per `kind:entity` coordinate label, the readback
    /// of running its `<kind>.resolve()` host-side — a [`ResolvOutcome`]. Demuxed SEPARATELY from the
    /// verdict + derivation lanes (keyed by the coordinate, not a site — resolution is a pure function
    /// of the coordinate). Read into a [`dorc_plan::Resolutions`] before the survival walk.
    resolutions: BTreeMap<String, ResolvOutcome>,
    /// The REACH expansion lane (24G §4): per `(coordinate label, arm index)`, the RAW ENTITY lines a
    /// DYNAMIC `reaches()` arm printed host-side (`reach <coord> arm=<n> entity=…`). Demuxed SEPARATELY
    /// (keyed by the coordinate + arm, a pure function of them). Read into the footprints (via
    /// [`dorc_plan::Footprint::add_reached`]) before the survival walk. NB the arm index re-keys each
    /// line back to the arm's LIFTED kind (the vocabulary fence — the kind is never host-minted).
    reaches: BTreeMap<(String, usize), Vec<String>>,
    /// The REPORT lane (`27W` §2 tier-3): the `<verb> <class> <tail>` emissions an oracle wrote on
    /// its declining paths, re-keyed to their emitting site by the probe scaffold (`report site=<key>
    /// …`). Decision-inert (`two-plane-aid-law`): classes route AID only, never the license plane.
    /// Noise-tolerant (`27W:rul-report-noise-tolerant`): nothing is silently dropped — an
    /// unrecognized verb/class or free-form line is RETAINED (`recognized=false`), sanitized +
    /// size-capped, for max-verbosity display (d4). Ordered by arrival (a `Vec`, deduped on the
    /// whole record).
    reports: Vec<ReportRecord>,
    /// Was the source stream FRAMED (`262` §2)? Gates the at-most deriv-family completeness
    /// check ([`merge_derived_footprints`]) — only a framed stream carries `deriv-end`
    /// close-records; the legacy authored fixtures are trusted-complete.
    framed: bool,
}

/// One ingested report-lane record (`27W` §2 tier-3 · `decline-class-emission`): an emission an
/// oracle wrote on a declining path (`printf '<verb> <class> <tail>' >>"${DREP_V1:-/dev/null}"`),
/// re-keyed to its site by the probe scaffold. Decision-inert. Noise-tolerant: an unrecognized
/// verb/class is kept (`recognized=false`) as a generic author-note, never dropped, never an error.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ReportRecord {
    /// The emitting site (the scaffold's `site=<key>`), if attached.
    site: Option<RecordKey>,
    /// The recognized decline class, or `None` (degrade-generic — unknown token / free-form line).
    class: Option<dorc_aid::narrative::DeclineClass>,
    /// The full raw `<verb> <class> <tail>` emission, sanitized + size-capped at ingestion (the
    /// BASIC cap only — full why-surface sanitization is the security round's, `an-output-sanitization`
    /// fence named; `law-whylog-is-sensitive`). Retained for max-verbosity display (d4).
    raw: String,
    /// Whether the verb + class were BOTH recognized (else retained as a generic author-note).
    recognized: bool,
}

/// The ingestion size-cap on a report-lane emission's raw text (`27W` §2 — the BASIC cap only). A
/// tail longer than this is truncated with an ellipsis; a curious admin still sees the head at max
/// verbosity, and the full text never reaches a decision (decision-inert).
const REPORT_RAW_CAP: usize = 200;

/// One coordinate's resolver readback (24F §3): the canonical form its `<kind>.resolve()` printed, or
/// [`Dangling`](ResolvOutcome::Dangling) — the resolver's natural failure on an enumerable kind (§4,
/// a reference to a non-existent entity), which rides the may-alias degrade + a loud diagnostic.
#[derive(Debug, Clone)]
enum ResolvOutcome {
    /// The resolver printed a canonical form (interned into the shared vocabulary at readback).
    Canonical(String),
    /// The resolver failed (non-zero rc / empty stdout) — a dangling reference (§4) ⇒ may-alias.
    Dangling,
}

/// One site's reported observation: the Effect-channel [`Verdict`], the raw probe-command
/// exit status, and the RESERVED `Stdout`/`Stderr` [`OutBytes`]s (`19F` §3 tuple shape).
/// The out-claims are parsed-and-stored but produce NOTHING this round — the probe never
/// emits `stdout=`/`stderr=`, so they arrive `Predicted::Top` in practice; the slots exist
/// so a future stdout-producing probe is a value-plumbing change, not a grammar change.
#[derive(Debug, Clone, Copy)]
struct SiteRecord {
    verdict: Verdict,
    rc: Rc,
    stdout: Predicted<OutBytes>,
    stderr: Predicted<OutBytes>,
    /// This record's identity as a probe EVENT (C6, `27V` §2): its arrival ordinal in the deframed
    /// stream (deterministic, no clock) plus the instant the controller observed it, when the edge
    /// injected a clock. Minted straight into the [`dorc_core::OriginKind::ProbeResult`] origin so
    /// the whylog can order/attribute probe events. A meet keeps the first-seen stamp.
    stamp: dorc_core::ProbeStamp,
    /// A DUPLICATE-MEET marker (`262` §2 / `26A` stop-1): set when two records for one
    /// (site, member) key DISAGREED and were met toward ⊤. The §1 tie-break law forbids
    /// first-wins/last-wins; a conflict is can't-tell. `verdict` is already `Unknown` when
    /// this is set (effect ⇒ run); this ALSO withholds the fold-usable Query rc
    /// ([`facts_from_sites`]) so a conflicting rc cannot substitute into the control-flow fold.
    conflicted: bool,
}

/// The rc a `128 + SIGPIPE` early-exit race lands on (`sigpipe-flap-class`, `279f` §5):
/// a `pipefail`-off pipeline whose early-exit consumer (`… | grep -q`) closed the pipe before an
/// upstream stage finished writing produces this race-dependently. It is opaque to Dorc's verdict
/// (a ≥2 flat-sink landing ⇒ cant-tell ⇒ run), so it is a WHY-lane nudge, never a decision input.
const SIGPIPE_RC: i32 = 141;

/// Emit the `sigpipe-flap-class` why-lane note (`279f` §5) for every probe record that landed
/// [`SIGPIPE_RC`]: a stderr advisory suggesting a full-read form over the early-exit `| grep -q`.
/// The landing is always SAFE (the site runs) and the verdict never flaps run-to-run, so this is a
/// pure nudge — no gate asserts it, and it feeds no decision. Ordered (records is a `BTreeMap`).
fn emit_sigpipe_race_notes(results: &SiteResults) {
    for (key, rec) in &results.records {
        if rec.rc.0 == SIGPIPE_RC {
            let site = match key.member {
                Some(m) => format!("{}.{m}", key.site.0),
                None => key.site.0.to_string(),
            };
            eprintln!(
                "note: site {site} landed rc 141 (likely benign early-exit SIGPIPE race; \
                 consider a full-read form over `| grep -q`)"
            );
        }
    }
}

/// The engine-owned display word for a decline class (`27W:rul-class-starter-set`). Delegates to
/// the one home ([`dorc_aid::narrative::DeclineClass::token`]); display only
/// (`inv-referent-agnostic`; spellings ride `27V:rul-output-form-unwelded`).
fn decline_class_word(class: dorc_aid::narrative::DeclineClass) -> &'static str {
    class.token()
}

/// Emit the report lane's SELECTED default disclosure (`27W` §2 · `decline-class-emission`): one
/// advisory note per RECOGNIZED author decline-class (the class-routing an admin sees by default).
/// The unrecognized / free-form NOISE is retained in `results.reports`
/// (`27W:rul-report-noise-tolerant`) but printed only at max verbosity — d4's surface, not this
/// default. THINNEST surface pending d4's arrangement (`27V:rul-output-form-unwelded`): the wording
/// re-blesses freely. Empty in the corpus (no oracle emits report lines ⇒
/// `empty-world-byte-identical`). `note:` prefix ⇒ never crosses the gate-3 error floor.
fn emit_report_lane_notes(results: &SiteResults) {
    for r in &results.reports {
        // Default surface = recognized records; noise waits for d4's max verbosity.
        let Some(class) = r.class.filter(|_| r.recognized) else {
            continue;
        };
        let at = match r.site {
            Some(k) => match k.member {
                Some(m) => format!(" at site {}.{m}", k.site.0),
                None => format!(" at site {}", k.site.0),
            },
            None => String::new(),
        };
        let tail = r.raw.splitn(3, ' ').nth(2).unwrap_or(r.raw.as_str());
        eprintln!(
            "note: author declines [{}]{at} -- {tail}",
            decline_class_word(class)
        );
    }
}

/// C3 report-lane pairing (`27W` §3 `rul-static-first-three-tier`): fold each ingested tier-3 report
/// record (a recognized decline class + a site) into that site's
/// [`dorc_aid::narrative::CollapseKind::VerdictDecline`] narrative via
/// [`dorc_aid::narrative::CollapseNarrative::with_authored_reason`]. The runtime record supplies only
/// the missing CLASS — a dynamic format string defeated static reading (`27W` §2 "one honest loss"),
/// so `classify_decline` traced the reached decline arm but left the class unread. The arm span +
/// file id are the [`dorc_aid::narrative::CollapseKind::VerdictDecline`]'s OWN already-traced reached
/// arm (the precise `file:line`,
/// `27V:mech-minting-line-threading`); the inventory-keyed lookup the spec sketched is redundant here
/// — a class-readable inventory arm implies static ALREADY populated the reason. Deduped
/// `(site, arm, class)` against tier-2: `with_authored_reason` is idempotent, so a runtime echo never
/// overwrites a statically populated reason (static wins). Empty in the corpus (no oracle emits ⇒
/// `empty-world-byte-identical`). Decision-inert (`two-plane-aid-law`): classes route AID only.
fn pair_authored_reasons(
    narratives: Vec<CollapseNarrative>,
    reports: &[ReportRecord],
) -> Vec<CollapseNarrative> {
    use dorc_aid::narrative::{AuthoredReason, CollapseKind};
    narratives
        .into_iter()
        .map(|ev| {
            let CollapseKind::VerdictDecline {
                site,
                arm,
                arm_file,
                ..
            } = *ev.kind()
            else {
                return ev;
            };
            // A recognized runtime record for this exact site (leaf + member), if one arrived.
            let paired = reports.iter().find_map(|r| {
                let rk = r.site?;
                let matches = r.recognized
                    && dorc_aid::diag::SiteId {
                        leaf: rk.site,
                        member: rk.member,
                    } == site;
                matches.then_some(r.class).flatten()
            });
            match paired {
                Some(class) => ev.with_authored_reason(AuthoredReason {
                    class,
                    arm,
                    arm_file,
                }),
                None => ev,
            }
        })
        .collect()
}

/// Emit the STATIC/paired decline-class disclosure (`27W` §3 `rul-static-first-three-tier`): one
/// why-lens line per site whose `VerdictDecline` narrative carries an `authored_reason` — the tier-2
/// static class (argv threaded statically) OR the tier-3 runtime class paired in by
/// [`pair_authored_reasons`]. A DECLINE is a why-a-line-RUNS disclosure (the author declined ⇒ the
/// site runs), so it belongs on the same `why:` lane as the run/survival attributions, surfacing the
/// emitting arm's `file:line` (C7). The tier-3 RUNTIME records ALSO take the advisory `note:` lane
/// ([`emit_report_lane_notes`]) for their free-tail text. The full why-lens CHAIN render (numbered
/// links, tier words) is the arrangement walker. Empty in the corpus (no oracle emits ⇒
/// `empty-world-byte-identical`); wording rides `27V:rul-output-form-unwelded`. Never `error[` ⇒
/// ignored by the gate-3 floor; the `why:` prefix lets gate-7 pin it.
fn emit_static_decline_notes(
    collapse_narrative: &[CollapseNarrative],
    oracle_paths: &[String],
    oracle_srcs: &[String],
) {
    for line in static_decline_notes(collapse_narrative, oracle_paths, oracle_srcs) {
        eprintln!("{line}");
    }
}

/// The pure half of [`emit_static_decline_notes`] — the ONE narrative-consuming render seat, split
/// out so it is assertable in-process (`289:rul-mint-hardening-package` item 4b: the "and the chain
/// renders it" clause, satisfiable today for exactly this class). Wording rides
/// `27V:rul-output-form-unwelded`.
fn static_decline_notes(
    collapse_narrative: &[CollapseNarrative],
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> Vec<String> {
    let mut lines = Vec::new();
    for ev in collapse_narrative {
        let CollapseKind::VerdictDecline {
            authored_reason: Some(reason),
            ..
        } = ev.kind()
        else {
            continue;
        };
        let at = oracle_locus(
            Some((reason.arm.0, reason.arm_file)),
            oracle_paths,
            oracle_srcs,
        )
        .map(|l| format!(" (at {l})"))
        .unwrap_or_default();
        lines.push(format!(
            "why: author declines [{}]{at} -- a deliberate decline the author classed, so the site \
             runs",
            decline_class_word(reason.class)
        ));
    }
    lines
}

/// A deterministic content digest binding the records stream to the exact analyzed book bytes
/// (`262` §2 `book=`; discharges `tc-probe-no-digest`). Hand-rolled SHA-256 in the kernel
/// (`28F:rul-digest-lands-now` retired the FNV-1a-64 stand-in; the kernel stays dependency-clean
/// per `inv-determinism`, so it is written out rather than pulled in). Computed at the I/O edge
/// (`io-at-edges-only`), never in the kernel.
fn book_digest(book_src: &str) -> String {
    dorc_plan::invocation::book_digest(book_src)
}

/// Parse stdin probe-results into the site-keyed [`SiteResults`]
/// (`inv-site-keyed-results`). One line form; blank lines and `#` comments are ignored
/// (so the probe's own `# site …` provenance echo can be piped back), and any
/// unrecognized line is dropped — a site with no record folds to `Unknown` ⇒ run (the
/// `kFAIL-perform` floor; the `garbage-stdin` case pins it):
///
/// * `site <leafid> effect=<holds|absent|cant-tell> rc=<n> [stdout=<text> stderr=<text>]`
///   — the records the rendered probe emits (the return channel, 202 §3). `effect` is the
///   Effect channel mapped to a [`Verdict`] (`holds`/`absent`/`cant-tell` ⇒
///   `Converged`/`Diverged`/`Unknown`). `rc` is the raw probe-command status, carried on
///   the wire; the FIREWALL ([`facts_from_sites`]) decides whether it is fold-usable (only
///   for a valid Query-class site). A missing/garbled `rc` defaults to `Rc(0)` for
///   carriage but is irrelevant unless the firewall admits it.
///
/// `stdout=`/`stderr=` are RESERVED (`19F` §3 tuple shape): the parser accepts-and-stores
/// them (interning the text into a [`OutBytes`] on the record) but NOTHING produces them —
/// the rendered probe emits no such keys, and the consumed-stdout/stderr gate stays the
/// unconditional block it is regardless. Reserving them means a future stdout-producing
/// probe is a value-plumbing change, not a grammar change. The interner is threaded for
/// this (the `cli` is the I/O edge; `inv-determinism` exempts it).
///
/// (The transitional `declared-rc <leafid> rc=N` lane — the 19I §2 rc-injection
/// mechanism — is DEAD as of task-D2: a Query site's own `rc=` carries the fold rc now.)
#[cfg(test)]
fn parse_results(
    records: &[String],
    framed: bool,
    clock: &mut RunClock,
    interner: &mut Interner,
) -> SiteResults {
    let mut out = SiteResults {
        framed,
        ..SiteResults::default()
    };
    for (idx, line) in records.iter().enumerate() {
        let line = line.as_str();
        let Some((tag, rest)) = line.split_once(' ') else {
            continue; // a bare tag with no body ⇒ drop (⇒ Unknown ⇒ run)
        };
        let stamp = dorc_core::ProbeStamp::received(idx as u64, clock.now());
        match tag {
            // 24E §5: `deriv <leafid> coord=<coord>` — `coord=` is the FREE-CONTENT field
            // (`262` §2 last-to-token): after deframing it runs to end-of-line, whitespace
            // included (the incumbent whitespace-truncation bug is FIXED here). A malformed
            // leaf ⇒ drop (empty derived footprint ⇒ wall, kFAIL-safe).
            "deriv" => {
                if let Some((site, coord)) = split_key(rest, "coord=")
                    && let Some(site) = parse_leaf(site)
                {
                    out.derivations
                        .entry(site)
                        .or_default()
                        .push(coord.to_owned());
                }
            }
            // `262` §2 / `26A` stop-1: `deriv-end <leafid> n=<K>` — the at-most family close.
            // Records the declared count; the consumer refuses a family whose received count
            // ≠ K (or that never closed) ⇒ wall-total (never a shrunken at-most footprint).
            "deriv-end" => {
                let mut it = rest.split_whitespace();
                if let Some(site) = it.next().and_then(parse_leaf) {
                    for tok in it {
                        if let Some(n) = tok.strip_prefix("n=").and_then(|n| n.parse::<u32>().ok())
                        {
                            out.derivation_ends.insert(site, n);
                        }
                    }
                }
            }
            // 24F §3: `resolv <coord> canon=<canonical>` | `resolv <coord> dangling`. `canon=`
            // is the FREE-CONTENT field (last-to-token) so a space-bearing canonical survives.
            "resolv" => {
                if let Some((coord, tail)) = rest.split_once(' ') {
                    let outcome = if tail == "dangling" {
                        Some(ResolvOutcome::Dangling)
                    } else {
                        tail.strip_prefix("canon=")
                            .map(|c| ResolvOutcome::Canonical(c.to_owned()))
                    };
                    if let Some(o) = outcome {
                        out.resolutions.insert(coord.to_owned(), o);
                    }
                }
            }
            // 24G §4: `reach <coord> arm=<n> entity=<line>` — `entity=` is the FREE-CONTENT
            // field (last-to-token): a reached entity with embedded spaces now survives (the
            // single-token truncation is fixed — `279f` rider generalization).
            "reach" => {
                if let Some((head, entity)) = split_key(rest, "entity=") {
                    let mut coord: Option<&str> = None;
                    let mut arm: Option<usize> = None;
                    for (i, tok) in head.split_whitespace().enumerate() {
                        if i == 0 {
                            coord = Some(tok);
                        } else if let Some(n) =
                            tok.strip_prefix("arm=").and_then(|n| n.parse().ok())
                        {
                            arm = Some(n);
                        }
                    }
                    if let (Some(c), Some(a)) = (coord, arm) {
                        out.reaches
                            .entry((c.to_owned(), a))
                            .or_default()
                            .push(entity.to_owned());
                    }
                }
            }
            "site" => parse_site_record(rest, stamp, &mut out, interner),
            "report" => parse_report_record(rest, &mut out), // `27W` §2 tier-3 (decision-inert lane)
            _ => {} // unrecognized inner tag ⇒ drop (kFAIL-perform: no verdict ⇒ run)
        }
    }
    out
}

/// Converts only grammar-admitted records. The legacy string parser above is replay-only until 3C2.
fn parse_admitted_results(
    records: &dorc_plan::records::AdmittedUnscopedHostRecords,
    clock: &mut RunClock,
    interner: &mut Interner,
) -> SiteResults {
    let mut out = SiteResults {
        framed: true,
        ..SiteResults::default()
    };
    for (ordinal, record) in records.iter().enumerate() {
        match record {
            dorc_plan::records::AdmittedHostRecord::Site {
                key,
                effect,
                rc,
                stdout,
                stderr,
                ..
            } => {
                let Some(key) = parse_site_key(key) else {
                    continue;
                };
                let rec = SiteRecord {
                    verdict: effect_word_to_verdict(effect),
                    rc: Rc(rc),
                    stdout: stdout.map_or(Predicted::Top, |value| {
                        Predicted::Value(OutBytes(interner.intern(value)))
                    }),
                    stderr: stderr.map_or(Predicted::Top, |value| {
                        Predicted::Value(OutBytes(interner.intern(value)))
                    }),
                    conflicted: false,
                    stamp: dorc_core::ProbeStamp::received(
                        ordinal as u64,
                        clock.at(ordinal as u64),
                    ),
                };
                out.records
                    .entry(key)
                    .and_modify(|prior| *prior = meet_record(*prior, rec))
                    .or_insert(rec);
            }
            dorc_plan::records::AdmittedHostRecord::Derivation { site, coord } => {
                out.derivations
                    .entry(dorc_plan::LeafId(site))
                    .or_default()
                    .push(coord.to_owned());
            }
            dorc_plan::records::AdmittedHostRecord::DerivationEnd { site, count } => {
                out.derivation_ends.insert(dorc_plan::LeafId(site), count);
            }
            dorc_plan::records::AdmittedHostRecord::Resolution { coord, canonical } => {
                out.resolutions.insert(
                    coord.to_owned(),
                    canonical.map_or(ResolvOutcome::Dangling, |value| {
                        ResolvOutcome::Canonical(value.to_owned())
                    }),
                );
            }
            dorc_plan::records::AdmittedHostRecord::Reach { coord, arm, entity } => {
                out.reaches
                    .entry((coord.to_owned(), arm))
                    .or_default()
                    .push(entity.to_owned());
            }
            dorc_plan::records::AdmittedHostRecord::Report { body } => {
                parse_report_record(body, &mut out);
            }
        }
    }
    out
}

/// Ingest one report-lane record (`27W` §2 tier-3): `report [site=<key>] <verb> <class> <tail…>`.
/// Decision-inert. Noise-tolerant (`27W:rul-report-noise-tolerant`): the verb/class are recognized
/// best-effort, but an unrecognized token or free-form line is RETAINED (`recognized=false`), never
/// dropped, never an error. Deduped on the whole record — a tier-3 echo of an already-ingested line
/// adds nothing (the dedup the tier-2 static classification will later key by (site, arm, class)).
fn parse_report_record(rest: &str, out: &mut SiteResults) {
    let (site, body) = match rest.strip_prefix("site=") {
        Some(after) => {
            let (key_tok, tail) = after.split_once(' ').unwrap_or((after, ""));
            (parse_site_key(key_tok), tail)
        }
        None => (None, rest),
    };
    // v1 grammar: verb `decline` + a starter-set class; either unrecognized ⇒ degrade-generic.
    let mut words = body.split_whitespace();
    let verb = words.next();
    let class = words
        .next()
        .and_then(dorc_aid::narrative::DeclineClass::from_token);
    let recognized = verb == Some("decline") && class.is_some();
    let rec = ReportRecord {
        site,
        class,
        raw: sanitize_report_raw(body),
        recognized,
    };
    if !out.reports.contains(&rec) {
        out.reports.push(rec);
    }
}

/// Sanitize + size-cap a report-lane emission's raw text at ingestion (`27W` §2).
///
/// A thin delegation to the shared display seat: the lane keeps its own budget
/// ([`REPORT_RAW_CAP`]) and its own destination (a plain advisory line, which nothing measures),
/// while the encoding itself is one implementation shared with every other display route. NEVER a
/// decision input (decision-inert), and encoding grants the bytes no trust
/// (`sinv-hostile-sensitive-orthogonal`).
fn sanitize_report_raw(s: &str) -> String {
    dorc_aid::display::encode_line(s, REPORT_RAW_CAP)
}

/// Parse `u32` leaf-id.
#[cfg(test)]
fn parse_leaf(tok: &str) -> Option<dorc_plan::LeafId> {
    tok.parse::<u32>().ok().map(dorc_plan::LeafId)
}

/// Split a record body at a FREE-CONTENT `key=` into `(head, value)` where `value` runs to
/// end-of-line (whitespace included — `262` §2 last-to-token). The key must be preceded by a
/// space (or begin the body). Returns `None` when the key is absent.
#[cfg(test)]
fn split_key<'a>(body: &'a str, key: &str) -> Option<(&'a str, &'a str)> {
    if let Some(v) = body.strip_prefix(key) {
        return Some(("", v));
    }
    let pat = format!(" {key}");
    let at = body.find(&pat)?;
    Some((&body[..at], &body[at..][pat.len()..]))
}

/// Parse one `site <leafid> effect=<word> rc=<n> [stdout=<free-content>]` record (`262` §2).
/// `stdout=` is the FREE-CONTENT field (last-to-token) — the read-value lane's future carrier
/// (`279f` rider): it runs to end-of-line so embedded spaces survive byte-exactly. `stderr=`
/// stays single-token (stderr handling is out of spike scope — churn-avoidance-disclosure).
/// Unknown keys BEFORE the free-content field are ignored (additive-keys, `24Kc`). A duplicate
/// (site, member) record MERGES BY MEET, never last-wins (`262` §1 tie-break law).
#[cfg(test)]
fn parse_site_record(
    rest: &str,
    stamp: dorc_core::ProbeStamp,
    out: &mut SiteResults,
    interner: &mut Interner,
) {
    // `stdout=` is the trailing free-content field; everything from it runs to EOL.
    let (head, stdout) = match split_key(rest, "stdout=") {
        Some((h, v)) => (h, Predicted::Value(OutBytes(interner.intern(v)))),
        None => (rest, Predicted::Top),
    };
    let mut it = head.split_whitespace();
    let Some(key) = it.next().and_then(parse_site_key) else {
        return; // malformed site key ⇒ drop (⇒ Unknown ⇒ run)
    };
    let mut verdict = Verdict::Unknown;
    let mut rc = Rc(0);
    let mut stderr = Predicted::Top;
    for tok in it {
        if let Some(w) = tok.strip_prefix("effect=") {
            verdict = effect_word_to_verdict(w);
        } else if let Some(n) = tok.strip_prefix("rc=").and_then(|n| n.parse::<i32>().ok()) {
            rc = Rc(n);
        } else if let Some(t) = tok.strip_prefix("stderr=") {
            stderr = Predicted::Value(OutBytes(interner.intern(t)));
        }
    }
    let rec = SiteRecord {
        verdict,
        rc,
        stdout,
        stderr,
        conflicted: false,
        stamp,
    };
    out.records
        .entry(key)
        .and_modify(|prior| *prior = meet_record(*prior, rec))
        .or_insert(rec);
}

/// Meet two records reported for one (site, member) key (`262` §2 duplicate-by-meet / §1
/// tie-break law). Identical ⇒ idempotent (unchanged). ANY disagreement ⇒ can't-tell: verdict
/// ⊤ (⇒ run), out-claims ⊤, and `conflicted` set so the fold-usable Query rc is withheld
/// ([`facts_from_sites`]). NEVER first-wins/last-wins; commutative + idempotent, so arrival
/// order cannot change the fold (`262` §1 pin-fold-permutation).
fn meet_record(a: SiteRecord, b: SiteRecord) -> SiteRecord {
    let rc_conflict = a.rc != b.rc;
    SiteRecord {
        verdict: if a.verdict == b.verdict {
            a.verdict
        } else {
            Verdict::Unknown
        },
        rc: a.rc,
        stdout: if a.stdout == b.stdout {
            a.stdout
        } else {
            Predicted::Top
        },
        stderr: if a.stderr == b.stderr {
            a.stderr
        } else {
            Predicted::Top
        },
        conflicted: a.conflicted || b.conflicted || rc_conflict || a.verdict != b.verdict,
        stamp: a.stamp, // keep the first-seen stamp (C6): the meet is order-independent
    }
}

/// Parse a record's site key token (task-L2 item-4): `N` ⇒ `RecordKey { site: N, member:
/// None }`; `N.M` ⇒ `RecordKey { site: N, member: Some(M) }` (member `M` of an in-loop
/// Members family). Both `N` and `M` are `u32`; a non-numeric / malformed token ⇒ `None`
/// (the record is dropped ⇒ that cell folds to Unknown ⇒ run, the kFAIL-perform floor).
fn parse_site_key(tok: &str) -> Option<RecordKey> {
    match tok.split_once('.') {
        Some((leaf, member)) => Some(RecordKey {
            site: dorc_plan::LeafId(leaf.parse::<u32>().ok()?),
            member: Some(member.parse::<u32>().ok()?),
        }),
        None => Some(RecordKey {
            site: dorc_plan::LeafId(tok.parse::<u32>().ok()?),
            member: None,
        }),
    }
}

/// Map the probe's three-outcome `effect=` word to a [`Verdict`] (the probe-record
/// convention, 202 §3): `holds ⇒ Converged`, `absent ⇒ Diverged`,
/// anything else (`cant-tell` / garbled) ⇒ `Unknown` (the safe direction).
fn effect_word_to_verdict(word: &str) -> Verdict {
    match word {
        "holds" => Verdict::Converged,
        "absent" => Verdict::Diverged,
        _ => Verdict::Unknown,
    }
}

/// Emit the escalation-POLICY disclosure (`27C:render-authority-disclosure` — the consent-legibility
/// line). Names the escalation posture the dial + capability set, and the entry-capable wrappers
/// loaded (a wrapper authoring BOTH a peeling `__predict` and an `__enter` form). One `Note` to
/// stderr (advisory), never a gate.
///
/// Emit the unloaded-sibling-oracle hint (`AID-NEEDS:aid-unloaded-sibling-oracle`, gap-5 / `24H`
/// ack-6): scan the directories of the loaded oracles + the book(s) for `*.oracle.sh` files that were
/// NOT loaded, and disclose them (suggest, never auto-load). A cli-edge disclosure — it reads the
/// filesystem, so it lives here, never in the kernel; the `read_dir` order is OS-dependent, so the
/// result is SORTED (`inv-determinism` at the edge). The payload's `detail` carries the DATA (the
/// sorted backtick-quoted path list); the user-facing framing prose stays `[unwritten:]` for the
/// conductor (`27V:rul-error-authorship-tier` — the builder authors no user-facing prose).
/// The comparison key that lets a LOADED oracle path and a DISCOVERED one denote the same file.
///
/// `289:rider-sibling-note-false-fires-relative`: the loaded set carries `-o` args verbatim
/// (`firewall.oracle.sh`) while discovery yields `read_dir` paths (`./firewall.oracle.sh`), so a raw
/// string compare reported every relatively-named oracle as unloaded. Both sides now spell an empty
/// parent as `.` and separators as `/`, so the two forms of one bare filename converge.
///
/// Deliberately textual, not `canonicalize`: this feeds a HINT, and a hint must not acquire the
/// power to touch the filesystem or to fail. Two spellings of one path through different symlinks
/// still miss, which costs a suppressed hint and never a wrong one.
fn oracle_path_key(path: &str) -> String {
    let path = std::path::Path::new(path);
    let parent = path
        .parent()
        .filter(|d| !d.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let name = path.file_name().unwrap_or_default();
    parent.join(name).to_string_lossy().replace('\\', "/")
}

fn emit_unloaded_sibling_oracles(advisory: bool, books: &[String], oracle_paths: &[String]) {
    use std::path::Path;
    // Normalize `\` → `/` before comparing: `read_dir` yields platform-separator paths (backslash on
    // Windows) while the loaded set carries the `-o` args verbatim (forward slash), so a raw string
    // compare would miss every loaded oracle on Windows and falsely report it unloaded.
    let norm = |p: &str| p.replace('\\', "/");
    let loaded: BTreeSet<String> = oracle_paths.iter().map(|p| oracle_path_key(p)).collect();
    let mut dirs: BTreeSet<std::path::PathBuf> = BTreeSet::new();
    for p in oracle_paths.iter().chain(books.iter()) {
        if let Some(parent) = Path::new(p).parent() {
            // An empty parent (a bare filename) means the current directory.
            let dir = if parent.as_os_str().is_empty() {
                Path::new(".").to_path_buf()
            } else {
                parent.to_path_buf()
            };
            dirs.insert(dir);
        }
    }
    let mut unloaded: Vec<String> = Vec::new();
    for dir in &dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let shown = norm(&entry.path().to_string_lossy());
            let key = oracle_path_key(&shown);
            if shown.ends_with(".oracle.sh") && !loaded.contains(&key) && !unloaded.contains(&shown)
            {
                unloaded.push(shown);
            }
        }
    }
    if unloaded.is_empty() {
        return;
    }
    unloaded.sort();
    let detail = unloaded
        .iter()
        .map(|p| format!("`{p}`"))
        .collect::<Vec<_>>()
        .join(", ");
    report_at(
        advisory,
        "oracle",
        None,
        &[Diag::new_spanless_site(DiagCode::AidUnloadedSiblingOracle(
            AidUnloadedSiblingOracle { detail },
        ))],
    );
}

/// SCOPE (honest for the spike): this is the POLICY in effect, not a per-book-SITE "will enter"
/// tally — the book-side entry-composed probe emission (which would count sites per entered context)
/// is the deferred integration (`27K` §9 / this lane's report). The dial × capability × the loaded
/// entry forms are all real; what is missing is the per-site consumption in the probe pipeline.
fn emit_escalation_policy(
    advisory: bool,
    interner: &mut Interner,
    oracle_refs: &[&str],
    dial: dorc_core::EscalationDial,
    capability: dorc_core::Capability,
) {
    use dorc_oracle::entry::{detect_entry_form, lift_entry_set};
    use dorc_oracle::predict::lift_predicts;

    // Entry-capable wrappers: a provider authoring an `__enter` form (whose predict also peels).
    let mut heads: BTreeMap<Symbol, String> = BTreeMap::new();
    for src in oracle_refs {
        let peels: BTreeSet<Symbol> = {
            let ps = lift_predicts(interner, src).value;
            ps.providers()
                .filter(|p| ps.get(*p).is_some_and(detect_peel_present))
                .collect()
        };
        let es = lift_entry_set(interner, src).value;
        for p in es.providers() {
            if peels.contains(&p)
                && let Some(form) = es.get(p).and_then(detect_entry_form)
            {
                heads.entry(p).or_insert_with(|| form.head.join(" "));
            }
        }
    }
    if heads.is_empty() {
        return; // no entry-capable wrapper loaded ⇒ no escalation is possible ⇒ nothing to disclose
    }
    let head_list = heads.values().cloned().collect::<Vec<_>>().join(", ");
    let cap = match capability {
        dorc_core::Capability::Root => "root",
        dorc_core::Capability::NonRootNopasswd => "non-root (NOPASSWD)",
        dorc_core::Capability::Degraded => "degraded",
    };
    let msg = match dial {
        dorc_core::EscalationDial::NoEscalation => format!(
            "escalation policy: NO oracle code will context-shift (--no-probe-escalation); \
             wrapped sites run/guard. Entry-capable wrappers loaded: {head_list}."
        ),
        dorc_core::EscalationDial::VouchedOnly => format!(
            "escalation policy: probe re-uses connection authority ({cap}) for \
             `tolerates:`-vouched functions only (default); entry forms: {head_list}. \
             Forbid with --no-probe-escalation; widen with --escalate-any-probe."
        ),
        dorc_core::EscalationDial::AnyProbe => format!(
            "escalation policy: probe re-uses connection authority ({cap}) for ALL oracles \
             (--escalate-any-probe overrides absent author consent); entry forms: {head_list}."
        ),
    };
    report_at(
        advisory,
        "escalation",
        None,
        &[Diag::new_spanless_site(DiagCode::EscalationPolicy(
            EscalationPolicy { detail: msg },
        ))],
    );
}

/// Whether a predict body peels (a wrapper) — the `detect_peel`-present predicate, factored so the
/// entry-policy scan reuses it (`inv-referent-agnostic`: structural, never decodes the command).
fn detect_peel_present(p: &dorc_oracle::predict::Predict) -> bool {
    dorc_oracle::wrapper::detect_peel(p).is_some()
}

/// The lane-integration `27N` product of the wrapped-BOOK-site analysis: the peel-map (for
/// `classify` to birth each wrapped fact in its context), the wrapped-probe decisions (for
/// `compile_probe`), and the adoption/disclosure hints.
struct WrappedAnalysis {
    /// Wrapped sites keyed by [`CfgNodeId`] → (inner argv, composed context) for `classify`.
    peeled: BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_analysis::effect::PeeledSite>,
    /// Wrapped-probe dispositions (Enter / Degrade) keyed by node, for `compile_probe`.
    wrapped: dorc_plan::WrappedProbes,
    /// One-line adoption hints (a degraded-on-vouch site) + degrade disclosures (`27C` §2/§6).
    hints: Vec<Diag>,
    /// Pure-predicate-carry attribution chains keyed by the carried site's [`AstId`] (`27C` §4(a)):
    /// the why-lens tether emitted for every carried elision (`emit_carry_attribution`). Keyed by
    /// `AstId` so the plan's per-site step re-keys to the site number for the `why: site N …` line.
    carried: BTreeMap<dorc_core::AstId, String>,
    /// C5 aid plane (`27V` Lane A): the decision-inert [`CollapseKind::EntryDenial`] narrative minted
    /// when a wrapped site's entry consent degrades to guard/run (`two-plane-aid-law`; steers
    /// nothing). Threaded to the why-lens seam by the cli edge (d4 renders).
    collapse_narrative: Vec<CollapseNarrative>,
}

/// Build the wrapped-BOOK-site analysis (`27C` §3 / lane-integration `27N`): recognize each site
/// whose head is a loaded wrapper, peel it into (inner command, composed context), decide entry
/// (dial × capability × vouch × entry-form), and produce the peel-map + wrapped-probe decisions +
/// hints. Empty when no wrapper oracle is loaded ⇒ the pipeline is byte-identical
/// (`empty-world-byte-identical`). The entry-composed probe ships ONLY oracle bytes
/// (`271:rul-only-oracle-bytes-ship`); the admin's argv flows through the inner oracle's argparse.
#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "the wrapped-site analysis threads the whole compiled context (oracle sources + predict/verdict sets + cfg/value) plus the two admin axes (dial/capability); the per-site peel→resolve-inner→decide loop is one cohesive unit (`27N`), its sub-steps already extracted to build_wrapper_index + resolve_inner_check"
)]
fn build_wrapped_analysis(
    oracle_srcs: &[String],
    oracle_refs: &[&str],
    oracle_paths: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    ast: &dorc_syntax::ast::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    dial: dorc_core::EscalationDial,
    capability: dorc_core::Capability,
    interner: &mut Interner,
) -> WrappedAnalysis {
    use dorc_aid::narrative::EntryDegradeTag;
    use dorc_analysis::cfg::{CfgNodeId, CfgNodeKind};
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::entry::{
        EntryDecision, EntryDegrade, adoption_hint, decide_entry, peel_book_chain,
    };
    use dorc_oracle::predict::map_provider_name;

    let WrapperIndexBundle {
        wrappers,
        enter_defs,
        tolerance,
    } = build_wrapper_index(oracle_refs, verdict_sets, interner);

    let mut out = WrappedAnalysis {
        peeled: BTreeMap::new(),
        wrapped: dorc_plan::WrappedProbes::new(),
        hints: Vec::new(),
        carried: BTreeMap::new(),
        collapse_narrative: Vec::new(),
    };
    if wrappers.is_empty() {
        return out; // no wrapper oracle ⇒ nothing peels (rung-0 byte-identical)
    }

    // (A) the authored axis-invariance index (`27C` §4(a) pure-predicate carry) — lifted once from
    // every `state_stored_only_in()` body; its netns-caveat contradictions surface as hints. Empty
    // when no invariance line is declared ⇒ carry never licenses (`silence-licenses-nothing`).
    let (invariance, inv_diags) = dorc_oracle::carry::InvarianceIndex::lift(interner, oracle_refs);
    out.hints.extend(inv_diags);

    let command_nodes: Vec<CfgNodeId> = cfg
        .iter()
        .filter(|(id, n)| {
            n.kind == CfgNodeKind::Command
                && !cfg.is_expansion_internal(*id)
                && !cfg.is_spliced_internal(*id)
        })
        .map(|(id, _)| id)
        .collect();
    for node in command_nodes {
        // Resolve the site's whole argv to literals; a ⊤ word ⇒ not peelable (walls opaquely).
        let argv = value.argv_values(node);
        let mut argv_strs: Vec<String> = Vec::with_capacity(argv.len());
        for w in &argv {
            match w {
                ValueOf::Literal(s) => argv_strs.push(interner.resolve(*s).to_owned()),
                ValueOf::Top(_) => {
                    argv_strs.clear();
                    break;
                }
            }
        }
        let argv_refs: Vec<&str> = argv_strs.iter().map(String::as_str).collect();
        let Some(chain) = peel_book_chain(&argv_refs, &wrappers) else {
            continue; // not a wrapped site (or a wrapper that cannot peel ⇒ walls)
        };
        let Some((inner_word, inner_rest)) = chain.inner_argv.split_first() else {
            continue;
        };
        let context = chain.composed.to_context(interner);
        let inner_provider = interner.intern(&map_provider_name(inner_word));
        let inner_operands: Vec<Symbol> = inner_rest.iter().map(|a| interner.intern(a)).collect();
        let mut peeled_argv = vec![ValueOf::Literal(inner_provider)];
        peeled_argv.extend(inner_operands.iter().map(|s| ValueOf::Literal(*s)));
        // The inner check body (predict first, else the auto-cell verdict body) — mirrors the ambient
        // shape `compile_probe` would ship, now composed inside the entry chain.
        let Some((inner_fn, inner_sh)) = resolve_inner_check(
            oracle_srcs,
            checks,
            verdict_sets,
            inner_word,
            inner_provider,
            &inner_operands,
            interner,
        ) else {
            // No inner check ⇒ run; the fact is still born in-context for classify.
            out.peeled.insert(
                node,
                dorc_analysis::effect::PeeledSite {
                    inner_argv: peeled_argv,
                    context,
                },
            );
            out.wrapped.insert(node, dorc_plan::WrappedProbe::Degrade);
            continue;
        };
        let composed_enter_defs: Vec<(String, String)> = chain
            .links
            .iter()
            .filter_map(|l| l.entry.as_ref().and(enter_defs.get(&l.provider)).cloned())
            .collect();
        let composed = dorc_plan::EntryComposed {
            enter_defs: composed_enter_defs,
            inner_fn,
            inner_sh,
            inner_argv: inner_operands,
        };
        // An identity chain (HostDefault) needs NO entry — it ships the plain inner check in the
        // ambient world. A shifted chain runs the two-axis consent decision (`27C` §1).
        let decision = if context == dorc_core::Context::HostDefault {
            EntryDecision::Enter
        } else {
            let has_entry_form = chain.links.iter().all(|l| l.entry.is_some());
            let tolerated = tolerance
                .get(&inner_provider)
                .map(|t| t.tolerated_on_path(inner_rest.first().map(String::as_str)))
                .unwrap_or_default();
            decide_entry(
                has_entry_form,
                capability,
                dial,
                &chain.composed.crossed(),
                &chain.composed.walls(),
                &tolerated,
            )
        };
        // The fact's context: Wrapped for Enter/Degrade (born in-context); HostDefault for a
        // pure-predicate CARRY (measure ambient, carry across the substrate boundary, `27C` §4(a)).
        let (fact_context, probe) = match decision {
            EntryDecision::Enter => (
                context,
                dorc_plan::WrappedProbe::Enter {
                    provider: inner_provider,
                    composed,
                },
            ),
            EntryDecision::Degrade(reason) => {
                // Try pure-predicate carry (`27C` §4(a)) before defaulting to run. Gated on the
                // shipped inner check BEING the verdict body (auto-cell) — the closed body must be
                // the measured body; the predict-inner carry path is deferred (disclosed, `27O`).
                let carried = if composed.inner_fn.ends_with("__is_converged") {
                    try_carry(&chain, inner_provider, verdict_sets, &invariance)
                } else {
                    None
                };
                if let Some(read_kinds) = carried {
                    // Attribution chain (`27C` §9: every cross-context elision renders it from day
                    // one): the crossed substrate axes; each backing kind's owner `invariant:<axis>`
                    // line (vouch-species); the engine read-set-closure proof. One note per site,
                    // deterministic. Rides the diagnostic + why lanes only (two-surfaces: never the
                    // `.sh` artifact).
                    let span = ast.node(cfg.node(node).ast).span;
                    // render 3/3 (`27C` §9): each carried kind's owner `invariant:<axis>` line as
                    // `file:line` (first crossed axis with a threaded span wins; absent ⇒ no locus).
                    let loci: BTreeMap<String, String> = read_kinds
                        .iter()
                        .filter_map(|k| {
                            chain
                                .composed
                                .crossed()
                                .iter()
                                .find_map(|d| invariance.invariant_span(k, *d))
                                .and_then(|sp| oracle_locus(Some(sp), oracle_paths, oracle_srcs))
                                .map(|loc| (k.clone(), loc))
                        })
                        .collect();
                    let text =
                        carry_attribution_text(&chain.composed.crossed(), &read_kinds, &loci);
                    out.hints.push(Diag::new(
                        DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                            detail: text.clone(),
                        }),
                        span,
                    ));
                    out.carried.insert(cfg.node(node).ast, text);
                    (
                        dorc_core::Context::HostDefault,
                        dorc_plan::WrappedProbe::Carry {
                            provider: inner_provider,
                            composed: dorc_plan::EntryComposed {
                                enter_defs: Vec::new(), // ambient: no entry form
                                ..composed
                            },
                        },
                    )
                } else {
                    // C5 aid: narrate the STATIC entry-degrade rung (`27C` §3; Consented-tier).
                    let rung = match reason {
                        EntryDegrade::NoCapability(_) => Some(EntryDegradeTag::NoCapability),
                        EntryDegrade::DialForbids => Some(EntryDegradeTag::DialForbids),
                        EntryDegrade::Unvouched(_) => Some(EntryDegradeTag::Unvouched),
                        EntryDegrade::TopDimension(_) => Some(EntryDegradeTag::TopDimension),
                        EntryDegrade::NoEntryForm => Some(EntryDegradeTag::NoEntryForm),
                        EntryDegrade::RuntimeEntryFailure => None,
                    };
                    if let Some(rung) = rung {
                        out.collapse_narrative.push(CollapseNarrative::new(
                            SpeechAct::Consented,
                            CollapseKind::EntryDenial { rung },
                        ));
                    }
                    if let EntryDegrade::Unvouched(dim) = reason {
                        out.hints.push(Diag::new(
                            DiagCode::WrappedSiteAdoptionHint(WrappedSiteAdoptionHint {
                                detail: adoption_hint(inner_word, dim),
                            }),
                            ast.node(cfg.node(node).ast).span,
                        ));
                    }
                    (context, dorc_plan::WrappedProbe::Degrade)
                }
            }
        };
        out.peeled.insert(
            node,
            dorc_analysis::effect::PeeledSite {
                inner_argv: peeled_argv,
                context: fact_context,
            },
        );
        out.wrapped.insert(node, probe);
    }
    out
}

/// Try pure-predicate carry (`27C` §4(a); steering `pure-predicate-carry`) for a wrapped site whose
/// entry DEGRADED: does the inner verdict body's read-set close (B) across a SUBSTRATE boundary
/// whose backing kinds are authored-invariant (A)? Runs [`dorc_oracle::carry::read_set_closed`] over
/// the inner verdict body and [`dorc_oracle::carry::decide_carry`] over the chain's crossed
/// dimensions. `Some(read_kinds)` (the (A) attribution inputs) on carry; `None` when there is no
/// inner verdict body, or (A)/(B)/substrate-scope fails — the site then runs (fail safe: a missed
/// carry loses an elision, never carries a hidden read).
fn try_carry(
    chain: &dorc_oracle::entry::PeeledChain,
    inner_provider: Symbol,
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    invariance: &dorc_oracle::carry::InvarianceIndex,
) -> Option<BTreeSet<String>> {
    let verdict = verdict_sets
        .iter()
        .find_map(|set| set.get(inner_provider))?;
    let closure = dorc_oracle::carry::read_set_closed(verdict);
    match dorc_oracle::carry::decide_carry(&chain.composed.crossed(), &closure, invariance) {
        dorc_oracle::carry::CarryDecision::Carry { read_kinds } => Some(read_kinds),
        dorc_oracle::carry::CarryDecision::NoCarry(_) => None,
    }
}

/// Render the pure-predicate-carry attribution chain (`27C` §9: every cross-context elision renders
/// its four-link chain from day one). Names the crossed substrate axes, each marked backing kind
/// whose owner's `invariant:<axis>` line licensed the crossing (vouch-species — the kind-owner's
/// attributable claim), and the engine read-set-closure proof. Deterministic (sorted axes/kinds).
fn carry_attribution_text(
    crossed: &[dorc_oracle::wrapper::Dimension],
    read_kinds: &BTreeSet<String>,
    loci: &BTreeMap<String, String>,
) -> String {
    let axes = crossed
        .iter()
        .map(|d| d.as_token())
        .collect::<Vec<_>>()
        .join("+");
    // render 3/3: each kind names its owner's `invariant:` line as `file:line` when threaded.
    let kinds = read_kinds
        .iter()
        .map(|k| match loci.get(k) {
            Some(loc) => format!("{k} (invariant: line at {loc})"),
            None => k.clone(),
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "pure-predicate carry across {axes} (unflagged, 27C section 4(a)): {kinds} -- each vouched invariant \
         across {axes} by its kind-owner's `invariant:` line (vouch-species); the verdict body is \
         engine-proved read-set-closed"
    )
}

/// The lifted wrapper models, per-provider stripped `__enter` defs, and `tolerates:` vouches — the
/// wrapper-side inputs [`build_wrapped_analysis`] peels book sites against (`27N`).
struct WrapperIndexBundle {
    wrappers: dorc_oracle::entry::WrapperIndex,
    enter_defs: BTreeMap<Symbol, (String, String)>,
    tolerance: BTreeMap<Symbol, dorc_oracle::entry::ToleranceVouch>,
}

/// Build the [`WrapperIndexBundle`] from the loaded oracle sources (`27N`): every peeling `__predict`
/// (with its ρ, `__lend_map`, `__enter`) keyed by book word, the stripped `__enter` funcdefs, and
/// the per-provider `tolerates:` vouches (off the already-lifted verdict bodies, `27C` §2).
fn build_wrapper_index(
    oracle_refs: &[&str],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    interner: &mut Interner,
) -> WrapperIndexBundle {
    use dorc_oracle::entry::{
        WrapperIndex, WrapperModel, detect_entry_form, lift_entry_set, lift_tolerance,
    };
    use dorc_oracle::predict::{lift_predicts, map_provider_name};
    use dorc_oracle::wrapper::{derive_lend_map, detect_peel, lift_lend_map_set};

    let mut wrappers: WrapperIndex = WrapperIndex::new();
    let mut enter_defs: BTreeMap<Symbol, (String, String)> = BTreeMap::new();
    let mut tolerance: BTreeMap<Symbol, dorc_oracle::entry::ToleranceVouch> = BTreeMap::new();
    for src in oracle_refs {
        let ps = lift_predicts(interner, src).value;
        let ls = lift_lend_map_set(interner, src).value;
        let es = lift_entry_set(interner, src).value;
        for p in ps.providers() {
            let Some(predict) = ps.get(p) else { continue };
            let Some(peel) = detect_peel(predict) else {
                continue; // not a peeling wrapper
            };
            let word = interner.resolve(p).to_owned();
            let lend_map = ls.get(p).cloned();
            let lend = lend_map
                .as_ref()
                .map_or_else(Default::default, |lm| derive_lend_map(lm).0);
            let enter = es.get(p).and_then(detect_entry_form);
            if let Some(form) = es.get(p) {
                let stripped = dorc_oracle::predict::strip_enter(src, form, interner);
                let fname = format!(
                    "{}__enter",
                    dorc_oracle::to_funcname_segment(&map_provider_name(&word))
                );
                enter_defs.entry(p).or_insert((fname, stripped));
            }
            wrappers.entry(word).or_insert(WrapperModel {
                predict: predict.clone(),
                rho: peel.rho,
                lend,
                lend_map,
                enter,
                provider: p,
            });
        }
    }
    for vs in verdict_sets {
        for p in vs.providers() {
            if let Some(v) = vs.get(p) {
                let (vouch, _) = lift_tolerance(v);
                tolerance.entry(p).or_insert(vouch);
            }
        }
    }
    WrapperIndexBundle {
        wrappers,
        enter_defs,
        tolerance,
    }
}

/// Resolve the inner oracle's check for a wrapped site's entry-composed probe (`27N`): the `__predict`
/// body if the inner is a modeled command, else the auto-cell `__is_converged` verdict body (the
/// markless shape). `None` ⇒ no inner check ⇒ the site can't be probed ⇒ runs. Returns
/// `(mangled funcname, stripped funcdef)` — the funcname matches the strip's mangled name byte-for-byte.
fn resolve_inner_check(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    inner_word: &str,
    inner_provider: Symbol,
    inner_operands: &[Symbol],
    interner: &Interner,
) -> Option<(String, String)> {
    use dorc_oracle::predict::map_provider_name;
    let seg = dorc_oracle::to_funcname_segment(&map_provider_name(inner_word));
    if let Some(shipped) = ship_predict_body(
        oracle_srcs,
        checks,
        interner,
        inner_provider,
        inner_operands,
    ) {
        return Some((format!("{seg}__predict"), shipped.sh));
    }
    // Entry-composition is out of both the tier-3 drain scope and the span-threading scope this
    // round: the composed body has no single defining funcdef to name, so its site stays span-less.
    let shipped = ship_verdict_body(oracle_srcs, verdict_sets, interner, inner_provider)?;
    Some((format!("{seg}__is_converged"), shipped.sh))
}

fn report_at(advisory: bool, stage: &str, source: Option<(&str, &str)>, diags: &[Diag]) {
    report(stage, source, &advisory_filter(advisory, diags));
}

/// Report per-oracle-file diagnostics, each against its OWN `(path, src)` source, so a funcdef-keyed
/// diagnostic's caret frame resolves against the RIGHT oracle (`law-lineno-identity`: the file index
/// disambiguates the line-number space a bare span cannot). A file with no resolvable source falls to
/// the byte-offset fallback — never a wrong-file frame. Deterministic (`BTreeMap` key order).
fn report_by_oracle_file(
    advisory: bool,
    stage: &str,
    oracle_paths: &[String],
    oracle_srcs: &[String],
    diags_by_file: &BTreeMap<usize, Vec<Diag>>,
) {
    for (idx, diags) in diags_by_file {
        let source = oracle_paths
            .get(*idx)
            .map(String::as_str)
            .zip(oracle_srcs.get(*idx).map(String::as_str));
        report_at(advisory, stage, source, diags);
    }
}

/// The advisory severity-filter (rec-1 / tc-apply-receipt-floor), factored pure for
/// testing. `advisory` ⇒ pass every diagnostic through (the `plan`/round-trip render
/// surface); `!advisory` (the receipt-free `apply` off-ramp) ⇒ keep ONLY Error-severity,
/// dropping warnings + notes. Errors are NEVER dropped — the floor that keeps `apply`
/// honest while receipt-free. Returns owned clones (the call sites are cold — once per
/// pipeline stage — so the copy is irrelevant against the SSH-tunnel cost DESIGN floors on).
fn advisory_filter(advisory: bool, diags: &[Diag]) -> Vec<Diag> {
    if advisory {
        diags.to_vec()
    } else {
        diags
            .iter()
            .filter(|d| d.severity() == Severity::Error)
            .cloned()
            .collect()
    }
}

/// Print a stage's diagnostics to stderr (keeping stdout = probe + apply).
///
/// The TITLE line is `<stage>: <severity>[<code>]: <message-first-line>` — its shape is
/// load-bearing: the e2e gate-3 floor (20B §2) keys on `^<stage>: error[` (an Error fails a case
/// unless declared in `expected-diagnostics`; warnings stay free-form). Below it, ack-8 (round-24):
/// the rustc-style REGION FRAME — `--> file:line:col`, the source line in a gutter, and a `^^^`
/// caret ([`dorc_aid::diag::render_legacy_region`]) — replaces the old raw byte-offset `-->
/// <lo>:<hi>`, WHEN a `source` (`(filename, text)`) is threaded for this stage; a stage whose diags
/// span an ambiguous/combined source (or are spanless) keeps the byte-offset / no-region fallback.
/// The frame's `N |` gutter is the SOURCE line (rul24-lineno-identity: the number the user reads is
/// the number they type back as `:N`). None of the frame lines start with `<stage>: error[`, so
/// they stay inert to gate-3. Any folded ` = note:` continuations (a lowered `Diag`'s body) print
/// AFTER the region, so the caret frame lands rustc-style right beneath the title. I/O-edge only.
///
/// ack-5 color: the SEVERITY WORD is the severity/tier channel — red error / yellow warning / cyan
/// note — written through `anstream::stderr()`, an [`anstream::AutoStream`] that AUTO-STRIPS the
/// ANSI on a non-tty and honors `NO_COLOR` (+ enables Windows VT on a real console). Plain-when-
/// piped is load-bearing: the e2e harness captures stderr to a FILE ⇒ non-tty ⇒ the color vanishes,
/// so the gate-3/gate-7 needle-matching (and every golden) is byte-identical to the un-colored form.
fn report(stage: &str, source: Option<(&str, &str)>, diags: &[Diag]) {
    use std::io::Write as _;
    // params_of resolves no interned handle at HEAD, so a default interner suffices (`27V`).
    let interner = Interner::default();
    let mut w = anstream::stderr();
    for d in diags {
        let (word, style) = severity_style(d.severity());
        let (filename, src) = source.unwrap_or(("", ""));
        let rendered = dorc_aid::diag::render_staged_cli_parts(
            stage,
            &dorc_aid::catalog::CONST_CATALOG,
            d,
            src,
            filename,
            &interner,
        )
        .text();
        let prefix = format!("{stage}: {word}");
        // ANSI decoration stays outside the typed render bytes.
        let _ = match rendered.strip_prefix(&prefix) {
            Some(rest) => write!(w, "{stage}: {style}{word}{style:#}{rest}"),
            None => write!(w, "{rendered}"),
        };
    }
    let _ = w.flush();
}

/// The (severity word, [`anstyle::Style`]) for a diagnostic (ack-5 — color as the severity/tier
/// channel): red error, yellow warning, cyan note. The style is rendered to ANSI only on a tty
/// (the [`anstream::AutoStream`] in [`report`] strips it otherwise), so the word text is what the
/// e2e needle-matching sees.
fn severity_style(severity: Severity) -> (&'static str, anstyle::Style) {
    use anstyle::AnsiColor;
    match severity {
        Severity::Error => (
            "error",
            anstyle::Style::new()
                .fg_color(Some(AnsiColor::Red.into()))
                .bold(),
        ),
        Severity::Warning => (
            "warning",
            anstyle::Style::new().fg_color(Some(AnsiColor::Yellow.into())),
        ),
        Severity::Note => (
            "note",
            anstyle::Style::new().fg_color(Some(AnsiColor::Cyan.into())),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{EntityRef, FactKey, Interner, KindId, OpaqueToken, SelectorId};
    use dorc_plan::{LeafId, ProbePlan, ProbePredict, ProbeSiteKind};

    /// Route an unframed record string through the PRODUCTION deframer (legacy path) into the
    /// inner parser — the exact pipeline the round-trip uses. Unframed input exercises the
    /// legacy passthrough; the framed contract is pinned separately (deframe unit tests + DST).
    fn parse_str(input: &str, interner: &mut Interner) -> SiteResults {
        let expect = dorc_plan::records::Framing::spike(String::new()).expect();
        let d =
            dorc_plan::records::deframe(input, &expect, dorc_plan::records::LegacyPolicy::Tolerate);
        parse_results(&d.records, d.framed, &mut RunClock::Absent, interner)
    }

    /// The two destination answers that do not depend on the environment. `--no-whylog` must win
    /// over an explicitly named directory: a refusal the admin typed is the one instruction in this
    /// family that nothing may override (`28D:pay-levers-are-subtractive` — the levers only ever
    /// REMOVE, and a subtractive control that a sibling flag can defeat is not one).
    #[test]
    fn a_refusal_beats_a_named_directory() {
        let args = |argv: &[&str]| match parse_args_from(
            argv.iter().map(|word| (*word).to_owned()).collect(),
        )
        .expect("invocation parses")
        {
            Invocation::Analyze(parsed) => parsed,
            other => panic!("expected an analysis invocation, got {other:?}"),
        };
        assert_eq!(
            durable_destination(&args(&["plan", "book.sh", "--whylog-dir=logs"])).as_deref(),
            Some("logs"),
            "a named directory is used as given"
        );
        assert_eq!(
            durable_destination(&args(&[
                "plan",
                "book.sh",
                "--whylog-dir=logs",
                "--no-whylog"
            ])),
            None,
            "and is still refused when the admin says no receipt"
        );
    }

    /// `289:rider-sibling-note-false-fires-relative`: the loaded `-o` spelling and the `read_dir`
    /// spelling of ONE file must key alike, or the unloaded-sibling hint accuses every relatively
    /// named oracle of being unloaded. The bare-name/dot-slash pair is the exact shape every
    /// in-corpus case drives (`-o firewall.oracle.sh` against a `read_dir(".")` walk), so it is the
    /// pair worth pinning; the sub-directory rows guard against a fix that only special-cases `.`.
    #[test]
    fn loaded_and_discovered_oracle_spellings_share_one_key() {
        assert_eq!(
            oracle_path_key("firewall.oracle.sh"),
            oracle_path_key("./firewall.oracle.sh"),
            "a bare -o name and its read_dir(\".\") path are one file"
        );
        assert_eq!(
            oracle_path_key("oracles/fw.oracle.sh"),
            oracle_path_key("oracles\\fw.oracle.sh"),
            "separators normalize, so a Windows walk matches a forward-slash arg"
        );
        assert_ne!(
            oracle_path_key("a/fw.oracle.sh"),
            oracle_path_key("b/fw.oracle.sh"),
            "same basename in different dirs stays distinct — the hint must still fire"
        );
    }

    /// ack-2 / rul24-lineno-identity: the `dorc why` address parser reads a SOURCE line-number from
    /// `book.sh:N` or bare `N` (the tail after the last `:` when numeric), so a `file:N` the report
    /// PRINTS round-trips to the `N` a query ACCEPTS. A non-numeric tail ⇒ `None` ⇒ content-match.
    #[test]
    fn why_address_parses_line_number_or_falls_to_content() {
        assert_eq!(parse_line_address("book.sh:12"), Some(12), "path:N ⇒ N");
        assert_eq!(parse_line_address("12"), Some(12), "bare N ⇒ N");
        assert_eq!(
            parse_line_address("/abs/path/book.sh:3"),
            Some(3),
            "abs path:N ⇒ N"
        );
        assert_eq!(
            parse_line_address("apt-get"),
            None,
            "non-numeric ⇒ content match"
        );
        assert_eq!(
            parse_line_address("make install"),
            None,
            "content with a space ⇒ content match"
        );
    }

    /// A file-QUALIFIED address must name the book this run analyzed. The render prints qualified
    /// pointers now, so the un-checked reading answers for the analyzed book whatever file the
    /// address named — a silent wrong surface at rc 0, which is the failure this pins shut.
    /// Path-shape tolerance is deliberate: a pasted `./web.sh:9` is the same address as `web.sh:9`.
    #[test]
    fn a_file_qualified_address_must_name_the_analyzed_book() {
        assert!(address_names_book("web.sh:9", "web.sh"));
        assert!(address_names_book("9", "web.sh"), "a bare N names no file");
        assert!(address_names_book("./web.sh:9", "web.sh"), "leading ./");
        assert!(
            address_names_book("/srv/books/web.sh:9", "web.sh"),
            "an absolute path still resolves to the same book"
        );
        assert!(
            address_names_book("web.sh:9", "books\\web.sh"),
            "a windows-separated book path compares on its last component"
        );
        assert!(
            !address_names_book("other.sh:9", "web.sh"),
            "a DIFFERENT book must not silently answer for this one"
        );
    }

    /// cheap-7: the firehose-suppression classifier. Assignments and pure/no-target-state builtins
    /// (the engine's own list) are structurally-unprobeable ⇒ suppressed; a real un-oracled command
    /// is NOT ⇒ it survives into the aggregate disclosure / the `dorc why` problem set.
    #[test]
    fn structurally_unprobeable_suppresses_assignments_and_pure_builtins() {
        assert!(
            is_structurally_unprobeable("pkg=nginx"),
            "a bare assignment"
        );
        assert!(is_structurally_unprobeable("set -eu"), "the `set` builtin");
        assert!(is_structurally_unprobeable(": harmless"), "the `:` builtin");
        assert!(
            is_structurally_unprobeable("echo hello"),
            "the `echo` builtin"
        );
        assert!(is_structurally_unprobeable("cd /tmp"), "the `cd` builtin");
        assert!(
            !is_structurally_unprobeable("make install"),
            "a real un-oracled command is NOT inert"
        );
        assert!(
            !is_structurally_unprobeable("apt-get install -y nginx"),
            "a real mutator is NOT inert (it is a genuine unprobed run worth disclosing)"
        );
        // A word that merely CONTAINS `=` past a non-name char is not an assignment.
        assert!(
            !is_structurally_unprobeable("./configure --prefix=/usr"),
            "a flag `=` is not an assignment"
        );
    }

    fn pkg(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("installed")),
            context: dorc_core::Context::HostDefault,
        }
    }

    /// `289:rul-touches-mismatch-own-lane` — the synthesized per-arm wrapper takes its ROLE segment
    /// from the shared suffix constant, so the emitted sh namespace cannot drift from the role's
    /// real spelling the way the derivation lane's `__touches`/`__disturbs` pair did. Kind-munge and
    /// arm index ride around it unchanged.
    #[test]
    fn reach_arm_wrapper_name_carries_the_role_suffix() {
        assert_eq!(
            reach_arm_fn_name("sm.dorc.Package", 0),
            "sm_dorc_Package__disturbance_reaches_only_0"
        );
        assert!(
            reach_arm_fn_name("package", 3)
                .contains(dorc_oracle::reaches::DISTURBANCE_REACHES_ONLY_SUFFIX),
            "the wrapper name is built from the shared constant, never a literal"
        );
    }

    /// 24F §3 / corr-kind-keying §10: the resolver confusability enforcement. A clean single
    /// `<kind>.resolve()` is resolver-bearing; two files declaring ONE kind's resolver REFUSE both
    /// (the kind keeps token-equality — never first-wins-silently); a resolver keyed to a known
    /// PROVIDER name is KEPT but flagged (the mis-key). Behaviour-pinned (the diagnostics themselves
    /// are verified end-to-end via the cli binary).
    #[test]
    fn resolver_confusability_conflict_refuses_both_collision_keeps() {
        let mut i = Interner::default();
        // A provider "apt-get" exists (a lifted disturbs provider) — a resolver whose kind munges to
        // it collides (in NAME space, `flag-forward-munge-keying`).
        let touches_src = "apt_get__disturbs() { printf 'package:%s\\n' \"$1\"; }";
        let touches_paired = vec![(
            touches_src,
            dorc_oracle::touches::TouchesSet::lift(&mut i, touches_src).value,
        )];
        let checks: Vec<dorc_oracle::predict::PredictSet> = vec![];

        // The RAW coordinate kinds present — a resolver is "bearing" only if a coord of its kind
        // exists (flag-forward-munge-keying: the map is re-keyed from the munged base to raw kinds).
        let coord_kinds: BTreeSet<Symbol> = [i.intern("package"), i.intern("apt_get")]
            .into_iter()
            .collect();

        // A clean single package resolver ⇒ resolver-bearing (kind-keyed by the munged base).
        let clean = vec!["package__resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr = build_kind_resolvers(
            &clean,
            &["clean.oracle.sh".to_string()],
            &checks,
            &touches_paired,
            &coord_kinds,
            &mut i,
            false,
        );
        assert!(
            kr.resolver_kinds().any(|k| i.resolve(k) == "package"),
            "a clean package resolver is resolver-bearing"
        );

        // Two files, both package resolvers ⇒ BOTH refused (no resolver kind).
        let dup = vec![
            "package__resolve() { printf '%s\\n' \"$1\"; }".to_string(),
            "package__resolve() { printf '%s\\n' \"$1\"; }".to_string(),
        ];
        let kr_dup = build_kind_resolvers(
            &dup,
            &["a.oracle.sh".to_string(), "b.oracle.sh".to_string()],
            &checks,
            &touches_paired,
            &coord_kinds,
            &mut i,
            false,
        );
        assert_eq!(
            kr_dup.resolver_kinds().count(),
            0,
            "a duplicate resolver for one kind refuses BOTH (token-equality floor)"
        );

        // A resolver whose kind munges to the known provider "apt-get" (base `apt_get`) ⇒ KEPT
        // (warned, not a silent dud) — the collision is now detected in NAME space, and a raw
        // `apt_get` coord kind re-keys it as bearing.
        let collide = vec!["apt_get__resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr_col = build_kind_resolvers(
            &collide,
            &["collide.oracle.sh".to_string()],
            &checks,
            &touches_paired,
            &coord_kinds,
            &mut i,
            false,
        );
        assert!(
            kr_col.resolver_kinds().any(|k| i.resolve(k) == "apt_get"),
            "a provider-named resolver is kept (the collision is a warning, not a refusal)"
        );
    }

    fn tool(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("present")),
            context: dorc_core::Context::HostDefault,
        }
    }

    /// pin-partial-deriv-demotes-to-wall (`262` §2 / `26A` stop-1 — THE safety inversion): a
    /// deriv family is an AT-MOST claim; a mid-stream cut (byte tier) SHRINKS it, which would
    /// license MORE survivals (the under-execution cardinal sin). So an incomplete family MUST
    /// fold WALL-TOTAL (no footprint), never a shrunken footprint. Driven through the PRODUCTION
    /// deframer: a framed deriv stream is cut, deframed, parsed, and merged — asserting the
    /// site's footprint is refused (walls) on any cut, and present only when complete.
    #[test]
    fn pin_partial_deriv_family_demotes_to_wall_total() {
        use dorc_analysis::cfg::CfgNodeId;
        use dorc_plan::records::{DEFAULT_NONCE, TERMINAL_TOKEN};

        // A framed deriv stream for site 5: `n` coord records + an OPTIONAL `deriv-end N n=<end>`.
        let framed = |coords: usize, end: Option<usize>| -> String {
            let coord_recs = (0..coords)
                .map(|c| format!("{DEFAULT_NONCE} deriv 5 coord=package:pkg{c} {TERMINAL_TOKEN}\n"))
                .collect::<Vec<_>>()
                .concat();
            let end_rec = end.map_or(String::new(), |n| {
                format!("{DEFAULT_NONCE} deriv-end 5 n={n} {TERMINAL_TOKEN}\n")
            });
            format!(
                "dorc-records/1 nonce={DEFAULT_NONCE} attempt=1 host=localhost book=bk sites=0 {TERMINAL_TOKEN}\n\
                 {coord_recs}{end_rec}dorc-records-end/1 nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n"
            )
        };

        let merged_contains = |stream: &str, i: &mut Interner| -> bool {
            let d = dorc_plan::records::deframe(
                stream,
                &dorc_plan::records::Framing::spike("bk".to_owned()).expect(),
                dorc_plan::records::LegacyPolicy::Refuse,
            );
            let results = parse_results(&d.records, d.framed, &mut RunClock::Absent, i);
            let derivations = dorc_plan::DerivationPlan {
                derivations: vec![dorc_plan::ProbeDerivation {
                    site: LeafId(5),
                    node: CfgNodeId(5),
                    provider: i.intern("apt-get"),
                    argv: vec![],
                    sh: "apt_get__disturbs() { :; }".to_string(),
                    call: "apt-manifest".to_string(),
                }],
            };
            let mut fps = dorc_plan::TrustedFootprints::new();
            let node_spans = BTreeMap::from([(
                CfgNodeId(5),
                dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
            )]);
            merge_derived_footprints(
                &mut fps,
                &derivations,
                &results,
                &[],
                &BTreeMap::new(),
                &node_spans,
                i,
                None,
                false,
            );
            fps.contains(CfgNodeId(5))
        };

        let mut i = Interner::default();
        // COMPLETE (2 coords, deriv-end n=2) ⇒ footprint lifted (the site can spare).
        assert!(
            merged_contains(&framed(2, Some(2)), &mut i),
            "a complete family lifts its footprint"
        );
        // CUT: coord dropped mid-stream (only 1 coord, but the family declared n=2) ⇒ WALL.
        assert!(
            !merged_contains(&framed(1, Some(2)), &mut i),
            "count mismatch ⇒ wall-total, never a shrunken footprint"
        );
        // CUT: the deriv-end close-record itself lost ⇒ the family never closed ⇒ WALL.
        assert!(
            !merged_contains(&framed(2, None), &mut i),
            "no deriv-end ⇒ wall-total"
        );
        // EMPTY-but-complete: the shipped body printed nothing (a genuinely absent oracle 127s
        // under PATH=mocks-only; so does a def↔invocation name disagreement). Silence is NOT an
        // empty at-most claim — an empty footprint would spare EVERYTHING. It must wall.
        assert!(
            !merged_contains(&framed(0, Some(0)), &mut i),
            "an empty family walls — the engine never manufactures a claim from silence"
        );
    }

    /// A no-member record key (the common single-fact site, `site N`).
    fn rk(n: u32) -> RecordKey {
        RecordKey {
            site: LeafId(n),
            member: None,
        }
    }

    /// A one-check probe over `fact` with the given site-kind (the firewall input).
    fn probe1(fact: FactKey, site_kind: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![ProbePredict {
                site: LeafId(0),
                member: None,
                fact,
                site_kind,
                provider: fact.kind.0,
                argv: vec![],
                sh: "{ :; }".to_string(),
                defining_span: None,
                connected: None,
                verdict: false,
                emits_report: false,
                entry: None,
            }],
            unresolvable: vec![],
        }
    }

    #[test]
    fn report_lane_ingests_recognized_declines_and_retains_noise() {
        use dorc_aid::narrative::DeclineClass;
        let mut i = Interner::default();
        let r = parse_str(
            "report site=5 decline unsound vm.drop_caches is a write-only trigger key\n\
             report decline bogusclass some free text\n\
             report totally freeform author noise\n\
             report site=5 decline unsound vm.drop_caches is a write-only trigger key\n",
            &mut i,
        );
        assert_eq!(r.reports.len(), 3, "the exact duplicate is deduped");
        assert!(
            r.reports[0].recognized
                && r.reports[0].class == Some(DeclineClass::Unsound)
                && r.reports[0].site == Some(rk(5)),
            "a recognized decline is classed + site-keyed: {:?}",
            r.reports[0]
        );
        assert!(
            !r.reports[1].recognized && r.reports[1].class.is_none(),
            "an unknown class ⇒ degrade-generic, retained (never dropped)"
        );
        assert!(
            !r.reports[2].recognized,
            "a free-form line (no `decline` verb) is retained, never dropped"
        );
    }

    #[test]
    fn pairing_folds_a_runtime_record_into_its_site_decline_and_tier2_wins() {
        // C3 (`27W` §3): a runtime record classes a site's previously-unread decline (static wins
        // on a populated reason; a non-matching site is untouched).
        use dorc_aid::narrative::{
            AuthoredReason, CollapseKind, DeclineClass, DeclineGate, MintSpan,
        };
        let sid = |leaf: u32| dorc_aid::diag::SiteId {
            leaf: LeafId(leaf),
            member: None,
        };
        let decline = |leaf: u32, reason: Option<AuthoredReason>| {
            CollapseNarrative::new(
                SpeechAct::Vouched,
                CollapseKind::VerdictDecline {
                    site: sid(leaf),
                    arm: MintSpan(dorc_core::Span::new(
                        dorc_core::BytePos(4),
                        dorc_core::BytePos(9),
                    )),
                    arm_file: dorc_core::OracleFileId(0),
                    gate: DeclineGate::Return,
                    authored_reason: reason,
                },
            )
        };
        let tier2 = AuthoredReason {
            class: DeclineClass::Hazard,
            arm: MintSpan(dorc_core::Span::new(
                dorc_core::BytePos(1),
                dorc_core::BytePos(2),
            )),
            arm_file: dorc_core::OracleFileId(0),
        };
        let evidence = vec![
            decline(5, None), // site 5 — a dynamic-format decline (class unread statically)
            decline(6, Some(tier2)), // site 6 — already tier-2 classed (static wins)
        ];
        let reports = vec![
            ReportRecord {
                site: Some(rk(5)),
                class: Some(DeclineClass::Unsound),
                raw: "decline unsound k".to_owned(),
                recognized: true,
            },
            ReportRecord {
                site: Some(rk(6)),
                class: Some(DeclineClass::Unsound),
                raw: "decline unsound k".to_owned(),
                recognized: true,
            },
        ];
        let paired = pair_authored_reasons(evidence, &reports);
        assert!(
            matches!(
                paired[0].kind(),
                CollapseKind::VerdictDecline { authored_reason: Some(r), .. }
                    if r.class == DeclineClass::Unsound
            ),
            "the runtime record classes site 5's previously-unclassed decline"
        );
        assert!(
            matches!(
                paired[1].kind(),
                CollapseKind::VerdictDecline { authored_reason: Some(r), .. }
                    if r.class == DeclineClass::Hazard
            ),
            "site 6's tier-2 static class is NOT overwritten by the runtime echo (static wins)"
        );

        // `289:rul-mint-hardening-package` item 4b, for the ONE class a render consumes today; the
        // other eight are minted and dropped (`289:seam-narrative-render-unconsumed`).
        let rendered = static_decline_notes(&paired, &[], &[]);
        assert_eq!(
            rendered.len(),
            2,
            "both classed declines reach the why lane: {rendered:?}"
        );
        assert!(
            rendered.iter().any(|line| line.contains("unsound"))
                && rendered.iter().any(|line| line.contains("hazard")),
            "each rendered line names its own decline class: {rendered:?}"
        );
        assert!(
            static_decline_notes(&[decline(7, None)], &[], &[]).is_empty(),
            "an unclassed decline narrates nothing on this lane"
        );
    }

    #[test]
    fn report_lane_sanitizes_and_caps_the_raw_tail() {
        let capped = sanitize_report_raw(&format!("decline hazard {}", "x".repeat(500)));
        assert!(
            capped.chars().count() <= REPORT_RAW_CAP + 3,
            "capped at REPORT_RAW_CAP chars (+ the three-dot ASCII ellipsis)"
        );
        assert!(capped.ends_with("..."), "an over-cap tail is ellipsized");
        assert!(
            capped.is_ascii(),
            "the truncation marker is ASCII (`rul-ascii-output-forever`): {capped:?}"
        );
        let cleaned = sanitize_report_raw("decline unsound has\ta\ttab and \u{7} bell");
        assert!(
            !cleaned.contains('\u{7}') && !cleaned.contains('\t'),
            "control bytes are neutralized (a minimal terminal-safety floor)"
        );
        // An author's tail is host-produced text on its way to a terminal, so the lane covers
        // the characters that change how their NEIGHBOURS display as well as the ones a terminal
        // acts on directly.
        let flipped = sanitize_report_raw("decline unsound \u{202e}yek regnad\u{202c} \u{feff}");
        assert!(
            !flipped.chars().any(dorc_aid::display::is_format_or_bidi),
            "bidi and zero-width format controls are neutralized too: {flipped:?}"
        );
    }

    #[test]
    fn whylog_exact_file_is_parsed_and_excludes_directory_selection() {
        let parsed = parse_args_from(vec![
            "why".to_owned(),
            "--last".to_owned(),
            "--whylog=.whylog".to_owned(),
        ])
        .expect("exact whylog input parses");
        let Invocation::Analyze(args) = parsed else {
            panic!("expected analysis invocation");
        };
        assert_eq!(args.whylog.as_deref(), Some(".whylog"));
        assert!(args.whylog_dir.is_none());
        assert!(
            parse_args_from(vec![
                "why".to_owned(),
                "--last".to_owned(),
                "--whylog=.whylog".to_owned(),
                "--whylog-dir=durables".to_owned(),
            ])
            .is_err()
        );
    }

    /// [`probe1`] but ENTRY-bearing (a wrapped-context site): the runtime-EntryFailure input.
    fn probe1_entry(fact: FactKey, site_kind: ProbeSiteKind) -> ProbePlan {
        let mut p = probe1(fact, site_kind);
        p.checks[0].entry = Some(dorc_plan::EntryComposed {
            enter_defs: vec![],
            inner_fn: "x__predict".to_string(),
            inner_sh: "x__predict() { :; }".to_string(),
            inner_argv: vec![],
        });
        p
    }

    #[test]
    fn entry_bearing_site_ge2_rc_mints_class_only_runtime_entry_failure() {
        use dorc_aid::narrative::EntryFailureTag;
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let entry_probe = probe1_entry(fact, ProbeSiteKind::Query { valid: true });
        let tag = |records: &str, i: &mut Interner| -> Option<EntryFailureTag> {
            facts_from_sites(&entry_probe, &parse_str(records, i))
                .1
                .into_iter()
                .find_map(|e| match e.kind() {
                    CollapseKind::EntryFailure { class, .. } => Some(*class),
                    _ => None,
                })
        };
        assert_eq!(
            tag("site 0 effect=cant-tell rc=127\n", &mut i),
            Some(EntryFailureTag::MissingDeps),
            "rc 127 ⇒ missing deps in the view"
        );
        assert_eq!(
            tag("site 0 effect=cant-tell rc=2\n", &mut i),
            Some(EntryFailureTag::InContextDecline),
            "other ≥2 ⇒ in-context decline"
        );
        assert_eq!(
            tag("site 0 effect=holds rc=0\n", &mut i),
            None,
            "an answered entry check (rc 0) is no failure"
        );
        let plain = probe1(fact, ProbeSiteKind::Query { valid: true });
        let results = parse_str("site 0 effect=cant-tell rc=127\n", &mut i);
        assert!(
            !facts_from_sites(&plain, &results)
                .1
                .iter()
                .any(|e| matches!(e.kind(), CollapseKind::EntryFailure { .. })),
            "a non-entry site mints no EntryFailure (the class is entry-scoped)"
        );
    }

    #[test]
    fn a_dial_forbidden_wrapped_site_mints_entry_denial_and_a_licensed_one_mints_none() {
        // `289:rul-mint-hardening-package` item 4a for `EntryDenial`, the one static entry-consent
        // class with no pin at all. Both directions over one world, varying only the admin's dial;
        // anti-masking, so the narrative is read out of the real analysis, never handed in.
        const SUDO: &str = "# dorc-lang/v0.2\n\
             sudo__predict() {\n\
             while [ \"${1#-}\" != \"$1\" ]; do case \"$1\" in -u) shift 2 ;; *) shift ;; esac; done\n\
             env -i HOME=/root \"$@\"\n\
             }\n\
             sudo__lend_map() {\n\
             printf '%s\\n' root : lends user\n\
             : lends fs-view\n\
             : lends netns\n\
             \"$@\"\n\
             }\n\
             sudo__enter() {\n\
             sudo -n \"$@\"\n\
             }\n";
        const HORK: &str = "# dorc-lang/v0.2\n\
             hork__is_converged() {\n\
             : safe-across user\n\
             case \"$1\" in\n\
             install) hork query \"$2\" ;;\n\
             *) return 2 ;;\n\
             esac\n\
             }\n";

        let rungs = |dial: dorc_core::EscalationDial| -> Vec<dorc_aid::narrative::EntryDegradeTag> {
            let mut interner = Interner::default();
            let srcs = vec![SUDO.to_owned(), HORK.to_owned()];
            let refs: Vec<&str> = srcs.iter().map(String::as_str).collect();
            let paths = vec!["sudo.oracle.sh".to_owned(), "hork.oracle.sh".to_owned()];
            let checks: Vec<_> = refs
                .iter()
                .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
                .collect();
            let verdict_sets: Vec<_> = refs
                .iter()
                .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
                .collect();
            let parsed = dorc_syntax::parse("sudo hork install frob\n");
            let cfg = dorc_analysis::cfg::build(&parsed.value).value;
            let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut interner);
            build_wrapped_analysis(
                &srcs,
                &refs,
                &paths,
                &checks,
                &verdict_sets,
                &parsed.value,
                &cfg,
                &value,
                dial,
                dorc_core::Capability::Root,
                &mut interner,
            )
            .collapse_narrative
            .iter()
            .filter_map(|n| match n.kind() {
                CollapseKind::EntryDenial { rung } => Some(*rung),
                _ => None,
            })
            .collect()
        };

        assert_eq!(
            rungs(dorc_core::EscalationDial::NoEscalation),
            vec![dorc_aid::narrative::EntryDegradeTag::DialForbids],
            "a dial-forbidden wrapped site narrates its denial rung"
        );
        assert!(
            rungs(dorc_core::EscalationDial::VouchedOnly).is_empty(),
            "a consented entry is no collapse and mints nothing"
        );
    }

    #[test]
    fn own_wall_coord_selects_kill_coord_for_kill_establish_for_establish() {
        // 24E §7 (resid-kill-coherence): the coherence comparand is the node's OWN effect
        // coordinate — its killed cell (a kill node, from `kill_coords`) OR its establish cell (an
        // establish class). `own_wall_coord` unifies both, extending the Stage-2 establish-wall
        // check to kill-walls in BOTH the authored and derived footprint lanes.
        use dorc_analysis::cfg::CfgNodeId;
        use dorc_analysis::effect::SkipClass;
        let mut i = Interner::default();
        let killed = pkg(&mut i, "nginx"); // package:nginx@installed (a purge's killed cell)
        let established = pkg(&mut i, "curl");
        let kill_node = CfgNodeId(7);
        let est_node = CfgNodeId(3);
        let classes = vec![(est_node, SkipClass::EstablishAmbient(established))];
        let mut kill_coords = BTreeMap::new();
        kill_coords.insert(kill_node, killed);
        assert_eq!(
            own_wall_coord(kill_node, &classes, &kill_coords),
            Some(dorc_plan::EntityCoord::new(killed.kind, killed.entity)),
            "a kill node's comparand is its killed coordinate (24E §7 — the new close)"
        );
        assert_eq!(
            own_wall_coord(est_node, &classes, &kill_coords),
            Some(dorc_plan::EntityCoord::new(
                established.kind,
                established.entity
            )),
            "an establish node's comparand is its establish coordinate (Stage 2, unchanged)"
        );
        assert_eq!(
            own_wall_coord(CfgNodeId(99), &classes, &kill_coords),
            None,
            "a node that is neither establish nor kill has no coherence comparand"
        );
    }

    #[test]
    fn parse_results_maps_three_outcome_and_carries_rc() {
        // The record maps holds/absent/cant-tell to the Effect verdict and carries the
        // raw rc on the wire (whether it is fold-usable is the firewall's call).
        let mut i = Interner::default();
        let r = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\nsite 2 effect=cant-tell rc=2\n",
            &mut i,
        );
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Converged)
        );
        assert_eq!(
            r.records.get(&rk(1)).map(|x| x.verdict),
            Some(Verdict::Diverged)
        );
        assert_eq!(
            r.records.get(&rk(2)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        assert_eq!(r.records.get(&rk(0)).map(|x| x.rc), Some(Rc(0)));
        assert_eq!(r.records.get(&rk(1)).map(|x| x.rc), Some(Rc(1)));
    }

    #[test]
    fn parse_results_drops_garbage_kfail_perform() {
        // Unrecognized / malformed lines are dropped (⇒ Unknown ⇒ run). This is the TWIN of the
        // retired `garbage-stdin` e2e case (`27D`/`24I` batch-3 de-graduation, cli-surface): it feeds
        // that case's EXACT specimen — a non-`site` leading token, a non-numeric site-id, a `site`
        // line with no `effect=`, a `declared-rc` line with a non-integer rc, and a leading-whitespace
        // garbage line. None resolve site 0's effect ⇒ it folds to Unknown ⇒ the install runs
        // (`kFAIL-perform`), and the cli never crashes on the malformed stream.
        let mut i = Interner::default();
        let r = parse_str(
            "this is not a valid result line\nsite notanumber effect=holds\n\
             site 0 garbled-no-effect-field\ndeclared-rc xyz rc=notanint\n\
             \x20\x20\x20leading-whitespace garbage\n",
            &mut i,
        );
        // `site 0 garbled-no-effect-field` parses the id but no effect= ⇒ Unknown (safe ⇒ run).
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Unknown),
            "site 0 has no valid effect ⇒ Unknown ⇒ run"
        );
        // `site notanumber` ⇒ no id ⇒ dropped; the `declared-rc`, non-`site`, and whitespace-garbage
        // lines ⇒ dropped. Only the id-parseable site 0 lands (never a crash).
        assert_eq!(r.records.len(), 1, "only the id-parseable site landed");
    }

    #[test]
    fn parse_results_reserves_stdout_stderr_keys_inert() {
        // item-2 (19F §3 tuple shape): the `stdout=`/`stderr=` keys are RESERVED — the
        // parser accepts-and-stores them into the record's tuple, but they produce no
        // behavior change. Pin BOTH halves: (1) absent ⇒ the slots are `Predicted::Top`
        // (the default, the only state the probe actually emits today); (2) present ⇒
        // they intern into a `Predicted::Value(OutBytes)` and ride the tuple, while the
        // firewall + consumption gate are untouched (the consumed-stdout/stderr block is
        // unconditional, never reading the claim). Anti-masking: this asserts the SHAPE
        // exists end-to-end, NOT that a check predicts a value (nothing does this round).
        let mut i = Interner::default();
        let r = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert_eq!(
            rec.stdout,
            Predicted::Top,
            "absent stdout= ⇒ ⊤ (the live default)"
        );
        assert_eq!(
            rec.stderr,
            Predicted::Top,
            "absent stderr= ⇒ ⊤ (the live default)"
        );
        // Reserved keys parse-and-store (a future stdout-producing probe is value-plumbing).
        // `262` §2 last-to-token: `stdout=` is the TRAILING free-content field (the read-value
        // lane's carrier — `279f` rider), so `stderr=` (single-token, out of spike scope) must
        // PRECEDE it; and `stdout=`'s value runs to end-of-line so embedded spaces survive.
        let r = parse_str(
            "site 0 effect=holds rc=0 stderr=warn stdout=hello there world\n",
            &mut i,
        );
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert_eq!(
            match rec.stdout {
                Predicted::Value(OutBytes(s)) => i.resolve(s),
                Predicted::Top => "<top>",
            },
            "hello there world",
            "stdout= is last-to-token: embedded spaces survive byte-exactly (279f pin)"
        );
        assert!(
            matches!(rec.stderr, Predicted::Value(OutBytes(_))),
            "a reserved stderr= is stored as a value claim: {:?}",
            rec.stderr
        );
        // The Effect/Status path is unaffected by the reserved keys' presence.
        assert_eq!(rec.verdict, Verdict::Converged);
        assert_eq!(rec.rc, Rc(0));
    }

    #[test]
    fn firewall_establish_site_rc_never_becomes_fold_status() {
        // THE wrong-concrete firewall, direction 1 (202 §3 / task-D2): an ESTABLISH
        // site's record-rc is the CHECK command's rc (dpkg-query's), NOT the mutator's.
        // It must NEVER reach the fold's Status — status stays Top unconditionally,
        // even though the record carries `rc=0`.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Establish);
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(obs.effect, Verdict::Converged, "Effect = reported verdict");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an establish site's probe-rc must NOT become fold status (the disaster class)"
        );
    }

    #[test]
    fn firewall_valid_query_site_rc_feeds_fold_status() {
        // THE wrong-concrete firewall, direction 2 (task-D2): a VALID Query site's
        // record-rc IS the guard's own rc ⇒ it feeds the fold's Status exactly. This is
        // the relaxation that replaces the dead `declared-rc` lane.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Query { valid: true });
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Value(Rc(0)),
            "a valid Query guard's own rc supplies the fold Status"
        );
        // A non-zero guard rc (nginx absent) carries through identically (Exit(n) path).
        let results = parse_str("site 0 effect=absent rc=1\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .unwrap();
        assert_eq!(obs.status, Predicted::Value(Rc(1)), "rc 1 carries through");
    }

    #[test]
    fn firewall_invalid_query_site_rc_withheld() {
        // THE wrong-concrete firewall, direction 3 (rule-query-validity, 205 §2): an
        // INVALID Query site (a mutator/opaque reached it from entry) has a stale
        // resting rc ⇒ status stays Top even though the record carries `rc=0` ⇒ the
        // guard runs for real at apply. The bit is the ENGINE's (classify); the cli only
        // honors it.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Query { valid: false });
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an INVALID Query guard's rc is stale ⇒ withheld (status Top ⇒ runs for real)"
        );
    }

    #[test]
    fn invalid_query_withhold_mints_substitution_refusal() {
        // C5 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): the invalid-Query withhold
        // mints one SubstitutionRefusal; a valid Query (substitutable rc) mints none.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);

        let invalid = probe1(fact, ProbeSiteKind::Query { valid: false });
        let (_facts, evidence) = facts_from_sites(&invalid, &results);
        assert_eq!(
            evidence
                .iter()
                .filter(|e| matches!(e.kind(), CollapseKind::SubstitutionRefusal { .. }))
                .count(),
            1,
            "an invalid Query withhold mints one SubstitutionRefusal"
        );

        let valid = probe1(fact, ProbeSiteKind::Query { valid: true });
        let (_facts, evidence) = facts_from_sites(&valid, &results);
        assert!(
            !evidence
                .iter()
                .any(|e| matches!(e.kind(), CollapseKind::SubstitutionRefusal { .. })),
            "a valid Query substitutes its rc ⇒ no refusal"
        );
    }

    #[test]
    fn probe_origins_keys_measured_receipt_by_fact_with_stream_ordinal() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Establish);
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let mut arena = ProvArena::new();
        let origins = probe_origins(&probe, &results, &mut arena);
        let attribution = origins
            .get(&fact)
            .copied()
            .expect("the establish fact carries a probe-result origin");
        let node = arena
            .node(attribution.origin)
            .expect("the measured origin resolves in the arena");
        assert_eq!(
            node.kind,
            dorc_core::OriginKind::ProbeResult(dorc_core::ProbeStamp::at_ordinal(0)),
            "the ProbeResult stamp is the record's stream ordinal (site 0 = the 0th deframed record)"
        );
    }

    /// Deframe `input` under an INJECTED stepping clock, so a per-record instant is a property of
    /// the injected source rather than of the machine the test runs on.
    fn parse_str_clocked(input: &str, at: u64, step: u64, interner: &mut Interner) -> SiteResults {
        let expect = dorc_plan::records::Framing::spike(String::new()).expect();
        let d =
            dorc_plan::records::deframe(input, &expect, dorc_plan::records::LegacyPolicy::Tolerate);
        let mut clock = RunClock::Ticking {
            at: dorc_core::RunInstant(at),
            step_millis: step,
        };
        parse_results(&d.records, d.framed, &mut clock, interner)
    }

    #[test]
    fn record_observation_instants_come_from_the_injected_clock_not_wall_time() {
        // A record.s instant must be EXACTLY what the injected source yielded, and a stepping
        // source must distinguish records. Wall time never enters a test.s answer.
        let mut i = Interner::default();
        let results = parse_str_clocked(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            9_000,
            7,
            &mut i,
        );
        let stamps: Vec<dorc_core::ProbeStamp> =
            results.records.values().map(|r| r.stamp).collect();
        assert_eq!(
            stamps
                .iter()
                .map(|s| s.received_at)
                .collect::<Vec<Option<dorc_core::RunInstant>>>(),
            vec![
                Some(dorc_core::RunInstant(9_000)),
                Some(dorc_core::RunInstant(9_007)),
            ],
            "each record carries the instant the injected clock yielded for it, in arrival order"
        );
        let clockless = parse_str("site 0 effect=holds rc=0\n", &mut i);
        assert_eq!(
            clockless
                .records
                .values()
                .next()
                .map(|r| r.stamp.received_at),
            Some(None),
            "no clock ⇒ no instant; never RunInstant(0) masquerading as a measurement"
        );
    }

    #[test]
    fn reported_observation_carries_this_records_rc_and_its_predicts_line() {
        // Each of the three must come from THIS record/check pair: a wrong-record rc or a
        // defaulted span renders a confidently-wrong attribution.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let mut probe = probe1(fact, ProbeSiteKind::Establish);
        let span = dorc_core::Span::new(dorc_core::BytePos(40), dorc_core::BytePos(52));
        probe.checks[0].defining_span = Some((span, dorc_core::OracleFileId(3)));
        let results = parse_str_clocked("site 0 effect=holds rc=7\n", 1_234, 0, &mut i);
        let mut arena = ProvArena::new();
        let origins = probe_origins(&probe, &results, &mut arena);
        let reported = origins
            .get(&fact)
            .and_then(|a| a.reported)
            .expect("one record measured the fact ⇒ one reporting observation");
        assert_eq!(reported.tool_rc, Rc(7), "the rc is this record's, verbatim");
        assert_eq!(
            reported.predict_span,
            Some((span, dorc_core::OracleFileId(3))),
            "the reporting line is the shipped check's own defining span, file-qualified"
        );
        assert_eq!(
            reported.stamp.received_at,
            Some(dorc_core::RunInstant(1_234)),
            "the observation instant rides the same stamp the receipt origin was minted from"
        );
    }

    #[test]
    fn two_records_on_one_fact_report_no_single_observation() {
        // Two records are two events with no one speaker, instant, or rc; picking a winner would
        // fabricate a measurement. The receipt still joins both — nothing leaves the evidence plane.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_str_clocked(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            500,
            1,
            &mut i,
        );
        let mut arena = ProvArena::new();
        let attribution = probe_origins(&probe, &results, &mut arena)
            .get(&fact)
            .copied()
            .expect("the fact is still keyed");
        assert!(
            attribution.reported.is_none(),
            "two reporting records ⇒ no single reported row"
        );
        assert_eq!(
            arena
                .node(attribution.origin)
                .expect("the joined origin resolves")
                .kind,
            dorc_core::OriginKind::Join,
            "both records survive as receipts under a join"
        );
    }

    /// Two checks over the SAME fact (distinct sites) — the conflict-floor input.
    fn probe2(fact: FactKey, k0: ProbeSiteKind, k1: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![
                ProbePredict {
                    site: LeafId(0),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k0,
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: false,
                    emits_report: false,
                    entry: None,
                },
                ProbePredict {
                    site: LeafId(1),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k1,
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: false,
                    emits_report: false,
                    entry: None,
                },
            ],
            unresolvable: vec![],
        }
    }

    #[test]
    fn same_cell_conflicting_records_degrade_to_top() {
        // 20I find-6a / item-5 (the conflict floor): two sites on the SAME cell whose
        // records DISAGREE merge to ⊤, never last-write-wins. Two establish sites: site 0
        // reports holds, site 1 reports absent (a self-contradicting / forged host). The
        // merged Effect must be `Unknown` (⊤) ⇒ the apply runs (kFAIL-perform), NOT the
        // last-written `absent` (or `holds`). Anti-masking: a constructed conflict, not a
        // hand-injected verdict the check should predict.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.effect,
            Verdict::Unknown,
            "disagreeing same-cell Effect verdicts degrade to ⊤ (Unknown), not last-write-wins"
        );
    }

    #[test]
    fn same_cell_agreeing_records_pass_through() {
        // The floor's other half: two same-cell sites that AGREE pass the value through
        // (no spurious ⊤). Two establish sites both reporting holds ⇒ merged Effect is
        // Converged (the agreed value), so a genuinely-converged cell still elides.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.effect,
            Verdict::Converged,
            "agreeing same-cell records keep the agreed verdict (no spurious ⊤)"
        );
    }

    #[test]
    fn same_cell_disagreement_mints_measured_narrative_agreement_mints_none() {
        // C4 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): a cross-site disagreement
        // mints one `Measured` FactMergeDisagreement; an agreement mints none.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);

        let conflict = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let (_facts, evidence) = facts_from_sites(&probe, &conflict);
        assert_eq!(
            evidence.len(),
            1,
            "one cross-site disagreement ⇒ one record"
        );
        assert_eq!(evidence[0].tier(), SpeechAct::Measured);
        assert!(matches!(
            evidence[0].kind(),
            CollapseKind::FactMergeDisagreement { .. }
        ));

        let agree = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        let (_facts, evidence) = facts_from_sites(&probe, &agree);
        assert!(evidence.is_empty(), "agreeing records mint no disagreement");
    }

    /// Build + deframe a FRAMED stream (the framed regime) for a set of inner records, then
    /// inner-parse — the exact production round-trip a real remote host drives.
    fn parse_framed(sites: usize, inners: &[&str], i: &mut Interner) -> SiteResults {
        use dorc_plan::records::{DEFAULT_NONCE, TERMINAL_TOKEN};
        let framing = dorc_plan::records::Framing::spike("bk".to_owned());
        let recs = inners
            .iter()
            .map(|inner| format!("{DEFAULT_NONCE} {inner} {TERMINAL_TOKEN}\n"))
            .collect::<Vec<_>>()
            .concat();
        let s = format!(
            "dorc-records/1 nonce={DEFAULT_NONCE} attempt=1 host=localhost book=bk sites={sites} {TERMINAL_TOKEN}\n\
             {recs}dorc-records-end/1 nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n"
        );
        let d = dorc_plan::records::deframe(
            &s,
            &framing.expect(),
            dorc_plan::records::LegacyPolicy::Refuse,
        );
        assert!(
            !d.refused,
            "the framed round-trip is not refused: {:?}",
            d.diagnostics
        );
        parse_results(&d.records, d.framed, &mut RunClock::Absent, i)
    }

    #[test]
    fn pin_duplicate_records_merge_by_meet_never_last_wins() {
        // `262` §2 / §1 tie-break law: two records for ONE (site, member) key merge by MEET,
        // never last-wins. Conflicting ⇒ ⊤ (run); identical ⇒ idempotent. Arrival order is
        // never consulted — the disagreement result is the same in either order.
        let mut i = Interner::default();
        let fwd = parse_str(
            "site 0 effect=holds rc=0\nsite 0 effect=absent rc=1\n",
            &mut i,
        );
        let rev = parse_str(
            "site 0 effect=absent rc=1\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        let ra = fwd.records.get(&rk(0)).copied().expect("site 0");
        let rb = rev.records.get(&rk(0)).copied().expect("site 0");
        assert_eq!(
            ra.verdict,
            Verdict::Unknown,
            "conflict ⇒ ⊤ (run), never last-wins"
        );
        assert!(
            ra.conflicted,
            "a conflicting duplicate is marked (withholds the fold rc)"
        );
        assert_eq!(
            (ra.verdict, ra.conflicted),
            (rb.verdict, rb.conflicted),
            "the meet is order-independent (no arrival-sequence tie-break)"
        );
        // Idempotent: two IDENTICAL records ⇒ unchanged (not spuriously conflicted).
        let dup = parse_str(
            "site 0 effect=holds rc=0\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        let r = dup.records.get(&rk(0)).copied().expect("site 0");
        assert!(
            !r.conflicted && r.verdict == Verdict::Converged,
            "identical dup is idempotent"
        );
    }

    #[test]
    fn pin_fold_permutation_records_are_leafid_keyed_order_free() {
        // `262` §1 pin-fold-permutation: fold(any permutation of records) ≡ fold(book order).
        // Records are leafid-keyed + self-describing (no positional meaning), so two
        // distinct-cell records fed in either order fold to the identical by_fact map.
        let mut i = Interner::default();
        let nginx = pkg(&mut i, "nginx");
        let curl = pkg(&mut i, "curl");
        let probe = probe_two_facts(nginx, curl);
        let book = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let rev = parse_str(
            "site 1 effect=absent rc=1\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        assert_eq!(
            facts_from_sites(&probe, &book).0,
            facts_from_sites(&probe, &rev).0,
            "record arrival order never changes the fold (leafid-keyed)"
        );
    }

    #[test]
    fn pin_terminal_determinism_framed_folds_like_unframed() {
        // `262` §1 pin-terminal-determinism: the FRAMED serial stream folds to the same facts
        // as the unframed equivalent (byte-identical modulo framing lines). Deframing +
        // inner-parsing a framed stream ≡ legacy-parsing the same inner records.
        let mut i = Interner::default();
        let nginx = pkg(&mut i, "nginx");
        let curl = pkg(&mut i, "curl");
        let probe = probe_two_facts(nginx, curl);
        let inners = ["site 0 effect=holds rc=0", "site 1 effect=absent rc=1"];
        let unframed = parse_str(&format!("{}\n{}\n", inners[0], inners[1]), &mut i);
        let framed = parse_framed(2, &inners, &mut i);
        assert_eq!(
            facts_from_sites(&probe, &unframed).0,
            facts_from_sites(&probe, &framed).0,
            "the framing lines are fold-invisible — the plan is unchanged"
        );
    }

    /// A two-check probe over two DISTINCT cells (both valid Query, sites 0 and 1).
    fn probe_two_facts(f0: FactKey, f1: FactKey) -> ProbePlan {
        let mk = |site: u32, fact: FactKey| ProbePredict {
            site: LeafId(site),
            member: None,
            fact,
            provider: fact.kind.0,
            argv: vec![],
            site_kind: ProbeSiteKind::Query { valid: true },
            sh: "{ :; }".to_string(),
            defining_span: None,
            connected: None,
            verdict: false,
            emits_report: false,
            entry: None,
        };
        ProbePlan {
            checks: vec![mk(0, f0), mk(1, f1)],
            unresolvable: vec![],
        }
    }

    #[test]
    fn same_cell_conflicting_query_status_degrades_to_top() {
        // The conflict floor on the Status channel: two VALID Query sites on one cell
        // reporting DIFFERENT rcs (rc=0 vs rc=1) ⇒ merged status ⊤ (a self-contradicting
        // guard cannot fold a branch). A valid Query's rc normally feeds Status (the
        // firewall), but a conflict on it must still degrade — the meet beats the firewall.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe2(
            fact,
            ProbeSiteKind::Query { valid: true },
            ProbeSiteKind::Query { valid: true },
        );
        let results = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        // Effect agrees (both holds) ⇒ Converged; but the rcs disagree ⇒ status ⊤.
        assert_eq!(obs.effect, Verdict::Converged, "effect agrees");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "disagreeing same-cell Query rcs degrade Status to ⊤ (no fold off a contradiction)"
        );
    }

    #[test]
    fn unresolvable_diagnostics_name_the_source_command() {
        // q-2 (`dq-site-unresolvable`, the cli-edge readout): a probe-unresolvable site is
        // disclosed on stderr naming its SOURCE command text (`219` q-1.f silent-3 closed). An
        // un-oracled command (`make install`) is Opaque ⇒ unresolvable ⇒ the apply runs it; the
        // Note must carry its source. Drives the full pipeline (parse → classify → compile_probe
        // → build_plan) so the LeafId→source mapping is the real one.
        let mut interner = Interner::default();
        let book = "make install\n";
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
        let idx = dorc_oracle::KindIndex::default();
        let mut arena = ProvArena::new();
        let classified = dorc_analysis::effect::classify(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &[],
            &BTreeSet::new(),
            &mut interner,
            &mut arena,
        );
        let classes = classified.value;
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg.value,
            &value,
            &classes,
            &BTreeMap::new(),
            &dorc_plan::ConnectedPipes::default(),
            |_, _| None,
            |_, _, _| None,
            |_| false,
        );
        let plan = dorc_plan::build_plan(
            book,
            &parsed.value,
            &cfg.value,
            &classes,
            // All-Unknown ⇒ nothing elides regardless of any vouch; empty is honest here.
            &dorc_plan::Vouches::new(),
            |_| Observable::verdict_only(Verdict::Unknown),
            &mut arena,
        );
        let diags = unresolvable_diagnostics(&probe, &plan, &parsed.value, book);
        let i = Interner::default();
        assert!(
            diags.iter().any(|d| d.code.slug() == "site-unresolvable"),
            "an Opaque site must be disclosed unresolvable: {diags:?}"
        );
        assert!(
            diags.iter().any(|d| d.code.slug() == "site-unresolvable"
                && dorc_aid::diag::render_body(d, &i).contains("make install")),
            "the disclosure must name the source command: {diags:?}"
        );
        assert!(
            diags.iter().all(|d| d.severity() == Severity::Note),
            "the readout is Note-severity (never trips gate-3): {diags:?}"
        );
    }

    #[test]
    fn advisory_filter_drops_warnings_notes_but_keeps_errors_in_apply() {
        // rec-1 / tc-apply-receipt-floor (ui-A): the receipt-free `apply` off-ramp keeps the
        // ERROR floor (a shippable artifact must never hide an error) while dropping the
        // advisory plane (warnings + notes); `plan`/round-trip (advisory=true) pass everything
        // through. This is the lone place the artifact-vs-render two-surface split becomes a
        // per-severity routing decision — pin BOTH directions so a future edit cannot silently
        // (a) leak advisory disclosure into the off-ramp surface, or (b) swallow an error there.
        // One code per severity, spelled with real catalog variants (registry-Error/Warning/Note).
        use dorc_aid::diag::{CfgBuiltinShadowed, RedirTargetTop, SiteId, SyntaxMalformed};
        use dorc_core::{BytePos, Span};
        let span = Span::new(BytePos(0), BytePos(1));
        let mixed = vec![
            Diag::new(
                DiagCode::SyntaxMalformed(SyntaxMalformed {
                    detail: "an error".to_owned(),
                }),
                span,
            ),
            Diag::new(
                DiagCode::CfgBuiltinShadowed(CfgBuiltinShadowed {
                    detail: "a warning".to_owned(),
                }),
                span,
            ),
            Diag::new(
                DiagCode::RedirTargetTop(RedirTargetTop {
                    site: SiteId::leaf(LeafId(0)),
                }),
                span,
            ),
        ];

        // advisory=true (plan / round-trip): every severity survives — the full cited-disclosure
        // render surface (ru-20 ui-3).
        let kept = advisory_filter(true, &mixed);
        assert_eq!(kept.len(), 3, "plan surface keeps every severity: {kept:?}");

        // advisory=false (apply off-ramp): ONLY the error survives — receipt-free, not blind.
        let kept = advisory_filter(false, &mixed);
        assert_eq!(
            kept.len(),
            1,
            "apply keeps only the error floor (no warnings/notes): {kept:?}"
        );
        assert_eq!(
            kept[0].severity(),
            Severity::Error,
            "the surviving diagnostic is the Error (the never-hide floor): {kept:?}"
        );
    }
}

#[cfg(test)]
mod first_wall_tests {
    //! upcoming-firstwall-hint (`USER_STORY` stage 3): the pure `first_wall_hint` M-computation and
    //! its wording. Each `WallStep` scenario mirrors a real book shape — the role classification
    //! (opaque = probe-unresolvable real command; honest = running modeled mutator; guard =
    //! converged-but-walled) is exercised end-to-end on `guard23-ternary-flagship` (M=1) and
    //! `strawman24-opaque-wall` (M=0) by the e2e corpus; here we pin the algorithm over the roles.
    use super::{Excerpt, FirstWallHint, WallRole, WallStep, first_wall_hint, oracle_excerpt};

    fn ws(leaf: u32, line: usize, word: &str, role: WallRole) -> WallStep {
        WallStep {
            leaf: dorc_plan::LeafId(leaf),
            line,
            word: word.to_owned(),
            role,
        }
    }

    /// The flagship shape (guard23-ternary-flagship): nginx elides, `hork` is the opaque wall,
    /// curl guards past it, vim runs diverged (an honest wall). Lifting `hork` un-walls exactly
    /// curl ⇒ M=1, no further unmodeled walls.
    #[test]
    fn opaque_wall_with_downstream_guard_yields_m1() {
        let steps = [
            ws(0, 20, "apt-get", WallRole::Transparent),
            ws(1, 21, "hork", WallRole::Opaque),
            ws(2, 22, "apt-get", WallRole::Guard),
            ws(3, 23, "apt-get", WallRole::Honest),
        ];
        let fw = first_wall_hint(&steps).expect("an opaque wall fires the hint");
        assert_eq!(fw.line, 21);
        assert_eq!(fw.word, "hork");
        assert_eq!(fw.unwall, 1);
        assert_eq!(fw.more_walls, 0);
    }

    /// No opaque wall (every command modeled) ⇒ no hint — the mission's no-unmodeled-wall negative.
    #[test]
    fn no_opaque_wall_yields_none() {
        let steps = [
            ws(0, 1, "apt-get", WallRole::Transparent),
            ws(1, 2, "apt-get", WallRole::Guard),
            ws(2, 3, "apt-get", WallRole::Honest),
        ];
        assert!(first_wall_hint(&steps).is_none());
    }

    /// A modeled-but-diverged wall (Honest) with NO opaque wall ⇒ no hint. The mission's explicit
    /// "NOT fire for modeled-but-diverged walls" — those are honest walls, not oracle-gaps.
    #[test]
    fn honest_wall_only_yields_none() {
        let steps = [
            ws(0, 1, "apt-get", WallRole::Honest),
            ws(1, 2, "systemctl", WallRole::Guard),
        ];
        assert!(first_wall_hint(&steps).is_none());
    }

    #[test]
    fn empty_book_yields_none() {
        assert!(first_wall_hint(&[]).is_none());
    }

    /// Two converged-but-walled guards before the next wall ⇒ M=2 (both upgrade guard→elide).
    #[test]
    fn two_guards_before_next_wall_yields_m2() {
        let steps = [
            ws(0, 1, "hork", WallRole::Opaque),
            ws(1, 2, "systemctl", WallRole::Guard),
            ws(2, 3, "ufw", WallRole::Guard),
            ws(3, 4, "apt-get", WallRole::Honest),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(fw.unwall, 2);
        assert_eq!(fw.more_walls, 0);
    }

    /// An honest wall BOUNDS the count: a guard past it is walled by IT, not by the opaque wall, so
    /// lifting the opaque wall would not recover it. Only the guard in the opaque wall's own window
    /// counts ⇒ M=1.
    #[test]
    fn honest_wall_bounds_the_count() {
        let steps = [
            ws(0, 1, "hork", WallRole::Opaque),
            ws(1, 2, "systemctl", WallRole::Guard),
            ws(2, 3, "apt-get", WallRole::Honest),
            ws(3, 4, "ufw", WallRole::Guard),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(
            fw.unwall, 1,
            "the guard past the honest wall is not this wall's to un-wall"
        );
    }

    /// The two-opaque-wall shape (`USER_STORY` foobar + hork): a second opaque wall both BOUNDS the
    /// first's window (ufw past `hork` is not foobar's to un-wall) and adds a trailing pointer.
    #[test]
    fn second_opaque_wall_bounds_and_is_counted() {
        let steps = [
            ws(0, 8, "foobar", WallRole::Opaque),
            ws(1, 9, "systemctl", WallRole::Guard),
            ws(2, 10, "hork", WallRole::Opaque),
            ws(3, 11, "ufw", WallRole::Guard),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(fw.word, "foobar");
        assert_eq!(fw.unwall, 1);
        assert_eq!(fw.more_walls, 1);
    }

    /// An opaque wall with nothing improvable downstream ⇒ M=0 (still fires — you can elide it).
    #[test]
    fn opaque_wall_with_no_downstream_yields_m0() {
        let steps = [
            ws(0, 1, "apt-get", WallRole::Transparent),
            ws(1, 2, "hork", WallRole::Opaque),
        ];
        let fw = first_wall_hint(&steps).unwrap();
        assert_eq!(fw.unwall, 0);
        assert_eq!(fw.more_walls, 0);
    }

    /// A transparent step (inert builtin run / omit) between the wall and a guard does NOT bound the
    /// count — only a wall (opaque or honest) stops it.
    #[test]
    fn transparent_step_does_not_bound() {
        let steps = [
            ws(0, 1, "hork", WallRole::Opaque),
            ws(1, 2, "echo", WallRole::Transparent),
            ws(2, 3, "systemctl", WallRole::Guard),
            ws(3, 4, "apt-get", WallRole::Honest),
        ];
        assert_eq!(first_wall_hint(&steps).unwrap().unwall, 1);
    }

    fn hint(unwall: usize, more_walls: usize) -> FirstWallHint {
        FirstWallHint {
            leaf: dorc_plan::LeafId(1),
            line: 8,
            word: "foobar".to_owned(),
            unwall,
            more_walls,
        }
    }

    #[test]
    fn body_wording_matches_the_user_story_register() {
        // M=1, no further walls — the USER_STORY stage-3 sharpened form.
        assert_eq!(
            hint(1, 0).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged, and un-wall 1 downstream site"
        );
        // M=2 ⇒ "sites"; a further wall ⇒ the trailing pointer.
        assert_eq!(
            hint(2, 1).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged, and un-wall 2 downstream sites; 1 more \
             unmodeled wall -- dorc why"
        );
        // M=0 ⇒ the un-wall clause is dropped (never "un-wall 0").
        assert_eq!(
            hint(0, 0).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall -- an oracle vouching its \
             convergence would elide it when converged"
        );
        // more_walls plural.
        assert!(
            hint(0, 2)
                .body()
                .ends_with("; 2 more unmodeled walls -- dorc why")
        );
    }

    /// The pull-surface detail carries the recovery COUNT when there is one, and never a bare zero.
    /// Structure, not bytes: the words are arrangement-registry rows and ride
    /// `27V:rul-output-form-unwelded`, so pinning them verbatim here would weld exactly what that
    /// rule keeps free — and would re-break on every prose pass.
    #[test]
    fn why_detail_carries_the_unwall_count() {
        let with_count = hint(1, 0).why_detail();
        assert!(
            with_count.contains('1'),
            "the recovery count must reach the reader: {with_count}"
        );
        let without = hint(0, 0).why_detail();
        assert!(
            !without.contains('0'),
            "a zero count is dropped, never rendered as `0 sites`: {without}"
        );
        assert!(
            without.len() < with_count.len(),
            "the count-free form is the shorter one (the clause was dropped, not blanked)"
        );
    }

    /// `rul-ascii-output-forever` (`28E` §0, human-typed: "no unicode, ever. period. anywhere").
    /// The why-surface strings are the ones this lane authored or respelled; a stray em-dash or
    /// arrow creeping back into one is exactly what this catches.
    #[test]
    fn the_why_surface_renders_pure_ascii() {
        let mut checked: usize = 0;
        for entry in dorc_aid::arrangement::ARRANGEMENTS {
            if !entry.slug.starts_with("why-") {
                continue;
            }
            for word in entry.words.words().unwrap_or(&[]) {
                assert!(
                    word.is_ascii(),
                    "arrangement `{}` carries non-ASCII output: {word:?}",
                    entry.slug
                );
                checked = checked.saturating_add(1);
            }
        }
        assert!(checked > 0, "the why-surface registry rows must be reached");
        assert!(hint(1, 1).body().is_ascii());
        assert!(hint(1, 1).why_detail().is_ascii());
    }

    /// A synthetic oracle whose `disturbs` arm is preceded by the author's comment and followed by
    /// a body long enough to force a cut.
    const ARM_SOURCE: &str = "#!/usr/bin/env dorc-sh\n\
        # dorc-lang/v0.2\n\
        \n\
        # surveyed 2026-05: cert store only.\n\
        certsync__disturbs() {\n\
        line six\n\
        line seven\n\
        line eight\n\
        line nine\n\
        line ten\n\
        line eleven\n\
        }\n";

    fn excerpt_of(lo: u32, hi: u32) -> Excerpt {
        oracle_excerpt(
            Some((
                dorc_core::Span::new(dorc_core::BytePos(lo), dorc_core::BytePos(hi)),
                dorc_core::OracleFileId(0),
            )),
            &["certsync.oracle.sh".to_owned()],
            &[ARM_SOURCE.to_owned()],
        )
        .expect("the fixture threads a span into a loaded oracle file")
    }

    /// The author's comment ABOVE an arm is the author explaining that arm, so the massaging
    /// license (`27W:rul-report-surface-massaging`) attaches it — and stops at the blank line,
    /// rather than dragging the file's whole header down with it.
    #[test]
    fn an_excerpt_attaches_the_authors_adjacent_comment_and_stops_at_the_gap() {
        let arm_start = ARM_SOURCE
            .find("certsync__disturbs")
            .expect("the fixture defines the member");
        let arm = excerpt_of(
            u32::try_from(arm_start).expect("fixture offsets fit"),
            u32::try_from(arm_start).expect("fixture offsets fit"),
        );
        let numbers: Vec<usize> = arm.head.iter().map(|(number, _)| *number).collect();
        assert_eq!(
            numbers,
            vec![4, 5],
            "the comment on line 4 rides along; the version marker on line 2 is across a blank \
             line and is not this arm's"
        );
        assert_eq!(arm.elided, 0, "a two-line excerpt is contiguous");
    }

    /// A long arm is CUT, and the cut is reported rather than closed over: an excerpt that quietly
    /// shortened an author's contract would misrepresent what they wrote.
    #[test]
    fn a_long_excerpt_reports_the_middle_it_cut() {
        let whole = excerpt_of(0, u32::try_from(ARM_SOURCE.len()).expect("fixture fits"));
        assert!(
            whole.elided > 0,
            "a twelve-line span exceeds the inline budget and must be cut"
        );
        assert!(
            !whole.tail.is_empty(),
            "the cut keeps the arm's LAST line: where a case arm returns is what the reader needs"
        );
        let shown = whole.head.len().saturating_add(whole.tail.len());
        assert_eq!(
            shown.saturating_add(whole.elided),
            ARM_SOURCE.lines().count(),
            "every source line is either shown or counted in the cut, never silently dropped"
        );
    }
}

#[cfg(test)]
mod not_ours_bytes_tests {
    //! The why surface shows bytes we did not write.
    //!
    //! Oracle arms, their authors' comments, book lines and host-reported text all reach a
    //! terminal through this surface. They are classed not-ours and encoded on the way in
    //! (`aid::weave::foreign` over the `aid::display` seat), and these four tests are what keeps
    //! that true for somebody who never read this comment. They read as one battery: the sweep
    //! covers every seat, the classification test pins which class the bytes wear, the hostile
    //! fixtures drive real dangerous input through the real render, and the last one pins the
    //! byte-floored artifact plane OUT of all of it.
    use super::*;

    /// Bytes that must never reach a terminal as themselves, plus one plain non-ASCII sequence.
    const HOSTILE_SAMPLES: &[&str] = &[
        "\u{1b}",
        "\u{1b}[31m before-and-after",
        "\u{202e}reversed\u{202c}",
        "nul\u{0}and\u{7f}del",
        "tab\there",
        "na\u{ef}ve \u{feff}zero-width",
    ];

    /// A distinctive run of printable ASCII: it survives encoding VERBATIM, so wherever it lands
    /// in a render is exactly where the seat that emitted it put it.
    const SOURCE_MARK: &str = "source-mark-9137";

    /// One hostile sample folded into a line of somebody else's source.
    fn hostile_line(index: usize) -> String {
        let sample = HOSTILE_SAMPLES
            .get(index.checked_rem(HOSTILE_SAMPLES.len()).unwrap_or_default())
            .copied()
            .unwrap_or_default();
        format!("{SOURCE_MARK} {sample} # arm {index}")
    }

    /// Render a why-surface node list the way the surface itself does, and hand back the span map.
    fn swept(nodes: Vec<Node<Face>>) -> weft::Rendered<Face> {
        let frame = weft::Frame::of_width(weft::Width::new(WHY_WIDTH)).inset(TRIPTYCH_INSET);
        weft::render_framed(&Document::new(nodes), &frame)
    }

    /// Every seat of the why surface, driven with bytes we did not write in every slot that takes
    /// them. Built from STRUCT LITERALS on purpose: a new field on any of these types stops this
    /// compiling, so whoever adds one has to decide what hostile content belongs in it.
    fn every_why_surface_node() -> Vec<Node<Face>> {
        let book = format!("#!/bin/sh\n{}\n{}\n", hostile_line(0), hostile_line(1));
        let excerpt = Excerpt {
            path: format!("{SOURCE_MARK}.oracle.sh"),
            head: vec![(10, hostile_line(2)), (11, hostile_line(3))],
            tail: vec![(90, hostile_line(4))],
            elided: 78,
        };
        let chain = ChainRender {
            crossed: hostile_line(5),
            claimant: hostile_line(0),
            outcome: Said::words(
                "why-outcome-contrastive",
                &[
                    &hostile_line(1),
                    &hostile_line(2),
                    &hostile_line(3),
                    &hostile_line(4),
                ],
            ),
            analysis_opener: Said::Lens(hostile_line(5)),
            links: vec![ChainLink {
                tier: SpeechAct::Claimed,
                speaker: Some(hostile_line(0)),
                payload: Said::Value(hostile_line(1)),
                quoted: true,
                event: Some(Said::words("why-chain-event-rc-only", &[&hostile_line(2)])),
                explanation: Some(Said::Lens(hostile_line(3))),
                excerpt: Some(excerpt),
            }],
            participants: vec![2, 3],
            shipped: Some(hostile_line(4)),
            join: Some(Said::Value(hostile_line(5))),
            next_steps: NextSteps {
                opener: Said::Lens(hostile_line(0)),
                rows: vec![
                    StepRow {
                        label: StepLabel::Fix,
                        body: Said::Value(hostile_line(1)),
                        alternative: false,
                    },
                    StepRow {
                        label: StepLabel::Review,
                        body: Said::words("why-next-step-review", &[&hostile_line(2)]),
                        alternative: true,
                    },
                ],
            },
        };
        let site = WhySite {
            line: 2,
            word: hostile_line(3),
            command: hostile_line(4),
            outcome: outcome_word(&dorc_plan::Disposition::Run),
            foil: foil_word(&dorc_plan::Disposition::Run),
            reasons: vec![Said::Value(hostile_line(5))],
            class: AggregateClass::Improvement,
            improvement: Some(Said::Lens(hostile_line(0))),
        };
        let receipt = Receipt {
            at: None,
            replayed: false,
            host: hostile_line(1),
            book: hostile_line(2),
            book_digest: hostile_line(3),
            at_head: None,
            oracles: vec![hostile_line(4), hostile_line(5)],
            risk_profile: Some(CONSENT_FLAG),
            counts: dorc_plan::DispositionCounts {
                sites: 2,
                elide: 1,
                elide_by_proof: 0,
                elide_by_trusted_claim: 1,
                omit: 0,
                guard: 0,
                run: 1,
            },
            deepest_tier: true,
            narratable: true,
        };
        let mut nodes = vec![receipt_banner(&receipt), receipt_banner(&at_head(&receipt))];
        nodes.extend(participating_block(
            &[2, 3],
            &format!("{SOURCE_MARK}.book.sh"),
            &book,
        ));
        nodes.extend(chain_nodes(&chain));
        nodes.push(aggregate_item(
            &site,
            &format!("{SOURCE_MARK}.book.sh"),
            &[&Said::Lens(hostile_line(1))],
        ));
        nodes.extend(chain_nodes(&plain_chain(&site)));
        nodes
    }

    /// The same receipt wearing its git-annotation row instead of its digest row. The two are
    /// exclusive, so the sweep renders both banners: a commit is a SUBPROCESS's stdout, as not-ours
    /// as anything a host reported (`28D:must-encode-per-surface`).
    fn at_head(receipt: &Receipt) -> Receipt {
        Receipt {
            at_head: Some(source_match::SourceMatch {
                commit: hostile_line(6),
            }),
            oracles: receipt.oracles.clone(),
            host: receipt.host.clone(),
            book: receipt.book.clone(),
            book_digest: receipt.book_digest.clone(),
            at: receipt.at,
            replayed: receipt.replayed,
            risk_profile: receipt.risk_profile,
            counts: receipt.counts,
            deepest_tier: receipt.deepest_tier,
            narratable: receipt.narratable,
        }
    }

    /// THE SWEEP. Over every seat the why surface has, every not-ours run is already
    /// encoder-clean, and nothing else carries a control or bidi character.
    ///
    /// This is the test that bites somebody who adds a show-the-code row and hands weft the
    /// bytes directly. It does not care where they added it or what they called it: if the run
    /// reaches the render carrying anything a terminal would act on, or carrying an escape a
    /// second encoding pass would change, the sweep names the span and fails.
    #[test]
    fn every_why_surface_run_is_already_encoded_and_only_not_ours_bytes_are_escaped() {
        let rendered = swept(every_why_surface_node());
        let text = rendered.text().to_owned();
        assert!(!text.is_empty(), "the sweep must reach a real render");
        let mut foreign_spans = 0_usize;
        for span in rendered.spans() {
            let bytes = text
                .get(span.start..span.end())
                .expect("a span lies within its own render");
            if matches!(span.provenance, weft::Provenance::Foreign { .. }) {
                foreign_spans = foreign_spans.saturating_add(1);
                assert_eq!(
                    dorc_aid::display::encode_foreign(bytes, WHY_SOURCE_CAP),
                    bytes,
                    "a not-ours run reached the render un-encoded (re-encoding changed it): \
                     {bytes:?}"
                );
                continue;
            }
            for c in bytes.chars() {
                assert!(
                    c == '\n' || dorc_aid::display::is_display_safe(c),
                    "a run that is NOT classed not-ours carries {c:?}, which a terminal acts on \
                     — either the bytes are somebody else's and belong in a foreign run, or the \
                     value needs the display seat before it is interleaved: {bytes:?}"
                );
                assert!(
                    c.is_ascii(),
                    "the surface is pure ASCII (`rul-ascii-output-forever`) and weft measures \
                     bytes as columns; {c:?} reached it: {bytes:?}"
                );
            }
        }
        assert!(
            foreign_spans > 4,
            "only {foreign_spans} not-ours runs reached the sweep — the seats it means to cover \
             are not being reached"
        );
    }

    /// CLASSIFICATION. Inlined source lands ONLY in not-ours runs.
    ///
    /// The marker is printable ASCII, so encoding leaves it verbatim and every occurrence in the
    /// render is a seat that emitted somebody else's bytes. Any occurrence outside a foreign run
    /// is a show-the-code site wearing the wrong class — which reads to a round-trip as OUR
    /// words, and therefore as rephrasable prose.
    #[test]
    fn inlined_source_bytes_appear_only_inside_not_ours_runs() {
        let rendered = swept(vec![
            {
                let excerpt = Excerpt {
                    path: "certsync.oracle.sh".to_owned(),
                    head: vec![(4, format!("# {SOURCE_MARK} the author's own comment"))],
                    tail: Vec::new(),
                    elided: 0,
                };
                Node::new(NodeKind::Section(Section {
                    header: vec![dorc_aid::weave::words(
                        why_words("why-analysis-heading", &[]),
                        "why-analysis-heading",
                    )],
                    counts: None,
                    body: excerpt_nodes(&excerpt),
                }))
            },
            shipped_block(&format!("( certsync__is_converged ) || {SOURCE_MARK}")),
        ]);
        let text = rendered.text().to_owned();
        assert!(
            text.matches(SOURCE_MARK).count() >= 2,
            "both inlined-source seats must reach the render: {text}"
        );
        let mut inside_foreign = 0_usize;
        for span in rendered.spans() {
            let bytes = text
                .get(span.start..span.end())
                .expect("a span lies within its own render");
            let hits = bytes.matches(SOURCE_MARK).count();
            if hits == 0 {
                continue;
            }
            assert!(
                matches!(span.provenance, weft::Provenance::Foreign { .. }),
                "inlined source landed in a run classed {:?} — somebody else's bytes must wear \
                 the not-ours class, never a template, value or arrangement one: {bytes:?}",
                span.provenance
            );
            inside_foreign = inside_foreign.saturating_add(hits);
        }
        assert!(
            inside_foreign >= 2,
            "the marker was found in the text but not inside any not-ours span"
        );
    }

    /// HOSTILE FIXTURES, through the real render. Each is encoded, and each survives non-empty:
    /// silently dropping an author's text would be its own kind of lie about the source.
    #[test]
    fn a_hostile_oracle_comment_is_encoded_and_never_silently_dropped() {
        let long = "L".repeat(WHY_SOURCE_CAP.saturating_mul(3));
        let cases: Vec<(&str, String)> = vec![
            ("a bare escape", "\u{1b}".to_owned()),
            ("a CSI colour sequence", "\u{1b}[31mred\u{1b}[0m".to_owned()),
            ("a bidi override", "# \u{202e}rewordppa\u{202c}".to_owned()),
            ("NUL and DEL", "a\u{0}b\u{7f}c".to_owned()),
            ("a line far past the cap", long),
            (
                "valid non-ASCII UTF-8",
                "# na\u{ef}ve \u{2014} surveyed".to_owned(),
            ),
        ];
        for (what, source) in cases {
            let excerpt = Excerpt {
                path: "hostile.oracle.sh".to_owned(),
                head: vec![(1, source.clone())],
                tail: Vec::new(),
                elided: 0,
            };
            let rendered = swept(excerpt_nodes(&excerpt));
            let text = rendered.text();
            assert!(
                text.is_ascii(),
                "{what} reached the terminal un-encoded: {text:?}"
            );
            for c in text.chars() {
                assert!(
                    c == '\n' || dorc_aid::display::is_display_safe(c),
                    "{what} left {c:?} in the render: {text:?}"
                );
            }
            let foreign: String = rendered
                .spans()
                .iter()
                .filter(|span| matches!(span.provenance, weft::Provenance::Foreign { .. }))
                .filter_map(|span| text.get(span.start..span.end()))
                .collect();
            assert!(
                !foreign.trim().is_empty(),
                "{what} was dropped rather than encoded — an author's text always survives in \
                 some readable form"
            );
        }
    }

    /// THE ARTIFACT PLANE STAYS OUT OF IT. Display encoding is a render-plane act; the emitted
    /// probe and apply are byte-floored (`two-surfaces`, `law-render-overlay-never-artifact`) and
    /// carry the book's bytes exactly as written.
    ///
    /// Driven over one book whose source carries an escape and a bidi override, so the two planes
    /// are forced apart in the same run: the artifacts must contain the RAW bytes and never the
    /// escaped spelling, and the why surface must contain the escaped spelling and never the raw
    /// bytes. A change that encoded on the way to an artifact fails the first half; a change that
    /// stopped encoding on the way to a terminal fails the second.
    #[test]
    fn display_encoding_never_reaches_the_emitted_artifacts() {
        let raw = "\u{1b}[31m\u{202e}";
        let book = format!("make install # {SOURCE_MARK} {raw}\n");
        let mut interner = Interner::default();
        let parsed = dorc_syntax::parse(&book);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
        let idx = dorc_oracle::KindIndex::default();
        let mut arena = ProvArena::new();
        let classified = dorc_analysis::effect::classify(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &[],
            &BTreeSet::new(),
            &mut interner,
            &mut arena,
        );
        let classes = classified.value;
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg.value,
            &value,
            &classes,
            &BTreeMap::new(),
            &dorc_plan::ConnectedPipes::default(),
            |_, _| None,
            |_, _, _| None,
            |_| false,
        );
        let plan = dorc_plan::build_plan(
            &book,
            &parsed.value,
            &cfg.value,
            &classes,
            &dorc_plan::Vouches::new(),
            |_| Observable::verdict_only(Verdict::Unknown),
            &mut arena,
        );
        let framing = dorc_plan::records::Framing::spike("fixture".to_owned());
        let artifacts = format!(
            "{}{}",
            probe.render_sh(&framing, &interner),
            plan.render_apply(&book, &parsed.value)
        );
        assert!(
            artifacts.contains(raw),
            "the byte-floored artifact must carry the book's bytes verbatim: {artifacts:?}"
        );
        assert!(
            !artifacts.contains("\\x1b"),
            "a display encoding reached an emitted artifact — the overlay never becomes the \
             artifact: {artifacts:?}"
        );

        let shown = swept(participating_block(&[1], "book.sh", &book));
        let text = shown.text();
        assert!(
            text.contains("\\x1b") && text.contains("\\xe2\\x80\\xae"),
            "the same bytes reach the terminal only in their encoded form: {text:?}"
        );
        assert!(
            !text.contains(raw),
            "raw escape/override bytes reached the terminal: {text:?}"
        );
    }
}
