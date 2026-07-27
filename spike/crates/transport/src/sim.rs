//! The DST driver: scripted bytes, no process, no clock, no socket.
//!
//! Faults are modelled at the OUTCOME, never at the mechanism (`plans/128` fc-5): there is no
//! netem, no real socket and nothing to be flaky. A scripted session produces the bytes a real
//! one would have produced and hands them to the SAME marker scan the real drivers use, so the
//! classification under test is the shipping classification and not a mock of it (`262` §5, the
//! byte tier).

use crate::{SessionDriver, SessionMarker, SessionOutcome, SessionRequest, TransportDiagnosis};
use std::time::Duration;

/// What one scripted session does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimScript {
    /// The artifact wrote `stdout` and exited with `status`; the marker arrives intact.
    Completes {
        /// Bytes the artifact wrote before exiting.
        stdout: Vec<u8>,
        /// The status the marker will carry.
        status: i32,
    },
    /// Bytes arrived and then the wire died mid-stream: no marker, no clean close.
    SeveredAfter {
        /// The prefix that made it across.
        stdout: Vec<u8>,
    },
    /// The channel closed tidily but no marker was ever emitted — the EOF-without-status gap
    /// that an exit-code reading would misread as a clean ending.
    ClosedWithoutMarker {
        /// Whatever arrived.
        stdout: Vec<u8>,
        /// The child's own status, which must not be allowed to decide anything.
        status: Option<i32>,
    },
    /// The session hit its ceiling and was killed.
    TimesOut {
        /// The ceiling that fired.
        after: Duration,
    },
    /// No process could be created, so nothing ran.
    NotSpawnable {
        /// The spawn failure to report.
        reason: String,
    },
}

/// One session's request, recorded for assertions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimCall {
    /// Where it was sent.
    pub host: String,
    /// Which phase's artifact it was.
    pub phase: crate::Phase,
    /// The exact bytes shipped.
    pub artifact: Vec<u8>,
}

/// Replays a queue of scripted sessions.
#[derive(Debug, Default)]
pub struct SimDriver {
    scripts: Vec<SimScript>,
    /// Every session this driver was asked to run, in order.
    ///
    /// Public so a test can assert what did NOT happen — that an apply was never re-shipped
    /// after a loss (`law-no-double-apply`), and that a probe retry re-shipped a DIFFERENT
    /// artifact carrying the next attempt rather than replaying the old one.
    pub calls: Vec<SimCall>,
}

impl SimDriver {
    /// A driver that will run `scripts` in order, one per session.
    #[must_use]
    pub fn new(scripts: Vec<SimScript>) -> Self {
        Self {
            scripts,
            calls: Vec::new(),
        }
    }

    /// How many scripted sessions remain unused. A test that expected no further sessions
    /// asserts this stayed put.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.scripts.len()
    }
}

impl SessionDriver for SimDriver {
    fn run(&mut self, request: &SessionRequest<'_>) -> SessionOutcome {
        self.calls.push(SimCall {
            host: request.host.as_str().to_owned(),
            phase: request.phase,
            artifact: request.artifact.to_vec(),
        });

        let script = if self.scripts.is_empty() {
            SimScript::NotSpawnable {
                reason: "sim: no script for this session".to_owned(),
            }
        } else {
            self.scripts.remove(0)
        };

        match script {
            SimScript::Completes { stdout, status } => {
                let wire = wire_with_marker(&stdout, request.marker, status);
                match request.marker.scan(&wire) {
                    Some(completion) => SessionOutcome::Completed {
                        status: completion.status,
                        stdout: SessionMarker::strip_from(&wire, &completion),
                        stderr: Vec::new(),
                    },
                    None => SessionOutcome::LostAfterSend {
                        stdout: wire,
                        stderr: Vec::new(),
                        diagnosis: TransportDiagnosis::ChildLost,
                    },
                }
            }
            SimScript::SeveredAfter { stdout } => SessionOutcome::LostAfterSend {
                stdout,
                stderr: Vec::new(),
                diagnosis: TransportDiagnosis::ChildLost,
            },
            SimScript::ClosedWithoutMarker { stdout, status } => SessionOutcome::LostAfterSend {
                stdout,
                stderr: Vec::new(),
                diagnosis: TransportDiagnosis::ChildExited { status },
            },
            SimScript::TimesOut { after } => SessionOutcome::LostAfterSend {
                stdout: Vec::new(),
                stderr: Vec::new(),
                diagnosis: TransportDiagnosis::TimedOut { after },
            },
            SimScript::NotSpawnable { reason } => SessionOutcome::NotAttempted { reason },
        }
    }
}

/// Append the marker a real remote command line would have printed.
fn wire_with_marker(stdout: &[u8], marker: &SessionMarker, status: i32) -> Vec<u8> {
    let mut wire = stdout.to_vec();
    wire.extend_from_slice(b"\n");
    wire.extend_from_slice(marker.marker_line(status).as_bytes());
    wire
}
