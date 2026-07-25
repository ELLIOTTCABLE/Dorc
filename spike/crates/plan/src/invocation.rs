//! The pure plan-invocation boundary shared by command-line and replay adapters.
//!
//! The boundary owns no filesystem, environment, terminal, clock, randomness, or process
//! interaction. Its inputs are the exact bytes an edge acquired; its output is an ordered
//! transcript for that edge to decorate and write.

use dorc_aid::diag::Diag;
use dorc_aid::tagged::RenderParts;
use dorc_core::{Capability, EscalationDial};

use crate::records::LegacyPolicy;
use crate::{Plan, ProbePlan};

/// Exact oracle bytes together with their edge-owned display identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleInput {
    /// The edge-owned path or other display identity.
    pub identity: String,
    /// The exact oracle source bytes.
    pub source: String,
}

/// Explicit knobs that affect one invocation's semantic plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanOptions {
    /// Whether the caller explicitly enabled the footprint survival tier.
    pub trust_footprints: bool,
    /// The injected context-entry escalation policy.
    pub dial: EscalationDial,
    /// The injected connection capability.
    pub capability: Capability,
}

/// All pure inputs needed to construct one plan invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationInput {
    /// Exact concatenated book bytes.
    pub book: String,
    /// Edge-selected oracles in deterministic load order.
    pub oracles: Vec<OracleInput>,
    /// Exact probe-record bytes, before deframing.
    pub records: String,
    /// Explicit plan knobs.
    pub options: PlanOptions,
    /// The edge-selected records compatibility policy.
    pub record_policy: LegacyPolicy,
    /// Exact invocation argv retained for replay/whylog assembly.
    pub argv: Vec<String>,
}

/// The semantic process result the CLI maps to an exit code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticOutcome {
    /// The invocation completed without a semantic fast-fail.
    Complete,
    /// The book was only partially modeled.
    BookUnmodeled,
    /// Wrapper declarations contradicted one another.
    WrapperIncoherent,
}

/// The undecorated output channel owned by an edge adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputChannel {
    /// Contractual artifact bytes.
    Stdout,
    /// Diagnostics and advisory bytes.
    Stderr,
}

/// A diagnostic event retains the exact diagnostic and render-seat provenance.
#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    /// The diagnostic emitted by the production pipeline.
    pub diagnostic: Diag,
    /// The production render parts for this exact diagnostic seat.
    pub render_parts: RenderParts,
}

/// One event in the invocation transcript, preserving stdout/stderr event-time order.
#[derive(Debug, Clone)]
pub struct OutputEvent {
    /// The undecorated target channel.
    pub channel: OutputChannel,
    /// Exact captured bytes, before terminal decoration.
    pub bytes: Vec<u8>,
    /// Provenance for a diagnostic render; absent for raw artifact and summary bytes.
    pub diagnostic: Option<DiagnosticEvent>,
}

impl OutputEvent {
    /// Preserve raw bytes without inventing diagnostic provenance.
    #[must_use]
    pub fn raw(channel: OutputChannel, bytes: Vec<u8>) -> Self {
        Self {
            channel,
            bytes,
            diagnostic: None,
        }
    }

    /// Preserve a diagnostic's exact render-seat provenance beside its rendered bytes.
    #[must_use]
    pub fn diagnostic(channel: OutputChannel, bytes: Vec<u8>, diagnostic: DiagnosticEvent) -> Self {
        Self {
            channel,
            bytes,
            diagnostic: Some(diagnostic),
        }
    }
}

/// Pure output from the shared plan pipeline.
#[derive(Debug, Clone)]
pub struct InvocationResult {
    /// The semantic result the CLI turns into an exit code.
    pub outcome: SemanticOutcome,
    /// The compiled probe artifact model needed by adapters.
    pub probe: ProbePlan,
    /// The final plan model needed by adapters.
    pub plan: Plan,
    /// Exact apply artifact bytes.
    pub apply: String,
    /// The deterministic decision digest.
    pub decision_digest: String,
    /// The pure whylog candidate; writing remains an edge concern.
    pub whylog_candidate: Option<crate::whylog::WhylogDoc>,
    /// All output bytes in their original cross-channel event order.
    pub events: Vec<OutputEvent>,
}

/// Compute the deterministic content identity used to bind an invocation's records to its book.
///
/// The spike's non-cryptographic digest is intentionally pure and dependency-free; an edge may
/// later substitute a stronger identity without changing the invocation boundary.
#[must_use]
pub fn book_digest(source: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in source.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_events_preserve_cross_channel_order_and_provenance_boundary() {
        let events = [
            OutputEvent::raw(OutputChannel::Stdout, b"probe\n".to_vec()),
            OutputEvent::raw(OutputChannel::Stderr, b"summary\n".to_vec()),
            OutputEvent::raw(OutputChannel::Stdout, b"apply\n".to_vec()),
        ];

        let transcript: Vec<(OutputChannel, Vec<u8>)> = events
            .iter()
            .map(|event| (event.channel, event.bytes.clone()))
            .collect();
        assert_eq!(
            transcript,
            vec![
                (OutputChannel::Stdout, b"probe\n".to_vec()),
                (OutputChannel::Stderr, b"summary\n".to_vec()),
                (OutputChannel::Stdout, b"apply\n".to_vec()),
            ]
        );
        assert!(events.iter().all(|event| event.diagnostic.is_none()));
    }

    #[test]
    fn digest_depends_on_exact_bytes() {
        assert_ne!(book_digest("hork\n"), book_digest("hork\r\n"));
    }
}
