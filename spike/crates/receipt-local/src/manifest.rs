//! The keyset manifest: `dorc-receipt-keyset/1`, its exact bytes, and the one parser that reads
//! them.
//!
//! # What the manifest is for
//!
//! Two things, and neither is trust. It is the COMPLETION marker — written last, so a keyset
//! without a valid one was never finished and is never treated as first use — and it is the
//! CROSS-FILE agreement, so a signing document that does not match the identity beside it is a
//! refusal rather than a keyset that half works.
//!
//! It selects nothing. No backend, no algorithm, no path, no provider. Loading derives both
//! identities from the private documents themselves and compares; the manifest is the second
//! opinion, never the source.
//!
//! # Why the grammar is this narrow
//!
//! Four lines, fixed order, no comments, no blank lines, no optional fields, no ignored bytes,
//! and a hard byte bound before parsing. There is exactly one writer form and the reader accepts
//! exactly it — the discipline the receipt grammar already runs on, for the same reason: a
//! permissive parser over a file that gates key material is a second, unreviewed way to say what
//! a keyset is.

use crate::limits::LocalLimits;

/// The version line, which is also the format's name.
pub const MANIFEST_VERSION_LINE: &str = "dorc-receipt-keyset/1";

/// The line closing the manifest. EOF follows it immediately.
pub const MANIFEST_END: &str = "keyset-end";

/// One keyset's manifest: the two identities it claims, in the order they are written.
///
/// Private fields and no `Default`: a manifest exists because bytes said so, and an all-zero one
/// would be a keyset claiming two identities nothing derived.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeysetManifest {
    signing_key_id: String,
    encryption_key_id: String,
}

/// Why a manifest was not accepted.
///
/// Closed, and pointed: each arm names one departure, so a caller reporting a damaged keyset can
/// say which line was wrong rather than that the file "did not parse". Never one `Malformed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestRefusal {
    /// The file is larger than a manifest may be. Checked before parsing, never after.
    OverBound,
    /// A byte the grammar does not admit anywhere — a carriage return or a tab.
    IllegalByte {
        /// Which byte.
        byte: u8,
    },
    /// The version line was absent or named a keyset version this reader does not implement.
    UnsupportedVersion,
    /// A required line was absent, out of order, or misspelled.
    Structure {
        /// Which line.
        what: &'static str,
    },
    /// An identity that was not exactly sixty-four lowercase hexadecimal characters.
    IdentityNotExactDigest {
        /// Which role.
        role: KeyRole,
    },
    /// Bytes after the terminator's newline.
    BytesAfterEnd,
}

/// Which of the two independent roles a value belongs to.
///
/// One type rather than a bare string, and no conversion between the two arms: the whole point of
/// the pair is that they are separately generated, separately stored, and never derived from one
/// another, so the word naming them may not be a value a caller can pick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyRole {
    /// The Ed25519 signing role.
    Signing,
    /// The Age X25519 encryption role.
    Encryption,
}

impl KeyRole {
    /// Both roles.
    pub const ALL: [Self; 2] = [Self::Signing, Self::Encryption];

    /// The manifest line key this role is written under.
    #[must_use]
    pub const fn manifest_key(self) -> &'static str {
        match self {
            Self::Signing => "signing-key-id",
            Self::Encryption => "encryption-key-id",
        }
    }
}

impl KeysetManifest {
    /// Bind the two identities a completed keyset claims.
    ///
    /// # Errors
    /// Refuses either identity that is not exactly the one spelling.
    pub fn of(signing_key_id: &str, encryption_key_id: &str) -> Result<Self, ManifestRefusal> {
        for (role, text) in [
            (KeyRole::Signing, signing_key_id),
            (KeyRole::Encryption, encryption_key_id),
        ] {
            if !is_lower_hex_64(text) {
                return Err(ManifestRefusal::IdentityNotExactDigest { role });
            }
        }
        Ok(Self {
            signing_key_id: signing_key_id.to_owned(),
            encryption_key_id: encryption_key_id.to_owned(),
        })
    }

    /// The identity claimed for `role`, as spelled.
    #[must_use]
    pub fn claimed(&self, role: KeyRole) -> &str {
        match role {
            KeyRole::Signing => &self.signing_key_id,
            KeyRole::Encryption => &self.encryption_key_id,
        }
    }

    /// The one serialization.
    #[must_use]
    pub fn serialize(&self) -> String {
        format!(
            "{MANIFEST_VERSION_LINE}\n{} {}\n{} {}\n{MANIFEST_END}\n",
            KeyRole::Signing.manifest_key(),
            self.signing_key_id,
            KeyRole::Encryption.manifest_key(),
            self.encryption_key_id,
        )
    }

    /// Read a manifest from its exact bytes.
    ///
    /// The bound is consulted BEFORE anything is decoded, so an oversized file costs a length
    /// check rather than a conversion.
    ///
    /// # Errors
    /// Refuses an over-bound file, an illegal byte, an unknown version, a missing or misordered
    /// line, an identity outside its one spelling, and any byte after the terminator.
    pub fn parse(bytes: &[u8], limits: &LocalLimits) -> Result<Self, ManifestRefusal> {
        if bytes.len() > limits.manifest_bytes {
            return Err(ManifestRefusal::OverBound);
        }
        let text =
            core::str::from_utf8(bytes).map_err(|_| ManifestRefusal::IllegalByte { byte: 0 })?;
        if let Some(byte) = text.bytes().find(|b| *b == b'\r' || *b == b'\t') {
            return Err(ManifestRefusal::IllegalByte { byte });
        }
        let mut lines = text.split('\n');

        match lines.next() {
            Some(line) if line == MANIFEST_VERSION_LINE => {}
            _ => return Err(ManifestRefusal::UnsupportedVersion),
        }
        let signing = take_identity(&mut lines, KeyRole::Signing)?;
        let encryption = take_identity(&mut lines, KeyRole::Encryption)?;
        match lines.next() {
            Some(line) if line == MANIFEST_END => {}
            _ => return Err(ManifestRefusal::Structure { what: MANIFEST_END }),
        }
        // The terminator's newline makes `split` yield one trailing empty piece and nothing
        // after it. Anything else is bytes past the end, which are refused rather than skipped.
        match (lines.next(), lines.next()) {
            (Some(""), None) => {}
            _ => return Err(ManifestRefusal::BytesAfterEnd),
        }

        Self::of(&signing, &encryption)
    }
}

fn take_identity<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
    role: KeyRole,
) -> Result<String, ManifestRefusal> {
    let key = role.manifest_key();
    let line = lines
        .next()
        .ok_or(ManifestRefusal::Structure { what: key })?;
    let value = line
        .strip_prefix(key)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or(ManifestRefusal::Structure { what: key })?;
    if !is_lower_hex_64(value) {
        return Err(ManifestRefusal::IdentityNotExactDigest { role });
    }
    Ok(value.to_owned())
}

fn is_lower_hex_64(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_roles_are_written_under_distinct_keys() {
        assert_ne!(
            KeyRole::Signing.manifest_key(),
            KeyRole::Encryption.manifest_key()
        );
    }

    #[test]
    fn a_manifest_round_trips_through_its_exact_bytes() {
        let manifest = KeysetManifest::of(&"a".repeat(64), &"b".repeat(64)).expect("well formed");
        let text = manifest.serialize();
        assert_eq!(
            KeysetManifest::parse(text.as_bytes(), &LocalLimits::V1),
            Ok(manifest)
        );
    }

    #[test]
    fn the_bound_is_checked_before_the_bytes_are_decoded() {
        // A file may be arbitrarily large; the refusal has to be the bound rather than whatever
        // the decoder would have said about the first megabyte of it.
        let oversized = vec![b'x'; LocalLimits::V1.manifest_bytes.saturating_add(1)];
        assert_eq!(
            KeysetManifest::parse(&oversized, &LocalLimits::V1),
            Err(ManifestRefusal::OverBound)
        );
        // Boundary-at: a file exactly at the bound is admitted to the parser, which then refuses
        // it for its own reasons rather than for its size.
        let at_bound = vec![b'x'; LocalLimits::V1.manifest_bytes];
        assert_eq!(
            KeysetManifest::parse(&at_bound, &LocalLimits::V1),
            Err(ManifestRefusal::UnsupportedVersion)
        );
    }
}
