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

#![allow(
    dead_code,
    reason = "one shared module, two harness binaries: the looms runner uses only its own half, so `expect` would go unfulfilled in the e2e binary that uses all of it"
)]

use std::collections::BTreeSet;
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

/// Every entry name directly under `roots`, `.loom` stripped — the vocabulary a caller's case
/// path may name. Deliberately WIDER than either walker: a name here that neither walker claims
/// is an `.rs` test's fixture space (the table above's last row), which is a path the runners
/// drive no trial for rather than a path that is not there. Telling those two apart is what lets
/// a scoped runner report honestly instead of aborting.
#[must_use]
pub(crate) fn case_root_names(roots: &[PathBuf]) -> BTreeSet<String> {
    roots
        .iter()
        .flat_map(|root| sorted_entries(root))
        .map(|(name, _)| name.strip_suffix(".loom").unwrap_or(&name).to_owned())
        .collect()
}

/// The case name a `crates/<c>/tests/<case>[.loom][/...]` path belongs to.
pub(crate) fn case_from_path(argument: &str) -> Option<String> {
    let normalized = argument.replace('\\', "/");
    let segment = normalized.split_once("/tests/")?.1.split('/').next()?;
    let case = segment.strip_suffix(".loom").unwrap_or(segment);
    (!case.is_empty()).then(|| case.to_owned())
}

/// Split argv into libtest's own arguments and the case paths a caller wants scoped.
///
/// A pre-commit hook knows which files are staged but not which trials they name, and libtest's
/// single substring filter cannot express a set — which is why selection happens here.
pub(crate) fn split_path_selectors<I: Iterator<Item = String>>(
    argv: I,
) -> (Vec<String>, BTreeSet<String>) {
    let (mut passthrough, mut cases) = (Vec::new(), BTreeSet::new());
    for argument in argv {
        if let Some(case) = case_from_path(&argument) {
            cases.insert(case);
        } else {
            passthrough.push(argument);
        }
    }
    (passthrough, cases)
}

/// What one path-selected case name resolves to in this run.
#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Selection {
    /// A trial of this run answers to the name.
    Runs,
    /// A case root answers to the name, but this runner mints no trial for it: an `aid` catalog
    /// loom under the e2e runner, an `.rs` test's fixture dir, a lane that is off. Benign.
    NoTrial,
    /// Nothing under any case root answers to the name: a typo, a stale path, or a collection
    /// that moved. A caller bug, and silence is its failure mode.
    Unknown,
}

/// Resolve one path-selected case name against the run set and the case roots' own vocabulary.
pub(crate) fn resolve_selection(
    name: &str,
    minted: &BTreeSet<&str>,
    present: &BTreeSet<String>,
) -> Selection {
    if minted.contains(name) {
        Selection::Runs
    } else if present.contains(name) {
        Selection::NoTrial
    } else {
        Selection::Unknown
    }
}

/// Resolve a path selection against one runner's minted trial names, reporting the two ways it
/// can select nothing. Returns whether any retained trial remains to run.
///
/// The discovery floor, applied to scoping: selecting by path and running nothing must never be
/// SILENT — that is how a hook reports success for work it never ran. A name no case root answers
/// to is a caller bug and aborts; a real case this runner drives no trial for is benign and
/// reports. Shared because the second copy is how the first rots: the looms runner had no path
/// selection at all, so a case PATH fell through to libtest's substring filter, matched no trial
/// name, and exited green.
pub(crate) fn report_path_selection(
    selected: &BTreeSet<String>,
    minted: &BTreeSet<&str>,
    roots: &[PathBuf],
) -> bool {
    let present = case_root_names(roots);
    let (mut no_trial, mut unknown): (Vec<&str>, Vec<&str>) = (Vec::new(), Vec::new());
    for name in selected {
        match resolve_selection(name, minted, &present) {
            Selection::Runs => {}
            Selection::NoTrial => no_trial.push(name),
            Selection::Unknown => unknown.push(name),
        }
    }
    if !unknown.is_empty() {
        eprintln!(
            "FATAL  path selection names no case: {} — no `crates/*/tests/` entry answers to it (a typo, a stale path, or a collection that moved).",
            unknown.join(", ")
        );
        eprintln!("aborting.");
        std::process::exit(3);
    }
    if no_trial.len() == selected.len() {
        eprintln!(
            "no trial here for: {} — a path this runner drives nothing for (no `run:` key, or an `.rs` test's fixture space).",
            no_trial.join(", ")
        );
        return false;
    }
    true
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
            // A SyncThing conflict copy beside a case is never a case (dorc-loom's walkers
            // hold the same rule); loading one is a duplicate-slug refusal at best.
            if name.contains(".sync-conflict-") {
                continue;
            }
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
