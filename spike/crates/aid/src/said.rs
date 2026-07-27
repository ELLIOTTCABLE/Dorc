//! What a why-surface line is MADE OF: the fragments it was composed from, each carrying where
//! its bytes came from.
//!
//! This is describe-plane vocabulary (`aid-is-the-describe-plane`), which is why it sits here
//! beside [`weave`](crate::weave) rather than at the cli edge where it grew: every dependency it
//! has — the arrangement registry, the display seat, the weave constructors — is already this
//! crate's, and both why surfaces (the plan-stderr lens and the `dorc why` report) have to speak
//! it or one of them ends up flattening prose to a string before the other can attribute it
//! (`289:seam-whylens-render-seat`).
//!
//! Carrying the origin past composition is what keeps `28G` §0 honest: the bytes reach weft
//! already interleaved (`a-chrome-line-is-one-span`), so without this the span map would name the
//! seat that assembled a line rather than the entry an edit has to rewrite. It also keeps the
//! classes apart — registry words are rephrasable, a computed value is not, and rewriting one
//! would be lying about the world.

use crate::arrangement::{
    CONST_ARRANGEMENTS, arrangement_sentence, sentence_words, unwritten_placeholder,
};
use crate::display::encode_foreign;
use crate::weave::Face;
use weft::Run;

/// The display budget for one computed value on the why surface: a coordinate, an address, a
/// speaker, a `N|command` reference. Generous enough that nothing the corpus produces is touched,
/// bounded so a pathological book word cannot own the whole render.
pub const WHY_VALUE_CAP: usize = 240;

/// The display budget for one quoted line of somebody else's source. Wider than a value's because
/// a wrapped-off source line is a worse lie than a long one, and still bounded.
pub const WHY_SOURCE_CAP: usize = 512;

/// A rendered fragment of the why surface, and where its bytes came from.
#[derive(Clone, Debug)]
pub enum Said {
    /// One registry-sourced line: the row it came from, the occurrence the seat keyed it at, and
    /// the values interleaved between its words.
    ///
    /// The values are kept UNCOMPOSED so the line can be stamped piece by piece
    /// ([`Said::runs`]): flattening it here is what left every value-bearing chrome line
    /// un-editable, since nothing downstream could recover where the words re-divide.
    Words {
        /// The arrangement-registry slug.
        slug: &'static str,
        /// The occurrence the seat resolved, or `None` for the whole-slug entry.
        occurrence: Option<usize>,
        /// What goes between the entry's words, in order.
        values: Vec<Said>,
    },
    /// A value the engine computed: a coordinate, an address, a count.
    Value(String),
    /// Punctuation the CONSUMER computed — the quotes around an inlined excerpt, the space
    /// between two joined fragments. Not a word (`layout-is-not-a-word`), so deliberately not a
    /// registry row: putting shape in an editable entry would weld the arrangement
    /// `27V:rul-output-form-unwelded` keeps free.
    Mark(&'static str, String),
    /// Bytes taken from somebody else's file, ENCODED AT MINT — so a surface that shows them is
    /// safe by construction rather than by each seat remembering (`28D:must-encode-per-surface`).
    Foreign {
        /// The bytes, already encoded.
        text: String,
        /// What they were taken from.
        source: String,
    },
    /// Several fragments that render as ONE sentence — a cause explanation alternating registry
    /// words, computed values and not-ours bytes. The stream is what a reason IS at birth;
    /// flattening it to a string was the old why-lens seat, and what left the stderr surface
    /// unable to name the row an edit would rewrite (`289:seam-whylens-render-seat`).
    Parts(Vec<Said>),
}

impl Said {
    /// One registry line, its values interleaved.
    #[must_use]
    pub fn words(slug: &'static str, values: &[&str]) -> Self {
        Said::words_at(slug, None, values)
    }

    /// [`Said::words`] for a registry row whose words are keyed by occurrence.
    #[must_use]
    pub fn words_at(slug: &'static str, occurrence: Option<usize>, values: &[&str]) -> Self {
        Said::sentence(
            slug,
            occurrence,
            values
                .iter()
                .map(|value| Said::Value((*value).to_owned()))
                .collect(),
        )
    }

    /// One registry line whose values are themselves composed fragments — a because-clause that
    /// is a whole sub-sentence, a payload that quotes somebody else's bytes.
    ///
    /// The distinction is a budget, not a shape: a RAW [`Said::Value`] is encoded and capped at
    /// [`WHY_VALUE_CAP`] on the way in, while a composed value arrived already encoded and
    /// already capped fragment by fragment, so re-capping it would truncate OUR OWN words
    /// (`28H:ask-because-clause-truncates-at-two-forty`).
    #[must_use]
    pub fn sentence(slug: &'static str, occurrence: Option<usize>, values: Vec<Said>) -> Self {
        Said::Words {
            slug,
            occurrence,
            values,
        }
    }

    /// Somebody else's bytes, encoded HERE so no later seat can forget to.
    #[must_use]
    pub fn foreign(text: &str, source: impl Into<String>) -> Self {
        Said::Foreign {
            text: encode_foreign(text, WHY_SOURCE_CAP),
            source: source.into(),
        }
    }

    /// The bytes this fragment renders as. A [`Said::Parts`] stream renders as its fragments
    /// concatenated: runs carry their own spacing, so nothing is inserted between them.
    #[must_use]
    pub fn text(&self) -> String {
        match self {
            Said::Words {
                slug,
                occurrence,
                values,
            } => {
                let interleaved = interleaved_values(values);
                let borrowed: Vec<&str> = interleaved.iter().map(String::as_str).collect();
                arrangement_sentence(&CONST_ARRANGEMENTS, slug, *occurrence, &borrowed)
            }
            Said::Value(text) | Said::Mark(_, text) | Said::Foreign { text, .. } => text.clone(),
            Said::Parts(parts) => parts.iter().map(Said::text).collect(),
        }
    }

    /// The fragment as attributed runs. `part` names the seat for anything with no registry entry
    /// of its own to point at.
    ///
    /// The ONE seat that turns fragments into bytes, for BOTH why surfaces: the report hands the
    /// runs to weft, the stderr lens concatenates their text. Neither can drift from the other
    /// about what a fragment says or which class it wears.
    #[must_use]
    pub fn runs(&self, part: &'static str) -> Vec<Run<Face>> {
        match self {
            Said::Words {
                slug,
                occurrence,
                values,
            } => sentence_runs(slug, *occurrence, values),
            Said::Value(text) => vec![crate::weave::value(text, part, WHY_VALUE_CAP)],
            Said::Mark(mark, text) => vec![crate::weave::mark(text.clone(), mark)],
            Said::Foreign { text, source } => {
                vec![crate::weave::foreign(text, source.clone(), WHY_SOURCE_CAP)]
            }
            Said::Parts(parts) => parts.iter().flat_map(|said| said.runs(part)).collect(),
        }
    }
}

/// One value's bytes as they enter a line: encoded at its own budget (see [`Said::sentence`]).
fn interleaved_value(said: &Said) -> String {
    match said {
        Said::Value(text) => encode_foreign(text, WHY_VALUE_CAP),
        composed => composed.text(),
    }
}

fn interleaved_values(values: &[Said]) -> Vec<String> {
    values.iter().map(interleaved_value).collect()
}

/// A registry line stamped PIECE BY PIECE: `words[0]`, `values[0]`, `words[1]`, … each its own
/// run, so the span map names the row an edit has to rewrite and the value boundaries the
/// transport has to preserve.
///
/// Byte-identical to [`arrangement_sentence`] by construction — runs carry their own spacing and
/// weft inserts nothing between them — so giving a line a face never moves a rendered byte.
fn sentence_runs(slug: &'static str, occurrence: Option<usize>, values: &[Said]) -> Vec<Run<Face>> {
    let interleaved = interleaved_values(values);
    let Some(words) = sentence_words(&CONST_ARRANGEMENTS, slug, occurrence, values.len()) else {
        // The placeholder is COMPUTED (`28F:rul-placeholders-are-computed`), but it still wears
        // its row's face: seeding an unwritten row by editing its placeholder is the ordinary
        // way a chrome slug acquires words at the loom surface.
        return vec![crate::weave::words(
            unwritten_placeholder(slug),
            slug,
            occurrence,
        )];
    };
    let mut runs = Vec::new();
    for (index, word) in words.iter().enumerate() {
        runs.push(crate::weave::words((*word).to_owned(), slug, occurrence));
        if let (Some(value), Some(text)) = (values.get(index), interleaved.get(index)) {
            runs.push(crate::weave::sentence_value(
                text,
                slug,
                occurrence,
                index,
                value_cap(value),
            ));
        }
    }
    runs
}

/// A raw value is capped; a composed one is not (see [`Said::sentence`]).
fn value_cap(said: &Said) -> usize {
    match said {
        Said::Value(_) => WHY_VALUE_CAP,
        _ => usize::MAX,
    }
}

/// One registry-sourced why-surface line, values interleaved between the entry's words.
///
/// The FLAT seat, for callers that want only the bytes — [`Said::sentence`] is the attributing
/// one. The registry words are never encoded — they are ours, and encoding them twice would be a
/// defect — while every value passes the display seat first (`sinv-sink-encoding`), so a value
/// carrying bytes we did not write is already safe before it enters our own words.
#[must_use]
pub fn words_text(slug: &str, occurrence: Option<usize>, values: &[&str]) -> String {
    let encoded: Vec<String> = values
        .iter()
        .map(|value| encode_foreign(value, WHY_VALUE_CAP))
        .collect();
    let borrowed: Vec<&str> = encoded.iter().map(String::as_str).collect();
    arrangement_sentence(&CONST_ARRANGEMENTS, slug, occurrence, &borrowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A parts stream renders as its fragments, in order, with nothing inserted between them —
    /// which is what lets a seat that still flattens keep printing the same bytes while an
    /// attributing one gets its runs.
    #[test]
    fn a_parts_stream_renders_as_its_fragments_concatenated() {
        let stream = Said::Parts(vec![
            Said::Value("6:20".to_owned()),
            Said::Mark("why-cause-quote", " `".to_owned()),
            Said::foreign("apt-get install -y \"$PKG\"", "book.sh"),
            Said::Mark("why-cause-quote", "`".to_owned()),
        ]);
        assert_eq!(stream.text(), "6:20 `apt-get install -y \"$PKG\"`");
        assert_eq!(
            stream
                .runs("why-lens")
                .iter()
                .map(|run| run.text.clone())
                .collect::<String>(),
            stream.text(),
            "the run seat and the text seat agree byte for byte"
        );
    }

    /// Encoding at MINT is the difference between a surface that is safe and one that remembers to
    /// be: a fragment carrying a terminal escape is already harmless before any seat sees it.
    #[test]
    fn not_ours_bytes_are_encoded_before_any_surface_sees_them() {
        let said = Said::foreign("red \u{1b}[31m alert", "oracle.sh");
        assert_eq!(said.text(), "red \\x1b[31m alert");
        assert!(said.text().is_ascii());
    }
}
