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

use dorc_analysis::lattice::{BoundedLattice, Flat, Lattice, MapL, May, Must, Powerset, Product};

/// The two-value laws: ⊥, idempotence, commutativity, absorption, and the two ⊑ readings.
fn binary_laws<L: Lattice + core::fmt::Debug + kani::Arbitrary>() {
    let a: L = kani::any();
    let b: L = kani::any();
    let bottom = L::bottom();

    assert_eq!(bottom.join(&a), a, "⊥ ⊔ a = a");
    assert_eq!(a.join(&bottom), a, "a ⊔ ⊥ = a");
    assert_eq!(bottom.meet(&a), bottom, "⊥ ⊓ a = ⊥");
    assert_eq!(a.join(&a), a, "⊔ idempotent");
    assert_eq!(a.meet(&a), a, "⊓ idempotent");
    assert_eq!(a.join(&b), b.join(&a), "⊔ commutative");
    assert_eq!(a.meet(&b), b.meet(&a), "⊓ commutative");
    assert_eq!(a.join(&a.meet(&b)), a, "a ⊔ (a ⊓ b) = a");
    assert_eq!(a.meet(&a.join(&b)), a, "a ⊓ (a ⊔ b) = a");

    assert!(a.leq(&a), "⊑ reflexive");
    assert!(bottom.leq(&a), "⊥ ⊑ a");
    assert_eq!(a.leq(&b), a.meet(&b) == a, "x ⊑ y ⟺ x ⊓ y = x");
    let lub = a.join(&b);
    let glb = a.meet(&b);
    assert!(a.leq(&lub) && b.leq(&lub), "a, b ⊑ a ⊔ b");
    assert!(glb.leq(&a) && glb.leq(&b), "a ⊓ b ⊑ a, b");
}

/// Associativity, split out because three arbitrary values cost markedly more than two.
fn associativity<L: Lattice + core::fmt::Debug + kani::Arbitrary>() {
    let a: L = kani::any();
    let b: L = kani::any();
    let c: L = kani::any();
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

/// Bounds: `Powerset<u8>`, at most 3 members. Deliberately NOT a `BoundedLattice` — the
/// universal set is unrepresentable — so there are no ⊤ laws to state here, and that absence is
/// the type-level asymmetry `165` predicted.
#[kani::proof]
#[kani::unwind(6)]
fn powerset_obeys_the_binary_laws() {
    binary_laws::<Powerset<u8>>();
}

/// Bounds: `Powerset<u8>`, at most 3 members, three values.
#[kani::proof]
#[kani::unwind(6)]
fn powerset_is_associative() {
    associativity::<Powerset<u8>>();
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

/// Bounds: `Product<Powerset<u8>, Flat<u8>>`, sets of at most 2. The mixed shape: a product
/// with an unbounded component is a lawful `Lattice` and no `BoundedLattice`, which is what
/// makes "a must-analysis over a bare powerset" a compile error rather than a runtime surprise.
#[kani::proof]
#[kani::unwind(6)]
fn mixed_product_is_a_lattice_without_a_top() {
    binary_laws::<Product<Powerset<u8>, Flat<u8>>>();
}

/// Bounds: `MapL<u8, Flat<u8>>`, at most 2 bindings.
#[kani::proof]
#[kani::unwind(6)]
fn maplattice_obeys_the_binary_laws() {
    binary_laws::<MapL<u8, Flat<u8>>>();
}

/// Bounds: `MapL<u8, Flat<u8>>`, at most 2 bindings, three values.
#[kani::proof]
#[kani::unwind(6)]
fn maplattice_is_associative() {
    associativity::<MapL<u8, Flat<u8>>>();
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

/// `MapL`'s canonical form: no key maps to `V::bottom()`, and it survives every mutation and
/// both merges. Bounds: `MapL<u8, Flat<u8>>`, at most 2 bindings.
///
/// This is the map-side twin of the facade's strict ascent — absent ≡ ⊥ is what makes
/// structural `Eq` semantic here, and convergence detection reads that `Eq`.
#[kani::proof]
#[kani::unwind(6)]
fn maplattice_keeps_its_canonical_form() {
    let mut map: MapL<u8, Flat<u8>> = kani::any();
    let other: MapL<u8, Flat<u8>> = kani::any();
    let key: u8 = kani::any();
    let value: Flat<u8> = kani::any();

    assert!(
        map.join(&other).no_key_maps_to_bottom(),
        "⊔ keeps the form"
    );
    assert!(
        map.meet(&other).no_key_maps_to_bottom(),
        "⊓ keeps the form"
    );

    map.insert(key, value.clone());
    assert!(map.no_key_maps_to_bottom(), "insert keeps the form");
    assert_eq!(map.get(&key), value, "…while binding what it was asked to");
}

/// `MapL`'s merges are POINTWISE at every key, present or absent. Bounds: `MapL<u8, Flat<u8>>`,
/// at most 2 bindings.
///
/// The absent case is the one worth the bounds: a key absent in either map reads ⊥ there, so a
/// ⊓ can only keep keys present in both — stated here as an equation rather than as a walk over
/// the implementation's own key list.
#[kani::proof]
#[kani::unwind(6)]
fn maplattice_merges_pointwise() {
    let a: MapL<u8, Flat<u8>> = kani::any();
    let b: MapL<u8, Flat<u8>> = kani::any();
    let key: u8 = kani::any();

    assert_eq!(a.join(&b).get(&key), a.get(&key).join(&b.get(&key)));
    assert_eq!(a.meet(&b).get(&key), a.get(&key).meet(&b.get(&key)));
}
