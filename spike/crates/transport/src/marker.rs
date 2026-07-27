//! The completion sentinel (`26A` stop-2) — the one thing that decides whether a session
//! finished.

/// The end-marker's fixed tag. Distinct from the records lane's `@@dorc@@` terminal token on
/// purpose: this is TRANSPORT framing, added and removed by this crate, and the records grammar
/// must never see it. Sharing a token would make one layer's tear look like the other's.
const SESSION_TAG: &str = "dorc-session/1";

/// The marker line's terminal token.
const SESSION_TOKEN: &str = "@@dorc-session@@";

/// Why a marker could not be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerRejected {
    /// The nonce was empty.
    EmptyNonce,
    /// The nonce carried something other than ASCII alphanumerics. The marker is emitted inside
    /// a single-quoted `printf` format in a command line the remote shell evaluates, so a nonce
    /// carrying a quote or a shell metacharacter would be a command-injection seat. Restricting
    /// the charset is cheaper and more obviously correct than quoting it.
    NonceNotAlphanumeric,
}

/// The controller-authored completion marker for one attempt.
///
/// Carries the run nonce and attempt counter so the marker is unique to THIS attempt: a zombie
/// writer from a killed earlier attempt cannot satisfy the current one, which is the same
/// mechanism `attempt=` gives the records lane (`26A` amend-retry-hygiene).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionMarker {
    nonce: String,
    attempt: u32,
}

impl SessionMarker {
    /// Build the marker for one attempt.
    ///
    /// # Errors
    /// [`MarkerRejected`] when the nonce is empty or not ASCII-alphanumeric.
    pub fn new(nonce: &str, attempt: u32) -> Result<Self, MarkerRejected> {
        if nonce.is_empty() {
            return Err(MarkerRejected::EmptyNonce);
        }
        if !nonce.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(MarkerRejected::NonceNotAlphanumeric);
        }
        Ok(Self {
            nonce: nonce.to_owned(),
            attempt,
        })
    }

    /// The literal prefix every marker line for this attempt begins with.
    fn line_prefix(&self) -> String {
        format!("{} {SESSION_TAG} attempt={} rc=", self.nonce, self.attempt)
    }

    /// The exact line the remote `printf` produces for `status`, newline included.
    ///
    /// The simulated driver builds its wire bytes with this rather than with a hand-written
    /// literal, so a change to the marker's shape cannot leave the DST tier asserting against a
    /// format nothing emits any more.
    #[must_use]
    pub fn marker_line(&self, status: i32) -> String {
        format!("{}{status} {SESSION_TOKEN}\n", self.line_prefix())
    }

    /// The remote command line that runs the artifact and then reports its status.
    ///
    /// `remote_sh -s` reads the artifact from stdin, so the artifact's own bytes are never
    /// touched (`law-artifact-floor`) and the marker rides OUTSIDE them. `-T` (no pty) is the
    /// caller's business; without it a pty would cook and merge these streams (`notes/140` f5).
    ///
    /// The leading newline guarantees the marker starts its own line even when the artifact's
    /// last write had no trailing newline; [`strip_from`](Self::strip_from) removes exactly that
    /// byte again, so the caller's stdout is byte-identical to what the artifact wrote.
    #[must_use]
    pub fn remote_command(&self, remote_sh: &str) -> String {
        format!(
            "{remote_sh} -s; printf '\\n{} {SESSION_TAG} attempt={} rc=%s {SESSION_TOKEN}\\n' \"$?\"",
            self.nonce, self.attempt
        )
    }

    /// Find this attempt's marker in captured stdout.
    ///
    /// Returns the carried status and the byte offset at which the artifact's own output ended,
    /// or `None` when the marker is absent — which is the whole classification (`26A` stop-2).
    /// Searches from the END so trailing output wins; a marker is only honoured when it is a
    /// whole line, so a fragment glued into other output cannot satisfy a session.
    #[must_use]
    pub fn scan(&self, stdout: &[u8]) -> Option<Completion> {
        let text = std::str::from_utf8(stdout).ok()?;
        let prefix = self.line_prefix();
        let mut found: Option<(usize, &str)> = None;
        let mut offset = 0_usize;
        for raw in text.split_inclusive('\n') {
            let line = raw.strip_suffix('\n').unwrap_or(raw);
            if line.starts_with(&prefix) && line.ends_with(SESSION_TOKEN) {
                found = Some((offset, line));
            }
            offset = offset.saturating_add(raw.len());
        }
        let (offset, line) = found?;

        let body = line.get(prefix.len()..line.len().saturating_sub(SESSION_TOKEN.len()))?;
        let status: i32 = body.trim().parse().ok()?;
        Some(Completion {
            status,
            artifact_stdout_len: offset.saturating_sub(1),
        })
    }

    /// Cut a marker line out of captured stdout, leaving exactly the artifact's own bytes.
    #[must_use]
    pub fn strip_from(stdout: &[u8], completion: &Completion) -> Vec<u8> {
        stdout
            .get(..completion.artifact_stdout_len)
            .unwrap_or(stdout)
            .to_vec()
    }
}

/// A located completion marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Completion {
    /// The status the marker carried — the remote artifact's genuine exit status.
    pub status: i32,
    /// Where the artifact's own stdout ended, excluding the newline this crate added.
    pub artifact_stdout_len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_charset_is_enforced_because_the_marker_rides_a_shell_command_line() {
        assert_eq!(SessionMarker::new("", 1), Err(MarkerRejected::EmptyNonce));
        assert_eq!(
            SessionMarker::new("a'; rm -rf /; echo '", 1),
            Err(MarkerRejected::NonceNotAlphanumeric)
        );
        assert!(SessionMarker::new("ab12", 1).is_ok());
    }

    #[test]
    fn a_completed_session_reconstructs_artifact_stdout_byte_for_byte() {
        let marker = SessionMarker::new("n1", 3).expect("valid nonce");
        for artifact in ["", "foo", "foo\n", "a\nb\n", "trailing\n\n"] {
            let wire = format!("{artifact}\nn1 dorc-session/1 attempt=3 rc=0 @@dorc-session@@\n");
            let found = marker.scan(wire.as_bytes()).expect("marker present");
            assert_eq!(found.status, 0);
            assert_eq!(
                SessionMarker::strip_from(wire.as_bytes(), &found),
                artifact.as_bytes(),
                "stdout must survive the round trip unchanged for {artifact:?}"
            );
        }
    }

    #[test]
    fn a_nonzero_remote_status_is_carried_not_inferred() {
        let marker = SessionMarker::new("n1", 1).expect("valid nonce");
        let wire = b"out\nn1 dorc-session/1 attempt=1 rc=255 @@dorc-session@@\n";
        let found = marker.scan(wire).expect("marker present");
        assert_eq!(
            found.status, 255,
            "255 is the collision ssh's own rc cannot resolve; the marker resolves it"
        );
    }

    #[test]
    fn a_marker_from_another_attempt_or_run_does_not_satisfy_this_one() {
        let marker = SessionMarker::new("n1", 2).expect("valid nonce");
        for foreign in [
            "n1 dorc-session/1 attempt=1 rc=0 @@dorc-session@@\n",
            "n2 dorc-session/1 attempt=2 rc=0 @@dorc-session@@\n",
        ] {
            assert!(
                marker.scan(foreign.as_bytes()).is_none(),
                "a stale attempt or foreign run must read as loss, not completion: {foreign}"
            );
        }
    }

    #[test]
    fn a_marker_glued_into_other_output_is_not_a_whole_line_and_does_not_count() {
        let marker = SessionMarker::new("n1", 1).expect("valid nonce");
        let glued = b"noise n1 dorc-session/1 attempt=1 rc=0 @@dorc-session@@\n";
        assert!(marker.scan(glued).is_none());
    }

    #[test]
    fn truncated_output_with_no_marker_is_loss() {
        let marker = SessionMarker::new("n1", 1).expect("valid nonce");
        assert!(
            marker
                .scan(b"partial records, then the wire died")
                .is_none()
        );
    }
}
