//! `core::spine` — the **one in-memory decision structure** (`plans/309` §0; census `notes/30E`).
//!
//! Every decision the engine reaches, the account of what it read, its influence grade, and its
//! narration hang off one structure. Everything a run appears to *produce* — the apply artifact,
//! the plan render, the orchestrator's connections, the `.whylog` durable — is a **projection** of
//! Spine × the input files, never an independently-assembled product.
//!
//! # Position, not a guard (`309:law-spine-outside-the-kernel`)
//!
//! Spine lies slightly outside the analysis kernel, and the firewall is one-way: the engine WRITES
//! finalized decisions onto Spine and the analysis loop never reads them back. That is realized by
//! POSITION — Spine is written post-decision, from outside anything the solver compares, so Spine
//! values never enter compared state and **no `Eq`-exclusion is needed anywhere**. The
//! `CollapseNarrative` `Eq`-exclusion (`22W` §2) is cited here as the failure-mode this positioning
//! AVOIDS, never as a technique to generalize: that precedent is safe only because narratives are
//! decision-inert, and Eq-excluding a license-bearing record from the machinery that guards it
//! would be a hole.
//!
//! `309:watch-firewall-is-default-not-dogma`: if a case ever appears where re-reading a finalized
//! decision is genuinely right, that is a deliberate, dearly-bought design act — not a workaround.
//!
//! # The durable is defined by EXCLUSION (`309:rul-durable-by-exclusion`)
//!
//! We do not choose what new things to make durable. Spine tracks totalistically and the `.whylog`
//! projection is what survives an exclusion set. [`SpineSpecies::census_arm`] is the no-wildcard
//! classification that makes that mechanical: a new species cannot land unclassified, silent
//! inclusion and silent omission are both unrepresentable, and ENTERING [`CensusArm::Durable`] is
//! the durable tripwire firing (`rul-durable-contents-reviewed-before-design`).
//!
//! Field-level exclusion is structural rather than classified: durable species reach the durable
//! ONLY through a per-species `DurableView` in `plan::whylog`, whose fields ARE the durable subset.
//! Records themselves never implement serialization, so a field that no View names cannot reach
//! disk, and lifting one exclusion is one field added to one View — a diff that IS the tripwire's
//! mechanical form. [`ExcludedContent`] enumerates what is ruled non-durable at the CONTENT tier.
//!
//! # Generic over the decide plane, so `core` stays dependency-clean
//!
//! A [`SpineDisposition`] is license-bearing and a [`SpineRecordStream`] holds admitted host bytes,
//! and both of those types are minted in `plan` — the crate whose law owns irreversible verbs and
//! the intake edge. `core` may not depend on `plan`, so [`Spine`] is generic over ONE seam,
//! [`DecidePlane`], which names those payload types. `plan` implements it once and aliases
//! `Spine<PlanPlane>`, so the parameter is invisible to every consumer downstream of that alias.

use std::collections::BTreeMap;

use crate::influence::InfluencePhase;
use crate::{AstId, DefinitionCustody, FactKey, KindId, LeafId, RunInstant, SiteId, SourceFileId};

/// The payload types a Spine carries that `core` cannot name (`309` §2 crate-home).
///
/// Two, and the split is meaningful: the license-bearing decision is minted under `plan`'s
/// sole-mint law, and the admitted record buffer is minted at `plan`'s intake edge. Both must stay
/// where their mints are guarded, so the Spine names them through this seam instead of holding
/// copies `core` would have to be trusted with.
pub trait DecidePlane {
    /// The license-bearing per-site decision (`plan::Disposition` at the one instantiation).
    type Decision: core::fmt::Debug + Clone;
    /// The admitted host-record buffer, as received (`plan::records::AdmittedUnscopedHostRecords`).
    ///
    /// Held as the plane's own type rather than as bytes on purpose: the durable writer takes it by
    /// reference precisely so untrusted result bytes have no raw serialization route
    /// (`rul-host-bytes-bounded-before-admission`), and copying them into `core` would open one.
    type Records: core::fmt::Debug + Clone;
    /// The decision-inert narration a collapse mints (`dorc_aid::CollapseNarrative`).
    ///
    /// Named through this seam rather than held by `core` because narration is the DESCRIBE plane
    /// (`aid-is-the-describe-plane`), and a `core → aid` edge would mean a decision reading a
    /// narration. `narrative-is-sealed-by-type-not-place` is what makes co-location safe: the seal
    /// is private fields and `ProvId: !Ord`, not which crate the value sits in.
    type Narrative: core::fmt::Debug + Clone;
}

/// How many exemplars an unbounded operand account keeps before it reports a count instead
/// (`309:law-spine-operands-capped`, promoting `operands-are-pure-and-capped` from a durable-file
/// rule to a construction law on the type).
pub const SPINE_OPERAND_CAP: usize = 8;

/// A k-capped by-value account of an unbounded operand list.
///
/// The truncation count is PART OF THE TYPE rather than a silently-lossy `Vec::truncate`: a reader
/// can always tell a short list from a clipped one, which is the property that makes an account
/// admissible where the evidence itself is not.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Account<T> {
    shown: Vec<T>,
    dropped: u32,
}

impl<T> Account<T> {
    /// Keep at most [`SPINE_OPERAND_CAP`] exemplars, counting the rest.
    pub fn capped(items: impl IntoIterator<Item = T>) -> Self {
        let mut shown = Vec::new();
        let mut dropped = 0u32;
        for item in items {
            if shown.len() < SPINE_OPERAND_CAP {
                shown.push(item);
            } else {
                dropped = dropped.saturating_add(1);
            }
        }
        Self { shown, dropped }
    }

    /// The retained exemplars.
    #[must_use]
    pub fn shown(&self) -> &[T] {
        &self.shown
    }

    /// How many were elided past the cap.
    #[must_use]
    pub const fn dropped(&self) -> u32 {
        self.dropped
    }

    /// The total the account stands for.
    #[must_use]
    pub fn total(&self) -> u32 {
        u32::try_from(self.shown.len())
            .unwrap_or(u32::MAX)
            .saturating_add(self.dropped)
    }
}

/// The influence grade a record was minted at (`309` §2; `306b` §1a via [`InfluencePhase`]).
///
/// `None` ⇒ `authored-before-contact`: everything behind this decision existed before the first
/// host exchange. `Some` ⇒ `host-influenced`, and the marker can only be obtained by having READ
/// host-reported material (`core::influence`'s pairing), so the stamp is evidence rather than an
/// assertion a mint site could get wrong.
///
/// v0 is positional and global (`306c` §2): the flip is a phase property carried by construction,
/// not a per-value dataflow analysis. That is why a mint site does not fill this in —
/// [`Spine::minted_at`] does, on every record, so a new mint site cannot forget and a future
/// per-record gradation (`306b` §1c, OPEN) has its room without being pre-committed here.
pub type Grade = Option<InfluencePhase>;

/// Every Spine record species (`30E` §2). A no-wildcard `match` in [`SpineSpecies::census_arm`]
/// forces a new species to be classified before it can land.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpineSpecies {
    /// Mode, argv, book, oracles, and the controller-minted run identity.
    Invocation,
    /// The admitted host-record stream as received, with its arrival instants.
    RecordStream,
    /// The per-site licensed decision — the license-bearing record `Plan` projects from.
    Disposition,
    /// The decision digest over the identity plane.
    Digest,
    /// Definition binding, custody, contested families, never-live, helper conflicts.
    LoadDecision,
    /// The classify tuple per site: class, verdict-lane, kills, backings, degrade causes.
    SiteClassification,
    /// Per-pass solve consistency and the certifier-trip latch.
    SolveCertification,
    /// A vouch's attachment or suspension, with custody.
    Vouch,
    /// Which body a probe site shipped, or why none could be.
    ProbeShip,
    /// The closed intake outcome and its influence phase marker.
    Admission,
    /// A site's observable, the by-fact merge, and collapsed cells.
    Observation,
    /// One validity-fixpoint round: its erasures and cascades.
    ValidityRound,
    /// Survival witnesses, wall crossings, demotions, re-derivation disagreements.
    Survival,
    /// The render-time decisions `30E` §3 audited out of hiding.
    RenderDecision,
    /// The run's outcome: exit-code class, advisory routing, durable eligibility.
    Outcome,
}

/// Which arm of the durable census a species sits in (`309:mech-census-three-states`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CensusArm {
    /// Written to `.whylog`, exclusively through a per-species `DurableView` whose fields ARE the
    /// durable subset. **Entering this arm is the durable tripwire firing** — human and/or opaque
    /// review, always (`rul-durable-contents-reviewed-before-design`).
    Durable,
    /// RULED non-durable: not a resting state but a decision. Empty of species today because
    /// `30E` §2's exclusions are all CONTENT-tier ([`ExcludedContent`]) and the `DurableView`
    /// mechanism excludes them structurally; the arm exists so a species-tier ruling has a home
    /// that reads differently from "not yet".
    Excluded,
    /// Transitory: non-durable in production but not ruled non-durable — the legal resting state
    /// for in-flight work. Dumps durably ONLY through the project-internal debug dump, which is
    /// structurally unable to ship (`309:pin-debug-dump-gating`).
    New,
}

impl SpineSpecies {
    /// Every species, in declaration order. The array is what makes the census walkable; the
    /// no-wildcard match in [`census_arm`](Self::census_arm) is what makes it complete.
    pub const ALL: [Self; 15] = [
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
        Self::Outcome,
    ];

    /// The species' census arm — the no-wildcard classification (`309:mech-census-three-states`).
    ///
    /// A new variant stops this compiling until it is classified, which is the whole mechanism:
    /// silent inclusion and silent omission are both unrepresentable.
    #[must_use]
    pub const fn census_arm(self) -> CensusArm {
        match self {
            Self::Invocation | Self::RecordStream | Self::Disposition | Self::Digest => {
                CensusArm::Durable
            }
            Self::LoadDecision
            | Self::SiteClassification
            | Self::SolveCertification
            | Self::Vouch
            | Self::ProbeShip
            | Self::Admission
            | Self::Observation
            | Self::ValidityRound
            | Self::Survival
            | Self::RenderDecision
            | Self::Outcome => CensusArm::New,
        }
    }

    /// The species' greppable name — for the debug dump and the census instrument.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Invocation => "SpineInvocation",
            Self::RecordStream => "SpineRecordStream",
            Self::Disposition => "SpineDisposition",
            Self::Digest => "SpineDigest",
            Self::LoadDecision => "SpineLoadDecision",
            Self::SiteClassification => "SpineSiteClassification",
            Self::SolveCertification => "SpineSolveCertification",
            Self::Vouch => "SpineVouch",
            Self::ProbeShip => "SpineProbeShip",
            Self::Admission => "SpineAdmission",
            Self::Observation => "SpineObservation",
            Self::ValidityRound => "SpineValidityRound",
            Self::Survival => "SpineSurvival",
            Self::RenderDecision => "SpineRenderDecision",
            Self::Outcome => "SpineOutcome",
        }
    }
}

/// What is ruled non-durable at the CONTENT tier (`30E` §2's `excluded` arm).
///
/// These are not species; they are content classes that no `DurableView` may ever name. Keeping
/// them enumerated rather than merely absent is what makes `rul-durable-by-exclusion` an exclusion
/// SET one can read, and what turns a future lift into a visible deletion from this list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExcludedContent {
    /// The influence grade. `306c` §2's load-bearing scope fence: v0 is in-memory precisely so this
    /// arc does not fire the durable tripwire, which is also why `306b` §3a's rehydration rules are
    /// not owed yet.
    InfluenceGrade,
    /// Narrative operands, `ProvId`s, and arena handles (`operands-are-pure-and-capped`).
    NarrativeOperands,
    /// Freeform host output (`306b` §2b) — retained bounded and inert, never projected.
    FreeformHostOutput,
    /// Working lattice state: the solver's intermediate values, which are not decisions at all.
    WorkingLatticeState,
}

impl ExcludedContent {
    /// Every ruled exclusion, for the census instrument.
    pub const ALL: [Self; 4] = [
        Self::InfluenceGrade,
        Self::NarrativeOperands,
        Self::FreeformHostOutput,
        Self::WorkingLatticeState,
    ];
}

// ===========================================================================
// The record species
// ===========================================================================

/// One oracle or book input the run loaded, by path and content digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceClaim {
    /// The path as the invocation named it.
    pub path: String,
    /// The content digest at load time.
    pub digest: String,
}

/// The invocation: controller-minted run identity plus what it was pointed at (`30E` §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineInvocation {
    /// `plan` / `apply` / `roundtrip` / `probe` / `why`.
    pub mode: String,
    /// The full argv, one word per element.
    pub argv: Vec<String>,
    /// The book path and its content digest.
    pub book: SourceClaim,
    /// Each oracle path and digest, in load order.
    pub oracles: Vec<SourceClaim>,
    /// The controller-minted per-attempt nonce.
    pub nonce: String,
    /// The controller-minted attempt serial.
    pub attempt: u32,
    /// The controller-selected host identity.
    pub host: String,
    /// When the controller started this run, from the edge's injected clock. `None` ⇒ no clock.
    pub started_at: Option<RunInstant>,
    /// Authored-before-contact by construction: every field is controller-owned invocation context.
    pub grade: Grade,
}

/// The admitted host-record stream, held as the plane's own admitted-bytes handle (`30E` §2).
#[derive(Debug, Clone)]
pub struct SpineRecordStream<P: DecidePlane> {
    /// The as-received buffer, still wearing its admission.
    pub records: P::Records,
    /// When the controller took each record in, by arrival ordinal, ascending
    /// (`28F:rul-probe-instants-host-says-no-times` — controller-minted, always).
    pub instants: Vec<(u64, RunInstant)>,
    /// Host-influenced by construction: these ARE the host-reported bytes.
    pub grade: Grade,
}

/// One site's licensed decision — the license-bearing record (`30E` §2).
///
/// This is the species that vindicates `DurableView` over a species-arity census (`309` critical-2):
/// the RECORD is `SiteId`-keyed and carries the license, while the VIEW emits a leaf plus a tag.
#[derive(Debug, Clone)]
pub struct SpineDisposition<P: DecidePlane> {
    /// The fine site key (`inv-site-keyed-results`): `(leaf, member)`, never collapsed.
    pub site: SiteId,
    /// The source back-map.
    pub ast: AstId,
    /// The verbatim leaf bytes.
    pub sh: String,
    /// The license-bearing decision.
    pub decision: P::Decision,
    /// The grade at mint.
    pub grade: Grade,
}

/// The decision digest over the identity plane (`22A` concl-3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineDigest {
    /// The 16-hex-char FNV-1a digest.
    pub digest: String,
    /// The grade at mint.
    pub grade: Grade,
}

/// A definition-plane decision: which body a role name binds to, and why a family was withheld.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineLoadDecision {
    /// The munged role or family name.
    pub name: String,
    /// Whose utterance the binding rests on. Compared, never read for its file id
    /// (`custody-is-one-newtype-and-one-crossing`).
    pub custody: Option<DefinitionCustody>,
    /// Why the family's licenses are withheld, if they are.
    pub withheld: Option<WithheldCause>,
    /// The grade at mint.
    pub grade: Grade,
}

/// Why a role family's licenses are withheld for a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WithheldCause {
    /// One unit's definition provably shadowed a different unit's (`28K` §1).
    Contested,
    /// The name's exit binding is ⊤, so it licenses nothing (`top-licenses-nothing`).
    Unprovable,
    /// Two loaded sources declared the same helper name (`helper-conflicts-report-at-the-load-edge`).
    HelperConflict,
}

/// One site's classification outcome — the analysis tuple, as an account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineSiteClassification {
    /// The fine site key.
    pub site: SiteId,
    /// The `SkipClass` discriminant name (referent-agnostic: a label, never branched on here).
    pub class: &'static str,
    /// Whether the site is verdict-lane (`verdict-lane-is-site-keyed`).
    pub verdict_lane: bool,
    /// Whether the site gens into reach as an invalidator.
    pub invalidator: bool,
    /// The cells this site's decision keys on, capped.
    pub cells: Account<FactKey>,
    /// The grade at mint.
    pub grade: Grade,
}

/// One solve pass's certification outcome (`plans/302`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineSolveCertification {
    /// The pass label (value · funcenv · reach · self-reach).
    pub pass: &'static str,
    /// Whether the answer certified. `false` ⇒ the whole analysis window demoted to its floor.
    pub consistent: bool,
    /// Whether the monotone trip latch is set at this point.
    pub tripped: bool,
    /// The grade at mint.
    pub grade: Grade,
}

/// A vouch's attachment or suspension at one site (`rul-vouch-is-verdict-authoring`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineVouch {
    /// The fine site key.
    pub site: SiteId,
    /// The cell the vouch answers about.
    pub fact: FactKey,
    /// Whose utterance it is.
    pub custody: Option<DefinitionCustody>,
    /// `false` ⇒ suspended: the composition that will run is not the region its author vouched.
    pub attached: bool,
    /// The grade at mint.
    pub grade: Grade,
}

/// Which body a probe site shipped, or why none could be (`ship-seam-reads-the-lane-not-the-kind`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineProbeShip {
    /// The fine site key.
    pub site: SiteId,
    /// Which lane shipped.
    pub lane: ShipLane,
    /// The defining file of the shipped body, for provenance and display only.
    pub defining_file: Option<SourceFileId>,
    /// The grade at mint.
    pub grade: Grade,
}

/// Which body a probe site shipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShipLane {
    /// The site's own verdict body — the measurement whose rc IS the convergence answer.
    Verdict,
    /// A `__predict` model, where elision is statically unavailable.
    Predict,
    /// Nothing shippable: the site is unresolvable and the apply runs it.
    Unresolvable,
}

/// The closed intake outcome (`rul-admission-is-a-closed-outcome`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineAdmission {
    /// Which of the three the intake answered.
    pub outcome: AdmissionOutcome,
    /// The named condition on a refusal, for attribution.
    pub fault: Option<String>,
    /// The grade at mint. Host-influenced once anything was read.
    pub grade: Grade,
}

/// The three intake answers, which are never interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdmissionOutcome {
    /// Usable facts arrived.
    Admitted,
    /// A well-owned attempt that produced no usable fact: ordinary conservative planning.
    NoObservation,
    /// Framing, bounds, attribution, or integrity failure: no plan carrying mutation authority.
    Refused,
}

/// One site's observable as the fold saw it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineObservation {
    /// The fine site key.
    pub site: SiteId,
    /// The cell measured.
    pub fact: FactKey,
    /// The Effect-channel verdict label (converged / diverged / unknown).
    pub verdict: &'static str,
    /// Whether the merge over same-cell measurements collapsed to ⊤.
    pub collapsed: bool,
    /// The grade at mint.
    pub grade: Grade,
}

/// One round of the validity fixpoint (`the-fixpoint-owns-the-rounds-and-builds-nothing-else`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineValidityRound {
    /// The round ordinal, from 1.
    pub round: u32,
    /// The sites this round proved dead and erased, capped.
    pub erased: Account<SiteId>,
    /// The grade at mint.
    pub grade: Grade,
}

/// A survival-tier outcome at one site (`survive-license`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineSurvival {
    /// The leaf whose elision was tested.
    pub leaf: LeafId,
    /// What the wall walk answered.
    pub outcome: SurvivalOutcome,
    /// The reach-function kind that poisoned it, where one did.
    pub poisoned_by: Option<KindId>,
    /// The grade at mint.
    pub grade: Grade,
}

/// What the survival walk decided about one elision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SurvivalOutcome {
    /// Crossed no wall — an ordinary pre-wall elision.
    Clean,
    /// Crossed ≥1 running wall, all provably disjoint, under the consent flag.
    SurvivedStandalone,
    /// One atomic aggregate survived only after every erased establish crossed independently.
    SurvivedAggregate {
        /// Exact number of establishes erased by the aggregate.
        establishes: u32,
    },
    /// Demoted to run, for one of the three reasons the walk distinguishes.
    Demoted(SurvivalDemote),
    /// The naive reference model declined to confirm a survival the wall walk had minted
    /// (`rederivation-is-demote-only`), naming the crossed wall's ordinal. Non-empty is a finding
    /// about OUR engine, never about the book's text.
    RederivationDisagreed {
        /// The crossed wall's ordinal in the accumulated set.
        wall: u32,
    },
}

/// Why a survival demoted (`survival::DemoteReason`, as decision-plane record content).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SurvivalDemote {
    /// A footprint-less running mutator totalised the wall — silence walls.
    TotalWall,
    /// The backing hit an accumulated footprint.
    Poisoned,
    /// A same-kind pair could not be canonicalized (`24F` §3a). A SWAMPED count is a finding to
    /// report about the resolvers, never a license to weaken the may-alias default.
    MayAlias,
    /// A reach solve failed its own post-fixpoint check, so nothing rests on its answer
    /// (`302:rul-whole-window-demotion`). Distinct from `TotalWall` because it is a finding about
    /// OUR solver and not a claim about the book's mutators — narrating it as a wall tells an admin
    /// their script caused a demotion we caused (`302` §5's admin-honesty, and `271:rul-sin-ordering`
    /// mis-attribution).
    SolveInconsistent,
}

/// A render-time decision, hoisted out of hiding (`30E` §3's audit).
///
/// Every one of these is license-relevant and was, before the reification, made inside the render
/// with only a diagnostic between it and the structured decision plane. Recording them here is what
/// lets a projection be compared against what was actually decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineRenderDecision {
    /// The site the decision belongs to, where it has one.
    pub site: Option<SiteId>,
    /// Which render-time decision this is.
    pub decision: RenderDecision,
    /// The grade at mint.
    pub grade: Grade,
}

/// The audited render-time decisions (`30E` §3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderDecision {
    /// `dec-pinned-definitions` — which body a guard invokes, under what name. A misalignment
    /// swaps WHOSE judgment executes, which is pope-sin tier (`271:rul-sin-ordering`).
    PinnedBinding {
        /// The emitted name the guard invokes.
        invoked: String,
    },
    /// `dec-render-refusal` — a leaf the disposition layer LICENSED that the span render refuses,
    /// so the artifact runs the bytes verbatim while the record still reads Replace/Guard.
    Refused {
        /// Why the span could not be edited.
        cause: RefusalCause,
    },
    /// `dec-omit-neutralisation` — whether an `Omit` renders as `:` or stays verbatim, decided by
    /// walking the controller. This is `erasure-demands-a-proof-and-a-rendered-death`'s wrong-yes
    /// fence, evaluated at render time.
    OmitNeutralised {
        /// `false` ⇒ the controller was not neutralised, so the body renders verbatim and runs.
        neutralised: bool,
    },
    /// `dec-defensive-emission` — the whole-artifact emission regime. Site-less: it is a property
    /// of the unit, not of any one insert.
    DefensiveEmission {
        /// `true` ⇒ every emitted name munges rather than trusting that a bare one still resolves.
        defensive: bool,
    },
    /// `dec-certifier-trip-cleanup` — a disposition demoted after construction because the solve
    /// certifier tripped (`302:rul-certifier-trip-guard-only`).
    CertifierTripDemote,
}

/// Why a span render refused a licensed decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefusalCause {
    /// The leaf's span covers `<<EOF` rather than the body, so it cannot be safely edited.
    Heredoc,
    /// A guard would sit in front of a blocking output redirect.
    BlockingRedirect,
}

/// The run's outcome — authority-adjacent, because `EXIT_BOOK_UNMODELED` exists precisely so a
/// `dorc … && deploy` chain STOPS (`30E` §4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineOutcome {
    /// The outcome discriminant's name.
    pub outcome: &'static str,
    /// Whether advisory (render-plane) disclosure was routed for this run.
    pub advisory: bool,
    /// Whether the run was eligible to write a durable.
    pub durable_eligible: bool,
    /// The grade at mint.
    pub grade: Grade,
}

// ===========================================================================
// The Spine
// ===========================================================================

/// The one in-memory decision structure (`309` §0).
///
/// Deterministic by construction (`inv-determinism`): site-keyed collections are `BTreeMap`s and
/// every sequence is mint-ordered, so a projection walking Spine is a pure function of the run.
#[derive(Debug)]
pub struct Spine<P: DecidePlane> {
    invocation: Option<SpineInvocation>,
    record_stream: Option<SpineRecordStream<P>>,
    dispositions: BTreeMap<SiteId, SpineDisposition<P>>,
    digest: Option<SpineDigest>,
    load_decisions: Vec<SpineLoadDecision>,
    classifications: BTreeMap<SiteId, SpineSiteClassification>,
    certifications: Vec<SpineSolveCertification>,
    vouches: Vec<SpineVouch>,
    ships: BTreeMap<SiteId, SpineProbeShip>,
    admission: Option<SpineAdmission>,
    observations: BTreeMap<SiteId, SpineObservation>,
    rounds: Vec<SpineValidityRound>,
    survivals: Vec<SpineSurvival>,
    render_decisions: Vec<SpineRenderDecision>,
    outcome: Option<SpineOutcome>,
    narratives: Vec<P::Narrative>,
    grade: Grade,
}

impl<P: DecidePlane> Default for Spine<P> {
    fn default() -> Self {
        Self {
            invocation: None,
            record_stream: None,
            dispositions: BTreeMap::new(),
            digest: None,
            load_decisions: Vec::new(),
            classifications: BTreeMap::new(),
            certifications: Vec::new(),
            vouches: Vec::new(),
            ships: BTreeMap::new(),
            admission: None,
            observations: BTreeMap::new(),
            rounds: Vec::new(),
            survivals: Vec::new(),
            render_decisions: Vec::new(),
            outcome: None,
            narratives: Vec::new(),
            grade: None,
        }
    }
}

impl<P: DecidePlane> Spine<P> {
    /// An empty Spine over material that exists before the first host exchange.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// An empty Spine whose records are all minted at `grade` (`309` §2 grade-stamping).
    ///
    /// v0's flip is POSITIONAL and GLOBAL (`306c` §2): once host bytes are read, every code path
    /// invoked after that point is within its scope, so the grade belongs to the Spine a run builds
    /// rather than to the discipline of each mint site. Handing it in at construction is what makes
    /// it unforgettable — there is no per-record decision to get wrong, and no widening a later mint
    /// site could omit.
    #[must_use]
    pub fn minted_at(grade: Grade) -> Self {
        Self {
            grade,
            ..Self::default()
        }
    }

    /// The grade every record on this Spine carries.
    #[must_use]
    pub const fn grade(&self) -> Grade {
        self.grade
    }

    /// Write the invocation record.
    pub fn set_invocation(&mut self, mut record: SpineInvocation) {
        record.grade = self.grade;
        self.invocation = Some(record);
    }

    /// The invocation record, if the run reached the point of minting one.
    #[must_use]
    pub const fn invocation(&self) -> Option<&SpineInvocation> {
        self.invocation.as_ref()
    }

    /// Write the admitted record stream.
    pub fn set_record_stream(&mut self, mut record: SpineRecordStream<P>) {
        record.grade = self.grade;
        self.record_stream = Some(record);
    }

    /// The admitted record stream, if any was admitted.
    #[must_use]
    pub const fn record_stream(&self) -> Option<&SpineRecordStream<P>> {
        self.record_stream.as_ref()
    }

    /// Write one site's licensed decision.
    pub fn set_disposition(&mut self, mut record: SpineDisposition<P>) {
        record.grade = self.grade;
        self.dispositions.insert(record.site, record);
    }

    /// Every site decision, in site order.
    pub fn dispositions(&self) -> impl Iterator<Item = &SpineDisposition<P>> {
        self.dispositions.values()
    }

    /// One site's decision.
    #[must_use]
    pub fn disposition(&self, site: SiteId) -> Option<&SpineDisposition<P>> {
        self.dispositions.get(&site)
    }

    /// Mutable access to one site's decision — for the post-construction demotions the render and
    /// the certifier-trip cleanup perform, which are Spine writes like any other.
    pub fn disposition_mut(&mut self, site: SiteId) -> Option<&mut SpineDisposition<P>> {
        self.dispositions.get_mut(&site)
    }

    /// Every site decision, mutably, in site order — for a whole-plan demotion sweep.
    pub fn dispositions_mut(&mut self) -> impl Iterator<Item = &mut SpineDisposition<P>> {
        self.dispositions.values_mut()
    }

    /// Write the decision digest.
    pub fn set_digest(&mut self, mut record: SpineDigest) {
        record.grade = self.grade;
        self.digest = Some(record);
    }

    /// The decision digest.
    #[must_use]
    pub const fn digest(&self) -> Option<&SpineDigest> {
        self.digest.as_ref()
    }

    /// Write the intake outcome.
    pub fn set_admission(&mut self, mut record: SpineAdmission) {
        record.grade = self.grade;
        self.admission = Some(record);
    }

    /// The intake outcome.
    #[must_use]
    pub const fn admission(&self) -> Option<&SpineAdmission> {
        self.admission.as_ref()
    }

    /// Write the run outcome.
    pub fn set_outcome(&mut self, mut record: SpineOutcome) {
        record.grade = self.grade;
        self.outcome = Some(record);
    }

    /// The run outcome.
    #[must_use]
    pub const fn outcome(&self) -> Option<&SpineOutcome> {
        self.outcome.as_ref()
    }

    /// Append a load-plane decision.
    pub fn push_load_decision(&mut self, mut record: SpineLoadDecision) {
        record.grade = self.grade;
        self.load_decisions.push(record);
    }

    /// The load-plane decisions, in mint order.
    #[must_use]
    pub fn load_decisions(&self) -> &[SpineLoadDecision] {
        &self.load_decisions
    }

    /// Write one site's classification.
    pub fn set_classification(&mut self, mut record: SpineSiteClassification) {
        record.grade = self.grade;
        self.classifications.insert(record.site, record);
    }

    /// Every site classification, in site order.
    pub fn classifications(&self) -> impl Iterator<Item = &SpineSiteClassification> {
        self.classifications.values()
    }

    /// Append a solve certification.
    pub fn push_certification(&mut self, mut record: SpineSolveCertification) {
        record.grade = self.grade;
        self.certifications.push(record);
    }

    /// The solve certifications, in pass order.
    #[must_use]
    pub fn certifications(&self) -> &[SpineSolveCertification] {
        &self.certifications
    }

    /// Append a vouch record.
    pub fn push_vouch(&mut self, mut record: SpineVouch) {
        record.grade = self.grade;
        self.vouches.push(record);
    }

    /// The vouch records, in mint order.
    #[must_use]
    pub fn vouches(&self) -> &[SpineVouch] {
        &self.vouches
    }

    /// Write one site's ship decision.
    pub fn set_ship(&mut self, mut record: SpineProbeShip) {
        record.grade = self.grade;
        self.ships.insert(record.site, record);
    }

    /// Every ship decision, in site order.
    pub fn ships(&self) -> impl Iterator<Item = &SpineProbeShip> {
        self.ships.values()
    }

    /// Write one site's observation.
    pub fn set_observation(&mut self, mut record: SpineObservation) {
        record.grade = self.grade;
        self.observations.insert(record.site, record);
    }

    /// Every observation, in site order.
    pub fn observations(&self) -> impl Iterator<Item = &SpineObservation> {
        self.observations.values()
    }

    /// Append a validity round.
    pub fn push_round(&mut self, mut record: SpineValidityRound) {
        record.grade = self.grade;
        self.rounds.push(record);
    }

    /// The validity rounds, in round order.
    #[must_use]
    pub fn rounds(&self) -> &[SpineValidityRound] {
        &self.rounds
    }

    /// Append a survival outcome.
    pub fn push_survival(&mut self, mut record: SpineSurvival) {
        record.grade = self.grade;
        self.survivals.push(record);
    }

    /// The survival outcomes, in mint order.
    #[must_use]
    pub fn survivals(&self) -> &[SpineSurvival] {
        &self.survivals
    }

    /// Append a render-time decision.
    pub fn push_render_decision(&mut self, mut record: SpineRenderDecision) {
        record.grade = self.grade;
        self.render_decisions.push(record);
    }

    /// The render-time decisions, in mint order.
    #[must_use]
    pub fn render_decisions(&self) -> &[SpineRenderDecision] {
        &self.render_decisions
    }

    /// Append narration minted while writing this Spine's decisions.
    ///
    /// Scope, and it is load-bearing for order: this holds the narration a DECISION WRITE minted,
    /// not the run's whole narrative stream. A projection that narrates its own drops returns those
    /// records to its caller instead of pushing them here, so a projection can never retroactively
    /// appear in an account something already read.
    pub fn push_narrative(&mut self, narrative: P::Narrative) {
        self.narratives.push(narrative);
    }

    /// The narration minted alongside these decisions, in mint order (`inv-determinism`).
    #[must_use]
    pub fn narratives(&self) -> &[P::Narrative] {
        &self.narratives
    }

    /// The `new`-arm DEBUG DUMP (`309` §3; `pin-debug-dump-gating`) — project-internal, and
    /// structurally unable to ship.
    ///
    /// The `new` arm is transitory by definition: non-durable in production, but not ruled
    /// non-durable, which is the legal resting state for in-flight work. It still has to be
    /// INSPECTABLE, or "we track it" is a claim nobody can check. This is that inspection.
    ///
    /// # The gating, and why it is a signature rather than a flag
    ///
    /// Following `admit_fixture_records`' shape (`rul-fixture-identity-never-production`: comments
    /// are not a fence — absence of a constructor is), this **cannot name a production sink**: it
    /// takes no path, no writer, no directory, no destination of any kind, and none is addable by a
    /// caller. It hands back a `String` and stops. The half no type can fence — that nobody CALLS it
    /// from a shipping path — is a lexical non-empty-walk gate
    /// (`the_new_arm_debug_dump_has_no_production_caller`), exactly as the fixture-intake gate does.
    ///
    /// Never confuse this with the migration smoke-diff (`309` §4): different mechanism, different
    /// lifetime. That one is build-to-kill scaffolding frozen at one commit; this one lives as long
    /// as the `new` arm does.
    #[must_use]
    pub fn debug_dump(&self) -> String {
        use std::fmt::Write as _;

        let mut out = String::new();
        let _ = writeln!(
            out,
            "dorc-spine-new-arm grade={}",
            if self.grade.is_some() {
                "host-influenced"
            } else {
                "authored-before-contact"
            }
        );
        for species in SpineSpecies::ALL {
            if species.census_arm() != CensusArm::New {
                continue;
            }
            let _ = writeln!(out, "{} n={}", species.name(), self.population(species));
        }
        for record in &self.load_decisions {
            let _ = writeln!(out, "  load {} withheld={:?}", record.name, record.withheld);
        }
        for record in self.classifications.values() {
            let _ = writeln!(
                out,
                "  classify {:?} class={} verdict-lane={} invalidator={} cells={}",
                record.site,
                record.class,
                record.verdict_lane,
                record.invalidator,
                record.cells.total()
            );
        }
        for record in &self.certifications {
            let _ = writeln!(
                out,
                "  certify {} consistent={} tripped={}",
                record.pass, record.consistent, record.tripped
            );
        }
        for record in &self.vouches {
            let _ = writeln!(
                out,
                "  vouch {:?} attached={} custody={:?}",
                record.site, record.attached, record.custody
            );
        }
        for record in self.ships.values() {
            let _ = writeln!(out, "  ship {:?} lane={:?}", record.site, record.lane);
        }
        if let Some(record) = &self.admission {
            let _ = writeln!(
                out,
                "  admission {:?} fault={:?}",
                record.outcome, record.fault
            );
        }
        for record in self.observations.values() {
            let _ = writeln!(
                out,
                "  observe {:?} verdict={} collapsed={}",
                record.site, record.verdict, record.collapsed
            );
        }
        for record in &self.rounds {
            let _ = writeln!(
                out,
                "  round {} erased={}",
                record.round,
                record.erased.total()
            );
        }
        for record in &self.survivals {
            let _ = writeln!(out, "  survival {:?} {:?}", record.leaf, record.outcome);
        }
        for record in &self.render_decisions {
            let _ = writeln!(out, "  render {:?} {:?}", record.site, record.decision);
        }
        if let Some(record) = &self.outcome {
            let _ = writeln!(
                out,
                "  outcome {} advisory={} durable-eligible={}",
                record.outcome, record.advisory, record.durable_eligible
            );
        }
        out
    }

    /// How many records of each species this Spine holds — the account a projection reports what it
    /// dropped against (`309:rul-drop-accounting-completes-the-narrative-law`).
    #[must_use]
    pub fn population(&self, species: SpineSpecies) -> u32 {
        let count = match species {
            SpineSpecies::Invocation => usize::from(self.invocation.is_some()),
            SpineSpecies::RecordStream => usize::from(self.record_stream.is_some()),
            SpineSpecies::Disposition => self.dispositions.len(),
            SpineSpecies::Digest => usize::from(self.digest.is_some()),
            SpineSpecies::LoadDecision => self.load_decisions.len(),
            SpineSpecies::SiteClassification => self.classifications.len(),
            SpineSpecies::SolveCertification => self.certifications.len(),
            SpineSpecies::Vouch => self.vouches.len(),
            SpineSpecies::ProbeShip => self.ships.len(),
            SpineSpecies::Admission => usize::from(self.admission.is_some()),
            SpineSpecies::Observation => self.observations.len(),
            SpineSpecies::ValidityRound => self.rounds.len(),
            SpineSpecies::Survival => self.survivals.len(),
            SpineSpecies::RenderDecision => self.render_decisions.len(),
            SpineSpecies::Outcome => usize::from(self.outcome.is_some()),
        };
        u32::try_from(count).unwrap_or(u32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The plane the unit tests instantiate: the payload types are opaque here, which is exactly
    /// the property that keeps `core` free of `plan`.
    #[derive(Debug)]
    struct TestPlane;

    impl DecidePlane for TestPlane {
        type Decision = &'static str;
        type Records = ();
        type Narrative = ();
    }

    #[test]
    fn the_census_classifies_every_species_and_the_durable_arm_holds_exactly_four() {
        // `309:mech-census-three-states`. The counts are the tripwire in its cheapest form: a
        // species ENTERING the durable arm moves this number, and that is a reviewable diff rather
        // than a silent widening of what reaches operator disk.
        let durable = SpineSpecies::ALL
            .iter()
            .filter(|s| s.census_arm() == CensusArm::Durable)
            .count();
        let excluded = SpineSpecies::ALL
            .iter()
            .filter(|s| s.census_arm() == CensusArm::Excluded)
            .count();
        let new = SpineSpecies::ALL
            .iter()
            .filter(|s| s.census_arm() == CensusArm::New)
            .count();
        assert_eq!(durable, 4, "`30E` §2: the durable arm is four species");
        assert_eq!(
            excluded, 0,
            "`30E` §2's exclusions are content-tier; a species ruled non-durable would land here"
        );
        assert_eq!(new, 11, "`30E` §2: eleven transitory species");
    }

    #[test]
    fn species_names_are_distinct_so_the_debug_dump_cannot_alias_two() {
        let mut names: Vec<&str> = SpineSpecies::ALL.iter().map(|s| s.name()).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len(), "two species share one dump label");
    }

    #[test]
    fn an_operand_account_reports_what_it_dropped_rather_than_truncating_silently() {
        // `law-spine-operands-capped`: the count is part of the type, so a reader can always tell a
        // short list from a clipped one.
        let cap = u32::try_from(SPINE_OPERAND_CAP).expect("the cap is a small constant");
        let account = Account::capped(0..20u32);
        assert_eq!(account.shown().len(), SPINE_OPERAND_CAP);
        assert_eq!(account.dropped(), 20 - cap);
        assert_eq!(account.total(), 20);
    }

    #[test]
    fn dispositions_are_site_keyed_so_two_members_of_one_leaf_never_collapse() {
        // `inv-site-keyed-results` at the Spine tier: the member index is part of the key, which is
        // the fix `30E:stop-siteid-digest-rekey` makes in memory (the durable keeps `leaf: u32`).
        let mut spine = Spine::<TestPlane>::new();
        let leaf = LeafId(3);
        for member in [None, Some(0), Some(1)] {
            spine.set_disposition(SpineDisposition {
                site: SiteId { leaf, member },
                ast: AstId(9),
                sh: String::from("apt-get install nginx"),
                decision: "Run",
                grade: None,
            });
        }
        assert_eq!(spine.population(SpineSpecies::Disposition), 3);
        assert!(
            spine
                .disposition(SiteId {
                    leaf,
                    member: Some(1)
                })
                .is_some()
        );
    }

    #[test]
    fn the_spine_stamps_the_grade_so_a_mint_site_cannot_forget_it() {
        // `309` §2 / `306c` §2: v0's flip is positional and global, so the grade belongs to the
        // Spine a run builds. A mint site passing `None` — which every one of them does — still
        // lands host-influenced on an influenced Spine, which is the property that makes a NEW mint
        // site correct by construction rather than by review.
        let phase =
            crate::influence::Influenced::<crate::influence::HostReported, ()>::host_reported(())
                .widen();
        let mut spine = Spine::<TestPlane>::minted_at(Some(phase));
        spine.set_disposition(SpineDisposition {
            site: SiteId::leaf(LeafId(0)),
            ast: AstId(0),
            sh: String::new(),
            decision: "Run",
            grade: None,
        });
        assert_eq!(
            spine
                .disposition(SiteId::leaf(LeafId(0)))
                .and_then(|record| record.grade),
            Some(phase),
            "the record wears the Spine's grade, not the one its mint site typed"
        );
        assert_eq!(
            Spine::<TestPlane>::new().grade(),
            None,
            "an intakeless Spine stays authored-before-contact"
        );
    }

    #[test]
    fn population_answers_zero_for_every_species_of_an_empty_spine() {
        // The drop-accounting rests on this: a projection reports what it dropped against the
        // population, so an unpopulated species must answer 0 rather than be unaskable.
        let spine = Spine::<TestPlane>::new();
        for species in SpineSpecies::ALL {
            assert_eq!(spine.population(species), 0, "{}", species.name());
        }
    }
}
