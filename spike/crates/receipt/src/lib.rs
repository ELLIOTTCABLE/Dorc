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
//!
//! # The seals, as compile-fail pins
//!
//! Each pin below is paired with the positive control that proves it fails for the stated
//! reason rather than because the example was malformed.
//!
//! A document identity is per-species, so one cannot stand in for another:
//!
//! ```
//! use dorc_receipt::ids::{PlanReceiptId, ApplyIntentId};
//! let text = "a".repeat(64);
//! let plan: Option<PlanReceiptId> = PlanReceiptId::of_hex(&text);
//! let intent: Option<ApplyIntentId> = ApplyIntentId::of_hex(&text);
//! assert!(plan.is_some() && intent.is_some());
//! ```
//!
//! ```compile_fail
//! use dorc_receipt::ids::{PlanReceiptId, ApplyIntentId};
//! let plan = PlanReceiptId::of_hex(&"a".repeat(64)).unwrap();
//! let _crossed: ApplyIntentId = plan;
//! ```
//!
//! The two provider roles never alias:
//!
//! ```compile_fail
//! use dorc_receipt::ids::{SigningKeyId, EncryptionKeyId};
//! let signing = SigningKeyId::of_public_material(b"m");
//! let _crossed: EncryptionKeyId = signing;
//! ```
//!
//! An image identity is not a presentation identity:
//!
//! ```compile_fail
//! use dorc_receipt::ids::{ApplyArtifactImageId, PresentedPlanId};
//! let image = ApplyArtifactImageId::of_canonical_image(b"bytes");
//! let _crossed: PresentedPlanId = image;
//! ```
//!
//! The species, projection and provenance traits are sealed, so no outside type joins the
//! set:
//!
//! ```compile_fail
//! #[derive(Debug, Clone, Copy)]
//! struct MySpecies;
//! impl dorc_receipt::Species for MySpecies {
//!     const TOKEN: &'static str = "mine";
//!     const KINDS: &'static [dorc_receipt::RecordKind] = &[];
//! }
//! ```
//!
//! ```compile_fail
//! #[derive(Debug, Clone, Copy)]
//! struct MyTrust;
//! impl dorc_receipt::SignerTrust for MyTrust {
//!     const TOKEN: &'static str = "mine";
//! }
//! ```
//!
//! A recorded model is buildable as a bare value, which is how a projection makes one:
//!
//! ```
//! use dorc_receipt::plan::RecordedPlanReceipt;
//! assert!(RecordedPlanReceipt::of_records(&[]).is_err(), "a document names its invocation");
//! ```
//!
//! But nothing can wrap one to look as though it came back from a document:
//!
//! ```compile_fail
//! use dorc_receipt::Reingested;
//! use dorc_receipt::plan::RecordedPlanReceipt;
//! fn forge(model: RecordedPlanReceipt) -> Reingested<RecordedPlanReceipt> {
//!     Reingested::seal(model)
//! }
//! ```
//!
//! And a sealed value offers no route back out — no unwrap:
//!
//! ```compile_fail
//! use dorc_receipt::Reingested;
//! use dorc_receipt::plan::RecordedPlanReceipt;
//! fn take(value: Reingested<RecordedPlanReceipt>) -> RecordedPlanReceipt {
//!     value.into_inner()
//! }
//! ```
//!
//! no dereference:
//!
//! ```compile_fail
//! use dorc_receipt::Reingested;
//! use dorc_receipt::plan::RecordedPlanReceipt;
//! fn take(value: &Reingested<RecordedPlanReceipt>) -> &RecordedPlanReceipt {
//!     &**value
//! }
//! ```
//!
//! and no generic accessor, which is the shape that would make the seal depend on which types
//! join the recorded set rather than on the wrapper itself:
//!
//! ```compile_fail
//! use dorc_receipt::Reingested;
//! use dorc_receipt::plan::RecordedPlanReceipt;
//! fn take(value: &Reingested<RecordedPlanReceipt>) -> &RecordedPlanReceipt {
//!     value.as_report()
//! }
//! ```
//!
//! An outside type cannot join the recorded set:
//!
//! ```compile_fail
//! #[derive(Debug)]
//! struct Mine;
//! impl dorc_receipt::reingested::RecordedType for Mine {}
//! ```
//!
//! A missing outcome is reached by correlation, so a caller holding an intent identity cannot
//! assert one:
//!
//! ```compile_fail
//! use dorc_receipt::ids::ApplyIntentId;
//! use dorc_receipt::outcome::MissingOutcome;
//! fn claim(intent: ApplyIntentId) -> MissingOutcome {
//!     MissingOutcome::of(intent)
//! }
//! ```
//!
//! An invocation's own spelling and a destination a standup resolved are different facts, and no
//! conversion joins them. The refusal below fails on `E0624` — the resolved destination's reader
//! is crate-private — so PRIVACY is the mechanism, and
//! `crate_boundary.rs`'s `a_resolved_destination_is_readable_at_exactly_one_slot` is what keeps
//! that true; this pin alone would prove only that something did not compile. Both legitimate
//! constructions compile:
//!
//! ```
//! use dorc_receipt::project::InvocationTarget;
//! let _spelled = InvocationTarget::Spelled(b"web1.example.net".to_vec());
//! let _fleet = InvocationTarget::NotOne;
//! ```
//!
//! ```compile_fail
//! use dorc_receipt::dispatch::ResolvedApplyContext;
//! use dorc_receipt::project::InvocationTarget;
//! fn launder(context: ResolvedApplyContext) -> InvocationTarget {
//!     InvocationTarget::Spelled(context.destination().bytes().to_vec())
//! }
//! ```
//!
//! A recorded influence grade reads at the conservative point when a document says nothing
//! usable:
//!
//! ```
//! use dorc_receipt::RecordedInfluence;
//! assert_eq!(
//!     RecordedInfluence::of_token(None),
//!     RecordedInfluence::MostInfluenced
//! );
//! ```
//!
//! That it never becomes a live account cannot be pinned here: this crate names `sha2` and
//! nothing else, so a doctest reaching for the live type fails on the unresolved path and would
//! go on passing if the conversion were added. The pin lives in `dorc-plan`, which sees both
//! crates, beside the one proving a recorded disposition is not a live one.
//!
//! A checked state cannot be built by a caller; only the crate's own check produces one:
//!
//! ```compile_fail
//! use dorc_receipt::reader::ReceiptSignatureChecked;
//! use dorc_receipt::TrustedReceiptSigner;
//! let _forged: ReceiptSignatureChecked<TrustedReceiptSigner> = ReceiptSignatureChecked {
//!     body: Vec::new(),
//!     skeleton: Vec::new(),
//!     armor: None,
//!     trust: core::marker::PhantomData,
//! };
//! ```
//!
//! A signed document is not `Clone`, so a publication cannot be replayed from a copy:
//!
//! ```compile_fail
//! use dorc_receipt::{DraftReceipt, Plain, PlanReceipt, Skeleton};
//! fn duplicate(signed: dorc_receipt::SignedReceipt<PlanReceipt, Plain>) {
//!     let _second = signed.clone();
//! }
//! ```
//!
//! # The pre-dispatch chain, as compile-fail pins
//!
//! One publication authorizes one dispatch OF THE INTENT IT WAS EARNED FOR, and that is carried by
//! ownership rather than by a check: accounting consumes the intent, the publication owns the
//! accounting, and the permit takes no second argument. So the failures below are compile errors
//! rather than refusals a runtime test could observe.
//!
//! MEASURED, each against its un-fenced form, because "it did not compile" is not on its own a
//! statement about what was refused: `E0061` (an argument with nowhere to go), `E0451` twice (a
//! private field), and `E0433`/`E0425` (a name that is gone). The positive control at the end is
//! what says the module and its public spellings still resolve.
//!
//! Publication A cannot dispatch intent B, because `permit` has nowhere to put a B:
//!
//! ```compile_fail
//! use dorc_receipt::dispatch::{PreparedApplyIntent, PublishedApplyIntentV1};
//! fn cross(published: PublishedApplyIntentV1, other: PreparedApplyIntent) {
//!     let _permit = published.permit(other);
//! }
//! ```
//!
//! An image witness cannot be lifted off one intent and set beside another, because there is no
//! witness value apart from the intent it accounted:
//!
//! ```compile_fail
//! use dorc_receipt::dispatch::{AccountedApplyIntent, PreparedApplyIntent};
//! fn relabel(accounted: AccountedApplyIntent, other: PreparedApplyIntent) -> AccountedApplyIntent {
//!     AccountedApplyIntent { intent: other }
//! }
//! ```
//!
//! No caller can mint a publication without a placement having been called through it — the mint
//! is private and the only entry is the consuming one:
//!
//! ```compile_fail
//! use dorc_receipt::dispatch::{AccountedApplyIntent, PublishedApplyIntentV1, RequiredPlacementLanding};
//! use dorc_receipt::ids::ApplyIntentId;
//! fn forge(accounted: AccountedApplyIntent, id: ApplyIntentId, landing: RequiredPlacementLanding)
//!     -> PublishedApplyIntentV1
//! {
//!     PublishedApplyIntentV1 { id, landing, accounted }
//! }
//! ```
//!
//! And there is no second route to a permit at all — the bypass capability and the two-armed gate
//! are gone, not merely unused:
//!
//! ```compile_fail
//! fn bypass() -> dorc_receipt::dispatch::ConfiguredReceiptBypass {
//!     dorc_receipt::dispatch::ConfiguredReceiptBypass::configured()
//! }
//! ```
//!
//! The positive control for all four, so they fail for the stated reason rather than because the
//! module moved: the chain's own public spellings resolve.
//!
//! ```
//! use dorc_receipt::dispatch::{
//!     AccountedApplyIntent, PreparedApplyIntent, PublishedApplyIntentV1, RequiredPlacementLanding,
//! };
//! use dorc_receipt::ids::Sha256Digest;
//! let landing = RequiredPlacementLanding::of(
//!     Sha256Digest::over("doc", b"bytes"),
//!     "required-local-v1",
//! );
//! assert_eq!(landing.policy_identity(), "required-local-v1");
//! ```

pub mod apply;
pub mod capability;
pub mod context;
pub mod dispatch;
pub mod durable_locator;
pub mod format;
pub mod grammar;
pub mod graph;
pub mod ids;
pub mod image;
pub mod limits;
pub mod model;
pub mod order;
pub mod outcome;
pub mod overlay;
pub mod plan;
pub mod project;
pub mod projection;
pub mod reader;
pub mod reingested;
pub mod report;
pub mod rows;
pub mod source;
pub mod tokens;
pub mod writer;

pub use format::{RefusalReason, Skeleton, SkeletonRecord};
pub use grammar::{FieldType, RecordKind};
pub use image::{ApplyArtifactImage, ApplyImageEntry, ImageRefusal, RecordedApplyPath};
pub use limits::ReceiptLimits;
pub use model::{
    ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Projection, Rich, SelfAssertedReceiptSigner,
    SignerTrust, Species, TrustedReceiptSigner,
};
pub use order::{ControllerClock, ReceiptOrderToken};
pub use reader::{
    BoundedReceiptBytes, PartialReceipt, ReadPlain, ReadRich, Receipt, read_plain, read_rich,
};
pub use reingested::{
    ReDerived, ReDerivedDisposition, RecordedCurrent, RecordedInfluence, Reingested,
};
pub use writer::{DraftReceipt, PublishedReceipt, SerializedReceipt, SignedReceipt};
