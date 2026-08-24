//! The reverse overlay: the decrypted region names structural slots, and only in that
//! direction.
//!
//! Nothing in the readable skeleton points into the region. The region carries the outer
//! identity, the species, the projection and a digest of the literal skeleton span, and its
//! entries key already-signed records. Validation is total before any value is readable: a
//! single departure yields a fault and releases nothing, never a partial enrichment.

use std::collections::BTreeSet;

use crate::format::Skeleton;
use crate::grammar;
use crate::ids::span_digest_hex;
use crate::limits::ReceiptLimits;
use crate::projection::{CAPTURED, OpaqueFieldTag, opaque_slots};

/// The region's own version line.
pub const OVERLAY_VERSION_LINE: &str = "dorc-receipt-overlay/1";

/// The token closing the region's entry sequence.
pub const OVERLAY_END: &str = "overlay-end";

/// Why a region did not validate. Closed: each fault names one condition, and none stands in
/// for another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayFault {
    /// A bound was exceeded before anything was read or reserved.
    OverBound {
        /// Which bound.
        what: &'static str,
    },
    /// A required header line was absent, out of order, or misspelled.
    Header {
        /// Which line.
        what: &'static str,
    },
    /// The region names a different document, species, or projection than the one it arrived
    /// in.
    DocumentMismatch,
    /// The region's digest of the skeleton is not this skeleton's digest.
    SkeletonDigestMismatch,
    /// An entry line was misshaped, or named a tag outside the closed set.
    EntryShape {
        /// Which part.
        what: &'static str,
    },
    /// The declared entry count and the entries present disagree.
    EntryCount,
    /// Entries are not in canonical order.
    Ordering,
    /// Two entries name one slot.
    DuplicateKey,
    /// An entry names a record the skeleton does not contain.
    DanglingRecord,
    /// An entry names a field its record's kind cannot carry.
    WrongFieldForKind,
    /// An entry names a slot whose skeleton state word did not say it was captured.
    Unaccounted,
    /// A slot the skeleton says was captured has no entry.
    MissingRequired,
    /// Bytes after the region terminator.
    Trailing,
}

/// One region entry: which record, which field, which exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayEntry {
    record: u64,
    tag: OpaqueFieldTag,
    bytes: Vec<u8>,
}

impl OverlayEntry {
    /// Declare one entry.
    #[must_use]
    pub const fn of(record: u64, tag: OpaqueFieldTag, bytes: Vec<u8>) -> Self {
        Self { record, tag, bytes }
    }

    /// Which record this enriches.
    #[must_use]
    pub const fn record(&self) -> u64 {
        self.record
    }

    /// Which field this fills.
    #[must_use]
    pub const fn tag(&self) -> OpaqueFieldTag {
        self.tag
    }

    /// The exact bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The canonical sort key: record first, then the tag table's order.
    fn key(&self) -> (u64, usize) {
        (self.record, self.tag.order())
    }
}

/// The slots a skeleton says are captured, in canonical order.
///
/// Computed from the skeleton alone. This is the account the region is matched against in
/// both directions, which is what makes a missing entry and an extra one equally refusable.
#[must_use]
pub fn captured_slots(skeleton: &Skeleton) -> Vec<(u64, OpaqueFieldTag)> {
    let mut out = Vec::new();
    for (index, record) in skeleton.records.iter().enumerate() {
        let Ok(id) = u64::try_from(index) else {
            continue;
        };
        for slot in opaque_slots(record.kind()) {
            if record.atom(slot.key) == Some(CAPTURED) {
                out.push((id, slot.tag));
            }
        }
    }
    out.sort_by_key(|(record, tag)| (*record, tag.order()));
    out
}

/// Serialize entries into the region's one exact form.
///
/// Entries are emitted in canonical order regardless of the order supplied, so a caller
/// cannot produce a region that differs from the one the reader expects.
#[must_use]
pub fn serialize(
    receipt_id: &str,
    species: &str,
    skeleton_span: &[u8],
    entries: &[OverlayEntry],
) -> Vec<u8> {
    let mut ordered: Vec<&OverlayEntry> = entries.iter().collect();
    ordered.sort_by_key(|entry| entry.key());

    let mut out = Vec::new();
    out.extend_from_slice(OVERLAY_VERSION_LINE.as_bytes());
    out.push(b'\n');
    out.extend_from_slice(b"receipt-id ");
    out.extend_from_slice(receipt_id.as_bytes());
    out.push(b'\n');
    out.extend_from_slice(b"species ");
    out.extend_from_slice(species.as_bytes());
    out.push(b'\n');
    out.extend_from_slice(b"projection rich\n");
    out.extend_from_slice(b"skeleton-sha256 ");
    out.extend_from_slice(span_digest_hex(skeleton_span).as_bytes());
    out.push(b'\n');
    out.extend_from_slice(b"entries ");
    out.extend_from_slice(ordered.len().to_string().as_bytes());
    out.push(b'\n');
    for entry in ordered {
        out.extend_from_slice(b"entry ");
        out.extend_from_slice(entry.record.to_string().as_bytes());
        out.push(b' ');
        out.extend_from_slice(entry.tag.token().as_bytes());
        out.push(b' ');
        out.extend_from_slice(entry.bytes.len().to_string().as_bytes());
        out.push(b'\n');
        out.extend_from_slice(&entry.bytes);
        out.push(b'\n');
    }
    out.extend_from_slice(OVERLAY_END.as_bytes());
    out.push(b'\n');
    out
}

/// A byte cursor. The region is binary-safe, so nothing here converts it to text.
struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    const fn of(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    /// The next line, without its newline. Refuses a final line with no newline.
    fn line(&mut self) -> Option<&'a [u8]> {
        let rest = self.bytes.get(self.at..)?;
        let end = rest.iter().position(|byte| *byte == b'\n')?;
        let line = rest.get(..end)?;
        self.at = self.at.checked_add(end)?.checked_add(1)?;
        Some(line)
    }

    /// Exactly `count` bytes.
    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(count)?;
        let slice = self.bytes.get(self.at..end)?;
        self.at = end;
        Some(slice)
    }

    /// One byte, matched exactly.
    fn byte(&mut self, want: u8) -> Option<()> {
        let got = *self.bytes.get(self.at)?;
        if got != want {
            return None;
        }
        self.at = self.at.checked_add(1)?;
        Some(())
    }

    const fn done(&self) -> bool {
        self.at == self.bytes.len()
    }
}

fn header_value<'a>(cursor: &mut Cursor<'a>, key: &'static str) -> Result<&'a [u8], OverlayFault> {
    let line = cursor.line().ok_or(OverlayFault::Header { what: key })?;
    let prefix = key
        .len()
        .checked_add(1)
        .ok_or(OverlayFault::Header { what: key })?;
    let (head, rest) = line
        .split_at_checked(prefix)
        .ok_or(OverlayFault::Header { what: key })?;
    if head.get(..key.len()) != Some(key.as_bytes()) || head.last() != Some(&b' ') {
        return Err(OverlayFault::Header { what: key });
    }
    if rest.is_empty() {
        return Err(OverlayFault::Header { what: key });
    }
    Ok(rest)
}

fn ascii<'a>(bytes: &'a [u8], what: &'static str) -> Result<&'a str, OverlayFault> {
    let text = core::str::from_utf8(bytes).map_err(|_| OverlayFault::Header { what })?;
    if text.is_ascii() {
        Ok(text)
    } else {
        Err(OverlayFault::Header { what })
    }
}

/// Parse the region's exact form into entries, checking shape and bounds only.
///
/// Answers entries in the order the region spelled them, so the caller can tell a
/// canonically-ordered region from one that merely contains the right entries.
fn parse(bytes: &[u8], limits: &ReceiptLimits) -> Result<ParsedOverlay, OverlayFault> {
    let measured = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if !limits.overlay_bytes.admits(measured) {
        return Err(OverlayFault::OverBound {
            what: "overlay-bytes",
        });
    }
    let mut cursor = Cursor::of(bytes);

    let version = cursor
        .line()
        .ok_or(OverlayFault::Header { what: "version" })?;
    if version != OVERLAY_VERSION_LINE.as_bytes() {
        return Err(OverlayFault::Header { what: "version" });
    }
    let receipt_id = ascii(header_value(&mut cursor, "receipt-id")?, "receipt-id")?.to_owned();
    let species = ascii(header_value(&mut cursor, "species")?, "species")?.to_owned();
    let projection = ascii(header_value(&mut cursor, "projection")?, "projection")?;
    if projection != "rich" {
        return Err(OverlayFault::DocumentMismatch);
    }
    let digest = ascii(
        header_value(&mut cursor, "skeleton-sha256")?,
        "skeleton-sha256",
    )?
    .to_owned();
    if !grammar::is_digest(&receipt_id) || !grammar::is_digest(&digest) {
        return Err(OverlayFault::Header {
            what: "skeleton-sha256",
        });
    }

    let declared = ascii(header_value(&mut cursor, "entries")?, "entries")?;
    let count = grammar::canonical_u64(declared).ok_or(OverlayFault::EntryCount)?;
    if !limits.overlay_entries.admits(count) {
        return Err(OverlayFault::OverBound {
            what: "overlay-entries",
        });
    }

    let mut entries = Vec::new();
    for _ in 0..count {
        entries.push(parse_entry(&mut cursor, limits)?);
    }

    let end = cursor.line().ok_or(OverlayFault::EntryCount)?;
    if end != OVERLAY_END.as_bytes() {
        return Err(OverlayFault::EntryCount);
    }
    if !cursor.done() {
        return Err(OverlayFault::Trailing);
    }

    Ok(ParsedOverlay {
        receipt_id,
        species,
        skeleton_digest: digest,
        entries,
    })
}

fn parse_entry(
    cursor: &mut Cursor<'_>,
    limits: &ReceiptLimits,
) -> Result<OverlayEntry, OverlayFault> {
    let line = cursor.line().ok_or(OverlayFault::EntryCount)?;
    let text = ascii(line, "entry")?;
    // Running into the terminator means the declared count promised more entries than the
    // region carries. That is a count disagreement, not a misshaped entry line.
    if text == OVERLAY_END {
        return Err(OverlayFault::EntryCount);
    }
    let rest = text
        .strip_prefix("entry ")
        .ok_or(OverlayFault::EntryShape { what: "entry" })?;
    let (record, rest) = rest
        .split_once(' ')
        .ok_or(OverlayFault::EntryShape { what: "record" })?;
    let (tag, length) = rest
        .split_once(' ')
        .ok_or(OverlayFault::EntryShape { what: "tag" })?;

    let record =
        grammar::canonical_u64(record).ok_or(OverlayFault::EntryShape { what: "record" })?;
    let tag = OpaqueFieldTag::of_token(tag).ok_or(OverlayFault::EntryShape { what: "tag" })?;
    let length =
        grammar::canonical_u64(length).ok_or(OverlayFault::EntryShape { what: "length" })?;
    if !limits.opaque_field_bytes.admits(length) {
        return Err(OverlayFault::OverBound {
            what: "opaque-field-bytes",
        });
    }
    // The declared length is consumed before the framing newline is sought, so a payload
    // containing newlines frames correctly and a short region is refused rather than
    // resynchronised onto whatever byte happens to follow.
    let wanted = usize::try_from(length).map_err(|_| OverlayFault::OverBound {
        what: "opaque-field-bytes",
    })?;
    let payload = cursor
        .take(wanted)
        .ok_or(OverlayFault::EntryShape { what: "payload" })?
        .to_vec();
    cursor
        .byte(b'\n')
        .ok_or(OverlayFault::EntryShape { what: "framing" })?;
    Ok(OverlayEntry::of(record, tag, payload))
}

struct ParsedOverlay {
    receipt_id: String,
    species: String,
    skeleton_digest: String,
    entries: Vec<OverlayEntry>,
}

/// Plaintext that has been opened but not validated.
///
/// Inert by construction: no accessor, not `Clone`, and the only thing that consumes it is
/// [`DecryptedOpaqueOverlay::validate`]. A value here has been authenticated as ciphertext
/// and has not yet been shown to belong to the skeleton in hand.
#[derive(Debug)]
pub struct DecryptedOpaqueOverlay {
    bytes: Vec<u8>,
}

impl DecryptedOpaqueOverlay {
    /// Take opened plaintext.
    #[must_use]
    pub const fn of(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Validate against the skeleton this region arrived with, consuming the plaintext.
    ///
    /// Total in both directions: every entry is accounted for by the skeleton, and every slot
    /// the skeleton says was captured has exactly one entry. One departure releases nothing.
    ///
    /// # Errors
    /// Answers the one condition that stopped validation.
    pub fn validate(
        self,
        skeleton: &Skeleton,
        skeleton_span: &[u8],
        species: &str,
        limits: &ReceiptLimits,
    ) -> Result<ValidatedOpaqueOverlay, OverlayFault> {
        let parsed = parse(&self.bytes, limits)?;

        if parsed.receipt_id != skeleton.receipt_id || parsed.species != species {
            return Err(OverlayFault::DocumentMismatch);
        }
        if parsed.skeleton_digest != span_digest_hex(skeleton_span) {
            return Err(OverlayFault::SkeletonDigestMismatch);
        }

        let mut seen: BTreeSet<(u64, usize)> = BTreeSet::new();
        let mut previous: Option<(u64, usize)> = None;
        for entry in &parsed.entries {
            let key = entry.key();
            if let Some(last) = previous {
                if key <= last {
                    return Err(if key == last {
                        OverlayFault::DuplicateKey
                    } else {
                        OverlayFault::Ordering
                    });
                }
            }
            if !seen.insert(key) {
                return Err(OverlayFault::DuplicateKey);
            }
            previous = Some(key);

            let index = usize::try_from(entry.record).map_err(|_| OverlayFault::DanglingRecord)?;
            let record = skeleton
                .records
                .get(index)
                .ok_or(OverlayFault::DanglingRecord)?;
            let slot = opaque_slots(record.kind())
                .iter()
                .find(|slot| slot.tag == entry.tag)
                .ok_or(OverlayFault::WrongFieldForKind)?;
            if record.atom(slot.key) != Some(CAPTURED) {
                return Err(OverlayFault::Unaccounted);
            }
        }

        let expected = captured_slots(skeleton);
        if expected.len() != parsed.entries.len() {
            return Err(OverlayFault::MissingRequired);
        }
        for ((want_record, want_tag), entry) in expected.iter().zip(parsed.entries.iter()) {
            if *want_record != entry.record || *want_tag != entry.tag {
                return Err(OverlayFault::MissingRequired);
            }
        }

        Ok(ValidatedOpaqueOverlay {
            entries: parsed.entries,
        })
    }
}

/// A region that validated completely against its skeleton.
///
/// The only value a detail byte can be read from. Reaching one means the region named this
/// document, this species, this skeleton, and exactly the slots the skeleton accounts for.
#[derive(Debug, PartialEq, Eq)]
pub struct ValidatedOpaqueOverlay {
    entries: Vec<OverlayEntry>,
}

impl ValidatedOpaqueOverlay {
    /// The bytes filling one slot, if the skeleton accounted for it.
    #[must_use]
    pub fn value(&self, record: u64, tag: OpaqueFieldTag) -> Option<&[u8]> {
        self.entries
            .iter()
            .find(|entry| entry.record == record && entry.tag == tag)
            .map(OverlayEntry::bytes)
    }

    /// Every entry, in canonical order.
    #[must_use]
    pub fn entries(&self) -> &[OverlayEntry] {
        &self.entries
    }
}
