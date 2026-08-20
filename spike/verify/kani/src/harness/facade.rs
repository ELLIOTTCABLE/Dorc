//! Facade canonicality — `core::sorted::{SortedSet, SortedMap}`.
//!
//! Seat: `300` §2a's invariant-seat bank (`SortedSet::insert` is THE canonical-form seat).
//! Law: the backing is strictly ascending, so the derived `PartialEq` is semantic equality —
//! which is what `solve`'s `joined != state[w]` convergence test rests on.
//!
//! Every generator draws an arbitrary `Vec` and assumes canonical (`300` §2a's Arbitrary law):
//! generating by repeated `insert` would make the `insert` harnesses assume what they prove.
//!
//! # One harness per length, and the measurement that forces it
//!
//! A `Vec` whose length is SYMBOLIC and whose backing is FULL reallocates at a symbolic size, and
//! reading the result back is what a bounded model checker cannot afford (`dorc_core::sorted`'s
//! generator docs carry the measurement). So each law below is stated at CONCRETE lengths, one
//! harness per length or length-pair, and each declares exactly the universe it verifies.
//!
//! The same wall reappears INSIDE the two set operators, which is why their enumerations are
//! lopsided. `union` reuses a canonical-prefix operand, but its general path clones the left side
//! and `insert`s the right side element by element; the second such insert reallocates a backing
//! whose length has ALREADY become symbolic. So an unconstrained `union` is affordable with at
//! most ONE element on the right (measured: green with one, over-budget with two, at every
//! left-hand length tried), and `intersection` — which inserts survivors of the LEFT side into a
//! fresh set — with at most one element on the left.
//!
//! **What that leaves unjudged, stated rather than buried:** `union` with two or more elements on
//! the right, `intersection` with two or more on the left, and therefore the commutativity of
//! either beyond one-element operands. Their seat tests in `sorted.rs` remain what they have.

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

// ── `SortedSet::insert`, the canonical-form seat ─────────────────────────────────────────────

/// `insert` preserves strict ascent and reports growth exactly when the value was absent.
fn insert_preserves_canonical_form(mut set: SortedSet<u8>) {
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

/// LAW: `insert` preserves strict ascent, reports growth iff it happened, and moves the length by
/// at most one. BOUNDS: `SortedSet<u8>`, EXACTLY 0 members, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_preserves_canonical_form_at_length_0() {
    insert_preserves_canonical_form(SortedSet::any_canonical_at_capacity::<0>());
}

/// LAW: as above. BOUNDS: `SortedSet<u8>`, EXACTLY 1 member, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_preserves_canonical_form_at_length_1() {
    insert_preserves_canonical_form(SortedSet::any_canonical_at_capacity::<1>());
}

/// LAW: as above. BOUNDS: `SortedSet<u8>`, EXACTLY 2 members, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_preserves_canonical_form_at_length_2() {
    insert_preserves_canonical_form(SortedSet::any_canonical_at_capacity::<2>());
}

/// LAW: as above. BOUNDS: `SortedSet<u8>`, EXACTLY 3 members, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_preserves_canonical_form_at_length_3() {
    insert_preserves_canonical_form(SortedSet::any_canonical_at_capacity::<3>());
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

/// `insert` moves exactly one membership answer and leaves every other alone.
fn insert_touches_only_its_own_member(before: SortedSet<u8>) {
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

/// LAW: `insert` moves exactly one membership answer and leaves every other alone. BOUNDS:
/// `SortedSet<u8>`, EXACTLY 0 members, one arbitrary bystander.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_touches_only_its_own_member_at_length_0() {
    insert_touches_only_its_own_member(SortedSet::any_canonical_at_capacity::<0>());
}

/// LAW: as above. BOUNDS: `SortedSet<u8>`, EXACTLY 1 member, one arbitrary bystander.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_touches_only_its_own_member_at_length_1() {
    insert_touches_only_its_own_member(SortedSet::any_canonical_at_capacity::<1>());
}

/// LAW: as above. BOUNDS: `SortedSet<u8>`, EXACTLY 2 members, one arbitrary bystander.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_touches_only_its_own_member_at_length_2() {
    insert_touches_only_its_own_member(SortedSet::any_canonical_at_capacity::<2>());
}

/// LAW: as above. BOUNDS: `SortedSet<u8>`, EXACTLY 3 members, one arbitrary bystander.
#[kani::proof]
#[kani::unwind(6)]
fn set_insert_touches_only_its_own_member_at_length_3() {
    insert_touches_only_its_own_member(SortedSet::any_canonical_at_capacity::<3>());
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

// ── The two set operators ────────────────────────────────────────────────────────────────────

/// ∪ is canonical and is membership-or at every element.
fn union_is_canonical_and_semantic(a: &SortedSet<u8>, b: &SortedSet<u8>) {
    let probe: u8 = kani::any();
    let united = a.union(b);

    assert!(united.is_strictly_ascending(), "∪ keeps the canonical form");
    assert_eq!(
        united.contains(&probe),
        a.contains(&probe) || b.contains(&probe),
        "∪ is membership-or"
    );
}

/// LAW: ∪ keeps the canonical form and is membership-or at every element. BOUNDS: two
/// `SortedSet<u8>`, EXACTLY 0 and 0 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic_at_lengths_0_0() {
    union_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 0 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic_at_lengths_0_1() {
    union_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 0 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic_at_lengths_1_0() {
    union_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic_at_lengths_1_1() {
    union_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 2 and 0 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic_at_lengths_2_0() {
    union_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<2>(),
        &SortedSet::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 2 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_is_canonical_and_semantic_at_lengths_2_1() {
    union_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<2>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: ∪ commutes. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 1 members — the largest pair at
/// which BOTH directions can be computed, since each costs one internal `insert` per element of
/// its right operand.
#[kani::proof]
#[kani::unwind(6)]
fn set_union_commutes_at_lengths_1_1() {
    let a = SortedSet::<u8>::any_canonical_at_capacity::<1>();
    let b = SortedSet::<u8>::any_canonical_at_capacity::<1>();
    assert_eq!(a.union(&b), b.union(&a), "∪ commutes");
}

/// ∩ is canonical and is membership-and at every element.
fn intersection_is_canonical_and_semantic(a: &SortedSet<u8>, b: &SortedSet<u8>) {
    let probe: u8 = kani::any();
    let met = a.intersection(b);

    assert!(met.is_strictly_ascending(), "∩ keeps the canonical form");
    assert_eq!(
        met.contains(&probe),
        a.contains(&probe) && b.contains(&probe),
        "∩ is membership-and"
    );
}

/// LAW: ∩ keeps the canonical form and is membership-and at every element. BOUNDS: two
/// `SortedSet<u8>`, EXACTLY 0 and 0 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic_at_lengths_0_0() {
    intersection_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 0 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic_at_lengths_0_1() {
    intersection_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 0 and 2 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic_at_lengths_0_2() {
    intersection_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 0 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic_at_lengths_1_0() {
    intersection_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic_at_lengths_1_1() {
    intersection_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 2 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_is_canonical_and_semantic_at_lengths_1_2() {
    intersection_is_canonical_and_semantic(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: ∩ commutes. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 1 members — the largest pair at
/// which both directions can be computed.
#[kani::proof]
#[kani::unwind(6)]
fn set_intersection_commutes_at_lengths_1_1() {
    let a = SortedSet::<u8>::any_canonical_at_capacity::<1>();
    let b = SortedSet::<u8>::any_canonical_at_capacity::<1>();
    assert_eq!(a.intersection(&b), b.intersection(&a), "∩ commutes");
}

// ── Structural equality IS semantic equality ─────────────────────────────────────────────────

/// Structural equality IS set equality, in both directions.
///
/// The dangerous direction is the second branch. Two sets with the same members that compared
/// UNEQUAL would only cost the solver an extra round; two sets with different members that
/// compared EQUAL would stop its climb early, under-approximating a may-set — a potential wrong
/// elision no golden can see (`300` §2a).
fn structural_eq_is_set_eq(a: &SortedSet<u8>, b: &SortedSet<u8>) {
    if a == b {
        assert!(
            !membership_differs(a, b),
            "equal values agree on every member"
        );
    } else {
        assert!(
            membership_differs(a, b),
            "unequal values differ on some member — never a bare representation difference"
        );
    }
}

/// LAW: structural equality IS set equality, in both directions. BOUNDS: two `SortedSet<u8>`,
/// EXACTLY 0 and 0 members. (The statement is symmetric in its two values, so unordered
/// length-pairs exhaust it.)
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_0_0() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 0 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_0_1() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 0 and 2 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_0_2() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 0 and 3 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_0_3() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<0>(),
        &SortedSet::any_canonical_at_capacity::<3>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 1 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_1_1() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 2 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_1_2() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 1 and 3 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_1_3() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<1>(),
        &SortedSet::any_canonical_at_capacity::<3>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 2 and 2 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_2_2() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<2>(),
        &SortedSet::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 2 and 3 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_2_3() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<2>(),
        &SortedSet::any_canonical_at_capacity::<3>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedSet<u8>`, EXACTLY 3 and 3 members.
#[kani::proof]
#[kani::unwind(6)]
fn set_structural_eq_is_set_eq_at_lengths_3_3() {
    structural_eq_is_set_eq(
        &SortedSet::any_canonical_at_capacity::<3>(),
        &SortedSet::any_canonical_at_capacity::<3>(),
    );
}

// ── `SortedMap` ──────────────────────────────────────────────────────────────────────────────

/// `insert` keeps keys ascending; rebinding replaces in place and returns the prior value.
fn map_insert_keeps_keys_ascending(mut map: SortedMap<u8, u8>) {
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

/// LAW: `insert` keeps keys ascending, returns the prior binding once, puts the new one in force,
/// and grows only on a fresh key. BOUNDS: `SortedMap<u8, u8>`, EXACTLY 0 bindings, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn map_insert_keeps_keys_ascending_and_rebinds_at_length_0() {
    map_insert_keeps_keys_ascending(SortedMap::any_canonical_at_capacity::<0>());
}

/// LAW: as above. BOUNDS: `SortedMap<u8, u8>`, EXACTLY 1 binding, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn map_insert_keeps_keys_ascending_and_rebinds_at_length_1() {
    map_insert_keeps_keys_ascending(SortedMap::any_canonical_at_capacity::<1>());
}

/// LAW: as above. BOUNDS: `SortedMap<u8, u8>`, EXACTLY 2 bindings, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn map_insert_keeps_keys_ascending_and_rebinds_at_length_2() {
    map_insert_keeps_keys_ascending(SortedMap::any_canonical_at_capacity::<2>());
}

/// LAW: as above. BOUNDS: `SortedMap<u8, u8>`, EXACTLY 3 bindings, at capacity.
#[kani::proof]
#[kani::unwind(6)]
fn map_insert_keeps_keys_ascending_and_rebinds_at_length_3() {
    map_insert_keeps_keys_ascending(SortedMap::any_canonical_at_capacity::<3>());
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

/// Structural equality IS binding-set equality, in both directions.
///
/// Same asymmetry as the set case: unequal-but-same-content only costs a round, equal-but-
/// different-content stops the climb early.
fn structural_eq_is_binding_eq(a: &SortedMap<u8, u8>, b: &SortedMap<u8, u8>) {
    if a == b {
        assert!(!bindings_differ(a, b), "equal maps answer alike everywhere");
    } else {
        assert!(
            bindings_differ(a, b),
            "unequal maps differ at some key — never a bare representation difference"
        );
    }
}

/// LAW: structural equality IS binding-set equality, in both directions. BOUNDS: two
/// `SortedMap<u8, u8>`, EXACTLY 0 and 0 bindings. (Symmetric in its two values, so unordered
/// length-pairs exhaust it.)
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq_at_lengths_0_0() {
    structural_eq_is_binding_eq(
        &SortedMap::any_canonical_at_capacity::<0>(),
        &SortedMap::any_canonical_at_capacity::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedMap<u8, u8>`, EXACTLY 0 and 1 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq_at_lengths_0_1() {
    structural_eq_is_binding_eq(
        &SortedMap::any_canonical_at_capacity::<0>(),
        &SortedMap::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedMap<u8, u8>`, EXACTLY 0 and 2 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq_at_lengths_0_2() {
    structural_eq_is_binding_eq(
        &SortedMap::any_canonical_at_capacity::<0>(),
        &SortedMap::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedMap<u8, u8>`, EXACTLY 1 and 1 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq_at_lengths_1_1() {
    structural_eq_is_binding_eq(
        &SortedMap::any_canonical_at_capacity::<1>(),
        &SortedMap::any_canonical_at_capacity::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedMap<u8, u8>`, EXACTLY 1 and 2 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq_at_lengths_1_2() {
    structural_eq_is_binding_eq(
        &SortedMap::any_canonical_at_capacity::<1>(),
        &SortedMap::any_canonical_at_capacity::<2>(),
    );
}

/// LAW: as above. BOUNDS: two `SortedMap<u8, u8>`, EXACTLY 2 and 2 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn map_structural_eq_is_binding_eq_at_lengths_2_2() {
    structural_eq_is_binding_eq(
        &SortedMap::any_canonical_at_capacity::<2>(),
        &SortedMap::any_canonical_at_capacity::<2>(),
    );
}
