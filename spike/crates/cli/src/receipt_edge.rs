//! The receipt edge: what a settled run RECORDS, and how that record is published.
//!
//! This seat lives lib-side so the binary and the in-process battery drive ONE of it. A test that
//! re-implemented the recording would demonstrate a capability it never observed, which is the
//! defect `one-definition-table-two-drivers` exists to refuse.
//!
//! Nothing here opens a file, reads the environment, asks a clock, or produces randomness. Every
//! such answer arrives as a VALUE — argv, the run instant, the identity source, the signer, the
//! sink — so the seam this module sits on is the one `lib-target-is-a-loom-seam` draws.

use std::collections::BTreeMap;

use dorc_plan::planning_input::PlanningMode;
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::records::{AdmittedUnscopedHostRecords, Framing};
use dorc_receipt::capability::{PublicationGrade, ReceiptSigner, ReceiptSink};
use dorc_receipt::format::Skeleton;
use dorc_receipt::ids::ReceiptIdSource;
use dorc_receipt::model::{Plain, PlanReceipt};
use dorc_receipt::projection::narrow_to_plain;
use dorc_receipt::tokens::RecordedInvocationMode;
use dorc_receipt::writer::DraftReceipt;

use crate::results::SiteResults;
use crate::snapshot::StaticLoadSnapshot;

/// The controller semantics two builds must not share when they could analyse one book
/// differently.
///
/// DISCLOSED: every spike build spells `0.0.0`, so today this discriminates nothing. It is the
/// honest value the tree has, and it becomes discriminating the moment the crate is versioned.
pub const CONTROLLER_SEMANTICS: &str = concat!("dorc/", env!("CARGO_PKG_VERSION"));

/// The invocation record, built ONCE so the witness and the Spine cannot describe two runs.
///
/// `argv` and `started_at` arrive as values because obtaining them are QUERIES, and queries stay
/// on the process edge.
#[must_use]
pub fn invocation_record(
    argv: Vec<String>,
    framing: &Framing,
    snapshot: &StaticLoadSnapshot,
    started_at: Option<dorc_core::RunInstant>,
    world_account: dorc_core::influence::InfluenceAccount,
) -> dorc_core::spine::SpineInvocation {
    dorc_core::spine::SpineInvocation::minted(
        dorc_core::spine::InvocationMode::WhylogReplay,
        argv,
        source_claims(snapshot),
        dorc_core::spine::RunIdentity {
            nonce: framing.nonce().0.clone(),
            attempt: framing.attempt(),
            host: framing.host().to_owned(),
            started_at,
        },
        world_account,
    )
}

/// Which surface an invocation asked for, in the planner's own closed vocabulary.
#[must_use]
pub const fn planning_mode(mode: crate::Mode) -> PlanningMode {
    match mode {
        crate::Mode::Bundle => PlanningMode::Bundle,
        crate::Mode::Probe => PlanningMode::Probe,
        crate::Mode::Plan => PlanningMode::Plan,
        crate::Mode::Apply => PlanningMode::Apply,
        crate::Mode::RoundTrip => PlanningMode::RoundTrip,
        crate::Mode::Why => PlanningMode::Why,
    }
}

/// Write the run's durable-arm records onto the Spine (`30E` §2's four species).
///
/// The document is projected from these; nothing here decides what reaches a sink. That
/// separation is the point: the driver states what the run WAS, and one seat decides what a
/// document KEEPS of it.
pub fn record_durable_arm(
    spine: &mut dorc_plan::Spine,
    invocation: dorc_core::spine::SpineInvocation,
    presentation: &FinalPresentation,
    results: &SiteResults,
    records: AdmittedUnscopedHostRecords,
    world_account: dorc_core::influence::InfluenceAccount,
) {
    spine.set_invocation(invocation);
    // The witness minted this identity over the settled surface, so the Spine's record and the
    // projection's row cannot disagree about which surface the run presented.
    spine.set_presented_plan(dorc_core::spine::SpinePresentedPlan::minted(
        presentation.presented_plan(),
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

/// Every acquired source as one ordered role-carrying claim, straight off the snapshot's own
/// triple seat.
///
/// The book is a row wearing `SourceRole::Book`, not a field beside the others. The snapshot's
/// three vectors are indexed by `SourceFileId`, so this vector's order IS the acquired-source
/// table order a document ordinal over it means.
fn source_claims(snapshot: &StaticLoadSnapshot) -> Vec<dorc_core::spine::SourceClaim> {
    snapshot
        .source_claims()
        .map(|(path, src, role)| dorc_core::spine::SourceClaim {
            path: path.to_owned(),
            digest: dorc_plan::invocation::book_digest(src),
            role,
            bytes: u64::try_from(src.len()).unwrap_or(u64::MAX),
        })
        .collect()
}

/// Why a run published no plan document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationRefusal {
    /// The Spine did not project.
    Projection(dorc_plan::receipt::ProjectionRefusal),
    /// A projected row did not satisfy the grammar table.
    Grammar(dorc_receipt::RefusalReason),
    /// The sink did not place the document.
    Sink,
}

/// Project, narrow, sign, and publish one plan document.
///
/// PLAIN, and that is a statement rather than a shortcut: the projection marks a slot `captured`
/// wherever the run HELD the value, and narrowing is what turns each of those into
/// `withheld-plain`. Reusing that one seat is what keeps a plain document's states honest instead
/// of a second assembly deciding them again. The rich projection needs the held bytes themselves
/// and is owed separately.
///
/// # Errors
/// Refuses a Spine that does not project, a row outside the grammar, or a sink that declines.
pub fn publish_plan_receipt(
    spine: &dorc_plan::Spine,
    mode: RecordedInvocationMode,
    world: dorc_core::influence::InfluenceAccount,
    presentation: &FinalPresentation,
    ids: &mut dyn ReceiptIdSource,
    signer: &dyn ReceiptSigner,
    sink: &mut dyn ReceiptSink,
) -> Result<PublicationGrade, PublicationRefusal> {
    let model = dorc_plan::receipt::project(spine, mode, world, presentation)
        .map_err(PublicationRefusal::Projection)?;
    let records = model.to_records().map_err(PublicationRefusal::Grammar)?;
    let assembled = Skeleton {
        receipt_id: ids.next_receipt_id().hex(),
        signing_key_id: signer.signing_key_id().hex(),
        encryption_key_id: None,
        records,
    };
    let plain = narrow_to_plain(&assembled, ids).map_err(PublicationRefusal::Grammar)?;
    let name = format!("plan-{}", plain.receipt_id);
    let document = DraftReceipt::<PlanReceipt, Plain>::of(plain)
        .serialize()
        .map_err(PublicationRefusal::Grammar)?
        .sign(signer);
    document
        .publish(&name, sink)
        .map(|published| published.grade())
        .map_err(|_| PublicationRefusal::Sink)
}
