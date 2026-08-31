//! Real published plan documents, projected, sealed, signed and read back.
//!
//! Built through the write and read paths rather than assembled as a model, for the reason
//! `receipt/tests/support` gives for its own twin: what is worth pinning here is that record
//! ordinals, detail keys and locator payloads still line up AFTER a round trip, and a hand-built
//! model agrees with itself. This is a deliberate re-spelling rather than a shared module — cargo
//! does not share test support across crates, and tests are not DRY by house rule.
//!
//! Every injected capability is INERT: no cryptography, no randomness, no clock. They satisfy the
//! real typestates, and none can promote its own output — a verifier answers a bare boolean and
//! every state it feeds is minted inside `dorc-receipt`. Fixture material by construction, and an
//! integration test compiles into no library (`rul-fixture-identity-never-production`).

#![allow(
    dead_code,
    reason = "one shared fixture module, two test binaries: each uses the accessors it needs"
)]
#![expect(
    clippy::expect_used,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "a fixture module is an ordinary module to clippy, so the central allow-in-tests keys do not reach it"
)]

use dorc_receipt::capability::{
    OverlayOpener, OverlaySealer, ReceiptSigner, ReceiptVerificationKey, ReceiptVerifier,
    VerificationKeyResolver,
};
use dorc_receipt::durable_locator::{DurableLocator, DurableStage, RecordedStageKind};
use dorc_receipt::format::Skeleton;
use dorc_receipt::ids::{EncryptionKeyId, PlanReceiptId, ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{PlanReceipt, Rich};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::overlay::DocumentRows;
use dorc_receipt::plan::{RecordedPlanReceipt, RecordedSiteDecision, RecordedSource, SourceSlots};
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::{Receipt, read_rich};
use dorc_receipt::reingested::{RecordedInfluence, Reingested};
use dorc_receipt::report::{
    AuthenticationState, DetailState, RequestedAddress, SiblingState, SourceObservation,
    WhyFactsInput, derive,
};
use dorc_receipt::rows::{
    RecordedAst, RecordedInvocation, RecordedLeaf, RecordedSite, SourceOrdinal,
};
use dorc_receipt::tokens::{
    OpaqueState, RecordedDisposition, RecordedInvocationMode, RecordedSourceClass,
    RecordedSourceRole,
};
use dorc_receipt::writer::{DraftReceipt, OverlayPlaintext};

/// The book every fixture document records. Line 2 carries the site.
pub(crate) const BOOK: &str = "#!/bin/sh\nhork tune --profile web\nufw allow 443/tcp\n";

/// The byte span of line 2 — what the site's authored locator names.
fn addressed_span() -> (u64, u64) {
    let start = BOOK
        .find("hork")
        .expect("the fixture book carries the site");
    let end = start + "hork tune --profile web\n".len();
    (start as u64, end as u64)
}

const FIXTURE_SIGNATURE: [u8; 64] = [7_u8; 64];

struct InertSigner;

impl ReceiptSigner for InertSigner {
    fn signing_key_id(&self) -> SigningKeyId {
        SigningKeyId::of_public_material(&[1_u8; 32])
    }
    fn sign(&self, _body: &[u8]) -> [u8; 64] {
        FIXTURE_SIGNATURE
    }
}

impl ReceiptVerifier for InertSigner {
    fn verify(&self, _body: &[u8], signature: &[u8; 64]) -> bool {
        *signature == FIXTURE_SIGNATURE
    }
}

impl ReceiptVerificationKey for InertSigner {
    fn signing_key_id(&self) -> SigningKeyId {
        SigningKeyId::of_public_material(&[1_u8; 32])
    }
}

struct TrustingResolver(InertSigner);

impl VerificationKeyResolver for TrustingResolver {
    fn material(&self, _id: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
        Some(&self.0)
    }
}

/// A reversible hex transcription inside the format's own armor shape. Not encryption, and not
/// pretending to be: what these cases exercise is VALIDATION of a region against the skeleton that
/// was signed, which is the crate's own act whatever produced the bytes.
struct HexArmor;

const ARMOR_WIDTH: usize = 64;

impl OverlaySealer for HexArmor {
    fn encryption_key_id(&self) -> EncryptionKeyId {
        EncryptionKeyId::of_recipient_material(&[2_u8; 32])
    }
    fn seal(&self, plaintext: &[u8]) -> Option<String> {
        let hex: String = plaintext.iter().fold(String::new(), |mut out, byte| {
            use core::fmt::Write as _;
            let _ = write!(out, "{byte:02x}");
            out
        });
        let mut out = String::from(dorc_receipt::format::ARMOR_BEGIN);
        for line in hex.as_bytes().chunks(ARMOR_WIDTH) {
            out.push('\n');
            out.push_str(core::str::from_utf8(line).ok()?);
        }
        out.push('\n');
        out.push_str(dorc_receipt::format::ARMOR_END);
        Some(out)
    }
}

impl OverlayOpener for HexArmor {
    fn open(&self, armor: &str, max_bytes: u64) -> Option<Vec<u8>> {
        let body = armor
            .strip_prefix(dorc_receipt::format::ARMOR_BEGIN)?
            .strip_suffix(dorc_receipt::format::ARMOR_END)?;
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if !hex.len().is_multiple_of(2) {
            return None;
        }
        let bytes: Option<Vec<u8>> = hex
            .as_bytes()
            .chunks(2)
            .map(|pair| u8::from_str_radix(core::str::from_utf8(pair).ok()?, 16).ok())
            .collect();
        bytes.filter(|opened| u64::try_from(opened.len()).is_ok_and(|len| len <= max_bytes))
    }
}

struct Counting(u8);

impl ReceiptIdSource for Counting {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

/// One published document, read back, plus the identities it was filed under.
pub(crate) struct DocumentUnderTest {
    pub(crate) receipt: Reingested<Receipt<PlanReceipt, Rich>>,
    pub(crate) model: Reingested<RecordedPlanReceipt>,
    pub(crate) id: PlanReceiptId,
    pub(crate) order: ReceiptOrderToken,
}

/// How one fixture document should differ from the ordinary one — the DST's own axis set.
///
/// Each knob names a real degraded shape a store can hand back, and they compose: what the
/// adversarial battery varies is the WORLD, never the assertions.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Shape {
    /// Plant a locator payload that will not parse.
    pub(crate) damaged_locator: bool,
    /// Record the site's shell text as withheld by a plain projection rather than captured.
    pub(crate) shell_withheld: bool,
    /// Record the source's own bytes as refused by a bound.
    pub(crate) content_over_bound: bool,
    /// Record the site as replaced rather than run.
    pub(crate) replaced: bool,
    /// Record the decision as taken before host contact.
    pub(crate) authored_before_contact: bool,
    /// File the document under the undated order token.
    pub(crate) undated: bool,
}

/// The ordinary document: one general-sh source carrying its exact bytes, one site carrying its
/// shell text and a one-stage authored locator naming line 2.
pub(crate) fn published() -> DocumentUnderTest {
    shaped(Shape::default(), 5)
}

/// One document built to `shape`, minted under `seed`.
pub(crate) fn shaped(shape: Shape, seed: u8) -> DocumentUnderTest {
    let locator = DurableLocator::of(
        vec![
            DurableStage::in_source(
                RecordedStageKind::Authored,
                SourceOrdinal::of(0),
                addressed_span(),
                Vec::new(),
            )
            .expect("a forward span in a source"),
        ],
        0,
        &ReceiptLimits::V1,
    )
    .expect("a valid one-stage graph");
    let payload = if shape.damaged_locator {
        b"not a locator".to_vec()
    } else {
        locator.encode()
    };

    let mut rows = DocumentRows::default();
    let invocation = RecordedInvocation::of(
        RecordedInvocationMode::Plan,
        None,
        OpaqueState::Uncollected,
        OpaqueState::Uncollected,
        1,
        RecordedInfluence::AuthoredBeforeContact,
    );
    rows.push(&invocation, &[])
        .expect("the invocation row is well formed");

    let content = if shape.content_over_bound {
        OpaqueState::OmittedLimit
    } else {
        OpaqueState::Captured
    };
    let source = RecordedSource::of(
        SourceOrdinal::of(0),
        RecordedSourceRole::Book,
        "a".repeat(64),
        BOOK.len() as u64,
        SourceSlots {
            path: OpaqueState::Uncollected,
            excerpt: OpaqueState::Uncollected,
            content,
        },
        RecordedSourceClass::GeneralSh,
        RecordedInfluence::AuthoredBeforeContact,
    );
    let source_details: Vec<(OpaqueFieldTag, Option<Vec<u8>>)> = if shape.content_over_bound {
        Vec::new()
    } else {
        vec![(
            OpaqueFieldTag::SourceContent,
            Some(BOOK.as_bytes().to_vec()),
        )]
    };
    rows.push(&source, &source_details)
        .expect("the source row is well formed");

    let shell = if shape.shell_withheld {
        OpaqueState::WithheldPlain
    } else {
        OpaqueState::Captured
    };
    let site = RecordedSiteDecision::of(
        RecordedSite::of(RecordedLeaf::of(0), None),
        RecordedAst::of(3),
        if shape.replaced {
            RecordedDisposition::Replace
        } else {
            RecordedDisposition::Run
        },
        shell,
        OpaqueState::Captured,
        if shape.authored_before_contact {
            RecordedInfluence::AuthoredBeforeContact
        } else {
            RecordedInfluence::HostInfluenced
        },
    );
    let mut site_details: Vec<(OpaqueFieldTag, Option<Vec<u8>>)> = Vec::new();
    if !shape.shell_withheld {
        site_details.push((
            OpaqueFieldTag::Shell,
            Some(b"hork tune --profile web".to_vec()),
        ));
    }
    site_details.push((OpaqueFieldTag::SiteLocator, Some(payload)));
    rows.push(&site, &site_details)
        .expect("the site row is well formed");

    let id = PlanReceiptId::mint(&mut Counting(seed));
    let order = if shape.undated {
        ReceiptOrderToken::UNDATED
    } else {
        ReceiptOrderToken::of_controller_millis(1_700_000_000_000)
    };
    let (records, details) = rows.into_parts();
    let skeleton = Skeleton {
        receipt_id: id.hex(),
        order,
        signing_key_id: ReceiptSigner::signing_key_id(&InertSigner).hex(),
        encryption_key_id: Some(HexArmor.encryption_key_id().hex()),
        records,
    };
    // The production seat's own sequence: serialize the readable skeleton, build the canonical
    // region over THAT exact span, seal, sign. Any other order binds the region to bytes the
    // signature does not cover, and this document's own reader would refuse it.
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&skeleton)
        .expect("the fixture skeleton serializes");
    let plaintext = OverlayPlaintext::canonical(
        &skeleton.receipt_id,
        <PlanReceipt as dorc_receipt::model::Species>::TOKEN,
        span.as_bytes(),
        &details,
    );
    let bytes = DraftReceipt::<PlanReceipt, Rich>::of(skeleton)
        .serialize(plaintext, &HexArmor)
        .expect("a rich draft seals")
        .sign(&InertSigner)
        .bytes()
        .to_vec();

    let receipt = read_rich::<PlanReceipt>(
        bytes,
        &ReceiptLimits::V1,
        &TrustingResolver(InertSigner),
        &HexArmor,
    )
    .expect("the fixture document reads back");
    let model = receipt.model().expect("its records close over themselves");
    DocumentUnderTest {
        receipt,
        model,
        id,
        order,
    }
}

/// The sealed report model for one document, with whatever the edge is pretending it saw.
pub(crate) fn facts(
    document: &DocumentUnderTest,
    siblings: Vec<SiblingState>,
    observations: Vec<SourceObservation>,
    address: Option<RequestedAddress>,
) -> dorc_receipt::report::RecordedWhyFacts {
    use dorc_receipt::graph::ReceiptGraph;
    use dorc_receipt::report::RecordedDocumentId;
    derive(&WhyFactsInput {
        root: &document.receipt,
        model: &document.model,
        order: document.order,
        authentication: AuthenticationState::Trusted,
        detail: DetailState::Available,
        // A plan root reaches nothing further whatever the store holds; the walk itself is pinned
        // in `dorc-receipt`, so building membership by hand here would exercise a route the API
        // does not have.
        reached: ReceiptGraph::new().closure_from(&RecordedDocumentId::Plan(document.id)),
        siblings,
        observations,
        address,
    })
}
