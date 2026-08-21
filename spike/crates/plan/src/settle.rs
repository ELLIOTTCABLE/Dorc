//! The settlement: one grow-only fixpoint from the frozen world to one certified set of decisions
//! (`30K` §4).
//!
//! Every apply-time answer is derived here, in one loop, from one fact — which mutations may
//! actually execute ([`crate::world`]). A round applies the ledger, re-derives the model, solves
//! effective reach, folds the frozen records through the validity that reach implies, decides every
//! site, and proves what cannot execute. If a round proved something new, every provisional product
//! is DISCARDED and the round runs again; when a round proves nothing new, that round — and only
//! that round — becomes the settled analysis and writes Spine.
//!
//! # Why the provisional round has no Spine
//!
//! `309:law-spine-write-only-during-run` is not a discipline here, it is a type: a
//! [`ProvisionalEffectiveRound`] has no Spine API at all, and the only way to reach one that does
//! is [`ProvisionalEffectiveRound::seal`], which demands a [`Quiescence`] the ledger alone mints.
//! A boolean `settled` field would have been a rule every future maintainer had to remember;
//! an absent method is not.
//!
//! # Termination
//!
//! The ledger holds CFG sites and only grows, so the loop runs at most once per effective
//! invalidator. Each growing round proves at least one more original mutation cannot execute, and
//! effects only ever DISAPPEAR: reaching-wall sets shrink, a Query moves invalid→valid and never
//! back, a fact moves stale→fresh and never back, and a survival is admitted only after the
//! independent reference model has already confirmed it, so nothing later retracts an erasure. The
//! cap exists so a monotonicity regression becomes a degraded answer rather than a hang: hitting it
//! DISCARDS the whole ledger and re-derives the maximal-effects answer, where every
//! mutation-capable original stays active.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::narrative::{ChannelCoverage, DemoteTag};
use dorc_aid::{CollapseKind, CollapseNarrative, SpeechAct};
use dorc_analysis::certify::{CertifierTrip, SolveConsistency};
use dorc_analysis::cfg::{Cfg, CfgNodeId, ExecutionOwner};
use dorc_analysis::effect::SkipClass;
use dorc_core::spine::{Grade, SpineSurvival, SurvivalDemote, SurvivalOutcome};
use dorc_core::{AstId, Channel, FactBacking, FactKey, KindId, LeafId, Observable};
use dorc_syntax::ast::Ast;

use dorc_core::region::ElisionRegion;

use crate::erase::{DeadBranchProof, RoundId, prove_dead_branches};
use crate::region::{
    RegionCensus, RouteAdmission, RouteConclusion, RouteInstance, RoutePopulation,
    RouteRegionProof, SharedConclusion, SharedGuard, SharedRegionDecision, decide_region,
};
use crate::world::{
    EffectiveAct, Freshness, FreshnessSubject, NoExecutionLedger, NoMutationProof, Quiescence,
    ReachingWalls, ReplacementDeathProof, StaleCause, WallPolicy, effective_invalidators,
    solve_reaching_walls,
};
use crate::{
    AggregateEstablish, AggregateEstablishes, ConnectedPipes, Disposition, Spine, Vouches,
    decide_site, leaf_facts, leaf_has_heredoc, site_order,
};

/// One round's PURE DERIVATION of the analyzer model from (frozen inputs, ledger).
///
/// Recomputed from scratch every round, never incrementally patched: the residual model IS the
/// origin model with the ledger's dead branches erased, and nothing else carries across.
#[derive(Debug, Clone)]
pub struct RoundClassification {
    /// Every classified leaf, in classification order.
    pub classes: Vec<(CfgNodeId, SkipClass)>,
    /// Leaves whose effect is a `Kills` — invisible in the `SkipClass`, needed by the footprint lane.
    pub kills: BTreeSet<CfgNodeId>,
    /// Every node that gens into the effective world (Establish / Kill / Opaque), leaves included
    /// and non-leaves too — the population `30K` §3.7's ownership census answers for.
    pub invalidators: BTreeSet<CfgNodeId>,
    /// Each fact's survival-backing provenance.
    pub fact_backings: BTreeMap<FactKey, FactBacking>,
}

/// The two things a settlement round must ask of its caller, and nothing else.
///
/// The loop cannot own these: reclassifying needs the analyzer and the interner, and folding needs
/// the intake's admitted records — both of which live above this crate. Keeping them a TRAIT rather
/// than two closures is what lets the fold stash its result: `observe` answers per cell many times
/// per round, and re-folding for each would be both slow and a second place for a round's
/// observations to come from.
pub trait RoundModel {
    /// Re-derive the model with `erased` applied at the analyzer's one effect seam.
    fn classify(&mut self, erased: &dorc_analysis::erase::ErasedSites) -> RoundClassification;

    /// Fold this round's records through `validity`; every later [`observe`](Self::observe) in the
    /// round answers from the result.
    fn fold(&mut self, validity: &BTreeMap<LeafId, bool>);

    /// This round's observation for one cell. `Verdict::Unknown` for a cell nothing measured.
    fn observe(&self, fact: FactKey) -> Observable;

    /// The run-wide certifier latch. The model OWNS it rather than the loop borrowing one
    /// alongside, because a round reclassifies (which solves, and latches) and then solves again
    /// itself: two independent `&mut` paths to one latch is exactly the shape that does not
    /// borrow-check, and splitting the latch would let a round-2 failure go unobserved.
    fn trip(&mut self) -> &mut CertifierTrip;
}

/// The frozen context a settlement never re-derives (`30K` §4.1).
#[derive(Debug)]
pub struct SettleInputs<'a> {
    /// The book's bytes (the verbatim sh each Step carries).
    pub src: &'a str,
    /// The parsed book.
    pub ast: &'a Ast,
    /// The book's control-flow graph.
    pub cfg: &'a Cfg,
    /// The per-site verdict vouches, built once from the origin model.
    pub vouches: &'a Vouches,
    /// The recognised connected check-pipes.
    pub connected: &'a ConnectedPipes,
    /// How this run treats a mutation that will really execute.
    pub policy: WallPolicy<'a>,
    /// The elision-region census — which authored regions exist and whose route populations are
    /// closed (`plans/30L` §3).
    ///
    /// FROZEN with everything else here, and for the same reason: the population freezes before
    /// round 1, and settlement may prove more regions non-executing but never discover an
    /// invocation or change a binding (`30L` §6). An EMPTY census is the honest answer for a driver
    /// that holds no region information — it decides nothing, so that driver's output is exactly
    /// what it was before regions existed.
    pub regions: &'a RegionCensus,
    /// The influence grade every Spine record this settlement writes carries.
    pub minted_at: Grade,
}

/// What one site's decision established, before anything is written anywhere (`30K` §3.4).
///
/// The disposition and the semantic act are minted TOGETHER by [`decide`], from the same proof.
/// That is what lets effective reach be decision-fed without any outcome becoming a premise: the
/// act is not a reading of the disposition, it is the other half of the same conclusion.
#[derive(Debug)]
struct ProvisionalSiteDecision {
    leaf: LeafId,
    node: CfgNodeId,
    ast: AstId,
    sh: String,
    disposition: Disposition,
    act: EffectiveAct,
    survival: SurvivalAccount,
}

/// What the survival tier concluded about one site, kept out of Spine until the round settles.
#[derive(Debug, Clone, Copy)]
pub(crate) enum SurvivalAccount {
    /// The survival tier said nothing about this site (it was never elision-eligible, or the run
    /// is honest-walls).
    Silent,
    /// An elision that crossed no wall at all.
    Clean,
    /// An elision kept past ≥1 running wall, reference-confirmed.
    SurvivedStandalone,
    /// An atomic aggregate whose every erased establish survived independently.
    SurvivedAggregate { establishes: u32 },
    /// An elision the walls refused.
    Demoted(StaleCause),
}

/// One AUTHORED REGION's settled answer: the one edit every invocation instance agreed to, the
/// instances that licensed it, and the per-instance no-execution proofs it establishes.
///
/// The proofs travel WITH the decision rather than beside it, because
/// `30L:inv-no-posthoc-shared-demotion` is the whole safety argument: no per-instance replacement
/// may enter the ledger before the shared agreement exists, or a later Run meet would have to
/// re-introduce walls and the grow-only proof would be gone.
#[derive(Debug)]
struct ProvisionalRegionDecision {
    region: ElisionRegion,
    ast: AstId,
    sh: String,
    disposition: Disposition,
    /// Which invocation each contributing route executes under, in census order — the route
    /// attribution `30L` §9 records on Spine, and the half `dorc why` walks call-ward.
    routes: Vec<dorc_core::spine::RegionRoute>,
    proofs: Vec<(CfgNodeId, NoMutationProof)>,
}

/// One settled round's decisions, sealed and therefore allowed to write Spine.
///
/// Private constructor: the only route here is [`ProvisionalEffectiveRound::seal`], which consumes
/// a [`Quiescence`] the ledger mints and nothing else can forge.
#[derive(Debug)]
pub struct SettledEffectiveAnalysis {
    decisions: Vec<ProvisionalSiteDecision>,
    regions: Vec<ProvisionalRegionDecision>,
    walls: Vec<(LeafId, Option<ElisionRegion>)>,
    minted_at: Grade,
}

/// One round's decisions, before quiescence is known. Deliberately without a Spine, Plan, render,
/// digest, or iterator API — a convenience there would recreate the projection path the law forbids.
#[derive(Debug)]
pub struct ProvisionalEffectiveRound {
    decisions: Vec<ProvisionalSiteDecision>,
    regions: Vec<ProvisionalRegionDecision>,
    walls: Vec<(LeafId, Option<ElisionRegion>)>,
    minted_at: Grade,
}

/// The cap path intentionally discards every no-execution proof and seals the maximal-effects
/// floor, rather than claiming ordinary ledger quiescence.
struct MaximalEffectsFloor;

impl ProvisionalEffectiveRound {
    /// The proofs this round established, for the ledger — per-site and per-region alike.
    fn no_execution_proofs(&self) -> Vec<(CfgNodeId, NoMutationProof)> {
        self.decisions
            .iter()
            .filter_map(|decision| match &decision.act {
                EffectiveAct::NoMutation(proof) => Some((decision.node, proof.clone())),
                EffectiveAct::MayMutate(_) => None,
            })
            .chain(
                self.regions
                    .iter()
                    .flat_map(|region| region.proofs.iter().cloned()),
            )
            .collect()
    }

    /// Seal this round. The witness is the whole gate: a round that proved something new cannot
    /// produce one, so it cannot reach any Spine setter.
    #[must_use]
    fn seal(self, _quiescent: Quiescence) -> SettledEffectiveAnalysis {
        SettledEffectiveAnalysis {
            decisions: self.decisions,
            regions: self.regions,
            walls: self.walls,
            minted_at: self.minted_at,
        }
    }

    fn seal_floor(self, _floor: MaximalEffectsFloor) -> SettledEffectiveAnalysis {
        SettledEffectiveAnalysis {
            decisions: self.decisions,
            regions: self.regions,
            walls: self.walls,
            minted_at: self.minted_at,
        }
    }
}

impl SettledEffectiveAnalysis {
    /// Write the settled decisions, survival outcomes, and wall narration onto a fresh Spine
    /// (`309:law-spine-write-only-during-run`).
    #[must_use]
    pub fn write_spine(self) -> Spine {
        let mut spine = Spine::minted_at(self.minted_at);
        for (leaf, region) in self.walls {
            spine.push_narrative(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::WallFormation {
                    participant: leaf,
                    region,
                    channel: ChannelCoverage {
                        channel: Channel::Effect,
                    },
                },
            ));
        }
        for region in self.regions {
            spine.push_region_decision(dorc_core::spine::SpineRegionDecision {
                region: region.region,
                ast: region.ast,
                sh: region.sh,
                decision: region.disposition,
                routes: dorc_core::spine::Account::capped(region.routes),
                grade: None,
            });
        }
        for decision in self.decisions {
            record_survival(&mut spine, decision.leaf, decision.survival);
            spine.set_disposition(dorc_core::spine::SpineDisposition {
                site: dorc_core::SiteId::leaf(decision.leaf),
                ast: decision.ast,
                sh: decision.sh,
                decision: decision.disposition,
                grade: None,
            });
        }
        spine
    }
}

/// Record one site's survival outcome on both planes — the decision record, and its narration.
fn record_survival(spine: &mut Spine, leaf: LeafId, account: SurvivalAccount) {
    let outcome = match account {
        SurvivalAccount::Silent => return,
        SurvivalAccount::Clean => SurvivalOutcome::Clean,
        SurvivalAccount::SurvivedStandalone => SurvivalOutcome::SurvivedStandalone,
        SurvivalAccount::SurvivedAggregate { establishes } => {
            SurvivalOutcome::SurvivedAggregate { establishes }
        }
        SurvivalAccount::Demoted(StaleCause::RederivationDisagreed { wall }) => {
            SurvivalOutcome::RederivationDisagreed { wall }
        }
        SurvivalAccount::Demoted(cause) => SurvivalOutcome::Demoted(match cause {
            StaleCause::Poisoned { .. } => SurvivalDemote::Poisoned,
            StaleCause::MayAlias => SurvivalDemote::MayAlias,
            // Same FLOOR as a total wall, its own NAME: calling our defect a wall points an admin
            // at their own mutators (`302` §5 admin-honesty · `271:rul-sin-ordering`).
            StaleCause::SolveInconsistent => SurvivalDemote::SolveInconsistent,
            StaleCause::TotalWall | StaleCause::RederivationDisagreed { .. } => {
                SurvivalDemote::TotalWall
            }
        }),
    };
    let poisoned_by: Option<KindId> = match account {
        SurvivalAccount::Demoted(StaleCause::Poisoned { via_reach }) => via_reach,
        _ => None,
    };
    spine.push_survival(SpineSurvival {
        leaf,
        outcome,
        poisoned_by,
        grade: None,
    });
    if let SurvivalAccount::Demoted(cause) = account {
        spine.push_narrative(CollapseNarrative::new(
            SpeechAct::Derived,
            CollapseKind::Demotion {
                site: dorc_aid::diag::SiteId::leaf(leaf),
                reason: match cause {
                    StaleCause::Poisoned { .. } => DemoteTag::Poisoned,
                    StaleCause::MayAlias => DemoteTag::MayAlias,
                    StaleCause::RederivationDisagreed { .. } => DemoteTag::RederivationDisagreement,
                    StaleCause::SolveInconsistent => DemoteTag::SolveInconsistent,
                    StaleCause::TotalWall => DemoteTag::TotalWall,
                },
            },
        ));
    }
}

/// Settle the effective world and write its Spine (`30K` §4.2 — the one grow-only loop).
///
/// `cap` bounds the rounds; hitting it discards the ledger and re-derives the maximal-effects
/// answer, so a monotonicity regression degrades rather than hangs. `capped` says whether that
/// happened, for the caller's own `debug_assert` and narration.
pub fn settle_effective_world(
    inputs: &SettleInputs<'_>,
    model: &mut dyn RoundModel,

    cap: u32,
) -> Settlement {
    let mut ledger = NoExecutionLedger::new();
    let mut number = 1u32;
    let mut origin_validity: Option<BTreeMap<LeafId, bool>> = None;
    // `30L:req-backings-freeze-at-probe-boundary` — every round consumes ONE backing account.
    let mut origin_backings: Option<BTreeMap<FactKey, FactBacking>> = None;
    let mut failures = 0u32;
    loop {
        let outcome = one_round(inputs, model, &ledger, origin_backings.as_ref());
        failures = failures.saturating_add(outcome.solve_failures);
        if origin_validity.is_none() {
            origin_validity = Some(outcome.validity.clone());
        }
        if origin_backings.is_none() {
            origin_backings = Some(outcome.classification.fact_backings.clone());
        }
        let quiescent = ledger.record_round(RoundId(number), outcome.round.no_execution_proofs());
        if let Some(witness) = quiescent {
            return Settlement {
                spine: outcome.round.seal(witness).write_spine(),
                classification: outcome.classification,
                validity: outcome.validity,
                origin_validity: origin_validity.unwrap_or_default(),
                ledger,
                capped: false,
                discarded_on_cap: 0,
                effective_solve_failures: failures,
            };
        }
        if number >= cap {
            let discarded = u32::try_from(ledger.len()).unwrap_or(u32::MAX);
            ledger.rebuild_from_origin();
            let outcome = one_round(inputs, model, &ledger, origin_backings.as_ref());
            failures = failures.saturating_add(outcome.solve_failures);
            return Settlement {
                spine: outcome.round.seal_floor(MaximalEffectsFloor).write_spine(),
                classification: outcome.classification,
                validity: outcome.validity,
                origin_validity: origin_validity.unwrap_or_default(),
                ledger,
                capped: true,
                discarded_on_cap: discarded,
                effective_solve_failures: failures,
            };
        }
        number = number.saturating_add(1);
    }
}

/// What the settlement settled on: the Spine it wrote, plus the FINAL round's model and validity
/// view, which every surface above this one reports from.
#[derive(Debug)]
pub struct Settlement {
    /// The settled decisions, written once.
    pub spine: Spine,
    /// The final round's model.
    pub classification: RoundClassification,
    /// The final round's effective Query validity, per leaf.
    pub validity: BTreeMap<LeafId, bool>,
    /// Round 1's validity — kept only so a cascade can tell a site that was always trustworthy from
    /// one that became trustworthy because something upstream was proven not to run.
    pub origin_validity: BTreeMap<LeafId, bool>,
    /// Everything the rounds proved cannot execute, round-tagged.
    pub ledger: NoExecutionLedger,
    /// Did the loop hit its cap and degrade to the maximal-effects answer?
    pub capped: bool,
    /// How many proofs the cap DISCARDED — captured before the ledger was rebuilt, because the
    /// number the narrative owes its reader is what was withdrawn, not what survived (nothing does).
    pub discarded_on_cap: u32,
    /// How many effective-reach post-fixpoint CHECKS failed across every round (`30K` §4.4).
    ///
    /// A scalar, and accumulated across ALL rounds rather than kept from the settled one, for the
    /// reason the certifier latch is threaded the same way: an intermediate round is never
    /// observed, so a failure there would otherwise be invisible to every reader of the settled
    /// answer. Scalars only — the failing lattice values stay behind in the `SolveConsistency`
    /// (`303:fnd-witness-operands-cannot-enter-narrative`).
    pub effective_solve_failures: u32,
}

/// One members-site's SELF-SUPPRESSED reach answer with the certification that decides whether it
/// may be read at all — together, because reading either alone is the defect.
struct MembersAnswer {
    walls: ReachingWalls,
    consistency: SolveConsistency<ReachingWalls>,
}

/// One round's products, none of which survives a growing ledger.
struct RoundOutcome {
    round: ProvisionalEffectiveRound,
    classification: RoundClassification,
    validity: BTreeMap<LeafId, bool>,
    /// Failing post-fixpoint CHECKS across this round's effective solves.
    solve_failures: u32,
}

#[expect(
    clippy::too_many_lines,
    reason = "one round keeps the effective solve, validity fold, decisions, and proof collection in their required causal order"
)]
fn one_round(
    inputs: &SettleInputs<'_>,
    model: &mut dyn RoundModel,
    ledger: &NoExecutionLedger,
    frozen_backings: Option<&BTreeMap<FactKey, FactBacking>>,
) -> RoundOutcome {
    let (ast, cfg) = (inputs.ast, inputs.cfg);
    let classification = model.classify(&ledger.classify_overlay());
    let ordered = site_order(ast, cfg, &classification.classes);
    let leaf_of: BTreeMap<CfgNodeId, LeafId> = ordered
        .iter()
        .map(|(leaf, node, _)| (*node, *leaf))
        .collect();

    // Round 1 IS the origin, so it supplies its own frozen backings.
    let backings = frozen_backings.unwrap_or(&classification.fact_backings);
    // The one fact. Everything below reads it and nothing re-derives a second answer.
    let effective = effective_invalidators(cfg, &classification.invalidators, ledger);
    let nothing_suppressed = BTreeSet::new();
    let (reach, consistency) = solve_reaching_walls(cfg, &effective, &nothing_suppressed);
    model.trip().record(&consistency);
    let mut solve_failures = failing_checks(&consistency);
    let trusted = consistency.is_consistent();
    let walls_at = |node: CfgNodeId| -> ReachingWalls {
        reach
            .states
            .get(node.index())
            .cloned()
            .unwrap_or_else(dorc_analysis::lattice::Lattice::bottom)
    };

    // The Members lanes answer from a SELF-SUPPRESSED solve: an in-loop aggregate's own writes
    // return to it over the back-edge, and its own elision is what removes them — the fixed-point
    // argument the Members license has always rested on (`effect::self_reach_holds`).
    let mut members_answers: BTreeMap<CfgNodeId, MembersAnswer> = BTreeMap::new();
    for (_, node, class) in &ordered {
        if !matches!(class, SkipClass::EstablishMembers { .. }) {
            continue;
        }
        let (solo, consistency) = solve_reaching_walls(cfg, &effective, &BTreeSet::from([*node]));
        model.trip().record(&consistency);
        solve_failures = solve_failures.saturating_add(failing_checks(&consistency));
        let walls = solo
            .states
            .get(node.index())
            .cloned()
            .unwrap_or_else(dorc_analysis::lattice::Lattice::bottom);
        members_answers.insert(*node, MembersAnswer { walls, consistency });
    }

    // Effective Query validity (`30K` §5.2): a probed rc is fold-usable iff no mutation that may
    // execute reaches its guard. Footprint disjointness deliberately does NOT relax this — using
    // it to revive a Query is a separate license widening, not a consequence of this one.
    let mut validity: BTreeMap<LeafId, bool> = BTreeMap::new();
    let mut valid_at: BTreeMap<CfgNodeId, bool> = BTreeMap::new();
    for (leaf, node, class) in &ordered {
        if matches!(class, SkipClass::QueryResolvable { .. }) {
            let valid = trusted && walls_at(*node).is_empty();
            validity.insert(*leaf, valid);
            valid_at.insert(*node, valid);
        }
    }
    model.fold(&validity);
    let observe = |fact: FactKey| model.observe(fact);

    let leaf_fact = leaf_facts(cfg, &classification.classes);
    let fold = crate::fold::fold(ast, |leaf| leaf_fact.get(&leaf).map(|f| observe(*f)));
    // The dead-branch derivations, taken BEFORE any decision so an `Omit`'s act rests on a proof
    // rather than on the disposition it is about to sit beside (`pin-no-outcome-as-generator`).
    let dead: BTreeMap<CfgNodeId, DeadBranchProof> = prove_dead_branches(
        ast,
        cfg,
        &classification.classes,
        &classification.invalidators,
        &valid_at,
        observe,
    )
    .into_iter()
    .map(|proof| (proof.site(), proof))
    .collect();

    let accounts_survival = matches!(inputs.policy, WallPolicy::RiskAccepted { .. });
    let mut decisions = Vec::with_capacity(ordered.len());
    let mut walls = Vec::new();
    for (leaf, node, class) in &ordered {
        let ast_id = cfg.node(*node).ast;
        let members = match class {
            SkipClass::EstablishMembers { .. } => members_answers.get(node),
            _ => None,
        };
        let site_walls = members.map_or_else(|| walls_at(*node), |answer| answer.walls.clone());
        let aggregate_establishes = aggregate_establishes(*node, class);
        let subject = match (class, aggregate_establishes.as_ref()) {
            (
                SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact),
                _,
            ) => FreshnessSubject::Standalone(*fact),
            (SkipClass::EstablishMembers { .. } | SkipClass::InlineCall { .. }, Some(entries)) => {
                FreshnessSubject::Aggregate(entries)
            }
            _ => FreshnessSubject::None,
        };
        // BOTH certifications floor this site: the self-suppressed solo solve is a SECOND answer,
        // and the window's certification says nothing about it (`30Mb:fnd-members-floor-is-a-sentinel`).
        let policy_answer = inputs
            .policy
            .freshness(&site_walls, subject, backings, &leaf_of);
        let freshness = members.map_or_else(
            || policy_answer.clone(),
            |answer| members_freshness(answer, policy_answer.clone()),
        );
        let freshness = floor_uncertified(&consistency, freshness);
        let owns_invalidator = classification.invalidators.iter().any(|invalidator| {
            matches!(cfg.execution_owner(*invalidator), ExecutionOwner::Leaf(owner) if owner == *node)
        });
        let decision = decide_site(&crate::DecideSite {
            cfg,
            ast,
            fold: &fold,
            node: *node,
            ast_id,
            class,
            freshness: &freshness,
            vouches: inputs.vouches,
            connected: inputs.connected,
            observe: &observe,
            valid_at: &valid_at,
            leaf_fact: &leaf_fact,
            dead: dead.get(node),
            invalidator: owns_invalidator,
            accounts_survival,
            aggregate_establishes: aggregate_establishes.as_ref(),
        });
        let disposition = decision.disposition;
        // `30K` §7 asks for a wall-formation account per effective mutation act; it is minted only
        // under the risk-accepted policy, exactly where it was minted before. DEVIATION, reported:
        // nothing consumes the record yet (`289:seam-narrative-render-unconsumed`), so widening it
        // to the honest path buys no account and costs every why-transcript an `[unnarrated: …]`
        // line. It widens with its consumer, not ahead of one.
        // `30L:req-wall-narrative-gains-region-operand` — the OPERAND only; the gate is untouched.
        if accounts_survival && let EffectiveAct::MayMutate(wall) = decision.act {
            walls.push((*leaf, region_of_node(inputs.regions, wall.node())));
        }
        decisions.push(ProvisionalSiteDecision {
            leaf: *leaf,
            node: *node,
            ast: ast_id,
            sh: crate::command_text(inputs.src, ast, ast_id),
            disposition,
            act: decision.act,
            survival: decision.survival,
        });
    }
    let regions = decide_regions(&RegionRound {
        inputs,
        classification: &classification,
        backings,
        effective: &effective,
        window: &consistency,
        reach: &reach,
        leaf_of: &leaf_of,
        fold: &fold,
        observe: &observe,
        valid_at: &valid_at,
        leaf_fact: &leaf_fact,
        dead: &dead,
        accounts_survival,
    });
    RoundOutcome {
        round: ProvisionalEffectiveRound {
            decisions,
            regions,
            walls,
            minted_at: inputs.minted_at,
        },
        classification,
        validity,
        solve_failures,
    }
}

/// Which authored region a CFG node executes, when it executes one at all.
fn region_of_node(census: &RegionCensus, node: CfgNodeId) -> Option<ElisionRegion> {
    census
        .regions()
        .find_map(|(region, population)| match population {
            RoutePopulation::Closed(routes) => routes
                .routes()
                .any(|route| route.cfg_node() == node)
                .then_some(*region),
            RoutePopulation::Open => None,
        })
}

/// Everything one round's region pass reads. A struct because it is genuinely the whole round —
/// bundling it is what keeps the pass's inputs visibly the SAME ones the site pass already
/// consumed, rather than a second derivation of the world.
struct RegionRound<'a> {
    inputs: &'a SettleInputs<'a>,
    classification: &'a RoundClassification,
    backings: &'a BTreeMap<FactKey, FactBacking>,
    effective: &'a BTreeSet<CfgNodeId>,
    window: &'a SolveConsistency<ReachingWalls>,
    reach: &'a dorc_analysis::solve::Solution<ReachingWalls>,
    leaf_of: &'a BTreeMap<CfgNodeId, LeafId>,
    fold: &'a crate::FoldResult,
    observe: &'a dyn Fn(FactKey) -> Observable,
    valid_at: &'a BTreeMap<CfgNodeId, bool>,
    leaf_fact: &'a BTreeMap<AstId, FactKey>,
    dead: &'a BTreeMap<CfgNodeId, DeadBranchProof>,
    accounts_survival: bool,
}

/// Decide every closed-population elision region: per-instance route proofs, the universal meet, and
/// the lowering into one license plus per-instance no-execution proofs (`30L` §6, steps 1–5).
///
/// An OPEN population is absent here rather than recorded as Run: an unenumerated invocation forces
/// Run for every region it may execute (`30L:pin-open-route-runs`), and a Run region is a region
/// whose authored bytes ship untouched — which is exactly the artifact a region nobody decided
/// produces. Nothing is hidden either way (`rul-attention-honesty`).
///
/// # The self-suppressed solve
///
/// Sibling instances of ONE region write to each other along the ordinary sequence, and the region's
/// own ATOMIC replacement is what removes them — so its freshness is answered with the whole
/// population silenced, the shape `effect::self_reach_holds` already takes one level down. Only for
/// a plural population (a lone instance is never in its own in-state), and only ever read beside
/// that solve's OWN certification.
///
/// # The shared guard's economics
///
/// `30L` §4.5 licenses the valve for DIVERGENT instances, and only those. Where some route measured
/// converged and a sibling did not, one parametric check recovers the converged invocation at apply.
/// Where NO route converged, the check is known to fall through at every invocation: it buys
/// nothing and costs the tax at each of them, which is the site tier's own `jc-mint-policy m-a`
/// reading and not something the region tier may undercut. So the candidates drop BEFORE the meet,
/// and the decision Spine records is the decision the artifact carries.
fn decide_regions(round: &RegionRound<'_>) -> Vec<ProvisionalRegionDecision> {
    let (ast, cfg, src) = (round.inputs.ast, round.inputs.cfg, round.inputs.src);
    // A spliced body site is not a plan leaf, so its class lives inside its owning CALL's aggregate.
    let body_class: BTreeMap<CfgNodeId, &SkipClass> = round
        .classification
        .classes
        .iter()
        .filter_map(|(_, class)| match class {
            SkipClass::InlineCall { sites } => Some(sites),
            _ => None,
        })
        .flatten()
        .map(|site| (site.node, &site.class))
        .collect();

    let mut decided = Vec::new();
    for (region, population) in round.inputs.regions.regions() {
        let RoutePopulation::Closed(routes) = population else {
            continue;
        };
        let Some(region_ast) = ast_of_region(cfg, routes.routes().map(|route| route.cfg_node()))
        else {
            continue;
        };
        let argv = crate::source_argv(src, ast, region_ast);
        let suppress: BTreeSet<CfgNodeId> = routes.routes().map(|route| route.cfg_node()).collect();
        let solo =
            (routes.count() > 1).then(|| solve_reaching_walls(cfg, round.effective, &suppress));
        let mut answers = Vec::with_capacity(routes.count());
        for route in routes.routes() {
            answers.push(decide_one_route(
                round,
                *route,
                &body_class,
                solo.as_ref(),
                argv.as_deref(),
            ));
        }
        if !answers
            .iter()
            .any(|answer| answer.verdict == dorc_core::Verdict::Converged)
        {
            for answer in &mut answers {
                answer.guard = None;
            }
        }
        let proofs: Vec<RouteRegionProof> = routes
            .routes()
            .zip(answers.iter())
            .map(|(route, answer)| {
                RouteRegionProof::new(
                    *route,
                    RouteAdmission::project(
                        &answer.conclusion,
                        answer
                            .guard
                            .as_ref()
                            .map(|license| SharedGuard::of(license.canonical())),
                    ),
                    round.inputs.minted_at,
                )
            })
            .collect();
        let decision = decide_region(*region, population, &proofs);
        let sh = crate::command_text(src, ast, region_ast);
        let (disposition, proofs) = lower_shared_decision(round, &decision, &answers, region_ast);
        decided.push(ProvisionalRegionDecision {
            region: *region,
            ast: region_ast,
            sh,
            disposition,
            routes: routes
                .routes()
                .filter_map(|route| {
                    let call = route.invocation().node();
                    Some(dorc_core::spine::RegionRoute {
                        invocation: dorc_core::SiteId::leaf(round.leaf_of.get(&call).copied()?),
                        ast: cfg.node(call).ast,
                    })
                })
                .collect(),
            proofs,
        });
    }
    decided
}

/// The one authored AST node every instance of a region shares — the edit unit.
fn ast_of_region(cfg: &Cfg, instances: impl Iterator<Item = CfgNodeId>) -> Option<AstId> {
    let mut ids = instances.map(|node| cfg.node(node).ast);
    let first = ids.next()?;
    // Clones lower the SAME subtree; a disagreeing population has no single span to be given.
    ids.all(|id| id == first).then_some(first)
}

/// One instance's answer, through the ordinary site seat.
fn decide_one_route(
    round: &RegionRound<'_>,
    route: RouteInstance,
    body_class: &BTreeMap<CfgNodeId, &SkipClass>,
    solo: Option<&(
        dorc_analysis::solve::Solution<ReachingWalls>,
        SolveConsistency<ReachingWalls>,
    )>,
    argv: Option<&str>,
) -> crate::RouteDecision {
    let node = route.cfg_node();
    let Some(class) = body_class.get(&node).copied() else {
        // No aggregate class admits nothing, so the region meets to Run: covered exactly, or not.
        return crate::RouteDecision {
            conclusion: RouteConclusion::Run,
            guard: None,
            establish: None,
            verdict: dorc_core::Verdict::Unknown,
        };
    };
    let at = |states: &dorc_analysis::solve::Solution<ReachingWalls>| {
        states
            .states
            .get(node.index())
            .cloned()
            .unwrap_or_else(dorc_analysis::lattice::Lattice::bottom)
    };
    let walls = solo
        .as_ref()
        .map_or_else(|| at(round.reach), |(s, _)| at(s));
    let subject = match class {
        SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
            FreshnessSubject::Standalone(*fact)
        }
        _ => FreshnessSubject::None,
    };
    let answer = round
        .inputs
        .policy
        .freshness(&walls, subject, round.backings, round.leaf_of);
    // BOTH certifications floor it, for the Members lane's reason: a solo is a SECOND answer.
    let freshness = floor_uncertified(round.window, answer);
    let freshness = match solo {
        Some((_, consistency)) => floor_uncertified(consistency, freshness),
        None => freshness,
    };
    crate::decide_route(
        &crate::DecideSite {
            cfg: round.inputs.cfg,
            ast: round.inputs.ast,
            fold: round.fold,
            node,
            ast_id: round.inputs.cfg.node(node).ast,
            class,
            freshness: &freshness,
            vouches: round.inputs.vouches,
            connected: round.inputs.connected,
            observe: round.observe,
            valid_at: round.valid_at,
            leaf_fact: round.leaf_fact,
            dead: round.dead.get(&node),
            invalidator: round.classification.invalidators.contains(&node),
            accounts_survival: round.accounts_survival,
            aggregate_establishes: None,
        },
        argv,
    )
}

/// Lower one shared conclusion into the artifact's disposition and the ledger's proofs
/// (`30L` §6 steps 4–5; `30N:rul-license-mints-at-settlement-from-shared-conclusion`).
///
/// The real license mints HERE, from the PRIVATE conclusion plus the cross-instance witness, and
/// never from the public outcome (`pin-no-outcome-as-generator`). The proofs are all-or-nothing by
/// construction: one license, then one death proof per instance, and an empty vector wherever the
/// license did not mint.
///
/// The GUARD arm takes any instance's license, because the meet already proved every instance admits
/// byte-identical guard bytes. Only the DISCLOSED probe word is re-stamped, to the population's
/// join: no single word is true of a region whose invocations answered differently, and picking one
/// route's would misattribute the others'.
///
/// The OMIT arm is NAMED RESIDUE, and a run floor rather than a gap. A region-tier omit needs a
/// `DeadBranchProof` per instance and a controller the artifact really neutralises, and the fold
/// cannot reach inside a spliced body today — its statuses key by the leaves it classified, and a
/// body site is not one. So the arm is unreachable; if it ever fires the region renders and retires
/// nothing, which is the safe direction rather than a silent `:`.
fn lower_shared_decision(
    round: &RegionRound<'_>,
    decision: &SharedRegionDecision,
    answers: &[crate::RouteDecision],
    region_ast: AstId,
) -> (Disposition, Vec<(CfgNodeId, NoMutationProof)>) {
    match decision.conclusion() {
        SharedConclusion::Replace(stand_in) => {
            match shared_replacement(round, answers, region_ast) {
                Some((license, proofs)) => {
                    (Disposition::Replace(license, stand_in.stand_in()), proofs)
                }
                None => (Disposition::Run, Vec::new()),
            }
        }
        SharedConclusion::Guard(_) => {
            match answers.iter().find_map(|answer| answer.guard.clone()) {
                Some(license) => (
                    Disposition::Guard(license.with_probe_verdict(joined_verdict(answers))),
                    Vec::new(),
                ),
                None => (Disposition::Run, Vec::new()),
            }
        }
        SharedConclusion::Omit { .. } | SharedConclusion::Run => (Disposition::Run, Vec::new()),
    }
}

/// What the world said about a whole region: the common answer, or `Unknown` where its routes
/// answered differently.
fn joined_verdict(answers: &[crate::RouteDecision]) -> dorc_core::Verdict {
    let mut verdicts = answers.iter().map(|answer| answer.verdict);
    let first = verdicts.next().unwrap_or(dorc_core::Verdict::Unknown);
    if verdicts.all(|verdict| verdict == first) {
        first
    } else {
        dorc_core::Verdict::Unknown
    }
}

/// The shared replacement's license and its per-instance death proofs, or nothing.
///
/// `consumed` is the UNION over every instance: one authored edit answers for every call context at
/// once, and a union can only block. The status is ⊤ (`fork-mutator-rc`), which each instance's own
/// license already proved its consumers can live with.
fn shared_replacement(
    round: &RegionRound<'_>,
    answers: &[crate::RouteDecision],
    region_ast: AstId,
) -> Option<(crate::ReplaceLicense, Vec<(CfgNodeId, NoMutationProof)>)> {
    let establishes: Option<Vec<AggregateEstablish>> = answers
        .iter()
        .map(|answer| {
            answer
                .establish
                .map(|(node, fact)| AggregateEstablish::new(node, fact))
        })
        .collect();
    let establishes = AggregateEstablishes::mint(establishes?)?;
    let all_vouched = crate::AllEstablishesVouched::mint(&establishes, round.inputs.vouches)?;
    let mut consumed = dorc_analysis::lattice::Powerset::default();
    for establish in establishes.iter() {
        for channel in round
            .inputs
            .cfg
            .consumed_observables(establish.site())
            .iter()
        {
            consumed.insert(*channel);
        }
    }
    let license = crate::ReplaceLicense::prove_shared_region_replaceable(
        all_vouched,
        &dorc_analysis::lattice::May(consumed),
        dorc_core::Predicted::Top,
    )?;
    let proofs: Option<Vec<_>> = establishes
        .iter()
        .map(|establish| {
            replacement_death(round.inputs.ast, establish.site(), region_ast, &license)
                .map(|proof| (establish.site(), NoMutationProof::Replaced(proof)))
        })
        .collect();
    Some((license, proofs?))
}

/// How many post-fixpoint checks one effective-reach answer failed — the scalar the aid plane may
/// carry (`303:fnd-witness-operands-cannot-enter-narrative`). Zero for a certified answer.
fn failing_checks(consistency: &SolveConsistency<ReachingWalls>) -> u32 {
    match consistency {
        SolveConsistency::Consistent(_) => 0,
        SolveConsistency::Inconsistent(report) => u32::try_from(report.total()).unwrap_or(u32::MAX),
    }
}

/// THE EFFECTIVE-REACH FLOOR (`30K` §4.4). An answer that failed its own post-fixpoint check is
/// inadmissible for freshness or survival: every fact reads STALE across potential mutations, so
/// nothing Replaces, Survives, or Omits on its strength. A guard may still stand — it rests on the
/// independent vouch and the probe's own measurement, and it re-decides live — and otherwise the
/// site runs.
///
/// A named seat rather than an inline `if`, for the reason `effect::self_reach_answer` is one: the
/// load-bearing half is that a CLEAN policy answer does not rescue an uncertified solve, and only a
/// test holding both can say so.
pub(crate) fn floor_uncertified(
    consistency: &SolveConsistency<ReachingWalls>,
    answer: Freshness,
) -> Freshness {
    if consistency.is_consistent() {
        answer
    } else {
        Freshness::Stale(StaleCause::SolveInconsistent)
    }
}

/// THE MEMBERS FLOOR (`30Mb:fnd-members-floor-is-a-sentinel`): the policy's answer over a members
/// site's SELF-SUPPRESSED walls, floored by that SOLO solve's own certification.
///
/// A second seat because it is a second certification, and the window's says nothing about it. The
/// retired shape substituted a synthetic unresolvable wall, which reached run-or-guard only because
/// the current footprint lift cannot resolve one — an accident rather than a floor, and one that
/// narrated OUR solver defect as a wall in the admin's book.
fn members_freshness(answer: &MembersAnswer, policy_answer: Freshness) -> Freshness {
    floor_uncertified(&answer.consistency, policy_answer)
}

/// The one exact aggregate identity shared by freshness and vouch authorization.
fn aggregate_establishes(node: CfgNodeId, class: &SkipClass) -> Option<AggregateEstablishes> {
    match class {
        SkipClass::EstablishMembers { members, .. } => AggregateEstablishes::mint(
            members
                .iter()
                .map(|fact| AggregateEstablish::new(node, *fact))
                .collect(),
        ),
        SkipClass::InlineCall { sites } => AggregateEstablishes::mint(
            sites
                .iter()
                .filter_map(|site| match site.class {
                    SkipClass::EstablishProbeAmbient(fact)
                    | SkipClass::EstablishProbeWritten(fact) => {
                        Some(AggregateEstablish::new(site.node, fact))
                    }
                    _ => None,
                })
                .collect(),
        ),
        SkipClass::EstablishProbeAmbient(_)
        | SkipClass::EstablishProbeWritten(_)
        | SkipClass::QueryResolvable { .. }
        | SkipClass::MustRun => None,
    }
}

/// Whether a licensed replacement's emitted artifact really neutralises the original bytes — the
/// render's OWN refusal predicate, consulted at the decision seat rather than a second
/// implementation of it (`30K` §2.4). A heredoc-carrying leaf renders verbatim, so a replacement
/// there proves nothing about what executes.
pub(crate) fn replacement_renders_dead(ast: &Ast, leaf: AstId) -> bool {
    !leaf_has_heredoc(ast, leaf)
}

/// Mint the replacement-death proof for a site whose decision replaced it. The ONE caller of
/// [`ReplacementDeathProof::mint`] (`replacement_death_mint_has_exactly_one_caller`).
pub(crate) fn replacement_death(
    ast: &Ast,
    node: CfgNodeId,
    ast_id: AstId,
    license: &crate::ReplaceLicense,
) -> Option<ReplacementDeathProof> {
    ReplacementDeathProof::mint(node, license, replacement_renders_dead(ast, ast_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// THE EFFECTIVE-REACH FLOOR, driven by a REAL `Inconsistent` (`30K` §4.4 · `302` §6.8's
    /// shape): a clean policy answer does not rescue an answer that failed its own post-fixpoint
    /// check. Both halves have to be held at once for the pin to mean anything, which is why the
    /// inconsistency comes from the real checker over a real (deliberately wrong) solution rather
    /// than from a hand-built outcome (`anti-masking-tests`).
    #[test]
    fn an_uncertified_effective_answer_makes_every_fact_stale() {
        use dorc_analysis::certify::certify_solution;
        use dorc_analysis::lattice::Lattice as _;
        use dorc_analysis::solve::{Direction, Solution};

        // A two-node line graph whose transfer gens a wall at node 0, against a solution claiming
        // the wall reaches nowhere: the per-edge check `transfer(0, s0) ⊑ s1` fails for real.
        struct Line;
        impl dorc_analysis::solve::Graph for Line {
            fn node_count(&self) -> usize {
                2
            }
            fn succ(&self, node: usize) -> &[usize] {
                if node == 0 { &[1] } else { &[] }
            }
            fn pred(&self, node: usize) -> &[usize] {
                if node == 1 { &[0] } else { &[] }
            }
        }
        let transfer = |node: usize, incoming: &ReachingWalls| {
            let mut out = incoming.clone();
            if node == 0 {
                out.insert(crate::world::WallId::of(CfgNodeId(0)));
            }
            out
        };
        let wrong = Solution {
            states: vec![ReachingWalls::bottom(), ReachingWalls::bottom()],
            converged: true,
            rounds: 1,
        };
        let init = vec![ReachingWalls::bottom(); 2];
        let consistency = certify_solution(&Line, Direction::Forward, &init, transfer, &wrong);
        assert!(
            !consistency.is_consistent(),
            "the fixture must produce a REAL inconsistency, or the pin proves nothing"
        );
        assert!(
            matches!(
                floor_uncertified(&consistency, Freshness::FreshClean),
                Freshness::Stale(StaleCause::SolveInconsistent)
            ),
            "a clean policy answer must not rescue an uncertified solve"
        );
        assert_eq!(failing_checks(&consistency), 1, "and the count is measured");
        let clean = certify_solution(
            &Line,
            Direction::Forward,
            &init,
            transfer,
            &Solution {
                states: vec![
                    ReachingWalls::bottom(),
                    ReachingWalls::singleton(crate::world::WallId::of(CfgNodeId(0))),
                ],
                converged: true,
                rounds: 1,
            },
        );
        assert!(
            matches!(
                floor_uncertified(&clean, Freshness::FreshClean),
                Freshness::FreshClean
            ),
            "and a certified answer passes through untouched — the floor is not a blanket refuse"
        );
    }

    /// THE MEMBERS FLOOR, on its own certification: an uncertified solo floors the site whatever
    /// the policy answered over its walls, and those walls are the solo's real ones rather than a
    /// synthetic sentinel (`30Mb:fnd-members-floor-is-a-sentinel`). Unreachable from any book
    /// today, which is why the SEAT is pinned rather than trusted to a corpus that cannot reach it.
    #[test]
    fn an_uncertified_members_solo_floors_the_site_whatever_the_walls_say() {
        use dorc_analysis::certify::certify_solution;
        use dorc_analysis::lattice::Lattice as _;
        use dorc_analysis::solve::{Direction, Solution};

        struct Line;
        impl dorc_analysis::solve::Graph for Line {
            fn node_count(&self) -> usize {
                2
            }
            fn succ(&self, node: usize) -> &[usize] {
                if node == 0 { &[1] } else { &[] }
            }
            fn pred(&self, node: usize) -> &[usize] {
                if node == 1 { &[0] } else { &[] }
            }
        }
        let transfer = |node: usize, incoming: &ReachingWalls| {
            let mut out = incoming.clone();
            if node == 0 {
                out.insert(crate::world::WallId::of(CfgNodeId(0)));
            }
            out
        };
        let init = vec![ReachingWalls::bottom(); 2];
        let uncertified = certify_solution(
            &Line,
            Direction::Forward,
            &init,
            transfer,
            &Solution {
                states: vec![ReachingWalls::bottom(), ReachingWalls::bottom()],
                converged: true,
                rounds: 1,
            },
        );
        assert!(!uncertified.is_consistent(), "the fixture must really fail");

        // EMPTY walls — the most permissive thing the policy could possibly have answered over,
        // and under the retired sentinel the one shape that could never occur.
        let answer = MembersAnswer {
            walls: ReachingWalls::bottom(),
            consistency: uncertified,
        };
        assert!(
            matches!(
                members_freshness(&answer, Freshness::FreshClean),
                Freshness::Stale(StaleCause::SolveInconsistent)
            ),
            "an uncertified solo floors the site, and names the solver rather than a wall"
        );

        let certified = certify_solution(
            &Line,
            Direction::Forward,
            &init,
            transfer,
            &Solution {
                states: vec![
                    ReachingWalls::bottom(),
                    ReachingWalls::singleton(crate::world::WallId::of(CfgNodeId(0))),
                ],
                converged: true,
                rounds: 1,
            },
        );
        let clean = MembersAnswer {
            walls: ReachingWalls::bottom(),
            consistency: certified,
        };
        assert!(
            matches!(
                members_freshness(&clean, Freshness::FreshClean),
                Freshness::FreshClean
            ),
            "...and a certified solo passes the policy answer through, or the floor is a blanket \
             refusal and the assertion above proves nothing"
        );
    }

    /// `30L:req-wall-narrative-gains-region-operand` — the wall account can now name a wall that
    /// stands where no plan leaf does.
    ///
    /// The gap this closes (`30Kb:finding-nonleaf-walls-have-no-account-seat`): effective reach
    /// holds handles for command-substitution internals, redirection writes, and spliced function
    /// bodies, and a `LeafId` alone points at whatever GOVERNS one rather than at the line. A
    /// spliced body instance now maps to the authored region a reader can actually find.
    ///
    /// OPERAND ONLY. Whether a wall narrates at all, and under which policy, is untouched — that is
    /// the ratify-or-mint question `30M` §3 leaves to the human, and this rider has to be correct
    /// under either answer.
    #[test]
    fn a_wall_names_the_authored_region_it_stands_in_and_nothing_else() {
        let src = "p() { apt-get install -y nginx; }\np\nufw allow 443/tcp\n";
        let parsed = dorc_syntax::parse(src);
        let built = dorc_analysis::cfg::build(&parsed.value);
        let universe =
            dorc_core::region::RegionUniverse::of_book_custody_files([dorc_core::SourceFileId(0)]);
        let string_execution = crate::region::StringExecutionSites::of_unit(&parsed.value);
        let (loads, vectors) = (BTreeSet::new(), BTreeSet::new());
        let census = crate::region::census(
            &parsed.value,
            &built.value,
            &built.diags,
            crate::region::CensusOpeners::of(&universe, &loads, &vectors, &string_execution),
            dorc_core::SourceFileId(0),
        );
        let instance = census
            .regions()
            .find_map(|(_, population)| match population {
                RoutePopulation::Closed(routes) => routes.routes().next().map(|r| r.cfg_node()),
                RoutePopulation::Open => None,
            })
            .expect("the spliced body holds one region instance");
        assert!(
            region_of_node(&census, instance).is_some(),
            "a wall standing at a spliced body instance names its authored region"
        );
        // An ordinary leaf wall: its participant IS the mutator, so the operand is honestly absent.
        let top_level = built
            .value
            .iter()
            .find(|(id, node)| {
                node.kind == dorc_analysis::cfg::CfgNodeKind::Command
                    && !built.value.is_spliced_internal(*id)
                    && src[parsed.value.node(node.ast).span.lo.0 as usize..].starts_with("ufw")
            })
            .map(|(id, _)| id)
            .expect("the book's own top-level leaf");
        assert!(
            region_of_node(&census, top_level).is_none(),
            "and an ordinary leaf wall gains no operand it does not have"
        );
    }

    /// `30L:pin-shared-edit-before-erasure` / `inv-no-posthoc-shared-demotion` — no per-instance
    /// replacement enters the ledger before the shared agreement exists.
    ///
    /// The property is about WHERE a proof can be minted, which no value-level test reaches: the
    /// lowering seat's `Replace` arm is the only arm that returns proofs at all, and it is reached
    /// only from the universal meet's `SharedConclusion::Replace`. So this scans the seat, the way
    /// `a_provisional_round_names_no_spine_setter` scans its own. What it protects is the grow-only
    /// argument itself: a proof minted per instance would have to be RETRACTED when a later route
    /// forced Run, and the ledger has no retraction — it would simply have retired a wall for a
    /// mutation the artifact still executes.
    #[test]
    fn only_the_universally_agreed_arm_retires_anything() {
        let source = include_str!("settle.rs");
        let seat = source
            .split("fn lower_shared_decision")
            .nth(1)
            .and_then(|tail| tail.split("\n/// What the world said").next())
            .expect("the shared lowering seat");
        // A `|`-shared arm splits in two, so what is asserted is the mint's LOCALITY, not a count.
        let arms: Vec<&str> = seat.split("SharedConclusion::").skip(1).collect();
        let minting: Vec<&&str> = arms
            .iter()
            .filter(|arm| arm.contains("shared_replacement"))
            .collect();
        assert_eq!(
            minting.len(),
            1,
            "exactly one conclusion may reach the proof mint: {arms:?}"
        );
        assert!(
            minting[0].starts_with("Replace"),
            "and it is the universally-agreed Replace: {}",
            minting[0]
        );
    }

    /// A provisional round has no route to Spine (`309:law-spine-write-only-during-run`).
    ///
    /// The real enforcement is the type: `write_spine` lives on `SettledEffectiveAnalysis`, and
    /// the only way there is `seal`, which consumes a `Quiescence` whose field is private and whose
    /// sole mint is the ledger's own no-growth answer. This scan is the second half — a maintainer
    /// can still ADD a method to the provisional impl, and that is what it catches. Doc lines are
    /// stripped: they are prose ABOUT the fence and necessarily name what it forbids.
    #[test]
    fn a_provisional_round_names_no_spine_setter() {
        let source = include_str!("settle.rs");
        let provisional = source
            .split("impl ProvisionalEffectiveRound {")
            .nth(1)
            .expect("the provisional impl block");
        let body: String = provisional
            .split("\n}\n")
            .next()
            .expect("its body")
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in ["Spine", "push_", "set_"] {
            assert!(
                !body.contains(forbidden),
                "a provisional round must not reach a Spine setter (`{forbidden}`); found:\n{body}"
            );
        }
    }
}
