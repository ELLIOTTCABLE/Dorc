//! Reference model of Dorc's sparing and composition algebra.
//!
//! Authored solely from the ratified English law-set and deliberately blind to
//! the production implementation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Opaque kind identity.
pub struct KindToken(u64);

impl KindToken {
    /// Creates a kind identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Opaque entity identity.
pub struct EntityToken(u64);

impl EntityToken {
    /// Creates an entity identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Opaque selector identity.
pub struct SelectorToken(u64);

impl SelectorToken {
    /// Creates a selector identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Opaque oracle-family identity.
pub struct FamilyToken(u64);

impl FamilyToken {
    /// Creates an oracle-family identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Selector information carried by a coordinate.
pub enum Selector {
    /// Selector-less whole-entity coordinate.
    Top,
    /// Token without minting authority.
    Unminted(SelectorToken),
    /// Token minted by a runnable marked line.
    Minted {
        /// Minted selector token.
        token: SelectorToken,
        /// Family that minted the token.
        family: FamilyToken,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Minimal cell coordinate used by the relation.
pub struct Coordinate {
    /// Coordinate kind.
    pub kind: KindToken,
    /// Coordinate entity.
    pub entity: EntityToken,
    /// Coordinate selector state.
    pub selector: Selector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Coordinate carried by a footprint claim.
pub struct Claim(pub Coordinate);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Coordinate backing an established fact.
pub struct Backing(pub Coordinate);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Selector vocabulary minted by one family for one kind.
pub struct Dialect<'a> {
    /// Minting family.
    pub family: FamilyToken,
    /// Kind interpreted by this vocabulary.
    pub kind: KindToken,
    /// Tokens in the vocabulary.
    pub selectors: &'a [SelectorToken],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Non-empty set of backing coordinates.
pub struct BackingSet<'a> {
    first: Backing,
    rest: &'a [Backing],
}

impl<'a> BackingSet<'a> {
    /// Creates a set containing `first` and every item in `rest`.
    #[must_use]
    pub const fn new(first: Backing, rest: &'a [Backing]) -> Self {
        Self { first, rest }
    }

    /// Returns `false`; emptiness is unrepresentable.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Closed result of the coordinate relation.
pub enum CompareVerdict {
    /// The coordinates identify the same cell.
    Same,
    /// A ratified generator proves the cells disjoint.
    ProvablyDisjoint,
    /// The relation cannot license either consumer.
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Closed result of applying the sparing consumer to one pair.
pub enum PairSparingVerdict {
    /// The claim spares the backing.
    Spares,
    /// The claim collides with the backing.
    Collides,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Closed result of universally lifting sparing over two sets.
pub enum SetSparingVerdict {
    /// Every footprint-by-backing pair is provably disjoint.
    Spares,
    /// At least one pair is not provably disjoint.
    Collides,
}

/// Compares one claim coordinate with one backing coordinate.
#[must_use]
pub fn compare(claim: Claim, backing: Backing, dialects: &[Dialect<'_>]) -> CompareVerdict {
    let claim = claim.0;
    let backing = backing.0;

    if claim.kind != backing.kind {
        return CompareVerdict::Unknown;
    }

    if claim.entity != backing.entity {
        // FLAGGED: the assigned law-set gives no unequal-entity disjointness generator.
        return CompareVerdict::Unknown;
    }

    let (
        Selector::Minted {
            token: claim_token, ..
        },
        Selector::Minted {
            token: backing_token,
            family: backing_family,
        },
    ) = (claim.selector, backing.selector)
    else {
        return CompareVerdict::Unknown;
    };

    if claim_token == backing_token {
        return CompareVerdict::Same;
    }

    for dialect in dialects {
        if dialect.family != backing_family || dialect.kind != backing.kind {
            continue;
        }

        for selector in dialect.selectors {
            if *selector == claim_token {
                return CompareVerdict::ProvablyDisjoint;
            }
        }
    }

    CompareVerdict::Unknown
}

/// Applies the survival-sparing consumer to one pair.
#[must_use]
pub fn spare_pair(claim: Claim, backing: Backing, dialects: &[Dialect<'_>]) -> PairSparingVerdict {
    match compare(claim, backing, dialects) {
        CompareVerdict::ProvablyDisjoint => PairSparingVerdict::Spares,
        CompareVerdict::Same | CompareVerdict::Unknown => PairSparingVerdict::Collides,
    }
}

/// Universally lifts sparing over a footprint and non-empty backing set.
#[must_use]
pub fn spare_set(
    footprint: &[Claim],
    backing_set: BackingSet<'_>,
    dialects: &[Dialect<'_>],
) -> SetSparingVerdict {
    if footprint.is_empty() {
        // FLAGGED: the assigned law-set requires non-empty backings, but is silent on footprints.
        return SetSparingVerdict::Collides;
    }

    for claim in footprint {
        if spare_pair(*claim, backing_set.first, dialects) == PairSparingVerdict::Collides {
            return SetSparingVerdict::Collides;
        }

        for backing in backing_set.rest {
            if spare_pair(*claim, *backing, dialects) == PairSparingVerdict::Collides {
                return SetSparingVerdict::Collides;
            }
        }
    }

    SetSparingVerdict::Spares
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIND_A: KindToken = KindToken::new(1);
    const KIND_B: KindToken = KindToken::new(2);
    const ENTITY_A: EntityToken = EntityToken::new(10);
    const ENTITY_B: EntityToken = EntityToken::new(11);
    const SELECTOR_A: SelectorToken = SelectorToken::new(20);
    const SELECTOR_B: SelectorToken = SelectorToken::new(21);
    const SELECTOR_C: SelectorToken = SelectorToken::new(22);
    const SELECTOR_D: SelectorToken = SelectorToken::new(23);
    const FAMILY_A: FamilyToken = FamilyToken::new(30);
    const FAMILY_B: FamilyToken = FamilyToken::new(31);

    const fn minted_coordinate(
        kind: KindToken,
        entity: EntityToken,
        token: SelectorToken,
        family: FamilyToken,
    ) -> Coordinate {
        Coordinate {
            kind,
            entity,
            selector: Selector::Minted { token, family },
        }
    }

    const fn top_coordinate(kind: KindToken, entity: EntityToken) -> Coordinate {
        Coordinate {
            kind,
            entity,
            selector: Selector::Top,
        }
    }

    const fn unminted_coordinate(
        kind: KindToken,
        entity: EntityToken,
        token: SelectorToken,
    ) -> Coordinate {
        Coordinate {
            kind,
            entity,
            selector: Selector::Unminted(token),
        }
    }

    #[test]
    fn the_same_minted_coordinate_compares_as_same() {
        let coordinate = minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A);

        assert_eq!(
            compare(Claim(coordinate), Backing(coordinate), &[]),
            CompareVerdict::Same
        );
    }

    #[test]
    fn different_minted_tokens_in_the_backing_dialect_are_provably_disjoint() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_A, SELECTOR_B],
        };

        assert_eq!(
            compare(claim, backing, &[dialect]),
            CompareVerdict::ProvablyDisjoint
        );
    }

    #[test]
    fn same_feeds_transport_only_and_collides_for_sparing() {
        let coordinate = minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A);

        assert_eq!(
            spare_pair(Claim(coordinate), Backing(coordinate), &[]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn provably_disjoint_feeds_survival_sparing() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(
            spare_pair(claim, backing, &[dialect]),
            PairSparingVerdict::Spares
        );
    }

    #[test]
    fn a_selectorless_claim_collides() {
        let claim = Claim(top_coordinate(KIND_A, ENTITY_A));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));

        assert_eq!(
            spare_pair(claim, backing, &[]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn a_selectorless_backing_collides() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let backing = Backing(top_coordinate(KIND_A, ENTITY_A));

        assert_eq!(
            spare_pair(claim, backing, &[]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn an_unminted_claim_collides() {
        let claim = Claim(unminted_coordinate(KIND_A, ENTITY_A, SELECTOR_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(
            spare_pair(claim, backing, &[dialect]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn an_unminted_backing_collides() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_A));
        let backing = Backing(unminted_coordinate(KIND_A, ENTITY_A, SELECTOR_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(
            spare_pair(claim, backing, &[dialect]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn a_cross_dialect_claim_collides() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let other_dialect = Dialect {
            family: FAMILY_B,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(
            spare_pair(claim, backing, &[other_dialect]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn the_same_selector_never_spares_itself() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_A],
        };

        assert_eq!(
            spare_pair(claim, backing, &[dialect]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn cross_kind_same_does_not_exist() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let backing = Backing(minted_coordinate(KIND_B, ENTITY_A, SELECTOR_A, FAMILY_A));

        assert_eq!(compare(claim, backing, &[]), CompareVerdict::Unknown);
    }

    #[test]
    fn top_identifies_with_nothing_including_itself() {
        let top = top_coordinate(KIND_A, ENTITY_A);

        assert_eq!(
            compare(Claim(top), Backing(top), &[]),
            CompareVerdict::Unknown
        );
    }

    #[test]
    fn silence_licenses_nothing() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));

        assert_eq!(compare(claim, backing, &[]), CompareVerdict::Unknown);
        assert_eq!(
            spare_pair(claim, backing, &[]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn every_pair_must_be_provably_disjoint() {
        let footprint = [
            Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B)),
            Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_C, FAMILY_B)),
        ];
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(
            spare_set(&footprint, BackingSet::new(backing, &[]), &[dialect]),
            SetSparingVerdict::Collides
        );
    }

    #[test]
    fn every_provably_disjoint_pair_spares_the_whole_set() {
        let footprint = [
            Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_C, FAMILY_B)),
            Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_D, FAMILY_B)),
        ];
        let first = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let rest = [Backing(minted_coordinate(
            KIND_A, ENTITY_A, SELECTOR_B, FAMILY_A,
        ))];
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_A, SELECTOR_B, SELECTOR_C, SELECTOR_D],
        };

        assert_eq!(
            spare_set(&footprint, BackingSet::new(first, &rest), &[dialect]),
            SetSparingVerdict::Spares
        );
    }

    #[test]
    fn an_unknown_member_collides_the_whole_set() {
        let footprint = [Claim(minted_coordinate(
            KIND_A, ENTITY_A, SELECTOR_C, FAMILY_B,
        ))];
        let first = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let rest = [Backing(top_coordinate(KIND_A, ENTITY_A))];
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_C],
        };

        assert_eq!(
            spare_set(&footprint, BackingSet::new(first, &rest), &[dialect]),
            SetSparingVerdict::Collides
        );
    }

    #[test]
    fn member_resolution_order_does_not_change_collision() {
        let footprint = [Claim(minted_coordinate(
            KIND_A, ENTITY_A, SELECTOR_C, FAMILY_B,
        ))];
        let known = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let unknown = Backing(top_coordinate(KIND_A, ENTITY_A));
        let known_then_unknown = [unknown];
        let unknown_then_known = [known];
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_C],
        };

        assert_eq!(
            spare_set(
                &footprint,
                BackingSet::new(known, &known_then_unknown),
                &[dialect]
            ),
            SetSparingVerdict::Collides
        );
        assert_eq!(
            spare_set(
                &footprint,
                BackingSet::new(unknown, &unknown_then_known),
                &[dialect]
            ),
            SetSparingVerdict::Collides
        );
    }

    #[test]
    fn a_compare_outcome_never_becomes_later_evidence() {
        let unknown_claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_C, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let known_claim = Claim(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(
            compare(unknown_claim, backing, &[dialect]),
            CompareVerdict::Unknown
        );
        assert_eq!(
            compare(known_claim, backing, &[dialect]),
            CompareVerdict::ProvablyDisjoint
        );
        assert_eq!(
            compare(unknown_claim, backing, &[dialect]),
            CompareVerdict::Unknown
        );
    }

    #[test]
    fn backing_sets_are_non_empty_by_construction() {
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let set = BackingSet::new(backing, &[]);

        assert_eq!(set.first, backing);
        assert!(!set.is_empty());
    }

    #[test]
    fn top_is_an_explicit_member_not_an_empty_set() {
        let footprint = [Claim(minted_coordinate(
            KIND_A, ENTITY_A, SELECTOR_B, FAMILY_B,
        ))];
        let top = Backing(top_coordinate(KIND_A, ENTITY_A));
        let set = BackingSet::new(top, &[]);

        assert_eq!(set.first, top);
        assert_eq!(spare_set(&footprint, set, &[]), SetSparingVerdict::Collides);
    }

    #[test]
    fn different_entities_collide_without_a_ratified_generator() {
        let claim = Claim(minted_coordinate(KIND_A, ENTITY_B, SELECTOR_B, FAMILY_B));
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));
        let dialect = Dialect {
            family: FAMILY_A,
            kind: KIND_A,
            selectors: &[SELECTOR_B],
        };

        assert_eq!(compare(claim, backing, &[dialect]), CompareVerdict::Unknown);
        assert_eq!(
            spare_pair(claim, backing, &[dialect]),
            PairSparingVerdict::Collides
        );
    }

    #[test]
    fn an_empty_footprint_collides_conservatively() {
        let backing = Backing(minted_coordinate(KIND_A, ENTITY_A, SELECTOR_A, FAMILY_A));

        assert_eq!(
            spare_set(&[], BackingSet::new(backing, &[]), &[]),
            SetSparingVerdict::Collides
        );
    }
}
