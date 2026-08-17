//! The validity fixpoint (`26H` §4 — W-C) and its cascade attribution.
//!
//! Lifted out of the binary so the loom seam can run the SAME rounds the binary runs
//! (`lib-target-is-a-loom-seam`): a why report that explains a cascaded elision has to be built by
//! the machinery that caused the cascade, or its round number is a decoration. Everything here is
//! pure over the frozen model (`inv-determinism`) — no clock, no host, no I/O.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::CollapseNarrative;
use dorc_aid::diag::Diag;
use dorc_aid::narrative::{CollapseKind, SpeechAct};
use dorc_core::{Interner, Observable, ProvArena, Verdict};

use crate::results::{SiteResults, facts_from_sites};
use crate::why::CascadeAttribution;

/// The FROZEN inputs of the validity fixpoint (`26H` §4¾): carried verbatim across every
/// round, never re-derived and never re-admitted. Book, CFG, spans, value-flow, the effect
/// map, the oracle lifts. The admitted records and the compiled probe are frozen too, and
/// ride beside this rather than in it — they belong to the intake edge, not the model.
#[derive(Debug)]
pub struct FrozenModel<'a> {
    /// The book's control-flow graph.
    pub cfg: &'a dorc_analysis::cfg::Cfg,
    /// The book's value-flow.
    pub value: &'a dorc_analysis::value::ValueFlow,
    /// The parsed book.
    pub ast: &'a dorc_syntax::ast::Ast,
    /// The loaded oracles' effect map.
    pub idx: &'a dorc_oracle::KindIndex,
    /// The loaded oracles' `predict` sets, in argv order.
    pub checks: &'a [dorc_oracle::predict::PredictSet],
    /// The loaded oracles' verdict index.
    pub verdicts: &'a dorc_oracle::verdict::VerdictIndex,
    /// The wrapper peel, per wrapped site.
    pub peeled: &'a BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_analysis::effect::PeeledSite>,
    /// Which definition is live AT each site (`28K` §2 rul-visibility-is-full-positional).
    ///
    /// It belongs in the frozen set for the same reason the rest of it does
    /// (`the-frozen-set-includes-the-function-environment`): the environment is solved ONCE from
    /// the origin model, and the ratchet erases EFFECTS with no authority over BINDINGS. Carrying
    /// it here is what lets the loop READ the same answer every round without being able to
    /// re-derive one — the driver never spells an env entry point inside the loop, which the
    /// lexical fence in `main.rs` enforces.
    pub live: dorc_analysis::funcenv::LiveDefinitions<'a>,
}

/// One round's PURE DERIVATION from (frozen inputs, erasure ledger) — recomputed from
/// scratch every round, never incrementally patched (`26H` §4¾). Every field here is a
/// function of the residual model alone.
#[derive(Debug)]
pub struct ClassifiedRound {
    /// Every classified leaf, in leaf order — the index IS the [`dorc_plan::LeafId`].
    pub classes: Vec<(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )>,
    /// This round's classify diagnostics.
    pub diags: Vec<Diag>,
    /// This round's why-lens diagnostics (the ⊤-cause disclosures).
    pub why_diags: Vec<Diag>,
    /// The killing sites.
    pub kills: BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    /// Each killing site's coordinate.
    pub kill_coords: BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_core::FactKey>,
    /// Each fact's backing set.
    pub fact_backings: BTreeMap<dorc_core::FactKey, dorc_core::FactBacking>,
    /// The collapse narratives this round minted.
    pub classify_narrative: Vec<CollapseNarrative>,
    /// The sites whose writes invalidate a downstream query.
    pub invalidators: BTreeSet<dorc_analysis::cfg::CfgNodeId>,
}

/// What the fixpoint settled on: the FINAL round's model and observations, plus the ledger
/// that produced it. Nothing from any earlier round is here — earlier rounds construct only
/// a classification and a fold, never a plan, a narrative surface, or a render.
#[derive(Debug)]
pub struct SettledFixpoint {
    /// The FINAL round's model.
    pub round: ClassifiedRound,
    /// The final fold: each fact's merged observable.
    pub by_fact: BTreeMap<dorc_core::FactKey, Observable>,
    /// The fold's collapse narratives.
    pub merge_narrative: Vec<CollapseNarrative>,
    /// Each shared cell whose cross-site merge degraded a channel, with how many sites measured it.
    pub collapsed: BTreeMap<dorc_core::FactKey, u32>,
    /// Every erasure the rounds proved, round-tagged.
    pub ledger: dorc_plan::erase::ErasureLedger,
    /// Did the loop hit its cap and degrade to origin? Unreachable at the production bound, so
    /// the caller `debug_assert`s it false; the fault-injection pin drives it true deliberately.
    pub capped: bool,
    /// Round 1.s validity bits — the ORIGIN model's answer, kept so the why-chain can tell a
    /// site that was always trustworthy from one whose guard only became trustworthy because
    /// something upstream was proven dead. The latter is the cascade `26H` §4.6 requires be
    /// renderable, and it is the only reason any round-1 quantity outlives its round.
    pub origin_validity: BTreeMap<dorc_plan::LeafId, bool>,
}

/// Attribute every round-2+ validity flip to the erasures that caused it.
///
/// A guard becomes valid exactly when every invalidator reaching it has been erased, so the
/// cause of site `L`'s flip is precisely the ledger entries whose sites REACH `L` in the
/// control-flow graph. Computed once, after quiescence, over the frozen CFG — forward
/// reachability from each erased site, which is exact and cheap next to the network this
/// whole engine exists to avoid.
#[must_use]
pub fn attribute_cascades(
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    ledger: &dorc_plan::erase::ErasureLedger,
    origin_validity: &BTreeMap<dorc_plan::LeafId, bool>,
) -> BTreeMap<dorc_plan::LeafId, CascadeAttribution> {
    let line_of_node = |node: dorc_analysis::cfg::CfgNodeId| {
        let lo = ast.node(cfg.node(node).ast).span.lo.0 as usize;
        dorc_aid::diag::line_col(book_src, lo).0
    };
    let mut out = BTreeMap::new();
    for (leaf, (node, class)) in classes.iter().enumerate() {
        let Ok(leaf) = u32::try_from(leaf) else {
            continue;
        };
        let leaf = dorc_plan::LeafId(leaf);
        if !matches!(
            class,
            dorc_analysis::effect::SkipClass::QueryResolvable { valid: true, .. }
        ) || origin_validity.get(&leaf) != Some(&false)
        {
            continue;
        }
        let causes: Vec<&dorc_plan::erase::ErasureEntry> = ledger
            .entries()
            .filter(|entry| reaches(cfg, entry.site(), *node))
            .collect();
        let Some(last) = causes.iter().max_by_key(|entry| entry.round()) else {
            continue;
        };
        out.insert(
            leaf,
            CascadeAttribution {
                erased_lines: causes.iter().map(|e| line_of_node(e.site())).collect(),
                controller_line: dorc_aid::diag::line_col(
                    book_src,
                    ast.node(last.proof().controller()).span.lo.0 as usize,
                )
                .0,
                round: last.round().0,
            },
        );
    }
    out
}

/// Is `to` reachable from `from` in the CFG? A plain forward walk over the frozen graph.
fn reaches(
    cfg: &dorc_analysis::cfg::Cfg,
    from: dorc_analysis::cfg::CfgNodeId,
    to: dorc_analysis::cfg::CfgNodeId,
) -> bool {
    use dorc_analysis::solve::Graph as _;
    let mut seen = vec![false; cfg.node_count()];
    let mut stack = vec![from];
    while let Some(node) = stack.pop() {
        for next in cfg.succ_ids(node) {
            if next == to {
                return true;
            }
            if seen.get(next.index()) == Some(&false) {
                if let Some(slot) = seen.get_mut(next.index()) {
                    *slot = true;
                }
                stack.push(next);
            }
        }
    }
    false
}

/// Classify the residual model named by `erased` (round 1 passes the empty overlay).
///
/// `trip` is the RUN-WIDE latch (`302:rul-certifier-trip-guard-only`), threaded rather than
/// returned per round on purpose: intermediate rounds are never observed
/// (`the-fixpoint-owns-the-rounds-and-builds-nothing-else`), so a round-2 reach failure would be
/// invisible to any consumer reading only the settled round — and a trip anywhere in the spine
/// disqualifies the whole spine.
#[must_use]
pub fn classify_round(
    frozen: &FrozenModel<'_>,
    erased: &dorc_analysis::erase::ErasedSites,
    interner: &mut Interner,
    arena: &mut ProvArena,
    degrades: &mut BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_oracle::predict::TopReason>,
    verdict_lane: &mut BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_analysis::effect::Measurement>,
    trip: &mut dorc_analysis::certify::CertifierTrip,
) -> ClassifiedRound {
    let (
        classified,
        why_diags,
        kills,
        kill_coords,
        fact_backings,
        classify_narrative,
        invalidators,
    ) = dorc_analysis::effect::classify_with_why_diags(
        frozen.cfg,
        frozen.value,
        frozen.ast,
        frozen.idx,
        frozen.checks,
        frozen.verdicts,
        frozen.peeled,
        erased,
        interner,
        arena,
        degrades,
        verdict_lane,
        trip,
        frozen.live,
    );
    ClassifiedRound {
        classes: classified.value,
        diags: classified.diags,
        why_diags,
        kills,
        kill_coords,
        fact_backings,
        classify_narrative,
        invalidators,
    }
}

/// The per-round VALIDITY VIEW: each Query leaf's `valid` bit, as this round's residual model
/// computes it. `classes` is leaf-ordered (the positional assignment `build_plan` and
/// `build_vouches` share), so the index IS the site's [`dorc_plan::LeafId`].
///
/// Round 1's view necessarily equals the bits baked into the frozen probe, which is what
/// keeps a world with nothing to erase byte-identical.
fn validity_view(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
) -> BTreeMap<dorc_plan::LeafId, bool> {
    classes
        .iter()
        .enumerate()
        .filter_map(|(leaf, (_, class))| match class {
            dorc_analysis::effect::SkipClass::QueryResolvable { valid, .. } => {
                Some((dorc_plan::LeafId(u32::try_from(leaf).ok()?), *valid))
            }
            _ => None,
        })
        .collect()
}

/// Run the validity fixpoint to quiescence (`26H` §4 — W-C, the flagship fix).
///
/// Round k derives the residual model from origin + ledger, re-folds the FROZEN records
/// through it, and appends every newly-proven-dead site; the loop ends when a round proves
/// nothing new. Monotone by construction (erasure only ever REMOVES invalidators, so a
/// query can only become valid, so a fold can only find more deadness) and bounded by the
/// site count, since every growing round adds at least one of finitely many sites. The cap
/// is therefore unreachable; it exists so a monotonicity regression cannot become a hang.
/// Hitting it DISCARDS the whole ledger and re-derives from the origin, so the run's answer is
/// exactly the pre-W-C one: no elision rests on a half-settled state nobody reasoned about, and
/// there is no partial fixpoint to be silent about. A `debug_assert` makes it loud in dev and
/// under DST — the same bargain `solve` strikes for its own unenforceable termination.
///
/// NO RE-PROBE (`26H` §0 v-no-reprobe-needed): invalid-Query checks already ship and their
/// rcs are already measured, merely withheld. This consumes measurements in hand; it never
/// asks a host anything, and `probe` is the frozen origin artifact throughout.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "the frozen model, the probe, the records, the origin round and the cap are the \
              fixpoint's inputs; the interner, the arena and the certifier-trip latch are the \
              three run-scoped accumulators it borrows. Bundling the accumulators into a context \
              struct would hide exactly which of them the loop may write, which is the property \
              the-fixpoint-owns-the-rounds-and-builds-nothing-else keeps visible"
)]
pub fn settle_validity_fixpoint(
    frozen: &FrozenModel<'_>,
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
    origin: ClassifiedRound,
    cap: u32,
    interner: &mut Interner,
    arena: &mut ProvArena,
    trip: &mut dorc_analysis::certify::CertifierTrip,
) -> SettledFixpoint {
    let mut ledger = dorc_plan::erase::ErasureLedger::new();
    let origin_validity = validity_view(&origin.classes);
    let mut round = origin;
    let mut number = 1u32;
    loop {
        let validity = validity_view(&round.classes);
        let (by_fact, merge_narrative, collapsed) = facts_from_sites(probe, results, &validity);
        let observe = |f: dorc_core::FactKey| {
            by_fact
                .get(&f)
                .copied()
                .unwrap_or(Observable::verdict_only(Verdict::Unknown))
        };
        let proofs = dorc_plan::erase::prove_dead_branches(
            frozen.ast,
            frozen.cfg,
            &round.classes,
            &round.invalidators,
            observe,
        );
        let before = ledger.len();
        for proof in proofs {
            ledger.record(proof, dorc_plan::erase::RoundId(number));
        }
        let grew = ledger.len() > before;
        if !grew {
            return SettledFixpoint {
                round,
                by_fact,
                merge_narrative,
                collapsed,
                ledger,
                capped: false,
                origin_validity,
            };
        }
        if number >= cap {
            let discarded = u32::try_from(ledger.len()).unwrap_or(u32::MAX);
            ledger.rebuild_from_origin();
            let round = classify_round(
                frozen,
                &ledger.overlay(),
                interner,
                arena,
                &mut BTreeMap::new(),
                &mut BTreeMap::new(),
                trip,
            );
            let validity = validity_view(&round.classes);
            let (by_fact, mut merge_narrative, collapsed) =
                facts_from_sites(probe, results, &validity);
            // Withdrawing licensed elisions is a safety-narrowing like any other, so it narrates.
            merge_narrative.push(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::FixpointCapDegrade {
                    rounds: number,
                    discarded,
                },
            ));
            return SettledFixpoint {
                round,
                by_fact,
                merge_narrative,
                collapsed,
                ledger,
                capped: true,
                origin_validity,
            };
        }
        number = number.saturating_add(1);
        round = classify_round(
            frozen,
            &ledger.overlay(),
            interner,
            arena,
            &mut BTreeMap::new(),
            &mut BTreeMap::new(),
            trip,
        );
    }
}
