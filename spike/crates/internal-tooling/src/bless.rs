//! The conductor's one-shot pre-bless verification and golden re-bless, ported from
//! `e2e/conduct-bless.sh`.
//!
//! Remit: the conductor's tokens are the expensive ones, so this is silent on success bar
//! a single tally line plus the diffstat of what the bless moved, and loud-and-complete on
//! failure. BLESS is orchestrator-exclusive (`spike/CLAUDE.md`) — never run by a builder,
//! never while a build-agent is in flight.
//!
//! Deliberately thinner than the script it replaces: that predated `gate:full-quiet` and
//! re-spelled a fresh build, the whole suite, and the four lint gates by hand. Those are
//! that task's job now, so this owns only what is actually its own — the bless pass, the
//! receipt, and the diff a human has to eyeball.

use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

/// The floor binaries a mint measures under — the pair `276:rul-spec-two-binary-floor` names.
const FLOOR_SHELLS: [&str; 2] = ["dash", "posh"];

/// `internal-tooling bless [--dry] [--floor] [<case substring>...]`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    let dry = args.iter().any(|a| a == "--dry");
    let floor = args.iter().any(|a| a == "--floor");
    let cases: Vec<String> = args
        .iter()
        .filter(|arg| !arg.starts_with('-'))
        .cloned()
        .collect();
    let spike = internal_tooling::repo_root().join("spike");

    // The mint's pre-flight. `expected.emitted` is what the floor binaries AGREED on, so a mint
    // that can only ask one of them has nothing to commit — and finding that out after a ten-minute
    // green gate is the failure mode this refusal exists to spare. Windows is the live case:
    // git's userland ships no `posh`, so the mint belongs to the WSL leg there.
    if floor {
        let absent: Vec<String> = FLOOR_SHELLS
            .iter()
            .filter(|name| internal_tooling::Posix::floor(name).is_err())
            .map(|name| (*name).to_owned())
            .collect();
        if !absent.is_empty() {
            eprintln!(
                "bless: REFUSING the floor mint — {} not resolvable here, so the differential cannot be measured. Run it from WSL/*nix.",
                absent.join(" and ")
            );
            return ExitCode::from(2);
        }
    }

    // Both of these have bitten under WSL, and both bite EXPENSIVELY without a pre-flight:
    // `mise` is absent from a non-login shell, and a git older than this repo's
    // `relativeWorktrees` extension refuses the whole repository — which lands on the final
    // golden listing AFTER a ten-minute green run. A refusal beats a tail when the
    // environment, not the tree, is wrong.
    for (what, program, arg) in [
        ("mise", "mise", "--version"),
        ("git in this worktree", "git", "rev-parse"),
    ] {
        if !ok(Command::new(program).arg(arg).current_dir(&spike)) {
            eprintln!("bless: REFUSING — {what} does not work here");
            return ExitCode::from(2);
        }
    }
    vacate_own_image();

    // ORDER. Unscoped, the gate comes first: never re-bless from a tree you have not verified.
    // SCOPED, it cannot — a named case is being re-blessed precisely because it is red on
    // purpose (a sanctioned transcript drift), so a gate-first run fails on the very drift it was
    // asked to accept. The verification still happens, over the WHOLE tree, immediately after.
    let scoped = !cases.is_empty();
    let mut blessed = None;
    if scoped && !dry {
        let Some(out) = bless_pass(&spike, &cases, floor) else {
            return ExitCode::FAILURE;
        };
        blessed = Some(out);
    }
    if let Err(text) = step(
        &spike,
        "gate:full-quiet",
        Command::new("mise").args(["run", "gate:full-quiet"]),
    ) {
        if !scoped {
            teach_the_scoped_route(&spike, &text);
        }
        return ExitCode::FAILURE;
    }
    if !scoped && !dry {
        let Some(out) = bless_pass(&spike, &cases, floor) else {
            return ExitCode::FAILURE;
        };
        blessed = Some(out);
    }
    // A freshly-minted manifest has been committed but not yet CHECKED: `gate:full-quiet` names no
    // floor shell, so gate-9 stays inert there. Re-running the differential over the whole corpus
    // is what turns the mint into a measured claim rather than a write.
    if floor
        && step(
            &spike,
            "test:floor",
            Command::new("mise").args(["run", "test:floor"]),
        )
        .is_err()
    {
        return ExitCode::FAILURE;
    }

    let e2e = blessed.as_deref().and_then(passed_count).map_or_else(
        || "not blessed (dry)".to_owned(),
        |count| format!("{count} blessed"),
    );
    println!("{}", success_summary(&e2e));

    // The goldens the bless touched, and nothing else: the runners live beside the cases
    // they drive, so exclude them — this listing is about DATA.
    let _ = Command::new("git")
        .current_dir(&spike)
        .args([
            "--no-pager",
            "diff",
            "--stat",
            "--",
            "crates/cli/tests",
            ":!crates/cli/tests/*.rs",
        ])
        .status();
    ExitCode::SUCCESS
}

/// Move this process's own executable aside, so the build it is about to drive can replace it.
///
/// `bless` drives `gate:full-quiet`, whose `cargo build --workspace` re-uplifts every workspace
/// binary — this one included. Windows refuses to REMOVE a running image (`os error 5`), and
/// removing the old artifact is cargo's first step in an uplift, so the gate died before it began
/// (`300:finding-bless-driver-self-lock-on-windows`). Windows does permit RENAMING one: the
/// directory entry moves, this process keeps the image it already mapped, and the real path is
/// free for cargo to create fresh.
///
/// Copying and then re-execing the copy — the obvious shape — does NOT work: whoever waits for
/// the child is still the parent, still running from the real path, still the lock. The process
/// that drives the gate has to be the one that is no longer at the real path, and a rename is how
/// a running process gets there without a second process or a lost exit status.
///
/// Unconditional rather than `cfg(windows)`: gating it would leave the one platform that needs it
/// as the one platform that never exercises it — `one-platform-green-is-not-cross-platform-green`.
/// A failure here is a warning, not a refusal: it restores exactly today's behaviour rather than
/// blocking the conductor's only blessing path over a step that is a no-op on *nix anyway.
fn vacate_own_image() {
    let Ok(current) = std::env::current_exe() else {
        eprintln!(
            "bless: cannot locate this executable; a workspace build may refuse to replace it"
        );
        return;
    };
    let name = current.file_name().unwrap_or_default().to_string_lossy();
    let mut aside = current.clone();
    aside.set_file_name(format!("{name}.driver-image"));
    // Last run's image is not running now, so it can go; the extension it lands under is
    // irrelevant, because nothing ever executes this file — it only holds the inode open.
    let _ = std::fs::remove_file(&aside);
    if let Err(why) = std::fs::rename(&current, &aside) {
        eprintln!(
            "bless: could not move {} aside ({why}); a workspace build may refuse to replace it",
            current.display()
        );
    }
}

/// The `BLESS=1` e2e pass, over every case or only the ones whose names match `cases`.
///
/// The filter is the RUNNER's ordinary trial filter, so a scoped pass leaves every other golden
/// byte-identical — which is how one sanctioned drift stops carrying an unrelated one in with it.
/// That scoping is what makes `floor` safe to spell at all: the mint re-measures ONE named case's
/// manifest rather than re-opening every committed measurement in the corpus.
fn bless_pass(spike: &Path, cases: &[String], floor: bool) -> Option<String> {
    let mut command = Command::new("mise");
    command.args([
        "exec", "--", "cargo", "test", "-p", "dorc-cli", "--test", "e2e",
    ]);
    if !cases.is_empty() {
        command.arg("--");
        command.args(cases);
    }
    command.env("BLESS", "1").env("DORC_E2E_QUIET", "1");
    if floor {
        command
            .env("BLESS_FLOOR", "1")
            .env("DORC_E2E_FLOOR_SHELLS", FLOOR_SHELLS.join(","));
    }
    step(spike, "e2e --bless", &mut command).ok()
}

/// Run a labelled step, capturing combined output. On failure print the label and the
/// captured tail — fail loud, swallow nothing. Either way the caller gets what the step said,
/// because a caller may have something to add about WHY it failed; a step that could not be
/// spawned at all said nothing, and reports itself here rather than through that channel.
fn step(dir: &Path, label: &str, command: &mut Command) -> Result<String, String> {
    let out = command
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|why| {
            eprintln!("bless: could not run [{label}] ({why})");
            String::new()
        })?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    if out.status.success() {
        return Ok(text);
    }
    eprintln!(
        "bless: FAILED at [{label}] (exit {})",
        out.status.code().unwrap_or(-1)
    );
    for line in text
        .lines()
        .rev()
        .take(40)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        eprintln!("{line}");
    }
    Err(text)
}

/// The route an UNFILTERED bless cannot take, printed at the moment it is needed.
///
/// Unfiltered, the gate runs first — never re-bless from a tree you have not verified — so a
/// case that is red BECAUSE its drift is sanctioned fails the gate and the bless never happens.
/// The scoped form inverts that order, and this is where an operator finds that out: the tool
/// teaches the spelling rather than leaving it in a ledger somebody has to have read
/// (`307:work-bless-sanctioned-drift-spelling`).
fn teach_the_scoped_route(spike: &Path, gate_output: &str) {
    let selectors: Vec<String> = failed_cases(gate_output)
        .into_iter()
        .map(|case| selector_for(spike, case))
        .collect();
    if selectors.is_empty() {
        return;
    }
    eprintln!("{}", scoped_route_advice(&selectors));
}

/// Every case name the captured gate output reports a failure for.
///
/// Both corpus runners spell a per-case failure `FAIL  <name>  [<what>]`, and that shape — not
/// the ~45 individual gate labels — is what this reads: a label list would silently stop
/// matching the day one gate reworded itself, and silence is this advice's failure mode.
///
/// Matched MID-LINE, never anchored: by the time these bytes arrive they have been through the
/// nested task's own line prefixing and nextest's failure-section indent, so the marker is not at
/// the start of anything. Anchoring it is what made the first cut of this advice never fire.
/// nextest's own `FAIL [   1.4s] <trial>` line carries a single space and cannot be confused.
fn failed_cases(gate_output: &str) -> Vec<&str> {
    let mut cases: Vec<&str> = gate_output
        .lines()
        .filter_map(|line| line.split_once("FAIL  "))
        .filter_map(|(_, rest)| rest.split_once("  ["))
        .map(|(name, _)| name)
        .collect();
    cases.sort_unstable();
    cases.dedup();
    cases
}

/// The path selector for a case name, or the bare name when no case root claims it.
///
/// A case is a directory or a single-file `.loom` under some `crates/*/tests/`, and the runners
/// resolve either to the exact trial. Printing the resolved path is the whole point: a selector
/// an operator has to construct is one they can get wrong.
fn selector_for(spike: &Path, case: &str) -> String {
    let Ok(crates) = std::fs::read_dir(spike.join("crates")) else {
        return case.to_owned();
    };
    for entry in crates.flatten() {
        let tests = entry.path().join("tests");
        for candidate in [tests.join(case), tests.join(format!("{case}.loom"))] {
            if candidate.exists() {
                return candidate
                    .strip_prefix(spike)
                    .unwrap_or(&candidate)
                    .to_string_lossy()
                    .replace('\\', "/");
            }
        }
    }
    case.to_owned()
}

/// The advice itself: both spellings, and why the unfiltered one cannot serve.
fn scoped_route_advice(selectors: &[String]) -> String {
    let command = format!("      mise run bless -- {}", selectors.join(" "));
    let substring_note = if selectors.len() == 1 {
        "The bare-substring form, `mise run bless -- <substring>`, is libtest's own filter and \
         takes exactly one."
    } else {
        "The bare-substring form, `mise run bless -- <substring>`, is libtest's own filter and \
         takes exactly one, so it cannot express this set."
    };
    format!(
        "bless: an UNFILTERED bless verifies BEFORE it writes, so it can never accept a \
         SANCTIONED drift — such a case is red until it is blessed, and that redness is what \
         stopped the gate above. Scoping inverts the order (bless the named cases, then verify \
         the whole tree):\n{command}\nThose are case PATHS, which the runner resolves to exact \
         trial names and which is the only form that can name SEVERAL cases. {substring_note}"
    )
}

/// The count before `passed`, from the directly-run bless pass's test summary.
fn passed_count(output: &str) -> Option<String> {
    let line = output.lines().rev().find(|line| line.contains(" passed"))?;
    let tokens: Vec<&str> = line.split_whitespace().collect();
    tokens
        .windows(2)
        .find(|pair| pair.get(1).is_some_and(|t| t.starts_with("passed")))
        .and_then(|pair| pair.first().copied())
        .map(str::to_owned)
}

fn success_summary(e2e: &str) -> String {
    format!("bless: gates ok | e2e {e2e}")
}

/// Did the command run and succeed? (Output discarded; this is a pre-flight, not a step.)
fn ok(command: &mut Command) -> bool {
    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

#[cfg(test)]
mod tests {
    use super::{failed_cases, scoped_route_advice, success_summary};

    #[test]
    fn dry_success_reports_only_the_completed_gate_and_bless_status() {
        assert_eq!(
            success_summary("not blessed (dry)"),
            "bless: gates ok | e2e not blessed (dry)"
        );
    }

    #[test]
    fn the_captured_gate_output_yields_each_failing_case_once() {
        // Verbatim shape, captured 2026-08-16 from an unfiltered `bless:dry` over a planted
        // drift: the nested task's line prefix and nextest's failure-section indent both sit
        // to the LEFT of the marker, which is why nothing here may be anchored.
        let captured = "\
[gate:full-quiet]      Summary [ 146.445s] 2139 tests run: 2138 passed, 1 failed, 1 skipped
[gate:full-quiet]         FAIL [   1.460s] dorc-cli::e2e top-eval
[gate:full-quiet]   stdout ───
[gate:full-quiet]     FAIL  top-eval  [content diff]
[gate:full-quiet]           15 committed lines, 15 fresh
[gate:full-quiet]             - \"apt-get install -y curl-PLANTED-DRIFT\"
[gate:full-quiet]     FAIL  whygallery-webhost-whole  [replay 1: `dorc why` no longer reproduces its committed transcript]
[gate:full-quiet]     FAIL  top-eval  [content diff]
[gate:full-quiet] error: test run failed
";
        assert_eq!(
            failed_cases(captured),
            vec!["top-eval", "whygallery-webhost-whole"]
        );
    }

    #[test]
    fn a_gate_that_failed_on_something_other_than_a_case_names_no_cases() {
        // A clippy or fmt refusal must not draw a bless-scoping lecture: the advice is about a
        // case whose golden moved, and warning-fatigue is what it would cost otherwise.
        let captured = "error: unused variable: `x`\nerror: could not compile `dorc-plan`\n";
        assert!(failed_cases(captured).is_empty());
    }

    #[test]
    fn the_advice_carries_both_selector_spellings() {
        let advice = scoped_route_advice(&[
            "crates/cli/tests/top-eval".to_owned(),
            "crates/aid/tests/cli-help-page.loom".to_owned(),
        ]);
        assert!(
            advice.contains(
                "mise run bless -- crates/cli/tests/top-eval crates/aid/tests/cli-help-page.loom"
            ),
            "the PATHS form has to arrive ready to paste: {advice}"
        );
        assert!(advice.contains("PATHS"), "{advice}");
        assert!(advice.contains("mise run bless -- <substring>"), "{advice}");
        // The whole reason the route exists, which is what an operator is missing when they
        // reach an unfiltered bless with a sanctioned drift in the tree.
        assert!(advice.contains("verifies BEFORE it writes"), "{advice}");
        assert!(advice.contains("stopped the gate above"), "{advice}");
        assert!(advice.contains("inverts the order"), "{advice}");
    }
}
