//! What `dorc why` prints from a receipt it read back.
//!
//! # A listing, not prose
//!
//! Every word this emits is either a field name from the receipt grammar or a token from one of
//! the recorded closed vocabularies. It authors no sentence and mints no register, which is
//! deliberate: the durable exists for degraded conditions — a drifted tree, an old binary, a
//! vendor handoff — and the thing a reader wants there is what the document SAYS, in the
//! document's own words.
//!
//! It is a MODE-OWNED stdout species (`cli/CLAUDE.md stdout-contract`), the way `dorc bundle`'s
//! archive is: nothing interleaves it with a plan render.
//!
//! # Report-only, by type
//!
//! Everything below reads a `Reingested<…>`, whose seal is what makes a recorded value unable to
//! become a live one. Nothing here converts, and nothing here decides.
//!
//! # Sink encoding
//!
//! The opaque region carries bytes somebody else wrote — a book's own shell text, a path from the
//! filesystem it was acquired from — so every one of them leaves through the terminal encoder
//! (`sinv-sink-encoding`). The skeleton's own fields are closed tokens, digests and counts, which
//! have no other spelling to escape.

use dorc_receipt::graph::{GraphFinding, ReceiptEdge, ReceiptGraph};
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, PlanReceipt, Rich, TrustedReceiptSigner};
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::Receipt;
use dorc_receipt::reingested::Reingested;
use dorc_receipt::tokens::ClosedToken;

/// How many bytes of one opaque field a listing shows.
///
/// A display cap, not a bound on what was read: the reader already bounded the whole region, and
/// this is the width past which a terminal line stops being one.
const FIELD_DISPLAY_CAP: usize = 240;

/// The recorded plan document, as lines.
///
/// The document is TRUSTED here by type — the signature checked against material controller
/// policy named — and that is the only tier this seat renders. An untrusted or partial read is a
/// different answer and its caller reports it as one, because promoting a field because it looked
/// plausible is exactly what the reader's states exist to refuse.
#[must_use]
pub fn recorded_plan_listing(
    document: &Reingested<Receipt<PlanReceipt, Rich, TrustedReceiptSigner>>,
) -> String {
    let mut out = String::new();
    push_line(&mut out, &format!("receipt {}", document.receipt_id_hex()));
    push_line(
        &mut out,
        &format!("signing-key {}", document.signing_key_id_hex()),
    );
    push_line(&mut out, &format!("records {}", document.record_count()));

    match document.model() {
        Ok(model) => {
            for source in model.sources() {
                push_line(
                    &mut out,
                    &format!(
                        "source {} {} {} {}",
                        source.ordinal(),
                        source.role().token(),
                        source.digest(),
                        source.bytes()
                    ),
                );
            }
            push_line(&mut out, &format!("sites {}", model.site_count()));
            for site in model.sites() {
                push_line(
                    &mut out,
                    &format!(
                        "site {} {}",
                        site.disposition().token(),
                        site.account().token()
                    ),
                );
            }
            push_line(&mut out, &format!("regions {}", model.region_count()));
            if let Some(presented) = model.presented_plan() {
                push_line(&mut out, &format!("presented-plan {}", presented.hex()));
            }
        }
        // A document whose record stream does not close over itself is still a document whose
        // signature checked, so the identity lines above stand and this says what is missing
        // rather than emitting nothing.
        Err(refusal) => push_line(&mut out, &format!("model-unavailable {refusal:?}")),
    }

    for line in opaque_lines(document) {
        push_line(&mut out, &line);
    }
    out
}

/// Every opaque field the validated region carried, in record then tag order.
///
/// Walked by RECORD ORDINAL rather than paired with the skeleton rows above: a position is
/// range-checked and never sense-checked, so pairing them would let a wrong ordinal enrich
/// whichever row shared its integer while the listing still read cleanly. These lines say which
/// record they came from and let a reader do the joining.
fn opaque_lines(
    document: &Reingested<Receipt<PlanReceipt, Rich, TrustedReceiptSigner>>,
) -> Vec<String> {
    let mut lines = Vec::new();
    for record in 0..u64::try_from(document.record_count()).unwrap_or(0) {
        for tag in OpaqueFieldTag::ALL {
            if let Some(bytes) = document.detail(record, tag) {
                let shown = dorc_aid::display::encode_foreign(
                    &String::from_utf8_lossy(bytes),
                    FIELD_DISPLAY_CAP,
                );
                lines.push(format!("opaque {record} {} {shown}", tag.token()));
            }
        }
    }
    lines
}

/// The recorded apply intent, as lines.
#[must_use]
pub fn recorded_intent_listing(
    document: &Reingested<Receipt<ApplyIntent, Rich, TrustedReceiptSigner>>,
) -> String {
    let mut out = String::new();
    push_line(&mut out, &format!("receipt {}", document.receipt_id_hex()));
    push_line(
        &mut out,
        &format!("signing-key {}", document.signing_key_id_hex()),
    );
    match document.model() {
        Ok(model) => {
            push_line(
                &mut out,
                &format!("assignments {}", model.assignment_count()),
            );
            for plan in model.origin_receipts() {
                push_line(&mut out, &format!("originating-plan {}", plan.hex()));
            }
        }
        Err(refusal) => push_line(&mut out, &format!("model-unavailable {refusal:?}")),
    }
    out
}

/// The recorded apply outcome, as lines.
#[must_use]
pub fn recorded_outcome_listing(
    document: &Reingested<Receipt<ApplyOutcome, Rich, TrustedReceiptSigner>>,
) -> String {
    let mut out = String::new();
    push_line(&mut out, &format!("receipt {}", document.receipt_id_hex()));
    push_line(
        &mut out,
        &format!("signing-key {}", document.signing_key_id_hex()),
    );
    match document.model() {
        Ok(model) => {
            push_line(&mut out, &format!("sites {}", model.site_count()));
            if let Some(intent) = model.intent() {
                push_line(&mut out, &format!("answers-intent {}", intent.hex()));
            }
        }
        Err(refusal) => push_line(&mut out, &format!("model-unavailable {refusal:?}")),
    }
    out
}

/// The correlations and findings a graph over several documents produced, as lines.
///
/// Findings are not errors in any document: each is a shape of the record SET that a reader would
/// otherwise have to infer, and the receipt architecture requires them to stay explicit rather
/// than be rounded into a story about what probably happened.
#[must_use]
pub fn recorded_graph_listing(graph: &ReceiptGraph) -> String {
    let mut out = String::new();
    for edge in graph.edges() {
        match edge {
            ReceiptEdge::PlanToIntent { plan, intent } => push_line(
                &mut out,
                &format!("edge plan {} apply-intent {}", plan.hex(), intent.hex()),
            ),
            ReceiptEdge::IntentToOutcome { intent, outcome } => push_line(
                &mut out,
                &format!(
                    "edge apply-intent {} apply-outcome {}",
                    intent.hex(),
                    outcome.hex()
                ),
            ),
        }
    }
    for finding in graph.findings() {
        push_line(&mut out, &format!("finding {}", finding_line(&finding)));
    }
    for partial in graph.partials() {
        push_line(&mut out, &format!("partial {:?}", partial.reason()));
    }
    out
}

/// One finding, in the graph's own closed vocabulary.
fn finding_line(finding: &GraphFinding) -> String {
    match finding {
        GraphFinding::IdentityCollision { species, identity } => {
            format!("identity-collision {} {identity}", species.token())
        }
        GraphFinding::OriginatingPlanAbsent { intent, plan } => {
            format!("originating-plan-absent {} {}", intent.hex(), plan.hex())
        }
        GraphFinding::OriginatingPlanUnavailable { intent } => {
            format!("originating-plan-unavailable {}", intent.hex())
        }
        GraphFinding::OutcomeWithoutIntent { outcome, intent } => {
            format!("outcome-without-intent {} {}", outcome.hex(), intent.hex())
        }
        GraphFinding::OutcomeIntentUnreadable { outcome } => {
            format!("outcome-intent-unreadable {}", outcome.hex())
        }
        GraphFinding::SupernumeraryOutcome { intent, outcome } => {
            format!("supernumerary-outcome {} {}", intent.hex(), outcome.hex())
        }
        GraphFinding::IdentityUnreadable { species } => {
            format!("identity-unreadable {}", species.token())
        }
    }
}

fn push_line(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}

/// What one bounded walk of the receipt store produced, as a value.
///
/// Every act that needed a filesystem or a key is already spent by the time this exists, which is
/// what lets the decision above it — which documents this invocation lists, and what it says when
/// the store cannot answer — be a pure function both drivers run
/// (`cli/CLAUDE.md lib-target-is-a-loom-seam`: values cross the seam, queries do not).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StoreReading {
    documents: Vec<RecordedDocument>,
    cohort: Vec<String>,
    graph: String,
}

impl StoreReading {
    /// Bind one walk's documents, the identities sharing its greatest order, and its graph lines.
    #[must_use]
    pub const fn of(documents: Vec<RecordedDocument>, cohort: Vec<String>, graph: String) -> Self {
        Self {
            documents,
            cohort,
            graph,
        }
    }

    /// Every recognized document, in the store's own order.
    #[must_use]
    pub fn documents(&self) -> &[RecordedDocument] {
        &self.documents
    }

    /// The identities sharing the store's greatest order — the ONE selection a store offers.
    #[must_use]
    pub fn cohort(&self) -> &[String] {
        &self.cohort
    }

    /// The correlations and findings a graph over the whole store produced.
    #[must_use]
    pub fn graph(&self) -> &str {
        &self.graph
    }
}

/// One recognized store entry, and what reading it produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedDocument {
    receipt_id: String,
    listing: Option<String>,
}

impl RecordedDocument {
    /// A document that verified and opened, with the lines it yielded.
    #[must_use]
    pub const fn read(receipt_id: String, listing: String) -> Self {
        Self {
            receipt_id,
            listing: Some(listing),
        }
    }

    /// A recognized entry that did not yield a trusted document.
    ///
    /// Typed absence rather than an empty listing: "the store holds this identity" and "this
    /// identity had something to say" are different facts, and a caller counting the first from
    /// the second would report a store as empty because its documents would not open.
    #[must_use]
    pub const fn unread(receipt_id: String) -> Self {
        Self {
            receipt_id,
            listing: None,
        }
    }

    /// The identity the store filed this entry under.
    #[must_use]
    pub fn receipt_id(&self) -> &str {
        &self.receipt_id
    }

    /// The lines this document yielded, where it yielded any.
    #[must_use]
    pub fn listing(&self) -> Option<&str> {
        self.listing.as_deref()
    }
}
