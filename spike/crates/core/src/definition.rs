//! `core::definition` — WHICH definition a derived row came from, and which one answers at a site
//! (`28Q` §1 `syn-definition-factored-indices`).
//!
//! Every derived row — a check, a cell declaration, an argparse arm-model, an enrolled dialect
//! token, a footprint claim — is produced by exactly one function definition, and the conversion
//! keys each row by that definition rather than by the position of the file that happened to win a
//! whole-unit scan. [`DefinitionId`] is that key; [`DefinitionProvenance`] is what a row carries;
//! [`LiveDefinition`] is what the function environment answers at a site; and
//! [`answering_file`] is the ONE place the three meet.
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
//! That applies to this module too — a change to [`answering_file`]'s rule is a licensure change
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

/// What a derived row knows about the definition that produced it.
///
/// Three states rather than an `Option`, because "no definition to point at" and "more than one"
/// are opposite answers and collapsing them would make one of them wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionProvenance {
    /// NO SOURCE TEXT exists for this row to have come from — a hand-built index, which the kernel
    /// unit tests and the instrument lanes construct from no source at all. The environment holds
    /// no opinion about such a row, and manufacturing one would wall every hand-built index in the
    /// workspace (`28P:dec-the-gate-applies-only-to-names-the-unit-knows`).
    ///
    /// This is also where the one genuine parser disagreement lands: a source name that is not a
    /// legal sh NAME lifts a row under its MUNGED funcname while `dorc_syntax` records the authored
    /// one, so the join finds nothing. That population is marked at Error severity by
    /// `oracle::reserved` and is byte-identically un-gated today, which is why it maps HERE rather
    /// than to [`Ambiguous`](Self::Ambiguous).
    Unkeyed,
    /// This row came from exactly this definition.
    Keyed(DefinitionId),
    /// The file holds MORE THAN ONE definition of this role and the lift kept one, so which
    /// definition spoke is unrecoverable. Answers nowhere: a row that cannot name its own author
    /// must not be read at any frame (`inv-top-reject`).
    Ambiguous,
}

/// What the function environment says is live for one name at one site.
///
/// Produced by `dorc_analysis::funcenv::LiveDefinitions`; named here because
/// [`answering_file`] must see it and `core` is the only crate both sides can reach.
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

/// **THE resolution seat** (`28Q` §1.3): which source file's row answers at this site.
///
/// It replaces the whole-unit last-definition-wins scan plus its positional agreement gate with one
/// question — *which definition is live here, and which row did that definition produce* — because
/// the two mechanisms were two readings of one environment and could disagree
/// (`28P:fnd-build-vouches-relifted-the-verdict-sets` is what that disagreement cost the last time
/// it was spelled twice). Every seat that used to scan calls this instead.
///
/// The call shape deliberately mirrors the scan it replaces: a `count` and a per-file lookup, so a
/// seat's own row storage stays its own business and only the RULE is shared.
///
/// Three arms, and each one is a licensure decision:
///
/// - [`Live(def)`](LiveDefinition::Live) — the row [`Keyed`](DefinitionProvenance::Keyed) to
///   exactly that definition, or nothing. This is what makes the chimera unrepresentable rather
///   than merely gated: identity and cells are read from ONE definition's rows, so the read that
///   measured one cell while keying the record to another cannot be spelled
///   (`271:rul-sin-ordering`).
/// - [`Withheld`](LiveDefinition::Withheld) — nothing answers.
/// - [`NoOpinion`](LiveDefinition::NoOpinion) — the sole candidate answers; PLURAL candidates
///   withhold. Without an environment there is no rule that picks between two authors, and
///   inventing one here would be exactly the load-order-as-trust-adjudicator fence `28K` §6 refuses.
///   Byte-identical on a single-definition corpus, which is every corpus that exists.
#[must_use]
pub fn answering_file(
    live: LiveDefinition,
    count: usize,
    provenance_of: impl Fn(usize) -> Option<DefinitionProvenance>,
) -> Option<usize> {
    match live {
        LiveDefinition::Withheld => None,
        LiveDefinition::Live(wanted) => {
            (0..count).find(|&i| provenance_of(i) == Some(DefinitionProvenance::Keyed(wanted)))
        }
        LiveDefinition::NoOpinion => {
            let mut sole = None;
            for i in 0..count {
                match provenance_of(i) {
                    None | Some(DefinitionProvenance::Ambiguous) => {}
                    Some(DefinitionProvenance::Keyed(_) | DefinitionProvenance::Unkeyed) => {
                        if sole.is_some() {
                            return None;
                        }
                        sole = Some(i);
                    }
                }
            }
            sole
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DefinitionId, DefinitionProvenance, LiveDefinition, answering_file};
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
        let rows = [
            Some(DefinitionProvenance::Keyed(def(0, 10))),
            Some(DefinitionProvenance::Keyed(def(1, 10))),
        ];
        let ask = |live| answering_file(live, rows.len(), |i| rows[i]);
        assert_eq!(ask(LiveDefinition::Live(def(1, 10))), Some(1));
        assert_eq!(ask(LiveDefinition::Live(def(0, 10))), Some(0));
    }

    /// A definition the rows do not carry answers nowhere — the honest result when the frame names
    /// a definition that produced no derived row (a body the dialect parser could not lift).
    #[test]
    fn a_live_definition_with_no_row_answers_nowhere() {
        let rows = [Some(DefinitionProvenance::Keyed(def(0, 10)))];
        assert_eq!(
            answering_file(LiveDefinition::Live(def(9, 99)), rows.len(), |i| rows[i]),
            None
        );
    }

    /// Withheld is total: it does not matter what rows exist. `Undefined`, ⊤, and unreached all
    /// arrive here, and all three must license nothing.
    #[test]
    fn withheld_answers_nowhere_whatever_the_rows() {
        let rows = [
            Some(DefinitionProvenance::Keyed(def(0, 10))),
            Some(DefinitionProvenance::Unkeyed),
        ];
        assert_eq!(
            answering_file(LiveDefinition::Withheld, rows.len(), |i| rows[i]),
            None
        );
    }

    /// The instrument/hand-built posture: with no environment, a SOLE row answers. This is the arm
    /// that keeps every source-less index in the workspace usable, and it is why the whole-unit
    /// scan can retire rather than surviving as a fallback.
    #[test]
    fn no_opinion_answers_from_a_sole_row() {
        let rows = [None, Some(DefinitionProvenance::Unkeyed), None];
        assert_eq!(
            answering_file(LiveDefinition::NoOpinion, rows.len(), |i| rows[i]),
            Some(1)
        );
    }

    /// ...and PLURAL rows withhold. Without an environment nothing licenses picking one author
    /// over another; taking the last would be load-order-as-trust-adjudicator (`28K` §6), which is
    /// a permanently rejected fence.
    #[test]
    fn no_opinion_withholds_when_two_rows_compete() {
        let rows = [
            Some(DefinitionProvenance::Keyed(def(0, 10))),
            Some(DefinitionProvenance::Keyed(def(1, 10))),
        ];
        assert_eq!(
            answering_file(LiveDefinition::NoOpinion, rows.len(), |i| rows[i]),
            None
        );
    }

    /// An ambiguous row answers at NO frame, under either arm. A row that cannot name its own
    /// author must never be read: under `Live` it cannot match, and under `NoOpinion` it does not
    /// even count as a candidate — so a file with a within-file redefinition never silently lends
    /// its surviving row to a sole-candidate answer.
    #[test]
    fn an_ambiguous_row_answers_at_no_frame() {
        let rows = [Some(DefinitionProvenance::Ambiguous)];
        let ask = |live| answering_file(live, rows.len(), |i| rows[i]);
        assert_eq!(ask(LiveDefinition::Live(def(0, 10))), None);
        assert_eq!(ask(LiveDefinition::NoOpinion), None);
    }

    /// An empty candidate set answers nowhere under every arm — the degenerate case a seat hits
    /// when no file describes the family at all.
    #[test]
    fn no_candidates_answer_nowhere() {
        let none = |_: usize| None;
        assert_eq!(answering_file(LiveDefinition::NoOpinion, 0, none), None);
        assert_eq!(answering_file(LiveDefinition::Withheld, 0, none), None);
        assert_eq!(
            answering_file(LiveDefinition::Live(def(0, 10)), 0, none),
            None
        );
    }
}
