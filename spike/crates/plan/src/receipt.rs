//! The Spine → `PlanReceipt` projection (`quarantine/30Rb`).
//!
//! One lossy transformation, at one seat. Everything the recorded model carries is derived from
//! the Spine the run wrote; nothing is re-observed, and a population this projection does not
//! carry mints an explicit omission row rather than vanishing.

use std::collections::BTreeMap;

use dorc_aid::narrative::{CollapseKind, SpeechAct};
use dorc_core::region::ElisionRegion;
use dorc_core::spine::{
    AdmissionOutcome, InfluenceBearing, RefusalCause, RenderDecision, ShipLane, SpineSiteClass,
    SpineSpecies, SurvivalDemote, SurvivalOutcome, WithheldCause,
};
use dorc_receipt::ids::ApplyArtifactImageId;
use dorc_receipt::overlay::{DocumentRows, OverlayEntry};
use dorc_receipt::plan::{
    RecordedAdmission, RecordedLicensor, RecordedLoadDecision, RecordedNarrative,
    RecordedPlanReceipt, RecordedPresentedPlan, RecordedProbeShip, RecordedRegionDecision,
    RecordedRenderDecision, RecordedSiteClassification, RecordedSiteDecision,
    RecordedSolveCertification, RecordedSource, RecordedSurvival, RenderSubject, SourceSlots,
};
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::rows::{
    LoadOrdinal, ModelRefusal, NarrativeOrdinal, RecordedAst, RecordedInvocation, RecordedLeaf,
    RecordedMember, RecordedOperands, RecordedProjectionOmission, RecordedRow, RecordedSite,
    RegionOrdinal, SourceOrdinal,
};
use dorc_receipt::tokens::{
    ClosedToken, OpaqueState, RecordedAdmissionOutcome, RecordedDisposition,
    RecordedInvocationMode, RecordedLicenseCustody, RecordedLicenseVerb, RecordedLoadOutcome,
    RecordedNarrativeKind, RecordedOmissionReason, RecordedRenderKind, RecordedShipLane,
    RecordedSiteClass, RecordedSolvePass, RecordedSourceClass, RecordedSourceRole,
    RecordedSpeechAct, RecordedSpineSpecies, RecordedSurvivalOutcome,
};
use dorc_receipt::{RecordedInfluence, RefusalReason, SkeletonRecord};

use crate::presentation::FinalPresentation;
use crate::{Disposition, PlanPlane, Spine};

/// Why a Spine did not project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionRefusal {
    /// The run recorded no invocation, so there is no document to write.
    NoInvocation,
    /// A row's atoms did not satisfy the grammar table.
    Grammar(RefusalReason),
    /// The records did not close over one another.
    Model(ModelRefusal),
    /// A stringly Spine field named a token outside its closed receipt vocabulary. Refused rather
    /// than defaulted: a projection that guessed here would record a pass nobody certified.
    UnknownToken {
        /// Which field.
        field: &'static str,
    },
    /// The run recorded no approval surface, so there is nothing for a witness to be checked
    /// against.
    NoPresentedPlan,
    /// The witness names a different approval surface than the Spine recorded.
    PresentationMismatch,
}

/// What one acquired source may contribute to a document beyond its identity.
///
/// The class is the whole of the custody decision (`30Ra:planning-book-bytes-and-durable-locators`):
/// general sh may mutate, so its exact bytes are what a later reader needs to address a historical
/// line; valid `dorc-lang` is mutation-pure by contract and its ordered identity plus digest
/// usually recovers matching current material without multiplying the durable corpus.
///
/// The bytes are BORROWED from what the run already acquired. That is the shape the
/// no-new-observation rule takes at the type level: this value cannot be built from a path, so a
/// projection cannot read a file to fill it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCustody<'bytes> {
    class: RecordedSourceClass,
    acquired: &'bytes str,
}

impl<'bytes> SourceCustody<'bytes> {
    /// A source the dialect gate accepted; its bytes stay out of the document.
    #[must_use]
    pub const fn dorc_lang() -> Self {
        Self {
            class: RecordedSourceClass::DorcLang,
            acquired: "",
        }
    }

    /// A source the dialect gate did not accept; its exact acquired bytes ride the rich overlay.
    #[must_use]
    pub const fn general_sh(acquired: &'bytes str) -> Self {
        Self {
            class: RecordedSourceClass::GeneralSh,
            acquired,
        }
    }
}

/// The run's own inputs, as much of them as a document may carry.
///
/// Handed to [`project`] rather than read off the Spine, because neither half is a Spine record:
/// the acquired bytes belong to the loader and the locators belong to the describe plane. Passing
/// them keeps the projection a pure function of what it was given.
#[derive(Debug, Clone, Default)]
pub struct RecordedInputs<'bytes> {
    sources: Vec<SourceCustody<'bytes>>,
    locators: BTreeMap<dorc_core::SiteId, dorc_receipt::durable_locator::DurableLocator>,
}

impl<'bytes> RecordedInputs<'bytes> {
    /// Bind per-source custody, in the invocation's own source order, and per-site locators.
    #[must_use]
    pub fn of(
        sources: Vec<SourceCustody<'bytes>>,
        locators: BTreeMap<dorc_core::SiteId, dorc_receipt::durable_locator::DurableLocator>,
    ) -> Self {
        Self { sources, locators }
    }

    /// The custody of the source at `ordinal`.
    ///
    /// A source the caller said nothing about is `dorc-lang`-shaped rather than general: the
    /// content slot then reads uncollected, which is the answer that persists nothing. Falling the
    /// other way would have an absent entry mean "write these bytes", and the bytes are not there
    /// to write.
    fn custody(&self, ordinal: usize) -> SourceCustody<'bytes> {
        self.sources
            .get(ordinal)
            .copied()
            .unwrap_or_else(SourceCustody::dorc_lang)
    }
}

/// Project the recorded plan-receipt model from the Spine.
///
/// `mode` comes from the command dispatch seat rather than the Spine: `core`'s own invocation mode
/// is a REPLAY instruction and states nothing about the invocation that produced the record, so
/// deriving the producing shape from it would be a fabricated claim.
///
/// The witness carries the two identities the Spine does not. The one it SHARES with the Spine is
/// what ties it to THIS run: a witness naming another surface refuses, and so does a Spine that
/// recorded none to check against, because a witness cannot vouch for itself.
///
/// # Errors
/// Refuses a Spine with no invocation or no recorded surface, a witness naming a different
/// surface, a row the grammar table rejects, a record set that does not close over itself, or a
/// stringly field outside its closed vocabulary.
pub fn project(
    spine: &Spine,
    mode: RecordedInvocationMode,
    world: dorc_core::influence::InfluenceAccount,
    presentation: &FinalPresentation,
    inputs: &RecordedInputs<'_>,
    limits: &dorc_receipt::limits::ReceiptLimits,
) -> Result<ProjectedPlan, ProjectionRefusal> {
    let mut rows = DocumentRows::default();
    let invocation = spine.invocation().ok_or(ProjectionRefusal::NoInvocation)?;
    let surface = spine
        .presented_plan()
        .ok_or(ProjectionRefusal::NoPresentedPlan)?;
    if *surface.identity() != presentation.presented_plan() {
        return Err(ProjectionRefusal::PresentationMismatch);
    }

    // THE ORDER BELOW IS `PlanReceipt::KINDS`, and it is load-bearing rather than tidy: a detail
    // entry is keyed by its record's POSITION, so a walk that emitted in one order while the
    // model re-emitted in another would enrich whichever row happened to share that integer.
    // `the_projected_order_is_the_canonical_one` is what holds the two together.
    push(
        &mut rows,
        &invocation_row(invocation, mode),
        &[(
            OpaqueFieldTag::TargetName,
            held_bytes(&identity_host(invocation)),
        )],
    )?;
    push_source_rows(&mut rows, invocation, inputs, limits)?;
    if let Some(row) = admission_row(spine) {
        push(&mut rows, &row, &[])?;
    }
    push(
        &mut rows,
        &presented_row(presentation, surface.account()),
        &[],
    )?;
    for record in spine.dispositions() {
        let locator = inputs
            .locators
            .get(&record.site())
            .map(dorc_receipt::durable_locator::DurableLocator::encode);
        push(
            &mut rows,
            &site_row(
                record,
                locator
                    .as_ref()
                    .map_or(OpaqueState::Uncollected, |_| OpaqueState::Captured),
            ),
            &[
                (OpaqueFieldTag::Shell, held_bytes(record.sh())),
                (OpaqueFieldTag::SiteLocator, locator),
            ],
        )?;
    }
    // ONE WALK numbers the regions and feeds the render rows that reference them. The model
    // range-checks a region ordinal and cannot range-check a WRONG one, so numbering in one order
    // and emitting in another would leave every region-keyed render row describing a different
    // region, with the document still validating cleanly.
    let mut region_of: BTreeMap<ElisionRegion, RegionOrdinal> = BTreeMap::new();
    for (position, record) in spine.region_decisions().iter().enumerate() {
        let ordinal = RegionOrdinal::of(count(position));
        region_of.insert(record.region(), ordinal);
        push(
            &mut rows,
            &region_row(ordinal, record),
            &[(OpaqueFieldTag::Shell, held_bytes(record.sh()))],
        )?;
    }
    for (ordinal, record) in spine.load_decisions().iter().enumerate() {
        push(
            &mut rows,
            &load_row(ordinal, record),
            &[(OpaqueFieldTag::ImportPath, held_bytes(record.name()))],
        )?;
    }
    for record in spine.classifications() {
        push(&mut rows, &classification_row(record), &[])?;
    }
    for record in spine.certifications() {
        push(&mut rows, &certification_row(record)?, &[])?;
    }
    for record in spine.ships() {
        push(&mut rows, &ship_row(record), &[])?;
    }
    for record in spine.survivals() {
        push(&mut rows, &survival_row(record), &[])?;
    }
    for record in spine.render_decisions() {
        push(&mut rows, &render_row(record, &region_of)?, &[])?;
    }
    for (ordinal, narrative) in spine.narratives().iter().enumerate() {
        push(&mut rows, &narrative_row(ordinal, narrative, world), &[])?;
    }
    for record in spine.dispositions() {
        if let Some(row) = licensor_row(record) {
            push(&mut rows, &row, &[])?;
        }
    }
    for row in omission_rows(spine, world) {
        push(&mut rows, &row, &[])?;
    }

    let (records, details) = rows.into_parts();
    let model = RecordedPlanReceipt::of_records(&records).map_err(ProjectionRefusal::Model)?;
    Ok(ProjectedPlan {
        model,
        records,
        details,
    })
}

/// One run's document: the typed model, the exact records it emitted, and the detail values
/// belonging to the slots those records marked captured.
///
/// The records travel WITH the details because the details are keyed by record position. Handing
/// back the model alone would leave a caller to re-derive that order, which is the drift this type
/// exists to make unspellable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedPlan {
    model: RecordedPlanReceipt,
    records: Vec<SkeletonRecord>,
    details: Vec<OverlayEntry>,
}

impl ProjectedPlan {
    /// The typed model.
    #[must_use]
    pub const fn model(&self) -> &RecordedPlanReceipt {
        &self.model
    }

    /// The exact records, in the canonical order, ready to become a skeleton.
    #[must_use]
    pub fn records(&self) -> &[SkeletonRecord] {
        &self.records
    }

    /// The detail values, one per captured slot.
    #[must_use]
    pub fn details(&self) -> &[OverlayEntry] {
        &self.details
    }

    /// Take the parts, for a caller assembling a document.
    #[must_use]
    pub fn into_parts(self) -> (RecordedPlanReceipt, Vec<SkeletonRecord>, Vec<OverlayEntry>) {
        (self.model, self.records, self.details)
    }
}

/// Emit one row, and the detail values for whichever of its slots it marked captured.
///
/// A thin skin over the shared accumulator so this projection's own refusal vocabulary reaches
/// its call sites. The rule it wraps — a value is carried only for a slot its own row marked
/// captured — lives at ONE seat, beside the account a reader checks it against.
fn push<R: RecordedRow>(
    rows: &mut DocumentRows,
    row: &R,
    values: &[(OpaqueFieldTag, Option<Vec<u8>>)],
) -> Result<(), ProjectionRefusal> {
    rows.push(row, values).map_err(ProjectionRefusal::Grammar)
}

/// The bytes of a value the run holds, or nothing where it holds none.
fn held_bytes(value: &str) -> Option<Vec<u8>> {
    (!value.is_empty()).then(|| value.as_bytes().to_vec())
}

/// The target this run named, as the invocation recorded it.
fn identity_host(invocation: &dorc_core::spine::SpineInvocation) -> String {
    invocation.identity().host.clone()
}

fn count(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn grade(account: dorc_core::influence::InfluenceAccount) -> RecordedInfluence {
    RecordedInfluence::of_token(Some(account.label()))
}

fn site_of(site: dorc_core::SiteId) -> RecordedSite {
    RecordedSite::of(
        RecordedLeaf::of(site.leaf.0),
        site.member.map(RecordedMember::of),
    )
}

/// A value the run HOLDS that this projection does not collect.
///
/// Distinct from `Unavailable`, which says the run held nothing at all. Every slot wearing this
/// holds something whose durable RENDERING is not designed — a custody, a span, a source-file id,
/// the record stream — and writing one would decide durable content at a projection seat rather
/// than at a reviewed one. The state says exactly that, and the region carries no entry for it.
const UNCOLLECTED: OpaqueState = OpaqueState::Uncollected;

/// Whether the run HOLDS a value — never whether a projection will carry it.
///
/// Narrowing to plain is what turns a held value into `withheld-plain`, so this seat answers the
/// one question it can: did the run have the thing at all.
const fn held(present: bool) -> OpaqueState {
    if present {
        OpaqueState::Captured
    } else {
        OpaqueState::Unavailable
    }
}

fn invocation_row(
    invocation: &dorc_core::spine::SpineInvocation,
    mode: RecordedInvocationMode,
) -> RecordedInvocation {
    let identity = invocation.identity();
    RecordedInvocation::of(
        mode,
        identity.started_at.map(|instant| instant.0),
        UNCOLLECTED,
        held(!identity.host.is_empty()),
        identity.attempt,
        grade(invocation.account()),
    )
}

/// Emit one row per acquired source, spending the content budget across the walk.
///
/// Its own seat because the aggregate is CUMULATIVE: the running total has to thread through every
/// iteration, and a caller that reset it per source would spend the aggregate bound once per file.
fn push_source_rows(
    rows: &mut DocumentRows,
    invocation: &dorc_core::spine::SpineInvocation,
    inputs: &RecordedInputs<'_>,
    limits: &dorc_receipt::limits::ReceiptLimits,
) -> Result<(), ProjectionRefusal> {
    let mut spent: u64 = 0;
    for (ordinal, claim) in invocation.sources().iter().enumerate() {
        let custody = inputs.custody(ordinal);
        let (content, bytes) = source_content(custody, limits, &mut spent);
        push(
            rows,
            &source_row(ordinal, claim, invocation.account(), custody.class, content),
            &[
                (OpaqueFieldTag::SourcePath, held_bytes(&claim.path)),
                (OpaqueFieldTag::SourceContent, bytes),
            ],
        )?;
    }
    Ok(())
}

/// Whether one source's exact bytes fit, and the bytes themselves where they do.
///
/// Two bounds, spent in order, and the aggregate is CUMULATIVE across the walk — which is why this
/// takes the running total by reference rather than answering per source in isolation. A document
/// that admitted every source because each passed the per-source bound is exactly the failure the
/// aggregate exists to stop.
///
/// Over either bound records `omitted-limit` and allocates nothing: the state word is the answer, so
/// a reader learns a bound fired rather than seeing a silently shortened file
/// (`30Rb:book-content-and-locator-projection` — never truncate).
fn source_content(
    custody: SourceCustody<'_>,
    limits: &dorc_receipt::limits::ReceiptLimits,
    spent: &mut u64,
) -> (OpaqueState, Option<Vec<u8>>) {
    if custody.class == RecordedSourceClass::DorcLang {
        return (OpaqueState::Uncollected, None);
    }
    let bytes = custody.acquired.as_bytes();
    let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if len > limits.source_content_bytes.get() {
        return (OpaqueState::OmittedLimit, None);
    }
    let Some(after) = spent.checked_add(len) else {
        return (OpaqueState::OmittedLimit, None);
    };
    if after > limits.source_content_aggregate_bytes.get() {
        return (OpaqueState::OmittedLimit, None);
    }
    *spent = after;
    (OpaqueState::Captured, Some(bytes.to_vec()))
}

fn source_row(
    ordinal: usize,
    claim: &dorc_core::spine::SourceClaim,
    account: dorc_core::influence::InfluenceAccount,
    class: RecordedSourceClass,
    content: OpaqueState,
) -> RecordedSource {
    RecordedSource::of(
        SourceOrdinal::of(count(ordinal)),
        match claim.role {
            dorc_core::SourceRole::Book => RecordedSourceRole::Book,
            dorc_core::SourceRole::NamedLoad => RecordedSourceRole::NamedLoad,
            dorc_core::SourceRole::BookSourced => RecordedSourceRole::BookSourced,
            dorc_core::SourceRole::LoadDependency => RecordedSourceRole::LoadDependency,
            dorc_core::SourceRole::PlainInclusion => RecordedSourceRole::PlainInclusion,
        },
        claim.digest.clone(),
        claim.bytes,
        SourceSlots {
            path: held(!claim.path.is_empty()),
            // V1 selects no source excerpts, so the value exists and this projection did not
            // collect it — a different state from the run never having held one.
            excerpt: OpaqueState::Uncollected,
            content,
        },
        class,
        // A source is a field of the invocation record, so it stands exactly where that record
        // does rather than at a floor nothing computed.
        grade(account),
    )
}

fn admission_row(spine: &Spine) -> Option<RecordedAdmission> {
    let admission = spine.admission()?;
    let stream = spine.record_stream();
    Some(RecordedAdmission::of(
        match admission.outcome() {
            AdmissionOutcome::Admitted => RecordedAdmissionOutcome::Admitted,
            AdmissionOutcome::NoObservation => RecordedAdmissionOutcome::NoObservation,
            AdmissionOutcome::Refused => RecordedAdmissionOutcome::Refused,
        },
        // The instants are SPARSE beside the buffer (a clockless run stamps none), so this counts
        // what the controller timed, not what arrived.
        stream.map_or(0, |record| u64::from(count(record.instants().len()))),
        0,
        UNCOLLECTED,
        grade(admission.account()),
    ))
}

/// The three identities of the surface this run presented.
///
/// The witness supplies all three; the account is the Spine record's own, so the row stands exactly
/// where the run stated the surface stood.
fn presented_row(
    presentation: &FinalPresentation,
    account: dorc_core::influence::InfluenceAccount,
) -> RecordedPresentedPlan {
    RecordedPresentedPlan::of(
        presentation.planning_input().hex(),
        presentation.presented_plan().hex(),
        presentation.planned_image().map(ApplyArtifactImageId::hex),
        grade(account),
    )
}

const fn disposition_of(decision: &Disposition) -> RecordedDisposition {
    match decision {
        Disposition::Run => RecordedDisposition::Run,
        Disposition::Replace(..) => RecordedDisposition::Replace,
        Disposition::Omit { .. } => RecordedDisposition::Omit,
        Disposition::Guard(_) => RecordedDisposition::Guard,
    }
}

fn site_row(
    record: &dorc_core::spine::SpineDisposition<PlanPlane>,
    locator: OpaqueState,
) -> RecordedSiteDecision {
    RecordedSiteDecision::of(
        site_of(record.site()),
        RecordedAst::of(record.ast().0),
        disposition_of(record.decision()),
        held(!record.sh().is_empty()),
        locator,
        grade(record.account()),
    )
}

/// What licensed one irreversible verb, where one was.
///
/// A guard's custody is `Vouched` structurally rather than by a read: its mint consumes exactly one
/// reached verdict vouch, so no other shape is representable. A replacement's is the license's own
/// answer.
fn licensor_row(
    record: &dorc_core::spine::SpineDisposition<PlanPlane>,
) -> Option<RecordedLicensor> {
    let (verb, custody) = match record.decision() {
        Disposition::Run | Disposition::Omit { .. } => return None,
        Disposition::Replace(license, _) => (
            RecordedLicenseVerb::Replace,
            match license.custody() {
                dorc_core::LicenseCustody::Vouched(_) => RecordedLicenseCustody::Vouched,
                dorc_core::LicenseCustody::VouchedSeverally => {
                    RecordedLicenseCustody::VouchedSeverally
                }
                dorc_core::LicenseCustody::MeasuredSelf => RecordedLicenseCustody::MeasuredSelf,
            },
        ),
        Disposition::Guard(_) => (RecordedLicenseVerb::Guard, RecordedLicenseCustody::Vouched),
    };
    Some(RecordedLicensor::of(
        site_of(record.site()),
        verb,
        custody,
        UNCOLLECTED,
        grade(record.account()),
    ))
}

fn region_row(
    ordinal: RegionOrdinal,
    record: &dorc_core::spine::SpineRegionDecision<PlanPlane>,
) -> RecordedRegionDecision {
    RecordedRegionDecision::of(
        ordinal,
        RecordedAst::of(record.ast().0),
        disposition_of(record.decision()),
        u64::from(count(record.routes().total())),
        held(!record.sh().is_empty()),
        grade(record.account()),
    )
}

fn load_row(ordinal: usize, record: &dorc_core::spine::SpineLoadDecision) -> RecordedLoadDecision {
    RecordedLoadDecision::of(
        LoadOrdinal::of(count(ordinal)),
        match record.withheld() {
            None => RecordedLoadOutcome::Bound,
            Some(WithheldCause::Contested) => RecordedLoadOutcome::Contested,
            Some(WithheldCause::Unprovable) => RecordedLoadOutcome::Unprovable,
            Some(WithheldCause::HelperConflict) => RecordedLoadOutcome::HelperConflict,
        },
        held(!record.name().is_empty()),
        UNCOLLECTED,
        grade(record.account()),
    )
}

fn classification_row(
    record: &dorc_core::spine::SpineSiteClassification,
) -> RecordedSiteClassification {
    RecordedSiteClassification::of(
        site_of(record.site()),
        RecordedAst::of(record.ast().0),
        match record.class() {
            SpineSiteClass::MustRun => RecordedSiteClass::MustRun,
            SpineSiteClass::EstablishProbeAmbient => RecordedSiteClass::EstablishProbeAmbient,
            SpineSiteClass::EstablishProbeWritten => RecordedSiteClass::EstablishProbeWritten,
            SpineSiteClass::QueryResolvableValid => RecordedSiteClass::QueryResolvableValid,
            SpineSiteClass::QueryResolvableStale => RecordedSiteClass::QueryResolvableStale,
            SpineSiteClass::EstablishMembersSelfReached => {
                RecordedSiteClass::EstablishMembersSelfReached
            }
            SpineSiteClass::EstablishMembersReached => RecordedSiteClass::EstablishMembersReached,
            SpineSiteClass::InlineCall => RecordedSiteClass::InlineCall,
        },
        record.verdict_lane(),
        record.invalidator(),
        RecordedOperands::of(
            count(record.cells().shown().len()),
            record.cells().dropped(),
        ),
        grade(record.account()),
    )
}

fn certification_row(
    record: &dorc_core::spine::SpineSolveCertification,
) -> Result<RecordedSolveCertification, ProjectionRefusal> {
    Ok(RecordedSolveCertification::of(
        RecordedSolvePass::of_token(record.pass()).ok_or(ProjectionRefusal::UnknownToken {
            field: "solve-certification.pass",
        })?,
        record.consistent(),
        record.tripped(),
        grade(record.account()),
    ))
}

fn ship_row(record: &dorc_core::spine::SpineProbeShip) -> RecordedProbeShip {
    RecordedProbeShip::of(
        site_of(record.site()),
        match record.lane() {
            ShipLane::Verdict => RecordedShipLane::Verdict,
            ShipLane::Predict => RecordedShipLane::Predict,
            ShipLane::Unresolvable => RecordedShipLane::Unresolvable,
        },
        UNCOLLECTED,
        grade(record.account()),
    )
}

fn survival_row(record: &dorc_core::spine::SpineSurvival) -> RecordedSurvival {
    let (outcome, wall, aggregate) = match record.outcome() {
        SurvivalOutcome::Clean => (RecordedSurvivalOutcome::Clean, None, None),
        SurvivalOutcome::SurvivedStandalone => {
            (RecordedSurvivalOutcome::SurvivedStandalone, None, None)
        }
        SurvivalOutcome::SurvivedAggregate { establishes } => (
            RecordedSurvivalOutcome::SurvivedAggregate,
            None,
            Some(establishes),
        ),
        SurvivalOutcome::Demoted(SurvivalDemote::TotalWall) => {
            (RecordedSurvivalOutcome::DemotedTotalWall, None, None)
        }
        SurvivalOutcome::Demoted(SurvivalDemote::Poisoned) => {
            (RecordedSurvivalOutcome::DemotedPoisoned, None, None)
        }
        SurvivalOutcome::Demoted(SurvivalDemote::MayAlias) => {
            (RecordedSurvivalOutcome::DemotedMayAlias, None, None)
        }
        SurvivalOutcome::Demoted(SurvivalDemote::SolveInconsistent) => (
            RecordedSurvivalOutcome::DemotedSolveInconsistent,
            None,
            None,
        ),
        // The ordinal this outcome names indexes the accumulated wall set, and the row's field is
        // typed and documented as a LEAF. Recording one as the other would key the wall to whatever
        // leaf shares that integer, so the projection states the outcome and withholds the number.
        SurvivalOutcome::RederivationDisagreed { wall: _ } => {
            (RecordedSurvivalOutcome::RederivationDisagreed, None, None)
        }
    };
    RecordedSurvival::of(
        // Leaf-keyed by construction: the survival walk answers per elision, never per member.
        RecordedSite::of(RecordedLeaf::of(record.leaf().0), None),
        outcome,
        wall,
        aggregate,
        UNCOLLECTED,
        grade(record.account()),
    )
}

fn render_row(
    record: &dorc_core::spine::SpineRenderDecision,
    region_of: &BTreeMap<ElisionRegion, RegionOrdinal>,
) -> Result<RecordedRenderDecision, ProjectionRefusal> {
    let region_keyed = record.region().is_some();
    let kind = match record.decision() {
        RenderDecision::PinnedBinding { .. } => RecordedRenderKind::PinnedBinding,
        RenderDecision::Refused { cause } => match (region_keyed, cause) {
            (false, RefusalCause::Heredoc) => RecordedRenderKind::RefusedHeredocSite,
            (false, RefusalCause::BlockingRedirect) => {
                RecordedRenderKind::RefusedBlockingRedirectSite
            }
            (true, RefusalCause::Heredoc) => RecordedRenderKind::RefusedHeredocRegion,
            (true, RefusalCause::BlockingRedirect) => {
                RecordedRenderKind::RefusedBlockingRedirectRegion
            }
        },
        RenderDecision::OmitNeutralised { neutralised } => {
            if *neutralised {
                RecordedRenderKind::OmitNeutralised
            } else {
                RecordedRenderKind::OmitNotNeutralised
            }
        }
        RenderDecision::DefensiveEmission { defensive } => {
            if *defensive {
                RecordedRenderKind::DefensiveEmissionOn
            } else {
                RecordedRenderKind::DefensiveEmissionOff
            }
        }
        RenderDecision::CertifierTripDemote => RecordedRenderKind::CertifierTripDemote,
        RenderDecision::ImportRewritten { verb, .. } => {
            if *verb == "inlined" {
                RecordedRenderKind::ImportInlined
            } else {
                RecordedRenderKind::ImportRepointed
            }
        }
    };
    // Minted through the constructor that refuses an axis the kind does not own, so a row keyed by
    // the wrong identity is refused at its mint rather than validating as somebody else's row.
    let subject = match (record.site(), record.region()) {
        (Some(site), _) => RenderSubject::Leaf(site_of(site)),
        (None, Some(region)) => region_of
            .get(&region)
            .copied()
            .map_or(RenderSubject::None, RenderSubject::Region),
        (None, None) => RenderSubject::None,
    };
    RecordedRenderDecision::of(subject, kind, UNCOLLECTED, grade(record.account()))
        .map_err(|fault| ProjectionRefusal::Model(fault.into()))
}

const fn no_operands() -> RecordedOperands {
    RecordedOperands::of(0, 0)
}

fn narrative_row(
    ordinal: usize,
    narrative: &dorc_aid::CollapseNarrative,
    world: dorc_core::influence::InfluenceAccount,
) -> RecordedNarrative {
    let (kind, operands) = match narrative.kind() {
        CollapseKind::FactMergeDisagreement { operands, .. } => (
            RecordedNarrativeKind::FactMergeDisagreement,
            RecordedOperands::of(count(operands.kept().len()), operands.truncated()),
        ),
        CollapseKind::SolverConsistencyFailure { operands, .. } => (
            RecordedNarrativeKind::SolverConsistencyFailure,
            RecordedOperands::of(count(operands.kept().len()), operands.truncated()),
        ),
        CollapseKind::VerdictDecline { .. } => {
            (RecordedNarrativeKind::VerdictDecline, no_operands())
        }
        CollapseKind::WallFormation { .. } => (RecordedNarrativeKind::WallFormation, no_operands()),
        CollapseKind::SubstitutionRefusal { .. } => {
            (RecordedNarrativeKind::SubstitutionRefusal, no_operands())
        }
        CollapseKind::EntryDenial { .. } => (RecordedNarrativeKind::EntryDenial, no_operands()),
        CollapseKind::WrapperPairIncoherent { .. } => {
            (RecordedNarrativeKind::WrapperPairIncoherent, no_operands())
        }
        CollapseKind::EntryFailure { .. } => (RecordedNarrativeKind::EntryFailure, no_operands()),
        CollapseKind::Demotion { .. } => (RecordedNarrativeKind::Demotion, no_operands()),
        CollapseKind::RenderRefusal { .. } => (RecordedNarrativeKind::RenderRefusal, no_operands()),
        CollapseKind::FixpointCapDegrade { .. } => {
            (RecordedNarrativeKind::FixpointCapDegrade, no_operands())
        }
        CollapseKind::RoleFamilyShadowed { .. } => {
            (RecordedNarrativeKind::RoleFamilyShadowed, no_operands())
        }
        CollapseKind::CompositionSuspended { .. } => {
            (RecordedNarrativeKind::CompositionSuspended, no_operands())
        }
        CollapseKind::ProjectionDrop { .. } => {
            (RecordedNarrativeKind::ProjectionDrop, no_operands())
        }
        // Uninhabited: the arm holds a slot and cannot be constructed, so the population is
        // provably empty rather than merely unobserved.
        CollapseKind::Cancellation(reserved) => match *reserved {},
    };
    RecordedNarrative::of(
        NarrativeOrdinal::of(count(ordinal)),
        match narrative.tier() {
            SpeechAct::Measured => RecordedSpeechAct::Measured,
            SpeechAct::Vouched => RecordedSpeechAct::Vouched,
            SpeechAct::Ran => RecordedSpeechAct::Ran,
            SpeechAct::Claimed => RecordedSpeechAct::Claimed,
            SpeechAct::Derived => RecordedSpeechAct::Derived,
            SpeechAct::Consented => RecordedSpeechAct::Consented,
            SpeechAct::Declined => RecordedSpeechAct::Declined,
        },
        kind,
        operands,
        // A narrative carries no account of its own. It was minted DURING this run, so it stands
        // where the run stands — the projection continuing the flow it was handed, never a grade
        // this seat decided (`306b:rul-projections-continue-influence-flow`).
        grade(world),
    )
}

/// One row per in-memory species this projection did not carry, with the population it declined.
///
/// A census over every species rather than a hand-kept list: a new species cannot land without an
/// answer in [`carriage`], and an uncarried population is stated rather than vanishing.
fn omission_rows(
    spine: &Spine,
    world: dorc_core::influence::InfluenceAccount,
) -> Vec<RecordedProjectionOmission> {
    SpineSpecies::ALL
        .iter()
        .filter_map(|species| {
            let (carried, reason) = carriage(*species);
            (!carried).then(|| {
                RecordedProjectionOmission::of(
                    recorded_species(*species),
                    spine.population(*species),
                    reason,
                    grade(world),
                )
            })
        })
        .collect()
}

/// Whether this projection carries a species, and why not where it does not.
///
/// No wildcard arm: a new species stops this compiling until it is answered for.
const fn carriage(species: SpineSpecies) -> (bool, RecordedOmissionReason) {
    match species {
        SpineSpecies::Invocation
        | SpineSpecies::Disposition
        | SpineSpecies::LoadDecision
        | SpineSpecies::SiteClassification
        | SpineSpecies::SolveCertification
        | SpineSpecies::ProbeShip
        | SpineSpecies::Admission
        | SpineSpecies::Survival
        | SpineSpecies::RenderDecision
        | SpineSpecies::RegionDecision
        | SpineSpecies::PresentedPlan => (true, RecordedOmissionReason::NotProjectedV1),
        // The stream's own bytes are an opaque slot on the admission row, never a row of their own.
        SpineSpecies::RecordStream => (false, RecordedOmissionReason::ContentExcluded),
        // The run outcome belongs to the apply-outcome document, never to a plan receipt.
        SpineSpecies::Vouch
        | SpineSpecies::Observation
        | SpineSpecies::ValidityRound
        | SpineSpecies::Outcome => (false, RecordedOmissionReason::Unminted),
    }
}

const fn recorded_species(species: SpineSpecies) -> RecordedSpineSpecies {
    match species {
        SpineSpecies::Invocation => RecordedSpineSpecies::Invocation,
        SpineSpecies::RecordStream => RecordedSpineSpecies::RecordStream,
        SpineSpecies::Disposition => RecordedSpineSpecies::Disposition,
        SpineSpecies::PresentedPlan => RecordedSpineSpecies::PresentedPlan,
        SpineSpecies::LoadDecision => RecordedSpineSpecies::LoadDecision,
        SpineSpecies::SiteClassification => RecordedSpineSpecies::SiteClassification,
        SpineSpecies::SolveCertification => RecordedSpineSpecies::SolveCertification,
        SpineSpecies::Vouch => RecordedSpineSpecies::Vouch,
        SpineSpecies::ProbeShip => RecordedSpineSpecies::ProbeShip,
        SpineSpecies::Admission => RecordedSpineSpecies::Admission,
        SpineSpecies::Observation => RecordedSpineSpecies::Observation,
        SpineSpecies::ValidityRound => RecordedSpineSpecies::ValidityRound,
        SpineSpecies::Survival => RecordedSpineSpecies::Survival,
        SpineSpecies::RenderDecision => RecordedSpineSpecies::RenderDecision,
        SpineSpecies::RegionDecision => RecordedSpineSpecies::RegionDecision,
        SpineSpecies::Outcome => RecordedSpineSpecies::Outcome,
    }
}
