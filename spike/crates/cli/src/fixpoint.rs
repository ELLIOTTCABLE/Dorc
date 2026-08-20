//! The validity fixpoint (`26H` §4 — W-C) and its cascade attribution.
//!
//! Lifted out of the binary so the loom seam can run the SAME rounds the binary runs
//! (`lib-target-is-a-loom-seam`): a why report that explains a cascaded elision has to be built by
//! the machinery that caused the cascade, or its round number is a decoration. Everything here is
//! pure over the frozen model (`inv-determinism`) — no clock, no host, no I/O.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::CollapseNarrative;
use dorc_aid::diag::Diag;

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

/// What the settlement settled on: the FINAL round's model and observations, the Spine it wrote,
/// and the ledger that produced them. Nothing from any earlier round is here — earlier rounds
/// construct only a classification and a fold, never a plan, a narrative surface, or a render.
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
    /// The settled decisions, written ONCE by the round that proved nothing new.
    pub spine: dorc_plan::Spine,
    /// The settled effective Query validity, per leaf.
    pub validity: BTreeMap<dorc_plan::LeafId, bool>,
    /// Everything the rounds proved cannot execute, round-tagged.
    pub ledger: dorc_plan::NoExecutionLedger,
    /// Did the loop hit its cap and degrade to the maximal-effects answer? Unreachable at the
    /// production bound, so the caller `debug_assert`s it false; the fault-injection pin drives it
    /// true deliberately.
    pub capped: bool,
    /// Round 1's validity bits — the ORIGIN model's answer, kept so the why-chain can tell a
    /// site that was always trustworthy from one whose guard only became trustworthy because
    /// something upstream was proven not to run. The latter is the cascade `26H` §4.6 requires be
    /// renderable, and it is the only reason any round-1 quantity outlives its round.
    pub origin_validity: BTreeMap<dorc_plan::LeafId, bool>,
}

/// Attribute every round-2+ validity flip to the DEAD-BRANCH erasures that caused it.
///
/// A guard becomes valid exactly when every invalidator reaching it has been retired, so the cause
/// of site `L`'s flip is the ledger entries whose sites REACH `L` in the control-flow graph.
/// Computed once, after quiescence, over the frozen CFG — forward reachability from each retired
/// site, which is exact and cheap next to the network this whole engine exists to avoid.
///
/// NAMED RESIDUE (`30K` §8 step-3): only the dead-branch species is attributed. A Query can now
/// also become valid because an upstream mutation was ELIDED, and that cascade has no controller
/// line to point at — its chain needs a shape this render does not have, so it is honestly absent
/// rather than mis-attributed to a controller that did not exist (`271:rul-sin-ordering`).
#[must_use]
pub fn attribute_dead_branch_cascades(
    cfg: &dorc_analysis::cfg::Cfg,
    ast: &dorc_syntax::ast::Ast,
    book_src: &str,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    ledger: &dorc_plan::NoExecutionLedger,
    validity: &BTreeMap<dorc_plan::LeafId, bool>,
    origin_validity: &BTreeMap<dorc_plan::LeafId, bool>,
) -> BTreeMap<dorc_plan::LeafId, CascadeAttribution> {
    let line_of_node = |node: dorc_analysis::cfg::CfgNodeId| {
        let lo = ast.node(cfg.node(node).ast).span.lo.0 as usize;
        dorc_aid::diag::line_col(book_src, lo).0
    };
    // The dead-branch causes, with the controller each one rests on.
    let dead: Vec<(
        dorc_analysis::cfg::CfgNodeId,
        dorc_plan::erase::RoundId,
        dorc_core::AstId,
    )> = ledger
        .entries()
        .filter_map(|(site, entry)| match entry.proof() {
            dorc_plan::world::NoMutationProof::DeadBranch(proof) => {
                Some((site, entry.round(), proof.controller()))
            }
            _ => None,
        })
        .collect();
    let mut out = BTreeMap::new();
    for (leaf, node) in dorc_plan::leaf_ids(ast, cfg, classes) {
        if validity.get(&leaf) != Some(&true) || origin_validity.get(&leaf) != Some(&false) {
            continue;
        }
        let causes: Vec<_> = dead
            .iter()
            .filter(|(site, _, _)| reaches(cfg, *site, node))
            .collect();
        let Some(last) = causes.iter().max_by_key(|(_, round, _)| *round) else {
            continue;
        };
        out.insert(
            leaf,
            CascadeAttribution {
                erased_lines: causes
                    .iter()
                    .map(|(site, _, _)| line_of_node(*site))
                    .collect(),
                controller_line: dorc_aid::diag::line_col(
                    book_src,
                    ast.node(last.2).span.lo.0 as usize,
                )
                .0,
                round: last.1.0,
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

/// The driver's settlement model: reclassify against the analyzer, re-fold the FROZEN records, and
/// own the run's accumulators (`30K` §4.2).
///
/// Everything the rounds produce beyond the settled answer is stashed here and OVERWRITTEN each
/// round, which is what keeps `the-fixpoint-owns-the-rounds-and-builds-nothing-else` true in the
/// new shape: an intermediate round's classification and fold exist only until the next one
/// replaces them, and no plan, narrative surface, render, or whylog write ever sees one.
#[derive(Debug)]
pub struct WorldRoundModel<'a> {
    frozen: &'a FrozenModel<'a>,
    probe: &'a dorc_plan::ProbePlan,
    results: &'a SiteResults,
    interner: &'a mut Interner,
    arena: &'a mut ProvArena,
    trip: &'a mut dorc_analysis::certify::CertifierTrip,
    round: Option<ClassifiedRound>,
    by_fact: BTreeMap<dorc_core::FactKey, Observable>,
    merge_narrative: Vec<CollapseNarrative>,
    collapsed: BTreeMap<dorc_core::FactKey, u32>,
}

impl dorc_plan::RoundModel for WorldRoundModel<'_> {
    fn classify(
        &mut self,
        erased: &dorc_analysis::erase::ErasedSites,
    ) -> dorc_plan::RoundClassification {
        // `degrades` and `verdict_lane` are the ORIGIN round's products and stay with it: they
        // decide which body SHIPPED, and the probe is frozen.
        let round = classify_round(
            self.frozen,
            erased,
            self.interner,
            self.arena,
            &mut BTreeMap::new(),
            &mut BTreeMap::new(),
            self.trip,
        );
        let classification = dorc_plan::RoundClassification {
            classes: round.classes.clone(),
            kills: round.kills.clone(),
            invalidators: round.invalidators.clone(),
            fact_backings: round.fact_backings.clone(),
        };
        self.round = Some(round);
        classification
    }

    fn fold(&mut self, validity: &BTreeMap<dorc_plan::LeafId, bool>) {
        let (by_fact, merge_narrative, collapsed) =
            facts_from_sites(self.probe, self.results, validity);
        self.by_fact = by_fact;
        self.merge_narrative = merge_narrative;
        self.collapsed = collapsed;
    }

    fn observe(&self, fact: dorc_core::FactKey) -> Observable {
        self.by_fact
            .get(&fact)
            .copied()
            .unwrap_or(Observable::verdict_only(Verdict::Unknown))
    }

    fn trip(&mut self) -> &mut dorc_analysis::certify::CertifierTrip {
        self.trip
    }
}

/// Settle the effective world to quiescence (`30K` §4.2), over the frozen model and the frozen
/// records.
///
/// One loop, not two: every round applies the ledger, re-derives the model, solves effective reach,
/// folds the frozen records through the validity that reach implies, decides every site, and proves
/// what cannot execute. W-C's dead-branch cascade and the effective walls settle TOGETHER, so a
/// Query that becomes valid because a mutation was proven not to run re-enters the dead-branch step
/// rather than arriving after everything was decided.
///
/// NO RE-PROBE (`26H` §0 v-no-reprobe-needed): invalid-Query checks already shipped and their rcs
/// are already measured, merely withheld. This consumes measurements in hand; it never asks a host
/// anything, and `probe` is the frozen origin artifact throughout.
#[expect(
    clippy::too_many_arguments,
    reason = "the frozen model, the probe, the records, and the plan-side inputs are the settlement's inputs; the interner, arena and certifier-trip latch are the three run-scoped accumulators it borrows. Bundling the accumulators would hide exactly which of them a round may write"
)]
pub fn settle_world(
    frozen: &FrozenModel<'_>,
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
    plan_inputs: &dorc_plan::SettleInputs<'_>,
    cap: u32,
    interner: &mut Interner,
    arena: &mut ProvArena,
    trip: &mut dorc_analysis::certify::CertifierTrip,
) -> SettledFixpoint {
    let mut model = WorldRoundModel {
        frozen,
        probe,
        results,
        interner,
        arena,
        trip,
        round: None,
        by_fact: BTreeMap::new(),
        merge_narrative: Vec::new(),
        collapsed: BTreeMap::new(),
    };
    let settlement = dorc_plan::settle_effective_world(plan_inputs, &mut model, cap);
    SettledFixpoint {
        round: model
            .round
            .expect("a settlement runs at least one classification round"),
        by_fact: model.by_fact,
        merge_narrative: model.merge_narrative,
        collapsed: model.collapsed,
        spine: settlement.spine,
        validity: settlement.validity,
        ledger: settlement.ledger,
        capped: settlement.capped,
        origin_validity: settlement.origin_validity,
    }
}
