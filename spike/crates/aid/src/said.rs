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

use crate::arrangement::{CONST_ARRANGEMENTS, arrangement_sentence};
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
    /// One registry-sourced line, with the arrangement slug it was composed from.
    Words(&'static str, String),
    /// A value the engine computed: a coordinate, an address, a count.
    Value(String),
    /// Prose the why-lens flattened to a string before this seat could see its parts — the
    /// standing `289:seam-whylens-render-seat`. It is not editable and cannot yet name the row it
    /// came from; giving it a real seat is `28G` Phase W4's.
    Lens(String),
}

impl Said {
    /// One registry line, its values interleaved.
    #[must_use]
    pub fn words(slug: &'static str, values: &[&str]) -> Self {
        Said::Words(slug, words_text(slug, None, values))
    }

    /// The bytes this fragment renders as.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Said::Words(_, text) | Said::Value(text) | Said::Lens(text) => text,
        }
    }

    /// The fragment as an attributed run. `part` names the seat for anything with no registry
    /// entry of its own to point at.
    #[must_use]
    pub fn run(&self, part: &'static str) -> Run<Face> {
        match self {
            Said::Words(slug, text) => crate::weave::words(text.clone(), slug),
            Said::Value(text) => crate::weave::value(text, part, "value", WHY_VALUE_CAP),
            Said::Lens(text) => crate::weave::value(text, "why-lens", "reason", WHY_SOURCE_CAP),
        }
    }
}

/// One registry-sourced why-surface line, values interleaved between the entry's words.
///
/// This is the ONE seat that interleaves a computed value into a registry line, and therefore the
/// one place a value carrying bytes we did not write can enter our own words. The registry words
/// are never encoded — they are ours, and encoding them twice would be a defect — while every
/// value passes the display seat first (`sinv-sink-encoding`). A chrome line renders as ONE span
/// (`a-chrome-line-is-one-span`), so the value cannot carry its own foreign-text span here and
/// must instead arrive already safe.
#[must_use]
pub fn words_text(slug: &str, occurrence: Option<usize>, values: &[&str]) -> String {
    let encoded: Vec<String> = values
        .iter()
        .map(|value| encode_foreign(value, WHY_VALUE_CAP))
        .collect();
    let borrowed: Vec<&str> = encoded.iter().map(String::as_str).collect();
    arrangement_sentence(&CONST_ARRANGEMENTS, slug, occurrence, &borrowed)
}
