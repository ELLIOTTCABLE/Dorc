//! Internal repo tooling — NOT part of Dorc. See `Cargo.toml` for why this crate exists.
//!
//! [`target_dir`] answers where cargo puts build output. The POSIX-shell seat now lives in
//! `dorc_transport` (`one-shell-answer`, `dorc_transport::Posix::find`) and the xfail-pin seat with
//! `repo_root` in `dorc_testbed`, the shared test substrate — so this crate, the xtask binary's
//! plumbing, keeps no dependents.

use std::path::PathBuf;

/// Where cargo puts build output — the ONE answer, for the reason `Posix::find` is the one
/// answer about shells.
///
/// `spike/target` is only the DEFAULT, and the WSL leg no longer uses it: the root `mise.toml`
/// redirects the target dir off drvfs there, so anything re-deriving the default looks in an
/// empty directory and reports a missing binary as an absent feature. Read this instead.
#[must_use]
pub fn target_dir() -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR").map_or_else(
        || dorc_testbed::repo_root().join("spike").join("target"),
        PathBuf::from,
    )
}
