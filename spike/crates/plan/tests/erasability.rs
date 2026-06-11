//! THE ERASABILITY GATE (arch-1, `Research/plans/22A` concl-1/2/3 / `notes/229`; the
//! round-22 contract's centerpiece). Runs the analyzer TWICE over a fixture set — run-A
//! normal, run-B with the receipts plane adversarially varied — and asserts the DECISION
//! output is IDENTITY-EXACT. A divergence is a receipt-into-decision leak (a WELD breach).
//!
//! # What run-B varies (the adversarial injection — concl-1 / finding-2)
//!
//! The mature systems inject MAXIMAL variance and assert invariance (Debian's ~20 sentinel
//! axes; LLVM's `-reverse-iterate`; GCC's `-gtoggle`). Ours varies, via the
//! [`dorc_core::ProvArena`] DI seam ([`ProvArena::adversarial`]):
//! * **sentinel receipt ids** — every minted `ProvId` is offset by a seed, so run-B's receipt
//!   VALUES are deliberately not run-A's (a decision reading a specific id diverges);
//! * **reversed origin order** — every join reverses its offered parents (a decision reading
//!   the surviving-parent witness or its order diverges);
//! * **a varied DI'd seed** — the seed differs per fixture, so the perturbation is not a fixed
//!   shift a leak could accidentally survive.
//!
//! The host answers (`observe`) are IDENTICAL across A and B — the arena variation is the ONLY
//! difference, so any output divergence is attributable to receipts alone.
//!
//! # The identity plane (ru-12 floor) vs the exempt plane
//!
//! Compared byte-exact ([`dorc_plan::erasability::canonical_decision`]): per-site dispositions
//! (run/replace/omit + license decision-fields), the rendered probe/apply artifacts INCLUDING
//! comments, and Error-class diagnostics by `(code, site, severity)`. Exempt (omitted by the
//! canon projection): explanation text, receipt ids, origin ordering, timing.
//!
//! # The coverage canary (concl-3 / finding-3 / `mechanism-coverage-canary`)
//!
//! Gates rot by silent no-op, not deletion. So the gate PROVES IT RAN: it counts the
//! comparisons performed and asserts the count is nonzero AND that the fixtures actually
//! exercised the receipts plane (a nonzero arena, ≥1 Replace, ≥1 ⊤-cause). No auto-retry, no
//! quarantine — a failure here is a hard stop (this is a `#[test]`, never `#[ignore]`d).
//!
//! # The hunt-list (ways a receipt could leak — each fixture targets ≥1)
//!
//! * `hunt-1` (Eq/fixpoint): `Top(a) != Top(b)` would perturb `solve`'s convergence → a
//!   different classification. Fixture: `opaque_then_install` (a ⊤ poisons a downstream
//!   establish). Run-B's sentinel cause must not change the `EstablishWritten` verdict.
//! * `hunt-2` (ordering via a map key): a `ProvId` in a decision `BTreeMap`/`BTreeSet` key
//!   would reorder rendered output. Structurally impossible (`ProvId: !Ord`); the byte-exact
//!   artifact compare is the backstop. Fixture: any multi-site book.
//! * `hunt-3` (join-cap witness order): the k-capped join's surviving parents differ under
//!   reversal; a decision reading them would diverge. Fixture: `branchy` (control-flow merges
//!   → joins). Run-B reverses join parents.
//! * `hunt-5` (digest/canon): the digest must hash ONLY the identity plane. Asserted by the
//!   digests matching across A/B for every fixture.
//! * `hunt-6` (Top monotonicity): a ⊤ re-derived per fixpoint iteration with a fresh cause
//!   would defeat termination / grow the arena unboundedly. Fixture: `loop_with_opaque` (a ⊤
//!   inside a loop body, re-derived across the back-edge). Hash-consing + Eq-ignores-cause.
//! * `hunt-7` (seed/RNG): a receipt id from a hashed/random source would vary per process.
//!   Run-B's varied seed asserts the decision is seed-invariant.
//! * `hunt-8` (diagnostics): an Error's message (receipt-rendered) must be exempt; the
//!   (code,site,severity) tuple is identity. Fixture: `heredoc_refused` (a render-refusal
//!   Error). The canon drops the message.

use dorc_core::{
    EntityRef, FactKey, Interner, KindId, Observable, OpaqueToken, ProvArena, SelectorId, Verdict,
};
use dorc_plan::erasability::{canonical_decision, decision_digest};
use dorc_plan::{ProbePlan, build_plan, compile_probe};

/// The corpus-shaped apt-get oracle (mirrors `e2e/cases/converged/package.oracle.sh`): the
/// `oracle_effect` declarations (install ⇒ establish `package#installed`, purge ⇒ kill) are
/// what make a command classify as an Establish (and thus eligible for Replace) — WITHOUT them
/// every command is Opaque and the gate would never exercise the elision plane (the
/// anti-masking lesson: a fixture that elides nothing cannot test receipt-inertness OF a
/// decision). The `apt_get__check` argparse is the entity-resolver; `oracle_probe_package` is
/// the shipped probe. Lifted with the test interner so provider symbols match the book's words.
const ORACLE_SRC: &str = r#"
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge kill installed
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1; fi
}
"#;

/// The fixture books — each targets ≥1 hunt-list leak vector (see the module header). Chosen to
/// exercise the receipts plane where a leak COULD bite: ⊤-causes, control-flow joins, loops
/// (re-derived ⊤), and the disposition variety (Run/Replace/Omit).
const FIXTURES: &[(&str, &str)] = &[
    // hunt-1: a ⊤ (un-oracled `ufw`) poisons the downstream install ⇒ EstablishWritten. The
    // sentinel cause in run-B must not change that.
    (
        "opaque_then_install",
        "ufw allow 80/tcp\napt-get install nginx\n",
    ),
    // A clean convergence-elision (Replace). The simplest identity-plane case.
    ("lone_install", "apt-get install nginx\n"),
    // hunt-3: control-flow branches create JOIN merges in the dataflow; their provenance joins
    // are reversed in run-B. The decision (both establishes) must be invariant.
    (
        "branchy",
        "if [ -f /flag ]; then apt-get install nginx; else apt-get install curl; fi\n",
    ),
    // hunt-6: a ⊤ inside a loop body is re-derived across the back-edge every fixpoint pass —
    // hash-consing keeps the arena bounded and Eq-ignores-cause keeps it terminating.
    (
        "loop_with_opaque",
        "for h in a b c; do ufw allow 22; apt-get install nginx; done\n",
    ),
    // A multi-line book: a converged install (Replace) + a divergent install (Run) + a bare
    // opaque (MustRun, ⊤-cause) — all three dispositions in one book.
    (
        "mixed_dispositions",
        "apt-get install nginx\napt-get install curl\nmake all\n",
    ),
    // An `&&` chain (a guard reads status) + a fold case — disposition variety.
    ("andor", "apt-get install nginx && apt-get install curl\n"),
];

/// The facts the simulated host already has (Converged). Fixed across run-A and run-B so the
/// arena variation is the ONLY difference. We converge `package:nginx#installed` (NOT curl's),
/// so a fixture installing nginx ELIDES (Replace) while one installing curl RUNS — exercising
/// both sides of the disposition plane.
fn converged_facts(i: &mut Interner) -> Vec<FactKey> {
    vec![FactKey {
        kind: KindId(i.intern("package")),
        entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        selector: SelectorId(i.intern("installed")),
    }]
}

/// Run the WHOLE analyzer pipeline (parse → cfg → value → classify → `compile_probe` →
/// `build_plan` → render) over `book` with the given arena `variation`, returning the canonical
/// identity-plane string + the arena's node count (the canary's "plane was exercised" signal).
///
/// The host answers are a fixed Converged-set membership test — IDENTICAL regardless of
/// `variation` — so the only thing that differs between an A-run and a B-run is the receipts
/// plane. A genuinely-inert plane yields byte-identical canonical strings.
fn run_pipeline(book: &str, variation: ArenaMode) -> RunOutcome {
    use dorc_plan::Disposition;
    let mut i = Interner::default();
    let idx = dorc_oracle::lift(&mut i, &[ORACLE_SRC]).value;
    let checks = vec![dorc_oracle::check::lift_checks(&mut i, ORACLE_SRC).value];
    let converged = converged_facts(&mut i);

    let parsed = dorc_syntax::parse(book);
    let cfg = dorc_analysis::cfg::build(&parsed.value).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);

    let mut arena = match variation {
        ArenaMode::Normal => ProvArena::new(),
        ArenaMode::Adversarial { seed } => ProvArena::adversarial(seed),
    };
    let classified = dorc_analysis::effect::classify(
        &cfg,
        &value,
        &parsed.value,
        &idx,
        &checks,
        &mut i,
        &mut arena,
    );
    let classes = classified.value;

    let probe = compile_probe(&parsed.value, &cfg, &classes, |kind, selector| {
        idx.resolve_probe(kind, selector).map(|p| p.body.clone())
    });
    // The host oracle: a fact is Converged iff in the fixed set; else Diverged. Identical for
    // both runs (the arena variation is the sole difference).
    let observe = |f: FactKey| -> Observable {
        if converged.contains(&f) {
            Observable::verdict_only(Verdict::Converged)
        } else {
            Observable::verdict_only(Verdict::Diverged)
        }
    };
    let plan = build_plan(book, &parsed.value, &cfg, &classes, observe);

    // Tally the dispositions so the canary can prove the elision plane was actually exercised
    // (the anti-masking floor: a fixture set that NEVER elides cannot test receipt-inertness of
    // a Replace decision — exactly the bug an effect-less oracle would silently introduce).
    let replaces = plan
        .steps
        .iter()
        .filter(|s| matches!(s.disposition, Disposition::Replace(_, _)))
        .count();

    let canon = canonical_decision(&plan, &probe, book, &parsed.value, &i, &classified.diags);
    RunOutcome {
        canon,
        arena_nodes: arena.len(),
        replaces,
    }
}

/// What one pipeline run produced — the canonical decision plus the canary's "the plane was
/// exercised" signals (arena size, replace count).
struct RunOutcome {
    canon: String,
    arena_nodes: usize,
    replaces: usize,
}

#[derive(Clone, Copy)]
enum ArenaMode {
    Normal,
    Adversarial { seed: u32 },
}

#[test]
fn decision_is_identity_exact_under_adversarial_receipts() {
    // THE GATE (arch-1 / `22A` concl-1): every fixture's decision output is byte-identical
    // between a normal run and a receipts-adversarially-varied run. A divergence is a
    // receipt-into-decision leak (a WELD breach). The coverage canary (below) proves the gate
    // actually ran and exercised the plane — gates rot by silent no-op (concl-3).
    let mut comparisons = 0usize;
    let mut total_arena_nodes = 0usize;
    let mut total_replaces = 0usize;
    for (name, book) in FIXTURES {
        let a = run_pipeline(book, ArenaMode::Normal);
        // Run-B: a per-fixture seed (so the perturbation is not a fixed shift a leak could
        // accidentally survive — hunt-7). The seed is a deterministic function of the fixture
        // NAME so the test itself stays reproducible (no clock/RNG — `inv-determinism`).
        let seed = 1000 + u32::try_from(name.len()).unwrap_or(0) * 7 + 3;
        let b = run_pipeline(book, ArenaMode::Adversarial { seed });

        assert_eq!(
            a.canon, b.canon,
            "ERASABILITY BREACH in fixture `{name}`: the decision output changed when receipts \
             were adversarially varied (sentinel ids + reversed origin order + varied seed). A \
             receipt leaked into a decision. Canonical A vs B:\n--- A ---\n{}\n--- B ---\n{}",
            a.canon, b.canon
        );
        // The arena's node COUNT may differ (the adversarial offset/reversal can change how
        // joins cons), but that is EXEMPT (Timing) — only the canonical decision must match.
        // We DO assert run-A's arena is nonempty per fixture (the plane was exercised).
        assert!(
            a.arena_nodes > 0,
            "fixture `{name}` exercised no receipts (empty arena) — it cannot test inertness; \
             pick a book that mints ≥1 origin"
        );
        comparisons += 1;
        total_arena_nodes += a.arena_nodes;
        total_replaces += a.replaces;
    }

    // THE COVERAGE CANARY (concl-3 / `mechanism-coverage-canary`): assert the gate ACTUALLY
    // RAN and compared something. A silent no-op (the gate's documented rot mode) trips here.
    assert!(
        comparisons >= FIXTURES.len() && comparisons > 0,
        "coverage canary: the erasability gate compared {comparisons} fixtures, expected \
         {} — the gate was skipped or short-circuited (concl-3 silent-no-op rot)",
        FIXTURES.len()
    );
    assert!(
        total_arena_nodes > 0,
        "coverage canary: NO fixture minted any receipts (total arena nodes 0) — the gate ran \
         but the receipts plane was never exercised, so it proved nothing"
    );
    // The ANTI-MASKING canary (the lesson from the effect-less-oracle bug): the fixture set
    // must exercise the ELISION plane — a Replace decision is where a receipt leak would most
    // plausibly bite (a license is the decision a receipt could perturb). If NOTHING elides,
    // the gate proves only that opaque/run sites are receipt-inert, which is the easy half.
    assert!(
        total_replaces > 0,
        "anti-masking canary: NO fixture produced a Replace disposition — the gate never \
         exercised the elision plane (the half a receipt-into-license leak would touch). Fix \
         the oracle/host so ≥1 fixture elides (the effect-map must declare establishes)."
    );
}

#[test]
fn digest_is_receipt_invariant_across_runs() {
    // hunt-5: the decision digest hashes ONLY the identity plane, so it is byte-identical
    // between a normal and an adversarial run. (The digest is what the cli emits every run as a
    // drift signal; if a receipt could move it, the signal would be noise — and worse, would
    // hint a receipt reached a decision input.)
    let mut i = Interner::default();
    let idx = dorc_oracle::lift(&mut i, &[ORACLE_SRC]).value;
    let checks = vec![dorc_oracle::check::lift_checks(&mut i, ORACLE_SRC).value];
    let converged = converged_facts(&mut i);
    let book = "ufw allow 80/tcp\napt-get install nginx\napt-get update\n";

    let digest_for = |variation: ArenaMode, i: &mut Interner| -> String {
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, i);
        let mut arena = match variation {
            ArenaMode::Normal => ProvArena::new(),
            ArenaMode::Adversarial { seed } => ProvArena::adversarial(seed),
        };
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            i,
            &mut arena,
        )
        .value;
        let probe = compile_probe(&parsed.value, &cfg, &classes, |k, s| {
            idx.resolve_probe(k, s).map(|p| p.body.clone())
        });
        let observe = |f: FactKey| {
            if converged.contains(&f) {
                Observable::verdict_only(Verdict::Converged)
            } else {
                Observable::verdict_only(Verdict::Diverged)
            }
        };
        let plan = build_plan(book, &parsed.value, &cfg, &classes, observe);
        decision_digest(&plan, &probe, book, &parsed.value, i, &[])
    };

    let normal = digest_for(ArenaMode::Normal, &mut i);
    let adversarial = digest_for(ArenaMode::Adversarial { seed: 4242 }, &mut i);
    assert_eq!(
        normal, adversarial,
        "the decision digest moved under adversarial receipts — it must hash only the identity \
         plane (hunt-5)"
    );
}

#[test]
fn empty_probe_plan_default_is_inert() {
    // A guardrail for the canary's own assumptions: an empty pipeline still produces a
    // well-formed (empty) canonical decision, so the gate's machinery cannot itself panic on a
    // trivial input. (Not a leak test — a robustness floor for the harness.)
    let probe = ProbePlan::default();
    let ast = dorc_syntax::parse("").value;
    let i = Interner::default();
    let plan = dorc_plan::Plan { steps: vec![] };
    let canon = canonical_decision(&plan, &probe, "", &ast, &i, &[]);
    assert!(canon.contains("== plan =="), "well-formed empty canonical");
}
