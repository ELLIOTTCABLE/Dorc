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
use dorc_analysis::cfg::{Cfg, CfgNodeId};
use dorc_analysis::effect::SkipClass;
use dorc_core::spine::{Grade, SpineSurvival, SurvivalDemote, SurvivalOutcome};
use dorc_core::{AstId, Channel, FactBacking, FactKey, KindId, LeafId, Observable};
use dorc_syntax::ast::Ast;

use crate::erase::{DeadBranchProof, RoundId, prove_dead_branches};
use crate::world::{
    EffectiveAct, Freshness, NoExecutionLedger, NoMutationProof, Quiescence, ReachingWalls,
    ReplacementDeathProof, StaleCause, WallPolicy, effective_invalidators, solve_reaching_walls,
};
use crate::{
    ConnectedPipes, Disposition, Spine, Vouches, decide_site, leaf_facts, leaf_has_heredoc,
    site_order,
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
    Survived,
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
        SurvivalAccount::Survived => SurvivalOutcome::Survived,
        SurvivalAccount::Demoted(StaleCause::RederivationDisagreed { wall }) => {
            SurvivalOutcome::RederivationDisagreed { wall }
        }
        SurvivalAccount::Demoted(cause) => SurvivalOutcome::Demoted(match cause {
            StaleCause::Poisoned { .. } => SurvivalDemote::Poisoned,
            StaleCause::MayAlias => SurvivalDemote::MayAlias,
            // A solve nobody may trust is not a claim about walls: it takes the total floor and
            // narrates as one, since the operand that would name a wall is exactly what failed.
            StaleCause::TotalWall
            | StaleCause::SolveInconsistent
            | StaleCause::RederivationDisagreed { .. } => SurvivalDemote::TotalWall,
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
                    StaleCause::TotalWall | StaleCause::SolveInconsistent => DemoteTag::TotalWall,
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
                effective_solve_failures: failures,
            };
        }
        if number >= cap {
            ledger.rebuild_from_origin();
            let outcome = one_round(inputs, model, &ledger);
            failures = failures.saturating_add(outcome.solve_failures);
            // The maximal-effects answer is a fixpoint of an empty ledger by construction: nothing
            // it proves is recorded, so nothing it proves can license anything downstream.
            let witness = NoExecutionLedger::new()
                .record_round(RoundId(number), [])
                .expect("an empty ledger records nothing");
            return Settlement {
                spine: outcome.round.seal(witness).write_spine(),
                classification: outcome.classification,
                validity: outcome.validity,
                origin_validity: origin_validity.unwrap_or_default(),
                ledger,
                capped: true,
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
    /// How many effective-reach post-fixpoint CHECKS failed across every round (`30K` §4.4).
    ///
    /// A scalar, and accumulated across ALL rounds rather than kept from the settled one, for the
    /// reason the certifier latch is threaded the same way: an intermediate round is never
    /// observed, so a failure there would otherwise be invisible to every reader of the settled
    /// answer. Scalars only — the failing lattice values stay behind in the `SolveConsistency`
    /// (`303:fnd-witness-operands-cannot-enter-narrative`).
    pub effective_solve_failures: u32,
}

/// One round's products, none of which survives a growing ledger.
struct RoundOutcome {
    round: ProvisionalEffectiveRound,
    classification: RoundClassification,
    validity: BTreeMap<LeafId, bool>,
    /// Failing post-fixpoint CHECKS across this round's effective solves.
    solve_failures: u32,
}

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
    let mut members_walls: BTreeMap<CfgNodeId, ReachingWalls> = BTreeMap::new();
    for (_, node, class) in &ordered {
        if !matches!(class, SkipClass::EstablishMembers { .. }) {
            continue;
        }
        let (solo, solo_consistency) = solve_reaching_walls(cfg, &effective, Some(*node));
        model.trip().record(&solo_consistency);
        solve_failures = solve_failures.saturating_add(failing_checks(&solo_consistency));
        let walls = if solo_consistency.is_consistent() {
            solo.states
                .get(node.index())
                .cloned()
                .unwrap_or_else(dorc_analysis::lattice::Lattice::bottom)
        } else {
            // An uncertified answer licenses nothing: hand the site a wall it cannot resolve.
            ReachingWalls::singleton(crate::world::WallId::of(*node))
        };
        members_walls.insert(*node, walls);
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
        let site_walls = match class {
            SkipClass::EstablishMembers { .. } => members_walls
                .get(node)
                .cloned()
                .unwrap_or_else(dorc_analysis::lattice::Lattice::bottom),
            _ => walls_at(*node),
        };
        let freshness = floor_uncertified(
            &consistency,
            inputs.policy.freshness(
                &site_walls,
                survival_subject(class),
                &classification.fact_backings,
                &leaf_of,
            ),
        );
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
            invalidator: classification.invalidators.contains(node),
            accounts_survival,
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

/// The cell a site's elision would be spared ON — the coordinate the survival tier compares each
/// crossed footprint against.
///
/// An aggregate answers with its REPRESENTATIVE member, which is the cell its own license carries
/// (`AllEstablishesVouched::representative`); a shape with no cell at all answers `None`, which the
/// policy reads as "everything collides" rather than as "nothing does".
fn survival_subject(class: &SkipClass) -> Option<FactKey> {
    match class {
        SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
            Some(*fact)
        }
        SkipClass::EstablishMembers { members, .. } => members.first().copied(),
        SkipClass::InlineCall { sites } => sites.iter().find_map(|site| match site.class {
            SkipClass::EstablishProbeAmbient(fact) => Some(fact),
            _ => None,
        }),
        SkipClass::QueryResolvable { .. } | SkipClass::MustRun => None,
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
    fact: FactKey,
) -> Option<ReplacementDeathProof> {
    ReplacementDeathProof::mint(node, fact, replacement_renders_dead(ast, ast_id))
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

    /// A provisional round has no route to Spine. This is a COMPILE-tier property, so the pin is a
    /// doctest-shaped one: the method does not exist, and a maintainer adding one has to delete
    /// this.
    #[test]
    fn a_provisional_round_names_no_spine_setter() {
        let source = include_str!("settle.rs");
        let provisional = source
            .split("impl ProvisionalEffectiveRound {")
            .nth(1)
            .expect("the provisional impl block");
        let body = provisional.split("\n}\n").next().expect("its body");
        assert!(
            !body.contains("Spine") && !body.contains("push_") && !body.contains("set_"),
            "a provisional round must not reach a Spine setter; found:\n{body}"
        );
    }
}
