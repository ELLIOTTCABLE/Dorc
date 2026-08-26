//! The receipt edge: what a settled run RECORDS, and how that record is published.
//!
//! This seat lives lib-side so the binary and the in-process battery drive ONE of it. A test that
//! re-implemented the recording would demonstrate a capability it never observed, which is the
//! defect `one-definition-table-two-drivers` exists to refuse.
//!
//! Nothing here opens a file or reads the environment. Every such answer arrives as a VALUE —
//! argv, the run instant, the signer, the sink — so the seam this module sits on is the one
//! `lib-target-is-a-loom-seam` draws.
//!
//! TWO seats hold an edge rather than a value, and both are injected in: [`OsEntropy`], which
//! reads the operating system's randomness, and [`RunClockOrder`], which spends a reading of a
//! run's own clock. They are what a test REPLACES; every other seat here stays a function of
//! what it was handed.

use std::collections::BTreeMap;

use dorc_plan::planning_input::PlanningMode;
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::records::{AdmittedUnscopedHostRecords, Framing};
use dorc_receipt::capability::{OverlaySealer, PublicationGrade, ReceiptSigner, ReceiptSink};
use dorc_receipt::dispatch::{ExactApplyImagesPresent, MutationDispatched, PreparedApplyIntent};
use dorc_receipt::format::{Skeleton, serialize_skeleton};
use dorc_receipt::ids::{
    ApplyIntentId, ApplyOutcomeId, EntropyReceiptIds, ReceiptIdEntropy, ReceiptIdSource,
};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Rich, Species};
use dorc_receipt::order::{ControllerClock, ReceiptOrderToken};
use dorc_receipt::overlay::{OverlayEntry, captured_slots};
use dorc_receipt::project::{
    ApplyInvocation, ApplyOutcomeReport, ApplyProjectionRefusal, project_apply_intent,
    project_apply_outcome,
};
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::projection::narrow_to_plain;
use dorc_receipt::tokens::RecordedInvocationMode;
use dorc_receipt::writer::{DraftReceipt, OverlayPlaintext, PublishedReceipt};
use dorc_receipt::{RecordedInfluence, SkeletonRecord};

use crate::results::SiteResults;
use crate::snapshot::StaticLoadSnapshot;

/// The controller semantics two builds must not share when they could analyse one book
/// differently.
///
/// DISCLOSED: every spike build spells `0.0.0`, so today this discriminates nothing. It is the
/// honest value the tree has, and it becomes discriminating the moment the crate is versioned.
pub const CONTROLLER_SEMANTICS: &str = concat!("dorc/", env!("CARGO_PKG_VERSION"));

/// The invocation record, built ONCE so the witness and the Spine cannot describe two runs.
///
/// `argv` and `started_at` arrive as values because obtaining them are QUERIES, and queries stay
/// on the process edge.
#[must_use]
pub fn invocation_record(
    argv: Vec<String>,
    framing: &Framing,
    snapshot: &StaticLoadSnapshot,
    started_at: Option<dorc_core::RunInstant>,
    world_account: dorc_core::influence::InfluenceAccount,
) -> dorc_core::spine::SpineInvocation {
    dorc_core::spine::SpineInvocation::minted(
        dorc_core::spine::InvocationMode::WhylogReplay,
        argv,
        source_claims(snapshot),
        dorc_core::spine::RunIdentity {
            nonce: framing.nonce().0.clone(),
            attempt: framing.attempt(),
            host: framing.host().to_owned(),
            started_at,
        },
        world_account,
    )
}

/// Which surface an invocation asked for, in the planner's own closed vocabulary.
#[must_use]
pub const fn planning_mode(mode: crate::Mode) -> PlanningMode {
    match mode {
        crate::Mode::Bundle => PlanningMode::Bundle,
        crate::Mode::Probe => PlanningMode::Probe,
        crate::Mode::Plan => PlanningMode::Plan,
        crate::Mode::Apply => PlanningMode::Apply,
        crate::Mode::RoundTrip => PlanningMode::RoundTrip,
        crate::Mode::Why => PlanningMode::Why,
    }
}

/// Write the run's durable-arm records onto the Spine (`30E` §2's four species).
///
/// The document is projected from these; nothing here decides what reaches a sink. That
/// separation is the point: the driver states what the run WAS, and one seat decides what a
/// document KEEPS of it.
pub fn record_durable_arm(
    spine: &mut dorc_plan::Spine,
    invocation: dorc_core::spine::SpineInvocation,
    presentation: &FinalPresentation,
    results: &SiteResults,
    records: AdmittedUnscopedHostRecords,
    world_account: dorc_core::influence::InfluenceAccount,
) {
    spine.set_invocation(invocation);
    // The witness minted this identity over the settled surface, so the Spine's record and the
    // projection's row cannot disagree about which surface the run presented.
    spine.set_presented_plan(dorc_core::spine::SpinePresentedPlan::minted(
        presentation.presented_plan(),
        world_account,
    ));
    spine.set_record_stream(dorc_core::spine::SpineRecordStream::minted(
        records,
        results
            .records
            .values()
            .filter_map(|record| Some((record.stamp.ordinal, record.stamp.received_at?)))
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect(),
        world_account,
    ));
}

/// Every acquired source as one ordered role-carrying claim, straight off the snapshot's own
/// triple seat.
///
/// The book is a row wearing `SourceRole::Book`, not a field beside the others. The snapshot's
/// three vectors are indexed by `SourceFileId`, so this vector's order IS the acquired-source
/// table order a document ordinal over it means.
fn source_claims(snapshot: &StaticLoadSnapshot) -> Vec<dorc_core::spine::SourceClaim> {
    snapshot
        .source_claims()
        .map(|(path, src, role)| dorc_core::spine::SourceClaim {
            path: path.to_owned(),
            digest: dorc_plan::invocation::book_digest(src),
            role,
            bytes: u64::try_from(src.len()).unwrap_or(u64::MAX),
        })
        .collect()
}

/// The operating system's randomness: the ONE seat in this binary that asks for any.
///
/// It asks rather than deriving because a receipt identity is required to be collision-resistant,
/// and every value a run already holds that could stand in for one — the session nonce, the
/// clock, the process id — carries only enough entropy to separate one attempt from the next.
///
/// It hands over BYTES and never an identity: the mint stays in the receipt crate's own one file,
/// which is what lets a production source exist without a second place able to spell one.
#[derive(Debug, Default)]
pub struct OsEntropy;

impl ReceiptIdEntropy for OsEntropy {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        getrandom::getrandom(raw).is_ok()
    }
}

/// The production identity source this binary mints documents with.
pub type OsReceiptIdSource = EntropyReceiptIds<OsEntropy>;

/// The capabilities this edge was injected with.
///
/// They travel together because they are one thing: what a run needs in order to turn a decision
/// into a published document. Bundling them is not a signature dodge — it is what stops a caller
/// pairing one run's identity source with another's sink.
pub struct ReceiptCapabilities<'a> {
    ids: &'a mut dyn ReceiptIdSource,
    clock: &'a mut dyn ControllerClock,
    signer: &'a dyn ReceiptSigner,
    sink: &'a mut dyn ReceiptSink,
}

impl<'a> ReceiptCapabilities<'a> {
    /// Bind one run's capabilities.
    pub fn of(
        ids: &'a mut dyn ReceiptIdSource,
        clock: &'a mut dyn ControllerClock,
        signer: &'a dyn ReceiptSigner,
        sink: &'a mut dyn ReceiptSink,
    ) -> Self {
        Self {
            ids,
            clock,
            signer,
            sink,
        }
    }
}

/// The order a document is stamped with, read from a run's own clock.
///
/// Every published document takes ONE reading, so a run's documents order by when each was
/// written rather than sharing one moment. An absent clock answers [`ReceiptOrderToken::UNDATED`]:
/// the token selects a store position and asserts nothing, so a run whose platform could not date
/// it sorts oldest instead of claiming a moment it never observed.
#[derive(Debug)]
pub struct RunClockOrder<'a>(&'a mut crate::results::RunClock);

impl<'a> RunClockOrder<'a> {
    /// Read orders from `clock`.
    pub fn of(clock: &'a mut crate::results::RunClock) -> Self {
        Self(clock)
    }
}

impl ControllerClock for RunClockOrder<'_> {
    fn order_token(&mut self) -> ReceiptOrderToken {
        self.0.now().map_or(ReceiptOrderToken::UNDATED, |instant| {
            ReceiptOrderToken::of_controller_millis(instant.0)
        })
    }
}

impl core::fmt::Debug for ReceiptCapabilities<'_> {
    /// Names the type and no material.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ReceiptCapabilities")
    }
}

/// Why a run published no document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationRefusal {
    /// The Spine did not project.
    Projection(dorc_plan::receipt::ProjectionRefusal),
    /// An apply-side value did not project.
    ApplyProjection(ApplyProjectionRefusal),
    /// A projected row did not satisfy the grammar table.
    Grammar(dorc_receipt::RefusalReason),
    /// The sink did not place the document.
    Sink,
    /// The region and the skeleton do not account for one another exactly.
    OverlayAccount,
    /// The region would be larger than a reader may open, so no document is emitted.
    ///
    /// Refusal rather than omission wherever the required arm is: an intent binds exact bytes,
    /// and a document that left some out could not fund the capability that arm demands.
    RegionOverBound,
    /// The region does not carry each assignment exact image, by value.
    ImageAccount,
    /// The published document's own identity is not a receipt identity.
    Identity,
}

/// Narrow, sign, and publish one PLAIN document, and answer its own identity.
///
/// PLAIN is a statement rather than a shortcut: a projection marks a slot `captured` wherever the
/// run HELD the value, and narrowing is what turns each of those into `withheld-plain`. Reusing
/// that one seat is what keeps a plain document's states honest instead of a second assembly
/// deciding them again — and what makes a plain document a REMINT, taking its own identity.
///
/// # Errors
/// Refuses a row outside the grammar and a sink that declines.
fn narrow_and_publish<D: Species>(
    records: Vec<SkeletonRecord>,
    prefix: &str,
    ids: &mut dyn ReceiptIdSource,
    order: ReceiptOrderToken,
    signer: &dyn ReceiptSigner,
    sink: &mut dyn ReceiptSink,
) -> Result<(String, PublicationGrade), PublicationRefusal> {
    // One clock reading covers both: the assembled value is scaffolding the narrow consumes, and
    // only the narrowed document is ever written, so a second reading would advance a run's clock
    // for a document nobody publishes.
    let assembled = Skeleton {
        receipt_id: ids.next_receipt_id().hex(),
        order,
        signing_key_id: signer.signing_key_id().hex(),
        encryption_key_id: None,
        records,
    };
    let plain = narrow_to_plain(&assembled, ids, order).map_err(PublicationRefusal::Grammar)?;
    let id = plain.receipt_id.clone();
    let name = format!("{prefix}-{id}");
    let document = DraftReceipt::<D, Plain>::of(plain)
        .serialize()
        .map_err(PublicationRefusal::Grammar)?
        .sign(signer);
    document
        .publish(&name, sink)
        .map(|published| (id, published.grade()))
        .map_err(|_| PublicationRefusal::Sink)
}

/// Account, seal, sign, and publish one RICH document.
///
/// The readable skeleton is the plain form's: it carries each slot's state word and never the
/// value, never an offset, and never a name pointing into the region. Enrichment runs the other
/// way — the authenticated region names already-signed slots — which is why the region is built
/// from `captured_slots` rather than from anything the skeleton could be read to request.
///
/// The account is checked HERE, in both directions, before a byte is sealed: a document whose
/// region does not exactly match its own captured slots would be refused by its own reader, and
/// emitting one would turn a writer bug into a reader-side mystery.
///
/// # Errors
/// Refuses a row outside the grammar, a region that does not account for the skeleton exactly, a
/// sealer that declines, or a sink that declines.
fn seal_and_publish<D: Species>(
    skeleton: Skeleton,
    details: &[OverlayEntry],
    prefix: &str,
    limits: &ReceiptLimits,
    signer: &dyn ReceiptSigner,
    sink: &mut dyn ReceiptSink,
    sealer: &dyn OverlaySealer,
) -> Result<PublishedReceipt<D, Rich>, PublicationRefusal> {
    let required = captured_slots(&skeleton);
    let offered: Vec<(u64, OpaqueFieldTag)> = {
        let mut keys: Vec<(u64, OpaqueFieldTag)> = details
            .iter()
            .map(|entry| (entry.record(), entry.tag()))
            .collect();
        keys.sort_by_key(|(record, tag)| (*record, tag.order()));
        keys
    };
    if offered != required {
        return Err(PublicationRefusal::OverlayAccount);
    }

    let span = serialize_skeleton::<D, Rich>(&skeleton).map_err(PublicationRefusal::Grammar)?;
    let plaintext =
        OverlayPlaintext::canonical(&skeleton.receipt_id, D::TOKEN, span.as_bytes(), details);
    if !limits.overlay_bytes.admits(plaintext.opened_bytes()) {
        return Err(PublicationRefusal::RegionOverBound);
    }
    let name = format!("{prefix}-{}", skeleton.receipt_id);
    let document = DraftReceipt::<D, Rich>::of(skeleton)
        .serialize(plaintext, sealer)
        .map_err(PublicationRefusal::Grammar)?
        .sign(signer);
    document
        .publish(&name, sink)
        .map_err(|_| PublicationRefusal::Sink)
}

/// The skeleton a rich document is assembled into, before its region is sealed.
fn rich_skeleton(
    receipt_id: String,
    order: ReceiptOrderToken,
    records: Vec<SkeletonRecord>,
    signer: &dyn ReceiptSigner,
    sealer: &dyn OverlaySealer,
) -> Skeleton {
    Skeleton {
        receipt_id,
        order,
        signing_key_id: signer.signing_key_id().hex(),
        encryption_key_id: Some(sealer.encryption_key_id().hex()),
        records,
    }
}

/// Project, narrow, sign, and publish one plan document.
///
/// # Errors
/// Refuses a Spine that does not project, a row outside the grammar, or a sink that declines.
pub fn publish_plan_receipt(
    spine: &dorc_plan::Spine,
    mode: RecordedInvocationMode,
    world: dorc_core::influence::InfluenceAccount,
    presentation: &FinalPresentation,
    caps: ReceiptCapabilities<'_>,
) -> Result<PublicationGrade, PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        sink,
    } = caps;
    let projected = dorc_plan::receipt::project(spine, mode, world, presentation)
        .map_err(PublicationRefusal::Projection)?;
    let (_, records, _) = projected.into_parts();
    narrow_and_publish::<PlanReceipt>(records, "plan", ids, clock.order_token(), signer, sink)
        .map(|(_, grade)| grade)
}

/// Project, seal, sign, and publish one RICH plan document.
///
/// # Errors
/// Refuses a Spine that does not project, a row outside the grammar, a region that does not
/// account for the skeleton exactly, a sealer that declines, or a sink that declines.
pub fn publish_rich_plan_receipt(
    spine: &dorc_plan::Spine,
    mode: RecordedInvocationMode,
    world: dorc_core::influence::InfluenceAccount,
    presentation: &FinalPresentation,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
    sealer: &dyn OverlaySealer,
) -> Result<PublicationGrade, PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        sink,
    } = caps;
    let projected = dorc_plan::receipt::project(spine, mode, world, presentation)
        .map_err(PublicationRefusal::Projection)?;
    let (_, records, details) = projected.into_parts();
    let skeleton = rich_skeleton(
        ids.next_receipt_id().hex(),
        clock.order_token(),
        records,
        signer,
        sealer,
    );
    seal_and_publish::<PlanReceipt>(skeleton, &details, "plan", limits, signer, sink, sealer)
        .map(|published| published.grade())
}

/// One published rich apply intent: what the publication gate's required arm is assembled from.
///
/// The identity travels beside the document because the gate CONSUMES both members, and an
/// outcome must still name the intent it answers afterwards.
#[derive(Debug)]
pub struct PublishedApplyIntent {
    id: ApplyIntentId,
    receipt: PublishedReceipt<ApplyIntent, Rich>,
    images: ExactApplyImagesPresent,
}

impl PublishedApplyIntent {
    /// The intent's own identity.
    #[must_use]
    pub const fn id(&self) -> ApplyIntentId {
        self.id
    }

    /// How durably the sink placed it.
    #[must_use]
    pub const fn grade(&self) -> PublicationGrade {
        self.receipt.grade()
    }

    /// Take the two values the required publication arm is built from.
    #[must_use]
    pub fn into_gate_parts(self) -> (PublishedReceipt<ApplyIntent, Rich>, ExactApplyImagesPresent) {
        (self.receipt, self.images)
    }
}

/// Project, account, seal, sign, and publish one RICH apply intent.
///
/// The image accounting runs against the entries that are ABOUT TO BE SEALED, keyed by the map
/// the projection recorded while emitting. That ordering is the whole of what the capability
/// means: a document published without it would say `captured` over a region whose bytes nobody
/// compared to the images the apply will run.
///
/// # Errors
/// Refuses an intent that does not project, a region that does not carry every assignment's own
/// image, a row outside the grammar, a region that does not account for the skeleton exactly, a
/// sealer that declines, or a sink that declines.
pub fn publish_apply_intent(
    intent: &PreparedApplyIntent,
    invocation: &ApplyInvocation,
    resolved: dorc_core::influence::InfluenceAccount,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
    sealer: &dyn OverlaySealer,
) -> Result<PublishedApplyIntent, PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        sink,
    } = caps;
    let projected = project_apply_intent(intent, invocation, grade(resolved), limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let images = intent
        .account_images(projected.details(), &|ordinal| projected.record_of(ordinal))
        .ok_or(PublicationRefusal::ImageAccount)?;
    let (_, records, details) = projected.into_parts();
    let id = ApplyIntentId::mint(ids);
    let skeleton = rich_skeleton(id.hex(), clock.order_token(), records, signer, sealer);
    let receipt = seal_and_publish::<ApplyIntent>(
        skeleton,
        &details,
        "apply-intent",
        limits,
        signer,
        sink,
        sealer,
    )?;
    Ok(PublishedApplyIntent {
        id,
        receipt,
        images,
    })
}

/// Project, narrow, sign, and publish one PLAIN apply intent, and answer its own identity.
///
/// Report data, never authority: a plain intent carries `withheld-plain` where the images would
/// be, and there is no route from this seat to the capability the required arm demands. The
/// identity comes back so a later outcome can name the document that was actually written.
///
/// # Errors
/// Refuses an intent that does not project, a row outside the grammar, a sink that declines, and
/// a narrowed document whose identity is not a receipt identity.
pub fn publish_plain_apply_intent(
    intent: &PreparedApplyIntent,
    invocation: &ApplyInvocation,
    resolved: dorc_core::influence::InfluenceAccount,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
) -> Result<(ApplyIntentId, PublicationGrade), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        sink,
    } = caps;
    let projected = project_apply_intent(intent, invocation, grade(resolved), limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let (_, records, _) = projected.into_parts();
    let (spelled, published) = narrow_and_publish::<ApplyIntent>(
        records,
        "apply-intent",
        ids,
        clock.order_token(),
        signer,
        sink,
    )?;
    let id = ApplyIntentId::of_hex(&spelled).ok_or(PublicationRefusal::Identity)?;
    Ok((id, published))
}

/// Project, seal, sign, and publish one RICH apply outcome.
///
/// # Errors
/// Refuses an outcome that does not project — including one naming an assignment the cleared
/// intent never declared — a row outside the grammar, a region that does not account for the
/// skeleton exactly, a sealer that declines, or a sink that declines.
pub fn publish_apply_outcome(
    dispatched: &MutationDispatched,
    report: &ApplyOutcomeReport,
    invocation: &ApplyInvocation,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
    sealer: &dyn OverlaySealer,
) -> Result<(ApplyOutcomeId, PublicationGrade), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        sink,
    } = caps;
    let projected = project_apply_outcome(dispatched, report, invocation, limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let (_, records, details) = projected.into_parts();
    let id = ApplyOutcomeId::mint(ids);
    let skeleton = rich_skeleton(id.hex(), clock.order_token(), records, signer, sealer);
    let receipt = seal_and_publish::<ApplyOutcome>(
        skeleton,
        &details,
        "apply-outcome",
        limits,
        signer,
        sink,
        sealer,
    )?;
    Ok((id, receipt.grade()))
}

/// Project, narrow, sign, and publish one PLAIN apply outcome.
///
/// The degraded terminal report: the route taken when a region cannot be sealed but the run can
/// still say what it reached. Every byte channel narrows to `withheld-plain`.
///
/// # Errors
/// Refuses an outcome that does not project, a row outside the grammar, a sink that declines, and
/// a narrowed document whose identity is not a receipt identity.
pub fn publish_plain_apply_outcome(
    dispatched: &MutationDispatched,
    report: &ApplyOutcomeReport,
    invocation: &ApplyInvocation,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
) -> Result<(ApplyOutcomeId, PublicationGrade), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        sink,
    } = caps;
    let projected = project_apply_outcome(dispatched, report, invocation, limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let (_, records, _) = projected.into_parts();
    let (spelled, published) = narrow_and_publish::<ApplyOutcome>(
        records,
        "apply-outcome",
        ids,
        clock.order_token(),
        signer,
        sink,
    )?;
    let id = ApplyOutcomeId::of_hex(&spelled).ok_or(PublicationRefusal::Identity)?;
    Ok((id, published))
}

/// The recorded grade of one live account.
///
/// The one conversion in this direction, and it goes through the closed token vocabulary rather
/// than a variant, so nothing here decides a grade.
fn grade(account: dorc_core::influence::InfluenceAccount) -> RecordedInfluence {
    RecordedInfluence::of_token(Some(account.label()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::RunClock;

    #[test]
    fn a_ticking_clock_stamps_the_instant_it_read_and_advances() {
        let mut clock = RunClock::Ticking {
            at: dorc_core::RunInstant(1_700_000_000_000),
            step_millis: 5,
        };
        let mut order = RunClockOrder::of(&mut clock);
        let first = order.order_token();
        let second = order.order_token();
        assert_eq!(first.spelled(), "00000001700000000000");
        assert!(
            first < second,
            "two documents of one run take two orders, as they would in a store"
        );
    }

    #[test]
    fn a_clock_that_cannot_answer_stamps_the_lowest_order_rather_than_a_moment() {
        // The direction matters more than the value. A run whose platform could not date it must
        // not out-sort a dated one in a selection that means "most recent", and the wire has no
        // spelling for "no order" — so it under-claims instead of inventing a reading.
        let mut clock = RunClock::Absent;
        let mut order = RunClockOrder::of(&mut clock);
        let undated = order.order_token();
        assert_eq!(undated, ReceiptOrderToken::UNDATED);
        assert!(undated < ReceiptOrderToken::of_controller_millis(1));
    }
}
