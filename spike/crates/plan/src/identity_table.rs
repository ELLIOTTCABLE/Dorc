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
//! neighbour. [`PRESENTED_PLAN_SECTIONS`] is section-delimited, and two of its sections carry bytes
//! that may spell the delimiter — see the note on that constant.

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
    /// A `== <name> ==\n` header line, then the component's bytes up to the next header.
    SectionDelimited,
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

/// Every section of `PresentedPlanId`'s canonical encoding, in write order.
///
/// KNOWN, MEASURED, AND NOT REPAIRED HERE: this encoding is section-delimited, and `render.probe`
/// and `render.apply` carry bytes that may contain a line spelling a section header — the apply
/// render is the book VERBATIM, so a book carrying such a line puts a second copy of that header
/// into the canon. The split between a section's content and the next section is therefore not
/// recoverable from the bytes, which is the property length framing gives the sibling table above.
/// Pinned by `p-x-presented-plan-sections-are-framed`; the interim measurement sits beside it.
pub const PRESENTED_PLAN_SECTIONS: &[Component] = &[
    c("== plan ==", Framing::SectionDelimited),
    c("== regions ==", Framing::SectionDelimited),
    c("== probe ==", Framing::SectionDelimited),
    c("== render.probe ==", Framing::SectionDelimited),
    c("== render.apply ==", Framing::SectionDelimited),
    c("== diags ==", Framing::SectionDelimited),
];

/// The one section that is omitted rather than emitted empty.
///
/// Its absence is load-bearing: a book with no eligible shared calls keeps the encoding it had
/// before regions existed, so a rung-0 world stays byte-identical
/// (`30L:pin-empty-function-world-parity`). Every other section is always written, header and all.
pub const PRESENTED_PLAN_OMITTED_WHEN_EMPTY: &str = "== regions ==";

#[cfg(test)]
mod tests {
    use super::*;

    /// The encoder's own source. Read lexically, because the property is "every tag this module
    /// writes has a row", and no type expresses that — the tags are string literals handed to
    /// four framing helpers.
    const PLANNING_INPUT_SOURCE: &str = include_str!("planning_input.rs");

    /// Likewise for the section headers.
    const ERASABILITY_SOURCE: &str = include_str!("erasability.rs");

    /// Every `put_*(…, "<tag>", …)` the encoder writes, paired with the helper it went through —
    /// which is what fixes the tag's framing.
    fn written_tags() -> Vec<(String, Framing)> {
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
            for piece in PLANNING_INPUT_SOURCE.split(&needle).skip(1) {
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

    #[test]
    fn the_table_and_the_planner_input_encoder_name_the_same_components() {
        // Two-way. A component written without a row is absent from a table that claims to be
        // exhaustive; a row nothing writes is a stale entry describing bytes that do not exist.
        let written = written_tags();
        assert!(
            written.len() > 20,
            "the lexical read found only {} tags; it is looking for the wrong shape",
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
            match PLANNING_INPUT_COMPONENTS.iter().find(|row| row.tag == tag) {
                None => failures.push(format!("{tag}: written, and the table has no row for it")),
                Some(row) if row.framing != framing => failures.push(format!(
                    "{tag}: the table says {:?}, the encoder writes {framing:?}",
                    row.framing
                )),
                Some(_) => {}
            }
        }
        for row in PLANNING_INPUT_COMPONENTS {
            if !tags.iter().any(|tag| tag == row.tag) {
                failures.push(format!("{}: in the table, written nowhere", row.tag));
            }
        }
        assert!(failures.is_empty(), "{failures:#?}");
    }

    #[test]
    fn the_planner_input_encoding_is_opened_and_terminated() {
        assert!(PLANNING_INPUT_SOURCE.contains(&format!("\"{PLANNING_INPUT_OPENER}\"")));
        assert!(PLANNING_INPUT_SOURCE.contains(&format!("b\"{PLANNING_INPUT_TERMINATOR}\\n\"")));
    }

    #[test]
    fn the_table_and_the_presented_plan_encoder_name_the_same_sections() {
        // Two-way over the section headers, on the same reasoning.
        let mut failures: Vec<String> = Vec::new();
        for row in PRESENTED_PLAN_SECTIONS {
            if !ERASABILITY_SOURCE.contains(&format!("{}\\n\"", row.tag)) {
                failures.push(format!("{}: in the table, written nowhere", row.tag));
            }
        }
        // Every emitted literal carrying a header, however it is spelled — two of the six carry a
        // leading newline, and matching only the bare opener would have missed exactly those.
        let written = ERASABILITY_SOURCE
            .split("out.push_str(\"")
            .skip(1)
            .filter(|piece| piece.starts_with("== ") || piece.starts_with("\\n== "))
            .count();
        assert_eq!(
            written,
            PRESENTED_PLAN_SECTIONS.len(),
            "the encoder writes {written} section headers and the table names {}",
            PRESENTED_PLAN_SECTIONS.len()
        );
        assert!(failures.is_empty(), "{failures:#?}");
    }

    #[test]
    fn only_the_regions_section_is_omitted_when_empty() {
        // The one conditional section, and the reason it is conditional. Every other header is
        // unconditional, so a reader counting headers knows what it is looking at.
        assert!(
            PRESENTED_PLAN_SECTIONS
                .iter()
                .any(|row| row.tag == PRESENTED_PLAN_OMITTED_WHEN_EMPTY)
        );
        assert_eq!(
            ERASABILITY_SOURCE.matches("if !regions.is_empty()").count(),
            1,
            "exactly one section is written under a condition"
        );
    }
}
