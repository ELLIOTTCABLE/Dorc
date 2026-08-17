//! `core::definition` — WHICH definition a derived row came from, and which one answers at a site
//! (`28Q` §1 `syn-definition-factored-indices`).
//!
//! Every derived row — a check, a cell declaration, an argparse arm-model, an enrolled dialect
//! token, a footprint claim — is produced by exactly one function definition, and it is keyed by
//! that definition: [`DefinitionId`] is the key, a row carries its own, [`LiveDefinition`] is what
//! the function environment answers at a site, and [`answering_row`] is where the two meet.
//!
//! # What a row's id is, and why nothing joins to get it
//!
//! A row's id is minted from the definition the DIALECT lift read: its file, and the funcdef span
//! that lift recorded. The environment's ids come from `dorc_syntax`'s parse of the same file. Two
//! parsers, one identity — so the two agree by MEASUREMENT, not by construction, and
//! `every_lifted_role_row_carries_its_parsed_definitions_span` is the gate that keeps it true. That
//! gate is load-bearing rather than tidy: a drifted span matches no frame answer, so every site
//! would withhold silently and corpus-wide, in the direction the byte-identity gate is weakest at
//! catching.
//!
//! The retired shape asked the definition table for a row's identity, joined on `(file, role name)`
//! (`28Q` §1.1's named INCORRECT INTERIM). Two states existed only to describe that join's failures
//! — a row the table could not find, and a file holding two definitions of one role so that which
//! one spoke was unrecoverable — and both are gone with it. Spans are unique within a file, so
//! "which of this file's definitions produced this row" is no longer a question anyone can fail to
//! answer.
//!
//! # Why this lives in `core`
//!
//! The rows live in `dorc-oracle` and the frame answer is computed in `dorc-analysis`, which
//! depends on `oracle` and not the reverse. A rule that reads both therefore has exactly one
//! possible home, and it is this one — which is also the right one on the merits: this is decide-
//! plane vocabulary every crate must agree on before any of them builds (dac-B).
//!
//! # The winner-shifting rider (`28Q` §1, permanent)
//!
//! Under true resolution every function-environment precision bug is WINNER-SHIFTING: it selects
//! whose judgment governs a site, with no agreement veto standing behind it. The whole frame solver
//! is therefore license-review-tier forever, and precision work on it is never ordinary value-add.
//! That applies to this module too — a change to [`answering_row`]'s rule is a licensure change
//! wearing a lookup's clothes.

use crate::{DefinitionCustody, SourceFileId, Span};

/// The identity of one function definition in the analysis unit: the file that spells it, and the
/// byte range it occupies there.
///
/// Two definitions of one name in one file are DISTINCT ids, which is the whole reason the span
/// rides along — the file alone cannot tell them apart, and the environment can bind either.
///
/// **Custody is DERIVED, never stored** ([`custody`](Self::custody)). Custody is keyed to the
/// defining file today, and `28M` §10 `dir-ownership-is-transitive-inclusion` (UNRULED) may re-key
/// it to an entry file's transitive sourcing-closure; deriving it keeps that re-key a change to one
/// method body, where storing it would mint a second field that could disagree with the first.
/// `core/CLAUDE.md` `custody-is-one-newtype-and-one-crossing` is the standing rule this honours.
///
/// [`file`](Self::file) and [`span`](Self::span) are PROVENANCE AND DISPLAY only — resolving a
/// definition's text for an emission, framing a diagnostic's caret. Branching on the raw file id to
/// decide anything re-creates the untyped keying this type exists to retire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinitionId {
    file: SourceFileId,
    span: Span,
}

impl DefinitionId {
    /// The ONE mint: a definition is identified by where it is written.
    #[must_use]
    pub const fn at(file: SourceFileId, span: Span) -> Self {
        Self { file, span }
    }

    /// The file that spells this definition — provenance and display only (see the type doc).
    #[must_use]
    pub const fn file(self) -> SourceFileId {
        self.file
    }

    /// The definition's byte range in its file — provenance and display only.
    #[must_use]
    pub const fn span(self) -> Span {
        self.span
    }

    /// Whose utterance this definition is (`28M` §8). The one crossing from a definition into the
    /// custody vocabulary, so a re-key has one seat to inspect.
    #[must_use]
    pub const fn custody(self) -> DefinitionCustody {
        DefinitionCustody::of_defining_file(self.file)
    }
}

/// What the function environment says is live for one name at one site.
///
/// Produced by `dorc_analysis::funcenv::LiveDefinitions`; named here because
/// [`answering_row`] must see it and `core` is the only crate both sides can reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveDefinition {
    /// The definition a shell executing the unit top-to-bottom would have live at this site.
    Live(DefinitionId),
    /// Provably nothing is live here (the name is `Undefined`), or the environment cannot say (⊤),
    /// or the point is unreached (⊥). All three withhold, and they withhold for the same reason:
    /// no definition can be named, so no row may answer (`silence-licenses-nothing`).
    Withheld,
    /// No environment was solved, or the unit's definition table has never heard of this name.
    /// The row answers on its own provenance alone — the instrument, hint, and hand-built-index
    /// posture (`28P:res-instrument-lanes-stay-ambient`).
    NoOpinion,
}

/// **THE resolution seat** (`28Q` §1.3): which of a seat's candidate rows answers at this site.
///
/// It replaces the whole-unit last-definition-wins scan plus its positional agreement gate with one
/// question — *which definition is live here, and which row did that definition produce* — because
/// the two mechanisms were two readings of one environment and could disagree
/// (`28P:fnd-build-vouches-relifted-the-verdict-sets` is what that disagreement cost the last time
/// it was spelled twice). Every seat that used to scan calls this instead.
///
/// The call shape deliberately mirrors the scan it replaces: a `count` and a per-candidate lookup,
/// so a seat's own row storage stays its own business and only the RULE is shared. `definition_of`
/// answers `None` for a slot holding no row for this question, and otherwise the id of the
/// definition that produced the row — which the row CARRIES; asking a table for it was the retired
/// join (`28Q` §1.1).
///
/// Three arms, and each one is a licensure decision:
///
/// - [`Live(def)`](LiveDefinition::Live) — the row that definition produced, or nothing. This is
///   what makes the chimera unrepresentable rather than merely gated: identity and cells are read
///   from ONE definition's rows, so the read that measured one cell while keying the record to
///   another cannot be spelled (`271:rul-sin-ordering`).
/// - [`Withheld`](LiveDefinition::Withheld) — nothing answers.
/// - [`NoOpinion`](LiveDefinition::NoOpinion) — the sole candidate answers; PLURAL candidates
///   withhold. Without an environment there is no rule that picks between two authors, and
///   inventing one here would be exactly the load-order-as-trust-adjudicator fence `28K` §6 refuses.
///   Byte-identical on a single-definition corpus, which is every corpus that exists.
#[must_use]
pub fn answering_row(
    live: LiveDefinition,
    count: usize,
    definition_of: impl Fn(usize) -> Option<DefinitionId>,
) -> Option<usize> {
    match live {
        LiveDefinition::Withheld => None,
        LiveDefinition::Live(wanted) => (0..count).find(|&i| definition_of(i) == Some(wanted)),
        LiveDefinition::NoOpinion => {
            let mut sole = None;
            for i in 0..count {
                if definition_of(i).is_none() {
                    continue;
                }
                if sole.is_some() {
                    return None;
                }
                sole = Some(i);
            }
            sole
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DefinitionId, LiveDefinition, answering_row};
    use crate::{BytePos, SourceFileId, Span};

    /// A definition at file `file`, spanning `[lo, hi)` — distinct `lo`s are distinct definitions.
    fn def(file: u32, lo: u32) -> DefinitionId {
        DefinitionId::at(SourceFileId(file), Span::new(BytePos(lo), BytePos(lo)))
    }

    /// Two definitions in ONE file are distinct ids. This is the whole reason the span is part of
    /// the identity: keying on the file alone would silently merge the within-file redefinition
    /// pair, and the environment can bind either of them.
    #[test]
    fn one_file_can_hold_two_distinct_definitions() {
        assert_ne!(def(0, 10), def(0, 40));
        assert_eq!(def(0, 10), def(0, 10));
    }

    /// Custody derives from the defining file and nothing else, so two definitions in one file
    /// share custody while remaining distinct definitions. Pinned because the derivation is what
    /// keeps `28M` §10's possible re-key confined to one method body.
    #[test]
    fn custody_derives_from_the_file_not_the_span() {
        assert_eq!(def(0, 10).custody(), def(0, 40).custody());
        assert_ne!(def(0, 10).custody(), def(1, 10).custody());
    }

    /// The chimera fence, stated positively: a live definition selects ITS OWN row, never a
    /// neighbour's. Under the retired agreement gate this was a checked coincidence; here the
    /// wrong answer is unrepresentable, because the lookup is BY the definition.
    #[test]
    fn a_live_definition_selects_only_its_own_row() {
        let rows = [Some(def(0, 10)), Some(def(1, 10))];
        let ask = |live| answering_row(live, rows.len(), |i| rows[i]);
        assert_eq!(ask(LiveDefinition::Live(def(1, 10))), Some(1));
        assert_eq!(ask(LiveDefinition::Live(def(0, 10))), Some(0));
    }

    /// Two rows from ONE file are told apart by their spans — the property the retired `(file,
    /// role name)` join could not express, and the reason it needed an "ambiguous" state at all.
    #[test]
    fn two_rows_from_one_file_resolve_independently() {
        let rows = [Some(def(0, 10)), Some(def(0, 40))];
        let ask = |live| answering_row(live, rows.len(), |i| rows[i]);
        assert_eq!(ask(LiveDefinition::Live(def(0, 40))), Some(1));
        assert_eq!(ask(LiveDefinition::Live(def(0, 10))), Some(0));
    }

    /// A definition the rows do not carry answers nowhere — the honest result when the frame names
    /// a definition that produced no derived row (a body the dialect parser could not lift).
    #[test]
    fn a_live_definition_with_no_row_answers_nowhere() {
        let rows = [Some(def(0, 10))];
        assert_eq!(
            answering_row(LiveDefinition::Live(def(9, 99)), rows.len(), |i| rows[i]),
            None
        );
    }

    /// Withheld is total: it does not matter what rows exist. `Undefined`, ⊤, and unreached all
    /// arrive here, and all three must license nothing.
    #[test]
    fn withheld_answers_nowhere_whatever_the_rows() {
        let rows = [Some(def(0, 10)), Some(def(1, 10))];
        assert_eq!(
            answering_row(LiveDefinition::Withheld, rows.len(), |i| rows[i]),
            None
        );
    }

    /// The instrument/hand-built posture: with no environment, a SOLE row answers. This is the arm
    /// that keeps every source-less index in the workspace usable, and it is why the whole-unit
    /// scan can retire rather than surviving as a fallback.
    #[test]
    fn no_opinion_answers_from_a_sole_row() {
        let rows = [None, Some(def(1, 10)), None];
        assert_eq!(
            answering_row(LiveDefinition::NoOpinion, rows.len(), |i| rows[i]),
            Some(1)
        );
    }

    /// ...and PLURAL rows withhold. Without an environment nothing licenses picking one author
    /// over another; taking the last would be load-order-as-trust-adjudicator (`28K` §6), which is
    /// a permanently rejected fence.
    #[test]
    fn no_opinion_withholds_when_two_rows_compete() {
        let rows = [Some(def(0, 10)), Some(def(1, 10))];
        assert_eq!(
            answering_row(LiveDefinition::NoOpinion, rows.len(), |i| rows[i]),
            None
        );
    }

    /// An empty candidate set answers nowhere under every arm — the degenerate case a seat hits
    /// when no file describes the family at all.
    #[test]
    fn no_candidates_answer_nowhere() {
        let none = |_: usize| None;
        assert_eq!(answering_row(LiveDefinition::NoOpinion, 0, none), None);
        assert_eq!(answering_row(LiveDefinition::Withheld, 0, none), None);
        assert_eq!(
            answering_row(LiveDefinition::Live(def(0, 10)), 0, none),
            None
        );
    }
}
