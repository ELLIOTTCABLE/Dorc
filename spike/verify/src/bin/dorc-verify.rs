//! `dorc-verify` — the minispec binder's CLI. Reach it through the root config's `verify:*`
//! tasks, which carry the cwd and the tier each subcommand expects.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a developer-facing instrument's output IS its product"
)]

use std::process::ExitCode;

use dorc_verify::badge::Badge;
use dorc_verify::catalogue_lock::LAWS;
use dorc_verify::evidence::Tier;
use dorc_verify::{check, derivation, evidence, kani, pipeline, promote, repo_root, report, unit};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest: Vec<&str> = args.iter().skip(1).map(String::as_str).collect();
    match args.first().map(String::as_str) {
        Some("check") => run_check(),
        Some("report") => run_report(&rest),
        Some("promote") => run_promote(&rest),
        Some("materialize") => run_materialize(),
        Some("lean-build") => run_lean_build(),
        Some("kani") => run_kani(rest.first().copied()),
        other => {
            eprintln!("dorc-verify: unknown task {:?}", other.unwrap_or("<none>"));
            eprintln!(
                "tasks: check, report [--write] [--with-lean] [--with-kani], promote \
                 [--with-lean] [--with-kani] [--seat|--proof|--harness <Slug>=<value>], \
                 materialize, lean-build, kani [<harness>]"
            );
            ExitCode::from(2)
        }
    }
}

fn run_check() -> ExitCode {
    match check::run(repo_root()) {
        Err(why) => {
            eprintln!("dorc-verify check: {why}");
            ExitCode::from(2)
        }
        Ok(findings) => {
            for note in &findings.advisories {
                println!("note: {note}");
            }
            if findings.failures.is_empty() {
                println!("dorc-verify: {} law(s) coherent", LAWS.len());
                ExitCode::SUCCESS
            } else {
                for failure in &findings.failures {
                    eprintln!("FAIL  {failure}");
                }
                ExitCode::from(1)
            }
        }
    }
}

fn run_report(args: &[&str]) -> ExitCode {
    let root = repo_root();
    // The COMMITTED report is the cheap tier's render, because the cheap gate is what verifies
    // it is current. Writing a with-lean render would commit evidence the ordinary gate cannot
    // recompute, which is the cached-verdict shape this whole design refuses.
    if args.contains(&"--write") && args.iter().any(|arg| arg.starts_with("--with-")) {
        eprintln!(
            "dorc-verify report: --write publishes the CHEAP-tier render, so engine flags cannot \
             ride it. Run the engine check and republish separately."
        );
        return ExitCode::from(2);
    }
    let built = args
        .contains(&"--with-lean")
        .then(|| pipeline::lean_build(root, &dorc_verify::lean_build_root()));
    // Only the harnesses the catalogue pairs: those are the ones a `pinned` cell reads, and the
    // battery around them costs tens of minutes to answer a question nobody asked here.
    let paired: Vec<&str> = LAWS.iter().filter_map(|law| law.harness).collect();
    let pinned = match args
        .contains(&"--with-kani")
        .then(|| kani::run(root, &paired, &mut |line| println!("{line}")))
    {
        None => None,
        Some(Ok(report)) => Some(report),
        Some(Err(why)) => {
            eprintln!("dorc-verify report: {why}");
            return ExitCode::from(2);
        }
    };
    let tier = if built.is_none() && pinned.is_none() {
        Tier::Cheap
    } else {
        Tier::WithEngines {
            lean_built: built.as_ref().map(Result::is_ok),
            kani: pinned.as_ref(),
        }
    };
    if let Some(Ok(built)) = &built
        && built.dependency_holes > 0
    {
        println!(
            "note: {} holed declaration(s) in the dependency closure — anything proved \
             through one is not proved",
            built.dependency_holes
        );
    }
    let units = match unit::load_all(root) {
        Ok(units) => units,
        Err(why) => {
            eprintln!("dorc-verify report: {why}");
            return ExitCode::from(2);
        }
    };
    let generated = root.join("minispec").join("Generated");
    let (holes, axioms) = pipeline::census(&generated).unwrap_or((0, 0));
    let rows: Vec<report::Row<'_>> = LAWS
        .iter()
        .map(|law| {
            let unit = units.iter().find(|u| u.slug == law.slug);
            report::Row {
                law,
                unit,
                evidence: evidence::compute(law, unit, root, tier),
            }
        })
        .collect();
    let text = report::render(&rows, tier, report::Census { holes, axioms }, root);

    if args.contains(&"--write") {
        if let Err(e) = std::fs::write(report::path(root), &text) {
            eprintln!("dorc-verify report: {e}");
            return ExitCode::from(2);
        }
        println!("wrote {}", report::path(root).display());
        return ExitCode::SUCCESS;
    }

    // THE VERDICT LEADS, and it leads on stderr so stdout stays the artifact's own bytes. It
    // used to be the last line under seventy lines of report, which is the one position a
    // reader skimming a generated document reliably does not reach.
    //
    // At an engine tier there is no committed copy to compare against — those renders are
    // deliberately never published — so the verdict is the badge comparison itself. Bare, the
    // committed report IS the drift alarm.
    let verdict = if built.is_some() || pinned.is_some() {
        mismatch_verdict(&rows)
    } else {
        freshness_verdict(root, &text)
    };
    match &verdict {
        Ok(line) => eprintln!("{line}"),
        Err(refusal) => eprintln!("FAIL  {refusal}"),
    }
    if let Ok(Some(warning)) = derivation::drift(root) {
        eprintln!("note: {warning}");
    }
    print!("{text}");
    if verdict.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn freshness_verdict(root: &std::path::Path, text: &str) -> Result<String, String> {
    let freshness = report::freshness(root, text);
    if matches!(freshness, report::Freshness::Current) {
        Ok("dorc-verify report: minispec/REPORT.md is CURRENT".to_owned())
    } else {
        Err(report::describe_staleness(&freshness))
    }
}

fn mismatch_verdict(rows: &[report::Row<'_>]) -> Result<String, String> {
    let mismatches = mismatches(rows);
    if mismatches.is_empty() {
        Ok(format!(
            "dorc-verify report: {} law(s) match what the catalogue promoted",
            rows.len()
        ))
    } else {
        Err(mismatches.join("\nFAIL  "))
    }
}

/// The promote act. It writes Rust source that this binary has already compiled in, so the
/// republish it names cannot happen in the same process — the next `report --write` rebuilds
/// against the new lock. Forgetting that step is what the cheap gate's freshness check catches.
fn run_promote(args: &[&str]) -> ExitCode {
    let root = repo_root();
    let inputs = match promote::Inputs::parse(args) {
        Ok(inputs) => inputs,
        Err(why) => {
            eprintln!("dorc-verify {why}");
            return ExitCode::from(2);
        }
    };
    let units = match unit::load_all(root) {
        Ok(units) => units,
        Err(why) => {
            eprintln!("dorc-verify promote: {why}");
            return ExitCode::from(2);
        }
    };
    // Claims first, so the Kani lane below is bounded to the harnesses this promote is ABOUT.
    let claimed = match promote::claims(&units, &LAWS, &inputs) {
        Ok(claimed) => claimed,
        Err(why) => {
            eprintln!("dorc-verify promote: {why}");
            return ExitCode::from(2);
        }
    };
    let built = args
        .contains(&"--with-lean")
        .then(|| pipeline::lean_build(root, &dorc_verify::lean_build_root()));
    let pinned = match args.contains(&"--with-kani").then(|| {
        kani::run(root, &promote::paired_harnesses(&claimed), &mut |line| {
            println!("{line}");
        })
    }) {
        None => None,
        Some(Ok(report)) => Some(report),
        Some(Err(why)) => {
            eprintln!("dorc-verify promote: {why}");
            return ExitCode::from(2);
        }
    };
    let tier = if built.is_none() && pinned.is_none() {
        Tier::Cheap
    } else {
        Tier::WithEngines {
            lean_built: built.as_ref().map(Result::is_ok),
            kani: pinned.as_ref(),
        }
    };
    let rows = promote::finish(root, tier, &units, &claimed);
    for row in &rows {
        for movement in &row.movements {
            println!("{}: {movement}", row.slug);
        }
    }
    if let Err(e) = std::fs::write(promote::path(root), promote::render(&rows)) {
        eprintln!("dorc-verify promote: {e}");
        return ExitCode::from(2);
    }
    println!(
        "promoted {} law(s) into {}",
        rows.len(),
        promote::path(root).display()
    );
    println!("next: `mise run verify:report -- --write`, then review both diffs");
    ExitCode::SUCCESS
}

fn run_materialize() -> ExitCode {
    match pipeline::materialize(repo_root()) {
        Err(why) => {
            eprintln!("dorc-verify materialize: {why}");
            ExitCode::from(2)
        }
        Ok(done) => {
            for file in &done.written {
                println!("wrote {file}");
            }
            println!(
                "census: {} proof hole(s), {} external axiom(s)",
                done.holes, done.axioms
            );
            if done.holes > 0 {
                eprintln!(
                    "FAIL  the strict pipeline emitted {} hole(s); a hole typechecks, so nothing \
                     downstream of one is proved",
                    done.holes
                );
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
    }
}

/// Every computed badge that disagrees with what the catalogue promoted, in EITHER direction —
/// rot (promoted earned, evidence gone) and ambition (promoted todo, evidence present) are both
/// a lie about coverage.
fn mismatches(rows: &[report::Row<'_>]) -> Vec<String> {
    rows.iter()
        .flat_map(|row| {
            row.evidence
                .iter()
                .zip(Badge::ALL)
                .filter(|(found, badge)| !found.agrees_with(row.law.expectation(*badge)))
                .map(|(found, badge)| {
                    format!(
                        "{}: `{badge}` promoted as {}, evidence says {}",
                        row.law.slug,
                        row.law.expectation(badge).render(),
                        found.render()
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// The Kani lane. Exit codes are a trichotomy on purpose: 0 every harness green, 1 a real
/// finding (a counterexample, or a harness that blew past its budget), 2 the lane could not run
/// at all. Collapsing the last two would let an absent toolchain read as a passing lane.
fn run_kani(arg: Option<&str>) -> ExitCode {
    if arg == Some("--setup") {
        return match kani::setup() {
            Ok(()) => ExitCode::SUCCESS,
            Err(why) => {
                eprintln!("dorc-verify kani: {why}");
                ExitCode::from(2)
            }
        };
    }
    // Per-harness lines are printed AS THEY LAND, not collected and dumped at the end. A full
    // battery is tens of minutes, and a run killed partway through used to lose every verdict
    // it had already earned — which is how a lane becomes hostage to its slowest harness.
    let selection: Vec<&str> = arg.into_iter().collect();
    match kani::run(repo_root(), &selection, &mut |line| println!("{line}")) {
        Err(why) => {
            eprintln!("dorc-verify kani: {why}");
            ExitCode::from(2)
        }
        Ok(report) => {
            println!(
                "kani: {} green, {} failed, {} over budget, of {} harness(es)",
                report.green.len(),
                report.failed.len(),
                report.over_budget.len(),
                report.harnesses.len()
            );
            for name in &report.failed {
                eprintln!(
                    "FAIL  {name}: a counterexample is a finding about the code or the law — \
                     capture it, never re-tune the harness"
                );
            }
            for name in &report.over_budget {
                eprintln!(
                    "FAIL  {name}: killed at the per-harness budget, so the law is UNJUDGED. \
                     The formula needs a shape the checker can afford, not a longer wait"
                );
            }
            if report.all_green() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}

fn run_lean_build() -> ExitCode {
    match pipeline::lean_build(repo_root(), &dorc_verify::lean_build_root()) {
        Ok(built) => {
            println!(
                "lake build: green ({} holed declaration(s) across the dependency closure)",
                built.dependency_holes
            );
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("dorc-verify lean-build: {why}");
            ExitCode::from(1)
        }
    }
}
