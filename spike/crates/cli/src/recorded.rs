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
use dorc_receipt::report::{
    CurrentSourceReading, RecordedDocumentId, RequestedAddress, SiblingState,
};
use dorc_why::Reconstruction;
use dorc_why::recorded::{
    AddressStanding, NonPlanRoot, Rooted, ShallowIntent, ShallowOutcome, reconstruct,
};
use dorc_why::{CorrelationFact, FindingKind, UnplaceableAddress};

use crate::durable::LocallyAuthenticated;
use crate::recorded_facts::{ObservedSource, SelectedRoot, facts_for};

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
    /// The named file read; these are its exact bytes and the physical line asked about.
    Read {
        /// Which physical line, 1-indexed, as the user spelled it.
        line: u32,
        /// The named file's exact bytes.
        bytes: Vec<u8>,
    },
    /// The address never became a file and a line at all.
    Unplaceable(UnplaceableAddress),
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
                let (address, observations, standing) = self.address.against(&root, &self.siblings);
                let facts = facts_for(&root, self.siblings, observations, address);
                reconstruct(&Rooted::Plan(&facts), standing)
            }
            // A non-plan root carries no recorded source table, so an address that named a file has
            // nothing to match against — the same word as a plan root whose sources it missed,
            // because the reader's position is the same: nothing places it.
            ReadRoot::OtherSpecies(root) => {
                let standing = match self.address {
                    AddressAsk::Unasked => AddressStanding::AsRecorded,
                    AddressAsk::Unplaceable(why) => AddressStanding::Unplaceable(why),
                    AddressAsk::Read { .. } => {
                        AddressStanding::Unplaceable(UnplaceableAddress::NoRecordedSourceMatches)
                    }
                };
                reconstruct(&Rooted::OtherSpecies(&root), standing)
            }
        }
    }
}

impl AddressAsk {
    /// Match the named file against the root's recorded sources, by CONTENT DIGEST.
    ///
    /// Digest and nothing else: there is no nearest-match, no path comparison, and no moved-line
    /// search, because each would answer confidently about a line the author moved
    /// (`271:rul-sin-ordering`'s worst rung). A file whose bytes no recorded source reproduces is
    /// unplaceable, and saying so is the whole answer this route can give.
    fn against(
        self,
        root: &SelectedRoot,
        siblings: &[SiblingState],
    ) -> (
        Option<RequestedAddress>,
        Vec<ObservedSource>,
        AddressStanding,
    ) {
        let (line, bytes) = match self {
            Self::Unasked => return (None, Vec::new(), AddressStanding::AsRecorded),
            Self::Unplaceable(why) => {
                return (None, Vec::new(), AddressStanding::Unplaceable(why));
            }
            Self::Read { line, bytes } => (line, bytes),
        };
        let digest = dorc_plan::invocation::book_digest(&String::from_utf8_lossy(&bytes));
        let survey = facts_for(root, siblings.to_vec(), Vec::new(), None);
        let Some(ordinal) = survey
            .sources()
            .iter()
            .find(|source| source.digest() == digest)
            .map(dorc_receipt::report::SourceFacts::ordinal)
        else {
            return (
                None,
                Vec::new(),
                AddressStanding::Unplaceable(UnplaceableAddress::NoRecordedSourceMatches),
            );
        };
        (
            Some(RequestedAddress::of(ordinal, line)),
            vec![ObservedSource {
                ordinal,
                reading: CurrentSourceReading::Read(bytes),
                matches_digest: true,
            }],
            AddressStanding::AsRecorded,
        )
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
