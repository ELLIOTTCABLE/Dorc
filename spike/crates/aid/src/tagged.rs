//! Core-owned render-part vocabulary (`282` §4).

/// Which catalog prose field a [`RenderPart`] came from (`282` §4). The render
/// fills exactly these two registers.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Field {
    /// The primary [`crate::catalog::CatalogEntry::message`] register.
    Message,
    /// The optional [`crate::catalog::CatalogEntry::help`] register.
    Help,
}

/// One ordered run of a diagnostic render.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderPart {
    /// Catalog-authored prose from one field occurrence.
    TemplateLiteral {
        /// Rendered bytes.
        text: String,
        /// The code slug the field belongs to.
        code: &'static str,
        /// Which prose register.
        field: Field,
        /// Zero-based paragraph index within the field.
        paragraph: usize,
        /// The occurrence of this `(code, field)` within the render.
        instance: usize,
    },
    /// A typed payload value from one catalog hole.
    ParamValue {
        /// Rendered bytes, which may be empty.
        text: String,
        /// The code slug the field belongs to.
        code: &'static str,
        /// Which prose register the hole sits in.
        field: Field,
        /// The declared param name the hole interpolated.
        param: &'static str,
        /// The occurrence of this `(code, field)` within the render.
        instance: usize,
    },
    /// Foreign payload text: bytes that are not ours, and never an edit region.
    ///
    /// Typed rather than a `String` (`282:rul-passthrough-type-gated`): an enum variant's fields
    /// are effectively public, so while these bytes were a `String` any literal in this repo could
    /// construct one and our own sentence would arrive wearing the not-ours badge. A
    /// [`ForeignText`](crate::ForeignText) can only be reached from an I/O edge, through the
    /// display seat.
    ForeignText {
        /// Rendered bytes, which may be empty.
        text: crate::ForeignText,
        /// Where the bytes came from — the declared param name on the catalog path, the file
        /// they were quoted out of on the weft path. Runtime-owned because the weft path's
        /// answer is a path a book named, not a name a catalog row declared
        /// (`_w4-map-DRAFT:friction-foreign-key-shape-mismatch`).
        source: String,
    },
    /// Render-owned structure around catalog fields: bytes the renderer computed itself, and
    /// therefore never an edit region.
    Arrangement {
        /// Rendered bytes.
        text: String,
        /// What structure this run is.
        slug: &'static str,
    },
    /// One run of a chrome LINE the renderer pulled from the ARRANGEMENT REGISTRY
    /// (`289:rul-arrangement-home-is-registry-plus-transcripts`) — chrome with an editable
    /// face. Only [`crate::arrangement::push_arrangement_sentence`] and the weft bridge
    /// (`crate::weave::to_render_parts`) mint this, so the bytes and the registry entry can
    /// never disagree.
    ArrangementWords {
        /// Rendered bytes.
        text: String,
        /// The registry key's arrangement slug.
        slug: &'static str,
        /// Which occurrence of `slug` this span is, when the seat renders the slug more than
        /// once and wants per-position entries. `None` ⇒ one entry serves every occurrence.
        /// The stamping is ALL-OR-NOTHING per slug within one render.
        occurrence: Option<usize>,
    },
    /// A computed value interleaved INSIDE a chrome line, at its positional index within that
    /// line's `words[0] values[0] words[1] …` sentence.
    ///
    /// Unlike [`RenderPart::Arrangement`] it does NOT close the section it sits in: a chrome
    /// line is one editable SECTION whose fragments alternate prose and value, and nothing
    /// splits one line across sections (`28H` ruling 3). Never editable itself — rewriting a
    /// value would be lying about the world rather than rephrasing a sentence.
    ArrangementValue {
        /// Rendered bytes.
        text: String,
        /// The registry key's arrangement slug — the row this value was interleaved into.
        slug: &'static str,
        /// The occurrence of `slug` this value's line is.
        occurrence: Option<usize>,
        /// The value's position in the line: `values[index]`.
        index: usize,
    },
}

impl RenderPart {
    /// The part's rendered bytes.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            RenderPart::ForeignText { text, .. } => text.as_str(),
            RenderPart::TemplateLiteral { text, .. }
            | RenderPart::ParamValue { text, .. }
            | RenderPart::Arrangement { text, .. }
            | RenderPart::ArrangementWords { text, .. }
            | RenderPart::ArrangementValue { text, .. } => text,
        }
    }
}

/// An ordered diagnostic render that preserves zero-width parameter occurrences.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RenderParts(Vec<RenderPart>);

impl RenderParts {
    /// Construct an empty part stream.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one ordered run.
    pub fn push(&mut self, part: RenderPart) {
        self.0.push(part);
    }

    /// Append another ordered stream.
    pub fn append(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    /// Construct from ordered runs a test or fixture already holds.
    #[cfg(test)]
    #[must_use]
    pub fn from_parts(parts: Vec<RenderPart>) -> Self {
        Self(parts)
    }

    /// The ordered runs, including empty parameter values.
    #[must_use]
    pub fn parts(&self) -> &[RenderPart] {
        &self.0
    }

    /// Concatenate the rendered bytes.
    #[must_use]
    pub fn text(&self) -> String {
        self.0.iter().map(RenderPart::text).collect()
    }
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
