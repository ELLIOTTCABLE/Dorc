//! `dorc-transport` — the controller↔host session edge (`260` §2, at N=1).
//!
//! One trait, [`SessionDriver`], with three implementations that differ ONLY in how a child
//! process is invoked: [`ssh::SshDriver`] (production, the system `ssh` binary),
//! [`local::LocalDriver`] (a local POSIX shell — the hermetic tier that must exercise the
//! production path, not a parallel one), and [`sim::SimDriver`] (scripted bytes, no process at
//! all — the DST tier). Everything after the spawn — capture, completion detection,
//! classification — is shared code in [`child`], so a hermetic green result is evidence about
//! the code that really ships.
//!
//! # What crosses, and what does not (`law-seam-1`)
//!
//! The unit is a WHOLE compiled artifact per host per phase. A site is never a network
//! operation. This crate ships bytes and returns bytes; it knows nothing of sites, facts,
//! verdicts, or the records grammar. Admission of what comes back belongs to the caller, behind
//! the bounded intake (`rul-host-bytes-bounded-before-admission`) — nothing here interprets a
//! managed host's output beyond locating one controller-authored marker in it.
//!
//! # Why completion is a marker and not an exit code (`26A` stop-2)
//!
//! `ssh` reports 255 for its own failures, and a remote script may legitimately exit 255; the
//! collision is unresolvable from exit status. Worse, classifying a severed session by grepping
//! its stderr for English phrases means an unrecognised sever reads as a clean failure —
//! assumed-failed-AND-complete, where `law-fail-direction` requires Unknown. So the remote
//! COMMAND LINE (never the artifact, whose bytes stay floored — `law-artifact-floor`) runs the
//! artifact and then prints an end-marker carrying `$?`. Marker present ⇒ a genuine remote exit,
//! classified by the carried status. Marker absent ⇒ [`SessionOutcome::LostAfterSend`],
//! whatever `ssh` said and whatever its stderr contains. Exit-status and stderr patterns survive
//! as [`TransportDiagnosis`] — an operator hint attached to the outcome, never an input to it.
//!
//! A wall-clock kill is not a special case: killing the child means no marker arrives, so a
//! timeout mints Unknown through the ordinary path rather than through a rule of its own.

#![forbid(unsafe_code)]

pub mod child;
pub mod local;
pub mod marker;
pub mod sim;
pub mod ssh;

pub use local::LocalDriver;
pub use marker::{MarkerRejected, SessionMarker};
pub use sim::{SimDriver, SimScript};
pub use ssh::{SshDriver, SshOptions};

use std::time::Duration;

/// An ssh destination, carried verbatim.
///
/// Dorc never parses this (`260` §2): an alias resolved by the user's own ssh config is
/// first-class, and `user@host`, a `Host` stanza name, and a bare hostname are all just strings
/// to us. The user's ssh config is the credential plane and we do not second-guess it.
///
/// The one thing rejected is a string that could be read as an option rather than a
/// destination. A leading `-` would let a destination smuggle ssh flags into the invocation, so
/// it refuses at construction rather than at the argv boundary.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HostId(String);

/// Why a destination string was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostIdRejected {
    /// The empty string.
    Empty,
    /// Begins with `-`, and would be parsed as an ssh option.
    LeadingDash,
    /// Carries whitespace or a control byte.
    NotOneWord,
}

impl HostId {
    /// Accept a destination string.
    ///
    /// # Errors
    /// [`HostIdRejected`] when the string is empty, option-shaped, or not a single word.
    pub fn new(raw: &str) -> Result<Self, HostIdRejected> {
        if raw.is_empty() {
            return Err(HostIdRejected::Empty);
        }
        if raw.starts_with('-') {
            return Err(HostIdRejected::LeadingDash);
        }
        if raw.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(HostIdRejected::NotOneWord);
        }
        Ok(Self(raw.to_owned()))
    }

    /// The destination as ssh will receive it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Which phase's artifact is crossing (`inv-kfail` — the fail directions are opposite, and the
/// caller keys retry policy on this: a probe is read-only by contract and may be retried, an
/// apply never is — `law-no-double-apply`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// The read-only probe artifact.
    Probe,
    /// The mutating apply artifact.
    Apply,
}

/// One artifact, shipped to one host, once.
#[derive(Debug)]
pub struct SessionRequest<'a> {
    /// Where it goes.
    pub host: &'a HostId,
    /// Which phase's artifact this is.
    pub phase: Phase,
    /// The artifact bytes, fed to the remote shell on stdin. Shipped verbatim: this crate never
    /// edits a byte of what it is handed (`law-artifact-floor`).
    pub artifact: &'a [u8],
    /// The controller-authored completion marker this session is classified by.
    pub marker: &'a SessionMarker,
    /// Wall-clock ceiling on the whole session. `None` is unlimited — the apply default, since
    /// an apply is the user's real work and killing it mints Unknown rather than a verdict.
    pub timeout: Option<Duration>,
}

/// What the session did — a closed vocabulary, each variant carrying its own proof obligation.
///
/// Note what is NOT here: there is no "unreachable" variant. Absence of output cannot prove
/// absence of execution (the remote may have run and had its bytes lost), so a session that
/// spawned and produced no marker is Unknown, not clean and not failed. Only
/// [`NotAttempted`](SessionOutcome::NotAttempted) claims nothing ran, and it claims it on the
/// strength of no process ever having existed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionOutcome {
    /// The marker arrived: the remote artifact genuinely exited, with the status it carried.
    ///
    /// `stdout` has had the marker line removed — it is exactly what the artifact wrote, ready
    /// for the caller's bounded intake. The status is reproduced, never interpreted
    /// (`law-lane-discipline`: the engine measures a status and passes it through).
    Completed {
        /// The status the marker carried.
        status: i32,
        /// Artifact stdout, marker line stripped.
        stdout: Vec<u8>,
        /// Artifact stderr, verbatim.
        stderr: Vec<u8>,
    },
    /// A process ran and the marker never arrived ⇒ the world's state is UNKNOWN.
    ///
    /// Per `rul-integrity-failure-withholds-mutation` this is a closed outcome, never rounded to
    /// "run everything": running everything is the safe answer to not knowing the WORLD, and the
    /// wrong answer to not knowing whether we are still talking to the world we think we are.
    /// The sanctioned recovery is re-probe-then-re-plan — the probe is the retry-file.
    LostAfterSend {
        /// Whatever stdout arrived before the loss. Retained for the caller to salvage or
        /// discard by its own lane rules; never trusted to be complete.
        stdout: Vec<u8>,
        /// Whatever stderr arrived before the loss.
        stderr: Vec<u8>,
        /// A guess at what severed it, for the operator. Decision-inert.
        diagnosis: TransportDiagnosis,
    },
    /// No process was ever created, so nothing ran anywhere.
    ///
    /// The only outcome licensed to claim the host was untouched, and it is licensed because the
    /// claim is local: the controller failed to spawn a child. It never covers a child that
    /// started and failed to connect — that is [`LostAfterSend`](SessionOutcome::LostAfterSend).
    NotAttempted {
        /// Why the spawn failed, for the operator.
        reason: String,
    },
}

/// A best-effort reading of what ended a session, attached to a
/// [`LostAfterSend`](SessionOutcome::LostAfterSend) for the operator's benefit.
///
/// This is diagnosis, demoted from its former role as classification (`26A` stop-2). Nothing may
/// branch a license, a verdict, or a plan on it: it exists so an error message can say "the name
/// did not resolve" instead of only "unknown", and its being wrong must cost nothing but a
/// less-useful sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportDiagnosis {
    /// The wall-clock ceiling fired and the child was killed.
    TimedOut {
        /// The ceiling that fired.
        after: Duration,
    },
    /// The child exited without the marker. Its status is recorded for the message only.
    ChildExited {
        /// The child's own exit status, if it had one.
        status: Option<i32>,
    },
    /// The child was killed by a signal, or its status could not be read.
    ChildLost,
}

/// The DI seam (`inv-determinism`): every path to a managed host goes through here, so the
/// kernel's purity is a property of the dependency graph rather than of reviewer vigilance.
///
/// Implementations own all nondeterminism — processes, sockets, the clock. A driver is free to
/// block; concurrency, when it arrives, is the caller's (`law-perf-redlines` forbids
/// fork-per-host as an architecture, and this signature does not commit to one).
pub trait SessionDriver {
    /// Ship one artifact and classify what came back.
    ///
    /// Total: transport failures are values, never panics and never `Err` (`inv-no-throw`).
    /// Every way a session can end is a [`SessionOutcome`], which is what makes the exhaustive
    /// `match` at the call site the place the fail-direction law is read off.
    fn run(&mut self, request: &SessionRequest<'_>) -> SessionOutcome;
}
