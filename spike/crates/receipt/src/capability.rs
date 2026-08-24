//! The injected capabilities, as traits.
//!
//! Every one of these is one purpose wide. None returns a checked, trusted, or complete
//! receipt state: a signer returns bytes, a verifier returns a plain yes or no, an opener
//! returns bounded plaintext. The states those results feed are minted in this crate and
//! nowhere else, so an implementation cannot promote its own output.
//!
//! The implementations live in `dorc-receipt-crypto`, which depends on this crate. Nothing
//! here depends on that one.

use crate::ids::{EncryptionKeyId, SigningKeyId};

/// Produce a signature over exact bytes.
pub trait ReceiptSigner {
    /// Which signing provider this is, for the header line.
    fn signing_key_id(&self) -> SigningKeyId;

    /// Sign `body` exactly as given. The caller has already applied the envelope; an
    /// implementation must not re-encode, normalize, or re-read anything.
    fn sign(&self, body: &[u8]) -> [u8; 64];
}

/// Check a signature over exact bytes.
///
/// The answer is a bare boolean on purpose: the state that says a document was checked is
/// minted here, from this answer plus material the resolver already marked, so an
/// implementation has nothing to promote.
pub trait ReceiptVerifier {
    /// Whether `signature` is valid over `body` under this provider's material.
    fn verify(&self, body: &[u8], signature: &[u8; 64]) -> bool;
}

/// Verification material whose provider controller policy names.
pub trait TrustedReceiptVerificationKey: ReceiptVerifier {
    /// Which signing provider this is.
    fn signing_key_id(&self) -> SigningKeyId;
}

/// Verification material whose provider controller policy does not name.
pub trait SelfAssertedReceiptVerificationKey: ReceiptVerifier {
    /// Which signing provider this is.
    fn signing_key_id(&self) -> SigningKeyId;
}

/// Turn a provider identity into verification material.
///
/// Two concrete answers rather than one generic one: a caller cannot ask for the provenance
/// it would prefer, because there is no type parameter to ask with.
pub trait VerificationKeyResolver {
    /// Material this controller's policy names.
    fn trusted(&self, id: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey>;

    /// Material this controller's policy does not name.
    fn self_asserted(&self, id: SigningKeyId) -> Option<&dyn SelfAssertedReceiptVerificationKey>;
}

/// Seal one overlay plaintext into one armored region.
pub trait OverlaySealer {
    /// Which encryption provider this is, for the header line.
    fn encryption_key_id(&self) -> EncryptionKeyId;

    /// Seal `plaintext`, answering the exact armored bytes.
    fn seal(&self, plaintext: &[u8]) -> Option<String>;
}

/// Open one armored region into bounded plaintext.
///
/// The plaintext is inert on return: it has not been validated against the skeleton, and no
/// opaque value it carries may be read until it has.
pub trait OverlayOpener {
    /// Open `armor`, answering at most `max_bytes` of plaintext.
    fn open(&self, armor: &str, max_bytes: u64) -> Option<Vec<u8>>;
}

/// Where a published document goes.
pub trait ReceiptSink {
    /// Publish `bytes` under `name`. Answering `None` is a publication failure.
    fn publish(&mut self, name: &str, bytes: &[u8]) -> Option<PublicationGrade>;
}

/// Where documents are read back from.
pub trait ReceiptSource {
    /// Every published name, bounded by the caller's own enumeration policy.
    fn names(&self) -> Vec<String>;

    /// The exact bytes published under `name`.
    fn read(&self, name: &str) -> Option<Vec<u8>>;
}

/// How durably a sink reports it placed a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PublicationGrade {
    /// The sink holds it in memory only.
    Volatile,
    /// The sink wrote it without confirming the write reached storage.
    Written,
    /// The sink wrote it and confirmed the write reached storage.
    Synchronized,
}

impl PublicationGrade {
    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Volatile => "volatile",
            Self::Written => "written",
            Self::Synchronized => "synchronized",
        }
    }
}
