//! Analyzer-model **erasure**: the vocabulary by which the model this crate analyzes
//! may SHRINK (`26H` §4, the human-ACKED overlay model).
//!
//! The three points the overlay model makes: the analyzer handles a MODEL of the code,
//! mapped back to the authoritative storage; that model may omit things permanently; and
//! a shrink must mint provenance in the global model saying what shrank and why. This
//! module owns the first two — the model side. The third (the proof, the ledger, the
//! round-tagged derivation links) lives in `dorc_plan::erase`, because the "why" is a
//! records-derived fact and this crate is records-blind by construction.
//!
//! What crosses the seam is an [`ErasedSites`] — an opaque set of node ids and nothing
//! else. This crate never learns why a site is in it, and no analysis consumer ever
//! consults it: [`crate::effect::classify_with_why_diags`] applies it ONCE, at the single
//! seam where the effect model is built, and then it is gone. That is what makes the
//! shrink uniform — an erased site is indistinguishable from one that never mutated, so
//! there is no flag for a present or future consumer to forget (the composition footgun
//! the mask-parameter design was rejected for).
//!
//! # The seal, and its one weak seam
//!
//! [`ErasedSites`] cannot be built from a bare set — only from [`ErasureLicense`]s,
//! consumed by value:
//!
//! ```compile_fail
//! use dorc_analysis::erase::ErasedSites;
//! use std::collections::BTreeSet;
//! // private field: an overlay cannot be forged from a bare set of ids
//! let _forged = ErasedSites(BTreeSet::new());
//! ```
//!
//! [`ErasureLicense::for_site`] is the weak seam, and it IS a seam rather than a
//! guarantee: this crate cannot depend on `plan` (the dependency runs the other way), so
//! the constructor must be public for `plan::erase` to call it, and the type system
//! therefore cannot prove that every licence traces to a proven-dead derivation. The
//! fence is lexical instead — `licence_mint_has_exactly_one_caller` in `dorc_plan`
//! asserts that `plan::erase` is the workspace's only caller. The STRONG seal (a ledger
//! entry demands a `DeadBranchProof`, which demands a `FoldResult` no crate outside
//! `plan` can populate) sits where the danger is, one layer up.

use std::collections::BTreeSet;

use crate::cfg::CfgNodeId;

/// Permission to erase ONE site's invalidator-hood from the analyzer model.
///
/// Minted solely by `dorc_plan::world::NoExecutionLedger::classify_overlay`, which exists only for a
/// ledger entry, which exists only for a records-proven-dead derivation. See the module
/// docs for why that chain is lexically rather than type-enforced at this hop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErasureLicense(CfgNodeId);

impl ErasureLicense {
    /// Mint a licence for `site`. THE weak seam — `dorc_plan::erase` is the only
    /// sanctioned caller, and a lexical census test in that crate fails if a second
    /// caller appears anywhere in the workspace.
    #[must_use]
    pub fn for_site(site: CfgNodeId) -> Self {
        Self(site)
    }

    /// The site this licence erases.
    #[must_use]
    pub fn site(self) -> CfgNodeId {
        self.0
    }
}

/// The residual model's shrink-set: sites whose invalidator-hood is absent from the
/// model this round. Applied once, at the effect-model seam; never consulted afterwards.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ErasedSites(BTreeSet<CfgNodeId>);

impl ErasedSites {
    /// The origin model — nothing erased. Round 1 always runs against this, and a
    /// no-oracle world never leaves it (`empty-world-byte-identical`).
    #[must_use]
    pub fn none() -> Self {
        Self(BTreeSet::new())
    }

    /// Build the overlay from ledger-minted licences, consumed by value: no entry ⇒ no
    /// licence ⇒ no member ⇒ no shrink.
    #[must_use]
    pub fn from_licenses(licenses: impl IntoIterator<Item = ErasureLicense>) -> Self {
        Self(licenses.into_iter().map(ErasureLicense::site).collect())
    }

    /// Is this site's invalidator-hood erased from the model?
    #[must_use]
    pub fn contains(&self, site: CfgNodeId) -> bool {
        self.0.contains(&site)
    }

    /// How many sites the overlay erases.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is the overlay empty (the origin model)?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The erased sites, in id order (`inv-determinism`).
    pub fn iter(&self) -> impl Iterator<Item = CfgNodeId> + '_ {
        self.0.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_overlay_carries_exactly_the_licensed_sites() {
        let overlay = ErasedSites::from_licenses([
            ErasureLicense::for_site(CfgNodeId(4)),
            ErasureLicense::for_site(CfgNodeId(1)),
        ]);
        assert!(overlay.contains(CfgNodeId(1)));
        assert!(overlay.contains(CfgNodeId(4)));
        assert!(!overlay.contains(CfgNodeId(2)));
        assert_eq!(overlay.len(), 2);
    }

    #[test]
    fn the_origin_overlay_erases_nothing() {
        let origin = ErasedSites::none();
        assert!(origin.is_empty());
        assert!(!origin.contains(CfgNodeId(0)));
    }

    #[test]
    fn overlay_iteration_is_id_ordered() {
        let overlay = ErasedSites::from_licenses([
            ErasureLicense::for_site(CfgNodeId(9)),
            ErasureLicense::for_site(CfgNodeId(2)),
            ErasureLicense::for_site(CfgNodeId(5)),
        ]);
        let seen: Vec<u32> = overlay.iter().map(|id| id.0).collect();
        assert_eq!(seen, vec![2, 5, 9]);
    }
}
