//! The translation unit charon compiles: the facade'd algebra, and nothing else.
//!
//! This file holds NO algebra of its own. It `#[path]`-includes the real
//! `core::sorted` and `analysis::lattice` sources, so the thing translated is the
//! thing shipped — a copy would drift, and a drifting derived definition makes every
//! law stated over it a statement about code that no longer exists.
//!
//! `extern crate self as dorc_core` is what lets `lattice.rs`'s own
//! `use dorc_core::sorted::…` resolve here without editing it.
//!
//! Scope, deliberately: `analysis::effect::Reach` is EXCLUDED
//! (`300:fnd-reach-lattice-outside-scope` — it still holds a raw `BTreeSet` behind a
//! hand-written cause-excluding `Eq`, whose eviction is deferred). `solve.rs` is
//! excluded too: it is not algebra, it depends on `cfg`, and no admitted law needs it
//! yet. Widening this file is a deliberate act, not a convenience.

#![allow(unused)]

extern crate self as dorc_core;

#[path = "../../../crates/core/src/sorted.rs"]
pub mod sorted;

#[path = "../../../crates/analysis/src/lattice.rs"]
pub mod lattice;
