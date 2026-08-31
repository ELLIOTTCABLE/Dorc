//! What the reconstruction must hold, whatever it was built from.
//!
//! Driven through REAL published documents rather than hand-built models, for the reason the
//! fixture module gives: a hand-built model agrees with itself.
//!
//! The censuses here are STRUCTURAL, never lexical (`lexical-fences-are-human-ack-instruments`):
//! totality is a no-wildcard match plus a permutation check over the model's own flat population,
//! and determinism is measured by re-deriving and comparing, not by grepping for a collection type.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "assertions beside the cases, where the in-tests allowance does not reach them"
)]

mod support;

use std::collections::BTreeSet;

use dorc_receipt::report::{
    CurrentSourceReading, RequestedAddress, SiblingState, SourceObservation,
};
use dorc_why::known::{CantTell, CarrierAbsence, Held, Known, WithholdReason};
use dorc_why::recorded::{Rooted, reconstruct};
use dorc_why::{
    CorrelationFact, Datum, Delivery, FamilyName, IdentityFact, Moment, NegativeSpace, Payload,
    Reconstruction, Separability, Speaker, StateFact, Subject, VoiceSet,
};

use support::{BOOK, Shape, facts, published, shaped};

/// A structural projection of one datum, for comparison and for the ordering gates.
///
/// It names a sealed value's CLASS and LENGTH and never its bytes: comparing two reconstructions by
/// their content would be exactly the equality `report::value` withholds, and a fingerprint that
/// leaked bytes into a test failure would be a display route around the encoder.
fn fingerprint(datum: &Datum) -> String {
    format!(
        "{}|{}|{}|{}",
        speaker_of(datum),
        world_of(datum),
        subject_of(datum),
        payload_of(datum)
    )
}

fn speaker_of(datum: &Datum) -> String {
    match datum.speaker() {
        Known::Knowable(Held::Present(speaker)) => {
            format!("{:?}/{}", speaker.act(), voices_of(speaker))
        }
        other => absence_of(other),
    }
}

fn voices_of(speaker: &Speaker) -> &'static str {
    match speaker.voices() {
        VoiceSet::Mine => "mine",
        VoiceSet::One(_) => "one",
        VoiceSet::Committee {
            separability: Separability::Separable,
            ..
        } => "committee-separable",
        VoiceSet::Committee {
            separability: Separability::Inseparable,
            ..
        } => "committee-inseparable",
    }
}

fn world_of(datum: &Datum) -> String {
    let moment = match datum.world().moment() {
        Known::Knowable(Held::Present(Moment::Filed(order))) => format!("filed:{order}"),
        Known::Knowable(Held::Present(Moment::Undated)) => "undated".to_owned(),
        other => absence_of(other),
    };
    let host = absence_or(datum.world().host(), |_| "host".to_owned());
    let lineage = absence_or(datum.world().lineage(), |_| "lineage".to_owned());
    format!("{moment},{host},{lineage}")
}

fn subject_of(datum: &Datum) -> String {
    absence_or(datum.subject(), |subject| match subject {
        Subject::Site(site) => format!("site:{}", site.leaf().get()),
        Subject::Source(source) => format!("source:{}", source.get()),
        Subject::Stage { site, index } => format!("stage:{}:{index}", site.leaf().get()),
        Subject::Document(document) => format!("document:{}", document.hex()),
        Subject::Address(address) => format!("address:{}:{}", address.source.get(), address.line),
        Subject::Family(family) => format!("family:{}", family.key()),
    })
}

/// EVERY payload kind, matched without a wildcard — the census that makes a new kind a compile
/// error here rather than a row that silently never reaches output.
fn payload_of(datum: &Datum) -> String {
    absence_or(datum.payload(), |payload| match payload {
        Payload::Decision(disposition) => format!("decision:{disposition:?}"),
        Payload::Influence(grade) => format!("influence:{grade:?}"),
        Payload::Identity(identity) => format!("identity:{}", identity_of(identity)),
        Payload::State(state) => format!("state:{}", state_of(*state)),
        // Class and LENGTH, never bytes.
        Payload::Text(value) => format!("text:{:?}:{}", value.class(), value.len()),
        Payload::Correlation(correlation) => format!("correlation:{}", correlation_of(correlation)),
        Payload::NegativeSpace(space) => format!("negative:{}", negative_of(*space)),
    })
}

fn identity_of(identity: &IdentityFact) -> String {
    match identity {
        IdentityFact::Document(document) => format!("document:{}", document.hex()),
        IdentityFact::Species(species) => format!("species:{}", species.token()),
        IdentityFact::Digest(digest) => format!("digest:{digest}"),
        IdentityFact::Bytes(bytes) => format!("bytes:{bytes}"),
        IdentityFact::Count(count) => format!("count:{count}"),
        IdentityFact::UncarriedSpecies(species) => format!("uncarried:{species:?}"),
        IdentityFact::SourceClass(class) => format!("class:{class:?}"),
        IdentityFact::Ast(ast) => format!("ast:{ast}"),
    }
}

fn state_of(state: StateFact) -> String {
    match state {
        StateFact::Authentication(value) => format!("auth:{value:?}"),
        StateFact::Projection(value) => format!("projection:{value:?}"),
        StateFact::Detail(value) => format!("detail:{value:?}"),
        StateFact::Closure(value) => format!("closure:{value:?}"),
        StateFact::CurrentSource(value) => format!("current:{value:?}"),
        StateFact::ReDerivation(value) => format!("rederivation:{value:?}"),
    }
}

fn correlation_of(correlation: &CorrelationFact) -> String {
    match correlation {
        CorrelationFact::PlanToIntent { .. } => "plan-to-intent".to_owned(),
        CorrelationFact::IntentToOutcome { .. } => "intent-to-outcome".to_owned(),
        CorrelationFact::Finding(kind) => format!("finding:{kind:?}"),
    }
}

fn negative_of(space: NegativeSpace) -> String {
    format!("{:?}:{}", space.kind, space.family.key())
}

/// The wrapper's own states, matched without a wildcard: every absence a slot can carry has a
/// distinct spelling here, so laundering one into another moves a byte a test can see.
fn absence_of<T>(known: &Known<T>) -> String {
    match known {
        Known::Knowable(Held::Present(_)) => "present".to_owned(),
        Known::Knowable(Held::AbsentFromCarrier(CarrierAbsence::RunHeldNoValue)) => {
            "absent:run-held-no-value".to_owned()
        }
        Known::Knowable(Held::AbsentFromCarrier(CarrierAbsence::ProjectionUncollected)) => {
            "absent:projection-uncollected".to_owned()
        }
        Known::Knowable(Held::AbsentFromCarrier(CarrierAbsence::ReportApiLacks)) => {
            "absent:report-api-lacks".to_owned()
        }
        Known::Knowable(Held::Withheld(WithholdReason::PlainProjection)) => {
            "withheld:plain".to_owned()
        }
        Known::Knowable(Held::Withheld(WithholdReason::BoundRefused)) => {
            "withheld:bound".to_owned()
        }
        Known::Knowable(Held::Withheld(WithholdReason::RegionUnavailable)) => {
            "withheld:region".to_owned()
        }
        Known::Knowable(Held::Withheld(WithholdReason::EncoderGated)) => {
            "withheld:encoder".to_owned()
        }
        Known::Knowable(Held::CouldNotTell(CantTell::ComparisonNotMade)) => {
            "cant-tell:no-comparison".to_owned()
        }
        Known::Knowable(Held::CouldNotTell(CantTell::Truncated)) => {
            "cant-tell:truncated".to_owned()
        }
        Known::KnowableNYI => "nyi".to_owned(),
        Known::Unknowable => "unknowable".to_owned(),
    }
}

fn absence_or<T>(known: &Known<T>, present: impl Fn(&T) -> String) -> String {
    known.value().map_or_else(|| absence_of(known), present)
}

/// The whole reconstruction as ordered lines — the comparison every determinism case makes.
fn transcript(reconstruction: &Reconstruction) -> Vec<String> {
    reconstruction.data().iter().map(fingerprint).collect()
}

fn plan_reconstruction(document: &support::DocumentUnderTest) -> Reconstruction {
    let facts = facts(document, Vec::new(), Vec::new(), None);
    reconstruct(&Rooted::Plan(&facts))
}

/// THE TOTALITY FLOOR: every recorded family the model names reaches the population exactly once.
///
/// A permutation check rather than a count: counts drift, and what must hold is that no family is
/// dropped and none is doubled. Non-empty first, so a reconstruction that produced nothing at all
/// could not satisfy it vacuously.
#[test]
fn every_named_family_reaches_the_population_exactly_once() {
    let document = published();
    let reconstruction = plan_reconstruction(&document);
    assert!(
        !reconstruction.data().is_empty(),
        "a reconstruction over a real document is non-empty; an empty one satisfies every \
         assertion below vacuously"
    );

    let mut seen: Vec<FamilyName> = reconstruction
        .audit()
        .into_iter()
        .map(|hole| hole.family)
        .collect();
    seen.sort_unstable();
    let mut expected = FamilyName::ALL.to_vec();
    expected.sort_unstable();
    assert_eq!(
        seen, expected,
        "every family the model names is audited exactly once; a family missing here is a hole \
         nobody would see, and a doubled one is two rows about one gap"
    );
}

/// Every audited hole says WHOSE it is, and at v1 every one of them is the report API's.
///
/// The distinction is the whole point of the audit (`30V` §5): a carrier hole is a durable
/// question and a report-API hole is not, and a surface that merged them would send the reader to
/// the wrong place.
#[test]
fn every_hole_names_its_cause_and_v1_holes_are_the_report_apis() {
    let document = published();
    let reconstruction = plan_reconstruction(&document);
    let holes = reconstruction.audit();
    assert!(!holes.is_empty(), "v1 has holes; an empty audit is a bug");
    for hole in holes {
        assert_eq!(
            hole.cause,
            CarrierAbsence::ReportApiLacks,
            "family {} is recorded by the durable and unprojected by the report API, so its hole \
             is the report API's; a carrier cause here would send a reader to widen the durable \
             for a gap that needs no durable change",
            hole.family.key()
        );
    }
}

/// Nothing in a v1 reconstruction is NYI.
///
/// The structural half of the NYI census: `KnowableNYI` is minted by exactly one constructor, and
/// no population walk reaches it today. A slot that starts answering `nyi` reddens here rather than
/// shipping silently as the product's voice (`30V` §3).
#[test]
fn no_slot_of_a_real_reconstruction_ships_as_not_yet_piped() {
    let document = published();
    let reconstruction = plan_reconstruction(&document);
    let unbuilt: Vec<String> = reconstruction
        .data()
        .iter()
        .filter(|datum| {
            datum.speaker().is_nyi()
                || datum.subject().is_nyi()
                || datum.payload().is_nyi()
                || datum.world().moment().is_nyi()
                || datum.world().host().is_nyi()
                || datum.world().lineage().is_nyi()
        })
        .map(fingerprint)
        .collect();
    assert!(
        unbuilt.is_empty(),
        "a KnowableNYI slot reached a reconstruction: {unbuilt:?}. Either pipe it or say what it \
         affirmatively is — laundering it into Unknowable is the named failure-mode"
    );
}

/// Re-deriving the same document twice produces the same ordered population.
#[test]
fn one_document_reconstructs_identically_every_time() {
    let document = published();
    let first = transcript(&plan_reconstruction(&document));
    let second = transcript(&plan_reconstruction(&document));
    assert_eq!(
        first, second,
        "the reconstruction is a pure function of its inputs"
    );
}

/// Permuting the ORDER the edge supplies siblings and observations in does not move a byte.
///
/// The real determinism question: the edge walks a store, and a store's enumeration order is not a
/// fact about the question. Two edges that found the same world must reconstruct the same thing.
#[test]
fn permuting_what_the_edge_supplies_does_not_move_the_reconstruction() {
    let document = published();
    let observations = || {
        vec![SourceObservation {
            ordinal: 0,
            reading: CurrentSourceReading::Read(BOOK.as_bytes().to_vec()),
            matches_digest: true,
        }]
    };
    let siblings = |reversed: bool| {
        let mut states = vec![
            SiblingState::Missing(dorc_receipt::report::RecordedDocumentId::Plan(document.id)),
            SiblingState::Unreadable(dorc_receipt::report::RecordedDocumentId::Plan(document.id)),
        ];
        if reversed {
            states.reverse();
        }
        states
    };

    let forward = facts(&document, siblings(false), observations(), None);
    let backward = facts(&document, siblings(true), observations(), None);
    let forward = transcript(&reconstruct(&Rooted::Plan(&forward)));
    let backward = transcript(&reconstruct(&Rooted::Plan(&backward)));

    // The sibling ROWS may legitimately follow the edge's order — what must not move is the set of
    // facts and the shape of every other row, so the comparison is over the sorted population.
    let forward_set: BTreeSet<&String> = forward.iter().collect();
    let backward_set: BTreeSet<&String> = backward.iter().collect();
    assert_eq!(
        forward_set, backward_set,
        "the same world reconstructs the same facts whatever order the edge walked it in"
    );
    assert_eq!(
        forward.len(),
        backward.len(),
        "and to the same number of rows"
    );
}

/// A source the edge compared, and one it did not, are different rows — never the same row twice.
#[test]
fn a_compared_source_and_an_uncompared_one_are_distinguishable() {
    let document = published();
    let compared = facts(
        &document,
        Vec::new(),
        vec![SourceObservation {
            ordinal: 0,
            reading: CurrentSourceReading::Read(BOOK.as_bytes().to_vec()),
            matches_digest: true,
        }],
        None,
    );
    let uncompared = facts(&document, Vec::new(), Vec::new(), None);
    assert_ne!(
        transcript(&reconstruct(&Rooted::Plan(&compared))),
        transcript(&reconstruct(&Rooted::Plan(&uncompared))),
        "a comparison that happened and one that did not are different facts; a model that read \
         the same either way would let a stale answer wear a fresh one's clothes"
    );
}

/// An address the question asked about reaches the population as its own subject.
#[test]
fn an_asked_address_reaches_the_population() {
    let document = published();
    let asked = facts(
        &document,
        Vec::new(),
        vec![SourceObservation {
            ordinal: 0,
            reading: CurrentSourceReading::Read(BOOK.as_bytes().to_vec()),
            matches_digest: true,
        }],
        Some(RequestedAddress::of(0, 2)),
    );
    let reconstruction = reconstruct(&Rooted::Plan(&asked));
    assert!(
        transcript(&reconstruction)
            .iter()
            .any(|row| row.contains("address:0:2")),
        "the address the user asked about is a subject of the reconstruction, not a parameter it \
         consumed and forgot"
    );
}

/// Every datum names the carrier that delivered it, and that carrier is in the closure.
///
/// The by-reference half of `30V` §3 field 5: standing is looked up on the carrier entity, so a
/// dangling reference would be a datum whose authentication nobody could answer.
#[test]
fn every_datum_resolves_to_a_carrier_that_is_actually_present() {
    let document = published();
    let reconstruction = plan_reconstruction(&document);
    for datum in reconstruction.data() {
        match datum.delivery() {
            Delivery::Recorded(_) => assert!(
                reconstruction.carrier_of(datum).is_some(),
                "a recorded datum names a carrier the closure holds"
            ),
            Delivery::Live => {
                panic!("v1 mints no live delivery; this arm is representable and unconstructed")
            }
        }
    }
}

/// A degraded document still reconstructs, and says which slots degraded.
#[test]
fn a_withheld_shell_and_a_bounded_source_read_as_their_own_absences() {
    let document = shaped(
        Shape {
            shell_withheld: true,
            content_over_bound: true,
            ..Shape::default()
        },
        11,
    );
    let reconstruction = plan_reconstruction(&document);
    let rows = transcript(&reconstruction);
    assert!(
        rows.iter().any(|row| row.contains("withheld:plain")),
        "a plain-withheld shell says so in its own word; got {rows:?}"
    );
    assert!(
        rows.iter().any(|row| row.contains("withheld:bound")),
        "a bound-refused source says so in its own word, distinct from the withheld one; got {rows:?}"
    );
}
