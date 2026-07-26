//! The node vocabulary: what a consumer hands weft to lay out.
//!
//! The vocabulary is generic over an opaque key `K` and knows nothing about any
//! particular consumer's domain. It is cut to carry the shapes a decision-report
//! actually needs — quoted speaker rows, code excerpts at three literalness
//! modes, labelled structural rows, joins, truncation — without naming what any
//! of them mean.
//!
//! Two marks ride on every node, and the split between them is the crate's
//! governing rule (`28E:rul-renderer-owns-layout`): the semantics engine MARKS,
//! the renderer RULES. A consumer says "this is critical" and "this is the full
//! register"; it never says "indent this by six" or "put this in a column".
//! Layout decisions that leak into a semantics engine are the failure this
//! separation exists to prevent, so there is deliberately nowhere in this
//! vocabulary to express one.

use crate::provenance::Run;

/// How much of a node's material is being shown.
///
/// One variant today, and that is the whole point: `kTASTE`'s data-model law
/// says the model retains both the welded synthesis and the narrative residue,
/// with the selection metadata relating them, and that registers are two goals
/// crossed with densities rather than one axis. None of that is built. The slot
/// exists so the eventual answer lands as a node property — where
/// `28E:prop-register-per-node` puts it, since truncation must be legal at any
/// link — rather than as a surface-wide flag that would have to be unpicked.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Register {
    /// Everything the node holds.
    #[default]
    Full,
}

/// Whether a node may be summarised away.
///
/// The mark is semantics; acting on it is layout. Weft records it and, today,
/// rules on it in exactly no way — no node is dropped, shortened, or reordered
/// by it. Marking can therefore start before the renderer learns to select,
/// which is the sequencing the separation buys.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Criticality {
    /// Must survive any selection.
    #[default]
    Critical,
    /// May be summarised or dropped when a narrower register is chosen.
    Summarizable,
}

/// The marks every node carries.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Mark {
    /// How much of the node is being shown.
    pub register: Register,
    /// Whether it may be summarised away.
    pub criticality: Criticality,
}

/// A node: its marks, and what it is.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node<K> {
    /// The semantics engine's marks.
    pub mark: Mark,
    /// The node's shape and content.
    pub kind: NodeKind<K>,
}

impl<K> Node<K> {
    /// Constructs a node, marked critical at the full register.
    ///
    /// Critical is the default because the surface this serves may never hide
    /// something that could bite: attention is saved by provable elision, never
    /// by quiet omission. A consumer opts material *out* of prominence, never
    /// into it.
    #[must_use]
    pub fn new(kind: NodeKind<K>) -> Self {
        Self {
            mark: Mark::default(),
            kind,
        }
    }

    /// Marks the node summarisable.
    #[must_use]
    pub fn summarizable(mut self) -> Self {
        self.mark.criticality = Criticality::Summarizable;
        self
    }

    /// Sets the node's register.
    #[must_use]
    pub fn at_register(mut self, register: Register) -> Self {
        self.mark.register = register;
        self
    }
}

/// What a node is.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NodeKind<K> {
    /// A titled division of the document.
    Section(Section<K>),
    /// A quoted-speaker row: somebody said something, and this is who and what.
    Speaker(SpeakerRow<K>),
    /// An excerpt of source, at one of three literalness modes.
    Code(CodeBlock<K>),
    /// A labelled structural row.
    Labeled(LabeledRow<K>),
    /// A cross-reference the reader can act on.
    Pointer(PointerLine<K>),
    /// A framing headline with an indented body.
    Banner(Banner<K>),
    /// A paragraph of prose.
    Prose(Paragraph<K>),
    /// Several branches meeting at one consequence.
    Join(Join<K>),
    /// A visible mark that material was left out.
    Truncation(Truncation<K>),
}

/// A titled division: a header, optional counts, and a body at the same depth.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Section<K> {
    /// The header words.
    pub header: Vec<Run<K>>,
    /// A tally rendered alongside the header, if the consumer has one.
    pub counts: Option<Vec<Run<K>>>,
    /// The section's contents.
    pub body: Vec<Node<K>>,
}

/// A quoted-speaker row.
///
/// The shape encodes that weft's consumer asserts nothing in its own voice: a
/// row names WHO spoke, the tier-word that IS the sentence's verb, and the
/// payload they said. Rendering keeps those in columns so a reader can scan the
/// speakers, the verbs, or the payloads independently.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpeakerRow<K> {
    /// A one-glyph rank mark in the leftmost slot, if the row carries one.
    pub gutter: Option<Run<K>>,
    /// Who spoke — a source locus, a line reference, a tool name.
    pub speaker: Vec<Run<K>>,
    /// The tier-word acting as the sentence's verb, if the row has one.
    pub verb: Option<Vec<Run<K>>>,
    /// What was said.
    pub payload: Payload<K>,
    /// Material hanging below the row: explanation, an excerpt of the source.
    pub attachments: Vec<Node<K>>,
    /// An alignment group: rows sharing this key share their mark, speaker and
    /// verb column widths, even when an attachment or a section boundary sits
    /// between them.
    pub align: Option<K>,
}

/// A speaker row's payload, and whether it is a verbatim quotation.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Payload<K> {
    /// Whether the payload is quoted material.
    pub quoting: Quoting,
    /// The payload words.
    pub runs: Vec<Run<K>>,
    /// Material about the *speaking*, not the thing said — when a check ran and
    /// what it exited with. It sits outside the quotation, because attributing
    /// it to the speaker would put words in their mouth: they said the payload,
    /// not the circumstances under which they were heard.
    pub trailer: Vec<Run<K>>,
}

/// Whether a payload is a verbatim quotation or the consumer's own prose.
///
/// A mark, not a layout instruction: it says what the payload *is*, and the
/// renderer decides how quotation is shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Quoting {
    /// A verbatim quotation.
    Quoted,
    /// Ordinary prose.
    Bare,
}

/// How literally shown source is being shown.
///
/// The three modes are `28E:prop-three-literalness-modes`, and the reason they
/// are a *rendered property* rather than a formatting detail is the
/// display-never-masquerades-as-runnable law: a reader must be able to tell, at
/// a glance, whether the sh in front of them would actually run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Literalness {
    /// Byte-honest. Never rewrapped, even past the right edge — overrunning the
    /// box is preferable to implying bytes that are not there.
    Literal,
    /// Rewrapped but still valid and runnable, breaking only where the
    /// consumer's own grammar licenses a break.
    Formatted,
    /// Ellipsized and non-runnable. Wrapped freely, and marked so it cannot be
    /// mistaken for something that would execute.
    Descriptive,
}

/// An excerpt of source.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CodeBlock<K> {
    /// How literally the lines are being shown.
    pub mode: Literalness,
    /// A label naming where the excerpt came from, rendered above it.
    pub locus: Option<Vec<Run<K>>>,
    /// The lines themselves.
    pub lines: Vec<CodeLine<K>>,
    /// An alignment group for the gutter column.
    ///
    /// Needed alongside the per-cell groups, not instead of them: sharing cell
    /// widths only lines two blocks up if their gutters are the same width too,
    /// and a two-digit line number in one excerpt would otherwise shunt all its
    /// columns one place right of the other's.
    pub align: Option<K>,
}

/// One line of an excerpt: a gutter cell and one or more content cells.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CodeLine<K> {
    /// The gutter cell — a line number, usually. Widths are derived from the
    /// block's contents, never supplied.
    pub gutter: Option<Run<K>>,
    /// The line's content cells, left to right.
    pub cells: Vec<CodeCell<K>>,
}

/// One cell of a code line, and a column the whole block shares.
///
/// Code is not a leaf, and treating it as one is the simplification that bites.
/// Real formatted source has boxes inside it — a run of trailing comments
/// aligned across several lines is a column that wraps within itself, aligns
/// with its neighbours above and below, and carries its own styling. So the
/// descent from box model into code is two-way: a block measures its cells into
/// columns exactly as a run of speaker rows does, and a future sh formatter
/// emits cells rather than having to invent a second layout system underneath
/// this one.
///
/// Splitting a line into cells is also the consumer's act of *licensing*
/// alignment padding: a one-cell line is emitted with its bytes untouched,
/// while a multi-cell line has asked for its columns to be squared up. That
/// keeps byte-honesty a property the consumer controls rather than one the
/// renderer quietly spends.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CodeCell<K> {
    /// The cell's text, which for real source is foreign.
    pub runs: Vec<Run<K>>,
    /// An alignment group: cells sharing this key share a column width across
    /// block boundaries, so two excerpts' trailing comments can line up without
    /// being siblings.
    pub align: Option<K>,
}

impl<K> CodeCell<K> {
    /// Constructs an ungrouped cell, aligned only within its own block.
    #[must_use]
    pub fn new(runs: Vec<Run<K>>) -> Self {
        Self { runs, align: None }
    }

    /// Puts the cell in a named alignment group.
    #[must_use]
    pub fn aligned(mut self, group: K) -> Self {
        self.align = Some(group);
        self
    }
}

/// A labelled structural row: a label, a body hanging under it, and optional
/// attached material.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LabeledRow<K> {
    /// The label.
    pub label: Vec<Run<K>>,
    /// The row's body.
    pub body: Vec<Run<K>>,
    /// Material hanging below the row.
    pub attachments: Vec<Node<K>>,
    /// An alignment group: rows sharing this key share one label column,
    /// wherever they sit. The remediation rows of a report are the motivating
    /// case — they read as one list even when a join splits them across
    /// branches, and a list whose labels do not line up does not read as one.
    pub align: Option<K>,
}

/// Where a pointer line sits relative to what precedes it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Placement {
    /// On its own line, at the box's left edge.
    Standalone,
    /// Set to the right, sharing the preceding line where it fits.
    Trailing,
}

/// A cross-reference the reader can act on.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PointerLine<K> {
    /// Where it sits.
    pub placement: Placement,
    /// The reference itself. It must stay copy-paste-true, so it is never
    /// wrapped.
    pub target: Vec<Run<K>>,
}

/// A framing headline with an indented body.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Banner<K> {
    /// The headline.
    pub headline: Vec<Run<K>>,
    /// Material indented beneath it.
    pub body: Vec<Node<K>>,
}

/// A paragraph of prose.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Paragraph<K> {
    /// The words.
    pub runs: Vec<Run<K>>,
}

/// Several branches meeting at one consequence.
///
/// Born this shape on purpose. Today's chains are linear with at most one join,
/// and a linear renderer would serve them — but numbering that forces a false
/// total order is expensive to retrofit once anything depends on it
/// (`28E:nit-why-steps-are-a-dag`), so the vocabulary is DAG-shaped from the
/// start even though the layout it receives is not yet.
///
/// The restatement is the load-bearing half: a join that merely converges leaves
/// the reader to reconstruct what it converged *to*.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Join<K> {
    /// The branches, in the order the consumer wants them read — which is a
    /// tunable seam, never semantic.
    pub branches: Vec<Branch<K>>,
    /// Prose restating what the branches together establish.
    pub restatement: Option<Paragraph<K>>,
}

/// One branch of a join.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Branch<K> {
    /// A connective introducing the branch, if it has one. Connectives are the
    /// consumer's vocabulary, never weft's.
    pub connective: Option<Vec<Run<K>>>,
    /// The branch's contents.
    pub nodes: Vec<Node<K>>,
}

/// A visible mark that material was left out.
///
/// Everything the reader sees about what is missing — how much, and how to ask
/// for it — comes from the note, because weft mints no words. Omission is
/// therefore always attributable to the consumer that chose it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Truncation<K> {
    /// What was left out and how to see it.
    pub note: Vec<Run<K>>,
}

/// A whole document: a sequence of nodes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Document<K> {
    /// The document's nodes.
    pub nodes: Vec<Node<K>>,
}

impl<K> Document<K> {
    /// Constructs a document.
    #[must_use]
    pub fn new(nodes: Vec<Node<K>>) -> Self {
        Self { nodes }
    }
}
