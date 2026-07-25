//! Shared case discovery for the two central runners (`288` §3 `rul-flat-test-tree`).
//!
//! A case is DATA; its own entry-point file names its kind, so no marker file has to be
//! minted and a collection move stays a pure rename:
//!
//! | on disk                  | kind                                              |
//! |--------------------------|---------------------------------------------------|
//! | `<case>.loom`            | single-file loom                                  |
//! | `<case>/<case>.loom`     | multi-file loom                                   |
//! | `<case>/cmd`             | a `dorc lint` case                                |
//! | `<case>/book.sh` + `expected.out` | a round-trip case                        |
//! | `<case>/book.sh` alone   | a real-tools lint fixture (opt-in lane)           |
//! | anything else            | an `.rs` test's fixture space — not a case        |
//!
//! `paths-are-manifest-relative` (`crates/aid/CLAUDE.md`): [`case_roots`] is resolved from
//! `CARGO_MANIFEST_DIR` and is depth-coupled to `crates/<c>/` — move this crate and the
//! walk finds nothing, silently. `crates/cli/tests/e2e.rs`'s discovery-floor trial is the
//! tripwire that makes that failure loud.

#![expect(
    dead_code,
    reason = "one shared module, two harness binaries: each runner uses its own half"
)]

use std::path::{Path, PathBuf};

/// The `spike/` workspace root (this crate lives at `spike/crates/cli`).
#[must_use]
pub(crate) fn spike_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .ancestors()
        .nth(2)
        .unwrap_or(manifest)
        .to_path_buf()
}

/// Every `crates/<c>/tests/` dir, sorted (`inv-determinism`). Both runners walk the same
/// roots and select by shape, so a collection can be re-homed to whichever crate owns its
/// steering (`288:rul-slug-decides-loom-placement`) without touching either runner.
#[must_use]
pub(crate) fn case_roots() -> Vec<PathBuf> {
    let crates = spike_root().join("crates");
    let Ok(entries) = std::fs::read_dir(&crates) else {
        return Vec::new();
    };
    let mut roots: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path().join("tests"))
        .filter(|dir| dir.is_dir())
        .collect();
    roots.sort();
    roots
}

/// What a discovered dir-form case is driven as.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum E2eKind {
    /// The whole-pipeline round-trip: book + oracles → probe → results → eliding apply.
    RoundTrip,
    /// A `dorc lint` case: `cmd` carries the flags, `expected.out` the hand-authored render.
    Lint,
    /// A real-external-linter fixture, driven only under `DORC_E2E_REAL_TOOLS`.
    LintReal,
}

/// One discovered dir-form case.
#[derive(Clone, Debug)]
pub(crate) struct E2eCase {
    /// The trial name — the case dir's own name.
    pub(crate) name: String,
    /// The case dir, absolute.
    pub(crate) dir: PathBuf,
    /// How the runner drives it.
    pub(crate) kind: E2eKind,
}

/// One discovered loom case.
#[derive(Clone, Debug)]
pub(crate) struct LoomCase {
    /// The trial name — the loom's slug (its file stem, or its dir name).
    pub(crate) name: String,
    /// The `.loom` file itself, absolute.
    pub(crate) path: PathBuf,
}

/// Directory entries of `root`, sorted by name (`inv-determinism`); absent root ⇒ empty.
fn sorted_entries(root: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    let mut found: Vec<(String, PathBuf)> = entries
        .flatten()
        .map(|entry| {
            (
                entry.file_name().to_string_lossy().into_owned(),
                entry.path(),
            )
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    found
}

/// Is this dir the multi-file form of a loom case (`<case>/<case>.loom`)?
fn multi_file_loom(name: &str, dir: &Path) -> Option<PathBuf> {
    let inner = dir.join(format!("{name}.loom"));
    inner.is_file().then_some(inner)
}

/// Walk `roots` for dir-form cases. Panics on a duplicate trial name — two cases sharing
/// a name would make the suite silently ambiguous under a filter.
#[must_use]
pub(crate) fn discover_e2e(roots: &[PathBuf]) -> Vec<E2eCase> {
    let mut cases: Vec<E2eCase> = Vec::new();
    for root in roots {
        for (name, path) in sorted_entries(root) {
            if !path.is_dir() || multi_file_loom(&name, &path).is_some() {
                continue;
            }
            let kind = if path.join("cmd").is_file() {
                E2eKind::Lint
            } else if !path.join("book.sh").is_file() {
                continue;
            } else if path.join("expected.out").is_file() {
                E2eKind::RoundTrip
            } else {
                E2eKind::LintReal
            };
            assert!(
                !cases.iter().any(|case| case.name == name),
                "duplicate case name `{name}` across roots"
            );
            cases.push(E2eCase {
                name,
                dir: path,
                kind,
            });
        }
    }
    cases
}

/// Walk `roots` for `.loom` cases in both sanctioned shapes.
#[must_use]
pub(crate) fn discover_looms(roots: &[PathBuf]) -> Vec<LoomCase> {
    let mut cases: Vec<LoomCase> = Vec::new();
    for root in roots {
        for (name, path) in sorted_entries(root) {
            let found = if path.is_dir() {
                multi_file_loom(&name, &path)
            } else if path.extension().is_some_and(|ext| ext == "loom") {
                Some(path)
            } else {
                None
            };
            let Some(found) = found else { continue };
            let slug = name.strip_suffix(".loom").unwrap_or(&name).to_owned();
            assert!(
                !cases.iter().any(|case| case.name == slug),
                "duplicate loom name `{slug}` across roots"
            );
            cases.push(LoomCase {
                name: slug,
                path: found,
            });
        }
    }
    cases
}
