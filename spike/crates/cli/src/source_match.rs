//! Whether the run's book is a commit, for the receipt header's annotation line.
//!
//! `28E:lean-git-source-tracking-secondary`: if the book is in version control, say which commit it
//! is, so "diff against the last good night" starts from a name instead of a guess. Secondary by
//! construction -- it must never block the plain "I slept, why did it break overnight" path, so
//! every failure here is silence.
//!
//! # The fence this cannot cross (`28E:nack-whylog-stores-book-bytes`)
//!
//! ANNOTATION TIER ONLY. This module answers one question -- is the book at HEAD, and under what
//! commit -- and hands back a name. It never reads book bytes OUT of git and never puts them into a
//! render. The receipt durable stays thin; git is asked about identity, never for content.
//!
//! # Exact-or-absent, and why there is no "HEAD has drifted" answer here
//!
//! Naming the commit a drifted book DOES match needs a history walk (`git log -- <path>`, then a
//! blob compare per candidate), which is a different size and a different failure surface. It is
//! deferred (`plans/28G` W3 shipped the matches-HEAD form only). A miss is silent rather than
//! partially-informative: "your book is not at HEAD" without saying what it IS would send a
//! firefighter looking for a change that may not exist.
//!
//! # The harness answer is a seam, not a fixture read here
//!
//! Whether to ask git or return a pinned answer is a member of the seam bundle now
//! (`crate::seam::SourceMatchSeam`): the shipped binary selects `Os` (this module's real
//! [`GitRepository`]); a harness selects `pinned:off` / `pinned:<commit>`. This module holds only
//! the production query (`30X:inv-fixture-state-never-typeable-into-main`).

use std::path::Path;
use std::process::Command;

/// The ANSWER this module resolves. It lives across the loom seam (`dorc_cli`) because the render
/// holds it and the render is drivable there; the QUERY — [`resolve`] and the git subprocess below
/// — stays here at the I/O edge (`lib-target-is-a-loom-seam`).
pub(crate) use crate::SourceMatch;

/// The narrow read this needs from a repository. One impl asks git; tests supply their own.
pub(crate) trait SourceRepository {
    /// The short commit id of `HEAD`, or `None` when there is no repository to ask.
    fn head_commit(&self, within: &Path) -> Option<String>;
    /// Whether `path` is tracked and byte-identical to its `HEAD` blob.
    fn is_unmodified_at_head(&self, path: &Path) -> Option<bool>;
}

/// Does the book sit at HEAD, and under which commit? `None` for every uncertainty.
pub(crate) fn resolve(repository: &impl SourceRepository, book: &Path) -> Option<SourceMatch> {
    let within = book.parent().unwrap_or_else(|| Path::new("."));
    repository
        .is_unmodified_at_head(book)?
        .then(|| {
            repository
                .head_commit(within)
                .map(|commit| SourceMatch { commit })
        })
        .flatten()
}

/// The production edge: the real `git`.
#[derive(Debug)]
pub(crate) struct GitRepository;

impl GitRepository {
    /// Run one read-only git query in `within`, or `None` if git is absent, fails, or is not UTF-8.
    ///
    /// No timeout, deliberately noted: `Command::output` blocks, so a git hung on an unresponsive
    /// network filesystem would hang `dorc why` with it. The mitigation available without a thread
    /// or a dependency is to ask git as little as possible, which is why this runs twice per run at
    /// most and never walks history. `churn-avoidance-disclosure`: a real tool wants a timeout here.
    fn query(within: &Path, args: &[&str]) -> Option<String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(within)
            .output()
            .ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8(output.stdout).ok())
            .flatten()
    }
}

impl SourceRepository for GitRepository {
    fn head_commit(&self, within: &Path) -> Option<String> {
        Self::query(within, &["rev-parse", "--short", "HEAD"])
            .map(|out| out.trim().to_owned())
            .filter(|commit| !commit.is_empty())
    }

    fn is_unmodified_at_head(&self, path: &Path) -> Option<bool> {
        let within = path.parent().unwrap_or_else(|| Path::new("."));
        let name = path.file_name()?.to_str()?;
        // `diff --quiet` would call an UNTRACKED file unchanged; `status --porcelain` will not.
        Some(
            Self::query(within, &["status", "--porcelain", "--", name])?
                .trim()
                .is_empty(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fake {
        tracked_and_clean: Option<bool>,
        head: Option<String>,
    }

    impl SourceRepository for Fake {
        fn head_commit(&self, _within: &Path) -> Option<String> {
            self.head.clone()
        }
        fn is_unmodified_at_head(&self, _path: &Path) -> Option<bool> {
            self.tracked_and_clean
        }
    }

    /// The annotation appears ONLY on the exact answer. The three near-misses each have a tempting
    /// wrong rendering -- "modified since HEAD", "in git somewhere", "at an unknown commit" -- and
    /// all three would be a provenance claim the engine cannot back
    /// (`28E:rul-never-a-dinna-do-it-layer`'s spirit: absence of evidence is not a conclusion).
    #[test]
    fn only_an_exact_match_annotates() {
        let book = Path::new("web.sh");
        assert_eq!(
            resolve(
                &Fake {
                    tracked_and_clean: Some(true),
                    head: Some("9f31c2e".to_owned())
                },
                book
            ),
            Some(SourceMatch {
                commit: "9f31c2e".to_owned()
            }),
            "clean, tracked, and HEAD is nameable"
        );
        assert_eq!(
            resolve(
                &Fake {
                    tracked_and_clean: Some(false),
                    head: Some("9f31c2e".to_owned())
                },
                book
            ),
            None,
            "modified or untracked says nothing"
        );
        assert_eq!(
            resolve(
                &Fake {
                    tracked_and_clean: Some(true),
                    head: None
                },
                book
            ),
            None,
            "a repository that cannot name HEAD says nothing"
        );
        assert_eq!(
            resolve(
                &Fake {
                    tracked_and_clean: None,
                    head: Some("9f31c2e".to_owned())
                },
                book
            ),
            None,
            "no repository at all says nothing"
        );
    }
}
