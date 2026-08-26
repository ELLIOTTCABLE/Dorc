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
//! This is the contracts stage: names, bounds, states, publication properties, the manifest
//! grammar, and the deterministic I/O vocabulary with its failure schedule. There is no
//! production key generation and no production store here, and nothing in this crate can select
//! one.
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
//!         _: dorc_receipt_local::io::Op,
//!         _: &str,
//!     ) -> Result<(), dorc_receipt_local::io::IoFault> {
//!         Ok(())
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

pub mod io;
pub mod keyset;
pub mod limits;
pub mod manifest;
pub mod model;
pub mod names;
pub mod roots;
pub mod store;

pub use keyset::KeyAvailability;
pub use limits::LocalLimits;
pub use manifest::{KeyRole, KeysetManifest, ManifestRefusal};
pub use names::{NameRefusal, NamedSpecies, ReceiptFileName};
pub use roots::{RootInputs, RootPlatform, RootRefusal, RootRole};
pub use store::{
    DirectorySync, EntryStanding, EnumerateFailure, IncompleteState, PlatformBaseline,
    PublicationProperties, PublishFailure, StoreReadFailure,
};
