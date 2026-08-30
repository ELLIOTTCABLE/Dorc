//! Which fields may carry a detail value, and under which tag.
//!
//! The tag set is closed and ordered; that order is the overlay canonical ordering and is
//! not a string sort. [`opaque_slots`] maps each record kind to its detail-capable fields,
//! and the map is injective within a kind: two fields of one record sharing a tag would
//! name one overlay slot from two places.

use crate::format::{RefusalReason, Skeleton, SkeletonRecord};
use crate::grammar::{FieldType, IMAGE_STATE, OPAQUE_STATE, RecordKind};
use crate::ids::ReceiptIdSource;

/// A detail-capable field wire tag.
///
/// Closed. The declaration order below is the overlay canonical ordering; adding a member is
/// a reviewed change to this table and to the vectors that pin it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpaqueFieldTag {
    /// The invocation argument vector.
    Argv,
    /// A target name as spelled.
    TargetName,
    /// A recorded source path.
    SourcePath,
    /// A bounded excerpt of a recorded source.
    SourceExcerpt,
    /// One general-sh source's exact acquired bytes.
    ///
    /// A distinct tag rather than a large excerpt, because the two answer different questions and
    /// a reader must be able to tell them apart: an excerpt is a region somebody chose, and this
    /// is the whole file as the run held it (`30Rb:book-content-and-locator-projection`).
    SourceContent,
    /// One recorded site's encoded provenance DAG.
    SiteLocator,
    /// The admitted record stream exact accounted bytes.
    RecordStream,
    /// Shell text.
    Shell,
    /// A coordinate, kind, entity, or selector.
    Fact,
    /// A source locator.
    Locator,
    /// A custody description.
    Custody,
    /// An import path.
    ImportPath,
    /// An emitted name.
    EmittedName,
    /// An operand of a diagnostic.
    DiagnosticOperand,
    /// One apply artifact image, by value.
    ApplyArtifactImage,
    /// Admitted standard output.
    Stdout,
    /// Admitted standard error.
    Stderr,
    /// An error detail tail.
    ErrorDetail,
    /// An apply assignment resolved context.
    ApplyContext,
}

impl OpaqueFieldTag {
    /// Every tag, in canonical order.
    pub const ALL: [Self; 19] = [
        Self::Argv,
        Self::TargetName,
        Self::SourcePath,
        Self::SourceExcerpt,
        Self::SourceContent,
        Self::SiteLocator,
        Self::RecordStream,
        Self::Shell,
        Self::Fact,
        Self::Locator,
        Self::Custody,
        Self::ImportPath,
        Self::EmittedName,
        Self::DiagnosticOperand,
        Self::ApplyArtifactImage,
        Self::Stdout,
        Self::Stderr,
        Self::ErrorDetail,
        Self::ApplyContext,
    ];

    /// The literal wire token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Argv => "argv",
            Self::TargetName => "target-name",
            Self::SourcePath => "source-path",
            Self::SourceExcerpt => "source-excerpt",
            Self::SourceContent => "source-content",
            Self::SiteLocator => "site-locator",
            Self::RecordStream => "record-stream",
            Self::Shell => "shell",
            Self::Fact => "fact",
            Self::Locator => "locator",
            Self::Custody => "custody",
            Self::ImportPath => "import-path",
            Self::EmittedName => "emitted-name",
            Self::DiagnosticOperand => "diagnostic-operand",
            Self::ApplyArtifactImage => "apply-artifact-image",
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
            Self::ErrorDetail => "error-detail",
            Self::ApplyContext => "apply-context",
        }
    }

    /// This tag canonical ordinal.
    #[must_use]
    pub fn order(self) -> usize {
        Self::ALL
            .iter()
            .position(|tag| *tag == self)
            .unwrap_or(usize::MAX)
    }

    /// The tag a token names. Matching the tag alphabet never admits an unknown token.
    #[must_use]
    pub fn of_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tag| tag.token() == token)
    }
}

/// One detail-capable field of one record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpaqueSlot {
    /// The skeleton field key whose state word governs this slot.
    pub key: &'static str,
    /// The tag the overlay keys this slot by.
    pub tag: OpaqueFieldTag,
}

const fn s(key: &'static str, tag: OpaqueFieldTag) -> OpaqueSlot {
    OpaqueSlot { key, tag }
}

const INVOCATION_SLOTS: &[OpaqueSlot] = &[
    s("argv", OpaqueFieldTag::Argv),
    s("target", OpaqueFieldTag::TargetName),
];
const SOURCE_SLOTS: &[OpaqueSlot] = &[
    s("path", OpaqueFieldTag::SourcePath),
    s("excerpt", OpaqueFieldTag::SourceExcerpt),
    s("content", OpaqueFieldTag::SourceContent),
];
const ADMISSION_SLOTS: &[OpaqueSlot] = &[s("stream", OpaqueFieldTag::RecordStream)];
const SHELL_SLOTS: &[OpaqueSlot] = &[s("shell", OpaqueFieldTag::Shell)];
/// A site decision carries its shell text AND its provenance DAG.
///
/// Split from [`SHELL_SLOTS`], which a region decision still takes: a region is one authored edit
/// many executions share, so it has no single site locator to carry and giving it the slot would
/// invite one instance's provenance to stand for all of them
/// (`30N:rul-region-refusal-discloses-region-keyed`, one level up).
const SITE_DECISION_SLOTS: &[OpaqueSlot] = &[
    s("shell", OpaqueFieldTag::Shell),
    s("locator", OpaqueFieldTag::SiteLocator),
];
const LOAD_SLOTS: &[OpaqueSlot] = &[
    s("name", OpaqueFieldTag::ImportPath),
    s("custody", OpaqueFieldTag::Custody),
];
const PROBE_SHIP_SLOTS: &[OpaqueSlot] = &[s("source", OpaqueFieldTag::Shell)];
const SURVIVAL_SLOTS: &[OpaqueSlot] = &[s("poison", OpaqueFieldTag::Locator)];
const RENDER_SLOTS: &[OpaqueSlot] = &[s("detail", OpaqueFieldTag::DiagnosticOperand)];
const LICENSOR_SLOTS: &[OpaqueSlot] = &[s("locus", OpaqueFieldTag::Locator)];
const ASSIGNMENT_SLOTS: &[OpaqueSlot] = &[
    s("target", OpaqueFieldTag::TargetName),
    s("context", OpaqueFieldTag::ApplyContext),
    s("image-state", OpaqueFieldTag::ApplyArtifactImage),
];
const SITE_OUTCOME_SLOTS: &[OpaqueSlot] = &[
    s("stdout", OpaqueFieldTag::Stdout),
    s("stderr", OpaqueFieldTag::Stderr),
];
const NO_SLOTS: &[OpaqueSlot] = &[];

/// The detail-capable fields of one record kind, in field order.
///
/// One arm per kind and no wildcard, so a new kind cannot land without being classified,
/// including as carrying nothing.
#[must_use]
pub const fn opaque_slots(kind: RecordKind) -> &'static [OpaqueSlot] {
    match kind {
        RecordKind::Invocation => INVOCATION_SLOTS,
        RecordKind::Source => SOURCE_SLOTS,
        RecordKind::Admission => ADMISSION_SLOTS,
        RecordKind::SiteDecision => SITE_DECISION_SLOTS,
        RecordKind::RegionDecision => SHELL_SLOTS,
        RecordKind::LoadDecision => LOAD_SLOTS,
        RecordKind::ProbeShip => PROBE_SHIP_SLOTS,
        RecordKind::Survival => SURVIVAL_SLOTS,
        RecordKind::RenderDecision => RENDER_SLOTS,
        RecordKind::Licensor => LICENSOR_SLOTS,
        RecordKind::ApplyAssignment => ASSIGNMENT_SLOTS,
        RecordKind::SiteOutcome => SITE_OUTCOME_SLOTS,
        RecordKind::PresentedPlan
        | RecordKind::SiteClassification
        | RecordKind::SolveCertification
        | RecordKind::Narrative
        | RecordKind::ProjectionOmission
        | RecordKind::ApplyIntent
        | RecordKind::PlanOrigin
        | RecordKind::ApplyOutcome => NO_SLOTS,
    }
}

/// The state word meaning a value was captured and therefore rides the overlay.
pub const CAPTURED: &str = "captured";

/// Whether a field key carries one of the detail state vocabularies.
#[must_use]
pub fn carries_a_state_word(kind: RecordKind, key: &str) -> bool {
    kind.fields().iter().any(|field| {
        field.key == key
            && matches!(field.ty, FieldType::Closed(set) if set == OPAQUE_STATE || set == IMAGE_STATE)
    })
}

/// The state word a detail value carries once its projection cannot hold it.
pub const WITHHELD_PLAIN: &str = "withheld-plain";

/// Narrow a rich skeleton to its plain projection, minting a fresh identity for the result.
///
/// A semantic remint, never a textual strip: every captured slot becomes a withheld one, the
/// encryption provider line goes, and the document gets its own identity from the injected
/// source like any other. The caller serializes and signs the result, which is what makes the
/// plain signature cover plain bytes under the plain domain.
///
/// The narrowed document carries no recorded reference back to the one it was narrowed from.
///
/// `order` is the caller's, not the origin's: a remint is a second document written at a second
/// moment, and inheriting the first one's order would put two documents at one store position
/// and call it a tie.
///
/// # Errors
/// Refuses if a narrowed record no longer satisfies its own field table.
pub fn narrow_to_plain(
    rich: &Skeleton,
    ids: &mut dyn ReceiptIdSource,
    order: crate::order::ReceiptOrderToken,
) -> Result<Skeleton, RefusalReason> {
    let mut records = Vec::with_capacity(rich.records.len());
    for record in &rich.records {
        let kind = record.kind();
        let slots = opaque_slots(kind);
        let mut atoms: Vec<String> = record.atoms().to_vec();
        for (index, field) in kind.fields().iter().enumerate() {
            if !slots.iter().any(|slot| slot.key == field.key) {
                continue;
            }
            if let Some(atom) = atoms.get_mut(index)
                && atom == CAPTURED
            {
                WITHHELD_PLAIN.clone_into(atom);
            }
        }
        records.push(SkeletonRecord::build(kind, atoms)?);
    }
    Ok(Skeleton {
        receipt_id: ids.next_receipt_id().hex(),
        order,
        signing_key_id: rich.signing_key_id.clone(),
        encryption_key_id: None,
        records,
    })
}

/// How two documents carrying one receipt identity relate.
///
/// An identity is minted per document, so two documents holding one is unambiguous: either they
/// are the same bytes seen twice, or they disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameIdentityPair {
    /// One byte-image, encountered twice.
    Identical,
    /// Two byte-images under one identity. A finding: retain both, prefer neither.
    Divergent,
}

/// Classify two documents already known to carry the same receipt identity.
#[must_use]
pub fn same_identity_pair(left_bytes: &[u8], right_bytes: &[u8]) -> SameIdentityPair {
    if left_bytes == right_bytes {
        SameIdentityPair::Identical
    } else {
        SameIdentityPair::Divergent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tag_round_trips_through_its_token_and_an_unknown_token_is_not_admitted() {
        for tag in OpaqueFieldTag::ALL {
            assert_eq!(OpaqueFieldTag::of_token(tag.token()), Some(tag));
        }
        assert_eq!(OpaqueFieldTag::of_token("argv-"), None);
        assert_eq!(OpaqueFieldTag::of_token(""), None);
    }

    #[test]
    fn the_tag_tokens_are_distinct_and_the_order_is_the_declaration_order() {
        let mut tokens: Vec<&str> = OpaqueFieldTag::ALL.iter().map(|t| t.token()).collect();
        let before = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(before, tokens.len(), "two tags share a token");
        for (index, tag) in OpaqueFieldTag::ALL.into_iter().enumerate() {
            assert_eq!(tag.order(), index);
        }
    }

    #[test]
    fn canonical_order_is_the_table_and_never_a_string_sort() {
        // target-name sorts after source-path alphabetically and before it here. Pinning the
        // disagreement stops a reader from tidying the ordering into a lexicographic one.
        assert!(OpaqueFieldTag::TargetName.order() < OpaqueFieldTag::SourcePath.order());
        assert!(OpaqueFieldTag::TargetName.token() > OpaqueFieldTag::SourcePath.token());
    }

    #[test]
    fn every_slot_names_a_field_its_own_kind_declares() {
        for kind in RecordKind::ALL {
            for slot in opaque_slots(kind) {
                assert!(
                    kind.fields().iter().any(|field| field.key == slot.key),
                    "{}: no field {}",
                    kind.token(),
                    slot.key
                );
            }
        }
    }

    #[test]
    fn a_kind_never_aliases_one_tag_across_two_of_its_fields() {
        // Two fields of one record sharing a tag would address one overlay slot from two
        // places, and the second would alias the first rather than be refused.
        for kind in RecordKind::ALL {
            let mut tags: Vec<OpaqueFieldTag> =
                opaque_slots(kind).iter().map(|slot| slot.tag).collect();
            let before = tags.len();
            tags.sort_unstable();
            tags.dedup();
            assert_eq!(before, tags.len(), "{} aliases a tag", kind.token());
        }
    }

    #[test]
    fn a_field_has_a_slot_exactly_when_it_carries_a_state_word() {
        // The grammar table and this one are two halves of one statement. A field whose state
        // word can say captured but has no slot is a value with nowhere to ride; a slot over a
        // field with no state word demands one that cannot be spelled.
        for kind in RecordKind::ALL {
            for field in kind.fields() {
                let stateful = carries_a_state_word(kind, field.key);
                let slotted = opaque_slots(kind).iter().any(|slot| slot.key == field.key);
                assert_eq!(
                    stateful,
                    slotted,
                    "{}.{}: stateful={stateful} slotted={slotted}",
                    kind.token(),
                    field.key
                );
            }
        }
    }
}
