//! The fixed-point solver â€” a propagation worklist generic over any [`Lattice`]
//! and a [`Graph`], in either [`Direction`].
//!
//! Pure + deterministic (`inv-determinism`): FIFO worklist, graph-order
//! neighbours, ordered lattice values â‡’ identical inputs converge to the
//! identical per-node fixed point.
//!
//! Termination is guaranteed ONLY when the caller upholds the preconditions
//! below; the type system cannot express them (see `Research/notes/165`), so the
//! solver fails *loud, not silent*: a precondition violation trips a generous
//! iteration cap and returns [`Solution::converged`]` == false` rather than
//! hanging (this was an empirically-real infinite loop, not a theoretical one â€”
//! note 164). A correctness-critical caller MUST check `converged`.

use crate::lattice::Lattice;
use std::collections::VecDeque;

/// A directed graph over nodes `0..node_count()`. The CFG implements this; the
/// solver stays decoupled so it can be validated on toy graphs and reused by
/// every analysis.
///
/// **Precondition:** every id returned by `succ`/`pred` is `< node_count()`.
/// `solve` `debug_assert`s this and, in release, defensively skips an
/// out-of-range edge rather than panicking (`inv-no-throw`).
pub trait Graph {
    fn node_count(&self) -> usize;
    /// Forward edges out of `node`.
    fn succ(&self, node: usize) -> &[usize];
    /// Reverse edges into `node`.
    fn pred(&self, node: usize) -> &[usize];
}

/// Dataflow direction â€” the only axis distinguishing, e.g., reaching-definitions
/// (forward) from the apply-phase minimization slice (backward). Same solver,
/// same lattice; only which neighbours a node's output flows to changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// The result of [`solve`].
///
/// `states[v]` is the *input* abstract state at node `v` (the state immediately
/// before it, for a forward analysis; after it, for backward); the output state
/// is `transfer(v, &states[v])`. `converged` is `false` iff the iteration cap was
/// hit before a fixed point â€” which happens ONLY when a [`solve`] precondition
/// was violated; a well-formed analysis always converges. `rounds` is the number
/// of node-visits performed (diagnostic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution<L> {
    pub states: Vec<L>,
    pub converged: bool,
    pub rounds: usize,
}

/// Solve a monotone dataflow problem to its least fixed point.
///
/// **Preconditions the caller must uphold** (the type system cannot â€” note 165):
/// 1. `transfer` is **monotone** (`x âŠ‘ y â‡’ f(x) âŠ‘ f(y)`);
/// 2. the lattice `L` has **finite height** for the values this analysis can
///    actually produce (e.g. a `MapL`/`Powerset` whose keys/elements are drawn
///    from a *bounded* set â€” a transfer that mints a fresh key/element every
///    visit climbs forever);
/// 3. `L`'s `Eq` is **semantic** (agrees with lattice equality);
/// 4. every graph edge endpoint is `< node_count()`.
///
/// Violating 1/2/3 is caught as `Solution::converged == false` (never a hang â€”
/// the iteration cap). Violating 4 is a `debug_assert` (release: skipped edge).
#[must_use]
pub fn solve<G: Graph, L: Lattice>(
    graph: &G,
    direction: Direction,
    transfer: impl Fn(usize, &L) -> L,
) -> Solution<L> {
    let n = graph.node_count();
    // A node's output flows to its successors (forward) or predecessors
    // (backward) â€” its consumer set, where we propagate-and-join.
    let flows_to = |v: usize| -> &[usize] {
        match direction {
            Direction::Forward => graph.succ(v),
            Direction::Backward => graph.pred(v),
        }
    };

    let mut state: Vec<L> = vec![L::bottom(); n];
    let mut queued: Vec<bool> = vec![true; n];
    let mut work: VecDeque<usize> = (0..n).collect();

    // Backstop: a well-behaved (monotone + finite-height) problem settles in far
    // fewer visits than this. Hitting it means a precondition was violated; we
    // stop and report non-convergence rather than loop forever.
    let cap = n.saturating_mul(1024).saturating_add(4096);
    let mut rounds = 0usize;
    let mut converged = true;

    while let Some(v) = work.pop_front() {
        if rounds >= cap {
            converged = false;
            break;
        }
        rounds += 1;
        queued[v] = false;
        let out = transfer(v, &state[v]);
        for &w in flows_to(v) {
            debug_assert!(
                w < n,
                "Graph edge endpoint {w} out of range (node_count {n})"
            );
            if w >= n {
                continue; // release-mode defensive skip â€” never panic (inv-no-throw)
            }
            let joined = state[w].join(&out);
            if joined != state[w] {
                state[w] = joined;
                if !queued[w] {
                    queued[w] = true;
                    work.push_back(w);
                }
            }
        }
    }
    Solution {
        states: state,
        converged,
        rounds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::Powerset;
    use std::collections::BTreeSet;

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
        Powerset(xs.iter().copied().collect::<BTreeSet<_>>())
    }

    /// Forward-may "gen" transfer: out = in âˆª {node-id}. Monotone + bounded.
    fn gen_xfer(v: usize, inp: &Powerset<usize>) -> Powerset<usize> {
        let mut s = inp.clone();
        s.0.insert(v);
        s
    }

    #[test]
    fn forward_chain_accumulates() {
        let g = TestGraph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let r = solve(&g, Direction::Forward, gen_xfer);
        assert!(r.converged);
        assert_eq!(r.states[0], set(&[]), "entry has no predecessors â‡’ âŠ¥");
        assert_eq!(
            r.states[3],
            set(&[0, 1, 2]),
            "everything generated upstream reaches node 3"
        );
    }

    #[test]
    fn forward_diamond_joins_at_merge() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let r = solve(&g, Direction::Forward, gen_xfer);
        assert!(r.converged);
        assert_eq!(
            r.states[3],
            set(&[0, 1, 2]),
            "both branches join at the merge"
        );
    }

    #[test]
    fn forward_cycle_terminates_at_fixed_point() {
        let g = TestGraph::from_edges(3, &[(0, 1), (1, 2), (2, 1)]);
        let r = solve(&g, Direction::Forward, gen_xfer);
        assert!(r.converged);
        assert_eq!(
            r.states[1],
            set(&[0, 1, 2]),
            "loop body reaches its own fixed point"
        );
        assert_eq!(r.states[2], set(&[0, 1, 2]));
    }

    #[test]
    fn backward_propagates_against_edges() {
        let g = TestGraph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let r = solve(&g, Direction::Backward, gen_xfer);
        assert!(r.converged);
        assert_eq!(
            r.states[3],
            set(&[]),
            "exit has no successors â‡’ âŠ¥ (backward boundary)"
        );
        assert!(
            r.states[0].contains(&3),
            "node 3's fact flows backward to node 0"
        );
        assert_eq!(
            r.states[0],
            set(&[1, 2, 3]),
            "all downstream gens are live at the entry"
        );
    }

    #[test]
    fn solve_is_deterministic() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let a = solve(&g, Direction::Forward, gen_xfer);
        let b = solve(&g, Direction::Forward, gen_xfer);
        assert_eq!(a, b, "same graph + transfer â‡’ identical solution");
    }

    #[test]
    fn cap_reports_nonconvergence_instead_of_hanging() {
        // A MONOTONE transfer over an UNBOUNDED lattice (a fresh element every
        // visit) violates the finite-height precondition and would climb forever.
        // The cap must make `solve` RETURN with converged=false, not hang.
        let g = TestGraph::from_edges(1, &[(0, 0)]); // self-loop
        let r = solve(&g, Direction::Forward, |_, s: &Powerset<u64>| {
            let mut t = s.clone();
            t.0.insert(u64::try_from(s.0.len()).unwrap_or(u64::MAX)); // always a new element
            t
        });
        assert!(
            !r.converged,
            "unbounded climb must report non-convergence, not loop"
        );
        assert!(r.rounds > 0);
    }

    #[test]
    fn empty_graph_is_fine() {
        let g = TestGraph::from_edges(0, &[]);
        let r = solve(&g, Direction::Forward, gen_xfer);
        assert!(r.converged);
        assert!(r.states.is_empty());
    }

    #[test]
    fn solve_runs_a_must_analysis_over_the_dual() {
        use crate::lattice::{Flat, Must};
        // The engine-wide-meet payoff (note 165 L1): a *must* analysis needs no new
        // solver â€” running the unchanged worklist over the order-dual `Must<L>`
        // turns âŠ”-at-merges into âŠ“-at-merges. On a diamond, a fact is must-true at
        // the join only if BOTH branches agree.
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);

        let agree = solve(&g, Direction::Forward, |v, inp: &Must<Flat<u8>>| match v {
            1 | 2 => Must(Flat::Elem(5)),
            _ => inp.clone(),
        });
        assert!(agree.converged);
        assert_eq!(
            agree.states[3],
            Must(Flat::Elem(5)),
            "both branches agree â‡’ must-Elem(5)"
        );

        let disagree = solve(&g, Direction::Forward, |v, inp: &Must<Flat<u8>>| match v {
            1 => Must(Flat::Elem(5)),
            2 => Must(Flat::Elem(6)),
            _ => inp.clone(),
        });
        assert!(disagree.converged);
        assert_eq!(
            disagree.states[3],
            Must(Flat::Bottom),
            "branches disagree â‡’ âŠ“ â‡’ âŠ¥, no must-fact"
        );
    }
}
