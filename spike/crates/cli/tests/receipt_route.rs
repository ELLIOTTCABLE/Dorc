//! The receipt write routes — plan and apply — driven in process against the real seats.
//!
//! Every seat this exercises past the pipeline is the one the binary calls: the recording seat and
//! the publication seats all live in `dorc_cli::receipt_edge`, so this battery cannot green while
//! the shipped route is broken. That is the whole point of the seat living lib-side — a battery
//! that re-implemented the recording would demonstrate a capability it never observed.
//!
//! DISCLOSED SCOPE CUT, apply lane: the standup values in the PUBLICATION cases below are fixture
//! material — six distinct answers, which is what makes a transposed axis visible. The live route
//! stands up a THIN session instead, and `deterministic_apply_route` at the foot of this file is
//! where that one is driven end to end. What no test target can reach either way is the binary's
//! own argv handling around the seat, which is the e2e corpus's.
//!
//! DISCLOSED SCOPE CUT: the Spine here comes from `WhyWorld`, the sanctioned second driver, rather
//! than from the binary's own pipeline, which no test target can reach. Both are real runs over
//! the same definition table (`cli/CLAUDE.md one-definition-table-two-drivers`); what is not proven
//! here is the binary's own assembly of its world. The e2e corpus is where that lives.
//!
//! The capabilities are INJECTED, and the binary links no implementation of them — it cannot sign
//! a document at all, which is why publishing is proven here and not through the subprocess.
#![expect(
    clippy::panic,
    clippy::expect_used,
    reason = "the fixture helpers sit beside the cases, where the in-tests allowance does not reach them"
)]

mod sandbox;

use dorc_cli::receipt_edge::{
    CONTROLLER_SEMANTICS, PlacedDocument, PlacedIntent, PlacementFailure, PublicationRefusal,
    ReceiptCapabilities, ReceiptPlacement, RecordedRun, invocation_record, planning_mode,
    publish_apply_intent, publish_apply_outcome, publish_plain_apply_intent, publish_plan_receipt,
    publish_rich_plan_receipt, record_durable_arm,
};
use dorc_cli::results::{RunClock, RunSources, SiteResults, admit_fixture_records};
use dorc_core::Interner;
use dorc_plan::planning_input::{PlanningInputs, PlanningPolicy};
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::receipt::RecordedInputs;
use dorc_plan::records::{Admission, Framing, frame, header_line, sentinel_line};
use dorc_receipt::capability::{
    PublicationGrade, ReceiptSigner, ReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::dispatch::RequiredPlacementLanding;
use dorc_receipt::format::RefusalReason;
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::{ByteLimit, ReceiptLimits};
use dorc_receipt::model::PlanReceipt;
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::{read_plain, read_rich};
use dorc_receipt::report::ByteAgreement;
use dorc_receipt::tokens::RecordedInvocationMode;
use dorc_receipt_crypto::{AgeOpener, AgeSealer, Ed25519Signer, Ed25519Verifier};

/// A signing identity that exists only in this target. No age material is involved: a plain
/// document carries no sealed region, so this route needs a signer and nothing else.
const FIXTURE_SECRET: [u8; 32] = [11_u8; 32];

const BOOK: &str = "#!/bin/sh\nset -eu\napt-get update\ncp ./nginx.conf /etc/nginx/nginx.conf\n";

/// The prefix a shipped emitter line carries, spelled without a literal quote byte.
const PRINTF_HEAD: &str = "printf \u{27}";

/// The suffix a shipped emitter line carries.
const PRINTF_TAIL: &str = "\\n\u{27}";

fn authored() -> dorc_core::influence::InfluenceAccount {
    dorc_core::influence::InfluenceAccount::authored_before_contact()
}

/// A deterministic identity source. The production edge would supply one backed by the operating
/// system; nothing below the edge reaches for either.
struct CountingIds(u8);

/// A deterministic order source. The production edge reads a real clock; nothing below the edge
/// reaches for either. It TICKS, so two documents of one run take two orders exactly as they
/// would in production, and a test asserting they differ is asserting something.
struct TickingClock(u64);

impl TickingClock {
    fn fixture() -> Self {
        Self(1_700_000_000_000)
    }
}

impl dorc_receipt::order::ControllerClock for TickingClock {
    fn order_token(&mut self) -> ReceiptOrderToken {
        self.0 = self.0.saturating_add(1);
        ReceiptOrderToken::of_controller_millis(self.0)
    }
}

impl ReceiptIdSource for CountingIds {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

/// The fixture destination: documents held in memory, named by species and identity.
///
/// A TEST'S OWN VALUE, deliberately. Production has exactly one `ReceiptPlacement` — the local
/// store's — so a volatile destination cannot be handed to a production composition root.
///
/// What it reports back is a [`RequiredPlacementLanding`], which carries NO authority: a required
/// publication is minted inside `dorc-receipt`, from an accounted intent this target cannot
/// build, so a fixture landing lets a battery drive the route and never lets one manufacture a
/// permit. That is the difference from the separately-mintable proof this replaced.
#[derive(Default)]
struct MemorySink(Vec<(String, Vec<u8>)>);

impl MemorySink {
    fn keep<D: dorc_receipt::model::Species, P: dorc_receipt::model::Projection>(
        &mut self,
        prefix: &str,
        id_hex: String,
        receipt: dorc_receipt::writer::SignedReceipt<D, P>,
    ) -> (PlacedDocument, RequiredPlacementLanding) {
        let name = format!("{prefix}-{id_hex}");
        let bytes = receipt.into_bytes();
        // The SAME domain the real store takes its landing digest under, because the required
        // publication compares the two: a fixture with a private domain would refuse every
        // publication, and the battery would be exercising the mismatch arm forever.
        let digest = dorc_receipt::ids::Sha256Digest::over(
            dorc_receipt::dispatch::REQUIRED_PLACEMENT_DIGEST_DOMAIN,
            &bytes,
        );
        self.0.push((name.clone(), bytes));
        (
            PlacedDocument::of(id_hex, name, None, PublicationGrade::Volatile),
            RequiredPlacementLanding::of(digest, "fixture-volatile"),
        )
    }
}

impl ReceiptPlacement for MemorySink {
    fn place_plan(
        &mut self,
        id: dorc_receipt::ids::PlanReceiptId,
        _order: ReceiptOrderToken,
        receipt: dorc_receipt::writer::SignedReceipt<PlanReceipt, dorc_receipt::model::Rich>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Ok(self.keep("plan", id.hex(), receipt).0)
    }

    fn place_plain_plan(
        &mut self,
        id: dorc_receipt::ids::PlanReceiptId,
        _order: ReceiptOrderToken,
        receipt: dorc_receipt::writer::SignedReceipt<PlanReceipt, dorc_receipt::model::Plain>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Ok(self.keep("plan", id.hex(), receipt).0)
    }

    fn place_intent(
        &mut self,
        id: dorc_receipt::ids::ApplyIntentId,
        _order: ReceiptOrderToken,
        receipt: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyIntent,
            dorc_receipt::model::Rich,
        >,
    ) -> Result<PlacedIntent, PlacementFailure> {
        let (placed, landing) = self.keep("apply-intent", id.hex(), receipt);
        Ok(PlacedIntent { placed, landing })
    }

    fn place_plain_intent(
        &mut self,
        id: dorc_receipt::ids::ApplyIntentId,
        _order: ReceiptOrderToken,
        receipt: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyIntent,
            dorc_receipt::model::Plain,
        >,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Ok(self.keep("apply-intent", id.hex(), receipt).0)
    }

    fn place_outcome(
        &mut self,
        id: dorc_receipt::ids::ApplyOutcomeId,
        _order: ReceiptOrderToken,
        receipt: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyOutcome,
            dorc_receipt::model::Rich,
        >,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Ok(self.keep("apply-outcome", id.hex(), receipt).0)
    }

    fn place_plain_outcome(
        &mut self,
        id: dorc_receipt::ids::ApplyOutcomeId,
        _order: ReceiptOrderToken,
        receipt: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyOutcome,
            dorc_receipt::model::Plain,
        >,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Ok(self.keep("apply-outcome", id.hex(), receipt).0)
    }
}

/// A placement that places nothing, so a publication failure is a real one rather than a return
/// value standing in for one.
struct RefusingSink;

impl ReceiptPlacement for RefusingSink {
    fn place_plan(
        &mut self,
        _: dorc_receipt::ids::PlanReceiptId,
        _: ReceiptOrderToken,
        _: dorc_receipt::writer::SignedReceipt<PlanReceipt, dorc_receipt::model::Rich>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Err(PlacementFailure::Declined)
    }

    fn place_plain_plan(
        &mut self,
        _: dorc_receipt::ids::PlanReceiptId,
        _: ReceiptOrderToken,
        _: dorc_receipt::writer::SignedReceipt<PlanReceipt, dorc_receipt::model::Plain>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Err(PlacementFailure::Declined)
    }

    fn place_intent(
        &mut self,
        _: dorc_receipt::ids::ApplyIntentId,
        _: ReceiptOrderToken,
        _: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyIntent,
            dorc_receipt::model::Rich,
        >,
    ) -> Result<PlacedIntent, PlacementFailure> {
        Err(PlacementFailure::Declined)
    }

    fn place_plain_intent(
        &mut self,
        _: dorc_receipt::ids::ApplyIntentId,
        _: ReceiptOrderToken,
        _: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyIntent,
            dorc_receipt::model::Plain,
        >,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Err(PlacementFailure::Declined)
    }

    fn place_outcome(
        &mut self,
        _: dorc_receipt::ids::ApplyOutcomeId,
        _: ReceiptOrderToken,
        _: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyOutcome,
            dorc_receipt::model::Rich,
        >,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Err(PlacementFailure::Declined)
    }

    fn place_plain_outcome(
        &mut self,
        _: dorc_receipt::ids::ApplyOutcomeId,
        _: ReceiptOrderToken,
        _: dorc_receipt::writer::SignedReceipt<
            dorc_receipt::model::ApplyOutcome,
            dorc_receipt::model::Plain,
        >,
    ) -> Result<PlacedDocument, PlacementFailure> {
        Err(PlacementFailure::Declined)
    }
}

struct PolicyNames(Ed25519Verifier);

impl VerificationKeyResolver for PolicyNames {
    fn material(&self, id: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
        (ReceiptVerificationKey::signing_key_id(&self.0) == id)
            .then_some(&self.0 as &dyn ReceiptVerificationKey)
    }
}

/// The emitters produce the line a probe would SHIP; the intake reads what that line would PRINT.
/// Unwrapping here keeps the fixture tied to the production emitters rather than hand-spelling a
/// second copy of the framing.
fn unprintf(line: &str) -> String {
    line.trim_end()
        .trim_start_matches(PRINTF_HEAD)
        .trim_end_matches(PRINTF_TAIL)
        .to_owned()
}

/// A framed record stream the real intake admits, built through the production emitters.
fn wire(inners: &[&str]) -> String {
    let framing = Framing::spike(dorc_plan::invocation::book_digest(BOOK));
    let sites = inners
        .iter()
        .filter(|inner| inner.starts_with("site "))
        .count();
    let header = unprintf(&header_line(&framing, sites));
    let sentinel = unprintf(&sentinel_line(framing.nonce()));
    let body = inners
        .iter()
        .map(|inner| format!("{}\n", frame(framing.nonce(), inner)))
        .collect::<Vec<_>>()
        .concat();
    format!("{header}\n{body}{sentinel}\n")
}

/// One settled run: a real analysis, a real intake, and the durable arm recorded through the seat
/// the binary uses.
fn settled_run() -> (dorc_plan::Spine, FinalPresentation) {
    let mut interner = Interner::default();
    let sources = RunSources {
        book_name: "book.sh",
        book: BOOK,
        oracle_paths: &[],
        oracle_sources: &[],
    };
    let stream = wire(&["site 0 effect=holds rc=0"]);
    let mut clock = RunClock::Absent;
    let admitted =
        match admit_fixture_records(&sources, stream.as_bytes(), &mut clock, &mut interner) {
            Admission::Admitted(admitted) => admitted,
            other => panic!("the fixture stream must be admitted: {other:?}"),
        };

    let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
        dorc_core::loadpath::Cwd::default(),
        Vec::new(),
        Vec::new(),
        &dorc_cli::snapshot::LoadPositions::roots_only(),
        "book.sh",
        BOOK,
    );
    let world =
        dorc_cli::world::WhyWorld::analyze_measured(&snapshot, admitted.scoped.results(), false)
            .expect("the shared engine analyses this fixture book");

    let framing = Framing::spike(dorc_plan::invocation::book_digest(BOOK));
    let invocation = invocation_record(
        vec![String::from("dorc"), String::from("plan")],
        &framing,
        &snapshot,
        None,
        authored(),
    );
    // Witnessed BEFORE the world is consumed, over the surface the world settled.
    let presentation = world.final_presentation(
        PlanningInputs::of(
            CONTROLLER_SEMANTICS,
            &invocation,
            None,
            Some(&admitted.records),
            PlanningPolicy::of(planning_mode(dorc_cli::Mode::Plan), false),
        ),
        None,
    );

    let mut spine = world.into_spine();
    record_durable_arm(
        &mut spine,
        invocation,
        &presentation,
        &SiteResults::default(),
        admitted.records,
        authored(),
    );
    (spine, presentation)
}

#[test]
fn a_settled_run_publishes_a_document_that_reads_back_naming_the_surface_it_decided() {
    let (spine, presentation) = settled_run();
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();

    let placed = publish_plan_receipt(
        &RecordedRun {
            spine: &spine,
            mode: RecordedInvocationMode::Plan,
            world: authored(),
            presentation: &presentation,
            inputs: &RecordedInputs::default(),
            limits: &ReceiptLimits::V1,
        },
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
    )
    .expect("a settled run publishes");
    assert_eq!(placed.grade(), PublicationGrade::Volatile);
    assert_eq!(sink.0.len(), 1, "one run publishes one document");

    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    let material = Ed25519Verifier::of_public_material(signer.public_material())
        .expect("the fixture material loads");
    let policy = PolicyNames(material);
    let recorded = match read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &policy) {
        Ok(recorded) => recorded,
        other => panic!("a document this controller signed must read trusted: {other:?}"),
    };

    let model = recorded
        .model()
        .expect("the record stream closes over itself");
    // Non-vacuity floor: a document recording no decision would satisfy the assertion below while
    // proving the route carried nothing.
    assert!(
        model.site_count() > 0,
        "the run decided nothing, so this document proves nothing"
    );
    assert_eq!(
        model.presented_plan(),
        Some(presentation.presented_plan()),
        "the document must name the surface this run actually settled"
    );
}

#[test]
fn a_sink_that_places_nothing_publishes_nothing_and_says_so() {
    // Pinned to its exact refusal: a publication failure must stay distinguishable from a
    // projection failure, because the two are repaired by different people.
    let (spine, presentation) = settled_run();
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = RefusingSink;

    assert_eq!(
        publish_plan_receipt(
            &RecordedRun {
                spine: &spine,
                mode: RecordedInvocationMode::Plan,
                world: authored(),
                presentation: &presentation,
                inputs: &RecordedInputs::default(),
                limits: &ReceiptLimits::V1,
            },
            ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
        ),
        Err(PublicationRefusal::Placement(PlacementFailure::Declined))
    );
}

/// Fixture-only age material, generated for this battery and reused from nowhere.
///
/// Committed rather than generated at run time because age encryption is not reproducible and a
/// generated identity would make this battery depend on OS randomness. It is fixture material by
/// construction — an integration test is compiled into no library — and
/// `the_fixture_identity_is_unreachable_from_production` is what holds it there.
const FIXTURE_ONLY_AGE_IDENTITY_FOR_THIS_BATTERY: &str =
    "AGE-SECRET-KEY-1WRNNRELNXYYJLWD2WTKAYUDDQRDP76K0QU4FCLT05LRYKTJ47JSQ6J8N8H";

fn age_pair() -> (AgeSealer, AgeOpener) {
    let opener = AgeOpener::of_identity_text(FIXTURE_ONLY_AGE_IDENTITY_FOR_THIS_BATTERY)
        .expect("the fixture identity parses");
    let sealer = AgeSealer::of_recipient_text(&opener.recipient_text())
        .expect("its public half names a recipient");
    (sealer, opener)
}

fn published_rich() -> (Vec<u8>, FinalPresentation, Ed25519Signer) {
    let (spine, presentation) = settled_run();
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let (sealer, _) = age_pair();
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();
    publish_rich_plan_receipt(
        &RecordedRun {
            spine: &spine,
            mode: RecordedInvocationMode::Plan,
            world: authored(),
            presentation: &presentation,
            inputs: &RecordedInputs::default(),
            limits: &ReceiptLimits::V1,
        },
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
        &sealer,
    )
    .expect("a settled run publishes richly");
    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    (bytes, presentation, signer)
}

fn policy_for(signer: &Ed25519Signer) -> PolicyNames {
    let material = Ed25519Verifier::of_public_material(signer.public_material())
        .expect("the fixture material loads");
    PolicyNames(material)
}

#[test]
fn a_rich_document_carries_its_held_values_in_the_region_and_never_in_the_skeleton() {
    // BOTH halves of the reverse-overlay bargain, in one case. The readable side must not contain
    // the value — not the bytes, not a name pointing at them — and the authenticated side must
    // hand back exactly what the run held. Asserting only the second would pass a document that
    // leaked every value in clear beside its own ciphertext.
    let (bytes, _, signer) = published_rich();
    let (_, opener) = age_pair();

    let readable = String::from_utf8_lossy(&bytes).to_string();
    let skeleton = readable
        .split("opaque-overlay")
        .next()
        .expect("the document has a readable half")
        .to_owned();
    assert!(
        skeleton.contains("target=captured"),
        "the skeleton states the slot was captured"
    );
    assert!(
        !skeleton.contains("localhost"),
        "the skeleton must carry the STATE and never the value"
    );
    assert!(
        !skeleton.contains("book.sh"),
        "a captured source path must not appear on the readable side either"
    );

    let recorded =
        match read_rich::<PlanReceipt>(bytes, &ReceiptLimits::V1, &policy_for(&signer), &opener) {
            Ok(recorded) => recorded,
            Err(partial) => panic!("a document this controller sealed must read: {partial:?}"),
        };
    assert_eq!(
        recorded
            .recorded_detail(0, OpaqueFieldTag::TargetName)
            .map(|detail| detail.value().agrees_with(b"localhost")),
        Some(ByteAgreement::Identical),
        "the region carries exactly the value the run held"
    );
}

#[test]
fn a_region_that_cannot_be_opened_is_told_apart_from_one_whose_signature_failed() {
    // The sharp one. Both failures end in a partial receipt and look alike from outside, and they
    // are different repairs: one says the document was edited, the other says the material is
    // wrong or the region is damaged. So this corrupts the ciphertext and RE-SIGNS, which passes
    // the signature and fails the open — isolating the second cause from the first.
    let (bytes, _, signer) = published_rich();
    let text = String::from_utf8(bytes).expect("the document is text");
    let (skeleton, rest) = text
        .split_once(
            "opaque-overlay
",
        )
        .expect("a rich document has a region");
    let armor = rest
        .split_once(
            "
opaque-end",
        )
        .map(|(armor, _)| armor.to_owned())
        .expect("the region is terminated");

    // Flip one byte of the base64 payload, leaving the armor SHAPE intact so the failure lands at
    // the open rather than at the lexical locator.
    let mut lines: Vec<String> = armor.lines().map(str::to_owned).collect();
    let last = lines.len().saturating_sub(2);
    let line = lines.get_mut(last).expect("the payload has a line");
    let head: String = line.chars().take(1).collect();
    let tail: String = line.chars().skip(1).collect();
    *line = format!("{}{tail}", if head == "A" { "B" } else { "A" });
    let edited = lines.join(
        "
",
    );

    let body = dorc_receipt::format::signed_body(skeleton, Some(&edited));
    let signature = signer.sign(&dorc_receipt::ids::pae(
        "application/vnd.dorc.receipt.v1.plan.rich",
        &body,
    ));
    let resigned = dorc_receipt::format::assemble(
        skeleton,
        Some(&edited),
        &dorc_receipt::ids::to_hex(&signature),
    );

    let partial = read_rich::<PlanReceipt>(
        resigned,
        &ReceiptLimits::V1,
        &policy_for(&signer),
        &age_pair().1,
    )
    .expect_err("a corrupted region cannot be opened");
    assert_eq!(partial.reason(), &RefusalReason::RegionUnopenable);
}

#[test]
fn rich_bytes_do_not_read_as_a_plain_document() {
    // Stronger than a signature failure, and pinned to the reason it actually gives: the plain
    // grammar rejects a region STRUCTURALLY, before any material is consulted. A rich-to-plain
    // strip therefore cannot produce something a plain reader accepts even in principle.
    let (bytes, _, signer) = published_rich();
    let partial = read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &policy_for(&signer))
        .expect_err("a rich document is not a plain one");
    assert_eq!(partial.reason(), &RefusalReason::OverlayPresence);
}

#[test]
fn a_tampered_region_releases_nothing() {
    // The companion to the case above: the SAME edit, without the re-sign, must land on the
    // signature instead. Pinning both is what proves the two causes are distinguished rather than
    // collapsed into one "rejected".
    let (bytes, _, signer) = published_rich();
    let mut broken = bytes;
    let at = broken
        .windows(5)
        .position(|window| window == b"-----")
        .map(|start| start.saturating_add(40))
        .expect("the armor is present");
    let byte = broken
        .get_mut(at)
        .expect("the offset is inside the document");
    *byte = if *byte == b"A"[0] { b"B"[0] } else { b"A"[0] };
    let partial = read_rich::<PlanReceipt>(
        broken,
        &ReceiptLimits::V1,
        &policy_for(&signer),
        &age_pair().1,
    )
    .expect_err("an edited region is refused");
    assert_eq!(partial.reason(), &RefusalReason::SignatureCheck);
}

/// The book an apply is handed, distinctive enough that finding its bytes in a readable half is
/// unambiguous.
const APPLY_BYTES: &[u8] = b"#!/bin/sh\napt-get install -y nginx\n";

/// The destination this battery's standup resolves, likewise distinctive.
const APPLY_DESTINATION: &str = "web7.example.net";

fn apply_invocation() -> dorc_receipt::project::ApplyInvocation {
    dorc_receipt::project::ApplyInvocation::of(
        RecordedInvocationMode::Apply,
        None,
        dorc_receipt::project::InvocationTarget::Spelled(APPLY_DESTINATION.as_bytes().to_vec()),
        1,
        dorc_receipt::RecordedInfluence::of_token(Some("authored-before-contact")),
    )
}

fn host_influenced() -> dorc_receipt::RecordedInfluence {
    dorc_receipt::RecordedInfluence::of_token(Some("host-influenced"))
}

/// One prepared intent over fixture standup values, plus the exact image it binds.
///
/// The context's six answers are distinct, which is what would make a transposed axis visible in
/// the region rather than agreeing with itself.
fn prepared_apply_intent(
    ids: &mut CountingIds,
) -> (
    dorc_receipt::dispatch::PreparedApplyIntent,
    dorc_receipt::image::ApplyArtifactImage,
) {
    use dorc_receipt::dispatch::{
        ApplyDestination, ApplySessionReady, PendingApplyAssignment, PendingOrigins,
        ReadyApplyTarget, ReceiptPolicyWitness, ResolvedApplyContext, ResolvedAxis,
    };
    use dorc_receipt::ids::{ApplyGenerationId, ApplySessionId, ReadyApplyTargetId};

    let entered = |text: &str| ResolvedAxis::Established(text.to_owned());
    let target = ReadyApplyTargetId::mint(ids);
    let context = ResolvedApplyContext::of(
        ApplyDestination::addressed(APPLY_DESTINATION.to_owned()),
        entered("deploy"),
        entered("netns-blue"),
        entered("/srv/app"),
        entered("inherited-minus-ssh"),
        entered("agent-forwarded"),
    );
    let ready = match ApplySessionReady::of(
        ApplySessionId::mint(ids),
        ApplyGenerationId::mint(ids),
        vec![ReadyApplyTarget::of(target, context)],
    ) {
        Ok(ready) => ready,
        Err(refusal) => panic!("a well-formed standup closes: {refusal:?}"),
    };
    let image = match dorc_receipt::image::ApplyArtifactImage::of_external_stream(
        dorc_receipt::image::ApplyEntryBytes::of(APPLY_BYTES.to_vec()),
        &ReceiptLimits::V1,
    ) {
        Ok(image) => image,
        Err(refusal) => panic!("a single stream builds: {refusal:?}"),
    };
    let assignment = PendingApplyAssignment::of(
        dorc_receipt::rows::AssignmentOrdinal::of(0),
        target,
        image.clone(),
        PendingOrigins::Unavailable,
    );
    match ready.prepare_intent(vec![assignment], ReceiptPolicyWitness::required_rich()) {
        Ok(prepared) => (prepared, image),
        Err(refusal) => panic!("a well-formed assignment prepares: {refusal:?}"),
    }
}

/// The post-dispatch phase, driven through the ONE route that reaches it.
///
/// There is no shortcut into a permit any more, so a battery that only wants the phase drives
/// the real chain — project, account, publish through a modelled landing, permit, spend — rather
/// than reaching for a second arm that no longer exists.
fn spent_permit(ids: &mut CountingIds) -> dorc_receipt::dispatch::MutationDispatched {
    let (intent, _) = prepared_apply_intent(ids);
    let projected = match dorc_receipt::project::project_apply_intent(
        &intent,
        &apply_invocation(),
        dorc_receipt::RecordedInfluence::of_token(Some("authored-before-contact")),
        &ReceiptLimits::V1,
    ) {
        Ok(projected) => projected,
        Err(refusal) => panic!("a prepared intent projects: {refusal:?}"),
    };
    let Some(accounted) =
        intent.account_images(projected.details(), &|ordinal| projected.record_of(ordinal))
    else {
        panic!("the projection carries the assignment's own image")
    };
    // One digest on both sides: the required route compares them, so a fixture answering a
    // different value would drive the mismatch arm rather than the ordinary route.
    let sealed = dorc_receipt::ids::Sha256Digest::over(
        dorc_receipt::dispatch::REQUIRED_PLACEMENT_DIGEST_DOMAIN,
        b"the modelled document",
    );
    let landing = RequiredPlacementLanding::of(sealed, "fixture-volatile");
    match accounted.publish_through(dorc_receipt::ids::ApplyIntentId::mint(ids), sealed, |_| {
        Ok::<_, ()>((landing, ()))
    }) {
        Ok((published, ())) => published.permit().spend(),
        Err(through) => panic!("a modelled landing clears the route: {through:?}"),
    }
}

/// How every value answering one tag stands against `expected`.
///
/// A VERDICT per detail rather than the bytes: a reingested document hands out no plaintext, so
/// a battery proves exactness by comparing against what it expects. The vector's LENGTH is the
/// other half of the assertion — how many details answered that tag at all.
fn agreements_for<D: dorc_receipt::Species>(
    recorded: &dorc_receipt::Reingested<dorc_receipt::Receipt<D, dorc_receipt::model::Rich>>,
    tag: OpaqueFieldTag,
    expected: &[u8],
) -> Vec<ByteAgreement> {
    recorded
        .recorded_details()
        .iter()
        .filter(|detail| detail.tag() == tag)
        .map(|detail| detail.value().agrees_with(expected))
        .collect()
}

#[test]
fn a_published_intent_carries_its_exact_image_in_the_region_and_never_beside_it() {
    // BOTH halves of the reverse-overlay bargain, on the value the required-publication route
    // exists to bind. The readable side must not contain the bytes an apply will run — not the
    // bytes, not a name pointing at them — and the authenticated side must hand back the
    // assignment's own canonical image, which is what the accounting compared before sealing.
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let (sealer, opener) = age_pair();
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();
    let (intent, image) = prepared_apply_intent(&mut ids);

    let published = publish_apply_intent(
        intent,
        &apply_invocation(),
        authored(),
        &ReceiptLimits::V1,
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
        &sealer,
    )
    .expect("a prepared intent publishes richly");
    assert_eq!(published.1.grade(), PublicationGrade::Volatile);
    let announced = published.0.id();

    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    let readable = String::from_utf8_lossy(&bytes).to_string();
    let skeleton = readable
        .split("opaque-overlay")
        .next()
        .expect("the document has a readable half")
        .to_owned();
    assert!(
        skeleton.contains("image-state=captured"),
        "the skeleton states the image rode the region"
    );
    assert!(
        !skeleton.contains("apt-get"),
        "the skeleton carries the STATE and never the bytes an apply would run"
    );
    assert!(
        !skeleton.contains(APPLY_DESTINATION),
        "a captured destination must not appear on the readable side either"
    );

    let recorded = match read_rich::<dorc_receipt::model::ApplyIntent>(
        bytes,
        &ReceiptLimits::V1,
        &policy_for(&signer),
        &opener,
    ) {
        Ok(recorded) => recorded,
        Err(partial) => panic!("a document this controller sealed must read: {partial:?}"),
    };
    assert_eq!(
        recorded.receipt_id(),
        Some(announced),
        "the identity the seat handed back is the document's own"
    );
    assert_eq!(
        agreements_for(
            &recorded,
            OpaqueFieldTag::ApplyArtifactImage,
            &image.encode()
        ),
        vec![ByteAgreement::Identical],
        "exactly one record carries the image, and it is this assignment's canonical bytes"
    );
    assert_eq!(
        agreements_for(
            &recorded,
            OpaqueFieldTag::ApplyArtifactImage,
            b"some other image"
        ),
        vec![ByteAgreement::Differing],
        "and the comparison would notice a cousin"
    );
    assert_eq!(
        agreements_for(&recorded, OpaqueFieldTag::ApplyContext, b"").len(),
        1,
        "the assignment's remaining resolved axes ride their own slot"
    );

    let model = recorded
        .model()
        .expect("the record stream closes over itself");
    assert_eq!(model.assignment_count(), 1);
    assert_eq!(
        model.policy(),
        dorc_receipt::tokens::RecordedApplyPolicy::RequiredRich
    );
}

#[test]
fn a_plain_intent_withholds_the_image_it_has_no_region_to_carry() {
    // The bypass route's report document. It records the identities and the shape and says
    // withheld-plain where the bytes would be, which is why it cannot satisfy required
    // publication even when it reads back perfectly.
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();
    let (intent, _) = prepared_apply_intent(&mut ids);

    let (id, placed) = publish_plain_apply_intent(
        &intent,
        &apply_invocation(),
        authored(),
        &ReceiptLimits::V1,
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
    )
    .expect("a prepared intent publishes plainly");
    assert_eq!(placed.grade(), PublicationGrade::Volatile);

    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    let readable = String::from_utf8_lossy(&bytes).to_string();
    assert!(readable.contains("image-state=withheld-plain"));
    assert!(
        !readable.contains("opaque-overlay"),
        "a plain document has no region at all"
    );
    assert!(!readable.contains("apt-get"));

    let recorded = match read_plain::<dorc_receipt::model::ApplyIntent>(
        bytes,
        &ReceiptLimits::V1,
        &policy_for(&signer),
    ) {
        Ok(recorded) => recorded,
        other => panic!("a document this controller signed must read trusted: {other:?}"),
    };
    assert_eq!(recorded.receipt_id(), Some(id));
    let model = recorded
        .model()
        .expect("the record stream closes over itself");
    assert_eq!(model.assignment_count(), 1);
    assert_eq!(
        model.origin_state(),
        dorc_receipt::tokens::RecordedOriginState::Unavailable,
        "an apply handed bytes cannot say which plan produced them"
    );
}

#[test]
fn an_outcome_published_past_the_permit_names_the_intent_that_authorized_it() {
    // The whole required-publication chain in one case: publish the rich intent, build the gate
    // from what publication answered, spend the permit, then record what execution reached. The
    // outcome names the identity the publication minted rather than one a caller supplied.
    use dorc_receipt::project::{ApplyOutcomeReport, ApplySiteReport};

    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let (sealer, opener) = age_pair();
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();
    let (intent, _) = prepared_apply_intent(&mut ids);

    let published = publish_apply_intent(
        intent,
        &apply_invocation(),
        authored(),
        &ReceiptLimits::V1,
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
        &sealer,
    )
    .expect("a prepared intent publishes richly");
    let intent_id = published.0.id();
    let phase = published.0.permit().spend();

    let tail = b"E: Unable to locate package\n".to_vec();
    let report = ApplyOutcomeReport::of(
        intent_id,
        dorc_receipt::tokens::RecordedTerminalState::CommandFailed,
        dorc_receipt::tokens::RecordedDurableState::Published,
        vec![ApplySiteReport::of(
            dorc_receipt::rows::AssignmentOrdinal::of(0),
            dorc_receipt::rows::RecordedSite::of(dorc_receipt::rows::RecordedLeaf::of(2), None),
            dorc_receipt::tokens::RecordedSiteStatus::Ran,
            Some(1),
            Some(tail.clone()),
            None,
            host_influenced(),
        )],
        host_influenced(),
    );

    let (outcome_id, placed) = publish_apply_outcome(
        &phase,
        &report,
        &apply_invocation(),
        &ReceiptLimits::V1,
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
        &sealer,
    )
    .expect("a declared outcome publishes richly");
    assert_eq!(placed.grade(), PublicationGrade::Volatile);
    assert_ne!(
        outcome_id.hex(),
        intent_id.hex(),
        "two documents never share one identity"
    );

    let (_, bytes) = sink.0.into_iter().nth(1).expect("the sink placed two");
    let readable = String::from_utf8_lossy(&bytes).to_string();
    assert!(
        !readable.contains("Unable to locate package"),
        "admitted host output rides the region, never the readable half"
    );

    let recorded = match read_rich::<dorc_receipt::model::ApplyOutcome>(
        bytes,
        &ReceiptLimits::V1,
        &policy_for(&signer),
        &opener,
    ) {
        Ok(recorded) => recorded,
        Err(partial) => panic!("a document this controller sealed must read: {partial:?}"),
    };
    assert_eq!(
        agreements_for(&recorded, OpaqueFieldTag::Stdout, &tail),
        vec![ByteAgreement::Identical]
    );
    let model = recorded
        .model()
        .expect("the record stream closes over itself");
    assert_eq!(model.intent(), Some(intent_id));
    assert_eq!(model.site_count(), 1);
    assert_eq!(
        model.terminal(),
        dorc_receipt::tokens::RecordedTerminalState::CommandFailed
    );
}

#[test]
fn an_intent_a_sink_will_not_place_refuses_as_a_sink_failure() {
    // Pinned to its exact refusal, like the plan lane's: a publication failure and a projection
    // failure are repaired by different people, and the intent lane adds a third look-alike —
    // the image accounting — that must stay distinguishable from both.
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let (sealer, _) = age_pair();
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = RefusingSink;
    let (intent, _) = prepared_apply_intent(&mut ids);

    assert_eq!(
        publish_apply_intent(
            intent,
            &apply_invocation(),
            authored(),
            &ReceiptLimits::V1,
            ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
            &sealer,
        )
        .err(),
        Some(PublicationRefusal::Placement(PlacementFailure::Declined))
    );
}

#[test]
fn a_plain_outcome_withholds_every_byte_channel_it_has_no_region_to_carry() {
    // The degraded terminal report: sealing failed or no material was configured, and the run can
    // still say what it reached. The states must narrow — a plain document claiming `captured`
    // would promise a region it does not have, which is a document its own reader refuses.
    use dorc_receipt::project::{ApplyOutcomeReport, ApplySiteReport};

    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();
    let phase = spent_permit(&mut ids);

    let report = ApplyOutcomeReport::of(
        dorc_receipt::ids::ApplyIntentId::of_hex(&"f".repeat(64))
            .expect("the fixture identity parses"),
        dorc_receipt::tokens::RecordedTerminalState::Unknown,
        dorc_receipt::tokens::RecordedDurableState::Failed,
        vec![ApplySiteReport::of(
            dorc_receipt::rows::AssignmentOrdinal::of(0),
            dorc_receipt::rows::RecordedSite::of(dorc_receipt::rows::RecordedLeaf::of(0), None),
            dorc_receipt::tokens::RecordedSiteStatus::Unknown,
            None,
            Some(b"partial output\n".to_vec()),
            None,
            host_influenced(),
        )],
        host_influenced(),
    );

    let (id, placed) = dorc_cli::receipt_edge::publish_plain_apply_outcome(
        &phase,
        &report,
        &apply_invocation(),
        &ReceiptLimits::V1,
        ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
    )
    .expect("a declared outcome publishes plainly");
    assert_eq!(placed.grade(), PublicationGrade::Volatile);

    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    let readable = String::from_utf8_lossy(&bytes).to_string();
    assert!(readable.contains("stdout=withheld-plain"));
    assert!(
        !readable.contains("partial output"),
        "a plain document carries no host bytes anywhere"
    );

    let recorded = match read_plain::<dorc_receipt::model::ApplyOutcome>(
        bytes,
        &ReceiptLimits::V1,
        &policy_for(&signer),
    ) {
        Ok(recorded) => recorded,
        other => panic!("a document this controller signed must read trusted: {other:?}"),
    };
    assert_eq!(recorded.receipt_id(), Some(id));
    let model = recorded
        .model()
        .expect("the record stream closes over itself");
    assert_eq!(model.site_count(), 1);
    assert_eq!(
        model.terminal(),
        dorc_receipt::tokens::RecordedTerminalState::Unknown,
        "a run that produced no completion marker says unknown, never not-attempted"
    );
}

#[test]
fn an_intent_whose_region_a_reader_could_not_open_refuses_before_anything_is_placed() {
    // REFUSAL, not omission. The required arm binds exact bytes by value, so a document that
    // left some out could not fund the capability that arm exists to mint — and a document
    // larger than a reader may open is one nobody can read back at all. The narrowed bound
    // stands in for a region of real size; what is asserted is the direction and the seat.
    //
    // Pinned apart from the two failures beside it: a sink that declines, and a region that
    // does not account for its own skeleton. All three end in no document, by three repairs.
    let narrow = ReceiptLimits {
        overlay_bytes: ByteLimit::of(10),
        ..ReceiptLimits::V1
    };
    let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
    let (sealer, _) = age_pair();
    let mut ids = CountingIds(0);
    let mut clock = TickingClock::fixture();
    let mut sink = MemorySink::default();
    let (intent, _) = prepared_apply_intent(&mut ids);

    assert_eq!(
        publish_apply_intent(
            intent,
            &apply_invocation(),
            authored(),
            &narrow,
            ReceiptCapabilities::of(&mut ids, &mut clock, &signer, &mut sink),
            &sealer,
        )
        .err(),
        Some(PublicationRefusal::RegionOverBound)
    );
    assert!(
        sink.0.is_empty(),
        "a refused publication places nothing, so no dispatch can follow it"
    );
}

/// The deterministic apply route, end to end: the seat the binary calls, over a scripted host.
///
/// This is the acceptance the whole apply lane exists for. It drives `consented_apply` — the SAME
/// seat `dorc apply --host` reaches — through the REQUIRED publication arm with injected fixture
/// capabilities, which is the arm the shipped binary structurally cannot take. What it proves is
/// the ORDER: the intent is a placed document before anything is shipped, the permit is spent
/// around the one shipment, and the outcome names the intent that authorized it.
///
/// The host is `SimDriver`, so no process, socket, or clock is involved; `remaining()` and
/// `calls` are what let the negatives below assert what did NOT happen.
mod deterministic_apply_route {
    use super::{
        CountingIds, FIXTURE_SECRET, MemorySink, RefusingSink, TickingClock, age_pair, authored,
        policy_for,
    };
    use dorc_cli::apply::{
        ApplyAuthorization, ApplyPublishingCapabilities, ConsentedApplyRefusal,
        ConsentedApplyRequest, apply_invocation, consented_apply,
    };
    use dorc_cli::receipt_edge::{
        PlacedDocument, PlacedIntent, PlacementFailure, ReceiptPlacement,
    };
    use dorc_receipt::dispatch::{
        AttributionIntegrityFailure, DurableFailure, ExecutionIntegrityFailure,
        GenerationIntegrityFailure, MutationIntegrityFailure, PostDispatchFailure,
        TargetIntegrityFailure, TransportIntegrityFailure,
    };
    use dorc_receipt::graph::ReceiptGraph;
    use dorc_receipt::limits::ReceiptLimits;
    use dorc_receipt::model::PlanReceipt;
    use dorc_receipt::order::ReceiptOrderToken;
    use dorc_receipt::outcome::OutcomeAvailability;
    use dorc_receipt::reader::read_rich;
    use dorc_receipt::tokens::{ClosedToken, RecordedApplyPolicy, RecordedTerminalState};
    use dorc_receipt_crypto::Ed25519Signer;
    use dorc_transport::sim::{SimDriver, SimScript};
    use dorc_transport::{HostId, Phase};

    /// The bytes this route consents to and the host runs.
    const PLAN: &[u8] = b"#!/bin/sh\nufw allow 443/tcp\n";

    /// The destination the controller addresses.
    const DESTINATION: &str = "web9.example.net";

    fn destination() -> HostId {
        HostId::new(DESTINATION).expect("the fixture destination is an ssh destination")
    }

    /// A host that runs the artifact and exits with `status`.
    fn host_running(status: i32) -> SimDriver {
        SimDriver::new(vec![SimScript::Completes {
            stdout: Vec::new(),
            status,
        }])
    }

    #[test]
    fn the_required_arm_publishes_the_intent_before_it_ships_and_names_it_afterwards() {
        let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
        let (sealer, opener) = age_pair();
        let mut ids = CountingIds(0);
        let mut clock = TickingClock::fixture();
        let mut sink = MemorySink::default();
        let mut driver = host_running(0);
        let destination = destination();
        let invocation = apply_invocation(DESTINATION, None);

        let reached = consented_apply(
            &ConsentedApplyRequest {
                plan: PLAN,
                destination: &destination,
                nonce: "r0",
                timeout: None,
                invocation: &invocation,
                limits: &ReceiptLimits::V1,
                standup_account: authored(),
            },
            &mut ids,
            ApplyAuthorization::RequiredPublication(ApplyPublishingCapabilities::of(
                &mut clock, &signer, &mut sink, &sealer,
            )),
            &mut driver,
        )
        .expect("a placed intent authorizes one dispatch");

        assert_eq!(
            driver.calls.len(),
            1,
            "an apply ships once and never retries"
        );
        let call = driver.calls.first().expect("one call");
        assert_eq!(call.phase, Phase::Apply);
        assert_eq!(call.artifact, PLAN, "the host runs the consented bytes");
        assert_eq!(call.host, DESTINATION);

        assert_eq!(sink.0.len(), 2, "one intent and one outcome");
        let intent_id = reached
            .intent
            .expect("the required arm published an intent");
        let outcome_id = reached
            .outcome
            .expect("a shipped apply records what it reached");
        assert_eq!(reached.durable_failure, None);
        assert!(
            sink.0
                .first()
                .expect("the sink placed two")
                .0
                .starts_with("apply-intent-"),
            "the intent is the FIRST thing placed; a dispatch that shipped before it would have \
             spent authority nothing recorded"
        );

        let mut graph = ReceiptGraph::new();
        // The word the INGESTING seat supplies: this battery holds the material it signed with,
        // which is what a validated local keyset would be.
        let trust = dorc_receipt::tokens::RecordedSignerTrust::Trusted;
        for (name, bytes) in &sink.0 {
            if name.starts_with("apply-intent-") {
                match read_rich::<dorc_receipt::model::ApplyIntent>(
                    bytes.clone(),
                    &ReceiptLimits::V1,
                    &policy_for(&signer),
                    &opener,
                ) {
                    Ok(document) => graph.ingest_intent(&document, trust, bytes),
                    other => panic!("the intent must read trusted: {other:?}"),
                }
            } else {
                match read_rich::<dorc_receipt::model::ApplyOutcome>(
                    bytes.clone(),
                    &ReceiptLimits::V1,
                    &policy_for(&signer),
                    &opener,
                ) {
                    Ok(document) => graph.ingest_outcome(&document, trust, bytes),
                    other => panic!("the outcome must read trusted: {other:?}"),
                }
            }
        }
        assert_eq!(graph.intents().len(), 1);
        assert_eq!(graph.outcomes().len(), 1);
        assert!(graph.collisions().is_empty());
        match graph.outcome_for(intent_id) {
            OutcomeAvailability::Recorded(recorded) => {
                assert_eq!(
                    recorded.terminal(),
                    RecordedTerminalState::Complete,
                    "the host exited zero, so the run reached completion"
                );
            }
            OutcomeAvailability::Missing(_) => {
                panic!("this route published an outcome, so the graph must find it")
            }
        }
        assert_ne!(
            outcome_id.hex(),
            intent_id.hex(),
            "two documents never share one identity"
        );
    }

    /// THE ordering negative: a sink that will not place the intent must leave the host untouched.
    ///
    /// Pinned on the DRIVER rather than on the return value, because "it returned an error" is
    /// satisfied by a route that shipped first and failed afterwards — which is the exact failure
    /// the pre-dispatch boundary exists to prevent.
    #[test]
    fn an_unplaceable_intent_refuses_before_the_host_is_contacted_at_all() {
        let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
        let (sealer, _) = age_pair();
        let mut ids = CountingIds(0);
        let mut clock = TickingClock::fixture();
        let mut sink = RefusingSink;
        let mut driver = host_running(0);
        let destination = destination();
        let invocation = apply_invocation(DESTINATION, None);

        let refusal = consented_apply(
            &ConsentedApplyRequest {
                plan: PLAN,
                destination: &destination,
                nonce: "r0",
                timeout: None,
                invocation: &invocation,
                limits: &ReceiptLimits::V1,
                standup_account: authored(),
            },
            &mut ids,
            ApplyAuthorization::RequiredPublication(ApplyPublishingCapabilities::of(
                &mut clock, &signer, &mut sink, &sealer,
            )),
            &mut driver,
        )
        .expect_err("an intent nothing placed authorizes nothing");

        assert!(
            matches!(refusal, ConsentedApplyRefusal::Publication(_)),
            "the refusal names the publication, not the bytes: {refusal:?}"
        );
        assert!(
            driver.calls.is_empty(),
            "NOTHING was shipped — this is the whole of the pre-dispatch boundary"
        );
        assert_eq!(
            driver.remaining(),
            1,
            "the scripted session went unused, so no session was opened by another name"
        );
    }

    /// A sink that places the intent and then refuses the OUTCOME: the apply still happened.
    ///
    /// The mirror of the case above, and the pair is the point. Before the permit, a durable
    /// failure withholds the mutation; after it, the mutation has happened and a durable failure
    /// is narration — reporting it is all that is left, and stopping would restore nothing.
    #[test]
    fn a_durable_failure_past_the_permit_is_reported_and_the_apply_still_ran() {
        /// Places the intent and refuses the outcome that follows it.
        ///
        /// Only the two apply methods are reachable from this route; the other four answer the
        /// refusal rather than a plausible success, so a route that started calling one would
        /// fail here rather than pass on a stand-in.
        #[derive(Default)]
        struct PlacesTheFirstOnly(MemorySink);

        impl ReceiptPlacement for PlacesTheFirstOnly {
            fn place_plan(
                &mut self,
                _: dorc_receipt::ids::PlanReceiptId,
                _: ReceiptOrderToken,
                _: dorc_receipt::writer::SignedReceipt<PlanReceipt, dorc_receipt::model::Rich>,
            ) -> Result<PlacedDocument, PlacementFailure> {
                Err(PlacementFailure::Declined)
            }

            fn place_plain_plan(
                &mut self,
                _: dorc_receipt::ids::PlanReceiptId,
                _: ReceiptOrderToken,
                _: dorc_receipt::writer::SignedReceipt<PlanReceipt, dorc_receipt::model::Plain>,
            ) -> Result<PlacedDocument, PlacementFailure> {
                Err(PlacementFailure::Declined)
            }

            fn place_intent(
                &mut self,
                id: dorc_receipt::ids::ApplyIntentId,
                order: ReceiptOrderToken,
                receipt: dorc_receipt::writer::SignedReceipt<
                    dorc_receipt::model::ApplyIntent,
                    dorc_receipt::model::Rich,
                >,
            ) -> Result<PlacedIntent, PlacementFailure> {
                self.0.place_intent(id, order, receipt)
            }

            fn place_plain_intent(
                &mut self,
                _: dorc_receipt::ids::ApplyIntentId,
                _: ReceiptOrderToken,
                _: dorc_receipt::writer::SignedReceipt<
                    dorc_receipt::model::ApplyIntent,
                    dorc_receipt::model::Plain,
                >,
            ) -> Result<PlacedDocument, PlacementFailure> {
                Err(PlacementFailure::Declined)
            }

            fn place_outcome(
                &mut self,
                _: dorc_receipt::ids::ApplyOutcomeId,
                _: ReceiptOrderToken,
                _: dorc_receipt::writer::SignedReceipt<
                    dorc_receipt::model::ApplyOutcome,
                    dorc_receipt::model::Rich,
                >,
            ) -> Result<PlacedDocument, PlacementFailure> {
                Err(PlacementFailure::Declined)
            }

            fn place_plain_outcome(
                &mut self,
                _: dorc_receipt::ids::ApplyOutcomeId,
                _: ReceiptOrderToken,
                _: dorc_receipt::writer::SignedReceipt<
                    dorc_receipt::model::ApplyOutcome,
                    dorc_receipt::model::Plain,
                >,
            ) -> Result<PlacedDocument, PlacementFailure> {
                Err(PlacementFailure::Declined)
            }
        }

        let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
        let (sealer, _) = age_pair();
        let mut ids = CountingIds(0);
        let mut clock = TickingClock::fixture();
        let mut sink = PlacesTheFirstOnly::default();
        let mut driver = host_running(0);
        let destination = destination();
        let invocation = apply_invocation(DESTINATION, None);

        let reached = consented_apply(
            &ConsentedApplyRequest {
                plan: PLAN,
                destination: &destination,
                nonce: "r0",
                timeout: None,
                invocation: &invocation,
                limits: &ReceiptLimits::V1,
                standup_account: authored(),
            },
            &mut ids,
            ApplyAuthorization::RequiredPublication(ApplyPublishingCapabilities::of(
                &mut clock, &signer, &mut sink, &sealer,
            )),
            &mut driver,
        )
        .expect("a placed intent authorizes the dispatch whatever the outcome document does");

        assert_eq!(driver.calls.len(), 1, "the apply ran");
        assert!(reached.intent.is_some(), "its intent was placed");
        assert_eq!(
            reached.outcome, None,
            "its outcome was not, and the run says so rather than inventing an identity"
        );
        assert_eq!(
            reached.durable_failure,
            Some(DurableFailure::Sink),
            "the failure is reported as what it was — a sink, not an execution"
        );
    }

    /// The bypass word survives in the RECORDED vocabulary and authorizes nothing.
    ///
    /// A document may spell a route that dispatched with no durable behind it — that is a fact
    /// about some other run, or some other version. What this V1 surface offers is one
    /// authorization arm, so there is no capability that reaches a permit without a placement
    /// having answered, and the compile-fail pins in `receipt/src/lib.rs` are where that is
    /// asserted.
    #[test]
    fn the_bypass_route_is_a_recorded_word_and_not_an_authorization_arm() {
        assert_eq!(
            RecordedApplyPolicy::ConfiguredBypass.token(),
            "configured-bypass",
            "the word a document may record, for a route this build does not offer"
        );
    }

    /// Bytes past what one exact image may carry never reach a permit.
    ///
    /// An intent binds EXACT bytes, so bytes no image can hold are bytes nothing can bind — and
    /// the refusal lands before the session, not after the shipment.
    #[test]
    fn bytes_that_cannot_be_recorded_exactly_refuse_before_the_session_is_stood_up() {
        let narrow = ReceiptLimits {
            image_entry_bytes: dorc_receipt::limits::ByteLimit::of(4),
            ..ReceiptLimits::V1
        };
        let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
        let (sealer, _) = age_pair();
        let mut ids = CountingIds(0);
        let mut clock = TickingClock::fixture();
        let mut sink = MemorySink::default();
        let mut driver = host_running(0);
        let destination = destination();
        let invocation = apply_invocation(DESTINATION, None);

        let refusal = consented_apply(
            &ConsentedApplyRequest {
                plan: PLAN,
                destination: &destination,
                nonce: "r0",
                timeout: None,
                invocation: &invocation,
                limits: &narrow,
                standup_account: authored(),
            },
            &mut ids,
            ApplyAuthorization::RequiredPublication(ApplyPublishingCapabilities::of(
                &mut clock, &signer, &mut sink, &sealer,
            )),
            &mut driver,
        )
        .expect_err("bytes no image can hold bind nothing");

        assert!(
            matches!(refusal, ConsentedApplyRefusal::Image(_)),
            "pinned to the image, apart from the publication and preparation refusals it \
             otherwise resembles: {refusal:?}"
        );
        assert!(driver.calls.is_empty(), "and nothing was shipped");
        assert!(
            sink.0.is_empty(),
            "and the refusal landed before anything was placed"
        );
    }

    /// A lost session records UNKNOWN, and the permit is still spent.
    ///
    /// Absence of output cannot prove absence of execution, so the host's state is neither clean
    /// nor failed. The document says so in its own word rather than rounding to either.
    #[test]
    fn a_session_lost_after_sending_records_unknown_and_never_re_ships() {
        let signer = Ed25519Signer::of_secret(FIXTURE_SECRET);
        let (sealer, opener) = age_pair();
        let mut ids = CountingIds(0);
        let mut clock = TickingClock::fixture();
        let mut sink = MemorySink::default();
        let mut driver = SimDriver::new(vec![SimScript::SeveredAfter {
            stdout: b"partial".to_vec(),
        }]);
        let destination = destination();
        let invocation = apply_invocation(DESTINATION, None);

        let reached = consented_apply(
            &ConsentedApplyRequest {
                plan: PLAN,
                destination: &destination,
                nonce: "r0",
                timeout: None,
                invocation: &invocation,
                limits: &ReceiptLimits::V1,
                standup_account: authored(),
            },
            &mut ids,
            ApplyAuthorization::RequiredPublication(ApplyPublishingCapabilities::of(
                &mut clock, &signer, &mut sink, &sealer,
            )),
            &mut driver,
        )
        .expect("a lost session is an outcome, not a refusal");

        assert_eq!(
            driver.calls.len(),
            1,
            "an unknown outcome is exactly when a re-ship would double-apply"
        );
        assert!(reached.outcome.is_some());
        let (_, bytes) = sink.0.into_iter().nth(1).expect("the sink placed two");
        let recorded = match read_rich::<dorc_receipt::model::ApplyOutcome>(
            bytes,
            &ReceiptLimits::V1,
            &policy_for(&signer),
            &opener,
        ) {
            Ok(recorded) => recorded,
            other => panic!("the outcome must read trusted: {other:?}"),
        };
        assert_eq!(
            recorded
                .model()
                .expect("the record stream closes over itself")
                .terminal(),
            RecordedTerminalState::Unknown,
            "not complete, and not failed: nobody knows"
        );
    }

    /// The asymmetry that stops a lost host being handled like a logging problem.
    ///
    /// Only the durable arm narrows to the continuation. Six arms answer `None`, so a caller
    /// holding one cannot reach `continue_after` by widening a match — the narrowing is the only
    /// door and it is closed to them.
    #[test]
    fn six_of_the_seven_post_dispatch_failures_never_narrow_to_a_durable_one() {
        for failure in [
            PostDispatchFailure::TransportIntegrity(TransportIntegrityFailure),
            PostDispatchFailure::ExecutionIntegrity(ExecutionIntegrityFailure),
            PostDispatchFailure::AttributionIntegrity(AttributionIntegrityFailure),
            PostDispatchFailure::GenerationIntegrity(GenerationIntegrityFailure),
            PostDispatchFailure::TargetIntegrity(TargetIntegrityFailure),
            PostDispatchFailure::MutationIntegrity(MutationIntegrityFailure),
        ] {
            assert_eq!(
                failure.durable_only(),
                None,
                "an integrity failure is not a durable failure: {failure:?}"
            );
        }
        assert_eq!(
            PostDispatchFailure::DurableOnly(DurableFailure::Sink).durable_only(),
            Some(DurableFailure::Sink),
            "and the one that IS narrows, or the asymmetry would prove nothing"
        );
    }
}

/// Asking for a stored durable belongs to the explain surface, and the binary is what enforces it.
///
/// Driven through the real binary on purpose. The rule is argv handling, and the parser seat is
/// already pinned beside `reads_the_receipt` in `dorc_cli`'s own tests — but a guard proven at one
/// seat, in one direction, is the shape this arc keeps finding after the fact. The e2e corpus
/// cannot express this cell either: its replay blocks require rc 0 and discard stderr, and its
/// lint lane fixes both the subcommand and the book, so a refusing invocation has nowhere to sit
/// there. Hence natively, beside the routes it protects.
///
/// MEASURED, verifying this in its failing direction: with the refusal disabled `dorc plan --last
/// book.sh` still exits non-zero, on `cli-file-not-found`. So the exit status alone proves
/// NOTHING here — the slug assertion is the whole test, and simplifying it to a rc check would
/// leave a guard that passes whatever the parser does.
///
/// The `why` leg is not decoration. Without it the case would pass just as happily if every
/// invocation refused for every reason, which would prove the flag unusable rather than confined.
#[test]
fn asking_a_plan_producing_mode_for_a_stored_durable_refuses_through_the_binary() {
    const SLUG: &str = "cli-flag-requires-mode";

    // A throwaway profile: these drive the REAL binary, which writes a durable by default, and
    // an inherited environment would deposit keys and receipts in whoever ran the suite.
    let sandbox = sandbox::ProfileSandbox::new("receipt-route");
    for mode in ["plan", "apply", "probe", "round-trip", "bundle"] {
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_dorc"));
        sandbox.apply(&mut command);
        let refused = command
            .args([mode, "--receipt-last", "book.sh"])
            .output()
            .expect("the built binary runs");
        assert!(
            !refused.status.success(),
            "`dorc {mode} --receipt-last` must not proceed: a stored record stream would stand \
             where a live measurement belongs"
        );
        let stderr = String::from_utf8_lossy(&refused.stderr);
        assert!(
            stderr.contains(SLUG) && stderr.contains("--receipt-last"),
            "`dorc {mode} --receipt-last` must refuse by naming the flag and the mode it belongs \
             to, rather than by any other refusal that happens to fire first; got: {stderr}"
        );
    }

    // The control: the same flag, on the surface that owns it, is not refused for this reason.
    // Whatever else a durable-less run reports, it must not be this.
    let mut control = std::process::Command::new(env!("CARGO_BIN_EXE_dorc"));
    sandbox.apply(&mut control);
    let explained = control
        .args(["why", "--receipt-last", "--receipts=no-such-directory"])
        .output()
        .expect("the built binary runs");
    let stderr = String::from_utf8_lossy(&explained.stderr);
    assert!(
        !stderr.contains(SLUG),
        "`dorc why --receipt-last` is the one invocation the flag is for; got: {stderr}"
    );
}
