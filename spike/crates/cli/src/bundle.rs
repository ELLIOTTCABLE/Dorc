//! Pure bundle projection over the loader's complete occurrence account (`30I` step 5b).
//!
//! This module resolves nothing and reads nothing. It pairs already-resolved load occurrences with
//! bytes from the immutable snapshot, strips through the off-ramp's one implementation, and assigns
//! controller-owned artifact names. Plan edits, publication, and locator consumption belong to
//! later stages.

use dorc_aid::diag::Diag;
use dorc_analysis::load::{LoadAccount, LoadOccurrence};
use dorc_core::{Interner, SourceFileId};
use std::fmt::Write as _;

use crate::snapshot::StaticLoadSnapshot;

const ORIGIN_COMMENT_CAP: usize = 1024;

/// An index into a [`BundleProjection`]'s generated files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BundleFileId(u32);

impl BundleFileId {
    #[must_use]
    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

/// An index into a [`BundleProjection`]'s occurrence-keyed roots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BundleRootId(u32);

impl BundleRootId {
    #[must_use]
    const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Exact stripped bytes copied from one source, with their original line identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopiedSegment {
    source: SourceFileId,
    text: String,
    line_map: Vec<u32>,
}

impl CopiedSegment {
    /// The immutable snapshot source these bytes came from.
    #[must_use]
    pub const fn source(&self) -> SourceFileId {
        self.source
    }

    /// Exact stripped source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// One original 1-based line for each stripped output line.
    #[must_use]
    pub fn line_map(&self) -> &[u32] {
        &self.line_map
    }
}

/// One generated dependency file. Its name is controller-owned and independent of source bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleFile {
    id: BundleFileId,
    storage_path: String,
    occurrence: usize,
    origin_comment: String,
    copied: CopiedSegment,
}

impl BundleFile {
    /// This file's projection-local identity.
    #[must_use]
    pub const fn id(&self) -> BundleFileId {
        self.id
    }

    /// Safe controller-generated archive path, not a runtime `.` target.
    ///
    /// Placement must also consult the corresponding [`ProjectedOccurrence`]: authored source
    /// operands resolve against runtime cwd, not against this storage path.
    #[must_use]
    pub fn storage_path(&self) -> &str {
        &self.storage_path
    }

    /// Index of the load occurrence represented by this file.
    #[must_use]
    pub const fn occurrence(&self) -> usize {
        self.occurrence
    }

    /// Exact stripped segment and retained line map.
    #[must_use]
    pub const fn copied(&self) -> &CopiedSegment {
        &self.copied
    }

    /// Render this file, optionally surrounding the exact copied segment with origin comments.
    #[must_use]
    pub fn render_sh(&self, origin_comments: bool) -> String {
        if !origin_comments {
            return self.copied.text.clone();
        }
        let mut out = format!("# dorc-bundle/v0: begin source={}\n", self.origin_comment);
        out.push_str(&self.copied.text);
        if !out.ends_with('\n') {
            out.push('\n');
        }
        let _ = writeln!(out, "# dorc-bundle/v0: end source={}", self.origin_comment);
        out
    }

    /// Byte offset where the exact copied segment begins in [`Self::render_sh`].
    #[must_use]
    pub(crate) fn copied_offset(&self, origin_comments: bool) -> usize {
        if origin_comments {
            format!("# dorc-bundle/v0: begin source={}\n", self.origin_comment).len()
        } else {
            0
        }
    }
}

/// One projected occurrence, retaining the loader's complete identity without flattening it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectedOccurrence {
    load: LoadOccurrence,
    file: BundleFileId,
    root: BundleRootId,
}

impl ProjectedOccurrence {
    /// The full loader-owned occurrence identity.
    #[must_use]
    pub const fn load(&self) -> &LoadOccurrence {
        &self.load
    }

    /// Generated file that carries this occurrence's target.
    #[must_use]
    pub const fn file(&self) -> BundleFileId {
        self.file
    }

    /// Root bundle this occurrence belongs to.
    #[must_use]
    pub const fn root(&self) -> BundleRootId {
        self.root
    }
}

/// One root load occurrence and every separate generated file nested beneath it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleRoot {
    id: BundleRootId,
    occurrence: usize,
    entry: BundleFileId,
    files: Vec<BundleFileId>,
}

impl BundleRoot {
    /// This root's projection-local identity.
    #[must_use]
    pub const fn id(&self) -> BundleRootId {
        self.id
    }

    /// Index of the loader occurrence that created this root.
    #[must_use]
    pub const fn occurrence(&self) -> usize {
        self.occurrence
    }

    /// Generated entry file for this root.
    #[must_use]
    pub const fn entry(&self) -> BundleFileId {
        self.entry
    }

    /// Entry and nested files in loader occurrence order.
    #[must_use]
    pub fn files(&self) -> &[BundleFileId] {
        &self.files
    }
}

/// The one deterministic bundle/dependency value consumed by explicit and multipart surfaces.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BundleProjection {
    occurrences: Vec<ProjectedOccurrence>,
    roots: Vec<BundleRoot>,
    files: Vec<BundleFile>,
}

/// A strip diagnostic kept with the immutable source it describes.
#[derive(Debug)]
pub struct BundleDiagnostic {
    file: BundleFileId,
    source: SourceFileId,
    diag: Diag,
}

impl BundleDiagnostic {
    /// Generated occurrence file whose copy produced this diagnostic.
    #[must_use]
    pub const fn file(&self) -> BundleFileId {
        self.file
    }

    /// Snapshot source the diagnostic's span belongs to.
    #[must_use]
    pub const fn source(&self) -> SourceFileId {
        self.source
    }

    /// Existing strip diagnostic, unchanged.
    #[must_use]
    pub const fn diag(&self) -> &Diag {
        &self.diag
    }
}

/// Projection value plus source-attributed diagnostics from the shared strip seat.
#[derive(Debug)]
pub struct BundleProjectionOutput {
    projection: BundleProjection,
    diagnostics: Vec<BundleDiagnostic>,
}

impl BundleProjectionOutput {
    /// The complete pure bundle/dependency value.
    #[must_use]
    pub const fn projection(&self) -> &BundleProjection {
        &self.projection
    }

    /// Strip diagnostics with their original source identities.
    #[must_use]
    pub fn diagnostics(&self) -> &[BundleDiagnostic] {
        &self.diagnostics
    }

    /// Consume the wrapper after edge reporting.
    #[must_use]
    pub fn into_projection(self) -> BundleProjection {
        self.projection
    }
}

impl BundleProjection {
    /// Every loader occurrence, with no pair-set collapse.
    #[must_use]
    pub fn occurrences(&self) -> &[ProjectedOccurrence] {
        &self.occurrences
    }

    /// Every distinct root load occurrence.
    #[must_use]
    pub fn roots(&self) -> &[BundleRoot] {
        &self.roots
    }

    /// All generated storage entries, in occurrence order.
    ///
    /// This is not by itself a materialization recipe. The occurrence account supplies the
    /// authored target and positional context that later placement must preserve.
    #[must_use]
    pub fn files(&self) -> &[BundleFile] {
        &self.files
    }

    /// Resolve one projection-local generated file identity.
    #[must_use]
    pub fn file(&self, id: BundleFileId) -> Option<&BundleFile> {
        self.files.get(id.index())
    }

    /// A deterministic, inert stdout archive of the same multipart value.
    ///
    /// Each shell body is quoted as here-document data. Executing or sourcing this inspection
    /// form therefore cannot flatten the separate source boundaries by accident.
    #[must_use]
    pub fn render_archive(&self) -> String {
        if self.roots.is_empty() {
            return String::new();
        }
        let mut out = String::from("# dorc-bundle-set/v0\n");
        for root in &self.roots {
            let _ = writeln!(
                out,
                "# dorc-bundle-root/v0: begin occurrence={}",
                root.occurrence
            );
            for &file in &root.files {
                let Some(file) = self.file(file) else {
                    continue;
                };
                let rendered = file.render_sh(true);
                let delimiter = archive_delimiter(file.id, &rendered);
                let _ = writeln!(
                    out,
                    "# dorc-bundle-file/v0: begin storage-path={}",
                    file.storage_path
                );
                let _ = writeln!(out, ": <<'{delimiter}'");
                out.push_str(&rendered);
                let _ = writeln!(out, "{delimiter}");
                let _ = writeln!(
                    out,
                    "# dorc-bundle-file/v0: end storage-path={}",
                    file.storage_path
                );
            }
            let _ = writeln!(
                out,
                "# dorc-bundle-root/v0: end occurrence={}",
                root.occurrence
            );
        }
        out
    }
}

/// A load account and snapshot that cannot describe one closed occurrence forest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundleProjectionError {
    /// A resolved occurrence target has no source in the immutable snapshot.
    MissingSource {
        /// Offending occurrence index.
        occurrence: usize,
    },
    /// A `within` edge does not point backward into the same occurrence account.
    InvalidParent {
        /// Offending occurrence index.
        occurrence: usize,
    },
    /// The projection cannot assign a collision-free fixed-width identity.
    IdentityOverflow,
}

/// Project every possible load occurrence without resolving or reading anything again.
///
/// # Errors
/// Returns a closed structural error when the account is not a forest over sources in the snapshot,
/// or when the occurrence population cannot be represented by the fixed-width identities.
pub fn project(
    snapshot: &StaticLoadSnapshot,
    account: &LoadAccount,
) -> Result<BundleProjectionOutput, BundleProjectionError> {
    let loads = account.occurrences();
    let mut root_occurrences = Vec::with_capacity(loads.len());
    for occurrence in 0..loads.len() {
        root_occurrences.push(root_of(occurrence, loads)?);
    }
    let mut root_ids = std::collections::BTreeMap::new();
    for &root in &root_occurrences {
        if !root_ids.contains_key(&root) {
            let next = BundleRootId(
                u32::try_from(root_ids.len())
                    .map_err(|_| BundleProjectionError::IdentityOverflow)?,
            );
            root_ids.insert(root, next);
        }
    }

    let mut diagnostics = Vec::new();
    let mut files = Vec::with_capacity(loads.len());
    let mut occurrences = Vec::with_capacity(loads.len());
    for (occurrence, load) in loads.iter().enumerate() {
        let source = snapshot
            .source_at_key(&load.target)
            .ok_or(BundleProjectionError::MissingSource { occurrence })?;
        let src = snapshot
            .source_srcs()
            .get(source)
            .ok_or(BundleProjectionError::MissingSource { occurrence })?;
        let mapped = dorc_oracle::strip::strip_file_with_map(&mut Interner::default(), src);
        let source_id = dorc_analysis::funcenv::source_file_of_index(source);
        let root_occurrence = *root_occurrences
            .get(occurrence)
            .ok_or(BundleProjectionError::InvalidParent { occurrence })?;
        let root = *root_ids
            .get(&root_occurrence)
            .ok_or(BundleProjectionError::InvalidParent { occurrence })?;
        let id = BundleFileId(
            u32::try_from(files.len()).map_err(|_| BundleProjectionError::IdentityOverflow)?,
        );
        diagnostics.extend(mapped.diags.into_iter().map(|diag| BundleDiagnostic {
            file: id,
            source: source_id,
            diag,
        }));
        let storage_path = format!(
            "dorc-bundle/v0/root-{:08}/occurrence-{occurrence:08}.sh",
            root.index()
        );
        let origin = snapshot
            .source_paths()
            .get(source)
            .map_or("", String::as_str);
        files.push(BundleFile {
            id,
            storage_path,
            occurrence,
            origin_comment: dorc_aid::display::encode_shell_comment(origin, ORIGIN_COMMENT_CAP),
            copied: CopiedSegment {
                source: source_id,
                text: mapped.value.text,
                line_map: mapped.value.line_map,
            },
        });
        occurrences.push(ProjectedOccurrence {
            load: load.clone(),
            file: id,
            root,
        });
    }

    let mut roots: Vec<BundleRoot> = root_ids
        .iter()
        .map(|(&occurrence, &id)| {
            let entry = occurrences
                .get(occurrence)
                .map(ProjectedOccurrence::file)
                .ok_or(BundleProjectionError::InvalidParent { occurrence })?;
            Ok(BundleRoot {
                id,
                occurrence,
                entry,
                files: Vec::new(),
            })
        })
        .collect::<Result<_, _>>()?;
    roots.sort_by_key(BundleRoot::id);
    for (occurrence, projected) in occurrences.iter().enumerate() {
        roots
            .get_mut(projected.root.index())
            .ok_or(BundleProjectionError::InvalidParent { occurrence })?
            .files
            .push(projected.file);
    }
    Ok(BundleProjectionOutput {
        projection: BundleProjection {
            occurrences,
            roots,
            files,
        },
        diagnostics,
    })
}

fn root_of(occurrence: usize, loads: &[LoadOccurrence]) -> Result<usize, BundleProjectionError> {
    let mut current = occurrence;
    for _ in 0..=loads.len() {
        let Some(load) = loads.get(current) else {
            return Err(BundleProjectionError::InvalidParent { occurrence });
        };
        match load.within {
            None => return Ok(current),
            Some(parent) if parent < current => current = parent,
            Some(_) => return Err(BundleProjectionError::InvalidParent { occurrence }),
        }
    }
    Err(BundleProjectionError::InvalidParent { occurrence })
}

fn archive_delimiter(id: BundleFileId, rendered: &str) -> String {
    let mut candidate = format!("DORC_BUNDLE_FILE_{:08}_00000000", id.index());
    while rendered.lines().any(|line| line == candidate) {
        candidate.push('_');
    }
    candidate
}

#[cfg(test)]
mod tests {
    use dorc_analysis::load::{LoadRoute, LoadSourcer};
    use dorc_core::loadpath::Cwd;

    use super::{BundleProjection, project};
    use crate::snapshot::{StaticLoadSnapshot, book_reached};

    const MARKER: &str = "# dorc-lang/v0.2\n";

    fn marked(body: &str) -> String {
        format!("{MARKER}{body}")
    }

    fn projection(book: &str, paths: Vec<String>, srcs: Vec<String>) -> BundleProjection {
        let cwd = Cwd::default();
        let reached = book_reached(&cwd, &paths, &srcs, book);
        let snapshot = StaticLoadSnapshot::over(cwd, paths, srcs, &reached, "book.sh", book);
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut interner = dorc_core::Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = crate::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        project(&snapshot, env.loads())
            .expect("one closed occurrence forest")
            .into_projection()
    }

    #[test]
    fn two_load_points_naming_one_entrypoint_are_two_roots() {
        let bundle = projection(
            ". ./entry.sh\n. ./entry.sh\n",
            vec!["entry.sh".to_owned()],
            vec![marked("entry() { :; }\n")],
        );
        assert_eq!(bundle.roots().len(), 2);
        assert_eq!(bundle.occurrences().len(), 2);
        assert_ne!(bundle.roots()[0].entry(), bundle.roots()[1].entry());
    }

    #[test]
    fn speculative_nested_fallback_is_included() {
        let entry =
            marked("if command -v _same >/dev/null 2>&1; then\n   :\nelse\n   . ./dep.sh\nfi\n");
        let bundle = projection(
            ". ./entry.sh\n",
            vec!["entry.sh".to_owned(), "dep.sh".to_owned()],
            vec![entry, marked("_same() { :; }\n")],
        );
        let nested = bundle
            .occurrences()
            .iter()
            .find(|occurrence| occurrence.load().target == "dep.sh")
            .expect("the speculative dependency is retained");
        assert_eq!(nested.load().route, LoadRoute::Speculative);
        assert_eq!(
            nested.load().sourcer,
            LoadSourcer::File("entry.sh".to_owned())
        );
        assert!(nested.load().locus.is_some());
        assert_eq!(nested.load().within, Some(0));
        assert_eq!(nested.load().at, bundle.occurrences()[0].load().at);
    }

    #[test]
    fn diamond_dependencies_remain_complete_per_root() {
        let bundle = projection(
            ". ./a.sh\n. ./b.sh\n",
            vec!["a.sh".to_owned(), "b.sh".to_owned(), "shared.sh".to_owned()],
            vec![
                marked(". ./shared.sh\na() { :; }\n"),
                marked(". ./shared.sh\nb() { :; }\n"),
                marked("shared() { :; }\n"),
            ],
        );
        assert_eq!(bundle.roots().len(), 2);
        assert_eq!(bundle.roots()[0].files().len(), 2);
        assert_eq!(bundle.roots()[1].files().len(), 2);
        for root in bundle.roots() {
            assert!(
                bundle
                    .file(root.entry())
                    .is_some_and(|file| file.copied().text().contains(". ./shared.sh")),
                "the nested dot remains inside its own generated source boundary"
            );
        }
        assert_eq!(
            bundle
                .occurrences()
                .iter()
                .filter(|occurrence| occurrence.load().target == "shared.sh")
                .count(),
            2
        );
    }

    #[test]
    fn archive_quotes_bodies_instead_of_flattening_them() {
        let bundle = projection(
            ". ./entry.sh\n",
            vec!["entry.sh".to_owned(), "shared.sh".to_owned()],
            vec![
                marked(". ./shared.sh\nentry() { :; }\n"),
                marked("shared() { :; }\n"),
            ],
        );
        let archive = bundle.render_archive();
        let opening = archive
            .find(": <<'DORC_BUNDLE_FILE_00000000_00000000'\n")
            .expect("the entry body is quoted");
        let authored_dot = archive
            .find("\n. ./shared.sh\n")
            .expect("the authored source remains exact");
        let closing = archive
            .find("\nDORC_BUNDLE_FILE_00000000_00000000\n")
            .expect("the quoted entry body ends");
        assert!(opening < authored_dot && authored_dot < closing);
    }

    #[test]
    fn archive_delimiters_cannot_be_injected_by_source_bytes() {
        let bundle = projection(
            "",
            vec!["entry.sh".to_owned()],
            vec![marked("DORC_BUNDLE_FILE_00000000_00000000\n")],
        );
        let archive = bundle.render_archive();
        assert!(archive.contains(": <<'DORC_BUNDLE_FILE_00000000_00000000_'\n"));
    }

    #[test]
    fn copied_bytes_and_line_map_come_from_the_strip_seat() {
        let src = marked("thing__is_converged() {\n   thing status : sm.dorc.Thing:@ready\n}\n");
        let bundle = projection("", vec!["thing.sh".to_owned()], vec![src.clone()]);
        let direct =
            dorc_oracle::strip::strip_file_with_map(&mut dorc_core::Interner::default(), &src)
                .value;
        let copied = bundle.files()[0].copied();
        assert_eq!(copied.text(), direct.text);
        assert_eq!(copied.line_map(), direct.line_map);
        assert_eq!(copied.line_map().first(), Some(&2));
        assert_eq!(bundle.files()[0].render_sh(false), direct.text);
    }

    #[test]
    fn generated_names_and_order_ignore_snapshot_storage_order() {
        let book = ". ./a.sh\n. ./b.sh\n";
        let a = marked("a() { :; }\n");
        let b = marked("b() { :; }\n");
        let forward = projection(
            book,
            vec!["a.sh".to_owned(), "b.sh".to_owned()],
            vec![a.clone(), b.clone()],
        );
        let reversed = projection(book, vec!["b.sh".to_owned(), "a.sh".to_owned()], vec![b, a]);
        assert_eq!(forward.render_archive(), reversed.render_archive());
    }

    #[test]
    fn hostile_origin_text_cannot_choose_a_path_or_open_a_comment_line() {
        let path = "../../evil\n# forged @@dorc@@.sh".to_owned();
        let bundle = projection("", vec![path], vec![marked("safe() { :; }\n")]);
        let archive = bundle.render_archive();
        assert!(bundle.files().iter().all(|file| {
            !file.storage_path().contains("..")
                && !file.storage_path().contains(['\n', '\r'])
                && file.storage_path().starts_with("dorc-bundle/v0/")
        }));
        assert!(!archive.contains("\n# forged"));
        assert!(archive.contains("\\x0a# forged"));
        assert!(!archive.lines().any(|line| line.starts_with("dorc site ")));
    }

    #[test]
    fn no_loads_project_to_no_bytes() {
        let bundle = projection("", Vec::new(), Vec::new());
        assert!(bundle.roots().is_empty());
        assert!(bundle.files().is_empty());
        assert_eq!(bundle.render_archive(), "");
    }

    #[test]
    fn the_projection_module_cannot_read_or_resolve_again() {
        let source = include_str!("bundle.rs");
        let body = source
            .split_once("mod tests {")
            .map_or(source, |(head, _)| head);
        for forbidden in [
            "std::fs",
            "read_to_string",
            "resolve_dot",
            "resolve_operand",
        ] {
            assert!(
                !body.contains(forbidden),
                "bundle projection names `{forbidden}`"
            );
        }
    }
}
