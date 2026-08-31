//! One datum: the five per-datum fields of `30V` §3, assigned GRANULARLY AT THE LEAF.
//!
//! Hoisting any of the five to a container is what creates laundering and drop pressure (`30V` §3,
//! `[TYPED]`; the influence account's carried-never-stamped law is the precedent), so the fields
//! are private, there is exactly one mint taking all five by value, and no container in this crate
//! carries a field of any of the five kinds.

use dorc_aid::narrative::{Knowability, SpeechAct};
use dorc_receipt::reingested::RecordedInfluence;
use dorc_receipt::report::PlanFamily;
use dorc_receipt::report::{
    AuthenticationState, ClosureCompleteness, CurrentSourceState, DetailState, ProjectionState,
    ReDerivationState, RecordedDocumentId, RecordedSpecies, RecordedValue,
};
use dorc_receipt::rows::{RecordedOperands, RecordedSite};
use dorc_receipt::tokens::{
    RecordedAdmissionOutcome, RecordedDisposition, RecordedInvocationMode, RecordedLicenseCustody,
    RecordedLicenseVerb, RecordedLoadOutcome, RecordedNarrativeKind, RecordedRenderKind,
    RecordedShipLane, RecordedSiteClass, RecordedSolvePass, RecordedSourceClass,
    RecordedSpineSpecies, RecordedSurvivalOutcome,
};

use crate::known::Known;

/// One reconstructed datum.
///
/// No equality, deliberately: it can carry a sealed `RecordedValue`, and `report::value`'s own doc
/// records that a derived equality composes into orderings and hashes, which leak structure.
#[derive(Debug, Clone)]
pub struct Datum {
    speaker: Known<Speaker>,
    world: WorldCoordinate,
    subject: Known<Subject>,
    payload: Known<Payload>,
    delivery: Delivery,
}

impl Datum {
    /// THE mint. All five by value, so a datum cannot exist with a field nobody decided.
    #[must_use]
    pub const fn minted(
        speaker: Known<Speaker>,
        world: WorldCoordinate,
        subject: Known<Subject>,
        payload: Known<Payload>,
        delivery: Delivery,
    ) -> Self {
        Self {
            speaker,
            world,
            subject,
            payload,
            delivery,
        }
    }

    /// Who spoke, and in what act.
    #[must_use]
    pub const fn speaker(&self) -> &Known<Speaker> {
        &self.speaker
    }

    /// Which world-moment this is about.
    #[must_use]
    pub const fn world(&self) -> &WorldCoordinate {
        &self.world
    }

    /// What it is about.
    #[must_use]
    pub const fn subject(&self) -> &Known<Subject> {
        &self.subject
    }

    /// What was said.
    #[must_use]
    pub const fn payload(&self) -> &Known<Payload> {
        &self.payload
    }

    /// Which carrier delivered it.
    #[must_use]
    pub const fn delivery(&self) -> Delivery {
        self.delivery
    }

    /// The trust ordering, DERIVED at projection through the one seat that owns it
    /// (`AID-NEEDS:law-trust-tier-is-syntax`), never stored beside the speaker.
    #[must_use]
    pub fn knowability(&self) -> Option<Knowability> {
        self.speaker
            .value()
            .map(|speaker| speaker.act().knowability())
    }
}

/// Speech-act kind and the voices performing it (`30V` §3 field 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speaker {
    act: SpeechAct,
    voices: Known<VoiceSet>,
}

impl Speaker {
    /// Bind an act to the set that performed it.
    #[must_use]
    pub const fn of(act: SpeechAct, voices: Known<VoiceSet>) -> Self {
        Self { act, voices }
    }

    /// The typed act. Rendered uniformly by arrangement code; never hand-written prose.
    #[must_use]
    pub const fn act(&self) -> SpeechAct {
        self.act
    }

    /// Who performed it.
    #[must_use]
    pub const fn voices(&self) -> &Known<VoiceSet> {
        &self.voices
    }
}

/// Who is speaking — committee-capable, with inseparability a first-class state OF THE SET.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceSet {
    /// The tool's own voice: the terminal attribution link (`30V` §2 rul-first-person-register).
    /// A claim grounded in nobody else's speech is ours.
    Mine,
    /// One named other, where the document places their bytes.
    One(Voice),
    /// Several, whose contributions may or may not be separable.
    ///
    /// NOT MINTED at v1: telling one contributor from several needs the licensor family, which
    /// this read surface does not yet project.
    Committee {
        /// The members.
        voices: Vec<Voice>,
        /// Whether the members' contributions can be told apart — the state a forked remedy needs
        /// (`30V` §2 rul-remedies-may-fork).
        separability: Separability,
    },
}

/// Whether a committee's members can be told apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Separability {
    /// Each member's contribution is individually attributable.
    Separable,
    /// The conclusion required all of them and names none — the leverage point forks.
    Inseparable,
}

/// One named voice, by where they authored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Voice {
    /// An author, named by the acquired source their utterance sits in.
    AuthoredIn(SourceRef),
}

/// One moment × host × attempt-lineage (`30V` §3 field 2).
///
/// A product of three LEAF slots, each carrying its own knowledge state: the coordinate container
/// always exists, and wrapping the container instead would hoist three answers into one.
#[derive(Debug, Clone)]
pub struct WorldCoordinate {
    moment: Known<Moment>,
    host: Known<HostName>,
    lineage: Known<AttemptLineage>,
}

impl WorldCoordinate {
    /// Bind the three leaves.
    #[must_use]
    pub const fn of(
        moment: Known<Moment>,
        host: Known<HostName>,
        lineage: Known<AttemptLineage>,
    ) -> Self {
        Self {
            moment,
            host,
            lineage,
        }
    }

    /// When.
    #[must_use]
    pub const fn moment(&self) -> &Known<Moment> {
        &self.moment
    }

    /// Where.
    #[must_use]
    pub const fn host(&self) -> &Known<HostName> {
        &self.host
    }

    /// Which attempt.
    #[must_use]
    pub const fn lineage(&self) -> &Known<AttemptLineage> {
        &self.lineage
    }
}

/// A world-moment, as the carrier can state it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Moment {
    /// The store order the document was filed under, as spelled.
    Filed(String),
    /// The document carries the undated token — a supported artifact, not a failure.
    Undated,
}

/// A host destination, as somebody spelled it — sealed, and encoder-mediated on the way out.
#[derive(Debug, Clone)]
pub struct HostName(RecordedValue);

impl HostName {
    /// Seal one recorded destination as a host name.
    #[must_use]
    pub const fn of(value: RecordedValue) -> Self {
        Self(value)
    }

    /// The name, encoder-mediated like every other recorded value.
    #[must_use]
    pub const fn value(&self) -> &RecordedValue {
        &self.0
    }
}

/// Which attempt a datum belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttemptLineage {
    /// The document the datum was read from.
    Document(RecordedDocumentId),
}

/// A typed reference INTO structure (`30V` §3 field 3).
///
/// `Site` stays the plain v1 identity: the site-granularity enrichment is held on the frozen
/// kernel, so nothing here is designed around its future shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subject {
    /// One recorded site decision.
    Site(RecordedSite),
    /// One acquired source.
    Source(SourceRef),
    /// One stage of one site's provenance chain.
    Stage {
        /// The site whose chain this stage sits on.
        site: RecordedSite,
        /// Its position in the chain, head first.
        index: u32,
    },
    /// One document in the rooted closure.
    Document(RecordedDocumentId),
    /// The address the question asked about.
    Address(AddressSubject),
    /// One decision-inert narrative, by its mint ordinal. It identifies no site, by the durable.s
    /// own design, and this subject must not suggest it does.
    Narrative(u32),
    /// One authored REGION, by its recorded ordinal.
    ///
    /// Its own arm rather than a site: a region is one authored edit many executions share
    /// (`30L:rul-two-identities-never-conflated`), so keying it by a leaf would let one instance
    /// stand for every other invocation of the same body.
    Region(u32),
    /// One definition-plane decision, by its recorded ordinal.
    Load(u32),
    /// A whole recorded FAMILY the projection declined, or the report API does not carry — the
    /// subject an audit row is about.
    Family(PlanFamily),
}

/// The address a question asked about, as this model can state it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressSubject {
    /// Which acquired source, by ordinal.
    pub source: SourceRef,
    /// Which physical line, 1-indexed, as the user spelled it.
    pub line: u32,
}

/// One acquired source, by its ordinal in the recorded table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceRef(u32);

impl SourceRef {
    /// Name one ordinal.
    #[must_use]
    pub const fn of(ordinal: u32) -> Self {
        Self(ordinal)
    }

    /// The ordinal.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// What was said (`30V` §3 field 4) — a type-FAMILY rather than an axis, and the least-settled of
/// the five. Closed, and census-gated: every constructible kind must reach output.
#[derive(Debug, Clone)]
pub enum Payload {
    /// What the plan did with a site.
    Decision(RecordedDisposition),
    /// Where a record stood relative to host contact. Never rehydrated into a live account; an
    /// absent or unverifiable grade already reads most-influenced at the recorded seat.
    Influence(RecordedInfluence),
    /// An identity, a digest, or a count — material with no other spelling to escape through.
    Identity(IdentityFact),
    /// One of the report's own closed state words.
    State(StateFact),
    /// Bytes the document carried. Sealed: the ONE exit is a `ValueEncoder`.
    Text(RecordedValue),
    /// A typed edge or finding of the receipt graph.
    Correlation(CorrelationFact),
    /// Which safety-narrowing a narrative recorded.
    Collapse(RecordedNarrativeKind),
    /// One word of a recorded closed vocabulary — what a family's own row SAYS.
    Token(RecordedToken),
    /// One named yes-or-no a recorded row carries.
    Flag(RecordedFlag),
    /// An affirmatively-known fact about not-knowing, carrying its own remedy.
    NegativeSpace(NegativeSpace),
}

/// One word of a recorded closed vocabulary.
///
/// Wrapped rather than flattened into [`IdentityFact`] because these are not identities: they are
/// what a row SAID, in the document's own closed words. Closed and no-wildcard at every consumer,
/// so a widened recorded vocabulary reddens the seats that render it rather than falling into a
/// neighbour's spelling (`inv-referent-agnostic`: the token is resolved for display, never branched
/// on for meaning).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedToken {
    /// What intake answered.
    AdmissionOutcome(RecordedAdmissionOutcome),
    /// What the definition plane decided about one load.
    LoadOutcome(RecordedLoadOutcome),
    /// What the analysis took a site to be.
    SiteClass(RecordedSiteClass),
    /// Which dataflow answer a certification is about.
    SolvePass(RecordedSolvePass),
    /// Which body a probe site shipped.
    ShipLane(RecordedShipLane),
    /// What a survival walk decided.
    SurvivalOutcome(RecordedSurvivalOutcome),
    /// Which render-time edit was made.
    RenderKind(RecordedRenderKind),
    /// Which irreversible verb was licensed.
    LicenseVerb(RecordedLicenseVerb),
    /// Whose utterance the license rests on.
    LicenseCustody(RecordedLicenseCustody),
}

/// One named predicate a recorded row carries.
///
/// The predicate travels WITH its answer: a bare `bool` payload would be substitutable with any
/// other bool, so a reader could not tell a tripped certifier from an invalidating site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedFlag {
    /// Whether a classified site is on the verdict lane.
    VerdictLane(bool),
    /// Whether a classified site invalidates anything.
    Invalidator(bool),
    /// Whether the certifier agreed with the solver.
    SolveConsistent(bool),
    /// Whether the certifier's latch tripped.
    SolveTripped(bool),
}

/// Machine-shaped identity material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityFact {
    /// A document identity.
    Document(RecordedDocumentId),
    /// A document species.
    Species(RecordedSpecies),
    /// A content digest, as spelled.
    Digest(String),
    /// A byte length.
    Bytes(u64),
    /// A count of members. Wide because the recorded counts are: a region's route tally and an
    /// admission's record tally are both `u64` on the wire, and narrowing one here would saturate
    /// a number the document spelled exactly.
    Count(u64),
    /// How many operands a capped account shows, and how many it dropped.
    Operands(RecordedOperands),
    /// An in-memory decision species the projection declined to carry.
    UncarriedSpecies(RecordedSpineSpecies),
    /// Which dialect a source was accepted as.
    SourceClass(RecordedSourceClass),
    /// A syntax-node ordinal, as the document numbered them.
    Ast(u32),
    /// What the run was doing, in the recorded vocabulary.
    InvocationMode(RecordedInvocationMode),
}

/// One of the report's closed state words, carried as a payload so it reaches the total surface
/// exactly like every other datum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateFact {
    /// What outer verification said.
    Authentication(AuthenticationState),
    /// Which projection the document is.
    Projection(ProjectionState),
    /// Whether the grouped detail region opened.
    Detail(DetailState),
    /// Whether the rooted closure was assembled whole.
    Closure(ClosureCompleteness),
    /// How the current tree stands against a recorded source.
    CurrentSource(CurrentSourceState),
    /// Whether anything was re-derived under current inputs.
    ReDerivation(ReDerivationState),
}

/// A typed correlation between documents. Missing edges stay missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorrelationFact {
    /// A plan feeds an apply intent.
    PlanToIntent {
        /// The plan.
        plan: RecordedDocumentId,
        /// The intent citing it.
        intent: RecordedDocumentId,
    },
    /// An intent has an outcome.
    IntentToOutcome {
        /// The intent.
        intent: RecordedDocumentId,
        /// Its outcome.
        outcome: RecordedDocumentId,
    },
    /// A shape of the record SET the reader would otherwise have to infer. Never rounded into a
    /// story about what probably happened.
    Finding(FindingKind),
}

/// The graph's own closed finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingKind {
    /// Two documents claim one identity and are not the same bytes.
    IdentityCollision,
    /// An intent names an originating plan the store does not hold.
    OriginatingPlanAbsent,
    /// An intent's originating plan could not be read.
    OriginatingPlanUnavailable,
    /// An outcome answers an intent the store does not hold.
    OutcomeWithoutIntent,
    /// An outcome's intent could not be read.
    OutcomeIntentUnreadable,
    /// An intent has more than one outcome.
    SupernumeraryOutcome,
    /// A document's identity would not read.
    IdentityUnreadable,
}

/// An affirmative fact about not-knowing, and where a remedy would land.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NegativeSpace {
    /// What kind of not-knowing this is.
    pub kind: NegativeKind,
    /// Which family a remedy would have to reach.
    pub family: PlanFamily,
}

/// Which species of not-knowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NegativeKind {
    /// The document holds the family and the report API does not project it.
    ReportApiGap,
    /// No receipt version has ever carried this.
    CarrierGap,
}

/// Which carrier delivered a datum (`30V` §3 field 5).
///
/// NOT wrapped in [`Known`]: this is a fact about our OWN act of collection, and a wrapper would
/// represent a state that cannot occur. The `Live` arm is representable and unconstructed at v1 —
/// the model must not bake ingest-only assumptions (`30V` §3, `[TYPED]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delivery {
    /// A receipt in the rooted closure, BY REFERENCE: authentication and completeness are looked up
    /// on the carrier entity and never copied onto the datum.
    Recorded(CarrierRef),
    /// The same reporting machinery driven in real time. Unconstructed at v1; `dorc apply --why`
    /// is the owed shape (`30V` §6).
    Live,
}

/// One carrier in the rooted closure, by position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarrierRef(usize);

impl CarrierRef {
    /// Name one position.
    #[must_use]
    pub const fn of(index: usize) -> Self {
        Self(index)
    }

    /// The position.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}
