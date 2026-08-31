//! What `dorc why` reads back from a receipt, as the value its render seat consumes.
//!
//! # One root, never a store
//!
//! `30R:receipt-rooted-attention-and-cli` licenses no whole-store explanation mode: the user-facing
//! unit is ONE selected root plus the causally relevant closure. The store walk survives only as
//! bounded DISCOVERY of typed reverse edges, and nothing here unions disconnected histories.
//!
//! # Report-only, by type
//!
//! Everything below reads a `Reingested<…>`, whose seal is what makes a recorded value unable to
//! become a live one. Nothing here converts, and nothing here decides.
//!
//! # The two-pass address
//!
//! An address names a FILE and the model names an ORDINAL, and the only bridge is the recorded
//! content digest (`30Vd:fnd-current-source-is-user-named-only`: a recorded source has no path
//! exit). So the reading derives the model once to see the source table, matches the bytes the edge
//! read, and derives it again with the address bound. Pure both times — the edge's only act was
//! opening the file the user named.

use dorc_receipt::apply::RecordedApplyIntent;
use dorc_receipt::graph::{GraphFinding, ReceiptEdge, ReceiptGraph};
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Rich};
use dorc_receipt::outcome::RecordedApplyOutcome;
use dorc_receipt::reader::Receipt;
use dorc_receipt::reingested::Reingested;
use dorc_receipt::report::{RecordedDocumentId, SiblingState};
use dorc_why::recorded::{NonPlanRoot, Rooted, ShallowIntent, ShallowOutcome, reconstruct};
use dorc_why::{
    AddressStanding, ComparedSources, CorrelationFact, FindingKind, Reconstruction,
    UnplaceableAddress,
};

use crate::durable::LocallyAuthenticated;
use crate::recorded_facts::{SelectedRoot, facts_for};
use crate::source_comparison::{NamedFile, compare_sources};

/// What the receipt read answered with. Three outcomes, and none is interchangeable.
#[derive(Debug)]
pub enum StoreAnswer {
    /// One root was selected and read.
    Rooted(Box<RootedReading>),
    /// The store's greatest order names a COHORT rather than a document.
    ///
    /// Reported and never resolved: a tie-break on receipt identity would choose a document by the
    /// value least related to when it was written. Answering with all of them instead would be the
    /// whole-store union `30R` refuses, so this invocation explains nothing and says why.
    Ambiguous(usize),
    /// Nothing the read could answer with, in the edge's own closed word.
    Unreadable(String),
}

/// One rooted question, read: the root, what the graph could not hold, and the address as asked.
#[derive(Debug)]
pub struct RootedReading {
    root: ReadRoot,
    siblings: Vec<SiblingState>,
    address: AddressAsk,
}

/// The selected root, by species.
#[derive(Debug)]
pub enum ReadRoot {
    /// A plan receipt, whose sealed report model covers it.
    Plan(Box<SelectedRoot>),
    /// An apply intent or outcome: identity, standing, correlations, and its own shallow facts.
    OtherSpecies(NonPlanRoot),
}

/// The address the question named, as far as the EDGE could take it.
#[derive(Debug)]
pub enum AddressAsk {
    /// The question named no address.
    Unasked,
    /// The named file read; the comparison seat decides which recorded source it is.
    Read(NamedFile),
    /// The address never became a file and a line at all.
    Unplaceable(UnplaceableAddress),
}

impl AddressAsk {
    /// The file the question named, where one was named and read.
    fn named_file(&self) -> Option<&NamedFile> {
        match self {
            Self::Read(named) => Some(named),
            Self::Unasked | Self::Unplaceable(_) => None,
        }
    }

    /// The standing an address has BEFORE any source was walked.
    ///
    /// Only the edge's own refusals: whether a named file corresponds to a recorded source is the
    /// comparison seat's answer, and pre-empting it here would be a second correspondence rule.
    const fn standing(&self) -> AddressStanding {
        match self {
            Self::Unasked | Self::Read(_) => AddressStanding::AsRecorded,
            Self::Unplaceable(why) => AddressStanding::Unplaceable(*why),
        }
    }
}

impl RootedReading {
    /// Bind one rooted question's read.
    #[must_use]
    pub const fn of(root: ReadRoot, siblings: Vec<SiblingState>, address: AddressAsk) -> Self {
        Self {
            root,
            siblings,
            address,
        }
    }

    /// The reconstruction this reading answers with.
    ///
    /// Consuming, because a reading is one question's whole answer and deriving it twice would
    /// hand two callers two models of one document.
    #[must_use]
    pub fn reconstruct(self) -> Reconstruction {
        match self.root {
            ReadRoot::Plan(root) => {
                // The FIRST derivation exists to hand the comparison seat a source table to walk;
                // the second binds what the seat established. Two passes rather than one because
                // the model names ordinals and a question names a FILE, and only the seat can join
                // them (`the-two-pass-address` above).
                let survey = facts_for(&root, self.siblings.clone(), Vec::new(), None);
                let outcome = compare_sources(&survey.source_material(), self.address.named_file());
                let compared = match self.address.standing() {
                    // An edge refusal outranks the seat's: a question that never became a file and
                    // a line was never a question about a source at all.
                    AddressStanding::Unplaceable(why) => ComparedSources::of(
                        AddressStanding::Unplaceable(why),
                        outcome.compared.into_named(),
                    ),
                    AddressStanding::AsRecorded => outcome.compared,
                };
                let facts = facts_for(&root, self.siblings, outcome.observations, outcome.address);
                reconstruct(&Rooted::Plan(&facts), &compared)
            }
            // A non-plan root carries no recorded source table, so an address that named a file has
            // nothing to match against — the same word as a plan root whose sources it missed,
            // because the reader's position is the same: nothing places it.
            ReadRoot::OtherSpecies(root) => {
                let standing = match self.address {
                    AddressAsk::Unasked => AddressStanding::AsRecorded,
                    AddressAsk::Unplaceable(why) => AddressStanding::Unplaceable(why),
                    AddressAsk::Read(_) => {
                        AddressStanding::Unplaceable(UnplaceableAddress::NoRecordedSourceMatches)
                    }
                };
                reconstruct(
                    &Rooted::OtherSpecies(&root),
                    &ComparedSources::of(standing, Vec::new()),
                )
            }
        }
    }
}

/// An apply intent's own facts, from the accessors its read-back wrapper already exposes.
///
/// SHALLOW on purpose (`30Vd:tc-nonplan-root-depth` stands): per-assignment rows would need sealed
/// accessors nobody has written, and inventing a deeper projection here would be a receipt-crate
/// change wearing a cli seat's clothes.
#[must_use]
pub fn shallow_intent(document: &Reingested<Receipt<ApplyIntent, Rich>>) -> Option<ShallowIntent> {
    let model: Reingested<RecordedApplyIntent> = document.model().ok()?;
    Some(ShallowIntent {
        policy: model.policy(),
        origin_state: model.origin_state(),
        assignment_count: model.assignment_count() as u64,
        origin_receipts: model
            .origin_receipts()
            .into_iter()
            .map(RecordedDocumentId::Plan)
            .collect(),
    })
}

/// An apply outcome's own facts, at the same depth.
#[must_use]
pub fn shallow_outcome(
    document: &Reingested<Receipt<ApplyOutcome, Rich>>,
) -> Option<ShallowOutcome> {
    let model: Reingested<RecordedApplyOutcome> = document.model().ok()?;
    Some(ShallowOutcome {
        terminal: model.terminal(),
        site_count: model.site_count() as u64,
        intent: model.intent().map(RecordedDocumentId::ApplyIntent),
    })
}

/// Every typed correlation a rooted question's own closure carries.
///
/// Edges are kept only where BOTH endpoints are in the closure: an edge into a document this
/// question never reached would state a relation the answer cannot show either end of. Findings are
/// kept whole, because a finding is a shape of the record SET and dropping one would let a reader
/// infer a clean history from a filtered view.
#[must_use]
pub fn correlations_of(
    graph: &ReceiptGraph,
    closure: &[RecordedDocumentId],
) -> Vec<CorrelationFact> {
    let holds = |id: &RecordedDocumentId| closure.contains(id);
    let mut out: Vec<CorrelationFact> = Vec::new();
    for edge in graph.edges() {
        let (from, to) = match edge {
            ReceiptEdge::PlanToIntent { plan, intent } => (
                RecordedDocumentId::Plan(plan),
                RecordedDocumentId::ApplyIntent(intent),
            ),
            ReceiptEdge::IntentToOutcome { intent, outcome } => (
                RecordedDocumentId::ApplyIntent(intent),
                RecordedDocumentId::ApplyOutcome(outcome),
            ),
        };
        if holds(&from) && holds(&to) {
            out.push(match edge {
                ReceiptEdge::PlanToIntent { .. } => CorrelationFact::PlanToIntent {
                    plan: from,
                    intent: to,
                },
                ReceiptEdge::IntentToOutcome { .. } => CorrelationFact::IntentToOutcome {
                    intent: from,
                    outcome: to,
                },
            });
        }
    }
    out.extend(
        graph
            .findings()
            .iter()
            .map(|finding| CorrelationFact::Finding(finding_kind(finding))),
    );
    out
}

/// What the graph's findings say a rooted question is MISSING.
///
/// Only the two shapes that name a document the question needed and the store does not hold: every
/// other finding is a fact about the set rather than an absent sibling, and reporting one as a
/// missing document would invent an edge the graph never had
/// (`inv-graph-edges-are-explicit`).
#[must_use]
pub fn siblings_of(graph: &ReceiptGraph, root: &RecordedDocumentId) -> Vec<SiblingState> {
    let mut out = Vec::new();
    for finding in graph.findings() {
        match finding {
            GraphFinding::OriginatingPlanAbsent { intent, plan }
                if root == &RecordedDocumentId::ApplyIntent(intent) =>
            {
                out.push(SiblingState::Missing(RecordedDocumentId::Plan(plan)));
            }
            GraphFinding::OutcomeWithoutIntent { outcome, intent }
                if root == &RecordedDocumentId::ApplyOutcome(outcome) =>
            {
                out.push(SiblingState::Missing(RecordedDocumentId::ApplyIntent(
                    intent,
                )));
            }
            _ => {}
        }
    }
    out
}

/// One graph finding, in the reconstruction's own closed vocabulary.
///
/// No-wildcard, so a widened graph vocabulary visits this seat rather than falling into a
/// neighbour's word.
const fn finding_kind(finding: &GraphFinding) -> FindingKind {
    match finding {
        GraphFinding::IdentityCollision { .. } => FindingKind::IdentityCollision,
        GraphFinding::OriginatingPlanAbsent { .. } => FindingKind::OriginatingPlanAbsent,
        GraphFinding::OriginatingPlanUnavailable { .. } => FindingKind::OriginatingPlanUnavailable,
        GraphFinding::OutcomeWithoutIntent { .. } => FindingKind::OutcomeWithoutIntent,
        GraphFinding::OutcomeIntentUnreadable { .. } => FindingKind::OutcomeIntentUnreadable,
        GraphFinding::SupernumeraryOutcome { .. } => FindingKind::SupernumeraryOutcome,
        GraphFinding::IdentityUnreadable { .. } => FindingKind::IdentityUnreadable,
    }
}

/// Drop cohort members that are typed graph predecessors of another cohort member.
///
/// Only edges whose BOTH endpoints share the cohort collapse anything: an intent outside the
/// newest order says nothing about which of the newest documents is last. Order is preserved, so
/// the survivors read in the store's own order rather than an incidental one.
///
/// Deliberately one hop, not a transitive closure. The receipt graph is plan → intent → outcome,
/// so a chain wholly inside one cohort collapses anyway — the plan loses to the intent and the
/// intent loses to the outcome — and computing a closure would only add a way to be wrong about a
/// species relation the graph does not have.
#[must_use]
pub fn collapse_predecessors(cohort: Vec<String>, edges: &[ReceiptEdge]) -> Vec<String> {
    let in_cohort = |id: &str| cohort.iter().any(|member| member == id);
    let mut superseded: Vec<String> = Vec::new();
    for edge in edges {
        let (from, to) = match edge {
            ReceiptEdge::PlanToIntent { plan, intent } => (plan.hex(), intent.hex()),
            ReceiptEdge::IntentToOutcome { intent, outcome } => (intent.hex(), outcome.hex()),
        };
        if in_cohort(&from) && in_cohort(&to) {
            superseded.push(from);
        }
    }
    let survivors: Vec<String> = cohort
        .iter()
        .filter(|member| !superseded.contains(member))
        .cloned()
        .collect();
    // A cohort that collapsed to nothing would be a cycle, which this graph's species ordering
    // cannot express; keeping the cohort is the honest answer if one ever appears, because
    // answering with an empty selection would silently explain nothing.
    if survivors.is_empty() {
        cohort
    } else {
        survivors
    }
}

/// The local-authentication envelope a read answers with, unwrapped for the graph's ingest.
///
/// Its own function because the ingest wants the sealed document and the trust word TOGETHER, and a
/// caller pairing them by hand could pair one document with another's standing.
pub fn ingest_plan(
    graph: &mut ReceiptGraph,
    document: &LocallyAuthenticated<Reingested<Receipt<dorc_receipt::model::PlanReceipt, Rich>>>,
    image: &[u8],
) {
    graph.ingest_plan(document.document(), document.signer_trust(), image);
}

/// As [`ingest_plan`], for an apply intent.
pub fn ingest_intent(
    graph: &mut ReceiptGraph,
    document: &LocallyAuthenticated<Reingested<Receipt<ApplyIntent, Rich>>>,
    image: &[u8],
) {
    graph.ingest_intent(document.document(), document.signer_trust(), image);
}

/// As [`ingest_plan`], for an apply outcome.
pub fn ingest_outcome(
    graph: &mut ReceiptGraph,
    document: &LocallyAuthenticated<Reingested<Receipt<ApplyOutcome, Rich>>>,
    image: &[u8],
) {
    graph.ingest_outcome(document.document(), document.signer_trust(), image);
}
