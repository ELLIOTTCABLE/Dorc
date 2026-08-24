//! The sealed type parameters: which document a receipt is, which projection it carries,
//! and where its verification material came from.
//!
//! All three traits are sealed by a private supertrait, so the set of species, projections
//! and provenance markers is closed to this crate. An outside type cannot implement one,
//! and a caller cannot ask for a marker by type parameter.

use crate::grammar::RecordKind;

mod sealed {
    pub trait Sealed {}
}

/// Which document this is. Each species names the record kinds its skeleton may contain.
pub trait Species: sealed::Sealed + core::fmt::Debug + Copy {
    /// The literal word in the `species` header line.
    const TOKEN: &'static str;
    /// The record kinds this species admits, in the order a writer emits them.
    const KINDS: &'static [RecordKind];
}

/// Which projection this is.
pub trait Projection: sealed::Sealed + core::fmt::Debug + Copy {
    /// The literal word in the `projection` header line.
    const TOKEN: &'static str;
    /// Whether this projection carries an encrypted region.
    const HAS_OVERLAY: bool;
}

/// Where the verification material for a checked document came from.
pub trait SignerTrust: sealed::Sealed + core::fmt::Debug + Copy {
    /// The word a report renders for this provenance.
    const TOKEN: &'static str;
}

/// The primary record of one planning invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanReceipt;

/// The pre-dispatch commitment of one apply invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyIntent;

/// What one apply invocation reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyOutcome;

impl sealed::Sealed for PlanReceipt {}
impl Species for PlanReceipt {
    const TOKEN: &'static str = "plan";
    const KINDS: &'static [RecordKind] = &[
        RecordKind::Invocation,
        RecordKind::Source,
        RecordKind::Admission,
        RecordKind::PresentedPlan,
        RecordKind::SiteDecision,
        RecordKind::RegionDecision,
        RecordKind::LoadDecision,
        RecordKind::SiteClassification,
        RecordKind::SolveCertification,
        RecordKind::ProbeShip,
        RecordKind::Survival,
        RecordKind::RenderDecision,
        RecordKind::Narrative,
        RecordKind::Licensor,
        RecordKind::ProjectionOmission,
    ];
}

impl sealed::Sealed for ApplyIntent {}
impl Species for ApplyIntent {
    const TOKEN: &'static str = "apply-intent";
    const KINDS: &'static [RecordKind] = &[
        RecordKind::Invocation,
        RecordKind::ApplyIntent,
        RecordKind::ApplyAssignment,
        RecordKind::PlanOrigin,
        RecordKind::ProjectionOmission,
    ];
}

impl sealed::Sealed for ApplyOutcome {}
impl Species for ApplyOutcome {
    const TOKEN: &'static str = "apply-outcome";
    const KINDS: &'static [RecordKind] = &[
        RecordKind::Invocation,
        RecordKind::ApplyOutcome,
        RecordKind::SiteOutcome,
        RecordKind::ProjectionOmission,
    ];
}

/// The projection with no encrypted region. Opaque-capable fields carry a state word and
/// no value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plain;

/// The projection carrying exactly one encrypted region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rich;

impl sealed::Sealed for Plain {}
impl Projection for Plain {
    const TOKEN: &'static str = "plain";
    const HAS_OVERLAY: bool = false;
}

impl sealed::Sealed for Rich {}
impl Projection for Rich {
    const TOKEN: &'static str = "rich";
    const HAS_OVERLAY: bool = true;
}

/// The verification material was named by controller policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrustedReceiptSigner;

/// The verification material was not named by controller policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfAssertedReceiptSigner;

impl sealed::Sealed for TrustedReceiptSigner {}
impl SignerTrust for TrustedReceiptSigner {
    const TOKEN: &'static str = "trusted";
}

impl sealed::Sealed for SelfAssertedReceiptSigner {}
impl SignerTrust for SelfAssertedReceiptSigner {
    const TOKEN: &'static str = "self-asserted";
}

/// The fixed payload type for one species and projection pair.
///
/// Derived from the type parameters, never chosen by a caller, so the value used to compute
/// a signature and the value parsed out of a document cannot come from different places.
#[must_use]
pub fn payload_type<D: Species, P: Projection>() -> String {
    format!("application/vnd.dorc.receipt.v1.{}.{}", D::TOKEN, P::TOKEN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_species_and_projection_pair_has_a_distinct_payload_type() {
        let types = [
            payload_type::<PlanReceipt, Plain>(),
            payload_type::<PlanReceipt, Rich>(),
            payload_type::<ApplyIntent, Plain>(),
            payload_type::<ApplyIntent, Rich>(),
            payload_type::<ApplyOutcome, Plain>(),
            payload_type::<ApplyOutcome, Rich>(),
        ];
        let mut sorted = types.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), types.len(), "two pairs share a payload type");
    }

    #[test]
    fn the_payload_types_are_exactly_the_specified_spellings() {
        assert_eq!(
            payload_type::<PlanReceipt, Plain>(),
            "application/vnd.dorc.receipt.v1.plan.plain"
        );
        assert_eq!(
            payload_type::<ApplyIntent, Rich>(),
            "application/vnd.dorc.receipt.v1.apply-intent.rich"
        );
        assert_eq!(
            payload_type::<ApplyOutcome, Plain>(),
            "application/vnd.dorc.receipt.v1.apply-outcome.plain"
        );
    }

    #[test]
    fn a_species_admits_only_kinds_it_names() {
        // The species table is what makes a rich-only or foreign record refusable without a
        // second vocabulary: a kind absent from this list is not part of the document.
        assert!(PlanReceipt::KINDS.contains(&RecordKind::SiteDecision));
        assert!(!PlanReceipt::KINDS.contains(&RecordKind::SiteOutcome));
        assert!(!ApplyOutcome::KINDS.contains(&RecordKind::SiteDecision));
        assert!(ApplyIntent::KINDS.contains(&RecordKind::ApplyAssignment));
    }

    #[test]
    fn every_species_carries_the_invocation_and_omission_kinds() {
        // Every document says which invocation produced it and what its projection declined
        // to carry, so neither question is answerable only for some species.
        for kinds in [PlanReceipt::KINDS, ApplyIntent::KINDS, ApplyOutcome::KINDS] {
            assert!(kinds.contains(&RecordKind::Invocation));
            assert!(kinds.contains(&RecordKind::ProjectionOmission));
        }
    }

    #[test]
    fn no_species_repeats_a_kind() {
        for kinds in [PlanReceipt::KINDS, ApplyIntent::KINDS, ApplyOutcome::KINDS] {
            let mut seen: Vec<RecordKind> = kinds.to_vec();
            let before = seen.len();
            seen.sort();
            seen.dedup();
            assert_eq!(before, seen.len());
        }
    }
}
