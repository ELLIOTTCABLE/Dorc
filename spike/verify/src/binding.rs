//! Bindings (`301` §2): the two-way agreement between a bound loom and the catalogue.
//!
//! A loom PROPOSES itself as law evidence by declaring [`crate::catalogue::BINDING_KEY`] in
//! its frontmatter; only a catalogue promote ACCEPTS it. Both halves are checked here, both
//! directions, because each silence is a different failure: a proposal nobody accepted is an
//! author who believes they armed something, and an accepted binding whose case stopped
//! declaring itself is evidence that quietly walked away.
//!
//! Demonstrations deliberately do NOT live in `minispec/`. They are ordinary whole-product
//! looms sited where they project-purpose-belong, run by the unchanged central runners, which
//! also keeps loom churn in builder-space cleanly outside the spec-touch frontier.

use std::path::{Path, PathBuf};

use crate::catalogue::{BINDING_KEY, LawRow};

/// One loom that declares itself law evidence.
#[derive(Clone, Debug)]
pub struct Proposal {
    /// Repo-relative path to the case.
    pub case: String,
    /// The law slug it claims to test.
    pub slug: String,
}

/// Every loom under `spike/crates/*/tests/` declaring [`BINDING_KEY`], sorted by case path.
///
/// # Errors
/// When a case declares the key but cannot be parsed as a loom.
pub fn proposals(repo_root: &Path) -> Result<Vec<Proposal>, String> {
    let mut found = Vec::new();
    let crates = repo_root.join("spike").join("crates");
    let Ok(entries) = std::fs::read_dir(&crates) else {
        return Ok(found);
    };
    for entry in entries.flatten() {
        collect_from_tests(&entry.path().join("tests"), repo_root, &mut found)?;
    }
    found.sort_by(|a, b| a.case.cmp(&b.case));
    Ok(found)
}

fn collect_from_tests(
    tests: &Path,
    repo_root: &Path,
    found: &mut Vec<Proposal>,
) -> Result<(), String> {
    let Ok(entries) = std::fs::read_dir(tests) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let is_loom = path.extension().is_some_and(|e| e == "loom");
        // The flat-test-tree shapes: `X.loom` and `X/X.loom` (`288:rul-flat-test-tree`).
        let candidate = if is_loom {
            Some(path.clone())
        } else if path.is_dir() {
            path.file_name()
                .map(|stem| path.join(stem).with_extension("loom"))
                .filter(|p| p.is_file())
        } else {
            None
        };
        if let Some(case) = candidate
            && let Some(slug) = declared_slug(&case)?
        {
            found.push(Proposal {
                case: relative(repo_root, &case),
                slug,
            });
        }
    }
    Ok(())
}

/// The law slug a case declares, if any.
///
/// Read through errorloom's own container parser, never a second frontmatter reader: two
/// parsers disagreeing about what a case says is how an assertion silently stops being one.
fn declared_slug(case: &Path) -> Result<Option<String>, String> {
    let text = std::fs::read_to_string(case).map_err(|e| format!("{}: {e}", case.display()))?;
    let parsed = errorloom::Case::parse(&text)
        .map_err(|e| format!("{}: unparseable loom ({e:?})", case.display()))?;
    Ok(parsed
        .frontmatter()
        .scalar(BINDING_KEY)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned))
}

fn relative(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// One direction of disagreement between the proposals and the catalogue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Disagreement {
    /// A case declares a law that no catalogue row accepts as a binding. The author armed
    /// nothing; the fix is a promote, or deleting the key.
    Unaccepted {
        /// The case that declares it.
        case: String,
        /// The law slug declared.
        slug: String,
    },
    /// The catalogue accepts a binding whose case no longer declares the key — the case was
    /// edited or moved by someone who did not know it was law evidence.
    Undeclared {
        /// The case the catalogue names.
        case: String,
        /// The law whose row names it.
        slug: String,
    },
    /// The catalogue names a case that is not on disk at all.
    Missing {
        /// The case the catalogue names.
        case: String,
        /// The law whose row names it.
        slug: String,
    },
}

/// Check both directions of the binding agreement.
#[must_use]
pub fn disagreements(
    laws: &[LawRow],
    proposals: &[Proposal],
    repo_root: &Path,
) -> Vec<Disagreement> {
    let mut out = Vec::new();
    for law in laws {
        for binding in law.bindings {
            let on_disk = PathBuf::from(repo_root).join(binding.case);
            if !on_disk.is_file() {
                out.push(Disagreement::Missing {
                    case: binding.case.to_owned(),
                    slug: law.slug.to_owned(),
                });
                continue;
            }
            let declared = proposals
                .iter()
                .any(|p| p.case == binding.case && p.slug == law.slug);
            if !declared {
                out.push(Disagreement::Undeclared {
                    case: binding.case.to_owned(),
                    slug: law.slug.to_owned(),
                });
            }
        }
    }
    for proposal in proposals {
        let accepted = laws.iter().any(|law| {
            law.slug == proposal.slug && law.bindings.iter().any(|b| b.case == proposal.case)
        });
        if !accepted {
            out.push(Disagreement::Unaccepted {
                case: proposal.case.clone(),
                slug: proposal.slug.clone(),
            });
        }
    }
    out
}

/// Render one disagreement as the line a refusal prints.
#[must_use]
pub fn describe(d: &Disagreement) -> String {
    match d {
        Disagreement::Unaccepted { case, slug } => format!(
            "{case} declares `{BINDING_KEY}: {slug}` but no catalogue row accepts it \
             (promote the binding, or drop the key — a proposal arms nothing)"
        ),
        Disagreement::Undeclared { case, slug } => format!(
            "{slug}'s binding names {case}, which no longer declares `{BINDING_KEY}` \
             (the case stopped being law evidence without the catalogue noticing)"
        ),
        Disagreement::Missing { case, slug } => {
            format!("{slug}'s binding names {case}, which is not on disk")
        }
    }
}
