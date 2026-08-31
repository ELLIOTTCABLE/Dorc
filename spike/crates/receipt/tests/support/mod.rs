//! One real rich plan document: projected, sealed, signed, and read back.
//!
//! Built through the write and read paths rather than assembled as a model, because the properties
//! worth pinning are the ones a round trip can break — record ordinals lining up with detail keys,
//! a locator payload surviving the region, a state word meaning the same thing on both sides.
//!
//! The injected capabilities are INERT and deliberately so: `capability.rs` exists precisely so a
//! deterministic test can drive the real states without reaching for cryptography, and none of
//! these implementations can promote its own output — a verifier answers a bare boolean and the
//! states it feeds are minted inside the crate.

#![allow(
    dead_code,
    reason = "one shared fixture module; each test binary uses the accessors it needs"
)]
#![expect(
    clippy::expect_used,
    clippy::arithmetic_side_effects,
    reason = "a fixture module is an ordinary module to clippy, so the central allow-in-tests keys \
              do not reach it; see spike/clippy.toml"
)]

use dorc_receipt::capability::{
    OverlayOpener, OverlaySealer, ReceiptSigner, ReceiptVerificationKey, ReceiptVerifier,
    VerificationKeyResolver,
};
use dorc_receipt::durable_locator::{DurableLocator, DurableStage, RecordedStageKind};
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::ids::{EncryptionKeyId, PlanReceiptId, ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{PlanReceipt, Rich};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::overlay::DocumentRows;
use dorc_receipt::plan::{
    RecordedAdmission, RecordedLicensor, RecordedLoadDecision, RecordedNarrative,
    RecordedPlanReceipt, RecordedPresentedPlan, RecordedProbeShip, RecordedRegionDecision,
    RecordedRenderDecision, RecordedSiteClassification, RecordedSiteDecision,
    RecordedSolveCertification, RecordedSource, RecordedSurvival, RenderSubject, SourceSlots,
};
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::{Receipt, read_rich};
use dorc_receipt::reingested::{RecordedInfluence, Reingested};
use dorc_receipt::rows::{
    LoadOrdinal, NarrativeOrdinal, RecordedAst, RecordedInvocation, RecordedLeaf, RecordedOperands,
    RecordedSite, RegionOrdinal, SourceOrdinal,
};
use dorc_receipt::tokens::{
    OpaqueState, RecordedAdmissionOutcome, RecordedDisposition, RecordedInvocationMode,
    RecordedLicenseCustody, RecordedLicenseVerb, RecordedLoadOutcome, RecordedNarrativeKind,
    RecordedRenderKind, RecordedShipLane, RecordedSiteClass, RecordedSolvePass,
    RecordedSourceClass, RecordedSourceRole, RecordedSpeechAct, RecordedSurvivalOutcome,
};
use dorc_receipt::writer::DraftReceipt;

/// The book every case addresses. Line 2 is the site the document records.
const BOOK: &str = "#!/bin/sh\nhork tune --profile web\nufw allow 443/tcp\n";

/// The acquired book, exactly as the run held it.
pub(crate) fn book() -> &'static str {
    BOOK
}

/// The byte span of line 2 — what the site's authored locator names.
fn addressed_span() -> (u64, u64) {
    let start = BOOK
        .find("hork")
        .expect("the fixture book carries the site");
    let end = start + "hork tune --profile web\n".len();
    (start as u64, end as u64)
}

/// A signature that is a constant. The states a document reaches are minted from a boolean the
/// verifier answers, so an inert one reaches exactly the states a real one would.
struct InertSigner;

const FIXTURE_SIGNATURE: [u8; 64] = [7_u8; 64];

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

/// A resolver holding the fixture provider material.
struct TrustingResolver(InertSigner);

impl VerificationKeyResolver for TrustingResolver {
    fn material(&self, _id: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
        Some(&self.0)
    }
}

/// An "armor" that is a reversible hex transcription inside the format's own marker shape.
///
/// Not encryption and not pretending to be: what these cases exercise is the VALIDATION of a
/// region against the skeleton that was signed, which is the crate's own act whatever produced the
/// bytes. It still has to satisfy `format::check_armor_shape` — the markers, the base64 alphabet
/// (hex is a subset), and uniform line widths — because a fixture that dodged that check would be
/// exercising a document this reader would never accept.
struct HexArmor;

/// The width every data line but the last takes.
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
        let hex: String = body
            .chars()
            .filter(|glyph| !glyph.is_whitespace())
            .collect();
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

/// One published document, read back and sealed, plus the identities it was filed under.
pub(crate) struct DocumentUnderTest {
    pub(crate) receipt: Reingested<Receipt<PlanReceipt, Rich>>,
    pub(crate) model: Reingested<RecordedPlanReceipt>,
    pub(crate) id: PlanReceiptId,
    pub(crate) order: ReceiptOrderToken,
}

/// The fixture document: one general-sh source carrying its exact bytes, one site carrying its
/// shell text and a one-stage authored locator naming line 2.
pub(crate) fn published() -> DocumentUnderTest {
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
    publish_with_locator(
        PlanReceiptId::mint(&mut Counting(5)),
        ReceiptOrderToken::of_controller_millis(1_700_000_000_000),
        locator.encode(),
    )
}

/// As [`published`], with whatever bytes the caller wants in the locator slot.
///
/// The payload is a parameter so a case can plant one that does not parse — which is a different
/// state from a site that recorded no provenance, and the model has to tell them apart.
pub(crate) fn publish_with_locator(
    id: PlanReceiptId,
    order: ReceiptOrderToken,
    locator_payload: Vec<u8>,
) -> DocumentUnderTest {
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

    let source = RecordedSource::of(
        SourceOrdinal::of(0),
        RecordedSourceRole::Book,
        "a".repeat(64),
        BOOK.len() as u64,
        SourceSlots {
            path: OpaqueState::Uncollected,
            excerpt: OpaqueState::Uncollected,
            content: OpaqueState::Captured,
        },
        RecordedSourceClass::GeneralSh,
        RecordedInfluence::AuthoredBeforeContact,
    );
    rows.push(
        &source,
        &[(
            OpaqueFieldTag::SourceContent,
            Some(BOOK.as_bytes().to_vec()),
        )],
    )
    .expect("the source row is well formed");

    push_singletons(&mut rows);

    let site = RecordedSiteDecision::of(
        RecordedSite::of(RecordedLeaf::of(0), None),
        RecordedAst::of(3),
        RecordedDisposition::Run,
        OpaqueState::Captured,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    );
    rows.push(
        &site,
        &[
            (
                OpaqueFieldTag::Shell,
                Some(b"hork tune --profile web".to_vec()),
            ),
            (OpaqueFieldTag::SiteLocator, Some(locator_payload)),
        ],
    )
    .expect("the site row is well formed");

    push_analysis_families(&mut rows);

    let (records, details) = rows.into_parts();
    let skeleton = Skeleton {
        receipt_id: id.hex(),
        order,
        signing_key_id: ReceiptSigner::signing_key_id(&InertSigner).hex(),
        encryption_key_id: Some(HexArmor.encryption_key_id().hex()),
        records,
    };
    // The production seat's own sequence (`receipt_edge::seal_and_sign`): serialize the readable
    // skeleton, build the canonical region over THAT exact span, seal, sign. Doing it in any other
    // order would produce a region bound to bytes the signature does not cover, which this
    // document's own reader would then refuse.
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&skeleton)
        .expect("the fixture skeleton serializes");
    let plaintext = dorc_receipt::writer::OverlayPlaintext::canonical(
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

    let read = read_rich::<PlanReceipt>(
        bytes,
        &ReceiptLimits::V1,
        &TrustingResolver(InertSigner),
        &HexArmor,
    )
    .expect("the fixture document reads back");
    let receipt = read;
    let model = receipt.model().expect("its records close over themselves");
    DocumentUnderTest {
        receipt,
        model,
        id,
        order,
    }
}

/// The two OPTIONAL singletons, pushed in the species' own kind order (before the site rows).
///
/// The fixture carries one of each, because a projection nothing exercises is a projection nobody
/// has checked: the coverage answer would say `Projected(1)` whether or not the decomposition
/// worked.
fn push_singletons(rows: &mut DocumentRows) {
    let admission = RecordedAdmission::of(
        RecordedAdmissionOutcome::Admitted,
        2,
        96,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    );
    rows.push(
        &admission,
        &[(
            OpaqueFieldTag::RecordStream,
            Some(b"fact hork 0\nfact ufw 1\n".to_vec()),
        )],
    )
    .expect("the admission row is well formed");

    let presented = RecordedPresentedPlan::of(
        "b".repeat(64),
        "c".repeat(64),
        Some("d".repeat(64)),
        RecordedInfluence::HostInfluenced,
    );
    rows.push(&presented, &[])
        .expect("the presented-plan row is well formed");
}

/// One row of every remaining analysis family, so each projection is exercised over real material.
#[expect(
    clippy::too_many_lines,
    reason = "one row per persisted family, in the species' own kind order: splitting it hides which families the fixture does and does not cover"
)]
fn push_analysis_families(rows: &mut DocumentRows) {
    let site = RecordedSite::of(RecordedLeaf::of(0), None);

    let region = RecordedRegionDecision::of(
        RegionOrdinal::of(0),
        RecordedAst::of(4),
        RecordedDisposition::Guard,
        3,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    );
    rows.push(
        &region,
        &[(OpaqueFieldTag::Shell, Some(b"ufw allow 443/tcp".to_vec()))],
    )
    .expect("the region row is well formed");

    let load = RecordedLoadDecision::of(
        LoadOrdinal::of(0),
        RecordedLoadOutcome::Bound,
        OpaqueState::Captured,
        OpaqueState::Captured,
        RecordedInfluence::AuthoredBeforeContact,
    );
    rows.push(
        &load,
        &[
            (
                OpaqueFieldTag::ImportPath,
                Some(b"./ufw.oracle.sh".to_vec()),
            ),
            (OpaqueFieldTag::Custody, Some(b"sourced-exact".to_vec())),
        ],
    )
    .expect("the load row is well formed");

    let classification = RecordedSiteClassification::of(
        site,
        RecordedAst::of(3),
        RecordedSiteClass::EstablishProbeWritten,
        true,
        false,
        RecordedOperands::of(2, 1),
        RecordedInfluence::HostInfluenced,
    );
    rows.push(&classification, &[])
        .expect("the classification row is well formed");

    let certification = RecordedSolveCertification::of(
        RecordedSolvePass::EffectiveReach,
        true,
        false,
        RecordedInfluence::HostInfluenced,
    );
    rows.push(&certification, &[])
        .expect("the certification row is well formed");

    let ship = RecordedProbeShip::of(
        site,
        RecordedShipLane::Predict,
        OpaqueState::Captured,
        RecordedInfluence::AuthoredBeforeContact,
    );
    rows.push(
        &ship,
        &[(
            OpaqueFieldTag::Shell,
            Some(b"ufw__predict allow 443/tcp".to_vec()),
        )],
    )
    .expect("the ship row is well formed");

    let survival = RecordedSurvival::of(
        site,
        RecordedSurvivalOutcome::DemotedPoisoned,
        Some(RecordedLeaf::of(0)),
        Some(2),
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    );
    rows.push(
        &survival,
        &[(OpaqueFieldTag::Locator, Some(b"sm.dorc.Service".to_vec()))],
    )
    .expect("the survival row is well formed");

    let render = RecordedRenderDecision::of(
        RenderSubject::Leaf(site),
        RecordedRenderKind::PinnedBinding,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    )
    .expect("a leaf-keyed kind takes a leaf subject");
    rows.push(
        &render,
        &[(
            OpaqueFieldTag::DiagnosticOperand,
            Some(b"ufw__is_converged".to_vec()),
        )],
    )
    .expect("the render row is well formed");

    let narrative = RecordedNarrative::of(
        NarrativeOrdinal::of(0),
        RecordedSpeechAct::Declined,
        RecordedNarrativeKind::VerdictDecline,
        RecordedOperands::of(1, 0),
        RecordedInfluence::HostInfluenced,
    );
    rows.push(&narrative, &[])
        .expect("the narrative row is well formed");

    let licensor = RecordedLicensor::of(
        site,
        RecordedLicenseVerb::Guard,
        RecordedLicenseCustody::Vouched,
        OpaqueState::Captured,
        RecordedInfluence::HostInfluenced,
    );
    rows.push(
        &licensor,
        &[(OpaqueFieldTag::Locator, Some(b"ufw.oracle.sh:12".to_vec()))],
    )
    .expect("the licensor row is well formed");
}

/// The one row helper the fixture needs that `DocumentRows` does not expose by name.
fn _row_shape_is_checked(record: &SkeletonRecord) -> bool {
    record.kind() == dorc_receipt::grammar::RecordKind::Source
}
