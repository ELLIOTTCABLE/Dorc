//! `dorc` — the thin spike CLI: the apply-2 round-trip over real files, as a
//! multi-mode plan/apply surface (ui-A, ru-25 / ru-20 ui-3).
//!
//! Reads a book + oracle files, runs the pure analyzer kernel, and emits one of the
//! engine's distinct user-facing behavioral modes. No executor — it *compiles* a probe
//! and an apply; it runs neither. The simulated host's answers arrive on stdin (in a
//! real deployment those come from running the probe on the host).
//!
//! ```text
//! usage: dorc [<mode>] --book=<book.sh> [-o <oracle.sh>]... [--debug-argv]
//!   modes:
//!     probe      emit the read-only probe artifact (phase 1) to stdout; reads no stdin
//!     plan       PREVIEW (ru-20 ui-3): the eliding-apply to stdout, PLUS the why-lens +
//!                diagnostics doubly-emitted to stderr (the cited-sections render surface)
//!     apply      the byte-floored, receipt-free shippable apply artifact to stdout;
//!                stderr carries ONLY error-severity diagnostics + the decision-digest
//!     <none>     the legacy round-trip: probe THEN apply on stdout, full disclosure on
//!                stderr — the shape the e2e harness drives (kept verbatim, do not break)
//!   stdin : probe results (plan/apply/round-trip), one per line —
//!           `site <leafid> effect=<holds|absent|cant-tell> rc=<n>`
//!   stdout: the selected mode's artifact(s); stderr: diagnostics / why-lens / digest
//!           + the plan-summary (every plan-building mode; the yardstick's metric):
//!           `dorc: plan-summary sites=<N> elide=<E> omit=<O> guard=<G> run=<R>`
//!           where sites == elide+omit+guard+run; elide = provably-skipped lines,
//!           omit = fold-dead branches, guard = 0 until the Stage-3 guard tier, run
//!           = the rest. Stable grammar (a parse target — plans/240 Stage-1 yardstick).
//! ```
//!
//! rec-1 TWO SURFACES (ru-12 + ru-20, spike/CLAUDE.md): the shipped `.sh` artifact on
//! stdout is byte-floored and receipt-free — `plan` and `apply` emit BYTE-IDENTICAL
//! apply bytes. The only difference is the RENDER surface (stderr): `plan` overlays the
//! per-line why-lens + advisory disclosure there; `apply` (the off-ramp) suppresses the
//! advisory plane, keeping only the error floor + digest. The why-lens is never woven
//! into the artifact bytes in any mode.
//!
//! Round-20 task-D1 (the WIRE — `inv-site-keyed-results`): the probe is a real,
//! self-reporting artifact; its results-records are keyed by command **site** (the
//! stable `LeafId`), not by fact. The simulated host's answers (the e2e
//! `probe-results.txt`, a stand-in for running the rendered probe remotely) are now
//! the site-keyed records the probe itself emits.
//!
//! I/O edge: `inv-determinism` exempts `cli`; the analyzer kernel it calls is pure.
//! Diagnostics go to stderr so stdout stays the artifact. The mode dispatch is a thin
//! driver over ONE pipeline call ([`analyze`]) — no kernel logic moves here (the
//! thin-driver mandate, crates/cli/CLAUDE.md).

#![forbid(unsafe_code)]
// The cli is the sanctioned I/O edge (workspace Cargo.toml: "I/O-edge crates may
// `#[expect]` these at the crate root, with reason"): stdout carries the
// probe-then-apply artifact, stderr carries diagnostics. The kernel it drives
// stays print-free. Not a seeded-ratchet expect — this one is permanent for the
// binary's edge.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "cli is the I/O edge: probe/apply to stdout, diagnostics to stderr; the kernel stays print-free"
)]

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::process::ExitCode;

mod artifact_store;
mod source_match;
mod transport_edge;

use dorc_aid::Severity;
use dorc_aid::diag::{Diag, DiagCode};
#[cfg(test)]
use dorc_aid::{CollapseKind, CollapseNarrative, SpeechAct};
use dorc_core::Interner;
#[cfg(test)]
use dorc_core::{ProvArena, Symbol};

// The invocation surface lives in the crate's INTERNAL lib target (`289:rul-worldless-route-
// honest-trigger`) so the loom harness can fire the real parser; this bin keeps every I/O edge.
#[cfg(test)]
use dorc_cli::engine::reach_arm_fn_name;
use dorc_cli::engine::{
    EngineEdges, EngineRequest, EngineStatus as RunOutcome, InvocationRecordRequest, Observation,
    ObservationRequest, OutputChannel, OutputEvent, OutputSink,
};
#[cfg(test)]
use dorc_cli::fixpoint::{FrozenModel, attribute_cascades, classify_round, settle_world};
#[cfg(test)]
use dorc_cli::kinds::{build_kind_reaches, build_kind_resolvers};
use dorc_cli::results::RunClock;
#[cfg(test)]
use dorc_cli::results::{ReportRecord, SiteResults, probe_origins};
#[cfg(test)]
use dorc_cli::survival::{
    WrapperSets, build_wrapped_analysis, expand_footprints_via_reaches, merge_derived_footprints,
};
use dorc_cli::world::definition_table;
#[cfg(test)]
use dorc_cli::world::{ship_predict_body, ship_verdict_body};
// The legacy headerless string parser below is `#[cfg(test)]`-gated law
// (`rul-fixture-identity-never-production`), so its tokenizers are imported on the same gate.
#[cfg(test)]
#[cfg(test)]
use dorc_cli::fixpoint::SettledFixpoint;
#[cfg(test)]
use dorc_cli::results::facts_from_sites;
#[cfg(test)]
use dorc_cli::results::{
    REPORT_RAW_CAP, RecordKey, ResolvOutcome, parse_leaf, parse_report_record, parse_site_record,
    sanitize_report_raw, split_key,
};
#[cfg(test)]
use dorc_cli::survival::own_wall_coord;
use dorc_cli::{Args, Invocation, LintArgs, LintFormat, Mode, humane_read_error, parse_args_from};
#[cfg(test)]
use dorc_core::{Observable, Verdict};
#[cfg(test)]
use dorc_core::{OutBytes, Predicted, Rc};
// The why REPORT composes across the same seam (`28L:rul-full-driver-this-arc`): this edge builds
// the world and prints the bytes, the lib turns that world into a stamped part stream.
#[cfg(test)]
use dorc_cli::why::{is_structurally_unprobeable, oracle_locus, unresolvable_diagnostics};

/// A usage/argument error, or an unreadable input file (the classic getopt convention).
const EXIT_USAGE: u8 = 2;
/// `dorc lint`: findings AT OR ABOVE the `--fail-on` threshold were reported (`27R` §5 exit
/// trichotomy). Distinct from clean (0) and from operational (below); shares linter convention.
const EXIT_LINT_FINDINGS: u8 = 1;
/// `dorc lint`: an OPERATIONAL error — the lint itself is compromised, distinct from both clean and
/// findings (`27R` §5, §8b): zero lintable files, an `--expect-files` mismatch, or a `--require-tools`
/// absence. NOT in the 10..=19 dorc-semantic family (a ⊤-reject book is a FINDING for lint, `27R` §5).
/// Numbered 3 (tc-lint-operational-exit-code — golangci-lint uses 3=Failure, shellcheck 3=bad-invoke;
/// the conservative lean, flagged for the human).
const EXIT_LINT_OPERATIONAL: u8 = 3;

/// Wall-clock ceiling on a probe session, in seconds (`260` s3-6).
///
/// Bounded by default because a probe is read-only by contract, so the worst a ceiling can cost
/// is a re-probe. An apply has no default ceiling for the mirror-image reason: killing one does
/// not fail it, it mints Unknown, so the caller must opt in with `--apply-timeout`.
const DEFAULT_PROBE_TIMEOUT_SECS: u64 = 120;

fn main() -> ExitCode {
    match parse_args() {
        Ok(Invocation::Help) => {
            print!("{}", dorc_cli::help_text(&render_ctx()));
            std::io::stdout().flush().ok();
            ExitCode::SUCCESS
        }
        Ok(Invocation::Version) => {
            println!("dorc {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(Invocation::Strip(path)) => match strip_command(&path) {
            Ok(()) => ExitCode::SUCCESS,
            Err(diag) => {
                report_invocation_error(&diag);
                ExitCode::from(EXIT_USAGE)
            }
        },
        Ok(Invocation::Lint(args)) => lint_command(&args),
        Ok(Invocation::Analyze(args)) => {
            let mut sink = ProductionOutputSink;
            let result = run_analysis(&args, &mut sink);
            match result {
                Ok(status) => ExitCode::from(status.exit_code()),
                Err(diag) => {
                    report_invocation_error(&diag);
                    ExitCode::from(EXIT_USAGE)
                }
            }
        }
        Err(diag) => {
            report_invocation_error(&diag);
            ExitCode::from(EXIT_USAGE)
        }
    }
}

#[expect(
    clippy::result_large_err,
    reason = "the binary print seat consumes the full diagnostic"
)]
fn run_analysis(args: &Args, sink: &mut dyn OutputSink) -> Result<RunOutcome, Diag> {
    if args.mode == Mode::Apply
        && let Some(host) = args.host.as_deref()
    {
        return ship_consented_apply(sink, args, host);
    }
    if args.reads_the_receipt() {
        let edge = production_receipt_edge(args);
        let label = edge
            .as_ref()
            .map_or(dorc_cli::engine::NO_STATE_ROOT, |edge| edge.state_base());
        let answer = match &edge {
            Ok(edge) => read_rooted_receipt(edge, args),
            Err(refusal) => dorc_cli::recorded::StoreAnswer::Unreadable(refusal.token().to_owned()),
        };
        return Ok(dorc_cli::engine::report_recorded_store(
            answer,
            args.why_register(),
            label,
            sink,
        ));
    }

    let stdout = stdout_posture();
    // Publication is gated on the admin's REFUSAL, never on whether they named a store: the store
    // has a standard per-user default and `28F:rul-w3-default-on-aim-high` makes a receipt the
    // thing you get without asking. Gating on a named directory would make default-on mean
    // default-off for every invocation that did not spell one.
    let options = dorc_cli::engine_options_from_args(
        args,
        stdout,
        args.artifact_dir.is_some(),
        !args.no_receipt,
    );
    let cwd = invocation_cwd();
    let ready = acquire_engine_request(args, &cwd)?;
    let mut edges = ProductionEdges {
        args,
        clock: clock_for_invocation(),
        receipt: production_receipt_edge(args),
    };
    dorc_cli::engine::run(
        &EngineRequest {
            snapshot: &ready.snapshot,
            options: &options,
            acquisition_diagnostics: &ready.acquisition_diagnostics,
        },
        &mut edges,
        sink,
    )
    .map_err(|error| *error)
    .map(|result| result.status)
}

struct AcquiredReady {
    snapshot: dorc_cli::snapshot::StaticLoadSnapshot,
    acquisition_diagnostics: Vec<Diag>,
}

#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn acquire_engine_request(
    args: &Args,
    cwd: &dorc_core::loadpath::Cwd,
) -> Result<Box<AcquiredReady>, Diag> {
    let oracle_paths = resolve_pre_sources(&args.pre_sources, &args.oracle_dirs)?;
    let oracle_srcs: Vec<String> = oracle_paths
        .iter()
        .map(|path| read_input("pre-source", path))
        .collect::<Result<_, _>>()?;
    let (oracle_paths, oracle_srcs, load_dependencies) =
        read_sourced_oracles(cwd, oracle_paths, oracle_srcs);
    let book_path = args.book.as_deref();
    let book_src = match book_path {
        Some(path) => read_input("book", path)?,
        None => String::new(),
    };
    let book_name = book_path.unwrap_or("book.sh");
    let acquisition_diagnostics = unloaded_sibling_oracle_diagnostics(book_path, &oracle_paths);
    let acquired = read_book_sourced(
        cwd,
        book_name,
        &book_src,
        oracle_paths,
        oracle_srcs,
        &load_dependencies,
    );
    let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
        cwd.clone(),
        acquired.paths,
        acquired.srcs,
        &dorc_cli::snapshot::LoadPositions::book_sourced(acquired.reached)
            .with_dependencies(load_dependencies),
        book_name,
        &book_src,
    );
    Ok(Box::new(AcquiredReady {
        snapshot,
        acquisition_diagnostics,
    }))
}

struct ProductionEdges<'a> {
    args: &'a Args,
    clock: RunClock,
    /// The production durable edge, or the refusal that stands in its place.
    ///
    /// Resolved ONCE at the process boundary, before the engine runs, so root resolution cannot
    /// happen twice with different answers and cannot happen inside the pipeline at all.
    receipt: Result<dorc_cli::durable::LocalReceiptEdgeV1, dorc_cli::durable::EdgeRefusal>,
}

impl EngineEdges for ProductionEdges<'_> {
    fn materialize_shims(&mut self, files: &BTreeMap<String, String>) -> Result<(), Box<Diag>> {
        match self.args.shim_dir.as_deref() {
            Some(dir) => materialize_shim_dir(dir, files).map_err(Box::new),
            None => Ok(()),
        }
    }

    fn observe(
        &mut self,
        request: &ObservationRequest<'_>,
        render_probe: &dyn Fn(&dorc_plan::records::Framing) -> String,
    ) -> Result<Observation, Box<Diag>> {
        let Some(raw_host) = self.args.host.as_deref() else {
            let evidence = if let Some("-") = self.args.results.as_deref() {
                dorc_plan::records::read_host_evidence(
                    std::io::stdin(),
                    dorc_plan::records::HostEvidenceLimits::spike_default(),
                )
            } else if let Some(path) = &self.args.results {
                let file = std::fs::File::open(path)
                    .map_err(|error| Box::new(humane_read_error("results", path, &error)))?;
                dorc_plan::records::read_host_evidence(
                    file,
                    dorc_plan::records::HostEvidenceLimits::spike_default(),
                )
            } else {
                dorc_plan::records::Admission::NoObservation
            };
            return Ok(Observation::Controller {
                framing: request.default_framing.clone(),
                evidence,
                stderr: Vec::new(),
            });
        };

        let host = dorc_transport::HostId::new(raw_host)
            .map_err(|_| Box::new(transport_edge::host_rejected(raw_host)))?;
        if let Some(line) = transport_edge::first_carriage_return(request.sources.book.as_bytes()) {
            return Err(Box::new(transport_edge::crlf_refusal(
                request.sources.book_name,
                line,
            )));
        }
        let nonce = transport_edge::mint_nonce();
        let mut driver = transport_edge::driver_for_invocation(
            self.args.connect_timeout,
            self.args.accept_new,
            self.args.ssh_config.as_deref(),
        );
        let timeout = Some(std::time::Duration::from_secs(
            self.args
                .probe_timeout
                .unwrap_or(DEFAULT_PROBE_TIMEOUT_SECS),
        ));
        Ok(
            match transport_edge::ship_probe(
                driver.as_mut(),
                &host,
                &nonce,
                &dorc_plan::invocation::book_digest(request.sources.book),
                timeout,
                render_probe,
            ) {
                transport_edge::ProbeShipment::Captured {
                    stdout,
                    framing,
                    stderr,
                } => Observation::Controller {
                    framing,
                    evidence: dorc_plan::records::read_host_evidence(
                        std::io::Cursor::new(stdout),
                        dorc_plan::records::HostEvidenceLimits::spike_default(),
                    ),
                    stderr: transport_edge::encoded_host_lines(&stderr),
                },
                transport_edge::ProbeShipment::Lost {
                    diagnosis,
                    attempts,
                } => Observation::Terminal {
                    status: RunOutcome::SessionLost,
                    diagnostic: transport_edge::session_lost(raw_host, attempts, &diagnosis),
                },
                transport_edge::ProbeShipment::NotAttempted(why) => Observation::Terminal {
                    status: RunOutcome::HostNotReached,
                    diagnostic: transport_edge::not_attempted(raw_host, &why),
                },
            },
        )
    }

    fn clock(&mut self) -> &mut RunClock {
        &mut self.clock
    }

    fn source_match(&mut self, book_name: &str) -> Option<dorc_cli::SourceMatch> {
        source_match::resolve(
            &source_match::GitRepository,
            std::path::Path::new(book_name),
        )
    }

    fn publish_artifact(
        &mut self,
        artifact: &dorc_cli::artifact::ArtifactSet,
    ) -> Result<(), &'static str> {
        let Some(dir) = self.args.artifact_dir.as_deref() else {
            return Ok(());
        };
        publish_artifact(dir, artifact).map_err(artifact_store::PublishRefusal::reason)
    }

    fn publish_receipt(
        &mut self,
        request: &dorc_cli::engine::ReceiptPublicationRequest<'_>,
    ) -> Result<Option<dorc_cli::receipt_edge::PlacedDocument>, String> {
        let edge = match &self.receipt {
            Ok(edge) => edge,
            Err(refusal) => return Err(refusal.token().to_owned()),
        };
        let mut io = dorc_cli::durable::NativeIo::new();
        let mut generator =
            dorc_cli::durable::OsKeysetGenerator::over(dorc_cli::durable::OsKeyEntropy);
        let open = edge
            .open_for_write(&mut io, &mut generator)
            .map_err(|refusal| refusal.token().to_owned())?;
        let mut ids =
            dorc_cli::receipt_edge::OsReceiptIdSource::over(dorc_cli::receipt_edge::OsEntropy);
        let mut order = dorc_cli::receipt_edge::RunClockOrder::of(&mut self.clock);
        let signer = open.keys().signer();
        let sealer = open.keys().encryption().sealer();
        let mut placement = open.placement(&mut io);
        dorc_cli::receipt_edge::publish_rich_plan_receipt(
            request,
            dorc_cli::receipt_edge::ReceiptCapabilities::of(
                &mut ids,
                &mut order,
                signer,
                &mut placement,
            ),
            &sealer,
        )
        .map(Some)
        .map_err(|refusal| refusal.token().to_owned())
    }

    fn receipt_label(&self) -> &str {
        self.receipt.as_ref().map_or(
            dorc_cli::engine::NO_STATE_ROOT,
            dorc_cli::durable::LocalReceiptEdgeV1::state_base,
        )
    }

    fn invocation_record(
        &mut self,
        request: InvocationRecordRequest<'_>,
    ) -> dorc_core::spine::SpineInvocation {
        // `argv` is a QUERY, so it is read here and handed over as a value.
        dorc_cli::receipt_edge::invocation_record(
            std::env::args().collect(),
            request.framing,
            request.snapshot,
            request.started_at,
            request.account,
        )
    }
}

/// Read ONE rooted receipt question, so the shared seat can render it.
///
/// Every act here needs a filesystem or a key, which is why it is the only part of this route that
/// stays at the process edge (`io-at-edges-only`): the RECONSTRUCTION and the render are
/// `engine::report_recorded_store`'s, so the binary and the loom driver share one.
///
/// Read-only in every respect that matters: the keyset is opened through the entry point that
/// cannot generate, the store through the one that cannot create, and no host is contacted. A
/// missing keyset, a missing store, or a damaged document is a REPORT state — asking why must
/// never mint an identity that cannot open the receipt being asked about.
fn read_rooted_receipt(
    edge: &dorc_cli::durable::LocalReceiptEdgeV1,
    args: &Args,
) -> dorc_cli::recorded::StoreAnswer {
    let mut io = dorc_cli::durable::NativeIo::new();
    let address = named_address(args.why_address.as_deref());
    match args.receipt_root() {
        dorc_cli::engine::ReceiptRoot::File(path) => root_from_file(edge, &mut io, path, address),
        selection => root_from_store(edge, &mut io, selection, address),
    }
}

/// One decoded document, with the identity and order the store filed it under.
struct HeldDocument {
    receipt_id: String,
    order: dorc_receipt::order::ReceiptOrderToken,
    receipt: HeldReceipt,
}

/// A decoded document, by species. Each arm is a locally-authenticated read and nothing else can
/// mint one.
enum HeldReceipt {
    Plan(dorc_cli::durable::LocallyAuthenticatedRead<dorc_receipt::model::PlanReceipt>),
    Intent(dorc_cli::durable::LocallyAuthenticatedRead<dorc_receipt::model::ApplyIntent>),
    Outcome(dorc_cli::durable::LocallyAuthenticatedRead<dorc_receipt::model::ApplyOutcome>),
}

impl HeldReceipt {
    /// The identity the DOCUMENT carries, rather than the one its filename claims.
    fn receipt_id(&self) -> String {
        match self {
            Self::Plan(document) => document.document().receipt_id_hex(),
            Self::Intent(document) => document.document().receipt_id_hex(),
            Self::Outcome(document) => document.document().receipt_id_hex(),
        }
    }
}

/// Root the question at a document the store holds.
fn root_from_store(
    edge: &dorc_cli::durable::LocalReceiptEdgeV1,
    io: &mut dorc_cli::durable::NativeIo,
    selection: dorc_cli::engine::ReceiptRoot<'_>,
    address: dorc_cli::recorded::AddressAsk,
) -> dorc_cli::recorded::StoreAnswer {
    use dorc_cli::recorded::StoreAnswer;
    let open = match edge.open_for_read(io) {
        Ok(open) => open,
        Err(refusal) => return StoreAnswer::Unreadable(refusal.token().to_owned()),
    };
    let mut graph = dorc_receipt::graph::ReceiptGraph::new();
    let Some((held, cohort)) = walk_store(&open, io, &mut graph) else {
        return StoreAnswer::Unreadable("walk-failed".to_owned());
    };
    let terminal = dorc_cli::recorded::collapse_predecessors(cohort, &graph.edges());
    if matches!(selection, dorc_cli::engine::ReceiptRoot::Last) && terminal.len() > 1 {
        return StoreAnswer::Ambiguous(terminal.len());
    }
    let Some(chosen) = held
        .into_iter()
        .find(|document| selection.takes(&document.receipt_id, &terminal))
    else {
        return StoreAnswer::Unreadable("no-receipt".to_owned());
    };
    rooted_reading(&graph, chosen, address)
}

/// Root the question at an explicit file OUTSIDE any store (`30R:receipt-rooted-attention-and-cli`).
///
/// It never publishes and never authorizes, and it does not need a store: a keyset is what a read
/// requires, and the store — when one opens — is only the bounded place this root's typed siblings
/// are resolved in, which is exactly the orthogonality `--receipts` has beside it.
///
/// The document's ORDER is the one fact a loose file cannot state for itself: the receipt carries it
/// and the read surface has no exit for it, so the store's own filename grammar answers. A file
/// renamed out of that grammar is refused rather than dated UNDATED, which would be a false claim
/// about the document (`30Ve:fnd-file-root-order-comes-from-the-name`).
fn root_from_file(
    edge: &dorc_cli::durable::LocalReceiptEdgeV1,
    io: &mut dorc_cli::durable::NativeIo,
    path: &str,
    address: dorc_cli::recorded::AddressAsk,
) -> dorc_cli::recorded::StoreAnswer {
    use dorc_cli::recorded::StoreAnswer;
    let reader = match edge.open_documents_for_read(io) {
        Ok(reader) => reader,
        Err(refusal) => return StoreAnswer::Unreadable(refusal.token().to_owned()),
    };
    let Some(order) = dorc_cli::durable::order_of_receipt_file(path) else {
        return StoreAnswer::Unreadable("receipt-file-unnamed".to_owned());
    };
    let Some(bytes) = read_receipt_file(path) else {
        return StoreAnswer::Unreadable("receipt-file-unreadable".to_owned());
    };
    let mut graph = dorc_receipt::graph::ReceiptGraph::new();
    if let Ok(open) = edge.open_for_read(io) {
        walk_store(&open, io, &mut graph);
    }
    let Some(receipt) = read_any_species(&reader, &mut graph, bytes) else {
        return StoreAnswer::Unreadable("receipt-file-unreadable".to_owned());
    };
    let receipt_id = receipt.receipt_id();
    rooted_reading(
        &graph,
        HeldDocument {
            receipt_id,
            order,
            receipt,
        },
        address,
    )
}

/// Walk the store once: every recognized document into the graph, and the greatest-order cohort.
///
/// The graph is built over the WHOLE store under its aggregate budget, whatever the selection above
/// it takes: a correlation is a fact about the record set, and answering "which intent does this
/// outcome answer" from a one-document read would be answering a different question. That is
/// bounded DISCOVERY of typed reverse edges, never a user-visible union of histories.
fn walk_store(
    open: &dorc_cli::durable::ReadEdge,
    io: &mut dorc_cli::durable::NativeIo,
    graph: &mut dorc_receipt::graph::ReceiptGraph,
) -> Option<(Vec<HeldDocument>, Vec<String>)> {
    let store = open.store();
    let entries = store.enumerate(io).ok()?;
    let mut budget = store.graph_budget();
    let mut held = Vec::new();
    for entry in entries.recognized() {
        let Ok(bytes) = store.read_into_budget(io, entry, &mut budget) else {
            continue;
        };
        if let Some(receipt) =
            read_recognized(open, graph, entry.species(), bytes.into_bytes().into_vec())
        {
            held.push(HeldDocument {
                receipt_id: entry.name().receipt_id().to_owned(),
                order: entry.name().order(),
                receipt,
            });
        }
    }
    let cohort = entries
        .maximum_order_cohort()
        .map(|cohort| {
            cohort
                .members()
                .iter()
                .map(|entry| entry.name().receipt_id().to_owned())
                .collect()
        })
        .unwrap_or_default();
    Some((held, cohort))
}

/// Read one recognized store entry into the graph.
///
/// The species comes from the FILENAME here, and the read is species-typed accordingly; a document
/// whose own header disagrees with its name fails to parse under the species asked for, which is
/// the disagreement staying a finding rather than being smoothed over.
///
/// The EXACT bytes go in beside each document: an identity is minted per document, so two of them
/// claiming one identity is a finding only if the graph can compare what they were read from.
/// Handing it an empty slice made every pair compare equal, which silenced
/// `GraphFinding::IdentityCollision` for every real store walk.
fn read_recognized(
    open: &dorc_cli::durable::ReadEdge,
    graph: &mut dorc_receipt::graph::ReceiptGraph,
    species: dorc_cli::durable::NamedSpecies,
    bytes: Vec<u8>,
) -> Option<HeldReceipt> {
    use dorc_cli::durable::NamedSpecies;
    let image = bytes.clone();
    // no self-asserted arm: this edge holds one keyset, so another provider is a read that did
    // not happen
    match species {
        NamedSpecies::Plan => open.read_plan(bytes).ok().map(|document| {
            dorc_cli::recorded::ingest_plan(graph, &document, &image);
            HeldReceipt::Plan(document)
        }),
        NamedSpecies::ApplyIntent => open.read_intent(bytes).ok().map(|document| {
            dorc_cli::recorded::ingest_intent(graph, &document, &image);
            HeldReceipt::Intent(document)
        }),
        NamedSpecies::ApplyOutcome => open.read_outcome(bytes).ok().map(|document| {
            dorc_cli::recorded::ingest_outcome(graph, &document, &image);
            HeldReceipt::Outcome(document)
        }),
    }
}

/// Read one document whose species is the DOCUMENT'S OWN to state.
///
/// A file named by an admin may carry any name at all, so the header decides: a read under the
/// wrong species fails at the skeleton's own `species` line, which makes trying each in turn a way
/// of ASKING the document rather than of guessing.
fn read_any_species(
    reader: &dorc_cli::durable::DocumentReader,
    graph: &mut dorc_receipt::graph::ReceiptGraph,
    bytes: Vec<u8>,
) -> Option<HeldReceipt> {
    let image = bytes.clone();
    if let Ok(document) = reader.read_plan(bytes.clone()) {
        dorc_cli::recorded::ingest_plan(graph, &document, &image);
        return Some(HeldReceipt::Plan(document));
    }
    if let Ok(document) = reader.read_intent(bytes.clone()) {
        dorc_cli::recorded::ingest_intent(graph, &document, &image);
        return Some(HeldReceipt::Intent(document));
    }
    if let Ok(document) = reader.read_outcome(bytes) {
        dorc_cli::recorded::ingest_outcome(graph, &document, &image);
        return Some(HeldReceipt::Outcome(document));
    }
    None
}

/// Bind one selected root into the reading the render seat consumes.
fn rooted_reading(
    graph: &dorc_receipt::graph::ReceiptGraph,
    chosen: HeldDocument,
    address: dorc_cli::recorded::AddressAsk,
) -> dorc_cli::recorded::StoreAnswer {
    use dorc_cli::recorded::{ReadRoot, RootedReading, StoreAnswer};
    use dorc_receipt::report::{AuthenticationState, DetailState, ProjectionState};
    let Some(document) = document_identity(&chosen.receipt, &chosen.receipt_id) else {
        return StoreAnswer::Unreadable("receipt-identity-unreadable".to_owned());
    };
    let siblings = dorc_cli::recorded::siblings_of(graph, &document);
    let closure = graph.closure_from(&document);
    let correlations = dorc_cli::recorded::correlations_of(graph, closure.documents());
    // Both established by the READ: the local-authentication envelope, and a region that validated.
    let authentication = AuthenticationState::Trusted;
    let detail = DetailState::Available;
    let order_spelled = (chosen.order != dorc_receipt::order::ReceiptOrderToken::UNDATED)
        .then(|| chosen.order.spelled());
    let root = match chosen.receipt {
        HeldReceipt::Plan(receipt) => {
            let Ok(model) = receipt.document().model() else {
                return StoreAnswer::Unreadable("receipt-model-unavailable".to_owned());
            };
            ReadRoot::Plan(Box::new(dorc_cli::recorded_facts::SelectedRoot {
                receipt,
                model,
                closure,
                order: chosen.order,
                authentication,
                detail,
            }))
        }
        HeldReceipt::Intent(receipt) => ReadRoot::OtherSpecies(dorc_why::recorded::NonPlanRoot {
            document,
            authentication,
            projection: ProjectionState::Rich,
            detail,
            order: order_spelled,
            correlations,
            siblings: siblings.clone(),
            intent: dorc_cli::recorded::shallow_intent(receipt.document()),
            outcome: None,
        }),
        HeldReceipt::Outcome(receipt) => ReadRoot::OtherSpecies(dorc_why::recorded::NonPlanRoot {
            document,
            authentication,
            projection: ProjectionState::Rich,
            detail,
            order: order_spelled,
            correlations,
            siblings: siblings.clone(),
            intent: None,
            outcome: dorc_cli::recorded::shallow_outcome(receipt.document()),
        }),
    };
    StoreAnswer::Rooted(Box::new(RootedReading::of(root, siblings, address)))
}

/// The typed identity one held document carries.
fn document_identity(
    receipt: &HeldReceipt,
    receipt_id: &str,
) -> Option<dorc_receipt::report::RecordedDocumentId> {
    use dorc_receipt::report::RecordedDocumentId;
    Some(match receipt {
        HeldReceipt::Plan(_) => {
            RecordedDocumentId::Plan(dorc_receipt::ids::PlanReceiptId::of_hex(receipt_id)?)
        }
        HeldReceipt::Intent(_) => {
            RecordedDocumentId::ApplyIntent(dorc_receipt::ids::ApplyIntentId::of_hex(receipt_id)?)
        }
        HeldReceipt::Outcome(_) => {
            RecordedDocumentId::ApplyOutcome(dorc_receipt::ids::ApplyOutcomeId::of_hex(receipt_id)?)
        }
    })
}

/// The address the question named, as far as a filesystem can take it.
///
/// `<file>:<line>` is the whole grammar this surface accepts (`30V` §2
/// rul-line-addresses-are-namespaced): the recorded model names an ORDINAL and the only bridge to
/// it is the exact bytes of the file the user named, so a content query or a bare line number has
/// nothing here to resolve against and says so rather than answering about something else.
fn named_address(spec: Option<&str>) -> dorc_cli::recorded::AddressAsk {
    use dorc_cli::recorded::AddressAsk;
    use dorc_why::UnplaceableAddress;
    let Some(spec) = spec else {
        return AddressAsk::Unasked;
    };
    // Split at the LAST colon, so a Windows drive letter stays part of the path.
    let Some((path, line)) = spec.rsplit_once(':') else {
        return AddressAsk::Unplaceable(UnplaceableAddress::NotAFileAndLine);
    };
    let Ok(line) = line.parse::<u32>() else {
        return AddressAsk::Unplaceable(UnplaceableAddress::NotAFileAndLine);
    };
    if line == 0 || path.is_empty() {
        return AddressAsk::Unplaceable(UnplaceableAddress::NotAFileAndLine);
    }
    read_current_source(path).map_or(
        AddressAsk::Unplaceable(UnplaceableAddress::CurrentSourceUnreadable),
        |bytes| {
            AddressAsk::Read(dorc_cli::source_comparison::NamedFile {
                path: path.to_owned(),
                line,
                bytes,
            })
        },
    )
}

/// One receipt file, bounded before anything is parsed from it.
fn read_receipt_file(path: &str) -> Option<Vec<u8>> {
    read_bounded(
        path,
        dorc_receipt::limits::ReceiptLimits::V1.outer_bytes.get(),
    )
}

/// One current source the question named, bounded by what a BOOK is rather than by what a
/// filesystem will hand over.
fn read_current_source(path: &str) -> Option<Vec<u8>> {
    read_bounded(
        path,
        dorc_receipt::limits::ReceiptLimits::V1
            .source_content_bytes
            .get(),
    )
}

/// Read at most `cap` bytes, refusing anything longer rather than truncating it.
///
/// Truncation is refused rather than reported because both consumers compare EXACT bytes — a
/// signature over a document, a digest over a source — and a short read would compare a prefix
/// against a whole and call the difference drift.
fn read_bounded(path: &str, cap: u64) -> Option<Vec<u8>> {
    use std::io::Read as _;
    let file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.take(cap.saturating_add(1))
        .read_to_end(&mut bytes)
        .ok()?;
    (u64::try_from(bytes.len()).unwrap_or(u64::MAX) <= cap).then_some(bytes)
}

/// The process's own environment, as the root-resolution rule's one query.
///
/// An empty value reads as absent: a variable set to nothing names no directory, and treating it
/// as one would land the durable at whatever the empty string resolves to.
struct ProcessEnvironment;

impl dorc_cli::durable::RootEnvironment for ProcessEnvironment {
    fn var(&self, name: &str) -> Option<String> {
        std::env::var(name).ok().filter(|value| !value.is_empty())
    }
}

/// This invocation's production durable edge, resolved once at the process boundary.
///
/// The refusal is CARRIED rather than reported here: whether a run without a per-user root is a
/// problem depends on what the run was going to do with one, and the seat that knows that is the
/// seat that later asks for a keyset.
///
/// `--receipts` is resolved to an absolute controller path HERE and nowhere else
/// (`30Rd:controller-root-resolution`): this is the one seat that may consult the process's own
/// working directory, so a store root settled anywhere downstream could move with a `cd`. Host
/// bytes, source text, receipt contents and TTY state reach none of it. The KEY root is untouched
/// by construction — `RootInputs` offers no way for a store root to reach the configuration role.
fn production_receipt_edge(
    args: &Args,
) -> Result<dorc_cli::durable::LocalReceiptEdgeV1, dorc_cli::durable::EdgeRefusal> {
    let roots =
        dorc_cli::durable::standard_roots(dorc_cli::durable::host_platform(), &ProcessEnvironment)
            .map_err(dorc_cli::durable::EdgeRefusal::Roots)?;
    let roots = match args.receipts.as_deref() {
        Some(folder) => roots
            .with_store_root(&absolute_controller_path(folder))
            .map_err(dorc_cli::durable::EdgeRefusal::Roots)?,
        None => roots,
    };
    Ok(dorc_cli::durable::LocalReceiptEdgeV1::of(roots))
}

/// One admin-typed folder, as an absolute controller path.
///
/// A relative spelling is joined to the process's working directory ONCE, here. Lexical rather
/// than canonicalizing: `canonicalize` requires the directory to already exist, and the
/// create-capable path is entitled to make it — resolving through the filesystem would make a
/// first run refuse the folder it was about to create.
fn absolute_controller_path(folder: &str) -> String {
    let path = std::path::Path::new(folder);
    if path.is_absolute() {
        return folder.to_owned();
    }
    std::env::current_dir().map_or_else(
        |_| folder.to_owned(),
        |cwd| cwd.join(path).to_string_lossy().into_owned(),
    )
}

struct ProductionOutputSink;

impl OutputSink for ProductionOutputSink {
    fn emit(&mut self, event: OutputEvent) {
        if let Some((stage, severity)) = event.diagnostic_presentation() {
            use std::io::Write as _;
            let rendered = event.text();
            let (word, style) = severity_style(severity);
            let prefix = format!("{stage}: {word}");
            let mut output = anstream::stderr();
            let _ = match rendered.strip_prefix(&prefix) {
                Some(rest) => write!(output, "{stage}: {style}{word}{style:#}{rest}"),
                None => write!(output, "{rendered}"),
            };
            return;
        }
        match event.channel() {
            OutputChannel::Stdout => print!("{}", event.text()),
            OutputChannel::Stderr => eprint!("{}", event.text()),
        }
    }

    fn flush(&mut self, channel: OutputChannel) {
        match channel {
            OutputChannel::Stdout => std::io::stdout().flush().ok(),
            OutputChannel::Stderr => std::io::stderr().flush().ok(),
        };
    }
}

/// Minimal hand-rolled parsing (no `clap` dep yet): resolve the whole invocation. `--help`
/// and `--version` win unconditionally (a pre-scan — ack-1 help-is-success, so a help request
/// beats a malformed flag) and return the stdout-and-exit-0 variants. Otherwise: an OPTIONAL
/// leading mode token (`bundle`/`probe`/`plan`/`apply`; absent ⇒ [`Mode::RoundTrip`]), then `--book=PATH` /
/// `--book PATH`, `-o PATH` / `-oPATH` / `--oracle PATH` (repeatable), `--debug-argv`,
/// `--risk-faultless-skips`. The mode is positional-first ONLY (a bare word after flags is still an
/// error) so the legacy `dorc --book=… < results` invocation parses unchanged.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn parse_args() -> Result<Invocation, Diag> {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    parse_args_from(raw)
}

/// The lint OPERATIONAL print seat (`27R` §5 exit trichotomy): the lint itself is compromised, so
/// the message rides the `dorc: lint: ` chrome and the caller returns `EXIT_LINT_OPERATIONAL`.
fn report_lint_operational(diag: &Diag) {
    eprintln!(
        "dorc: lint: {}",
        dorc_aid::diag::render_body(diag, &Interner::default())
    );
}

/// The tables and box every render seat in THIS file reads through
/// (`28L:rul-render-context-struct`).
///
/// Always the compiled-in ones, and that is a fact about this file rather than a shortcut: `main.rs`
/// is the I/O edge, and the loom drives `lib.rs` (`lib-target-is-a-loom-seam`), never anything here
/// — so no seat in this file can ever be handed an editable mirror. Named ONCE all the same, so the
/// day one can be, there is a single place that changes.
fn render_ctx() -> dorc_aid::RenderCtx<'static> {
    dorc_aid::RenderCtx::production()
}

/// One registry-sourced chrome line, its computed values interleaved between the entry's words
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`). These stderr lines have a registry
/// HOME but not yet an editable face: no case drives them, so their words are edited in the lock
/// until a page case exists for them.
#[cfg(test)]
fn chrome_parts(slug: &'static str, values: &[&str]) -> dorc_aid::tagged::RenderParts {
    let mut parts = dorc_cli::chrome_line_parts(&render_ctx(), slug, values);
    parts.push(dorc_aid::tagged::RenderPart::Arrangement {
        text: "\n".into(),
        slug: "cli-chrome-line-ending",
    });
    parts
}

fn report_invocation_error(diag: &Diag) {
    eprint!(
        "{}",
        dorc_cli::invocation_error_parts(&render_ctx(), diag, &Interner::default()).text()
    );
}

/// `dorc strip <path>` (`27D` rider-dorc-sh-unbuilt / `274` §13): read the file, erase every dorc
/// dialect construct (parser-backed — [`dorc_oracle::strip_file`]), print runnable stock sh to
/// stdout. An unmarked file passes through byte-identical (idempotent). Pure — the strip carries no
/// diagnostics today, but any it grows are reported to stderr so stdout stays exactly the artifact.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn strip_command(path: &str) -> Result<(), Diag> {
    let src = std::fs::read_to_string(path).map_err(|e| humane_read_error("source", path, &e))?;
    let mut interner = Interner::default();
    let stripped = dorc_oracle::strip_file(&mut interner, &src);
    for d in &stripped.diags {
        eprintln!("dorc: strip: {}", dorc_aid::diag::render_body(d, &interner));
    }
    print!("{}", stripped.value);
    std::io::stdout().flush().ok();
    Ok(())
}

/// Read one named input, taking `-` as stdin (`30I:rul-dash-is-stdin-in-any-filename-position`).
///
/// The claim on stdin was already adjudicated by the parser — two claimants refuse before anything
/// is read — so this seat only has to honour the spelling. A file literally named `-` is `./-`, per
/// the same convention.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn read_input(kind: &str, path: &str) -> Result<String, Diag> {
    if path == "-" {
        let mut text = String::new();
        return std::io::Read::read_to_string(&mut std::io::stdin(), &mut text)
            .map(|_| text)
            .map_err(|e| humane_read_error(kind, "<stdin>", &e));
    }
    std::fs::read_to_string(path).map_err(|e| humane_read_error(kind, path, &e))
}

/// Resolve the ordered PRE-SOURCE paths (`30I:rul-pre-source-is-dot-prelude`): the explicit
/// `--pre-source` list first, then every `*.oracle.sh` in each `--oracle-dir` (glob-sorted for
/// determinism — the cli is the I/O edge, but the ORDER it hands the kernel must be stable). A
/// directory that cannot be read is a humane error.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn resolve_pre_sources(
    pre_sources: &[String],
    oracle_dirs: &[String],
) -> Result<Vec<String>, Diag> {
    let mut paths: Vec<String> = pre_sources.to_vec();
    for dir in oracle_dirs {
        let entries =
            std::fs::read_dir(dir).map_err(|e| humane_read_error("oracle directory", dir, &e))?;
        let mut found: Vec<String> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.to_string_lossy().ends_with(".oracle.sh"))
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        found.sort();
        paths.extend(found);
    }
    Ok(paths)
}

/// Append every file the loaded oracles `.`-source, transitively, to the loaded set
/// (`28Q:pin-oracle-side-sourcing-amendment`; the pure half is `dorc_cli::sourcing`).
///
/// The filesystem half of the include-tree, and therefore sited at this edge rather than in the lib
/// (`io-at-edges-only` · `lib-target-is-a-loom-seam`). A target that cannot be read, or that does
/// not satisfy the dorc-lang contract, is NOT appended and NOT an error here: `sourcing::
/// include_tree` sees the same absence and suspends the sourcer's own vouches, which is where that
/// refusal belongs — attributed to the composition, once, rather than as a whole-run failure over a
/// file the admin may not even have known was involved.
///
/// Dedup is by the same lexical normal form the edge derivation matches on, so a file named on the
/// command line AND sourced by an entrypoint is loaded once, under the spelling that reached it
/// first. The loop re-scans what it appends, which is what makes it transitive; it terminates
/// because a path already present is never appended again.
///
/// The third return is exactly what this appended: sources acquired for somebody's load program
/// rather than named by the invocation. They are LOADABLE, never ambient roots — the roots reach
/// them at their authored `.` positions, and a synthetic second run of their programs would restore
/// definitions the author removed (`30Mc:required-root-occurrence-identity`).
fn read_sourced_oracles(
    cwd: &dorc_core::loadpath::Cwd,
    mut paths: Vec<String>,
    mut srcs: Vec<String>,
) -> (Vec<String>, Vec<String>, BTreeSet<usize>) {
    let named = paths.len();
    let mut cursor = 0;
    while let Some(src) = srcs.get(cursor).cloned() {
        cursor = cursor.saturating_add(1);
        if !dorc_cli::sourcing::satisfies_the_contract(&src) {
            continue;
        }
        for target in dorc_cli::sourcing::top_level_load_targets(&src) {
            let Some(wanted) = cwd.resolve_dot(&target) else {
                continue;
            };
            if paths
                .iter()
                .any(|path| cwd.resolve_operand(path).as_deref() == Some(wanted.as_str()))
            {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&wanted) else {
                continue;
            };
            if !dorc_cli::sourcing::satisfies_the_contract(&text) {
                continue;
            }
            paths.push(wanted);
            srcs.push(text);
        }
    }
    let acquired = (named..paths.len()).collect();
    (paths, srcs, acquired)
}

/// Read the sources a BOOK `.`-sources, transitively (`30I:rul-books-load-but-do-not-speak`).
///
/// The I/O half only: which of them a book reaches is [`dorc_cli::snapshot::book_reached`], asked
/// once the reading is done, so the binary and the in-process why driver partition by ONE rule.
///
/// A book's `.` is ordinary flowing sh — its operand resolves through the same value plane every
/// other operand does, so `SM_ORACLE_ROOT=./oracles; . "$SM_ORACLE_ROOT/alpha.oracle.sh"` names a
/// file without the engine recognizing one variable name (`30I` §2.1).
///
/// **A MARKED dorc-lang target is MODELLED; any other resolvable target is INCLUDED.** An ordinary
/// sh file a book `.`-sources is read for its BYTES and modelled not at all
/// (`30P:principle-book-code-source-is-inclusion`, r30's `mech-acquire-and-ship-plain-sh`): the
/// site walls exactly as it always has, nothing it declares binds, and what the reading buys is
/// that the artifact can MIRROR it beside the plan. Before that, the generated plan carried a `.`
/// naming a file the artifact never carried, which the atlas measured fatal on the host
/// (`floor30-atlas-dot-missing-file-is-fatal`) — the most common multi-file book shape died at
/// that line on a real apply. The splice and the single-stream paste stay forfeited
/// (`FORFEITS:forfeit-plain-sh-inclusion-analysis`), so a book's own non-dorc-lang material —
/// top-level `return`, caller-loop control, anything a dumb inliner would miscompile — stays where
/// its author put it (`30I` §7.2).
///
/// `load_dependencies` rides through because each round SOLVES: an include guard decides
/// differently when a dependency is wrongly a root, and one deciding "already loaded" wants
/// nothing — so a stale world here LOSES a file rather than merely over-reading one.
fn read_book_sourced(
    cwd: &dorc_core::loadpath::Cwd,
    book_path: &str,
    book_src: &str,
    mut paths: Vec<String>,
    mut srcs: Vec<String>,
    load_dependencies: &BTreeSet<usize>,
) -> BookSourced {
    let ambient = paths.len();
    let book_ast = dorc_syntax::parse(book_src).value;
    let mut refused: BTreeSet<String> = BTreeSet::new();
    for _ in 0..ACQUISITION_ROUNDS_CAP {
        let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
            cwd.clone(),
            paths.clone(),
            srcs.clone(),
            &dorc_cli::snapshot::LoadPositions::book_sourced((ambient..paths.len()).collect())
                .with_dependencies(load_dependencies.clone()),
            book_path,
            book_src,
        );
        let mut interner = Interner::default();
        let cfg = dorc_analysis::cfg::build(&book_ast).value;
        let value = dorc_analysis::value::analyze(&cfg, &book_ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = definition_table(&snapshot, &book_ast);
        let env = dorc_analysis::funcenv::analyze(&book_ast, &cfg, &definitions, &plane);

        let mut grew = false;
        for wanted in env.loads().wanted() {
            if !refused.insert(wanted.clone()) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(wanted) else {
                continue;
            };
            // A file that CLAIMS the dialect and fails its own contract stays REFUSED: the author
            // asked to be held to it by writing the marker
            // (`30G:rul-inertness-is-contract-never-engine-fact`), and admitting it as an inclusion
            // would make a lint failure a route to shipping — and would mirror an UNSTRIPPED marked
            // file onto the host. A marker-free one is an ordinary sh INCLUSION, read for its bytes
            // and modelled not at all; the snapshot derives that from the same marker.
            if dorc_oracle::marker::has_marker(&text)
                && !dorc_cli::sourcing::satisfies_the_contract(&text)
            {
                continue;
            }
            paths.push(wanted.clone());
            srcs.push(text);
            grew = true;
        }
        if !grew {
            break;
        }
    }
    BookSourced {
        reached: (ambient..paths.len()).collect(),
        paths,
        srcs,
    }
}

/// What the book-sourced acquisition read.
///
/// Which of them the engine MODELS is not carried here: the snapshot derives that from each file's
/// own `# dorc-lang` marker, so there is no second index set to keep in step
/// (`snapshot::LoadPositions::role_of`).
struct BookSourced {
    paths: Vec<String>,
    srcs: Vec<String>,
    /// Every source the loop appended: they load at a book `.`, never before line 1.
    reached: BTreeSet<usize>,
}

/// How many times the acquisition re-solves before settling.
///
/// Each round reads at least one file it had never seen or stops, and a chain of nested packages
/// is a handful deep, so the cap is a backstop rather than the real bound. Running out leaves a
/// package UNREAD, which is an unresolvable load — the withholding direction.
const ACQUISITION_ROUNDS_CAP: usize = 32;

/// Where this invocation stands (`30I:rul-dot-resolves-as-sh`) — the ONE environment read the load
/// model rests on, taken here because this file is the I/O edge (`io-at-edges-only`).
///
/// A platform that cannot answer yields [`Cwd::unknown`], under which every RELATIVE load resolves
/// nowhere and suspends rather than being guessed at (`30I` §3.2). The v0 profile models one cwd
/// for a whole run: marked oracle top level cannot change directory, and full book cwd flow is
/// owed rather than built.
fn invocation_cwd() -> dorc_core::loadpath::Cwd {
    std::env::current_dir()
        .ok()
        .map_or_else(dorc_core::loadpath::Cwd::unknown, |dir| {
            dorc_core::loadpath::Cwd::at(dir.to_string_lossy().into_owned())
        })
}

/// The REAL external-tool runner at the cli edge (`27R` §1 dir-runner-is-the-di-seam): the ONLY
/// non-hermetic part, kept out of the deterministic `dorc-lint` crate. Feeds the stripped bytes on the
/// tool's stdin (so the tool sees `-`/stdin, never a temp path — `dir-paths-stay-yours`).
struct SubprocessRunner;

impl dorc_lint::ExternalToolRunner for SubprocessRunner {
    fn available(&self, tool: &str) -> bool {
        tool_on_path(tool)
    }

    fn run(&self, tool: &str, args: &[&str], stdin: &[u8]) -> dorc_lint::ToolRun {
        use std::process::{Command, Stdio};
        let mut child = match Command::new(tool)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            // Vanished since the availability probe: rc 127, which the adapter turns into a finding.
            Err(e) => {
                return dorc_lint::ToolRun {
                    rc: 127,
                    stdout: Vec::new(),
                    stderr: e.to_string().into_bytes(),
                };
            }
        };
        if let Some(mut si) = child.stdin.take() {
            // A tool reading only a prefix of stdin closes early (BrokenPipe) — not an error here.
            let _ = si.write_all(stdin);
        }
        match child.wait_with_output() {
            Ok(out) => dorc_lint::ToolRun {
                rc: out.status.code().unwrap_or(-1),
                stdout: out.stdout,
                stderr: out.stderr,
            },
            Err(e) => dorc_lint::ToolRun {
                rc: -1,
                stdout: Vec::new(),
                stderr: e.to_string().into_bytes(),
            },
        }
    }
}

/// Is `tool` an executable on `PATH`? A `which`-style scan (no process spawn) — cross-platform via
/// `PATHEXT` on Windows (an extensionless script like a POSIX `shellcheck` won't be CreateProcess-able
/// on Windows, so a `.exe`/`.cmd` is required there; see `27S` for the e2e consequence).
fn tool_on_path(tool: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    for dir in std::env::split_paths(&path) {
        for ext in executable_exts() {
            if dir.join(format!("{tool}{ext}")).is_file() {
                return true;
            }
        }
    }
    false
}

/// The executable suffixes to try: just `""` on unix; `""` plus each `PATHEXT` entry on Windows.
fn executable_exts() -> Vec<String> {
    let mut exts = vec![String::new()];
    if cfg!(windows)
        && let Ok(pathext) = std::env::var("PATHEXT")
    {
        for e in pathext.split(';') {
            let e = e.trim();
            if !e.is_empty() {
                exts.push(e.to_owned());
            }
        }
    }
    exts
}

/// `dorc lint` driver (`27R` §5): resolve inputs, run `dorc-lint`, render, and compute the exit
/// trichotomy (0 clean / 1 findings-at-or-above / operational distinct from both). Operational checks
/// take precedence over the findings threshold (a compromised run must not read as a clean/findings
/// signal — `27R` §8 delta-exit-trichotomy-sharpened).
fn lint_command(args: &LintArgs) -> ExitCode {
    if args.list_sources {
        print!("{}", dorc_cli::lint_sources_parts(&render_ctx()).text());
        return ExitCode::SUCCESS;
    }
    let inputs = match read_lint_inputs("file", &args.files) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!(
                "dorc: lint: {}",
                dorc_aid::diag::render_body(&msg, &Interner::default())
            );
            return ExitCode::from(EXIT_LINT_OPERATIONAL);
        }
    };
    let oracle_paths = match resolve_pre_sources(&args.oracles, &args.oracle_dirs) {
        Ok(p) => p,
        Err(diag) => {
            report_invocation_error(&diag);
            return ExitCode::from(EXIT_USAGE);
        }
    };
    let oracles = match read_lint_inputs("oracle", &oracle_paths) {
        Ok(v) => v,
        Err(msg) => {
            eprintln!(
                "dorc: lint: {}",
                dorc_aid::diag::render_body(&msg, &Interner::default())
            );
            return ExitCode::from(EXIT_LINT_OPERATIONAL);
        }
    };
    let options = dorc_lint::LintOptions {
        tools_enabled: args.tools_enabled,
    };
    let only = (!args.sources.is_empty()).then_some(args.sources.as_slice());
    let report = if let [input] = inputs.as_slice()
        && !args.tools_enabled
        && oracles.is_empty()
        && only.is_none()
    {
        dorc_lint::lint_materialized_source(
            input.path.clone(),
            input.src.clone(),
            dorc_lint::SourcePolicy {
                tools_enabled: false,
            },
        )
        .report()
        .clone()
    } else {
        dorc_lint::lint(&inputs, &oracles, options, &SubprocessRunner, only)
    };

    let operational = dorc_cli::lint_operational_diagnostic(args, inputs.len(), &report);
    if inputs.is_empty() {
        if args.format == LintFormat::Jsonl {
            print!("{}", dorc_lint::render::render_jsonl(&report));
            std::io::stdout().flush().ok();
        }
        if let Some(diagnostic) = &operational {
            report_lint_operational(diagnostic);
        }
        return ExitCode::from(EXIT_LINT_OPERATIONAL);
    }

    match args.format {
        LintFormat::Human => print!(
            "{}",
            dorc_lint::render::render_human_parts_at(&render_ctx(), &report, args.verbosity).text()
        ),
        LintFormat::Jsonl => print!("{}", dorc_lint::render::render_jsonl(&report)),
    }
    std::io::stdout().flush().ok();

    if let Some(diagnostic) = operational {
        report_lint_operational(&diagnostic);
        return ExitCode::from(EXIT_LINT_OPERATIONAL);
    }
    if report.count_at_or_above(args.fail_on) > 0 {
        return ExitCode::from(EXIT_LINT_FINDINGS);
    }
    ExitCode::SUCCESS
}

/// Read a set of paths into [`dorc_lint::LintInput`]s; an unreadable file is a hard error (the lint
/// cannot lint what it cannot read — an operational failure, `27R` §8b). `kind` labels the humane error.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn read_lint_inputs(kind: &str, paths: &[String]) -> Result<Vec<dorc_lint::LintInput>, Diag> {
    let mut inputs = Vec::new();
    for path in paths {
        let src = std::fs::read_to_string(path).map_err(|e| humane_read_error(kind, path, &e))?;
        inputs.push(dorc_lint::LintInput {
            path: path.clone(),
            src,
        });
    }
    Ok(inputs)
}

/// Materialize the per-run PATH shim files into `dir` (`274` §5 / `27L` task-14). `files` is the
/// deterministic kernel product ([`dorc_plan::ProbePlan::shim_files`]); this is the I/O half at the
/// cli edge (`io-at-edges-only`): create the dir, write each `(name, content)`, mark it executable so
/// a `sudo -n <inner-check>` can exec the guest across the wrapper boundary. On unix the executable
/// bit is set here; on other platforms (msys) the exec permission is supplied by the session harness
/// (the e2e runner `chmod +x`s them), so a plain write suffices and this stays cross-platform.
#[expect(
    clippy::result_large_err,
    reason = "cold invocation path; see dorc_cli::parse_args_from"
)]
fn materialize_shim_dir(dir: &str, files: &BTreeMap<String, String>) -> Result<(), Diag> {
    if files.is_empty() {
        return Ok(()); // wrapper-free / already-answered run — nothing to materialize.
    }
    std::fs::create_dir_all(dir).map_err(|e| dorc_cli::shim_write_error(dir, &e))?;
    let mut staged: Vec<(std::path::PathBuf, std::path::PathBuf)> = Vec::new();
    for (name, content) in files {
        let path = std::path::Path::new(dir).join(name);
        let temp = std::path::Path::new(dir).join(format!(".{name}.dorc-shim-tmp"));
        if let Err(error) = write_shim(&temp, content) {
            for (temp, _) in &staged {
                let _ = std::fs::remove_file(temp);
            }
            let _ = std::fs::remove_file(&temp);
            return Err(dorc_cli::shim_write_error(
                &path.display().to_string(),
                &error,
            ));
        }
        staged.push((temp, path));
    }
    for (temp, path) in staged {
        std::fs::rename(&temp, &path)
            .map_err(|e| dorc_cli::shim_write_error(&path.display().to_string(), &e))?;
    }
    Ok(())
}

/// Every shim lands on a sibling temp first, and only a complete set is renamed into place: a
/// direct write left a failed run's dir holding this run's first shims beside the last run's rest,
/// under the names the next PATH lookup finds. The temps are ours by name, so a failed set removes
/// only what it just wrote.
fn write_shim(temp: &std::path::Path, content: &str) -> std::io::Result<()> {
    std::fs::write(temp, content)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(temp, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

/// The harness's clock pin (`rul-fixture-identity-never-production`) — Unix milliseconds, and the
/// ONE substitution point for the run's instant, exactly as `records::Framing::spike` is the one
/// substitution point for the run's nonce/host.
///
/// It exists because the why surface now DATES its output: a receipt header and a `reported` row's
/// run-instant are wall-clock values, so a committed transcript could otherwise never be a
/// fixpoint. A rendered-but-wrong timestamp was the alternative and is strictly worse — dating a
/// receipt wrongly is mis-attribution, the top of `271:rul-sin-ordering` — so the real clock stays
/// the default and the pin is something a harness must deliberately set.
const FIXTURE_CLOCK_ENV: &str = "DORC_FIXTURE_CLOCK_MS";

/// The clock this invocation runs on: the harness pin when one is set, else the real one.
/// Read at the process edge, once (`io-at-edges-only`). A free function rather than a
/// [`RunClock`] method because the type itself is pure and lives across the loom seam; the
/// environment read is what has to stay on this side of it.
fn clock_for_invocation() -> RunClock {
    match std::env::var(FIXTURE_CLOCK_ENV)
        .ok()
        .as_deref()
        .map(str::parse::<u64>)
    {
        Some(Ok(millis)) => RunClock::Ticking {
            at: dorc_core::RunInstant(millis),
            step_millis: 0,
        },
        Some(Err(_)) => RunClock::Absent,
        None => system_clock(),
    }
}

/// The harness's stdout-posture pin, on [`FIXTURE_CLOCK_ENV`]'s footing and for the same reason:
/// the fact is real, non-hermetic, and read once at the process edge, and a battery that drives the
/// binary as a subprocess has to be able to say which cell it means.
///
/// CLOSED vocabulary — `interactive` or `piped`. Anything else, including absence, asks the terminal
/// itself, so a typo degrades to the truth rather than to a chosen answer.
const STDOUT_POSTURE_ENV: &str = "DORC_STDOUT_POSTURE";

/// Is a person reading this run's stdout (`30Ng:rul-piped-stdout-carries-a-full-plan`)?
///
/// The ONE terminal read, at the edge. It decides which stream carries the ARTIFACT, and therefore
/// whether the plan on stdout has to be complete — so it is an injected value from here inward,
/// never a question anything below the edge asks (`io-at-edges-only` · `inv-determinism`).
fn stdout_posture() -> dorc_cli::artifact::StdoutPosture {
    use dorc_cli::artifact::StdoutPosture;
    use std::io::IsTerminal as _;

    match std::env::var(STDOUT_POSTURE_ENV).ok().as_deref() {
        Some("interactive") => StdoutPosture::Interactive,
        Some("piped") => StdoutPosture::NonInteractive,
        _ if std::io::stdout().is_terminal() => StdoutPosture::Interactive,
        _ => StdoutPosture::NonInteractive,
    }
}

/// The ONE wall-clock read. A clock the platform cannot place after the epoch answers
/// [`RunClock::Absent`] rather than saturating to a fabricated zero (`inv-no-throw`).
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

/// Publish a whole artifact set under `dir`, atomically (`30I` §7.5).
///
/// # Errors
/// Returns the publisher's closed refusal, having left no partial generation behind.
fn publish_artifact(
    dir: &str,
    set: &dorc_cli::artifact::ArtifactSet,
) -> Result<(), artifact_store::PublishRefusal> {
    artifact_store::publish(
        dir,
        set.files()
            .map(|file| (file.path.as_str(), file.bytes.as_str())),
    )
    .map(|_| ())
}

#[cfg(test)]
mod the_store_walk_hands_the_graph_real_documents {
    /// Every store-walk ingest passes the document's EXACT bytes, never a stand-in.
    ///
    /// `ReceiptGraph` classifies a second claimant to one identity by comparing what the two were
    /// READ FROM, so an empty slice makes every pair compare equal and `IdentityCollision` becomes
    /// unfirable on a real walk — green, silent, and wrong in the direction that hides a finding.
    /// Lexical because the property is about what the CALL SITE hands over: the graph's own battery
    /// already proves the classifier with real images, and could not have caught this.
    #[test]
    fn no_ingest_call_site_passes_a_stand_in_image() {
        let src = include_str!("main.rs");
        for species in ["ingest_plan", "ingest_intent", "ingest_outcome"] {
            let needle = format!("{species}(");
            let calls: Vec<&str> = src
                .lines()
                .map(str::trim)
                .filter(|line| line.contains(&needle) && !line.starts_with("///"))
                .collect();
            assert!(
                !calls.is_empty(),
                "no `{species}` call site found, so this census is counting the wrong thing"
            );
            for call in calls {
                assert!(
                    !call.contains("&[]"),
                    "`{species}` is handed a stand-in image: {call}"
                );
            }
        }
    }
}

#[cfg(test)]
mod fixpoint_freezes_the_environment_tests {
    /// The validity fixpoint must not reach the function environment (`28K` §2; `cli/CLAUDE.md`
    /// the-fixpoint-owns-the-rounds-and-builds-nothing-else).
    ///
    /// Env resolution is computed ONCE from the ORIGIN model and joins the frozen set alongside
    /// the book, the CFG, value-flow, the admitted records, the vouches, and the compiled probe.
    /// The forbidden scenario is concrete: a records-proven-dead branch containing a funcdef must
    /// not re-run resolution and un-contest a family mid-run. The fold's ratchet erases EFFECTS;
    /// it holds no authority over BINDINGS, and a license once withheld is never regained by a
    /// later round.
    ///
    /// Lexical, deliberately — the property is "the loop body cannot even spell it", which a type
    /// bound cannot express (`dorc_plan::erase`'s `licence_mint_has_exactly_one_caller` is the
    /// precedent). Its twin lives in `dorc_analysis::funcenv` and guards the other direction: that
    /// module cannot name a fixpoint-reachable type either.
    #[test]
    fn the_fixpoint_loop_body_calls_no_funcenv_entry_point() {
        let src = include_str!("fixpoint.rs");
        // Column-0 anchored (`spanless-gate-is-lexical`); the driver moved to the lib seam's
        // `fixpoint.rs` at the loom-final fold, so that is the file the fence scans. TWO regions
        // are in scope since `30K`: the settlement driver, and the round MODEL whose `classify` is
        // what a round actually re-derives through. Scanning the driver alone would leave the real
        // loop body unguarded.
        let region = |anchor: &str| -> String {
            let start = src
                .find(anchor)
                .unwrap_or_else(|| panic!("`{anchor}` is still a column-0 item of this file"));
            // Bounded at the column-0 closer; a slice running to EOF would make the gate worthless.
            let rest = &src[start..];
            let end = rest
                .find("\n}\n")
                .expect("the region has a column-0 closing brace");
            rest[..end].to_owned()
        };
        let body = format!(
            "{}{}",
            region("\npub fn settle_world("),
            region("\nimpl dorc_plan::RoundModel for WorldRoundModel<'_> {")
        );
        for forbidden in [
            "funcenv",
            "FuncEnv",
            "SourceLiteralPlane",
            "DefinitionTable",
        ] {
            assert!(
                !body.contains(forbidden),
                "`{forbidden}` appears inside the settlement — env resolution is frozen pre-loop, \
                 and re-deriving it per round would let a later round change which definition was \
                 live"
            );
        }
    }
}

#[cfg(test)]
mod acquisition_tests {
    use dorc_core::LiveDefinition;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    const MARKER: &str = "# dorc-lang/v0.2\n";

    /// A throwaway package tree, removed on drop. The acquisition's whole subject is which files
    /// it OPENS, so it cannot be exercised without real ones.
    struct Package {
        root: PathBuf,
    }

    /// Distinguishes two live packages that share a tag. Two `#[test]`s calling one helper run
    /// CONCURRENTLY, so a pid+tag path let each one's `Drop` delete the other's tree mid-run — a
    /// flake that surfaced as a missing dependency (`NoOpinion` where the case wanted `Withheld`)
    /// and as a `PermissionDenied` on Windows, where a pending directory delete blocks the
    /// re-create.
    static PACKAGES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    impl Package {
        fn new(tag: &str, files: &[(&str, String)]) -> Self {
            let serial = PACKAGES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let root = std::env::temp_dir()
                .join(format!("dorc-acq-{}-{tag}-{serial}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).expect("create package root");
            for (name, body) in files {
                std::fs::write(root.join(name), body).expect("write package file");
            }
            Self { root }
        }

        fn cwd(&self) -> dorc_core::loadpath::Cwd {
            dorc_core::loadpath::Cwd::at(self.root.to_string_lossy().into_owned())
        }
    }

    impl Drop for Package {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn names(paths: &[String]) -> BTreeSet<String> {
        paths
            .iter()
            .filter_map(|path| path.rsplit('/').next().map(str::to_owned))
            .collect()
    }

    fn ordered_names(paths: &[String]) -> Vec<String> {
        paths
            .iter()
            .filter_map(|path| path.rsplit('/').next().map(str::to_owned))
            .collect()
    }

    /// THE ACQUISITION, end to end (`30I:force-root-value-flow` · `30I:force-guarded-fallback`):
    /// a book sets an ordinary root, sources one entrypoint through it, and the entrypoint's
    /// include guard names a dependency through that same root — which nothing could have resolved
    /// before the book ran.
    ///
    /// What makes this the pin rather than a convenience: the files are found by DRIVING THE REAL
    /// LOADER and reading what it says it still wants, so the engine that decides a package's
    /// dependencies is the engine that reads them. A second resolver at this edge would answer
    /// this case and then drift (`30I:rul-one-loader-many-projections`).
    #[test]
    fn a_books_root_reaches_a_guarded_dependency_through_the_loader() {
        let package = Package::new(
            "rooted",
            &[
                (
                    "entry.dorc.sh",
                    format!(
                        "{MARKER}if command -v sm_q >/dev/null 2>&1; then\n   :\nelse\n   . \"$OPS_LIB/common.dorc.sh\"\nfi\n\nstep() {{ sm_q \"$1\" ;}}\n"
                    ),
                ),
                (
                    "common.dorc.sh",
                    format!("{MARKER}sm_q() {{ common \"$@\" ;}}\n"),
                ),
                (
                    "stranger.dorc.sh",
                    format!("{MARKER}elsewhere() {{ :; }}\n"),
                ),
            ],
        );
        let book = "OPS_LIB=.\n. \"$OPS_LIB/entry.dorc.sh\"\nstep first\n";
        let super::BookSourced {
            paths,
            srcs,
            reached,
            ..
        } = super::read_book_sourced(
            &package.cwd(),
            "book.sh",
            book,
            Vec::new(),
            Vec::new(),
            &BTreeSet::new(),
        );

        assert_eq!(
            names(&paths),
            ["common.dorc.sh".to_owned(), "entry.dorc.sh".to_owned()].into(),
            "the entrypoint AND the dependency its guard names; the co-resident stranger is \
             nobody's dependency and is never opened"
        );
        assert_eq!(
            reached,
            (0..paths.len()).collect::<BTreeSet<usize>>(),
            "everything a book `.` reached loads AT that line, never before line 1"
        );
        assert_eq!(srcs.len(), paths.len());
    }

    /// A book sourcing ordinary shell READS it and models nothing in it: the target signs no
    /// dorc-lang contract, so it enters the snapshot as a [`SourceRole::PlainInclusion`] whose
    /// bytes exist to be mirrored and whose site walls exactly as it always has
    /// (`30P:principle-book-code-source-is-inclusion`, r30's acquire-and-ship slice). That is what
    /// keeps a book's non-dorc-lang material where its author put it — including the top-level
    /// `return` and failing-command shapes a dumb inliner would miscompile
    /// (`FORFEITS:forfeit-plain-sh-inclusion-analysis`).
    ///
    /// The MARKED-but-not-inert cousin is the control, and it stays refused: an author who wrote
    /// the marker asked to be held to the contract, and admitting them here would make a lint
    /// failure a route to shipping.
    #[test]
    fn an_unmarked_target_is_included_but_a_failing_marked_one_is_refused() {
        let package = Package::new(
            "unmarked",
            &[("child.sh", "SM_LOADED=1\nsm_q() { :; }\nfalse\n".to_owned())],
        );
        let included = super::read_book_sourced(
            &package.cwd(),
            "book.sh",
            ". ./child.sh\n",
            Vec::new(),
            Vec::new(),
            &BTreeSet::new(),
        );
        assert_eq!(names(&included.paths), ["child.sh".to_owned()].into());
        assert_eq!(included.reached, [0].into());
        let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
            package.cwd(),
            included.paths.clone(),
            included.srcs.clone(),
            &dorc_cli::snapshot::LoadPositions::book_sourced(included.reached.clone()),
            "book.sh",
            ". ./child.sh\n",
        );
        assert_eq!(
            snapshot.role_of(0),
            Some(dorc_cli::snapshot::SourceRole::PlainInclusion),
            "read for its BYTES, and classified as modelled-not-at-all"
        );

        let claiming = Package::new(
            "marked-but-running",
            &[(
                "child.sh",
                format!("{MARKER}sm_q() {{ :; }}\nfalse\n").to_owned(),
            )],
        );
        let refused = super::read_book_sourced(
            &claiming.cwd(),
            "book.sh",
            ". ./child.sh\n",
            Vec::new(),
            Vec::new(),
            &BTreeSet::new(),
        );
        assert!(
            refused.paths.is_empty(),
            "a file that CLAIMS the dialect and runs a command at load stays refused: {:?}",
            refused.paths
        );
    }

    /// One acquisition-and-solve run over a package: which files the book's `.` lines REACHED, and
    /// the environment the acquired snapshot binds.
    ///
    /// Every load pin below asks those two questions of a different book, and the answer is the
    /// loader, the snapshot, the value plane and the environment driven in the ORDER the binary
    /// drives them. Spelled out per pin, that order drifts silently and a pin starts measuring a
    /// world the run never has (`30I:rul-one-loader-many-projections` is the same argument one
    /// layer down).
    struct Loaded {
        found: Vec<String>,
        reached: BTreeSet<usize>,
        cfg: dorc_analysis::cfg::Cfg,
        definitions: dorc_analysis::funcenv::DefinitionTable,
        env: dorc_analysis::funcenv::FuncEnv,
    }

    impl Loaded {
        fn of(cwd: dorc_core::loadpath::Cwd, book_path: &str, book: &str) -> Self {
            let acquired = super::read_book_sourced(
                &cwd,
                book_path,
                book,
                Vec::new(),
                Vec::new(),
                &BTreeSet::new(),
            );
            let (paths, srcs) = (acquired.paths, acquired.srcs);
            let reached = acquired.reached;
            let found = ordered_names(&paths);
            let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
                cwd,
                paths,
                srcs,
                &dorc_cli::snapshot::LoadPositions::book_sourced(reached.clone()),
                book_path,
                book,
            );
            let ast = dorc_syntax::parse(book).value;
            let cfg = dorc_analysis::cfg::build(&ast).value;
            let mut interner = dorc_core::Interner::default();
            let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
            let definitions = dorc_cli::world::definition_table(&snapshot, &ast);
            let env = {
                let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
                dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane)
            };
            Self {
                found,
                reached,
                cfg,
                definitions,
                env,
            }
        }

        /// What a shell would have live for `name` at the book's last line.
        fn at_exit(&self, name: &str) -> LiveDefinition {
            dorc_analysis::funcenv::LiveDefinitions::new(&self.env, &self.definitions)
                .definition_before(self.cfg.exit(), name)
        }

        /// The files a BOOK `.` really loaded, in walk order — the ordering half of a set-valued
        /// operand, which the acquired-path list alone cannot show.
        fn book_loads(&self) -> Vec<String> {
            self.env
                .loads()
                .occurrences()
                .iter()
                .filter(|occurrence| {
                    matches!(occurrence.sourcer, dorc_analysis::load::LoadSourcer::Book)
                })
                .filter_map(|occurrence| occurrence.target.rsplit('/').next().map(str::to_owned))
                .collect()
        }
    }

    /// The script-relative load Dorc may evaluate ENTIRELY ITSELF (né
    /// `p-x-load-operand-param-expansion-of-dollar-zero`, promoted).
    ///
    /// `${0%/*}` is pure parameter expansion over `$0`, and `$0` is the authored book path, which
    /// the controller owns. So the operand is a function of program text plus controller-known
    /// inputs, evaluable through the closed allowlist of pure shell operations, with no command
    /// run and no tool modelled — which is exactly what separates it from the two pins below.
    #[test]
    fn a_dollar_zero_parameter_expansion_sites_a_book_dependency() {
        let package = Package::new(
            "book-param-zero-load",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let book_path = package.root.join("book.sh").to_string_lossy().into_owned();
        let loaded = Loaded::of(
            package.cwd(),
            &book_path,
            ". \"${0%/*}/helpers.dorc.sh\"\nbook_helper\n",
        );
        let helper = loaded.at_exit("book_helper");

        assert_eq!(loaded.found, ["helpers.dorc.sh"]);
        assert_eq!(loaded.reached, [0].into());
        assert!(matches!(helper, LiveDefinition::Live(_)));
    }

    /// The SECOND live spelling of `$0`, and the one the world actually types (né
    /// `p-x-dollar-zero-slashless-book-path-resolves`, promoted).
    ///
    /// `dorc plan book.sh` from the book's own directory hands the controller a slashless path, and
    /// `sh book.sh` hands the shell the same. A slashless `$0` has no directory component at all, so
    /// `${0%/*}` is the whole word — the trap `30P:model-symbolic-dollar-zero` measured — and the
    /// engine has to normalise the spelling against the load cwd (the world's `dirname` answers `.`
    /// here) instead of trimming a slash that is not there. The three sibling pins all build an
    /// ABSOLUTE book path, so a model derived from one alone would green them and silently answer
    /// `book.sh/helpers.dorc.sh` for this one.
    ///
    /// CFG shape exercised: one top-level `.` whose operand is a single double-quoted word made of
    /// a parameter expansion plus a literal tail, with the call to the bound helper below it —
    /// straight-line, so nothing but the operand's own evaluation can decide the binding.
    #[test]
    fn a_slashless_book_path_still_names_the_books_own_directory() {
        let package = Package::new(
            "book-slashless-zero-load",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let loaded = Loaded::of(
            package.cwd(),
            "book.sh",
            ". \"${0%/*}/helpers.dorc.sh\"\nbook_helper\n",
        );
        let helper = loaded.at_exit("book_helper");

        assert_eq!(loaded.found, ["helpers.dorc.sh"]);
        assert!(matches!(helper, LiveDefinition::Live(_)));
    }

    /// Floor-valid text is never a PARSE violation (né `p-x-computed-dot-parses-and-havocs`,
    /// promoted).
    ///
    /// `. "$(dirname "$0")/helpers.dorc.sh"` parses and runs under `posh ∩ dash`, so
    /// `30P:rul-floor-valid-text-never-parse-fails` forbids the parser refusing it — which it did
    /// until now, taking the whole invocation's exit code with it before any analysis happened.
    /// The shape is three-part: the parser hands back a rich AST with no Error, the LOAD plane
    /// answers (absent the static-predict tier the operand is ⊤, so the site is an ordinary point
    /// havoc), and the CAUSE is named, which is what the cli's `EXIT_LOAD_UNRESOLVABLE` keys on —
    /// the outcome kept, only its tier moved.
    ///
    /// What is NOT asserted, deliberately: the operand RESOLVING. That is
    /// `p-x-load-operand-dirname-of-dollar-zero`, which waits on an authored `dirname__predict`
    /// (`30P:rul-static-predict-sites-loads`) and stays red past this lane.
    ///
    /// CFG shape exercised: one top-level `.` whose operand word carries a `CommandSubst` part.
    #[test]
    fn a_computed_dot_operand_parses_and_havocs_instead_of_refusing_the_book() {
        let package = Package::new(
            "book-computed-dot-parses",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let book = ". \"$(dirname \"$0\")/helpers.dorc.sh\"\nbook_helper\n";
        let parsed = dorc_syntax::parse(book);
        let errors = parsed
            .diags
            .iter()
            .filter(|d| d.severity() == dorc_aid::Severity::Error)
            .count();
        let book_path = package.root.join("book.sh").to_string_lossy().into_owned();
        let loaded = Loaded::of(package.cwd(), &book_path, book);

        assert_eq!(errors, 0, "floor-valid text is never a parse violation");
        assert_eq!(
            loaded.env.unresolvable_loads().len(),
            1,
            "and the load plane owns the answer: a `.` whose operand it cannot evaluate is a \
             point havoc, exactly like any other unresolvable source"
        );
        assert!(
            loaded.env.havoc_causes().values().any(|cause| matches!(
                cause,
                dorc_analysis::funcenv::HavocCause::ComputedSubstitution
            )),
            "and it names the CAUSE, which is what the cli's pre-network outcome keys on"
        );
    }

    /// The cwd domain of `30P:principle-unknown-source-is-a-point-havoc` (né
    /// `p-x-unknown-source-havocs-the-cwd`, promoted), and what it does and does NOT cost.
    ///
    /// An unresolvable `.` runs arbitrary sh in the caller's own shell, and `cd` persists out of a
    /// sourced file (floor-measured). So the modeled working directory is ⊤ below it, and a
    /// RELATIVE operand there names a file the controller cannot IDENTIFY.
    ///
    /// What that costs is BINDING AUTHORITY and nothing else (ruled 2026-08-22): the file is still
    /// acquired and still mirrored at its authored relative path, because cwd-parity is what keeps
    /// the shipped tree faithful to the author's, and a generated plan that dies at the `.` on the
    /// host would be a regression on today's poisoned books. So the site behaves exactly like an
    /// unresolvable one — it walls, it takes no custody, it lifts no vouch — while the artifact
    /// still carries the bytes.
    ///
    /// The control below is what says this asks for precision and not for a blanket withdrawal: the
    /// SAME relative load with nothing unknown above it resolves and binds today and must keep
    /// doing so, so the withholding is the unknown source's doing and nothing else's.
    ///
    /// CFG shape exercised: two top-level `.` commands in sequence, the first with a ⊤ operand and
    /// the second with a cwd-relative literal — a straight-line flow, so the only thing that can
    /// carry the first's effect to the second is the domain this cell asks for.
    #[test]
    fn a_relative_source_below_an_unknown_one_cannot_be_identified() {
        let package = Package::new(
            "book-cwd-havoc",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let havoced = Loaded::of(
            package.cwd(),
            "book.sh",
            ". \"$SITE_PROFILE/rc\"\n. ./helpers.dorc.sh\nbook_helper\n",
        );
        let control = Loaded::of(
            package.cwd(),
            "book.sh",
            ". ./helpers.dorc.sh\nbook_helper\n",
        );

        assert_eq!(
            havoced.found,
            ["helpers.dorc.sh"],
            "the file is still READ — acquisition is kept because the acquisition fixpoint reads \
             a transient round-1 clobber, and a withheld name is the safe direction. What it \
             loses is its CARRIAGE as well as its authority: nothing is shipped for a load whose \
             operand may resolve elsewhere (`30P:law-no-unsoundness-below-a-blind-act`), which \
             `artifact::tests::a_load_below_a_blind_act_ships_no_copy` is where that half is \
             measured"
        );
        assert_eq!(
            havoced.env.unresolvable_loads().len(),
            2,
            "and both sites wall — the second for the cwd, not for its own operand"
        );
        assert_eq!(
            havoced.at_exit("book_helper"),
            LiveDefinition::Withheld,
            "the unknown source may have cd'd, so nothing the second load declares can be said to \
             bind: {:?}",
            havoced.at_exit("book_helper")
        );
        assert_eq!(
            control.found,
            ["helpers.dorc.sh"],
            "control: with nothing unknown above it the same operand resolves, so the withholding \
             above is the load's doing and not a blanket refusal of relative operands"
        );
        assert!(
            matches!(control.at_exit("book_helper"), LiveDefinition::Live(_)),
            "control: and it binds"
        );
    }

    /// A `cd` clobbers the working directory for every relative load BELOW it — and a `cd` inside
    /// `( … )` clobbers nothing outside the paren.
    ///
    /// `( cd "$dir" && … )` is the idiom books are full of, and the whole reason the cwd closure is
    /// a scope-aware walk rather than a suffix: without the paren rule this cell would lose its
    /// package to a subshell that provably cannot move the caller's directory. The CFG already
    /// says which is which — `cfg::lower_scoped` pushes a scope for `( )` and `$( )` and for
    /// nothing else — so the answer is sh's, not a special case.
    ///
    /// CFG shape exercised: one subshell (a real `ScopeEnter`/`ScopeExit` pair) whose body `cd`s,
    /// against a top-level `cd`, each followed by the SAME relative `.` and a call to what it
    /// declares.
    #[test]
    fn a_cd_inside_a_subshell_clobbers_nothing_outside_it() {
        let package = Package::new(
            "book-cd-scope",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let tail = ". ./helpers.dorc.sh\nbook_helper\n";
        let scoped = Loaded::of(
            package.cwd(),
            "book.sh",
            &format!("( cd nested && wombat sync )\n{tail}"),
        );
        let bare = Loaded::of(package.cwd(), "book.sh", &format!("cd nested\n{tail}"));

        assert!(
            matches!(scoped.at_exit("book_helper"), LiveDefinition::Live(_)),
            "a subshell's `cd` dies at the paren, so the load below it binds: {:?}",
            scoped.at_exit("book_helper")
        );
        assert_eq!(
            bare.at_exit("book_helper"),
            LiveDefinition::Withheld,
            "a top-level `cd` really does move the coordinate the next relative `.` resolves \
             against, so that load names a file the controller cannot identify"
        );
        assert_eq!(
            bare.found,
            ["helpers.dorc.sh"],
            "and it is still acquired and mirrored — cwd-⊤ costs authority, never the shipped tree"
        );
    }

    /// `p-x-load-operand-case-over-dollar-zero` — the script-relative spelling that is EXACT under
    /// BOTH invocations, and the one this lane's evaluator cannot reach.
    ///
    /// `${0%/*}` is exact for the spelling Dorc invokes and DEAD for the slashless one; the `case`
    /// fold is what an author writes to be correct under both, so it is the form Dorc should be
    /// steering toward (`KNOBS:kLANG` stewardship: never teach the spelling that breaks). It is
    /// also strictly harder: the computation left the WORD and became CONTROL FLOW, so answering
    /// it needs a `case`-pattern member of `dec-decidable-set-v0` AND a per-spelling solve whose
    /// results meet — two license-review-tier changes, one of them structural.
    ///
    /// CFG shape exercised: a two-armed `case` over `$0` assigning one variable, then a top-level
    /// `.` of a word built from that variable — the value plane joins the arms to ⊤ at the load,
    /// which is why the site havocs today for exactly the right reason.
    #[test]
    fn a_case_over_dollar_zero_sites_a_book_dependency() {
        let package = Package::new(
            "book-case-zero-load",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let book_path = package.root.join("book.sh").to_string_lossy().into_owned();
        let loaded = Loaded::of(
            package.cwd(),
            &book_path,
            "case $0 in\n*/*) here=${0%/*} ;;\n*) here=. ;;\nesac\n. \"$here/helpers.dorc.sh\"\nbook_helper\n",
        );
        let helper = loaded.at_exit("book_helper");

        internal_tooling::xfail::xfail_until("p-x-load-operand-case-over-dollar-zero", || {
            assert_eq!(loaded.found, ["helpers.dorc.sh"]);
            assert!(matches!(helper, LiveDefinition::Live(_)));
        });
    }

    /// `p-x-load-operand-dirname-of-dollar-zero` — the same dependency, spelled through a COMMAND.
    ///
    /// Book libraries overwhelmingly locate sibling files as `$(dirname "$0")/helpers.sh`, and the
    /// answer is the same directory the pin above resolves without help. What differs is the route:
    /// predicting `dirname`'s output inside the engine is tool-modelling, which
    /// `identity-declared-never-inferred` forbids the engine outright. So the target is real and
    /// the ROUTE to it is open — `ask-dollar-zero-command-substitution-path`, an authored-model
    /// path rather than one hard-coded utility.
    #[test]
    fn a_dirname_command_substitution_sites_a_book_dependency() {
        let package = Package::new(
            "book-dirname-load",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let book_path = package.root.join("book.sh").to_string_lossy().into_owned();
        let loaded = Loaded::of(
            package.cwd(),
            &book_path,
            ". \"$(dirname \"$0\")/helpers.dorc.sh\"\nbook_helper\n",
        );
        let helper = loaded.at_exit("book_helper");

        internal_tooling::xfail::xfail_until("p-x-load-operand-dirname-of-dollar-zero", || {
            assert_eq!(loaded.found, ["helpers.dorc.sh"]);
            assert_eq!(loaded.reached, [0].into());
            assert!(matches!(helper, LiveDefinition::Live(_)));
        });
    }

    /// `p-x-load-operand-cd-pwd-of-dollar-zero` — the absolutizing spelling of the same question,
    /// and the one that runs TWO commands to ask it.
    ///
    /// `SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)` is the idiom authors reach for when a relative
    /// `$0` would break a later `cd`. It rides the same open ruling as the pin above and is pinned
    /// separately because the value reaches the `.` through a VARIABLE: a lane that special-cases
    /// the operand's syntax rather than its provenance would green one and not the other.
    #[test]
    fn a_cd_pwd_script_dir_sites_a_book_dependency() {
        let package = Package::new(
            "book-cd-pwd-load",
            &[(
                "helpers.dorc.sh",
                format!("{MARKER}book_helper() {{ :; }}\n"),
            )],
        );
        let book_path = package.root.join("book.sh").to_string_lossy().into_owned();
        let loaded = Loaded::of(
            package.cwd(),
            &book_path,
            "SCRIPT_DIR=$(cd \"$(dirname \"$0\")\" && pwd)\n. \"$SCRIPT_DIR/helpers.dorc.sh\"\nbook_helper\n",
        );
        let helper = loaded.at_exit("book_helper");

        internal_tooling::xfail::xfail_until("p-x-load-operand-cd-pwd-of-dollar-zero", || {
            assert_eq!(loaded.found, ["helpers.dorc.sh"]);
            assert_eq!(loaded.reached, [0].into());
            assert!(matches!(helper, LiveDefinition::Live(_)));
        });
    }

    /// `p-x-glob-load-acquires-members` — a source glob is a SET-valued operand.
    ///
    /// A finite glob over authored book-local files is an ordered family of ordinary `.` acts, not
    /// an excuse to reject the whole book. It expands against the authored SNAPSHOT — the same
    /// bytes every other consumer reads — so the population is closed, and it reuses the
    /// loop-propagation lane's member machinery rather than minting a second one.
    #[test]
    fn a_book_load_glob_acquires_each_matching_dependency_in_order() {
        let package = Package::new(
            "book-glob-load",
            &[
                ("a.dorc.sh", format!("{MARKER}a_helper() {{ :; }}\n")),
                ("b.dorc.sh", format!("{MARKER}b_helper() {{ :; }}\n")),
            ],
        );
        let loaded = Loaded::of(
            package.cwd(),
            "book.sh",
            "for plugin in ./*.dorc.sh; do\n   . \"$plugin\"\ndone\na_helper\nb_helper\n",
        );
        let book_loads = loaded.book_loads();
        let (a, b) = (loaded.at_exit("a_helper"), loaded.at_exit("b_helper"));

        internal_tooling::xfail::xfail_until("p-x-glob-load-acquires-members", || {
            assert_eq!(loaded.found, ["a.dorc.sh", "b.dorc.sh"]);
            assert_eq!(loaded.reached, [0, 1].into());
            assert_eq!(book_loads, ["a.dorc.sh", "b.dorc.sh"]);
            assert!(matches!(a, LiveDefinition::Live(_)));
            assert!(matches!(b, LiveDefinition::Live(_)));
        });
    }

    /// `p-x-glob-load-members-are-order-unknown` — the members are a SET, and the answer is a
    /// UNIVERSAL MEET over every order of their whole load PROGRAMS.
    ///
    /// Pathname expansion sorts by the TARGET's collation, which depends on its locale and on
    /// bytes the controller never sees. The collision cell is the obvious half: two members
    /// defining one name with DIFFERENT bytes leave the winner genuinely undetermined, so the name
    /// must WITHHOLD — picking either is a wrong-elision under `visibility-is-full-positional`.
    ///
    /// The RETARGET (`30Pb:fnd-glob-order-needs-whole-program-meet`, AGREED at `30P`): a
    /// collision-only rule is too narrow, because a member's load program is not a list of
    /// definitions. `b.dorc.sh` here `unset -f`s a name `a.dorc.sh` is the SOLE definer of, and the
    /// two orders disagree — a,b removes it, b,a leaves it live — so the meet withholds a name no
    /// member contests by bytes. Assignments, `cd`, and `exit` inside a member are the same rule
    /// and the same meet; they are named rather than asserted because they want the deferred glob
    /// lane's whole-program walk, not a third fixture here.
    ///
    /// CFG shape exercised: a `for`-loop over a glob word whose body is a single `.` of the
    /// iteration variable — one lowered load site standing for an unordered member family, which
    /// is exactly the shape a per-order meet has to answer at.
    #[test]
    fn glob_members_meet_over_every_order_of_their_load_programs() {
        let package = Package::new(
            "book-glob-collide",
            &[
                (
                    "a.dorc.sh",
                    format!(
                        "{MARKER}shared_helper() {{ common a \"$@\" ;}}\nonly_in_a() {{ :; }}\n"
                    ),
                ),
                (
                    "b.dorc.sh",
                    format!("{MARKER}shared_helper() {{ common b \"$@\" ;}}\nunset -f only_in_a\n"),
                ),
            ],
        );
        let loaded = Loaded::of(
            package.cwd(),
            "book.sh",
            "for plugin in ./*.dorc.sh; do\n   . \"$plugin\"\ndone\nshared_helper x\nonly_in_a\n",
        );
        let shared = loaded.at_exit("shared_helper");
        let removed = loaded.at_exit("only_in_a");

        internal_tooling::xfail::xfail_until("p-x-glob-load-members-are-order-unknown", || {
            assert_eq!(loaded.found, ["a.dorc.sh", "b.dorc.sh"]);
            assert_eq!(
                shared,
                LiveDefinition::Withheld,
                "no member may win a name two members spell differently"
            );
            assert_eq!(
                removed,
                LiveDefinition::Withheld,
                "and a name ONE member declares while another removes it is undetermined the same \
                 way — the meet is over programs, not over declaration sets: {removed:?}"
            );
        });
    }

    /// `p-x-glob-load-no-match-aborts` — a glob that matches nothing sources the LITERAL PATTERN.
    ///
    /// Unmatched pathname expansion leaves the word alone, so the loop body runs once with
    /// `plugin` holding `./*.dorc.sh` and `.` is handed a filename no directory entry answers.
    /// Today's answer already walls — but for the WRONG reason (the operand was never evaluated),
    /// so the discriminator this pin asserts is that the pattern reaches the load account as a
    /// NAMED target: the engine read the operand, and what it named is unloadable.
    ///
    /// FLOOR QUESTION, not asserted here: `.` is a POSIX special builtin, so a failing one should
    /// terminate a non-interactive shell outright — which would make everything after the loop
    /// unreachable rather than merely unbound. That is a claim about `posh ∩ dash` and belongs to
    /// the differential lane, not to a unit pin.
    #[test]
    fn a_glob_matching_nothing_sources_the_literal_pattern() {
        let package = Package::new(
            "book-glob-no-match",
            &[("README", "no dorc-lang member lives here\n".to_owned())],
        );
        let loaded = Loaded::of(
            package.cwd(),
            "book.sh",
            "for plugin in ./*.dorc.sh; do\n   . \"$plugin\"\ndone\n",
        );
        let wanted = loaded.env.loads().wanted().clone();

        internal_tooling::xfail::xfail_until("p-x-glob-load-no-match-aborts", || {
            assert!(
                loaded.found.is_empty(),
                "nothing matched, so nothing is acquired: {:?}",
                loaded.found
            );
            assert_eq!(
                loaded.env.unresolvable_loads().len(),
                1,
                "the site still walls — the pattern names no loadable file"
            );
            assert!(
                wanted.iter().any(|target| target.ends_with("*.dorc.sh")),
                "and the operand was EVALUATED to the pattern itself, which is the fact that \
                 separates this from an operand the engine could not read: {wanted:?}"
            );
        });
    }

    /// `p-x-book-code-source-is-inclusion` — a resolvable `.` of ORDINARY sh is textual inclusion.
    ///
    /// Two cells, and the unconditional one is the bigger hole. Cell (a): `. ./helpers.sh` of plain
    /// sh splices those definitions in at the load site, so the helper is live below it and the
    /// book's own later role definition is untouched — today the target signs no dorc-lang contract,
    /// is never opened, and walls the rest of the book. Cell (b): the same inclusion under a
    /// filesystem-existence guard, where the branch decides whether it happened, so the guarded
    /// helper is `May` (withheld) while the unconditional role definition below the `fi` is live.
    ///
    /// Requiring the target to sign the dorc-lang contract is the refused alternative: it turns
    /// near-universal book acceptance into oracle ceremony for the admin
    /// (`two-users-never-conflated`).
    #[test]
    fn an_ordinary_sh_source_is_textual_inclusion() {
        let plain = Package::new(
            "book-plain-sh-load",
            &[("helpers.sh", "helper_fn() { :; }\n".to_owned())],
        );
        let unconditional = Loaded::of(
            plain.cwd(),
            "book.sh",
            ". ./helpers.sh\nhork__is_converged() { hork status \"$1\" ;}\nhelper_fn\nhork tune web\n",
        );
        let (helper, role) = (
            unconditional.at_exit("helper_fn"),
            unconditional.at_exit("hork__is_converged"),
        );

        let guarded_package = Package::new(
            "book-guarded-load",
            &[("optional.sh", "optional_helper() { :; }\n".to_owned())],
        );
        let guarded = Loaded::of(
            guarded_package.cwd(),
            "book.sh",
            "if [ -r ./optional.sh ]; then\n   . ./optional.sh\n   optional_helper\nfi\nhork__is_converged() { :; }\nhork tune web\n",
        );
        let optional = guarded.at_exit("optional_helper");
        let guarded_role = guarded.at_exit("hork__is_converged");

        internal_tooling::xfail::xfail_until("p-x-book-code-source-is-inclusion", || {
            assert_eq!(unconditional.found, ["helpers.sh"]);
            assert_eq!(unconditional.reached, [0].into());
            assert!(
                matches!(helper, LiveDefinition::Live(_)),
                "cell (a): an unconditional inclusion binds its definitions: {helper:?}"
            );
            assert!(
                matches!(role, LiveDefinition::Live(_)),
                "cell (a): and leaves the book's own later definition alone: {role:?}"
            );

            assert_eq!(guarded.found, ["optional.sh"]);
            assert_eq!(guarded.reached, [0].into());
            assert_eq!(
                optional,
                LiveDefinition::Withheld,
                "cell (b): the guard decides whether the inclusion happened, so its helper is May"
            );
            assert!(
                matches!(guarded_role, LiveDefinition::Live(_)),
                "cell (b): the definition below the `fi` is unconditional: {guarded_role:?}"
            );
        });
    }

    /// The conservative half of the answer for the unconditional cell above, and it did NOT move
    /// when acquisition landed — which is the whole point of pinning it separately.
    ///
    /// "Walls" is a compound of two facts a repair could deliver separately: the site is disclosed
    /// as an unresolvable load (which is what walls it and arms defensive emission), and the file's
    /// names stay outside the unit's universe entirely, which is `NoOpinion` rather than a
    /// withholding. `mech-acquire-and-ship-plain-sh` changed only whether the bytes are READ; a
    /// lane that let the reading bind names, or that stopped disclosing the site, would leave one
    /// half standing and this says which
    /// (`FORFEITS:forfeit-plain-sh-inclusion-analysis`).
    #[test]
    fn a_plain_sh_source_walls_even_though_it_is_acquired() {
        let package = Package::new(
            "book-plain-sh-interim",
            &[("helpers.sh", "helper_fn() { :; }\n".to_owned())],
        );
        let loaded = Loaded::of(package.cwd(), "book.sh", ". ./helpers.sh\nhelper_fn\n");

        assert_eq!(
            loaded.env.unresolvable_loads().len(),
            1,
            "the site is disclosed as an unresolvable load, which is what walls it"
        );
        assert_eq!(
            loaded.at_exit("helper_fn"),
            LiveDefinition::NoOpinion,
            "its names are outside the unit's universe, not withheld within it"
        );
    }

    /// The r30 slice of `30P:principle-book-code-source-is-inclusion`, and the twin of the wall pin
    /// above (promoted from `p-x-plain-sh-inclusion-ships-beside-the-plan`).
    ///
    /// `30P:ask-inclusion-in-r30` splits inclusion into three mechanics and takes exactly one:
    /// ACQUIRE the plain-sh target and record it as a load occurrence, so the placement already in
    /// `cli::artifact` mirrors it beside the plan — and analyze NOTHING in it. Today the file is not
    /// read at all, so the generated plan carries a `.` naming a file the artifact never carried,
    /// which the atlas measured FATAL on the host (`floor30-atlas-dot-missing-file-is-fatal`); the
    /// most common multi-file book shape dies at that line on a real apply.
    ///
    /// The two halves that must NOT move with it are asserted beside the two that must: the site
    /// still walls, and the file's names stay outside the unit's universe
    /// (`FORFEITS:forfeit-plain-sh-inclusion-analysis` — no splice, no bindings, no sites). A lane
    /// that acquired the file and let `definition_table` register its declarations would deliver
    /// tier-1 inclusion by accident, which is forfeited and reddens
    /// `p-x-book-code-source-is-inclusion`'s guarded cell as well.
    ///
    /// CFG shape exercised: one top-level `.` of a literal relative operand, straight-line, with a
    /// call to the included file's helper below it.
    #[test]
    fn a_plain_sh_source_is_acquired_and_placed_without_being_analyzed() {
        let package = Package::new(
            "book-plain-sh-ships",
            &[("helpers.sh", "helper_fn() { :; }\n".to_owned())],
        );
        let loaded = Loaded::of(package.cwd(), "book.sh", ". ./helpers.sh\nhelper_fn\n");
        let occurrences = loaded.book_loads();
        let helper = loaded.at_exit("helper_fn");

        assert_eq!(
            loaded.found,
            ["helpers.sh"],
            "the file is READ, which is what gives the artifact bytes to mirror"
        );
        assert_eq!(
            occurrences,
            ["helpers.sh"],
            "and it enters the load account as an occurrence, which is what the placement keys to \
             (`30I:rul-bundles-key-to-load-occurrences`)"
        );
        assert_eq!(
            loaded.env.unresolvable_loads().len(),
            1,
            "the site still WALLS: acquiring bytes is not modelling them"
        );
        assert_eq!(
            helper,
            LiveDefinition::NoOpinion,
            "and its names stay outside the unit's universe — the splice is forfeited, not \
             delivered by the back door: {helper:?}"
        );
    }

    /// An invocation-named oracle stays AMBIENT even though the acquisition runs over it: only
    /// what the loop APPENDS is book-reached. Ambience decides whether a definition can license
    /// sites above its own load point (`visibility-is-full-positional`), so the boundary is the
    /// safety-relevant half of the answer.
    #[test]
    fn an_invocation_named_oracle_stays_ambient() {
        let package = Package::new(
            "ambient",
            &[("pkg.oracle.sh", format!("{MARKER}sm_q() {{ :; }}\n"))],
        );
        let named = package.root.join("pkg.oracle.sh").display().to_string();
        let acquired = super::read_book_sourced(
            &package.cwd(),
            "book.sh",
            "sm_q\n",
            vec![named],
            vec![format!("{MARKER}sm_q() {{ :; }}\n")],
            &BTreeSet::new(),
        );
        assert_eq!(acquired.paths.len(), 1);
        assert!(
            acquired.reached.is_empty(),
            "named on the command line ⇒ ambient"
        );
    }

    /// The whole acquisition, from one named pre-source to the environment a book site reads, so
    /// the answer is the run's own rather than a hand-built cousin of it.
    fn live_at_exit(files: &[(&str, String)], named: &str, role: &str) -> LiveDefinition {
        let package = Package::new("pre-source-roots", files);
        let cwd = package.cwd();
        let named_path = package.root.join(named).to_string_lossy().into_owned();
        let named_src = files
            .iter()
            .find(|(name, _)| *name == named)
            .map(|(_, src)| src.clone())
            .expect("the named root is one of the files");
        let (paths, srcs, dependencies) =
            super::read_sourced_oracles(&cwd, vec![named_path], vec![named_src]);
        let book = "wombat sync a.conf\n";
        let acquired = super::read_book_sourced(&cwd, "book.sh", book, paths, srcs, &dependencies);
        let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
            cwd,
            acquired.paths,
            acquired.srcs,
            &dorc_cli::snapshot::LoadPositions::book_sourced(acquired.reached)
                .with_dependencies(dependencies),
            "book.sh",
            book,
        );
        let ast = dorc_syntax::parse(book).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut interner = dorc_core::Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        let definitions = dorc_cli::world::definition_table(&snapshot, &ast);
        let env = dorc_analysis::funcenv::analyze(&ast, &cfg, &definitions, &plane);
        dorc_analysis::funcenv::LiveDefinitions::new(&env, &definitions)
            .definition_before(cfg.exit(), role)
    }

    /// THE REPLAY. A pre-source's dependency runs where its author `.`'d it and nowhere else, so
    /// the `unset -f` after that `.` is the last word, as in sh. Promoted to a root, its program
    /// ran again after the authored one finished and RESTORED the verdict function — vouch
    /// authority no live judgment stands behind (`30Mc:finding-transitive-pre-source-replays-as-root`).
    #[test]
    fn a_pre_source_dependency_runs_only_at_its_authored_dot() {
        let role = "wombat__is_converged";
        assert_eq!(
            live_at_exit(
                &[
                    (
                        "entry.dorc.sh",
                        format!("{MARKER}. ./verdict.dorc.sh\nunset -f {role}\n")
                    ),
                    ("verdict.dorc.sh", format!("{MARKER}{role}() {{ :; }}\n")),
                ],
                "entry.dorc.sh",
                role,
            ),
            LiveDefinition::Withheld
        );
    }

    /// THE REVERSE CELL — the repair is positional, not a suppression: a definition the root makes
    /// AFTER its `.` still wins, because the dependency ran first. Replayed as a root it would run
    /// LAST and shadow this.
    #[test]
    fn a_definition_after_a_sourced_dependency_stays_positionally_later() {
        let role = "wombat__is_converged";
        let live = live_at_exit(
            &[
                (
                    "entry.dorc.sh",
                    format!("{MARKER}. ./verdict.dorc.sh\n{role}() {{ :; }}\n"),
                ),
                ("verdict.dorc.sh", format!("{MARKER}{role}() {{ :; }}\n")),
            ],
            "entry.dorc.sh",
            role,
        );
        let LiveDefinition::Live(id) = live else {
            panic!("the root's own definition is live at exit, not {live:?}");
        };
        assert_eq!(
            id.file(),
            dorc_core::SourceFileId(0),
            "file 0 is the named root; file 1 is the dependency it sourced first"
        );
    }
}

#[cfg(test)]
mod snapshot_id_space_tests {
    use dorc_cli::snapshot::StaticLoadSnapshot;

    use dorc_cli::world::source_file_id;

    use super::oracle_locus;

    fn table() -> (Vec<String>, Vec<String>) {
        let snapshot = StaticLoadSnapshot::over(
            dorc_core::loadpath::Cwd::default(),
            vec!["a.oracle.sh".to_owned(), "b.oracle.sh".to_owned()],
            vec!["# a\n".to_owned(), "# b\nsecond\n".to_owned()],
            &dorc_cli::snapshot::LoadPositions::roots_only(),
            "webhost.sh",
            "# book\n",
        );
        (
            snapshot.source_paths().to_vec(),
            snapshot.source_srcs().to_vec(),
        )
    }

    /// The load-order property the whole space rests on (`28K` §2a): oracles keep their positions,
    /// so joining the book to the id space cannot move a span any already-minted id points at.
    /// A future edit that sorted the book first would silently re-point every threaded oracle
    /// span at the wrong file — the failure this pins is unreadable from any golden.
    #[test]
    fn oracle_ids_are_unmoved_by_the_book_joining_the_space() {
        let (paths, srcs) = table();
        assert_eq!(paths[0], "a.oracle.sh");
        assert_eq!(paths[1], "b.oracle.sh");
        assert_eq!(srcs[0], "# a\n");
        assert_eq!(srcs[1], "# b\nsecond\n");
    }

    /// The book sorts last, which is also the order the function environment reads (CLI files are
    /// the ambient prefix, the book's text executes after), so an id comparison IS a load-order
    /// comparison.
    #[test]
    fn the_book_sorts_after_every_oracle() {
        let (paths, srcs) = table();
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[2], "webhost.sh");
        assert_eq!(srcs[2], "# book\n");
    }

    /// A span threaded with the BOOK's id now resolves to a real locus. Before the space was
    /// widened the book had no id at all, so a book-sited definition — which `28K` makes
    /// first-class — could not be cited by the shadow refusal or by pinned-definition attribution.
    #[test]
    fn a_book_sited_span_resolves_to_a_locus() {
        let (paths, srcs) = table();
        let second_line = dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1));
        assert_eq!(
            oracle_locus(Some((second_line, source_file_id(2))), &paths, &srcs),
            Some("webhost.sh:1".to_owned())
        );
        assert_eq!(
            oracle_locus(Some((second_line, source_file_id(9))), &paths, &srcs),
            None
        );
    }
}

/// The gate-5 disposition tag for a [`dorc_plan::Disposition`] — `run`/`replace`/`omit`.
/// gate-5 asserts the bare-book argv-echo ONLY for `run` sites: a `replace`d or `omit`ted
/// site is deliberately not in the apply run-set, and a guarded omit may be absent from the
/// BARE book too (a preceding guard short-circuits it), so it must not be asserted ⊆ the
/// log (task-O / strain-D3b-fold-vs-gate5).
/// Ship an already-rendered plan to a host and report how it ended.
///
/// The one path in this binary that runs a mutating artifact somewhere. Three properties are
/// load-bearing and each is visible in the shape below: the artifact arrives already rendered
/// (nothing here can build one); it is shipped exactly ONCE, with no loop and no retry parameter
/// (`law-no-double-apply`); and a lost session reports Unknown rather than a failure, because the
/// remote may have done everything, nothing, or half of it.
#[expect(
    clippy::result_large_err,
    reason = "the Err is a full `Diag`, as everywhere on this once-per-process path"
)]
fn ship_consented_apply(
    sink: &mut dyn OutputSink,
    args: &Args,
    host: &str,
) -> Result<RunOutcome, Diag> {
    // `owed-no-flag-defaults-to-stdin`: the artifact is NAMED or it is nothing. `-` is stdin like
    // any other filename position; absent is refused by the parser, so the `None` arm here is
    // unreachable and says so rather than quietly re-acquiring the stream.
    let artifact = match args.plan.as_deref() {
        Some("-") => {
            let mut bytes = Vec::new();
            std::io::Read::read_to_end(&mut std::io::stdin(), &mut bytes)
                .map_err(|e| humane_read_error("plan", "<stdin>", &e))?;
            bytes
        }
        Some(path) => std::fs::read(path).map_err(|e| humane_read_error("plan", path, &e))?,
        None => {
            return Err(Diag::new_spanless_site(DiagCode::CliModeNeedsFlag(
                dorc_aid::diag::CliModeNeedsFlag {
                    mode: "dorc apply --host",
                    flag: "--plan",
                },
            )));
        }
    };
    // Before any identity is minted and any session stood up: these bytes are a file the user has
    // had in their hands and may have edited on any OS, and no parser of ours has seen them.
    if let Some(line) = transport_edge::first_carriage_return(&artifact) {
        return Err(transport_edge::crlf_refusal("the plan", line));
    }
    let destination =
        dorc_transport::HostId::new(host).map_err(|_| transport_edge::host_rejected(host))?;
    // The apply route stands up no engine, so it holds no clock from one: the reading is this
    // seat's own, taken at the process edge like every other. Dating it `None` unconditionally
    // would be a different claim — that this run had no clock — rather than a missing wire.
    let mut clock = clock_for_invocation();
    let invocation = dorc_cli::apply::apply_invocation(host, clock.now());
    let request = dorc_cli::apply::ConsentedApplyRequest {
        plan: &artifact,
        destination: &destination,
        nonce: &transport_edge::mint_nonce(),
        timeout: args.apply_timeout.map(std::time::Duration::from_secs),
        invocation: &invocation,
        limits: &dorc_receipt::limits::ReceiptLimits::V1,
        // A thin session established nothing on the far side: the destination is argv and every
        // context axis is unentered, so what it produced is controller-authored.
        standup_account: dorc_core::influence::InfluenceAccount::authored_before_contact(),
    };
    let mut ids =
        dorc_cli::receipt_edge::OsReceiptIdSource::over(dorc_cli::receipt_edge::OsEntropy);
    // The REQUIRED arm, and the only one this binary can build. A bypass is a disjoint type
    // nothing here constructs: an apply that cannot publish its intent refuses before the host is
    // contacted, which is what the pre-dispatch boundary is for.
    let edge = production_receipt_edge(args)
        .map_err(|refusal| apply_edge_refused(&refusal, dorc_cli::engine::NO_STATE_ROOT))?;
    let store = edge.state_base().to_owned();
    let mut io = dorc_cli::durable::NativeIo::new();
    let mut generator = dorc_cli::durable::OsKeysetGenerator::over(dorc_cli::durable::OsKeyEntropy);
    let open = edge
        .open_for_write(&mut io, &mut generator)
        .map_err(|refusal| apply_edge_refused(&refusal, &store))?;
    let mut order = dorc_cli::receipt_edge::RunClockOrder::of(&mut clock);
    let signer = open.keys().signer();
    let sealer = open.keys().encryption().sealer();
    let mut placement = open.placement(&mut io);
    // The driver is built only once the durable edge is open. It opens nothing by itself, but a
    // run that cannot record its intent should not have announced a transport either — the
    // pre-dispatch boundary is easier to read when nothing transport-shaped precedes it.
    let mut driver = transport_edge::driver_for_invocation(
        args.connect_timeout,
        args.accept_new,
        args.ssh_config.as_deref(),
    );
    let reached = dorc_cli::apply::consented_apply(
        &request,
        &mut ids,
        dorc_cli::apply::ApplyAuthorization::RequiredPublication(
            dorc_cli::apply::ApplyPublishingCapabilities::of(
                &mut order,
                signer,
                &mut placement,
                &sealer,
            ),
        ),
        driver.as_mut(),
    )
    .map_err(|refusal| apply_refused(&refusal, &store))?;

    // What the durable recorded — the published intent and outcome, and a failure past the permit —
    // is DELIBERATELY unread here. Both are user-facing surfaces with no honest driving or authoring
    // route until the in-process receipt world exists, so the reporting half of
    // `30Rs:fix-apply-durable-reporting` is deferred to `notes/30X` §11 lane C. The pre-dispatch
    // half is not: a refusal still carries the edge's own word and its store.
    Ok(report_shipment(
        sink,
        host,
        transport_edge::classify_shipment(reached.shipped),
    ))
}

/// Report one classified shipment and answer with the run's own status.
fn report_shipment(
    sink: &mut dyn OutputSink,
    host: &str,
    applied: transport_edge::AppliedOutcome,
) -> RunOutcome {
    let (diagnostic, outcome) = match applied {
        transport_edge::AppliedOutcome::Ran { status: 0 } => return RunOutcome::Complete,
        transport_edge::AppliedOutcome::Ran { status } => (
            transport_edge::apply_failed(host, status),
            RunOutcome::ApplyFailed,
        ),
        transport_edge::AppliedOutcome::Unknown { diagnosis } => (
            transport_edge::session_lost(host, 1, &diagnosis),
            RunOutcome::SessionLost,
        ),
        transport_edge::AppliedOutcome::NotAttempted(why) => (
            transport_edge::not_attempted(host, &why),
            RunOutcome::HostNotReached,
        ),
    };
    report_at(sink, true, "apply", None, &[diagnostic]);
    outcome
}

/// An apply whose local durable edge would not open, in the word for what was unavailable.
///
/// The word is the EDGE's own (`no-controller-root`, `store-not-a-directory`, a keyset state, …)
/// rather than the step everyone can already see. Rounding every route to `intent-not-published`
/// named the step and dropped the only thing a reader acts on: those repairs are in different
/// places, and one of them is not even in the operator's profile
/// (`30Rs:fix-apply-durable-reporting`).
fn apply_edge_refused(refusal: &dorc_cli::durable::EdgeRefusal, store: &str) -> Diag {
    Diag::new_spanless_site(DiagCode::ApplyPlanNotDispatchable(
        dorc_aid::diag::ApplyPlanNotDispatchable {
            reason: refusal.token(),
            store: store.to_owned(),
        },
    ))
}

/// The diagnostic for an apply that reached no dispatch, in the words of what did not close.
///
/// One code and a closed reason word rather than sibling codes: the world is one — an apply that
/// bound nothing and shipped nothing — and only the step that did not close differs.
fn apply_refused(refusal: &dorc_cli::apply::ConsentedApplyRefusal, store: &str) -> Diag {
    use dorc_cli::apply::ConsentedApplyRefusal;
    let reason = match refusal {
        ConsentedApplyRefusal::Image(_) => "image-not-recordable",
        ConsentedApplyRefusal::Preparation(_) => "session-not-preparable",
        ConsentedApplyRefusal::Publication(publication) => {
            dorc_cli::apply::publication_refusal_word(publication)
        }
    };
    Diag::new_spanless_site(DiagCode::ApplyPlanNotDispatchable(
        dorc_aid::diag::ApplyPlanNotDispatchable {
            reason,
            store: store.to_owned(),
        },
    ))
}

/// The engine-owned display word for a decline class (`27W:rul-class-starter-set`). Delegates to
/// the one home ([`dorc_aid::narrative::DeclineClass::token`]); display only
/// (`inv-referent-agnostic`; spellings ride `27V:rul-output-form-unwelded`).
#[cfg(test)]
fn decline_class_word(class: dorc_aid::narrative::DeclineClass) -> &'static str {
    class.token()
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
#[cfg(test)]
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

/// The pure half of [`emit_static_decline_notes`] — the ONE narrative-consuming render seat, split
/// out so it is assertable in-process (`289:rul-mint-hardening-package` item 4b: the "and the chain
/// renders it" clause, satisfiable today for exactly this class). Wording rides
/// `27V:rul-output-form-unwelded`.
#[cfg(test)]
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

/// Parse stdin probe-results into the site-keyed [`SiteResults`]
/// (`inv-site-keyed-results`). One line form; blank lines and `#` comments are ignored
/// (so the probe's own `# site …` provenance echo can be piped back), and any
/// unrecognized line is dropped — a site with no record folds to `Unknown` ⇒ run (the
/// `kFAIL-perform` floor; the `garbage-stdin` case pins it):
///
/// * `site <leafid> effect=<holds|absent|cant-tell> rc=<n> [stdout=<text> stderr=<text>]`
///   — the records the rendered probe emits (the return channel, 202 §3). `effect` is the
///   Effect channel mapped to a [`Verdict`] (`holds`/`absent`/`cant-tell` ⇒
///   `Converged`/`Diverged`/`Unknown`). `rc` is the raw probe-command status, carried on
///   the wire; the FIREWALL ([`facts_from_sites`]) decides whether it is fold-usable (only
///   for a valid Query-class site). A missing/garbled `rc` defaults to `Rc(0)` for
///   carriage but is irrelevant unless the firewall admits it.
///
/// `stdout=`/`stderr=` are RESERVED (`19F` §3 tuple shape): the parser accepts-and-stores
/// them (interning the text into a [`OutBytes`] on the record) but NOTHING produces them —
/// the rendered probe emits no such keys, and the consumed-stdout/stderr gate stays the
/// unconditional block it is regardless. Reserving them means a future stdout-producing
/// probe is a value-plumbing change, not a grammar change. The interner is threaded for
/// this (the `cli` is the I/O edge; `inv-determinism` exempts it).
///
/// (The transitional `declared-rc <leafid> rc=N` lane — the 19I §2 rc-injection
/// mechanism — is DEAD as of task-D2: a Query site's own `rc=` carries the fold rc now.)
#[cfg(test)]
/// The reach arm's close, both declarations (`28P` item0's mechanism at its second consumer); a
/// malformed one reads as never-closed ⇒ the footprint is refused.
fn parse_reach_end_record(rest: &str, out: &mut SiteResults) {
    let mut it = rest.split_whitespace();
    let Some(coord) = it.next() else { return };
    let (mut arm, mut count, mut body_rc) = (None, None, None);
    for tok in it {
        if let Some(a) = tok.strip_prefix("arm=") {
            arm = a.parse::<usize>().ok();
        } else if let Some(n) = tok.strip_prefix("n=") {
            count = n.parse::<u32>().ok();
        } else if let Some(r) = tok.strip_prefix("body-rc=") {
            body_rc = r.parse::<u32>().ok();
        }
    }
    if let (Some(arm), Some(count), Some(body_rc)) = (arm, count, body_rc) {
        out.reach_ends.insert(
            (coord.to_owned(), arm),
            dorc_cli::results::EmissionClose { count, body_rc },
        );
    }
}

#[cfg(test)]
fn parse_results(
    records: &[String],
    framed: bool,
    clock: &mut RunClock,
    interner: &mut Interner,
) -> SiteResults {
    let mut out = SiteResults {
        framed,
        ..SiteResults::default()
    };
    for (idx, line) in records.iter().enumerate() {
        let line = line.as_str();
        let Some((tag, rest)) = line.split_once(' ') else {
            continue; // a bare tag with no body ⇒ drop (⇒ Unknown ⇒ run)
        };
        let stamp = dorc_core::ProbeStamp::received(idx as u64, clock.now());
        match tag {
            // 24E §5: `deriv <leafid> coord=<coord>` — `coord=` is the FREE-CONTENT field
            // (`262` §2 last-to-token): after deframing it runs to end-of-line, whitespace
            // included (the incumbent whitespace-truncation bug is FIXED here). A malformed
            // leaf ⇒ drop (empty derived footprint ⇒ wall, kFAIL-safe).
            "deriv" => {
                if let Some((site, coord)) = split_key(rest, "coord=")
                    && let Some(site) = parse_leaf(site)
                {
                    out.derivations
                        .entry(site)
                        .or_default()
                        .push(coord.to_owned());
                }
            }
            // The at-most family close, BOTH declarations (`262` §2 / `26A` stop-1 +
            // `28P:dec-whole-body-atomic-refusal`); a malformed one reads as never-closed.
            "deriv-end" => {
                let mut it = rest.split_whitespace();
                if let Some(site) = it.next().and_then(parse_leaf) {
                    let mut count = None;
                    let mut body_rc = None;
                    for tok in it {
                        if let Some(n) = tok.strip_prefix("n=") {
                            count = n.parse::<u32>().ok();
                        } else if let Some(r) = tok.strip_prefix("body-rc=") {
                            body_rc = r.parse::<u32>().ok();
                        }
                    }
                    if let (Some(count), Some(body_rc)) = (count, body_rc) {
                        out.derivation_ends
                            .insert(site, dorc_cli::results::EmissionClose { count, body_rc });
                    }
                }
            }
            // 24F §3: `resolv <coord> canon=<canonical>` | `resolv <coord> dangling`. `canon=`
            // is the FREE-CONTENT field (last-to-token) so a space-bearing canonical survives.
            "resolv" => {
                if let Some((coord, tail)) = rest.split_once(' ') {
                    let outcome = if tail == "dangling" {
                        Some(ResolvOutcome::Dangling)
                    } else {
                        tail.strip_prefix("canon=")
                            .map(|c| ResolvOutcome::Canonical(c.to_owned()))
                    };
                    if let Some(o) = outcome {
                        out.resolutions.insert(coord.to_owned(), o);
                    }
                }
            }
            // 24G §4: `reach <coord> arm=<n> entity=<line>` — `entity=` is the FREE-CONTENT
            // field (last-to-token): a reached entity with embedded spaces now survives (the
            // single-token truncation is fixed — `279f` rider generalization).
            "reach" => {
                if let Some((head, entity)) = split_key(rest, "entity=") {
                    let mut coord: Option<&str> = None;
                    let mut arm: Option<usize> = None;
                    for (i, tok) in head.split_whitespace().enumerate() {
                        if i == 0 {
                            coord = Some(tok);
                        } else if let Some(n) =
                            tok.strip_prefix("arm=").and_then(|n| n.parse().ok())
                        {
                            arm = Some(n);
                        }
                    }
                    if let (Some(c), Some(a)) = (coord, arm) {
                        out.reaches
                            .entry((c.to_owned(), a))
                            .or_default()
                            .push(entity.to_owned());
                    }
                }
            }
            "reach-end" => parse_reach_end_record(rest, &mut out),
            "site" => parse_site_record(rest, stamp, &mut out, interner),
            "report" => parse_report_record(rest, &mut out), // `27W` §2 tier-3 (decision-inert lane)
            _ => {} // unrecognized inner tag ⇒ drop (kFAIL-perform: no verdict ⇒ run)
        }
    }
    out
}

/// The unloaded-sibling-oracle hint (`AID-NEEDS:aid-unloaded-sibling-oracle`, gap-5 / `24H`
/// ack-6): scan the directories of the loaded oracles + the book(s) for `*.oracle.sh` files that were
/// NOT loaded, and disclose them (suggest, never auto-load). A cli-edge disclosure — it reads the
/// filesystem, so it lives here, never in the kernel; the `read_dir` order is OS-dependent, so the
/// result is SORTED (`inv-determinism` at the edge). The payload's `detail` carries the DATA (the
/// sorted backtick-quoted path list); the user-facing framing prose stays `[unwritten:]` for the
/// conductor (`27V:rul-error-authorship-tier` — the builder authors no user-facing prose).
fn unloaded_sibling_oracle_diagnostics(book: Option<&str>, oracle_paths: &[String]) -> Vec<Diag> {
    use std::path::Path;
    let norm = |p: &str| p.replace('\\', "/");
    let mut dirs: BTreeSet<std::path::PathBuf> = BTreeSet::new();
    for p in oracle_paths.iter().map(String::as_str).chain(book) {
        if let Some(parent) = Path::new(p).parent() {
            // An empty parent (a bare filename) means the current directory.
            let dir = if parent.as_os_str().is_empty() {
                Path::new(".").to_path_buf()
            } else {
                parent.to_path_buf()
            };
            dirs.insert(dir);
        }
    }
    let mut discovered = Vec::new();
    for dir in &dirs {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let shown = norm(&entry.path().to_string_lossy());
            if shown.ends_with(".oracle.sh") {
                discovered.push(shown);
            }
        }
    }
    dorc_cli::unloaded_sibling_oracle_diagnostics(oracle_paths, &discovered)
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
    // params_of resolves no interned handle at HEAD, so a default interner suffices (`27V`).
    let interner = Interner::default();
    for d in diags {
        let (filename, src) = source.unwrap_or(("", ""));
        let parts = dorc_aid::diag::render_staged_cli_parts(
            stage,
            &render_ctx(),
            d,
            src,
            filename,
            &interner,
        );
        emit_diagnostic(sink, stage, d, parts);
    }
}

fn emit_diagnostic(
    sink: &mut dyn OutputSink,
    stage: &str,
    diag: &Diag,
    parts: dorc_aid::tagged::RenderParts,
) {
    sink.emit(OutputEvent::diagnostic(stage, diag.clone(), parts));
    sink.flush(OutputChannel::Stderr);
}

/// The (severity word, [`anstyle::Style`]) for a diagnostic (ack-5 — color as the severity/tier
/// channel): red error, yellow warning, cyan note. The style is rendered to ANSI only on a tty
/// (the [`anstream::AutoStream`] in [`report`] strips it otherwise), so the word text is what the
/// e2e needle-matching sees.
fn severity_style(severity: Severity) -> (&'static str, anstyle::Style) {
    use anstyle::AnsiColor;
    match severity {
        Severity::Error => (
            "error",
            anstyle::Style::new()
                .fg_color(Some(AnsiColor::Red.into()))
                .bold(),
        ),
        Severity::Warning => (
            "warning",
            anstyle::Style::new().fg_color(Some(AnsiColor::Yellow.into())),
        ),
        Severity::Note => (
            "note",
            anstyle::Style::new().fg_color(Some(AnsiColor::Cyan.into())),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_cli::oracle_path_key;
    use dorc_core::{EntityRef, FactKey, Interner, KindId, OpaqueToken, SelectorId};
    use dorc_plan::{LeafId, ProbePlan, ProbePredict, ProbeSiteKind};
    #[test]
    fn run_contains_no_direct_output_writes() {
        let source = include_str!("main.rs");
        let start = source.find("fn run(").expect("run function");
        let end = source[start..]
            .find("\nfn ")
            .map_or(source.len(), |offset| start + offset);
        let run_source = &source[start..end];
        for needle in [
            "print!(",
            "println!(",
            "eprint!(",
            "eprintln!(",
            "std::io::stdout()",
            "std::io::stderr()",
            ".flush()",
        ] {
            assert!(!run_source.contains(needle), "direct run output: {needle}");
        }
    }

    /// Route an unframed record string through the PRODUCTION deframer (legacy path) into the
    /// inner parser — the exact pipeline the round-trip uses. Unframed input exercises the
    /// legacy passthrough; the framed contract is pinned separately (deframe unit tests + DST).
    fn parse_str(input: &str, interner: &mut Interner) -> SiteResults {
        let expect = dorc_plan::records::Framing::spike(String::new()).expect();
        let d =
            dorc_plan::records::deframe(input, &expect, dorc_plan::records::LegacyPolicy::Tolerate);
        parse_results(&d.records, d.framed, &mut RunClock::Absent, interner)
    }

    /// The two destination answers that do not depend on the environment. `--no-receipt` must win
    /// over an explicitly named directory: a refusal the admin typed is the one instruction in this
    /// family that nothing may override (`28D:pay-levers-are-subtractive` — the levers only ever
    /// REMOVE, and a subtractive control that a sibling flag can defeat is not one).
    #[test]
    fn chrome_line_events_keep_one_immutable_newline() {
        let events = [
            OutputEvent::plain_tagged(
                OutputChannel::Stderr,
                chrome_parts("cli-why-pointer-line", &["book.sh"]),
            ),
            OutputEvent::plain_tagged(
                OutputChannel::Stderr,
                chrome_parts("cli-plan-summary-line", &["1", "2", "3", "4", "5", "6"]),
            ),
            OutputEvent::plain_tagged(
                OutputChannel::Stderr,
                chrome_parts("cli-decision-digest-line", &["digest"]),
            ),
        ];

        for event in events {
            let text = event.text();
            assert!(text.ends_with('\n'));
            assert!(!text[..text.len() - 1].ends_with('\n'));
            let parts = event.tagged_parts().expect("tagged chrome line");
            assert!(matches!(
                parts.parts().last(),
                Some(dorc_aid::tagged::RenderPart::Arrangement { text, slug })
                    if text == "\n" && *slug == "cli-chrome-line-ending"
            ));
            assert!(parts.parts().len() > 1);
        }
    }

    /// `289:rider-sibling-note-false-fires-relative`: the loaded `-o` spelling and the `read_dir`
    /// spelling of ONE file must key alike, or the unloaded-sibling hint accuses every relatively
    /// named oracle of being unloaded. The bare-name/dot-slash pair is the exact shape every
    /// in-corpus case drives (`-o firewall.oracle.sh` against a `read_dir(".")` walk), so it is the
    /// pair worth pinning; the sub-directory rows guard against a fix that only special-cases `.`.
    #[test]
    fn loaded_and_discovered_oracle_spellings_share_one_key() {
        assert_eq!(
            oracle_path_key("firewall.oracle.sh"),
            oracle_path_key("./firewall.oracle.sh"),
            "a bare -o name and its read_dir(\".\") path are one file"
        );
        assert_eq!(
            oracle_path_key("oracles/fw.oracle.sh"),
            oracle_path_key("oracles\\fw.oracle.sh"),
            "separators normalize, so a Windows walk matches a forward-slash arg"
        );
        // The three spellings must land on ONE key, not merely agree pairwise: the bug this replaced
        // folded separators AFTER reading components, so on Unix the backslash spelling grew a `./`
        // prefix its forward-slash twin never had, and only the platform that splits `\` was green.
        assert_eq!(
            oracle_path_key("oracles\\fw.oracle.sh"),
            oracle_path_key("./oracles/fw.oracle.sh"),
            "a leading `.` is dropped at any depth, on either platform"
        );
        assert_ne!(
            oracle_path_key("a/fw.oracle.sh"),
            oracle_path_key("b/fw.oracle.sh"),
            "same basename in different dirs stays distinct — the hint must still fire"
        );
    }

    /// cheap-7: the firehose-suppression classifier. Assignments and pure/no-target-state builtins
    /// (the engine's own list) are structurally-unprobeable ⇒ suppressed; a real un-oracled command
    /// is NOT ⇒ it survives into the aggregate disclosure / the `dorc why` problem set.
    #[test]
    fn structurally_unprobeable_suppresses_assignments_and_pure_builtins() {
        assert!(
            is_structurally_unprobeable("pkg=nginx"),
            "a bare assignment"
        );
        assert!(is_structurally_unprobeable("set -eu"), "the `set` builtin");
        assert!(is_structurally_unprobeable(": harmless"), "the `:` builtin");
        assert!(
            is_structurally_unprobeable("echo hello"),
            "the `echo` builtin"
        );
        assert!(is_structurally_unprobeable("cd /tmp"), "the `cd` builtin");
        assert!(
            !is_structurally_unprobeable("make install"),
            "a real un-oracled command is NOT inert"
        );
        assert!(
            !is_structurally_unprobeable("apt-get install -y nginx"),
            "a real mutator is NOT inert (it is a genuine unprobed run worth disclosing)"
        );
        // A word that merely CONTAINS `=` past a non-name char is not an assignment.
        assert!(
            !is_structurally_unprobeable("./configure --prefix=/usr"),
            "a flag `=` is not an assignment"
        );
    }

    fn pkg(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("installed")),
            context: dorc_core::Context::HostDefault,
        }
    }

    /// `289:rul-touches-mismatch-own-lane` — the synthesized per-arm wrapper takes its ROLE segment
    /// from the shared suffix constant, so the emitted sh namespace cannot drift from the role's
    /// real spelling the way the derivation lane's `__touches`/`__disturbs` pair did. Kind-munge and
    /// arm index ride around it unchanged.
    #[test]
    fn reach_arm_wrapper_name_carries_the_role_suffix() {
        assert_eq!(
            reach_arm_fn_name("sm.dorc.Package", 0),
            "sm_dorc_Package__disturbance_reaches_only_0"
        );
        assert!(
            reach_arm_fn_name("package", 3)
                .contains(dorc_oracle::reaches::DISTURBANCE_REACHES_ONLY_SUFFIX),
            "the wrapper name is built from the shared constant, never a literal"
        );
    }

    /// 24F §3 / corr-kind-keying §10: the resolver confusability enforcement. A clean single
    /// `<kind>.resolve()` is resolver-bearing; two files declaring ONE kind's resolver REFUSE both
    /// (the kind keeps token-equality — never first-wins-silently); a resolver keyed to a known
    /// PROVIDER name is KEPT but flagged (the mis-key).
    ///
    /// The conflict's DIAGNOSTIC is asserted here as a returned value, keyed to the first declaring
    /// file. It used to be asserted only end-to-end through the binary, which meant this test drove
    /// a helper that printed it: a red caret frame across a green run, and a diagnostic whose
    /// presence nothing local pinned. Both are the same bug — a decision-carrying helper writing to
    /// fd 2 (`io-at-edges-only`).
    #[test]
    fn resolver_confusability_conflict_refuses_both_collision_keeps() {
        let mut i = Interner::default();
        // A provider "apt-get" exists (a lifted disturbs provider) — a resolver whose kind munges to
        // it collides (in NAME space, `flag-forward-munge-keying`).
        let touches_src = "apt_get__disturbs() { printf 'package:%s\\n' \"$1\"; }";
        let touches_paired = vec![(
            touches_src,
            dorc_oracle::touches::TouchesSet::lift(&mut i, touches_src).value,
        )];
        let checks: Vec<dorc_oracle::predict::PredictSet> = vec![];

        // The RAW coordinate kinds present — a resolver is "bearing" only if a coord of its kind
        // exists (flag-forward-munge-keying: the map is re-keyed from the munged base to raw kinds).
        let coord_kinds: BTreeSet<Symbol> = [i.intern("package"), i.intern("apt_get")]
            .into_iter()
            .collect();

        // A clean single package resolver ⇒ resolver-bearing (kind-keyed by the munged base).
        let clean = vec!["package__resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr = build_kind_resolvers(&clean, &checks, &touches_paired, &coord_kinds, &mut i);
        assert!(
            kr.value.resolver_kinds().any(|k| i.resolve(k) == "package"),
            "a clean package resolver is resolver-bearing"
        );
        assert!(
            kr.lift.is_empty() && kr.confusability.is_empty(),
            "a clean resolver raises nothing"
        );

        // Two files, both package resolvers ⇒ BOTH refused (no resolver kind).
        let dup = vec![
            "package__resolve() { printf '%s\\n' \"$1\"; }".to_string(),
            "package__resolve() { printf '%s\\n' \"$1\"; }".to_string(),
        ];
        let kr_dup = build_kind_resolvers(&dup, &checks, &touches_paired, &coord_kinds, &mut i);
        assert_eq!(
            kr_dup.value.resolver_kinds().count(),
            0,
            "a duplicate resolver for one kind refuses BOTH (token-equality floor)"
        );
        let dup_diags = kr_dup
            .confusability
            .get(&0)
            .expect("the conflict frames into the FIRST declaring file");
        assert!(
            matches!(dup_diags.as_slice(), [d] if matches!(&d.code, DiagCode::ResolverConflict(c) if c.kind == "package" && c.count == 2)),
            "the refusal comes back as a diagnostic VALUE, never a write to fd 2"
        );

        // A resolver whose kind munges to the known provider "apt-get" (base `apt_get`) ⇒ KEPT
        // (warned, not a silent dud) — the collision is now detected in NAME space, and a raw
        // `apt_get` coord kind re-keys it as bearing.
        let collide = vec!["apt_get__resolve() { printf '%s\\n' \"$1\"; }".to_string()];
        let kr_col = build_kind_resolvers(&collide, &checks, &touches_paired, &coord_kinds, &mut i);
        assert!(
            kr_col
                .value
                .resolver_kinds()
                .any(|k| i.resolve(k) == "apt_get"),
            "a provider-named resolver is kept (the collision is a warning, not a refusal)"
        );
        assert!(
            matches!(
                kr_col.confusability.get(&0).map(Vec::as_slice),
                Some([d]) if matches!(&d.code, DiagCode::ResolverProviderCollision(c) if c.name == "apt_get")
            ),
            "the mis-key comes back as a diagnostic VALUE too"
        );
    }

    fn tool(i: &mut Interner, e: &str) -> FactKey {
        FactKey {
            kind: KindId(i.intern("tool")),
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector: SelectorId(i.intern("present")),
            context: dorc_core::Context::HostDefault,
        }
    }

    /// pin-partial-deriv-demotes-to-wall (`262` §2 / `26A` stop-1 — THE safety inversion): a
    /// deriv family is an AT-MOST claim; a mid-stream cut (byte tier) SHRINKS it, which would
    /// license MORE survivals (the under-execution cardinal sin). So an incomplete family MUST
    /// fold WALL-TOTAL (no footprint), never a shrunken footprint. Driven through the PRODUCTION
    /// deframer: a framed deriv stream is cut, deframed, parsed, and merged — asserting the
    /// site's footprint is refused (walls) on any cut, and present only when complete.
    #[test]
    fn pin_partial_deriv_family_demotes_to_wall_total() {
        use dorc_analysis::cfg::CfgNodeId;
        use dorc_plan::records::{DEFAULT_NONCE, TERMINAL_TOKEN};

        // A framed deriv stream for site 5: coords + an OPTIONAL `deriv-end N n=<end> body-rc=<R>`.
        let framed_rc = |coords: usize, end: Option<usize>, body_rc: u32| -> String {
            let coord_recs = (0..coords)
                .map(|c| format!("{DEFAULT_NONCE} deriv 5 coord=package:pkg{c} {TERMINAL_TOKEN}\n"))
                .collect::<Vec<_>>()
                .concat();
            let end_rec = end.map_or(String::new(), |n| {
                format!("{DEFAULT_NONCE} deriv-end 5 n={n} body-rc={body_rc} {TERMINAL_TOKEN}\n")
            });
            format!(
                "dorc-records/1 nonce={DEFAULT_NONCE} attempt=1 host=localhost book=bk sites=0 {TERMINAL_TOKEN}\n\
                 {coord_recs}{end_rec}dorc-records-end/1 nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n"
            )
        };

        let merged_contains = |stream: &str, i: &mut Interner| -> bool {
            let d = dorc_plan::records::deframe(
                stream,
                &dorc_plan::records::Framing::spike("bk".to_owned()).expect(),
                dorc_plan::records::LegacyPolicy::Refuse,
            );
            let results = parse_results(&d.records, d.framed, &mut RunClock::Absent, i);
            let derivations = dorc_plan::DerivationPlan {
                derivations: vec![dorc_plan::ProbeDerivation {
                    site: LeafId(5),
                    node: CfgNodeId(5),
                    provider: i.intern("apt-get"),
                    argv: vec![],
                    sh: "apt_get__disturbs() { :; }".to_string(),
                    call: "apt-manifest".to_string(),
                }],
            };
            let mut fps = dorc_plan::TrustedFootprints::new();
            let node_spans = BTreeMap::from([(
                CfgNodeId(5),
                dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
            )]);
            drop(merge_derived_footprints(
                &mut fps,
                &derivations,
                &results,
                &[],
                &BTreeMap::new(),
                &node_spans,
                i,
            ));
            fps.contains(CfgNodeId(5))
        };

        let framed = |coords: usize, end: Option<usize>| framed_rc(coords, end, 0);

        let mut i = Interner::default();
        // COMPLETE (2 coords, deriv-end n=2) ⇒ footprint lifted (the site can spare).
        assert!(
            merged_contains(&framed(2, Some(2)), &mut i),
            "a complete family lifts its footprint"
        );
        // CUT: coord dropped mid-stream (only 1 coord, but the family declared n=2) ⇒ WALL.
        assert!(
            !merged_contains(&framed(1, Some(2)), &mut i),
            "count mismatch ⇒ wall-total, never a shrunken footprint"
        );
        // CUT: the deriv-end close-record itself lost ⇒ the family never closed ⇒ WALL.
        assert!(
            !merged_contains(&framed(2, None), &mut i),
            "no deriv-end ⇒ wall-total"
        );
        // EMPTY-but-complete: the shipped body printed nothing (a genuinely absent oracle 127s
        // under PATH=mocks-only; so does a def↔invocation name disagreement). Silence is NOT an
        // empty at-most claim — an empty footprint would spare EVERYTHING. It must wall.
        assert!(
            !merged_contains(&framed(0, Some(0)), &mut i),
            "an empty family walls — the engine never manufactures a claim from silence"
        );
        // BODY DEATH (`28P:dec-whole-body-atomic-refusal`): the cell one line up proves the count
        // gate ACCEPTS these exact bytes, so only `body-rc` stands between them and a spared cell.
        assert!(
            !merged_contains(&framed_rc(2, Some(2), 127), &mut i),
            "an abnormally-terminated emission body walls the site TOTAL, however well its record \
             stream agrees with itself"
        );
    }

    /// The reach lane's twin of the pin above (`28P:dec-reach-expansion-refuses-whole-footprint`),
    /// over one stream builder so both gates are asserted on identical bytes.
    ///
    /// The inversion is the same one arrived at from the opposite side: a `disturbance_reaches_only`
    /// survey is complete-by-contract and its expansion WIDENS an at-most footprint, so an arm that
    /// cannot show it finished leaves the claim wrongly NARROW, and narrow SPARES MORE. The retired
    /// reading — that a silent arm is the honest un-expanded floor — is what
    /// `an-kind-reach`'s "widens claims only" row licensed, and it holds only while the `disturbs`
    /// claim is independently total, which is when a `reaches_only` is not wanted at all.
    #[test]
    fn pin_reach_arm_atomicity_refuses_the_whole_footprint() {
        use dorc_analysis::cfg::CfgNodeId;
        use dorc_plan::records::{DEFAULT_NONCE, TERMINAL_TOKEN};

        let coord = "sm.dorc.Package:nginx";
        let framed = |entities: usize, close: Option<(usize, u32)>| -> String {
            let recs = (0..entities)
                .map(|e| {
                    format!(
                        "{DEFAULT_NONCE} reach {coord} arm=0 entity=/etc/f{e}.conf {TERMINAL_TOKEN}\n"
                    )
                })
                .collect::<Vec<_>>()
                .concat();
            let close = close.map_or(String::new(), |(n, body_rc)| {
                format!(
                    "{DEFAULT_NONCE} reach-end {coord} arm=0 n={n} body-rc={body_rc} {TERMINAL_TOKEN}\n"
                )
            });
            format!(
                "dorc-records/1 nonce={DEFAULT_NONCE} attempt=1 host=localhost book=bk sites=0 {TERMINAL_TOKEN}\n\
                 {recs}{close}dorc-records-end/1 nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n"
            )
        };

        // Survives ⇒ its wall can still spare a disjoint downstream cell; absent ⇒ wall-total.
        let footprint_survives = |stream: &str| -> bool {
            let mut i = Interner::default();
            let src = "sm_dorc_Package__disturbance_reaches_only() {\n   \
                       dpkg -L \"$1\"    : disturbs sm.dorc.File\n}"
                .to_string();
            let kind = i.intern("sm.dorc.Package");
            let coord_kinds: BTreeSet<Symbol> = [kind].into_iter().collect();
            let reaches = build_kind_reaches(&[src], &[], &[], &coord_kinds, &mut i).value;
            let reach_kinds: BTreeSet<Symbol> = reaches.reach_kinds().collect();
            assert!(
                reach_kinds.contains(&kind),
                "the fixture kind must be reach-bearing, or the cells below are vacuous"
            );
            let d = dorc_plan::records::deframe(
                stream,
                &dorc_plan::records::Framing::spike("bk".to_owned()).expect(),
                dorc_plan::records::LegacyPolicy::Refuse,
            );
            let results = parse_results(&d.records, d.framed, &mut RunClock::Absent, &mut i);
            let provider = i.intern("hork");
            let entity = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
            let fp = dorc_plan::Footprint::authored(
                provider,
                vec![dorc_plan::EntityCoord::new(KindId(kind), entity)],
            )
            .expect("a one-coordinate footprint is non-empty");
            let mut fps = dorc_plan::TrustedFootprints::new();
            fps.insert(CfgNodeId(3), fp);
            let node_spans = BTreeMap::from([(
                CfgNodeId(3),
                dorc_core::Span::new(dorc_core::BytePos(0), dorc_core::BytePos(1)),
            )]);
            drop(expand_footprints_via_reaches(
                &mut fps,
                &reaches,
                &reach_kinds,
                &results,
                &node_spans,
                &mut i,
            ));
            fps.contains(CfgNodeId(3))
        };

        assert!(
            footprint_survives(&framed(1, Some((1, 0)))),
            "a complete arm expands the footprint and leaves it standing"
        );
        assert!(
            !footprint_survives(&framed(1, Some((1, 127)))),
            "a dead arm body refuses the WHOLE footprint, however well its stream agrees with itself"
        );
        assert!(
            !footprint_survives(&framed(1, Some((2, 0)))),
            "a cut stream refuses the whole footprint, never a partly-widened one"
        );
        assert!(
            !footprint_survives(&framed(1, None)),
            "an arm that never closed refuses the whole footprint"
        );
    }

    /// A no-member record key (the common single-fact site, `site N`).
    fn rk(n: u32) -> RecordKey {
        RecordKey {
            site: LeafId(n),
            member: None,
        }
    }

    /// A one-check probe over `fact` with the given site-kind (the firewall input).
    fn probe1(fact: FactKey, site_kind: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![ProbePredict {
                site: LeafId(0),
                member: None,
                fact,
                site_kind,
                provider: fact.kind.0,
                argv: vec![],
                sh: "{ :; }".to_string(),
                defining_span: None,
                connected: None,
                verdict: false,
                emits_report: false,
                entry: None,
            }],
            unresolvable: vec![],
            unresolvable_causes: BTreeMap::new(),
        }
    }

    #[test]
    fn report_lane_ingests_recognized_declines_and_retains_noise() {
        use dorc_aid::narrative::DeclineClass;
        let mut i = Interner::default();
        let r = parse_str(
            "report site=5 decline unsound vm.drop_caches is a write-only trigger key\n\
             report decline bogusclass some free text\n\
             report totally freeform author noise\n\
             report site=5 decline unsound vm.drop_caches is a write-only trigger key\n",
            &mut i,
        );
        assert_eq!(r.reports.len(), 3, "the exact duplicate is deduped");
        assert!(
            r.reports[0].recognized
                && r.reports[0].class == Some(DeclineClass::Unsound)
                && r.reports[0].site == Some(rk(5)),
            "a recognized decline is classed + site-keyed: {:?}",
            r.reports[0]
        );
        assert!(
            !r.reports[1].recognized && r.reports[1].class.is_none(),
            "an unknown class ⇒ degrade-generic, retained (never dropped)"
        );
        assert!(
            !r.reports[2].recognized,
            "a free-form line (no `decline` verb) is retained, never dropped"
        );
    }

    #[test]
    fn pairing_folds_a_runtime_record_into_its_site_decline_and_tier2_wins() {
        // C3 (`27W` §3): a runtime record classes a site's previously-unread decline (static wins
        // on a populated reason; a non-matching site is untouched).
        use dorc_aid::narrative::{
            AuthoredReason, CollapseKind, DeclineClass, DeclineGate, MintSpan,
        };
        let sid = |leaf: u32| dorc_aid::diag::SiteId {
            leaf: LeafId(leaf),
            member: None,
        };
        let decline = |leaf: u32, reason: Option<AuthoredReason>| {
            CollapseNarrative::new(
                SpeechAct::Vouched,
                CollapseKind::VerdictDecline {
                    site: sid(leaf),
                    arm: MintSpan(dorc_core::Span::new(
                        dorc_core::BytePos(4),
                        dorc_core::BytePos(9),
                    )),
                    arm_file: dorc_core::SourceFileId(0),
                    gate: DeclineGate::Return,
                    authored_reason: reason,
                },
            )
        };
        let tier2 = AuthoredReason {
            class: DeclineClass::Hazard,
            arm: MintSpan(dorc_core::Span::new(
                dorc_core::BytePos(1),
                dorc_core::BytePos(2),
            )),
            arm_file: dorc_core::SourceFileId(0),
        };
        let evidence = vec![
            decline(5, None), // site 5 — a dynamic-format decline (class unread statically)
            decline(6, Some(tier2)), // site 6 — already tier-2 classed (static wins)
        ];
        let reports = vec![
            ReportRecord {
                site: Some(rk(5)),
                class: Some(DeclineClass::Unsound),
                raw: "decline unsound k".to_owned(),
                recognized: true,
            },
            ReportRecord {
                site: Some(rk(6)),
                class: Some(DeclineClass::Unsound),
                raw: "decline unsound k".to_owned(),
                recognized: true,
            },
        ];
        let paired = pair_authored_reasons(evidence, &reports);
        assert!(
            matches!(
                paired[0].kind(),
                CollapseKind::VerdictDecline { authored_reason: Some(r), .. }
                    if r.class == DeclineClass::Unsound
            ),
            "the runtime record classes site 5's previously-unclassed decline"
        );
        assert!(
            matches!(
                paired[1].kind(),
                CollapseKind::VerdictDecline { authored_reason: Some(r), .. }
                    if r.class == DeclineClass::Hazard
            ),
            "site 6's tier-2 static class is NOT overwritten by the runtime echo (static wins)"
        );

        // `289:rul-mint-hardening-package` item 4b, for the ONE class a render consumes today; the
        // other eight are minted and dropped (`289:seam-narrative-render-unconsumed`).
        let rendered = static_decline_notes(&paired, &[], &[]);
        assert_eq!(
            rendered.len(),
            2,
            "both classed declines reach the why lane: {rendered:?}"
        );
        assert!(
            rendered.iter().any(|line| line.contains("unsound"))
                && rendered.iter().any(|line| line.contains("hazard")),
            "each rendered line names its own decline class: {rendered:?}"
        );
        assert!(
            static_decline_notes(&[decline(7, None)], &[], &[]).is_empty(),
            "an unclassed decline narrates nothing on this lane"
        );
    }

    #[test]
    fn report_lane_sanitizes_and_caps_the_raw_tail() {
        let capped = sanitize_report_raw(&format!("decline hazard {}", "x".repeat(500)));
        assert!(
            capped.chars().count() <= REPORT_RAW_CAP + 3,
            "capped at REPORT_RAW_CAP chars (+ the three-dot ASCII ellipsis)"
        );
        assert!(capped.ends_with("..."), "an over-cap tail is ellipsized");
        assert!(
            capped.is_ascii(),
            "the truncation marker is ASCII (`rul-ascii-output-forever`): {capped:?}"
        );
        let cleaned = sanitize_report_raw("decline unsound has\ta\ttab and \u{7} bell");
        assert!(
            !cleaned.contains('\u{7}') && !cleaned.contains('\t'),
            "control bytes are neutralized (a minimal terminal-safety floor)"
        );
        // An author's tail is host-produced text on its way to a terminal, so the lane covers
        // the characters that change how their NEIGHBOURS display as well as the ones a terminal
        // acts on directly.
        let flipped = sanitize_report_raw("decline unsound \u{202e}yek regnad\u{202c} \u{feff}");
        assert!(
            !flipped.chars().any(dorc_aid::display::is_format_or_bidi),
            "bidi and zero-width format controls are neutralized too: {flipped:?}"
        );
    }

    /// An explicit root file parses, and the store that may resolve its siblings is ORTHOGONAL to
    /// it (`30R:receipt-rooted-attention-and-cli`).
    ///
    /// The pair `--receipt` / `--receipts` is one letter apart and means two unrelated things —
    /// which document, versus which store — so the thing worth pinning is that naming both is
    /// ordinary rather than a collision. The old surface refused the analogous pair, and carrying
    /// that refusal forward would have made the orthogonality unspellable.
    #[test]
    fn an_explicit_root_file_parses_beside_the_store_that_resolves_its_siblings() {
        let parsed = parse_args_from(vec![
            "why".to_owned(),
            "--receipt=run.dorc-receipt".to_owned(),
            "--receipts=durables".to_owned(),
        ])
        .expect("an explicit root beside a store parses");
        let Invocation::Analyze(args) = parsed else {
            panic!("expected analysis invocation");
        };
        assert_eq!(args.receipt_file.as_deref(), Some("run.dorc-receipt"));
        assert_eq!(args.receipts.as_deref(), Some("durables"));
        assert!(args.receipt_id.is_none() && !args.receipt_last);
    }

    /// The three ROOT selectors are mutually exclusive, in every pairing.
    ///
    /// Each names one attention root, and ranking two against each other would be inventing a
    /// preference the design refuses to have. Exhaustive over the pairs rather than one sample:
    /// the refusal is a three-arm table, and a table with one arm tested is a table with two arms
    /// nobody checked.
    #[test]
    fn the_three_root_selectors_refuse_one_another() {
        for pair in [
            ["--receipt=a".to_owned(), "--receipt-id=b".to_owned()],
            ["--receipt=a".to_owned(), "--receipt-last".to_owned()],
            ["--receipt-id=b".to_owned(), "--receipt-last".to_owned()],
        ] {
            let argv = vec!["why".to_owned(), pair[0].clone(), pair[1].clone()];
            assert!(
                parse_args_from(argv).is_err(),
                "{} and {} both name a root",
                pair[0],
                pair[1]
            );
        }
    }

    /// A remote apply REFUSES `--no-receipt`, at the parser — before the plan file, the keyset, the
    /// store, the clock, or the transport (`30R:publication-and-dispatch-boundary`).
    ///
    /// The named plan does not exist, and that is the assertion: reading it is the FIRST thing
    /// `ship_consented_apply` does, so a refusal arriving here reached no I/O. The same argv
    /// WITHOUT the flag parses, which keys the refusal to the flag rather than to the form.
    #[test]
    fn a_remote_apply_refuses_the_receipt_opt_out_before_it_reads_anything() {
        let shipping = vec![
            "apply".to_owned(),
            "--host=web1".to_owned(),
            "--plan=no-such-plan.sh".to_owned(),
        ];
        let mut declining = shipping.clone();
        declining.push("--no-receipt".to_owned());

        let refusal = parse_args_from(declining)
            .expect_err("a remote apply cannot decline the receipt that authorizes its dispatch");
        assert_eq!(refusal.code.slug(), "apply-receipt-not-optional");
        assert!(
            parse_args_from(shipping).is_ok(),
            "the same invocation without the flag is ordinary"
        );
    }

    /// A refused durable edge keeps its OWN word, and names the store it would have filed under.
    ///
    /// Every route used to round to `intent-not-published`, which dropped the only thing a reader
    /// acts on: a store standing where a directory belongs and an unusable keyset are repaired in
    /// different places (`30Rs:fix-apply-durable-reporting`). Both halves are asserted — the same
    /// word means different things at a per-user root and at a named one.
    #[test]
    fn a_refused_durable_edge_keeps_its_own_word_and_names_its_store() {
        let refusal = dorc_cli::durable::EdgeRefusal::Store(
            dorc_cli::durable::StoreOpenRefusal::NotADirectory,
        );
        let diag = apply_edge_refused(&refusal, "/state/dorc/receipts");
        assert!(
            matches!(
                &diag.code,
                DiagCode::ApplyPlanNotDispatchable(payload)
                    if payload.reason == "store-not-a-directory"
                        && payload.store == "/state/dorc/receipts"
            ),
            "the edge's own word and place survive; got: {:?}",
            diag.code
        );
    }

    /// The flag survives wherever it still means something: nothing else publishes an
    /// `ApplyIntent`, so nothing else is asking for a bypass by declining a receipt.
    ///
    /// Both cells, because neither implies the other: a remote PLAN probes read-only and mints no
    /// permit, and a local `apply` renders to stdout and contacts nothing. Refusing either would
    /// turn a narrow authority rule into a blanket ban on a subtractive lever
    /// (`28D:pay-levers-are-subtractive`).
    #[test]
    fn the_receipt_opt_out_survives_everywhere_it_still_means_something() {
        for argv in [
            vec![
                "plan".to_owned(),
                "--host=web1".to_owned(),
                "--book=book.sh".to_owned(),
                "--no-receipt".to_owned(),
            ],
            vec![
                "apply".to_owned(),
                "--book=book.sh".to_owned(),
                "--no-receipt".to_owned(),
            ],
        ] {
            let spelled = argv.join(" ");
            let Ok(Invocation::Analyze(args)) = parse_args_from(argv) else {
                panic!("`{spelled}` publishes no intent, so declining a receipt is ordinary");
            };
            assert!(args.no_receipt, "`{spelled}` keeps the admin's refusal");
        }
    }

    /// [`probe1`] but ENTRY-bearing (a wrapped-context site): the runtime-EntryFailure input.
    fn probe1_entry(fact: FactKey, site_kind: ProbeSiteKind) -> ProbePlan {
        let mut p = probe1(fact, site_kind);
        p.checks[0].entry = Some(dorc_plan::EntryComposed {
            enter_defs: vec![],
            inner_fn: "x__predict".to_string(),
            inner_sh: "x__predict() { :; }".to_string(),
            inner_argv: vec![],
        });
        p
    }

    #[test]
    fn entry_bearing_site_ge2_rc_mints_class_only_runtime_entry_failure() {
        use dorc_aid::narrative::EntryFailureTag;
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let entry_probe = probe1_entry(fact, ProbeSiteKind::Query { valid: true });
        let tag = |records: &str, i: &mut Interner| -> Option<EntryFailureTag> {
            facts_from_sites(&entry_probe, &parse_str(records, i), &BTreeMap::new())
                .1
                .into_iter()
                .find_map(|e| match e.kind() {
                    CollapseKind::EntryFailure { class, .. } => Some(*class),
                    _ => None,
                })
        };
        assert_eq!(
            tag("site 0 effect=cant-tell rc=127\n", &mut i),
            Some(EntryFailureTag::MissingDeps),
            "rc 127 ⇒ missing deps in the view"
        );
        assert_eq!(
            tag("site 0 effect=cant-tell rc=2\n", &mut i),
            Some(EntryFailureTag::InContextDecline),
            "other ≥2 ⇒ in-context decline"
        );
        assert_eq!(
            tag("site 0 effect=holds rc=0\n", &mut i),
            None,
            "an answered entry check (rc 0) is no failure"
        );
        let plain = probe1(fact, ProbeSiteKind::Query { valid: true });
        let results = parse_str("site 0 effect=cant-tell rc=127\n", &mut i);
        assert!(
            !facts_from_sites(&plain, &results, &BTreeMap::new())
                .1
                .iter()
                .any(|e| matches!(e.kind(), CollapseKind::EntryFailure { .. })),
            "a non-entry site mints no EntryFailure (the class is entry-scoped)"
        );
    }

    #[test]
    fn a_dial_forbidden_wrapped_site_mints_entry_denial_and_a_licensed_one_mints_none() {
        // `289:rul-mint-hardening-package` item 4a for `EntryDenial`, the one static entry-consent
        // class with no pin at all. Both directions over one world, varying only the admin's dial;
        // anti-masking, so the narrative is read out of the real analysis, never handed in.
        const SUDO: &str = "# dorc-lang/v0.2\n\
             sudo__predict() {\n\
             while [ \"${1#-}\" != \"$1\" ]; do case \"$1\" in -u) shift 2 ;; *) shift ;; esac; done\n\
             env -i HOME=/root \"$@\"\n\
             }\n\
             sudo__lend_map() {\n\
             printf '%s\\n' root : lends user\n\
             : lends fs-view\n\
             : lends netns\n\
             \"$@\"\n\
             }\n\
             sudo__enter() {\n\
             sudo -n \"$@\"\n\
             }\n";
        const HORK: &str = "# dorc-lang/v0.2\n\
             hork__is_converged() {\n\
             : safe-across user\n\
             case \"$1\" in\n\
             install) hork query \"$2\" ;;\n\
             *) return 2 ;;\n\
             esac\n\
             }\n";

        let rungs = |dial: dorc_core::EscalationDial| -> Vec<dorc_aid::narrative::EntryDegradeTag> {
            let mut interner = Interner::default();
            let srcs = vec![SUDO.to_owned(), HORK.to_owned()];
            let refs: Vec<&str> = srcs.iter().map(String::as_str).collect();
            let paths = vec!["sudo.oracle.sh".to_owned(), "hork.oracle.sh".to_owned()];
            let checks: Vec<_> = refs
                .iter()
                .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
                .collect();
            let verdict_sets: Vec<_> = refs
                .iter()
                .map(|src| dorc_oracle::verdict::VerdictSet::lift(&mut interner, src).value)
                .collect();
            let parsed = dorc_syntax::parse("sudo hork install frob\n");
            let cfg = dorc_analysis::cfg::build(&parsed.value).value;
            let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut interner);
            let wrapper_sets =
                WrapperSets::lift(&refs, &mut interner, &dorc_core::ContestedFamilies::none());
            build_wrapped_analysis(
                &srcs,
                &refs,
                &paths,
                &dorc_oracle::closure::HelperIndex::build(&refs, None),
                &checks,
                &verdict_sets,
                &wrapper_sets,
                &parsed.value,
                &cfg,
                &value,
                dial,
                dorc_core::Capability::Root,
                &mut interner,
                dorc_analysis::funcenv::LiveDefinitions::unsolved(),
            )
            .collapse_narrative
            .iter()
            .filter_map(|n| match n.kind() {
                CollapseKind::EntryDenial { rung } => Some(*rung),
                _ => None,
            })
            .collect()
        };

        assert_eq!(
            rungs(dorc_core::EscalationDial::NoEscalation),
            vec![dorc_aid::narrative::EntryDegradeTag::DialForbids],
            "a dial-forbidden wrapped site narrates its denial rung"
        );
        assert!(
            rungs(dorc_core::EscalationDial::VouchedOnly).is_empty(),
            "a consented entry is no collapse and mints nothing"
        );
    }

    #[test]
    fn own_wall_coord_selects_kill_coord_for_kill_establish_for_establish() {
        // 24E §7 (resid-kill-coherence): the coherence comparand is the node's OWN effect
        // coordinate — its killed cell (a kill node, from `kill_coords`) OR its establish cell (an
        // establish class). `own_wall_coord` unifies both, extending the Stage-2 establish-wall
        // check to kill-walls in BOTH the authored and derived footprint lanes.
        use dorc_analysis::cfg::CfgNodeId;
        use dorc_analysis::effect::SkipClass;
        let mut i = Interner::default();
        let killed = pkg(&mut i, "nginx"); // package:nginx@installed (a purge's killed cell)
        let established = pkg(&mut i, "curl");
        let kill_node = CfgNodeId(7);
        let est_node = CfgNodeId(3);
        let classes = vec![(est_node, SkipClass::EstablishProbeAmbient(established))];
        let mut kill_coords = BTreeMap::new();
        kill_coords.insert(kill_node, killed);
        assert_eq!(
            own_wall_coord(kill_node, &classes, &kill_coords),
            Some(dorc_plan::EntityCoord::new(killed.kind, killed.entity)),
            "a kill node's comparand is its killed coordinate (24E §7 — the new close)"
        );
        assert_eq!(
            own_wall_coord(est_node, &classes, &kill_coords),
            Some(dorc_plan::EntityCoord::new(
                established.kind,
                established.entity
            )),
            "an establish node's comparand is its establish coordinate (Stage 2, unchanged)"
        );
        assert_eq!(
            own_wall_coord(CfgNodeId(99), &classes, &kill_coords),
            None,
            "a node that is neither establish nor kill has no coherence comparand"
        );
    }

    #[test]
    fn parse_results_maps_three_outcome_and_carries_rc() {
        // The record maps holds/absent/cant-tell to the Effect verdict and carries the
        // raw rc on the wire (whether it is fold-usable is the firewall's call).
        let mut i = Interner::default();
        let r = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\nsite 2 effect=cant-tell rc=2\n",
            &mut i,
        );
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Converged)
        );
        assert_eq!(
            r.records.get(&rk(1)).map(|x| x.verdict),
            Some(Verdict::Diverged)
        );
        assert_eq!(
            r.records.get(&rk(2)).map(|x| x.verdict),
            Some(Verdict::Unknown)
        );
        assert_eq!(r.records.get(&rk(0)).map(|x| x.rc), Some(Rc(0)));
        assert_eq!(r.records.get(&rk(1)).map(|x| x.rc), Some(Rc(1)));
    }

    #[test]
    fn parse_results_drops_garbage_kfail_perform() {
        // Unrecognized / malformed lines are dropped (⇒ Unknown ⇒ run). This is the TWIN of the
        // retired `garbage-stdin` e2e case (`27D`/`24I` batch-3 de-graduation, cli-surface): it feeds
        // that case's EXACT specimen — a non-`site` leading token, a non-numeric site-id, a `site`
        // line with no `effect=`, a `declared-rc` line with a non-integer rc, and a leading-whitespace
        // garbage line. None resolve site 0's effect ⇒ it folds to Unknown ⇒ the install runs
        // (`kFAIL-perform`), and the cli never crashes on the malformed stream.
        let mut i = Interner::default();
        let r = parse_str(
            "this is not a valid result line\nsite notanumber effect=holds\n\
             site 0 garbled-no-effect-field\ndeclared-rc xyz rc=notanint\n\
             \x20\x20\x20leading-whitespace garbage\n",
            &mut i,
        );
        // `site 0 garbled-no-effect-field` parses the id but no effect= ⇒ Unknown (safe ⇒ run).
        assert_eq!(
            r.records.get(&rk(0)).map(|x| x.verdict),
            Some(Verdict::Unknown),
            "site 0 has no valid effect ⇒ Unknown ⇒ run"
        );
        // `site notanumber` ⇒ no id ⇒ dropped; the `declared-rc`, non-`site`, and whitespace-garbage
        // lines ⇒ dropped. Only the id-parseable site 0 lands (never a crash).
        assert_eq!(r.records.len(), 1, "only the id-parseable site landed");
    }

    #[test]
    fn parse_results_reserves_stdout_stderr_keys_inert() {
        // item-2 (19F §3 tuple shape): the `stdout=`/`stderr=` keys are RESERVED — the
        // parser accepts-and-stores them into the record's tuple, but they produce no
        // behavior change. Pin BOTH halves: (1) absent ⇒ the slots are `Predicted::Top`
        // (the default, the only state the probe actually emits today); (2) present ⇒
        // they intern into a `Predicted::Value(OutBytes)` and ride the tuple, while the
        // firewall + consumption gate are untouched (the consumed-stdout/stderr block is
        // unconditional, never reading the claim). Anti-masking: this asserts the SHAPE
        // exists end-to-end, NOT that a check predicts a value (nothing does this round).
        let mut i = Interner::default();
        let r = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert_eq!(
            rec.stdout,
            Predicted::Top,
            "absent stdout= ⇒ ⊤ (the live default)"
        );
        assert_eq!(
            rec.stderr,
            Predicted::Top,
            "absent stderr= ⇒ ⊤ (the live default)"
        );
        // Reserved keys parse-and-store (a future stdout-producing probe is value-plumbing).
        // `262` §2 last-to-token: `stdout=` is the TRAILING free-content field (the read-value
        // lane's carrier — `279f` rider), so `stderr=` (single-token, out of spike scope) must
        // PRECEDE it; and `stdout=`'s value runs to end-of-line so embedded spaces survive.
        let r = parse_str(
            "site 0 effect=holds rc=0 stderr=warn stdout=hello there world\n",
            &mut i,
        );
        let rec = r.records.get(&rk(0)).expect("site 0");
        assert_eq!(
            match rec.stdout {
                Predicted::Value(OutBytes(s)) => i.resolve(s),
                Predicted::Top => "<top>",
            },
            "hello there world",
            "stdout= is last-to-token: embedded spaces survive byte-exactly (279f pin)"
        );
        assert!(
            matches!(rec.stderr, Predicted::Value(OutBytes(_))),
            "a reserved stderr= is stored as a value claim: {:?}",
            rec.stderr
        );
        // The Effect/Status path is unaffected by the reserved keys' presence.
        assert_eq!(rec.verdict, Verdict::Converged);
        assert_eq!(rec.rc, Rc(0));
    }

    /// Two same-command sites on DISTINCT authored cells fold independently, and the fold does not
    /// care what order their records arrived in (`26H` §3.6).
    ///
    /// This is the behavioural close of `26G`'s cold trail. Before W-B both sites shared the
    /// per-provider auto-cell, and which of the pair kept a disposition depended on their relative
    /// POSITION (the later site read as written-upstream by its own sibling). Distinct keys remove
    /// the coupling at the root, so there is no order left to depend on — and this pins that
    /// directly rather than quoting the mechanism it retired.
    #[test]
    fn distinct_authored_cells_fold_independently_whatever_the_record_order() {
        let mut i = Interner::default();
        let file = KindId(i.intern("sm.dorc.File"));
        let content = SelectorId(i.intern("content"));
        let cell = |i: &mut Interner, path: &str| {
            FactKey::cell(
                file,
                EntityRef::Operand(OpaqueToken(i.intern(path))),
                content,
            )
        };
        let a = cell(&mut i, "/etc/a.conf");
        let b = cell(&mut i, "/etc/b.conf");
        let probe = ProbePlan {
            checks: vec![
                ProbePredict {
                    site: LeafId(0),
                    member: None,
                    fact: a,
                    site_kind: ProbeSiteKind::Establish,
                    provider: file.0,
                    argv: vec![],
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: true,
                    emits_report: false,
                    entry: None,
                },
                ProbePredict {
                    site: LeafId(1),
                    member: None,
                    fact: b,
                    site_kind: ProbeSiteKind::Establish,
                    provider: file.0,
                    argv: vec![],
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: true,
                    emits_report: false,
                    entry: None,
                },
            ],
            unresolvable: vec![],
            unresolvable_causes: BTreeMap::new(),
        };
        // The finding's own amplification world: one site converged, its sibling unreadable.
        let forward = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=cant-tell rc=3\n",
            &mut i,
        );
        let reversed = parse_str(
            "site 1 effect=cant-tell rc=3\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        let (f_facts, f_narrative, _) = facts_from_sites(&probe, &forward, &BTreeMap::new());
        let (r_facts, r_narrative, _) = facts_from_sites(&probe, &reversed, &BTreeMap::new());
        assert_eq!(
            f_facts, r_facts,
            "the fold is a pure function of the records, not of their arrival order"
        );
        assert_eq!(f_narrative.len(), r_narrative.len());
        assert_eq!(
            f_facts.get(&a).map(|o| o.effect),
            Some(Verdict::Converged),
            "the converged site keeps its own answer — a sibling that could not report is not \
             evidence about THIS cell (oracle-contract §4)"
        );
        assert_eq!(
            f_facts.get(&b).map(|o| o.effect),
            Some(Verdict::Unknown),
            "the unreadable site is unknown ⇒ its own line runs, and only its own line"
        );
        assert!(
            f_narrative.is_empty(),
            "distinct cells never collided, so nothing was merged and nothing is announced"
        );
    }

    #[test]
    fn firewall_establish_site_rc_never_becomes_fold_status() {
        // THE wrong-concrete firewall, direction 1 (202 §3 / task-D2): an ESTABLISH
        // site's record-rc is the CHECK command's rc (dpkg-query's), NOT the mutator's.
        // It must NEVER reach the fold's Status — status stays Top unconditionally,
        // even though the record carries `rc=0`.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Establish);
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(obs.effect, Verdict::Converged, "Effect = reported verdict");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an establish site's probe-rc must NOT become fold status (the disaster class)"
        );
    }

    #[test]
    fn firewall_valid_query_site_rc_feeds_fold_status() {
        // THE wrong-concrete firewall, direction 2 (task-D2): a VALID Query site's
        // record-rc IS the guard's own rc ⇒ it feeds the fold's Status exactly. This is
        // the relaxation that replaces the dead `declared-rc` lane.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Query { valid: true });
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Value(Rc(0)),
            "a valid Query guard's own rc supplies the fold Status"
        );
        // A non-zero guard rc (nginx absent) carries through identically (Exit(n) path).
        let results = parse_str("site 0 effect=absent rc=1\n", &mut i);
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .unwrap();
        assert_eq!(obs.status, Predicted::Value(Rc(1)), "rc 1 carries through");
    }

    #[test]
    fn firewall_invalid_query_site_rc_withheld() {
        // THE wrong-concrete firewall, direction 3 (rule-query-validity, 205 §2): an
        // INVALID Query site (a mutator/opaque reached it from entry) has a stale
        // resting rc ⇒ status stays Top even though the record carries `rc=0` ⇒ the
        // guard runs for real at apply. The bit is the ENGINE's (classify); the cli only
        // honors it.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Query { valid: false });
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "an INVALID Query guard's rc is stale ⇒ withheld (status Top ⇒ runs for real)"
        );
    }

    #[test]
    fn invalid_query_withhold_mints_substitution_refusal() {
        // C5 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): the invalid-Query withhold
        // mints one SubstitutionRefusal; a valid Query (substitutable rc) mints none.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);

        let invalid = probe1(fact, ProbeSiteKind::Query { valid: false });
        let (_facts, evidence, _collapsed) = facts_from_sites(&invalid, &results, &BTreeMap::new());
        assert_eq!(
            evidence
                .iter()
                .filter(|e| matches!(e.kind(), CollapseKind::SubstitutionRefusal { .. }))
                .count(),
            1,
            "an invalid Query withhold mints one SubstitutionRefusal"
        );

        let valid = probe1(fact, ProbeSiteKind::Query { valid: true });
        let (_facts, evidence, _collapsed) = facts_from_sites(&valid, &results, &BTreeMap::new());
        assert!(
            !evidence
                .iter()
                .any(|e| matches!(e.kind(), CollapseKind::SubstitutionRefusal { .. })),
            "a valid Query substitutes its rc ⇒ no refusal"
        );
    }

    #[test]
    fn probe_origins_keys_measured_receipt_by_fact_with_stream_ordinal() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe1(fact, ProbeSiteKind::Establish);
        let results = parse_str("site 0 effect=holds rc=0\n", &mut i);
        let mut arena = ProvArena::new();
        let origins = probe_origins(&probe, &results, &mut arena);
        let attribution = origins
            .get(&fact)
            .copied()
            .expect("the establish fact carries a probe-result origin");
        let node = arena
            .node(attribution.origin)
            .expect("the measured origin resolves in the arena");
        assert_eq!(
            node.kind,
            dorc_core::OriginKind::ProbeResult(dorc_core::ProbeStamp::at_ordinal(0)),
            "the ProbeResult stamp is the record's stream ordinal (site 0 = the 0th deframed record)"
        );
    }

    /// Deframe `input` under an INJECTED stepping clock, so a per-record instant is a property of
    /// the injected source rather than of the machine the test runs on.
    fn parse_str_clocked(input: &str, at: u64, step: u64, interner: &mut Interner) -> SiteResults {
        let expect = dorc_plan::records::Framing::spike(String::new()).expect();
        let d =
            dorc_plan::records::deframe(input, &expect, dorc_plan::records::LegacyPolicy::Tolerate);
        let mut clock = RunClock::Ticking {
            at: dorc_core::RunInstant(at),
            step_millis: step,
        };
        parse_results(&d.records, d.framed, &mut clock, interner)
    }

    #[test]
    fn record_observation_instants_come_from_the_injected_clock_not_wall_time() {
        // A record's instant must be EXACTLY what the injected source yielded, and a stepping
        // source must distinguish records. Wall time never enters a test's answer.
        let mut i = Interner::default();
        let results = parse_str_clocked(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            9_000,
            7,
            &mut i,
        );
        let stamps: Vec<dorc_core::ProbeStamp> =
            results.records.values().map(|r| r.stamp).collect();
        assert_eq!(
            stamps
                .iter()
                .map(|s| s.received_at)
                .collect::<Vec<Option<dorc_core::RunInstant>>>(),
            vec![
                Some(dorc_core::RunInstant(9_000)),
                Some(dorc_core::RunInstant(9_007)),
            ],
            "each record carries the instant the injected clock yielded for it, in arrival order"
        );
        let clockless = parse_str("site 0 effect=holds rc=0\n", &mut i);
        assert_eq!(
            clockless
                .records
                .values()
                .next()
                .map(|r| r.stamp.received_at),
            Some(None),
            "no clock ⇒ no instant; never RunInstant(0) masquerading as a measurement"
        );
    }

    #[test]
    fn reported_observation_carries_this_records_rc_and_its_predicts_line() {
        // Each of the three must come from THIS record/check pair: a wrong-record rc or a
        // defaulted span renders a confidently-wrong attribution.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let mut probe = probe1(fact, ProbeSiteKind::Establish);
        let span = dorc_core::Span::new(dorc_core::BytePos(40), dorc_core::BytePos(52));
        probe.checks[0].defining_span = Some((span, dorc_core::SourceFileId(3)));
        let results = parse_str_clocked("site 0 effect=holds rc=7\n", 1_234, 0, &mut i);
        let mut arena = ProvArena::new();
        let origins = probe_origins(&probe, &results, &mut arena);
        let reported = origins
            .get(&fact)
            .and_then(|a| a.reported)
            .expect("one record measured the fact ⇒ one reporting observation");
        assert_eq!(reported.tool_rc, Rc(7), "the rc is this record's, verbatim");
        assert_eq!(
            reported.predict_span,
            Some((span, dorc_core::SourceFileId(3))),
            "the reporting line is the shipped check's own defining span, file-qualified"
        );
        assert_eq!(
            reported.stamp.received_at,
            Some(dorc_core::RunInstant(1_234)),
            "the observation instant rides the same stamp the receipt origin was minted from"
        );
    }

    #[test]
    fn two_records_on_one_fact_report_no_single_observation() {
        // Two records are two events with no one speaker, instant, or rc; picking a winner would
        // fabricate a measurement. The receipt still joins both — nothing leaves the evidence plane.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_str_clocked(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            500,
            1,
            &mut i,
        );
        let mut arena = ProvArena::new();
        let attribution = probe_origins(&probe, &results, &mut arena)
            .get(&fact)
            .copied()
            .expect("the fact is still keyed");
        assert!(
            attribution.reported.is_none(),
            "two reporting records ⇒ no single reported row"
        );
        assert_eq!(
            arena
                .node(attribution.origin)
                .expect("the joined origin resolves")
                .kind,
            dorc_core::OriginKind::Join,
            "both records survive as receipts under a join"
        );
    }

    /// Two checks over the SAME fact (distinct sites) — the conflict-floor input.
    fn probe2(fact: FactKey, k0: ProbeSiteKind, k1: ProbeSiteKind) -> ProbePlan {
        ProbePlan {
            checks: vec![
                ProbePredict {
                    site: LeafId(0),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k0,
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: false,
                    emits_report: false,
                    entry: None,
                },
                ProbePredict {
                    site: LeafId(1),
                    member: None,
                    fact,
                    provider: fact.kind.0,
                    argv: vec![],
                    site_kind: k1,
                    sh: "{ :; }".to_string(),
                    defining_span: None,
                    connected: None,
                    verdict: false,
                    emits_report: false,
                    entry: None,
                },
            ],
            unresolvable: vec![],
            unresolvable_causes: BTreeMap::new(),
        }
    }

    #[test]
    fn same_cell_conflicting_records_degrade_to_top() {
        // 20I find-6a / item-5 (the conflict floor): two sites on the SAME cell whose
        // records DISAGREE merge to ⊤, never last-write-wins. Two establish sites: site 0
        // reports holds, site 1 reports absent (a self-contradicting / forged host). The
        // merged Effect must be `Unknown` (⊤) ⇒ the apply runs (kFAIL-perform), NOT the
        // last-written `absent` (or `holds`). Anti-masking: a constructed conflict, not a
        // hand-injected verdict the check should predict.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.effect,
            Verdict::Unknown,
            "disagreeing same-cell Effect verdicts degrade to ⊤ (Unknown), not last-write-wins"
        );
    }

    #[test]
    fn same_cell_agreeing_records_pass_through() {
        // The floor's other half: two same-cell sites that AGREE pass the value through
        // (no spurious ⊤). Two establish sites both reporting holds ⇒ merged Effect is
        // Converged (the agreed value), so a genuinely-converged cell still elides.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let results = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        assert_eq!(
            obs.effect,
            Verdict::Converged,
            "agreeing same-cell records keep the agreed verdict (no spurious ⊤)"
        );
    }

    #[test]
    fn same_cell_disagreement_mints_measured_narrative_agreement_mints_none() {
        // C4 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): a cross-site disagreement
        // mints one `Measured` FactMergeDisagreement; an agreement mints none.
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);

        let conflict = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let (_facts, evidence, _collapsed) = facts_from_sites(&probe, &conflict, &BTreeMap::new());
        assert_eq!(
            evidence.len(),
            1,
            "one cross-site disagreement ⇒ one record"
        );
        assert_eq!(evidence[0].tier(), SpeechAct::Measured);
        assert!(matches!(
            evidence[0].kind(),
            CollapseKind::FactMergeDisagreement { .. }
        ));

        let agree = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        let (_facts, evidence, _collapsed) = facts_from_sites(&probe, &agree, &BTreeMap::new());
        assert!(evidence.is_empty(), "agreeing records mint no disagreement");
    }

    /// SILENCE IS NOT A SECOND OPINION. Two sites on one cell where only ONE reported: the meet
    /// still ⊤s the cell — an unmeasured site contributes `Unknown` and unsure ⇒ run — but nothing
    /// was contradicted, so the `Measured` narrative must not claim the host said two things. It
    /// used to, because the fold compared every check's observable whether a record backed it or
    /// not, and the reader got an `[unnarrated: FactMergeDisagreement]` for a cell no two records
    /// ever shared.
    #[test]
    fn one_measured_site_and_one_silent_one_mint_no_disagreement() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);

        for stream in ["site 0 effect=holds rc=0\n", "site 1 effect=holds rc=0\n"] {
            let results = parse_str(stream, &mut i);
            let (facts, evidence, _collapsed) =
                facts_from_sites(&probe, &results, &BTreeMap::new());
            assert!(
                evidence.is_empty(),
                "one record and one silence is not a contradiction: {evidence:?} ({stream})"
            );
            assert_eq!(
                facts[&fact].effect,
                Verdict::Unknown,
                "the meet still ⊤s the cell, so the licence is withheld ({stream})"
            );
        }
    }

    /// The shared-cell readout is CELL-keyed, not pair-keyed (`26G:fnd-shared-auto-cell-collides`):
    /// three sites on one cell disagreeing pairwise is ONE collapse an admin can act on, and three
    /// lines would read as three unrelated problems. The count travels so the line can say how wide
    /// the collapse is.
    #[test]
    fn a_shared_cell_collapse_reports_once_however_many_sites_disagree() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let mut probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let mut third = probe.checks[0].clone();
        third.site = LeafId(2);
        probe.checks.push(third);

        let records = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\nsite 2 effect=cant-tell rc=2\n",
            &mut i,
        );
        let (_facts, _evidence, collapsed) = facts_from_sites(&probe, &records, &BTreeMap::new());
        assert_eq!(
            collapsed.len(),
            1,
            "one entry, not one per pair: {collapsed:?}"
        );
        assert_eq!(
            collapsed.get(&fact),
            Some(&3),
            "and it counts every site on the cell, including the ones that agreed"
        );
    }

    /// The ladder oracle: a query verb (`:?`) and a mutating verb (`:`) — the two halves of the
    /// idiom the validity fixpoint exists to make cascade.
    const FIXPOINT_ORACLE: &str = r#"
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"@installed
}
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
      esac
   fi
}
"#;

    const REPLACEMENT_CASCADE_ORACLE: &str = r#"
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"@installed
}
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   case $verb in
      install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
   esac
}
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 : sm.dorc.Package:"$1"@installed ;;
      *) return 2 ;;
   esac
}
"#;

    /// Drive the REAL fixpoint over a two-rung ladder at the given iteration `cap`.
    fn settle_ladder(cap: u32, records: &str) -> SettledFixpoint {
        let book = "dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha\n\
                    dpkg -s beta >/dev/null 2>&1 || apt-get install -y beta\n";
        let mut interner = Interner::default();
        let oracle_srcs = vec![FIXPOINT_ORACLE.to_owned()];
        let idx = dorc_oracle::lift(&mut interner, &[FIXPOINT_ORACLE]).value;
        let checks =
            vec![dorc_oracle::predict::lift_predicts(&mut interner, FIXPOINT_ORACLE).value];
        let verdicts = dorc_oracle::verdict::VerdictIndex::default();
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut interner);
        let mut arena = ProvArena::new();
        let peeled = BTreeMap::new();
        let frozen = FrozenModel {
            cfg: &cfg,
            value: &value,
            ast: &parsed.value,
            idx: &idx,
            checks: &checks,
            verdicts: &verdicts,
            peeled: &peeled,
            live: dorc_analysis::funcenv::LiveDefinitions::unsolved(),
        };
        let origin = classify_round(
            &frozen,
            &dorc_analysis::erase::ErasedSites::none(),
            &mut interner,
            &mut arena,
            &mut BTreeMap::new(),
            &mut BTreeMap::new(),
            &mut dorc_analysis::certify::CertifierTrip::default(),
        );
        let probe = {
            let ship = |n, p, a: &[Symbol]| {
                ship_predict_body(
                    &oracle_srcs,
                    &dorc_oracle::closure::HelperIndex::default(),
                    &checks,
                    &interner,
                    p,
                    a,
                    n,
                    dorc_analysis::funcenv::LiveDefinitions::unsolved(),
                )
            };
            dorc_plan::compile_probe(
                &parsed.value,
                &cfg,
                &value,
                &origin.classes,
                &BTreeMap::new(),
                &dorc_plan::ConnectedPipes::default(),
                ship,
                |_, _, _, _| None,
                |_, _| false,
            )
        };
        let results = parse_str(records, &mut interner);
        let vouches = dorc_plan::Vouches::new();
        let connected = dorc_plan::ConnectedPipes::default();
        let plan_inputs = dorc_plan::SettleInputs {
            src: book,
            ast: &parsed.value,
            cfg: &cfg,
            vouches: &vouches,
            verdict_lane: &BTreeSet::new(),
            connected: &connected,
            policy: dorc_plan::WallPolicy::Honest,
            regions: &dorc_plan::region::RegionCensus::default(),
            world_account: dorc_core::influence::InfluenceAccount::authored_before_contact(),
        };
        let _ = origin;
        settle_world(
            &frozen,
            &probe,
            &results,
            &plan_inputs,
            cap,
            &mut interner,
            &mut arena,
            &mut dorc_analysis::certify::CertifierTrip::default(),
        )
    }

    /// The cascade's own liveness: uncapped, rung 2's guard becomes valid because rung 1's
    /// install was proven dead — what `26G:fnd-dead-branch-still-invalidates` says must happen.
    #[test]
    fn the_fixpoint_erases_a_dead_mutator_and_revalidates_below_it() {
        let settled = settle_ladder(
            64,
            "site 0 effect=holds rc=0\nsite 1 effect=holds\n\
             site 2 effect=holds rc=0\nsite 3 effect=holds\n",
        );
        assert_eq!(settled.ledger.len(), 2, "both installs are proven dead");
        assert!(
            settled.ledger.entries().any(|(_, e)| e.round().0 >= 2),
            "the second erasure is a round-2+ finding — that IS the cascade"
        );
    }

    /// FAULT INJECTION for `CollapseKind::FixpointCapDegrade`: force the cap to 1 so a fixpoint
    /// Effective Query validity flips ONLY as walls disappear (`30K` §5.2).
    ///
    /// A guard's measured rc is fold-usable exactly when nothing that may execute reaches it. The
    /// ladder's second rung is the witness: with both rungs measured, rung 1's install is proven
    /// dead, its wall goes, and rung 2's guard becomes valid — which is what lets the whole ladder
    /// cascade. Break the effective-validity derivation and only the first rung folds.
    #[test]
    fn a_query_becomes_valid_only_as_the_walls_above_it_disappear() {
        let settled = settle_ladder(
            64,
            "site 0 effect=holds rc=0\nsite 1 effect=holds\n\
             site 2 effect=holds rc=0\nsite 3 effect=holds\n",
        );
        let valid: Vec<bool> = settled.validity.values().copied().collect();
        assert_eq!(
            valid,
            vec![true, true],
            "both guards end effectively valid: nothing that may execute reaches either"
        );
        assert!(
            settled.origin_validity.values().any(|v| !*v),
            "and the SECOND one was not valid at origin — that gap IS the cascade this pins"
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one anti-masking specimen drives the real settlement, ledger, and attribution path"
    )]
    fn a_replacement_cascade_retains_its_typed_cause_without_a_controller() {
        let book = "apt-get install -y oldpkg\n\
                    dpkg -s beta >/dev/null 2>&1 || apt-get install -y beta\n";
        let mut interner = Interner::default();
        let oracle_srcs = vec![REPLACEMENT_CASCADE_ORACLE.to_owned()];
        let oracle_refs = vec![REPLACEMENT_CASCADE_ORACLE];
        let oracle_paths = vec!["replacement-cascade.oracle.sh"];
        let helpers = dorc_oracle::closure::HelperIndex::default();
        let checks = vec![
            dorc_oracle::predict::lift_predicts(&mut interner, REPLACEMENT_CASCADE_ORACLE).value,
        ];
        let verdict_sets = vec![
            dorc_oracle::verdict::VerdictSet::lift(&mut interner, REPLACEMENT_CASCADE_ORACLE).value,
        ];
        let verdicts = dorc_oracle::verdict::VerdictIndex::from_sets(&mut interner, &verdict_sets);
        let idx = dorc_oracle::lift(&mut interner, &oracle_refs).value;
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut interner);
        let peeled = BTreeMap::new();
        let live = dorc_analysis::funcenv::LiveDefinitions::unsolved();
        let frozen = FrozenModel {
            cfg: &cfg,
            value: &value,
            ast: &parsed.value,
            idx: &idx,
            checks: &checks,
            verdicts: &verdicts,
            peeled: &peeled,
            live,
        };
        let mut arena = ProvArena::new();
        let mut verdict_lane = BTreeMap::new();
        let origin = classify_round(
            &frozen,
            &dorc_analysis::erase::ErasedSites::none(),
            &mut interner,
            &mut arena,
            &mut BTreeMap::new(),
            &mut verdict_lane,
            &mut dorc_analysis::certify::CertifierTrip::default(),
        );
        let upstream_fact = origin
            .classes
            .iter()
            .find_map(|(_, class)| match class {
                dorc_analysis::effect::SkipClass::EstablishProbeAmbient(fact)
                | dorc_analysis::effect::SkipClass::EstablishProbeWritten(fact) => Some(*fact),
                _ => None,
            })
            .expect("the upstream install establishes one fact");
        let (vouches, _) = dorc_plan::build_vouches(
            &oracle_refs,
            &oracle_paths,
            &helpers,
            &origin.classes,
            &value,
            &mut interner,
            live,
        );
        let vouches = vouches.value;
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &origin.classes,
            &BTreeMap::new(),
            &dorc_plan::ConnectedPipes::default(),
            |node, provider, argv| {
                ship_predict_body(
                    &oracle_srcs,
                    &helpers,
                    &checks,
                    &interner,
                    provider,
                    argv,
                    node,
                    live,
                )
            },
            |node, subjects, provider, _| {
                verdict_lane
                    .get(&node)
                    .is_some_and(|measurement| measurement.subjects() == subjects)
                    .then(|| {
                        ship_verdict_body(
                            &oracle_srcs,
                            &helpers,
                            &verdict_sets,
                            &interner,
                            provider,
                            node,
                            live,
                        )
                    })?
            },
            |_, _| false,
        );
        let results = parse_str(
            "site 0 effect=holds\nsite 1 effect=holds rc=0\nsite 2 effect=holds\n",
            &mut interner,
        );
        let connected = dorc_plan::ConnectedPipes::default();
        let plan_inputs = dorc_plan::SettleInputs {
            src: book,
            ast: &parsed.value,
            cfg: &cfg,
            vouches: &vouches,
            verdict_lane: &BTreeSet::new(),
            connected: &connected,
            policy: dorc_plan::WallPolicy::Honest,
            regions: &dorc_plan::region::RegionCensus::default(),
            world_account: dorc_core::influence::InfluenceAccount::authored_before_contact(),
        };
        let settled = settle_world(
            &frozen,
            &probe,
            &results,
            &plan_inputs,
            64,
            &mut interner,
            &mut arena,
            &mut dorc_analysis::certify::CertifierTrip::default(),
        );
        let cascades = attribute_cascades(
            &cfg,
            &parsed.value,
            book,
            &settled.round.classes,
            &settled.ledger,
            &settled.validity,
            &settled.origin_validity,
        );
        let attribution = cascades
            .values()
            .find(|attribution| {
                attribution
                    .causes
                    .iter()
                    .any(|cause| matches!(cause, dorc_cli::why::CascadeCause::Replacement { .. }))
            })
            .expect("the query validity flip retains its replacement cause");
        let replacement = attribution
            .causes
            .iter()
            .find_map(|cause| match cause {
                dorc_cli::why::CascadeCause::Replacement {
                    replaced_line,
                    fact,
                    round,
                } => Some((*replaced_line, *fact, *round)),
                dorc_cli::why::CascadeCause::DeadBranch { .. } => None,
            })
            .expect("the retained cause is replacement-shaped");
        assert_eq!(replacement, (1, upstream_fact, 1));
        assert!(
            attribution.dead_branch.is_none(),
            "a replacement-only cascade must not fabricate a dead-branch controller"
        );
    }

    /// A guard whose own rc says the fallback is LIVE keeps its wall, so the rung below it stays
    /// invalid: validity tracks what may execute, not what was measured (`30K` §5.2).
    #[test]
    fn a_live_fallback_keeps_the_rung_below_it_invalid() {
        let settled = settle_ladder(
            64,
            "site 0 effect=holds rc=1\nsite 1 effect=holds\n\
             site 2 effect=holds rc=0\nsite 3 effect=holds\n",
        );
        let valid: Vec<bool> = settled.validity.values().copied().collect();
        assert_eq!(
            valid,
            vec![true, false],
            "rung 1's install is LIVE (rc 1 left of `||`), so it walls rung 2's guard"
        );
        assert!(
            settled.ledger.is_empty(),
            "and nothing is proven un-runnable, so no wall is retired"
        );
    }

    /// FAULT INJECTION for `CollapseKind::FixpointCapDegrade`: force the cap to 1 so a settlement
    /// that genuinely wants a second round trips it. The degrade must discard EVERY erasure (the
    /// answer becomes the pre-W-C one, never a partial fixpoint) and must narrate — withdrawing
    /// licensed elisions is a safety-narrowing like any other.
    #[test]
    fn a_capped_fixpoint_degrades_to_origin_and_narrates() {
        let records = "site 0 effect=holds rc=0\nsite 1 effect=holds\n\
                       site 2 effect=holds rc=0\nsite 3 effect=holds\n";
        let capped = settle_ladder(1, records);
        assert!(
            capped.ledger.is_empty(),
            "a capped fixpoint ships NO erasure at all, not a partial set"
        );
        assert_eq!(
            capped
                .merge_narrative
                .iter()
                .filter(|e| matches!(e.kind(), CollapseKind::FixpointCapDegrade { .. }))
                .count(),
            1,
            "the degrade mints exactly one narrative"
        );
        assert!(
            !settle_ladder(64, records)
                .merge_narrative
                .iter()
                .any(|e| matches!(e.kind(), CollapseKind::FixpointCapDegrade { .. })),
            "a fixpoint that quiesces normally narrates no degrade"
        );
    }

    /// The negative pin: sites that AGREE on a shared cell produce no readout. A shared cell is
    /// ordinary and common; only its collapse is worth a line.
    #[test]
    fn an_agreeing_shared_cell_reports_nothing() {
        let mut i = Interner::default();
        let fact = pkg(&mut i, "nginx");
        let probe = probe2(fact, ProbeSiteKind::Establish, ProbeSiteKind::Establish);
        let agree = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=0\n",
            &mut i,
        );
        assert!(
            facts_from_sites(&probe, &agree, &BTreeMap::new())
                .2
                .is_empty()
        );
    }

    /// Build + deframe a FRAMED stream (the framed regime) for a set of inner records, then
    /// inner-parse — the exact production round-trip a real remote host drives.
    fn parse_framed(sites: usize, inners: &[&str], i: &mut Interner) -> SiteResults {
        use dorc_plan::records::{DEFAULT_NONCE, TERMINAL_TOKEN};
        let framing = dorc_plan::records::Framing::spike("bk".to_owned());
        let recs = inners
            .iter()
            .map(|inner| format!("{DEFAULT_NONCE} {inner} {TERMINAL_TOKEN}\n"))
            .collect::<Vec<_>>()
            .concat();
        let s = format!(
            "dorc-records/1 nonce={DEFAULT_NONCE} attempt=1 host=localhost book=bk sites={sites} {TERMINAL_TOKEN}\n\
             {recs}dorc-records-end/1 nonce={DEFAULT_NONCE} {TERMINAL_TOKEN}\n"
        );
        let d = dorc_plan::records::deframe(
            &s,
            &framing.expect(),
            dorc_plan::records::LegacyPolicy::Refuse,
        );
        assert!(
            !d.refused,
            "the framed round-trip is not refused: {:?}",
            d.diagnostics
        );
        parse_results(&d.records, d.framed, &mut RunClock::Absent, i)
    }

    #[test]
    fn pin_duplicate_records_merge_by_meet_never_last_wins() {
        // `262` §2 / §1 tie-break law: two records for ONE (site, member) key merge by MEET,
        // never last-wins. Conflicting ⇒ ⊤ (run); identical ⇒ idempotent. Arrival order is
        // never consulted — the disagreement result is the same in either order.
        let mut i = Interner::default();
        let fwd = parse_str(
            "site 0 effect=holds rc=0\nsite 0 effect=absent rc=1\n",
            &mut i,
        );
        let rev = parse_str(
            "site 0 effect=absent rc=1\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        let ra = fwd.records.get(&rk(0)).copied().expect("site 0");
        let rb = rev.records.get(&rk(0)).copied().expect("site 0");
        assert_eq!(
            ra.verdict,
            Verdict::Unknown,
            "conflict ⇒ ⊤ (run), never last-wins"
        );
        assert!(
            ra.conflicted,
            "a conflicting duplicate is marked (withholds the fold rc)"
        );
        assert_eq!(
            (ra.verdict, ra.conflicted),
            (rb.verdict, rb.conflicted),
            "the meet is order-independent (no arrival-sequence tie-break)"
        );
        // Idempotent: two IDENTICAL records ⇒ unchanged (not spuriously conflicted).
        let dup = parse_str(
            "site 0 effect=holds rc=0\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        let r = dup.records.get(&rk(0)).copied().expect("site 0");
        assert!(
            !r.conflicted && r.verdict == Verdict::Converged,
            "identical dup is idempotent"
        );
    }

    #[test]
    fn pin_fold_permutation_records_are_leafid_keyed_order_free() {
        // `262` §1 pin-fold-permutation: fold(any permutation of records) ≡ fold(book order).
        // Records are leafid-keyed + self-describing (no positional meaning), so two
        // distinct-cell records fed in either order fold to the identical by_fact map.
        let mut i = Interner::default();
        let nginx = pkg(&mut i, "nginx");
        let curl = pkg(&mut i, "curl");
        let probe = probe_two_facts(nginx, curl);
        let book = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=absent rc=1\n",
            &mut i,
        );
        let rev = parse_str(
            "site 1 effect=absent rc=1\nsite 0 effect=holds rc=0\n",
            &mut i,
        );
        assert_eq!(
            facts_from_sites(&probe, &book, &BTreeMap::new()).0,
            facts_from_sites(&probe, &rev, &BTreeMap::new()).0,
            "record arrival order never changes the fold (leafid-keyed)"
        );
    }

    #[test]
    fn pin_terminal_determinism_framed_folds_like_unframed() {
        // `262` §1 pin-terminal-determinism: the FRAMED serial stream folds to the same facts
        // as the unframed equivalent (byte-identical modulo framing lines). Deframing +
        // inner-parsing a framed stream ≡ legacy-parsing the same inner records.
        let mut i = Interner::default();
        let nginx = pkg(&mut i, "nginx");
        let curl = pkg(&mut i, "curl");
        let probe = probe_two_facts(nginx, curl);
        let inners = ["site 0 effect=holds rc=0", "site 1 effect=absent rc=1"];
        let unframed = parse_str(&format!("{}\n{}\n", inners[0], inners[1]), &mut i);
        let framed = parse_framed(2, &inners, &mut i);
        assert_eq!(
            facts_from_sites(&probe, &unframed, &BTreeMap::new()).0,
            facts_from_sites(&probe, &framed, &BTreeMap::new()).0,
            "the framing lines are fold-invisible — the plan is unchanged"
        );
    }

    /// A two-check probe over two DISTINCT cells (both valid Query, sites 0 and 1).
    fn probe_two_facts(f0: FactKey, f1: FactKey) -> ProbePlan {
        let mk = |site: u32, fact: FactKey| ProbePredict {
            site: LeafId(site),
            member: None,
            fact,
            provider: fact.kind.0,
            argv: vec![],
            site_kind: ProbeSiteKind::Query { valid: true },
            sh: "{ :; }".to_string(),
            defining_span: None,
            connected: None,
            verdict: false,
            emits_report: false,
            entry: None,
        };
        ProbePlan {
            checks: vec![mk(0, f0), mk(1, f1)],
            unresolvable: vec![],
            unresolvable_causes: BTreeMap::new(),
        }
    }

    #[test]
    fn same_cell_conflicting_query_status_degrades_to_top() {
        // The conflict floor on the Status channel: two VALID Query sites on one cell
        // reporting DIFFERENT rcs (rc=0 vs rc=1) ⇒ merged status ⊤ (a self-contradicting
        // guard cannot fold a branch). A valid Query's rc normally feeds Status (the
        // firewall), but a conflict on it must still degrade — the meet beats the firewall.
        let mut i = Interner::default();
        let fact = tool(&mut i, "nginx");
        let probe = probe2(
            fact,
            ProbeSiteKind::Query { valid: true },
            ProbeSiteKind::Query { valid: true },
        );
        let results = parse_str(
            "site 0 effect=holds rc=0\nsite 1 effect=holds rc=1\n",
            &mut i,
        );
        let obs = facts_from_sites(&probe, &results, &BTreeMap::new())
            .0
            .get(&fact)
            .copied()
            .expect("keyed");
        // Effect agrees (both holds) ⇒ Converged; but the rcs disagree ⇒ status ⊤.
        assert_eq!(obs.effect, Verdict::Converged, "effect agrees");
        assert_eq!(
            obs.status,
            Predicted::Top,
            "disagreeing same-cell Query rcs degrade Status to ⊤ (no fold off a contradiction)"
        );
    }

    #[test]
    fn unresolvable_diagnostics_name_the_source_command() {
        // q-2 (`dq-site-unresolvable`, the cli-edge readout): a probe-unresolvable site is
        // disclosed on stderr naming its SOURCE command text (`219` q-1.f silent-3 closed). An
        // un-oracled command (`make install`) is Opaque ⇒ unresolvable ⇒ the apply runs it; the
        // Note must carry its source. Drives the full pipeline (parse → classify → compile_probe
        // → build_plan) so the LeafId→source mapping is the real one.
        let mut interner = Interner::default();
        let book = "make install\n";
        let parsed = dorc_syntax::parse(book);
        let cfg = dorc_analysis::cfg::build(&parsed.value);
        let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
        let idx = dorc_oracle::KindIndex::default();
        let mut arena = ProvArena::new();
        let classified = dorc_analysis::effect::classify(
            &cfg.value,
            &value,
            &parsed.value,
            &idx,
            &[],
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut interner,
            &mut arena,
        );
        let classes = classified.value;
        let invalidators = classified.invalidators;
        let probe = dorc_plan::compile_probe(
            &parsed.value,
            &cfg.value,
            &value,
            &classes,
            &BTreeMap::new(),
            &dorc_plan::ConnectedPipes::default(),
            |_, _, _| None,
            |_, _, _, _| None,
            |_, _| false,
        );
        let plan = dorc_plan::build_plan(
            book,
            &parsed.value,
            &cfg.value,
            &classes,
            &invalidators,
            // All-Unknown ⇒ nothing elides regardless of any vouch; empty is honest here.
            &dorc_plan::Vouches::new(),
            |_| Observable::verdict_only(Verdict::Unknown),
            &mut arena,
        );
        let diags = unresolvable_diagnostics(&probe, &plan, &parsed.value, book);
        let i = Interner::default();
        assert!(
            diags.iter().any(|d| d.code.slug() == "site-unresolvable"),
            "an Opaque site must be disclosed unresolvable: {diags:?}"
        );
        assert!(
            diags.iter().any(|d| d.code.slug() == "site-unresolvable"
                && dorc_aid::diag::render_body(d, &i).contains("make install")),
            "the disclosure must name the source command: {diags:?}"
        );
        assert!(
            diags.iter().all(|d| d.severity() == Severity::Note),
            "the readout is Note-severity (never trips gate-3): {diags:?}"
        );
    }

    #[test]
    fn advisory_filter_drops_warnings_notes_but_keeps_errors_in_apply() {
        // rec-1 / tc-apply-receipt-floor (ui-A): the receipt-free `apply` off-ramp keeps the
        // ERROR floor (a shippable artifact must never hide an error) while dropping the
        // advisory plane (warnings + notes); `plan`/round-trip (advisory=true) pass everything
        // through. This is the lone place the artifact-vs-render two-surface split becomes a
        // per-severity routing decision — pin BOTH directions so a future edit cannot silently
        // (a) leak advisory disclosure into the off-ramp surface, or (b) swallow an error there.
        // One code per severity, spelled with real catalog variants (registry-Error/Warning/Note).
        use dorc_aid::diag::{
            CfgBuiltinShadowed, RedirTargetTop, SiteId, SyntaxMalformed, SyntaxMalformedReason,
        };
        use dorc_core::{BytePos, Span};
        let span = Span::new(BytePos(0), BytePos(1));
        let mixed = vec![
            Diag::new(
                DiagCode::SyntaxMalformed(SyntaxMalformed {
                    reason: SyntaxMalformedReason::ExpectedFiToCloseIf,
                }),
                span,
            ),
            Diag::new(
                DiagCode::CfgBuiltinShadowed(CfgBuiltinShadowed {
                    name: "cd".to_owned(),
                }),
                span,
            ),
            Diag::new(
                DiagCode::RedirTargetTop(RedirTargetTop {
                    site: SiteId::leaf(LeafId(0)),
                }),
                span,
            ),
        ];

        // advisory=true (plan / round-trip): every severity survives — the full cited-disclosure
        // render surface (ru-20 ui-3).
        let kept = advisory_filter(true, &mixed);
        assert_eq!(kept.len(), 3, "plan surface keeps every severity: {kept:?}");

        // advisory=false (apply off-ramp): ONLY the error survives — receipt-free, not blind.
        let kept = advisory_filter(false, &mixed);
        assert_eq!(
            kept.len(),
            1,
            "apply keeps only the error floor (no warnings/notes): {kept:?}"
        );
        assert_eq!(
            kept[0].severity(),
            Severity::Error,
            "the surviving diagnostic is the Error (the never-hide floor): {kept:?}"
        );
    }
}
