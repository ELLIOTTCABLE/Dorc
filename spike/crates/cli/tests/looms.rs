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
//! an `unsupported replay` failure, never an escalation to generic execution.

#![expect(
    clippy::print_stderr,
    reason = "the discovery floor aborts before any trial runs; it has no Failed to return"
)]

mod support;

use std::sync::Arc;

use errorloom::{Case, CaseFile, fixpoint_check};
use libtest_mimic::{Arguments, Failed, Trial};

use dorc_loom::DorcConsumer;
use support::{LoomCase, case_roots, discover_looms};

/// The cases known NOT to be render fixpoints at HEAD, with the conductor's ruling that
/// banked them: `289` §2j — a blank-line divergence in the whylog render, deferred to
/// `288:phase-e2e-loom-conversion`. A pin, not an excuse: a case that starts reproducing
/// is a loud XPASS to promote, so the list cannot silently outlive the defect.
const KNOWN_NON_FIXPOINTS: [&str; 3] = [
    "whylog-book-desync",
    "whylog-corrupt",
    "whylog-version-refused",
];

/// Parse, hygiene-check, and render-fixpoint one committed case.
fn run_case(case: &LoomCase) -> Result<(), Failed> {
    let name = &case.name;
    let text = std::fs::read_to_string(&case.path)
        .map_err(|error| format!("FAIL  {name}  [read {}: {error}]", case.path.display()))?;
    Case::parse(&text)
        .map_err(|error| format!("FAIL  {name}  [case does not parse: {error}]"))?
        .check_hygiene(Some("code"))
        .map_err(|error| format!("FAIL  {name}  [hygiene: {error}]"))?;

    let file = CaseFile::new(format!("{name}.loom"), text);
    let reproduced = fixpoint_check(&DorcConsumer::new(), std::slice::from_ref(&file));
    match (reproduced, KNOWN_NON_FIXPOINTS.contains(&name.as_str())) {
        (Ok(()), false) | (Err(_), true) => Ok(()),
        (Ok(()), true) => Err(format!(
            "XPASS {name}  [known non-fixpoint now reproduces — drop it from KNOWN_NON_FIXPOINTS]"
        )
        .into()),
        (Err(error), false) => Err(format!(
            "FAIL  {name}  [render fixpoint: the case no longer reproduces from the current engine + catalog — re-bless it, or fix the drift: {error:?}]"
        )
        .into()),
    }
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
