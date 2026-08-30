#![expect(
    clippy::panic,
    reason = "clippy.toml's allow-panic-in-tests reaches `#[test]` functions in this crate but \
              not the plain fixture-building helpers beside them, and threading a Result through \
              fixtures that must succeed buys nothing"
)]
//! The live → recorded projection for the two apply-side species, pinned where it can be
//! silently wrong.
//!
//! Three of these guard failures that VALIDATE. A document whose assignment map is off by one, a
//! document whose two context slots came from different assignments, and a document whose site
//! rows name an assignment nobody authorized all parse cleanly and read plausibly; only a test
//! that looks at the correspondence can tell.

use dorc_receipt::apply::OriginatingPlans;
use dorc_receipt::context::{RecordedApplyContext, RecordedAxis};
use dorc_receipt::dispatch::{
    ApplyDestination, ApplySessionReady, MutationDispatched, PendingApplyAssignment,
    PendingOrigins, PlanOriginOccurrence, PreparedApplyIntent, ReadyApplyTarget,
    ReceiptPolicyWitness, RequiredPlacementLanding, ResolvedApplyContext, ResolvedAxis,
};
use dorc_receipt::ids::{
    ApplyGenerationId, ApplyIntentId, ApplySessionId, PlanReceiptId, PresentedPlanId,
    ReadyApplyTargetId, ReceiptId, ReceiptIdSource,
};
use dorc_receipt::image::{ApplyArtifactImage, ApplyEntryBytes};
use dorc_receipt::limits::{ByteLimit, ReceiptLimits};
use dorc_receipt::project::{
    ApplyInvocation, ApplyOutcomeReport, ApplyProjectionRefusal, ApplySiteReport, InvocationTarget,
    project_apply_intent, project_apply_outcome,
};
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::report::ByteAgreement;
use dorc_receipt::rows::{
    AssignmentOrdinal, OriginOrdinal, RecordedLeaf, RecordedMember, RecordedSite,
};
use dorc_receipt::tokens::{
    ClosedToken, RecordedDurableState, RecordedInvocationMode, RecordedOriginState,
    RecordedSiteStatus, RecordedTerminalState,
};
use dorc_receipt::{RecordKind, RecordedInfluence};

/// A counting identity source. The production edge fills these from the operating system.
struct Counter(u8);

impl ReceiptIdSource for Counter {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

/// Six DISTINCT answers. Distinctness is what lets a transposed axis fail: with one value
/// everywhere, a writer that swapped two of them would agree with a reader that swapped them
/// back.
fn context(destination: &str) -> ResolvedApplyContext {
    ResolvedApplyContext::of(
        ApplyDestination::addressed(destination.to_owned()),
        entered("deploy"),
        entered("netns-blue"),
        entered("/srv/app"),
        entered("inherited-minus-ssh"),
        entered("agent-forwarded"),
    )
}

fn entered(text: &str) -> ResolvedAxis {
    ResolvedAxis::Established(text.to_owned())
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

fn authored() -> RecordedInfluence {
    RecordedInfluence::of_token(Some("authored-before-contact"))
}

fn influenced() -> RecordedInfluence {
    RecordedInfluence::of_token(Some("host-influenced"))
}

fn invocation() -> ApplyInvocation {
    ApplyInvocation::of(
        RecordedInvocationMode::Apply,
        Some(17),
        InvocationTarget::Spelled(b"web1.example.net".to_vec()),
        1,
        authored(),
    )
}

/// One session over `targets` destinations, and the identities its assignments name.
fn session(
    ids: &mut Counter,
    destinations: &[&str],
) -> (ApplySessionReady, Vec<ReadyApplyTargetId>) {
    let mut targets = Vec::new();
    let mut names = Vec::new();
    for destination in destinations {
        let id = ReadyApplyTargetId::mint(ids);
        names.push(id);
        targets.push(ReadyApplyTarget::of(id, context(destination)));
    }
    match ApplySessionReady::of(
        ApplySessionId::mint(ids),
        ApplyGenerationId::mint(ids),
        targets,
    ) {
        Ok(ready) => (ready, names),
        Err(refusal) => panic!("a well-formed standup should close: {refusal:?}"),
    }
}

/// One assignment against a fresh single-target session, with no originating plan.
fn one_assignment(ids: &mut Counter) -> PreparedApplyIntent {
    let (ready, names) = session(ids, &["web1.example.net"]);
    let target = *names.first().unwrap_or_else(|| panic!("one target"));
    let assignment = PendingApplyAssignment::of(
        AssignmentOrdinal::of(0),
        target,
        image(b"#!/bin/sh\napt-get install -y nginx\n"),
        PendingOrigins::Unavailable,
    );
    match ready.prepare_intent(vec![assignment], ReceiptPolicyWitness::required_rich()) {
        Ok(prepared) => prepared,
        Err(refusal) => panic!("a well-formed assignment should prepare: {refusal:?}"),
    }
}

/// The phase an outcome projection is checked against.
///
/// Driven through the one real route — account, publish, permit, spend — with a modelled
/// placement, because there is no shortcut into a permit and this battery should not pretend
/// there is. What it models is the STORE answering; everything above that is production code.
fn dispatched(ids: &mut Counter) -> MutationDispatched {
    let intent = one_assignment(ids);
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };
    let Some(accounted) =
        intent.account_images(projected.details(), &|ordinal| projected.record_of(ordinal))
    else {
        panic!("the projection carries every assignment's own image")
    };
    // One digest on both sides: the required route COMPARES what the placement reports against
    // what the caller sealed, so a fixture must answer the same value to drive the ordinary route
    // rather than the mismatch arm.
    let sealed = dorc_receipt::ids::Sha256Digest::over(
        dorc_receipt::dispatch::REQUIRED_PLACEMENT_DIGEST_DOMAIN,
        b"the modelled document",
    );
    match accounted.publish_through(ApplyIntentId::mint(ids), sealed, |_| {
        Ok::<_, ()>((
            RequiredPlacementLanding::of(sealed, "required-local-v1"),
            (),
        ))
    }) {
        Ok((published, ())) => published.permit().spend(),
        Err(through) => panic!("a modelled placement clears the route: {through:?}"),
    }
}

#[test]
fn an_intent_projects_the_order_its_own_model_re_emits() {
    // The load-bearing agreement: a detail entry is keyed by its record's POSITION, so a walk
    // emitting in one order while the model re-emitted in another would enrich whichever row
    // shared the integer, with the document still validating cleanly.
    let mut ids = Counter(0);
    let intent = one_assignment(&mut ids);
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };

    // Non-vacuity floor: a document with no assignment row would satisfy the comparison below
    // while proving the walk carried nothing.
    assert!(
        projected
            .records()
            .iter()
            .any(|record| record.kind() == RecordKind::ApplyAssignment),
        "the projection carried no assignment, so this proves nothing"
    );
    let reemitted = match projected.model().to_records() {
        Ok(records) => records,
        Err(refusal) => panic!("the model re-serializes: {refusal:?}"),
    };
    assert_eq!(reemitted, projected.records());
}

#[test]
fn the_recorded_map_is_the_one_the_image_accounting_answers_to() {
    // THE reason the projection hands back a map at all. The capability is a byte comparison
    // keyed by record, so a map naming the wrong row makes the accounting refuse — and a caller
    // that guessed the offset instead would mint a capability against a row it never checked.
    // Each arm rebuilds the intent, because accounting CONSUMES one: a caller cannot retry an
    // accounting that refused, which is why the negatives below cannot share a fixture.
    let mut ids = Counter(0);
    let intent = one_assignment(&mut ids);
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };

    assert!(
        intent
            .account_images(projected.details(), &|ordinal| projected.record_of(ordinal))
            .is_some(),
        "the region carries this assignment's own canonical bytes at the recorded row"
    );

    let recorded = projected
        .record_of(AssignmentOrdinal::of(0))
        .unwrap_or_else(|| panic!("the assignment was emitted"));
    assert!(
        one_assignment(&mut ids)
            .account_images(projected.details(), &|_| Some(recorded.saturating_add(1)))
            .is_none(),
        "a map naming a neighbouring row accounts for nothing"
    );
    assert!(
        one_assignment(&mut ids)
            .account_images(projected.details(), &|_| None)
            .is_none(),
        "an assignment the map cannot place accounts for nothing"
    );
}

#[test]
fn an_assignments_destination_and_its_remaining_axes_ride_one_record() {
    // The two slots are recombined by a reader, so this asks the question a reader would: does
    // record N's target-name and record N's apply-context describe the SAME standup answer. Six
    // distinct fixture values are what make a transposition visible rather than vacuous.
    let mut ids = Counter(0);
    let intent = one_assignment(&mut ids);
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };
    let record = projected
        .record_of(AssignmentOrdinal::of(0))
        .unwrap_or_else(|| panic!("the assignment was emitted"));

    // Asked as a VERDICT: a detail is plaintext-bearing, so a battery proves exactness by
    // comparing against the encoding it expects rather than reading the bytes back out.
    let agrees = |tag: OpaqueFieldTag, expected: &[u8]| {
        projected
            .details()
            .iter()
            .find(|entry| entry.record() == record && entry.tag() == tag)
            .map(|entry| entry.agrees_with(expected))
    };
    assert_eq!(
        agrees(OpaqueFieldTag::TargetName, b"web1.example.net"),
        Some(ByteAgreement::Identical)
    );

    let expected_context = RecordedApplyContext::of(
        RecordedAxis::Established(b"deploy".to_vec()),
        RecordedAxis::Established(b"netns-blue".to_vec()),
        RecordedAxis::Established(b"/srv/app".to_vec()),
        RecordedAxis::Established(b"inherited-minus-ssh".to_vec()),
        RecordedAxis::Established(b"agent-forwarded".to_vec()),
    );
    assert_eq!(
        agrees(OpaqueFieldTag::ApplyContext, &expected_context.encode()),
        Some(ByteAgreement::Identical),
        "each axis arrives carrying the answer the standup gave for IT"
    );
    // Six DISTINCT fixture answers, so a transposed pair really would differ: an expectation
    // built from one repeated value would agree with a writer that swapped two of them.
    let transposed = RecordedApplyContext::of(
        RecordedAxis::Established(b"netns-blue".to_vec()),
        RecordedAxis::Established(b"deploy".to_vec()),
        RecordedAxis::Established(b"/srv/app".to_vec()),
        RecordedAxis::Established(b"inherited-minus-ssh".to_vec()),
        RecordedAxis::Established(b"agent-forwarded".to_vec()),
    );
    assert_eq!(
        agrees(OpaqueFieldTag::ApplyContext, &transposed.encode()),
        Some(ByteAgreement::Differing),
        "and the comparison would notice a swap"
    );
}

#[test]
fn a_session_that_entered_no_context_records_that_and_not_five_empty_answers() {
    // The thin session's whole claim, at the seat that writes it. Five unentered axes must reach
    // the region as unentered — a projection that flattened them into empty values would record
    // "established as nothing", which is a statement about the host nobody made.
    let mut ids = Counter(0);
    let target = ReadyApplyTargetId::mint(&mut ids);
    let thin = ResolvedApplyContext::of(
        ApplyDestination::addressed("web1.example.net".to_owned()),
        ResolvedAxis::NotEstablished,
        ResolvedAxis::NotEstablished,
        ResolvedAxis::NotEstablished,
        ResolvedAxis::NotEstablished,
        ResolvedAxis::NotEstablished,
    );
    let ready = match ApplySessionReady::of(
        ApplySessionId::mint(&mut ids),
        ApplyGenerationId::mint(&mut ids),
        vec![ReadyApplyTarget::of(target, thin)],
    ) {
        Ok(ready) => ready,
        Err(refusal) => panic!("a thin standup should close: {refusal:?}"),
    };
    let intent = match ready.prepare_intent(
        vec![PendingApplyAssignment::of(
            AssignmentOrdinal::of(0),
            target,
            image(b"#!/bin/sh\n:\n"),
            PendingOrigins::Unavailable,
        )],
        ReceiptPolicyWitness::configured_bypass(),
    ) {
        Ok(intent) => intent,
        Err(refusal) => panic!("a thin assignment should prepare: {refusal:?}"),
    };
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a thin intent projects: {refusal:?}"),
        };
    let record = projected
        .record_of(AssignmentOrdinal::of(0))
        .unwrap_or_else(|| panic!("the assignment was emitted"));
    let agrees = |tag: OpaqueFieldTag, expected: &[u8]| {
        projected
            .details()
            .iter()
            .find(|entry| entry.record() == record && entry.tag() == tag)
            .map(|entry| entry.agrees_with(expected))
    };
    let unentered = RecordedApplyContext::of(
        RecordedAxis::NotEstablished,
        RecordedAxis::NotEstablished,
        RecordedAxis::NotEstablished,
        RecordedAxis::NotEstablished,
        RecordedAxis::NotEstablished,
    );
    assert_eq!(
        agrees(OpaqueFieldTag::ApplyContext, &unentered.encode()),
        Some(ByteAgreement::Identical),
        "a thin assignment still captures its context, and every axis says unentered"
    );
    // The destination is a different question and is still answered: a thin session knows where
    // it addressed, and only the five context axes are unentered.
    assert_eq!(
        agrees(OpaqueFieldTag::TargetName, b"web1.example.net"),
        Some(ByteAgreement::Identical)
    );
}

#[test]
fn origins_reach_their_own_assignment_and_the_row_states_which_state_that_is() {
    // Two assignments, one composing a plan twice and one composing none. The model's own
    // closure check is what would catch a mis-filed origin, so this drives BOTH states through
    // one document rather than asserting a count.
    let mut ids = Counter(0);
    let (ready, names) = session(&mut ids, &["web1.example.net", "web2.example.net"]);
    let receipt = PlanReceiptId::mint(&mut ids);
    let presented = PresentedPlanId::of_hex(&"c".repeat(64))
        .unwrap_or_else(|| panic!("the fixture surface identity parses"));
    let composed = PendingOrigins::known(vec![
        PlanOriginOccurrence::of(OriginOrdinal::of(0), receipt, presented),
        PlanOriginOccurrence::of(OriginOrdinal::of(1), receipt, presented),
    ])
    .unwrap_or_else(|| panic!("a non-empty occurrence list"));

    let assignments = vec![
        PendingApplyAssignment::of(
            AssignmentOrdinal::of(0),
            *names.first().unwrap_or_else(|| panic!("two targets")),
            image(b"#!/bin/sh\n:\n"),
            composed,
        ),
        PendingApplyAssignment::of(
            AssignmentOrdinal::of(1),
            *names.get(1).unwrap_or_else(|| panic!("two targets")),
            image(b"#!/bin/sh\n: \n"),
            PendingOrigins::Unavailable,
        ),
    ];
    let intent = match ready.prepare_intent(assignments, ReceiptPolicyWitness::required_rich()) {
        Ok(intent) => intent,
        Err(refusal) => panic!("a well-formed pair should prepare: {refusal:?}"),
    };
    let projected =
        match project_apply_intent(&intent, &invocation(), influenced(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };

    let model = projected.model();
    assert_eq!(model.intent().origin_state(), RecordedOriginState::Known);
    let first = model
        .assignments()
        .first()
        .unwrap_or_else(|| panic!("two assignments"));
    let second = model
        .assignments()
        .get(1)
        .unwrap_or_else(|| panic!("two assignments"));
    assert_eq!(
        first.origins().len(),
        2,
        "a plan composed twice is retained twice"
    );
    assert!(matches!(second.origins(), &OriginatingPlans::Unavailable));
    assert_eq!(
        first.assignment().account().token(),
        "host-influenced",
        "the standup's own account reaches its rows"
    );
}

#[test]
fn a_prepared_intent_with_no_origin_records_the_unavailable_state() {
    let mut ids = Counter(0);
    let intent = one_assignment(&mut ids);
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };
    assert_eq!(
        projected.model().intent().origin_state(),
        RecordedOriginState::Unavailable
    );
}

fn site(assignment: u32, leaf: u32, stdout: Option<Vec<u8>>) -> ApplySiteReport {
    ApplySiteReport::of(
        AssignmentOrdinal::of(assignment),
        RecordedSite::of(RecordedLeaf::of(leaf), Some(RecordedMember::of(0))),
        RecordedSiteStatus::Ran,
        Some(0),
        stdout,
        None,
        influenced(),
    )
}

#[test]
fn an_outcome_naming_an_assignment_the_intent_never_declared_refuses_at_that_ordinal() {
    // Pinned to the EXACT refusal and the exact ordinal. The alternative failures in this family
    // — a grammar refusal on the same row, a model refusal on the declared count — look alike
    // from outside, and recording the row anyway would attribute execution to a target nobody
    // authorized.
    let mut ids = Counter(0);
    let phase = dispatched(&mut ids);
    assert!(phase.declares(AssignmentOrdinal::of(0)));

    let report = ApplyOutcomeReport::of(
        ApplyIntentId::of_hex(&"a".repeat(64))
            .unwrap_or_else(|| panic!("the fixture identity parses")),
        RecordedTerminalState::Complete,
        RecordedDurableState::Published,
        vec![site(4, 0, None)],
        influenced(),
    );
    assert_eq!(
        project_apply_outcome(&phase, &report, &invocation(), &ReceiptLimits::V1).err(),
        Some(ApplyProjectionRefusal::UndeclaredAssignment { assignment: 4 })
    );
}

#[test]
fn an_outcome_projects_the_order_its_own_model_re_emits_and_numbers_its_sites_from_zero() {
    let mut ids = Counter(0);
    let phase = dispatched(&mut ids);
    let intent = ApplyIntentId::of_hex(&"b".repeat(64))
        .unwrap_or_else(|| panic!("the fixture identity parses"));
    let report = ApplyOutcomeReport::of(
        intent,
        RecordedTerminalState::CommandFailed,
        RecordedDurableState::Published,
        vec![site(0, 3, Some(b"ok\n".to_vec())), site(0, 7, None)],
        influenced(),
    );
    let projected = match project_apply_outcome(&phase, &report, &invocation(), &ReceiptLimits::V1)
    {
        Ok(projected) => projected,
        Err(refusal) => panic!("a declared outcome projects: {refusal:?}"),
    };

    let model = projected.model();
    assert_eq!(model.outcome().intent(), intent.hex().as_str());
    assert_eq!(model.outcome().sites(), 2);
    assert_eq!(model.sites().len(), 2);
    for (position, row) in model.sites().iter().enumerate() {
        assert_eq!(
            row.ordinal().get(),
            u32::try_from(position).unwrap_or(u32::MAX)
        );
    }
    let reemitted = match model.to_records() {
        Ok(records) => records,
        Err(refusal) => panic!("the model re-serializes: {refusal:?}"),
    };
    assert_eq!(reemitted, projected.records());
}

#[test]
fn a_site_holding_no_output_records_unavailable_and_carries_no_entry() {
    // `unavailable` says the run held nothing, which is a different statement from a projection
    // declining to carry what it has — and a slot marked captured with no entry produces a
    // document this crate's own reader refuses, so the two halves are asserted together.
    let mut ids = Counter(0);
    let phase = dispatched(&mut ids);
    let report = ApplyOutcomeReport::of(
        ApplyIntentId::of_hex(&"d".repeat(64))
            .unwrap_or_else(|| panic!("the fixture identity parses")),
        RecordedTerminalState::Complete,
        RecordedDurableState::NotAttempted,
        vec![site(0, 1, Some(b"listening\n".to_vec()))],
        influenced(),
    );
    let projected = match project_apply_outcome(&phase, &report, &invocation(), &ReceiptLimits::V1)
    {
        Ok(projected) => projected,
        Err(refusal) => panic!("a declared outcome projects: {refusal:?}"),
    };

    let row = projected
        .model()
        .sites()
        .first()
        .unwrap_or_else(|| panic!("one site"));
    assert_eq!(row.channels().stdout().token(), "captured");
    assert_eq!(row.channels().stderr().token(), "unavailable");

    let tags: Vec<OpaqueFieldTag> = projected
        .details()
        .iter()
        .filter(|entry| matches!(entry.tag(), OpaqueFieldTag::Stdout | OpaqueFieldTag::Stderr))
        .map(dorc_receipt::overlay::OverlayEntry::tag)
        .collect();
    assert_eq!(
        tags,
        vec![OpaqueFieldTag::Stdout],
        "the held channel rides its slot and the absent one carries nothing"
    );
}

#[test]
fn a_multi_target_invocation_names_no_target_of_its_own() {
    // The invocation row states what the INVOCATION spelled. Naming one assignment's resolved
    // destination there would be a claim the invocation did not make, and the row says
    // `unavailable` rather than picking one.
    let mut ids = Counter(0);
    let (ready, names) = session(&mut ids, &["web1.example.net", "web2.example.net"]);
    let assignments = vec![
        PendingApplyAssignment::of(
            AssignmentOrdinal::of(0),
            *names.first().unwrap_or_else(|| panic!("two targets")),
            image(b"#!/bin/sh\n:\n"),
            PendingOrigins::Unavailable,
        ),
        PendingApplyAssignment::of(
            AssignmentOrdinal::of(1),
            *names.get(1).unwrap_or_else(|| panic!("two targets")),
            image(b"#!/bin/sh\n: \n"),
            PendingOrigins::Unavailable,
        ),
    ];
    let intent = match ready.prepare_intent(assignments, ReceiptPolicyWitness::required_rich()) {
        Ok(intent) => intent,
        Err(refusal) => panic!("a well-formed pair should prepare: {refusal:?}"),
    };
    let fleet = ApplyInvocation::of(
        RecordedInvocationMode::Apply,
        None,
        InvocationTarget::NotOne,
        1,
        authored(),
    );
    let projected = match project_apply_intent(&intent, &fleet, authored(), &ReceiptLimits::V1) {
        Ok(projected) => projected,
        Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
    };
    assert_eq!(
        projected.model().invocation().target().token(),
        "unavailable"
    );
    assert!(
        projected.details().iter().all(|entry| entry.record() != 0),
        "an invocation naming no target carries no target value"
    );
    assert_eq!(
        projected.model().assignments().len(),
        2,
        "both assignments still record their own resolved answers"
    );
}

#[test]
fn a_prepared_intent_records_the_policy_that_prepared_it() {
    // The word a durable carries about which route authorized the apply. A bypass recorded as
    // required publication would put a claim in the document that no publication funded.
    let mut ids = Counter(0);
    let (ready, names) = session(&mut ids, &["web1.example.net"]);
    let assignment = PendingApplyAssignment::of(
        AssignmentOrdinal::of(0),
        *names.first().unwrap_or_else(|| panic!("one target")),
        image(b"#!/bin/sh\n:\n"),
        PendingOrigins::Unavailable,
    );
    let intent =
        match ready.prepare_intent(vec![assignment], ReceiptPolicyWitness::configured_bypass()) {
            Ok(intent) => intent,
            Err(refusal) => panic!("a well-formed assignment should prepare: {refusal:?}"),
        };
    let projected =
        match project_apply_intent(&intent, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };
    assert_eq!(
        projected.model().intent().policy().token(),
        "configured-bypass"
    );

    let mut other = Counter(40);
    let required = one_assignment(&mut other);
    let projected =
        match project_apply_intent(&required, &invocation(), authored(), &ReceiptLimits::V1) {
            Ok(projected) => projected,
            Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
        };
    assert_eq!(
        projected.model().intent().policy().token(),
        "required-rich",
        "the two routes are told apart in the document, not only in the type"
    );
}

#[test]
fn a_channel_past_the_run_budget_records_omitted_and_carries_nothing() {
    // `omitted-limit` and `unavailable` are different statements — a bound stopped this carry,
    // versus the run holding nothing — and one document must be able to say each. The budget is
    // spent in row order, so the falsifiable claim is that the FIRST site keeps its output and
    // the second loses it, rather than both surviving or both being dropped.
    let narrow = ReceiptLimits {
        host_output_bytes: ByteLimit::of(6),
        ..ReceiptLimits::V1
    };
    let mut ids = Counter(0);
    let phase = dispatched(&mut ids);
    let report = ApplyOutcomeReport::of(
        ApplyIntentId::of_hex(&"e".repeat(64))
            .unwrap_or_else(|| panic!("the fixture identity parses")),
        RecordedTerminalState::Complete,
        RecordedDurableState::Published,
        vec![
            site(0, 1, Some(b"123456".to_vec())),
            site(0, 2, Some(b"7".to_vec())),
        ],
        influenced(),
    );
    let projected = match project_apply_outcome(&phase, &report, &invocation(), &narrow) {
        Ok(projected) => projected,
        Err(refusal) => panic!("a declared outcome projects: {refusal:?}"),
    };

    let rows = projected.model().sites();
    let first = rows.first().unwrap_or_else(|| panic!("two sites"));
    let second = rows.get(1).unwrap_or_else(|| panic!("two sites"));
    assert_eq!(first.channels().stdout().token(), "captured");
    assert_eq!(second.channels().stdout().token(), "omitted-limit");
    assert_eq!(
        second.channels().stderr().token(),
        "unavailable",
        "a channel the run never held still says so, beside one a bound stopped"
    );

    let carried: Vec<ByteAgreement> = projected
        .details()
        .iter()
        .filter(|entry| entry.tag() == OpaqueFieldTag::Stdout)
        .map(|entry| entry.agrees_with(b"123456"))
        .collect();
    assert_eq!(
        carried,
        vec![ByteAgreement::Identical],
        "the omitted channel carries no entry at all, and the one that rode is exact"
    );
}
