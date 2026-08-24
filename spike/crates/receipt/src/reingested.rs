//! The read-back seal.
//!
//! Everything recovered from a published document wears [`Reingested`]. The wrapper has a
//! private field and a private mint, implements no dereference or borrow, and offers no
//! generic map or unwrap: decomposition answers another [`Reingested`] or a report-only
//! scalar. A value that came back from a document therefore cannot be handed to anything
//! that takes a live one.

use crate::model::{Projection, SignerTrust, Species};
use crate::reader::Receipt;

mod sealed {
    pub trait RecordedType {}
}

/// The closed set of things a document can yield back.
pub trait RecordedType: sealed::RecordedType {}

impl<D: Species, P: Projection, T: SignerTrust> sealed::RecordedType for Receipt<D, P, T> {}
impl<D: Species, P: Projection, T: SignerTrust> RecordedType for Receipt<D, P, T> {}

/// A value recovered from a published document.
#[derive(Debug)]
pub struct Reingested<T: RecordedType>(T);

impl<T: RecordedType> Reingested<T> {
    /// Seal a value the reader produced. Crate-private: the only way a `Reingested` comes
    /// into being is by reading a document.
    pub(crate) const fn seal(value: T) -> Self {
        Self(value)
    }

    /// Borrow for a report projection. Answers a shared borrow and nothing that outlives it,
    /// so a caller can describe the value without taking it.
    #[must_use]
    pub const fn as_report(&self) -> &T {
        &self.0
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
/// Flattened on the way in and never rehydrated: there is no accessor yielding a live
/// account, no conversion, and no join. Absent, unknown, malformed, or unverifiable material
/// reads [`RecordedInfluence::MostInfluenced`], which is the conservative direction — losing
/// this metadata can only make a reader more careful.
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
    use crate::model::{Plain, PlanReceipt, TrustedReceiptSigner};

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
