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
//! analysis. The two collection-shaped ones sit on [`SortedSet`]/[`SortedMap`],
//! never on hashed or `BTree*` storage: iteration over a lattice value is
//! deterministic (`inv-determinism`), and the `Vec` backing is what lets this
//! tier translate for the Lean law statements (see `dorc_core::sorted`).

use dorc_core::sorted::{SortedMap, SortedSet};

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
///
/// The backing is private: the sorted-dedup canonical form is what makes `Eq`
/// semantic, and [`SortedSet::insert`] is its one seat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Powerset<T: Ord + Clone>(SortedSet<T>);

impl<T: Ord + Clone> Default for Powerset<T> {
    fn default() -> Self {
        Powerset(SortedSet::new())
    }
}

impl<T: Ord + Clone> Powerset<T> {
    #[must_use]
    pub fn singleton(x: T) -> Self {
        Powerset(SortedSet::singleton(x))
    }

    #[must_use]
    pub fn contains(&self, x: &T) -> bool {
        self.0.contains(x)
    }

    /// Add `x`; `false` if it was already a member.
    pub fn insert(&mut self, x: T) -> bool {
        self.0.insert(x)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate the members in element order (`inv-determinism`).
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T: Ord + Clone> FromIterator<T> for Powerset<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Powerset(iter.into_iter().collect())
    }
}

impl<T: Ord + Clone> Lattice for Powerset<T> {
    fn bottom() -> Self {
        Powerset(SortedSet::new())
    }
    fn join(&self, other: &Self) -> Self {
        Powerset(self.0.union(&other.0))
    }
    fn meet(&self, other: &Self) -> Self {
        Powerset(self.0.intersection(&other.0))
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
pub struct MapL<K: Ord + Clone, V: Lattice>(SortedMap<K, V>);

impl<K: Ord + Clone, V: Lattice> Default for MapL<K, V> {
    fn default() -> Self {
        MapL(SortedMap::new())
    }
}

impl<K: Ord + Clone, V: Lattice> MapL<K, V> {
    /// Value at `k`, or `V::bottom()` if absent (the semantic view).
    #[must_use]
    pub fn get(&self, k: &K) -> V {
        match self.0.get(k) {
            Some(v) => v.clone(),
            None => V::bottom(),
        }
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
        MapL(SortedMap::new())
    }
    fn join(&self, other: &Self) -> Self {
        let mut out = self.clone();
        let mut index = 0usize;
        while let Some((k, v)) = other.0.get_at(index) {
            let joined = out.get(k).join(v);
            out.insert(k.clone(), joined);
            index = index.saturating_add(1);
        }
        out
    }
    fn meet(&self, other: &Self) -> Self {
        // Pointwise ⊓. A key absent in either map is ⊥ there (the no-⊥ canonical
        // form), and `⊥ ⊓ v = ⊥`, so only keys present in BOTH can survive — and
        // even then only if their value-meet is non-⊥ (`insert` drops ⊥, keeping
        // the form canonical so `Eq` stays semantic).
        let mut out = MapL::default();
        let mut index = 0usize;
        while let Some((k, v)) = self.0.get_at(index) {
            if let Some(v2) = other.0.get(k) {
                out.insert(k.clone(), v.meet(v2));
            }
            index = index.saturating_add(1);
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

#[cfg(test)]
mod tests {
    use super::*;

    // Generic lattice-law checks, run against a few sample elements of each
    // combinator. (Hand-written rather than a proptest dependency — `inv-no-dep`
    // on the deterministic core; the samples are chosen to exercise ⊥, joins
    // that climb, and joins that saturate to ⊤.)
    fn assert_laws<L: Lattice + std::fmt::Debug>(samples: &[L]) {
        let bot = L::bottom();
        for a in samples {
            // ⊥ identities, idempotence, reflexivity.
            assert_eq!(bot.join(a), *a, "⊥ ⊔ a = a");
            assert_eq!(a.join(&bot), *a, "a ⊔ ⊥ = a");
            assert_eq!(a.join(a), *a, "⊔ idempotent");
            assert_eq!(a.meet(a), *a, "⊓ idempotent");
            assert_eq!(bot.meet(a), bot, "⊥ ⊓ a = ⊥ (⊥ absorbs under ⊓)");
            assert!(a.leq(a), "reflexive ⊑");
            assert!(bot.leq(a), "⊥ ⊑ a");
            for b in samples {
                assert_eq!(a.join(b), b.join(a), "⊔ commutative");
                assert_eq!(a.meet(b), b.meet(a), "⊓ commutative");
                // absorption — the join/meet duality (a wrong `meet` breaks these).
                assert_eq!(a.join(&a.meet(b)), *a, "a ⊔ (a ⊓ b) = a");
                assert_eq!(a.meet(&a.join(b)), *a, "a ⊓ (a ⊔ b) = a");
                let lub = a.join(b);
                let glb = a.meet(b);
                assert!(a.leq(&lub) && b.leq(&lub), "a,b ⊑ a⊔b");
                assert!(glb.leq(a) && glb.leq(b), "a⊓b ⊑ a,b");
                // the two ⊑ characterisations agree.
                assert_eq!(a.leq(b), a.meet(b) == *a, "x⊑y ⟺ x⊓y=x");
                for c in samples {
                    assert_eq!(a.join(&b.join(c)), a.join(b).join(c), "⊔ associative");
                    assert_eq!(a.meet(&b.meet(c)), a.meet(b).meet(c), "⊓ associative");
                }
            }
        }
    }

    /// The ⊤ laws, for the lattices that have a representable greatest element.
    fn assert_bounded<L: BoundedLattice + std::fmt::Debug>(samples: &[L]) {
        let top = L::top();
        for a in samples {
            assert_eq!(top.meet(a), *a, "⊤ ⊓ a = a");
            assert_eq!(a.meet(&top), *a, "a ⊓ ⊤ = a");
            assert_eq!(top.join(a), top, "⊤ ⊔ a = ⊤ (⊤ absorbs under ⊔)");
            assert!(a.leq(&top), "a ⊑ ⊤");
        }
    }

    #[test]
    fn powerset_is_a_lattice() {
        let p = |xs: &[u8]| xs.iter().copied().collect::<Powerset<u8>>();
        assert_laws(&[p(&[]), p(&[1]), p(&[2]), p(&[1, 2]), p(&[1, 2, 3])]);
        assert_eq!(p(&[1]).join(&p(&[2])), p(&[1, 2]));
        assert_eq!(p(&[1, 2]).meet(&p(&[2, 3])), p(&[2]), "⊓ = ∩");
        assert!(p(&[1]).leq(&p(&[1, 2])));
        assert!(!p(&[1, 2]).leq(&p(&[1])));
    }

    #[test]
    fn collection_domains_are_insertion_order_independent() {
        // Since the backings became sorted `Vec`s, "structural Eq = semantic Eq" is upheld by
        // `SortedSet`/`SortedMap`'s canonical form rather than by the collection type. `solve`
        // detects its fixed point with `!=`, so a wrapper that let insertion order leak would
        // stop the climb at the wrong state — a wrong elision, not a cosmetic diff.
        type M = MapL<&'static str, Flat<u8>>;

        let ascending: Powerset<u8> = [1u8, 2, 3].into_iter().collect();
        let descending: Powerset<u8> = [3u8, 2, 1].into_iter().collect();
        assert_eq!(ascending, descending);

        let mut forward = M::default();
        let mut backward = M::default();
        for (k, v) in [("a", 1u8), ("b", 2), ("c", 3)] {
            forward.insert(k, Flat::Elem(v));
        }
        for (k, v) in [("c", 3u8), ("b", 2), ("a", 1)] {
            backward.insert(k, Flat::Elem(v));
        }
        assert_eq!(forward, backward);
    }

    #[test]
    fn flat_saturates_distinct_elems_to_top() {
        use Flat::{Bottom, Elem, Top};
        assert_laws(&[Bottom, Elem(1u8), Elem(2u8), Top]);
        assert_bounded(&[Bottom, Elem(1u8), Elem(2u8), Top]);
        assert_eq!(Elem(1u8).join(&Elem(1)), Elem(1), "same elem stays");
        assert_eq!(Elem(1u8).join(&Elem(2)), Top, "distinct elems → ⊤");
        assert_eq!(Elem(1u8).meet(&Elem(2)), Bottom, "distinct elems ⊓ → ⊥");
        assert_eq!(Top.meet(&Elem(1u8)), Elem(1), "⊤ ⊓ a = a");
        assert!(Elem(1u8).leq(&Top) && !Top.leq(&Elem(1)));
    }

    #[test]
    fn product_is_componentwise() {
        type P = Product<Powerset<u8>, Flat<u8>>;
        type FF = Product<Flat<u8>, Flat<u8>>;
        let mk = |s: &[u8], f: Flat<u8>| Product(s.iter().copied().collect::<Powerset<u8>>(), f);
        assert_laws::<P>(&[
            P::bottom(),
            mk(&[1], Flat::Elem(9)),
            mk(&[2], Flat::Elem(8)),
            mk(&[1, 2], Flat::Top),
        ]);
        // join distinct flat components → ⊤ in that component only.
        let j = mk(&[1], Flat::Elem(9)).join(&mk(&[2], Flat::Elem(8)));
        assert_eq!(j.1, Flat::Top);
        assert_eq!(j.0, [1u8, 2].into_iter().collect::<Powerset<u8>>());

        // A product is BoundedLattice only when BOTH components are: Flat×Flat is,
        // but the Powerset×Flat above is NOT (Powerset has no ⊤) — the type-level
        // asymmetry, exercised. ⊓ is componentwise.
        assert_bounded::<FF>(&[FF::bottom(), Product(Flat::Elem(1), Flat::Top), FF::top()]);
        assert_eq!(
            Product(Flat::Elem(1u8), Flat::Top).meet(&Product(Flat::Top, Flat::Elem(2u8))),
            Product(Flat::Elem(1), Flat::Elem(2)),
            "componentwise ⊓"
        );
    }

    #[test]
    fn maplattice_is_pointwise_and_canonical() {
        type M = MapL<&'static str, Powerset<u8>>;
        let mut lhs = M::default();
        lhs.insert("pkg", Powerset::singleton(1));
        let mut rhs = M::default();
        rhs.insert("pkg", Powerset::singleton(2));
        rhs.insert("svc", Powerset::singleton(7));

        let joined = lhs.join(&rhs);
        assert_eq!(
            joined.get(&"pkg"),
            [1u8, 2].into_iter().collect::<Powerset<u8>>(),
            "pointwise join"
        );
        assert_eq!(
            joined.get(&"svc"),
            Powerset::singleton(7),
            "key only in rhs"
        );
        assert_eq!(joined.get(&"absent"), Powerset::bottom(), "absent ≡ ⊥");

        // Canonical form: inserting ⊥ removes the key (so Eq is semantic).
        let mut c = M::default();
        c.insert("x", Powerset::singleton(1));
        c.insert("x", Powerset::bottom());
        assert_eq!(
            c,
            M::default(),
            "⊥-valued key is dropped → equals empty map"
        );

        // meet: pointwise ∩; only keys present in BOTH and non-⊥ survive.
        let mut d = M::default();
        d.insert("pkg", [1u8, 2].into_iter().collect());
        let mut e = M::default();
        e.insert("pkg", [2u8, 3].into_iter().collect());
        e.insert("svc", Powerset::singleton(7));
        let m = d.meet(&e);
        assert_eq!(
            m.get(&"pkg"),
            Powerset::singleton(2),
            "pkg intersection is the singleton 2"
        );
        assert_eq!(
            m.get(&"svc"),
            Powerset::bottom(),
            "svc only in e, dropped by meet"
        );

        assert_laws::<M>(&[M::default(), lhs, rhs, joined]);
    }

    #[test]
    fn orientation_wrappers_are_dual_lattices() {
        use Flat::{Bottom, Elem, Top};
        // May<L> is the identity wrapper — same ⊥/⊔/⊓ as L.
        assert_laws(&[May(Bottom), May(Elem(1u8)), May(Elem(2u8)), May(Top)]);
        assert_bounded(&[May(Bottom), May(Elem(1u8)), May(Elem(2u8)), May(Top)]);
        assert_eq!(
            May(Elem(1u8)).join(&May(Elem(2))),
            May(Top),
            "May ⊔ = L's ⊔"
        );

        // Must<L> is the order-dual — still a lawful (bounded) lattice, with ⊥/⊔
        // and ⊓ swapped. assert_laws passing IS the proof the dual is correct.
        assert_laws(&[Must(Bottom), Must(Elem(1u8)), Must(Elem(2u8)), Must(Top)]);
        assert_bounded(&[Must(Bottom), Must(Elem(1u8)), Must(Elem(2u8)), Must(Top)]);
        assert_eq!(Must::<Flat<u8>>::bottom(), Must(Top), "Must's ⊥ = L's ⊤");
        assert_eq!(Must::<Flat<u8>>::top(), Must(Bottom), "Must's ⊤ = L's ⊥");
        assert_eq!(
            Must(Elem(1u8)).join(&Must(Elem(2))),
            Must(Bottom),
            "Must's ⊔ = L's ⊓ (distinct elems meet to ⊥)"
        );
    }
}
