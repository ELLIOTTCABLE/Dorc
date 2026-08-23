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

use crate::influence::InfluenceAccount;
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
    /// The license-bearing decision one AUTHORED REGION reached (`plan::Disposition` at the one
    /// instantiation).
    ///
    /// Its own associated type rather than a reuse of [`Decision`](Self::Decision), because the two
    /// are keyed by different identities and `30L:rul-two-identities-never-conflated` is the point:
    /// a `Decision` keys by `SiteId` (execution), a region decision by `ElisionRegion` (edit). They
    /// happen to instantiate to one enum today; a seam that let one be handed where the other was
    /// expected would make that coincidence load-bearing.
    type RegionDecision: core::fmt::Debug + Clone;
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
///
/// Named `Operand` rather than bare `Account` so it can never read as a sibling of
/// [`InfluenceAccount`](crate::influence::InfluenceAccount), which every record also carries and
/// which answers an unrelated question. This one is WHAT a record's inputs were; that one is where
/// they stand relative to host contact.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OperandAccount<T> {
    shown: Vec<T>,
    dropped: u32,
}

impl<T> OperandAccount<T> {
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

/// The sealing module: [`InfluenceBearing`]'s supertrait is private, so the contract can be
/// implemented only inside this module — which is where the discipline is visible.
mod sealed {
    pub trait Sealed {}
}

/// The contract `306b:rul-consequential-sinks-require-influence` asks for: every stable Spine
/// record answers where it stands relative to host contact, and answers it from the account its
/// OWN semantic mint joined.
///
/// **Sealed.** A new species cannot land carrying no account, because it cannot be constructed
/// (private fields), cannot be stored (the setters take sealed records), and cannot be classified
/// (the no-wildcard [`SpineSpecies::account_carriage`] census stops compiling until it is).
pub trait InfluenceBearing: sealed::Sealed {
    /// Where this record stands. Immutable: there is no setter and no `&mut` route, here or on
    /// [`Spine`] (`309:rul-spine-preserves-never-stamps`).
    fn account(&self) -> InfluenceAccount;
}

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
    /// One authored elision region's shared decision and its contributing route attribution.
    RegionDecision,
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

/// How a species' writer arrives at the account its records carry
/// (`306b:rul-semantic-mints-join-influence`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AccountCarriage {
    /// The writer JOINS the accounts of the inputs the record was derived from. The target state
    /// for every species that has a writer at all.
    Joined,
    /// The writer carries an explicit [`InfluenceAccount::untracked`] because some contributor is
    /// unconverted or unenumerable — a STAGED SEAM (`306b:rul-untracked-is-not-authored`), never an
    /// absence. Empty today, and growth here is the signal the discipline exists to watch.
    UntrackedAdapter,
    /// The species has no writer, so there is no population whose account could have been joined
    /// (`core/CLAUDE.md a-record-says-what-its-population-holds`: an unminted species says so at
    /// the type). Classifying these `Joined` would be a claim about a mint that does not exist.
    Unminted,
}

impl AccountCarriage {
    /// Every carriage class, for the census instrument.
    pub const ALL: [Self; 3] = [Self::Joined, Self::UntrackedAdapter, Self::Unminted];
}

impl SpineSpecies {
    /// Every species, in declaration order. The array is what makes the census walkable; the
    /// no-wildcard match in [`census_arm`](Self::census_arm) is what makes it complete.
    pub const ALL: [Self; 16] = [
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
            // Putting any of a region decision on operator disk ENTERS the durable arm.
            | Self::RegionDecision
            | Self::Outcome => CensusArm::New,
        }
    }

    /// How the species' WRITER arrives at its account — the second no-wildcard census, and the
    /// consumer half `306b:rul-consequential-sinks-require-influence` asks for.
    ///
    /// [`InfluenceBearing`] makes a new species carry an account; this makes it say WHERE that
    /// account came from. The two are different failures: a species can carry an account and still
    /// have been handed a constant nobody derived.
    #[must_use]
    pub const fn account_carriage(self) -> AccountCarriage {
        match self {
            Self::Invocation
            | Self::RecordStream
            | Self::Disposition
            | Self::Digest
            | Self::LoadDecision
            | Self::SiteClassification
            | Self::SolveCertification
            | Self::ProbeShip
            | Self::Admission
            | Self::Survival
            | Self::RenderDecision
            | Self::RegionDecision => AccountCarriage::Joined,
            Self::Vouch | Self::Observation | Self::ValidityRound | Self::Outcome => {
                AccountCarriage::Unminted
            }
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
            Self::RegionDecision => "SpineRegionDecision",
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

/// The controller-minted per-attempt identity of one run.
///
/// Grouped rather than spread across [`SpineInvocation::minted`]'s parameters because it is one
/// thing — the identity the controller owns and a payload frame may only be CHECKED against
/// (`rul-attribution-is-controller-minted`) — and because four more positional parameters of
/// mostly-`String` type is exactly the signature a caller mis-orders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunIdentity {
    /// The controller-minted per-attempt nonce.
    pub nonce: String,
    /// The controller-minted attempt serial.
    pub attempt: u32,
    /// The controller-selected host identity.
    pub host: String,
    /// When the controller started this run, from the edge's injected clock. `None` ⇒ no clock.
    pub started_at: Option<RunInstant>,
}

/// The invocation: controller-minted run identity plus what it was pointed at (`30E` §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineInvocation {
    mode: String,
    argv: Vec<String>,
    book: SourceClaim,
    oracles: Vec<SourceClaim>,
    identity: RunIdentity,
    account: InfluenceAccount,
}

impl SpineInvocation {
    /// Mint the invocation record.
    #[must_use]
    pub fn minted(
        mode: String,
        argv: Vec<String>,
        book: SourceClaim,
        oracles: Vec<SourceClaim>,
        identity: RunIdentity,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            mode,
            argv,
            book,
            oracles,
            identity,
            account,
        }
    }

    /// `plan` / `apply` / `roundtrip` / `probe` / `why`.
    ///
    /// FALSE AS POPULATED, and deliberately unrepaired here: the sole writer hard-codes
    /// `"whylog-replay"` from a seat unreachable on the replay branch, so the field describes
    /// neither producing invocation (`30Mc` F3). The value is DURABLE-persisted and re-ingested on
    /// replay, so correcting it is gated behind `rul-durable-contents-reviewed-before-design`
    /// (`30N` §4's `stop-spine-mode-is-durable`) rather than being a local fix. Do not "tidy" it.
    #[must_use]
    pub fn mode(&self) -> &str {
        &self.mode
    }

    /// The full argv, one word per element.
    #[must_use]
    pub fn argv(&self) -> &[String] {
        &self.argv
    }

    /// The book path and its content digest.
    #[must_use]
    pub const fn book(&self) -> &SourceClaim {
        &self.book
    }

    /// Each oracle path and digest, in load order.
    #[must_use]
    pub fn oracles(&self) -> &[SourceClaim] {
        &self.oracles
    }

    /// The controller-minted run identity.
    #[must_use]
    pub const fn identity(&self) -> &RunIdentity {
        &self.identity
    }
}

/// The admitted host-record stream, held as the plane's own admitted-bytes handle (`30E` §2).
#[derive(Debug, Clone)]
pub struct SpineRecordStream<P: DecidePlane> {
    records: P::Records,
    instants: Vec<(u64, RunInstant)>,
    account: InfluenceAccount,
}

impl<P: DecidePlane> SpineRecordStream<P> {
    /// Mint the record-stream record. Host-influenced by construction: these ARE the host-reported
    /// bytes, so the account its caller joins can only be the intake's own.
    #[must_use]
    pub const fn minted(
        records: P::Records,
        instants: Vec<(u64, RunInstant)>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            records,
            instants,
            account,
        }
    }

    /// The as-received buffer, still wearing its admission.
    #[must_use]
    pub const fn records(&self) -> &P::Records {
        &self.records
    }

    /// When the controller took a record in, by arrival ordinal, ascending
    /// (`28F:rul-probe-instants-host-says-no-times` — controller-minted, always).
    ///
    /// SPARSE, not one-per-record: a run with no clock (`RunClock::Absent`, every loom path) stamps
    /// nothing, so this is EMPTY beside a full record buffer. An ordinal's absence means the
    /// controller took no time, never that no record arrived.
    #[must_use]
    pub fn instants(&self) -> &[(u64, RunInstant)] {
        &self.instants
    }
}

/// One site's licensed decision — the license-bearing record (`30E` §2).
///
/// This is the species that vindicates `DurableView` over a species-arity census (`309` critical-2):
/// the RECORD is `SiteId`-keyed and carries the license, while the VIEW emits a leaf plus a tag.
#[derive(Debug, Clone)]
pub struct SpineDisposition<P: DecidePlane> {
    site: SiteId,
    ast: AstId,
    sh: String,
    decision: P::Decision,
    account: InfluenceAccount,
}

impl<P: DecidePlane> SpineDisposition<P> {
    /// Mint one site's decision record.
    #[must_use]
    pub const fn minted(
        site: SiteId,
        ast: AstId,
        sh: String,
        decision: P::Decision,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            site,
            ast,
            sh,
            decision,
            account,
        }
    }

    /// The fine site key (`inv-site-keyed-results`): `(leaf, member)`, never collapsed.
    ///
    /// AS POPULATED (`30Nd` meaning-audit): the member axis is `None` on every row today — the
    /// settlement decides per LEAF, and a member population arrives with the loop-propagation lane
    /// (`30N` §3's `pin-loop-types-need-no-rekey`). The key is fine-grained so that arrival is a
    /// widening rather than a re-key; it is not evidence that members are being distinguished yet.
    #[must_use]
    pub const fn site(&self) -> SiteId {
        self.site
    }

    /// The source back-map.
    #[must_use]
    pub const fn ast(&self) -> AstId {
        self.ast
    }

    /// The verbatim leaf bytes.
    #[must_use]
    pub fn sh(&self) -> &str {
        &self.sh
    }

    /// The license-bearing decision.
    #[must_use]
    pub const fn decision(&self) -> &P::Decision {
        &self.decision
    }

    fn demote(&mut self, decision: P::Decision, witness: InfluenceAccount) {
        self.decision = decision;
        self.account = self.account.join(witness);
    }

    fn reattach(&mut self, decision: P::Decision) {
        self.decision = decision;
    }
}

/// The decision digest over the identity plane (`22A` concl-3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineDigest {
    digest: String,
    account: InfluenceAccount,
}

impl SpineDigest {
    /// Mint the decision digest record.
    #[must_use]
    pub const fn minted(digest: String, account: InfluenceAccount) -> Self {
        Self { digest, account }
    }

    /// The 16-hex-char FNV-1a digest.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// A definition-plane decision: which body a role name binds to, and why a family was withheld.
///
/// AS POPULATED, narrower than the species name suggests (`30Nd` meaning-audit): the only writer
/// records WITHHOLDINGS, so an ordinary binding — the "which body a role name binds to" half —
/// reaches no record at all, and [`withheld`](Self::withheld) is `Some` on every row that exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineLoadDecision {
    name: String,
    custody: Option<DefinitionCustody>,
    withheld: Option<WithheldCause>,
    account: InfluenceAccount,
}

impl SpineLoadDecision {
    /// Mint one definition-plane decision.
    #[must_use]
    pub const fn minted(
        name: String,
        custody: Option<DefinitionCustody>,
        withheld: Option<WithheldCause>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            name,
            custody,
            withheld,
            account,
        }
    }

    /// What the withholding is keyed by, and it is NOT one kind of thing: the `Contested` arm
    /// carries a munged role-family base, while the `Unprovable` arm carries a synthetic
    /// `load@<ast-id>` locator for an unresolvable `.` — an unresolvable load has no name to blame.
    /// Display and provenance only; nothing keys a decision off it.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whose utterance the binding rests on. Compared, never read for its file id
    /// (`custody-is-one-newtype-and-one-crossing`).
    ///
    /// UNIVERSALLY `None` today — the custody column is the unbuilt half of this species
    /// (`30F` §4.5). Read it as "not recorded", never as "no custody".
    #[must_use]
    pub const fn custody(&self) -> Option<DefinitionCustody> {
        self.custody
    }

    /// Why the family's licenses are withheld. `Some` on every recorded row, per the type doc.
    #[must_use]
    pub const fn withheld(&self) -> Option<WithheldCause> {
        self.withheld
    }
}

/// Why a role family's licenses are withheld for a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WithheldCause {
    /// One unit's definition provably shadowed a different unit's (`28K` §1).
    Contested,
    /// The name's exit binding is ⊤, so it licenses nothing (`top-licenses-nothing`).
    Unprovable,
    /// Two loaded sources declared the same helper name (`helper-conflicts-report-at-the-load-edge`).
    ///
    /// NO WRITER: helper conflicts are reported at the load edge as diagnostics and reach no Spine
    /// record, so this arm is representation the population does not yet occupy.
    HelperConflict,
}

/// One site's classification outcome — the analysis tuple, as an account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineSiteClassification {
    site: SiteId,
    class: &'static str,
    verdict_lane: bool,
    invalidator: bool,
    cells: OperandAccount<FactKey>,
    account: InfluenceAccount,
}

impl SpineSiteClassification {
    /// Mint one site's classification record.
    #[must_use]
    pub const fn minted(
        site: SiteId,
        class: &'static str,
        verdict_lane: bool,
        invalidator: bool,
        cells: OperandAccount<FactKey>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            site,
            class,
            verdict_lane,
            invalidator,
            cells,
            account,
        }
    }

    /// The fine site key.
    #[must_use]
    pub const fn site(&self) -> SiteId {
        self.site
    }

    /// The `SkipClass` discriminant name (referent-agnostic: a label, never branched on here).
    #[must_use]
    pub const fn class(&self) -> &'static str {
        self.class
    }

    /// Whether the site is verdict-lane (`verdict-lane-is-site-keyed`).
    #[must_use]
    pub const fn verdict_lane(&self) -> bool {
        self.verdict_lane
    }

    /// Whether this LEAF gens into reach as an invalidator — leaf-scoped, the narrower truth on
    /// purpose. The effective set also holds non-leaves (a `$( … )` body command, a write-shaped
    /// redirection, an unmodeled construct — `classify-answers-with-its-invalidators`) which have
    /// no site to be keyed by, so a `false` here never means "nothing gens at this position".
    /// Widening the record to carry them is a representation question, not a fix.
    #[must_use]
    pub const fn invalidator(&self) -> bool {
        self.invalidator
    }

    /// The cells this site's decision keys on, capped. For an aggregate that is its ORDERED member
    /// account, not a representative (`aggregate-mints-carry-the-same-demand`).
    #[must_use]
    pub const fn cells(&self) -> &OperandAccount<FactKey> {
        &self.cells
    }
}

/// One certification outcome (`plans/302`).
///
/// AS POPULATED (`30Nd` meaning-audit): the sole production writer emits ONE `whole-window` row per
/// run, derived from the run-wide latch — not one row per solve pass. Whether it should be per-pass
/// is a pending human direction (`30M:ask-certification-row-shape`); the fields below say what the
/// row means TODAY rather than what a per-pass row would.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineSolveCertification {
    pass: &'static str,
    consistent: bool,
    tripped: bool,
    account: InfluenceAccount,
}

impl SpineSolveCertification {
    /// Mint one certification record.
    #[must_use]
    pub const fn minted(
        pass: &'static str,
        consistent: bool,
        tripped: bool,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            pass,
            consistent,
            tripped,
            account,
        }
    }

    /// What this row's answer is ABOUT. The one production value is `"whole-window"`; the per-pass
    /// vocabulary (value · funcenv · reach · self-reach) is where the pending direction would take
    /// it. Referent-agnostic: a label, never branched on.
    #[must_use]
    pub const fn pass(&self) -> &'static str {
        self.pass
    }

    /// Whether the answer certified. `false` ⇒ the whole analysis window demoted to its floor.
    ///
    /// On a whole-window row this is exactly `!tripped` — one bit spelled twice. Under a per-pass
    /// row the two separate (a pass may certify on a spine that already tripped), which is why both
    /// fields exist rather than one.
    #[must_use]
    pub const fn consistent(&self) -> bool {
        self.consistent
    }

    /// Whether the monotone trip latch is set at this point.
    #[must_use]
    pub const fn tripped(&self) -> bool {
        self.tripped
    }
}

/// A vouch's attachment or suspension at one site (`rul-vouch-is-verdict-authoring`).
///
/// NOT MINTED (`30F` §4.5): the `Vouches` map exposes no iteration, so every field below describes
/// an empty population. The species is classified so the census is complete, not because anything
/// has been recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineVouch {
    site: SiteId,
    fact: FactKey,
    custody: Option<DefinitionCustody>,
    attached: bool,
    account: InfluenceAccount,
}

impl SpineVouch {
    /// Mint one vouch record.
    #[must_use]
    pub const fn minted(
        site: SiteId,
        fact: FactKey,
        custody: Option<DefinitionCustody>,
        attached: bool,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            site,
            fact,
            custody,
            attached,
            account,
        }
    }

    /// The fine site key.
    #[must_use]
    pub const fn site(&self) -> SiteId {
        self.site
    }

    /// The cell the vouch answers about.
    #[must_use]
    pub const fn fact(&self) -> FactKey {
        self.fact
    }

    /// Whose utterance it is.
    #[must_use]
    pub const fn custody(&self) -> Option<DefinitionCustody> {
        self.custody
    }

    /// `false` ⇒ suspended: the composition that will run is not the region its author vouched.
    #[must_use]
    pub const fn attached(&self) -> bool {
        self.attached
    }
}

/// Which body a probe site shipped, or why none could be (`ship-seam-reads-the-lane-not-the-kind`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineProbeShip {
    site: SiteId,
    lane: ShipLane,
    defining_file: Option<SourceFileId>,
    account: InfluenceAccount,
}

impl SpineProbeShip {
    /// Mint one site's ship record.
    #[must_use]
    pub const fn minted(
        site: SiteId,
        lane: ShipLane,
        defining_file: Option<SourceFileId>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            site,
            lane,
            defining_file,
            account,
        }
    }

    /// The fine site key.
    #[must_use]
    pub const fn site(&self) -> SiteId {
        self.site
    }

    /// Which lane shipped.
    #[must_use]
    pub const fn lane(&self) -> ShipLane {
        self.lane
    }

    /// The defining file of the shipped body, for provenance and display only. `None` where the
    /// ship seat resolved no defining span, and always `None` on an [`ShipLane::Unresolvable`] row
    /// — nothing shipped, so there is no body to attribute.
    #[must_use]
    pub const fn defining_file(&self) -> Option<SourceFileId> {
        self.defining_file
    }
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
///
/// AS POPULATED (`30Nd` meaning-audit): the recording seat runs AFTER the refusal path has already
/// returned (`rul-integrity-failure-withholds-mutation` — a refusal emits no plan and never reaches
/// here), so only the two authority-carrying arms are ever written. A run that refused has no
/// admission record at all rather than a `Refused` one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineAdmission {
    outcome: AdmissionOutcome,
    fault: Option<String>,
    account: InfluenceAccount,
}

impl SpineAdmission {
    /// Mint the intake-outcome record. Host-influenced once anything was read.
    #[must_use]
    pub const fn minted(
        outcome: AdmissionOutcome,
        fault: Option<String>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            outcome,
            fault,
            account,
        }
    }

    /// Which of the three the intake answered. `Refused` is representable and unreachable at the
    /// only writer, per the type doc.
    #[must_use]
    pub const fn outcome(&self) -> AdmissionOutcome {
        self.outcome
    }

    /// The named condition on a refusal, for attribution. UNIVERSALLY `None`, because the arm that
    /// would carry one never reaches this record.
    #[must_use]
    pub fn fault(&self) -> Option<&str> {
        self.fault.as_deref()
    }
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
///
/// NOT MINTED (`30F` §4.5): the `by_fact` merge is consumed by closure rather than by collection,
/// so every field below describes an empty population.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineObservation {
    site: SiteId,
    fact: FactKey,
    verdict: &'static str,
    collapsed: bool,
    account: InfluenceAccount,
}

impl SpineObservation {
    /// Mint one site's observation record.
    #[must_use]
    pub const fn minted(
        site: SiteId,
        fact: FactKey,
        verdict: &'static str,
        collapsed: bool,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            site,
            fact,
            verdict,
            collapsed,
            account,
        }
    }

    /// The fine site key.
    #[must_use]
    pub const fn site(&self) -> SiteId {
        self.site
    }

    /// The cell measured.
    #[must_use]
    pub const fn fact(&self) -> FactKey {
        self.fact
    }

    /// The Effect-channel verdict label (converged / diverged / unknown).
    #[must_use]
    pub const fn verdict(&self) -> &'static str {
        self.verdict
    }

    /// Whether the merge over same-cell measurements collapsed to ⊤.
    #[must_use]
    pub const fn collapsed(&self) -> bool {
        self.collapsed
    }
}

/// One round of the validity fixpoint (`the-fixpoint-owns-the-rounds-and-builds-nothing-else`).
///
/// NOT MINTED (`30F` §4.5): intermediate rounds are never BUILT, so recording one means first
/// deciding what a never-survives round may leave behind. Every field below describes an empty
/// population.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineValidityRound {
    round: u32,
    erased: OperandAccount<SiteId>,
    account: InfluenceAccount,
}

impl SpineValidityRound {
    /// Mint one validity-round record.
    #[must_use]
    pub const fn minted(
        round: u32,
        erased: OperandAccount<SiteId>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            round,
            erased,
            account,
        }
    }

    /// The round ordinal, from 1.
    #[must_use]
    pub const fn round(&self) -> u32 {
        self.round
    }

    /// The sites this round proved dead and erased, capped.
    #[must_use]
    pub const fn erased(&self) -> &OperandAccount<SiteId> {
        &self.erased
    }
}

/// A survival-tier outcome at one site (`survive-license`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineSurvival {
    leaf: LeafId,
    outcome: SurvivalOutcome,
    poisoned_by: Option<KindId>,
    account: InfluenceAccount,
}

impl SpineSurvival {
    /// Mint one survival-tier record.
    #[must_use]
    pub const fn minted(
        leaf: LeafId,
        outcome: SurvivalOutcome,
        poisoned_by: Option<KindId>,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            leaf,
            outcome,
            poisoned_by,
            account,
        }
    }

    /// The leaf whose elision was tested.
    #[must_use]
    pub const fn leaf(&self) -> LeafId {
        self.leaf
    }

    /// What the wall walk answered.
    #[must_use]
    pub const fn outcome(&self) -> SurvivalOutcome {
        self.outcome
    }

    /// The reach-function kind that poisoned it, where one did.
    #[must_use]
    pub const fn poisoned_by(&self) -> Option<KindId> {
        self.poisoned_by
    }
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
    /// A reach solve failed its own post-fixpoint check (`302:rul-whole-window-demotion`). Distinct
    /// from `TotalWall`: a finding about OUR solver, not a claim about the book's mutators
    /// (`302` §5 admin-honesty · `271:rul-sin-ordering`).
    SolveInconsistent,
}

/// A render-time decision, hoisted out of hiding (`30E` §3's audit).
///
/// Every one of these is license-relevant and was, before the reification, made inside the render
/// with only a diagnostic between it and the structured decision plane. Recording them here is what
/// lets a projection be compared against what was actually decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineRenderDecision {
    site: Option<SiteId>,
    region: Option<crate::region::ElisionRegion>,
    decision: RenderDecision,
    account: InfluenceAccount,
}

impl SpineRenderDecision {
    /// Mint one render-time decision record.
    #[must_use]
    pub const fn minted(
        site: Option<SiteId>,
        region: Option<crate::region::ElisionRegion>,
        decision: RenderDecision,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            site,
            region,
            decision,
            account,
        }
    }

    /// The site the decision belongs to, where it has one.
    #[must_use]
    pub const fn site(&self) -> Option<SiteId> {
        self.site
    }

    /// The authored REGION the decision belongs to, where it has one
    /// (`30N:rul-region-refusal-discloses-region-keyed`).
    ///
    /// A SECOND key axis rather than a widening of `site`, on `SpineRegionDecision`'s precedent: a
    /// region owns no execution, so a row that keyed it by a contributing invocation's `SiteId`
    /// would be the smearing the ruling forbids. At most one axis is populated on any row.
    #[must_use]
    pub const fn region(&self) -> Option<crate::region::ElisionRegion> {
        self.region
    }

    /// Which render-time decision this is.
    #[must_use]
    pub const fn decision(&self) -> &RenderDecision {
        &self.decision
    }
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
    /// `dec-import-rewrite` — a GENERATED plan's import line now names the bundle this run composed,
    /// or the bundle's own bytes stand where it did
    /// (`30Ng:rul-bundle-at-dorc-lang-boundaries`, human-typed).
    ///
    /// A recorded decision rather than an emission-time substitution for the same reason every other
    /// arm here is one: it changes what the artifact does, and a change nothing wrote down cannot be
    /// accounted for by a projection, a why report, or a second artifact form.
    ImportRewritten {
        /// The closed verb word: `repointed` or `inlined`.
        verb: &'static str,
        /// The artifact-relative path the import names, empty where nothing is named any more.
        names: String,
    },
}

/// Why a span render refused a licensed decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefusalCause {
    /// The leaf's span covers `<<EOF` rather than the body, so it cannot be safely edited.
    Heredoc,
    /// A guard would sit in front of a blocking output redirect.
    BlockingRedirect,
}

/// One contributing route of a shared region decision: which invocation it executes under, on both
/// identities the two surfaces need.
///
/// `invocation` is the site key a why report walks call-ward; `ast` is the same call's source
/// back-map, which is what the RENDER asks when it wants to know whether that call still executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionRoute {
    /// The invocation's plan site.
    pub invocation: SiteId,
    /// The invocation's source back-map.
    pub ast: AstId,
}

/// Why a contributing route carries no plan-site identity.
///
/// A typed reason rather than absence, because absence was how the whole route went missing: the
/// round used to `filter_map` an unkeyable invocation away, leaving an account that read as complete
/// and was not (`30Ng` §2's entire-DAG directive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegionRouteUnkeyed {
    /// The invocation is not itself a plan leaf, so this round minted no `SiteId` for it. The call
    /// still executes the shared edit, and its source back-map still names it.
    NoPlanLeaf,
}

/// A contributing route the round could not key to a plan site — RETAINED, with the identity it
/// does have and the typed reason it lacks the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnkeyedRegionRoute {
    /// The invocation's source back-map — enough for every surface that answers by LINE.
    pub ast: AstId,
    /// Why no site identity exists for it.
    pub reason: RegionRouteUnkeyed,
}

/// Every contributing route of one shared region decision, COMPLETE (`30Ng` §2, human-typed: the
/// narrative must carry the entire DAG of causative contributors, not a sample).
///
/// Deliberately NOT an [`OperandAccount`], and the carve is narrow and reasoned. The cap
/// (`309:law-spine-operands-capped`) bounds an operand list whose length is a property of the WORLD;
/// a region's contributor population is a property of the analysed unit — bounded by the census,
/// which is bounded by `cfg::inline_budget`'s per-book node budget — and it is the answer two pull
/// surfaces ask for by name. A sampled contributor set would point a reader at some of the calls that
/// share an edit and silently omit the rest, which is the mis-attribution direction
/// (`271:rul-sin-ordering`). This species is transitory (never durable), so nothing here reaches
/// operator disk.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegionRoutes {
    keyed: Vec<RegionRoute>,
    unkeyed: Vec<UnkeyedRegionRoute>,
}

impl RegionRoutes {
    /// The complete population, split by whether the round could key it.
    #[must_use]
    pub fn of(keyed: Vec<RegionRoute>, unkeyed: Vec<UnkeyedRegionRoute>) -> Self {
        Self { keyed, unkeyed }
    }

    /// The routes carrying a plan-site identity, in census order.
    #[must_use]
    pub fn keyed(&self) -> &[RegionRoute] {
        &self.keyed
    }

    /// The routes carrying only a source back-map, in census order.
    #[must_use]
    pub fn unkeyed(&self) -> &[UnkeyedRegionRoute] {
        &self.unkeyed
    }

    /// Every contributing invocation's source back-map, keyed or not — what a surface answering by
    /// LINE walks, and the reason an unkeyable route is retained rather than filtered.
    pub fn asts(&self) -> impl Iterator<Item = AstId> + '_ {
        self.keyed
            .iter()
            .map(|route| route.ast)
            .chain(self.unkeyed.iter().map(|route| route.ast))
    }

    /// How many invocations share this edit.
    #[must_use]
    pub fn total(&self) -> usize {
        self.keyed.len().saturating_add(self.unkeyed.len())
    }

    /// Did every contributing route key to a plan site? `false` is the conservative trigger: a
    /// consumer that has to ask something of every invocation cannot ask it of one with no site.
    #[must_use]
    pub fn every_route_is_keyed(&self) -> bool {
        self.unkeyed.is_empty()
    }

    /// Did the census find no contributing route at all?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
}

/// One AUTHORED ELISION REGION's shared decision (`plans/30L` §9).
///
/// Keyed by [`ElisionRegion`](crate::region::ElisionRegion) rather than by
/// [`SiteId`](crate::SiteId), and that is the species' reason for existing: a region has MANY
/// executions and exactly ONE edit, so a leaf-keyed record cannot hold it without either collapsing
/// the instances or inventing a leaf the edit does not have
/// (`30L:rul-two-identities-never-conflated`; `spike/CLAUDE.md inv-leaf-seam`).
///
/// `routes` is the attribution that makes `dorc why` bidirectional: definition region → the
/// invocations that licensed this edit, and (read backwards) call instance → the shared edits it
/// executes.
#[derive(Debug, Clone)]
pub struct SpineRegionDecision<P: DecidePlane> {
    region: crate::region::ElisionRegion,
    ast: AstId,
    sh: String,
    decision: P::RegionDecision,
    routes: RegionRoutes,
    account: InfluenceAccount,
}

impl<P: DecidePlane> SpineRegionDecision<P> {
    /// Mint one authored region's shared-decision record.
    #[must_use]
    pub const fn minted(
        region: crate::region::ElisionRegion,
        ast: AstId,
        sh: String,
        decision: P::RegionDecision,
        routes: RegionRoutes,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            region,
            ast,
            sh,
            decision,
            routes,
            account,
        }
    }

    /// The authored span all instances would edit.
    #[must_use]
    pub const fn region(&self) -> crate::region::ElisionRegion {
        self.region
    }

    /// The source back-map for that span.
    #[must_use]
    pub const fn ast(&self) -> AstId {
        self.ast
    }

    /// The verbatim region bytes.
    #[must_use]
    pub fn sh(&self) -> &str {
        &self.sh
    }

    /// The one shared, license-bearing decision.
    #[must_use]
    pub const fn decision(&self) -> &P::RegionDecision {
        &self.decision
    }

    /// Which invocation each contributing route executes under, in census order — COMPLETE, with an
    /// unkeyable one retained under its typed reason rather than filtered away.
    #[must_use]
    pub const fn routes(&self) -> &RegionRoutes {
        &self.routes
    }

    fn demote(&mut self, decision: P::RegionDecision, witness: InfluenceAccount) {
        self.decision = decision;
        self.account = self.account.join(witness);
    }
}

/// The run's outcome — authority-adjacent, because `EXIT_BOOK_UNMODELED` exists precisely so a
/// `dorc … && deploy` chain STOPS (`30E` §4).
///
/// NOT MINTED (`30Nd` meaning-audit; the fifth unminted species, where `30F` §4.5 disclosed four):
/// its seat is the cli driver's exit-code computation, which runs after every projection and owns
/// no Spine at that point. Recording it means deciding what an outcome record means for a run that
/// refused before planning — the same question the admission species answers by absence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpineOutcome {
    outcome: &'static str,
    advisory: bool,
    durable_eligible: bool,
    account: InfluenceAccount,
}

impl SpineOutcome {
    /// Mint the run-outcome record.
    #[must_use]
    pub const fn minted(
        outcome: &'static str,
        advisory: bool,
        durable_eligible: bool,
        account: InfluenceAccount,
    ) -> Self {
        Self {
            outcome,
            advisory,
            durable_eligible,
            account,
        }
    }

    /// The outcome discriminant's name.
    #[must_use]
    pub const fn outcome(&self) -> &'static str {
        self.outcome
    }

    /// Whether advisory (render-plane) disclosure was routed for this run.
    #[must_use]
    pub const fn advisory(&self) -> bool {
        self.advisory
    }

    /// Whether the run was eligible to write a durable.
    #[must_use]
    pub const fn durable_eligible(&self) -> bool {
        self.durable_eligible
    }
}

// ===========================================================================
// The sealed carriage contract, one impl per species
// ===========================================================================

// Sixteen hand-written blocks rather than a macro (authored macros are banned): a new species is
// covered by nothing generic, so it must be written here, beside the census that refuses it.
impl sealed::Sealed for SpineInvocation {}
impl InfluenceBearing for SpineInvocation {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl<P: DecidePlane> sealed::Sealed for SpineRecordStream<P> {}
impl<P: DecidePlane> InfluenceBearing for SpineRecordStream<P> {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl<P: DecidePlane> sealed::Sealed for SpineDisposition<P> {}
impl<P: DecidePlane> InfluenceBearing for SpineDisposition<P> {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineDigest {}
impl InfluenceBearing for SpineDigest {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineLoadDecision {}
impl InfluenceBearing for SpineLoadDecision {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineSiteClassification {}
impl InfluenceBearing for SpineSiteClassification {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineSolveCertification {}
impl InfluenceBearing for SpineSolveCertification {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineVouch {}
impl InfluenceBearing for SpineVouch {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineProbeShip {}
impl InfluenceBearing for SpineProbeShip {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineAdmission {}
impl InfluenceBearing for SpineAdmission {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineObservation {}
impl InfluenceBearing for SpineObservation {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineValidityRound {}
impl InfluenceBearing for SpineValidityRound {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineSurvival {}
impl InfluenceBearing for SpineSurvival {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineRenderDecision {}
impl InfluenceBearing for SpineRenderDecision {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl<P: DecidePlane> sealed::Sealed for SpineRegionDecision<P> {}
impl<P: DecidePlane> InfluenceBearing for SpineRegionDecision<P> {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
}

impl sealed::Sealed for SpineOutcome {}
impl InfluenceBearing for SpineOutcome {
    fn account(&self) -> InfluenceAccount {
        self.account
    }
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
    region_decisions: Vec<SpineRegionDecision<P>>,
    outcome: Option<SpineOutcome>,
    narratives: Vec<P::Narrative>,
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
            region_decisions: Vec::new(),
            outcome: None,
            narratives: Vec::new(),
        }
    }
}

impl<P: DecidePlane> Spine<P> {
    /// An empty Spine over material that exists before the first host exchange.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Write the invocation record.
    pub fn set_invocation(&mut self, record: SpineInvocation) {
        self.invocation = Some(record);
    }

    /// The invocation record, if the run reached the point of minting one.
    #[must_use]
    pub const fn invocation(&self) -> Option<&SpineInvocation> {
        self.invocation.as_ref()
    }

    /// Write the admitted record stream.
    pub fn set_record_stream(&mut self, record: SpineRecordStream<P>) {
        self.record_stream = Some(record);
    }

    /// The admitted record stream, if any was admitted.
    #[must_use]
    pub const fn record_stream(&self) -> Option<&SpineRecordStream<P>> {
        self.record_stream.as_ref()
    }

    /// Write one site's licensed decision.
    pub fn set_disposition(&mut self, record: SpineDisposition<P>) {
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

    /// DEMOTE every site decision `stands` refuses, re-minting each demoted record's account as
    /// the join of the demotion's `witness` and what the record already carried. Answers the
    /// demoted sites, in site order.
    ///
    /// A named act rather than a `&mut` accessor (`tc-spine-record-mut-accessors-survive`, ruled):
    /// a post-construction rewrite IS a new semantic mint, whose inputs are the witness and the
    /// original record, so its account is the JOIN of both — never a reset, and never a
    /// pass-through that ignores the witness. Handing out `&mut` made "join, don't reset" a rule a
    /// caller had to remember; this makes the join the only thing a caller CAN do. At v0 both
    /// operands carry the same value and the join is a no-op — the SHAPE is what lands.
    pub fn demote_dispositions(
        &mut self,
        witness: InfluenceAccount,
        stands: impl Fn(&P::Decision) -> bool,
        demoted: &P::Decision,
    ) -> Vec<SiteId> {
        let mut sites = Vec::new();
        for record in self.dispositions.values_mut() {
            if stands(&record.decision) {
                continue;
            }
            record.demote(demoted.clone(), witness);
            sites.push(record.site);
        }
        sites
    }

    /// Rewrite every site decision through `attach`, leaving every account UNTOUCHED.
    ///
    /// `fnd-provenance-attach-raises-nothing`: the one caller attaches EXEMPT output-only probe
    /// provenance for the very fact the license already decided on, so a `Replace`/`Guard` record's
    /// account absorbed that measurement at its own mint and the `Run`/`Omit` arms are no-ops.
    /// Stated as a property rather than defended by a re-join — a join here would claim a new input
    /// where there is none, which is its own kind of dishonesty about what a record read.
    pub fn reattach_dispositions(
        &mut self,
        mut attach: impl FnMut(AstId, P::Decision) -> P::Decision,
    ) {
        for record in self.dispositions.values_mut() {
            let ast = record.ast;
            let decision = attach(ast, record.decision.clone());
            record.reattach(decision);
        }
    }

    /// Write the decision digest.
    pub fn set_digest(&mut self, record: SpineDigest) {
        self.digest = Some(record);
    }

    /// The decision digest.
    #[must_use]
    pub const fn digest(&self) -> Option<&SpineDigest> {
        self.digest.as_ref()
    }

    /// Write the intake outcome.
    pub fn set_admission(&mut self, record: SpineAdmission) {
        self.admission = Some(record);
    }

    /// The intake outcome.
    #[must_use]
    pub const fn admission(&self) -> Option<&SpineAdmission> {
        self.admission.as_ref()
    }

    /// Write the run outcome.
    pub fn set_outcome(&mut self, record: SpineOutcome) {
        self.outcome = Some(record);
    }

    /// The run outcome.
    #[must_use]
    pub const fn outcome(&self) -> Option<&SpineOutcome> {
        self.outcome.as_ref()
    }

    /// Append a load-plane decision.
    pub fn push_load_decision(&mut self, record: SpineLoadDecision) {
        self.load_decisions.push(record);
    }

    /// The load-plane decisions, in mint order.
    #[must_use]
    pub fn load_decisions(&self) -> &[SpineLoadDecision] {
        &self.load_decisions
    }

    /// Write one site's classification.
    pub fn set_classification(&mut self, record: SpineSiteClassification) {
        self.classifications.insert(record.site, record);
    }

    /// Every site classification, in site order.
    pub fn classifications(&self) -> impl Iterator<Item = &SpineSiteClassification> {
        self.classifications.values()
    }

    /// Append a solve certification.
    pub fn push_certification(&mut self, record: SpineSolveCertification) {
        self.certifications.push(record);
    }

    /// The solve certifications, in pass order.
    #[must_use]
    pub fn certifications(&self) -> &[SpineSolveCertification] {
        &self.certifications
    }

    /// Append a vouch record.
    pub fn push_vouch(&mut self, record: SpineVouch) {
        self.vouches.push(record);
    }

    /// The vouch records, in mint order.
    #[must_use]
    pub fn vouches(&self) -> &[SpineVouch] {
        &self.vouches
    }

    /// Write one site's ship decision.
    pub fn set_ship(&mut self, record: SpineProbeShip) {
        self.ships.insert(record.site, record);
    }

    /// Every ship decision, in site order.
    pub fn ships(&self) -> impl Iterator<Item = &SpineProbeShip> {
        self.ships.values()
    }

    /// Write one site's observation.
    pub fn set_observation(&mut self, record: SpineObservation) {
        self.observations.insert(record.site, record);
    }

    /// Every observation, in site order.
    pub fn observations(&self) -> impl Iterator<Item = &SpineObservation> {
        self.observations.values()
    }

    /// Append a validity round.
    pub fn push_round(&mut self, record: SpineValidityRound) {
        self.rounds.push(record);
    }

    /// The validity rounds, in round order.
    #[must_use]
    pub fn rounds(&self) -> &[SpineValidityRound] {
        &self.rounds
    }

    /// Append a survival outcome.
    pub fn push_survival(&mut self, record: SpineSurvival) {
        self.survivals.push(record);
    }

    /// The survival outcomes, in mint order.
    #[must_use]
    pub fn survivals(&self) -> &[SpineSurvival] {
        &self.survivals
    }

    /// Append a render-time decision.
    pub fn push_render_decision(&mut self, record: SpineRenderDecision) {
        self.render_decisions.push(record);
    }

    /// Append one authored region's shared decision.
    pub fn push_region_decision(&mut self, record: SpineRegionDecision<P>) {
        self.region_decisions.push(record);
    }

    /// Every authored region's shared decision, in census order.
    #[must_use]
    pub fn region_decisions(&self) -> &[SpineRegionDecision<P>] {
        &self.region_decisions
    }

    /// DEMOTE every shared region decision `stands` refuses, on exactly
    /// [`demote_dispositions`](Self::demote_dispositions)'s terms. Answers the demoted regions'
    /// contributing routes, in census order — the identity their narration keys by, since a region
    /// owns no leaf of its own.
    pub fn demote_region_decisions(
        &mut self,
        witness: InfluenceAccount,
        stands: impl Fn(&P::RegionDecision) -> bool,
        demoted: &P::RegionDecision,
    ) -> Vec<RegionRoutes> {
        let mut routes = Vec::new();
        for record in &mut self.region_decisions {
            if stands(&record.decision) {
                continue;
            }
            record.demote(demoted.clone(), witness);
            routes.push(record.routes.clone());
        }
        routes
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
        let _ = writeln!(out, "dorc-spine-new-arm");
        for species in SpineSpecies::ALL {
            if species.census_arm() != CensusArm::New {
                continue;
            }
            let _ = writeln!(out, "{} n={}", species.name(), self.population(species));
        }
        for record in &self.load_decisions {
            let _ = writeln!(
                out,
                "  load {} withheld={:?} account={}",
                record.name,
                record.withheld,
                record.account.label()
            );
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
        for record in &self.region_decisions {
            let _ = writeln!(
                out,
                "  region {:?} routes={} unkeyed={} account={} {:?}",
                record.region,
                record.routes.total(),
                record.routes.unkeyed().len(),
                record.account.label(),
                record.decision
            );
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
            SpineSpecies::RegionDecision => self.region_decisions.len(),
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
        type RegionDecision = &'static str;
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
        assert_eq!(
            new, 12,
            "`30E` §2's transitory species, plus `30L`'s region decision"
        );
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
        let account = OperandAccount::capped(0..20u32);
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
            spine.set_disposition(SpineDisposition::minted(
                SiteId { leaf, member },
                AstId(9),
                String::from("apt-get install nginx"),
                "Run",
                InfluenceAccount::authored_before_contact(),
            ));
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
    fn the_spine_stores_the_account_a_mint_supplied_and_computes_none() {
        // `309:rul-spine-preserves-never-stamps`. This test is the REWRITE of the one that pinned
        // the opposite: the Spine used to hold a run-wide grade and every setter assigned it over
        // whatever a mint supplied, which is what made a per-object account unobservable at the
        // reader. There is no stamp to hand in any more — the two records below differ only in what
        // their own constructors joined, and the Spine hands both back unchanged.
        // WIDENED, never MINTED: the influence mint has exactly one caller in the workspace
        // (`the_influence_grade_has_exactly_one_mint`), so a test that wants a phase marker builds
        // one the free, one-way way rather than opening a second intake seat.
        let phase = crate::influence::Influenced::authored_before_contact(()).widen();
        let mut spine = Spine::<TestPlane>::new();
        spine.set_disposition(SpineDisposition::minted(
            SiteId::leaf(LeafId(0)),
            AstId(0),
            String::new(),
            "Run",
            InfluenceAccount::authored_before_contact(),
        ));
        spine.set_disposition(SpineDisposition::minted(
            SiteId::leaf(LeafId(1)),
            AstId(1),
            String::new(),
            "Run",
            InfluenceAccount::of_phase(phase),
        ));
        assert_eq!(
            spine
                .disposition(SiteId::leaf(LeafId(0)))
                .map(InfluenceBearing::account),
            Some(InfluenceAccount::authored_before_contact()),
            "a pre-contact decision must stop wearing a post-contact run's phase"
        );
        assert_eq!(
            spine
                .disposition(SiteId::leaf(LeafId(1)))
                .map(InfluenceBearing::account),
            Some(InfluenceAccount::of_phase(phase)),
            "and an influenced one keeps its own evidence, on the same Spine"
        );
    }

    #[test]
    fn every_species_declares_how_its_writer_reaches_an_account() {
        // `306b:rul-consequential-sinks-require-influence`'s consumer half. Carrying an account and
        // having DERIVED one are different properties: the sealed trait forces the first, and this
        // census is what forces a new species to answer the second. The counts are the tripwire —
        // a species sliding from `Joined` to `UntrackedAdapter` is a staged hole somebody chose,
        // and it shows up here as a diff rather than as silence.
        let mut counts = [0usize; 3];
        for species in SpineSpecies::ALL {
            let index = AccountCarriage::ALL
                .iter()
                .position(|arm| *arm == species.account_carriage())
                .expect("the carriage census is closed over `AccountCarriage::ALL`");
            counts[index] += 1;
        }
        assert_eq!(
            counts[0], 12,
            "every species with a writer joins its inputs' accounts"
        );
        assert_eq!(
            counts[1], 0,
            "no writer carries an untracked adapter; growth here is the staging debt"
        );
        assert_eq!(
            counts[2], 4,
            "`30F` §4.5 + `30Nd`: vouch, observation, validity-round and outcome have no writer"
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
