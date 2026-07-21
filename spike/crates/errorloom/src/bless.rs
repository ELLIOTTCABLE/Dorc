use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use crate::container::{Case, CaseError};

/// Re-renders a complete case from consumer-owned state.
pub trait CaseRenderer {
    /// The consumer's render failure.
    type Error: fmt::Display;

    /// Re-render `case`'s full transcript.
    ///
    /// # Errors
    /// Returns the consumer's render failure.
    fn render_case(&self, case: &Case) -> Result<String, Self::Error>;
}

/// Read-only repository state needed to reject dirty transcript cases.
pub trait Git {
    /// Working-tree paths that differ from HEAD.
    ///
    /// # Errors
    /// Returns the git implementation's inspection failure.
    fn dirty_paths(&self) -> Result<Vec<PathBuf>, GitError>;
}

/// Git query failure.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GitError(pub String);

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for GitError {}

/// In-memory git state for tests.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FakeGit {
    dirty: Vec<PathBuf>,
}

impl FakeGit {
    /// An empty clean repository.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark a path dirty.
    #[must_use]
    pub fn mark_dirty(mut self, path: impl Into<PathBuf>) -> Self {
        self.dirty.push(path.into());
        self
    }
}

impl Git for FakeGit {
    fn dirty_paths(&self) -> Result<Vec<PathBuf>, GitError> {
        Ok(self.dirty.clone())
    }
}

/// A case file and its current text.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CaseFile {
    path: PathBuf,
    text: String,
}

impl CaseFile {
    /// Bundle a repository-relative path with its text.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
        }
    }

    /// The case path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The case text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Regenerated case text keyed by repository-relative path.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BlessResult {
    regenerated: BTreeMap<PathBuf, String>,
}

impl BlessResult {
    /// Regenerated cases.
    #[must_use]
    pub fn regenerated(&self) -> &BTreeMap<PathBuf, String> {
        &self.regenerated
    }
}

/// Structure regeneration or fixpoint failure.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BlessError {
    /// Git inspection failed.
    Git(GitError),
    /// A transcript is dirty and cannot be structure-regenerated.
    DirtyCase(PathBuf),
    /// The generated catalog is dirty and cannot be structure-regenerated.
    DirtyCatalog(PathBuf),
    /// Case parsing failed.
    Container(CaseError),
    /// The consumer failed to render a case.
    Consumer(String),
    /// Committed cases do not reproduce.
    Fixpoint(Vec<PathBuf>),
}

/// Regenerate structure only when no case transcript is dirty.
///
/// # Errors
/// Returns a dirty-case, parse, git, or consumer-render failure.
pub fn structure_bless<C: CaseRenderer, G: Git>(
    consumer: &C,
    git: &G,
    corpus: &[CaseFile],
    catalog: &Path,
) -> Result<BlessResult, BlessError> {
    let cases: BTreeSet<_> = corpus.iter().map(CaseFile::path).collect();
    let dirty = git.dirty_paths().map_err(BlessError::Git)?;
    if let Some(path) = dirty.iter().find(|path| path.as_path() == catalog) {
        return Err(BlessError::DirtyCatalog(path.clone()));
    }
    if let Some(path) = dirty
        .into_iter()
        .find(|path| cases.contains(path.as_path()))
    {
        return Err(BlessError::DirtyCase(path));
    }
    regenerate(consumer, corpus)
}

#[cfg(test)]
#[expect(
    clippy::items_after_test_module,
    reason = "helpers stay below public API"
)]
mod tests {
    use super::*;

    struct Renderer;
    impl CaseRenderer for Renderer {
        type Error = String;
        fn render_case(&self, case: &Case) -> Result<String, Self::Error> {
            Ok(case.to_text())
        }
    }
    fn corpus() -> Vec<CaseFile> {
        vec![CaseFile::new(
            "cases/a.txt",
            "---\n---\n-- replay --\n$ tool\nok\n",
        )]
    }
    #[test]
    fn structure_bless_requires_clean_catalog_and_cases() {
        let catalog = Path::new("catalog.rs");
        assert!(matches!(
            structure_bless(
                &Renderer,
                &FakeGit::new().mark_dirty("catalog.rs"),
                &corpus(),
                catalog
            ),
            Err(BlessError::DirtyCatalog(_))
        ));
        assert!(matches!(
            structure_bless(
                &Renderer,
                &FakeGit::new().mark_dirty("cases/a.txt"),
                &corpus(),
                catalog
            ),
            Err(BlessError::DirtyCase(_))
        ));
        assert!(
            structure_bless(
                &Renderer,
                &FakeGit::new().mark_dirty("src/layout.rs"),
                &corpus(),
                catalog
            )
            .is_ok()
        );
        assert!(matches!(
            structure_bless(
                &Renderer,
                &FakeGit::new()
                    .mark_dirty("catalog.rs")
                    .mark_dirty("cases/a.txt"),
                &corpus(),
                catalog
            ),
            Err(BlessError::DirtyCatalog(_))
        ));
    }
}

/// Ensure committed cases reproduce exactly.
///
/// # Errors
/// Returns the paths whose current render differs from committed bytes.
pub fn fixpoint_check<C: CaseRenderer>(
    consumer: &C,
    corpus: &[CaseFile],
) -> Result<(), BlessError> {
    let mut drifted = Vec::new();
    for case_file in corpus {
        let case = Case::parse(case_file.text()).map_err(BlessError::Container)?;
        let rendered = consumer
            .render_case(&case)
            .map_err(|error| BlessError::Consumer(error.to_string()))?;
        if rendered != case_file.text() {
            drifted.push(case_file.path().to_owned());
        }
    }
    if drifted.is_empty() {
        Ok(())
    } else {
        Err(BlessError::Fixpoint(drifted))
    }
}

fn regenerate<C: CaseRenderer>(
    consumer: &C,
    corpus: &[CaseFile],
) -> Result<BlessResult, BlessError> {
    let mut regenerated = BTreeMap::new();
    for case_file in corpus {
        let case = Case::parse(case_file.text()).map_err(BlessError::Container)?;
        let text = consumer
            .render_case(&case)
            .map_err(|error| BlessError::Consumer(error.to_string()))?;
        regenerated.insert(case_file.path().to_owned(), text);
    }
    Ok(BlessResult { regenerated })
}
