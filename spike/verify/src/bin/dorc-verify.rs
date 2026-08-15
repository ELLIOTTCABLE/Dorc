//! `dorc-verify` — the minispec binder's CLI. Reach it through the root config's `verify:*`
//! tasks, which carry the cwd and the tier each subcommand expects.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a developer-facing instrument's output IS its product"
)]

use std::process::ExitCode;

use dorc_verify::catalogue_lock::LAWS;
use dorc_verify::evidence::Tier;
use dorc_verify::{check, evidence, pipeline, repo_root, report, unit};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest: Vec<&str> = args.iter().skip(1).map(String::as_str).collect();
    match args.first().map(String::as_str) {
        Some("check") => run_check(),
        Some("report") => run_report(&rest),
        Some("materialize") => run_materialize(),
        Some("lean-build") => run_lean_build(),
        other => {
            eprintln!("dorc-verify: unknown task {:?}", other.unwrap_or("<none>"));
            eprintln!("tasks: check, report [--write] [--with-lean], materialize, lean-build");
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
    let tier = if args.contains(&"--with-lean") {
        Tier::WithLean {
            lean_built: pipeline::lean_build(root, &dorc_verify::lean_build_root()).is_ok(),
        }
    } else {
        Tier::Cheap
    };
    let units = match unit::load_all(root) {
        Ok(units) => units,
        Err(why) => {
            eprintln!("dorc-verify report: {why}");
            return ExitCode::from(2);
        }
    };
    let generated = root.join("minispec").join("Generated");
    let holes = pipeline::census(&generated).map_or(0, |(holes, _)| holes);
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
    let text = report::render(&rows, tier, holes);

    if args.contains(&"--write") {
        if let Err(e) = std::fs::write(report::path(root), &text) {
            eprintln!("dorc-verify report: {e}");
            return ExitCode::from(2);
        }
        println!("wrote {}", report::path(root).display());
        return ExitCode::SUCCESS;
    }
    // Unwritten, the report is still the drift alarm: a committed copy that no longer matches
    // what the evidence says is exactly the stale-coverage claim this system exists to catch.
    print!("{text}");
    match std::fs::read_to_string(report::path(root)) {
        Ok(committed) if committed == text => ExitCode::SUCCESS,
        Ok(_) => {
            eprintln!(
                "FAIL  minispec/REPORT.md is stale — re-run with --write and review the diff"
            );
            ExitCode::from(1)
        }
        Err(_) => {
            eprintln!("FAIL  minispec/REPORT.md is missing — re-run with --write");
            ExitCode::from(1)
        }
    }
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

fn run_lean_build() -> ExitCode {
    match pipeline::lean_build(repo_root(), &dorc_verify::lean_build_root()) {
        Ok(()) => {
            println!("lake build: green");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("dorc-verify lean-build: {why}");
            ExitCode::from(1)
        }
    }
}
