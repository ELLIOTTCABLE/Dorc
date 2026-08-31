//! What the ONE source-comparison seat established, as the reconstruction consumes it.
//!
//! Everything here ARRIVES from the cli seat that owns `dorc_receipt::report`'s comparison packet:
//! path rehydration, bounded reads, correspondence policy and the same-physical-line rule are all
//! its, and this crate only carries the answers. Nothing in this file reads a filesystem, resolves
//! a path, or decides what two sources being "the same" means.
//!
//! # Why the naming is a table rather than a field on the address
//!
//! A `LocusAddress` is `Eq` and cheap, and a recorded PATH is a sealed value with no equality at
//! all (`report::value`: a derived one composes into orderings and hashes, and those leak). Keeping
//! the naming beside the population rather than inside every address is what lets both stay what
//! they are.

use dorc_receipt::report::RecordedValue;

use crate::datum::SourceRef;

/// Where an acquired source lives, and how its bytes divide into physical lines.
///
/// `line_starts` is OFFSETS, never content: it is what turns a recorded locator span into a line
/// NUMBER without any byte leaving except through an encoder.
#[derive(Debug, Clone)]
pub struct NamedSource {
    /// Which acquired source, by ordinal.
    pub source: SourceRef,
    /// Its exact recorded path. Encoder-mediated like every recorded value.
    pub file: RecordedValue,
    /// The byte offset each physical line of the recorded content starts at, first line first.
    pub line_starts: Vec<u64>,
}

/// What the EDGE could do with the address the question named.
///
/// A plan root's placed address travels inside `RecordedWhyFacts`, so the only thing this adds is
/// the case the report model cannot represent: a request the seat could not turn into an ordinal at
/// all. That is a fact about the QUESTION rather than about any document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddressStanding {
    /// No address was named, or the root's own facts already answer the one that was.
    #[default]
    AsRecorded,
    /// The question named an address this route could not place at all.
    Unplaceable(crate::datum::UnplaceableAddress),
}

/// The comparison seat's whole answer, as one value.
#[derive(Debug, Clone, Default)]
pub struct ComparedSources {
    address: AddressStanding,
    named: Vec<NamedSource>,
}

impl ComparedSources {
    /// Bind what the seat established.
    #[must_use]
    pub const fn of(address: AddressStanding, named: Vec<NamedSource>) -> Self {
        Self { address, named }
    }

    /// What became of the address the question named.
    #[must_use]
    pub const fn address(&self) -> AddressStanding {
        self.address
    }

    /// The namings alone, for a caller re-binding them under a different address standing.
    #[must_use]
    pub fn into_named(self) -> Vec<NamedSource> {
        self.named
    }

    /// Where one acquired source lives, where the seat could say.
    #[must_use]
    pub fn name_of(&self, source: SourceRef) -> Option<&NamedSource> {
        self.named.iter().find(|named| named.source == source)
    }

    /// Which physical line (1-indexed) an acquired source's byte `offset` falls on.
    ///
    /// The last line start at or below the offset, which is what a line NUMBER is. `None` where the
    /// seat supplied no line map — a source whose content the document does not carry has no lines
    /// to count, and answering 1 would be a number about nothing.
    #[must_use]
    pub fn line_of(&self, source: SourceRef, offset: u64) -> Option<u32> {
        let starts = &self.name_of(source)?.line_starts;
        let index = starts.partition_point(|start| *start <= offset);
        u32::try_from(index).ok().filter(|line| *line > 0)
    }
}
