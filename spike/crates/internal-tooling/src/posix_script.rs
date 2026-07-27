//! Run a repo-relative POSIX script, and get out of the way.
//!
//! This exists only because a mise task may not spell `sh <script>`: native Windows ships no POSIX
//! shell and git's is deliberately kept off PATH (`mise.toml`'s no-`sh`-in-tasks note), so the ONE
//! answer to "where is a shell" has to be asked in Rust. Everything the scripts actually do lives
//! in the scripts.

use internal_tooling::Posix;
use std::ffi::OsString;
use std::process::{Command, ExitCode};

/// Run `rel` under a located POSIX shell, forwarding `args` and adding `env`.
pub(crate) fn run(label: &str, rel: &str, args: &[String], env: &[(&str, OsString)]) -> ExitCode {
    let posix = match Posix::find() {
        Ok(found) => found,
        Err(why) => {
            eprintln!("{label}: no POSIX shell — {why}");
            return ExitCode::from(3);
        }
    };
    let script = internal_tooling::repo_root().join(rel);
    if !script.is_file() {
        eprintln!("{label}: no script at {}", script.display());
        return ExitCode::from(3);
    }

    let mut cmd = Command::new(&posix.shell);
    cmd.arg(&script).args(args).env("PATH", posix.child_path());
    for (key, value) in env {
        cmd.env(key, value);
    }
    match cmd.status() {
        // The script's own exit code is the verdict and is reproduced, never re-interpreted: a
        // baseline mismatch and a missing runtime are different answers and must stay different.
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
        Err(e) => {
            eprintln!("{label}: could not run {}: {e}", posix.shell.display());
            ExitCode::from(3)
        }
    }
}
