//! `plan::rederive` — the survival re-derivation seat: every SURVIVAL verdict the wall walk mints
//! is re-derived through the independent reference model (`dorc-sparing-reference`) before the plan
//! ships, and a disagreement DEMOTES that site to the guard/run tier.
//!
//! # The structure is demote-only, and that is the whole safety argument
//!
//! [`recheck_survival`] takes an already-minted [`SurvivalWitness`] **by value** and can only hand
//! that same value back or refuse it. Nothing here — code or test — reaches the witness mint, which
//! `rederivation_never_mints_a_survival_witness` holds lexically over this whole file, so agreement
//! licenses NOTHING new: the re-check
//! can remove a survival, never add one (`271:rul-net-quality-u-curve`'s pass condition). Every
//! approximation below therefore fails in one direction only: a model or adapter that is too
//! PERMISSIVE loses checking power, and one that is too STRICT costs an elision. Neither can
//! manufacture a survival, because the input set is production's own output.
//!
//! # What the adapter does, and does not, decide
//!
//! The model is structurally independent — its own opaque token vocabulary, one pass, no worklist,
//! zero production imports. This module is the ONLY translation layer, and it maps DATA: it reads
//! the same inputs [`crate::survival::disjoint`] reads and never calls the production comparison
//! logic (`compare` / `selector_covers` / `Relation` are unreachable from here, and the lexical
//! fence `rederivation_never_calls_the_production_compare_path` keeps them so).
//!
//! Three mapping decisions are load-bearing:
//!
//! * **Entities are fed CANONICALIZED.** The resolver is production's job (`277` §2's generator
//!   registry: each generator feeds the chokepoint its licensed evidence); the model is
//!   resolver-blind and compares entity NAMES (`300:rul-reference-entity-name-floor`). A coordinate
//!   that does not canonicalize is [`Unmappable::Unresolved`] — outside the model's domain, never
//!   silently token-equal.
//! * **Footprints are asserted NON-EMPTY.** `Footprint`'s constructors already refuse ∅
//!   (silence = wall), so ∅ reaching a sparing meet is a production FINDING, not something to
//!   normalize: it surfaces as [`Unmappable::EmptyFootprint`], which demotes here and FAILS the
//!   differential (`300:rul-reference-empty-footprint-assert`).
//! * **Backing-side mintedness is dialect membership.** Production carries no minted-bit on a
//!   selector; its only evidence is membership in the backing family's dialect for the kind, so
//!   that is what the adapter reads. The CLAIM side is handed to the model as minted-with-a-token
//!   and the model's own dialect scan decides it — a footprint claim genuinely has no minting
//!   family (`sparing-algebra`: claim/disturbs emissions never mint), so its family slot carries
//!   the reserved [`CLAIM_SIDE_FAMILY`] the relation never reads.

use dorc_core::{Dialect, EntityRef, KindId, ProviderId, SelectorId, Symbol};
use dorc_sparing_reference as model;

use crate::survival::{
    AccumulatedWall, Backing, EntityCoord, Footprint, Resolution, Resolutions, SurvivalWitness,
};

/// The family slot handed to a CLAIM-side selector. A footprint claim carries a token but no
/// minting family (`277` §3: claim/disturbs emissions never mint), and the relation reads only the
/// BACKING's family — so this value is inert by construction, and
/// `the_claim_side_family_token_is_inert` pins that. Reserved above every real family: production
/// families intern as [`Symbol`]s, so their tokens live in `0..=u32::MAX`.
const CLAIM_SIDE_FAMILY: model::FamilyToken = model::FamilyToken::new(u64::MAX);

/// Why one `(footprint, backing)` pair lies outside the model's v1 domain. Each arm is a case
/// production answers at its own conservative floor, so a refusal here never contradicts a
/// production survival — on the survival path all three are unreachable, which is exactly what
/// makes reaching one a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unmappable {
    /// A typeless-floor auto-cell kind on either side (`24L` §7 `fence-no-disjoint`): production
    /// bars it from proving disjoint before the chokepoint, and the model has no counterpart.
    AutoCell,
    /// A resolver-bearing coordinate the resolver did not canonicalize (`24F` §3a).
    Unresolved,
    /// The footprint's hit-surface was EMPTY. A finding, never a normalization.
    EmptyFootprint,
}

/// The re-check's answer for one survival (`302`-style closed outcome).
#[derive(Debug)]
pub(crate) enum Recheck {
    /// The model agreed on every crossing: the survival stands, unchanged and un-widened.
    Confirmed(SurvivalWitness),
    /// The model did not confirm some crossing: demote the site to the guard/run tier.
    Demoted(Disagreement),
}

/// Which crossing the model declined to confirm, as pure scalars (`operands-are-pure-and-capped`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Disagreement {
    /// The crossed wall's index within the accumulated walls — the ordinal the narrative reports.
    pub(crate) wall: usize,
    /// The domain refusal, when the disagreement was an unmappable input rather than a plain
    /// model-says-collide.
    pub(crate) unmappable: Option<Unmappable>,
}

/// Re-derive one minted survival through the reference model.
///
/// Consumes the witness by value and returns it untouched on agreement — the demote-only shape.
/// Every accumulated wall must SPARE in the model, exactly as production required every wall to be
/// disjoint; the first wall that does not confirms nothing and demotes the site.
pub(crate) fn recheck_survival(
    survived: SurvivalWitness,
    backing: &Backing,
    walls: &[AccumulatedWall],
    resolutions: &Resolutions,
    dialect: &Dialect,
) -> Recheck {
    for (index, wall) in walls.iter().enumerate() {
        match wall_spares(&wall.footprint, backing, resolutions, dialect) {
            Ok(true) => {}
            Ok(false) => {
                return Recheck::Demoted(Disagreement {
                    wall: index,
                    unmappable: None,
                });
            }
            Err(unmappable) => {
                return Recheck::Demoted(Disagreement {
                    wall: index,
                    unmappable: Some(unmappable),
                });
            }
        }
    }
    Recheck::Confirmed(survived)
}

/// Does the reference model spare `backing` from `footprint`? The universal meet over
/// footprint × backing-member is the MODEL's (`277` §5 set-lifting); this function only translates.
///
/// # Errors
///
/// [`Unmappable`] when the pair lies outside the model's v1 domain — every arm a case production
/// answers at its own conservative floor, so a refusal is never a contradicted survival.
pub fn wall_spares(
    footprint: &Footprint,
    backing: &Backing,
    resolutions: &Resolutions,
    dialect: &Dialect,
) -> Result<bool, Unmappable> {
    let backing_kind = backing.coord().kind();
    if resolutions.is_auto(backing_kind)
        || footprint
            .hit_surface()
            .any(|c| resolutions.is_auto(c.kind()))
    {
        return Err(Unmappable::AutoCell);
    }
    let same_kind_pair_exists = footprint.hit_surface().any(|c| c.kind() == backing_kind);
    let backing_entity = entity_for_pairing(
        resolutions,
        backing.coord(),
        same_kind_pair_exists.then_some(backing_kind),
    )?;

    let claims = claims_of(footprint, resolutions, backing_kind)?;
    let (first, rest) = members_of(backing, backing_entity, dialect);
    let vocabularies = vocabularies_of(backing, backing_kind, dialect);
    let dialects = model_dialects(backing_kind, &vocabularies);

    let verdict = model::spare_set(&claims, model::BackingSet::new(first, &rest), &dialects);
    Ok(verdict == model::SetSparingVerdict::Spares)
}

/// The model's TERNARY verdict for ONE `(footprint coordinate, backing member)` pair — the window
/// onto `277` §2's relation that [`wall_spares`]'s boolean hides, so the differential can pin the
/// consumer map itself and not merely its sparing projection. Same mapping, same inputs; the
/// difference is only that the meet is not lifted.
///
/// # Errors
///
/// [`Unmappable`], on the same terms as [`wall_spares`].
pub fn pair_verdict(
    footprint: &Footprint,
    claim: EntityCoord,
    backing: &Backing,
    member: Option<SelectorId>,
    resolutions: &Resolutions,
    dialect: &Dialect,
) -> Result<model::CompareVerdict, Unmappable> {
    let backing_kind = backing.coord().kind();
    if resolutions.is_auto(backing_kind) || resolutions.is_auto(claim.kind()) {
        return Err(Unmappable::AutoCell);
    }
    let same_kind_pair = claim.kind() == backing_kind;
    let backing_entity = entity_for_pairing(
        resolutions,
        backing.coord(),
        same_kind_pair.then_some(backing_kind),
    )?;
    let claim = model_claim(footprint, claim, resolutions, backing_kind)?;
    let member = model_member(backing, backing_entity, member, dialect);
    let vocabularies = vocabularies_of(backing, backing_kind, dialect);
    let dialects = model_dialects(backing_kind, &vocabularies);
    Ok(model::compare(claim, member, &dialects))
}

/// The `(family, kind)` vocabulary entries the model scans, borrowed from [`vocabularies_of`]'s
/// owned token vectors.
fn model_dialects(
    kind: KindId,
    vocabularies: &[(model::FamilyToken, Vec<model::SelectorToken>)],
) -> Vec<model::Dialect<'_>> {
    vocabularies
        .iter()
        .map(|(family, tokens)| model::Dialect {
            family: *family,
            kind: kind_token(kind),
            selectors: tokens,
        })
        .collect()
}

/// One footprint coordinate as a model claim.
fn model_claim(
    footprint: &Footprint,
    coord: EntityCoord,
    resolutions: &Resolutions,
    backing_kind: KindId,
) -> Result<model::Claim, Unmappable> {
    Ok(model::Claim(model::Coordinate {
        kind: kind_token(coord.kind()),
        entity: entity_for_pairing(resolutions, coord, Some(backing_kind))?,
        selector: claim_selector(footprint.selector_of(coord)),
    }))
}

/// One backing member as a model backing, on the entity token the pairing decided.
fn model_member(
    backing: &Backing,
    entity: model::EntityToken,
    selector: Option<SelectorId>,
    dialect: &Dialect,
) -> model::Backing {
    let kind = backing.coord().kind();
    model::Backing(model::Coordinate {
        kind: kind_token(kind),
        entity,
        selector: backing_selector(
            selector,
            member_family(backing, kind, selector, dialect),
            kind,
            dialect,
        ),
    })
}

/// The footprint's hit-surface as model claims. Refuses an empty surface rather than handing the
/// meet a vacuous ∀ (`300:rul-reference-empty-footprint-assert`).
fn claims_of(
    footprint: &Footprint,
    resolutions: &Resolutions,
    backing_kind: KindId,
) -> Result<Vec<model::Claim>, Unmappable> {
    let mut claims = Vec::new();
    for coord in footprint.hit_surface() {
        claims.push(model_claim(footprint, coord, resolutions, backing_kind)?);
    }
    if claims.is_empty() {
        return Err(Unmappable::EmptyFootprint);
    }
    Ok(claims)
}

/// The backing's member SET as model backings (`277` §5): the fact's own cell first, then each
/// observe-widened sibling. Non-empty by construction on both sides of the seam.
fn members_of(
    backing: &Backing,
    entity: model::EntityToken,
    dialect: &Dialect,
) -> (model::Backing, Vec<model::Backing>) {
    let kind = backing.coord().kind();
    let mut members = backing
        .member_selectors()
        .map(|selector| model_member(backing, entity, selector, dialect));
    // `member_selectors` always yields the fact's own cell first, so the fallback is unreachable;
    // spelling it as ⊤ keeps the function total without a panic path (`inv-no-throw`).
    let first = members.next().unwrap_or(model::Backing(model::Coordinate {
        kind: kind_token(kind),
        entity,
        selector: model::Selector::Top,
    }));
    (first, members.collect())
}

/// The `(family, tokens)` vocabularies the model must be able to look up: one per DISTINCT member
/// family, read straight off the production dialect map. The model does its own scan over these.
fn vocabularies_of(
    backing: &Backing,
    kind: KindId,
    dialect: &Dialect,
) -> Vec<(model::FamilyToken, Vec<model::SelectorToken>)> {
    let mut families: Vec<ProviderId> = Vec::new();
    for selector in backing.member_selectors() {
        if let Some(family) = member_family(backing, kind, selector, dialect)
            && !families.contains(&family)
        {
            families.push(family);
        }
    }
    families
        .into_iter()
        .map(|family| {
            let tokens = dialect
                .tokens(Some(family), kind)
                .iter()
                .map(|s| selector_token(*s))
                .collect();
            (family_token(family), tokens)
        })
        .collect()
}

/// One member's minting family (`277` §3 backing provenance) — the THREADED family, else the
/// `sole_family` reverse-lookup floor. The same input `disjoint` computes.
fn member_family(
    backing: &Backing,
    kind: KindId,
    selector: Option<SelectorId>,
    dialect: &Dialect,
) -> Option<ProviderId> {
    backing
        .family()
        .or_else(|| selector.and_then(|s| dialect.sole_family(kind, s)))
}

/// A CLAIM-side selector state. A token is handed over as minted-with-a-token and the model's own
/// dialect scan decides whether it is in the backing family's vocabulary; the family slot is the
/// inert [`CLAIM_SIDE_FAMILY`].
fn claim_selector(selector: Option<SelectorId>) -> model::Selector {
    match selector {
        None => model::Selector::Top,
        Some(s) => model::Selector::Minted {
            token: selector_token(s),
            family: CLAIM_SIDE_FAMILY,
        },
    }
}

/// A BACKING-side selector state. Mintedness is membership in the member family's dialect for the
/// kind — production's only evidence that a token was minted at all.
fn backing_selector(
    selector: Option<SelectorId>,
    family: Option<ProviderId>,
    kind: KindId,
    dialect: &Dialect,
) -> model::Selector {
    match (selector, family) {
        (None, _) => model::Selector::Top,
        (Some(s), Some(f)) if dialect.tokens(Some(f), kind).contains(&s) => {
            model::Selector::Minted {
                token: selector_token(s),
                family: family_token(f),
            }
        }
        (Some(s), _) => model::Selector::Unminted(selector_token(s)),
    }
}

/// The entity token a coordinate contributes to the relation.
///
/// The kind fence fires FIRST in both implementations, so a coordinate whose kind differs from the
/// one it will be paired against never has its entity read — `paired_against` carries that
/// knowledge (`None`, or a kind that does not match, means the slot is inert). For such a slot the
/// raw interned entity travels: it is the honest un-resolved datum, and it is never compared.
///
/// Where the entity IS read, it must be the resolver's CANONICAL output or nothing: a coordinate
/// the resolver could not canonicalize is [`Unmappable::Unresolved`], because the model decides the
/// entity axis by token inequality and feeding it a raw token there would manufacture separation
/// out of a resolver gap (`never-derive-separation`).
fn entity_for_pairing(
    resolutions: &Resolutions,
    coord: EntityCoord,
    paired_against: Option<KindId>,
) -> Result<model::EntityToken, Unmappable> {
    if paired_against != Some(coord.kind()) {
        return Ok(entity_token(coord.entity()));
    }
    match resolutions.canonicalize(coord) {
        Resolution::Canonical(canonical) => Ok(entity_token(canonical.entity())),
        Resolution::MayAlias(_) => Err(Unmappable::Unresolved),
    }
}

fn symbol_token(symbol: Symbol) -> u64 {
    u64::from(symbol.as_u32())
}

fn kind_token(kind: KindId) -> model::KindToken {
    model::KindToken::new(symbol_token(kind.0))
}

fn selector_token(selector: SelectorId) -> model::SelectorToken {
    model::SelectorToken::new(symbol_token(selector.0))
}

fn family_token(family: ProviderId) -> model::FamilyToken {
    model::FamilyToken::new(symbol_token(family.0))
}

/// `EntityRef` → an opaque token, injectively: the singleton takes 0 and every operand symbol
/// shifts one clear of it. A no-wildcard match, so a future variant must visit this seam.
fn entity_token(entity: EntityRef) -> model::EntityToken {
    match entity {
        EntityRef::Singleton => model::EntityToken::new(0),
        EntityRef::Operand(token) => {
            model::EntityToken::new(symbol_token(token.0).saturating_add(1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LeafId;
    use crate::survival::{AccumulatedWall, WallVerdict, wall_verdict};
    use dorc_core::{Context, FactKey, Interner, OpaqueToken};

    /// A survival witness obtained the ONLY way one legitimately exists: from production's own wall
    /// walk, over a wall the algebra clears. Deliberately never the witness mint — this whole
    /// module is lexically fenced against the mint (`rederivation_never_mints_a_survival_witness`,
    /// in `tests/sparing_differential.rs`), and its tests hold to the same rule its code does.
    fn minted_witness(backing: &Backing, interner: &mut Interner) -> SurvivalWitness {
        let elsewhere = AccumulatedWall {
            wall_leaf: LeafId(0),
            footprint: Footprint::authored(
                interner.intern("elsewhere"),
                vec![EntityCoord::new(
                    KindId(interner.intern("com.dorc.Elsewhere")),
                    EntityRef::Singleton,
                )],
            )
            .expect("non-empty footprint"),
        };
        match wall_verdict(
            false,
            &[elsewhere],
            backing,
            &Resolutions::none(),
            &Dialect::empty(),
        ) {
            WallVerdict::Survived(witness) => witness,
            other => panic!("a cross-kind wall must yield a survival, got {other:?}"),
        }
    }

    fn fact(interner: &mut Interner, entity: &str, selector: &str) -> FactKey {
        FactKey {
            kind: KindId(interner.intern("com.dorc.Kind")),
            entity: EntityRef::Operand(OpaqueToken(interner.intern(entity))),
            selector: SelectorId(interner.intern(selector)),
            context: Context::HostDefault,
        }
    }

    #[test]
    fn agreement_hands_back_the_very_witness_it_was_given() {
        // Agreement licenses NOTHING new (`271:rul-net-quality-u-curve`'s pass condition): the
        // confirmed witness is the input witness, un-widened -- no crossing gained, none lost.
        let mut interner = Interner::default();
        let backed = fact(&mut interner, "nginx", "installed");
        let coord = EntityCoord::new(backed.kind, backed.entity);
        let backing = Backing::of_fact(backed);
        let witness = minted_witness(&backing, &mut interner);
        let crossings_in = witness.crossings().len();
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(1),
            footprint: Footprint::authored(
                interner.intern("apt-get"),
                vec![EntityCoord::new(
                    KindId(interner.intern("com.dorc.Other")),
                    EntityRef::Singleton,
                )],
            )
            .expect("non-empty footprint"),
        }];

        match recheck_survival(
            witness,
            &backing,
            &walls,
            &Resolutions::none(),
            &Dialect::empty(),
        ) {
            Recheck::Confirmed(confirmed) => {
                assert_eq!(confirmed.backing(), coord);
                assert_eq!(
                    confirmed.crossings().len(),
                    crossings_in,
                    "the re-check returns the witness unchanged; it never adds a crossing"
                );
            }
            Recheck::Demoted(d) => panic!("a cross-kind wall must confirm, got {d:?}"),
        }
    }

    #[test]
    fn a_model_collide_demotes_and_names_the_wall() {
        // The other direction of the same structure: the re-check's only other answer is a demote,
        // which the caller turns into `Disposition::Run`.
        let mut interner = Interner::default();
        let backed = fact(&mut interner, "nginx", "installed");
        let coord = EntityCoord::new(backed.kind, backed.entity);
        let backing = Backing::of_fact(backed);
        let witness = minted_witness(&backing, &mut interner);
        let provider = interner.intern("apt-get");
        let other = EntityCoord::new(
            KindId(interner.intern("com.dorc.Other")),
            EntityRef::Singleton,
        );
        let walls = [
            AccumulatedWall {
                wall_leaf: LeafId(1),
                footprint: Footprint::authored(provider, vec![other]).expect("non-empty footprint"),
            },
            AccumulatedWall {
                wall_leaf: LeafId(2),
                footprint: Footprint::authored(provider, vec![coord]).expect("non-empty footprint"),
            },
        ];

        match recheck_survival(
            witness,
            &backing,
            &walls,
            &Resolutions::none(),
            &Dialect::empty(),
        ) {
            Recheck::Demoted(d) => {
                assert_eq!(
                    d.wall, 1,
                    "the SECOND wall is the one the model collides on"
                );
                assert_eq!(d.unmappable, None, "a plain collide, not a domain refusal");
            }
            Recheck::Confirmed(_) => panic!("a same-cell wall must demote"),
        }
    }

    #[test]
    fn an_unmappable_input_demotes_rather_than_confirming() {
        // Every adapter approximation fails toward demote: an input outside the model's domain can
        // cost an elision, never license one.
        let mut interner = Interner::default();
        let backed = fact(&mut interner, "nginx", "installed");
        let coord = EntityCoord::new(backed.kind, backed.entity);
        let backing = Backing::of_fact(backed);
        let witness = minted_witness(&backing, &mut interner);
        let mut resolutions = Resolutions::none();
        resolutions.add_auto_kind(backed.kind);
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(1),
            footprint: Footprint::authored(interner.intern("apt-get"), vec![coord])
                .expect("non-empty footprint"),
        }];

        match recheck_survival(witness, &backing, &walls, &resolutions, &Dialect::empty()) {
            Recheck::Demoted(d) => assert_eq!(d.unmappable, Some(Unmappable::AutoCell)),
            Recheck::Confirmed(_) => panic!("an auto-cell backing must never confirm a survival"),
        }
    }
}
