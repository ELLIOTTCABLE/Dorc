//! Which persisted family is projected, and which is not — the read surface made EXHAUSTIVE.
//!
//! `RecordedWhyFacts` began as a projection of three families out of the fifteen a plan document
//! persists, and a consumer had no way to tell a family that is genuinely absent from one nobody
//! had projected yet. That is the gap this closes: every persisted family is either a typed facts
//! collection on the model or explicitly classified, and the classification is CLOSED, so a
//! sixteenth family cannot land unclassified.
//!
//! # What this is not
//!
//! Not a widening of what the durable persists, nor of the grammar, the writer, the wire, the
//! projection states, or the providers. Nothing new is recorded and nothing new is read from a
//! document — this is read-surface projection over material the reader ALREADY validated. Every
//! standing law binds unchanged: closed recorded tokens stay typed
//! (`inv-identities-never-cross-domains`), arbitrary values leave only through the class-aware
//! encoder (`inv-report-is-the-public-read-boundary`), and nothing here converts to a live claim
//! (`inv-recorded-values-stay-recorded`). No raw model accessor and no overlay accessor is exposed.

use crate::plan::RenderSubject;
use crate::reingested::RecordedInfluence;
use crate::rows::{RecordedAst, RecordedOperands, RecordedSite};
use crate::tokens::{
    RecordedAdmissionOutcome, RecordedDisposition, RecordedInvocationMode, RecordedLicenseCustody,
    RecordedLicenseVerb, RecordedLoadOutcome, RecordedNarrativeKind, RecordedRenderKind,
    RecordedShipLane, RecordedSiteClass, RecordedSolvePass, RecordedSpeechAct,
    RecordedSurvivalOutcome,
};

use super::states::MaterialState;
use super::value::RecordedValue;

/// One family of rows a plan document persists.
///
/// CLOSED and exhaustive over the recorded plan model. The point of naming them here is that
/// [`super::RecordedWhyFacts::coverage`] must answer for every one, so a family reaching the
/// durable without reaching this list is a compile error rather than a silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlanFamily {
    /// The invocation singleton.
    Invocation,
    /// The acquired-source table.
    Sources,
    /// The records-admission singleton.
    Admission,
    /// The presented-plan singleton.
    PresentedPlan,
    /// Per-site decisions.
    Sites,
    /// Per-region decisions.
    Regions,
    /// Per-load decisions.
    Loads,
    /// Per-site classifications.
    Classifications,
    /// Solve certifications.
    Certifications,
    /// Probe ships.
    Ships,
    /// Survivals.
    Survivals,
    /// Render decisions.
    Renders,
    /// Decision-inert narratives.
    Narratives,
    /// Licensors.
    Licensors,
    /// Projection omissions.
    Omissions,
}

impl PlanFamily {
    /// Every family, in the recorded model's own field order.
    pub const ALL: &'static [Self] = &[
        Self::Invocation,
        Self::Sources,
        Self::Admission,
        Self::PresentedPlan,
        Self::Sites,
        Self::Regions,
        Self::Loads,
        Self::Classifications,
        Self::Certifications,
        Self::Ships,
        Self::Survivals,
        Self::Renders,
        Self::Narratives,
        Self::Licensors,
        Self::Omissions,
    ];

    /// The word a report names it by.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Invocation => "invocation",
            Self::Sources => "sources",
            Self::Admission => "admission",
            Self::PresentedPlan => "presented-plan",
            Self::Sites => "sites",
            Self::Regions => "regions",
            Self::Loads => "loads",
            Self::Classifications => "classifications",
            Self::Certifications => "certifications",
            Self::Ships => "ships",
            Self::Survivals => "survivals",
            Self::Renders => "renders",
            Self::Narratives => "narratives",
            Self::Licensors => "licensors",
            Self::Omissions => "omissions",
        }
    }
}

/// What this model can say about one family.
///
/// FOUR states, and they are different facts: a family with a typed projection, one the document
/// carries that nobody has projected yet, one this document genuinely does not carry, and one that
/// does not apply to the question. Merging any pair would let a reader infer a durable gap from a
/// read-surface gap, which are repaired in completely different places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyCoverage {
    /// A typed facts collection carries it, with this many members.
    Projected(usize),
    /// The document persists it and this read surface does not project it yet. NOT a durable
    /// question: closing it is projection work in this module.
    RecordedButUnprojected,
    /// This document carries no rows of the family.
    NotCarried,
    /// The family does not apply to the rooted question.
    NotRelevant,
}

impl FamilyCoverage {
    /// A projected family whose collection may legitimately be empty.
    pub(crate) const fn of(members: usize) -> Self {
        Self::Projected(members)
    }

    /// A projected SINGLETON family, which is either present or genuinely not in the document.
    ///
    /// The distinction the four states exist for: an absent singleton is a row the run never wrote,
    /// repaired by nothing, and calling it `Projected(0)` would read as a projection that found an
    /// empty one.
    pub(crate) const fn of_singleton(present: bool) -> Self {
        if present {
            Self::of(1)
        } else {
            Self::NotCarried
        }
    }

    /// Whether a consumer can read typed facts for this family today.
    #[must_use]
    pub const fn is_projected(self) -> bool {
        matches!(self, Self::Projected(_))
    }
}

/// The invocation singleton, as the document recorded it.
#[derive(Debug, Clone)]
pub struct InvocationFacts {
    pub(crate) mode: RecordedInvocationMode,
    pub(crate) started: Option<u64>,
    pub(crate) attempt: u32,
    pub(crate) argv: MaterialState,
    pub(crate) target: MaterialState,
    pub(crate) target_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl InvocationFacts {
    /// What the run was doing.
    #[must_use]
    pub const fn mode(&self) -> RecordedInvocationMode {
        self.mode
    }

    /// The controller's own start reading, where one was taken. Controller-minted: a managed host
    /// never contributes an instant.
    #[must_use]
    pub const fn started(&self) -> Option<u64> {
        self.started
    }

    /// Which attempt of its target this was.
    #[must_use]
    pub const fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Whether the argument vector is in the document.
    #[must_use]
    pub const fn argv(&self) -> MaterialState {
        self.argv
    }

    /// Whether the host destination is in the document.
    #[must_use]
    pub const fn target(&self) -> MaterialState {
        self.target
    }

    /// That destination, where it is. Encoder-mediated like every other recorded value.
    #[must_use]
    pub const fn target_text(&self) -> Option<&RecordedValue> {
        self.target_text.as_ref()
    }

    /// Where the run stood relative to host contact.
    ///
    /// Read straight off the recorded grade, whose own seat reads an absent or unrecognised token
    /// as the MOST-influenced grade. Never re-derived here, and never rounded downward.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One decision-inert narrative the run minted.
///
/// The family that carries the recorded SPEECH ACT, which is why projecting it matters out of
/// proportion to its size: without it a reconstruction can say what the engine decided and not in
/// what act anybody spoke. The row identifies no site — narrative operands are not durable — so a
/// reader learns that N collapses of a class occurred and never which line each was about, and
/// this projection must not suggest otherwise.
#[derive(Debug, Clone, Copy)]
pub struct NarrativeFacts {
    pub(crate) ordinal: u32,
    pub(crate) speech: RecordedSpeechAct,
    pub(crate) kind: RecordedNarrativeKind,
    pub(crate) operands: RecordedOperands,
    pub(crate) influence: RecordedInfluence,
}

impl NarrativeFacts {
    /// Where this narrative sat in mint order.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// The typed speech act.
    #[must_use]
    pub const fn speech(&self) -> RecordedSpeechAct {
        self.speech
    }

    /// Which collapse class narrowed.
    #[must_use]
    pub const fn kind(&self) -> RecordedNarrativeKind {
        self.kind
    }

    /// How many operands were kept, and how many the cap dropped.
    #[must_use]
    pub const fn operands(&self) -> RecordedOperands {
        self.operands
    }

    /// Where the collapse stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// The records-admission singleton: what the intake edge answered.
///
/// `rul-admission-is-a-closed-outcome` says the three outcomes are not interchangeable, so the
/// projection carries the recorded token rather than a boolean anybody could round.
#[derive(Debug, Clone)]
pub struct AdmissionFacts {
    pub(crate) outcome: RecordedAdmissionOutcome,
    pub(crate) records: u64,
    pub(crate) bytes: u64,
    pub(crate) stream: MaterialState,
    pub(crate) stream_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl AdmissionFacts {
    /// What intake answered.
    #[must_use]
    pub const fn outcome(&self) -> RecordedAdmissionOutcome {
        self.outcome
    }

    /// How many records were admitted.
    #[must_use]
    pub const fn records(&self) -> u64 {
        self.records
    }

    /// How many bytes they accounted for.
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Whether the accounted record stream is in the document.
    #[must_use]
    pub const fn stream(&self) -> MaterialState {
        self.stream
    }

    /// That stream, where it is. Encoder-mediated: these are bytes a managed HOST produced.
    #[must_use]
    pub const fn stream_text(&self) -> Option<&RecordedValue> {
        self.stream_text.as_ref()
    }

    /// Where the intake stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// The presented-plan singleton: the identities of one complete approval surface.
///
/// Three identities rather than one, because `sinv-decision-identity` keeps the planner input, the
/// approval surface, and the planned image distinct; folding them would let a report say a plan was
/// approved when only its inputs matched.
#[derive(Debug, Clone)]
pub struct PresentedPlanFacts {
    pub(crate) planning_input: String,
    pub(crate) presented_plan: String,
    pub(crate) planned_image: Option<String>,
    pub(crate) influence: RecordedInfluence,
}

impl PresentedPlanFacts {
    /// The identity of the complete planner input tuple, as spelled.
    #[must_use]
    pub fn planning_input(&self) -> &str {
        &self.planning_input
    }

    /// The identity of the approval surface, as spelled.
    #[must_use]
    pub fn presented_plan(&self) -> &str {
        &self.presented_plan
    }

    /// The identity of the planned apply image, where the run had one.
    #[must_use]
    pub fn planned_image(&self) -> Option<&str> {
        self.planned_image.as_deref()
    }

    /// Where the presentation stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One authored region's shared outcome.
///
/// Region-keyed and never leaf-keyed (`30L:rul-two-identities-never-conflated`): a region is ONE
/// authored edit that many executions share, so it carries no site locator and this projection
/// must not imply one.
#[derive(Debug, Clone)]
pub struct RegionFacts {
    pub(crate) region: u32,
    pub(crate) ast: RecordedAst,
    pub(crate) disposition: RecordedDisposition,
    pub(crate) routes: u64,
    pub(crate) shell: MaterialState,
    pub(crate) shell_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl RegionFacts {
    /// Which region this decided, as the document numbers them.
    #[must_use]
    pub const fn region(&self) -> u32 {
        self.region
    }

    /// Which syntax node the region body came from.
    #[must_use]
    pub const fn ast(&self) -> RecordedAst {
        self.ast
    }

    /// What the plan does with every invocation of the region.
    #[must_use]
    pub const fn disposition(&self) -> RecordedDisposition {
        self.disposition
    }

    /// How many routes reach it.
    #[must_use]
    pub const fn routes(&self) -> u64 {
        self.routes
    }

    /// Whether the region's own shell text is in the document.
    #[must_use]
    pub const fn shell(&self) -> MaterialState {
        self.shell
    }

    /// That shell text, where it is. Encoder-mediated.
    #[must_use]
    pub const fn shell_text(&self) -> Option<&RecordedValue> {
        self.shell_text.as_ref()
    }

    /// Where the decision stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One definition-plane outcome — what a load did to the function environment.
#[derive(Debug, Clone)]
pub struct LoadFacts {
    pub(crate) ordinal: u32,
    pub(crate) outcome: RecordedLoadOutcome,
    pub(crate) name: MaterialState,
    pub(crate) name_text: Option<RecordedValue>,
    pub(crate) custody: MaterialState,
    pub(crate) custody_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl LoadFacts {
    /// Where this decision sat in decision order.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// What the definition plane decided.
    #[must_use]
    pub const fn outcome(&self) -> RecordedLoadOutcome {
        self.outcome
    }

    /// Whether the loaded name is in the document.
    #[must_use]
    pub const fn name(&self) -> MaterialState {
        self.name
    }

    /// That name, where it is. Encoder-mediated — it is a path off somebody's filesystem.
    #[must_use]
    pub const fn name_text(&self) -> Option<&RecordedValue> {
        self.name_text.as_ref()
    }

    /// Whether the custody description is in the document.
    #[must_use]
    pub const fn custody(&self) -> MaterialState {
        self.custody
    }

    /// That description, where it is. Encoder-mediated.
    #[must_use]
    pub const fn custody_text(&self) -> Option<&RecordedValue> {
        self.custody_text.as_ref()
    }

    /// Where the decision stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One site's analysis classification — what the engine took the site to BE.
#[derive(Debug, Clone, Copy)]
pub struct ClassificationFacts {
    pub(crate) site: RecordedSite,
    pub(crate) ast: RecordedAst,
    pub(crate) class: RecordedSiteClass,
    pub(crate) verdict_lane: bool,
    pub(crate) invalidator: bool,
    pub(crate) cells: RecordedOperands,
    pub(crate) influence: RecordedInfluence,
}

impl ClassificationFacts {
    /// Which site this classified.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which syntax node it came from.
    #[must_use]
    pub const fn ast(&self) -> RecordedAst {
        self.ast
    }

    /// The classification.
    #[must_use]
    pub const fn class(&self) -> RecordedSiteClass {
        self.class
    }

    /// Whether the site is on the verdict lane.
    #[must_use]
    pub const fn verdict_lane(&self) -> bool {
        self.verdict_lane
    }

    /// Whether the site invalidates anything.
    #[must_use]
    pub const fn invalidator(&self) -> bool {
        self.invalidator
    }

    /// How many cells the classification keys on, and how many the cap dropped.
    #[must_use]
    pub const fn cells(&self) -> RecordedOperands {
        self.cells
    }

    /// Where the classification stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One dataflow certification answer — the solver's own second opinion about itself.
#[derive(Debug, Clone, Copy)]
pub struct CertificationFacts {
    pub(crate) pass: RecordedSolvePass,
    pub(crate) consistent: bool,
    pub(crate) tripped: bool,
    pub(crate) influence: RecordedInfluence,
}

impl CertificationFacts {
    /// Which dataflow answer this certified.
    #[must_use]
    pub const fn pass(&self) -> RecordedSolvePass {
        self.pass
    }

    /// Whether the certifier agreed with the solver.
    #[must_use]
    pub const fn consistent(&self) -> bool {
        self.consistent
    }

    /// Whether the latch tripped.
    #[must_use]
    pub const fn tripped(&self) -> bool {
        self.tripped
    }

    /// Where the certification stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One probe shipment — which authored body a site sent to the host.
///
/// `rul-only-oracle-bytes-ship`: the lane a ship names is the whole answer to whose bytes travelled,
/// which is why the token is projected rather than reduced to a count.
#[derive(Debug, Clone)]
pub struct ShipFacts {
    pub(crate) site: RecordedSite,
    pub(crate) lane: RecordedShipLane,
    pub(crate) source: MaterialState,
    pub(crate) source_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl ShipFacts {
    /// Which site shipped.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which body it shipped.
    #[must_use]
    pub const fn lane(&self) -> RecordedShipLane {
        self.lane
    }

    /// Whether the defining source text is in the document.
    #[must_use]
    pub const fn source(&self) -> MaterialState {
        self.source
    }

    /// That text, where it is. Encoder-mediated.
    #[must_use]
    pub const fn source_text(&self) -> Option<&RecordedValue> {
        self.source_text.as_ref()
    }

    /// Where the shipment stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One survival-tier outcome — whether a fact reached a site, and what stopped it.
#[derive(Debug, Clone)]
pub struct SurvivalFacts {
    pub(crate) site: RecordedSite,
    pub(crate) outcome: RecordedSurvivalOutcome,
    pub(crate) wall: Option<u32>,
    pub(crate) aggregate: Option<u32>,
    pub(crate) poison: MaterialState,
    pub(crate) poison_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl SurvivalFacts {
    /// Which site the walk decided.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// What it decided.
    #[must_use]
    pub const fn outcome(&self) -> RecordedSurvivalOutcome {
        self.outcome
    }

    /// The leaf of the wall that stood, where one did.
    #[must_use]
    pub const fn wall(&self) -> Option<u32> {
        self.wall
    }

    /// How many establishes an aggregate carried, where the outcome names one.
    #[must_use]
    pub const fn aggregate(&self) -> Option<u32> {
        self.aggregate
    }

    /// Whether the poisoning kind is in the document.
    #[must_use]
    pub const fn poison(&self) -> MaterialState {
        self.poison
    }

    /// That kind, where it is. Encoder-mediated — an interned coordinate is never displayed raw.
    #[must_use]
    pub const fn poison_text(&self) -> Option<&RecordedValue> {
        self.poison_text.as_ref()
    }

    /// Where the walk stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One render-time decision — an edit Dorc made to the artifact it generated.
#[derive(Debug, Clone)]
pub struct RenderFacts {
    pub(crate) subject: RenderSubject,
    pub(crate) kind: RecordedRenderKind,
    pub(crate) detail: MaterialState,
    pub(crate) detail_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl RenderFacts {
    /// Which identity the row is keyed by — an execution, a region, or neither.
    ///
    /// One value rather than two nullable slots, because the axis is a function of the kind: a
    /// region-keyed row carrying a member is unrepresentable rather than merely refused.
    #[must_use]
    pub const fn subject(&self) -> RenderSubject {
        self.subject
    }

    /// Which decision the row records.
    #[must_use]
    pub const fn kind(&self) -> RecordedRenderKind {
        self.kind
    }

    /// Whether the decision's own detail is in the document.
    #[must_use]
    pub const fn detail(&self) -> MaterialState {
        self.detail
    }

    /// That detail, where it is. Encoder-mediated.
    #[must_use]
    pub const fn detail_text(&self) -> Option<&RecordedValue> {
        self.detail_text.as_ref()
    }

    /// Where the decision stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// What licensed one irreversible verb.
///
/// The family the wrongness-concentrates doctrine cares about most (`30V` §4): custody names whose
/// utterance a license rested on, and the locus is where a remedy would land.
#[derive(Debug, Clone)]
pub struct LicensorFacts {
    pub(crate) site: RecordedSite,
    pub(crate) license: RecordedLicenseVerb,
    pub(crate) custody: RecordedLicenseCustody,
    pub(crate) locus: MaterialState,
    pub(crate) locus_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl LicensorFacts {
    /// Which site the verb applied to.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// Which verb was licensed.
    #[must_use]
    pub const fn license(&self) -> RecordedLicenseVerb {
        self.license
    }

    /// Whose utterance it rests on.
    #[must_use]
    pub const fn custody(&self) -> RecordedLicenseCustody {
        self.custody
    }

    /// Whether the authoring locus is in the document.
    #[must_use]
    pub const fn locus(&self) -> MaterialState {
        self.locus
    }

    /// That locus, where it is. Encoder-mediated.
    #[must_use]
    pub const fn locus_text(&self) -> Option<&RecordedValue> {
        self.locus_text.as_ref()
    }

    /// Where the license stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}
