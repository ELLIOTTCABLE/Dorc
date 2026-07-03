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
fn every_topology_class_and_both_behaviours_are_reached() {
    use TopologyClass::{
        HitConverged, KillWall, MissConverged, MissDiverged, MultiWall, SilentWall,
    };
    let want = [
        MissConverged,
        MissDiverged,
        HitConverged,
        KillWall,
        SilentWall,
        MultiWall,
    ];
    let n = seed_count();
    let mut seen: BTreeSet<TopologyClass> = BTreeSet::new();
    let mut honest_elisions = 0u64;
    let mut lying_divergences = 0u64;
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
        flag_distinguishes > 0,
        "--trust-footprints NEVER changed the plan in {n} seeds — the flag is inert, so the \
         survival tier is untested."
    );
    eprintln!(
        "sweep coverage over {n} seeds: all 6 topology classes reached by seed {first_all:?}; \
         honest_elisions={honest_elisions} lying_divergences={lying_divergences} \
         flag_distinguishes={flag_distinguishes}"
    );
}
