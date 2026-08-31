//! What the reconstruction must hold, whatever it was built from.
//!
//! Driven through REAL published documents rather than hand-built models, for the reason the
//! fixture module gives: a hand-built model agrees with itself.
//!
//! The censuses here are STRUCTURAL, never lexical (`lexical-fences-are-human-ack-instruments`):
//! totality is a no-wildcard match plus a permutation check over the model's own flat population,
//! and determinism is measured by re-deriving and comparing, not by grepping for a collection type.

mod support;

use std::collections::BTreeSet;

use dorc_receipt::report::{
    CurrentSourceReading, FamilyCoverage, PlanFamily, RequestedAddress, SiblingState,
    SourceObservation,
};
use dorc_why::known::{CantTell, CarrierAbsence, Held, Known, WithholdReason};
use dorc_why::recorded::{Rooted, reconstruct};
use dorc_why::{
    ComparedSources, CorrelationFact, Datum, Delivery, IdentityFact, Moment, NegativeSpace,
    Payload, Reconstruction, RecordedFlag, RecordedToken, Separability, Speaker, StateFact,
    Subject, VoiceSet,
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

fn voices_of(speaker: &Speaker) -> String {
    let Known::Knowable(Held::Present(voices)) = speaker.voices() else {
        return absence_of(speaker.voices());
    };
    match voices {
        VoiceSet::Mine => "mine".to_owned(),
        VoiceSet::One(_) => "one".to_owned(),
        VoiceSet::Committee {
            separability: Separability::Separable,
            ..
        } => "committee-separable".to_owned(),
        VoiceSet::Committee {
            separability: Separability::Inseparable,
            ..
        } => "committee-inseparable".to_owned(),
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
        Subject::Family(family) => format!("family:{}", family.token()),
        Subject::Narrative(ordinal) => format!("narrative:{ordinal}"),
        Subject::Region(ordinal) => format!("region:{ordinal}"),
        Subject::Load(ordinal) => format!("load:{ordinal}"),
        Subject::Question => "question".to_owned(),
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
        Payload::Collapse(kind) => format!("collapse:{kind:?}"),
        Payload::Token(token) => format!("token:{}", token_of(*token)),
        Payload::Flag(flag) => format!("flag:{}", flag_of(*flag)),
        Payload::NegativeSpace(space) => format!("negative:{}", negative_of(*space)),
        Payload::Unplaceable(why) => format!("unplaceable:{why:?}"),
    })
}

/// Every recorded closed vocabulary a payload can carry, matched without a wildcard.
fn token_of(token: RecordedToken) -> String {
    match token {
        RecordedToken::AdmissionOutcome(value) => format!("admission:{value:?}"),
        RecordedToken::LoadOutcome(value) => format!("load:{value:?}"),
        RecordedToken::SiteClass(value) => format!("class:{value:?}"),
        RecordedToken::SolvePass(value) => format!("pass:{value:?}"),
        RecordedToken::ShipLane(value) => format!("lane:{value:?}"),
        RecordedToken::SurvivalOutcome(value) => format!("survival:{value:?}"),
        RecordedToken::RenderKind(value) => format!("render:{value:?}"),
        RecordedToken::LicenseVerb(value) => format!("license:{value:?}"),
        RecordedToken::LicenseCustody(value) => format!("custody:{value:?}"),
        RecordedToken::ApplyPolicy(value) => format!("policy:{value:?}"),
        RecordedToken::OriginState(value) => format!("origin:{value:?}"),
        RecordedToken::TerminalState(value) => format!("terminal:{value:?}"),
    }
}

/// Each named predicate travels with its answer, so the two are never separable in a fingerprint.
fn flag_of(flag: RecordedFlag) -> String {
    match flag {
        RecordedFlag::VerdictLane(value) => format!("verdict-lane:{value}"),
        RecordedFlag::Invalidator(value) => format!("invalidator:{value}"),
        RecordedFlag::SolveConsistent(value) => format!("solve-consistent:{value}"),
        RecordedFlag::SolveTripped(value) => format!("solve-tripped:{value}"),
    }
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
        IdentityFact::InvocationMode(mode) => format!("invocation-mode:{mode:?}"),
        IdentityFact::Operands(operands) => {
            format!("operands:{}+{}", operands.shown(), operands.dropped())
        }
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
    format!("{:?}:{}", space.kind, space.family.token())
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
    reconstruct(&Rooted::Plan(&facts), &ComparedSources::default())
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

    let facts = facts(&document, Vec::new(), Vec::new(), None);
    let mut seen: Vec<PlanFamily> = reconstruction
        .audit()
        .into_iter()
        .map(|hole| hole.family)
        .collect();
    seen.sort_unstable();
    // The expectation comes from the REPORT's own coverage answer, never from a list kept here: a
    // second list would disagree the moment a family is projected, and the disagreement would read
    // as a hole that no longer exists.
    let mut expected: Vec<PlanFamily> = facts
        .coverage()
        .into_iter()
        .filter(|(_, coverage)| !coverage.is_projected())
        .map(|(family, _)| family)
        .collect();
    expected.sort_unstable();
    assert_eq!(
        seen, expected,
        "every unprojected family is audited exactly once, and every projected one is audited not \
         at all; a doubled row is two rows about one gap, and a row for a projected family sends a \
         reader to widen something already there"
    );
}

/// The read surface is EXHAUSTIVE: no family answers `RecordedButUnprojected` any more.
///
/// The lane's central claim, and the only case that would notice it regressing. A family sliding
/// back to that word is a projection that was removed, which is different from a document that
/// carries no such row — and only the first is repaired by projection work
/// (`inv-report-projection-exhaustive-or-classified`).
#[test]
fn no_persisted_family_is_left_unprojected() {
    let document = published();
    let facts = facts(&document, Vec::new(), Vec::new(), None);
    let coverage = facts.coverage();
    assert_eq!(
        coverage.len(),
        PlanFamily::ALL.len(),
        "coverage answers for every persisted family; an unanswered one is the silence the \
         classification exists to make impossible"
    );
    for (family, cover) in &coverage {
        assert_ne!(
            *cover,
            FamilyCoverage::RecordedButUnprojected,
            "family {} is persisted by the durable and the report API projects it; that word is \
             now reachable only by a projection somebody removed",
            family.token()
        );
    }
    assert!(
        coverage
            .iter()
            .any(|(family, cover)| *family == PlanFamily::Narratives && cover.is_projected()),
        "the narrative family is projected — it is what carries the recorded speech acts"
    );
}

/// A document holding no rows of a SINGLETON family says so in the carrier's word, not the report
/// API's.
///
/// The distinction is the whole point of the audit (`30V` §5): a carrier hole is a durable question
/// and a report-API hole is not, and a surface that merged them would send the reader to the wrong
/// place. Driven over a document published WITHOUT its two optional singletons, which is an ordinary
/// store shape — the full fixture audits nothing at all, which is the other half of the claim and is
/// pinned by its neighbour above.
#[test]
fn every_hole_names_its_cause_and_the_v1_holes_are_the_carriers() {
    let document = shaped(
        Shape {
            without_singletons: true,
            ..Shape::default()
        },
        11,
    );
    let facts = facts(&document, Vec::new(), Vec::new(), None);
    let reconstruction = reconstruct(&Rooted::Plan(&facts), &ComparedSources::default());
    let holes = reconstruction.audit();
    assert!(
        !holes.is_empty(),
        "the fixture document carries neither singleton, so the audit has something to say; an \
         empty audit here would make the causes below vacuous"
    );
    for hole in holes {
        let coverage = facts
            .coverage()
            .into_iter()
            .find(|(family, _)| *family == hole.family)
            .map(|(_, cover)| cover)
            .expect("an audited family is one the coverage answer names");
        assert_eq!(
            coverage,
            FamilyCoverage::NotCarried,
            "family {} is audited, so the report answered something other than typed facts for it",
            hole.family.token()
        );
        assert_eq!(
            hole.cause,
            CarrierAbsence::RunHeldNoValue,
            "family {} is one this document does not carry, so its hole is the carrier's; a \
             report-API cause here would send a reader to widen a projection that is already there",
            hole.family.token()
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
    let forward = transcript(&reconstruct(
        &Rooted::Plan(&forward),
        &ComparedSources::default(),
    ));
    let backward = transcript(&reconstruct(
        &Rooted::Plan(&backward),
        &ComparedSources::default(),
    ));

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
        transcript(&reconstruct(
            &Rooted::Plan(&compared),
            &ComparedSources::default()
        )),
        transcript(&reconstruct(
            &Rooted::Plan(&uncompared),
            &ComparedSources::default()
        )),
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
    let reconstruction = reconstruct(&Rooted::Plan(&asked), &ComparedSources::default());
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

/// Every widened family actually reaches the population — the projections are WALKED, not merely
/// available.
///
/// Structural rather than lexical: the discriminant names come from the same no-wildcard matches the
/// fingerprints use, so a new recorded vocabulary reddens `token_of` at compile time and this
/// expectation in the diff beside it. A family whose rows exist and whose reconstruction dropped
/// them would otherwise look exactly like a document that carried nothing.
#[test]
fn every_widened_family_reaches_the_population() {
    let reconstruction = plan_reconstruction(&published());

    let mut tokens: BTreeSet<String> = BTreeSet::new();
    let mut flags: BTreeSet<String> = BTreeSet::new();
    let mut subjects: BTreeSet<String> = BTreeSet::new();
    for datum in reconstruction.data() {
        match datum.payload() {
            Known::Knowable(Held::Present(Payload::Token(token))) => {
                tokens.insert(token_of(*token));
            }
            Known::Knowable(Held::Present(Payload::Flag(flag))) => {
                flags.insert(flag_of(*flag));
            }
            _ => {}
        }
        subjects.insert(subject_of(datum));
    }

    let named: Vec<&str> = tokens
        .iter()
        .map(|token| token.split(':').next().unwrap_or(""))
        .collect();
    for family in [
        "admission",
        "load",
        "class",
        "pass",
        "lane",
        "survival",
        "render",
        "license",
        "custody",
    ] {
        assert!(
            named.contains(&family),
            "no datum carries a `{family}` token; its family's rows exist in the document, so the \
             reconstruction dropped them: {tokens:?}"
        );
    }
    assert_eq!(
        flags.len(),
        4,
        "all four named predicates reach the population, each with its own answer: {flags:?}"
    );
    for subject in ["region:0", "load:0"] {
        assert!(
            subjects.contains(subject),
            "no datum is about `{subject}`; a region keyed by one of its executions would be the \
             two-identities conflation `30L:rul-two-identities-never-conflated` refuses"
        );
    }
}

/// A licensor row speaks in the act its recorded CUSTODY names, with its voices honestly unnamed.
///
/// `30V` §2 rul-first-person-register puts the tool's "I" only where no more-correct register
/// exists, and a recorded custody is one: the license rested on somebody's vouch. The voice-set
/// stays a separate answer because the authoring locus is an opaque slot — the act is knowable and
/// the speaker is not (`a-voice-set-is-its-own-leaf`).
#[test]
fn a_licensed_verb_speaks_in_the_act_its_custody_names() {
    let reconstruction = plan_reconstruction(&published());
    let licensed: Vec<&Datum> = reconstruction
        .data()
        .iter()
        .filter(|datum| {
            matches!(
                datum.payload(),
                Known::Knowable(Held::Present(Payload::Token(RecordedToken::LicenseVerb(_))))
            )
        })
        .collect();
    assert!(!licensed.is_empty(), "the fixture publishes a licensor row");
    for datum in licensed {
        assert_eq!(
            speaker_of(datum),
            "Vouched/absent:report-api-lacks",
            "a vouched custody speaks as a vouch, and names nobody"
        );
    }
}
