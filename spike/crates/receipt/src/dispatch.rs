//! The pre-dispatch authority chain: standup, prepared intent, publication gate, and the
//! one-use permit the first potentially mutative dispatch consumes.
//!
//! Every state here is affine and privately constructed. The chain exists so that "we spent
//! authority to mutate a host" is a thing a type records rather than a thing a call site
//! remembers to do, and so that the two routes to a permit — required publication of a rich
//! intent, and an explicit configured bypass — cannot be reached from one another.
//!
//! The crate stays pure: a session's resolved identity arrives as VALUES from whatever edge
//! established it. Nothing here opens a connection, reads an environment, or asks a clock.

use crate::ids::{
    ApplyGenerationId, ApplyIntentId, ApplySessionId, PlanReceiptId, PresentedPlanId,
    ReadyApplyTargetId, Sha256Digest,
};
use crate::image::ApplyArtifactImage;
use crate::overlay::OverlayEntry;
use crate::projection::OpaqueFieldTag;
use crate::rows::{AssignmentOrdinal, OriginOrdinal};
use crate::tokens::RecordedApplyPolicy;

/// Where a controller is sending an apply's bytes.
///
/// `addressed` rather than resolved: what a controller knows without asking anybody is the
/// destination it will hand its own transport, which is invocation material and therefore its
/// own fact. Nothing on the far side has confirmed a name, and this type claims none.
///
/// The spelling is private and readable only inside this crate, so the ONE slot that records a
/// destination is the only thing that can read one. That is what closes the substitution
/// [`crate::project::InvocationTarget`] exists to refuse: a row recording what somebody TYPED
/// takes bytes, and no caller can obtain bytes from one of these to hand it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyDestination(String);

impl ApplyDestination {
    /// Name the destination this controller will address.
    #[must_use]
    pub const fn addressed(spelling: String) -> Self {
        Self(spelling)
    }

    /// The exact bytes, for the one slot that records them.
    pub(crate) fn bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// One context axis a standup either entered or did not.
///
/// Two arms and no third, because "the axis was entered and resolved to nothing" and "no context
/// was entered" are different statements and a session that established nothing makes only the
/// second. An empty [`Self::Established`] is therefore a real answer rather than a spelling of
/// absence — which is why absence needs an arm of its own instead of a sentinel value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedAxis {
    /// The standup entered this axis, and this is what it resolved to.
    Established(String),
    /// Nothing was entered on this axis.
    NotEstablished,
}

/// The six dimensions a standup answers about one target.
///
/// All six are required and NONE has a default. The list is not decoration: a shift in any of
/// them changes which world the artifact's own reads answer in, so an axis nobody can speak for
/// must say so rather than be filled in.
///
/// A session that entered no context answers [`ResolvedAxis::NotEstablished`] five times, which
/// is a true statement a controller can make about itself: nothing escalated, nothing entered,
/// running as whatever the destination resolves to. It establishes very little, and saying so is
/// the point — the constructor takes six arguments however thin the session is, so growing this
/// value as machinery arrives is filling arms in, never adding fields nobody fills.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedApplyContext {
    destination: ApplyDestination,
    account: ResolvedAxis,
    namespace: ResolvedAxis,
    working_directory: ResolvedAxis,
    environment_policy: ResolvedAxis,
    credential_scope: ResolvedAxis,
}

impl ResolvedApplyContext {
    /// Take one standup's destination and its five context answers.
    #[must_use]
    pub const fn of(
        destination: ApplyDestination,
        account: ResolvedAxis,
        namespace: ResolvedAxis,
        working_directory: ResolvedAxis,
        environment_policy: ResolvedAxis,
        credential_scope: ResolvedAxis,
    ) -> Self {
        Self {
            destination,
            account,
            namespace,
            working_directory,
            environment_policy,
            credential_scope,
        }
    }

    /// Where the controller addressed this target.
    #[must_use]
    pub const fn destination(&self) -> &ApplyDestination {
        &self.destination
    }

    /// The principal the session authenticated as.
    #[must_use]
    pub const fn account(&self) -> &ResolvedAxis {
        &self.account
    }

    /// The namespace the session entered.
    #[must_use]
    pub const fn namespace(&self) -> &ResolvedAxis {
        &self.namespace
    }

    /// Where the session stands.
    #[must_use]
    pub const fn working_directory(&self) -> &ResolvedAxis {
        &self.working_directory
    }

    /// Which environment the session carries.
    #[must_use]
    pub const fn environment_policy(&self) -> &ResolvedAxis {
        &self.environment_policy
    }

    /// What the session's credentials reach.
    #[must_use]
    pub const fn credential_scope(&self) -> &ResolvedAxis {
        &self.credential_scope
    }
}

/// One target a standup resolved, bound to the session that resolved it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadyApplyTarget {
    id: ReadyApplyTargetId,
    context: ResolvedApplyContext,
}

impl ReadyApplyTarget {
    /// Name one resolved target.
    #[must_use]
    pub const fn of(id: ReadyApplyTargetId, context: ResolvedApplyContext) -> Self {
        Self { id, context }
    }

    /// This target's identity within its session.
    #[must_use]
    pub const fn id(&self) -> ReadyApplyTargetId {
        self.id
    }

    /// What the standup resolved.
    #[must_use]
    pub const fn context(&self) -> &ResolvedApplyContext {
        &self.context
    }
}

/// One originating plan an assignment composes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanOriginOccurrence {
    ordinal: OriginOrdinal,
    receipt: PlanReceiptId,
    presented: PresentedPlanId,
}

impl PlanOriginOccurrence {
    /// Name one occurrence. Duplicates are legal: the admin may compose one plan twice.
    #[must_use]
    pub const fn of(
        ordinal: OriginOrdinal,
        receipt: PlanReceiptId,
        presented: PresentedPlanId,
    ) -> Self {
        Self {
            ordinal,
            receipt,
            presented,
        }
    }

    /// Where this occurrence sat within its assignment.
    #[must_use]
    pub const fn ordinal(&self) -> OriginOrdinal {
        self.ordinal
    }

    /// Which plan document it came from.
    #[must_use]
    pub const fn receipt(&self) -> PlanReceiptId {
        self.receipt
    }

    /// Which approval surface it came from.
    #[must_use]
    pub const fn presented(&self) -> PresentedPlanId {
        self.presented
    }
}

/// Which presented plans an assignment composes, before the intent is prepared.
///
/// [`OriginatingPlans`]'s live counterpart: `Unavailable` is explicit, and an empty `Known` is
/// unrepresentable because the constructor refuses it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingOrigins {
    /// The apply cannot say which plans this assignment came from.
    Unavailable,
    /// At least one occurrence, in the order the admin composed them.
    Known(Vec<PlanOriginOccurrence>),
}

impl PendingOrigins {
    /// Take a non-empty occurrence list, refusing an empty one.
    #[must_use]
    pub fn known(occurrences: Vec<PlanOriginOccurrence>) -> Option<Self> {
        (!occurrences.is_empty()).then_some(Self::Known(occurrences))
    }

    /// How many occurrences this declares.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Unavailable => 0,
            Self::Known(occurrences) => occurrences.len(),
        }
    }

    /// Whether this declares no occurrence.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The occurrences, empty when unavailable.
    #[must_use]
    pub fn occurrences(&self) -> &[PlanOriginOccurrence] {
        match self {
            Self::Unavailable => &[],
            Self::Known(occurrences) => occurrences,
        }
    }

    /// The closed word the intent row records.
    #[must_use]
    pub const fn state(&self) -> crate::tokens::RecordedOriginState {
        match self {
            Self::Unavailable => crate::tokens::RecordedOriginState::Unavailable,
            Self::Known(_) => crate::tokens::RecordedOriginState::Known,
        }
    }
}

/// One target-and-image pairing the admin asked for, before a session validates it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingApplyAssignment {
    ordinal: AssignmentOrdinal,
    target: ReadyApplyTargetId,
    image: ApplyArtifactImage,
    origins: PendingOrigins,
}

impl PendingApplyAssignment {
    /// Ask for one image to be applied to one target.
    #[must_use]
    pub const fn of(
        ordinal: AssignmentOrdinal,
        target: ReadyApplyTargetId,
        image: ApplyArtifactImage,
        origins: PendingOrigins,
    ) -> Self {
        Self {
            ordinal,
            target,
            image,
            origins,
        }
    }

    /// Where this assignment sits in the intent.
    #[must_use]
    pub const fn ordinal(&self) -> AssignmentOrdinal {
        self.ordinal
    }
}

/// One assignment bound to the session that resolved its target.
///
/// No public constructor: the only mint is [`ApplySessionReady::prepare_intent`], which copies
/// the session's own answer for the named target rather than accepting one from a caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionApplyAssignment {
    ordinal: AssignmentOrdinal,
    session: ApplySessionId,
    target: ReadyApplyTargetId,
    context: ResolvedApplyContext,
    image: ApplyArtifactImage,
    origins: PendingOrigins,
}

impl SessionApplyAssignment {
    /// Where this assignment sits in the intent.
    #[must_use]
    pub const fn ordinal(&self) -> AssignmentOrdinal {
        self.ordinal
    }

    /// The session this assignment is bound to.
    #[must_use]
    pub const fn session(&self) -> ApplySessionId {
        self.session
    }

    /// Which resolved target it names.
    #[must_use]
    pub const fn target(&self) -> ReadyApplyTargetId {
        self.target
    }

    /// The session's own answer for that target, copied at the mint.
    #[must_use]
    pub const fn context(&self) -> &ResolvedApplyContext {
        &self.context
    }

    /// The exact image, by value.
    #[must_use]
    pub const fn image(&self) -> &ApplyArtifactImage {
        &self.image
    }

    /// Which presented plans it composes.
    #[must_use]
    pub const fn origins(&self) -> &PendingOrigins {
        &self.origins
    }
}

/// Why a session refused to prepare an intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntentPreparationRefusal {
    /// No assignment was offered.
    NoAssignments,
    /// An assignment named a target this session did not resolve.
    UnknownTarget,
    /// Two assignments claimed one ordinal.
    DuplicateOrdinal,
    /// Assignment ordinals are not contiguous from zero.
    OrdinalNotContiguous {
        /// The ordinal the sequence wanted next.
        expected: u32,
        /// The ordinal it found.
        found: u32,
    },
    /// A target this session resolved was left unassigned.
    ReadyTargetOmitted,
    /// An origin list declared occurrence ordinals that are not contiguous from zero.
    OriginNotContiguous {
        /// The ordinal the sequence wanted next.
        expected: u32,
        /// The ordinal it found.
        found: u32,
    },
}

/// One `dorc apply` invocation's standup: every target it resolved, as one aggregate.
///
/// ONE per invocation, never one per target. A per-target standup record licenses nothing by
/// itself, because the thing being authorized is an apply, not a connection.
#[derive(Debug)]
pub struct ApplySessionReady {
    id: ApplySessionId,
    generation: ApplyGenerationId,
    targets: Vec<ReadyApplyTarget>,
}

impl ApplySessionReady {
    /// Close a standup over the targets it resolved.
    ///
    /// # Errors
    /// Refuses an empty standup and a repeated target identity.
    pub fn of(
        id: ApplySessionId,
        generation: ApplyGenerationId,
        targets: Vec<ReadyApplyTarget>,
    ) -> Result<Self, IntentPreparationRefusal> {
        if targets.is_empty() {
            return Err(IntentPreparationRefusal::NoAssignments);
        }
        let mut seen: Vec<ReadyApplyTargetId> = targets.iter().map(ReadyApplyTarget::id).collect();
        seen.sort_unstable();
        let before = seen.len();
        seen.dedup();
        if seen.len() != before {
            return Err(IntentPreparationRefusal::UnknownTarget);
        }
        Ok(Self {
            id,
            generation,
            targets,
        })
    }

    /// This session's identity.
    #[must_use]
    pub const fn id(&self) -> ApplySessionId {
        self.id
    }

    /// This session's dispatch generation.
    #[must_use]
    pub const fn generation(&self) -> ApplyGenerationId {
        self.generation
    }

    /// The targets it resolved.
    #[must_use]
    pub fn targets(&self) -> &[ReadyApplyTarget] {
        &self.targets
    }

    /// Bind assignments to this session, consuming it.
    ///
    /// Each assignment's target must name exactly one member, and the mint COPIES that
    /// member's resolved context into the record rather than accepting a caller's. Every
    /// resolved target must be assigned: a standup that reached a host the intent then omits
    /// is a session the intent does not describe.
    ///
    /// # Errors
    /// Refuses an empty, unknown-target, duplicate, non-contiguous, or partial assignment set.
    pub fn prepare_intent(
        self,
        assignments: Vec<PendingApplyAssignment>,
        policy: ReceiptPolicyWitness,
    ) -> Result<PreparedApplyIntent, IntentPreparationRefusal> {
        if assignments.is_empty() {
            return Err(IntentPreparationRefusal::NoAssignments);
        }
        let mut ordinals: Vec<u32> = assignments
            .iter()
            .map(|a| a.ordinal.get())
            .collect::<Vec<_>>();
        ordinals.sort_unstable();
        for (index, ordinal) in ordinals.iter().enumerate() {
            let expected = u32::try_from(index).unwrap_or(u32::MAX);
            if *ordinal < expected {
                return Err(IntentPreparationRefusal::DuplicateOrdinal);
            }
            if *ordinal != expected {
                return Err(IntentPreparationRefusal::OrdinalNotContiguous {
                    expected,
                    found: *ordinal,
                });
            }
        }
        let mut bound: Vec<SessionApplyAssignment> = Vec::with_capacity(assignments.len());
        for assignment in assignments {
            let Some(target) = self
                .targets
                .iter()
                .find(|target| target.id == assignment.target)
            else {
                return Err(IntentPreparationRefusal::UnknownTarget);
            };
            for (index, occurrence) in assignment.origins.occurrences().iter().enumerate() {
                let expected = u32::try_from(index).unwrap_or(u32::MAX);
                if occurrence.ordinal.get() != expected {
                    return Err(IntentPreparationRefusal::OriginNotContiguous {
                        expected,
                        found: occurrence.ordinal.get(),
                    });
                }
            }
            bound.push(SessionApplyAssignment {
                ordinal: assignment.ordinal,
                session: self.id,
                target: assignment.target,
                context: target.context.clone(),
                image: assignment.image,
                origins: assignment.origins,
            });
        }
        if !self
            .targets
            .iter()
            .all(|target| bound.iter().any(|a| a.target == target.id))
        {
            return Err(IntentPreparationRefusal::ReadyTargetOmitted);
        }
        bound.sort_by_key(SessionApplyAssignment::ordinal);
        Ok(PreparedApplyIntent {
            session: self.id,
            generation: self.generation,
            assignments: bound,
            policy,
        })
    }
}

/// Which publication policy an apply is running under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptPolicyWitness(RecordedApplyPolicy);

impl ReceiptPolicyWitness {
    /// The default posture: a rich intent is published before dispatch or nothing dispatches.
    #[must_use]
    pub const fn required_rich() -> Self {
        Self(RecordedApplyPolicy::RequiredRich)
    }

    /// The explicitly configured posture that permits dispatch without required publication.
    #[must_use]
    pub const fn configured_bypass() -> Self {
        Self(RecordedApplyPolicy::ConfiguredBypass)
    }

    /// The closed word the intent row records.
    #[must_use]
    pub const fn token(self) -> RecordedApplyPolicy {
        self.0
    }
}

/// An intent whose assignments, session, generation and policy are frozen.
///
/// Not `Clone`: an intent is prepared once and either reaches a gate or is dropped.
#[derive(Debug)]
pub struct PreparedApplyIntent {
    session: ApplySessionId,
    generation: ApplyGenerationId,
    assignments: Vec<SessionApplyAssignment>,
    policy: ReceiptPolicyWitness,
}

impl PreparedApplyIntent {
    /// The session this intent was prepared under.
    #[must_use]
    pub const fn session(&self) -> ApplySessionId {
        self.session
    }

    /// The dispatch generation.
    #[must_use]
    pub const fn generation(&self) -> ApplyGenerationId {
        self.generation
    }

    /// The bound assignments, in ordinal order and never empty.
    #[must_use]
    pub fn assignments(&self) -> &[SessionApplyAssignment] {
        &self.assignments
    }

    /// The policy in force.
    #[must_use]
    pub const fn policy(&self) -> ReceiptPolicyWitness {
        self.policy
    }

    /// Which presented-plan state the intent row records across every assignment.
    #[must_use]
    pub fn origin_state(&self) -> crate::tokens::RecordedOriginState {
        if self
            .assignments
            .iter()
            .any(|assignment| !assignment.origins.is_empty())
        {
            crate::tokens::RecordedOriginState::Known
        } else {
            crate::tokens::RecordedOriginState::Unavailable
        }
    }

    /// Prove that every assignment's exact image reached the region about to be published.
    ///
    /// The accounting is a byte comparison against each assignment's own canonical image
    /// encoding, keyed by the record the assignment occupies. A caller cannot hand over the
    /// capability, and cannot obtain one by declaring the images present: the entries checked
    /// here are the entries that will be sealed.
    ///
    /// `record_of` answers which skeleton record an assignment ordinal occupies, because that
    /// numbering belongs to the document being assembled rather than to this type.
    #[must_use]
    pub fn account_images(
        &self,
        entries: &[OverlayEntry],
        record_of: &dyn Fn(AssignmentOrdinal) -> Option<u64>,
    ) -> Option<ExactApplyImagesPresent> {
        for assignment in &self.assignments {
            let record = record_of(assignment.ordinal)?;
            let carried = entries.iter().find(|entry| {
                entry.record() == record && entry.tag() == OpaqueFieldTag::ApplyArtifactImage
            })?;
            if carried.bytes() != assignment.image.encode() {
                return None;
            }
        }
        Some(ExactApplyImagesPresent(()))
    }
}

/// Proof that a published rich intent carried every assignment's exact image by value.
///
/// Minted only by [`PreparedApplyIntent::account_images`]. Not `Clone`, and its field is a
/// private unit, so there is no literal spelling of it outside this module.
#[derive(Debug)]
pub struct ExactApplyImagesPresent(());

/// An explicitly configured decision to dispatch without required publication.
///
/// Not `Clone`, and deliberately verbose to construct: this is the one value that lets a
/// mutation proceed with no durable intent behind it, so a reader grepping for it finds every
/// site that spends it.
#[derive(Debug)]
pub struct ConfiguredReceiptBypass(());

impl ConfiguredReceiptBypass {
    /// Declare that this invocation is configured to dispatch without required publication.
    #[must_use]
    pub const fn configured() -> Self {
        Self(())
    }
}

/// Proof that a durable store placed one exact document at its platform's required baseline.
///
/// Carries the three facts a gate has to bind and nothing else: which document identity was
/// filed, the digest of the exact bytes filed under it, and which policy judged the placement.
/// Not `Clone`, so one placement funds one gate.
///
/// The mint is public because the store that earns one lives in a crate downstream of this one,
/// and no type can privilege that crate over any other. The fence is therefore lexical and
/// two-way, in `receipt/tests/crate_boundary.rs`.
#[derive(Debug)]
pub struct DurablePublicationProof {
    receipt_id_hex: String,
    document_digest: Sha256Digest,
    policy_identity: &'static str,
}

impl DurablePublicationProof {
    /// Record that a store placed `receipt_id_hex`'s document, whose bytes digest to
    /// `document_digest`, under the policy `policy_identity` names.
    #[must_use]
    pub const fn of_required_placement(
        receipt_id_hex: String,
        document_digest: Sha256Digest,
        policy_identity: &'static str,
    ) -> Self {
        Self {
            receipt_id_hex,
            document_digest,
            policy_identity,
        }
    }

    /// The identity of the document that was placed.
    #[must_use]
    pub fn receipt_id_hex(&self) -> &str {
        &self.receipt_id_hex
    }

    /// The digest of the exact bytes placed.
    #[must_use]
    pub const fn document_digest(&self) -> Sha256Digest {
        self.document_digest
    }

    /// Which policy the placement was judged under.
    #[must_use]
    pub const fn policy_identity(&self) -> &'static str {
        self.policy_identity
    }
}

/// Why a required publication could not be assembled into one value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentPublicationMismatch {
    /// The durability proof names a document other than this intent.
    ProofNamesAnotherDocument,
    /// The requested policy is not the one a required publication answers.
    PolicyIsNotRequired,
}

/// One published rich apply intent, with everything the permit mint rests on, bound together.
///
/// The four members `30Rb:critical-type-effect-map` names — the exact intent receipt, the
/// image-account witness, the requested policy, and the durable publication proof — arrive at
/// ONE mint that CHECKS their agreement rather than trusting it: the proof must name this
/// intent's own identity, and the policy must be the required one. Nothing hands the members
/// back out, so a caller cannot pair one intent's publication with another's image witness after
/// the fact.
///
/// Not `Clone`: one publication authorizes one dispatch.
#[derive(Debug)]
pub struct PublishedApplyIntentV1 {
    id: ApplyIntentId,
    document_digest: Sha256Digest,
    policy: ReceiptPolicyWitness,
    #[expect(
        dead_code,
        reason = "held to prove the images reached the placed region; there is deliberately no \
                  accessor, because reading it back would be a second use of a one-use witness"
    )]
    images: ExactApplyImagesPresent,
}

impl PublishedApplyIntentV1 {
    /// Bind one placement to the intent, images and policy it was earned by.
    ///
    /// # Errors
    /// Refuses a proof naming another document and a policy that is not the required one.
    pub fn minted(
        id: ApplyIntentId,
        images: ExactApplyImagesPresent,
        policy: ReceiptPolicyWitness,
        durability: DurablePublicationProof,
    ) -> Result<Self, IntentPublicationMismatch> {
        // Destructured rather than read through accessors: the proof is SPENT here, and one
        // placement funds one gate. A borrow would let a single publication clear two.
        let DurablePublicationProof {
            receipt_id_hex,
            document_digest,
            policy_identity: _,
        } = durability;
        if receipt_id_hex != id.hex() {
            return Err(IntentPublicationMismatch::ProofNamesAnotherDocument);
        }
        if policy.token() != RecordedApplyPolicy::RequiredRich {
            return Err(IntentPublicationMismatch::PolicyIsNotRequired);
        }
        Ok(Self {
            id,
            document_digest,
            policy,
            images,
        })
    }

    /// The identity of the intent that was published.
    #[must_use]
    pub const fn id(&self) -> ApplyIntentId {
        self.id
    }

    /// The digest of the exact bytes that were placed.
    #[must_use]
    pub const fn document_digest(&self) -> Sha256Digest {
        self.document_digest
    }

    /// The policy the publication answered.
    #[must_use]
    pub const fn policy(&self) -> ReceiptPolicyWitness {
        self.policy
    }
}

/// How an intent cleared the pre-dispatch boundary.
///
/// The two arms are disjoint and neither converts to the other: there is no route from a
/// plain publication, an attempted publication, or a failed one into `Published`.
#[derive(Debug)]
pub enum IntentPublicationGate {
    /// A rich intent was placed durably, and every assignment's exact image was in it.
    Published(PublishedApplyIntentV1),
    /// An explicit configuration permitted dispatch without required publication.
    ConfiguredBypass(ConfiguredReceiptBypass),
}

impl IntentPublicationGate {
    /// The closed word describing which route was taken.
    #[must_use]
    pub const fn policy(&self) -> RecordedApplyPolicy {
        match self {
            Self::Published(_) => RecordedApplyPolicy::RequiredRich,
            Self::ConfiguredBypass(_) => RecordedApplyPolicy::ConfiguredBypass,
        }
    }

    /// Mint the one-use permit, consuming BOTH the gate and the intent it cleared.
    ///
    /// The intent is spent rather than borrowed so one prepared intent cannot clear two
    /// gates: publication runs against a borrow, and the value itself ends here. What
    /// survives into the permit is the DECLARED assignment set, because an outcome may only
    /// name an assignment this intent actually declared.
    #[must_use]
    pub fn permit(self, intent: PreparedApplyIntent) -> MutationDispatchPermit {
        let policy = self.policy();
        let PreparedApplyIntent {
            session,
            generation,
            assignments,
            policy: _,
        } = intent;
        MutationDispatchPermit {
            policy,
            session,
            generation,
            declared: assignments
                .into_iter()
                .map(|assignment| assignment.ordinal)
                .collect(),
        }
    }
}

/// The authority to dispatch the first potentially mutative command of one apply.
///
/// Not `Clone`, and spent by value. There is no constructor: the sole mint is
/// [`IntentPublicationGate::permit`], so a permit cannot exist without a gate having been
/// cleared, and cannot be spent twice.
#[derive(Debug)]
pub struct MutationDispatchPermit {
    policy: RecordedApplyPolicy,
    session: ApplySessionId,
    generation: ApplyGenerationId,
    declared: Vec<AssignmentOrdinal>,
}

impl MutationDispatchPermit {
    /// Spend the permit, entering the authority-spent phase.
    ///
    /// Spent immediately BEFORE the dispatching call, and spent even when that call turns out
    /// to have attempted nothing: the controller committed, and a committed apply must not be
    /// retried on the strength of an unknown answer.
    #[must_use]
    pub fn spend(self) -> MutationDispatched {
        MutationDispatched {
            policy: self.policy,
            session: self.session,
            generation: self.generation,
            declared: self.declared,
        }
    }
}

/// The phase after a permit is spent. Durable-only failure no longer withholds mutation.
///
/// Not `Copy`: it owns the declared assignment set, which is what an outcome projection
/// checks a site row against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationDispatched {
    policy: RecordedApplyPolicy,
    session: ApplySessionId,
    generation: ApplyGenerationId,
    declared: Vec<AssignmentOrdinal>,
}

impl MutationDispatched {
    /// Which route authorized the dispatch.
    #[must_use]
    pub const fn policy(&self) -> RecordedApplyPolicy {
        self.policy
    }

    /// The session whose authority was spent.
    #[must_use]
    pub const fn session(&self) -> ApplySessionId {
        self.session
    }

    /// The generation whose authority was spent.
    #[must_use]
    pub const fn generation(&self) -> ApplyGenerationId {
        self.generation
    }

    /// Did the cleared intent declare this assignment?
    ///
    /// An outcome projection asks before recording a site row: a row naming an assignment
    /// the intent never declared would attribute execution to a target nobody authorized.
    #[must_use]
    pub fn declares(&self, ordinal: AssignmentOrdinal) -> bool {
        self.declared.contains(&ordinal)
    }

    /// Every assignment the cleared intent declared, in ordinal order.
    #[must_use]
    pub fn declared(&self) -> &[AssignmentOrdinal] {
        &self.declared
    }

    /// Continue orchestration past a failure that is only about the durable.
    ///
    /// Takes [`DurableFailure`] and not [`PostDispatchFailure`], so a caller holding an
    /// integrity failure cannot reach this by widening a match arm.
    #[must_use]
    pub fn continue_after(self, failure: DurableFailure) -> DurableFailureReported {
        DurableFailureReported {
            phase: self,
            failure,
        }
    }
}

/// A durable failure that was reported and did not stop the apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableFailureReported {
    phase: MutationDispatched,
    failure: DurableFailure,
}

impl DurableFailureReported {
    /// The phase the apply continued in.
    #[must_use]
    pub const fn phase(&self) -> &MutationDispatched {
        &self.phase
    }

    /// What failed.
    #[must_use]
    pub const fn failure(&self) -> DurableFailure {
        self.failure
    }
}

/// What went wrong with the durable, and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableFailure {
    /// The document could not be projected from what the run held.
    Projection,
    /// A projected row did not satisfy the grammar.
    Grammar,
    /// The region could not be sealed.
    Seal,
    /// The document could not be signed.
    Signature,
    /// The sink did not place the document.
    Sink,
}

/// Transport integrity was lost after dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportIntegrityFailure;

/// Execution integrity was lost after dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionIntegrityFailure;

/// Controller attribution was lost after dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributionIntegrityFailure;

/// The dispatch generation was superseded or revoked after dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenerationIntegrityFailure;

/// The target the apply reached is not the target it was authorized for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetIntegrityFailure;

/// Mutation integrity was lost after dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationIntegrityFailure;

/// Everything that can go wrong after the permit is spent, with the durable kept apart.
///
/// Six of the seven arms retain their existing abort behaviour. The seventh is the only one
/// [`MutationDispatched::continue_after`] accepts, and that asymmetry is the whole point of
/// the enum: a generic fallback that swallowed the other six would turn a lost host into a
/// logging problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostDispatchFailure {
    /// Only the durable failed.
    DurableOnly(DurableFailure),
    /// The channel to the host was lost.
    TransportIntegrity(TransportIntegrityFailure),
    /// What executed is no longer known.
    ExecutionIntegrity(ExecutionIntegrityFailure),
    /// Who the controller is talking to is no longer established.
    AttributionIntegrity(AttributionIntegrityFailure),
    /// This generation's authority was revoked.
    GenerationIntegrity(GenerationIntegrityFailure),
    /// The reached target is not the authorized one.
    TargetIntegrity(TargetIntegrityFailure),
    /// Mutation integrity was lost.
    MutationIntegrity(MutationIntegrityFailure),
}

impl PostDispatchFailure {
    /// The durable failure this is, where it is one.
    ///
    /// The ONE narrowing, and it answers `None` for every integrity arm. A caller reaching
    /// [`MutationDispatched::continue_after`] therefore has to have come through here and
    /// handled the `None`.
    #[must_use]
    pub const fn durable_only(self) -> Option<DurableFailure> {
        match self {
            Self::DurableOnly(failure) => Some(failure),
            Self::TransportIntegrity(_)
            | Self::ExecutionIntegrity(_)
            | Self::AttributionIntegrity(_)
            | Self::GenerationIntegrity(_)
            | Self::TargetIntegrity(_)
            | Self::MutationIntegrity(_) => None,
        }
    }
}
