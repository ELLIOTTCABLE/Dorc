//! The read-back seal.
//!
//! Everything recovered from a published document wears [`Reingested`]. The wrapper has a
//! private field and a private mint, implements no dereference or borrow, and offers no
//! generic map, unwrap, or accessor of any kind: there is deliberately no way to ask a
//! `Reingested<T>` for its `T`. Decomposition is per-species and answers either another
//! [`Reingested`] or a report-only scalar, so a value that came back from a document cannot be
//! handed to anything that takes a live one.
//!
//! The seal is structural rather than a rule about which types join [`RecordedType`]: even if
//! a live type were admitted to that set tomorrow, no accessor exists that would hand it out.

use crate::apply::RecordedApplyIntent;
use crate::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId, PresentedPlanId};
use crate::model::{ApplyIntent, ApplyOutcome, PlanReceipt, Projection, SignerTrust, Species};
use crate::outcome::RecordedApplyOutcome;
use crate::plan::{RecordedPlanReceipt, RecordedSource};
use crate::reader::Receipt;
use crate::rows::{ModelRefusal, RecordedProjectionOmission};
use crate::tokens::{
    RecordedApplyPolicy, RecordedInvocationMode, RecordedOmissionReason, RecordedOriginState,
    RecordedSourceRole, RecordedSpineSpecies, RecordedTerminalState,
};

mod sealed {
    pub trait RecordedType {}
}

/// The closed set of things a document can yield back.
pub trait RecordedType: sealed::RecordedType {}

impl<D: Species, P: Projection, T: SignerTrust> sealed::RecordedType for Receipt<D, P, T> {}
impl<D: Species, P: Projection, T: SignerTrust> RecordedType for Receipt<D, P, T> {}

impl sealed::RecordedType for RecordedPlanReceipt {}
impl RecordedType for RecordedPlanReceipt {}
impl sealed::RecordedType for RecordedApplyIntent {}
impl RecordedType for RecordedApplyIntent {}
impl sealed::RecordedType for RecordedApplyOutcome {}
impl RecordedType for RecordedApplyOutcome {}
impl sealed::RecordedType for RecordedSource {}
impl RecordedType for RecordedSource {}
impl sealed::RecordedType for RecordedProjectionOmission {}
impl RecordedType for RecordedProjectionOmission {}

/// A value recovered from a published document.
#[derive(Debug)]
pub struct Reingested<T: RecordedType>(T);

impl<T: RecordedType> Reingested<T> {
    /// Seal a value the reader produced. Crate-private: the only way a `Reingested` comes into
    /// being is by reading a document.
    pub(crate) const fn seal(value: T) -> Self {
        Self(value)
    }
}

/// Two sealed values compare without either being unwrapped.
///
/// Comparison rather than extraction is what lets graph correlation tell one document read twice
/// from two documents sharing an identity, with no accessor handing the inner value out.
impl<T: RecordedType + PartialEq> PartialEq for Reingested<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: RecordedType + Eq> Eq for Reingested<T> {}

/// Cloning a sealed value answers another sealed value. Nothing is extracted, so a report may
/// hold a second handle on one recorded document without the seal being weakened.
impl<T: RecordedType + Clone> Clone for Reingested<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D: Species, P: Projection, T: SignerTrust> Reingested<Receipt<D, P, T>> {
    /// The word a report renders for this document's provenance.
    #[must_use]
    pub const fn signer_provenance(&self) -> &'static str {
        T::TOKEN
    }

    /// How many records the document carries.
    #[must_use]
    pub fn record_count(&self) -> usize {
        self.0.skeleton().records.len()
    }

    /// The document's own identity, as spelled.
    #[must_use]
    pub fn receipt_id_hex(&self) -> String {
        self.0.skeleton().receipt_id.clone()
    }

    /// The signing provider's identity, as spelled.
    #[must_use]
    pub fn signing_key_id_hex(&self) -> String {
        self.0.skeleton().signing_key_id.clone()
    }
}

impl<P: Projection, T: SignerTrust> Reingested<Receipt<PlanReceipt, P, T>> {
    /// The typed model of this document.
    ///
    /// # Errors
    /// Refuses a record stream that parsed but does not close over itself.
    pub fn model(&self) -> Result<Reingested<RecordedPlanReceipt>, ModelRefusal> {
        RecordedPlanReceipt::of_records(&self.0.skeleton().records).map(Reingested::seal)
    }

    /// This document's own identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<PlanReceiptId> {
        PlanReceiptId::of_hex(&self.0.skeleton().receipt_id)
    }
}

impl<P: Projection, T: SignerTrust> Reingested<Receipt<ApplyIntent, P, T>> {
    /// The typed model of this document.
    ///
    /// # Errors
    /// Refuses a record stream that parsed but does not close over itself.
    pub fn model(&self) -> Result<Reingested<RecordedApplyIntent>, ModelRefusal> {
        RecordedApplyIntent::of_records(&self.0.skeleton().records).map(Reingested::seal)
    }

    /// This document's own identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<ApplyIntentId> {
        ApplyIntentId::of_hex(&self.0.skeleton().receipt_id)
    }
}

impl<P: Projection, T: SignerTrust> Reingested<Receipt<ApplyOutcome, P, T>> {
    /// The typed model of this document.
    ///
    /// # Errors
    /// Refuses a record stream that parsed but does not close over itself.
    pub fn model(&self) -> Result<Reingested<RecordedApplyOutcome>, ModelRefusal> {
        RecordedApplyOutcome::of_records(&self.0.skeleton().records).map(Reingested::seal)
    }

    /// This document's own identity.
    #[must_use]
    pub fn receipt_id(&self) -> Option<ApplyOutcomeId> {
        ApplyOutcomeId::of_hex(&self.0.skeleton().receipt_id)
    }
}

impl Reingested<RecordedPlanReceipt> {
    /// Which invocation shape produced the document.
    #[must_use]
    pub const fn mode(&self) -> RecordedInvocationMode {
        self.0.invocation().mode()
    }

    /// Where the invocation stood relative to host contact.
    #[must_use]
    pub const fn invocation_account(&self) -> RecordedInfluence {
        self.0.invocation().account()
    }

    /// Every acquired source, each still sealed.
    #[must_use]
    pub fn sources(&self) -> Vec<Reingested<RecordedSource>> {
        self.0
            .sources()
            .iter()
            .cloned()
            .map(Reingested::seal)
            .collect()
    }

    /// Every population the projection declined to carry, each still sealed.
    #[must_use]
    pub fn omissions(&self) -> Vec<Reingested<RecordedProjectionOmission>> {
        self.0
            .omissions()
            .iter()
            .cloned()
            .map(Reingested::seal)
            .collect()
    }

    /// How many site decisions the document carries.
    #[must_use]
    pub fn site_count(&self) -> usize {
        self.0.sites().len()
    }

    /// How many region decisions the document carries.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.0.regions().len()
    }

    /// The approval-surface identity this plan presented, where it recorded one.
    #[must_use]
    pub fn presented_plan(&self) -> Option<PresentedPlanId> {
        PresentedPlanId::of_hex(self.0.presented()?.presented_plan())
    }
}

impl Reingested<RecordedSource> {
    /// Where this source sat in the acquired-source table.
    #[must_use]
    pub fn ordinal(&self) -> u32 {
        self.0.ordinal().get()
    }

    /// What the source was to the run.
    #[must_use]
    pub const fn role(&self) -> RecordedSourceRole {
        self.0.role()
    }

    /// The content digest, as spelled.
    #[must_use]
    pub fn digest(&self) -> String {
        self.0.digest().to_owned()
    }

    /// How many bytes the source held.
    #[must_use]
    pub const fn bytes(&self) -> u64 {
        self.0.bytes()
    }
}

impl Reingested<RecordedProjectionOmission> {
    /// Which in-memory decision species went uncarried.
    #[must_use]
    pub const fn species(&self) -> RecordedSpineSpecies {
        self.0.species()
    }

    /// How many of it there were.
    #[must_use]
    pub const fn count(&self) -> u32 {
        self.0.count()
    }

    /// Why the projection did not carry it.
    #[must_use]
    pub const fn reason(&self) -> RecordedOmissionReason {
        self.0.reason()
    }
}

impl Reingested<RecordedApplyIntent> {
    /// Which publication route authorized the apply.
    #[must_use]
    pub const fn policy(&self) -> RecordedApplyPolicy {
        self.0.intent().policy()
    }

    /// Whether any assignment names an originating plan.
    #[must_use]
    pub const fn origin_state(&self) -> RecordedOriginState {
        self.0.intent().origin_state()
    }

    /// How many assignments the intent carries.
    #[must_use]
    pub fn assignment_count(&self) -> usize {
        self.0.assignments().len()
    }

    /// Every originating plan document identity this intent names, with duplicates retained.
    #[must_use]
    pub fn origin_receipts(&self) -> Vec<PlanReceiptId> {
        let mut out = Vec::new();
        for target in self.0.assignments() {
            if let crate::apply::OriginatingPlans::Known(origins) = target.origins() {
                for origin in origins.get() {
                    if let Some(id) = PlanReceiptId::of_hex(origin.receipt()) {
                        out.push(id);
                    }
                }
            }
        }
        out
    }
}

impl Reingested<RecordedApplyOutcome> {
    /// The graceful terminal state the apply reached.
    #[must_use]
    pub const fn terminal(&self) -> RecordedTerminalState {
        self.0.outcome().terminal()
    }

    /// How many site rows the outcome carries.
    #[must_use]
    pub fn site_count(&self) -> usize {
        self.0.sites().len()
    }

    /// The intent this outcome answers.
    #[must_use]
    pub fn intent(&self) -> Option<ApplyIntentId> {
        ApplyIntentId::of_hex(self.0.outcome().intent())
    }
}

/// How a recorded conclusion and one derived under current inputs relate.
///
/// Four states, and they never substitute for one another. Disagreement is a finding that
/// keeps both values, never a resolution that picks one.
#[derive(Debug)]
pub enum RecordedCurrent<R: RecordedType, C> {
    /// Only the document has it.
    RecordedOnly(Reingested<R>),
    /// Only the current derivation has it.
    CurrentOnly(C),
    /// Both have it and they agree.
    BothAgreeing {
        /// What the document recorded.
        recorded: Reingested<R>,
        /// What the current derivation produced.
        current: C,
    },
    /// Both have it and they disagree.
    BothDisagreeing {
        /// What the document recorded.
        recorded: Reingested<R>,
        /// What the current derivation produced.
        current: C,
    },
}

impl<R: RecordedType, C> RecordedCurrent<R, C> {
    /// The word a report renders for this comparison.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::RecordedOnly(_) => "recorded-only",
            Self::CurrentOnly(_) => "current-only",
            Self::BothAgreeing { .. } => "both-agreeing",
            Self::BothDisagreeing { .. } => "both-disagreeing",
        }
    }

    /// Whether this comparison is a finding a report must surface.
    #[must_use]
    pub const fn is_finding(&self) -> bool {
        matches!(self, Self::BothDisagreeing { .. })
    }
}

/// An influence grade as a document carries it.
///
/// Flattened on the way in and never rehydrated: there is no accessor yielding a live account,
/// no conversion, and no join. Absent, unknown, malformed, or unverifiable material reads
/// [`RecordedInfluence::MostInfluenced`], which is the conservative direction — losing this
/// metadata can only make a reader more careful.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordedInfluence {
    /// The document recorded that the object was computed before host contact.
    AuthoredBeforeContact,
    /// The document recorded that host-reported material reached the object.
    HostInfluenced,
    /// The document recorded nothing usable, or recorded that the account was untracked.
    MostInfluenced,
}

impl RecordedInfluence {
    /// Read one grade from a document's closed vocabulary.
    #[must_use]
    pub fn of_token(token: Option<&str>) -> Self {
        match token {
            Some("authored-before-contact") => Self::AuthoredBeforeContact,
            Some("host-influenced") => Self::HostInfluenced,
            _ => Self::MostInfluenced,
        }
    }

    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::AuthoredBeforeContact => "authored-before-contact",
            Self::HostInfluenced => "host-influenced",
            Self::MostInfluenced => "untracked",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Plain, TrustedReceiptSigner};

    #[test]
    fn an_unreadable_grade_reads_at_the_most_influenced_point() {
        // Losing this metadata must only ever make a reader more careful, so every way of
        // failing to read one lands at the same conservative point.
        assert_eq!(
            RecordedInfluence::of_token(None),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("")),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("untracked")),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("not-a-grade")),
            RecordedInfluence::MostInfluenced
        );
        assert_eq!(
            RecordedInfluence::of_token(Some("AUTHORED-BEFORE-CONTACT")),
            RecordedInfluence::MostInfluenced,
            "case is exact"
        );
    }

    #[test]
    fn a_recorded_grade_reads_back_as_itself() {
        for grade in [
            RecordedInfluence::AuthoredBeforeContact,
            RecordedInfluence::HostInfluenced,
            RecordedInfluence::MostInfluenced,
        ] {
            assert_eq!(RecordedInfluence::of_token(Some(grade.token())), grade);
        }
    }

    #[test]
    fn only_disagreement_is_a_finding() {
        let agreeing: RecordedCurrent<Receipt<PlanReceipt, Plain, TrustedReceiptSigner>, u8> =
            RecordedCurrent::CurrentOnly(1);
        assert!(!agreeing.is_finding());
        assert_eq!(agreeing.token(), "current-only");
    }
}
