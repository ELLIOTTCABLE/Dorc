//! VERBATIM extract of the kernel algebra tier, for measuring rewrite distance to
//! Aeneas-translatable Rust. Both modules are copied byte-for-byte from
//! `spike/crates/analysis/src/{lattice,solve}.rs` minus their `#[cfg(test)]` tails
//! (charon compiles a normal `--crate-type=lib`, so a test module is invisible
//! anyway). Every subsequent edit in this file is a MEASURED rewrite: see the git
//! log of this directory.

#![allow(dead_code)]

pub mod lattice {
    //! Generic lattice framework — the substrate of every Dorc dataflow analysis. A
    //! [`Lattice`] supplies ⊥ (`bottom`), ⊔ (`join`), AND ⊓ (`meet`). The two merge
    //! operators are what let *one* solver run both *may* analyses (over-approximate:
    //! start ⊥, merge ⊔) and *must* analyses (under-approximate: start ⊤, merge ⊓) —
    //! the orientation that, silently mis-chosen, is a wrong-skip (note 165). A *must*
    //! analysis additionally needs a representable ⊤ to seed its interior nodes, so it
    //! runs over a [`BoundedLattice`]; crucially **not every lattice has one** — a
    //! [`Powerset`]/[`MapL`] over an unbounded element/key type has a perfectly good ⊓
    //! (∩ / pointwise) but no finite ⊤ (the universal set is unrepresentable). The
    //! type system therefore forbids a must-analysis over a bare powerset — the
    //! asymmetry note 165 predicted, made a compile error rather than a runtime
    //! surprise. The solver climbs/descends a finite-height chain to the fixed point.
    //!
    //! Domains are built compositionally from the combinators below
    //! ([`Powerset`]/[`Flat`]/[`Product`]/[`MapL`]) rather than hand-rolled per
    //! analysis. All use *ordered* collections (`BTreeSet`/`BTreeMap`), never hashed,
    //! so any iteration over a lattice value is deterministic (`inv-determinism`).

    use std::collections::{BTreeMap, BTreeSet};

    /// A lattice of finite height: ⊥, ⊔ (`join`), and ⊓ (`meet`).
    ///
    /// Laws (not type-enforceable — property-tested in `tests` below): `join` and
    /// `meet` are each associative, commutative, and idempotent; they **absorb**
    /// (`a ⊔ (a ⊓ b) = a` and `a ⊓ (a ⊔ b) = a`); `bottom` is `join`'s identity and
    /// `meet`'s absorbing element (`⊥ ⊓ a = ⊥`); and the induced order
    /// `x ⊑ y ⟺ x ⊔ y = y` (equivalently `x ⊓ y = x`) has finite height. Transfer
    /// functions must additionally be **monotone** (`x ⊑ y ⇒ f(x) ⊑ f(y)`) or the
    /// fixed point is not guaranteed.
    pub trait Lattice: Clone + Eq {
        /// The least element ⊥ — the identity of [`join`](Lattice::join), and the
        /// absorbing element of [`meet`](Lattice::meet) (`⊥ ⊓ a = ⊥`).
        fn bottom() -> Self;

        /// The least upper bound `self ⊔ other` (the *may* merge: over-approximate).
        #[must_use]
        fn join(&self, other: &Self) -> Self;

        /// The greatest lower bound `self ⊓ other` (the *must* merge: under-approx).
        /// Dual to [`join`](Lattice::join) — see the absorption laws above.
        #[must_use]
        fn meet(&self, other: &Self) -> Self;

        /// `self ⊑ other` — "`other` is a safe over-approximation of `self`". Derived
        /// from `join`: `x ⊑ y ⟺ x ⊔ y = y`.
        #[must_use]
        fn leq(&self, other: &Self) -> bool {
            &self.join(other) == other
        }
    }

    /// A [`Lattice`] with a representable greatest element ⊤ — the identity of
    /// [`meet`](Lattice::meet) (`⊤ ⊓ a = a`) and the absorbing element of `join`
    /// (`⊤ ⊔ a = ⊤`). A *must* dataflow seeds its interior nodes at ⊤ and descends via
    /// ⊓, so it runs only over a `BoundedLattice`. [`Powerset`]/[`MapL`] over an
    /// unbounded element/key type deliberately do NOT implement it (no finite
    /// universal set), making "a must-analysis over a bare powerset" a compile error.
    pub trait BoundedLattice: Lattice {
        /// The greatest element ⊤.
        fn top() -> Self;
    }

    /// Powerset lattice `(P(T), ⊆)`: ⊥ = ∅, ⊔ = ∪, ⊓ = ∩. A full [`Lattice`] (it has
    /// a meet), but with **no representable ⊤** for an unbounded `T` (the universal
    /// set), so deliberately NOT a [`BoundedLattice`] — a *must* analysis needing a ⊤
    /// seed must use an explicit-top domain instead (note 165's predicted asymmetry).
    /// Typically a *may* domain (over-approximate, started at ⊥).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Powerset<T: Ord + Clone>(pub BTreeSet<T>);

    impl<T: Ord + Clone> Default for Powerset<T> {
        fn default() -> Self {
            Powerset(BTreeSet::new())
        }
    }

    impl<T: Ord + Clone> Powerset<T> {
        #[must_use]
        pub fn singleton(x: T) -> Self {
            let mut s = BTreeSet::new();
            s.insert(x);
            Powerset(s)
        }

        #[must_use]
        pub fn contains(&self, x: &T) -> bool {
            self.0.contains(x)
        }
    }

    impl<T: Ord + Clone> Lattice for Powerset<T> {
        fn bottom() -> Self {
            Powerset(BTreeSet::new())
        }
        fn join(&self, other: &Self) -> Self {
            Powerset(self.0.union(&other.0).cloned().collect())
        }
        fn meet(&self, other: &Self) -> Self {
            Powerset(self.0.intersection(&other.0).cloned().collect())
        }
    }

    /// Flat lattice `flat(T)`, height 2: ⊥ below a layer of mutually-incomparable
    /// elements below ⊤. Joining two *different* elements jumps to ⊤ ("don't know").
    /// The constant / single-known-value shape (Dorc's per-fact qualifier:
    /// absent vs present@v vs ⊤).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Flat<T: Clone + Eq> {
        Bottom,
        Elem(T),
        Top,
    }

    impl<T: Clone + Eq> Lattice for Flat<T> {
        fn bottom() -> Self {
            Flat::Bottom
        }
        fn join(&self, other: &Self) -> Self {
            match (self, other) {
                (Flat::Bottom, x) | (x, Flat::Bottom) => x.clone(),
                (Flat::Top, _) | (_, Flat::Top) => Flat::Top,
                (Flat::Elem(a), Flat::Elem(b)) => {
                    if a == b {
                        Flat::Elem(a.clone())
                    } else {
                        Flat::Top
                    }
                }
            }
        }
        fn meet(&self, other: &Self) -> Self {
            match (self, other) {
                (Flat::Top, x) | (x, Flat::Top) => x.clone(),
                (Flat::Bottom, _) | (_, Flat::Bottom) => Flat::Bottom,
                (Flat::Elem(a), Flat::Elem(b)) => {
                    if a == b {
                        Flat::Elem(a.clone())
                    } else {
                        Flat::Bottom
                    }
                }
            }
        }
    }

    impl<T: Clone + Eq> BoundedLattice for Flat<T> {
        fn top() -> Self {
            Flat::Top
        }
    }

    /// Product lattice `A × B`, ordered componentwise — for bundling independent
    /// facts (e.g. the several fields of the shell-environment state).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Product<A, B>(pub A, pub B);

    impl<A: Lattice, B: Lattice> Lattice for Product<A, B> {
        fn bottom() -> Self {
            Product(A::bottom(), B::bottom())
        }
        fn join(&self, other: &Self) -> Self {
            Product(self.0.join(&other.0), self.1.join(&other.1))
        }
        fn meet(&self, other: &Self) -> Self {
            Product(self.0.meet(&other.0), self.1.meet(&other.1))
        }
    }

    /// A product is bounded only when **both** components are — surfacing, in the
    /// type system, that `Product<Powerset<_>, _>` (Powerset has no ⊤) is a usable
    /// [`Lattice`] but not a must-domain.
    impl<A: BoundedLattice, B: BoundedLattice> BoundedLattice for Product<A, B> {
        fn top() -> Self {
            Product(A::top(), B::top())
        }
    }

    /// Map lattice `K → V`, ordered pointwise — the workhorse (Dorc's system-state
    /// fact store is a `MapL<Fact, Qualifier>`). Maintains a **canonical** form: no
    /// key maps to `V::bottom()` (absent ≡ ⊥). This makes structural `Eq` coincide
    /// with semantic equality, which the fixed-point loop relies on to detect
    /// convergence — so the field is private and only the methods below may mutate
    /// it.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MapL<K: Ord + Clone, V: Lattice>(BTreeMap<K, V>);

    impl<K: Ord + Clone, V: Lattice> Default for MapL<K, V> {
        fn default() -> Self {
            MapL(BTreeMap::new())
        }
    }

    impl<K: Ord + Clone, V: Lattice> MapL<K, V> {
        /// Value at `k`, or `V::bottom()` if absent (the semantic view).
        #[must_use]
        pub fn get(&self, k: &K) -> V {
            self.0.get(k).cloned().unwrap_or_else(V::bottom)
        }

        /// Set `k ↦ v`, preserving the no-⊥ canonical form.
        pub fn insert(&mut self, k: K, v: V) {
            if v == V::bottom() {
                self.0.remove(&k);
            } else {
                self.0.insert(k, v);
            }
        }

        /// Iterate the (canonical, non-⊥) bindings in deterministic key order.
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.0.iter()
        }
    }

    impl<K: Ord + Clone, V: Lattice> Lattice for MapL<K, V> {
        fn bottom() -> Self {
            MapL(BTreeMap::new())
        }
        fn join(&self, other: &Self) -> Self {
            let mut out = self.clone();
            for (k, v) in &other.0 {
                let joined = out.get(k).join(v);
                out.insert(k.clone(), joined);
            }
            out
        }
        fn meet(&self, other: &Self) -> Self {
            // Pointwise ⊓. A key absent in either map is ⊥ there (the no-⊥ canonical
            // form), and `⊥ ⊓ v = ⊥`, so only keys present in BOTH can survive — and
            // even then only if their value-meet is non-⊥ (`insert` drops ⊥, keeping
            // the form canonical so `Eq` stays semantic).
            let mut out = MapL::default();
            for (k, v) in &self.0 {
                if let Some(v2) = other.0.get(k) {
                    out.insert(k.clone(), v.meet(v2));
                }
            }
            out
        }
    }

    /// Orientation wrapper: an **over-approximate** (*may*) value — `truth ⊆ self`
    /// ("at most these"). The identity wrapper on `L` (⊥-start, ⊔-merge). A `May`
    /// result is safe for "this MIGHT hold / might need to run"; per `inv-must-may` it
    /// can NEVER license a skip — that authority is the dual's (note 165 L1).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct May<L>(pub L);

    impl<L: Lattice> Lattice for May<L> {
        fn bottom() -> Self {
            May(L::bottom())
        }
        fn join(&self, other: &Self) -> Self {
            May(self.0.join(&other.0))
        }
        fn meet(&self, other: &Self) -> Self {
            May(self.0.meet(&other.0))
        }
    }

    impl<L: BoundedLattice> BoundedLattice for May<L> {
        fn top() -> Self {
            May(L::top())
        }
    }

    /// Orientation wrapper: an **under-approximate** (*must*) value — `self ⊆ truth`
    /// ("at least these"). Implemented as the **order-dual** of `L`: its ⊥ is `L`'s ⊤
    /// and its ⊔ is `L`'s ⊓, so running the (always ⊥-start, ⊔-merge) [`solve`] over
    /// `Must<L>` performs a *must* analysis on `L` (⊤-start, ⊓-merge) — one engine,
    /// both orientations, the merge picked by the *type* (note 165 L1; this is what
    /// kills the union-where-you-needed-intersection bug). Only a `Must` value may
    /// license a skip.
    ///
    /// Requires `L: BoundedLattice` for the ⊤ that becomes the dual's ⊥ — which is
    /// precisely why a must-analysis over a bare [`Powerset`] does not type-check.
    ///
    /// *Boundary note:* a forward-must analysis whose entry in-state is **not** ⊤
    /// (e.g. available-expressions, entry = ∅) must seed that boundary explicitly;
    /// the default [`solve`] starts every node at the merge-identity (`Must`'s ⊥ =
    /// `L`'s ⊤). Add boundary seeding when the first such analysis lands (none yet —
    /// don't half-build it; cf. note 167 DP-8).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Must<L>(pub L);

    impl<L: BoundedLattice> Lattice for Must<L> {
        fn bottom() -> Self {
            Must(L::top()) // dual ⊥ = L's ⊤
        }
        fn join(&self, other: &Self) -> Self {
            Must(self.0.meet(&other.0)) // dual ⊔ = L's ⊓
        }
        fn meet(&self, other: &Self) -> Self {
            Must(self.0.join(&other.0)) // dual ⊓ = L's ⊔
        }
    }

    impl<L: BoundedLattice> BoundedLattice for Must<L> {
        fn top() -> Self {
            Must(L::bottom()) // dual ⊤ = L's ⊥
        }
    }

}

pub mod solve {
    //! The fixed-point solver — a propagation worklist generic over any [`Lattice`]
    //! and a [`Graph`], in either [`Direction`].
    //!
    //! Pure + deterministic (`inv-determinism`): FIFO worklist, graph-order
    //! neighbours, ordered lattice values ⇒ identical inputs converge to the
    //! identical per-node fixed point.
    //!
    //! Termination is guaranteed ONLY when the caller upholds the preconditions
    //! below; the type system cannot express them (see `Research/notes/165`), so the
    //! solver fails *loud, not silent*: a precondition violation trips a generous
    //! iteration cap and returns [`Solution::converged`]` == false` rather than
    //! hanging (this was an empirically-real infinite loop, not a theoretical one —
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

    /// Dataflow direction — the only axis distinguishing, e.g., reaching-definitions
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
    /// hit before a fixed point — which happens ONLY when a [`solve`] precondition
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
    /// **Preconditions the caller must uphold** (the type system cannot — note 165):
    /// 1. `transfer` is **monotone** (`x ⊑ y ⇒ f(x) ⊑ f(y)`);
    /// 2. the lattice `L` has **finite height** for the values this analysis can
    ///    actually produce (e.g. a `MapL`/`Powerset` whose keys/elements are drawn
    ///    from a *bounded* set — a transfer that mints a fresh key/element every
    ///    visit climbs forever);
    /// 3. `L`'s `Eq` is **semantic** (agrees with lattice equality);
    /// 4. every graph edge endpoint is `< node_count()`.
    ///
    /// Violating 1/2/3 is caught as `Solution::converged == false` (never a hang —
    /// the iteration cap). Violating 4 is a `debug_assert` (release: skipped edge).
    #[must_use]
    pub fn solve<G: Graph, L: Lattice>(
        graph: &G,
        direction: Direction,
        transfer: impl Fn(usize, &L) -> L,
    ) -> Solution<L> {
        let n = graph.node_count();
        // A node's output flows to its successors (forward) or predecessors
        // (backward) — its consumer set, where we propagate-and-join.
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
                    continue; // release-mode defensive skip — never panic (inv-no-throw)
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

}
