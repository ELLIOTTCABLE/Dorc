//! The sparing-algebra **internal differential** (`300:lane-sparing-rederivation`): production's
//! `survival::disjoint` (through `core`'s `compare`/`selector_covers` chokepoints) against the
//! structurally independent `dorc-sparing-reference` model, over seeded, permuted inputs.
//!
//! # Why a differential and not more unit tests
//!
//! The two implementations were written under DIFFERENT constraints — production is a
//! canonicalizing, provenance-threading walk over interned handles; the model is one pass over
//! opaque tokens, authored from the ratified English with no sight of the code. That structural
//! asymmetry is the whole value (`300` §2's charter: never authorial lineage). Agreement over a
//! generated input space is evidence no unit test buys, because neither author chose the cases.
//!
//! # A disagreement is a FINDING, never something to reconcile here
//!
//! `law-never-weaken-the-question`: if this file goes red, the repair is NOT to nudge either side
//! into agreement. It is a production bug, a model bug, or a law that does not decide the case —
//! and which of the three is the conductor's call. The failure messages therefore print the exact
//! operands, and the seed replays the trial bit-for-bit.
//!
//! # What is NOT differentially covered (stated so it is not mistaken for coverage)
//!
//! The adapter derives BACKING-side mintedness from dialect membership, because that is
//! production's only evidence that a token was ever minted; so that one conjunct of `277` §3 is
//! translated rather than re-decided. Everything else — the ⊤-on-either-side rule, no-self-sparing,
//! the claim-side dialect scan, the kind fence, the entity name-floor, and the universal meet over
//! both sets — is the model's own answer.

// A generator harness over fixed-size arrays it owns: the no-panic lints guard untrusted-INPUT
// paths, and this file's only "input" is its own seeds. A panic here is a red test, which is the
// point.
#![expect(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    reason = "seeded generator over self-owned fixed arrays; a panic is a test failure, not an \
              untrusted-input path"
)]

use std::collections::BTreeSet;

use dorc_core::{
    Context, Dialect, EntityRef, FactKey, Interner, KindId, OpaqueToken, ProviderId, SelectorId,
    Symbol,
};

use dorc_hostsim::Lcg;
use dorc_plan::rederive::{self, Unmappable};
use dorc_plan::{Backing, DisjointOutcome, EntityCoord, Footprint, Resolutions, disjoint};
use dorc_sparing_reference as model;

/// How many seeds each differential drives. Cheap (pure in-memory algebra, no I/O), so the count is
/// set by coverage of the input space rather than by budget.
const TRIALS: u64 = 4_000;

/// The interned universe every trial draws from — deliberately TINY, so collisions between the
/// axes are frequent and the interesting cells (same kind + same entity + sibling selectors) come
/// up constantly rather than by luck.
struct Universe {
    kinds: [KindId; 2],
    entities: [EntityRef; 3],
    selectors: [SelectorId; 3],
    families: [ProviderId; 2],
    provider: Symbol,
}

impl Universe {
    fn new(interner: &mut Interner) -> Self {
        let kinds = [
            KindId(interner.intern("com.dorc.KindOne")),
            KindId(interner.intern("com.dorc.KindTwo")),
        ];
        let entities = [
            EntityRef::Operand(OpaqueToken(interner.intern("nginx"))),
            EntityRef::Operand(OpaqueToken(interner.intern("curl"))),
            EntityRef::Singleton,
        ];
        let selectors = [
            SelectorId(interner.intern("installed")),
            SelectorId(interner.intern("enabled")),
            SelectorId(interner.intern("active")),
        ];
        let families = [
            ProviderId(interner.intern("aptget")),
            ProviderId(interner.intern("systemctl")),
        ];
        let provider = interner.intern("wall-provider");
        Self {
            kinds,
            entities,
            selectors,
            families,
            provider,
        }
    }

    fn kind(&self, rng: &mut Lcg) -> KindId {
        self.kinds[pick(rng, self.kinds.len())]
    }

    fn entity(&self, rng: &mut Lcg) -> EntityRef {
        self.entities[pick(rng, self.entities.len())]
    }

    fn selector(&self, rng: &mut Lcg) -> SelectorId {
        self.selectors[pick(rng, self.selectors.len())]
    }

    /// A selector cell, or `None` for the whole-entity ⊤ form — drawn often enough that the
    /// `279f:fix-spare-top-backing` cell (a ⊤ backing under a minted claim) is well covered.
    fn maybe_selector(&self, rng: &mut Lcg) -> Option<SelectorId> {
        if rng.chance(1, 4) {
            None
        } else {
            Some(self.selector(rng))
        }
    }

    fn family(&self, rng: &mut Lcg) -> ProviderId {
        self.families[pick(rng, self.families.len())]
    }
}

fn pick(rng: &mut Lcg, len: usize) -> usize {
    usize::try_from(rng.below(len as u64)).unwrap_or(0)
}

/// One generated trial: the four inputs `disjoint` takes, plus the seed that reproduces them.
struct Trial {
    footprint: Footprint,
    backing: Backing,
    resolutions: Resolutions,
    dialect: Dialect,
}

/// Draw a dialect: each `(family, kind)` mints a random subset of the universe's selectors. An
/// empty draw is the `empty-world-byte-identical` floor and is deliberately reachable.
fn draw_dialect(rng: &mut Lcg, u: &Universe) -> Dialect {
    let mut dialect = Dialect::empty();
    for family in u.families {
        for kind in u.kinds {
            for selector in u.selectors {
                if rng.chance(1, 2) {
                    dialect.mint(family, kind, selector);
                }
            }
        }
    }
    dialect
}

/// Draw the resolver world: usually the honest token-equality floor, sometimes a resolver-bearing
/// kind (with recorded canonicals, absences, and danglings), sometimes an auto-cell kind.
fn draw_resolutions(rng: &mut Lcg, u: &Universe) -> Resolutions {
    let mut resolutions = Resolutions::none();
    if rng.chance(1, 3) {
        let kind = u.kind(rng);
        resolutions.add_resolver_kind(kind);
        for entity in u.entities {
            let coord = EntityCoord::new(kind, entity);
            if rng.chance(1, 5) {
                resolutions.record_dangling(coord);
            } else if rng.chance(3, 4) {
                // Collapse both operands onto one canonical often enough that the aliasing closure
                // (two names, one referent) is exercised, not just identity resolution.
                let canonical = if rng.chance(1, 2) {
                    entity
                } else {
                    u.entities[0]
                };
                resolutions.record(coord, canonical);
            }
        }
    }
    if rng.chance(1, 8) {
        resolutions.add_auto_kind(u.kind(rng));
    }
    resolutions
}

fn draw_footprint(rng: &mut Lcg, u: &Universe) -> Footprint {
    let count = 1 + pick(rng, 3);
    let mut coords = Vec::new();
    let mut selectors = Vec::new();
    for _ in 0..count {
        let coord = EntityCoord::new(u.kind(rng), u.entity(rng));
        coords.push(coord);
        selectors.push((coord, u.maybe_selector(rng)));
    }
    let own = if rng.chance(1, 3) {
        Some(EntityCoord::new(u.kind(rng), u.entity(rng)))
    } else {
        None
    };
    let mut footprint = Footprint::authored(u.provider, coords)
        .expect("a drawn footprint always carries >= 1 coordinate")
        .with_own(own);
    for (coord, selector) in selectors {
        if let Some(selector) = selector {
            footprint.set_selector(coord, selector);
        }
    }
    footprint
}

fn draw_backing(rng: &mut Lcg, u: &Universe) -> Backing {
    let fact = FactKey {
        kind: u.kind(rng),
        entity: u.entity(rng),
        selector: u.selector(rng),
        context: Context::HostDefault,
    };
    let family = if rng.chance(1, 2) {
        Some(u.family(rng))
    } else {
        None
    };
    let mut observed = BTreeSet::new();
    for selector in u.selectors {
        if rng.chance(1, 5) {
            observed.insert(selector);
        }
    }
    if rng.chance(1, 3) {
        Backing::of_fact(fact)
    } else {
        Backing::widened(fact, family, observed)
    }
}

fn draw_trial(seed: u64, u: &Universe) -> Trial {
    let mut rng = Lcg::new(seed);
    Trial {
        dialect: draw_dialect(&mut rng, u),
        resolutions: draw_resolutions(&mut rng, u),
        footprint: draw_footprint(&mut rng, u),
        backing: draw_backing(&mut rng, u),
    }
}

/// The census of production outcomes a run saw — the non-vacuity control. A differential that only
/// ever drew one cell proves nothing, and a generator can silently drift into that (the `24C`
/// LCG-thinning lesson), so the run asserts every outcome was reached.
#[derive(Default)]
struct Census {
    /// Both sides spared.
    disjoint: u32,
    /// Both sides collided, with the algebra actually consulted on both.
    collide: u32,
    /// Production answered at a floor OUTSIDE the model's domain (an auto-cell kind, an
    /// unresolvable coordinate), and the adapter refused. Counted apart from `collide` because it
    /// is agreement that the algebra never ran, not agreement about what the algebra says.
    out_of_domain: u32,
}

#[test]
fn the_reference_model_and_production_agree_on_every_sparing_verdict() {
    let mut interner = Interner::default();
    let u = Universe::new(&mut interner);
    let mut census = Census::default();

    for seed in 0..TRIALS {
        let t = draw_trial(seed, &u);
        let production = disjoint(&t.footprint, &t.backing, &t.resolutions, &t.dialect);
        let reference = rederive::wall_spares(&t.footprint, &t.backing, &t.resolutions, &t.dialect);

        // `300:rul-reference-empty-footprint-assert`: production builds footprints non-empty by
        // construction, so ∅ reaching the meet is a production FINDING and this test says so
        // rather than letting the adapter normalize it away.
        assert_ne!(
            reference,
            Err(Unmappable::EmptyFootprint),
            "seed {seed}: an EMPTY footprint reached the sparing meet -- production is supposed to \
             make that unrepresentable (`Footprint::authored` refuses an empty emission). This is \
             a finding about production, not about the model."
        );

        match (&production, reference) {
            (DisjointOutcome::Disjoint(_), Ok(true)) => census.disjoint += 1,
            (DisjointOutcome::Hit { .. } | DisjointOutcome::MayAlias(_), Ok(false)) => {
                census.collide += 1;
            }
            // The model's domain is narrower than production's floor set: an auto-cell kind and an
            // unresolvable coordinate are cases production answers WITHOUT the algebra. Both are
            // collide-side there, so a refusal here is agreement, not a gap.
            (
                DisjointOutcome::Hit { .. } | DisjointOutcome::MayAlias(_),
                Err(Unmappable::AutoCell | Unmappable::Unresolved),
            ) => census.out_of_domain += 1,
            (DisjointOutcome::Disjoint(_), other) => panic!(
                "seed {seed}: DISAGREEMENT -- production SPARED and the reference model did not \
                 ({other:?}). This is the dangerous direction (a survival production would license \
                 that the independent model refuses). Do not reconcile: capture and report."
            ),
            (other, Ok(true)) => panic!(
                "seed {seed}: DISAGREEMENT -- the reference model SPARED and production did not \
                 ({other:?}). Do not reconcile: capture and report."
            ),
            (other, refusal) => panic!(
                "seed {seed}: DISAGREEMENT -- production {other:?} against reference {refusal:?}. \
                 Do not reconcile: capture and report."
            ),
        }
    }

    assert!(
        census.disjoint > 0 && census.collide > 0 && census.out_of_domain > 0,
        "the generator went vacuous: disjoint={} collide={} out_of_domain={} -- every production \
         outcome must be reached or the agreement above proves nothing",
        census.disjoint,
        census.collide,
        census.out_of_domain
    );
}

#[test]
fn the_reference_model_and_production_agree_on_the_ternary_relation() {
    // The sparing projection above collapses `same` and `unknown` into one collide answer. This
    // drives the pair-level relation instead (`277` §2's consumer map): a footprint of ONE
    // coordinate against ONE backing member, so the model's ternary is observable and the
    // consumer map -- provably-disjoint spares, everything else collides -- is pinned directly.
    let mut interner = Interner::default();
    let u = Universe::new(&mut interner);
    let mut saw_same = 0_u32;
    let mut saw_disjoint = 0_u32;
    let mut saw_collide_without_identity = 0_u32;
    let mut saw_out_of_domain = 0_u32;

    for seed in 0..TRIALS {
        let mut rng = Lcg::new(seed.wrapping_mul(2_654_435_761));
        let dialect = draw_dialect(&mut rng, &u);
        let resolutions = draw_resolutions(&mut rng, &u);

        // A single-coordinate footprint against a single-member backing: one pair, one verdict.
        let claim = EntityCoord::new(u.kind(&mut rng), u.entity(&mut rng));
        let mut footprint = Footprint::authored(u.provider, vec![claim])
            .expect("one coordinate is a non-empty footprint");
        if let Some(selector) = u.maybe_selector(&mut rng) {
            footprint.set_selector(claim, selector);
        }
        let member = u.selector(&mut rng);
        let fact = FactKey {
            kind: u.kind(&mut rng),
            entity: u.entity(&mut rng),
            selector: member,
            context: Context::HostDefault,
        };
        let backing = if rng.chance(1, 2) {
            Backing::of_fact(fact)
        } else {
            Backing::widened(fact, Some(u.family(&mut rng)), BTreeSet::new())
        };

        let production = disjoint(&footprint, &backing, &resolutions, &dialect);
        let reference = rederive::pair_verdict(
            &footprint,
            claim,
            &backing,
            Some(member),
            &resolutions,
            &dialect,
        );

        match (&production, reference) {
            // provably-disjoint feeds survival sparing, on BOTH sides.
            (DisjointOutcome::Disjoint(_), Ok(model::CompareVerdict::ProvablyDisjoint)) => {
                saw_disjoint += 1;
            }
            // The collide side. Production's one `Overlaps` reading covers the model's `Same`
            // (identical minted tokens) and its `Unknown` (no licensing generator); both collide,
            // which is exactly why the relation is ternary and the consumer map is welded.
            (DisjointOutcome::Hit { .. }, Ok(model::CompareVerdict::Same)) => saw_same += 1,
            (
                DisjointOutcome::Hit { .. } | DisjointOutcome::MayAlias(_),
                Ok(model::CompareVerdict::Unknown),
            ) => saw_collide_without_identity += 1,
            (
                DisjointOutcome::Hit { .. } | DisjointOutcome::MayAlias(_),
                Err(Unmappable::AutoCell | Unmappable::Unresolved),
            ) => saw_out_of_domain += 1,
            (production, reference) => panic!(
                "seed {seed}: TERNARY DISAGREEMENT -- production {production:?} against reference \
                 {reference:?}. Do not reconcile: capture and report."
            ),
        }
    }

    assert!(
        saw_same > 0
            && saw_disjoint > 0
            && saw_collide_without_identity > 0
            && saw_out_of_domain > 0,
        "the ternary generator went vacuous: same={saw_same} disjoint={saw_disjoint} \
         other-collide={saw_collide_without_identity} out-of-domain={saw_out_of_domain}"
    );
}

#[test]
fn a_deliberately_broken_reading_of_the_algebra_is_caught() {
    // The differential's own non-vacuity control: a MUTANT of the sparing rule -- the pre-`279f`
    // reading, where a ⊤/whole-entity BACKING was spared by any dialect-member claim -- must be
    // visibly rejected by the reference model. If this ever passes, the differential above is
    // asserting nothing about the ⊤ cell (`anti-masking-tests`).
    let mut interner = Interner::default();
    let u = Universe::new(&mut interner);
    let mut dialect = Dialect::empty();
    dialect.mint(u.families[0], u.kinds[0], u.selectors[0]);
    dialect.mint(u.families[0], u.kinds[0], u.selectors[1]);

    let coord = EntityCoord::new(u.kinds[0], u.entities[0]);
    let mut footprint = Footprint::authored(u.provider, vec![coord]).expect("non-empty footprint");
    footprint.set_selector(coord, u.selectors[1]);

    let top_backing = Backing::widened(
        FactKey {
            kind: u.kinds[0],
            entity: u.entities[0],
            selector: u.selectors[0],
            context: Context::HostDefault,
        },
        Some(u.families[0]),
        BTreeSet::new(),
    );
    // The honest cell: two minted, distinct siblings under one family's dialect -> spared.
    assert_eq!(
        rederive::wall_spares(&footprint, &top_backing, &Resolutions::none(), &dialect),
        Ok(true),
        "precondition: the reference model does spare a genuine sibling cell"
    );

    // The mutant cell: the SAME claim against a backing whose selector was never minted for that
    // family. The pre-`279f` reading spared it; the model must not.
    let unminted_backing = Backing::widened(
        FactKey {
            kind: u.kinds[0],
            entity: u.entities[0],
            selector: u.selectors[2],
            context: Context::HostDefault,
        },
        Some(u.families[0]),
        BTreeSet::new(),
    );
    assert_eq!(
        rederive::wall_spares(
            &footprint,
            &unminted_backing,
            &Resolutions::none(),
            &dialect
        ),
        Ok(false),
        "an unminted backing token must collide -- the 279f under-execution class"
    );
}

/// The re-derivation adapter's own source. Read from a NEIGHBOUR file so the needle literals below
/// never live inside the text they police (the `spanless_mint_allow_list_is_exact` lesson: a fence
/// spelled in the file it scans trips on itself, and the usual repair — cutting the file at its
/// first `#[cfg(test)]` — is exactly the blindness the certifier lane had to undo).
fn adapter_source() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("rederive.rs");
    std::fs::read_to_string(path).expect("the adapter's source is readable")
}

#[test]
fn rederivation_never_mints_a_survival_witness() {
    // The demote-only structure, held lexically over the WHOLE file, tests included: the re-check
    // takes a witness by value and hands that same value back, so the only way it could ever
    // manufacture a survival is by calling the mint. If this needle appears, agreement has stopped
    // being inert and the re-check has become a second licensing authority.
    let source = adapter_source();
    assert!(
        source.contains("model::spare_set"),
        "positive control: the scan must actually be reading the adapter"
    );
    assert!(
        !source.contains("SurvivalWitness::new"),
        "the re-derivation seat must never mint a survival -- it may only pass back the one it was \
         handed, or refuse it"
    );
}

#[test]
fn rederivation_never_calls_the_production_compare_path() {
    // The checker's value is STRUCTURAL difference: an adapter reaching for production's own
    // comparison logic would be checking that code against itself. The adapter maps DATA only.
    let source = adapter_source();
    assert!(
        source.contains("dorc_core::"),
        "positive control: the adapter does import from core"
    );
    for needle in [
        "dorc_core::compare",
        "dorc_core::selector_covers",
        "dorc_core::selector_identifies",
        "Relation::",
        "survival::disjoint(",
        "crate::disjoint",
    ] {
        assert!(
            !source.contains(needle),
            "`{needle}` reaches the re-derivation adapter -- the model must not be fed \
             production's own comparison logic"
        );
    }
    let imports = source
        .split_once("use dorc_core::{")
        .and_then(|(_, rest)| rest.split_once("};"))
        .map(|(list, _)| list.to_owned())
        .expect("the adapter's core import list is a braced group");
    for name in [
        "compare",
        "selector_covers",
        "selector_identifies",
        "Relation",
    ] {
        assert!(
            !imports.contains(name),
            "`{name}` is imported into the adapter from core -- the compare path must stay \
             unreachable there"
        );
    }
}

#[test]
fn the_claim_side_family_token_is_inert() {
    // The adapter hands a claim-side selector a RESERVED family token, because a footprint claim
    // genuinely has no minting family. That choice must not be load-bearing: the relation reads
    // only the BACKING's family, so re-running the model with a different claim-side family must
    // give the same answer. Asserted against the model directly, where the token is visible.
    let kind = model::KindToken::new(1);
    let entity = model::EntityToken::new(2);
    let minted = model::SelectorToken::new(3);
    let backing_token = model::SelectorToken::new(4);
    let backing_family = model::FamilyToken::new(5);
    let vocabulary = [minted, backing_token];
    let dialects = [model::Dialect {
        family: backing_family,
        kind,
        selectors: &vocabulary,
    }];
    let backing = model::Backing(model::Coordinate {
        kind,
        entity,
        selector: model::Selector::Minted {
            token: backing_token,
            family: backing_family,
        },
    });
    let claim_under = |family| {
        model::Claim(model::Coordinate {
            kind,
            entity,
            selector: model::Selector::Minted {
                token: minted,
                family,
            },
        })
    };

    let reserved = model::compare(
        claim_under(model::FamilyToken::new(u64::MAX)),
        backing,
        &dialects,
    );
    let arbitrary = model::compare(claim_under(model::FamilyToken::new(9)), backing, &dialects);
    let same_as_backing = model::compare(claim_under(backing_family), backing, &dialects);

    assert_eq!(reserved, model::CompareVerdict::ProvablyDisjoint);
    assert_eq!(reserved, arbitrary);
    assert_eq!(reserved, same_as_backing);
}
