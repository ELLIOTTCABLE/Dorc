//! The emission planner's decision procedure: where the artifact may place a definition, and under
//! what name (`plans/30P:the-emission-planner`; `28Q:pin-emission-planner-universal`).
//!
//! # A decision procedure, not a pass
//!
//! Two consumers ask this question and they cannot run at one moment. `cli::artifact::Selection`
//! settles BEFORE the plan exists, because an [`crate::ImportEdit`] is an input to
//! `Plan::decided`; `pin_definitions` runs INSIDE it, over the settled dispositions. So nothing
//! here schedules anything: it answers *may bytes binding name-set N stand at position P, and
//! under what name*, from authored-before-contact inputs alone, and each consumer asks at its own
//! moment with its own candidate set. `the-render-decides-nothing` is preserved because the
//! answers are recorded on the plane at `Plan::decided` like every other render answer.
//!
//! # Why a definition inherits its SOURCE's placement
//!
//! `30Qb:rul-a-loaded-definitions-placement-is-its-load-position` — a definition cannot stand
//! anywhere its own file's bytes do not. A `--pre-source` root is AMBIENT
//! (`cli/CLAUDE.md only-invocation-roots-are-ambient`): the analysis already models its bindings as
//! live before the book's first line, so hoisting it is faithful and needs no predicate. A source a
//! book `.` reaches binds AT that `.`, and since the bundling
//! (`30Ng:rul-bundle-at-dorc-lang-boundaries`) the artifact already carries its bytes there — so
//! the correct emission is to stop hoisting a second copy, not to add machinery. A hoist of such a
//! source is then the ladder-gated OPTIMISATION the front-lift tiers describe, never the default.
//!
//! `placement` is the reserved word being spent as reserved (`30P:rul-emission-is-the-umbrella-name`):
//! `emission` stays the umbrella, `layout` stays weft's textual-emission word, and `lift` is never a
//! placement word here — "the lift" is the static lift of oracle text into the engine.

use std::collections::BTreeMap;

use dorc_core::{AstId, SourceFileId};

/// The authored `.` whose position a loaded source's bytes stand at.
///
/// Newtyped against [`GuardSite`] because an `AstId` carries no role: a swap type-checks, and a
/// wrong id here RELOCATES a definition — the pope-sin neighbourhood (`271:rul-sin-ordering`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoadSite(pub AstId);

/// The guarded site whose `( … )` subshell would host a sunk definition.
///
/// The other half of [`LoadSite`]'s pair, for the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GuardSite(pub AstId);

/// Where the artifact stands a definition (`30P:the-emission-planner`'s placement axis).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Placement {
    /// Ahead of the whole book, in the lifted section. Faithful without a predicate only for
    /// AMBIENT material; for anything a book `.` binds it is the ladder-gated optimisation.
    Hoist,
    /// Where the author's own `.` put it — the artifact carries the source's bytes at that line.
    InPlace(LoadSite),
    /// Inside the one guard subshell that uses it, where it dies at the `)` and so has no namespace
    /// footprint at all (`28Q:pin-emission-planner-universal`'s death-at-the-paren truth).
    Sink(GuardSite),
}

/// What name the artifact binds a definition under (`30P:the-emission-planner`'s naming axis).
///
/// The munge is HEADER-ONLY by construction (`28R:rul-munge-oracle-names-only`): a body owns only
/// its own bytes, so renaming its header rewrites no reference inside anybody's authored text.
/// Renaming CALLS is alpha-rename and stays reserved (`d-alpha-rename-equivalence`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmittedName {
    /// The name its author gave it.
    Authored,
    /// A hash-disambiguated emission, because something else in the unit already claims the
    /// authored name.
    Munged {
        /// The name the author wrote — what a reader needs to map the emission back.
        authored: String,
        /// The name the artifact binds.
        emitted: String,
    },
}

impl EmittedName {
    /// The name the artifact actually binds, given the authored one.
    #[must_use]
    pub fn binds<'a>(&'a self, authored: &'a str) -> &'a str {
        match self {
            Self::Authored => authored,
            Self::Munged { emitted, .. } => emitted,
        }
    }
}

/// Which condition of the front-lift ladder decided this placement (`30Ng` §7;
/// `30P:rul-front-lift-is-the-planners-first-consumer`).
///
/// A reason ENUM beside the decision, never sibling diagnostic codes
/// (`28L:rul-reason-enums-not-sibling-codes`), and minted in the same act as the placement it
/// explains (`acts-and-dispositions-mint-together`) so no seat re-derives it from the outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementReason {
    /// T1: pure code motion — nothing the bundle binds is observed or mutated above the `.`, and no
    /// dynamism opener stands earlier in the unit.
    HoistedAsIs,
    /// T2a: the bundle hoists under a munged name because the book claims the authored one, and
    /// every reference to it is engine-emitted.
    HoistedMunged {
        /// The colliding authored name.
        collided: String,
    },
    /// T2b: a colliding name is a helper or a file-level constant, whose references live inside
    /// authored bodies — rewriting those is alpha-rename, which is reserved, so the bundle stays
    /// where the author put it. The name rides along so the disclosure can name what to rename.
    KeptInPlaceNameCollides {
        /// The colliding authored name.
        name: String,
    },
    /// T3: the unit carries book dynamism the engine allows but cannot enumerate through, so no
    /// hoist is licensed (`rul-happy-path-is-a-closed-set`).
    KeptInPlaceDynamismOpener,
    /// T3: the `.` itself sits outside the shape `floor30-inline-dot-boundary` measured, so the
    /// bytes stay exactly where they are.
    KeptInPlaceShapeUnmeasured,
}

/// One placement question, answered: where the bytes stand, under what name, and which ladder
/// condition decided it.
///
/// A plain struct on purpose — `lane-influence-carriage` converts every stable semantic object to
/// carry an influence account, and minting this one plain now is what lets that lane do it once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementDecision {
    placement: Placement,
    naming: EmittedName,
    why: PlacementReason,
}

impl PlacementDecision {
    /// Mint a decision. The reason is REQUIRED rather than attachable, because it is the other half
    /// of the conclusion rather than a reading of it (`pin-no-outcome-as-generator`).
    #[must_use]
    pub const fn new(placement: Placement, naming: EmittedName, why: PlacementReason) -> Self {
        Self {
            placement,
            naming,
            why,
        }
    }

    /// Where the bytes stand.
    #[must_use]
    pub const fn placement(&self) -> &Placement {
        &self.placement
    }

    /// Under what name.
    #[must_use]
    pub const fn naming(&self) -> &EmittedName {
        &self.naming
    }

    /// Which ladder condition decided it.
    #[must_use]
    pub const fn why(&self) -> &PlacementReason {
        &self.why
    }
}

/// What the artifact does with one source's bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourcePlacement<'a> {
    /// No book `.` reaches this source, so it loads before the book's first line (a `--pre-source`
    /// root, or a file that root's own load program sources) or it IS the book. Hoisting is
    /// faithful and needs no predicate.
    Ambient,
    /// A book `.` reaches it and the artifact carries its bytes; this is the decision it took.
    Carried(&'a PlacementDecision),
    /// A book `.` reaches it and the artifact carries NO copy of it — the form could neither absorb
    /// it nor place it beside the plan. There is no position for its definitions to stand at, so
    /// nothing may be derived from them: no hoist, no guard, no elide, the site runs
    /// (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`, the residual).
    Uncarried,
}

/// Where the artifact places every source a book `.` reaches.
///
/// DEMANDED rather than defaulted at the seats that consume it, on
/// `30N:rul-census-inputs-are-non-optional`'s reasoning: a source the map does not mention reads as
/// ambient, and a driver that forgot to record one would hoist bytes the book binds positionally.
/// The only silence it admits is the projection's own answer about which sources a book `.` reaches
/// at all.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlacedSources {
    reached: BTreeMap<SourceFileId, Option<PlacementDecision>>,
}

impl PlacedSources {
    /// No book `.` reaches anything: every source loads before the book's first line, so hoisting
    /// is faithful everywhere. The shape a book with no `.` takes, and the DST/test posture.
    #[must_use]
    pub fn all_ambient() -> Self {
        Self::default()
    }

    /// Record where the artifact places one book-reached source.
    ///
    /// The FIRST decision for a source wins: two textual `.`s of one file bind it twice and the
    /// artifact carries it at both, so what matters is that it is carried at all, and the earliest
    /// position is the one a disclosure should name.
    pub fn carried(&mut self, source: SourceFileId, decision: PlacementDecision) {
        self.reached.entry(source).or_insert(Some(decision));
    }

    /// Record that a book `.` reaches this source and the artifact carries no copy of it.
    pub fn uncarried(&mut self, source: SourceFileId) {
        self.reached.entry(source).or_insert(None);
    }

    /// What the artifact does with this source's bytes.
    #[must_use]
    pub fn of(&self, source: SourceFileId) -> SourcePlacement<'_> {
        match self.reached.get(&source) {
            None => SourcePlacement::Ambient,
            Some(None) => SourcePlacement::Uncarried,
            Some(Some(decision)) => SourcePlacement::Carried(decision),
        }
    }

    /// Where a definition authored in `source` stands, or `None` where nothing carries it.
    ///
    /// The one seat that turns a SOURCE's placement into a DEFINITION's, so the inheritance rule
    /// lives in one place rather than at each consumer.
    #[must_use]
    pub fn of_definition(&self, source: Option<SourceFileId>) -> Option<Placement> {
        match source.map_or(SourcePlacement::Ambient, |file| self.of(file)) {
            SourcePlacement::Ambient => Some(Placement::Hoist),
            SourcePlacement::Carried(decision) => Some(decision.placement().clone()),
            SourcePlacement::Uncarried => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EmittedName, PlacedSources, Placement, PlacementDecision, PlacementReason, SourcePlacement,
    };
    use dorc_core::SourceFileId;

    #[test]
    fn a_source_no_book_dot_reaches_is_ambient_and_hoists() {
        let placed = PlacedSources::all_ambient();
        assert_eq!(placed.of(SourceFileId(3)), SourcePlacement::Ambient);
        assert_eq!(
            placed.of_definition(Some(SourceFileId(3))),
            Some(Placement::Hoist)
        );
    }

    #[test]
    fn a_definition_with_no_known_source_hoists_as_it_always_did() {
        // The DST/test vouch constructors thread no defining span, and the emission they drive has
        // no bundle to stand in — so the absent answer must stay the pre-repair one.
        let placed = PlacedSources::all_ambient();
        assert_eq!(placed.of_definition(None), Some(Placement::Hoist));
    }

    #[test]
    fn a_carried_book_reached_source_inherits_the_bundles_placement() {
        let mut placed = PlacedSources::all_ambient();
        let site = super::LoadSite(dorc_core::AstId(7));
        placed.carried(
            SourceFileId(1),
            PlacementDecision::new(
                Placement::InPlace(site),
                EmittedName::Authored,
                PlacementReason::KeptInPlaceShapeUnmeasured,
            ),
        );
        assert_eq!(
            placed.of_definition(Some(SourceFileId(1))),
            Some(Placement::InPlace(site))
        );
    }

    #[test]
    fn an_uncarried_book_reached_source_places_nothing() {
        let mut placed = PlacedSources::all_ambient();
        placed.uncarried(SourceFileId(2));
        assert_eq!(placed.of(SourceFileId(2)), SourcePlacement::Uncarried);
        assert_eq!(placed.of_definition(Some(SourceFileId(2))), None);
    }

    #[test]
    fn the_first_placement_of_a_twice_loaded_source_wins() {
        let mut placed = PlacedSources::all_ambient();
        let first = PlacementDecision::new(
            Placement::InPlace(super::LoadSite(dorc_core::AstId(4))),
            EmittedName::Authored,
            PlacementReason::KeptInPlaceShapeUnmeasured,
        );
        let second = PlacementDecision::new(
            Placement::InPlace(super::LoadSite(dorc_core::AstId(9))),
            EmittedName::Authored,
            PlacementReason::KeptInPlaceShapeUnmeasured,
        );
        placed.carried(SourceFileId(1), first.clone());
        placed.carried(SourceFileId(1), second);
        placed.uncarried(SourceFileId(1));
        assert_eq!(placed.of(SourceFileId(1)), SourcePlacement::Carried(&first));
    }
}
