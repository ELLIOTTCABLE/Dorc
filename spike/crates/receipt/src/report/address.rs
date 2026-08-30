//! Resolving a `path:N` address against recorded source and recorded provenance.
//!
//! # The rule, and why it is this narrow
//!
//! A source address means the SAME physical path and the SAME line number in the current and
//! recorded book, and Dorc never guesses that a moved line is the same operation
//! (`30R:receipt-rooted-attention-and-cli`). So the comparison is: recorded physical line N against
//! current physical line N of the same source, byte for byte. Identical admits the recorded site.
//! Anything else refuses the address-SPECIFIC conclusion and says why — while every unrelated
//! receipt fact still renders, because one unanswerable address is not a reason to stop explaining
//! the rest.
//!
//! There is deliberately no content-similarity search, no nearest-match, and no fuzzy window. Each
//! would answer confidently about a line the author moved, and a confident wrong attribution is
//! `271:rul-sin-ordering`'s worst rung.
//!
//! # The byte domain
//!
//! LF indexes physical lines and a CR in CRLF is one of the line's bytes, because that is the
//! domain every locator span is stated against. A CRLF→LF conversion therefore reads as a changed
//! line, which is exactly what `30R` rules it: a source change, not an invisible equivalence.

use super::states::CurrentSourceState;
use super::value::{ByteAgreement, RecordedValue};
use crate::rows::RecordedSite;

/// The address a question asked about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestedAddress {
    source: u32,
    line: u32,
}

impl RequestedAddress {
    /// Name one line of one acquired source.
    #[must_use]
    pub const fn of(source: u32, line: u32) -> Self {
        Self { source, line }
    }

    /// Which acquired source, by its ordinal in the recorded table.
    #[must_use]
    pub const fn source(self) -> u32 {
        self.source
    }

    /// Which physical line, 1-indexed.
    #[must_use]
    pub const fn line(self) -> u32 {
        self.line
    }
}

/// Why an address could not be resolved at all.
///
/// Distinct from an address that was resolvable and DISAGREED: these are absences in the document
/// or the question, and none of them is evidence about the current tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnresolvedReason {
    /// The document does not carry the source's exact bytes, so there is no recorded line to
    /// compare against.
    SourceContentUnavailable,
    /// No recorded site carries a locator naming that source, so nothing places a decision there.
    LocatorUnavailable,
    /// The recorded source has no such physical line.
    NoRecordedLine,
    /// The line exists and no recorded site's authored span falls within it.
    NoSiteAtLine,
}

/// What became of the requested address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressResolution {
    /// Current and recorded line N are byte-identical, and this site is the one recorded there.
    Resolved {
        /// The site recorded there.
        site: RecordedSite,
    },
    /// Both lines exist and differ. The address-specific answer is refused pending an explicit
    /// current-versus-recorded selector; both source states remain available, and no moved-line
    /// equivalence is inferred.
    ChangedLine {
        /// The site the RECORDED line carries, which is still a true statement about the past.
        recorded_site: Option<RecordedSite>,
    },
    /// The current source could not be compared. A recorded-only answer may still stand, qualified
    /// by the comparison that did not happen.
    ComparisonUnavailable {
        /// The site the recorded line carries.
        recorded_site: Option<RecordedSite>,
        /// Why no comparison was made.
        why: CurrentSourceState,
    },
    /// Nothing placed the address at all.
    Unresolved(UnresolvedReason),
}

/// The requested address and what became of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressFacts {
    requested: RequestedAddress,
    resolution: AddressResolution,
    current: CurrentSourceState,
}

impl AddressFacts {
    pub(crate) const fn of(
        requested: RequestedAddress,
        resolution: AddressResolution,
        current: CurrentSourceState,
    ) -> Self {
        Self {
            requested,
            resolution,
            current,
        }
    }

    /// The address as asked.
    #[must_use]
    pub const fn requested(&self) -> RequestedAddress {
        self.requested
    }

    /// What became of it.
    #[must_use]
    pub const fn resolution(&self) -> &AddressResolution {
        &self.resolution
    }

    /// How the current source stood, whatever the resolution did with that.
    #[must_use]
    pub const fn current(&self) -> CurrentSourceState {
        self.current
    }

    /// The site the address ADMITS, which is only the exact-line case.
    ///
    /// A changed line and an unavailable comparison both carry a recorded site, and neither of
    /// them admits it as the answer to the address — that is the whole distinction, so it lives in
    /// the method rather than in a caller's `match`.
    #[must_use]
    pub const fn resolved_site(&self) -> Option<RecordedSite> {
        match self.resolution {
            AddressResolution::Resolved { site } => Some(site),
            AddressResolution::ChangedLine { .. }
            | AddressResolution::ComparisonUnavailable { .. }
            | AddressResolution::Unresolved(_) => None,
        }
    }
}

/// One recorded site's authored placement, as the resolver needs it.
pub(crate) struct AuthoredPlacement {
    pub(crate) site: RecordedSite,
    pub(crate) source: u32,
    pub(crate) span: (u64, u64),
}

/// Resolve `requested` against the recorded source and the sites placed in it.
///
/// Pure: every filesystem read already happened at the edge, and `current` is what it SAW.
pub(crate) fn resolve(
    requested: RequestedAddress,
    recorded: Option<&RecordedValue>,
    current: Option<&[u8]>,
    standing: CurrentSourceState,
    placements: &[AuthoredPlacement],
) -> AddressFacts {
    let Some(recorded) = recorded else {
        return AddressFacts::of(
            requested,
            AddressResolution::Unresolved(UnresolvedReason::SourceContentUnavailable),
            standing,
        );
    };
    let Some(span) = recorded.line_span(requested.line) else {
        return AddressFacts::of(
            requested,
            AddressResolution::Unresolved(UnresolvedReason::NoRecordedLine),
            standing,
        );
    };
    if !placements
        .iter()
        .any(|placement| placement.source == requested.source)
    {
        return AddressFacts::of(
            requested,
            AddressResolution::Unresolved(UnresolvedReason::LocatorUnavailable),
            standing,
        );
    }

    // A site belongs to the line when its authored span STARTS inside it. Start rather than
    // overlap: a construct spanning several lines is addressed at the line it begins on, which is
    // the line a reader typed, and an overlap test would answer the same site for every line it
    // covers.
    let recorded_site = placements
        .iter()
        .find(|placement| {
            placement.source == requested.source
                && placement.span.0 >= span.0
                && placement.span.0 < span.1
        })
        .map(|placement| placement.site);

    let Some(recorded_site) = recorded_site else {
        return AddressFacts::of(
            requested,
            AddressResolution::Unresolved(UnresolvedReason::NoSiteAtLine),
            standing,
        );
    };

    let Some(current) = current.filter(|_| standing.was_compared()) else {
        return AddressFacts::of(
            requested,
            AddressResolution::ComparisonUnavailable {
                recorded_site: Some(recorded_site),
                why: standing,
            },
            standing,
        );
    };

    // THE COMPARISON, and it is line N against line N of the same source. Nothing searches.
    let current_line = physical_line(current, requested.line);
    let recorded_line = recorded.physical_line(requested.line);
    let agreement = match (recorded_line, current_line) {
        (Some(recorded_line), Some(current_line)) => recorded_line.agrees_with(current_line),
        // The current file is shorter than the recorded one at this line: the line is missing,
        // which is a difference and never an absent comparison.
        _ => ByteAgreement::Differing,
    };

    let resolution = match agreement {
        ByteAgreement::Identical => AddressResolution::Resolved {
            site: recorded_site,
        },
        ByteAgreement::Differing => AddressResolution::ChangedLine {
            recorded_site: Some(recorded_site),
        },
    };
    AddressFacts::of(requested, resolution, standing)
}

/// Physical line `line` (1-indexed) of `bytes`, terminator included.
///
/// The same indexing [`RecordedValue::physical_line`] does, over bytes that arrived from the edge
/// rather than from a document — one rule, two sources, because a comparison whose two halves
/// counted lines differently would disagree about which lines it was even comparing.
fn physical_line(bytes: &[u8], line: u32) -> Option<&[u8]> {
    let wanted = usize::try_from(line).ok()?.checked_sub(1)?;
    bytes.split_inclusive(|byte| *byte == b'\n').nth(wanted)
}
