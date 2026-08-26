//! The reviewed identity table: every component of the two plan-side identities, in write order,
//! with the exact framing each is spelled under (`quarantine/30Rb:receipt-identity-map`).
//!
//! This table GENERATES NOTHING. It is the reviewed statement both encoders are checked against,
//! the way the grammar table is the reviewed statement the writer and the reader agree with. Its
//! value is that it is exhaustive and two-way: a component the encoder writes without a row here
//! fails, and a row naming a component nothing writes fails as a stale entry.
//!
//! # Why framing is the column that matters
//!
//! An identity is a hash of a byte string, so the only property that makes it an identity of
//! anything is INJECTIVITY: two different values must not encode alike. Nothing in the mints'
//! signatures can enforce that — they take bare bytes — so the framing of each component is what
//! carries it, and writing the framing down per component is the whole point of the table.
//!
//! [`PLANNING_INPUT_COMPONENTS`] is length-framed throughout, so no value can reach across into a
//! neighbour. [`PRESENTED_PLAN_COMPONENTS`] is length-framed for the same reason and, since
//! `ruling-frame-the-presented-plan-sections`, in the same idiom: two of its components carry the
//! book verbatim, so a delimiter was whatever a book chose to spell.

/// How one component's bytes are separated from its neighbours'.
///
/// Closed and exhaustive: a new spelling adds an arm here, which is a visible act, rather than
/// being absorbed by a default nobody reviews.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framing {
    /// `<tag> <decimal byte length> <exact bytes>\n`. The length is what makes the value unable to
    /// impersonate a neighbouring component, whatever bytes it carries.
    LengthPrefixed,
    /// [`Self::LengthPrefixed`] when a value is present, `<tag> absent\n` when it is not. Never an
    /// empty value standing in for absence: an empty string and no string are different facts.
    LengthPrefixedOrAbsent,
    /// `<tag> <token>\n`, where the token is a decimal integer or a closed word. It cannot contain
    /// a space or a newline, so it needs no length.
    Scalar,
    /// [`Self::Scalar`] when a value is present, `<tag> absent\n` when it is not.
    ScalarOrAbsent,
    /// `<tag> absent\n`, always: the component has no value to spell at v1.
    ///
    /// Written anyway rather than omitted, so the day it gains one is a visible change of encoding
    /// instead of a silent widening of what the identity covers.
    AlwaysAbsent,
}

/// One component of one identity's canonical encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Component {
    /// The literal tag, as written.
    pub tag: &'static str,
    /// How its bytes are separated from its neighbours'.
    pub framing: Framing,
}

const fn c(tag: &'static str, framing: Framing) -> Component {
    Component { tag, framing }
}

/// Every component of `PlanningInputId`'s canonical encoding, in write order.
///
/// The membership is the human's (`quarantine/30Rc`): authored planning state plus the admitted
/// world state the planner consumed, and never what the planner PRODUCED. This table fixes how
/// each member is spelled, not which members there are — `planning_input::CENSUS` and its two
/// tests own that question, and they are checked against this table in both directions.
///
/// Repeated groups are listed once. The `source-*` rows repeat per acquired source, and the
/// record rows repeat per admitted record; each group is preceded by its own count, which is what
/// makes the repetition recoverable rather than a run of look-alike lines.
pub const PLANNING_INPUT_COMPONENTS: &[Component] = &[
    // Controller scope.
    c("semantics", Framing::LengthPrefixed),
    c("host", Framing::LengthPrefixed),
    c("attempt", Framing::Scalar),
    c("generation", Framing::AlwaysAbsent),
    // Parsed policy that can move analysis or settlement.
    c("policy-risk-faultless-skips", Framing::Scalar),
    c("policy-mode", Framing::LengthPrefixed),
    // The ordered acquired-source table, one group per source.
    c("sources", Framing::Scalar),
    c("source-ordinal", Framing::Scalar),
    c("source-role", Framing::LengthPrefixed),
    c("source-path", Framing::LengthPrefixed),
    c("source-digest", Framing::LengthPrefixed),
    c("source-bytes", Framing::Scalar),
    // The admitted world state.
    c("admission", Framing::LengthPrefixedOrAbsent),
    c("records", Framing::ScalarOrAbsent),
    c("record-kind", Framing::LengthPrefixed),
    // ... a site fact and its inert operands.
    c("site-key", Framing::LengthPrefixed),
    c("site-effect", Framing::LengthPrefixed),
    c("site-rc", Framing::Scalar),
    c("site-stdout", Framing::LengthPrefixedOrAbsent),
    c("site-stderr", Framing::LengthPrefixedOrAbsent),
    c("site-inert", Framing::Scalar),
    c("inert-name", Framing::LengthPrefixed),
    c("inert-value", Framing::LengthPrefixed),
    // ... a derivation family.
    c("derivation-site", Framing::Scalar),
    c("derivation-coord", Framing::LengthPrefixed),
    c("derivation-end-site", Framing::Scalar),
    c("derivation-end-count", Framing::Scalar),
    c("derivation-end-body-rc", Framing::Scalar),
    // ... an entity resolution.
    c("resolution-coord", Framing::LengthPrefixed),
    c("resolution-canonical", Framing::LengthPrefixedOrAbsent),
    // ... a reach family.
    c("reach-coord", Framing::LengthPrefixed),
    c("reach-arm", Framing::Scalar),
    c("reach-entity", Framing::LengthPrefixed),
    c("reach-end-coord", Framing::LengthPrefixed),
    c("reach-end-arm", Framing::Scalar),
    c("reach-end-count", Framing::Scalar),
    c("reach-end-body-rc", Framing::Scalar),
    // ... an author's own report line.
    c("report-body", Framing::LengthPrefixed),
];

/// The line opening `PlanningInputId`'s encoding, and the line closing it.
///
/// The opener is what domain-separates the encoding from any other length-framed byte string that
/// might reach the same mint; the terminator is what stops a truncation reading as a shorter but
/// complete value.
pub const PLANNING_INPUT_OPENER: &str = "dorc-planning-inputs/1";

/// See [`PLANNING_INPUT_OPENER`].
pub const PLANNING_INPUT_TERMINATOR: &str = "inputs-end";

/// Every component of `PresentedPlanId`s canonical encoding, in write order.
///
/// LENGTH-FRAMED, exactly as the sibling table above is. Two of these components are the rendered
/// artifacts and the apply render is the BOOK verbatim, so a separator-delimited encoding ended a
/// component wherever a book chose to spell one — measured, and repaired by adopting the declared
/// length. The identity never authorizes, but it IS recompute-and-compare, and a collision there is
/// a confident wrong answer rather than a missing one.
pub const PRESENTED_PLAN_COMPONENTS: &[Component] = &[
    c("plan", Framing::LengthPrefixed),
    c("regions", Framing::LengthPrefixedOrAbsent),
    c("probe", Framing::LengthPrefixed),
    c("render.probe", Framing::LengthPrefixed),
    c("render.apply", Framing::LengthPrefixed),
    c("diags", Framing::LengthPrefixed),
];

/// The line opening `PresentedPlanId`s encoding, and the line closing it.
pub const PRESENTED_PLAN_OPENER: &str = "dorc-presented-plan/1";

/// See [`PRESENTED_PLAN_OPENER`].
pub const PRESENTED_PLAN_TERMINATOR: &str = "decision-end";

/// The one component spelled `absent` rather than emitted empty.
///
/// The distinction is load-bearing: a book with no eligible shared calls is a different fact from
/// one whose regions all vanished, and an empty value standing in for absence would merge them
/// (`30L:pin-empty-function-world-parity`). Every other component is always written.
pub const PRESENTED_PLAN_ABSENT_WHEN_EMPTY: &str = "regions";

#[cfg(test)]
mod tests {
    use super::*;

    /// The encoder's own source. Read lexically, because the property is "every tag this module
    /// writes has a row", and no type expresses that — the tags are string literals handed to
    /// four framing helpers.
    const PLANNING_INPUT_SOURCE: &str = include_str!("planning_input.rs");

    /// The presented-plan encoder. Since `ruling-frame-the-presented-plan-sections` it writes in
    /// the SAME idiom, so one reader serves both files rather than two readers drifting apart.
    const ERASABILITY_SOURCE: &str = include_str!("erasability.rs");

    /// Every `put_*(…, "<tag>", …)` the encoder writes, paired with the helper it went through —
    /// which is what fixes the tag's framing.
    fn written_tags(source: &str) -> Vec<(String, Framing)> {
        let mut out: Vec<(String, Framing)> = Vec::new();
        for (helper, framing) in [
            ("put_str", Framing::LengthPrefixed),
            ("put_opt_str", Framing::LengthPrefixedOrAbsent),
            ("put_absent", Framing::AlwaysAbsent),
            ("put_u64", Framing::Scalar),
            ("put_i64", Framing::Scalar),
            ("put_count", Framing::Scalar),
            ("put_bool", Framing::Scalar),
        ] {
            let needle = format!("{helper}(");
            for piece in source.split(&needle).skip(1) {
                // A CALL, never the helper's own body: the sink argument comes first, and
                // requiring it is what stops a literal inside `put_bool` reading as a tag.
                // Whitespace-insensitive: rustfmt breaks a long call across lines, and a reader
                // that only understood the one-line form would silently miss exactly the tags
                // with the longest names.
                let flat: String = piece.chars().filter(|ch| !ch.is_whitespace()).collect();
                let after_sink = flat
                    .strip_prefix("&mutout,")
                    .or_else(|| flat.strip_prefix("out,"))
                    .map(str::to_owned);
                let Some(after_sink) = after_sink else {
                    continue;
                };
                let Some(rest) = after_sink.strip_prefix('"') else {
                    continue;
                };
                let Some(close) = rest.find('"') else {
                    continue;
                };
                let Some(tag) = rest.get(..close) else {
                    continue;
                };
                out.push((tag.to_owned(), framing));
            }
        }
        out
    }

    /// The framing a tag ends up with when it is written through more than one helper.
    ///
    /// Exactly two combinations occur and both are an optional value: a present arm plus the
    /// absent word. Anything else is two different framings for one tag, which is a tag that
    /// cannot be read back, so it is refused rather than merged.
    fn combine(tag: &str, seen: &[Framing]) -> Framing {
        let mut distinct: Vec<Framing> = seen.to_vec();
        distinct.sort_by_key(|framing| format!("{framing:?}"));
        distinct.dedup();
        // The sort above fixes the order, so the absent half is always first: `AlwaysAbsent`
        // sorts ahead of every other name. That is what lets the pair arms be two rows rather
        // than four.
        match distinct.as_slice() {
            [one] => *one,
            [Framing::AlwaysAbsent, Framing::LengthPrefixed] => Framing::LengthPrefixedOrAbsent,
            [Framing::AlwaysAbsent, Framing::Scalar] => Framing::ScalarOrAbsent,
            _ => panic!("{tag} is written under framings that are not one optional value"),
        }
    }

    /// Check one table against the encoder that writes it, in BOTH directions.
    ///
    /// One helper over both identities, because since the framing repair the two encoders write
    /// in one idiom — two readers would be two places for the same rule to drift.
    fn agree(source: &str, table: &[Component], floor: usize, what: &str) {
        let written = written_tags(source);
        assert!(
            written.len() >= floor,
            "{what}: the lexical read found only {} tags; it is looking for the wrong shape",
            written.len()
        );

        let mut failures: Vec<String> = Vec::new();
        let mut tags: Vec<String> = written.iter().map(|(tag, _)| tag.clone()).collect();
        tags.sort_unstable();
        tags.dedup();

        for tag in &tags {
            let seen: Vec<Framing> = written
                .iter()
                .filter(|(name, _)| name == tag)
                .map(|(_, framing)| *framing)
                .collect();
            let framing = combine(tag, &seen);
            match table.iter().find(|row| row.tag == tag) {
                None => failures.push(format!("{tag}: written, and the table has no row for it")),
                Some(row) if row.framing != framing => failures.push(format!(
                    "{tag}: the table says {:?}, the encoder writes {framing:?}",
                    row.framing
                )),
                Some(_) => {}
            }
        }
        for row in table {
            if !tags.iter().any(|tag| tag == row.tag) {
                failures.push(format!("{}: in the table, written nowhere", row.tag));
            }
        }
        assert!(failures.is_empty(), "{what}: {failures:#?}");
    }

    #[test]
    fn the_table_and_the_planner_input_encoder_name_the_same_components() {
        // Two-way. A component written without a row is absent from a table that claims to be
        // exhaustive; a row nothing writes is a stale entry describing bytes that do not exist.
        agree(
            PLANNING_INPUT_SOURCE,
            PLANNING_INPUT_COMPONENTS,
            20,
            "planning-input",
        );
    }

    #[test]
    fn the_table_and_the_presented_plan_encoder_name_the_same_components() {
        // The same check over the same idiom, which is the point of the repair: the presented-plan
        // encoding is no longer a second kind of thing that has to be read a second way.
        agree(
            ERASABILITY_SOURCE,
            PRESENTED_PLAN_COMPONENTS,
            6,
            "presented-plan",
        );
    }

    #[test]
    fn both_encodings_are_opened_and_terminated() {
        // A truncation has to be a different value rather than a shorter complete one, and the
        // rename moves both together or fails to compile. A `contains` over the source text would
        // have been satisfied by a comment.
        assert_eq!(PLANNING_INPUT_OPENER, crate::planning_input::ENCODING);
        assert_eq!(PLANNING_INPUT_TERMINATOR, crate::planning_input::TERMINATOR);
        assert_eq!(PRESENTED_PLAN_OPENER, crate::erasability::ENCODING);
        assert_eq!(PRESENTED_PLAN_TERMINATOR, crate::erasability::TERMINATOR);
        assert_ne!(
            PLANNING_INPUT_OPENER, PRESENTED_PLAN_OPENER,
            "one opener for two encodings would be one encoding"
        );
    }

    #[test]
    fn only_the_regions_component_is_spelled_absent_when_empty() {
        // The one conditional component, and the reason it is conditional. Every other one is
        // written unconditionally, so a reader walking the declared lengths knows what it has.
        let absent_row = PRESENTED_PLAN_COMPONENTS
            .iter()
            .find(|row| row.tag == PRESENTED_PLAN_ABSENT_WHEN_EMPTY);
        assert_eq!(
            absent_row.map(|row| row.framing),
            Some(Framing::LengthPrefixedOrAbsent)
        );
        for row in PRESENTED_PLAN_COMPONENTS {
            if row.tag != PRESENTED_PLAN_ABSENT_WHEN_EMPTY {
                assert_eq!(
                    row.framing,
                    Framing::LengthPrefixed,
                    "{} is conditional and only one component may be",
                    row.tag
                );
            }
        }
    }
}
