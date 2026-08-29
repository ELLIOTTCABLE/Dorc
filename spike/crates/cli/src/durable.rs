//! The production durable edge: one assembly, from standard roots to a keyset and a store.
//!
//! # One assembly, and no second one
//!
//! [`LocalReceiptEdgeV1`] is the only route this binary has to a key provider or a receipt store.
//! Nothing here selects a backend, reads a Dorc-specific environment variable, or answers
//! differently because of a terminal, a receipt's own contents, or a command shape. What varies
//! between an ordinary invocation and the deterministic battery is the FILESYSTEM handed in and
//! the standard roots resolved at the process edge — never which assembly runs.
//!
//! # Values in
//!
//! Nothing here reads the environment or opens a file. Root resolution is a RULE this module
//! owns and a QUERY the process edge answers ([`RootEnvironment`]), and every filesystem act
//! travels through the `io` a caller hands in, which is what keeps the whole edge drivable by the
//! local crate's deterministic model as well as by the real filesystem.
//!
//! # Read and write are different entry points
//!
//! [`LocalReceiptEdgeV1::open_for_read`] creates nothing: no product root, no store, no keyset.
//! `dorc why` takes only that one, so asking why can never bring into being an identity that
//! cannot open the receipt being read.

use dorc_receipt::capability::{
    SelfAssertedReceiptVerificationKey, TrustedReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::ids::{ApplyIntentId, ApplyOutcomeId, PlanReceiptId, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Projection, Rich};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::reader::{ReadRich, read_rich};
use dorc_receipt::writer::SignedReceipt;
use dorc_receipt_crypto::{
    EntropyKeysetGenerator, KeySecretEntropy, KeysetGenerator, TrustedEd25519Key,
};
use dorc_receipt_local::io::LocalIo;
use dorc_receipt_local::keyset::{
    KeyAvailability, LocalReadKeysV1, LocalReadOpenV1, LocalWriteKeysV1, LocalWriteOpenV1,
    StorePresence, open_for_read, open_or_initialize_for_write,
};
pub use dorc_receipt_local::store::{
    BoundedReceiptEntries, LocalReceiptStoreV1, OwnedReceiptEntry,
};
use dorc_receipt_local::store::{
    PublishFailure, PublishRefusal, StoreLimits, StoreOpenRefusal, StoredSpecies,
};
use dorc_receipt_local::{LocalLimits, RootInputs, RootPlatform, RootRefusal, RootRole};
/// The real filesystem and the store-side values a caller needs to walk and select, re-exported
/// so the ONE production seat naming the local edge is this module: a caller elsewhere in the
/// binary asks this composition root rather than reaching past it.
pub use dorc_receipt_local::{NamedSpecies, NativeIo};

use crate::receipt_edge::{PlacedDocument, PlacedIntent, PlacementFailure, ReceiptPlacement};

/// The process edge's answer to "what does this environment say".
///
/// A trait rather than direct reads, for the reason every other edge here is one: WHICH variables
/// a platform keeps its per-user configuration and state under is a rule worth testing, and
/// reading them is a query that belongs at the process boundary.
pub trait RootEnvironment {
    /// The value of one standard variable, or `None` where it is unset or empty.
    fn var(&self, name: &str) -> Option<String>;
}

/// Resolve this platform's two standard role-typed roots.
///
/// Fixed per platform, with no Dorc-specific override and no fallback to a working directory, a
/// repository, a temporary directory, or one role standing in for the other. A test drives this
/// by setting the platform's OWN variables to a sandbox, which is what keeps the production
/// resolution the only resolution.
///
/// # Errors
/// Refuses an absent, empty, or non-absolute base, naming the role.
pub fn standard_roots(
    platform: RootPlatform,
    environment: &dyn RootEnvironment,
) -> Result<RootInputs, RootRefusal> {
    let (configuration, state) = match platform {
        RootPlatform::Windows => (
            environment.var("APPDATA").unwrap_or_default(),
            environment.var("LOCALAPPDATA").unwrap_or_default(),
        ),
        RootPlatform::MacOs => {
            let support = environment
                .var("HOME")
                .map(|home| format!("{home}/Library/Application Support"))
                .unwrap_or_default();
            (support.clone(), support)
        }
        RootPlatform::OtherUnix => {
            let home = environment.var("HOME");
            let configuration = environment
                .var("XDG_CONFIG_HOME")
                .or_else(|| home.as_ref().map(|home| format!("{home}/.config")))
                .unwrap_or_default();
            let state = environment
                .var("XDG_STATE_HOME")
                .or_else(|| home.as_ref().map(|home| format!("{home}/.local/state")))
                .unwrap_or_default();
            (configuration, state)
        }
    };
    RootInputs::of(platform, &configuration, &state)
}

/// Which platform's standard locations this build resolves against.
#[must_use]
pub const fn host_platform() -> RootPlatform {
    if cfg!(windows) {
        RootPlatform::Windows
    } else if cfg!(target_os = "macos") {
        RootPlatform::MacOs
    } else {
        RootPlatform::OtherUnix
    }
}

/// Why the production durable edge could not be opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeRefusal {
    /// The platform could not answer where a per-user root lives.
    Roots(RootRefusal),
    /// The store's own root could not be opened or created.
    Store(StoreOpenRefusal),
    /// The keyset is not in a state this invocation may use.
    Keys(KeyAvailability),
}

impl EdgeRefusal {
    /// The closed word a report names this refusal by.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Roots(RootRefusal::ControllerRootUnavailable { .. })
            | Self::Store(StoreOpenRefusal::RootUnavailable) => "no-controller-root",
            Self::Roots(RootRefusal::NotAbsolute { .. }) => "controller-root-not-absolute",
            Self::Store(StoreOpenRefusal::NotInitialized) => "store-not-initialized",
            Self::Store(StoreOpenRefusal::NotADirectory) => "store-not-a-directory",
            Self::Store(StoreOpenRefusal::PermissionRefused) => "store-permission-refused",
            Self::Store(StoreOpenRefusal::TemporarilyUnavailable) => "store-unavailable",
            Self::Store(StoreOpenRefusal::CreateFailed) => "store-create-failed",
            Self::Store(StoreOpenRefusal::SyncFailed) => "store-sync-failed",
            Self::Keys(state) => state.token(),
        }
    }
}

/// The production durable edge: the roots it stands on and the policy it runs under.
///
/// One value, assembled once. It holds no filesystem and no keys — those are what an OPEN
/// produces, and the two opens are separate methods with separate answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalReceiptEdgeV1 {
    roots: RootInputs,
    limits: LocalLimits,
}

impl LocalReceiptEdgeV1 {
    /// Bind the edge to one invocation's resolved roots.
    #[must_use]
    pub const fn of(roots: RootInputs) -> Self {
        Self {
            roots,
            limits: LocalLimits::V1,
        }
    }

    /// The roots this edge stands on.
    #[must_use]
    pub const fn roots(&self) -> &RootInputs {
        &self.roots
    }

    /// The per-user base this edge files receipts under, for a report naming where a run wrote.
    #[must_use]
    pub fn state_base(&self) -> &str {
        self.roots.base(RootRole::State)
    }

    const fn store_limits(&self) -> StoreLimits {
        StoreLimits {
            receipt: ReceiptLimits::V1,
            local: self.limits,
        }
    }

    /// Open the keyset and the store for READING. Creates nothing, initializes nothing.
    ///
    /// # Errors
    /// Refuses unresolvable roots, a store that is absent or unusable, and a keyset that cannot
    /// be validated for reading.
    pub fn open_for_read(&self, io: &mut dyn LocalIo) -> Result<ReadEdge, EdgeRefusal> {
        let store = LocalReceiptStoreV1::open_for_read(&self.roots, io, self.store_limits())
            .map_err(EdgeRefusal::Store)?;
        match open_for_read(&self.roots, io, &self.limits) {
            LocalReadOpenV1::Ready(keys) => Ok(ReadEdge { keys, store }),
            LocalReadOpenV1::Unavailable(state) => Err(EdgeRefusal::Keys(state)),
        }
    }

    /// Open the keyset and the store for WRITING, initializing the keyset on genuine first use.
    ///
    /// The store's own standing is read BEFORE the keyset is opened and handed to the keyset's
    /// entry point, which is what makes "a whole keyset is absent while receipts exist" refuse
    /// rather than quietly begin a new key era over old history.
    ///
    /// # Errors
    /// Refuses unresolvable roots, a store that could not be created or validated, and a keyset
    /// that is incomplete, damaged, or unusable.
    pub fn open_for_write(
        &self,
        io: &mut dyn LocalIo,
        generator: &mut dyn KeysetGenerator,
    ) -> Result<WriteEdge, EdgeRefusal> {
        // The store is PROBED read-only first — its standing is an input to whether first-use
        // generation is even a candidate — and CREATED last. A keyset that cannot be opened means
        // nothing will ever be published, and a run in that state must not leave a store
        // directory behind as though it had been about to.
        let presence = StorePresence::probe(&self.roots, io, &self.limits);
        let keys = match open_or_initialize_for_write(
            &self.roots,
            io,
            &self.limits,
            presence,
            generator,
        ) {
            LocalWriteOpenV1::Ready(keys) => keys,
            LocalWriteOpenV1::Refused(state) => return Err(EdgeRefusal::Keys(state)),
        };
        let store = LocalReceiptStoreV1::open_or_create(&self.roots, io, self.store_limits())
            .map_err(EdgeRefusal::Store)?;
        Ok(WriteEdge { keys, store })
    }
}

/// A validated keyset and store, open for reading.
#[derive(Debug)]
pub struct ReadEdge {
    keys: LocalReadKeysV1,
    store: LocalReceiptStoreV1,
}

impl ReadEdge {
    /// The verification and opening material this controller's policy selected.
    #[must_use]
    pub const fn keys(&self) -> &LocalReadKeysV1 {
        &self.keys
    }

    /// The store, for a bounded walk and bounded reads.
    #[must_use]
    pub const fn store(&self) -> &LocalReceiptStoreV1 {
        &self.store
    }

    /// Read one plan document back: verify under this controller's own material, then open its
    /// region.
    ///
    /// # Errors
    /// Answers the partial receipt for every condition that stopped the read — a signature that
    /// did not check, a region that would not open, a region that did not account for its own
    /// skeleton. None of them releases an opaque value.
    pub fn read_plan(
        &self,
        bytes: Vec<u8>,
    ) -> Result<ReadRich<PlanReceipt>, dorc_receipt::reader::PartialReceipt> {
        let policy = ControllerNamedKey(self.keys.verifier());
        let Some(opener) = self.keys.opener() else {
            // No encryption role: the signature could still be checked, but this V1 read wants
            // the region, and a reader that quietly answered a skeleton-only document would look
            // like a successful read of a thinner receipt.
            return Err(dorc_receipt::reader::PartialReceipt::of(
                dorc_receipt::format::RefusalReason::RegionUnopenable,
            ));
        };
        read_rich::<PlanReceipt>(bytes, &ReceiptLimits::V1, &policy, opener)
    }

    /// Read one apply intent back.
    ///
    /// # Errors
    /// As [`Self::read_plan`].
    pub fn read_intent(
        &self,
        bytes: Vec<u8>,
    ) -> Result<ReadRich<ApplyIntent>, dorc_receipt::reader::PartialReceipt> {
        let policy = ControllerNamedKey(self.keys.verifier());
        let Some(opener) = self.keys.opener() else {
            return Err(dorc_receipt::reader::PartialReceipt::of(
                dorc_receipt::format::RefusalReason::RegionUnopenable,
            ));
        };
        read_rich::<ApplyIntent>(bytes, &ReceiptLimits::V1, &policy, opener)
    }

    /// Read one apply outcome back.
    ///
    /// # Errors
    /// As [`Self::read_plan`].
    pub fn read_outcome(
        &self,
        bytes: Vec<u8>,
    ) -> Result<ReadRich<ApplyOutcome>, dorc_receipt::reader::PartialReceipt> {
        let policy = ControllerNamedKey(self.keys.verifier());
        let Some(opener) = self.keys.opener() else {
            return Err(dorc_receipt::reader::PartialReceipt::of(
                dorc_receipt::format::RefusalReason::RegionUnopenable,
            ));
        };
        read_rich::<ApplyOutcome>(bytes, &ReceiptLimits::V1, &policy, opener)
    }
}

/// The verification material controller policy names, as a resolver.
///
/// One key and no discovery: a document naming any other signing identity is UNKNOWN here, and
/// nothing about that answer scans a directory, imports embedded public material, or tries
/// another provider. The trusted marker is minted because policy selected and validated THIS
/// keyset, never because a receipt named its identity.
struct ControllerNamedKey<'a>(&'a TrustedEd25519Key);

impl VerificationKeyResolver for ControllerNamedKey<'_> {
    fn trusted(&self, id: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey> {
        (self.0.signing_key_id() == id).then_some(self.0 as &dyn TrustedReceiptVerificationKey)
    }

    fn self_asserted(&self, _: SigningKeyId) -> Option<&dyn SelfAssertedReceiptVerificationKey> {
        None
    }
}

/// A validated, synchronized keyset and store, open for publication.
#[derive(Debug)]
pub struct WriteEdge {
    keys: LocalWriteKeysV1,
    store: LocalReceiptStoreV1,
}

impl WriteEdge {
    /// The signing and sealing material.
    #[must_use]
    pub const fn keys(&self) -> &LocalWriteKeysV1 {
        &self.keys
    }

    /// The store this edge publishes into.
    #[must_use]
    pub const fn store(&self) -> &LocalReceiptStoreV1 {
        &self.store
    }

    /// A placement that files documents into this edge's store.
    pub fn placement<'a>(&'a self, io: &'a mut dyn LocalIo) -> StorePlacement<'a> {
        StorePlacement {
            store: &self.store,
            io,
        }
    }
}

/// The production placement: a store that mints its own filenames from typed identities.
///
/// The ONE implementor of [`ReceiptPlacement`] in production code. A fixture placement is a
/// test's own value, which is what keeps a volatile destination structurally unable to answer a
/// production route.
pub struct StorePlacement<'a> {
    store: &'a LocalReceiptStoreV1,
    io: &'a mut dyn LocalIo,
}

impl core::fmt::Debug for StorePlacement<'_> {
    /// Names the type; the store is already `Debug` and the filesystem is not a value.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("StorePlacement")
    }
}

impl StorePlacement<'_> {
    /// File one document, answering where it went and — where the species asks for it — the
    /// durability the placement proved.
    fn file<D: StoredSpecies, P: Projection>(
        &mut self,
        id: D::Id,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<D, P>,
    ) -> Result<
        (
            PlacedDocument,
            dorc_receipt::dispatch::DurablePublicationProof,
        ),
        PlacementFailure,
    > {
        // THE UNDATED REFUSAL, sited at the production composition root and nowhere lower. A
        // clockless run is a supported capability — the library emits an undated document
        // happily, which is what stable tests and a future diffable artifact both need — but a
        // store that selects by order must never hold one, because a document sorting below
        // every dated one would make a `--last` answer with older history. Delete these three
        // lines the day stable-format output becomes a supported mode.
        if order == ReceiptOrderToken::UNDATED {
            return Err(PlacementFailure::UndatedDocument);
        }
        let policy = self.store.required_policy();
        let publication = self
            .store
            .publish_required_v1::<D, P>(self.io, order, id, receipt, policy)
            .map_err(|refusal| placement_failure(&refusal))?;
        let name = publication.file_name().spelled();
        let path = self
            .store
            .root()
            .child(&name)
            .map(|at| at.as_str().to_owned());
        let placed = PlacedDocument::of(
            D::id_hex(id),
            name,
            path,
            dorc_receipt::capability::PublicationGrade::Synchronized,
        );
        Ok((placed, publication.durable_proof()))
    }
}

impl ReceiptPlacement for StorePlacement<'_> {
    fn place_plan(
        &mut self,
        id: PlanReceiptId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<PlanReceipt, Rich>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        self.file::<PlanReceipt, Rich>(id, order, receipt)
            .map(|(placed, _)| placed)
    }

    fn place_plain_plan(
        &mut self,
        id: PlanReceiptId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<PlanReceipt, Plain>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        self.file::<PlanReceipt, Plain>(id, order, receipt)
            .map(|(placed, _)| placed)
    }

    fn place_intent(
        &mut self,
        id: ApplyIntentId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyIntent, Rich>,
    ) -> Result<PlacedIntent, PlacementFailure> {
        self.file::<ApplyIntent, Rich>(id, order, receipt)
            .map(|(placed, durability)| PlacedIntent { placed, durability })
    }

    fn place_plain_intent(
        &mut self,
        id: ApplyIntentId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyIntent, Plain>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        self.file::<ApplyIntent, Plain>(id, order, receipt)
            .map(|(placed, _)| placed)
    }

    fn place_outcome(
        &mut self,
        id: ApplyOutcomeId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyOutcome, Rich>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        self.file::<ApplyOutcome, Rich>(id, order, receipt)
            .map(|(placed, _)| placed)
    }

    fn place_plain_outcome(
        &mut self,
        id: ApplyOutcomeId,
        order: ReceiptOrderToken,
        receipt: SignedReceipt<ApplyOutcome, Plain>,
    ) -> Result<PlacedDocument, PlacementFailure> {
        self.file::<ApplyOutcome, Plain>(id, order, receipt)
            .map(|(placed, _)| placed)
    }
}

/// The store's own refusal, in the placement's closed words.
///
/// Borrowed, and the refusal is DROPPED by the caller that maps it: dropping an ownership token
/// is a legitimate outcome and leaves bounded partial evidence no later writer replaces. Removing
/// what a failed publication left is a separate, deliberate act.
fn placement_failure(refusal: &PublishRefusal) -> PlacementFailure {
    match refusal.reason() {
        PublishFailure::OverReceiptBound => PlacementFailure::OverBound,
        PublishFailure::NameAlreadyTaken => PlacementFailure::NameAlreadyTaken,
        PublishFailure::NameUnmintable | PublishFailure::RootUnusable => PlacementFailure::Unusable,
        PublishFailure::CreateFailed
        | PublishFailure::WriteIncomplete
        | PublishFailure::SyncFailed
        | PublishFailure::BaselineUnmet => PlacementFailure::NotDurable,
    }
}

/// The operating system's randomness, for key generation.
///
/// A second seat asking for randomness beside the receipt-identity one, and deliberately so:
/// they are different questions, and a generator that borrowed the identity source would make a
/// run's key material a function of how many documents it had minted.
#[derive(Debug, Default)]
pub struct OsKeyEntropy;

impl KeySecretEntropy for OsKeyEntropy {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        getrandom::getrandom(raw).is_ok()
    }
}

/// The production key generator this binary initializes a first-use keyset with.
pub type OsKeysetGenerator = EntropyKeysetGenerator<OsKeyEntropy>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    struct Environment(BTreeMap<&'static str, &'static str>);

    impl RootEnvironment for Environment {
        fn var(&self, name: &str) -> Option<String> {
            self.0
                .get(name)
                .filter(|value| !value.is_empty())
                .map(|value| (*value).to_owned())
        }
    }

    fn environment(pairs: &[(&'static str, &'static str)]) -> Environment {
        Environment(pairs.iter().copied().collect())
    }

    #[test]
    fn the_unix_roots_prefer_the_xdg_variables_and_fall_back_to_the_home_defaults() {
        // Both halves in one case, because the FALLBACK is the interesting half: an environment
        // that sets neither variable is the ordinary one, and a resolution that only worked when
        // XDG was set would leave the shipped binary with nowhere to write on most machines.
        let named = standard_roots(
            RootPlatform::OtherUnix,
            &environment(&[
                ("HOME", "/home/x"),
                ("XDG_CONFIG_HOME", "/cfg"),
                ("XDG_STATE_HOME", "/state"),
            ]),
        )
        .expect("both bases are absolute");
        assert_eq!(named.base(RootRole::Configuration), "/cfg");
        assert_eq!(named.base(RootRole::State), "/state");

        let bare = standard_roots(
            RootPlatform::OtherUnix,
            &environment(&[("HOME", "/home/x")]),
        )
        .expect("the home defaults are absolute");
        assert_eq!(bare.base(RootRole::Configuration), "/home/x/.config");
        assert_eq!(bare.base(RootRole::State), "/home/x/.local/state");
    }

    #[test]
    fn an_environment_answering_neither_base_refuses_rather_than_landing_somewhere() {
        // There is no fallback to a cwd, a repository, or a temporary directory: a run with
        // nowhere to put a durable says so, because a durable written somewhere nobody named is
        // worse than one not written.
        let refusal = standard_roots(RootPlatform::OtherUnix, &environment(&[]))
            .expect_err("no HOME and no XDG leaves no base");
        assert_eq!(
            refusal,
            RootRefusal::ControllerRootUnavailable {
                role: RootRole::Configuration
            }
        );
    }

    #[test]
    fn the_windows_roots_come_from_the_two_distinct_profile_variables() {
        let roots = standard_roots(
            RootPlatform::Windows,
            &environment(&[
                ("APPDATA", "C:\\Users\\x\\AppData\\Roaming"),
                ("LOCALAPPDATA", "C:\\Users\\x\\AppData\\Local"),
            ]),
        )
        .expect("both bases are absolute");
        assert_ne!(
            roots.base(RootRole::Configuration),
            roots.base(RootRole::State),
            "the two roles are separate directories on Windows, which is what selective \
             propagation of state without keys rests on"
        );
    }

    #[test]
    fn no_dorc_specific_variable_appears_in_the_resolution() {
        // The stop condition in its own words: a Dorc-specific variable selecting a provider, a
        // key, a store, or a weaker policy. Lexical, because the property is about which NAMES
        // this seat can spell.
        let source = include_str!("durable.rs");
        let rule = source
            .split("mod tests")
            .next()
            .expect("the module body precedes its tests");
        for spelled in rule.match_indices("var(\"") {
            let (_, tail) = spelled;
            let named = rule[spelled.0..]
                .strip_prefix("var(\"")
                .and_then(|rest| rest.split('"').next())
                .unwrap_or(tail);
            assert!(
                !named.to_ascii_uppercase().contains("DORC"),
                "{named} is a Dorc-specific variable, and root resolution reads only the \
                 platform's own"
            );
        }
    }
}
