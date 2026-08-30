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
use dorc_receipt::capability::{OverlaySealer, PublicationGrade, ReceiptSigner};
use dorc_receipt::dispatch::{
    IntentPublicationMismatch, MutationDispatched, PreparedApplyIntent, PublicationThrough,
    PublishedApplyIntentV1, RequiredPlacementLanding,
};
use dorc_receipt::format::{Skeleton, serialize_skeleton};
use dorc_receipt::ids::{
    ApplyIntentId, ApplyOutcomeId, EntropyReceiptIds, PlanReceiptId, ReceiptIdEntropy,
    ReceiptIdSource,
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
use dorc_receipt::writer::{DraftReceipt, OverlayPlaintext, SignedReceipt};
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

/// The recorded invocation shape for a mode, in the document's own closed vocabulary.
///
/// `Bundle`, `Probe`, `Plan` and `Why` all record as a planning invocation: the three-word
/// vocabulary says what a document was produced BY, and none of those applies anything.
#[must_use]
pub const fn recorded_mode(mode: crate::Mode) -> RecordedInvocationMode {
    match mode {
        crate::Mode::Apply => RecordedInvocationMode::Apply,
        crate::Mode::RoundTrip => RecordedInvocationMode::RoundTrip,
        crate::Mode::Bundle | crate::Mode::Probe | crate::Mode::Plan | crate::Mode::Why => {
            RecordedInvocationMode::Plan
        }
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

/// Why a document was not placed.
///
/// Closed and placement-neutral: the words a report needs, rather than any one destination's own
/// internal arms. A placement that knows more maps into these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementFailure {
    /// The document is larger than the placement admits.
    OverBound,
    /// The name this document would take is already taken; nothing was replaced.
    NameAlreadyTaken,
    /// The destination could not be reached or written to.
    Unusable,
    /// Bytes were created but the write or its synchronization did not complete.
    NotDurable,
    /// The document carries no order, and this placement files by order.
    UndatedDocument,
    /// The placement declined without a further word.
    Declined,
}

/// Where one document went, and how durably.
///
/// Carries its own identity and its own path rather than dropping them: a later surface naming
/// the durable a run wrote can only do so if the seat that wrote it said where it went, and
/// recovering that afterwards is archaeology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedDocument {
    receipt_id: String,
    name: String,
    path: Option<String>,
    grade: PublicationGrade,
}

impl PlacedDocument {
    /// Record one placement.
    #[must_use]
    pub const fn of(
        receipt_id: String,
        name: String,
        path: Option<String>,
        grade: PublicationGrade,
    ) -> Self {
        Self {
            receipt_id,
            name,
            path,
            grade,
        }
    }

    /// The identity of the document placed.
    #[must_use]
    pub fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    /// The name it was filed under.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Where it landed, where the placement is one with paths.
    #[must_use]
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// How durably the placement reported holding it.
    #[must_use]
    pub const fn grade(&self) -> PublicationGrade {
        self.grade
    }
}

/// One placed rich intent: where it went, and what its placement reported landing.
#[derive(Debug)]
pub struct PlacedIntent {
    /// Where the document went.
    pub placed: PlacedDocument,
    /// What the placement reports about the landing, which a required publication reads.
    pub landing: RequiredPlacementLanding,
}

/// Where this run's signed documents are placed.
///
/// One method per document a run writes, over concrete species — never a generic
/// `place(name, bytes)`. That shape is deliberate: a store mints its own filename from the typed
/// identity, so there is no string a caller could hand it and no place a caller could aim a
/// publication at.
///
/// Production has exactly ONE implementor, the local store's. A fixture placement is a test's own
/// value, which is what keeps a volatile destination structurally unable to answer a production
/// route.
pub trait ReceiptPlacement {
    /// Place one rich plan document.
    ///
    /// # Errors
    /// Answers the closed word for what the placement refused.
    fn place_plan(
        &mut self,
        id: PlanReceiptId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<PlanReceipt, Rich>,
    ) -> Result<PlacedDocument, PlacementFailure>;

    /// Place one plain plan document.
    ///
    /// # Errors
    /// Answers the closed word for what the placement refused.
    fn place_plain_plan(
        &mut self,
        id: PlanReceiptId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<PlanReceipt, Plain>,
    ) -> Result<PlacedDocument, PlacementFailure>;

    /// Place one rich apply intent, answering what its landing reports.
    ///
    /// The identity is handed IN by the required publication rather than chosen here, so a
    /// document is filed under the identity the publication will record and no other.
    ///
    /// # Errors
    /// Answers the closed word for what the placement refused.
    fn place_intent(
        &mut self,
        id: ApplyIntentId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyIntent, Rich>,
    ) -> Result<PlacedIntent, PlacementFailure>;

    /// Place one plain apply intent. Report data: no durability is proved and none is answered.
    ///
    /// # Errors
    /// Answers the closed word for what the placement refused.
    fn place_plain_intent(
        &mut self,
        id: ApplyIntentId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyIntent, Plain>,
    ) -> Result<PlacedDocument, PlacementFailure>;

    /// Place one rich apply outcome.
    ///
    /// # Errors
    /// Answers the closed word for what the placement refused.
    fn place_outcome(
        &mut self,
        id: ApplyOutcomeId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyOutcome, Rich>,
    ) -> Result<PlacedDocument, PlacementFailure>;

    /// Place one plain apply outcome, the degraded terminal report.
    ///
    /// # Errors
    /// Answers the closed word for what the placement refused.
    fn place_plain_outcome(
        &mut self,
        id: ApplyOutcomeId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyOutcome, Plain>,
    ) -> Result<PlacedDocument, PlacementFailure>;
}

/// The capabilities this edge was injected with.
///
/// They travel together because they are one thing: what a run needs in order to turn a decision
/// into a published document. Bundling them is not a signature dodge — it is what stops a caller
/// pairing one run's identity source with another's placement.
pub struct ReceiptCapabilities<'a> {
    ids: &'a mut dyn ReceiptIdSource,
    clock: &'a mut dyn ControllerClock,
    signer: &'a dyn ReceiptSigner,
    placement: &'a mut dyn ReceiptPlacement,
}

impl<'a> ReceiptCapabilities<'a> {
    /// Bind one run's capabilities.
    pub fn of(
        ids: &'a mut dyn ReceiptIdSource,
        clock: &'a mut dyn ControllerClock,
        signer: &'a dyn ReceiptSigner,
        placement: &'a mut dyn ReceiptPlacement,
    ) -> Self {
        Self {
            ids,
            clock,
            signer,
            placement,
        }
    }
}

/// The order a document is stamped with, read from a run own clock.
///
/// Every published document takes ONE reading, so a run documents order by when each was written
/// rather than sharing one moment. An absent clock answers [`ReceiptOrderToken::UNDATED`], which
/// this adapter carries faithfully — the undated token is a supported value, not a failure.
///
/// OWED, and deliberately not sited here: the PRODUCTION composition root must refuse to EMIT an
/// undated document, so a store that selects by order never has one to sort. This adapter is a
/// lib seam that a test drives as readily as the binary, so a refusal here would refuse the very
/// runs that want an undated artifact. The seat is `LocalReceiptEdgeV1`, which does not exist yet;
/// nothing in the binary publishes today, so there is no live exposure.
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
    /// The placement did not place the document.
    Placement(PlacementFailure),
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
    /// The intent's own policy is not one a required publication answers.
    GateMismatch(IntentPublicationMismatch),
}

impl PublicationRefusal {
    /// The closed word a report names this refusal by.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Projection(_) | Self::ApplyProjection(_) => "projection",
            Self::Grammar(_) => "grammar",
            Self::Placement(PlacementFailure::OverBound) => "over-bound",
            Self::Placement(PlacementFailure::NameAlreadyTaken) => "name-taken",
            Self::Placement(PlacementFailure::Unusable) => "store-unusable",
            Self::Placement(PlacementFailure::NotDurable) => "not-durable",
            Self::Placement(PlacementFailure::UndatedDocument) => "undated",
            Self::Placement(PlacementFailure::Declined) => "declined",
            Self::OverlayAccount => "overlay-account",
            Self::RegionOverBound => "region-over-bound",
            Self::ImageAccount => "image-account",
            Self::Identity => "identity",
            Self::GateMismatch(_) => "gate-mismatch",
        }
    }
}

/// One run, as the two plan-receipt publications read it.
///
/// A borrowed request rather than loose arguments, so the seat holding the keys receives one
/// coherent description and cannot pair one run's Spine with another's presentation or inputs.
/// Both projections read the same value, which is what keeps plain a NARROWING of rich rather than
/// a second assembly that could collect differently.
#[derive(Debug)]
pub struct RecordedRun<'a> {
    /// The settled decision plane.
    pub spine: &'a dorc_plan::Spine,
    /// Which surface the invocation asked for, in the recorded vocabulary.
    pub mode: RecordedInvocationMode,
    /// The run's own influence account.
    pub world: dorc_core::influence::InfluenceAccount,
    /// The final presentation the identities were minted over.
    pub presentation: &'a FinalPresentation,
    /// Exact general-sh source bytes, and one locator per decided site.
    pub inputs: &'a dorc_plan::receipt::RecordedInputs<'a>,
    /// The bounds every projection and seal is spent against.
    pub limits: &'a ReceiptLimits,
}

impl RecordedRun<'_> {
    /// Project this run's rows, in the one vocabulary both publications use.
    fn project(&self) -> Result<dorc_plan::receipt::ProjectedPlan, PublicationRefusal> {
        dorc_plan::receipt::project(
            self.spine,
            self.mode,
            self.world,
            self.presentation,
            self.inputs,
            self.limits,
        )
        .map_err(PublicationRefusal::Projection)
    }
}

/// Narrow and sign one PLAIN document, and answer its own reminted identity beside its bytes.
///
/// PLAIN is a statement rather than a shortcut: a projection marks a slot `captured` wherever the
/// run HELD the value, and narrowing is what turns each of those into `withheld-plain`. Reusing
/// that one seat is what keeps a plain document's states honest instead of a second assembly
/// deciding them again — and what makes a plain document a REMINT, taking its own identity.
///
/// # Errors
/// Refuses a row outside the grammar.
fn narrow_and_sign<D: Species>(
    records: Vec<SkeletonRecord>,
    ids: &mut dyn ReceiptIdSource,
    order: ReceiptOrderToken,
    signer: &dyn ReceiptSigner,
) -> Result<(String, SignedReceipt<D, Plain>), PublicationRefusal> {
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
    let document = DraftReceipt::<D, Plain>::of(plain)
        .serialize()
        .map_err(PublicationRefusal::Grammar)?
        .sign(signer);
    Ok((id, document))
}

/// Account, seal and sign one RICH document.
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
/// Refuses a row outside the grammar, a region that does not account for the skeleton exactly, or
/// a sealer that declines.
fn seal_and_sign<D: Species>(
    skeleton: Skeleton,
    details: &[OverlayEntry],
    limits: &ReceiptLimits,
    signer: &dyn ReceiptSigner,
    sealer: &dyn OverlaySealer,
) -> Result<SignedReceipt<D, Rich>, PublicationRefusal> {
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
    Ok(DraftReceipt::<D, Rich>::of(skeleton)
        .serialize(plaintext, sealer)
        .map_err(PublicationRefusal::Grammar)?
        .sign(signer))
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

/// Project, narrow, sign, and place one PLAIN plan document.
///
/// # Errors
/// Refuses a Spine that does not project, a row outside the grammar, a narrowed document whose
/// identity is not a receipt identity, or a placement that declines.
pub fn publish_plan_receipt(
    run: &RecordedRun<'_>,
    caps: ReceiptCapabilities<'_>,
) -> Result<PlacedDocument, PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        placement,
    } = caps;
    // The plain path projects the SAME rows the rich one would and then narrows them: a plain
    // document says `withheld-plain` where the run held a value, which is a statement about
    // custody rather than a projection that collected less.
    let projected = run.project()?;
    let (_, records, _) = projected.into_parts();
    let order = clock.order_token();
    let (spelled, document) = narrow_and_sign::<PlanReceipt>(records, ids, order, signer)?;
    let id = PlanReceiptId::of_hex(&spelled).ok_or(PublicationRefusal::Identity)?;
    placement
        .place_plain_plan(id, order, document)
        .map_err(PublicationRefusal::Placement)
}

/// Project, seal, sign, and place one RICH plan document.
///
/// # Errors
/// Refuses a Spine that does not project, a row outside the grammar, a region that does not
/// account for the skeleton exactly, a sealer that declines, or a placement that declines.
pub fn publish_rich_plan_receipt(
    run: &RecordedRun<'_>,
    caps: ReceiptCapabilities<'_>,
    sealer: &dyn OverlaySealer,
) -> Result<PlacedDocument, PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        placement,
    } = caps;
    let projected = run.project()?;
    let (_, records, details) = projected.into_parts();
    let id = PlanReceiptId::mint(ids);
    let order = clock.order_token();
    let skeleton = rich_skeleton(id.hex(), order, records, signer, sealer);
    let document = seal_and_sign::<PlanReceipt>(skeleton, &details, run.limits, signer, sealer)?;
    placement
        .place_plan(id, order, document)
        .map_err(PublicationRefusal::Placement)
}

/// Project, account, seal, sign, and place one RICH apply intent.
///
/// The image accounting runs against the entries that are ABOUT TO BE SEALED, keyed by the map
/// the projection recorded while emitting. That ordering is the whole of what the capability
/// means: a document published without it would say `captured` over a region whose bytes nobody
/// compared to the images the apply will run.
///
/// The intent arrives BY VALUE and is never handed back. It is consumed into the accounted
/// state and then into the publication, so what comes back owns the exact intent it was earned
/// for and the caller has nothing left to pair a second publication with.
///
/// # Errors
/// Refuses an intent that does not project, a region that does not carry every assignment's own
/// image, a row outside the grammar, a region that does not account for the skeleton exactly, a
/// sealer that declines, a placement that declines, and an intent whose own policy is not the
/// required one.
pub fn publish_apply_intent(
    intent: PreparedApplyIntent,
    invocation: &ApplyInvocation,
    resolved: dorc_core::influence::InfluenceAccount,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
    sealer: &dyn OverlaySealer,
) -> Result<(PublishedApplyIntentV1, PlacedDocument), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        placement,
    } = caps;
    let projected = project_apply_intent(&intent, invocation, grade(resolved), limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let accounted = intent
        .account_images(projected.details(), &|ordinal| projected.record_of(ordinal))
        .ok_or(PublicationRefusal::ImageAccount)?;
    let (_, records, details) = projected.into_parts();
    let id = ApplyIntentId::mint(ids);
    let order = clock.order_token();
    let skeleton = rich_skeleton(id.hex(), order, records, signer, sealer);
    let document = seal_and_sign::<ApplyIntent>(skeleton, &details, limits, signer, sealer)?;
    // Taken over the bytes about to be handed over, so what comes back can be compared against
    // what went in: a placement that filed some other document is a refusal rather than a
    // publication naming bytes nobody wrote.
    let sealed = dorc_receipt::ids::Sha256Digest::over(
        dorc_receipt::dispatch::REQUIRED_PLACEMENT_DIGEST_DOMAIN,
        document.bytes(),
    );
    // The placement is CALLED FROM INSIDE the publication, with the identity the publication
    // will record. There is no route by which a publication value exists without this call
    // having happened, which is the whole of what replaced a separately-mintable proof.
    accounted
        .publish_through(id, sealed, |id| {
            placement
                .place_intent(id, order, document)
                .map(|PlacedIntent { placed, landing }| (landing, placed))
        })
        .map_err(|through| match through {
            PublicationThrough::Placement(failure) => PublicationRefusal::Placement(failure),
            PublicationThrough::Mismatch(mismatch) => PublicationRefusal::GateMismatch(mismatch),
        })
}

/// Project, narrow, sign, and place one PLAIN apply intent.
///
/// Report data, never authority: a plain intent carries `withheld-plain` where the images would
/// be, and there is no route from this seat to the capability the required arm demands.
///
/// # Errors
/// Refuses an intent that does not project, a row outside the grammar, a placement that declines,
/// and a narrowed document whose identity is not a receipt identity.
pub fn publish_plain_apply_intent(
    intent: &PreparedApplyIntent,
    invocation: &ApplyInvocation,
    resolved: dorc_core::influence::InfluenceAccount,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
) -> Result<(ApplyIntentId, PlacedDocument), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        placement,
    } = caps;
    let projected = project_apply_intent(intent, invocation, grade(resolved), limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let (_, records, _) = projected.into_parts();
    let order = clock.order_token();
    let (spelled, document) = narrow_and_sign::<ApplyIntent>(records, ids, order, signer)?;
    let id = ApplyIntentId::of_hex(&spelled).ok_or(PublicationRefusal::Identity)?;
    let placed = placement
        .place_plain_intent(id, order, document)
        .map_err(PublicationRefusal::Placement)?;
    Ok((id, placed))
}

/// Project, seal, sign, and place one RICH apply outcome.
///
/// # Errors
/// Refuses an outcome that does not project — including one naming an assignment the cleared
/// intent never declared — a row outside the grammar, a region that does not account for the
/// skeleton exactly, a sealer that declines, or a placement that declines.
pub fn publish_apply_outcome(
    dispatched: &MutationDispatched,
    report: &ApplyOutcomeReport,
    invocation: &ApplyInvocation,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
    sealer: &dyn OverlaySealer,
) -> Result<(ApplyOutcomeId, PlacedDocument), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        placement,
    } = caps;
    let projected = project_apply_outcome(dispatched, report, invocation, limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let (_, records, details) = projected.into_parts();
    let id = ApplyOutcomeId::mint(ids);
    let order = clock.order_token();
    let skeleton = rich_skeleton(id.hex(), order, records, signer, sealer);
    let document = seal_and_sign::<ApplyOutcome>(skeleton, &details, limits, signer, sealer)?;
    let placed = placement
        .place_outcome(id, order, document)
        .map_err(PublicationRefusal::Placement)?;
    Ok((id, placed))
}

/// Project, narrow, sign, and place one PLAIN apply outcome.
///
/// The degraded terminal report: the route taken when a region cannot be sealed but the run can
/// still say what it reached. Every byte channel narrows to `withheld-plain`.
///
/// # Errors
/// Refuses an outcome that does not project, a row outside the grammar, a placement that
/// declines, and a narrowed document whose identity is not a receipt identity.
pub fn publish_plain_apply_outcome(
    dispatched: &MutationDispatched,
    report: &ApplyOutcomeReport,
    invocation: &ApplyInvocation,
    limits: &ReceiptLimits,
    caps: ReceiptCapabilities<'_>,
) -> Result<(ApplyOutcomeId, PlacedDocument), PublicationRefusal> {
    let ReceiptCapabilities {
        ids,
        clock,
        signer,
        placement,
    } = caps;
    let projected = project_apply_outcome(dispatched, report, invocation, limits)
        .map_err(PublicationRefusal::ApplyProjection)?;
    let (_, records, _) = projected.into_parts();
    let order = clock.order_token();
    let (spelled, document) = narrow_and_sign::<ApplyOutcome>(records, ids, order, signer)?;
    let id = ApplyOutcomeId::of_hex(&spelled).ok_or(PublicationRefusal::Identity)?;
    let placed = placement
        .place_plain_outcome(id, order, document)
        .map_err(PublicationRefusal::Placement)?;
    Ok((id, placed))
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
    fn a_clock_that_cannot_answer_carries_the_undated_token_faithfully() {
        // The adapter CARRIES an absent clock rather than inventing a reading, because an undated
        // receipt is a supported artifact. What stops an undated document reaching a store that
        // selects by order is a refusal at the production composition root — see this seat own
        // doc comment for why it does not live here.
        let mut clock = RunClock::Absent;
        let mut order = RunClockOrder::of(&mut clock);
        let undated = order.order_token();
        assert_eq!(undated, ReceiptOrderToken::UNDATED);
        assert!(undated < ReceiptOrderToken::of_controller_millis(1));
    }
}
