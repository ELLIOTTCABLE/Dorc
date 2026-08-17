//! Wiring self-test for `hk.pkl`'s path globs: does each pre-commit step SEE the files it owns?
//!
//! It earns its keep for the same reason the commit-msg battery does — the failure direction is
//! OPEN. A step whose glob matches nothing is skipped in SILENCE, with no line printed for it at
//! all, so a glob that stops reaching a case shape drops a whole gate tier and looks exactly like
//! a clean commit. That is not hypothetical: single-file whole-product looms went unreached for
//! the life of the loom conversion. `hk run --plan` answers "would this step see this file"
//! without running the step, so each assertion costs one short-lived process and executes nothing.

use std::path::Path;
use std::process::{Command, Stdio};

/// One representative path, and whether the named pre-commit step must see it.
struct Reach {
    name: &'static str,
    step: &'static str,
    path: &'static str,
    want_seen: bool,
}

/// Both directions, because that glob is a COST decision as much as a coverage one: the two
/// shapes a whole-product case comes in must be seen, and the `aid` catalog looms must not be —
/// the e2e runner mints no trial for those, so reaching them would spawn it to do nothing.
const REACH: &[Reach] = &[
    Reach {
        name: "sees-a-single-file-whole-product-loom",
        step: "e2e",
        path: "spike/crates/cli/tests/whygallery-webhost-whole.loom",
        want_seen: true,
    },
    Reach {
        name: "sees-a-dir-shaped-case",
        step: "e2e",
        path: "spike/crates/cli/tests/headline-partial/book.sh",
        want_seen: true,
    },
    Reach {
        name: "leaves-an-aid-catalog-loom-alone",
        step: "e2e",
        path: "spike/crates/aid/tests/cli-help-page.loom",
        want_seen: false,
    },
    // The detached-unit fmt step fails open the same way: `cargo fmt --manifest-path
    // spike/Cargo.toml` cannot reach a crate carrying its own `[workspace]`, so if this glob
    // stops matching, the units go unformatted with nothing printed. Both directions, because
    // the narrowing is deliberate — `verify` itself IS a workspace member and `cargo_fmt`
    // already owns it.
    Reach {
        name: "sees-a-detached-verification-unit",
        step: "cargo_fmt_detached",
        path: "spike/verify/kani/src/lib.rs",
        want_seen: true,
    },
    Reach {
        name: "leaves-the-workspace-member-verify-crate-alone",
        step: "cargo_fmt_detached",
        path: "spike/verify/src/check.rs",
        want_seen: false,
    },
];

/// Ask hk whether `step` would see `path`.
///
/// `Err` covers a plan that says neither: an output-format change, or a step this hook no longer
/// has, must FAIL this battery rather than quietly satisfy it.
fn step_sees(hk: &Path, step: &str, path: &str) -> Result<bool, String> {
    let out = Command::new(hk)
        .current_dir(internal_tooling::repo_root())
        .args(["run", "pre-commit", "--plan", "--step", step, path])
        .env("HK_SKIP_HOOK", "")
        .stderr(Stdio::null())
        .output()
        .map_err(|why| format!("could not run {}: {why}", hk.display()))?;
    let plan = String::from_utf8_lossy(&out.stdout);
    if plan.contains("no files matched") {
        Ok(false)
    } else if plan.contains("matched") {
        Ok(true)
    } else {
        Err(format!(
            "unreadable plan for {path}: {:?}",
            plan.trim_end().lines().last().unwrap_or("<empty>")
        ))
    }
}

pub(crate) fn run() -> u8 {
    let Some(hk) = internal_tooling::which("hk") else {
        eprintln!("step-globs: hk is not on PATH — the pinned one comes from mise.toml");
        return 2;
    };
    let mut failures = 0_u32;
    for case in REACH {
        let name = case.name;
        match step_sees(&hk, case.step, case.path) {
            Err(why) => {
                eprintln!("step-globs: {why}");
                return 2;
            }
            Ok(seen) if seen == case.want_seen => println!("ok   {name}"),
            Ok(seen) => {
                let verdict = |seen: bool| if seen { "seen" } else { "unseen" };
                let (want, got) = (verdict(case.want_seen), verdict(seen));
                println!("FAIL {name} (want {want}, got {got}: {})", case.path);
                failures = failures.saturating_add(1);
            }
        }
    }
    if failures == 0 {
        println!("pre-commit globs: every path reaches the step it should");
        0
    } else {
        eprintln!("{failures} case(s) failed");
        1
    }
}
