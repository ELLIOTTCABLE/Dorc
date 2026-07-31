//! The seal on bytes we did not write (`282:rul-passthrough-type-gated`).
//!
//! A passthrough hole shows a person text nobody on this side of the boundary composed: a
//! platform's own error words, an excerpt of somebody's book, a tool's raw output. Those holes are
//! not editable prose, and the render says so by stamping them a class of their own. What was
//! missing until now was any reason to believe a hole so stamped really held foreign bytes: the
//! catalog decided by asking whether the declared parameter happened to be NAMED `detail`, so a
//! sentence we wrote in a `format!` at an emit site arrived wearing the same badge — un-editable at
//! the loom and, worse, indistinguishable from a host's words to anything downstream reasoning
//! about provenance.
//!
//! The fix is the [`core::room`](dorc_core::room) move: make the property a TYPE rather than a
//! convention. [`ForeignBytes`] holds text no in-repo literal can reach, because its constructors
//! sit at genuine I/O edges and take the edge's own values; [`ForeignText`] is what comes back out,
//! and it exists only on the far side of the display seat. A render part carrying [`ForeignText`]
//! is therefore foreign because of where its bytes came from, not because of what a parameter was
//! called — and our own sentences physically cannot ride a passthrough.
//!
//! # The two types, and why there are two
//!
//! Hostility and encoding are orthogonal (`sinv-hostile-sensitive-orthogonal`): passing bytes
//! through an encoder makes them safe to print and says nothing about who wrote them, so the raw
//! material and the sink-encoded material are different things and are kept apart.
//!
//! * [`ForeignBytes`] — the raw seal. Minted at an edge, never read raw: the only ways out are the
//!   two sink encoders, so nothing can display foreign material by forgetting to encode it.
//! * [`ForeignText`] — the encoded seal. Safe to show, by construction, which is why it is the
//!   thing render parts store and why reading it back as `&str` is sound.
//!
//! The cap and the destination both belong to the SURFACE, not to the bytes, which is why encoding
//! happens at the render seat rather than at the edge: a quoted source line and a diagnostic
//! passthrough want different budgets and different encodings of the same value.

use crate::display::{encode_foreign, encode_line};

/// Bytes a managed host, a platform, or somebody else's file produced, held raw.
///
/// **When-blocked:** if a signature wants a `ForeignBytes` and you hold a `String` you composed,
/// you are about to put OUR words on a passthrough. Do not reach for
/// [`from_io_edge`](ForeignBytes::from_io_edge) to get past it — that constructor is fenced to the
/// edges listed on it. The sentence belongs in the catalog register with typed holes for the values
/// it interpolates (`282:rul-passthrough-type-gated`).
#[derive(Clone, PartialEq, Eq)]
pub struct ForeignBytes {
    /// Raw and private: the encoders below are the only readers, which is what makes the display
    /// seat total rather than remembered.
    raw: String,
}

impl ForeignBytes {
    /// The platform's own words about a failed operation — the one edge whose type says foreign by
    /// itself, so no fence is needed to keep our sentences out of it.
    #[must_use]
    pub fn from_os_error(err: &std::io::Error) -> Self {
        Self {
            raw: err.to_string(),
        }
    }

    /// Bytes read from a file somebody else wrote, or captured from a tool we ran — the edges whose
    /// own type is already `String` by the time a diagnostic sees them.
    ///
    /// Loudly named and LEXICALLY FENCED (`foreign_edge_constructor_is_fenced`, the
    /// `admit_fixture_records` precedent) because a bare-`&str` constructor is exactly the hole the
    /// seal exists to close: unfenced, any literal reaches it. Call sites are the book/oracle
    /// source relays, external-tool output capture, and the fixture table.
    #[must_use]
    pub fn from_io_edge(raw: &str) -> Self {
        Self {
            raw: raw.to_owned(),
        }
    }

    /// For a surface that MEASURES its bytes as columns — a weft-laid render, a quoted source line.
    /// Non-printable bytes become `\xNN` so the escaped form occupies the columns it claims.
    #[must_use]
    pub fn on_measured_sink(&self, cap: usize) -> ForeignText {
        ForeignText {
            text: encode_foreign(&self.raw, cap),
        }
    }

    /// For a surface nothing measures — an advisory line printed straight to a terminal. Control
    /// and bidi characters become spaces; everything else survives as its author wrote it.
    #[must_use]
    pub fn on_plain_sink(&self, cap: usize) -> ForeignText {
        ForeignText {
            text: encode_line(&self.raw, cap),
        }
    }
}

/// Foreign bytes that have passed the display seat: safe to print, and still marked not-ours.
///
/// Reading these back as `&str` is sound precisely because the only way to obtain one is through
/// [`ForeignBytes::on_measured_sink`] or [`ForeignBytes::on_plain_sink`] — there is no constructor
/// that skips the encoder, so "already encoded" is a property of the type rather than a claim a
/// caller makes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ForeignText {
    text: String,
}

impl ForeignText {
    /// The encoded bytes.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Re-wrap bytes THIS CRATE already encoded — the weft span map handing a foreign run back on
    /// its way to a render part (`crate::weave::to_render_parts`).
    ///
    /// Crate-private on purpose: it is the one door that skips the encoder, and it is sound only
    /// because the bytes it re-wraps came out of [`ForeignBytes`] a moment earlier, inside this
    /// crate. Outside `aid` there is no way to reach a `ForeignText` except through an edge.
    pub(crate) fn already_encoded(text: String) -> Self {
        Self { text }
    }
}

/// A debug view shows the ENCODED form, so a payload dumped into a panic message or a test failure
/// cannot carry a terminal escape out of a surface that never asked to render foreign material.
impl std::fmt::Debug for ForeignBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ForeignBytes")
            .field(&encode_foreign(&self.raw, usize::MAX))
            .finish()
    }
}

/// One filled catalog hole, carrying the CLASS of its bytes rather than the name they were declared
/// under (`282:rul-passthrough-type-gated`).
///
/// This is what replaced `is_foreign_param`. The old seat asked whether a parameter was called
/// `detail`; this one asks what the payload field's type is, so a code that renames its passthrough
/// keeps its class and a code that composes a sentence cannot acquire one.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParamText {
    /// A value the engine computed: a count, a coordinate, a path we resolved.
    Ours(String),
    /// Bytes from an I/O edge, already through the display seat.
    Foreign(ForeignText),
    /// A whole prose-component read out of the arrangement registry — a typed reason's sentence
    /// (`28L:rul-reason-enums-not-sibling-codes`).
    ///
    /// A third class rather than a flavour of [`Ours`](ParamText::Ours) because these bytes ALREADY
    /// have an authoring home: they are registry words, so where an edit to them belongs is the
    /// entry, never the register that interpolated them. Carrying the component's identity to the
    /// render seat is what lets it say so (`28L:rul-empty-registers-for-pure-holes`).
    Component(crate::arrangement::ComponentText),
}

impl ParamText {
    /// The bytes to substitute into the hole.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            ParamText::Ours(text) => text,
            ParamText::Foreign(foreign) => foreign.as_str(),
            ParamText::Component(component) => component.text(),
        }
    }

    /// Whether these bytes are somebody else's — the render's cue to stamp an un-editable class.
    /// True only for values minted from a [`ForeignBytes`], which is the whole point.
    #[must_use]
    pub fn is_foreign(&self) -> bool {
        matches!(self, ParamText::Foreign(_))
    }
}

impl From<String> for ParamText {
    fn from(text: String) -> Self {
        ParamText::Ours(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The seal's whole claim: what comes out of a `ForeignBytes` has been through the display
    /// seat, whichever sink asked for it.
    #[test]
    fn neither_sink_can_hand_back_raw_bytes() {
        let bytes = ForeignBytes::from_io_edge("red \u{1b}[31m alert\ttab");
        assert_eq!(
            bytes.on_measured_sink(200).as_str(),
            "red \\x1b[31m alert\\x09tab"
        );
        assert_eq!(bytes.on_plain_sink(200).as_str(), "red  [31m alert tab");
    }

    /// An OS error is foreign because of where it came from, and the type is what records that —
    /// the words themselves look like ordinary English.
    #[test]
    fn an_os_error_is_foreign_by_its_edge_not_its_shape() {
        let err = std::io::Error::new(std::io::ErrorKind::IsADirectory, "Is a directory");
        let param = ParamText::Foreign(ForeignBytes::from_os_error(&err).on_plain_sink(64));
        assert!(param.is_foreign());
        assert_eq!(param.text(), "Is a directory");
    }

    /// A sentence we composed carries the ours class no matter what the hole is called — the
    /// property the `detail`-name heuristic could not express.
    #[test]
    fn our_own_words_are_never_foreign() {
        let ours = ParamText::from("the run nonce is not a usable marker".to_owned());
        assert!(!ours.is_foreign());
    }

    /// A debug dump is not a display surface, so it must not become one by accident.
    #[test]
    fn a_debug_dump_carries_no_terminal_escape() {
        let dumped = format!("{:?}", ForeignBytes::from_io_edge("\u{1b}[2J"));
        assert!(!dumped.contains('\u{1b}'), "{dumped}");
    }
}
