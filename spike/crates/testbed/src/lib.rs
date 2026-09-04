//! The suite's shared substrate — NOT part of Dorc. See `Cargo.toml` for the charter.
//!
//! Dependency-free (std only) and sited below `cli`, so both `harness = false` runners, the loom
//! driver, and product-crate tests all reach it. [`seam_vars`] is the seam-variable vocabulary (the
//! `DORC_SEED`/`DORC_SEAM_*` names; `cli::seam` owns the parser and imports these). [`run_seed`] is
//! the ONE run-seed seat — "what is this run's seed", drawn once, printed and named the same way
//! everywhere. [`xfail`] is the workspace's ONE xfail-pin seat and its census
//! (`spike/CLAUDE.md xfail-pins-ride-one-seat`); it and [`repo_root`] sit here so every crate can
//! dev-depend one seat and a second copy cannot silently rot.

use std::path::Path;

pub mod run_seed;
pub mod seam_vars;
pub mod xfail;

/// The worktree root, from this crate's compile-time location (`<root>/spike/crates/…`).
///
/// The one answer to "where is the repo", for the reason [`run_seed`] is the one answer about seeds:
/// [`xfail`]'s corpus walk resolves from it, and the shared test substrate is where it belongs so no
/// consumer re-derives it.
#[must_use]
pub fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap_or(Path::new("."))
}
