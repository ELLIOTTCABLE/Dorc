//! `dorc-receipt` — the durable receipt family: recorded models, the exact
//! `dorc-receipt/1` grammar, identities, limits, projections, and the reader and writer
//! states.
//!
//! # What this crate is, mechanically
//!
//! Three document species — a plan receipt, an apply intent, and an apply outcome — share
//! one physical grammar and two projections. A plain document carries structural fields
//! only. A rich document additionally carries exactly one encrypted region, which maps back
//! to structural slots from the region's side.
//!
//! # Determinism
//!
//! This crate depends on `sha2` and nothing else. It reads no clock, no environment, no
//! filesystem, and no operating-system randomness, so it is safe inside the deterministic
//! kernel and `dorc-plan` may depend on it. Every capability that would break that —
//! signing, sealing, opening, minting a document identity, publishing — is a trait in
//! [`capability`] and an implementation in `dorc-receipt-crypto`, which depends on this
//! crate and which this crate must never depend on.
//!
//! # What the states mean
//!
//! Writing runs `DraftReceipt -> SerializedReceipt -> SignedReceipt -> PublishedReceipt`,
//! each transition consuming its predecessor. Reading runs `BoundedReceiptBytes ->
//! LocatedReceiptEnvelope -> ReceiptSignatureChecked -> ParsedReceiptSkeleton -> Receipt`,
//! and never skips a step. A [`reader::Receipt`] is format-complete for its projection; it
//! is not a statement that its contents are so, current, or safe to share.
//!
//! Everything read back wears [`reingested::Reingested`], which has no route to a live
//! value.

pub mod capability;
pub mod format;
pub mod grammar;
pub mod ids;
pub mod limits;
pub mod model;
pub mod reader;
pub mod reingested;
pub mod writer;

pub use format::{RefusalReason, Skeleton, SkeletonRecord};
pub use grammar::{FieldType, RecordKind};
pub use limits::ReceiptLimits;
pub use model::{
    ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Projection, Rich, SelfAssertedReceiptSigner,
    SignerTrust, Species, TrustedReceiptSigner,
};
pub use reader::{BoundedReceiptBytes, PartialReceipt, ReadPlain, Receipt, read_plain};
pub use reingested::{RecordedCurrent, RecordedInfluence, Reingested};
pub use writer::{DraftReceipt, PublishedReceipt, SerializedReceipt, SignedReceipt};
