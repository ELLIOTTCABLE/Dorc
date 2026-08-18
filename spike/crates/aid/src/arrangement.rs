//! The arrangement registry — the SECOND generated prose table
//! (`289:rul-arrangement-home-is-registry-plus-transcripts`), holding the render-owned CHROME
//! the catalog never covered: help/usage pages, structural connectives, preambles, summary
//! lines. It generalizes the proven catalog pipeline rather than inventing a second one:
//! a generated lock (`arrangement_lock.rs`, the sibling of `catalog_lock.rs`), the same
//! prose-provenance protocol, the same two fixpoint gates, and the same
//! renderer-stamped-span edit surface.
//!
//! # What an entry is
//!
//! An entry is ORDERED WORDS keyed by an arrangement slug plus an OPTIONAL occurrence
//! discriminator. The sequence shape earns itself immediately: a chrome line with interpolated
//! counts stores its fixed runs as the words and lets the seat interleave the computed values
//! ([`arrangement_sentence`]). It is also the room a future chain-link narration needs for
//! per-link connectives and tier-word slots (`289:rider-arrangement-home-anticipates-chains`).
//!
//! What is NOT built: re-splitting. A single-word entry edits back from a transcript exactly as
//! catalog prose does; a multi-word entry refuses, because nothing recovers where an edited line
//! re-divides at its value boundaries.
//!
//! Occurrence resolution is selector-shaped, matching the coordinate vocabulary elsewhere in
//! the project: an occurrence-keyed entry wins for its own occurrence, and the occurrence-less
//! entry serves every occurrence that has none of its own. The emitter contract is therefore
//! ALL-OR-NOTHING per slug: stamp every occurrence of a slug, or stamp none of them.
//!
//! # Prose states, out of band
//!
//! An entry's words are `None` or a [`ProseTier`], exactly as a catalog register is. The tier is a
//! TYPE rather than the catalog's old in-band `sm ` prefix because chrome renders verbatim into
//! product bytes; the trade is that a transcript cannot show it, so the gate reads the type.
//!
//! Nothing here decides anything (`two-plane-aid-law`), and nothing here reads the world:
//! the registry is const data plus a pure render (`aid-is-dst-clean`).

use crate::prose::ProseTier;
use crate::tagged::{RenderPart, RenderParts};

/// One arrangement-registry entry: the key + the machine-facing metadata + the prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrangementEntry {
    /// The stable arrangement slug — the same slug the renderer stamps on its span.
    pub slug: &'static str,
    /// The occurrence this entry answers for, or `None` for the whole-slug entry.
    pub occurrence: Option<usize>,
    /// Where this chrome is rendered (machine-facing metadata; builder-authored).
    pub when_used: &'static str,
    /// Why the entry exists — cites the governing slug(s) (machine-facing metadata).
    pub why: &'static str,
    /// The user-facing prose, or `None` while unwritten.
    pub words: Option<ProseTier<&'static [&'static str]>>,
}

#[path = "arrangement_lock.rs"]
mod arrangement_lock;
pub use arrangement_lock::ARRANGEMENTS;

/// The render seat's view of the arrangement registry — the twin of
/// [`CatalogLookup`](crate::catalog::CatalogLookup), so a render can source chrome from the
/// compiled-in const OR a publish-time mutable mirror through ONE seat.
pub trait ArrangementLookup {
    /// The words stored under EXACTLY this key, or `None` when no such entry exists or it is
    /// unwritten. Implementors do no fallback; [`ArrangementLookup::words`] owns that.
    fn words_exact(&self, slug: &str, occurrence: Option<usize>) -> Option<Vec<&str>>;

    /// The words serving `(slug, occurrence)`: the occurrence's own entry when one exists,
    /// else the whole-slug entry (see the module docs on occurrence resolution).
    fn words(&self, slug: &str, occurrence: Option<usize>) -> Option<Vec<&str>> {
        match occurrence {
            Some(_) => self
                .words_exact(slug, occurrence)
                .or_else(|| self.words_exact(slug, None)),
            None => self.words_exact(slug, None),
        }
    }
}

/// The production [`ArrangementLookup`]: the compiled-in [`ARRANGEMENTS`] const.
#[derive(Debug)]
pub struct ConstArrangements;

/// The one production [`ArrangementLookup`] value.
pub const CONST_ARRANGEMENTS: ConstArrangements = ConstArrangements;

impl ArrangementLookup for ConstArrangements {
    fn words_exact(&self, slug: &str, occurrence: Option<usize>) -> Option<Vec<&str>> {
        ARRANGEMENTS
            .iter()
            .find(|entry| entry.slug == slug && entry.occurrence == occurrence)
            .and_then(|entry| entry.words)
            .map(|tier| tier.text().to_vec())
    }
}

/// An owned entry — the publish-time MUTABLE mirror's element, the twin of
/// [`OwnedEntry`](crate::catalog::OwnedEntry).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OwnedArrangement {
    /// The stable arrangement slug.
    pub slug: String,
    /// The occurrence this entry answers for, or `None` for the whole-slug entry.
    pub occurrence: Option<usize>,
    /// Where this chrome is rendered (machine-facing metadata).
    pub when_used: String,
    /// Why the entry exists (machine-facing metadata).
    pub why: String,
    /// The user-facing prose, or `None` while unwritten.
    pub words: Option<ProseTier<Vec<String>>>,
}

/// The compiled-in registry as an owned, mutable mirror — the starting state publish edits
/// before re-serializing. Carry-forward is by construction.
#[must_use]
pub fn owned_arrangements() -> Vec<OwnedArrangement> {
    ARRANGEMENTS
        .iter()
        .map(|entry| OwnedArrangement {
            slug: entry.slug.to_owned(),
            occurrence: entry.occurrence,
            when_used: entry.when_used.to_owned(),
            why: entry.why.to_owned(),
            words: entry.words.map(|tier| tier.map(owned_words)),
        })
        .collect()
}

fn owned_words(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

impl ArrangementLookup for Vec<OwnedArrangement> {
    fn words_exact(&self, slug: &str, occurrence: Option<usize>) -> Option<Vec<&str>> {
        self.iter()
            .find(|entry| entry.slug == slug && entry.occurrence == occurrence)
            .and_then(|entry| entry.words.as_ref())
            .map(|tier| tier.text().iter().map(String::as_str).collect())
    }
}

/// The rendered bytes for one arrangement key: the entry's words concatenated, or the
/// `[unwritten: <slug>]` placeholder the catalog renders for the same state.
#[must_use]
pub fn arrangement_text(
    lookup: &dyn ArrangementLookup,
    slug: &str,
    occurrence: Option<usize>,
) -> String {
    arrangement_sentence(lookup, slug, occurrence, &[])
}

/// One chrome line composed from its entry's ORDERED WORDS and the renderer's computed values,
/// alternating `words[0] values[0] words[1] …` — the shape a sentence with interpolated counts
/// needs, and the reason entries are sequences rather than flat strings.
///
/// The whole line is ONE editable SECTION, never several: a value may sit INSIDE it, but nothing
/// splits one chrome line across sections (`28H` ruling 3, amending the older one-span rule). This
/// seat CONCATENATES, for callers that only want the bytes; a caller that wants an editable face
/// stamps the pieces separately and lets the transport re-split them
/// ([`sentence_words`] is the shared arity seat both go through).
#[must_use]
pub fn arrangement_sentence(
    lookup: &dyn ArrangementLookup,
    slug: &str,
    occurrence: Option<usize>,
    values: &[&str],
) -> String {
    match sentence_words(lookup, slug, occurrence, values.len()) {
        Some(words) => interleave(&words, values),
        None => unwritten_placeholder(slug),
    }
}

/// `words[0] values[0] words[1] …` — the one arithmetic both the flat seat and [`ComponentText`]
/// concatenate by, so a component's bytes and its pieces can never disagree.
fn interleave(words: &[impl AsRef<str>], values: &[impl AsRef<str>]) -> String {
    let mut out = String::new();
    for (index, word) in words.iter().enumerate() {
        out.push_str(word.as_ref());
        if let Some(value) = values.get(index) {
            out.push_str(value.as_ref());
        }
    }
    out
}

/// One prose-component resolved WHOLE: the entry's own words and the values the seat interleaves,
/// kept apart rather than concatenated, plus the bytes they concatenate to.
///
/// The separation is what lets a component be stamped with its OWN face when a catalog register is
/// nothing but the hole it fills (`28L:rul-empty-registers-for-pure-holes`): the words belong to the
/// registry entry, so the entry is where an edit to them lands, and the value boundaries the entry's
/// arity depends on survive the round trip.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComponentText {
    slug: &'static str,
    occurrence: Option<usize>,
    words: Vec<String>,
    values: Vec<String>,
    text: String,
}

impl ComponentText {
    /// The rendered bytes.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The registry slug this component came from.
    #[must_use]
    pub fn slug(&self) -> &'static str {
        self.slug
    }

    /// The occurrence the seat resolved, or `None` for the whole-slug entry.
    #[must_use]
    pub fn occurrence(&self) -> Option<usize> {
        self.occurrence
    }

    /// Stamp the component onto a part stream as its own editable line — `words[0]`, `values[0]`,
    /// `words[1]`, … — so the transport re-splits an edit at exactly the boundaries the render
    /// placed.
    pub fn push_parts(&self, parts: &mut RenderParts) {
        for (index, word) in self.words.iter().enumerate() {
            parts.push(RenderPart::ArrangementWords {
                text: word.clone(),
                slug: self.slug,
                occurrence: self.occurrence,
            });
            if let Some(value) = self.values.get(index) {
                parts.push(RenderPart::ArrangementValue {
                    text: value.clone(),
                    slug: self.slug,
                    occurrence: self.occurrence,
                    index,
                });
            }
        }
    }
}

/// Resolve one prose-component for a seat passing `values`, or the greppable placeholder when the
/// entry is unwritten or cannot serve that arity.
#[must_use]
pub fn component_text(
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
    values: &[&str],
) -> ComponentText {
    let (words, values): (Vec<String>, Vec<String>) =
        match sentence_words(lookup, slug, occurrence, values.len()) {
            Some(words) => (
                words.iter().map(|word| (*word).to_owned()).collect(),
                values.iter().map(|value| (*value).to_owned()).collect(),
            ),
            None => (vec![unwritten_placeholder(slug)], Vec::new()),
        };
    let text = interleave(&words, &values);
    ComponentText {
        slug,
        occurrence,
        words,
        values,
        text,
    }
}

/// The greppable placeholder a row with no words yet renders as. COMPUTED, never a registry row
/// of its own (`28F:rul-placeholders-are-computed`).
#[must_use]
pub fn unwritten_placeholder(slug: &str) -> String {
    format!("[unwritten: {slug}]")
}

/// The ORDERED WORDS serving `(slug, occurrence)` for a seat passing `value_count` values, or
/// `None` when the row is unwritten or cannot serve that arity.
///
/// The ONE seat that rules the `words == values + 1` arity, so a renderer that interleaves the
/// values itself and one that concatenates them agree about which rows are serviceable
/// ([`arrangement_sentence`] is the concatenating caller). An arity disagreement between a
/// WRITTEN entry and its seat is a wiring defect, not a resting state, and it is LOUD: the debug
/// assertion names the row and both counts. In release the render still degrades to the greppable
/// placeholder rather than a mangled line — but a degradation nothing announces is invisible to
/// every check except a transcript that happens to cover the seat, which is how a word-boundary
/// slip once reached a lock (`28F` loom-cleanup A1).
#[must_use]
pub fn sentence_words<'a>(
    lookup: &'a dyn ArrangementLookup,
    slug: &str,
    occurrence: Option<usize>,
    value_count: usize,
) -> Option<Vec<&'a str>> {
    let words = lookup.words(slug, occurrence)?;
    if words.len() != value_count.saturating_add(1) {
        debug_assert!(
            false,
            "arrangement `{slug}` occurrence {occurrence:?}: {} words cannot serve a seat passing \
             {value_count} values (a sentence needs values + 1 words); the render degrades to \
             `[unwritten: {slug}]`",
            words.len(),
        );
        return None;
    }
    Some(words)
}

/// Push one registry-sourced chrome LINE onto a part stream — the ONE direct seat that mints an
/// editable chrome span (the weft bridge is the other, over a laid-out render). A span minted
/// here is an EDIT REGION precisely because its bytes came out of the registry; chrome computed
/// inline stays [`RenderPart::Arrangement`] (immutable structure), so a transcript edit can never
/// rewrite a registry entry the render does not read.
pub fn push_arrangement_words(
    parts: &mut RenderParts,
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
) {
    push_arrangement_sentence(parts, lookup, slug, occurrence, &[]);
}

/// [`push_arrangement_words`] for a line carrying computed values.
///
/// The line is stamped PIECE BY PIECE — `words[0]`, `values[0]`, `words[1]`, … — so the
/// transport can re-split an edited line at exactly the boundaries the render placed. The bytes
/// are [`arrangement_sentence`]'s by construction; only the attribution differs.
pub fn push_arrangement_sentence(
    parts: &mut RenderParts,
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
    values: &[&str],
) {
    component_text(lookup, slug, occurrence, values).push_parts(parts);
}

/// The whole rendered PAGE for one arrangement key, as a one-span part stream — an invocation
/// whose entire output is one entry, laid out by its author (`288:rul-help-text-is-loomable`).
#[must_use]
pub fn arrangement_parts(
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
) -> RenderParts {
    let mut parts = RenderParts::new();
    parts.push(RenderPart::ArrangementPage {
        text: arrangement_text(lookup, slug, occurrence),
        slug,
    });
    parts
}

// ===========================================================================
// The arrangement-lock serializer (the twin of `catalog::serialize_lock`)
// ===========================================================================

/// One fully-sourced generated arrangement row. The case-first fields (`when_used`/`why` from
/// the defining case's frontmatter) are computed by the dorc-loom generator; `aid` owns only
/// the serializer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ArrangementRow {
    /// The stable arrangement slug.
    pub slug: String,
    /// The occurrence this entry answers for.
    pub occurrence: Option<usize>,
    /// Where this chrome is rendered.
    pub when_used: String,
    /// Why the entry exists.
    pub why: String,
    /// The user-facing prose, or `None` while unwritten.
    pub words: Option<ProseTier<Vec<String>>>,
}

/// Serialize the wholly-generated `arrangement_lock.rs` from ordered [`ArrangementRow`]s. Same
/// contract as [`serialize_lock`](crate::catalog::serialize_lock): the whole file is
/// generator-owned, `#[rustfmt::skip]` keeps single-line string emission `cargo fmt`-stable, and
/// the generated bytes ARE the committed bytes under the byte-identity fixpoint gate.
#[must_use]
pub fn serialize_arrangement_lock(rows: &[ArrangementRow]) -> String {
    use std::fmt::Write as _;
    let mut out = String::from(
        "// @generated by dorc-loom; DO NOT EDIT.\n\
         // This whole file is overwritten by arrangement promotion.\n\n\
         use super::{ArrangementEntry, ProseTier};\n\n\
         #[rustfmt::skip]\n\
         pub const ARRANGEMENTS: &[ArrangementEntry] = &[\n",
    );
    for row in rows {
        out.push_str("    ArrangementEntry {\n");
        let _ = writeln!(out, "        slug: {:?},", row.slug);
        match row.occurrence {
            Some(occurrence) => {
                let _ = writeln!(out, "        occurrence: Some({occurrence}),");
            }
            None => out.push_str("        occurrence: None,\n"),
        }
        let _ = writeln!(out, "        when_used: {:?},", row.when_used);
        let _ = writeln!(out, "        why: {:?},", row.why);
        out.push_str("        words: ");
        match &row.words {
            None => out.push_str("None,\n"),
            Some(tier) => write_words(&mut out, tier),
        }
        out.push_str("    },\n");
    }
    out.push_str("];\n");
    out
}

/// The arrangement twin of `catalog::tier_literal`, kept a writer because a page is one long word.
fn write_words(out: &mut String, tier: &ProseTier<Vec<String>>) {
    use std::fmt::Write as _;
    let (variant, words) = match tier {
        ProseTier::Migrated(words) => ("Migrated", words),
        ProseTier::Slop(words) => ("Slop", words),
        ProseTier::WrittenByHumanOnly(words) => ("WrittenByHumanOnly", words),
    };
    let _ = write!(out, "Some(ProseTier::{variant}(&[");
    for (index, word) in words.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{word:?}");
    }
    out.push_str("])),\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Gate: no two entries share a key — one entry answers one `(slug, occurrence)`.
    #[test]
    fn no_duplicate_keys() {
        let mut seen = std::collections::BTreeSet::new();
        for entry in ARRANGEMENTS {
            assert!(
                seen.insert((entry.slug, entry.occurrence)),
                "duplicate arrangement entry `{}` occurrence {:?}",
                entry.slug,
                entry.occurrence
            );
        }
    }

    /// Gate: the machine-facing metadata is non-empty, and no entry stores empty prose (the
    /// unwritten state is `None`, never `[""]`).
    #[test]
    fn required_metadata_is_non_empty() {
        for entry in ARRANGEMENTS {
            assert!(!entry.slug.is_empty(), "empty arrangement slug");
            assert!(!entry.when_used.is_empty(), "`{}`: when_used", entry.slug);
            assert!(!entry.why.is_empty(), "`{}`: why", entry.slug);
            if let Some(words) = entry.words.map(|tier| *tier.text()) {
                // An INDIVIDUAL word may be empty — a line ending in a computed value needs a
                // trailing empty word to satisfy the words = values + 1 arity. An entry with no
                // words at all is the unwritten state, which has its own variant.
                assert!(
                    words.iter().any(|word| !word.is_empty()),
                    "`{}`: a written entry has at least one word — unwritten is `None`",
                    entry.slug
                );
            }
        }
    }

    use crate::case_ownership::is_case_owned;

    /// Gate (the `catalog::loom_minted_registers_are_case_owned` twin): a row the loom minted —
    /// `Slop` or `WrittenByHumanOnly` alike — needs a case, so its bytes are fixpoint-protected.
    #[test]
    fn loom_minted_words_are_case_owned() {
        for entry in ARRANGEMENTS {
            if entry.words.is_some_and(|tier| tier.is_loom_minted()) {
                assert!(
                    is_case_owned(entry.slug),
                    "arrangement `{}`: loom-minted words need a defining case — own them from a \
                     case's `owns:` frontmatter (the declaration-union; a component filling \
                     another code's hole is owned by THAT code's case), or name the case for the \
                     slug. Re-tiering to `ProseTier::Migrated` is for PRE-PIPELINE builder text \
                     only, never for words minted now",
                    entry.slug
                );
            }
        }
    }

    /// The generated lock owns the whole table: no hand-written row hides in this file.
    #[test]
    fn generated_lock_owns_the_complete_table() {
        let lock = include_str!("arrangement_lock.rs");
        assert!(lock.starts_with("// @generated by dorc-loom; DO NOT EDIT.\n"));
        assert_eq!(
            lock.matches("\n    ArrangementEntry {\n").count(),
            ARRANGEMENTS.len()
        );
        assert_eq!(
            include_str!("arrangement.rs")
                .matches("\n    ArrangementEntry {\n")
                .count(),
            0
        );
    }

    /// Occurrence resolution: an occurrence's own entry wins; otherwise the whole-slug entry
    /// serves it. This is what lets repeated chrome share ONE editable entry while a future
    /// per-position entry can override exactly one of them.
    #[test]
    fn an_occurrence_falls_back_to_the_whole_slug_entry() {
        let registry = vec![
            OwnedArrangement {
                slug: "shared".to_owned(),
                occurrence: None,
                when_used: "w".to_owned(),
                why: "y".to_owned(),
                words: Some(ProseTier::Migrated(vec!["shared words".to_owned()])),
            },
            OwnedArrangement {
                slug: "shared".to_owned(),
                occurrence: Some(2),
                when_used: "w".to_owned(),
                why: "y".to_owned(),
                words: Some(ProseTier::Migrated(vec!["third only".to_owned()])),
            },
        ];
        assert_eq!(
            registry.words("shared", None),
            Some(vec!["shared words"]),
            "the whole-slug entry answers an unstamped span"
        );
        assert_eq!(
            registry.words("shared", Some(0)),
            Some(vec!["shared words"]),
            "an unclaimed occurrence falls back"
        );
        assert_eq!(
            registry.words("shared", Some(2)),
            Some(vec!["third only"]),
            "a claimed occurrence wins"
        );
        assert_eq!(registry.words("absent", None), None);
    }

    fn tally_registry() -> Vec<OwnedArrangement> {
        vec![OwnedArrangement {
            slug: "tally".to_owned(),
            occurrence: None,
            when_used: "w".to_owned(),
            why: "y".to_owned(),
            words: Some(ProseTier::Slop(vec![
                "found ".to_owned(),
                " thing".to_owned(),
                ".".to_owned(),
            ])),
        }]
    }

    /// A sentence entry interleaves its words with the seat's computed values; an absent entry
    /// renders the greppable placeholder.
    #[test]
    fn a_sentence_interleaves_words_and_values() {
        assert_eq!(
            arrangement_sentence(&tally_registry(), "tally", None, &["2", "s"]),
            "found 2 things."
        );
        assert_eq!(
            arrangement_text(&tally_registry(), "nope", None),
            "[unwritten: nope]"
        );
    }

    /// The loud half: a WRITTEN entry that cannot serve its seat is a wiring defect. Before this
    /// assertion the render simply degraded to the unwritten placeholder, so editing a row's word
    /// boundaries was invisible to everything but a transcript that happened to cover the seat.
    /// The message must carry the row and BOTH counts — the diagnosis is the arithmetic.
    #[test]
    #[should_panic(expected = "3 words cannot serve a seat passing 1 values")]
    fn an_arity_slip_names_the_row_and_both_counts() {
        let _ = arrangement_sentence(&tally_registry(), "tally", None, &["2"]);
    }

    /// The serializer emits the pinned generated header and a `#[rustfmt::skip]` const, so the
    /// generator output IS the committed bytes (the byte-identity fixpoint precondition).
    #[test]
    fn serialize_emits_the_pinned_generated_header() {
        let src = serialize_arrangement_lock(&[ArrangementRow {
            slug: "x".to_owned(),
            occurrence: Some(1),
            when_used: "w".to_owned(),
            why: "y".to_owned(),
            words: Some(ProseTier::Migrated(vec![
                "one".to_owned(),
                " two".to_owned(),
            ])),
        }]);
        assert!(src.starts_with("// @generated by dorc-loom; DO NOT EDIT.\n"));
        assert!(
            src.contains("#[rustfmt::skip]\npub const ARRANGEMENTS: &[ArrangementEntry] = &[\n")
        );
        assert!(src.contains("        occurrence: Some(1),\n"));
        assert!(src.contains("        words: Some(ProseTier::Migrated(&[\"one\", \" two\"])),\n"));
        assert!(src.trim_end().ends_with("];"));
    }
}
