//! `RecordedWhyFacts` — the one inert model a receipt-rooted `why` question produces.
//!
//! ```text
//! selected root + causal receipt closure
//!   + recorded site decisions
//!   + exact recorded general-sh source
//!   + durable locators
//!   + current-source observations supplied by the edge
//!   -> RecordedWhyFacts
//! ```
//!
//! # What this is not
//!
//! Not a renderer: nothing here produces a user-facing sentence, and the only way a byte leaves is
//! through a [`ValueEncoder`] the CALLER supplies. Not a kernel input: no arm converts to a
//! `Disposition`, a licence, a `PlanAuthority`, or anything an apply could consume, and there is no
//! constructor that would let one in. It is a HANDOFF — a later conductor joins it to the aid plane
//! and never enters receipt internals.
//!
//! # Why it lives in the receipt crate
//!
//! Everything it reads is receipt-owned: sealed `Reingested` values, the durable locator, the
//! recorded source table, the graph. Homing it in `cli` or `aid` would put the decomposition of
//! recorded material behind a crate that also knows how to render, and the seal would then be one
//! refactor from leaking. This crate depends on nothing but its own pure types and `std`.
//!
//! # Independence
//!
//! Authentication, closure completeness, influence, source comparison and re-derivation
//! availability are five separate typed answers and stay that way. A document can be
//! authenticated and incomplete, or complete and unopenable; folding any pair would let a reader
//! infer a fact from a value that never stated it.

mod address;
mod families;
mod states;
mod value;

pub use address::{AddressFacts, AddressResolution, RequestedAddress, UnresolvedReason};
pub use families::{
    AdmissionFacts, CertificationFacts, ClassificationFacts, FamilyCoverage, InvocationFacts,
    LicensorFacts, LoadFacts, NarrativeFacts, PlanFamily, PresentedPlanFacts, RegionFacts,
    RenderFacts, ShipFacts, SurvivalFacts,
};
pub use states::{
    AuthenticationState, ClosureCompleteness, CurrentSourceState, DetailState, MaterialState,
    ProjectionState, ReDerivationState, RecordedDocumentId, RecordedSpecies, SiblingState,
};
pub use value::{ByteAgreement, RecordedValue, ValueClass, ValueEncoder};

use crate::durable_locator::RecordedStageKind;
use crate::graph::ReachedClosure;
use crate::order::ReceiptOrderToken;
use crate::reingested::RecordedInfluence;
use crate::rows::{RecordedAst, RecordedSite};
use crate::tokens::{RecordedDisposition, RecordedSourceClass, RecordedSpineSpecies};

/// The selected root, as the question found it.
#[derive(Debug, Clone)]
pub struct RootFacts {
    document: RecordedDocumentId,
    order: ReceiptOrderToken,
    authentication: AuthenticationState,
    projection: ProjectionState,
    detail: DetailState,
}

impl RootFacts {
    /// Bind one root's identity and standing.
    #[must_use]
    pub const fn of(
        document: RecordedDocumentId,
        order: ReceiptOrderToken,
        authentication: AuthenticationState,
        projection: ProjectionState,
        detail: DetailState,
    ) -> Self {
        Self {
            document,
            order,
            authentication,
            projection,
            detail,
        }
    }

    /// Which document the question is rooted at.
    #[must_use]
    pub const fn document(&self) -> &RecordedDocumentId {
        &self.document
    }

    /// Its species.
    #[must_use]
    pub const fn species(&self) -> RecordedSpecies {
        self.document.species()
    }

    /// The store order it was filed under.
    ///
    /// The token the store already holds, never a spelling re-parsed here: a `String` on this seat
    /// was substitutable with any other text and could reach a render as one
    /// (`30Rh:open-report-api-close-residue`).
    #[must_use]
    pub const fn order(&self) -> ReceiptOrderToken {
        self.order
    }

    /// What outer verification said.
    #[must_use]
    pub const fn authentication(&self) -> AuthenticationState {
        self.authentication
    }

    /// Which projection it is.
    #[must_use]
    pub const fn projection(&self) -> ProjectionState {
        self.projection
    }

    /// Whether its grouped detail region opened.
    #[must_use]
    pub const fn detail(&self) -> DetailState {
        self.detail
    }
}

/// The causal closure the rooted question needed.
///
/// QUESTION-DIRECTED, never the whole undirected component: an outcome reaches its intent and that
/// intent's originating plans, and selecting one plan does not pull every later apply attempt that
/// happens to share a component. A disconnected DAG contributes nothing at all.
#[derive(Debug, Clone)]
pub struct ClosureFacts {
    reached: ReachedClosure,
    completeness: ClosureCompleteness,
    siblings: Vec<SiblingState>,
}

impl ClosureFacts {
    /// Bind what the closure reached and what it could not.
    ///
    /// Membership arrives as a [`ReachedClosure`], whose one mint is the graph's own walk, so a
    /// caller cannot name a document the graph never reached. Completeness is DERIVED from the
    /// sibling states rather than passed: a caller that could declare a closure complete while
    /// naming a missing sibling would be able to say two things at once, and the sibling list is
    /// the one that carries evidence.
    #[must_use]
    pub fn of(reached: ReachedClosure, siblings: Vec<SiblingState>) -> Self {
        let completeness = if siblings.is_empty() {
            ClosureCompleteness::Complete
        } else {
            ClosureCompleteness::Partial
        };
        Self {
            reached,
            completeness,
            siblings,
        }
    }

    /// Every document the rooted question reached, root included.
    #[must_use]
    pub fn reached(&self) -> &[RecordedDocumentId] {
        self.reached.documents()
    }

    /// Whether the closure was assembled whole.
    #[must_use]
    pub const fn completeness(&self) -> ClosureCompleteness {
        self.completeness
    }

    /// What is wrong with each sibling that is not in hand.
    #[must_use]
    pub fn siblings(&self) -> &[SiblingState] {
        &self.siblings
    }
}

/// One acquired source, as the document recorded it.
#[derive(Debug, Clone)]
pub struct SourceFacts {
    ordinal: u32,
    class: RecordedSourceClass,
    digest: String,
    bytes: u64,
    content: MaterialState,
    path: MaterialState,
    current: CurrentSourceState,
    text: Option<RecordedValue>,
}

impl SourceFacts {
    /// Where the source sat in the acquired-source table.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// Which dialect the run accepted it as — the boundary that decided byte custody.
    #[must_use]
    pub const fn class(&self) -> RecordedSourceClass {
        self.class
    }

    /// Its content digest at load time, as spelled.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }

    /// How many bytes the run acquired.
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Whether its exact bytes are in the document.
    #[must_use]
    pub const fn content(&self) -> MaterialState {
        self.content
    }

    /// Whether its path is in the document.
    #[must_use]
    pub const fn path(&self) -> MaterialState {
        self.path
    }

    /// How the current tree stands against it.
    #[must_use]
    pub const fn current(&self) -> CurrentSourceState {
        self.current
    }

    /// Its exact recorded bytes, where the document carries them. Encoder-mediated.
    #[must_use]
    pub const fn text(&self) -> Option<&RecordedValue> {
        self.text.as_ref()
    }
}

/// One stage of a site's recorded provenance.
#[derive(Debug, Clone)]
pub struct StageFacts {
    kind: RecordedStageKind,
    source: Option<u32>,
    span: Option<(u64, u64)>,
    text: Option<RecordedValue>,
}

impl StageFacts {
    /// Which stage this is.
    #[must_use]
    pub const fn kind(&self) -> RecordedStageKind {
        self.kind
    }

    /// The acquired source it names, where it names one.
    #[must_use]
    pub const fn source(&self) -> Option<u32> {
        self.source
    }

    /// The byte range it names, in the acquired byte domain.
    #[must_use]
    pub const fn span(&self) -> Option<(u64, u64)> {
        self.span
    }

    /// A generated artifact's label, or a bundle's own origin claim. Encoder-mediated.
    #[must_use]
    pub const fn text(&self) -> Option<&RecordedValue> {
        self.text.as_ref()
    }
}

/// One recorded site decision, and what the document says about where it came from.
#[derive(Debug, Clone)]
pub struct SiteFacts {
    site: RecordedSite,
    ast: RecordedAst,
    disposition: RecordedDisposition,
    influence: RecordedInfluence,
    shell: MaterialState,
    shell_text: Option<RecordedValue>,
    locator: MaterialState,
    chain: Vec<StageFacts>,
}

impl SiteFacts {
    /// Which site this is — the leaf, with its in-loop member index where it has one.
    ///
    /// One value rather than two integers, because the pair is otherwise substitutable with any
    /// other pair and two same-command sites must not collapse
    /// (`spike/CLAUDE.md:inv-site-keyed-results`).
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// The syntax node it came from, as the document numbered them.
    #[must_use]
    pub const fn ast(&self) -> RecordedAst {
        self.ast
    }

    /// What the plan did with it.
    #[must_use]
    pub const fn disposition(&self) -> RecordedDisposition {
        self.disposition
    }

    /// Where the decision stood relative to host contact.
    ///
    /// Never rehydrated into a live account, and an absent or unverifiable grade reads
    /// the most-influenced grade — the conservative direction, which can only make a reader more
    /// careful. The floor itself is decided at one seat in `reingested`, and naming its token here
    /// would put this module on a roster it has no business joining.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }

    /// Whether the site's own shell text is in the document.
    #[must_use]
    pub const fn shell(&self) -> MaterialState {
        self.shell
    }

    /// That shell text, where it is. Encoder-mediated.
    #[must_use]
    pub const fn shell_text(&self) -> Option<&RecordedValue> {
        self.shell_text.as_ref()
    }

    /// Whether the site's provenance DAG is in the document.
    #[must_use]
    pub const fn locator(&self) -> MaterialState {
        self.locator
    }

    /// That DAG's chain from its head, head first and origins outward.
    #[must_use]
    pub fn chain(&self) -> &[StageFacts] {
        &self.chain
    }

    /// The first authored stage on the chain — where a source address resolves.
    #[must_use]
    pub fn authored_origin(&self) -> Option<&StageFacts> {
        self.chain
            .iter()
            .find(|stage| stage.kind == RecordedStageKind::Authored)
    }
}

/// A population the projection declined to carry, and why.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OmissionFacts {
    species: RecordedSpineSpecies,
    count: u32,
}

impl OmissionFacts {
    /// Which in-memory decision species went uncarried.
    #[must_use]
    pub const fn species(&self) -> RecordedSpineSpecies {
        self.species
    }

    /// How many members it had.
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.count
    }
}

/// Everything a receipt-rooted `why` question could establish, and nothing it could not.
#[derive(Debug, Clone)]
pub struct RecordedWhyFacts {
    root: RootFacts,
    closure: ClosureFacts,
    address: Option<AddressFacts>,
    invocation: InvocationFacts,
    sites: Vec<SiteFacts>,
    sources: Vec<SourceFacts>,
    narratives: Vec<NarrativeFacts>,
    omissions: Vec<OmissionFacts>,
    admission: Option<AdmissionFacts>,
    presented: Option<PresentedPlanFacts>,
    regions: Vec<RegionFacts>,
    loads: Vec<LoadFacts>,
    classifications: Vec<ClassificationFacts>,
    certifications: Vec<CertificationFacts>,
    ships: Vec<ShipFacts>,
    survivals: Vec<SurvivalFacts>,
    renders: Vec<RenderFacts>,
    licensors: Vec<LicensorFacts>,
    rederivation: ReDerivationState,
}

impl RecordedWhyFacts {
    /// The root the question is about.
    #[must_use]
    pub const fn root(&self) -> &RootFacts {
        &self.root
    }

    /// The causal closure it needed.
    #[must_use]
    pub const fn closure(&self) -> &ClosureFacts {
        &self.closure
    }

    /// The address the question asked about, where it asked about one.
    #[must_use]
    pub const fn address(&self) -> Option<&AddressFacts> {
        self.address.as_ref()
    }

    /// Every recorded site decision.
    #[must_use]
    pub fn sites(&self) -> &[SiteFacts] {
        &self.sites
    }

    /// Every acquired source.
    #[must_use]
    pub fn sources(&self) -> &[SourceFacts] {
        &self.sources
    }

    /// What the run was, as the document recorded it.
    #[must_use]
    pub const fn invocation(&self) -> &InvocationFacts {
        &self.invocation
    }

    /// Every decision-inert narrative the run minted — the family carrying the recorded speech
    /// acts. Identifies no site, by the durable's own design.
    #[must_use]
    pub fn narratives(&self) -> &[NarrativeFacts] {
        &self.narratives
    }

    /// The intake outcome, where the document recorded one.
    #[must_use]
    pub const fn admission(&self) -> Option<&AdmissionFacts> {
        self.admission.as_ref()
    }

    /// The approval-surface identities, where the document recorded them.
    #[must_use]
    pub const fn presented(&self) -> Option<&PresentedPlanFacts> {
        self.presented.as_ref()
    }

    /// Every authored region's shared outcome.
    #[must_use]
    pub fn regions(&self) -> &[RegionFacts] {
        &self.regions
    }

    /// Every definition-plane decision.
    #[must_use]
    pub fn loads(&self) -> &[LoadFacts] {
        &self.loads
    }

    /// Every site classification.
    #[must_use]
    pub fn classifications(&self) -> &[ClassificationFacts] {
        &self.classifications
    }

    /// Every dataflow certification.
    #[must_use]
    pub fn certifications(&self) -> &[CertificationFacts] {
        &self.certifications
    }

    /// Every probe shipment.
    #[must_use]
    pub fn ships(&self) -> &[ShipFacts] {
        &self.ships
    }

    /// Every survival-tier outcome.
    #[must_use]
    pub fn survivals(&self) -> &[SurvivalFacts] {
        &self.survivals
    }

    /// Every render-time decision.
    #[must_use]
    pub fn renders(&self) -> &[RenderFacts] {
        &self.renders
    }

    /// Every licensor of an irreversible verb.
    #[must_use]
    pub fn licensors(&self) -> &[LicensorFacts] {
        &self.licensors
    }

    /// What this model can say about EVERY family a plan document persists.
    ///
    /// Exhaustive and no-wildcard, so a family added to the recorded model cannot land here
    /// unclassified — which is the whole point: a consumer must be able to tell a family the
    /// document does not carry from one this read surface has not projected yet, because the two
    /// are repaired in different places and only one of them is a durable question.
    ///
    /// The two OPTIONAL singletons answer `NotCarried` when the document holds no such row, which
    /// is a different fact from an empty collection: a plan that admitted no records did not record
    /// an admission at all, and reporting that as `Projected(0)` would say the projection found an
    /// intake that answered nothing.
    #[must_use]
    pub fn coverage(&self) -> Vec<(PlanFamily, FamilyCoverage)> {
        PlanFamily::ALL
            .iter()
            .map(|family| {
                let coverage = match family {
                    PlanFamily::Invocation => FamilyCoverage::of(1),
                    PlanFamily::Sources => FamilyCoverage::of(self.sources.len()),
                    PlanFamily::Sites => FamilyCoverage::of(self.sites.len()),
                    PlanFamily::Narratives => FamilyCoverage::of(self.narratives.len()),
                    PlanFamily::Omissions => FamilyCoverage::of(self.omissions.len()),
                    PlanFamily::Admission => FamilyCoverage::of_singleton(self.admission.is_some()),
                    PlanFamily::PresentedPlan => {
                        FamilyCoverage::of_singleton(self.presented.is_some())
                    }
                    PlanFamily::Regions => FamilyCoverage::of(self.regions.len()),
                    PlanFamily::Loads => FamilyCoverage::of(self.loads.len()),
                    PlanFamily::Classifications => FamilyCoverage::of(self.classifications.len()),
                    PlanFamily::Certifications => FamilyCoverage::of(self.certifications.len()),
                    PlanFamily::Ships => FamilyCoverage::of(self.ships.len()),
                    PlanFamily::Survivals => FamilyCoverage::of(self.survivals.len()),
                    PlanFamily::Renders => FamilyCoverage::of(self.renders.len()),
                    PlanFamily::Licensors => FamilyCoverage::of(self.licensors.len()),
                };
                (*family, coverage)
            })
            .collect()
    }

    /// Every population the projection declined to carry.
    #[must_use]
    pub fn omissions(&self) -> &[OmissionFacts] {
        &self.omissions
    }

    /// Whether anything was re-derived under current inputs.
    ///
    /// Always an explicit state, never an absence: a reader must be able to tell "checked, and
    /// they agree" from "nobody checked", and only one of those is true today.
    #[must_use]
    pub const fn rederivation(&self) -> ReDerivationState {
        self.rederivation
    }

    /// The site the address resolved to, where one did.
    #[must_use]
    pub fn addressed_site(&self) -> Option<&SiteFacts> {
        let resolved = self.address.as_ref()?.resolved_site()?;
        self.sites.iter().find(|site| site.site == resolved)
    }
}

mod build;

pub use build::{CurrentSourceReading, SourceObservation, WhyFactsInput, derive};
