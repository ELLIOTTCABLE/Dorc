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

use dorc_core::{Interner, Observable, Predicted, Rc, Severity, Verdict};

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
    /// `--debug-argv` (gate-5 / cm-2): emit the engine's per-site resolved argv to stderr,
    /// then proceed normally — a cli-edge readout the e2e argv-echo differential consumes.
    debug_argv: bool,
}

/// Minimal hand-rolled parsing (no `clap` dep): `--book=PATH` / `--book PATH`, and
/// `-o PATH` / `-oPATH` / `--oracle PATH` (repeatable); `--debug-argv` (gate-5 readout).
fn parse_args() -> Result<Args, String> {
    let mut book: Option<String> = None;
    let mut oracles = Vec::new();
    let mut debug_argv = false;
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
        } else if arg == "--debug-argv" {
            debug_argv = true;
        } else if arg == "-h" || arg == "--help" {
            return Err(USAGE.to_string());
        } else {
            return Err(format!("unexpected argument {arg:?}; {USAGE}"));
        }
    }
    Ok(Args {
        book: book.ok_or(USAGE)?,
        oracles,
        debug_argv,
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
    // site's resolved fact + its `site_kind`, so a site-record maps site→fact AND the
    // firewall knows whether the rc is fold-usable. CRITICAL (the wrong-concrete
    // firewall, 202 §3 / task-D2): a record's `rc` feeds the fold's Status ONLY for a
    // VALID Query-class site (the guard's own rc); an establish site's rc is the PROBE
    // command's (dpkg-query's), NOT the mutator's, so it feeds the fold NOTHING.
    let by_fact = facts_from_sites(&probe, &results);
    let plan = dorc_plan::build_plan(&book_src, &parsed.value, &cfg.value, &classes, |f| {
        by_fact
            .get(&f)
            .copied()
            .unwrap_or(Observable::verdict_only(Verdict::Unknown))
    });

    // gate-5 (cm-2 argv-echo differential): per-site resolved argv to stderr, behind the flag.
    if args.debug_argv {
        emit_debug_argv(&plan, &cfg.value, &value, &interner);
    }

    print!("{}", plan.render_apply(&book_src, &parsed.value));
    Ok(())
}

/// gate-5 / cm-2 readout: per command site, emit `argv <leafid> <word|TOP per word>` on
/// stderr (a resolved literal verbatim, an unresolved word `TOP`). The leaf-ids are the
/// plan's own ([`dorc_plan::Step::leaf`]) — the same span-sorted space the probe records
/// share (`inv-site-keyed-results`), so `argv N` keys to the same site as `site N`. The
/// argv is the book-side value-flow ([`dorc_analysis::value::ValueFlow::argv_values`]),
/// keyed by `CfgNodeId` (mapped back from the leaf's `AstId`). Cli-edge only.
fn emit_debug_argv(
    plan: &dorc_plan::Plan,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    interner: &Interner,
) {
    use dorc_analysis::value::ValueOf;
    // AstId → CfgNodeId for Command nodes (argv_values is keyed by CfgNodeId; the plan
    // step carries the AstId). One CfgNode per command AstId in the modeled subset.
    let node_of_ast: BTreeMap<dorc_core::AstId, dorc_analysis::cfg::CfgNodeId> = cfg
        .iter()
        .filter(|(_, n)| n.kind == dorc_analysis::cfg::CfgNodeKind::Command)
        .map(|(id, n)| (n.ast, id))
        .collect();
    for step in &plan.steps {
        let Some(&node) = node_of_ast.get(&step.ast) else {
            continue;
        };
        let words: Vec<String> = value
            .argv_values(node)
            .into_iter()
            .map(|w| match w {
                ValueOf::Literal(sym) => interner.resolve(sym).to_string(),
                ValueOf::Top => "TOP".to_string(),
            })
            .collect();
        eprintln!("argv {} {}", step.leaf.0, words.join(" "));
    }
}

/// Re-key the site-keyed [`SiteResults`] to the `FactKey → Observable` map
/// [`dorc_plan::build_plan`] consumes (`inv-site-keyed-results`): for each resolvable
/// site the probe compiled, look up its reported [`Verdict`] (the Effect channel) and
/// — gated by the wrong-concrete firewall — its rc (the Status channel), keyed by the
/// site's resolved fact. A site with no reported record folds to `Unknown` ⇒ run
/// (`kFAIL-perform`).
///
/// THE WRONG-CONCRETE FIREWALL, Query-only (202 §3 / 20C §7 / task-D2 — the heart of
/// the task): a record's `rc` feeds the fold's Status channel ONLY for a Query-class
/// site that passed rule-query-validity. The asymmetry is load-bearing and
/// disaster-class if wrong:
/// * an **establish** site's record-rc is the PROBE command's rc (`dpkg-query`'s), NOT
///   the mutator's (`apt-get`'s) — feeding it would be a confidently-wrong concrete, so
///   its status stays `Predicted::Top` UNCONDITIONALLY (the check's rc is never the
///   mutator's rc);
/// * a **valid Query** site's record-rc IS the guard's own rc (`command -v`'s) — the
///   exact value the `&&`/`||`/`if`/errexit consumer reads — so it feeds Status;
/// * an **invalid Query** site (a mutator/opaque reached it from entry) has a stale
///   resting rc, so its status also stays `Predicted::Top` ⇒ the guard runs for real.
///
/// Two sites sharing a fact (two same-command sites — `inv-site-keyed-results`) write
/// the same fact; last-in-site-order wins, which is sound here (they are the same cell
/// on the same host, so they report the same verdict).
fn facts_from_sites(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
) -> BTreeMap<dorc_core::FactKey, Observable> {
    use dorc_plan::ProbeSiteKind;
    let mut by_fact = BTreeMap::new();
    for check in &probe.checks {
        let record = results.records.get(&check.site);
        let effect = record.map_or(Verdict::Unknown, |r| r.verdict);
        // The firewall: only a VALID Query site's rc is fold-usable as Status.
        let status = match check.site_kind {
            ProbeSiteKind::Query { valid: true } => {
                record.map_or(Predicted::Top, |r| Predicted::Value(r.rc))
            }
            // Establish site (check's rc, not the mutator's) OR an invalid Query
            // (stale resting rc) ⇒ withhold the rc, status stays ⊤.
            ProbeSiteKind::Establish | ProbeSiteKind::Query { valid: false } => Predicted::Top,
        };
        by_fact.insert(check.fact, Observable { effect, status });
    }
    by_fact
}

/// The probe results parsed from stdin, keyed by command **site** (the stable
/// `LeafId`, `inv-site-keyed-results`). One record per site: the reported Effect
/// [`Verdict`] plus the raw probe-command rc carried alongside it. Whether that rc is
/// fold-usable is the FIREWALL's decision ([`facts_from_sites`]), not the parser's —
/// the parser faithfully carries what the probe reported (`inv-superposition`: the
/// wire transports the observed rc; the phased caller decides which channel, if any,
/// it feeds).
#[derive(Debug, Default)]
struct SiteResults {
    records: BTreeMap<dorc_plan::LeafId, SiteRecord>,
}

/// One site's reported observation: the Effect-channel [`Verdict`] and the raw
/// probe-command exit status.
#[derive(Debug, Clone, Copy)]
struct SiteRecord {
    verdict: Verdict,
    rc: Rc,
}

/// Parse stdin probe-results into the site-keyed [`SiteResults`]
/// (`inv-site-keyed-results`). One line form; blank lines and `#` comments are ignored
/// (so the probe's own `# site …` provenance echo can be piped back), and any
/// unrecognized line is dropped — a site with no record folds to `Unknown` ⇒ run (the
/// `kFAIL-perform` floor; the `garbage-stdin` case pins it):
///
/// * `site <leafid> effect=<holds|absent|cant-tell> rc=<n>` — the records the rendered
///   probe emits (the return channel, 202 §3). `effect` is the Effect channel mapped to
///   a [`Verdict`] (`holds`/`absent`/`cant-tell` ⇒ `Converged`/`Diverged`/`Unknown`).
///   `rc` is the raw probe-command status, carried on the wire; the FIREWALL
///   ([`facts_from_sites`]) decides whether it is fold-usable (only for a valid
///   Query-class site). A missing/garbled `rc` defaults to `Rc(0)` for carriage but is
///   irrelevant unless the firewall admits it.
///
/// (The transitional `declared-rc <leafid> rc=N` lane — the 19I §2 rc-injection
/// mechanism — is DEAD as of task-D2: a Query site's own `rc=` carries the fold rc now.)
fn parse_results(input: &str) -> SiteResults {
    let mut out = SiteResults::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        if it.next() != Some("site") {
            continue; // unrecognized line ⇒ drop (kFAIL-perform: no verdict ⇒ run)
        }
        let Some(site) = it.next().and_then(parse_site) else {
            continue; // malformed site id ⇒ drop (⇒ Unknown ⇒ run)
        };
        // The remaining tokens carry `effect=<word>` and `rc=<n>` in any order. A
        // missing/garbled `effect` ⇒ Unknown (the safe direction); a missing/garbled
        // `rc` ⇒ 0 (carried, but irrelevant unless the firewall admits it for a valid
        // Query — and a Query reporting no rc is degenerate).
        let mut verdict = Verdict::Unknown;
        let mut rc = Rc(0);
        for tok in it {
            if let Some(w) = tok.strip_prefix("effect=") {
                verdict = effect_word_to_verdict(w);
            } else if let Some(n) = tok.strip_prefix("rc=").and_then(|n| n.parse::<i32>().ok()) {
                rc = Rc(n);
            }
        }
        out.records.insert(site, SiteRecord { verdict, rc });
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
///
/// Format `<stage>: <severity>[<code>]: <message>` — the severity word is load-bearing:
/// the e2e gate-3 floor (20B §2) keys on the `error[` shape (an Error fails a case unless
/// declared in `expected-diagnostics`; warnings stay free-form). I/O-edge formatting only.
fn report(stage: &str, diags: &[dorc_core::Diagnostic]) {
    for d in diags {
        let sev = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        };
        eprintln!("{stage}: {sev}[{}]: {}", d.code.0, d.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{EntityRef, FactKey, Interner, KindId, OpaqueToken, SelectorId};
    use dorc_plan::{LeafId, ProbeCheck, ProbePlan, ProbeSiteKind};

    fn pkg(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("installed")),
        }
    }

    fn tool(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("present")),
        }
    }

    /// A one-check probe over `fact` with the given site-kind (the firewall input).
    fn probe1(fact: FactKey, site_kind: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![ProbeCheck {
                site: LeafId(0),
                fact,
                site_kind,
                sh: "{ :; }".to_string(),
            }],
            unresolvable: vec![],
        }
    }

    #[test]
    fn parse_results_maps_three_outcome_and_carries_rc() {
        // The record maps holds/absent/cant-tell to the Effect verdict and carries the
        // raw rc on the wire (whether it is fold-usable is the firewall's call).
        let r = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\nsite 2 effect=cant-tell rc=2\n",
        );
        assert_eq!(
            r.records.get(&LeafId(0)).map(|x| x.verdict),
            Some(Verdict::Converged)
        );
        assert_eq!(
            r.records.get(&LeafId(1)).map(|x| x.verdict),
            Some(Verdict::Diverged)
        );
        assert_eq!(
            r.records.get(&LeafId(2)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        assert_eq!(r.records.get(&LeafId(0)).map(|x| x.rc), Some(Rc(0)));
        assert_eq!(r.records.get(&LeafId(1)).map(|x| x.rc), Some(Rc(1)));
    }

    #[test]
    fn parse_results_drops_garbage_kfail_perform() {
        // Unrecognized / malformed lines are dropped (⇒ Unknown ⇒ run). Pins the
        // garbage-stdin behavior at the unit layer (`kFAIL-perform`). The dead
        // `declared-rc` lane is now just an unrecognized line ⇒ dropped.
        let r = parse_results(
            "this is not a record\nsite notanumber effect=holds\n\
             site 0 garbled-no-effect\ndeclared-rc 0 rc=0\n# a comment\n",
        );
        // `site 0 garbled-no-effect` parses the id but no effect= ⇒ Unknown (safe), rc 0.
        assert_eq!(
            r.records.get(&LeafId(0)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        // `site notanumber` ⇒ no id ⇒ dropped; the dead `declared-rc` line ⇒ dropped.
        assert_eq!(r.records.len(), 1, "only the id-parseable site landed");
    }

    #[test]
    fn firewall_establish_site_rc_never_becomes_fold_status() {
        // THE wrong-concrete firewall, direction 1 (202 §3 / task-D2): an ESTABLISH
        // site's record-rc is the CHECK command's rc (dpkg-query's), NOT the mutator's.
        // It must NEVER reach the fold's Status — status stays Top unconditionally,
        // even though the record carries `rc=0`.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Establish);
        let results = parse_results("site 0 effect=holds rc=0\n");
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(obs.effect, Verdict::Converged, "Effect = reported verdict");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an establish site's probe-rc must NOT become fold status (the disaster class)"
        );
    }

    #[test]
    fn firewall_valid_query_site_rc_feeds_fold_status() {
        // THE wrong-concrete firewall, direction 2 (task-D2): a VALID Query site's
        // record-rc IS the guard's own rc ⇒ it feeds the fold's Status exactly. This is
        // the relaxation that replaces the dead `declared-rc` lane.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Query { valid: true });
        let results = parse_results("site 0 effect=holds rc=0\n");
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Value(Rc(0)),
            "a valid Query guard's own rc supplies the fold Status"
        );
        // A non-zero guard rc (nginx absent) carries through identically (Exit(n) path).
        let results = parse_results("site 0 effect=absent rc=1\n");
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .unwrap();
        assert_eq!(obs.status, Predicted::Value(Rc(1)), "rc 1 carries through");
    }

    #[test]
    fn firewall_invalid_query_site_rc_withheld() {
        // THE wrong-concrete firewall, direction 3 (rule-query-validity, 205 §2): an
        // INVALID Query site (a mutator/opaque reached it from entry) has a stale
        // resting rc ⇒ status stays Top even though the record carries `rc=0` ⇒ the
        // guard runs for real at apply. The bit is the ENGINE's (classify); the cli only
        // honors it.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Query { valid: false });
        let results = parse_results("site 0 effect=holds rc=0\n");
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an INVALID Query guard's rc is stale ⇒ withheld (status Top ⇒ runs for real)"
        );
    }
}
