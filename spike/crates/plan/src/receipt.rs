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
use dorc_receipt::plan::{
    RecordedAdmission, RecordedLicensor, RecordedLoadDecision, RecordedNarrative,
    RecordedPlanReceipt, RecordedProbeShip, RecordedRegionDecision, RecordedRenderDecision,
    RecordedSiteClassification, RecordedSiteDecision, RecordedSolveCertification, RecordedSource,
    RecordedSurvival, RenderSubject,
};
use dorc_receipt::rows::{
    LoadOrdinal, ModelRefusal, NarrativeOrdinal, RecordedAst, RecordedInvocation, RecordedLeaf,
    RecordedMember, RecordedOperands, RecordedProjectionOmission, RecordedRow, RecordedSite,
    RegionOrdinal, SourceOrdinal,
};
use dorc_receipt::tokens::{
    ClosedToken, OpaqueState, RecordedAdmissionOutcome, RecordedDisposition,
    RecordedInvocationMode, RecordedLicenseCustody, RecordedLicenseVerb, RecordedLoadOutcome,
    RecordedNarrativeKind, RecordedOmissionReason, RecordedRenderKind, RecordedShipLane,
    RecordedSiteClass, RecordedSolvePass, RecordedSourceRole, RecordedSpeechAct,
    RecordedSpineSpecies, RecordedSurvivalOutcome,
};
use dorc_receipt::{RecordedInfluence, RefusalReason, SkeletonRecord};

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
}

/// Project the recorded plan-receipt model from the Spine.
///
/// `mode` comes from the command dispatch seat rather than the Spine: `core`'s own invocation mode
/// is a REPLAY instruction and states nothing about the invocation that produced the record, so
/// deriving the producing shape from it would be a fabricated claim.
///
/// # Errors
/// Refuses a Spine with no invocation, a row the grammar table rejects, a record set that does not
/// close over itself, or a stringly field outside its closed vocabulary.
pub fn project(
    spine: &Spine,
    mode: RecordedInvocationMode,
    world: dorc_core::influence::InfluenceAccount,
) -> Result<RecordedPlanReceipt, ProjectionRefusal> {
    let mut records: Vec<SkeletonRecord> = Vec::new();
    let invocation = spine.invocation().ok_or(ProjectionRefusal::NoInvocation)?;

    push(&mut records, &invocation_row(invocation, mode))?;
    for (ordinal, claim) in invocation.sources().iter().enumerate() {
        push(
            &mut records,
            &source_row(ordinal, claim, invocation.account()),
        )?;
    }
    if let Some(row) = admission_row(spine) {
        push(&mut records, &row)?;
    }
    for record in spine.dispositions() {
        push(&mut records, &site_row(record))?;
        if let Some(row) = licensor_row(record) {
            push(&mut records, &row)?;
        }
    }
    // ONE WALK numbers the regions and feeds the render rows that reference them. The model
    // range-checks a region ordinal and cannot range-check a WRONG one, so numbering in one order
    // and emitting in another would leave every region-keyed render row describing a different
    // region, with the document still validating cleanly.
    let mut region_of: BTreeMap<ElisionRegion, RegionOrdinal> = BTreeMap::new();
    for (position, record) in spine.region_decisions().iter().enumerate() {
        let ordinal = RegionOrdinal::of(count(position));
        region_of.insert(record.region(), ordinal);
        push(&mut records, &region_row(ordinal, record))?;
    }
    for (ordinal, record) in spine.load_decisions().iter().enumerate() {
        push(&mut records, &load_row(ordinal, record))?;
    }
    for record in spine.classifications() {
        push(&mut records, &classification_row(record))?;
    }
    for record in spine.certifications() {
        push(&mut records, &certification_row(record)?)?;
    }
    for record in spine.ships() {
        push(&mut records, &ship_row(record))?;
    }
    for record in spine.survivals() {
        push(&mut records, &survival_row(record))?;
    }
    for record in spine.render_decisions() {
        push(&mut records, &render_row(record, &region_of)?)?;
    }
    for (ordinal, narrative) in spine.narratives().iter().enumerate() {
        push(&mut records, &narrative_row(ordinal, narrative, world))?;
    }
    for row in omission_rows(spine, world) {
        push(&mut records, &row)?;
    }

    RecordedPlanReceipt::of_records(&records).map_err(ProjectionRefusal::Model)
}

fn push<R: RecordedRow>(out: &mut Vec<SkeletonRecord>, row: &R) -> Result<(), ProjectionRefusal> {
    out.push(row.to_record().map_err(ProjectionRefusal::Grammar)?);
    Ok(())
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
        held(!invocation.argv().is_empty()),
        held(!identity.host.is_empty()),
        identity.attempt,
        grade(invocation.account()),
    )
}

fn source_row(
    ordinal: usize,
    claim: &dorc_core::spine::SourceClaim,
    account: dorc_core::influence::InfluenceAccount,
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
        held(!claim.path.is_empty()),
        // V1 selects no source excerpts, so the value exists and this projection did not collect
        // it — which is a different state from the run never having held one.
        OpaqueState::Uncollected,
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
        held(stream.is_some()),
        grade(admission.account()),
    ))
}

const fn disposition_of(decision: &Disposition) -> RecordedDisposition {
    match decision {
        Disposition::Run => RecordedDisposition::Run,
        Disposition::Replace(..) => RecordedDisposition::Replace,
        Disposition::Omit { .. } => RecordedDisposition::Omit,
        Disposition::Guard(_) => RecordedDisposition::Guard,
    }
}

fn site_row(record: &dorc_core::spine::SpineDisposition<PlanPlane>) -> RecordedSiteDecision {
    RecordedSiteDecision::of(
        site_of(record.site()),
        RecordedAst::of(record.ast().0),
        disposition_of(record.decision()),
        held(!record.sh().is_empty()),
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
    let (verb, custody, locus) = match record.decision() {
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
            license.derivation().vouch_span.is_some(),
        ),
        Disposition::Guard(license) => (
            RecordedLicenseVerb::Guard,
            RecordedLicenseCustody::Vouched,
            license.insert().defining_span().is_some(),
        ),
    };
    Some(RecordedLicensor::of(
        site_of(record.site()),
        verb,
        custody,
        held(locus),
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
        held(record.custody().is_some()),
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
        held(record.defining_file().is_some()),
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
        held(record.poisoned_by().is_some()),
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
    RecordedRenderDecision::of(
        subject,
        kind,
        held(matches!(
            record.decision(),
            RenderDecision::PinnedBinding { .. } | RenderDecision::ImportRewritten { .. }
        )),
        grade(record.account()),
    )
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
        | SpineSpecies::RegionDecision => (true, RecordedOmissionReason::NotProjectedV1),
        // The stream's own bytes are an opaque slot on the admission row, never a row of their own.
        SpineSpecies::RecordStream => (false, RecordedOmissionReason::ContentExcluded),
        // The approval-surface identities are not minted yet, so no row can state them.
        SpineSpecies::PresentedPlan => (false, RecordedOmissionReason::NotProjectedV1),
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
