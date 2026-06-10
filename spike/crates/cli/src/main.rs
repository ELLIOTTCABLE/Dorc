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
//!   stdin : probe results, one per line — `site <leafid> effect=<holds|absent|cant-tell> rc=<n>`
//!           (+ a transitional `declared-rc <leafid> rc=<n>` line for the fold)
//!   stdout: the probe script, then (after stdin EOF) the eliding-apply book
//! ```
//!
//! Round-20 task-D1 (the WIRE — `inv-site-keyed-results`): the probe is a real,
//! self-reporting artifact; its results-records are keyed by command **site** (the
//! stable `LeafId`), not by fact. The simulated host's answers (the e2e
//! `probe-results.txt`, a stand-in for running the rendered probe remotely) are now
//! the site-keyed records the probe itself emits.
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

    // (1) compile + emit the read-only, SELF-REPORTING probe (site-keyed —
    // `inv-site-keyed-results`). Each resolvable site invokes its kind's
    // `oracle_probe_*` wrapper and emits `site <leafid> effect=… rc=…` on stdout.
    let probe = dorc_plan::compile_probe(&parsed.value, &cfg.value, &classes, |kind| {
        idx.probe_for(kind).map(|p| p.body.clone())
    });
    print!("{}", probe.render_sh(&interner));
    std::io::stdout().flush().ok();

    // (2) read the (simulated) probe results from stdin — the site-keyed records the
    // rendered probe would emit when run remotely.
    let mut stdin_buf = String::new();
    std::io::stdin()
        .read_to_string(&mut stdin_buf)
        .map_err(|e| format!("reading stdin: {e}"))?;
    let results = parse_results(&stdin_buf);

    // (3) re-key the site-keyed records to the FactKey-keyed observations `build_plan`
    // consumes (its fold/elision machinery is fact-keyed; only this probe-answer
    // plumbing re-keys — `inv-site-keyed-results`). The probe's `checks` carry each
    // site's resolved fact, so a site-record maps site→fact. CRITICAL (the
    // wrong-concrete firewall, 202 §3): a `site` record's `rc` is the PROBE command's
    // rc (dpkg-query's), NOT the book command's (apt-get's) — for an establish site
    // these are different observables, so it is carried but feeds the fold NOTHING.
    // The site's fold `status` comes ONLY from the transitional `declared-rc` line (the
    // legacy `fold-oror-guard` Query exception, consumed exactly as today's
    // AndOrStatus relaxation); D2's Query class is what will legitimately equate them.
    let by_fact = facts_from_sites(&probe, &results);
    let plan = dorc_plan::build_plan(&book_src, &parsed.value, &cfg.value, &classes, |f| {
        by_fact
            .get(&f)
            .copied()
            .unwrap_or(Observable::verdict_only(Verdict::Unknown))
    });
    print!("{}", plan.render_apply(&book_src, &parsed.value));
    Ok(())
}

/// Re-key the site-keyed [`SiteResults`] to the `FactKey → Observable` map
/// [`dorc_plan::build_plan`] consumes (`inv-site-keyed-results`): for each resolvable
/// site the probe compiled, look up its reported [`Verdict`] (the Effect channel) and
/// its transitional declared-rc (the Status channel), keyed by the site's resolved
/// fact. A site with no reported record folds to `Unknown` ⇒ run (`kFAIL-perform`).
///
/// Two sites sharing a fact (two same-command sites — `inv-site-keyed-results`) write
/// the same fact; last-in-site-order wins, which is sound here (they are the same cell
/// on the same host, so they report the same verdict). The wrong-concrete firewall:
/// the `status` is NEVER the site-record's probe-rc (that is the check command's rc);
/// it is the declared-rc line alone (the legacy Query fold), else `Predicted::Top`.
fn facts_from_sites(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
) -> BTreeMap<dorc_core::FactKey, Observable> {
    let mut by_fact = BTreeMap::new();
    for check in &probe.checks {
        let effect = results
            .verdict
            .get(&check.site)
            .copied()
            .unwrap_or(Verdict::Unknown);
        let status = results
            .declared_rc
            .get(&check.site)
            .map_or(Predicted::Top, |&rc| Predicted::Value(rc));
        by_fact.insert(check.fact, Observable { effect, status });
    }
    by_fact
}

/// The probe results parsed from stdin, keyed by command **site** (the stable
/// `LeafId`, `inv-site-keyed-results`). Two lanes:
/// * `verdict` — the Effect channel: each site's reported three-outcome
///   (`holds`/`absent`/`cant-tell` ⇒ `Converged`/`Diverged`/`Unknown`);
/// * `declared_rc` — the transitional Status lane (the legacy `fold-oror-guard` Query
///   exception): a `declared-rc <site> rc=N` line. Consumed exactly as today's
///   `AndOrStatus` relaxation; never widened (D2's Query class supersedes it).
#[derive(Debug, Default)]
struct SiteResults {
    verdict: BTreeMap<dorc_plan::LeafId, Verdict>,
    declared_rc: BTreeMap<dorc_plan::LeafId, Rc>,
}

/// Parse stdin probe-results into the site-keyed [`SiteResults`]
/// (`inv-site-keyed-results`). Two line forms; blank lines and `#` comments are
/// ignored (so the probe's own `# site …` provenance echo can be piped back), and any
/// unrecognized line is dropped — a site with no record folds to `Unknown` ⇒ run (the
/// `kFAIL-perform` floor; the `garbage-stdin` case pins it):
///
/// * `site <leafid> effect=<holds|absent|cant-tell> rc=<n>` — the records the rendered
///   probe emits (the return channel, 202 §3). `effect` is the Effect channel mapped to
///   a [`Verdict`] (`holds`/`absent`/`cant-tell` ⇒ `Converged`/`Diverged`/`Unknown`).
///   `rc` is the PROBE command's rc — parsed for grammar-validity but **discarded**:
///   it is the check command's status (dpkg-query's), not the book command's, so
///   feeding it to the fold would be a confidently-wrong concrete (the wrong-concrete
///   firewall, 202 §3). It is carried only on the wire; D2's Query class is what will
///   legitimately consume a probe-sourced rc.
/// * `declared-rc <leafid> rc=<n>` — the TRANSITIONAL Status lane (the legacy
///   `fold-oror-guard` Query exception): a probe-sourced rc the fold reads exactly as
///   today's `AndOrStatus` relaxation (`19D`). Never widened.
fn parse_results(input: &str) -> SiteResults {
    let mut out = SiteResults::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        match it.next() {
            Some("site") => {
                let Some(site) = it.next().and_then(parse_site) else {
                    continue; // malformed site id ⇒ drop (⇒ Unknown ⇒ run)
                };
                // `effect=<word>` ⇒ the Effect channel verdict. A missing/garbled
                // effect ⇒ Unknown (the safe direction).
                let verdict = it
                    .find_map(|tok| tok.strip_prefix("effect="))
                    .map_or(Verdict::Unknown, effect_word_to_verdict);
                out.verdict.insert(site, verdict);
                // The trailing `rc=N` is intentionally NOT read into a fold input here
                // (wrong-concrete firewall): a `site` record's rc is the check's rc.
            }
            Some("declared-rc") => {
                let Some(site) = it.next().and_then(parse_site) else {
                    continue;
                };
                if let Some(rc) =
                    it.find_map(|tok| tok.strip_prefix("rc=").and_then(|n| n.parse::<i32>().ok()))
                {
                    out.declared_rc.insert(site, Rc(rc));
                }
            }
            _ => {} // unrecognized line ⇒ drop (kFAIL-perform: no verdict ⇒ run)
        }
    }
    out
}

/// Parse a site-id token (`LeafId`'s `u32`).
fn parse_site(tok: &str) -> Option<dorc_plan::LeafId> {
    tok.parse::<u32>().ok().map(dorc_plan::LeafId)
}

/// Map the probe's three-outcome `effect=` word to a [`Verdict`] (the existing
/// `oracle_probe` convention, 202 §3): `holds ⇒ Converged`, `absent ⇒ Diverged`,
/// anything else (`cant-tell` / garbled) ⇒ `Unknown` (the safe direction).
fn effect_word_to_verdict(word: &str) -> Verdict {
    match word {
        "holds" => Verdict::Converged,
        "absent" => Verdict::Diverged,
        _ => Verdict::Unknown,
    }
}

/// Print a stage's diagnostics to stderr (keeping stdout = probe + apply).
fn report(stage: &str, diags: &[dorc_core::Diagnostic]) {
    for d in diags {
        eprintln!("{stage}: {}: {}", d.code.0, d.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{EntityRef, FactKey, Interner, KindId, OpaqueToken, SelectorId};
    use dorc_plan::{LeafId, ProbeCheck, ProbePlan};

    fn pkg(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("installed")),
        }
    }

    #[test]
    fn parse_results_maps_three_outcome_and_declared_rc() {
        // The site lane maps holds/absent/cant-tell to the Effect verdict; the
        // transitional declared-rc lane carries a probe-sourced rc, site-keyed.
        let r = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n\
             site 2 effect=cant-tell rc=2\ndeclared-rc 0 rc=0\n",
        );
        assert_eq!(r.verdict.get(&LeafId(0)), Some(&Verdict::Converged));
        assert_eq!(r.verdict.get(&LeafId(1)), Some(&Verdict::Diverged));
        assert_eq!(r.verdict.get(&LeafId(2)), Some(&Verdict::Unknown));
        assert_eq!(r.declared_rc.get(&LeafId(0)), Some(&Rc(0)));
        assert!(
            !r.declared_rc.contains_key(&LeafId(1)),
            "no declared-rc for site 1"
        );
    }

    #[test]
    fn parse_results_drops_garbage_kfail_perform() {
        // Unrecognized / malformed lines are dropped (⇒ Unknown ⇒ run). Pins the
        // garbage-stdin behavior at the unit layer (`kFAIL-perform`).
        let r = parse_results(
            "this is not a record\nsite notanumber effect=holds\n\
             site 0 garbled-no-effect\ndeclared-rc xyz rc=bad\n# a comment\n",
        );
        assert!(r.declared_rc.is_empty(), "garbled declared-rc dropped");
        // `site 0 garbled-no-effect` parses the id but no effect= ⇒ Unknown (safe).
        assert_eq!(r.verdict.get(&LeafId(0)), Some(&Verdict::Unknown));
        // `site notanumber` ⇒ no id ⇒ dropped entirely.
        assert_eq!(r.verdict.len(), 1, "only the id-parseable site landed");
    }

    #[test]
    fn firewall_site_record_rc_never_becomes_fold_status() {
        // THE wrong-concrete firewall (202 §3): a `site` record's rc is the CHECK
        // command's rc, NOT the book command's. It must NEVER reach the fold's Status
        // channel. Here site 0 reports `holds rc=0` but declares NO declared-rc line ⇒
        // the re-keyed Observable's status MUST be Top (undeclared), never Value(0).
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = ProbePlan {
            checks: vec![ProbeCheck {
                site: LeafId(0),
                fact,
                sh: "{ :; }".to_string(),
            }],
            unresolvable: vec![],
        };
        let results = parse_results("site 0 effect=holds rc=0\n");
        let by_fact = facts_from_sites(&probe, &results);
        let obs = by_fact
            .get(&fact)
            .copied()
            .expect("the site's fact is keyed");
        assert_eq!(
            obs.effect,
            Verdict::Converged,
            "Effect channel = the reported verdict"
        );
        assert_eq!(
            obs.status,
            Predicted::Top,
            "the site-record rc (check's rc) must NOT become fold status — only declared-rc does"
        );
    }

    #[test]
    fn declared_rc_lane_feeds_fold_status() {
        // The legacy Query exception (fold-oror-guard): the transitional declared-rc
        // line — and ONLY it — supplies a site's fold Status. Contrast the firewall test.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = ProbePlan {
            checks: vec![ProbeCheck {
                site: LeafId(0),
                fact,
                sh: "{ :; }".to_string(),
            }],
            unresolvable: vec![],
        };
        let results = parse_results("site 0 effect=holds rc=0\ndeclared-rc 0 rc=0\n");
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Value(Rc(0)),
            "declared-rc 0 supplies the fold status (the AndOrStatus relaxation seam)"
        );
    }
}
