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

use crate::erase::{DeadBranchProof, RoundId, prove_dead_branches};
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

/// One settled round's decisions, sealed and therefore allowed to write Spine.
///
/// Private constructor: the only route here is [`ProvisionalEffectiveRound::seal`], which consumes
/// a [`Quiescence`] the ledger mints and nothing else can forge.
#[derive(Debug)]
pub struct SettledEffectiveAnalysis {
    decisions: Vec<ProvisionalSiteDecision>,
    walls: Vec<LeafId>,
    minted_at: Grade,
}

/// One round's decisions, before quiescence is known. Deliberately without a Spine, Plan, render,
/// digest, or iterator API — a convenience there would recreate the projection path the law forbids.
#[derive(Debug)]
pub struct ProvisionalEffectiveRound {
    decisions: Vec<ProvisionalSiteDecision>,
    walls: Vec<LeafId>,
    minted_at: Grade,
}

/// The cap path intentionally discards every no-execution proof and seals the maximal-effects
/// floor, rather than claiming ordinary ledger quiescence.
struct MaximalEffectsFloor;

impl ProvisionalEffectiveRound {
    /// The proofs this round established, for the ledger.
    fn no_execution_proofs(&self) -> Vec<(CfgNodeId, NoMutationProof)> {
        self.decisions
            .iter()
            .filter_map(|decision| match &decision.act {
                EffectiveAct::NoMutation(proof) => Some((decision.node, proof.clone())),
                EffectiveAct::MayMutate(_) => None,
            })
            .collect()
    }

    /// Seal this round. The witness is the whole gate: a round that proved something new cannot
    /// produce one, so it cannot reach any Spine setter.
    #[must_use]
    fn seal(self, _quiescent: Quiescence) -> SettledEffectiveAnalysis {
        SettledEffectiveAnalysis {
            decisions: self.decisions,
            walls: self.walls,
            minted_at: self.minted_at,
        }
    }

    fn seal_floor(self, _floor: MaximalEffectsFloor) -> SettledEffectiveAnalysis {
        SettledEffectiveAnalysis {
            decisions: self.decisions,
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
        for leaf in self.walls {
            spine.push_narrative(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::WallFormation {
                    participant: leaf,
                    channel: ChannelCoverage {
                        channel: Channel::Effect,
                    },
                },
            ));
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
            // A solve nobody may trust takes the same FLOOR as a total wall and wears its own name:
            // the demotion is ours, and calling it a wall points an admin at their own mutators for
            // an engine defect (`302` §5 admin-honesty · `271:rul-sin-ordering`).
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
    let mut failures = 0u32;
    loop {
        let outcome = one_round(inputs, model, &ledger);
        failures = failures.saturating_add(outcome.solve_failures);
        if origin_validity.is_none() {
            origin_validity = Some(outcome.validity.clone());
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
            let outcome = one_round(inputs, model, &ledger);
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

/// One members-site's SELF-SUPPRESSED reach answer, with the certification that decides whether it
/// may be read at all — carried together because reading either alone is the defect
/// (`30Mb:fnd-members-floor-is-a-sentinel`).
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
) -> RoundOutcome {
    let (ast, cfg) = (inputs.ast, inputs.cfg);
    let classification = model.classify(&ledger.classify_overlay());
    let ordered = site_order(ast, cfg, &classification.classes);
    let leaf_of: BTreeMap<CfgNodeId, LeafId> = ordered
        .iter()
        .map(|(leaf, node, _)| (*node, *leaf))
        .collect();

    // The one fact. Everything below reads it and nothing re-derives a second answer.
    let effective = effective_invalidators(cfg, &classification.invalidators, ledger);
    let (reach, consistency) = solve_reaching_walls(cfg, &effective, None);
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
        let (solo, consistency) = solve_reaching_walls(cfg, &effective, Some(*node));
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
        // BOTH certifications floor this site, and the members one is the whole reason the seat is
        // named: the self-suppressed solo solve is a SECOND answer, so a members site rests on a
        // certification the window's own says nothing about (`30Mb:fnd-members-floor-is-a-sentinel`).
        // The retired shape handed an uncertified solo a synthetic unresolvable wall, which reached
        // the same run-or-guard outcome only because the current footprint lift cannot resolve one —
        // an accident, and one that narrated our solver defect as the book's wall.
        let policy_answer = inputs.policy.freshness(
            &site_walls,
            subject,
            &classification.fact_backings,
            &leaf_of,
        );
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
        if accounts_survival && matches!(decision.act, EffectiveAct::MayMutate(_)) {
            walls.push(*leaf);
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
    RoundOutcome {
        round: ProvisionalEffectiveRound {
            decisions,
            walls,
            minted_at: inputs.minted_at,
        },
        classification,
        validity,
        solve_failures,
    }
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
/// A second seat because it is a second certification: the window's answer says nothing about the
/// solo one, so a members site that read only the window's floor would rest on a check nobody ran.
/// The retired shape handed an uncertified solo a synthetic unresolvable wall instead — which
/// reached run-or-guard only because the current footprint lift cannot resolve one, an accident
/// rather than a floor, and one that narrated OUR solver defect as a wall in the admin's book.
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

    /// THE MEMBERS FLOOR, on its own certification. A members site answers from a SECOND,
    /// self-suppressed solve, and the window's check says nothing about it — so an uncertified solo
    /// must floor the site whatever the policy answered over its walls, and the walls it answered
    /// over are the solo's real ones rather than a synthetic sentinel
    /// (`30Mb:fnd-members-floor-is-a-sentinel`). Unreachable from any book today, which is exactly
    /// why the seat has to be pinned rather than trusted to a corpus that cannot exercise it.
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
