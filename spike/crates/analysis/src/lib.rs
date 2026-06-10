//! `dorc-analysis` — Dorc's static-analysis engine.
//!
//! A generic **monotone-dataflow framework** (the classical Kildall/Kam–Ullman
//! construction: a finite-height complete lattice + monotone per-node transfer
//! functions, solved to the least fixed point by a worklist) plus Dorc's
//! sh-specific CFG construction and effect analyses on top of it.
//!
//! The framework ([`lattice`] + [`solve`]) is pure, deterministic,
//! and analysis-agnostic; it knows nothing about shell. The shell-specific
//! modeling lives in the `cfg`/`effect` modules. Design + the why behind every
//! choice: `Research/notes/163-analysis-engine-design-spa-grounded.md`.

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; this crate-root expect
// ratchets away during the rebuild (an unfulfilled `expect` warns, so it
// self-removes as the seeded layer is replaced). It never relaxes the policy
// for new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

pub mod cfg;
pub mod effect;
pub mod lattice;
pub mod solve;
pub mod value;
