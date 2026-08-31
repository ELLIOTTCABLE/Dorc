//! The structure edge-families (`30V` §3): navigated, never projected along.
//!
//! Four families are named by the model. Two are POPULATED from a v1 plan receipt — the receipt
//! graph and the locus DAG — and two are not: derivation operands and program topology reach the
//! durable in families `dorc_receipt::report` does not project, so they surface as typed absence
//! carrying `CarrierAbsence::ReportApiLacks` rather than as empty vectors nobody explains
//! (`core/CLAUDE.md a-record-says-what-its-population-holds`: a field says what its population
//! actually holds).

use dorc_receipt::durable_locator::RecordedStageKind;
use dorc_receipt::rows::RecordedSite;

use crate::datum::{CorrelationFact, SourceRef};
use crate::known::Known;

/// Every edge-family, together.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Structure {
    receipts: Vec<CorrelationFact>,
    loci: LocusDag,
}

impl Structure {
    /// Bind the two families a v1 plan receipt can populate.
    #[must_use]
    pub const fn of(receipts: Vec<CorrelationFact>, loci: LocusDag) -> Self {
        Self { receipts, loci }
    }

    /// The receipt graph's typed correlations, as walked from the root.
    #[must_use]
    pub fn receipts(&self) -> &[CorrelationFact] {
        &self.receipts
    }

    /// The locus DAG.
    #[must_use]
    pub const fn loci(&self) -> &LocusDag {
        &self.loci
    }
}

/// One site's recorded provenance, as a walkable DAG with its per-namespace projections.
///
/// Chains are stored FLAT with explicit edges rather than nested, so a consumer walks the same
/// shape whatever the chain's depth, and a stage with two origins stays representable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LocusDag {
    nodes: Vec<Locus>,
    edges: Vec<LocusEdge>,
}

impl LocusDag {
    /// Bind nodes and the edges between them.
    #[must_use]
    pub const fn of(nodes: Vec<Locus>, edges: Vec<LocusEdge>) -> Self {
        Self { nodes, edges }
    }

    /// Every locus, in walk order.
    #[must_use]
    pub fn nodes(&self) -> &[Locus] {
        &self.nodes
    }

    /// The head-to-origin edges.
    #[must_use]
    pub fn edges(&self) -> &[LocusEdge] {
        &self.edges
    }
}

/// One stage of one site's provenance, in one namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locus {
    /// The site this stage belongs to.
    pub site: RecordedSite,
    /// Which stage kind.
    pub stage: RecordedStageKind,
    /// Its position on the chain, head first.
    pub index: u32,
    /// Which namespace the address below is spoken in (`30V` §2
    /// rul-line-addresses-are-namespaced).
    pub namespace: Namespace,
    /// The address, as far as this carrier can spell one.
    pub address: Known<LocusAddress>,
    /// How the current tree stands against the recorded source this stage names.
    pub agreement: SourceAgreement,
}

/// Which namespace an address is spoken in.
///
/// The user's own namespace and the recorded one are kept apart because they can disagree, and
/// stating a recorded address as though it were current is the mis-attribution `271:rul-sin-ordering`
/// ranks worst.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    /// As the document recorded it.
    Recorded,
    /// As the current tree stands.
    Current,
}

/// An address, as far as the carrier can spell one.
///
/// DELIBERATELY NOT `file.sh:N`, which `30V` §2 rul-line-addresses-are-namespaced sets as the
/// minimum: a source's PATH has no exit from `dorc_receipt::report` (the raw-detail accessor is
/// crate-private by design), and the byte-offset-to-line map is crate-private too, so neither half
/// of `file.sh:N` is derivable here. The ordinal-and-span pair is what IS true, and the missing
/// halves are audited as report-API holes rather than guessed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocusAddress {
    /// Which acquired source, by ordinal.
    pub source: SourceRef,
    /// The byte range it names, in the acquired byte domain.
    pub span: (u64, u64),
}

/// How the current tree stands against a recorded source.
///
/// THREE states, and the absent fourth is the point: `30V` §3's `né <oldline>` moved-line state is
/// NOT derivable at v1, because the address rule refuses moved-line matching outright
/// (`30R:receipt-rooted-attention-and-cli` — Dorc never guesses that a moved line is the same
/// operation), so nothing could ever mint it. An arm nothing mints would read as a promise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAgreement {
    /// Current and recorded agree byte-for-byte.
    Agrees,
    /// They differ. WHERE they differ is not said — saying it is the leak.
    Differs,
    /// No comparison was made: no observation was supplied for this source.
    NotCompared,
}

/// One edge of the locus DAG, by node position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocusEdge {
    /// The nearer-to-head node.
    pub from: usize,
    /// The nearer-to-origin node.
    pub to: usize,
}
