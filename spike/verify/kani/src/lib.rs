//! The Kani verification unit: the algebra tier's real sources, and the harnesses over them.
//!
//! Like `../aeneas`, this file holds NO algebra of its own — it `#[path]`-includes
//! `crates/core/src/lib.rs` and `crates/analysis/src/lattice.rs`, so the thing verified is the
//! thing shipped. A copy would drift, and a drifted copy makes every harness over it a claim
//! about code that no longer exists.
//!
//! Two mechanics make the include work, and both are load-bearing:
//!
//! * `pub use core_included::*` lifts core's public items to THIS crate's root, which is what
//!   lets the included submodules' own `crate::EntityRef` / `crate::sorted::SortedSet` paths
//!   resolve unedited.
//! * `extern crate self as dorc_core` does the same job for `lattice.rs`'s
//!   `use dorc_core::sorted::…` — the aeneas unit's trick, for the same reason. The second
//!   alias, `dorc_analysis`, is for the harnesses: it lets them spell their imports exactly as
//!   a consumer of the real crates would, so a harness reads as ordinary client code.
//!
//! # What this tier is FOR, and what it cannot be asked
//!
//! The seat tests in `sorted.rs` and `lattice.rs` are examples: they check the cases their
//! author thought of. These harnesses check EVERY case at their declared bounds, which is a
//! different kind of claim — and it is the kind the asymmetric risk needs (`300` §2a). A bug
//! that makes two semantically-DIFFERENT values compare equal stops the solver's climb early:
//! an under-approximated may-set, so a potential wrong elision, and one no golden can see. The
//! opposite bug only costs a `converged: false`.
//!
//! Bounds are part of every statement. "Verified" here means "verified for every value up to
//! N", never "verified", so each harness declares its N in its doc-comment. Raising one to
//! dodge a failure is the forbidden move (`law-never-weaken-the-question`): a counterexample is
//! a finding about the code or about the law, never harness noise.
//!
//! Scope, deliberately: `analysis::effect::Reach` is absent (it is `BTreeSet`-backed, which
//! this tier cannot reach — `300:fnd-reach-lattice-outside-scope`), and so is `plan`'s span-edit
//! machinery. Both carry exhaustive-small twins in their own crates' suites instead.

#![allow(unused)]

extern crate self as dorc_core;
extern crate self as dorc_analysis;

#[path = "../../../crates/core/src/lib.rs"]
mod core_included;
pub use core_included::*;

#[path = "../../../crates/analysis/src/lattice.rs"]
pub mod lattice;

#[cfg(kani)]
mod harness;
