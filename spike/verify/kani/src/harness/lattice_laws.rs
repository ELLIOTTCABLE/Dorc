//! The lattice laws `analysis::lattice` documents as "not type-enforceable", per combinator.
//!
//! Seat: `dorc_analysis::lattice::Lattice::{join, meet, leq}` and `BoundedLattice::top`.
//! Law: the doc-comment on [`Lattice`] — ⊔ and ⊓ each associative, commutative and idempotent;
//! absorption both ways; ⊥ the identity of ⊔ and the absorbing element of ⊓; and `x ⊑ y ⟺
//! x ⊔ y = y ⟺ x ⊓ y = x`. Those sentences are the whole contract the solver's termination and
//! its safety direction rest on, and nothing but a check like this one holds them.
//!
//! Helpers are generic; every harness instantiates one concretely, because Kani verifies
//! monomorphic entry points.
//!
//! # Two tiers of combinator, shaped differently, and the measurement that forces it
//!
//! `Flat`, `Product<Flat, Flat>`, `May` and `Must` hold no collection. Their whole domain fits in
//! one harness, so each states the laws over a symbolic pair and covers everything.
//!
//! `Powerset`, `MapL` and the mixed product hold a `SortedSet`/`SortedMap`, and their merges
//! (`union` / `intersection` / pointwise insert) build a result by REPEATED `insert`. The second
//! such insert reallocates a backing whose length has already become symbolic, which is exactly
//! the combination `dorc_core::sorted`'s generator docs measure as unaffordable — and it is an
//! INTERNAL shape, so concrete input lengths do not rescue it. Measured on this corpus: `union`
//! is green with one element on the right and over-budget with two, at every input length tried.
//!
//! A merge is therefore affordable only when it performs at most ONE insert, which is why the
//! collection-shaped combinators are stated at exactly 0 or 1 member and why the two-value laws
//! are split into three groups: no clause may feed one merge's result into another.
//!
//! **Left unjudged at this tier, stated rather than buried:** associativity over a
//! collection-shaped combinator, at ANY size — `a ⊔ (b ⊔ c)` composes two merges by
//! construction, so there is no shape of it a bounded model checker can afford here. Its seat
//! tests in `lattice.rs` remain what it has.

use dorc_analysis::lattice::{BoundedLattice, Flat, Lattice, MapL, May, Must, Powerset, Product};

/// The two-value laws over a symbolic pair — the shape a whole-domain combinator can afford.
fn binary_laws<L: Lattice + core::fmt::Debug + kani::Arbitrary>() {
    binary_laws_of::<L>(kani::any(), kani::any());
}

/// Every two-value clause, as one statement. Split into the three groups below so a
/// collection-shaped combinator can take them one group at a time; a whole-domain combinator
/// takes them all at once through here, and there is still only ONE spelling of each law.
fn binary_laws_of<L: Lattice + core::fmt::Debug>(a: L, b: L) {
    one_value_laws_of(a.clone());
    one_value_laws_of(b.clone());
    merges_commute_of(a.clone(), b.clone());
    merges_absorb_of(a.clone(), b.clone());
    order_reads_the_meet_of(a.clone(), b.clone());
    the_merges_are_the_bounds_of(a, b);
}

/// Everything the contract says about ONE value: ⊥ is ⊔'s identity and ⊓'s absorbing element,
/// both merges are idempotent, and ⊑ is reflexive above ⊥.
fn one_value_laws_of<L: Lattice + core::fmt::Debug>(a: L) {
    let bottom = L::bottom();
    assert_eq!(bottom.join(&a), a, "⊥ ⊔ a = a");
    assert_eq!(a.join(&bottom), a, "a ⊔ ⊥ = a");
    assert_eq!(bottom.meet(&a), bottom, "⊥ ⊓ a = ⊥");
    assert_eq!(a.join(&a), a, "⊔ idempotent");
    assert_eq!(a.meet(&a), a, "⊓ idempotent");
    assert!(a.leq(&a), "⊑ reflexive");
    assert!(bottom.leq(&a), "⊥ ⊑ a");
}

/// Commutativity. Literally symmetric in `a` and `b`, so an UNORDERED enumeration of
/// length-pairs exhausts it.
fn merges_commute_of<L: Lattice + core::fmt::Debug>(a: L, b: L) {
    assert_eq!(a.join(&b), b.join(&a), "⊔ commutative");
    assert_eq!(a.meet(&b), b.meet(&a), "⊓ commutative");
}

/// Absorption, stated from both values so an unordered enumeration is complete. Separate from
/// commutativity because every clause here feeds one merge's result into another.
fn merges_absorb_of<L: Lattice + core::fmt::Debug>(a: L, b: L) {
    assert_eq!(a.join(&a.meet(&b)), a, "a ⊔ (a ⊓ b) = a");
    assert_eq!(a.meet(&a.join(&b)), a, "a ⊓ (a ⊔ b) = a");
    assert_eq!(b.join(&b.meet(&a)), b, "b ⊔ (b ⊓ a) = b");
    assert_eq!(b.meet(&b.join(&a)), b, "b ⊓ (b ⊔ a) = b");
}

/// The induced order agrees with ⊓, both ways round. This is the reading the solver's `leq`
/// default implementation makes, so a disagreement here is a disagreement about what "safe
/// over-approximation" means.
fn order_reads_the_meet_of<L: Lattice + core::fmt::Debug>(a: L, b: L) {
    assert_eq!(a.leq(&b), a.meet(&b) == a, "x ⊑ y ⟺ x ⊓ y = x");
    assert_eq!(b.leq(&a), b.meet(&a) == b, "…and from the other side");
}

/// ⊔ really is an upper bound and ⊓ really is a lower one — the clauses that make the names
/// honest. Split from the group above because each one reads `leq` AGAINST a merge result, which
/// composes two merges.
fn the_merges_are_the_bounds_of<L: Lattice + core::fmt::Debug>(a: L, b: L) {
    let lub = a.join(&b);
    assert!(a.leq(&lub) && b.leq(&lub), "a, b ⊑ a ⊔ b");
    let glb = a.meet(&b);
    assert!(glb.leq(&a) && glb.leq(&b), "a ⊓ b ⊑ a, b");
}

/// Associativity, split out because three arbitrary values cost markedly more than two.
fn associativity<L: Lattice + core::fmt::Debug + kani::Arbitrary>() {
    associativity_of::<L>(kani::any(), kani::any(), kani::any());
}

/// Associativity over given values.
fn associativity_of<L: Lattice + core::fmt::Debug>(a: L, b: L, c: L) {
    assert_eq!(a.join(&b.join(&c)), a.join(&b).join(&c), "⊔ associative");
    assert_eq!(a.meet(&b.meet(&c)), a.meet(&b).meet(&c), "⊓ associative");
}

/// The ⊤ laws, for the combinators with a representable greatest element.
fn top_laws<L: BoundedLattice + core::fmt::Debug + kani::Arbitrary>() {
    let a: L = kani::any();
    let top = L::top();
    assert_eq!(top.meet(&a), a, "⊤ ⊓ a = a");
    assert_eq!(a.meet(&top), a, "a ⊓ ⊤ = a");
    assert_eq!(top.join(&a), top, "⊤ ⊔ a = ⊤");
    assert!(a.leq(&top), "a ⊑ ⊤");
}

// ── Whole-domain combinators: no collection inside, so one harness covers everything ─────────

/// Bounds: `Flat<u8>`, whole domain (⊥, two distinct elements, ⊤ — `Flat` has height 2, so this
/// is the entire lattice).
#[kani::proof]
fn flat_obeys_the_binary_laws() {
    binary_laws::<Flat<u8>>();
}

/// Bounds: `Flat<u8>`, whole domain.
#[kani::proof]
fn flat_is_associative() {
    associativity::<Flat<u8>>();
}

/// Bounds: `Flat<u8>`, whole domain. `Flat` is the one combinator with an intrinsic ⊤.
#[kani::proof]
fn flat_obeys_the_top_laws() {
    top_laws::<Flat<u8>>();
}

/// Bounds: `Product<Flat<u8>, Flat<u8>>`, whole domain — componentwise, so both components'
/// laws must survive the pairing.
#[kani::proof]
fn product_obeys_the_binary_laws() {
    binary_laws::<Product<Flat<u8>, Flat<u8>>>();
}

/// Bounds: `Product<Flat<u8>, Flat<u8>>`, whole domain.
#[kani::proof]
fn product_obeys_the_top_laws() {
    top_laws::<Product<Flat<u8>, Flat<u8>>>();
}

/// `May<L>` is the identity wrapper: same ⊥, same ⊔, same ⊓ as `L`. Bounds: `May<Flat<u8>>`,
/// whole domain.
#[kani::proof]
fn may_obeys_the_binary_and_top_laws() {
    binary_laws::<May<Flat<u8>>>();
    top_laws::<May<Flat<u8>>>();
}

/// `Must<L>` is the ORDER-DUAL of `L`, and these laws passing IS the proof the dual is right:
/// the solver always starts at ⊥ and merges with ⊔, so a must-analysis is a may-analysis over
/// this type and nothing else picks the orientation. Bounds: `Must<Flat<u8>>`, whole domain.
#[kani::proof]
fn must_obeys_the_binary_and_top_laws() {
    binary_laws::<Must<Flat<u8>>>();
    top_laws::<Must<Flat<u8>>>();
}

/// The duality itself, at the operation level. Bounds: `Flat<u8>`, whole domain.
#[kani::proof]
fn must_is_the_order_dual_of_its_inner_lattice() {
    let a: Flat<u8> = kani::any();
    let b: Flat<u8> = kani::any();
    assert_eq!(Must(a.clone()).join(&Must(b.clone())), Must(a.meet(&b)));
    assert_eq!(Must(a.clone()).meet(&Must(b.clone())), Must(a.join(&b)));
    assert_eq!(Must::<Flat<u8>>::bottom(), Must(Flat::Top));
    assert_eq!(Must::<Flat<u8>>::top(), Must(Flat::Bottom));
}

// ── `Powerset<u8>`: one length (or length-pair) per harness ──────────────────────────────────

/// LAW: the one-value clauses (⊥'s two roles, both idempotences, ⊑ reflexive above ⊥).
/// BOUNDS: `Powerset<u8>`, EXACTLY 0 members.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_obeys_the_one_value_laws_at_length_0() {
    one_value_laws_of(Powerset::<u8>::any_at_length::<0>());
}

/// LAW: as above. BOUNDS: `Powerset<u8>`, EXACTLY 1 member.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_obeys_the_one_value_laws_at_length_1() {
    one_value_laws_of(Powerset::<u8>::any_at_length::<1>());
}

/// LAW: ⊔ and ⊓ commute. BOUNDS: two `Powerset<u8>`, EXACTLY 0 and 0 members.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_merges_commute_at_lengths_0_0() {
    merges_commute_of(
        Powerset::<u8>::any_at_length::<0>(),
        Powerset::<u8>::any_at_length::<0>(),
    );
}

/// LAW: ⊔ and ⊓ commute. BOUNDS: two `Powerset<u8>`, EXACTLY 0 and 1 members.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_merges_commute_at_lengths_0_1() {
    merges_commute_of(
        Powerset::<u8>::any_at_length::<0>(),
        Powerset::<u8>::any_at_length::<1>(),
    );
}

/// LAW: ⊔ and ⊓ commute. BOUNDS: two `Powerset<u8>`, EXACTLY 1 and 1 members.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_merges_commute_at_lengths_1_1() {
    merges_commute_of(
        Powerset::<u8>::any_at_length::<1>(),
        Powerset::<u8>::any_at_length::<1>(),
    );
}

/// LAW: `x ⊑ y ⟺ x ⊓ y = x`, both ways round. BOUNDS: two `Powerset<u8>`, EXACTLY 0 and 0.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_order_reads_the_meet_at_lengths_0_0() {
    order_reads_the_meet_of(
        Powerset::<u8>::any_at_length::<0>(),
        Powerset::<u8>::any_at_length::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `Powerset<u8>`, EXACTLY 0 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_order_reads_the_meet_at_lengths_0_1() {
    order_reads_the_meet_of(
        Powerset::<u8>::any_at_length::<0>(),
        Powerset::<u8>::any_at_length::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `Powerset<u8>`, EXACTLY 1 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn powerset_order_reads_the_meet_at_lengths_1_1() {
    order_reads_the_meet_of(
        Powerset::<u8>::any_at_length::<1>(),
        Powerset::<u8>::any_at_length::<1>(),
    );
}

// ── `MapL<u8, Flat<u8>>` ─────────────────────────────────────────────────────────────────────

/// LAW: the one-value clauses. BOUNDS: `MapL<u8, Flat<u8>>`, EXACTLY 0 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_obeys_the_one_value_laws_at_length_0() {
    one_value_laws_of(MapL::<u8, Flat<u8>>::any_at_length::<0>());
}

/// LAW: as above. BOUNDS: `MapL<u8, Flat<u8>>`, EXACTLY 1 binding.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_obeys_the_one_value_laws_at_length_1() {
    one_value_laws_of(MapL::<u8, Flat<u8>>::any_at_length::<1>());
}

/// LAW: ⊔ and ⊓ commute. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 0 and 0 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_merges_commute_at_lengths_0_0() {
    merges_commute_of(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
    );
}

/// LAW: ⊔ and ⊓ commute. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 0 and 1 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_merges_commute_at_lengths_0_1() {
    merges_commute_of(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: ⊔ and ⊓ commute. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 1 and 1 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_merges_commute_at_lengths_1_1() {
    merges_commute_of(
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: `x ⊑ y ⟺ x ⊓ y = x`, both ways round. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 0 and 0.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_order_reads_the_meet_at_lengths_0_0() {
    order_reads_the_meet_of(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 0 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_order_reads_the_meet_at_lengths_0_1() {
    order_reads_the_meet_of(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 1 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_order_reads_the_meet_at_lengths_1_1() {
    order_reads_the_meet_of(
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: `MapL`'s canonical form (no key bound to `V::bottom()`, absent ≡ ⊥) survives both merges
/// in either order and survives `insert`, which also binds what it was asked to. BOUNDS: two
/// `MapL<u8, Flat<u8>>`, EXACTLY 0 and 0 bindings.
///
/// This is the map-side twin of the facade's strict ascent — absent ≡ ⊥ is what makes structural
/// `Eq` semantic here, and convergence detection reads that `Eq`.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_keeps_its_canonical_form_at_lengths_0_0() {
    canonical_form_survives(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 0 and 1 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_keeps_its_canonical_form_at_lengths_0_1() {
    canonical_form_survives(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 1 and 1 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_keeps_its_canonical_form_at_lengths_1_1() {
    canonical_form_survives(
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: both merges are POINTWISE at every key, present or absent. BOUNDS: two
/// `MapL<u8, Flat<u8>>`, EXACTLY 0 and 0 bindings.
///
/// The absent case is the one worth the bounds: a key absent in either map reads ⊥ there, so a
/// ⊓ can only keep keys present in both — stated here as an equation rather than as a walk over
/// the implementation's own key list.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_merges_pointwise_at_lengths_0_0() {
    merges_are_pointwise(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
    );
}

/// LAW: as above. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 0 and 1 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_merges_pointwise_at_lengths_0_1() {
    merges_are_pointwise(
        MapL::<u8, Flat<u8>>::any_at_length::<0>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// LAW: as above. BOUNDS: two `MapL<u8, Flat<u8>>`, EXACTLY 1 and 1 bindings.
#[kani::proof]
#[kani::unwind(8)]
fn maplattice_merges_pointwise_at_lengths_1_1() {
    merges_are_pointwise(
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
        MapL::<u8, Flat<u8>>::any_at_length::<1>(),
    );
}

/// `MapL`'s canonical form under both merges (either order) and under `insert`.
fn canonical_form_survives(a: MapL<u8, Flat<u8>>, b: MapL<u8, Flat<u8>>) {
    assert!(a.join(&b).no_key_maps_to_bottom(), "⊔ keeps the form");
    assert!(b.join(&a).no_key_maps_to_bottom(), "…either way round");
    assert!(a.meet(&b).no_key_maps_to_bottom(), "⊓ keeps the form");
    assert!(b.meet(&a).no_key_maps_to_bottom(), "…either way round");

    let mut grown = a;
    let key: u8 = kani::any();
    let value: Flat<u8> = kani::any();
    grown.insert(key, value.clone());
    assert!(grown.no_key_maps_to_bottom(), "insert keeps the form");
    assert_eq!(
        grown.get(&key),
        value,
        "…while binding what it was asked to"
    );
}

/// Both merges read pointwise at an arbitrary key, from either side.
fn merges_are_pointwise(a: MapL<u8, Flat<u8>>, b: MapL<u8, Flat<u8>>) {
    let key: u8 = kani::any();
    assert_eq!(a.join(&b).get(&key), a.get(&key).join(&b.get(&key)));
    assert_eq!(b.join(&a).get(&key), b.get(&key).join(&a.get(&key)));
    assert_eq!(a.meet(&b).get(&key), a.get(&key).meet(&b.get(&key)));
    assert_eq!(b.meet(&a).get(&key), b.get(&key).meet(&a.get(&key)));
}

// ── `Product<Powerset<u8>, Flat<u8>>` — the mixed shape ──────────────────────────────────────
//
// A product with an unbounded component is a lawful `Lattice` and no `BoundedLattice`, which is
// what makes "a must-analysis over a bare powerset" a compile error rather than a runtime
// surprise. There are deliberately no ⊤ laws to state here, and that absence is the type-level
// asymmetry `165` predicted.

/// LAW: the one-value clauses survive the pairing of a bounded and an unbounded component.
/// BOUNDS: `Product<Powerset<u8>, Flat<u8>>`, the set EXACTLY 0 members, `Flat` whole domain.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_obeys_the_one_value_laws_at_length_0() {
    one_value_laws_of(mixed::<0>());
}

/// LAW: as above. BOUNDS: the set EXACTLY 1 member, `Flat` whole domain.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_obeys_the_one_value_laws_at_length_1() {
    one_value_laws_of(mixed::<1>());
}

/// LAW: ⊔ and ⊓ commute componentwise. BOUNDS: two mixed products, sets EXACTLY 0 and 0.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_merges_commute_at_lengths_0_0() {
    merges_commute_of(mixed::<0>(), mixed::<0>());
}

/// LAW: as above. BOUNDS: two mixed products, sets EXACTLY 0 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_merges_commute_at_lengths_0_1() {
    merges_commute_of(mixed::<0>(), mixed::<1>());
}

/// LAW: as above. BOUNDS: two mixed products, sets EXACTLY 1 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_merges_commute_at_lengths_1_1() {
    merges_commute_of(mixed::<1>(), mixed::<1>());
}

/// LAW: `x ⊑ y ⟺ x ⊓ y = x`, both ways round. BOUNDS: two mixed products, sets EXACTLY 0 and 0.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_order_reads_the_meet_at_lengths_0_0() {
    order_reads_the_meet_of(mixed::<0>(), mixed::<0>());
}

/// LAW: as above. BOUNDS: two mixed products, sets EXACTLY 0 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_order_reads_the_meet_at_lengths_0_1() {
    order_reads_the_meet_of(mixed::<0>(), mixed::<1>());
}

/// LAW: as above. BOUNDS: two mixed products, sets EXACTLY 1 and 1.
#[kani::proof]
#[kani::unwind(8)]
fn mixed_product_order_reads_the_meet_at_lengths_1_1() {
    order_reads_the_meet_of(mixed::<1>(), mixed::<1>());
}

fn mixed<const LEN: usize>() -> Product<Powerset<u8>, Flat<u8>> {
    Product(Powerset::<u8>::any_at_length::<LEN>(), kani::any())
}
