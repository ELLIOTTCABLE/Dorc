//! Closed loom-only failures for nondeterministic edge operations.
//!
//! These values stop at production edge APIs. This module never constructs, names, or renders a
//! diagnostic; production maps each injected operation outcome through its normal error path.

use errorloom::Case;

const SECTION: &str = "edge-fault";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum EdgeFault {
    Read {
        path: String,
        failure: IoFailure,
    },
    ShimWrite {
        path: String,
        failure: IoFailure,
    },
    ShimExec(IoFailure),
    ArtifactPublish(&'static str),
    ReceiptPublish(String),
    /// A remote apply's OUTCOME publication, past the permit its intent already minted. Its own
    /// operation rather than a `ReceiptPublish` reason word: which of an apply's two publications
    /// failed is the difference between dispatching nothing and losing the record of a dispatch.
    ApplyOutcomePublish(dorc_receipt::dispatch::DurableFailure),
    HostEvidence(dorc_plan::records::AdmissionRefusal),
    ToolRun {
        tool: String,
        rc: i32,
        stdout: Vec<u8>,
    },
    Transport(TransportFailure),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum IoFailure {
    NotFound(String),
    PermissionDenied,
    Other(String),
}

impl IoFailure {
    pub(crate) fn error(&self) -> std::io::Error {
        match self {
            Self::NotFound(detail) => {
                std::io::Error::new(std::io::ErrorKind::NotFound, detail.clone())
            }
            Self::PermissionDenied => std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "permission denied by injected edge",
            ),
            Self::Other(detail) => std::io::Error::other(detail.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TransportFailure {
    Crlf { line: usize },
    SessionLost,
    SpawnRefused(String),
    MarkerUnusable,
    ApplyFailed { status: i32 },
}

impl EdgeFault {
    pub(crate) fn from_case(case: &Case) -> Result<Option<Self>, String> {
        let Some(section) = case
            .sections()
            .iter()
            .find(|section| section.name() == SECTION)
        else {
            return Ok(None);
        };
        let content = section.content();
        let (head, body) = content.split_once('\n').unwrap_or((content, ""));
        let words = head.split_ascii_whitespace().collect::<Vec<_>>();
        let fault = match words.as_slice() {
            ["read", outcome, path] => Self::Read {
                path: (*path).to_owned(),
                failure: io_failure(outcome, body)?,
            },
            ["shim-write", outcome, path] => Self::ShimWrite {
                path: (*path).to_owned(),
                failure: io_failure(outcome, body)?,
            },
            ["shim-exec", outcome] => Self::ShimExec(io_failure(outcome, body)?),
            ["artifact-publish", "directory"] => Self::ArtifactPublish("directory"),
            ["artifact-publish", "write"] => Self::ArtifactPublish("write"),
            ["receipt-publish", "outcome", step] => Self::ApplyOutcomePublish(durable_step(step)?),
            ["receipt-publish", reason] => Self::ReceiptPublish((*reason).to_owned()),
            ["host-evidence", "invalid-utf8"] => {
                Self::HostEvidence(dorc_plan::records::AdmissionRefusal::InvalidUtf8)
            }
            ["host-evidence", "control-byte"] => {
                Self::HostEvidence(dorc_plan::records::AdmissionRefusal::ControlByte)
            }
            ["tool-run", tool, rc] => Self::ToolRun {
                tool: (*tool).to_owned(),
                rc: rc
                    .parse::<i32>()
                    .map_err(|_| format!("edge fault has invalid tool status `{rc}`"))?,
                stdout: body.as_bytes().to_vec(),
            },
            ["transport", "crlf", line] => Self::Transport(TransportFailure::Crlf {
                line: line
                    .parse::<usize>()
                    .map_err(|_| format!("edge fault has invalid CR line `{line}`"))?,
            }),
            ["transport", "session-lost"] => Self::Transport(TransportFailure::SessionLost),
            ["transport", "spawn-refused"] => {
                Self::Transport(TransportFailure::SpawnRefused(body.trim_end().to_owned()))
            }
            ["transport", "marker-unusable"] => Self::Transport(TransportFailure::MarkerUnusable),
            ["transport", "apply-failed", status] => {
                Self::Transport(TransportFailure::ApplyFailed {
                    status: status
                        .parse::<i32>()
                        .map_err(|_| format!("edge fault has invalid apply status `{status}`"))?,
                })
            }
            _ => return Err(format!("unsupported `{SECTION}` declaration `{head}`")),
        };
        Ok(Some(fault))
    }

    pub(crate) fn read_failure(&self, path: &str) -> Option<&IoFailure> {
        match self {
            Self::Read {
                path: fault_path,
                failure,
            } if fault_path == path => Some(failure),
            _ => None,
        }
    }
}

/// Which step of writing a document a declaration says did not close — the OPERATION's outcome,
/// not a diagnostic: the same closed set the receipt library answers a publication with.
fn durable_step(step: &str) -> Result<dorc_receipt::dispatch::DurableFailure, String> {
    use dorc_receipt::dispatch::DurableFailure;
    match step {
        "projection" => Ok(DurableFailure::Projection),
        "grammar" => Ok(DurableFailure::Grammar),
        "seal" => Ok(DurableFailure::Seal),
        "signature" => Ok(DurableFailure::Signature),
        "sink" => Ok(DurableFailure::Sink),
        _ => Err(format!("unsupported durable write step `{step}`")),
    }
}

fn io_failure(outcome: &str, detail: &str) -> Result<IoFailure, String> {
    match outcome {
        "not-found" => Ok(IoFailure::NotFound(detail.trim_end().to_owned())),
        "permission-denied" => Ok(IoFailure::PermissionDenied),
        "other" if !detail.trim().is_empty() => Ok(IoFailure::Other(detail.trim_end().to_owned())),
        _ => Err(format!("unsupported I/O edge outcome `{outcome}`")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declarations_name_edge_operations_and_outcomes_not_diagnostics() {
        let case = Case::parse(
            "---\n---\n-- edge-fault --\nread other results.txt\nIs a directory (os error 21)\n\n-- replay --\n$ dorc plan --book=book.sh\n",
        )
        .expect("case");
        assert_eq!(
            EdgeFault::from_case(&case),
            Ok(Some(EdgeFault::Read {
                path: "results.txt".to_owned(),
                failure: IoFailure::Other("Is a directory (os error 21)".to_owned()),
            }))
        );
        let source = include_str!("edge_fault.rs");
        assert!(!source.contains(concat!("Diag", "Code")));
        assert!(!source.contains(concat!("Harness", "Scenario")));
    }
}
