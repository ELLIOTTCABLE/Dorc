//! THE source-comparison seat: the one implementation of `SourceComparisonConsumer`, and the only
//! place recorded source material meets a filesystem.
//!
//! # Why exactly one
//!
//! `dorc_receipt` validates and classifies recorded sources and releases them through one visit; it
//! compares nothing, resolves no path, and reads no file. Everything on this side of that line is
//! POLICY — how a recorded path rehydrates on this platform, what a bounded read is willing to
//! open, what "the same source" means, what a line number is counted against — and policy scattered
//! across seats is policy nobody can change. Future correspondence features (diffs, content-aware
//! matching, moved-file rules) extend THIS seat; none of them becomes a receipt API and none of them
//! becomes a second path exit.
//!
//! # The authentication asymmetry
//!
//! A receipt-provided path is a place this process is about to open, so whose word it rests on
//! matters. For material this controller AUTHENTICATED, the seat rehydrates the recorded path and
//! reads it without being asked. For imported or self-asserted material it reads nothing a receipt
//! named: the comparison is available only against a file the USER names, which is a path the user
//! typed rather than one a document supplied.
//!
//! # Two encoders, two destinations
//!
//! `sinv-sink-encoding` binds a value to the encoder for its DESTINATION. A path about to be opened
//! and a path about to be shown are different destinations, so this seat has its own
//! [`FilesystemBytes`] encoder — exact, refusing, never a display seat — while everything rendered
//! still leaves through the terminal or JSON encoders. Neither may stand in for the other.
//!
//! # Bounded, non-following, regular files only
//!
//! A read here opens the operator's own book, and it is still bounded (`source_content_bytes`),
//! refuses to follow a symlink (`symlink_metadata`, then regular-file only), and refuses rather
//! than truncates. Disclosed limit (`churn-avoidance-disclosure`): the stat and the open are two
//! syscalls, so a path replaced between them is read as whatever replaced it. Closing that needs an
//! open-then-fstat handle this spike's edge does not have, and the consequence is bounded — a
//! wrong-but-bounded comparison answer, never an action.

use std::path::PathBuf;

use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::report::{
    AuthenticationState, CurrentSourceReading, RecordedSourceMaterial, RecordedValue,
    RequestedAddress, SourceComparison, SourceComparisonConsumer, ValueClass, ValueEncoder,
};
use dorc_why::{AddressStanding, ComparedSources, NamedSource, SourceRef, UnplaceableAddress};

use crate::recorded_facts::ObservedSource;

/// The file the question named, as the edge read it.
#[derive(Debug, Clone)]
pub struct NamedFile {
    /// The path exactly as the user spelled it.
    pub path: String,
    /// Which physical line, 1-indexed.
    pub line: u32,
    /// The file's exact current bytes.
    pub bytes: Vec<u8>,
}

/// What one walk of a document's recorded sources established.
#[derive(Debug, Default)]
pub struct ComparedOutcome {
    /// The per-source current-tree observations, for the report model.
    pub observations: Vec<ObservedSource>,
    /// The address the question named, placed.
    pub address: Option<RequestedAddress>,
    /// Everything the reconstruction needs to spell `file.sh:N`, and what became of the address.
    pub compared: ComparedSources,
}

/// Walk one document's recorded sources and answer everything the surface needs about them.
///
/// The ONE call site of `visit_for_comparison`, as [`SourceComparisonSeat`] is its one
/// implementation. Both are enumerated by the roster in `receipt/tests/crate_boundary.rs`.
#[must_use]
pub fn compare_sources(
    material: &RecordedSourceMaterial<'_>,
    named: Option<&NamedFile>,
) -> ComparedOutcome {
    let mut seat = SourceComparisonSeat {
        named,
        observations: Vec::new(),
        naming: Vec::new(),
        placed: None,
    };
    material.visit_for_comparison(&mut seat);
    let address = seat
        .placed
        .and_then(|ordinal| named.map(|named| RequestedAddress::of(ordinal, named.line)));
    // An address the walk could not place is a fact about the QUESTION, and it is the only thing
    // this seat can say about one: no nearest-match, no moved-file search, because either would
    // answer confidently about a file the author moved.
    let standing = match (named, address) {
        (Some(_), None) => {
            AddressStanding::Unplaceable(UnplaceableAddress::NoRecordedSourceMatches)
        }
        _ => AddressStanding::AsRecorded,
    };
    ComparedOutcome {
        observations: seat.observations,
        address,
        compared: ComparedSources::of(standing, seat.naming),
    }
}

/// The one implementation of `dorc_receipt::report::SourceComparisonConsumer`.
struct SourceComparisonSeat<'a> {
    named: Option<&'a NamedFile>,
    observations: Vec<ObservedSource>,
    naming: Vec<NamedSource>,
    placed: Option<u32>,
}

impl SourceComparisonConsumer for SourceComparisonSeat<'_> {
    fn accept(&mut self, source: &SourceComparison<'_>) {
        let ordinal = source.ordinal();
        let recorded_path = source.path().and_then(exact_path);

        // The NAMING, for `file.sh:N`. It rides the recorded path and the recorded line map, so a
        // source whose content the document does not carry contributes no line map and its
        // addresses keep the honest ordinal-and-span fallback.
        if let Some(file) = source.path().cloned() {
            self.naming.push(NamedSource {
                source: SourceRef::of(ordinal),
                file,
                line_starts: source.line_starts(),
            });
        }

        // WHICH source the question named. Exact recorded PATH first — that is
        // `30R:receipt-rooted-attention-and-cli`'s own rule, the same physical path in the current
        // and recorded book — and exact recorded CONTENT second, which identifies the same file
        // under a different spelling without guessing that anything moved.
        let named = self.named.filter(|named| {
            recorded_path.as_deref() == Some(named.path.as_str())
                || source.digest() == dorc_plan::invocation::book_digest(&lossy(&named.bytes))
        });
        if named.is_some() {
            self.placed = self.placed.or(Some(ordinal));
        }

        let reading = match named {
            Some(named) => CurrentSourceReading::Read(named.bytes.clone()),
            // The asymmetry: a receipt-provided path is followed only for material this controller
            // authenticated. Anything else is a path somebody else's document supplied, and this
            // process does not open one of those unprompted.
            None if source.authentication() == AuthenticationState::Trusted => recorded_path
                .as_deref()
                .map_or(CurrentSourceReading::NotLookedFor, read_regular_file),
            None => CurrentSourceReading::NotLookedFor,
        };
        let matches_digest = current_bytes(&reading).is_some_and(|bytes| {
            source.digest() == dorc_plan::invocation::book_digest(&lossy(bytes))
        });
        self.observations.push(ObservedSource {
            ordinal,
            reading,
            matches_digest,
        });
    }
}

/// The bytes a reading holds, where it holds any.
fn current_bytes(reading: &CurrentSourceReading) -> Option<&[u8]> {
    match reading {
        CurrentSourceReading::Read(bytes) => Some(bytes),
        CurrentSourceReading::Absent
        | CurrentSourceReading::Unreadable
        | CurrentSourceReading::NotLookedFor => None,
    }
}

/// One acquired source's bytes as the digest seat spells them.
///
/// Lossy because the digest is computed over `&str` at the seat that MINTED it
/// (`dorc_plan::invocation::book_digest`, over the acquired source text), so recomputing it any
/// other way would answer a different question about the same bytes.
fn lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// A recorded path as this platform spells one, or `None` where it is not a path this seat may act
/// on.
///
/// UTF-8 or nothing. A lossy conversion would make two different paths compare equal and would hand
/// `open` a name nobody recorded, and both are worse than declining to look.
fn exact_path(value: &RecordedValue) -> Option<String> {
    let spelled = value.render(&mut FilesystemBytes);
    (!spelled.is_empty()).then_some(spelled)
}

/// Read one regular file, bounded and without following a link.
fn read_regular_file(path: &str) -> CurrentSourceReading {
    use std::io::Read as _;
    let path = PathBuf::from(path);
    match std::fs::symlink_metadata(&path) {
        Err(_) => return CurrentSourceReading::Absent,
        // A symlink, a directory, or a device is not a source this seat reads. `symlink_metadata`
        // does not follow, so `is_file` here means the NAME itself is a regular file.
        Ok(meta) if !meta.is_file() => return CurrentSourceReading::Unreadable,
        Ok(_) => {}
    }
    let cap = ReceiptLimits::V1.source_content_bytes.get();
    let Ok(file) = std::fs::File::open(&path) else {
        return CurrentSourceReading::Unreadable;
    };
    let mut bytes = Vec::new();
    if file
        .take(cap.saturating_add(1))
        .read_to_end(&mut bytes)
        .is_err()
    {
        return CurrentSourceReading::Unreadable;
    }
    // Refused rather than truncated: the comparison is byte-exact, and a prefix compared against a
    // whole would read as drift.
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > cap {
        return CurrentSourceReading::Unreadable;
    }
    CurrentSourceReading::Read(bytes)
}

/// The FILESYSTEM destination's encoder: exact bytes back, for the one seat that acts on them.
///
/// Its own destination rather than a display one (`sinv-sink-encoding` binds a value to the encoder
/// for its destination): a path a process is about to open is not a path a person is about to read,
/// and the encoder answering one must never answer the other. It refuses every class but a path,
/// and refuses non-UTF-8, so a value that reached it by accident comes back EMPTY rather than raw.
#[derive(Debug, Default)]
struct FilesystemBytes;

impl ValueEncoder for FilesystemBytes {
    fn encode(&mut self, class: ValueClass, bytes: &[u8]) -> String {
        match class {
            ValueClass::SourcePath => String::from_utf8(bytes.to_vec()).unwrap_or_default(),
            ValueClass::ShellText
            | ValueClass::SourceText
            | ValueClass::ArtifactLabel
            | ValueClass::OriginClaim
            | ValueClass::Argv
            | ValueClass::TargetName
            | ValueClass::HostOutput
            | ValueClass::Coordinate
            | ValueClass::EncodedStructure
            | ValueClass::DiagnosticDetail => String::new(),
        }
    }
}
