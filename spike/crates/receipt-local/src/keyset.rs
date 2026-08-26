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

use crate::manifest::KeyRole;

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
        ];
        for role in KeyRole::ALL {
            out.push(KeyAvailability::MissingAfterInitialization { role });
            out.push(KeyAvailability::MalformedKeyDocument { role });
            out.push(KeyAvailability::NonCanonicalKeyDocument { role });
            out.push(KeyAvailability::ManifestMismatch { role });
            out.push(KeyAvailability::PermissionRefused {
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
