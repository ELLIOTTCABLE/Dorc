//! The plan-receipt write route, driven in process against the real pipeline.
//!
//! Every seat this exercises past the pipeline is the one the binary calls: the recording seat and
//! the publication seat both live in `dorc_cli::receipt_edge`, so this battery cannot green while
//! the shipped route is broken. That is the whole point of the seat living lib-side — a battery
//! that re-implemented the recording would demonstrate a capability it never observed.
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

use dorc_cli::receipt_edge::{
    CONTROLLER_SEMANTICS, PublicationRefusal, ReceiptCapabilities, invocation_record,
    planning_mode, publish_plan_receipt, publish_rich_plan_receipt, record_durable_arm,
};
use dorc_cli::results::{RunClock, RunSources, SiteResults, admit_fixture_records};
use dorc_core::Interner;
use dorc_plan::planning_input::{PlanningInputs, PlanningPolicy};
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::records::{Admission, Framing, frame, header_line, sentinel_line};
use dorc_receipt::capability::{
    PublicationGrade, ReceiptSigner, ReceiptSink, SelfAssertedReceiptVerificationKey,
    TrustedReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::format::RefusalReason;
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::PlanReceipt;
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::{ReadPlain, read_plain};
use dorc_receipt::reader::{ReadRich, read_rich};
use dorc_receipt::tokens::RecordedInvocationMode;
use dorc_receipt_crypto::{
    AgeOpener, AgeSealer, Ed25519Signer, Ed25519Verifier, TrustedEd25519Key,
};

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

impl ReceiptIdSource for CountingIds {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

#[derive(Default)]
struct MemorySink(Vec<(String, Vec<u8>)>);

impl ReceiptSink for MemorySink {
    fn publish(&mut self, name: &str, bytes: &[u8]) -> Option<PublicationGrade> {
        self.0.push((name.to_owned(), bytes.to_vec()));
        Some(PublicationGrade::Volatile)
    }
}

/// A sink that places nothing, so a publication failure is a real one rather than a return value
/// standing in for one.
struct RefusingSink;

impl ReceiptSink for RefusingSink {
    fn publish(&mut self, _: &str, _: &[u8]) -> Option<PublicationGrade> {
        None
    }
}

struct PolicyNames(TrustedEd25519Key);

impl VerificationKeyResolver for PolicyNames {
    fn trusted(&self, id: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey> {
        (self.0.signing_key_id() == id).then_some(&self.0 as &dyn TrustedReceiptVerificationKey)
    }
    fn self_asserted(&self, _: SigningKeyId) -> Option<&dyn SelfAssertedReceiptVerificationKey> {
        None
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
        dorc_cli::world::WhyWorld::analyze_measured(&snapshot, admitted.scoped.results(), false);

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
    let mut sink = MemorySink::default();

    let grade = publish_plan_receipt(
        &spine,
        RecordedInvocationMode::Plan,
        authored(),
        &presentation,
        ReceiptCapabilities::of(&mut ids, &signer, &mut sink),
    )
    .expect("a settled run publishes");
    assert_eq!(grade, PublicationGrade::Volatile);
    assert_eq!(sink.0.len(), 1, "one run publishes one document");

    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    let material = Ed25519Verifier::of_public_material(signer.public_material())
        .expect("the fixture material loads");
    let policy = PolicyNames(TrustedEd25519Key::of(material));
    let recorded = match read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &policy) {
        Ok(ReadPlain::Trusted(recorded)) => recorded,
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
    let mut sink = RefusingSink;

    assert_eq!(
        publish_plan_receipt(
            &spine,
            RecordedInvocationMode::Plan,
            authored(),
            &presentation,
            ReceiptCapabilities::of(&mut ids, &signer, &mut sink),
        ),
        Err(PublicationRefusal::Sink)
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
    let mut sink = MemorySink::default();
    publish_rich_plan_receipt(
        &spine,
        RecordedInvocationMode::Plan,
        authored(),
        &presentation,
        ReceiptCapabilities::of(&mut ids, &signer, &mut sink),
        &sealer,
    )
    .expect("a settled run publishes richly");
    let (_, bytes) = sink.0.into_iter().next().expect("the sink placed one");
    (bytes, presentation, signer)
}

fn policy_for(signer: &Ed25519Signer) -> PolicyNames {
    let material = Ed25519Verifier::of_public_material(signer.public_material())
        .expect("the fixture material loads");
    PolicyNames(TrustedEd25519Key::of(material))
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
            Ok(ReadRich::Trusted(recorded)) => recorded,
            Err(partial) => panic!("a document this controller sealed must read: {partial:?}"),
            Ok(other) => panic!("expected a trusted read: {other:?}"),
        };
    assert_eq!(
        recorded.detail(0, OpaqueFieldTag::TargetName),
        Some(b"localhost".as_slice()),
        "the region must hand back exactly the value the run held"
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
