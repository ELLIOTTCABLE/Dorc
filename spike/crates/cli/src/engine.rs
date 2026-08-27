//! The parser-independent production invocation engine.

use std::collections::{BTreeMap, BTreeSet};

use crate::artifact::{ArtifactForm, ArtifactSet, StdoutPosture};
use crate::fixpoint::{FrozenModel, attribute_cascades, classify_round, settle_world};
use crate::kinds::{KindReaches, KindResolvers, build_kind_reaches, build_kind_resolvers};
use crate::results::{ReportRecord, RunClock, RunSources, SiteResults, probe_origins};
use crate::snapshot::StaticLoadSnapshot;
use crate::survival::{
    WrapperSets, build_resolutions, build_survival_footprints, build_wrapped_analysis,
    collect_coord_kinds, collect_resolver_coords, dangling_diagnostics, entity_text_of,
    expand_footprints_via_reaches, lift_touches_sets, merge_derived_footprints, pair_touches_sets,
    resolve_touches_footprint, ship_touches_body,
};
use crate::why::{
    WhyReport, collect_wall_steps, first_wall_hint, oracle_locus, render_coord,
    unresolvable_diagnostics, why_report_parts,
};
use crate::world::{
    WhyWorld, definition_table, demote_on_certifier_trip, never_live_predict_rows,
    record_pre_network_trip, ship_predict_body, ship_verdict_body, shipping_source,
};
use crate::{CONSENT_FLAG, Mode, PlanTally, Receipt, SourceMatch};
use dorc_aid::diag::{
    Diag, DiagCode, EmittedLineUnsafeForPaste, EscalationPolicy, OracleMatchedZeroSites,
    PasteHygieneHazardReason,
};
use dorc_aid::said::Said;
use dorc_aid::tagged::RenderParts;
use dorc_aid::{Carrier, CollapseKind, CollapseNarrative, Severity, SpeechAct};
use dorc_core::{Capability, EscalationDial, Interner, ProvArena, Symbol};

/// Whether the semantic pipeline has a separate artifact destination.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactDestinationShape {
    /// The artifact is carried by stdout.
    Stdout,
    /// The artifact has a separate directory destination.
    Directory,
}

impl ArtifactDestinationShape {
    /// Selects the shape from the edge's directory-request fact.
    #[must_use]
    pub const fn from_directory_requested(requested: bool) -> Self {
        if requested {
            Self::Directory
        } else {
            Self::Stdout
        }
    }
    /// Reports whether the artifact has a separate directory destination.
    #[must_use]
    pub const fn is_directory(self) -> bool {
        matches!(self, Self::Directory)
    }
}

/// Whether the engine emits its deterministic argv readout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgvReadout {
    /// Do not emit the readout.
    Hidden,
    /// Emit the readout on stderr.
    Visible,
}

/// Which wall policy the admin admitted for this invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurvivalPolicy {
    /// Running mutation forms an honest total wall.
    HonestWalls,
    /// Authored at-most claims may license survival.
    RiskAccepted,
}

/// How much of a pull report to render.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WhyDepth {
    /// Apply the ordinary curated selection.
    Curated,
    /// Render the deepest available tier.
    All,
}

/// Semantic reporting choices, after parsing.
#[derive(Clone, Debug)]
pub struct ReportingOptions {
    /// Optional source address for `why`.
    pub why_address: Option<String>,
    /// The pull-report depth.
    pub why_depth: WhyDepth,
    /// The deterministic argv readout policy.
    pub argv_readout: ArgvReadout,
}

/// Probe-context and survival choices, after parsing.
#[derive(Clone, Copy, Debug)]
pub struct AnalysisOptions {
    /// The admin's survival policy.
    pub survival: SurvivalPolicy,
    /// Whether a wrapper may enter another context.
    pub escalation: EscalationDial,
    /// Mechanical authority already held by the connection.
    pub capability: Capability,
}

/// Artifact routing choices, after terminal observation.
#[derive(Clone, Copy, Debug)]
pub struct ArtifactOptions {
    /// A specifically requested form, or `None` for automatic selection.
    pub form: Option<ArtifactForm>,
    /// Whether stdout is being watched.
    pub stdout: StdoutPosture,
    /// Whether a separate artifact destination exists.
    pub destination: ArtifactDestinationShape,
}

/// Whether a completed live run should generate a whylog durable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOutput {
    /// Generate no durable.
    Disabled,
    /// Generate and ask the injected edge to publish it.
    Enabled,
}

/// Parser-independent semantic choices consumed by the shared pipeline.
#[derive(Clone, Debug)]
pub struct EngineOptions {
    /// The selected product mode.
    pub mode: Mode,
    /// Analysis authority and context choices.
    pub analysis: AnalysisOptions,
    /// Reporting choices.
    pub reporting: ReportingOptions,
    /// Artifact routing choices.
    pub artifact: ArtifactOptions,
    /// Durable generation policy.
    pub durable: DurableOutput,
}

impl EngineOptions {
    const fn risk_faultless_skips(&self) -> bool {
        matches!(self.analysis.survival, SurvivalPolicy::RiskAccepted)
    }

    fn why_address(&self) -> Option<&str> {
        self.reporting.why_address.as_deref()
    }

    const fn all(&self) -> bool {
        matches!(self.reporting.why_depth, WhyDepth::All)
    }

    const fn debug_argv(&self) -> bool {
        matches!(self.reporting.argv_readout, ArgvReadout::Visible)
    }

    const fn dial(&self) -> EscalationDial {
        self.analysis.escalation
    }

    const fn capability(&self) -> Capability {
        self.analysis.capability
    }

    const fn form(&self) -> Option<ArtifactForm> {
        self.artifact.form
    }

    const fn stdout(&self) -> StdoutPosture {
        self.artifact.stdout
    }

    const fn artifact_destination(&self) -> ArtifactDestinationShape {
        self.artifact.destination
    }
}

/// A replay admitted and acquired by the production boundary.
#[derive(Debug)]
pub struct Replay {
    /// The digest the recorded decision must reproduce.
    pub decision_digest: String,
    /// The original run's start instant.
    pub started_at: Option<dorc_core::RunInstant>,
    /// The recorded narrative stream version.
    pub record_stream_version: u32,
    /// Recorded per-record instants.
    pub instants: BTreeMap<u64, dorc_core::RunInstant>,
    /// The already-admitted record stream, when one existed.
    pub records: Option<dorc_plan::records::AdmittedUnscopedHostRecords>,
}

/// One acquired semantic invocation.
#[derive(Debug)]
pub struct EngineRequest<'a> {
    /// The immutable, fully acquired source snapshot.
    pub snapshot: &'a StaticLoadSnapshot,
    /// Parsed and edge-observed semantic choices.
    pub options: &'a EngineOptions,
    /// An admitted replay, or `None` for a live run.
    pub replay: Option<&'a Replay>,
    /// Diagnostics produced by source acquisition at the filesystem edge.
    pub acquisition_diagnostics: &'a [Diag],
}

/// What the engine asks the production or harness controller to observe after probe publication.
#[derive(Debug)]
pub struct ObservationRequest<'a> {
    /// The analyzed sources whose controller scope is minted.
    pub sources: RunSources<'a>,
    /// The deterministic spike framing for a hostless run.
    pub default_framing: &'a dorc_plan::records::Framing,
}

/// Host observation supplied by an injected controller edge.
#[derive(Debug)]
pub enum Observation {
    /// Fresh controller evidence, bounded before it crossed this seam.
    Controller {
        /// The controller-minted frame against which records are checked.
        framing: dorc_plan::records::Framing,
        /// Bounded host bytes, no observation, or an intake refusal.
        evidence: dorc_plan::records::Admission<dorc_plan::records::BoundedHostBytes>,
        /// Already sink-encoded remote stderr, in emission order.
        stderr: Vec<String>,
    },
    /// Already-admitted fixture results from the separate harness controller.
    Fixture {
        /// Typed results admitted before this adapter was called.
        results: SiteResults,
    },
    /// A terminal edge outcome that cannot license planning.
    Terminal {
        /// The status returned by the engine.
        status: EngineStatus,
        /// The diagnostic describing the terminal edge outcome.
        diagnostic: Diag,
    },
}

/// Nondeterministic and mutating operations injected into the deterministic engine.
pub trait EngineEdges {
    /// Materialize generated PATH shims, if the invocation requested that edge effect.
    ///
    /// # Errors
    /// Returns the edge diagnostic when publication fails.
    fn materialize_shims(&mut self, files: &BTreeMap<String, String>) -> Result<(), Box<Diag>>;
    /// Wait for host evidence. Called only after any round-trip probe write has been flushed.
    ///
    /// # Errors
    /// Returns the edge diagnostic when evidence cannot be acquired.
    fn observe(
        &mut self,
        request: &ObservationRequest<'_>,
        render_probe: &dyn Fn(&dorc_plan::records::Framing) -> String,
    ) -> Result<Observation, Box<Diag>>;
    /// Borrow the injected clock used for record and run instants.
    fn clock(&mut self) -> &mut RunClock;
    /// Resolve source revision metadata for a report.
    fn source_match(&mut self, book_name: &str) -> Option<SourceMatch>;
    /// Publish a generated artifact set.
    ///
    /// # Errors
    /// Returns a closed refusal word when publication fails.
    fn publish_artifact(&mut self, artifact: &ArtifactSet) -> Result<(), &'static str>;
    /// Publish generated whylog bytes.
    ///
    /// # Errors
    /// Returns a closed refusal description when publication fails.
    fn publish_whylog(&mut self, bytes: &[u8]) -> Result<(), String>;
    /// Display label for the configured durable destination.
    fn durable_label(&self) -> &str;
    /// Mint the invocation record at the production boundary, where raw process arguments live.
    fn invocation_record(
        &mut self,
        request: InvocationRecordRequest<'_>,
    ) -> dorc_core::spine::SpineInvocation;
}

/// Typed semantic facts needed to mint a durable invocation record.
#[derive(Debug)]
pub struct InvocationRecordRequest<'a> {
    /// The controller framing for this attempt.
    pub framing: &'a dorc_plan::records::Framing,
    /// The analyzed snapshot.
    pub snapshot: &'a StaticLoadSnapshot,
    /// The original run's start instant.
    pub started_at: Option<dorc_core::RunInstant>,
    /// The run's influence account.
    pub account: dorc_core::influence::InfluenceAccount,
}

/// A process-independent semantic completion status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineStatus {
    /// Analysis and requested publication completed.
    Complete,
    /// The book contained an unmodelled syntax or CFG construct.
    BookUnmodeled,
    /// A wrapper's authored members were incoherent.
    WrapperIncoherent,
    /// Host evidence admission refused.
    IngressRefused,
    /// No host process was created.
    HostNotReached,
    /// A host session ended without a completion marker.
    SessionLost,
    /// A remote apply completed non-zero.
    ApplyFailed,
    /// The requested artifact could not be produced or published.
    ArtifactUnservable,
    /// A computed load target remained unresolved.
    LoadUnresolvable,
}

impl EngineStatus {
    /// The process status used by every adapter for this semantic result.
    #[must_use]
    pub const fn exit_code(self) -> u8 {
        match self {
            Self::Complete => 0,
            Self::BookUnmodeled => 10,
            Self::WrapperIncoherent => 11,
            Self::IngressRefused => 12,
            Self::HostNotReached => 13,
            Self::SessionLost => 14,
            Self::ApplyFailed => 15,
            Self::ArtifactUnservable => 16,
            Self::LoadUnresolvable => 17,
        }
    }
}

type RunOutcome = EngineStatus;

/// Generated file state returned independently of where production published it.
#[derive(Clone, Debug)]
pub enum GeneratedOutput {
    /// The complete executable artifact set.
    Artifact(ArtifactSet),
    /// A serialized whylog durable.
    Whylog(Vec<u8>),
}

/// The shared engine's complete process-independent result.
#[derive(Debug)]
pub struct EngineResult {
    /// Semantic completion status.
    pub status: EngineStatus,
    /// Generated file values, in generation order.
    pub generated: Vec<GeneratedOutput>,
    /// The complete shared analyzed world, when this mode built one.
    pub world: Option<WhyWorld>,
}

/// The publication stream for plain output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputChannel {
    /// Sends text through standard output.
    Stdout,
    /// Sends text through standard error.
    Stderr,
}

/// A closed destination and presentation choice for one output event.
#[derive(Clone, Debug)]
pub enum OutputDestination {
    /// Plain output sent to the selected stream.
    Plain(OutputChannel),
    /// Styled diagnostic output, necessarily sent to standard error.
    Diagnostic {
        /// Stage name shown before the severity word.
        stage: String,
        /// Severity controlling terminal styling.
        severity: Severity,
        /// The typed diagnostic represented by the tagged body.
        diagnostic: Box<Diag>,
    },
}

/// One exact output body, optionally retaining renderer-owned provenance.
#[derive(Clone, Debug)]
enum OutputBody {
    Text(String),
    Tagged(RenderParts),
}

/// One ordered output publication from the shared invocation engine.
#[derive(Clone, Debug)]
pub struct OutputEvent {
    destination: OutputDestination,
    body: OutputBody,
}

impl OutputEvent {
    /// Constructs plain UTF-8 text for a selected stream.
    #[must_use]
    pub fn plain_text(channel: OutputChannel, text: impl Into<String>) -> Self {
        Self {
            destination: OutputDestination::Plain(channel),
            body: OutputBody::Text(text.into()),
        }
    }

    /// Constructs plain tagged text for a selected stream.
    #[must_use]
    pub fn plain_tagged(channel: OutputChannel, parts: RenderParts) -> Self {
        Self {
            destination: OutputDestination::Plain(channel),
            body: OutputBody::Tagged(parts),
        }
    }

    /// Constructs a tagged diagnostic event; diagnostics always target standard error.
    #[must_use]
    pub fn diagnostic(stage: impl Into<String>, diagnostic: Diag, parts: RenderParts) -> Self {
        Self {
            destination: OutputDestination::Diagnostic {
                stage: stage.into(),
                severity: diagnostic.severity(),
                diagnostic: Box::new(diagnostic),
            },
            body: OutputBody::Tagged(parts),
        }
    }

    /// Returns the destination stream, including diagnostic stream enforcement.
    #[must_use]
    pub fn channel(&self) -> OutputChannel {
        match self.destination {
            OutputDestination::Plain(channel) => channel,
            OutputDestination::Diagnostic { .. } => OutputChannel::Stderr,
        }
    }

    /// Returns diagnostic presentation metadata when present.
    #[must_use]
    pub fn diagnostic_presentation(&self) -> Option<(&str, Severity)> {
        match &self.destination {
            OutputDestination::Plain(_) => None,
            OutputDestination::Diagnostic {
                stage, severity, ..
            } => Some((stage, *severity)),
        }
    }

    /// Returns the typed diagnostic carried by a diagnostic event.
    #[must_use]
    pub fn diagnostic_payload(&self) -> Option<&Diag> {
        match &self.destination {
            OutputDestination::Diagnostic { diagnostic, .. } => Some(diagnostic.as_ref()),
            OutputDestination::Plain(_) => None,
        }
    }

    /// Returns the exact rendered text.
    #[must_use]
    pub fn text(&self) -> String {
        match &self.body {
            OutputBody::Text(text) => text.clone(),
            OutputBody::Tagged(parts) => parts.text(),
        }
    }

    /// Returns tagged renderer parts when the event retains them.
    #[must_use]
    pub fn tagged_parts(&self) -> Option<&RenderParts> {
        match &self.body {
            OutputBody::Text(_) => None,
            OutputBody::Tagged(parts) => Some(parts),
        }
    }
}

/// Render one typed diagnostic through the production diagnostic-event path.
#[must_use]
pub fn diagnostic_event(
    ctx: &dorc_aid::RenderCtx<'_>,
    stage: &str,
    diagnostic: &Diag,
    source: &str,
    filename: &str,
) -> OutputEvent {
    let parts = dorc_aid::diag::render_staged_cli_parts(
        stage,
        ctx,
        diagnostic,
        source,
        filename,
        &Interner::default(),
    );
    OutputEvent::diagnostic(stage, diagnostic.clone(), parts)
}

/// A live output boundary for the shared invocation engine.
pub trait OutputSink {
    /// The prose tables and frame used to construct tagged output.
    fn render_ctx(&self) -> dorc_aid::RenderCtx<'_> {
        dorc_aid::RenderCtx::production()
    }
    /// Publishes one event immediately.
    fn emit(&mut self, event: OutputEvent);
    /// Flushes the selected stream immediately.
    fn flush(&mut self, channel: OutputChannel);
}

/// An ordered collector for loom-facing tests.
#[derive(Clone, Debug, Default)]
pub struct OutputEvents {
    actions: Vec<OutputAction>,
}

/// One accepted output or flush action.
#[derive(Clone, Debug)]
pub enum OutputAction {
    /// An output event accepted by the sink.
    Event(OutputEvent),
    /// A channel flush accepted by the sink.
    Flush(OutputChannel),
}

impl OutputEvents {
    /// Iterates over accepted actions in order.
    pub fn iter(&self) -> std::slice::Iter<'_, OutputAction> {
        self.actions.iter()
    }

    /// Consumes the collector, preserving action order.
    #[must_use]
    pub fn into_actions(self) -> Vec<OutputAction> {
        self.actions
    }
}

impl<'a> IntoIterator for &'a OutputEvents {
    type Item = &'a OutputAction;
    type IntoIter = std::slice::Iter<'a, OutputAction>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl OutputSink for OutputEvents {
    fn emit(&mut self, event: OutputEvent) {
        self.actions.push(OutputAction::Event(event));
    }

    fn flush(&mut self, channel: OutputChannel) {
        self.actions.push(OutputAction::Flush(channel));
    }
}

macro_rules! emit_stderr {
    ($sink:expr, $($arg:tt)*) => {
        $sink.emit(OutputEvent::plain_text(
            OutputChannel::Stderr,
            format!("{}\n", format_args!($($arg)*)),
        ));
    };
}

fn chrome_parts(ctx: &dorc_aid::RenderCtx<'_>, slug: &'static str, values: &[&str]) -> RenderParts {
    let mut parts = crate::chrome_line_parts(ctx, slug, values);
    parts.push(dorc_aid::tagged::RenderPart::Arrangement {
        text: "\n".into(),
        slug: "cli-chrome-line-ending",
    });
    parts
}

/// Run one acquired invocation through the production semantic pipeline.
/// # Errors
/// Returns a diagnostic only when an injected production edge cannot perform its requested action.
pub fn run(
    request: &EngineRequest<'_>,
    edges: &mut dyn EngineEdges,
    sink: &mut dyn OutputSink,
) -> Result<EngineResult, Box<Diag>> {
    let mut generated = Vec::new();
    let mut world = None;
    let status = run_status(request, edges, sink, &mut generated, &mut world)?;
    Ok(EngineResult {
        status,
        generated,
        world,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "one linear semantic pipeline; splitting it would let production and harness ordering drift"
)]
fn run_status(
    request: &EngineRequest<'_>,
    edges: &mut dyn EngineEdges,
    sink: &mut dyn OutputSink,
    generated: &mut Vec<GeneratedOutput>,
    world_out: &mut Option<WhyWorld>,
) -> Result<EngineStatus, Box<Diag>> {
    let options = request.options;
    let replay = request.replay;
    let snapshot = request.snapshot;
    let mut interner = Interner::default();
    let mode = options.mode;
    // rec-1 advisory routing: `plan` and the legacy round-trip overlay the FULL advisory plane
    // on stderr (warnings, notes, the why-lens, the unresolvable readout); `apply` (the
    // off-ramp shippable) suppresses it, keeping only the error floor + digest. `probe`'s
    // stage diagnostics are advisory-or-error like any analysis run. tc-apply-receipt-floor:
    // WHERE this line falls (advisory-suppressed but error-kept, digest-kept) is the
    // load-bearing surface judgment — flagged to the conductor, not silently settled.
    let advisory = !matches!(mode, Mode::Apply);

    // ---- the shared, pure pipeline (one call-shape for every mode — the thin-driver
    // mandate: no mode branches the kernel; only the stdout/stderr ROUTING below differs) ----
    let book_src = snapshot.book_src().to_owned();
    let book_name = snapshot.book_path();
    let oracle_paths = snapshot.oracle_paths();
    let oracle_srcs = snapshot.oracle_srcs();
    let source_paths = snapshot.source_paths();
    let source_srcs = snapshot.source_srcs();
    // MODELLED text for every lift and index below — a `PlainInclusion` reads empty there, so it
    // lifts nothing, declares nothing, and indexes nothing (`snapshot::modelled_refs`, the one
    // selection seat). `source_srcs`/`oracle_srcs` above stay the REAL bytes, which is what the
    // mirroring, the diagnostics, and the durable want.
    let oracle_refs: Vec<&str> = snapshot.modelled_oracle_refs();
    let source_refs: Vec<&str> = snapshot.modelled_refs();
    let source_path_refs: Vec<&str> = source_paths.iter().map(String::as_str).collect();
    let book_index = Some(snapshot.book_index());

    // The book-free oracle-side lints, factored into one entry the lint rung-oracle-solo lane also
    // uses (`27S:seam-oracle-validate-factoring`); `wrapper_incoherent` is the pre-network fail-fast.
    let validation = dorc_oracle::validate::validate(&mut interner, &oracle_refs);
    let wrapper_incoherent = validation.wrapper_incoherent;
    if mode != Mode::Bundle {
        for stage in &validation.stages {
            let source = stage
                .file
                .and_then(|i| Some((oracle_paths.get(i)?.as_str(), oracle_srcs.get(i)?.as_str())));
            report_at(sink, advisory, stage.stage, source, &stage.diags);
        }
    }

    // The per-file PredictSets (the entity-resolution mechanism; shared interner — 204 seam #2). The
    // per-file `check`-dialect diags were emitted by `validate` above; the effect map (23D §1) is
    // built from these below, once they are withdrawn.
    let checks: Vec<dorc_oracle::predict::PredictSet> = source_refs
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
        .collect();

    // Pre-lift each file's verdict funcdefs so the (immutable-interner) probe ship-closure can
    // strip a verdict-lane site's body without a mutating re-lift (`24L` §2 probe emission). Diags
    // drop here — `validate` surfaces them once, per-file, for gate-3.
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = source_refs
        .iter()
        .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
        .collect();

    // Parse + analyze the book (shared interner, so symbols match the oracles). Multiple books
    // CONCATENATE into one analyzed unit (`\n`-joined so no two files' lines merge). `book_name`
    // is the display path (the first book) — for a single book (the norm) the frame's line numbers
    // are exact source lines; a multi-book unit's line numbers are into the concatenation.
    //
    // The unloaded-sibling-oracle hint (gap-5 / `24H` ack-6): a cli-edge, filesystem-reading disclosure.
    if mode != Mode::Bundle {
        report_at(
            sink,
            advisory,
            "oracle",
            None,
            request.acquisition_diagnostics,
        );
    }
    // `--last` desync guard (`22F` book-identity): re-read digests must match the durable's.
    // ack-8: the book-stage diags (parse/cfg/classify/probe/render) all span into `book_src`;
    // this pair feeds their file:line:col frames (rul24-lineno-identity — the SOURCE line space).
    let book_source = Some((book_name, book_src.as_str()));
    let parsed = dorc_syntax::parse(&book_src);
    report_at(sink, advisory, "parse", book_source, &parsed.diags);
    // The marker gate also covers a BOOK that HOSTS oracle functions (share-a-file): an unmarked
    // book carrying a bind/mark errors, while a stripped off-ramp artifact (dialect erased) stays
    // marker-free and only warns on the reserved-name squat below (guard23-reingest-collision).
    report_at(
        sink,
        advisory,
        "marker",
        book_source,
        &dorc_oracle::marker::check_dialect_marker(&mut interner, &book_src),
    );
    // The munge-reservation squat lint (24M rul24M-bare-dorcism-names): a book funcdef
    // coincidentally named `<x>__<role>` squats the reserved emitted namespace — surfaced LOUDLY
    // as a Warning (the loud-friend law; rul24-warnings-tune-high). The live corpus instance is
    // guard23-reingest-collision-verbatim's `apt_get__predict` book function. Warning-severity, so
    // it never fails a case (gate-3 keys on `error[`).
    report_at(
        sink,
        advisory,
        "reserved",
        book_source,
        &dorc_oracle::reserved::lint_book_reserved_names(&parsed.value),
    );
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    report_at(sink, advisory, "cfg", book_source, &cfg.diags);
    // ack-1 exit-code family: a book carrying a parse/CFG ⊤-reject (`inv-top-reject`) — a
    // syntax-unsupported/malformed construct, or its downstream CFG ⊤-node — leaves the analysis
    // built on partial understanding. The artifact still ships byte-identically (the stdout
    // fence), but the process fast-fails with EXIT_BOOK_UNMODELED so a `dorc … && deploy` chain
    // STOPS. Keyed on Error-severity from the parse OR cfg stage (the only Error codes there are
    // syntax-{unsupported,malformed} and cfg-top-node — the "book cannot be modeled" set); a
    // render-refusal / oracle-lift Error is kFAIL-safe and does NOT fast-fail (the artifact is
    // valid). Computed here (before the `probe` early-return) so every mode signals it.
    let book_unmodeled = parsed
        .diags
        .iter()
        .chain(cfg.diags.iter())
        .any(|d| d.severity() == Severity::Error);
    let mut book_outcome = if book_unmodeled {
        RunOutcome::BookUnmodeled
    } else if wrapper_incoherent {
        // Dual-peel incoherence is pre-network fail-fast (`273` §5), independent of the book;
        // ranked after book-unmodeled only because the exit codes are distinct signals.
        RunOutcome::WrapperIncoherent
    } else {
        RunOutcome::Complete
    };
    // Book-side value-flow: resolve each command-site's argv (constant/variable
    // propagation) — the input entity-resolution consumes (19H §1 / 202 §1).
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);

    // ── `28K` §1/§2: the function environment, and the cross-unit shadow refusal ──
    //
    // Computed ONCE from the ORIGIN model, joining the FROZEN set: the fixpoint's ratchet erases
    // EFFECTS and holds no authority over BINDINGS (`the-frozen-set-includes-the-function-environment`).
    let definitions = definition_table(snapshot, &parsed.value);
    let env = {
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        dorc_analysis::funcenv::analyze(&parsed.value, &cfg.value, &definitions, &plane)
    };
    // The computed `.` at its ruled TIER: post-analysis, pre-network, whole-run
    // (`30P:rul-floor-valid-text-never-parse-fails` kept the outcome and moved only the tier).
    if matches!(book_outcome, RunOutcome::Complete)
        && env.havoc_causes().values().any(|cause| {
            matches!(
                cause,
                dorc_analysis::funcenv::HavocCause::ComputedSubstitution
            )
        })
    {
        book_outcome = RunOutcome::LoadUnresolvable;
    }
    if mode == Mode::Bundle {
        let Ok(projection) = crate::bundle::project(snapshot, env.loads()) else {
            return Ok(RunOutcome::BookUnmodeled);
        };
        let load_acts =
            crate::provenance::LoadActs::of(snapshot, &cfg.value, &parsed.value, env.loads());
        for stage in &validation.stages {
            for diagnostic in &stage.diags {
                report_bundle_diagnostic(
                    sink,
                    advisory,
                    stage.stage,
                    snapshot,
                    projection.projection(),
                    &load_acts,
                    stage.file.map_or(
                        BundleDiagnosticSite::Unlocated,
                        BundleDiagnosticSite::EveryOccurrence,
                    ),
                    diagnostic,
                );
            }
        }
        for diagnostic in projection.diagnostics() {
            report_bundle_diagnostic(
                sink,
                advisory,
                "bundle",
                snapshot,
                projection.projection(),
                &load_acts,
                BundleDiagnosticSite::Exact(diagnostic.file()),
                diagnostic.diag(),
            );
        }
        if wrapper_incoherent {
            return Ok(book_outcome);
        }
        sink.emit(OutputEvent::plain_text(
            OutputChannel::Stdout,
            projection.projection().render_archive(),
        ));
        sink.flush(OutputChannel::Stdout);
        return Ok(book_outcome);
    }
    // ABOVE the probe emission on purpose: the planner's inputs are authored-before-contact, so an
    // unservable form refuses with nothing probed, contacted or written (`30I` §10).
    let form_selection = match select_artifact_form(
        options,
        snapshot,
        &cfg.value,
        &parsed.value,
        &book_src,
        &env,
    ) {
        Ok(selection) => selection,
        Err(refusal) => {
            report_at(
                sink,
                advisory,
                "emission",
                None,
                &[Diag::new_spanless_site(DiagCode::ArtifactFormRefused(
                    dorc_aid::diag::ArtifactFormRefused {
                        form: refusal.form(),
                        cause: refusal.cause(),
                        loads: refusal.loads(),
                    },
                ))],
            );
            return Ok(RunOutcome::ArtifactUnservable);
        }
    };
    // One non-role-declaration index per unit, consulted by every seat that emits a body (`28K` §4).
    // The book is the LAST source, and naming it is what lets the custody predicate see what the
    // admin defines (`rul-vouch-reaches-own-custody-only`). Sited BELOW the environment because the
    // include-tree is now the loader's own account of the loads it followed, not a second walk over
    // literal operands (`crate::sourcing::include_tree`).
    let include_tree = crate::sourcing::include_tree(snapshot, &env);
    let helpers = dorc_oracle::closure::HelperIndex::build(&source_refs, book_index)
        .with_include_tree(
            dorc_core::CustodyClosures::from_edges(source_refs.len(), &include_tree.edges),
            include_tree.unresolved.clone(),
        )
        .with_selection(dorc_core::CustodyClosures::from_edges(
            source_refs.len(),
            &include_tree.selected,
        ));
    let shadows = dorc_analysis::funcenv::contests(&parsed.value, &cfg.value, &definitions, &env);
    let unprovable = dorc_analysis::funcenv::unprovable(&definitions, &env, cfg.value.exit());
    // `28K` §2 rul-visibility-is-full-positional — solved ONCE here, beside the whole-unit
    // refusal, and carried (never re-derived) into the frozen model.
    let live_defs = dorc_analysis::funcenv::LiveDefinitions::new(&env, &definitions);
    // The license-plane fact is minted FIRST; the diagnostics derive FROM it, never the reverse
    // (`two-plane-aid-law`). Two sources feed it and only the first complains: a PROVEN shadow,
    // and a ⊤ binding (rider 1 `⊤-licenses-nothing`, silent — that is what lets it under-fire).
    let contested = dorc_core::ContestedFamilies::new(
        shadows
            .iter()
            .map(|c| c.name.as_str())
            .chain(unprovable.iter().map(String::as_str))
            .filter_map(|name| {
                dorc_oracle::reserved::role_family(name).map(|(base, _)| base.to_owned())
            }),
    );
    // MINTED HERE, above the intake, so nothing influenced can reach them by accident later; the
    // Spine write below is transcription, never the mint (`tc-load-decisions-read-authored`).
    let load_decisions = mint_load_decisions(&cfg.value, &contested, &env);
    // `302` §4 — the two pre-network solve seats, reported the moment they give up: both run
    // before the probe is compiled, so this is fail-fast in the project's sense (loud, on human
    // timescales), and the plan that follows is the honest floor rather than nothing.
    let (consistency_diags, consistency_narrative) = solve_consistency_reports(&value, &env);
    report_at(sink, advisory, "solve", book_source, &consistency_diags);
    // `302:rul-certifier-trip-guard-only` — ONE latch per analysis spine, opened at the first
    // seat that can set it and carried to the terminal cleanup below. The two pre-network solves
    // record here; every classify round (origin and fixpoint alike) records its own.
    let mut trip = dorc_analysis::certify::CertifierTrip::default();
    record_pre_network_trip(&mut trip, &value, &env);
    let shadow_narrative = shadow_narratives(&shadows, &definitions);
    for (file, diags) in shadow_diagnostics(&shadows, &definitions, source_paths, &source_refs) {
        let source = source_paths
            .get(file)
            .zip(source_refs.get(file))
            .map(|(path, src)| (path.as_str(), *src));
        report_at(sink, advisory, "loading", source, &diags);
    }
    report_at(
        sink,
        advisory,
        "loading",
        book_source,
        &positional_loading_notices(&parsed.value, &cfg.value, &value, &interner, live_defs),
    );
    report_at(
        sink,
        advisory,
        "loading",
        book_source,
        &load_head_notices(&parsed.value, &cfg.value, &env, &book_src),
    );
    for (file, diags) in helper_conflict_diagnostics(&helpers, source_paths, &source_refs) {
        let source = source_paths
            .get(file)
            .zip(source_refs.get(file))
            .map(|(path, src)| (path.as_str(), *src));
        report_at(sink, advisory, "loading", source, &diags);
    }
    // The withdrawal, applied ONCE to the lifted sets so no downstream consumer has to remember to
    // ask: a contested family becomes indistinguishable from one nobody described.
    //
    // It used to carry a SECOND withdrawal beside the contested one — per file, the roles whose
    // definition there the environment proves binds nowhere. The frame conversion retired it
    // (`28Q` §1): a never-live definition is live at no frame, so no resolution seat can select its
    // rows, and subtracting them bought nothing a lookup was not already doing. The liveness itself
    // is still owed to the one seat resolution does not cover — `build_dialect`'s whole-unit fold —
    // and travels there as data (`binds_somewhere`) rather than as a missing row.
    let never_live = dorc_analysis::funcenv::never_live(&definitions, &env);
    let checks: Vec<dorc_oracle::predict::PredictSet> = checks
        .into_iter()
        .map(|set| set.withdrawing(&contested, &interner))
        .collect();
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = verdict_sets
        .into_iter()
        .map(|set| set.withdrawing(&contested, &interner))
        .collect();
    let dead_predicts = never_live_predict_rows(&never_live, &checks, &interner);
    let idx = dorc_oracle::lift_from_sets(&mut interner, &checks, |file, provider| {
        !dead_predicts.contains(&(file, provider))
    })
    .value
    .withdrawing(&contested, &interner);
    // The `24L` §7 kernel seam, widened by `26H` §3: the kernel stays verdict-unaware, so the edge
    // keys the role by provider and threads it in as DATA. From the sets above ⇒ ONE lift.
    let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);

    // The per-run receipts plane (arch-1): give-up causes (`Top(cause)`) and license
    // witnesses land here. EXEMPT — it informs no decision (the `plan::erasability` gate
    // proves the apply/probe artifacts are byte-identical with it stripped); the cli holds it
    // only to emit the decision-digest line and (future) the why-lens.
    let mut arena = ProvArena::new();
    // stage-3 (the why-lens): take the TYPED cmdsub-⊤ disclosures too — `report`/gate-3 consume the
    // LOWERED `diags` (cause-dropped), but the why-lens render reads the `cause` off the typed
    // `Diag`s (`to_legacy` drops it). The arena is shared (the typed diags' causes resolve in it).
    // `kills` (R3 / 24A §3): the kill-bearing leaf set the wall predicate cannot read off the
    // `MustRun` SkipClass alone. Threaded to `build_plan_walled` so a running `apt-get purge`
    // walls downstream, closing the kill gap fd10's establish-only wall left open. `kill_coords`
    // (24E §7): each single-kill node's killed coordinate — the kill-wall coherence comparand
    // (own-killed-coord ⊆ footprint), closing resid-kill-coherence.
    // `fact_backings` (`277` §5 backing-SETS): each establish fact's survival-backing provenance
    // — its minting family + observe-backing-widening selectors — threaded to `build_plan_walled`
    // so the survival tier builds each fact's backing SET (a widened backing GROWS kill-surface).
    // `27N` — peel wrapped BOOK sites into (inner command, composed context) + decide entry. Empty
    // for a wrapper-free run ⇒ the whole pipeline is byte-identical (`empty-world-byte-identical`).
    // THE EDGE, for the wrapper lane's own two members: lifted and withdrawn in one call, so a
    // contested wrapper family is gone from the peel model, the lend map, and the entry-form bytes
    // before any site is considered (`withdrawal-is-applied-once-never-consulted`).
    let wrapper_sets = WrapperSets::lift(&source_refs, &mut interner, &contested);
    // The escalation-POLICY disclosure (`27C:render-authority-disclosure`): one advisory line naming
    // the escalation posture (the dial × the connection capability) and the entry-capable wrappers
    // loaded. Consent legibility — the admin sees, once, what authority the probe re-uses. It sits
    // BELOW the withdrawal because it reads the withdrawn vectors: a family whose sites now wall must
    // not narrate as entry-capable.
    report_at(
        sink,
        advisory,
        "escalation",
        None,
        &escalation_policy_diagnostics(
            &checks,
            &wrapper_sets,
            options.dial(),
            options.capability(),
        ),
    );
    let wrapped_analysis = build_wrapped_analysis(
        source_srcs,
        &source_refs,
        source_paths,
        &helpers,
        &checks,
        &verdict_sets,
        &wrapper_sets,
        &parsed.value,
        &cfg.value,
        &value,
        options.dial(),
        options.capability(),
        &mut interner,
        live_defs,
    );
    let peeled_sites = wrapped_analysis.peeled;
    let wrapped_probes = wrapped_analysis.wrapped;
    let carried_attribution = wrapped_analysis.carried;
    let entry_narrative = wrapped_analysis.collapse_narrative;
    report_at(
        sink,
        advisory,
        "wrapped",
        book_source,
        &wrapped_analysis.hints,
    );
    // `degrades` (`26G:fnd-existence-gate-darkens-oracle`): why each ⊤-degrading site's oracle
    // check gave up. Diagnostics only — it reaches the `site-unresolvable` note and nothing else.
    let mut degrades = BTreeMap::new();
    // `26H` §3.5 — sites whose establish came from the VERDICT lane, so their probe ships the
    // verdict body. Site-keyed: nothing about the FACT distinguishes an authored verdict cell.
    let mut verdict_lane = BTreeMap::new();
    let frozen = FrozenModel {
        cfg: &cfg.value,
        value: &value,
        ast: &parsed.value,
        idx: &idx,
        checks: &checks,
        verdicts: &verdicts,
        peeled: &peeled_sites,
        live: live_defs,
    };
    let origin = classify_round(
        &frozen,
        &dorc_analysis::erase::ErasedSites::none(),
        &mut interner,
        &mut arena,
        &mut degrades,
        &mut verdict_lane,
        &mut trip,
    );
    let classes = origin.classes.clone();
    let kills = origin.kills.clone();
    let kill_coords = origin.kill_coords.clone();

    // `302` §4 — the ORIGIN round is PRE-NETWORK, so its consistency failures are reported HERE,
    // not with the rest of the classify diags far below (R3, cross-lineage review). That later
    // seat sits past the `Mode::Probe` return AND past `ship_probe`, so a reaching-defs or
    // self-reach give-up known before any host was touched would have been disclosed late in a
    // host-backed run and not at all in `probe` mode — the floor stayed conservative, but the
    // posture this lane specified was not kept. The fixpoint rounds keep the batched post-probe
    // surface; only the pre-network round moves.
    let origin_consistency_diags = consistency_diags_of(&origin.diags);
    report_at(
        sink,
        advisory,
        "solve",
        book_source,
        &origin_consistency_diags,
    );
    let origin_consistency_narrative = consistency_narratives_of(&origin.classify_narrative);

    // The per-site guard VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c) —
    // ALWAYS-ON (guards are the un-flagged baseline; rul24-mode-gate governs only the survival
    // tier, NOT this). A vouched past-wall establish ships its read-only probe (the witness needs
    // the verdict) and, converged, mints a `Disposition::Guard`.
    // Lift diags drop here: `validate` above surfaces them per-file. This lane could only report
    // them sourceless, which framed every verdict give-up at a fileless `1:1`.
    let vouch_lift = build_vouches(
        &source_refs,
        &source_path_refs,
        &verdict_sets,
        &helpers,
        &classes,
        &value,
        &mut interner,
        live_defs,
    );
    let (mut vouches, vouch_aid) = vouch_lift.value;
    let decline_narrative = vouch_aid.narrative;
    // The composition suspensions ride the LOAD-edge report stream, where a diagnostic's span is
    // file-qualified (`AID:law-lineno-identity`) — the same seat the helper-collision report uses,
    // for the same reason: both are one sentence about a composition, not one per site.
    for (file, diag) in vouch_aid.suspensions {
        let source = source_paths
            .get(file)
            .zip(source_refs.get(file))
            .map(|(path, src)| (path.as_str(), *src));
        report_at(sink, advisory, "vouching", source, &[diag]);
    }
    // `27N` — wrapped-entering sites vouch on the INNER verdict over the peeled argv (argv[0] is the
    // wrapper word, invisible to `build_vouches`). Disjoint nodes ⇒ a plain merge.
    vouches.extend(dorc_plan::build_wrapped_vouches(
        &verdict_sets,
        &classes,
        &wrapped_probes,
        &mut interner,
        live_defs,
    ));

    // `30Qe:fruit-oracle-matched-zero-sites` — aggregated from the run's own final, frozen `vouches`.
    report_at(
        sink,
        advisory,
        "oracle",
        None,
        &oracle_matched_zero_sites_diagnostics(oracle_paths, &verdict_sets, &vouches, &interner),
    );

    // The CONNECTED check-pipes (`24J` §2, repaired — `271:rul-only-oracle-bytes-ship`): a simple
    // all-vouched-read-only pipeline `A | F [| F…]` ships as ONE composed probe keyed to its
    // governing (last) stage — each stage replaced by its oracle's stripped predict; the non-last
    // stages are subsumed. `connected_check_pipes` is the DECIDER: it resolves each stage + applies
    // the per-channel coverage rule (rider 1 — a non-last stage must produce REAL stdout), refusing
    // any compound whose stage can't be model-substituted (⇒ its stages run). Empty for a book with
    // no such pipe. Threaded into BOTH the probe compiler (ship the composed body) and the plan
    // builder (omit the subsumed members).
    let ship_stage = |n, p, a: &[Symbol]| {
        ship_predict_stage(
            source_srcs,
            &helpers,
            &checks,
            &interner,
            p,
            a,
            n,
            live_defs,
        )
    };
    let connected =
        dorc_plan::connected_check_pipes(&parsed.value, &cfg.value, &value, &classes, ship_stage);

    // The read-only, SELF-REPORTING, site-keyed probe (R3 / 23D §1 — the check IS the oracle):
    // each site ships its provider's stripped `<provider>__predict` invoked with the site's argv.
    // `is_vouched` closes strain-classify-coupling (24C): a vouched past-wall `EstablishProbeWritten`
    // site ships its probe here (at HEAD it would be `unresolvable-no-probe`).
    let ship = |n, p, a: &[Symbol]| {
        ship_predict_body(
            source_srcs,
            &helpers,
            &checks,
            &interner,
            p,
            a,
            n,
            live_defs,
        )
    };
    // `24L` §2 — a VERDICT-LANE site ships the oracle's own `is_converged` funcdef, strip-only
    // (rul-only-oracle-bytes-ship). Keyed on the SITE's lane, never its fact's KIND (`26H` §3.5):
    // an authored verdict cell is an ordinary kind, so `is_auto_kind` would route it to the
    // predict lane, find nothing, and run the site. Try-order cannot stand in either —
    // `command_effect` reaches this lane from two fallbacks, and the second leaves a shippable
    // predict on a site whose cell the verdict body owns.
    let ship_auto = |node: dorc_analysis::cfg::CfgNodeId,
                     subjects: &[dorc_core::FactKey],
                     p: Symbol,
                     _a: &[Symbol]|
     -> Option<dorc_plan::ShippedCheck> {
        if verdict_lane
            .get(&node)
            .is_none_or(|measurement| measurement.subjects() != subjects)
        {
            return None;
        }
        ship_verdict_body(
            source_srcs,
            &helpers,
            &verdict_sets,
            &interner,
            p,
            node,
            live_defs,
        )
    };
    let probe = dorc_plan::compile_probe(
        &parsed.value,
        &cfg.value,
        &value,
        &classes,
        &wrapped_probes,
        &connected,
        ship,
        ship_auto,
        |node, fact| vouches.get(node, fact).is_some(),
    )
    .with_unresolvable_causes(&parsed.value, &cfg.value, &classes, &degrades);

    edges.materialize_shims(&probe.shim_files())?;

    // The DERIVATION-probe (24E §2 corr-§2 — the SECOND probe-shipping path, a NEW pipeline
    // stage): under `--risk-faultless-skips`, a wall-candidate whose `touches()` body ESCALATED (it
    // reached a host query the static `evaluate_touches` could not resolve) ships that body into
    // phase-1, runs read-only, and its stdout coord-lines are read back into a `Derived` footprint
    // (merged below, pre-`build_plan_walled`). Lifted for the derivation lane here; the authored
    // lane (`build_survival_footprints`) lifts its own — both pure + cheap, and a clean oracle
    // reports no touches diag either way (fork-s4-compile: a parallel compiler, NOT a `compile_probe`
    // extension — different site-set/body-source/readback, the convergence path left unperturbed).
    let touches_paired: Vec<(&str, dorc_oracle::touches::TouchesSet)> =
        if options.risk_faultless_skips() {
            pair_touches_sets(&oracle_refs, &mut interner, &contested)
        } else {
            Vec::new()
        };
    let derivations = if options.risk_faultless_skips() {
        let derive = |n, p, a: &[Symbol]| {
            ship_touches_body(&touches_paired, &helpers, &interner, p, a, n, live_defs)
        };
        dorc_plan::compile_derivations(&parsed.value, &cfg.value, &value, &classes, &kills, derive)
    } else {
        dorc_plan::DerivationPlan::default()
    };

    // The RESOLVER-probe (24F §3 — the identity CANONICALIZATION lane, a THIRD phase-1 shipping path).
    // Lift the per-kind resolvers + enforce confusability ALWAYS (an oracle-authoring correctness
    // check, flag-independent). The round-trip (coord enumeration + probe compile) is flag-on: for
    // each resolver-bearing coordinate (footprint + backing sides) ship `<kind>.resolve()` to
    // canonicalize its entity; the `resolv` readback builds the `Resolutions` merged pre-survival-walk.
    // The RAW coordinate kinds — used to re-key the munged kind-keyed resolver/reaches maps
    // (`flag-forward-munge-keying`: funcdefs are named by the kind's forward-munge, coords carry the
    // raw dotted kind, so the two are bridged here once).
    let coord_kinds = {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_coord_kinds(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &mut interner,
            live_defs,
        )
    };
    let resolver_lift = build_kind_resolvers(
        oracle_srcs,
        &checks,
        &touches_paired,
        &coord_kinds,
        &mut interner,
    );
    report_at(sink, advisory, "resolve", None, &resolver_lift.lift);
    report_by_oracle_file(
        sink,
        advisory,
        "resolve",
        oracle_paths,
        oracle_srcs,
        &resolver_lift.confusability,
    );
    let kind_resolvers = resolver_lift.value;
    let resolver_kinds: BTreeSet<Symbol> = kind_resolvers.resolver_kinds().collect();
    let resolver_coords = if options.risk_faultless_skips() && !resolver_kinds.is_empty() {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_resolver_coords(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &resolver_kinds,
            &mut interner,
            live_defs,
        )
    } else {
        BTreeSet::new()
    };
    let resolvers = compile_resolvers(
        &resolver_coords,
        &kind_resolvers,
        oracle_srcs,
        &helpers,
        &interner,
    );

    // The REACH-probe (24G §4 — the reaches() EXPANSION lane, a FOURTH phase-1 shipping path). Lift
    // the per-kind reach-functions + enforce confusability ALWAYS (kind-keyed like the resolver). The
    // round-trip (dynamic-arm shipping) is flag-on: for each reach-bearing AUTHORED footprint coord,
    // ship each DYNAMIC arm strip-clean, invoked with the entity; the `reach` readback expands the
    // footprints (via `Footprint::add_reached`) before the survival walk. STATIC arms never ship.
    let reaches_lift = build_kind_reaches(
        oracle_srcs,
        &checks,
        &touches_paired,
        &coord_kinds,
        &mut interner,
    );
    report_at(sink, advisory, "reaches", None, &reaches_lift.lift);
    report_by_oracle_file(
        sink,
        advisory,
        "reaches",
        oracle_paths,
        oracle_srcs,
        &reaches_lift.confusability,
    );
    let kind_reaches = reaches_lift.value;
    let reach_kinds: BTreeSet<Symbol> = kind_reaches.reach_kinds().collect();
    let reaches_plan = if options.risk_faultless_skips() && !reach_kinds.is_empty() {
        let touches_sets: Vec<_> = touches_paired.iter().map(|(_, s)| s.clone()).collect();
        collect_reach_probes(
            &classes,
            &kills,
            &value,
            &touches_sets,
            &kind_reaches,
            &reach_kinds,
            oracle_srcs,
            &helpers,
            &mut interner,
            live_defs,
        )
    } else {
        dorc_plan::ReachPlan::default()
    };

    // `probe` mode stops here: emit the probe artifact and return. It reads no stdin (no
    // results exist yet — this is phase 1, what you ship to GET them), builds no plan, and so
    // emits no apply, no why-lens, no digest (there is no plan/identity-plane to hash —
    // tc-probe-no-digest, flagged). Stage diagnostics above already routed to stderr.
    // `262` §2 framing: the run's records-lane keys, minted at THIS controller edge (the DI
    // seam — `inv-determinism`, no ambient RNG in the kernel). The nonce/host/attempt are the
    // spike fixed defaults (deterministic goldens; a real fleet mints per-attempt/per-host);
    // `book=` binds the stream to the exact analyzed book bytes. The end-sentinel follows the
    // final record lane (`records::sentinel_line`); the drain keys on it, never on EOF.
    // A function of the framing because `attempt=` is baked into these bytes: a retry must
    // RE-RENDER, never re-send (`26A` amend-retry-hygiene).
    let render_probe_artifact = |f: &dorc_plan::records::Framing| -> String {
        let mut out = probe.render_sh(f, &interner);
        out.push_str(&derivations.render_sh(f.nonce(), &interner)); // 24E §2: SAME phase-1 block
        out.push_str(&resolvers.render_sh(f.nonce())); // 24F §3: SAME phase-1 block
        out.push_str(&reaches_plan.render_sh(f.nonce())); // 24G §4: SAME phase-1 block
        out.push_str(&dorc_plan::records::sentinel_line(f.nonce()));
        out
    };
    let framing = dorc_plan::records::Framing::spike(book_digest(&book_src));
    if mode == Mode::Probe {
        sink.emit(OutputEvent::plain_text(
            OutputChannel::Stdout,
            render_probe_artifact(&framing),
        ));
        sink.flush(OutputChannel::Stdout);
        return Ok(book_outcome);
    }

    // The round-trip emits the probe FIRST (phase 1 on stdout), then the apply (phase 2)
    // after stdin EOF — the e2e harness splits the two on the `#!/bin/sh` shebang. `plan`
    // and `apply` emit ONLY the apply artifact (the probe is an internal compile there).
    if mode == Mode::RoundTrip {
        sink.emit(OutputEvent::plain_text(
            OutputChannel::Stdout,
            render_probe_artifact(&framing),
        ));
        sink.flush(OutputChannel::Stdout);
    }

    let run_sources = RunSources {
        book_name,
        book: &book_src,
        oracle_paths,
        oracle_sources: oracle_srcs,
    };
    let observation = if replay.is_none() {
        Some(edges.observe(
            &ObservationRequest {
                sources: run_sources,
                default_framing: &framing,
            },
            &render_probe_artifact,
        )?)
    } else {
        None
    };
    let (framing, evidence, fixture_results) = match observation {
        None => (framing, dorc_plan::records::Admission::NoObservation, None),
        Some(Observation::Controller {
            framing,
            evidence,
            stderr,
        }) => {
            for line in stderr {
                sink.emit(OutputEvent::plain_text(
                    OutputChannel::Stderr,
                    format!("{line}\n"),
                ));
            }
            (framing, evidence, None)
        }
        Some(Observation::Fixture { results }) => (
            framing,
            dorc_plan::records::Admission::NoObservation,
            Some(results),
        ),
        Some(Observation::Terminal { status, diagnostic }) => {
            report_at(sink, advisory, "transport", None, &[diagnostic]);
            return Ok(status);
        }
    };
    let scope = crate::results::replay_scope(&framing, &run_sources);
    // The authority to produce an authority-bearing projection rides out of the intake beside the
    // records (`306b:rul-report-only-output-cannot-plan`). It is a value rather than a check: the
    // refusal arm below returns, and no arm that continues can reach a plan without holding one.
    let (admitted_records, scoped_results, whylog_eligible, authority) =
        if let Some(results) = fixture_results {
            (
                None,
                crate::results::scope_fixture_results(&framing, &run_sources, results),
                false,
                dorc_plan::PlanAuthority::without_intake(),
            )
        } else if let Some(r) = replay.as_ref() {
            let scoped = crate::results::replayed_records(
                scope,
                r.records.as_ref(),
                &mut RunClock::Recorded(r.instants.clone()),
                &mut interner,
            );
            (
                None,
                scoped,
                false,
                dorc_plan::PlanAuthority::of_admitted_replay(),
            )
        } else {
            let admitted = match evidence {
                dorc_plan::records::Admission::Admitted(bytes) => {
                    crate::results::admit_controller_records(
                        &framing,
                        &run_sources,
                        &bytes,
                        edges.clock(),
                        &mut interner,
                    )
                }
                dorc_plan::records::Admission::NoObservation => {
                    dorc_plan::records::Admission::NoObservation
                }
                dorc_plan::records::Admission::Refused(reason) => {
                    dorc_plan::records::Admission::Refused(reason)
                }
            };
            match dorc_plan::PlanAuthority::authorise(admitted) {
                dorc_plan::Authorised::Admitted(admitted, authority) => {
                    (Some(admitted.records), admitted.scoped, true, authority)
                }
                dorc_plan::Authorised::NoObservation(authority) => (
                    None,
                    crate::results::no_observation(scope),
                    false,
                    authority,
                ),
                // The report-only state: intake integrity is lost, so this arm holds no authority and
                // the plan-producing projection is not reachable from it. The return is what the engine
                // does with that state today; the absent witness is what makes it safe.
                dorc_plan::Authorised::Refused(reason) => {
                    report_at(
                        sink,
                        advisory,
                        "records",
                        None,
                        &[reason.spanless_diagnostic()],
                    );
                    return Ok(RunOutcome::IngressRefused);
                }
            }
        };
    let _scope = scoped_results.scope();
    // The run's own account, read ONCE at the driver seat that holds the carrier: everything below
    // is downstream of intake, so every record joins from here rather than re-deriving where the
    // run stands (`30Qd:fnd-two-drivers-compute-one-fact-twice`).
    let world_account = scoped_results.account();
    let results = scoped_results.results();

    // re-key the site-keyed records to the FactKey-keyed observations `build_plan`
    // consumes (its fold/elision machinery is fact-keyed; only this probe-answer
    // plumbing re-keys — `inv-site-keyed-results`). The probe's `checks` carry each
    // site's resolved fact + its `site_kind`, so a site-record maps site→fact AND the
    // firewall knows whether the rc is fold-usable. CRITICAL (the wrong-concrete
    // firewall, 202 §3 / task-D2): a record's `rc` feeds the fold's Status ONLY for a
    // VALID Query-class site (the guard's own rc); an establish site's rc is the PROBE
    // command's (dpkg-query's), NOT the mutator's, so it feeds the fold NOTHING.
    let probe_origins = probe_origins(&probe, results, &mut arena);

    // The survival tier (Stage 2 / rul24-mode-gate, TC-1): footprints are lifted ONLY under
    // `--risk-faultless-skips` — off ⇒ `None` ⇒ the honest Stage-1 total wall, the data never exists.
    let survival = options.risk_faultless_skips().then(|| {
        let touches = lift_touches_sets(&oracle_refs, &mut interner, &contested);
        report_at(sink, advisory, "touches", None, &touches.diags);
        let lifted = build_survival_footprints(
            &touches.value,
            &classes,
            &kills,
            &kill_coords,
            &value,
            &cfg.value,
            &parsed.value,
            &mut interner,
            live_defs,
        );
        report_at(sink, advisory, "footprint", None, &lifted.diags);
        let mut fps = lifted.value;
        // 24E §2 corr-§2: merge the host-DERIVED footprints (read back from the phase-1
        // derivation-probe's `deriv` coord-records) into the authored set, before the survival
        // walk. An escalated site has NO authored footprint (its static trace ⊤'d), so the two
        // sets are disjoint by construction — no collision.
        // The escalated sites' book spans (`aid-caret-span-precision`), precomputed at the edge (the
        // merge runs interner-only): each derivation's `CfgNodeId`→AST span, total by construction.
        let derived_node_spans: BTreeMap<_, _> = derivations
            .derivations
            .iter()
            .map(|d| (d.node, parsed.value.node(cfg.value.node(d.node).ast).span))
            .collect();
        report_at(
            sink,
            advisory,
            "derive",
            book_source,
            &merge_derived_footprints(
                &mut fps,
                &derivations,
                results,
                &classes,
                &kill_coords,
                &derived_node_spans,
                &mut interner,
            ),
        );
        // 24G §4: EXPAND each reach-bearing footprint coord via reaches(), after the
        // authored/derived merge and before the walk. An arm that cannot show it finished refuses
        // the footprint — see `expand_footprints_via_reaches`.
        let reach_node_spans: BTreeMap<_, _> = fps
            .nodes()
            .map(|n| (n, parsed.value.node(cfg.value.node(n).ast).span))
            .collect();
        report_at(
            sink,
            advisory,
            "reach",
            book_source,
            &expand_footprints_via_reaches(
                &mut fps,
                &kind_reaches,
                &reach_kinds,
                results,
                &reach_node_spans,
                &mut interner,
            ),
        );
        fps
    });
    // 24F §3: build the identity-canonicalization map from the `resolv` readback (both footprint and
    // backing coords canonicalized in the survival walk). Flag-off / no-resolver ⇒ empty ⇒ the
    // token-equality floor (identical to today). §4: each DANGLING coordinate is a loud diagnostic.
    let mut resolutions =
        build_resolutions(&resolver_coords, &resolver_kinds, results, &mut interner);
    // fence-no-disjoint (`24L` §7): register every verdict-provider's auto-cell kind so the survival
    // tier reads an auto coordinate as may-touch (`survival::disjoint`). The plan is interner-free,
    // so this resolution happens here (the edge holds the interner) and rides the Resolutions the
    // walk already threads. Re-interning `dorc-auto:<provider>` returns the KindId classify minted.
    //
    // Still per-PROVIDER now that a verdict body can also key an authored cell: the fence guards
    // the synthetic singleton, so every provider that could mint one keeps it. Whole-unit and
    // file-blind — registering only widens may-touch, so covering every file errs toward safety.
    let verdict_names: Vec<String> = verdicts
        .providers()
        .map(|(_, p)| interner.resolve(p.0).to_owned())
        .collect();
    for name in verdict_names {
        let kind = dorc_core::auto_fact(&mut interner, &name).kind;
        resolutions.add_auto_kind(kind);
    }
    report_at(
        sink,
        advisory,
        "resolve",
        None, // dangling-reference notes are spanless (no book/oracle location)
        &dangling_diagnostics(&resolutions, &interner),
    );
    // THE SETTLEMENT (`30K` §4.2): one grow-only loop from the frozen world to one certified set of
    // decisions. It runs HERE, after the survival inputs are lifted, because the wall policy is one
    // of its frozen authorities — a settlement that discovered its own footprints mid-loop would be
    // deciding what it is allowed to trust while it decided what to trust it for.
    //
    // The footprints above are lifted from the ORIGIN classification rather than the settled one:
    // erasure only ever REMOVES sites, so the origin-lifted set is a superset whose extra entries
    // belong to sites that gen no wall and are therefore never looked up.
    let dialect = dorc_oracle::build_dialect(&idx);
    let policy = match survival.as_ref() {
        Some(footprints) => dorc_plan::WallPolicy::RiskAccepted {
            footprints,
            resolutions: &resolutions,
            dialect: &dialect,
        },
        None => dorc_plan::WallPolicy::Honest,
    };
    // Book custody only: a dorc-lang interior is EXCLUDED, never deferred.
    let region_universe = dorc_core::region::RegionUniverse::of_book_custody_files(
        source_refs
            .iter()
            .enumerate()
            .filter(|(_, src)| !dorc_oracle::marker::has_marker(src))
            .map(|(index, _)| dorc_analysis::funcenv::source_file_of_index(index)),
    );
    // Every opener the census cannot see for itself, through one constructor that demands each.
    // Two are defensive emission's own signals below — the same question, one layer out.
    let string_execution = dorc_plan::region::StringExecutionSites::of_unit(&parsed.value);
    let definition_vectors = dorc_oracle::closure::definition_vectors(&source_refs);
    let regions = dorc_plan::region::census(
        &parsed.value,
        &cfg.value,
        &cfg.diags,
        dorc_plan::region::CensusOpeners::of(
            &region_universe,
            env.unresolvable_loads(),
            &definition_vectors,
            &string_execution,
        ),
        snapshot.book_file(),
    );
    let plan_inputs = dorc_plan::SettleInputs {
        src: &book_src,
        ast: &parsed.value,
        cfg: &cfg.value,
        vouches: &vouches,
        connected: &connected,
        policy,
        regions: &regions,
        // `309` §2 grade-stamping: this settlement is downstream of the intake, so every record it
        // writes is host-influenced. Carried by construction, not by each mint site remembering.
        world_account,
    };
    // The ledger holds CFG SITES (leaves and non-leaves alike) and grows by at least one per
    // non-quiescent round, so the bound is the node count plus the settling round. The leaf count
    // is NOT the bound — it misses every non-leaf invalidator, and it is one short besides.
    let fixpoint_cap =
        u32::try_from(dorc_analysis::solve::Graph::node_count(&cfg.value).saturating_add(1))
            .unwrap_or(u32::MAX)
            .max(1);
    let settled = settle_world(
        &frozen,
        &probe,
        results,
        &plan_inputs,
        fixpoint_cap,
        &mut interner,
        &mut arena,
        &mut trip,
    );
    debug_assert!(
        !settled.capped,
        "the settlement hit its site-count cap — no-execution stopped being monotone"
    );
    let round = settled.round;
    let classes = round.classes;
    let kills = round.kills;
    let invalidators = round.invalidators;
    let why_diags = round.why_diags;
    let classify_narrative = round.classify_narrative;
    let round_diags = round.diags;
    let (merge_narrative, collapsed_cells) = (settled.merge_narrative, settled.collapsed);
    let cascades = attribute_cascades(
        &cfg.value,
        &parsed.value,
        &book_src,
        &classes,
        &settled.ledger,
        &settled.validity,
        &settled.origin_validity,
    );
    report_at(sink, advisory, "classify", book_source, &round_diags);
    // `30K` §4.4: the effective-reach solve reports under its OWN pass name. It is a different
    // question from the origin reaching-defs one — only this answer can license an apply-time
    // elision — so a failure here must not read as the other's. The failing INDICES stay behind in
    // the in-memory outcome; the push surface carries the count (`rul-chain-is-pull-only`).
    if settled.effective_solve_failures > 0 {
        report_at(
            sink,
            advisory,
            "solve",
            None,
            &[Diag::new_spanless_site(DiagCode::SolverConsistencyFailure(
                dorc_aid::diag::SolverConsistencyFailure {
                    pass: dorc_aid::diag::SolvePass::EffectiveReach,
                    failing: settled.effective_solve_failures.to_string(),
                },
            ))],
        );
    }
    // The shared-cell collapse reaches a surface (`26G:fnd-shared-auto-cell-collides`): sites that
    // reported cleanly lose their licence because a SIBLING on the same cell disagreed or could not
    // answer, and until now the only trace was an unconsumed narrative. Spanless — the cell is a
    // cross-site coordinate, so blaming any one line's caret would misattribute a shared collapse.
    report_at(
        sink,
        advisory,
        "records",
        None,
        &collapsed_cells
            .iter()
            .map(|(fact, sites)| {
                Diag::new_spanless_site(DiagCode::SharedCellMeasurementsDisagree(
                    dorc_aid::diag::SharedCellMeasurementsDisagree {
                        cell: dorc_plan::fact_label(&interner, *fact),
                        sites: *sites,
                    },
                ))
            })
            .collect::<Vec<_>>(),
    );
    let mut spine = settled.spine;
    dorc_plan::attach_spine_probe_provenance(&mut spine, &parsed.value, &probe_origins, &mut arena);
    // DEFENSIVE emission (`28R:rul-defensive-mode-definition-vectors`): if the unit carries an
    // unresolved in-process definition vector, a BARE emitted name is no longer a proof that the
    // artifact's own body is what answers it, so every emitted name munges. Two halves, and the
    // asymmetry is the point — a lexical scan for the vectors that bind a name in THIS shell
    // (`eval` · `alias` · a computed command word), plus the environment's own unresolvable loads.
    // Never any-⊤: an unmodeled command is an external binary and cannot define a function here.
    spine.push_render_decision(dorc_core::spine::SpineRenderDecision::minted(
        None,
        None,
        dorc_core::spine::RenderDecision::DefensiveEmission {
            defensive: !dorc_oracle::closure::definition_vectors(&source_refs).is_empty()
                || !env.unresolvable_loads().is_empty(),
        },
        world_account,
    ));
    // `302:rul-certifier-trip-guard-only` — THE TERMINAL CLEANUP. Sited here, at the one moment
    // the whole Spine exists and before anything projects it, so the digest, the why report, the
    // summary and the artifact all describe the SAME decisions. Nothing between here and the
    // projection touches a disposition, so "immediately before plan-emission" and "immediately
    // after plan-construction" are the same seat, and only this one keeps every consumer honest.
    let (trip_diags, trip_narrative, spent) =
        demote_on_certifier_trip(&mut spine, trip, &definitions, world_account);
    report(sink, "solve", book_source, &trip_diags);
    // THE projection (`309` §0): every product below reads this derived `Plan`, never a second
    // assembly, and it exists at all only because the intake handed this run an authority and the
    // certifier latch was spent. It also DECIDES and records the render (`30E` §3).
    let plan = dorc_plan::project_plan(
        &mut spine,
        &book_src,
        &parsed.value,
        form_selection.emission(),
        &authority,
        &spent,
        world_account,
    );
    record_new_arm(
        &mut spine,
        &probe,
        &classes,
        &cfg.value,
        &invalidators,
        trip,
        load_decisions,
        &verdict_lane,
        &plan
            .steps()
            .iter()
            .map(|step| (step.ast, step.leaf))
            .collect::<Vec<_>>(),
        admitted_records.is_some(),
        world_account,
    );

    // q-2 (`dq-site-unresolvable`, the cli-edge readout): a `unresolvable-no-probe` comment lands
    // in the probe artifact, but nothing reached stderr (`219` q-1.f silent-3). Disclose each
    // probe-unresolvable site's source command as a Note — the apply runs it (`kFAIL-perform`).
    // ADVISORY (Note-severity): the off-ramp `apply` mode suppresses it; `plan`/round-trip show
    // it (the ui-3 cited-disclosure surface). The apply still RUNS the site either way, so no
    // correctness rides on this readout — it is purely the render surface (rec-1).
    report_at(
        sink,
        advisory,
        "probe",
        book_source, // the unresolvable-site notes span into the book (file:line:col frame)
        &unresolvable_diagnostics(&probe, &plan, &parsed.value, &book_src),
    );

    // upcoming-firstwall-hint (USER_STORY stage 3): the FIRST poison wall formed by an UNMODELED
    // command, plus the counterfactual count of downstream sites an oracle for it would un-wall.
    // Computed once (cheap, pure over the built plan) and consumed by BOTH the advisory `hint:` nag
    // (below) and the `dorc why` detail (`emit_why_report`). `None` ⇒ no unmodeled wall ⇒ no hint
    // (a modeled-but-diverged wall is an honest wall, never this hint's subject).
    let wall_steps = collect_wall_steps(
        &plan,
        &probe,
        &classes,
        &cfg.value,
        &kills,
        &parsed.value,
        &book_src,
    );
    let first_wall = first_wall_hint(&wall_steps);

    // stage-3 (the why-lens, `22D` §1): the FIRST receipt-READER made user-visible. For each
    // forced-run (never-elided) command whose ⊤ has a wired cause, surface — on the RENDER surface
    // (stderr), at the decision point — "why did this run?", cause-derived + remediation-classed.
    // rec-1 WELD: this is the plan-render surface ONLY; it is NEVER woven into the byte-floored
    // `.sh` artifact on stdout (the artifact stays receipt-free). The off-ramp `apply` mode
    // suppresses it (advisory); `plan` + round-trip emit it (ru-20 ui-3: "doubly-emit cited
    // sections + their warnings to the console").
    // The `plan`/round-trip render surface keeps its per-line `why:` disclosures (the attribution
    // lanes are load-bearing correctness disclosures gate-7 pins). `why` mode SKIPS them — its
    // stdout report (below) is the detail surface, so a stderr echo would just double it.
    // `27W` §3 C3 pairing: fold each ingested tier-3 report record (recognized class + site)
    // into its site's `VerdictDecline` via `with_authored_reason` (idempotent — tier-2 static
    // wins). Then union the collapse-narratives onto the why-lens seam (d4 renders; decision-inert).
    let paired_declines = pair_authored_reasons(decline_narrative, &results.reports);
    let collapse_narrative: Vec<CollapseNarrative> = classify_narrative
        .iter()
        .cloned()
        .chain(paired_declines)
        .chain(entry_narrative.iter().cloned())
        .chain(shadow_narrative.iter().cloned())
        .chain(consistency_narrative.iter().cloned())
        .chain(origin_consistency_narrative.iter().cloned())
        .chain(merge_narrative.iter().cloned())
        .chain(plan.survival_report.collapse_narrative().iter().cloned())
        .chain(trip_narrative)
        .chain(plan.render_refusal_narratives())
        .collect();
    if advisory && mode != Mode::Why {
        emit_why_lens(sink, &why_diags, &arena, &book_src, &collapse_narrative);
        // sigpipe-flap-class (`279f` §5): a probe record landing rc 141 (128+SIGPIPE) is the
        // NAMED early-exit-race nondeterminism class — a `pipefail`-off `A | grep -q` whose
        // consumer closed the pipe before an upstream stage finished writing. The landing is SAFE
        // (cant-tell ⇒ Unknown ⇒ run) and never flaps the verdict, so this is an advisory nudge,
        // not an error. (A `--exit-code`-like surface must source from divergence-of-world, never
        // this raw rc — see `dorc_plan::render::probe::record_scaffold`.)
        emit_sigpipe_race_notes(sink, results);
        emit_report_lane_notes(sink, results); // `27W` §2 tier-3 RUNTIME records; empty in-corpus
        // `27W` §3 tier-2 STATIC decline classes at plan time, with the emitting arm's file:line.
        emit_static_decline_notes(sink, &collapse_narrative, source_paths, source_srcs);
        // Stage 2 co-primary (rul24-divergence-is-the-game / TC-3): every SURVIVED elision names,
        // on this same why-lens lane, which running walls it crossed and whose footprint licensed
        // each crossing. This is the attribution tether under the sharpest claim in the design —
        // a wrong footprint silently under-executes someone else's line, so the render surface
        // must always say whose footprint you trusted. Empty when unflagged (no survivals).
        emit_survival_attribution(sink, &plan, &interner, source_paths, source_srcs);
        // 24G Part B: every converged elision a reaches() expansion DEMOTED names the reach-function
        // (the cross-author demote); empty when no reach expansion poisoned an elision.
        emit_reach_poisonings(sink, &plan, &interner);
        // Stage 3 (rul-guard-license / X-why): every GUARDED site names, on the same lane, the
        // mechanism + its converged-vouch license + the vouching oracle (a render-REFUSED guard
        // discloses the refusal instead). Empty when no site guards.
        emit_guard_attribution(sink, &plan, &interner, source_paths, source_srcs);
        // `27C` §4(a): every pure-predicate-CARRY elision names its cross-context attribution chain
        // on this same lane (the crossed substrate axes, each backing kind's owner `invariant:` line,
        // the read-set-closure proof). Empty when no site carried.
        emit_carry_attribution(sink, &plan, &carried_attribution);
        // upcoming-firstwall-hint (USER_STORY stage 3): the forward NAG — ONE aggregated line for
        // the FIRST unmodeled wall, naming the count an oracle for it would un-wall. `hint: ` prefix
        // (never `error[`), so the gate-3 stderr floor ignores it. rul24-warnings-tune-high: the
        // nag-loop drives the entire enhancement curve — this hint IS the product, not noise.
        if let Some(fw) = &first_wall {
            emit_stderr!(sink, "hint: {}", fw.body());
        }
        // ack-2 aggregate POINTER: the `plan` preview points the reader at the focused query
        // surface. (This pass keeps the per-line `why:` detail here too — gate-7 pins it; fully
        // moving the detail into `dorc why` is a sanctioned follow-on that churns the 13
        // expected-why needles + rewires gate-7, deferred to keep this pass green.)
        sink.emit(OutputEvent::plain_tagged(
            OutputChannel::Stderr,
            chrome_parts(&sink.render_ctx(), "cli-why-pointer-line", &[book_name]),
        ));
    }

    // gate-5 (cm-2 argv-echo differential): per-site resolved argv to stderr, behind the flag.
    // Independent of the advisory plane — it is a mechanized readout the harness consumes, not
    // human-facing disclosure, so it fires in any mode when asked (the round-trip is the only
    // caller in-corpus, but `plan --debug-argv` is a legitimate inspection).
    if options.debug_argv() {
        emit_debug_argv(sink, &plan, &cfg.value, &value, &interner);
    }

    // arch-1 d-6: the leaf-exact render refuses to elide a leaf whose span can't be safely
    // edited (a heredoc-bearing command — its span covers `<<EOF`, not the body), running it
    // verbatim instead (kFAIL-perform). Surface WHY on stderr (else a converged mutator
    // silently running is invisible); the gate-3 floor requires the case to declare it. These
    // are ERROR-severity, so they cross the floor in EVERY mode (incl. `apply`): the off-ramp
    // must never silently ship an artifact whose render had to refuse a licensed elision.
    let refusals = plan.render_refusal_diagnostics(&parsed.value, &interner);
    report(sink, "render", book_source, &refusals);

    // The generated plan's own imports, on the PLAN surface (`two-surfaces`). Advisory-routed: a
    // Note about the emission this run chose, not a give-up an `apply` must be told about.
    report_at(
        sink,
        advisory,
        "emission",
        book_source,
        &plan.import_diagnostics(&parsed.value),
    );

    // `300:lane-sparing-rederivation`: a survival the wall walk minted that the independent
    // reference model would not confirm. Empty in a healthy engine; non-empty means OUR two
    // implementations of one algebra disagreed, and the site took the guard/run floor. Reported,
    // never folded into `identity_diags`: the demotion is already a plan-shape change the decision
    // digest covers, so adding the diagnostic would double-count one event.
    report(
        sink,
        "rederive",
        book_source,
        &plan.rederivation_diagnostics(&parsed.value),
    );

    // The trip banner joins the IDENTITY plane (`302:rul-certifier-trip-guard-only` — the boolean
    // is a spine row). It needs no new durable field: `canon_diag` keys an Error by slug, span and
    // severity, so a tripped run's digest differs from the same book's clean one even when the
    // cleanup had nothing to evict and the dispositions are byte-identical.
    let identity_diags: Vec<Diag> = round_diags
        .iter()
        .cloned()
        .chain(refusals.iter().cloned())
        .chain(trip_diags)
        .collect();
    let decision_digest = dorc_plan::erasability::decision_digest(
        &plan,
        &probe,
        &book_src,
        &parsed.value,
        &interner,
        &identity_diags,
    );

    // ack-2 `dorc why`: NOT an artifact-producing invocation. Emit the source-line-keyed report to
    // STDOUT (its own non-analysis output) and return — no artifact, no plan-summary, no digest.
    // It runs the full pipeline above so it reports on the CURRENT run's real dispositions.
    if mode == Mode::Why {
        // `--last` belt-and-suspenders: a diverged decision digest (same inputs) ⇒ refuse, not narrate.
        if let Some(r) = replay
            && decision_digest != r.decision_digest
        {
            report_at(
                sink,
                advisory,
                "whylog",
                None,
                &[Diag::new_spanless_site(DiagCode::WhylogBookDesync(
                    dorc_aid::diag::WhylogBookDesync {
                        which: "decision-digest".to_owned(),
                    },
                ))],
            );
            return Ok(book_outcome);
        }
        let receipt = Receipt {
            at: replay.map_or_else(|| edges.clock().now(), |r| r.started_at),
            replayed: replay.is_some(),
            host: framing.host().to_owned(),
            book: book_name.to_owned(),
            book_digest: book_digest(&book_src),
            at_head: edges.source_match(book_name),
            oracles: oracle_paths.to_vec(),
            risk_profile: options.risk_faultless_skips().then_some(CONSENT_FLAG),
            tally: PlanTally::Derived(plan.disposition_counts()),
            deepest_tier: options.all(),
            // Only a replay can disagree, and it declares its stream rather than being assumed.
            narratable: replay
                .is_none_or(|r| r.record_stream_version == dorc_aid::narrative::PLANE_VERSION),
        };
        let parts = why_report_parts(
            &sink.render_ctx(),
            &WhyReport {
                address: options.why_address(),
                plan: &plan,
                probe: &probe,
                first_wall: first_wall.as_ref(),
                wall_steps: &wall_steps,
                why_diags: &why_diags,
                refusals: &refusals,
                arena: &arena,
                ast: &parsed.value,
                book_src: &book_src,
                filename: book_name,
                interner: &interner,
                source_paths,
                source_srcs,
                narrative: &collapse_narrative,
                cascades: &cascades,
                receipt: &receipt,
            },
        );
        sink.emit(OutputEvent::plain_tagged(OutputChannel::Stdout, parts));
        sink.flush(OutputChannel::Stdout);
        *world_out = Some(WhyWorld {
            snapshot: snapshot.clone(),
            interner,
            arena,
            ast: parsed.value,
            spine,
            plan,
            probe,
            narrative: collapse_narrative,
            why_diags,
            refusals,
            wall_steps,
            first_wall,
            cascades,
        });
        return Ok(book_outcome);
    }

    // ONE structure: the stream and the published tree both READ it, and there is deliberately no
    // second assembly to fall back to. rec-1 / ru-12 BYTE FLOOR holds inside it — `plan` and
    // `apply` emit byte-identical receipt-free bytes, and so does the round-trip's second block.
    let artifact =
        form_selection.with_plan(plan.render_apply(&book_src, &parsed.value), plan.account());
    generated.push(GeneratedOutput::Artifact(artifact.clone()));
    sink.emit(OutputEvent::plain_text(
        OutputChannel::Stdout,
        artifact.primary().bytes.clone(),
    ));

    // `30Qe:fruit-emit-hygiene-paste-rules` (`KNOBS:kBOOT`) — the paste/splice-floor damage watch:
    // scan the FINALIZED artifact bytes (post `with_plan`, the exact bytes stdout/a published tree
    // ship) for a physical line that a live human-mediated paste could corrupt or truncate.
    // Detection only (`two-surfaces`): a hazard is a diagnostic, never a rewrite of authored bytes.
    report_at(
        sink,
        advisory,
        "emission",
        None,
        &emitted_line_unsafe_for_paste_diagnostics(&artifact.primary().bytes),
    );

    // On the PLAN surface, never woven into the artifact bytes (`two-surfaces`).
    if let Some(fallback) = artifact.fallback() {
        report_at(
            sink,
            advisory,
            "emission",
            None,
            &[Diag::new_spanless_site(DiagCode::ArtifactFormFallback(
                dorc_aid::diag::ArtifactFormFallback {
                    form: artifact.form().name(),
                    cause: fallback.cause(),
                    loads: fallback.loads(),
                },
            ))],
        );
    }
    if options.artifact.destination.is_directory()
        && let Err(reason) = edges.publish_artifact(&artifact)
    {
        report(
            sink,
            "emission",
            None,
            &[Diag::new_spanless_site(DiagCode::ArtifactPublishRefused(
                dorc_aid::diag::ArtifactPublishRefused { reason },
            ))],
        );
        return Ok(RunOutcome::ArtifactUnservable);
    }

    // plans/240 Stage-1 yardstick: the plan-summary on stderr, alongside the digest below.
    emit_plan_summary(sink, &plan);

    sink.emit(OutputEvent::plain_tagged(
        OutputChannel::Stderr,
        chrome_parts(
            &sink.render_ctx(),
            "cli-decision-digest-line",
            &[&decision_digest],
        ),
    ));

    // Default-on: the receipt nobody asked for is the only kind that exists on the bad morning.
    if options.durable == DurableOutput::Enabled
        && whylog_eligible
        && let Some(records) = admitted_records
    {
        let started_at = edges.clock().now();
        record_durable_arm(
            edges,
            &mut spine,
            &framing,
            snapshot,
            &decision_digest,
            started_at,
            results,
            records,
            world_account,
        );
        // The durable is a PROJECTION of what the run decided (`309` §0), so what reaches disk is
        // decided at one seat, per species, and what it drops is countable there too.
        if let Some(projection) = dorc_plan::whylog::DurableProjection::project(&spine) {
            write_whylog(edges, sink, generated, &projection);
        }
    }
    *world_out = Some(WhyWorld {
        snapshot: snapshot.clone(),
        interner,
        arena,
        ast: parsed.value,
        spine,
        plan,
        probe,
        narrative: collapse_narrative,
        why_diags,
        refusals,
        wall_steps,
        first_wall,
        cascades,
    });
    Ok(book_outcome)
}

fn write_whylog(
    edges: &mut dyn EngineEdges,
    sink: &mut dyn OutputSink,
    generated: &mut Vec<GeneratedOutput>,
    projection: &dorc_plan::whylog::DurableProjection<'_>,
) {
    let write = dorc_plan::whylog::WhylogV2Write::of_projection(projection);
    let bytes = match dorc_plan::whylog::try_serialize_v2(
        &write,
        dorc_plan::whylog::WhylogLimits::spike_default(),
    ) {
        Ok(bytes) => bytes,
        Err(refusal) => {
            report_whylog_unwritten(
                sink,
                edges.durable_label(),
                serialize_refusal_reason(refusal),
            );
            return;
        }
    };
    generated.push(GeneratedOutput::Whylog(bytes.clone()));
    if let Err(reason) = edges.publish_whylog(&bytes) {
        report_whylog_unwritten(sink, edges.durable_label(), &reason);
    }
}

fn report_whylog_unwritten(sink: &mut dyn OutputSink, destination: &str, reason: &str) {
    report(
        sink,
        "whylog",
        None,
        &[Diag::new_spanless_site(DiagCode::WhylogUnwritten(
            dorc_aid::diag::WhylogUnwritten {
                dir: destination.to_owned(),
                reason: reason.to_owned(),
            },
        ))],
    );
}

const fn serialize_refusal_reason(refusal: dorc_plan::whylog::WhylogWriteRefusal) -> &'static str {
    use dorc_plan::whylog::WhylogWriteRefusal as R;
    match refusal {
        R::Limit => "limit",
        R::Grammar => "grammar",
        R::Numeric => "numeric",
        R::Digest => "digest",
        R::ArithmeticOverflow => "overflow",
    }
}

/// Decide this run's emission form from authored inputs alone (`30I` §7.1).
///
/// Answered for every mode that reaches it, including the ones that return before an artifact is
/// printed: the selector is pure and cheap, and computing it unconditionally is what lets the print
/// seat take its bytes from the artifact SET with no second assembly to fall back to
/// (`30I:step-7-reify-plan-artifact-forms`: one final structure, not two).
///
/// The STREAM POSTURE derives from the injected stdout fact
/// (`30Ng:rul-piped-stdout-carries-a-full-plan`, human-typed): a stdout nobody is watching is one
/// the user is KEEPING — piping it to a pager, an editor, a file they will read and then hand to
/// `apply` — so it carries the artifact, complete, and naming a directory beside it claims that same
/// artifact twice.
///
/// # Errors
/// Refuses when two things claim this run's artifact, when the invocation NAMED a form this book
/// cannot be given, or when a kept stream can carry no complete plan.
fn select_artifact_form(
    options: &EngineOptions,
    snapshot: &StaticLoadSnapshot,
    cfg: &dorc_analysis::cfg::Cfg,
    book: &dorc_syntax::Ast,
    book_src: &str,
    env: &dorc_analysis::funcenv::FuncEnv,
) -> Result<crate::artifact::Selection, crate::artifact::FormRefusal> {
    use crate::artifact::{FormRequest, artifact_stream, book_loads, select};

    let projection = crate::bundle::project(snapshot, env.loads())
        .map(crate::bundle::BundleProjectionOutput::into_projection)
        .unwrap_or_default();
    let loads = book_loads(cfg, book, book_src, &projection, env);
    let posture = artifact_stream(
        options.stdout(),
        options.artifact_destination().is_directory(),
    )?;
    let request = options
        .form()
        .map_or(FormRequest::Auto, FormRequest::Explicit);
    select(snapshot, &projection, &loads, request, posture)
}

/// Mint the definition-plane decisions, ABOVE THE INTAKE (`tc-load-decisions-read-authored`,
/// human lean applied).
///
/// The carve is structural rather than asserted. `30I:rul-load-decisions-are-authored-before-contact`
/// says these wear authored-before-contact, and every input here earns it: the contested families
/// come from the function environment's own answer, which admits a word only when it is a literal
/// graded `ValueGrade::ProgramText` (`funcenv-reads-source-literal-plane-only`), and the
/// unresolvable loads are CFG positions. But the RECORDS used to be built inside
/// [`record_new_arm`], on a path the intake had already reached — so the authored answer was a
/// label the seat asserted rather than a fact about where it stood.
///
/// Called before the intake edge, this cannot read influenced material even by accident: there is
/// nothing influenced in scope yet. Transcribing the finished records onto the Spine later is
/// transcription, not a mint, so it joins nothing (`309:rul-spine-preserves-never-stamps`).
fn mint_load_decisions(
    cfg: &dorc_analysis::cfg::Cfg,
    contested: &dorc_core::ContestedFamilies,
    env: &dorc_analysis::funcenv::FuncEnv,
) -> Vec<dorc_core::spine::SpineLoadDecision> {
    use dorc_core::spine::{SpineLoadDecision, WithheldCause};

    let authored = dorc_core::influence::InfluenceAccount::authored_before_contact();
    contested
        .families()
        .map(|name| {
            SpineLoadDecision::minted(
                name.to_owned(),
                None,
                Some(WithheldCause::Contested),
                authored,
            )
        })
        .chain(env.unresolvable_loads().iter().map(|node| {
            SpineLoadDecision::minted(
                format!("load@{}", cfg.node(*node).ast.0),
                None,
                Some(WithheldCause::Unprovable),
                authored,
            )
        }))
        .collect()
}

/// Write the run's `new`-arm records onto the Spine (`30E` §2's transitory species).
///
/// These are non-durable in production but not RULED non-durable — the census's legal resting state
/// for in-flight work — so nothing here reaches a projection. What they buy is that the arm is real:
/// a species classified `New` and populated by nobody is a claim to track something that nothing can
/// check, and the debug dump is what checks it.
///
/// NOT YET MINTED, with their seats named rather than left to be discovered: `SpineVouch` (the
/// `Vouches` map exposes no iteration yet), `SpineObservation` (the `by_fact` merge, which the fold
/// consumes by closure rather than by collection), `SpineValidityRound` (the fixpoint's rounds
/// are deliberately never-survives — `the-fixpoint-owns-the-rounds-and-builds-nothing-else` — so
/// recording them means deciding what a round may leave behind, which is its own question), and
/// `SpineOutcome` (the exit-code seat runs past every projection and holds no Spine there; found by
/// the `30Nd` meaning-audit, where `30F` §4.5 had disclosed only the first three).
#[expect(
    clippy::too_many_arguments,
    reason = "one recording pass over independent analysis products; a params struct would be this signature re-spelled"
)]
fn record_new_arm(
    spine: &mut dorc_plan::Spine,
    probe: &dorc_plan::ProbePlan,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    cfg: &dorc_analysis::cfg::Cfg,
    invalidators: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    trip: dorc_analysis::certify::CertifierTrip,
    load_decisions: Vec<dorc_core::spine::SpineLoadDecision>,
    verdict_lane: &BTreeMap<dorc_analysis::cfg::CfgNodeId, dorc_analysis::effect::Measurement>,
    spine_leaves: &[(dorc_core::AstId, dorc_core::LeafId)],
    admitted: bool,
    world_account: dorc_core::influence::InfluenceAccount,
) {
    use dorc_core::spine::{
        AdmissionOutcome, ShipLane, SpineAdmission, SpineProbeShip, SpineSiteClassification,
        SpineSolveCertification,
    };

    spine.set_admission(SpineAdmission::minted(
        // The refusal arm returned long before here, so the only two states this seat can be in are
        // the two that carry authority (`Authorised`).
        if admitted {
            AdmissionOutcome::Admitted
        } else {
            AdmissionOutcome::NoObservation
        },
        None,
        world_account,
    ));
    // ONE whole-window row, not four per-pass ones: the trip latch is monotone across every pass
    // (`certifier-trip-is-a-monotone-latch`), so what this run can honestly state is the window's
    // verdict. Per-pass consistency lives in the in-memory `SolveConsistency` and is a later record.
    spine.push_certification(SpineSolveCertification::minted(
        "whole-window",
        !trip.tripped(),
        trip.tripped(),
        world_account,
    ));
    for record in load_decisions {
        spine.push_load_decision(record);
    }
    for check in &probe.checks {
        spine.set_ship(SpineProbeShip::minted(
            dorc_core::SiteId {
                leaf: check.site,
                member: check.member,
            },
            if check.verdict {
                ShipLane::Verdict
            } else {
                ShipLane::Predict
            },
            check.defining_span.map(|(_, file)| file),
            world_account,
        ));
    }
    for leaf in &probe.unresolvable {
        spine.set_ship(SpineProbeShip::minted(
            dorc_core::SiteId::leaf(*leaf),
            ShipLane::Unresolvable,
            None,
            world_account,
        ));
    }
    // A classification is CFG-node-keyed while the decision plane is SITE-keyed, and the two spaces
    // are unrelated integers — reading one as the other would key a record to somebody else's site,
    // which is precisely what `inv-site-keyed-results` exists to forbid. The plan already carries the
    // one true mapping (leaf ids are assigned by span in `build_plan_walled`), so the bridge is its
    // `ast → leaf` back-map; a node with no leaf has no site to be keyed by and is skipped.
    let leaf_of: BTreeMap<dorc_core::AstId, dorc_core::LeafId> =
        spine_leaves.iter().copied().collect();
    for (node, class) in classes {
        let Some(leaf) = leaf_of.get(&cfg.node(*node).ast).copied() else {
            continue;
        };
        spine.set_classification(SpineSiteClassification::minted(
            dorc_core::SiteId::leaf(leaf),
            class_label(class),
            verdict_lane.contains_key(node),
            // The REAL invalidator set (`classify-answers-with-its-invalidators`): `kills` alone
            // read false for every ordinary establish and every opaque leaf.
            invalidators.contains(node),
            dorc_core::spine::OperandAccount::capped(class_cells(class)),
            world_account,
        ));
    }
}

/// The `SkipClass` discriminant's greppable label — referent-agnostic, never branched on.
const fn class_label(class: &dorc_analysis::effect::SkipClass) -> &'static str {
    use dorc_analysis::effect::SkipClass;
    match class {
        SkipClass::MustRun => "MustRun",
        SkipClass::EstablishProbeAmbient(_) => "EstablishProbeAmbient",
        SkipClass::EstablishProbeWritten(_) => "EstablishProbeWritten",
        SkipClass::QueryResolvable { .. } => "QueryResolvable",
        SkipClass::EstablishMembers { .. } => "EstablishMembers",
        SkipClass::InlineCall { .. } => "InlineCall",
    }
}

/// Every cell one classification keys on, in member order (`aggregate-mints-carry-the-same-demand`).
///
/// RECURSIVE over aggregates, and that is the point: an `InlineCall`'s members are themselves
/// classifications, so a member that resolves a QUERY cell keys the call exactly as an establish
/// member does. The first cut of this matched only the two establish arms and dropped the rest —
/// which made the account narrower than the decision it claims to describe, the same species of
/// falsehood as the empty list it replaced (`30Mc` F3, completed).
fn class_cells(class: &dorc_analysis::effect::SkipClass) -> Vec<dorc_core::FactKey> {
    use dorc_analysis::effect::SkipClass;
    match class {
        SkipClass::MustRun => Vec::new(),
        SkipClass::EstablishProbeAmbient(fact)
        | SkipClass::EstablishProbeWritten(fact)
        | SkipClass::QueryResolvable { fact, .. } => vec![*fact],
        SkipClass::EstablishMembers { members, .. } => members.clone(),
        SkipClass::InlineCall { sites } => sites
            .iter()
            .flat_map(|site| class_cells(&site.class))
            .collect(),
    }
}

/// Write the run's durable-arm records onto the Spine (`30E` §2's four species).
///
/// The durable itself is projected from these through `plan::whylog`'s per-species Views; nothing
/// here decides what reaches disk. That separation is the point: the driver states what the run WAS,
/// and one seat decides what a durable KEEPS of it.
#[expect(
    clippy::too_many_arguments,
    reason = "one recording pass over independent durable facts"
)]
fn record_durable_arm(
    edges: &mut dyn EngineEdges,
    spine: &mut dorc_plan::Spine,
    framing: &dorc_plan::records::Framing,
    snapshot: &StaticLoadSnapshot,
    decision_digest: &str,
    started_at: Option<dorc_core::RunInstant>,
    results: &SiteResults,
    records: dorc_plan::records::AdmittedUnscopedHostRecords,
    world_account: dorc_core::influence::InfluenceAccount,
) {
    spine.set_invocation(edges.invocation_record(InvocationRecordRequest {
        framing,
        snapshot,
        started_at,
        account: world_account,
    }));
    spine.set_digest(dorc_core::spine::SpineDigest::minted(
        decision_digest.to_owned(),
        world_account,
    ));
    spine.set_record_stream(dorc_core::spine::SpineRecordStream::minted(
        records,
        results
            .records
            .values()
            .filter_map(|record| Some((record.stamp.ordinal, record.stamp.received_at?)))
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect(),
        world_account,
    ));
}

/// The two notices the full-positional regime owes a book author (`28K` §2
/// `rul-visibility-is-full-positional`), both aid-plane and neither changing any license.
///
/// **The move-it-up hint** — a book defines a COMMAND role below sites its family could otherwise
/// have answered. The design's named, accepted consequence is that such a definition licenses
/// nothing above itself; the recovery is one line of cut-and-paste, and the engine is the only
/// party positioned to notice. Fired only where NOTHING answers the site: if some other unit's
/// definition is live there the family is contested (or genuinely served) and this would be noise.
///
/// **The in-book vocabulary refusal** (`28M:obl-in-book-vocabulary-role-notice`) — kind-owner
/// members load from the ambient prefix only, so an in-book one never takes effect. Refused WITH a
/// notice rather than silently, since silence here reads as "my resolver is broken".
fn positional_loading_notices(
    book: &dorc_syntax::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    interner: &Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Vec<Diag> {
    use dorc_analysis::cfg::CfgNodeKind;
    use dorc_analysis::value::ValueOf;
    use dorc_syntax::ast::NodeKind;

    let mut diags = Vec::new();
    for (_, node) in book.iter() {
        let NodeKind::FuncDef {
            name, name_span, ..
        } = &node.kind
        else {
            continue;
        };
        let Some((family, role)) = dorc_oracle::reserved::role_family(name) else {
            continue;
        };
        if dorc_oracle::reserved::is_vocabulary_role(role) {
            diags.push(Diag::new(
                DiagCode::InBookVocabularyRole(dorc_aid::diag::InBookVocabularyRole {
                    name: name.clone(),
                    role: role.to_owned(),
                }),
                *name_span,
            ));
            continue;
        }
        let unanswered = cfg
            .iter()
            .filter(|(id, n)| n.kind == CfgNodeKind::Command && !cfg.is_expansion_internal(*id))
            .filter(|(id, _)| live.source_before(*id, name).is_none())
            .filter(|(id, _)| match value.argv_values(*id).first() {
                Some(ValueOf::Literal(word)) => {
                    dorc_oracle::to_funcname_segment(interner.resolve(*word)) == family
                }
                _ => false,
            })
            .count();
        if unanswered > 0 {
            diags.push(Diag::new(
                DiagCode::RoleDefinedBelowItsSites(dorc_aid::diag::RoleDefinedBelowItsSites {
                    name: name.clone(),
                    sites: unanswered,
                }),
                *name_span,
            ));
        }
    }
    diags
}

/// The LOAD-HEAD diagnostics (`30P:the-load-principles`), each spanned at its own `.` line.
///
/// The environment records every population as data and mints nothing of its own (the
/// `funcenv::unresolvable_loads` precedent); this driver is where they become diagnostics. Two are
/// lints that change no verdict — an OFF-RAMP hazard on a load that resolves fine here, and why a
/// load the controller could not follow was unfollowable. The third is the computed `.`, whose
/// whole-run outcome is decided beside `wrapper_incoherent` at the caller.
fn load_head_notices(
    book: &dorc_syntax::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    env: &dorc_analysis::funcenv::FuncEnv,
    src: &str,
) -> Vec<Diag> {
    use dorc_analysis::funcenv::HavocCause;

    let at = |node: &dorc_analysis::cfg::CfgNodeId| book.node(cfg.node(*node).ast).span;
    let dies = env.dies_slashless().iter().map(|node| {
        Diag::new(
            DiagCode::ScriptRelativeLoadDiesSlashless(
                dorc_aid::diag::ScriptRelativeLoadDiesSlashless,
            ),
            at(node),
        )
    });
    let searches = env.searches_path().iter().map(|node| {
        Diag::new(
            DiagCode::SlashlessSourceSearchesPath(dorc_aid::diag::SlashlessSourceSearchesPath),
            at(node),
        )
    });
    let computed = env
        .havoc_causes()
        .iter()
        .filter(|(_, cause)| matches!(cause, HavocCause::ComputedSubstitution))
        .map(|(node, _)| {
            Diag::new(
                DiagCode::ComputedSourceOperand(dorc_aid::diag::ComputedSourceOperand),
                at(node),
            )
        });
    // The one notice whose SUBJECT is a line other than the one it is spanned at: the reader can
    // see the `.` that lost its carriage and cannot see what moved the ground under it.
    let withheld = env
        .havoc_causes()
        .iter()
        .filter_map(|(node, cause)| match cause {
            HavocCause::CwdUnknown { clobbered_at } => Some((node, clobbered_at)),
            _ => None,
        })
        .map(|(node, clobbered_at)| {
            Diag::new(
                DiagCode::LoadCarriageWithheldUnderUnknownCwd(
                    dorc_aid::diag::LoadCarriageWithheldUnderUnknownCwd {
                        line: line_of(book, cfg, *clobbered_at, src),
                    },
                ),
                at(node),
            )
        });
    dies.chain(searches)
        .chain(computed)
        .chain(withheld)
        .collect()
}

/// The 1-based line a CFG node's own bytes start on.
///
/// Counted from the book source rather than carried, because a span is all the analysis holds and
/// the reader wants a line. Zero-length or out-of-range spans answer line 1, which is the same
/// conservative floor the locator takes.
fn line_of(
    book: &dorc_syntax::Ast,
    cfg: &dorc_analysis::cfg::Cfg,
    node: dorc_analysis::cfg::CfgNodeId,
    src: &str,
) -> usize {
    let span = book.node(cfg.node(node).ast).span;
    src.get(..span.lo.0 as usize)
        .map_or(1, |before| before.matches('\n').count().saturating_add(1))
}

/// The decision-inert narrative each proven shadow mints (`collapse-mints-narrative`). Tier
/// `Derived`: this is the engine's own reading of the environment, not anybody's claim.
/// The consistency-failure account for the two PRE-NETWORK kernel seats (`302` §4).
///
/// `analysis::value` and `analysis::funcenv` record the failure as DATA and mint nothing of their
/// own (the `funcenv::unresolvable_loads` precedent); this driver is where it becomes a diagnostic
/// and a narrative. Both seats run before the probe is compiled and before any host byte is read,
/// so the posture is tier-2 fail-fast: loud, on human timescales, with the honest floor still
/// producing a valid plan.
///
/// Scalars only cross (`operands-are-pure-and-capped`): the failing check INDICES and the counts.
/// The lattice values that failed stay in the in-memory `SolveConsistency`.
fn solve_consistency_reports(
    value: &dorc_analysis::value::ValueFlow,
    env: &dorc_analysis::funcenv::FuncEnv,
) -> (Vec<Diag>, Vec<CollapseNarrative>) {
    use dorc_aid::diag::{SolvePass, SolverConsistencyFailure};
    use dorc_analysis::certify::SolveConsistency;
    use dorc_analysis::funcenv::EnvFloor;

    let mut diags = Vec::new();
    let mut narratives = Vec::new();
    if let SolveConsistency::Inconsistent(report) = value.consistency() {
        diags.push(Diag::new_spanless_site(DiagCode::SolverConsistencyFailure(
            SolverConsistencyFailure {
                pass: SolvePass::ValueFlow,
                failing: report.total().to_string(),
            },
        )));
        narratives.push(consistency_narrative(SolvePass::ValueFlow, report));
    }
    // Only a failure of the ENVIRONMENT's own solve is reported here. A `ValuePlaneUntrusted`
    // floor is the CASCADE of the value failure above, and reporting it too would present one
    // defect as two (`271:rul-sin-ordering`: only root-cause is reported).
    if let Some(EnvFloor::SolverInconsistent(consistency)) = env.floor()
        && let SolveConsistency::Inconsistent(report) = consistency.as_ref()
    {
        diags.push(Diag::new_spanless_site(DiagCode::SolverConsistencyFailure(
            SolverConsistencyFailure {
                pass: SolvePass::FunctionEnvironment,
                failing: report.total().to_string(),
            },
        )));
        narratives.push(consistency_narrative(
            SolvePass::FunctionEnvironment,
            report,
        ));
    }
    (diags, narratives)
}

/// The consistency-failure diagnostics out of a classify round, split from the rest so the
/// PRE-NETWORK round can report them at a pre-network seat (R3).
///
/// Matched on the TYPED payload rather than on rendered words, per `prose-pins-live-where-the-
/// prose-does`: the wording is unwelded and a text match here would silently stop selecting the
/// moment anyone edits the register.
fn consistency_diags_of(diags: &[Diag]) -> Vec<Diag> {
    diags
        .iter()
        .filter(|diag| matches!(diag.code, DiagCode::SolverConsistencyFailure(_)))
        .cloned()
        .collect()
}

/// The consistency-failure narratives out of a classify round — the origin round's share, which
/// otherwise never reached the confluence at all (its whole narrative slice is dropped in favour
/// of the fixpoint round's).
///
/// The fixpoint rounds keep reporting their OWN failures at the batched surface: a later round is
/// a different solve, so its failure is a second event rather than an echo of this one.
fn consistency_narratives_of(narratives: &[CollapseNarrative]) -> Vec<CollapseNarrative> {
    narratives
        .iter()
        .filter(|narrative| {
            matches!(
                narrative.kind(),
                CollapseKind::SolverConsistencyFailure { .. }
            )
        })
        .cloned()
        .collect()
}

/// The scalar narrative for one failing solve — the cli's copy of the `analysis::effect` mint,
/// over its own lattice type.
fn consistency_narrative<L>(
    pass: dorc_aid::diag::SolvePass,
    report: &dorc_analysis::certify::FailedChecks<L>,
) -> CollapseNarrative {
    use dorc_aid::narrative::{FailedCheck, Operands};

    let mut checks: Vec<FailedCheck> = Vec::new();
    for &node in report.failing().boundary() {
        checks.push(FailedCheck::Boundary {
            node: u32::try_from(node).unwrap_or(u32::MAX),
        });
    }
    for &(from, to) in report.failing().edges() {
        checks.push(FailedCheck::Edge {
            from: u32::try_from(from).unwrap_or(u32::MAX),
            to: u32::try_from(to).unwrap_or(u32::MAX),
        });
    }
    let advisory = report.advisory();
    CollapseNarrative::new(
        SpeechAct::Derived,
        CollapseKind::SolverConsistencyFailure {
            pass,
            operands: Operands::capped(checks),
            shown: u32::try_from(report.shown()).unwrap_or(u32::MAX),
            total: u32::try_from(report.total()).unwrap_or(u32::MAX),
            solves: 1,
            advisory: dorc_aid::narrative::SolverRounds {
                converged: advisory.converged,
                rounds: u32::try_from(advisory.rounds).unwrap_or(u32::MAX),
            },
        },
    )
}

fn shadow_narratives(
    shadows: &[dorc_analysis::funcenv::Contest],
    definitions: &dorc_analysis::funcenv::DefinitionTable,
) -> Vec<CollapseNarrative> {
    use dorc_aid::narrative::{DefinitionSite, MintSpan};
    shadows
        .iter()
        .filter_map(|contest| {
            let prior = definitions.get(contest.prior)?;
            let shadowing = definitions.get(contest.shadowing)?;
            let site = |d: &dorc_analysis::funcenv::Definition| DefinitionSite {
                file: d.file,
                name: MintSpan(d.name_span),
            };
            Some(CollapseNarrative::new(
                SpeechAct::Derived,
                CollapseKind::RoleFamilyShadowed {
                    prior: site(prior),
                    shadowing: site(shadowing),
                },
            ))
        })
        .collect()
}

/// The shadow refusal's diagnostics, grouped by the file the SHADOWING definition lives in — the
/// one file whose frame can carry the caret. The overridden definition rides the payload as
/// `path:line` text, since one `report_at` threads one source (`AID-NEEDS:law-lineno-identity`).
fn shadow_diagnostics(
    shadows: &[dorc_analysis::funcenv::Contest],
    definitions: &dorc_analysis::funcenv::DefinitionTable,
    source_paths: &[String],
    source_srcs: &[&str],
) -> Vec<(usize, Vec<Diag>)> {
    let mut by_file: BTreeMap<usize, Vec<Diag>> = BTreeMap::new();
    for contest in shadows {
        let (Some(prior), Some(shadowing)) = (
            definitions.get(contest.prior),
            definitions.get(contest.shadowing),
        ) else {
            continue;
        };
        let Some((family, _)) = dorc_oracle::reserved::role_family(&contest.name) else {
            continue;
        };
        let prior_file = prior.file.0 as usize;
        let shadowing_file = shadowing.file.0 as usize;
        let (Some(prior_path), Some(prior_src), true) = (
            source_paths.get(prior_file),
            source_srcs.get(prior_file),
            shadowing_file < source_paths.len(),
        ) else {
            continue;
        };
        let (line, _) = dorc_aid::diag::line_col(prior_src, prior.name_span.lo.0 as usize);
        by_file.entry(shadowing_file).or_default().push(Diag::new(
            DiagCode::RoleFamilyContested(dorc_aid::diag::RoleFamilyContested {
                family: family.to_owned(),
                name: contest.name.clone(),
                prior: format!("{prior_path}:{line}"),
            }),
            shadowing.name_span,
        ));
    }
    by_file.into_iter().collect()
}

/// The closure refusal's diagnostics, grouped by the file the LATER declaration lives in — the
/// sibling of [`shadow_diagnostics`] one namespace down (`28K` §4; `28M` §8's diamond rider). The
/// earlier declaration rides the payload as `path:line`, since one `report_at` threads one source.
///
/// Reported at the LOAD edge, not per pinned definition: the collision rebinds the name for every
/// caller the moment both sources load, so it is one claim about the loaded set with one
/// remediation, and a per-definition report would point N-1 authors at somebody else's file
/// (`271:rul-sin-ordering`).
fn helper_conflict_diagnostics(
    helpers: &dorc_oracle::closure::HelperIndex,
    source_paths: &[String],
    source_srcs: &[&str],
) -> Vec<(usize, Vec<Diag>)> {
    let mut by_file: BTreeMap<usize, Vec<Diag>> = BTreeMap::new();
    for conflict in helpers.conflicts() {
        let (Some(&(prior_file, prior_span)), Some(&(later_file, later_span))) =
            (conflict.sites.first(), conflict.sites.get(1))
        else {
            continue;
        };
        let (Some(prior_path), Some(prior_src), true) = (
            source_paths.get(prior_file),
            source_srcs.get(prior_file),
            later_file < source_paths.len(),
        ) else {
            continue;
        };
        let (line, _) = dorc_aid::diag::line_col(prior_src, prior_span.lo.0 as usize);
        by_file.entry(later_file).or_default().push(Diag::new(
            DiagCode::HelperDeclarationContested(dorc_aid::diag::HelperDeclarationContested {
                name: conflict.name.clone(),
                prior: format!("{prior_path}:{line}"),
            }),
            later_span,
        ));
    }
    by_file.into_iter().collect()
}

/// Resolve a connected pipe STAGE's stripped `<provider>__predict` body PLUS its STDOUT coverage
/// (`271:rul-only-oracle-bytes-ship` rider 1 — the composed-probe repair). Mirrors
/// [`ship_predict_body`]'s check-resolution, then asks
/// [`predict_stage_stdout`](dorc_oracle::predict::predict_stage_stdout) whether the arm this argv
/// selects produces REAL (delegation-produced) stdout bytes — the coverage a downstream byte-consumer
/// requires. `None` ⇒ no check resolves ⇒ the stage is un-shippable ⇒ the compound refuses (⇒ runs).
#[expect(
    clippy::too_many_arguments,
    reason = "a composed stage ships the same definition-plus-closure unit as an ordinary site (`28K` §4), so the source set and its non-role index arrive together"
)]
fn ship_predict_stage(
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    checks: &[dorc_oracle::predict::PredictSet],
    interner: &Interner,
    provider: Symbol,
    argv: &[Symbol],
    node: dorc_analysis::cfg::CfgNodeId,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Option<dorc_plan::StageShip> {
    use dorc_oracle::predict::{
        PREDICT_SUFFIX, Resolution, StageStdout, evaluate, map_provider_name, predict_stage_stdout,
        strip_predict,
    };
    let want = map_provider_name(interner.resolve(provider));
    let named = |cs: &dorc_oracle::predict::PredictSet| {
        cs.providers()
            .find(|cp| map_provider_name(interner.resolve(*cp)) == want)
            .and_then(|cp| cs.get(cp).cloned())
    };
    // A composed stage is a SITE like any other (`28K` §2), through the SHARED seat rather than a
    // second copy of it — the open-coded twin this replaces was the same rule spelled twice, which
    // is the failure `oracle/CLAUDE.md the-frame-lookup-is-the-only-resolution-seat` records.
    let idx = shipping_source(
        checks.len(),
        node,
        live,
        &format!("{want}{PREDICT_SUFFIX}"),
        |i| checks.get(i).and_then(named).map(|p| p.span),
    )?;
    let check = checks.get(idx).and_then(named)?;
    let arg_texts: Vec<String> = argv
        .iter()
        .map(|s| interner.resolve(*s).to_owned())
        .collect();
    let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
    if !matches!(evaluate(&check, &arg_refs), Resolution::Resolved(_)) {
        return None;
    }
    let body = strip_predict(oracle_srcs.get(idx)?, &check, interner);
    let live_source = |name: &str| live.source_index_before(node, name);
    let closure = helpers
        .closure_for(
            idx,
            &body,
            dorc_oracle::closure::SiteFrame::at(&live_source),
        )
        .ok()?;
    Some(dorc_plan::StageShip {
        sh: format!("{}{body}", closure.sh()),
        produces_real_stdout: predict_stage_stdout(&check, &arg_refs) == StageStdout::RealBytes,
    })
}

/// Compile the resolver-probe (24F §3): for each resolver-bearing coordinate, ship its kind's
/// stripped `<kind>__resolve` funcdef + a per-coordinate invocation with the entity. Deterministic
/// (coords arrive `BTreeSet`-ordered). A coord whose kind's resolver cannot be resolved/stripped is
/// dropped (defensive — the kind is in `resolver_kinds` by construction, so this is unreachable, but
/// dropping degrades it to may-alias at readback, the safe direction).
fn compile_resolvers(
    coords: &BTreeSet<dorc_plan::EntityCoord>,
    kind_resolvers: &KindResolvers,
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    interner: &Interner,
) -> dorc_plan::ResolverPlan {
    use dorc_oracle::predict::strip_resolve;
    let mut probes = Vec::new();
    for coord in coords {
        let kind_sym = coord.kind().0;
        let Some((idx, resolver)) = kind_resolvers.get(kind_sym) else {
            continue;
        };
        let Some(src) = oracle_srcs.get(idx) else {
            continue;
        };
        let entity_text = match coord.entity() {
            dorc_core::EntityRef::Operand(tok) => interner.resolve(tok.0).to_owned(),
            dorc_core::EntityRef::Singleton => String::new(),
        };
        let body = strip_resolve(src, resolver, interner);
        // The kind-owner lanes ship their snapshot too (`FORFEITS:forfeit-survival-lanes-closure-less`,
        // captured): a resolver calling a helper shipped alone 127s and canonicalizes nothing, which
        // degrades to may-alias — safe, and a silent loss of every aliasing closure the author wrote.
        // Frameless: a resolver is a VOCABULARY act, loaded from the ambient prefix and
        // deliberately not routed through the positional oracle (`vocabulary-acts-stay-ambient`),
        // so there is no site whose frame could answer for it.
        let Ok(closure) =
            helpers.closure_for(idx, &body, dorc_oracle::closure::SiteFrame::unsolved())
        else {
            continue;
        };
        probes.push(dorc_plan::ResolverProbe {
            coord_label: render_coord(*coord, interner),
            kind_label: interner.resolve(kind_sym).to_owned(),
            kind_fn: format!(
                "{}__resolve",
                dorc_oracle::to_funcname_segment(interner.resolve(kind_sym))
            ),
            entity_text,
            sh: format!("{}{body}", closure.sh()),
        });
    }
    dorc_plan::ResolverPlan { probes }
}

/// The per-arm wrapper funcname a dynamic `reaches()` arm ships and is invoked under. Engine-
/// synthesized scaffolding, so def and invocation are one string by construction; the ROLE part is
/// taken from the shared suffix constant so the emitted namespace tracks the role's real spelling
/// (`289:rul-touches-mismatch-own-lane` — the half-landed respell left `__reaches_<n>` behind).
#[doc(hidden)]
#[must_use]
pub fn reach_arm_fn_name(kind_name: &str, arm_index: usize) -> String {
    format!(
        "{}{}_{arm_index}",
        dorc_oracle::to_funcname_segment(kind_name),
        dorc_oracle::reaches::DISTURBANCE_REACHES_ONLY_SUFFIX,
    )
}

/// Compile the reach-probe (24G §4): for each reach-bearing AUTHORED footprint coordinate, ship each
/// DYNAMIC `reaches()` arm's per-arm wrapper ([`reach_arm_fn_name`]`() { <arm bytes> ; }` — the arm
/// command's byte-exact span-slice, mark-free by construction) invoked with the entity; its stdout is
/// the RAW ENTITIES it drags. STATIC arms never ship (traced at expansion). Deduped by (coord, arm).
/// Dynamic arms apply to AUTHORED footprint coords only this pass (derived coords resolved only
/// post-results — the `resid-kindfn-derived` deferral, 24G §3). `inv-referent-agnostic`: the entity
/// text is resolved for the invocation, never decoded.
#[expect(
    clippy::too_many_arguments,
    reason = "the reach-probe compile threads the compiled context (classes/kills/value/touches/reaches/reach-kinds/oracle-srcs/interner) plus the `28K` §2 positional pair; each is a distinct pipeline output, not a bundle-able struct"
)]
fn collect_reach_probes(
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    kills: &BTreeSet<dorc_analysis::cfg::CfgNodeId>,
    value: &dorc_analysis::value::ValueFlow,
    touches_sets: &[dorc_oracle::touches::TouchesSet],
    reaches: &KindReaches,
    reach_kinds: &BTreeSet<Symbol>,
    oracle_srcs: &[String],
    helpers: &dorc_oracle::closure::HelperIndex,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> dorc_plan::ReachPlan {
    use dorc_analysis::effect::SkipClass;
    use dorc_oracle::reaches::{ArmOutcome, evaluate_reaches};
    let mut probes: BTreeMap<(String, usize), dorc_plan::ReachProbe> = BTreeMap::new();
    for (node, class) in classes {
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishProbeAmbient(_) | SkipClass::EstablishProbeWritten(_)
        ) || kills.contains(node);
        if !is_wall_candidate {
            continue;
        }
        let Some((_, fp_coords, _)) =
            resolve_touches_footprint(*node, value, touches_sets, interner, live)
        else {
            continue;
        };
        for (coord, _selector) in fp_coords {
            let kind_sym = coord.kind().0;
            if !reach_kinds.contains(&kind_sym) {
                continue;
            }
            let Some((idx, reaches_fn)) = reaches.get(kind_sym) else {
                continue;
            };
            let Some(src) = oracle_srcs.get(idx) else {
                continue;
            };
            let entity_text = entity_text_of(coord, interner);
            let coord_label = render_coord(coord, interner);
            let kind_name = interner.resolve(kind_sym).to_owned();
            let exp = evaluate_reaches(reaches_fn, &entity_text);
            for arm in &exp.arms {
                let ArmOutcome::Dynamic { cmd_span } = &arm.outcome else {
                    continue; // a STATIC arm ships nothing (traced at expansion)
                };
                let bytes = src
                    .get(cmd_span.lo.0 as usize..cmd_span.hi.0 as usize)
                    .unwrap_or_default()
                    .trim();
                let arm_fn = reach_arm_fn_name(&kind_name, arm.index);
                // The arm's own snapshot precedes the engine-synthesized wrapper (the same capture as
                // the resolver lane). A denial drops the arm, which walls the footprint total — the
                // at-most claim's conservative direction (`an-at-most-claim-has-two-atomicities`).
                // Frameless for the same reason the resolver lane is: a `reaches` arm is a
                // vocabulary act, ambient by design (`vocabulary-acts-stay-ambient`).
                let Ok(closure) =
                    helpers.closure_for(idx, bytes, dorc_oracle::closure::SiteFrame::unsolved())
                else {
                    continue;
                };
                let arm_sh = format!("{}{arm_fn}() {{ {bytes} ; }}", closure.sh());
                probes
                    .entry((coord_label.clone(), arm.index))
                    .or_insert(dorc_plan::ReachProbe {
                        coord_label: coord_label.clone(),
                        kind_label: kind_name.clone(),
                        arm_fn,
                        arm_index: arm.index,
                        entity_text: entity_text.clone(),
                        arm_sh,
                    });
            }
        }
    }
    dorc_plan::ReachPlan {
        probes: probes.into_values().collect(),
    }
}

/// Lift the per-site GUARD VOUCHES (rul-guard-license / rul24-vouch-is-verdict-authoring, 24A §1c).
/// Called ALWAYS-ON — guards are the un-flagged baseline (rul24-mode-gate governs only the survival
/// tier, NOT this). For each establish-bearing site whose provider authored a verdict function
/// (`<provider>.is_converged`) that REACHES a vouching path over the site's resolved
/// argv (`evaluate_verdict` ⇒ `Vouched`), build a [`dorc_plan::Vouches`] entry: a
/// `ByVouch<VerdictVouch>` carrying the guard emitter's data (the mangled funcname, the strip-only
/// preamble, the invocation, the declared sense, the fact's kind label), keyed by the site's
/// `CfgNodeId`. A `Declined` (unhandled path — hz-refusepath: a refuse path that returns 0
/// vacuously never vouches) or ⊤ (P-topargv: an unpropagatable argv) resolution, or no verdict
/// function, ⇒ absence from the map ⇒ the site never guards (no vouch ⇒ run — the judgment tier the
/// map carries is exactly what [`dorc_plan::GuardLicense::mint`] DEMANDS, TC-tier-2).
///
/// Verdict-lift diagnostics are surfaced AS-IS (inv-top-reject: under-modeling is a loud
/// correctness boundary, never a silent degrade). The Part-A `tc-verdict-return` softening
/// (⊤-reject → warning) is REVERTED (find-return-vouches, 24C): the tracer now models a reached
/// `return N` as a DECLINE and the corpus arity-refuse is spelled in-dialect
/// (`if [ … ]; then return N; fi`), so no corpus verdict body ⊤-rejects — one that still does is
/// genuinely out of dialect and SHOULD fail loudly. A verdict function that fails to lift yields
/// no vouch (the site runs, kFAIL-perform) regardless.
///
/// `inv-referent-agnostic`: the kind label + operands are resolved for the invocation/attribution,
/// never decoded for meaning; the vouch travels the site's own value-flow (the 24A §1b fence).
///
/// The `verdict_sets` are the driver's WITHDRAWN ones. This seat re-lifting them from source read
/// a population every other seat had already narrowed (`28P:fnd-build-vouches-relifted-the-verdict-sets`).
#[expect(
    clippy::too_many_arguments,
    reason = "the reshaping edge over `dorc_plan::build_vouches_from_sets`; each argument is a \
              distinct world that lift reads, and the source PATHS are the caller's by law \
              (`AID:law-lineno-identity`)"
)]
fn build_vouches(
    oracle_refs: &[&str],
    oracle_paths: &[&str],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    helpers: &dorc_oracle::closure::HelperIndex,
    classes: &[(
        dorc_analysis::cfg::CfgNodeId,
        dorc_analysis::effect::SkipClass,
    )],
    value: &dorc_analysis::value::ValueFlow,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Carrier<(dorc_plan::Vouches, dorc_plan::VouchLiftAid)> {
    // The composition lives in `dorc_plan::build_vouches_from_sets` (the ONE home — the
    // sweep/coverage DSTs share its re-lifting sibling). This edge only RESHAPES the lift: its
    // diagnostics ride out AS-IS (inv-top-reject — the tc-verdict-return softening is reverted,
    // find-return-vouches 24C), so a genuinely out-of-dialect verdict body fails gate-3's
    // error-floor rather than degrading silently.
    let (lifted, aid) = dorc_plan::build_vouches_from_sets(
        oracle_refs,
        oracle_paths,
        verdict_sets,
        helpers,
        classes,
        value,
        interner,
        live,
    );
    lifted.map(|vouches| (vouches, aid))
}

/// gate-5 / cm-2 readout: per command site, emit `argv <leafid> <disposition> <word|TOP
/// per word>` on stderr (a resolved literal verbatim, an unresolved word `TOP`). The
/// leaf-ids are the plan's own ([`dorc_plan::Step::leaf`]) — the same span-sorted space the
/// probe records share (`inv-site-keyed-results`), so `argv N` keys to the same site as
/// `site N`. The argv is the book-side value-flow
/// ([`dorc_analysis::value::ValueFlow::argv_values`]), keyed by `CfgNodeId` (mapped back
/// from the leaf's `AstId`). Cli-edge only.
///
/// The `<disposition>` tag (task-O / `tc-gate5-omit`, strain-D3b-fold-vs-gate5): one of
/// `run`/`replace`/`omit`, so gate-5 can SKIP a site the plan does not run. An `Omit`ted or
/// `Replace`d site legitimately never appears in the bare book's argv log when a preceding
/// guard short-circuits it (e.g. a shimmed Query-guard fold) — asserting it ⊆ the log would
/// be a false failure, the exact structural exclusion that confined the fold/omit
/// demonstration to builtin guards (20G §5). Filtering on `run` removes that exclusion
/// without weakening the gate for the sites that DO run.
fn emit_debug_argv(
    sink: &mut dyn OutputSink,
    plan: &dorc_plan::Plan,
    cfg: &dorc_analysis::cfg::Cfg,
    value: &dorc_analysis::value::ValueFlow,
    interner: &Interner,
) {
    use dorc_analysis::value::ValueOf;
    // AstId → CfgNodeId for Command nodes (argv_values is keyed by CfgNodeId; the plan
    // step carries the AstId). One CfgNode per command AstId in the modeled subset.
    let node_of_ast: BTreeMap<dorc_core::AstId, dorc_analysis::cfg::CfgNodeId> = cfg
        .iter()
        .filter(|(_, n)| n.kind == dorc_analysis::cfg::CfgNodeKind::Command)
        .map(|(id, n)| (n.ast, id))
        .collect();
    for step in plan.steps() {
        let Some(&node) = node_of_ast.get(&step.ast) else {
            continue;
        };
        let words: Vec<String> = value
            .argv_values(node)
            .into_iter()
            .map(|w| match w {
                ValueOf::Literal(sym) => interner.resolve(sym).to_string(),
                ValueOf::Top(_) => "TOP".to_string(),
            })
            .collect();
        emit_stderr!(
            sink,
            "argv {} {} {}",
            step.leaf.0,
            disposition_tag(&step.disposition),
            words.join(" ")
        );
    }
    // gate-6 `guardcmd` attribution (23A §5): one line per DISTINCT check-command a GUARDED site's
    // verdict body runs (`guardcmd dpkg-query`). The widened dual-rail judge allowlists these as
    // legitimate apply-only lines (the guard's live check runs at apply, absent from the bare
    // book) — never an unrelated one (cf-5). Deterministic (`BTreeSet`, `inv-determinism`).
    let region_verb: BTreeMap<dorc_core::AstId, &str> = plan
        .regions()
        .iter()
        .map(|region| (region.ast, disposition_tag(&region.disposition)))
        .collect();
    for step in plan.steps() {
        let Some(&node) = node_of_ast.get(&step.ast) else {
            continue;
        };
        let Some(sites) = cfg.call_body_sites(node) else {
            continue;
        };
        let call_verb = disposition_tag(&step.disposition);
        for &site in sites {
            // A replaced/omitted CALL licenses its whole body; otherwise the region answers.
            let verb = if matches!(call_verb, "replace" | "omit") {
                call_verb
            } else {
                region_verb
                    .get(&cfg.node(site).ast)
                    .copied()
                    .unwrap_or("run")
            };
            let words: Vec<String> = value
                .argv_values(site)
                .into_iter()
                .map(|w| match w {
                    ValueOf::Literal(sym) => interner.resolve(sym).to_string(),
                    ValueOf::Top(_) => "TOP".to_string(),
                })
                .collect();
            emit_stderr!(sink, "region {} {verb} {}", step.leaf.0, words.join(" "));
        }
    }
    let mut guard_cmds: BTreeSet<&str> = BTreeSet::new();
    for disposition in plan
        .steps()
        .iter()
        .map(|step| &step.disposition)
        .chain(plan.regions().iter().map(|region| &region.disposition))
    {
        if let dorc_plan::Disposition::Guard(license) = disposition {
            for c in license.insert().check_cmds() {
                guard_cmds.insert(c.as_str());
            }
        }
    }
    for c in &guard_cmds {
        emit_stderr!(sink, "guardcmd {c}");
    }
}

/// q-2 (`dq-site-unresolvable`) + cheap-7 (the firehose fix): ONE aggregated Note for the
/// probe-unresolvable sites that are worth disclosing — never the per-site stanza-per-site
/// firehose the recon flagged (a 5,000-line book emitted 50,002 stderr lines). Two moves:
///
/// 1. **SUPPRESS the structurally-unprobeable** ([`is_structurally_unprobeable`]): a bare
///    assignment (`pkg=nginx`), `set -eu`, or a pure/no-target-state builtin (`:`/`echo`/`cd`/…,
///    the ENGINE's own [`dorc_analysis::effect::is_target_state_pure_builtin`] list) has NO probe
///    that could ever exist — "declare a read-only probe for `set -eu`" is actively wrong advice —
///    so it earns NO disclosure at all.
/// 2. **AGGREGATE the remainder** into ONE honest Note naming every real command (`make install`,
///    an un-oracled `apt-get …`) and pointing at `dorc why` for the per-site detail. The frame
///    points at the FIRST real site as a representative (a caret example), constant-size regardless
///    of how many sites run unprobed. rul-attention-honesty is intact: the artifact still RUNS every
///    one (this only collapses the stderr readout — no run is hidden).
///
/// Reuses the migrated `DiagCode::SiteUnresolvable` spine (the sanctioned emit for this slug —
/// tidy-gate reachability unchanged). The `unresolvable` [`LeafId`]s share the apply plan's
/// span-sorted site space (`inv-site-keyed-results`), so each maps to a [`dorc_plan::Step`]'s `ast`,
/// whose span resolves to the book source. A site with no matching step is ASSERTED-UNREACHABLE
/// (human ruling 22-q2: `unresolvable ⊆ plan.steps()` by construction) then skipped — `debug_assert`
/// loud in debug/DST, safe-degrade (skip) in release (never-vouch: the reachability claim is ours).
/// plans/240 Stage-1 yardstick: emit the plan-summary — a one-line, greppable, stable-grammar
/// readout of the per-disposition tally (the round's north-star metric, elision frequency) — on
/// stderr, the render surface. rec-1 TWO SURFACES: NEVER woven into the byte-floored `.sh`
/// artifact on stdout. The cli emits it in every plan-building mode (`probe` returns before any
/// plan exists, so it emits none). Shaped `dorc: plan-summary …`, never `<stage>: error[…]`, so
/// the e2e gate-3 stderr floor (keyed on the `error[` shape) ignores it. Counts derive from the
/// Plan value alone (`inv-determinism`).
fn emit_plan_summary(sink: &mut dyn OutputSink, plan: &dorc_plan::Plan) {
    let counts = plan.disposition_counts();
    let parts = chrome_parts(
        &sink.render_ctx(),
        "cli-plan-summary-line",
        &[
            &counts.sites.to_string(),
            &counts.elide.to_string(),
            &counts.omit.to_string(),
            &counts.guard.to_string(),
            &counts.run.to_string(),
            &plan.survival_report.may_alias_fires().to_string(),
        ],
    );
    sink.emit(OutputEvent::plain_tagged(OutputChannel::Stderr, parts));
}

/// stage-3 (the why-lens render, `22D` §1): surface — on stderr, the RENDER surface — the
/// per-line "why did this command RUN (never elided)?" disclosure for each forced-run command
/// whose ⊤ carries a wired cause. The render + stage-4 dedup is [`why_lens_lines`] (pure,
/// unit-testable); this is just its stderr driver.
///
/// rec-1 WELD (two surfaces): this prints to STDERR only — the plan-render surface. It is NEVER
/// woven into the byte-floored `.sh` artifact on stdout (the artifact stays receipt-free). The
/// line is prefixed `why:` and never `error[`, so the e2e gate-3 stderr-floor (which keys on the
/// `<stage>: error[` shape) ignores it — the why-lens is additive, never a case-failing diagnostic.
///
/// `_collapse_narrative` is the C3/C4 decision-inert narrative seam (`27V` Lane A): the collapse
/// records the why-lens will render (d4). Carried through here and IGNORED for now — the render
/// arrangement is d4's, so surfacing it early would freeze `render-form-unwelded` output.
fn emit_why_lens(
    sink: &mut dyn OutputSink,
    why_diags: &[Diag],
    arena: &ProvArena,
    src: &str,
    _collapse_narrative: &[CollapseNarrative],
) {
    let lines = {
        let ctx = sink.render_ctx();
        why_lens_reasons(&ctx, why_diags, arena, src)
            .iter()
            .map(|reason| why_lens_line(&ctx, reason))
            .collect::<Vec<_>>()
    };
    for line in lines {
        emit_stderr!(sink, "why: {line}");
    }
}

/// The stderr lens's render seat: one reason's fragments, stamped as runs and concatenated.
///
/// It stamps the SAME runs the `dorc why` report hands weft (`Said::runs`), and then throws the
/// attribution away, because a bare stderr line has no span map to carry it. Going through the
/// stamp anyway is what keeps the two surfaces from drifting — every fragment is classed once, so
/// the book's own bytes are encoded here for the same reason they are there
/// (`ask-why-lens-stderr-unencoded`).
fn why_lens_line(ctx: &dorc_aid::RenderCtx<'_>, reason: &Said) -> String {
    reason
        .runs(ctx, "why-lens")
        .iter()
        .map(|run| run.text.as_str())
        .collect()
}

/// Stage 2 attribution (TC-3 / rul24-divergence-is-the-game): emit, on the why-lens stderr
/// lane (the `why: ` prefix, alongside the run-cause disclosures — one lens, two directions:
/// why-a-line-runs and why-a-line-survived), one line per SURVIVED elision — naming the
/// surviving site, each running wall it crossed, whose footprint licensed the crossing (the
/// provider and its claimed coordinates), and the backing coordinate proven disjoint. Reads the
/// [`dorc_plan::SurvivalWitness`] the wall walk minted — NEVER recomputes disjointness (the
/// witness IS the attribution). rec-1 WELD: stderr render surface only; the byte-floored `.sh`
/// artifact stays receipt-free (a survived elision's artifact bytes are identical to any other
/// elision's). Never `error[`, so the gate-3 stderr floor ignores it; the `why: ` prefix lets
/// gate-7 (`expected-why`) pin the attribution end-to-end.
fn emit_survival_attribution(
    sink: &mut dyn OutputSink,
    plan: &dorc_plan::Plan,
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) {
    for step in plan.steps() {
        let dorc_plan::Disposition::Replace(license, _) = &step.disposition else {
            continue;
        };
        let Some(survival) = &license.derivation().survival else {
            continue;
        };
        let members: Vec<_> = match survival {
            dorc_plan::SurvivalAttribution::Standalone(witness) => {
                vec![(witness, license.derivation().vouch_span)]
            }
            dorc_plan::SurvivalAttribution::Aggregate(witness) => witness
                .members()
                .map(|member| {
                    let defining = license
                        .derivation()
                        .establish_vouches
                        .iter()
                        .find(|receipt| {
                            receipt.site == member.site() && receipt.fact == member.fact()
                        })
                        .and_then(|receipt| receipt.defining_span);
                    (member.survival(), defining)
                })
                .collect(),
        };
        for (witness, defining) in members {
            let crossings: Vec<String> = witness
                .crossings()
                .iter()
                .map(|c| {
                    let provider = interner.resolve(c.provider());
                    let coords: Vec<String> = c
                        .footprint()
                        .iter()
                        .map(|fc| render_coord(*fc, interner))
                        .collect();
                    // 24E §9: name a host-DERIVED footprint's provenance ("DERIVED at probe from
                    // <call>"); an authored (static) footprint carries no extra locus.
                    let origin = match c.origin() {
                        dorc_plan::FootprintOrigin::Derived { call } => {
                            format!("; DERIVED at probe from {call}")
                        }
                        dorc_plan::FootprintOrigin::Authored => String::new(),
                    };
                    // 24F §6: name the resolver that canonicalized this crossing's coords ("disjoint
                    // AFTER <kind>.resolve()"). The aliasing closure is the sharpest claim in the design,
                    // so a survival it licensed must always name whose identity-judgment it trusted.
                    let via = c.via_resolver().map_or_else(String::new, |k| {
                        format!(
                            "; disjoint AFTER {}.resolve() canonicalization",
                            interner.resolve(k.0)
                        )
                    });
                    // 24G §8: name the engine-supplied OWN-effect coordinate distinctly — present only
                    // when the union WIDENED the footprint (the derived lane; the authored lane's canary
                    // folds own into the `touches()` claim, so it is not repeated). Provenance: the site's
                    // declared effect, NOT the author's claim.
                    let own = c.own().map_or_else(String::new, |o| {
                        format!("; own-effect {}", render_coord(o, interner))
                    });
                    format!(
                        "wall site {} ({provider} touches {{{}}}{own}{origin}{via})",
                        c.wall_leaf().0,
                        coords.join(" ")
                    )
                })
                .collect();
            let locus = oracle_locus(defining, oracle_paths, oracle_srcs)
                .map(|value| format!("; vouched at {value}"))
                .unwrap_or_default();
            emit_stderr!(
                sink,
                "why: site {} survives+elides past {} -- backing {} disjoint (trusted footprint){locus}",
                step.leaf.0,
                crossings.join(", "),
                render_coord(witness.backing(), interner),
            );
        }
    }
}

/// The REACH-POISON why-lane (24G Part B): one `why:` line per converged elision that DEMOTED to run
/// because a `<kind>.reaches()` EXPANSION coordinate hit its backing — the cross-author demote the
/// reach mechanism exists for. Mirrors the resolver-attribution shape (the sharpest claims name whose
/// knowledge they trusted): here the demote names the reach-function whose widening caught the
/// otherwise-wrongly-surviving elision. rec-1 WELD: stderr render surface only. Never `error[`, so the
/// gate-3 floor ignores it; the `why: ` prefix lets the render surface pin it.
fn emit_reach_poisonings(sink: &mut dyn OutputSink, plan: &dorc_plan::Plan, interner: &Interner) {
    for (leaf, kind) in plan.survival_report.reach_poisonings() {
        emit_stderr!(
            sink,
            "why: site {} runs -- poisoned via {}.reaches() (a reach-expanded coordinate hit its \
             backing; the wall drags it cross-author)",
            leaf.0,
            interner.resolve(kind.0),
        );
    }
}

/// The GUARD why-lane (rul-guard-license / X-why): one `why:` line per guarded site, naming
/// (i) the mechanism (`guard`), (ii) the license (a converged-`vouch`), (iii) the vouching oracle
/// (the fact's kind) — the `guard23-why-attribution` conjoined pattern (`guard && vouch && <kind>`
/// in ONE line). Attribution is the guard-license's whole enforcement story ("we can't prevent, so
/// we attribute" — plans/233 §guard-license); rul-attention-honesty makes it load-bearing (a guard
/// the user can't trace to its licensor is hidden risk). rec-1 WELD: stderr render surface only —
/// the byte-floored artifact carries the inline `# dorc: guard …` comment; this is the disclosure.
/// Never `error[`, so the gate-3 floor ignores it; the `why: ` prefix lets gate-7 pin it.
fn emit_guard_attribution(
    sink: &mut dyn OutputSink,
    plan: &dorc_plan::Plan,
    interner: &Interner,
    oracle_paths: &[String],
    oracle_srcs: &[String],
) {
    // A render-REFUSED guard (heredoc / non-devnull output redirect) does NOT guard the site — the
    // mutator runs verbatim. rul-attention-honesty: never claim a skip that did not happen; disclose
    // the refusal (gate-7 `refus`) instead of the licensing line.
    let refused = plan.guard_refused_asts();
    for step in plan.steps() {
        let dorc_plan::Disposition::Guard(license) = &step.disposition else {
            continue;
        };
        let kind = interner.resolve(license.fact().kind.0);
        if refused.contains(&step.ast) {
            emit_stderr!(
                sink,
                "why: site {} guard refused -- the site's structurally-awkward form (a heredoc \
                 body, or a non-`/dev/null` output redirect) would corrupt the artifact or suppress \
                 an admin-spelled side-effect, so the original bytes RUN VERBATIM (to stay safe), \
                 the {kind} oracle's vouch that it is already satisfied notwithstanding",
                step.leaf.0,
            );
        } else {
            // C7: the vouch's defining `file:line` (the reached check arm), when the plan threaded it.
            let locus = oracle_locus(license.insert().defining_span(), oracle_paths, oracle_srcs)
                .map(|l| format!(" (at {l})"))
                .unwrap_or_default();
            emit_stderr!(
                sink,
                "why: site {} guard [{kind}] -- licensed by the {kind} oracle's vouch{locus} that \
                 it is already satisfied; the original bytes survive and the check re-runs live at \
                 apply (to stay safe)",
                step.leaf.0,
            );
        }
    }
}

/// Every pure-predicate-CARRY elision names, on the why-lens lane, its cross-context attribution
/// chain (`27C` §9 / steering `pure-predicate-carry`): the crossed substrate axes, each backing
/// kind's owner `invariant:<axis>` line (vouch-species), and the engine read-set-closure proof. The
/// block acceptance demands this "from day one" — an UNFLAGGED cross-boundary answer resting on a
/// kind-owner's typed line + an engine proof MUST disclose whose line and what proof licensed it.
/// `carried` is keyed by the site's `AstId` (built at the carry decision); this re-keys to the plan's
/// per-site number. Empty when no site carried. Deterministic (plan step order).
fn emit_carry_attribution(
    sink: &mut dyn OutputSink,
    plan: &dorc_plan::Plan,
    carried: &BTreeMap<dorc_core::AstId, String>,
) {
    for step in plan.steps() {
        if let Some(text) = carried.get(&step.ast) {
            emit_stderr!(sink, "why: site {} {text}", step.leaf.0);
        }
    }
}

/// The why-lens reasons + stage-4 dedup, factored PURE (the stderr side is [`emit_why_lens`]) so
/// the dedup is unit-testable (`x2-fd1`). For each caused-⊤ diag it takes the "why did this run"
/// reason from [`dorc_aid::diag::why`], showing a given cause-SITE once.
///
/// stage-4 DEDUP KEY = `(cause, site)`, NOT the cause [`dorc_core::ProvId`] alone (`x2-fd1` fix,
/// `224` §10): under function inlining two call-sites splice the SAME body `AstId` (`inv-leaf-seam`)
/// ⇒ both `CmdsubOperandTop` diags hash-cons to ONE cause `ProvId`. Keying on cause alone collapsed
/// two GENUINELY INDEPENDENT forced runs (suppressing the 2nd `why:` — the over-suppression). They
/// differ by `site` (the stable `site N.M` leaf), so `(cause, site)` keeps them separately disclosed
/// while still deduping a true re-disclosure (same cause AND same site). Tracked in a `Vec` of
/// first-occurrences — `ProvId` is `!Ord` (no `BTreeSet`) and the diags arrive in node order, so
/// first-seen order is deterministic (`inv-determinism`). The only suppression built (no general
/// subsystem — `22D` §1 stage-4).
fn why_lens_reasons(
    ctx: &dorc_aid::RenderCtx<'_>,
    why_diags: &[Diag],
    arena: &ProvArena,
    src: &str,
) -> Vec<Said> {
    let mut shown: Vec<(dorc_core::ProvId, dorc_aid::diag::SiteId)> = Vec::new();
    let mut reasons = Vec::new();
    for diag in why_diags {
        if let Some(key) = cmdsub_cause_site(diag) {
            if shown.contains(&key) {
                continue; // stage-4: this (cause, site) was already explained — show it once
            }
            shown.push(key);
        }
        if let Some(explanation) = dorc_aid::diag::why(ctx, diag, arena, src) {
            reasons.push(Said::Parts(explanation.parts));
        }
    }
    reasons
}

/// The stage-4 render-dedup key a why-lens diag carries, if any: `(⊤-cause, site)`. Only a
/// `CmdsubOperandTop` carries a cause at HEAD (stage-1); any other diag returns `None` (the why-lens
/// does not explain it anyway, fd-G), so it never participates in the dedup. The `site` half is what
/// separates two inlined call-sites sharing one cause `ProvId` (`x2-fd1`).
fn cmdsub_cause_site(diag: &Diag) -> Option<(dorc_core::ProvId, dorc_aid::diag::SiteId)> {
    match &diag.code {
        DiagCode::CmdsubOperandTop(p) => p.cause.map(|c| (c, p.site)),
        _ => None,
    }
}

/// The gate-5 disposition tag for a [`dorc_plan::Disposition`] — `run`/`replace`/`omit`.
/// gate-5 asserts the bare-book argv-echo ONLY for `run` sites: a `replace`d or `omit`ted
/// site is deliberately not in the apply run-set, and a guarded omit may be absent from the
/// BARE book too (a preceding guard short-circuits it), so it must not be asserted ⊆ the
/// log (task-O / strain-D3b-fold-vs-gate5).
fn disposition_tag(disposition: &dorc_plan::Disposition) -> &'static str {
    use dorc_plan::Disposition;
    match disposition {
        Disposition::Run => "run",
        Disposition::Replace(_, _) => "replace",
        Disposition::Omit { .. } => "omit",
        // A guard's ledger tag (gate-6's widened judge reads it — cf-5/cf-6): gate-5 skips it (a
        // guarded site's run-set argv is the check invocation, not the bare book's mutator argv).
        Disposition::Guard(_) => "guard",
    }
}

/// The rc a `128 + SIGPIPE` early-exit race lands on (`sigpipe-flap-class`, `279f` §5):
/// a `pipefail`-off pipeline whose early-exit consumer (`… | grep -q`) closed the pipe before an
/// upstream stage finished writing produces this race-dependently. It is opaque to Dorc's verdict
/// (a ≥2 flat-sink landing ⇒ cant-tell ⇒ run), so it is a WHY-lane nudge, never a decision input.
const SIGPIPE_RC: i32 = 141;

/// Emit the `sigpipe-flap-class` why-lane note (`279f` §5) for every probe record that landed
/// [`SIGPIPE_RC`]: a stderr advisory suggesting a full-read form over the early-exit `| grep -q`.
/// The landing is always SAFE (the site runs) and the verdict never flaps run-to-run, so this is a
/// pure nudge — no gate asserts it, and it feeds no decision. Ordered (records is a `BTreeMap`).
fn emit_sigpipe_race_notes(sink: &mut dyn OutputSink, results: &SiteResults) {
    for (key, rec) in &results.records {
        if rec.rc.0 == SIGPIPE_RC {
            let site = match key.member {
                Some(m) => format!("{}.{m}", key.site.0),
                None => key.site.0.to_string(),
            };
            emit_stderr!(
                sink,
                "note: site {site} landed rc 141 (likely benign early-exit SIGPIPE race; \
                 consider a full-read form over `| grep -q`)"
            );
        }
    }
}

/// The engine-owned display word for a decline class (`27W:rul-class-starter-set`). Delegates to
/// the one home ([`dorc_aid::narrative::DeclineClass::token`]); display only
/// (`inv-referent-agnostic`; spellings ride `27V:rul-output-form-unwelded`).
fn decline_class_word(class: dorc_aid::narrative::DeclineClass) -> &'static str {
    class.token()
}

/// Emit the report lane's SELECTED default disclosure (`27W` §2 · `decline-class-emission`): one
/// advisory note per RECOGNIZED author decline-class (the class-routing an admin sees by default).
/// The unrecognized / free-form NOISE is retained in `results.reports`
/// (`27W:rul-report-noise-tolerant`) but printed only at max verbosity — d4's surface, not this
/// default. THINNEST surface pending d4's arrangement (`27V:rul-output-form-unwelded`): the wording
/// re-blesses freely. Empty in the corpus (no oracle emits report lines ⇒
/// `empty-world-byte-identical`). `note:` prefix ⇒ never crosses the gate-3 error floor.
fn emit_report_lane_notes(sink: &mut dyn OutputSink, results: &SiteResults) {
    for r in &results.reports {
        // Default surface = recognized records; noise waits for d4's max verbosity.
        let Some(class) = r.class.filter(|_| r.recognized) else {
            continue;
        };
        let at = match r.site {
            Some(k) => match k.member {
                Some(m) => format!(" at site {}.{m}", k.site.0),
                None => format!(" at site {}", k.site.0),
            },
            None => String::new(),
        };
        let tail = r.raw.splitn(3, ' ').nth(2).unwrap_or(r.raw.as_str());
        emit_stderr!(
            sink,
            "note: author declines [{}]{at} -- {tail}",
            decline_class_word(class)
        );
    }
}

/// C3 report-lane pairing (`27W` §3 `rul-static-first-three-tier`): fold each ingested tier-3 report
/// record (a recognized decline class + a site) into that site's
/// [`dorc_aid::narrative::CollapseKind::VerdictDecline`] narrative via
/// [`dorc_aid::narrative::CollapseNarrative::with_authored_reason`]. The runtime record supplies only
/// the missing CLASS — a dynamic format string defeated static reading (`27W` §2 "one honest loss"),
/// so `classify_decline` traced the reached decline arm but left the class unread. The arm span +
/// file id are the [`dorc_aid::narrative::CollapseKind::VerdictDecline`]'s OWN already-traced reached
/// arm (the precise `file:line`,
/// `27V:mech-minting-line-threading`); the inventory-keyed lookup the spec sketched is redundant here
/// — a class-readable inventory arm implies static ALREADY populated the reason. Deduped
/// `(site, arm, class)` against tier-2: `with_authored_reason` is idempotent, so a runtime echo never
/// overwrites a statically populated reason (static wins). Empty in the corpus (no oracle emits ⇒
/// `empty-world-byte-identical`). Decision-inert (`two-plane-aid-law`): classes route AID only.
fn pair_authored_reasons(
    narratives: Vec<CollapseNarrative>,
    reports: &[ReportRecord],
) -> Vec<CollapseNarrative> {
    use dorc_aid::narrative::{AuthoredReason, CollapseKind};
    narratives
        .into_iter()
        .map(|ev| {
            let CollapseKind::VerdictDecline {
                site,
                arm,
                arm_file,
                ..
            } = *ev.kind()
            else {
                return ev;
            };
            // A recognized runtime record for this exact site (leaf + member), if one arrived.
            let paired = reports.iter().find_map(|r| {
                let rk = r.site?;
                let matches = r.recognized
                    && dorc_aid::diag::SiteId {
                        leaf: rk.site,
                        member: rk.member,
                    } == site;
                matches.then_some(r.class).flatten()
            });
            match paired {
                Some(class) => ev.with_authored_reason(AuthoredReason {
                    class,
                    arm,
                    arm_file,
                }),
                None => ev,
            }
        })
        .collect()
}

/// Emit the STATIC/paired decline-class disclosure (`27W` §3 `rul-static-first-three-tier`): one
/// why-lens line per site whose `VerdictDecline` narrative carries an `authored_reason` — the tier-2
/// static class (argv threaded statically) OR the tier-3 runtime class paired in by
/// [`pair_authored_reasons`]. A DECLINE is a why-a-line-RUNS disclosure (the author declined ⇒ the
/// site runs), so it belongs on the same `why:` lane as the run/survival attributions, surfacing the
/// emitting arm's `file:line` (C7). The tier-3 RUNTIME records ALSO take the advisory `note:` lane
/// ([`emit_report_lane_notes`]) for their free-tail text. The full why-lens CHAIN render (numbered
/// links, tier words) is the arrangement walker. Empty in the corpus (no oracle emits ⇒
/// `empty-world-byte-identical`); wording rides `27V:rul-output-form-unwelded`. Never `error[` ⇒
/// ignored by the gate-3 floor; the `why:` prefix lets gate-7 pin it.
fn emit_static_decline_notes(
    sink: &mut dyn OutputSink,
    collapse_narrative: &[CollapseNarrative],
    oracle_paths: &[String],
    oracle_srcs: &[String],
) {
    for line in static_decline_notes(collapse_narrative, oracle_paths, oracle_srcs) {
        emit_stderr!(sink, "{line}");
    }
}

/// The pure half of [`emit_static_decline_notes`] — the ONE narrative-consuming render seat, split
/// out so it is assertable in-process (`289:rul-mint-hardening-package` item 4b: the "and the chain
/// renders it" clause, satisfiable today for exactly this class). Wording rides
/// `27V:rul-output-form-unwelded`.
fn static_decline_notes(
    collapse_narrative: &[CollapseNarrative],
    oracle_paths: &[String],
    oracle_srcs: &[String],
) -> Vec<String> {
    let mut lines = Vec::new();
    for ev in collapse_narrative {
        let CollapseKind::VerdictDecline {
            authored_reason: Some(reason),
            ..
        } = ev.kind()
        else {
            continue;
        };
        let at = oracle_locus(
            Some((reason.arm.0, reason.arm_file)),
            oracle_paths,
            oracle_srcs,
        )
        .map(|l| format!(" (at {l})"))
        .unwrap_or_default();
        lines.push(format!(
            "why: author declines [{}]{at} -- a deliberate decline the author classed, so the site \
             runs",
            decline_class_word(reason.class)
        ));
    }
    lines
}

/// A deterministic content digest binding the records stream to the exact analyzed book bytes
/// (`262` §2 `book=`; discharges `tc-probe-no-digest`). Hand-rolled SHA-256 in the kernel
/// (`28F:rul-digest-lands-now` retired the FNV-1a-64 stand-in; the kernel stays dependency-clean
/// per `inv-determinism`, so it is written out rather than pulled in). Computed at the I/O edge
/// (`io-at-edges-only`), never in the kernel.
fn book_digest(book_src: &str) -> String {
    dorc_plan::invocation::book_digest(book_src)
}

/// The zero-matched-sites oracle warning (`30Qe:fruit-oracle-matched-zero-sites`; `KNOBS:kWARN`
/// rich, tune-high) — the silent-decline value-evaporation detector: a LOADED oracle whose
/// `__is_converged` verdict never actually vouched any site this run, either because the book
/// never invokes its command at all, or every invocation's argv declined.
///
/// `oracle_paths`/`verdict_sets` are index-paired: oracle files sort FIRST in the source-wide
/// vectors the verdict lift consumes, with the book last (`cli/CLAUDE.md
/// the-book-is-a-definition-source`), and `oracle_paths` names only the oracle-only prefix — so
/// zipping them walks exactly the loaded oracle files, in order, never the book. `vouches` is the
/// run's FINAL, frozen vouch set.
///
/// Conservative by construction, and documented so, not silently: the comparison is FAMILY-NAME
/// level (the munged `<name>__is_converged`), so a family whose command word DOES appear in the
/// book but whose argv every shape declines still correctly reads as zero-matched (no vouch
/// reached) — but a book-defined verdict that SHADOWS an oracle's same-named family
/// (`oracle/CLAUDE.md visibility-is-full-positional`: at most one definition is live per name) is
/// indistinguishable from the oracle's own vouch by `fn_name` alone, so a shadowed oracle can read
/// as matched when its own body never ran. A rare edge, not chased here.
fn oracle_matched_zero_sites_diagnostics(
    oracle_paths: &[String],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    run_vouches: &dorc_plan::Vouches,
    interner: &Interner,
) -> Vec<Diag> {
    let vouched = run_vouches.vouched_fn_names();
    oracle_paths
        .iter()
        .zip(verdict_sets)
        .filter_map(|(path, set)| {
            let defined: Vec<String> = set
                .providers()
                .map(|sym| {
                    format!(
                        "{}__is_converged",
                        dorc_oracle::to_funcname_segment(interner.resolve(sym))
                    )
                })
                .collect();
            let matched =
                !defined.is_empty() && defined.iter().any(|f| vouched.contains(f.as_str()));
            (!defined.is_empty() && !matched).then(|| {
                Diag::new_spanless_site(DiagCode::OracleMatchedZeroSites(OracleMatchedZeroSites {
                    oracle: path.clone(),
                }))
            })
        })
        .collect()
}

/// The paste/splice-floor damage-watch diagnostics (`30Qe:fruit-emit-hygiene-paste-rules`;
/// `KNOBS:kBOOT`) — one [`DiagCode::EmittedLineUnsafeForPaste`] per
/// [`dorc_plan::render::PasteHygieneHazard`] [`dorc_plan::render::paste_hygiene_hazards`] finds in
/// `rendered`, the FINALIZED artifact bytes (post `with_plan` — the exact bytes stdout or a
/// published tree ships). Spanless: the claim is about a RENDERED PHYSICAL LINE, which has no
/// book-AST span.
fn emitted_line_unsafe_for_paste_diagnostics(rendered: &str) -> Vec<Diag> {
    dorc_plan::render::paste_hygiene_hazards(rendered)
        .into_iter()
        .map(|hazard| {
            let (line, reason) = match hazard {
                dorc_plan::render::PasteHygieneHazard::LineTooLong { line, len } => {
                    (line, PasteHygieneHazardReason::LineTooLong { len })
                }
                dorc_plan::render::PasteHygieneHazard::LeadingTilde { line } => {
                    (line, PasteHygieneHazardReason::LeadingTilde)
                }
            };
            Diag::new_spanless_site(DiagCode::EmittedLineUnsafeForPaste(
                EmittedLineUnsafeForPaste { line, reason },
            ))
        })
        .collect()
}

/// The escalation-POLICY disclosure (`27C:render-authority-disclosure` — the consent-legibility
/// line). Names the escalation posture the dial + capability set, and the entry-capable wrappers
/// loaded (a wrapper authoring BOTH a peeling `__predict` and an `__enter` form). One `Note` the
/// edge reports (advisory), never a gate.
///
/// SCOPE (honest for the spike): this is the POLICY in effect, not a per-book-SITE "will enter"
/// tally — the book-side entry-composed probe emission (which would count sites per entered context)
/// is the deferred integration (`27K` §9 / this lane's report). The dial × capability × the loaded
/// entry forms are all real; what is missing is the per-site consumption in the probe pipeline.
///
/// WHOLE-UNIT, deliberately (`308:rul-escalation-policy-consumes-withdrawn-stays-whole-unit`): this
/// answers a LOAD-SET question with no site to ask it from, so unlike the wrapper lane's consuming
/// acts it is not frame-converted. It consumes the driver's WITHDRAWN vectors all the same, so a
/// contested wrapper family — whose sites now wall — stops narrating as entry-capable.
fn escalation_policy_diagnostics(
    checks: &[dorc_oracle::predict::PredictSet],
    wrapper_sets: &WrapperSets,
    dial: EscalationDial,
    capability: Capability,
) -> Vec<Diag> {
    use dorc_oracle::entry::detect_entry_form;

    // Entry-capable wrappers: a provider authoring an `__enter` form (whose predict also peels).
    let mut heads: BTreeMap<Symbol, String> = BTreeMap::new();
    for (ps, es) in checks.iter().zip(wrapper_sets.entries()) {
        let peels: BTreeSet<Symbol> = ps
            .providers()
            .filter(|p| ps.get(*p).is_some_and(detect_peel_present))
            .collect();
        for p in es.providers() {
            if peels.contains(&p)
                && let Some(form) = es.get(p).and_then(detect_entry_form)
            {
                heads.entry(p).or_insert_with(|| form.head.join(" "));
            }
        }
    }
    if heads.is_empty() {
        // no entry-capable wrapper loaded ⇒ no escalation is possible ⇒ nothing to disclose
        return Vec::new();
    }
    let head_list = heads.values().cloned().collect::<Vec<_>>().join(", ");
    vec![Diag::new_spanless_site(DiagCode::EscalationPolicy(
        EscalationPolicy {
            dial,
            capability,
            entry_forms: head_list,
        },
    ))]
}

/// Whether a predict body peels (a wrapper) — the `detect_peel`-present predicate, factored so the
/// entry-policy scan reuses it (`inv-referent-agnostic`: structural, never decodes the command).
fn detect_peel_present(p: &dorc_oracle::predict::Predict) -> bool {
    dorc_oracle::wrapper::detect_peel(p).is_some()
}

#[derive(Clone, Copy)]
enum BundleDiagnosticSite {
    Unlocated,
    EveryOccurrence(usize),
    Exact(crate::bundle::BundleFileId),
}

#[expect(
    clippy::too_many_arguments,
    reason = "bundle location and provenance are independent reporting inputs"
)]
fn report_bundle_diagnostic(
    sink: &mut dyn OutputSink,
    advisory: bool,
    stage: &str,
    snapshot: &StaticLoadSnapshot,
    projection: &crate::bundle::BundleProjection,
    load_acts: &crate::provenance::LoadActs,
    site: BundleDiagnosticSite,
    diag: &Diag,
) {
    if !advisory && diag.severity() != Severity::Error {
        return;
    }
    let (source_file, exact_file) = match site {
        BundleDiagnosticSite::Unlocated => {
            report(sink, stage, None, std::slice::from_ref(diag));
            return;
        }
        BundleDiagnosticSite::EveryOccurrence(source) => (source, None),
        BundleDiagnosticSite::Exact(file) => {
            let Some(source) = projection
                .file(file)
                .map(|entry| entry.copied().source().0 as usize)
            else {
                report(sink, stage, None, std::slice::from_ref(diag));
                return;
            };
            (source, Some(file))
        }
    };
    let source = snapshot
        .source_paths()
        .get(source_file)
        .zip(snapshot.source_srcs().get(source_file));
    let Some((filename, src)) = source else {
        report(sink, stage, None, std::slice::from_ref(diag));
        return;
    };
    let Some(span) = diag.primary.span() else {
        report(
            sink,
            stage,
            Some((filename, src)),
            std::slice::from_ref(diag),
        );
        return;
    };
    let matching: Vec<_> = projection
        .files()
        .iter()
        .filter(|file| file.copied().source().0 as usize == source_file)
        .filter(|file| exact_file.is_none_or(|wanted| file.id() == wanted))
        .collect();
    if matching.is_empty() {
        report(
            sink,
            stage,
            Some((filename, src)),
            std::slice::from_ref(diag),
        );
        return;
    }
    for file in matching {
        let Some((locator, head)) =
            load_acts.locator_for_bundle(snapshot, projection, file.id(), span)
        else {
            report(
                sink,
                stage,
                Some((filename, src)),
                std::slice::from_ref(diag),
            );
            continue;
        };
        let owned = crate::provenance::locator_frames(&locator, head, snapshot, projection);
        report_located(sink, stage, filename, src, diag, &owned);
    }
}

fn report_located(
    sink: &mut dyn OutputSink,
    stage: &str,
    filename: &str,
    src: &str,
    diag: &Diag,
    frames: &[crate::provenance::LocatorFrame],
) {
    let interner = Interner::default();
    let borrowed: Vec<_> = frames
        .iter()
        .map(|frame| dorc_aid::diag::DiagnosticFrame {
            filename: &frame.filename,
            source: &frame.source,
            span: frame.span,
        })
        .collect();
    let parts = dorc_aid::diag::render_staged_cli_parts_with_frames(
        stage,
        &sink.render_ctx(),
        diag,
        src,
        filename,
        &borrowed,
        &interner,
    );
    emit_diagnostic(sink, stage, diag, parts);
}

fn report_at(
    sink: &mut dyn OutputSink,
    advisory: bool,
    stage: &str,
    source: Option<(&str, &str)>,
    diags: &[Diag],
) {
    report(sink, stage, source, &advisory_filter(advisory, diags));
}

/// Report per-oracle-file diagnostics, each against its OWN `(path, src)` source, so a funcdef-keyed
/// diagnostic's caret frame resolves against the RIGHT oracle (`law-lineno-identity`: the file index
/// disambiguates the line-number space a bare span cannot). A file with no resolvable source falls to
/// the byte-offset fallback — never a wrong-file frame. Deterministic (`BTreeMap` key order).
fn report_by_oracle_file(
    sink: &mut dyn OutputSink,
    advisory: bool,
    stage: &str,
    oracle_paths: &[String],
    oracle_srcs: &[String],
    diags_by_file: &BTreeMap<usize, Vec<Diag>>,
) {
    for (idx, diags) in diags_by_file {
        let source = oracle_paths
            .get(*idx)
            .map(String::as_str)
            .zip(oracle_srcs.get(*idx).map(String::as_str));
        report_at(sink, advisory, stage, source, diags);
    }
}

/// The advisory severity-filter (rec-1 / tc-apply-receipt-floor), factored pure for
/// testing. `advisory` ⇒ pass every diagnostic through (the `plan`/round-trip render
/// surface); `!advisory` (the receipt-free `apply` off-ramp) ⇒ keep ONLY Error-severity,
/// dropping warnings + notes. Errors are NEVER dropped — the floor that keeps `apply`
/// honest while receipt-free. Returns owned clones (the call sites are cold — once per
/// pipeline stage — so the copy is irrelevant against the SSH-tunnel cost DESIGN floors on).
fn advisory_filter(advisory: bool, diags: &[Diag]) -> Vec<Diag> {
    if advisory {
        diags.to_vec()
    } else {
        diags
            .iter()
            .filter(|d| d.severity() == Severity::Error)
            .cloned()
            .collect()
    }
}

/// Print a stage's diagnostics to stderr (keeping stdout = probe + apply).
///
/// The TITLE line is `<stage>: <severity>[<code>]: <message-first-line>` — its shape is
/// load-bearing: the e2e gate-3 floor (20B §2) keys on `^<stage>: error[` (an Error fails a case
/// unless declared in `expected-diagnostics`; warnings stay free-form). Below it, ack-8 (round-24):
/// the rustc-style REGION FRAME — `--> file:line:col`, the source line in a gutter, and a `^^^`
/// caret ([`dorc_aid::diag::render_legacy_region`]) — replaces the old raw byte-offset `-->
/// <lo>:<hi>`, WHEN a `source` (`(filename, text)`) is threaded for this stage; a stage whose diags
/// span an ambiguous/combined source (or are spanless) keeps the byte-offset / no-region fallback.
/// The frame's `N |` gutter is the SOURCE line (rul24-lineno-identity: the number the user reads is
/// the number they type back as `:N`). None of the frame lines start with `<stage>: error[`, so
/// they stay inert to gate-3. Any folded ` = note:` continuations (a lowered `Diag`'s body) print
/// AFTER the region, so the caret frame lands rustc-style right beneath the title. I/O-edge only.
///
/// ack-5 color: the SEVERITY WORD is the severity/tier channel — red error / yellow warning / cyan
/// note — written through `anstream::stderr()`, an [`anstream::AutoStream`] that AUTO-STRIPS the
/// ANSI on a non-tty and honors `NO_COLOR` (+ enables Windows VT on a real console). Plain-when-
/// piped is load-bearing: the e2e harness captures stderr to a FILE ⇒ non-tty ⇒ the color vanishes,
/// so the gate-3/gate-7 needle-matching (and every golden) is byte-identical to the un-colored form.
fn report(sink: &mut dyn OutputSink, stage: &str, source: Option<(&str, &str)>, diags: &[Diag]) {
    for d in diags {
        let (filename, src) = source.unwrap_or(("", ""));
        let event = diagnostic_event(&sink.render_ctx(), stage, d, src, filename);
        sink.emit(event);
        sink.flush(OutputChannel::Stderr);
    }
}

fn emit_diagnostic(sink: &mut dyn OutputSink, stage: &str, diag: &Diag, parts: RenderParts) {
    sink.emit(OutputEvent::diagnostic(stage, diag.clone(), parts));
    sink.flush(OutputChannel::Stderr);
}

#[cfg(test)]
mod spine_record_tests {
    use std::collections::{BTreeMap, BTreeSet};

    use dorc_analysis::cfg::{CfgNodeId, CfgNodeKind};
    use dorc_analysis::effect::{InlineSite, SkipClass};
    use dorc_core::{EntityRef, FactKey, KindId, LeafId, OpaqueToken, SelectorId};

    fn cell(interner: &mut dorc_core::Interner, entity: &str) -> FactKey {
        FactKey::cell(
            KindId(interner.intern("package")),
            EntityRef::Operand(OpaqueToken(interner.intern(entity))),
            SelectorId(interner.intern("installed")),
        )
    }

    /// `30Mc` F3, both flat falsehoods at once: `invalidator` is documented "gens into reach" and
    /// was written from `kills` alone (false for every ordinary establish), and an `InlineCall`
    /// mapped its ordered member account to an EMPTY cell list. Driven through `record_new_arm`
    /// because the defect was the WIRING — which set the seat reads — and a pure per-record helper
    /// would have been just as wrong while passing (`anti-masking-tests`).
    #[test]
    fn a_classification_record_states_what_its_fields_promise() {
        let src = "apt-get install -y nginx\napt-get install -y curl\n";
        let ast = dorc_syntax::parse(src).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut interner = dorc_core::Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = dorc_analysis::funcenv::DefinitionTable::default();
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);

        let commands: Vec<CfgNodeId> = cfg
            .iter()
            .filter(|(_, node)| node.kind == CfgNodeKind::Command)
            .map(|(id, _)| id)
            .collect();
        let (standalone, aggregate) = (commands[0], commands[1]);
        let (nginx, curl) = (cell(&mut interner, "nginx"), cell(&mut interner, "curl"));
        let query = cell(&mut interner, "wombat");
        let classes = vec![
            (standalone, SkipClass::EstablishProbeAmbient(nginx)),
            (
                aggregate,
                SkipClass::InlineCall {
                    sites: vec![
                        InlineSite {
                            node: aggregate,
                            member: None,
                            class: SkipClass::EstablishProbeAmbient(curl),
                        },
                        // A QUERY member keys the call exactly as an establish member does; the
                        // first repair matched only the establish arms and dropped this one.
                        InlineSite {
                            node: aggregate,
                            member: None,
                            class: SkipClass::QueryResolvable {
                                fact: query,
                                valid: true,
                            },
                        },
                    ],
                },
            ),
        ];
        let spine_leaves: Vec<(dorc_core::AstId, LeafId)> = commands
            .iter()
            .enumerate()
            .map(|(n, node)| (cfg.node(*node).ast, LeafId(u32::try_from(n).unwrap_or(0))))
            .collect();

        let mut spine = dorc_plan::Spine::new();
        super::record_new_arm(
            &mut spine,
            &dorc_plan::ProbePlan::default(),
            &classes,
            &cfg,
            // An establish gens into reach and is NOT a kill: the exact population the retired
            // `kills`-only read reported as `false`.
            &BTreeSet::from([standalone, aggregate]),
            dorc_analysis::certify::CertifierTrip::default(),
            super::mint_load_decisions(&cfg, &dorc_core::ContestedFamilies::default(), &env),
            &BTreeMap::new(),
            &spine_leaves,
            true,
            dorc_core::influence::InfluenceAccount::authored_before_contact(),
        );

        let recorded: Vec<(&str, bool, usize)> = spine
            .classifications()
            .map(|record| {
                (
                    record.class(),
                    record.invalidator(),
                    record.cells().shown().len(),
                )
            })
            .collect();
        assert_eq!(
            recorded,
            [("EstablishProbeAmbient", true, 1), ("InlineCall", true, 2)],
            "both sites gen into reach, and the aggregate keys on EVERY member — establish and \
             query alike, not on nothing and not on a filtered subset"
        );
    }
}

#[cfg(test)]
mod why_lens_dedup_tests {
    //! `x2-fd1` (`22E`, `224` §10): the stage-4 render-dedup must key on `(cause, site)`, not the
    //! cause `ProvId` alone — else two inlined call-sites sharing one body-span cause collapse and
    //! the 2nd forced run's `why:` is wrongly suppressed. The arena hash-conses identical
    //! `(OriginKind, span)` origins (`core::prov` `hash_cons_shares_identical_origins`), so two
    //! `arena.leaf(TopCause, same_span)` calls reproduce the inlined-body cause collision.
    use dorc_aid::diag::{CmdsubOperandTop, CommandName, Diag, DiagCode, OperandPosition, SiteId};
    use dorc_core::{BytePos, LeafId, OriginKind, ProvArena, Span, TopCause};

    fn cmdsub_top(arena: &mut ProvArena, leaf: u32, body_span: Span) -> Diag {
        let cause = arena.leaf(OriginKind::TopCause, Some(body_span));
        Diag::new(
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: SiteId::leaf(LeafId(leaf)),
                position: OperandPosition::Operand(1),
                cause: Some(cause),
                top_cause: TopCause::UnmodeledExpansion,
                command: CommandName::Literal("apt-get".to_owned()),
            }),
            Span::new(BytePos(0), BytePos(20)),
        )
    }

    #[test]
    fn two_inlined_sites_sharing_one_cause_both_disclose() {
        // `apt_install "$(curl a)"; apt_install "$(curl b)"`: both calls inline ONE wrapper body ⇒
        // one shared cause ProvId, distinct call-site leaves. (cause, site) keeps BOTH `why:`s; the
        // old cause-alone key suppressed the 2nd (x2-fd1, disclosure-only over-suppression).
        let mut arena = ProvArena::new();
        let body = Span::new(BytePos(11), BytePos(20));
        let diags = [
            cmdsub_top(&mut arena, 3, body),
            cmdsub_top(&mut arena, 7, body),
        ];
        let lines = super::why_lens_reasons(
            &dorc_aid::RenderCtx::production(),
            &diags,
            &arena,
            "apt_install \"$(curl a)\"",
        );
        assert_eq!(
            lines.len(),
            2,
            "two inlined sites sharing one cause must BOTH disclose: {lines:?}"
        );
    }

    #[test]
    fn an_identical_cause_and_site_is_shown_once() {
        // The dedup still FIRES for a true duplicate (same cause AND same site) — the (cause, site)
        // key didn't neuter the stage-4 dedup into a no-op.
        let mut arena = ProvArena::new();
        let body = Span::new(BytePos(11), BytePos(20));
        let diags = [
            cmdsub_top(&mut arena, 3, body),
            cmdsub_top(&mut arena, 3, body),
        ];
        let lines = super::why_lens_reasons(
            &dorc_aid::RenderCtx::production(),
            &diags,
            &arena,
            "apt-get install \"$(date)\"",
        );
        assert_eq!(
            lines.len(),
            1,
            "an identical (cause, site) re-disclosure is shown once: {lines:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    struct SharedSink(Rc<RefCell<Vec<OutputAction>>>);

    impl OutputSink for SharedSink {
        fn emit(&mut self, event: OutputEvent) {
            self.0.borrow_mut().push(OutputAction::Event(event));
        }

        fn flush(&mut self, channel: OutputChannel) {
            self.0.borrow_mut().push(OutputAction::Flush(channel));
        }
    }

    struct LocalEdges {
        actions: Rc<RefCell<Vec<OutputAction>>>,
        observed_after_flush: Rc<Cell<bool>>,
        clock: RunClock,
    }

    impl EngineEdges for LocalEdges {
        fn materialize_shims(
            &mut self,
            _files: &BTreeMap<String, String>,
        ) -> Result<(), Box<Diag>> {
            Ok(())
        }

        fn observe(
            &mut self,
            request: &ObservationRequest<'_>,
            _render_probe: &dyn Fn(&dorc_plan::records::Framing) -> String,
        ) -> Result<Observation, Box<Diag>> {
            self.observed_after_flush.set(matches!(
                self.actions.borrow().last(),
                Some(OutputAction::Flush(OutputChannel::Stdout))
            ));
            Ok(Observation::Controller {
                framing: request.default_framing.clone(),
                evidence: dorc_plan::records::Admission::NoObservation,
                stderr: Vec::new(),
            })
        }

        fn clock(&mut self) -> &mut RunClock {
            &mut self.clock
        }

        fn source_match(&mut self, _book_name: &str) -> Option<SourceMatch> {
            None
        }

        fn publish_artifact(&mut self, _artifact: &ArtifactSet) -> Result<(), &'static str> {
            Ok(())
        }

        fn publish_whylog(&mut self, _bytes: &[u8]) -> Result<(), String> {
            Ok(())
        }

        fn durable_label(&self) -> &'static str {
            "<disabled>"
        }

        fn invocation_record(
            &mut self,
            request: InvocationRecordRequest<'_>,
        ) -> dorc_core::spine::SpineInvocation {
            dorc_core::spine::SpineInvocation::minted(
                dorc_core::spine::InvocationMode::WhylogReplay,
                Vec::new(),
                dorc_core::spine::SourceClaim {
                    path: request.snapshot.book_path().to_owned(),
                    digest: book_digest(request.snapshot.book_src()),
                },
                Vec::new(),
                dorc_core::spine::RunIdentity {
                    nonce: request.framing.nonce().0.clone(),
                    attempt: request.framing.attempt(),
                    host: request.framing.host().to_owned(),
                    started_at: request.started_at,
                },
                request.account,
            )
        }
    }

    #[test]
    fn ordered_actions_retain_writes_flushes_and_exact_text() {
        let mut events = OutputEvents::default();
        events.emit(OutputEvent::plain_text(
            OutputChannel::Stderr,
            "diagnostic\n",
        ));
        events.emit(OutputEvent::plain_text(OutputChannel::Stdout, "artifact\n"));
        events.flush(OutputChannel::Stdout);
        events.emit(OutputEvent::plain_text(OutputChannel::Stderr, "digest\n"));

        let actions = events.into_actions();
        assert!(matches!(
            actions[0],
            OutputAction::Event(ref event) if event.channel() == OutputChannel::Stderr && event.diagnostic_presentation().is_none()
        ));
        assert!(matches!(
            actions[1],
            OutputAction::Event(ref event) if event.channel() == OutputChannel::Stdout && event.diagnostic_presentation().is_none()
        ));
        assert!(matches!(
            actions[2],
            OutputAction::Flush(OutputChannel::Stdout)
        ));
        assert!(matches!(
            actions[3],
            OutputAction::Event(ref event) if event.channel() == OutputChannel::Stderr && event.diagnostic_presentation().is_none()
        ));
        assert_eq!(event_text(&actions[0]), "diagnostic\n");
        assert_eq!(event_text(&actions[1]), "artifact\n");
        assert_eq!(event_text(&actions[3]), "digest\n");
    }

    #[test]
    fn tagged_text_retains_non_empty_parts() {
        let mut parts = RenderParts::new();
        parts.push(dorc_aid::tagged::RenderPart::Arrangement {
            text: "tagged text".into(),
            slug: "test-output",
        });
        let expected = parts.text();
        let event = OutputEvent::plain_tagged(OutputChannel::Stderr, parts);

        assert!(!expected.is_empty());
        assert_eq!(event.text(), expected);
        assert!(event.tagged_parts().is_some());
        assert!(event.diagnostic_presentation().is_none());
    }

    #[test]
    fn diagnostic_destination_retains_stage_and_severity() {
        let event = OutputEvent::diagnostic(
            "parse",
            Diag::new_spanless_site(DiagCode::SyntaxUnsupported(
                dorc_aid::diag::SyntaxUnsupported {
                    reason: dorc_aid::diag::SyntaxUnsupportedReason::BackgroundAmp,
                },
            )),
            RenderParts::new(),
        );

        assert_eq!(event.channel(), OutputChannel::Stderr);
        assert!(event.diagnostic_presentation().is_some());
        assert!(event.diagnostic_payload().is_some());
    }

    #[test]
    fn source_has_no_process_control_surface() {
        let source = include_str!("engine.rs");
        let forbidden = [
            concat!("crate::", "Args"),
            concat!("&", "Args"),
            concat!("parse_", "args_from"),
            concat!("std::env::", "args"),
            concat!("std::env::", "var"),
            concat!("cl", "ap::"),
            concat!("bp", "af::"),
            concat!("raw_", "argv"),
        ];
        for needle in forbidden {
            assert!(
                !source.contains(needle),
                "forbidden engine surface: {needle}"
            );
        }
    }

    #[test]
    fn production_main_calls_the_shared_engine() {
        let source = include_str!("main.rs");
        assert!(source.contains("dorc_cli::engine::run("));
        assert!(!source.contains("\nfn run("));
        assert!(!source.contains("Observation::Fixture"));
    }

    #[test]
    fn round_trip_flushes_probe_before_observation_and_retains_generated_artifact() {
        let snapshot = StaticLoadSnapshot::over(
            dorc_core::loadpath::Cwd::default(),
            Vec::new(),
            Vec::new(),
            &crate::snapshot::LoadPositions::roots_only(),
            "book.sh",
            "hork\n",
        );
        let options = EngineOptions {
            mode: Mode::RoundTrip,
            analysis: AnalysisOptions {
                survival: SurvivalPolicy::HonestWalls,
                escalation: EscalationDial::VouchedOnly,
                capability: Capability::Root,
            },
            reporting: ReportingOptions {
                why_address: None,
                why_depth: WhyDepth::Curated,
                argv_readout: ArgvReadout::Hidden,
            },
            artifact: ArtifactOptions {
                form: None,
                stdout: StdoutPosture::NonInteractive,
                destination: ArtifactDestinationShape::Stdout,
            },
            durable: DurableOutput::Disabled,
        };
        let actions = Rc::new(RefCell::new(Vec::new()));
        let observed_after_flush = Rc::new(Cell::new(false));
        let mut sink = SharedSink(Rc::clone(&actions));
        let mut edges = LocalEdges {
            actions: Rc::clone(&actions),
            observed_after_flush: Rc::clone(&observed_after_flush),
            clock: RunClock::Absent,
        };

        let result = run(
            &EngineRequest {
                snapshot: &snapshot,
                options: &options,
                replay: None,
                acquisition_diagnostics: &[],
            },
            &mut edges,
            &mut sink,
        );
        assert!(result.is_ok(), "in-memory edge failure: {result:?}");
        let Some(result) = result.ok() else {
            return;
        };

        assert!(observed_after_flush.get());
        assert_eq!(result.status, EngineStatus::Complete);
        assert!(matches!(
            result.generated.as_slice(),
            [GeneratedOutput::Artifact(_)]
        ));
        let actions = actions.borrow();
        let probe = actions
            .iter()
            .position(|action| matches!(action, OutputAction::Event(event) if event.channel() == OutputChannel::Stdout))
            ;
        assert!(probe.is_some(), "the round-trip emitted no probe");
        let Some(probe) = probe else {
            return;
        };
        assert!(matches!(
            actions.get(probe + 1),
            Some(OutputAction::Flush(OutputChannel::Stdout))
        ));
        assert!(actions[probe + 2..].iter().any(
            |action| matches!(action, OutputAction::Event(event) if event.channel() == OutputChannel::Stdout)
        ));
    }

    fn event_text(action: &OutputAction) -> String {
        match action {
            OutputAction::Event(event) => event.text(),
            OutputAction::Flush(_) => String::new(),
        }
    }
}
