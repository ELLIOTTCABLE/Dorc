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
    remote_sh: String,
}

impl LocalDriver {
    /// A driver over `shell`, which also plays the part of the remote interpreter.
    #[must_use]
    pub fn new(shell: PathBuf) -> Self {
        let remote_sh = shell.to_string_lossy().into_owned();
        Self { shell, remote_sh }
    }
}

impl SessionDriver for LocalDriver {
    fn run(&mut self, request: &SessionRequest<'_>) -> SessionOutcome {
        let mut command = Command::new(&self.shell);
        command
            .arg("-c")
            .arg(request.marker.remote_command(&self.remote_sh));
        crate::child::run(command, request)
    }
}
