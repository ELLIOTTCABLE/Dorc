//! The fixed-point solver — a propagation worklist generic over any [`Lattice`]
//! and a [`Graph`], in either [`Direction`].
//!
//! Pure + deterministic (`inv-determinism`): the worklist is FIFO and neighbour
//! order is the graph's, so identical inputs always converge to the identical
//! per-node fixed point. Terminates because the lattice has finite height (each
//! state can only climb finitely far) and the worklist de-duplicates.

use crate::lattice::Lattice;
use std::collections::VecDeque;

/// A directed graph over nodes `0..node_count()`. The CFG implements this; the
/// solver stays decoupled from the concrete CFG so it can be validated against
/// toy graphs and reused by every analysis.
pub trait Graph {
    fn node_count(&self) -> usize;
    /// Forward edges out of `node`.
    fn succ(&self, node: usize) -> &[usize];
    /// Reverse edges into `node`.
    fn pred(&self, node: usize) -> &[usize];
}

/// Dataflow direction — the *only* axis that distinguishes, e.g., reaching-
/// definitions (forward) from the apply-phase minimization slice (backward).
/// Same solver, same lattice; only which neighbours a node's output flows to
/// changes (successors for forward, predecessors for backward).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

/// Solve a monotone dataflow problem to its least fixed point.
///
/// Returns the *input* abstract state for each node: the state at the program
/// point immediately **before** the node for a forward analysis (after, for
/// backward). A node's output state is `transfer(node, &result[node])`.
///
/// `transfer` must be monotone (see [`Lattice`]). The entry/exit boundary needs
/// no separate argument — it is seeded by `transfer` itself (a node with no
/// inflowing neighbours simply transfers `⊥`).
#[must_use]
pub fn solve<G: Graph, L: Lattice>(
    graph: &G,
    direction: Direction,
    transfer: impl Fn(usize, &L) -> L,
) -> Vec<L> {
    let n = graph.node_count();
    // A node's output flows to its successors (forward) or predecessors
    // (backward). That consumer set is where we propagate-and-join.
    let flows_to = |v: usize| -> &[usize] {
        match direction {
            Direction::Forward => graph.succ(v),
            Direction::Backward => graph.pred(v),
        }
    };

    let mut state: Vec<L> = vec![L::bottom(); n];
    let mut queued: Vec<bool> = vec![true; n];
    let mut work: VecDeque<usize> = (0..n).collect();

    while let Some(v) = work.pop_front() {
        queued[v] = false;
        let out = transfer(v, &state[v]);
        for &w in flows_to(v) {
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
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::Powerset;
    use std::collections::BTreeSet;

    /// Adjacency-list test graph.
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

    /// A forward-may "gen" transfer: out = in ∪ {node-id}.
    fn gen(v: usize, inp: &Powerset<usize>) -> Powerset<usize> {
        let mut s = inp.clone();
        s.0.insert(v);
        s
    }

    #[test]
    fn forward_chain_accumulates() {
        // 0 → 1 → 2 → 3. before[v] = union of predecessors' outputs.
        let g = TestGraph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let r = solve(&g, Direction::Forward, gen);
        assert_eq!(r[0], set(&[]), "entry has no predecessors ⇒ ⊥");
        assert_eq!(r[3], set(&[0, 1, 2]), "everything generated upstream reaches node 3");
    }

    #[test]
    fn forward_diamond_joins_at_merge() {
        // 0 →{1,2}→ 3. The merge node 3 sees both branches' gens (the ⊔/JOIN).
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let r = solve(&g, Direction::Forward, gen);
        assert_eq!(r[3], set(&[0, 1, 2]), "both branches join at the merge");
    }

    #[test]
    fn forward_cycle_terminates_at_fixed_point() {
        // 0 → 1 ⇄ 2 (back-edge 2→1). The loop must converge, not spin.
        let g = TestGraph::from_edges(3, &[(0, 1), (1, 2), (2, 1)]);
        let r = solve(&g, Direction::Forward, gen);
        assert_eq!(r[1], set(&[0, 1, 2]), "loop body reaches its own fixed point");
        assert_eq!(r[2], set(&[0, 1, 2]));
    }

    #[test]
    fn backward_propagates_against_edges() {
        // Same chain 0 → 1 → 2 → 3, solved BACKWARD: node 3's gen must reach 0.
        let g = TestGraph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]);
        let r = solve(&g, Direction::Backward, gen);
        assert_eq!(r[3], set(&[]), "exit has no successors ⇒ ⊥ (backward boundary)");
        assert!(r[0].contains(&3), "node 3's fact flows backward to node 0");
        assert_eq!(r[0], set(&[1, 2, 3]), "all downstream gens are live at the entry");
    }

    #[test]
    fn solve_is_deterministic() {
        let g = TestGraph::from_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let a = solve(&g, Direction::Forward, gen);
        let b = solve(&g, Direction::Forward, gen);
        assert_eq!(a, b, "same graph + transfer ⇒ identical fixed point");
    }
}
