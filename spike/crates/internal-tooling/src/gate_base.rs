//! `gate-base-is-determinable` — refuse a gate that cannot say what it is about to check.
//!
//! `gate:full*` open with three selections (`--pr`, `--staged`, `--unstaged`), and hk exits 0
//! when a selection is empty — so a gate that selected nothing is byte-identical, at the exit
//! code, to a gate that passed. `--pr` is `--from-ref <default_branch> --to-ref HEAD`, and
//! measured 2026-08-24 hk answers an unanswerable base by degrading in two OPPOSITE directions,
//! both rc 0: an unresolvable ref widens to every file in the repo, and an absent
//! `default_branch` guesses (`origin/HEAD`, then `main`) and reports on whatever the guess
//! selected. This runs first and refuses instead.
//!
//! An EMPTY branch diff over a base that DOES resolve is legitimate — `ai/main` after a fold is
//! exactly that — and passes. It says so on the pass line, because that state is also how a lane
//! reports green having checked nothing, and only the person running it knows which it was.

use std::process::{Command, ExitCode};

const TAG: &str = "gate base";

/// What could be learned about the base `hk check --pr` will diff against.
#[derive(Debug)]
struct Facts {
    /// hk's own answer; `None` when it has no `default_branch` to give.
    configured: Option<String>,
    /// The commit that ref names HERE; `None` when it names none.
    base: Option<String>,
    /// HEAD's commit; `None` on an unborn branch or outside a worktree.
    head: Option<String>,
    /// HEAD's branch; `None` when detached — which `--pr` handles fine, so it is reported, not refused.
    branch: Option<String>,
    /// Commits on HEAD's side of the fork — hk selects exactly these. `None` when the two share
    /// no history, and therefore no fork to count from.
    ahead_of_fork: Option<u64>,
}

/// Answer the precondition, printing one line either way.
pub(crate) fn run() -> ExitCode {
    match verdict(&probe()) {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(line) => {
            println!("{line}");
            ExitCode::FAILURE
        }
    }
}

/// Rule on the facts. Pure, so the refusal table is testable without a repository.
fn verdict(b: &Facts) -> Result<String, String> {
    let Some(head) = b.head.as_deref() else {
        return Err(format!(
            "{TAG}: REFUSED — git cannot name HEAD's commit (unborn branch, or not a git \
             worktree). Commit something, or run the gate from inside a worktree."
        ));
    };
    let Some(configured) = b.configured.as_deref() else {
        return Err(format!(
            "{TAG}: REFUSED — hk has no default_branch, so `--pr` would guess one \
             (origin/HEAD, then `main`) and report on whatever that guess selected. Set \
             `default_branch` in hk.pkl."
        ));
    };
    let Some(base) = b.base.as_deref() else {
        return Err(format!(
            "{TAG}: REFUSED — default_branch {configured:?} names no commit here, and `--pr` \
             answers that by silently widening to every file in the repo. Fetch or create that \
             branch, or correct `default_branch` in hk.pkl."
        ));
    };
    let Some(ahead) = b.ahead_of_fork else {
        return Err(format!(
            "{TAG}: REFUSED — cannot measure {configured:?} against HEAD; they share no history. \
             Rebase this branch onto {configured} (`git rebase --onto {configured}`), or correct \
             `default_branch` in hk.pkl."
        ));
    };

    let here = b.branch.as_deref().unwrap_or("DETACHED HEAD");
    // The one state worth naming on a PASS: legitimate on the default branch after a fold,
    // and indistinguishable from a lane whose work never reached its branch.
    let note = if ahead == 0 {
        " — branch diff EMPTY, so only staged and unstaged changes are being checked"
    } else {
        ""
    };
    Ok(format!(
        "{TAG}: ok — {configured} ({base}) -> {here} ({head}), {ahead} ahead{note}"
    ))
}

/// Gather the facts, asking each question of whoever owns the answer.
fn probe() -> Facts {
    let configured = hk_default_branch();
    let base = configured.as_deref().and_then(commit_of);
    let head = commit_of("HEAD");
    let ahead_of_fork = base
        .as_deref()
        .and_then(|b| git(&["merge-base", b, "HEAD"]))
        .and_then(|fork| git(&["rev-list", "--count", &format!("{fork}..HEAD")]))
        .and_then(|count| count.parse().ok());
    Facts {
        configured,
        base,
        head,
        branch: git(&["symbolic-ref", "--quiet", "--short", "HEAD"]),
        ahead_of_fork,
    }
}

/// hk's own answer, never a re-parse of `hk.pkl`: git config outranks the project file in hk's
/// precedence chain, so a second reader can disagree with the tool that will actually diff.
fn hk_default_branch() -> Option<String> {
    let out = Command::new("hk")
        .args(["config", "get", "default_branch"])
        .output()
        .ok()?;
    let raw = String::from_utf8_lossy(&out.stdout);
    let value = raw.trim().trim_matches('"');
    (out.status.success() && !value.is_empty() && value != "null").then(|| value.to_owned())
}

/// The short commit a ref names here, or `None` if it names none.
fn commit_of(reference: &str) -> Option<String> {
    git(&[
        "rev-parse",
        "--verify",
        "--short",
        &format!("{reference}^{{commit}}"),
    ])
}

/// Trimmed stdout of a successful git command, `None` otherwise.
///
/// Inherits the caller's cwd deliberately: the gate runs from the worktree that invoked it, while
/// a compile-time root would name whichever worktree last built this binary.
fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::{Facts, verdict};

    fn determinable() -> Facts {
        Facts {
            configured: Some("ai/main".to_owned()),
            base: Some("a2d2a3e2".to_owned()),
            head: Some("beef1234".to_owned()),
            branch: Some("ai/r30-gate-floor".to_owned()),
            ahead_of_fork: Some(3),
        }
    }

    /// Either arm's line, so a test can assert the verdict and the wording separately.
    fn line_of(verdict: &Result<String, String>) -> &str {
        match verdict {
            Ok(line) | Err(line) => line,
        }
    }

    #[test]
    fn a_determinable_base_passes_and_names_what_it_will_check() {
        let got = verdict(&determinable());
        assert!(got.is_ok(), "a resolvable base is not a refusal");
        let line = line_of(&got);
        assert!(line.contains("ok — ai/main (a2d2a3e2) -> ai/r30-gate-floor (beef1234), 3 ahead"));
        assert!(
            !line.contains("EMPTY"),
            "a branch with commits ahead has a diff to check"
        );
    }

    #[test]
    fn an_empty_branch_diff_passes_but_says_so() {
        let got = verdict(&Facts {
            ahead_of_fork: Some(0),
            ..determinable()
        });
        assert!(
            got.is_ok(),
            "an empty diff over a resolvable base is legitimate — `ai/main` after a fold"
        );
        assert!(line_of(&got).contains("0 ahead — branch diff EMPTY"));
    }

    #[test]
    fn a_detached_head_is_reported_rather_than_refused() {
        let got = verdict(&Facts {
            branch: None,
            ..determinable()
        });
        assert!(
            got.is_ok(),
            "`--pr` resolves HEAD detached exactly as it does on a branch"
        );
        assert!(line_of(&got).contains("DETACHED HEAD"));
    }

    #[test]
    fn every_indeterminable_base_refuses_with_its_remedy() {
        // The four ways the gate cannot say what it is checking, each named by what hk would
        // otherwise do about it.
        let cases = [
            (
                Facts {
                    head: None,
                    ..determinable()
                },
                "not a git worktree",
            ),
            (
                Facts {
                    configured: None,
                    ..determinable()
                },
                "Set `default_branch` in hk.pkl",
            ),
            (
                Facts {
                    base: None,
                    ..determinable()
                },
                "widening to every file in the repo",
            ),
            (
                Facts {
                    ahead_of_fork: None,
                    ..determinable()
                },
                "share no history",
            ),
        ];
        for (base, remedy) in cases {
            let got = verdict(&base);
            assert!(got.is_err(), "an indeterminable base must refuse: {remedy}");
            let line = line_of(&got);
            assert!(line.contains("REFUSED"), "{line}");
            assert!(line.contains(remedy), "{line}");
        }
    }
}
