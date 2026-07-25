//! The central loom runner (`288` §3): one named, filterable trial per committed case,
//! across every collection.
//!
//! Per case it asserts the three things that hold for EVERY loom, whatever code it
//! defines:
//!
//! 1. the case parses as a txtar container;
//! 2. it is hygiene-clean and surfaces its own `code` slug in each replay block
//!    (`282` §2's coherence gate);
//! 3. it is a RENDER FIXPOINT — re-deriving every replay block through the production
//!    route (`DorcConsumer::render_case`) reproduces the committed transcript byte for
//!    byte. This is the first of the two fixpoint gates `spike/CLAUDE.md`'s
//!    `defining-case-catalog` names; the second (the generated `catalog_lock.rs`
//!    byte-identity gate) is corpus-global and stays in `dorc-loom`'s own suite, as do
//!    the class-specific batteries (lint production-report route, mark mutations).
//!
//! Nothing here reaches for a shell: a command the in-process driver cannot dispatch is
//! an `unsupported replay` failure, never an escalation to generic execution. That is why
//! item 3 has ONE alternative: a WHOLE-PRODUCT loom (`run:` + `fixpoint: executed`) is proven
//! by running the real binary in `e2e.rs`, and this runner asserts only that it declares who
//! does prove it. Items 1 and 2 still hold for every case.

#![expect(
    clippy::print_stderr,
    reason = "the discovery floor aborts before any trial runs; it has no Failed to return"
)]

mod support;

use std::fmt::Write as _;
use std::sync::Arc;

use errorloom::{Case, CaseFile, CaseRenderer as _, fixpoint_check};
use libtest_mimic::{Arguments, Failed, Trial};

use dorc_loom::DorcConsumer;
use support::{LoomCase, case_roots, discover_looms};

/// Parse, hygiene-check, and render-fixpoint one committed case.
fn run_case(case: &LoomCase) -> Result<(), Failed> {
    let name = &case.name;
    let text = std::fs::read_to_string(&case.path)
        .map_err(|error| format!("FAIL  {name}  [read {}: {error}]", case.path.display()))?;
    let parsed = Case::parse(&text)
        .map_err(|error| format!("FAIL  {name}  [case does not parse: {error}]"))?;
    parsed
        .check_hygiene(Some("code"))
        .map_err(|error| format!("FAIL  {name}  [hygiene: {error}]"))?;

    // A WHOLE-PRODUCT loom's transcript is proven by running the real binary in the e2e runner,
    // which is the stricter proof and the only one the sanctioned-executor law permits for a case
    // that materializes mocks. Nothing here reaches for a shell, so this runner asserts only what
    // it can: that the case declares who does prove it.
    if parsed.frontmatter().scalar("fixpoint") == Some("executed") {
        return match parsed.frontmatter().scalar("run") {
            Some(_) => Ok(()),
            None => Err(format!(
                "FAIL  {name}  [fixpoint: `executed` with no `run:` — no runner would ever execute this case, so its transcript is proven by nothing]"
            )
            .into()),
        };
    }

    let file = CaseFile::new(format!("{name}.loom"), text.clone());
    if fixpoint_check(&DorcConsumer::new(), std::slice::from_ref(&file)).is_ok() {
        return Ok(());
    }
    // `fixpoint_check` reports only WHICH case drifted, so re-render for the window: the
    // usual cause is a case authored in a layout the container does not canonicalize to,
    // and the offending line is the whole diagnosis.
    let rendered = DorcConsumer::new()
        .render_case(&Case::parse(&text).map_err(|error| format!("FAIL  {name}  [{error}]"))?)
        .map_err(|error| format!("FAIL  {name}  [render: {error}]"))?;
    Err(format!(
        "FAIL  {name}  [render fixpoint: the case no longer reproduces from the current engine + catalog — re-bless it, or fix the drift]\n{}",
        divergence(&text, &rendered)
    )
    .into())
}

/// A compact first-divergence window over the committed and re-rendered transcripts.
fn divergence(want: &str, got: &str) -> String {
    let want: Vec<&str> = want.lines().collect();
    let got: Vec<&str> = got.lines().collect();
    let at = (0..want.len().max(got.len()))
        .find(|i| want.get(*i) != got.get(*i))
        .unwrap_or(0);
    let mut out = format!("      first divergence at line {}\n", at.saturating_add(1));
    for i in at..(at.saturating_add(3)).min(want.len().max(got.len())) {
        let _ = writeln!(
            out,
            "      -{:?}\n      +{:?}",
            want.get(i).copied().unwrap_or("<eof>"),
            got.get(i).copied().unwrap_or("<eof>")
        );
    }
    out.trim_end().to_owned()
}

fn main() {
    let mut args = Arguments::from_args();
    if args.format.is_none() && std::env::var("DORC_E2E_QUIET").as_deref() == Ok("1") {
        args.format = Some(libtest_mimic::FormatSetting::Terse);
    }
    let discovered = discover_looms(&case_roots());
    // The DISCOVERY FLOOR (see the e2e runner's): walking the wrong roots yields zero
    // trials, and a suite of zero trials EXITS GREEN.
    if discovered.is_empty() {
        eprintln!(
            "FATAL  discovery floor: no `.loom` cases found under any of {:?} — the collection is not where the runner looks, and an empty suite would otherwise pass.",
            case_roots()
        );
        std::process::exit(3);
    }
    let trials: Vec<Trial> = discovered
        .into_iter()
        .map(|case| {
            let case = Arc::new(case);
            Trial::test(case.name.clone(), move || run_case(&case))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit();
}
