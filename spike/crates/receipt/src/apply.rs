//! The `apply-intent` species recorded model, and the admin-owned plan-to-target mapping.
//!
//! The mapping is many-to-many because the admin owns it, not Dorc: one presented plan may
//! feed many assignments, one assignment may compose several presented plans or none, and a
//! plan specialized for one target may be assigned to another. The model records both truths
//! and never rewrites provenance to make them agree.

use crate::format::{RefusalReason, SkeletonRecord};
use crate::grammar::RecordKind;
use crate::model::{ApplyIntent, Species};
use crate::reingested::RecordedInfluence;
use crate::rows::{
    self, AssignmentOrdinal, ModelRefusal, OriginOrdinal, RecordedInvocation,
    RecordedProjectionOmission, RecordedRow, RelationFault,
};
use crate::tokens::{
    ClosedToken, ImageState, OpaqueState, RecordedApplyPolicy, RecordedOriginState,
};

/// The apply pre-dispatch commitment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyIntentRow {
    session: String,
    generation: String,
    policy: RecordedApplyPolicy,
    assignments: u32,
    origin_state: RecordedOriginState,
    account: RecordedInfluence,
}

impl RecordedApplyIntentRow {
    /// One apply-intent row.
    #[must_use]
    pub const fn of(
        session: String,
        generation: String,
        policy: RecordedApplyPolicy,
        assignments: u32,
        origin_state: RecordedOriginState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            session,
            generation,
            policy,
            assignments,
            origin_state,
            account,
        }
    }

    /// The apply session identity, as spelled.
    #[must_use]
    pub fn session(&self) -> &str {
        &self.session
    }

    /// The generation identity, as spelled.
    #[must_use]
    pub fn generation(&self) -> &str {
        &self.generation
    }

    /// Which publication route authorized the apply.
    #[must_use]
    pub const fn policy(&self) -> RecordedApplyPolicy {
        self.policy
    }

    /// How many assignments the intent declares.
    #[must_use]
    pub const fn assignments(&self) -> u32 {
        self.assignments
    }

    /// Whether any assignment names an originating plan.
    #[must_use]
    pub const fn origin_state(&self) -> RecordedOriginState {
        self.origin_state
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedApplyIntentRow {
    const KIND: RecordKind = RecordKind::ApplyIntent;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.session.clone(),
            self.generation.clone(),
            self.policy.token().to_owned(),
            self.assignments.to_string(),
            self.origin_state.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            rows::digest(record, "session")?,
            rows::digest(record, "generation")?,
            rows::closed(record, "policy")?,
            rows::count(record, "assignments")?,
            rows::closed(record, "origin-state")?,
            rows::account(record),
        ))
    }
}

/// One target the apply was assigned to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyAssignment {
    ordinal: AssignmentOrdinal,
    target: OpaqueState,
    context: OpaqueState,
    image: String,
    image_state: ImageState,
    origins: u32,
    account: RecordedInfluence,
}

impl RecordedApplyAssignment {
    /// One apply-assignment row.
    #[must_use]
    pub const fn of(
        ordinal: AssignmentOrdinal,
        target: OpaqueState,
        context: OpaqueState,
        image: String,
        image_state: ImageState,
        origins: u32,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            ordinal,
            target,
            context,
            image,
            image_state,
            origins,
            account,
        }
    }

    /// Where this assignment sat in the intent.
    #[must_use]
    pub const fn ordinal(&self) -> AssignmentOrdinal {
        self.ordinal
    }

    /// What the projection holds in place of the target name.
    #[must_use]
    pub const fn target(&self) -> OpaqueState {
        self.target
    }

    /// What the projection holds in place of the target context.
    #[must_use]
    pub const fn context(&self) -> OpaqueState {
        self.context
    }

    /// The identity of the image this assignment applies, as spelled.
    #[must_use]
    pub fn image(&self) -> &str {
        &self.image
    }

    /// What the projection holds in place of the image itself.
    #[must_use]
    pub const fn image_state(&self) -> ImageState {
        self.image_state
    }

    /// How many originating plans this assignment declares.
    #[must_use]
    pub const fn origins(&self) -> u32 {
        self.origins
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedApplyAssignment {
    const KIND: RecordKind = RecordKind::ApplyAssignment;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.ordinal.get().to_string(),
            self.target.token().to_owned(),
            self.context.token().to_owned(),
            self.image.clone(),
            self.image_state.token().to_owned(),
            self.origins.to_string(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            AssignmentOrdinal::of(rows::count(record, "ordinal")?),
            rows::closed(record, "target")?,
            rows::closed(record, "context")?,
            rows::digest(record, "image")?,
            rows::closed(record, "image-state")?,
            rows::count(record, "origins")?,
            rows::account(record),
        ))
    }
}

/// One presented plan an assignment came from.
///
/// Duplicate occurrences are legal and retained: an admin may compose one plan into an
/// assignment more than once, and collapsing that to a set would report a mapping nobody made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedPlanOrigin {
    assignment: AssignmentOrdinal,
    ordinal: OriginOrdinal,
    receipt: String,
    presented: String,
    account: RecordedInfluence,
}

impl RecordedPlanOrigin {
    /// One plan-origin row.
    #[must_use]
    pub const fn of(
        assignment: AssignmentOrdinal,
        ordinal: OriginOrdinal,
        receipt: String,
        presented: String,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            assignment,
            ordinal,
            receipt,
            presented,
            account,
        }
    }

    /// Which assignment this origin belongs to.
    #[must_use]
    pub const fn assignment(&self) -> AssignmentOrdinal {
        self.assignment
    }

    /// Where this origin sat within its assignment.
    #[must_use]
    pub const fn ordinal(&self) -> OriginOrdinal {
        self.ordinal
    }

    /// The originating plan document identity, as spelled.
    #[must_use]
    pub fn receipt(&self) -> &str {
        &self.receipt
    }

    /// The originating approval-surface identity, as spelled.
    #[must_use]
    pub fn presented(&self) -> &str {
        &self.presented
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedPlanOrigin {
    const KIND: RecordKind = RecordKind::PlanOrigin;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.assignment.get().to_string(),
            self.ordinal.get().to_string(),
            self.receipt.clone(),
            self.presented.clone(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            AssignmentOrdinal::of(rows::count(record, "assignment")?),
            OriginOrdinal::of(rows::count(record, "ordinal")?),
            rows::digest(record, "receipt")?,
            rows::digest(record, "presented")?,
            rows::account(record),
        ))
    }
}

/// A non-empty run of originating plans.
///
/// Private field and one validating mint, so an empty run is unrepresentable rather than
/// merely refused at a caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownOrigins(Vec<RecordedPlanOrigin>);

impl KnownOrigins {
    /// Take a non-empty run whose ordinals are contiguous from zero.
    ///
    /// # Errors
    /// Refuses an empty run or a non-contiguous ordinal sequence.
    pub fn of(occurrences: Vec<RecordedPlanOrigin>) -> Result<Self, RelationFault> {
        if occurrences.is_empty() {
            return Err(RelationFault::CountDisagrees {
                kind: RecordKind::PlanOrigin.token(),
                declared: 1,
                present: 0,
            });
        }
        rows::contiguous(
            RecordKind::PlanOrigin,
            occurrences.iter().map(|row| row.ordinal().get()),
        )?;
        Ok(Self(occurrences))
    }

    /// The occurrences, in order.
    #[must_use]
    pub fn get(&self) -> &[RecordedPlanOrigin] {
        &self.0
    }

    /// How many occurrences the run holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the run is empty. Always false; present because the lint asks for it beside
    /// [`KnownOrigins::len`].
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Whether an assignment knows which presented plans it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OriginatingPlans {
    /// The document names no originating plan for this assignment.
    Unavailable,
    /// The document names at least one.
    Known(KnownOrigins),
}

impl OriginatingPlans {
    /// How many originating plans this assignment names.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Unavailable => 0,
            Self::Known(origins) => origins.len(),
        }
    }

    /// Whether the assignment names none.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// One assignment together with the plans it came from.
///
/// Composed by the aggregate from two record kinds; there is no public constructor, because a
/// caller assembling one by hand could pair an assignment with another assignment's origins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignedTarget {
    assignment: RecordedApplyAssignment,
    origins: OriginatingPlans,
}

impl AssignedTarget {
    /// The assignment row.
    #[must_use]
    pub const fn assignment(&self) -> &RecordedApplyAssignment {
        &self.assignment
    }

    /// The plans this assignment came from.
    #[must_use]
    pub const fn origins(&self) -> &OriginatingPlans {
        &self.origins
    }
}

/// One apply-intent document, as a typed model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyIntent {
    invocation: RecordedInvocation,
    intent: RecordedApplyIntentRow,
    assignments: Vec<AssignedTarget>,
    omissions: Vec<RecordedProjectionOmission>,
}

impl RecordedApplyIntent {
    /// Build the model from one document record stream, closing the records over one another.
    ///
    /// # Errors
    /// Refuses an unreadable row, a missing or repeated singleton, a declared count that
    /// disagrees with the rows present, an origin naming an assignment the document does not
    /// declare, or an origin state the assignments contradict.
    pub fn of_records(records: &[SkeletonRecord]) -> Result<Self, ModelRefusal> {
        let invocation: RecordedInvocation = rows::singleton_of(records)?;
        let intent: RecordedApplyIntentRow = rows::singleton_of(records)?;
        let assignment_rows: Vec<RecordedApplyAssignment> = rows::rows_of(records)?;
        let origin_rows: Vec<RecordedPlanOrigin> = rows::rows_of(records)?;
        let omissions: Vec<RecordedProjectionOmission> = rows::rows_of(records)?;

        rows::declared_count(
            RecordKind::ApplyIntent,
            u64::from(intent.assignments()),
            assignment_rows.len(),
        )?;
        rows::contiguous(
            RecordKind::ApplyAssignment,
            assignment_rows.iter().map(|row| row.ordinal().get()),
        )?;

        let declared = u32::try_from(assignment_rows.len()).unwrap_or(u32::MAX);
        for origin in &origin_rows {
            if origin.assignment().get() >= declared {
                return Err(RelationFault::DanglingAssignment {
                    assignment: origin.assignment().get(),
                }
                .into());
            }
        }

        let mut assignments = Vec::with_capacity(assignment_rows.len());
        for row in assignment_rows {
            let mine: Vec<RecordedPlanOrigin> = origin_rows
                .iter()
                .filter(|origin| origin.assignment() == row.ordinal())
                .cloned()
                .collect();
            rows::declared_count(
                RecordKind::ApplyAssignment,
                u64::from(row.origins()),
                mine.len(),
            )?;
            let origins = if mine.is_empty() {
                OriginatingPlans::Unavailable
            } else {
                OriginatingPlans::Known(KnownOrigins::of(mine)?)
            };
            assignments.push(AssignedTarget {
                assignment: row,
                origins,
            });
        }

        let with_origins = u32::try_from(
            assignments
                .iter()
                .filter(|target| !target.origins().is_empty())
                .count(),
        )
        .unwrap_or(u32::MAX);
        let declared_state = intent.origin_state();
        let consistent = match declared_state {
            RecordedOriginState::Known => with_origins > 0,
            RecordedOriginState::Unavailable => with_origins == 0,
        };
        if !consistent {
            return Err(RelationFault::OriginStateDisagrees {
                declared: declared_state.token(),
                with_origins,
            }
            .into());
        }

        Ok(Self {
            invocation,
            intent,
            assignments,
            omissions,
        })
    }

    /// Serialize the model, in the species kind order.
    ///
    /// # Errors
    /// Refuses whatever the grammar table refuses.
    pub fn to_records(&self) -> Result<Vec<SkeletonRecord>, RefusalReason> {
        let mut out = Vec::new();
        for kind in ApplyIntent::KINDS {
            match *kind {
                RecordKind::Invocation => out.push(self.invocation.to_record()?),
                RecordKind::ApplyIntent => out.push(self.intent.to_record()?),
                RecordKind::ApplyAssignment => {
                    for target in &self.assignments {
                        out.push(target.assignment().to_record()?);
                    }
                }
                RecordKind::PlanOrigin => {
                    for target in &self.assignments {
                        if let OriginatingPlans::Known(origins) = target.origins() {
                            for origin in origins.get() {
                                out.push(origin.to_record()?);
                            }
                        }
                    }
                }
                RecordKind::ProjectionOmission => {
                    for row in &self.omissions {
                        out.push(row.to_record()?);
                    }
                }
                _ => {}
            }
        }
        Ok(out)
    }

    /// The producing invocation.
    #[must_use]
    pub const fn invocation(&self) -> &RecordedInvocation {
        &self.invocation
    }

    /// The pre-dispatch commitment.
    #[must_use]
    pub const fn intent(&self) -> &RecordedApplyIntentRow {
        &self.intent
    }

    /// Every assignment, with the plans it came from.
    #[must_use]
    pub fn assignments(&self) -> &[AssignedTarget] {
        &self.assignments
    }

    /// Every population the projection declined to carry.
    #[must_use]
    pub fn omissions(&self) -> &[RecordedProjectionOmission] {
        &self.omissions
    }
}
