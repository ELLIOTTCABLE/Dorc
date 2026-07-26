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

/// `internal-tooling bless [--dry]`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    let dry = args.iter().any(|a| a == "--dry");
    let spike = internal_tooling::repo_root().join("spike");

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

    let Some(gate) = step(
        &spike,
        "gate:full-quiet",
        Command::new("mise").args(["run", "gate:full-quiet"]),
    ) else {
        return ExitCode::FAILURE;
    };

    let blessed = if dry {
        None
    } else {
        let mut command = Command::new("mise");
        command.args([
            "exec", "--", "cargo", "test", "-p", "dorc-cli", "--test", "e2e",
        ]);
        command.env("BLESS", "1").env("DORC_E2E_QUIET", "1");
        let Some(out) = step(&spike, "e2e --bless", &mut command) else {
            return ExitCode::FAILURE;
        };
        Some(out)
    };

    let suite = passed(&gate).unwrap_or_else(|| "?".to_owned());
    let e2e = blessed.as_deref().and_then(passed).map_or_else(
        || "not blessed (dry)".to_owned(),
        |count| format!("{count} blessed"),
    );
    println!("bless: gates ok | suite {suite} | e2e {e2e}");

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

/// Run a labelled step, capturing combined output. On failure print the label and the
/// captured tail, then hand the caller a `None` to abort on — fail loud, swallow nothing.
fn step(dir: &Path, label: &str, command: &mut Command) -> Option<String> {
    let out = command
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .ok()?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    if out.status.success() {
        return Some(text);
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
    None
}

/// The count before `passed`, from nextest's `Summary` line or libtest's `test result:`.
/// A conductor wants one number, not a transcript; the transcript only shows up on failure.
fn passed(output: &str) -> Option<String> {
    let line = output.lines().rev().find(|line| line.contains(" passed"))?;
    let tokens: Vec<&str> = line.split_whitespace().collect();
    tokens
        .windows(2)
        .find(|pair| pair.get(1).is_some_and(|t| t.starts_with("passed")))
        .and_then(|pair| pair.first().copied())
        .map(str::to_owned)
}

/// Did the command run and succeed? (Output discarded; this is a pre-flight, not a step.)
fn ok(command: &mut Command) -> bool {
    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}
