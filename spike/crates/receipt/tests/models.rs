//! The recorded-model corpus: every row round-trips, and the aggregates refuse a record set
//! that parses under the grammar and does not close over itself.
//!
//! The round trip is the transposition fence. [`RecordedRow::atoms`] writes positionally and
//! `of_record` reads by key, so a row whose atoms are emitted in the wrong order comes back
//! with its fields swapped. Every row below is therefore built with DISTINCT values in every
//! same-typed field: `leaf` and `ast` differ, `subject` and `member` differ, `operands` and
//! `dropped` differ. Equal values would make a transposition invisible.

#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "spike/clippy.toml's allow-*-in-tests keys reach the #[test] functions of an \
              integration-test crate but not the plain helper functions beside them, which is \
              what these files are largely made of; the file-top expect is the documented answer"
)]

use dorc_receipt::apply::{
    RecordedApplyAssignment, RecordedApplyIntent, RecordedApplyIntentRow, RecordedPlanOrigin,
};
use dorc_receipt::format::SkeletonRecord;
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::outcome::{
    RecordedApplyOutcome, RecordedApplyOutcomeRow, RecordedChannels, RecordedSiteOutcome,
};
use dorc_receipt::plan::{
    RecordedAdmission, RecordedLicensor, RecordedLoadDecision, RecordedNarrative,
    RecordedPlanReceipt, RecordedPresentedPlan, RecordedProbeShip, RecordedRegionDecision,
    RecordedRenderDecision, RecordedSiteClassification, RecordedSiteDecision,
    RecordedSolveCertification, RecordedSource, RecordedSurvival, RenderSubject, SourceSlots,
};
use dorc_receipt::reingested::RecordedInfluence;
use dorc_receipt::rows::{
    AssignmentOrdinal, LoadOrdinal, ModelRefusal, NarrativeOrdinal, OriginOrdinal, RecordedAst,
    RecordedInvocation, RecordedLeaf, RecordedMember, RecordedOperands, RecordedProjectionOmission,
    RecordedRow, RecordedSite, RegionOrdinal, RelationFault, SiteOutcomeOrdinal, SourceOrdinal,
};
use dorc_receipt::tokens::{
    ImageState, OpaqueState, RecordedAdmissionOutcome, RecordedApplyPolicy, RecordedDisposition,
    RecordedDurableState, RecordedInvocationMode, RecordedLicenseCustody, RecordedLicenseVerb,
    RecordedLoadOutcome, RecordedNarrativeKind, RecordedOmissionReason, RecordedOriginState,
    RecordedRenderKind, RecordedShipLane, RecordedSiteClass, RecordedSiteStatus, RecordedSolvePass,
    RecordedSourceClass, RecordedSourceRole, RecordedSpeechAct, RecordedSpineSpecies,
    RecordedSurvivalOutcome, RecordedTerminalState,
};

fn digest_of(seed: char) -> String {
    std::iter::repeat_n(seed, 64).collect()
}

/// A site with a member, so the leaf and member slots are both exercised and distinguishable.
fn site(leaf: u32, member: u32) -> RecordedSite {
    RecordedSite::of(RecordedLeaf::of(leaf), Some(RecordedMember::of(member)))
}

fn round_trip<R: RecordedRow + PartialEq + core::fmt::Debug>(row: &R) {
    let record = row.to_record().expect("the row must satisfy the table");
    assert_eq!(record.kind(), R::KIND);
    let back = R::of_record(&record).expect("the row must read back");
    assert_eq!(&back, row, "the row did not survive its own round trip");
}

#[test]
fn every_plan_row_survives_its_own_round_trip_with_distinct_same_typed_fields() {
    round_trip(&RecordedInvocation::of(
        RecordedInvocationMode::Plan,
        Some(1_700_000_000_000),
        OpaqueState::Captured,
        OpaqueState::WithheldPlain,
        3,
        RecordedInfluence::AuthoredBeforeContact,
    ));
    round_trip(&RecordedProjectionOmission::of(
        RecordedSpineSpecies::Observation,
        7,
        RecordedOmissionReason::Unminted,
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedSource::of(
        SourceOrdinal::of(2),
        RecordedSourceRole::BookSourced,
        digest_of('b'),
        4096,
        SourceSlots {
            path: OpaqueState::Captured,
            excerpt: OpaqueState::Uncollected,
            content: OpaqueState::Captured,
        },
        RecordedSourceClass::GeneralSh,
        RecordedInfluence::AuthoredBeforeContact,
    ));
    round_trip(&RecordedAdmission::of(
        RecordedAdmissionOutcome::Admitted,
        11,
        22,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedPresentedPlan::of(
        digest_of('1'),
        digest_of('2'),
        Some(digest_of('3')),
        RecordedInfluence::AuthoredBeforeContact,
    ));
    // leaf 4, member 5, ast 6: three counts, all different, so a swap cannot survive.
    round_trip(&RecordedSiteDecision::of(
        site(4, 5),
        RecordedAst::of(6),
        RecordedDisposition::Guard,
        OpaqueState::Captured,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedRegionDecision::of(
        RegionOrdinal::of(7),
        RecordedAst::of(8),
        RecordedDisposition::Replace,
        9,
        OpaqueState::WithheldPlain,
        RecordedInfluence::AuthoredBeforeContact,
    ));
    round_trip(&RecordedLoadDecision::of(
        LoadOrdinal::of(10),
        RecordedLoadOutcome::Contested,
        OpaqueState::Captured,
        OpaqueState::Unavailable,
        RecordedInfluence::AuthoredBeforeContact,
    ));
}

#[test]
fn every_plan_analysis_row_survives_its_own_round_trip_with_distinct_same_typed_fields() {
    round_trip(&RecordedSiteClassification::of(
        site(11, 12),
        RecordedAst::of(13),
        RecordedSiteClass::QueryResolvableStale,
        true,
        false,
        RecordedOperands::of(14, 15),
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedSolveCertification::of(
        RecordedSolvePass::WholeWindow,
        false,
        true,
        RecordedInfluence::AuthoredBeforeContact,
    ));
    round_trip(&RecordedProbeShip::of(
        site(16, 17),
        RecordedShipLane::Predict,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedSurvival::of(
        site(18, 19),
        RecordedSurvivalOutcome::DemotedPoisoned,
        Some(RecordedLeaf::of(20)),
        Some(21),
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(
        &RecordedRenderDecision::of(
            RenderSubject::Leaf(site(22, 23)),
            RecordedRenderKind::PinnedBinding,
            OpaqueState::Captured,
            RecordedInfluence::AuthoredBeforeContact,
        )
        .expect("a leaf subject fits a leaf-keyed kind"),
    );
    round_trip(&RecordedNarrative::of(
        NarrativeOrdinal::of(24),
        RecordedSpeechAct::Declined,
        RecordedNarrativeKind::VerdictDecline,
        RecordedOperands::of(25, 26),
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedLicensor::of(
        site(27, 28),
        RecordedLicenseVerb::Guard,
        RecordedLicenseCustody::VouchedSeverally,
        OpaqueState::Captured,
        RecordedInfluence::AuthoredBeforeContact,
    ));
}

#[test]
fn every_apply_row_survives_its_own_round_trip_with_distinct_same_typed_fields() {
    round_trip(&RecordedApplyIntentRow::of(
        digest_of('4'),
        digest_of('5'),
        RecordedApplyPolicy::RequiredRich,
        2,
        RecordedOriginState::Known,
        RecordedInfluence::AuthoredBeforeContact,
    ));
    round_trip(&RecordedApplyAssignment::of(
        AssignmentOrdinal::of(29),
        OpaqueState::Captured,
        OpaqueState::Unavailable,
        digest_of('6'),
        ImageState::Captured,
        30,
        RecordedInfluence::AuthoredBeforeContact,
    ));
    round_trip(&RecordedPlanOrigin::of(
        AssignmentOrdinal::of(31),
        OriginOrdinal::of(32),
        digest_of('7'),
        digest_of('8'),
        RecordedInfluence::AuthoredBeforeContact,
    ));
}

#[test]
fn every_outcome_row_survives_its_own_round_trip_with_distinct_same_typed_fields() {
    round_trip(&RecordedApplyOutcomeRow::of(
        digest_of('9'),
        RecordedTerminalState::Unknown,
        1,
        RecordedDurableState::Failed,
        RecordedInfluence::HostInfluenced,
    ));
    round_trip(&RecordedSiteOutcome::of(
        SiteOutcomeOrdinal::of(33),
        AssignmentOrdinal::of(34),
        site(35, 36),
        RecordedSiteStatus::GuardFellThrough,
        Some(37),
        RecordedChannels::of(OpaqueState::Captured, OpaqueState::Uncollected),
        RecordedInfluence::HostInfluenced,
    ));
}

#[test]
fn the_round_trip_preserves_which_count_went_in_which_slot() {
    // The fence stated as an assertion rather than as an equality: if `atoms` emitted `ast`
    // where `leaf` belongs, this row would come back with 6 and 4 exchanged and every field
    // would still be a legal count, so the grammar table alone could never notice.
    let row = RecordedSiteDecision::of(
        site(4, 5),
        RecordedAst::of(6),
        RecordedDisposition::Run,
        OpaqueState::Unavailable,
        OpaqueState::Unavailable,
        RecordedInfluence::AuthoredBeforeContact,
    );
    let back = RecordedSiteDecision::of_record(&row.to_record().unwrap()).unwrap();
    assert_eq!(back.site().leaf(), RecordedLeaf::of(4));
    assert_eq!(back.site().member(), Some(RecordedMember::of(5)));
    assert_eq!(back.ast(), RecordedAst::of(6));
}

#[test]
fn a_row_model_refuses_a_record_of_another_kind() {
    let other = RecordedSolveCertification::of(
        RecordedSolvePass::SelfReach,
        true,
        false,
        RecordedInfluence::AuthoredBeforeContact,
    )
    .to_record()
    .unwrap();
    assert_eq!(
        RecordedSource::of_record(&other).unwrap_err(),
        ModelRefusal::Kind {
            expected: "source",
            found: "solve-certification",
        }
    );
}

#[test]
fn a_render_row_cannot_carry_a_subject_its_kind_does_not_own() {
    // The axis is a function of the kind, so these are refused at construction rather than
    // written and discovered by a reader. Each refusal names the axis the kind owns AND the axis
    // it was handed, so the three cases below are distinguishable from one another rather than
    // all satisfied by any axis complaint.
    let cases = [
        (
            RenderSubject::Region(RegionOrdinal::of(0)),
            RecordedRenderKind::PinnedBinding,
            "leaf",
            "region",
        ),
        (
            RenderSubject::Leaf(site(1, 2)),
            RecordedRenderKind::ImportInlined,
            "none",
            "leaf",
        ),
        (
            RenderSubject::None,
            RecordedRenderKind::RefusedHeredocRegion,
            "region",
            "none",
        ),
    ];
    for (subject, kind, expected, supplied) in cases {
        assert_eq!(
            RecordedRenderDecision::of(
                subject,
                kind,
                OpaqueState::Captured,
                RecordedInfluence::AuthoredBeforeContact,
            )
            .unwrap_err(),
            RelationFault::SubjectAxisDisagrees {
                expected,
                supplied,
                kind: "render-decision",
            },
            "{kind:?} was handed a {supplied} subject"
        );
    }
    // And the three that DO agree are accepted, so the refusals above are about the axis and
    // not about the constructor refusing everything.
    for (subject, kind) in [
        (
            RenderSubject::Leaf(site(1, 2)),
            RecordedRenderKind::OmitNeutralised,
        ),
        (
            RenderSubject::Region(RegionOrdinal::of(0)),
            RecordedRenderKind::RefusedBlockingRedirectRegion,
        ),
        (RenderSubject::None, RecordedRenderKind::DefensiveEmissionOn),
    ] {
        assert!(
            RecordedRenderDecision::of(
                subject,
                kind,
                OpaqueState::Captured,
                RecordedInfluence::AuthoredBeforeContact,
            )
            .is_ok(),
            "{kind:?} should accept its own axis"
        );
    }
}

fn invocation_record() -> SkeletonRecord {
    RecordedInvocation::of(
        RecordedInvocationMode::Plan,
        None,
        OpaqueState::WithheldPlain,
        OpaqueState::WithheldPlain,
        1,
        RecordedInfluence::AuthoredBeforeContact,
    )
    .to_record()
    .unwrap()
}

fn source_record(ordinal: u32) -> SkeletonRecord {
    RecordedSource::of(
        SourceOrdinal::of(ordinal),
        RecordedSourceRole::Book,
        digest_of('b'),
        1,
        SourceSlots {
            path: OpaqueState::WithheldPlain,
            excerpt: OpaqueState::Uncollected,
            content: OpaqueState::WithheldPlain,
        },
        RecordedSourceClass::DorcLang,
        RecordedInfluence::AuthoredBeforeContact,
    )
    .to_record()
    .unwrap()
}

#[test]
fn a_plan_model_wants_exactly_one_invocation() {
    // Each refusal is pinned to its exact operands. A negative test that asserts only "it was
    // refused" is satisfied by a refusal for any other reason, which is how a guard stops
    // covering the departure it is named for.
    assert_eq!(
        RecordedPlanReceipt::of_records(&[source_record(0)]).unwrap_err(),
        ModelRefusal::Relation(RelationFault::MissingSingleton { kind: "invocation" })
    );
    assert_eq!(
        RecordedPlanReceipt::of_records(&[invocation_record(), invocation_record()]).unwrap_err(),
        ModelRefusal::Relation(RelationFault::DuplicateSingleton { kind: "invocation" })
    );
    assert!(RecordedPlanReceipt::of_records(&[invocation_record()]).is_ok());
}

#[test]
fn a_plan_model_wants_its_ordinals_contiguous_from_zero() {
    // A gap means a row was dropped somewhere between the projection and the document, which a
    // reader counting rows would silently absorb. The pinned operands say WHICH ordinal broke
    // the run, so a refusal arriving from another kind cannot satisfy this.
    let records = vec![invocation_record(), source_record(0), source_record(2)];
    assert_eq!(
        RecordedPlanReceipt::of_records(&records).unwrap_err(),
        ModelRefusal::Relation(RelationFault::OrdinalNotContiguous {
            kind: "source",
            expected: 1,
            found: 2,
        })
    );
    let good = vec![invocation_record(), source_record(0), source_record(1)];
    assert!(RecordedPlanReceipt::of_records(&good).is_ok());
}

#[test]
fn a_render_row_cannot_name_a_region_the_document_does_not_declare() {
    let render = RecordedRenderDecision::of(
        RenderSubject::Region(RegionOrdinal::of(3)),
        RecordedRenderKind::RefusedHeredocRegion,
        OpaqueState::Captured,
        RecordedInfluence::AuthoredBeforeContact,
    )
    .unwrap()
    .to_record()
    .unwrap();
    assert_eq!(
        RecordedPlanReceipt::of_records(&[invocation_record(), render]).unwrap_err(),
        ModelRefusal::Relation(RelationFault::DanglingRegion { region: 3 })
    );
}

#[test]
fn a_plan_model_reserializes_in_one_canonical_order() {
    // Two documents carrying the same content must not differ in bytes, so emission follows the
    // species kind order rather than whatever order the KINDS arrived in. Ordinals within a
    // kind are a separate matter and must already ascend — a document whose sources run 1 then
    // 0 is refused, which the contiguity test above pins.
    let interleaved = vec![source_record(0), invocation_record(), source_record(1)];
    let model = RecordedPlanReceipt::of_records(&interleaved).unwrap();
    let emitted = model.to_records().unwrap();
    let kinds: Vec<RecordKind> = emitted.iter().map(SkeletonRecord::kind).collect();
    assert_eq!(
        kinds,
        vec![
            RecordKind::Invocation,
            RecordKind::Source,
            RecordKind::Source
        ]
    );
    let again = RecordedPlanReceipt::of_records(&emitted).unwrap();
    assert_eq!(
        again.to_records().unwrap(),
        emitted,
        "emission is a fixpoint"
    );
}

fn intent_records(assignments: u32, origins: &[(u32, u32)]) -> Vec<SkeletonRecord> {
    let state = if origins.is_empty() {
        RecordedOriginState::Unavailable
    } else {
        RecordedOriginState::Known
    };
    let mut records = vec![
        RecordedInvocation::of(
            RecordedInvocationMode::Apply,
            None,
            OpaqueState::WithheldPlain,
            OpaqueState::WithheldPlain,
            1,
            RecordedInfluence::AuthoredBeforeContact,
        )
        .to_record()
        .unwrap(),
        RecordedApplyIntentRow::of(
            digest_of('5'),
            digest_of('6'),
            RecordedApplyPolicy::RequiredRich,
            assignments,
            state,
            RecordedInfluence::AuthoredBeforeContact,
        )
        .to_record()
        .unwrap(),
    ];
    for ordinal in 0..assignments {
        let mine = u32::try_from(
            origins
                .iter()
                .filter(|(assignment, _)| *assignment == ordinal)
                .count(),
        )
        .unwrap();
        records.push(
            RecordedApplyAssignment::of(
                AssignmentOrdinal::of(ordinal),
                OpaqueState::WithheldPlain,
                OpaqueState::WithheldPlain,
                digest_of('7'),
                ImageState::WithheldPlain,
                mine,
                RecordedInfluence::AuthoredBeforeContact,
            )
            .to_record()
            .unwrap(),
        );
    }
    for (assignment, ordinal) in origins {
        records.push(
            RecordedPlanOrigin::of(
                AssignmentOrdinal::of(*assignment),
                OriginOrdinal::of(*ordinal),
                digest_of('a'),
                digest_of('c'),
                RecordedInfluence::AuthoredBeforeContact,
            )
            .to_record()
            .unwrap(),
        );
    }
    records
}

#[test]
fn an_intent_model_closes_assignments_over_their_origins() {
    // One presented plan feeding two assignments, and one assignment composing two plans: the
    // mapping is many-to-many in both directions, and duplicates are retained rather than
    // collapsed to a set.
    let records = intent_records(2, &[(0, 0), (0, 1), (1, 0)]);
    let model = RecordedApplyIntent::of_records(&records).unwrap();
    assert_eq!(model.assignments().len(), 2);
    assert_eq!(model.assignments()[0].origins().len(), 2);
    assert_eq!(model.assignments()[1].origins().len(), 1);
    assert_eq!(model.to_records().unwrap().len(), records.len());
}

#[test]
fn an_intent_model_refuses_a_declared_count_the_rows_contradict() {
    let mut records = intent_records(2, &[]);
    records.pop();
    assert_eq!(
        RecordedApplyIntent::of_records(&records).unwrap_err(),
        ModelRefusal::Relation(RelationFault::CountDisagrees {
            kind: "apply-intent",
            declared: 2,
            present: 1,
        })
    );
}

#[test]
fn an_intent_model_refuses_an_origin_naming_no_assignment() {
    let records = intent_records(1, &[(4, 0)]);
    assert_eq!(
        RecordedApplyIntent::of_records(&records).unwrap_err(),
        ModelRefusal::Relation(RelationFault::DanglingAssignment { assignment: 4 })
    );
}

#[test]
fn an_intent_model_refuses_an_origin_state_its_assignments_contradict() {
    // The document-level summary and the per-assignment truth must agree, or a reader learns
    // one thing from the header and another from the rows.
    let mut records = intent_records(1, &[]);
    records[1] = RecordedApplyIntentRow::of(
        digest_of('5'),
        digest_of('6'),
        RecordedApplyPolicy::RequiredRich,
        1,
        RecordedOriginState::Known,
        RecordedInfluence::AuthoredBeforeContact,
    )
    .to_record()
    .unwrap();
    assert_eq!(
        RecordedApplyIntent::of_records(&records).unwrap_err(),
        ModelRefusal::Relation(RelationFault::OriginStateDisagrees {
            declared: "known",
            with_origins: 0,
        })
    );
}

#[test]
fn an_outcome_model_closes_its_declared_site_count() {
    let mut records = vec![
        RecordedInvocation::of(
            RecordedInvocationMode::Apply,
            None,
            OpaqueState::WithheldPlain,
            OpaqueState::WithheldPlain,
            1,
            RecordedInfluence::HostInfluenced,
        )
        .to_record()
        .unwrap(),
        RecordedApplyOutcomeRow::of(
            digest_of('9'),
            RecordedTerminalState::Complete,
            1,
            RecordedDurableState::Published,
            RecordedInfluence::HostInfluenced,
        )
        .to_record()
        .unwrap(),
    ];
    assert_eq!(
        RecordedApplyOutcome::of_records(&records).unwrap_err(),
        ModelRefusal::Relation(RelationFault::CountDisagrees {
            kind: "apply-outcome",
            declared: 1,
            present: 0,
        })
    );
    records.push(
        RecordedSiteOutcome::of(
            SiteOutcomeOrdinal::of(0),
            AssignmentOrdinal::of(0),
            RecordedSite::of(RecordedLeaf::of(1), None),
            RecordedSiteStatus::Ran,
            Some(0),
            RecordedChannels::of(OpaqueState::Uncollected, OpaqueState::Uncollected),
            RecordedInfluence::HostInfluenced,
        )
        .to_record()
        .unwrap(),
    );
    assert!(RecordedApplyOutcome::of_records(&records).is_ok());
}
