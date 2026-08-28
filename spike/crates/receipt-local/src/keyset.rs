//! What a local keyset can BE, as one closed set of states.
//!
//! # Why these are states and not an `Option`
//!
//! The distinctions here are the whole subject. "Not initialized" and "initialized and one member
//! is damaged" look alike to a caller holding `None`, and they demand opposite behaviour: the
//! first may generate, the second must never. Likewise a keyset that is mid-initialization
//! elsewhere is not a keyset that is missing, and a role that is unreadable right now is not a
//! role that is gone.
//!
//! Nothing here is `PermanentlyLost`, deliberately. A caller told a key is permanently gone will
//! discard the encrypted material it was the only way to read; a caller told the key is
//! unavailable will not.
//!
//! # What none of these can do
//!
//! No state in this module signs, seals, opens, publishes, initializes, or mints a dispatch
//! witness. They say what was FOUND. The capabilities live behind their own types, and a state is
//! what a caller reads to learn whether asking for one is even sensible.

use dorc_receipt_crypto::{
    EncryptionPrivateDocument, KeyDocumentBound, KeyDocumentRefusal, KeysetGenerator,
    SigningPrivateDocument, TrustedEd25519Key,
};

use crate::io::{self, GroupAndOtherAccess, IoFault, LocalIo, ObjectFacts, ObjectKind, OpenIntent};
use crate::limits::LocalLimits;
use crate::manifest::{KeyRole, KeysetManifest};
use crate::names::{
    ENCRYPTION_PRIVATE_FILE, KEY_DIR, KEYSET_DIR, KEYSET_MANIFEST_FILE, LocalPath,
    SIGNING_PRIVATE_FILE,
};
use crate::roots::{RootInputs, RootRole};
use crate::store::PlatformBaseline;

/// What a look at the local keyset found.
///
/// One arm per outcome, and no arm is a synonym for another — the failure sweep asserts that every
/// interruption of initialization lands in exactly one of them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAvailability {
    /// No keyset path exists at all. The one state under which first-use generation is even a
    /// candidate — and it is still gated on the store being absent or empty.
    NotInitialized,
    /// No keyset exists, and the V1 store is not provably absent-or-empty.
    ///
    /// Generation is FORBIDDEN here. Whole-keyset loss with receipts still on disk would otherwise
    /// become an unannounced new key era, and every one of those receipts would stop being
    /// readable without anyone being told.
    KeysetMissingWithExistingStore,
    /// A keyset directory exists without a valid final manifest.
    ///
    /// Never read as first use, even where both key files look valid: the manifest is the
    /// completion act, so its absence means nothing licensed this material for publication.
    IncompleteOrInProgress,
    /// The controller root could not be resolved to an absolute, validated location.
    RootUnavailable,
    /// The platform refused right now in a way that says nothing about the keyset's contents.
    TemporarilyUnavailable,
    /// A manifest names a role whose document is not there.
    MissingAfterInitialization {
        /// Which role.
        role: KeyRole,
    },
    /// A key document that did not parse.
    MalformedKeyDocument {
        /// Which role.
        role: KeyRole,
    },
    /// A key document that parsed and did not re-serialize to the bytes it came from.
    ///
    /// Separate from malformed on purpose: this is material a library accepted and would write
    /// differently, which is a V1 canonicality refusal rather than damage.
    NonCanonicalKeyDocument {
        /// Which role.
        role: KeyRole,
    },
    /// An object whose permissions or ownership are not what a private local keyset requires.
    PermissionRefused {
        /// What was refused.
        subject: PermissionSubject,
    },
    /// A role whose derived identity disagrees with the manifest's claim.
    ManifestMismatch {
        /// Which role.
        role: KeyRole,
    },
    /// A keyset naming a version this implementation does not know.
    UnsupportedKeysetVersion,
    /// An object where a keyset member belongs is not the kind of object it must be.
    ///
    /// Separate from every arm above: a file standing where the keyset directory belongs is not
    /// an incomplete keyset, and reading it as one would suggest waiting for a writer that does
    /// not exist.
    UnexpectedObject {
        /// Which member.
        subject: PermissionSubject,
    },
    /// The signing role is validated and usable for verification alone.
    VerificationReady,
    /// Both roles are validated for reading: verification, and opening a region.
    RichReadReady,
    /// Both roles are validated AND synchronized, so publication may proceed.
    ///
    /// The only state a write path may act on, and it is all-or-nothing: a half-ready keyset
    /// publishes nothing.
    ReadyForPublication,
}

/// What a permission refusal was about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionSubject {
    /// One of the two private key documents.
    KeyDocument {
        /// Which role.
        role: KeyRole,
    },
    /// The manifest.
    Manifest,
    /// A directory Dorc owns.
    Directory,
}

impl KeyAvailability {
    /// Whether this state licenses first-use generation.
    ///
    /// Exactly one arm does. Spelled as a method over an exhaustive match rather than as a
    /// comparison at each caller, so a new arm cannot be silently absorbed into "not ready, so
    /// presumably generate".
    #[must_use]
    pub const fn licenses_first_use_generation(&self) -> bool {
        match self {
            Self::NotInitialized => true,
            Self::KeysetMissingWithExistingStore
            | Self::IncompleteOrInProgress
            | Self::RootUnavailable
            | Self::TemporarilyUnavailable
            | Self::MissingAfterInitialization { .. }
            | Self::MalformedKeyDocument { .. }
            | Self::NonCanonicalKeyDocument { .. }
            | Self::PermissionRefused { .. }
            | Self::ManifestMismatch { .. }
            | Self::UnsupportedKeysetVersion
            | Self::UnexpectedObject { .. }
            | Self::VerificationReady
            | Self::RichReadReady
            | Self::ReadyForPublication => false,
        }
    }

    /// Whether this state exposes any capability that can write.
    #[must_use]
    pub const fn exposes_write_capability(&self) -> bool {
        matches!(self, Self::ReadyForPublication)
    }

    /// The closed word a report names this state by.
    ///
    /// Engine-owned vocabulary, not prose: one token per arm, exhaustive, so a new arm cannot be
    /// absorbed into a neighbour's word. Roles and subjects are deliberately NOT spelled into the
    /// token — which document is which is a detail a report renders from its own payload, and a
    /// token that varied by role would multiply the vocabulary without adding a world-state.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::NotInitialized => "not-initialized",
            Self::KeysetMissingWithExistingStore => "keyset-missing-with-existing-store",
            Self::IncompleteOrInProgress => "incomplete",
            Self::RootUnavailable => "root-unavailable",
            Self::TemporarilyUnavailable => "temporarily-unavailable",
            Self::MissingAfterInitialization { .. } => "key-missing",
            Self::MalformedKeyDocument { .. } => "key-malformed",
            Self::NonCanonicalKeyDocument { .. } => "key-non-canonical",
            Self::PermissionRefused { .. } => "permission-refused",
            Self::ManifestMismatch { .. } => "manifest-mismatch",
            Self::UnsupportedKeysetVersion => "unsupported-version",
            Self::UnexpectedObject { .. } => "unexpected-object",
            Self::VerificationReady => "verification-ready",
            Self::RichReadReady => "rich-read-ready",
            Self::ReadyForPublication => "ready-for-publication",
        }
    }
}

/// Where one keyset's objects live under a configuration root.
///
/// Built once from the roots and then addressed by value: every location below is the product
/// root plus fixed single components, so no spelling assembled anywhere else can reach one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeysetLocation {
    product_root: LocalPath,
    keys_dir: LocalPath,
    keyset_dir: LocalPath,
    signing: LocalPath,
    encryption: LocalPath,
    manifest: LocalPath,
}

impl KeysetLocation {
    /// The keyset's locations under `roots`, or nothing where the configuration base does not
    /// admit this project's fixed components.
    #[must_use]
    pub fn under(roots: &RootInputs) -> Option<Self> {
        let product_root = roots.product_root(RootRole::Configuration)?;
        let keys_dir = product_root.child(KEY_DIR)?;
        let keyset_dir = keys_dir.child(KEYSET_DIR)?;
        Some(Self {
            signing: keyset_dir.child(SIGNING_PRIVATE_FILE)?,
            encryption: keyset_dir.child(ENCRYPTION_PRIVATE_FILE)?,
            manifest: keyset_dir.child(KEYSET_MANIFEST_FILE)?,
            product_root,
            keys_dir,
            keyset_dir,
        })
    }

    /// The keyset directory, whose exclusive creation is the first-use arbitration point.
    #[must_use]
    pub const fn keyset_dir(&self) -> &LocalPath {
        &self.keyset_dir
    }

    /// The document for `role`.
    #[must_use]
    pub const fn document(&self, role: KeyRole) -> &LocalPath {
        match role {
            KeyRole::Signing => &self.signing,
            KeyRole::Encryption => &self.encryption,
        }
    }

    /// The manifest, written last.
    #[must_use]
    pub const fn manifest(&self) -> &LocalPath {
        &self.manifest
    }
}

/// Whether the V1 store is provably absent or empty.
///
/// Private field and one mint: first-use generation is gated on this answer, and an answer a
/// caller could assemble would let whole-keyset loss beside a full store become an unannounced
/// new key era. Everything short of a proof is the other arm — an unknown entry, an inaccessible
/// one, an enumeration that failed, and an overflow all say the same thing here, which is that
/// nothing established the store was empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePresence {
    provably_absent_or_empty: bool,
}

impl StorePresence {
    /// Look, read-only and bounded, at the V1 store under `roots`.
    pub fn probe(roots: &RootInputs, io: &mut dyn LocalIo, limits: &LocalLimits) -> Self {
        Self {
            provably_absent_or_empty: probe_store_is_absent_or_empty(roots, io, limits),
        }
    }

    /// Whether first-use generation is even a candidate.
    #[must_use]
    pub const fn provably_absent_or_empty(self) -> bool {
        self.provably_absent_or_empty
    }
}

fn probe_store_is_absent_or_empty(
    roots: &RootInputs,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
) -> bool {
    // The store's own derivation, so the gate and the store cannot disagree about which
    // directory the question was even about.
    let Some(store) = crate::store::store_root(roots) else {
        return false;
    };
    match io::open_existing_no_follow(io, store.as_str(), OpenIntent::Read) {
        Ok(()) => {}
        Err(IoFault::NotFound) => return true,
        Err(_) => return false,
    }
    match io::inspect_opened(io, store.as_str()) {
        Ok(facts) if facts.kind() == ObjectKind::Directory && !facts.redirected() => {}
        _ => return false,
    }
    match io::enumerate_bounded(io, store.as_str(), limits.store_entries) {
        Ok(entries) => entries.is_empty(),
        Err(_) => false,
    }
}

/// What a read-only look at the local keyset produced.
///
/// A separate type from the write open's answer, and that separation is structural rather than
/// stylistic: `dorc why` calls only the function returning this, and no arm of it carries or
/// converts into anything that can sign, seal, publish, or initialize.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "exactly one of these exists per invocation and it is moved once, so the lint's \n              cost model — many values in a collection — does not apply; boxing would put an \n              indirection in the API to save a copy nothing performs"
)]
pub enum LocalReadOpenV1 {
    /// Material validated for reading.
    Ready(LocalReadKeysV1),
    /// Nothing readable, and why.
    Unavailable(KeyAvailability),
}

/// What a write open produced.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "one per invocation, moved once; see the sibling above"
)]
pub enum LocalWriteOpenV1 {
    /// A validated, synchronized keyset that may publish.
    Ready(LocalWriteKeysV1),
    /// No publication, and why. Never a weaker capability.
    Refused(KeyAvailability),
}

/// The witness that a keyset's documents, manifest, and required ancestry were validated AND
/// successfully synchronized by this attempt.
///
/// Private, non-`Clone`, no `Default`, minted at one seat. It exists because a manifest can be
/// whole on disk while the directory entry that makes it reachable has never been synchronized —
/// the interruption that leaves a keyset LOOKING complete — so a write open re-synchronizes
/// rather than reading completeness off the manifest's presence.
#[derive(Debug)]
pub struct KeysetSynchronizedForPublicationV1(());

/// Material validated for reading, and nothing more.
///
/// Role-specific: a valid signing document with a matching manifest identity exposes trusted
/// verification even where the encryption document is missing or damaged, because a receipt whose
/// opaque half cannot be opened is still a receipt whose authorship can be checked.
#[derive(Debug)]
pub struct LocalReadKeysV1 {
    verifier: TrustedEd25519Key,
    opener: Option<EncryptionPrivateDocument>,
    status: KeyAvailability,
}

impl LocalReadKeysV1 {
    /// Verification material this controller's policy selected by validating this keyset.
    ///
    /// Trusted because policy chose the keyset, never because a document named its identity.
    #[must_use]
    pub const fn verifier(&self) -> &TrustedEd25519Key {
        &self.verifier
    }

    /// Material for opening a region, where the encryption role validated.
    #[must_use]
    pub const fn opener(&self) -> Option<&EncryptionPrivateDocument> {
        self.opener.as_ref()
    }

    /// Which of the two read states this is.
    #[must_use]
    pub const fn status(&self) -> &KeyAvailability {
        &self.status
    }
}

/// Material validated and synchronized for publication.
///
/// All-or-nothing: both roles, or nothing. A half-ready keyset publishes nothing, so there is no
/// arm here for one role being available.
#[derive(Debug)]
pub struct LocalWriteKeysV1 {
    signing: SigningPrivateDocument,
    encryption: EncryptionPrivateDocument,
    verifier: TrustedEd25519Key,
    readiness: KeysetSynchronizedForPublicationV1,
}

impl LocalWriteKeysV1 {
    /// The signing capability.
    #[must_use]
    pub const fn signer(&self) -> &SigningPrivateDocument {
        &self.signing
    }

    /// The encryption capability, which also opens.
    #[must_use]
    pub const fn encryption(&self) -> &EncryptionPrivateDocument {
        &self.encryption
    }

    /// Verification material for this keyset's own signing identity.
    #[must_use]
    pub const fn verifier(&self) -> &TrustedEd25519Key {
        &self.verifier
    }

    /// The synchronization witness this readiness rests on.
    #[must_use]
    pub const fn readiness(&self) -> &KeysetSynchronizedForPublicationV1 {
        &self.readiness
    }
}

/// Open the local keyset for reading. Mutation-free.
///
/// `dorc why` calls only this. Asking why must never create an identity that cannot open the
/// receipt being examined, so nothing on this path creates, writes, synchronizes, or removes.
pub fn open_for_read(
    roots: &RootInputs,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
) -> LocalReadOpenV1 {
    let Some(location) = KeysetLocation::under(roots) else {
        return LocalReadOpenV1::Unavailable(KeyAvailability::RootUnavailable);
    };
    let baseline = roots.platform().baseline();
    match load_validated(&location, io, limits, baseline) {
        Ok(loaded) => LocalReadOpenV1::Ready(loaded.into_read_keys()),
        Err(state) => LocalReadOpenV1::Unavailable(state),
    }
}

/// Open the local keyset for writing, initializing it on genuine first use.
///
/// Separate from [`open_for_read`] in every respect that matters: it is the only entry point that
/// may generate, it takes the generator and the store's standing, and its success type carries
/// capabilities the read answer has no arm for.
pub fn open_or_initialize_for_write(
    roots: &RootInputs,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
    store: StorePresence,
    generator: &mut dyn KeysetGenerator,
) -> LocalWriteOpenV1 {
    let Some(location) = KeysetLocation::under(roots) else {
        return LocalWriteOpenV1::Refused(KeyAvailability::RootUnavailable);
    };
    let baseline = roots.platform().baseline();

    match io::open_existing_no_follow(io, location.keyset_dir.as_str(), OpenIntent::Read) {
        Ok(()) => return reopen_for_write(&location, io, limits, baseline),
        Err(IoFault::NotFound) => {}
        Err(IoFault::Redirect) => {
            return LocalWriteOpenV1::Refused(KeyAvailability::PermissionRefused {
                subject: PermissionSubject::Directory,
            });
        }
        Err(_) => return LocalWriteOpenV1::Refused(KeyAvailability::TemporarilyUnavailable),
    }

    if !store.provably_absent_or_empty() {
        return LocalWriteOpenV1::Refused(KeyAvailability::KeysetMissingWithExistingStore);
    }
    match initialize(&location, io, limits, baseline, generator) {
        Ok(()) => reopen_for_write(&location, io, limits, baseline),
        Err(state) => LocalWriteOpenV1::Refused(state),
    }
}

/// The exclusive initialization sequence, in `30Rd`'s fixed order.
fn initialize(
    location: &KeysetLocation,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
    baseline: PlatformBaseline,
    generator: &mut dyn KeysetGenerator,
) -> Result<(), KeyAvailability> {
    bootstrap_directory(io, &location.product_root, baseline)?;

    // Generation precedes the keyset path: no path is created until both documents are in hand.
    let Some(generated) = generator.generate() else {
        return Err(KeyAvailability::TemporarilyUnavailable);
    };

    bootstrap_directory(io, &location.keys_dir, baseline)?;

    // The arbitration point: exactly one process creates this, and a loser discards what it
    // generated.
    match io::create_directory_exclusive(io, location.keyset_dir.as_str()) {
        Ok(()) => {}
        Err(IoFault::AlreadyExists) => return Err(KeyAvailability::IncompleteOrInProgress),
        Err(IoFault::Denied) => {
            return Err(KeyAvailability::PermissionRefused {
                subject: PermissionSubject::Directory,
            });
        }
        Err(_) => return Err(KeyAvailability::TemporarilyUnavailable),
    }

    let (signing, encryption) = generated.into_parts();
    let signing_bytes = signing
        .with_canonical_bytes(<[u8]>::to_vec)
        .ok_or(KeyAvailability::TemporarilyUnavailable)?;
    place(io, location.document(KeyRole::Signing), &signing_bytes)?;
    let encryption_bytes = encryption.with_canonical_bytes(<[u8]>::to_vec);
    place(
        io,
        location.document(KeyRole::Encryption),
        &encryption_bytes,
    )?;

    let manifest = KeysetManifest::of(
        &signing.signing_key_id().hex(),
        &encryption.encryption_key_id().hex(),
    )
    .map_err(|_| KeyAvailability::TemporarilyUnavailable)?;
    let manifest_bytes = manifest.serialize().into_bytes();
    if manifest_bytes.len() > limits.manifest_bytes {
        return Err(KeyAvailability::TemporarilyUnavailable);
    }
    // LAST. The manifest's presence is what makes a keyset complete, so nothing may be written
    // after it: a keyset whose manifest exists has both documents beside it, always.
    place(io, location.manifest(), &manifest_bytes)?;

    synchronize_ancestry(io, location)
}

/// Create one directory this project owns, or validate the one already there.
fn bootstrap_directory(
    io: &mut dyn LocalIo,
    path: &LocalPath,
    baseline: PlatformBaseline,
) -> Result<(), KeyAvailability> {
    match io::create_directory_exclusive(io, path.as_str()) {
        // Not synchronized here. Directory synchronization has ONE seat, at the end of the
        // sequence, where its failure fails the attempt.
        Ok(()) => Ok(()),
        // A race is answered by validating the winner, never by assuming it is what was asked
        // for. Every Dorc-owned component is refused if it is a redirect or the wrong kind.
        Err(IoFault::AlreadyExists) => {
            let facts = open_and_inspect(io, path, PermissionSubject::Directory)?;
            require_directory(facts, baseline)
        }
        Err(IoFault::Denied) => Err(KeyAvailability::PermissionRefused {
            subject: PermissionSubject::Directory,
        }),
        Err(_) => Err(KeyAvailability::TemporarilyUnavailable),
    }
}

/// Create, write, and synchronize one file, in that order and with no step folded into another.
fn place(io: &mut dyn LocalIo, path: &LocalPath, bytes: &[u8]) -> Result<(), KeyAvailability> {
    io::create_file_exclusive(io, path.as_str()).map_err(create_failure)?;
    io::write_all(io, path.as_str(), bytes).map_err(|_| KeyAvailability::TemporarilyUnavailable)?;
    // Never retried: a second call can report success over pages the kernel already discarded.
    io::sync_file(io, path.as_str()).map_err(|_| KeyAvailability::TemporarilyUnavailable)
}

const fn create_failure(fault: IoFault) -> KeyAvailability {
    match fault {
        IoFault::Denied => KeyAvailability::PermissionRefused {
            subject: PermissionSubject::Directory,
        },
        _ => KeyAvailability::TemporarilyUnavailable,
    }
}

/// Synchronize the keyset directory and the ancestry that makes it reachable.
fn synchronize_ancestry(
    io: &mut dyn LocalIo,
    location: &KeysetLocation,
) -> Result<(), KeyAvailability> {
    for directory in [
        &location.keyset_dir,
        &location.keys_dir,
        &location.product_root,
    ] {
        io::sync_directory(io, directory.as_str())
            .map_err(|_| KeyAvailability::TemporarilyUnavailable)?;
    }
    Ok(())
}

/// Everything a validated keyset holds, before it is narrowed to a read or write answer.
struct LoadedKeyset {
    signing: Option<SigningPrivateDocument>,
    encryption: Option<EncryptionPrivateDocument>,
    verifier: TrustedEd25519Key,
}

impl LoadedKeyset {
    fn into_read_keys(self) -> LocalReadKeysV1 {
        let status = if self.encryption.is_some() {
            KeyAvailability::RichReadReady
        } else {
            KeyAvailability::VerificationReady
        };
        LocalReadKeysV1 {
            verifier: self.verifier,
            opener: self.encryption,
            status,
        }
    }
}

/// The ordinary open path: validate every member before any capability exists.
fn load_validated(
    location: &KeysetLocation,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
    baseline: PlatformBaseline,
) -> Result<LoadedKeyset, KeyAvailability> {
    let directory = open_and_inspect(io, &location.keyset_dir, PermissionSubject::Directory)
        .map_err(|state| match state {
            KeyAvailability::MissingAfterInitialization { .. } => KeyAvailability::NotInitialized,
            other => other,
        })?;
    require_directory(directory, baseline)?;

    // The manifest is the completion act, so its absence means nothing here was ever licensed
    // for publication — never first use, even where both documents look valid.
    let manifest_bytes = read_member(
        io,
        location.manifest(),
        limits.manifest_bytes,
        PermissionSubject::Manifest,
        baseline,
    )
    .map_err(|state| match state {
        KeyAvailability::MissingAfterInitialization { .. } => {
            KeyAvailability::IncompleteOrInProgress
        }
        other => other,
    })?;
    let manifest = KeysetManifest::parse(&manifest_bytes, limits).map_err(|_| {
        // A manifest this reader cannot read is a keyset era it does not implement. It is never
        // repaired and never treated as absent.
        KeyAvailability::UnsupportedKeysetVersion
    })?;

    let signing_bytes = read_member(
        io,
        location.document(KeyRole::Signing),
        limits.signing_document_bytes,
        PermissionSubject::KeyDocument {
            role: KeyRole::Signing,
        },
        baseline,
    )?;
    let signing = SigningPrivateDocument::parse(
        &signing_bytes,
        KeyDocumentBound::of(limits.signing_document_bytes),
    )
    .map_err(|refusal| document_refusal(refusal, KeyRole::Signing))?;
    if signing.signing_key_id().hex() != manifest.claimed(KeyRole::Signing) {
        return Err(KeyAvailability::ManifestMismatch {
            role: KeyRole::Signing,
        });
    }
    let verifier = signing.verifier().map(TrustedEd25519Key::of).ok_or(
        KeyAvailability::MalformedKeyDocument {
            role: KeyRole::Signing,
        },
    )?;

    // The encryption role is allowed to be unavailable without taking the signing role with it:
    // a receipt whose opaque half cannot be opened is still one whose authorship can be checked.
    let encryption = load_encryption(location, io, limits, baseline, &manifest);

    Ok(LoadedKeyset {
        signing: Some(signing),
        encryption,
        verifier,
    })
}

fn load_encryption(
    location: &KeysetLocation,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
    baseline: PlatformBaseline,
    manifest: &KeysetManifest,
) -> Option<EncryptionPrivateDocument> {
    let bytes = read_member(
        io,
        location.document(KeyRole::Encryption),
        limits.encryption_document_bytes,
        PermissionSubject::KeyDocument {
            role: KeyRole::Encryption,
        },
        baseline,
    )
    .ok()?;
    let document = EncryptionPrivateDocument::parse(
        &bytes,
        KeyDocumentBound::of(limits.encryption_document_bytes),
    )
    .ok()?;
    (document.encryption_key_id().hex() == manifest.claimed(KeyRole::Encryption))
        .then_some(document)
}

/// A write open over an existing keyset: validate, require both roles, and re-synchronize.
fn reopen_for_write(
    location: &KeysetLocation,
    io: &mut dyn LocalIo,
    limits: &LocalLimits,
    baseline: PlatformBaseline,
) -> LocalWriteOpenV1 {
    let loaded = match load_validated(location, io, limits, baseline) {
        Ok(loaded) => loaded,
        Err(state) => return LocalWriteOpenV1::Refused(state),
    };
    let (Some(signing), Some(encryption)) = (loaded.signing, loaded.encryption) else {
        return LocalWriteOpenV1::Refused(KeyAvailability::MissingAfterInitialization {
            role: KeyRole::Encryption,
        });
    };

    // A write open re-synchronizes the documents, the manifest, and the ancestry, and only a
    // synchronization that SUCCEEDS mints readiness. A whole manifest is not the question here.
    for path in [
        location.document(KeyRole::Signing),
        location.document(KeyRole::Encryption),
        location.manifest(),
    ] {
        // Re-opened declaring what this handle must be able to do. Validation above read these
        // through handles that cannot flush, which is deliberate: the read entry point uses the
        // same helper and must never acquire one that could.
        if io::open_existing_no_follow(io, path.as_str(), OpenIntent::ReadAndSynchronize).is_err()
            || io::sync_file(io, path.as_str()).is_err()
        {
            return LocalWriteOpenV1::Refused(KeyAvailability::TemporarilyUnavailable);
        }
    }
    if let Err(state) = synchronize_ancestry(io, location) {
        return LocalWriteOpenV1::Refused(state);
    }

    LocalWriteOpenV1::Ready(LocalWriteKeysV1 {
        signing,
        encryption,
        verifier: loaded.verifier,
        readiness: KeysetSynchronizedForPublicationV1(()),
    })
}

/// Open one member without following a final-component redirect, then inspect the opened object
/// BEFORE anything is read from it.
fn open_and_inspect(
    io: &mut dyn LocalIo,
    path: &LocalPath,
    subject: PermissionSubject,
) -> Result<ObjectFacts, KeyAvailability> {
    match io::open_existing_no_follow(io, path.as_str(), OpenIntent::Read) {
        Ok(()) => {}
        Err(IoFault::NotFound) => {
            return Err(missing(subject));
        }
        Err(IoFault::Redirect | IoFault::Denied) => {
            return Err(KeyAvailability::PermissionRefused { subject });
        }
        Err(_) => return Err(KeyAvailability::TemporarilyUnavailable),
    }
    io::inspect_opened(io, path.as_str()).map_err(|fault| match fault {
        IoFault::Denied => KeyAvailability::PermissionRefused { subject },
        _ => KeyAvailability::TemporarilyUnavailable,
    })
}

const fn missing(subject: PermissionSubject) -> KeyAvailability {
    match subject {
        PermissionSubject::KeyDocument { role } => {
            KeyAvailability::MissingAfterInitialization { role }
        }
        PermissionSubject::Manifest | PermissionSubject::Directory => {
            KeyAvailability::MissingAfterInitialization {
                role: KeyRole::Signing,
            }
        }
    }
}

/// Read one member under its own bound, after its opened handle has been inspected.
fn read_member(
    io: &mut dyn LocalIo,
    path: &LocalPath,
    bound: usize,
    subject: PermissionSubject,
    baseline: PlatformBaseline,
) -> Result<Vec<u8>, KeyAvailability> {
    let facts = open_and_inspect(io, path, subject)?;
    if facts.kind() != ObjectKind::RegularFile {
        return Err(KeyAvailability::UnexpectedObject { subject });
    }
    require_private(facts, baseline, subject)?;
    io::read_bounded(io, path.as_str(), bound).map_err(|fault| match fault {
        IoFault::OverBound => malformed(subject),
        IoFault::Denied => KeyAvailability::PermissionRefused { subject },
        _ => KeyAvailability::TemporarilyUnavailable,
    })
}

const fn malformed(subject: PermissionSubject) -> KeyAvailability {
    match subject {
        PermissionSubject::KeyDocument { role } => KeyAvailability::MalformedKeyDocument { role },
        PermissionSubject::Manifest | PermissionSubject::Directory => {
            KeyAvailability::UnsupportedKeysetVersion
        }
    }
}

const fn document_refusal(refusal: KeyDocumentRefusal, role: KeyRole) -> KeyAvailability {
    match refusal {
        KeyDocumentRefusal::NonCanonical => KeyAvailability::NonCanonicalKeyDocument { role },
        KeyDocumentRefusal::OverBound
        | KeyDocumentRefusal::Malformed
        | KeyDocumentRefusal::NotOneCanonicalLine { .. } => {
            KeyAvailability::MalformedKeyDocument { role }
        }
    }
}

/// A Dorc-owned directory must be a directory, not a redirect, and private where the platform
/// answers the question.
fn require_directory(
    facts: ObjectFacts,
    baseline: PlatformBaseline,
) -> Result<(), KeyAvailability> {
    if facts.kind() != ObjectKind::Directory {
        return Err(KeyAvailability::UnexpectedObject {
            subject: PermissionSubject::Directory,
        });
    }
    require_private(facts, baseline, PermissionSubject::Directory)
}

/// The platform's honest posture, applied.
///
/// Unix answers TWO things and both are required: group and other access must be none, and the
/// object must belong to whoever this process is. Windows answers neither comparably, and the
/// baseline there rests on the per-user profile's inherited access plus the refusal of redirects
/// — explicitly weaker, and never rendered as equivalent.
///
/// The owner half is the narrower of the two. On a mode-enforcing filesystem `0700` plus this
/// process's ability to read the object is already transitive proof of ownership for a non-root
/// process; what the comparison closes is the case where the process holds DAC-override, and
/// only the owner answer distinguishes that from an object of its own.
fn require_private(
    facts: ObjectFacts,
    baseline: PlatformBaseline,
    subject: PermissionSubject,
) -> Result<(), KeyAvailability> {
    if facts.redirected() {
        return Err(KeyAvailability::PermissionRefused { subject });
    }
    match (baseline, facts.group_and_other()) {
        (PlatformBaseline::UnixLike, GroupAndOtherAccess::None) => {
            if facts.ownership_established() {
                Ok(())
            } else {
                Err(KeyAvailability::PermissionRefused { subject })
            }
        }
        (PlatformBaseline::Windows, GroupAndOtherAccess::NotInspectable) => Ok(()),
        _ => Err(KeyAvailability::PermissionRefused { subject }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every state, so the two predicates below are asked of all of them rather than of the ones
    /// a writer happened to remember.
    fn every_state() -> Vec<KeyAvailability> {
        let mut out = vec![
            KeyAvailability::NotInitialized,
            KeyAvailability::KeysetMissingWithExistingStore,
            KeyAvailability::IncompleteOrInProgress,
            KeyAvailability::RootUnavailable,
            KeyAvailability::TemporarilyUnavailable,
            KeyAvailability::UnsupportedKeysetVersion,
            KeyAvailability::VerificationReady,
            KeyAvailability::RichReadReady,
            KeyAvailability::ReadyForPublication,
            KeyAvailability::PermissionRefused {
                subject: PermissionSubject::Manifest,
            },
            KeyAvailability::PermissionRefused {
                subject: PermissionSubject::Directory,
            },
            KeyAvailability::UnexpectedObject {
                subject: PermissionSubject::Manifest,
            },
            KeyAvailability::UnexpectedObject {
                subject: PermissionSubject::Directory,
            },
        ];
        for role in KeyRole::ALL {
            out.push(KeyAvailability::MissingAfterInitialization { role });
            out.push(KeyAvailability::MalformedKeyDocument { role });
            out.push(KeyAvailability::NonCanonicalKeyDocument { role });
            out.push(KeyAvailability::ManifestMismatch { role });
            out.push(KeyAvailability::PermissionRefused {
                subject: PermissionSubject::KeyDocument { role },
            });
            out.push(KeyAvailability::UnexpectedObject {
                subject: PermissionSubject::KeyDocument { role },
            });
        }
        out
    }

    #[test]
    fn exactly_one_state_licenses_generation() {
        // The sharp one. Every other arm is a reason NOT to generate, and several of them are
        // reasons a hurried caller would read as "nothing usable is here, so make one" — which is
        // exactly how a damaged keyset becomes a silent new key era.
        let licensing: Vec<KeyAvailability> = every_state()
            .into_iter()
            .filter(KeyAvailability::licenses_first_use_generation)
            .collect();
        assert_eq!(licensing.len(), 1, "{licensing:?}");
        assert_eq!(licensing.first(), Some(&KeyAvailability::NotInitialized));
    }

    #[test]
    fn no_state_short_of_publication_readiness_exposes_a_write_capability() {
        for state in every_state() {
            let writes = state.exposes_write_capability();
            assert_eq!(
                writes,
                state == KeyAvailability::ReadyForPublication,
                "{state:?} answered {writes}"
            );
        }
    }

    #[test]
    fn a_missing_keyset_beside_a_store_is_not_a_missing_keyset() {
        // The two look alike from a caller holding an `Option`, and only one of them may
        // generate. Pinned as inequality because the distinction is the whole reason both exist.
        assert_ne!(
            KeyAvailability::NotInitialized,
            KeyAvailability::KeysetMissingWithExistingStore
        );
        assert!(
            !KeyAvailability::KeysetMissingWithExistingStore.licenses_first_use_generation(),
            "a store on disk forbids a new key era"
        );
    }
}
