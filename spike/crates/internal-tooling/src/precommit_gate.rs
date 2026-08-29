//! Self-test for `.githooks/pre-commit` — the gate, and the one sanctioned way past a red one.
//!
//! Two batteries, because the ack can rot in two opposite directions. SHIM drives the hook against
//! a stubbed `mise` and reads the only thing git reads: the exit code. PARTITION asks the real hk
//! which steps `--profile commit-floor` switches off, because the shim's whole safety story is
//! that the excused set is exactly the steps which compile product code — and that tag lives over
//! in `hk.pkl`, where nothing else would notice it drifting.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use internal_tooling::Posix;

/// One run of the hook: what the environment says, and what git must be told.
struct Shim {
    name: &'static str,
    /// `DORC_KNOWN_BROKEN`. Empty is no ack.
    ack: &'static str,
    /// What the stub returns for the floor pass, and for every other pass.
    floor_rc: &'static str,
    full_rc: &'static str,
    /// Whether git is told to make the commit.
    want_commit: bool,
    /// hk invocations. Only an ack that is actually needed may pay for a second one.
    want_runs: usize,
    want_banner: bool,
}

const SHIMS: &[Shim] = &[
    Shim {
        name: "commits-when-the-gate-is-green",
        ack: "",
        floor_rc: "0",
        full_rc: "0",
        want_commit: true,
        want_runs: 1,
        want_banner: false,
    },
    Shim {
        name: "refuses-a-red-gate-with-no-ack",
        ack: "",
        floor_rc: "0",
        full_rc: "1",
        want_commit: false,
        want_runs: 1,
        want_banner: false,
    },
    Shim {
        name: "waves-through-a-red-corpus-on-the-ack",
        ack: "mid-refactor",
        floor_rc: "0",
        full_rc: "1",
        want_commit: true,
        want_runs: 2,
        want_banner: true,
    },
    // The half that makes the ack a partition rather than a bypass.
    Shim {
        name: "still-refuses-a-red-floor-on-the-ack",
        ack: "mid-refactor",
        floor_rc: "1",
        full_rc: "0",
        want_commit: false,
        want_runs: 1,
        want_banner: false,
    },
    Shim {
        name: "says-nothing-when-the-ack-was-not-needed",
        ack: "mid-refactor",
        floor_rc: "0",
        full_rc: "0",
        want_commit: true,
        want_runs: 2,
        want_banner: false,
    },
];

/// A stand-in for `mise`, which is how the hook reaches hk. It records the argv it was handed and
/// answers with the exit code the case asked for, keyed on the flag that names the floor pass.
const STUB: &str = r#"#!/bin/sh
echo "$*" >> "$DORC_STUB_LOG"
case " $* " in
   *" --profile commit-floor "*) exit "$DORC_STUB_FLOOR_RC" ;;
esac
exit "$DORC_STUB_FULL_RC"
"#;

/// Give the stub the bit unix `PATH` lookup demands.
///
/// Without it every shim case reads "ran hk 0 times": the hook's `mise` resolves to nothing, so
/// git's own answer — the exit code — is 127 and no case can distinguish a refusal it asked for
/// from a stub that was never reachable. Windows never calls this; the bit is not consulted there,
/// and a stand-in that always answered `Ok` would be a second, lying seat.
#[cfg(unix)]
fn make_executable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(perms.mode() | 0o111);
    std::fs::set_permissions(path, perms)
}

/// The phrase the hook prints when, and only when, it forgives a failure.
const BANNER: &str = "waved it through";

struct Outcome {
    commit: bool,
    /// One line per hk invocation, in order.
    runs: Vec<String>,
    stderr: String,
}

/// The stub's directory, ahead of the shell's own userland.
///
/// Joined by the platform's rule rather than spliced with a separator: on Windows a `:` splits a
/// drive letter and a `;` is not a separator at all, and either mistake silently resolves the REAL
/// `mise` — which would point this battery at the developer's own worktree.
fn stub_path(dir: &Path, posix: &Posix) -> OsString {
    let base = posix.child_path();
    std::env::join_paths(std::iter::once(PathBuf::from(dir)).chain(std::env::split_paths(&base)))
        .unwrap_or(base)
}

fn run_shim(case: &Shim, posix: &Posix, hook: &Path, dir: &Path) -> Result<Outcome, String> {
    let log = dir.join("runs");
    let _ = std::fs::remove_file(&log);
    let out = Command::new(&posix.shell)
        .arg(hook)
        .current_dir(internal_tooling::repo_root())
        .env("PATH", stub_path(dir, posix))
        .env("DORC_KNOWN_BROKEN", case.ack)
        .env("DORC_STUB_LOG", &log)
        .env("DORC_STUB_FLOOR_RC", case.floor_rc)
        .env("DORC_STUB_FULL_RC", case.full_rc)
        // Pins the hook's own dispatch branch so the recorded argv is the same on every machine.
        .env("HK_FIX", "0")
        // Belt and braces for the stub-was-missed case above: a real hk reached from here finds no
        // config and refuses, rather than running the real gate over a real tree.
        .env("HK_FILE", dir.join("no-such-hk.pkl"))
        .output()
        .map_err(|why| format!("could not run {}: {why}", posix.shell.display()))?;
    Ok(Outcome {
        commit: out.status.success(),
        runs: std::fs::read_to_string(&log)
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    })
}

/// What the two passes of an ack'd run must be, beyond how many there are.
///
/// The floor pass runs the fixers, against the tree hk stashed for it; the second pass is
/// read-only because its verdict is about to be discarded, and a pass that both writes and is
/// forgiven is how a failed stash-restore would go unnoticed.
fn ack_pass_shapes(runs: &[String]) -> Result<(), String> {
    let [floor, full] = runs else {
        return Ok(());
    };
    if !floor.contains("--profile commit-floor") {
        return Err(format!("first pass is not the floor: {floor}"));
    }
    if floor.contains("--check") {
        return Err(format!("floor pass would not fix: {floor}"));
    }
    if !full.contains("--check") || !full.contains("--stash none") {
        return Err(format!("forgiven pass is not read-only: {full}"));
    }
    Ok(())
}

/// A pre-commit step, and whether `--profile commit-floor` must switch it off.
struct Partition {
    step: &'static str,
    excused: bool,
}

const PARTITION: &[Partition] = &[
    Partition {
        step: "loom-hygiene",
        excused: true,
    },
    Partition {
        step: "minispec",
        excused: true,
    },
    Partition {
        step: "e2e",
        excused: true,
    },
    Partition {
        step: "docids",
        excused: false,
    },
    Partition {
        step: "typos",
        excused: false,
    },
];

/// One path per step in `PARTITION`, so every row is answered by a step that actually SELECTED
/// something. Without that, a glob which stopped reaching its files would report "no files
/// matched" and satisfy this battery by accident — the silent shape `step_globs` exists to refuse.
const PARTITION_PATHS: &[&str] = &[
    "spike/crates/cli/tests/contest28-cross-unit-shadow-runs.loom",
    "minispec/Generated.lean",
    "README.md",
];

const DISABLED: &str = "disabled by active profile";

fn plan(hk: &Path, profile: Option<&str>) -> Result<String, String> {
    let mut cmd = Command::new(hk);
    cmd.current_dir(internal_tooling::repo_root())
        .args(["run", "pre-commit", "--plan"]);
    if let Some(profile) = profile {
        cmd.args(["--profile", profile]);
    }
    let out = cmd
        .args(PARTITION_PATHS)
        .env("HK_SKIP_HOOK", "")
        .stderr(Stdio::null())
        .output()
        .map_err(|why| format!("could not run {}: {why}", hk.display()))?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// The plan's line for `step`, e.g. `    ✓ docids  (1 file matched)`.
fn step_line<'a>(plan: &'a str, step: &str) -> Option<&'a str> {
    plan.lines().find(|line| {
        line.split_whitespace()
            .nth(1)
            .is_some_and(|word| word == step)
    })
}

fn selected(line: &str) -> bool {
    line.contains("matched") && !line.contains("no files matched")
}

fn partition(hk: &Path) -> Result<u32, String> {
    let plain = plan(hk, None)?;
    let floor = plan(hk, Some("commit-floor"))?;
    let mut failures = 0_u32;
    for case in PARTITION {
        let step = case.step;
        let (Some(plain), Some(floor)) = (step_line(&plain, step), step_line(&floor, step)) else {
            return Err(format!(
                "no plan line for {step} — has the step been renamed?"
            ));
        };
        if !selected(plain) {
            return Err(format!("{step} selected nothing: {plain}"));
        }
        let excused = floor.contains(DISABLED);
        if excused == case.excused {
            println!("ok   commit-floor-{}-{step}", verdict(case.excused));
        } else {
            let want = verdict(case.excused);
            println!("FAIL commit-floor-{want}-{step} (got: {})", floor.trim());
            failures = failures.saturating_add(1);
        }
    }
    Ok(failures)
}

fn verdict(excused: bool) -> &'static str {
    if excused { "excuses" } else { "keeps" }
}

pub(crate) fn run() -> u8 {
    let posix = match Posix::find() {
        Ok(posix) => posix,
        Err(why) => {
            eprintln!("precommit-gate: no POSIX shell — {why}");
            return 2;
        }
    };
    let Some(hk) = internal_tooling::which("hk") else {
        eprintln!("precommit-gate: hk is not on PATH — the pinned one comes from mise.toml");
        return 2;
    };
    let hook = internal_tooling::repo_root().join(".githooks/pre-commit");
    let dir = std::env::temp_dir().join(format!("dorc-precommit-gate-{}", std::process::id()));
    if let Err(why) = std::fs::create_dir_all(&dir)
        .and_then(|()| std::fs::write(dir.join("mise"), STUB))
        .map_err(|why| format!("cannot lay out {}: {why}", dir.display()))
    {
        eprintln!("precommit-gate: {why}");
        return 2;
    }
    // Loud HERE, because the alternative is not silence — it is five cases reporting
    // "ran hk 0 time(s)", which reads as a counting failure and is a reachability one.
    #[cfg(unix)]
    if let Err(why) = make_executable(&dir.join("mise")) {
        eprintln!(
            "precommit-gate: cannot make the stub executable at {}: {why}",
            dir.join("mise").display()
        );
        let _ = std::fs::remove_dir_all(&dir);
        return 2;
    }

    let mut failures = 0_u32;
    for case in SHIMS {
        let name = case.name;
        match run_shim(case, &posix, &hook, &dir) {
            Err(why) => {
                let _ = std::fs::remove_dir_all(&dir);
                eprintln!("precommit-gate: {why}");
                return 2;
            }
            Ok(outcome) => {
                let mut wrong = vec![];
                if outcome.commit != case.want_commit {
                    let told = |ok: bool| if ok { "commit" } else { "abort" };
                    wrong.push(format!(
                        "told git to {}, wanted {}",
                        told(outcome.commit),
                        told(case.want_commit)
                    ));
                }
                if outcome.runs.len() != case.want_runs {
                    wrong.push(format!(
                        "ran hk {} time(s), wanted {}",
                        outcome.runs.len(),
                        case.want_runs
                    ));
                }
                if outcome.stderr.contains(BANNER) != case.want_banner {
                    wrong.push(if case.want_banner {
                        "said nothing about forgiving the failure".to_owned()
                    } else {
                        "announced a wave-through it did not do".to_owned()
                    });
                }
                if let Err(why) = ack_pass_shapes(&outcome.runs) {
                    wrong.push(why);
                }
                if wrong.is_empty() {
                    println!("ok   {name}");
                } else {
                    println!("FAIL {name} ({})", wrong.join("; "));
                    failures = failures.saturating_add(1);
                }
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);

    match partition(&hk) {
        Err(why) => {
            eprintln!("precommit-gate: {why}");
            return 2;
        }
        Ok(count) => failures = failures.saturating_add(count),
    }

    if failures == 0 {
        println!("pre-commit hook: the ack forgives the corpora and nothing else");
        0
    } else {
        eprintln!("{failures} case(s) failed");
        1
    }
}
