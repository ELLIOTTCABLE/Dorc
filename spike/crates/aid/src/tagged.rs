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
    /// Foreign payload text from a passthrough hole.
    ForeignText {
        /// Rendered bytes, which may be empty.
        text: String,
        /// The declared param name the hole interpolated.
        param: &'static str,
    },
    /// Render-owned structure around catalog fields: bytes the renderer computed itself, and
    /// therefore never an edit region.
    Arrangement {
        /// Rendered bytes.
        text: String,
        /// What structure this run is.
        slug: &'static str,
    },
    /// Prose the renderer pulled from the ARRANGEMENT REGISTRY
    /// (`289:rul-arrangement-home-is-registry-plus-transcripts`) — chrome with an editable
    /// face. Only [`crate::arrangement::push_arrangement_words`] mints this, so the bytes and
    /// the registry entry can never disagree.
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
}

impl RenderPart {
    /// The part's rendered bytes.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            RenderPart::TemplateLiteral { text, .. }
            | RenderPart::ParamValue { text, .. }
            | RenderPart::ForeignText { text, .. }
            | RenderPart::Arrangement { text, .. }
            | RenderPart::ArrangementWords { text, .. } => text,
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
