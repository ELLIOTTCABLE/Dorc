//! [`IterSuppressedMap`] — a map whose iteration API is **removed**, so an ordering leak is a
//! compile error rather than a review catch or a gate catch (`Research/plans/22A` concl-4;
//! `notes/229` finding-5 / `mechanism-unord-newtype`, modelled on rustc's `UnordMap`).
//!
//! # Why
//!
//! Ordering nondeterminism is the central receipts-into-decision leak class (`22A` concl-4):
//! a `HashMap` iterated into a decision output leaks its (seed-dependent) order; even a
//! `BTreeMap` keyed on receipt data would leak the receipt order. The strong projects enforce
//! determinism *structurally* — rustc's `UnordMap` removes the iteration API so the leak won't
//! compile, and ships `potential_query_instability`/`untracked_query_information` lints.
//!
//! Ours is the boring pure-Rust half: a thin wrapper exposing only `get`/`insert`/`contains`
//! and an explicit, deterministic `into_sorted_vec` (you must hand it a key-ordering, making
//! "produce output in some order" a deliberate, sorted act). There is **no** `iter`, no
//! `values`, no `IntoIterator` — so "iterate this map into a decision" does not type-check.
//!
//! # Scope (judgment, per the arch-1 contract)
//!
//! Wrap what the receipts plane touches; do not crusade through the whole codebase. Today the
//! one plane-owned collection is [`crate::prov::ProvArena`]'s hash-cons index, which needs
//! only `get`/`insert` and must never leak its order — it uses this newtype. Any future
//! *decision-internal* map that a receipt key/value flows through must use it too (the
//! `untracked_query_information` analogue); a `BTreeMap` keyed on `ProvId` is already
//! impossible (`ProvId` is deliberately not `Ord`).

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

/// A `HashMap` with NO iteration API — point lookup and insertion only, plus an explicit
/// `into_sorted_vec` that forces a deterministic ordering at the one place output is produced.
///
/// The backing store is a `HashMap` (random-seeded, so any accidental order-dependence breaks
/// immediately under test — the LLVM `-reverse-iterate` spirit), but no method exposes that
/// order: the type makes an iteration-order leak a *compile* error. `K: Ord` is required only
/// by [`into_sorted_vec`](Self::into_sorted_vec), the sanctioned ordered exit.
#[derive(Debug, Clone)]
pub struct IterSuppressedMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> Default for IterSuppressedMap<K, V> {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V> IterSuppressedMap<K, V> {
    /// An empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert `value` for `key`, returning any prior value (as `HashMap::insert`).
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// The value for `key`, or `None`. Point lookup leaks no order.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(key)
    }

    /// Whether `key` is present. Point lookup, no order leaked.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.contains_key(key)
    }

    /// The number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether the map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Consume the map into a `Vec<(K, V)>` sorted by key — the SOLE ordered exit, and
    /// deliberately explicit: producing output from this map is a sorted act, never an
    /// incidental hash-order iteration. Requires `K: Ord` so the order is the keys' total
    /// order, deterministic by construction (`inv-determinism`).
    #[must_use]
    pub fn into_sorted_vec(self) -> Vec<(K, V)>
    where
        K: Ord,
    {
        let mut v: Vec<(K, V)> = self.inner.into_iter().collect();
        v.sort_by(|(a, _), (b, _)| a.cmp(b));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_insert_contains_roundtrip() {
        let mut m: IterSuppressedMap<String, u32> = IterSuppressedMap::new();
        assert!(m.is_empty());
        assert_eq!(m.insert("a".to_owned(), 1), None);
        assert_eq!(m.insert("a".to_owned(), 2), Some(1), "insert returns prior");
        assert_eq!(m.get("a"), Some(&2));
        assert!(m.contains_key("a"));
        assert!(!m.contains_key("b"));
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn into_sorted_vec_is_key_ordered_deterministic() {
        // The sole ordered exit is sorted by key — so output is deterministic regardless of
        // insertion order or the HashMap's seed (the whole point: no hash-order leak).
        let mut a: IterSuppressedMap<u32, &str> = IterSuppressedMap::new();
        for (k, v) in [(3, "c"), (1, "a"), (2, "b")] {
            a.insert(k, v);
        }
        let mut b: IterSuppressedMap<u32, &str> = IterSuppressedMap::new();
        for (k, v) in [(2, "b"), (3, "c"), (1, "a")] {
            b.insert(k, v);
        }
        assert_eq!(
            a.into_sorted_vec(),
            vec![(1, "a"), (2, "b"), (3, "c")],
            "sorted by key, insertion-order-independent"
        );
        assert_eq!(
            b.into_sorted_vec(),
            vec![(1, "a"), (2, "b"), (3, "c")],
            "a different insertion order yields the identical sorted output"
        );
    }

    // NB there is intentionally NO test of `.iter()`/`.values()`/`IntoIterator` — those
    // methods do not exist, which is the entire contract (an ordering leak won't compile).
}
