//! The transport edge: where a run acquires a host, and the only place this binary decides
//! anything about reaching one.
//!
//! Everything here is I/O-side by construction (`io-at-edges-only`). The analysis pipeline never
//! learns that a host exists: it is handed evidence bytes exactly as it would be handed the
//! contents of `--results`, and it cannot tell which happened.

use dorc_plan::records::{Framing, Nonce, RemoteIdentity};
use dorc_transport::{
    HostId, Phase, SessionDriver, SessionMarker, SessionOutcome, SessionRequest, TransportDiagnosis,
};

use dorc_aid::diag::{Diag, DiagCode};
use std::time::Duration;

/// How many times a probe may be re-shipped after a transport loss (`260` dec-26-probe-retry).
///
/// A probe is read-only BY CONTRACT, so re-running one cannot double anything; an apply has no
/// such licence and is never retried (`law-no-double-apply`). The asymmetry is `kFAIL`
/// phase-keying spelled at the transport.
const PROBE_RETRIES: u32 = 2;

/// The run's nonce, minted from the clock and pid — the `Os` nonce seam's production draw.
///
/// Unique, not unpredictable — and the distinction is deliberate. The host is HANDED this nonce
/// (it is baked into the artifact we ship it), so secrecy against the host is not a property
/// this value could have. What it must do is separate one attempt from another and one run from
/// another, so a killed attempt's zombie writer and a mis-plumbed stream fail to parse rather
/// than fold. Clock and pid give that without a dependency. A harness selects the seeded nonce
/// seam instead (`crate::seam::NonceSeam`); this is the query, the seam is the value.
///
/// This reasoning stops holding if records from several sessions ever share one channel; at that
/// point unpredictability starts to matter and this needs revisiting.
pub(crate) fn minted_process_nonce() -> String {
    minted_nonce(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos()),
        std::process::id(),
    )
}

/// The derivation, kept a pure function of its inputs so it is testable without mutating the
/// process environment — which `unsafe_code = "forbid"` makes impossible in a test anyway.
fn minted_nonce(nanos: u128, pid: u32) -> String {
    let mixed = u64::try_from(nanos & u128::from(u64::MAX)).unwrap_or(0) ^ (u64::from(pid) << 32);
    format!("r{mixed:016x}")
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
    crate::transport_crlf_error(which, line)
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

/// Classify what one apply shipment did, echoing the host's streams as it goes.
///
/// `None` is a run that could not build a session marker, so nothing was ever sent. Total: every
/// way a shipment can end is a value here, which is what lets the caller sit immediately past a
/// spent permit with nothing between them that could fail.
pub(crate) fn classify_shipment(shipped: Option<SessionOutcome>) -> AppliedOutcome {
    let Some(shipped) = shipped else {
        return AppliedOutcome::NotAttempted(NotAttempted::MarkerUnusable);
    };
    match shipped {
        SessionOutcome::Completed {
            status,
            stdout,
            stderr,
        } => {
            echo(&stdout, false);
            echo(&stderr, true);
            AppliedOutcome::Ran { status }
        }
        SessionOutcome::LostAfterSend {
            stdout,
            stderr,
            diagnosis,
        } => {
            echo(&stdout, false);
            echo(&stderr, true);
            AppliedOutcome::Unknown { diagnosis }
        }
        SessionOutcome::NotAttempted { reason } => {
            AppliedOutcome::NotAttempted(NotAttempted::SpawnRefused(reason))
        }
    }
}

pub(crate) fn encoded_host_lines(stream: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(stream)
        .lines()
        .map(|line| dorc_aid::display::encode_line(line, ECHO_LINE_CAP))
        .collect()
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
    for safe in encoded_host_lines(stream) {
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
    crate::transport_session_lost(host, attempts, diagnosis)
}

/// Report a remote apply that ran and exited non-zero.
pub(crate) fn apply_failed(host: &str, status: i32) -> Diag {
    crate::transport_apply_failed(host, status)
}

/// Report a host that was never contacted, as one of the two worlds that can claim it.
///
/// The platform's spawn words reach us as a `String`: `dorc-transport` is deliberately
/// dependency-free (its `Cargo.toml` carries the weld), so no `io::Error` can survive the crate
/// boundary and this edge is the first place that can seal them. That is exactly the relay
/// `from_io_edge` names.
pub(crate) fn not_attempted(host: &str, why: &NotAttempted) -> Diag {
    match why {
        NotAttempted::SpawnRefused(platform) => crate::transport_spawn_refused(host, platform),
        NotAttempted::MarkerUnusable => crate::transport_marker_unusable(host),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minted_nonces_separate_runs_and_are_always_marker_safe() {
        let a = minted_nonce(1_700_000_000_000_000_000, 4242);
        let b = minted_nonce(1_700_000_000_000_000_001, 4242);
        let c = minted_nonce(1_700_000_000_000_000_000, 4243);
        assert_ne!(a, b, "two runs a nanosecond apart must not collide");
        assert_ne!(a, c, "two concurrent processes must not collide");
        for nonce in [&a, &b, &c] {
            assert!(
                !nonce.is_empty() && nonce.chars().all(|c| c.is_ascii_alphanumeric()),
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
