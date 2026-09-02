//! The advisory `near:` line for `SLUGS.md`: a few corpus spans that discuss a slug's concept
//! WITHOUT citing it, so a design that strayed into existing territory under a new name surfaces
//! beside the canonical key.
//!
//! NOT WIRED YET — this is the seam, not a backend. The near line is meant to be SEMANTIC ("the
//! point is to catch nearby concepts"), and any embedding backend has to live OUTSIDE this crate:
//! `internal-tooling` keeps a hard zero-native-dependency bar (its `Cargo.toml` explains why —
//! fingerprint stability against the shared workspace build, and disk in a crate whose own job
//! includes guarding it), so a backend is an external subprocess, never a crate we compile.
//! `build_ranker` returns `None` until one is wired; until then `slugs::resolve_near` carries the
//! previous line forward / computes nothing, and `SLUGS.md` simply has no `near:` lines. When a
//! backend does land, the staleness rule keeps the cost down: a row's line refreshes only when that
//! row's other lines change.

use std::collections::BTreeSet;

/// A source of advisory `near:` hits for one slug's concept.
pub(crate) trait NearRanker {
    /// Up to three `path:line` spans discussing `query`, one per document, excluding the documents
    /// that define or cite the slug (`excluded`, keyed by docID). `None` when nothing qualifies.
    fn near(&self, query: &str, excluded: &BTreeSet<String>) -> Option<String>;
}

/// The ranker to use for this run, or `None` when no backend is wired (⇒ carry the old line forward
/// / compute nothing). No backend is wired today, so this is always `None`.
pub(crate) fn build_ranker(_files: &[(String, String)]) -> Option<Box<dyn NearRanker>> {
    None
}
