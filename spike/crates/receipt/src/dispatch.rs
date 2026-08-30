//! The pre-dispatch authority chain: standup, prepared intent, required publication, and the
//! one-use permit the first potentially mutative dispatch consumes.
//!
//! Every state here is affine and privately constructed, and each one OWNS its predecessor.
//! That ownership is the whole mechanism: a permit is reached only by moving one exact prepared
//! intent through image accounting and through a publication, so there is no signature anywhere
//! that pairs one intent's publication with another intent. A caller cannot supply the binding
//! identity, because no step takes one.
//!
//! There is ONE route, and it is required publication. A configured bypass is not part of this
//! V1 surface: the words for it exist in the recorded vocabulary a document may spell, and
//! nothing here mints authority under them.
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
///
/// A RECORDED WORD, not authority. [`Self::configured_bypass`] exists because the document
/// vocabulary can spell that posture and a projection has to be able to write the row; there is
/// deliberately no route from an intent wearing it to a permit, and
/// [`AccountedApplyIntent::publish_through`] refuses one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptPolicyWitness(RecordedApplyPolicy);

impl ReceiptPolicyWitness {
    /// The V1 posture, and the only one that dispatches: a rich intent is published before
    /// dispatch or nothing dispatches.
    #[must_use]
    pub const fn required_rich() -> Self {
        Self(RecordedApplyPolicy::RequiredRich)
    }

    /// The posture a document may RECORD for an apply that ran with no durable intent behind
    /// it. No route in this crate turns one into a permit.
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
/// Not `Clone`: an intent is prepared once and is either MOVED through accounting and
/// publication or dropped. Every state past this one owns it, so there is no borrow a second
/// publication could be built against.
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

    /// Account every assignment's exact image against the region about to be published,
    /// CONSUMING this intent into the accounted state.
    ///
    /// The accounting is a byte comparison against each assignment's own canonical image
    /// encoding, keyed by the record the assignment occupies. It takes `self` by value so the
    /// witness cannot be separated from the intent it was earned for: what comes back OWNS this
    /// exact intent, and a caller holding two intents cannot account one and publish the other.
    ///
    /// `record_of` answers which skeleton record an assignment ordinal occupies, because that
    /// numbering belongs to the document being assembled rather than to this type.
    #[must_use]
    pub fn account_images(
        self,
        entries: &[OverlayEntry],
        record_of: &dyn Fn(AssignmentOrdinal) -> Option<u64>,
    ) -> Option<AccountedApplyIntent> {
        for assignment in &self.assignments {
            let record = record_of(assignment.ordinal)?;
            let carried = entries.iter().find(|entry| {
                entry.record() == record && entry.tag() == OpaqueFieldTag::ApplyArtifactImage
            })?;
            if carried.bytes() != assignment.image.encode() {
                return None;
            }
        }
        Some(AccountedApplyIntent { intent: self })
    }
}

/// One prepared intent whose every assignment's exact image was found in the region about to be
/// sealed.
///
/// Minted only by [`PreparedApplyIntent::account_images`], which consumes the intent, so the
/// accounting and the intent are one value. Not `Clone`, and its field is private, so there is
/// no literal spelling of it outside this module and no way to swap the intent inside one.
#[derive(Debug)]
pub struct AccountedApplyIntent {
    intent: PreparedApplyIntent,
}

/// What a placement answered about one required landing.
///
/// A REPORT and never authority: it says what a placement claims it did, in primitives, and the
/// gate value is minted from it inside [`AccountedApplyIntent::publish_through`]. Holding one
/// authorizes nothing, because the scarce half of a publication is the accounted intent, which
/// nothing outside this crate can build.
///
/// That is the honest boundary and it is worth stating plainly: no Rust type can prove a file
/// reached a disk in a crate this one does not know about. What the type system carries is that
/// a publication value exists only where an accounted intent was moved through a placement call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredPlacementLanding {
    document_digest: Sha256Digest,
    policy_identity: &'static str,
}

impl RequiredPlacementLanding {
    /// Report that a placement landed the document, whose bytes digest to `document_digest`,
    /// under the policy `policy_identity` names.
    #[must_use]
    pub const fn of(document_digest: Sha256Digest, policy_identity: &'static str) -> Self {
        Self {
            document_digest,
            policy_identity,
        }
    }

    /// The digest of the exact bytes the placement says it wrote.
    #[must_use]
    pub const fn document_digest(self) -> Sha256Digest {
        self.document_digest
    }

    /// Which policy the placement was judged under.
    #[must_use]
    pub const fn policy_identity(self) -> &'static str {
        self.policy_identity
    }
}

/// Why a required publication could not be assembled into one value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentPublicationMismatch {
    /// The intent's own policy is not the one a required publication answers.
    PolicyIsNotRequired,
}

/// Why publishing an accounted intent did not produce a gate value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationThrough<E> {
    /// The placement itself refused, in its own words.
    Placement(E),
    /// The placement answered, and the intent it answered for is not publishable this way.
    Mismatch(IntentPublicationMismatch),
}

impl AccountedApplyIntent {
    /// Publish this exact intent through `place`, minting the gate value from what the
    /// placement answered.
    ///
    /// The mint of [`PublishedApplyIntentV1`] is private to this module and lives inside this
    /// call, so no code anywhere can hold a publication value that no placement produced. The
    /// document identity is handed TO the placement rather than taken from it, which is what
    /// keeps the identity a publication records and the identity a placement filed the same
    /// one.
    ///
    /// `T` is whatever the placement wants to carry back out beside its landing — where the
    /// document went, typically — so a caller needs no side channel out of the closure.
    ///
    /// # Errors
    /// Answers the placement's own refusal, and a policy that is not the required one.
    pub fn publish_through<T, E>(
        self,
        id: ApplyIntentId,
        place: impl FnOnce(ApplyIntentId) -> Result<(RequiredPlacementLanding, T), E>,
    ) -> Result<(PublishedApplyIntentV1, T), PublicationThrough<E>> {
        if self.intent.policy.token() != RecordedApplyPolicy::RequiredRich {
            return Err(PublicationThrough::Mismatch(
                IntentPublicationMismatch::PolicyIsNotRequired,
            ));
        }
        let (landing, carried) = place(id).map_err(PublicationThrough::Placement)?;
        Ok((
            PublishedApplyIntentV1 {
                id,
                landing,
                accounted: self,
            },
            carried,
        ))
    }
}

/// One published rich apply intent, with everything the permit mint rests on, bound together.
///
/// The members `30Rb:critical-type-effect-map` names — the exact prepared intent, the
/// image-account witness, the policy, the published identity and the landing's digest — are one
/// value here because each OWNS the one below it. Nothing hands the members back out and
/// nothing takes a second intent, so pairing one intent's publication with another's witness is
/// not an error a caller can make: it is not spellable.
///
/// Not `Clone`: one publication authorizes one dispatch.
#[derive(Debug)]
pub struct PublishedApplyIntentV1 {
    id: ApplyIntentId,
    landing: RequiredPlacementLanding,
    accounted: AccountedApplyIntent,
}

impl PublishedApplyIntentV1 {
    /// The identity of the intent that was published.
    #[must_use]
    pub const fn id(&self) -> ApplyIntentId {
        self.id
    }

    /// The digest of the exact bytes the placement reported writing.
    #[must_use]
    pub const fn document_digest(&self) -> Sha256Digest {
        self.landing.document_digest()
    }

    /// The policy the publication answered.
    #[must_use]
    pub const fn policy(&self) -> ReceiptPolicyWitness {
        self.accounted.intent.policy
    }

    /// Mint the one-use permit, consuming the publication that earned it.
    ///
    /// There is no second argument, and that absence is the repair: the intent this permit is
    /// for is the one this publication has owned since it was accounted. What survives into the
    /// permit is the DECLARED assignment set, because an outcome may only name an assignment
    /// this intent actually declared.
    #[must_use]
    pub fn permit(self) -> MutationDispatchPermit {
        let PreparedApplyIntent {
            session,
            generation,
            assignments,
            policy,
        } = self.accounted.intent;
        MutationDispatchPermit {
            policy: policy.token(),
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
/// [`PublishedApplyIntentV1::permit`], so a permit cannot exist without a publication of THIS
/// intent having happened, and cannot be spent twice.
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
