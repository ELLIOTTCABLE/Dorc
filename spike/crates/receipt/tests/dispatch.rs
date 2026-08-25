#![expect(
    clippy::panic,
    reason = "clippy.toml's allow-panic-in-tests reaches `#[test]` functions in this crate but \
              not the plain fixture-building helpers beside them, and threading a Result through \
              fixtures that must succeed buys nothing"
)]
//! The pre-dispatch authority chain, and the refusals that keep it from being talked around.
//!
//! Every negative below is pinned to its EXACT refusal rather than to "it was rejected": the
//! failures in this family look alike from the outside — an unknown target, a duplicated
//! ordinal and a missing target all end in "no permit" — and a chain that refused for the
//! wrong reason would still pass a test that only asked whether it refused.

use dorc_receipt::dispatch::{
    ApplySessionReady, ConfiguredReceiptBypass, DurableFailure, ExecutionIntegrityFailure,
    IntentPreparationRefusal, IntentPublicationGate, PendingApplyAssignment, PendingOrigins,
    PlanOriginOccurrence, PostDispatchFailure, ReadyApplyTarget, ReceiptPolicyWitness,
    ResolvedApplyContext,
};
use dorc_receipt::ids::{
    ApplyGenerationId, ApplySessionId, PlanReceiptId, PresentedPlanId, ReadyApplyTargetId,
    ReceiptId, ReceiptIdSource,
};
use dorc_receipt::image::{ApplyArtifactImage, ApplyEntryBytes};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::overlay::OverlayEntry;
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::rows::AssignmentOrdinal;
use dorc_receipt::tokens::{RecordedApplyPolicy, RecordedOriginState};

/// A counting identity source. The production edge fills these from the operating system;
/// nothing below the edge reaches for either.
struct Counter(u8);

impl ReceiptIdSource for Counter {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

fn context(destination: &str) -> ResolvedApplyContext {
    ResolvedApplyContext::of(
        destination.to_owned(),
        "root".to_owned(),
        "host".to_owned(),
        "/root".to_owned(),
        "inherited".to_owned(),
        "session".to_owned(),
    )
}

fn image(bytes: &[u8]) -> ApplyArtifactImage {
    match ApplyArtifactImage::of_external_stream(
        ApplyEntryBytes::of(bytes.to_vec()),
        &ReceiptLimits::V1,
    ) {
        Ok(image) => image,
        Err(refusal) => panic!("a single stream should build: {refusal:?}"),
    }
}

/// One session over one resolved target, and the target's own identity beside it.
fn session(ids: &mut Counter) -> (ApplySessionReady, ReadyApplyTargetId) {
    let target = ReadyApplyTargetId::mint(ids);
    let ready = match ApplySessionReady::of(
        ApplySessionId::mint(ids),
        ApplyGenerationId::mint(ids),
        vec![ReadyApplyTarget::of(target, context("web1.example.net"))],
    ) {
        Ok(ready) => ready,
        Err(refusal) => panic!("a one-target standup should close: {refusal:?}"),
    };
    (ready, target)
}

fn assignment(target: ReadyApplyTargetId, ordinal: u32) -> PendingApplyAssignment {
    PendingApplyAssignment::of(
        AssignmentOrdinal::of(ordinal),
        target,
        image(b"#!/bin/sh\nufw allow 443/tcp\n"),
        PendingOrigins::Unavailable,
    )
}

#[test]
fn a_prepared_intent_copies_the_sessions_own_answer_rather_than_a_callers() {
    // The mint's whole job: the assignment named a target, and what lands in the record is the
    // context the STANDUP resolved for it. A caller supplies the pairing and never the answer.
    let mut ids = Counter(0);
    let (ready, target) = session(&mut ids);
    let prepared = match ready.prepare_intent(
        vec![assignment(target, 0)],
        ReceiptPolicyWitness::required_rich(),
    ) {
        Ok(prepared) => prepared,
        Err(refusal) => panic!("a well-formed assignment should prepare: {refusal:?}"),
    };

    let bound = prepared.assignments();
    assert_eq!(bound.len(), 1, "one assignment in, one out");
    let Some(only) = bound.first() else {
        panic!("the assignment vector is non-empty by construction");
    };
    assert_eq!(
        only.context().destination(),
        "web1.example.net",
        "the record carries the standup's resolved destination"
    );
    assert_eq!(
        only.session(),
        prepared.session(),
        "the assignment is bound to the session that resolved its target"
    );
    assert_eq!(
        prepared.origin_state(),
        RecordedOriginState::Unavailable,
        "no assignment named a plan, so the intent row says so explicitly"
    );
}

#[test]
fn an_assignment_naming_a_target_the_session_never_resolved_refuses_as_unknown() {
    // Distinct from every other refusal below: the pairing is well-formed and the SESSION is
    // the thing that cannot answer for it.
    let mut ids = Counter(0);
    let (ready, _) = session(&mut ids);
    let stranger = ReadyApplyTargetId::mint(&mut ids);

    assert_eq!(
        ready
            .prepare_intent(
                vec![assignment(stranger, 0)],
                ReceiptPolicyWitness::required_rich(),
            )
            .err(),
        Some(IntentPreparationRefusal::UnknownTarget),
        "a target from outside this session is unknown, never merely omitted"
    );
}

#[test]
fn a_resolved_target_left_unassigned_refuses_as_omitted_not_as_unknown() {
    // The mirror of the case above, and the pair is the point: one is "you named something I
    // do not have", the other is "I stood something up that you did not name". Collapsing them
    // would report a stranger where a partial apply was asked for.
    let mut ids = Counter(0);
    let first = ReadyApplyTargetId::mint(&mut ids);
    let second = ReadyApplyTargetId::mint(&mut ids);
    let ready = match ApplySessionReady::of(
        ApplySessionId::mint(&mut ids),
        ApplyGenerationId::mint(&mut ids),
        vec![
            ReadyApplyTarget::of(first, context("web1.example.net")),
            ReadyApplyTarget::of(second, context("web2.example.net")),
        ],
    ) {
        Ok(ready) => ready,
        Err(refusal) => panic!("a two-target standup should close: {refusal:?}"),
    };

    assert_eq!(
        ready
            .prepare_intent(
                vec![assignment(first, 0)],
                ReceiptPolicyWitness::required_rich(),
            )
            .err(),
        Some(IntentPreparationRefusal::ReadyTargetOmitted),
        "the second target was stood up and never assigned"
    );
}

#[test]
fn two_assignments_claiming_one_ordinal_refuse_as_duplicate_not_as_non_contiguous() {
    // Also a look-alike pair: a repeated ordinal and a gapped one both break the sequence, but
    // only one of them means two assignments are fighting over a position.
    let mut ids = Counter(0);
    let (ready, target) = session(&mut ids);

    assert_eq!(
        ready
            .prepare_intent(
                vec![assignment(target, 0), assignment(target, 0)],
                ReceiptPolicyWitness::required_rich(),
            )
            .err(),
        Some(IntentPreparationRefusal::DuplicateOrdinal),
        "one position, two claimants"
    );
}

#[test]
fn an_assignment_sequence_with_a_gap_names_the_ordinal_it_wanted() {
    let mut ids = Counter(0);
    let (ready, target) = session(&mut ids);

    assert_eq!(
        ready
            .prepare_intent(
                vec![assignment(target, 1)],
                ReceiptPolicyWitness::required_rich(),
            )
            .err(),
        Some(IntentPreparationRefusal::OrdinalNotContiguous {
            expected: 0,
            found: 1,
        }),
        "the refusal names both ordinals, so a reader is not left to infer which end drifted"
    );
}

#[test]
fn an_origin_list_with_a_gap_refuses_separately_from_an_assignment_gap() {
    // Two ordinal sequences run through one mint and they are NOT the same sequence. A single
    // shared refusal would send a reader to the assignment list when the origin list drifted.
    let mut ids = Counter(0);
    let (ready, target) = session(&mut ids);
    let Some(origins) = PendingOrigins::known(vec![PlanOriginOccurrence::of(
        dorc_receipt::rows::OriginOrdinal::of(3),
        PlanReceiptId::mint(&mut ids),
        PresentedPlanId::of_canonical_decision(b"a settled surface"),
    )]) else {
        panic!("a one-occurrence list is non-empty");
    };
    let pending = PendingApplyAssignment::of(
        AssignmentOrdinal::of(0),
        target,
        image(b"#!/bin/sh\n:\n"),
        origins,
    );

    assert_eq!(
        ready
            .prepare_intent(vec![pending], ReceiptPolicyWitness::required_rich())
            .err(),
        Some(IntentPreparationRefusal::OriginNotContiguous {
            expected: 0,
            found: 3,
        }),
        "the origin sequence is its own sequence, and says so"
    );
}

#[test]
fn an_empty_standup_and_an_empty_assignment_set_both_refuse_before_anything_is_bound() {
    let mut ids = Counter(0);
    assert_eq!(
        ApplySessionReady::of(
            ApplySessionId::mint(&mut ids),
            ApplyGenerationId::mint(&mut ids),
            Vec::new(),
        )
        .err(),
        Some(IntentPreparationRefusal::NoAssignments),
        "a standup that resolved nothing is not a session"
    );

    let (ready, _) = session(&mut ids);
    assert_eq!(
        ready
            .prepare_intent(Vec::new(), ReceiptPolicyWitness::required_rich())
            .err(),
        Some(IntentPreparationRefusal::NoAssignments),
        "an intent with no assignment authorizes nothing"
    );
}

#[test]
fn image_accounting_answers_only_when_the_regions_bytes_are_the_images_own() {
    // The capability is a BYTE comparison, not a declaration. A region carrying some other
    // image's bytes under the right tag is exactly the shape a caller would reach for to get a
    // permit it has not earned.
    let mut ids = Counter(0);
    let (ready, target) = session(&mut ids);
    let prepared = match ready.prepare_intent(
        vec![assignment(target, 0)],
        ReceiptPolicyWitness::required_rich(),
    ) {
        Ok(prepared) => prepared,
        Err(refusal) => panic!("a well-formed assignment should prepare: {refusal:?}"),
    };
    let Some(only) = prepared.assignments().first() else {
        panic!("the assignment vector is non-empty by construction");
    };
    let exact = only.image().encode().to_vec();
    let record_of = |_: AssignmentOrdinal| Some(7_u64);

    assert!(
        prepared
            .account_images(
                &[OverlayEntry::of(
                    7,
                    OpaqueFieldTag::ApplyArtifactImage,
                    exact.clone(),
                )],
                &record_of,
            )
            .is_some(),
        "the region carries this assignment's own canonical bytes"
    );

    let cousin = image(b"#!/bin/sh\nufw allow 80/tcp\n").encode().to_vec();
    assert_ne!(
        cousin, exact,
        "the fixture cousin really is different bytes"
    );
    assert!(
        prepared
            .account_images(
                &[OverlayEntry::of(
                    7,
                    OpaqueFieldTag::ApplyArtifactImage,
                    cousin
                )],
                &record_of,
            )
            .is_none(),
        "a region carrying a DIFFERENT image accounts for nothing"
    );

    assert!(
        prepared
            .account_images(
                &[OverlayEntry::of(7, OpaqueFieldTag::Argv, exact.clone())],
                &record_of,
            )
            .is_none(),
        "the right bytes under the wrong tag are not an image slot"
    );

    assert!(
        prepared
            .account_images(
                &[OverlayEntry::of(
                    9,
                    OpaqueFieldTag::ApplyArtifactImage,
                    exact
                )],
                &record_of,
            )
            .is_none(),
        "the right bytes against the wrong record enrich a different row"
    );

    assert!(
        prepared.account_images(&[], &record_of).is_none(),
        "an empty region cannot account for an assignment"
    );
}

#[test]
fn a_bypass_permit_records_the_bypass_and_never_the_required_route() {
    // The two routes reach one permit and the permit remembers WHICH. A bypass that recorded
    // itself as required publication would put a policy word in the durable that no publication
    // backs.
    let mut ids = Counter(0);
    let (ready, target) = session(&mut ids);
    let prepared = match ready.prepare_intent(
        vec![assignment(target, 0)],
        ReceiptPolicyWitness::configured_bypass(),
    ) {
        Ok(prepared) => prepared,
        Err(refusal) => panic!("a well-formed assignment should prepare: {refusal:?}"),
    };
    let session_id = prepared.session();
    let gate = IntentPublicationGate::ConfiguredBypass(ConfiguredReceiptBypass::configured());
    assert_eq!(gate.policy(), RecordedApplyPolicy::ConfiguredBypass);

    let dispatched = gate.permit(prepared).spend();
    assert_eq!(
        dispatched.policy(),
        RecordedApplyPolicy::ConfiguredBypass,
        "the spent phase carries the route that authorized it"
    );
    assert_eq!(
        dispatched.session(),
        session_id,
        "and the session whose authority it spent"
    );
    assert!(
        dispatched.declares(AssignmentOrdinal::of(0)),
        "the intent declared assignment zero"
    );
    assert!(
        !dispatched.declares(AssignmentOrdinal::of(1)),
        "and declared nothing else, so an outcome cannot name a second target"
    );
}

#[test]
fn only_a_durable_failure_narrows_out_of_the_post_dispatch_set() {
    // The asymmetry IS the type: six arms answer `None` and the seventh is the only one that
    // reaches the continue path, so a caller cannot widen a match into swallowing a lost host.
    assert_eq!(
        PostDispatchFailure::DurableOnly(DurableFailure::Sink).durable_only(),
        Some(DurableFailure::Sink),
        "a sink that declined is a durable failure and nothing more"
    );
    assert_eq!(
        PostDispatchFailure::ExecutionIntegrity(ExecutionIntegrityFailure).durable_only(),
        None,
        "not knowing what executed never narrows to a logging problem"
    );
}
