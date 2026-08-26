//! `dorc-receipt-local` — the default local durable edge: one keyset under the configuration
//! root, one immutable receipt store under the state root, and the bounded acts that reach them.
//!
//! # What this crate is, and is not
//!
//! It STORES exact already-signed bytes and SUPPLIES already-validated key capabilities. It does
//! not parse a receipt semantically, mint a trusted or complete receipt state, render a
//! diagnostic, inspect a host's bytes, or select an algorithm. Every one of those lives in
//! `dorc-receipt` or above it, so the ordering they depend on — verify before interpret — is
//! enforced by this crate boundary rather than by anyone remembering to call in the right order.
//!
//! Nothing here reads the environment. Root resolution happens at the process edge and arrives as
//! a value ([`roots::RootInputs`]), which is what keeps the whole crate drivable by a
//! deterministic model.
//!
//! # What the baseline claims
//!
//! That a receipt copy travelling WITHOUT the configuration keyset keeps its opaque material
//! closed, and that a keyset recognized as ready was completely initialized. It claims nothing
//! against the account that owns the keys, nothing about a copy carrying both, and no equivalence
//! between what Unix ownership and mode bits say and what an inherited Windows profile ACL says.
//! No surface may call any of it scrubbed, secret-free, or safe to share.
//!
//! # Stage
//!
//! The keyset is live: root validation, the exclusive first-use initialization sequence, ordinary
//! reopen validation, and a read-only open that is a separate entry point rather than a mode of
//! the write one. There is no receipt store here yet, and nothing in this crate can select a
//! provider — the one generator is the value a caller hands in.
//!
//! # The seals, as compile-fail pins
//!
//! Each pin is paired with the positive control that proves it fails for the stated reason rather
//! than because the example was malformed.
//!
//! The filesystem surface is sealed, so no type from outside supplies one — a production route
//! cannot be handed some other filesystem, and that is a property of the type. Naming the trait is
//! fine; implementing it is not:
//!
//! ```
//! fn takes(io: &mut dyn dorc_receipt_local::io::LocalIo) -> dorc_receipt_local::DirectorySync {
//!     io.directory_sync()
//! }
//! ```
//!
//! ```compile_fail
//! struct MyFilesystem;
//! impl dorc_receipt_local::io::LocalIo for MyFilesystem {
//!     fn perform(
//!         &mut self,
//!         _: dorc_receipt_local::io::Request<'_>,
//!         _: &str,
//!     ) -> Result<dorc_receipt_local::io::Answer, dorc_receipt_local::io::IoFault> {
//!         Ok(dorc_receipt_local::io::Answer::Done)
//!     }
//!     fn directory_sync(&self) -> dorc_receipt_local::DirectorySync {
//!         dorc_receipt_local::DirectorySync::Synchronized
//!     }
//! }
//! ```
//!
//! A publication proof is what a policy check consumes, and it exists because operations
//! succeeded. It has no `Default`, so no caller can produce one it did not earn:
//!
//! ```
//! use dorc_receipt_local::{DirectorySync, PublicationProperties};
//! let earned = PublicationProperties::of(true, true, true, DirectorySync::Synchronized);
//! assert!(earned.file_is_durable());
//! ```
//!
//! ```compile_fail
//! let _unearned: dorc_receipt_local::PublicationProperties = Default::default();
//! ```
//!
//! And the two platform baselines do not sit on one ladder, so there is no ordering to compare
//! them by:
//!
//! ```compile_fail
//! use dorc_receipt_local::{DirectorySync, PublicationProperties};
//! let a = PublicationProperties::of(true, true, true, DirectorySync::Synchronized);
//! let b = PublicationProperties::of(true, true, true, DirectorySync::UnavailableOnPlatform);
//! let _stronger = a > b;
//! ```
//!
//! A read open and a write open are separate entry points, and their answers are separate types.
//! Nothing narrows one into the other, so a route that asked why cannot end up holding something
//! that publishes:
//!
//! ```compile_fail
//! fn wants(_: dorc_receipt_local::LocalWriteOpenV1) {}
//! fn give(read: dorc_receipt_local::LocalReadOpenV1) { wants(read); }
//! ```
//!
//! ```compile_fail
//! fn wants(_: dorc_receipt_local::LocalWriteKeysV1) {}
//! fn give(read: dorc_receipt_local::LocalReadKeysV1) { wants(read); }
//! ```

// The dependency-graph fact `dorc-receipt-crypto` already carries, inherited by naming it: `age`
// reaches two major lines of several hashing crates through separate subtrees, which
// `-D warnings` then makes fatal. No version choice avoids it, and `deny.toml` sets
// `multiple-versions = "warn"` for the workspace. `expect`, so it warns once the duplication
// clears.
#![expect(
    clippy::multiple_crate_versions,
    reason = "a transitive-dependency fact; see the note above"
)]

pub mod io;
pub mod keyset;
pub mod limits;
pub mod manifest;
pub mod model;
pub mod names;
pub mod native;
pub mod roots;
pub mod store;

pub use keyset::{
    KeyAvailability, KeysetLocation, LocalReadKeysV1, LocalReadOpenV1, LocalWriteKeysV1,
    LocalWriteOpenV1, PermissionSubject, StorePresence, open_for_read,
    open_or_initialize_for_write,
};
pub use limits::LocalLimits;
pub use manifest::{KeyRole, KeysetManifest, ManifestRefusal};
pub use names::{LocalPath, NameRefusal, NamedSpecies, ReceiptFileName};
pub use native::NativeIo;
pub use roots::{RootInputs, RootPlatform, RootRefusal, RootRole};
pub use store::{
    BoundedReceiptEntries, CleanupFailure, DirectorySync, EntryStanding, EnumerateFailure,
    GraphAggregateBudget, HeaderClaims, IncompletePublicationOwned, IncompleteState,
    LocalReceiptStoreV1, LocalRequiredReceiptPolicyV1, MaximumOrderCohort, NameAgreement,
    NameComponent, OwnedReceiptEntry, PlatformBaseline, PublicationProperties, PublishFailure,
    PublishRefusal, RequiredLocalPublicationV1, StoreLimits, StoreOpenRefusal, StoreReadFailure,
    StoredReceiptRead, StoredSpecies, UnrecognizedEntry, store_root,
};
