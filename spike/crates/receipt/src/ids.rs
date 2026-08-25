//! Identities, their domain-separated encodings, and the injective envelope every one of
//! them is computed over.
//!
//! Each public identity wraps a private shared representation, so a value minted in one
//! domain cannot construct another domain's newtype. No public constructor takes a bare
//! array, string, or digest text: a mint consumes the complete typed material and hashes it
//! in the same operation.

use sha2::{Digest as _, Sha256};

/// The injective envelope every signed body and every derived identity is computed over.
///
/// One length-prefixed encoding, so two different `(type, body)` pairs cannot produce the
/// same bytes. Used for the document signature and, with a distinct type string, for each
/// identity domain — which is what makes the domains separate rather than merely different
/// inputs.
#[must_use]
pub fn pae(payload_type: &str, body: &[u8]) -> Vec<u8> {
    let type_bytes = payload_type.as_bytes();
    let mut out = Vec::with_capacity(
        type_bytes
            .len()
            .saturating_add(body.len())
            .saturating_add(64),
    );
    out.extend_from_slice(b"DSSEv1 ");
    out.extend_from_slice(type_bytes.len().to_string().as_bytes());
    out.push(b' ');
    out.extend_from_slice(type_bytes);
    out.push(b' ');
    out.extend_from_slice(body.len().to_string().as_bytes());
    out.push(b' ');
    out.extend_from_slice(body);
    out
}

/// A SHA-256 output. Private field: the only way to obtain one is to hash something.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    /// Hash `body` under `domain`, through the injective envelope.
    #[must_use]
    pub fn over(domain: &str, body: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(pae(domain, body));
        Self(hasher.finalize().into())
    }

    /// The lowercase hexadecimal spelling, exactly 64 characters.
    #[must_use]
    pub fn hex(self) -> String {
        to_hex(&self.0)
    }

    /// Read a digest back from its exact spelling. Used only where a digest is being
    /// compared to one recomputed here; it recovers a value, never authority.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(from_hex_32(text)?))
    }
}

/// Lowercase hexadecimal, one encoding.
#[must_use]
pub fn to_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        let hi = usize::from(byte >> 4);
        let lo = usize::from(byte & 0x0f);
        if let (Some(h), Some(l)) = (DIGITS.get(hi), DIGITS.get(lo)) {
            out.push(char::from(*h));
            out.push(char::from(*l));
        }
    }
    out
}

/// Exactly 32 bytes from exactly 64 lowercase hexadecimal characters.
#[must_use]
pub fn from_hex_32(text: &str) -> Option<[u8; 32]> {
    if !crate::grammar::is_digest(text) {
        return None;
    }
    let bytes = text.as_bytes();
    let mut out = [0_u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        let hi = nibble(*bytes.get(index.checked_mul(2)?)?)?;
        let lo = nibble(*bytes.get(index.checked_mul(2)?.checked_add(1)?)?)?;
        *slot = (hi << 4) | lo;
    }
    Some(out)
}

const fn nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte.wrapping_sub(b'0')),
        b'a'..=b'f' => Some(byte.wrapping_sub(b'a').wrapping_add(10)),
        _ => None,
    }
}

/// The shared representation behind every document identity. Private: a receipt identity is
/// controller-minted per document and is not a content hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptId([u8; 32]);

impl ReceiptId {
    /// The one seam a [`ReceiptIdSource`] mints through.
    ///
    /// Takes the exact 32 bytes the source produced. The production source at the
    /// command-line edge fills them from the operating system; a source that fills them from
    /// a counter is a fixture and lives only in a test.
    #[must_use]
    pub const fn of_source_bytes(raw: [u8; 32]) -> Self {
        Self(raw)
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        to_hex(&self.0)
    }
}

/// Where a fresh document identity comes from. Injected, so no kernel reaches OS randomness
/// and a deterministic source drives every test.
pub trait ReceiptIdSource {
    /// Mint one fresh identity.
    fn next_receipt_id(&mut self) -> ReceiptId;
}

/// One plan receipt's identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanReceiptId(ReceiptId);

/// One apply intent's identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyIntentId(ReceiptId);

/// One apply outcome's identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyOutcomeId(ReceiptId);

impl PlanReceiptId {
    /// Take the next identity for a plan receipt.
    pub fn mint(source: &mut dyn ReceiptIdSource) -> Self {
        Self(source.next_receipt_id())
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover an identity from a receipt's own bytes, for correlation only.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(ReceiptId(from_hex_32(text)?)))
    }
}

impl ApplyIntentId {
    /// Take the next identity for an apply intent.
    pub fn mint(source: &mut dyn ReceiptIdSource) -> Self {
        Self(source.next_receipt_id())
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover an identity from a receipt's own bytes, for correlation only.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(ReceiptId(from_hex_32(text)?)))
    }
}

impl ApplyOutcomeId {
    /// Take the next identity for an apply outcome.
    pub fn mint(source: &mut dyn ReceiptIdSource) -> Self {
        Self(source.next_receipt_id())
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover an identity from a receipt's own bytes, for correlation only.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(ReceiptId(from_hex_32(text)?)))
    }
}

/// The shared representation behind a provider identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyId([u8; 32]);

impl KeyId {
    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        to_hex(&self.0)
    }
}

/// The domain a signing provider's identity is derived under.
pub const SIGNING_KEY_DOMAIN: &str = "application/vnd.dorc.receipt.v1.signing-key-id";

/// The domain an encryption provider's identity is derived under.
pub const ENCRYPTION_KEY_DOMAIN: &str = "application/vnd.dorc.receipt.v1.encryption-key-id";

/// A signing provider's identity, derived from its exact public verification material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SigningKeyId(KeyId);

/// An encryption provider's identity, derived from its exact recipient material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EncryptionKeyId(KeyId);

impl SigningKeyId {
    /// Derive from the exact public verification material.
    #[must_use]
    pub fn of_public_material(material: &[u8]) -> Self {
        Self(KeyId(Sha256Digest::over(SIGNING_KEY_DOMAIN, material).0))
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover an identity from a receipt's own bytes. A provider identity aids lookup and
    /// never selects an implementation or grants acceptance.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(KeyId(from_hex_32(text)?)))
    }
}

impl EncryptionKeyId {
    /// Derive from the exact recipient material.
    #[must_use]
    pub fn of_recipient_material(material: &[u8]) -> Self {
        Self(KeyId(Sha256Digest::over(ENCRYPTION_KEY_DOMAIN, material).0))
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover an identity from a receipt's own bytes, for lookup only.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(KeyId(from_hex_32(text)?)))
    }
}

/// The domain the complete planner input tuple is identified under.
pub const PLANNING_INPUT_DOMAIN: &str = "application/vnd.dorc.receipt.v1.planning-input";

/// The domain one complete approval surface is identified under.
pub const PRESENTED_PLAN_DOMAIN: &str = "application/vnd.dorc.receipt.v1.presented-plan";

/// The domain one exact apply image is identified under.
pub const APPLY_IMAGE_DOMAIN: &str = "application/vnd.dorc.apply-artifact-image.v1";

/// The complete input tuple the planner consumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanningInputId(Sha256Digest);

/// One complete approval surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentedPlanId(Sha256Digest);

/// The exact bytes and topology an apply will use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplyArtifactImageId(Sha256Digest);

impl PlanningInputId {
    /// Compute from the complete canonical encoding of the planner's inputs.
    ///
    /// Named for what it CONSUMES, and lexically fenced over its callers (`crate_boundary.rs`):
    /// nothing in the type stops a caller hashing bytes that are not a complete canonical
    /// encoding, so the gate over its callers cannot be a type. The fence names this file, which
    /// DECLARES it, and `plan/src/planning_input.rs`, the one production file that CALLS it —
    /// the module owning the typed inputs value and its encoding. A third entry means a second
    /// seat is deciding what the planner's inputs were.
    #[must_use]
    pub fn of_canonical_inputs(canonical: &[u8]) -> Self {
        Self(Sha256Digest::over(PLANNING_INPUT_DOMAIN, canonical))
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover a recorded claim, for comparison against a recomputation.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(Sha256Digest::of_hex(text)?))
    }
}

impl PresentedPlanId {
    /// Compute from the complete canonical encoding of one settled approval surface.
    ///
    /// Named for what it CONSUMES, and lexically fenced over its callers (`crate_boundary.rs`) on
    /// `of_canonical_inputs`' reasoning. Its one production caller is the seat holding the settled
    /// canonical identity plane — after the human view, the executable view, and the artifact
    /// bytes are all final.
    #[must_use]
    pub fn of_canonical_decision(canonical: &[u8]) -> Self {
        Self(Sha256Digest::over(PRESENTED_PLAN_DOMAIN, canonical))
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover a recorded claim, for comparison against a recomputation.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(Sha256Digest::of_hex(text)?))
    }
}

impl ApplyArtifactImageId {
    /// Compute from the exact canonical image encoding.
    ///
    /// Named for what it CONSUMES, and lexically fenced to its one production caller
    /// (`crate_boundary.rs`), on `of_canonical_inputs`' reasoning.
    #[must_use]
    pub fn of_canonical_image(canonical: &[u8]) -> Self {
        Self(Sha256Digest::over(APPLY_IMAGE_DOMAIN, canonical))
    }

    /// The lowercase hexadecimal spelling.
    #[must_use]
    pub fn hex(self) -> String {
        self.0.hex()
    }

    /// Recover a recorded claim, for comparison against a recomputation.
    #[must_use]
    pub fn of_hex(text: &str) -> Option<Self> {
        Some(Self(Sha256Digest::of_hex(text)?))
    }
}

/// The bare SHA-256 of a span, in lowercase hexadecimal.
///
/// Deliberately not domain-separated and deliberately not a [`Sha256Digest`]. This is a
/// binding check between a decrypted region and the skeleton it enriches, not an identity
/// that names an object, and a reader holding the span must be able to reproduce it with an
/// ordinary `sha256sum`. Never use it where an identity is expected.
#[must_use]
pub fn span_digest_hex(span: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(span);
    to_hex(&hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A deterministic source: the fixture shape every test drives, and the reason no test
    /// needs OS randomness.
    struct Counting(u8);

    impl ReceiptIdSource for Counting {
        fn next_receipt_id(&mut self) -> ReceiptId {
            let mut raw = [0_u8; 32];
            if let Some(slot) = raw.first_mut() {
                *slot = self.0;
            }
            self.0 = self.0.wrapping_add(1);
            ReceiptId::of_source_bytes(raw)
        }
    }

    #[test]
    fn the_envelope_is_injective_across_a_moved_boundary() {
        // The property the whole construction rests on: two different splits of the same
        // concatenated bytes must not encode alike, or one pair could be read as another.
        assert_ne!(pae("ab", b"c"), pae("a", b"bc"));
        assert_ne!(pae("", b"ab"), pae("a", b"b"));
        assert_eq!(pae("a", b"b"), pae("a", b"b"));
    }

    #[test]
    fn the_envelope_matches_its_specified_spelling() {
        assert_eq!(pae("t", b"body"), b"DSSEv1 1 t 4 body".to_vec());
        assert_eq!(pae("", b""), b"DSSEv1 0  0 ".to_vec());
    }

    #[test]
    fn separate_domains_over_one_body_produce_separate_digests() {
        let body = b"identical";
        assert_ne!(
            Sha256Digest::over(PLANNING_INPUT_DOMAIN, body),
            Sha256Digest::over(PRESENTED_PLAN_DOMAIN, body),
        );
        assert_ne!(
            Sha256Digest::over(PRESENTED_PLAN_DOMAIN, body),
            Sha256Digest::over(APPLY_IMAGE_DOMAIN, body),
        );
        assert_ne!(
            Sha256Digest::over(SIGNING_KEY_DOMAIN, body),
            Sha256Digest::over(ENCRYPTION_KEY_DOMAIN, body),
        );
    }

    #[test]
    fn a_digest_round_trips_through_its_exact_spelling() {
        let digest = Sha256Digest::over(PLANNING_INPUT_DOMAIN, b"x");
        let text = digest.hex();
        assert!(crate::grammar::is_digest(&text));
        assert_eq!(Sha256Digest::of_hex(&text), Some(digest));
        assert_eq!(Sha256Digest::of_hex(&text.to_uppercase()), None);
        assert_eq!(Sha256Digest::of_hex(&text[..63]), None);
    }

    #[test]
    fn hex_round_trips_every_byte_value() {
        let all: Vec<u8> = (0..=u8::MAX).collect();
        let text = to_hex(&all);
        assert_eq!(text.len(), 512);
        assert!(
            text.bytes()
                .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
        );
        let head: [u8; 32] = all.get(..32).and_then(|s| s.try_into().ok()).expect("32");
        assert_eq!(from_hex_32(&to_hex(&head)), Some(head));
    }

    #[test]
    fn a_minted_identity_is_taken_from_the_source_not_from_content() {
        // A document identity is controller-minted per document; two receipts over identical
        // content must not collide, which a content hash would guarantee they do.
        let mut source = Counting(7);
        let first = PlanReceiptId::mint(&mut source);
        let second = PlanReceiptId::mint(&mut source);
        assert_ne!(first, second);
        assert_eq!(PlanReceiptId::of_hex(&first.hex()), Some(first));
    }

    #[test]
    fn a_provider_identity_follows_its_exact_material() {
        let a = SigningKeyId::of_public_material(b"material-a");
        let b = SigningKeyId::of_public_material(b"material-b");
        assert_ne!(a, b);
        assert_eq!(SigningKeyId::of_public_material(b"material-a"), a);
        // The two provider roles derive under separate domains, so one material cannot
        // produce a matching identity in the other role.
        assert_ne!(
            SigningKeyId::of_public_material(b"m").hex(),
            EncryptionKeyId::of_recipient_material(b"m").hex(),
        );
    }
}
