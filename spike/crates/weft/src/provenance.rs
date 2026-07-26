//! Where every emitted byte came from.
//!
//! Weft's output is not a string; it is a string *plus an accounting*. Rendering
//! flattens a tree into bytes, and flattening is exactly the operation that
//! destroys attribution — the diagnosis behind `28E` §1, where prose was found
//! flattened to bytes earlier than its last consumer needed. So the renderer
//! keeps the accounting: every byte it emits belongs to exactly one [`Span`],
//! and the spans concatenate back to the bytes.
//!
//! The vocabulary here is a deliberate generic mirror of the region vocabulary
//! the sibling transport crate uses, with every consumer-specific identity
//! replaced by an opaque key. Weft never inspects a key, never compares two
//! keys, and never mints one.

/// Which occurrence of a repeated key a run came from.
///
/// One render may emit the same catalog row several times — the same tier verb
/// on six chain rows, the same label on five structural rows. The key alone
/// cannot tell those apart, so an edit round-tripping through the span map
/// cannot know which occurrence it is editing. The instance disambiguates.
/// Numbering is the consumer's, not weft's: weft copies it through untouched.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Instance(pub u32);

/// The authorship of a run of output text, over opaque consumer-supplied keys.
///
/// Four authorship classes, because four different things may be done with a
/// byte. Template text is the consumer's own prose and may be rewritten by a
/// human. Param text is an interpolated value and may not — rewriting it would
/// be lying about the world. Foreign text is *not ours at all* (inlined oracle
/// arms, an author's comments, as-shipped guard sh: `28G` §0's foreign-text
/// class), which is both un-editable and the material an output edge must
/// encode before display (`28D:must-encode-per-surface`). Arrangement text is
/// skeleton — connectives, section words, and the layout weft itself mints.
///
/// A consumer typically supplies one key type spanning every identity it needs
/// (rows, template fields, parameter names, foreign sources); weft imposes no
/// structure on it, so keeping those namespaces straight is the consumer's job.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Provenance<K> {
    /// Consumer prose from a template row: which row, which field of it, which
    /// occurrence.
    Template {
        /// The template row's identity.
        key: K,
        /// Which field of that row (a message, a help line, a title).
        field: K,
        /// Which occurrence of that field in this render.
        instance: Instance,
    },
    /// An interpolated value: which row it was interpolated into, which
    /// parameter of that row, which occurrence.
    Param {
        /// The template row the value was interpolated into.
        key: K,
        /// Which parameter of that row.
        param: K,
        /// Which occurrence of that parameter in this render.
        instance: Instance,
    },
    /// Bytes that are not ours: inlined source, author comments, captured
    /// output. Never editable, always an encoding obligation at the edge.
    Foreign {
        /// What the bytes were taken from.
        key: K,
    },
    /// Skeleton. `Some` is a consumer-supplied arrangement row (a connective, a
    /// section word, a label). `None` is weft's own layout: indentation,
    /// padding, line breaks, column separators, and the handful of structural
    /// glyphs weft mints.
    ///
    /// The `None` case is load-bearing rather than a shrug — it is how a
    /// consumer mechanically tells its own vocabulary apart from the
    /// renderer's, so a round-trip never mistakes weft's punctuation for
    /// editable prose.
    Arrangement {
        /// The arrangement row, or `None` for renderer-minted layout.
        key: Option<K>,
    },
}

/// A run of text with its authorship — the leaf of every node in the tree.
///
/// Runs carry their own inter-word spacing; weft inserts no spaces between
/// adjacent runs. That keeps every space byte attributable to somebody, and it
/// means a consumer that wants a space to belong to a particular row can simply
/// put it there.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Run<K> {
    /// The text itself. Printable ASCII, per the crate's ASCII contract.
    pub text: String,
    /// Who authored it.
    pub provenance: Provenance<K>,
}

impl<K> Run<K> {
    /// Constructs a run with explicit authorship.
    #[must_use]
    pub fn new(text: impl Into<String>, provenance: Provenance<K>) -> Self {
        Self {
            text: text.into(),
            provenance,
        }
    }

    /// Constructs consumer prose from a template field.
    #[must_use]
    pub fn template(text: impl Into<String>, key: K, field: K, instance: Instance) -> Self {
        Self::new(
            text,
            Provenance::Template {
                key,
                field,
                instance,
            },
        )
    }

    /// Constructs an interpolated value.
    #[must_use]
    pub fn param(text: impl Into<String>, key: K, param: K, instance: Instance) -> Self {
        Self::new(
            text,
            Provenance::Param {
                key,
                param,
                instance,
            },
        )
    }

    /// Constructs not-ours bytes taken from `key`.
    #[must_use]
    pub fn foreign(text: impl Into<String>, key: K) -> Self {
        Self::new(text, Provenance::Foreign { key })
    }

    /// Constructs a consumer-supplied arrangement word.
    #[must_use]
    pub fn arrangement(text: impl Into<String>, key: K) -> Self {
        Self::new(text, Provenance::Arrangement { key: Some(key) })
    }
}

/// One contiguous stretch of rendered output, attributed.
///
/// Spans are emitted in output order, are contiguous, and cover the whole
/// output: `spans` concatenated by `text[start..start + len]` reproduces the
/// rendered bytes exactly, with nothing shared and nothing left over. That
/// total-cover property is the crate's central promise and is pinned by
/// property test rather than by documentation.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Span<K> {
    /// Who authored the bytes.
    pub provenance: Provenance<K>,
    /// Byte offset of the first byte, into the rendered text.
    pub start: usize,
    /// Length in bytes.
    pub len: usize,
}

impl<K> Span<K> {
    /// The byte offset one past the span's last byte.
    #[must_use]
    pub fn end(&self) -> usize {
        self.start.saturating_add(self.len)
    }
}
