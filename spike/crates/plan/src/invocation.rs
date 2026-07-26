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
    /// Whether the caller explicitly enabled the footprint survival tier (`--risk-faultless-skips`).
    pub risk_faultless_skips: bool,
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

/// Compute the deterministic content identity binding an invocation's records to its book.
///
/// SHA-256, hand-rolled because the kernel is dependency-free (`inv-determinism`) and pure. It
/// replaced an FNV-1a-64 under `28F:rul-digest-lands-now`: FNV is a drift-detector, and
/// `rul-fixture-identity-never-production` forbids that class from reaching a production boundary,
/// naming DEFAULT PERSISTENCE among them. The whylog is opt-in today and the r30 flip is one line,
/// so the identity had to stop being the thing blocking it. This is still the ONE substitution
/// point (`whylog.rs` says so); it is now one a durable can carry honestly.
///
/// Not a keyed MAC and not a signature: it answers "are these the same bytes", not "did someone
/// authorized produce them".
#[must_use]
pub fn book_digest(source: &str) -> String {
    let digest = sha256(source.as_bytes());
    let mut hex = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// The SHA-256 round constants (FIPS 180-4 §4.2.2).
///
/// `rustfmt::skip` (precedent: `aid`'s generated `ARRANGEMENTS`): a published constant table is
/// checked against its source by eye, and one-value-per-line over 64 lines makes that harder, not
/// easier.
#[rustfmt::skip]
const SHA256_K: [u32; 64] = [
    0x428a_2f98, 0x7137_4491, 0xb5c0_fbcf, 0xe9b5_dba5, 0x3956_c25b, 0x59f1_11f1, 0x923f_82a4,
    0xab1c_5ed5, 0xd807_aa98, 0x1283_5b01, 0x2431_85be, 0x550c_7dc3, 0x72be_5d74, 0x80de_b1fe,
    0x9bdc_06a7, 0xc19b_f174, 0xe49b_69c1, 0xefbe_4786, 0x0fc1_9dc6, 0x240c_a1cc, 0x2de9_2c6f,
    0x4a74_84aa, 0x5cb0_a9dc, 0x76f9_88da, 0x983e_5152, 0xa831_c66d, 0xb003_27c8, 0xbf59_7fc7,
    0xc6e0_0bf3, 0xd5a7_9147, 0x06ca_6351, 0x1429_2967, 0x27b7_0a85, 0x2e1b_2138, 0x4d2c_6dfc,
    0x5338_0d13, 0x650a_7354, 0x766a_0abb, 0x81c2_c92e, 0x9272_2c85, 0xa2bf_e8a1, 0xa81a_664b,
    0xc24b_8b70, 0xc76c_51a3, 0xd192_e819, 0xd699_0624, 0xf40e_3585, 0x106a_a070, 0x19a4_c116,
    0x1e37_6c08, 0x2748_774c, 0x34b0_bcb5, 0x391c_0cb3, 0x4ed8_aa4a, 0x5b9c_ca4f, 0x682e_6ff3,
    0x748f_82ee, 0x78a5_636f, 0x84c8_7814, 0x8cc7_0208, 0x90be_fffa, 0xa450_6ceb, 0xbef9_a3f7,
    0xc671_78f2,
];

/// SHA-256 over `message` (FIPS 180-4). Pure, allocation-light, and deterministic.
///
/// Written against the spec rather than lifted from anywhere: `inv-no-unsafe` and the zero-dep
/// graph rule out a crate, and `book_digest` needs an identity a durable can carry. The NIST
/// one-block and two-block vectors pin it (see this module's tests) — a hand-rolled hash with no
/// published vector behind it would be a worse identity than the FNV it replaced.
#[expect(
    clippy::indexing_slicing,
    reason = "every index here is a compile-time-bounded walk over fixed [u32; 64] / [u32; 8] arrays and a 64-byte chunk; rewriting it through get() would obscure the FIPS 180-4 correspondence that makes it reviewable"
)]
fn sha256(message: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];

    let mut padded = message.to_vec();
    padded.push(0x80);
    while padded.len().checked_rem(64) != Some(56) {
        padded.push(0);
    }
    let bits = (message.len() as u64).wrapping_mul(8);
    padded.extend_from_slice(&bits.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (index, word) in chunk.chunks_exact(4).enumerate() {
            w[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }

        let mut v = state;
        for index in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let choose = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let temp1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(choose)
                .wrapping_add(SHA256_K[index])
                .wrapping_add(w[index]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let majority = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let temp2 = s0.wrapping_add(majority);
            v[7] = v[6];
            v[6] = v[5];
            v[5] = v[4];
            v[4] = v[3].wrapping_add(temp1);
            v[3] = v[2];
            v[2] = v[1];
            v[1] = v[0];
            v[0] = temp1.wrapping_add(temp2);
        }
        for (slot, value) in state.iter_mut().zip(v) {
            *slot = slot.wrapping_add(value);
        }
    }

    let mut out = [0u8; 32];
    for (slot, word) in out.chunks_exact_mut(4).zip(state) {
        slot.copy_from_slice(&word.to_be_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The published FIPS 180-4 vectors. A hand-rolled hash is only worth more than the FNV it
    /// replaced if something outside this repository says it is right, and these are that
    /// something: the one-block ("abc"), two-block (56 bytes, exercising a second chunk and the
    /// length encoding), and empty (padding-only, no message bytes at all) cases.
    #[test]
    fn the_digest_matches_the_published_sha256_vectors() {
        assert_eq!(
            book_digest("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            "FIPS 180-4 one-block vector"
        );
        assert_eq!(
            book_digest("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            "FIPS 180-4 two-block vector"
        );
        assert_eq!(
            book_digest(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "the empty message, which is pure padding"
        );
    }

    /// One flipped byte must move the digest. Trivially true for any real hash, and worth pinning
    /// anyway: this is the whole property the durable's book-identity check rests on, and a digest
    /// that silently degenerated (a mis-wired state array returning the constants) would still
    /// pass a vectors-only test if someone later "simplified" the padding.
    #[test]
    fn a_one_byte_change_moves_the_digest() {
        assert_ne!(
            book_digest("apt-get update"),
            book_digest("apt-get upgrade")
        );
        assert_eq!(book_digest("abc").len(), 64, "rendered as full hex");
    }

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
