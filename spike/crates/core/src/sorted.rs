//! `Vec`-backed ordered collections — the algebra tier's stand-in for `BTreeSet`/`BTreeMap`.
//!
//! # Why
//!
//! The kernel's algebra is translated into Lean so its laws can be stated against the real
//! definitions rather than a hand-written paraphrase. The translator cannot see through
//! `BTreeMap`/`BTreeSet` (their `alloc` internals are `unsafe`); a plain `Vec` translates. So an
//! ordered collection reachable from the algebra tier is one of the two facades below.
//!
//! Two consequences shape everything here, and both are load-bearing:
//!
//! * **Canonical form is no longer type-carried.** `BTreeSet` made sortedness and dedup
//!   structural; here one seat upholds each ([`SortedSet::insert`] / [`SortedMap::insert`], both
//!   deciding through the sole scan, `position`). That canonical form is exactly what makes the
//!   derived `PartialEq` *semantic* equality — which the fixpoint solver reads to detect
//!   convergence — so an ordering or dedup bug here is a premature-fixpoint bug (a wrong elision),
//!   not a cosmetic one.
//! * **No slice-iterator internals on the algebra path.** `slice::Iter::next` is `unsafe`, so the
//!   walks below index through [`Vec::get`] instead of `for x in &v`. The shape is also total —
//!   no indexing panic (`inv-no-throw`).
//!
//! [`iter`](SortedSet::iter) is the ergonomic ordered exit for code *outside* the algebra tier;
//! [`get_at`](SortedSet::get_at) is the walk primitive inside it.
//!
//! Not to be confused with [`crate::unord`], the other owned-collection facade: that one hides a
//! `HashMap`'s iteration order from the receipts plane. This one is ordered *by construction*.

use std::cmp::Ordering;

/// Where a probe sits in a sorted, deduplicated backing.
enum Slot {
    /// Present, at this index.
    At(usize),
    /// Absent; inserting here keeps the backing sorted.
    Before(usize),
}

/// An ordered set backed by a sorted, deduplicated `Vec`.
///
/// Structural `PartialEq` is semantic set equality *because* the backing is canonical — see the
/// module docs for why that is the property the solver rests on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedSet<T> {
    items: Vec<T>,
}

impl<T> SortedSet<T> {
    /// The empty set. `const` so a `static` empty can back a borrowing accessor.
    #[must_use]
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// The number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The element at `index` in sort order — the algebra tier's walk primitive.
    #[must_use]
    pub fn get_at(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Iterate in sort order — the ergonomic exit, for consumers outside the algebra tier.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }
}

impl<T> Default for SortedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> IntoIterator for &'a SortedSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Ord> SortedSet<T> {
    /// The sole scan: every membership question and every mutation decides here, so sortedness
    /// and dedup have exactly one place to be got wrong.
    fn position(&self, probe: &T) -> Slot {
        let mut index = 0usize;
        while let Some(item) = self.items.get(index) {
            match probe.cmp(item) {
                Ordering::Less => return Slot::Before(index),
                Ordering::Equal => return Slot::At(index),
                Ordering::Greater => index = index.saturating_add(1),
            }
        }
        Slot::Before(self.items.len())
    }

    /// The one-element set.
    #[must_use]
    pub fn singleton(value: T) -> Self {
        Self { items: vec![value] }
    }

    /// Insert `value`; `false` if it was already present. **The canonical-form seat** — the only
    /// growth path, and so the only thing keeping `PartialEq` semantic.
    pub fn insert(&mut self, value: T) -> bool {
        match self.position(&value) {
            Slot::At(_) => false,
            Slot::Before(at) => {
                self.items.insert(at, value);
                true
            }
        }
    }

    /// Remove `value`; `false` if it was absent.
    pub fn remove(&mut self, value: &T) -> bool {
        match self.position(value) {
            Slot::At(at) => {
                self.items.remove(at);
                true
            }
            Slot::Before(_) => false,
        }
    }

    /// Whether `value` is a member.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        matches!(self.position(value), Slot::At(_))
    }
}

impl<T: Ord + Clone> SortedSet<T> {
    /// `self ∪ other`.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut out = self.clone();
        let mut index = 0usize;
        while let Some(item) = other.items.get(index) {
            out.insert(item.clone());
            index = index.saturating_add(1);
        }
        out
    }

    /// `self ∩ other`. Routes every element through [`insert`](Self::insert) rather than pushing
    /// the (already ordered) survivors, so the canonical form has one keeper, not two.
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut out = Self::new();
        let mut index = 0usize;
        while let Some(item) = self.items.get(index) {
            if other.contains(item) {
                out.insert(item.clone());
            }
            index = index.saturating_add(1);
        }
        out
    }
}

impl<T: Ord> FromIterator<T> for SortedSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut out = Self::new();
        for value in iter {
            out.insert(value);
        }
        out
    }
}

/// An ordered map backed by a key-sorted, key-unique `Vec` of pairs. The `SortedSet` reasoning
/// applies verbatim, one axis over: canonical form is what makes `PartialEq` semantic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> SortedMap<K, V> {
    /// The empty map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// The number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The binding at `index` in key order — the algebra tier's walk primitive.
    #[must_use]
    pub fn get_at(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get(index).map(|(k, v)| (k, v))
    }

    /// Iterate bindings in key order — the ergonomic exit.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
}

impl<K, V> Default for SortedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> SortedMap<K, V> {
    /// The sole scan — see [`SortedSet::position`].
    fn position(&self, probe: &K) -> Slot {
        let mut index = 0usize;
        while let Some((key, _)) = self.entries.get(index) {
            match probe.cmp(key) {
                Ordering::Less => return Slot::Before(index),
                Ordering::Equal => return Slot::At(index),
                Ordering::Greater => index = index.saturating_add(1),
            }
        }
        Slot::Before(self.entries.len())
    }

    /// The value bound to `key`, if any.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.position(key) {
            Slot::At(at) => self.entries.get(at).map(|(_, v)| v),
            Slot::Before(_) => None,
        }
    }

    /// Mutable access to the value bound to `key`, if any.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.position(key) {
            Slot::At(at) => self.entries.get_mut(at).map(|(_, v)| v),
            Slot::Before(_) => None,
        }
    }

    /// Bind `key ↦ value`, returning any prior value. **The canonical-form seat.**
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.position(&key) {
            Slot::At(at) => self
                .entries
                .get_mut(at)
                .map(|entry| std::mem::replace(&mut entry.1, value)),
            Slot::Before(at) => {
                self.entries.insert(at, (key, value));
                None
            }
        }
    }

    /// Unbind `key`, returning its value if it was bound.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.position(key) {
            Slot::At(at) => Some(self.entries.remove(at).1),
            Slot::Before(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn elements(set: &SortedSet<u8>) -> Vec<u8> {
        set.iter().copied().collect()
    }

    fn keys(map: &SortedMap<u8, &'static str>) -> Vec<u8> {
        map.iter().map(|(k, _)| *k).collect()
    }

    #[test]
    fn set_insert_sorts_and_dedups() {
        // The seat's whole job. Scrambled input, one duplicate, one already-present re-insert.
        let mut s = SortedSet::new();
        for x in [5u8, 1, 9, 1, 3] {
            s.insert(x);
        }
        assert_eq!(elements(&s), vec![1, 3, 5, 9], "ascending, deduplicated");
        assert!(!s.insert(3), "re-inserting a member reports no growth");
        assert_eq!(s.len(), 4, "…and does not grow it");
        assert!(s.insert(0), "a new minimum reports growth");
        assert_eq!(elements(&s), vec![0, 1, 3, 5, 9], "…inserted in order");
    }

    #[test]
    fn set_structural_eq_is_semantic_eq() {
        // THE load-bearing property: the fixpoint solver detects convergence with `!=`, so two
        // sets of the same members built in different orders MUST compare equal, and sets
        // differing by a single member MUST NOT. A dedup or ordering bug shows up here first.
        let ascending: SortedSet<u8> = [1u8, 2, 3, 4].into_iter().collect();
        let descending: SortedSet<u8> = [4u8, 3, 2, 1].into_iter().collect();
        let with_repeats: SortedSet<u8> = [3u8, 1, 4, 1, 2, 3].into_iter().collect();
        assert_eq!(ascending, descending, "insertion order is not observable");
        assert_eq!(ascending, with_repeats, "repeats are not observable");
        let extra: SortedSet<u8> = [1u8, 2, 3, 4, 5].into_iter().collect();
        assert_ne!(ascending, extra, "a differing member IS observable");
    }

    #[test]
    fn set_contains_and_remove_agree_with_membership() {
        // `contains` is the survival chokepoint's primitive (`selector_covers`): a false negative
        // there turns a collide into a spare, which under-executes.
        let mut s: SortedSet<u8> = [2u8, 4, 6].into_iter().collect();
        for present in [2u8, 4, 6] {
            assert!(s.contains(&present));
        }
        for absent in [1u8, 3, 5, 7, 0] {
            assert!(!s.contains(&absent), "absent below, between, and above");
        }
        assert!(s.remove(&4));
        assert!(!s.contains(&4));
        assert!(!s.remove(&4), "removing an absent member reports nothing");
        assert_eq!(elements(&s), vec![2, 6], "order survives removal");
    }

    #[test]
    fn set_union_and_intersection_stay_canonical() {
        let a: SortedSet<u8> = [1u8, 3, 5].into_iter().collect();
        let b: SortedSet<u8> = [5u8, 3, 9].into_iter().collect();
        assert_eq!(elements(&a.union(&b)), vec![1, 3, 5, 9], "∪, still ordered");
        assert_eq!(
            elements(&a.intersection(&b)),
            vec![3, 5],
            "∩, still ordered"
        );
        assert_eq!(a.union(&b), b.union(&a), "∪ commutes");
        assert_eq!(a.intersection(&b), b.intersection(&a), "∩ commutes");
        let empty = SortedSet::new();
        assert_eq!(a.union(&empty), a, "∅ is ∪'s identity");
        assert_eq!(a.intersection(&empty), empty, "∅ absorbs under ∩");
    }

    #[test]
    fn map_insert_sorts_keys_and_replaces_values() {
        let mut m = SortedMap::new();
        for (k, v) in [(5u8, "e"), (1, "a"), (9, "i")] {
            assert_eq!(m.insert(k, v), None, "a fresh key has no prior value");
        }
        assert_eq!(keys(&m), vec![1, 5, 9], "ascending by key");
        assert_eq!(m.insert(5, "E"), Some("e"), "a bound key returns its prior");
        assert_eq!(m.len(), 3, "…and does not grow the map");
        assert_eq!(m.get(&5), Some(&"E"));
        assert_eq!(m.get(&7), None, "absent between existing keys");
    }

    #[test]
    fn map_structural_eq_is_semantic_eq() {
        let mut ascending = SortedMap::new();
        let mut descending = SortedMap::new();
        for (k, v) in [(1u8, "a"), (2, "b"), (3, "c")] {
            ascending.insert(k, v);
        }
        for (k, v) in [(3u8, "c"), (2, "b"), (1, "a")] {
            descending.insert(k, v);
        }
        assert_eq!(ascending, descending, "insertion order is not observable");
        descending.insert(2, "B");
        assert_ne!(ascending, descending, "a differing value IS observable");
    }

    #[test]
    fn map_remove_and_get_at_keep_key_order() {
        let mut m = SortedMap::new();
        for (k, v) in [(3u8, "c"), (1, "a"), (2, "b")] {
            m.insert(k, v);
        }
        assert_eq!(m.get_at(0), Some((&1u8, &"a")), "get_at walks in key order");
        assert_eq!(m.get_at(2), Some((&3u8, &"c")));
        assert_eq!(m.get_at(3), None, "past the end");
        assert_eq!(m.remove(&1), Some("a"));
        assert_eq!(m.remove(&1), None, "removing an unbound key returns None");
        assert_eq!(keys(&m), vec![2, 3], "order survives removal");
    }
}
