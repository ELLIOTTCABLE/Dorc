//! `dorc` — the thin spike CLI. Wires the network-free pipeline
//! (parse → cfg → classify → plan) and prints the plan as sh.
//!
//! This is an I/O edge: `inv-determinism` exempts `cli` (and `hostsim`); the
//! analyzer kernel it calls is a pure function. Diagnostics go to stderr, the
//! rendered plan to stdout (so `dorc book.sh >plan.sh` captures just the plan).
//!
//! ```text
//! usage: dorc <book.sh> [--oracle <oracle.sh>]... [--has <kind:entity>]...
//!   --oracle  lift this oracle file into the kind-index (repeatable)
//!   --has     the (simulated) host already holds this fact (repeatable) — stands
//!             in for a real probe verdict; a held fact reads Converged.
//! ```

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::process::ExitCode;

use dorc_analysis::effect::FactKey;
use dorc_core::{Interner, KindId, OpaqueToken, Verdict};

const USAGE: &str = "usage: dorc <book.sh> [--oracle <oracle.sh>]... [--has <kind:entity>]...";

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
    has: Vec<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut book: Option<String> = None;
    let mut oracles = Vec::new();
    let mut has = Vec::new();
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--oracle" => oracles.push(it.next().ok_or("--oracle needs a path")?),
            "--has" => has.push(it.next().ok_or("--has needs a kind:entity")?),
            "-h" | "--help" => return Err(USAGE.to_string()),
            other if other.starts_with('-') => return Err(format!("unknown flag {other}")),
            other => {
                if book.replace(other.to_string()).is_some() {
                    return Err("only one book may be given".to_string());
                }
            }
        }
    }
    Ok(Args {
        book: book.ok_or(USAGE)?,
        oracles,
        has,
    })
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let mut interner = Interner::default();

    // Lift the oracle files into one kind-index.
    let oracle_srcs: Vec<String> = args
        .oracles
        .iter()
        .map(|p| std::fs::read_to_string(p).map_err(|e| format!("reading oracle {p}: {e}")))
        .collect::<Result<_, _>>()?;
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();
    let lifted = dorc_oracle::lift(&mut interner, &oracle_refs);
    report("oracle", &lifted.diags);
    let idx = lifted.value;

    // Parse + analyze the book.
    let book_src = std::fs::read_to_string(&args.book)
        .map_err(|e| format!("reading book {}: {e}", args.book))?;
    let parsed = dorc_syntax::parse(&book_src);
    report("parse", &parsed.diags);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    report("cfg", &cfg.diags);
    let classes = dorc_analysis::effect::classify(&cfg.value, &parsed.value, &idx, &mut interner);

    // The simulated host holds exactly the `--has` facts (a held fact reads
    // Converged; anything else Diverged — the `--has` set is the whole host state).
    let held: BTreeSet<FactKey> = args
        .has
        .iter()
        .map(|s| parse_fact(&mut interner, s))
        .collect::<Result<_, _>>()?;
    let plan = dorc_plan::build_plan(&book_src, &parsed.value, &cfg.value, &classes, |f| {
        if held.contains(&f) {
            Verdict::Converged
        } else {
            Verdict::Diverged
        }
    });

    print!("{}", plan.render_sh(&interner));
    Ok(())
}

/// Print a stage's diagnostics to stderr (keeping stdout = the plan).
fn report(stage: &str, diags: &[dorc_core::Diagnostic]) {
    for d in diags {
        eprintln!("{stage}: {}: {}", d.code.0, d.message);
    }
}

/// Parse a `kind:entity` spec from `--has`.
fn parse_fact(interner: &mut Interner, spec: &str) -> Result<FactKey, String> {
    let (kind, entity) = spec
        .split_once(':')
        .ok_or_else(|| format!("--has expects kind:entity, got {spec:?}"))?;
    if kind.is_empty() || entity.is_empty() {
        return Err(format!("--has kind and entity must be non-empty, got {spec:?}"));
    }
    Ok(FactKey {
        kind: KindId(interner.intern(kind)),
        entity: OpaqueToken(interner.intern(entity)),
    })
}
