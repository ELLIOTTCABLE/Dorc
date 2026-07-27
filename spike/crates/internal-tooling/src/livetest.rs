//! `livetest` — hand the live-acceptance script a POSIX shell and get out of the way.
//!
//! This exists only because a mise task may not spell `sh e2e/livetest.sh`: native Windows ships
//! no POSIX shell and git's is deliberately kept off PATH (`mise.toml`'s no-`sh`-in-tasks note),
//! so the ONE answer to "where is a shell" has to be asked in Rust. Everything the live loop
//! actually does lives in the script; nothing here knows what a container is.

use internal_tooling::Posix;
use std::process::{Command, ExitCode};

/// Run `spike/e2e/livetest.sh`, forwarding every argument.
pub(crate) fn run(args: &[String]) -> ExitCode {
    let posix = match Posix::find() {
        Ok(found) => found,
        Err(why) => {
            eprintln!("livetest: no POSIX shell — {why}");
            return ExitCode::from(3);
        }
    };
    let script = internal_tooling::repo_root().join("spike/e2e/livetest.sh");
    if !script.is_file() {
        eprintln!("livetest: no script at {}", script.display());
        return ExitCode::from(3);
    }

    let status = Command::new(&posix.shell)
        .arg(&script)
        .args(args)
        .env("PATH", posix.child_path())
        .status();
    match status {
        // The script's own exit code is the verdict and is reproduced, never re-interpreted: a
        // baseline mismatch and a missing runtime are different answers and must stay different.
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
        Err(e) => {
            eprintln!("livetest: could not run {}: {e}", posix.shell.display());
            ExitCode::from(3)
        }
    }
}
