//! `plan::whylog` — the thin posthoc-why durable (`27V` Lane B · `22A:concl-10` ·
//! `whylog-write-only-replay`). The durable is THIN — only what cannot be recomputed — and the
//! full narration is a rendering of a RE-RUN: `dorc why --last` replays it through the SAME kernel
//! (determinism is the license). rec-5 (`probe-tape-not-a-cache`): write-only, replay-driven,
//! nothing re-ingests it across runs for any decision.
//!
//! # Format (`tc-whylog-serialization`, conductor-ACCEPTED line-framed)
//!
//! Hand-rolled line-framed on the [`crate::records`] discipline (version-tagged header + the
//! `@@dorc@@` terminal token + free content last-to-token), NOT serde/JSON — zero deps
//! (`inv-determinism`), proven tear/glue tolerance, and the format's DECLARED byte-instability
//! (additive-only fields, no cross-version byte-stability) makes a rich schema worthless. The one
//! twist: the records stream is stored AS-RECEIVED (`tc-whylog-stores-raw-buffer`) and itself
//! contains `@@dorc@@` tokens, so it rides a BYTE-COUNT-prefixed opaque block (`results bytes=N`)
//! rather than a token-scanned region — replay then re-deframes it through the identical path (the
//! strongest determinism guarantee).
//!
//! # `inv-no-throw`
//!
//! [`parse`] is total: a truncated / clobbered / wrong-version durable yields diagnostics
//! ([`DiagCode::WhylogCorrupt`] / [`DiagCode::WhylogVersionRefused`]), never a panic.

use dorc_aid::diag::{Diag, DiagCode, WhylogCorrupt, WhylogCorruptReason, WhylogVersionRefused};

use std::io::Read;

// One digest, one substitution point (`rul-fixture-identity-never-production`): the content
// identity a durable carries has exactly ONE definition. Never re-inline it locally.
use crate::invocation::book_digest;
use crate::records::{
    Admission, AdmissionRefusal, AdmittedUnscopedHostRecords, BoundedHostBytes, Framing,
    HostEvidenceLimits, TERMINAL_TOKEN, admit_unscoped_host_records,
};

/// The durable.s version tag — the format.s identity (`27V` §2; the `report-lane-versioned-entry`
/// posture). Recognized once published; a new grammar mints a new tag. NO byte-stability within a
/// version (additive-only fields). ONE grammar exists (`rul-strawman-formats-no-compat`): the
/// permissive v1 reader is deleted, so a durable this binary cannot admit is refused rather than
/// parsed by a shape that has drifted from the writer.
pub const WHYLOG_V2_TAG: &str = "dorc-whylog/2";

/// The record-stream version this durable declares, as a number the narrative plane can compare
/// itself against (`dorc_aid::narrative::PLANE_VERSION`).
///
/// The two version together or the `[unnarrated:]` census lies about old receipts
/// (`28E:prop-unnarrated-is-visible`'s caveat). `record_stream_version_matches_the_narrative_plane`
/// is the gate; a bump here that leaves the plane behind fails it.
pub const RECORD_STREAM_VERSION: u32 = 2;
/// The v2 sentinel is exact and must be followed immediately by EOF.
pub const WHYLOG_V2_END: &str = "dorc-whylog-end/2";

/// One apply-report line (`27V` §2). SPIKE (`tc-apply-report-is-prediction`,
/// `churn-avoidance-disclosure`): there is NO apply executor (`cli/CLAUDE.md` scope-boundary), so
/// `disposition` is the PREDICTED disposition and [`predicted`](Self::predicted) is ALWAYS `true`.
/// The reader must never render a prediction in a measurement's clothes (ruling: a prediction must
/// not wear a measurement's clothes — `law-trust-tier-is-syntax`'s cousin). The field shape stays
/// additive so a real executor later fills genuine ran/guard-passed/guard-fell-through/replaced
/// outcomes + divergence flags + apply-rcs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyLine {
    /// The plan leaf id (`inv-site-keyed-results`).
    pub leaf: u32,
    /// The predicted disposition tag (`run` / `replace` / `guard` / `omit`).
    pub disposition: String,
    /// `true` ⇒ a PREDICTED disposition, not a measured apply outcome (spike: always `true`).
    pub predicted: bool,
    /// Where the decision this row projects stood relative to host contact — the DURABLE ACCOUNT
    /// EXPORT, gated by [`ACCOUNT_EXPORT`].
    ///
    /// Naming it here is the lift of `ExcludedContent::InfluenceGrade`: field-level exclusion was
    /// structural precisely because no View named the account, and this View now does. What keeps
    /// production bytes unchanged is no longer the absence of this field but the switch, and that
    /// substitution is deliberate and is the reviewable act.
    pub account: DurableAccount,
}

/// THE SWITCH for the durable account export (`30Q` §5g, human-typed flow: build it, disable it,
/// review it, then enable it).
///
/// ONE `const`, and it gates BOTH ends. `false` ⇒ the writer emits no `account=` field and the
/// reader is never handed one, so every production durable byte is exactly what it was before this
/// row existed — the byte-identity gate still binds, and it is what proves that. `true` ⇒ each
/// `apply` row carries its decision's account and replay reads it back through
/// [`DurableAccount`].
///
/// A human flips this after the review the growth of a durable's contents owes
/// (`rul-durable-contents-reviewed-before-design`). Nothing in the engine reads it as policy, no
/// flag sets it, and no environment variable reaches it: a switch a run could turn on is not a
/// switch that has been reviewed.
pub const ACCOUNT_EXPORT: bool = false;

/// An account AS THE DURABLE CARRIES IT — the flattening, at the type level
/// (`rul-influence-flattens-at-the-durable`, human-typed).
///
/// The durable transition is one-way. A live account goes IN through
/// [`of_decision`](Self::of_decision) at the View, and what comes back out on replay is this and
/// only this: there is no accessor yielding an
/// [`InfluenceAccount`](dorc_core::influence::InfluenceAccount), no `From`, and no join. So a
/// replayed account cannot be joined into a live decision's, cannot reach a license mint, and
/// cannot reach a Spine record — post-reingest, influence is REPORT/WHY-plane only, and that stops
/// being a rule somebody has to remember. It can be DISPLAYED, and that is all it can do.
///
/// `306b:rul-reingestion-drives-no-action` is why that is not redundant with rehydrating at the
/// right grade: not all persisted material is influenced, and uninfluenced persisted material
/// describes a world-moment that has passed just as surely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableAccount(dorc_core::influence::InfluenceAccount);

impl DurableAccount {
    /// Project a live decision's account into the durable plane — the ONE way in, taken at the
    /// View. One-way by construction: nothing converts back.
    #[must_use]
    pub const fn of_decision(account: dorc_core::influence::InfluenceAccount) -> Self {
        Self(account)
    }

    /// The word this account renders as, for the report and why planes, and the token the writer
    /// emits. Referent-agnostic: never branched on.
    #[must_use]
    pub const fn label(self) -> &'static str {
        self.0.label()
    }

    /// Did host-reported material reach the decision this row projects — or do we not know?
    ///
    /// A REPORT question, answered for a reader. A `bool` rather than the account itself, so that
    /// nothing a caller gets back can be joined into anything.
    #[must_use]
    pub const fn was_influenced(self) -> bool {
        self.0.is_influenced()
    }

    /// Rehydrate one `apply` row's account from the durable's closed vocabulary.
    ///
    /// ABSENT, unrecognised, or malformed all read `untracked` — the TOP of the chain, the MOST
    /// influenced point (`306b:rul-missing-influence-grade-reads-highest`). That direction is the
    /// whole safety property: removing account metadata from a durable can then only make a reader
    /// more careful, never less, so metadata loss degrades conservatively rather than permissively.
    ///
    /// `host-influenced` rehydrates at `untracked` rather than at itself, and deliberately: the
    /// phase marker inside a live host-influenced account is minted by the act of READING host
    /// bytes, and a word in a file is not that act. Reconstructing one from a token would be the
    /// laundering `306b` §3a forbids, so the read lands one point above instead.
    #[must_use]
    fn rehydrated(token: Option<&str>) -> Self {
        match token {
            Some("authored-before-contact") => {
                Self(dorc_core::influence::InfluenceAccount::authored_before_contact())
            }
            _ => Self(dorc_core::influence::InfluenceAccount::untracked()),
        }
    }
}

/// Inspect one exact durable and name why it cannot be replayed.
///
/// One grammar, one reader (`rul-strawman-formats-no-compat`): the durable this reads is the same
/// v2 the engine writes and [`admit_unscoped_whylog`] admits, so a transcript case and a real run
/// answer from identical bytes. The permissive v1 grammar this used to carry is GONE — it had no
/// production caller in either direction and had already diverged from the product's shape, which is
/// exactly the rot `rul-fixture-identity-never-production` describes.
///
/// # Bounded-input fence (`rul-host-bytes-bounded-before-admission`)
///
/// The bytes go through the real bounded admission, so this seat adds no unbounded read. What it
/// adds is NAMING: an admission answers with a closed refusal, and a reader who asked `dorc why`
/// deserves to be told which of the four framing conditions its durable hit.
///
/// `current_book` and `current_source` are how the caller answers "what is on disk now". The oracle
/// side is a RESOLVER rather than a list because only the durable knows which paths it recorded, and
/// a caller that had to enumerate them first would be re-parsing the envelope to feed the function
/// that parses the envelope.
#[must_use]
pub fn inspect(
    raw: Option<&str>,
    identity: &str,
    current_book: Option<&str>,
    current_source: impl Fn(&str) -> Option<String>,
) -> Vec<Diag> {
    let Some(raw) = raw else {
        return vec![Diag::new_spanless_site(DiagCode::WhylogAbsent(
            dorc_aid::diag::WhylogAbsent {
                dir: identity.to_owned(),
            },
        ))];
    };
    let envelope = match admit_unscoped_whylog(raw.as_bytes(), WhylogLimits::spike_default()) {
        Admission::Admitted(envelope) => envelope,
        Admission::NoObservation => return vec![corrupt(WhylogCorruptReason::Headerless)],
        Admission::Refused(reason) => return vec![name_refusal(raw, reason)],
    };
    // DESYNC: the durable stores digests and re-reads the sources from disk, so a changed input
    // breaks the replay tie and the recorded run is never silently re-derived against the wrong
    // source (`determinism-is-the-replay-license`).
    let desync = current_book
        .filter(|book| book_digest(book) != envelope.claims().book_digest())
        .map(|_| "book".to_owned())
        .or_else(|| {
            envelope.recorded_oracles().iter().find_map(|recorded| {
                let path = recorded.path().as_str();
                let source = current_source(path)?;
                (book_digest(&source) != recorded.digest()).then(|| format!("oracle {path}"))
            })
        });
    desync
        .map(|which| {
            Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                dorc_aid::diag::WhylogBookDesync { which },
            ))
        })
        .into_iter()
        .collect()
}

/// Name a refused durable for a reader.
///
/// The four [`WhylogCorruptReason`]s are framing conditions a reader can be told about in its own
/// vocabulary, so they are recognised structurally, here, from the exact bytes. Everything else
/// keeps the ADMISSION's own named refusal rather than being flattened into a generic "corrupt":
/// the intake already says precisely what it observed, and re-labelling that would lose attribution
/// (`271:rul-sin-ordering` — mis-attributed beats unattributed only in the wrong direction).
fn name_refusal(raw: &str, reason: AdmissionRefusal) -> Diag {
    let Some(first) = raw.lines().next() else {
        return corrupt(WhylogCorruptReason::Headerless);
    };
    let Some(tag) = first.split_whitespace().next() else {
        return corrupt(WhylogCorruptReason::Headerless);
    };
    if !tag.starts_with("dorc-whylog/") {
        return corrupt(WhylogCorruptReason::HeaderTagMissing);
    }
    if tag != WHYLOG_V2_TAG {
        return Diag::new_spanless_site(DiagCode::WhylogVersionRefused(WhylogVersionRefused {
            found: tag.to_owned(),
        }));
    }
    if results_block_overruns(raw) {
        return corrupt(WhylogCorruptReason::ResultsBlockOverruns);
    }
    if !raw.lines().any(|line| line.starts_with(WHYLOG_V2_END)) {
        return corrupt(WhylogCorruptReason::EndSentinelMissing);
    }
    reason.spanless_diagnostic()
}

/// Does a `results bytes=N` line declare more bytes than the file holds after it?
fn results_block_overruns(raw: &str) -> bool {
    let Some(at) = raw.find("results bytes=") else {
        return false;
    };
    let tail = &raw[at + "results bytes=".len()..];
    let Some(declared) = tail
        .split_whitespace()
        .next()
        .and_then(|n| n.parse::<usize>().ok())
    else {
        return false;
    };
    let after_line = tail
        .find('\n')
        .map_or(raw.len(), |nl| at + "results bytes=".len() + nl + 1);
    declared > raw.len().saturating_sub(after_line)
}

/// One [`DiagCode::WhylogCorrupt`], so every corrupt exit carries the code the completeness gate
/// demands and its typed reason.
fn corrupt(reason: WhylogCorruptReason) -> Diag {
    Diag::new_spanless_site(DiagCode::WhylogCorrupt(WhylogCorrupt { reason }))
}

/// Explicit controller-edge ceilings for the decision-inert v2 durable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhylogLimits {
    outer_bytes: usize,
    outer_line_bytes: usize,
    outer_field_bytes: usize,
    outer_retained_bytes: usize,
    numeric_digits: usize,
    argv_entries: usize,
    oracle_entries: usize,
    apply_entries: usize,
    digest_hex_min: usize,
    digest_hex_max: usize,
}

impl WhylogLimits {
    /// The injectable spike policy mirrors ingress while reserving an independent inner budget.
    #[must_use]
    pub const fn spike_default() -> Self {
        Self {
            outer_bytes: 16 * 1024 * 1024,
            outer_line_bytes: 64 * 1024,
            outer_field_bytes: 16 * 1024,
            outer_retained_bytes: 4 * 1024 * 1024,
            numeric_digits: 16,
            argv_entries: 32_768,
            oracle_entries: 32_768,
            apply_entries: 32_768,
            digest_hex_min: 16,
            digest_hex_max: 128,
        }
    }

    /// Constructs an explicit test or embedding policy without public mutable fields.
    #[expect(
        clippy::too_many_arguments,
        reason = "each frozen ingress ceiling remains independently injectable"
    )]
    #[must_use]
    pub const fn new(
        outer_bytes: usize,
        outer_line_bytes: usize,
        outer_field_bytes: usize,
        outer_retained_bytes: usize,
        numeric_digits: usize,
        argv_entries: usize,
        oracle_entries: usize,
        apply_entries: usize,
        digest_hex_min: usize,
        digest_hex_max: usize,
    ) -> Self {
        Self {
            outer_bytes,
            outer_line_bytes,
            outer_field_bytes,
            outer_retained_bytes,
            numeric_digits,
            argv_entries,
            oracle_entries,
            apply_entries,
            digest_hex_min,
            digest_hex_max,
        }
    }
}

/// A recorded, untrusted source path a durable claims its run read.
///
/// # What it is and is not (`28F:rul-path-hint-must-match-its-doc`)
///
/// It is a CLAIM, never an authority. It mints no scope, no framing, and no trust: nothing derived
/// from it may be believed until [`RecordedReplayClaims::book_digest`] (or the per-oracle
/// [`RecordedOracleSource::digest`]) has been compared against a digest of what was actually read.
///
/// A replay edge does open the named path — that is what reconstructing a run's inputs MEANS — and
/// the honest statement is therefore narrower than "never a source-loading capability", which this
/// type used to claim while its one caller loaded sources with it. The edge owes a BOUNDED,
/// regular-file-only read whose result is a candidate until the digest matches (`dorc-cli`'s
/// `read_replay_source`); the type owes callers no more authority than that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedSourcePathHint(String);

impl RecordedSourcePathHint {
    /// The recorded untrusted claim. Comparing it is always sound; acting on it demands the
    /// bounded-read-then-digest-check discipline the type doc names.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One ordered recorded oracle identity claim, on [`RecordedSourcePathHint`]'s terms: a claim to be
/// digest-checked, never an authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedOracleSource {
    ordinal: usize,
    digest: String,
    path: RecordedSourcePathHint,
}

impl RecordedOracleSource {
    /// The durable ordering claim, retained for later exact source-set comparison.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// The recorded untrusted content digest claim.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }

    /// The recorded untrusted source path hint.
    #[must_use]
    pub fn path(&self) -> &RecordedSourcePathHint {
        &self.path
    }
}

/// Wire claims retained only for later controller comparison; none constructs a scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedReplayClaims {
    nonce: String,
    attempt: u32,
    host: String,
    target: String,
    generation: String,
    book_digest: String,
    decision_digest: String,
    started_at: Option<dorc_core::RunInstant>,
}

impl RecordedReplayClaims {
    /// Wire values are untrusted replay claims, never controller identity constructors.
    #[must_use]
    pub fn nonce(&self) -> &str {
        &self.nonce
    }
    #[must_use]
    pub const fn attempt(&self) -> u32 {
        self.attempt
    }
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }
    #[must_use]
    pub fn generation(&self) -> &str {
        &self.generation
    }
    #[must_use]
    pub fn book_digest(&self) -> &str {
        &self.book_digest
    }
    #[must_use]
    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    /// WHEN the recorded run started — the receipt's date line, and the one whylog field that
    /// genuinely cannot be recomputed by replaying the durable through the kernel (the thin-durable
    /// test). `None` when the writing controller had no clock; absence renders as absence.
    #[must_use]
    pub const fn started_at(&self) -> Option<dorc_core::RunInstant> {
        self.started_at
    }
}

/// A grammar-valid v2 durable before a controller compares it with its expected framing.
#[derive(Debug)]
pub struct UnscopedWhylogEnvelope {
    backing: BoundedHostBytes,
    inner_range: std::ops::Range<usize>,
    claims: RecordedReplayClaims,
    mode: String,
    book_path: RecordedSourcePathHint,
    oracle_sources: Vec<RecordedOracleSource>,
    argv: Vec<String>,
    apply: Vec<ApplyLine>,
    /// The controller instants the durable recorded, by record ordinal, ascending.
    instants: Vec<(u64, dorc_core::RunInstant)>,
}

/// Unscoped records accepted through a matching controller framing. Checkpoint 3C owns scope minting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedUnscopedWhylogReplay {
    claims: RecordedReplayClaims,
    mode: String,
    book_path: RecordedSourcePathHint,
    oracle_sources: Vec<RecordedOracleSource>,
    argv: Vec<String>,
    apply: Vec<ApplyLine>,
    records: AdmittedUnscopedHostRecords,
}

/// A whole-document v2 writer refusal. It never yields partial durable text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhylogWriteRefusal {
    Limit,
    Grammar,
    Numeric,
    Digest,
    ArithmeticOverflow,
}

/// Controller-produced metadata for one v2 durable. The wire writer borrows this separately from
/// admitted host evidence so untrusted result bytes have no raw serialization route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhylogV2Metadata {
    /// Controller-selected invocation mode.
    pub mode: String,
    /// Ordered invocation arguments.
    pub argv: Vec<String>,
    /// Controller-selected book path and content digest.
    pub book: (String, String),
    /// Ordered oracle path and digest claims.
    pub oracles: Vec<(String, String)>,
    /// Controller-minted run nonce.
    pub nonce: String,
    /// Controller-minted retry attempt.
    pub attempt: u32,
    /// Controller-selected host identity.
    pub host: String,
    /// Controller-computed decision digest.
    pub decision_digest: String,
    /// When the controller started this run, from the edge's injected clock. Stored because it is
    /// the one invocation fact replay CANNOT recompute — re-deriving through the kernel yields the
    /// plan again, never the moment it was first made. `None` ⇒ the edge had no clock, and the
    /// durable says so rather than dating the run from replay.
    pub started_at: Option<dorc_core::RunInstant>,
    /// When the controller took each probe record in, by that record's arrival ordinal, ascending.
    ///
    /// Stored for the same reason `started_at` is: replay re-derives the DECISION through the same
    /// kernel, but it cannot re-derive WHEN the original run heard anything, and a replay clock
    /// reading here would present the moment of reading as the moment of measurement. Without
    /// these the receipt view loses its per-row instants on every replay — the run's own records
    /// stop being able to say when they arrived.
    ///
    /// Controller-minted, like every other instant Dorc holds
    /// (`28F:rul-probe-instants-host-says-no-times`, human-typed). Records with no instant are
    /// simply absent from the list rather than carrying a fabricated one.
    pub instants: Vec<(u64, dorc_core::RunInstant)>,
    /// Ordered predicted apply dispositions.
    pub apply: Vec<ApplyLine>,
}

/// The per-species DURABLE VIEWS, and the projection that builds them from the Spine
/// (`309:mech-census-three-states`; census `30E` §2).
///
/// # Why the fields live here and not on the records
///
/// A Spine record is `SiteId`-keyed, license-bearing, and grade-stamped. A View's fields ARE the
/// durable subset and nothing else. Because records themselves never implement serialization, a
/// field that no View names **cannot reach disk** — field-level exclusion is structural rather than
/// remembered, silent field-growth is unrepresentable, and lifting one exclusion is one field added
/// to one View, which is the durable tripwire's mechanical form
/// (`rul-durable-contents-reviewed-before-design`).
///
/// The exemplar is the disposition: the RECORD carries the site's fine key and its license, and
/// [`ApplyLine`] — the View — carries a leaf and a tag. `30E:stop-siteid-digest-rekey` keeps it that
/// way this stage; re-keying the durable to `SiteId` is `lift-durable-siteid-keying`, deferred
/// behind the tripwire.
pub mod view {
    use dorc_core::RunInstant;
    use dorc_core::SourceRole;
    use dorc_core::spine::{SpineInvocation, SpinePresentedPlan};

    use super::ApplyLine;
    use crate::Disposition;

    /// The `SpineInvocation` View: everything the durable keeps of the run's identity.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Invocation {
        /// What mode a reader should REPLAY under — never the producing invocation's mode.
        pub mode: dorc_core::spine::InvocationMode,
        /// The full argv, one word per element.
        pub argv: Vec<String>,
        /// Book path and content digest.
        pub book: (String, String),
        /// Oracle paths and digests, in load order.
        pub oracles: Vec<(String, String)>,
        /// The per-attempt nonce.
        pub nonce: String,
        /// The attempt serial.
        pub attempt: u32,
        /// The session host id.
        pub host: String,
        /// When the controller started the run. `None` ⇒ the edge had no clock, and the durable
        /// says so rather than dating the run from replay.
        pub started_at: Option<RunInstant>,
    }

    impl Invocation {
        /// Project the record. The grade is dropped by NOT BEING NAMED — `306c` §2's scope fence in
        /// its structural form.
        ///
        /// `None` when the source vector names no book. The vector is role-carrying and ordered by
        /// LOAD position, so the book is found by its role rather than by sitting last; a run that
        /// recorded no book row has no durable to write, which is the projection's own contract.
        #[must_use]
        pub fn of(record: &SpineInvocation) -> Option<Self> {
            let identity = record.identity();
            let book = record.sources_in_role(SourceRole::Book).next()?;
            Some(Self {
                mode: record.mode(),
                argv: record.argv().to_vec(),
                book: (book.path.clone(), book.digest.clone()),
                oracles: record
                    .sources()
                    .iter()
                    .filter(|claim| claim.role != SourceRole::Book)
                    .map(|claim| (claim.path.clone(), claim.digest.clone()))
                    .collect(),
                nonce: identity.nonce.clone(),
                attempt: identity.attempt,
                host: identity.host.clone(),
                started_at: identity.started_at,
            })
        }
    }

    /// The `SpinePresentedPlan` View: the digest string, and nothing beside it.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Digest {
        /// The decision digest at write time.
        pub digest: String,
    }

    impl Digest {
        /// Project the record.
        #[must_use]
        pub fn of(record: &SpinePresentedPlan<crate::PlanPlane>) -> Self {
            Self {
                digest: record.identity().hex(),
            }
        }
    }

    /// The `SpineRecordStream` View: the arrival instants. The admitted BYTES are not a field here
    /// — they ride to the writer as the borrowed admitted handle, so untrusted result bytes keep
    /// having no raw serialization route (`rul-host-bytes-bounded-before-admission`).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RecordStream {
        /// When the controller took each record in, by arrival ordinal, ascending.
        pub instants: Vec<(u64, RunInstant)>,
    }

    /// Project a `SpineDisposition` to its durable row.
    ///
    /// THE lossy step, and the one worth reading twice: the fine `SiteId` narrows to its leaf and
    /// the whole license becomes a four-letter tag. `predicted` is always `true` — the spike has no
    /// apply executor, so every row is a PREDICTION and must never wear a measurement's clothes
    /// (`tc-apply-report-is-prediction`).
    #[must_use]
    pub fn disposition(
        site: dorc_core::SiteId,
        decision: &Disposition,
        account: dorc_core::influence::InfluenceAccount,
    ) -> ApplyLine {
        ApplyLine {
            leaf: site.leaf.0,
            disposition: tag(decision).to_owned(),
            predicted: true,
            account: super::DurableAccount::of_decision(account),
        }
    }

    /// The durable's disposition vocabulary.
    #[must_use]
    pub const fn tag(disposition: &Disposition) -> &'static str {
        match disposition {
            Disposition::Run => "run",
            Disposition::Replace(_, _) => "replace",
            Disposition::Omit { .. } => "omit",
            Disposition::Guard(_) => "guard",
        }
    }
}

/// The whole `.whylog` projection: exactly the four `CensusArm::Durable` species, each through its
/// own View, plus the account of everything it chose not to keep.
///
/// This is the ONLY route from decisions to disk. It is built from a Spine and nothing else, so a
/// driver cannot assemble a durable out of scattered locals — which is what made silent divergence
/// between the digest, the report, and the artifact possible before the reification.
#[derive(Debug)]
pub struct DurableProjection<'a> {
    metadata: WhylogV2Metadata,
    records: &'a AdmittedUnscopedHostRecords,
    drops: Vec<dorc_aid::CollapseNarrative>,
}

impl<'a> DurableProjection<'a> {
    /// Project the durable from the Spine.
    ///
    /// `None` when a durable-arm record the projection needs is absent — an invocation, a digest, or
    /// an admitted record stream. That is an honest "there is no durable to write", never a partial
    /// one: a whole-document refusal is the writer's contract, and it starts here.
    #[must_use]
    pub fn project(spine: &'a crate::Spine) -> Option<Self> {
        let invocation = view::Invocation::of(spine.invocation()?)?;
        let digest = view::Digest::of(spine.presented_plan()?);
        let stream = spine.record_stream()?;
        let apply = spine
            .dispositions()
            .map(|record| {
                use dorc_core::spine::InfluenceBearing as _;
                view::disposition(record.site(), record.decision(), record.account())
            })
            .collect();
        Some(Self {
            metadata: WhylogV2Metadata {
                mode: invocation.mode.token().to_owned(),
                argv: invocation.argv,
                book: invocation.book,
                oracles: invocation.oracles,
                nonce: invocation.nonce,
                attempt: invocation.attempt,
                host: invocation.host,
                decision_digest: digest.digest,
                started_at: invocation.started_at,
                instants: stream.instants().to_vec(),
                apply,
            },
            records: stream.records(),
            drops: drop_account(spine),
        })
    }

    /// The controller metadata this projection carries.
    #[must_use]
    pub const fn metadata(&self) -> &WhylogV2Metadata {
        &self.metadata
    }

    /// The admitted record bytes this projection carries, still wearing their admission.
    #[must_use]
    pub const fn records(&self) -> &'a AdmittedUnscopedHostRecords {
        self.records
    }

    /// What the projection dropped, narrated
    /// (`309:rul-drop-accounting-completes-the-narrative-law`).
    ///
    /// "The durable is not permitted to be poor; it may be forced to be poor" (`306b` §2a) becomes
    /// mechanical here: every non-durable species the Spine actually held is countable at projection
    /// time, so the run can say what it chose not to keep instead of the loss being invisible.
    #[must_use]
    pub fn drops(&self) -> &[dorc_aid::CollapseNarrative] {
        &self.drops
    }
}

/// One `ProjectionDrop` per non-durable species the Spine actually held.
///
/// Walks `SpineSpecies::ALL`, so a new species is accounted for the moment the census classifies it
/// — there is no second list to keep in step.
fn drop_account(spine: &crate::Spine) -> Vec<dorc_aid::CollapseNarrative> {
    use dorc_core::spine::{CensusArm, SpineSpecies};

    SpineSpecies::ALL
        .iter()
        .filter(|species| species.census_arm() != CensusArm::Durable)
        .filter_map(|species| {
            let dropped = spine.population(*species);
            (dropped > 0).then(|| {
                dorc_aid::CollapseNarrative::new(
                    dorc_aid::narrative::SpeechAct::Derived,
                    dorc_aid::narrative::CollapseKind::ProjectionDrop {
                        projection: "whylog",
                        species: species.name(),
                        dropped,
                    },
                )
            })
        })
        .collect()
}

/// The only v2 serialization input: controller metadata paired with already-admitted records.
#[derive(Debug)]
pub struct WhylogV2Write<'a> {
    metadata: &'a WhylogV2Metadata,
    records: &'a AdmittedUnscopedHostRecords,
}

impl<'a> WhylogV2Write<'a> {
    /// Couples controller-produced metadata to the exact framed bytes admission retained.
    #[must_use]
    pub fn new(metadata: &'a WhylogV2Metadata, records: &'a AdmittedUnscopedHostRecords) -> Self {
        Self { metadata, records }
    }

    /// The write a durable projection describes — the driver's only route to these bytes.
    #[must_use]
    pub const fn of_projection(projection: &'a DurableProjection<'a>) -> Self {
        Self {
            metadata: &projection.metadata,
            records: projection.records,
        }
    }
}

impl UnscopedWhylogEnvelope {
    /// Retains exact ordered wire identity claims without minting source or controller authority.
    #[must_use]
    pub fn recorded_oracles(&self) -> &[RecordedOracleSource] {
        &self.oracle_sources
    }

    #[must_use]
    pub fn claims(&self) -> &RecordedReplayClaims {
        &self.claims
    }
    #[must_use]
    pub fn mode(&self) -> &str {
        &self.mode
    }

    /// The record-stream version this durable declared.
    ///
    /// One value today by construction: [`parse_v2`] admits exactly the [`WHYLOG_V2_TAG`] header
    /// and refuses every other version outright, so anything that parsed is a `2`. The accessor
    /// exists anyway because it is the seam a multi-version reader answers differently from — and
    /// because a consumer keying on the version (the `[unnarrated:]` census) should ASK the
    /// durable rather than assume the number its own binary was built with.
    #[must_use]
    pub const fn record_stream_version(&self) -> u32 {
        RECORD_STREAM_VERSION
    }

    /// When the ORIGINAL run took each probe record in, by that record's arrival ordinal.
    #[must_use]
    pub fn recorded_instants(&self) -> &[(u64, dorc_core::RunInstant)] {
        &self.instants
    }
    #[must_use]
    pub fn recorded_book_path(&self) -> &RecordedSourcePathHint {
        &self.book_path
    }
    #[must_use]
    pub fn argv(&self) -> &[String] {
        &self.argv
    }
    #[must_use]
    pub fn apply(&self) -> &[ApplyLine] {
        &self.apply
    }
}

impl AdmittedUnscopedWhylogReplay {
    /// Preserves exact ordered wire identity claims for checkpoint 3C's source comparison.
    #[must_use]
    pub fn recorded_oracles(&self) -> &[RecordedOracleSource] {
        &self.oracle_sources
    }

    #[must_use]
    pub fn claims(&self) -> &RecordedReplayClaims {
        &self.claims
    }
    #[must_use]
    pub fn mode(&self) -> &str {
        &self.mode
    }
    #[must_use]
    pub fn recorded_book_path(&self) -> &RecordedSourcePathHint {
        &self.book_path
    }
    #[must_use]
    pub fn argv(&self) -> &[String] {
        &self.argv
    }
    #[must_use]
    pub fn apply(&self) -> &[ApplyLine] {
        &self.apply
    }
    #[must_use]
    pub fn records(&self) -> &AdmittedUnscopedHostRecords {
        &self.records
    }
}

/// Reads a bounded v2 durable into untrusted metadata plus one borrowed inner-record range.
pub fn admit_unscoped_whylog<R: Read>(
    reader: R,
    limits: WhylogLimits,
) -> Admission<UnscopedWhylogEnvelope> {
    let Some(read_limit) = limits.outer_bytes.checked_add(1) else {
        return Admission::Refused(AdmissionRefusal::ArithmeticOverflow);
    };
    let mut backing = Vec::new();
    if reader
        .take(read_limit as u64)
        .read_to_end(&mut backing)
        .is_err()
    {
        return Admission::Refused(AdmissionRefusal::Framing);
    }
    if backing.len() > limits.outer_bytes {
        return Admission::Refused(AdmissionRefusal::StreamLimit);
    }
    parse_v2(backing, limits)
}

/// Compares only durable wire claims with controller framing, then applies the independent records budget.
#[must_use]
pub fn admit_unscoped_whylog_replay(
    envelope: UnscopedWhylogEnvelope,
    expected: &Framing,
    inner_limits: HostEvidenceLimits,
) -> Admission<AdmittedUnscopedWhylogReplay> {
    if envelope.claims.nonce != expected.nonce().0
        || envelope.claims.attempt != expected.attempt()
        || envelope.claims.host != expected.host()
        || envelope.claims.book_digest != expected.book_digest()
    {
        return Admission::Refused(AdmissionRefusal::Framing);
    }
    if envelope.inner_range.len() > inner_limits.stream_bytes() {
        return Admission::Refused(AdmissionRefusal::StreamLimit);
    }
    let Some(inner) = envelope.backing.with_admitted_range(envelope.inner_range) else {
        return Admission::Refused(AdmissionRefusal::ArithmeticOverflow);
    };
    match admit_unscoped_host_records(&inner, expected, inner_limits) {
        // The replay's influence phase is re-derived at the cli edge rather than threaded through
        // this type: a durable carries no grade (persisting one is `306b` §3a/§3b, deliberately
        // unbuilt), and re-deriving it there lands on the same conservative answer.
        Admission::Admitted(records) => Admission::Admitted(AdmittedUnscopedWhylogReplay {
            claims: envelope.claims,
            mode: envelope.mode,
            book_path: envelope.book_path,
            oracle_sources: envelope.oracle_sources,
            argv: envelope.argv,
            apply: envelope.apply,
            records: records.into_read().0,
        }),
        Admission::NoObservation => Admission::NoObservation,
        Admission::Refused(reason) => Admission::Refused(reason),
    }
}

/// Writes a complete v2 durable or refuses before returning any bytes. V1 remains the temporary CLI path.
///
/// # Errors
///
/// Returns a closed refusal when a frozen grammar or resource ceiling is not met.
pub fn try_serialize_v2(
    write: &WhylogV2Write<'_>,
    limits: WhylogLimits,
) -> Result<Vec<u8>, WhylogWriteRefusal> {
    let doc = write.metadata;
    let results = write.records.admitted_wire_bytes();
    if !mode_valid(&doc.mode) || !atom_valid(&doc.nonce, limits) || !atom_valid(&doc.host, limits) {
        return Err(WhylogWriteRefusal::Grammar);
    }
    if digits_valid(doc.attempt.to_string().as_str(), limits).is_err() {
        return Err(WhylogWriteRefusal::Numeric);
    }
    if !digest_valid(&doc.book.1, limits) || !digest_valid(&doc.decision_digest, limits) {
        return Err(WhylogWriteRefusal::Digest);
    }
    if !free_valid(&doc.book.0, limits) || doc.argv.len() > limits.argv_entries {
        return Err(WhylogWriteRefusal::Limit);
    }
    if doc.oracles.len() > limits.oracle_entries || doc.apply.len() > limits.apply_entries {
        return Err(WhylogWriteRefusal::Limit);
    }
    valid_apply_rows(&doc.apply, limits)?;

    let mut out = Vec::new();
    let mut retained = 0usize;
    retain_metadata(&mut retained, &doc.nonce, limits)?;
    retain_metadata(&mut retained, &doc.host, limits)?;
    retain_metadata(&mut retained, &doc.mode, limits)?;
    retain_metadata(&mut retained, &doc.book.0, limits)?;
    retain_metadata(&mut retained, &doc.book.1, limits)?;
    // The READER.s predicate, so the writer cannot emit a header its parser refuses. No retained
    // budget: the reader keeps a `u64`, not the String every atom above costs.
    let started = render_started(doc.started_at);
    if parse_started(&started, limits).is_err() {
        return Err(WhylogWriteRefusal::Numeric);
    }
    write_v2_line(
        &mut out,
        format!(
            "{WHYLOG_V2_TAG} nonce={} attempt={} host={} target=width-one generation=width-one mode={} started={started} {TERMINAL_TOKEN}",
            doc.nonce, doc.attempt, doc.host, doc.mode
        ),
        limits,
    )?;
    write_v2_line(
        &mut out,
        format!(
            "book digest={} path={} {TERMINAL_TOKEN}",
            doc.book.1, doc.book.0
        ),
        limits,
    )?;
    for (ordinal, (path, digest)) in doc.oracles.iter().enumerate() {
        if !digest_valid(digest, limits) || !free_valid(path, limits) {
            return Err(WhylogWriteRefusal::Grammar);
        }
        retain_metadata(&mut retained, digest, limits)?;
        retain_metadata(&mut retained, path, limits)?;
        write_v2_line(
            &mut out,
            format!("oracle ordinal={ordinal} digest={digest} path={path} {TERMINAL_TOKEN}"),
            limits,
        )?;
    }
    for argument in &doc.argv {
        if !free_valid(argument, limits) {
            return Err(WhylogWriteRefusal::Grammar);
        }
        retain_metadata(&mut retained, argument, limits)?;
        write_v2_line(
            &mut out,
            format!("argv value={argument} {TERMINAL_TOKEN}"),
            limits,
        )?;
    }
    write_v2_line(
        &mut out,
        format!("digest decision={} {TERMINAL_TOKEN}", doc.decision_digest),
        limits,
    )?;
    retain_metadata(&mut retained, &doc.decision_digest, limits)?;
    write_instants(&mut out, &doc.instants, limits)?;
    write_apply_rows(&mut out, &mut retained, &doc.apply, limits)?;
    write_v2_line(
        &mut out,
        format!("results bytes={} {TERMINAL_TOKEN}", results.len()),
        limits,
    )?;
    checked_append(&mut out, results, limits)?;
    checked_append(
        &mut out,
        format!("{WHYLOG_V2_END} {TERMINAL_TOKEN}\n").as_bytes(),
        limits,
    )?;
    Ok(out)
}

#[expect(
    clippy::too_many_lines,
    reason = "the ordered outer framing state machine keeps every refusal before ownership"
)]
fn parse_v2(backing: Vec<u8>, limits: WhylogLimits) -> Admission<UnscopedWhylogEnvelope> {
    let mut cursor = 0usize;
    let header = match v2_line(&backing, &mut cursor, limits) {
        Ok(line) => line,
        Err(reason) => return Admission::Refused(reason),
    };
    if header.starts_with("dorc-whylog/1") {
        return Admission::Refused(AdmissionRefusal::IncompatibleVersion);
    }
    let Some(header) = token_body(header) else {
        return Admission::Refused(AdmissionRefusal::Framing);
    };
    let Some(header) = header.strip_prefix(&format!("{WHYLOG_V2_TAG} ")) else {
        return Admission::Refused(AdmissionRefusal::Grammar);
    };
    let Some((nonce, attempt, host, target, generation, mode, started_at)) =
        parse_v2_header(header, limits)
    else {
        return Admission::Refused(AdmissionRefusal::Grammar);
    };
    let Some(book) = v2_line(&backing, &mut cursor, limits)
        .ok()
        .and_then(token_body)
        .and_then(|line| parse_book(line, limits))
    else {
        return Admission::Refused(AdmissionRefusal::Grammar);
    };
    let mut oracle_sources = Vec::new();
    let mut argv = Vec::new();
    let mut apply = Vec::new();
    let mut instants: Vec<(u64, dorc_core::RunInstant)> = Vec::new();
    let mut last_instant_ordinal: Option<u64> = None;
    let mut argv_started = false;
    let mut retained = 0usize;
    for value in [nonce, host, target, generation, mode, book.0, book.1] {
        if retain(&mut retained, value.len(), limits).is_err() {
            return Admission::Refused(AdmissionRefusal::RetainedLimit);
        }
    }
    let mut expected_ordinal = 0usize;
    let mut last_apply_leaf = None;
    loop {
        let line = match v2_line(&backing, &mut cursor, limits) {
            Ok(line) => line,
            Err(reason) => return Admission::Refused(reason),
        };
        let Some(line) = token_body(line) else {
            return Admission::Refused(AdmissionRefusal::Framing);
        };
        if line.starts_with("oracle ") {
            if argv_started {
                return Admission::Refused(AdmissionRefusal::Grammar);
            }
            let Some((ordinal, digest, path)) = parse_oracle(line, limits) else {
                return Admission::Refused(AdmissionRefusal::Grammar);
            };
            if ordinal != expected_ordinal || oracle_sources.len() >= limits.oracle_entries {
                return Admission::Refused(AdmissionRefusal::Grammar);
            }
            expected_ordinal = match expected_ordinal.checked_add(1) {
                Some(value) => value,
                None => return Admission::Refused(AdmissionRefusal::ArithmeticOverflow),
            };
            if retain(&mut retained, digest.len(), limits).is_err()
                || retain(&mut retained, path.len(), limits).is_err()
            {
                return Admission::Refused(AdmissionRefusal::RetainedLimit);
            }
            oracle_sources.push(RecordedOracleSource {
                ordinal,
                digest: digest.to_owned(),
                path: RecordedSourcePathHint(path.to_owned()),
            });
            continue;
        }
        if let Some(value) = line.strip_prefix("argv value=") {
            argv_started = true;
            if argv.len() >= limits.argv_entries || !free_valid(value, limits) {
                return Admission::Refused(AdmissionRefusal::Grammar);
            }
            if retain(&mut retained, value.len(), limits).is_err() {
                return Admission::Refused(AdmissionRefusal::RetainedLimit);
            }
            argv.push(value.to_owned());
            continue;
        }
        let Some(decision_digest) = line.strip_prefix("digest decision=") else {
            return Admission::Refused(AdmissionRefusal::Grammar);
        };
        if !digest_valid(decision_digest, limits) {
            return Admission::Refused(AdmissionRefusal::Grammar);
        }
        if retain(&mut retained, decision_digest.len(), limits).is_err() {
            return Admission::Refused(AdmissionRefusal::RetainedLimit);
        }
        let decision_digest = decision_digest.to_owned();
        let results_range = loop {
            let line = match v2_line(&backing, &mut cursor, limits) {
                Ok(line) => line,
                Err(reason) => return Admission::Refused(reason),
            };
            let Some(line) = token_body(line) else {
                return Admission::Refused(AdmissionRefusal::Framing);
            };
            if line.starts_with("instant ") {
                // An ordinal is a KEY: a duplicate silently redates one record from another's.
                let Some((ordinal, at)) = parse_instant(line, limits) else {
                    return Admission::Refused(AdmissionRefusal::Grammar);
                };
                if instants.len() >= limits.apply_entries {
                    return Admission::Refused(AdmissionRefusal::CollectionLimit);
                }
                if last_instant_ordinal.is_some_and(|previous| ordinal <= previous) {
                    return Admission::Refused(AdmissionRefusal::Grammar);
                }
                last_instant_ordinal = Some(ordinal);
                instants.push((ordinal, at));
                continue;
            }
            if line.starts_with("apply ") {
                let Some((leaf, disposition, predicted, account)) = parse_v2_apply(line, limits)
                else {
                    return Admission::Refused(AdmissionRefusal::Grammar);
                };
                if apply.len() >= limits.apply_entries
                    || last_apply_leaf.is_some_and(|previous| leaf <= previous)
                {
                    return Admission::Refused(AdmissionRefusal::CollectionLimit);
                }
                if retain(&mut retained, disposition.len(), limits).is_err() {
                    return Admission::Refused(AdmissionRefusal::RetainedLimit);
                }
                apply.push(ApplyLine {
                    leaf,
                    disposition: disposition.to_owned(),
                    predicted,
                    account,
                });
                last_apply_leaf = Some(leaf);
                continue;
            }
            let Some(size) = line.strip_prefix("results bytes=") else {
                return Admission::Refused(AdmissionRefusal::Grammar);
            };
            let Ok(size) = bounded_number(size, limits) else {
                return Admission::Refused(AdmissionRefusal::Numeric);
            };
            let start = cursor;
            let Some(end) = start.checked_add(size) else {
                return Admission::Refused(AdmissionRefusal::ArithmeticOverflow);
            };
            if end > backing.len() {
                return Admission::Refused(AdmissionRefusal::Framing);
            }
            cursor = end;
            break start..end;
        };
        let end = match v2_line(&backing, &mut cursor, limits) {
            Ok(line) => line,
            Err(reason) => return Admission::Refused(reason),
        };
        if token_body(end) != Some(WHYLOG_V2_END) || cursor != backing.len() {
            return Admission::Refused(AdmissionRefusal::Framing);
        }
        let claims = RecordedReplayClaims {
            nonce: nonce.to_owned(),
            attempt,
            host: host.to_owned(),
            target: target.to_owned(),
            generation: generation.to_owned(),
            book_digest: book.1.to_owned(),
            decision_digest,
            started_at,
        };
        let mode = mode.to_owned();
        let book_path = RecordedSourcePathHint(book.0.to_owned());
        return Admission::Admitted(UnscopedWhylogEnvelope {
            backing: BoundedHostBytes::from_owned_backing(backing),
            inner_range: results_range,
            claims,
            mode,
            book_path,
            oracle_sources,
            argv,
            apply,
            instants,
        });
    }
}

fn v2_line<'a>(
    bytes: &'a [u8],
    cursor: &mut usize,
    limits: WhylogLimits,
) -> Result<&'a str, AdmissionRefusal> {
    let rest = bytes
        .get(*cursor..)
        .ok_or(AdmissionRefusal::ArithmeticOverflow)?;
    let newline = rest
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or(AdmissionRefusal::Framing)?;
    let end = cursor
        .checked_add(newline)
        .ok_or(AdmissionRefusal::ArithmeticOverflow)?;
    let line = bytes
        .get(*cursor..end)
        .ok_or(AdmissionRefusal::ArithmeticOverflow)?;
    *cursor = end
        .checked_add(1)
        .ok_or(AdmissionRefusal::ArithmeticOverflow)?;
    if line.len() > limits.outer_line_bytes {
        return Err(AdmissionRefusal::LineLimit);
    }
    if line.iter().any(|byte| *byte < 0x20 || *byte == 0x7f) {
        return Err(AdmissionRefusal::ControlByte);
    }
    std::str::from_utf8(line).map_err(|_| AdmissionRefusal::InvalidUtf8)
}

fn token_body(line: &str) -> Option<&str> {
    line.strip_suffix(TERMINAL_TOKEN)?.strip_suffix(' ')
}

/// The `started=` header atom: Unix milliseconds, or `ABSENT_ATOM` when the writer had no clock.
/// A closed two-shape grammar, so "we do not know when" can never be confused with an instant.
const ABSENT_ATOM: &str = "-";

fn render_started(at: Option<dorc_core::RunInstant>) -> String {
    at.map_or_else(|| ABSENT_ATOM.to_owned(), |instant| instant.0.to_string())
}

fn parse_started(
    value: &str,
    limits: WhylogLimits,
) -> Result<Option<dorc_core::RunInstant>, AdmissionRefusal> {
    if value == ABSENT_ATOM {
        return Ok(None);
    }
    digits_valid(value, limits)?;
    value
        .parse()
        .map(|millis| Some(dorc_core::RunInstant(millis)))
        .map_err(|_| AdmissionRefusal::Numeric)
}

type V2Header<'a> = (
    &'a str,
    u32,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    Option<dorc_core::RunInstant>,
);

fn parse_v2_header(line: &str, limits: WhylogLimits) -> Option<V2Header<'_>> {
    let mut fields = line.split(' ');
    let nonce = fields.next()?.strip_prefix("nonce=")?;
    let attempt = fields.next()?.strip_prefix("attempt=")?;
    let host = fields.next()?.strip_prefix("host=")?;
    let target = fields.next()?.strip_prefix("target=")?;
    let generation = fields.next()?.strip_prefix("generation=")?;
    let mode = fields.next()?.strip_prefix("mode=")?;
    let started = parse_started(fields.next()?.strip_prefix("started=")?, limits).ok()?;
    if fields.next().is_some()
        || !atom_valid(nonce, limits)
        || !atom_valid(host, limits)
        || target != "width-one"
        || generation != "width-one"
        || !mode_valid(mode)
    {
        return None;
    }
    Some((
        nonce,
        bounded_u32(attempt, limits).ok()?,
        host,
        target,
        generation,
        mode,
        started,
    ))
}

fn parse_book(line: &str, limits: WhylogLimits) -> Option<(&str, &str)> {
    let body = line.strip_prefix("book ")?;
    let (digest, path) = body.split_once(" path=")?;
    let digest = digest.strip_prefix("digest=")?;
    (digest_valid(digest, limits) && free_valid(path, limits)).then_some((path, digest))
}

fn parse_oracle(line: &str, limits: WhylogLimits) -> Option<(usize, &str, &str)> {
    let body = line.strip_prefix("oracle ordinal=")?;
    let (ordinal, rest) = body.split_once(" digest=")?;
    let (digest, path) = rest.split_once(" path=")?;
    let ordinal = bounded_number(ordinal, limits).ok()?;
    (digest_valid(digest, limits) && free_valid(path, limits)).then_some((ordinal, digest, path))
}

fn parse_v2_apply(line: &str, limits: WhylogLimits) -> Option<(u32, &str, bool, DurableAccount)> {
    let body = line.strip_prefix("apply leaf=")?;
    let (leaf, rest) = body.split_once(" disposition=")?;
    let (disposition, tail) = rest.split_once(" predicted=")?;
    // OPTIONAL on the read side whatever the writer's switch says — a durable written with the
    // export off is ordinary input and must not refuse; absent or unknown reads most-influenced.
    let (predicted, account) = match tail.split_once(" account=") {
        Some((predicted, account)) => (predicted, DurableAccount::rehydrated(Some(account))),
        None => (tail, DurableAccount::rehydrated(None)),
    };
    let predicted = match predicted {
        "0" => false,
        "1" => true,
        _ => return None,
    };
    Some((
        u32::try_from(bounded_number(leaf, limits).ok()?).ok()?,
        disposition_valid(disposition).then_some(disposition)?,
        predicted,
        account,
    ))
}

/// Write the predicted apply rows — and, when [`ACCOUNT_EXPORT`] is on, each row's account beside
/// its disposition.
///
/// The gate is spelled ONCE, here, on the write side. Switched off, a row's bytes are exactly what
/// they were before the account existed, which is what lets the byte-identity gate prove the export
/// is inert rather than merely believed to be.
fn write_apply_rows(
    out: &mut Vec<u8>,
    retained: &mut usize,
    apply: &[ApplyLine],
    limits: WhylogLimits,
) -> Result<(), WhylogWriteRefusal> {
    for row in apply {
        retain_metadata(retained, &row.disposition, limits)?;
        let account = if ACCOUNT_EXPORT {
            format!(" account={}", row.account.label())
        } else {
            String::new()
        };
        write_v2_line(
            out,
            format!(
                "apply leaf={} disposition={} predicted={}{account} {TERMINAL_TOKEN}",
                row.leaf,
                row.disposition,
                u8::from(row.predicted)
            ),
            limits,
        )?;
    }
    Ok(())
}

/// Write the per-record arrival instants, refusing a repeated or out-of-order ordinal before any
/// of them reach the buffer — the writer's half of the same check the reader makes.
fn write_instants(
    out: &mut Vec<u8>,
    instants: &[(u64, dorc_core::RunInstant)],
    limits: WhylogLimits,
) -> Result<(), WhylogWriteRefusal> {
    let mut last: Option<u64> = None;
    for (ordinal, at) in instants {
        if last.is_some_and(|previous| *ordinal <= previous) {
            return Err(WhylogWriteRefusal::Grammar);
        }
        last = Some(*ordinal);
        write_v2_line(
            out,
            format!("instant ordinal={ordinal} at={} {TERMINAL_TOKEN}", at.0),
            limits,
        )?;
    }
    Ok(())
}

/// `instant ordinal=<n> at=<unix-millis>` — one probe record's controller-minted arrival moment.
///
/// Both fields go through the ordinary digit bounds before any parse; the instant is read with the
/// SAME predicate the header's `started=` uses, so one durable cannot carry two notions of what a
/// well-formed moment is.
fn parse_instant(line: &str, limits: WhylogLimits) -> Option<(u64, dorc_core::RunInstant)> {
    let body = line.strip_prefix("instant ordinal=")?;
    let (ordinal, at) = body.split_once(" at=")?;
    let ordinal = u64::try_from(bounded_number(ordinal, limits).ok()?).ok()?;
    Some((ordinal, parse_started(at, limits).ok()??))
}

fn bounded_number(value: &str, limits: WhylogLimits) -> Result<usize, AdmissionRefusal> {
    digits_valid(value, limits)?;
    value.parse().map_err(|_| AdmissionRefusal::Numeric)
}

fn bounded_u32(value: &str, limits: WhylogLimits) -> Result<u32, AdmissionRefusal> {
    u32::try_from(bounded_number(value, limits)?).map_err(|_| AdmissionRefusal::Numeric)
}

fn digits_valid(value: &str, limits: WhylogLimits) -> Result<(), AdmissionRefusal> {
    if value.is_empty()
        || value.len() > limits.numeric_digits
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(AdmissionRefusal::Numeric);
    }
    Ok(())
}

fn digest_valid(value: &str, limits: WhylogLimits) -> bool {
    (limits.digest_hex_min..=limits.digest_hex_max).contains(&value.len())
        && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn free_valid(value: &str, limits: WhylogLimits) -> bool {
    !value.is_empty()
        && value.len() <= limits.outer_field_bytes
        && !value.contains(TERMINAL_TOKEN)
        && value.bytes().all(|byte| byte >= 0x20 && byte != 0x7f)
}

fn atom_valid(value: &str, limits: WhylogLimits) -> bool {
    free_valid(value, limits) && !value.bytes().any(|byte| byte.is_ascii_whitespace())
}

fn mode_valid(value: &str) -> bool {
    matches!(value, "plan" | "apply" | "roundtrip" | "whylog-replay")
}

fn disposition_valid(value: &str) -> bool {
    matches!(value, "run" | "replace" | "guard" | "omit")
}

fn valid_apply_rows(apply: &[ApplyLine], limits: WhylogLimits) -> Result<(), WhylogWriteRefusal> {
    let mut previous_leaf = None;
    for apply in apply {
        if !disposition_valid(&apply.disposition)
            || digits_valid(apply.leaf.to_string().as_str(), limits).is_err()
            || previous_leaf.is_some_and(|previous| apply.leaf <= previous)
        {
            return Err(WhylogWriteRefusal::Grammar);
        }
        previous_leaf = Some(apply.leaf);
    }
    Ok(())
}

fn retain(total: &mut usize, amount: usize, limits: WhylogLimits) -> Result<(), AdmissionRefusal> {
    *total = total
        .checked_add(amount)
        .ok_or(AdmissionRefusal::ArithmeticOverflow)?;
    (*total <= limits.outer_retained_bytes)
        .then_some(())
        .ok_or(AdmissionRefusal::RetainedLimit)
}

fn retain_metadata(
    total: &mut usize,
    value: &str,
    limits: WhylogLimits,
) -> Result<(), WhylogWriteRefusal> {
    retain(total, value.len(), limits).map_err(|reason| match reason {
        AdmissionRefusal::ArithmeticOverflow => WhylogWriteRefusal::ArithmeticOverflow,
        _ => WhylogWriteRefusal::Limit,
    })
}

fn checked_append(
    out: &mut Vec<u8>,
    value: &[u8],
    limits: WhylogLimits,
) -> Result<(), WhylogWriteRefusal> {
    let total = out
        .len()
        .checked_add(value.len())
        .ok_or(WhylogWriteRefusal::ArithmeticOverflow)?;
    if total > limits.outer_bytes {
        return Err(WhylogWriteRefusal::Limit);
    }
    out.extend_from_slice(value);
    Ok(())
}

fn write_v2_line(
    out: &mut Vec<u8>,
    line: impl AsRef<str>,
    limits: WhylogLimits,
) -> Result<(), WhylogWriteRefusal> {
    let line = line.as_ref();
    if line.len() > limits.outer_line_bytes {
        return Err(WhylogWriteRefusal::Limit);
    }
    checked_append(out, line.as_bytes(), limits)?;
    checked_append(out, b"\n", limits)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The version coupling `28E:prop-unnarrated-is-visible`'s caveat demands, as a gate rather
    /// than a comment. The `[unnarrated: <class>]` census states which narrative classes this
    /// binary's renders consume; run against a durable from a binary whose plane held a different
    /// class set, it would be a confident claim about a run it cannot see. Bumping the durable's
    /// stream without bumping the plane is exactly how that would happen silently, so it fails
    /// here — and the declared number must match the tag it is derived from, or the whole coupling
    /// is keyed on nothing.
    #[test]
    fn record_stream_version_matches_the_narrative_plane() {
        assert_eq!(
            WHYLOG_V2_TAG,
            format!("dorc-whylog/{RECORD_STREAM_VERSION}"),
            "the declared stream version must be the one the wire tag actually carries"
        );
        assert_eq!(
            RECORD_STREAM_VERSION,
            dorc_aid::narrative::PLANE_VERSION,
            "the durable's record stream and the narrative plane version TOGETHER, or the \
             unnarrated census lies about old receipts"
        );
    }

    /// The four framing conditions a reader is told about in the durable's own vocabulary, plus the
    /// version refusal — over ONE grammar (`rul-strawman-formats-no-compat`).
    #[test]
    fn exact_inspection_names_absence_corruption_version_and_desync() {
        assert_eq!(
            inspect(None, ".whylog", None, no_sources)[0].code.slug(),
            "whylog-absent"
        );
        assert_eq!(
            inspect(
                Some("this is not a whylog at all\n"),
                ".whylog",
                None,
                no_sources
            )[0]
            .code
            .slug(),
            "whylog-corrupt"
        );
        assert_eq!(
            inspect(
                Some("dorc-whylog/3 nonce=dorc @@dorc@@\n"),
                ".whylog",
                None,
                no_sources
            )[0]
            .code
            .slug(),
            "whylog-version-refused"
        );

        let durable = clean_v2_durable();
        assert!(
            inspect(Some(&durable), ".whylog", None, no_sources).is_empty(),
            "a clean durable names nothing"
        );
        assert_eq!(
            inspect(Some(&durable), ".whylog", Some("changed book"), no_sources)[0]
                .code
                .slug(),
            "whylog-book-desync"
        );
        // The ORACLE arm, which the loom's `--last` route is the only reachable renderer of: a
        // book-digest drift enters the DEGRADED receipt instead (`28F:rul-drift-replay-d1`), so this
        // is the arm that still refuses outright.
        assert_eq!(
            inspect(Some(&durable), ".whylog", None, |path| {
                (path == "oracle.sh").then(|| "changed oracle".to_owned())
            })[0]
                .code
                .slug(),
            "whylog-book-desync"
        );
    }

    /// A resolver that finds nothing on disk — for the arms that are not about desync.
    fn no_sources(_: &str) -> Option<String> {
        None
    }

    /// A truncated durable loses its end sentinel, and that is the condition the reader is named —
    /// never a panic (`inv-no-throw`: malformed bytes are DATA).
    #[test]
    fn a_truncated_durable_is_named_by_its_missing_sentinel() {
        let mut durable = clean_v2_durable();
        durable.truncate(durable.len() / 2);
        assert_eq!(
            inspect(Some(&durable), ".whylog", None, no_sources)[0]
                .code
                .slug(),
            "whylog-corrupt"
        );
    }

    /// A results block claiming more bytes than the file holds is its own named condition.
    #[test]
    fn an_overrunning_results_block_is_named_apart_from_a_missing_sentinel() {
        let raw = format!(
            "{WHYLOG_V2_TAG} nonce=dorc attempt=0 host=fixture target=width-one \
             generation=width-one mode=plan started=- {TERMINAL_TOKEN}\n\
             results bytes=9999 {TERMINAL_TOKEN}\n"
        );
        let named = inspect(Some(&raw), ".whylog", None, no_sources);
        assert_eq!(named[0].code.slug(), "whylog-corrupt");
        assert!(
            matches!(
                &named[0].code,
                DiagCode::WhylogCorrupt(WhylogCorrupt {
                    reason: WhylogCorruptReason::ResultsBlockOverruns
                })
            ),
            "the overrun outranks the missing sentinel: it is the more specific observation"
        );
    }

    /// One clean v2 durable, serialized through the real writer.
    fn clean_v2_durable() -> String {
        let bytes = try_serialize_fixture_v2(
            &v2_doc(),
            WhylogLimits::spike_default(),
            HostEvidenceLimits::spike_default(),
        )
        .expect("the fixture serializes");
        String::from_utf8(bytes).expect("the durable is text")
    }

    #[derive(Clone)]
    struct V2Fixture {
        mode: String,
        argv: Vec<String>,
        book: (String, String),
        oracles: Vec<(String, String)>,
        nonce: String,
        attempt: u32,
        host: String,
        decision_digest: String,
        started_at: Option<dorc_core::RunInstant>,
        instants: Vec<(u64, dorc_core::RunInstant)>,
        raw_results: String,
        apply: Vec<ApplyLine>,
    }

    fn v2_doc() -> V2Fixture {
        V2Fixture {
            mode: "plan".to_owned(),
            argv: vec!["dorc".to_owned(), "plan".to_owned()],
            book: ("book.sh".to_owned(), "0123456789abcdef".to_owned()),
            oracles: vec![("oracle.sh".to_owned(), "fedcba9876543210".to_owned())],
            nonce: "dorc".to_owned(),
            attempt: 1,
            host: "localhost".to_owned(),
            decision_digest: "0011223344556677".to_owned(),
            started_at: None,
            instants: Vec::new(),
            raw_results: format!(
                "dorc-records/1 nonce=dorc attempt=1 host=localhost book=0123456789abcdef sites=1 {TERMINAL_TOKEN}\n\
                 dorc site 0 effect=holds rc=0 {TERMINAL_TOKEN}\n\
                 dorc-records-end/1 nonce=dorc {TERMINAL_TOKEN}\n"
            ),
            apply: vec![ApplyLine {
                leaf: 0,
                disposition: "replace".to_owned(),
                predicted: true,
                account: DurableAccount::of_decision(
                    dorc_core::influence::InfluenceAccount::authored_before_contact(),
                ),
            }],
        }
    }

    fn inner_limits(stream_bytes: usize) -> HostEvidenceLimits {
        HostEvidenceLimits::new(
            stream_bytes,
            64 * 1024,
            65_536,
            16 * 1024,
            4 * 1024 * 1024,
            32_768,
            16,
        )
    }

    fn parser_limits(
        outer_line_bytes: usize,
        outer_field_bytes: usize,
        outer_retained_bytes: usize,
        numeric_digits: usize,
        argv_entries: usize,
        oracle_entries: usize,
        apply_entries: usize,
    ) -> WhylogLimits {
        WhylogLimits::new(
            16 * 1024 * 1024,
            outer_line_bytes,
            outer_field_bytes,
            outer_retained_bytes,
            numeric_digits,
            argv_entries,
            oracle_entries,
            apply_entries,
            16,
            128,
        )
    }

    fn try_serialize_fixture_v2(
        fixture: &V2Fixture,
        limits: WhylogLimits,
        inner_limits: HostEvidenceLimits,
    ) -> Result<Vec<u8>, WhylogWriteRefusal> {
        let metadata = WhylogV2Metadata {
            mode: fixture.mode.clone(),
            argv: fixture.argv.clone(),
            book: fixture.book.clone(),
            oracles: fixture.oracles.clone(),
            nonce: fixture.nonce.clone(),
            attempt: fixture.attempt,
            host: fixture.host.clone(),
            decision_digest: fixture.decision_digest.clone(),
            started_at: fixture.started_at,
            instants: fixture.instants.clone(),
            apply: fixture.apply.clone(),
        };
        let Admission::Admitted(bytes) =
            crate::records::read_host_evidence(fixture.raw_results.as_bytes(), inner_limits)
        else {
            return Err(WhylogWriteRefusal::Limit);
        };
        let framing = Framing::spike(fixture.book.1.clone());
        let Admission::Admitted(records) =
            admit_unscoped_host_records(&bytes, &framing, inner_limits)
        else {
            return Err(WhylogWriteRefusal::Grammar);
        };
        let write = WhylogV2Write::new(&metadata, records.read().0);
        try_serialize_v2(&write, limits)
    }

    fn v2_wire(doc: &V2Fixture) -> Vec<u8> {
        try_serialize_fixture_v2(
            doc,
            WhylogLimits::spike_default(),
            inner_limits(8 * 1024 * 1024),
        )
        .expect("the admitted fixture is v2 grammar-valid")
    }

    #[test]
    fn run_instant_survives_the_durable_and_absence_stays_absence() {
        // The one invocation fact replay cannot recompute, so it must survive the wire EXACTLY —
        // rounding dates a receipt wrong, and a clockless writer must not read back as the epoch.
        let limits = WhylogLimits::spike_default();
        let read_back = |doc: &V2Fixture| {
            let Admission::Admitted(envelope) = admit_unscoped_whylog(&v2_wire(doc)[..], limits)
            else {
                panic!("clean v2 durable must admit")
            };
            envelope.claims.started_at
        };
        let mut timed = v2_doc();
        timed.started_at = Some(dorc_core::RunInstant(1_753_401_637_123));
        assert_eq!(
            read_back(&timed),
            Some(dorc_core::RunInstant(1_753_401_637_123)),
            "millisecond-exact, not rounded to the second"
        );

        let mut clockless = v2_doc();
        clockless.started_at = None;
        assert_eq!(
            read_back(&clockless),
            None,
            "no clock ⇒ no instant, never RunInstant(0)"
        );
        assert_ne!(
            read_back(&clockless),
            Some(dorc_core::RunInstant(0)),
            "the absent atom and a zero instant are distinct wire shapes"
        );

        // Symmetry: an over-wide instant is refused at write, not emitted then called corrupt.
        let mut overwide = v2_doc();
        overwide.started_at = Some(dorc_core::RunInstant(u64::MAX));
        let narrow = parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 10, 2, 1, 1);
        assert_eq!(
            try_serialize_fixture_v2(&overwide, narrow, inner_limits(8 * 1024 * 1024)),
            Err(WhylogWriteRefusal::Numeric),
        );
    }

    /// One row's account through the write, the wire, and the read — the DURABLE ACCOUNT EXPORT,
    /// built whole and then switched off (`30Q` §5g, the human's typed flow).
    ///
    /// Pinned red because [`ACCOUNT_EXPORT`] is `false`: with the switch off the writer emits no
    /// `account=` field, so the reader rehydrates every row at `untracked` and an authored row
    /// cannot come back authored. That is not a defect — it is the export being off, and this pin
    /// is what stops "off" from being indistinguishable from "unbuilt". A human flipping the
    /// switch after the review greens it.
    #[test]
    fn an_exported_account_survives_the_durable_round_trip() {
        let limits = WhylogLimits::spike_default();
        let mut doc = v2_doc();
        doc.apply = vec![ApplyLine {
            leaf: 0,
            disposition: "replace".to_owned(),
            predicted: true,
            account: DurableAccount::of_decision(
                dorc_core::influence::InfluenceAccount::authored_before_contact(),
            ),
        }];
        let Admission::Admitted(envelope) = admit_unscoped_whylog(&v2_wire(&doc)[..], limits)
        else {
            panic!("clean v2 durable must admit")
        };
        let read_back = envelope.apply[0].account;
        internal_tooling::xfail::xfail_until("p-x-durable-account-export-is-enabled", || {
            assert_eq!(
                read_back.label(),
                "authored-before-contact",
                "an exported authored account must come back authored, not as the absent-reads-highest floor"
            );
            assert!(
                !read_back.was_influenced(),
                "and the report plane must be able to say so"
            );
        });
    }

    /// The SWITCH-OFF half, green and staying green: an absent `account=` field rehydrates at the
    /// MOST-influenced point, never the least (`306b:rul-missing-influence-grade-reads-highest`).
    ///
    /// This is the property that makes the export safe to ship disabled, and safe to lose: a
    /// durable with no account metadata — one written before the export existed, one written by a
    /// build with the switch off, one truncated — reads as `untracked`, so metadata loss can only
    /// make a reader more careful.
    #[test]
    fn an_absent_exported_account_reads_at_the_most_influenced_point() {
        let limits = WhylogLimits::spike_default();
        let doc = v2_doc();
        let wire = v2_wire(&doc);
        assert!(
            !String::from_utf8_lossy(&wire).contains("account="),
            "with the export switched off the wire carries no account field at all"
        );
        let Admission::Admitted(envelope) = admit_unscoped_whylog(&wire[..], limits) else {
            panic!("clean v2 durable must admit")
        };
        assert_eq!(envelope.apply[0].account.label(), "untracked");
        assert!(
            envelope.apply[0].account.was_influenced(),
            "absent is never authored: an unreadable account is not an absent constraint"
        );
    }

    /// An UNRECOGNISED account token is bounded inert material, not a refusal and not a lower
    /// grade: it reads at the same most-influenced point an absent one does.
    ///
    /// A refusal would be the wrong answer for a field a future version may widen; a lower grade
    /// would be the laundering `306b` §3a forbids. The closed vocabulary decides what a reader
    /// BELIEVES, never whether the document is admissible.
    #[test]
    fn an_unrecognised_account_token_reads_at_the_most_influenced_point() {
        let limits = WhylogLimits::spike_default();
        let wire = String::from_utf8(v2_wire(&v2_doc())).expect("the v2 wire is utf-8");
        let doctored = wire.replace(
            &format!("predicted=1 {TERMINAL_TOKEN}"),
            &format!("predicted=1 account=from-the-future {TERMINAL_TOKEN}"),
        );
        assert_ne!(doctored, wire, "the doctoring must actually land");
        let Admission::Admitted(envelope) = admit_unscoped_whylog(doctored.as_bytes(), limits)
        else {
            panic!("an unknown account token is inert material, never a framing refusal")
        };
        assert_eq!(envelope.apply[0].account.label(), "untracked");
    }

    #[test]
    fn v2_round_trips_to_the_same_unscoped_records_as_direct_ingress() {
        let doc = v2_doc();
        let limits = WhylogLimits::spike_default();
        let serialized = try_serialize_fixture_v2(&doc, limits, inner_limits(8 * 1024 * 1024))
            .expect("complete v2 durable");
        let Admission::Admitted(envelope) = admit_unscoped_whylog(&serialized[..], limits) else {
            panic!("clean v2 durable must admit")
        };
        let framing = Framing::spike(doc.book.1.clone());
        let Admission::Admitted(replay) =
            admit_unscoped_whylog_replay(envelope, &framing, HostEvidenceLimits::spike_default())
        else {
            panic!("matching controller framing must admit the nested records")
        };
        let Admission::Admitted(direct_bytes) = crate::records::read_host_evidence(
            doc.raw_results.as_bytes(),
            HostEvidenceLimits::spike_default(),
        ) else {
            panic!("direct bounded input")
        };
        let Admission::Admitted(direct) = admit_unscoped_host_records(
            &direct_bytes,
            &framing,
            HostEvidenceLimits::spike_default(),
        ) else {
            panic!("direct nested records")
        };
        assert_eq!(replay.records, direct.into_read().0);
    }

    #[test]
    fn v2_refuses_v1_and_any_trailing_or_reordered_terminal_bytes() {
        let doc = v2_doc();
        // The v1 GRAMMAR is deleted, so this is now the only sense in which v1 still exists: a
        // header tag this binary refuses. Written as literal bytes because there is no longer a
        // writer that can produce one — which is the point of deleting it.
        assert!(matches!(
            admit_unscoped_whylog(
                format!("dorc-whylog/1 nonce=dorc {TERMINAL_TOKEN}\n").as_bytes(),
                WhylogLimits::spike_default()
            ),
            Admission::Refused(AdmissionRefusal::IncompatibleVersion)
        ));
        let raw = String::from_utf8(
            try_serialize_fixture_v2(
                &doc,
                WhylogLimits::spike_default(),
                inner_limits(8 * 1024 * 1024),
            )
            .expect("v2 write"),
        )
        .expect("fixture v2 is text outside the opaque block");
        for malformed in [
            format!("{raw}trailing"),
            raw.replacen(WHYLOG_V2_END, "wrong-end", 1),
            raw.replacen("oracle ordinal=0", "oracle ordinal=1", 1),
            raw.replacen("results bytes=", "results bytes=99999999999999999", 1),
        ] {
            assert!(matches!(
                admit_unscoped_whylog(malformed.as_bytes(), WhylogLimits::spike_default()),
                Admission::Refused(_)
            ));
        }
    }

    #[test]
    fn v2_defaults_are_exact_and_the_writer_refuses_instead_of_truncating() {
        assert_eq!(
            WhylogLimits::spike_default(),
            WhylogLimits::new(
                16 * 1024 * 1024,
                64 * 1024,
                16 * 1024,
                4 * 1024 * 1024,
                16,
                32_768,
                32_768,
                32_768,
                16,
                128,
            )
        );
        let doc = v2_doc();
        let refusal = try_serialize_fixture_v2(
            &doc,
            WhylogLimits::new(
                1,
                64 * 1024,
                16 * 1024,
                4 * 1024 * 1024,
                16,
                32_768,
                32_768,
                32_768,
                16,
                128,
            ),
            inner_limits(8 * 1024 * 1024),
        );
        assert_eq!(refusal, Err(WhylogWriteRefusal::Limit));
    }

    #[test]
    fn v2_enforces_order_singletons_and_closed_values() {
        let raw = String::from_utf8(
            try_serialize_fixture_v2(
                &v2_doc(),
                WhylogLimits::spike_default(),
                inner_limits(8 * 1024 * 1024),
            )
            .expect("v2 write"),
        )
        .expect("fixture is text outside results");
        for malformed in [
            raw.replacen("book digest=", "argv value=x ", 1),
            raw.replacen(
                "digest decision=",
                "digest decision=0011223344556677\ndigest decision=",
                1,
            ),
            raw.replacen("results bytes=", "results bytes=0\nresults bytes=", 1),
            raw.replacen(
                "oracle ordinal=0",
                "argv value=before @@dorc@@\noracle ordinal=0",
                1,
            ),
            raw.replacen("mode=plan", "mode=unknown", 1),
            raw.replacen("target=width-one", "target=wide", 1),
            raw.replacen("nonce=dorc", "nonce=dorc nonce=again", 1),
            raw.replacen("apply leaf=0", "apply leaf=-1", 1),
        ] {
            assert!(matches!(
                admit_unscoped_whylog(malformed.as_bytes(), WhylogLimits::spike_default()),
                Admission::Refused(_)
            ));
        }
    }

    #[test]
    fn v2_writer_refuses_duplicate_or_nonincreasing_apply_leaves() {
        let mut duplicate = v2_doc();
        duplicate.apply.push(ApplyLine {
            leaf: 0,
            disposition: "run".to_owned(),
            predicted: true,
            account: DurableAccount::of_decision(
                dorc_core::influence::InfluenceAccount::authored_before_contact(),
            ),
        });
        let mut non_increasing = v2_doc();
        non_increasing.apply = vec![
            ApplyLine {
                leaf: 1,
                disposition: "replace".to_owned(),
                predicted: true,
                account: DurableAccount::of_decision(
                    dorc_core::influence::InfluenceAccount::authored_before_contact(),
                ),
            },
            ApplyLine {
                leaf: 0,
                disposition: "run".to_owned(),
                predicted: true,
                account: DurableAccount::of_decision(
                    dorc_core::influence::InfluenceAccount::authored_before_contact(),
                ),
            },
        ];
        for doc in [duplicate, non_increasing] {
            assert_eq!(
                try_serialize_fixture_v2(
                    &doc,
                    WhylogLimits::spike_default(),
                    inner_limits(8 * 1024 * 1024),
                ),
                Err(WhylogWriteRefusal::Grammar)
            );
        }
    }

    #[test]
    fn v2_limits_apply_to_every_outer_owned_metadata_class() {
        let mut doc = v2_doc();
        let exact = WhylogLimits::new(
            16 * 1024 * 1024,
            64 * 1024,
            16 * 1024,
            4096,
            16,
            2,
            1,
            1,
            16,
            128,
        );
        assert_eq!(
            try_serialize_fixture_v2(&doc, exact, inner_limits(8 * 1024 * 1024)).map(|_| ()),
            Ok(())
        );

        doc.argv.push("third".to_owned());
        assert_eq!(
            try_serialize_fixture_v2(&doc, exact, inner_limits(8 * 1024 * 1024)),
            Err(WhylogWriteRefusal::Limit)
        );
        doc.argv.pop();
        doc.oracles
            .push(("second.sh".to_owned(), "0123456789abcdef".to_owned()));
        assert_eq!(
            try_serialize_fixture_v2(&doc, exact, inner_limits(8 * 1024 * 1024)),
            Err(WhylogWriteRefusal::Limit)
        );
        doc.oracles.pop();
        doc.apply.push(ApplyLine {
            leaf: 1,
            disposition: "run".to_owned(),
            predicted: true,
            account: DurableAccount::of_decision(
                dorc_core::influence::InfluenceAccount::authored_before_contact(),
            ),
        });
        assert_eq!(
            try_serialize_fixture_v2(&doc, exact, inner_limits(8 * 1024 * 1024)),
            Err(WhylogWriteRefusal::Limit)
        );

        let field_limited =
            WhylogLimits::new(16 * 1024 * 1024, 64 * 1024, 3, 128, 16, 8, 8, 8, 16, 128);
        assert_eq!(
            try_serialize_fixture_v2(&v2_doc(), field_limited, inner_limits(8 * 1024 * 1024)),
            Err(WhylogWriteRefusal::Grammar)
        );
        let retained_limited = WhylogLimits::new(
            16 * 1024 * 1024,
            64 * 1024,
            16 * 1024,
            1,
            16,
            8,
            8,
            8,
            16,
            128,
        );
        assert_eq!(
            try_serialize_fixture_v2(&v2_doc(), retained_limited, inner_limits(8 * 1024 * 1024)),
            Err(WhylogWriteRefusal::Limit)
        );
    }

    #[test]
    fn v2_preserves_ordered_oracle_digest_and_path_claims() {
        let mut doc = v2_doc();
        doc.oracles = vec![
            ("same.oracle".to_owned(), "0123456789abcdef".to_owned()),
            ("same.oracle".to_owned(), "fedcba9876543210".to_owned()),
        ];
        let wire = try_serialize_fixture_v2(
            &doc,
            WhylogLimits::spike_default(),
            inner_limits(8 * 1024 * 1024),
        )
        .expect("v2 write");
        let Admission::Admitted(envelope) =
            admit_unscoped_whylog(&wire[..], WhylogLimits::spike_default())
        else {
            panic!("ordered identities must be retained")
        };
        let identities = envelope.recorded_oracles();
        assert_eq!(identities.len(), 2);
        assert_eq!(identities[0].ordinal(), 0);
        assert_eq!(identities[0].digest(), "0123456789abcdef");
        assert_eq!(identities[1].ordinal(), 1);
        assert_eq!(identities[1].digest(), "fedcba9876543210");
        assert_eq!(identities[0].path().as_str(), identities[1].path().as_str());
    }

    #[test]
    fn v2_direct_and_replay_admission_share_the_injected_inner_ceiling() {
        let mut doc = v2_doc();
        let nine_mebibytes = 9 * 1024 * 1024;
        let smaller_inner = inner_limits(8 * 1024 * 1024);
        let larger_inner = inner_limits(nine_mebibytes);
        doc.raw_results = inner_at_exact_size(nine_mebibytes);
        let outer_limits = WhylogLimits::spike_default();
        let wire = try_serialize_fixture_v2(&doc, outer_limits, larger_inner)
            .expect("the explicit nine MiB policy writes the complete durable");

        let Admission::Admitted(direct_bytes) =
            crate::records::read_host_evidence(doc.raw_results.as_bytes(), larger_inner)
        else {
            panic!("direct admission honors the injected nine MiB ceiling")
        };
        let Admission::Admitted(direct) = admit_unscoped_host_records(
            &direct_bytes,
            &Framing::spike(doc.book.1.clone()),
            larger_inner,
        ) else {
            panic!("the valid direct site record admits")
        };

        let Admission::Admitted(envelope) = admit_unscoped_whylog(&wire[..], outer_limits) else {
            panic!("outer admission does not impose an inner policy")
        };
        let Admission::Admitted(replay) = admit_unscoped_whylog_replay(
            envelope,
            &Framing::spike(doc.book.1.clone()),
            larger_inner,
        ) else {
            panic!("the valid replay site record admits")
        };
        assert_eq!(replay.records, direct.into_read().0);

        assert!(matches!(
            crate::records::read_host_evidence(doc.raw_results.as_bytes(), smaller_inner),
            Admission::Refused(AdmissionRefusal::StreamLimit)
        ));
        assert_eq!(
            try_serialize_fixture_v2(&doc, outer_limits, smaller_inner),
            Err(WhylogWriteRefusal::Limit)
        );
        let Admission::Admitted(envelope) = admit_unscoped_whylog(&wire[..], outer_limits) else {
            panic!("outer admission remains independent of the smaller inner policy")
        };
        assert!(matches!(
            admit_unscoped_whylog_replay(
                envelope,
                &Framing::spike(doc.book.1.clone()),
                smaller_inner
            ),
            Admission::Refused(AdmissionRefusal::StreamLimit)
        ));

        assert!(matches!(
            admit_unscoped_whylog(vec![b'x'; 16 * 1024 * 1024 + 1].as_slice(), outer_limits),
            Admission::Refused(AdmissionRefusal::StreamLimit)
        ));
    }

    #[test]
    fn v2_parser_outer_boundaries_are_exact_and_one_over() {
        let doc = v2_doc();
        let wire = v2_wire(&doc);
        let text = String::from_utf8(wire.clone()).expect("outer fixture is text");
        let header_len = text.find('\n').expect("header newline");
        let outer_exact = WhylogLimits::new(
            wire.len(),
            64 * 1024,
            16 * 1024,
            4 * 1024 * 1024,
            16,
            32_768,
            32_768,
            32_768,
            16,
            128,
        );
        assert!(matches!(
            admit_unscoped_whylog(&wire[..], outer_exact),
            Admission::Admitted(_)
        ));
        let line_limits = parser_limits(header_len, 16 * 1024, 4 * 1024 * 1024, 16, 2, 1, 1);
        assert!(matches!(
            admit_unscoped_whylog(&wire[..], line_limits),
            Admission::Admitted(_)
        ));
        let one_longer_header = text.replacen("host=localhost", "host=localhostx", 1);
        assert!(matches!(
            admit_unscoped_whylog(one_longer_header.as_bytes(), line_limits),
            Admission::Refused(AdmissionRefusal::LineLimit)
        ));

        let field_limits = parser_limits(64 * 1024, 9, 4 * 1024 * 1024, 16, 2, 1, 1);
        let field_exact = text.replacen("argv value=dorc", "argv value=123456789", 1);
        let field_one_over =
            field_exact.replacen("argv value=123456789", "argv value=1234567890", 1);
        assert!(matches!(
            admit_unscoped_whylog(field_exact.as_bytes(), field_limits),
            Admission::Admitted(_)
        ));
        assert!(matches!(
            admit_unscoped_whylog(field_one_over.as_bytes(), field_limits),
            Admission::Refused(AdmissionRefusal::Grammar)
        ));

        let digits_exact = text.replacen("attempt=1", "attempt=4294967295", 1);
        let digits_one_over = digits_exact.replacen("attempt=4294967295", "attempt=42949672950", 1);
        let digit_limits = parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 10, 2, 1, 1);
        assert!(matches!(
            admit_unscoped_whylog(digits_exact.as_bytes(), digit_limits),
            Admission::Admitted(_)
        ));
        assert!(matches!(
            admit_unscoped_whylog(digits_one_over.as_bytes(), digit_limits),
            Admission::Refused(_)
        ));

        let retained_exact = parser_limits(64 * 1024, 16 * 1024, 114, 16, 2, 1, 1);
        let retained_one_over = parser_limits(64 * 1024, 16 * 1024, 113, 16, 2, 1, 1);
        assert!(matches!(
            admit_unscoped_whylog(&wire[..], retained_exact),
            Admission::Admitted(_)
        ));
        assert!(matches!(
            admit_unscoped_whylog(&wire[..], retained_one_over),
            Admission::Refused(AdmissionRefusal::RetainedLimit)
        ));
    }

    #[test]
    fn v2_parser_cardinality_and_control_boundaries_refuse() {
        let doc = v2_doc();
        let text = String::from_utf8(v2_wire(&doc)).expect("outer fixture is text");
        let mut argv_one_over = doc.clone();
        argv_one_over.argv.push("third".to_owned());
        let mut oracle_one_over = doc.clone();
        oracle_one_over
            .oracles
            .push(("second.sh".to_owned(), "0123456789abcdef".to_owned()));
        let mut apply_exact = doc.clone();
        apply_exact.apply.push(ApplyLine {
            leaf: 1,
            disposition: "run".to_owned(),
            predicted: true,
            account: DurableAccount::of_decision(
                dorc_core::influence::InfluenceAccount::authored_before_contact(),
            ),
        });
        let cardinality_cases = [
            (
                v2_wire(&doc),
                parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 16, 2, 1, 1),
                true,
            ),
            (
                v2_wire(&argv_one_over),
                parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 16, 2, 1, 1),
                false,
            ),
            (
                v2_wire(&oracle_one_over),
                parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 16, 2, 1, 1),
                false,
            ),
            (
                v2_wire(&apply_exact),
                parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 16, 2, 1, 2),
                true,
            ),
            (
                v2_wire(&apply_exact),
                parser_limits(64 * 1024, 16 * 1024, 4 * 1024 * 1024, 16, 2, 1, 1),
                false,
            ),
        ];
        for (wire, limits, admitted) in cardinality_cases {
            assert_eq!(
                matches!(
                    admit_unscoped_whylog(&wire[..], limits),
                    Admission::Admitted(_)
                ),
                admitted
            );
        }

        let duplicate_apply = String::from_utf8(v2_wire(&apply_exact))
            .expect("outer fixture is text")
            .replacen("apply leaf=1", "apply leaf=0", 1);
        let non_increasing_apply = String::from_utf8(v2_wire(&apply_exact))
            .expect("outer fixture is text")
            .replacen(
                "apply leaf=0 disposition=replace",
                "apply leaf=1 disposition=replace",
                1,
            )
            .replacen(
                "apply leaf=1 disposition=run",
                "apply leaf=0 disposition=run",
                1,
            );
        for malformed in [
            duplicate_apply,
            non_increasing_apply,
            text.replace('\n', "\r\n"),
            text.replacen("mode=plan", "mode=pl\u{0001}an", 1),
        ] {
            assert!(matches!(
                admit_unscoped_whylog(malformed.as_bytes(), WhylogLimits::spike_default()),
                Admission::Refused(_)
            ));
        }
    }

    #[test]
    fn v2_inner_range_stays_internal_to_bounded_host_bytes() {
        let source = include_str!("whylog.rs");
        let records = include_str!("records.rs");
        assert!(source.contains("with_admitted_range(envelope.inner_range)"));
        assert!(!source.contains(&["backing.", "clone()"].concat()));
        assert!(!records.contains("pub fn from_owned_backing"));
        assert!(!records.contains("pub fn admitted_bytes"));
    }

    #[test]
    fn v2_writer_has_no_legacy_or_raw_result_input_route() {
        let source = include_str!("whylog.rs");
        let Some(writer) = source
            .split("pub fn try_serialize_v2")
            .nth(1)
            .and_then(|tail| tail.split("fn parse_v2").next())
        else {
            panic!("v2 writer source must remain identifiable");
        };
        assert!(source.contains("pub struct WhylogV2Write<'a> {\n    metadata:"));
        assert!(source.contains("    records: &'a AdmittedUnscopedHostRecords,"));
        assert!(writer.contains("&WhylogV2Write<'_>"));
        assert!(!writer.contains("WhylogDoc"));
        assert!(!writer.contains("raw_results"));
    }

    #[test]
    fn v2_treats_results_as_opaque_until_inner_admission() {
        let doc = v2_doc();
        let mut wire = try_serialize_fixture_v2(
            &doc,
            WhylogLimits::spike_default(),
            inner_limits(8 * 1024 * 1024),
        )
        .expect("v2 write");
        let start = wire
            .windows(b"results bytes=".len())
            .position(|window| window == b"results bytes=")
            .and_then(|offset| {
                wire[offset..]
                    .iter()
                    .position(|byte| *byte == b'\n')
                    .map(|n| offset + n + 1)
            })
            .expect("results line");
        wire[start] = 0xff;
        let Admission::Admitted(envelope) =
            admit_unscoped_whylog(&wire[..], WhylogLimits::spike_default())
        else {
            panic!("outer grammar must not decode opaque result bytes")
        };
        assert!(matches!(
            admit_unscoped_whylog_replay(
                envelope,
                &Framing::spike(doc.book.1),
                HostEvidenceLimits::spike_default()
            ),
            Admission::Refused(AdmissionRefusal::InvalidUtf8)
        ));
    }

    /// The instants a run heard its records at are the one thing replay CANNOT re-derive: running
    /// the kernel again yields the plan, never the moment. If they do not survive the durable, a
    /// replayed receipt silently loses every per-row instant it had live — which is exactly the
    /// gap this line-kind exists to close, so the round-trip is pinned rather than assumed.
    #[test]
    fn recorded_instants_survive_the_durable_round_trip() {
        let mut doc = v2_doc();
        doc.instants = vec![
            (0, dorc_core::RunInstant(1_769_306_437_000)),
            (3, dorc_core::RunInstant(1_769_306_439_500)),
        ];
        let wire = try_serialize_fixture_v2(
            &doc,
            WhylogLimits::spike_default(),
            HostEvidenceLimits::spike_default(),
        )
        .expect("v2 write");
        let Admission::Admitted(envelope) =
            admit_unscoped_whylog(&wire[..], WhylogLimits::spike_default())
        else {
            panic!("the durable must admit")
        };
        assert_eq!(envelope.recorded_instants(), doc.instants.as_slice());
    }

    /// A record ordinal is a KEY. A durable claiming two arrivals for one record would let a
    /// reader date a row from a moment that belongs to nothing, so both halves refuse it — the
    /// writer before any bytes exist, the reader before any of them are believed.
    #[test]
    fn a_repeated_instant_ordinal_is_refused_by_both_halves() {
        let mut doc = v2_doc();
        doc.instants = vec![
            (2, dorc_core::RunInstant(1_000)),
            (2, dorc_core::RunInstant(2_000)),
        ];
        assert!(matches!(
            try_serialize_fixture_v2(
                &doc,
                WhylogLimits::spike_default(),
                HostEvidenceLimits::spike_default()
            ),
            Err(WhylogWriteRefusal::Grammar)
        ));

        doc.instants = vec![(2, dorc_core::RunInstant(1_000))];
        let wire = try_serialize_fixture_v2(
            &doc,
            WhylogLimits::spike_default(),
            HostEvidenceLimits::spike_default(),
        )
        .expect("v2 write");
        let text = String::from_utf8(wire).expect("ascii wire");
        let duplicated = text.replace(
            &format!("instant ordinal=2 at=1000 {TERMINAL_TOKEN}\n"),
            &format!(
                "instant ordinal=2 at=1000 {TERMINAL_TOKEN}\n\
                 instant ordinal=2 at=9999 {TERMINAL_TOKEN}\n"
            ),
        );
        assert!(matches!(
            admit_unscoped_whylog(duplicated.as_bytes(), WhylogLimits::spike_default()),
            Admission::Refused(AdmissionRefusal::Grammar)
        ));
    }

    fn inner_at_exact_size(size: usize) -> String {
        let framing = Framing::spike("0123456789abcdef".to_owned());
        let header = crate::records::header_line(&framing, 1)
            .trim_start_matches("printf '")
            .trim_end_matches("\\n'\n")
            .to_owned();
        let site_record = format!("dorc site 0 effect=holds rc=0 {TERMINAL_TOKEN}\n");
        let sentinel = format!("dorc-records-end/1 nonce=dorc {TERMINAL_TOKEN}\n");
        let mut inner = format!("{header}\n{site_record}{sentinel}");
        while size.saturating_sub(inner.len()) > 64 * 1024 {
            inner.push('#');
            inner.push_str(&"x".repeat(64 * 1024 - 2));
            inner.push('\n');
        }
        let remaining = size
            .checked_sub(inner.len())
            .expect("eight MiB exceeds framing");
        assert!(remaining >= 2);
        inner.push('#');
        inner.push_str(&"x".repeat(remaining - 2));
        inner.push('\n');
        assert_eq!(inner.len(), size);
        inner
    }
}
