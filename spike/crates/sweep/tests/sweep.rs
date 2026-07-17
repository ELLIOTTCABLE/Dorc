//! The round-24 chronology-net sweep (24B §1-C / §3 / §5). Thousands of seeds run in the default
//! `cargo test`; `SWEEP_SEEDS=<n>` env-gates a deeper nightly run. Each `#[test]` is a full seed
//! sweep — cargo runs them in parallel, so wall time ≈ one sweep.
//!
//! The four guards (24B §5 / `128` / `notes/123` f20):
//!   * HONEST end-state equality — a mismatch is engine unsoundness (a wrong elision the frame
//!     rule permitted), always RED with the replay seed;
//!   * attribution-under-lies — a LYING divergence MUST be a survived elision whose witness names
//!     the generator's recorded liar (the dangerous tier stays accountable);
//!   * the determinism guard — a sampled seed replays bit-identically (a nondeterminism leak is a
//!     real `inv-determinism` bug);
//!   * the sometimes-asserts — every topology class + both load-bearing behaviours are actually
//!     REACHED, else the sweep is greenwashing (coverage is the unsolved half — the net HUNTS
//!     bugs, never proves their absence).

use std::collections::BTreeSet;

use dorc_sweep::{Honesty, Seed, TopologyClass, trial_for_seed};

/// The default seed budget (thousands, fast); `SWEEP_SEEDS=<n>` overrides for a deep run.
fn seed_count() -> u64 {
    std::env::var("SWEEP_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
}

/// The core net: over every seed, the flag-OFF baseline never under-executes, an HONEST scenario's
/// flag-ON end-state equals the bare book's (soundness), and a LYING divergence is attributed to
/// the liar (accountability). One pass, all three per-seed invariants (24B §5).
#[test]
fn end_state_equality_and_attribution_under_lies() {
    let n = seed_count();
    for s in 0..n {
        let t = trial_for_seed(Seed(s));

        // The flag-OFF baseline demotes every post-wall elision, so it can NEVER diverge from the
        // bare book. A divergence here is a deeper bug than any survival-tier error.
        assert!(
            !t.off_diverged(),
            "seed {s} [{:?}]: flag-OFF baseline under-executed (s_bare != s_apply_off) — the \
             conservative baseline elided something unsafe.\n  victim={}\n  bare={:?}\n  off={:?}",
            t.topology,
            t.victim_label,
            t.s_bare,
            t.s_apply_off,
        );

        match t.honesty {
            // HONEST: declared == true ⇒ the elided plan MUST reach the bare end-state. A mismatch
            // is ENGINE UNSOUNDNESS — a wrong elision the frame rule wrongly permitted (24B §5 red).
            Honesty::Honest => assert!(
                !t.on_diverged(),
                "seed {s} [{:?}]: HONEST scenario DIVERGED (s_bare != s_apply_on) = ENGINE \
                 UNSOUNDNESS — the survival tier elided a command the bare book needed.\n  \
                 victim={}\n  bare={:?}\n  apply_on={:?}\n  survivals={:?}",
                t.topology,
                t.victim_label,
                t.s_bare,
                t.s_apply_on,
                t.survivals,
            ),
            // LYING: the engine trusted a footprint that lied ⇒ end-states MAY diverge (the priced
            // residue). If they do, the assertion FLIPS to attribution: the divergence must be a
            // survived elision of the VICTIM whose witness names the LIAR (24B §5 / TC-3).
            Honesty::Lying { liar_leaf } => {
                if t.on_diverged() {
                    let diverged: BTreeSet<&String> =
                        t.s_bare.symmetric_difference(&t.s_apply_on).collect();
                    assert!(
                        diverged.contains(&t.victim_label),
                        "seed {s} [{:?}]: LYING divergence is NOT at the victim cell {} — got \
                         {diverged:?}. The generator's lie clobbers the victim; a divergence \
                         elsewhere means the net's model drifted.",
                        t.topology,
                        t.victim_label,
                    );
                    let attributed = t.survivals.iter().any(|sv| {
                        sv.elided_label == t.victim_label
                            && sv.crossed_walls.iter().any(|w| w.wall_leaf == liar_leaf)
                    });
                    assert!(
                        attributed,
                        "seed {s} [{:?}]: LYING divergence with NO survival witness attributing \
                         the victim's elision to the liar wall (leaf {liar_leaf}) — the dangerous \
                         tier is UNACCOUNTABLE (attribution-under-lies failed).\n  victim={}\n  \
                         survivals={:?}",
                        t.topology, t.victim_label, t.survivals,
                    );
                    // 24F §6 — the identity closure is the SHARPEST claim, so a lying-RESOLVER
                    // under-execute must ALSO name the resolver (the why-lens: "disjoint AFTER
                    // package__resolve() canonicalization"). Pin it for the aliasing lane: the
                    // victim's survival crossing carries `resolver = Some("package")`.
                    if t.topology == TopologyClass::AliasWall {
                        let names_resolver = t.survivals.iter().any(|sv| {
                            sv.elided_label == t.victim_label
                                && sv
                                    .crossed_walls
                                    .iter()
                                    .any(|w| w.resolver.as_deref() == Some("package"))
                        });
                        assert!(
                            names_resolver,
                            "seed {s} [AliasWall]: a lying-resolver divergence whose survival \
                             witness does NOT name the canonicalizing resolver (24F §6 \
                             attribution) — the sharpest claim in the design is unattributed.\n  \
                             survivals={:?}",
                            t.survivals,
                        );
                    }
                }
            }
        }
    }
}

/// The determinism guard (24B §5 C-determinism-guard / `notes/123` f20): a sampled seed replays
/// BIT-IDENTICALLY. For a pure kernel any divergence is a real bug — an observable `HashMap`
/// iteration or a non-`BTree` order leak (`inv-determinism`). The single highest-value guard here.
#[test]
fn trials_replay_bit_identically() {
    let n = seed_count();
    let step = (n / 128).max(1); // ~128 samples on the default run; every seed on a small SWEEP_SEEDS
    let mut checked = 0u64;
    let mut s = 0;
    while s < n {
        let first = trial_for_seed(Seed(s));
        let second = trial_for_seed(Seed(s));
        assert_eq!(
            first, second,
            "seed {s}: trial NOT bit-identical on replay — a nondeterminism leak (an observable \
             HashMap iteration / a non-BTree order, inv-determinism). The pure kernel must be a \
             pure function of its inputs."
        );
        checked += 1;
        s += step;
    }
    assert!(checked > 0, "determinism guard checked no seeds");
}

/// The sometimes-asserts (24B §1-C / `128` fc-5 — the reachability half): every topology class,
/// and both load-bearing behaviours (an HONEST survived elision, a LYING divergence, and the flag
/// actually changing the plan), are GENERATED across the seed range. A class that never appears
/// means the net silently stopped exercising a kernel path — it must fail loud, never greenwash.
/// COVERAGE (which *shapes* beyond the template) is the unsolved half; this is only reachability.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one pass accumulates every coverage counter (9 classes + honest/lying/derived/alias/reach nets + the flag-distinguishes gate) then asserts each non-vacuous in place — splitting it would either re-sweep the seeds N times or hide the counters from the assertions that read them"
)]
fn every_topology_class_and_both_behaviours_are_reached() {
    use TopologyClass::{
        AliasWall, DerivedWall, HitConverged, KillWall, MissConverged, MissDiverged, MultiWall,
        ReachWall, SilentWall,
    };
    let want = [
        MissConverged,
        MissDiverged,
        HitConverged,
        KillWall,
        SilentWall,
        MultiWall,
        DerivedWall,
        AliasWall,
        ReachWall,
    ];
    let n = seed_count();
    let mut seen: BTreeSet<TopologyClass> = BTreeSet::new();
    let mut honest_elisions = 0u64;
    let mut lying_divergences = 0u64;
    // The DERIVED-footprint soundness net (24E §6 / §11): a LYING derived footprint (⊂ true) that
    // makes the victim wrongly survive ⇒ RED. Counted separately from `lying_divergences` because
    // per `find-net-covers-what` (24C) the honest e2e fixture STRUCTURALLY cannot catch a
    // survival-tier under-execute — ONLY a lying scenario can, and this pins it for the DERIVED lane.
    let mut derived_lying_divergences = 0u64;
    // The lying-RESOLVER soundness net (24F §7.1): an alias scenario whose LYING resolver kept two
    // names of one referent apart ⇒ the victim wrongly survived ⇒ RED. Counted separately (like
    // derived_lying_divergences) because ONLY a lying scenario can catch a survival-tier under-
    // execute — the honest e2e fixture structurally cannot (find-net-covers-what). Non-vacuity for
    // the identity closure's soundness proof.
    let mut alias_lying_divergences = 0u64;
    // The lying-REACHES soundness net (24G): a ReachWall scenario whose LYING reach-answer OMITTED a
    // truly-reached coord ⇒ the expansion missed ⇒ the victim wrongly survived ⇒ RED. Counted
    // separately (like derived/alias) because ONLY a lying scenario can catch a survival-tier
    // under-execute (find-net-covers-what). Omission is the sharp edge; non-vacuity is fc-5.
    let mut reach_lying_divergences = 0u64;
    // The cross-author DEMOTE the mechanism exists for: an HONEST ReachWall scenario whose reach
    // expansion HIT the victim's backing ⇒ the victim DEMOTED, attributed "poisoned via
    // package__disturbance_reaches_only()". Counts the reach-ATTRIBUTED demotes (24G — the demote fires AND names the
    // reach-function). A zero here means the honest expansion never demoted ⇒ the mechanism is inert.
    let mut reach_poison_attributions = 0u64;
    let mut flag_distinguishes = 0u64;
    let mut first_all: Option<u64> = None;

    for s in 0..n {
        let t = trial_for_seed(Seed(s));
        seen.insert(t.topology);
        if matches!(t.honesty, Honesty::Honest) && !t.survivals.is_empty() {
            honest_elisions += 1;
        }
        if matches!(t.honesty, Honesty::Lying { .. }) && t.on_diverged() {
            lying_divergences += 1;
        }
        if t.topology == DerivedWall
            && matches!(t.honesty, Honesty::Lying { .. })
            && t.on_diverged()
        {
            derived_lying_divergences += 1;
        }
        if t.topology == AliasWall && matches!(t.honesty, Honesty::Lying { .. }) && t.on_diverged()
        {
            alias_lying_divergences += 1;
        }
        if t.topology == ReachWall && matches!(t.honesty, Honesty::Lying { .. }) && t.on_diverged()
        {
            reach_lying_divergences += 1;
        }
        // The honest cross-author demote: a ReachWall scenario whose reach expansion demoted the
        // victim and attributed it to `package__disturbance_reaches_only()`.
        if t.topology == ReachWall && t.reach_poisonings.iter().any(|(_, kind)| kind == "package") {
            reach_poison_attributions += 1;
        }
        if t.plan_on_fp != t.plan_off_fp {
            flag_distinguishes += 1;
        }
        if first_all.is_none() && want.iter().all(|c| seen.contains(c)) {
            first_all = Some(s + 1);
        }
    }

    for c in want {
        assert!(
            seen.contains(&c),
            "topology class {c:?} was NEVER generated across {n} seeds — the sweep is \
             GREENWASHING (a sometimes-assert failure, 128 fc-5). Either the generator stopped \
             producing it or a filter is silently excluding it."
        );
    }
    assert!(
        honest_elisions > 0,
        "no HONEST survived elision in {n} seeds — the net never exercised the golden-hill \
         survival path (the whole point of the flag)."
    );
    assert!(
        lying_divergences > 0,
        "no LYING divergence in {n} seeds — the attribution-under-lies branch was never \
         exercised, so its assertion is vacuous."
    );
    assert!(
        derived_lying_divergences > 0,
        "no LYING-DERIVED divergence in {n} seeds — a too-narrow DERIVED footprint (Host::derive \
         ⊂ the wall's true CellDelta) never produced the wrong-survival RED. The derived-footprint \
         soundness net (24E §6) is VACUOUS: without a lying-derived divergence the derived lane's \
         attribution-under-lies assertion is never exercised (find-net-covers-what — the honest \
         e2e fixture structurally cannot catch this)."
    );
    assert!(
        alias_lying_divergences > 0,
        "no LYING-RESOLVER divergence in {n} seeds — a LYING resolver (Host::resolve keeping two \
         names of one referent apart) never produced the wrong-survival RED. The aliasing closure's \
         soundness net (24F §7.1) is VACUOUS: without a lying-resolver divergence the identity \
         lane's attribution-under-lies assertion is never exercised (find-net-covers-what — fc-5)."
    );
    assert!(
        reach_lying_divergences > 0,
        "no LYING-REACHES divergence in {n} seeds — a LYING reach-answer (Host::reach OMITTING a \
         truly-reached coord) never produced the wrong-survival RED. The reaches() soundness net \
         (24G) is VACUOUS: without a lying-reaches divergence the reach lane's attribution-under-lies \
         assertion is never exercised (find-net-covers-what — omission is the sharp edge, fc-5)."
    );
    assert!(
        reach_poison_attributions > 0,
        "no HONEST reaches()-attributed demote in {n} seeds — a `package__disturbance_reaches_only()` expansion never \
         HIT a victim's backing to demote it (the cross-author demote the mechanism EXISTS for). The \
         reach expansion is inert: every reach-bearing footprint stayed narrow, so the compositional \
         half (24G §2/§3) is never exercised end-to-end."
    );
    assert!(
        flag_distinguishes > 0,
        "--trust-footprints NEVER changed the plan in {n} seeds — the flag is inert, so the \
         survival tier is untested."
    );
    eprintln!(
        "sweep coverage over {n} seeds: all 9 topology classes reached by seed {first_all:?}; \
         honest_elisions={honest_elisions} lying_divergences={lying_divergences} \
         derived_lying_divergences={derived_lying_divergences} \
         alias_lying_divergences={alias_lying_divergences} \
         reach_lying_divergences={reach_lying_divergences} \
         reach_poison_attributions={reach_poison_attributions} \
         flag_distinguishes={flag_distinguishes}"
    );
}
