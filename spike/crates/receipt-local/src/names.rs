//! The V1 persistent names: the exact directory and file spellings, and the one parser that
//! reads a receipt filename back.
//!
//! # Why every name carries `v1`
//!
//! The format inside a file already says `/1`, so the version in the NAME is redundant on
//! purpose: an operator can tell which era a directory belongs to without opening anything, and a
//! future implementation never has to decide what an unversioned name meant.
//!
//! # What a filename is, and is not
//!
//! A filename is a SELECTION HINT. Everything it spells — species, order, receipt identity —
//! also sits inside the signed body, and the authenticated values are the ones that count. Reading
//! a name back is how a store narrows a directory listing before opening anything; it never
//! settles what a document says, and a disagreement between a name and a verified header is a
//! finding rather than a tie the name wins.

use dorc_receipt::order::{ORDER_DIGITS, ReceiptOrderToken};

use crate::roots::RootPlatform;

/// An absolute location this crate is allowed to address.
///
/// A validated root, plus fixed single-component names beneath it. The type is what enforces
/// `30Rd`'s "only fixed, typed, single-component internal names": a component carrying a
/// separator, a drive letter, or a parent reference is refused rather than joined, so no string a
/// caller assembles can reach outside the landing it was rooted at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPath {
    text: String,
    separator: char,
}

impl LocalPath {
    /// Root at `root`, spelled the way `platform` spells a path.
    #[must_use]
    pub fn of_root(platform: RootPlatform, root: &str) -> Self {
        Self {
            text: root.to_owned(),
            separator: match platform {
                RootPlatform::Windows => '\\',
                RootPlatform::MacOs | RootPlatform::OtherUnix => '/',
            },
        }
    }

    /// The location of `component` directly beneath this one.
    ///
    /// Answers nothing for a component that is not one single ordinary name — which is every
    /// spelling that could leave the subtree.
    #[must_use]
    pub fn child(&self, component: &str) -> Option<Self> {
        let ordinary = !component.is_empty()
            && component != "."
            && component != ".."
            && !component
                .chars()
                .any(|c| matches!(c, '/' | '\\' | ':') || c.is_control());
        if !ordinary {
            return None;
        }
        Some(Self {
            text: format!("{}{}{component}", self.text, self.separator),
            separator: self.separator,
        })
    }

    /// The spelling an I/O implementation is handed.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

/// This project's one component beneath a platform base. Everything else is versioned; this is
/// not, because it names the product rather than an era.
pub const PRODUCT_DIR: &str = "dorc";

/// The versioned directory holding the local keyset, under the configuration root.
pub const KEY_DIR: &str = "receipt-keys-v1";

/// The versioned directory holding one keyset, under [`KEY_DIR`].
///
/// Its exclusive creation is the first-use arbitration point, which is why it is a component of
/// its own rather than the key files sitting directly under [`KEY_DIR`].
pub const KEYSET_DIR: &str = "keyset-v1";

/// The signing private document.
pub const SIGNING_PRIVATE_FILE: &str = "signing-private-v1.pk8";

/// The encryption private document.
pub const ENCRYPTION_PRIVATE_FILE: &str = "encryption-private-v1.age";

/// The keyset manifest, written last, whose presence is what makes a keyset complete.
pub const KEYSET_MANIFEST_FILE: &str = "keyset-manifest-v1.txt";

/// The versioned directory holding published receipts, under the state root.
pub const STORE_DIR: &str = "receipts-v1";

/// The extension every published receipt carries.
pub const RECEIPT_EXTENSION: &str = ".dorc-receipt";

/// Which species a filename names. Its own type rather than a borrowed one: the wire species token
/// and the filename stem are separate vocabularies that happen to agree today, and a shared type
/// would silently couple a rename of one to the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamedSpecies {
    /// A plan receipt.
    Plan,
    /// An apply intent.
    ApplyIntent,
    /// An apply outcome.
    ApplyOutcome,
}

impl NamedSpecies {
    /// Every species, in one order.
    pub const ALL: [Self; 3] = [Self::Plan, Self::ApplyIntent, Self::ApplyOutcome];

    /// The versioned stem this species' filenames open with.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        match self {
            Self::Plan => "plan-v1",
            Self::ApplyIntent => "apply-intent-v1",
            Self::ApplyOutcome => "apply-outcome-v1",
        }
    }

    /// The species a stem names, or nothing.
    #[must_use]
    pub fn of_stem(stem: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|species| species.stem() == stem)
    }
}

/// One published receipt's filename, parsed.
///
/// Private fields: the only way to hold one is to have minted it from typed values or to have
/// read a name that was exactly this shape. Nothing here is authority — see the module header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptFileName {
    species: NamedSpecies,
    order: ReceiptOrderToken,
    receipt_id: String,
}

/// Why a directory entry is not a receipt name.
///
/// Closed, and each arm is a different fact about the entry. Never one catch-all arm: an entry
/// malformed under a recognized prefix and an entry that is simply something else are different
/// findings, and only the first says anything about this store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameRefusal {
    /// The entry does not carry the receipt extension.
    NotAReceiptExtension,
    /// The entry names no species this version knows.
    UnknownSpecies,
    /// The entry has a recognized species stem and does not have the rest of the shape.
    MalformedUnderKnownSpecies,
    /// The order component was not exactly the fixed-width spelling.
    OrderNotExactWidth,
    /// The identity component was not exactly sixty-four lowercase hexadecimal characters.
    IdentityNotExactDigest,
    /// The entry is longer than a V1 name may be.
    OverNameBound,
}

impl ReceiptFileName {
    /// Mint the exact name for one published document.
    #[must_use]
    pub fn of(species: NamedSpecies, order: ReceiptOrderToken, receipt_id: &str) -> Option<Self> {
        if !is_lower_hex_64(receipt_id) {
            return None;
        }
        Some(Self {
            species,
            order,
            receipt_id: receipt_id.to_owned(),
        })
    }

    /// The one spelling.
    #[must_use]
    pub fn spelled(&self) -> String {
        format!(
            "{}-{}-{}{RECEIPT_EXTENSION}",
            self.species.stem(),
            self.order.spelled(),
            self.receipt_id
        )
    }

    /// The species this name claims.
    #[must_use]
    pub const fn species(&self) -> NamedSpecies {
        self.species
    }

    /// The order this name claims.
    #[must_use]
    pub const fn order(&self) -> ReceiptOrderToken {
        self.order
    }

    /// The receipt identity this name claims, as spelled.
    #[must_use]
    pub fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    /// Read a directory entry back as a receipt name.
    ///
    /// Strict in one direction only: it recognizes exactly what [`Self::spelled`] writes, and
    /// every departure is a named refusal rather than a normalization. A store that repaired a
    /// name would be inventing a document identity from a filesystem entry.
    ///
    /// # Errors
    /// Refuses an over-long entry, a missing extension, an unknown species, and each component
    /// that is not exactly its own shape.
    pub fn of_entry(entry: &str, limits: &crate::limits::LocalLimits) -> Result<Self, NameRefusal> {
        if entry.len() > limits.name_bytes {
            return Err(NameRefusal::OverNameBound);
        }
        let stem = entry
            .strip_suffix(RECEIPT_EXTENSION)
            .ok_or(NameRefusal::NotAReceiptExtension)?;
        let species = NamedSpecies::ALL
            .into_iter()
            .find(|candidate| {
                stem.strip_prefix(candidate.stem())
                    .is_some_and(|rest| rest.starts_with('-'))
            })
            .ok_or(NameRefusal::UnknownSpecies)?;
        let rest = stem
            .strip_prefix(species.stem())
            .and_then(|rest| rest.strip_prefix('-'))
            .ok_or(NameRefusal::MalformedUnderKnownSpecies)?;
        let (order, receipt_id) = rest
            .split_once('-')
            .ok_or(NameRefusal::MalformedUnderKnownSpecies)?;
        if order.len() != ORDER_DIGITS {
            return Err(NameRefusal::OrderNotExactWidth);
        }
        let order = ReceiptOrderToken::of_spelling(order).ok_or(NameRefusal::OrderNotExactWidth)?;
        if !is_lower_hex_64(receipt_id) {
            return Err(NameRefusal::IdentityNotExactDigest);
        }
        Ok(Self {
            species,
            order,
            receipt_id: receipt_id.to_owned(),
        })
    }
}

/// Exactly sixty-four lowercase hexadecimal characters — the one identity spelling.
///
/// Lowercase only, deliberately: the alphabet has to be invariant under case folding and under
/// Unicode normalization for a name to mean the same thing on every filesystem, and admitting
/// uppercase would let one identity spell two entries that some filesystems call one.
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
    fn a_species_round_trips_through_its_stem_and_the_stems_are_distinct() {
        let mut stems: Vec<&str> = NamedSpecies::ALL.iter().map(|s| s.stem()).collect();
        for species in NamedSpecies::ALL {
            assert_eq!(NamedSpecies::of_stem(species.stem()), Some(species));
        }
        let before = stems.len();
        stems.sort_unstable();
        stems.dedup();
        assert_eq!(before, stems.len(), "two species share a stem");
        assert_eq!(
            NamedSpecies::of_stem("plan"),
            None,
            "the version is part of it"
        );
    }

    #[test]
    fn no_species_stem_is_a_prefix_of_another() {
        // The parser splits on the stem, so a stem that prefixed another would let the shorter one
        // claim the longer one's names and answer with the wrong species.
        for outer in NamedSpecies::ALL {
            for inner in NamedSpecies::ALL {
                if outer != inner {
                    assert!(
                        !outer.stem().starts_with(inner.stem()),
                        "{} prefixes {}",
                        inner.stem(),
                        outer.stem()
                    );
                }
            }
        }
    }
}
