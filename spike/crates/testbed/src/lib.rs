//! The suite's shared substrate — NOT part of Dorc. See `Cargo.toml` for the charter.
//!
//! Dependency-free (std only) and sited below `cli`, so both `harness = false` runners, the loom
//! driver, and product-crate tests all reach it. [`seam_vars`] is the seam-variable vocabulary (the
//! `DORC_SEED`/`DORC_SEAM_*` names; `cli::seam` owns the parser and imports these). [`run_seed`] is
//! the ONE run-seed seat — "what is this run's seed", drawn once, printed and named the same way
//! everywhere.

pub mod run_seed;
pub mod seam_vars;
