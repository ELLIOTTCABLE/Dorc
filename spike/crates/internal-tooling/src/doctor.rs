//! Two read-only inventories of what the fleet has left lying around.
//!
//! `doctor` answers "what is eating the disk". The observed pattern is pile-up, not any single
//! hog: nineteen abandoned lane caches held 51 GiB inside an 87.9 GiB vhdx, and finished agent
//! worktrees carried ~9 GiB of `target/` apiece.
//!
//! `doctor unused` answers the other half — "what is sitting around unused" — and is built to be
//! COMPARABLE: sorted, no sizes, no timestamps, so a conductor runs it at the start and the end of
//! a session and reads the difference as that session's residue. Sizes are excluded on purpose;
//! they move under ordinary work and would drown the signal.
//!
//! Both DELETE NOTHING, permanently. Automatic reaping was ruled out: it needs eyes in the loop,
//! and a cache reaped without them defeats the point of a cache. Reporting is the whole job.
//!
//! Each leg sees only its own filesystem: the Windows leg cannot see WSL's `~/.cache`, and the WSL
//! leg reaches the worktrees over `/mnt/c`. `mise run both doctor…` is the paired form and the one
//! that shows the whole picture.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crate::preflight::gib;

/// The branch lane work is expected to land on. Every merged/unmerged answer is asked against it,
/// and its absence is reported rather than guessed around.
const LINEAGE: &str = "ai/main";

/// Where worktrees are made, under the primary checkout: the harness's agent fleet and the
/// conductors' own trees. A removal that fails partway (a locked file, on Windows) leaves its
/// directory here with no registration, invisible to `git worktree list`.
const FLEET_ROOTS: [&str; 2] = [".claude/worktrees", ".tmp/trees"];

/// What a stray directory may lose for free: the build cache rebuilds, and a leftover `.git`
/// pointer names a registration that is already gone.
const DISPOSABLE: [&str; 2] = ["spike/target", ".git"];

/// The size inventory by default; the comparable hygiene report under `unused`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        None => sizes(),
        Some("unused") => unused(),
        Some(other) => {
            eprintln!("doctor: unknown mode {other:?}; modes: <none> (sizes), unused");
            ExitCode::from(2)
        }
    }
}

/// Walk every store, print a row each, then the total.
fn sizes() -> ExitCode {
    let mut at_risk = 0_u64;

    println!("== worktrees ==");
    match worktrees() {
        Err(why) => println!("  (unavailable: {why})"),
        Ok(list) if list.is_empty() => println!("  (none)"),
        Ok(list) => {
            // The worktree fleet lives at `<primary>/.claude/worktrees/`, INSIDE the primary
            // checkout, so a naive walk bills every lane's target dir twice — once to the
            // lane and once to the tree containing it. Every other registered worktree is a
            // pruning boundary.
            let registered: Vec<PathBuf> = list.iter().map(|t| t.path.clone()).collect();
            for tree in &list {
                let bytes = tree_size_excluding(&tree.path, &registered);
                at_risk = at_risk.saturating_add(bytes);
                println!(
                    "  {:>10}  {:<7} {:<7} {}  [{}]",
                    gib(bytes),
                    tree.state,
                    if tree.locked { "locked" } else { "" },
                    short(&tree.path),
                    tree.branch
                );
            }
        }
    }

    // A BREAKDOWN of a store already billed above (it sits inside this worktree, or — on the
    // WSL leg — under the lane caches). Printed because "where did the 9 GiB go" is the next
    // question; excluded from the total because it is the same bytes.
    let target = internal_tooling::target_dir();
    println!("== this leg's target dir (breakdown, counted above) ==");
    println!("  {}", target.display());
    for (path, bytes) in children_with_sizes(&target) {
        println!("  {:>10}  {}", gib(bytes), short(&path));
    }

    println!("== lane caches ==");
    match user_cache_dir() {
        None => {
            println!("  (no XDG_CACHE_HOME or HOME — Windows keeps none of these)");
            #[cfg(windows)]
            for (label, bytes) in wsl_vhdxs() {
                println!(
                    "  {:>10}  {label}  — WSL distro disk; grows with WSL builds, never \
                     auto-shrinks",
                    gib(bytes)
                );
            }
        }
        Some(cache) => {
            let mut rows = children_with_sizes(&cache);
            rows.retain(|(path, _)| is_lane_cache(&file_name(path)));
            if rows.is_empty() {
                println!("  (none)");
            }
            for (path, bytes) in rows {
                at_risk = at_risk.saturating_add(bytes);
                println!("  {:>10}  {}", gib(bytes), short(&path));
            }
        }
    }

    println!(
        "== {} at risk: worktrees + lane caches, this leg only ==",
        gib(at_risk)
    );
    ExitCode::SUCCESS
}

/// The comparable report: what is registered, what has already landed, what is keyed to a
/// worktree that no longer exists, and what a removed worktree left behind.
///
/// Every column is a fact with a git or filesystem answer behind it. Nothing here recommends a
/// reap — the join a reader makes (landed AND no worktree, or a cache whose key names nothing) is
/// theirs to make, which is exactly the "eyes in the loop" the auto-reap ruling asked for.
fn unused() -> ExitCode {
    let lineage = rev_parse(LINEAGE);
    if lineage.is_none() {
        println!("(no {LINEAGE} in this repository — every landed column reads unknown)");
    }

    let trees = worktrees();
    let live_keys: BTreeSet<String> = trees.iter().flatten().map(|t| key_of(&t.path)).collect();
    let checked_out: BTreeSet<&str> = trees.iter().flatten().map(|t| t.branch.as_str()).collect();

    match &trees {
        Err(why) => println!("== worktrees ==\n  (unavailable: {why})"),
        Ok(list) => {
            println!("== worktrees ({}) ==", list.len());
            for tree in list {
                println!(
                    "  {:<6} {:<8} {:<7} {:<26} {}",
                    tree.state,
                    landed(&tree.head, lineage.as_deref()),
                    if tree.locked { "locked" } else { "" },
                    key_of(&tree.path),
                    tree.branch
                );
            }
        }
    }

    match (&trees, primary_checkout()) {
        (Ok(list), Some(primary)) => {
            let registered: Vec<PathBuf> = list.iter().map(|t| t.path.clone()).collect();
            let strays: Vec<(PathBuf, Option<Vec<PathBuf>>)> = stray_dirs(&primary, &registered)
                .into_iter()
                .map(|dir| {
                    let files = kept_files(&dir);
                    (dir, files)
                })
                .collect();
            // The slowest query in this report, so asked only when some stray holds a file.
            let objects = if strays
                .iter()
                .any(|(_, files)| files.as_ref().is_some_and(|f| !f.is_empty()))
            {
                git_in(&primary, &["rev-list", "--objects", "--branches", "--tags"])
            } else {
                Ok(String::new())
            };
            let landed: Option<BTreeSet<&str>> = objects.as_ref().ok().map(|out| {
                out.lines()
                    .map(|line| line.split_once(' ').map_or(line, |(id, _)| id))
                    .collect()
            });
            println!("== stray directories ({}) ==", strays.len());
            for (dir, files) in &strays {
                println!(
                    "  {:<8} {}",
                    stray_state(&primary, files.as_deref(), landed.as_ref()),
                    short(dir)
                );
            }
        }
        _ => println!(
            "== stray directories ==\n  (unavailable: no worktree list or primary checkout)"
        ),
    }

    match local_branches() {
        Err(why) => println!("== branches ==\n  (unavailable: {why})"),
        Ok(names) => {
            let merged = lineage.as_deref().map(merged_branches).unwrap_or_default();
            println!("== branches ({}) ==", names.len());
            for name in &names {
                println!(
                    "  {:<8} {:<12} {}",
                    match lineage {
                        None => "unknown",
                        Some(_) if merged.contains(name) => "merged",
                        Some(_) => "unmerged",
                    },
                    if checked_out.contains(name.as_str()) {
                        "worktree"
                    } else {
                        "no-worktree"
                    },
                    name
                );
            }
        }
    }

    match user_cache_dir().map(|cache| lane_caches(&cache)) {
        None => println!("== lane caches ==\n  (no XDG_CACHE_HOME or HOME — Windows keeps none)"),
        Some(names) => {
            println!("== lane caches ({}) ==", names.len());
            for name in &names {
                println!("  {:<7} {}", cache_state(name, &live_keys), name);
            }
        }
    }

    ExitCode::SUCCESS
}

/// One checkout git knows about.
#[derive(Debug)]
struct Tree {
    path: PathBuf,
    branch: String,
    /// The tip commit, which is what "has this landed" is actually asked about — a detached
    /// checkout has no branch to ask about at all.
    head: String,
    /// `clean`, `dirty`, or why we could not tell — never guessed.
    state: String,
    /// A locked worktree belongs to a live lane. Reaping one is how concurrent work dies.
    locked: bool,
}

/// Every worktree, from git rather than from a glob: a glob finds directories, while git
/// finds the ones that are actually registered, and a stale directory is a different
/// problem with a different fix (`git worktree prune`).
fn worktrees() -> Result<Vec<Tree>, String> {
    let mut trees = parse_worktree_list(&git(&["worktree", "list", "--porcelain"])?);
    for tree in &mut trees {
        tree.state = tree_state(&tree.path);
    }
    // Sorted so both reports are byte-stable across runs; git's own order is registration order.
    trees.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(trees)
}

/// Split from its `git` call so the format can be pinned without one: shelling out per
/// worktree is both slow and non-hermetic, and neither belongs in the unit tier.
///
/// Porcelain shape: one `worktree <path>` line opens a record, and `HEAD`/`branch`/`locked` are
/// attributes of the record still open.
fn parse_worktree_list(porcelain: &str) -> Vec<Tree> {
    let mut trees: Vec<Tree> = Vec::new();
    for line in porcelain.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            trees.push(Tree {
                path: PathBuf::from(path),
                branch: "detached".to_owned(),
                head: String::new(),
                state: "unknown".to_owned(),
                locked: false,
            });
        } else if let Some(branch) = line.strip_prefix("branch ")
            && let Some(tree) = trees.last_mut()
        {
            branch
                .trim_start_matches("refs/heads/")
                .clone_into(&mut tree.branch);
        } else if let Some(head) = line.strip_prefix("HEAD ")
            && let Some(tree) = trees.last_mut()
        {
            head.clone_into(&mut tree.head);
        } else if (line.trim() == "locked" || line.starts_with("locked "))
            && let Some(tree) = trees.last_mut()
        {
            tree.locked = true;
        }
    }
    trees
}

/// Clean or dirty, by git's own answer — untracked files included, because they are exactly
/// what a reap would destroy.
fn tree_state(path: &Path) -> String {
    git_in(path, &["status", "--porcelain"]).map_or_else(
        |_| "unknown".to_owned(),
        |out| {
            if out.is_empty() {
                "clean".to_owned()
            } else {
                "DIRTY".to_owned()
            }
        },
    )
}

/// The primary checkout, where the fleets live. Not [`dorc_testbed::repo_root`], which is
/// whichever tree this binary was built in — a lane's own, when run from one.
fn primary_checkout() -> Option<PathBuf> {
    let common = git(&["rev-parse", "--path-format=absolute", "--git-common-dir"]).ok()?;
    Path::new(common.trim()).parent().map(Path::to_path_buf)
}

/// Every directory under a [`FLEET_ROOTS`] entry that git has no worktree registered at, sorted.
fn stray_dirs(primary: &Path, registered: &[PathBuf]) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = FLEET_ROOTS
        .iter()
        .filter_map(|root| std::fs::read_dir(primary.join(root)).ok())
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && !registered.contains(path))
        .collect();
    found.sort();
    found
}

/// Every file under `dir` but the [`DISPOSABLE`] paths, or `None` when any part could not be
/// read: an unread corner would otherwise report `empty`, the one wrong answer that invites a
/// delete.
fn kept_files(dir: &Path) -> Option<Vec<PathBuf>> {
    let skip: Vec<PathBuf> = DISPOSABLE.iter().map(|path| dir.join(path)).collect();
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(path) = stack.pop() {
        if skip.contains(&path) {
            continue;
        }
        if std::fs::symlink_metadata(&path).ok()?.is_dir() {
            for entry in std::fs::read_dir(&path).ok()? {
                stack.push(entry.ok()?.path());
            }
        } else {
            files.push(path);
        }
    }
    files.sort();
    Some(files)
}

/// `empty` (nothing but [`DISPOSABLE`] paths), `landed` (every file's exact content is reachable
/// from a branch or tag), `UNIQUE` (some file's content is on no branch or tag — a delete loses
/// it), or `unknown`. Content, not path: a stray is usually an older checkout of files that have
/// since moved on, so its paths answer nothing.
fn stray_state(
    primary: &Path,
    files: Option<&[PathBuf]>,
    landed: Option<&BTreeSet<&str>>,
) -> &'static str {
    let Some(files) = files else {
        return "unknown";
    };
    if files.is_empty() {
        return "empty";
    }
    let (Some(landed), Ok(ids)) = (landed, blob_ids(primary, files)) else {
        return "unknown";
    };
    if ids.len() != files.len() {
        "unknown"
    } else if ids.iter().all(|id| landed.contains(id.as_str())) {
        "landed"
    } else {
        "UNIQUE"
    }
}

/// The blob id each file would have if committed: `hash-object` without `-w`, so a query like
/// every other git call here. Asked of the primary checkout so its line-ending attributes apply,
/// and chunked to stay under Windows' command-line limit.
fn blob_ids(primary: &Path, files: &[PathBuf]) -> Result<Vec<String>, String> {
    let mut ids = Vec::with_capacity(files.len());
    for chunk in files.chunks(100) {
        let paths: Vec<String> = chunk
            .iter()
            .map(|file| file.to_string_lossy().into_owned())
            .collect();
        let mut args = vec!["hash-object", "--"];
        args.extend(paths.iter().map(String::as_str));
        ids.extend(git_in(primary, &args)?.lines().map(str::to_owned));
    }
    Ok(ids)
}

/// Every local branch, sorted by name.
fn local_branches() -> Result<Vec<String>, String> {
    let mut names: Vec<String> = git(&["for-each-ref", "--format=%(refname:short)", "refs/heads"])?
        .lines()
        .map(str::to_owned)
        .collect();
    names.sort();
    Ok(names)
}

/// The branches already contained in the lineage — one call rather than a `merge-base` per
/// branch. Worktrees cannot share it: a detached checkout has no ref for `--merged` to answer
/// about, which is why [`landed`] asks about a commit instead.
fn merged_branches(lineage: &str) -> BTreeSet<String> {
    git(&[
        "for-each-ref",
        "--merged",
        lineage,
        "--format=%(refname:short)",
        "refs/heads",
    ])
    .map(|out| out.lines().map(str::to_owned).collect())
    .unwrap_or_default()
}

/// Whether `sha` is already contained in the lineage — the question behind "is this still work,
/// or residue".
///
/// Read from the exit code's own trichotomy rather than `success()`: 1 means genuinely not an
/// ancestor, and anything else (a bad revision, a missing object) is a failure to measure, which
/// must not be reported as an answer.
fn landed(sha: &str, lineage: Option<&str>) -> &'static str {
    let Some(target) = lineage else {
        return "unknown";
    };
    let Ok(out) = Command::new("git")
        .arg("-C")
        .arg(dorc_testbed::repo_root())
        .args(["merge-base", "--is-ancestor", sha, target])
        .output()
    else {
        return "unknown";
    };
    match out.status.code() {
        Some(0) => "merged",
        Some(1) => "unmerged",
        _ => "unknown",
    }
}

/// A commit id for `rev`, or `None` when the repository has no such ref.
fn rev_parse(rev: &str) -> Option<String> {
    git(&["rev-parse", "--verify", "--quiet", rev])
        .ok()
        .map(|out| out.trim().to_owned())
}

/// Run a read-only git query at the repository root.
///
/// Every git call this module makes goes through here or [`git_in`], and every one of them is a
/// query — `doctor` reports, and the moment it also writes, the report stops being something a
/// conductor can run twice without consequence.
fn git(args: &[&str]) -> Result<String, String> {
    git_in(dorc_testbed::repo_root(), args)
}

/// As [`git`], but rooted at one worktree — the only way to ask a per-tree question.
fn git_in(dir: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_owned())
    }
}

/// The `dorc-*` stores this leg can see, sorted.
fn lane_caches(cache: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(cache) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|e| file_name(&e.path()))
        .filter(|name| is_lane_cache(name))
        .collect();
    names.sort();
    names
}

fn is_lane_cache(name: &str) -> bool {
    name.starts_with("dorc-")
}

/// Whether a lane cache still belongs to something.
///
/// Three families, each named by an owner elsewhere and re-derived here by rule — if an owner
/// moves its cache, move this with it: `dorc-wsl-target-<worktree>` (root `mise.toml`'s
/// `CARGO_TARGET_DIR`), `dorc-minispec-lean-<worktree>` (`dorc_verify::lean_build_root`), and the
/// shared `dorc-kani-target` (`dorc_verify::kani::build_root`), which is keyed to nothing and so
/// can never be orphaned.
fn cache_state(name: &str, live_keys: &BTreeSet<String>) -> &'static str {
    if name == "dorc-kani-target" {
        return "shared";
    }
    for prefix in ["dorc-wsl-target-", "dorc-minispec-lean-"] {
        if let Some(key) = name.strip_prefix(prefix) {
            return if live_keys.contains(key) {
                "live"
            } else {
                "ORPHAN"
            };
        }
    }
    // An unrecognized `dorc-` store has no derivable worktree key, and inventing one would
    // manufacture an orphan verdict out of a naming coincidence.
    "unkeyed"
}

/// A worktree's directory name — the key the lane caches are named after (`worktree_key` in
/// `dorc-verify`, `config_root | last` in `mise.toml`).
fn key_of(path: &Path) -> String {
    file_name(path)
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned())
}

/// Immediate children of `dir` with their recursive sizes, largest first.
fn children_with_sizes(dir: &Path) -> Vec<(PathBuf, u64)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut rows: Vec<(PathBuf, u64)> = entries
        .flatten()
        .map(|e| {
            let path = e.path();
            let bytes = tree_size(&path);
            (path, bytes)
        })
        .collect();
    rows.sort_by_key(|row| std::cmp::Reverse(row.1));
    rows
}

/// Recursive byte total, iteratively: these trees run to hundreds of thousands of files and
/// a recursive walk is one hostile layout away from blowing the stack.
///
/// Symlinks are counted as links, never followed — a followed link double-counts at best and
/// loops at worst. Unreadable entries contribute zero rather than aborting the inventory.
fn tree_size(root: &Path) -> u64 {
    tree_size_excluding(root, &[])
}

/// As [`tree_size`], but treating any path in `boundaries` other than `root` itself as a
/// wall the walk does not cross.
fn tree_size_excluding(root: &Path, boundaries: &[PathBuf]) -> u64 {
    let mut total = 0_u64;
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        if path != root && boundaries.contains(&path) {
            continue;
        }
        let Ok(meta) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&path) {
                stack.extend(entries.flatten().map(|e| e.path()));
            }
        } else {
            total = total.saturating_add(meta.len());
        }
    }
    total
}

/// `XDG_CACHE_HOME`, else `$HOME/.cache`. Windows keeps no such store, and inventing one
/// there would print an empty section that reads as "nothing here" rather than "not here".
fn user_cache_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        return None;
    }
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
}

/// Every WSL distro's disk under `packages` — a UWP package folder name paired with the size of
/// its `LocalState/ext4.vhdx`, if it has one. Split from [`wsl_vhdxs`] so the walk itself is
/// unit-testable without `%LOCALAPPDATA%`.
///
/// Provenance (`300` §2): this is the SAME store `preflight`'s WSL host-volume check exists for
/// — a sparse vhdx that grows into whichever host drive holds it and never shrinks back. Windows
/// cannot see WSL's own `~/.cache` lane caches (hence the branch this sits in), but it can always
/// see this file, so it is the one WSL-side hint the Windows leg of `doctor` can offer.
#[cfg(windows)]
fn vhdxs_under(packages: &Path) -> Vec<(String, u64)> {
    let Ok(entries) = std::fs::read_dir(packages) else {
        return Vec::new();
    };
    let mut found: Vec<(String, u64)> = entries
        .flatten()
        .filter_map(|entry| {
            let vhdx = entry.path().join("LocalState").join("ext4.vhdx");
            let bytes = std::fs::symlink_metadata(&vhdx).ok()?.len();
            Some((
                format!("{}/LocalState/ext4.vhdx", file_name(&entry.path())),
                bytes,
            ))
        })
        .collect();
    found.sort();
    found
}

#[cfg(windows)]
fn wsl_vhdxs() -> Vec<(String, u64)> {
    let Some(local_appdata) = std::env::var_os("LOCALAPPDATA") else {
        return Vec::new();
    };
    vhdxs_under(&PathBuf::from(local_appdata).join("Packages"))
}

/// The last two path components — enough to tell two worktrees apart without a column of
/// identical prefixes drowning the numbers.
fn short(path: &Path) -> String {
    let mut parts: Vec<String> = path
        .components()
        .rev()
        .take(2)
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    parts.reverse();
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::{
        cache_state, children_with_sizes, kept_files, parse_worktree_list, short, tree_size,
        tree_size_excluding,
    };
    use std::collections::BTreeSet;
    use std::path::Path;

    /// Real `git worktree list --porcelain` output, trimmed to the shapes that matter: a
    /// detached checkout, a branch, and a lock.
    const PORCELAIN: &str = "\
worktree C:/repo
HEAD abc123
branch refs/heads/ai/main

worktree C:/repo/.claude/worktrees/agent-one
HEAD def456
detached

worktree C:/repo/.claude/worktrees/agent-two
HEAD 789abc
branch refs/heads/ai/lane
locked
";

    #[test]
    fn the_porcelain_attributes_land_on_the_record_they_belong_to() {
        // The parse is positional — attributes follow the `worktree` line that opens their
        // record — so a lock read onto the wrong tree is exactly how a reap kills live work,
        // and a HEAD read onto the wrong tree answers "has this landed" about someone else.
        let trees = parse_worktree_list(PORCELAIN);
        assert_eq!(trees.len(), 3);
        assert_eq!(trees[0].branch, "ai/main");
        assert_eq!(trees[0].head, "abc123");
        assert!(!trees[0].locked);
        assert_eq!(trees[1].branch, "detached");
        assert_eq!(trees[1].head, "def456");
        assert!(!trees[1].locked);
        assert_eq!(trees[2].branch, "ai/lane");
        assert_eq!(trees[2].head, "789abc");
        assert!(trees[2].locked, "the lock belongs to the tree above it");
    }

    #[test]
    fn a_cache_is_orphaned_only_when_its_key_names_no_live_worktree() {
        // The orphan verdict is the one thing here a reader might act destructively on, so each
        // family's key must come out of the name exactly, and a name with no derivable key at
        // all must not round down to "orphan".
        let live: BTreeSet<String> = ["agent-one".to_owned()].into_iter().collect();
        assert_eq!(cache_state("dorc-wsl-target-agent-one", &live), "live");
        assert_eq!(cache_state("dorc-wsl-target-agent-gone", &live), "ORPHAN");
        assert_eq!(cache_state("dorc-minispec-lean-agent-one", &live), "live");
        assert_eq!(cache_state("dorc-minispec-lean-old", &live), "ORPHAN");
        assert_eq!(
            cache_state("dorc-kani-target", &live),
            "shared",
            "the kani root is keyed to no worktree and cannot be orphaned"
        );
        assert_eq!(cache_state("dorc-something-new", &live), "unkeyed");
    }

    #[test]
    fn doctor_never_gains_the_power_to_delete() {
        // Automatic reaping was ruled out, permanently: it needs eyes in the loop. That ruling
        // is only worth as much as its enforcement, and a future edit adding a tidy-up here
        // would look entirely reasonable in review.
        let source = include_str!("doctor.rs");
        let production = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(before, _)| before);
        for banned in [
            "remove_dir_all",
            "remove_file",
            "remove_dir",
            "File::create",
            "fs::write",
            "\"prune\"",
            "\"-D\"",
            "\"--force\"",
        ] {
            assert!(
                !production.contains(banned),
                "{banned} would make this report destructive; it reports and nothing else"
            );
        }
    }

    #[test]
    fn a_stray_looks_away_from_its_build_cache_and_git_pointer_only() {
        // The skip list is the one place the stray check looks away, so it must not match by
        // name alone: a skipped file would let a directory holding unique work read `empty`.
        let root =
            std::env::temp_dir().join(format!("dorc-doctor-stray-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("spike/target/debug")).expect("scratch dir");
        std::fs::create_dir_all(root.join("notes/target")).expect("scratch dir");
        std::fs::write(root.join("spike/target/debug/dorc"), "built").expect("scratch file");
        std::fs::write(root.join(".git"), "gitdir: gone").expect("scratch file");
        let cache_only = kept_files(&root);
        std::fs::write(root.join("notes/target/plan.md"), "work").expect("scratch file");
        let with_work = kept_files(&root);
        let _ = std::fs::remove_dir_all(&root);

        assert_eq!(cache_only, Some(Vec::new()));
        assert_eq!(with_work, Some(vec![root.join("notes/target/plan.md")]));
    }

    #[test]
    fn a_nested_worktree_is_not_billed_to_the_tree_containing_it() {
        // The fleet lives INSIDE the primary checkout, so without a pruning boundary every
        // lane's target dir is counted twice and the total is nonsense.
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let whole = tree_size(&src);
        let pruned = tree_size_excluding(&src, &[src.join("doctor.rs")]);
        assert!(pruned < whole, "an excluded path must stop being counted");
        assert_eq!(
            tree_size_excluding(&src, std::slice::from_ref(&src)),
            whole,
            "the root itself is never its own boundary"
        );
    }

    #[test]
    fn a_size_walk_counts_a_known_tree() {
        // This crate's own sources: small, always present, and non-zero.
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        assert!(tree_size(&src) > 0, "a tree of sources cannot measure zero");
    }

    #[test]
    fn a_missing_directory_inventories_as_empty_rather_than_panicking() {
        let absent = dorc_testbed::repo_root().join("no-such-store");
        assert!(children_with_sizes(&absent).is_empty());
        assert_eq!(tree_size(&absent), 0);
    }

    #[test]
    fn paths_shorten_to_their_tail() {
        assert_eq!(short(Path::new("/a/b/c/d")), "c/d");
    }

    /// Windows keeps each WSL distribution's disk image below the user's `Packages` directory.
    #[cfg(windows)]
    mod on_windows {
        use super::super::vhdxs_under;

        #[test]
        fn a_vhdx_is_found_and_sized_under_its_packages_entry() {
            let root =
                std::env::temp_dir().join(format!("dorc-doctor-vhdx-test-{}", std::process::id()));
            let local_state = root
                .join("Packages")
                .join("SomeDistro_abc123")
                .join("LocalState");
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&local_state).expect("scratch dir");
            std::fs::write(local_state.join("ext4.vhdx"), [0_u8; 4096]).expect("scratch vhdx");

            let found = vhdxs_under(&root.join("Packages"));
            let _ = std::fs::remove_dir_all(&root);

            assert_eq!(found.len(), 1);
            assert!(found[0].0.contains("SomeDistro_abc123"));
            assert_eq!(found[0].1, 4096);
        }

        #[test]
        fn an_absent_packages_directory_finds_nothing() {
            let absent = dorc_testbed::repo_root().join("no-such-packages-dir");
            assert!(vhdxs_under(&absent).is_empty());
        }
    }
}
