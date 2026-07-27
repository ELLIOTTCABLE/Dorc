//! The hermetic driver: a local POSIX shell standing in for the remote one.
//!
//! Its whole value is that it is NOT a separate implementation. It builds the same remote
//! command line, ships the artifact the same way on stdin, and classifies through the same
//! marker scan and the same capture code — only the spawned program differs (`260` §5, driver
//! 2). A green hermetic run is therefore evidence about the shipping path.
//!
//! The shell is supplied by the caller rather than resolved here, because this repo has exactly
//! one answer to "where is a POSIX shell" (`one-shell-answer`, `internal_tooling::Posix::find`)
//! and a second resolver is how the first one silently rots.

use crate::{SessionDriver, SessionOutcome, SessionRequest};
use std::path::PathBuf;
use std::process::Command;

/// Runs an artifact through a local shell, over the production code path.
#[derive(Debug, Clone)]
pub struct LocalDriver {
    shell: PathBuf,
    interpreter: String,
}

impl LocalDriver {
    /// A driver over `shell`, which also plays the part of the remote interpreter.
    ///
    /// Two spellings of one shell, because the two consumers do not agree on what a path is: the
    /// OS spawns `shell` and needs its native form, while `interpreter` is re-resolved INSIDE that
    /// shell and needs a form that shell understands. On Windows those genuinely differ — msys
    /// `dash` cannot exec `C:\Program Files\...\dash.exe`, and `/usr/bin/dash` is not a thing
    /// `CreateProcess` can start. Collapsing them works everywhere except the platform this repo
    /// is developed on.
    #[must_use]
    pub fn new(shell: PathBuf, interpreter: String) -> Self {
        Self { shell, interpreter }
    }

    /// A driver whose interpreter spelling is its own path — correct wherever the OS and the
    /// shell agree about paths, which is everywhere but Windows.
    #[must_use]
    pub fn same_spelling(shell: PathBuf) -> Self {
        let interpreter = shell.to_string_lossy().into_owned();
        Self::new(shell, interpreter)
    }
}

impl SessionDriver for LocalDriver {
    fn run(&mut self, request: &SessionRequest<'_>) -> SessionOutcome {
        // The interpreter arrives as `$0` rather than interpolated into the command line, because
        // a path is DATA and a command line is CODE: `C:\Program Files\Git\usr\bin\dash.exe`
        // word-splits into `C:Program` the moment it is pasted into a shell string. Passing it as
        // an argument lets the OS carry it across intact and the shell quote it for us.
        let mut command = Command::new(&self.shell);
        command
            .arg("-c")
            .arg(request.marker.remote_command("\"$0\""))
            .arg(&self.interpreter);
        crate::child::run(command, request)
    }
}
