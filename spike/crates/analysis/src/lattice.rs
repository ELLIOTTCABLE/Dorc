//! Generic complete-lattice framework — the substrate of every Dorc dataflow
//! analysis. A lattice supplies ⊥ (`bottom`) and ⊔ (`join`); the solver climbs
//! the chain `⊥ ⊑ f(⊥) ⊑ f²(⊥) ⊑ …` to the least fixed point, which terminates
//! because every lattice here has **finite height**.
//!
//! Domains are built compositionally from the combinators below
//! ([`Powerset`]/[`Flat`]/[`Product`]/[`MapL`]) rather than hand-rolled per
//! analysis. All use *ordered* collections (`BTreeSet`/`BTreeMap`), never hashed,
//! so any iteration over a lattice value is deterministic (`inv-determinism`).

use std::collections::{BTreeMap, BTreeSet};

/// A complete lattice of finite height.
///
/// Laws (not type-enforceable — property-tested in `tests` below): `join` is
/// associative, commutative, and idempotent; `bottom` is its identity; and the
/// induced order `x ⊑ y ⟺ x ⊔ y = y` has finite height. Transfer functions
/// written against a lattice must additionally be **monotone**
/// (`x ⊑ y ⇒ f(x) ⊑ f(y)`) or the least fixed point is not guaranteed.
pub trait Lattice: Clone + Eq {
    /// The least element ⊥ — the identity of [`join`](Lattice::join).
    fn bottom() -> Self;

    /// The least upper bound `self ⊔ other`.
    #[must_use]
    fn join(&self, other: &Self) -> Self;

    /// `self ⊑ other` — "`other` is a safe approximation of `self`". Derived
    /// from `join`: `x ⊑ y ⟺ x ⊔ y = y`.
    #[must_use]
    fn leq(&self, other: &Self) -> bool {
        &self.join(other) == other
    }
}

/// Powerset lattice `(P(T), ⊆)`: ⊥ = ∅, ⊔ = ∪ — a *may* domain (over-approx).
/// (A *must* analysis collects with ∩ from a ⊤ seed; that is an analysis-level
/// choice, not a separate type — the spike's analyses are all *may*.)
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
            // ⊥ is the identity; join is idempotent; a ⊑ a.
            assert_eq!(bot.join(a), *a, "⊥ ⊔ a = a");
            assert_eq!(a.join(&bot), *a, "a ⊔ ⊥ = a");
            assert_eq!(a.join(a), *a, "idempotent");
            assert!(a.leq(a), "reflexive ⊑");
            assert!(bot.leq(a), "⊥ ⊑ a");
            for b in samples {
                // commutative; a ⊑ a⊔b ⊒ b.
                assert_eq!(a.join(b), b.join(a), "commutative");
                let lub = a.join(b);
                assert!(a.leq(&lub) && b.leq(&lub), "a,b ⊑ a⊔b");
                for c in samples {
                    assert_eq!(a.join(&b.join(c)), a.join(b).join(c), "associative");
                }
            }
        }
    }

    #[test]
    fn powerset_is_a_lattice() {
        let p = |xs: &[u8]| Powerset(xs.iter().copied().collect::<BTreeSet<_>>());
        assert_laws(&[p(&[]), p(&[1]), p(&[2]), p(&[1, 2]), p(&[1, 2, 3])]);
        assert_eq!(p(&[1]).join(&p(&[2])), p(&[1, 2]));
        assert!(p(&[1]).leq(&p(&[1, 2])));
        assert!(!p(&[1, 2]).leq(&p(&[1])));
    }

    #[test]
    fn flat_saturates_distinct_elems_to_top() {
        use Flat::{Bottom, Elem, Top};
        assert_laws(&[Bottom, Elem(1u8), Elem(2u8), Top]);
        assert_eq!(Elem(1u8).join(&Elem(1)), Elem(1), "same elem stays");
        assert_eq!(Elem(1u8).join(&Elem(2)), Top, "distinct elems → ⊤");
        assert!(Elem(1u8).leq(&Top) && !Top.leq(&Elem(1)));
    }

    #[test]
    fn product_is_componentwise() {
        type P = Product<Powerset<u8>, Flat<u8>>;
        let mk = |s: &[u8], f: Flat<u8>| Product(Powerset(s.iter().copied().collect()), f);
        assert_laws::<P>(&[
            P::bottom(),
            mk(&[1], Flat::Elem(9)),
            mk(&[2], Flat::Elem(8)),
            mk(&[1, 2], Flat::Top),
        ]);
        // join distinct flat components → ⊤ in that component only.
        let j = mk(&[1], Flat::Elem(9)).join(&mk(&[2], Flat::Elem(8)));
        assert_eq!(j.1, Flat::Top);
        assert_eq!(j.0, Powerset([1, 2].into_iter().collect()));
    }

    #[test]
    fn maplattice_is_pointwise_and_canonical() {
        type M = MapL<&'static str, Powerset<u8>>;
        let mut a = M::default();
        a.insert("pkg", Powerset::singleton(1));
        let mut b = M::default();
        b.insert("pkg", Powerset::singleton(2));
        b.insert("svc", Powerset::singleton(7));

        let j = a.join(&b);
        assert_eq!(j.get(&"pkg"), Powerset([1, 2].into_iter().collect()), "pointwise join");
        assert_eq!(j.get(&"svc"), Powerset::singleton(7), "key only in b");
        assert_eq!(j.get(&"absent"), Powerset::bottom(), "absent ≡ ⊥");

        // Canonical form: inserting ⊥ removes the key (so Eq is semantic).
        let mut c = M::default();
        c.insert("x", Powerset::singleton(1));
        c.insert("x", Powerset::bottom());
        assert_eq!(c, M::default(), "⊥-valued key is dropped → equals empty map");

        assert_laws::<M>(&[M::default(), a, b, j]);
    }
}
