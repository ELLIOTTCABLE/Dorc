//! The derived-definitions drift alarm.
//!
//! `minispec/Generated/` is committed so a regeneration diff is reviewable, and every law is
//! stated over it. Nothing tied it to the Rust it came from: the translator runs Linux-side,
//! by hand, and whether the committed Lean still described the shipping algebra was a question
//! only somebody re-running the pipeline could answer. This module records the answer.
//!
//! # What it is, and what it deliberately is not
//!
//! It is a DRIFT ALARM: a digest that changes when the translation's inputs change, so a reader
//! is told to re-derive. It is not an identity, an authenticity claim, or evidence about the
//! derived Lean — a matching digest says only "the inputs have not moved since the recorded
//! derivation", never "the translation is correct". Nothing may read it as trust, and the hash
//! is a plain FNV-1a chosen for being dependency-free and deterministic (`301` keeps
//! correctness-critical kernels dependency-clean, and this instrument ships to nobody).
//!
//! It is also WARN-TIER, not a gate. Regeneration needs a Linux-only toolchain and produces a
//! diff a human reviews; a gate that can only be cleared on one platform by a heavyweight
//! manual act is a gate people learn to route around. So the alarm speaks, loudly, and the
//! report carries the RECORDED digest rather than a live verdict — which keeps the artifact's
//! bytes stable while the sources move, and keeps the freshness check a freshness check.
//!
//! # Over-detection is the safe direction
//!
//! The digest is over whole source files, so a change the translator provably ignores — a
//! `#[cfg(kani)]` block, a doc comment — still trips it. That is deliberate, and it is the same
//! call the hole census makes: a false alarm costs one re-derivation whose diff is empty, while
//! a missed one is a law stated over code that no longer exists.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::relative;

/// The translation unit charon compiles. Its `#[path]` includes ARE the algebra being derived.
const TRANSLATION_UNIT: [&str; 2] = ["spike", "verify"];

/// Where the recorded digests live.
#[must_use]
pub fn path(repo_root: &Path) -> PathBuf {
    aeneas_dir(repo_root).join("derivation.lock")
}

fn aeneas_dir(repo_root: &Path) -> PathBuf {
    let mut dir = PathBuf::from(repo_root);
    for segment in TRANSLATION_UNIT {
        dir.push(segment);
    }
    dir.join("aeneas")
}

/// Every file the translation reads, sorted, repo-relative.
///
/// Discovered rather than listed: the unit's `#[path]` includes are parsed out of it, so
/// widening the translation — which its own header calls a deliberate act — cannot silently
/// leave a source outside the alarm. The fence in `Cargo.toml` joins them because it decides
/// what is translated versus axiomatized, which changes the output as surely as the code does.
///
/// # Errors
/// When the translation unit cannot be read — it is committed, so its absence is a broken
/// checkout rather than a finding.
pub fn inputs(repo_root: &Path) -> Result<Vec<PathBuf>, String> {
    let dir = aeneas_dir(repo_root);
    let unit = dir.join("src").join("lib.rs");
    let text = std::fs::read_to_string(&unit).map_err(|e| format!("{}: {e}", unit.display()))?;
    let mut found = vec![dir.join("Cargo.toml"), unit.clone()];
    for line in text.lines() {
        let Some(rest) = line.trim().strip_prefix("#[path = \"") else {
            continue;
        };
        if let Some((target, _)) = rest.split_once('"') {
            found.push(normalize(&dir.join("src").join(target)));
        }
    }
    found.sort();
    found.dedup();
    Ok(found)
}

/// Resolve `..` segments without touching the filesystem, so the recorded path is the one a
/// reader recognizes rather than the include's relative spelling.
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for part in path.components() {
        if part.as_os_str() == ".." {
            out.pop();
        } else if part.as_os_str() != "." {
            out.push(part);
        }
    }
    out
}

/// The lock's whole text, computed from the sources as they are right now.
///
/// # Errors
/// When an input cannot be read.
pub fn compute(repo_root: &Path) -> Result<String, String> {
    let mut out = String::from(HEADER);
    for input in inputs(repo_root)? {
        let bytes = std::fs::read(&input).map_err(|e| format!("{}: {e}", input.display()))?;
        let _ = writeln!(out, "{}  {}", digest(&bytes), relative(repo_root, &input));
    }
    Ok(out)
}

/// What the last recorded derivation digested, if anything has been recorded.
#[must_use]
pub fn recorded(repo_root: &Path) -> Option<String> {
    std::fs::read_to_string(path(repo_root)).ok()
}

/// One short digest standing for the whole recorded set — what the report cites.
#[must_use]
pub fn short(lock: &str) -> String {
    digest(lock.as_bytes())
}

/// The alarm: which inputs have moved since the recorded derivation.
///
/// `Ok(None)` is agreement. `Ok(Some(_))` is the warning text, naming the files rather than
/// only the fact, because "something moved" sends a reader to re-derive without telling them
/// whether to expect a diff.
///
/// # Errors
/// When the current sources cannot be read.
pub fn drift(repo_root: &Path) -> Result<Option<String>, String> {
    let Some(was) = recorded(repo_root) else {
        return Ok(Some(format!(
            "DERIVATION UNRECORDED — no digest at {}, so minispec/Generated/ is committed with \
             nothing tying it to the Rust it came from. `mise run verify:translate` records one",
            relative(repo_root, &path(repo_root))
        )));
    };
    let is = compute(repo_root)?;
    if was == is {
        return Ok(None);
    }
    let moved: Vec<&str> = is
        .lines()
        .filter(|line| !line.starts_with('#') && !was.lines().any(|old| old == *line))
        .filter_map(|line| line.split_whitespace().nth(1))
        .collect();
    Ok(Some(format!(
        "DERIVATION DRIFT — minispec/Generated/ was translated from source that has since MOVED \
         ({}). Every law is stated over the committed translation, so re-derive with `mise run \
         verify:translate` (Linux/WSL) and review the diff. A digest is a drift alarm, never \
         evidence: an empty re-derivation diff is what closes this",
        moved.join(", ")
    )))
}

/// FNV-1a, 64-bit, over LF-normalized bytes.
///
/// Normalized because the two platform legs check the same file out with different line
/// endings, and a digest that disagreed across legs would put a permanent false alarm on
/// whichever leg did not record it.
#[must_use]
pub fn digest(bytes: &[u8]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for byte in bytes.iter().filter(|b| **b != b'\r') {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("{hash:016x}")
}

const HEADER: &str = "\
# @generated by `dorc-verify materialize` (the `verify:translate` lane) — do not edit by hand.
#
# One digest per file the translation reads, as of the derivation that produced the committed
# minispec/Generated/. A DRIFT ALARM only: a match says the inputs have not moved, never that
# the translation is right, and nothing may read it as trust.
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_digest_ignores_line_endings_and_nothing_else() {
        // Both legs check out the same file; a digest that disagreed across them would put a
        // permanent false alarm on whichever leg did not record it.
        assert_eq!(digest(b"a\r\nb\r\n"), digest(b"a\nb\n"));
        assert_ne!(digest(b"a\nb\n"), digest(b"a\nc\n"));
        assert_ne!(digest(b""), digest(b" "));
    }

    #[test]
    fn the_include_targets_are_discovered_rather_than_listed() {
        // The translation unit's own header calls widening it a deliberate act. Discovery is
        // what stops a widening from leaving a new source outside the alarm — silently, and
        // exactly when the alarm matters most.
        let repo = std::env::temp_dir().join("dorc-verify-derivation-pin");
        let src = repo.join("spike").join("verify").join("aeneas").join("src");
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("lib.rs"),
            "#[path = \"../../../crates/core/src/sorted.rs\"]\npub mod sorted;\n",
        )
        .unwrap();

        let found: Vec<String> = inputs(&repo)
            .unwrap()
            .iter()
            .map(|p| relative(&repo, p))
            .collect();
        assert_eq!(
            found,
            vec![
                "spike/crates/core/src/sorted.rs".to_owned(),
                "spike/verify/aeneas/Cargo.toml".to_owned(),
                "spike/verify/aeneas/src/lib.rs".to_owned(),
            ],
            "the include resolves through its `..` segments, and the fence joins it"
        );
        std::fs::remove_dir_all(&repo).unwrap();
    }
}
