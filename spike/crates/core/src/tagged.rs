//! Core-owned span types for the tagged render (`282` §4): the render seat emits,
//! alongside the bytes, a map classifying every output run as exactly one
//! [`Region`]. Dorc core OWNS this vocabulary and takes NO dependency on
//! `errorloom` (`28A` §1 — kernel-dep-cleanliness); the `dorc-loom` adapter maps
//! these 1:1 onto `errorloom`'s generic span schema, where the promote flow lives.
//!
//! The four region kinds mirror `errorloom`'s. `TemplateLiteral`/`ParamValue`
//! carry an occurrence `instance`, stamped ALWAYS by the emitter — the per-key
//! all-or-nothing floor (`28A:rul-tagged-render-emits-instance-ids`). The gap-free
//! total-cover contract (`28A:rul-span-cover-stays-total`) is enforced downstream
//! by `errorloom::TaggedRender::new`; this core type is the plain carrier.

use std::ops::Range;

/// Which catalog prose field a template run came from (`282` §4 — the `field` of
/// [`Region::TemplateLiteral`]/[`Region::ParamValue`]). The render fills exactly
/// these two registers.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Field {
    /// The primary [`crate::catalog::CatalogEntry::message`] register.
    Message,
    /// The optional [`crate::catalog::CatalogEntry::help`] register.
    Help,
}

impl Field {
    /// The field's stable slug — the discriminator the adapter folds into its key.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Field::Message => "message",
            Field::Help => "help",
        }
    }
}

/// The classification of one byte-run of a tagged render (`282` §4). Exhaustive by
/// design: the adapter matches every arm, so a new kind is a compile error there.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Region {
    /// The code's own prose words for one paragraph of one field — the only
    /// prose-editable class.
    TemplateLiteral {
        /// The code slug the field belongs to.
        code: &'static str,
        /// Which prose register.
        field: Field,
        /// Zero-based paragraph index within the field.
        paragraph: usize,
        /// The occurrence of this `(code, field)` within the render (stamped
        /// always).
        instance: usize,
    },
    /// Interpolated payload for a declared `param` — data, not prose.
    ParamValue {
        /// The code slug the field belongs to.
        code: &'static str,
        /// Which prose register the hole sits in.
        field: Field,
        /// The declared param name the hole interpolated.
        param: &'static str,
        /// The occurrence of this `(code, field)` within the render (stamped
        /// always).
        instance: usize,
    },
    /// Passthrough foreign text riding a `detail`-style hole
    /// (`282:rul-passthrough-type-gated`): tainted bytes, never our prose.
    ForeignText {
        /// The hole the foreign text rode.
        param: &'static str,
    },
    /// Render-owned structure: connectives, blank structure, placeholders.
    Arrangement {
        /// What structure this run is.
        slug: &'static str,
    },
}

/// One classified byte-run of a tagged render.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span {
    /// The byte range within the render this span covers.
    pub range: Range<usize>,
    /// How the run is classified.
    pub region: Region,
}

/// A rendered string paired with its span map (`282` §4). The plain carrier the
/// render seat emits; the gap-free total-cover check is the adapter's
/// `errorloom::TaggedRender::new`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TaggedRender {
    text: String,
    spans: Vec<Span>,
}

impl TaggedRender {
    /// Bundle a render with its span map.
    #[must_use]
    pub fn new(text: String, spans: Vec<Span>) -> Self {
        TaggedRender { text, spans }
    }

    /// The rendered text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The span map, in output order.
    #[must_use]
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }
}
