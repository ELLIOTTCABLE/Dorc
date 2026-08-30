//! The sealed type parameters: which document a receipt is, and which projection it carries.
//!
//! Both traits are sealed by a private supertrait, so the set of species and projections is
//! closed to this crate. An outside type cannot implement one.
//!
//! There is deliberately no PROVENANCE parameter. A third marker used to ride here saying
//! whether the verification material was "named by controller policy", and it was minted from
//! two public traits any crate could implement — so the state meant only that somebody had said
//! so. This crate answers whether a signature is valid under a key; who owns the key is a
//! question about a keyset on disk, answered by the seat that opened one.

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
///
/// The associated region is what makes a plain document unable to hold detail: its region
/// type has no value to hold one, so the state is unrepresentable rather than merely unused.
pub trait Projection: sealed::Sealed + core::fmt::Debug + Copy {
    /// The literal word in the `projection` header line.
    const TOKEN: &'static str;
    /// Whether this projection carries an encrypted region.
    const HAS_OVERLAY: bool;
    /// What this projection carries where a region would be.
    type Region: core::fmt::Debug;
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

/// What a plain document carries where a region would be: nothing, and no way to hold one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoOpaqueOverlay;

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
    type Region = NoOpaqueOverlay;
}

impl sealed::Sealed for Rich {}
impl Projection for Rich {
    const TOKEN: &'static str = "rich";
    const HAS_OVERLAY: bool = true;
    type Region = crate::overlay::ValidatedOpaqueOverlay;
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
