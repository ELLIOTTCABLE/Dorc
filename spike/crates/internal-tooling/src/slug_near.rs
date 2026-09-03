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
//!
//! Why none is wired (measured 2026-09-02, so nobody re-walks it): every in-tree route
//! (`model2vec-rs`, `ck-search` from source) reaches `HuggingFace` `tokenizers`, whose C/C++ deps
//! (`onig`/`esaxx`) fail to build on Windows — `cargo install ck-search` dies at link on an MSVC
//! static-vs-dynamic CRT mismatch. The prebuilt `ck` 0.7.11 binary installs on every platform via
//! mise, but its `--sem` scores were 0.000 on Windows and on WSL over drvfs, and its indexer hung
//! on WSL ext4; its `--hybrid` scores are RRF ranks, unusable as a floor. macOS is untested and is
//! the one cheap experiment left (`ck --index` a two-file dir, one `--sem --scores` query, expect
//! cosines near 0.6–0.9 for the related file). Only if that works is an adapter behind this seam
//! worth writing; its index must then sit outside the tree (`CK_INDEX_DIR`).

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
