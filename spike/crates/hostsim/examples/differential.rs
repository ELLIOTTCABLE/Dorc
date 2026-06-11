//! `differential` — the round-21 arch-7 seeded-random differential testing harness, as a
//! thin CLI over [`dorc_hostsim::differential`]. The cheapest local approximation of the
//! deferred cm-1 product-gate: it drives the REAL `dorc` binary over generated
//! `(book, oracle-set, host-state)` triples and asserts the soundness property (the apply
//! trace == the bare trace modulo licensed elisions; an under-execute is the disaster
//! class).
//!
//! ```text
//! usage: cargo run --example differential -- [OPTIONS]
//!   --seed N                one reproducible trial (verbose: book, ledger, traces, verdict)
//!   --sweep COUNT           run COUNT trials (summary: clean/findings/rejects + finding seeds)
//!   --start-seed N          first seed of a sweep (default 0)
//!   --max-secs S            wall-clock budget for a sweep (best-effort; checked per trial)
//!   --emit-findings         write finding case-dir drafts under e2e/findings/<seed>-<slug>/
//! ```
//!
//! Determinism: a `u64` seed fully determines a trial (the generator's randomness flows
//! through a seeded LCG). The harness itself is an I/O edge (it spawns `dorc`/`dash` and
//! writes scratch dirs) — like `cli`, it is the sanctioned boundary, not the pure kernel.

// The example is an I/O edge (CLI output + process/disk I/O to drive dorc/dash), exactly
// like the `cli` crate. The kernel it ultimately exercises (via the binary) stays
// print-free; this front-end may print its report and touch disk.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "differential is a CLI harness I/O edge: report to stdout, drives dorc/dash"
)]

use std::path::{Path, PathBuf};
use std::time::Instant;

use dorc_hostsim::differential::{
    self, FindingClass, Tools, Trial, Verdict, emit_finding, generate, judge, minimize, run_trial,
    shimmed_apply_cmds,
};

struct Opts {
    seed: Option<u64>,
    sweep: Option<u64>,
    start_seed: u64,
    max_secs: Option<u64>,
    emit_findings: bool,
}

fn main() -> std::process::ExitCode {
    let opts = match parse_opts() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("differential: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let spike_root = spike_root();
    let tools = match Tools::locate(&spike_root) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("differential: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    if let Some(seed) = opts.seed {
        one_trial(&tools, seed, &spike_root, opts.emit_findings);
        std::process::ExitCode::SUCCESS
    } else {
        let count = opts.sweep.unwrap_or(500);
        sweep(&tools, &spike_root, &opts, count)
    }
}

/// Run one trial verbosely (the `--seed N` reproducer path).
fn one_trial(tools: &Tools, seed: u64, spike_root: &Path, emit: bool) {
    let trial = generate(seed);
    println!("=== seed {seed} · shape {:?} ===", trial.shape);
    println!("--- book.sh ---\n{}", trial.book);
    for (name, _) in &trial.oracles {
        println!("--- oracle: {name} ---");
    }
    match run_trial(tools, &trial) {
        Ok(outcome) => {
            println!("--- probe-results ---\n{}", outcome.probe_results);
            println!("--- disposition ledger (--debug-argv) ---");
            for e in &outcome.ledger {
                println!("argv {} {} {}", e.leafid, e.disposition, e.argv);
            }
            println!("--- bare trace ---");
            for l in &outcome.bare_trace {
                println!("ran: {l}");
            }
            println!("--- apply trace ---");
            for l in &outcome.apply_trace {
                println!("ran: {l}");
            }
            let shimmed = shimmed_apply_cmds(&trial);
            match judge(&trial, &outcome, &shimmed) {
                Verdict::Clean => println!("\nVERDICT: clean"),
                Verdict::Finding(f) => {
                    println!("\nVERDICT: FINDING [{}]\n{}", f.class.slug(), f.diagnosis);
                    if emit {
                        emit_one_finding(tools, spike_root, &trial, &outcome, &f, &shimmed);
                    }
                }
            }
        }
        Err(e) => println!("\nRUN ERROR (generator/harness fault): {e:?}"),
    }
}

/// Run a sweep, streaming a one-line tick per finding and a final summary.
fn sweep(tools: &Tools, spike_root: &Path, opts: &Opts, count: u64) -> std::process::ExitCode {
    let start = Instant::now();
    let mut trials = 0u64;
    let mut clean = 0u64;
    let mut rejects = 0u64;
    // class → list of seeds.
    let mut findings: Vec<(u64, FindingClass)> = Vec::new();

    for i in 0..count {
        if let Some(budget) = opts.max_secs
            && start.elapsed().as_secs() >= budget
        {
            println!("(budget {budget}s reached after {trials} trials)");
            break;
        }
        let seed = opts.start_seed.wrapping_add(i);
        let trial = generate(seed);
        trials = trials.saturating_add(1);
        match run_trial(tools, &trial) {
            Ok(outcome) => {
                let shimmed = shimmed_apply_cmds(&trial);
                match judge(&trial, &outcome, &shimmed) {
                    Verdict::Clean => clean = clean.saturating_add(1),
                    Verdict::Finding(f) => {
                        findings.push((seed, f.class));
                        println!(
                            "FINDING seed {seed} [{}] {}",
                            f.class.slug(),
                            short(&f.diagnosis)
                        );
                        if opts.emit_findings {
                            emit_one_finding(tools, spike_root, &trial, &outcome, &f, &shimmed);
                        }
                    }
                }
            }
            Err(e) => {
                rejects = rejects.saturating_add(1);
                println!("REJECT seed {seed}: {e:?}");
            }
        }
    }

    println!("---");
    println!(
        "sweep: {trials} trials · {clean} clean · {} findings · {rejects} generator-rejects · {:.1}s",
        findings.len(),
        start.elapsed().as_secs_f64()
    );
    if findings.is_empty() {
        std::process::ExitCode::SUCCESS
    } else {
        // Group seeds by class for the summary.
        let mut by_class: std::collections::BTreeMap<FindingClass, Vec<u64>> =
            std::collections::BTreeMap::new();
        for (seed, class) in &findings {
            by_class.entry(*class).or_default().push(*seed);
        }
        for (class, seeds) in &by_class {
            println!("  {} ({}): seeds {seeds:?}", class.slug(), seeds.len());
        }
        std::process::ExitCode::FAILURE
    }
}

/// Minimize then emit a finding draft; print where it landed.
fn emit_one_finding(
    tools: &Tools,
    spike_root: &Path,
    trial: &Trial,
    outcome: &differential::RunOutcome,
    finding: &differential::Finding,
    shimmed: &[String],
) {
    let minimized = minimize(tools, trial, finding.class, shimmed);
    // Re-run the minimized trial to get a faithful outcome for the draft.
    let (emit_trial, emit_outcome) = match run_trial(tools, &minimized) {
        Ok(o) if matches!(judge(&minimized, &o, shimmed), Verdict::Finding(_)) => (minimized, o),
        _ => (trial.clone(), outcome.clone()), // minimization lost it ⇒ keep the original
    };
    let findings_root = spike_root.join("e2e").join("findings");
    match emit_finding(&findings_root, &emit_trial, &emit_outcome, finding) {
        Ok(dir) => println!("  emitted draft: {}", dir.display()),
        Err(e) => eprintln!("  failed to emit finding: {e}"),
    }
}

fn short(s: &str) -> String {
    let first = s.lines().next().unwrap_or("");
    if first.len() > 100 {
        format!("{}…", &first[..100])
    } else {
        first.to_string()
    }
}

fn parse_opts() -> Result<Opts, String> {
    let mut seed = None;
    let mut sweep = None;
    let mut start_seed = 0u64;
    let mut max_secs = None;
    let mut emit_findings = false;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--seed" => {
                seed = Some(
                    it.next()
                        .and_then(|v| v.parse().ok())
                        .ok_or("--seed needs a u64")?,
                );
            }
            "--sweep" => {
                sweep = Some(
                    it.next()
                        .and_then(|v| v.parse().ok())
                        .ok_or("--sweep needs a count")?,
                );
            }
            "--start-seed" => {
                start_seed = it
                    .next()
                    .and_then(|v| v.parse().ok())
                    .ok_or("--start-seed needs a u64")?;
            }
            "--max-secs" => {
                max_secs = Some(
                    it.next()
                        .and_then(|v| v.parse().ok())
                        .ok_or("--max-secs needs seconds")?,
                );
            }
            "--emit-findings" => emit_findings = true,
            "-h" | "--help" => return Err("see the module doc-comment for usage".into()),
            other => return Err(format!("unexpected argument {other:?}")),
        }
    }
    Ok(Opts {
        seed,
        sweep,
        start_seed,
        max_secs,
        emit_findings,
    })
}

/// The spike root from this example's manifest dir (`crates/hostsim` → `spike`).
fn spike_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // crates
    p.pop(); // spike
    p
}
