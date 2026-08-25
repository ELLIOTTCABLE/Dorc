//! Turning what an apply will actually use into the exact image an `ApplyIntent` binds.
//!
//! # Why this is not a field rename
//!
//! [`ArtifactSet`](crate::artifact::ArtifactSet) is a flat bag: a primary and a list of
//! `{path, bytes}`. An [`ApplyArtifactImage`] additionally records ROOTS (which top-level
//! authored unit each entry materializes), EDGES (which entry loads or contains which), and a
//! MODE per entry. The first two are derived at selection time from the bundle projection and
//! the load account, and `Selection::with_plan` does not carry them forward — so a set with
//! dependencies cannot yet be turned into an exact image, and this module REFUSES rather than
//! synthesizing a topology it cannot observe.
//!
//! That refusal is the whole design here. A flat `plan.sh loads everything` edge set would be
//! wrong for the mirrored form the moment one dependency sources another: the image would
//! record the plan as the loader of a file some other dependency loads, which is a false
//! statement about what the apply uses, in a container whose entire promise is exactness.
//!
//! # Why every mode is `unused`, and why that is a statement
//!
//! [`RecordedMode`] has no unknown arm, deliberately: a caller that cannot say whether an
//! entry's mode is an execution input must refuse at its own seat. This is that seat, and the
//! answer is measured rather than assumed. Everything the artifact publisher writes is created
//! `0o600` inside a `0o700` directory (`artifact_store::create_exclusive`), and the artifact is
//! invoked as an ARGUMENT to an interpreter — the multipart execution contract is
//! `cd <artifact> && sh ./plan.sh` — never through its own execute bit; dependencies are
//! reached by `.`, which needs read and not execute. So mode is genuinely not an execution
//! input for anything Dorc emits, and `unused` is true of it.
//!
//! The refusal arm stays reachable for anything NOT minted from an artifact set or an external
//! stream. An admin-supplied tree whose entrypoint runs as `./run.sh` would have a mode that
//! IS an execution input, and there is no constructor here that would let it through wearing
//! `unused`.

use dorc_receipt::image::{
    ApplyArtifactImage, ApplyEntryBytes, ApplyEntryId, ApplyImageEntry, ApplyRoot, ApplyRootId,
    ApplyTopology, ImageRefusal, RecordedApplyPath, RecordedArtifactForm, RecordedMode,
};
use dorc_receipt::limits::ReceiptLimits;

use crate::artifact::{ArtifactForm, ArtifactSet};

/// Why an emitted artifact set could not be recorded as an exact image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageCarriageRefusal {
    /// The set carries dependency files whose load topology the selection did not retain.
    ///
    /// Not a bound and not a malformed input: the artifact is fine and the RECORD cannot yet
    /// be made exact. Named separately from every [`ImageRefusal`] so a reader is never sent
    /// looking for a bad path when the missing thing is an edge.
    TopologyNotCarried {
        /// How many dependency files the set placed.
        dependencies: usize,
    },
    /// The image model refused the entries this set produced.
    Image(ImageRefusal),
}

impl From<ImageRefusal> for ImageCarriageRefusal {
    fn from(refusal: ImageRefusal) -> Self {
        Self::Image(refusal)
    }
}

/// The recorded form word for an emitted form.
const fn recorded_form(form: ArtifactForm) -> RecordedArtifactForm {
    match form {
        ArtifactForm::Flattened => RecordedArtifactForm::Flattened,
        ArtifactForm::Multipart => RecordedArtifactForm::Multipart,
        ArtifactForm::MirroredTree => RecordedArtifactForm::MirroredTree,
        ArtifactForm::PreservedBookTree => RecordedArtifactForm::PreservedBookTree,
    }
}

/// Record one emitted artifact set as the exact image an apply would use.
///
/// # Errors
/// Refuses a set carrying dependencies (their load topology is not carried yet) and every
/// structural condition the image model refuses.
pub fn image_of_artifact_set(
    set: &ArtifactSet,
    limits: &ReceiptLimits,
) -> Result<ApplyArtifactImage, ImageCarriageRefusal> {
    let dependencies = set.dependencies().len();
    if dependencies > 0 {
        return Err(ImageCarriageRefusal::TopologyNotCarried { dependencies });
    }
    // With no file beside it the plan IS the whole artifact: one entry, the one authored unit
    // that entry materializes, one entrypoint, and no edge to state — every relation the
    // container can express is either present or vacuously absent, so the image is exact
    // rather than merely plausible.
    let primary = set.primary();
    let path = RecordedApplyPath::of(primary.path.as_bytes(), limits)
        .map_err(|refusal| ImageCarriageRefusal::Image(refusal.into()))?;
    let entry = ApplyImageEntry::file(
        ApplyEntryId::of(0),
        path,
        RecordedMode::Unused,
        ApplyEntryBytes::of(primary.bytes.clone().into_bytes()),
    );
    ApplyArtifactImage::of_parts(
        recorded_form(set.form()),
        vec![entry],
        vec![ApplyRoot::of(ApplyRootId::of(0), ApplyEntryId::of(0))],
        vec![ApplyEntryId::of(0)],
        ApplyTopology::of(Vec::new()),
        limits,
    )
    .map_err(ImageCarriageRefusal::Image)
}

/// Record bytes the apply was HANDED — `--plan <file>` or stdin — as the exact image.
///
/// A distinct constructor rather than a flag on the one above, because there is no artifact
/// set here and nothing to invent one from: no form Dorc chose, no path the bytes belong at,
/// and no bundle root. The container's own single-stream shape says exactly that.
///
/// # Errors
/// Refuses content past the per-entry or aggregate bound.
pub fn image_of_external_stream(
    bytes: Vec<u8>,
    limits: &ReceiptLimits,
) -> Result<ApplyArtifactImage, ImageCarriageRefusal> {
    ApplyArtifactImage::of_external_stream(ApplyEntryBytes::of(bytes), limits)
        .map_err(ImageCarriageRefusal::Image)
}

#[cfg(test)]
mod tests {
    use super::{ImageCarriageRefusal, image_of_artifact_set, image_of_external_stream};
    use crate::artifact::{ArtifactForm, ArtifactSet, FormRequest, StreamPosture, select};
    use crate::bundle::BundleProjection;
    use dorc_core::loadpath::Cwd;
    use dorc_receipt::image::{ApplyEntryKind, RecordedArtifactForm, RecordedMode};
    use dorc_receipt::limits::ReceiptLimits;

    fn set_of(plan_sh: &str) -> ArtifactSet {
        let snapshot = crate::snapshot::StaticLoadSnapshot::over(
            Cwd::default(),
            Vec::new(),
            Vec::new(),
            &crate::snapshot::LoadPositions::roots_only(),
            "book.sh",
            "",
        );
        select(
            &snapshot,
            &BundleProjection::default(),
            &[],
            FormRequest::Auto,
            StreamPosture::TerminalRender,
        )
        .expect("a book with no loads places nothing")
        // The authored posture is TRUE here and is asserted rather than assumed: this fixture
        // reaches no host, so every byte of it is controller-supplied invocation material and
        // authored text. The seat joins `plan::spine`'s two-way census in the same commit.
        .with_plan(
            plan_sh.to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        )
    }

    /// The whole corpus's shape, recorded exactly: one entry, its own root, its own entrypoint,
    /// and NO edge — every relation the container can state is present or vacuously absent, which
    /// is what makes a dependency-free set exactly recordable today.
    #[test]
    fn a_dependency_free_set_records_as_one_entry_carrying_the_plans_own_bytes() {
        let set = set_of("#!/bin/sh\napt-get install -y nginx\n");
        assert_eq!(set.form(), ArtifactForm::Flattened);
        let image = image_of_artifact_set(&set, &ReceiptLimits::V1).expect("no dependencies");

        assert_eq!(image.form(), RecordedArtifactForm::Flattened);
        assert_eq!(image.entries().len(), 1);
        assert_eq!(image.roots().len(), 1);
        assert_eq!(image.entrypoints().len(), 1);
        assert!(
            image.topology().edges().is_empty(),
            "one entry loads nothing, so the edge set is empty rather than invented"
        );

        let entry = image.entries().first().expect("one entry");
        assert_eq!(entry.kind(), ApplyEntryKind::File);
        assert_eq!(
            entry.path().map(|path| path.text().to_owned()),
            Some("plan.sh".to_owned())
        );
        assert_eq!(
            entry.bytes().get(),
            b"#!/bin/sh\napt-get install -y nginx\n",
            "the recorded bytes are the plan projection's own"
        );
        assert_eq!(
            entry.mode(),
            RecordedMode::Unused,
            "measured, not assumed: nothing published is executable and the plan is run as an \
             argument to an interpreter"
        );
    }

    /// Bytes the apply was HANDED carry no path, because there is none to fabricate — and the
    /// image says that with its kind rather than with an empty string.
    #[test]
    fn handed_bytes_record_as_a_path_less_stream() {
        let image = image_of_external_stream(
            b"#!/bin/sh\nufw allow 443/tcp\n".to_vec(),
            &ReceiptLimits::V1,
        )
        .expect("a single stream is within bounds");

        assert_eq!(image.form(), RecordedArtifactForm::ExternalStream);
        let entry = image.entries().first().expect("one entry");
        assert_eq!(entry.kind(), ApplyEntryKind::Stream);
        assert_eq!(
            entry.path(),
            None,
            "a stream has no name to materialize under"
        );
    }

    /// Two different books never share an image identity — the identity is over the bytes and
    /// the topology, so a cousin is a different image rather than the same one.
    #[test]
    fn two_plans_differing_in_one_byte_record_as_different_images() {
        let one = image_of_artifact_set(&set_of("#!/bin/sh\n:\n"), &ReceiptLimits::V1)
            .expect("no dependencies");
        let two = image_of_artifact_set(&set_of("#!/bin/sh\n: \n"), &ReceiptLimits::V1)
            .expect("no dependencies");
        assert_ne!(one.id(), two.id());
    }

    /// The refusal is a VARIANT, not a panic and not a silent empty topology: a reader of the
    /// error learns that the artifact was fine and the record could not be made exact.
    #[test]
    fn the_carriage_refusal_names_dependencies_rather_than_a_malformed_input() {
        let refusal = ImageCarriageRefusal::TopologyNotCarried { dependencies: 3 };
        assert_ne!(
            refusal,
            ImageCarriageRefusal::TopologyNotCarried { dependencies: 2 },
            "the count is part of the refusal, so a reader is not told merely that something \
             was carried"
        );
    }
}
