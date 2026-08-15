//! The Kani lane driver — the `pinned` badge's evidence source (`301` §5).
//!
//! Kani answers two different questions and the binder needs both. `cargo kani list` reports
//! the harnesses that EXIST, which is what a catalogue citation is resolved against; a
//! verification run reports which of them are GREEN at their declared bounds. A `pinned` badge
//! needs the pair, and it needs the first from the toolchain rather than from a grep over
//! source text: a name that no longer exists must fail to resolve, and a `#[kani::proof]`
//! commented out is exactly the case string-matching cannot see.
//!
//! # Why this drives Kani one harness at a time
//!
//! A bounded model checker has no graceful degradation: a harness whose formula blows up does
//! not slow down, it consumes the machine. Measured on this corpus, one CBMC process reached
//! 3.6 GB in twenty-one minutes before reporting its own out-of-memory, and a later unattended
//! battery run took a whole 15 GiB VM down with it. Two consequences are built in here rather
//! than left to whoever runs the lane:
//!
//! * **A per-harness wall-clock budget.** Past it the harness is over-budget, which is a
//!   FINDING to report — a formula that needs a different shape — never something to wait out.
//! * **An explicit reaper after every harness.** Killing `cargo-kani` does not kill CBMC: it is
//!   a grandchild, it survives, and a survivor competes with the next harness for the same
//!   memory. `pkill -9 -x cbmc` runs between harnesses, exact-name only — a `-f` match would
//!   also match this process's own command line.
//!
//! The lane is opt-in and Linux/WSL-only (upstream publishes no Windows asset). Every failure
//! mode below is loud and names its remedy — an absent toolchain is never a silent pass, the
//! same posture the real-tools lint lane takes (`real-tools-lane-opt-in`).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// The per-harness wall-clock budget, in seconds, unless `DORC_KANI_HARNESS_BUDGET_SECS` says
/// otherwise. Two minutes: every harness that verifies at all on this corpus does so inside
/// three seconds, so the budget is not a race — it is the line past which a harness is
/// understood to have blown up rather than to be working.
const DEFAULT_BUDGET_SECS: u64 = 120;

/// How often the driver checks whether a harness has finished.
const POLL: Duration = Duration::from_millis(250);

/// How long past its own deadline a harness may linger before this process kills it outright.
const GRACE: Duration = Duration::from_secs(30);

/// The per-harness address-space cap, in KiB (~6 GiB) — `ulimit -v`, inherited by CBMC.
///
/// Sized well under the VM it runs in, because the failure being prevented is not a slow
/// harness: it is one CBMC taking the whole machine down, which has now happened twice. A
/// harness that wants more address space than this is over-budget by definition — the finding
/// is that its formula needs a different shape.
const ADDRESS_SPACE_CAP_KB: u64 = 6_000_000;

/// Why the lane could not produce evidence. Distinct from a harness FAILING, which is a real
/// finding about the code and is reported as one.
#[derive(Debug)]
pub enum Refusal {
    /// Not this platform.
    UnsupportedPlatform,
    /// The toolchain is not installed. Carries the task that installs it.
    ToolAbsent(String),
    /// Kani ran and something went wrong that is not a verification verdict.
    ToolFailed(String),
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedPlatform => f.write_str(
                "the Kani lane is Linux/WSL only (upstream publishes no Windows asset); \
                 run it from the WSL leg",
            ),
            Self::ToolAbsent(remedy) => write!(f, "cargo-kani is not on PATH — run `{remedy}`"),
            Self::ToolFailed(why) => write!(f, "{why}"),
        }
    }
}

/// What one lane run learned.
#[derive(Debug, Default)]
pub struct Report {
    /// Every harness the TOOLCHAIN reported, whatever its verdict — the resolution universe a
    /// catalogue citation is checked against.
    pub harnesses: BTreeSet<String>,
    /// The harnesses that verified at their declared bounds.
    pub green: BTreeSet<String>,
    /// The harnesses that reported a verification failure. Any entry here is a REAL FINDING: a
    /// counterexample is a bug in the code or in the law, never harness noise
    /// (`law-never-weaken-the-question`).
    pub failed: BTreeSet<String>,
    /// The harnesses killed at the budget. Also a finding, and a different one: the law is
    /// unjudged, and the harness needs a shape the checker can afford.
    pub over_budget: BTreeSet<String>,
    /// Per-harness wall-clock, in the order run — the instrument that shows a harness drifting
    /// toward the budget before it crosses it.
    pub timings: Vec<(String, Duration)>,
}

impl Report {
    /// Whether `harness` exists and verified.
    #[must_use]
    pub fn is_green(&self, harness: &str) -> bool {
        self.green.contains(harness)
    }

    /// Whether the toolchain knows `harness` at all.
    #[must_use]
    pub fn resolves(&self, harness: &str) -> bool {
        self.harnesses.contains(harness)
    }

    /// Whether every harness that ran came back green.
    #[must_use]
    pub fn all_green(&self) -> bool {
        self.failed.is_empty() && self.over_budget.is_empty()
    }
}

/// The verification unit's directory: the detached crate at `spike/verify/kani`.
#[must_use]
pub fn unit_dir(repo_root: &Path) -> PathBuf {
    repo_root.join("spike").join("verify").join("kani")
}

/// The lane's build root: user-local and ext4, for the same reason the Lean lane's is. A drvfs
/// (`/mnt/c`) target directory makes the WSL leg unusably slow, and sharing `spike/target/`
/// would let a Kani-compiled artifact collide with the ordinary build's.
#[must_use]
pub fn build_root() -> PathBuf {
    std::env::var_os("DORC_KANI_TARGET_DIR").map_or_else(
        || {
            let cache = std::env::var_os("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
                .unwrap_or_else(std::env::temp_dir);
            cache.join("dorc-kani-target")
        },
        PathBuf::from,
    )
}

fn budget() -> Duration {
    let secs = std::env::var("DORC_KANI_HARNESS_BUDGET_SECS")
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .unwrap_or(DEFAULT_BUDGET_SECS);
    Duration::from_secs(secs)
}

/// Install Kani's engine bundle. Idempotent; the bundle and the nightly it needs are Kani's
/// own homes (`~/.kani`, rustup's toolchain store) and it offers no redirect for either.
///
/// # Errors
/// [`Refusal`] when the platform is wrong or the shim is not installed.
pub fn setup() -> Result<(), Refusal> {
    if cfg!(windows) {
        return Err(Refusal::UnsupportedPlatform);
    }
    let status = Command::new("cargo-kani")
        .arg("setup")
        .status()
        .map_err(|e| map_spawn_error(&e))?;
    if status.success() {
        Ok(())
    } else {
        Err(Refusal::ToolFailed(format!("cargo-kani setup: {status}")))
    }
}

/// Run the lane: enumerate the harnesses, then verify them one at a time under a budget,
/// reaping between each. `progress` receives one line per harness AS IT LANDS — a full battery
/// is tens of minutes, and a run interrupted partway through must not lose the verdicts it has
/// already earned.
///
/// # Errors
/// [`Refusal`], for anything that is not a verification verdict.
pub fn run(
    repo_root: &Path,
    filter: Option<&str>,
    progress: &mut dyn FnMut(&str),
) -> Result<Report, Refusal> {
    if cfg!(windows) {
        return Err(Refusal::UnsupportedPlatform);
    }
    let unit = unit_dir(repo_root);
    // Kani's `--exact` filter matches only a FULLY-QUALIFIED name, while a catalogue cites the
    // harness function. Both live here: the qualified spellings drive invocation, the bare ones
    // are the citation universe.
    let qualified = parse_harness_list(&invoke_capturing(&unit, &["list".to_owned()])?);
    if qualified.is_empty() {
        return Err(Refusal::ToolFailed(
            "cargo-kani list named no harnesses — either the unit has none, or this Kani's \
             list format no longer matches `dorc_verify::kani::parse_harness_list`"
                .to_owned(),
        ));
    }
    let mut report = Report {
        harnesses: qualified.iter().map(|q| bare_name(q)).collect(),
        ..Report::default()
    };

    let selected: Vec<String> = match filter {
        None => qualified.iter().cloned().collect(),
        Some(name) => {
            let bare = bare_name(name);
            let matched: Vec<String> = qualified
                .iter()
                .filter(|q| bare_name(q) == bare)
                .cloned()
                .collect();
            if matched.is_empty() {
                return Err(Refusal::ToolFailed(format!(
                    "no harness named `{bare}` — the toolchain lists {}",
                    qualified.len()
                )));
            }
            matched
        }
    };

    let budget = budget();
    for path in selected {
        let outcome = verify_one(&unit, &path, budget);
        reap();
        let name = bare_name(&path);
        match outcome {
            Ok(Outcome::Green(elapsed)) => {
                progress(&format!(
                    "{:>8.2}s  green        {name}",
                    elapsed.as_secs_f64()
                ));
                report.timings.push((name.clone(), elapsed));
                report.green.insert(name);
            }
            Ok(Outcome::Failed(elapsed)) => {
                progress(&format!(
                    "{:>8.2}s  FAILED       {name}",
                    elapsed.as_secs_f64()
                ));
                report.timings.push((name.clone(), elapsed));
                report.failed.insert(name);
            }
            Ok(Outcome::OverBudget) => {
                progress(&format!(
                    "{:>8.2}s  OVER-BUDGET  {name}",
                    budget.as_secs_f64()
                ));
                report.timings.push((name.clone(), budget));
                report.over_budget.insert(name);
            }
            Err(why) => return Err(why),
        }
    }
    Ok(report)
}

enum Outcome {
    Green(Duration),
    Failed(Duration),
    OverBudget,
}

/// Verify one harness behind BOTH memory gates.
///
/// The gates are `ulimit -v` and `timeout`, applied by a shell wrapper rather than by this
/// process, and that is deliberate: an address-space cap has to be set on the process that
/// allocates, and CBMC is a grandchild — Kani's driver spawns it. Capping here would cap the
/// wrong process; capping the shell that becomes `cargo-kani` is inherited all the way down.
///
/// Both gates are the lane's own, not a habit anyone has to remember at the call site. Twice
/// now an unattended battery run has taken a whole VM down, and the second time was after the
/// discipline was already known.
fn verify_one(unit: &Path, name: &str, budget: Duration) -> Result<Outcome, Refusal> {
    if name.contains('\'') {
        return Err(Refusal::ToolFailed(format!(
            "harness name `{name}` carries a quote and cannot be passed through the gate shell"
        )));
    }
    let started = Instant::now();
    // `exec` so the timeout signals cargo-kani itself rather than an intervening shell; `-k 10`
    // follows an ignored TERM with a KILL ten seconds later.
    let gated = format!(
        "ulimit -v {ADDRESS_SPACE_CAP_KB}; exec timeout -k 10 {} \
         cargo-kani --harness '{name}' --exact --output-format terse",
        budget.as_secs()
    );
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&gated)
        .current_dir(unit)
        .env("CARGO_TARGET_DIR", build_root())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| map_spawn_error(&e))?;

    // A belt-and-braces poll past the shell's own deadline: if `timeout` itself were missing or
    // wedged, this still ends the harness rather than the machine.
    let hard_stop = budget.saturating_add(GRACE);
    loop {
        match child.try_wait() {
            Err(e) => return Err(Refusal::ToolFailed(format!("cargo-kani {name}: {e}"))),
            Ok(Some(_)) => break,
            Ok(None) => {
                if started.elapsed() >= hard_stop {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Ok(Outcome::OverBudget);
                }
                std::thread::sleep(POLL);
            }
        }
    }

    let out = child
        .wait_with_output()
        .map_err(|e| Refusal::ToolFailed(format!("cargo-kani {name}: {e}")))?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    let elapsed = started.elapsed();

    // THE GATE CHECK COMES FIRST, and the order is load-bearing. CBMC prints
    // `VERIFICATION:- FAILED` after running out of memory, so a verdict-first reading turns
    // every gate trip into a counterexample — announcing a bug nobody found, in the one report
    // whose entire value is that it does not do that. An OOM'd run has no verdict, whatever it
    // printed on the way down.
    if tripped_a_gate(out.status.code(), &text) {
        return Ok(Outcome::OverBudget);
    }
    if text.contains("VERIFICATION:- SUCCESSFUL") {
        return Ok(Outcome::Green(elapsed));
    }
    if text.contains("VERIFICATION:- FAILED") {
        return Ok(Outcome::Failed(elapsed));
    }
    // No verdict and no gate trip is a BROKEN RUN, not a failing law. Rounding it up to
    // "failed" would report a counterexample nobody found; rounding it to over-budget would
    // hide a real breakage behind a resource excuse.
    Err(Refusal::ToolFailed(format!(
        "cargo-kani --harness {name} exited {} with no verdict:\n{text}",
        out.status
    )))
}

/// Did the harness die on one of the two gates rather than answer?
///
/// `timeout` reports 124 for the deadline and 137 for its own follow-up KILL. An address-space
/// refusal surfaces as whatever allocation failure the component noticed first, and CBMC, the
/// allocator, and Kani's driver each word it differently — so the memory case is matched on the
/// vocabulary rather than on an exit code none of them agree about.
fn tripped_a_gate(code: Option<i32>, text: &str) -> bool {
    matches!(code, Some(124 | 137)) || OUT_OF_MEMORY.iter().any(|marker| text.contains(marker))
}

const OUT_OF_MEMORY: [&str; 6] = [
    "out of memory",
    "Out of memory",
    "CBMC failed with status",
    "bad_alloc",
    "Cannot allocate memory",
    "memory allocation of",
];

/// Kill any surviving CBMC. Exact-name only: `pkill -f cbmc` matches every command line
/// carrying the string, including this driver's own.
fn reap() {
    let _ = Command::new("pkill")
        .args(["-9", "-x", "cbmc"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

fn invoke_capturing(unit: &Path, args: &[String]) -> Result<String, Refusal> {
    let out = Command::new("cargo-kani")
        .args(args)
        .current_dir(unit)
        .env("CARGO_TARGET_DIR", build_root())
        .output()
        .map_err(|e| map_spawn_error(&e))?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    if !out.status.success() {
        return Err(Refusal::ToolFailed(format!(
            "cargo-kani {} exited {}:\n{text}",
            args.join(" "),
            out.status
        )));
    }
    Ok(text)
}

fn map_spawn_error(e: &std::io::Error) -> Refusal {
    if e.kind() == std::io::ErrorKind::NotFound {
        Refusal::ToolAbsent("mise run verify:kani-setup".to_owned())
    } else {
        Refusal::ToolFailed(format!("cargo-kani: {e}"))
    }
}

/// Pull FULLY-QUALIFIED harness names out of `cargo kani list`'s report. Qualified because
/// that is the only spelling Kani's `--exact` filter accepts; [`bare_name`] projects them down
/// to the citation vocabulary afterwards.
///
/// The report renders a three-column table — a blank marker column, the crate, and the
/// fully-qualified harness path — bracketed by `+---+` rules. Rows are recognized structurally
/// (three pipe-delimited fields whose last one carries a `::` path), so the header, the rules
/// and the `Total` footer fall out without being enumerated. A format drift that defeats this
/// empties the list, and an empty list is a refusal rather than "nothing resolves".
fn parse_harness_list(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }
        let fields: Vec<&str> = trimmed.trim_matches('|').split('|').collect();
        let [_marker, _crate_name, path] = fields.as_slice() else {
            continue;
        };
        let path = path.trim();
        if path.contains("::") {
            out.insert(path.to_owned());
        }
    }
    out
}

/// The last path segment of a fully-qualified harness name. A catalogue cites the harness
/// FUNCTION, and the module path around it is a refactoring detail that must not break a
/// citation.
fn bare_name(qualified: &str) -> String {
    qualified
        .trim()
        .rsplit("::")
        .next()
        .unwrap_or(qualified)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_harness_list_is_read_from_the_toolchain_not_from_source_text() {
        // The whole point of `cargo kani list`: a `#[kani::proof]` that was commented out still
        // matches a grep of the source and does NOT appear here. The bytes below are Kani
        // 0.67's real report shape, header rules and `Total` footer included, because those are
        // what the parser has to walk past.
        let rendered = "\
Contracts:
No contracts or contract harnesses found.

Standard Harnesses (#[kani::proof]):
+-------+-----------+------------------------------------------------------+
|       | Crate     | Harness                                              |
+==============================================================================+
|       | dorc_kani | harness::facade::set_insert_preserves_canonical_form  |
|-------+-----------+------------------------------------------------------|
|       | dorc_kani | harness::lattice_laws::flat_obeys_the_binary_laws     |
+-------+-----------+------------------------------------------------------+
| Total |           | 2                                                    |
+-------+-----------+------------------------------------------------------+
";
        let found = parse_harness_list(rendered);
        assert!(found.contains("harness::facade::set_insert_preserves_canonical_form"));
        assert!(found.contains("harness::lattice_laws::flat_obeys_the_binary_laws"));
        assert_eq!(found.len(), 2, "the footer and the rules are not harnesses");
    }

    #[test]
    fn a_citation_resolves_by_function_name_not_by_module_path() {
        // A catalogue cites the harness FUNCTION. Moving one between harness modules is a
        // refactor, and a refactor that silently unpinned every law citing it would be the
        // rot this binder exists to catch, wearing the binder's own clothes.
        assert_eq!(
            bare_name("dorc_kani::harness::facade::set_structural_eq_is_set_eq"),
            "set_structural_eq_is_set_eq"
        );
        assert_eq!(bare_name("bare_already"), "bare_already");
    }

    #[test]
    fn a_memory_gate_trip_is_not_read_as_a_counterexample() {
        // MEASURED, not imagined: CBMC prints `VERIFICATION:- FAILED` on its own way down after
        // exhausting the address-space cap. Reading the verdict first turned every gate trip in
        // the first full battery into a "counterexample" — three laws reported broken that
        // nothing had refuted. The gate check runs first for exactly this.
        let ooms = "\
Runtime Convert SSA: 4.6953s
Out of memory

CBMC failed with status 6
VERIFICATION:- FAILED
";
        assert!(
            tripped_a_gate(Some(1), ooms),
            "an out-of-memory run is a gate trip, not a verdict"
        );
        assert!(tripped_a_gate(Some(124), ""), "the timeout's own exit code");
        assert!(
            !tripped_a_gate(Some(1), "VERIFICATION:- FAILED\n"),
            "a plain failure is still a real finding about the law"
        );
    }

    #[test]
    fn an_over_budget_harness_is_not_green_and_is_not_a_counterexample() {
        // The three outcomes are three different claims and the report keeps them apart. A
        // harness killed at the budget has proved nothing AND refuted nothing: reporting it as
        // failed would announce a counterexample nobody found, and reporting it as green would
        // pin a law on a run that never finished.
        let mut report = Report::default();
        report.harnesses.insert("blown_up_harness".to_owned());
        report.over_budget.insert("blown_up_harness".to_owned());

        assert!(report.resolves("blown_up_harness"), "it exists");
        assert!(!report.is_green("blown_up_harness"), "but it is not pinned");
        assert!(
            !report.failed.contains("blown_up_harness"),
            "and it is not a finding about the law"
        );
        assert!(!report.all_green());
    }
}
