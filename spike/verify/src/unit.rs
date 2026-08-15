//! Reading a law unit (`301` §1) — discovery, slug discipline, and the declaration contract.
//!
//! The unit file is the AUTHORED artifact and everything else is derived from it. This module
//! never writes one: unit content is frontier-authored under `301:law-spec-touch-frontier-human-only`,
//! and the binder's whole value rests on being an acceptance surface its own maintainers
//! cannot write to.
//!
//! # The declaration contract (STRAWMAN names; `rul-strawman-formats-no-compat`)
//!
//! A written unit `<Slug>.lean` declares:
//!
//! * `def <Slug> : Prop := …` — the law, stated over the derived definitions.
//! * `theorem <Slug>_nonvacuous : …` — the anti-vacuity probe: one positive witness whose
//!   precondition is genuinely satisfied. Without it a green battery proves nothing, because
//!   a vacuously-true implication is green forever.
//! * `example`/`#guard` battery entries — the boundary cases (empty · singleton · ⊤) and the
//!   worked examples that are the review surface for non-proof-literate readers.
//!
//! and its proof, if any, lives at `Minispec/Proofs/<Slug>.lean` as
//! `theorem <Slug>_holds : <Slug> := …`.
//!
//! An UNWRITTEN unit carries [`UNWRITTEN_MARKER`] and asserts nothing. It is a legal resting
//! state: every badge reads `todo`, the report nags, and nothing pretends.

use std::path::{Path, PathBuf};

/// The line a stub unit carries so the binder — and the next model to open the file — can tell
/// "no law here yet" from "a law that fails its checks".
pub const UNWRITTEN_MARKER: &str = "UNWRITTEN";

/// The advisory byte-length budget for a unit file (`301` §1 byte-budget tripwire).
///
/// Bytes, never lines: the enemy is how much an LLM must load to hold the whole law, and a
/// wrapped 400-byte paragraph costs the same attention however many lines it occupies.
///
/// Calibrated (`301` §7 item 4) against the only real evidence available at v0 — the
/// hand-written Lean law units in the sparing-algebra research spike, whose largest law-bearing
/// file is ~7.5 KB and whose median is ~3 KB — then set ABOVE the largest of them, because the
/// tripwire must fire on decomposition-worthy growth and not on the first genuinely rich law.
/// It is advisory: exceeding it prints one consider-decomposing line and nothing else.
/// Readability and sanity trump the limit.
pub const BYTE_BUDGET: usize = 12_288;

/// What a unit file says, at the coarsest grain that matters: the three states are genuinely
/// different, and collapsing any two of them loses the distinction between "not written yet"
/// (fine, and marked) and "written wrong" (a failure).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Statement {
    /// A marked stub. Asserts nothing, deliberately; every badge reads `todo`.
    Unwritten,
    /// `def <Slug> : Prop` is declared.
    Stated,
    /// Neither marked nor stating a law — a unit file that is a law about nothing.
    Missing,
}

/// One law unit as read off disk.
#[derive(Clone, Debug)]
pub struct Unit {
    /// The `DromedaryCase` slug — the file stem, the catalogue key, and the demonstration
    /// loom's stem, all ONE identical string.
    pub slug: String,
    /// Repo-relative path to the unit file.
    pub path: PathBuf,
    /// The file's byte length, for the tripwire.
    pub bytes: usize,
    /// What the file says.
    pub statement: Statement,
    /// Whether the anti-vacuity probe is declared.
    pub has_nonvacuity_probe: bool,
    /// How many battery entries (`example` / `#guard`) the unit carries.
    pub battery_entries: usize,
    /// Whether the file carries a proof hole.
    pub has_hole: bool,
}

/// Every hole spelling that makes a Lean declaration unproven. Counted, never tolerated
/// silently: a `sorry` typechecks, so anything downstream of one is vacuous
/// (`301` §0's sorry-census law, and the turn08 finding that made it one — a green lake
/// build proves nothing without a census, because lenient translation emits SILENT holes).
pub const HOLE_SPELLINGS: [&str; 3] = ["sorry", "sorryAx", "admit"];

/// Read every unit under `minispec/Minispec/`, sorted by slug.
///
/// The two path segments differ only in case, and on a case-insensitive filesystem a wrong
/// root silently resolves to the package directory instead of the unit directory — so the
/// join happens here, once, from the repo root, rather than at each call site.
///
/// # Errors
/// When the directory cannot be listed or a unit cannot be read.
pub fn load_all(repo_root: &Path) -> Result<Vec<Unit>, String> {
    let dir = repo_root.join("minispec").join("Minispec");
    let mut units = Vec::new();
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "lean") {
            units.push(read(&path)?);
        }
    }
    units.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(units)
}

/// Read one unit file.
///
/// # Errors
/// When the file cannot be read or has no stem.
pub fn read(path: &Path) -> Result<Unit, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("{}: unreadable file stem", path.display()))?
        .to_owned();
    Ok(Unit {
        bytes: text.len(),
        statement: if text.contains(UNWRITTEN_MARKER) {
            Statement::Unwritten
        } else if text.contains(&format!("def {slug} : Prop")) {
            Statement::Stated
        } else {
            Statement::Missing
        },
        has_nonvacuity_probe: text.contains(&format!("theorem {slug}_nonvacuous")),
        battery_entries: text
            .lines()
            .filter(|line| {
                let l = line.trim_start();
                l.starts_with("example") || l.starts_with("#guard")
            })
            .count(),
        has_hole: contains_hole(&text),
        slug,
        path: path.to_path_buf(),
    })
}

/// Whether `text` carries a proof hole, as a whole word.
///
/// Whole-word so an identifier like `sorryFree` does not trip it, but deliberately NOT
/// comment-aware: a census's safe direction is over-detection. A false positive is one loud
/// line a human clears in seconds; a false negative is the halo this whole system exists to
/// prevent. Prose near a law says "hole", not the word itself.
#[must_use]
pub fn contains_hole(text: &str) -> bool {
    HOLE_SPELLINGS.iter().any(|hole| {
        text.split(|c: char| !c.is_alphanumeric() && c != '_')
            .any(|word| word == *hole)
    })
}

/// Whether `slug` obeys the naming law: `DromedaryCase`, at least three full English words, no
/// separators (`301` §1 — Lean forbids hyphens in module names, so law slugs leave kebab-case
/// and lean fully into the ecosystem's convention).
///
/// The dividend the convention buys: `DromedaryCase` in prose or in a filename means a hard law
/// with machinery behind it, kebab-case means a soft corpus reference. That only holds if the
/// shape is mechanical.
#[must_use]
pub fn slug_is_well_formed(slug: &str) -> bool {
    let starts_upper = slug.chars().next().is_some_and(char::is_uppercase);
    let alnum_only = slug.chars().all(char::is_alphanumeric);
    starts_upper && alnum_only && word_count(slug) >= 3
}

/// The number of `DromedaryCase` words in `slug`.
#[must_use]
pub fn word_count(slug: &str) -> usize {
    slug.chars().filter(|c| c.is_uppercase()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_discipline_demands_three_dromedary_words() {
        assert!(slug_is_well_formed("UnknownMemberCollides"));
        assert!(slug_is_well_formed("JoinIsCommutativeOverPowerset"));
        assert!(
            !slug_is_well_formed("JoinCommutative"),
            "two words is short"
        );
        assert!(
            !slug_is_well_formed("unknownMemberCollides"),
            "lowercase head"
        );
        assert!(
            !slug_is_well_formed("Unknown_Member_Collides"),
            "separators"
        );
        assert!(
            !slug_is_well_formed("unknown-member-collides"),
            "kebab-case"
        );
    }

    #[test]
    fn hole_detection_is_whole_word_and_errs_toward_detecting() {
        // A hole census may not be dodgeable by a name that merely contains the word, and its
        // errors must fall on the loud side: over-detection costs a human one glance, while
        // under-detection is a vacuous proof nobody notices.
        assert!(contains_hole("theorem x : P := by\n  sorry\n"));
        assert!(contains_hole("  admit"));
        assert!(contains_hole("axiom foo : sorryAx"));
        assert!(
            contains_hole("-- a comment carrying the bare word sorry"),
            "not comment-aware, deliberately: over-detection is the safe direction"
        );
        assert!(!contains_hole("def sorryFree : Prop := True"), "identifier");
        assert!(!contains_hole("def not_sorry : Prop := True"), "identifier");
    }
}
