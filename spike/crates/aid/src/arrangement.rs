//! The arrangement registry — the SECOND generated prose table
//! (`289:rul-arrangement-home-is-registry-plus-transcripts`), holding the render-owned CHROME
//! the catalog never covered: help/usage pages, structural connectives, preambles, summary
//! lines. It generalizes the proven catalog pipeline rather than inventing a second one:
//! a generated lock (`arrangement_lock.rs`, the sibling of `catalog_lock.rs`), the same
//! three-state prose protocol, the same two fixpoint gates, and the same
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
//! # Three prose states, out of band
//!
//! [`Words`] carries the catalog's three states — unwritten / verbatim-migrated /
//! authored — but as a TYPE rather than the catalog's in-band `sm ` prefix. Chrome is rendered
//! verbatim into product output, so an in-band marker would change the shipped bytes and the
//! migration would stop being a pure storage move. The trade is that the marker is invisible in
//! a transcript; the gate reads the type instead.
//!
//! Nothing here decides anything (`two-plane-aid-law`), and nothing here reads the world:
//! the registry is const data plus a pure render (`aid-is-dst-clean`).

use crate::tagged::{RenderPart, RenderParts};

/// The three legal states of one entry's prose (the catalog's `sm `/`[unwritten:]`/authored
/// protocol, moved out of band — see the module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Words {
    /// No words authored yet; renders the greppable `[unwritten: <slug>]` placeholder.
    Unwritten,
    /// Shipped text migrated VERBATIM into the registry, awaiting human rewrite. The typed
    /// twin of the catalog's `sm ` marker (`27V:rul-error-authorship-tier`: builders migrate,
    /// they never author).
    Migrated(&'static [&'static str]),
    /// Human/conductor-authored words, reachable only through a case's transcript.
    Authored(&'static [&'static str]),
}

impl Words {
    /// The words, or `None` when unwritten.
    #[must_use]
    pub fn words(&self) -> Option<&'static [&'static str]> {
        match self {
            Words::Unwritten => None,
            Words::Migrated(words) | Words::Authored(words) => Some(words),
        }
    }
}

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
    /// The user-facing prose.
    pub words: Words,
}

#[path = "arrangement_lock.rs"]
mod arrangement_lock;
pub use arrangement_lock::ARRANGEMENTS;

/// The render seat's view of the arrangement registry — the twin of
/// [`CatalogLookup`](crate::catalog::CatalogLookup), so a render can source chrome from the
/// compiled-in const OR a promote-time mutable mirror through ONE seat.
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
            .and_then(|entry| entry.words.words())
            .map(<[&str]>::to_vec)
    }
}

/// An owned entry — the promote-time MUTABLE mirror's element, the twin of
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
    /// The user-facing prose.
    pub words: OwnedWords,
}

/// The owned twin of [`Words`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum OwnedWords {
    /// No words authored yet.
    Unwritten,
    /// Shipped text migrated verbatim, awaiting human rewrite.
    Migrated(Vec<String>),
    /// Human/conductor-authored words.
    Authored(Vec<String>),
}

impl OwnedWords {
    /// The words, or `None` when unwritten.
    #[must_use]
    pub fn words(&self) -> Option<&[String]> {
        match self {
            OwnedWords::Unwritten => None,
            OwnedWords::Migrated(words) | OwnedWords::Authored(words) => Some(words),
        }
    }
}

/// The compiled-in registry as an owned, mutable mirror — the starting state promote edits
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
            words: match entry.words {
                Words::Unwritten => OwnedWords::Unwritten,
                Words::Migrated(words) => OwnedWords::Migrated(owned_words(words)),
                Words::Authored(words) => OwnedWords::Authored(owned_words(words)),
            },
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
            .and_then(|entry| entry.words.words())
            .map(|words| words.iter().map(String::as_str).collect())
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
/// The whole line renders as ONE span, so a value never splits a chrome line into fragments the
/// edit transport has to re-anchor between (short computed runs — a bare digit, an empty plural
/// suffix — are not anchors, and fragmenting on them breaks attribution for every OTHER prose
/// section in the same render). The price, stated: a multi-word entry cannot be edited back from a
/// transcript yet, because nothing re-splits an edited line at its value boundaries — the edit path
/// refuses loudly rather than guessing (`DorcApplyRefusal::ArrangementIsSequenceStructured`).
///
/// An arity disagreement between the entry and its seat renders the greppable unwritten
/// placeholder: an entry that cannot serve its seat has no words for it.
#[must_use]
pub fn arrangement_sentence(
    lookup: &dyn ArrangementLookup,
    slug: &str,
    occurrence: Option<usize>,
    values: &[&str],
) -> String {
    let Some(words) = lookup.words(slug, occurrence) else {
        return format!("[unwritten: {slug}]");
    };
    if words.len() != values.len().saturating_add(1) {
        return format!("[unwritten: {slug}]");
    }
    let mut out = String::new();
    for (index, word) in words.iter().enumerate() {
        out.push_str(word);
        if let Some(value) = values.get(index) {
            out.push_str(value);
        }
    }
    out
}

/// Push one registry-sourced arrangement span onto a part stream — the ONE seat that mints an
/// editable chrome span. A span minted here is an EDIT REGION precisely because its bytes came
/// out of the registry; chrome computed inline stays [`RenderPart::Arrangement`] (immutable
/// structure), so a transcript edit can never rewrite a registry entry the render does not read.
pub fn push_arrangement_words(
    parts: &mut RenderParts,
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
) {
    push_arrangement_sentence(parts, lookup, slug, occurrence, &[]);
}

/// [`push_arrangement_words`] for a line carrying computed values (see [`arrangement_sentence`]).
pub fn push_arrangement_sentence(
    parts: &mut RenderParts,
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
    values: &[&str],
) {
    parts.push(RenderPart::ArrangementWords {
        text: arrangement_sentence(lookup, slug, occurrence, values),
        slug,
        occurrence,
    });
}

/// The whole rendered page/line for one arrangement key, as a one-span part stream.
#[must_use]
pub fn arrangement_parts(
    lookup: &dyn ArrangementLookup,
    slug: &'static str,
    occurrence: Option<usize>,
) -> RenderParts {
    let mut parts = RenderParts::new();
    push_arrangement_words(&mut parts, lookup, slug, occurrence);
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
    /// The user-facing prose.
    pub words: OwnedWords,
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
         use super::{ArrangementEntry, Words};\n\n\
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
            OwnedWords::Unwritten => out.push_str("Words::Unwritten,\n"),
            OwnedWords::Migrated(words) => {
                write_words(&mut out, "Migrated", words);
            }
            OwnedWords::Authored(words) => {
                write_words(&mut out, "Authored", words);
            }
        }
        out.push_str("    },\n");
    }
    out.push_str("];\n");
    out
}

fn write_words(out: &mut String, variant: &str, words: &[String]) {
    use std::fmt::Write as _;
    let _ = write!(out, "Words::{variant}(&[");
    for (index, word) in words.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{word:?}");
    }
    out.push_str("]),\n");
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
    /// unwritten state is [`Words::Unwritten`], never `[""]`).
    #[test]
    fn required_metadata_is_non_empty() {
        for entry in ARRANGEMENTS {
            assert!(!entry.slug.is_empty(), "empty arrangement slug");
            assert!(!entry.when_used.is_empty(), "`{}`: when_used", entry.slug);
            assert!(!entry.why.is_empty(), "`{}`: why", entry.slug);
            if let Some(words) = entry.words.words() {
                assert!(
                    !words.is_empty() && words.iter().all(|word| !word.is_empty()),
                    "`{}`: written words are non-empty — unwritten is `Words::Unwritten`",
                    entry.slug
                );
            }
        }
    }

    /// Whether `slug`'s prose is CASE-OWNED: an arrangement case file exists for it in the
    /// primary loom collection. The twin of the catalog's `is_case_owned`, and the reason a
    /// human may author words at all — the render-level fixpoint gate protects them.
    fn is_case_owned(slug: &str) -> bool {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join(format!("{slug}.loom"))
            .exists()
    }

    /// Gate (the arrangement twin of `message_registers_are_sm_or_unwritten`): authored words
    /// exist only for a case-owned slug, so every string a human wrote is fixpoint-protected
    /// and every string a BUILDER put here is marked [`Words::Migrated`]
    /// (`27V:rul-error-authorship-tier` — builders migrate verbatim, they never author).
    #[test]
    fn authored_words_are_case_owned() {
        for entry in ARRANGEMENTS {
            if matches!(entry.words, Words::Authored(_)) {
                assert!(
                    is_case_owned(entry.slug),
                    "arrangement `{}`: authored words need a defining case; builder-migrated \
                     text is `Words::Migrated`",
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
                words: OwnedWords::Migrated(vec!["shared words".to_owned()]),
            },
            OwnedArrangement {
                slug: "shared".to_owned(),
                occurrence: Some(2),
                when_used: "w".to_owned(),
                why: "y".to_owned(),
                words: OwnedWords::Migrated(vec!["third only".to_owned()]),
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

    /// A sentence entry interleaves its words with the seat's computed values; an absent entry, or
    /// one whose word count cannot serve the seat's value count, renders the greppable placeholder
    /// rather than a silently-wrong line.
    #[test]
    fn a_sentence_interleaves_words_and_values_or_refuses_by_arity() {
        let registry = vec![OwnedArrangement {
            slug: "tally".to_owned(),
            occurrence: None,
            when_used: "w".to_owned(),
            why: "y".to_owned(),
            words: OwnedWords::Authored(vec![
                "found ".to_owned(),
                " thing".to_owned(),
                ".".to_owned(),
            ]),
        }];
        assert_eq!(
            arrangement_sentence(&registry, "tally", None, &["2", "s"]),
            "found 2 things."
        );
        assert_eq!(
            arrangement_sentence(&registry, "tally", None, &["2"]),
            "[unwritten: tally]",
            "an entry that cannot serve its seat has no words for it"
        );
        assert_eq!(
            arrangement_text(&registry, "nope", None),
            "[unwritten: nope]"
        );
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
            words: OwnedWords::Migrated(vec!["one".to_owned(), " two".to_owned()]),
        }]);
        assert!(src.starts_with("// @generated by dorc-loom; DO NOT EDIT.\n"));
        assert!(
            src.contains("#[rustfmt::skip]\npub const ARRANGEMENTS: &[ArrangementEntry] = &[\n")
        );
        assert!(src.contains("        occurrence: Some(1),\n"));
        assert!(src.contains("        words: Words::Migrated(&[\"one\", \" two\"]),\n"));
        assert!(src.trim_end().ends_with("];"));
    }
}
