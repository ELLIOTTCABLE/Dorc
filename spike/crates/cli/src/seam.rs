//! The one bundle of typed nondeterministic edges the engine consumes
//! (`30X:model-seams-are-one-bundle`).
//!
//! Every edge whose answer is not a pure function of the book — the clock, receipt-id and key
//! entropy, the attempt nonce, the stdout posture, the source-match query, the transport, and the
//! receipt roots — is a member of ONE bundle, and each member selects its implementation
//! independently. [`Seams`] is the union the composition root ([`crate::compose::run`]) takes;
//! [`Seams::os`] is the ONLY constructor of the production variants, called from `bin/dorc.rs` and
//! the livetest composition and nowhere else. [`HarnessSeams`] is the constructor-side subtype the
//! ordinary harness parses from its environment: it structurally CANNOT name the production
//! transport (`RealSsh`) or the production roots (`Os`) — the type has no such variant
//! (`30X:bin-harness-sibling-not-produced-cli`, `rul-fixture-identity-never-production`: the type
//! is the fence, never a runtime check).
//!
//! VALUES cross into the engine; the queries stay here (`lib-target-is-a-loom-seam`). A `Seeded`
//! member is dependency-free (`30X:bin-seeded-entropy-is-dependency-free`): a hand-rolled generator
//! over the seed, hostsim's `lcg-only-entropy` posture, so keys and ids are deterministic per
//! (case, seed) with no `rand` dependency.

#![expect(
    clippy::result_large_err,
    reason = "cold seam-parse path; the Diag is the harness print seat's own value, as in main.rs"
)]

use std::path::{Path, PathBuf};

use dorc_aid::diag::{Diag, DiagCode};
use dorc_receipt::ids::{EntropyReceiptIds, ReceiptIdEntropy, ReceiptIdSource};
use dorc_receipt_crypto::{EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator};
// The seam-variable NAMES live in the shared substrate (`30X:loom-seams-are-sh-lines`); this crate owns the PARSER over them.
use dorc_testbed::seam_vars::{
    CLOCK_ENV, KEY_ENTROPY_ENV, NONCE_ENV, POSTURE_ENV, RECEIPT_IDS_ENV, ROOTS_ENV, SEAM_ENV_VARS,
    SEED_ENV, SOURCE_MATCH_ENV, TRANSPORT_ENV,
};
use dorc_transport::{SessionDriver, SshOptions};

use crate::SourceMatch;
use crate::artifact::StdoutPosture;
use crate::durable::{RootEnvironment, host_platform, standard_roots};
use crate::results::RunClock;

/// A tiny deterministic linear-congruential generator — the ONLY entropy a `Seeded` seam draws.
///
/// Hand-rolled with the common 64-bit LCG constants (Knuth/PCG lineage), matching hostsim's
/// `lcg-only-entropy` posture so a seeded seam pulls in no `rand` dependency and stays
/// bit-reproducible from its seed (`inv-determinism`). A cli-private copy is deliberate — sharing
/// one generator with hostsim is not this lane's question.
#[derive(Debug, Clone)]
struct SeamLcg(u64);

impl SeamLcg {
    /// Seed the generator. Same seed ⇒ same stream, forever.
    const fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// The next 64-bit draw (advances the state).
    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// Fill `raw` with deterministic bytes, eight at a time from full draws.
    fn fill(&mut self, raw: &mut [u8]) {
        for chunk in raw.chunks_mut(8) {
            let bytes = self.next_u64().to_le_bytes();
            for (dst, src) in chunk.iter_mut().zip(bytes) {
                *dst = src;
            }
        }
    }
}

/// Per-seam salts folded into [`DORC_SEED`]'s umbrella value, so two `Seeded` members never share a
/// stream (`30X:loom-seams-are-sh-lines`: seed + a per-seam salt). The values are arbitrary
/// distinct constants; only their distinctness is load-bearing.
const RECEIPT_ID_SALT: u64 = 0x5245_4345_4950_5449; // "RECEIPTI"
const KEY_ENTROPY_SALT: u64 = 0x4b45_5945_4e54_5250; // "KEYENTRP"
const NONCE_SALT: u64 = 0x4e4f_4e43_4553_4544; // "NONCESED"

/// A `Seeded` receipt-id entropy source: the hand-rolled generator standing in for the platform's
/// randomness (`30X:bin-seeded-entropy-is-dependency-free`), so a receipt id is deterministic per
/// seed. Never reachable from `bin/dorc.rs` — [`Seams::os`] constructs only [`crate::receipt_edge::OsEntropy`].
#[derive(Debug)]
pub struct SeededReceiptIdEntropy(SeamLcg);

impl SeededReceiptIdEntropy {
    fn from_seed(seed: u64) -> Self {
        Self(SeamLcg::new(seed ^ RECEIPT_ID_SALT))
    }
}

impl ReceiptIdEntropy for SeededReceiptIdEntropy {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        self.0.fill(raw);
        true
    }
}

/// A `Seeded` key-secret entropy source, on [`SeededReceiptIdEntropy`]'s footing but its own
/// distinct stream (`inv-key-roles-never-meet` in spirit: the two draws never coincide).
#[derive(Debug)]
pub struct SeededKeyEntropy(SeamLcg);

impl SeededKeyEntropy {
    fn from_seed(seed: u64) -> Self {
        Self(SeamLcg::new(seed ^ KEY_ENTROPY_SALT))
    }
}

impl KeySecretEntropy for SeededKeyEntropy {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        self.0.fill(raw);
        true
    }
}

/// A deterministic, marker-safe attempt nonce derived from a seed — the `Seeded` nonce seam. The
/// bytes are lowercase hex of one generator draw, which the session marker's ascii-alphanumeric
/// charset accepts.
fn seeded_nonce(seed: u64) -> String {
    let mut lcg = SeamLcg::new(seed ^ NONCE_SALT);
    format!("s{:016x}", lcg.next_u64())
}

/// The base instant a `Seeded` clock ticks from. Linear in the seed and rooted at a plausible 2026
/// epoch so a re-blessed date reads as a real morning; monotonic in the seed so a runner giving
/// later blocks larger seeds gets later — and distinct — receipt order tokens
/// (`30X`-lane-a: the per-block base offset that retired the shared-order-token constant).
const SEEDED_CLOCK_EPOCH_MS: u64 = 1_769_306_437_000;
/// How far apart two consecutive seeds sit — one day, so distinct seeds are unmistakably distinct
/// instants.
const SEEDED_CLOCK_SPACING_MS: u64 = 86_400_000;
/// The non-zero step a seeded clock advances by per read, so two publishes in ONE invocation take
/// distinct order tokens rather than sharing one (`30X`-lane-a).
const SEEDED_CLOCK_STEP_MS: u64 = 1000;

/// The instant a `Seeded(seed)` clock starts at.
const fn seeded_clock_base(seed: u64) -> u64 {
    SEEDED_CLOCK_EPOCH_MS.saturating_add(seed.saturating_mul(SEEDED_CLOCK_SPACING_MS))
}

/// The clock member: a seeded ticking clock, a pinned instant, the real wall clock, or none.
#[derive(Debug, Clone)]
pub enum ClockSeam {
    /// A ticking clock from a seed-derived base with a non-zero step.
    Seeded(u64),
    /// A fixed instant, no step — the shape a case pins to date one exact output.
    Pinned(u64),
    /// The real wall clock (`Os`).
    Os,
    /// No clock at all — the shape that drives the undated-run path (a run whose receipt cannot
    /// take an order token), which a platform whose clock cannot be placed also reaches at `Os`.
    Absent,
}

impl ClockSeam {
    /// Materialize a fresh [`RunClock`]. `Os` reads the platform once, here, at the edge.
    fn into_clock(self) -> RunClock {
        match self {
            Self::Seeded(seed) => RunClock::Ticking {
                at: dorc_core::RunInstant(seeded_clock_base(seed)),
                step_millis: SEEDED_CLOCK_STEP_MS,
            },
            Self::Pinned(ms) => RunClock::Ticking {
                at: dorc_core::RunInstant(ms),
                step_millis: 0,
            },
            Self::Os => system_clock(),
            Self::Absent => RunClock::Absent,
        }
    }
}

/// The ONE wall-clock read (`io-at-edges-only`). A clock the platform cannot place after the epoch
/// answers [`RunClock::Absent`] rather than fabricating a zero (`inv-no-throw`).
fn system_clock() -> RunClock {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|d| u64::try_from(d.as_millis()).ok())
        .map_or(RunClock::Absent, |millis| RunClock::Ticking {
            at: dorc_core::RunInstant(millis),
            step_millis: 0,
        })
}

/// The receipt-id entropy member: seeded, or the platform's randomness.
#[derive(Debug, Clone)]
pub enum ReceiptIdSeam {
    /// Deterministic ids from a seed.
    Seeded(u64),
    /// The operating system's randomness (`Os`).
    Os,
}

impl ReceiptIdSeam {
    /// Build the id source this member selects.
    fn into_source(self) -> Box<dyn ReceiptIdSource> {
        match self {
            Self::Seeded(seed) => Box::new(EntropyReceiptIds::over(
                SeededReceiptIdEntropy::from_seed(seed),
            )),
            Self::Os => Box::new(EntropyReceiptIds::over(crate::receipt_edge::OsEntropy)),
        }
    }
}

/// The key-secret entropy member: seeded, or the platform's randomness.
#[derive(Debug, Clone)]
pub enum KeyEntropySeam {
    /// Deterministic key material from a seed.
    Seeded(u64),
    /// The operating system's randomness (`Os`).
    Os,
}

impl KeyEntropySeam {
    /// Build the keyset generator this member selects.
    fn into_generator(self) -> Box<dyn KeysetGenerator> {
        match self {
            Self::Seeded(seed) => Box::new(EntropyKeysetGenerator::over(
                SeededKeyEntropy::from_seed(seed),
            )),
            Self::Os => Box::new(EntropyKeysetGenerator::over(crate::durable::OsKeyEntropy)),
        }
    }
}

/// The attempt-nonce member: a seeded marker-safe nonce, or a minted one.
#[derive(Debug, Clone)]
pub enum NonceSeam {
    /// A deterministic marker-safe nonce from a seed.
    Seeded(u64),
    /// A minted nonce, unique per attempt from the clock and pid (`Os`).
    Os,
}

impl NonceSeam {
    /// Mint the run's nonce this member selects.
    fn mint(&self) -> String {
        match self {
            Self::Seeded(seed) => seeded_nonce(*seed),
            Self::Os => crate::transport_edge::minted_process_nonce(),
        }
    }
}

/// The stdout-posture member: pinned to a cell, or detected from the terminal.
#[derive(Debug, Clone, Copy)]
pub enum PostureSeam {
    /// A person is reading it (`interactive`).
    PinnedInteractive,
    /// It is being kept for review (`kept`).
    PinnedKept,
    /// Ask the terminal itself (`Os`).
    Os,
}

impl PostureSeam {
    /// Resolve the posture; `Os` performs the one terminal read at the edge.
    fn resolve(self) -> StdoutPosture {
        use std::io::IsTerminal as _;
        match self {
            Self::PinnedInteractive => StdoutPosture::Interactive,
            Self::Os if std::io::stdout().is_terminal() => StdoutPosture::Interactive,
            Self::PinnedKept | Self::Os => StdoutPosture::NonInteractive,
        }
    }
}

/// The source-match member: pinned to an answer, or asked of the real repository.
#[derive(Debug, Clone)]
pub enum SourceMatchSeam {
    /// No annotation at all, whatever the repository would say (the old `off`).
    PinnedAbsent,
    /// A match at this exact commit, no repository needed (the old `<commit>`).
    PinnedAt(String),
    /// Ask the real `git` repository (`Os`).
    Os,
}

impl SourceMatchSeam {
    /// Resolve whether the book sits at a nameable commit.
    fn resolve(&self, book_name: &str) -> Option<SourceMatch> {
        match self {
            Self::PinnedAbsent => None,
            Self::PinnedAt(commit) => Some(SourceMatch {
                commit: commit.clone(),
            }),
            Self::Os => crate::source_match::resolve(
                &crate::source_match::GitRepository,
                Path::new(book_name),
            ),
        }
    }
}

/// Proof a value was minted by the production composition root ([`Seams::os`]).
///
/// Its only constructor is private to this module and [`Seams::os`] holds the sole call, so
/// [`TransportSeam::RealSsh`] and [`RootsSeam::Os`] — the two production variants — cannot be
/// spelled anywhere else, `From<HarnessSeams>` included (`rul-fixture-identity-never-production`:
/// the type is the fence). A test in this crate cannot forge one either.
#[derive(Debug, Clone, Copy)]
pub struct ProductionWitness(());

/// A local transport interpreter — the non-ssh driver a fixture selects (`local:<shell>[:<interp>]`).
#[derive(Debug, Clone)]
pub struct LocalTransport {
    /// The shell binary to run the shipped artifact through.
    pub shell: PathBuf,
    /// The shell's own name for itself, when it differs from the OS's name for it.
    pub interpreter: Option<String>,
}

/// The transport member. `RealSsh` is production-only (`Seams::os`); the local interpreter is the
/// fixture; there is NO `Os` variant (`30X:model-seams-are-one-bundle`: transport has no `Os`).
#[derive(Debug, Clone)]
pub enum TransportSeam {
    /// The production ssh driver — mintable only by [`Seams::os`].
    RealSsh(ProductionWitness),
    /// A local fixture interpreter, or `None` when the harness set no transport (a `--host` run
    /// then reaches an unspawnable driver and lands on the ordinary host-not-reached path).
    Local(Option<LocalTransport>),
}

impl TransportSeam {
    /// Build the session driver this member selects, given the invocation's ssh options.
    fn into_driver(
        self,
        connect_timeout: Option<u64>,
        accept_new: bool,
        ssh_config: Option<&str>,
    ) -> Box<dyn SessionDriver> {
        match self {
            Self::RealSsh(_) => {
                let mut options = SshOptions {
                    accept_new_host_key: accept_new,
                    config_file: ssh_config.map(PathBuf::from),
                    ..SshOptions::default()
                };
                if let Some(secs) = connect_timeout {
                    options.connect_timeout = std::time::Duration::from_secs(secs);
                }
                Box::new(dorc_transport::SshDriver::new(options))
            }
            Self::Local(Some(local)) => Box::new(match local.interpreter {
                Some(interpreter) if !interpreter.is_empty() => {
                    dorc_transport::LocalDriver::new(local.shell, interpreter)
                }
                _ => dorc_transport::LocalDriver::same_spelling(local.shell),
            }),
            // Unconfigured harness transport: an empty spelling `--host` fails to spawn against.
            Self::Local(None) => {
                Box::new(dorc_transport::LocalDriver::same_spelling(PathBuf::new()))
            }
        }
    }
}

/// The receipt-roots member. `Os` (the platform resolution behind [`RootEnvironment`]) is
/// production-only (`Seams::os`); `Pinned` carries a runner-owned throwaway directory — there is NO
/// `Os` variant in the harness subtype (`30X:bin-harness-sibling-not-produced-cli`; persistence
/// under runner-owned roots is not a production variant).
#[derive(Debug, Clone)]
pub enum RootsSeam {
    /// The platform-resolved per-user config/state roots — mintable only by [`Seams::os`].
    Os(ProductionWitness),
    /// A runner-owned throwaway directory the config/state roots derive UNDER, carried as a literal
    /// (`30Xa:rul-roots-pinned-is-a-literal`): it consults no platform variable, so a scrubbed
    /// session with no `APPDATA`/`HOME`/`XDG_*` still resolves.
    Pinned(PathBuf),
}

impl RootsSeam {
    /// Resolve this member's base roots. `Os` reads the platform variables through
    /// [`RootEnvironment`]; `Pinned` derives them under its literal directory and reads no
    /// environment (`30Xa:rul-roots-pinned-is-a-literal`).
    fn base_roots(
        &self,
        environment: &dyn RootEnvironment,
    ) -> Result<dorc_receipt_local::RootInputs, dorc_receipt_local::RootRefusal> {
        match self {
            Self::Os(_) => standard_roots(host_platform(), environment),
            Self::Pinned(directory) => pinned_roots(directory),
        }
    }
}

/// Derive the config/state roots UNDER a runner-owned directory, consulting no platform variable
/// (`30Xa:rul-roots-pinned-is-a-literal`). The two roles stay separate subdirectories, exactly as
/// every platform keeps them and as the runner creates them, so a pinned run writes precisely where
/// the platform route would under a runner-owned sandbox.
fn pinned_roots(
    directory: &Path,
) -> Result<dorc_receipt_local::RootInputs, dorc_receipt_local::RootRefusal> {
    let base = |role: &str| directory.join(role).to_string_lossy().into_owned();
    dorc_receipt_local::RootInputs::of(host_platform(), &base("config"), &base("state"))
}

/// The full bundle the engine's composition root consumes. Its fields are PRIVATE and its only
/// constructors are [`Seams::os`] and `From<HarnessSeams>`, so no mixed fixture/production bundle is
/// ever built (`rul-fixture-identity-never-production`; `sinv-private-authority-mints`).
#[derive(Debug, Clone)]
pub struct Seams {
    clock: ClockSeam,
    receipt_ids: ReceiptIdSeam,
    key_entropy: KeyEntropySeam,
    attempt_nonce: NonceSeam,
    stdout_posture: PostureSeam,
    source_match: SourceMatchSeam,
    transport: TransportSeam,
    roots: RootsSeam,
}

impl Seams {
    /// The all-production row: the shipped `dorc`'s ONLY bundle, and the sole constructor of the
    /// production variants (`RealSsh`, `Os` roots). Called from `bin/dorc.rs` and the livetest
    /// composition and nowhere else (`30X:inv-fixture-state-never-typeable-into-main`).
    #[must_use]
    pub fn os() -> Self {
        Self {
            clock: ClockSeam::Os,
            receipt_ids: ReceiptIdSeam::Os,
            key_entropy: KeyEntropySeam::Os,
            attempt_nonce: NonceSeam::Os,
            stdout_posture: PostureSeam::Os,
            source_match: SourceMatchSeam::Os,
            transport: TransportSeam::RealSsh(ProductionWitness(())),
            roots: RootsSeam::Os(ProductionWitness(())),
        }
    }

    /// Materialize this run's clock (once, at the edge).
    #[must_use]
    pub fn clock(&self) -> RunClock {
        self.clock.clone().into_clock()
    }

    /// The receipt-id source this run mints identities with.
    #[must_use]
    pub fn receipt_id_source(&self) -> Box<dyn ReceiptIdSource> {
        self.receipt_ids.clone().into_source()
    }

    /// The keyset generator this run initializes a first-use keyset with.
    #[must_use]
    pub fn keyset_generator(&self) -> Box<dyn KeysetGenerator> {
        self.key_entropy.clone().into_generator()
    }

    /// This run's attempt nonce.
    #[must_use]
    pub fn attempt_nonce(&self) -> String {
        self.attempt_nonce.mint()
    }

    /// This run's stdout posture (the one terminal read at the edge, if `Os`).
    #[must_use]
    pub fn stdout_posture(&self) -> StdoutPosture {
        self.stdout_posture.resolve()
    }

    /// Whether the book sits at a nameable commit.
    #[must_use]
    pub fn source_match(&self, book_name: &str) -> Option<SourceMatch> {
        self.source_match.resolve(book_name)
    }

    /// The session driver this run reaches a host through.
    #[must_use]
    pub fn transport_driver(
        &self,
        connect_timeout: Option<u64>,
        accept_new: bool,
        ssh_config: Option<&str>,
    ) -> Box<dyn SessionDriver> {
        self.transport
            .clone()
            .into_driver(connect_timeout, accept_new, ssh_config)
    }

    /// The base receipt roots this run resolves.
    ///
    /// # Errors
    /// Refuses when the platform cannot place a per-user root.
    pub fn base_roots(
        &self,
        environment: &dyn RootEnvironment,
    ) -> Result<dorc_receipt_local::RootInputs, dorc_receipt_local::RootRefusal> {
        self.roots.base_roots(environment)
    }
}

/// The transport subtype the ordinary harness parses: a local interpreter or nothing. No `RealSsh`
/// variant exists (`30X:bin-harness-sibling-not-produced-cli`).
#[derive(Debug, Clone)]
pub enum HarnessTransportSeam {
    /// The fixture local interpreter (`local:<shell>[:<interp>]`).
    Local(LocalTransport),
    /// No transport configured — a `--host` run reaches an unspawnable driver.
    Unset,
}

/// The roots subtype the ordinary harness parses: a runner-owned pinned directory, never `Os`
/// (`30X:bin-harness-sibling-not-produced-cli`). It carries the throwaway directory as a literal
/// (`30Xa:rul-roots-pinned-is-a-literal`).
#[derive(Debug, Clone)]
pub enum HarnessRootsSeam {
    /// The runner's throwaway directory, the config/state roots derive under it.
    Pinned(PathBuf),
}

/// The constructor-side subtype the ordinary harness parses from its environment. It shares the
/// value seams with [`Seams`] but its transport and roots are the fenced subtypes: it cannot name
/// `RealSsh` or `Os` roots (`30X:bin-harness-sibling-not-produced-cli`).
#[derive(Debug, Clone)]
pub struct HarnessSeams {
    clock: ClockSeam,
    receipt_ids: ReceiptIdSeam,
    key_entropy: KeyEntropySeam,
    attempt_nonce: NonceSeam,
    stdout_posture: PostureSeam,
    source_match: SourceMatchSeam,
    transport: HarnessTransportSeam,
    roots: HarnessRootsSeam,
}

impl From<HarnessSeams> for Seams {
    /// Total, and it can never produce a production variant: the local transport maps to
    /// `Local(Some|None)` and the pinned roots to `Pinned`, so no `From` path spells `RealSsh` or
    /// `Os` roots (`30X:bin-harness-sibling-not-produced-cli`).
    fn from(harness: HarnessSeams) -> Self {
        Self {
            clock: harness.clock,
            receipt_ids: harness.receipt_ids,
            key_entropy: harness.key_entropy,
            attempt_nonce: harness.attempt_nonce,
            stdout_posture: harness.stdout_posture,
            source_match: harness.source_match,
            transport: match harness.transport {
                HarnessTransportSeam::Local(local) => TransportSeam::Local(Some(local)),
                HarnessTransportSeam::Unset => TransportSeam::Local(None),
            },
            roots: match harness.roots {
                HarnessRootsSeam::Pinned(directory) => RootsSeam::Pinned(directory),
            },
        }
    }
}

/// The narrow read the seam parser needs from an environment: one variable, or `None` when it is
/// unset or empty. The real process supplies [`ProcessSeamEnv`]; lane B's modelled session supplies
/// its own (`30X:loom-seams-are-sh-lines`: ONE parser serves both).
pub trait SeamEnv {
    /// The value of one variable, or `None` where it is unset or empty.
    fn var(&self, name: &str) -> Option<String>;
}

/// The real process environment, as the seam parser's query.
#[derive(Debug, Default)]
pub struct ProcessSeamEnv;

impl SeamEnv for ProcessSeamEnv {
    fn var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.is_empty())
    }
}

impl HarnessSeams {
    /// Whether the environment names any seam at all — the harness binary refuses loudly when it
    /// does not, so it can never be mistaken for the shipped product
    /// (`30X:bin-harness-sibling-not-produced-cli`).
    #[must_use]
    pub fn any_seam_set(environment: &dyn SeamEnv) -> bool {
        SEAM_ENV_VARS
            .iter()
            .any(|name| environment.var(name).is_some())
    }

    /// Parse the harness bundle from the environment. A malformed selection is a typed refusal,
    /// never a silent default (`30X:loom-seams-are-sh-lines`).
    ///
    /// # Errors
    /// Returns a typed invocation diagnostic naming the variable and its unrecognized value.
    pub fn from_env(environment: &dyn SeamEnv) -> Result<Self, Diag> {
        let umbrella = umbrella_seed(environment)?;
        Ok(Self {
            clock: parse_clock(environment, umbrella)?,
            receipt_ids: parse_receipt_ids(environment, umbrella)?,
            key_entropy: parse_key_entropy(environment, umbrella)?,
            attempt_nonce: parse_nonce(environment, umbrella)?,
            stdout_posture: parse_posture(environment)?,
            source_match: parse_source_match(environment)?,
            transport: parse_transport(environment)?,
            roots: parse_roots(environment)?,
        })
    }
}

/// The umbrella `DORC_SEED`, defaulting to 0 when unset (a bare seeded run is deterministic).
fn umbrella_seed(environment: &dyn SeamEnv) -> Result<u64, Diag> {
    match environment.var(SEED_ENV) {
        None => Ok(0),
        Some(raw) => raw
            .parse::<u64>()
            .map_err(|_| seam_value_error(SEED_ENV, &raw, "a u64")),
    }
}

/// A `seeded[:<u64>]` selection — the umbrella when no explicit seed is spelled.
fn seeded_or_umbrella(name: &str, rest: Option<&str>, umbrella: u64) -> Result<u64, Diag> {
    match rest {
        None => Ok(umbrella),
        Some(value) => value
            .parse::<u64>()
            .map_err(|_| seam_value_error(name, value, "a u64")),
    }
}

fn parse_clock(environment: &dyn SeamEnv, umbrella: u64) -> Result<ClockSeam, Diag> {
    let Some(raw) = environment.var(CLOCK_ENV) else {
        return Ok(ClockSeam::Seeded(umbrella));
    };
    let (head, rest) = split_selection(&raw);
    match head {
        "seeded" => Ok(ClockSeam::Seeded(seeded_or_umbrella(
            CLOCK_ENV, rest, umbrella,
        )?)),
        "pinned" => rest
            .and_then(|v| v.parse::<u64>().ok())
            .map(ClockSeam::Pinned)
            .ok_or_else(|| seam_value_error(CLOCK_ENV, &raw, "pinned:<unix-millis>")),
        "os" => Ok(ClockSeam::Os),
        "absent" => Ok(ClockSeam::Absent),
        _ => Err(seam_value_error(
            CLOCK_ENV,
            &raw,
            "seeded[:<u64>] | pinned:<ms> | os | absent",
        )),
    }
}

fn parse_receipt_ids(environment: &dyn SeamEnv, umbrella: u64) -> Result<ReceiptIdSeam, Diag> {
    let Some(raw) = environment.var(RECEIPT_IDS_ENV) else {
        return Ok(ReceiptIdSeam::Seeded(umbrella));
    };
    let (head, rest) = split_selection(&raw);
    match head {
        "seeded" => Ok(ReceiptIdSeam::Seeded(seeded_or_umbrella(
            RECEIPT_IDS_ENV,
            rest,
            umbrella,
        )?)),
        "os" => Ok(ReceiptIdSeam::Os),
        _ => Err(seam_value_error(
            RECEIPT_IDS_ENV,
            &raw,
            "seeded[:<u64>] | os",
        )),
    }
}

fn parse_key_entropy(environment: &dyn SeamEnv, umbrella: u64) -> Result<KeyEntropySeam, Diag> {
    let Some(raw) = environment.var(KEY_ENTROPY_ENV) else {
        return Ok(KeyEntropySeam::Seeded(umbrella));
    };
    let (head, rest) = split_selection(&raw);
    match head {
        "seeded" => Ok(KeyEntropySeam::Seeded(seeded_or_umbrella(
            KEY_ENTROPY_ENV,
            rest,
            umbrella,
        )?)),
        "os" => Ok(KeyEntropySeam::Os),
        _ => Err(seam_value_error(
            KEY_ENTROPY_ENV,
            &raw,
            "seeded[:<u64>] | os",
        )),
    }
}

fn parse_nonce(environment: &dyn SeamEnv, umbrella: u64) -> Result<NonceSeam, Diag> {
    let Some(raw) = environment.var(NONCE_ENV) else {
        return Ok(NonceSeam::Seeded(umbrella));
    };
    let (head, rest) = split_selection(&raw);
    match head {
        "seeded" => Ok(NonceSeam::Seeded(seeded_or_umbrella(
            NONCE_ENV, rest, umbrella,
        )?)),
        "os" => Ok(NonceSeam::Os),
        _ => Err(seam_value_error(NONCE_ENV, &raw, "seeded[:<u64>] | os")),
    }
}

fn parse_posture(environment: &dyn SeamEnv) -> Result<PostureSeam, Diag> {
    let Some(raw) = environment.var(POSTURE_ENV) else {
        return Ok(PostureSeam::Os);
    };
    match raw.as_str() {
        "pinned:interactive" => Ok(PostureSeam::PinnedInteractive),
        "pinned:kept" => Ok(PostureSeam::PinnedKept),
        "os" => Ok(PostureSeam::Os),
        _ => Err(seam_value_error(
            POSTURE_ENV,
            &raw,
            "pinned:interactive | pinned:kept | os",
        )),
    }
}

fn parse_source_match(environment: &dyn SeamEnv) -> Result<SourceMatchSeam, Diag> {
    let Some(raw) = environment.var(SOURCE_MATCH_ENV) else {
        return Ok(SourceMatchSeam::Os);
    };
    match split_selection(&raw) {
        ("pinned", Some("off")) => Ok(SourceMatchSeam::PinnedAbsent),
        ("pinned", Some(commit)) => Ok(SourceMatchSeam::PinnedAt(commit.to_owned())),
        ("os", None) => Ok(SourceMatchSeam::Os),
        _ => Err(seam_value_error(
            SOURCE_MATCH_ENV,
            &raw,
            "pinned:off | pinned:<commit> | os",
        )),
    }
}

fn parse_transport(environment: &dyn SeamEnv) -> Result<HarnessTransportSeam, Diag> {
    let Some(raw) = environment.var(TRANSPORT_ENV) else {
        return Ok(HarnessTransportSeam::Unset);
    };
    // `;` (not `:`) splits the optional interpreter off the END: a Windows shell PATH carries a
    // drive-letter colon a `:` would split, and neither a path nor an interpreter contains `;`.
    match raw.strip_prefix("local:") {
        Some(rest) => {
            let (shell, interpreter) = match rest.rsplit_once(';') {
                Some((shell, interpreter)) => (shell, Some(interpreter.to_owned())),
                None => (rest, None),
            };
            Ok(HarnessTransportSeam::Local(LocalTransport {
                shell: PathBuf::from(shell),
                interpreter,
            }))
        }
        None => Err(seam_value_error(
            TRANSPORT_ENV,
            &raw,
            "local:<shell>[;<interpreter>]",
        )),
    }
}

/// Parse the roots seam. `pinned:<absolute dir>` is the ONLY spelling — the harness cannot name
/// `Os` roots (`30X:bin-harness-sibling-not-produced-cli`), and a runner-owned directory has no
/// sensible default, so an absent or path-less `pinned` is a typed refusal
/// (`30Xa:rul-roots-pinned-is-a-literal`), never a silent fallback (`30X:loom-seams-are-sh-lines`).
fn parse_roots(environment: &dyn SeamEnv) -> Result<HarnessRootsSeam, Diag> {
    let Some(raw) = environment.var(ROOTS_ENV) else {
        return Err(seam_value_error(ROOTS_ENV, "", "pinned:<absolute dir>"));
    };
    match split_selection(&raw) {
        ("pinned", Some(directory)) if !directory.is_empty() => {
            Ok(HarnessRootsSeam::Pinned(PathBuf::from(directory)))
        }
        _ => Err(seam_value_error(ROOTS_ENV, &raw, "pinned:<absolute dir>")),
    }
}

/// Split `head[:rest]` on the FIRST colon, so a value can itself carry colons (a Windows shell path).
fn split_selection(raw: &str) -> (&str, Option<&str>) {
    match raw.split_once(':') {
        Some((head, rest)) => (head, Some(rest)),
        None => (raw, None),
    }
}

/// A malformed seam selection, as the shared invocation-error carrier.
fn seam_value_error(variable: &str, got: &str, expected: &'static str) -> Diag {
    Diag::new_spanless_site(DiagCode::CliFlagValueNotRecognized(
        dorc_aid::diag::CliFlagValueNotRecognized {
            flag: variable.to_owned(),
            got: got.to_owned(),
            expected,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    struct Env(BTreeMap<&'static str, &'static str>);
    impl SeamEnv for Env {
        fn var(&self, name: &str) -> Option<String> {
            self.0.get(name).map(|v| (*v).to_owned())
        }
    }
    /// A test environment carrying `pairs`, with a valid roots pin injected unless one is spelled —
    /// roots is required (`30Xa:rul-roots-pinned-is-a-literal`), so an unrelated case would else
    /// refuse before reaching what it means to check.
    fn env(pairs: &[(&'static str, &'static str)]) -> Env {
        let mut map: BTreeMap<&'static str, &'static str> = pairs.iter().copied().collect();
        map.entry(ROOTS_ENV).or_insert("pinned:/srv/throwaway");
        Env(map)
    }

    #[test]
    fn a_seed_makes_ids_and_keys_deterministic_and_independent() {
        let mut a = SeededReceiptIdEntropy::from_seed(7);
        let mut b = SeededReceiptIdEntropy::from_seed(7);
        let (mut ra, mut rb) = ([0_u8; 32], [0_u8; 32]);
        a.fill(&mut ra);
        b.fill(&mut rb);
        assert_eq!(ra, rb, "same seed, same bytes");

        let mut key = SeededKeyEntropy::from_seed(7);
        let mut rk = [0_u8; 32];
        key.fill(&mut rk);
        assert_ne!(
            ra, rk,
            "the id and key streams must not share a draw at one seed"
        );
    }

    #[test]
    fn the_seeded_clock_is_monotonic_in_the_seed_and_steps_within_a_block() {
        let RunClock::Ticking { at: block0, .. } = ClockSeam::Seeded(0).into_clock() else {
            panic!("seeded is ticking");
        };
        let RunClock::Ticking { at: block1, .. } = ClockSeam::Seeded(1).into_clock() else {
            panic!("seeded is ticking");
        };
        assert!(block1.0 > block0.0, "a later block sits later");
        let mut clock = ClockSeam::Seeded(0).into_clock();
        let first = clock.now().expect("a ticking clock reads");
        let second = clock.now().expect("a ticking clock reads");
        assert!(
            second.0 > first.0,
            "a second read inside one block advances"
        );
    }

    #[test]
    fn a_seeded_nonce_survives_the_marker_charset() {
        let nonce = seeded_nonce(42);
        assert!(
            !nonce.is_empty() && nonce.chars().all(|c| c.is_ascii_alphanumeric()),
            "a seeded nonce must be marker-safe: {nonce}"
        );
        assert_eq!(nonce, seeded_nonce(42), "same seed, same nonce");
    }

    #[test]
    fn with_only_roots_pinned_every_other_member_defaults_to_seeded_or_os() {
        let seams = HarnessSeams::from_env(&env(&[])).expect("a bare environment parses");
        assert!(matches!(seams.clock, ClockSeam::Seeded(0)));
        assert!(matches!(seams.receipt_ids, ReceiptIdSeam::Seeded(0)));
        assert!(matches!(seams.stdout_posture, PostureSeam::Os));
        assert!(matches!(seams.transport, HarnessTransportSeam::Unset));
        assert!(matches!(seams.roots, HarnessRootsSeam::Pinned(_)));
    }

    #[test]
    fn the_umbrella_seed_feeds_every_seeded_member() {
        let seams = HarnessSeams::from_env(&env(&[(SEED_ENV, "9")])).expect("parses");
        assert!(matches!(seams.clock, ClockSeam::Seeded(9)));
        assert!(matches!(seams.receipt_ids, ReceiptIdSeam::Seeded(9)));
        assert!(matches!(seams.key_entropy, KeyEntropySeam::Seeded(9)));
        assert!(matches!(seams.attempt_nonce, NonceSeam::Seeded(9)));
    }

    #[test]
    fn an_explicit_member_seed_overrides_the_umbrella() {
        let seams = HarnessSeams::from_env(&env(&[(SEED_ENV, "9"), (CLOCK_ENV, "seeded:3")]))
            .expect("parses");
        assert!(matches!(seams.clock, ClockSeam::Seeded(3)));
        assert!(matches!(seams.receipt_ids, ReceiptIdSeam::Seeded(9)));
    }

    #[test]
    fn a_malformed_selection_is_a_typed_refusal_never_a_default() {
        assert!(
            HarnessSeams::from_env(&Env(BTreeMap::new())).is_err(),
            "no roots at all is a refusal, never a silent default"
        );
        for (var, value) in [
            (CLOCK_ENV, "seeded:not-a-number"),
            (CLOCK_ENV, "pinned:nope"),
            (CLOCK_ENV, "wat"),
            (RECEIPT_IDS_ENV, "pinned:5"),
            (POSTURE_ENV, "loud"),
            (SEED_ENV, "-1"),
            (ROOTS_ENV, "pinned"),
            (ROOTS_ENV, "os"),
        ] {
            assert!(
                HarnessSeams::from_env(&env(&[(var, value)])).is_err(),
                "{var}={value} must refuse, never fall back to a default"
            );
        }
    }

    #[test]
    fn the_transport_seam_parses_a_local_interpreter() {
        let seams =
            HarnessSeams::from_env(&env(&[(TRANSPORT_ENV, "local:/bin/dash")])).expect("parses");
        assert!(matches!(
            seams.transport,
            HarnessTransportSeam::Local(LocalTransport {
                interpreter: None,
                ..
            })
        ));
        // A Windows-shaped shell PATH keeps its drive-letter colon; the interpreter splits off `;`.
        let with_interp = HarnessSeams::from_env(&env(&[(
            TRANSPORT_ENV,
            r"local:C:\git\dash.exe;/usr/bin/dash",
        )]))
        .expect("parses");
        let HarnessTransportSeam::Local(LocalTransport { shell, interpreter }) =
            with_interp.transport
        else {
            panic!("a local transport");
        };
        assert_eq!(shell, PathBuf::from(r"C:\git\dash.exe"));
        assert_eq!(interpreter.as_deref(), Some("/usr/bin/dash"));
    }

    #[test]
    fn the_harness_bundle_never_carries_a_production_variant() {
        // The whole point of the two-type split: no `from_env` path can produce `RealSsh` transport
        // or `Os` roots. Checked structurally by converting and matching; the pinned roots carry the
        // runner's literal directory verbatim (`30Xa:rul-roots-pinned-is-a-literal`).
        let seams: Seams = HarnessSeams::from_env(&env(&[
            (TRANSPORT_ENV, "local:/bin/sh"),
            (ROOTS_ENV, "pinned:/srv/case-42"),
        ]))
        .expect("parses")
        .into();
        assert!(matches!(seams.transport, TransportSeam::Local(_)));
        let RootsSeam::Pinned(directory) = seams.roots else {
            panic!("the harness bundle resolves to pinned roots");
        };
        assert_eq!(directory, PathBuf::from("/srv/case-42"));
    }

    #[test]
    fn os_selects_the_production_query_without_a_witness_leak() {
        // `Os`-spelled value seams are reachable from the harness (a case may un-peg one member),
        // but the FENCED variants (RealSsh, Os roots) are not: only `Seams::os()` mints those.
        let seams = HarnessSeams::from_env(&env(&[(CLOCK_ENV, "os"), (RECEIPT_IDS_ENV, "os")]))
            .expect("parses");
        assert!(matches!(seams.clock, ClockSeam::Os));
        assert!(matches!(seams.receipt_ids, ReceiptIdSeam::Os));
    }
}
