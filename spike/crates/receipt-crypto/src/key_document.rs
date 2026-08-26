//! The two private key documents, in the encodings their own libraries own.
//!
//! # The encodings are not this project's
//!
//! A signing document is canonical unencrypted PKCS#8 DER, emitted and parsed by the signature
//! package. An encryption document is exactly one canonical identity line, emitted and parsed by
//! the encryption package. Nothing here invents a container, a header, a length prefix, or a
//! wrapper: an encoding authored in this crate would be a second, unreviewed answer to what a
//! key IS, in the one place where being wrong is unrecoverable.
//!
//! Canonicality is therefore checked rather than assumed. Loading parses through the library and
//! serializes back through the same library, and requires byte equality with the bounded input
//! before the material is accepted. A document a library would write differently is refused as
//! non-canonical — separately from malformed, because it is material a library ACCEPTED, and the
//! two say different things about what is on disk.
//!
//! # The two roles never meet
//!
//! They are generated independently, from independent sources, and neither is derived from the
//! other or from a shared stored root. There is no conversion in either direction and no
//! constructor that takes the other role's material: the identities are separate newtypes owned
//! by the pure crate, and the documents below expose no bytes a caller could carry across.
//!
//! # What a document will not do
//!
//! No `Clone`, no `Default`, no equality, no ordering, no hash, no serde, and no accessor
//! handing out private bytes. The exclusive write callback is the one way canonical bytes leave,
//! and what it hands over lives only for that call.
//!
//! # The seals, as compile-fail pins
//!
//! Each is paired with the positive control that proves it fails for the stated reason rather
//! than because the example was malformed.
//!
//! A document is obtained by generating one or by reading one. There is no constructor over bare
//! secret bytes, so no second place can decide what this project's key material is:
//!
//! ```
//! use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator};
//! struct Fixed;
//! impl KeySecretEntropy for Fixed {
//!     fn fill(&mut self, raw: &mut [u8; 32]) -> bool { raw.fill(1); true }
//! }
//! let keyset = EntropyKeysetGenerator::over(Fixed).generate().expect("generated");
//! assert_ne!(keyset.signing().signing_key_id().hex(), String::new());
//! ```
//!
//! ```compile_fail
//! let _ = dorc_receipt_crypto::SigningPrivateDocument::of_secret_bytes(&[0_u8; 32]);
//! ```
//!
//! The two roles are non-convertible in BOTH directions, and that is a property of the types
//! rather than of anyone's discipline. A signing document is not an encryption document:
//!
//! ```compile_fail
//! fn wants(_: dorc_receipt_crypto::EncryptionPrivateDocument) {}
//! fn give(signing: dorc_receipt_crypto::SigningPrivateDocument) { wants(signing); }
//! ```
//!
//! ```compile_fail
//! fn wants(_: dorc_receipt_crypto::SigningPrivateDocument) {}
//! fn give(encryption: dorc_receipt_crypto::EncryptionPrivateDocument) { wants(encryption); }
//! ```
//!
//! ... and neither is their identity, so a manifest cannot record one role's value under the
//! other's key by passing the wrong argument:
//!
//! ```
//! use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator};
//! struct Fixed;
//! impl KeySecretEntropy for Fixed {
//!     fn fill(&mut self, raw: &mut [u8; 32]) -> bool { raw.fill(2); true }
//! }
//! fn wants_encryption(_: dorc_receipt::ids::EncryptionKeyId) {}
//! let keyset = EntropyKeysetGenerator::over(Fixed).generate().expect("generated");
//! wants_encryption(keyset.encryption().encryption_key_id());
//! ```
//!
//! ```compile_fail
//! use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator};
//! struct Fixed;
//! impl KeySecretEntropy for Fixed {
//!     fn fill(&mut self, raw: &mut [u8; 32]) -> bool { raw.fill(2); true }
//! }
//! fn wants_encryption(_: dorc_receipt::ids::EncryptionKeyId) {}
//! let keyset = EntropyKeysetGenerator::over(Fixed).generate().expect("generated");
//! wants_encryption(keyset.signing().signing_key_id());
//! ```
//!
//! Canonical bytes may be COPIED out of the write callback and may not ESCAPE it, so a caller
//! cannot hold a borrow of material whose container has already erased itself:
//!
//! ```
//! use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator};
//! struct Fixed;
//! impl KeySecretEntropy for Fixed {
//!     fn fill(&mut self, raw: &mut [u8; 32]) -> bool { raw.fill(3); true }
//! }
//! let keyset = EntropyKeysetGenerator::over(Fixed).generate().expect("generated");
//! let copied: Vec<u8> = keyset.encryption().with_canonical_bytes(<[u8]>::to_vec);
//! assert!(!copied.is_empty());
//! ```
//!
//! ```compile_fail
//! use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator};
//! struct Fixed;
//! impl KeySecretEntropy for Fixed {
//!     fn fill(&mut self, raw: &mut [u8; 32]) -> bool { raw.fill(3); true }
//! }
//! let keyset = EntropyKeysetGenerator::over(Fixed).generate().expect("generated");
//! let escaped: &[u8] = keyset.encryption().with_canonical_bytes(|bytes| bytes);
//! ```
//!
//! And nothing here is `Clone` or `Default`, so no route produces a second copy of a secret or
//! a keyset nothing generated. The bound is demanded BY VALUE, deliberately: writing
//! `document.clone()` against a borrow resolves to `<&T as Clone>` and merely copies the
//! reference, which rustc reports as a lint and rustdoc counts as a successful compile — a pin
//! written that way passes whether or not the type is cloneable.
//!
//! ```
//! fn needs_clone<T: Clone>(_: T) {}
//! fn check(text: String) { needs_clone(text); }
//! ```
//!
//! ```compile_fail
//! fn needs_clone<T: Clone>(_: T) {}
//! fn check(document: dorc_receipt_crypto::SigningPrivateDocument) { needs_clone(document); }
//! ```
//!
//! ```compile_fail
//! fn needs_clone<T: Clone>(_: T) {}
//! fn check(document: dorc_receipt_crypto::EncryptionPrivateDocument) { needs_clone(document); }
//! ```
//!
//! ```compile_fail
//! fn needs_clone<T: Clone>(_: T) {}
//! fn check(keyset: dorc_receipt_crypto::GeneratedKeysetV1) { needs_clone(keyset); }
//! ```
//!
//! ```compile_fail
//! let _unearned: dorc_receipt_crypto::EncryptionPrivateDocument = Default::default();
//! ```

use age::secrecy::{ExposeSecret as _, SecretString};
use age::x25519;
use dorc_receipt::capability::{OverlayOpener, ReceiptSigner};
use dorc_receipt::ids::{EncryptionKeyId, SigningKeyId};
use ed25519_dalek::SigningKey;
use ed25519_dalek::pkcs8::{DecodePrivateKey as _, EncodePrivateKey as _};
use zeroize::Zeroize as _;

use crate::{AgeOpener, AgeSealer, Ed25519Signer, Ed25519Verifier};

/// How many bytes of a key document a caller will admit before parsing it.
///
/// A newtype rather than a bare integer because the two documents are bounded separately by
/// policy the local edge owns; this crate holds no bound of its own and takes the caller's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyDocumentBound(usize);

impl KeyDocumentBound {
    /// Admit at most `bytes`.
    #[must_use]
    pub const fn of(bytes: usize) -> Self {
        Self(bytes)
    }

    /// The bound.
    #[must_use]
    pub const fn bytes(self) -> usize {
        self.0
    }
}

/// Why a key document was not accepted.
///
/// Closed, and each arm is a different fact about what is on disk. Malformed and non-canonical
/// are deliberately not one arm: the first is material no library would produce, and the second
/// is material a library accepted and would rewrite, which the local edge reports as its own
/// state and never repairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyDocumentRefusal {
    /// Larger than the caller's bound. Checked before anything is decoded.
    OverBound,
    /// The identity document is not exactly one canonical line. Encryption role only.
    NotOneCanonicalLine {
        /// How it departed.
        departure: LineDeparture,
    },
    /// The library refused to parse it.
    Malformed,
    /// The library parsed it and would write it differently.
    NonCanonical,
}

/// How an identity document departed from its one line.
///
/// Named departures rather than one refusal, because each is a different thing to have happened
/// to a file: a stray carriage return is a transport that rewrote it, a second line is somebody
/// keeping two identities in one place, and surrounding whitespace is an editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineDeparture {
    /// Not text at all.
    NotText,
    /// A carriage return anywhere.
    CarriageReturn,
    /// No final newline.
    NoFinalNewline,
    /// More than one line before the terminator — a second identity, a blank, or a comment.
    MoreThanOneLine,
    /// The line is empty.
    Empty,
    /// Space or tab around the identity.
    SurroundingWhitespace,
    /// A comment line.
    Comment,
}

/// One Ed25519 signing key, as the document that holds it.
///
/// It IS the signing capability: a caller with one can sign, and a caller without one cannot.
/// That is deliberately not the same shape as a bundle carrying a copy of the key beside it —
/// there is one copy of the secret in the process, and it lives here.
pub struct SigningPrivateDocument {
    signer: Ed25519Signer,
}

impl core::fmt::Debug for SigningPrivateDocument {
    /// Names the type and the public identity, and no material.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SigningPrivateDocument")
            .field("signing_key_id", &self.signing_key_id().hex())
            .finish()
    }
}

impl SigningPrivateDocument {
    /// Take the exact secret bytes a generator produced.
    ///
    /// Crate-private: the public way to obtain one is to generate it or to parse a document.
    /// A public constructor over bare bytes would be a second place able to decide what this
    /// project's signing material is.
    pub(crate) fn of_secret_bytes(secret: &[u8; 32]) -> Self {
        Self {
            signer: Ed25519Signer::of_secret(*secret),
        }
    }

    /// Read a signing document from its exact bytes.
    ///
    /// # Errors
    /// Refuses an over-bound document, one the library will not parse, and one the library would
    /// write differently.
    pub fn parse(bytes: &[u8], bound: KeyDocumentBound) -> Result<Self, KeyDocumentRefusal> {
        if bytes.len() > bound.bytes() {
            return Err(KeyDocumentRefusal::OverBound);
        }
        let key = SigningKey::from_pkcs8_der(bytes).map_err(|_| KeyDocumentRefusal::Malformed)?;
        let document = Self {
            signer: Ed25519Signer::of_secret(key.to_bytes()),
        };
        let canonical = document
            .with_canonical_bytes(|written| written == bytes)
            .ok_or(KeyDocumentRefusal::Malformed)?;
        if canonical {
            Ok(document)
        } else {
            Err(KeyDocumentRefusal::NonCanonical)
        }
    }

    /// Hand the canonical document bytes to `write`, for exactly that call.
    ///
    /// The one way they leave. The library's own zeroizing container holds them and drops at the
    /// end of this call, so no copy outlives the write it was produced for. Answers `None` where
    /// the library declines to encode, which is a refusal and never an empty document.
    pub fn with_canonical_bytes<R>(&self, write: impl FnOnce(&[u8]) -> R) -> Option<R> {
        let encoded = self.signer.key().to_pkcs8_der().ok()?;
        Some(write(encoded.as_bytes()))
    }

    /// This document's provider identity, derived from its PUBLIC material.
    #[must_use]
    pub fn signing_key_id(&self) -> SigningKeyId {
        self.signer.signing_key_id()
    }

    /// The exact public verification material.
    #[must_use]
    pub fn verification_material(&self) -> [u8; 32] {
        self.signer.public_material()
    }

    /// Verification material for this document's own key.
    ///
    /// Answers `None` only where the public half does not load, which cannot happen for material
    /// this crate produced and can for material a caller reconstructed.
    #[must_use]
    pub fn verifier(&self) -> Option<Ed25519Verifier> {
        Ed25519Verifier::of_public_material(self.verification_material())
    }

    /// The signer, for a caller wanting the concrete capability rather than the trait.
    #[must_use]
    pub const fn as_signer(&self) -> &Ed25519Signer {
        &self.signer
    }
}

impl ReceiptSigner for SigningPrivateDocument {
    fn signing_key_id(&self) -> SigningKeyId {
        self.signer.signing_key_id()
    }

    fn sign(&self, body: &[u8]) -> [u8; 64] {
        self.signer.sign(body)
    }
}

/// One Age X25519 identity, as the document that holds it.
///
/// Like its signing sibling it IS the capability: holding one is what lets a region be opened.
/// The public half is answered freely, because a recipient encoding discloses nothing the
/// document does not already publish.
pub struct EncryptionPrivateDocument {
    opener: AgeOpener,
}

impl core::fmt::Debug for EncryptionPrivateDocument {
    /// Names the type and the public identity, and no material.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EncryptionPrivateDocument")
            .field("encryption_key_id", &self.encryption_key_id().hex())
            .finish()
    }
}

impl EncryptionPrivateDocument {
    /// Take one identity the package generated.
    pub(crate) fn of_identity(identity: x25519::Identity) -> Self {
        Self {
            opener: AgeOpener::of(identity),
        }
    }

    /// Read an identity document from its exact bytes.
    ///
    /// # Errors
    /// Refuses an over-bound document, anything that is not exactly one line ending in one
    /// newline, one the library will not parse, and one the library would write differently.
    pub fn parse(bytes: &[u8], bound: KeyDocumentBound) -> Result<Self, KeyDocumentRefusal> {
        if bytes.len() > bound.bytes() {
            return Err(KeyDocumentRefusal::OverBound);
        }
        let line = sole_line(bytes)
            .map_err(|departure| KeyDocumentRefusal::NotOneCanonicalLine { departure })?;
        let identity: x25519::Identity = line.parse().map_err(|_| KeyDocumentRefusal::Malformed)?;
        let document = Self::of_identity(identity);
        if document.with_canonical_bytes(|written| written == bytes) {
            Ok(document)
        } else {
            Err(KeyDocumentRefusal::NonCanonical)
        }
    }

    /// Hand the canonical document bytes to `write`, for exactly that call.
    ///
    /// The package answers its identity inside a container that erases itself, and the newline
    /// the document ends in is appended into a second one; both drop at the end of this call.
    pub fn with_canonical_bytes<R>(&self, write: impl FnOnce(&[u8]) -> R) -> R {
        let identity = self.opener.identity().to_string();
        let document = SecretString::from(format!("{}\n", identity.expose_secret()));
        write(document.expose_secret().as_bytes())
    }

    /// This document's provider identity, derived from its PUBLIC recipient encoding.
    #[must_use]
    pub fn encryption_key_id(&self) -> EncryptionKeyId {
        EncryptionKeyId::of_recipient_material(self.recipient_text().as_bytes())
    }

    /// The public recipient encoding.
    #[must_use]
    pub fn recipient_text(&self) -> String {
        self.opener.recipient_text()
    }

    /// A sealer for this document's own recipient.
    ///
    /// Carries the public half only, so handing one out copies nothing secret.
    #[must_use]
    pub fn sealer(&self) -> AgeSealer {
        AgeSealer::of(self.opener.recipient())
    }

    /// The opener, for a caller wanting the concrete capability rather than the trait.
    #[must_use]
    pub const fn as_opener(&self) -> &AgeOpener {
        &self.opener
    }
}

impl OverlayOpener for EncryptionPrivateDocument {
    fn open(&self, armor: &str, max_bytes: u64) -> Option<Vec<u8>> {
        self.opener.open(armor, max_bytes)
    }
}

/// The exactly-one-line rule, as its own function so every departure is named.
fn sole_line(bytes: &[u8]) -> Result<&str, LineDeparture> {
    let text = core::str::from_utf8(bytes).map_err(|_| LineDeparture::NotText)?;
    if text.contains('\r') {
        return Err(LineDeparture::CarriageReturn);
    }
    let body = text
        .strip_suffix('\n')
        .ok_or(LineDeparture::NoFinalNewline)?;
    if body.contains('\n') {
        return Err(LineDeparture::MoreThanOneLine);
    }
    if body.is_empty() {
        return Err(LineDeparture::Empty);
    }
    if body.starts_with('#') {
        return Err(LineDeparture::Comment);
    }
    if body.trim() != body {
        return Err(LineDeparture::SurroundingWhitespace);
    }
    Ok(body)
}

/// Both key documents, generated independently, held in memory.
///
/// One value rather than two returns, because the sequence that consumes it must hold both
/// before it creates anything on disk: a generator that could answer one role and fail the other
/// would leave the durable act to discover the failure halfway through.
pub struct GeneratedKeysetV1 {
    signing: SigningPrivateDocument,
    encryption: EncryptionPrivateDocument,
}

impl core::fmt::Debug for GeneratedKeysetV1 {
    /// Names the type and the two public identities, and no material.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GeneratedKeysetV1")
            .field("signing_key_id", &self.signing.signing_key_id().hex())
            .field(
                "encryption_key_id",
                &self.encryption.encryption_key_id().hex(),
            )
            .finish()
    }
}

impl GeneratedKeysetV1 {
    /// The signing document.
    #[must_use]
    pub const fn signing(&self) -> &SigningPrivateDocument {
        &self.signing
    }

    /// The encryption document.
    #[must_use]
    pub const fn encryption(&self) -> &EncryptionPrivateDocument {
        &self.encryption
    }

    /// Take both documents apart.
    #[must_use]
    pub fn into_parts(self) -> (SigningPrivateDocument, EncryptionPrivateDocument) {
        (self.signing, self.encryption)
    }
}

/// Where a production edge's secret key bytes come from.
///
/// Separate from the identity source the receipt crate declares, and deliberately not
/// interchangeable with it: one produces the unpredictable part of a document identity and the
/// other produces a key, and no value of either kind can become the other.
pub trait KeySecretEntropy {
    /// Fill `raw` with fresh unpredictable bytes, answering whether the platform could.
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool;
}

/// Where a keyset's two documents come from.
///
/// One capability producing BOTH, so a test replaces the whole act rather than reaching inside
/// either package for a nonce or a primitive. A generator that fails produces nothing at all.
pub trait KeysetGenerator {
    /// Generate both documents, or answer that the platform could not.
    fn generate(&mut self) -> Option<GeneratedKeysetV1>;
}

/// The production generator: platform bytes for the signing role, the package's own generator
/// for the encryption role.
///
/// The two halves come from independent draws and neither is derived from the other. The signing
/// secret passes through one stack buffer that is erased before this returns, whether or not the
/// platform answered — a buffer the platform declined to fill is a buffer of zeros, and a key
/// made from it would be the same key on every machine, so a refusal here produces no document.
#[derive(Debug)]
pub struct EntropyKeysetGenerator<E: KeySecretEntropy> {
    entropy: E,
}

impl<E: KeySecretEntropy> EntropyKeysetGenerator<E> {
    /// Generate from `entropy`.
    pub const fn over(entropy: E) -> Self {
        Self { entropy }
    }
}

impl<E: KeySecretEntropy> KeysetGenerator for EntropyKeysetGenerator<E> {
    fn generate(&mut self) -> Option<GeneratedKeysetV1> {
        let mut secret = [0_u8; 32];
        let filled = self.entropy.fill(&mut secret);
        let signing = filled.then(|| SigningPrivateDocument::of_secret_bytes(&secret));
        secret.zeroize();
        Some(GeneratedKeysetV1 {
            signing: signing?,
            encryption: EncryptionPrivateDocument::of_identity(x25519::Identity::generate()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_departure_from_one_line_is_named_separately() {
        // The refusals a real file produces, each pinned to its own arm rather than to "it did
        // not parse": a transport that rewrote the line endings and an operator keeping two
        // identities in one file are different things to tell somebody.
        for (bytes, expected) in [
            (b"AGE\r\n".as_slice(), LineDeparture::CarriageReturn),
            (b"AGE".as_slice(), LineDeparture::NoFinalNewline),
            (b"AGE\nAGE\n".as_slice(), LineDeparture::MoreThanOneLine),
            (b"\n".as_slice(), LineDeparture::Empty),
            (b"# a note\n".as_slice(), LineDeparture::Comment),
            (b" AGE\n".as_slice(), LineDeparture::SurroundingWhitespace),
            (b"AGE \n".as_slice(), LineDeparture::SurroundingWhitespace),
            (&[0xff, b'\n'], LineDeparture::NotText),
        ] {
            assert_eq!(sole_line(bytes), Err(expected), "{bytes:?}");
        }
        assert_eq!(sole_line(b"AGE-SOMETHING\n"), Ok("AGE-SOMETHING"));
    }

    #[test]
    fn a_generator_whose_platform_cannot_answer_produces_no_document() {
        // The sharp one. A zero buffer is a perfectly well-formed key, and a generator that
        // built one from a failed draw would mint the SAME keyset on every machine that failed
        // the same way — so the failure has to stop the whole act rather than weaken it.
        struct Refuses;
        impl KeySecretEntropy for Refuses {
            fn fill(&mut self, _: &mut [u8; 32]) -> bool {
                false
            }
        }
        assert!(EntropyKeysetGenerator::over(Refuses).generate().is_none());
    }
}
