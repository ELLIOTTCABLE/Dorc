//! Effective world reach — the ONE apply-time answer to "what may still have moved" (`30K`).
//!
//! Three mechanisms used to answer one staleness question and disagreed about it: origin
//! `Reach`'s per-cell `EstablishProbeAmbient`/`EstablishProbeWritten` split, `Reach::is_pristine`'s Query
//! validity bit, and a post-disposition wall walk that could only demote `Replace` to `Run`. The
//! split is what cost the guard tier below a MODELED running mutator: an honest declining verdict
//! function stopped the site going `Opaque`, so downstream establishes classified ambient, minted
//! a `Replace`, and the walk had no rung between elide and run to put them on
//! (`trial/r26/predictions.md` §7). Classing an honest decline therefore produced a strictly worse
//! plan than shipping no oracle at all.
//!
//! Here there is one fact: the set of mutation-capable acts that MAY ACTUALLY EXECUTE before a
//! CFG position. Freshness, Query validity, total walls, and footprint-scoped survival all read
//! it, so they cannot disagree.
//!
//! # What is in the lattice, and what is deliberately not
//!
//! [`ReachingWalls`] is a `Powerset` of wall HANDLES — node identities, nothing more. That is not
//! parsimony: the solver's `Eq` is its termination test, so anything in the domain becomes part of
//! the fixpoint, and footprints, resolvers, dialects, and narration have no business deciding when
//! a worklist stops. The wall POLICY resolves a handle to total-wall or to trusted-footprint data
//! at the CONSUMER, outside the lattice, where the flag-gated authority already lives. Reusing
//! origin `Reach` was never open either: a `FactKey` set cannot represent a total running wall,
//! ordered wall attribution, aliases, backing sets, or footprint dialect comparison.
//!
//! The element type is what keeps the two species apart. `WallId` is minted here and nowhere else,
//! so no origin answer can be handed to an effective consumer or the reverse — no `ReachLike`
//! trait, no shared helper, no conversion.
//!
//! # Why the acts are private, and never dispositions
//!
//! The reach transfer consumes an [`EffectiveAct`], never a [`Disposition`](crate::Disposition).
//! `pin-no-outcome-as-generator` forbids a rendered outcome re-entering analysis as evidence;
//! `rul-rc-reaches-genkill-only-through-decisions` requires the typed decision to come FIRST. Both
//! hold because the decision constructor mints the act and the disposition from ONE proof
//! (`crate::settle`), so the act is a premise the decision established rather than a reading of
//! what the decision produced. There is deliberately no `From<Disposition> for EffectiveAct`.

use std::collections::{BTreeMap, BTreeSet};

use dorc_analysis::certify::{SolveConsistency, solve_certified};
use dorc_analysis::cfg::{Cfg, CfgNodeId, ExecutionOwner};
use dorc_analysis::lattice::Powerset;
use dorc_analysis::solve::{Direction, Solution};
use dorc_core::{Dialect, FactBacking, FactKey, LeafId};

use crate::erase::{DeadBranchProof, RoundId};
use crate::rederive;
use crate::survival::{
    AccumulatedWall, Backing, DemoteReason, Resolutions, SurvivalWitness, TrustedFootprints,
    WallVerdict, wall_verdict,
};

/// One mutation-capable act that may execute, identified by the CFG node that performs it.
///
/// A handle, not evidence: everything a consumer needs beyond identity (a footprint, a leaf id for
/// attribution) is looked up OUTSIDE the lattice. Minted only by [`EffectiveAct::may_mutate`], so
/// the set can never contain a node nobody decided about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WallId(CfgNodeId);

impl WallId {
    /// The CFG node this wall is, for footprint lookup and attribution.
    #[must_use]
    pub fn node(self) -> CfgNodeId {
        self.0
    }

    /// A wall standing at `node` — the floor a consumer hands itself when an answer it cannot
    /// trust would otherwise read as an empty set.
    #[must_use]
    pub fn of(node: CfgNodeId) -> Self {
        WallId(node)
    }
}

/// The mutations that may execute before a CFG position — union-joined, `⊥` = none.
pub type ReachingWalls = Powerset<WallId>;

/// What one site's decision established about whether its ORIGINAL mutation can still run.
///
/// The distinction between a final `Run` and a final `Guard` is deliberately absent: both leave
/// the authored bytes able to execute, so both are `MayMutate`. Only a proof that the artifact
/// really neutralises the site — or that its branch is dead — buys `NoMutation`.
#[derive(Debug, Clone)]
pub enum EffectiveAct {
    /// This site cannot mutate at apply, on the named proof.
    NoMutation(NoMutationProof),
    /// This site's original mutation may execute.
    MayMutate(WallId),
}

impl EffectiveAct {
    /// The act of a site whose original mutation survives into the artifact.
    #[must_use]
    pub fn may_mutate(node: CfgNodeId) -> Self {
        EffectiveAct::MayMutate(WallId(node))
    }
}

/// Why a site cannot mutate. `NotEffective` is the statically-quiet case (a pure builtin, a Query,
/// a site with no modeled effect); the other two carry real proofs and are what the ledger keeps.
#[derive(Debug, Clone)]
pub enum NoMutationProof {
    /// Nothing at this site gens into the effective world in the first place.
    NotEffective,
    /// The fold proved this site's branch unreachable AND its controller substitutes away.
    DeadBranch(DeadBranchProof),
    /// A licensed replacement whose emitted artifact really neutralises the original bytes.
    Replaced(ReplacementDeathProof),
}

/// A licensed replacement PLUS the render's own agreement that the original bytes disappear
/// (`30K` §2.4 — the newly-visible wrong-yes boundary).
///
/// `Disposition::Replace` alone is not a proof: the span render REFUSES a heredoc-carrying leaf
/// and keeps it verbatim, so a site that "was replaced" can still run. Erasing its wall off the
/// disposition would license every downstream elision against a mutation the artifact still
/// executes. Fields are private and the sole mint is [`ReplacementDeathProof::mint`], whose one
/// caller is the provisional decision constructor — `replacement_death_mint_has_exactly_one_caller`
/// is the lexical fence, the shape `erase::licence_mint_has_exactly_one_caller` already uses,
/// because `analysis` cannot depend on `plan` and no type can carry this.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplacementDeathProof {
    site: CfgNodeId,
    fact: FactKey,
}

impl ReplacementDeathProof {
    /// Mint the proof. `licensed` is the site's real [`crate::ReplaceLicense`]-bearing decision and
    /// `renders_dead` is the render's own refusal predicate, both supplied by the ONE caller.
    pub(crate) fn mint(site: CfgNodeId, fact: FactKey, renders_dead: bool) -> Option<Self> {
        renders_dead.then_some(ReplacementDeathProof { site, fact })
    }

    /// The site whose mutation this proof retires.
    #[must_use]
    pub fn site(&self) -> CfgNodeId {
        self.site
    }

    /// The cell the retired mutation would have written.
    #[must_use]
    pub fn fact(&self) -> FactKey {
        self.fact
    }
}

/// One ledger entry: the proof, and the settlement round that first proved it.
#[derive(Debug, Clone)]
pub struct NoExecutionEntry {
    proof: NoMutationProof,
    round: RoundId,
}

impl NoExecutionEntry {
    /// The derivation behind this entry.
    #[must_use]
    pub fn proof(&self) -> &NoMutationProof {
        &self.proof
    }

    /// Which settlement round FIRST proved it (the why-chain's round tag).
    #[must_use]
    pub fn round(&self) -> RoundId {
        self.round
    }
}

/// The settlement's ONE cross-round authority: sites proven unable to execute, grow-only within a
/// fixed record-world (`30K` §3.5, generalising `26H` §4¾'s erasure ledger).
///
/// Two proof species, two consumers, and the split is load-bearing rather than tidy. A
/// `DeadBranch` reaches the ANALYSIS effect seam through [`classify_overlay`](Self::classify_overlay),
/// where erasure is spelled `CommandEffect::Pure`. A `Replaced` must NOT: that spelling also
/// destroys the site's own `SkipClass`, so a replaced site would reclassify `MustRun` on the next
/// round, lose the license that replaced it, and settle as `Run` while every downstream decision
/// had already been taken as though it would not execute — a wrong elision, not a precision loss.
/// Both species suppress the site's wall GEN, which is the one rule that must be uniform, and
/// [`proves_no_execution`](Self::proves_no_execution) is its single seat.
#[derive(Debug, Clone, Default)]
pub struct NoExecutionLedger {
    entries: BTreeMap<CfgNodeId, NoExecutionEntry>,
}

/// Proof that a settlement round added nothing — the ONLY way to seal a provisional round
/// (`30K` §3.6). Private field, and the sole mint is [`NoExecutionLedger::record_round`], so
/// "quiescent" cannot be asserted from a stale length or a hopeful boolean.
#[derive(Debug, Clone, Copy)]
pub struct Quiescence(());

impl NoExecutionLedger {
    /// An empty ledger — the origin world, nothing yet proven un-runnable.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record this round's proofs, answering [`Quiescence`] iff the ledger did not grow.
    ///
    /// A site already proven is not re-recorded: the round tag names the round that FIRST proved
    /// it, which is what a cascade renders. `NotEffective` is not ledger material — it is the
    /// absence of a mutation, not a proof that one was removed, and recording it would make the
    /// monotone-growth argument vacuous.
    pub fn record_round(
        &mut self,
        round: RoundId,
        proofs: impl IntoIterator<Item = (CfgNodeId, NoMutationProof)>,
    ) -> Option<Quiescence> {
        let before = self.entries.len();
        for (site, proof) in proofs {
            if matches!(proof, NoMutationProof::NotEffective) {
                continue;
            }
            self.entries
                .entry(site)
                .or_insert(NoExecutionEntry { proof, round });
        }
        (self.entries.len() == before).then_some(Quiescence(()))
    }

    /// Discard everything and return to the origin world (`brg-ledger-resets-on-record-world-change`).
    /// The record-world changed; nothing proven under the old one survives.
    pub fn rebuild_from_origin(&mut self) {
        self.entries.clear();
    }

    /// How many sites are proven un-runnable.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Is nothing proven?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entries, in site order (`inv-determinism`).
    pub fn entries(&self) -> impl Iterator<Item = (CfgNodeId, &NoExecutionEntry)> + '_ {
        self.entries.iter().map(|(site, entry)| (*site, entry))
    }

    /// Is this site proven unable to execute? The ONE seat the wall-gen suppression reads.
    #[must_use]
    pub fn proves_no_execution(&self, site: CfgNodeId) -> bool {
        self.entries.contains_key(&site)
    }

    /// The dead-branch half, as the analysis-side overlay (`erasure-is-applied-once-never-consulted`).
    #[must_use]
    pub fn classify_overlay(&self) -> dorc_analysis::erase::ErasedSites {
        dorc_analysis::erase::ErasedSites::from_licenses(self.entries.iter().filter_map(
            |(site, entry)| {
                matches!(entry.proof, NoMutationProof::DeadBranch(_))
                    .then(|| dorc_analysis::erase::ErasureLicense::for_site(*site))
            },
        ))
    }
}

/// The mutation-capable acts that may still execute, given what the ledger has proven.
///
/// An invalidator disappears exactly when the render unit that GOVERNS it disappears — which is
/// why ownership is a recorded CFG fact and not an adjacency guess (`cfg::ExecutionOwner`). A
/// `$( … )` body command, a write-shaped redirection, and a spliced function body all mutate
/// without owning a span anyone can decide about; each is retired by its owner's decision, and an
/// unclaimed one keeps its wall.
#[must_use]
pub fn effective_invalidators(
    cfg: &Cfg,
    invalidators: &BTreeSet<CfgNodeId>,
    ledger: &NoExecutionLedger,
) -> BTreeSet<CfgNodeId> {
    invalidators
        .iter()
        .copied()
        .filter(|&node| match cfg.execution_owner(node) {
            ExecutionOwner::AlwaysAtNode => true,
            ExecutionOwner::Leaf(owner) => !ledger.proves_no_execution(owner),
        })
        .collect()
}

/// Solve the effective reach: which mutations may execute before each CFG position.
///
/// Forward, `out = in ∪ gen(node)`, gen non-empty exactly at an effective invalidator — monotone
/// and finite-height by construction, and certified like every production answer
/// (`solve-is-certified-only`). `suppress` is the self-reach shape (`effect::self_reach_holds`):
/// an in-loop aggregate's own writes come back to it over the back-edge, and its own elision is
/// what removes them, so its freshness is answered with itself silenced — the same fixed-point
/// argument the Members license has always rested on.
#[must_use]
pub fn solve_reaching_walls(
    cfg: &Cfg,
    effective: &BTreeSet<CfgNodeId>,
    suppress: Option<CfgNodeId>,
) -> (Solution<ReachingWalls>, SolveConsistency<ReachingWalls>) {
    solve_certified(cfg, Direction::Forward, |i, incoming: &ReachingWalls| {
        let node = CfgNodeId(u32::try_from(i).unwrap_or(u32::MAX));
        if suppress == Some(node) || !effective.contains(&node) {
            return incoming.clone();
        }
        let mut out = incoming.clone();
        out.insert(WallId(node));
        out
    })
}

/// How the run treats a mutation that will really execute (`30K` §0
/// `constraint-wall-policy-is-typed`).
///
/// Two inhabited types, never a boolean beside optional data: the risk-accepted mode is
/// constructible ONLY with every authority its decision needs, so no consumer can reach a
/// footprint the admin did not consent to and no maintainer can consult one unflagged. The cli
/// builds `RiskAccepted` on the `--risk-faultless-skips` path alone, where the footprints were
/// lifted at all.
///
/// A fact's BACKING rides beside the policy rather than inside it: backings are derived from the
/// residual model and change as the settlement erases, while the three fields here are the run's
/// frozen authorities. Keeping a per-round value out of the authority type is what stops the
/// policy pretending to be frozen when it is not.
#[derive(Debug, Clone, Copy)]
pub enum WallPolicy<'a> {
    /// Every effective mutator walls TOTAL — the default, and the only honest answer without the
    /// admin's typed consent (`silence-licenses-nothing`).
    Honest,
    /// `--risk-faultless-skips`: a footprinted mutator scopes its wall, and a downstream elision
    /// may survive past it on the author's at-most claim.
    RiskAccepted {
        /// Per-wall-site at-most claims; an absent site walls total.
        footprints: &'a TrustedFootprints,
        /// The kind-owners' identity canonicalization.
        resolutions: &'a Resolutions,
        /// The selector dialect the sparing algebra compares within.
        dialect: &'a Dialect,
    },
}

/// Whether a probed fact is still good where its site sits.
#[derive(Debug, Clone)]
pub enum Freshness {
    /// No mutation reaches this position — the ordinary elision.
    FreshClean,
    /// Mutations reach it, every one provably disjoint from the fact's backing, and the reference
    /// model confirmed it: the design's one naked-trust cell, fully attributed.
    FreshSurvived(SurvivalWitness),
    /// The measurement may have been overtaken. A guard can still re-check it live; an elision
    /// cannot stand on it.
    Stale(StaleCause),
}

/// Why a fact is stale. Carried so the demotion narrates the operand it actually failed on
/// rather than a generic wall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaleCause {
    /// A reaching mutator has no trustworthy footprint (or the honest policy is in force).
    TotalWall,
    /// A footprint coordinate canonicalized onto the fact's own backing.
    Poisoned {
        /// The reach-function KIND, where a `<kind>.reaches()` expansion was the hitting coordinate.
        via_reach: Option<dorc_core::KindId>,
    },
    /// A same-kind pair could not be canonicalized (the resolver gap; fail toward run).
    MayAlias,
    /// The independent reference model declined to confirm a survival the production relation
    /// minted — a finding about OUR engine, never about the book.
    RederivationDisagreed {
        /// The crossed-wall ordinal the models disagreed on.
        wall: u32,
    },
    /// The effective reach answer failed its own post-fixpoint check, so no freshness rests on it
    /// (`302:rul-whole-window-demotion`).
    SolveInconsistent,
}

impl WallPolicy<'_> {
    /// Decide one establish fact's freshness against the walls that reach it (`30K` §5.1).
    ///
    /// The re-derivation runs HERE, before any caller can turn the answer into a no-execution
    /// proof: the reference model consumes the minted witness by value and can only hand it back
    /// or refuse it, so this seat removes survivals and never adds one, and a refusal leaves the
    /// site running — which then keeps its own wall standing for everything below it.
    pub(crate) fn freshness(
        &self,
        walls: &ReachingWalls,
        fact: Option<FactKey>,
        backings: &BTreeMap<FactKey, FactBacking>,
        leaf_of: &BTreeMap<CfgNodeId, LeafId>,
    ) -> Freshness {
        if walls.is_empty() {
            return Freshness::FreshClean;
        }
        let WallPolicy::RiskAccepted {
            footprints,
            resolutions,
            dialect,
        } = self
        else {
            return Freshness::Stale(StaleCause::TotalWall);
        };
        // No cell to compare means no backing, and a backing-set is non-empty by construction
        // (`inv-backing-set-nonempty-by-construction`): the honest reading is that everything
        // collides, never that nothing does.
        let Some(fact) = fact else {
            return Freshness::Stale(StaleCause::TotalWall);
        };
        let mut accumulated: Vec<AccumulatedWall> = Vec::with_capacity(walls.len());
        for wall in walls.iter() {
            let (Some(footprint), Some(leaf)) = (
                footprints.get(wall.node()),
                leaf_of.get(&wall.node()).copied(),
            ) else {
                // No lifted claim, or a mutator with no plan leaf to attribute to (a redirection
                // write, a `$()` body, an unmodeled construct): silence walls, total.
                return Freshness::Stale(StaleCause::TotalWall);
            };
            accumulated.push(AccumulatedWall {
                wall_leaf: leaf,
                footprint: footprint.clone(),
            });
        }
        // Execution order, so the attribution chain reads the way the book does. The set arrives
        // in node order, which is allocation order and not always source order.
        accumulated.sort_by_key(|wall| wall.wall_leaf.0);
        let backing = match backings.get(&fact) {
            Some(fb) => Backing::widened(fact, fb.family, fb.observed.clone()),
            None => Backing::of_fact(fact),
        };
        match wall_verdict(false, &accumulated, &backing, resolutions, dialect) {
            WallVerdict::SurvivedClean => Freshness::FreshClean,
            WallVerdict::Survived(witness) => {
                match rederive::recheck_survival(
                    witness,
                    &backing,
                    &accumulated,
                    resolutions,
                    dialect,
                ) {
                    rederive::Recheck::Confirmed(witness) => Freshness::FreshSurvived(witness),
                    rederive::Recheck::Demoted(disagreement) => {
                        Freshness::Stale(StaleCause::RederivationDisagreed {
                            wall: u32::try_from(disagreement.wall).unwrap_or(u32::MAX),
                        })
                    }
                }
            }
            WallVerdict::Demoted(DemoteReason::TotalWall) => {
                Freshness::Stale(StaleCause::TotalWall)
            }
            WallVerdict::Demoted(DemoteReason::Poisoned { via_reach }) => {
                Freshness::Stale(StaleCause::Poisoned { via_reach })
            }
            WallVerdict::Demoted(DemoteReason::MayAlias) => Freshness::Stale(StaleCause::MayAlias),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Any real cell — these tests are about the ledger's bookkeeping, not about the coordinate.
    fn fixture_fact() -> FactKey {
        dorc_core::auto_fact(&mut dorc_core::Interner::default(), "wombat")
    }

    /// A second minter of the replacement-death proof is not a refactor; it is an unproven route
    /// to erasing a mutation's wall. The seal cannot be a type — `analysis` cannot depend on
    /// `plan` — so it is lexical, exactly as `erase::licence_mint_has_exactly_one_caller` is.
    #[test]
    fn replacement_death_mint_has_exactly_one_caller() {
        let crates = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crates dir");
        let mut callers: Vec<String> = Vec::new();
        let mut stack = vec![crates.to_path_buf()];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir)
                .expect("readable crates dir")
                .flatten()
            {
                let path = entry.path();
                if path.is_dir() {
                    if path.file_name().is_some_and(|n| n == "target") {
                        continue;
                    }
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let src = std::fs::read_to_string(&path).unwrap_or_default();
                    // Split so this scan does not find ITSELF (the `erase` fence's own lesson).
                    if src.contains(concat!("ReplacementDeathProof", "::mint(")) {
                        callers.push(path.display().to_string().replace('\\', "/"));
                    }
                }
            }
        }
        callers.sort();
        assert_eq!(
            callers.len(),
            1,
            "exactly one caller of the replacement-death mint; found {callers:?}"
        );
        assert!(
            callers[0].ends_with("plan/src/settle.rs"),
            "the sole caller is the provisional decision constructor; found {callers:?}"
        );
    }

    #[test]
    fn a_ledger_keeps_the_first_round_and_never_records_a_non_proof() {
        let mut ledger = NoExecutionLedger::new();
        let proof = ReplacementDeathProof {
            site: CfgNodeId(3),
            fact: fixture_fact(),
        };
        assert!(
            ledger
                .record_round(
                    RoundId(1),
                    [(CfgNodeId(3), NoMutationProof::Replaced(proof))]
                )
                .is_none(),
            "a growing round is not quiescent"
        );
        assert!(
            ledger
                .record_round(
                    RoundId(2),
                    [
                        (CfgNodeId(3), NoMutationProof::Replaced(proof)),
                        (CfgNodeId(9), NoMutationProof::NotEffective),
                    ]
                )
                .is_some(),
            "re-proving a known site, and a non-proof, add nothing"
        );
        assert_eq!(ledger.len(), 1);
        assert_eq!(
            ledger.entries().next().map(|(_, e)| e.round()),
            Some(RoundId(1)),
            "the round tag names the round that FIRST proved it"
        );
        assert!(!ledger.proves_no_execution(CfgNodeId(9)));
    }

    /// `brg-ledger-resets-on-record-world-change`: monotonicity is conditional on ONE fixed
    /// record-world, so a change to it discards everything proven under the old one. Carrying an
    /// erasure across is the exact composition mistake the cross-round state law forbids.
    #[test]
    fn rebuilding_from_origin_discards_every_proof_and_licenses_no_shrink() {
        let mut ledger = NoExecutionLedger::new();
        let _ = ledger.record_round(
            RoundId(1),
            [(
                CfgNodeId(1),
                NoMutationProof::DeadBranch(DeadBranchProof::fixture(CfgNodeId(1))),
            )],
        );
        assert_eq!(ledger.len(), 1);
        assert!(ledger.classify_overlay().contains(CfgNodeId(1)));
        ledger.rebuild_from_origin();
        assert!(ledger.is_empty(), "a new record-world keeps no proof");
        assert!(
            ledger.classify_overlay().is_empty(),
            "and licenses no shrink"
        );
        assert!(
            !ledger.proves_no_execution(CfgNodeId(1)),
            "and retires no wall"
        );
    }

    #[test]
    fn only_dead_branches_reach_the_classify_overlay() {
        // The wrong-elision this split exists to stop: erasing a REPLACED site's effects would
        // also erase its `SkipClass`, so the next round could not re-mint the license that
        // replaced it and the site would settle as `Run` with its wall already gone.
        let mut ledger = NoExecutionLedger::new();
        let _ = ledger.record_round(
            RoundId(1),
            [(
                CfgNodeId(4),
                NoMutationProof::Replaced(ReplacementDeathProof {
                    site: CfgNodeId(4),
                    fact: fixture_fact(),
                }),
            )],
        );
        assert!(
            ledger.classify_overlay().is_empty(),
            "a replacement death never shrinks the analyzer's effect model"
        );
        assert!(
            ledger.proves_no_execution(CfgNodeId(4)),
            "but it does retire the site's wall"
        );
    }
}
