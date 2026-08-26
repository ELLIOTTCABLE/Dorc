//! `dorc-receipt-crypto` — the implementations of `dorc-receipt`'s capability traits.
//!
//! This crate exists so the randomness these packages carry stays out of the analyzer
//! kernel's dependency graph: `dorc-plan` depends on `dorc-receipt` alone, and only the
//! command-line edge depends on this crate.
//!
//! Nothing here mints a checked, trusted, or complete receipt state. A signer answers bytes,
//! a verifier answers a boolean, an opener answers bounded plaintext; the states those feed
//! are constructed in `dorc-receipt` and are unreachable from here.
//!
//! The one lint carve here is about the dependency graph rather than this crate's code:
//! `age` reaches two major lines of `sha2`, `digest`, `crypto-common`, `block-buffer`,
//! `cpufeatures`, `const-oid`, and `hybrid-array` through separate subtrees, which
//! `clippy::multiple_crate_versions` reports and `-D warnings` then makes fatal. Both current
//! `age` lines carry it, so no version choice avoids it, and `deny.toml` already sets
//! `multiple-versions = "warn"` for the workspace. Scoped here rather than to `clippy.toml`,
//! whose workspace-wide key would also have to name `syn`, `thiserror`, and `thiserror-impl` —
//! ordinary ecosystem churn unrelated to this dependency. `expect`, so it warns once the
//! duplication clears.
#![expect(
    clippy::multiple_crate_versions,
    reason = "a transitive-dependency fact; see the module note above"
)]

pub mod key_document;

pub use key_document::{
    EncryptionPrivateDocument, EntropyKeysetGenerator, GeneratedKeysetV1, KeyDocumentBound,
    KeyDocumentRefusal, KeySecretEntropy, KeysetGenerator, LineDeparture, SigningPrivateDocument,
};

use age::armor::{ArmoredReader, ArmoredWriter, Format};
use age::x25519;
use dorc_receipt::capability::{
    OverlayOpener, OverlaySealer, ReceiptSigner, ReceiptVerifier,
    SelfAssertedReceiptVerificationKey, TrustedReceiptVerificationKey,
};
use dorc_receipt::ids::{EncryptionKeyId, SigningKeyId};
use ed25519_dalek::{Signature, Signer as _, SigningKey, VerifyingKey};
use std::io::{Read as _, Write as _};

/// Signs with one Ed25519 key.
#[derive(Debug)]
pub struct Ed25519Signer {
    key: SigningKey,
}

impl Ed25519Signer {
    /// Take an existing key by its exact 32 secret bytes.
    ///
    /// No generation surface: a key is supplied, never invented here.
    #[must_use]
    pub fn of_secret(secret: [u8; 32]) -> Self {
        Self {
            key: SigningKey::from_bytes(&secret),
        }
    }

    /// The exact public verification material.
    #[must_use]
    pub fn public_material(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }

    /// The key itself, for the one module that must encode it as a document.
    pub(crate) const fn key(&self) -> &SigningKey {
        &self.key
    }
}

impl ReceiptSigner for Ed25519Signer {
    fn signing_key_id(&self) -> SigningKeyId {
        SigningKeyId::of_public_material(&self.public_material())
    }

    fn sign(&self, body: &[u8]) -> [u8; 64] {
        self.key.sign(body).to_bytes()
    }
}

/// Checks against one Ed25519 public key, under strict verification.
#[derive(Debug)]
pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Verifier {
    /// Take public material by its exact 32 bytes.
    #[must_use]
    pub fn of_public_material(material: [u8; 32]) -> Option<Self> {
        VerifyingKey::from_bytes(&material)
            .ok()
            .map(|key| Self { key })
    }

    /// This provider's identity.
    #[must_use]
    pub fn id(&self) -> SigningKeyId {
        SigningKeyId::of_public_material(&self.key.to_bytes())
    }
}

impl ReceiptVerifier for Ed25519Verifier {
    fn verify(&self, body: &[u8], signature: &[u8; 64]) -> bool {
        let candidate = Signature::from_bytes(signature);
        self.key.verify_strict(body, &candidate).is_ok()
    }
}

/// Verification material whose provider controller policy names.
#[derive(Debug)]
pub struct TrustedEd25519Key(Ed25519Verifier);

/// Verification material whose provider controller policy does not name.
#[derive(Debug)]
pub struct SelfAssertedEd25519Key(Ed25519Verifier);

impl TrustedEd25519Key {
    /// Mark material as named by controller policy.
    #[must_use]
    pub const fn of(verifier: Ed25519Verifier) -> Self {
        Self(verifier)
    }
}

impl SelfAssertedEd25519Key {
    /// Mark material as not named by controller policy.
    #[must_use]
    pub const fn of(verifier: Ed25519Verifier) -> Self {
        Self(verifier)
    }
}

impl ReceiptVerifier for TrustedEd25519Key {
    fn verify(&self, body: &[u8], signature: &[u8; 64]) -> bool {
        self.0.verify(body, signature)
    }
}

impl TrustedReceiptVerificationKey for TrustedEd25519Key {
    fn signing_key_id(&self) -> SigningKeyId {
        self.0.id()
    }
}

impl ReceiptVerifier for SelfAssertedEd25519Key {
    fn verify(&self, body: &[u8], signature: &[u8; 64]) -> bool {
        self.0.verify(body, signature)
    }
}

impl SelfAssertedReceiptVerificationKey for SelfAssertedEd25519Key {
    fn signing_key_id(&self) -> SigningKeyId {
        self.0.id()
    }
}

/// Seals one region to a single recipient, in canonical armor.
pub struct AgeSealer {
    recipient: x25519::Recipient,
}

impl core::fmt::Debug for AgeSealer {
    /// Names the type and no material.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AgeSealer")
    }
}

impl AgeSealer {
    /// Take one recipient.
    #[must_use]
    pub const fn of(recipient: x25519::Recipient) -> Self {
        Self { recipient }
    }

    /// Take one recipient by its exact public encoding.
    ///
    /// Material is SUPPLIED, never invented: there is deliberately no generation surface here,
    /// and this constructor adds none — it parses what a provider already holds.
    #[must_use]
    pub fn of_recipient_text(text: &str) -> Option<Self> {
        text.parse().ok().map(Self::of)
    }
}

impl OverlaySealer for AgeSealer {
    fn encryption_key_id(&self) -> EncryptionKeyId {
        EncryptionKeyId::of_recipient_material(self.recipient.to_string().as_bytes())
    }

    fn seal(&self, plaintext: &[u8]) -> Option<String> {
        let encryptor = age::Encryptor::with_recipients(core::iter::once(
            &self.recipient as &dyn age::Recipient,
        ))
        .ok()?;
        let mut armored = Vec::new();
        let armor = ArmoredWriter::wrap_output(&mut armored, Format::AsciiArmor).ok()?;
        let mut writer = encryptor.wrap_output(armor).ok()?;
        writer.write_all(plaintext).ok()?;
        writer.finish().ok()?.finish().ok()?;
        let text = String::from_utf8(armored).ok()?;
        // This writer emits CRLF, and the receipt grammar admits LF only, so the stored form
        // is normalized here at the seam. Line endings are framing around the base64 payload,
        // not part of the ciphertext, and the reader below accepts the LF form; the outer
        // signature then binds exactly what is stored. The region is stored without a trailing
        // newline, because the format supplies the one that closes the region.
        let lf = text.replace("\r\n", "\n");
        Some(lf.trim_end_matches(['\n', '\r']).to_owned())
    }
}

/// Opens one region with one identity.
pub struct AgeOpener {
    identity: x25519::Identity,
}

impl core::fmt::Debug for AgeOpener {
    /// Names the type and no material.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AgeOpener")
    }
}

impl AgeOpener {
    /// Take one identity.
    #[must_use]
    pub const fn of(identity: x25519::Identity) -> Self {
        Self { identity }
    }

    /// Take one identity by its exact secret encoding. Parses; never generates.
    #[must_use]
    pub fn of_identity_text(text: &str) -> Option<Self> {
        text.parse().ok().map(Self::of)
    }

    /// The PUBLIC half of the material this holds, for naming the recipient a sealer needs.
    ///
    /// Public by construction, so answering it discloses nothing the recipient encoding does not
    /// already say.
    #[must_use]
    pub fn recipient_text(&self) -> String {
        self.identity.to_public().to_string()
    }

    /// The public recipient, so a document holding one of these can hand out a sealer without
    /// a second copy of the secret half existing anywhere.
    pub(crate) fn recipient(&self) -> x25519::Recipient {
        self.identity.to_public()
    }

    /// The identity itself, for the one module that must encode it as a document.
    pub(crate) const fn identity(&self) -> &x25519::Identity {
        &self.identity
    }
}

impl OverlayOpener for AgeOpener {
    fn open(&self, armor: &str, max_bytes: u64) -> Option<Vec<u8>> {
        let mut framed = String::from(armor);
        framed.push('\n');
        let reader = ArmoredReader::new(framed.as_bytes());
        let decryptor = age::Decryptor::new(reader).ok()?;
        let mut stream = decryptor
            .decrypt(std::iter::once(&self.identity as &dyn age::Identity))
            .ok()?;
        let mut out = Vec::new();
        // Read one byte past the bound so an oversized region is refused rather than
        // silently truncated into something that looks whole.
        let ceiling = max_bytes.saturating_add(1);
        let mut limited = stream.by_ref().take(ceiling);
        limited.read_to_end(&mut out).ok()?;
        if u64::try_from(out.len()).unwrap_or(u64::MAX) > max_bytes {
            return None;
        }
        Some(out)
    }
}
