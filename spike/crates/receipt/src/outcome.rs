//! The `apply-outcome` species recorded model, and the availability of an outcome at all.
//!
//! An outcome document records only what execution knows. Its per-site rows reference an
//! assignment ordinal that lives in the INTENT, so that reference is checked by graph
//! correlation across documents rather than here: an outcome read on its own cannot know
//! which assignments existed.

use crate::format::{RefusalReason, SkeletonRecord};
use crate::grammar::RecordKind;
use crate::ids::ApplyIntentId;
use crate::model::{ApplyOutcome, Species};
use crate::reingested::{RecordedInfluence, Reingested};
use crate::rows::{
    self, AssignmentOrdinal, ModelRefusal, RecordedInvocation, RecordedLeaf, RecordedMember,
    RecordedProjectionOmission, RecordedRow, RecordedSite, SiteOutcomeOrdinal,
};
use crate::tokens::{
    ClosedToken, OpaqueState, RecordedDurableState, RecordedSiteStatus, RecordedTerminalState,
};

/// What the apply reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyOutcomeRow {
    intent: String,
    terminal: RecordedTerminalState,
    sites: u32,
    durable: RecordedDurableState,
    account: RecordedInfluence,
}

impl RecordedApplyOutcomeRow {
    /// One apply-outcome row.
    #[must_use]
    pub const fn of(
        intent: String,
        terminal: RecordedTerminalState,
        sites: u32,
        durable: RecordedDurableState,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            intent,
            terminal,
            sites,
            durable,
            account,
        }
    }

    /// The intent this outcome answers, as spelled.
    #[must_use]
    pub fn intent(&self) -> &str {
        &self.intent
    }

    /// The graceful terminal state the apply reached.
    #[must_use]
    pub const fn terminal(&self) -> RecordedTerminalState {
        self.terminal
    }

    /// How many site rows the outcome declares.
    #[must_use]
    pub const fn sites(&self) -> u32 {
        self.sites
    }

    /// Whether the terminal report itself reached durable storage.
    #[must_use]
    pub const fn durable(&self) -> RecordedDurableState {
        self.durable
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedApplyOutcomeRow {
    const KIND: RecordKind = RecordKind::ApplyOutcome;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.intent.clone(),
            self.terminal.token().to_owned(),
            self.sites.to_string(),
            self.durable.token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            rows::digest(record, "intent")?,
            rows::closed(record, "terminal")?,
            rows::count(record, "sites")?,
            rows::closed(record, "durable")?,
            rows::account(record),
        ))
    }
}

/// What a projection holds in place of a site's two byte channels.
///
/// One value rather than two adjacent slots of the same type, which is the shape a projection
/// can silently transpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordedChannels {
    stdout: OpaqueState,
    stderr: OpaqueState,
}

impl RecordedChannels {
    /// One channel pair.
    #[must_use]
    pub const fn of(stdout: OpaqueState, stderr: OpaqueState) -> Self {
        Self { stdout, stderr }
    }

    /// What the projection holds in place of the standard output.
    #[must_use]
    pub const fn stdout(self) -> OpaqueState {
        self.stdout
    }

    /// What the projection holds in place of the standard error.
    #[must_use]
    pub const fn stderr(self) -> OpaqueState {
        self.stderr
    }
}

/// What one site did during an apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSiteOutcome {
    ordinal: SiteOutcomeOrdinal,
    assignment: AssignmentOrdinal,
    site: RecordedSite,
    status: RecordedSiteStatus,
    tool_rc: Option<u32>,
    channels: RecordedChannels,
    account: RecordedInfluence,
}

impl RecordedSiteOutcome {
    /// One site-outcome row.
    #[must_use]
    pub const fn of(
        ordinal: SiteOutcomeOrdinal,
        assignment: AssignmentOrdinal,
        site: RecordedSite,
        status: RecordedSiteStatus,
        tool_rc: Option<u32>,
        channels: RecordedChannels,
        account: RecordedInfluence,
    ) -> Self {
        Self {
            ordinal,
            assignment,
            site,
            status,
            tool_rc,
            channels,
            account,
        }
    }

    /// Where this row sat in the outcome.
    #[must_use]
    pub const fn ordinal(&self) -> SiteOutcomeOrdinal {
        self.ordinal
    }

    /// Which assignment of the intent this site belongs to.
    #[must_use]
    pub const fn assignment(&self) -> AssignmentOrdinal {
        self.assignment
    }

    /// Which site executed.
    #[must_use]
    pub const fn site(&self) -> RecordedSite {
        self.site
    }

    /// What the site did.
    #[must_use]
    pub const fn status(&self) -> RecordedSiteStatus {
        self.status
    }

    /// The tool exit status, where one was observed.
    #[must_use]
    pub const fn tool_rc(&self) -> Option<u32> {
        self.tool_rc
    }

    /// What the projection holds in place of the two byte channels.
    #[must_use]
    pub const fn channels(&self) -> RecordedChannels {
        self.channels
    }

    /// Where this record stood relative to host contact.
    #[must_use]
    pub const fn account(&self) -> RecordedInfluence {
        self.account
    }
}

impl RecordedRow for RecordedSiteOutcome {
    const KIND: RecordKind = RecordKind::SiteOutcome;

    fn atoms(&self) -> Vec<String> {
        vec![
            self.ordinal.get().to_string(),
            self.assignment.get().to_string(),
            self.site.leaf_atom(),
            self.site.member_atom(),
            self.status.token().to_owned(),
            rows::spell_opt_count(self.tool_rc),
            self.channels.stdout().token().to_owned(),
            self.channels.stderr().token().to_owned(),
            self.account.token().to_owned(),
        ]
    }

    fn of_record(record: &SkeletonRecord) -> Result<Self, ModelRefusal> {
        rows::expect_kind(record, Self::KIND)?;
        Ok(Self::of(
            SiteOutcomeOrdinal::of(rows::count(record, "ordinal")?),
            AssignmentOrdinal::of(rows::count(record, "assignment")?),
            RecordedSite::of(
                RecordedLeaf::of(rows::count(record, "leaf")?),
                rows::opt_count(record, "member")?.map(RecordedMember::of),
            ),
            rows::closed(record, "status")?,
            rows::opt_count(record, "tool-rc")?,
            RecordedChannels::of(
                rows::closed(record, "stdout")?,
                rows::closed(record, "stderr")?,
            ),
            rows::account(record),
        ))
    }
}

/// One apply-outcome document, as a typed model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedApplyOutcome {
    invocation: RecordedInvocation,
    outcome: RecordedApplyOutcomeRow,
    sites: Vec<RecordedSiteOutcome>,
    omissions: Vec<RecordedProjectionOmission>,
}

impl RecordedApplyOutcome {
    /// Build the model from one document record stream, closing the records over one another.
    ///
    /// # Errors
    /// Refuses an unreadable row, a missing or repeated singleton, a declared site count that
    /// disagrees with the rows present, or a non-contiguous ordinal sequence.
    pub fn of_records(records: &[SkeletonRecord]) -> Result<Self, ModelRefusal> {
        let invocation: RecordedInvocation = rows::singleton_of(records)?;
        let outcome: RecordedApplyOutcomeRow = rows::singleton_of(records)?;
        let sites: Vec<RecordedSiteOutcome> = rows::rows_of(records)?;
        let omissions: Vec<RecordedProjectionOmission> = rows::rows_of(records)?;

        rows::declared_count(
            RecordKind::ApplyOutcome,
            u64::from(outcome.sites()),
            sites.len(),
        )?;
        rows::contiguous(
            RecordKind::SiteOutcome,
            sites.iter().map(|row| row.ordinal().get()),
        )?;

        Ok(Self {
            invocation,
            outcome,
            sites,
            omissions,
        })
    }

    /// Serialize the model, in the species kind order.
    ///
    /// # Errors
    /// Refuses whatever the grammar table refuses.
    pub fn to_records(&self) -> Result<Vec<SkeletonRecord>, RefusalReason> {
        let mut out = Vec::new();
        for kind in ApplyOutcome::KINDS {
            match *kind {
                RecordKind::Invocation => out.push(self.invocation.to_record()?),
                RecordKind::ApplyOutcome => out.push(self.outcome.to_record()?),
                RecordKind::SiteOutcome => {
                    for row in &self.sites {
                        out.push(row.to_record()?);
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

    /// What the apply reached.
    #[must_use]
    pub const fn outcome(&self) -> &RecordedApplyOutcomeRow {
        &self.outcome
    }

    /// Every site row.
    #[must_use]
    pub fn sites(&self) -> &[RecordedSiteOutcome] {
        &self.sites
    }

    /// Every population the projection declined to carry.
    #[must_use]
    pub fn omissions(&self) -> &[RecordedProjectionOmission] {
        &self.omissions
    }
}

/// The witness that a missing outcome was reached by correlation.
///
/// A private unit with no public constructor, so [`MissingOutcome`] cannot be assembled by a
/// caller who merely holds an intent identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Correlated;

/// An intent for which no outcome document was found.
///
/// This says only that: it is not a fabricated outcome, it is never serialized, and it implies
/// nothing about whether anything executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingOutcome {
    intent: ApplyIntentId,
    correlated: Correlated,
}

impl MissingOutcome {
    pub(crate) const fn of(intent: ApplyIntentId) -> Self {
        Self {
            intent,
            correlated: Correlated,
        }
    }

    /// The intent no outcome answered.
    #[must_use]
    pub const fn intent(&self) -> ApplyIntentId {
        self.intent
    }
}

/// Whether an intent has a recorded outcome.
///
/// [`OutcomeAvailability::Missing`] is constructed by graph correlation alone and is never a
/// document: absence of an outcome is a gap in the record, not a statement about the world.
#[derive(Debug)]
pub enum OutcomeAvailability {
    /// An outcome document answered this intent.
    Recorded(Reingested<RecordedApplyOutcome>),
    /// No outcome document answered this intent.
    Missing(MissingOutcome),
}

impl OutcomeAvailability {
    /// The word a report renders for this availability.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Recorded(_) => "recorded",
            Self::Missing(_) => "missing",
        }
    }
}
