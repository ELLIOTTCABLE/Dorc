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
    ApplyDestination, ApplySessionReady, DurableFailure, ExecutionIntegrityFailure,
    IntentPreparationRefusal, IntentPublicationMismatch, PendingApplyAssignment, PendingOrigins,
    PlanOriginOccurrence, PostDispatchFailure, PreparedApplyIntent, PublicationThrough,
    REQUIRED_PLACEMENT_DIGEST_DOMAIN, ReadyApplyTarget, ReceiptPolicyWitness,
    RequiredPlacementLanding, ResolvedApplyContext, ResolvedAxis,
};
use dorc_receipt::ids::{
    ApplyGenerationId, ApplyIntentId, ApplySessionId, PlanReceiptId, PresentedPlanId,
    ReadyApplyTargetId, ReceiptId, ReceiptIdSource, Sha256Digest,
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

fn entered(text: &str) -> ResolvedAxis {
    ResolvedAxis::Established(text.to_owned())
}

fn context(destination: &str) -> ResolvedApplyContext {
    ResolvedApplyContext::of(
        ApplyDestination::addressed(destination.to_owned()),
        entered("root"),
        entered("host"),
        entered("/root"),
        entered("inherited"),
        entered("session"),
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

/// One fresh prepared intent under `policy`, over one session and one assignment.
///
/// Rebuilt per use rather than shared, because every state past this one CONSUMES its
/// predecessor: an intent accounted once is gone, which is exactly the property under test.
fn prepared_under(ids: &mut Counter, policy: ReceiptPolicyWitness) -> PreparedApplyIntent {
    let (ready, target) = session(ids);
    match ready.prepare_intent(vec![assignment(target, 0)], policy) {
        Ok(prepared) => prepared,
        Err(refusal) => panic!("a well-formed assignment should prepare: {refusal:?}"),
    }
}

/// The canonical image bytes one prepared intent's only assignment carries.
fn only_image_bytes(prepared: &PreparedApplyIntent) -> Vec<u8> {
    let Some(only) = prepared.assignments().first() else {
        panic!("the assignment vector is non-empty by construction");
    };
    only.image().encode().to_vec()
}

/// The digest of the bytes this battery pretends to have sealed and placed.
///
/// One value on both sides, because the required route COMPARES them: a fixture answering a
/// different digest would be exercising the mismatch arm rather than the ordinary route.
fn sealed_digest() -> Sha256Digest {
    Sha256Digest::over(REQUIRED_PLACEMENT_DIGEST_DOMAIN, b"the sealed document")
}

/// A landing a fixture placement reports. Carries no authority of its own — the publication is
/// minted inside `publish_through`, which is the point.
fn landing() -> RequiredPlacementLanding {
    RequiredPlacementLanding::of(sealed_digest(), "required-local-v1")
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
        only.context(),
        &context("web1.example.net"),
        "the record carries the standup's own answer for this target, whole"
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
    //
    // Each arm rebuilds the intent because accounting CONSUMES it: an intent that survived a
    // refused accounting would be an intent a caller could try again with different entries.
    let mut ids = Counter(0);
    let record_of = |_: AssignmentOrdinal| Some(7_u64);
    let required = ReceiptPolicyWitness::required_rich;

    let intent = prepared_under(&mut ids, required());
    let exact = only_image_bytes(&intent);
    assert!(
        intent
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
        prepared_under(&mut ids, required())
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
        prepared_under(&mut ids, required())
            .account_images(
                &[OverlayEntry::of(7, OpaqueFieldTag::Argv, exact.clone())],
                &record_of,
            )
            .is_none(),
        "the right bytes under the wrong tag are not an image slot"
    );

    assert!(
        prepared_under(&mut ids, required())
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
        prepared_under(&mut ids, required())
            .account_images(&[], &record_of)
            .is_none(),
        "an empty region cannot account for an assignment"
    );
}

#[test]
fn a_permit_carries_the_session_and_the_declared_set_of_the_intent_that_was_published() {
    // The permit is minted from the publication and takes no second argument, so the session and
    // the declared assignment set it reports are this intent's own by construction rather than
    // by a caller having passed the matching pair.
    let mut ids = Counter(0);
    let intent = prepared_under(&mut ids, ReceiptPolicyWitness::required_rich());
    let session_id = intent.session();
    let exact = only_image_bytes(&intent);
    let accounted = intent
        .account_images(
            &[OverlayEntry::of(
                7,
                OpaqueFieldTag::ApplyArtifactImage,
                exact,
            )],
            &|_: AssignmentOrdinal| Some(7_u64),
        )
        .expect("the region carries the image's own bytes");

    let id = ApplyIntentId::mint(&mut ids);
    let (published, filed) = accounted
        .publish_through(id, sealed_digest(), |handed| {
            Ok::<_, ()>((landing(), handed))
        })
        .expect("a placement that answered clears the publication");
    assert_eq!(
        filed.hex(),
        id.hex(),
        "the placement is handed the identity the publication records, not one of its own"
    );
    assert_eq!(published.id().hex(), id.hex());

    let dispatched = published.permit().spend();
    assert_eq!(
        dispatched.policy(),
        RecordedApplyPolicy::RequiredRich,
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
fn a_placement_that_refuses_produces_no_publication_and_therefore_no_permit() {
    // The other half of the route: a refusal comes back in the placement's OWN words and there
    // is no partially-published value left over for a caller to salvage a permit from.
    let mut ids = Counter(0);
    let intent = prepared_under(&mut ids, ReceiptPolicyWitness::required_rich());
    let exact = only_image_bytes(&intent);
    let accounted = intent
        .account_images(
            &[OverlayEntry::of(
                7,
                OpaqueFieldTag::ApplyArtifactImage,
                exact,
            )],
            &|_: AssignmentOrdinal| Some(7_u64),
        )
        .expect("the region carries the image's own bytes");

    let id = ApplyIntentId::mint(&mut ids);
    let refused = accounted
        .publish_through(id, sealed_digest(), |_| {
            Err::<(RequiredPlacementLanding, ()), _>("the store declined")
        })
        .err();
    assert_eq!(
        refused,
        Some(PublicationThrough::Placement("the store declined")),
        "the placement's own refusal survives rather than becoming a generic mismatch"
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

#[test]
fn a_landing_over_other_bytes_than_the_ones_sealed_clears_no_gate() {
    // The one thing the placement's answer is still CHECKED for. A store that filed some other
    // document — or reported a digest it did not compute over what it was handed — would
    // otherwise produce a publication naming bytes nobody wrote, and the outcome document would
    // point a later reader at them.
    let mut ids = Counter(0);
    let intent = prepared_under(&mut ids, ReceiptPolicyWitness::required_rich());
    let exact = only_image_bytes(&intent);
    let accounted = intent
        .account_images(
            &[OverlayEntry::of(
                7,
                OpaqueFieldTag::ApplyArtifactImage,
                exact,
            )],
            &|_: AssignmentOrdinal| Some(7_u64),
        )
        .expect("the region carries the image's own bytes");

    let elsewhere = RequiredPlacementLanding::of(
        Sha256Digest::over(REQUIRED_PLACEMENT_DIGEST_DOMAIN, b"some other document"),
        "required-local-v1",
    );
    assert_ne!(
        elsewhere.document_digest(),
        sealed_digest(),
        "the two fixture documents really do digest differently"
    );
    assert_eq!(
        accounted
            .publish_through(ApplyIntentId::mint(&mut ids), sealed_digest(), |_| {
                Ok::<_, ()>((elsewhere, ()))
            })
            .err(),
        Some(PublicationThrough::Mismatch(
            IntentPublicationMismatch::LandingNamesOtherBytes
        )),
        "a landing over other bytes is a refusal, not a publication"
    );
}

/// A publication and the intent it is for are ONE value, so the pairing has no failure mode.
///
/// The atomicity `30Rb:critical-type-effect-map` demands is no longer "the mint refuses a bad
/// pairing" — a mint that checks a pairing can be handed the wrong four values and has to notice.
/// It is that a wrong pairing is unspellable: accounting consumes the intent, the publication owns
/// the accounting, and `permit` takes no second argument. The compile-fail pins in
/// `receipt/src/lib.rs` are where that unspellability is asserted, because a runtime test cannot
/// express code that does not compile.
///
/// What remains checkable at runtime is the ONE thing still decided from a value: the policy.
#[test]
fn a_bypass_policy_intent_cannot_be_published_through_the_required_route() {
    // A caller holding a real placement and an intent prepared under the bypass word must not be
    // able to assemble a publication out of them: the route records `required-rich`, and an
    // intent wearing the other word would be a false claim about what authorized the dispatch.
    let mut ids = Counter(0);
    let intent = prepared_under(&mut ids, ReceiptPolicyWitness::configured_bypass());
    let exact = only_image_bytes(&intent);
    let accounted = intent
        .account_images(
            &[OverlayEntry::of(
                7,
                OpaqueFieldTag::ApplyArtifactImage,
                exact,
            )],
            &|_: AssignmentOrdinal| Some(7_u64),
        )
        .expect("the region carries the image's own bytes");

    let mut placement_was_called = false;
    let refused = accounted
        .publish_through(ApplyIntentId::mint(&mut ids), sealed_digest(), |_| {
            placement_was_called = true;
            Ok::<_, ()>((landing(), ()))
        })
        .err();
    assert_eq!(
        refused,
        Some(PublicationThrough::Mismatch(
            IntentPublicationMismatch::PolicyIsNotRequired
        )),
        "the required route is for the required policy and no other"
    );
    assert!(
        !placement_was_called,
        "the policy is judged BEFORE the placement, so a refused intent writes nothing"
    );
}
