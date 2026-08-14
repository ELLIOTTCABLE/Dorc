//! Self-test for `.githooks/commit-msg`, ported from `test-commit-msg.sh`.
//!
//! The hook stays sh — git runs it that way, so testing anything else would test the
//! wrong artifact. Only the HARNESS moved to Rust, which is what lets it run from a
//! native Windows shell.
//!
//! Worth its keep because the gate is unusual on three counts: it is the only mechanical
//! enforcement of a convention agents actively fight, it is untyped sh, and its failure
//! direction is OPEN — a broken matcher stops refusing and says nothing.

use std::process::{Command, Stdio};

use internal_tooling::Posix;

/// One message put to the hook, and the verdict it must draw.
struct Case {
    name: &'static str,
    want_pass: bool,
    /// Session markers set for this case, on top of the neutralised baseline.
    env: &'static [(&'static str, &'static str)],
    message: &'static str,
    /// A throwaway repository to run the hook in, when the case is about what the hook READS.
    census: Option<Census>,
}

/// A fixture lock.s human-register counts at HEAD and staged. A real repository rather than a
/// stubbed `git`: the fail-OPEN half is the READ, and a mis-spelled path answers zero to zero.
struct Census {
    /// `None` puts the lock in the index alone — the absent-from-HEAD arm, counted as zero.
    head: Option<usize>,
    staged: usize,
}

/// An agent session, which is what makes the `AI` label mandatory.
const AGENT: &[(&str, &str)] = &[("CLAUDECODE", "1")];

/// The subtle cases (generated merge/revert exempted, the editor's `#` block stripped, a
/// HUMAN co-author left alone) are each here because getting one wrong is invisible in
/// review.
const CASES: &[Case] = &[
    Case {
        name: "accepts-a-labelled-ai-commit",
        want_pass: true,
        env: AGENT,
        message: "(AI fix) Move the index onto the hot path\n",
        census: None,
    },
    Case {
        name: "refuses-the-claude-coauthor-trailer",
        want_pass: false,
        env: AGENT,
        message: "(AI fix) Move it\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n",
        census: None,
    },
    // Isolates the IDENTITY matcher: a foreign-harness trailer the anthropic-footer
    // matcher does not also catch. Without it, sabotaging either matcher alone left the
    // suite green — found by falsifying this harness, 2026-07-26.
    Case {
        name: "refuses-a-foreign-ai-coauthor",
        want_pass: false,
        env: AGENT,
        message: "(AI fix) Move it\n\nCo-Authored-By: Codex <codex@example.com>\n",
        census: None,
    },
    Case {
        name: "refuses-the-session-trailer",
        want_pass: false,
        env: AGENT,
        message: "(AI fix) Move it\n\nClaude-Session: https://claude.ai/code/session_01\n",
        census: None,
    },
    Case {
        name: "refuses-the-generation-footer",
        want_pass: false,
        env: AGENT,
        message: "(AI new) Add the thing\n\nGenerated with [Claude Code](https://claude.com/claude-code)\n",
        census: None,
    },
    Case {
        name: "leaves-a-human-coauthor-alone",
        want_pass: true,
        env: AGENT,
        message: "(AI fix) Move it\n\nCo-Authored-By: Jane Doe <jane@example.com>\n",
        census: None,
    },
    Case {
        name: "refuses-a-subject-with-no-labels",
        want_pass: false,
        env: AGENT,
        message: "Fix the thing\n",
        census: None,
    },
    Case {
        name: "exempts-a-generated-merge",
        want_pass: true,
        env: AGENT,
        message: "Merge branch 'ai/r28-unify' into ai/main\n",
        census: None,
    },
    Case {
        name: "exempts-a-generated-revert",
        want_pass: true,
        env: AGENT,
        message: "Revert \"(AI test) Prove the transcript-driven loop\"\n",
        census: None,
    },
    Case {
        name: "refuses-an-agent-commit-without-ai",
        want_pass: false,
        env: AGENT,
        message: "(fix) Move the index onto the hot path\n",
        census: None,
    },
    Case {
        name: "honours-the-human-escape-hatch",
        want_pass: true,
        env: &[("CLAUDECODE", "1"), ("DORC_HUMAN_COMMIT", "1")],
        message: "(fix) Move the index onto the hot path\n",
        census: None,
    },
    Case {
        name: "leaves-a-non-agent-commit-alone",
        want_pass: true,
        env: &[],
        message: "(fix) Move the index onto the hot path\n",
        census: None,
    },
    Case {
        name: "warns-but-admits-an-unknown-label",
        want_pass: true,
        env: AGENT,
        message: "(AI fix loom) Reword the catalog register\n",
        census: None,
    },
    Case {
        name: "accepts-the-purpose-labels",
        want_pass: true,
        env: AGENT,
        message: "(AI fix aid cli) Reword a register and the usage line\n",
        census: None,
    },
    Case {
        name: "strips-the-editor-comment-block",
        want_pass: true,
        env: AGENT,
        message: "(AI doc) Explain the seam\n\n# Please enter the commit message for your changes.\n# Lines starting with '#' will be ignored.\n",
        census: None,
    },
    // The prose ratchet, both directions. Growth is the only refused shape, and it is keyed on the
    // LABEL rather than the session — which is what the last case says.
    Case {
        name: "refuses-a-growing-human-census",
        want_pass: false,
        env: AGENT,
        message: "(AI aid) Reword a register\n",
        census: Some(Census {
            head: Some(1),
            staged: 2,
        }),
    },
    Case {
        name: "refuses-a-first-human-register",
        want_pass: false,
        env: AGENT,
        message: "(AI aid) Reword a register\n",
        census: Some(Census {
            head: None,
            staged: 1,
        }),
    },
    Case {
        name: "admits-a-steady-human-census",
        want_pass: true,
        env: AGENT,
        message: "(AI aid) Reword a register\n",
        census: Some(Census {
            head: Some(2),
            staged: 2,
        }),
    },
    Case {
        name: "admits-a-shrinking-human-census",
        want_pass: true,
        env: AGENT,
        message: "(AI aid) Rework a register the loom re-marked\n",
        census: Some(Census {
            head: Some(2),
            staged: 1,
        }),
    },
    Case {
        name: "leaves-a-human-labelled-commit-alone",
        want_pass: true,
        env: &[],
        message: "(new aid) Write the register\n",
        census: Some(Census {
            head: Some(1),
            staged: 2,
        }),
    },
];

/// The lock the ratchet reads, spelled exactly as the hook spells it.
const LOCK: &str = "spike/crates/aid/src/catalog_lock.rs";

/// A throwaway repository carrying `census`.s counts. Unsigned and unhooked deliberately: a
/// fixture commit waiting on a passphrase would hang this gate on somebody else.s configuration.
fn fixture_repo(census: &Census, at: &std::path::Path) -> Result<(), String> {
    let git = |args: &[&str]| -> Result<(), String> {
        let status = Command::new("git")
            .args([
                "-c",
                "user.name=dorc-selftest",
                "-c",
                "user.email=selftest@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=no-such-hooks",
            ])
            .args(args)
            .current_dir(at)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| format!("could not run git: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("git {args:?} failed in {}", at.display()))
        }
    };
    let lock = at.join(LOCK);
    let write = |count: usize| -> Result<(), String> {
        let rows =
            "        message: Some(ProseTier::WrittenByHumanOnly(\"words\")),\n".repeat(count);
        std::fs::create_dir_all(lock.parent().unwrap_or(at))
            .and_then(|()| std::fs::write(&lock, format!("// @generated\n{rows}")))
            .map_err(|e| format!("cannot write {}: {e}", lock.display()))
    };

    git(&["init", "-q"])?;
    match census.head {
        Some(count) => write(count)?,
        // HEAD must exist for `HEAD:<lock>` to be the ABSENT-blob case rather than a broken repo.
        None => std::fs::write(at.join("seed"), "seed\n")
            .map_err(|e| format!("cannot seed {}: {e}", at.display()))?,
    }
    git(&["add", "-A"])?;
    git(&["commit", "-q", "-m", "seed"])?;
    write(census.staged)?;
    git(&["add", "-A"])
}

/// The session markers the hook gates on, neutralised before each case so an inherited
/// value cannot decide the verdict. The sh version leaked these from its caller.
const NEUTRALISED: &[&str] = &["CLAUDECODE", "CLAUDE_CODE_ENTRYPOINT", "DORC_HUMAN_COMMIT"];

/// Put one message to the hook and report whether it was accepted.
fn run_case(
    case: &Case,
    shell: &std::path::Path,
    hook: &std::path::Path,
    path: &std::ffi::OsStr,
    msg_file: &std::path::Path,
) -> Result<bool, String> {
    std::fs::write(msg_file, case.message)
        .map_err(|e| format!("cannot write {}: {e}", msg_file.display()))?;
    let mut child = Command::new(shell);
    child
        .arg(hook)
        .arg(msg_file)
        .env("PATH", path)
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    for key in NEUTRALISED {
        child.env(key, "");
    }
    for (key, value) in case.env {
        child.env(key, value);
    }
    // The hook reads the repository it is invoked IN, which is the whole point of a fixture one.
    let fixture = case.census.as_ref().map(|census| {
        let at = std::env::temp_dir().join(format!(
            "dorc-hook-census-{}-{}",
            std::process::id(),
            case.name
        ));
        let _ = std::fs::remove_dir_all(&at);
        std::fs::create_dir_all(&at)
            .map_err(|e| format!("cannot make {}: {e}", at.display()))
            .and_then(|()| fixture_repo(census, &at))
            .map(|()| at)
    });
    let status = match fixture.transpose()? {
        Some(at) => {
            let status = child.current_dir(&at).status();
            let _ = std::fs::remove_dir_all(&at);
            status
        }
        None => child.status(),
    }
    .map_err(|e| format!("could not run {}: {e}", shell.display()));
    let _ = std::fs::remove_file(msg_file);
    status.map(|status| status.success())
}

pub(crate) fn run() -> u8 {
    let posix = match Posix::find() {
        Ok(posix) => posix,
        Err(why) => {
            eprintln!("hook-selftest: no POSIX shell — {why}");
            return 2;
        }
    };
    let hook = internal_tooling::repo_root().join(".githooks/commit-msg");
    let msg_file = std::env::temp_dir().join(format!("dorc-hook-selftest-{}", std::process::id()));

    // ~2.4s for 15 shell spawns, which is why this rides `mise run gate` and NOT the
    // pre-commit hook. Left sequential deliberately: concurrency bought 1.6x here (msys
    // shell startup dominates, not scheduling) and no amount of it would make this a
    // pre-commit-shaped check.
    let path = posix.child_path();
    let mut failures = 0_u32;
    for case in CASES {
        let name = case.name;
        match run_case(case, &posix.shell, &hook, &path, &msg_file) {
            Err(why) => {
                eprintln!("hook-selftest: {why}");
                return 2;
            }
            Ok(passed) if passed == case.want_pass => println!("ok   {name}"),
            Ok(passed) => {
                let verdict = |ok: bool| if ok { "pass" } else { "fail" };
                let (want, got) = (verdict(case.want_pass), verdict(passed));
                println!("FAIL {name} (want {want}, got {got})");
                failures = failures.saturating_add(1);
            }
        }
    }

    if failures == 0 {
        println!(
            "commit-msg hook: all cases green (via {})",
            posix.shell.display()
        );
        0
    } else {
        eprintln!("{failures} case(s) failed");
        1
    }
}
