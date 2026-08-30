//! Graph correlation over fixture documents.
//!
//! The documents here are signed by an INERT deterministic stand-in, not by the real signature
//! implementation, which lives in the sibling crate. That keeps this corpus inside the pure
//! crate and lets it exercise the whole read path — bound, locate, check, parse, seal — instead
//! of hand-building reader states. It is fixture identity material: it is structurally test-only
//! because it lives in a test target, and the crate-boundary walk asserts that rather than
//! trusting it.

#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "spike/clippy.toml's allow-*-in-tests keys reach the #[test] functions of an \
              integration-test crate but not the plain helper functions beside them, which is \
              what these files are largely made of; the file-top expect is the documented answer"
)]

use dorc_receipt::ReceiptLimits;
use dorc_receipt::RefusalReason;
use dorc_receipt::apply::{RecordedApplyAssignment, RecordedApplyIntentRow, RecordedPlanOrigin};
use dorc_receipt::capability::PublicationGrade;
use dorc_receipt::capability::{
    ReceiptSigner, ReceiptVerificationKey, ReceiptVerifier, VerificationKeyResolver,
};
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::graph::{GraphFinding, GraphSpecies, ReceiptEdge, ReceiptGraph};
use dorc_receipt::ids::{
    ApplyIntentId, ApplyOutcomeId, PlanReceiptId, Sha256Digest, SigningKeyId, from_hex_32,
};
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Species};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::outcome::{OutcomeAvailability, RecordedApplyOutcomeRow, RecordedSiteOutcome};
use dorc_receipt::plan::{RecordedSource, SourceSlots};
use dorc_receipt::reader::{PartialReceipt, read_plain};
use dorc_receipt::reingested::RecordedInfluence;
use dorc_receipt::report::RecordedDocumentId;
use dorc_receipt::rows::{
    AssignmentOrdinal, OriginOrdinal, RecordedInvocation, RecordedLeaf, RecordedRow, RecordedSite,
    SiteOutcomeOrdinal, SourceOrdinal,
};
use dorc_receipt::tokens::ClosedToken;
use dorc_receipt::tokens::{
    ImageState, OpaqueState, RecordedApplyPolicy, RecordedDurableState, RecordedInvocationMode,
    RecordedOriginState, RecordedSiteStatus, RecordedSourceClass, RecordedSourceRole,
    RecordedTerminalState,
};
use dorc_receipt::writer::DraftReceipt;

/// A deterministic stand-in for a signature. Two domain-separated digests over the exact body,
/// which is enough to be stable and to disagree when the body changes, and is not a signature.
fn inert_signature(body: &[u8]) -> [u8; 64] {
    let head = from_hex_32(&Sha256Digest::over("fixture-signature-head", body).hex()).unwrap();
    let tail = from_hex_32(&Sha256Digest::over("fixture-signature-tail", body).hex()).unwrap();
    let mut out = [0_u8; 64];
    out[..32].copy_from_slice(&head);
    out[32..].copy_from_slice(&tail);
    out
}

fn fixture_key_id() -> SigningKeyId {
    SigningKeyId::of_public_material(b"fixture-verification-material")
}

struct InertSigner;

impl ReceiptSigner for InertSigner {
    fn signing_key_id(&self) -> SigningKeyId {
        fixture_key_id()
    }
    fn sign(&self, body: &[u8]) -> [u8; 64] {
        inert_signature(body)
    }
}

struct InertKey;

impl ReceiptVerifier for InertKey {
    fn verify(&self, body: &[u8], signature: &[u8; 64]) -> bool {
        inert_signature(body) == *signature
    }
}

impl ReceiptVerificationKey for InertKey {
    fn signing_key_id(&self) -> SigningKeyId {
        fixture_key_id()
    }
}

struct PolicyNames(InertKey);

impl VerificationKeyResolver for PolicyNames {
    fn material(&self, _id: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
        Some(&self.0)
    }
}

fn digest_of(seed: char) -> String {
    std::iter::repeat_n(seed, 64).collect()
}

/// One identity, spelled distinctly per fixture so a collision is deliberate rather than lucky.
fn identity(tag: &str) -> String {
    Sha256Digest::over("fixture-document-identity", tag.as_bytes()).hex()
}

fn bytes_of<D: Species>(receipt_id: &str, records: Vec<SkeletonRecord>) -> Vec<u8> {
    let skeleton = Skeleton {
        receipt_id: receipt_id.to_owned(),
        order: ReceiptOrderToken::of_controller_millis(1_700_000_000_000),
        signing_key_id: fixture_key_id().hex(),
        encryption_key_id: None,
        records,
    };
    DraftReceipt::<D, Plain>::of(skeleton)
        .serialize()
        .expect("the fixture skeleton must serialize")
        .sign(&InertSigner)
        .bytes()
        .to_vec()
}

fn invocation(mode: RecordedInvocationMode) -> SkeletonRecord {
    RecordedInvocation::of(
        mode,
        None,
        OpaqueState::WithheldPlain,
        OpaqueState::WithheldPlain,
        1,
        RecordedInfluence::AuthoredBeforeContact,
    )
    .to_record()
    .unwrap()
}

/// A plan document. `flavour` varies the content without varying the identity, which is what
/// separates "the same document twice" from "two documents claiming one identity".
fn plan_bytes(tag: &str, flavour: char) -> Vec<u8> {
    let source = RecordedSource::of(
        SourceOrdinal::of(0),
        RecordedSourceRole::Book,
        digest_of(flavour),
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
    .unwrap();
    bytes_of::<PlanReceipt>(
        &identity(tag),
        vec![invocation(RecordedInvocationMode::Plan), source],
    )
}

/// An intent naming zero or more originating plans by their document identities.
fn intent_bytes(tag: &str, origins: &[&str]) -> Vec<u8> {
    let state = if origins.is_empty() {
        RecordedOriginState::Unavailable
    } else {
        RecordedOriginState::Known
    };
    let mut records = vec![
        invocation(RecordedInvocationMode::Apply),
        RecordedApplyIntentRow::of(
            digest_of('5'),
            digest_of('6'),
            RecordedApplyPolicy::RequiredRich,
            1,
            state,
            RecordedInfluence::AuthoredBeforeContact,
        )
        .to_record()
        .unwrap(),
        RecordedApplyAssignment::of(
            AssignmentOrdinal::of(0),
            OpaqueState::WithheldPlain,
            OpaqueState::WithheldPlain,
            digest_of('7'),
            ImageState::WithheldPlain,
            u32::try_from(origins.len()).unwrap(),
            RecordedInfluence::AuthoredBeforeContact,
        )
        .to_record()
        .unwrap(),
    ];
    for (ordinal, plan) in origins.iter().enumerate() {
        records.push(
            RecordedPlanOrigin::of(
                AssignmentOrdinal::of(0),
                OriginOrdinal::of(u32::try_from(ordinal).unwrap()),
                identity(plan),
                digest_of('c'),
                RecordedInfluence::AuthoredBeforeContact,
            )
            .to_record()
            .unwrap(),
        );
    }
    bytes_of::<ApplyIntent>(&identity(tag), records)
}

fn outcome_bytes(tag: &str, intent: &str, terminal: RecordedTerminalState) -> Vec<u8> {
    let records = vec![
        invocation(RecordedInvocationMode::Apply),
        RecordedApplyOutcomeRow::of(
            identity(intent),
            terminal,
            1,
            RecordedDurableState::Published,
            RecordedInfluence::HostInfluenced,
        )
        .to_record()
        .unwrap(),
        RecordedSiteOutcome::of(
            SiteOutcomeOrdinal::of(0),
            AssignmentOrdinal::of(0),
            RecordedSite::of(RecordedLeaf::of(1), None),
            RecordedSiteStatus::Ran,
            Some(0),
            dorc_receipt::outcome::RecordedChannels::of(
                OpaqueState::Uncollected,
                OpaqueState::Uncollected,
            ),
            RecordedInfluence::HostInfluenced,
        )
        .to_record()
        .unwrap(),
    ];
    bytes_of::<ApplyOutcome>(&identity(tag), records)
}

/// Which species a fixture is, so one loader can feed the graph in any order.
#[derive(Clone, Copy)]
enum Kind {
    Plan,
    Intent,
    Outcome,
}

fn feed(graph: &mut ReceiptGraph, documents: &[(Kind, Vec<u8>)]) {
    let limits = ReceiptLimits::V1;
    let resolver = PolicyNames(InertKey);
    // The fixture resolver IS this battery own keyset, so the word it reports is the one a
    // seat holding a validated keyset would report. It is a value the ingesting seat supplies,
    // never a property the read derived.
    let trust = dorc_receipt::tokens::RecordedSignerTrust::Trusted;
    for (kind, bytes) in documents {
        match kind {
            Kind::Plan => match read_plain::<PlanReceipt>(bytes.clone(), &limits, &resolver) {
                Ok(document) => graph.ingest_plan(&document, trust, bytes),
                Err(partial) => graph.ingest_partial(partial),
            },
            Kind::Intent => match read_plain::<ApplyIntent>(bytes.clone(), &limits, &resolver) {
                Ok(document) => graph.ingest_intent(&document, trust, bytes),
                Err(partial) => graph.ingest_partial(partial),
            },
            Kind::Outcome => match read_plain::<ApplyOutcome>(bytes.clone(), &limits, &resolver) {
                Ok(document) => graph.ingest_outcome(&document, trust, bytes),
                Err(partial) => graph.ingest_partial(partial),
            },
        }
    }
}

fn graph_of(documents: &[(Kind, Vec<u8>)]) -> ReceiptGraph {
    let mut graph = ReceiptGraph::new();
    feed(&mut graph, documents);
    graph
}

fn plan_id(tag: &str) -> PlanReceiptId {
    PlanReceiptId::of_hex(&identity(tag)).unwrap()
}

fn intent_id(tag: &str) -> ApplyIntentId {
    ApplyIntentId::of_hex(&identity(tag)).unwrap()
}

fn outcome_id(tag: &str) -> ApplyOutcomeId {
    ApplyOutcomeId::of_hex(&identity(tag)).unwrap()
}

/// The complete shape of a correlated set: how many of each node, and the EXACT edge and finding
/// lists.
///
/// Findings here are retentions plus a verdict, not refusals, so "a finding was recorded" is
/// satisfied by a correlator that recorded the wrong finding, the right finding for the wrong
/// pair, or one finding where two were owed. Every case below therefore pins the whole shape.
#[derive(Debug, PartialEq, Eq)]
struct Shape {
    plans: usize,
    intents: usize,
    outcomes: usize,
    collisions: usize,
    partials: usize,
    edges: Vec<ReceiptEdge>,
    findings: Vec<GraphFinding>,
}

impl Shape {
    /// The expected shape, with the two lists sorted so a case may list them in any order.
    fn expected(
        counts: (usize, usize, usize, usize, usize),
        mut edges: Vec<ReceiptEdge>,
        mut findings: Vec<GraphFinding>,
    ) -> Self {
        edges.sort();
        findings.sort();
        Self {
            plans: counts.0,
            intents: counts.1,
            outcomes: counts.2,
            collisions: counts.3,
            partials: counts.4,
            edges,
            findings,
        }
    }
}

fn shape_of(graph: &ReceiptGraph) -> Shape {
    Shape {
        plans: graph.plans().len(),
        intents: graph.intents().len(),
        outcomes: graph.outcomes().len(),
        collisions: graph.collisions().len(),
        partials: graph.partials().len(),
        edges: graph.edges(),
        findings: graph.findings(),
    }
}

fn plan_to(plan: &str, intent: &str) -> ReceiptEdge {
    ReceiptEdge::PlanToIntent {
        plan: plan_id(plan),
        intent: intent_id(intent),
    }
}

fn intent_to(intent: &str, outcome: &str) -> ReceiptEdge {
    ReceiptEdge::IntentToOutcome {
        intent: intent_id(intent),
        outcome: outcome_id(outcome),
    }
}

#[test]
fn the_fixture_read_path_reaches_a_sealed_document() {
    // The corpus is only correlating real reads if this holds: everything below goes through
    // bound, locate, signature check, parse, and seal.
    let graph = graph_of(&[(Kind::Plan, plan_bytes("p", 'a'))]);
    assert!(graph.faults().is_empty());
    assert_eq!(
        shape_of(&graph),
        Shape::expected((1, 0, 0, 0, 0), vec![], vec![])
    );
}

#[test]
fn one_plan_feeds_many_intents_and_each_intent_has_at_most_one_outcome() {
    // The shape the design draws: one plan, three applies, one of them never answered. The
    // unanswered apply contributes NO finding — an absent outcome is an availability reached by
    // correlation, not a fault in the record set.
    let documents = vec![
        (Kind::Plan, plan_bytes("p", 'a')),
        (Kind::Intent, intent_bytes("a1", &["p"])),
        (Kind::Intent, intent_bytes("a2", &["p"])),
        (Kind::Intent, intent_bytes("a3", &["p"])),
        (
            Kind::Outcome,
            outcome_bytes("o1", "a1", RecordedTerminalState::Complete),
        ),
        (
            Kind::Outcome,
            outcome_bytes("o3", "a3", RecordedTerminalState::CommandFailed),
        ),
    ];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (1, 3, 2, 0, 0),
            vec![
                plan_to("p", "a1"),
                plan_to("p", "a2"),
                plan_to("p", "a3"),
                intent_to("a1", "o1"),
                intent_to("a3", "o3"),
            ],
            vec![],
        )
    );
}

#[test]
fn one_intent_may_compose_many_presented_plans() {
    // The other direction of the many-to-many: the admin composed two plans into one apply.
    let documents = vec![
        (Kind::Plan, plan_bytes("p1", 'a')),
        (Kind::Plan, plan_bytes("p2", 'b')),
        (Kind::Intent, intent_bytes("a1", &["p1", "p2"])),
    ];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (2, 1, 0, 0, 0),
            vec![plan_to("p1", "a1"), plan_to("p2", "a1")],
            vec![],
        )
    );
}

#[test]
fn one_plan_named_twice_by_one_assignment_is_retained_twice() {
    // Duplicate origin occurrences are legal and are not a set: collapsing them would report a
    // mapping the admin did not make. The expected edge list carries the SAME edge twice, which
    // is the assertion that the occurrence count survived.
    let documents = vec![
        (Kind::Plan, plan_bytes("p", 'a')),
        (Kind::Intent, intent_bytes("a1", &["p", "p"])),
    ];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (1, 1, 0, 0, 0),
            vec![plan_to("p", "a1"), plan_to("p", "a1")],
            vec![],
        )
    );
}

// The next two are the pair worth reading together. They differ by ONE argument, and their
// verdicts are opposite: identical content under one identity is one document, and differing
// content under one identity is two documents and a finding. Collapsing the second the way the
// first is collapsed is the tempting bug, and it is tempting precisely because the first MUST
// collapse. Both pin the WHOLE shape, so a correlator that got the node count right and the
// finding list wrong fails here.

#[test]
fn the_same_document_read_twice_is_one_document() {
    let documents = vec![
        (Kind::Plan, plan_bytes("p", 'a')),
        (Kind::Plan, plan_bytes("p", 'a')),
    ];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected((1, 0, 0, 0, 0), vec![], vec![])
    );
}

#[test]
fn two_documents_claiming_one_identity_are_both_retained_as_a_finding() {
    let documents = vec![
        (Kind::Plan, plan_bytes("p", 'a')),
        (Kind::Plan, plan_bytes("p", 'b')),
    ];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (1, 0, 0, 1, 0),
            vec![],
            vec![GraphFinding::IdentityCollision {
                species: GraphSpecies::Plan,
                identity: identity("p"),
            }],
        )
    );
}

#[test]
fn a_second_outcome_for_one_intent_is_a_finding_naming_the_later_one() {
    // Which outcome is supernumerary is decided by identity order, not arrival order, so the
    // expectation is computed the same way rather than guessed.
    let documents = vec![
        (Kind::Intent, intent_bytes("a1", &[])),
        (
            Kind::Outcome,
            outcome_bytes("o1", "a1", RecordedTerminalState::Complete),
        ),
        (
            Kind::Outcome,
            outcome_bytes("o2", "a1", RecordedTerminalState::Unknown),
        ),
    ];
    let later = if outcome_id("o1") < outcome_id("o2") {
        "o2"
    } else {
        "o1"
    };
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (0, 1, 2, 0, 0),
            vec![intent_to("a1", "o1"), intent_to("a1", "o2")],
            vec![
                GraphFinding::OriginatingPlanUnavailable {
                    intent: intent_id("a1"),
                },
                GraphFinding::SupernumeraryOutcome {
                    intent: intent_id("a1"),
                    outcome: outcome_id(later),
                },
            ],
        )
    );
}

#[test]
fn an_intent_whose_named_plan_is_absent_carries_that_finding_and_no_other() {
    // Contrast with the collision case above: both leave an intent with no plan edge, and only
    // one of them is a disagreement about an identity. The exact finding list keeps them apart.
    let documents = vec![(Kind::Intent, intent_bytes("a1", &["p"]))];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (0, 1, 0, 0, 0),
            vec![],
            vec![GraphFinding::OriginatingPlanAbsent {
                intent: intent_id("a1"),
                plan: plan_id("p"),
            }],
        )
    );
}

#[test]
fn an_intent_naming_no_plan_at_all_is_a_different_finding() {
    // Naming a plan that is not held and naming none at all are different reports, because they
    // ask the reader for different things.
    assert_eq!(
        shape_of(&graph_of(&[(Kind::Intent, intent_bytes("a1", &[]))])),
        Shape::expected(
            (0, 1, 0, 0, 0),
            vec![],
            vec![GraphFinding::OriginatingPlanUnavailable {
                intent: intent_id("a1"),
            }],
        )
    );
}

#[test]
fn an_outcome_without_its_intent_is_first_class() {
    let documents = vec![(
        Kind::Outcome,
        outcome_bytes("o1", "a1", RecordedTerminalState::Complete),
    )];
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (0, 0, 1, 0, 0),
            vec![],
            vec![GraphFinding::OutcomeWithoutIntent {
                outcome: outcome_id("o1"),
                intent: intent_id("a1"),
            }],
        )
    );
}

#[test]
fn a_missing_outcome_says_only_that_none_was_found() {
    // The absence is reached by correlation and is never a document. It implies nothing about
    // whether the apply ran, which is the whole reason it is not an outcome with a status.
    let graph = graph_of(&[(Kind::Intent, intent_bytes("a1", &[]))]);
    let availability = graph.outcome_for(intent_id("a1"));
    assert_eq!(availability.token(), "missing");
    match availability {
        OutcomeAvailability::Missing(missing) => {
            assert_eq!(missing.intent(), intent_id("a1"));
        }
        OutcomeAvailability::Recorded(_) => panic!("no outcome was fed"),
    }

    let answered = graph_of(&[
        (Kind::Intent, intent_bytes("a1", &[])),
        (
            Kind::Outcome,
            outcome_bytes("o1", "a1", RecordedTerminalState::Complete),
        ),
    ]);
    assert_eq!(answered.outcome_for(intent_id("a1")).token(), "recorded");
}

#[test]
fn the_arrival_order_of_documents_never_changes_the_graph() {
    // There is no filename parameter anywhere in the ingest surface, so the only way order could
    // reach a verdict is through iteration. Feeding the same set backwards proves it does not,
    // and comparing WHOLE shapes rather than lengths is what makes that meaningful.
    let mut documents = vec![
        (Kind::Plan, plan_bytes("p1", 'a')),
        (Kind::Plan, plan_bytes("p2", 'b')),
        (Kind::Intent, intent_bytes("a1", &["p1"])),
        (Kind::Intent, intent_bytes("a2", &["p2", "p1"])),
        (
            Kind::Outcome,
            outcome_bytes("o1", "a1", RecordedTerminalState::Complete),
        ),
        (
            Kind::Outcome,
            outcome_bytes("o9", "missing", RecordedTerminalState::Unknown),
        ),
    ];
    let forward = shape_of(&graph_of(&documents));
    documents.reverse();
    let backward = shape_of(&graph_of(&documents));
    assert_eq!(forward, backward);
    // And the shape itself is the expected one, so the two are not equal by both being wrong.
    assert_eq!(
        forward,
        Shape::expected(
            (2, 2, 2, 0, 0),
            vec![
                plan_to("p1", "a1"),
                plan_to("p1", "a2"),
                plan_to("p2", "a2"),
                intent_to("a1", "o1"),
            ],
            vec![GraphFinding::OutcomeWithoutIntent {
                outcome: outcome_id("o9"),
                intent: intent_id("missing"),
            }],
        )
    );
}

#[test]
fn a_damaged_document_sits_beside_the_findings_and_correlates_nothing() {
    let mut damaged = plan_bytes("p", 'a');
    let keep = damaged.len().saturating_sub(20);
    damaged.truncate(keep);
    let documents = vec![
        (Kind::Plan, damaged),
        (Kind::Intent, intent_bytes("a1", &["p"])),
    ];
    // The damaged document is retained as a partial, completes nothing, and mints no edge — so
    // the intent that named it reports its plan absent, exactly as if it had never been offered.
    assert_eq!(
        shape_of(&graph_of(&documents)),
        Shape::expected(
            (0, 1, 0, 0, 1),
            vec![],
            vec![GraphFinding::OriginatingPlanAbsent {
                intent: intent_id("a1"),
                plan: plan_id("p"),
            }],
        )
    );
}

#[test]
fn a_document_signed_by_unheld_material_never_completes() {
    // The read path is what admits a document to the graph, so material the resolver does not
    // hold keeps it out entirely rather than admitting it unchecked.
    struct PolicyHoldsNothing;
    impl VerificationKeyResolver for PolicyHoldsNothing {
        fn material(&self, _id: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
            None
        }
    }
    let refusal = read_plain::<PlanReceipt>(
        plan_bytes("p", 'a'),
        &ReceiptLimits::V1,
        &PolicyHoldsNothing,
    )
    .expect_err("no material, no complete receipt");
    assert_eq!(refusal.reason(), &RefusalReason::KeyUnavailable);
}

#[test]
fn the_graph_exposes_no_route_to_world_state() {
    // A negative pin on the API rather than on a value: correlation is by typed identity only,
    // so there is deliberately nothing here that could join freshness, generation, authority, or
    // an influence account. If a later change adds one, this test is where the argument for it
    // has to be made.
    let graph = graph_of(&[(Kind::Plan, plan_bytes("p", 'a'))]);
    let node = graph.plans().values().next().unwrap();
    // What a node offers is its sealed model and the provenance of the material that checked it.
    // The model answers report scalars and further sealed values, and nothing else.
    assert_eq!(node.signer().token(), "trusted");
    assert_eq!(node.model().mode(), RecordedInvocationMode::Plan);
    assert_eq!(node.model().sources().len(), 1);
    assert_eq!(node.model().sources()[0].ordinal(), 0);
    assert_eq!(
        node.model().invocation_account(),
        RecordedInfluence::AuthoredBeforeContact
    );
}

#[test]
fn a_publication_grade_is_not_a_correlation_input() {
    // Grades describe how a sink placed bytes; they are narration and never reach the graph.
    assert_eq!(PublicationGrade::Volatile.token(), "volatile");
    assert_eq!(
        shape_of(&graph_of(&[(Kind::Plan, plan_bytes("p", 'a'))])),
        Shape::expected((1, 0, 0, 0, 0), vec![], vec![])
    );
}

/// The rooted closure follows CAUSES and stops: an outcome reaches its intent and that intent's
/// originating plans, and a plan reaches nothing later
/// (`30R:receipt-rooted-attention-and-cli`).
///
/// One world, three roots, so the direction is pinned by the CONTRAST rather than by three
/// separately-arranged fixtures agreeing with themselves. The plan case is the load-bearing one:
/// `p` is an ancestor of both later documents, so a closure that walked the connected component
/// would answer three here.
#[test]
fn a_rooted_closure_walks_to_causes_and_never_forward_to_later_attempts() {
    let graph = graph_of(&[
        (Kind::Plan, plan_bytes("p", 'a')),
        (Kind::Intent, intent_bytes("i", &["p"])),
        (
            Kind::Outcome,
            outcome_bytes("o", "i", RecordedTerminalState::Complete),
        ),
    ]);

    assert_eq!(
        graph
            .closure_from(&RecordedDocumentId::ApplyOutcome(outcome_id("o")))
            .documents(),
        [
            RecordedDocumentId::ApplyOutcome(outcome_id("o")),
            RecordedDocumentId::ApplyIntent(intent_id("i")),
            RecordedDocumentId::Plan(plan_id("p")),
        ],
        "an outcome reaches its intent and that intent's originating plans, root first"
    );
    assert_eq!(
        graph
            .closure_from(&RecordedDocumentId::ApplyIntent(intent_id("i")))
            .documents(),
        [
            RecordedDocumentId::ApplyIntent(intent_id("i")),
            RecordedDocumentId::Plan(plan_id("p")),
        ],
        "an intent reaches its origins and not the outcome that answered it"
    );
    assert_eq!(
        graph
            .closure_from(&RecordedDocumentId::Plan(plan_id("p")))
            .documents(),
        [RecordedDocumentId::Plan(plan_id("p"))],
        "and a plan pulls no later apply attempt, though both share its component"
    );
}

/// A closure never names a document the graph does not hold — the root excepted, because the root
/// is the question's own subject and may have been opened as an explicit file outside any store.
#[test]
fn a_closure_names_only_held_documents_and_always_its_own_root() {
    let graph = graph_of(&[(Kind::Intent, intent_bytes("i", &["p"]))]);
    assert_eq!(
        graph
            .closure_from(&RecordedDocumentId::ApplyIntent(intent_id("i")))
            .documents(),
        [RecordedDocumentId::ApplyIntent(intent_id("i"))],
        "an absent origin is the sibling report's to make, never a closure member"
    );
    assert_eq!(
        graph.findings(),
        vec![GraphFinding::OriginatingPlanAbsent {
            intent: intent_id("i"),
            plan: plan_id("p"),
        }],
        "and the absence is still surfaced, so nothing went quiet"
    );

    let root = RecordedDocumentId::Plan(plan_id("elsewhere"));
    let empty = ReceiptGraph::new().closure_from(&root);
    assert_eq!(empty.root(), &root);
    assert_eq!(empty.documents(), [root]);
}

#[test]
fn a_partial_receipt_carries_its_reason_and_promotes_nothing() {
    let partial = PartialReceipt::of(RefusalReason::SignatureCheck);
    assert!(partial.bounded_structure().is_none());
    assert_eq!(partial.reason(), &RefusalReason::SignatureCheck);
    let mut graph = ReceiptGraph::new();
    graph.ingest_partial(partial);
    assert_eq!(
        shape_of(&graph),
        Shape::expected((0, 0, 0, 0, 1), vec![], vec![])
    );
}
