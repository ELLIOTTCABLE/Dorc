//! Shared-region decisions: the route census, the per-route property product, and the universal
//! meet (`plans/30L` §§2–5).
//!
//! # What this answers, and what it deliberately does not
//!
//! The analyzer already builds a per-call CFG instance for every statically resolved function
//! call. What it lacked was a way to say anything about ONE AUTHORED REGION across all of them, so
//! a single live command anywhere in a body forfeited the whole call. This module supplies that
//! missing unit: group per-instance answers by the authored span they would edit, then meet them.
//!
//! It computes decisions and NOTHING consumes them yet. Settlement projection, effective reach,
//! rendering, and Spine are the next lane's; every existing corpus case is byte-identical with this
//! module built.
//!
//! # The meet is universal, and it is biased to Run
//!
//! `30L:rul-shared-region-needs-universal-must`: a region transforms only when every CFG route to
//! every statically possible invocation instance is closed and known AND every license-bearing
//! property holds at `Must` on every one of them. Any unknown, failure, or disagreement is Run.
//! There is no `May` transformation and no per-invocation specialization — an authored region has
//! ONE set of bytes, and there is no second author to answer for a specialized copy
//! (`30L:rul-no-specialized-shell`).
//!
//! Cardinality one is not a case: a single-route population falls out of the same meet as a
//! twenty-route one, and nothing here branches on route count
//! (`30L:pin-no-singleton-special-case`).
//!
//! # Clones for invocations, overlays for iterations
//!
//! `30L` §3.4 asked whether per-invocation body CLONES remain the representation. They do, and the
//! answer is a split rather than a choice: an INVOCATION instance is a clone (its own CFG nodes, so
//! its own reaching-wall in-state at its own program point), while an ITERATION instance is an
//! overlay on ONE lowered body (a loop is a real cycle; every iteration executes the same nodes).
//! [`RouteInstance`] therefore carries both axes — `cfg_node` for the clone, `iteration` for the
//! overlay — and a population may mix them without either axis meaning the other.

use std::collections::BTreeMap;

use dorc_aid::diag::{Diag, DiagCode};
use dorc_analysis::cfg::{Cfg, CfgNodeId, CfgNodeKind, ExecutionOwner};
use dorc_analysis::solve::Graph;
use dorc_core::influence::InfluencePhase;
use dorc_core::region::{ElisionRegion, IterationSlot, RegionUniverse};
use dorc_core::{AstId, DefinitionId, FactKey, SourceFileId, Span};
use dorc_syntax::ast::{Ast, NodeKind, UnsupportedReason};

use crate::StandIn;

/// Which invocation an analyzed instance belongs to.
///
/// The OUTERMOST enclosing call, which is the node `cfg::ExecutionOwner::Leaf` already names as the
/// unit whose decision governs the instance. Taking the innermost call instead would name a
/// different thing at every nesting depth while the render unit stayed put.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvocationId(CfgNodeId);

impl InvocationId {
    /// The CALL node this invocation is.
    #[must_use]
    pub fn node(self) -> CfgNodeId {
        self.0
    }
}

/// One analyzed execution instance of a body region (`30L` §2).
///
/// `cfg_node` is the instance's own spliced node, so two invocations of one definition are two
/// instances by construction. `region` is what they SHARE — the authored span a transformation
/// would edit — and grouping by it is the whole mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteInstance {
    definition: DefinitionId,
    invocation: InvocationId,
    cfg_node: CfgNodeId,
    region: ElisionRegion,
    iteration: IterationSlot,
}

impl RouteInstance {
    /// The definition whose body holds this instance.
    #[must_use]
    pub fn definition(self) -> DefinitionId {
        self.definition
    }

    /// The invocation this instance executes under.
    #[must_use]
    pub fn invocation(self) -> InvocationId {
        self.invocation
    }

    /// This instance's own CFG node — its analysis identity, and where its reaching walls are read.
    #[must_use]
    pub fn cfg_node(self) -> CfgNodeId {
        self.cfg_node
    }

    /// The authored region every instance of this route would edit.
    #[must_use]
    pub fn region(self) -> ElisionRegion {
        self.region
    }

    /// Which evaluation of an authored loop this instance is, where one applies.
    #[must_use]
    pub fn iteration(self) -> IterationSlot {
        self.iteration
    }
}

/// A closed, ordered, NON-EMPTY route population (`30L:inv-closed-route-set-never-empty`).
///
/// Head-plus-tail, the `AllEstablishesVouched` shape, because emptiness is what a universal
/// quantifier must never be handed: "every route admits this" over no routes is vacuously true, and
/// an unreached definition would acquire authority nobody granted it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedRoutes {
    head: RouteInstance,
    tail: Vec<RouteInstance>,
}

impl ClosedRoutes {
    /// Every route, in census order.
    pub fn routes(&self) -> impl Iterator<Item = &RouteInstance> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }

    /// How many routes this population holds. Provenance and narration only — no decision here
    /// keys on it (`30L:pin-no-singleton-special-case`).
    #[must_use]
    pub fn count(&self) -> usize {
        1 + self.tail.len()
    }
}

/// Whether every statically possible invocation instance of a region is enumerated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutePopulation {
    /// Every route is known, and there is at least one.
    Closed(ClosedRoutes),
    /// Something may execute this region that the census could not enumerate. Always runs.
    Open,
}

/// The census: which regions exist in this book, and whether each one's route set is closed.
///
/// `30L:pin-census-is-execution-not-scope` — the population quantifies over what may EXECUTE in the
/// produced program, never over what some mode chose to check. Nothing here reads a probe result,
/// a records fold, or a check selection; a site nobody probed is still a route.
#[derive(Debug, Clone, Default)]
pub struct RegionCensus {
    populations: BTreeMap<ElisionRegion, RoutePopulation>,
}

impl RegionCensus {
    /// Every region the census found, with its population, in region order.
    pub fn regions(&self) -> impl Iterator<Item = (&ElisionRegion, &RoutePopulation)> {
        self.populations.iter()
    }

    /// This region's population, if the census knows it.
    #[must_use]
    pub fn population(&self, region: ElisionRegion) -> Option<&RoutePopulation> {
        self.populations.get(&region)
    }

    /// How many regions the census holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.populations.len()
    }

    /// Did the census find no regions at all? The empty-world answer, and the shape a book with no
    /// eligible calls takes (`30L:pin-empty-function-world-parity`).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.populations.is_empty()
    }
}

/// Build the census over one book's analysis (`30L` §3).
///
/// `book` is the source id the book's definitions belong to; `diags` are the CFG build's own
/// diagnostics, which are where a refused inline is recorded.
///
/// Closedness (`30L:rul-call-census-must-be-closed`) fails toward OPEN on three signals, each of
/// which means some execution of the region is not in the enumerated set:
///
/// * a shell-level DYNAMIC-EXECUTION construct anywhere in the unit — `eval`, a computed `.`, a
///   dynamic command word, a command-position `"$@"`. Whole-unit, because such a construct can name
///   any function and silence never means "no other calls". An unmodeled external COMMAND is not
///   one of these and never opens a census: external commands cannot invoke shell functions.
/// * a refused inline naming the region's function — recursion, a budget, a redefinition, an
///   unmodeled positional, an unthreaded positional. Name-keyed because a refusal knows only a
///   name, so one refusal opens every same-named definition's population: conservative in the
///   only direction that is safe, and never a sharing of populations
///   (`30L:pin-definition-not-name`).
/// * an instance inside a loop body — today, always (`30L:pin-loop-population-open-until-proven`).
///   The propagation lane turns exactly this into `Closed` member populations, and that is the one
///   thing it changes.
#[must_use]
pub fn census(
    ast: &Ast,
    cfg: &Cfg,
    diags: &[Diag],
    universe: &RegionUniverse,
    book: SourceFileId,
) -> RegionCensus {
    let definitions = definition_spans(ast, book);
    let dynamic_execution = unit_has_dynamic_execution(ast);
    let refused = refused_function_names(diags);
    let owners = execution_owners(cfg);

    let reachable = reachable_from_entry(cfg);
    let mut instances: BTreeMap<ElisionRegion, Vec<RouteInstance>> = BTreeMap::new();
    let mut opened: BTreeMap<ElisionRegion, bool> = BTreeMap::new();
    for (node, cfg_node) in cfg.iter() {
        // A detached funcdef-body lowering is not an EXECUTION: nothing reaches it, and reading its
        // vacuous-⊥ in-state as ambient is a wrong-elision (`analysis/CLAUDE.md
        // vacuous-entry-fold`). Only the spliced copies a call actually reaches are routes.
        if cfg_node.kind != CfgNodeKind::Command
            || !cfg.is_spliced_internal(node)
            || !reachable[node.index()]
            || cfg.call_body_sites(node).is_some()
        {
            continue;
        }
        let Some((definition, name)) = enclosing_definition(&definitions, ast, cfg_node.ast) else {
            continue;
        };
        let Some(region) = ElisionRegion::mint(universe, definition, ast.node(cfg_node.ast).span)
        else {
            continue;
        };
        let Some(invocation) = owners.get(&node).copied() else {
            continue;
        };
        let opens = dynamic_execution
            || refused.iter().any(|refused| refused == name)
            || cfg.in_loop_body(node);
        *opened.entry(region).or_insert(false) |= opens;
        instances.entry(region).or_default().push(RouteInstance {
            definition,
            invocation: InvocationId(invocation),
            cfg_node: node,
            region,
            iteration: IterationSlot::NotIterated,
        });
    }

    let populations = instances
        .into_iter()
        .map(|(region, mut routes)| {
            let open = opened.get(&region).copied().unwrap_or(true);
            routes.sort_by_key(|route| route.cfg_node);
            let population = match (open, routes.split_first()) {
                (false, Some((head, tail))) => RoutePopulation::Closed(ClosedRoutes {
                    head: *head,
                    tail: tail.to_vec(),
                }),
                _ => RoutePopulation::Open,
            };
            (region, population)
        })
        .collect();
    RegionCensus { populations }
}

/// The reproducing stand-in a route admits, as the shared decision compares it.
///
/// Deliberately NOT keyed on the fact: two invocations of `apt-get install "$1"` establish
/// different cells, each licensed by its own reached vouch, and the EDIT they admit is the same
/// bytes. What must agree is the observable tuple the replacement reproduces
/// (`30L:rul-shared-edit-reproduces-every-route`), which is exactly the stand-in. The edit SPAN
/// needs no field: routes are grouped by region, and the region is the span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedStandIn(StandIn);

impl SharedStandIn {
    /// The shared stand-in reproducing `stand_in`.
    #[must_use]
    pub fn of(stand_in: StandIn) -> Self {
        SharedStandIn(stand_in)
    }

    /// The value-preserving substitution every contributing route reproduces.
    #[must_use]
    pub fn stand_in(self) -> StandIn {
        self.0
    }
}

/// The parametric guard a route admits, as the shared decision compares it.
///
/// `canonical` is the guard's DECISION-relevant bytes — emitted function name, invocation, and
/// preamble. Comparing those is how `30L:pin-guard-resolution-is-frame-live` is enforced without
/// promoting a display value into a decision: two instances resolving DIFFERENT live verdict
/// definitions ship different preamble bytes, so they compare unequal and the shared guard
/// refuses; two instances resolving byte-identical bodies are the same definition under the
/// artifact's own content-dedup rule, so they compare equal and correctly may share.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedGuard {
    fact: FactKey,
    canonical: String,
}

impl SharedGuard {
    /// The cell the shared guard re-verifies.
    #[must_use]
    pub fn fact(&self) -> FactKey {
        self.fact
    }
}

/// What ONE route independently admits at its region — the per-route property product (`30L` §4).
///
/// A PRODUCT, not a choice, because the meet quantifies over each license-bearing property
/// separately (`30L:rul-every-property-meets-universally`): a route may admit a replacement and no
/// guard, or a guard and no replacement, and the shared answer needs both answers rather than the
/// route's own preferred one. All-`None` is Run.
///
/// Fields are private and the sole production mint is [`RouteAdmission::project`], which reads the
/// engine's own per-site conclusion. That is what keeps this a PROJECTION rather than a second
/// bookkeeping plane populated by hand at each seat (`30L:rul-route-proofs-are-projections`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RouteAdmission {
    replace: Option<SharedStandIn>,
    omit: Option<AstId>,
    guard: Option<SharedGuard>,
}

impl RouteAdmission {
    /// Project one site's decided conclusion into what it admits at its region.
    ///
    /// Every failed property — an absent vouch, a diverged verdict, a stale world, a consumed
    /// channel with no probe-sourced stand-in, a ⊤ successor — has already collapsed the conclusion
    /// to `Run` at the site seat, so an all-`None` admission here IS "some property did not hold",
    /// with no second place for the answer to be wrong.
    #[must_use]
    pub fn project(conclusion: &RouteConclusion) -> Self {
        match conclusion {
            RouteConclusion::Run => RouteAdmission::default(),
            RouteConclusion::Replace(stand_in) => RouteAdmission {
                replace: Some(SharedStandIn(*stand_in)),
                ..RouteAdmission::default()
            },
            RouteConclusion::Omit { controller } => RouteAdmission {
                omit: Some(*controller),
                ..RouteAdmission::default()
            },
            RouteConclusion::Guard { fact, canonical } => RouteAdmission {
                guard: Some(SharedGuard {
                    fact: *fact,
                    canonical: canonical.clone(),
                }),
                ..RouteAdmission::default()
            },
        }
    }
}

/// One site's decided conclusion, in the region plane's vocabulary.
///
/// The site seat's own conclusion type stays private — it carries the irreversible licenses, and
/// `30Kb`'s projects-twice discipline keeps it unexported. This is its region-facing shadow, and
/// its ONE sanctioned producer is a total match over that conclusion at the site seat (the bridge
/// the settlement stage writes). Nothing else may populate it: a seat that built one from an
/// outcome, a render, or a re-derivation would be the second bookkeeping plane
/// `30L:rul-route-proofs-are-projections` forbids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteConclusion {
    /// Some license-bearing property did not hold at this route.
    Run,
    /// The route admits an observable-preserving replacement reproducing this stand-in.
    Replace(StandIn),
    /// The fold proved this route's branch dead, controlled by this leaf.
    Omit { controller: AstId },
    /// The route admits a guard: the cell it re-verifies, and the guard's decision-relevant bytes
    /// (`GuardInsert::canonical` — emitted name, invocation, preamble).
    Guard { fact: FactKey, canonical: String },
}

/// One route's proof: which instance, what it admits, and how far its answer stands from
/// host-produced bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteRegionProof {
    instance: RouteInstance,
    admits: RouteAdmission,
    influence: Option<InfluencePhase>,
}

impl RouteRegionProof {
    /// Assemble a route's proof from its instance and its projected admission.
    #[must_use]
    pub fn new(
        instance: RouteInstance,
        admits: RouteAdmission,
        influence: Option<InfluencePhase>,
    ) -> Self {
        Self {
            instance,
            admits,
            influence,
        }
    }

    /// The instance this proof is about.
    #[must_use]
    pub fn instance(&self) -> RouteInstance {
        self.instance
    }
}

/// The private semantic conclusion a region reaches — the meet's result, before it projects
/// (`30L` §5, `rul-region-edit-is-one-must-result`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SharedConclusion {
    Replace(SharedStandIn),
    Omit { controller: AstId },
    Guard(SharedGuard),
    Run,
}

/// What a region's shared decision establishes about whether its instances can still mutate.
///
/// The settlement-facing half of the conclusion, minted BESIDE the public outcome and never read
/// off it (`plan/CLAUDE.md acts-and-dispositions-mint-together`; `pin-no-outcome-as-generator`).
/// There is deliberately no conversion from [`SharedOutcome`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedRegionAct {
    /// Every contributing instance's original mutation is retired by the shared edit — subject to
    /// the render's own agreement, which is where the next lane lowers this into per-instance
    /// `world::EffectiveAct::NoMutation` proofs through their existing fenced mints.
    RetiresEveryInstance,
    /// Every contributing instance's original mutation may still execute.
    MayMutateEveryInstance,
}

/// The public per-region outcome — what the plan does with one authored region.
///
/// Region-level rather than [`crate::Disposition`] on purpose: a `Disposition::Replace` carries a
/// license, and the license a SHARED replacement must carry is the cross-instance witness spanning
/// every contributing establish (`30L:pin-shared-witness-spans-instances`). Putting one route's
/// per-call license here would be exactly the per-call-witness substitution that pin forbids, and
/// that witness's mint belongs to the settlement stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedOutcome {
    Replace(SharedStandIn),
    Omit { controller: AstId },
    Guard(SharedGuard),
    Run,
}

/// One region's decision, at the influence grade of its most-influenced contributing route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedRegionDecision {
    region: ElisionRegion,
    outcome: SharedOutcome,
    act: SharedRegionAct,
    contributing: Vec<RouteInstance>,
    influence: Option<InfluencePhase>,
}

impl SharedRegionDecision {
    /// The authored region this decision edits.
    #[must_use]
    pub fn region(&self) -> ElisionRegion {
        self.region
    }

    /// What the plan does with the region.
    #[must_use]
    pub fn outcome(&self) -> &SharedOutcome {
        &self.outcome
    }

    /// What the decision establishes for effective reach.
    #[must_use]
    pub fn act(&self) -> SharedRegionAct {
        self.act
    }

    /// Every route that contributed, in census order — the ordered population the next lane's
    /// cross-instance witness is built from.
    #[must_use]
    pub fn contributing(&self) -> &[RouteInstance] {
        &self.contributing
    }

    /// Present when any contributing route's answer was host-influenced.
    #[must_use]
    pub fn influence(&self) -> Option<InfluencePhase> {
        self.influence
    }
}

/// Meet one region's route proofs into one decision (`30L` §5).
///
/// `proofs` must be the region's WHOLE population, in census order; an `Open` population never
/// reaches here — [`decide_region`] is the seat that enforces that.
///
/// The meet tries the strongest shared answer first and falls to Run, and every arm quantifies
/// UNIVERSALLY: a property held on one route and absent on another meets to failure. Equivalence
/// is semantic, not tag-level — two routes both admitting `Replace` still meet to Run unless they
/// reproduce the same observables.
fn meet(proofs: &[RouteRegionProof]) -> SharedConclusion {
    let Some((first, rest)) = proofs.split_first() else {
        // Universal-over-∅ is forbidden (`30L:inv-closed-route-set-never-empty`); reaching here at
        // all means a caller bypassed the population type, so answer with the floor.
        return SharedConclusion::Run;
    };
    if let Some(stand_in) = first.admits.replace
        && rest
            .iter()
            .all(|proof| proof.admits.replace == Some(stand_in))
    {
        return SharedConclusion::Replace(stand_in);
    }
    if let Some(controller) = first.admits.omit
        && rest
            .iter()
            .all(|proof| proof.admits.omit == Some(controller))
    {
        return SharedConclusion::Omit { controller };
    }
    if let Some(guard) = first.admits.guard.clone()
        && rest
            .iter()
            .all(|proof| proof.admits.guard.as_ref() == Some(&guard))
    {
        return SharedConclusion::Guard(guard);
    }
    SharedConclusion::Run
}

impl SharedConclusion {
    /// Project the one conclusion into BOTH halves at once (`30Kb`'s projects-twice discipline).
    ///
    /// The act is derived from the conclusion, never from the outcome: there is no
    /// `From<SharedOutcome> for SharedRegionAct` and there must never be one, or a rendered result
    /// would re-enter the analysis as evidence for itself (`pin-no-outcome-as-generator`).
    fn project(self) -> (SharedOutcome, SharedRegionAct) {
        match self {
            SharedConclusion::Replace(stand_in) => (
                SharedOutcome::Replace(stand_in),
                SharedRegionAct::RetiresEveryInstance,
            ),
            SharedConclusion::Omit { controller } => (
                SharedOutcome::Omit { controller },
                SharedRegionAct::RetiresEveryInstance,
            ),
            // A guard leaves the authored bytes able to execute, exactly as a run does, so both are
            // may-mutate: only a proof that the artifact really neutralises the site retires a wall
            // (`plan/CLAUDE.md only-a-proof-retires-a-wall`).
            SharedConclusion::Guard(guard) => (
                SharedOutcome::Guard(guard),
                SharedRegionAct::MayMutateEveryInstance,
            ),
            SharedConclusion::Run => (SharedOutcome::Run, SharedRegionAct::MayMutateEveryInstance),
        }
    }
}

/// Decide one region from its population and its routes' proofs (`30L` §5, §4.4).
///
/// An `Open` population is Run without consulting a proof: one unenumerated invocation forces Run
/// for every region it may execute (`30L:pin-open-route-runs`). A `Closed` population whose proofs
/// do not correspond exactly — a missing route, an extra one, a different one — is also Run: the
/// meet must quantify over the population the census proved, never over whatever a caller supplied.
///
/// Influence JOINS toward the most influenced (`30L:pin-influence-joins-most`): one uninfluenced
/// route never cleanses a host-influenced sibling, and the grade is carried, never lowered, because
/// `core::influence` has no lowering conversion at all.
#[must_use]
pub fn decide_region(
    region: ElisionRegion,
    population: &RoutePopulation,
    proofs: &[RouteRegionProof],
) -> SharedRegionDecision {
    let influence = proofs.iter().find_map(|proof| proof.influence);
    let contributing: Vec<RouteInstance> = proofs.iter().map(|proof| proof.instance).collect();
    let corresponds = match population {
        RoutePopulation::Open => false,
        RoutePopulation::Closed(routes) => {
            routes.count() == proofs.len()
                && routes
                    .routes()
                    .zip(proofs.iter())
                    .all(|(route, proof)| *route == proof.instance)
        }
    };
    let conclusion = if corresponds {
        meet(proofs)
    } else {
        SharedConclusion::Run
    };
    let (outcome, act) = conclusion.project();
    SharedRegionDecision {
        region,
        outcome,
        act,
        contributing,
        influence,
    }
}

// --- census helpers ---------------------------------------------------------

/// Every funcdef in the unit as `(whole-definition span, name)`, keyed the way `funcenv`'s
/// definition table keys one: the whole `name() { … }` span in its own file.
fn definition_spans(ast: &Ast, book: SourceFileId) -> Vec<(Span, DefinitionId, String)> {
    ast.iter()
        .filter_map(|(id, node)| match &node.kind {
            NodeKind::FuncDef { name, .. } => {
                let span = ast.node(id).span;
                Some((span, DefinitionId::at(book, span), name.clone()))
            }
            _ => None,
        })
        .collect()
}

/// The innermost definition whose span contains `id` — spans nest by construction, so the smallest
/// container is the owner.
fn enclosing_definition<'a>(
    definitions: &'a [(Span, DefinitionId, String)],
    ast: &Ast,
    id: AstId,
) -> Option<(DefinitionId, &'a str)> {
    let inner = ast.node(id).span;
    definitions
        .iter()
        .filter(|(span, _, _)| span.lo.0 <= inner.lo.0 && inner.hi.0 <= span.hi.0)
        .min_by_key(|(span, _, _)| span.hi.0.saturating_sub(span.lo.0))
        .map(|(_, definition, name)| (*definition, name.as_str()))
}

/// Does the unit hold a shell-level dynamic-execution construct? Keyed on the SYNTAX reason, never
/// on an effect classification: an unmodeled external command is `Opaque` and opens no census,
/// because an external command cannot invoke a shell function.
fn unit_has_dynamic_execution(ast: &Ast) -> bool {
    ast.iter().any(|(_, node)| {
        matches!(
            &node.kind,
            NodeKind::Unsupported {
                reason: UnsupportedReason::DynamicExecution,
                ..
            }
        )
    })
}

/// Every function name some call could not be inlined for.
fn refused_function_names(diags: &[Diag]) -> Vec<String> {
    use dorc_aid::diag::CfgInlineRefusedReason as Reason;
    diags
        .iter()
        .filter_map(|diag| match &diag.code {
            DiagCode::CfgInlineRefused(refused) => Some(match &refused.reason {
                Reason::Redefined { name }
                | Reason::RecursiveCall { name }
                | Reason::DepthBudget { name, .. }
                | Reason::UnmodeledPositional { name, .. }
                | Reason::WriteRedirect { name, .. }
                | Reason::PerCallNodeBudget { name, .. }
                | Reason::PerBookNodeBudget { name, .. } => name.clone(),
            }),
            DiagCode::Depth2PositionalUnthreaded(payload) => Some(payload.name.clone()),
            _ => None,
        })
        .collect()
}

/// Nodes control can actually reach — a forward walk from entry.
fn reachable_from_entry(cfg: &Cfg) -> Vec<bool> {
    let mut seen = vec![false; cfg.node_count()];
    let mut stack = vec![cfg.entry()];
    seen[cfg.entry().index()] = true;
    while let Some(node) = stack.pop() {
        for next in cfg.succ_ids(node) {
            if !seen[next.index()] {
                seen[next.index()] = true;
                stack.push(next);
            }
        }
    }
    seen
}

/// Per spliced node, the CALL whose decision governs it.
fn execution_owners(cfg: &Cfg) -> BTreeMap<CfgNodeId, CfgNodeId> {
    cfg.iter()
        .filter_map(|(id, _)| match cfg.execution_owner(id) {
            ExecutionOwner::Leaf(call) if call != id => Some((id, call)),
            _ => None,
        })
        .collect()
}
