//! The shared spawn-and-capture core.
//!
//! `ssh` and local execution differ by exactly one thing — the [`Command`] handed in — and
//! everything downstream of the spawn lives here. That is deliberate: the hermetic tier is only
//! worth running if it exercises the code that ships, so the local driver must not be a
//! reimplementation that happens to agree (`260` §5, driver 2).

use crate::{SessionMarker, SessionOutcome, SessionRequest, TransportDiagnosis};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// How often a bounded wait re-checks the child. Small enough to be invisible next to a network
/// round-trip, large enough not to spin.
const POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Ceiling on ONE captured stream (`260` §5 backpressure floor).
///
/// A managed host controls how much it writes, so an unbounded read is a memory-growth path the
/// host itself chooses. This bound needs no failure rule of its own: a capture that hits it is by
/// definition missing its completion marker, so it classifies as a loss and the run mints
/// Unknown — the safe direction, reached through the ordinary path.
const CAPTURE_CAP: u64 = 8 * 1024 * 1024;

/// Spawn `command`, feed it the artifact, capture both streams, and classify by the marker.
///
/// Total by construction (`inv-no-throw`): a failure to spawn is
/// [`NotAttempted`](SessionOutcome::NotAttempted), and every other ending is a value.
pub fn run(mut command: Command, request: &SessionRequest<'_>) -> SessionOutcome {
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return SessionOutcome::NotAttempted {
                reason: error.to_string(),
            };
        }
    };

    let artifact = request.artifact.to_vec();
    let feeder = child.stdin.take().map(|mut sink| {
        std::thread::spawn(move || {
            let _ = sink.write_all(&artifact);
            let _ = sink.flush();
        })
    });
    let out_reader = child.stdout.take().map(drain);
    let err_reader = child.stderr.take().map(drain);

    let waited = wait_for(&mut child, request.timeout);

    if let Some(feeder) = feeder {
        let _ = feeder.join();
    }
    let stdout = out_reader.and_then(|h| h.join().ok()).unwrap_or_default();
    let stderr = err_reader.and_then(|h| h.join().ok()).unwrap_or_default();

    match request.marker.scan(&stdout) {
        Some(completion) => SessionOutcome::Completed {
            status: completion.status,
            stdout: SessionMarker::strip_from(&stdout, &completion),
            stderr,
        },
        None => SessionOutcome::LostAfterSend {
            stdout,
            stderr,
            diagnosis: waited,
        },
    }
}

/// Read a pipe to end on its own thread, so a child that fills one stream while we read the
/// other cannot deadlock us.
fn drain<R: Read + Send + 'static>(source: R) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = source.take(CAPTURE_CAP).read_to_end(&mut buffer);
        buffer
    })
}

/// Wait for the child, killing it if the ceiling fires.
///
/// The returned diagnosis is only consulted when no marker arrived, so a clean run never pays
/// attention to it.
fn wait_for(child: &mut std::process::Child, timeout: Option<Duration>) -> TransportDiagnosis {
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return TransportDiagnosis::ChildExited {
                    status: status.code(),
                };
            }
            Err(_) => return TransportDiagnosis::ChildLost,
            Ok(None) => {}
        }
        if let Some(limit) = timeout
            && started.elapsed() >= limit
        {
            let _ = child.kill();
            let _ = child.wait();
            return TransportDiagnosis::TimedOut { after: limit };
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}
