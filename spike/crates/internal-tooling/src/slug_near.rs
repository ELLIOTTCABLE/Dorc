//! The advisory `near:` line for `SLUGS.md`: a few corpus spans that discuss a slug's concept
//! WITHOUT citing it, so a design that strayed into existing territory under a new name surfaces
//! beside the canonical key.
//!
//! The ranker is SEMANTIC ("the point is to catch nearby concepts") and lives OUTSIDE our own
//! dependency tree. `internal-tooling` keeps a hard zero-native-dependency bar — its `Cargo.toml`
//! explains why (fingerprint stability against the shared workspace build, and disk in a crate whose
//! own job includes guarding it) — so the embedding backend is the external `ck` binary, resolved
//! through `mise`, never a crate we compile. `build_ranker` returns `None` when that binary or its
//! model is absent, and `slugs::resolve_near` then carries the previous line forward verbatim, which
//! is why the near line is allowed to be stale and the build never needs a network or a model.

use std::collections::BTreeSet;

/// Named in `SLUGS.md`'s header so a reader knows how the `near:` lines were computed. Per-machine
/// by nature (an embedding backend), hence advisory and excluded from `--check`.
pub(crate) const FINGERPRINT: &str = "the ck semantic search tool (BGE-Small embeddings)";

/// A source of advisory `near:` hits for one slug's concept.
pub(crate) trait NearRanker {
    /// Up to three `path:line` spans discussing `query`, one per document, excluding the documents
    /// that define or cite the slug (`excluded`, keyed by docID). `None` when nothing qualifies.
    fn near(&self, query: &str, excluded: &BTreeSet<String>) -> Option<String>;
}

/// The ranker to use for this run, or `None` when no backend is available (⇒ carry the old line
/// forward). The `ck`-backed implementation is wired separately; a dependency-free build has none.
pub(crate) fn build_ranker(_files: &[(String, String)]) -> Option<Box<dyn NearRanker>> {
    None
}
