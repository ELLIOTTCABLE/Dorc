//! The coordinate chokepoints and the ternary they mint — `core::coord`.
//!
//! Seats: `dorc_core::coord::{selector_covers, selector_identifies, compare}` (the
//! `selector-chokepoint` and `relational-compare-chokepoint` of `core/CLAUDE.md`).
//! Laws: `sparing-algebra` · `set-lifting-universal-meet` · `pin-set-meet-order-independence` ·
//! `pin-no-outcome-as-generator` · `never-derive-separation` · `top-identifies-with-nothing` ·
//! `empty-world-byte-identical`.
//!
//! Symbols are drawn from a two-identity domain: the engine may only compare them
//! (`inv-referent-agnostic`), so two identities exhaust every equality pattern a pair can
//! express and a wider domain would buy nothing but solver time.

use dorc_core::coord::{
    Context, Coord, Dialect, EntityResolution, Relation, compare, selector_covers,
    selector_identifies,
};
use dorc_core::sorted::SortedSet;
use dorc_core::{ProviderId, SelectorId};

/// ⊤ on either side collides, and a token equal to its backing collides. Bounds: dialects of at
/// most 3 tokens over a two-identity symbol domain.
///
/// The second assertion is the `279f:fix-spare-top-backing` regression made exhaustive: the
/// pre-amendment algebra spared whole-entity backings, which is an under-execution path.
#[kani::proof]
#[kani::unwind(6)]
fn selector_covers_never_spares_a_top_or_a_self() {
    let claim: Option<SelectorId> = kani::any();
    let backing: Option<SelectorId> = kani::any();
    let dialect = SortedSet::<SelectorId>::any_canonical::<3>();

    let spares = !selector_covers(claim, backing, &dialect);

    if claim.is_none() || backing.is_none() {
        assert!(!spares, "⊤ identifies with nothing and spares nothing");
    }
    if claim == backing {
        assert!(!spares, "a cell never spares itself");
    }
}

/// A token absent from the backing family's dialect collides, whichever side carries it, and an
/// empty dialect spares nothing at all. Bounds: dialects of at most 3 tokens.
///
/// The empty-dialect half is `empty-world-byte-identical` at this seat: with no oracles loaded
/// the comparison is entity-granular and the whole sparing algebra is invisible.
#[kani::proof]
#[kani::unwind(6)]
fn selector_covers_needs_both_tokens_minted() {
    let claim: Option<SelectorId> = kani::any();
    let backing: Option<SelectorId> = kani::any();
    let dialect = SortedSet::<SelectorId>::any_canonical::<3>();

    let unminted = match (claim, backing) {
        (Some(c), Some(b)) => !dialect.contains(&c) || !dialect.contains(&b),
        _ => true,
    };
    if unminted {
        assert!(
            selector_covers(claim, backing, &dialect),
            "an unminted token on either side is ⊤-selector, and ⊤ collides"
        );
    }

    let empty = SortedSet::<SelectorId>::new();
    assert!(
        selector_covers(claim, backing, &empty),
        "no dialect, no sparing"
    );
}

/// Growing a dialect can only ever create sparing, never destroy it. Bounds: dialects of at
/// most 2 tokens, one added token.
///
/// This is what makes loading another oracle monotone at this seat: an authored mark may widen
/// what the algebra can separate, and may never silently re-collide something it already
/// spared, which would be an unexplained verdict change on an unrelated line.
#[kani::proof]
#[kani::unwind(6)]
fn selector_covers_is_monotone_in_the_dialect() {
    let claim: Option<SelectorId> = kani::any();
    let backing: Option<SelectorId> = kani::any();
    let smaller = SortedSet::<SelectorId>::any_canonical::<2>();
    let mut larger = smaller.clone();
    larger.insert(kani::any());

    if !selector_covers(claim, backing, &smaller) {
        assert!(
            !selector_covers(claim, backing, &larger),
            "a spare survives a dialect that grew"
        );
    }
}

/// ⊤ identifies with nothing, including itself; identity is otherwise exactly token equality.
/// Bounds: a two-identity symbol domain.
#[kani::proof]
fn selector_identifies_only_two_concrete_equal_tokens() {
    let a: Option<SelectorId> = kani::any();
    let b: Option<SelectorId> = kani::any();

    let identifies = selector_identifies(a, b);
    assert_eq!(identifies, selector_identifies(b, a), "symmetric");
    if a.is_none() || b.is_none() {
        assert!(!identifies, "⊤ never identifies, not even with itself");
    }
    if identifies {
        assert_eq!(a, b, "identity is token equality");
    }
}

/// `compare` never manufactures separation. Bounds: arbitrary coordinates over a two-identity
/// symbol domain, dialects of at most 2 mints.
///
/// `never-derive-separation` is the law: address-inequality is not referent-inequality, so
/// `ProvablyDisjoint` may come only from ground truth (a different kind), the resolve generator
/// (distinct canonical entities), or an authored dialect spare — and from nothing else, ever.
/// A resolver gap or an unminted selector must land on `Unknown`/`Overlaps`, which are the safe
/// bottom for both consumers.
#[kani::proof]
#[kani::unwind(6)]
fn compare_derives_separation_only_from_its_three_sources() {
    let claim: Coord = kani::any();
    let backing: Coord = kani::any();
    let claim_canon: EntityResolution = kani::any();
    let backing_canon: EntityResolution = kani::any();
    let dialect = Dialect::any_minted::<2>();
    let family: Option<ProviderId> = kani::any();

    let relation = compare(
        claim,
        backing,
        claim_canon,
        backing_canon,
        &dialect,
        family,
    );

    if relation == Relation::ProvablyDisjoint {
        let different_kind = claim.kind != backing.kind;
        let distinct_entities = match (claim_canon, backing_canon) {
            (EntityResolution::Canonical(a), EntityResolution::Canonical(b)) => a != b,
            _ => false,
        };
        let dialect_spared = !selector_covers(
            claim.selector,
            backing.selector,
            dialect.tokens(family, backing.kind),
        );
        assert!(
            different_kind || distinct_entities || dialect_spared,
            "separation has exactly three sources"
        );
        assert_eq!(
            claim.context, backing.context,
            "and none of them crosses a context gap"
        );
    }
}

/// A context gap is the safe bottom for both consumers, checked before any short-circuit.
/// Bounds: as above.
///
/// `27C` §3's non-negotiable requirement: a fact born in a wrapper-denoted world neither
/// transports to the ambient world nor spares a disturbance there. The ordering inside
/// `compare` is what makes transport-by-collision unrepresentable rather than merely unlikely,
/// so it is pinned as an implication over EVERY input rather than as two example worlds.
#[kani::proof]
#[kani::unwind(6)]
fn compare_answers_unknown_across_a_context_gap() {
    let claim: Coord = kani::any();
    let mut backing: Coord = kani::any();
    kani::assume(claim.context != backing.context);

    let relation = compare(
        claim,
        backing,
        kani::any(),
        kani::any(),
        &Dialect::any_minted::<2>(),
        kani::any(),
    );

    assert_eq!(relation, Relation::Unknown, "different worlds, no verdict");
}

/// A resolver gap on either side, within one kind, is `Unknown`. Bounds: as above.
#[kani::proof]
#[kani::unwind(6)]
fn compare_answers_unknown_on_a_resolver_gap() {
    let claim: Coord = kani::any();
    let mut backing: Coord = kani::any();
    backing.kind = claim.kind;
    backing.context = claim.context;
    let claim_canon: EntityResolution = kani::any();
    let backing_canon: EntityResolution = kani::any();
    kani::assume(
        claim_canon == EntityResolution::Unresolvable
            || backing_canon == EntityResolution::Unresolvable,
    );

    let relation = compare(
        claim,
        backing,
        claim_canon,
        backing_canon,
        &Dialect::any_minted::<2>(),
        kani::any(),
    );

    assert_eq!(relation, Relation::Unknown, "fail toward run");
}

// ── The welded consumer map (`ternary-compare-consumer-map`) ────────────────────────────────
//
// The map itself is the law, not an implementation: *provably-disjoint* feeds survival sparing
// and nothing else; *overlaps* feeds survival collide, and licenses transport only when
// `selector_identifies` separately says the two coordinates name one cell; *unknown* is the
// safe bottom for BOTH. It is ternary because of the safety inversion — believed-no-overlap is
// safe for transport and dangerous for kill-traffic, and vice versa — so no binary default is
// safe for both consumers.

fn spares_survival(relation: Relation) -> bool {
    matches!(relation, Relation::ProvablyDisjoint)
}

fn licenses_transport(relation: Relation, identifies: bool) -> bool {
    matches!(relation, Relation::Overlaps) && identifies
}

/// Every verdict lands in exactly one consumer, and `Unknown` licenses neither. Bounds: the
/// whole `Relation` domain (three variants) crossed with both identity answers.
#[kani::proof]
fn the_consumer_map_is_exhaustive_and_exclusive() {
    let relation: Relation = kani::any();
    let identifies: bool = kani::any();

    assert!(
        !(spares_survival(relation) && licenses_transport(relation, identifies)),
        "no verdict feeds both consumers"
    );
    if relation == Relation::Unknown {
        assert!(!spares_survival(relation), "the safe bottom spares nothing");
        assert!(
            !licenses_transport(relation, identifies),
            "the safe bottom transports nothing"
        );
    }
    if relation == Relation::Overlaps {
        assert!(!spares_survival(relation), "an overlap collides");
        assert_eq!(
            licenses_transport(relation, identifies),
            identifies,
            "an overlap alone is not sameness — transport is separately gated"
        );
    }
}

// ── The universal meet over backing-SETS (`277` §5) ─────────────────────────────────────────
//
// Backing-sets are a RESERVED seam (`core/CLAUDE.md` seam-backing-sets — singletons at v1), so
// the fold below is the harness's, exactly as `coord.rs`'s own test writes it. What is being
// pinned is the LAW the seam must be built to, before there is anything to get wrong. The fold
// is a PURE map-and-conjunction: no member's outcome is ever an input to another's, which is
// `pin-no-outcome-as-generator` at the shape level.

fn set_spares(members: &[Relation]) -> bool {
    let mut i = 0usize;
    while i < members.len() {
        match members.get(i) {
            Some(Relation::ProvablyDisjoint) => i = i.saturating_add(1),
            _ => return false,
        }
    }
    true
}

/// Any member that is not provably-disjoint collides the whole set, whatever the resolution
/// order. Bounds: sets of exactly 3 members over the whole `Relation` domain — every one of the
/// 27 member-triples, in all 6 orders.
///
/// `pin-set-meet-order-independence` exists because a set-meet that answered differently
/// depending on which member resolved first would make a sparing verdict depend on iteration
/// order — nondeterminism in a decision that licenses skipping a mutation.
#[kani::proof]
fn the_universal_meet_is_order_independent() {
    let a: Relation = kani::any();
    let b: Relation = kani::any();
    let c: Relation = kani::any();

    let answer = set_spares(&[a, b, c]);
    assert_eq!(answer, set_spares(&[a, c, b]));
    assert_eq!(answer, set_spares(&[b, a, c]));
    assert_eq!(answer, set_spares(&[b, c, a]));
    assert_eq!(answer, set_spares(&[c, a, b]));
    assert_eq!(answer, set_spares(&[c, b, a]));

    let any_member_collides = [a, b, c].iter().any(|r| *r != Relation::ProvablyDisjoint);
    assert_eq!(
        answer, !any_member_collides,
        "the set spares iff EVERY member does"
    );
}

/// The empty set spares vacuously — which is why backing-sets must be non-empty by
/// construction, and why ⊤ is never encoded as ∅. Bounds: the empty set, and sets of one
/// member over the whole `Relation` domain.
///
/// `inv-backing-set-nonempty-by-construction` and `inv-top-never-encoded-as-empty` are one
/// hazard seen from two sides: a universal quantifier over nothing is true, so an empty
/// backing-set would spare every disturbance, and a ⊤ coordinate spelled as an empty set would
/// be the most permissive value in the algebra rather than the most conservative. The
/// assertion below states the vacuity outright rather than leaving it as a thing to notice.
#[kani::proof]
fn an_empty_backing_set_would_spare_vacuously() {
    assert!(
        set_spares(&[]),
        "∀ over ∅ is true — the reason the seam must never admit one"
    );

    let only: Relation = kani::any();
    assert_eq!(
        set_spares(&[only]),
        only == Relation::ProvablyDisjoint,
        "a real, non-empty set answers from its members"
    );
}
