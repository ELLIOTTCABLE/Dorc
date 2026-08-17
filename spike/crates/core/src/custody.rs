//! `core::custody` — whose utterance a licence may rest on, once files can source each other
//! (`28Q` §2 `syn-closure-is-the-speaker`; `30C:dec-custody-is-containment-not-equivalence`).
//!
//! Sourcing is a PROMISE — "I treated this as if I wrote it" (`28M` §10
//! `dir-ownership-is-transitive-inclusion`) — so a file's author answers for everything their file
//! pulls in. That makes custody a CONTAINMENT relation over the include-tree, not an equivalence
//! over files:
//!
//! ```text
//! custody(F) = {F} ∪ transitive descendants of F along speaker-minting edges
//! ```
//!
//! # Why containment, and not one identity per closure
//!
//! A scalar closure-id compared with `==` merges SIBLINGS. An entry sourcing two strangers' files
//! would put both under one id and make them one speaker, which is exactly the reading the `28R`
//! review found DISSOLVES the fence. Containment gives the ruled shape directly: an ANCESTOR edge
//! takes custody of what it sources, while sibling and cousin edges fence — so the two-strangers
//! entry holds two mutually-fenced units beneath its own custody of the composition.
//!
//! The relation is therefore asymmetric, and [`reaches`](CustodyClosures::reaches) reads
//! `asker` first for that reason. `28M` bitem3 reserved this re-key as "a change to the custody
//! type's internals; consumers still only compare" — they do, and there is no consumer churn,
//! but the compare a containment relation supports is directional rather than `==`.
//!
//! # What counts as a speaker-minting edge (`307:§ack-implementation-open`, human-typed)
//!
//! Only an explicit TOP-LEVEL `.` in a MARKED file, of a file that is itself dorc-lang oracle code
//! with no top-level commands. Two exclusions carry the ruling's whole weight, and this module
//! only ever sees edges the driver already filtered:
//!
//! - **CLI co-loading composes nothing.** Naming files on one command line is INGESTION. It mints
//!   no edge, so no load-order arrangement of strangers' files lets one serve another's vouch.
//! - **A book's `.` mints no speaker.** A book contributes no edges at all; its sourcing buys
//!   un-walling and nothing else.
//!
//! The admission itself is a HYGIENE CONTRACT, never an engine proof of inertness
//! (`30C:rul-inertness-is-contract-never-engine-fact`): the licence grounds on the marker plus the
//! author's no-top-level-commands promise, and a refusal attributes to that contract.

use crate::{SortedSet, SourceFileId};

/// The include-tree's transitive closure, per source file — built once at the driver edge and
/// consulted wherever a licence must not cross an authorship boundary.
///
/// Deliberately tiny: one question, asked one way. Everything about resolving paths, reading files,
/// and checking the dorc-lang contract happens at the edge that builds this, so the relation itself
/// stays pure data and every seat that consumes it stays a pure function (`inv-determinism`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CustodyClosures {
    /// Per source index, the files that index's custody reaches — ALWAYS including itself, so the
    /// empty-edge world is exactly the singleton world and needs no special case at consumers.
    reaches: Vec<SortedSet<u32>>,
}

impl CustodyClosures {
    /// Every file its own closure: the no-oracle-sourcing world, which is today's entire corpus.
    ///
    /// This is what makes the whole re-key byte-identical where nothing sources anything — the
    /// predicate reduces to "the resolved declaration is in the asker's own file", term for term
    /// the pre-sourcing rule. Every lane that holds no include-tree (the instrument, hint, and
    /// hand-built-index seats) passes this and keeps its current answers.
    #[must_use]
    pub fn singletons(count: usize) -> Self {
        Self {
            reaches: (0..count)
                .map(|i| SortedSet::from_iter([u32::try_from(i).unwrap_or(u32::MAX)]))
                .collect(),
        }
    }

    /// Build from the driver's speaker-minting `(sourcer, sourced)` edges, transitively closed.
    ///
    /// Edges naming an out-of-range index are DROPPED rather than panicking (`inv-no-throw`): the
    /// driver owns the source vector, and a stale index is a driver bug whose safe reading here is
    /// "no custody", which withholds. Cycles are tolerated — a file sourcing its way back to
    /// itself is legal sh (the second `.` re-applies the same declarations), and the fixpoint below
    /// terminates because the reachable set only grows and is bounded by the file count.
    #[must_use]
    pub fn from_edges(count: usize, edges: &[(usize, usize)]) -> Self {
        let mut closures = Self::singletons(count);
        let mut changed = true;
        while changed {
            changed = false;
            for &(sourcer, sourced) in edges {
                if sourcer >= count {
                    continue;
                }
                let Some(child) = closures.reaches.get(sourced).cloned() else {
                    continue;
                };
                for file in &child {
                    changed |= closures.reaches[sourcer].insert(*file);
                }
            }
        }
        closures
    }

    /// THE licensure question: does `asker`'s custody reach `target`?
    ///
    /// An index outside the built range reaches NOTHING, which withholds — the direction a lookup
    /// against an unknown file must fail in.
    #[must_use]
    pub fn reaches(&self, asker: usize, target: usize) -> bool {
        u32::try_from(target).ok().is_some_and(|target| {
            self.reaches
                .get(asker)
                .is_some_and(|reached| reached.contains(&target))
        })
    }

    /// Whether any file's custody reaches beyond itself — the single-closure world's own test.
    ///
    /// The migration gate (`28Q` §8 `syn-single-frame-byte-identical`, closure half) says a
    /// single-closure world is byte-identical, and this is how a seat asserts it is looking at one.
    #[must_use]
    pub fn all_singleton(&self) -> bool {
        self.reaches.iter().all(|reached| reached.len() <= 1)
    }
}

/// Custody as a definition-grade value carries the same relation
/// ([`DefinitionCustody`](crate::DefinitionCustody) names the defining file; this answers whether
/// one such custody reaches another's file).
///
/// Kept beside the relation rather than as an inherent method on `DefinitionCustody`, because the
/// answer needs the run's include-tree and that type is `Copy` data with no room for it. The type
/// doc's "never read the file id to decide anything" stands: this is the sanctioned read, at the
/// one seat that holds the tree.
#[must_use]
pub fn custody_reaches(
    closures: &CustodyClosures,
    asker: crate::DefinitionCustody,
    target: crate::DefinitionCustody,
) -> bool {
    closures.reaches(
        index_of(asker.defining_file()),
        index_of(target.defining_file()),
    )
}

/// The source index a [`SourceFileId`] denotes (`28O:dec-load-order-is-the-id-order`: load order IS
/// the id order), for a seat holding one and needing the other.
#[must_use]
pub const fn index_of(file: SourceFileId) -> usize {
    file.0 as usize
}

#[cfg(test)]
mod tests {
    use super::CustodyClosures;

    /// The world every lane without an include-tree passes, and today's whole corpus: each file
    /// reaches itself and nothing else. This is the byte-identity floor — the re-keyed predicate
    /// must reduce to the pre-sourcing one here or the migration gate is meaningless.
    #[test]
    fn singletons_reach_only_themselves() {
        let closures = CustodyClosures::singletons(3);
        assert!(closures.reaches(0, 0));
        assert!(!closures.reaches(0, 1));
        assert!(!closures.reaches(2, 1));
        assert!(closures.all_singleton());
    }

    /// An ancestor takes custody of what it sources, transitively — the sh fact
    /// `floor30-sourcing-is-transitive` measures, made a licence. Without transitivity a closure
    /// deeper than one edge would ship bodies no execution binds.
    #[test]
    fn an_ancestor_reaches_its_whole_subtree() {
        let closures = CustodyClosures::from_edges(3, &[(0, 1), (1, 2)]);
        assert!(closures.reaches(0, 1));
        assert!(
            closures.reaches(0, 2),
            "custody is transitive through the middle file"
        );
        assert!(closures.reaches(1, 2));
        assert!(!closures.all_singleton());
    }

    /// ...and a descendant never reaches back up. Sourcing is a promise the SOURCER makes; the
    /// sourced file made none, so a helpers file's own vouch may not rest on its entrypoints.
    #[test]
    fn a_descendant_does_not_reach_its_ancestor() {
        let closures = CustodyClosures::from_edges(2, &[(0, 1)]);
        assert!(!closures.reaches(1, 0));
    }

    /// THE fence: two files sourced by one entry are strangers to each other. A scalar
    /// closure-identity compared with `==` would merge them, which is the reading `28Q` §2 records
    /// as dissolving the fence entirely — so this is the test that would catch that regression.
    #[test]
    fn siblings_under_one_entry_stay_fenced() {
        let closures = CustodyClosures::from_edges(3, &[(0, 1), (0, 2)]);
        assert!(closures.reaches(0, 1) && closures.reaches(0, 2));
        assert!(!closures.reaches(1, 2), "siblings fence");
        assert!(!closures.reaches(2, 1), "in both directions");
    }

    /// Cousins fence too — the deeper form of the same rule, and the one a naive "same root"
    /// implementation passes the sibling test while getting wrong.
    #[test]
    fn cousins_under_one_root_stay_fenced() {
        let closures = CustodyClosures::from_edges(5, &[(0, 1), (0, 2), (1, 3), (2, 4)]);
        assert!(closures.reaches(0, 3) && closures.reaches(0, 4));
        assert!(!closures.reaches(3, 4));
        assert!(!closures.reaches(1, 4));
    }

    /// A diamond gives BOTH entries custody of the shared file, which is the honest answer: each
    /// one sourced it and each one promised for it. Nothing here has to pick a single owner, which
    /// is why the closed `pin-closure-membership-and-diamond` could allow diamonds at all.
    #[test]
    fn a_diamond_gives_both_entries_custody() {
        let closures = CustodyClosures::from_edges(3, &[(0, 2), (1, 2)]);
        assert!(closures.reaches(0, 2) && closures.reaches(1, 2));
        assert!(
            !closures.reaches(0, 1),
            "the two entries are still strangers"
        );
    }

    /// A cycle terminates and behaves like mutual custody, which is what sh does: the second `.`
    /// re-applies the same declarations and both files' authors promised for the other.
    #[test]
    fn a_cycle_terminates_and_reaches_both_ways() {
        let closures = CustodyClosures::from_edges(2, &[(0, 1), (1, 0)]);
        assert!(closures.reaches(0, 1) && closures.reaches(1, 0));
    }

    /// A stale or out-of-range index reaches nothing rather than panicking — the withholding
    /// direction, and `inv-no-throw` besides.
    #[test]
    fn out_of_range_lookups_withhold() {
        let closures = CustodyClosures::from_edges(2, &[(0, 9), (9, 0)]);
        assert!(!closures.reaches(9, 0));
        assert!(!closures.reaches(0, 9));
        assert!(closures.reaches(0, 0));
    }
}
