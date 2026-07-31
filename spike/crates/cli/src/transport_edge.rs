//! The transport edge: where a run acquires a host, and the only place this binary decides
//! anything about reaching one.
//!
//! Everything here is I/O-side by construction (`io-at-edges-only`). The analysis pipeline never
//! learns that a host exists: it is handed evidence bytes exactly as it would be handed the
//! contents of `--results`, and it cannot tell which happened.

use dorc_aid::diag::{Diag, DiagCode};
use dorc_plan::records::{Framing, Nonce, RemoteIdentity};
use dorc_transport::{
    HostId, LocalDriver, Phase, SessionDriver, SessionMarker, SessionOutcome, SessionRequest,
    SshDriver, SshOptions, TransportDiagnosis,
};
use std::path::PathBuf;
use std::time::Duration;

/// Selects a non-ssh driver, for the acceptance tiers that must exercise the shipping path
/// without a network (`26D` §5 T1/T2).
///
/// Read only in debug builds. A release binary has no code path from this variable to a driver
/// choice at all, which matters because the hazard is mis-attribution rather than mere
/// weirdness: a run that believes it reached `web1` while executing locally would write `web1`
/// into its records and its receipt (`271:rul-sin-ordering` puts mis-attributed error at the
/// top). Selecting it also announces itself, so a transcript never quietly says "host" and means
/// "here".
const TRANSPORT_ENV: &str = "DORC_TRANSPORT";

/// The local shell's own name for itself, when it differs from the OS's name for it.
const TRANSPORT_INTERPRETER_ENV: &str = "DORC_TRANSPORT_INTERPRETER";

/// Pins the run nonce so a committed transcript can be a fixpoint, exactly as
/// `DORC_FIXTURE_CLOCK_MS` pins the clock. No normalizer may touch rendered output
/// (`seam-tolerated-nondeterminism-stops-at-the-run-log`), so pinning the source is the only way
/// a driven transcript is reproducible.
const FIXTURE_NONCE_ENV: &str = "DORC_FIXTURE_NONCE";

/// How many times a probe may be re-shipped after a transport loss (`260` dec-26-probe-retry).
///
/// A probe is read-only BY CONTRACT, so re-running one cannot double anything; an apply has no
/// such licence and is never retried (`law-no-double-apply`). The asymmetry is `kFAIL`
/// phase-keying spelled at the transport.
const PROBE_RETRIES: u32 = 2;

/// The run's nonce, minted once at the edge.
///
/// Unique, not unpredictable — and the distinction is deliberate. The host is HANDED this nonce
/// (it is baked into the artifact we ship it), so secrecy against the host is not a property
/// this value could have. What it must do is separate one attempt from another and one run from
/// another, so a killed attempt's zombie writer and a mis-plumbed stream fail to parse rather
/// than fold. Clock and pid give that without a dependency.
///
/// This reasoning stops holding if records from several sessions ever share one channel; at that
/// point unpredictability starts to matter and this needs revisiting.
pub(crate) fn mint_nonce() -> String {
    match std::env::var(FIXTURE_NONCE_ENV) {
        Ok(raw) if usable_pinned_nonce(&raw) => raw,
        _ => minted_nonce(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos()),
            std::process::id(),
        ),
    }
}

/// Whether a pinned nonce could survive the marker's charset. One that could not is IGNORED
/// rather than fatal: the pin is a harness convenience, and a harness typo should cost
/// determinism, never correctness.
fn usable_pinned_nonce(raw: &str) -> bool {
    !raw.is_empty() && raw.chars().all(|c| c.is_ascii_alphanumeric())
}

/// The derivation, kept a pure function of its inputs so it is testable without mutating the
/// process environment — which `unsafe_code = "forbid"` makes impossible in a test anyway.
fn minted_nonce(nanos: u128, pid: u32) -> String {
    let mixed = u64::try_from(nanos & u128::from(u64::MAX)).unwrap_or(0) ^ (u64::from(pid) << 32);
    format!("r{mixed:016x}")
}

/// What the caller asked for, resolved into a driver.
pub(crate) fn driver_for_invocation(
    connect_timeout: Option<u64>,
    accept_new: bool,
    ssh_config: Option<&str>,
) -> Box<dyn SessionDriver> {
    if cfg!(debug_assertions)
        && let Ok(spec) = std::env::var(TRANSPORT_ENV)
        && let Some(shell) = spec.strip_prefix("local:")
    {
        eprintln!("dorc: {TRANSPORT_ENV}=local — running through a local shell, not ssh");
        return Box::new(match std::env::var(TRANSPORT_INTERPRETER_ENV) {
            Ok(interpreter) if !interpreter.is_empty() => {
                LocalDriver::new(PathBuf::from(shell), interpreter)
            }
            _ => LocalDriver::same_spelling(PathBuf::from(shell)),
        });
    }
    let mut options = SshOptions {
        accept_new_host_key: accept_new,
        config_file: ssh_config.map(PathBuf::from),
        ..SshOptions::default()
    };
    if let Some(secs) = connect_timeout {
        options.connect_timeout = Duration::from_secs(secs);
    }
    Box::new(SshDriver::new(options))
}

/// What a probe shipment produced.
pub(crate) enum ProbeShipment {
    /// The artifact ran to completion and these are the bytes it wrote.
    Captured {
        /// Exactly the artifact's stdout, ready for the bounded intake.
        stdout: Vec<u8>,
        /// The framing the WINNING attempt was rendered with. Admission checks incoming records
        /// against this one, so a superseded attempt's records cannot satisfy it.
        framing: Framing,
        /// Whatever the artifact wrote to stderr.
        stderr: Vec<u8>,
    },
    /// Every attempt lost the channel. The world is unknown, and no plan may claim otherwise.
    Lost {
        /// The last attempt's diagnosis, for the operator.
        diagnosis: TransportDiagnosis,
        /// How many attempts were made.
        attempts: u32,
    },
    /// No process was ever created, so nothing was contacted.
    NotAttempted(NotAttempted),
}

/// Why a shipment never reached a host at all — the two worlds behind one untouched-host claim
/// (`296:tc-transport-not-attempted-is-two-worlds`). They mint different codes because the
/// operator's next move differs: fix the environment, or fix the invocation.
pub(crate) enum NotAttempted {
    /// The platform refused to create the session process; these are its own words.
    SpawnRefused(String),
    /// The run's nonce could not become a session marker, so nothing was ever shipped.
    MarkerUnusable,
}

/// Ship the probe, re-minting the attempt on each try.
///
/// The re-render is the point: `attempt=` is baked into the artifact's own bytes, so a retry
/// SHIPS A DIFFERENT ARTIFACT and the previous attempt's records become un-foldable by their
/// stale key rather than by anyone remembering to discard them (`26A` amend-retry-hygiene).
/// A retry loop inside the driver could not do this, which is why it lives here.
pub(crate) fn ship_probe(
    driver: &mut dyn SessionDriver,
    host: &HostId,
    nonce: &str,
    book_digest: &str,
    timeout: Option<Duration>,
    render: &dyn Fn(&Framing) -> String,
) -> ProbeShipment {
    let mut last = TransportDiagnosis::ChildLost;
    let mut attempts = 0;
    for attempt in 1..=PROBE_RETRIES.saturating_add(1) {
        attempts = attempt;
        let identity =
            RemoteIdentity::new(Nonce(nonce.to_owned()), attempt, host.as_str().to_owned());
        let framing = Framing::for_remote(&identity, book_digest.to_owned());
        let artifact = render(&framing);
        let Ok(marker) = SessionMarker::new(nonce, attempt) else {
            return ProbeShipment::NotAttempted(NotAttempted::MarkerUnusable);
        };
        let outcome = driver.run(&SessionRequest {
            host,
            phase: Phase::Probe,
            artifact: artifact.as_bytes(),
            marker: &marker,
            timeout,
        });
        match outcome {
            SessionOutcome::Completed { stdout, stderr, .. } => {
                return ProbeShipment::Captured {
                    stdout,
                    framing,
                    stderr,
                };
            }
            SessionOutcome::NotAttempted { reason } => {
                return ProbeShipment::NotAttempted(NotAttempted::SpawnRefused(reason));
            }
            SessionOutcome::LostAfterSend { diagnosis, .. } => last = diagnosis,
        }
    }
    ProbeShipment::Lost {
        diagnosis: last,
        attempts,
    }
}

/// Ship an already-rendered apply artifact. Once.
///
/// There is no retry parameter and no loop, because there is no licence for one: under Unknown a
/// re-ship risks double-applying, and the sanctioned recovery is re-probe-then-re-plan — the
/// probe is the retry-file (`law-no-double-apply`).
pub(crate) fn ship_apply(
    driver: &mut dyn SessionDriver,
    host: &HostId,
    nonce: &str,
    artifact: &[u8],
    timeout: Option<Duration>,
) -> Result<SessionOutcome, NotAttempted> {
    let Ok(marker) = SessionMarker::new(nonce, 1) else {
        return Err(NotAttempted::MarkerUnusable);
    };
    Ok(driver.run(&SessionRequest {
        host,
        phase: Phase::Apply,
        artifact,
        marker: &marker,
        timeout,
    }))
}

/// Where a CR byte sits in bytes about to be shipped, as a 1-based line number.
///
/// The gate is on CR rather than on `\r\n` specifically: a lone CR line ending breaks a remote
/// shell exactly as thoroughly, and a literal CR byte in a source file is essentially always a
/// line-ending artifact rather than intent (`\r` in a script is the two characters backslash and
/// r, which this never sees).
///
/// Detection only. Rewriting the user's bytes to be helpful is the one thing this must not do:
/// silently changing what someone is about to run on a server trades a loud, one-line fix for an
/// invisible edit (`260` dec-26-crlf, never-lie over convenience).
pub(crate) fn first_carriage_return(bytes: &[u8]) -> Option<usize> {
    let mut line = 1_usize;
    for byte in bytes {
        match *byte {
            b'\r' => return Some(line),
            b'\n' => line = line.saturating_add(1),
            _ => {}
        }
    }
    None
}

/// Refuse a shipment whose bytes are not LF-only.
pub(crate) fn crlf_refusal(which: &str, line: usize) -> Diag {
    Diag::new_spanless_site(DiagCode::TransportCrlfRefused(
        dorc_aid::diag::TransportCrlfRefused {
            which: which.to_owned(),
            line: line.to_string(),
        },
    ))
}

/// How a remote apply ended, once classified.
pub(crate) enum AppliedOutcome {
    /// The artifact ran to completion and exited with this status.
    Ran {
        /// The status the marker carried, reproduced and never interpreted.
        status: i32,
    },
    /// The session was lost. The host's state is UNKNOWN — not applied, not un-applied.
    Unknown {
        /// What probably severed it, for the operator.
        diagnosis: TransportDiagnosis,
    },
    /// Nothing was contacted.
    NotAttempted(NotAttempted),
}

/// Ship an already-rendered apply artifact and classify the result.
///
/// Reads the artifact from `source`, re-checks it for carriage returns, ships it ONCE, and echoes
/// the host's streams through the shared display encoder.
///
/// The CRLF re-check is not redundant with the plan-time one: the bytes shipped here are a file
/// the user has had in their hands and may have edited on any OS, and no parser of ours has seen
/// them (`26A` amend-smalls).
#[expect(
    clippy::result_large_err,
    reason = "the Err is a full `Diag`, as everywhere on this once-per-process path"
)]
pub(crate) fn apply_to_host(
    driver: &mut dyn SessionDriver,
    host: &HostId,
    nonce: &str,
    artifact: &[u8],
    timeout: Option<Duration>,
) -> Result<AppliedOutcome, Diag> {
    if let Some(line) = first_carriage_return(artifact) {
        return Err(crlf_refusal("the plan", line));
    }
    let shipped = match ship_apply(driver, host, nonce, artifact, timeout) {
        Ok(outcome) => outcome,
        Err(why) => return Ok(AppliedOutcome::NotAttempted(why)),
    };
    match shipped {
        SessionOutcome::Completed {
            status,
            stdout,
            stderr,
        } => {
            echo(&stdout, false);
            echo(&stderr, true);
            Ok(AppliedOutcome::Ran { status })
        }
        SessionOutcome::LostAfterSend {
            stdout,
            stderr,
            diagnosis,
        } => {
            echo(&stdout, false);
            echo(&stderr, true);
            Ok(AppliedOutcome::Unknown { diagnosis })
        }
        SessionOutcome::NotAttempted { reason } => Ok(AppliedOutcome::NotAttempted(
            NotAttempted::SpawnRefused(reason),
        )),
    }
}

/// Surface a probe's freeform stderr.
///
/// At v1 the report lane has no remote file home, so an oracle's declines and any tool noise ride
/// stderr and are captured per host (`260` §4). It is passthrough: never parsed for control, and
/// nothing on it can influence a verdict.
pub(crate) fn echo_host_stderr(stream: &[u8]) {
    echo(stream, true);
}

/// Echo a captured host stream, one encoded line at a time.
///
/// Every line goes through the shared display seat (`dorc_aid::display::encode_line`), because
/// these are bytes a managed host chose and a terminal is a sink that executes some of them.
/// Encoding grants them no trust — it makes them safe to LOOK at, nothing more.
fn echo(stream: &[u8], to_stderr: bool) {
    if stream.is_empty() {
        return;
    }
    for line in String::from_utf8_lossy(stream).lines() {
        let safe = dorc_aid::display::encode_line(line, ECHO_LINE_CAP);
        if to_stderr {
            eprintln!("{safe}");
        } else {
            println!("{safe}");
        }
    }
}

/// Per-line ceiling on echoed host output.
const ECHO_LINE_CAP: usize = 4096;

/// Refuse a `--host` value that cannot be an ssh destination.
pub(crate) fn host_rejected(raw: &str) -> Diag {
    Diag::new_spanless_site(DiagCode::CliFlagValueNotRecognized(
        dorc_aid::diag::CliFlagValueNotRecognized {
            flag: "--host".to_owned(),
            got: raw.to_owned(),
            expected: "an ssh destination",
        },
    ))
}

/// Report a session that never reported completion.
pub(crate) fn session_lost(host: &str, attempts: u32, diagnosis: &TransportDiagnosis) -> Diag {
    Diag::new_spanless_site(DiagCode::TransportSessionLost(
        dorc_aid::diag::TransportSessionLost {
            host: host.to_owned(),
            attempts: attempts.to_string(),
            diagnosis: describe(diagnosis),
        },
    ))
}

/// Report a remote apply that ran and exited non-zero.
pub(crate) fn apply_failed(host: &str, status: i32) -> Diag {
    Diag::new_spanless_site(DiagCode::TransportApplyFailed(
        dorc_aid::diag::TransportApplyFailed {
            host: host.to_owned(),
            status: status.to_string(),
        },
    ))
}

/// Report a host that was never contacted, as one of the two worlds that can claim it.
///
/// The platform's spawn words reach us as a `String`: `dorc-transport` is deliberately
/// dependency-free (its `Cargo.toml` carries the weld), so no `io::Error` can survive the crate
/// boundary and this edge is the first place that can seal them. That is exactly the relay
/// `from_io_edge` names.
pub(crate) fn not_attempted(host: &str, why: &NotAttempted) -> Diag {
    use dorc_aid::diag::{TransportMarkerUnusable, TransportSpawnRefused};
    let host = host.to_owned();
    match why {
        NotAttempted::SpawnRefused(platform) => {
            let detail = dorc_aid::ForeignBytes::from_io_edge(platform);
            Diag::new_spanless_site(DiagCode::TransportSpawnRefused(TransportSpawnRefused {
                host,
                detail,
            }))
        }
        NotAttempted::MarkerUnusable => {
            Diag::new_spanless_site(DiagCode::TransportMarkerUnusable(TransportMarkerUnusable {
                host,
            }))
        }
    }
}

/// A one-line reading of what severed a session, for the operator.
///
/// Decision-inert by construction: it is a `String` on a diagnostic payload, reachable from no
/// license, verdict or plan. Being wrong here costs a less useful sentence and nothing else
/// (`26A` stop-2 demoted exactly this from classification to diagnosis).
fn describe(diagnosis: &TransportDiagnosis) -> String {
    match diagnosis {
        TransportDiagnosis::TimedOut { after } => format!("timed out after {}s", after.as_secs()),
        TransportDiagnosis::ChildExited { status: Some(code) } => {
            format!("ssh exited {code}")
        }
        TransportDiagnosis::ChildExited { status: None } => "ssh exited on a signal".to_owned(),
        TransportDiagnosis::ChildLost => "the session ended without a status".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pinned_nonce_is_only_honoured_when_it_is_marker_safe() {
        assert!(usable_pinned_nonce("abc123"));
        assert!(!usable_pinned_nonce(""));
        for unsafe_pin in ["not safe!", "a'; rm -rf /; echo '", "a-b", "n=1"] {
            assert!(
                !usable_pinned_nonce(unsafe_pin),
                "a pin that could not survive the marker's charset must be ignored: {unsafe_pin}"
            );
        }
    }

    #[test]
    fn minted_nonces_separate_runs_and_are_always_marker_safe() {
        let a = minted_nonce(1_700_000_000_000_000_000, 4242);
        let b = minted_nonce(1_700_000_000_000_000_001, 4242);
        let c = minted_nonce(1_700_000_000_000_000_000, 4243);
        assert_ne!(a, b, "two runs a nanosecond apart must not collide");
        assert_ne!(a, c, "two concurrent processes must not collide");
        for nonce in [&a, &b, &c] {
            assert!(
                usable_pinned_nonce(nonce),
                "a minted nonce must satisfy the marker's charset: {nonce}"
            );
        }
    }

    #[test]
    fn carriage_returns_are_located_by_line_and_never_repaired() {
        assert_eq!(first_carriage_return(b"a\nb\n"), None);
        assert_eq!(first_carriage_return(b"a\nb\r\nc\n"), Some(2));
        assert_eq!(first_carriage_return(b"\r\n"), Some(1));
        let original = b"a\r\nb\n";
        assert!(first_carriage_return(original).is_some());
        assert_eq!(original, b"a\r\nb\n", "detection must not rewrite anything");
    }
}
