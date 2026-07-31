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
mod transport_edge;
mod whylog_store;

use dorc_aid::diag::{AidUnloadedSiblingOracle, Diag, DiagCode, EscalationPolicy};
use dorc_aid::said::Said;
use dorc_aid::{Carrier, CollapseKind, CollapseNarrative, Severity, SpeechAct};
use dorc_core::{Interner, Observable, ProvArena, Symbol, Verdict};

// The invocation surface lives in the crate's INTERNAL lib target (`289:rul-worldless-route-
// honest-trigger`) so the loom harness can fire the real parser; this bin keeps every I/O edge.
use dorc_cli::kinds::{KindReaches, KindResolvers, build_kind_reaches, build_kind_resolvers};
use dorc_cli::results::{ReportRecord, RunClock, SiteResults, facts_from_sites, probe_origins};
use dorc_cli::survival::{
    build_resolutions, build_survival_footprints, build_wrapped_analysis, collect_coord_kinds,
    collect_resolver_coords, dangling_diagnostics, entity_text_of, expand_footprints_via_reaches,
    lift_touches_sets, merge_derived_footprints, resolve_touches_footprint, ship_touches_body,
};
use dorc_cli::world::{ship_predict_body, ship_verdict_body};
// The legacy headerless string parser below is `#[cfg(test)]`-gated law
// (`rul-fixture-identity-never-production`), so its tokenizers are imported on the same gate.
#[cfg(test)]
use dorc_cli::results::{
    REPORT_RAW_CAP, RecordKey, ResolvOutcome, parse_leaf, parse_report_record, parse_site_record,
    sanitize_report_raw, split_key,
};
#[cfg(test)]
use dorc_cli::survival::own_wall_coord;
use dorc_cli::{
    Args, CONSENT_FLAG, DriftedReceipt, Invocation, LintArgs, LintFormat, Mode, PlanTally, Receipt,
    humane_read_error, parse_args_from,
};
#[cfg(test)]
use dorc_core::{OutBytes, Predicted, Rc};
// The why REPORT composes across the same seam (`28L:rul-full-driver-this-arc`): this edge builds
// the world and prints the bytes, the lib turns that world into a stamped part stream.
use dorc_cli::why::{
    CascadeAttribution, WhyReport, collect_wall_steps, first_wall_hint, flatten_ws,
    is_structurally_unprobeable, oracle_locus, render_coord, why_report_parts,
};

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
/// Evidence offered to the pipeline failed admission — host records the framing refused, or a
/// receipt whose book has drifted. Nothing was measured, so no artifact is honest. Third of the
/// 10..=19 range.
const EXIT_INGRESS_REFUSED: u8 = 12;
/// No session process was ever created, so the destination was never contacted. The one transport
/// exit licensed to claim the host is untouched, which is what makes a bare retry safe.
const EXIT_HOST_NOT_REACHED: u8 = 13;
/// A session ran and never reported completion: whether the remote artifact ran is UNKNOWN
/// (`rul-integrity-failure-withholds-mutation`). Deliberately NOT folded into
/// [`EXIT_HOST_NOT_REACHED`] — the caller's remedy differs, and a caller that retries this one
/// blindly may re-apply a mutation that already landed.
const EXIT_SESSION_LOST: u8 = 14;
/// A remote apply ran to completion and its artifact exited non-zero. The one KNOWN transport
/// outcome; the remote status is reported in the diagnostic, never reproduced as our own exit
/// (a plan exiting 13 must not be read as our "host not reached").
const EXIT_APPLY_FAILED: u8 = 15;

/// `dorc lint`: findings AT OR ABOVE the `--fail-on` threshold were reported (`27R` §5 exit
/// trichotomy). Distinct from clean (0) and from operational (below); shares linter convention.
const EXIT_LINT_FINDINGS: u8 = 1;
/// `dorc lint`: an OPERATIONAL error — the lint itself is compromised, distinct from both clean and
/// findings (`27R` §5, §8b): zero lintable files, an `--expect-files` mismatch, or a `--require-tools`
/// absence. NOT in the 10..=19 dorc-semantic family (a ⊤-reject book is a FINDING for lint, `27R` §5).
/// Numbered 3 (tc-lint-operational-exit-code — golangci-lint uses 3=Failure, shellcheck 3=bad-invoke;
/// the conservative lean, flagged for the human).
const EXIT_LINT_OPERATIONAL: u8 = 3;

/// Wall-clock ceiling on a probe session, in seconds (`260` s3-6).
///
/// Bounded by default because a probe is read-only by contract, so the worst a ceiling can cost
/// is a re-probe. An apply has no default ceiling for the mirror-image reason: killing one does
/// not fail it, it mints Unknown, so the caller must opt in with `--apply-timeout`.
const DEFAULT_PROBE_TIMEOUT_SECS: u64 = 120;

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
    /// Evidence offered to the pipeline failed admission — refused host records, or a drifted
    /// receipt — so no honest artifact could be built.
    IngressRefused,
    /// No session process was created ⇒ the destination is provably untouched.
    HostNotReached,
    /// A session ran without reporting completion ⇒ the world's state is unknown.
    SessionLost,
    /// A remote apply completed and its artifact exited non-zero.
    ApplyFailed,
}

fn main() -> ExitCode {
    match parse_args() {
        Ok(Invocation::Help) => {
            print!("{}", dorc_cli::help_text(&render_ctx()));
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
        Ok(Invocation::Analyze(args)) => match run(&args, &mut clock_for_invocation()) {
            Ok(RunOutcome::Complete) => ExitCode::SUCCESS,
            Ok(RunOutcome::BookUnmodeled) => ExitCode::from(EXIT_BOOK_UNMODELED),
            Ok(RunOutcome::WrapperIncoherent) => ExitCode::from(EXIT_WRAPPER_INCOHERENT),
            Ok(RunOutcome::IngressRefused) => ExitCode::from(EXIT_INGRESS_REFUSED),
            Ok(RunOutcome::HostNotReached) => ExitCode::from(EXIT_HOST_NOT_REACHED),
            Ok(RunOutcome::SessionLost) => ExitCode::from(EXIT_SESSION_LOST),
            Ok(RunOutcome::ApplyFailed) => ExitCode::from(EXIT_APPLY_FAILED),
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
            detail: dorc_aid::ForeignBytes::from_os_error(err),
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

/// The tables and box every render seat in THIS file reads through
/// (`28L:rul-render-context-struct`).
///
/// Always the compiled-in ones, and that is a fact about this file rather than a shortcut: `main.rs`
/// is the I/O edge, and the loom drives `lib.rs` (`lib-target-is-a-loom-seam`), never anything here
/// — so no seat in this file can ever be handed an editable mirror. Named ONCE all the same, so the
/// day one can be, there is a single place that changes.
fn render_ctx() -> dorc_aid::RenderCtx<'static> {
    dorc_aid::RenderCtx::production()
}

/// One registry-sourced chrome line, its computed values interleaved between the entry's words
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`). These stderr lines have a registry
/// HOME but not yet an editable face: no case drives them, so their words are edited in the lock
/// until a page case exists for them.
fn chrome(slug: &'static str, values: &[&str]) -> String {
    dorc_cli::chrome_line_parts(&render_ctx(), slug, values).text()
}

fn report_invocation_error(diag: &Diag) {
    eprint!(
        "{}",
        dorc_cli::invocation_error_parts(&render_ctx(), diag, &Interner::default()).text()
    );
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
        print!("{}", dorc_cli::lint_sources_parts(&render_ctx()).text());
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
            dorc_lint::render::render_human_parts_at(&render_ctx(), &report, args.verbosity).text()
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

/// The clock this invocation runs on: the harness pin when one is set, else the real one.
/// Read at the process edge, once (`io-at-edges-only`). A free function rather than a
/// [`RunClock`] method because the type itself is pure and lives across the loom seam; the
/// environment read is what has to stay on this side of it.
fn clock_for_invocation() -> RunClock {
    match std::env::var(FIXTURE_CLOCK_ENV)
        .ok()
        .as_deref()
        .map(str::parse::<u64>)
    {
        Some(Ok(millis)) => RunClock::Ticking {
            at: dorc_core::RunInstant(millis),
            step_millis: 0,
        },
        Some(Err(_)) => RunClock::Absent,
        None => system_clock(),
    }
}

/// The ONE wall-clock read. A clock the platform cannot place after the epoch answers
/// [`RunClock::Absent`] rather than saturating to a fabricated zero (`inv-no-throw`).
fn system_clock() -> RunClock {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|d| u64::try_from(d.as_millis()).ok())
        .map_or(RunClock::Absent, |millis| RunClock::Ticking {
            at: dorc_core::RunInstant(millis),
            step_millis: 0,
        })
}

#[expect(
    clippy::too_many_lines,
    clippy::result_large_err,
    reason = "the top-level pipeline driver: lift → analyze → probe → plan → render, one linear sequence with mode-routing; splitting it into sub-drivers would scatter the ONE call-shape the thin-driver mandate keeps here. The Err is a full `Diag` on a once-per-process path"
)]
fn run(args: &Args, clock: &mut RunClock) -> Result<RunOutcome, Diag> {
    if args.mode == Mode::Apply
        && let Some(host) = args.host.as_deref()
    {
        return ship_consented_apply(args, host);
    }

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
        let loaded = load_whylog_replay(args)?;
        report_at(advisory, "whylog", None, &loaded.diags);
        match loaded.value {
            ReplayLoad::Admitted(replay) | ReplayLoad::NoObservation(replay) => Some(replay),
            // Answered ABOVE the pipeline, whose first act is analyzing the book at the recorded
            // path — under drift, not the run's book. Only `why` has a degraded surface; every
            // other receipt-reading mode wants a plan, and there is no honest degraded plan.
            ReplayLoad::Drifted(drifted) if mode == Mode::Why => {
                emit_drifted_why(args.why_address.as_deref(), &drifted);
                std::io::stdout().flush().ok();
                return Ok(RunOutcome::Complete);
            }
            ReplayLoad::Drifted(_) | ReplayLoad::Refused => {
                return Ok(RunOutcome::IngressRefused);
            }
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

    // Pre-lift each file's verdict funcdefs so the (immutable-interner) probe ship-closure can
    // strip a verdict-lane site's body without a mutating re-lift (`24L` §2 probe emission). Diags
    // drop here — `validate` surfaces them once, per-file, for gate-3.
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = oracle_refs
        .iter()
        .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
        .collect();
    // The `24L` §7 kernel seam, widened by `26H` §3: the kernel stays verdict-unaware, so the edge
    // keys the role by provider and threads it in as DATA. From the sets above ⇒ ONE lift.
    let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);

    // The escalation-POLICY disclosure (`27C:render-authority-disclosure`): one advisory line naming
    // the escalation posture (the dial × the connection capability) and the entry-capable wrappers
    // loaded. Consent legibility — the admin sees, once, what authority the probe re-uses.
    report_at(
        advisory,
        "escalation",
        None,
        &escalation_policy_diagnostics(&mut interner, &oracle_refs, args.dial, args.capability),
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
    report_at(
        advisory,
        "oracle",
        None,
        &unloaded_sibling_oracle_diagnostics(books, &oracle_paths),
    );
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
    // `degrades` (`26G:fnd-existence-gate-darkens-oracle`): why each ⊤-degrading site's oracle
    // check gave up. Diagnostics only — it reaches the `site-unresolvable` note and nothing else.
    let mut degrades = BTreeMap::new();
    // `26H` §3.5 — sites whose establish came from the VERDICT lane, so their probe ships the
    // verdict body. Site-keyed: nothing about the FACT distinguishes an authored verdict cell.
    let mut verdict_lane = BTreeSet::new();
    let frozen = FrozenModel {
        cfg: &cfg.value,
        value: &value,
        ast: &parsed.value,
        idx: &idx,
        checks: &checks,
        verdicts: &verdicts,
        peeled: &peeled_sites,
    };
    let origin = classify_round(
        &frozen,
        &dorc_analysis::erase::ErasedSites::none(),
        &mut interner,
        &mut arena,
        &mut degrades,
        &mut verdict_lane,
    );
    let classes = origin.classes.clone();
    let kills = origin.kills.clone();

    // The per-site guard VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c) —
    // ALWAYS-ON (guards are the un-flagged baseline; rul24-mode-gate governs only the survival
    // tier, NOT this). A vouched past-wall establish ships its read-only probe (the witness needs
    // the verdict) and, converged, mints a `Disposition::Guard`.
    // Lift diags drop here: `validate` above surfaces them per-file. This lane could only report
    // them sourceless, which framed every verdict give-up at a fileless `1:1`.
    let vouch_lift = build_vouches(&oracle_refs, &classes, &value, &mut interner);
    let (mut vouches, decline_narrative) = vouch_lift.value;
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
    // `24L` §2 — a VERDICT-LANE site ships the oracle.s own `is_converged` funcdef, strip-only
    // (rul-only-oracle-bytes-ship). Keyed on the SITE.s lane, never its fact.s KIND (`26H` §3.5):
    // an authored verdict cell is an ordinary kind, so `is_auto_kind` would route it to the
    // predict lane, find nothing, and run the site. Try-order cannot stand in either —
    // `command_effect` reaches this lane from two fallbacks, and the second leaves a shippable
    // predict on a site whose cell the verdict body owns.
    let ship_auto = |node: dorc_analysis::cfg::CfgNodeId,
                     p: Symbol,
                     _a: &[Symbol]|
     -> Option<dorc_plan::ShippedCheck> {
        if !verdict_lane.contains(&node) {
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
    )
    .with_unresolvable_causes(&parsed.value, &cfg.value, &classes, &degrades);

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
    let touches_paired: Vec<(&str, dorc_oracle::touches::TouchesSet)> = if args.risk_faultless_skips
    {
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
    let derivations = if args.risk_faultless_skips {
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
    let resolver_lift = build_kind_resolvers(
        &oracle_srcs,
        &checks,
        &touches_paired,
        &coord_kinds,
        &mut interner,
    );
    report_at(advisory, "resolve", None, &resolver_lift.lift);
    report_by_oracle_file(
        advisory,
        "resolve",
        &oracle_paths,
        &oracle_srcs,
        &resolver_lift.confusability,
    );
    let kind_resolvers = resolver_lift.value;
    let resolver_kinds: BTreeSet<Symbol> = kind_resolvers.resolver_kinds().collect();
    let resolver_coords = if args.risk_faultless_skips && !resolver_kinds.is_empty() {
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
    let reaches_lift = build_kind_reaches(
        &oracle_srcs,
        &checks,
        &touches_paired,
        &coord_kinds,
        &mut interner,
    );
    report_at(advisory, "reaches", None, &reaches_lift.lift);
    report_by_oracle_file(
        advisory,
        "reaches",
        &oracle_paths,
        &oracle_srcs,
        &reaches_lift.confusability,
    );
    let kind_reaches = reaches_lift.value;
    let reach_kinds: BTreeSet<Symbol> = kind_reaches.reach_kinds().collect();
    let reaches_plan = if args.risk_faultless_skips && !reach_kinds.is_empty() {
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
    // A function of the framing because `attempt=` is baked into these bytes: a retry must
    // RE-RENDER, never re-send (`26A` amend-retry-hygiene).
    let render_probe_artifact = |f: &dorc_plan::records::Framing| -> String {
        let mut out = probe.render_sh(f, &interner);
        out.push_str(&derivations.render_sh(f.nonce(), &interner)); // 24E §2: SAME phase-1 block
        out.push_str(&resolvers.render_sh(f.nonce())); // 24F §3: SAME phase-1 block
        out.push_str(&reaches_plan.render_sh(f.nonce())); // 24G §4: SAME phase-1 block
        out.push_str(&dorc_plan::records::sentinel_line(f.nonce()));
        out
    };
    let framing = dorc_plan::records::Framing::spike(book_digest(&book_src));
    if mode == Mode::Probe {
        print!("{}", render_probe_artifact(&framing));
        std::io::stdout().flush().ok();
        return Ok(book_outcome);
    }

    // The round-trip emits the probe FIRST (phase 1 on stdout), then the apply (phase 2)
    // after stdin EOF — the e2e harness splits the two on the `#!/bin/sh` shebang. `plan`
    // and `apply` emit ONLY the apply artifact (the probe is an internal compile there).
    if mode == Mode::RoundTrip {
        print!("{}", render_probe_artifact(&framing));
        std::io::stdout().flush().ok();
    }

    let (framing, shipped_evidence) = match args.host.as_deref() {
        None => (framing, None),
        Some(raw) => {
            let host =
                dorc_transport::HostId::new(raw).map_err(|_| transport_edge::host_rejected(raw))?;
            if let Some(line) = transport_edge::first_carriage_return(book_src.as_bytes()) {
                return Err(transport_edge::crlf_refusal(book_name, line));
            }
            let nonce = transport_edge::mint_nonce();
            let mut driver = transport_edge::driver_for_invocation(
                args.connect_timeout,
                args.accept_new,
                args.ssh_config.as_deref(),
            );
            let timeout = Some(std::time::Duration::from_secs(
                args.probe_timeout.unwrap_or(DEFAULT_PROBE_TIMEOUT_SECS),
            ));
            match transport_edge::ship_probe(
                driver.as_mut(),
                &host,
                &nonce,
                &book_digest(&book_src),
                timeout,
                &render_probe_artifact,
            ) {
                transport_edge::ProbeShipment::Captured {
                    stdout,
                    framing,
                    stderr,
                } => {
                    transport_edge::echo_host_stderr(&stderr);
                    (framing, Some(stdout))
                }
                // NOT the analysis fail-direction (unsure ⇒ run): not knowing whether we still
                // talk to the world we think we do is no fact about it
                // (`rul-integrity-failure-withholds-mutation`).
                transport_edge::ProbeShipment::Lost {
                    diagnosis,
                    attempts,
                } => {
                    report_at(
                        advisory,
                        "transport",
                        None,
                        &[transport_edge::session_lost(raw, attempts, &diagnosis)],
                    );
                    return Ok(RunOutcome::SessionLost);
                }
                transport_edge::ProbeShipment::NotAttempted { reason } => {
                    report_at(
                        advisory,
                        "transport",
                        None,
                        &[transport_edge::not_attempted(raw, &reason)],
                    );
                    return Ok(RunOutcome::HostNotReached);
                }
            }
        }
    };

    // read the (simulated) probe results — the site-keyed records the rendered probe would emit
    // when run remotely (the round-trip's return channel). From `--results FILE` when given, else
    // the default stdin (the harness pipes them in).
    let run_sources = dorc_cli::results::RunSources {
        book_name,
        book: &book_src,
        oracle_paths: &oracle_paths,
        oracle_sources: &oracle_srcs,
    };
    let scope = dorc_cli::results::replay_scope(&framing, &run_sources);
    let (admitted_records, scoped_results, whylog_eligible) = if let Some(r) = replay.as_ref() {
        let scoped = dorc_cli::results::replayed_records(
            scope,
            r.records.as_ref(),
            &mut RunClock::Recorded(r.instants.clone()),
            &mut interner,
        );
        (None, scoped, false)
    } else {
        // The BOUNDED READ is this edge's (`rul-host-bytes-bounded-before-admission`): the limit
        // is spent against the real reader, before anything is allocated, and only the bounded
        // bytes cross the seam.
        let evidence = if let Some(captured) = shipped_evidence.as_deref() {
            dorc_plan::records::read_host_evidence(
                std::io::Cursor::new(captured),
                dorc_plan::records::HostEvidenceLimits::spike_default(),
            )
        } else if let Some(path) = &args.results {
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
                dorc_cli::results::admit_controller_records(
                    &framing,
                    &run_sources,
                    &bytes,
                    clock,
                    &mut interner,
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
            dorc_plan::records::Admission::Admitted(admitted) => {
                (Some(admitted.records), admitted.scoped, true)
            }
            dorc_plan::records::Admission::NoObservation => {
                (None, dorc_cli::results::no_observation(scope), false)
            }
            dorc_plan::records::Admission::Refused(reason) => {
                report_at(advisory, "records", None, &[reason.spanless_diagnostic()]);
                return Ok(RunOutcome::IngressRefused);
            }
        }
    };
    let _scope = scoped_results.scope();
    let results = scoped_results.results();

    // re-key the site-keyed records to the FactKey-keyed observations `build_plan`
    // consumes (its fold/elision machinery is fact-keyed; only this probe-answer
    // plumbing re-keys — `inv-site-keyed-results`). The probe's `checks` carry each
    // site's resolved fact + its `site_kind`, so a site-record maps site→fact AND the
    // firewall knows whether the rc is fold-usable. CRITICAL (the wrong-concrete
    // firewall, 202 §3 / task-D2): a record's `rc` feeds the fold's Status ONLY for a
    // VALID Query-class site (the guard's own rc); an establish site's rc is the PROBE
    // command's (dpkg-query's), NOT the mutator's, so it feeds the fold NOTHING.
    let fixpoint_cap = u32::try_from(origin.classes.len())
        .unwrap_or(u32::MAX)
        .max(1);
    let settled = settle_validity_fixpoint(
        &frozen,
        &probe,
        results,
        origin,
        fixpoint_cap,
        &mut interner,
        &mut arena,
    );
    debug_assert!(
        !settled.capped,
        "the validity fixpoint hit its site-count cap — erasure stopped being monotone"
    );
    let round = settled.round;
    let classes = round.classes;
    let kills = round.kills;
    let kill_coords = round.kill_coords;
    let fact_backings = round.fact_backings;
    let why_diags = round.why_diags;
    let classify_narrative = round.classify_narrative;
    let round_diags = round.diags;
    let (by_fact, merge_narrative, collapsed_cells) =
        (settled.by_fact, settled.merge_narrative, settled.collapsed);
    let cascades = attribute_cascades(
        &cfg.value,
        &parsed.value,
        &book_src,
        &classes,
        &settled.ledger,
        &settled.origin_validity,
    );
    report_at(advisory, "classify", book_source, &round_diags);
    // The shared-cell collapse reaches a surface (`26G:fnd-shared-auto-cell-collides`): sites that
    // reported cleanly lose their licence because a SIBLING on the same cell disagreed or could not
    // answer, and until now the only trace was an unconsumed narrative. Spanless — the cell is a
    // cross-site coordinate, so blaming any one line's caret would misattribute a shared collapse.
    report_at(
        advisory,
        "records",
        None,
        &collapsed_cells
            .iter()
            .map(|(fact, sites)| {
                Diag::new_spanless_site(DiagCode::SharedCellMeasurementsDisagree(
                    dorc_aid::diag::SharedCellMeasurementsDisagree {
                        cell: dorc_plan::fact_label(&interner, *fact),
                        sites: *sites,
                    },
                ))
            })
            .collect::<Vec<_>>(),
    );
    let probe_origins = probe_origins(&probe, results, &mut arena);

    // The survival tier (Stage 2 / rul24-mode-gate, TC-1): footprints are lifted ONLY under
    // `--risk-faultless-skips` — off ⇒ `None` ⇒ the honest Stage-1 total wall, the data never exists.
    let survival = args.risk_faultless_skips.then(|| {
        let touches = lift_touches_sets(&oracle_refs, &mut interner);
        report_at(advisory, "touches", None, &touches.diags);
        let lifted = build_survival_footprints(
            &touches.value,
            &classes,
            &kills,
            &kill_coords,
            &value,
            &cfg.value,
            &parsed.value,
            &mut interner,
        );
        report_at(advisory, "footprint", None, &lifted.diags);
        let mut fps = lifted.value;
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
        report_at(
            advisory,
            "derive",
            book_source,
            &merge_derived_footprints(
                &mut fps,
                &derivations,
                results,
                &classes,
                &kill_coords,
                &derived_node_spans,
                &mut interner,
            ),
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
    //
    // Still per-PROVIDER now that a verdict body can also key an authored cell: the fence guards
    // the synthetic singleton, so every provider that could mint one keeps it.
    let verdict_names: Vec<String> = verdicts
        .providers()
        .map(|p| interner.resolve(p.0).to_owned())
        .collect();
    for name in verdict_names {
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
        args.risk_faultless_skips.then_some(&resolutions),
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

    let identity_diags: Vec<Diag> = round_diags
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
            host: framing.host().to_owned(),
            book: book_name.to_owned(),
            book_digest: book_digest(&book_src),
            at_head: source_match::resolve(
                &source_match::GitRepository,
                std::path::Path::new(book_name),
            ),
            oracles: oracle_paths.clone(),
            risk_profile: args.risk_faultless_skips.then_some(CONSENT_FLAG),
            tally: PlanTally::Derived(plan.disposition_counts()),
            deepest_tier: args.all,
            // Only a replay can disagree, and it declares its stream rather than being assumed.
            narratable: replay
                .as_ref()
                .is_none_or(|r| r.record_stream_version == dorc_aid::narrative::PLANE_VERSION),
        };
        print!(
            "{}",
            why_report_parts(
                &render_ctx(),
                &WhyReport {
                    address: args.why_address.as_deref(),
                    plan: &plan,
                    probe: &probe,
                    first_wall: first_wall.as_ref(),
                    wall_steps: &wall_steps,
                    why_diags: &why_diags,
                    refusals: &refusals,
                    arena: &arena,
                    ast: &parsed.value,
                    book_src: &book_src,
                    filename: book_name,
                    interner: &interner,
                    oracle_paths: &oracle_paths,
                    oracle_srcs: &oracle_srcs,
                    narrative: &collapse_narrative,
                    cascades: &cascades,
                    receipt: &receipt,
                }
            )
            .text()
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
    /// The recorded book digest disagrees with the file now at that path: answerable, degraded.
    Drifted(Box<DriftedReceipt>),
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
    reason = "one linear admission ladder: select the durable, bound it, read back the book and oracles it names, check the framing, then admit the records. Every rung answers on its own terms — refusing, or in the book-digest rung's case degrading — and splitting it would scatter the ONE place a replay's inputs are validated"
)]
fn load_whylog_replay(args: &Args) -> Result<Carrier<ReplayLoad>, Diag> {
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
            return Ok(Carrier::new(
                ReplayLoad::Refused,
                vec![Diag::new_spanless_site(DiagCode::WhylogAbsent(
                    dorc_aid::diag::WhylogAbsent {
                        dir: dir.to_owned(),
                    },
                ))],
            ));
        };
        path
    };
    let Ok(file) = std::fs::File::open(&path) else {
        return Ok(refuse_replay(dorc_plan::records::AdmissionRefusal::Framing));
    };
    let envelope = match dorc_plan::whylog::admit_unscoped_whylog(
        file,
        dorc_plan::whylog::WhylogLimits::spike_default(),
    ) {
        dorc_plan::records::Admission::Admitted(envelope) => envelope,
        dorc_plan::records::Admission::NoObservation => {
            return Ok(refuse_replay(dorc_plan::records::AdmissionRefusal::Framing));
        }
        dorc_plan::records::Admission::Refused(reason) => {
            return Ok(refuse_replay(reason));
        }
    };
    let book_path = envelope.recorded_book_path().as_str().to_owned();
    let oracle_paths: Vec<String> = envelope
        .recorded_oracles()
        .iter()
        .map(|oracle| oracle.path().as_str().to_owned())
        .collect();
    let Ok(book) = read_replay_source(&book_path) else {
        return Ok(refuse_replay(dorc_plan::records::AdmissionRefusal::Framing));
    };
    let oracle_sources: Vec<String> = match oracle_paths.iter().map(read_replay_source).collect() {
        Ok(sources) => sources,
        Err(()) => {
            return Ok(refuse_replay(dorc_plan::records::AdmissionRefusal::Framing));
        }
    };
    let framing = dorc_plan::records::Framing::spike(book_digest(&book));
    let scope = dorc_cli::results::replay_scope(
        &framing,
        &dorc_cli::results::RunSources {
            book_name: &book_path,
            book: &book,
            oracle_paths: &oracle_paths,
            oracle_sources: &oracle_sources,
        },
    );
    // An edited book is the ordinary mismatch, so it is NAMED rather than reported as generic
    // framing — and it is the ENTRY to the degraded receipt (`28F:rul-drift-replay-d1`) rather than
    // a dead end. The diag still fires: drift loud on the report lane, receipt on stdout.
    if envelope.claims().book_digest() != scope.book_digest() {
        return Ok(Carrier::new(
            ReplayLoad::Drifted(Box::new(dorc_cli::drifted_receipt(&envelope))),
            vec![Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                dorc_aid::diag::WhylogBookDesync {
                    which: "book".to_owned(),
                },
            ))],
        ));
    }
    if !scope.matches_claims(&envelope) {
        return Ok(refuse_replay(dorc_plan::records::AdmissionRefusal::Framing));
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
        dorc_plan::records::Admission::Admitted(replay) => {
            Ok(Carrier::pure(ReplayLoad::Admitted(Replay {
                book_path,
                oracle_paths,
                decision_digest,
                started_at,
                record_stream_version,
                instants: instants.clone(),
                records: Some(replay.records().clone()),
            })))
        }
        dorc_plan::records::Admission::NoObservation => {
            Ok(Carrier::pure(ReplayLoad::NoObservation(Replay {
                book_path,
                oracle_paths,
                decision_digest,
                started_at,
                record_stream_version,
                instants: instants.clone(),
                records: None,
            })))
        }
        dorc_plan::records::Admission::Refused(reason) => Ok(refuse_replay(reason)),
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

fn refuse_replay(reason: dorc_plan::records::AdmissionRefusal) -> Carrier<ReplayLoad> {
    Carrier::new(ReplayLoad::Refused, vec![reason.spanless_diagnostic()])
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
        nonce: framing.nonce().0.clone(),
        attempt: framing.attempt(),
        host: framing.host().to_owned(),
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
) -> Carrier<(dorc_plan::Vouches, Vec<CollapseNarrative>)> {
    // The composition lives in `dorc_plan::build_vouches` (the ONE home — the sweep/coverage DSTs
    // share it). This edge only RESHAPES the lift: its diagnostics ride out AS-IS (inv-top-reject —
    // the tc-verdict-return softening is reverted, find-return-vouches 24C), so a genuinely
    // out-of-dialect verdict body fails gate-3's error-floor rather than degrading silently.
    let (lifted, decline_narrative) =
        dorc_plan::build_vouches(oracle_refs, classes, value, interner);
    lifted.map(|vouches| (vouches, decline_narrative))
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

    // The REAL (worth-disclosing) unresolvable sites, in the probe's site order — each with the
    // tracer's give-up reason where one exists (`26G:fnd-existence-gate-darkens-oracle`: naming the
    // site without the cause is what let a `|| return 2` gate darken a whole oracle in silence).
    let mut real: Vec<(dorc_plan::LeafId, dorc_core::Span, String)> = Vec::new();
    for &leaf in &probe.unresolvable {
        let Some(&id) = ast_of_leaf.get(&leaf) else {
            debug_assert!(
                false,
                "unresolvable site has no plan step -- unresolvable is a subset of plan.steps by \
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
    let names: Vec<String> = real
        .iter()
        .map(|(leaf, _, t)| match probe.unresolvable_causes.get(leaf) {
            Some(cause) => format!("`{t}` ({})", cause.as_str()),
            None => format!("`{t}`"),
        })
        .collect();
    let first_text = book_src
        .get(first_span.lo.0 as usize..first_span.hi.0 as usize)
        .unwrap_or("<source unavailable>");
    vec![Diag::new(
        DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: SiteId::leaf(first_leaf),
            count: real.len().to_string(),
            site_word: if real.len() == 1 { "site" } else { "sites" },
            names: dorc_aid::ForeignBytes::from_io_edge(&names.join(", ")),
            excerpt: dorc_aid::ForeignBytes::from_io_edge(first_text),
        }),
        first_span,
    )]
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
    for reason in why_lens_reasons(why_diags, arena, src) {
        eprintln!("why: {}", why_lens_line(&reason));
    }
}

/// The stderr lens's render seat: one reason's fragments, stamped as runs and concatenated.
///
/// It stamps the SAME runs the `dorc why` report hands weft (`Said::runs`), and then throws the
/// attribution away, because a bare stderr line has no span map to carry it. Going through the
/// stamp anyway is what keeps the two surfaces from drifting — every fragment is classed once, so
/// the book's own bytes are encoded here for the same reason they are there
/// (`ask-why-lens-stderr-unencoded`).
fn why_lens_line(reason: &Said) -> String {
    reason
        .runs(&render_ctx(), "why-lens")
        .iter()
        .map(|run| run.text.as_str())
        .collect()
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

/// [`dorc_cli::drifted_why_parts`], printed — the whole of this edge's share of the degraded
/// report (`28H:prop-drifted-why-is-the-thin-driver`). The composition lives across the seam so a
/// loom case can drive the same render and carry an editable transcript of it.
fn emit_drifted_why(address: Option<&str>, drifted: &DriftedReceipt) {
    print!(
        "{}",
        dorc_cli::drifted_why_parts(&render_ctx(), address, drifted).text()
    );
}

/// The why-lens reasons + stage-4 dedup, factored PURE (the stderr side is [`emit_why_lens`]) so
/// the dedup is unit-testable (`x2-fd1`). For each caused-⊤ diag it takes the "why did this run"
/// reason from [`dorc_aid::diag::why`], showing a given cause-SITE once.
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
fn why_lens_reasons(why_diags: &[Diag], arena: &ProvArena, src: &str) -> Vec<Said> {
    let mut shown: Vec<(dorc_core::ProvId, dorc_aid::diag::SiteId)> = Vec::new();
    let mut reasons = Vec::new();
    for diag in why_diags {
        if let Some(key) = cmdsub_cause_site(diag) {
            if shown.contains(&key) {
                continue; // stage-4: this (cause, site) was already explained — show it once
            }
            shown.push(key);
        }
        if let Some(explanation) = dorc_aid::diag::why(&render_ctx(), diag, arena, src) {
            reasons.push(Said::Parts(explanation.parts));
        }
    }
    reasons
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
        let lines = super::why_lens_reasons(&diags, &arena, "apt_install \"$(curl a)\"");
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
        let lines = super::why_lens_reasons(&diags, &arena, "apt-get install \"$(date)\"");
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
/// The third product is the SHARED-CELL COLLAPSE readout (`26G:fnd-shared-auto-cell-collides`):
/// each cell whose cross-site merge above degraded a channel, with how many sites measured it.
/// `validity` is the PER-ROUND validity view (`26H` §4): the probe is the frozen origin
/// artifact and its baked `valid` bits are round 1's, so once an erasure removes an upstream
/// invalidator this view is what makes the guard's already-measured rc fold-usable. It is the
/// only thing that moves between rounds — an erased site keeps contributing its measurement,
/// because the deadness of the line that measured the world does not un-measure the world. An
/// EMPTY view means "use the baked bit", which is exactly round-1 semantics.
///
/// Decision-inert and cell-keyed, so the caller renders ONE line per cell rather than one per
/// disagreeing pair — the collapse is a property of the cell, and an admin who sees it per-pair
/// reads N unrelated problems instead of one shared one. It exists because this de-licenses sites
/// that reported perfectly well, and until now said nothing at all.
/// The FROZEN inputs of the validity fixpoint (`26H` §4¾): carried verbatim across every
/// round, never re-derived and never re-admitted. Book, CFG, spans, value-flow, the effect
/// map, the oracle lifts. The admitted records and the compiled probe are frozen too, and
/// ride beside this rather than in it — they belong to the intake edge, not the model.
struct FrozenModel<'a> {
    cfg: &'a dorc_analysis::cfg::Cfg,
    value: &'a dorc_analysis::value::ValueFlow,
    ast: &'a dorc_syntax::ast::Ast,
    idx: &'a dorc_oracle::KindIndex,
    checks: &'a [dorc_oracle::predict::PredictSet],
    verdicts: &'a dorc_oracle::verdict::VerdictIndex,
    peeled: &'a BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_analysis::effect::PeeledSite>,
}

/// One round's PURE DERIVATION from (frozen inputs, erasure ledger) — recomputed from
/// scratch every round, never incrementally patched (`26H` §4¾). Every field here is a
/// function of the residual model alone.
struct ClassifiedRound {
    classes: Vec<(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )>,
    diags: Vec<Diag>,
    why_diags: Vec<Diag>,
    kills: BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    kill_coords: BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    fact_backings: BTreeMap<dorc_core::FactKey, dorc_core::FactBacking>,
    classify_narrative: Vec<CollapseNarrative>,
    invalidators: BTreeSet<dorc_analysis::cfg::CfgNodeId>,
}

/// What the fixpoint settled on: the FINAL round's model and observations, plus the ledger
/// that produced it. Nothing from any earlier round is here — earlier rounds construct only
/// a classification and a fold, never a plan, a narrative surface, or a render.
struct SettledFixpoint {
    round: ClassifiedRound,
    by_fact: BTreeMap<dorc_core::FactKey, Observable>,
    merge_narrative: Vec<CollapseNarrative>,
    collapsed: BTreeMap<dorc_core::FactKey, u32>,
    ledger: dorc_plan::erase::ErasureLedger,
    /// Did the loop hit its cap and degrade to origin? Unreachable at the production bound, so
    /// the caller `debug_assert`s it false; the fault-injection pin drives it true deliberately.
    capped: bool,
    /// Round 1.s validity bits — the ORIGIN model's answer, kept so the why-chain can tell a
    /// site that was always trustworthy from one whose guard only became trustworthy because
    /// something upstream was proven dead. The latter is the cascade `26H` §4.6 requires be
    /// renderable, and it is the only reason any round-1 quantity outlives its round.
    origin_validity: BTreeMap<dorc_plan::LeafId, bool>,
}

/// Attribute every round-2+ validity flip to the erasures that caused it.
///
/// A guard becomes valid exactly when every invalidator reaching it has been erased, so the
/// cause of site `L`'s flip is precisely the ledger entries whose sites REACH `L` in the
/// control-flow graph. Computed once, after quiescence, over the frozen CFG — forward
/// reachability from each erased site, which is exact and cheap next to the network this
/// whole engine exists to avoid.
fn attribute_cascades(
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    ledger: &dorc_plan::erase::ErasureLedger,
    origin_validity: &BTreeMap<dorc_plan::LeafId, bool>,
) -> BTreeMap<dorc_plan::LeafId, CascadeAttribution> {
    let line_of_node = |node: dorc_analysis::cfg::CfgNodeId| {
        let lo = ast.node(cfg.node(node).ast).span.lo.0 as usize;
        dorc_aid::diag::line_col(book_src, lo).0
    };
    let mut out = BTreeMap::new();
    for (leaf, (node, class)) in classes.iter().enumerate() {
        let Ok(leaf) = u32::try_from(leaf) else {
            continue;
        };
        let leaf = dorc_plan::LeafId(leaf);
        if !matches!(
            class,
            dorc_analysis::effect::SkipClass::QueryResolvable { valid: true, .. }
        ) || origin_validity.get(&leaf) != Some(&false)
        {
            continue;
        }
        let causes: Vec<&dorc_plan::erase::ErasureEntry> = ledger
            .entries()
            .filter(|entry| reaches(cfg, entry.site(), *node))
            .collect();
        let Some(last) = causes.iter().max_by_key(|entry| entry.round()) else {
            continue;
        };
        out.insert(
            leaf,
            CascadeAttribution {
                erased_lines: causes.iter().map(|e| line_of_node(e.site())).collect(),
                controller_line: dorc_aid::diag::line_col(
                    book_src,
                    ast.node(last.proof().controller()).span.lo.0 as usize,
                )
                .0,
                round: last.round().0,
            },
        );
    }
    out
}

/// Is `to` reachable from `from` in the CFG? A plain forward walk over the frozen graph.
fn reaches(
    cfg: &dorc_analysis::cfg::Cfg,
    from: dorc_analysis::cfg::CfgNodeId,
    to: dorc_analysis::cfg::CfgNodeId,
) -> bool {
    use dorc_analysis::solve::Graph as _;
    let mut seen = vec![false; cfg.node_count()];
    let mut stack = vec![from];
    while let Some(node) = stack.pop() {
        for next in cfg.succ_ids(node) {
            if next == to {
                return true;
            }
            if seen.get(next.index()) == Some(&false) {
                if let Some(slot) = seen.get_mut(next.index()) {
                    *slot = true;
                }
                stack.push(next);
            }
        }
    }
    false
}

/// Classify the residual model named by `erased` (round 1 passes the empty overlay).
fn classify_round(
    frozen: &FrozenModel<'_>,
    erased: &dorc_analysis::erase::ErasedSites,
    interner: &mut Interner,
    arena: &mut ProvArena,
    degrades: &mut BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_oracle::predict::TopReason>,
    verdict_lane: &mut BTreeSet<dorc_analysis::cfg::CfgNodeId>,
) -> ClassifiedRound {
    let (
        classified,
        why_diags,
        kills,
        kill_coords,
        fact_backings,
        classify_narrative,
        invalidators,
    ) = dorc_analysis::effect::classify_with_why_diags(
        frozen.cfg,
        frozen.value,
        frozen.ast,
        frozen.idx,
        frozen.checks,
        frozen.verdicts,
        frozen.peeled,
        erased,
        interner,
        arena,
        degrades,
        verdict_lane,
    );
    ClassifiedRound {
        classes: classified.value,
        diags: classified.diags,
        why_diags,
        kills,
        kill_coords,
        fact_backings,
        classify_narrative,
        invalidators,
    }
}

/// The per-round VALIDITY VIEW: each Query leaf's `valid` bit, as this round's residual model
/// computes it. `classes` is leaf-ordered (the positional assignment `build_plan` and
/// `build_vouches` share), so the index IS the site's [`dorc_plan::LeafId`].
///
/// Round 1's view necessarily equals the bits baked into the frozen probe, which is what
/// keeps a world with nothing to erase byte-identical.
fn validity_view(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
) -> BTreeMap<dorc_plan::LeafId, bool> {
    classes
        .iter()
        .enumerate()
        .filter_map(|(leaf, (_, class))| match class {
            dorc_analysis::effect::SkipClass::QueryResolvable { valid, .. } => {
                Some((dorc_plan::LeafId(u32::try_from(leaf).ok()?), *valid))
            }
            _ => None,
        })
        .collect()
}

/// Run the validity fixpoint to quiescence (`26H` §4 — W-C, the flagship fix).
///
/// Round k derives the residual model from origin + ledger, re-folds the FROZEN records
/// through it, and appends every newly-proven-dead site; the loop ends when a round proves
/// nothing new. Monotone by construction (erasure only ever REMOVES invalidators, so a
/// query can only become valid, so a fold can only find more deadness) and bounded by the
/// site count, since every growing round adds at least one of finitely many sites. The cap
/// is therefore unreachable; it exists so a monotonicity regression cannot become a hang.
/// Hitting it DISCARDS the whole ledger and re-derives from the origin, so the run's answer is
/// exactly the pre-W-C one: no elision rests on a half-settled state nobody reasoned about, and
/// there is no partial fixpoint to be silent about. A `debug_assert` makes it loud in dev and
/// under DST — the same bargain `solve` strikes for its own unenforceable termination.
///
/// NO RE-PROBE (`26H` §0 v-no-reprobe-needed): invalid-Query checks already ship and their
/// rcs are already measured, merely withheld. This consumes measurements in hand; it never
/// asks a host anything, and `probe` is the frozen origin artifact throughout.
fn settle_validity_fixpoint(
    frozen: &FrozenModel<'_>,
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
    origin: ClassifiedRound,
    cap: u32,
    interner: &mut Interner,
    arena: &mut ProvArena,
) -> SettledFixpoint {
    let mut ledger = dorc_plan::erase::ErasureLedger::new();
    let origin_validity = validity_view(&origin.classes);
    let mut round = origin;
    let mut number = 1u32;
    loop {
        let validity = validity_view(&round.classes);
        let (by_fact, merge_narrative, collapsed) = facts_from_sites(probe, results, &validity);
        let observe = |f: dorc_core::FactKey| {
            by_fact
                .get(&f)
                .copied()
                .unwrap_or(Observable::verdict_only(Verdict::Unknown))
        };
        let proofs = dorc_plan::erase::prove_dead_branches(
            frozen.ast,
            frozen.cfg,
            &round.classes,
            &round.invalidators,
            observe,
        );
        let before = ledger.len();
        for proof in proofs {
            ledger.record(proof, dorc_plan::erase::RoundId(number));
        }
        let grew = ledger.len() > before;
        if !grew {
            return SettledFixpoint {
                round,
                by_fact,
                merge_narrative,
                collapsed,
                ledger,
                capped: false,
                origin_validity,
            };
        }
        if number >= cap {
            let discarded = u32::try_from(ledger.len()).unwrap_or(u32::MAX);
            ledger.rebuild_from_origin();
            let round = classify_round(
                frozen,
                &ledger.overlay(),
                interner,
                arena,
                &mut BTreeMap::new(),
                &mut BTreeSet::new(),
            );
            let validity = validity_view(&round.classes);
            let (by_fact, mut merge_narrative, collapsed) =
                facts_from_sites(probe, results, &validity);
            // Withdrawing licensed elisions is a safety-narrowing like any other, so it narrates.
            merge_narrative.push(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::FixpointCapDegrade {
                    rounds: number,
                    discarded,
                },
            ));
            return SettledFixpoint {
                round,
                by_fact,
                merge_narrative,
                collapsed,
                ledger,
                capped: true,
                origin_validity,
            };
        }
        number = number.saturating_add(1);
        round = classify_round(
            frozen,
            &ledger.overlay(),
            interner,
            arena,
            &mut BTreeMap::new(),
            &mut BTreeSet::new(),
        );
    }
}

/// Ship an already-rendered plan to a host and report how it ended.
///
/// The one path in this binary that runs a mutating artifact somewhere. Three properties are
/// load-bearing and each is visible in the shape below: the artifact arrives already rendered
/// (nothing here can build one); it is shipped exactly ONCE, with no loop and no retry parameter
/// (`law-no-double-apply`); and a lost session reports Unknown rather than a failure, because the
/// remote may have done everything, nothing, or half of it.
#[expect(
    clippy::result_large_err,
    reason = "the Err is a full `Diag`, as everywhere on this once-per-process path"
)]
fn ship_consented_apply(args: &Args, host: &str) -> Result<RunOutcome, Diag> {
    let artifact = if let Some(path) = args.plan.as_deref() {
        std::fs::read(path).map_err(|e| humane_read_error("plan", path, &e))?
    } else {
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut std::io::stdin(), &mut bytes)
            .map_err(|e| humane_read_error("plan", "<stdin>", &e))?;
        bytes
    };
    let destination =
        dorc_transport::HostId::new(host).map_err(|_| transport_edge::host_rejected(host))?;
    let mut driver = transport_edge::driver_for_invocation(
        args.connect_timeout,
        args.accept_new,
        args.ssh_config.as_deref(),
    );
    let timeout = args.apply_timeout.map(std::time::Duration::from_secs);

    match transport_edge::apply_to_host(
        driver.as_mut(),
        &destination,
        &transport_edge::mint_nonce(),
        &artifact,
        timeout,
    )? {
        transport_edge::AppliedOutcome::Ran { status } => {
            if status == 0 {
                Ok(RunOutcome::Complete)
            } else {
                report_at(
                    true,
                    "apply",
                    None,
                    &[transport_edge::apply_failed(host, status)],
                );
                Ok(RunOutcome::ApplyFailed)
            }
        }
        transport_edge::AppliedOutcome::Unknown { diagnosis } => {
            report_at(
                true,
                "apply",
                None,
                &[transport_edge::session_lost(host, 1, &diagnosis)],
            );
            Ok(RunOutcome::SessionLost)
        }
        transport_edge::AppliedOutcome::NotAttempted { reason } => {
            report_at(
                true,
                "apply",
                None,
                &[transport_edge::not_attempted(host, &reason)],
            );
            Ok(RunOutcome::HostNotReached)
        }
    }
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

/// The comparison key that lets a LOADED oracle path and a DISCOVERED one denote the same file.
///
/// `289:rider-sibling-note-false-fires-relative`: the loaded set carries `-o` args verbatim
/// (`firewall.oracle.sh`) while discovery yields `read_dir` paths (`./firewall.oracle.sh`), so a raw
/// string compare reported every relatively-named oracle as unloaded. Both spellings converge here
/// by dropping `.` components and spelling every separator `/`.
///
/// The separator fold happens BEFORE the components are read, because `\` is a separator only on
/// Windows: a Unix `Path` reads `oracles\fw.oracle.sh` as one nameless-parent file whose name
/// contains a backslash, so folding afterwards would leave a `./` the forward-slash spelling of the
/// same path never grows (`one-platform-green-is-not-cross-platform-green`).
///
/// Deliberately textual, not `canonicalize`: this feeds a HINT, and a hint must not acquire the
/// power to touch the filesystem or to fail. Two spellings of one path through different symlinks
/// still miss, which costs a suppressed hint and never a wrong one.
fn oracle_path_key(path: &str) -> String {
    use std::path::{Component, Path, PathBuf};

    let slash_separated = path.replace('\\', "/");
    let keyed: PathBuf = Path::new(&slash_separated)
        .components()
        .filter(|c| !matches!(c, Component::CurDir))
        .collect();
    let keyed = keyed.to_string_lossy().replace('\\', "/");
    if keyed.is_empty() {
        ".".to_string()
    } else {
        keyed
    }
}

/// The unloaded-sibling-oracle hint (`AID-NEEDS:aid-unloaded-sibling-oracle`, gap-5 / `24H`
/// ack-6): scan the directories of the loaded oracles + the book(s) for `*.oracle.sh` files that were
/// NOT loaded, and disclose them (suggest, never auto-load). A cli-edge disclosure — it reads the
/// filesystem, so it lives here, never in the kernel; the `read_dir` order is OS-dependent, so the
/// result is SORTED (`inv-determinism` at the edge). The payload's `detail` carries the DATA (the
/// sorted backtick-quoted path list); the user-facing framing prose stays `[unwritten:]` for the
/// conductor (`27V:rul-error-authorship-tier` — the builder authors no user-facing prose).
fn unloaded_sibling_oracle_diagnostics(books: &[String], oracle_paths: &[String]) -> Vec<Diag> {
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
        return Vec::new();
    }
    unloaded.sort();
    let oracles = unloaded
        .iter()
        .map(|p| format!("`{p}`"))
        .collect::<Vec<_>>()
        .join(", ");
    vec![Diag::new_spanless_site(DiagCode::AidUnloadedSiblingOracle(
        AidUnloadedSiblingOracle { oracles },
    ))]
}

/// The escalation-POLICY disclosure (`27C:render-authority-disclosure` — the consent-legibility
/// line). Names the escalation posture the dial + capability set, and the entry-capable wrappers
/// loaded (a wrapper authoring BOTH a peeling `__predict` and an `__enter` form). One `Note` the
/// edge reports (advisory), never a gate.
///
/// SCOPE (honest for the spike): this is the POLICY in effect, not a per-book-SITE "will enter"
/// tally — the book-side entry-composed probe emission (which would count sites per entered context)
/// is the deferred integration (`27K` §9 / this lane's report). The dial × capability × the loaded
/// entry forms are all real; what is missing is the per-site consumption in the probe pipeline.
fn escalation_policy_diagnostics(
    interner: &mut Interner,
    oracle_refs: &[&str],
    dial: dorc_core::EscalationDial,
    capability: dorc_core::Capability,
) -> Vec<Diag> {
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
        // no entry-capable wrapper loaded ⇒ no escalation is possible ⇒ nothing to disclose
        return Vec::new();
    }
    let head_list = heads.values().cloned().collect::<Vec<_>>().join(", ");
    vec![Diag::new_spanless_site(DiagCode::EscalationPolicy(
        EscalationPolicy {
            dial,
            capability,
            entry_forms: head_list,
        },
    ))]
}

/// Whether a predict body peels (a wrapper) — the `detect_peel`-present predicate, factored so the
/// entry-policy scan reuses it (`inv-referent-agnostic`: structural, never decodes the command).
fn detect_peel_present(p: &dorc_oracle::predict::Predict) -> bool {
    dorc_oracle::wrapper::detect_peel(p).is_some()
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
            &render_ctx(),
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
        // The three spellings must land on ONE key, not merely agree pairwise: the bug this replaced
        // folded separators AFTER reading components, so on Unix the backslash spelling grew a `./`
        // prefix its forward-slash twin never had, and only the platform that splits `\` was green.
        assert_eq!(
            oracle_path_key("oracles\\fw.oracle.sh"),
            oracle_path_key("./oracles/fw.oracle.sh"),
            "a leading `.` is dropped at any depth, on either platform"
        );
        assert_ne!(
            oracle_path_key("a/fw.oracle.sh"),
            oracle_path_key("b/fw.oracle.sh"),
            "same basename in different dirs stays distinct — the hint must still fire"
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
    /// PROVIDER name is KEPT but flagged (the mis-key).
    ///
    /// The conflict's DIAGNOSTIC is asserted here as a returned value, keyed to the first declaring
    /// file. It used to be asserted only end-to-end through the binary, which meant this test drove
    /// a helper that printed it: a red caret frame across a green run, and a diagnostic whose
    /// presence nothing local pinned. Both are the same bug — a decision-carrying helper writing to
    /// fd 2 (`io-at-edges-only`).
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
        let kr = build_kind_resolvers(&clean, &checks, &touches_paired, &coord_kinds, &mut i);
        assert!(
            kr.value.resolver_kinds().any(|k| i.resolve(k) == "package"),
            "a clean package resolver is resolver-bearing"
        );
        assert!(
            kr.lift.is_empty() && kr.confusability.is_empty(),
            "a clean resolver raises nothing"
        );

        // Two files, both package resolvers ⇒ BOTH refused (no resolver kind).
        let dup = vec![
            "package__resolve() { printf '%s\\n' \"$1\"; }".to_string(),
            "package__resolve() { printf '%s\\n' \"$1\"; }".to_string(),
        ];
        let kr_dup = build_kind_resolvers(&dup, &checks, &touches_paired, &coord_kinds, &mut i);
        assert_eq!(
            kr_dup.value.resolver_kinds().count(),
            0,
            "a duplicate resolver for one kind refuses BOTH (token-equality floor)"
        );
        let dup_diags = kr_dup
            .confusability
            .get(&0)
            .expect("the conflict frames into the FIRST declaring file");
        assert!(
            matches!(dup_diags.as_slice(), [d] if matches!(&d.code, DiagCode::ResolverConflict(c) if c.kind == "package" && c.count == 2)),
            "the refusal comes back as a diagnostic VALUE, never a write to fd 2"
        );

        // A resolver whose kind munges to the known provider "apt-get" (base `apt_get`) ⇒ KEPT
        // (warned, not a silent dud) — the collision is now detected in NAME space, and a raw
        // `apt_get` coord kind re-keys it as bearing.
        let collide = vec!["apt_get__resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr_col = build_kind_resolvers(&collide, &checks, &touches_paired, &coord_kinds, &mut i);
        assert!(
            kr_col
                .value
                .resolver_kinds()
                .any(|k| i.resolve(k) == "apt_get"),
            "a provider-named resolver is kept (the collision is a warning, not a refusal)"
        );
        assert!(
            matches!(
                kr_col.confusability.get(&0).map(Vec::as_slice),
                Some([d]) if matches!(&d.code, DiagCode::ResolverProviderCollision(c) if c.name == "apt_get")
            ),
            "the mis-key comes back as a diagnostic VALUE too"
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
            drop(merge_derived_footprints(
                &mut fps,
                &derivations,
                &results,
                &[],
                &BTreeMap::new(),
                &node_spans,
                i,
            ));
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
            unresolvable_causes: BTreeMap::new(),
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

    /// `28F:rul-drift-replay-d1`: a drifted receipt's ONLY count comes from the durable's stored
    /// disposition WORDS, so this fold is the whole tally. Two things worth pinning: the word
    /// keying (rename a tag at the writer and a silently-zeroed tally is what a reader would see —
    /// the drifted receipt has no second source to disagree with), and that an unrecognized word
    /// lands in no bucket rather than a guessed one, so the tally under-reports rather than
    /// mis-reports (`271:rul-sin-ordering`). `omit` is absent from the render by design: the
    /// receipt tally has always been ran/guarded/skipped.
    #[test]
    fn a_drifted_tally_counts_the_stored_words_and_guesses_at_none() {
        let line = |leaf: u32, disposition: &str| dorc_plan::whylog::ApplyLine {
            leaf,
            disposition: disposition.to_owned(),
            predicted: true,
        };
        let tally = dorc_cli::recorded_tally(&[
            line(0, "run"),
            line(1, "replace"),
            line(2, "replace"),
            line(3, "guard"),
            line(4, "omit"),
            line(5, "a-word-no-writer-emits"),
        ]);
        let PlanTally::DriftedUnsplit { run, guard, elide } = tally else {
            panic!("a recorded tally is always the unsplit, drifted shape");
        };
        assert_eq!((run, guard, elide), (1, 1, 2));
        assert!(
            tally.is_drifted(),
            "the unsplit tally IS the receipt's drift state — nothing else carries it"
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
            facts_from_sites(&entry_probe, &parse_str(records, i), &BTreeMap::new())
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
            !facts_from_sites(&plain, &results, &BTreeMap::new())
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

    /// Two same-command sites on DISTINCT authored cells fold independently, and the fold does not
    /// care what order their records arrived in (`26H` §3.6).
    ///
    /// This is the behavioural close of `26G`'s cold trail. Before W-B both sites shared the
    /// per-provider auto-cell, and which of the pair kept a disposition depended on their relative
    /// POSITION (the later site read as written-upstream by its own sibling). Distinct keys remove
    /// the coupling at the root, so there is no order left to depend on — and this pins that
    /// directly rather than quoting the mechanism it retired.
    #[test]
    fn distinct_authored_cells_fold_independently_whatever_the_record_order() {
        let mut i = Interner::default();
        let file = KindId(i.intern("sm.dorc.File"));
        let content = SelectorId(i.intern("content"));
        let cell = |i: &mut Interner, path: &str| {
            FactKey::cell(
                file,
                EntityRef::Operand(OpaqueToken(i.intern(path))),
                content,
            )
        };
        let a = cell(&mut i, "/etc/a.conf");
        let b = cell(&mut i, "/etc/b.conf");
        let probe = ProbePlan {
            checks: vec![
                ProbePredict {
                    site: LeafId(0),
                    member: None,
                    fact: a,
                    site_kind: ProbeSiteKind::Establish,
                    provider: file.0,
                    argv: vec![],
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: true,
                    emits_report: false,
                    entry: None,
                },
                ProbePredict {
                    site: LeafId(1),
                    member: None,
                    fact: b,
                    site_kind: ProbeSiteKind::Establish,
                    provider: file.0,
                    argv: vec![],
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: true,
                    emits_report: false,
                    entry: None,
                },
            ],
            unresolvable: vec![],
            unresolvable_causes: BTreeMap::new(),
        };
        // The finding's own amplification world: one site converged, its sibling unreadable.
        let forward = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=cant-tell rc=3\n",
            &mut i,
        );
        let reversed = parse_str(
            "site 1 effect=cant-tell rc=3\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        let (f_facts, f_narrative, _) = facts_from_sites(&probe, &forward, &BTreeMap::new());
        let (r_facts, r_narrative, _) = facts_from_sites(&probe, &reversed, &BTreeMap::new());
        assert_eq!(
            f_facts, r_facts,
            "the fold is a pure function of the records, not of their arrival order"
        );
        assert_eq!(f_narrative.len(), r_narrative.len());
        assert_eq!(
            f_facts.get(&a).map(|o| o.effect),
            Some(Verdict::Converged),
            "the converged site keeps its own answer — a sibling that could not report is not \
             evidence about THIS cell (oracle-contract §4)"
        );
        assert_eq!(
            f_facts.get(&b).map(|o| o.effect),
            Some(Verdict::Unknown),
            "the unreadable site is unknown ⇒ its own line runs, and only its own line"
        );
        assert!(
            f_narrative.is_empty(),
            "distinct cells never collided, so nothing was merged and nothing is announced"
        );
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
        let (_facts, evidence, _collapsed) = facts_from_sites(&invalid, &results, &BTreeMap::new());
        assert_eq!(
            evidence
                .iter()
                .filter(|e| matches!(e.kind(), CollapseKind::SubstitutionRefusal { .. }))
                .count(),
            1,
            "an invalid Query withhold mints one SubstitutionRefusal"
        );

        let valid = probe1(fact, ProbeSiteKind::Query { valid: true });
        let (_facts, evidence, _collapsed) = facts_from_sites(&valid, &results, &BTreeMap::new());
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
            unresolvable_causes: BTreeMap::new(),
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
        let (_facts, evidence, _collapsed) = facts_from_sites(&probe, &conflict, &BTreeMap::new());
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
        let (_facts, evidence, _collapsed) = facts_from_sites(&probe, &agree, &BTreeMap::new());
        assert!(evidence.is_empty(), "agreeing records mint no disagreement");
    }

    /// The shared-cell readout is CELL-keyed, not pair-keyed (`26G:fnd-shared-auto-cell-collides`):
    /// three sites on one cell disagreeing pairwise is ONE collapse an admin can act on, and three
    /// lines would read as three unrelated problems. The count travels so the line can say how wide
    /// the collapse is.
    #[test]
    fn a_shared_cell_collapse_reports_once_however_many_sites_disagree() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let mut probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let mut third = probe.checks[0].clone();
        third.site = LeafId(2);
        probe.checks.push(third);

        let records = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\nsite 2 effect=cant-tell rc=2\n",
            &mut i,
        );
        let (_facts, _evidence, collapsed) = facts_from_sites(&probe, &records, &BTreeMap::new());
        assert_eq!(
            collapsed.len(),
            1,
            "one entry, not one per pair: {collapsed:?}"
        );
        assert_eq!(
            collapsed.get(&fact),
            Some(&3),
            "and it counts every site on the cell, including the ones that agreed"
        );
    }

    /// The ladder oracle: a query verb (`:?`) and a mutating verb (`:`) — the two halves of the
    /// idiom the validity fixpoint exists to make cascade.
    const FIXPOINT_ORACLE: &str = r#"
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"@installed
}
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
      esac
   fi
}
"#;

    /// Drive the REAL fixpoint over a two-rung ladder at the given iteration `cap`.
    fn settle_ladder(cap: u32, records: &str) -> SettledFixpoint {
        let book = "dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha\n\
                    dpkg -s beta >/dev/null 2>&1 || apt-get install -y beta\n";
        let mut interner = Interner::default();
        let oracle_srcs = vec![FIXPOINT_ORACLE.to_owned()];
        let idx = dorc_oracle::lift(&mut interner, &[FIXPOINT_ORACLE]).value;
        let checks =
            vec![dorc_oracle::predict::lift_predicts(&mut interner, FIXPOINT_ORACLE).value];
        let verdicts = dorc_oracle::verdict::VerdictIndex::default();
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut interner);
        let mut arena = ProvArena::new();
        let peeled = BTreeMap::new();
        let frozen = FrozenModel {
            cfg: &cfg,
            value: &value,
            ast: &parsed.value,
            idx: &idx,
            checks: &checks,
            verdicts: &verdicts,
            peeled: &peeled,
        };
        let origin = classify_round(
            &frozen,
            &dorc_analysis::erase::ErasedSites::none(),
            &mut interner,
            &mut arena,
            &mut BTreeMap::new(),
            &mut BTreeSet::new(),
        );
        let probe = {
            let ship = |p, a: &[Symbol]| ship_predict_body(&oracle_srcs, &checks, &interner, p, a);
            dorc_plan::compile_probe(
                &parsed.value,
                &cfg,
                &value,
                &origin.classes,
                &BTreeMap::new(),
                &dorc_plan::ConnectedPipes::default(),
                ship,
                |_, _, _| None,
                |_| false,
            )
        };
        let results = parse_str(records, &mut interner);
        settle_validity_fixpoint(
            &frozen,
            &probe,
            &results,
            origin,
            cap,
            &mut interner,
            &mut arena,
        )
    }

    /// The cascade's own liveness: uncapped, rung 2's guard becomes valid because rung 1's
    /// install was proven dead — what `26G:fnd-dead-branch-still-invalidates` says must happen.
    #[test]
    fn the_fixpoint_erases_a_dead_mutator_and_revalidates_below_it() {
        let settled = settle_ladder(
            64,
            "site 0 effect=holds rc=0\nsite 1 effect=holds\n\
             site 2 effect=holds rc=0\nsite 3 effect=holds\n",
        );
        assert_eq!(settled.ledger.len(), 2, "both installs are proven dead");
        assert!(
            settled.ledger.entries().any(|e| e.round().0 >= 2),
            "the second erasure is a round-2+ finding — that IS the cascade"
        );
    }

    /// FAULT INJECTION for `CollapseKind::FixpointCapDegrade`: force the cap to 1 so a fixpoint
    /// that genuinely wants a second round trips it. The degrade must discard EVERY erasure (the
    /// answer becomes the pre-W-C one, never a partial fixpoint) and must narrate — withdrawing
    /// licensed elisions is a safety-narrowing like any other.
    #[test]
    fn a_capped_fixpoint_degrades_to_origin_and_narrates() {
        let records = "site 0 effect=holds rc=0\nsite 1 effect=holds\n\
                       site 2 effect=holds rc=0\nsite 3 effect=holds\n";
        let capped = settle_ladder(1, records);
        assert!(
            capped.ledger.is_empty(),
            "a capped fixpoint ships NO erasure at all, not a partial set"
        );
        assert_eq!(
            capped
                .merge_narrative
                .iter()
                .filter(|e| matches!(e.kind(), CollapseKind::FixpointCapDegrade { .. }))
                .count(),
            1,
            "the degrade mints exactly one narrative"
        );
        assert!(
            !settle_ladder(64, records)
                .merge_narrative
                .iter()
                .any(|e| matches!(e.kind(), CollapseKind::FixpointCapDegrade { .. })),
            "a fixpoint that quiesces normally narrates no degrade"
        );
    }

    /// The negative pin: sites that AGREE on a shared cell produce no readout. A shared cell is
    /// ordinary and common; only its collapse is worth a line.
    #[test]
    fn an_agreeing_shared_cell_reports_nothing() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let agree = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        assert!(
            facts_from_sites(&probe, &agree, &BTreeMap::new())
                .2
                .is_empty()
        );
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
            facts_from_sites(&probe, &book, &BTreeMap::new()).0,
            facts_from_sites(&probe, &rev, &BTreeMap::new()).0,
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
            facts_from_sites(&probe, &unframed, &BTreeMap::new()).0,
            facts_from_sites(&probe, &framed, &BTreeMap::new()).0,
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
            unresolvable_causes: BTreeMap::new(),
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
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
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
            &dorc_oracle::verdict::VerdictIndex::default(),
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
        use dorc_aid::diag::{
            CfgBuiltinShadowed, RedirTargetTop, SiteId, SyntaxMalformed, SyntaxMalformedReason,
        };
        use dorc_core::{BytePos, Span};
        let span = Span::new(BytePos(0), BytePos(1));
        let mixed = vec![
            Diag::new(
                DiagCode::SyntaxMalformed(SyntaxMalformed {
                    reason: SyntaxMalformedReason::ExpectedFiToCloseIf,
                }),
                span,
            ),
            Diag::new(
                DiagCode::CfgBuiltinShadowed(CfgBuiltinShadowed {
                    name: "cd".to_owned(),
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
