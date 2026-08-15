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
/// reaping between each.
///
/// # Errors
/// [`Refusal`], for anything that is not a verification verdict.
pub fn run(repo_root: &Path, filter: Option<&str>) -> Result<Report, Refusal> {
    if cfg!(windows) {
        return Err(Refusal::UnsupportedPlatform);
    }
    let unit = unit_dir(repo_root);
    let mut report = Report {
        harnesses: parse_harness_list(&invoke_capturing(&unit, &["list".to_owned()])?),
        ..Report::default()
    };
    if report.harnesses.is_empty() {
        return Err(Refusal::ToolFailed(
            "cargo-kani list named no harnesses — either the unit has none, or this Kani's \
             list format no longer matches `dorc_verify::kani::parse_harness_list`"
                .to_owned(),
        ));
    }

    let selected: Vec<String> = match filter {
        None => report.harnesses.iter().cloned().collect(),
        Some(name) => {
            let bare = bare_name(name);
            if !report.harnesses.contains(&bare) {
                return Err(Refusal::ToolFailed(format!(
                    "no harness named `{bare}` — the toolchain lists {}",
                    report.harnesses.len()
                )));
            }
            vec![bare]
        }
    };

    let budget = budget();
    for name in selected {
        let outcome = verify_one(&unit, &name, budget);
        reap();
        match outcome {
            Ok(Outcome::Green(elapsed)) => {
                report.timings.push((name.clone(), elapsed));
                report.green.insert(name);
            }
            Ok(Outcome::Failed(elapsed)) => {
                report.timings.push((name.clone(), elapsed));
                report.failed.insert(name);
            }
            Ok(Outcome::OverBudget) => {
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

/// Verify one harness, killing it at the budget.
fn verify_one(unit: &Path, name: &str, budget: Duration) -> Result<Outcome, Refusal> {
    let started = Instant::now();
    let mut child = Command::new("cargo-kani")
        .args(["--harness", name, "--exact", "--output-format", "terse"])
        .current_dir(unit)
        .env("CARGO_TARGET_DIR", build_root())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| map_spawn_error(&e))?;

    loop {
        match child.try_wait() {
            Err(e) => return Err(Refusal::ToolFailed(format!("cargo-kani {name}: {e}"))),
            Ok(Some(_)) => break,
            Ok(None) => {
                if started.elapsed() >= budget {
                    // The child is `cargo-kani`; CBMC is its grandchild and outlives it. The
                    // caller's reap is what actually frees the memory.
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

    if text.contains("VERIFICATION:- SUCCESSFUL") {
        return Ok(Outcome::Green(elapsed));
    }
    if text.contains("VERIFICATION:- FAILED") {
        return Ok(Outcome::Failed(elapsed));
    }
    // No verdict at all is not a failing law — it is a broken run, and rounding it up to
    // "failed" would report a counterexample that does not exist.
    Err(Refusal::ToolFailed(format!(
        "cargo-kani --harness {name} exited {} with no verdict:\n{text}",
        out.status
    )))
}

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

/// Pull harness names out of `cargo kani list`'s report.
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
            out.insert(bare_name(path));
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
        assert!(found.contains("set_insert_preserves_canonical_form"));
        assert!(found.contains("flat_obeys_the_binary_laws"));
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
