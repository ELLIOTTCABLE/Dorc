//! Facade canonicality — `core::sorted::{SortedSet, SortedMap}`.
//!
//! Seat: `300` §2a's invariant-seat bank (`SortedSet::insert` is THE canonical-form seat).
//! Law: the backing is strictly ascending, so the derived `PartialEq` is semantic equality —
//! which is what `solve`'s `joined != state[w]` convergence test rests on.
//!
//! Every generator draws an arbitrary `Vec` and assumes canonical (`300` §2a's Arbitrary law):
//! generating by repeated `insert` would make the `insert` harnesses assume what they prove.

use dorc_core::sorted::{SortedMap, SortedSet};

/// Membership by naive walk — the independent second opinion `position`'s scan is checked
/// against. Written as a linear walk on purpose: the point is that it shares no code with the
/// thing it judges.
fn member_by_walk(set: &SortedSet<u8>, probe: u8) -> bool {
    let mut i = 0usize;
    while let Some(x) = set.get_at(i) {
        if *x == probe {
            return true;
        }
        i = i.saturating_add(1);
    }
    false
}

/// Does some element witness a membership difference? Under canonical form two sets with the
/// same members are the same vector, so a structural difference must be observable as one.
fn membership_differs(a: &SortedSet<u8>, b: &SortedSet<u8>) -> bool {
    let mut i = 0usize;
    while let Some(x) = a.get_at(i) {
        if !b.contains(x) {
            return true;
        }
        i = i.saturating_add(1);
    }
    let mut j = 0usize;
    while let Some(y) = b.get_at(j) {
        if !a.contains(y) {
            return true;
        }
        j = j.saturating_add(1);
    }
    false
}

/// `insert` preserves strict ascent, and reports growth exactly when the value was absent.
/// Bounds: `SortedSet<u8>`, at most 3 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_preserves_canonical_form() {
    let mut set = SortedSet::<u8>::any_canonical::<3>();
    let value: u8 = kani::any();
    let was_present = set.contains(&value);
    let before = set.len();

    let grew = set.insert(value);

    assert!(set.is_strictly_ascending(), "the canonical-form seat");
    assert_eq!(grew, !was_present, "growth is reported iff it happened");
    assert!(set.contains(&value), "the inserted value is a member");
    assert_eq!(
        set.len(),
        if grew { before + 1 } else { before },
        "a set grows by one member or not at all"
    );
}

/// The same law on the value a growing `insert` has to move: a FULL backing, so the mutation
/// reallocates. Bounds: `SortedSet<u8>` of exactly 2 members, at capacity.
///
/// It is a separate harness because a symbolic length and a reallocating write together are
/// what a bounded model checker cannot afford here (`core::sorted`'s generator docs carry the
/// measurement). The law is the same one; only the shape of the value differs.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_preserves_canonical_form_when_the_backing_moves() {
    let mut set = SortedSet::<u8>::any_canonical_at_capacity::<2>();
    let value: u8 = kani::any();
    let was_present = set.contains(&value);

    let grew = set.insert(value);

    assert!(set.is_strictly_ascending(), "…across a reallocation");
    assert_eq!(grew, !was_present);
    assert!(set.contains(&value));
}

/// `insert` moves exactly one membership answer and leaves every other alone. Bounds:
/// `SortedSet<u8>`, at most 3 members, one arbitrary bystander.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_touches_only_its_own_member() {
    let before = SortedSet::<u8>::any_canonical::<3>();
    let value: u8 = kani::any();
    let bystander: u8 = kani::any();
    kani::assume(bystander != value);

    let mut after = before.clone();
    after.insert(value);

    assert_eq!(
        after.contains(&bystander),
        before.contains(&bystander),
        "an unrelated cell's membership is untouched"
    );
}

/// `remove` preserves strict ascent, and reports removal exactly when the value was present.
/// Bounds: `SortedSet<u8>`, at most 3 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_remove_preserves_canonical_form() {
    let mut set = SortedSet::<u8>::any_canonical::<3>();
    let value: u8 = kani::any();
    let was_present = set.contains(&value);
    let before = set.len();

    let removed = set.remove(&value);

    assert!(set.is_strictly_ascending(), "order survives removal");
    assert_eq!(removed, was_present, "removal is reported iff it happened");
    assert!(!set.contains(&value), "the removed value is not a member");
    assert_eq!(
        set.len(),
        if removed { before - 1 } else { before },
        "a set shrinks by one member or not at all"
    );
}

/// `position`'s single scan agrees with a naive walk at every value, present or absent, below,
/// between, and above. Bounds: `SortedSet<u8>`, at most 3 members.
///
/// This is the survival chokepoint's primitive (`selector_covers` decides through `contains`):
/// a false negative there turns a collide into a spare, which under-executes.
#[kani::proof]
#[kani::unwind(6)]
fn set_membership_agrees_with_the_walk() {
    let set = SortedSet::<u8>::any_canonical::<3>();
    let probe: u8 = kani::any();
    assert_eq!(set.contains(&probe), member_by_walk(&set, probe));
}

/// ∪ is canonical and is the union at every element. Bounds: two `SortedSet<u8>` of at most 2
/// members each.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic() {
    let a = SortedSet::<u8>::any_canonical::<2>();
    let b = SortedSet::<u8>::any_canonical::<2>();
    let probe: u8 = kani::any();

    let united = a.union(&b);

    assert!(united.is_strictly_ascending(), "∪ keeps the canonical form");
    assert_eq!(
        united.contains(&probe),
        a.contains(&probe) || b.contains(&probe),
        "∪ is membership-or"
    );
    assert_eq!(a.union(&b), b.union(&a), "∪ commutes");
}

/// ∩ is canonical and is the intersection at every element. Bounds: two `SortedSet<u8>` of at
/// most 2 members each.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic() {
    let a = SortedSet::<u8>::any_canonical::<2>();
    let b = SortedSet::<u8>::any_canonical::<2>();
    let probe: u8 = kani::any();

    let met = a.intersection(&b);

    assert!(met.is_strictly_ascending(), "∩ keeps the canonical form");
    assert_eq!(
        met.contains(&probe),
        a.contains(&probe) && b.contains(&probe),
        "∩ is membership-and"
    );
    assert_eq!(a.intersection(&b), b.intersection(&a), "∩ commutes");
}

/// Structural equality IS set equality, in both directions. Bounds: two `SortedSet<u8>` of at
/// most 3 members each.
///
/// The dangerous direction is the second assertion. Two sets with the same members that
/// compared UNEQUAL would only cost the solver an extra round; two sets with different members
/// that compared EQUAL would stop its climb early, under-approximating a may-set — a potential
/// wrong elision no golden can see (`300` §2a).
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq() {
    let a = SortedSet::<u8>::any_canonical::<3>();
    let b = SortedSet::<u8>::any_canonical::<3>();

    if a == b {
        assert!(
            !membership_differs(&a, &b),
            "equal values agree on every member"
        );
    } else {
        assert!(
            membership_differs(&a, &b),
            "unequal values differ on some member — never a bare representation difference"
        );
    }
}

/// `insert` keeps keys ascending; rebinding replaces in place and returns the prior value.
/// Bounds: `SortedMap<u8, u8>`, at most 3 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_insert_keeps_keys_ascending_and_rebinds() {
    let mut map = SortedMap::<u8, u8>::any_canonical::<3>();
    let key: u8 = kani::any();
    let value: u8 = kani::any();
    let prior = map.get(&key).copied();
    let before = map.len();

    let returned = map.insert(key, value);

    assert!(map.keys_are_strictly_ascending(), "the canonical-form seat");
    assert_eq!(returned, prior, "the prior binding is returned, once");
    assert_eq!(map.get(&key), Some(&value), "the new binding is in force");
    assert_eq!(
        map.len(),
        if prior.is_some() { before } else { before + 1 },
        "a rebind does not grow the map"
    );
}

/// `remove` keeps keys ascending and returns the unbound value. Bounds: `SortedMap<u8, u8>`, at
/// most 3 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_remove_keeps_key_order() {
    let mut map = SortedMap::<u8, u8>::any_canonical::<3>();
    let key: u8 = kani::any();
    let prior = map.get(&key).copied();

    let returned = map.remove(&key);

    assert!(map.keys_are_strictly_ascending(), "order survives removal");
    assert_eq!(returned, prior, "the removed value is returned");
    assert_eq!(map.get(&key), None, "the key is unbound");
}

/// `get_at` walks key order and agrees with `get` at every index. Bounds: `SortedMap<u8, u8>`,
/// at most 3 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_get_at_walks_key_order() {
    let map = SortedMap::<u8, u8>::any_canonical::<3>();
    let index: usize = kani::any();
    kani::assume(index < 3);

    match map.get_at(index) {
        Some((key, value)) => assert_eq!(
            map.get(key),
            Some(value),
            "the walk and the lookup see one map"
        ),
        None => assert!(index >= map.len(), "only past the end"),
    }
}

/// Structural equality IS binding-set equality, in both directions. Bounds: two
/// `SortedMap<u8, u8>` of at most 2 bindings each.
///
/// Same asymmetry as the set case: unequal-but-same-content only costs a round, equal-but-
/// different-content stops the climb early.
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq() {
    let a = SortedMap::<u8, u8>::any_canonical::<2>();
    let b = SortedMap::<u8, u8>::any_canonical::<2>();

    if a == b {
        assert!(!bindings_differ(&a, &b), "equal maps answer alike everywhere");
    } else {
        assert!(
            bindings_differ(&a, &b),
            "unequal maps differ at some key — never a bare representation difference"
        );
    }
}

/// Does some key witness a binding difference? Under key-canonical form two maps with the same
/// bindings are the same vector, so a structural difference must be observable as one.
fn bindings_differ(a: &SortedMap<u8, u8>, b: &SortedMap<u8, u8>) -> bool {
    let mut i = 0usize;
    while let Some((k, v)) = a.get_at(i) {
        if b.get(k) != Some(v) {
            return true;
        }
        i = i.saturating_add(1);
    }
    let mut j = 0usize;
    while let Some((k, v)) = b.get_at(j) {
        if a.get(k) != Some(v) {
            return true;
        }
        j = j.saturating_add(1);
    }
    false
}
