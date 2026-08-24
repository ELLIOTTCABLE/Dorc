//! Typed bounds every parser accepts as one policy value.
//!
//! Each bound is a private-field newtype so a caller cannot pass one where another was
//! meant, and [`ReceiptLimits`] is assembled whole rather than defaulted per-call: a parser
//! that took its own bound would let two parsers disagree about the same document. A nested
//! parser consumes both the parent budget and its own.

/// The most digits a canonical integer may carry. Sized so a `u64` always fits and a longer
/// run is refused before any parse is attempted.
pub const MAX_INTEGER_DIGITS: usize = 20;

/// One byte bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteLimit(u64);

impl ByteLimit {
    /// Declare a bound.
    #[must_use]
    pub const fn of(bytes: u64) -> Self {
        Self(bytes)
    }

    /// The bound, for comparison against a measured length.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Whether `measured` is within the bound.
    #[must_use]
    pub const fn admits(self, measured: u64) -> bool {
        measured <= self.0
    }
}

/// One cardinality bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CountLimit(u64);

impl CountLimit {
    /// Declare a bound.
    #[must_use]
    pub const fn of(count: u64) -> Self {
        Self(count)
    }

    /// The bound, for comparison against a measured count.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Whether `measured` is within the bound.
    #[must_use]
    pub const fn admits(self, measured: u64) -> bool {
        measured <= self.0
    }
}

/// The complete bound policy one document is read under.
///
/// Assembled, never defaulted at a call site. [`ReceiptLimits::V1`] is the committed
/// conformance policy; a caller may narrow a bound to keep a path bounded, and widening one
/// is a reviewed act with boundary tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptLimits {
    /// The whole document, before anything is parsed or allocated.
    pub outer_bytes: ByteLimit,
    /// The literal skeleton span.
    pub skeleton_bytes: ByteLimit,
    /// One structural line.
    pub line_bytes: ByteLimit,
    /// One structural field.
    pub field_bytes: ByteLimit,
    /// Skeleton records.
    pub records: CountLimit,
    /// The armored region.
    pub armor_bytes: ByteLimit,
    /// The decrypted overlay.
    pub overlay_bytes: ByteLimit,
    /// Overlay entries.
    pub overlay_entries: CountLimit,
    /// One opaque field's value.
    pub opaque_field_bytes: ByteLimit,
    /// One apply image, whole.
    pub image_bytes: ByteLimit,
    /// One apply-image entry.
    pub image_entry_bytes: ByteLimit,
    /// Apply-image entries.
    pub image_entries: CountLimit,
    /// Topology edges.
    pub topology_edges: CountLimit,
    /// Topology depth.
    pub topology_depth: CountLimit,
    /// One recorded relative path.
    pub path_bytes: ByteLimit,
    /// Argv words.
    pub argv_entries: CountLimit,
    /// Recorded source identities.
    pub source_identities: CountLimit,
    /// One source excerpt.
    pub excerpt_bytes: ByteLimit,
    /// Every source excerpt together.
    pub excerpt_aggregate_bytes: ByteLimit,
    /// The admitted record block.
    pub admitted_records_bytes: ByteLimit,
    /// Admitted host output, together.
    pub host_output_bytes: ByteLimit,
}

impl ReceiptLimits {
    /// The committed conformance policy.
    pub const V1: Self = Self {
        outer_bytes: ByteLimit::of(64 * MIB),
        skeleton_bytes: ByteLimit::of(8 * MIB),
        line_bytes: ByteLimit::of(64 * KIB),
        field_bytes: ByteLimit::of(16 * KIB),
        records: CountLimit::of(65_536),
        armor_bytes: ByteLimit::of(48 * MIB),
        overlay_bytes: ByteLimit::of(32 * MIB),
        overlay_entries: CountLimit::of(65_536),
        opaque_field_bytes: ByteLimit::of(24 * MIB),
        image_bytes: ByteLimit::of(24 * MIB),
        image_entry_bytes: ByteLimit::of(16 * MIB),
        image_entries: CountLimit::of(8_192),
        topology_edges: CountLimit::of(65_536),
        topology_depth: CountLimit::of(1_024),
        path_bytes: ByteLimit::of(4 * KIB),
        argv_entries: CountLimit::of(32_768),
        source_identities: CountLimit::of(32_768),
        excerpt_bytes: ByteLimit::of(64 * KIB),
        excerpt_aggregate_bytes: ByteLimit::of(MIB),
        admitted_records_bytes: ByteLimit::of(4 * MIB),
        host_output_bytes: ByteLimit::of(4 * MIB),
    };
}

const KIB: u64 = 1024;
const MIB: u64 = 1024 * KIB;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bound_admits_its_own_value_and_refuses_one_past_it() {
        // Boundary-minus / at / plus on the shared predicate, so every bound inherits it.
        let bound = ByteLimit::of(10);
        assert!(bound.admits(9));
        assert!(bound.admits(10));
        assert!(!bound.admits(11));
        let count = CountLimit::of(0);
        assert!(count.admits(0), "a zero bound still admits emptiness");
        assert!(!count.admits(1));
    }

    #[test]
    fn the_skeleton_bound_sits_inside_the_outer_bound() {
        // A skeleton bound above the document bound would be unreachable, which reads as a
        // policy nobody checked rather than one deliberately set.
        let l = ReceiptLimits::V1;
        assert!(l.skeleton_bytes.get() <= l.outer_bytes.get());
        assert!(l.armor_bytes.get() <= l.outer_bytes.get());
        assert!(l.field_bytes.get() <= l.line_bytes.get());
        assert!(l.line_bytes.get() <= l.skeleton_bytes.get());
        assert!(l.image_entry_bytes.get() <= l.image_bytes.get());
        assert!(l.excerpt_bytes.get() <= l.excerpt_aggregate_bytes.get());
    }

    #[test]
    fn the_digit_cap_admits_every_u64_and_refuses_a_longer_run() {
        assert_eq!(u64::MAX.to_string().len(), MAX_INTEGER_DIGITS);
    }
}
