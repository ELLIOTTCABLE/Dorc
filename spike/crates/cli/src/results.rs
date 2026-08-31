//! The controller↔host INTAKE edge: bounded bytes in, an attributed [`SiteResults`] out.
//!
//! This is the segment `rul-attribution-is-controller-minted` governs — host, target, attempt,
//! oracle-source-set and generation identity minted from immutable controller-owned invocation
//! context, attached to accepted records, and carried by [`ScopedHostEvidence`] through every
//! conversion. It sits on the loom seam (`lib-target-is-a-loom-seam`) because it is a pure function
//! of already-read bytes: opening the file, reading stdin, and reading the clock all stay in
//! `main.rs`, and only VALUES cross.
//!
//! Two entry points, deliberately not one ([`admit_controller_records`] and
//! [`admit_fixture_records`]); read the latter's doc for why the split is the fence.

use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::{CollapseKind, CollapseNarrative, SpeechAct};
use dorc_core::influence::{InfluenceAccount, InfluencePhase, Influenced};
use dorc_core::{Interner, Observable, OutBytes, Predicted, ProvArena, Rc, Verdict};
use dorc_plan::invocation::book_digest;
use dorc_plan::records::{
    Admission, AdmittedUnscopedHostRecords, BoundedHostBytes, Framing, HostEvidenceLimits,
    admit_unscoped_host_records, read_host_evidence,
};

/// The run's instant source — the DI seam for wall clock (`io-at-edges-only`). It lives HERE, in
/// the binary, and nowhere else: the analyzer kernel owns no clock type at all, so no kernel
/// signature can accept one and no kernel path can "reach for a clock to help". Only
/// [`dorc_core::RunInstant`] values (already read) cross inward.
///
/// Nondeterminism enters ONCE, at [`system`](RunClock::system) — the single wall-clock read in the
/// product, exactly as `records::Nonce` is minted once at this edge and DI'd inward
/// (`inv-determinism`: nondeterminism is seeded and injected, never ambient). Everything after is a
/// deterministic tick, so a seeded DST clock and the production clock are the same code path.
///
/// [`Absent`](RunClock::Absent) is a first-class "no clock here", not a failure mode: a replayed
/// durable does not carry the original run's per-record observation times, and re-stamping them
/// from the REPLAY's clock would present this moment as the original measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunClock {
    /// Yields `at`, then advances by `step_millis`. Production reads a whole record stream in one
    /// slurp, so it ticks by zero — every record of one read genuinely shares one instant. A DST
    /// seed supplies a non-zero step to make per-record instants distinguishable.
    Ticking {
        /// The instant the next read yields.
        at: dorc_core::RunInstant,
        /// How far the read advances the clock afterwards.
        step_millis: u64,
    },
    /// No clock: every read is `None`.
    Absent,
    /// The instants a durable RECORDED, keyed by the record ordinal they belong to.
    ///
    /// Not a clock at all, which is the point: a replay must date its records from the run that
    /// made them, and reading any live clock here would present the moment of reading as the
    /// moment of measurement. An ordinal the durable carries no instant for answers `None`.
    Recorded(BTreeMap<u64, dorc_core::RunInstant>),
}

impl RunClock {
    /// The next instant, advancing a ticking clock.
    pub fn now(&mut self) -> Option<dorc_core::RunInstant> {
        match self {
            Self::Ticking { at, step_millis } => {
                let read = *at;
                *at = dorc_core::RunInstant(read.0.saturating_add(*step_millis));
                Some(read)
            }
            Self::Absent | Self::Recorded(_) => None,
        }
    }

    /// The instant belonging to the record at `ordinal`.
    ///
    /// A live run reads its own clock and ignores the ordinal — the reading IS the record's
    /// arrival. A replay looks the ordinal up, because its records arrived once, already, and the
    /// only honest answer is the one that run wrote down.
    pub fn at(&mut self, ordinal: u64) -> Option<dorc_core::RunInstant> {
        match self {
            Self::Recorded(instants) => instants.get(&ordinal).copied(),
            Self::Ticking { .. } | Self::Absent => self.now(),
        }
    }
}

/// A record's key: the command **site** (the stable `LeafId`, `inv-site-keyed-results`)
/// plus an optional MEMBER index (task-L2 item-4): `None` for an ordinary single-fact
/// record (`site N`), `Some(m)` for member `m` of an in-loop Members family (`site N.M`).
/// The probe's [`dorc_plan::ProbePredict`] carries the same `(site, member)` pair, so the
/// bridge ([`facts_from_sites`]) keys a member record back to that member's cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecordKey {
    /// The command site.
    pub site: dorc_plan::LeafId,
    /// The in-loop member index, or `None` for a single-fact record.
    pub member: Option<u32>,
}

/// The already-read SOURCES one run is about — the values the controller scope is minted from,
/// bundled so every seat that needs the run's identity names it once (`lib-target-is-a-loom-seam`:
/// these crossed the seam as values; nothing here reopens a file).
#[derive(Debug, Clone, Copy)]
pub struct RunSources<'a> {
    /// The book's display path.
    pub book_name: &'a str,
    /// The book's bytes.
    pub book: &'a str,
    /// The loaded oracle paths, in load order.
    pub oracle_paths: &'a [String],
    /// Their bytes, in the same order.
    pub oracle_sources: &'a [String],
}

/// Controller-owned width-one identity. Payload records never construct or refresh this scope.
#[derive(Debug)]
pub struct WidthOneAttemptScope {
    host: String,
    target: WidthOneLocalTargetId,
    nonce: String,
    attempt: u32,
    sources: Vec<(String, String)>,
    generation: InitialWidthOneGeneration,
    book: (String, String),
}

#[derive(Debug)]
struct WidthOneLocalTargetId;

#[derive(Debug)]
struct InitialWidthOneGeneration;

impl WidthOneAttemptScope {
    fn new(framing: &Framing, sources: &RunSources<'_>) -> Self {
        Self {
            host: framing.host().to_owned(),
            target: WidthOneLocalTargetId,
            nonce: framing.nonce().0.clone(),
            attempt: framing.attempt(),
            sources: sources
                .oracle_paths
                .iter()
                .zip(sources.oracle_sources)
                .map(|(path, source)| (path.clone(), book_digest(source)))
                .collect(),
            generation: InitialWidthOneGeneration,
            book: (sources.book_name.to_owned(), book_digest(sources.book)),
        }
    }

    fn retain(&self) {
        let _ = (
            &self.host,
            &self.target,
            &self.nonce,
            self.attempt,
            &self.sources,
            &self.generation,
            &self.book,
        );
    }
}

/// Keeps controller attribution attached while live evidence participates in planning, and — since
/// `306b` §1 — the influence phase beside it.
#[derive(Debug)]
pub struct ScopedHostEvidence<T> {
    scope: WidthOneAttemptScope,
    value: T,
    influence: InfluencePhase,
}

impl<T> ScopedHostEvidence<T> {
    fn new(scope: WidthOneAttemptScope, value: T, influence: InfluencePhase) -> Self {
        Self {
            scope,
            value,
            influence,
        }
    }

    /// The typed value this attribution is attached to.
    #[must_use]
    pub fn results(&self) -> &T {
        &self.value
    }

    /// The controller identity that admitted it.
    pub fn scope(&self) -> &WidthOneAttemptScope {
        self.scope.retain();
        &self.scope
    }

    /// Where this run stands relative to host contact (`306b` §1b). In-memory only, decision-inert,
    /// and carried no further than the decision plane — nothing persists it.
    #[must_use]
    pub fn influence(&self) -> InfluencePhase {
        self.influence
    }

    /// This attempt's evidence as an INFLUENCE ACCOUNT — the driver seat that HOLDS the carrier
    /// (`fnd-two-drivers-compute-one-fact-twice`).
    ///
    /// One of exactly two phase→account transitions in the engine, and deliberately not unified
    /// with [`account_after_reaching_for_host_bytes`]: that one exists for the paths holding no
    /// carrier, and its widening argument is different in kind from this one's evidence.
    #[must_use]
    pub fn account(&self) -> InfluenceAccount {
        InfluenceAccount::of_phase(self.influence)
    }
}

/// The phase every attempt that reached for host bytes stands at, for the paths that hold no
/// graded carrier of their own: a well-owned attempt that produced nothing, and a replay.
///
/// Built by WIDENING (free, one-way — `306b` §1a), never by a second intake mint. Over-claiming
/// influence is the conservative direction on this axis, and both paths earn it honestly: whether
/// bytes arrived at all is host-determined (`306b` §1b names arrival a channel), and a durable's
/// contents are host-shaped by construction.
pub(crate) fn influence_after_reaching_for_host_bytes() -> InfluencePhase {
    Influenced::authored_before_contact(()).widen()
}

/// The probe results parsed from stdin, keyed by [`RecordKey`] (site, optional member —
/// `inv-site-keyed-results` + task-L2 item-4). One record per (site, member): the reported
/// Effect [`Verdict`] plus the raw probe-command rc carried alongside it. Whether that rc
/// is fold-usable is the FIREWALL's decision ([`facts_from_sites`]), not the parser's —
/// the parser faithfully carries what the probe reported (`inv-superposition`: the wire
/// transports the observed rc; the phased caller decides which channel, if any, it feeds).
#[derive(Debug, Default, Clone)]
pub struct SiteResults {
    /// One reported observation per (site, member).
    pub records: BTreeMap<RecordKey, SiteRecord>,
    /// The DERIVATION coord-blob lane (24E §5 / fork-s4-coordwire): per escalated wall-site, the
    /// raw `kind:entity` coordinate lines its host-run `touches()` printed (`deriv <leafid>
    /// coord=…`). Demuxed SEPARATELY from the `site` verdict records (a derivation-blob never
    /// collides with a site's `effect=`/`rc=` record — `inv-site-keyed-results`). Read back into a
    /// `Derived` [`dorc_plan::Footprint`] before the survival walk (24E §2 corr-§2).
    pub derivations: BTreeMap<dorc_plan::LeafId, Vec<String>>,
    /// The DERIV FAMILY end-records (`262` §2 / `26A` stop-1): per escalated wall-site, its
    /// `deriv-end <leafid> n=<K> body-rc=<R>` close-record. THE SAFETY INVERSION: a deriv footprint
    /// is an AT-MOST claim, so a mid-family cut SHRINKS it (⇒ more survivals — the under-execution
    /// direction). The consumer ([`merge_derived_footprints`]) refuses a family whose received coord
    /// count ≠ `K`, whose `R` is non-zero, or that has no end-record at all ⇒ wall-total. Absent key
    /// ⇒ the family never closed ⇒ refused.
    pub derivation_ends: BTreeMap<dorc_plan::LeafId, EmissionClose>,
    /// The RESOLVER canonicalization lane (24F §3): per `kind:entity` coordinate label, the readback
    /// of running its `<kind>.resolve()` host-side — a [`ResolvOutcome`]. Demuxed SEPARATELY from the
    /// verdict + derivation lanes (keyed by the coordinate, not a site — resolution is a pure function
    /// of the coordinate). Read into a [`dorc_plan::Resolutions`] before the survival walk.
    pub resolutions: BTreeMap<String, ResolvOutcome>,
    /// The REACH expansion lane (24G §4): per `(coordinate label, arm index)`, the RAW ENTITY lines a
    /// DYNAMIC `reaches()` arm printed host-side (`reach <coord> arm=<n> entity=…`). Demuxed SEPARATELY
    /// (keyed by the coordinate + arm, a pure function of them). Read into the footprints (via
    /// [`dorc_plan::Footprint::add_reached`]) before the survival walk. NB the arm index re-keys each
    /// line back to the arm's LIFTED kind (the vocabulary fence — the kind is never host-minted).
    pub reaches: BTreeMap<(String, usize), Vec<String>>,
    /// The REACH ARM close-records (`28P` item0's mechanism at its second consumer): per
    /// `(coordinate label, arm index)`, that arm's `reach-end <coord> arm=<n> n=<K> body-rc=<R>`.
    /// SAME safety inversion as the deriv close, arrived at from the opposite direction: a
    /// `disturbance_reaches_only` arm WIDENS an at-most footprint, so a cut or aborted arm leaves
    /// it wrongly NARROW, and narrow SPARES MORE. [`expand_footprints_via_reaches`] refuses the
    /// whole FOOTPRINT (the site walls total) when an arm's received count ≠ `K`, its `R` is
    /// non-zero, or it never closed at all.
    ///
    /// [`expand_footprints_via_reaches`]: crate::survival::expand_footprints_via_reaches
    pub reach_ends: BTreeMap<(String, usize), EmissionClose>,
    /// The REPORT lane (`27W` §2 tier-3): the `<verb> <class> <tail>` emissions an oracle wrote on
    /// its declining paths, re-keyed to their emitting site by the probe scaffold (`report site=<key>
    /// …`). Decision-inert (`two-plane-aid-law`): classes route AID only, never the license plane.
    /// Noise-tolerant (`27W:rul-report-noise-tolerant`): nothing is silently dropped — an
    /// unrecognized verb/class or free-form line is RETAINED (`recognized=false`), sanitized +
    /// size-capped, for max-verbosity display (d4). Ordered by arrival (a `Vec`, deduped on the
    /// whole record).
    pub reports: Vec<ReportRecord>,
    /// Was the source stream FRAMED (`262` §2)? Gates the at-most deriv-family completeness
    /// check ([`merge_derived_footprints`]) — only a framed stream carries `deriv-end`
    /// close-records; the legacy authored fixtures are trusted-complete.
    pub framed: bool,
}

/// One ingested report-lane record (`27W` §2 tier-3 · `decline-class-emission`): an emission an
/// oracle wrote on a declining path (`printf '<verb> <class> <tail>' >>"${DREP_V1:-/dev/null}"`),
/// re-keyed to its site by the probe scaffold. Decision-inert. Noise-tolerant: an unrecognized
/// verb/class is kept (`recognized=false`) as a generic author-note, never dropped, never an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportRecord {
    /// The emitting site (the scaffold's `site=<key>`), if attached.
    pub site: Option<RecordKey>,
    /// The recognized decline class, or `None` (degrade-generic — unknown token / free-form line).
    pub class: Option<dorc_aid::narrative::DeclineClass>,
    /// The full raw `<verb> <class> <tail>` emission, sanitized + size-capped at ingestion (the
    /// BASIC cap only — full why-surface sanitization is the security round's, `an-output-sanitization`
    /// fence named; `law-receipts-are-sensitive`). Retained for max-verbosity display (d4).
    pub raw: String,
    /// Whether the verb + class were BOTH recognized (else retained as a generic author-note).
    pub recognized: bool,
}

/// The ingestion size-cap on a report-lane emission's raw text (`27W` §2 — the BASIC cap only). A
/// tail longer than this is truncated with an ellipsis; a curious admin still sees the head at max
/// verbosity, and the full text never reaches a decision (decision-inert).
pub const REPORT_RAW_CAP: usize = 200;

/// One host-run emission family's close-record — `deriv-end` for an escalated site's derivation
/// (`262` §2 / `26A` stop-1) and `reach-end` for one dynamic `disturbance_reaches_only` arm. ONE
/// type for both because they answer one question in one grammar: the two lanes write the same
/// kind of open-ended, complete-by-contract survey, and forking the close would let the two gates
/// drift apart.
///
/// The two fields answer two INDEPENDENT questions, and conflating them was the hole
/// (`28P:fnd-the-count-gate-cannot-see-a-body-death`): `count` proves the RECORD STREAM arrived
/// whole, while `body_rc` proves the EMITTING BODY ran to completion. A body that emits three
/// coordinates and then dies on an unbound helper closes at `n=3` — self-consistent transport
/// carrying a wrongly-NARROW at-most claim. Both must pass before the footprint is trusted.
#[derive(Debug, Clone, Copy)]
pub struct EmissionClose {
    /// The `n=<K>` the family declared: how many records the scaffold emitted.
    pub count: u32,
    /// The `body-rc=<R>` the family declared: the emitting body's own termination status,
    /// captured before the record pipe (a pipeline's status is its RHS's, so the pre-`28P`
    /// scaffold discarded it). Non-zero ⇒ the survey did not finish ⇒ refuse the family.
    pub body_rc: u32,
}

/// THE PROBE-PROVENANCE FIREWALL, whole, in one seat: which of a site's measured channels may be
/// believed, keyed on the site's kind (`28M` §8; `inv-probe-sourced-values`).
///
/// The Status half is long-standing — only a VALID Query site's rc is fold-usable, and only when
/// the record is not a duplicate-meet CONFLICT (`262` §2: a conflicting rc is can't-tell, so it must
/// not substitute into the control-flow fold). An Establish site's rc is the CHECK's, not the
/// mutator's, and an invalid Query's is a stale resting rc; both are withheld to ⊤.
///
/// The out-channel half is `28P:dec-the-stdout-firewall-is-structural-too`. `28M` §8 named the
/// stdout parallel of the rc firewall ~SUSPECT and untraced; the trace found the property TRUE but
/// held by two accidents rather than by this mechanism — nothing emits `stdout=` today, and
/// `consumption_ok` blocks a consumed stdout UNCONDITIONALLY (16F §3) without ever reading the
/// value. That is exactly the "emergent, not typed" shape the custody work exists to retire: the day
/// a probe starts producing establish stdout, absence stops holding and only the consumption block
/// is left standing. So an Establish site's out-channels are ⊤ BY CONSTRUCTION, on the same line and
/// for the same reason as its rc — the probe never ran the mutator, so the mutator's own observables
/// cannot have probe-provenance. Inert today (the values are already ⊤); the point is that it stays
/// true when they stop being.
///
/// Extracted rather than inlined so the whole firewall reads as one thing: it was two `match`es on
/// one discriminant, forty lines apart, and a reader could satisfy themselves about the rc without
/// ever meeting the out-channels.
fn measured_channels(
    site_kind: dorc_plan::ProbeSiteKind,
    record: Option<&SiteRecord>,
) -> (Predicted<Rc>, Predicted<OutBytes>, Predicted<OutBytes>) {
    match site_kind {
        dorc_plan::ProbeSiteKind::Query { valid: true } => (
            record.map_or(Predicted::Top, |r| {
                if r.conflicted {
                    Predicted::Top
                } else {
                    Predicted::Value(r.rc)
                }
            }),
            record.map_or(Predicted::Top, |r| r.stdout),
            record.map_or(Predicted::Top, |r| r.stderr),
        ),
        // An invalid Query still carries reserved out-claims (its BYTES are not stale the way its
        // resting rc is); only the rc is withheld.
        dorc_plan::ProbeSiteKind::Query { valid: false } => (
            Predicted::Top,
            record.map_or(Predicted::Top, |r| r.stdout),
            record.map_or(Predicted::Top, |r| r.stderr),
        ),
        dorc_plan::ProbeSiteKind::Establish => (Predicted::Top, Predicted::Top, Predicted::Top),
    }
}

/// One coordinate's resolver readback (24F §3): the canonical form its `<kind>.resolve()` printed, or
/// [`Dangling`](ResolvOutcome::Dangling) — the resolver's natural failure on an enumerable kind (§4,
/// a reference to a non-existent entity), which rides the may-alias degrade + a loud diagnostic.
#[derive(Debug, Clone)]
pub enum ResolvOutcome {
    /// The resolver printed a canonical form (interned into the shared vocabulary at readback).
    Canonical(String),
    /// The resolver failed (non-zero rc / empty stdout) — a dangling reference (§4) ⇒ may-alias.
    Dangling,
}

/// One site's reported observation: the Effect-channel [`Verdict`], the raw probe-command
/// exit status, and the RESERVED `Stdout`/`Stderr` [`OutBytes`]s (`19F` §3 tuple shape).
/// The out-claims are parsed-and-stored but produce NOTHING this round — the probe never
/// emits `stdout=`/`stderr=`, so they arrive `Predicted::Top` in practice; the slots exist
/// so a future stdout-producing probe is a value-plumbing change, not a grammar change.
#[derive(Debug, Clone, Copy)]
pub struct SiteRecord {
    /// The Effect-channel verdict the probe reported.
    pub verdict: Verdict,
    /// The raw probe-command exit status.
    pub rc: Rc,
    /// The reserved stdout claim (`19F` §3; ⊤ in practice).
    pub stdout: Predicted<OutBytes>,
    /// The reserved stderr claim (`19F` §3; ⊤ in practice).
    pub stderr: Predicted<OutBytes>,
    /// This record's identity as a probe EVENT (C6, `27V` §2): its arrival ordinal in the deframed
    /// stream (deterministic, no clock) plus the instant the controller observed it, when the edge
    /// injected a clock. Minted straight into the [`dorc_core::OriginKind::ProbeResult`] origin so
    /// the receipt can order/attribute probe events. A meet keeps the first-seen stamp.
    pub stamp: dorc_core::ProbeStamp,
    /// A DUPLICATE-MEET marker (`262` §2 / `26A` stop-1): set when two records for one
    /// (site, member) key DISAGREED and were met toward ⊤. The §1 tie-break law forbids
    /// first-wins/last-wins; a conflict is can't-tell. `verdict` is already `Unknown` when
    /// this is set (effect ⇒ run); this ALSO withholds the fold-usable Query rc
    /// ([`facts_from_sites`]) so a conflicting rc cannot substitute into the control-flow fold.
    pub conflicted: bool,
}

/// Converts only grammar-admitted records. The legacy string parser above is replay-only until 3C2.
fn parse_admitted_results(
    records: &AdmittedUnscopedHostRecords,
    clock: &mut RunClock,
    interner: &mut Interner,
) -> SiteResults {
    let mut out = SiteResults {
        framed: true,
        ..SiteResults::default()
    };
    for (ordinal, record) in records.iter().enumerate() {
        match record {
            dorc_plan::records::AdmittedHostRecord::Site {
                key,
                effect,
                rc,
                stdout,
                stderr,
                ..
            } => {
                let Some(key) = parse_site_key(key) else {
                    continue;
                };
                let rec = SiteRecord {
                    verdict: effect_word_to_verdict(effect),
                    rc: Rc(rc),
                    stdout: stdout.map_or(Predicted::Top, |value| {
                        Predicted::Value(OutBytes(interner.intern(value)))
                    }),
                    stderr: stderr.map_or(Predicted::Top, |value| {
                        Predicted::Value(OutBytes(interner.intern(value)))
                    }),
                    conflicted: false,
                    stamp: dorc_core::ProbeStamp::received(
                        ordinal as u64,
                        clock.at(ordinal as u64),
                    ),
                };
                out.records
                    .entry(key)
                    .and_modify(|prior| *prior = meet_record(*prior, rec))
                    .or_insert(rec);
            }
            dorc_plan::records::AdmittedHostRecord::Derivation { site, coord } => {
                out.derivations
                    .entry(dorc_plan::LeafId(site))
                    .or_default()
                    .push(coord.to_owned());
            }
            dorc_plan::records::AdmittedHostRecord::DerivationEnd {
                site,
                count,
                body_rc,
            } => {
                out.derivation_ends
                    .insert(dorc_plan::LeafId(site), EmissionClose { count, body_rc });
            }
            dorc_plan::records::AdmittedHostRecord::Resolution { coord, canonical } => {
                out.resolutions.insert(
                    coord.to_owned(),
                    canonical.map_or(ResolvOutcome::Dangling, |value| {
                        ResolvOutcome::Canonical(value.to_owned())
                    }),
                );
            }
            dorc_plan::records::AdmittedHostRecord::Reach { coord, arm, entity } => {
                out.reaches
                    .entry((coord.to_owned(), arm))
                    .or_default()
                    .push(entity.to_owned());
            }
            dorc_plan::records::AdmittedHostRecord::ReachEnd {
                coord,
                arm,
                count,
                body_rc,
            } => {
                out.reach_ends
                    .insert((coord.to_owned(), arm), EmissionClose { count, body_rc });
            }
            dorc_plan::records::AdmittedHostRecord::Report { body } => {
                parse_report_record(body, &mut out);
            }
        }
    }
    out
}

/// Ingest one report-lane record (`27W` §2 tier-3): `report [site=<key>] <verb> <class> <tail…>`.
/// Decision-inert. Noise-tolerant (`27W:rul-report-noise-tolerant`): the verb/class are recognized
/// best-effort, but an unrecognized token or free-form line is RETAINED (`recognized=false`), never
/// dropped, never an error. Deduped on the whole record — a tier-3 echo of an already-ingested line
/// adds nothing (the dedup the tier-2 static classification will later key by (site, arm, class)).
/// Type one report-lane emission (`27W` §2 tier-3, decision-inert). Shared with the legacy parser.
pub fn parse_report_record(rest: &str, out: &mut SiteResults) {
    let (site, body) = match rest.strip_prefix("site=") {
        Some(after) => {
            let (key_tok, tail) = after.split_once(' ').unwrap_or((after, ""));
            (parse_site_key(key_tok), tail)
        }
        None => (None, rest),
    };
    // v1 grammar: verb `decline` + a starter-set class; either unrecognized ⇒ degrade-generic.
    let mut words = body.split_whitespace();
    let verb = words.next();
    let class = words
        .next()
        .and_then(dorc_aid::narrative::DeclineClass::from_token);
    let recognized = verb == Some("decline") && class.is_some();
    let rec = ReportRecord {
        site,
        class,
        raw: sanitize_report_raw(body),
        recognized,
    };
    if !out.reports.contains(&rec) {
        out.reports.push(rec);
    }
}

/// Sanitize + size-cap a report-lane emission's raw text at ingestion (`27W` §2).
///
/// A thin delegation to the shared display seat: the lane keeps its own budget
/// ([`REPORT_RAW_CAP`]) and its own destination (a plain advisory line, which nothing measures),
/// while the encoding itself is one implementation shared with every other display route. NEVER a
/// decision input (decision-inert), and encoding grants the bytes no trust
/// (`sinv-hostile-sensitive-orthogonal`). Public because the binary's test-gated legacy parser
/// shares it (`rul-fixture-identity-never-production` keeps that parser behind compile-time test
/// exposure, so it cannot move here).
#[must_use]
pub fn sanitize_report_raw(s: &str) -> String {
    dorc_aid::display::encode_line(s, REPORT_RAW_CAP)
}

/// Parse a `u32` leaf-id.
///
/// This and its two siblings below are the LEGACY string parser's tokenizers. The legacy ENTRY
/// (`parse_results`) stays `#[cfg(test)]`-gated in the binary, which is where
/// `rul-fixture-identity-never-production` puts headerless parsing; a field splitter grants no
/// parser authority by itself, so these are ordinary lib items the binary's test lane drives.
/// A `site`/`deriv` token to a `LeafId`. Public because the binary's test-gated legacy
/// headerless parser shares it (`rul-fixture-identity-never-production` keeps that parser behind
/// compile-time test exposure, so it cannot move here).
#[must_use]
pub fn parse_leaf(tok: &str) -> Option<dorc_plan::LeafId> {
    tok.parse::<u32>().ok().map(dorc_plan::LeafId)
}

/// Split a record body at a FREE-CONTENT `key=` into `(head, value)` where `value` runs to
/// end-of-line (whitespace included — `262` §2 last-to-token). The key must be preceded by a
/// space (or begin the body). Returns `None` when the key is absent.
/// Split `<head> <key><value>` at the first occurrence of `key`. Shared with the legacy parser.
#[must_use]
pub fn split_key<'a>(body: &'a str, key: &str) -> Option<(&'a str, &'a str)> {
    if let Some(v) = body.strip_prefix(key) {
        return Some(("", v));
    }
    let pat = format!(" {key}");
    let at = body.find(&pat)?;
    Some((&body[..at], &body[at..][pat.len()..]))
}

/// Parse one `site <leafid> effect=<word> rc=<n> [stdout=<free-content>]` record (`262` §2).
/// `stdout=` is the FREE-CONTENT field (last-to-token) — the read-value lane's future carrier
/// (`279f` rider): it runs to end-of-line so embedded spaces survive byte-exactly. `stderr=`
/// stays single-token (stderr handling is out of spike scope — churn-avoidance-disclosure).
/// Unknown keys BEFORE the free-content field are ignored (additive-keys, `24Kc`). A duplicate
/// (site, member) record MERGES BY MEET, never last-wins (`262` §1 tie-break law).
/// Type one `site` record into the results map. Shared with the legacy parser.
pub fn parse_site_record(
    rest: &str,
    stamp: dorc_core::ProbeStamp,
    out: &mut SiteResults,
    interner: &mut Interner,
) {
    // `stdout=` is the trailing free-content field; everything from it runs to EOL.
    let (head, stdout) = match split_key(rest, "stdout=") {
        Some((h, v)) => (h, Predicted::Value(OutBytes(interner.intern(v)))),
        None => (rest, Predicted::Top),
    };
    let mut it = head.split_whitespace();
    let Some(key) = it.next().and_then(parse_site_key) else {
        return; // malformed site key ⇒ drop (⇒ Unknown ⇒ run)
    };
    let mut verdict = Verdict::Unknown;
    let mut rc = Rc(0);
    let mut stderr = Predicted::Top;
    for tok in it {
        if let Some(w) = tok.strip_prefix("effect=") {
            verdict = effect_word_to_verdict(w);
        } else if let Some(n) = tok.strip_prefix("rc=").and_then(|n| n.parse::<i32>().ok()) {
            rc = Rc(n);
        } else if let Some(t) = tok.strip_prefix("stderr=") {
            stderr = Predicted::Value(OutBytes(interner.intern(t)));
        }
    }
    let rec = SiteRecord {
        verdict,
        rc,
        stdout,
        stderr,
        conflicted: false,
        stamp,
    };
    out.records
        .entry(key)
        .and_modify(|prior| *prior = meet_record(*prior, rec))
        .or_insert(rec);
}

/// Meet two records reported for one (site, member) key (`262` §2 duplicate-by-meet / §1
/// tie-break law). Identical ⇒ idempotent (unchanged). ANY disagreement ⇒ can't-tell: verdict
/// ⊤ (⇒ run), out-claims ⊤, and `conflicted` set so the fold-usable Query rc is withheld
/// ([`facts_from_sites`]). NEVER first-wins/last-wins; commutative + idempotent, so arrival
/// order cannot change the fold (`262` §1 pin-fold-permutation).
fn meet_record(a: SiteRecord, b: SiteRecord) -> SiteRecord {
    let rc_conflict = a.rc != b.rc;
    SiteRecord {
        verdict: if a.verdict == b.verdict {
            a.verdict
        } else {
            Verdict::Unknown
        },
        rc: a.rc,
        stdout: if a.stdout == b.stdout {
            a.stdout
        } else {
            Predicted::Top
        },
        stderr: if a.stderr == b.stderr {
            a.stderr
        } else {
            Predicted::Top
        },
        conflicted: a.conflicted || b.conflicted || rc_conflict || a.verdict != b.verdict,
        stamp: a.stamp, // keep the first-seen stamp (C6): the meet is order-independent
    }
}

/// Parse a record's site key token (task-L2 item-4): `N` ⇒ `RecordKey { site: N, member:
/// None }`; `N.M` ⇒ `RecordKey { site: N, member: Some(M) }` (member `M` of an in-loop
/// Members family). Both `N` and `M` are `u32`; a non-numeric / malformed token ⇒ `None`
/// (the record is dropped ⇒ that cell folds to Unknown ⇒ run, the kFAIL-perform floor).
fn parse_site_key(tok: &str) -> Option<RecordKey> {
    match tok.split_once('.') {
        Some((leaf, member)) => Some(RecordKey {
            site: dorc_plan::LeafId(leaf.parse::<u32>().ok()?),
            member: Some(member.parse::<u32>().ok()?),
        }),
        None => Some(RecordKey {
            site: dorc_plan::LeafId(tok.parse::<u32>().ok()?),
            member: None,
        }),
    }
}

/// Map the probe's three-outcome `effect=` word to a [`Verdict`] (the probe-record
/// convention, 202 §3): `holds ⇒ Converged`, `absent ⇒ Diverged`,
/// anything else (`cant-tell` / garbled) ⇒ `Unknown` (the safe direction).
fn effect_word_to_verdict(word: &str) -> Verdict {
    match word {
        "holds" => Verdict::Converged,
        "absent" => Verdict::Diverged,
        _ => Verdict::Unknown,
    }
}

impl WidthOneAttemptScope {
    /// The book digest this run's controller computed, for the drift comparison a replay makes
    /// BEFORE it trusts a durable (`22F` book-identity).
    #[must_use]
    pub fn book_digest(&self) -> &str {
        &self.book.1
    }
}

/// One admitted attempt: the grammar-checked wire records (the durable writer's input) and the
/// typed results, still carrying the controller scope that admitted them.
#[derive(Debug)]
pub struct ScopedRecords {
    /// The bounded, grammar-admitted records, retained verbatim for the durable.
    pub records: AdmittedUnscopedHostRecords,
    /// The typed results, scope attached (`rul-attribution-is-controller-minted`: scope must
    /// survive every conversion).
    pub scoped: ScopedHostEvidence<SiteResults>,
}

/// Frame-check, type and SCOPE one already-bounded host-evidence stream — the intake edge, in the
/// order `rul-admission-is-a-closed-outcome` fixes.
///
/// `framing` is the controller's, minted at its own edge before any byte was read; the stream's
/// header is CHECKED against it and never mints it (`rul-attribution-is-controller-minted`). The
/// aggregate-stream bound is already spent by the caller's [`read_host_evidence`], which is where
/// it belongs: bounding is a property of the READ (`rul-host-bytes-bounded-before-admission`), and
/// this seat never opens anything. The three outcomes stay three — `Refused` is a broken channel,
/// never a measurement.
pub fn admit_controller_records(
    framing: &Framing,
    sources: &RunSources<'_>,
    bytes: &BoundedHostBytes,
    clock: &mut RunClock,
    interner: &mut Interner,
) -> Admission<ScopedRecords> {
    let scope = WidthOneAttemptScope::new(framing, sources);
    match admit_unscoped_host_records(bytes, framing, HostEvidenceLimits::spike_default()) {
        Admission::Admitted(admitted) => {
            // The grade rides the CONVERSION rather than being re-asserted on the far side, which
            // is what lets the one intake mint serve the whole bytes → records → results chain.
            let results = admitted.map(|records| parse_admitted_results(records, clock, interner));
            let (results, influence) = results.into_read();
            // The durable takes the RECORDS, never the grade: persisting a grade is durable
            // enrichment, deliberately out of v0 (`306c` §2's scope fence).
            let (records, _) = admitted.into_read();
            Admission::Admitted(ScopedRecords {
                records,
                scoped: ScopedHostEvidence::new(scope, results, influence),
            })
        }
        Admission::NoObservation => Admission::NoObservation,
        Admission::Refused(reason) => Admission::Refused(reason),
    }
}

/// The FIXTURE intake: the same edge, over a case's own committed `dorc-records/1` bytes.
///
/// **Why this is legal, and why it is a separate function** (`28L:rul-records-seam-approved`).
/// `rul-attribution-is-controller-minted` names its re-entry trigger as "any second SCOPE becoming
/// representable at all", and lists what makes one: real transport, concurrency, retry, cross-host
/// reuse, saved approval. None of those becomes representable here. A loom run mints exactly ONE
/// scope, for one book, in one process, with no transport and no retry; two scopes never co-exist
/// in one world. What appears is a second CONTROLLER — `dorc-loom`, of its own hermetic in-process
/// run — which is precisely the entity the law says must mint the scope. Carrying the scope does
/// not yet have to become checking it.
///
/// The fence is the SIGNATURE, on the `rul-fixture-identity-never-production` reading that "comments
/// are not a fence — absence of a constructor is": there is no `Framing`, `RemoteIdentity`, host,
/// nonce or attempt parameter, and none can be added by a caller. The framing is
/// [`Framing::spike`] — the ONE named substitution point, itself structurally unable to reach a
/// managed host — built inside. A fixture caller therefore cannot name a host even in principle.
///
/// What a signature cannot fence is who CALLS the production sibling above, because a binary target
/// and a harness are both foreign crates to this lib and neither can be privileged by a type. That
/// half is pinned lexically instead, by `fixture_intake_is_unreachable_from_production` — stated
/// here so the next reader does not mistake the type system for doing more work than it does.
///
/// The bound is spent here rather than by the caller because a case's bytes arrive as a slice: the
/// read is a cursor over memory, not an I/O act, so it stays on this side of the seam.
pub fn admit_fixture_records(
    sources: &RunSources<'_>,
    stream: &[u8],
    clock: &mut RunClock,
    interner: &mut Interner,
) -> Admission<ScopedRecords> {
    let framing = Framing::spike(book_digest(sources.book));
    match read_host_evidence(stream, HostEvidenceLimits::spike_default()) {
        Admission::Admitted(bytes) => {
            admit_controller_records(&framing, sources, &bytes, clock, interner)
        }
        Admission::NoObservation => Admission::NoObservation,
        Admission::Refused(reason) => Admission::Refused(reason),
    }
}

/// Frame compact fixture records, then admit them through [`admit_fixture_records`].
#[must_use]
pub fn admit_fixture_inner_records(
    sources: &RunSources<'_>,
    inner: &str,
    clock: &mut RunClock,
    interner: &mut Interner,
) -> Admission<ScopedRecords> {
    let framing = Framing::spike(book_digest(sources.book));
    let records = inner
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect::<Vec<_>>();
    let sites = records
        .iter()
        .filter(|line| line.starts_with("site "))
        .count();
    let mut stream = format!(
        "{} sites={sites} {}\n",
        dorc_plan::records::expected_header_prefix(&framing),
        dorc_plan::records::TERMINAL_TOKEN,
    );
    for record in records {
        let _ = std::fmt::Write::write_fmt(
            &mut stream,
            format_args!("dorc {record} {}\n", dorc_plan::records::TERMINAL_TOKEN),
        );
    }
    let _ = std::fmt::Write::write_fmt(
        &mut stream,
        format_args!(
            "dorc-records-end/1 nonce={} {}\n",
            framing.nonce().0,
            dorc_plan::records::TERMINAL_TOKEN
        ),
    );
    admit_fixture_records(sources, stream.as_bytes(), clock, interner)
}

/// A well-owned attempt that produced no usable fact, scoped exactly as an admitted one is
/// (`rul-admission-is-a-closed-outcome`: `NoObservation` is ordinary conservative planning, and it
/// must not lose its attribution on the way to the fold).
#[must_use]
pub fn no_observation(
    framing: &Framing,
    sources: &RunSources<'_>,
) -> ScopedHostEvidence<SiteResults> {
    ScopedHostEvidence::new(
        WidthOneAttemptScope::new(framing, sources),
        SiteResults {
            framed: true,
            ..SiteResults::default()
        },
        influence_after_reaching_for_host_bytes(),
    )
}

/// Attach a fixture controller's already-admitted results to its width-one scope.
pub(crate) fn scope_fixture_results(
    framing: &Framing,
    sources: &RunSources<'_>,
    results: SiteResults,
) -> ScopedHostEvidence<SiteResults> {
    ScopedHostEvidence::new(
        WidthOneAttemptScope::new(framing, sources),
        results,
        influence_after_reaching_for_host_bytes(),
    )
}

// ---- moved verbatim from `main.rs` (the records fold + its probe-origin mint) ----
/// Fold the admitted records into the per-cell [`Observable`]s the plan builder consults, plus the
/// narrative every safety-narrowing on the way mints and the cells a cross-site disagreement
/// collapsed.
///
/// THE FIREWALL of this edge: a record's Effect verdict always feeds the cell, but its rc becomes
/// fold-usable Status only for a VALID Query site whose record did not self-contradict — an
/// establish's rc is the check's, not the mutator's, and a stale rc under an erased branch measured
/// a question that is no longer asked. Everything else is withheld to ⊤ ⇒ run
/// (`inv-probe-sourced-values`; `kFAIL-perform`).
///
/// `validity` is the fixpoint's per-site view; a caller with no fixpoint passes an empty map and
/// every site keeps the validity the probe recorded.
#[must_use]
pub fn facts_from_sites(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
    validity: &BTreeMap<dorc_plan::LeafId, bool>,
) -> (
    BTreeMap<dorc_core::FactKey, Observable>,
    Vec<CollapseNarrative>,
    BTreeMap<dorc_core::FactKey, u32>,
) {
    use dorc_plan::ProbeSiteKind;
    let mut by_fact: BTreeMap<dorc_core::FactKey, Observable> = BTreeMap::new();
    let mut sites_per_fact: BTreeMap<dorc_core::FactKey, u32> = BTreeMap::new();
    let mut collapsed: BTreeSet<dorc_core::FactKey> = BTreeSet::new();
    // C4 (`27V` Lane A): the `Measured` fact-merge narrative minted beside the ⊤-fold.
    // `first_measured` remembers each cell's first RECORD-BACKED establisher, so a cross-site
    // conflict names both operands — and so a cell only narrates when the HOST really said two
    // things. A check with no record contributes ⊤ to the meet (correctly: unmeasured ⇒ run), and
    // disagreeing with ⊤-from-silence is a measurement that never happened, not a contradiction.
    let mut collapse_narrative: Vec<CollapseNarrative> = Vec::new();
    let mut first_measured: BTreeMap<dorc_core::FactKey, dorc_aid::diag::SiteId> = BTreeMap::new();
    for check in &probe.checks {
        let site_id = dorc_aid::diag::SiteId {
            leaf: check.site,
            member: check.member,
        };
        // Key the record by (site, member) — a member check (`site N.M`) reads its own
        // sub-record (task-L2 item-4); an ordinary check (`site N`) reads `member: None`.
        let record = results.records.get(&RecordKey {
            site: check.site,
            member: check.member,
        });
        let effect = record.map_or(Verdict::Unknown, |r| r.verdict);
        let site_kind = match check.site_kind {
            ProbeSiteKind::Query { valid } => ProbeSiteKind::Query {
                valid: validity.get(&check.site).copied().unwrap_or(valid),
            },
            ProbeSiteKind::Establish => ProbeSiteKind::Establish,
        };
        let (status, stdout, stderr) = measured_channels(site_kind, record);
        let obs = Observable {
            effect,
            status,
            stdout,
            stderr,
        };
        // Source 1 — a WITHIN-site conflict: a valid Query whose parse-merged record contradicts
        // itself (`r.conflicted`), so its fold-usable rc is withheld to ⊤ above.
        if matches!(site_kind, ProbeSiteKind::Query { valid: true })
            && record.is_some_and(|r| r.conflicted)
        {
            collapse_narrative.push(measured_merge_disagreement(site_id, &[site_id]));
        }
        // C5 substitution refusal. tc-substitution-refusal-scope: minted ONLY for the invalid-Query
        // withhold (a genuine consumed-channel refusal), NOT the establish withhold (firewall-by-
        // design; it elides via Effect). Flagged UP — a scoping judgment (`inv-superposition`).
        if matches!(site_kind, ProbeSiteKind::Query { valid: false }) {
            collapse_narrative.push(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::SubstitutionRefusal {
                    site: site_id,
                    top_channel: dorc_core::Channel::StatusRelaxable,
                },
            ));
        }
        // Runtime EntryFailure (`27C` §3): entry-bearing ≥2 sink-landing, class-only + inert. rc 127
        // ⇒ missing deps; other ≥2 ⇒ in-context decline. Refused/Impossible unminted (SEAM: a marker).
        if check.entry.is_some()
            && let Some(rc) = record.map(|r| r.rc.0)
            && rc >= 2
        {
            let class = if rc == 127 {
                dorc_aid::narrative::EntryFailureTag::MissingDeps
            } else {
                dorc_aid::narrative::EntryFailureTag::InContextDecline
            };
            collapse_narrative.push(CollapseNarrative::new(
                SpeechAct::Measured,
                CollapseKind::EntryFailure {
                    site: site_id,
                    class,
                },
            ));
        }
        // Source 2 — a CROSS-site conflict: two sites on one cell disagree ⇒ the meet ⊤s the channel.
        let per_fact = sites_per_fact.entry(check.fact).or_default();
        *per_fact = per_fact.saturating_add(1);
        if let Some(prior) = by_fact.get(&check.fact).copied() {
            if prior != obs {
                if record.is_some()
                    && let Some(prior_site) = first_measured.get(&check.fact).copied()
                {
                    collapse_narrative
                        .push(measured_merge_disagreement(site_id, &[prior_site, site_id]));
                }
                collapsed.insert(check.fact);
            }
            by_fact.insert(check.fact, merge_observable(prior, obs));
        } else {
            by_fact.insert(check.fact, obs);
        }
        if record.is_some() {
            first_measured.entry(check.fact).or_insert(site_id);
        }
    }
    let collapsed = collapsed
        .into_iter()
        .map(|fact| (fact, sites_per_fact.get(&fact).copied().unwrap_or_default()))
        .collect();
    (by_fact, collapse_narrative, collapsed)
}

/// C6 (`27V` Lane A · `OriginKind::ProbeResult`): mint one probe-result origin per received record
/// and key it by the fact it establishes, so [`dorc_plan::build_plan_walled`] can attach it to a
/// licensing disposition's `Witness` — the why-chain's tie from "why THIS elision" back to the
/// exact record that measured it. The stamp is the record's stream ordinal (deterministic, no
/// clock — `inv-determinism`). A fact backed by two records JOINS their origins (two records are
/// two events). Runs at the cli edge where the arena lives (`io-at-edges-only`); the [`Observable`]
/// stays receipt-clean (the tc-c6-scope ruling: the receipt rides the record, not the value).
///
/// The origin NODE's source span stays `None`: an [`dorc_core::OriginNode`] carries a bare
/// [`dorc_core::Span`], which is file-ambiguous once >1 oracle is loaded (`law-lineno-identity`).
/// The file-qualified reporting span therefore rides the [`dorc_plan::ReportedObservation`] beside
/// the receipt, which is also where the tool-rc and the observation instant live.
///
/// A fact measured by SEVERAL records keeps the joined receipt but reports NO single observation:
/// two records are two events with no one speaker, instant, or rc, and inventing a winner would be
/// a fabricated measurement.
pub fn probe_origins(
    probe: &dorc_plan::ProbePlan,
    results: &SiteResults,
    arena: &mut ProvArena,
) -> BTreeMap<dorc_core::FactKey, dorc_plan::ProbeAttribution> {
    let mut origins: BTreeMap<dorc_core::FactKey, dorc_plan::ProbeAttribution> = BTreeMap::new();
    for check in &probe.checks {
        let Some(record) = results.records.get(&RecordKey {
            site: check.site,
            member: check.member,
        }) else {
            continue;
        };
        let origin = arena.leaf(dorc_core::OriginKind::ProbeResult(record.stamp), None);
        let reported = Some(dorc_plan::ReportedObservation {
            stamp: record.stamp,
            tool_rc: record.rc,
            predict_span: check.defining_span,
        });
        let attribution = match origins.get(&check.fact) {
            Some(prior) => dorc_plan::ProbeAttribution {
                origin: arena.join(None, &[prior.origin, origin]).unwrap_or(origin),
                reported: None,
            },
            None => dorc_plan::ProbeAttribution { origin, reported },
        };
        origins.insert(check.fact, attribution);
    }
    origins
}

/// Build the `Measured`-tier fact-merge narrative a probe-result disagreement mints (C4;
/// `27V` Lane A, `AID-NEEDS:law-collapse-mints-narrative`): a host self-contradiction at `cell`,
/// carrying the participating establisher sites as operands (`minting_line`/`shown` filled by d3).
/// Decision-inert (`two-plane-aid-law`): the conservative meet already folded the channel to ⊤
/// (`kFAIL-perform`, the only safe resolution of a self-contradicting host); this only narrates why.
fn measured_merge_disagreement(
    cell: dorc_aid::diag::SiteId,
    sites: &[dorc_aid::diag::SiteId],
) -> CollapseNarrative {
    let operands = dorc_aid::narrative::Operands::capped(
        sites
            .iter()
            .map(|&site| dorc_aid::narrative::ValueOperand {
                site,
                minting_line: None,
                shown: None,
            })
            .collect(),
    );
    CollapseNarrative::new(
        SpeechAct::Measured,
        CollapseKind::FactMergeDisagreement { cell, operands },
    )
}

/// Conservatively merge two [`Observable`]s reported for the SAME cell (20I find-6a /
/// item-5). Per channel: equal values pass through; ANY disagreement degrades the
/// channel to ⊤ (`Verdict::Unknown` for Effect, `Predicted::Top` for status/stdout/
/// stderr). This is the meet toward ⊤ — never last-write-wins — so a self-contradicting
/// host folds to run (`kFAIL-perform`), the only safe resolution. Order-independent
/// (commutative + idempotent): merging in any site order yields the same ⊤-on-conflict.
fn merge_observable(a: Observable, b: Observable) -> Observable {
    Observable {
        effect: if a.effect == b.effect {
            a.effect
        } else {
            Verdict::Unknown
        },
        status: if a.status == b.status {
            a.status
        } else {
            Predicted::Top
        },
        stdout: if a.stdout == b.stdout {
            a.stdout
        } else {
            Predicted::Top
        },
        stderr: if a.stderr == b.stderr {
            a.stderr
        } else {
            Predicted::Top
        },
    }
}
