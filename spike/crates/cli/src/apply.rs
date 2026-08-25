//! Turning what an apply will actually use into the exact image an `ApplyIntent` binds.
//!
//! # Why this is not a field rename
//!
//! An [`ApplyArtifactImage`] records more than a list of `{path, bytes}`: ROOTS (which published
//! path each authored load materializes), EDGES (which file reaches which), and a MODE per entry.
//! The first two are answerable only where the placement happens — the bundle projection knows
//! which entry a root materializes and the load account knows which occurrence encloses which —
//! so [`ArtifactSet`](crate::artifact::ArtifactSet) CARRIES them
//! ([`ArtifactTopology`](crate::artifact::ArtifactTopology)) and this module reads that carriage
//! rather than reconstructing it.
//!
//! Reading it is the whole design here. A flat `plan.sh loads everything` edge set would be wrong
//! for the mirrored form the moment one dependency sources another: the image would record the
//! plan as the loader of a file some other dependency loads, which is a false statement about
//! what the apply uses, in a container whose entire promise is exactness. Where the carriage is
//! incomplete the FORM refuses at selection, before any network contact; where it names a path
//! the set does not publish, this module refuses. Neither guesses.
//!
//! # Roots, and why the plan is one
//!
//! Root 0 is always the plan projection itself, with the authored loads' roots appended after it.
//! A `root` line therefore means the same thing in a one-file image as in a multi-file one — the
//! alternative would change the field's shape the moment a dependency appeared, which is the
//! class of drift a positional record cannot sense-check.
//!
//! Every edge this form can state is a `loads`. An ABSORBED dependency is bytes inside another
//! file rather than a file of its own, so it has no entry to name and casts no edge; `contains`
//! would need both ends published, which no form here does.
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

use std::collections::BTreeMap;

use dorc_receipt::image::{
    ApplyArtifactImage, ApplyEdge, ApplyEdgeKind, ApplyEntryBytes, ApplyEntryId, ApplyImageEntry,
    ApplyRoot, ApplyRootId, ApplyTopology, ImageRefusal, RecordedApplyPath, RecordedArtifactForm,
    RecordedMode,
};
use dorc_receipt::limits::ReceiptLimits;

use crate::artifact::{ArtifactForm, ArtifactSet};

/// Why an emitted artifact set could not be recorded as an exact image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageCarriageRefusal {
    /// The carried topology names a path the set does not publish.
    ///
    /// Not a bound and not a malformed input: the two halves of the carriage disagree about
    /// which files exist. Named separately from every [`ImageRefusal`] so a reader is never sent
    /// looking for a bad path when the wrong thing is an edge.
    TopologyNamesUnpublishedPath {
        /// The path the topology named.
        path: String,
    },
    /// The set publishes more files than an entry ordinal can name.
    TooManyEntries {
        /// How many files the set publishes.
        files: usize,
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

/// The entry ordinal the plan projection always occupies.
const PLAN_ENTRY: ApplyEntryId = ApplyEntryId::of(0);

/// Record one emitted artifact set as the exact image an apply would use.
///
/// The set's own publication order fixes the entry ordinals — the plan first, then its
/// dependencies — and the carried topology is resolved against those same paths, so nothing here
/// re-derives which file reaches which.
///
/// # Errors
/// Refuses a topology naming an unpublished path, a set past the ordinal space, and every
/// structural condition the image model refuses.
pub fn image_of_artifact_set(
    set: &ArtifactSet,
    limits: &ReceiptLimits,
) -> Result<ApplyArtifactImage, ImageCarriageRefusal> {
    let mut entries: Vec<ApplyImageEntry> = Vec::new();
    let mut entry_of: BTreeMap<&str, ApplyEntryId> = BTreeMap::new();
    for (ordinal, file) in set.files().enumerate() {
        let ordinal = u32::try_from(ordinal).map_err(|_| ImageCarriageRefusal::TooManyEntries {
            files: set.dependencies().len().saturating_add(1),
        })?;
        let id = ApplyEntryId::of(ordinal);
        let path = RecordedApplyPath::of(file.path.as_bytes(), limits)
            .map_err(|refusal| ImageCarriageRefusal::Image(refusal.into()))?;
        entries.push(ApplyImageEntry::file(
            id,
            path,
            RecordedMode::Unused,
            ApplyEntryBytes::of(file.bytes.clone().into_bytes()),
        ));
        entry_of.insert(file.path.as_str(), id);
    }

    let resolve = |path: &str| {
        entry_of.get(path).copied().ok_or_else(|| {
            ImageCarriageRefusal::TopologyNamesUnpublishedPath {
                path: path.to_owned(),
            }
        })
    };

    let mut roots = vec![ApplyRoot::of(ApplyRootId::of(0), PLAN_ENTRY)];
    for path in set.topology().roots() {
        let ordinal =
            u32::try_from(roots.len()).map_err(|_| ImageCarriageRefusal::TooManyEntries {
                files: roots.len().saturating_add(1),
            })?;
        roots.push(ApplyRoot::of(ApplyRootId::of(ordinal), resolve(path)?));
    }

    let mut edges: Vec<ApplyEdge> = Vec::new();
    for edge in set.topology().edges() {
        edges.push(ApplyEdge::of(
            resolve(&edge.parent)?,
            resolve(&edge.child)?,
            ApplyEdgeKind::Loads,
        ));
    }

    ApplyArtifactImage::of_parts(
        recorded_form(set.form()),
        entries,
        roots,
        vec![PLAN_ENTRY],
        ApplyTopology::of(edges),
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
    use super::{image_of_artifact_set, image_of_external_stream};
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

    /// Build a whole world over a BOOK-sourced tree and settle one named form over it.
    ///
    /// Spelled out here rather than shared with `artifact`'s own battery: these tests are about
    /// what the IMAGE records, and a helper that quietly changed which world it built would move
    /// both sides of the comparison at once.
    fn image_over(
        book: &str,
        paths: Vec<String>,
        srcs: Vec<String>,
        request: FormRequest,
    ) -> dorc_receipt::image::ApplyArtifactImage {
        let cwd = Cwd::default();
        let reached = crate::snapshot::book_reached(&cwd, &paths, &srcs, book);
        let snapshot = crate::snapshot::StaticLoadSnapshot::over(
            cwd.clone(),
            paths,
            srcs,
            &crate::snapshot::LoadPositions::book_sourced(reached),
            "book.sh",
            book,
        );
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut interner = dorc_core::Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        let projection = crate::bundle::project(&snapshot, env.loads())
            .map(crate::bundle::BundleProjectionOutput::into_projection)
            .expect("one closed occurrence forest");
        let loads = crate::artifact::book_loads(&cfg, &ast, book, &projection, &env);
        let set = select(
            &snapshot,
            &projection,
            &loads,
            request,
            StreamPosture::Materializable,
        )
        .expect("a relative dependency is placeable")
        .with_plan(
            "#!/bin/sh\n:\n".to_owned(),
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );
        image_of_artifact_set(&set, &ReceiptLimits::V1).expect("the topology is carried")
    }

    /// Which entry an edge names, as the published path — the form every assertion below reads,
    /// because an ordinal says nothing to a reader of a failure.
    fn edges_of(image: &dorc_receipt::image::ApplyArtifactImage) -> Vec<(String, String)> {
        let path_of = |id: dorc_receipt::image::ApplyEntryId| {
            image
                .entries()
                .iter()
                .find(|entry| entry.id() == id)
                .and_then(|entry| entry.path().map(|path| path.text().to_owned()))
                .unwrap_or_else(|| "<stream>".to_owned())
        };
        image
            .topology()
            .edges()
            .iter()
            .map(|edge| (path_of(edge.parent()), path_of(edge.child())))
            .collect()
    }

    /// THE FALSIFIER for `ruling-image-roots-are-plan-zero-plus-bundle-roots`: root 0 means the
    /// same thing whether or not a dependency exists.
    ///
    /// A `root` line whose subject changed shape once a second file appeared would be a positional
    /// record that is range-checked and never sense-checked — valid-looking in both cases and
    /// describing different things. If a later change demotes the plan out of the root set, this
    /// is the test that says so.
    #[test]
    fn root_zero_is_the_plan_in_a_one_file_image_and_in_a_multi_file_one() {
        let alone = image_of_artifact_set(
            &set_of("#!/bin/sh\napt-get install -y nginx\n"),
            &ReceiptLimits::V1,
        )
        .expect("no dependencies");
        let beside = image_over(
            ". ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec![
                "# dorc-lang/v0.2\nwombat__is_converged() { wombat status : sm.dorc.W:@ok ;}\n"
                    .to_owned(),
            ],
            FormRequest::Auto,
        );

        for image in [&alone, &beside] {
            let root = image.roots().first().expect("every image has a root");
            assert_eq!(root.id().get(), 0);
            assert_eq!(
                root.entry(),
                image.entrypoints()[0],
                "root 0 materializes the entrypoint"
            );
            let entry = image
                .entries()
                .iter()
                .find(|entry| entry.id() == root.entry())
                .expect("the root names a published entry");
            assert_eq!(
                entry.path().map(|path| path.text().to_owned()),
                Some("plan.sh".to_owned()),
                "root 0 is the plan projection under both shapes"
            );
        }
        assert!(
            beside.roots().len() > alone.roots().len(),
            "the authored load's root is APPENDED rather than replacing the plan's"
        );
    }

    /// A multipart set records its bundle as a file the plan loads.
    #[test]
    fn a_bundled_dependency_is_an_entry_the_plan_reaches() {
        let image = image_over(
            ". ./wombat.oracle.sh\nwombat sync a.conf\n",
            vec!["wombat.oracle.sh".to_owned()],
            vec![
                "# dorc-lang/v0.2\nwombat__is_converged() { wombat status : sm.dorc.W:@ok ;}\n"
                    .to_owned(),
            ],
            FormRequest::Auto,
        );
        assert_eq!(image.form(), RecordedArtifactForm::Multipart);
        assert_eq!(image.entries().len(), 2);
        assert_eq!(
            edges_of(&image),
            vec![(
                "plan.sh".to_owned(),
                "wombat.oracle.dorc-bundle.sh".to_owned()
            )],
            "the plan's re-said import is what reaches the bundle"
        );
    }

    /// THE CASE THE FLAT SHORTCUT GETS WRONG: one dependency sourcing another, mirrored.
    ///
    /// Every file stands at its own authored path here, so `inner` is reached by `outer` and NOT
    /// by the plan. An edge set that said `plan.sh loads inner.oracle.sh` would validate cleanly,
    /// read plausibly, and be false about what the apply does — inside a container whose entire
    /// promise is reproducing exactly what the apply uses. Nothing in the corpus exercises this
    /// shape, which is why it is built here rather than borrowed.
    #[test]
    fn a_dependency_sourcing_another_records_the_inner_reach_and_not_a_flat_one() {
        let image = image_over(
            ". ./outer.oracle.sh\nwombat sync a.conf\n",
            vec!["outer.oracle.sh".to_owned(), "inner.oracle.sh".to_owned()],
            vec![
                "# dorc-lang/v0.2\n. ./inner.oracle.sh\n".to_owned(),
                "# dorc-lang/v0.2\nwombat__is_converged() { wombat status : sm.dorc.W:@ok ;}\n"
                    .to_owned(),
            ],
            FormRequest::Explicit(ArtifactForm::MirroredTree),
        );
        assert_eq!(image.form(), RecordedArtifactForm::MirroredTree);
        // In the container's canonical endpoint order, which puts the plan's own reach first
        // because the plan is entry 0.
        assert_eq!(
            edges_of(&image),
            vec![
                ("plan.sh".to_owned(), "outer.oracle.sh".to_owned()),
                ("outer.oracle.sh".to_owned(), "inner.oracle.sh".to_owned()),
            ],
            "the inner file's loader is the file that sources it"
        );
        assert!(
            !edges_of(&image).contains(&("plan.sh".to_owned(), "inner.oracle.sh".to_owned())),
            "the plan does not load what its dependency loads"
        );
    }
}
