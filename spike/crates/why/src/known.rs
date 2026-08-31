//! The two nested wrappers every slot of the reconstruction is spelled through (`30V` §3).
//!
//! They answer different questions and merging them is the named builder failure-mode:
//!
//! - [`Known`] — has the ENGINE been wired to answer this at all ([`Known::KnowableNYI`]), or can
//!   nothing ever answer it ([`Known::Unknowable`]), or is there an answer.
//! - [`Held`] — the answer itself, which may be an AFFIRMATIVE not-knowing. Every
//!   failed-to-know-but-TRIED state lives here, inside `Knowable`, as domain data.
//!
//! # Why laundering is hard rather than discouraged
//!
//! `30V` §3 names laundering NYI upward into `Unknowable` as the failure this wrapper exists to
//! prevent, so the two are separated by construction: there is no `From`, no `Into`, and no method
//! on either that yields the other. [`Known::nyi`] is the sole NYI mint, and the census over a real
//! reconstruction (`tests/reconstruction.rs`) is what says no slot ships as one.
//!
//! [`Known::Unknowable`]'s reason is taken by the constructor and DROPPED: `30V` §3 rules it
//! compile-time material for reviewers, not a runtime value, so it cannot reach a render and be
//! mistaken for something the world could have told us.

/// Whether a slot is answerable at all, above the domain's own plane (`30V` §3, `[TYPED]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Known<T> {
    /// There is an answer. It may itself be an affirmative not-knowing — see [`Held`].
    Knowable(Held<T>),
    /// The engine holds this and nothing has piped it here yet. ARGUMENT-FREE on purpose: a reason
    /// would make an unbuilt slot look like a considered one, and the census is what tracks these.
    KnowableNYI,
    /// Nothing can ever answer this, by construction, forever.
    Unknowable,
}

/// Why a slot can never be answered. Consumed by [`Known::unknowable`] and never stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnknowableReason {
    /// The question is about a world-moment no longer reachable from any record.
    MomentHasPassed,
    /// The value was never representable in any receipt version, present or future.
    NotRepresentable,
}

/// What the carrier had to say about one slot.
///
/// The absences are kept APART deliberately: `report::MaterialState`'s own doc records that merging
/// them invents a cause, and a reader is owed the difference between a bound that fired, a
/// projection that withheld, a run that held no such value, and a region that would not open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Held<T> {
    /// The answer.
    Present(T),
    /// Affirmatively known to be absent from THIS carrier — neither NYI (piping cannot help
    /// immutable bytes) nor unknowable (a later receipt version can carry it).
    AbsentFromCarrier(CarrierAbsence),
    /// The carrier holds it and would not release it here.
    Withheld(WithholdReason),
    /// Somebody tried to establish it and could not.
    CouldNotTell(CantTell),
}

/// Which absence, and — the half the durable-gap audit turns on — WHOSE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CarrierAbsence {
    /// The run held no such value (`report::MaterialState::Unavailable`).
    RunHeldNoValue,
    /// The receipt projection did not collect it (`report::MaterialState::Uncollected`).
    ProjectionUncollected,
    /// The DOCUMENT carries it and `dorc_receipt::report` does not project it. The distinct cause
    /// the audit splits on: closing this needs no durable change, only report-API coverage.
    ReportApiLacks,
}

/// Why a carrier would not release a value it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WithholdReason {
    /// A plain projection withheld it (`report::MaterialState::WithheldPlain`).
    PlainProjection,
    /// A bound refused it; nothing was truncated (`report::MaterialState::OmittedByLimit`).
    BoundRefused,
    /// The document says it is there and the detail region did not open
    /// (`report::MaterialState::Undecodable`).
    RegionUnavailable,
    /// The encoder gated the value at the EXIT boundary (`30V` §3's encoder-withholding event).
    EncoderGated,
}

/// A tried-and-failed establishment. Plural and domain-owned, per `30V` §3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CantTell {
    /// A comparison the edge never made — no observation was supplied for this source.
    ComparisonNotMade,
    /// The recorded material stops short of what the slot needs.
    Truncated,
}

impl<T> Known<T> {
    /// The answer.
    #[must_use]
    pub const fn present(value: T) -> Self {
        Self::Knowable(Held::Present(value))
    }

    /// Affirmatively absent from this carrier.
    #[must_use]
    pub const fn absent(absence: CarrierAbsence) -> Self {
        Self::Knowable(Held::AbsentFromCarrier(absence))
    }

    /// The DOCUMENT holds it and the report API does not project it — the audit's second cause.
    #[must_use]
    pub const fn report_api_lacks() -> Self {
        Self::absent(CarrierAbsence::ReportApiLacks)
    }

    /// Not piped here yet. THE sole NYI mint; its call sites are lexically fenced.
    #[must_use]
    pub const fn nyi() -> Self {
        Self::KnowableNYI
    }

    /// Unanswerable forever. `reason` is for the reviewer reading this call site and is dropped
    /// here, so it can never reach a render as though the world had spoken.
    #[must_use]
    pub const fn unknowable(_reason: UnknowableReason) -> Self {
        Self::Unknowable
    }

    /// The value, where there is one. Deliberately not `Option`-shaped sugar over the whole
    /// wrapper: a consumer that wants to TELL the states apart must match, and this is only for
    /// consumers that genuinely want the value or nothing.
    #[must_use]
    pub const fn value(&self) -> Option<&T> {
        match self {
            Self::Knowable(Held::Present(value)) => Some(value),
            Self::Knowable(_) | Self::KnowableNYI | Self::Unknowable => None,
        }
    }

    /// Whether this slot is unbuilt rather than unanswered — what the NYI census counts.
    #[must_use]
    pub const fn is_nyi(&self) -> bool {
        matches!(self, Self::KnowableNYI)
    }
}
