//! The closed states [`RecordedWhyFacts`](super::RecordedWhyFacts) is spelled in.
//!
//! Every one is an enum rather than a bool or a string, and they are kept INDEPENDENT of one
//! another on purpose. A document can be authenticated and incomplete, complete and unopenable,
//! openable and drifted; folding any pair into one word would make a reader infer the other half
//! from a value that never said it (`30Ra`: authentication, structural recovery, and authority
//! remain independent).

use crate::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId};

/// Which species a document is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordedSpecies {
    /// A plan receipt.
    Plan,
    /// An apply intent.
    ApplyIntent,
    /// An apply outcome.
    ApplyOutcome,
}

impl RecordedSpecies {
    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::ApplyIntent => "apply-intent",
            Self::ApplyOutcome => "apply-outcome",
        }
    }
}

/// One document's identity, by species.
///
/// The species and the identity travel together because a bare hex string is substitutable across
/// species: an intent id where a plan id belongs would join the wrong two documents and the graph
/// would still validate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordedDocumentId {
    /// A plan receipt's identity.
    Plan(PlanReceiptId),
    /// An apply intent's identity.
    ApplyIntent(ApplyIntentId),
    /// An apply outcome's identity.
    ApplyOutcome(ApplyOutcomeId),
}

impl RecordedDocumentId {
    /// Which species this identity belongs to.
    #[must_use]
    pub const fn species(&self) -> RecordedSpecies {
        match self {
            Self::Plan(_) => RecordedSpecies::Plan,
            Self::ApplyIntent(_) => RecordedSpecies::ApplyIntent,
            Self::ApplyOutcome(_) => RecordedSpecies::ApplyOutcome,
        }
    }

    /// The identity, as spelled. A receipt id is a controller-minted digest, not host material.
    #[must_use]
    pub fn hex(&self) -> String {
        match self {
            Self::Plan(id) => id.hex(),
            Self::ApplyIntent(id) => id.hex(),
            Self::ApplyOutcome(id) => id.hex(),
        }
    }
}

/// What the outer verification said, and nothing more.
///
/// Deliberately not folded into completeness or into detail availability: a document whose
/// signature checked can still be structurally partial, and one that failed verification can still
/// carry bounded recoverable structure a report may show. Rounding either way is what
/// `30Ra`'s independence clause refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationState {
    /// The signature checked against material controller policy named.
    Trusted,
    /// The document asserts its own signer and nothing independently establishes it.
    SelfAsserted,
    /// Verification did not succeed. Nothing here is an authenticated explanation.
    Failed,
}

impl AuthenticationState {
    /// Whether an explanation drawn from this document may be called authenticated.
    ///
    /// The ONE positive answer, and narrow on purpose.
    #[must_use]
    pub const fn is_authenticated(self) -> bool {
        matches!(self, Self::Trusted)
    }
}

/// Which projection the document is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionState {
    /// Structural content only; every detail slot reads withheld.
    Plain,
    /// Structural content plus one grouped detail region.
    Rich,
}

/// Whether the grouped detail region opened.
///
/// Separate from [`ProjectionState`] because they answer different questions: a rich document
/// whose region did not validate is not a plain one, and reporting it as plain would say the run
/// chose to withhold what a failure actually took away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailState {
    /// The region validated whole and its values are available.
    Available,
    /// The document carries no region, because it is plain.
    NotCarried,
    /// A region is declared and did not validate as a whole.
    Unavailable,
}

/// Whether one recorded value is present, and if not, why not.
///
/// The four absences are different facts and a report that merged them would invent a cause: a
/// bound that fired, a projection that withheld, a run that never held the value, and a region
/// that did not open are four different things to tell somebody.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialState {
    /// The document carries it.
    Held,
    /// A plain projection withheld it.
    WithheldPlain,
    /// A bound refused it. Nothing was truncated — the state IS the answer.
    OmittedByLimit,
    /// The run held no such value.
    Unavailable,
    /// The projection did not collect it.
    Uncollected,
    /// The document says it is there and the detail region did not release it.
    Undecodable,
}

impl MaterialState {
    /// Read one skeleton state word, with the region's own standing folded in.
    ///
    /// `captured` means the SKELETON says a value rides the region; whether the region actually
    /// released it is the region's answer, so a captured slot in an unavailable region is
    /// `Undecodable` rather than `Held`. That distinction is the whole reason this takes two
    /// inputs.
    #[must_use]
    pub const fn of(state: crate::tokens::OpaqueState, detail: DetailState) -> Self {
        use crate::tokens::OpaqueState;
        match state {
            OpaqueState::Captured => match detail {
                DetailState::Available => Self::Held,
                DetailState::NotCarried | DetailState::Unavailable => Self::Undecodable,
            },
            OpaqueState::WithheldPlain => Self::WithheldPlain,
            OpaqueState::OmittedLimit => Self::OmittedByLimit,
            OpaqueState::Unavailable => Self::Unavailable,
            OpaqueState::Uncollected => Self::Uncollected,
        }
    }

    /// Whether the value is actually in hand.
    #[must_use]
    pub const fn is_held(self) -> bool {
        matches!(self, Self::Held)
    }
}

/// Whether the causal closure this question needed was assembled whole.
///
/// `Complete` is the default because it is what an EMPTY sibling list means; a default of `Partial`
/// would make a closure that found everything look like one that had not started.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClosureCompleteness {
    /// Every document the rooted question required was read.
    #[default]
    Complete,
    /// At least one required sibling is missing, unreadable, partial, or disagreeing.
    Partial,
}

/// What is wrong with one required sibling.
///
/// A missing edge is INCOMPLETENESS and never a history: none of these arms implies success,
/// failure, or that no mutation happened (`30Ra`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiblingState {
    /// The graph names it and the store does not hold it.
    Missing(RecordedDocumentId),
    /// It is in the store and did not read back.
    Unreadable(RecordedDocumentId),
    /// It read back without completing.
    Partial(RecordedDocumentId),
    /// Two documents claim one identity and are not the same bytes.
    Disagreeing(RecordedDocumentId),
}

impl SiblingState {
    /// Which document this is about.
    #[must_use]
    pub const fn document(&self) -> &RecordedDocumentId {
        match self {
            Self::Missing(id)
            | Self::Unreadable(id)
            | Self::Partial(id)
            | Self::Disagreeing(id) => id,
        }
    }
}

/// How the current tree stands against the source a conclusion was drawn from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentSourceState {
    /// The current file reproduces the recorded bytes exactly.
    Matching,
    /// A file is there and its content has moved.
    Drifted,
    /// Nothing is at the recorded path.
    Absent,
    /// Something is there and could not be read.
    Unreadable,
    /// The comparison was never made — no path was recorded, or none was supplied.
    NotCompared,
}

impl CurrentSourceState {
    /// Whether a comparison actually happened, either way.
    ///
    /// The discriminator an address resolution turns on: a comparison that could not be made is
    /// not a comparison that disagreed, and a report must not present one as the other.
    #[must_use]
    pub const fn was_compared(self) -> bool {
        matches!(self, Self::Matching | Self::Drifted)
    }
}

/// Whether a conclusion was re-derived under current inputs, and if not, why not.
///
/// One arm today, and it is an honest one rather than an empty `Option`: the kernel seat that
/// would answer this is deliberately not built in this arc, and a model that said nothing would
/// leave a reader unable to tell "we checked and they agree" from "nobody checked". A fabricated
/// current disposition is exactly what this type exists to make unspellable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReDerivationState {
    /// No re-derivation was attempted; the engine support for one is not built.
    PendingKernelSupport,
}
