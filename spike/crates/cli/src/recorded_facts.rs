//! The seat that turns one selected receipt into `RecordedWhyFacts`.
//!
//! # The split this exists to keep
//!
//! `dorc_receipt::report` is pure: it opens no file, resolves no root, and holds no key. Every one
//! of those is the CLI's, and this module is where the two meet — the edge does its reading, hands
//! the outcomes over as data, and the receipt crate does the decomposition.
//!
//! Nothing here renders. The model is the handoff to a later why-surface conductor, and the
//! current listing output is deliberately untouched: this seat exists, is fed by the real reading
//! path, and produces the model for all three root selectors, which is what the next builder needs
//! in order to start from something real.

use dorc_receipt::model::{PlanReceipt, Rich, TrustedReceiptSigner};
use dorc_receipt::plan::RecordedPlanReceipt;
use dorc_receipt::reader::Receipt;
use dorc_receipt::reingested::Reingested;
use dorc_receipt::report::{
    AuthenticationState, CurrentSourceReading, DetailState, RecordedDocumentId, RecordedWhyFacts,
    RequestedAddress, SiblingState, SourceObservation, WhyFactsInput, derive,
};

/// One decoded plan document, with everything the edge learned about it.
///
/// A struct rather than loose arguments because every field is something the EDGE established and
/// the pure model must not re-establish: which document this is, what verification said, whether
/// the region opened. A model that took fewer of these would be one that guessed the rest.
#[derive(Debug)]
pub struct SelectedRoot {
    /// The document, decoded and sealed.
    pub receipt: Reingested<Receipt<PlanReceipt, Rich, TrustedReceiptSigner>>,
    /// Its own model.
    pub model: Reingested<RecordedPlanReceipt>,
    /// Its identity.
    pub identity: RecordedDocumentId,
    /// The store order it was filed under, as spelled.
    pub order: String,
    /// What outer verification said.
    pub authentication: AuthenticationState,
    /// Whether the grouped detail region opened.
    pub detail: DetailState,
}

/// What the edge found when it looked for one acquired source in the current tree.
///
/// The digest comparison stays the CALLER's: it owns the hash, and handing the answer over rather
/// than the function is what keeps a digest implementation off the pure model's path.
#[derive(Debug)]
pub struct ObservedSource {
    /// Which acquired source, by ordinal.
    pub ordinal: u32,
    /// What the edge read, or why it read nothing.
    pub reading: CurrentSourceReading,
    /// Whether what it read still hashes to the recorded digest.
    pub matches_digest: bool,
}

/// Derive the inert model for one rooted question.
///
/// `reached` and `siblings` come from the graph the edge already built: this seat performs no
/// traversal of its own, because a second traversal is a second answer to which documents the
/// question needed.
#[must_use]
pub fn facts_for(
    root: &SelectedRoot,
    reached: Vec<RecordedDocumentId>,
    siblings: Vec<SiblingState>,
    observations: Vec<ObservedSource>,
    address: Option<RequestedAddress>,
) -> RecordedWhyFacts {
    derive(&WhyFactsInput {
        root: &root.receipt,
        model: &root.model,
        identity: root.identity.clone(),
        order: root.order.clone(),
        authentication: root.authentication,
        detail: root.detail,
        reached,
        siblings,
        observations: observations
            .into_iter()
            .map(|observed| SourceObservation {
                ordinal: observed.ordinal,
                reading: observed.reading,
                matches_digest: observed.matches_digest,
            })
            .collect(),
        address,
    })
}
