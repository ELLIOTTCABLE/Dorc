//! The **solve-certifier** (`plans/302`) — a per-answer, post-fixpoint validator at the solve
//! seam. After every production [`solve`](crate::solve::solve), one flat pass re-checks that the
//! returned states actually ARE a post-fixpoint of the transfer system the solver was given. The
//! solver stays untrusted; its every answer is checked, not believed.
//!
//! # Why this instrument is admissible (`302:rul-certifier-value-is-stupidity`)
//!
//! The find/check asymmetry (`28T` §4 T1): the worklist iteration is where an implementation bug —
//! a dropped re-queue, a mis-scheduled update, a premature convergence claim — yields a silently
//! STALE answer that goldens bless right past, and downstream a stale answer is a wrong elision.
//! The finished answer, by contrast, has a purely local characterization checkable in one sweep.
//! The checker is therefore strictly simpler than what it checks: small, total, panic-free,
//! single-sweep, no early exit. Every pressure toward cleverness resolves by routing it elsewhere
//! (aid → the engine, [`replay_solve`]) or refusing it (recovery — `302` §9, there is none).
//!
//! # The guarantee, with its hypothesis
//!
//! **Given the transfer function is monotone** — already `solve`'s documented caller-upheld
//! precondition, so this leans on nothing new — a [`SolveConsistency::Consistent`] answer is a
//! valid post-fixpoint and therefore over-approximates every abstract path from the boundary.
//! Concrete (γ-soundness) coverage is EXPLICITLY out of scope: the certifier shares the transfer
//! model with the solver and is not foreign ground truth. It catches SOLVER bugs (worklist,
//! ordering, convergence-detection, state-management), never MODEL bugs. Non-monotone transfers
//! are not detected; a `Consistent` answer then claims only the inequalities themselves.
//!
//! The checker's `⊑` is join + structural equality, so its soundness leans on the facade
//! canonicality seats (`dorc_core::sorted`) and the lattice laws — pinned independently. A broken
//! `Eq` or canonical form can blind this instrument; that limit is accepted with eyes open.
//!
//! # `converged` is advisory; the states are what certify (`302` §1)
//!
//! A cap-tripped (`converged: false`) answer that satisfies the checks is legitimately
//! `Consistent` — under monotone ascent from ⊥ it is exactly the least fixpoint (an ascent state
//! is always ⊑ lfp; a post-fixpoint that is ⊑ lfp equals it), so nothing is lost: the solver
//! merely stopped without noticing it had landed. The reverse mismatch — `converged: true` with a
//! failing check — is the defect class this instrument exists to catch.

use dorc_core::sorted::SortedSet;

use crate::lattice::Lattice;
use crate::solve::{Direction, Graph, Solution, SolveObserver, run, solve};

/// How many by-value [`Inconsistency`] items an [`SolveConsistency::Inconsistent`] carries
/// (`302` §2). The complete failing-check INDEX set is always whole; only the value-bearing items
/// are capped, with `shown`/`total` disclosed — the house deterministic-and-disclosed k-cap.
pub const INCONSISTENCY_CAP: usize = 8;

/// How many focused [`ReplayUpdate`]s a [`SolveReplay`] retains (conductor ruling, `302` §5): ONE
/// cap discipline — the first N in walk order within the focused slice, `shown`/`total` disclosed.
/// Per-node accounts ("this node last moved in round 3 via that edge") are computed at RENDER time
/// from what was retained, never by a second retention policy.
pub const REPLAY_UPDATE_CAP: usize = 256;

/// One failed check, carrying the values that failed it (`302` §2).
///
/// **An inconsistency is not a cause.** It is evidence that a named check failed; the actual cause
/// is a code defect no runtime artifact can name. The honest verbs are "failed its post-fixpoint
/// check at" and "first breaks at" — never "caused by".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inconsistency<L> {
    /// `init[node] ⊑ state[node]` failed.
    Boundary { node: usize, init: L, state: L },
    /// `transfer(from, state[from]) ⊑ state[to]` failed.
    Edge {
        from: usize,
        to: usize,
        transferred: L,
        state: L,
    },
}

/// The COMPLETE set of failing check indices — always carried whole (`302` §2): scalars, cheap,
/// canonical, and the substrate every downstream summary reads. Sets, not sequences, so an
/// edge-insertion permutation compares equal (`303:fnd-permutation-pin-is-set-not-sequence`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FailingChecks {
    boundary: SortedSet<usize>,
    edges: SortedSet<(usize, usize)>,
}

impl FailingChecks {
    /// The nodes whose boundary check failed.
    #[must_use]
    pub fn boundary(&self) -> &SortedSet<usize> {
        &self.boundary
    }

    /// The `(from, to)` pairs whose per-edge check failed.
    #[must_use]
    pub fn edges(&self) -> &SortedSet<(usize, usize)> {
        &self.edges
    }

    /// How many checks failed in total.
    #[must_use]
    pub fn len(&self) -> usize {
        self.boundary.len().saturating_add(self.edges.len())
    }

    /// Whether nothing failed — the private-mint invariant is that this is never true inside an
    /// [`SolveConsistency::Inconsistent`].
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.boundary.is_empty() && self.edges.is_empty()
    }

    /// The failing REGION: every node participating in a failed check, either as a failing
    /// boundary or as an endpoint of a failing edge. This is the replay's focus slice.
    #[must_use]
    pub fn nodes(&self) -> SortedSet<usize> {
        let mut out = SortedSet::new();
        for &node in &self.boundary {
            out.insert(node);
        }
        for &(from, to) in &self.edges {
            out.insert(from);
            out.insert(to);
        }
        out
    }
}

/// One recorded state change from an instrumented re-run (`302` §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayUpdate<L> {
    /// The solver round (node-visit count) at which the change landed.
    pub round: usize,
    /// The node whose output was propagated.
    pub from: usize,
    /// The node whose state changed.
    pub to: usize,
    /// `to`'s state before the join.
    pub old: L,
    /// `to`'s state after the join.
    pub new: L,
}

/// The instrumented re-run's per-update log, sliced to the failing region
/// (`302:rul-rerun-is-the-self-report-engine`).
///
/// `inv-determinism` is what makes this sound: a pure kernel replays the IDENTICAL trajectory,
/// defect included, now narrated. It is EVIDENCE OF WHAT HAPPENED, never a trusted computation —
/// the checker remains the judge. Pull-tier (`rul-chain-is-pull-only`): the push surface carries
/// the compact scalar record instead.
///
/// Owned, lifetime-free, and self-disclosing, so a future opt-in durable could take it whole. This
/// lane builds no writer, no serializer, and no switch; and per `300:rul-whylog-is-the-spine` the
/// value-bearing half stays in-memory/pull-tier while only scalars reach a record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolveReplay<L> {
    updates: Vec<ReplayUpdate<L>>,
    total: usize,
    focus: SortedSet<usize>,
}

impl<L> SolveReplay<L> {
    /// The empty replay — what [`certify_solution`] mints (`302` §2: the checker never re-runs
    /// anything; [`solve_certified`] fills the slot on the inconsistent path).
    #[must_use]
    fn empty() -> Self {
        Self {
            updates: Vec::new(),
            total: 0,
            focus: SortedSet::new(),
        }
    }

    /// The retained updates, in walk order.
    #[must_use]
    pub fn updates(&self) -> &[ReplayUpdate<L>] {
        &self.updates
    }

    /// How many updates are retained.
    #[must_use]
    pub fn shown(&self) -> usize {
        self.updates.len()
    }

    /// How many updates the focused slice produced, retained or not.
    #[must_use]
    pub fn total(&self) -> usize {
        self.total
    }

    /// The nodes the slice was focused on.
    #[must_use]
    pub fn focus(&self) -> &SortedSet<usize> {
        &self.focus
    }
}

/// The solver's own ADVISORY report about the run that produced the failing answer (`302` §5:
/// these are narrative operands, never a gate). Carried here so a consumer that has kept only the
/// verdict can still narrate the run, and so nothing has to thread a second value alongside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolverAdvisory {
    /// Whether the solver believed it had settled. Advisory: a cap-tripped answer that satisfies
    /// the checks is the least fixpoint and is used regardless (`302` §1).
    pub converged: bool,
    /// How many node-visits the solver performed.
    pub rounds: usize,
}

/// The evidence behind a passing verdict: counts for the narrative plane, nothing more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsistentChecks {
    checks: usize,
}

impl ConsistentChecks {
    /// How many inequalities were checked.
    #[must_use]
    pub fn checks(&self) -> usize {
        self.checks
    }
}

/// The evidence behind a failing verdict (`302` §2). Fields are private and minted in exactly one
/// place, so an `Inconsistent` with nothing failing is unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedChecks<L> {
    failing: FailingChecks,
    inconsistencies: Vec<Inconsistency<L>>,
    total: usize,
    first_break_edges: SortedSet<(usize, usize)>,
    unstable_components: Vec<SortedSet<usize>>,
    advisory: SolverAdvisory,
    replay: SolveReplay<L>,
}

impl<L> FailedChecks<L> {
    /// The complete failing-check index set.
    #[must_use]
    pub fn failing(&self) -> &FailingChecks {
        &self.failing
    }

    /// The retained by-value items, in canonical order, capped at [`INCONSISTENCY_CAP`].
    #[must_use]
    pub fn inconsistencies(&self) -> &[Inconsistency<L>] {
        &self.inconsistencies
    }

    /// How many by-value items are retained.
    #[must_use]
    pub fn shown(&self) -> usize {
        self.inconsistencies.len()
    }

    /// How many checks failed in total (the uncapped count).
    #[must_use]
    pub fn total(&self) -> usize {
        self.total
    }

    /// The edges where consistency FIRST breaks along the flow: failing checks whose source node
    /// is itself fully clean (its boundary holds and no incoming edge failed). Computed from the
    /// COMPLETE index set before any cap (`302:rul-first-break-and-unstable-components`).
    #[must_use]
    pub fn first_break_edges(&self) -> &SortedSet<(usize, usize)> {
        &self.first_break_edges
    }

    /// The un-stabilized region, named only when no first-break edge exists (every node in a cycle
    /// failing — the runaway shape): NON-TRIVIAL strongly-connected components touching a failing
    /// check. A singleton without a self-loop names no region and is excluded. Canonical order is
    /// by least member.
    #[must_use]
    pub fn unstable_components(&self) -> &[SortedSet<usize>] {
        &self.unstable_components
    }

    /// The instrumented re-run's account of the failing region.
    #[must_use]
    pub fn replay(&self) -> &SolveReplay<L> {
        &self.replay
    }

    /// What the solver itself reported about the run — advisory narrative operands only.
    #[must_use]
    pub fn advisory(&self) -> SolverAdvisory {
        self.advisory
    }

    /// Attach the replay [`solve_certified`] gathered. Private: the checker mints the slot empty
    /// and only the wrapper fills it.
    fn with_replay(mut self, replay: SolveReplay<L>) -> Self {
        self.replay = replay;
        self
    }
}

/// The closed outcome of certifying one solve (`302` §2).
///
/// Never a bool, and deliberately without an `is_ok()`-shaped accessor: a consumer MATCHES, and
/// the `Inconsistent` arm is what forces it to supply its floor. Both payloads have private
/// fields with a single mint, so neither a forged pass nor an empty failure is representable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveConsistency<L> {
    /// Every check passed. The answer is a valid post-fixpoint of the system as given.
    Consistent(ConsistentChecks),
    /// At least one check failed ⇒ WHOLE-WINDOW demotion (`302:rul-whole-window-demotion`): every
    /// consumer of this answer takes its floor, every license fed by it lapses. No per-node trust,
    /// no region carve, no recovery — the summaries exist to EXPLAIN, never to scope.
    Inconsistent(FailedChecks<L>),
}

impl<L> SolveConsistency<L> {
    /// Whether the answer certified — the gate every consumer's floor keys on. Not an
    /// `is_ok()`-shaped escape from matching: the `Inconsistent` payload is unreachable through
    /// it, so a consumer wanting the evidence must still match.
    #[must_use]
    pub fn is_consistent(&self) -> bool {
        matches!(self, SolveConsistency::Consistent(_))
    }
}

/// Certify one solved system against `302` §1's two inequality families.
///
/// 1. **boundary, all nodes**: `∀ n: init[n] ⊑ state[n]`. Checking EVERY node — not only entries —
///    is deliberate: entry nodes have no in-edges, so a pure per-edge walk never sees a seed. This
///    clause does, in both orientations (`Must::bottom()` is the dual's ⊥).
/// 2. **per-edge**: `∀ solver-oriented edges (v → w): transfer(v, state[v]) ⊑ state[w]`.
///
/// `Must<L>` duality costs zero branches: the dual order is carried by the `Must` lattice instance
/// itself, so these two families cover both orientations. Nothing here inspects orientation,
/// phase, or domain semantics — one checker, no orientation parameter.
///
/// The walk is canonical and worklist-free: boundary checks in node order, then edge checks in
/// (node index × successor index) order, transfer evaluated once per node, NO early exit. The
/// solver's release-mode out-of-range guard is mirrored verbatim — mirror the solver, never
/// validate the graph (`303:fnd-mirror-the-out-of-range-skip`).
///
/// `init` is `&[L]`; `solve` seeds nothing today, so [`solve_certified`] passes all-⊥ and this
/// clause executes as the trivially-true-still-executed case, live the day real seeding lands. An
/// entry `init` does not cover reads as ⊥ (trivially true), never as a skipped check.
#[must_use]
pub fn certify_solution<G: Graph, L: Lattice>(
    graph: &G,
    direction: Direction,
    init: &[L],
    transfer: impl Fn(usize, &L) -> L,
    solution: &Solution<L>,
) -> SolveConsistency<L> {
    let node_count = graph.node_count();
    let bottom = L::bottom();
    let mut checks = 0usize;
    let mut failing = FailingChecks::default();
    let mut items: Vec<Inconsistency<L>> = Vec::new();

    for node in 0..node_count {
        let Some(state) = solution.states.get(node) else {
            continue;
        };
        let seed = init.get(node).unwrap_or(&bottom);
        checks = checks.saturating_add(1);
        if !seed.leq(state) {
            failing.boundary.insert(node);
            if items.len() < INCONSISTENCY_CAP {
                items.push(Inconsistency::Boundary {
                    node,
                    init: seed.clone(),
                    state: state.clone(),
                });
            }
        }
    }

    for from in 0..node_count {
        let Some(state_from) = solution.states.get(from) else {
            continue;
        };
        let transferred = transfer(from, state_from);
        for &to in flows_to(graph, direction, from) {
            debug_assert!(
                to < node_count,
                "Graph edge endpoint {to} out of range (node_count {node_count})"
            );
            let Some(state_to) = solution.states.get(to) else {
                continue; // release-mode defensive skip, mirroring `solve` (inv-no-throw)
            };
            checks = checks.saturating_add(1);
            if !transferred.leq(state_to) {
                failing.edges.insert((from, to));
                if items.len() < INCONSISTENCY_CAP {
                    items.push(Inconsistency::Edge {
                        from,
                        to,
                        transferred: transferred.clone(),
                        state: state_to.clone(),
                    });
                }
            }
        }
    }

    if failing.is_empty() {
        return SolveConsistency::Consistent(ConsistentChecks { checks });
    }
    let total = failing.len();
    let first_break_edges = first_break_edges(&failing);
    let unstable_components = if first_break_edges.is_empty() {
        unstable_components(graph, direction, &failing)
    } else {
        Vec::new()
    };
    SolveConsistency::Inconsistent(FailedChecks {
        failing,
        inconsistencies: items,
        total,
        first_break_edges,
        unstable_components,
        advisory: SolverAdvisory {
            converged: solution.converged,
            rounds: solution.rounds,
        },
        replay: SolveReplay::empty(),
    })
}

/// Solve, then certify — the ONE production entry point, so no call-site can obtain an answer
/// while forgetting its certification (`302` §3). On the inconsistent path it also runs the
/// instrumented re-run and fills the replay slot, so no call-site can hold an inconsistency
/// verdict while losing its account.
#[must_use]
pub fn solve_certified<G: Graph, L: Lattice>(
    graph: &G,
    direction: Direction,
    transfer: impl Fn(usize, &L) -> L,
) -> (Solution<L>, SolveConsistency<L>) {
    let solution = solve(graph, direction, &transfer);
    let init = vec![L::bottom(); graph.node_count()];
    let certified = certify_solution(graph, direction, &init, &transfer, &solution);
    let certified = match certified {
        SolveConsistency::Inconsistent(report) => {
            let focus = report.failing().nodes();
            let replay = replay_solve(graph, direction, &transfer, focus);
            SolveConsistency::Inconsistent(report.with_replay(replay))
        }
        consistent @ SolveConsistency::Consistent(_) => consistent,
    };
    (solution, certified)
}

/// Re-run the identical solve with instrumentation on, retaining the updates that landed on
/// `focus` (`302:rul-rerun-is-the-self-report-engine`).
///
/// The trajectory is identical BY CONSTRUCTION, not by test: this drives the very same
/// [`run`](crate::solve::run) loop the production solver does, differing only in an observer that
/// receives and never returns. The `inv-determinism` purity of the kernel is what makes replaying
/// a defect meaningful at all.
#[must_use]
pub fn replay_solve<G: Graph, L: Lattice>(
    graph: &G,
    direction: Direction,
    transfer: impl Fn(usize, &L) -> L,
    focus: SortedSet<usize>,
) -> SolveReplay<L> {
    let mut recorder = FocusedReplay {
        focus,
        updates: Vec::new(),
        total: 0,
    };
    let _ = run(graph, direction, transfer, &mut recorder);
    SolveReplay {
        updates: recorder.updates,
        total: recorder.total,
        focus: recorder.focus,
    }
}

/// The recording observer. Receives and never returns — it cannot perturb the trajectory it exists
/// to witness.
struct FocusedReplay<L> {
    focus: SortedSet<usize>,
    updates: Vec<ReplayUpdate<L>>,
    total: usize,
}

impl<L: Clone> SolveObserver<L> for FocusedReplay<L> {
    fn observe_update(&mut self, round: usize, from: usize, to: usize, old: &L, new: &L) {
        if !self.focus.contains(&to) {
            return;
        }
        self.total = self.total.saturating_add(1);
        if self.updates.len() < REPLAY_UPDATE_CAP {
            self.updates.push(ReplayUpdate {
                round,
                from,
                to,
                old: old.clone(),
                new: new.clone(),
            });
        }
    }
}

/// The nodes `from`'s output flows to under `direction` — the solver's own oriented edge view,
/// spelled once so the checker and the solver cannot disagree about what an edge is.
fn flows_to<G: Graph>(graph: &G, direction: Direction, from: usize) -> &[usize] {
    match direction {
        Direction::Forward => graph.succ(from),
        Direction::Backward => graph.pred(from),
    }
}

/// The failing edges whose SOURCE is itself fully clean — where consistency first breaks along the
/// flow. A source is clean iff its own boundary check passed and no failing edge arrives at it,
/// which the complete index set answers on its own (no second graph walk).
fn first_break_edges(failing: &FailingChecks) -> SortedSet<(usize, usize)> {
    let mut has_failing_incoming = SortedSet::new();
    for &(_, to) in &failing.edges {
        has_failing_incoming.insert(to);
    }
    let mut out = SortedSet::new();
    for &(from, to) in &failing.edges {
        if !failing.boundary.contains(&from) && !has_failing_incoming.contains(&from) {
            out.insert((from, to));
        }
    }
    out
}

/// The non-trivial strongly-connected components touching the failing region, in least-member
/// order (`302` §2, as amended: a singleton without a self-loop names no region).
///
/// Iterative Tarjan — no recursion, so a deep graph cannot blow the stack (`inv-no-throw`).
fn unstable_components<G: Graph>(
    graph: &G,
    direction: Direction,
    failing: &FailingChecks,
) -> Vec<SortedSet<usize>> {
    let region = failing.nodes();
    let mut components: Vec<SortedSet<usize>> = Vec::new();
    for component in strongly_connected(graph, direction) {
        let touches_failure = component.iter().any(|node| region.contains(node));
        if touches_failure && is_non_trivial(graph, direction, &component) {
            components.push(component);
        }
    }
    components.sort_by_key(|component| component.get_at(0).copied());
    components
}

/// A component names an un-stabilized REGION only if it can actually cycle: two or more members,
/// or a lone member that reaches itself.
fn is_non_trivial<G: Graph>(graph: &G, direction: Direction, component: &SortedSet<usize>) -> bool {
    if component.len() >= 2 {
        return true;
    }
    component
        .get_at(0)
        .is_some_and(|&node| flows_to(graph, direction, node).contains(&node))
}

/// Tarjan's strongly-connected components over the solver-oriented edge view, iteratively.
fn strongly_connected<G: Graph>(graph: &G, direction: Direction) -> Vec<SortedSet<usize>> {
    let node_count = graph.node_count();
    let mut index_of: Vec<Option<usize>> = vec![None; node_count];
    let mut lowlink: Vec<usize> = vec![0; node_count];
    let mut on_stack: Vec<bool> = vec![false; node_count];
    let mut stack: Vec<usize> = Vec::new();
    let mut components: Vec<SortedSet<usize>> = Vec::new();
    let mut next_index = 0usize;

    for root in 0..node_count {
        if index_of.get(root).is_some_and(Option::is_some) {
            continue;
        }
        let mut frames: Vec<(usize, usize)> = vec![(root, 0)];
        open(
            root,
            &mut index_of,
            &mut lowlink,
            &mut on_stack,
            &mut stack,
            &mut next_index,
        );
        while let Some(&(node, cursor)) = frames.last() {
            let edges = flows_to(graph, direction, node);
            if let Some(&next) = edges.get(cursor) {
                if let Some(frame) = frames.last_mut() {
                    frame.1 = cursor.saturating_add(1);
                }
                if next >= node_count {
                    continue;
                }
                match index_of.get(next).copied().flatten() {
                    None => {
                        open(
                            next,
                            &mut index_of,
                            &mut lowlink,
                            &mut on_stack,
                            &mut stack,
                            &mut next_index,
                        );
                        frames.push((next, 0));
                    }
                    Some(seen) => {
                        if on_stack.get(next).copied().unwrap_or(false) {
                            lower(&mut lowlink, node, seen);
                        }
                    }
                }
                continue;
            }
            frames.pop();
            let own_low = lowlink.get(node).copied().unwrap_or(0);
            if let Some(&(parent, _)) = frames.last() {
                lower(&mut lowlink, parent, own_low);
            }
            if index_of.get(node).copied().flatten() == Some(own_low) {
                components.push(pop_component(node, &mut stack, &mut on_stack));
            }
        }
    }
    components
}

/// Give `node` its discovery index and push it onto the component stack.
fn open(
    node: usize,
    index_of: &mut [Option<usize>],
    lowlink: &mut [usize],
    on_stack: &mut [bool],
    stack: &mut Vec<usize>,
    next_index: &mut usize,
) {
    if let Some(slot) = index_of.get_mut(node) {
        *slot = Some(*next_index);
    }
    if let Some(slot) = lowlink.get_mut(node) {
        *slot = *next_index;
    }
    if let Some(slot) = on_stack.get_mut(node) {
        *slot = true;
    }
    stack.push(node);
    *next_index = next_index.saturating_add(1);
}

/// Pull `node`'s lowlink down to `candidate` if that is lower.
fn lower(lowlink: &mut [usize], node: usize, candidate: usize) {
    if let Some(slot) = lowlink.get_mut(node)
        && candidate < *slot
    {
        *slot = candidate;
    }
}

/// Pop the component rooted at `root` off the stack.
fn pop_component(root: usize, stack: &mut Vec<usize>, on_stack: &mut [bool]) -> SortedSet<usize> {
    let mut component = SortedSet::new();
    while let Some(member) = stack.pop() {
        if let Some(slot) = on_stack.get_mut(member) {
            *slot = false;
        }
        component.insert(member);
        if member == root {
            break;
        }
    }
    component
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::{Flat, Must, Powerset};

    struct TestGraph {
        succ: Vec<Vec<usize>>,
        pred: Vec<Vec<usize>>,
    }

    impl TestGraph {
        fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
            let mut succ = vec![Vec::new(); n];
            let mut pred = vec![Vec::new(); n];
            for &(a, b) in edges {
                succ[a].push(b);
                pred[b].push(a);
            }
            TestGraph { succ, pred }
        }
    }

    impl Graph for TestGraph {
        fn node_count(&self) -> usize {
            self.succ.len()
        }
        fn succ(&self, v: usize) -> &[usize] {
            &self.succ[v]
        }
        fn pred(&self, v: usize) -> &[usize] {
            &self.pred[v]
        }
    }

    fn set(xs: &[usize]) -> Powerset<usize> {
        xs.iter().copied().collect()
    }

    /// Forward-may "gen": out = in ∪ {node-id}. Monotone + bounded.
    fn gen_xfer(v: usize, inp: &Powerset<usize>) -> Powerset<usize> {
        let mut s = inp.clone();
        s.insert(v);
        s
    }

    /// A transfer that climbs forever: the finite-height precondition violated on purpose, which
    /// is the ONLY way this solver fails to settle (its update is `state ⊔ out`, so states ascend
    /// monotonically and true oscillation is unrepresentable).
    fn runaway(_: usize, s: &Powerset<u64>) -> Powerset<u64> {
        let mut t = s.clone();
        t.insert(u64::try_from(s.len()).unwrap_or(u64::MAX));
        t
    }

    fn chain4() -> TestGraph {
        TestGraph::from_edges(4, &[(0, 1), (1, 2), (2, 3)])
    }

    fn all_bottom(n: usize) -> Vec<Powerset<usize>> {
        vec![Powerset::bottom(); n]
    }

    fn failed<L>(outcome: &SolveConsistency<L>) -> &FailedChecks<L> {
        match outcome {
            SolveConsistency::Inconsistent(report) => report,
            SolveConsistency::Consistent(passed) => {
                panic!("expected Inconsistent, got Consistent({})", passed.checks())
            }
        }
    }

    fn edges_of(pairs: &[(usize, usize)]) -> SortedSet<(usize, usize)> {
        pairs.iter().copied().collect()
    }

    fn nodes_of(nodes: &[usize]) -> SortedSet<usize> {
        nodes.iter().copied().collect()
    }

    /// §6.1 — a real fixpoint certifies, and the checker really did look at every inequality (the
    /// count is the non-vacuity witness: 4 boundary + 3 edges, none skipped).
    #[test]
    fn a_real_fixpoint_certifies_and_every_check_ran() {
        let g = chain4();
        let solution = solve(&g, Direction::Forward, gen_xfer);
        let outcome = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        match outcome {
            SolveConsistency::Consistent(passed) => assert_eq!(passed.checks(), 7),
            SolveConsistency::Inconsistent(_) => panic!("a real fixpoint must certify"),
        }
    }

    /// §6.1 — LOWERING a state breaks the edge that FEEDS it, and nothing else. The exact item is
    /// asserted, not merely the verdict.
    #[test]
    fn lowering_one_state_names_exactly_the_edge_that_feeds_it() {
        let g = chain4();
        let mut solution = solve(&g, Direction::Forward, gen_xfer);
        solution.states[3] = set(&[]);

        let outcome = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        let report = failed(&outcome);
        assert!(report.failing().boundary().is_empty(), "⊥ seeds still hold");
        assert_eq!(*report.failing().edges(), edges_of(&[(2, 3)]));
        assert_eq!(report.total(), 1);
        assert_eq!(report.shown(), 1);
        assert_eq!(
            report.inconsistencies(),
            &[Inconsistency::Edge {
                from: 2,
                to: 3,
                transferred: set(&[0, 1, 2]),
                state: set(&[]),
            }]
        );
        assert_eq!(*report.first_break_edges(), edges_of(&[(2, 3)]));
        assert!(
            report.unstable_components().is_empty(),
            "a first-break edge exists, so no component summary is owed"
        );
    }

    /// §6.1 — RAISING a state breaks the edge LEAVING it (its transfer now over-runs its
    /// successor): the opposite end from the lowering case.
    #[test]
    fn raising_one_state_breaks_the_edge_leaving_it() {
        let g = chain4();
        let mut solution = solve(&g, Direction::Forward, gen_xfer);
        solution.states[1] = set(&[0, 99]);

        let outcome = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        let report = failed(&outcome);
        assert_eq!(*report.failing().edges(), edges_of(&[(1, 2)]));
        assert_eq!(*report.first_break_edges(), edges_of(&[(1, 2)]));
    }

    /// §6.1 + §6.4 — SWAPPING two states fails two edges, reported in canonical
    /// (node × successor index) order; the first-break summary keeps only the UPSTREAM one,
    /// because the downstream failure is a CASUALTY of the first, not a second defect.
    #[test]
    fn swapped_states_are_canonical_and_first_break_excludes_the_casualty() {
        let g = chain4();
        let mut solution = solve(&g, Direction::Forward, gen_xfer);
        solution.states.swap(1, 3);

        let outcome = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        let report = failed(&outcome);
        assert_eq!(*report.failing().edges(), edges_of(&[(1, 2), (2, 3)]));
        assert_eq!(report.total(), 2);
        match report.inconsistencies() {
            [
                Inconsistency::Edge { from: 1, to: 2, .. },
                Inconsistency::Edge { from: 2, to: 3, .. },
            ] => {}
            other => panic!("canonical order is (node × successor index); got {other:?}"),
        }
        assert_eq!(
            *report.first_break_edges(),
            edges_of(&[(1, 2)]),
            "(2,3)'s source has a failing incoming edge, so it is downstream of the break"
        );
    }

    /// §6.1 — the by-value items cap while the INDEX set stays whole, and `shown`/`total`
    /// disclose the difference rather than hiding it.
    #[test]
    fn items_cap_but_the_index_set_stays_whole() {
        let n = 20;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let g = TestGraph::from_edges(n, &edges);
        let mut solution = solve(&g, Direction::Forward, gen_xfer);
        for state in &mut solution.states {
            *state = set(&[]);
        }

        let outcome = certify_solution(&g, Direction::Forward, &all_bottom(n), gen_xfer, &solution);
        let report = failed(&outcome);
        assert_eq!(report.failing().edges().len(), n - 1);
        assert_eq!(report.total(), n - 1);
        assert_eq!(report.shown(), INCONSISTENCY_CAP);
    }

    /// §6.2 — the boundary family is NON-VACUOUS: a violated seed is caught at an ENTRY node (no
    /// in-edges, so no per-edge check could ever see it) and at an INTERIOR node.
    #[test]
    fn violated_seeds_are_caught_at_entry_and_interior_nodes() {
        let g = chain4();
        let solution = solve(&g, Direction::Forward, gen_xfer);

        let mut init = all_bottom(4);
        init[0] = set(&[7]);
        let entry = certify_solution(&g, Direction::Forward, &init, gen_xfer, &solution);
        assert_eq!(
            *failed(&entry).failing().boundary(),
            nodes_of(&[0]),
            "an entry seed is visible ONLY to the boundary family"
        );

        let mut init = all_bottom(4);
        init[2] = set(&[99]);
        let interior = certify_solution(&g, Direction::Forward, &init, gen_xfer, &solution);
        assert_eq!(*failed(&interior).failing().boundary(), nodes_of(&[2]));
    }

    /// §6.2 — the boundary family under the ORDER-DUAL, witnessed by a test rather than argued:
    /// `Must`'s ⊥ is `L`'s ⊤, so the VIOLATING seed is `Must(Bottom)` — the dual's ⊤.
    #[test]
    fn the_dual_orders_boundary_check_is_exercised() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let xfer = |v: usize, inp: &Must<Flat<u8>>| match v {
            1 | 2 => Must(Flat::Elem(5)),
            _ => inp.clone(),
        };
        let solution = solve(&g, Direction::Forward, xfer);
        assert_eq!(solution.states[3], Must(Flat::Elem(5)));

        let holds = certify_solution(
            &g,
            Direction::Forward,
            &vec![Must::<Flat<u8>>::bottom(); 4],
            xfer,
            &solution,
        );
        assert!(holds.is_consistent(), "the dual's ⊥ seed is ⊑ every state");

        let mut init = vec![Must::<Flat<u8>>::bottom(); 4];
        init[3] = Must(Flat::Bottom);
        let violated = certify_solution(&g, Direction::Forward, &init, xfer, &solution);
        assert_eq!(
            *failed(&violated).failing().boundary(),
            nodes_of(&[3]),
            "Must(Bottom) is the dual's ⊤ and is NOT ⊑ Must(Elem(5))"
        );
    }

    /// §6.2 — and in the BACKWARD orientation, where the boundary sits at the exit instead.
    #[test]
    fn the_boundary_family_runs_in_the_backward_orientation_too() {
        let g = chain4();
        let solution = solve(&g, Direction::Backward, gen_xfer);
        assert_eq!(
            solution.states[3],
            set(&[]),
            "exit is the backward ⊥ boundary"
        );

        let mut init = all_bottom(4);
        init[3] = set(&[42]);
        let outcome = certify_solution(&g, Direction::Backward, &init, gen_xfer, &solution);
        assert_eq!(*failed(&outcome).failing().boundary(), nodes_of(&[3]));
    }

    /// §6.5 — ONE checker covers both orientations: same call shape, same `Direction`, no
    /// may/must flag anywhere. The duality is carried by the lattice TYPE, and that is the whole
    /// mechanism (`must-lattice-by-type`).
    #[test]
    fn one_checker_certifies_a_may_system_and_its_must_dual() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);

        let may = solve(&g, Direction::Forward, gen_xfer);
        let may_outcome = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &may);

        let must_xfer = |v: usize, inp: &Must<Flat<u8>>| match v {
            1 => Must(Flat::Elem(5)),
            2 => Must(Flat::Elem(6)),
            _ => inp.clone(),
        };
        let must = solve(&g, Direction::Forward, must_xfer);
        let must_outcome = certify_solution(
            &g,
            Direction::Forward,
            &vec![Must::<Flat<u8>>::bottom(); 4],
            must_xfer,
            &must,
        );

        assert!(may_outcome.is_consistent());
        assert!(must_outcome.is_consistent());
        assert_eq!(
            must.states[3],
            Must(Flat::Bottom),
            "branches disagree ⇒ the dual joins to ⊥, and that fixpoint certifies"
        );
    }

    /// §6.3 — a RUNAWAY CLIMB under the round-cap reports `Inconsistent`; with the only failing
    /// edge arriving at its own source there is no first-break, so the UNSTABLE COMPONENT is what
    /// names the region.
    #[test]
    fn a_runaway_climb_at_the_cap_is_localized_by_its_unstable_component() {
        let g = TestGraph::from_edges(1, &[(0, 0)]);
        let solution = solve(&g, Direction::Forward, runaway);
        assert!(!solution.converged, "an unbounded climb trips the cap");

        let outcome = certify_solution(
            &g,
            Direction::Forward,
            &vec![Powerset::<u64>::bottom(); 1],
            runaway,
            &solution,
        );
        let report = failed(&outcome);
        assert_eq!(*report.failing().edges(), edges_of(&[(0, 0)]));
        assert!(
            report.first_break_edges().is_empty(),
            "nothing upstream is clean — the failing edge arrives at its own source"
        );
        assert_eq!(report.unstable_components(), &[nodes_of(&[0])]);
    }

    /// §6.3, the other way — an answer that LANDED on its fixpoint but ran out of rounds before
    /// noticing is legitimately `Consistent`. Only the ADVISORY FLAG is hand-written; the states
    /// come from a real solve and the checker runs for real (`303` §4).
    #[test]
    fn landing_on_the_fixpoint_at_the_cap_still_certifies() {
        let g = chain4();
        let mut solution = solve(&g, Direction::Forward, gen_xfer);
        assert!(solution.converged, "the real solve converged");
        solution.converged = false;

        let outcome = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        assert!(
            outcome.is_consistent(),
            "`converged` is advisory; the STATES are what certify"
        );
    }

    /// §6.3 — a trivial singleton (no self-loop) names no region, so a failing node in an acyclic
    /// graph yields no component summary even with no first-break edge to report.
    #[test]
    fn a_singleton_without_a_self_loop_names_no_region() {
        let g = chain4();
        let solution = solve(&g, Direction::Forward, gen_xfer);
        let mut init = all_bottom(4);
        init[0] = set(&[7]);

        let outcome = certify_solution(&g, Direction::Forward, &init, gen_xfer, &solution);
        let report = failed(&outcome);
        assert!(
            report.first_break_edges().is_empty(),
            "no edge failed at all"
        );
        assert!(
            report.unstable_components().is_empty(),
            "an acyclic failing node is not an un-stabilized REGION"
        );
    }

    /// §6.6 — the whole outcome is a pure function of its inputs.
    #[test]
    fn certification_repeats_identically() {
        let g = chain4();
        let mut solution = solve(&g, Direction::Forward, gen_xfer);
        solution.states[3] = set(&[]);
        let once = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        let twice = certify_solution(&g, Direction::Forward, &all_bottom(4), gen_xfer, &solution);
        assert_eq!(once, twice);
    }

    /// §6.6 — under EDGE-INSERTION PERMUTATION the verdict, the failing SET, and both summaries
    /// compare equal. The by-value item SEQUENCE legitimately reorders with the successor order,
    /// which is exactly why the index set is a SET
    /// (`303:fnd-permutation-pin-is-set-not-sequence`).
    #[test]
    fn permuting_edge_insertion_moves_no_set_and_no_summary() {
        let forward = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let reversed = TestGraph::from_edges(4, &[(0, 2), (0, 1), (2, 3), (1, 3)]);

        let mut solution = solve(&forward, Direction::Forward, gen_xfer);
        solution.states[1] = set(&[]);
        solution.states[2] = set(&[]);

        let a = certify_solution(
            &forward,
            Direction::Forward,
            &all_bottom(4),
            gen_xfer,
            &solution,
        );
        let b = certify_solution(
            &reversed,
            Direction::Forward,
            &all_bottom(4),
            gen_xfer,
            &solution,
        );
        let (a, b) = (failed(&a), failed(&b));

        assert_eq!(a.failing(), b.failing());
        assert_eq!(a.first_break_edges(), b.first_break_edges());
        assert_eq!(a.unstable_components(), b.unstable_components());
        assert_eq!(a.total(), b.total());
        assert_eq!(*a.failing().edges(), edges_of(&[(0, 1), (0, 2)]));
    }

    /// §6.9 — the instrumented run reproduces the production trajectory. It is identical BY
    /// CONSTRUCTION (one loop, an observer that only receives); this pins that the seam did not
    /// change the answer — the instrumented Solution equals the plain solver's, exactly.
    #[test]
    fn the_instrumented_run_reproduces_the_plain_solution() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let plain = solve(&g, Direction::Forward, gen_xfer);

        let mut recorder = FocusedReplay {
            focus: nodes_of(&[0, 1, 2, 3]),
            updates: Vec::new(),
            total: 0,
        };
        let instrumented = run(&g, Direction::Forward, gen_xfer, &mut recorder);

        assert_eq!(plain, instrumented, "the observer perturbs nothing");
        assert!(recorder.total > 0, "the run really did record updates");
    }

    /// §6.9 — the recorded sequence is itself deterministic, so a replay is a stable ACCOUNT
    /// rather than a re-derivation that might differ from the run it explains.
    #[test]
    fn the_recorded_update_sequence_is_deterministic() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let focus = nodes_of(&[0, 1, 2, 3]);
        let once = replay_solve(&g, Direction::Forward, gen_xfer, focus.clone());
        let twice = replay_solve(&g, Direction::Forward, gen_xfer, focus);
        assert_eq!(once, twice);
    }

    /// §6.9 — the slice is exactly the failing region, and `solve_certified` wires it in without
    /// the call-site asking.
    #[test]
    fn the_replay_slice_is_exactly_the_failing_region() {
        let g = TestGraph::from_edges(1, &[(0, 0)]);
        let (_, outcome) = solve_certified(&g, Direction::Forward, runaway);
        let report = failed(&outcome);

        assert_eq!(*report.replay().focus(), report.failing().nodes());
        assert!(
            report.replay().updates().iter().all(|u| u.to == 0),
            "every retained update landed on a focused node"
        );
    }

    /// §6.9 — the disclosed cap is honoured: retention stops at [`REPLAY_UPDATE_CAP`] while the
    /// total keeps counting, so a surface can say how much it is not showing.
    #[test]
    fn the_replay_honours_its_disclosed_cap() {
        let g = TestGraph::from_edges(1, &[(0, 0)]);
        let (_, outcome) = solve_certified(&g, Direction::Forward, runaway);
        let replay = failed(&outcome).replay();

        assert_eq!(replay.shown(), REPLAY_UPDATE_CAP);
        assert!(
            replay.total() > REPLAY_UPDATE_CAP,
            "the runaway climb produced more than the cap retains ({} total)",
            replay.total()
        );
    }

    /// A consistent answer costs no replay: the checker mints the slot empty and only the
    /// inconsistent path pays for an instrumented re-run (`302` §2).
    #[test]
    fn a_consistent_answer_costs_no_replay() {
        let g = chain4();
        let (_, outcome) = solve_certified(&g, Direction::Forward, gen_xfer);
        assert!(outcome.is_consistent());
    }
}
