//! `dorc` — the thin spike CLI: the apply-2 round-trip over real files, as a
//! multi-mode plan/apply surface (ui-A, ru-25 / ru-20 ui-3).
//!
//! Reads a book + oracle files, runs the pure analyzer kernel, and emits one of the
//! engine's distinct user-facing behavioral modes. No executor — it *compiles* a probe
//! and an apply; it runs neither. The simulated host's answers arrive on stdin (in a
//! real deployment those come from running the probe on the host).
//!
//! ```text
//! usage: dorc [<mode>] --book=<book.sh> [-o <oracle.sh>]... [--debug-argv]
//!   modes:
//!     probe      emit the read-only probe artifact (phase 1) to stdout; reads no stdin
//!     plan       PREVIEW (ru-20 ui-3): the eliding-apply to stdout, PLUS the why-lens +
//!                diagnostics doubly-emitted to stderr (the cited-sections render surface)
//!     apply      the byte-floored, receipt-free shippable apply artifact to stdout;
//!                stderr carries ONLY error-severity diagnostics + the decision-digest
//!     <none>     the legacy round-trip: probe THEN apply on stdout, full disclosure on
//!                stderr — the shape the e2e harness drives (kept verbatim, do not break)
//!   stdin : probe results (plan/apply/round-trip), one per line —
//!           `site <leafid> effect=<holds|absent|cant-tell> rc=<n>`
//!   stdout: the selected mode's artifact(s); stderr: diagnostics / why-lens / digest
//!           + the plan-summary (every plan-building mode; the yardstick's metric):
//!           `dorc: plan-summary sites=<N> elide=<E> omit=<O> guard=<G> run=<R>`
//!           where sites == elide+omit+guard+run; elide = provably-skipped lines,
//!           omit = fold-dead branches, guard = 0 until the Stage-3 guard tier, run
//!           = the rest. Stable grammar (a parse target — plans/240 Stage-1 yardstick).
//! ```
//!
//! rec-1 TWO SURFACES (ru-12 + ru-20, spike/CLAUDE.md): the shipped `.sh` artifact on
//! stdout is byte-floored and receipt-free — `plan` and `apply` emit BYTE-IDENTICAL
//! apply bytes. The only difference is the RENDER surface (stderr): `plan` overlays the
//! per-line why-lens + advisory disclosure there; `apply` (the off-ramp) suppresses the
//! advisory plane, keeping only the error floor + digest. The why-lens is never woven
//! into the artifact bytes in any mode.
//!
//! Round-20 task-D1 (the WIRE — `inv-site-keyed-results`): the probe is a real,
//! self-reporting artifact; its results-records are keyed by command **site** (the
//! stable `LeafId`), not by fact. The simulated host's answers (the e2e
//! `probe-results.txt`, a stand-in for running the rendered probe remotely) are now
//! the site-keyed records the probe itself emits.
//!
//! I/O edge: `inv-determinism` exempts `cli`; the analyzer kernel it calls is pure.
//! Diagnostics go to stderr so stdout stays the artifact. The mode dispatch is a thin
//! driver over ONE pipeline call ([`analyze`]) — no kernel logic moves here (the
//! thin-driver mandate, crates/cli/CLAUDE.md).

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

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::process::ExitCode;

use dorc_core::{
    Interner, Observable, OutClaim, Predicted, ProvArena, Rc, Severity, Symbol, Verdict,
};

const USAGE: &str =
    "usage: dorc [probe|plan|apply] --book=<book.sh> [-o <oracle.sh>]... [--debug-argv]";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("dorc: {msg}");
            ExitCode::FAILURE
        }
    }
}

/// Which user-facing behavioral mode of the core to drive (ui-A — a fair-shape CLI over
/// the core invocation modes, NOT flag-complete; ru-25). Each maps to one of the engine's
/// distinct surfaces; `RoundTrip` is the legacy bare-flag invocation the e2e harness drives
/// (kept so the corpus stays green without a harness rewrite — the least-disruptive path).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// `dorc probe …`: emit ONLY the read-only probe artifact (round-trip phase 1). Reads no
    /// stdin (there are no results yet — this is what you ship to the host to GET them).
    Probe,
    /// `dorc plan …`: the human-facing PREVIEW (ru-20 ui-3 / DESIGN approach-3 "still as a
    /// simple shell-script"). Emits the eliding-apply to stdout AND doubly-emits the why-lens
    /// + diagnostics to stderr — the cited-sections render surface.
    Plan,
    /// `dorc apply …`: the byte-floored, receipt-free shippable artifact (rec-1). Emits the
    /// SAME apply bytes as `plan` to stdout, but the stderr render surface carries only the
    /// error floor + the decision-digest (no why-lens, no advisory notes).
    Apply,
    /// No mode token: the legacy round-trip (probe THEN apply on stdout, full disclosure on
    /// stderr). The exact shape `e2e/run.sh` drives — preserved verbatim (tc-subcommand-shape).
    RoundTrip,
}

struct Args {
    mode: Mode,
    book: String,
    oracles: Vec<String>,
    /// `--debug-argv` (gate-5 / cm-2): emit the engine's per-site resolved argv to stderr,
    /// then proceed normally — a cli-edge readout the e2e argv-echo differential consumes.
    debug_argv: bool,
    /// `--trust-footprints` (rul24-mode-gate): opt into the survival tier — a converged line
    /// may ELIDE past a RUNNING wall when the wall's authored `touches()` footprint is disjoint
    /// from the line's fact's backing (Stage 2, the golden hill). DEFAULT OFF; not recommended
    /// by hints/docs beyond noting availability. Honest framing (24A §1a-addendum): marketing at
    /// best (the admin chose the danger), theatre at worst (everyone enables it) — demanded
    /// anyway as the non-vacuous CYA. When off, the footprints are never even lifted (TC-1).
    trust_footprints: bool,
}

/// Minimal hand-rolled parsing (no `clap` dep): an OPTIONAL leading mode token
/// (`probe`/`plan`/`apply`; absent ⇒ [`Mode::RoundTrip`]), then `--book=PATH` / `--book
/// PATH`, `-o PATH` / `-oPATH` / `--oracle PATH` (repeatable), and `--debug-argv` (gate-5
/// readout). The mode is positional-first ONLY (a bare word after flags is still an
/// error) so the legacy `dorc --book=… < results` invocation parses unchanged.
fn parse_args() -> Result<Args, String> {
    let mut book: Option<String> = None;
    let mut oracles = Vec::new();
    let mut debug_argv = false;
    let mut trust_footprints = false;
    let mut it = std::env::args().skip(1).peekable();

    // A leading bare word (no `-` prefix) selects the mode; anything else ⇒ RoundTrip and
    // the token is left for the flag loop (which rejects an unexpected bare word, as before).
    let mode = match it.peek().map(String::as_str) {
        Some("probe") => {
            it.next();
            Mode::Probe
        }
        Some("plan") => {
            it.next();
            Mode::Plan
        }
        Some("apply") => {
            it.next();
            Mode::Apply
        }
        _ => Mode::RoundTrip,
    };

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
        } else if arg == "--trust-footprints" {
            trust_footprints = true;
        } else if arg == "-h" || arg == "--help" {
            return Err(USAGE.to_string());
        } else {
            return Err(format!("unexpected argument {arg:?}; {USAGE}"));
        }
    }
    Ok(Args {
        mode,
        book: book.ok_or(USAGE)?,
        oracles,
        debug_argv,
        trust_footprints,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "the top-level pipeline driver: lift → analyze → probe → plan → render, one linear sequence with mode-routing; splitting it into sub-drivers would scatter the ONE call-shape the thin-driver mandate keeps here"
)]
fn run() -> Result<(), String> {
    let args = parse_args()?;
    let mut interner = Interner::default();
    let mode = args.mode;
    // rec-1 advisory routing: `plan` and the legacy round-trip overlay the FULL advisory plane
    // on stderr (warnings, notes, the why-lens, the unresolvable readout); `apply` (the
    // off-ramp shippable) suppresses it, keeping only the error floor + digest. `probe`'s
    // stage diagnostics are advisory-or-error like any analysis run. tc-apply-receipt-floor:
    // WHERE this line falls (advisory-suppressed but error-kept, digest-kept) is the
    // load-bearing surface judgment — flagged to the conductor, not silently settled.
    let advisory = !matches!(mode, Mode::Apply);

    // ---- the shared, pure pipeline (one call-shape for every mode — the thin-driver
    // mandate: no mode branches the kernel; only the stdout/stderr ROUTING below differs) ----

    // Lift the oracle files into one shared kind-index.
    let oracle_srcs: Vec<String> = args
        .oracles
        .iter()
        .map(|p| std::fs::read_to_string(p).map_err(|e| format!("reading oracle {p}: {e}")))
        .collect::<Result<_, _>>()?;
    let oracle_refs: Vec<&str> = oracle_srcs.iter().map(String::as_str).collect();
    // The effect-map is derived from the inline check bodies (23D §1 — the check is the
    // oracle); the probe lane (R3) ships the same stripped check bodies per-site.
    let lifted = dorc_oracle::lift(&mut interner, &oracle_refs);
    report_at(advisory, "oracle", &lifted.diags);
    let idx = lifted.value;

    // Lift each oracle's `<provider>__predict` functions into a per-file PredictSet (the
    // real entity-resolution mechanism — the engine threads the book's value-flow
    // through these, never parsing argv itself). Shared interner, so provider symbols
    // match the book's command words (204 seam #2).
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracle_refs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::predict::lift_predicts(&mut interner, src);
            report_at(advisory, "check", &lifted.diags);
            lifted.value
        })
        .collect();

    // Parse + analyze the book (shared interner, so symbols match the oracles).
    let book_src = std::fs::read_to_string(&args.book)
        .map_err(|e| format!("reading book {}: {e}", args.book))?;
    let parsed = dorc_syntax::parse(&book_src);
    report_at(advisory, "parse", &parsed.diags);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    report_at(advisory, "cfg", &cfg.diags);
    // Book-side value-flow: resolve each command-site's argv (constant/variable
    // propagation) — the input entity-resolution consumes (19H §1 / 202 §1).
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
    // The per-run receipts plane (arch-1): give-up causes (`Top(cause)`) and license
    // witnesses land here. EXEMPT — it informs no decision (the `plan::erasability` gate
    // proves the apply/probe artifacts are byte-identical with it stripped); the cli holds it
    // only to emit the decision-digest line and (future) the why-lens.
    let mut arena = ProvArena::new();
    // stage-3 (the why-lens): take the TYPED cmdsub-⊤ disclosures too — `report`/gate-3 consume the
    // LOWERED `diags` (cause-dropped), but the why-lens render reads the `cause` off the typed
    // `Diag`s (`to_legacy` drops it). The arena is shared (the typed diags' causes resolve in it).
    // `kills` (R3 / 24A §3): the kill-bearing leaf set the wall predicate cannot read off the
    // `MustRun` SkipClass alone. Threaded to `build_plan_walled` so a running `apt-get purge`
    // walls downstream, closing the kill gap fd10's establish-only wall left open. `kill_coords`
    // (24E §7): each single-kill node's killed coordinate — the kill-wall coherence comparand
    // (own-killed-coord ⊆ footprint), closing resid-kill-coherence.
    let (classified, why_diags, kills, kill_coords) =
        dorc_analysis::effect::classify_with_why_diags(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &mut interner,
            &mut arena,
        );
    report_at(advisory, "classify", &classified.diags);
    let classes = classified.value;

    // The per-site guard VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c) —
    // ALWAYS-ON (guards are the un-flagged baseline; rul24-mode-gate governs only the survival
    // tier, NOT this). A vouched past-wall establish ships its read-only probe (the witness needs
    // the verdict) and, converged, mints a `Disposition::Guard`.
    let vouches = build_vouches(&oracle_refs, &classes, &value, &mut interner, advisory);

    // The read-only, SELF-REPORTING, site-keyed probe (R3 / 23D §1 — the check IS the oracle):
    // each site ships its provider's stripped `<provider>__predict` invoked with the site's argv.
    // `is_vouched` closes strain-classify-coupling (24C): a vouched past-wall `EstablishWritten`
    // site ships its probe here (at HEAD it would be `skip-unresolvable`).
    let ship = |p, a: &[Symbol]| ship_predict_body(&oracle_srcs, &checks, &interner, p, a);
    let probe =
        dorc_plan::compile_probe(&parsed.value, &cfg.value, &value, &classes, ship, |node| {
            vouches.contains_key(&node)
        });

    // The DERIVATION-probe (24E §2 corr-§2 — the SECOND probe-shipping path, a NEW pipeline
    // stage): under `--trust-footprints`, a wall-candidate whose `touches()` body ESCALATED (it
    // reached a host query the static `evaluate_touches` could not resolve) ships that body into
    // phase-1, runs read-only, and its stdout coord-lines are read back into a `Derived` footprint
    // (merged below, pre-`build_plan_walled`). Lifted for the derivation lane here; the authored
    // lane (`build_survival_footprints`) lifts its own — both pure + cheap, and a clean oracle
    // reports no touches diag either way (fork-s4-compile: a parallel compiler, NOT a `compile_probe`
    // extension — different site-set/body-source/readback, the convergence path left unperturbed).
    let touches_paired: Vec<(&str, dorc_oracle::touches::TouchesSet)> = if args.trust_footprints {
        oracle_refs
            .iter()
            .map(|src| {
                (
                    *src,
                    dorc_oracle::touches::TouchesSet::lift(&mut interner, src).value,
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let derivations = if args.trust_footprints {
        let derive = |p, a: &[Symbol]| ship_touches_body(&touches_paired, &interner, p, a);
        dorc_plan::compile_derivations(&parsed.value, &cfg.value, &value, &classes, &kills, derive)
    } else {
        dorc_plan::DerivationPlan::default()
    };

    // The RESOLVER-probe (24F §3 — the identity CANONICALIZATION lane, a THIRD phase-1 shipping path).
    // Lift the per-kind resolvers + enforce confusability ALWAYS (an oracle-authoring correctness
    // check, flag-independent). The round-trip (coord enumeration + probe compile) is flag-on: for
    // each resolver-bearing coordinate (footprint + backing sides) ship `<kind>.resolve()` to
    // canonicalize its entity; the `resolv` readback builds the `Resolutions` merged pre-survival-walk.
    let kind_resolvers = build_kind_resolvers(
        &oracle_srcs,
        &checks,
        &touches_paired,
        &mut interner,
        advisory,
    );
    let resolver_kinds: BTreeSet<Symbol> = kind_resolvers.resolver_kinds().collect();
    let resolver_coords = if args.trust_footprints && !resolver_kinds.is_empty() {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_resolver_coords(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &resolver_kinds,
            &mut interner,
        )
    } else {
        BTreeSet::new()
    };
    let resolvers = compile_resolvers(&resolver_coords, &kind_resolvers, &oracle_srcs, &interner);

    // `probe` mode stops here: emit the probe artifact and return. It reads no stdin (no
    // results exist yet — this is phase 1, what you ship to GET them), builds no plan, and so
    // emits no apply, no why-lens, no digest (there is no plan/identity-plane to hash —
    // tc-probe-no-digest, flagged). Stage diagnostics above already routed to stderr.
    if mode == Mode::Probe {
        print!("{}", probe.render_sh(&interner));
        print!("{}", derivations.render_sh(&interner)); // 24E §2: rides the SAME phase-1 block
        print!("{}", resolvers.render_sh()); // 24F §3: rides the SAME phase-1 block
        std::io::stdout().flush().ok();
        return Ok(());
    }

    // The round-trip emits the probe FIRST (phase 1 on stdout), then the apply (phase 2)
    // after stdin EOF — the e2e harness splits the two on the `#!/bin/sh` shebang. `plan`
    // and `apply` emit ONLY the apply artifact (the probe is an internal compile there).
    if mode == Mode::RoundTrip {
        print!("{}", probe.render_sh(&interner));
        print!("{}", derivations.render_sh(&interner)); // 24E §2: rides the SAME phase-1 block
        print!("{}", resolvers.render_sh()); // 24F §3: rides the SAME phase-1 block
        std::io::stdout().flush().ok();
    }

    // read the (simulated) probe results from stdin — the site-keyed records the rendered
    // probe would emit when run remotely (the round-trip's return channel).
    let mut stdin_buf = String::new();
    std::io::stdin()
        .read_to_string(&mut stdin_buf)
        .map_err(|e| format!("reading stdin: {e}"))?;
    let results = parse_results(&stdin_buf, &mut interner);

    // re-key the site-keyed records to the FactKey-keyed observations `build_plan`
    // consumes (its fold/elision machinery is fact-keyed; only this probe-answer
    // plumbing re-keys — `inv-site-keyed-results`). The probe's `checks` carry each
    // site's resolved fact + its `site_kind`, so a site-record maps site→fact AND the
    // firewall knows whether the rc is fold-usable. CRITICAL (the wrong-concrete
    // firewall, 202 §3 / task-D2): a record's `rc` feeds the fold's Status ONLY for a
    // VALID Query-class site (the guard's own rc); an establish site's rc is the PROBE
    // command's (dpkg-query's), NOT the mutator's, so it feeds the fold NOTHING.
    let by_fact = facts_from_sites(&probe, &results);

    // The survival tier (Stage 2 / rul24-mode-gate, TC-1): footprints are lifted ONLY under
    // `--trust-footprints` — off ⇒ `None` ⇒ the honest Stage-1 total wall, the data never exists.
    let survival = args.trust_footprints.then(|| {
        let mut fps = build_survival_footprints(
            &oracle_refs,
            &classes,
            &kills,
            &kill_coords,
            &value,
            &cfg.value,
            &parsed.value,
            &mut interner,
            advisory,
        );
        // 24E §2 corr-§2: merge the host-DERIVED footprints (read back from the phase-1
        // derivation-probe's `deriv` coord-records) into the authored set, before the survival
        // walk. An escalated site has NO authored footprint (its static trace ⊤'d), so the two
        // sets are disjoint by construction — no collision.
        merge_derived_footprints(
            &mut fps,
            &derivations,
            &results,
            &classes,
            &kill_coords,
            &mut interner,
            advisory,
        );
        fps
    });
    // 24F §3: build the identity-canonicalization map from the `resolv` readback (both footprint and
    // backing coords canonicalized in the survival walk). Flag-off / no-resolver ⇒ empty ⇒ the
    // token-equality floor (identical to today). §4: each DANGLING coordinate is a loud diagnostic.
    let resolutions = build_resolutions(&resolver_coords, &resolver_kinds, &results, &mut interner);
    report_at(
        advisory,
        "resolve",
        &dangling_diagnostics(&resolutions, &interner),
    );
    let plan = dorc_plan::build_plan_walled(
        &book_src,
        &parsed.value,
        &cfg.value,
        &classes,
        &kills,
        survival.as_ref(),
        args.trust_footprints.then_some(&resolutions),
        &vouches,
        |f| {
            by_fact
                .get(&f)
                .copied()
                .unwrap_or(Observable::verdict_only(Verdict::Unknown))
        },
        &mut arena,
    );

    // q-2 (`dq-site-unresolvable`, the cli-edge readout): a `skip-unresolvable` comment lands
    // in the probe artifact, but nothing reached stderr (`219` q-1.f silent-3). Disclose each
    // probe-unresolvable site's source command as a Note — the apply runs it (`kFAIL-perform`).
    // ADVISORY (Note-severity): the off-ramp `apply` mode suppresses it; `plan`/round-trip show
    // it (the ui-3 cited-disclosure surface). The apply still RUNS the site either way, so no
    // correctness rides on this readout — it is purely the render surface (rec-1).
    report_at(
        advisory,
        "probe",
        &unresolvable_diagnostics(&probe, &plan, &parsed.value, &book_src, &mut interner),
    );

    // stage-3 (the why-lens, `22D` §1): the FIRST receipt-READER made user-visible. For each
    // forced-run (never-elided) command whose ⊤ has a wired cause, surface — on the RENDER surface
    // (stderr), at the decision point — "why did this run?", cause-derived + remediation-classed.
    // rec-1 WELD: this is the plan-render surface ONLY; it is NEVER woven into the byte-floored
    // `.sh` artifact on stdout (the artifact stays receipt-free). The off-ramp `apply` mode
    // suppresses it (advisory); `plan` + round-trip emit it (ru-20 ui-3: "doubly-emit cited
    // sections + their warnings to the console").
    if advisory {
        emit_why_lens(&why_diags, &arena, &book_src);
        // Stage 2 co-primary (rul24-divergence-is-the-game / TC-3): every SURVIVED elision names,
        // on this same why-lens lane, which running walls it crossed and whose footprint licensed
        // each crossing. This is the attribution tether under the sharpest claim in the design —
        // a wrong footprint silently under-executes someone else's line, so the render surface
        // must always say whose footprint you trusted. Empty when unflagged (no survivals).
        emit_survival_attribution(&plan, &interner);
        // Stage 3 (rul-guard-license / X-why): every GUARDED site names, on the same lane, the
        // mechanism + its converged-vouch license + the vouching oracle (a render-REFUSED guard
        // discloses the refusal instead). Empty when no site guards.
        emit_guard_attribution(&plan, &parsed.value, &interner);
    }

    // gate-5 (cm-2 argv-echo differential): per-site resolved argv to stderr, behind the flag.
    // Independent of the advisory plane — it is a mechanized readout the harness consumes, not
    // human-facing disclosure, so it fires in any mode when asked (the round-trip is the only
    // caller in-corpus, but `plan --debug-argv` is a legitimate inspection).
    if args.debug_argv {
        emit_debug_argv(&plan, &cfg.value, &value, &interner);
    }

    // arch-1 d-6: the leaf-exact render refuses to elide a leaf whose span can't be safely
    // edited (a heredoc-bearing command — its span covers `<<EOF`, not the body), running it
    // verbatim instead (kFAIL-perform). Surface WHY on stderr (else a converged mutator
    // silently running is invisible); the gate-3 floor requires the case to declare it. These
    // are ERROR-severity, so they cross the floor in EVERY mode (incl. `apply`): the off-ramp
    // must never silently ship an artifact whose render had to refuse a licensed elision.
    let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
    report("render", &refusals);

    // rec-1 / ru-12 BYTE FLOOR: `plan` and `apply` emit BYTE-IDENTICAL apply bytes here — the
    // artifact is receipt-free in both; only the stderr disclosure above differed. The
    // round-trip emits the same bytes as its second shebang block.
    print!("{}", plan.render_apply(&book_src, &parsed.value));

    // plans/240 Stage-1 yardstick: the plan-summary on stderr, alongside the digest below.
    emit_plan_summary(&plan);

    emit_decision_digest(
        &plan,
        &probe,
        &book_src,
        &parsed.value,
        &interner,
        classified.diags,
        refusals,
    );
    Ok(())
}

/// arch-1 decision-digest (`mechanism-decision-digest`, `22A` concl-3): a one-line hash of the
/// canonical IDENTITY plane, emitted on every plan-building run as a cheap always-on drift
/// signal. Receipts cannot move it — it hashes only the identity plane (the `plan::erasability`
/// gate proves that). To stderr (stdout stays the artifact). KEPT even in the receipt-free
/// `apply` mode: the digest is identity-plane, not a receipt. The Error-class diagnostics on the
/// identity plane are the analyzer's accumulated ones (classify) plus the render refusals;
/// warnings/notes are exempt (dropped by the canon).
fn emit_decision_digest(
    plan: &dorc_plan::Plan,
    probe: &dorc_plan::ProbePlan,
    book_src: &str,
    ast: &dorc_syntax::ast::Ast,
    interner: &Interner,
    classify_diags: Vec<dorc_core::Diagnostic>,
    refusals: Vec<dorc_core::Diagnostic>,
) {
    let mut identity_diags = classify_diags;
    identity_diags.extend(refusals);
    eprintln!(
        "dorc: decision-digest {}",
        dorc_plan::erasability::decision_digest(
            plan,
            probe,
            book_src,
            ast,
            interner,
            &identity_diags,
        )
    );
}

/// R3 (23D §1 — the check IS the oracle): resolve the stripped `<provider>__predict` funcdef
/// a probe site ships, given its resolved (provider-word, argv-after-word0). Re-runs the
/// SAME resolution [`dorc_analysis::effect`] used — the FIRST check, in oracle-file order,
/// whose provider matches (through the shared hyphen↔underscore
/// [`map_provider_name`](dorc_oracle::predict::map_provider_name) convention) AND whose own
/// argparse [`evaluate`](dorc_oracle::predict::evaluate)s this argv concretely — then
/// [`strip_predict`](dorc_oracle::predict::strip_predict)s it. Matching the analysis's resolution
/// is load-bearing: the shipped probe must check exactly the fact the analysis decided
/// (a provider with two checks — `apt-get` as `package` and `pkgindex` — resolves per argv,
/// `install …` ⇒ package, `update` ⇒ whichever resolves first). `None` ⇒ no check resolves
/// ⇒ the site is un-shippable ⇒ un-elidable (`kFAIL-perform`).
fn ship_predict_body(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<String> {
    use dorc_oracle::predict::{Resolution, evaluate, map_provider_name, strip_predict};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    for (src, cs) in oracle_srcs.iter().zip(checks) {
        for cp in cs.providers() {
            if map_provider_name(interner.resolve(cp)) != want {
                continue;
            }
            let Some(check) = cs.get(cp) else { continue };
            if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                return Some(strip_predict(src, check, interner));
            }
        }
    }
    None
}

/// Lift the survival footprints (Stage 2 / rul24-mode-gate) — called ONLY on the
/// `--trust-footprints` path (TC-1: the footprint data does not exist unflagged). For each
/// wall-candidate site (an establish-bearing class, or a kill) whose provider declares a
/// `touches()`, trace it over the site's resolved argv and record the emitted footprint —
/// after a **coherence check** (23M / the Stage-2 brief): the site's OWN establish coordinate
/// must be ⊆ its lifted footprint (at-least ⊆ at-most), else the footprint is a loud
/// contradiction and is REFUSED (⇒ the site walls). A ⊤/empty lift, a non-literal argv, or a
/// missing `touches()` all mean "no trustworthy footprint" ⇒ absence from the map ⇒ wall.
///
/// `inv-referent-agnostic`: emitted `kind:entity` fragments are interned into the SAME
/// vocabulary the book/predict analysis uses (one interner) — `package` here is the SAME
/// [`KindId`] a predict annotation minted — never a parallel string-typed universe (24A §1b).
#[expect(
    clippy::too_many_arguments,
    reason = "the cli-edge footprint lift threads the whole compiled context (oracles/classes/kills/kill-coords/value/cfg/ast/interner) + the advisory routing flag; each is a distinct pipeline output, not a bundle-able struct"
)]
fn build_survival_footprints(
    oracle_refs: &[&str],
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    value: &dorc_analysis::value::ValueFlow,
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    interner: &mut Interner,
    advisory: bool,
) -> dorc_plan::TrustedFootprints {
    use dorc_analysis::effect::SkipClass;
    let touches_sets: Vec<dorc_oracle::touches::TouchesSet> = oracle_refs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::touches::TouchesSet::lift(interner, src);
            report_at(advisory, "touches", &lifted.diags);
            lifted.value
        })
        .collect();

    let mut footprints = dorc_plan::TrustedFootprints::new();
    let mut diags = Vec::new();
    for (node, class) in classes {
        // A wall candidate: an establish-bearing class OR a kill. Both now carry their OWN effect
        // coordinate for the coherence check (24E §7: the kill's coord rides `kill_coords`).
        let establish = match class {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        };
        if establish.is_none() && !kills.contains(node) {
            continue; // not a wall candidate (a pure builtin, a Query, an opaque)
        }
        let Some((provider, coords)) =
            resolve_touches_footprint(*node, value, &touches_sets, interner)
        else {
            continue; // no touches / non-literal argv / ⊤ / empty emission ⇒ no footprint ⇒ wall
        };
        // Coherence (establish AND kill sites, 24E §7): the site's OWN effect coordinate — its
        // establish, or its killed cell — must be ⊆ its footprint (at-least ⊆ at-most). A violation
        // is a loud contradiction ⇒ refuse ⇒ wall. Closes resid-kill-coherence: a drifted kill
        // `touches()` (too-narrow footprint omitting the really-killed entity) is now caught.
        if let Some(own) = own_wall_coord(*node, classes, kill_coords)
            && !coords.contains(&own)
        {
            let span = ast.node(cfg.node(*node).ast).span;
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("footprint-incoherent"),
                Some(span),
                "touches() footprint omits this command's own effect coordinate \
                 (at-least ⊄ at-most) — footprint refused, the site walls",
            ));
            continue;
        }
        if let Some(footprint) = dorc_plan::Footprint::authored(provider, coords) {
            footprints.insert(*node, footprint);
        }
    }
    report_at(advisory, "footprint", &diags);
    footprints
}

/// Resolve a wall-candidate site's `touches()` footprint: split its resolved argv into
/// `(provider, operands)` (all must be literal — a ⊤ word ⇒ no footprint), find the provider's
/// touches funcdef (through the shared hyphen↔underscore convention, like the probe), trace it,
/// and intern the emitted coordinates. `None` ⇒ any of: non-literal argv, no matching
/// `touches()`, a ⊤ trace, or an EMPTY emission (no claim = wall).
fn resolve_touches_footprint(
    node: dorc_analysis::cfg::CfgNodeId,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    interner: &mut Interner,
) -> Option<(Symbol, Vec<dorc_plan::EntityCoord>)> {
    use dorc_analysis::value::ValueOf;
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::touches::{TouchesResolution, evaluate_touches};

    let argv = value.argv_values(node);
    let (first, rest) = argv.split_first()?;
    let ValueOf::Literal(provider) = first else {
        return None; // ⊤ command word
    };
    let mut arg_texts = Vec::with_capacity(rest.len());
    for w in rest {
        let ValueOf::Literal(s) = w else {
            return None; // a ⊤ operand ⇒ the argparse cannot resolve ⇒ no footprint
        };
        arg_texts.push(interner.resolve(*s).to_owned());
    }
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();

    let want = map_provider_name(interner.resolve(*provider));
    let coords = touches_sets.iter().find_map(|set| {
        set.providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)
            .and_then(|p| set.get(p))
            .and_then(|touches| match evaluate_touches(touches, &arg_refs) {
                TouchesResolution::Emitted(coords) if !coords.is_empty() => Some(coords),
                // Emitted(empty) = no claim = wall; Top = ⊤ = wall. Both ⇒ no footprint.
                TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
            })
    })?;

    // Intern each opaque `kind:entity` fragment into the shared vocabulary (the fence).
    let entity_coords = coords
        .iter()
        .map(|c| {
            let kind = dorc_core::KindId(interner.intern(&c.kind));
            let entity = match &c.entity {
                Some(text) => {
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(text)))
                }
                None => dorc_core::EntityRef::Singleton,
            };
            dorc_plan::EntityCoord::new(kind, entity)
        })
        .collect();
    Some((*provider, entity_coords))
}

/// The derivation-probe seam (24E §2/§3 — fork-4A: the SAME self-vouch tier as `predict`, no new
/// trust edge): for a wall-candidate site's (provider-word, argv), find the provider's `touches()`
/// funcdef and trace it statically. `Some(DerivationShip)` iff the trace ESCALATED — it ⊤'d
/// specifically on a `NonPrintfCommand` (the body reached a host query the static tracer cannot
/// resolve, e.g. `dpkg -L`), the sanctioned escalation trigger (fork-4B). The body then ships
/// strip-only (`strip_touches`; `<provider>.touches` → `<provider>__touches`), the SAME strip
/// discipline as the probe/guard lanes. `None` for: a statically-resolvable body (`Emitted` — the
/// authored-footprint lane owns it), any OTHER ⊤ (degrade-to-wall, fork-4B — the site runs), an
/// empty emission, or a provider with no touches funcdef. `inv-referent-agnostic`: the operands are
/// resolved for the trace/invocation, never decoded.
fn ship_touches_body(
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
) -> Option<dorc_plan::DerivationShip> {
    use dorc_oracle::predict::{map_provider_name, strip_touches};
    use dorc_oracle::touches::{TouchesResolution, TouchesTop, evaluate_touches};
    let want = map_provider_name(interner.resolve(provider));
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    touches_paired.iter().find_map(|(src, set)| {
        let p = set
            .providers()
            .find(|p| map_provider_name(interner.resolve(*p)) == want)?;
        let touches = set.get(p)?;
        match evaluate_touches(touches, &arg_refs) {
            // The EXPECTED escalation (24E §4): the body reached a host query ⇒ ship it.
            TouchesResolution::Top(TouchesTop::NonPrintfCommand) => {
                Some(dorc_plan::DerivationShip {
                    sh: strip_touches(src, touches, interner),
                    call: format!("{}.touches()", interner.resolve(p)),
                })
            }
            // Static-resolvable, an OTHER ⊤ (degrade-to-wall), or empty ⇒ NOT a derivation.
            TouchesResolution::Emitted(_) | TouchesResolution::Top(_) => None,
        }
    })
}

/// Read back the host-DERIVED footprints (24E §2 corr-§2) and merge them into the survival set.
/// For each escalated [`dorc_plan::ProbeDerivation`], intern its readback `deriv` coordinate lines
/// into the SHARED vocabulary (the 24A §1b fence — `package` here is the SAME [`dorc_core::KindId`]
/// a predict annotation minted), build a `Derived` [`dorc_plan::Footprint`], run the coherence
/// check (the site's own establish coordinate must be ⊆ its footprint — at-least ⊆ at-most; a drift
/// refuses ⇒ wall), and insert it keyed by the site's node. An escalated site with NO readback
/// records ⇒ empty ⇒ wall (silence = wall, kFAIL-safe).
///
/// ALL-OR-NOTHING (24E §4 / the static path's TC-4): a MALFORMED derived coordinate refuses the
/// WHOLE footprint (the site walls) — never silently dropped, because a footprint is an *at-most*
/// claim and dropping a coordinate NARROWS it (⇒ a downstream fact wrongly survives ⇒ under-execute).
///
/// SPIKE-ONLY (ru-26): the `touches-escalated` advisory below makes the static→dynamic boundary
/// visible in the render/differential; it must NOT leak into greenfield as a permanent
/// per-escalation requirement.
fn merge_derived_footprints(
    footprints: &mut dorc_plan::TrustedFootprints,
    derivations: &dorc_plan::DerivationPlan,
    results: &SiteResults,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    interner: &mut Interner,
    advisory: bool,
) {
    let mut diags = Vec::new();
    for d in &derivations.derivations {
        diags.push(dorc_core::Diagnostic::note(
            dorc_core::DiagCode("touches-escalated"),
            None,
            format!(
                "site {}: touches() escalated to host-derivation ({})",
                d.site.0, d.call
            ),
        ));
        let Some(coord_strs) = results.derivations.get(&d.site) else {
            continue; // no readback records ⇒ empty derived footprint ⇒ wall (kFAIL-safe)
        };
        let mut coords = Vec::with_capacity(coord_strs.len());
        let mut malformed = false;
        for line in coord_strs {
            if let Some(c) = intern_coordinate(line, interner) {
                coords.push(c);
            } else {
                malformed = true;
                break;
            }
        }
        if malformed {
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("footprint-incoherent"),
                None,
                "derived touches() emitted a malformed coordinate (not kind:entity) — footprint \
                 refused, the site walls (an at-most claim cannot be partial)"
                    .to_string(),
            ));
            continue;
        }
        // Coherence (establish AND kill sites, 24E §7): the site's OWN effect coordinate — its
        // establish, or its killed cell (from `kill_coords`) — must be ⊆ its derived footprint.
        if let Some(own) = own_wall_coord(d.node, classes, kill_coords)
            && !coords.contains(&own)
        {
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("footprint-incoherent"),
                None,
                "derived touches() footprint omits this command's own effect coordinate \
                 (at-least ⊄ at-most) — footprint refused, the site walls"
                    .to_string(),
            ));
            continue;
        }
        if let Some(fp) = dorc_plan::Footprint::derived(d.provider, coords, d.call.clone()) {
            footprints.insert(d.node, fp);
        }
    }
    report_at(advisory, "derive", &diags);
}

/// Intern one readback `kind:entity` coordinate line into the shared vocabulary (24A §1b fence —
/// split on the FIRST `:`; an empty entity is the kind's singleton). `None` on a malformed line
/// (no `:` / empty kind) — the caller refuses the WHOLE footprint (all-or-nothing).
fn intern_coordinate(line: &str, interner: &mut Interner) -> Option<dorc_plan::EntityCoord> {
    let (kind, entity) = line.split_once(':')?;
    if kind.is_empty() {
        return None;
    }
    let kind = dorc_core::KindId(interner.intern(kind));
    let entity = if entity.is_empty() {
        dorc_core::EntityRef::Singleton
    } else {
        dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(interner.intern(entity)))
    };
    Some(dorc_plan::EntityCoord::new(kind, entity))
}

/// The establish fact a wall-candidate node establishes, if it is an establish class. A kill's
/// coordinate rides the `kill_coords` side-map instead (24E §7).
fn establish_fact_of(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    node: dorc_analysis::cfg::CfgNodeId,
) -> Option<dorc_core::FactKey> {
    use dorc_analysis::effect::SkipClass;
    classes.iter().find_map(|(n, c)| {
        if *n != node {
            return None;
        }
        match c {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => Some(*f),
            _ => None,
        }
    })
}

/// The wall-candidate node's OWN effect coordinate — the coherence comparand (own ⊆ footprint,
/// 24E §7): its establish coordinate (an establish class) OR its killed coordinate (a kill node,
/// from `kill_coords`). `None` for a node with neither (nothing to check coherence against). This
/// unifies the establish-wall check (Stage 2) with the kill-wall check (24E §7) for BOTH the
/// authored and derived footprint lanes.
fn own_wall_coord(
    node: dorc_analysis::cfg::CfgNodeId,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kill_coords: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
) -> Option<dorc_plan::EntityCoord> {
    establish_fact_of(classes, node)
        .or_else(|| kill_coords.get(&node).copied())
        .map(|f| dorc_plan::EntityCoord::new(f.kind, f.entity))
}

/// The per-KIND resolvers (24F §3, corr-kind-keying §10): `<kind>.resolve()` funcdefs lifted per
/// oracle file, combined with CONFUSABILITY enforcement. Resolvers are a SECOND family keyed by KIND
/// (the kind-owner holds the nouns — 23M contribution-vs-identity), NOT per-command role-siblings;
/// the engine looks one up by a coordinate's kind symbol, never its provider.
struct KindResolvers {
    /// Per-file resolver sets (indexed by `by_kind`).
    sets: Vec<dorc_oracle::resolve::ResolverSet>,
    /// The kept, non-conflicting `kind → file-index` map. A kind ABSENT here is resolver-LESS (the
    /// token-equality floor) — whether never declared, or REFUSED for a cross-file duplicate.
    by_kind: BTreeMap<Symbol, usize>,
}

impl KindResolvers {
    /// The resolver-bearing kinds (the engine marks each; a coordinate of such a kind that fails to
    /// resolve degrades to may-alias, §3a).
    fn resolver_kinds(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.by_kind.keys().copied()
    }

    /// The `(file-index, resolver funcdef)` for a kind, if it has a kept resolver.
    fn get(&self, kind: Symbol) -> Option<(usize, &dorc_oracle::predict::Predict)> {
        let idx = *self.by_kind.get(&kind)?;
        self.sets.get(idx)?.get(kind).map(|p| (idx, p))
    }
}

/// Lift the per-kind resolvers + ENFORCE confusability (24F §3 / corr-kind-keying §10 — a LOUD
/// diagnostic, never a silent dud). Two checks: (1) at-most-one-resolver-per-kind — two files
/// declaring `<kind>.resolve()` for the SAME kind ⇒ REFUSE BOTH (the kind stays resolver-less) + an
/// error; (2) a resolver keyed to a name matching a known PROVIDER (a lifted predict/touches
/// command) ⇒ a WARNING (the exact mis-keying the brief itself made — `apt-get.resolve()` would mint
/// identity for a "kind" no coordinate uses). `inv-referent-agnostic`: names compared as interned
/// symbols/strings, never decoded.
fn build_kind_resolvers(
    oracle_srcs: &[String],
    checks: &[dorc_oracle::predict::PredictSet],
    touches_paired: &[(&str, dorc_oracle::touches::TouchesSet)],
    interner: &mut Interner,
    advisory: bool,
) -> KindResolvers {
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::resolve::ResolverSet;

    let sets: Vec<ResolverSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = ResolverSet::lift(interner, src);
            report_at(advisory, "resolve", &lifted.diags);
            lifted.value
        })
        .collect();

    // Every (kind, file-index) declaration, grouped by kind (the same kind in ≥2 files is a conflict).
    let mut per_kind: BTreeMap<Symbol, Vec<usize>> = BTreeMap::new();
    for (idx, set) in sets.iter().enumerate() {
        for kind in set.kinds() {
            per_kind.entry(kind).or_default().push(idx);
        }
    }

    // The known PROVIDER names (predict + touches command words, `_`→`-` normalized) — a resolver
    // keyed to one is the mis-keying we warn on.
    let mut providers: BTreeSet<String> = BTreeSet::new();
    for cs in checks {
        for p in cs.providers() {
            providers.insert(map_provider_name(interner.resolve(p)));
        }
    }
    for (_, ts) in touches_paired {
        for p in ts.providers() {
            providers.insert(map_provider_name(interner.resolve(p)));
        }
    }

    let mut diags = Vec::new();
    let mut by_kind = BTreeMap::new();
    for (kind, files) in per_kind {
        let name = interner.resolve(kind).to_owned();
        if files.len() > 1 {
            diags.push(dorc_core::Diagnostic::error(
                dorc_core::DiagCode("resolver-conflict"),
                None,
                format!(
                    "kind '{name}' has {} resolvers across oracle files — at-most-one-resolver-per-kind \
                     (24F §3): BOTH refused, the kind keeps token-equality (never first-wins-silently)",
                    files.len()
                ),
            ));
            continue; // refuse both ⇒ resolver-less
        }
        if providers.contains(&name) {
            diags.push(dorc_core::Diagnostic::warning(
                dorc_core::DiagCode("resolver-provider-collision"),
                None,
                format!(
                    "resolver '{name}.resolve()' is keyed to a name matching a known COMMAND provider \
                     — resolvers are keyed by KIND, not command (corr-kind-keying §10); this mints \
                     identity for a kind no coordinate may use (a likely mis-key)"
                ),
            ));
            // Kept (it may legitimately match a kind of the same name); the warning surfaces the risk.
        }
        if let Some(&idx) = files.first() {
            by_kind.insert(kind, idx);
        }
    }
    report_at(advisory, "resolve", &diags);
    KindResolvers { sets, by_kind }
}

/// Collect the coordinates that need canonicalization (24F §3): every establish/query BACKING coord
/// and every wall-candidate FOOTPRINT coord whose KIND is resolver-bearing. Deduplicated (resolution
/// is a pure function of `(kind, entity)`) and deterministic (`BTreeSet`). Derived-footprint coords
/// (escalated walls, resolved only post-results) are NOT covered — a resolver+derived combination is
/// a second round-trip, deferred (noted `resid-resolve-derived`).
fn collect_resolver_coords(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    resolver_kinds: &BTreeSet<Symbol>,
    interner: &mut Interner,
) -> BTreeSet<dorc_plan::EntityCoord> {
    use dorc_analysis::effect::SkipClass;
    let mut coords = BTreeSet::new();
    let consider = |coord: dorc_plan::EntityCoord, coords: &mut BTreeSet<_>| {
        if resolver_kinds.contains(&coord.kind().0) {
            coords.insert(coord);
        }
    };
    for (node, class) in classes {
        // Backing coords: the cell each establish/query site is about.
        if let SkipClass::EstablishAmbient(f)
        | SkipClass::EstablishWritten(f)
        | SkipClass::QueryResolvable { fact: f, .. } = class
        {
            consider(dorc_plan::EntityCoord::new(f.kind, f.entity), &mut coords);
        }
        // Footprint coords: a wall-candidate's touches() emissions.
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
        ) || kills.contains(node);
        if is_wall_candidate
            && let Some((_, fp_coords)) =
                resolve_touches_footprint(*node, value, touches_sets, interner)
        {
            for c in fp_coords {
                consider(c, &mut coords);
            }
        }
    }
    coords
}

/// Compile the resolver-probe (24F §3): for each resolver-bearing coordinate, ship its kind's
/// stripped `<kind>__resolve` funcdef + a per-coordinate invocation with the entity. Deterministic
/// (coords arrive `BTreeSet`-ordered). A coord whose kind's resolver cannot be resolved/stripped is
/// dropped (defensive — the kind is in `resolver_kinds` by construction, so this is unreachable, but
/// dropping degrades it to may-alias at readback, the safe direction).
fn compile_resolvers(
    coords: &BTreeSet<dorc_plan::EntityCoord>,
    kind_resolvers: &KindResolvers,
    oracle_srcs: &[String],
    interner: &Interner,
) -> dorc_plan::ResolverPlan {
    use dorc_oracle::predict::strip_resolve;
    let mut probes = Vec::new();
    for coord in coords {
        let kind_sym = coord.kind().0;
        let Some((idx, resolver)) = kind_resolvers.get(kind_sym) else {
            continue;
        };
        let Some(src) = oracle_srcs.get(idx) else {
            continue;
        };
        let entity_text = match coord.entity() {
            dorc_core::EntityRef::Operand(tok) => interner.resolve(tok.0).to_owned(),
            dorc_core::EntityRef::Singleton => String::new(),
        };
        probes.push(dorc_plan::ResolverProbe {
            coord_label: render_coord(*coord, interner),
            kind_label: interner.resolve(kind_sym).to_owned(),
            kind_fn: format!(
                "{}__resolve",
                dorc_oracle::to_funcname_segment(interner.resolve(kind_sym))
            ),
            entity_text,
            sh: strip_resolve(src, resolver, interner),
        });
    }
    dorc_plan::ResolverPlan { probes }
}

/// Build the [`dorc_plan::Resolutions`] map (24F §3) from the resolver-probe readback: mark every
/// resolver-bearing kind, record each `canon`, and flag each `dangling`. A resolver-bearing coord
/// with NO readback record degrades to may-alias at canonicalization (§3a — the safe direction).
/// Interning the canonical form through the SHARED interner keeps it in the one vocabulary (the
/// fence); the engine compares canonical tokens as symbols, never decoding (`inv-referent-agnostic`).
fn build_resolutions(
    coords: &BTreeSet<dorc_plan::EntityCoord>,
    resolver_kinds: &BTreeSet<Symbol>,
    readback: &SiteResults,
    interner: &mut Interner,
) -> dorc_plan::Resolutions {
    let mut resolutions = dorc_plan::Resolutions::none();
    for kind in resolver_kinds {
        resolutions.add_resolver_kind(dorc_core::KindId(*kind));
    }
    for coord in coords {
        let label = render_coord(*coord, interner);
        match readback.resolutions.get(&label) {
            Some(ResolvOutcome::Canonical(canon_text)) => {
                let entity = if canon_text.is_empty() {
                    dorc_core::EntityRef::Singleton
                } else {
                    dorc_core::EntityRef::Operand(dorc_core::OpaqueToken(
                        interner.intern(canon_text),
                    ))
                };
                resolutions.record(*coord, entity);
            }
            // Dangling OR no record ⇒ leave unrecorded ⇒ may-alias at canonicalization (§3a). A
            // dangling is additionally flagged for the loud diagnostic (§4).
            Some(ResolvOutcome::Dangling) => resolutions.record_dangling(*coord),
            None => {}
        }
    }
    resolutions
}

/// The DANGLING-reference diagnostics (24F §4): one loud per-coordinate note for each coordinate the
/// resolver flagged dangling (a reference to a non-existent entity on an enumerable kind — the
/// resolver's natural `dpkg-query -W` non-zero). Turns the third-party-typo case from silent
/// value-loss into a pointed hint; the coordinate ALSO rides the may-alias degrade (§3a). ADVISORY —
/// the apply runs the affected site either way (fail toward run), so no correctness rides on this
/// readout; it is the render surface (rec-1). `inv-referent-agnostic`: the coord label is display.
fn dangling_diagnostics(
    resolutions: &dorc_plan::Resolutions,
    interner: &Interner,
) -> Vec<dorc_core::Diagnostic> {
    resolutions
        .dangling()
        .map(|coord| {
            dorc_core::Diagnostic::note(
                dorc_core::DiagCode("dangling-reference"),
                None,
                format!(
                    "coordinate {} resolved DANGLING — the kind's resolver reports no such entity \
                     (a likely typo / stale name); it degrades to may-alias (the site runs)",
                    render_coord(coord, interner)
                ),
            )
        })
        .collect()
}

/// Lift the per-site GUARD VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c).
/// Called ALWAYS-ON — guards are the un-flagged baseline (rul24-mode-gate governs only the survival
/// tier, NOT this). For each establish-bearing site whose provider authored a verdict function
/// (`<provider>.is_converged`/`.is_diverged`) that REACHES a vouching path over the site's resolved
/// argv (`evaluate_verdict` ⇒ `Vouched`), build a [`dorc_plan::Vouches`] entry: a
/// `ByVouch<VerdictVouch>` carrying the guard emitter's data (the mangled funcname, the strip-only
/// preamble, the invocation, the declared sense, the fact's kind label), keyed by the site's
/// `CfgNodeId`. A `Declined` (unhandled path — hz-refusepath: a refuse path that returns 0
/// vacuously never vouches) or ⊤ (P-topargv: an unpropagatable argv) resolution, or no verdict
/// function, ⇒ absence from the map ⇒ the site never guards (no vouch ⇒ run — the judgment tier the
/// map carries is exactly what [`dorc_plan::GuardLicense::mint`] DEMANDS, TC-tier-2).
///
/// Verdict-lift diagnostics are surfaced AS-IS (inv-top-reject: under-modeling is a loud
/// correctness boundary, never a silent degrade). The Part-A `tc-verdict-return` softening
/// (⊤-reject → warning) is REVERTED (find-return-vouches, 24C): the tracer now models a reached
/// `return N` as a DECLINE and the corpus arity-refuse is spelled in-dialect
/// (`if [ … ]; then return N; fi`), so no corpus verdict body ⊤-rejects — one that still does is
/// genuinely out of dialect and SHOULD fail loudly. A verdict function that fails to lift yields
/// no vouch (the site runs, kFAIL-perform) regardless.
///
/// `inv-referent-agnostic`: the kind label + operands are resolved for the invocation/attribution,
/// never decoded for meaning; the vouch travels the site's own value-flow (the 24A §1b fence).
fn build_vouches(
    oracle_refs: &[&str],
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    value: &dorc_analysis::value::ValueFlow,
    interner: &mut Interner,
    advisory: bool,
) -> dorc_plan::Vouches {
    // The composition lives in `dorc_plan::build_vouches` (the ONE home — the sweep/coverage DSTs
    // share it). This edge only ROUTES the lift diagnostics: surfaced AS-IS (inv-top-reject — the
    // tc-verdict-return softening is reverted, find-return-vouches 24C), so a genuinely
    // out-of-dialect verdict body fails gate-3's error-floor rather than degrading silently.
    let lifted = dorc_plan::build_vouches(oracle_refs, classes, value, interner);
    report_at(advisory, "verdict", &lifted.diags);
    lifted.value
}

/// gate-5 / cm-2 readout: per command site, emit `argv <leafid> <disposition> <word|TOP
/// per word>` on stderr (a resolved literal verbatim, an unresolved word `TOP`). The
/// leaf-ids are the plan's own ([`dorc_plan::Step::leaf`]) — the same span-sorted space the
/// probe records share (`inv-site-keyed-results`), so `argv N` keys to the same site as
/// `site N`. The argv is the book-side value-flow
/// ([`dorc_analysis::value::ValueFlow::argv_values`]), keyed by `CfgNodeId` (mapped back
/// from the leaf's `AstId`). Cli-edge only.
///
/// The `<disposition>` tag (task-O / `tc-gate5-omit`, strain-D3b-fold-vs-gate5): one of
/// `run`/`replace`/`omit`, so gate-5 can SKIP a site the plan does not run. An `Omit`ted or
/// `Replace`d site legitimately never appears in the bare book's argv log when a preceding
/// guard short-circuits it (e.g. a shimmed Query-guard fold) — asserting it ⊆ the log would
/// be a false failure, the exact structural exclusion that confined the fold/omit
/// demonstration to builtin guards (20G §5). Filtering on `run` removes that exclusion
/// without weakening the gate for the sites that DO run.
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
        eprintln!(
            "argv {} {} {}",
            step.leaf.0,
            disposition_tag(&step.disposition),
            words.join(" ")
        );
    }
    // gate-6 `guardcmd` attribution (23A §5): one line per DISTINCT check-command a GUARDED site's
    // verdict body runs (`guardcmd dpkg-query`). The widened dual-rail judge allowlists these as
    // legitimate apply-only lines (the guard's live check runs at apply, absent from the bare
    // book) — never an unrelated one (cf-5). Deterministic (`BTreeSet`, `inv-determinism`).
    let mut guard_cmds: BTreeSet<&str> = BTreeSet::new();
    for step in &plan.steps {
        if let dorc_plan::Disposition::Guard(license) = &step.disposition {
            for c in license.insert().check_cmds() {
                guard_cmds.insert(c.as_str());
            }
        }
    }
    for c in &guard_cmds {
        eprintln!("guardcmd {c}");
    }
}

/// q-2 (`dq-site-unresolvable`): one Note per probe-unresolvable site, naming its source
/// command text. The cli-edge readout of [`dorc_plan::ProbePlan::unresolvable`] — a
/// `skip-unresolvable` comment lands in the probe artifact, but nothing reaches stderr today
/// (`219` q-1.f silent-3). Mirrors the `report()`/`emit_debug_argv` plumbing: the
/// `unresolvable` [`LeafId`]s share the apply plan's span-sorted site space
/// (`inv-site-keyed-results`), so each maps to a [`dorc_plan::Step`]'s `ast`, whose span
/// resolves to the book's source text.
///
/// A site with NO matching step is ASSERTED-UNREACHABLE then SKIPPED (no diagnostic emitted), not
/// named by a bare id. None is expected — every unresolvable site is a runnable command leaf with a
/// plan step (`unresolvable ⊆ plan.steps` by construction). NB (f-7, `224` §10): the legacy form
/// could emit a bare-id/no-span Note here; the migration onto the mandatory-primary-span spine
/// (`21Z` drop-B) replaced that with a skip rather than fabricate a span-less `SiteUnresolvable`.
/// Per human ruling 22-q2 ("shouldn't something unreachable be an ASSERT?") the miss now fires a
/// `debug_assert!(false, …)` — a miss is a Dorc-internal plan/probe divergence, not malformed input.
/// This is the CLI EDGE (not the kernel, so `inv-no-throw` does not formally bind), but it
/// deliberately does NOT release-panic: the reachability claim is OURS, not vouched-hard
/// (never-vouch — so no `unreachable!()`), so release safe-degrades to the skip. The site still RUNS
/// at apply (it is in `unresolvable`), so no disclosure-correctness is lost even if it somehow fired.
fn unresolvable_diagnostics(
    probe: &dorc_plan::ProbePlan,
    plan: &dorc_plan::Plan,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
    interner: &mut Interner,
) -> Vec<dorc_core::Diagnostic> {
    use dorc_core::diag::{Diag, DiagCode, SiteId, SiteUnresolvable};
    let ast_of_leaf: BTreeMap<dorc_plan::LeafId, dorc_core::AstId> =
        plan.steps.iter().map(|s| (s.leaf, s.ast)).collect();
    probe
        .unresolvable
        .iter()
        .filter_map(|&leaf| {
            // A site with no matching step cannot key a span. ASSERTED-UNREACHABLE (human ruling
            // 22-q2): unresolvable ⊆ plan.steps by construction, so a miss is a Dorc-internal
            // plan/probe inconsistency, never malformed book input. debug_assert (not a release
            // panic): this reachability claim is OURS, not vouched-hard (never-vouch), and the same
            // safe-degrade shape as the kernel site keeps a release miss skipping rather than
            // aborting — loud in debug/test/DST, safe fallback (skip) in release.
            let Some(&id) = ast_of_leaf.get(&leaf) else {
                debug_assert!(
                    false,
                    "unresolvable site has no plan step — unresolvable ⊆ plan.steps by \
                     construction (f-7); a hit means the probe/plan site spaces diverged"
                );
                return None;
            };
            let span = ast.node(id).span;
            let text = book_src
                .get(span.lo.0 as usize..span.hi.0 as usize)
                .unwrap_or("<source unavailable>");
            // The migrated `DiagCode::SiteUnresolvable` spine (`22B` §5 worked-1): the SiteId is
            // first-class (a real plan LeafId here), the source command is a referent-agnostic
            // OutClaim, and the suggestion carries a remediation class. Lowered to the legacy
            // stream so `report()`/gate-3 are unchanged.
            let diag = Diag::new(
                DiagCode::SiteUnresolvable(SiteUnresolvable {
                    site: SiteId::leaf(leaf),
                    source_excerpt: OutClaim(interner.intern(text)),
                }),
                span,
            )
            .label("no read-only probe could be shipped for this site")
            .note("the apply runs it unconditionally (kFAIL-perform)")
            .suggest(dorc_core::diag::Suggestion {
                message: "declare a read-only probe for this kind's selector in its oracle"
                    .to_owned(),
                applicability: dorc_core::diag::Applicability::MaybeIncorrect,
                remediation: dorc_core::diag::RemediationClass::AuthorOracle,
            });
            Some(diag.to_legacy(interner))
        })
        .collect()
}

/// plans/240 Stage-1 yardstick: emit the plan-summary — a one-line, greppable, stable-grammar
/// readout of the per-disposition tally (the round's north-star metric, elision frequency) — on
/// stderr, the render surface. rec-1 TWO SURFACES: NEVER woven into the byte-floored `.sh`
/// artifact on stdout. The cli emits it in every plan-building mode (`probe` returns before any
/// plan exists, so it emits none). Shaped `dorc: plan-summary …`, never `<stage>: error[…]`, so
/// the e2e gate-3 stderr floor (keyed on the `error[` shape) ignores it. Counts derive from the
/// Plan value alone (`inv-determinism`).
fn emit_plan_summary(plan: &dorc_plan::Plan) {
    let counts = plan.disposition_counts();
    // 24F §3a: the may-alias fire-rate — converged elisions demoted because a same-kind pair could
    // not be canonicalized (the resolver ⊤'d / dangled / was absent). Surfaced so a SWAMPED count is
    // a finding to REPORT (the resolver is too weak/broken), never a license to silently flip the
    // may-alias default. 0 when no resolver-bearing kind participates (the token-equality floor).
    eprintln!(
        "dorc: plan-summary sites={} elide={} omit={} guard={} run={} may-alias={}",
        counts.sites,
        counts.elide,
        counts.omit,
        counts.guard,
        counts.run,
        plan.survival_report.may_alias_fires(),
    );
}

/// stage-3 (the why-lens render, `22D` §1): surface — on stderr, the RENDER surface — the
/// per-line "why did this command RUN (never elided)?" disclosure for each forced-run command
/// whose ⊤ carries a wired cause. The render + stage-4 dedup is [`why_lens_lines`] (pure,
/// unit-testable); this is just its stderr driver.
///
/// rec-1 WELD (two surfaces): this prints to STDERR only — the plan-render surface. It is NEVER
/// woven into the byte-floored `.sh` artifact on stdout (the artifact stays receipt-free). The
/// line is prefixed `why:` and never `error[`, so the e2e gate-3 stderr-floor (which keys on the
/// `<stage>: error[` shape) ignores it — the why-lens is additive, never a case-failing diagnostic.
fn emit_why_lens(why_diags: &[dorc_core::diag::Diag], arena: &ProvArena, src: &str) {
    for line in why_lens_lines(why_diags, arena, src) {
        eprintln!("why: {line}");
    }
}

/// Stage 2 attribution (TC-3 / rul24-divergence-is-the-game): emit, on the why-lens stderr
/// lane (the `why: ` prefix, alongside the run-cause disclosures — one lens, two directions:
/// why-a-line-runs and why-a-line-survived), one line per SURVIVED elision — naming the
/// surviving site, each running wall it crossed, whose footprint licensed the crossing (the
/// provider and its claimed coordinates), and the backing coordinate proven disjoint. Reads the
/// [`dorc_plan::SurvivalWitness`] the wall walk minted — NEVER recomputes disjointness (the
/// witness IS the attribution). rec-1 WELD: stderr render surface only; the byte-floored `.sh`
/// artifact stays receipt-free (a survived elision's artifact bytes are identical to any other
/// elision's). Never `error[`, so the gate-3 stderr floor ignores it; the `why: ` prefix lets
/// gate-7 (`expected-why`) pin the attribution end-to-end.
fn emit_survival_attribution(plan: &dorc_plan::Plan, interner: &Interner) {
    for step in &plan.steps {
        let dorc_plan::Disposition::Replace(license, _) = &step.disposition else {
            continue;
        };
        let Some(witness) = &license.derivation().survival else {
            continue;
        };
        let crossings: Vec<String> = witness
            .crossings()
            .iter()
            .map(|c| {
                let provider = interner.resolve(c.provider());
                let coords: Vec<String> = c
                    .footprint()
                    .iter()
                    .map(|fc| render_coord(*fc, interner))
                    .collect();
                // 24E §9: name a host-DERIVED footprint's provenance ("DERIVED at probe from
                // <call>"); an authored (static) footprint carries no extra locus.
                let origin = match c.origin() {
                    dorc_plan::FootprintOrigin::Derived { call } => {
                        format!("; DERIVED at probe from {call}")
                    }
                    dorc_plan::FootprintOrigin::Authored => String::new(),
                };
                // 24F §6: name the resolver that canonicalized this crossing's coords ("disjoint
                // AFTER <kind>.resolve()"). The aliasing closure is the sharpest claim in the design,
                // so a survival it licensed must always name whose identity-judgment it trusted.
                let via = c.via_resolver().map_or_else(String::new, |k| {
                    format!(
                        "; disjoint AFTER {}.resolve() canonicalization",
                        interner.resolve(k.0)
                    )
                });
                format!(
                    "wall site {} ({provider} touches {{{}}}{origin}{via})",
                    c.wall_leaf().0,
                    coords.join(" ")
                )
            })
            .collect();
        eprintln!(
            "why: site {} survives+elides past {} — backing {} disjoint (trusted footprint)",
            step.leaf.0,
            crossings.join(", "),
            render_coord(witness.backing(), interner),
        );
    }
}

/// The GUARD why-lane (rul-guard-license / X-why): one `why:` line per guarded site, naming
/// (i) the mechanism (`guard`), (ii) the license (a converged-`vouch`), (iii) the vouching oracle
/// (the fact's kind) — the `guard23-why-attribution` conjoined pattern (`guard && vouch && <kind>`
/// in ONE line). Attribution is the guard-license's whole enforcement story ("we can't prevent, so
/// we attribute" — plans/233 §guard-license); rul-attention-honesty makes it load-bearing (a guard
/// the user can't trace to its licensor is hidden risk). rec-1 WELD: stderr render surface only —
/// the byte-floored artifact carries the inline `# dorc: guard …` comment; this is the disclosure.
/// Never `error[`, so the gate-3 floor ignores it; the `why: ` prefix lets gate-7 pin it.
fn emit_guard_attribution(
    plan: &dorc_plan::Plan,
    ast: &dorc_syntax::ast::Ast,
    interner: &Interner,
) {
    // A render-REFUSED guard (heredoc / non-devnull output redirect) does NOT guard the site — the
    // mutator runs verbatim. rul-attention-honesty: never claim a skip that did not happen; disclose
    // the refusal (gate-7 `refus`) instead of the licensing line.
    let refused = plan.guard_refused_asts(ast);
    for step in &plan.steps {
        let dorc_plan::Disposition::Guard(license) = &step.disposition else {
            continue;
        };
        let kind = interner.resolve(license.fact().kind.0);
        if refused.contains(&step.ast) {
            eprintln!(
                "why: site {} guard refused — the site's structurally-awkward form (a heredoc \
                 body, or a non-`/dev/null` output redirect) would corrupt the artifact or suppress \
                 an admin-spelled side-effect, so the original bytes RUN VERBATIM (kFAIL-perform), \
                 the {kind} converged-vouch notwithstanding",
                step.leaf.0,
            );
        } else {
            eprintln!(
                "why: site {} guard [{kind}] — licensed by a converged-vouch (the {kind} oracle's \
                 authored is_converged); the original bytes survive and the check re-runs live at \
                 apply (kFAIL-perform)",
                step.leaf.0,
            );
        }
    }
}

/// Render a [`dorc_plan::EntityCoord`] as `kind:entity` for the attribution surface (empty
/// entity ⇒ `kind:`, the singleton form). DISPLAY only — resolving an interned symbol for
/// provenance is explicitly permitted; the engine never DECODES it for meaning
/// (`inv-referent-agnostic`).
fn render_coord(coord: dorc_plan::EntityCoord, interner: &Interner) -> String {
    let kind = interner.resolve(coord.kind().0);
    let entity = match coord.entity() {
        dorc_core::EntityRef::Operand(token) => interner.resolve(token.0),
        dorc_core::EntityRef::Singleton => "",
    };
    format!("{kind}:{entity}")
}

/// The why-lens render + stage-4 dedup, factored PURE (the stderr side is [`emit_why_lens`]) so
/// the dedup is unit-testable (`x2-fd1`). For each caused-⊤ diag it renders the "why did this run"
/// line via [`dorc_core::diag::why`], showing a given cause-SITE once.
///
/// stage-4 DEDUP KEY = `(cause, site)`, NOT the cause [`dorc_core::ProvId`] alone (`x2-fd1` fix,
/// `224` §10): under function inlining two call-sites splice the SAME body `AstId` (`inv-leaf-seam`)
/// ⇒ both `CmdsubOperandTop` diags hash-cons to ONE cause `ProvId`. Keying on cause alone collapsed
/// two GENUINELY INDEPENDENT forced runs (suppressing the 2nd `why:` — the over-suppression). They
/// differ by `site` (the stable `site N.M` leaf), so `(cause, site)` keeps them separately disclosed
/// while still deduping a true re-disclosure (same cause AND same site). Tracked in a `Vec` of
/// first-occurrences — `ProvId` is `!Ord` (no `BTreeSet`) and the diags arrive in node order, so
/// first-seen order is deterministic (`inv-determinism`). The only suppression built (no general
/// subsystem — `22D` §1 stage-4).
fn why_lens_lines(
    why_diags: &[dorc_core::diag::Diag],
    arena: &ProvArena,
    src: &str,
) -> Vec<String> {
    let mut shown: Vec<(dorc_core::ProvId, dorc_core::diag::SiteId)> = Vec::new();
    let mut lines = Vec::new();
    for diag in why_diags {
        if let Some(key) = cmdsub_cause_site(diag) {
            if shown.contains(&key) {
                continue; // stage-4: this (cause, site) was already explained — show it once
            }
            shown.push(key);
        }
        if let Some(explanation) = dorc_core::diag::why(diag, arena, src) {
            lines.push(explanation.reason);
        }
    }
    lines
}

/// The stage-4 render-dedup key a why-lens diag carries, if any: `(⊤-cause, site)`. Only a
/// `CmdsubOperandTop` carries a cause at HEAD (stage-1); any other diag returns `None` (the why-lens
/// does not explain it anyway, fd-G), so it never participates in the dedup. The `site` half is what
/// separates two inlined call-sites sharing one cause `ProvId` (`x2-fd1`).
fn cmdsub_cause_site(
    diag: &dorc_core::diag::Diag,
) -> Option<(dorc_core::ProvId, dorc_core::diag::SiteId)> {
    match &diag.code {
        dorc_core::diag::DiagCode::CmdsubOperandTop(p) => p.cause.map(|c| (c, p.site)),
        _ => None,
    }
}

#[cfg(test)]
mod why_lens_dedup_tests {
    //! `x2-fd1` (`22E`, `224` §10): the stage-4 render-dedup must key on `(cause, site)`, not the
    //! cause `ProvId` alone — else two inlined call-sites sharing one body-span cause collapse and
    //! the 2nd forced run's `why:` is wrongly suppressed. The arena hash-conses identical
    //! `(OriginKind, span)` origins (`core::prov` `hash_cons_shares_identical_origins`), so two
    //! `arena.leaf(TopCause, same_span)` calls reproduce the inlined-body cause collision.
    use dorc_core::diag::{CmdsubOperandTop, Diag, DiagCode, OperandPosition, SiteId};
    use dorc_core::{BytePos, LeafId, OriginKind, ProvArena, Span};

    fn cmdsub_top(arena: &mut ProvArena, leaf: u32, body_span: Span) -> Diag {
        let cause = arena.leaf(OriginKind::TopCause, Some(body_span));
        Diag::new(
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: SiteId::leaf(LeafId(leaf)),
                position: OperandPosition::Operand(1),
                cause: Some(cause),
            }),
            Span::new(BytePos(0), BytePos(20)),
        )
    }

    #[test]
    fn two_inlined_sites_sharing_one_cause_both_disclose() {
        // `apt_install "$(curl a)"; apt_install "$(curl b)"`: both calls inline ONE wrapper body ⇒
        // one shared cause ProvId, distinct call-site leaves. (cause, site) keeps BOTH `why:`s; the
        // old cause-alone key suppressed the 2nd (x2-fd1, disclosure-only over-suppression).
        let mut arena = ProvArena::new();
        let body = Span::new(BytePos(11), BytePos(20));
        let diags = [
            cmdsub_top(&mut arena, 3, body),
            cmdsub_top(&mut arena, 7, body),
        ];
        let lines = super::why_lens_lines(&diags, &arena, "apt_install \"$(curl a)\"");
        assert_eq!(
            lines.len(),
            2,
            "two inlined sites sharing one cause must BOTH disclose: {lines:?}"
        );
    }

    #[test]
    fn an_identical_cause_and_site_is_shown_once() {
        // The dedup still FIRES for a true duplicate (same cause AND same site) — the (cause, site)
        // key didn't neuter the stage-4 dedup into a no-op.
        let mut arena = ProvArena::new();
        let body = Span::new(BytePos(11), BytePos(20));
        let diags = [
            cmdsub_top(&mut arena, 3, body),
            cmdsub_top(&mut arena, 3, body),
        ];
        let lines = super::why_lens_lines(&diags, &arena, "apt-get install \"$(date)\"");
        assert_eq!(
            lines.len(),
            1,
            "an identical (cause, site) re-disclosure is shown once: {lines:?}"
        );
    }
}

/// The gate-5 disposition tag for a [`dorc_plan::Disposition`] — `run`/`replace`/`omit`.
/// gate-5 asserts the bare-book argv-echo ONLY for `run` sites: a `replace`d or `omit`ted
/// site is deliberately not in the apply run-set, and a guarded omit may be absent from the
/// BARE book too (a preceding guard short-circuits it), so it must not be asserted ⊆ the
/// log (task-O / strain-D3b-fold-vs-gate5).
fn disposition_tag(disposition: &dorc_plan::Disposition) -> &'static str {
    use dorc_plan::Disposition;
    match disposition {
        Disposition::Run => "run",
        Disposition::Replace(_, _) => "replace",
        Disposition::Omit { .. } => "omit",
        // A guard's ledger tag (gate-6's widened judge reads it — cf-5/cf-6): gate-5 skips it (a
        // guarded site's run-set argv is the check invocation, not the bare book's mutator argv).
        Disposition::Guard(_) => "guard",
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
/// SAME-CELL CONFLICT FLOOR (20I find-6a / item-5): two sites mapping to the SAME cell
/// merge **conservatively** — a per-channel DISAGREEMENT degrades that channel to ⊤
/// (`Verdict::Unknown` for Effect, `Predicted::Top` for the others), NEVER last-write-wins.
/// Normally only one site per cell is resolvable (a same-command re-establish is
/// `EstablishWritten` ⇒ unresolvable ⇒ absent from `checks`, strain-D1-samecell), so this
/// is a defensive floor: it cannot be argued the two records "must agree" (a forged or
/// flaky host could disagree), and the conservative ⊤ folds to run (`kFAIL-perform`) — the
/// only safe resolution of a self-contradicting host. [`merge_observable`] does the join.
fn facts_from_sites(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
) -> BTreeMap<dorc_core::FactKey, Observable> {
    use dorc_plan::ProbeSiteKind;
    let mut by_fact: BTreeMap<dorc_core::FactKey, Observable> = BTreeMap::new();
    for check in &probe.checks {
        // Key the record by (site, member) — a member check (`site N.M`) reads its own
        // sub-record (task-L2 item-4); an ordinary check (`site N`) reads `member: None`.
        let record = results.records.get(&RecordKey {
            site: check.site,
            member: check.member,
        });
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
        // The reserved Stdout/Stderr claims ride into the tuple verbatim (19F §3 shape).
        // INERT this round: nothing emits them, and `consumption_ok` blocks a consumed
        // stdout/stderr UNCONDITIONALLY (16F §3) — never reading the claim value — so a
        // (hypothetical) non-⊤ claim cannot relax that block. The slot is plumbed so a
        // future stdout-producing probe + vouch is a value change, not a representation one.
        let stdout = record.map_or(Predicted::Top, |r| r.stdout);
        let stderr = record.map_or(Predicted::Top, |r| r.stderr);
        let obs = Observable {
            effect,
            status,
            stdout,
            stderr,
        };
        by_fact
            .entry(check.fact)
            .and_modify(|prior| *prior = merge_observable(*prior, obs))
            .or_insert(obs);
    }
    by_fact
}

/// Conservatively merge two [`Observable`]s reported for the SAME cell (20I find-6a /
/// item-5). Per channel: equal values pass through; ANY disagreement degrades the
/// channel to ⊤ (`Verdict::Unknown` for Effect, `Predicted::Top` for status/stdout/
/// stderr). This is the meet toward ⊤ — never last-write-wins — so a self-contradicting
/// host folds to run (`kFAIL-perform`), the only safe resolution. Order-independent
/// (commutative + idempotent): merging in any site order yields the same ⊤-on-conflict.
fn merge_observable(a: Observable, b: Observable) -> Observable {
    Observable {
        effect: if a.effect == b.effect {
            a.effect
        } else {
            Verdict::Unknown
        },
        status: if a.status == b.status {
            a.status
        } else {
            Predicted::Top
        },
        stdout: if a.stdout == b.stdout {
            a.stdout
        } else {
            Predicted::Top
        },
        stderr: if a.stderr == b.stderr {
            a.stderr
        } else {
            Predicted::Top
        },
    }
}

/// A record's key: the command **site** (the stable `LeafId`, `inv-site-keyed-results`)
/// plus an optional MEMBER index (task-L2 item-4): `None` for an ordinary single-fact
/// record (`site N`), `Some(m)` for member `m` of an in-loop Members family (`site N.M`).
/// The probe's [`dorc_plan::ProbePredict`] carries the same `(site, member)` pair, so the
/// bridge ([`facts_from_sites`]) keys a member record back to that member's cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RecordKey {
    site: dorc_plan::LeafId,
    member: Option<u32>,
}

/// The probe results parsed from stdin, keyed by [`RecordKey`] (site, optional member —
/// `inv-site-keyed-results` + task-L2 item-4). One record per (site, member): the reported
/// Effect [`Verdict`] plus the raw probe-command rc carried alongside it. Whether that rc
/// is fold-usable is the FIREWALL's decision ([`facts_from_sites`]), not the parser's —
/// the parser faithfully carries what the probe reported (`inv-superposition`: the wire
/// transports the observed rc; the phased caller decides which channel, if any, it feeds).
#[derive(Debug, Default)]
struct SiteResults {
    records: BTreeMap<RecordKey, SiteRecord>,
    /// The DERIVATION coord-blob lane (24E §5 / fork-s4-coordwire): per escalated wall-site, the
    /// raw `kind:entity` coordinate lines its host-run `touches()` printed (`deriv <leafid>
    /// coord=…`). Demuxed SEPARATELY from the `site` verdict records (a derivation-blob never
    /// collides with a site's `effect=`/`rc=` record — `inv-site-keyed-results`). Read back into a
    /// `Derived` [`dorc_plan::Footprint`] before the survival walk (24E §2 corr-§2).
    derivations: BTreeMap<dorc_plan::LeafId, Vec<String>>,
    /// The RESOLVER canonicalization lane (24F §3): per `kind:entity` coordinate label, the readback
    /// of running its `<kind>.resolve()` host-side — a [`ResolvOutcome`]. Demuxed SEPARATELY from the
    /// verdict + derivation lanes (keyed by the coordinate, not a site — resolution is a pure function
    /// of the coordinate). Read into a [`dorc_plan::Resolutions`] before the survival walk.
    resolutions: BTreeMap<String, ResolvOutcome>,
}

/// One coordinate's resolver readback (24F §3): the canonical form its `<kind>.resolve()` printed, or
/// [`Dangling`](ResolvOutcome::Dangling) — the resolver's natural failure on an enumerable kind (§4,
/// a reference to a non-existent entity), which rides the may-alias degrade + a loud diagnostic.
#[derive(Debug, Clone)]
enum ResolvOutcome {
    /// The resolver printed a canonical form (interned into the shared vocabulary at readback).
    Canonical(String),
    /// The resolver failed (non-zero rc / empty stdout) — a dangling reference (§4) ⇒ may-alias.
    Dangling,
}

/// One site's reported observation: the Effect-channel [`Verdict`], the raw probe-command
/// exit status, and the RESERVED `Stdout`/`Stderr` [`OutClaim`]s (`19F` §3 tuple shape).
/// The out-claims are parsed-and-stored but produce NOTHING this round — the probe never
/// emits `stdout=`/`stderr=`, so they arrive `Predicted::Top` in practice; the slots exist
/// so a future stdout-producing probe is a value-plumbing change, not a grammar change.
#[derive(Debug, Clone, Copy)]
struct SiteRecord {
    verdict: Verdict,
    rc: Rc,
    stdout: Predicted<OutClaim>,
    stderr: Predicted<OutClaim>,
}

/// Parse stdin probe-results into the site-keyed [`SiteResults`]
/// (`inv-site-keyed-results`). One line form; blank lines and `#` comments are ignored
/// (so the probe's own `# site …` provenance echo can be piped back), and any
/// unrecognized line is dropped — a site with no record folds to `Unknown` ⇒ run (the
/// `kFAIL-perform` floor; the `garbage-stdin` case pins it):
///
/// * `site <leafid> effect=<holds|absent|cant-tell> rc=<n> [stdout=<text> stderr=<text>]`
///   — the records the rendered probe emits (the return channel, 202 §3). `effect` is the
///   Effect channel mapped to a [`Verdict`] (`holds`/`absent`/`cant-tell` ⇒
///   `Converged`/`Diverged`/`Unknown`). `rc` is the raw probe-command status, carried on
///   the wire; the FIREWALL ([`facts_from_sites`]) decides whether it is fold-usable (only
///   for a valid Query-class site). A missing/garbled `rc` defaults to `Rc(0)` for
///   carriage but is irrelevant unless the firewall admits it.
///
/// `stdout=`/`stderr=` are RESERVED (`19F` §3 tuple shape): the parser accepts-and-stores
/// them (interning the text into a [`OutClaim`] on the record) but NOTHING produces them —
/// the rendered probe emits no such keys, and the consumed-stdout/stderr gate stays the
/// unconditional block it is regardless. Reserving them means a future stdout-producing
/// probe is a value-plumbing change, not a grammar change. The interner is threaded for
/// this (the `cli` is the I/O edge; `inv-determinism` exempts it).
///
/// (The transitional `declared-rc <leafid> rc=N` lane — the 19I §2 rc-injection
/// mechanism — is DEAD as of task-D2: a Query site's own `rc=` carries the fold rc now.)
fn parse_results(input: &str, interner: &mut Interner) -> SiteResults {
    let mut out = SiteResults::default();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut it = line.split_whitespace();
        match it.next() {
            // 24E §5 (fork-s4-coordwire): `deriv <leafid> coord=<kind:entity>…` — accumulate an
            // escalated wall-site's derived-footprint coordinates, demuxed by leaf-id in its OWN
            // lane (never the `site` verdict records). A malformed leaf ⇒ drop (the site's derived
            // footprint stays empty ⇒ wall, the kFAIL-safe direction).
            Some("deriv") => {
                if let Some(site) = it
                    .next()
                    .and_then(|t| t.parse::<u32>().ok())
                    .map(dorc_plan::LeafId)
                {
                    for tok in it {
                        if let Some(coord) = tok.strip_prefix("coord=") {
                            out.derivations
                                .entry(site)
                                .or_default()
                                .push(coord.to_owned());
                        }
                    }
                }
                continue;
            }
            // 24F §3: `resolv <kind:entity> canon=<canonical>` | `resolv <kind:entity> dangling` —
            // the resolver readback, demuxed by the COORDINATE label in its OWN lane. A malformed
            // line ⇒ drop (the coord stays unrecorded ⇒ may-alias, the kFAIL-safe direction).
            Some("resolv") => {
                if let Some(coord) = it.next() {
                    let outcome = match it.next() {
                        Some("dangling") => Some(ResolvOutcome::Dangling),
                        Some(tok) => tok
                            .strip_prefix("canon=")
                            .map(|c| ResolvOutcome::Canonical(c.to_owned())),
                        None => None,
                    };
                    if let Some(o) = outcome {
                        out.resolutions.insert(coord.to_owned(), o);
                    }
                }
                continue;
            }
            Some("site") => {}
            _ => continue, // unrecognized line ⇒ drop (kFAIL-perform: no verdict ⇒ run)
        }
        let Some(key) = it.next().and_then(parse_site_key) else {
            continue; // malformed site key ⇒ drop (⇒ Unknown ⇒ run)
        };
        // The remaining tokens carry `effect=<word>`, `rc=<n>`, and the reserved
        // `stdout=`/`stderr=` in any order. A missing/garbled `effect` ⇒ Unknown (the safe
        // direction); a missing/garbled `rc` ⇒ 0 (carried, but irrelevant unless the
        // firewall admits it for a valid Query). Absent out-claims stay `Predicted::Top`.
        let mut verdict = Verdict::Unknown;
        let mut rc = Rc(0);
        let mut stdout = Predicted::Top;
        let mut stderr = Predicted::Top;
        for tok in it {
            if let Some(w) = tok.strip_prefix("effect=") {
                verdict = effect_word_to_verdict(w);
            } else if let Some(n) = tok.strip_prefix("rc=").and_then(|n| n.parse::<i32>().ok()) {
                rc = Rc(n);
            } else if let Some(t) = tok.strip_prefix("stdout=") {
                stdout = Predicted::Value(OutClaim(interner.intern(t)));
            } else if let Some(t) = tok.strip_prefix("stderr=") {
                stderr = Predicted::Value(OutClaim(interner.intern(t)));
            }
        }
        out.records.insert(
            key,
            SiteRecord {
                verdict,
                rc,
                stdout,
                stderr,
            },
        );
    }
    out
}

/// Parse a record's site key token (task-L2 item-4): `N` ⇒ `RecordKey { site: N, member:
/// None }`; `N.M` ⇒ `RecordKey { site: N, member: Some(M) }` (member `M` of an in-loop
/// Members family). Both `N` and `M` are `u32`; a non-numeric / malformed token ⇒ `None`
/// (the record is dropped ⇒ that cell folds to Unknown ⇒ run, the kFAIL-perform floor).
fn parse_site_key(tok: &str) -> Option<RecordKey> {
    match tok.split_once('.') {
        Some((leaf, member)) => Some(RecordKey {
            site: dorc_plan::LeafId(leaf.parse::<u32>().ok()?),
            member: Some(member.parse::<u32>().ok()?),
        }),
        None => Some(RecordKey {
            site: dorc_plan::LeafId(tok.parse::<u32>().ok()?),
            member: None,
        }),
    }
}

/// Map the probe's three-outcome `effect=` word to a [`Verdict`] (the probe-record
/// convention, 202 §3): `holds ⇒ Converged`, `absent ⇒ Diverged`,
/// anything else (`cant-tell` / garbled) ⇒ `Unknown` (the safe direction).
fn effect_word_to_verdict(word: &str) -> Verdict {
    match word {
        "holds" => Verdict::Converged,
        "absent" => Verdict::Diverged,
        _ => Verdict::Unknown,
    }
}

/// Advisory-gated [`report`] (rec-1 / tc-apply-receipt-floor): the stderr driver over
/// [`advisory_filter`]. When `advisory` is true, emit every severity (the `plan` /
/// round-trip render surface — the ui-3 cited-disclosure console); when false (the off-ramp
/// `apply` mode), emit ONLY Error-severity diagnostics. The error floor is never suppressed
/// in any mode — a shippable artifact must never hide an error — so `apply` stays
/// receipt-free WITHOUT going blind. The filter is factored PURE (the printing is the I/O
/// edge) so the lone per-severity routing decision rec-1 forces here is unit-testable, the
/// same pure/driver split as [`why_lens_lines`]/[`emit_why_lens`].
fn report_at(advisory: bool, stage: &str, diags: &[dorc_core::Diagnostic]) {
    report(stage, &advisory_filter(advisory, diags));
}

/// The advisory severity-filter (rec-1 / tc-apply-receipt-floor), factored pure for
/// testing. `advisory` ⇒ pass every diagnostic through (the `plan`/round-trip render
/// surface); `!advisory` (the receipt-free `apply` off-ramp) ⇒ keep ONLY Error-severity,
/// dropping warnings + notes. Errors are NEVER dropped — the floor that keeps `apply`
/// honest while receipt-free. Returns owned clones (the call sites are cold — once per
/// pipeline stage — so the copy is irrelevant against the SSH-tunnel cost DESIGN floors on).
fn advisory_filter(advisory: bool, diags: &[dorc_core::Diagnostic]) -> Vec<dorc_core::Diagnostic> {
    if advisory {
        diags.to_vec()
    } else {
        diags
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .cloned()
            .collect()
    }
}

/// Print a stage's diagnostics to stderr (keeping stdout = probe + apply).
///
/// Format `<stage>: <severity>[<code>]: <message>`, then a ` --> <lo>:<hi>` region line when the
/// diagnostic carries a span (the round-22 drop-A fix: the span was previously DROPPED at this
/// one user surface — `21Z` drop-A — so a structured diagnostic's location never reached the
/// user; now it does). The severity word is load-bearing: the e2e gate-3 floor (20B §2) keys on
/// the `error[` shape (an Error fails a case unless declared in `expected-diagnostics`; warnings
/// stay free-form), and the region line never starts with `<stage>: error[`, so it is inert to
/// gate-3. The byte-coordinate form (no source excerpt) is the multi-stage-safe minimum: `report`
/// receives only the diagnostics, not the per-stage source (oracle vs book), so it renders the
/// span coordinates; the source-resolved narrative is [`dorc_core::diag::render_cli`]'s job.
/// I/O-edge formatting only.
fn report(stage: &str, diags: &[dorc_core::Diagnostic]) {
    for d in diags {
        let sev = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        };
        eprintln!("{stage}: {sev}[{}]: {}", d.code.0, d.message);
        if let Some(span) = d.span {
            eprintln!("  --> {}:{}", span.lo.0, span.hi.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{EntityRef, FactKey, Interner, KindId, OpaqueToken, SelectorId};
    use dorc_plan::{LeafId, ProbePlan, ProbePredict, ProbeSiteKind};

    fn pkg(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("installed")),
        }
    }

    /// 24F §3 / corr-kind-keying §10: the resolver confusability enforcement. A clean single
    /// `<kind>.resolve()` is resolver-bearing; two files declaring ONE kind's resolver REFUSE both
    /// (the kind keeps token-equality — never first-wins-silently); a resolver keyed to a known
    /// PROVIDER name is KEPT but flagged (the mis-key). Behaviour-pinned (the diagnostics themselves
    /// are verified end-to-end via the cli binary).
    #[test]
    fn resolver_confusability_conflict_refuses_both_collision_keeps() {
        let mut i = Interner::default();
        // A provider "apt-get" exists (a lifted touches provider) — a resolver keyed to it collides.
        let touches_src = "apt-get.touches() { printf 'package:%s\\n' \"$1\"; }";
        let touches_paired = vec![(
            touches_src,
            dorc_oracle::touches::TouchesSet::lift(&mut i, touches_src).value,
        )];
        let checks: Vec<dorc_oracle::predict::PredictSet> = vec![];

        // A clean single package.resolve() ⇒ resolver-bearing.
        let clean = vec!["package.resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr = build_kind_resolvers(&clean, &checks, &touches_paired, &mut i, false);
        assert!(
            kr.resolver_kinds().any(|k| i.resolve(k) == "package"),
            "a clean package.resolve() is resolver-bearing"
        );

        // Two files, both package.resolve() ⇒ BOTH refused (no resolver kind).
        let dup = vec![
            "package.resolve() { printf '%s\\n' \"$1\"; }".to_string(),
            "package.resolve() { printf '%s\\n' \"$1\"; }".to_string(),
        ];
        let kr_dup = build_kind_resolvers(&dup, &checks, &touches_paired, &mut i, false);
        assert_eq!(
            kr_dup.resolver_kinds().count(),
            0,
            "a duplicate resolver for one kind refuses BOTH (token-equality floor)"
        );

        // A resolver keyed to the known provider "apt-get" ⇒ KEPT (warned, not a silent dud).
        let collide = vec!["apt-get.resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr_col = build_kind_resolvers(&collide, &checks, &touches_paired, &mut i, false);
        assert!(
            kr_col.resolver_kinds().any(|k| i.resolve(k) == "apt-get"),
            "a provider-named resolver is kept (the collision is a warning, not a refusal)"
        );
    }

    fn tool(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("present")),
        }
    }

    /// A no-member record key (the common single-fact site, `site N`).
    fn rk(n: u32) -> RecordKey {
        RecordKey {
            site: LeafId(n),
            member: None,
        }
    }

    /// A one-check probe over `fact` with the given site-kind (the firewall input).
    fn probe1(fact: FactKey, site_kind: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![ProbePredict {
                site: LeafId(0),
                member: None,
                fact,
                site_kind,
                provider: fact.kind.0,
                argv: vec![],
                sh: "{ :; }".to_string(),
            }],
            unresolvable: vec![],
        }
    }

    #[test]
    fn own_wall_coord_selects_kill_coord_for_kill_establish_for_establish() {
        // 24E §7 (resid-kill-coherence): the coherence comparand is the node's OWN effect
        // coordinate — its killed cell (a kill node, from `kill_coords`) OR its establish cell (an
        // establish class). `own_wall_coord` unifies both, extending the Stage-2 establish-wall
        // check to kill-walls in BOTH the authored and derived footprint lanes.
        use dorc_analysis::cfg::CfgNodeId;
        use dorc_analysis::effect::SkipClass;
        let mut i = Interner::default();
        let killed = pkg(&mut i, "nginx"); // package:nginx#installed (a purge's killed cell)
        let established = pkg(&mut i, "curl");
        let kill_node = CfgNodeId(7);
        let est_node = CfgNodeId(3);
        let classes = vec![(est_node, SkipClass::EstablishAmbient(established))];
        let mut kill_coords = BTreeMap::new();
        kill_coords.insert(kill_node, killed);
        assert_eq!(
            own_wall_coord(kill_node, &classes, &kill_coords),
            Some(dorc_plan::EntityCoord::new(killed.kind, killed.entity)),
            "a kill node's comparand is its killed coordinate (24E §7 — the new close)"
        );
        assert_eq!(
            own_wall_coord(est_node, &classes, &kill_coords),
            Some(dorc_plan::EntityCoord::new(
                established.kind,
                established.entity
            )),
            "an establish node's comparand is its establish coordinate (Stage 2, unchanged)"
        );
        assert_eq!(
            own_wall_coord(CfgNodeId(99), &classes, &kill_coords),
            None,
            "a node that is neither establish nor kill has no coherence comparand"
        );
    }

    #[test]
    fn parse_results_maps_three_outcome_and_carries_rc() {
        // The record maps holds/absent/cant-tell to the Effect verdict and carries the
        // raw rc on the wire (whether it is fold-usable is the firewall's call).
        let mut i = Interner::default();
        let r = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\nsite 2 effect=cant-tell rc=2\n",
            &mut i,
        );
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Converged)
        );
        assert_eq!(
            r.records.get(&rk(1)).map(|x| x.verdict),
            Some(Verdict::Diverged)
        );
        assert_eq!(
            r.records.get(&rk(2)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        assert_eq!(r.records.get(&rk(0)).map(|x| x.rc), Some(Rc(0)));
        assert_eq!(r.records.get(&rk(1)).map(|x| x.rc), Some(Rc(1)));
    }

    #[test]
    fn parse_results_drops_garbage_kfail_perform() {
        // Unrecognized / malformed lines are dropped (⇒ Unknown ⇒ run). Pins the
        // garbage-stdin behavior at the unit layer (`kFAIL-perform`). The dead
        // `declared-rc` lane is now just an unrecognized line ⇒ dropped.
        let mut i = Interner::default();
        let r = parse_results(
            "this is not a record\nsite notanumber effect=holds\n\
             site 0 garbled-no-effect\ndeclared-rc 0 rc=0\n# a comment\n",
            &mut i,
        );
        // `site 0 garbled-no-effect` parses the id but no effect= ⇒ Unknown (safe), rc 0.
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        // `site notanumber` ⇒ no id ⇒ dropped; the dead `declared-rc` line ⇒ dropped.
        assert_eq!(r.records.len(), 1, "only the id-parseable site landed");
    }

    #[test]
    fn parse_results_reserves_stdout_stderr_keys_inert() {
        // item-2 (19F §3 tuple shape): the `stdout=`/`stderr=` keys are RESERVED — the
        // parser accepts-and-stores them into the record's tuple, but they produce no
        // behavior change. Pin BOTH halves: (1) absent ⇒ the slots are `Predicted::Top`
        // (the default, the only state the probe actually emits today); (2) present ⇒
        // they intern into a `Predicted::Value(OutClaim)` and ride the tuple, while the
        // firewall + consumption gate are untouched (the consumed-stdout/stderr block is
        // unconditional, never reading the claim). Anti-masking: this asserts the SHAPE
        // exists end-to-end, NOT that a check predicts a value (nothing does this round).
        let mut i = Interner::default();
        let r = parse_results("site 0 effect=holds rc=0\n", &mut i);
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert_eq!(
            rec.stdout,
            Predicted::Top,
            "absent stdout= ⇒ ⊤ (the live default)"
        );
        assert_eq!(
            rec.stderr,
            Predicted::Top,
            "absent stderr= ⇒ ⊤ (the live default)"
        );
        // Reserved keys parse-and-store (a future stdout-producing probe is value-plumbing).
        let r = parse_results(
            "site 0 effect=holds rc=0 stdout=hello stderr=warn\n",
            &mut i,
        );
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert!(
            matches!(rec.stdout, Predicted::Value(OutClaim(_))),
            "a reserved stdout= is stored as a value claim: {:?}",
            rec.stdout
        );
        assert!(
            matches!(rec.stderr, Predicted::Value(OutClaim(_))),
            "a reserved stderr= is stored as a value claim: {:?}",
            rec.stderr
        );
        // The Effect/Status path is unaffected by the reserved keys' presence.
        assert_eq!(rec.verdict, Verdict::Converged);
        assert_eq!(rec.rc, Rc(0));
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
        let results = parse_results("site 0 effect=holds rc=0\n", &mut i);
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
        let results = parse_results("site 0 effect=holds rc=0\n", &mut i);
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
        let results = parse_results("site 0 effect=absent rc=1\n", &mut i);
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
        let results = parse_results("site 0 effect=holds rc=0\n", &mut i);
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

    /// Two checks over the SAME fact (distinct sites) — the conflict-floor input.
    fn probe2(fact: FactKey, k0: ProbeSiteKind, k1: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![
                ProbePredict {
                    site: LeafId(0),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k0,
                    sh: "{ :; }".to_string(),
                },
                ProbePredict {
                    site: LeafId(1),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k1,
                    sh: "{ :; }".to_string(),
                },
            ],
            unresolvable: vec![],
        }
    }

    #[test]
    fn same_cell_conflicting_records_degrade_to_top() {
        // 20I find-6a / item-5 (the conflict floor): two sites on the SAME cell whose
        // records DISAGREE merge to ⊤, never last-write-wins. Two establish sites: site 0
        // reports holds, site 1 reports absent (a self-contradicting / forged host). The
        // merged Effect must be `Unknown` (⊤) ⇒ the apply runs (kFAIL-perform), NOT the
        // last-written `absent` (or `holds`). Anti-masking: a constructed conflict, not a
        // hand-injected verdict the check should predict.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.effect,
            Verdict::Unknown,
            "disagreeing same-cell Effect verdicts degrade to ⊤ (Unknown), not last-write-wins"
        );
    }

    #[test]
    fn same_cell_agreeing_records_pass_through() {
        // The floor's other half: two same-cell sites that AGREE pass the value through
        // (no spurious ⊤). Two establish sites both reporting holds ⇒ merged Effect is
        // Converged (the agreed value), so a genuinely-converged cell still elides.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.effect,
            Verdict::Converged,
            "agreeing same-cell records keep the agreed verdict (no spurious ⊤)"
        );
    }

    #[test]
    fn same_cell_conflicting_query_status_degrades_to_top() {
        // The conflict floor on the Status channel: two VALID Query sites on one cell
        // reporting DIFFERENT rcs (rc=0 vs rc=1) ⇒ merged status ⊤ (a self-contradicting
        // guard cannot fold a branch). A valid Query's rc normally feeds Status (the
        // firewall), but a conflict on it must still degrade — the meet beats the firewall.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe2(
            fact,
            ProbeSiteKind::Query { valid: true },
            ProbeSiteKind::Query { valid: true },
        );
        let results = parse_results(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results)
            .get(&fact)
            .copied()
            .expect("keyed");
        // Effect agrees (both holds) ⇒ Converged; but the rcs disagree ⇒ status ⊤.
        assert_eq!(obs.effect, Verdict::Converged, "effect agrees");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "disagreeing same-cell Query rcs degrade Status to ⊤ (no fold off a contradiction)"
        );
    }

    #[test]
    fn unresolvable_diagnostics_name_the_source_command() {
        // q-2 (`dq-site-unresolvable`, the cli-edge readout): a probe-unresolvable site is
        // disclosed on stderr naming its SOURCE command text (`219` q-1.f silent-3 closed). An
        // un-oracled command (`make install`) is Opaque ⇒ unresolvable ⇒ the apply runs it; the
        // Note must carry its source. Drives the full pipeline (parse → classify → compile_probe
        // → build_plan) so the LeafId→source mapping is the real one.
        let mut interner = Interner::default();
        let book = "make install\n";
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
        let idx = dorc_oracle::KindIndex::default();
        let mut arena = ProvArena::new();
        let classified = dorc_analysis::effect::classify(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &[],
            &mut interner,
            &mut arena,
        );
        let classes = classified.value;
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg.value,
            &value,
            &classes,
            |_, _| None,
            |_| false,
        );
        let plan = dorc_plan::build_plan(
            book,
            &parsed.value,
            &cfg.value,
            &classes,
            // All-Unknown ⇒ nothing elides regardless of any vouch; empty is honest here.
            &dorc_plan::Vouches::new(),
            |_| Observable::verdict_only(Verdict::Unknown),
            &mut arena,
        );
        let diags = unresolvable_diagnostics(&probe, &plan, &parsed.value, book, &mut interner);
        assert!(
            diags.iter().any(|d| d.code.0 == "dq-site-unresolvable"),
            "an Opaque site must be disclosed unresolvable: {diags:?}"
        );
        assert!(
            diags
                .iter()
                .any(|d| d.code.0 == "dq-site-unresolvable" && d.message.contains("make install")),
            "the disclosure must name the source command: {diags:?}"
        );
        assert!(
            diags.iter().all(|d| d.severity == Severity::Note),
            "the readout is Note-severity (never trips gate-3): {diags:?}"
        );
    }

    #[test]
    fn advisory_filter_drops_warnings_notes_but_keeps_errors_in_apply() {
        // rec-1 / tc-apply-receipt-floor (ui-A): the receipt-free `apply` off-ramp keeps the
        // ERROR floor (a shippable artifact must never hide an error) while dropping the
        // advisory plane (warnings + notes); `plan`/round-trip (advisory=true) pass everything
        // through. This is the lone place the artifact-vs-render two-surface split becomes a
        // per-severity routing decision — pin BOTH directions so a future edit cannot silently
        // (a) leak advisory disclosure into the off-ramp surface, or (b) swallow an error there.
        // The slugs are the diag_tidy-recognized throwaway-fixture set (`x-err`/`x-warn`/
        // `x-note`, core::tests::diag_tidy::is_test_fixture_slug) — NOT real catalog codes, so
        // the legacy-allow-list completeness gate (226 §1) exempts them without an allow-list entry.
        use dorc_core::{BytePos, DiagCode, Diagnostic, Span};
        let span = Some(Span::new(BytePos(0), BytePos(1)));
        let mixed = vec![
            Diagnostic::error(DiagCode("x-err"), span, "an error"),
            Diagnostic::warning(DiagCode("x-warn"), span, "a warning"),
            Diagnostic::note(DiagCode("x-note"), span, "a note"),
        ];

        // advisory=true (plan / round-trip): every severity survives — the full cited-disclosure
        // render surface (ru-20 ui-3).
        let kept = advisory_filter(true, &mixed);
        assert_eq!(kept.len(), 3, "plan surface keeps every severity: {kept:?}");

        // advisory=false (apply off-ramp): ONLY the error survives — receipt-free, not blind.
        let kept = advisory_filter(false, &mixed);
        assert_eq!(
            kept.len(),
            1,
            "apply keeps only the error floor (no warnings/notes): {kept:?}"
        );
        assert_eq!(
            kept[0].severity,
            Severity::Error,
            "the surviving diagnostic is the Error (the never-hide floor): {kept:?}"
        );
    }
}
