//! `plan::records` — the `dorc-records/1` wire framing (`262` §2, as REWRITTEN per
//! `notes/26A` stop-1; landed by `270:wire-records-v1-import`). The emission side lives in
//! [`crate::render`]; this module owns the shared vocabulary (the terminal token + framing
//! tags), the framing DATA the controller edge supplies ([`Framing`]/[`Nonce`]/[`Expect`]),
//! the header/sentinel emitters, and the temporary legacy deframer ([`deframe`]) used until
//! checkpoint 3C migrates the current CLI to the bounded admission API.
//!
//! # Why a framing layer at all (`262` §2 / `26A` stop-1 — the safety inversion)
//!
//! The inner record grammar (`site N effect=… rc=…`, `deriv N coord=…`) is unbounded tool
//! output on a best-effort pipe: a `>PIPE_BUF` line can tear, and a coordinate can carry
//! spaces. Without a per-line token a torn record's leading fragment parses as a *valid*
//! record with a truncated coordinate ⇒ wrong disjointness ⇒ wrong survival — the
//! under-execution cardinal sin re-entering through the wire. So EVERY line carries the
//! terminal token (tear-detector), free-content fields run LAST-TO-TOKEN (the incumbent
//! whitespace-truncation fix), and the deriv *at-most* family closes with an end-record so a
//! mid-family cut folds ⊤/wall-total rather than a smaller (more-licensing) claim.
//!
//! # Legacy tolerance (a spike scope-cut — `churn-avoidance-disclosure`)
//!
//! A stream with NO terminal token anywhere is treated as UNFRAMED and passed through
//! leniently (the pre-framing parse). This keeps the ~128 authored (header-free)
//! `probe-results.txt` fixtures green without re-authoring each with a matching
//! nonce/book-digest. It is SAFE because a real `dorc` probe ALWAYS frames: a headerless
//! stream can only come from a hand-authored fixture or a non-dorc source, never from a
//! mis-plumbed peer host (whose framed records carry its own `host=` and get refused). The
//! full contract (integrity keys, torn/glued/alien/late, sentinel) rides the FRAMED path,
//! exercised end-to-end by the `sweep` byte-tier DST and the plan/cli unit pins.

use dorc_aid::diag::{
    Diag, DiagCode, HostEvidenceAdmissionRefused, HostEvidenceRefusalKind, RecordsAlienLine,
    RecordsFactTruncated, RecordsGluedLine, RecordsHeaderMismatch, RecordsHeaderMissing,
    RecordsHeaderlessRefused, RecordsIntegrityRefused, RecordsLateLine, RecordsSentinelNonce,
    RecordsTornLine,
};
use std::collections::BTreeMap;
use std::io::Read;
use std::ops::Range;

/// The per-record terminal token (`262` §2 / `26A` stop-1). Requirements: fixed, never
/// produced by the inner grammar, cheap to append in one `printf`. `@@dorc@@` is printable
/// (no control bytes in goldens/SyncThing), distinctive (the structured grammar never emits
/// `@@`), single-`printf`-literal-cheap, and free of shell/sed metacharacters. A free-content
/// field (coord/stdout) that coincidentally *contains* these bytes is the documented
/// README-class limitation (the glued-detector then refuses the read unit — the safe
/// direction, fold-to-run).
pub const TERMINAL_TOKEN: &str = "@@dorc@@";

/// The header line's leading tag: `dorc-records/1 nonce=… attempt=… host=… book=… sites=N`.
const HEADER_TAG: &str = "dorc-records/1";

/// The end-sentinel's leading tag: `dorc-records-end/1 nonce=…` (`262` §2 — emitted after the
/// artifact's final `wait`; the drain never keys on EOF).
const SENTINEL_TAG: &str = "dorc-records-end/1";

/// The per-attempt run nonce (`262` §2): minted at the controller edge and DI'd — never
/// ambient RNG in kernel code (`inv-determinism`). The spike's cli edge uses a fixed default
/// so goldens stay deterministic; the DST supplies varying nonces to exercise mismatch
/// refusal. Records carry it as a bare line prefix; the header/sentinel carry it as a
/// `nonce=` key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nonce(pub String);

/// The spike's fixed default nonce. Deterministic ⇒ stable goldens; a real controller mints a
/// fresh per-attempt nonce at the edge (the anti-zombie mechanism the DST exercises).
pub const DEFAULT_NONCE: &str = "dorc";

/// The spike's fixed default `HostId` (single-host width-1). A real fleet supplies the session's
/// resolved host.
pub const DEFAULT_HOST: &str = "localhost";

impl Nonce {
    #[must_use]
    pub fn spike_default() -> Self {
        Self(DEFAULT_NONCE.to_owned())
    }
}

impl Framing {
    /// The FIXTURE framing: fixed nonce/host/attempt + the supplied book digest.
    ///
    /// Deterministic, and therefore usable only where determinism is worth more than identity —
    /// goldens, unit fixtures, and the hostless local run whose bytes those goldens pin. It is
    /// structurally unable to reach a managed host because reaching one requires a
    /// [`RemoteIdentity`], which this constructor does not produce and cannot be turned into
    /// (`rul-fixture-identity-never-production`: the fence is the absent constructor, not a
    /// comment). `spike_identity_is_not_reachable_from_transport` pins that lexically as well.
    #[must_use]
    pub fn spike(book_digest: String) -> Self {
        Self {
            nonce: Nonce::spike_default(),
            attempt: 1,
            host: DEFAULT_HOST.to_owned(),
            book_digest,
        }
    }

    /// The framing for a run that will really contact `identity`'s host.
    ///
    /// Every key here is minted from immutable controller-owned invocation context — the
    /// destination the admin typed, a per-process nonce, and an attempt counter this edge owns
    /// (`rul-attribution-is-controller-minted`). That is what gives [`Expect`] something to
    /// check: while all three were compile-time constants, a `host=` comparison was a constant
    /// against itself and the partition law had no wire tripwire at all.
    #[must_use]
    pub fn for_remote(identity: &RemoteIdentity, book_digest: String) -> Self {
        Self {
            nonce: identity.nonce.clone(),
            attempt: identity.attempt,
            host: identity.host.clone(),
            book_digest,
        }
    }

    /// The run nonce.
    #[must_use]
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }

    /// Which attempt at this artifact this is.
    #[must_use]
    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    /// The session's destination.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The digest binding this stream to the analyzed book bytes.
    #[must_use]
    pub fn book_digest(&self) -> &str {
        &self.book_digest
    }

    /// The [`Expect`] a deframer checks incoming records against — the SAME edge values this
    /// framing emitted.
    #[must_use]
    pub fn expect(&self) -> Expect {
        Expect {
            nonce: self.nonce.clone(),
            attempt: self.attempt,
            host: self.host.clone(),
            book_digest: self.book_digest.clone(),
        }
    }
}

/// The controller-owned identity of one attempt against one real host.
///
/// Exists to be the thing a fixture cannot forge its way into: [`Framing::for_remote`] is the
/// only route to a framing that names a real destination, and it consumes one of these. Minted
/// at the cli edge from the invocation, never from anything a host said
/// (`rul-attribution-is-controller-minted`).
#[derive(Debug, Clone)]
pub struct RemoteIdentity {
    nonce: Nonce,
    attempt: u32,
    host: String,
}

impl RemoteIdentity {
    /// Mint the identity for one attempt against `host`.
    #[must_use]
    pub fn new(nonce: Nonce, attempt: u32, host: String) -> Self {
        Self {
            nonce,
            attempt,
            host,
        }
    }

    /// The run nonce, which the transport marker must agree with.
    #[must_use]
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }

    /// Which attempt this is.
    #[must_use]
    pub fn attempt(&self) -> u32 {
        self.attempt
    }
}

/// The framing the emission bakes into the probe artifact (`262` §2). `sites` (the fact-lane
/// census) is derived at emit from the site-record count, not carried here.
///
/// Fields are private: a framing is minted by [`Framing::spike`] (deterministic, hostless) or
/// [`Framing::for_remote`] (controller-minted, host-bound), and there is deliberately no third
/// way to assemble one field by field.
#[derive(Debug, Clone)]
pub struct Framing {
    nonce: Nonce,
    /// Per-attempt keying (`26A` amend-retry-hygiene): a retry re-mints, so a killed attempt's
    /// zombie writer cannot fold into its successor.
    attempt: u32,
    /// The session's `HostId` (`262` §2 — the partition law's wire tripwire).
    host: String,
    /// A digest binding the stream to the exact analyzed book bytes (`262` §2 `book=`;
    /// discharges `tc-probe-no-digest`). SHA-256, hand-rolled to keep the kernel
    /// dependency-clean (`inv-determinism`) — see [`crate::invocation::book_digest`], which
    /// is the one substitution point. It answers "are these the same bytes", not "did someone
    /// authorized produce them": still not a keyed MAC and not a signature.
    book_digest: String,
}

/// What the deframer expects the incoming framed stream to declare (`262` §2 integrity keys).
/// The cli builds it from the SAME edge values it emitted; a mismatch on any key ⇒ refuse
/// fold (a mis-plumbed/stale/wrong-book stream).
#[derive(Debug, Clone)]
pub struct Expect {
    pub nonce: Nonce,
    pub attempt: u32,
    pub host: String,
    pub book_digest: String,
}

/// The header printf line (the artifact's FIRST output when run). `sites` is the resolvable
/// site-record census (fact-lane truncation range, `26A` amend-smalls).
#[must_use]
pub fn header_line(f: &Framing, sites: usize) -> String {
    format!(
        "printf '{tag} nonce={n} attempt={a} host={h} book={b} sites={s} {tok}\\n'\n",
        tag = HEADER_TAG,
        n = f.nonce.0,
        a = f.attempt,
        h = f.host,
        b = f.book_digest,
        s = sites,
        tok = TERMINAL_TOKEN,
    )
}

/// The header line a stream must carry for `framing`, up to the `sites=` census only its writer can
/// count.
///
/// Public so a REFUSAL can name it verbatim (`28L:rul-refusals-name-the-next-command`): the author
/// of a committed fixture stream who has just edited their book needs the new digest, and that is
/// exactly the byte a framing mismatch is usually about. It is spelled HERE, beside
/// [`header_line`] and the parser, because a refusal that derived the expected bytes independently
/// could drift from the bytes actually demanded.
#[must_use]
pub fn expected_header_prefix(f: &Framing) -> String {
    format!(
        "{tag} nonce={n} attempt={a} host={h} book={b}",
        tag = HEADER_TAG,
        n = f.nonce.0,
        a = f.attempt,
        h = f.host,
        b = f.book_digest,
    )
}

/// The end-sentinel printf line (the artifact's LAST output). Emitted after every record lane;
/// the drain recognizes end-of-stream by THIS line, never by EOF (`notes/141` g5).
#[must_use]
pub fn sentinel_line(nonce: &Nonce) -> String {
    format!(
        "printf '{tag} nonce={n} {tok}\\n'\n",
        tag = SENTINEL_TAG,
        n = nonce.0,
        tok = TERMINAL_TOKEN,
    )
}

/// Wrap one record's INNER printf-format body (`site {key} effect=%s rc=%s`) into a framed
/// format body: `{nonce} {inner} {token}` — the bare-nonce prefix + the terminal token, both
/// literals baked into the printf format (the `%s` placeholders stay in `inner`). The caller
/// adds the surrounding `printf '…\n' args`.
#[must_use]
pub fn frame(nonce: &Nonce, inner: &str) -> String {
    format!("{} {inner} {TERMINAL_TOKEN}", nonce.0)
}

/// The result of deframing a raw probe-results stream.
#[derive(Debug, Default)]
pub struct Deframed {
    /// The clean INNER record lines (nonce prefix + terminal token stripped), in arrival
    /// order, ready for the inner parser. A free-content field now runs to end-of-line (the
    /// token was the boundary — last-to-token), so the inner parser reads `coord=`/`stdout=`
    /// as rest-of-line.
    pub records: Vec<String>,
    /// Accumulated diagnostics (torn/alien/late aggregates, integrity mismatches, truncation
    /// range). Data, never a panic (`inv-no-throw`).
    pub diagnostics: Vec<Diag>,
    /// The whole read unit is refused (fold everything to Unknown ⇒ run): a header integrity
    /// mismatch (book/host/attempt/nonce) or a glued line (`262` §2 — reject the read unit).
    pub refused: bool,
    /// Was this stream FRAMED (a terminal token present)? The at-most deriv-family completeness
    /// gate (`26A` stop-1: a family with no `deriv-end` ⇒ wall-total) fires only here: the
    /// legacy (unframed, hand-authored) fixtures carry no end-records and are trusted-complete.
    pub framed: bool,
}

/// Whether the deframer tolerates a headerless (unframed) stream (`27D` errand-E4 /
/// `disposition-legacy-deframe-tolerance`). The lenient legacy passthrough is a
/// harness/test-only escape: a truncated-before-header stream carries no terminal token, so
/// under [`Tolerate`](LegacyPolicy::Tolerate) it would bypass EVERY integrity key — the
/// production hole stage-5 flagged. Production reads pass [`Refuse`](LegacyPolicy::Refuse):
/// a headerless stream refuses the read unit (kFAIL-withhold — no records fold ⇒ every site
/// runs). The cli edge reads the escape from the environment (`io-at-edges-only`); the kernel
/// stays a pure function of this parameter (`inv-determinism` — no env read inside `plan`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyPolicy {
    /// Strict (production): a headerless stream refuses the read unit.
    Refuse,
    /// Lenient (harness/test-only): a headerless stream passes through the pre-framing parse.
    Tolerate,
}

/// Deframe a raw probe-results stream into clean inner records + diagnostics (`262` §2, the
/// PRODUCTION deframer). Pure and total (`inv-no-throw`): malformed bytes are DATA.
///
/// Regime is chosen by terminal-token PRESENCE (robust to a torn header): any occurrence of
/// [`TERMINAL_TOKEN`] ⇒ the FRAMED contract; none ⇒ the legacy passthrough (see the module
/// doc's scope-cut), but ONLY under [`LegacyPolicy::Tolerate`] — the strict production path
/// ([`LegacyPolicy::Refuse`]) refuses a headerless stream instead of bypassing the integrity
/// keys (`27D` errand-E4).
#[must_use]
pub fn deframe(raw: &str, expect: &Expect, legacy: LegacyPolicy) -> Deframed {
    if raw.contains(TERMINAL_TOKEN) {
        deframe_framed(raw, expect)
    } else {
        match legacy {
            LegacyPolicy::Tolerate => deframe_legacy(raw),
            LegacyPolicy::Refuse => deframe_headerless_refused(),
        }
    }
}

/// The strict production disposition for a headerless (tokenless) stream (`27D` errand-E4): a
/// truncated-before-header artifact, or a non-dorc source, carries no framing at all. Refuse the
/// whole read unit — every site folds Unknown ⇒ the host runs (kFAIL-withhold). This is what
/// closes the `disposition-legacy-deframe-tolerance` hole: a real `dorc` probe ALWAYS frames,
/// so on the production path a headerless stream can only be corruption or an alien source.
fn deframe_headerless_refused() -> Deframed {
    let mut out = Deframed {
        refused: true,
        ..Deframed::default()
    };
    out.diagnostics
        .push(Diag::new_spanless_site(DiagCode::RecordsHeaderlessRefused(
            RecordsHeaderlessRefused,
        )));
    out
}

/// The legacy (unframed) passthrough: non-blank, non-`#` lines pass straight to the inner
/// parser, exactly as the pre-framing `parse_results` consumed stdin. No integrity, no
/// token — the free-content last-to-token fix still applies (the inner parser reads
/// rest-of-line), so an unframed space-bearing coordinate parses correctly too.
fn deframe_legacy(raw: &str) -> Deframed {
    let mut out = Deframed::default();
    for line in raw.lines() {
        let line = line.trim_end_matches('\r').trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        out.records.push(line.to_owned());
    }
    out
}

/// The framed contract (`262` §2 / `26A` stop-1). Every line must end with the token; the
/// header integrity keys are checked; records carry the bare-nonce prefix; the sentinel ends
/// the stream; torn/glued/alien/late lines are diagnosed (aggregated), never folded.
/// Running counts across a framed stream's lines — finalized into diagnostics after the walk.
#[derive(Default)]
struct Tally {
    torn: usize,
    alien: usize,
    late: usize,
    header_seen: bool,
    sentinel_seen: bool,
    declared_sites: Option<usize>,
    site_records: usize,
}

fn deframe_framed(raw: &str, expect: &Expect) -> Deframed {
    let mut out = Deframed {
        framed: true,
        ..Deframed::default()
    };
    let mut t = Tally::default();
    let nonce_prefix = format!("{} ", expect.nonce.0);

    for line in raw.lines() {
        let line = line.trim_end_matches('\r');
        let trimmed = line.trim();
        // Provenance comments (the probe's own `# site …` echo) and blanks are inert.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        // Token presence is the first cut. A line without it is TORN (a fragment that lost
        // its terminating write) if it looks like ours, else ALIEN (freeform leakage).
        let Some(body) = strip_terminal(trimmed) else {
            if trimmed.starts_with(&nonce_prefix)
                || trimmed.starts_with(HEADER_TAG)
                || trimmed.starts_with(SENTINEL_TAG)
            {
                t.torn += 1;
            } else {
                t.alien += 1;
            }
            continue;
        };
        // A residual token in the body means bytes preceded the terminal one: two atomic
        // writes glued (or a content-embedded token — the documented limitation). Reject the
        // whole read unit (`262` §2), the safe direction.
        if body.contains(TERMINAL_TOKEN) {
            out.refused = true;
            out.diagnostics
                .push(Diag::new_spanless_site(DiagCode::RecordsGluedLine(
                    RecordsGluedLine,
                )));
            continue;
        }

        if let Some(rest) = body.strip_prefix(&format!("{HEADER_TAG} ")) {
            t.header_seen = true;
            t.declared_sites = read_header(rest, expect, &mut out);
            continue;
        }
        if let Some(rest) = body.strip_prefix(&format!("{SENTINEL_TAG} ")) {
            t.sentinel_seen = true;
            if header_nonce_key(rest).is_some_and(|n| n != expect.nonce.0) {
                out.diagnostics
                    .push(Diag::new_spanless_site(DiagCode::RecordsSentinelNonce(
                        RecordsSentinelNonce,
                    )));
            }
            continue;
        }
        // A record: it must carry THIS attempt's bare-nonce prefix. A different prefix is a
        // zombie writer / stale-attempt leak (`26A` amend-retry-hygiene) ⇒ alien.
        if let Some(inner) = body.strip_prefix(&nonce_prefix) {
            if t.sentinel_seen {
                t.late += 1;
                continue;
            }
            if inner.starts_with("site ") {
                t.site_records += 1;
            }
            out.records.push(inner.to_owned());
        } else {
            t.alien += 1;
        }
    }

    finalize(&mut out, &t);
    out
}

/// Turn the stream's [`Tally`] into diagnostics (`262` §1/§2): a missing header refuses the
/// read unit; `sites=` truncation is a computable range (not a refusal — the unseen sites fold
/// Unknown ⇒ run on their own, `plans/128` fc-2); torn/alien/late get one aggregated warning
/// each (pin-late-and-alien-records — counted, never folded).
fn finalize(out: &mut Deframed, t: &Tally) {
    if !t.header_seen {
        out.refused = true;
        out.diagnostics
            .push(Diag::new_spanless_site(DiagCode::RecordsHeaderMissing(
                RecordsHeaderMissing,
            )));
    }
    if let Some(declared) = t.declared_sites
        && t.site_records < declared
    {
        out.diagnostics
            .push(Diag::new_spanless_site(DiagCode::RecordsFactTruncated(
                RecordsFactTruncated {
                    received: t.site_records,
                    declared,
                    unseen: declared - t.site_records,
                },
            )));
    }
    // Each aggregate is spelled with a LITERAL `new_spanless_site(DiagCode::…(` so the diag_tidy
    // spanless-mint gate's needle-scan sees it (a variable-built code would be invisible — t-4).
    if t.torn > 0 {
        out.diagnostics
            .push(Diag::new_spanless_site(DiagCode::RecordsTornLine(
                RecordsTornLine { count: t.torn },
            )));
    }
    if t.alien > 0 {
        out.diagnostics
            .push(Diag::new_spanless_site(DiagCode::RecordsAlienLine(
                RecordsAlienLine { count: t.alien },
            )));
    }
    if t.late > 0 {
        out.diagnostics
            .push(Diag::new_spanless_site(DiagCode::RecordsLateLine(
                RecordsLateLine { count: t.late },
            )));
    }
}

/// Strip the trailing ` {token}` (one separator space + the token). Returns the body without
/// it, or `None` when the line does not end with the token. The separator-space strip is what
/// makes a free-content field round-trip byte-exactly: the emit adds exactly one space before
/// the token, so removing exactly one restores the content's own trailing bytes.
fn strip_terminal(line: &str) -> Option<&str> {
    let body = line.strip_suffix(TERMINAL_TOKEN)?;
    Some(body.strip_suffix(' ').unwrap_or(body))
}

/// Read + integrity-check a header body (`nonce=… attempt=… host=… book=… sites=N`). Unknown
/// keys are ignored (additive-keys, `24Kc`). Any known-key mismatch ⇒ refuse the read unit.
/// Returns the declared `sites=` count when parseable.
fn read_header(rest: &str, expect: &Expect, out: &mut Deframed) -> Option<usize> {
    let mut sites = None;
    let mut mismatch = None;
    for tok in rest.split_whitespace() {
        if let Some(v) = tok.strip_prefix("nonce=") {
            if v != expect.nonce.0 {
                mismatch = Some(RecordsHeaderMismatch::Nonce);
            }
        } else if let Some(v) = tok.strip_prefix("attempt=") {
            if v != expect.attempt.to_string() {
                mismatch = Some(RecordsHeaderMismatch::Attempt);
            }
        } else if let Some(v) = tok.strip_prefix("host=") {
            if v != expect.host {
                mismatch = Some(RecordsHeaderMismatch::Host);
            }
        } else if let Some(v) = tok.strip_prefix("book=") {
            if v != expect.book_digest {
                mismatch = Some(RecordsHeaderMismatch::Book);
            }
        } else if let Some(v) = tok.strip_prefix("sites=") {
            sites = v.parse::<usize>().ok();
        }
    }
    if let Some(which) = mismatch {
        out.refused = true;
        out.diagnostics
            .push(Diag::new_spanless_site(DiagCode::RecordsIntegrityRefused(
                RecordsIntegrityRefused { which },
            )));
    }
    sites
}

/// The `nonce=` value from a sentinel body, if present (additive-keys tolerant).
fn header_nonce_key(rest: &str) -> Option<&str> {
    rest.split_whitespace()
        .find_map(|t| t.strip_prefix("nonce="))
}

/// Resource ceilings for hostile host-result bytes. Construction keeps the policy explicit at
/// the controller boundary while the parser remains deterministic and allocation-bounded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostEvidenceLimits {
    stream_bytes: usize,
    line_bytes: usize,
    records: usize,
    field_bytes: usize,
    retained_bytes: usize,
    collection_entries: usize,
    numeric_digits: usize,
}

impl HostEvidenceLimits {
    /// The deliberately conservative width-one spike policy.
    #[must_use]
    pub const fn spike_default() -> Self {
        Self {
            stream_bytes: 8 * 1024 * 1024,
            line_bytes: 64 * 1024,
            records: 65_536,
            field_bytes: 16 * 1024,
            retained_bytes: 4 * 1024 * 1024,
            collection_entries: 32_768,
            numeric_digits: 16,
        }
    }

    /// Supply a test or embedding policy without exposing mutable limit fields.
    #[must_use]
    pub const fn new(
        stream_bytes: usize,
        line_bytes: usize,
        records: usize,
        field_bytes: usize,
        retained_bytes: usize,
        collection_entries: usize,
        numeric_digits: usize,
    ) -> Self {
        Self {
            stream_bytes,
            line_bytes,
            records,
            field_bytes,
            retained_bytes,
            collection_entries,
            numeric_digits,
        }
    }

    pub(crate) const fn stream_bytes(self) -> usize {
        self.stream_bytes
    }
}

/// Bytes admitted under the stream ceiling, before text or record ownership exists.
#[derive(Debug)]
pub struct BoundedHostBytes {
    backing: Vec<u8>,
    admitted: Range<usize>,
}

impl BoundedHostBytes {
    /// Keep a single checked range over controller-owned backing for a nested durable.
    pub(crate) fn from_owned_backing(backing: Vec<u8>) -> Self {
        let end = backing.len();
        Self {
            backing,
            admitted: 0..end,
        }
    }

    /// Narrows the admitted view without reconstructing or copying the backing bytes.
    pub(crate) fn with_admitted_range(mut self, admitted: Range<usize>) -> Option<Self> {
        if admitted.start > admitted.end || admitted.end > self.backing.len() {
            return None;
        }
        self.admitted = admitted;
        Some(self)
    }

    fn admitted_bytes(&self) -> Option<&[u8]> {
        self.backing.get(self.admitted.clone())
    }
}

/// A closed reason why evidence cannot enter the decision plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissionRefusal {
    IncompatibleVersion,
    StreamLimit,
    LineLimit,
    InvalidUtf8,
    ControlByte,
    Framing,
    Grammar,
    Numeric,
    RecordLimit,
    FieldLimit,
    RetainedLimit,
    CollectionLimit,
    Duplicate,
    ArithmeticOverflow,
}

impl AdmissionRefusal {
    /// Keeps malformed ingress outside source spans.
    #[must_use]
    pub fn spanless_diagnostic(self) -> Diag {
        Diag::new_spanless_site(DiagCode::HostEvidenceAdmissionRefused(
            HostEvidenceAdmissionRefused { kind: self.kind() },
        ))
    }

    fn kind(self) -> HostEvidenceRefusalKind {
        match self {
            Self::IncompatibleVersion => HostEvidenceRefusalKind::IncompatibleVersion,
            Self::StreamLimit => HostEvidenceRefusalKind::StreamLimit,
            Self::LineLimit => HostEvidenceRefusalKind::LineLimit,
            Self::InvalidUtf8 => HostEvidenceRefusalKind::InvalidUtf8,
            Self::ControlByte => HostEvidenceRefusalKind::ControlByte,
            Self::Framing => HostEvidenceRefusalKind::Framing,
            Self::Grammar => HostEvidenceRefusalKind::Grammar,
            Self::Numeric => HostEvidenceRefusalKind::Numeric,
            Self::RecordLimit => HostEvidenceRefusalKind::RecordLimit,
            Self::FieldLimit => HostEvidenceRefusalKind::FieldLimit,
            Self::RetainedLimit => HostEvidenceRefusalKind::RetainedLimit,
            Self::CollectionLimit => HostEvidenceRefusalKind::CollectionLimit,
            Self::Duplicate => HostEvidenceRefusalKind::Duplicate,
            Self::ArithmeticOverflow => HostEvidenceRefusalKind::ArithmeticOverflow,
        }
    }
}

/// The only three outcomes for host evidence admission.
#[derive(Debug)]
pub enum Admission<T> {
    Admitted(T),
    NoObservation,
    Refused(AdmissionRefusal),
}

/// Read at most `limit + 1` bytes before decoding or allocating record text.
pub fn read_host_evidence<R: Read>(
    reader: R,
    limits: HostEvidenceLimits,
) -> Admission<BoundedHostBytes> {
    let Some(read_limit) = limits.stream_bytes.checked_add(1) else {
        return Admission::Refused(AdmissionRefusal::ArithmeticOverflow);
    };
    let mut bytes = Vec::new();
    let mut limited = reader.take(read_limit as u64);
    if limited.read_to_end(&mut bytes).is_err() {
        return Admission::Refused(AdmissionRefusal::Framing);
    }
    if bytes.len() > limits.stream_bytes {
        return Admission::Refused(AdmissionRefusal::StreamLimit);
    }
    Admission::Admitted(BoundedHostBytes::from_owned_backing(bytes))
}

/// Bounded wire records, unscoped and decision-inert until checkpoint 3C.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedUnscopedHostRecords {
    records: Vec<TypedHostRecord>,
    wire: Vec<u8>,
}

/// A borrowed, grammar-checked record. It remains unscoped until the controller binds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmittedHostRecord<'a> {
    Site {
        key: &'a str,
        effect: &'a str,
        rc: i32,
        stdout: Option<&'a str>,
        stderr: Option<&'a str>,
        inert: &'a [(String, String)],
    },
    Derivation {
        site: u32,
        coord: &'a str,
    },
    DerivationEnd {
        site: u32,
        count: u32,
        body_rc: u32,
    },
    Resolution {
        coord: &'a str,
        canonical: Option<&'a str>,
    },
    Reach {
        coord: &'a str,
        arm: usize,
        entity: &'a str,
    },
    ReachEnd {
        coord: &'a str,
        arm: usize,
        count: u32,
        body_rc: u32,
    },
    Report {
        body: &'a str,
    },
}

impl AdmittedUnscopedHostRecords {
    /// The exact bounded wire range, retained only for sibling durable serialization.
    pub(crate) fn admitted_wire_bytes(&self) -> &[u8] {
        &self.wire
    }
    /// Borrows validated records without exposing their owning representation or a raw byte route.
    pub fn iter(&self) -> impl Iterator<Item = AdmittedHostRecord<'_>> {
        self.records.iter().map(|record| match record {
            TypedHostRecord::Site {
                key,
                effect,
                rc,
                stdout,
                stderr,
                inert,
            } => AdmittedHostRecord::Site {
                key,
                effect,
                rc: *rc,
                stdout: stdout.as_deref(),
                stderr: stderr.as_deref(),
                inert,
            },
            TypedHostRecord::Derivation { site, coord } => {
                AdmittedHostRecord::Derivation { site: *site, coord }
            }
            TypedHostRecord::DerivationEnd {
                site,
                count,
                body_rc,
            } => AdmittedHostRecord::DerivationEnd {
                site: *site,
                count: *count,
                body_rc: *body_rc,
            },
            TypedHostRecord::Resolution { coord, canonical } => AdmittedHostRecord::Resolution {
                coord,
                canonical: canonical.as_deref(),
            },
            TypedHostRecord::Reach { coord, arm, entity } => AdmittedHostRecord::Reach {
                coord,
                arm: *arm,
                entity,
            },
            TypedHostRecord::ReachEnd {
                coord,
                arm,
                count,
                body_rc,
            } => AdmittedHostRecord::ReachEnd {
                coord,
                arm: *arm,
                count: *count,
                body_rc: *body_rc,
            },
            TypedHostRecord::Report { body } => AdmittedHostRecord::Report { body },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypedHostRecord {
    Site {
        key: String,
        effect: String,
        rc: i32,
        stdout: Option<String>,
        stderr: Option<String>,
        inert: Vec<(String, String)>,
    },
    Derivation {
        site: u32,
        coord: String,
    },
    DerivationEnd {
        site: u32,
        count: u32,
        body_rc: u32,
    },
    Resolution {
        coord: String,
        canonical: Option<String>,
    },
    Reach {
        coord: String,
        arm: usize,
        entity: String,
    },
    ReachEnd {
        coord: String,
        arm: usize,
        count: u32,
        body_rc: u32,
    },
    Report {
        body: String,
    },
}

/// Compare bounded bytes with expected framing; a match grants no attribution.
#[must_use]
pub fn admit_unscoped_host_records(
    bytes: &BoundedHostBytes,
    expected: &Framing,
    limits: HostEvidenceLimits,
) -> Admission<AdmittedUnscopedHostRecords> {
    let Some(bytes) = bytes.admitted_bytes() else {
        return Admission::Refused(AdmissionRefusal::ArithmeticOverflow);
    };
    admit_records(bytes, expected, limits)
}

#[expect(
    clippy::too_many_lines,
    reason = "the ordered framing state machine keeps every refusal before ownership in one auditable boundary"
)]
fn admit_records(
    bytes: &[u8],
    expected: &Framing,
    limits: HostEvidenceLimits,
) -> Admission<AdmittedUnscopedHostRecords> {
    if bytes.is_empty() || std::str::from_utf8(bytes).is_err() {
        return Admission::Refused(AdmissionRefusal::InvalidUtf8);
    }
    if !bytes.ends_with(b"\n") {
        return Admission::Refused(AdmissionRefusal::Framing);
    }
    let lines = bytes.split_inclusive(|byte| *byte == b'\n');
    let mut header = false;
    let mut sentinel = false;
    let mut declared_sites = None;
    let mut received_sites = 0usize;
    let mut retained = 0usize;
    let mut records = Vec::new();
    let mut identities = BTreeMap::new();
    let mut collections = CollectionCounts::default();
    let mut physical_records = 0usize;
    for raw in lines {
        let line = raw.strip_suffix(b"\n").unwrap_or(raw);
        if line.len() > limits.line_bytes {
            return Admission::Refused(AdmissionRefusal::LineLimit);
        }
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        if line
            .iter()
            .any(|byte| *byte == 0 || (*byte < 0x20 && *byte != b'\r'))
        {
            return Admission::Refused(AdmissionRefusal::ControlByte);
        }
        let Ok(line) = std::str::from_utf8(line) else {
            return Admission::Refused(AdmissionRefusal::InvalidUtf8);
        };
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !line.ends_with(TERMINAL_TOKEN) {
            return Admission::Refused(AdmissionRefusal::Framing);
        }
        let Some(body) = line
            .strip_suffix(TERMINAL_TOKEN)
            .and_then(|s| s.strip_suffix(' '))
        else {
            return Admission::Refused(AdmissionRefusal::Framing);
        };
        if body.contains(TERMINAL_TOKEN) {
            return Admission::Refused(AdmissionRefusal::Framing);
        }
        if let Some(rest) = body.strip_prefix(&format!("{HEADER_TAG} ")) {
            if header || sentinel || !records.is_empty() {
                return Admission::Refused(AdmissionRefusal::Framing);
            }
            declared_sites = match parse_header(rest, expected, limits) {
                Ok(sites) => sites,
                Err(refusal) => return Admission::Refused(refusal),
            };
            header = true;
            continue;
        }
        if let Some(rest) = body.strip_prefix(&format!("{SENTINEL_TAG} ")) {
            if !header || sentinel || rest != format!("nonce={}", expected.nonce.0) {
                return Admission::Refused(AdmissionRefusal::Framing);
            }
            sentinel = true;
            continue;
        }
        if !header || sentinel {
            return Admission::Refused(AdmissionRefusal::Framing);
        }
        let Some(record) = body
            .strip_prefix(&expected.nonce.0)
            .and_then(|rest| rest.strip_prefix(' '))
        else {
            return Admission::Refused(AdmissionRefusal::Framing);
        };
        physical_records = match physical_records.checked_add(1) {
            Some(total) => total,
            None => return Admission::Refused(AdmissionRefusal::ArithmeticOverflow),
        };
        if physical_records > limits.records {
            return Admission::Refused(AdmissionRefusal::RecordLimit);
        }
        let parsed = match parse_record(record, limits) {
            Ok(record) => record,
            Err(refusal) => return Admission::Refused(refusal),
        };
        if matches!(parsed, ParsedRecord::Site(_)) {
            received_sites = match received_sites.checked_add(1) {
                Some(total) => total,
                None => return Admission::Refused(AdmissionRefusal::ArithmeticOverflow),
            };
        }
        retained = match charge_retained(retained, &parsed, limits) {
            Ok(total) => total,
            Err(refusal) => return Admission::Refused(refusal),
        };
        let identity = parsed.identity();
        if let Some(existing) = identities.get(&identity) {
            if *existing != parsed {
                return Admission::Refused(AdmissionRefusal::Duplicate);
            }
            continue;
        }
        if !collections.can_insert(parsed.collection(), limits.collection_entries) {
            return Admission::Refused(AdmissionRefusal::CollectionLimit);
        }
        collections.insert(parsed.collection());
        let owned = parsed.to_owned();
        identities.insert(identity, parsed);
        records.push(owned);
    }
    if !header || !sentinel {
        return Admission::Refused(AdmissionRefusal::Framing);
    }
    if declared_sites != Some(received_sites) {
        return Admission::Refused(AdmissionRefusal::Framing);
    }
    if records.is_empty() {
        return Admission::NoObservation;
    }
    Admission::Admitted(AdmittedUnscopedHostRecords {
        records,
        wire: bytes.to_vec(),
    })
}

fn parse_header(
    rest: &str,
    expected: &Framing,
    limits: HostEvidenceLimits,
) -> Result<Option<usize>, AdmissionRefusal> {
    let mut nonce = None;
    let mut attempt = None;
    let mut host = None;
    let mut book = None;
    let mut sites = None;
    for token in rest.split_whitespace() {
        let Some((key, value)) = token.split_once('=') else {
            return Err(AdmissionRefusal::Grammar);
        };
        match key {
            "nonce" if nonce.replace(value).is_none() => {}
            "attempt" if attempt.replace(number_u32(value, limits)?).is_none() => {}
            "host" if host.replace(value).is_none() => {}
            "book" if book.replace(value).is_none() => {}
            "sites" if sites.replace(number(value, limits)?).is_none() => {}
            _ => return Err(AdmissionRefusal::Grammar),
        }
    }
    if nonce != Some(expected.nonce.0.as_str())
        || attempt != Some(expected.attempt)
        || host != Some(expected.host.as_str())
        || book != Some(expected.book_digest.as_str())
    {
        return Err(AdmissionRefusal::Framing);
    }
    sites.map(Some).ok_or(AdmissionRefusal::Grammar)
}

fn number(token: &str, limits: HostEvidenceLimits) -> Result<usize, AdmissionRefusal> {
    if token.is_empty()
        || token.len() > limits.numeric_digits
        || !token.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(AdmissionRefusal::Numeric);
    }
    token
        .parse::<usize>()
        .map_err(|_| AdmissionRefusal::Numeric)
}

fn number_u32(token: &str, limits: HostEvidenceLimits) -> Result<u32, AdmissionRefusal> {
    u32::try_from(number(token, limits)?).map_err(|_| AdmissionRefusal::Numeric)
}

fn parse_record(
    record: &str,
    limits: HostEvidenceLimits,
) -> Result<ParsedRecord<'_>, AdmissionRefusal> {
    let Some((tag, rest)) = record.split_once(' ') else {
        return Err(AdmissionRefusal::Grammar);
    };
    match tag {
        "site" => parse_site(rest, limits).map(ParsedRecord::Site),
        "deriv" => {
            let (site, coord) = rest
                .split_once(" coord=")
                .ok_or(AdmissionRefusal::Grammar)?;
            checked_field(coord, limits)?;
            Ok(ParsedRecord::Derivation {
                site: number_u32(site, limits)?,
                coord,
            })
        }
        "deriv-end" => {
            let (site, tail) = rest.split_once(" n=").ok_or(AdmissionRefusal::Grammar)?;
            let (count, body_rc) = tail
                .split_once(" body-rc=")
                .ok_or(AdmissionRefusal::Grammar)?;
            Ok(ParsedRecord::DerivationEnd {
                site: number_u32(site, limits)?,
                count: number_u32(count, limits)?,
                body_rc: number_u32(body_rc, limits)?,
            })
        }
        "resolv" => {
            let (coord, outcome) = rest.split_once(' ').ok_or(AdmissionRefusal::Grammar)?;
            checked_field(coord, limits)?;
            let canonical = match outcome {
                "dangling" => None,
                value if value.starts_with("canon=") => {
                    let value = &value[6..];
                    checked_field(value, limits)?;
                    Some(value)
                }
                _ => return Err(AdmissionRefusal::Grammar),
            };
            Ok(ParsedRecord::Resolution { coord, canonical })
        }
        "reach" => {
            let (head, entity) = rest
                .split_once(" entity=")
                .ok_or(AdmissionRefusal::Grammar)?;
            let mut words = head.split_whitespace();
            let coord = words.next().ok_or(AdmissionRefusal::Grammar)?;
            let arm = words
                .next()
                .and_then(|word| word.strip_prefix("arm="))
                .ok_or(AdmissionRefusal::Grammar)?;
            if words.next().is_some() {
                return Err(AdmissionRefusal::Grammar);
            }
            checked_field(coord, limits)?;
            checked_field(entity, limits)?;
            Ok(ParsedRecord::Reach {
                coord,
                arm: number(arm, limits)?,
                entity,
            })
        }
        "reach-end" => {
            let mut words = rest.split_whitespace();
            let coord = words.next().ok_or(AdmissionRefusal::Grammar)?;
            let arm = words
                .next()
                .and_then(|word| word.strip_prefix("arm="))
                .ok_or(AdmissionRefusal::Grammar)?;
            let count = words
                .next()
                .and_then(|word| word.strip_prefix("n="))
                .ok_or(AdmissionRefusal::Grammar)?;
            let body_rc = words
                .next()
                .and_then(|word| word.strip_prefix("body-rc="))
                .ok_or(AdmissionRefusal::Grammar)?;
            if words.next().is_some() {
                return Err(AdmissionRefusal::Grammar);
            }
            checked_field(coord, limits)?;
            Ok(ParsedRecord::ReachEnd {
                coord,
                arm: number(arm, limits)?,
                count: number_u32(count, limits)?,
                body_rc: number_u32(body_rc, limits)?,
            })
        }
        "report" => {
            checked_field(rest, limits)?;
            Ok(ParsedRecord::Report { body: rest })
        }
        _ => Err(AdmissionRefusal::Grammar),
    }
}

fn site_key(token: &str, limits: HostEvidenceLimits) -> bool {
    let mut parts = token.split('.');
    let Some(site) = parts.next() else {
        return false;
    };
    let member = parts.next();
    parts.next().is_none()
        && number(site, limits).is_ok()
        && member.is_none_or(|member| number(member, limits).is_ok())
}

fn parse_i32(token: &str, limits: HostEvidenceLimits) -> Result<i32, AdmissionRefusal> {
    let digits = token.strip_prefix('-').unwrap_or(token);
    if digits.len() > limits.numeric_digits
        || digits.is_empty()
        || !digits.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(AdmissionRefusal::Numeric);
    }
    token.parse().map_err(|_| AdmissionRefusal::Numeric)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SiteBorrowed<'a> {
    key: &'a str,
    effect: &'a str,
    rc: i32,
    stdout: Option<&'a str>,
    stderr: Option<&'a str>,
    inert: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParsedRecord<'a> {
    Site(SiteBorrowed<'a>),
    Derivation {
        site: u32,
        coord: &'a str,
    },
    DerivationEnd {
        site: u32,
        count: u32,
        body_rc: u32,
    },
    Resolution {
        coord: &'a str,
        canonical: Option<&'a str>,
    },
    Reach {
        coord: &'a str,
        arm: usize,
        entity: &'a str,
    },
    ReachEnd {
        coord: &'a str,
        arm: usize,
        count: u32,
        body_rc: u32,
    },
    Report {
        body: &'a str,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RecordIdentity<'a> {
    Site(&'a str),
    Derivation(u32, &'a str),
    DerivationEnd(u32),
    Resolution(&'a str),
    Reach(&'a str, usize),
    ReachEnd(&'a str, usize),
    Report(&'a str),
}

#[derive(Clone, Copy)]
enum Collection {
    Site,
    Derivation,
    DerivationEnd,
    Resolution,
    Reach,
    ReachEnd,
    Report,
}

#[derive(Default)]
struct CollectionCounts {
    site: usize,
    derivation: usize,
    derivation_end: usize,
    resolution: usize,
    reach: usize,
    reach_end: usize,
    report: usize,
}

impl CollectionCounts {
    fn can_insert(&self, collection: Collection, limit: usize) -> bool {
        match collection {
            Collection::Site => self.site < limit,
            Collection::Derivation => self.derivation < limit,
            Collection::DerivationEnd => self.derivation_end < limit,
            Collection::Resolution => self.resolution < limit,
            Collection::Reach => self.reach < limit,
            Collection::ReachEnd => self.reach_end < limit,
            Collection::Report => self.report < limit,
        }
    }

    fn insert(&mut self, collection: Collection) {
        match collection {
            Collection::Site => self.site += 1,
            Collection::Derivation => self.derivation += 1,
            Collection::DerivationEnd => self.derivation_end += 1,
            Collection::Resolution => self.resolution += 1,
            Collection::Reach => self.reach += 1,
            Collection::ReachEnd => self.reach_end += 1,
            Collection::Report => self.report += 1,
        }
    }
}

impl<'a> ParsedRecord<'a> {
    fn identity(self) -> RecordIdentity<'a> {
        match self {
            Self::Site(site) => RecordIdentity::Site(site.key),
            Self::Derivation { site, coord } => RecordIdentity::Derivation(site, coord),
            Self::DerivationEnd { site, .. } => RecordIdentity::DerivationEnd(site),
            Self::Resolution { coord, .. } => RecordIdentity::Resolution(coord),
            Self::Reach { coord, arm, .. } => RecordIdentity::Reach(coord, arm),
            Self::ReachEnd { coord, arm, .. } => RecordIdentity::ReachEnd(coord, arm),
            Self::Report { body } => RecordIdentity::Report(body),
        }
    }

    fn collection(self) -> Collection {
        match self {
            Self::Site(_) => Collection::Site,
            Self::Derivation { .. } => Collection::Derivation,
            Self::DerivationEnd { .. } => Collection::DerivationEnd,
            Self::Resolution { .. } => Collection::Resolution,
            Self::Reach { .. } => Collection::Reach,
            Self::ReachEnd { .. } => Collection::ReachEnd,
            Self::Report { .. } => Collection::Report,
        }
    }

    fn to_owned(self) -> TypedHostRecord {
        match self {
            Self::Site(site) => TypedHostRecord::Site {
                key: site.key.to_owned(),
                effect: site.effect.to_owned(),
                rc: site.rc,
                stdout: site.stdout.map(str::to_owned),
                stderr: site.stderr.map(str::to_owned),
                inert: inert_fields(site.inert)
                    .map(|(name, value)| (name.to_owned(), value.to_owned()))
                    .collect(),
            },
            Self::Derivation { site, coord } => TypedHostRecord::Derivation {
                site,
                coord: coord.to_owned(),
            },
            Self::DerivationEnd {
                site,
                count,
                body_rc,
            } => TypedHostRecord::DerivationEnd {
                site,
                count,
                body_rc,
            },
            Self::Resolution { coord, canonical } => TypedHostRecord::Resolution {
                coord: coord.to_owned(),
                canonical: canonical.map(str::to_owned),
            },
            Self::Reach { coord, arm, entity } => TypedHostRecord::Reach {
                coord: coord.to_owned(),
                arm,
                entity: entity.to_owned(),
            },
            Self::ReachEnd {
                coord,
                arm,
                count,
                body_rc,
            } => TypedHostRecord::ReachEnd {
                coord: coord.to_owned(),
                arm,
                count,
                body_rc,
            },
            Self::Report { body } => TypedHostRecord::Report {
                body: body.to_owned(),
            },
        }
    }
}

fn parse_site(
    rest: &str,
    limits: HostEvidenceLimits,
) -> Result<SiteBorrowed<'_>, AdmissionRefusal> {
    let Some((key, fields)) = rest.split_once(' ') else {
        return Err(AdmissionRefusal::Grammar);
    };
    if !site_key(key, limits) {
        return Err(AdmissionRefusal::Grammar);
    }
    checked_field(key, limits)?;
    let mut effect = None;
    let mut rc = None;
    let mut stdout = None;
    let mut stderr = None;
    let mut offset = 0;
    let mut inert_end = fields.len();
    while offset < fields.len() {
        let remaining = &fields[offset..];
        let trimmed = remaining.trim_start_matches(char::is_whitespace);
        offset += remaining.len() - trimmed.len();
        if trimmed.is_empty() {
            break;
        }
        let word_end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let word = &trimmed[..word_end];
        let Some((name, value)) = word.split_once('=') else {
            return Err(AdmissionRefusal::Grammar);
        };
        match name {
            "effect" => {
                if effect.is_some() || !matches!(value, "holds" | "absent" | "cant-tell") {
                    return Err(AdmissionRefusal::Grammar);
                }
                effect = Some(value);
                checked_field(value, limits)?;
            }
            "rc" => {
                if rc.is_some() {
                    return Err(AdmissionRefusal::Grammar);
                }
                rc = Some(parse_i32(value, limits)?);
            }
            "stdout" | "stderr" => {
                if (name == "stdout" && stdout.is_some()) || (name == "stderr" && stderr.is_some())
                {
                    return Err(AdmissionRefusal::Grammar);
                }
                let free = &trimmed[name.len() + 1..];
                if free.is_empty()
                    || free.split_whitespace().skip(1).any(|later| {
                        matches!(
                            later.split_once('=').map(|(key, _)| key),
                            Some("effect" | "rc" | "stdout" | "stderr")
                        )
                    })
                {
                    return Err(AdmissionRefusal::Grammar);
                }
                checked_field(free, limits)?;
                if name == "stdout" {
                    stdout = Some(free);
                } else {
                    stderr = Some(free);
                }
                inert_end = offset;
                break;
            }
            _ if !name.is_empty() && !value.is_empty() => {
                checked_field(name, limits)?;
                checked_field(value, limits)?;
            }
            _ => return Err(AdmissionRefusal::Grammar),
        }
        offset += word_end;
    }
    Ok(SiteBorrowed {
        key,
        effect: effect.ok_or(AdmissionRefusal::Grammar)?,
        rc: rc.ok_or(AdmissionRefusal::Grammar)?,
        stdout,
        stderr,
        inert: &fields[..inert_end],
    })
}

fn inert_fields(fields: &str) -> impl Iterator<Item = (&str, &str)> {
    fields.split_whitespace().filter_map(|word| {
        let (name, value) = word.split_once('=')?;
        (!matches!(name, "effect" | "rc" | "stdout" | "stderr")).then_some((name, value))
    })
}

fn charge_retained(
    retained: usize,
    record: &ParsedRecord<'_>,
    limits: HostEvidenceLimits,
) -> Result<usize, AdmissionRefusal> {
    let mut total = retained;
    let mut charge = |field: &str| {
        checked_field(field, limits)?;
        total = total
            .checked_add(field.len())
            .ok_or(AdmissionRefusal::ArithmeticOverflow)?;
        (total <= limits.retained_bytes)
            .then_some(())
            .ok_or(AdmissionRefusal::RetainedLimit)
    };
    match record {
        ParsedRecord::Site(site) => {
            charge(site.key)?;
            charge(site.effect)?;
            if let Some(stdout) = site.stdout {
                charge(stdout)?;
            }
            if let Some(stderr) = site.stderr {
                charge(stderr)?;
            }
            for (name, value) in inert_fields(site.inert) {
                charge(name)?;
                charge(value)?;
            }
        }
        ParsedRecord::Derivation { coord, .. } | ParsedRecord::ReachEnd { coord, .. } => {
            charge(coord)?;
        }
        ParsedRecord::DerivationEnd { .. } => {}
        ParsedRecord::Resolution { coord, canonical } => {
            charge(coord)?;
            if let Some(canonical) = canonical {
                charge(canonical)?;
            }
        }
        ParsedRecord::Reach { coord, entity, .. } => {
            charge(coord)?;
            charge(entity)?;
        }
        ParsedRecord::Report { body } => charge(body)?,
    }
    Ok(total)
}

fn checked_field(value: &str, limits: HostEvidenceLimits) -> Result<(), AdmissionRefusal> {
    if value.len() > limits.field_bytes {
        return Err(AdmissionRefusal::FieldLimit);
    }
    if value.is_empty() {
        return Err(AdmissionRefusal::Grammar);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    type RecordFactory = fn(usize) -> String;
    type FieldFactory = fn(&str) -> String;

    /// The default expectation the emitter's spike framing produces (nonce `dorc`, host
    /// `localhost`, attempt 1, book `bk`).
    fn expect() -> Expect {
        Framing::spike("bk".to_owned()).expect()
    }

    /// One framed record line (`{nonce} {inner} {token}`).
    fn rec(inner: &str) -> String {
        format!("{DEFAULT_NONCE} {inner} {TERMINAL_TOKEN}")
    }

    /// A well-formed header line for a stream declaring `sites` fact records.
    fn header(sites: usize) -> String {
        header_line(&Framing::spike("bk".to_owned()), sites)
            .trim_end()
            .trim_start_matches("printf '")
            .trim_end_matches("\\n'")
            .to_owned()
    }

    /// The well-formed end-sentinel line (with a trailing newline).
    fn sentinel() -> String {
        format!("{SENTINEL_TAG} nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n")
    }

    /// Assemble a framed stream from a header + record inners + sentinel.
    fn stream(sites: usize, inners: &[&str]) -> String {
        let recs = inners
            .iter()
            .map(|i| format!("{}\n", rec(i)))
            .collect::<Vec<_>>()
            .concat();
        format!("{}\n{recs}{}", header(sites), sentinel())
    }

    #[test]
    fn legacy_headerless_tolerated_passes_through_dropping_comments_and_blanks() {
        // E4 the TOLERATE direction (`27D` disposition-legacy-deframe-tolerance): no terminal
        // token anywhere + the harness/test escape ⇒ the legacy regime: non-blank, non-# lines
        // pass to the inner parser verbatim (the ~128 authored fixtures ride this via run.sh's
        // `DORC_ALLOW_LEGACY_RESULTS`).
        let d = deframe(
            "# a comment\nsite 0 effect=holds rc=0\n\nsite 1 effect=absent rc=1\n",
            &expect(),
            LegacyPolicy::Tolerate,
        );
        assert!(!d.framed && !d.refused);
        assert_eq!(
            d.records,
            vec!["site 0 effect=holds rc=0", "site 1 effect=absent rc=1"]
        );
    }

    #[test]
    fn strict_headerless_refuses_fold_no_records() {
        // E4 the REFUSE direction (`27D` disposition-legacy-deframe-tolerance — the production
        // fix): the SAME headerless stream on the strict path refuses the whole read unit. A
        // truncated-before-header artifact would otherwise bypass every integrity key; strict
        // refuses ⇒ no records fold ⇒ every site folds Unknown ⇒ the host runs (kFAIL-withhold).
        let d = deframe(
            "# a comment\nsite 0 effect=holds rc=0\n\nsite 1 effect=absent rc=1\n",
            &expect(),
            LegacyPolicy::Refuse,
        );
        assert!(
            d.refused,
            "a headerless stream is refused on the strict production path"
        );
        assert!(d.records.is_empty(), "a refused read unit folds no records");
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-headerless-refused"),
            "the refusal carries the registered code: {:?}",
            d.diagnostics
        );
    }

    #[test]
    fn framed_strips_nonce_prefix_and_terminal_token() {
        let d = deframe(
            &stream(1, &["site 0 effect=holds rc=0"]),
            &expect(),
            LegacyPolicy::Refuse,
        );
        assert!(d.framed && !d.refused, "diags: {:?}", d.diagnostics);
        assert_eq!(d.records, vec!["site 0 effect=holds rc=0"]);
    }

    #[test]
    fn coord_free_content_keeps_embedded_spaces_to_the_token() {
        // `262` §2 last-to-token: the deframer strips exactly ` {token}`, so a coordinate with
        // embedded spaces survives byte-exactly — the incumbent whitespace-truncation fix, and
        // the `279f` stdout-rider generalization. (The inner parser then reads to end-of-line.)
        let d = deframe(
            &stream(0, &["deriv 0 coord=/etc/a file/with spaces"]),
            &expect(),
            LegacyPolicy::Refuse,
        );
        assert_eq!(d.records, vec!["deriv 0 coord=/etc/a file/with spaces"]);
    }

    #[test]
    fn torn_line_no_token_counted_never_folded() {
        // A nonce-prefixed line missing the terminal token is a torn fragment (a lost
        // terminating write): dropped + counted, never folded (`26A` stop-1).
        let s = format!(
            "{}\n{DEFAULT_NONCE} site 0 effect=holds rc=0\n{}", // middle line has no token ⇒ torn
            header(1),
            sentinel()
        );
        let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
        assert!(d.records.is_empty(), "the torn fragment never folds");
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-torn-line"),
            "torn is counted + warned"
        );
    }

    #[test]
    fn glued_line_bytes_after_token_refuses_read_unit() {
        // Two atomic writes merged on one line (`…{token}…{token}`) ⇒ bytes after the first
        // token ⇒ reject the WHOLE read unit (`262` §2), the safe direction.
        let s = format!(
            "{}\n{DEFAULT_NONCE} site 0 effect=holds rc=0 {TERMINAL_TOKEN}{DEFAULT_NONCE} site 1 effect=absent rc=1 {TERMINAL_TOKEN}\n{}",
            header(1),
            sentinel()
        );
        let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
        assert!(d.refused, "a glued read unit is refused");
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-glued-line")
        );
    }

    #[test]
    fn alien_non_nonce_line_counted() {
        let alien = format!("garbage leakage {TERMINAL_TOKEN}\n{SENTINEL_TAG}");
        let d = deframe(
            &stream(1, &["site 0 effect=holds rc=0"]).replace(SENTINEL_TAG, &alien),
            &expect(),
            LegacyPolicy::Refuse,
        );
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-alien-line"),
            "a non-nonce framed line is alien: {:?}",
            d.diagnostics
        );
        assert_eq!(d.records, vec!["site 0 effect=holds rc=0"]);
    }

    #[test]
    fn late_record_after_sentinel_discarded() {
        let mut s = stream(1, &["site 0 effect=holds rc=0"]);
        s.push_str(&rec("site 1 effect=absent rc=1")); // AFTER the sentinel ⇒ late
        s.push('\n');
        let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
        assert_eq!(
            d.records,
            vec!["site 0 effect=holds rc=0"],
            "late is dropped"
        );
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-late-line")
        );
    }

    #[test]
    fn integrity_mismatch_refuses_each_key() {
        // book / host / attempt / nonce mismatch each refuses the whole read unit.
        for (bad_key, bad_val) in [
            ("book", "WRONG"),
            ("host", "otherhost"),
            ("attempt", "2"),
            ("nonce", "stale"),
        ] {
            let hdr = header(1).replace(
                &format!("{bad_key}="),
                &format!("{bad_key}={bad_val}~"), // corrupt just this key's value
            );
            let s = format!(
                "{hdr}\n{}\n{SENTINEL_TAG} nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n",
                rec("site 0 effect=holds rc=0")
            );
            let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
            assert!(d.refused, "{bad_key} mismatch must refuse the fold");
        }
    }

    #[test]
    fn missing_header_refuses_even_with_records() {
        // A framed stream (has tokens) but no `dorc-records/1` header (torn/absent) ⇒ refuse.
        let s = format!(
            "{}\n{SENTINEL_TAG} nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n",
            rec("site 0 effect=holds rc=0")
        );
        let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
        assert!(d.refused);
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-header-missing")
        );
    }

    #[test]
    fn additive_unknown_keys_ignored_in_header_and_record() {
        // `24Kc` additive-keys: an unknown `ms=…`/`future=…` key is ignored, never a refusal.
        // The extra header key rides BEFORE the terminal token (a real emitter would append it
        // as a normal field).
        let hdr = header(1).replace(TERMINAL_TOKEN, &format!("ms=42 {TERMINAL_TOKEN}"));
        let s = format!(
            "{hdr}\n{}\n{SENTINEL_TAG} nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n",
            rec("site 0 future=x effect=holds rc=0")
        );
        let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
        assert!(!d.refused, "unknown keys are additive, not fatal");
        assert_eq!(d.records, vec!["site 0 future=x effect=holds rc=0"]);
    }

    #[test]
    fn sites_truncation_is_a_computable_range_note_not_refusal() {
        // Declared sites=3 but only 1 site record ⇒ a NOTE (the 2 unseen fold Unknown ⇒ run),
        // never a refusal (`26A` amend-smalls).
        let d = deframe(
            &stream(3, &["site 0 effect=holds rc=0"]),
            &expect(),
            LegacyPolicy::Refuse,
        );
        assert!(!d.refused);
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-fact-truncated"),
            "truncation is a computable range: {:?}",
            d.diagnostics
        );
    }

    #[test]
    fn wrong_nonce_prefix_record_is_alien_zombie_writer() {
        // A record carrying a DIFFERENT nonce prefix (a stale-attempt / zombie writer leak) is
        // un-foldable ⇒ alien, never folded under this attempt's key (`26A` amend-retry-hygiene).
        let s = format!(
            "{}\nzombie site 0 effect=holds rc=0 {TERMINAL_TOKEN}\n{}",
            header(1),
            sentinel()
        );
        let d = deframe(&s, &expect(), LegacyPolicy::Refuse);
        assert!(d.records.is_empty(), "a stale-nonce record never folds");
        assert!(
            d.diagnostics
                .iter()
                .any(|x| x.code.slug() == "records-alien-line")
        );
    }

    fn admitted(raw: &str, limits: HostEvidenceLimits) -> Admission<AdmittedUnscopedHostRecords> {
        let framing = Framing::spike("bk".to_owned());
        let bytes = match read_host_evidence(raw.as_bytes(), limits) {
            Admission::Admitted(bytes) => bytes,
            other => panic!("test input must pass the byte reader: {other:?}"),
        };
        admit_unscoped_host_records(&bytes, &framing, limits)
    }

    fn strict_limits() -> HostEvidenceLimits {
        HostEvidenceLimits::new(4096, 512, 8, 64, 512, 8, 4)
    }

    fn strict_stream(inners: &[&str]) -> String {
        stream(
            inners
                .iter()
                .filter(|line| line.starts_with("site "))
                .count(),
            inners,
        )
    }

    #[test]
    fn admission_refusal_diagnostic_mapping_is_exhaustive_and_closed() {
        for (refusal, kind) in [
            (
                AdmissionRefusal::IncompatibleVersion,
                HostEvidenceRefusalKind::IncompatibleVersion,
            ),
            (
                AdmissionRefusal::StreamLimit,
                HostEvidenceRefusalKind::StreamLimit,
            ),
            (
                AdmissionRefusal::LineLimit,
                HostEvidenceRefusalKind::LineLimit,
            ),
            (
                AdmissionRefusal::InvalidUtf8,
                HostEvidenceRefusalKind::InvalidUtf8,
            ),
            (
                AdmissionRefusal::ControlByte,
                HostEvidenceRefusalKind::ControlByte,
            ),
            (AdmissionRefusal::Framing, HostEvidenceRefusalKind::Framing),
            (AdmissionRefusal::Grammar, HostEvidenceRefusalKind::Grammar),
            (AdmissionRefusal::Numeric, HostEvidenceRefusalKind::Numeric),
            (
                AdmissionRefusal::RecordLimit,
                HostEvidenceRefusalKind::RecordLimit,
            ),
            (
                AdmissionRefusal::FieldLimit,
                HostEvidenceRefusalKind::FieldLimit,
            ),
            (
                AdmissionRefusal::RetainedLimit,
                HostEvidenceRefusalKind::RetainedLimit,
            ),
            (
                AdmissionRefusal::CollectionLimit,
                HostEvidenceRefusalKind::CollectionLimit,
            ),
            (
                AdmissionRefusal::Duplicate,
                HostEvidenceRefusalKind::Duplicate,
            ),
            (
                AdmissionRefusal::ArithmeticOverflow,
                HostEvidenceRefusalKind::ArithmeticOverflow,
            ),
        ] {
            assert_eq!(
                refusal.spanless_diagnostic().code,
                DiagCode::HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused { kind })
            );
            assert_eq!(refusal.spanless_diagnostic().primary.span(), None);
        }
    }

    #[test]
    fn bounded_reader_refuses_only_the_stream_plus_one_byte() {
        let exact = HostEvidenceLimits::new(3, 32, 8, 32, 32, 8, 4);
        assert!(matches!(
            read_host_evidence(&b"abc"[..], exact),
            Admission::Admitted(_)
        ));
        assert!(matches!(
            read_host_evidence(&b"abcd"[..], exact),
            Admission::Refused(AdmissionRefusal::StreamLimit)
        ));
    }

    #[test]
    fn width_one_admission_distinguishes_empty_observation_from_headerless_bytes() {
        let empty = strict_stream(&[]);
        assert!(matches!(
            admitted(&empty, strict_limits()),
            Admission::NoObservation
        ));
        let framing = Framing::spike("bk".to_owned());
        let Admission::Admitted(bytes) =
            read_host_evidence(&b"site 0 effect=holds rc=0\n"[..], strict_limits())
        else {
            panic!("the bounded reader must accept the short test input");
        };
        assert!(matches!(
            admit_unscoped_host_records(&bytes, &framing, strict_limits()),
            Admission::Refused(AdmissionRefusal::Framing)
        ));
    }

    #[test]
    fn report_only_zero_site_frame_remains_admitted() {
        let report = strict_stream(&["report site=0 decline unsound host detail"]);
        let Admission::Admitted(records) = admitted(&report, strict_limits()) else {
            panic!("a valid report-only frame must preserve its decision-inert record");
        };
        assert!(matches!(
            records.iter().next(),
            Some(AdmittedHostRecord::Report { body }) if body == "site=0 decline unsound host detail"
        ));
    }

    #[test]
    fn admission_rejects_bad_text_controls_and_forged_framing_before_record_ownership() {
        let invalid = b"\xff";
        assert!(matches!(
            read_host_evidence(&invalid[..], strict_limits()),
            Admission::Admitted(_)
        ));
        let framing = Framing::spike("bk".to_owned());
        let Admission::Admitted(bytes) = read_host_evidence(&invalid[..], strict_limits()) else {
            panic!("the bounded reader must retain invalid UTF-8 for admission");
        };
        assert!(matches!(
            admit_unscoped_host_records(&bytes, &framing, strict_limits()),
            Admission::Refused(AdmissionRefusal::InvalidUtf8)
        ));
        let control = strict_stream(&["report bad\u{0000}"]);
        assert!(matches!(
            admitted(&control, strict_limits()),
            Admission::Refused(AdmissionRefusal::ControlByte)
        ));
        let forged =
            strict_stream(&["site 0 effect=holds rc=0"]).replace("host=localhost", "host=forged");
        assert!(matches!(
            admitted(&forged, strict_limits()),
            Admission::Refused(AdmissionRefusal::Framing)
        ));
    }

    #[test]
    fn admission_bounds_lines_records_fields_retention_and_collections() {
        let long = strict_stream(&[
            "report xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        ]);
        let line_limited = HostEvidenceLimits::new(4096, 32, 8, 64, 512, 8, 4);
        assert!(matches!(
            admitted(&long, line_limited),
            Admission::Refused(AdmissionRefusal::LineLimit)
        ));
        let field_limited = HostEvidenceLimits::new(4096, 512, 8, 4, 512, 8, 4);
        assert!(matches!(
            admitted(&strict_stream(&["report abcde"]), field_limited),
            Admission::Refused(AdmissionRefusal::FieldLimit)
        ));
        let record_limited = HostEvidenceLimits::new(4096, 512, 1, 64, 512, 8, 4);
        assert!(matches!(
            admitted(
                &strict_stream(&["report one", "report two"]),
                record_limited
            ),
            Admission::Refused(AdmissionRefusal::RecordLimit)
        ));
        let retained_limited = HostEvidenceLimits::new(4096, 512, 8, 64, 3, 8, 4);
        assert!(matches!(
            admitted(&strict_stream(&["report four"]), retained_limited),
            Admission::Refused(AdmissionRefusal::RetainedLimit)
        ));
        let collection_limited = HostEvidenceLimits::new(4096, 512, 8, 64, 512, 1, 4);
        assert!(matches!(
            admitted(
                &strict_stream(&["report one", "report two"]),
                collection_limited
            ),
            Admission::Refused(AdmissionRefusal::CollectionLimit)
        ));
    }

    #[test]
    fn admission_rejects_numeric_duplicate_and_framing_ambiguity() {
        assert!(matches!(
            admitted(
                &strict_stream(&["site 0 effect=holds rc=99999"]),
                strict_limits()
            ),
            Admission::Refused(AdmissionRefusal::Numeric)
        ));
        assert!(matches!(
            admitted(
                &strict_stream(&["site 0 effect=holds rc=0", "site 0 effect=holds rc=0"]),
                strict_limits()
            ),
            Admission::Admitted(_)
        ));
        let repeated = format!("{}{}", strict_stream(&[]), sentinel());
        assert!(matches!(
            admitted(&repeated, strict_limits()),
            Admission::Refused(AdmissionRefusal::Framing)
        ));
        let torn = strict_stream(&[]).replace(TERMINAL_TOKEN, "");
        assert!(matches!(
            admitted(&torn, strict_limits()),
            Admission::Refused(AdmissionRefusal::Framing)
        ));
        assert!(matches!(
            admitted(&strict_stream(&["unknown field"]), strict_limits()),
            Admission::Refused(AdmissionRefusal::Grammar)
        ));
        assert!(matches!(
            admitted(
                &strict_stream(&["site 0 effect=holds rc=-1"]),
                strict_limits()
            ),
            Admission::Admitted(_)
        ));
        let overflow = HostEvidenceLimits::new(4096, 512, 8, 64, 512, 8, 16);
        assert!(matches!(
            admitted(
                &strict_stream(&["site 0 effect=holds rc=9999999999"]),
                overflow
            ),
            Admission::Refused(AdmissionRefusal::Numeric)
        ));
    }

    #[test]
    fn admission_retains_each_recognized_free_text_lane_after_validation() {
        let raw = strict_stream(&[
            "site 0 effect=holds rc=0 stdout=out with spaces",
            "deriv 0 coord=derived",
            "resolv source canon=canonical",
            "reach source arm=0 entity=reached",
            "report a bounded report",
        ]);
        let Admission::Admitted(evidence) = admitted(&raw, strict_limits()) else {
            panic!("all lanes must admit")
        };
        assert_eq!(evidence.records.len(), 5);
        assert!(matches!(evidence.records[0], TypedHostRecord::Site { .. }));
        assert!(matches!(
            evidence.records[1],
            TypedHostRecord::Derivation { .. }
        ));
        assert!(matches!(
            evidence.records[2],
            TypedHostRecord::Resolution { .. }
        ));
        assert!(matches!(evidence.records[3], TypedHostRecord::Reach { .. }));
        assert!(matches!(
            evidence.records[4],
            TypedHostRecord::Report { .. }
        ));
        let stderr = strict_stream(&["site 0 effect=holds rc=0 stderr=err with spaces"]);
        assert!(matches!(
            admitted(&stderr, strict_limits()),
            Admission::Admitted(_)
        ));
    }

    #[test]
    fn typed_lane_duplicates_deduplicate_only_when_equal_and_refuse_conflicts() {
        let exact = strict_stream(&["site 0 effect=holds rc=0", "site 0 effect=holds rc=0"]);
        let Admission::Admitted(records) = admitted(&exact, strict_limits()) else {
            panic!("an exact duplicate is an idempotent record after charging")
        };
        assert_eq!(records.records.len(), 1);

        for records in [
            strict_stream(&["site 0 effect=holds rc=0", "site 0 effect=absent rc=1"]),
            strict_stream(&["deriv-end 0 n=1 body-rc=0", "deriv-end 0 n=2 body-rc=0"]),
            strict_stream(&["resolv source canon=one", "resolv source canon=two"]),
            strict_stream(&[
                "reach source arm=0 entity=one",
                "reach source arm=0 entity=two",
            ]),
        ] {
            assert!(matches!(
                admitted(&records, strict_limits()),
                Admission::Refused(AdmissionRefusal::Duplicate)
            ));
        }
    }

    #[test]
    fn admission_is_unscoped_and_offers_no_record_extraction_api() {
        let source = include_str!("records.rs");
        for forbidden in [
            ["Width", "OneAttemptScope"].concat(),
            ["Scoped", "HostEvidence"].concat(),
            ["Oracle", "SourceIdentity"].concat(),
            ["admit_width_one", "_host_records"].concat(),
            ["pub fn ", "records"].concat(),
        ] {
            assert!(
                !source.contains(&forbidden),
                "forbidden authority API: {forbidden}"
            );
        }
    }

    #[test]
    fn physical_record_limit_counts_before_grammar_and_deduplication() {
        for (count, expected) in [(1, true), (2, true), (3, false)] {
            let records = vec!["site 0 effect=holds rc=0"; count];
            let raw = strict_stream(&records);
            assert_eq!(
                matches!(
                    admitted(&raw, HostEvidenceLimits::new(4096, 512, 2, 64, 512, 8, 4)),
                    Admission::Admitted(_)
                ),
                expected,
                "physical record count {count}"
            );
        }
        assert!(matches!(
            admitted(
                &strict_stream(&["site 0 effect=holds rc=0"]),
                HostEvidenceLimits::new(4096, 512, 0, 64, 512, 8, 4)
            ),
            Admission::Refused(AdmissionRefusal::RecordLimit)
        ));
        let duplicate_storm = strict_stream(&[
            "site 0 effect=holds rc=0",
            "site 0 effect=holds rc=0",
            "site 0 effect=holds rc=0",
        ]);
        assert!(matches!(
            admitted(
                &duplicate_storm,
                HostEvidenceLimits::new(4096, 512, 2, 64, 512, 8, 4)
            ),
            Admission::Refused(AdmissionRefusal::RecordLimit)
        ));
    }

    #[test]
    fn physical_lines_are_accounted_before_crlf_and_ignored_framing() {
        let mixed = format!(
            "# before header\r\n\r\n{}\r\n# before record\n{}\r\n\r\n{}# after sentinel\r\n",
            header(1),
            rec("site 0 effect=holds rc=0"),
            sentinel(),
        );
        assert!(matches!(
            admitted(&mixed, strict_limits()),
            Admission::Admitted(_)
        ));

        let oversized_comment = format!("# {}\n{}", "x".repeat(65), strict_stream(&[]));
        assert!(matches!(
            admitted(
                &oversized_comment,
                HostEvidenceLimits::new(4096, 64, 8, 64, 512, 8, 4)
            ),
            Admission::Refused(AdmissionRefusal::LineLimit)
        ));
        assert!(matches!(
            admitted(strict_stream(&[]).trim_end_matches('\n'), strict_limits()),
            Admission::Refused(AdmissionRefusal::Framing)
        ));
    }

    #[test]
    fn retained_budget_charges_only_owned_fields_before_materialization() {
        let structural = strict_stream(&["site 0 effect=holds rc=0", "report abc"]);
        assert!(matches!(
            admitted(
                &structural,
                HostEvidenceLimits::new(4096, 512, 8, 64, 9, 8, 4)
            ),
            Admission::Admitted(_)
        ));
        for limit in [8, 9, 10] {
            let result = admitted(
                &structural,
                HostEvidenceLimits::new(4096, 512, 8, 64, limit, 8, 4),
            );
            if limit == 8 {
                assert!(matches!(
                    result,
                    Admission::Refused(AdmissionRefusal::RetainedLimit)
                ));
            } else {
                assert!(
                    matches!(result, Admission::Admitted(_)),
                    "retained exact/plus {limit}: {result:?}"
                );
            }
        }
        assert_eq!(
            charge_retained(
                usize::MAX,
                &ParsedRecord::Report { body: "x" },
                strict_limits()
            ),
            Err(AdmissionRefusal::ArithmeticOverflow)
        );
    }

    #[test]
    fn site_fields_are_closed_and_only_one_final_free_field_is_allowed() {
        for record in [
            "site 0 effect=holds rc=0 stdout=x stderr=y",
            "site 0 effect=holds rc=0 stderr=y stdout=x",
            "site 0 effect=holds effect=absent rc=0",
            "site 0 effect=holds rc=0 rc=1",
            "site 0 effect=holds rc=0 stdout=x stdout=y",
            "site 0 effect=holds rc=0 stderr=x stderr=y",
        ] {
            assert!(
                matches!(
                    admitted(&strict_stream(&[record]), strict_limits()),
                    Admission::Refused(AdmissionRefusal::Grammar)
                ),
                "recognized site key duplication/coexistence: {record}"
            );
        }
        let additive = strict_stream(&[
            "site 0.1 future=one extra=two effect=holds rc=0 stdout=final free value",
        ]);
        let Admission::Admitted(records) = admitted(&additive, strict_limits()) else {
            panic!("additive structured fields before the final free field must admit")
        };
        let TypedHostRecord::Site { inert, stdout, .. } = &records.records[0] else {
            panic!("expected site")
        };
        assert_eq!(
            inert,
            &vec![
                ("future".to_owned(), "one".to_owned()),
                ("extra".to_owned(), "two".to_owned())
            ]
        );
        assert_eq!(stdout.as_deref(), Some("final free value"));
    }

    #[test]
    fn typed_collection_caps_are_independent_and_dedup_does_not_retain() {
        let limits = HostEvidenceLimits::new(4096, 512, 8, 64, 512, 1, 4);
        for records in [
            strict_stream(&["site 0 effect=holds rc=0", "site 1 effect=holds rc=0"]),
            strict_stream(&["deriv 0 coord=one", "deriv 1 coord=two"]),
            strict_stream(&["deriv-end 0 n=0 body-rc=0", "deriv-end 1 n=0 body-rc=0"]),
            strict_stream(&["resolv one dangling", "resolv two dangling"]),
            strict_stream(&["reach one arm=0 entity=x", "reach two arm=0 entity=x"]),
            strict_stream(&["report one", "report two"]),
        ] {
            assert!(matches!(
                admitted(&records, limits),
                Admission::Refused(AdmissionRefusal::CollectionLimit)
            ));
        }
        let exact_duplicates =
            strict_stream(&["site 0 effect=holds rc=0", "site 0 effect=holds rc=0"]);
        let Admission::Admitted(records) = admitted(&exact_duplicates, strict_limits()) else {
            panic!("exact duplicates retain one owned record")
        };
        assert_eq!(records.records.len(), 1);
        assert!(matches!(
            admitted(
                &exact_duplicates,
                HostEvidenceLimits::new(4096, 512, 8, 64, 11, 8, 4)
            ),
            Admission::Refused(AdmissionRefusal::RetainedLimit)
        ));
    }

    #[test]
    fn stream_and_line_limits_cover_exact_plus_one_and_overflow() {
        let defaults = HostEvidenceLimits::spike_default();
        assert!(matches!(
            read_host_evidence(vec![b'x'; 8 * 1024 * 1024].as_slice(), defaults),
            Admission::Admitted(_)
        ));
        assert!(matches!(
            read_host_evidence(vec![b'x'; 8 * 1024 * 1024 + 1].as_slice(), defaults),
            Admission::Refused(AdmissionRefusal::StreamLimit)
        ));
        assert!(matches!(
            read_host_evidence(
                &b"x"[..],
                HostEvidenceLimits::new(usize::MAX, 1, 1, 1, 1, 1, 1)
            ),
            Admission::Refused(AdmissionRefusal::ArithmeticOverflow)
        ));

        for (bytes, expected) in [(65_536, true), (65_537, false)] {
            let comment = format!("#{}\r\n{}", "x".repeat(bytes - 2), strict_stream(&[]));
            assert_eq!(
                matches!(
                    admitted(
                        &comment,
                        HostEvidenceLimits::new(128 * 1024, 65_536, 8, 64, 512, 8, 4)
                    ),
                    Admission::NoObservation
                ),
                expected,
                "CRLF physical line bytes={bytes}"
            );
        }
    }

    #[test]
    fn record_and_collection_boundaries_are_independent() {
        let limit = 32_768;
        let mut records = Vec::with_capacity(limit * 2 + 1);
        for index in 0..limit {
            records.push(format!("site {index} effect=holds rc=0"));
        }
        for index in 0..limit {
            records.push(format!("deriv {index} coord=d{index}"));
        }
        let borrowed = records.iter().map(String::as_str).collect::<Vec<_>>();
        let exact = stream(limit, &borrowed);
        let limits = HostEvidenceLimits::new(
            8 * 1024 * 1024,
            64 * 1024,
            65_536,
            16 * 1024,
            4 * 1024 * 1024,
            limit,
            16,
        );
        assert!(matches!(admitted(&exact, limits), Admission::Admitted(_)));
        records.push("report extra".to_owned());
        let borrowed = records.iter().map(String::as_str).collect::<Vec<_>>();
        assert!(matches!(
            admitted(&stream(limit, &borrowed), limits),
            Admission::Refused(AdmissionRefusal::RecordLimit)
        ));

        let cases: [(&str, RecordFactory); 7] = [
            ("site", |index| format!("site {index} effect=holds rc=0")),
            ("deriv", |index| format!("deriv {index} coord=d{index}")),
            ("deriv-end", |index| {
                format!("deriv-end {index} n=0 body-rc=0")
            }),
            ("resolv", |index| format!("resolv c{index} dangling")),
            ("reach", |index| {
                format!("reach c{index} arm=0 entity=e{index}")
            }),
            ("reach-end", |index| {
                format!("reach-end c{index} arm=0 n=0 body-rc=0")
            }),
            ("report", |index| format!("report r{index}")),
        ];
        for (name, make) in cases {
            let records = (0..=limit).map(make).collect::<Vec<_>>();
            let borrowed = records.iter().map(String::as_str).collect::<Vec<_>>();
            let exact = stream(if name == "site" { limit } else { 0 }, &borrowed[..limit]);
            let plus = stream(if name == "site" { limit + 1 } else { 0 }, &borrowed);
            assert!(
                matches!(
                    admitted(&exact, limits),
                    Admission::Admitted(_) | Admission::NoObservation
                ),
                "{name} exact"
            );
            assert!(
                matches!(
                    admitted(&plus, limits),
                    Admission::Refused(AdmissionRefusal::CollectionLimit)
                ),
                "{name} plus"
            );
        }
    }

    #[test]
    fn free_field_boundaries_cover_every_retained_lane() {
        let exact = "x".repeat(16 * 1024);
        let plus = "x".repeat(16 * 1024 + 1);
        let cases: [(&str, FieldFactory); 10] = [
            ("stdout", |value| {
                format!("site 0 effect=holds rc=0 stdout={value}")
            }),
            ("stderr", |value| {
                format!("site 0 effect=holds rc=0 stderr={value}")
            }),
            ("inert-name", |value| {
                format!("site 0 {value}=x effect=holds rc=0")
            }),
            ("inert-value", |value| {
                format!("site 0 field={value} effect=holds rc=0")
            }),
            ("deriv", |value| format!("deriv 0 coord={value}")),
            ("resolv-coord", |value| format!("resolv {value} dangling")),
            ("resolv-canonical", |value| {
                format!("resolv c canon={value}")
            }),
            ("reach-coord", |value| {
                format!("reach {value} arm=0 entity=e")
            }),
            ("reach-entity", |value| {
                format!("reach c arm=0 entity={value}")
            }),
            ("report", |value| format!("report {value}")),
        ];
        let limits = HostEvidenceLimits::new(
            8 * 1024 * 1024,
            64 * 1024,
            65_536,
            16 * 1024,
            4 * 1024 * 1024,
            32_768,
            16,
        );
        for (name, make) in cases {
            let exact_record = make(&exact);
            let plus_record = make(&plus);
            let sites = usize::from(exact_record.starts_with("site "));
            assert!(
                matches!(
                    admitted(&stream(sites, &[&exact_record]), limits),
                    Admission::Admitted(_) | Admission::NoObservation
                ),
                "{name} exact"
            );
            assert!(
                matches!(
                    admitted(&stream(sites, &[&plus_record]), limits),
                    Admission::Refused(AdmissionRefusal::FieldLimit)
                ),
                "{name} plus"
            );
        }
    }

    #[test]
    fn retained_and_numeric_boundaries_are_checked_before_ownership() {
        let report = format!("report {}", "x".repeat(16 * 1024));
        let records = vec![report.as_str(); 256];
        let limits = HostEvidenceLimits::new(
            8 * 1024 * 1024,
            64 * 1024,
            65_536,
            16 * 1024,
            4 * 1024 * 1024,
            32_768,
            16,
        );
        assert!(matches!(
            admitted(&stream(0, &records), limits),
            Admission::Admitted(_)
        ));
        let plus = vec![report.as_str(); 257];
        assert!(matches!(
            admitted(&stream(0, &plus), limits),
            Admission::Refused(AdmissionRefusal::RetainedLimit)
        ));

        let digits16 = "9".repeat(16);
        let digits17 = "9".repeat(17);
        let numeric_limits = HostEvidenceLimits::new(4096, 512, 8, 64, 512, 8, 16);
        assert!(number(&digits16, numeric_limits).is_ok());
        assert_eq!(
            number(&digits17, numeric_limits),
            Err(AdmissionRefusal::Numeric)
        );
        assert_eq!(
            number(
                &digits16,
                HostEvidenceLimits::new(4096, 512, 8, 64, 512, 8, 15)
            ),
            Err(AdmissionRefusal::Numeric)
        );
        assert_eq!(
            number_u32("4294967296", numeric_limits),
            Err(AdmissionRefusal::Numeric)
        );
        assert_eq!(
            parse_i32("2147483648", numeric_limits),
            Err(AdmissionRefusal::Numeric)
        );
        assert_eq!(
            parse_i32("-2147483649", numeric_limits),
            Err(AdmissionRefusal::Numeric)
        );
        assert!(number_u32("0000004294967295", numeric_limits).is_ok());
        assert_eq!(
            number_u32("00000004294967295", numeric_limits),
            Err(AdmissionRefusal::Numeric)
        );
        assert!(parse_i32("0000002147483647", numeric_limits).is_ok());
        assert_eq!(
            parse_i32("00000002147483647", numeric_limits),
            Err(AdmissionRefusal::Numeric)
        );
    }

    #[test]
    fn spike_default_limits_pin_the_frozen_policy() {
        assert_eq!(
            HostEvidenceLimits::spike_default(),
            HostEvidenceLimits::new(
                8 * 1024 * 1024,
                64 * 1024,
                65_536,
                16 * 1024,
                4 * 1024 * 1024,
                32_768,
                16,
            )
        );
    }

    #[test]
    fn identity_and_framing_matrix_refuse_ambiguity() {
        let exact = [
            "site 0 effect=holds rc=0",
            "deriv 0 coord=one",
            "deriv-end 0 n=1 body-rc=0",
            "resolv one dangling",
            "reach one arm=0 entity=e",
            "reach-end one arm=0 n=1 body-rc=0",
            "report note",
        ];
        for record in exact {
            let sites = usize::from(record.starts_with("site "));
            let duplicate = stream(sites * 2, &[record, record]);
            assert!(
                matches!(
                    admitted(&duplicate, strict_limits()),
                    Admission::Admitted(_) | Admission::NoObservation
                ),
                "exact {record}"
            );
        }
        for (left, right) in [
            ("site 0 effect=holds rc=0", "site 0 effect=absent rc=1"),
            ("deriv-end 0 n=0 body-rc=0", "deriv-end 0 n=1 body-rc=0"),
            ("resolv one dangling", "resolv one canon=two"),
            ("reach one arm=0 entity=e", "reach one arm=0 entity=f"),
            // A close is IDENTIFIED by (coord, arm) alone, so two disagreeing closes for one arm
            // are ambiguous rather than last-wins — the gate must never pick the permissive one.
            (
                "reach-end one arm=0 n=1 body-rc=0",
                "reach-end one arm=0 n=1 body-rc=127",
            ),
        ] {
            assert!(matches!(
                admitted(&stream(0, &[left, right]), strict_limits()),
                Admission::Refused(AdmissionRefusal::Duplicate)
            ));
        }
        assert!(matches!(
            admitted(
                &stream(0, &["deriv 0 coord=one", "deriv 0 coord=two"]),
                strict_limits()
            ),
            Admission::Admitted(_)
        ));

        for raw in [
            format!("{}\n{}", header(0), header(0)),
            format!("{}\n{}{}", header(0), sentinel(), sentinel()),
            format!(
                "{}\n{}\n{}",
                header(0),
                sentinel().trim_end(),
                rec("report late")
            ),
            "site 0 effect=holds rc=0\n".to_owned(),
            strict_stream(&["site 0 effect=unknown rc=0"]),
            strict_stream(&["site 0 effect=holds rc=0 bad"]),
            strict_stream(&["unknown x"]),
        ] {
            assert!(
                matches!(admitted(&raw, strict_limits()), Admission::Refused(_)),
                "{raw:?}"
            );
        }
        let valid = format!(
            "\r\n# before\r\n{}\r\n\r\n{}\r\n# after\r\n",
            header(0),
            sentinel()
        );
        assert!(matches!(
            admitted(&valid, strict_limits()),
            Admission::NoObservation
        ));
    }
}
