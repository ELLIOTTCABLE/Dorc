//! The report-only receipt graph.
//!
//! Correlation joins immutable typed identities and nothing else. It never consults world
//! freshness, host generation, authority, or influence, and it has no parameter through which a
//! filename or an enumeration order could reach it: documents go in, and the same set produces
//! the same graph whatever order it arrived in.
//!
//! A missing edge is incompleteness. It synthesizes no history and says nothing about whether
//! anything executed.

use std::collections::BTreeMap;

use crate::apply::RecordedApplyIntent;
use crate::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId};
use crate::model::{ApplyIntent, ApplyOutcome, PlanReceipt, Projection, SignerTrust};
use crate::outcome::{MissingOutcome, OutcomeAvailability, RecordedApplyOutcome};
use crate::plan::RecordedPlanReceipt;
use crate::projection::{SameIdentityPair, same_identity_pair};
use crate::reader::{PartialReceipt, Receipt};
use crate::reingested::{RecordedType, Reingested};
use crate::rows::ModelRefusal;
use crate::tokens::{ClosedToken, RecordedSignerTrust};

/// Which species an identity belongs to, for a finding that names one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphSpecies {
    /// A plan document.
    Plan,
    /// An apply intent.
    Intent,
    /// An apply outcome.
    Outcome,
}

impl GraphSpecies {
    /// The word a report renders.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Intent => "apply-intent",
            Self::Outcome => "apply-outcome",
        }
    }
}

/// One correlation the graph could make.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReceiptEdge {
    /// An intent names a plan the graph holds.
    PlanToIntent {
        /// The originating plan.
        plan: PlanReceiptId,
        /// The intent that named it.
        intent: ApplyIntentId,
    },
    /// An outcome answers an intent the graph holds.
    IntentToOutcome {
        /// The intent.
        intent: ApplyIntentId,
        /// The outcome that answered it.
        outcome: ApplyOutcomeId,
    },
}

/// Something a report must surface about the correlation.
///
/// None of these is an error in the document; each is a shape of the record set that a reader
/// would otherwise have to infer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GraphFinding {
    /// Two documents share an identity and are not the same document.
    IdentityCollision {
        /// Which species.
        species: GraphSpecies,
        /// The shared identity, as spelled.
        identity: String,
    },
    /// An intent names an originating plan the graph does not hold.
    OriginatingPlanAbsent {
        /// The intent.
        intent: ApplyIntentId,
        /// The plan it named.
        plan: PlanReceiptId,
    },
    /// An intent names no originating plan at all.
    OriginatingPlanUnavailable {
        /// The intent.
        intent: ApplyIntentId,
    },
    /// An outcome answers an intent the graph does not hold.
    OutcomeWithoutIntent {
        /// The outcome.
        outcome: ApplyOutcomeId,
        /// The intent it named.
        intent: ApplyIntentId,
    },
    /// An outcome names an intent identity this reader could not read.
    OutcomeIntentUnreadable {
        /// The outcome.
        outcome: ApplyOutcomeId,
    },
    /// More than one outcome answers one intent.
    SupernumeraryOutcome {
        /// The intent.
        intent: ApplyIntentId,
        /// The outcome beyond the first.
        outcome: ApplyOutcomeId,
    },
    /// A document carried an identity this reader could not read.
    IdentityUnreadable {
        /// Which species.
        species: GraphSpecies,
    },
}

/// A document held beside another that claimed its identity.
///
/// Both are retained. The graph never resolves a collision by preferring one document, because
/// nothing in a document can say which of two claimants to an identity is the real one.
#[derive(Debug)]
pub enum CollidedDocument {
    /// A colliding plan document.
    Plan {
        /// The contested identity.
        identity: PlanReceiptId,
        /// The document held beside the one already keyed by it.
        model: Box<Reingested<RecordedPlanReceipt>>,
    },
    /// A colliding apply intent.
    Intent {
        /// The contested identity.
        identity: ApplyIntentId,
        /// The document held beside the one already keyed by it.
        model: Box<Reingested<RecordedApplyIntent>>,
    },
    /// A colliding apply outcome.
    Outcome {
        /// The contested identity.
        identity: ApplyOutcomeId,
        /// The document held beside the one already keyed by it.
        model: Box<Reingested<RecordedApplyOutcome>>,
    },
}

impl CollidedDocument {
    /// Which species the contested identity belongs to.
    #[must_use]
    pub const fn species(&self) -> GraphSpecies {
        match self {
            Self::Plan { .. } => GraphSpecies::Plan,
            Self::Intent { .. } => GraphSpecies::Intent,
            Self::Outcome { .. } => GraphSpecies::Outcome,
        }
    }

    /// The contested identity, as spelled.
    #[must_use]
    pub fn identity_hex(&self) -> String {
        match self {
            Self::Plan { identity, .. } => identity.hex(),
            Self::Intent { identity, .. } => identity.hex(),
            Self::Outcome { identity, .. } => identity.hex(),
        }
    }
}

/// One document the graph holds, with the provenance of the material that checked it.
///
/// Retains the exact document image because that is what classifying a second claimant to one
/// identity requires: an identity is minted per document, so two documents holding one is a
/// finding whenever their bytes differ, and both are retained. The projection word is kept for
/// reporting, never to excuse a difference.
#[derive(Debug)]
pub struct GraphNode<M: RecordedType> {
    model: Reingested<M>,
    signer: RecordedSignerTrust,
    projection: &'static str,
    image: Vec<u8>,
}

impl<M: RecordedType> GraphNode<M> {
    /// The typed model, still sealed.
    #[must_use]
    pub const fn model(&self) -> &Reingested<M> {
        &self.model
    }

    /// Where the verification material came from.
    #[must_use]
    pub const fn signer(&self) -> RecordedSignerTrust {
        self.signer
    }

    /// Which projection of the receipt-event this node holds.
    #[must_use]
    pub const fn projection(&self) -> &'static str {
        self.projection
    }
}

/// The correlated record set.
#[derive(Debug, Default)]
pub struct ReceiptGraph {
    plans: BTreeMap<PlanReceiptId, GraphNode<RecordedPlanReceipt>>,
    intents: BTreeMap<ApplyIntentId, GraphNode<RecordedApplyIntent>>,
    outcomes: BTreeMap<ApplyOutcomeId, GraphNode<RecordedApplyOutcome>>,
    collisions: Vec<CollidedDocument>,
    partials: Vec<PartialReceipt>,
    faults: Vec<ModelRefusal>,
}

fn provenance<T: SignerTrust>() -> RecordedSignerTrust {
    RecordedSignerTrust::of_token(T::TOKEN).unwrap_or(RecordedSignerTrust::SelfAsserted)
}

impl ReceiptGraph {
    /// An empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Take one plan document, with the exact bytes it was read from.
    ///
    /// The image is required because a second claimant to one identity is classified by
    /// [`same_identity_pair`], not by comparing models: two models can agree while the bytes
    /// they were read from do not.
    pub fn ingest_plan<P: Projection, T: SignerTrust>(
        &mut self,
        document: &Reingested<Receipt<PlanReceipt, P, T>>,
        image: &[u8],
    ) {
        let Some(id) = document.receipt_id() else {
            return;
        };
        match document.model() {
            Ok(model) => match self.plans.get(&id) {
                Some(held) => {
                    if same_identity_pair(&held.image, image) == SameIdentityPair::Divergent {
                        self.collisions.push(CollidedDocument::Plan {
                            identity: id,
                            model: Box::new(model),
                        });
                    }
                }
                None => {
                    self.plans.insert(
                        id,
                        GraphNode {
                            model,
                            signer: provenance::<T>(),
                            projection: P::TOKEN,
                            image: image.to_vec(),
                        },
                    );
                }
            },
            Err(fault) => self.faults.push(fault),
        }
    }

    /// Take one apply intent, with the exact bytes it was read from.
    pub fn ingest_intent<P: Projection, T: SignerTrust>(
        &mut self,
        document: &Reingested<Receipt<ApplyIntent, P, T>>,
        image: &[u8],
    ) {
        let Some(id) = document.receipt_id() else {
            return;
        };
        match document.model() {
            Ok(model) => match self.intents.get(&id) {
                Some(held) => {
                    if same_identity_pair(&held.image, image) == SameIdentityPair::Divergent {
                        self.collisions.push(CollidedDocument::Intent {
                            identity: id,
                            model: Box::new(model),
                        });
                    }
                }
                None => {
                    self.intents.insert(
                        id,
                        GraphNode {
                            model,
                            signer: provenance::<T>(),
                            projection: P::TOKEN,
                            image: image.to_vec(),
                        },
                    );
                }
            },
            Err(fault) => self.faults.push(fault),
        }
    }

    /// Take one apply outcome, with the exact bytes it was read from.
    pub fn ingest_outcome<P: Projection, T: SignerTrust>(
        &mut self,
        document: &Reingested<Receipt<ApplyOutcome, P, T>>,
        image: &[u8],
    ) {
        let Some(id) = document.receipt_id() else {
            return;
        };
        match document.model() {
            Ok(model) => match self.outcomes.get(&id) {
                Some(held) => {
                    if same_identity_pair(&held.image, image) == SameIdentityPair::Divergent {
                        self.collisions.push(CollidedDocument::Outcome {
                            identity: id,
                            model: Box::new(model),
                        });
                    }
                }
                None => {
                    self.outcomes.insert(
                        id,
                        GraphNode {
                            model,
                            signer: provenance::<T>(),
                            projection: P::TOKEN,
                            image: image.to_vec(),
                        },
                    );
                }
            },
            Err(fault) => self.faults.push(fault),
        }
    }

    /// Take one document that did not complete.
    pub fn ingest_partial(&mut self, partial: PartialReceipt) {
        self.partials.push(partial);
    }

    /// Every correlation, in identity order.
    ///
    /// Derived on demand from the held identities alone, which is what makes the answer
    /// independent of the order documents arrived in.
    #[must_use]
    pub fn edges(&self) -> Vec<ReceiptEdge> {
        let mut out = Vec::new();
        for (intent, node) in &self.intents {
            for plan in node.model().origin_receipts() {
                if self.plans.contains_key(&plan) {
                    out.push(ReceiptEdge::PlanToIntent {
                        plan,
                        intent: *intent,
                    });
                }
            }
        }
        for (outcome, node) in &self.outcomes {
            if let Some(intent) = node.model().intent()
                && self.intents.contains_key(&intent)
            {
                out.push(ReceiptEdge::IntentToOutcome {
                    intent,
                    outcome: *outcome,
                });
            }
        }
        out.sort();
        out
    }

    /// Everything a report must surface, in a stable order.
    #[must_use]
    pub fn findings(&self) -> Vec<GraphFinding> {
        let mut out = Vec::new();
        for collision in &self.collisions {
            out.push(GraphFinding::IdentityCollision {
                species: collision.species(),
                identity: collision.identity_hex(),
            });
        }
        for (intent, node) in &self.intents {
            let origins = node.model().origin_receipts();
            if origins.is_empty() {
                out.push(GraphFinding::OriginatingPlanUnavailable { intent: *intent });
            }
            for plan in origins {
                if !self.plans.contains_key(&plan) {
                    out.push(GraphFinding::OriginatingPlanAbsent {
                        intent: *intent,
                        plan,
                    });
                }
            }
        }
        let mut answered: BTreeMap<ApplyIntentId, u32> = BTreeMap::new();
        for (outcome, node) in &self.outcomes {
            match node.model().intent() {
                Some(intent) => {
                    if self.intents.contains_key(&intent) {
                        let seen = answered.entry(intent).or_default();
                        *seen = seen.saturating_add(1);
                        if *seen > 1 {
                            out.push(GraphFinding::SupernumeraryOutcome {
                                intent,
                                outcome: *outcome,
                            });
                        }
                    } else {
                        out.push(GraphFinding::OutcomeWithoutIntent {
                            outcome: *outcome,
                            intent,
                        });
                    }
                }
                None => out.push(GraphFinding::OutcomeIntentUnreadable { outcome: *outcome }),
            }
        }
        out.sort();
        out
    }

    /// Whether an intent has a recorded outcome.
    ///
    /// The one mint of a missing outcome: it is reached by correlation, is never a document, and
    /// says only that no outcome answering this intent is held.
    #[must_use]
    pub fn outcome_for(&self, intent: ApplyIntentId) -> OutcomeAvailability {
        for node in self.outcomes.values() {
            if node.model().intent() == Some(intent) {
                return OutcomeAvailability::Recorded(node.model().clone());
            }
        }
        OutcomeAvailability::Missing(MissingOutcome::of(intent))
    }

    /// Every plan the graph holds.
    #[must_use]
    pub fn plans(&self) -> &BTreeMap<PlanReceiptId, GraphNode<RecordedPlanReceipt>> {
        &self.plans
    }

    /// Every intent the graph holds.
    #[must_use]
    pub fn intents(&self) -> &BTreeMap<ApplyIntentId, GraphNode<RecordedApplyIntent>> {
        &self.intents
    }

    /// Every outcome the graph holds.
    #[must_use]
    pub fn outcomes(&self) -> &BTreeMap<ApplyOutcomeId, GraphNode<RecordedApplyOutcome>> {
        &self.outcomes
    }

    /// Every document held beside another that claimed its identity.
    #[must_use]
    pub fn collisions(&self) -> &[CollidedDocument] {
        &self.collisions
    }

    /// Every document that did not complete.
    #[must_use]
    pub fn partials(&self) -> &[PartialReceipt] {
        &self.partials
    }

    /// Every record set that parsed and did not close over itself.
    #[must_use]
    pub fn faults(&self) -> &[ModelRefusal] {
        &self.faults
    }
}
