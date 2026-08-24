//! The exact `dorc-receipt/1` physical grammar: one writer form, one reader form.
//!
//! Byte equality is the format's equality relation. The reader locates exact spans first and
//! parses the same span it checked; no path here re-encodes, normalizes, or re-reads
//! anything to produce a second copy of a document's bytes.

use crate::grammar::{self, Field, RecordKind};
use crate::limits::ReceiptLimits;
use crate::model::{Projection, Species};

/// The format's version line.
pub const VERSION_LINE: &str = "dorc-receipt/1";

/// The token closing the readable skeleton.
pub const SKELETON_END: &str = "skeleton-end";

/// The token opening the encrypted region.
pub const OVERLAY_BEGIN: &str = "opaque-overlay";

/// The token closing the encrypted region.
pub const OVERLAY_END: &str = "opaque-end";

/// Why a document was not accepted. Closed: every refusal names one condition, and none is
/// interchangeable with another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusalReason {
    /// The input exceeded a bound before parsing began.
    OverBound {
        /// Which bound.
        what: &'static str,
    },
    /// The version line was absent or named a version this reader does not implement.
    UnsupportedVersion,
    /// A required structural line was absent, out of order, or misspelled.
    Structure {
        /// Which line.
        what: &'static str,
    },
    /// A byte the grammar does not admit.
    IllegalByte {
        /// Which byte.
        byte: u8,
    },
    /// The document declared one species or projection and the header selected another.
    DomainMismatch,
    /// A record kind this species does not admit, or no kind at all.
    UnknownRecordKind,
    /// A field key absent, duplicated, out of order, or unknown for its kind.
    FieldShape {
        /// Which kind.
        kind: &'static str,
    },
    /// A field atom outside its declared token set or numeric width.
    FieldAtom {
        /// Which key.
        key: &'static str,
    },
    /// The declared record count and the records present disagree.
    RecordCount,
    /// A record identity that was not contiguous from zero.
    RecordIdentity,
    /// A plain document carrying an encrypted region, or a rich one without exactly one.
    OverlayPresence,
    /// The signature line was absent, misshaped, or not followed immediately by end of input.
    SignatureShape,
    /// The signature did not check against the material the resolver supplied.
    SignatureCheck,
    /// No verification material was available for the named provider.
    KeyUnavailable,
}

/// One skeleton record: its kind and one atom per field, in table order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkeletonRecord {
    kind: RecordKind,
    atoms: Vec<String>,
}

impl SkeletonRecord {
    /// Assemble a record, checking every atom against the table.
    ///
    /// The writer's own acceptance check: a record that could not be read back is refused
    /// here rather than emitted.
    ///
    /// # Errors
    /// Refuses an atom count that does not match the table, or an atom the table refuses.
    pub fn build(kind: RecordKind, atoms: Vec<String>) -> Result<Self, RefusalReason> {
        let fields = kind.fields();
        if atoms.len() != fields.len() {
            return Err(RefusalReason::FieldShape { kind: kind.token() });
        }
        for (field, atom) in fields.iter().zip(atoms.iter()) {
            if !field.ty.admits(atom) {
                return Err(RefusalReason::FieldAtom { key: field.key });
            }
        }
        Ok(Self { kind, atoms })
    }

    /// This record's kind.
    #[must_use]
    pub const fn kind(&self) -> RecordKind {
        self.kind
    }

    /// The atom under `key`, if this kind declares that key.
    #[must_use]
    pub fn atom(&self, key: &str) -> Option<&str> {
        let index = self.kind.fields().iter().position(|f| f.key == key)?;
        self.atoms.get(index).map(String::as_str)
    }

    /// Every atom, in table order.
    #[must_use]
    pub fn atoms(&self) -> &[String] {
        &self.atoms
    }
}

/// A parsed skeleton: the header identities and the records, in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skeleton {
    /// The document's own identity, as spelled.
    pub receipt_id: String,
    /// The signing provider's identity, as spelled.
    pub signing_key_id: String,
    /// The encryption provider's identity, present exactly when the projection is rich.
    pub encryption_key_id: Option<String>,
    /// The records, in document order.
    pub records: Vec<SkeletonRecord>,
}

/// Serialize a skeleton to its one exact form, closing with the terminator.
///
/// The result is the literal skeleton span: byte zero through the terminator's newline.
///
/// # Errors
/// Refuses a projection whose region presence disagrees, or a kind the species excludes.
pub fn serialize_skeleton<D: Species, P: Projection>(
    skeleton: &Skeleton,
) -> Result<String, RefusalReason> {
    if P::HAS_OVERLAY != skeleton.encryption_key_id.is_some() {
        return Err(RefusalReason::OverlayPresence);
    }
    for record in &skeleton.records {
        if !D::KINDS.contains(&record.kind) {
            return Err(RefusalReason::UnknownRecordKind);
        }
    }
    let mut out = String::new();
    out.push_str(VERSION_LINE);
    out.push('\n');
    out.push_str("species ");
    out.push_str(D::TOKEN);
    out.push('\n');
    out.push_str("projection ");
    out.push_str(P::TOKEN);
    out.push('\n');
    out.push_str("receipt-id ");
    out.push_str(&skeleton.receipt_id);
    out.push('\n');
    out.push_str("signing-key-id ");
    out.push_str(&skeleton.signing_key_id);
    out.push('\n');
    if let Some(id) = &skeleton.encryption_key_id {
        out.push_str("encryption-key-id ");
        out.push_str(id);
        out.push('\n');
    }
    out.push_str("records ");
    out.push_str(&skeleton.records.len().to_string());
    out.push('\n');
    for (index, record) in skeleton.records.iter().enumerate() {
        out.push_str("record ");
        out.push_str(&index.to_string());
        out.push(' ');
        out.push_str(record.kind.token());
        for (field, atom) in record.kind.fields().iter().zip(record.atoms.iter()) {
            out.push(' ');
            out.push_str(field.key);
            out.push('=');
            out.push_str(atom);
        }
        out.push('\n');
    }
    out.push_str(SKELETON_END);
    out.push('\n');
    Ok(out)
}

/// The exact spans a document is made of, located without interpreting any field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedReceiptEnvelope {
    /// The bytes the signature is computed over: byte zero through the last terminator.
    pub body: Vec<u8>,
    /// The literal skeleton span, byte zero through the skeleton terminator.
    pub skeleton: Vec<u8>,
    /// The armored region's exact bytes, when the document carries one.
    pub armor: Option<String>,
    /// The signature, as spelled.
    pub signature_hex: String,
    /// The provider identity the locator read, for lookup only.
    pub signing_key_id: String,
}

/// Locate the exact spans of a document, reading only fixed prefixes and the trailer.
///
/// Interprets no field, resolves nothing, allocates nothing from a declared count, and
/// reaches no capability but the caller's own bounds.
///
/// # Errors
/// Refuses an over-bound input, an illegal byte, an unknown version, or a missing span.
pub fn locate(
    bytes: &[u8],
    limits: &ReceiptLimits,
) -> Result<LocatedReceiptEnvelope, RefusalReason> {
    let total = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if !limits.outer_bytes.admits(total) {
        return Err(RefusalReason::OverBound {
            what: "outer-bytes",
        });
    }
    let text = core::str::from_utf8(bytes).map_err(|_| RefusalReason::IllegalByte { byte: 0 })?;
    if let Some(byte) = text.bytes().find(|b| *b == b'\r' || *b == b'\t') {
        return Err(RefusalReason::IllegalByte { byte });
    }
    if !text.starts_with(VERSION_LINE) || !text[VERSION_LINE.len()..].starts_with('\n') {
        return Err(RefusalReason::UnsupportedVersion);
    }

    let skeleton_marker = format!("\n{SKELETON_END}\n");
    let skeleton_at = text
        .find(&skeleton_marker)
        .ok_or(RefusalReason::Structure { what: SKELETON_END })?;
    let skeleton_end = skeleton_at
        .checked_add(skeleton_marker.len())
        .ok_or(RefusalReason::Structure { what: SKELETON_END })?;
    let skeleton = text
        .get(..skeleton_end)
        .ok_or(RefusalReason::Structure { what: SKELETON_END })?;
    if !limits
        .skeleton_bytes
        .admits(u64::try_from(skeleton.len()).unwrap_or(u64::MAX))
    {
        return Err(RefusalReason::OverBound {
            what: "skeleton-bytes",
        });
    }

    let rest = text
        .get(skeleton_end..)
        .ok_or(RefusalReason::Structure { what: SKELETON_END })?;
    let (armor, body_end) = if let Some(after) = rest.strip_prefix(&format!("{OVERLAY_BEGIN}\n")) {
        let close = format!("\n{OVERLAY_END}\n");
        let at = after
            .find(&close)
            .ok_or(RefusalReason::Structure { what: OVERLAY_END })?;
        let region = after
            .get(..at)
            .ok_or(RefusalReason::Structure { what: OVERLAY_END })?;
        if !limits
            .armor_bytes
            .admits(u64::try_from(region.len()).unwrap_or(u64::MAX))
        {
            return Err(RefusalReason::OverBound {
                what: "armor-bytes",
            });
        }
        let consumed = skeleton_end
            .checked_add(OVERLAY_BEGIN.len())
            .and_then(|n| n.checked_add(1))
            .and_then(|n| n.checked_add(at))
            .and_then(|n| n.checked_add(close.len()))
            .ok_or(RefusalReason::Structure { what: OVERLAY_END })?;
        (Some(region.to_owned()), consumed)
    } else {
        (None, skeleton_end)
    };

    let trailer = text.get(body_end..).ok_or(RefusalReason::SignatureShape)?;
    let signature_hex = trailer
        .strip_prefix("signature ")
        .and_then(|s| s.strip_suffix('\n'))
        .ok_or(RefusalReason::SignatureShape)?;
    if signature_hex.len() != 128
        || !signature_hex
            .bytes()
            .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
    {
        return Err(RefusalReason::SignatureShape);
    }

    let signing_key_id = header_value(skeleton, "signing-key-id")
        .ok_or(RefusalReason::Structure {
            what: "signing-key-id",
        })?
        .to_owned();

    Ok(LocatedReceiptEnvelope {
        body: bytes.get(..body_end).unwrap_or_default().to_vec(),
        skeleton: bytes.get(..skeleton_end).unwrap_or_default().to_vec(),
        armor,
        signature_hex: signature_hex.to_owned(),
        signing_key_id,
    })
}

/// The value of one header line, read positionally from the skeleton's fixed prefix.
fn header_value<'a>(skeleton: &'a str, key: &str) -> Option<&'a str> {
    skeleton
        .lines()
        .take(8)
        .find_map(|line| line.strip_prefix(key)?.strip_prefix(' '))
}

/// Parse the literal skeleton span under the species and projection the header selected.
///
/// Takes the skeleton span, which is a prefix of the span the signature was checked over —
/// for plain the two are the same bytes, and for rich the remainder is the armored region,
/// which goes to the opener. Both are slices of the one checked body, so nothing is
/// re-derived or re-read. The span ends at the terminator's newline, and bytes past it are
/// refused here rather than skipped.
///
/// # Errors
/// Refuses any departure from the exact grammar, including a count or identity mismatch.
pub fn parse_skeleton_span<D: Species, P: Projection>(
    skeleton: &[u8],
    limits: &ReceiptLimits,
) -> Result<Skeleton, RefusalReason> {
    let text =
        core::str::from_utf8(skeleton).map_err(|_| RefusalReason::IllegalByte { byte: 0 })?;
    let mut lines = text.split('\n');

    expect(&mut lines, VERSION_LINE, "version")?;
    expect_kv(&mut lines, "species", D::TOKEN)?;
    expect_kv(&mut lines, "projection", P::TOKEN)?;
    let receipt_id = take_kv(&mut lines, "receipt-id")?;
    if !grammar::is_digest(&receipt_id) {
        return Err(RefusalReason::FieldAtom { key: "receipt-id" });
    }
    let signing_key_id = take_kv(&mut lines, "signing-key-id")?;
    if !grammar::is_digest(&signing_key_id) {
        return Err(RefusalReason::FieldAtom {
            key: "signing-key-id",
        });
    }
    let encryption_key_id = if P::HAS_OVERLAY {
        let id = take_kv(&mut lines, "encryption-key-id")?;
        if !grammar::is_digest(&id) {
            return Err(RefusalReason::FieldAtom {
                key: "encryption-key-id",
            });
        }
        Some(id)
    } else {
        None
    };

    let declared = take_kv(&mut lines, "records")?;
    let count = grammar::canonical_u64(&declared).ok_or(RefusalReason::RecordCount)?;
    if !limits.records.admits(count) {
        return Err(RefusalReason::OverBound { what: "records" });
    }

    let mut records = Vec::new();
    for index in 0..count {
        let line = lines.next().ok_or(RefusalReason::RecordCount)?;
        if !limits
            .line_bytes
            .admits(u64::try_from(line.len()).unwrap_or(u64::MAX))
        {
            return Err(RefusalReason::OverBound { what: "line-bytes" });
        }
        records.push(parse_record::<D>(line, index, limits)?);
    }

    match lines.next() {
        Some(SKELETON_END) => {}
        _ => return Err(RefusalReason::RecordCount),
    }
    // The body ends with the terminator's newline, so `split` yields one trailing empty
    // piece and nothing after it. Anything else is bytes between the terminator and the
    // trailer.
    match (lines.next(), lines.next()) {
        (Some(""), None) => {}
        _ => return Err(RefusalReason::Structure { what: SKELETON_END }),
    }

    Ok(Skeleton {
        receipt_id,
        signing_key_id,
        encryption_key_id,
        records,
    })
}

fn parse_record<D: Species>(
    line: &str,
    expected_id: u64,
    limits: &ReceiptLimits,
) -> Result<SkeletonRecord, RefusalReason> {
    // The literal one-space grammar, consumed piece by piece. Never a whitespace splitter:
    // that would accept runs, tabs, and trailing spaces the format does not admit.
    let rest = line
        .strip_prefix("record ")
        .ok_or(RefusalReason::Structure { what: "record" })?;
    let (id, rest) = rest
        .split_once(' ')
        .ok_or(RefusalReason::Structure { what: "record" })?;
    if grammar::canonical_u64(id) != Some(expected_id) {
        return Err(RefusalReason::RecordIdentity);
    }
    let (kind_token, rest) = match rest.split_once(' ') {
        Some(split) => split,
        None => (rest, ""),
    };
    let kind = RecordKind::of_token(kind_token).ok_or(RefusalReason::UnknownRecordKind)?;
    if !D::KINDS.contains(&kind) {
        return Err(RefusalReason::UnknownRecordKind);
    }

    let fields: &[Field] = kind.fields();
    let mut atoms = Vec::with_capacity(fields.len());
    let mut cursor = rest;
    for (position, field) in fields.iter().enumerate() {
        let last = position.saturating_add(1) == fields.len();
        let (piece, next) = if last {
            (cursor, "")
        } else {
            cursor
                .split_once(' ')
                .ok_or(RefusalReason::FieldShape { kind: kind.token() })?
        };
        if !limits
            .field_bytes
            .admits(u64::try_from(piece.len()).unwrap_or(u64::MAX))
        {
            return Err(RefusalReason::OverBound {
                what: "field-bytes",
            });
        }
        let (key, atom) = piece
            .split_once('=')
            .ok_or(RefusalReason::FieldShape { kind: kind.token() })?;
        if key != field.key {
            return Err(RefusalReason::FieldShape { kind: kind.token() });
        }
        if !field.ty.admits(atom) {
            return Err(RefusalReason::FieldAtom { key: field.key });
        }
        atoms.push(atom.to_owned());
        cursor = next;
    }
    if !cursor.is_empty() {
        return Err(RefusalReason::FieldShape { kind: kind.token() });
    }
    SkeletonRecord::build(kind, atoms)
}

fn expect<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    want: &str,
    what: &'static str,
) -> Result<(), RefusalReason> {
    match lines.next() {
        Some(line) if line == want => Ok(()),
        _ if what == "version" => Err(RefusalReason::UnsupportedVersion),
        _ => Err(RefusalReason::Structure { what }),
    }
}

fn expect_kv<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    key: &'static str,
    want: &str,
) -> Result<(), RefusalReason> {
    let got = take_kv(lines, key)?;
    if got == want {
        Ok(())
    } else {
        Err(RefusalReason::DomainMismatch)
    }
}

fn take_kv<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    key: &'static str,
) -> Result<String, RefusalReason> {
    let line = lines.next().ok_or(RefusalReason::Structure { what: key })?;
    let rest = line
        .strip_prefix(key)
        .and_then(|r| r.strip_prefix(' '))
        .ok_or(RefusalReason::Structure { what: key })?;
    if rest.is_empty() || rest.starts_with(' ') || rest.ends_with(' ') {
        return Err(RefusalReason::Structure { what: key });
    }
    Ok(rest.to_owned())
}

/// Assemble the complete document bytes from its parts.
///
/// The one place a document's bytes come into being, so the span the signature covers and
/// the span the reader parses are the same span by construction.
#[must_use]
pub fn assemble(skeleton: &str, armor: Option<&str>, signature_hex: &str) -> Vec<u8> {
    let mut out = String::from(skeleton);
    if let Some(region) = armor {
        out.push_str(OVERLAY_BEGIN);
        out.push('\n');
        out.push_str(region);
        out.push('\n');
        out.push_str(OVERLAY_END);
        out.push('\n');
    }
    out.push_str("signature ");
    out.push_str(signature_hex);
    out.push('\n');
    out.into_bytes()
}

/// The exact span a signature is computed over, given a skeleton and an optional region.
#[must_use]
pub fn signed_body(skeleton: &str, armor: Option<&str>) -> Vec<u8> {
    let mut out = String::from(skeleton);
    if let Some(region) = armor {
        out.push_str(OVERLAY_BEGIN);
        out.push('\n');
        out.push_str(region);
        out.push('\n');
        out.push_str(OVERLAY_END);
        out.push('\n');
    }
    out.into_bytes()
}
