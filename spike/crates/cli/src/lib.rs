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
//! What lives here is exactly the PURE invocation surface: usage text, the parsed shapes, and the
//! parsers. Every I/O edge — reading files, resolving oracle dirs, `std::env::args`, printing —
//! stays in `main.rs` (`io-at-edges-only`). Nothing here reads the world; the parsers are total
//! functions of their argv, which is what makes the harness deterministic (`inv-determinism`).
//!
//! Do not grow this into a general-purpose library. If something here starts wanting a clock, a
//! file, or an environment read, it belongs on the other side of the seam.

#![forbid(unsafe_code)]

use dorc_aid::Severity;
use dorc_aid::arrangement::{CONST_ARRANGEMENTS, arrangement_text};
use dorc_aid::diag::{Diag, DiagCode};

/// The invocation-error carrier (`288` §6). A plain [`Diag`]: the parsers hand the print seat the
/// same typed value every other surface carries, and boxing it would buy nothing but an indirection
/// on a path that runs at most once per process.
pub type InvocationError = Diag;

/// The arrangement slug of the one-line usage synopsis every invocation-error print seat
/// appends. Its words live in the arrangement registry, like every other user-facing string
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`); [`usage_text`] renders it.
pub const USAGE_ARRANGEMENT: &str = "cli-usage-synopsis";

/// The arrangement slug of the long help page `--help`/`-h` prints
/// (`288:rul-help-text-is-loomable`). Its defining loom drives `$ dorc --help` and its
/// transcript IS the editing surface for the page's prose.
pub const HELP_ARRANGEMENT: &str = "cli-help-page";

/// The one-line usage synopsis, appended to argument errors by the print seats.
#[must_use]
pub fn usage_text() -> String {
    arrangement_text(&CONST_ARRANGEMENTS, USAGE_ARRANGEMENT, None)
}

/// The long help (ack-1 + the cheap help-is-success item): `--help`/`-h` prints this to STDOUT
/// and exits 0 (a help request is a success, not a usage error).
#[must_use]
pub fn help_text() -> String {
    arrangement_text(&CONST_ARRANGEMENTS, HELP_ARRANGEMENT, None)
}

/// What the arg-parse resolved to: an analysis run, or a help/version request (both of which
/// are successes printed to stdout, ack-1 help-is-success — never a usage error).
#[derive(Debug)]
pub enum Invocation {
    /// A normal analysis run with the parsed [`Args`].
    Analyze(Args),
    /// `-h`/`--help`: print [`help_text`] to stdout, exit 0.
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
    /// The book(s) to analyze — a positional (`dorc plan book.sh`, the day-one ergonomic) OR
    /// `--book=PATH`, repeatable. Multiple books CONCATENATE into one analyzed unit (a book split
    /// across files reads as one). At least one is required.
    pub books: Vec<String>,
    /// `-o`/`--oracle PATH`: the explicitly-named oracle files, in argv order.
    pub oracles: Vec<String>,
    /// `--oracle-dir DIR` (ack-6): load every `*.oracle.sh` in DIR (glob-sorted, deterministic),
    /// repeatable — the explicit bulk form alongside `-o` for the spike.
    pub oracle_dirs: Vec<String>,
    /// `--results FILE` (flow pick): read the probe results from FILE instead of the default stdin.
    pub results: Option<String>,
    /// `--debug-argv` (gate-5 / cm-2): emit the engine's per-site resolved argv to stderr,
    /// then proceed normally — a cli-edge readout the e2e argv-echo differential consumes.
    pub debug_argv: bool,
    /// `--trust-footprints` (rul24-mode-gate): opt into the survival tier — a converged line
    /// may ELIDE past a RUNNING wall when the wall's authored `touches()` footprint is disjoint
    /// from the line's fact's backing (Stage 2, the golden hill). DEFAULT OFF; not recommended
    /// by hints/docs beyond noting availability. Honest framing (24A §1a-addendum): marketing at
    /// best (the admin chose the danger), theatre at worst (everyone enables it) — demanded
    /// anyway as the non-vacuous CYA. When off, the footprints are never even lifted (TC-1).
    pub trust_footprints: bool,
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
    /// memory, and it still means something on the other modes.
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
    last || (matches!(mode, Mode::Why) && !has_results)
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
    if raw.iter().any(|a| a == "-h" || a == "--help") {
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
    let mut oracles = Vec::new();
    let mut oracle_dirs = Vec::new();
    let mut results: Option<String> = None;
    let mut debug_argv = false;
    let mut trust_footprints = false;
    let mut why_address: Option<String> = None;
    let mut dial = dorc_core::EscalationDial::VouchedOnly;
    let mut capability = dorc_core::Capability::Root;
    let mut whylog_dir: Option<String> = None;
    let mut whylog: Option<String> = None;
    let mut last = false;
    let mut no_whylog = false;
    let mut all = false;
    let mut shim_dir: Option<String> = None;
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
            if let Some(sugg) = nearest(w, &["probe", "plan", "apply", "why", "strip"]) {
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
        } else if arg == "-o" || arg == "--oracle" {
            oracles.push(it.next().ok_or_else(|| flag_needs_value("-o", "a path"))?);
        } else if let Some(p) = arg.strip_prefix("-o").filter(|p| !p.is_empty()) {
            oracles.push(p.to_string());
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
        } else if arg == "--trust-footprints" {
            trust_footprints = true;
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
        } else if arg == "--all" {
            all = true;
        } else if arg == "--last" {
            last = true;
        } else if arg == "--no-whylog" {
            no_whylog = true;
        } else if let Some(p) = arg.strip_prefix("--shim-dir=") {
            shim_dir = Some(p.to_string());
        } else if arg == "--shim-dir" {
            shim_dir = Some(
                it.next()
                    .ok_or_else(|| flag_needs_value("--shim-dir", "a directory"))?,
            );
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
            // A bare word (no `-`): a positional book (the day-one `dorc plan book.sh` ergonomic;
            // repeatable ⇒ multi-book concatenation).
            books.push(arg);
        }
    }
    // A receipt names its own book (`28E:lean-why-is-whylog-reconciliation`), so demanding one on
    // the command line would make the fold's whole point -- asking "why did that happen" with
    // nothing in hand -- impossible to type. Records handed in still need one: they describe a book
    // this invocation is not otherwise told about.
    if books.is_empty() && !reads_the_receipt(mode, last, results.is_some()) {
        return Err(Diag::new_spanless_site(DiagCode::CliNoBookGiven(
            dorc_aid::diag::CliNoBookGiven,
        )));
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
    Ok(Invocation::Analyze(Args {
        mode,
        books,
        oracles,
        oracle_dirs,
        results,
        debug_argv,
        trust_footprints,
        why_address,
        dial,
        capability,
        whylog_dir,
        no_whylog,
        whylog,
        last,
        all,
        shim_dir,
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
                detail: err.to_string(),
            },
        )),
    }
}

/// The parsed `dorc lint` invocation (`27R` §5). Files + oracle sources + the render/exit knobs.
#[derive(Debug)]
pub struct LintArgs {
    /// The lintable files given as positionals.
    pub files: Vec<String>,
    /// `-o`/`--oracle PATH`: oracle sources to lint as oracles.
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
        } else if arg == "-o" || arg == "--oracle" {
            oracles.push(it.next().ok_or_else(|| flag_needs_value("-o", "a path"))?);
        } else if let Some(p) = arg.strip_prefix("-o").filter(|p| !p.is_empty()) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(argv: &[&str]) -> Invocation {
        parse_args_from(argv.iter().map(|word| (*word).to_owned()).collect())
            .expect("invocation parses")
    }

    fn analyzed(words: &[&str]) -> Args {
        match parse(words) {
            Invocation::Analyze(args) => args,
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

    /// A book is required exactly when the invocation cannot learn one from a receipt. The
    /// `--results`-without-a-book row is the one worth having: records describe a book, and
    /// accepting them with none named would analyse the empty string and report on nothing.
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
            after_last.books.is_empty(),
            "and must not be mistaken for a positional book"
        );

        // The `why`-mode carve does not leak: every other mode still reads a bare word as a book.
        let planned = args(&["plan", "book.sh"]);
        assert_eq!(planned.books, vec!["book.sh".to_owned()]);
    }
}
