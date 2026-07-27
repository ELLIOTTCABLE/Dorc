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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HostId;

    /// A records stream shaped like the one a real probe artifact writes.
    const RECORDS: &[u8] = b"dorc-records/1 nonce=n1 attempt=1 host=web1 book=bk sites=1 @@dorc@@\nn1 site 0 effect=converged rc=0 @@dorc@@\ndorc-records-end/1 nonce=n1 @@dorc@@\n";

    fn drive(script: SimScript) -> (SessionOutcome, SimDriver) {
        let host = HostId::new("web1").expect("valid destination");
        let marker = SessionMarker::new("n1", 1).expect("valid nonce");
        let mut driver = SimDriver::new(vec![script]);
        let outcome = driver.run(&SessionRequest {
            host: &host,
            phase: crate::Phase::Probe,
            artifact: b"#!/bin/sh\n",
            marker: &marker,
            timeout: None,
        });
        (outcome, driver)
    }

    #[test]
    fn a_completed_session_yields_the_artifacts_own_bytes_and_its_carried_status() {
        let (outcome, _) = drive(SimScript::Completes {
            stdout: RECORDS.to_vec(),
            status: 0,
        });
        match outcome {
            SessionOutcome::Completed { status, stdout, .. } => {
                assert_eq!(status, 0);
                assert_eq!(
                    stdout, RECORDS,
                    "the records lane must reach the caller exactly as written: the transport's \
                     marker is its own framing, and framing left in the stream would be refused \
                     by the deframer as a line after the sentinel"
                );
            }
            other => panic!("expected completion, got {other:?}"),
        }
    }

    #[test]
    fn a_nonzero_remote_status_is_carried_through_rather_than_reinterpreted() {
        for status in [1, 2, 127, 255] {
            let (outcome, _) = drive(SimScript::Completes {
                stdout: b"out\n".to_vec(),
                status,
            });
            match outcome {
                SessionOutcome::Completed { status: got, .. } => assert_eq!(
                    got, status,
                    "255 in particular: ssh's own failure code and a plan's genuine exit are \
                     indistinguishable by exit status, and the marker is what separates them"
                ),
                other => panic!("expected completion, got {other:?}"),
            }
        }
    }

    #[test]
    fn a_sever_mid_stream_is_unknown_however_much_arrived() {
        let (outcome, _) = drive(SimScript::SeveredAfter {
            stdout: b"dorc-records/1 nonce=n1 attempt=1 host=web1 book=bk sites=9 @@dorc@@\n"
                .to_vec(),
        });
        assert!(
            matches!(outcome, SessionOutcome::LostAfterSend { .. }),
            "a partial stream is not a partial success: without the marker nothing witnessed the \
             remote finishing, and that absence is not a fact about the world"
        );
    }

    #[test]
    fn a_tidy_close_without_a_marker_is_still_unknown() {
        for status in [Some(0), Some(1), None] {
            let (outcome, _) = drive(SimScript::ClosedWithoutMarker {
                stdout: b"partial\n".to_vec(),
                status,
            });
            assert!(
                matches!(outcome, SessionOutcome::LostAfterSend { .. }),
                "the EOF-without-status gap: the channel ended politely and the child even \
                 reported {status:?}, but reading THAT as the artifact's ending is exactly the \
                 mistake the marker exists to prevent"
            );
        }
    }

    #[test]
    fn a_timeout_kill_mints_unknown_and_never_a_clean_ending() {
        let (outcome, _) = drive(SimScript::TimesOut {
            after: Duration::from_mins(2),
        });
        match outcome {
            SessionOutcome::LostAfterSend { diagnosis, .. } => assert_eq!(
                diagnosis,
                TransportDiagnosis::TimedOut {
                    after: Duration::from_mins(2)
                },
                "the ceiling is recorded for the operator, but it did not decide the outcome — \
                 the absent marker did"
            ),
            other => panic!("a killed session must be unknown, got {other:?}"),
        }
    }

    #[test]
    fn a_spawn_failure_is_the_one_outcome_that_may_claim_nothing_ran() {
        let (outcome, _) = drive(SimScript::NotSpawnable {
            reason: "program not found".to_owned(),
        });
        assert!(
            matches!(outcome, SessionOutcome::NotAttempted { .. }),
            "no process ever existed, so the claim is local and provable — unlike every other \
             no-output case, which cannot separate 'never ran' from 'ran and was lost'"
        );
    }

    #[test]
    fn the_driver_records_what_it_was_asked_to_ship() {
        let (_, driver) = drive(SimScript::Completes {
            stdout: Vec::new(),
            status: 0,
        });
        assert_eq!(driver.calls.len(), 1);
        assert_eq!(driver.calls[0].host, "web1");
        assert_eq!(driver.calls[0].artifact, b"#!/bin/sh\n");
        assert_eq!(driver.remaining(), 0);
    }
}
