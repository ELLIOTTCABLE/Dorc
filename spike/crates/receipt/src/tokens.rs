//! The receipt-local closed vocabularies: one enum per token set in [`crate::grammar`].
//!
//! Each enum projects a source enum exhaustively. [`ClosedToken::token`] is a match with no
//! wildcard arm, so a new variant stops the crate compiling until it is spelled, and the census in
//! this module pairs every variant with a token in the grammar constant, both ways. Two source
//! variants may not share one token where they differ in what they ask a reader to repair, what
//! they license, or how they were known.

use crate::grammar;

/// One closed vocabulary: a fixed variant set, a fixed token set, and one spelling each way.
pub trait ClosedToken: Sized + Copy + PartialEq + 'static {
    /// The grammar constant this vocabulary must agree with, in both directions.
    const TOKENS: &'static [&'static str];
    /// Every variant, in the token set order.
    const ALL: &'static [Self];

    /// The literal word this variant spells.
    fn token(self) -> &'static str;

    /// The variant a literal word names.
    fn of_token(text: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.token() == text)
    }
}

/// The boolean spelling.
#[must_use]
pub const fn bool_token(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// Read a boolean from its exact spelling.
#[must_use]
pub fn bool_of_token(text: &str) -> Option<bool> {
    match text {
        "yes" => Some(true),
        "no" => Some(false),
        _ => None,
    }
}

/// What a document holds in place of an opaque value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpaqueState {
    /// The value is in the encrypted region.
    Captured,
    /// The projection has no region to carry it.
    WithheldPlain,
    /// The run never held the value.
    Unavailable,
    /// The value exists and this projection did not collect it.
    Uncollected,
    /// A bound stopped the value being carried.
    OmittedLimit,
}

impl ClosedToken for OpaqueState {
    const TOKENS: &'static [&'static str] = grammar::OPAQUE_STATE;
    const ALL: &'static [Self] = &[
        Self::Captured,
        Self::WithheldPlain,
        Self::Unavailable,
        Self::Uncollected,
        Self::OmittedLimit,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::WithheldPlain => "withheld-plain",
            Self::Unavailable => "unavailable",
            Self::Uncollected => "uncollected",
            Self::OmittedLimit => "omitted-limit",
        }
    }
}

/// What a document holds in place of an apply image. Narrower than [`OpaqueState`]: an image is
/// never partially collected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageState {
    /// The image is in the encrypted region.
    Captured,
    /// The projection has no region to carry it.
    WithheldPlain,
    /// The run never held the image.
    Unavailable,
    /// A bound stopped the image being carried.
    OmittedLimit,
}

impl ClosedToken for ImageState {
    const TOKENS: &'static [&'static str] = grammar::IMAGE_STATE;
    const ALL: &'static [Self] = &[
        Self::Captured,
        Self::WithheldPlain,
        Self::Unavailable,
        Self::OmittedLimit,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::WithheldPlain => "withheld-plain",
            Self::Unavailable => "unavailable",
            Self::OmittedLimit => "omitted-limit",
        }
    }
}

/// The invocation shape that produced a document. Minted from the command dispatch seat, never from
/// the analyzer own invocation vocabulary, whose inhabitant set is ruled elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedMode {
    /// A planning invocation.
    Plan,
    /// An applying invocation.
    Apply,
    /// One invocation that planned and applied.
    RoundTrip,
}

impl ClosedToken for RecordedMode {
    const TOKENS: &'static [&'static str] = grammar::MODE;
    const ALL: &'static [Self] = &[Self::Plan, Self::Apply, Self::RoundTrip];
    fn token(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Apply => "apply",
            Self::RoundTrip => "round-trip",
        }
    }
}

/// What a recorded source was to the run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSourceRole {
    /// The runbook.
    Book,
    /// A source the command line named.
    NamedLoad,
    /// A source the book reached.
    BookSourced,
    /// A source another load reached.
    LoadDependency,
    /// An ordinary shell file acquired and shipped unanalysed.
    PlainInclusion,
}

impl ClosedToken for RecordedSourceRole {
    const TOKENS: &'static [&'static str] = grammar::SOURCE_ROLE;
    const ALL: &'static [Self] = &[
        Self::Book,
        Self::NamedLoad,
        Self::BookSourced,
        Self::LoadDependency,
        Self::PlainInclusion,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Book => "book",
            Self::NamedLoad => "named-load",
            Self::BookSourced => "book-sourced",
            Self::LoadDependency => "load-dependency",
            Self::PlainInclusion => "plain-inclusion",
        }
    }
}

/// The closed intake answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedAdmissionOutcome {
    /// Host material was admitted.
    Admitted,
    /// A well-owned attempt produced no usable fact.
    NoObservation,
    /// Framing, bounds, attribution, or integrity failed.
    Refused,
}

impl ClosedToken for RecordedAdmissionOutcome {
    const TOKENS: &'static [&'static str] = grammar::ADMISSION_OUTCOME;
    const ALL: &'static [Self] = &[Self::Admitted, Self::NoObservation, Self::Refused];
    fn token(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::NoObservation => "no-observation",
            Self::Refused => "refused",
        }
    }
}

/// The per-site and per-region plan outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedDisposition {
    /// The authored bytes execute.
    Run,
    /// The authored bytes are replaced by a value-preserving stand-in.
    Replace,
    /// The site lies in a branch proved dead.
    Omit,
    /// A check is inserted ahead of the authored bytes, which survive verbatim.
    Guard,
}

impl ClosedToken for RecordedDisposition {
    const TOKENS: &'static [&'static str] = grammar::DISPOSITION;
    const ALL: &'static [Self] = &[Self::Run, Self::Replace, Self::Omit, Self::Guard];
    fn token(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Replace => "replace",
            Self::Omit => "omit",
            Self::Guard => "guard",
        }
    }
}

/// What the definition plane decided for one name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedLoadOutcome {
    /// The name bound to one definition.
    Bound,
    /// Two definitions contested the name.
    Contested,
    /// The binding could not be proved.
    Unprovable,
    /// Helpers under one name disagreed.
    HelperConflict,
}

impl ClosedToken for RecordedLoadOutcome {
    const TOKENS: &'static [&'static str] = grammar::LOAD_OUTCOME;
    const ALL: &'static [Self] = &[
        Self::Bound,
        Self::Contested,
        Self::Unprovable,
        Self::HelperConflict,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::Contested => "contested",
            Self::Unprovable => "unprovable",
            Self::HelperConflict => "helper-conflict",
        }
    }
}

/// One site analysis classification. The two source variants carrying a decision-bearing boolean
/// occupy two tokens each, because that boolean is what decides whether the row licenses anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSiteClass {
    /// Nothing licenses replacing this site.
    MustRun,
    /// An establish whose cell an ambient probe reads.
    EstablishProbeAmbient,
    /// An establish whose cell a written probe reads.
    EstablishProbeWritten,
    /// A query whose resolved value is still valid.
    QueryResolvableValid,
    /// A query whose resolved value is stale.
    QueryResolvableStale,
    /// A member population whose own member established its reach.
    EstablishMembersSelfReached,
    /// A member population something else reached.
    EstablishMembersReached,
    /// A call whose members are themselves classified.
    InlineCall,
}

impl ClosedToken for RecordedSiteClass {
    const TOKENS: &'static [&'static str] = grammar::SITE_CLASS;
    const ALL: &'static [Self] = &[
        Self::MustRun,
        Self::EstablishProbeAmbient,
        Self::EstablishProbeWritten,
        Self::QueryResolvableValid,
        Self::QueryResolvableStale,
        Self::EstablishMembersSelfReached,
        Self::EstablishMembersReached,
        Self::InlineCall,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::MustRun => "must-run",
            Self::EstablishProbeAmbient => "establish-probe-ambient",
            Self::EstablishProbeWritten => "establish-probe-written",
            Self::QueryResolvableValid => "query-resolvable-valid",
            Self::QueryResolvableStale => "query-resolvable-stale",
            Self::EstablishMembersSelfReached => "establish-members-self-reached",
            Self::EstablishMembersReached => "establish-members-reached",
            Self::InlineCall => "inline-call",
        }
    }
}

/// Which dataflow answer a certification row is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSolvePass {
    /// The whole analysis window.
    WholeWindow,
    /// Value flow.
    ValueFlow,
    /// The function environment.
    FunctionEnvironment,
    /// Reaching definitions.
    ReachingDefs,
    /// Self reach.
    SelfReach,
    /// Effective reach.
    EffectiveReach,
}

impl ClosedToken for RecordedSolvePass {
    const TOKENS: &'static [&'static str] = grammar::SOLVE_PASS;
    const ALL: &'static [Self] = &[
        Self::WholeWindow,
        Self::ValueFlow,
        Self::FunctionEnvironment,
        Self::ReachingDefs,
        Self::SelfReach,
        Self::EffectiveReach,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::WholeWindow => "whole-window",
            Self::ValueFlow => "value-flow",
            Self::FunctionEnvironment => "function-environment",
            Self::ReachingDefs => "reaching-defs",
            Self::SelfReach => "self-reach",
            Self::EffectiveReach => "effective-reach",
        }
    }
}

/// Which body a probe site shipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedShipLane {
    /// The site own verdict body.
    Verdict,
    /// A predict body.
    Predict,
    /// Nothing shippable resolved.
    Unresolvable,
}

impl ClosedToken for RecordedShipLane {
    const TOKENS: &'static [&'static str] = grammar::SHIP_LANE;
    const ALL: &'static [Self] = &[Self::Verdict, Self::Predict, Self::Unresolvable];
    fn token(self) -> &'static str {
        match self {
            Self::Verdict => "verdict",
            Self::Predict => "predict",
            Self::Unresolvable => "unresolvable",
        }
    }
}

/// What the survival walk decided. The four demotion causes stay distinct: two are claims about the
/// runbook mutators, one is a finding about resolver quality, and one about our own solver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSurvivalOutcome {
    /// No wall stood between the measurement and the site.
    Clean,
    /// One elision survived a running wall.
    SurvivedStandalone,
    /// Every member of an aggregate survived.
    SurvivedAggregate,
    /// An unfootprinted wall walled everything below it.
    DemotedTotalWall,
    /// A footprint intersected the backing.
    DemotedPoisoned,
    /// A resolver could not separate two names.
    DemotedMayAlias,
    /// The solve certifier tripped.
    DemotedSolveInconsistent,
    /// The reference model disagreed with the production answer.
    RederivationDisagreed,
}

impl ClosedToken for RecordedSurvivalOutcome {
    const TOKENS: &'static [&'static str] = grammar::SURVIVAL_OUTCOME;
    const ALL: &'static [Self] = &[
        Self::Clean,
        Self::SurvivedStandalone,
        Self::SurvivedAggregate,
        Self::DemotedTotalWall,
        Self::DemotedPoisoned,
        Self::DemotedMayAlias,
        Self::DemotedSolveInconsistent,
        Self::RederivationDisagreed,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::SurvivedStandalone => "survived-standalone",
            Self::SurvivedAggregate => "survived-aggregate",
            Self::DemotedTotalWall => "demoted-total-wall",
            Self::DemotedPoisoned => "demoted-poisoned",
            Self::DemotedMayAlias => "demoted-may-alias",
            Self::DemotedSolveInconsistent => "demoted-solve-inconsistent",
            Self::RederivationDisagreed => "rederivation-disagreed",
        }
    }
}

/// Which identity axis a render row is keyed by, read off its kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderSubjectAxis {
    /// The subject is a leaf, and a member may accompany it.
    Leaf,
    /// The subject is a region ordinal, in the `region-decision` space.
    Region,
    /// The row owns neither axis and both slots are absent.
    None,
}

impl RenderSubjectAxis {
    /// The word a report renders for this axis.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Leaf => "leaf",
            Self::Region => "region",
            Self::None => "none",
        }
    }
}

/// Which render-time decision a row records. The key axis rides the token, because a region owns no
/// execution and a row keyed by a contributing invocation would name the wrong thing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedRenderKind {
    /// Which body a guard invokes, and under what name.
    PinnedBinding,
    /// A leaf edit refused because the span carries a heredoc.
    RefusedHeredocSite,
    /// A leaf edit refused because the span carries a blocking redirect.
    RefusedBlockingRedirectSite,
    /// A region edit refused because the span carries a heredoc.
    RefusedHeredocRegion,
    /// A region edit refused because the span carries a blocking redirect.
    RefusedBlockingRedirectRegion,
    /// An omitted leaf whose controller was neutralised.
    OmitNeutralised,
    /// An omitted leaf whose controller was not neutralised, so the bytes render verbatim.
    OmitNotNeutralised,
    /// The artifact emits every name munged.
    DefensiveEmissionOn,
    /// The artifact emits idiomatic names.
    DefensiveEmissionOff,
    /// A record the certifier trip demoted.
    CertifierTripDemote,
    /// An import edit that repointed an operand.
    ImportRepointed,
    /// An import edit that inlined a source.
    ImportInlined,
}

impl RecordedRenderKind {
    /// Which identity axis this kind is keyed by.
    #[must_use]
    pub const fn subject_axis(self) -> RenderSubjectAxis {
        match self {
            Self::PinnedBinding
            | Self::RefusedHeredocSite
            | Self::RefusedBlockingRedirectSite
            | Self::OmitNeutralised
            | Self::OmitNotNeutralised
            | Self::CertifierTripDemote => RenderSubjectAxis::Leaf,
            Self::RefusedHeredocRegion | Self::RefusedBlockingRedirectRegion => {
                RenderSubjectAxis::Region
            }
            Self::DefensiveEmissionOn
            | Self::DefensiveEmissionOff
            | Self::ImportRepointed
            | Self::ImportInlined => RenderSubjectAxis::None,
        }
    }
}

impl ClosedToken for RecordedRenderKind {
    const TOKENS: &'static [&'static str] = grammar::RENDER_KIND;
    const ALL: &'static [Self] = &[
        Self::PinnedBinding,
        Self::RefusedHeredocSite,
        Self::RefusedBlockingRedirectSite,
        Self::RefusedHeredocRegion,
        Self::RefusedBlockingRedirectRegion,
        Self::OmitNeutralised,
        Self::OmitNotNeutralised,
        Self::DefensiveEmissionOn,
        Self::DefensiveEmissionOff,
        Self::CertifierTripDemote,
        Self::ImportRepointed,
        Self::ImportInlined,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::PinnedBinding => "pinned-binding",
            Self::RefusedHeredocSite => "refused-heredoc-site",
            Self::RefusedBlockingRedirectSite => "refused-blocking-redirect-site",
            Self::RefusedHeredocRegion => "refused-heredoc-region",
            Self::RefusedBlockingRedirectRegion => "refused-blocking-redirect-region",
            Self::OmitNeutralised => "omit-neutralised",
            Self::OmitNotNeutralised => "omit-not-neutralised",
            Self::DefensiveEmissionOn => "defensive-emission-on",
            Self::DefensiveEmissionOff => "defensive-emission-off",
            Self::CertifierTripDemote => "certifier-trip-demote",
            Self::ImportRepointed => "import-repointed",
            Self::ImportInlined => "import-inlined",
        }
    }
}

/// The speech act of a narrative row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSpeechAct {
    /// A probe read the world.
    Measured,
    /// A named author accepted a judgment.
    Vouched,
    /// A command executed.
    Ran,
    /// A named author asserted something no machine verified.
    Claimed,
    /// The engine computed a consequence.
    Derived,
    /// The operator typed a flag.
    Consented,
    /// An author declined to answer.
    Declined,
}

impl ClosedToken for RecordedSpeechAct {
    const TOKENS: &'static [&'static str] = grammar::SPEECH_ACT;
    const ALL: &'static [Self] = &[
        Self::Measured,
        Self::Vouched,
        Self::Ran,
        Self::Claimed,
        Self::Derived,
        Self::Consented,
        Self::Declined,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Vouched => "vouched",
            Self::Ran => "ran",
            Self::Claimed => "claimed",
            Self::Derived => "derived",
            Self::Consented => "consented",
            Self::Declined => "declined",
        }
    }
}

/// The collapse class of a narrative row. The reserved cancellation class is deliberately absent:
/// its source variant is unconstructable, and a token that cannot be written is a promise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedNarrativeKind {
    /// Two measurements of one cell disagreed.
    FactMergeDisagreement,
    /// A verdict body declined.
    VerdictDecline,
    /// An unmodeled command formed a wall.
    WallFormation,
    /// A substitution was refused.
    SubstitutionRefusal,
    /// Context entry was denied.
    EntryDenial,
    /// A wrapper pair disagreed about where its argument tail begins.
    WrapperPairIncoherent,
    /// Context entry failed.
    EntryFailure,
    /// A decision was demoted.
    Demotion,
    /// A render edit was refused.
    RenderRefusal,
    /// A fixpoint hit its round cap.
    FixpointCapDegrade,
    /// One role family shadowed another.
    RoleFamilyShadowed,
    /// The solver disagreed with its own certifier.
    SolverConsistencyFailure,
    /// A composition was suspended.
    CompositionSuspended,
    /// A projection declined to carry a population.
    ProjectionDrop,
}

impl ClosedToken for RecordedNarrativeKind {
    const TOKENS: &'static [&'static str] = grammar::NARRATIVE_KIND;
    const ALL: &'static [Self] = &[
        Self::FactMergeDisagreement,
        Self::VerdictDecline,
        Self::WallFormation,
        Self::SubstitutionRefusal,
        Self::EntryDenial,
        Self::WrapperPairIncoherent,
        Self::EntryFailure,
        Self::Demotion,
        Self::RenderRefusal,
        Self::FixpointCapDegrade,
        Self::RoleFamilyShadowed,
        Self::SolverConsistencyFailure,
        Self::CompositionSuspended,
        Self::ProjectionDrop,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::FactMergeDisagreement => "fact-merge-disagreement",
            Self::VerdictDecline => "verdict-decline",
            Self::WallFormation => "wall-formation",
            Self::SubstitutionRefusal => "substitution-refusal",
            Self::EntryDenial => "entry-denial",
            Self::WrapperPairIncoherent => "wrapper-pair-incoherent",
            Self::EntryFailure => "entry-failure",
            Self::Demotion => "demotion",
            Self::RenderRefusal => "render-refusal",
            Self::FixpointCapDegrade => "fixpoint-cap-degrade",
            Self::RoleFamilyShadowed => "role-family-shadowed",
            Self::SolverConsistencyFailure => "solver-consistency-failure",
            Self::CompositionSuspended => "composition-suspended",
            Self::ProjectionDrop => "projection-drop",
        }
    }
}

/// Which irreversible verb a licensor row attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedLicenseVerb {
    /// The site was replaced.
    Replace,
    /// A check was inserted ahead of the site.
    Guard,
}

impl ClosedToken for RecordedLicenseVerb {
    const TOKENS: &'static [&'static str] = grammar::LICENSE_VERB;
    const ALL: &'static [Self] = &[Self::Replace, Self::Guard];
    fn token(self) -> &'static str {
        match self {
            Self::Replace => "replace",
            Self::Guard => "guard",
        }
    }
}

/// Whose utterance a replacement license rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedLicenseCustody {
    /// One author vouched for it.
    Vouched,
    /// Several authors each vouched for their own member.
    VouchedSeverally,
    /// The site measured its own value.
    MeasuredSelf,
}

impl ClosedToken for RecordedLicenseCustody {
    const TOKENS: &'static [&'static str] = grammar::LICENSE_CUSTODY;
    const ALL: &'static [Self] = &[Self::Vouched, Self::VouchedSeverally, Self::MeasuredSelf];
    fn token(self) -> &'static str {
        match self {
            Self::Vouched => "vouched",
            Self::VouchedSeverally => "vouched-severally",
            Self::MeasuredSelf => "measured-self",
        }
    }
}

/// Every in-memory decision species a projection can decline to carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSpineSpecies {
    /// The producing invocation.
    Invocation,
    /// The admitted record stream.
    RecordStream,
    /// Per-site dispositions.
    Disposition,
    /// The decision digest.
    Digest,
    /// Definition-plane outcomes.
    LoadDecision,
    /// Site classifications.
    SiteClassification,
    /// Solve certifications.
    SolveCertification,
    /// Vouch attachments.
    Vouch,
    /// Probe shipments.
    ProbeShip,
    /// The intake outcome.
    Admission,
    /// Per-cell observations.
    Observation,
    /// Intermediate validity rounds.
    ValidityRound,
    /// Survival-tier outcomes.
    Survival,
    /// Render-time decisions.
    RenderDecision,
    /// Per-region decisions.
    RegionDecision,
    /// The run outcome.
    Outcome,
}

impl ClosedToken for RecordedSpineSpecies {
    const TOKENS: &'static [&'static str] = grammar::SPINE_SPECIES;
    const ALL: &'static [Self] = &[
        Self::Invocation,
        Self::RecordStream,
        Self::Disposition,
        Self::Digest,
        Self::LoadDecision,
        Self::SiteClassification,
        Self::SolveCertification,
        Self::Vouch,
        Self::ProbeShip,
        Self::Admission,
        Self::Observation,
        Self::ValidityRound,
        Self::Survival,
        Self::RenderDecision,
        Self::RegionDecision,
        Self::Outcome,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Invocation => "invocation",
            Self::RecordStream => "record-stream",
            Self::Disposition => "disposition",
            Self::Digest => "digest",
            Self::LoadDecision => "load-decision",
            Self::SiteClassification => "site-classification",
            Self::SolveCertification => "solve-certification",
            Self::Vouch => "vouch",
            Self::ProbeShip => "probe-ship",
            Self::Admission => "admission",
            Self::Observation => "observation",
            Self::ValidityRound => "validity-round",
            Self::Survival => "survival",
            Self::RenderDecision => "render-decision",
            Self::RegionDecision => "region-decision",
            Self::Outcome => "outcome",
        }
    }
}

/// Why a projection did not carry a population.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedOmissionReason {
    /// Nothing mints the species.
    Unminted,
    /// This version does not project it.
    NotProjectedV1,
    /// Its content is excluded from the durable by policy.
    ContentExcluded,
    /// A bound stopped it being carried.
    OverLimit,
}

impl ClosedToken for RecordedOmissionReason {
    const TOKENS: &'static [&'static str] = grammar::OMISSION_REASON;
    const ALL: &'static [Self] = &[
        Self::Unminted,
        Self::NotProjectedV1,
        Self::ContentExcluded,
        Self::OverLimit,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Unminted => "unminted",
            Self::NotProjectedV1 => "not-projected-v1",
            Self::ContentExcluded => "content-excluded",
            Self::OverLimit => "over-limit",
        }
    }
}

/// Whether an assignment knows which presented plans it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedOriginState {
    /// No assignment names an originating plan.
    Unavailable,
    /// At least one assignment names an originating plan.
    Known,
}

impl ClosedToken for RecordedOriginState {
    const TOKENS: &'static [&'static str] = grammar::ORIGIN_STATE;
    const ALL: &'static [Self] = &[Self::Unavailable, Self::Known];
    fn token(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Known => "known",
        }
    }
}

/// Which publication route authorized an apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedApplyPolicy {
    /// A rich intent was published before dispatch.
    RequiredRich,
    /// An explicit configured bypass permitted dispatch.
    ConfiguredBypass,
}

impl ClosedToken for RecordedApplyPolicy {
    const TOKENS: &'static [&'static str] = grammar::APPLY_POLICY;
    const ALL: &'static [Self] = &[Self::RequiredRich, Self::ConfiguredBypass];
    fn token(self) -> &'static str {
        match self {
            Self::RequiredRich => "required-rich",
            Self::ConfiguredBypass => "configured-bypass",
        }
    }
}

/// The graceful terminal state an apply reached. A session that produced no completion marker is
/// unknown, never not-attempted: absence of output cannot prove absence of execution, and only a
/// spawn that never happened may claim nothing ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedTerminalState {
    /// Every site reached its end and the marker arrived.
    Complete,
    /// The artifact ran and a command failed.
    CommandFailed,
    /// A process ran and the marker never arrived.
    Unknown,
    /// No process was ever created.
    NotAttempted,
    /// Transport integrity was lost.
    TransportFailed,
    /// Mutation integrity was lost and the apply withheld further mutation.
    MutationIntegrityAborted,
    /// The apply was cancelled.
    Cancelled,
}

impl ClosedToken for RecordedTerminalState {
    const TOKENS: &'static [&'static str] = grammar::TERMINAL_STATE;
    const ALL: &'static [Self] = &[
        Self::Complete,
        Self::CommandFailed,
        Self::Unknown,
        Self::NotAttempted,
        Self::TransportFailed,
        Self::MutationIntegrityAborted,
        Self::Cancelled,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::CommandFailed => "command-failed",
            Self::Unknown => "unknown",
            Self::NotAttempted => "not-attempted",
            Self::TransportFailed => "transport-failed",
            Self::MutationIntegrityAborted => "mutation-integrity-aborted",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Whether the terminal report itself reached durable storage. Narration only, and never a
/// statement about execution integrity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedDurableState {
    /// The report was published.
    Published,
    /// Publication was attempted and failed.
    Failed,
    /// Publication was never attempted.
    NotAttempted,
}

impl ClosedToken for RecordedDurableState {
    const TOKENS: &'static [&'static str] = grammar::DURABLE_STATE;
    const ALL: &'static [Self] = &[Self::Published, Self::Failed, Self::NotAttempted];
    fn token(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Failed => "failed",
            Self::NotAttempted => "not-attempted",
        }
    }
}

/// What one site did during an apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSiteStatus {
    /// The authored bytes executed.
    Ran,
    /// The inserted check passed and the bytes were skipped.
    GuardPassed,
    /// The inserted check failed and the bytes executed.
    GuardFellThrough,
    /// A stand-in executed in place of the bytes.
    Replaced,
    /// The site was absent from the artifact.
    Omitted,
    /// Execution stopped before reaching the site.
    NotReached,
    /// What the site did is not known.
    Unknown,
}

impl ClosedToken for RecordedSiteStatus {
    const TOKENS: &'static [&'static str] = grammar::SITE_STATUS;
    const ALL: &'static [Self] = &[
        Self::Ran,
        Self::GuardPassed,
        Self::GuardFellThrough,
        Self::Replaced,
        Self::Omitted,
        Self::NotReached,
        Self::Unknown,
    ];
    fn token(self) -> &'static str {
        match self {
            Self::Ran => "ran",
            Self::GuardPassed => "guard-passed",
            Self::GuardFellThrough => "guard-fell-through",
            Self::Replaced => "replaced",
            Self::Omitted => "omitted",
            Self::NotReached => "not-reached",
            Self::Unknown => "unknown",
        }
    }
}

/// The tokens for a signer provenance. Not a grammar field: a document never spells its own
/// provenance, which is a property of the material the reader resolved.
pub const SIGNER_TRUST: &[&str] = &["trusted", "self-asserted"];

/// Where the verification material for a read document came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedSignerTrust {
    /// Controller policy named the provider.
    Trusted,
    /// Controller policy did not name the provider.
    SelfAsserted,
}

impl ClosedToken for RecordedSignerTrust {
    const TOKENS: &'static [&'static str] = SIGNER_TRUST;
    const ALL: &'static [Self] = &[Self::Trusted, Self::SelfAsserted];
    fn token(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::SelfAsserted => "self-asserted",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{SelfAssertedReceiptSigner, SignerTrust, TrustedReceiptSigner};

    /// Both directions, for one vocabulary: every variant spells a token the grammar declares, and
    /// every token the grammar declares is spelled by some variant.
    fn census<T: ClosedToken + core::fmt::Debug>(name: &str) {
        assert_eq!(
            T::ALL.len(),
            T::TOKENS.len(),
            "{name}: {} variants against {} tokens",
            T::ALL.len(),
            T::TOKENS.len()
        );
        for (variant, token) in T::ALL.iter().zip(T::TOKENS.iter()) {
            assert_eq!(
                variant.token(),
                *token,
                "{name}: {variant:?} is out of order or misspelled"
            );
            assert_eq!(T::of_token(token), Some(*variant), "{name}: {token}");
        }
        for token in T::TOKENS {
            assert!(
                T::ALL.iter().any(|v| v.token() == *token),
                "{name}: no variant spells {token}"
            );
        }
        assert_eq!(T::of_token("not-a-token"), None, "{name}");
        assert_eq!(T::of_token(""), None, "{name}");
    }

    #[test]
    fn every_vocabulary_projects_its_token_set_exhaustively_both_ways() {
        // The whole point of the table: a source variant that gained a token, or a token that lost
        // its variant, is a projection that silently drops or invents a state.
        census::<OpaqueState>("opaque-state");
        census::<ImageState>("image-state");
        census::<RecordedMode>("mode");
        census::<RecordedSourceRole>("source-role");
        census::<RecordedAdmissionOutcome>("admission-outcome");
        census::<RecordedDisposition>("disposition");
        census::<RecordedLoadOutcome>("load-outcome");
        census::<RecordedSiteClass>("site-class");
        census::<RecordedSolvePass>("solve-pass");
        census::<RecordedShipLane>("ship-lane");
        census::<RecordedSurvivalOutcome>("survival-outcome");
        census::<RecordedRenderKind>("render-kind");
        census::<RecordedSpeechAct>("speech-act");
        census::<RecordedNarrativeKind>("narrative-kind");
        census::<RecordedLicenseVerb>("license-verb");
        census::<RecordedLicenseCustody>("license-custody");
        census::<RecordedSpineSpecies>("spine-species");
        census::<RecordedOmissionReason>("omission-reason");
        census::<RecordedOriginState>("origin-state");
        census::<RecordedApplyPolicy>("apply-policy");
        census::<RecordedTerminalState>("terminal-state");
        census::<RecordedDurableState>("durable-state");
        census::<RecordedSiteStatus>("site-status");
        census::<RecordedSignerTrust>("signer-trust");
    }

    #[test]
    fn the_signer_trust_tokens_are_the_ones_the_marker_types_spell() {
        // Two spellings of one vocabulary would let a report and a document disagree about a
        // provenance neither of them can re-derive.
        assert_eq!(
            RecordedSignerTrust::Trusted.token(),
            TrustedReceiptSigner::TOKEN
        );
        assert_eq!(
            RecordedSignerTrust::SelfAsserted.token(),
            SelfAssertedReceiptSigner::TOKEN
        );
    }

    #[test]
    fn a_render_kind_names_exactly_one_identity_axis() {
        // The axis is a function of the token, which is what lets one polymorphic subject slot
        // carry a leaf, a region ordinal, or nothing without a reader having to guess which.
        let leaf = [
            RecordedRenderKind::PinnedBinding,
            RecordedRenderKind::RefusedHeredocSite,
            RecordedRenderKind::RefusedBlockingRedirectSite,
            RecordedRenderKind::OmitNeutralised,
            RecordedRenderKind::OmitNotNeutralised,
            RecordedRenderKind::CertifierTripDemote,
        ];
        for kind in leaf {
            assert_eq!(kind.subject_axis(), RenderSubjectAxis::Leaf, "{kind:?}");
        }
        for kind in [
            RecordedRenderKind::RefusedHeredocRegion,
            RecordedRenderKind::RefusedBlockingRedirectRegion,
        ] {
            assert_eq!(kind.subject_axis(), RenderSubjectAxis::Region, "{kind:?}");
        }
        for kind in [
            RecordedRenderKind::DefensiveEmissionOn,
            RecordedRenderKind::DefensiveEmissionOff,
            RecordedRenderKind::ImportRepointed,
            RecordedRenderKind::ImportInlined,
        ] {
            assert_eq!(kind.subject_axis(), RenderSubjectAxis::None, "{kind:?}");
        }
    }

    #[test]
    fn the_boolean_spelling_is_exact() {
        assert_eq!(bool_token(true), "yes");
        assert_eq!(bool_token(false), "no");
        assert_eq!(bool_of_token("yes"), Some(true));
        assert_eq!(bool_of_token("no"), Some(false));
        assert_eq!(bool_of_token("true"), None);
        assert_eq!(bool_of_token("1"), None);
        assert_eq!(bool_of_token("Yes"), None, "case is exact");
    }
}
