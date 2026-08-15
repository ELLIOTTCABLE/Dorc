//! A read-only inventory of what is quietly eating the disk.
//!
//! The observed pattern is pile-up, not any single hog: nineteen abandoned lane caches held
//! 51 GiB inside an 87.9 GiB vhdx, and finished agent worktrees carried ~9 GiB of `target/`
//! apiece. None of it is visible until something runs out, which is why this exists — the
//! numbers, in one place, before they bite.
//!
//! It DELETES NOTHING, on purpose. Reaping a worktree needs a containment proof this tool
//! has no business making, and reaping a cache is cheap to do by hand once you can see it.
//!
//! Each leg sees only its own filesystem: the Windows leg cannot see WSL's `~/.cache`, and
//! the WSL leg reaches the worktrees over `/mnt/c`. `mise run both doctor` is the paired
//! form and the one that shows the whole picture.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use crate::preflight::gib;

/// Walk every store, print a row each, then the total.
pub(crate) fn run() -> ExitCode {
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
        None => println!("  (no XDG_CACHE_HOME or HOME — Windows keeps none of these)"),
        Some(cache) => {
            let mut rows = children_with_sizes(&cache);
            rows.retain(|(path, _)| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("dorc-"))
            });
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

/// One checkout git knows about.
#[derive(Debug)]
struct Tree {
    path: PathBuf,
    branch: String,
    /// `clean`, `dirty`, or why we could not tell — never guessed.
    state: String,
    /// A locked worktree belongs to a live lane. Reaping one is how concurrent work dies.
    locked: bool,
}

/// Every worktree, from git rather than from a glob: a glob finds directories, while git
/// finds the ones that are actually registered, and a stale directory is a different
/// problem with a different fix (`git worktree prune`).
fn worktrees() -> Result<Vec<Tree>, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(internal_tooling::repo_root())
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_owned());
    }

    let mut trees: Vec<Tree> = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            let path = PathBuf::from(path);
            let state = tree_state(&path);
            trees.push(Tree {
                path,
                branch: "detached".to_owned(),
                state,
                locked: false,
            });
        } else if let Some(branch) = line.strip_prefix("branch ")
            && let Some(tree) = trees.last_mut()
        {
            branch
                .trim_start_matches("refs/heads/")
                .clone_into(&mut tree.branch);
        } else if (line.trim() == "locked" || line.starts_with("locked "))
            && let Some(tree) = trees.last_mut()
        {
            tree.locked = true;
        }
    }
    Ok(trees)
}

/// Clean or dirty, by git's own answer — untracked files included, because they are exactly
/// what a reap would destroy.
fn tree_state(path: &Path) -> String {
    let Ok(out) = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(["status", "--porcelain"])
        .output()
    else {
        return "unknown".to_owned();
    };
    if !out.status.success() {
        return "unknown".to_owned();
    }
    if out.stdout.is_empty() {
        "clean".to_owned()
    } else {
        "DIRTY".to_owned()
    }
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
    use super::{children_with_sizes, short, tree_size, worktrees};

    #[test]
    fn the_inventory_finds_at_least_this_checkout() {
        // A worktree walk that silently answers zero is the failure mode worth pinning:
        // an empty inventory reads as "nothing at risk" rather than "the walk broke".
        let found = worktrees().expect("git knows about this checkout");
        assert!(!found.is_empty(), "the running checkout must appear");
    }

    #[test]
    fn a_size_walk_counts_a_known_tree() {
        // This crate's own sources: small, always present, and non-zero.
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        assert!(tree_size(&src) > 0, "a tree of sources cannot measure zero");
    }

    #[test]
    fn a_missing_directory_inventories_as_empty_rather_than_panicking() {
        let absent = internal_tooling::repo_root().join("no-such-store");
        assert!(children_with_sizes(&absent).is_empty());
        assert_eq!(tree_size(&absent), 0);
    }

    #[test]
    fn paths_shorten_to_their_tail() {
        assert_eq!(short(std::path::Path::new("/a/b/c/d")), "c/d");
    }
}
