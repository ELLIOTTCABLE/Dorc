//! `core::region` — the **elision region**: the authored span a shared transformation edits, and
//! the iteration dimension a route instance carries (`plans/30L` §2, §7).
//!
//! # Two identities, never conflated (`30L:rul-two-identities-never-conflated`)
//!
//! A [`LeafId`](crate::LeafId) is EXECUTION identity: which command runs, which probe record keys
//! to it, which `Step` the artifact renders. An [`ElisionRegion`] is EDIT identity: the one
//! authored source span every invocation of a function body would rewrite. They are different
//! questions and they answer differently — a body region has many executions and exactly one edit,
//! which is the whole reason a per-call decision cannot be the unit
//! (`spike/CLAUDE.md inv-leaf-seam`).
//!
//! # Why the universe is a value, and why it is checked at the mint
//!
//! Elision regions exist ONLY on the book-custody surface (`30L:rul-region-universe-is-book-custody`):
//! the book plus its non-dorc-lang sourced tree. dorc-lang files are contracted non-mutative, so
//! their interiors are not attention product and receive no elision work at any tier — an
//! EXCLUSION, not a deferral. Checking it at the sole mint is what makes "no region is ever minted
//! inside a dorc-lang file" a property of the type rather than a rule every future consumer must
//! remember, the same shape `plan::erase`'s licence fence and `cli`'s contested withdrawal take.
//!
//! The universe is pure data built once at the driver edge, exactly like
//! [`CustodyClosures`](crate::custody::CustodyClosures): everything about reading files and testing
//! the dorc-lang marker happens at that edge, so this module stays a pure function.

use crate::{DefinitionCustody, DefinitionId, SortedSet, SourceFileId, Span};

/// The one authored source span all instances of a body region would edit — definition-keyed, and
/// mintable only inside the book-custody universe.
///
/// Keyed by [`DefinitionId`] and NOT by function name: two definitions of one name are two
/// regions, because they are two authored texts and a shared edit to one is not a shared edit to
/// the other (`30L:pin-definition-not-name`). The span is the REGION's own extent inside that
/// definition, so one definition holds as many regions as it holds command leaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElisionRegion {
    definition: DefinitionId,
    span: Span,
}

impl ElisionRegion {
    /// THE mint. `None` when the definition's custody is outside the book-custody universe — the
    /// dorc-lang exclusion, enforced here so no consumer can forget it.
    #[must_use]
    pub fn mint(universe: &RegionUniverse, definition: DefinitionId, span: Span) -> Option<Self> {
        universe
            .admits(definition.custody())
            .then_some(ElisionRegion { definition, span })
    }

    /// The definition this region lives inside — the identity that keeps same-named definitions
    /// apart.
    #[must_use]
    pub const fn definition(self) -> DefinitionId {
        self.definition
    }

    /// The authored byte range a shared transformation would edit.
    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }
}

/// Which loaded files may hold elision regions: the book-custody surface, and nothing else.
///
/// Built at the driver edge from the loaded source set — the edge knows which files carry the
/// `# dorc-lang/v0.2` marker; this value only answers. An EMPTY universe admits nothing, which is
/// the safe reading for a lane holding no source table: no regions, no shared decisions, and the
/// engine behaves exactly as it did before regions existed
/// (`30L:pin-empty-function-world-parity`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegionUniverse {
    admitted: SortedSet<u32>,
}

impl RegionUniverse {
    /// The universe over the files the edge measured to be book-custody (non-dorc-lang).
    #[must_use]
    pub fn of_book_custody_files(files: impl IntoIterator<Item = SourceFileId>) -> Self {
        Self {
            admitted: files.into_iter().map(|file| file.0).collect(),
        }
    }

    /// Does this custody sit on the book surface? The sanctioned read of a custody's file, at the
    /// one seat that holds the universe — the shape `custody::custody_reaches` already takes, and
    /// for the same reason: `DefinitionCustody`'s own doc forbids consumers keying on the raw file
    /// id, and this is the seat that is allowed to.
    #[must_use]
    pub fn admits(&self, custody: DefinitionCustody) -> bool {
        self.admitted.contains(&custody.defining_file().0)
    }
}

/// The member/iteration dimension of one analyzed instance (`30L` §7,
/// `rul-one-call-site-is-not-one-evaluation`).
///
/// A syntactically singular call inside an authored loop is MANY evaluations, so instance identity
/// needs an axis that CFG position does not supply: a loop body is lowered ONCE, with a real
/// back-edge, and every iteration executes those same nodes. This is that axis, present in the
/// types NOW so the propagation lane can turn an `Open` population into a closed member population
/// without re-keying any identity or witness (`30L:pin-loop-types-need-no-rekey`).
///
/// Member indices are positions in the loop's ORDERED, NON-deduplicated list: `for x in a a` is two
/// members, because dash iterates twice and each iteration is its own evaluation
/// (`30N` §2, the `20S` member commitments). The index is exactly what
/// [`SiteId::member`](crate::SiteId::member) carries, so a member route and its `site N.M` record
/// speak one numbering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IterationSlot {
    /// The instance evaluates at most once per route — the region is not inside an authored loop.
    NotIterated,
    /// The instance is member `index` of an authored loop's ordered member list.
    Member(u32),
}

impl IterationSlot {
    /// The member index, in the spelling [`SiteId`](crate::SiteId) takes — so a route instance and
    /// the probe record measuring it cannot disagree about which member they mean.
    #[must_use]
    pub const fn member(self) -> Option<u32> {
        match self {
            IterationSlot::NotIterated => None,
            IterationSlot::Member(index) => Some(index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ElisionRegion, IterationSlot, RegionUniverse};
    use crate::{BytePos, DefinitionId, SiteId, SourceFileId, Span};

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(BytePos(lo), BytePos(hi))
    }

    fn def(file: u32, lo: u32) -> DefinitionId {
        DefinitionId::at(SourceFileId(file), span(lo, lo + 100))
    }

    fn book_only() -> RegionUniverse {
        RegionUniverse::of_book_custody_files([SourceFileId(7)])
    }

    /// `30L:pin-region-universe-excludes-dorc-lang`, stated as the mint's own refusal: a definition
    /// whose custody is not on the book surface yields NO region, so no later stage can be handed
    /// one to decide about. The exclusion is a ruling, not a deferral — dorc-lang interiors are
    /// contracted non-mutative and are simply not attention product.
    #[test]
    fn a_definition_outside_the_book_surface_mints_no_region() {
        let universe = book_only();
        assert!(ElisionRegion::mint(&universe, def(7, 0), span(10, 20)).is_some());
        assert!(ElisionRegion::mint(&universe, def(3, 0), span(10, 20)).is_none());
    }

    /// The empty universe admits nothing. This is the byte-identity floor: a lane that never built
    /// a source table gets no regions rather than a permissive default, so the engine's behaviour
    /// is exactly its pre-region behaviour (`30L:pin-empty-function-world-parity`).
    #[test]
    fn an_empty_universe_admits_nothing() {
        let universe = RegionUniverse::default();
        assert!(ElisionRegion::mint(&universe, def(0, 0), span(10, 20)).is_none());
    }

    /// `30L:pin-definition-not-name`: two definitions are two regions even at the same span offsets
    /// within their own bodies, because identity runs through the DEFINITION. A name-keyed region
    /// would silently pool two authors' texts under one shared edit.
    #[test]
    fn same_named_definitions_never_share_a_region() {
        let universe = RegionUniverse::of_book_custody_files([SourceFileId(7)]);
        let first = ElisionRegion::mint(&universe, def(7, 0), span(10, 20));
        let second = ElisionRegion::mint(&universe, def(7, 400), span(10, 20));
        assert!(first.is_some() && second.is_some());
        assert_ne!(first, second);
    }

    /// One definition holds as many regions as it holds command leaves — the grain is the leaf
    /// shape, and this stage changes the region's VENUE, not its GRAIN
    /// (`30L:rul-elision-region-is-the-unit`).
    #[test]
    fn one_definition_holds_many_regions() {
        let universe = book_only();
        let d = def(7, 0);
        assert_ne!(
            ElisionRegion::mint(&universe, d, span(10, 20)),
            ElisionRegion::mint(&universe, d, span(30, 40))
        );
    }

    /// The iteration axis speaks `SiteId`'s numbering. Pinned rather than assumed because the
    /// propagation lane's whole promise is that closing a loop population re-keys nothing: a member
    /// route and the `site N.M` record measuring it must already agree on what "member 2" means.
    #[test]
    fn the_iteration_slot_speaks_the_site_records_member_numbering() {
        assert_eq!(
            IterationSlot::NotIterated.member(),
            SiteId::leaf(crate::LeafId(0)).member
        );
        assert_eq!(IterationSlot::Member(2).member(), Some(2));
    }
}
