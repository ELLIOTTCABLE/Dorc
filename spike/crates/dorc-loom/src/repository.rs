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
    staged: BTreeSet<String>,
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

    /// The subset of [`Self::touched`] whose edit is wholly staged rather than wholly unstaged.
    ///
    /// Classification treats the two alike — neither reads the index. They part company only
    /// after publication, which strands a staged case's index copy on pre-promote bytes.
    #[must_use]
    pub fn staged(&self) -> &BTreeSet<String> {
        &self.staged
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

/// A dirty generated lock means a PRIOR promote is still uncommitted, and a reader who has never
/// seen this tool's two-file publication has no way to guess that from the path alone.
///
/// `compile` refusing on `promote`'s condition reads as over-reach until you know the receipt binds
/// the lock's bytes. And the way out most readers want is not to commit between cases but to stop
/// promoting one at a time — nothing else in the tool says a CASE list is legal at all, so the
/// batched shape is learnable only by being refused first.
fn lock_not_clean(path: &str) -> String {
    format!(
        "the generated lock {path} differs from HEAD, which means an earlier `dorc-loom promote` \
         has not been committed. Both verbs refuse here: a promote would publish on top of it and \
         the two changes could no longer be committed apart, and a compile would bind those \
         uncommitted bytes into its receipt. Three ways on: commit the pending promotion (the lock \
         and the case it rewrote); or `git restore` both and start over; or, when several cases \
         are in flight, promote them TOGETHER -- compile and promote each take a CASE list, and \
         bare they take the whole corpus and narrow to the cases you edited, so one compile and \
         one promote publish all of them at once."
    )
}

/// How far a refused command echo is allowed to run before the refusal stops being readable.
const MAX_ECHOED_COMMAND: usize = 120;

/// How many changed names one clause lists before it counts the rest.
const MAX_LISTED_NAMES: usize = 4;

fn ellipsized(text: &str) -> String {
    if text.len() <= MAX_ECHOED_COMMAND {
        return text.to_owned();
    }
    let mut end = MAX_ECHOED_COMMAND;
    while !text.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    format!("{}...", &text[..end])
}

fn listed(names: &[String]) -> String {
    let shown: Vec<&str> = names
        .iter()
        .take(MAX_LISTED_NAMES)
        .map(String::as_str)
        .collect();
    match names.len().saturating_sub(shown.len()) {
        0 => shown.join(", "),
        rest => format!("{}, and {rest} more", shown.join(", ")),
    }
}

/// Which frontmatter keys differ between the committed case and the worktree one.
fn changed_frontmatter_keys(head: &Case, current: &Case) -> Vec<String> {
    let (head, current) = (head.frontmatter(), current.frontmatter());
    let mut keys: BTreeSet<&str> = head.keys().collect();
    keys.extend(current.keys());
    keys.into_iter()
        .filter(|key| head.get(key) != current.get(key))
        .map(str::to_owned)
        .collect()
}

/// Which named file sections differ (content or presence).
fn changed_sections(head: &Case, current: &Case) -> Vec<String> {
    let committed: BTreeMap<&str, &str> = head
        .sections()
        .iter()
        .map(|section| (section.name(), section.content()))
        .collect();
    let mut working: BTreeMap<&str, &str> = current
        .sections()
        .iter()
        .map(|section| (section.name(), section.content()))
        .collect();
    let mut changed: Vec<String> = Vec::new();
    for (name, content) in &committed {
        match working.remove(name) {
            Some(other) if other == *content => {}
            _ => changed.push((*name).to_owned()),
        }
    }
    changed.extend(working.into_keys().map(str::to_owned));
    changed.sort();
    changed
}

/// The first replay block whose COMMAND moved, or the arity change that made the lists incomparable.
fn changed_replay_commands(head: &Case, current: &Case) -> Option<String> {
    let (committed, working) = (head.replay().blocks(), current.replay().blocks());
    if committed.len() != working.len() {
        return Some(format!(
            "the case now has {} replay {}, not {}",
            working.len(),
            if working.len() == 1 {
                "block"
            } else {
                "blocks"
            },
            committed.len()
        ));
    }
    committed
        .iter()
        .zip(working)
        .enumerate()
        .find(|(_, (one, other))| one.command() != other.command())
        .map(|(index, (one, other))| {
            format!(
                "replay {index}'s command is now `{}`, not `{}`",
                ellipsized(other.command()),
                ellipsized(one.command())
            )
        })
}

/// Name the class of non-prose change and the ONE way forward for it
/// (`28L:rul-refusals-name-the-next-command`).
///
/// A prose edit is the difference between HEAD and the worktree INSIDE the replay-output islands;
/// everything else is a case-structure change, and each class has a different way out. Saying only
/// "non-prose changes" named the bytes the tool refuses rather than the edit the author made, which
/// leaves a reader guessing which of three unrelated flows they are in.
///
/// This is git-diff triage over two already-parsed cases — a comparison of frontmatter maps,
/// section names, and replay command strings. It never re-derives editability or word boundaries
/// from byte shapes (`28L:rul-editability-is-stamped-never-re-derived`); that remains the stamped
/// part stream's alone.
fn non_prose_diagnosis(case: &str, head: &Case, current: &Case) -> String {
    let slug = case
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".loom"))
        .unwrap_or(case);
    let mut clauses: Vec<String> = Vec::new();
    let keys = changed_frontmatter_keys(head, current);
    if !keys.is_empty() {
        clauses.push(format!(
            "FRONTMATTER changed ({}). Commit that first, with the transcript as authored -- it is \
             case structure, not prose. If a metadata key moved (when-fires / when-used / why), the \
             promote that follows needs `--accept-metadata` to acknowledge that those words replace \
             the committed registry entry",
            listed(&keys)
        ));
    }
    if let Some(moved) = changed_replay_commands(head, current) {
        clauses.push(format!(
            "a REPLAY COMMAND changed ({moved}). A new or retyped command moves bytes outside the \
             replay-output islands, so nothing fills it in place:\n{}\nCommit the filled case, then \
             edit its prose",
            crate::dump_rescue_hint(slug)
        ));
    }
    let sections = changed_sections(head, current);
    if !sections.is_empty() {
        clauses.push(format!(
            "a FILE SECTION changed ({}). A fixture edit is case structure too: commit it, re-derive \
             the transcript through the dump above, and edit prose against the committed bytes",
            listed(&sections)
        ));
    }
    if clauses.is_empty() {
        return format!(
            "its non-replay bytes moved somewhere this triage cannot name -- the text above the \
             first section, or the blank-line layout the container canonicalizes. Commit the case as \
             it now stands, then edit its prose; `mise run test:looms -- {slug}` says whether the \
             committed bytes are a render fixpoint"
        );
    }
    clauses.join("; also, ")
}

/// Parse and classify the complete repository snapshot without performing I/O.
///
/// Only selected cases whose edit sits wholly on one side of the index may
/// differ, and only within raw replay-output islands. Replay provenance decides
/// whether those islands are editable in the subsequent inspection pass.
///
/// The refusal is BLAST-RADIUS scoped: only `selected` and the two generated locks (`catalog`,
/// `arrangement`) are this run's concern. Dirt elsewhere in the repository no longer refuses it.
///
/// # Errors
///
/// Returns a refusal for malformed Git state, a dirty generated lock, a selected case in the wrong
/// git state, or any non-output transcript difference.
pub fn classify_prose_changes(
    repository: &impl Repository,
    selected: Vec<String>,
    catalog: &str,
    arrangement: &str,
) -> Result<ProseClassification, String> {
    validate_selected(&selected)?;
    if !safe_path(catalog) || !safe_path(arrangement) {
        return Err("unsafe generated-lock path".to_owned());
    }
    let statuses = parse_porcelain(&repository.status_porcelain()?)?;
    let mut by_path = BTreeMap::new();
    for status in statuses {
        if by_path.insert(status.path.clone(), status).is_some() {
            return Err("duplicate git status path".to_owned());
        }
    }
    for (path, status) in &by_path {
        if path == catalog || path == arrangement {
            return Err(lock_not_clean(path));
        }
        if selected.binary_search(path).is_err() {
            // Outside the blast radius: this run never reads or writes it.
            continue;
        }
        if status.prose_edit_shape().is_none() {
            // A SELECTED case in the wrong git state: "outside selected prose edits" reads as a
            // contradiction here, and an untracked case has no HEAD side to diff prose against.
            return Err(match status.index {
                IndexStatus::Untracked => format!(
                    "selected case {path} is not committed. Promote reads the prose edit as the \
                     difference between HEAD and your worktree, so a brand-new case has to be \
                     committed (with its transcript as authored) before its prose can be promoted"
                ),
                _ => format!(
                    "selected case {path} is in git state `{code}`; dorc-loom reads a prose edit \
                     as the difference between HEAD and your worktree, so a selected case must be \
                     either wholly unstaged (` M`) or wholly staged (`M `) and nothing else",
                    code = String::from_utf8_lossy(&status.code)
                ),
            });
        }
    }
    for lock in [catalog, arrangement] {
        if repository.current_bytes(lock)? != repository.head_bytes(lock)? {
            return Err(lock_not_clean(lock));
        }
    }

    let (mut touched, mut staged) = (BTreeSet::new(), BTreeSet::new());
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
            let head_case =
                Case::parse(head).map_err(|error| format!("parse HEAD case {path}: {error}"))?;
            let current_case = Case::parse(current)
                .map_err(|error| format!("parse selected case {path}: {error}"))?;
            return Err(format!(
                "selected case {path} changed outside its replay outputs, which is the only place a \
                 prose edit lives: {}",
                non_prose_diagnosis(path, &head_case, &current_case)
            ));
        }
        let changed = current != head;
        match (changed, by_path.get(path)) {
            (false, None) => {}
            (true, Some(status)) => match status.prose_edit_shape() {
                Some(shape) => {
                    touched.insert(path.clone());
                    if shape == EditShape::Staged {
                        staged.insert(path.clone());
                    }
                }
                None => return Err(format!("selected case has invalid status: {path}")),
            },
            (false, Some(_)) => return Err(format!("status differs without case bytes: {path}")),
            (true, None) => return Err(format!("case bytes differ without git status: {path}")),
        }
    }
    Ok(ProseClassification {
        selected,
        touched,
        staged,
    })
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct StatusEntry {
    path: String,
    source: Option<String>,
    index: IndexStatus,
    worktree: WorktreeStatus,
    /// Retained so a refusal can name the state the author is in, not just the one it wanted.
    code: [u8; 2],
}

impl StatusEntry {
    /// The two porcelain states in which HEAD -> worktree is the author's whole edit.
    ///
    /// Porcelain's second column reports worktree-against-INDEX, so a clean one means a wholly
    /// staged case holds no third version for that diff to miss; `MM` does, and is refused. A
    /// rename keeps its bytes at another path, where `head_bytes` would not find them.
    fn prose_edit_shape(&self) -> Option<EditShape> {
        if self.source.is_some() {
            return None;
        }
        match (self.index, self.worktree) {
            (IndexStatus::Clean, WorktreeStatus::Modified) => Some(EditShape::Unstaged),
            (IndexStatus::Modified, WorktreeStatus::Clean) => Some(EditShape::Staged),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EditShape {
    Unstaged,
    Staged,
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
            code: [*x, *y],
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

    const CATALOG: &str = "spike/crates/aid/src/catalog_lock.rs";
    const ARRANGEMENT: &str = "spike/crates/aid/src/arrangement_lock.rs";
    const CASE: &str = "spike/crates/aid/tests/one.loom";

    fn case(frontmatter: &str, preamble: &str, book: &str, command: &str, output: &str) -> String {
        format!(
            "---\n{frontmatter}---\n{preamble}-- book.sh --\n{book}\n\n-- replay --\n$ {command}\n{output}"
        )
    }

    /// The two generated locks and the one selected case, at whatever bytes/status the test wants.
    /// Every caller passes `classify_prose_changes(&repository, ..., CATALOG, ARRANGEMENT)`.
    fn repository(head_case: String, current_case: String, status: &[u8]) -> FakeRepository {
        let mut current = BTreeMap::new();
        let mut head = BTreeMap::new();
        current.insert(CATALOG.to_owned(), b"catalog".to_vec());
        head.insert(CATALOG.to_owned(), b"catalog".to_vec());
        current.insert(ARRANGEMENT.to_owned(), b"arrangement".to_vec());
        head.insert(ARRANGEMENT.to_owned(), b"arrangement".to_vec());
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
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect("accept");
        assert_eq!(result.selected(), &[CASE.to_owned()]);
        assert_eq!(result.touched(), &BTreeSet::from([CASE.to_owned()]));
        assert!(result.staged().is_empty());
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
            assert!(
                classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                    .is_err()
            );
        }
    }

    /// Refusing is half the job: the three classes of non-prose change have three unrelated ways
    /// out, and a reader holding "non-prose changes" has been told which bytes the tool declined
    /// rather than what they did or what to do next
    /// (`28L:rul-refusals-name-the-next-command`).
    #[test]
    fn each_class_of_non_prose_change_names_its_own_way_out() {
        let head = case(
            "code: one\nwhy: old\n",
            "preamble\n",
            "book",
            "dorc plan --book=book.sh",
            "old prose\n",
        );
        let refusal = |changed: String| {
            let repository = repository(head.clone(), changed, format!(" M {CASE}\0").as_bytes());
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect_err("a non-prose change refuses")
        };

        let metadata = refusal(head.replace("why: old", "why: new"));
        assert!(metadata.contains("FRONTMATTER changed (why)"), "{metadata}");
        assert!(metadata.contains("--accept-metadata"), "{metadata}");

        let retyped = refusal(head.replace("$ dorc plan", "$ dorc explain"));
        assert!(retyped.contains("REPLAY COMMAND changed"), "{retyped}");
        assert!(
            retyped.contains("`dorc explain --book=book.sh`"),
            "{retyped}"
        );
        assert!(retyped.contains("DORC_LOOM_DUMP=<dir>"), "{retyped}");

        let added = refusal(head.replace(
            "$ dorc plan --book=book.sh\n",
            "$ dorc plan --book=book.sh\nold prose\n\n$ dorc why 1\n",
        ));
        assert!(added.contains("2 replay blocks, not 1"), "{added}");

        let fixture = refusal(head.replace("book\n", "book changed\n"));
        assert!(
            fixture.contains("FILE SECTION changed (book.sh)"),
            "{fixture}"
        );

        // Layout residue: every named class agrees, so the fallback has to carry a way forward too.
        let layout = refusal(head.replace("preamble\n", "preamble changed\n"));
        assert!(layout.contains("cannot name"), "{layout}");
        assert!(layout.contains("mise run test:looms -- one"), "{layout}");
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
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect("clean");
        assert!(result.touched().is_empty());
    }

    /// Porcelain `M ` guarantees the worktree and index agree, so HEAD -> worktree is still the
    /// author's whole edit and nothing about the read changes. Only the aftermath differs.
    #[test]
    fn a_wholly_staged_case_is_touched_and_reported_as_staged() {
        let head = case(
            "code: one\nwhy: old\n",
            "preamble\n",
            "book",
            "dorc plan --book=book.sh",
            "old prose\n",
        );
        let current = head.replace("old prose", "new prose");
        let repository = repository(head, current, format!("M  {CASE}\0").as_bytes());
        let result =
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect("accept");
        assert_eq!(result.touched(), &BTreeSet::from([CASE.to_owned()]));
        assert_eq!(result.staged(), &BTreeSet::from([CASE.to_owned()]));
    }

    #[test]
    fn rejects_porcelain_classes_outside_the_two_legal_edit_shapes() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        for status in [
            format!("?? {CASE}\0"),
            format!("MM {CASE}\0"),
            format!("A  {CASE}\0"),
            format!("R  {CASE}\0old.txt\0"),
            format!("C  {CASE}\0old.txt\0"),
            format!(" D {CASE}\0"),
            format!("UU {CASE}\0"),
        ] {
            let repository = repository(source.clone(), source.clone(), status.as_bytes());
            assert!(
                classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                    .is_err()
            );
        }
    }

    /// A brand-new case is untracked, and the old refusal called it a "dirty path OUTSIDE selected
    /// prose edits" — a sentence that contradicts itself for a path the author just named on the
    /// command line, and that names no way forward. The two refusals must now read differently and
    /// the untracked one must say what to do.
    #[test]
    fn an_untracked_selected_case_says_it_must_be_committed() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        let repository = repository(source.clone(), source, format!("?? {CASE}\0").as_bytes());
        let error =
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect_err("an untracked case is unpromotable");
        assert!(error.contains("not committed"), "{error}");
        assert!(!error.contains("outside selected prose edits"), "{error}");
    }

    /// The gate is shared by `compile` and `promote`, so a refusal phrased around `promote` names
    /// a command the author may not have run. Every refused state has a different way out, so the
    /// observed `XY` pair has to appear.
    #[test]
    fn a_refused_git_state_is_named_and_no_verb_is_blamed() {
        let head = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "old prose\n",
        );
        let current = head.replace("old prose", "new prose");
        let repository = repository(head, current, format!("MM {CASE}\0").as_bytes());
        let error =
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect_err("a half-staged case holds a third version");
        assert!(error.contains("`MM`"), "{error}");
        assert!(!error.contains("promote"), "{error}");
    }

    /// Dirt INSIDE the blast radius — either generated lock — refuses and names the dirty file
    /// (`28L` friction §4 blast-radius-scoped dirty gate), and names the batched shape that avoids
    /// the serialization rather than only the commit that clears it.
    #[test]
    fn dirty_generated_lock_refuses_and_names_the_file() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        for dirty in [CATALOG, ARRANGEMENT] {
            let repository = repository(
                source.clone(),
                source.clone(),
                format!(" M {dirty}\0").as_bytes(),
            );
            let error =
                classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                    .expect_err("a dirty generated lock refuses");
            assert!(error.contains(dirty), "{error}");
            assert!(error.contains("CASE list"), "{error}");
            assert!(error.contains("TOGETHER"), "{error}");
        }
    }

    /// Dirt OUTSIDE the blast radius — neither a selected case nor a generated lock — no longer
    /// refuses: a source edit anywhere else in the repo used to force a throwaway commit before
    /// `compile`/`promote` would even run (`28L` friction §4 blast-radius-scoped dirty gate).
    #[test]
    fn dirt_outside_the_blast_radius_does_not_refuse() {
        let source = case(
            "code: one\n",
            "",
            "book",
            "dorc plan --book=book.sh",
            "prose\n",
        );
        let repository = repository(
            source.clone(),
            source,
            b"?? unrelated.txt\0 M crates/dorc-loom/src/bin/dorc-loom.rs\0",
        );
        let result =
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .expect("dirt outside the blast radius proceeds");
        assert!(result.touched().is_empty());
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
        assert!(
            classify_prose_changes(&repository, vec![CASE.to_owned()], CATALOG, ARRANGEMENT)
                .is_err()
        );
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
