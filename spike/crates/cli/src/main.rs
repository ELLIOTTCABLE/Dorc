//! `dorc` — the thin spike CLI: the whole apply-2 round-trip over real files.
//!
//! Reads a book + oracle files, prints a read-only **probe** to stdout, reads the
//! probe **results** from stdin, then prints the eliding **apply** (the book with
//! already-converged lines commented out) to stdout. No executor — it *compiles* a
//! probe and an apply; it runs neither. The simulated host's answers arrive on stdin
//! (in a real deployment those come from running the probe on the host).
//!
//! ```text
//! usage: dorc --book=<book.sh> [-o <oracle.sh>]...
//!   stdin : probe results, one per line — `kind:entity converged|diverged|unknown`
//!   stdout: the probe script, then (after stdin EOF) the eliding-apply book
//! ```
//!
//! I/O edge: `inv-determinism` exempts `cli`; the analyzer kernel it calls is pure.
//! Diagnostics go to stderr so stdout stays the probe+apply.

#![forbid(unsafe_code)]
// The cli is the sanctioned I/O edge (workspace Cargo.toml: "I/O-edge crates may
// `#[expect]` these at the crate root, with reason"): stdout carries the
// probe-then-apply artifact, stderr carries diagnostics. The kernel it drives
// stays print-free. Not a seeded-ratchet expect — this one is permanent for the
// binary's edge.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "cli is the I/O edge: probe/apply to stdout, diagnostics to stderr; the kernel stays print-free"
)]

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::process::ExitCode;

use dorc_core::{Interner, Observable, Predicted, Rc, Verdict};
use dorc_plan::fact_label;

const USAGE: &str = "usage: dorc --book=<book.sh> [-o <oracle.sh>]...";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("dorc: {msg}");
            ExitCode::FAILURE
        }
    }
}

struct Args {
    book: String,
    oracles: Vec<String>,
}

/// Minimal hand-rolled parsing (no `clap` dep): `--book=PATH` / `--book PATH`, and
/// `-o PATH` / `-oPATH` / `--oracle PATH` (repeatable).
fn parse_args() -> Result<Args, String> {
    let mut book: Option<String> = None;
    let mut oracles = Vec::new();
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        if let Some(p) = arg.strip_prefix("--book=") {
            book = Some(p.to_string());
        } else if arg == "--book" {
            book = Some(it.next().ok_or("--book needs a path")?);
        } else if arg == "-o" || arg == "--oracle" {
            oracles.push(it.next().ok_or("-o needs a path")?);
        } else if let Some(p) = arg.strip_prefix("-o").filter(|p| !p.is_empty()) {
            oracles.push(p.to_string());
        } else if arg == "-h" || arg == "--help" {
            return Err(USAGE.to_string());
        } else {
            return Err(format!("unexpected argument {arg:?}; {USAGE}"));
        }
    }
    Ok(Args {
        book: book.ok_or(USAGE)?,
        oracles,
    })
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let mut interner = Interner::default();

    // Lift the oracle files into one shared kind-index.
    let oracle_srcs: Vec<String> = args
        .oracles
        .iter()
        .map(|p| std::fs::read_to_string(p).map_err(|e| format!("reading oracle {p}: {e}")))
        .collect::<Result<_, _>>()?;
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();
    let lifted = dorc_oracle::lift(&mut interner, &oracle_refs);
    report("oracle", &lifted.diags);
    let idx = lifted.value;

    // Lift each oracle's `<provider>__check` functions into a per-file CheckSet (the
    // real entity-resolution mechanism — the engine threads the book's value-flow
    // through these, never parsing argv itself). Shared interner, so provider symbols
    // match the book's command words (204 seam #2).
    let checks: Vec<dorc_oracle::check::CheckSet> = oracle_refs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::check::lift_checks(&mut interner, src);
            report("check", &lifted.diags);
            lifted.value
        })
        .collect();

    // Parse + analyze the book (shared interner, so symbols match the oracles).
    let book_src = std::fs::read_to_string(&args.book)
        .map_err(|e| format!("reading book {}: {e}", args.book))?;
    let parsed = dorc_syntax::parse(&book_src);
    report("parse", &parsed.diags);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    report("cfg", &cfg.diags);
    // Book-side value-flow: resolve each command-site's argv (constant/variable
    // propagation) — the input entity-resolution consumes (19H §1 / 202 §1).
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
    let classified =
        dorc_analysis::effect::classify(&cfg.value, &value, &idx, &checks, &mut interner);
    report("classify", &classified.diags);
    let classes = classified.value;

    // (1) compile + emit the read-only probe.
    let probe =
        dorc_plan::compile_probe(&classes, |kind| idx.probe_for(kind).map(|p| p.body.clone()));
    print!("{}", probe.render_sh(&interner));
    std::io::stdout().flush().ok();

    // (2) read the (simulated) probe results from stdin.
    let mut stdin_buf = String::new();
    std::io::stdin()
        .read_to_string(&mut stdin_buf)
        .map_err(|e| format!("reading stdin: {e}"))?;
    let results = parse_results(&stdin_buf);

    // (3) compile + emit the eliding apply, driven by the probe observations. A fact
    // with no reported observation folds to Unknown / no-rc ⇒ run, no fold
    // (kFAIL-perform).
    let plan = dorc_plan::build_plan(&book_src, &parsed.value, &cfg.value, &classes, |f| {
        results
            .get(&fact_label(&interner, f))
            .copied()
            .unwrap_or(Observable::verdict_only(Verdict::Unknown))
    });
    print!("{}", plan.render_apply(&book_src, &parsed.value));
    Ok(())
}

/// Parse stdin probe-results: `kind:entity#sel converged|diverged|unknown [rc=N]` per
/// line. The optional `rc=N` is the **injected observed exit status** (`19B` build-1)
/// the apply fold + value-preserving substitution read. Blank lines and `#` comments
/// are ignored, so the probe's own `# probe:` echo can be piped back.
///
/// The rc is the OUT-OF-BAND lane as plain data (`19B §2`): it never collides with the
/// verdict token (`unknown` stays a distinct word; the real rc rides as `rc=2`).
///
/// **An rc is carried ONLY when explicitly declared** (`19D`, the `kFAIL-perform`
/// fix): a converged fact with no `rc=N` carries `rc=None` (⊤ for the fold), never a
/// fabricated `rc=0`. The old conforming-`rc=0` default was a confident *wrong* value
/// for a **non-conforming** establish (`useradd` exits 9 when converged): fabricating
/// 0 let the fold short-circuit a `useradd || mkdir` fallback dead — a priority-1
/// under-execute (`inv-kfail`). `core::Observable` already documents that an un-injected
/// rc is ⊤ and the conforming-0 fallback is the *caller's* choice; this caller now
/// declines it, deferring rc-production entirely to build-2's oracle contract (opt-B,
/// `19B §1`). The trade: a conforming establish that does not declare `rc=0` no longer
/// folds its branch — correct (never under-execute > avoid unnecessary-execute); its
/// bare convergence-elision is unaffected (status dead ⇒ `true` stand-in, `19C` §3).
fn parse_results(input: &str) -> BTreeMap<String, Observable> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let mut it = line.split_whitespace();
            let key = it.next()?.to_string();
            let verdict = match it.next() {
                Some("converged") => Verdict::Converged,
                Some("diverged") => Verdict::Diverged,
                _ => Verdict::Unknown,
            };
            // rc is present ONLY when explicitly declared (`rc=N`); an undeclared rc is
            // None ⇒ ⊤ ⇒ no fold through this leaf (the safe `kFAIL-perform` floor).
            let rc = it.find_map(|tok| {
                tok.strip_prefix("rc=")
                    .and_then(|n| n.parse::<i32>().ok())
                    .map(Rc)
            });
            let status = rc.map_or(Predicted::Top, Predicted::Value);
            Some((
                key,
                Observable {
                    effect: verdict,
                    status,
                },
            ))
        })
        .collect()
}

/// Print a stage's diagnostics to stderr (keeping stdout = probe + apply).
fn report(stage: &str, diags: &[dorc_core::Diagnostic]) {
    for d in diags {
        eprintln!("{stage}: {}: {}", d.code.0, d.message);
    }
}
