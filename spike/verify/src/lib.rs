//! `dorc-verify` — the binder over the minispec law corpus (`Research/notes/301`).
//!
//! minispec is a deliberately tiny corpus of literate Lean law-units: the project's reviewable
//! statement of the few kernel laws it opts to verify. This crate computes each law's earned
//! coverage badges from machine-checkable evidence and emits the one generated report. Both
//! are internal instruments — for wrangling LLM maintainers, and for reviewers who are not
//! proof-literate. Neither is ever user-facing.
//!
//! # The one thing to understand before touching this
//!
//! The binder is an EXTERNAL acceptance surface. Its whole value rests on the maintainers it
//! judges being unable to write to what it judges: unit content is frontier-authored under
//! `301:law-spec-touch-frontier-human-only`, and an acceptance surface the worker cannot write
//! to cannot be gamed by the worker. If a check here is in your way, the two correct moves are
//! to fix the code or to escalate with the failure in hand — never to widen the check.
//!
//! # Gate tiers
//!
//! Cheap checks (unit parse, slug discipline, citation resolution, catalogue coherence, the
//! hole census, the byte-budget tripwire) ride the ordinary gate on both platform legs with
//! zero external toolchains. Evidence that needs Lean, Kani or `cargo-mutants` is recomputed
//! in the opt-in verify lane at the fold/bless tier. Never a hand-updated cache: a tier that
//! cannot look says so.

pub mod badge;
pub mod binding;
pub mod catalogue;
pub mod catalogue_lock;
pub mod check;
pub mod evidence;
pub mod kani;
pub mod pipeline;
pub mod promote;
pub mod report;
pub mod seat;
pub mod unit;

use std::path::{Path, PathBuf};

/// The worktree root, from this crate's compile-time location (`<root>/spike/verify`).
#[must_use]
pub fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap_or(Path::new("."))
}

/// The staged Lean build root: user-local, ext4, deliberately outside the synced tree.
///
/// Per-WORKTREE, for the reason `mise.toml` gives `CARGO_TARGET_DIR` the same treatment: the
/// fleet lives in `.claude/worktrees/*`, and one shared root means this worktree builds the UNION
/// of its own corpus and whatever a sibling left behind — a channel that carried a real green
/// build over a unit the repository had deleted (`30B:fnd-lean-staging-never-removes-stale-files`).
/// The key is the worktree directory's own name, exactly as that file spells it.
/// `internal-tooling`'s preflight re-derives this path by rule; the two move together.
#[must_use]
pub fn lean_build_root() -> PathBuf {
    std::env::var_os("DORC_MINISPEC_BUILD_ROOT").map_or_else(
        || {
            let cache = std::env::var_os("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
                .unwrap_or_else(std::env::temp_dir);
            cache.join(format!("dorc-minispec-lean-{}", worktree_key()))
        },
        PathBuf::from,
    )
}

/// `path`, spelled the one way anything the binder PUBLISHES is allowed to spell it:
/// repo-relative, forward slashes.
///
/// The committed report is byte-compared against a fresh render, so an absolute path in it is
/// not a cosmetic wart — it pins the artifact to the worktree that generated it, and every
/// other checkout of this repository then reads as stale forever. Every published path goes
/// through here; a path that cannot be made relative is rendered as it is rather than silently
/// dropped.
#[must_use]
pub fn relative(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// This worktree's own name, the suffix that keeps one lane's caches out of another's.
#[must_use]
pub fn worktree_key() -> String {
    repo_root()
        .file_name()
        .map_or_else(|| "root".to_owned(), |name| name.to_string_lossy().into())
}
