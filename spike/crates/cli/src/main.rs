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
use std::io::{Read, Write};
use std::process::ExitCode;

use dorc_core::{
    Interner, Observable, OutBytes, Predicted, ProvArena, Rc, Severity, Symbol, Verdict,
};

/// The one-line usage synopsis, embedded in argument-error messages. The full
/// mode/flag/exit-code reference is [`HELP`] (printed by `--help` to stdout, exit 0).
const USAGE: &str =
    "usage: dorc [probe|plan|apply] --book=<book.sh> [-o <oracle.sh>]... [--debug-argv]";

/// The long help (ack-1 + the cheap help-is-success item): `--help`/`-h` prints this to
/// STDOUT and exits 0 (a help request is a success, not a usage error). Documents the
/// mode/flag surface AND the exit-code family the harness crash-guard mirrors.
const HELP: &str = "\
dorc — spec-mining static-analysis orchestrator (implementation spike)

usage: dorc [<mode>] --book=<book.sh> [-o <oracle.sh>]... [options]

modes (an optional leading token; default is the probe-then-apply round-trip):
  probe        emit only the read-only probe artifact (phase 1) to stdout; reads no stdin
  plan         preview the eliding apply on stdout, with the why-lens + diagnostics on stderr
  apply        emit the byte-floored, receipt-free shippable apply artifact to stdout
  why [<addr>] report (to stdout) WHY the run decided as it did — bare: the run's problems;
               `book.sh:N`: the site on that source line; free text: matching commands
  (none)       the round-trip: probe then apply on stdout, full disclosure on stderr

options:
  <book.sh>...          the book(s) to analyze — a positional path (`dorc plan book.sh`) or
                        --book=PATH / --book PATH; repeatable ⇒ concatenated as one unit
  -o, --oracle <o.sh>   an oracle file to load (repeatable; -o PATH, -oPATH, --oracle PATH)
  --oracle-dir <dir>    load every *.oracle.sh in <dir> (repeatable; glob-sorted)
  --results <file>      read the probe results from <file> (default: stdin)
  --trust-footprints    opt into the survival tier (default off)
  --debug-argv          echo the engine's per-site resolved argv to stderr
  -h, --help            print this help to stdout and exit 0
  --version             print the version to stdout and exit 0

stdin:  probe results, one per line — `site <leafid> effect=<holds|absent|cant-tell> rc=<n>`
        (unless --results <file>); stdout: the selected mode's artifact(s); stderr:
        diagnostics / why-lens / decision-digest.

exit codes:
  0    success — the analysis completed and the artifact was emitted
  2    usage error — a bad/unknown argument, a missing --book, or an unreadable file
  10   parse error — the book carries a construct dorc cannot model (a syntax-level
       ⊤-reject / CFG ⊤-node); the artifact still ships byte-identically, but the exit
       signals partial understanding so a `dorc … && deploy` chain stops. First of the
       reserved 10..19 dorc-semantic fast-fail range (vacuous/obvious, dorc-specific).
";

/// A usage/argument error, or an unreadable input file (the classic getopt convention).
const EXIT_USAGE: u8 = 2;
/// A parse-error / unmodeled book (`inv-top-reject`): the book carries a construct dorc
/// cannot model. The artifact still ships, but the exit signals partial understanding
/// (ack-1). First of the reserved 10..=19 dorc-semantic fast-fail range.
const EXIT_BOOK_UNMODELED: u8 = 10;

/// What the arg-parse resolved to: an analysis run, or a help/version request (both of which
/// are successes printed to stdout, ack-1 help-is-success — never a usage error).
enum Invocation {
    /// A normal analysis run with the parsed [`Args`].
    Analyze(Args),
    /// `-h`/`--help`: print [`HELP`] to stdout, exit 0.
    Help,
    /// `--version`: print the version to stdout, exit 0.
    Version,
}

/// The outcome of a completed analysis run — the process exit code (ack-1). `Complete` is the
/// ordinary success; `BookUnmodeled` still emitted the artifact but the book carried an
/// `inv-top-reject` construct, so the process fast-fails with [`EXIT_BOOK_UNMODELED`].
enum RunOutcome {
    /// The analysis completed cleanly ⇒ exit 0.
    Complete,
    /// The book carried a parse/CFG ⊤-reject ⇒ the artifact shipped, but exit [`EXIT_BOOK_UNMODELED`].
    BookUnmodeled,
}

fn main() -> ExitCode {
    match parse_args() {
        Ok(Invocation::Help) => {
            print!("{HELP}");
            std::io::stdout().flush().ok();
            ExitCode::SUCCESS
        }
        Ok(Invocation::Version) => {
            println!("dorc {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(Invocation::Analyze(args)) => match run(&args) {
            Ok(RunOutcome::Complete) => ExitCode::SUCCESS,
            Ok(RunOutcome::BookUnmodeled) => ExitCode::from(EXIT_BOOK_UNMODELED),
            Err(msg) => {
                eprintln!("dorc: {msg}");
                ExitCode::from(EXIT_USAGE)
            }
        },
        Err(msg) => {
            eprintln!("dorc: {msg}");
            ExitCode::from(EXIT_USAGE)
        }
    }
}

/// Which user-facing behavioral mode of the core to drive (ui-A — a fair-shape CLI over
/// the core invocation modes, NOT flag-complete; ru-25). Each maps to one of the engine's
/// distinct surfaces; `RoundTrip` is the legacy bare-flag invocation the e2e harness drives
/// (kept so the corpus stays green without a harness rewrite — the least-disruptive path).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
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
    /// stderr). The exact shape `e2e/run.sh` drives — preserved verbatim (tc-subcommand-shape).
    RoundTrip,
    /// `dorc why [<address>] …`: the WHY-query surface (ack-2). NOT an artifact-producing
    /// invocation — its report goes to STDOUT (help/version/why are their own non-analysis
    /// invocations, per the fences). Runs the full pipeline (it reports on the CURRENT run's
    /// dispositions, so it consumes stdin results like `plan`), then prints a source-line-keyed
    /// report (rul24-lineno-identity) instead of an artifact: bare ⇒ the run's PROBLEMS; a
    /// `book.sh:N` / content address ⇒ that site's cause-chain. Emits no artifact, no digest.
    Why,
}

struct Args {
    mode: Mode,
    /// The book(s) to analyze — a positional (`dorc plan book.sh`, the day-one ergonomic) OR
    /// `--book=PATH`, repeatable. Multiple books CONCATENATE into one analyzed unit (a book split
    /// across files reads as one). At least one is required.
    books: Vec<String>,
    oracles: Vec<String>,
    /// `--oracle-dir DIR` (ack-6): load every `*.oracle.sh` in DIR (glob-sorted, deterministic),
    /// repeatable — the explicit bulk form alongside `-o` for the spike.
    oracle_dirs: Vec<String>,
    /// `--results FILE` (flow pick): read the probe results from FILE instead of the default stdin.
    results: Option<String>,
    /// `--debug-argv` (gate-5 / cm-2): emit the engine's per-site resolved argv to stderr,
    /// then proceed normally — a cli-edge readout the e2e argv-echo differential consumes.
    debug_argv: bool,
    /// `--trust-footprints` (rul24-mode-gate): opt into the survival tier — a converged line
    /// may ELIDE past a RUNNING wall when the wall's authored `touches()` footprint is disjoint
    /// from the line's fact's backing (Stage 2, the golden hill). DEFAULT OFF; not recommended
    /// by hints/docs beyond noting availability. Honest framing (24A §1a-addendum): marketing at
    /// best (the admin chose the danger), theatre at worst (everyone enables it) — demanded
    /// anyway as the non-vacuous CYA. When off, the footprints are never even lifted (TC-1).
    trust_footprints: bool,
    /// The optional `dorc why <address>` positional (ack-2): `book.sh:N` (a source line-address —
    /// rul24-lineno-identity), or free content to substring-match a command; `None` ⇒ the
    /// unargumented default (report the CURRENT run's problems). Only meaningful for [`Mode::Why`].
    why_address: Option<String>,
}

/// Minimal hand-rolled parsing (no `clap` dep yet): resolve the whole invocation. `-h`/`--help`
/// and `--version` win unconditionally (a pre-scan — ack-1 help-is-success, so a help request
/// beats a malformed flag) and return the stdout-and-exit-0 variants. Otherwise: an OPTIONAL
/// leading mode token (`probe`/`plan`/`apply`; absent ⇒ [`Mode::RoundTrip`]), then `--book=PATH` /
/// `--book PATH`, `-o PATH` / `-oPATH` / `--oracle PATH` (repeatable), `--debug-argv`,
/// `--trust-footprints`. The mode is positional-first ONLY (a bare word after flags is still an
/// error) so the legacy `dorc --book=… < results` invocation parses unchanged.
#[expect(
    clippy::too_many_lines,
    reason = "one linear arg surface: the help/version pre-scan, the mode + why-address token, then the flag/positional loop with did-you-mean; splitting it would scatter the ONE parse"
)]
fn parse_args() -> Result<Invocation, String> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    // ack-1 help-is-success: `--help`/`--version` are stdout-and-exit-0 requests, not usage
    // errors, and they win even alongside a malformed flag (the conventional precedence).
    if raw.iter().any(|a| a == "-h" || a == "--help") {
        return Ok(Invocation::Help);
    }
    if raw.iter().any(|a| a == "--version") {
        return Ok(Invocation::Version);
    }

    let mut books: Vec<String> = Vec::new();
    let mut oracles = Vec::new();
    let mut oracle_dirs = Vec::new();
    let mut results: Option<String> = None;
    let mut debug_argv = false;
    let mut trust_footprints = false;
    let mut why_address: Option<String> = None;
    let mut it = raw.into_iter().peekable();

    // A leading bare word (no `-` prefix) selects the mode. A near-miss (`pln`, `aply`) is a
    // did-you-mean, not a silent book (the recon's missing-suggestion hazard).
    let mode = match it.peek().map(String::as_str) {
        Some("probe") => {
            it.next();
            Mode::Probe
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
            if let Some(sugg) = nearest(w, &["probe", "plan", "apply", "why"]) {
                return Err(format!(
                    "unknown mode {w:?} — did you mean `{sugg}`? {USAGE}"
                ));
            }
            Mode::RoundTrip
        }
        _ => Mode::RoundTrip,
    };

    while let Some(arg) = it.next() {
        if let Some(p) = arg.strip_prefix("--book=") {
            books.push(p.to_string());
        } else if arg == "--book" {
            books.push(it.next().ok_or("--book needs a path")?);
        } else if arg == "-o" || arg == "--oracle" {
            oracles.push(it.next().ok_or("-o needs a path")?);
        } else if let Some(p) = arg.strip_prefix("-o").filter(|p| !p.is_empty()) {
            oracles.push(p.to_string());
        } else if let Some(p) = arg.strip_prefix("--oracle-dir=") {
            oracle_dirs.push(p.to_string());
        } else if arg == "--oracle-dir" {
            oracle_dirs.push(it.next().ok_or("--oracle-dir needs a directory")?);
        } else if let Some(p) = arg.strip_prefix("--results=") {
            results = Some(p.to_string());
        } else if arg == "--results" {
            results = Some(it.next().ok_or("--results needs a path")?);
        } else if arg == "--debug-argv" {
            debug_argv = true;
        } else if arg == "--trust-footprints" {
            trust_footprints = true;
        } else if arg.starts_with('-') {
            // An unrecognized FLAG: suggest the nearest known one (did-you-mean) rather than a bare
            // "unexpected argument" (the recon's missing-suggestion hazard).
            let known = [
                "--book",
                "--oracle",
                "--oracle-dir",
                "--results",
                "--debug-argv",
                "--trust-footprints",
                "--help",
                "--version",
            ];
            return match nearest(&arg, &known) {
                Some(sugg) => Err(format!(
                    "unknown flag {arg:?} — did you mean `{sugg}`? {USAGE}"
                )),
                None => Err(format!("unknown flag {arg:?}; {USAGE}")),
            };
        } else {
            // A bare word (no `-`): a positional book (the day-one `dorc plan book.sh` ergonomic;
            // repeatable ⇒ multi-book concatenation).
            books.push(arg);
        }
    }
    if books.is_empty() {
        return Err(format!(
            "no book given (a positional path or --book=PATH); {USAGE}"
        ));
    }
    Ok(Invocation::Analyze(Args {
        mode,
        books,
        oracles,
        oracle_dirs,
        results,
        debug_argv,
        trust_footprints,
        why_address,
    }))
}

/// A tiny did-you-mean: the nearest `candidate` to `word` within edit-distance 2 (a typo, not a
/// wholly different word), or `None`. Case-sensitive; ASCII. Used for mode + flag suggestions.
fn nearest<'a>(word: &str, candidates: &[&'a str]) -> Option<&'a str> {
    candidates
        .iter()
        .map(|c| (levenshtein(word, c), *c))
        .filter(|(d, _)| *d <= 2)
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
fn humane_read_error(kind: &str, path: &str, err: &std::io::Error) -> String {
    let why = match err.kind() {
        std::io::ErrorKind::NotFound => "no such file".to_owned(),
        std::io::ErrorKind::PermissionDenied => "permission denied".to_owned(),
        _ => err.to_string(),
    };
    format!("cannot read {kind} `{path}`: {why}")
}

/// Read + CONCATENATE the book(s) into one analyzed unit (`\n`-joined so no two files' lines
/// merge — multi-book concatenation-as-one-unit). Humane per-file errors.
fn read_books(books: &[String]) -> Result<String, String> {
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
fn resolve_oracle_paths(oracles: &[String], oracle_dirs: &[String]) -> Result<Vec<String>, String> {
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

#[expect(
    clippy::too_many_lines,
    reason = "the top-level pipeline driver: lift → analyze → probe → plan → render, one linear sequence with mode-routing; splitting it into sub-drivers would scatter the ONE call-shape the thin-driver mandate keeps here"
)]
fn run(args: &Args) -> Result<RunOutcome, String> {
    let mut interner = Interner::default();
    let mode = args.mode;
    // rec-1 advisory routing: `plan` and the legacy round-trip overlay the FULL advisory plane
    // on stderr (warnings, notes, the why-lens, the unresolvable readout); `apply` (the
    // off-ramp shippable) suppresses it, keeping only the error floor + digest. `probe`'s
    // stage diagnostics are advisory-or-error like any analysis run. tc-apply-receipt-floor:
    // WHERE this line falls (advisory-suppressed but error-kept, digest-kept) is the
    // load-bearing surface judgment — flagged to the conductor, not silently settled.
    let advisory = !matches!(mode, Mode::Apply);

    // ---- the shared, pure pipeline (one call-shape for every mode — the thin-driver
    // mandate: no mode branches the kernel; only the stdout/stderr ROUTING below differs) ----

    // Resolve the oracle PATHS: the explicit `-o` list, then every `*.oracle.sh` in each
    // `--oracle-dir` (glob-sorted, deterministic — ack-6). Then read each (humane errors).
    let oracle_paths = resolve_oracle_paths(&args.oracles, &args.oracle_dirs)?;
    let oracle_srcs: Vec<String> = oracle_paths
        .iter()
        .map(|p| std::fs::read_to_string(p).map_err(|e| humane_read_error("oracle", p, &e)))
        .collect::<Result<_, _>>()?;
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();
    // The effect-map is derived from the inline check bodies (23D §1 — the check is the
    // oracle); the probe lane (R3) ships the same stripped check bodies per-site.
    let lifted = dorc_oracle::lift(&mut interner, &oracle_refs);
    report_at(advisory, "oracle", None, &lifted.diags);
    let idx = lifted.value;

    // Lift each oracle's `<provider>__predict` functions into a per-file PredictSet (the
    // real entity-resolution mechanism — the engine threads the book's value-flow
    // through these, never parsing argv itself). Shared interner, so provider symbols
    // match the book's command words (204 seam #2).
    // ack-8: the per-file `check` diags span into THIS oracle's source, so zip the path back in
    // for the file:line:col frame (the check-dialect give-ups are the main oracle-side errors).
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracle_refs
        .iter()
        .zip(oracle_paths.iter())
        .map(|(src, path)| {
            let lifted = dorc_oracle::predict::lift_predicts(&mut interner, src);
            report_at(advisory, "check", Some((path.as_str(), src)), &lifted.diags);
            lifted.value
        })
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

    // The munge-reservation lint (24Kc fix-munge-reservation / 24M ca-munge-charclass): refuse an
    // emitted `<munged>__<role>` funcname that is not a legal sh NAME (charclass) or that two
    // distinct source names collide onto (non-injective munge), over the whole oracle unit. No
    // threaded source (the `oracle`-stage precedent — a cross-file collision has no single file to
    // frame into); the corpus is clean, so these Error-severity lints never fire in-corpus.
    report_at(
        advisory,
        "reserved",
        None,
        &dorc_oracle::reserved::lint_oracle_reserved_names(&mut interner, &oracle_refs),
    );

    // The marker gate (marker-gates-syntax-only): a dialect construct (bind/mark) in an UNMARKED
    // oracle is a loud error naming the missing `# dorc-lang/v0.1`. The corpus is marker-stamped
    // corpus-wide, so this is silent there; the bare `__role` floor lifts markerless regardless.
    for (src, path) in oracle_refs.iter().zip(oracle_paths.iter()) {
        report_at(
            advisory,
            "marker",
            Some((path.as_str(), src)),
            &dorc_oracle::marker::check_dialect_marker(&mut interner, src),
        );
    }

    // Parse + analyze the book (shared interner, so symbols match the oracles). Multiple books
    // CONCATENATE into one analyzed unit (`\n`-joined so no two files' lines merge). `book_name`
    // is the display path (the first book) — for a single book (the norm) the frame's line numbers
    // are exact source lines; a multi-book unit's line numbers are into the concatenation.
    let book_src = read_books(&args.books)?;
    let book_name = args.books.first().map_or("book.sh", String::as_str);
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
        .any(|d| d.severity == Severity::Error);
    let book_outcome = if book_unmodeled {
        RunOutcome::BookUnmodeled
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
    let (classified, why_diags, kills, kill_coords) =
        dorc_analysis::effect::classify_with_why_diags(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &verdict_providers,
            &mut interner,
            &mut arena,
        );
    report_at(advisory, "classify", book_source, &classified.diags);
    let classes = classified.value;

    // The per-site guard VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c) —
    // ALWAYS-ON (guards are the un-flagged baseline; rul24-mode-gate governs only the survival
    // tier, NOT this). A vouched past-wall establish ships its read-only probe (the witness needs
    // the verdict) and, converged, mints a `Disposition::Guard`.
    let vouches = build_vouches(&oracle_refs, &classes, &value, &mut interner, advisory);

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
    // site ships its probe here (at HEAD it would be `skip-unresolvable`).
    let ship = |p, a: &[Symbol]| ship_predict_body(&oracle_srcs, &checks, &interner, p, a);
    // `24L` §2 — the typeless-floor auto-cell ships its stripped VERDICT body (the probe IS the
    // verdict). `Some` ONLY for an auto-cell fact (keyed on the reserved auto-kind), so `compile_probe`
    // reads a `Some` as the auto-cell signal. rul-only-oracle-bytes-ship: the shipped bytes are the
    // oracle's OWN authored `is_converged` funcdef, strip-only; the admin's argv flows as arguments.
    let ship_auto = |fact: dorc_core::FactKey, p: Symbol, _a: &[Symbol]| -> Option<String> {
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
        &connected,
        ship,
        ship_auto,
        |node| vouches.contains_key(&node),
    );

    // The DERIVATION-probe (24E §2 corr-§2 — the SECOND probe-shipping path, a NEW pipeline
    // stage): under `--trust-footprints`, a wall-candidate whose `touches()` body ESCALATED (it
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
    if mode == Mode::Probe {
        print!("{}", probe.render_sh(&interner));
        print!("{}", derivations.render_sh(&interner)); // 24E §2: rides the SAME phase-1 block
        print!("{}", resolvers.render_sh()); // 24F §3: rides the SAME phase-1 block
        print!("{}", reaches_plan.render_sh()); // 24G §4: rides the SAME phase-1 block
        std::io::stdout().flush().ok();
        return Ok(book_outcome);
    }

    // The round-trip emits the probe FIRST (phase 1 on stdout), then the apply (phase 2)
    // after stdin EOF — the e2e harness splits the two on the `#!/bin/sh` shebang. `plan`
    // and `apply` emit ONLY the apply artifact (the probe is an internal compile there).
    if mode == Mode::RoundTrip {
        print!("{}", probe.render_sh(&interner));
        print!("{}", derivations.render_sh(&interner)); // 24E §2: rides the SAME phase-1 block
        print!("{}", resolvers.render_sh()); // 24F §3: rides the SAME phase-1 block
        print!("{}", reaches_plan.render_sh()); // 24G §4: rides the SAME phase-1 block
        std::io::stdout().flush().ok();
    }

    // read the (simulated) probe results — the site-keyed records the rendered probe would emit
    // when run remotely (the round-trip's return channel). From `--results FILE` when given, else
    // the default stdin (the harness pipes them in).
    let results_buf = if let Some(path) = &args.results {
        std::fs::read_to_string(path).map_err(|e| humane_read_error("results", path, &e))?
    } else {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("reading probe results from stdin: {e}"))?;
        buf
    };
    let results = parse_results(&results_buf, &mut interner);

    // re-key the site-keyed records to the FactKey-keyed observations `build_plan`
    // consumes (its fold/elision machinery is fact-keyed; only this probe-answer
    // plumbing re-keys — `inv-site-keyed-results`). The probe's `checks` carry each
    // site's resolved fact + its `site_kind`, so a site-record maps site→fact AND the
    // firewall knows whether the rc is fold-usable. CRITICAL (the wrong-concrete
    // firewall, 202 §3 / task-D2): a record's `rc` feeds the fold's Status ONLY for a
    // VALID Query-class site (the guard's own rc); an establish site's rc is the PROBE
    // command's (dpkg-query's), NOT the mutator's, so it feeds the fold NOTHING.
    let by_fact = facts_from_sites(&probe, &results);

    // The survival tier (Stage 2 / rul24-mode-gate, TC-1): footprints are lifted ONLY under
    // `--trust-footprints` — off ⇒ `None` ⇒ the honest Stage-1 total wall, the data never exists.
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
        merge_derived_footprints(
            &mut fps,
            &derivations,
            &results,
            &classes,
            &kill_coords,
            &mut interner,
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
            &results,
            &mut interner,
        );
        fps
    });
    // 24F §3: build the identity-canonicalization map from the `resolv` readback (both footprint and
    // backing coords canonicalized in the survival walk). Flag-off / no-resolver ⇒ empty ⇒ the
    // token-equality floor (identical to today). §4: each DANGLING coordinate is a loud diagnostic.
    let mut resolutions =
        build_resolutions(&resolver_coords, &resolver_kinds, &results, &mut interner);
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
        &vouches,
        &connected,
        |f| {
            by_fact
                .get(&f)
                .copied()
                .unwrap_or(Observable::verdict_only(Verdict::Unknown))
        },
        &mut arena,
    );

    // q-2 (`dq-site-unresolvable`, the cli-edge readout): a `skip-unresolvable` comment lands
    // in the probe artifact, but nothing reached stderr (`219` q-1.f silent-3). Disclose each
    // probe-unresolvable site's source command as a Note — the apply runs it (`kFAIL-perform`).
    // ADVISORY (Note-severity): the off-ramp `apply` mode suppresses it; `plan`/round-trip show
    // it (the ui-3 cited-disclosure surface). The apply still RUNS the site either way, so no
    // correctness rides on this readout — it is purely the render surface (rec-1).
    report_at(
        advisory,
        "probe",
        book_source, // the unresolvable-site notes span into the book (file:line:col frame)
        &unresolvable_diagnostics(&probe, &plan, &parsed.value, &book_src, &mut interner),
    );

    // upcoming-firstwall-hint (USER_STORY stage 3): the FIRST poison wall formed by an UNMODELED
    // command, plus the counterfactual count of downstream sites an oracle for it would un-wall.
    // Computed once (cheap, pure over the built plan) and consumed by BOTH the advisory `hint:` nag
    // (below) and the `dorc why` detail (`emit_why_report`). `None` ⇒ no unmodeled wall ⇒ no hint
    // (a modeled-but-diverged wall is an honest wall, never this hint's subject).
    let first_wall = first_wall_hint(&collect_wall_steps(
        &plan,
        &probe,
        &classes,
        &cfg.value,
        &kills,
        &parsed.value,
        &book_src,
    ));

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
    if advisory && mode != Mode::Why {
        emit_why_lens(&why_diags, &arena, &book_src);
        // sigpipe-flap-class (`279f` §5): a probe record landing rc 141 (128+SIGPIPE) is the
        // NAMED early-exit-race nondeterminism class — a `pipefail`-off `A | grep -q` whose
        // consumer closed the pipe before an upstream stage finished writing. The landing is SAFE
        // (cant-tell ⇒ Unknown ⇒ run) and never flaps the verdict, so this is an advisory nudge,
        // not an error. (A `--exit-code`-like surface must source from divergence-of-world, never
        // this raw rc — see `dorc_plan::render::probe::record_scaffold`.)
        emit_sigpipe_race_notes(&results);
        // Stage 2 co-primary (rul24-divergence-is-the-game / TC-3): every SURVIVED elision names,
        // on this same why-lens lane, which running walls it crossed and whose footprint licensed
        // each crossing. This is the attribution tether under the sharpest claim in the design —
        // a wrong footprint silently under-executes someone else's line, so the render surface
        // must always say whose footprint you trusted. Empty when unflagged (no survivals).
        emit_survival_attribution(&plan, &interner);
        // 24G Part B: every converged elision a reaches() expansion DEMOTED names the reach-function
        // (the cross-author demote); empty when no reach expansion poisoned an elision.
        emit_reach_poisonings(&plan, &interner);
        // Stage 3 (rul-guard-license / X-why): every GUARDED site names, on the same lane, the
        // mechanism + its converged-vouch license + the vouching oracle (a render-REFUSED guard
        // discloses the refusal instead). Empty when no site guards.
        emit_guard_attribution(&plan, &parsed.value, &interner);
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
        eprintln!(
            "dorc: run `dorc why` for the per-site cause-chains, or `dorc why {book_name}:N` to query a source line"
        );
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

    // ack-2 `dorc why`: NOT an artifact-producing invocation. Emit the source-line-keyed report to
    // STDOUT (its own non-analysis output) and return — no artifact, no plan-summary, no digest.
    // It runs the full pipeline above so it reports on the CURRENT run's real dispositions.
    if mode == Mode::Why {
        emit_why_report(
            args.why_address.as_deref(),
            &plan,
            &probe,
            first_wall.as_ref(),
            &why_diags,
            &refusals,
            &arena,
            &parsed.value,
            &book_src,
            book_name,
            &interner,
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

    emit_decision_digest(
        &plan,
        &probe,
        &book_src,
        &parsed.value,
        &interner,
        classified.diags,
        refusals,
    );
    Ok(book_outcome)
}

/// arch-1 decision-digest (`mechanism-decision-digest`, `22A` concl-3): a one-line hash of the
/// canonical IDENTITY plane, emitted on every plan-building run as a cheap always-on drift
/// signal. Receipts cannot move it — it hashes only the identity plane (the `plan::erasability`
/// gate proves that). To stderr (stdout stays the artifact). KEPT even in the receipt-free
/// `apply` mode: the digest is identity-plane, not a receipt. The Error-class diagnostics on the
/// identity plane are the analyzer's accumulated ones (classify) plus the render refusals;
/// warnings/notes are exempt (dropped by the canon).
fn emit_decision_digest(
    plan: &dorc_plan::Plan,
    probe: &dorc_plan::ProbePlan,
    book_src: &str,
    ast: &dorc_syntax::ast::Ast,
    interner: &Interner,
    classify_diags: Vec<dorc_core::Diagnostic>,
    refusals: Vec<dorc_core::Diagnostic>,
) {
    let mut identity_diags = classify_diags;
    identity_diags.extend(refusals);
    eprintln!(
        "dorc: decision-digest {}",
        dorc_plan::erasability::decision_digest(
            plan,
            probe,
            book_src,
            ast,
            interner,
            &identity_diags,
        )
    );
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
) -> Option<String> {
    use dorc_oracle::predict::{Resolution, evaluate, map_provider_name, strip_predict};
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
                return Some(strip_predict(src, check, interner));
            }
        }
    }
    None
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
) -> Option<String> {
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    use dorc_oracle::verdict::VERDICT_SUFFIX;
    let want = map_provider_name(interner.resolve(provider));
    for (src, set) in oracle_srcs.iter().zip(verdict_sets) {
        for vp in set.providers() {
            if map_provider_name(interner.resolve(vp)) != want {
                continue;
            }
            let Some(verdict) = set.get(vp) else { continue };
            return Some(strip_verdict(src, verdict, interner, VERDICT_SUFFIX));
        }
    }
    None
}

/// Lift the survival footprints (Stage 2 / rul24-mode-gate) — called ONLY on the
/// `--trust-footprints` path (TC-1: the footprint data does not exist unflagged). For each
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
        let Some((provider, coords_with_selectors)) =
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
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("footprint-incoherent"),
                Some(span),
                "touches() footprint omits this command's own effect coordinate \
                 (at-least ⊄ at-most) — footprint refused, the site walls",
            ));
            continue;
        }
        // 24G §8: UNION the site's own effect coordinate (engine-supplied provenance) into the
        // footprint. A no-op on the hit-surface HERE (the canary just proved own ∈ coords), but it
        // records own for the why-lens and keeps the two lanes uniform. Empty emission ⇒ None from
        // `authored` ⇒ `with_own` cannot resurrect it (anti-233).
        if let Some(mut footprint) =
            dorc_plan::Footprint::authored(provider, coords).map(|fp| fp.with_own(own))
        {
            // `277` §3: record each emission's `#selector` so a selector-bearing disturbs mark can
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
/// [`dorc_plan::EntityCoord`] that drives canonicalization/render, and the `#selector` cell the
/// dialect consults (`None` ⇒ whole-entity ⊤).
type FootprintCoord = (dorc_plan::EntityCoord, Option<dorc_core::SelectorId>);

fn resolve_touches_footprint(
    node: dorc_analysis::cfg::CfgNodeId,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
) -> Option<(Symbol, Vec<FootprintCoord>)> {
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::touches::{TouchesResolution, evaluate_touches};

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
    let coords = touches_sets.iter().find_map(|set| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .and_then(|touches| match evaluate_touches(touches, &arg_refs) {
                TouchesResolution::Emitted(coords) if !coords.is_empty() => Some(coords),
                // Emitted(empty) = no claim = wall; Top = ⊤ = wall. Both ⇒ no footprint.
                TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
            })
    })?;

    // Intern each opaque `kind:entity#selector` fragment into the shared vocabulary (the fence).
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
    Some((*provider, entity_coords))
}

/// The derivation-probe seam (24E §2/§3 — fork-4A: the SAME self-vouch tier as `predict`, no new
/// trust edge): for a wall-candidate site's (provider-word, argv), find the provider's `touches()`
/// funcdef and trace it statically. `Some(DerivationShip)` iff the trace ESCALATED — it ⊤'d
/// specifically on a `NonPrintfCommand` (the body reached a host query the static tracer cannot
/// resolve, e.g. `dpkg -L`), the sanctioned escalation trigger (fork-4B). The body then ships
/// strip-only (`strip_touches`; `<provider>.touches` → `<provider>__touches`), the SAME strip
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
fn merge_derived_footprints(
    footprints: &mut dorc_plan::TrustedFootprints,
    derivations: &dorc_plan::DerivationPlan,
    results: &SiteResults,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    interner: &mut Interner,
    advisory: bool,
) {
    let mut diags = Vec::new();
    for d in &derivations.derivations {
        diags.push(dorc_core::Diagnostic::note(
            dorc_core::DiagCode("touches-escalated"),
            None,
            format!(
                "site {}: touches() escalated to host-derivation ({})",
                d.site.0, d.call
            ),
        ));
        let Some(coord_strs) = results.derivations.get(&d.site) else {
            continue; // no readback records ⇒ empty derived footprint ⇒ wall (kFAIL-safe)
        };
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
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("footprint-incoherent"),
                None,
                "derived touches() emitted a malformed coordinate (not kind:entity) — footprint \
                 refused, the site walls (an at-most claim cannot be partial)"
                    .to_string(),
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
    report_at(advisory, "derive", None, &diags);
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

    let mut diags = Vec::new();
    let mut base_to_idx: BTreeMap<Symbol, usize> = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        if files.len() > 1 {
            diags.push(dorc_core::Diagnostic::error(
                dorc_core::DiagCode("resolver-conflict"),
                None,
                format!(
                    "kind '{name}' has {} resolvers across oracle files — at-most-one-resolver-per-kind \
                     (24F §3): BOTH refused, the kind keeps token-equality (never first-wins-silently)",
                    files.len()
                ),
            ));
            continue; // refuse both ⇒ resolver-less
        }
        if providers.contains(&name) {
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("resolver-provider-collision"),
                None,
                format!(
                    "resolver '{name}.resolve()' is keyed to a name matching a known COMMAND provider \
                     — resolvers are keyed by KIND, not command (corr-kind-keying §10); this mints \
                     identity for a kind no coordinate may use (a likely mis-key)"
                ),
            ));
            // Kept (it may legitimately match a kind of the same name); the warning surfaces the risk.
        }
        if let Some(&idx) = files.first() {
            base_to_idx.insert(kind, idx);
        }
    }
    report_at(advisory, "resolve", None, &diags);
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
            && let Some((_, fp_coords)) =
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
            && let Some((_, fp_coords)) =
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
fn dangling_diagnostics(
    resolutions: &dorc_plan::Resolutions,
    interner: &Interner,
) -> Vec<dorc_core::Diagnostic> {
    resolutions
        .dangling()
        .map(|coord| {
            dorc_core::Diagnostic::note(
                dorc_core::DiagCode("dangling-reference"),
                None,
                format!(
                    "coordinate {} resolved DANGLING — the kind's resolver reports no such entity \
                     (a likely typo / stale name); it degrades to may-alias (the site runs)",
                    render_coord(coord, interner)
                ),
            )
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

    let mut diags = Vec::new();
    let mut base_to_idx: BTreeMap<Symbol, usize> = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        if files.len() > 1 {
            diags.push(dorc_core::Diagnostic::error(
                dorc_core::DiagCode("reaches-conflict"),
                None,
                format!(
                    "kind '{name}' has {} reach-functions across oracle files — at-most-one-reaches-per-kind \
                     (24G §4): BOTH refused, the kind's footprints do not expand (never first-wins-silently)",
                    files.len()
                ),
            ));
            continue;
        }
        if providers.contains(&name) {
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("reaches-provider-collision"),
                None,
                format!(
                    "reach-function '{name}.reaches()' is keyed to a name matching a known COMMAND provider \
                     — reaches is keyed by KIND, not command (24G §4); this expands a kind no coordinate \
                     may use (a likely mis-key)"
                ),
            ));
        }
        if let Some(&idx) = files.first() {
            base_to_idx.insert(kind, idx);
        }
    }
    report_at(advisory, "reaches", None, &diags);
    let by_kind = rekey_to_raw_kinds(&base_to_idx, coord_kinds, interner);
    KindReaches { sets, by_kind }
}

/// Compile the reach-probe (24G §4): for each reach-bearing AUTHORED footprint coordinate, ship each
/// DYNAMIC `reaches()` arm's per-arm wrapper (`<kind>__reaches_<n>() { <arm bytes> ; }` — the arm
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
        let Some((_, fp_coords)) = resolve_touches_footprint(*node, value, touches_sets, interner)
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
                let arm_fn = format!(
                    "{}__reaches_{}",
                    dorc_oracle::to_funcname_segment(&kind_name),
                    arm.index
                );
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
/// (`<provider>.is_converged`/`.is_diverged`) that REACHES a vouching path over the site's resolved
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
) -> dorc_plan::Vouches {
    // The composition lives in `dorc_plan::build_vouches` (the ONE home — the sweep/coverage DSTs
    // share it). This edge only ROUTES the lift diagnostics: surfaced AS-IS (inv-top-reject — the
    // tc-verdict-return softening is reverted, find-return-vouches 24C), so a genuinely
    // out-of-dialect verdict body fails gate-3's error-floor rather than degrading silently.
    let lifted = dorc_plan::build_vouches(oracle_refs, classes, value, interner);
    report_at(advisory, "verdict", None, &lifted.diags);
    lifted.value
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
                ValueOf::Top => "TOP".to_string(),
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
    interner: &mut Interner,
) -> Vec<dorc_core::Diagnostic> {
    use dorc_core::diag::{Diag, DiagCode, SiteId, SiteUnresolvable};
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
        "{} site{plural} run unprobed (no read-only check could be shipped): {} — \
         run `dorc why` for the per-site detail (the apply runs each anyway, to stay safe)",
        real.len(),
        names.join(", "),
    );
    let first_text = book_src
        .get(first_span.lo.0 as usize..first_span.hi.0 as usize)
        .unwrap_or("<source unavailable>");
    let diag = Diag::new(
        DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: SiteId::leaf(first_leaf),
            source_excerpt: OutBytes(interner.intern(first_text)),
        }),
        first_span,
    )
    .label(label);
    vec![diag.to_legacy(interner)]
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
        "dorc: plan-summary sites={} elide={} omit={} guard={} run={} may-alias={}",
        counts.sites,
        counts.elide,
        counts.omit,
        counts.guard,
        counts.run,
        plan.survival_report.may_alias_fires(),
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
fn emit_why_lens(why_diags: &[dorc_core::diag::Diag], arena: &ProvArena, src: &str) {
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
fn emit_survival_attribution(plan: &dorc_plan::Plan, interner: &Interner) {
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
        eprintln!(
            "why: site {} survives+elides past {} — backing {} disjoint (trusted footprint)",
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
            "why: site {} runs — poisoned via {}.reaches() (a reach-expanded coordinate hit its \
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
                "why: site {} guard refused — the site's structurally-awkward form (a heredoc \
                 body, or a non-`/dev/null` output redirect) would corrupt the artifact or suppress \
                 an admin-spelled side-effect, so the original bytes RUN VERBATIM (to stay safe), \
                 the {kind} oracle's vouch that it is already satisfied notwithstanding",
                step.leaf.0,
            );
        } else {
            eprintln!(
                "why: site {} guard [{kind}] — licensed by the {kind} oracle's vouch that it is \
                 already satisfied; the original bytes survive and the check re-runs live at apply \
                 (to stay safe)",
                step.leaf.0,
            );
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
            format!("; {} more unmodeled {walls} — dorc why", self.more_walls)
        };
        format!(
            "'{}' (line {}) is unmodeled: it is the first wall — an oracle vouching its \
             convergence would elide it when converged{unwall_clause}{more_clause}",
            self.word, self.line
        )
    }

    /// The `dorc why` detail line for the wall's own site (the reasoning behind the plan-mode nag).
    fn why_detail(&self) -> String {
        if self.unwall == 0 {
            "first wall (book order) — an oracle vouching its convergence would elide it when \
             converged"
                .to_owned()
        } else {
            let sites = if self.unwall == 1 { "site" } else { "sites" };
            format!(
                "first wall (book order) — an oracle vouching its convergence would elide it and \
                 un-wall {} downstream {sites}",
                self.unwall
            )
        }
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
            let line = dorc_core::diag::line_col(book_src, lo).0;
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

/// One site's WHY-record ([`emit_why_report`]): its SOURCE line (rul24-lineno-identity), the
/// one-line command, the disposition tag, the ASCII cause-chain, and whether it is a PROBLEM
/// (the unargumented `dorc why` filter — a ⊤/unprobed run, a guard, or a render-refusal, never a
/// clean elide/omit).
struct WhySite {
    line: usize,
    command: String,
    tag: &'static str,
    reasons: Vec<String>,
    is_problem: bool,
}

/// ack-2 `dorc why`: the source-line-keyed WHY report — the focused query surface (the `plan`
/// preview points here). **rul24-lineno-identity** (a product invariant): the ONE line-number
/// space is the SOURCE file's, so a `file:N` this report PRINTS is exactly the `book.sh:N` a query
/// ACCEPTS — the mapping is 1:1 through [`dorc_core::diag::line_col`]. Three addressing forms:
/// * `None` (unargumented) — the CURRENT run's PROBLEMS: every site that runs on a ⊤, runs
///   unprobed, or carries a guard / render-refusal (never a clean elide/omit) — "can't be typing
///   lines manually when you're already annoyed" (NO cross-run state; kSTATE stays parked).
/// * a `book.sh:N` / bare `N` line-address — the site(s) on that source line.
/// * free content — the site(s) whose command text contains it.
///
/// Each reported site prints a `file:line` header with its disposition tag and command, then an
/// ASCII depth-indented cause-chain (the `└─` glyph), one root per site (rustc's nested notes).
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
    why_diags: &[dorc_core::diag::Diag],
    refusals: &[dorc_core::Diagnostic],
    arena: &ProvArena,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
    filename: &str,
    interner: &Interner,
) {
    use dorc_plan::Disposition;
    let mut sites: Vec<WhySite> = Vec::new();
    for step in &plan.steps {
        let span = ast.node(step.ast).span;
        let (lo, hi) = (span.lo.0 as usize, span.hi.0 as usize);
        let line = dorc_core::diag::line_col(book_src, lo).0;
        let command = flatten_ws(book_src.get(lo..hi).unwrap_or("<source unavailable>"));
        let refused = refusals
            .iter()
            .any(|d| d.span.is_some_and(|s| s.lo == span.lo && s.hi == span.hi));
        let (tag, reasons, is_problem): (&'static str, Vec<String>, bool) = match &step.disposition
        {
            Disposition::Run => {
                if let Some(reason) = top_run_reason(span, why_diags, arena, book_src) {
                    ("run", vec![reason], true)
                } else if probe.unresolvable.contains(&step.leaf)
                    && !is_structurally_unprobeable(&command)
                {
                    let mut reasons = vec![
                        "runs unprobed — no read-only check could be shipped (unsure ⇒ dorc \
                         runs it, to stay safe)"
                            .to_owned(),
                    ];
                    // upcoming-firstwall-hint: the FIRST unmodeled wall carries the forward
                    // reasoning here — the `dorc why` detail behind the plan-mode `hint:` nag.
                    if let Some(fw) = first_wall.filter(|fw| fw.leaf == step.leaf) {
                        reasons.push(fw.why_detail());
                    }
                    ("run", reasons, true)
                } else {
                    (
                        "run",
                        vec![
                            "runs — not elidable (a mutator with no converged probe, an inert \
                             builtin, or a running wall blocks elision)"
                                .to_owned(),
                        ],
                        false,
                    )
                }
            }
            Disposition::Replace(license, _) => {
                let mut reasons = vec![format!(
                    "elided — {} is converged (probe: holds)",
                    dorc_plan::fact_label(interner, license.fact())
                )];
                if let Some(w) = &license.derivation().survival {
                    reasons.push(format!(
                        "survived past {} running wall(s) — backing {} proven disjoint (trusted \
                         footprint)",
                        w.crossings().len(),
                        render_coord(w.backing(), interner),
                    ));
                }
                if refused {
                    reasons.push(
                        "render REFUSED (heredoc): the line runs verbatim instead, to stay safe"
                            .to_owned(),
                    );
                    ("elide", reasons, true)
                } else {
                    ("elide", reasons, false)
                }
            }
            Disposition::Guard(license) => {
                let kind = interner.resolve(license.fact().kind.0).to_owned();
                if refused {
                    (
                        "guard",
                        vec![format!(
                            "guard REFUSED — the site's awkward form (heredoc / non-`/dev/null` \
                             redirect) runs verbatim (to stay safe), the {kind} oracle's vouch that \
                             it is already satisfied notwithstanding"
                        )],
                        true,
                    )
                } else {
                    (
                        "guard",
                        vec![format!(
                            "guarded — the {kind} oracle vouches it is already satisfied, so the \
                             original bytes survive and the oracle's check re-runs live at apply"
                        )],
                        true,
                    )
                }
            }
            Disposition::Omit { .. } => (
                "omit",
                vec![
                    "omitted — dead branch (a guard's known status proves it never runs)"
                        .to_owned(),
                ],
                false,
            ),
        };
        sites.push(WhySite {
            line,
            command,
            tag,
            reasons,
            is_problem,
        });
    }

    // The three addressing forms (rul24-lineno-identity: a line-address matches `s.line`, the
    // SOURCE line every `WhySite` was keyed on).
    let (heading, matched): (String, Vec<&WhySite>) = match address {
        None => (
            String::new(), // the problem-set heading is emitted below (it names a count)
            sites.iter().filter(|s| s.is_problem).collect(),
        ),
        Some(addr) => match parse_line_address(addr) {
            Some(n) => (
                format!("dorc why {filename}:{n}:"),
                sites.iter().filter(|s| s.line == n).collect(),
            ),
            None => (
                format!("dorc why `{addr}`:"),
                sites.iter().filter(|s| s.command.contains(addr)).collect(),
            ),
        },
    };

    if address.is_none() {
        // The unargumented default: the run's PROBLEMS, with a count-bearing heading.
        if matched.is_empty() {
            println!(
                "dorc why: no problems in the current run of {filename} — every site elided, ran \
                 cleanly, or was omitted."
            );
            return;
        }
        println!(
            "dorc why: {} problem(s) in the current run of {filename} (source-line order):\n",
            matched.len()
        );
    } else if matched.is_empty() {
        println!(
            "{heading} no analyzed command matched (rul24-lineno-identity: a line-address is a SOURCE line)."
        );
        return;
    } else {
        println!("{heading}\n");
    }
    for s in matched {
        print_why_site(s, filename);
    }
}

/// Print one [`WhySite`]: the `file:line` header (with disposition tag + command) then the ASCII
/// depth-indented cause-chain (`└─` per reason). rul24-lineno-identity: `line` is the SOURCE line.
fn print_why_site(s: &WhySite, filename: &str) {
    println!("{filename}:{}  [{}]  `{}`", s.line, s.tag, s.command);
    for r in &s.reasons {
        println!("  └─ {r}");
    }
    println!();
}

/// The ⊤-run cause for a Run site, if a `why_diags` disclosure covers it: the FIRST diag whose
/// primary span starts inside this command's span (the cmdsub-⊤ origin sits at/within the
/// command), rendered through the why-lens [`dorc_core::diag::why`] (the same cause-chain the
/// `plan` render surfaces). `None` ⇒ no ⊤-cause (the caller falls to unprobed / not-elidable).
fn top_run_reason(
    span: dorc_core::Span,
    why_diags: &[dorc_core::diag::Diag],
    arena: &ProvArena,
    book_src: &str,
) -> Option<String> {
    why_diags.iter().find_map(|d| {
        let psp = d.primary.span()?;
        (psp.lo.0 >= span.lo.0 && psp.lo.0 < span.hi.0)
            .then(|| dorc_core::diag::why(d, arena, book_src).map(|e| e.reason))
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
/// line via [`dorc_core::diag::why`], showing a given cause-SITE once.
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
fn why_lens_lines(
    why_diags: &[dorc_core::diag::Diag],
    arena: &ProvArena,
    src: &str,
) -> Vec<String> {
    let mut shown: Vec<(dorc_core::ProvId, dorc_core::diag::SiteId)> = Vec::new();
    let mut lines = Vec::new();
    for diag in why_diags {
        if let Some(key) = cmdsub_cause_site(diag) {
            if shown.contains(&key) {
                continue; // stage-4: this (cause, site) was already explained — show it once
            }
            shown.push(key);
        }
        if let Some(explanation) = dorc_core::diag::why(diag, arena, src) {
            lines.push(explanation.reason);
        }
    }
    lines
}

/// The stage-4 render-dedup key a why-lens diag carries, if any: `(⊤-cause, site)`. Only a
/// `CmdsubOperandTop` carries a cause at HEAD (stage-1); any other diag returns `None` (the why-lens
/// does not explain it anyway, fd-G), so it never participates in the dedup. The `site` half is what
/// separates two inlined call-sites sharing one cause `ProvId` (`x2-fd1`).
fn cmdsub_cause_site(
    diag: &dorc_core::diag::Diag,
) -> Option<(dorc_core::ProvId, dorc_core::diag::SiteId)> {
    match &diag.code {
        dorc_core::diag::DiagCode::CmdsubOperandTop(p) => p.cause.map(|c| (c, p.site)),
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
    use dorc_core::diag::{CmdsubOperandTop, Diag, DiagCode, OperandPosition, SiteId};
    use dorc_core::{BytePos, LeafId, OriginKind, ProvArena, Span};

    fn cmdsub_top(arena: &mut ProvArena, leaf: u32, body_span: Span) -> Diag {
        let cause = arena.leaf(OriginKind::TopCause, Some(body_span));
        Diag::new(
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: SiteId::leaf(LeafId(leaf)),
                position: OperandPosition::Operand(1),
                cause: Some(cause),
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
) -> BTreeMap<dorc_core::FactKey, Observable> {
    use dorc_plan::ProbeSiteKind;
    let mut by_fact: BTreeMap<dorc_core::FactKey, Observable> = BTreeMap::new();
    for check in &probe.checks {
        // Key the record by (site, member) — a member check (`site N.M`) reads its own
        // sub-record (task-L2 item-4); an ordinary check (`site N`) reads `member: None`.
        let record = results.records.get(&RecordKey {
            site: check.site,
            member: check.member,
        });
        let effect = record.map_or(Verdict::Unknown, |r| r.verdict);
        // The firewall: only a VALID Query site's rc is fold-usable as Status.
        let status = match check.site_kind {
            ProbeSiteKind::Query { valid: true } => {
                record.map_or(Predicted::Top, |r| Predicted::Value(r.rc))
            }
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
        by_fact
            .entry(check.fact)
            .and_modify(|prior| *prior = merge_observable(*prior, obs))
            .or_insert(obs);
    }
    by_fact
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
}

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
fn parse_results(input: &str, interner: &mut Interner) -> SiteResults {
    let mut out = SiteResults::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        match it.next() {
            // 24E §5 (fork-s4-coordwire): `deriv <leafid> coord=<kind:entity>…` — accumulate an
            // escalated wall-site's derived-footprint coordinates, demuxed by leaf-id in its OWN
            // lane (never the `site` verdict records). A malformed leaf ⇒ drop (the site's derived
            // footprint stays empty ⇒ wall, the kFAIL-safe direction).
            Some("deriv") => {
                if let Some(site) = it
                    .next()
                    .and_then(|t| t.parse::<u32>().ok())
                    .map(dorc_plan::LeafId)
                {
                    for tok in it {
                        if let Some(coord) = tok.strip_prefix("coord=") {
                            out.derivations
                                .entry(site)
                                .or_default()
                                .push(coord.to_owned());
                        }
                    }
                }
                continue;
            }
            // 24F §3: `resolv <kind:entity> canon=<canonical>` | `resolv <kind:entity> dangling` —
            // the resolver readback, demuxed by the COORDINATE label in its OWN lane. A malformed
            // line ⇒ drop (the coord stays unrecorded ⇒ may-alias, the kFAIL-safe direction).
            Some("resolv") => {
                if let Some(coord) = it.next() {
                    let outcome = match it.next() {
                        Some("dangling") => Some(ResolvOutcome::Dangling),
                        Some(tok) => tok
                            .strip_prefix("canon=")
                            .map(|c| ResolvOutcome::Canonical(c.to_owned())),
                        None => None,
                    };
                    if let Some(o) = outcome {
                        out.resolutions.insert(coord.to_owned(), o);
                    }
                }
                continue;
            }
            // 24G §4: `reach <kind:entity> arm=<n> entity=<line>` — accumulate a DYNAMIC reaches()
            // arm's emitted RAW ENTITY, demuxed by (coordinate, arm) in its OWN lane. A malformed
            // line ⇒ drop (the coord's expansion stays narrower ⇒ the un-expanded floor, kFAIL-safe:
            // an omitted reach only fails to WIDEN, never a wrong-reach). NB `entity=<line>` is the
            // LAST token — a reached entity with an embedded space would truncate (the SAME
            // single-token limitation as the `deriv`/`resolv` lanes; file paths rarely carry spaces).
            Some("reach") => {
                if let Some(coord) = it.next() {
                    let mut arm: Option<usize> = None;
                    let mut entity: Option<String> = None;
                    for tok in it {
                        if let Some(n) = tok.strip_prefix("arm=").and_then(|n| n.parse().ok()) {
                            arm = Some(n);
                        } else if let Some(e) = tok.strip_prefix("entity=") {
                            entity = Some(e.to_owned());
                        }
                    }
                    if let (Some(a), Some(e)) = (arm, entity) {
                        out.reaches
                            .entry((coord.to_owned(), a))
                            .or_default()
                            .push(e);
                    }
                }
                continue;
            }
            Some("site") => {}
            _ => continue, // unrecognized line ⇒ drop (kFAIL-perform: no verdict ⇒ run)
        }
        let Some(key) = it.next().and_then(parse_site_key) else {
            continue; // malformed site key ⇒ drop (⇒ Unknown ⇒ run)
        };
        // The remaining tokens carry `effect=<word>`, `rc=<n>`, and the reserved
        // `stdout=`/`stderr=` in any order. A missing/garbled `effect` ⇒ Unknown (the safe
        // direction); a missing/garbled `rc` ⇒ 0 (carried, but irrelevant unless the
        // firewall admits it for a valid Query). Absent out-claims stay `Predicted::Top`.
        let mut verdict = Verdict::Unknown;
        let mut rc = Rc(0);
        let mut stdout = Predicted::Top;
        let mut stderr = Predicted::Top;
        for tok in it {
            if let Some(w) = tok.strip_prefix("effect=") {
                verdict = effect_word_to_verdict(w);
            } else if let Some(n) = tok.strip_prefix("rc=").and_then(|n| n.parse::<i32>().ok()) {
                rc = Rc(n);
            } else if let Some(t) = tok.strip_prefix("stdout=") {
                stdout = Predicted::Value(OutBytes(interner.intern(t)));
            } else if let Some(t) = tok.strip_prefix("stderr=") {
                stderr = Predicted::Value(OutBytes(interner.intern(t)));
            }
        }
        out.records.insert(
            key,
            SiteRecord {
                verdict,
                rc,
                stdout,
                stderr,
            },
        );
    }
    out
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

/// Advisory-gated [`report`] (rec-1 / tc-apply-receipt-floor): the stderr driver over
/// [`advisory_filter`]. When `advisory` is true, emit every severity (the `plan` /
/// round-trip render surface — the ui-3 cited-disclosure console); when false (the off-ramp
/// `apply` mode), emit ONLY Error-severity diagnostics. The error floor is never suppressed
/// in any mode — a shippable artifact must never hide an error — so `apply` stays
/// receipt-free WITHOUT going blind. The filter is factored PURE (the printing is the I/O
/// edge) so the lone per-severity routing decision rec-1 forces here is unit-testable, the
/// same pure/driver split as [`why_lens_lines`]/[`emit_why_lens`].
fn report_at(
    advisory: bool,
    stage: &str,
    source: Option<(&str, &str)>,
    diags: &[dorc_core::Diagnostic],
) {
    report(stage, source, &advisory_filter(advisory, diags));
}

/// The advisory severity-filter (rec-1 / tc-apply-receipt-floor), factored pure for
/// testing. `advisory` ⇒ pass every diagnostic through (the `plan`/round-trip render
/// surface); `!advisory` (the receipt-free `apply` off-ramp) ⇒ keep ONLY Error-severity,
/// dropping warnings + notes. Errors are NEVER dropped — the floor that keeps `apply`
/// honest while receipt-free. Returns owned clones (the call sites are cold — once per
/// pipeline stage — so the copy is irrelevant against the SSH-tunnel cost DESIGN floors on).
fn advisory_filter(advisory: bool, diags: &[dorc_core::Diagnostic]) -> Vec<dorc_core::Diagnostic> {
    if advisory {
        diags.to_vec()
    } else {
        diags
            .iter()
            .filter(|d| d.severity == Severity::Error)
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
/// caret ([`dorc_core::diag::render_legacy_region`]) — replaces the old raw byte-offset `-->
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
fn report(stage: &str, source: Option<(&str, &str)>, diags: &[dorc_core::Diagnostic]) {
    use std::io::Write as _;
    let mut w = anstream::stderr();
    for d in diags {
        let (word, style) = severity_style(d.severity);
        // Split the message so the region frame lands right after the TITLE line, before any
        // folded ` = note:`/` = help:` continuations a lowered `Diag` carries in its message.
        let (title, folded) = match d.message.split_once('\n') {
            Some((t, rest)) => (t, Some(rest)),
            None => (d.message.as_str(), None),
        };
        // The severity word carries the ANSI (stripped when piped); the `[code]` + region + notes
        // are plain. `{style}` opens the style, `{style:#}` resets it (anstyle's Display shape).
        let _ = write!(w, "{stage}: {style}{word}{style:#}[{}]: {title}", d.code.0);
        let _ = write!(w, "{}", dorc_core::diag::render_legacy_region(d, source));
        let _ = match folded {
            Some(rest) => writeln!(w, "\n{rest}"),
            None => writeln!(w),
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
        let flags = ["--trust-footprints", "--debug-argv", "--book"];
        assert_eq!(
            nearest("--tust-footprints", &flags),
            Some("--trust-footprints")
        );
        assert_eq!(nearest("--boook", &flags), Some("--book"), "one insertion");
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
        }
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
        let kr_dup =
            build_kind_resolvers(&dup, &checks, &touches_paired, &coord_kinds, &mut i, false);
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
        }
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
                connected: None,
                verdict: false,
            }],
            unresolvable: vec![],
        }
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
        let killed = pkg(&mut i, "nginx"); // package:nginx#installed (a purge's killed cell)
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
        let r = parse_results(
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
        // Unrecognized / malformed lines are dropped (⇒ Unknown ⇒ run). Pins the
        // garbage-stdin behavior at the unit layer (`kFAIL-perform`). The dead
        // `declared-rc` lane is now just an unrecognized line ⇒ dropped.
        let mut i = Interner::default();
        let r = parse_results(
            "this is not a record\nsite notanumber effect=holds\n\
             site 0 garbled-no-effect\ndeclared-rc 0 rc=0\n# a comment\n",
            &mut i,
        );
        // `site 0 garbled-no-effect` parses the id but no effect= ⇒ Unknown (safe), rc 0.
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        // `site notanumber` ⇒ no id ⇒ dropped; the dead `declared-rc` line ⇒ dropped.
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
        let r = parse_results("site 0 effect=holds rc=0\n", &mut i);
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
        let r = parse_results(
            "site 0 effect=holds rc=0 stdout=hello stderr=warn\n",
            &mut i,
        );
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert!(
            matches!(rec.stdout, Predicted::Value(OutBytes(_))),
            "a reserved stdout= is stored as a value claim: {:?}",
            rec.stdout
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
        let results = parse_results("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
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
        let results = parse_results("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Value(Rc(0)),
            "a valid Query guard's own rc supplies the fold Status"
        );
        // A non-zero guard rc (nginx absent) carries through identically (Exit(n) path).
        let results = parse_results("site 0 effect=absent rc=1\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
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
        let results = parse_results("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an INVALID Query guard's rc is stale ⇒ withheld (status Top ⇒ runs for real)"
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
                    connected: None,
                    verdict: false,
                },
                ProbePredict {
                    site: LeafId(1),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k1,
                    sh: "{ :; }".to_string(),
                    connected: None,
                    verdict: false,
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
        let results = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
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
        let results = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
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
        let results = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
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
        let diags = unresolvable_diagnostics(&probe, &plan, &parsed.value, book, &mut interner);
        assert!(
            diags.iter().any(|d| d.code.0 == "dq-site-unresolvable"),
            "an Opaque site must be disclosed unresolvable: {diags:?}"
        );
        assert!(
            diags
                .iter()
                .any(|d| d.code.0 == "dq-site-unresolvable" && d.message.contains("make install")),
            "the disclosure must name the source command: {diags:?}"
        );
        assert!(
            diags.iter().all(|d| d.severity == Severity::Note),
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
        // The slugs are the diag_tidy-recognized throwaway-fixture set (`x-err`/`x-warn`/
        // `x-note`, core::tests::diag_tidy::is_test_fixture_slug) — NOT real catalog codes, so
        // the legacy-allow-list completeness gate (226 §1) exempts them without an allow-list entry.
        use dorc_core::{BytePos, DiagCode, Diagnostic, Span};
        let span = Some(Span::new(BytePos(0), BytePos(1)));
        let mixed = vec![
            Diagnostic::error(DiagCode("x-err"), span, "an error"),
            Diagnostic::warning(DiagCode("x-warn"), span, "a warning"),
            Diagnostic::note(DiagCode("x-note"), span, "a note"),
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
            kept[0].severity,
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
    use super::{FirstWallHint, WallRole, WallStep, first_wall_hint};

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
            "'foobar' (line 8) is unmodeled: it is the first wall — an oracle vouching its \
             convergence would elide it when converged, and un-wall 1 downstream site"
        );
        // M=2 ⇒ "sites"; a further wall ⇒ the trailing pointer.
        assert_eq!(
            hint(2, 1).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall — an oracle vouching its \
             convergence would elide it when converged, and un-wall 2 downstream sites; 1 more \
             unmodeled wall — dorc why"
        );
        // M=0 ⇒ the un-wall clause is dropped (never "un-wall 0").
        assert_eq!(
            hint(0, 0).body(),
            "'foobar' (line 8) is unmodeled: it is the first wall — an oracle vouching its \
             convergence would elide it when converged"
        );
        // more_walls plural.
        assert!(
            hint(0, 2)
                .body()
                .ends_with("; 2 more unmodeled walls — dorc why")
        );
    }

    #[test]
    fn why_detail_carries_the_unwall_count() {
        assert_eq!(
            hint(1, 0).why_detail(),
            "first wall (book order) — an oracle vouching its convergence would elide it and \
             un-wall 1 downstream site"
        );
        assert_eq!(
            hint(0, 0).why_detail(),
            "first wall (book order) — an oracle vouching its convergence would elide it when \
             converged"
        );
    }
}
