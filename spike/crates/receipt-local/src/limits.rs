//! The local edge's own bounds.
//!
//! Injected policy, never timeless truth: every one of these is a V1 number chosen to keep a path
//! bounded, and the values are separate from the receipt crate's because they bound different
//! things — a key document and a directory walk are not receipts.
//!
//! One receipt stays bounded by `dorc_receipt::limits::ReceiptLimits`; nothing here widens it.

/// The V1 local-edge bounds, as one value every local operation is handed.
///
/// Public fields on a plain policy struct, exactly like the receipt crate's limits: this carries
/// no authority and lowering a bound is a legitimate local act. Widening one is not — it needs
/// boundary-minus/at/plus cases and an allocation argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalLimits {
    /// The keyset manifest, whole.
    pub manifest_bytes: usize,
    /// The signing private document, whole.
    pub signing_document_bytes: usize,
    /// The encryption private identity document, whole.
    pub encryption_document_bytes: usize,
    /// One persistent filename.
    pub name_bytes: usize,
    /// How many entries one store enumeration may collect.
    ///
    /// The walk goes to this PLUS ONE before classifying anything, so overflow is a fact the walk
    /// observed rather than a silence at the boundary.
    pub store_entries: usize,
    /// How many bytes of receipts one graph build may admit in total.
    pub graph_bytes: u64,
}

impl LocalLimits {
    /// The V1 policy (`30Rd:v1-local-edge-limits`).
    pub const V1: Self = Self {
        manifest_bytes: 256,
        signing_document_bytes: 256,
        encryption_document_bytes: 256,
        name_bytes: 192,
        store_entries: 4_096,
        graph_bytes: 256 * 1024 * 1024,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_v1_policy_is_exactly_its_specified_numbers() {
        // Spelled out here as well as at the declaration. A bound is policy, and a policy that
        // drifted by an edit nobody reviewed would still pass every test that merely uses it.
        assert_eq!(LocalLimits::V1.manifest_bytes, 256);
        assert_eq!(LocalLimits::V1.signing_document_bytes, 256);
        assert_eq!(LocalLimits::V1.encryption_document_bytes, 256);
        assert_eq!(LocalLimits::V1.name_bytes, 192);
        assert_eq!(LocalLimits::V1.store_entries, 4_096);
        assert_eq!(LocalLimits::V1.graph_bytes, 268_435_456);
    }

    #[test]
    fn one_receipt_name_fits_the_name_bound_with_room() {
        // The bound has to admit what this crate itself mints, or the store could publish a
        // document it could never enumerate. Measured against the longest species stem rather
        // than assumed.
        let longest = crate::names::NamedSpecies::ALL
            .into_iter()
            .map(|species| {
                crate::names::ReceiptFileName::of(
                    species,
                    dorc_receipt::order::ReceiptOrderToken::of_controller_millis(u64::MAX),
                    &"f".repeat(64),
                )
                .map_or(0, |name| name.spelled().len())
            })
            .max()
            .unwrap_or(0);
        assert!(longest > 0, "a name minted");
        assert!(
            longest < LocalLimits::V1.name_bytes,
            "the longest name this crate mints is {longest} bytes"
        );
    }
}
