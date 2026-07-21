//! Bless orchestration (`282` §6 / `28A` §1) — the product's heart: git-driven
//! mode inference, the prose-bless / structure-bless exclusivity (never both),
//! the baseline-verify that enforces it, and the CI fixpoint gate.
//!
//! Two abstractions, and only two (`28A` §1, "concrete over abstract"): a
//! [`Consumer`] (baseline tagged render · apply field-edits · re-render a case)
//! and a two-method [`Git`] (`head_version_of` · `dirty_paths`). errorloom drives
//! the loop over them; the catalog, tagged-render emission, and case schema stay
//! consumer-side.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ConsumerKey;
use crate::container::{Case, CaseError};
use crate::promote::{ParamTables, Refusal, promote};
use crate::prose::FieldTemplate;
use crate::span::TaggedRender;

/// A consumer's baseline for prose-bless: the tagged render plus the param tables
/// promote needs to re-hole. Produced from CURRENT catalog state (`282` §6).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TaggedBaseline<K> {
    render: TaggedRender<K>,
    params: ParamTables<K>,
}

impl<K> TaggedBaseline<K> {
    /// Bundle a tagged render with its param tables.
    #[must_use]
    pub fn new(render: TaggedRender<K>, params: ParamTables<K>) -> Self {
        TaggedBaseline { render, params }
    }

    /// The tagged render.
    #[must_use]
    pub fn render(&self) -> &TaggedRender<K> {
        &self.render
    }

    /// The param tables.
    #[must_use]
    pub fn params(&self) -> &ParamTables<K> {
        &self.params
    }
}

/// What generic case regeneration requires (`tc-phase-two-promotion-continuity`).
pub trait CaseRenderer {
    /// The consumer's error, surfaced through [`BlessError::Consumer`].
    type Error: fmt::Display;

    /// Re-render `case`'s full transcript text from current state.
    ///
    /// # Errors
    /// Any renderer-side failure regenerating the case.
    fn render_case(&self, case: &Case) -> Result<String, Self::Error>;
}

/// What prose promotion additionally requires. The catalog and case schema live
/// entirely behind these methods.
pub trait Consumer: CaseRenderer {
    /// The consumer's opaque field key (Dorc: `(code, field)`).
    type Key: ConsumerKey;

    /// The tagged render of `case`'s editable transcript from CURRENT catalog
    /// state — the prose-bless baseline whose span map is the attribution
    /// authority (`282` §5).
    ///
    /// # Errors
    /// Any consumer-side failure producing the render.
    fn tagged_render(&self, case: &Case) -> Result<TaggedBaseline<Self::Key>, Self::Error>;

    /// The editable transcript text as it currently stands in `case` (the prose
    /// surface the author edits — the on-disk bytes, not a fresh render).
    ///
    /// # Errors
    /// Any consumer-side failure extracting the text.
    fn editable_text(&self, case: &Case) -> Result<String, Self::Error>;

    /// Apply extracted field-edits into the consumer's catalog. Taken BY VALUE
    /// (`taste-F4`): the caller owns and drops the map, so an impl may move each
    /// [`FieldTemplate`] into its catalog without cloning.
    ///
    /// # Errors
    /// Any consumer-side failure writing the catalog.
    fn apply_field_edits(
        &mut self,
        edits: BTreeMap<Self::Key, FieldTemplate>,
    ) -> Result<(), Self::Error>;
}

/// A two-method git façade (`282:rul-git-repo-dependence-accepted`): just enough
/// to read HEAD and detect a dirty tree. The trait IS the gix swap seam.
pub trait Git {
    /// The committed content of `path` at HEAD, or `None` if it is not tracked.
    ///
    /// # Errors
    /// [`GitError`] if git cannot be run or its output is not UTF-8.
    fn head_version_of(&self, path: &Path) -> Result<Option<String>, GitError>;

    /// The working-tree paths that differ from HEAD (modified, added, untracked).
    ///
    /// # Errors
    /// [`GitError`] if git cannot be run or its output is not UTF-8.
    fn dirty_paths(&self) -> Result<Vec<PathBuf>, GitError>;
}

/// Why a git query failed (`282` §6). Blunt.
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum GitError {
    /// git could not be spawned.
    Spawn(String),
    /// git ran but its output was not UTF-8.
    NonUtf8,
    /// git ran but exited nonzero for a reason that is NOT a legitimate
    /// path-not-in-HEAD signal (`swe-F2`): a genuine failure, no longer conflated
    /// with an untracked file.
    NonZeroExit {
        /// git's captured stderr.
        stderr: String,
    },
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::Spawn(message) => write!(f, "git: cannot run: {message}"),
            GitError::NonUtf8 => f.write_str("git: non-UTF-8 output"),
            GitError::NonZeroExit { stderr } => write!(f, "git: exited nonzero: {}", stderr.trim()),
        }
    }
}

impl std::error::Error for GitError {}

/// The subprocess-`git` implementation (`282` §6): every real host has the binary,
/// and this dodges the heavy gix/git2 license surface.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SubprocessGit {
    repo: PathBuf,
}

impl SubprocessGit {
    /// A git façade rooted at `repo` (the working directory git runs in).
    pub fn new(repo: impl Into<PathBuf>) -> Self {
        SubprocessGit { repo: repo.into() }
    }
}

impl Git for SubprocessGit {
    fn head_version_of(&self, path: &Path) -> Result<Option<String>, GitError> {
        let rel = path.to_string_lossy().replace('\\', "/");
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.repo)
            .arg("show")
            .arg(format!("HEAD:{rel}"))
            .output()
            .map_err(|e| GitError::Spawn(e.to_string()))?;
        if !output.status.success() {
            // `git show HEAD:<path>` exits nonzero BOTH for a path absent from
            // HEAD (the legitimate untracked signal → None) and for a real git
            // failure; only the former carries git's tree-lookup phrasing.
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("does not exist in") || stderr.contains("exists on disk, but not in")
            {
                return Ok(None);
            }
            return Err(GitError::NonZeroExit {
                stderr: stderr.into_owned(),
            });
        }
        String::from_utf8(output.stdout)
            .map(Some)
            .map_err(|_| GitError::NonUtf8)
    }

    fn dirty_paths(&self) -> Result<Vec<PathBuf>, GitError> {
        let output = Command::new("git")
            .arg("-C")
            .arg(&self.repo)
            .arg("status")
            .arg("--porcelain")
            .output()
            .map_err(|e| GitError::Spawn(e.to_string()))?;
        if !output.status.success() {
            return Err(GitError::NonZeroExit {
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }
        let text = String::from_utf8(output.stdout).map_err(|_| GitError::NonUtf8)?;
        let mut paths: Vec<PathBuf> = Vec::new();
        for line in text.lines() {
            let Some(rest) = line.get(3..) else { continue };
            let rest = rest.rsplit(" -> ").next().unwrap_or(rest);
            let rest = rest.trim().trim_matches('"');
            if !rest.is_empty() {
                paths.push(PathBuf::from(rest));
            }
        }
        Ok(paths)
    }
}

/// An in-memory [`Git`] for tests (`282` §6): committed HEAD contents plus a
/// dirty-path set, with no subprocess.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FakeGit {
    head: BTreeMap<PathBuf, String>,
    dirty: Vec<PathBuf>,
}

impl FakeGit {
    /// An empty repo (nothing committed, nothing dirty).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record `path`'s committed HEAD content.
    #[must_use]
    pub fn commit(mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        self.head.insert(path.into(), text.into());
        self
    }

    /// Mark `path` dirty in the working tree.
    #[must_use]
    pub fn mark_dirty(mut self, path: impl Into<PathBuf>) -> Self {
        self.dirty.push(path.into());
        self
    }
}

impl Git for FakeGit {
    fn head_version_of(&self, path: &Path) -> Result<Option<String>, GitError> {
        Ok(self.head.get(path).cloned())
    }

    fn dirty_paths(&self) -> Result<Vec<PathBuf>, GitError> {
        Ok(self.dirty.clone())
    }
}

/// Which bless is legal for the current touched-set (`282` §6).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlessMode {
    /// Only case files are dirty and the catalog is clean.
    Prose,
    /// Only code/arrangement is dirty (case prose untouched).
    Structure,
}

/// Why mode inference or a bless-mode call refuses (`282` §6).
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum ModeRefusal {
    /// The generated catalog itself was hand-edited (dirty).
    DirtyCatalog,
    /// Both case-prose and code/arrangement changed — never both in one bless.
    BothClasses,
    /// `prose_bless` was called but the touched-set is structure-only.
    NotProse,
    /// `structure_bless` was called but the touched-set is prose-only.
    NotStructure,
}

impl fmt::Display for ModeRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModeRefusal::DirtyCatalog => {
                f.write_str("the generated catalog is dirty (hand-edited)")
            }
            ModeRefusal::BothClasses => {
                f.write_str("both case prose and code changed — never both in one bless")
            }
            ModeRefusal::NotProse => {
                f.write_str("touched-set is structure-only; structure-bless it")
            }
            ModeRefusal::NotStructure => f.write_str("touched-set is prose-only; prose-bless it"),
        }
    }
}

/// Classify the touched-set (`282` §6). `catalog` and every `cases` entry must be
/// in git's repo-relative path form.
///
/// # Errors
/// [`BlessError::Git`] on a git failure, or [`BlessError::Mode`] with
/// [`ModeRefusal::DirtyCatalog`] / [`ModeRefusal::BothClasses`].
pub fn infer_mode<G: Git, K>(
    git: &G,
    catalog: &Path,
    cases: &[PathBuf],
) -> Result<BlessMode, BlessError<K>> {
    let dirty = git.dirty_paths().map_err(BlessError::Git)?;
    let case_set: BTreeSet<&Path> = cases.iter().map(PathBuf::as_path).collect();
    let mut catalog_dirty = false;
    let mut case_dirty = false;
    let mut code_dirty = false;
    for path in &dirty {
        if path.as_path() == catalog {
            catalog_dirty = true;
        } else if case_set.contains(path.as_path()) {
            case_dirty = true;
        } else {
            code_dirty = true;
        }
    }
    if catalog_dirty {
        return Err(BlessError::Mode(ModeRefusal::DirtyCatalog));
    }
    match (case_dirty, code_dirty) {
        (true, true) => Err(BlessError::Mode(ModeRefusal::BothClasses)),
        (true, false) => Ok(BlessMode::Prose),
        (false, _) => Ok(BlessMode::Structure),
    }
}

/// A case file: its repo-relative path and current on-disk text.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CaseFile {
    path: PathBuf,
    text: String,
}

impl CaseFile {
    /// Bundle a path with its current text.
    pub fn new(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        CaseFile {
            path: path.into(),
            text: text.into(),
        }
    }

    /// The repo-relative path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The current on-disk text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// The regenerated case texts a bless produced, keyed by path (`282` §6). The
/// caller overwrites the corpus with these; the review surface is the git diff.
#[derive(Clone, PartialEq, Eq, Debug)]
#[must_use = "a bless result holds the regenerated case texts to write"]
pub struct BlessResult {
    regenerated: BTreeMap<PathBuf, String>,
}

impl BlessResult {
    /// The regenerated case texts, keyed by path.
    #[must_use]
    pub fn regenerated(&self) -> &BTreeMap<PathBuf, String> {
        &self.regenerated
    }
}

/// Why a bless failed (`282` §6). Blunt (`282:rul-internal-tool-sharp-edges`).
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum BlessError<K> {
    /// Mode inference refused.
    Mode(ModeRefusal),
    /// A git query failed.
    Git(GitError),
    /// The consumer returned an error (rendered).
    Consumer(String),
    /// A case failed to parse.
    Container(CaseError),
    /// A prose extraction refused.
    Refusal(Refusal<K>),
    /// The current re-render's structure diverged from HEAD — structure-bless
    /// first (the never-both law, enforced by the baseline-verify).
    StructureDrift {
        /// The case whose structure drifted.
        case: PathBuf,
    },
    /// A dirty case had no HEAD version to verify against.
    MissingHeadVersion {
        /// The case with no HEAD version.
        case: PathBuf,
    },
    /// The fixpoint gate found cases whose render no longer reproduces the commit.
    Fixpoint {
        /// The drifted case paths.
        drifted: Vec<PathBuf>,
    },
}

impl<K: fmt::Debug> fmt::Display for BlessError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlessError::Mode(refusal) => write!(f, "bless: {refusal}"),
            BlessError::Git(inner) => write!(f, "bless: {inner}"),
            BlessError::Consumer(message) => write!(f, "bless: consumer: {message}"),
            BlessError::Container(inner) => write!(f, "bless: {inner}"),
            BlessError::Refusal(refusal) => write!(f, "bless: {refusal}"),
            BlessError::StructureDrift { case } => {
                write!(
                    f,
                    "bless: {} drifted structurally — structure-bless first",
                    case.display()
                )
            }
            BlessError::MissingHeadVersion { case } => {
                write!(
                    f,
                    "bless: {} has no HEAD version to verify against",
                    case.display()
                )
            }
            BlessError::Fixpoint { drifted } => {
                write!(
                    f,
                    "bless: fixpoint gate: {} case(s) do not reproduce",
                    drifted.len()
                )
            }
        }
    }
}

impl<K: fmt::Debug> std::error::Error for BlessError<K> {}

/// Prose-bless (promote): extract the author's prose edits, regenerate the
/// catalog, and re-render the corpus (`282` §6). Requires prose-mode + a clean
/// structure (the baseline-verify IS the never-both law).
///
/// # Errors
/// [`BlessError`] for a wrong mode, a git failure, a parse failure, a structure
/// drift (structure-bless first), a promote refusal, or a consumer error.
pub fn prose_bless<C: Consumer, G: Git>(
    consumer: &mut C,
    git: &G,
    corpus: &[CaseFile],
    catalog: &Path,
) -> Result<BlessResult, BlessError<C::Key>> {
    let case_paths: Vec<PathBuf> = corpus.iter().map(|c| c.path.clone()).collect();
    if infer_mode(git, catalog, &case_paths)? != BlessMode::Prose {
        return Err(BlessError::Mode(ModeRefusal::NotProse));
    }
    let dirty = git.dirty_paths().map_err(BlessError::Git)?;
    let dirty_set: BTreeSet<&Path> = dirty.iter().map(PathBuf::as_path).collect();

    let mut edits: BTreeMap<C::Key, FieldTemplate> = BTreeMap::new();
    for case_file in corpus {
        if !dirty_set.contains(case_file.path.as_path()) {
            continue;
        }
        let work_case = Case::parse(&case_file.text).map_err(BlessError::Container)?;
        let head_text = git
            .head_version_of(&case_file.path)
            .map_err(BlessError::Git)?
            .ok_or_else(|| BlessError::MissingHeadVersion {
                case: case_file.path.clone(),
            })?;
        let head_case = Case::parse(&head_text).map_err(BlessError::Container)?;

        let head_editable = consumer.editable_text(&head_case).map_err(consumer_err)?;
        let work_editable = consumer.editable_text(&work_case).map_err(consumer_err)?;
        let baseline = consumer.tagged_render(&work_case).map_err(consumer_err)?;

        if promote(baseline.render(), &head_editable, baseline.params()).is_err() {
            return Err(BlessError::StructureDrift {
                case: case_file.path.clone(),
            });
        }
        let outcome = promote(baseline.render(), &work_editable, baseline.params())
            .map_err(BlessError::Refusal)?;
        for (key, template) in outcome.field_edits() {
            edits.insert(key.clone(), template.clone());
        }
    }

    consumer.apply_field_edits(edits).map_err(consumer_err)?;
    regenerate::<C, C::Key>(consumer, corpus)
}

/// Structure-bless: regenerate every case from the (unchanged) catalog (`282`
/// §6). Requires structure-mode; prose provably cannot drift.
///
/// # Errors
/// [`BlessError`] for a wrong mode, a git failure, a parse failure, or a consumer
/// error.
pub fn structure_bless<C: CaseRenderer, G: Git>(
    consumer: &C,
    git: &G,
    corpus: &[CaseFile],
    catalog: &Path,
) -> Result<BlessResult, BlessError<()>> {
    let case_paths: Vec<PathBuf> = corpus.iter().map(|c| c.path.clone()).collect();
    if infer_mode(git, catalog, &case_paths)? != BlessMode::Structure {
        return Err(BlessError::Mode(ModeRefusal::NotStructure));
    }
    regenerate::<C, ()>(consumer, corpus)
}

/// The CI fixpoint gate (`282` §6): every committed case must re-render to its own
/// committed text. Catches ANY hand-edit of the generated catalog, since a catalog
/// change moves the render away from the committed transcript.
///
/// # Errors
/// [`BlessError::Fixpoint`] with the drifted cases, or a parse / consumer error.
pub fn fixpoint_check<C: CaseRenderer>(
    consumer: &C,
    corpus: &[CaseFile],
) -> Result<(), BlessError<()>> {
    let mut drifted: Vec<PathBuf> = Vec::new();
    for case_file in corpus {
        let case = Case::parse(&case_file.text).map_err(BlessError::Container)?;
        let rendered = consumer.render_case(&case).map_err(consumer_err)?;
        if rendered != case_file.text {
            drifted.push(case_file.path.clone());
        }
    }
    if drifted.is_empty() {
        Ok(())
    } else {
        Err(BlessError::Fixpoint { drifted })
    }
}

fn regenerate<C: CaseRenderer, K>(
    consumer: &C,
    corpus: &[CaseFile],
) -> Result<BlessResult, BlessError<K>> {
    let mut regenerated: BTreeMap<PathBuf, String> = BTreeMap::new();
    for case_file in corpus {
        let case = Case::parse(&case_file.text).map_err(BlessError::Container)?;
        let text = consumer.render_case(&case).map_err(consumer_err)?;
        regenerated.insert(case_file.path.clone(), text);
    }
    Ok(BlessResult { regenerated })
}

fn consumer_err<K, E: fmt::Display>(error: E) -> BlessError<K> {
    BlessError::Consumer(error.to_string())
}
