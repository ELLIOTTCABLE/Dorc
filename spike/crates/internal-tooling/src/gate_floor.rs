//! `gate-refuses-a-vacuous-run` — the completion gate's DISCOVERY FLOOR, for the reason each
//! corpus runner carries one: hk exits 0 on an empty selection, so a gate that asked nothing and
//! a gate that passed share an exit code, and a lane reads the second.
//!
//! Two vacuous runs were measured on one branch in r30, with DIFFERENT causes, which is why this
//! asserts the OBSERVABLE — that work was actually selected — rather than any one cause:
//!
//! - the base `--pr` diffs against could not be determined. hk degrades silently in BOTH
//!   directions and rc 0 either way: an unresolvable ref widens to EVERY file in the repo, and an
//!   absent `default_branch` guesses `origin/HEAD`, then `main`.
//! - the base resolved, git reported 114 changed files, and hk's own selection still chose
//!   nothing — on one platform leg only, at the same commit as a leg that selected correctly.
//!
//! So git and hk are asked the same question independently and their answers compared. When hk
//! selects nothing, hk is asked a second time whether it covers the paths git named: hk is its own
//! coverage oracle, so this never re-states its globs, and "nothing hk cares about changed" stays
//! quiet while "hk covers these and still picked none" refuses.
//!
//! What it does NOT assert is EXECUTION. It predicts, from hk's own plan, moments before the gate
//! runs; a selection that is then not executed is invisible from in front. Closing that needs a
//! post-gate step reading hk state, which is a materially bigger mechanism than this one.

use std::process::{Command, ExitCode};

const TAG: &str = "gate floor";

/// Paths handed to hk in the coverage question. Only EXISTENCE of a covered path matters, and a
/// Windows command line is bounded, so a very large diff is sampled rather than passed whole.
const COVERAGE_SAMPLE: usize = 400;

/// What the floor could learn, from whichever tool owns each answer.
#[derive(Debug)]
struct Facts {
    /// hk's own `default_branch`; `None` when it has none to give.
    configured: Option<String>,
    /// The commit that ref names HERE; `None` when it names none.
    base: Option<String>,
    /// HEAD's commit; `None` on an unborn branch or outside a worktree.
    head: Option<String>,
    /// HEAD's branch; `None` when detached — which `--pr` handles fine, so it is reported.
    branch: Option<String>,
    /// Commits on HEAD's side of the fork — what `--pr` selects over. `None` when the two share
    /// no history, and so have no fork to count from.
    ahead_of_fork: Option<u64>,
    /// Steps hk's OWN selection includes across the gate's three questions; `None` when hk could
    /// not be asked at all.
    selected: Option<u32>,
    /// Paths git reports for those same three questions.
    changed: Vec<String>,
    /// Steps hk includes when HANDED those paths — hk answering whether it covers them.
    covering: Option<u32>,
}

/// Answer the precondition, printing one line either way.
pub(crate) fn run(profiles: &[String]) -> ExitCode {
    match verdict(&probe(profiles)) {
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
fn verdict(f: &Facts) -> Result<String, String> {
    let Some(head) = f.head.as_deref() else {
        return Err(format!(
            "{TAG}: REFUSED — git cannot name HEAD's commit (unborn branch, or not a git \
             worktree). Commit something, or run the gate from inside a worktree."
        ));
    };
    let Some(configured) = f.configured.as_deref() else {
        return Err(format!(
            "{TAG}: REFUSED — hk has no default_branch, so `--pr` would guess one \
             (origin/HEAD, then `main`) and report on whatever that guess selected. Set \
             `default_branch` in hk.pkl."
        ));
    };
    let Some(base) = f.base.as_deref() else {
        return Err(format!(
            "{TAG}: REFUSED — default_branch {configured:?} names no commit here, and `--pr` \
             answers that by silently widening to every file in the repo. Fetch or create that \
             branch, or correct `default_branch` in hk.pkl."
        ));
    };
    let Some(ahead) = f.ahead_of_fork else {
        return Err(format!(
            "{TAG}: REFUSED — cannot measure {configured:?} against HEAD; they share no history. \
             Rebase this branch onto {configured} (`git rebase --onto {configured}`), or correct \
             `default_branch` in hk.pkl."
        ));
    };
    let (Some(selected), Some(covering)) = (f.selected, f.covering) else {
        return Err(format!(
            "{TAG}: REFUSED — hk would not say what it is about to select \
             (`hk check --plan -J` failed). The gate's result cannot be trusted until it does; \
             run `hk check --pr --plan` and read the error."
        ));
    };

    let where_ = f.branch.as_deref().unwrap_or("DETACHED HEAD");
    let head_line = format!("{configured} ({base}) -> {where_} ({head}), {ahead} ahead");
    let changed = f.changed.len();

    if selected == 0 && covering > 0 {
        return Err(format!(
            "{TAG}: REFUSED — {head_line}; hk selected NO check, yet git reports {changed} \
             changed file(s) that hk matches {covering} check(s) against when handed them \
             directly. The gate would report success having run nothing. Inspect with \
             `hk check --pr --plan`; treat no result from this checkout as meaningful until it \
             selects work."
        ));
    }
    // Both quiet cases are genuine, and they are different: nothing changed at all (`ai/main`
    // after a fold), versus changes no step in the graph covers.
    let note = match (selected, changed) {
        (0, 0) => " — nothing changed against the base and the tree is clean; no check is due",
        (0, _) => " — no check in the graph covers what changed",
        _ => "",
    };
    Ok(format!(
        "{TAG}: ok — {head_line}; {changed} changed file(s), {selected} check(s) selected{note}"
    ))
}

/// Gather the facts, asking each question of whoever owns the answer.
///
/// git answers first and hk is asked only once it confirms a worktree: `hk config get` HANGS
/// forever outside one (measured, hk 1.53.0). Inside one this adds no hang the gate did not
/// already carry — its own `hk check` follows immediately.
fn probe(profiles: &[String]) -> Facts {
    let head = commit_of("HEAD");
    let configured = head.as_ref().and_then(|_| hk_default_branch());
    let base = configured.as_deref().and_then(commit_of);
    let ahead_of_fork = base
        .as_deref()
        .and_then(|b| git(&["merge-base", b, "HEAD"]))
        .and_then(|fork| git(&["rev-list", "--count", &format!("{fork}..HEAD")]))
        .and_then(|count| count.parse().ok());

    let (selected, changed, covering) = match base.as_deref() {
        None => (None, Vec::new(), None),
        Some(base) => {
            let selected = ["--pr", "--staged", "--unstaged"]
                .into_iter()
                .try_fold(0_u32, |sum, flag| {
                    plan_included(profiles, &[flag]).map(|n| sum.saturating_add(n))
                });
            let changed = changed_paths(base);
            // Only asked when it can change the verdict; it costs another hk evaluation.
            let covering = match selected {
                Some(0) if !changed.is_empty() => {
                    let sample: Vec<&str> = changed
                        .iter()
                        .take(COVERAGE_SAMPLE)
                        .map(String::as_str)
                        .collect();
                    plan_included(profiles, &sample)
                }
                _ => Some(0),
            };
            (selected, changed, covering)
        }
    };

    Facts {
        configured,
        base,
        head,
        branch: git(&["symbolic-ref", "--quiet", "--short", "HEAD"]),
        ahead_of_fork,
        selected,
        changed,
        covering,
    }
}

/// How many steps hk's plan marks included, or `None` when hk would not answer.
///
/// `selection` is either one selection flag or a list of paths — hk narrows each step's globs to
/// the paths it is handed, which is what lets it answer "do you cover these?" about itself.
fn plan_included(profiles: &[String], selection: &[&str]) -> Option<u32> {
    let mut args = vec!["check", "--plan", "-J"];
    args.extend(profiles.iter().map(String::as_str));
    args.extend_from_slice(selection);
    let out = Command::new("hk").args(&args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    // Whitespace-collapsed first, so a pretty-printer change reads as zero and REFUSES loudly
    // rather than matching nothing in silence.
    let plan: String = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .collect();
    u32::try_from(plan.matches(r#""status":"included""#).count()).ok()
}

/// Every path git reports for the three questions the gate asks, deduplicated.
fn changed_paths(base: &str) -> Vec<String> {
    let range = format!("{base}...HEAD");
    let questions: [&[&str]; 4] = [
        &["diff", "--name-only", &range],
        &["diff", "--name-only", "--cached"],
        &["diff", "--name-only"],
        &["ls-files", "--others", "--exclude-standard"],
    ];
    let mut paths: Vec<String> = questions
        .iter()
        .filter_map(|question| {
            // Unquoted, so a non-ASCII path reaches hk as the bytes hk would match on.
            let mut args = vec!["-c", "core.quotePath=false"];
            args.extend_from_slice(question);
            git(&args)
        })
        .flat_map(|out| {
            out.lines()
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect();
    paths.sort_unstable();
    paths.dedup();
    paths
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

    fn selecting() -> Facts {
        Facts {
            configured: Some("ai/main".to_owned()),
            base: Some("a2d2a3e2".to_owned()),
            head: Some("beef1234".to_owned()),
            branch: Some("ai/r30-gate-floor".to_owned()),
            ahead_of_fork: Some(3),
            selected: Some(6),
            changed: vec!["spike/crates/core/src/lib.rs".to_owned()],
            covering: Some(0),
        }
    }

    /// Either arm's line, so a test can assert the verdict and the wording separately.
    fn line_of(verdict: &Result<String, String>) -> &str {
        match verdict {
            Ok(line) | Err(line) => line,
        }
    }

    #[test]
    fn a_run_that_selects_work_passes_and_says_how_much() {
        let got = verdict(&selecting());
        assert!(got.is_ok(), "a gate about to run checks is not a refusal");
        let line = line_of(&got);
        assert!(line.contains("ai/main (a2d2a3e2) -> ai/r30-gate-floor (beef1234), 3 ahead"));
        assert!(line.contains("1 changed file(s), 6 check(s) selected"));
    }

    /// The measured WSL vacuity: base fine, diff large, hk's own selection empty anyway.
    #[test]
    fn selecting_nothing_over_covered_changes_refuses() {
        let got = verdict(&Facts {
            selected: Some(0),
            changed: (0..114).map(|n| format!("spike/f{n}.rs")).collect(),
            covering: Some(9),
            ..selecting()
        });
        assert!(got.is_err(), "a gate that will run nothing must not pass");
        let line = line_of(&got);
        assert!(line.contains("REFUSED"), "{line}");
        assert!(line.contains("114 changed file(s)"), "{line}");
        assert!(line.contains("9 check(s)"), "{line}");
    }

    #[test]
    fn an_empty_clean_tree_at_the_base_passes_quietly() {
        let got = verdict(&Facts {
            ahead_of_fork: Some(0),
            selected: Some(0),
            changed: Vec::new(),
            covering: Some(0),
            ..selecting()
        });
        assert!(
            got.is_ok(),
            "`ai/main` after a fold legitimately has nothing to check"
        );
        assert!(line_of(&got).contains("no check is due"));
    }

    /// The cry-wolf case the coverage question exists to prevent: something changed, but no step
    /// in the graph globs it, so selecting nothing is the right answer.
    #[test]
    fn changes_no_step_covers_pass_rather_than_refuse() {
        let got = verdict(&Facts {
            selected: Some(0),
            changed: vec![".gitlabels".to_owned()],
            covering: Some(0),
            ..selecting()
        });
        assert!(got.is_ok(), "an uncovered change is not a broken gate");
        assert!(line_of(&got).contains("no check in the graph covers what changed"));
    }

    #[test]
    fn a_detached_head_is_reported_rather_than_refused() {
        let got = verdict(&Facts {
            branch: None,
            ..selecting()
        });
        assert!(
            got.is_ok(),
            "`--pr` resolves HEAD detached exactly as it does on a branch"
        );
        assert!(line_of(&got).contains("DETACHED HEAD"));
    }

    #[test]
    fn every_unanswerable_question_refuses_with_its_remedy() {
        // Each way the floor cannot say what the gate will check, named by what hk would
        // otherwise have done about it.
        let cases = [
            (
                Facts {
                    head: None,
                    ..selecting()
                },
                "not a git worktree",
            ),
            (
                Facts {
                    configured: None,
                    ..selecting()
                },
                "Set `default_branch` in hk.pkl",
            ),
            (
                Facts {
                    base: None,
                    ..selecting()
                },
                "widening to every file in the repo",
            ),
            (
                Facts {
                    ahead_of_fork: None,
                    ..selecting()
                },
                "share no history",
            ),
            (
                Facts {
                    selected: None,
                    ..selecting()
                },
                "would not say what it is about to select",
            ),
        ];
        for (facts, remedy) in cases {
            let got = verdict(&facts);
            assert!(
                got.is_err(),
                "an unanswerable question must refuse: {remedy}"
            );
            let line = line_of(&got);
            assert!(line.contains("REFUSED"), "{line}");
            assert!(line.contains(remedy), "{line}");
        }
    }
}
