//! The live → recorded projection for the two apply-side species.
//!
//! `plan`'s Spine projection has a Spine to read. This lane has none: `dorc apply` returns
//! before a book is read, so an intent's content comes from the dispatch chain's own values and
//! an outcome's from what execution reported. Everything a document says about the INVOCATION
//! therefore arrives as [`ApplyInvocation`], a controller-owned value, rather than being derived
//! from something that happens to be lying around.
//!
//! Two properties are load-bearing and neither is expressible as a type:
//!
//! * rows are emitted in their species' `KINDS` order, so the model's own re-serialization
//!   reproduces this walk. A detail entry is keyed by its record's POSITION, and a walk that
//!   numbered in one order while the model emitted in another would enrich whichever row shared
//!   the integer, with the document still validating cleanly; and
//! * the assignment→record map is returned rather than re-derived, because
//!   [`crate::dispatch::PreparedApplyIntent::account_images`] needs it to prove the region
//!   about to be sealed carries each assignment's own image. A projection handing back records
//!   and details alone would leave that proof to be reconstructed from a guess.

use crate::apply::{
    RecordedApplyAssignment, RecordedApplyIntent, RecordedApplyIntentRow, RecordedPlanOrigin,
};
use crate::context::RecordedApplyContext;
use crate::dispatch::{MutationDispatched, PreparedApplyIntent, SessionApplyAssignment};
use crate::format::{RefusalReason, SkeletonRecord};
use crate::ids::ApplyIntentId;
use crate::limits::ReceiptLimits;
use crate::outcome::{
    RecordedApplyOutcome, RecordedApplyOutcomeRow, RecordedChannels, RecordedSiteOutcome,
};
use crate::overlay::{DocumentRows, OverlayEntry};
use crate::projection::OpaqueFieldTag;
use crate::reingested::RecordedInfluence;
use crate::rows::{
    AssignmentOrdinal, ModelRefusal, RecordedInvocation, RecordedSite, SiteOutcomeOrdinal,
};
use crate::tokens::{
    ImageState, OpaqueState, RecordedDurableState, RecordedInvocationMode, RecordedSiteStatus,
    RecordedTerminalState,
};

/// Why an apply-side projection produced no document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyProjectionRefusal {
    /// A row's atoms did not satisfy the grammar table.
    Grammar(RefusalReason),
    /// The records did not close over one another.
    Model(ModelRefusal),
    /// A site outcome named an assignment the cleared intent never declared.
    ///
    /// Recording it would attribute execution to a target nobody authorized, which is the
    /// mis-attribution tier rather than the merely-incomplete one.
    UndeclaredAssignment {
        /// The ordinal the site row named.
        assignment: u32,
    },
}

/// What one byte channel costs a document, and what its row says about it.
///
/// A value past the per-field bound, or one that would spend past the run's whole host-output
/// budget, is left out and SAID to be left out. `omitted-limit` is the word for a bound stopping
/// a carry, and it is a different statement from `unavailable` — the run held these bytes.
///
/// The budget is spent in row order, so which channel loses is the document's own order rather
/// than a size comparison across sites.
fn admit_channel(
    value: Option<&Vec<u8>>,
    spent: &mut u64,
    limits: &ReceiptLimits,
) -> (OpaqueState, Option<Vec<u8>>) {
    let Some(bytes) = value else {
        return (OpaqueState::Unavailable, None);
    };
    let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    let after = spent.saturating_add(measured);
    if !limits.opaque_field_bytes.admits(measured) || !limits.host_output_bytes.admits(after) {
        return (OpaqueState::OmittedLimit, None);
    }
    *spent = after;
    (OpaqueState::Captured, Some(bytes.clone()))
}

/// One value a row offers for one of its slots, absent where the run held none.
type Detail = (OpaqueFieldTag, Option<Vec<u8>>);

/// Whether a value the run HELD rides its slot, or the run held none.
///
/// Never whether a projection will carry it: narrowing to plain is what turns a held value into
/// `withheld-plain`, so this seat answers only the question it can.
const fn held(present: bool) -> OpaqueState {
    if present {
        OpaqueState::Captured
    } else {
        OpaqueState::Unavailable
    }
}

/// The invocation facts a document records, for a lane with no Spine holding them.
///
/// The account is the invocation record's own. Every apply-side record beside it takes its
/// account from the value describing that moment instead, because a standup's answers and an
/// execution's answers stand at different sides of host contact from the invocation and from
/// each other.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyInvocation {
    mode: RecordedInvocationMode,
    started: Option<u64>,
    target: Option<Vec<u8>>,
    attempt: u32,
    account: RecordedInfluence,
}

impl ApplyInvocation {
    /// Name one apply invocation.
    ///
    /// `target` is what the invocation SPELLED, where it spelled one. A multi-target apply names
    /// none here and its assignments carry the resolved answers; recording one of them as the
    /// invocation's own would be a claim the invocation did not make.
    #[must_use]
    pub const fn of(
        mode: RecordedInvocationMode,
        started: Option<u64>,
        target: Option<Vec<u8>>,
        attempt: u32,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            mode,
            started,
            target,
            attempt,
            account,
        }
    }

    /// The invocation row, and the detail value for whichever slot it captured.
    ///
    /// `argv` reads `uncollected` rather than being carried: the run holds it, and writing it
    /// decides a durable rendering that the plan lane also declines at its own seat. The two
    /// lanes agree so a successor funding argv funds it once.
    fn row(&self) -> (RecordedInvocation, [Detail; 1]) {
        let row = RecordedInvocation::of(
            self.mode,
            self.started,
            OpaqueState::Uncollected,
            held(self.target.is_some()),
            self.attempt,
            self.account,
        );
        (row, [(OpaqueFieldTag::TargetName, self.target.clone())])
    }
}

/// One run's apply-intent document: the typed model, the exact records, the detail values, and
/// which record each assignment occupies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedApplyIntent {
    model: RecordedApplyIntent,
    records: Vec<SkeletonRecord>,
    details: Vec<OverlayEntry>,
    assignment_records: Vec<(AssignmentOrdinal, u64)>,
}

impl ProjectedApplyIntent {
    /// The typed model.
    #[must_use]
    pub const fn model(&self) -> &RecordedApplyIntent {
        &self.model
    }

    /// The exact records, in the canonical order, ready to become a skeleton.
    #[must_use]
    pub fn records(&self) -> &[SkeletonRecord] {
        &self.records
    }

    /// The detail values, one per captured slot.
    #[must_use]
    pub fn details(&self) -> &[OverlayEntry] {
        &self.details
    }

    /// Which record one assignment occupies.
    ///
    /// The map the image accounting consumes. It is recorded by the walk that emitted the rows
    /// rather than computed from an offset, so a record inserted ahead of the assignments moves
    /// both together.
    #[must_use]
    pub fn record_of(&self, ordinal: AssignmentOrdinal) -> Option<u64> {
        self.assignment_records
            .iter()
            .find(|(assignment, _)| *assignment == ordinal)
            .map(|(_, record)| *record)
    }

    /// Take the parts, for a caller assembling a document.
    #[must_use]
    pub fn into_parts(self) -> (RecordedApplyIntent, Vec<SkeletonRecord>, Vec<OverlayEntry>) {
        (self.model, self.records, self.details)
    }
}

/// Project the recorded apply-intent model from one prepared intent.
///
/// `resolved` is the account for everything the STANDUP produced — the intent row, its
/// assignments, and their origins. It is separate from the invocation's own because a resolved
/// target is an answer from the far side of a connection, whatever the invocation was.
///
/// # Errors
/// Refuses a row the grammar table rejects and a record set that does not close over itself.
pub fn project_apply_intent(
    intent: &PreparedApplyIntent,
    invocation: &ApplyInvocation,
    resolved: RecordedInfluence,
) -> Result<ProjectedApplyIntent, ApplyProjectionRefusal> {
    let mut rows = DocumentRows::default();
    let (row, values) = invocation.row();
    push(&mut rows, &row, &values)?;

    let assignments = u32::try_from(intent.assignments().len()).unwrap_or(u32::MAX);
    push(
        &mut rows,
        &RecordedApplyIntentRow::of(
            intent.session().hex(),
            intent.generation().hex(),
            intent.policy().token(),
            assignments,
            intent.origin_state(),
            resolved,
        ),
        &[],
    )?;

    // ONE walk numbers the assignments and records where each landed. The map is what the image
    // accounting keys by, and a second walk deriving it from an offset would agree with this one
    // only until a record was added above.
    let mut assignment_records: Vec<(AssignmentOrdinal, u64)> = Vec::new();
    for assignment in intent.assignments() {
        assignment_records.push((assignment.ordinal(), rows.next_record()));
        let (row, values) = assignment_row(assignment, resolved);
        push(&mut rows, &row, &values)?;
    }
    for assignment in intent.assignments() {
        for occurrence in assignment.origins().occurrences() {
            push(
                &mut rows,
                &RecordedPlanOrigin::of(
                    assignment.ordinal(),
                    occurrence.ordinal(),
                    occurrence.receipt().hex(),
                    occurrence.presented().hex(),
                    resolved,
                ),
                &[],
            )?;
        }
    }

    let (records, details) = rows.into_parts();
    let model = RecordedApplyIntent::of_records(&records).map_err(ApplyProjectionRefusal::Model)?;
    Ok(ProjectedApplyIntent {
        model,
        records,
        details,
        assignment_records,
    })
}

/// One assignment's row, and the three values its slots may carry.
///
/// The image rides its slot as the exact canonical encoding, which is the same byte-image the
/// accounting compares against — the region proves the image is present by BEING it. The
/// destination and the remaining resolved axes ride two slots of the same row, so a reader
/// recombining them cannot pair one assignment's host with another's siting.
fn assignment_row(
    assignment: &SessionApplyAssignment,
    account: RecordedInfluence,
) -> (RecordedApplyAssignment, [Detail; 3]) {
    let context = assignment.context();
    let destination = context.destination().as_bytes().to_vec();
    let recorded = RecordedApplyContext::of(
        context.account().as_bytes().to_vec(),
        context.namespace().as_bytes().to_vec(),
        context.working_directory().as_bytes().to_vec(),
        context.environment_policy().as_bytes().to_vec(),
        context.credential_scope().as_bytes().to_vec(),
    );
    let origins = u32::try_from(assignment.origins().len()).unwrap_or(u32::MAX);
    let row = RecordedApplyAssignment::of(
        assignment.ordinal(),
        held(!destination.is_empty()),
        OpaqueState::Captured,
        assignment.image().id().hex(),
        ImageState::Captured,
        origins,
        account,
    );
    (
        row,
        [
            (
                OpaqueFieldTag::TargetName,
                (!destination.is_empty()).then_some(destination),
            ),
            (OpaqueFieldTag::ApplyContext, Some(recorded.encode())),
            (
                OpaqueFieldTag::ApplyArtifactImage,
                Some(assignment.image().encode().to_vec()),
            ),
        ],
    )
}

/// What one site did during an apply, as execution reported it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplySiteReport {
    assignment: AssignmentOrdinal,
    site: RecordedSite,
    status: RecordedSiteStatus,
    tool_rc: Option<u32>,
    stdout: Option<Vec<u8>>,
    stderr: Option<Vec<u8>>,
    account: RecordedInfluence,
}

impl ApplySiteReport {
    /// Report one site.
    ///
    /// `stdout` and `stderr` are the bytes the run ALREADY holds under its own collection
    /// policy. Absent means the run holds none, which is a different statement from a
    /// projection declining to carry what it has.
    #[must_use]
    pub const fn of(
        assignment: AssignmentOrdinal,
        site: RecordedSite,
        status: RecordedSiteStatus,
        tool_rc: Option<u32>,
        stdout: Option<Vec<u8>>,
        stderr: Option<Vec<u8>>,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            assignment,
            site,
            status,
            tool_rc,
            stdout,
            stderr,
            account,
        }
    }
}

/// What one apply reached, before it becomes a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyOutcomeReport {
    intent: ApplyIntentId,
    terminal: RecordedTerminalState,
    durable: RecordedDurableState,
    sites: Vec<ApplySiteReport>,
    account: RecordedInfluence,
}

impl ApplyOutcomeReport {
    /// Report one apply's terminal state and whatever per-site detail it has.
    #[must_use]
    pub const fn of(
        intent: ApplyIntentId,
        terminal: RecordedTerminalState,
        durable: RecordedDurableState,
        sites: Vec<ApplySiteReport>,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            intent,
            terminal,
            durable,
            sites,
            account,
        }
    }
}

/// One run's apply-outcome document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedApplyOutcome {
    model: RecordedApplyOutcome,
    records: Vec<SkeletonRecord>,
    details: Vec<OverlayEntry>,
}

impl ProjectedApplyOutcome {
    /// The typed model.
    #[must_use]
    pub const fn model(&self) -> &RecordedApplyOutcome {
        &self.model
    }

    /// The exact records, in the canonical order.
    #[must_use]
    pub fn records(&self) -> &[SkeletonRecord] {
        &self.records
    }

    /// The detail values, one per captured slot.
    #[must_use]
    pub fn details(&self) -> &[OverlayEntry] {
        &self.details
    }

    /// Take the parts, for a caller assembling a document.
    #[must_use]
    pub fn into_parts(self) -> (RecordedApplyOutcome, Vec<SkeletonRecord>, Vec<OverlayEntry>) {
        (self.model, self.records, self.details)
    }
}

/// Project the recorded apply-outcome model from what execution reported.
///
/// `dispatched` is consulted rather than decorative: a site row naming an assignment the cleared
/// intent never declared is refused here, at the seat that could still tell.
///
/// `limits` bounds what the byte channels may cost. A host's output is unbounded by nature, and a
/// document that carried all of it would be one its own reader refuses whole — so the budget is
/// spent per row and what it will not cover is recorded as omitted rather than dropped silently.
///
/// # Errors
/// Refuses a site naming an undeclared assignment, a row the grammar table rejects, and a record
/// set that does not close over itself.
pub fn project_apply_outcome(
    dispatched: &MutationDispatched,
    report: &ApplyOutcomeReport,
    invocation: &ApplyInvocation,
    limits: &ReceiptLimits,
) -> Result<ProjectedApplyOutcome, ApplyProjectionRefusal> {
    for site in &report.sites {
        if !dispatched.declares(site.assignment) {
            return Err(ApplyProjectionRefusal::UndeclaredAssignment {
                assignment: site.assignment.get(),
            });
        }
    }

    let mut rows = DocumentRows::default();
    let (row, values) = invocation.row();
    push(&mut rows, &row, &values)?;

    let sites = u32::try_from(report.sites.len()).unwrap_or(u32::MAX);
    push(
        &mut rows,
        &RecordedApplyOutcomeRow::of(
            report.intent.hex(),
            report.terminal,
            sites,
            report.durable,
            report.account,
        ),
        &[],
    )?;

    let mut spent: u64 = 0;
    for (position, site) in report.sites.iter().enumerate() {
        let ordinal = SiteOutcomeOrdinal::of(u32::try_from(position).unwrap_or(u32::MAX));
        let (out_state, out_bytes) = admit_channel(site.stdout.as_ref(), &mut spent, limits);
        let (err_state, err_bytes) = admit_channel(site.stderr.as_ref(), &mut spent, limits);
        push(
            &mut rows,
            &RecordedSiteOutcome::of(
                ordinal,
                site.assignment,
                site.site,
                site.status,
                site.tool_rc,
                RecordedChannels::of(out_state, err_state),
                site.account,
            ),
            &[
                (OpaqueFieldTag::Stdout, out_bytes),
                (OpaqueFieldTag::Stderr, err_bytes),
            ],
        )?;
    }

    let (records, details) = rows.into_parts();
    let model =
        RecordedApplyOutcome::of_records(&records).map_err(ApplyProjectionRefusal::Model)?;
    Ok(ProjectedApplyOutcome {
        model,
        records,
        details,
    })
}

fn push<R: crate::rows::RecordedRow>(
    rows: &mut DocumentRows,
    row: &R,
    values: &[(OpaqueFieldTag, Option<Vec<u8>>)],
) -> Result<(), ApplyProjectionRefusal> {
    rows.push(row, values)
        .map_err(ApplyProjectionRefusal::Grammar)
}
