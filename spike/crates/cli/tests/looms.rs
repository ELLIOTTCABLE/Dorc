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

use errorloom::{Case, CaseFile, CaseRenderer as _, describe_divergence, fixpoint_check};
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
    if let Err(error) = parsed.check_hygiene(Some("code")) {
        // Dump here too, not only on the fixpoint path. Authoring a NEW replay block is `$ cmd`
        // with no output yet, and an empty output cannot surface the case's own slug — so hygiene,
        // which runs first, is exactly where that author is standing when they need the candidate.
        let candidate = DorcConsumer::new()
            .render_case(&parsed)
            .map(|rendered| dump_candidate(name, &rendered))
            .unwrap_or_default();
        return Err(format!("FAIL  {name}  [hygiene: {error}]{candidate}").into());
    }

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
        "FAIL  {name}  [render fixpoint: the case no longer reproduces from the current engine + catalog — re-bless it, or fix the drift]\n{}{}",
        divergence(&text, &rendered),
        dump_candidate(name, &rendered)
    )
    .into())
}

/// `DORC_LOOM_DUMP=<dir>` — write each drifted case's CANDIDATE transcript there, so a render
/// iteration is `diff` against a file instead of promote-then-`git diff`-then-`git checkout`.
/// Read-only with respect to the corpus: the dump is a scratch copy, never the committed case,
/// and only drifted cases are written (an unchanged candidate is the committed bytes).
fn dump_candidate(name: &str, rendered: &str) -> String {
    let Some(dir) = std::env::var_os("DORC_LOOM_DUMP") else {
        return String::new();
    };
    let dir = std::path::PathBuf::from(dir);
    let target = dir.join(format!("{name}.loom"));
    match std::fs::create_dir_all(&dir).and_then(|()| std::fs::write(&target, rendered)) {
        Ok(()) => format!("\n      candidate written to {}", target.display()),
        Err(error) => format!("\n      DORC_LOOM_DUMP write failed: {error}"),
    }
}

/// An aligned first-divergence window over the committed and re-rendered transcripts, indented
/// into the runner's failure block.
fn divergence(want: &str, got: &str) -> String {
    let mut out = String::new();
    for line in describe_divergence(want, got)
        .unwrap_or_else(|| String::from("byte-identical (the check and the report disagree)"))
        .lines()
    {
        let _ = writeln!(out, "      {line}");
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
