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
    reason = "the settled-run helper sits beside the cases, where the in-tests allowance does not reach it"
)]

use dorc_cli::receipt_edge::{
    CONTROLLER_SEMANTICS, PublicationRefusal, invocation_record, planning_mode,
    publish_plan_receipt, record_durable_arm,
};
use dorc_cli::results::{RunClock, RunSources, SiteResults, admit_fixture_records};
use dorc_core::Interner;
use dorc_plan::planning_input::{PlanningInputs, PlanningPolicy};
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::records::{Admission, Framing, frame, header_line, sentinel_line};
use dorc_receipt::capability::{
    PublicationGrade, ReceiptSink, SelfAssertedReceiptVerificationKey,
    TrustedReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::PlanReceipt;
use dorc_receipt::reader::{ReadPlain, read_plain};
use dorc_receipt::tokens::RecordedInvocationMode;
use dorc_receipt_crypto::{Ed25519Signer, Ed25519Verifier, TrustedEd25519Key};

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
        &mut ids,
        &signer,
        &mut sink,
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
            &mut ids,
            &signer,
            &mut sink,
        ),
        Err(PublicationRefusal::Sink)
    );
}
