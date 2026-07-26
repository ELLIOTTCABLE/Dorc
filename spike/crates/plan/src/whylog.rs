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

use dorc_aid::diag::{Diag, DiagCode, WhylogCorrupt, WhylogVersionRefused};

use std::io::Read;

// One digest, one substitution point (`rul-fixture-identity-never-production`): the content
// identity a durable carries has exactly ONE definition. Never re-inline it locally.
use crate::invocation::book_digest;
use crate::records::{
    Admission, AdmissionRefusal, AdmittedUnscopedHostRecords, BoundedHostBytes, Framing,
    HostEvidenceLimits, TERMINAL_TOKEN, admit_unscoped_host_records,
};

/// The durable's version tag — the format's identity (`27V` §2; the `report-lane-versioned-entry`
/// posture). Recognized once published; a new grammar mints a new tag. NO byte-stability within a
/// version (additive-only fields).
pub const WHYLOG_TAG: &str = "dorc-whylog/1";
/// The end sentinel (a truncated write is detected by its absence — `inv-no-throw` ⇒
/// [`DiagCode::WhylogCorrupt`]).
pub const WHYLOG_END: &str = "dorc-whylog-end/1";

/// The bounded v2 durable tag. V1 remains temporarily available only for current CLI callers.
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
}

/// The assembled thin durable (`27V` §2). Reused for [`serialize`] (write) and [`parse`] (replay).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhylogDoc {
    /// The invocation mode (`plan` / `apply` / `roundtrip`).
    pub mode: String,
    /// The full argv, one word per element (consent flags are chain-links, kept verbatim).
    pub argv: Vec<String>,
    /// The book path + its content digest (re-read from disk on replay; the digest is verified —
    /// a mismatch is [`DiagCode::WhylogBookDesync`]).
    pub book: (String, String),
    /// Each oracle path + its content digest (re-read + verified on replay).
    pub oracles: Vec<(String, String)>,
    /// The per-attempt run nonce (the deterministic replay "seed"; `27V` §2 — no RNG/clock).
    pub nonce: String,
    /// The attempt serial (`26A` retry-hygiene).
    pub attempt: u32,
    /// The session host id.
    pub host: String,
    /// The decision digest at write time — a drift signal; the re-derived digest must match on
    /// replay (else [`DiagCode::WhylogBookDesync`]).
    pub decision_digest: String,
    /// The records stream AS-RECEIVED — the raw deframed buffer (site records + the report lane +
    /// deriv/resolv/reach), replayed through the identical deframe path.
    pub raw_results: String,
    /// The apply report (per-leaf PREDICTED outcomes; spike prediction-only — see [`ApplyLine`]).
    pub apply: Vec<ApplyLine>,
}

/// Serialize a [`WhylogDoc`] to the line-framed durable. Deterministic (fixed field order; no
/// hashed-collection iteration — `inv-determinism`). The records stream rides a byte-count-prefixed
/// opaque block so its own `@@dorc@@` tokens never collide with the whylog framing.
#[must_use]
pub fn serialize(doc: &WhylogDoc) -> String {
    use std::fmt::Write;
    let t = TERMINAL_TOKEN;
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{WHYLOG_TAG} nonce={} attempt={} host={} {t}",
        doc.nonce, doc.attempt, doc.host
    );
    let _ = writeln!(out, "invoc mode={} {t}", doc.mode);
    // argv words + paths run LAST-TO-TOKEN (free content — embedded spaces survive).
    for a in &doc.argv {
        let _ = writeln!(out, "argv {a} {t}");
    }
    let _ = writeln!(out, "book digest={} path={} {t}", doc.book.1, doc.book.0);
    for (path, digest) in &doc.oracles {
        let _ = writeln!(out, "oracle digest={digest} path={path} {t}");
    }
    let _ = writeln!(out, "digest decision={} {t}", doc.decision_digest);
    for a in &doc.apply {
        let _ = writeln!(
            out,
            "apply leaf={} disposition={} predicted={} {t}",
            a.leaf,
            a.disposition,
            u8::from(a.predicted)
        );
    }
    // The records stream: byte-count-prefixed opaque block (its own tokens ride verbatim).
    let _ = writeln!(out, "results bytes={} {t}", doc.raw_results.len());
    out.push_str(&doc.raw_results);
    let _ = writeln!(out, "{WHYLOG_END} {t}");
    out
}

/// The result of parsing a durable (`inv-no-throw`): the reconstructed doc (or `None` when refused)
/// plus any refusal diagnostics.
#[derive(Debug, Default)]
pub struct WhylogParse {
    /// The reconstructed durable, or `None` when the parse refused (version/corrupt).
    pub doc: Option<WhylogDoc>,
    /// Refusal diagnostics (version-refused / corrupt). Spanless — the durable is not a source file.
    pub diagnostics: Vec<Diag>,
}

/// Current replay inputs supplied by an I/O edge.
#[derive(Debug, Clone, Copy)]
pub struct WhylogCurrent<'a> {
    /// Current book bytes.
    pub book: Option<&'a str>,
    /// Current oracle bytes by recorded path.
    pub oracles: &'a [(&'a str, &'a str)],
}

/// Inspection outcome for one durable.
#[derive(Debug)]
pub struct WhylogInspection {
    /// Parsed durable when replay remains valid.
    pub doc: Option<WhylogDoc>,
    /// Typed refusal diagnostics.
    pub diagnostics: Vec<Diag>,
}

/// Inspect one exact durable without filesystem or directory access.
///
/// Keeps CLI and transcript replay on one typed-refusal path.
///
/// # Bounded-input fence (`rul-host-bytes-bounded-before-admission`)
///
/// This takes RAW `&str` and applies NO host-evidence limits: no stream, line, record-count,
/// field, or retained-byte bound runs before [`parse`]. It is sound only for CONTROLLER-LOCAL
/// material a repo-local fixture supplied — its one caller today materializes transcript-case
/// content. It is NOT an intake path, and a durable that a managed host produced or influenced
/// must reach the engine through [`admit_unscoped_whylog`] instead, which bounds bytes-first and
/// answers with a closed admission outcome.
///
/// Adding a host-sourced caller without converting this to a bounded, scope-carrying signature
/// re-opens the unbounded-read the ingress work closed. The conversion is deliberately deferred
/// (its consumer crate is mid-restructure), so this fence is prose rather than a type: if you are
/// about to add a caller, that deferral has expired.
#[must_use]
pub fn inspect(
    raw: Option<&str>,
    identity: &str,
    current: Option<WhylogCurrent<'_>>,
) -> WhylogInspection {
    let Some(raw) = raw else {
        return WhylogInspection {
            doc: None,
            diagnostics: vec![Diag::new_spanless_site(DiagCode::WhylogAbsent(
                dorc_aid::diag::WhylogAbsent {
                    dir: identity.to_owned(),
                },
            ))],
        };
    };
    let parsed = parse(raw);
    let Some(doc) = parsed.doc else {
        return WhylogInspection {
            doc: None,
            diagnostics: parsed.diagnostics,
        };
    };
    if let Some(current) = current {
        let desync = current
            .book
            .filter(|book| book_digest(book) != doc.book.1)
            .map(|_| "book".to_owned())
            .or_else(|| {
                doc.oracles.iter().find_map(|(path, digest)| {
                    current.oracles.iter().find_map(|(current_path, source)| {
                        (*current_path == path && book_digest(source) != *digest)
                            .then(|| format!("oracle {path}"))
                    })
                })
            });
        if let Some(which) = desync {
            return WhylogInspection {
                doc: None,
                diagnostics: vec![Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                    dorc_aid::diag::WhylogBookDesync { which },
                ))],
            };
        }
    }
    WhylogInspection {
        doc: Some(doc),
        diagnostics: Vec::new(),
    }
}

/// Parse a durable's bytes into a [`WhylogDoc`] (`27V` Lane B). Total (`inv-no-throw`): a
/// wrong-version durable refuses with [`DiagCode::WhylogVersionRefused`]; a truncated/clobbered one
/// with [`DiagCode::WhylogCorrupt`]. The records block is read by its byte-count prefix (robust to
/// the stream's embedded `@@dorc@@` tokens).
#[must_use]
pub fn parse(raw: &str) -> WhylogParse {
    let mut out = WhylogParse::default();
    let Some(header_end) = raw.find('\n') else {
        return corrupt(&mut out, "empty or headerless durable");
    };
    let header = strip_token(raw[..header_end].trim_end());
    // Version gate: a different `dorc-whylog/N` refuses politely; anything else ⇒ corrupt.
    let Some(rest_after_tag) = header.strip_prefix(WHYLOG_TAG) else {
        if let Some(found) = header
            .split_whitespace()
            .next()
            .filter(|w| w.starts_with("dorc-whylog/"))
        {
            out.diagnostics
                .push(Diag::new_spanless_site(DiagCode::WhylogVersionRefused(
                    WhylogVersionRefused {
                        found: found.to_owned(),
                    },
                )));
            return out;
        }
        return corrupt(&mut out, "missing dorc-whylog header tag");
    };

    let mut doc = WhylogDoc::default();
    read_header_keys(rest_after_tag, &mut doc);

    // A `results bytes=N` line switches to reading N raw bytes verbatim.
    let mut cursor = header_end + 1;
    let bytes = raw.as_bytes();
    let mut saw_end = false;
    while cursor < raw.len() {
        let line_end = raw[cursor..].find('\n').map_or(raw.len(), |i| cursor + i);
        let line = strip_token(raw[cursor..line_end].trim_end());
        cursor = line_end + 1;
        if line == WHYLOG_END || line.starts_with(&format!("{WHYLOG_END} ")) {
            saw_end = true;
            break;
        }
        if let Some(n) = line
            .strip_prefix("results bytes=")
            .and_then(|n| n.trim().parse::<usize>().ok())
        {
            let block_end = cursor.saturating_add(n);
            if block_end > bytes.len() {
                return corrupt(&mut out, "results block byte-count exceeds the file");
            }
            raw[cursor..block_end].clone_into(&mut doc.raw_results);
            cursor = block_end;
            continue;
        }
        absorb_line(line, &mut doc);
    }
    if !saw_end {
        return corrupt(&mut out, "no dorc-whylog-end sentinel (a truncated write?)");
    }
    out.doc = Some(doc);
    out
}

/// Push a [`DiagCode::WhylogCorrupt`] and return the (doc-less) parse. Centralised so every corrupt
/// exit carries the code the completeness gate demands.
fn corrupt(out: &mut WhylogParse, detail: &str) -> WhylogParse {
    out.diagnostics
        .push(Diag::new_spanless_site(DiagCode::WhylogCorrupt(
            WhylogCorrupt {
                detail: detail.to_owned(),
            },
        )));
    std::mem::take(out)
}

/// Read `nonce=`/`attempt=`/`host=` from the header tail (additive-key tolerant — unknown keys
/// ignored, `24Kc`).
fn read_header_keys(rest: &str, doc: &mut WhylogDoc) {
    for tok in rest.split_whitespace() {
        if let Some(v) = tok.strip_prefix("nonce=") {
            v.clone_into(&mut doc.nonce);
        } else if let Some(v) = tok.strip_prefix("attempt=") {
            doc.attempt = v.parse().unwrap_or(0);
        } else if let Some(v) = tok.strip_prefix("host=") {
            v.clone_into(&mut doc.host);
        }
    }
}

/// Absorb one non-header, non-results line into the doc. Unknown tags are ignored (additive-key
/// tolerant); a malformed value drops that line (never a panic — `inv-no-throw`).
fn absorb_line(line: &str, doc: &mut WhylogDoc) {
    let Some((tag, rest)) = line.split_once(' ') else {
        return;
    };
    match tag {
        "invoc" => {
            if let Some(m) = rest.strip_prefix("mode=") {
                m.trim().clone_into(&mut doc.mode);
            }
        }
        "argv" => doc.argv.push(rest.trim().to_owned()),
        "book" => {
            if let (Some(d), Some(p)) = split_digest_path(rest) {
                doc.book = (p, d);
            }
        }
        "oracle" => {
            if let (Some(d), Some(p)) = split_digest_path(rest) {
                doc.oracles.push((p, d));
            }
        }
        "digest" => {
            if let Some(d) = rest.strip_prefix("decision=") {
                d.trim().clone_into(&mut doc.decision_digest);
            }
        }
        "apply" => {
            if let Some(a) = parse_apply(rest) {
                doc.apply.push(a);
            }
        }
        _ => {}
    }
}

/// Split a `digest=<hex> path=<free-content>` body — `path=` is last-to-token so a space-bearing
/// path survives. Returns `(digest, path)`.
fn split_digest_path(rest: &str) -> (Option<String>, Option<String>) {
    let digest = rest
        .split_whitespace()
        .find_map(|t| t.strip_prefix("digest="))
        .map(str::to_owned);
    let path = rest
        .find("path=")
        .map(|at| rest[at + "path=".len()..].to_owned());
    (digest, path)
}

/// Parse an `apply leaf=<n> disposition=<tag> predicted=<0|1>` body.
fn parse_apply(rest: &str) -> Option<ApplyLine> {
    let mut leaf = None;
    let mut disposition = None;
    let mut predicted = true;
    for tok in rest.split_whitespace() {
        if let Some(v) = tok.strip_prefix("leaf=") {
            leaf = v.parse::<u32>().ok();
        } else if let Some(v) = tok.strip_prefix("disposition=") {
            disposition = Some(v.to_owned());
        } else if let Some(v) = tok.strip_prefix("predicted=") {
            predicted = v != "0";
        }
    }
    Some(ApplyLine {
        leaf: leaf?,
        disposition: disposition?,
        predicted,
    })
}

/// Strip a trailing ` @@dorc@@` (the whylog reuses the records terminal token as a tear-detector on
/// its own single-line records). A line without it is taken as-is (defensive).
fn strip_token(line: &str) -> &str {
    line.strip_suffix(TERMINAL_TOKEN)
        .map_or(line, |b| b.strip_suffix(' ').unwrap_or(b))
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
    if envelope.claims.nonce != expected.nonce.0
        || envelope.claims.attempt != expected.attempt
        || envelope.claims.host != expected.host
        || envelope.claims.book_digest != expected.book_digest
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
        Admission::Admitted(records) => Admission::Admitted(AdmittedUnscopedWhylogReplay {
            claims: envelope.claims,
            mode: envelope.mode,
            book_path: envelope.book_path,
            oracle_sources: envelope.oracle_sources,
            argv: envelope.argv,
            apply: envelope.apply,
            records,
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
    for apply in &doc.apply {
        retain_metadata(&mut retained, &apply.disposition, limits)?;
        write_v2_line(
            &mut out,
            format!(
                "apply leaf={} disposition={} predicted={} {TERMINAL_TOKEN}",
                apply.leaf,
                apply.disposition,
                u8::from(apply.predicted)
            ),
            limits,
        )?;
    }
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
                let Some((leaf, disposition, predicted)) = parse_v2_apply(line, limits) else {
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

fn parse_v2_apply(line: &str, limits: WhylogLimits) -> Option<(u32, &str, bool)> {
    let body = line.strip_prefix("apply leaf=")?;
    let (leaf, rest) = body.split_once(" disposition=")?;
    let (disposition, predicted) = rest.split_once(" predicted=")?;
    let predicted = match predicted {
        "0" => false,
        "1" => true,
        _ => return None,
    };
    Some((
        u32::try_from(bounded_number(leaf, limits).ok()?).ok()?,
        disposition_valid(disposition).then_some(disposition)?,
        predicted,
    ))
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

    fn doc() -> WhylogDoc {
        WhylogDoc {
            mode: "plan".to_owned(),
            argv: vec!["dorc".to_owned(), "plan".to_owned(), "--risk-faultless-skips".to_owned()],
            book: ("web host.sh".to_owned(), "abc123".to_owned()), // a space-bearing path
            oracles: vec![("foobar.oracle.sh".to_owned(), "def456".to_owned())],
            nonce: "dorc".to_owned(),
            attempt: 1,
            host: "localhost".to_owned(),
            decision_digest: "0011223344556677".to_owned(),
            // raw_results carries its OWN terminal tokens (the byte-count block must survive them).
            raw_results: "dorc site 0 effect=holds rc=0 @@dorc@@\ndorc report site=1 decline unsound k @@dorc@@\n".to_owned(),
            apply: vec![
                ApplyLine { leaf: 0, disposition: "replace".to_owned(), predicted: true },
                ApplyLine { leaf: 1, disposition: "run".to_owned(), predicted: true },
            ],
        }
    }

    #[test]
    fn round_trips_including_embedded_tokens_and_spaced_paths() {
        let d = doc();
        let parsed = parse(&serialize(&d));
        assert!(
            parsed.diagnostics.is_empty(),
            "a clean durable refuses nothing: {:?}",
            parsed.diagnostics
        );
        assert_eq!(parsed.doc.as_ref(), Some(&d), "serialize→parse is identity");
    }

    #[test]
    fn wrong_version_refuses_politely_never_panics() {
        let raw =
            format!("dorc-whylog/2 nonce=dorc {TERMINAL_TOKEN}\n{WHYLOG_END} {TERMINAL_TOKEN}\n");
        let p = parse(&raw);
        assert!(p.doc.is_none());
        assert!(
            p.diagnostics
                .iter()
                .any(|d| d.code.slug() == "whylog-version-refused")
        );
    }

    #[test]
    fn truncated_durable_is_corrupt_not_a_panic() {
        let mut raw = serialize(&doc());
        raw.truncate(raw.len() / 2);
        let p = parse(&raw);
        assert!(p.doc.is_none());
        assert!(
            p.diagnostics
                .iter()
                .any(|d| d.code.slug() == "whylog-corrupt")
        );
    }

    #[test]
    fn headerless_bytes_are_corrupt() {
        let p = parse("this is not a whylog at all\n");
        assert!(p.doc.is_none());
        assert!(
            p.diagnostics
                .iter()
                .any(|d| d.code.slug() == "whylog-corrupt")
        );
    }

    #[test]
    fn exact_inspection_distinguishes_absence_parse_version_and_desync() {
        let absent = inspect(None, ".whylog", None);
        assert_eq!(absent.diagnostics[0].code.slug(), "whylog-absent");

        let corrupt = inspect(Some("dorc-whylog/1\n"), ".whylog", None);
        assert_eq!(corrupt.diagnostics[0].code.slug(), "whylog-corrupt");

        let version = inspect(
            Some("dorc-whylog/2 nonce=dorc @@dorc@@\ndorc-whylog-end/1 @@dorc@@\n"),
            ".whylog",
            None,
        );
        assert_eq!(version.diagnostics[0].code.slug(), "whylog-version-refused");

        let durable = serialize(&doc());
        let desync = inspect(
            Some(&durable),
            ".whylog",
            Some(WhylogCurrent {
                book: Some("changed book"),
                oracles: &[("foobar.oracle.sh", "changed oracle")],
            }),
        );
        assert_eq!(desync.diagnostics[0].code.slug(), "whylog-book-desync");
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
        let write = WhylogV2Write::new(&metadata, &records);
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
        assert_eq!(replay.records, direct);
    }

    #[test]
    fn v2_refuses_v1_and_any_trailing_or_reordered_terminal_bytes() {
        let doc = v2_doc();
        assert!(matches!(
            admit_unscoped_whylog(
                serialize(&WhylogDoc::default()).as_bytes(),
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
        });
        let mut non_increasing = v2_doc();
        non_increasing.apply = vec![
            ApplyLine {
                leaf: 1,
                disposition: "replace".to_owned(),
                predicted: true,
            },
            ApplyLine {
                leaf: 0,
                disposition: "run".to_owned(),
                predicted: true,
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
        assert_eq!(replay.records, direct);

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
