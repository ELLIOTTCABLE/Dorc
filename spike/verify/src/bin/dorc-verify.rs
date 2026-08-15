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
    // The COMMITTED report is the cheap tier's render, because the cheap gate is what verifies
    // it is current. Writing a with-lean render would commit evidence the ordinary gate cannot
    // recompute, which is the cached-verdict shape this whole design refuses.
    if args.contains(&"--write") && args.contains(&"--with-lean") {
        eprintln!(
            "dorc-verify report: --write publishes the CHEAP-tier render, so --with-lean cannot \
             ride it. Run them separately: --with-lean to recompute and compare, --write to \
             republish."
        );
        return ExitCode::from(2);
    }
    let built = args
        .contains(&"--with-lean")
        .then(|| pipeline::lean_build(root, &dorc_verify::lean_build_root()));
    let tier = match &built {
        None => Tier::Cheap,
        Some(result) => Tier::WithLean {
            lean_built: result.is_ok(),
        },
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
    let text = report::render(&rows, tier, report::Census { holes, axioms });

    if args.contains(&"--write") {
        if let Err(e) = std::fs::write(report::path(root), &text) {
            eprintln!("dorc-verify report: {e}");
            return ExitCode::from(2);
        }
        println!("wrote {}", report::path(root).display());
        return ExitCode::SUCCESS;
    }
    print!("{text}");

    // At the with-lean tier there is no committed copy to compare against — that render is
    // deliberately never published — so the gate is the badge comparison itself.
    if built.is_some() {
        let mismatches: Vec<String> = rows
            .iter()
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
            .collect();
        if mismatches.is_empty() {
            return ExitCode::SUCCESS;
        }
        for line in &mismatches {
            eprintln!("FAIL  {line}");
        }
        return ExitCode::from(1);
    }

    // Bare, the report is the drift alarm: a committed copy that no longer matches what the
    // evidence says is exactly the stale-coverage claim this system exists to catch.
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
