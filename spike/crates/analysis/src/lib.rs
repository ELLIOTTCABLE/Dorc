//! `dorc-analysis` — Dorc's static-analysis engine.
//!
//! A generic **monotone-dataflow framework** (the classical Kildall/Kam–Ullman
//! construction: a finite-height complete lattice + monotone per-node transfer
//! functions, solved to the least fixed point by a worklist) plus Dorc's
//! sh-specific CFG construction and effect analyses on top of it.
//!
//! The framework ([`lattice`] + the forthcoming `solve`) is pure, deterministic,
//! and analysis-agnostic; it knows nothing about shell. The shell-specific
//! modeling lives in the `cfg`/`effect` modules. Design + the why behind every
//! choice: `Research/notes/163-analysis-engine-design-spa-grounded.md`.

#![forbid(unsafe_code)]

pub mod cfg;
pub mod lattice;
pub mod solve;
