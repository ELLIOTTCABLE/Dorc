//! The immutable V1 receipt store: what a publication PROVED, what a bounded walk of the store
//! found, and the ownership-bearing handles both of those speak in.
//!
//! # Why the proof is a set of properties and not a grade
//!
//! Windows has no operation matching a Unix directory synchronization, so the platforms do not
//! sit on one ladder and an `Ord` over grades would invent a comparison the world does not
//! support. The proof therefore records independent PROPERTIES, and a policy check asks whether
//! the ones it requires are present — never whether a number is big enough.
//!
//! The distinction that matters most, and the reason directory synchronization is one property
//! with two negative answers: "this platform has no such operation" and "the operation ran and
//! failed" are different facts about a publication, and collapsing them would let a real failure
//! read as an ordinary platform limit.
//!
//! # Why a name is never enough to reach an entry
//!
//! Nothing here accepts a string as a place to write, read, or remove. Publication mints its own
//! filename from typed values, and a walk hands back [`OwnedReceiptEntry`] values carrying the
//! store they came from — so a caller cannot assemble a name, and cannot spend one store's entry
//! against another's root.
//!
//! # What is deliberately absent
//!
//! Retention, pruning, repair, renaming, a mutable latest pointer, an index, a name-attempt
//! window, and any removal that runs on its own initiative. The one removal path consumes an
//! ownership token that only a failed publication hands out.
//!
//! And there is no "newest complete" selection. [`BoundedReceiptEntries::maximum_order_cohort`]
//! is the only selection the store offers, so a caller finding the newest candidate partial has
//! nothing here to fall back to.

use core::marker::PhantomData;

use dorc_receipt::dispatch::DurablePublicationProof;
use dorc_receipt::format::RefusalReason;
use dorc_receipt::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId, Sha256Digest};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, PlanReceipt, Projection, Species};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::reader::BoundedReceiptBytes;
use dorc_receipt::writer::SignedReceipt;

use crate::io::{self, GroupAndOtherAccess, IoFault, LocalIo, ObjectKind, OpenIntent};
use crate::limits::LocalLimits;
use crate::names::{LocalPath, NameRefusal, NamedSpecies, ReceiptFileName, STORE_DIR};
use crate::roots::{RootInputs, RootRole};

/// Whether one publication achieved one property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectorySync {
    /// The containing directory was synchronized.
    Synchronized,
    /// The platform exposes no meaningful operation. Recorded, never simulated.
    UnavailableOnPlatform,
}

/// What one publication actually did.
///
/// Private fields, no `Default`, no `Ord`: a proof exists because operations succeeded, and there
/// is no ordering between two platforms' proofs to derive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationProperties {
    exclusive_final_name_created: bool,
    complete_bytes_written: bool,
    file_synchronized: bool,
    directory: DirectorySync,
}

impl PublicationProperties {
    /// Record what a publication achieved.
    ///
    /// Every property is stated; none defaults. A caller that cannot say whether it synchronized
    /// has to answer the question rather than omit it.
    #[must_use]
    pub const fn of(
        exclusive_final_name_created: bool,
        complete_bytes_written: bool,
        file_synchronized: bool,
        directory: DirectorySync,
    ) -> Self {
        Self {
            exclusive_final_name_created,
            complete_bytes_written,
            file_synchronized,
            directory,
        }
    }

    /// Whether the file itself was exclusively created, fully written, and synchronized.
    #[must_use]
    pub const fn file_is_durable(self) -> bool {
        self.exclusive_final_name_created && self.complete_bytes_written && self.file_synchronized
    }

    /// What happened to the containing directory.
    #[must_use]
    pub const fn directory(self) -> DirectorySync {
        self.directory
    }

    /// Whether this satisfies the required local baseline for `platform`.
    ///
    /// A typed question over properties, never a numeric comparison. The Windows baseline is
    /// EXPLICITLY weaker and says so here rather than being quietly folded into the same answer:
    /// it demands everything Unix does except the directory synchronization the platform does not
    /// offer, and it is not equivalent.
    #[must_use]
    pub const fn meets_required_baseline(self, platform: PlatformBaseline) -> bool {
        if !self.file_is_durable() {
            return false;
        }
        match platform {
            PlatformBaseline::UnixLike => matches!(self.directory, DirectorySync::Synchronized),
            PlatformBaseline::Windows => {
                matches!(self.directory, DirectorySync::UnavailableOnPlatform)
            }
        }
    }
}

/// Which platform's honest baseline a proof is being read against.
///
/// Two arms, and no third for "whichever": the required properties differ, so a caller has to say
/// which world it is in rather than being handed a portable answer that is true nowhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformBaseline {
    /// Unix and macOS on a local filesystem: the directory synchronization is required.
    UnixLike,
    /// Windows: the file is flushed and the directory operation does not exist.
    Windows,
}

/// Why a publication did not happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishFailure {
    /// The bytes are larger than one receipt may be. Checked before the filesystem is touched.
    OverReceiptBound,
    /// The typed values did not spell one ordinary single-component name within the name bound.
    ///
    /// Unreachable for the identities this crate is handed, and present so the mint has no reason
    /// to assert its way past a refusal it should be returning.
    NameUnmintable,
    /// A file already exists under the exact final name. Never replaced.
    NameAlreadyTaken,
    /// The store root is not a validated directory this process may write to.
    RootUnusable,
    /// The exclusive create did not happen.
    CreateFailed,
    /// Some bytes were written and some were not. No retry into another name.
    WriteIncomplete,
    /// Synchronization failed. Never retried: a second call can report success over pages the
    /// kernel already discarded.
    SyncFailed,
    /// Every operation succeeded and the properties still do not meet the required baseline.
    ///
    /// Refused rather than demoted. Reachable where a filesystem answers a synchronization the
    /// platform's baseline does not expect, which is a machine this proof cannot describe.
    BaselineUnmet,
}

/// Why a store root could not be opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOpenRefusal {
    /// The state root does not admit this project's fixed components.
    RootUnavailable,
    /// No store exists. The read-only path answers this and creates nothing.
    NotInitialized,
    /// Something that is not a directory stands where the store belongs.
    NotADirectory,
    /// A redirect, a refusal, or a root anyone but the owner may write.
    PermissionRefused,
    /// The platform refused right now in a way that says nothing about the store's contents.
    TemporarilyUnavailable,
    /// A directory this project owns could not be created.
    CreateFailed,
    /// A newly created directory could not be synchronized. Never retried.
    SyncFailed,
}

/// Why an enumeration did not produce a bounded listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumerateFailure {
    /// The store root is not a validated directory.
    RootUnusable,
    /// The walk found more entries than the bound admits.
    ///
    /// A fact the walk OBSERVED — it goes to the bound plus one — rather than a silence at the
    /// boundary that would read as a complete short listing. Refused rather than truncated,
    /// because a listing missing its tail could hide the very entry a selection wanted.
    OverEntryBound,
    /// The platform refused the walk.
    WalkFailed,
}

/// Why one entry could not be read back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreReadFailure {
    /// The entry belongs to a different store root than the one asked.
    NotThisStore,
    /// The entry is no longer there. Enumeration is not a snapshot.
    Vanished,
    /// The entry is not a regular file, or is a link or reparse point. Never followed.
    NotARegularFile,
    /// The entry is larger than one receipt may be, measured independently of whatever wrote it.
    OverReceiptBound,
    /// Reading it would take a graph build past its aggregate budget.
    OverGraphBudget,
    /// The platform refused the read.
    ReadFailed,
}

/// Why an owned object was not removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupFailure {
    /// The object is not one this attempt created. Never broadened into removal by pathname.
    NotOwned,
    /// It is already gone.
    Vanished,
    /// The platform refused.
    Refused,
}

/// What a recognized entry turned out to be, once read.
///
/// `IncompletePublication` is the arm that earns its place: publication creates the final name
/// directly, so a crash can leave a prefix on disk, and such a file can never locate as a whole
/// document. What it CANNOT say is which side of the crash it is on, and its payload says so
/// rather than guessing.
///
/// The arms are drawn where the BOUNDED LEXICAL LOCATOR draws them — the one thing `30Ra`'s
/// reader ordering permits before a signature is checked. A missing span or trailer is the shape
/// a truncation always takes, so it reads as incomplete; every other refusal is damage. Neither
/// answer says a document is VALID: that is the reader's question and it needs a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryStanding {
    /// Every span is present and the trailer closes the document.
    CompleteBytes,
    /// A span or the trailer is missing. Presence alone cannot say whether a writer is still
    /// working or stopped.
    IncompletePublication {
        /// The one honest reading.
        state: IncompleteState,
    },
    /// The bytes are present and are not this format's. Distinct from incomplete, because a
    /// damaged whole file is not a publication anybody is still finishing.
    Damaged,
}

impl EntryStanding {
    /// Whether the bytes located as a whole document.
    ///
    /// An exhaustive match rather than a comparison at each caller, so a future arm cannot be
    /// silently absorbed into "not obviously broken, so presumably fine".
    #[must_use]
    pub const fn is_complete(self) -> bool {
        match self {
            Self::CompleteBytes => true,
            Self::IncompletePublication { .. } | Self::Damaged => false,
        }
    }
}

/// What an incomplete entry can be said to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncompleteState {
    /// Either a publication still running or one that stopped. Presence cannot distinguish them,
    /// and this arm is the refusal to pretend otherwise.
    InProgressOrAbandoned,
}

/// The store root under `roots`, as one derivation.
///
/// One seat rather than two spellings: the first-use gate probes this location read-only and the
/// store publishes into it, and two independently assembled paths would let those two disagree
/// about which directory the question was even about. An admin-named folder is the root EXACTLY,
/// with no component beneath it, so that agreement covers both selections.
#[must_use]
pub fn store_root(roots: &RootInputs) -> Option<LocalPath> {
    match roots.explicit_store() {
        Some(folder) => Some(folder),
        None => roots
            .product_root(RootRole::State)
            .and_then(|root| root.child(STORE_DIR)),
    }
}

/// The complete bound policy one store is opened under.
///
/// Both halves together, held by the store rather than passed per call: a receipt bound and a
/// walk bound describe one store, and two calls handed different policies would disagree about
/// the same directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreLimits {
    /// What one document may be.
    pub receipt: ReceiptLimits,
    /// What the local edge itself bounds — names, walks, and the graph aggregate.
    pub local: LocalLimits,
}

impl StoreLimits {
    /// The V1 policy, both halves.
    pub const V1: Self = Self {
        receipt: ReceiptLimits::V1,
        local: LocalLimits::V1,
    };
}

/// The species a receipt store can file, and the typed identity each one is filed under.
///
/// A separate trait from [`Species`], which it requires: that one is sealed inside the receipt
/// crate, so the set of types able to implement this is closed to the three below. What it adds
/// is the store's own two facts — the filename vocabulary and the typed identity — so a plan
/// identity cannot file an outcome.
pub trait StoredSpecies: Species + sealed::Sealed {
    /// The filename vocabulary this species is spelled in.
    const NAMED: NamedSpecies;

    /// The typed identity documents of this species carry.
    type Id: Copy + core::fmt::Debug;

    /// The one lowercase hexadecimal spelling of `id`.
    fn id_hex(id: Self::Id) -> String;
}

mod sealed {
    /// Implemented for the three receipt species and nothing else.
    pub trait Sealed {}
}

impl sealed::Sealed for PlanReceipt {}
impl StoredSpecies for PlanReceipt {
    const NAMED: NamedSpecies = NamedSpecies::Plan;
    type Id = PlanReceiptId;
    fn id_hex(id: Self::Id) -> String {
        id.hex()
    }
}

impl sealed::Sealed for ApplyIntent {}
impl StoredSpecies for ApplyIntent {
    const NAMED: NamedSpecies = NamedSpecies::ApplyIntent;
    type Id = ApplyIntentId;
    fn id_hex(id: Self::Id) -> String {
        id.hex()
    }
}

impl sealed::Sealed for ApplyOutcome {}
impl StoredSpecies for ApplyOutcome {
    const NAMED: NamedSpecies = NamedSpecies::ApplyOutcome;
    type Id = ApplyOutcomeId;
    fn id_hex(id: Self::Id) -> String {
        id.hex()
    }
}

/// The wire species token a filename vocabulary corresponds to.
///
/// The two vocabularies are deliberately separate types, and this is the ONE place they are
/// related — so a rename of either is a change here rather than a silent disagreement between a
/// filename and the body it names.
const fn wire_token_of(species: NamedSpecies) -> &'static str {
    match species {
        NamedSpecies::Plan => PlanReceipt::TOKEN,
        NamedSpecies::ApplyIntent => ApplyIntent::TOKEN,
        NamedSpecies::ApplyOutcome => ApplyOutcome::TOKEN,
    }
}

/// The domain the publication proof's document digest is taken under.
///
/// Its own domain, and never a bare digest: the value identifies WHICH BYTES this publication
/// placed, and one computed under another domain must not be substitutable for it.
const PUBLICATION_DIGEST_DOMAIN: &str = "application/vnd.dorc.receipt.v1.local-publication";

/// What the fixed V1 production publication demands.
///
/// Private field, and the only mint is [`LocalReceiptStoreV1::required_policy`]: the baseline a
/// publication is judged against comes from the store that validated its own root, so no caller
/// can present a weaker platform's rules to a publication running on this one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalRequiredReceiptPolicyV1 {
    baseline: PlatformBaseline,
}

impl LocalRequiredReceiptPolicyV1 {
    /// The one policy identity V1 has. Recorded in every proof, so a later reader can say which
    /// rules a publication was judged under rather than assuming today's.
    pub const IDENTITY: &'static str = "required-local-v1";

    /// Which platform's honest baseline this policy demands.
    #[must_use]
    pub const fn baseline(self) -> PlatformBaseline {
        self.baseline
    }
}

/// The proof that one exact document reached the store at the required baseline.
///
/// Private fields, no `Default`, no `Clone`: it exists because operations succeeded, and it is
/// bound to the exact document that succeeded — identity, species, projection, order, the digest
/// of the bytes placed, the policy it was judged under, and the properties achieved. A caller
/// holding one for a plan receipt cannot spend it for an intent, because the species is a type
/// parameter rather than a field.
///
/// A fixture or volatile sink answers `dorc_receipt::writer::PublishedReceipt`, which is a
/// different type with no route to this one. That disjointness is what keeps a required arm from
/// being satisfied by something held in memory.
#[derive(Debug)]
pub struct RequiredLocalPublicationV1<D: StoredSpecies, P: Projection> {
    receipt_id: D::Id,
    order: ReceiptOrderToken,
    file_name: ReceiptFileName,
    document_digest: Sha256Digest,
    policy: &'static str,
    properties: PublicationProperties,
    species: PhantomData<D>,
    projection: PhantomData<P>,
}

impl<D: StoredSpecies, P: Projection> RequiredLocalPublicationV1<D, P> {
    /// The identity of the document this publication placed.
    #[must_use]
    pub const fn receipt_id(&self) -> D::Id {
        self.receipt_id
    }

    /// The order it was filed under.
    #[must_use]
    pub const fn order(&self) -> ReceiptOrderToken {
        self.order
    }

    /// The exact name it was created under.
    #[must_use]
    pub const fn file_name(&self) -> &ReceiptFileName {
        &self.file_name
    }

    /// The digest of the exact bytes placed.
    #[must_use]
    pub const fn document_digest(&self) -> Sha256Digest {
        self.document_digest
    }

    /// Which policy this publication was judged under.
    #[must_use]
    pub const fn policy_identity(&self) -> &'static str {
        self.policy
    }

    /// What the publication achieved.
    #[must_use]
    pub const fn properties(&self) -> PublicationProperties {
        self.properties
    }

    /// The proof a pre-dispatch gate consumes, carrying this exact placement's three facts.
    ///
    /// Minted from the publication rather than beside it: reaching this method at all means the
    /// exclusive create, the complete write, and every synchronization the platform's required
    /// baseline demands all succeeded, because nothing else produces the value it is called on.
    #[must_use]
    pub fn durable_proof(&self) -> DurablePublicationProof {
        DurablePublicationProof::of_required_placement(
            D::id_hex(self.receipt_id),
            self.document_digest,
            self.policy,
        )
    }
}

/// A file this attempt created and did not finish.
///
/// The only thing that can be removed, and the only way to obtain one is to have had a
/// publication fail after its exclusive create. Not `Clone`, so removal is a single act; consumed
/// by [`LocalReceiptStoreV1::remove_owned`], the crate's one removal, which nothing inside the
/// crate calls.
///
/// Dropping one is a legitimate outcome: what it leaves is bounded partial evidence that no later
/// writer replaces.
#[derive(Debug)]
pub struct IncompletePublicationOwned {
    root: LocalPath,
    path: LocalPath,
    file_name: ReceiptFileName,
}

impl IncompletePublicationOwned {
    /// The name the incomplete object was created under.
    #[must_use]
    pub const fn file_name(&self) -> &ReceiptFileName {
        &self.file_name
    }
}

/// Why a publication failed, and what it left behind.
///
/// The two travel in one value because they are one answer: a failure after the exclusive create
/// left an object this attempt owns, and a caller deciding what to do about it needs the
/// ownership beside the reason. A failure before the create carries none, which is itself the
/// statement that nothing was left.
#[derive(Debug)]
pub struct PublishRefusal {
    reason: PublishFailure,
    incomplete: Option<IncompletePublicationOwned>,
}

impl PublishRefusal {
    /// Why it failed.
    #[must_use]
    pub const fn reason(&self) -> PublishFailure {
        self.reason
    }

    /// The object this attempt created and did not finish, where there is one.
    #[must_use]
    pub fn into_incomplete(self) -> Option<IncompletePublicationOwned> {
        self.incomplete
    }
}

/// One entry a bounded walk recognized, held with the store it came from.
///
/// Private fields and no public constructor: a string is never enough to read or remove an entry,
/// so the only way to address one is to have walked the store that holds it.
///
/// What the name claims is a SELECTION HINT and never authority. Everything it spells also sits
/// inside the signed body, and [`Self::agreement`] is where a caller compares the two after the
/// document has been verified and parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedReceiptEntry {
    root: LocalPath,
    path: LocalPath,
    name: ReceiptFileName,
}

impl OwnedReceiptEntry {
    /// What the filename claims.
    #[must_use]
    pub const fn name(&self) -> &ReceiptFileName {
        &self.name
    }

    /// The species the filename claims.
    #[must_use]
    pub const fn species(&self) -> NamedSpecies {
        self.name.species()
    }

    /// The order the filename claims.
    #[must_use]
    pub const fn order(&self) -> ReceiptOrderToken {
        self.name.order()
    }

    /// Compare what the filename claims against what a verified document says.
    ///
    /// Comparison only: nothing here parses a document. The caller supplies values it read out of
    /// a document it has already verified, and the answer is a FINDING — a disagreement never
    /// promotes the filename, and never rejects the document either.
    #[must_use]
    pub fn agreement(&self, claims: &HeaderClaims<'_>) -> NameAgreement {
        let mut disagreements = Vec::new();
        if claims.version != dorc_receipt::format::VERSION_LINE {
            disagreements.push(NameComponent::Version);
        }
        if claims.species != wire_token_of(self.name.species()) {
            disagreements.push(NameComponent::Species);
        }
        if claims.order != self.name.order() {
            disagreements.push(NameComponent::Order);
        }
        if claims.receipt_id != self.name.receipt_id() {
            disagreements.push(NameComponent::ReceiptId);
        }
        NameAgreement { disagreements }
    }
}

/// What a verified document says about itself, for comparison against its filename.
///
/// A plain input value carrying no authority. A caller that supplied wrong values gets a wrong
/// FINDING, which is why the finding decides nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderClaims<'a> {
    /// The document's version line, exactly as it was read.
    pub version: &'a str,
    /// The species token the document declares.
    pub species: &'a str,
    /// The order token inside the signed body.
    pub order: ReceiptOrderToken,
    /// The receipt identity inside the signed body.
    pub receipt_id: &'a str,
}

/// Which part of a filename a document disagreed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NameComponent {
    /// The format version.
    Version,
    /// The receipt species.
    Species,
    /// The store-selection order.
    Order,
    /// The document identity.
    ReceiptId,
}

/// Whether a filename and a verified document agree, component by component.
///
/// Every disagreement, not the first: a name and a body differing in two places is a different
/// finding from one differing in one, and a check that stopped early would report the smaller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameAgreement {
    disagreements: Vec<NameComponent>,
}

impl NameAgreement {
    /// Whether every component agreed.
    #[must_use]
    pub fn agrees(&self) -> bool {
        self.disagreements.is_empty()
    }

    /// The components that did not.
    #[must_use]
    pub fn disagreements(&self) -> &[NameComponent] {
        &self.disagreements
    }
}

/// A directory entry the V1 name grammar did not recognize.
///
/// Retained as a bounded finding, never deleted and never repaired. Sync-client conflict names
/// land here, and so does anything else somebody left in the directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnrecognizedEntry {
    name: String,
    refusal: NameRefusal,
}

impl UnrecognizedEntry {
    /// The entry's name, where it was within the name bound. An over-long entry keeps none.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Which part of the grammar it failed.
    #[must_use]
    pub const fn refusal(&self) -> NameRefusal {
        self.refusal
    }
}

/// One bounded walk of a store.
///
/// Everything the walk saw is counted, recognized or not: unknown names, conflict files, and
/// directories all consume the budget, so a directory somebody filled costs a refusal rather than
/// an unbounded listing behind a short recognized one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedReceiptEntries {
    recognized: Vec<OwnedReceiptEntry>,
    unrecognized: Vec<UnrecognizedEntry>,
    walked: usize,
}

impl BoundedReceiptEntries {
    /// The entries whose names are V1 receipt names, ascending by order.
    #[must_use]
    pub fn recognized(&self) -> &[OwnedReceiptEntry] {
        &self.recognized
    }

    /// The entries whose names are not, as bounded findings.
    #[must_use]
    pub fn unrecognized(&self) -> &[UnrecognizedEntry] {
        &self.unrecognized
    }

    /// How many directory entries the walk saw in total.
    #[must_use]
    pub const fn walked(&self) -> usize {
        self.walked
    }

    /// The entries sharing the greatest order this walk recognized.
    ///
    /// The ONE selection this store offers, and it deliberately offers no other. There is no
    /// "newest complete" and no "next one down": a caller finding the newest candidate partial or
    /// damaged has to report that, because falling back to older history is not a call it can
    /// make from anything here.
    ///
    /// Several entries at one order is an ambiguity the cohort CARRIES rather than resolves. A
    /// tie-break on receipt identity would be choosing a document by the value least related to
    /// when it was written.
    #[must_use]
    pub fn maximum_order_cohort(&self) -> Option<MaximumOrderCohort<'_>> {
        let order = self.recognized.last()?.order();
        let first = self
            .recognized
            .iter()
            .position(|entry| entry.order() == order)?;
        Some(MaximumOrderCohort {
            order,
            members: self.recognized.get(first..)?,
        })
    }
}

/// The entries at one store's greatest recognized order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaximumOrderCohort<'a> {
    order: ReceiptOrderToken,
    members: &'a [OwnedReceiptEntry],
}

impl<'a> MaximumOrderCohort<'a> {
    /// The order every member shares.
    #[must_use]
    pub const fn order(self) -> ReceiptOrderToken {
        self.order
    }

    /// Every member, in one deterministic order.
    #[must_use]
    pub const fn members(self) -> &'a [OwnedReceiptEntry] {
        self.members
    }

    /// Whether more than one document claims the greatest order.
    #[must_use]
    pub const fn is_ambiguous(self) -> bool {
        self.members.len() > 1
    }
}

/// What a read of one entry produced.
#[derive(Debug)]
pub struct StoredReceiptRead {
    standing: EntryStanding,
    byte_length: usize,
    bytes: BoundedReceiptBytes,
}

impl StoredReceiptRead {
    /// What the bytes turned out to be, as far as a bounded lexical look can say.
    #[must_use]
    pub const fn standing(&self) -> EntryStanding {
        self.standing
    }

    /// How many bytes came back.
    #[must_use]
    pub const fn byte_length(&self) -> usize {
        self.byte_length
    }

    /// The bounded bytes, whatever their standing.
    #[must_use]
    pub const fn bytes(&self) -> &BoundedReceiptBytes {
        &self.bytes
    }

    /// Take the bounded bytes.
    #[must_use]
    pub fn into_bytes(self) -> BoundedReceiptBytes {
        self.bytes
    }
}

/// How many receipt bytes one graph build may still admit.
///
/// Spent as documents are retained, and consulted BEFORE the next one is read: a read is bounded
/// by whichever is smaller, the per-receipt bound or what is left, so nothing is allocated from a
/// count or a length a file declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphAggregateBudget {
    remaining: u64,
}

impl GraphAggregateBudget {
    /// Open a budget under `limits`.
    #[must_use]
    pub const fn of(limits: &LocalLimits) -> Self {
        Self {
            remaining: limits.graph_bytes,
        }
    }

    /// What is left.
    #[must_use]
    pub const fn remaining(self) -> u64 {
        self.remaining
    }
}

/// The one production receipt store: a validated root, its bound policy, and the acts that reach
/// it.
///
/// Holds no filesystem of its own. Every act takes the `io` it is performed through, which is
/// what lets the deterministic model and the real filesystem drive the same code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalReceiptStoreV1 {
    root: LocalPath,
    baseline: PlatformBaseline,
    limits: StoreLimits,
}

impl LocalReceiptStoreV1 {
    /// Open an existing store. Creates nothing.
    ///
    /// `dorc why` reaches the store only through this. A missing store is a report state, not a
    /// directory to make: asking why must never bring a store into being for the answer to be
    /// read out of.
    ///
    /// # Errors
    /// Refuses an unresolvable root, an absent store, a non-directory, a redirect, and a root
    /// anyone but the owner may write.
    pub fn open_for_read(
        roots: &RootInputs,
        io: &mut dyn LocalIo,
        limits: StoreLimits,
    ) -> Result<Self, StoreOpenRefusal> {
        let owned = locations(roots)?;
        let baseline = roots.platform().baseline();
        // EVERY owned component, exactly as the create-capable path validates them. Validating
        // only the store itself would accept one reached through a product root somebody else may
        // write, and the two opens would then disagree about the same profile.
        for component in &owned {
            validate_directory(io, component, baseline)?;
        }
        let root = owned
            .into_iter()
            .next_back()
            .ok_or(StoreOpenRefusal::RootUnavailable)?;
        Ok(Self {
            root,
            baseline,
            limits,
        })
    }

    /// Open the store, creating this project's fixed components where they are absent.
    ///
    /// # Errors
    /// Refuses an unresolvable root, a component it could not create, a synchronization that
    /// failed, and every validation the read-only path refuses.
    pub fn open_or_create(
        roots: &RootInputs,
        io: &mut dyn LocalIo,
        limits: StoreLimits,
    ) -> Result<Self, StoreOpenRefusal> {
        let owned = locations(roots)?;
        let baseline = roots.platform().baseline();
        // Outermost first: a component is created only once the one containing it exists and has
        // been validated, which is the bootstrap protocol's own order
        // (`30Rd:clean-profile-root-bootstrap`).
        for component in &owned {
            ensure_directory(io, component, baseline)?;
        }
        // Innermost first: the entry that makes a directory reachable lives in its parent, so
        // syncing the child before the parent is what makes the pair durable together.
        for component in owned.iter().rev() {
            io::sync_directory(io, component.as_str()).map_err(|_| StoreOpenRefusal::SyncFailed)?;
        }
        let root = owned
            .into_iter()
            .next_back()
            .ok_or(StoreOpenRefusal::RootUnavailable)?;
        validate_directory(io, &root, baseline)?;
        Ok(Self {
            root,
            baseline,
            limits,
        })
    }

    /// The validated root every child of this store is taken relative to.
    #[must_use]
    pub const fn root(&self) -> &LocalPath {
        &self.root
    }

    /// The bound policy this store was opened under.
    #[must_use]
    pub const fn limits(&self) -> StoreLimits {
        self.limits
    }

    /// A fresh aggregate budget for one graph build over this store.
    #[must_use]
    pub const fn graph_budget(&self) -> GraphAggregateBudget {
        GraphAggregateBudget::of(&self.limits.local)
    }

    /// The fixed V1 required policy, judged against the platform this store validated under.
    #[must_use]
    pub const fn required_policy(&self) -> LocalRequiredReceiptPolicyV1 {
        LocalRequiredReceiptPolicyV1 {
            baseline: self.baseline,
        }
    }

    /// Publish one complete signed document at the required baseline.
    ///
    /// The name is minted HERE, from the species type, the order, and the typed identity — a
    /// caller supplies no string and therefore cannot aim a publication anywhere. Creation is
    /// exclusive and directly under the final name, so a taken name is a refusal rather than a
    /// replacement, and an interrupted publication leaves a prefix that can never locate as a
    /// whole document.
    ///
    /// Synchronization is never retried. A second call can report success over pages the kernel
    /// has already discarded, so the attempt fails and a later one revalidates from disk
    /// [A-rebello-fsync-failures-2020].
    ///
    /// # Errors
    /// Refuses over-bound bytes before touching the filesystem, a taken name, a create, write, or
    /// synchronization that failed, and properties that do not meet the required baseline. Every
    /// failure after the create carries the ownership of what it left behind.
    pub fn publish_required_v1<D: StoredSpecies, P: Projection>(
        &self,
        io: &mut dyn LocalIo,
        order: ReceiptOrderToken,
        receipt_id: D::Id,
        receipt: SignedReceipt<D, P>,
        policy: LocalRequiredReceiptPolicyV1,
    ) -> Result<RequiredLocalPublicationV1<D, P>, PublishRefusal> {
        let bytes = receipt.into_bytes();
        if !self
            .limits
            .receipt
            .outer_bytes
            .admits(u64::try_from(bytes.len()).unwrap_or(u64::MAX))
        {
            return Err(before_create(PublishFailure::OverReceiptBound));
        }
        let name_bound = self.limits.local.name_bytes;
        let Some(file_name) = ReceiptFileName::of(D::NAMED, order, &D::id_hex(receipt_id))
            .filter(|name| name.spelled().len() <= name_bound)
        else {
            return Err(before_create(PublishFailure::NameUnmintable));
        };
        let Some(path) = self.root.child(&file_name.spelled()) else {
            return Err(before_create(PublishFailure::NameUnmintable));
        };

        match io::create_file_exclusive(io, path.as_str()) {
            Ok(()) => {}
            Err(IoFault::AlreadyExists) => {
                return Err(before_create(PublishFailure::NameAlreadyTaken));
            }
            Err(IoFault::Denied | IoFault::NotFound) => {
                return Err(before_create(PublishFailure::RootUnusable));
            }
            Err(_) => return Err(before_create(PublishFailure::CreateFailed)),
        }
        let owned = IncompletePublicationOwned {
            root: self.root.clone(),
            path,
            file_name: file_name.clone(),
        };

        let directory = match self.write_and_synchronize(io, owned.path.as_str(), &bytes) {
            Ok(directory) => directory,
            Err(reason) => {
                return Err(PublishRefusal {
                    reason,
                    incomplete: Some(owned),
                });
            }
        };

        let properties = PublicationProperties::of(true, true, true, directory);
        if !properties.meets_required_baseline(policy.baseline()) {
            return Err(PublishRefusal {
                reason: PublishFailure::BaselineUnmet,
                incomplete: Some(owned),
            });
        }
        Ok(RequiredLocalPublicationV1 {
            receipt_id,
            order,
            file_name,
            document_digest: Sha256Digest::over(PUBLICATION_DIGEST_DOMAIN, &bytes),
            policy: LocalRequiredReceiptPolicyV1::IDENTITY,
            properties,
            species: PhantomData,
            projection: PhantomData,
        })
    }

    /// Write every byte, synchronize the file, then synchronize the containing directory.
    fn write_and_synchronize(
        &self,
        io: &mut dyn LocalIo,
        path: &str,
        bytes: &[u8],
    ) -> Result<DirectorySync, PublishFailure> {
        io::write_all(io, path, bytes).map_err(|_| PublishFailure::WriteIncomplete)?;
        io::sync_file(io, path).map_err(|_| PublishFailure::SyncFailed)?;
        io::sync_directory(io, self.root.as_str()).map_err(|_| PublishFailure::SyncFailed)
    }

    /// Walk the store to the entry bound plus one, then classify what was found.
    ///
    /// Everything is counted before anything is filtered, so an unknown or conflict name costs
    /// budget exactly as a receipt does — which is what stops a directory full of foreign entries
    /// from hiding an unbounded walk behind a short recognized listing.
    ///
    /// # Errors
    /// Refuses a root that is not a validated directory, a walk the platform refused, and a walk
    /// that saw more entries than the bound admits.
    pub fn enumerate(
        &self,
        io: &mut dyn LocalIo,
    ) -> Result<BoundedReceiptEntries, EnumerateFailure> {
        let limits = &self.limits.local;
        let entries = io::enumerate_bounded(io, self.root.as_str(), limits.store_entries).map_err(
            |fault| match fault {
                IoFault::NotFound | IoFault::WrongKind | IoFault::Redirect => {
                    EnumerateFailure::RootUnusable
                }
                _ => EnumerateFailure::WalkFailed,
            },
        )?;
        if entries.over_bound() {
            return Err(EnumerateFailure::OverEntryBound);
        }

        let walked = entries.names().len();
        let mut recognized: Vec<OwnedReceiptEntry> = Vec::new();
        let mut unrecognized: Vec<UnrecognizedEntry> = Vec::new();
        for entry in entries.names() {
            match ReceiptFileName::of_entry(entry, limits) {
                Ok(name) => match self.root.child(&name.spelled()) {
                    Some(path) => recognized.push(OwnedReceiptEntry {
                        root: self.root.clone(),
                        path,
                        name,
                    }),
                    None => unrecognized.push(UnrecognizedEntry {
                        name: String::new(),
                        refusal: NameRefusal::MalformedUnderKnownSpecies,
                    }),
                },
                Err(refusal) => unrecognized.push(UnrecognizedEntry {
                    name: if refusal == NameRefusal::OverNameBound {
                        String::new()
                    } else {
                        entry.clone()
                    },
                    refusal,
                }),
            }
        }
        recognized.sort_by(|left, right| {
            (
                left.name.order(),
                left.name.species(),
                left.name.receipt_id(),
            )
                .cmp(&(
                    right.name.order(),
                    right.name.species(),
                    right.name.receipt_id(),
                ))
        });
        Ok(BoundedReceiptEntries {
            recognized,
            unrecognized,
            walked,
        })
    }

    /// Read one entry back under its own independent bound.
    ///
    /// The bound is this reader's, not the writer's: a file already on disk proves nothing about
    /// what wrote it, so its size is measured again here rather than inferred from a policy some
    /// earlier process was running under.
    ///
    /// # Errors
    /// Refuses an entry from another store, one that has vanished, one that is not a regular file
    /// or has become a redirect, one past the receipt bound, and a read the platform refused.
    pub fn read(
        &self,
        io: &mut dyn LocalIo,
        entry: &OwnedReceiptEntry,
    ) -> Result<StoredReceiptRead, StoreReadFailure> {
        self.read_bounded_by(io, entry, self.limits.receipt.outer_bytes.get())
    }

    /// Read one entry as part of a graph build, spending the aggregate budget.
    ///
    /// # Errors
    /// Everything [`Self::read`] refuses, plus a budget with nothing left for this document.
    pub fn read_into_budget(
        &self,
        io: &mut dyn LocalIo,
        entry: &OwnedReceiptEntry,
        budget: &mut GraphAggregateBudget,
    ) -> Result<StoredReceiptRead, StoreReadFailure> {
        let outer = self.limits.receipt.outer_bytes.get();
        if budget.remaining == 0 {
            return Err(StoreReadFailure::OverGraphBudget);
        }
        let ceiling = outer.min(budget.remaining);
        let read = self
            .read_bounded_by(io, entry, ceiling)
            .map_err(|failure| match failure {
                // The read was bounded by whichever was smaller, so an over-bound answer under a
                // narrowed ceiling is the AGGREGATE running out rather than one oversized
                // document. Naming it otherwise would report a size the store never measured.
                StoreReadFailure::OverReceiptBound if ceiling < outer => {
                    StoreReadFailure::OverGraphBudget
                }
                other => other,
            })?;
        let spent = u64::try_from(read.byte_length).unwrap_or(u64::MAX);
        budget.remaining = budget.remaining.saturating_sub(spent);
        Ok(read)
    }

    fn read_bounded_by(
        &self,
        io: &mut dyn LocalIo,
        entry: &OwnedReceiptEntry,
        ceiling: u64,
    ) -> Result<StoredReceiptRead, StoreReadFailure> {
        if entry.root != self.root {
            return Err(StoreReadFailure::NotThisStore);
        }
        let path = entry.path.as_str();
        match io::open_existing_no_follow(io, path, OpenIntent::Read) {
            Ok(()) => {}
            Err(IoFault::NotFound) => return Err(StoreReadFailure::Vanished),
            Err(IoFault::Redirect | IoFault::WrongKind) => {
                return Err(StoreReadFailure::NotARegularFile);
            }
            Err(_) => return Err(StoreReadFailure::ReadFailed),
        }
        let facts = io::inspect_opened(io, path).map_err(|_| StoreReadFailure::ReadFailed)?;
        if facts.kind() != ObjectKind::RegularFile || facts.redirected() {
            return Err(StoreReadFailure::NotARegularFile);
        }
        let ceiling = usize::try_from(ceiling).unwrap_or(usize::MAX);
        let raw = io::read_bounded(io, path, ceiling).map_err(|fault| match fault {
            IoFault::OverBound => StoreReadFailure::OverReceiptBound,
            IoFault::NotFound => StoreReadFailure::Vanished,
            _ => StoreReadFailure::ReadFailed,
        })?;
        let byte_length = raw.len();
        // Measured before the bytes are handed over, because the bounded type exposes none.
        let truncated_opening = is_truncated_opening(&raw);
        let bytes = BoundedReceiptBytes::of(raw, &self.limits.receipt)
            .map_err(|_| StoreReadFailure::OverReceiptBound)?;
        let standing = standing_of(&bytes, &self.limits.receipt, truncated_opening);
        Ok(StoredReceiptRead {
            standing,
            byte_length,
            bytes,
        })
    }

    /// Remove an object one of this store's failed publications created and did not finish.
    ///
    /// The crate's ONE removal, and nothing inside the crate calls it: the token is the caller's
    /// to spend or to drop, and dropping it leaves bounded partial evidence no later writer
    /// replaces. The underlying operation refuses any path this attempt did not create, so a
    /// failure here is reported and never broadened into removal by pathname.
    ///
    /// # Errors
    /// Refuses a token from another store, an object this attempt does not own, one already gone,
    /// and a removal the platform refused.
    pub fn remove_owned(
        &self,
        io: &mut dyn LocalIo,
        owned: IncompletePublicationOwned,
    ) -> Result<(), CleanupFailure> {
        let IncompletePublicationOwned { root, path, .. } = owned;
        if root != self.root {
            return Err(CleanupFailure::NotOwned);
        }
        io::remove_owned(io, path.as_str()).map_err(|fault| match fault {
            IoFault::Denied => CleanupFailure::NotOwned,
            IoFault::NotFound => CleanupFailure::Vanished,
            _ => CleanupFailure::Refused,
        })
    }
}

/// The Dorc-owned components a store lives in, OUTERMOST FIRST, and the store root last.
///
/// One derivation for both open paths, so neither can validate a component the other does not.
/// The standard selection owns two — the product root and the store beneath it — while an
/// admin-named folder owns exactly one, itself: nothing above it is Dorc's to validate or create,
/// and appending a component beneath it would put the store somewhere the admin did not name.
///
/// Ordered rather than a pair, because that is the only difference between the two selections and
/// keeping it a length lets every caller walk one list instead of branching.
fn locations(roots: &RootInputs) -> Result<Vec<LocalPath>, StoreOpenRefusal> {
    if let Some(folder) = roots.explicit_store() {
        return Ok(vec![folder]);
    }
    let product = roots
        .product_root(RootRole::State)
        .ok_or(StoreOpenRefusal::RootUnavailable)?;
    let root = product
        .child(STORE_DIR)
        .ok_or(StoreOpenRefusal::RootUnavailable)?;
    Ok(vec![product, root])
}

/// A refusal that happened before anything was created, so nothing was left behind.
const fn before_create(reason: PublishFailure) -> PublishRefusal {
    PublishRefusal {
        reason,
        incomplete: None,
    }
}

/// Whether the bytes are a PROPER PREFIX of the one line every V1 document opens with.
///
/// The distinction that separates the shortest truncations from foreign bytes. A publication
/// interrupted early leaves a prefix of the opening line — an empty file being the shortest of
/// them — which the locator can only report as an unknown version, the same answer it gives some
/// other format entirely. Comparing against the opening is what tells the two apart without a
/// second parser.
fn is_truncated_opening(raw: &[u8]) -> bool {
    let opening = format!("{}\n", dorc_receipt::format::VERSION_LINE);
    raw.len() < opening.len() && opening.as_bytes().starts_with(raw)
}

/// What a bounded lexical look says the bytes are.
fn standing_of(
    bytes: &BoundedReceiptBytes,
    limits: &ReceiptLimits,
    truncated_opening: bool,
) -> EntryStanding {
    let incomplete = EntryStanding::IncompletePublication {
        state: IncompleteState::InProgressOrAbandoned,
    };
    match bytes.locate(limits) {
        Ok(_) => EntryStanding::CompleteBytes,
        // A span or the trailer being absent is the shape a truncated document always takes.
        Err(RefusalReason::Structure { .. } | RefusalReason::SignatureShape) => incomplete,
        Err(RefusalReason::UnsupportedVersion) if truncated_opening => incomplete,
        Err(_) => EntryStanding::Damaged,
    }
}

/// Create one directory this project owns, or validate the one already there.
fn ensure_directory(
    io: &mut dyn LocalIo,
    path: &LocalPath,
    baseline: PlatformBaseline,
) -> Result<(), StoreOpenRefusal> {
    match io::create_directory_exclusive(io, path.as_str()) {
        // Not synchronized here. Directory synchronization has ONE seat, after the components
        // exist, where its failure fails the open.
        Ok(()) => Ok(()),
        // A race is answered by validating the winner, never by assuming it is what was asked
        // for.
        Err(IoFault::AlreadyExists) => validate_directory(io, path, baseline),
        Err(IoFault::Denied) => Err(StoreOpenRefusal::PermissionRefused),
        Err(_) => Err(StoreOpenRefusal::CreateFailed),
    }
}

/// Open one directory without following a redirect, and inspect it before it is used.
///
/// A store root anyone else may WRITE is refused; one anyone else may READ is not. Receipts are
/// sensitive and are created owner-only, but an operator who widened the containing directory's
/// readability has not made it a place another account can plant entries — which is the property
/// this validation is actually about. A Unix landing that answers nothing is refused, because
/// there the answer is required; only the explicitly weaker Windows baseline accepts it.
fn validate_directory(
    io: &mut dyn LocalIo,
    path: &LocalPath,
    baseline: PlatformBaseline,
) -> Result<(), StoreOpenRefusal> {
    match io::open_existing_no_follow(io, path.as_str(), OpenIntent::Read) {
        Ok(()) => {}
        Err(IoFault::NotFound) => return Err(StoreOpenRefusal::NotInitialized),
        Err(IoFault::Redirect | IoFault::Denied) => {
            return Err(StoreOpenRefusal::PermissionRefused);
        }
        Err(_) => return Err(StoreOpenRefusal::TemporarilyUnavailable),
    }
    let facts = io::inspect_opened(io, path.as_str())
        .map_err(|_| StoreOpenRefusal::TemporarilyUnavailable)?;
    if facts.kind() != ObjectKind::Directory {
        return Err(StoreOpenRefusal::NotADirectory);
    }
    if facts.redirected() {
        return Err(StoreOpenRefusal::PermissionRefused);
    }
    match (baseline, facts.group_and_other()) {
        (PlatformBaseline::UnixLike, GroupAndOtherAccess::None | GroupAndOtherAccess::Present)
        | (PlatformBaseline::Windows, GroupAndOtherAccess::NotInspectable) => Ok(()),
        _ => Err(StoreOpenRefusal::PermissionRefused),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn everything(directory: DirectorySync) -> PublicationProperties {
        PublicationProperties::of(true, true, true, directory)
    }

    #[test]
    fn the_two_platform_baselines_are_not_interchangeable() {
        // The point of refusing an ordering. A Windows proof does not satisfy the Unix baseline,
        // and — just as load-bearing — a Unix proof does not satisfy the Windows one either: a
        // proof claiming a directory synchronization on a platform that has none is describing
        // some other machine.
        let unix = everything(DirectorySync::Synchronized);
        let windows = everything(DirectorySync::UnavailableOnPlatform);
        assert!(unix.meets_required_baseline(PlatformBaseline::UnixLike));
        assert!(windows.meets_required_baseline(PlatformBaseline::Windows));
        assert!(!windows.meets_required_baseline(PlatformBaseline::UnixLike));
        assert!(!unix.meets_required_baseline(PlatformBaseline::Windows));
    }

    #[test]
    fn every_missing_file_property_defeats_the_baseline_on_both_platforms() {
        // Exhaustive over the three file properties rather than one representative: each is
        // separately required, and a check that read only the last of them would pass a
        // publication that never created its own name.
        for (which, properties) in [
            (
                "no exclusive create",
                PublicationProperties::of(false, true, true, DirectorySync::Synchronized),
            ),
            (
                "incomplete write",
                PublicationProperties::of(true, false, true, DirectorySync::Synchronized),
            ),
            (
                "no file sync",
                PublicationProperties::of(true, true, false, DirectorySync::Synchronized),
            ),
        ] {
            assert!(
                !properties.meets_required_baseline(PlatformBaseline::UnixLike),
                "{which}"
            );
            assert!(
                !properties.meets_required_baseline(PlatformBaseline::Windows),
                "{which}"
            );
        }
    }

    #[test]
    fn a_platform_without_the_operation_is_not_a_platform_that_failed_it() {
        // There is no `DirectorySync::Failed` arm, and that is deliberate: a failed
        // synchronization fails the whole publication (`PublishFailure::SyncFailed`) rather than
        // being recorded as a weaker proof. Only the platform-limit answer survives into a proof.
        assert_ne!(
            DirectorySync::Synchronized,
            DirectorySync::UnavailableOnPlatform
        );
        assert_ne!(PublishFailure::SyncFailed, PublishFailure::WriteIncomplete);
    }

    #[test]
    fn every_species_files_under_its_own_vocabulary_and_the_two_vocabularies_agree() {
        // The species is a type parameter rather than a field, so the only thing that could drift
        // is the correspondence below: a species whose filename vocabulary disagreed with its
        // wire token would file documents whose own reader calls the name a mismatch.
        assert_eq!(<PlanReceipt as StoredSpecies>::NAMED, NamedSpecies::Plan);
        assert_eq!(
            <ApplyIntent as StoredSpecies>::NAMED,
            NamedSpecies::ApplyIntent
        );
        assert_eq!(
            <ApplyOutcome as StoredSpecies>::NAMED,
            NamedSpecies::ApplyOutcome
        );
        assert_eq!(wire_token_of(NamedSpecies::Plan), PlanReceipt::TOKEN);
        assert_eq!(wire_token_of(NamedSpecies::ApplyIntent), ApplyIntent::TOKEN);
        assert_eq!(
            wire_token_of(NamedSpecies::ApplyOutcome),
            ApplyOutcome::TOKEN
        );
    }

    #[test]
    fn a_standing_short_of_located_is_never_complete() {
        for standing in [
            EntryStanding::IncompletePublication {
                state: IncompleteState::InProgressOrAbandoned,
            },
            EntryStanding::Damaged,
        ] {
            assert!(!standing.is_complete(), "{standing:?}");
        }
        assert!(EntryStanding::CompleteBytes.is_complete());
    }
}
