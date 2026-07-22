//! Read-only repository classification for transcript-prose compilation.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use errorloom::Case;

/// The narrow I/O edge required to classify one repository snapshot.
pub trait Repository {
    /// NUL-delimited `git status --porcelain=v1` records.
    ///
    /// # Errors
    ///
    /// Returns an edge-specific read refusal.
    fn status_porcelain(&self) -> Result<Vec<u8>, String>;
    /// Exact current worktree bytes for one repository-relative path.
    ///
    /// # Errors
    ///
    /// Returns an edge-specific read refusal.
    fn current_bytes(&self, path: &str) -> Result<Vec<u8>, String>;
    /// Exact `HEAD` blob bytes for one repository-relative path.
    ///
    /// # Errors
    ///
    /// Returns an edge-specific read refusal.
    fn head_bytes(&self, path: &str) -> Result<Vec<u8>, String>;
}

/// The selected and accepted prose-touched case paths.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProseClassification {
    selected: Vec<String>,
    touched: BTreeSet<String>,
}

impl ProseClassification {
    /// Canonical selected paths, including clean cases.
    #[must_use]
    pub fn selected(&self) -> &[String] {
        &self.selected
    }

    /// Selected cases with accepted replay-output differences.
    #[must_use]
    pub fn touched(&self) -> &BTreeSet<String> {
        &self.touched
    }
}

/// Read-only production Git/filesystem edge.
#[derive(Clone, Debug)]
pub struct GitRepository {
    root: PathBuf,
}

impl GitRepository {
    /// Locate the repository root through Git rather than guessing from cwd.
    ///
    /// # Errors
    ///
    /// Returns a refusal when Git cannot identify the enclosing repository.
    pub fn open() -> Result<Self, String> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()
            .map_err(|error| format!("locate git repository: {error}"))?;
        if !output.status.success() {
            return Err("dorc-loom requires a git repository".to_owned());
        }
        let root = String::from_utf8(output.stdout)
            .map_err(|_| "git root is not UTF-8".to_owned())?
            .trim()
            .to_owned();
        if root.is_empty() {
            return Err("git root is empty".to_owned());
        }
        Ok(Self {
            root: PathBuf::from(root),
        })
    }

    /// Canonicalize one selected path to a safe slash-normalized repository path.
    ///
    /// # Errors
    ///
    /// Returns a refusal for unreadable, outside-root, or unsafe paths.
    pub fn repository_path(&self, path: &Path) -> Result<String, String> {
        let path = fs::canonicalize(path).map_err(|error| format!("canonicalize case: {error}"))?;
        let root = fs::canonicalize(&self.root)
            .map_err(|error| format!("canonicalize git root: {error}"))?;
        let relative = path
            .strip_prefix(root)
            .map_err(|_| "case is outside git repository".to_owned())?;
        let path = relative.to_string_lossy().replace('\\', "/");
        if !safe_path(&path) {
            return Err("unsafe case path".to_owned());
        }
        Ok(path)
    }
}

impl Repository for GitRepository {
    fn status_porcelain(&self) -> Result<Vec<u8>, String> {
        let output = Command::new("git")
            .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
            .current_dir(&self.root)
            .output()
            .map_err(|error| format!("read git status: {error}"))?;
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err("read git status failed".to_owned())
        }
    }

    fn current_bytes(&self, path: &str) -> Result<Vec<u8>, String> {
        fs::read(self.root.join(path))
            .map_err(|error| format!("read worktree path {path}: {error}"))
    }

    fn head_bytes(&self, path: &str) -> Result<Vec<u8>, String> {
        let output = Command::new("git")
            .args(["cat-file", "blob", &format!("HEAD:{path}")])
            .current_dir(&self.root)
            .output()
            .map_err(|error| format!("read HEAD path {path}: {error}"))?;
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(format!("path is absent from HEAD: {path}"))
        }
    }
}

/// Parse and classify the complete repository snapshot without performing I/O.
///
/// Only selected, worktree-only modified cases may differ, and only within raw
/// replay-output islands. Replay provenance decides whether those islands are
/// editable in the subsequent inspection pass.
///
/// # Errors
///
/// Returns a refusal for malformed Git state, dirty unrelated paths, or any
/// non-output transcript difference.
pub fn classify_prose_changes(
    repository: &impl Repository,
    selected: Vec<String>,
    catalog: &str,
) -> Result<ProseClassification, String> {
    validate_selected(&selected)?;
    if !safe_path(catalog) {
        return Err("unsafe catalog path".to_owned());
    }
    let statuses = parse_porcelain(&repository.status_porcelain()?)?;
    let mut by_path = BTreeMap::new();
    for status in statuses {
        if by_path.insert(status.path.clone(), status).is_some() {
            return Err("duplicate git status path".to_owned());
        }
    }
    for (path, status) in &by_path {
        if path == catalog {
            return Err("catalog is not clean against HEAD".to_owned());
        }
        if selected.binary_search(path).is_err() || !status.is_worktree_modified_only() {
            return Err(format!("dirty path outside selected prose edits: {path}"));
        }
    }
    if repository.current_bytes(catalog)? != repository.head_bytes(catalog)? {
        return Err("catalog is not clean against HEAD".to_owned());
    }

    let mut touched = BTreeSet::new();
    for path in &selected {
        let current = repository.current_bytes(path)?;
        let head = repository.head_bytes(path)?;
        let current = std::str::from_utf8(&current)
            .map_err(|_| format!("selected case is not UTF-8: {path}"))?;
        let head =
            std::str::from_utf8(&head).map_err(|_| format!("HEAD case is not UTF-8: {path}"))?;
        let current_layout = Case::raw_layout(current)
            .map_err(|error| format!("parse selected case {path}: {error}"))?;
        let head_layout =
            Case::raw_layout(head).map_err(|error| format!("parse HEAD case {path}: {error}"))?;
        if !head_layout.same_non_replay_output_bytes(head, &current_layout, current) {
            return Err(format!("selected case has non-prose changes: {path}"));
        }
        let changed = current != head;
        match (changed, by_path.get(path)) {
            (false, None) => {}
            (true, Some(status)) if status.is_worktree_modified_only() => {
                touched.insert(path.clone());
            }
            (false, Some(_)) => return Err(format!("status differs without case bytes: {path}")),
            (true, None) => return Err(format!("case bytes differ without git status: {path}")),
            (true, Some(_)) => return Err(format!("selected case has invalid status: {path}")),
        }
    }
    Ok(ProseClassification { selected, touched })
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct StatusEntry {
    path: String,
    source: Option<String>,
    index: IndexStatus,
    worktree: WorktreeStatus,
}

impl StatusEntry {
    fn is_worktree_modified_only(&self) -> bool {
        self.source.is_none()
            && self.index == IndexStatus::Clean
            && self.worktree == WorktreeStatus::Modified
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum IndexStatus {
    Clean,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Updated,
    Unmerged,
    Untracked,
    Ignored,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WorktreeStatus {
    Clean,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Updated,
    Unmerged,
    Untracked,
    Ignored,
}

fn parse_porcelain(bytes: &[u8]) -> Result<Vec<StatusEntry>, String> {
    let records: Vec<_> = bytes
        .split(|byte| *byte == b'\0')
        .filter(|record| !record.is_empty())
        .collect();
    let mut index = 0usize;
    let mut entries = Vec::new();
    while let Some(record) = records.get(index) {
        let [x, y, separator, path @ ..] = *record else {
            return Err("malformed git porcelain status".to_owned());
        };
        if *separator != b' ' {
            return Err("malformed git porcelain status".to_owned());
        }
        let path = status_path(path)?;
        let (index_status, worktree_status) = status_classes(*x, *y)?;
        let renamed_or_copied = matches!(index_status, IndexStatus::Renamed | IndexStatus::Copied)
            || matches!(
                worktree_status,
                WorktreeStatus::Renamed | WorktreeStatus::Copied
            );
        let source = if renamed_or_copied {
            index = index.saturating_add(1);
            Some(status_path(records.get(index).ok_or_else(|| {
                "truncated rename/copy porcelain record".to_owned()
            })?)?)
        } else {
            None
        };
        entries.push(StatusEntry {
            path,
            source,
            index: index_status,
            worktree: worktree_status,
        });
        index = index.saturating_add(1);
    }
    Ok(entries)
}

fn status_classes(x: u8, y: u8) -> Result<(IndexStatus, WorktreeStatus), String> {
    if x == b'?' && y == b'?' {
        return Ok((IndexStatus::Untracked, WorktreeStatus::Untracked));
    }
    if x == b'!' && y == b'!' {
        return Ok((IndexStatus::Ignored, WorktreeStatus::Ignored));
    }
    let index = match x {
        b' ' => IndexStatus::Clean,
        b'M' => IndexStatus::Modified,
        b'A' => IndexStatus::Added,
        b'D' => IndexStatus::Deleted,
        b'R' => IndexStatus::Renamed,
        b'C' => IndexStatus::Copied,
        b'T' => IndexStatus::Updated,
        b'U' => IndexStatus::Unmerged,
        _ => return Err("malformed git porcelain index status".to_owned()),
    };
    let worktree = match y {
        b' ' => WorktreeStatus::Clean,
        b'M' => WorktreeStatus::Modified,
        b'D' => WorktreeStatus::Deleted,
        b'R' => WorktreeStatus::Renamed,
        b'C' => WorktreeStatus::Copied,
        b'T' => WorktreeStatus::Updated,
        b'U' => WorktreeStatus::Unmerged,
        _ => return Err("malformed git porcelain worktree status".to_owned()),
    };
    Ok((index, worktree))
}

fn status_path(bytes: &[u8]) -> Result<String, String> {
    let path = std::str::from_utf8(bytes)
        .map_err(|_| "git status path is not UTF-8".to_owned())?
        .to_owned();
    if safe_path(&path) {
        Ok(path)
    } else {
        Err("unsafe git status path".to_owned())
    }
}

fn validate_selected(selected: &[String]) -> Result<(), String> {
    if selected.is_empty() {
        return Err("no selected cases".to_owned());
    }
    if selected.windows(2).any(|pair| {
        pair.first()
            .zip(pair.get(1))
            .is_some_and(|(left, right)| left >= right)
    }) || selected.iter().any(|path| !safe_path(path))
    {
        return Err("selected paths are not canonical".to_owned());
    }
    Ok(())
}

fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':', '\0'])
        && path.split('/').all(|part| !matches!(part, "" | "." | ".."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default)]
    struct FakeRepository {
        status: Vec<u8>,
        current: BTreeMap<String, Vec<u8>>,
        head: BTreeMap<String, Vec<u8>>,
    }

    impl Repository for FakeRepository {
        fn status_porcelain(&self) -> Result<Vec<u8>, String> {
            Ok(self.status.clone())
        }
        fn current_bytes(&self, path: &str) -> Result<Vec<u8>, String> {
            self.current
                .get(path)
                .cloned()
                .ok_or_else(|| "missing current".to_owned())
        }
        fn head_bytes(&self, path: &str) -> Result<Vec<u8>, String> {
            self.head
                .get(path)
                .cloned()
                .ok_or_else(|| "missing HEAD".to_owned())
        }
    }

    const CATALOG: &str = "spike/crates/core/src/catalog_lock.rs";
    const CASE: &str = "spike/crates/dorc-loom/cases/one.txt";

    fn case(frontmatter: &str, preamble: &str, book: &str, command: &str, output: &str) -> String {
        format!(
            "---\n{frontmatter}---\n{preamble}-- book.sh --\n{book}\n\n-- replay --\n$ {command}\n{output}"
        )
    }

    fn repository(head_case: String, current_case: String, status: &[u8]) -> FakeRepository {
        let mut current = BTreeMap::new();
        let mut head = BTreeMap::new();
        current.insert(CATALOG.to_owned(), b"catalog".to_vec());
        head.insert(CATALOG.to_owned(), b"catalog".to_vec());
        current.insert(CASE.to_owned(), current_case.into_bytes());
        head.insert(CASE.to_owned(), head_case.into_bytes());
        FakeRepository {
            status: status.to_vec(),
            current,
            head,
        }
    }

    #[test]
    fn accepts_only_worktree_prose_and_records_touched_paths() {
        let head = case(
            "code: one\nwhy: old\n",
            "preamble\n",
            "book",
            "dorc plan --book=book.sh",
            "old prose\n",
        );
        let current = head.replace("old prose", "new prose");
        let repository = repository(head, current, format!(" M {CASE}\0").as_bytes());
        let result =
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).expect("accept");
        assert_eq!(result.selected(), &[CASE.to_owned()]);
        assert_eq!(result.touched(), &BTreeSet::from([CASE.to_owned()]));
    }

    #[test]
    fn rejects_every_non_output_case_change() {
        let head = case(
            "code: one\nwhy: old\nall: keys\n",
            "preamble\n",
            "book",
            "dorc plan --book=book.sh",
            "old prose\n",
        );
        for changed in [
            head.replace("code: one", "code: two"),
            head.replace("why: old", "why: new"),
            head.replace("all: keys", "all: changed"),
            head.replace("preamble\n", "preamble changed\n"),
            head.replace("book\n", "book changed\n"),
            head.replace("$ dorc plan", "$ dorc explain"),
            head.replace("\n\n-- replay --", "\n \n-- replay --"),
        ] {
            let repository = repository(head.clone(), changed, format!(" M {CASE}\0").as_bytes());
            assert!(classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).is_err());
        }
    }

    #[test]
    fn accepts_clean_selected_cases_without_touching_them() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        let repository = repository(source.clone(), source, b"");
        let result =
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).expect("clean");
        assert!(result.touched().is_empty());
    }

    #[test]
    fn rejects_all_porcelain_classes_other_than_worktree_modified() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        for status in [
            format!("M  {CASE}\0"),
            format!("?? {CASE}\0"),
            format!("R  {CASE}\0old.txt\0"),
            format!("C  {CASE}\0old.txt\0"),
            format!(" D {CASE}\0"),
            format!("UU {CASE}\0"),
        ] {
            let repository = repository(source.clone(), source.clone(), status.as_bytes());
            assert!(classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).is_err());
        }
    }

    #[test]
    fn rejects_dirty_catalog_and_unselected_paths() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        let mut repository = repository(
            source.clone(),
            source.clone(),
            format!(" M {CATALOG}\0").as_bytes(),
        );
        assert!(classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).is_err());
        repository.status = b"?? unrelated.txt\0".to_vec();
        assert!(classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).is_err());
    }

    #[test]
    fn rejection_does_not_mutate_the_injected_repository_snapshot() {
        let head = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        let repository = repository(
            head.clone(),
            head.replace("-- book.sh --", "-- renamed.sh --"),
            format!(" M {CASE}\0").as_bytes(),
        );
        let before = repository.clone();
        assert!(classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG).is_err());
        assert_eq!(repository.status, before.status);
        assert_eq!(repository.current, before.current);
        assert_eq!(repository.head, before.head);
    }

    #[test]
    fn porcelain_parser_consumes_rename_copy_sources_and_models_all_classes() {
        let records =
            parse_porcelain(b"R  new.txt\0old.txt\0C  copy.txt\0source.txt\0 M modified.txt\0")
                .expect("records");
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].source.as_deref(), Some("old.txt"));
        assert_eq!(records[1].source.as_deref(), Some("source.txt"));
        for (x, y) in [
            (b' ', b' '),
            (b'M', b' '),
            (b'A', b' '),
            (b'D', b' '),
            (b'R', b' '),
            (b'C', b' '),
            (b'T', b' '),
            (b'U', b' '),
            (b' ', b'M'),
            (b' ', b'D'),
            (b' ', b'T'),
            (b' ', b'U'),
            (b'?', b'?'),
            (b'!', b'!'),
        ] {
            assert!(status_classes(x, y).is_ok(), "{x:?}{y:?}");
        }
        assert!(parse_porcelain(b"R  new.txt\0").is_err());
        assert!(parse_porcelain(b" M ../escape\0").is_err());
        assert!(status_classes(b'?', b' ').is_err());
    }
}
