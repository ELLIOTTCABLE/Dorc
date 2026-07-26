//! `weft` — a firewalled ASCII layout engine for structured explanations.
//!
//! Weft takes a tree of marked nodes and a box, and returns text plus a
//! total-cover map of where every emitted byte came from. It knows nothing
//! about what it is explaining.
//!
//! # The dependency law
//!
//! **engine → adapter → weft. One direction, permanently.**
//!
//! The engine that decides things must never learn what its output looks like,
//! and weft must never learn what the engine's types mean. An adapter in the
//! middle translates decisions into this vocabulary. Concretely, for the
//! consumer this crate was cut for: nothing here may name a Dorc type, and
//! `dorc-core` may never depend on weft.
//!
//! The rule exists because the failure it prevents is silent and expensive.
//! Once printing can reach into engine logic, layout convenience starts
//! deciding what the engine computes — a column that is awkward to render
//! becomes a fact that stops being derived. The firewall keeps that
//! impossible rather than merely discouraged (`28E:rul-tree-render-is-a-
//! firewalled-crate`), and forces the engine to surface well-structured,
//! homogeneous, totalistic data instead of pre-formatted strings.
//!
//! Two consequences worth stating outright:
//!
//! - **The semantics engine MARKS; the renderer RULES**
//!   (`28E:rul-renderer-owns-layout`). A consumer says "this is critical", "this
//!   is a quotation", "this source is descriptive, not runnable". It never says
//!   how wide, how deep, or in what column. There is deliberately nowhere in
//!   [`tree`] to express a layout decision.
//! - **Weft mints no words.** Every word in the output is a consumer-supplied
//!   [`provenance::Run`]. Weft contributes whitespace and a short fixed set of
//!   structural glyphs, all stamped [`provenance::Provenance::Arrangement`] with
//!   a `None` key — so a consumer can mechanically tell its own vocabulary from
//!   the renderer's and never mistake punctuation for editable prose.
//!
//! # What it promises
//!
//! - **Layout is a pure function of (tree, frame)** (`28E:prop-pure-layout-
//!   function`): no clock, no randomness, no filesystem, no carried state.
//!   Resize is recompute. Deterministic enough to golden and to fuzz.
//! - **Total cover**: [`render`] returns spans that are contiguous, in output
//!   order, and reproduce the text exactly when concatenated. Every byte is
//!   attributable; nothing is shared and nothing is orphaned.
//! - **Printable ASCII, forever** (`28E:rul-ascii-output-forever`). This is not
//!   only taste: the contract makes a byte and a column the same thing, which is
//!   what lets the box model do arithmetic instead of grapheme measurement. Weft
//!   does not sanitise consumer text — encoding belongs at the output edge, per
//!   surface — so non-ASCII input is a contract violation that will mis-measure.
//! - **Content is never dropped to satisfy a width.** A word too wide for its
//!   line overruns; a literal source line is never rewrapped. Omission is always
//!   a visible, consumer-authored act ([`tree::Truncation`]).
//!
//! # The box model
//!
//! Geometry is a [`frame::Frame`]: a left edge, a right edge, and per-line
//! [`frame::Reservation`]s. Nesting, hanging indents, column layouts, and
//! eventual floats are one mechanism, because the shapes this serves nest and
//! float — an annotation set beside a code excerpt, outside it, narrowing that
//! excerpt for exactly the lines it sits beside. Declaring reservations is here;
//! *solving* for float placement is not.
//!
//! Crucially the descent into code is **two-way, not a clean one-way leaf**.
//! Formatted source has boxes inside it: a run of trailing comments aligned
//! across several lines is itself a column that wraps within itself, aligns with
//! its neighbours, and takes its own styling. So a [`tree::CodeLine`] is made of
//! [`tree::CodeCell`]s that a block measures into columns by the same rule a run
//! of speaker rows uses, and the eventual sh formatter emits cells rather than
//! inventing a second layout system underneath this one. Styling interleaves the
//! same way: a token class is just another provenance key on a run inside a
//! cell inside a box, so highlighting never needs a parallel tree.
//!
//! # Seams — what is deliberately absent
//!
//! Each of these is a known next piece, not an oversight:
//!
//! - **The sh formatter with teeth.** [`tree::Literalness::Formatted`] promises
//!   correctness-preserving rewrapping that stays valid sh. Weft has no grammar,
//!   so `Formatted` renders exactly as `Literal` today. The mode is carried so
//!   the distinction is already in the data when the formatter lands.
//! - **Deciding what to elide.** [`tree::Literalness::Descriptive`] is rendered
//!   and marked non-runnable, but weft never chooses what to ellipsize; the
//!   consumer supplies already-shortened lines. That opinion is derived from
//!   what a decision actually read, which is engine knowledge.
//! - **Syntax highlighting and ANSI colour.** Colour is layout-tier and
//!   edge-owned: the span map is the hook, so an edge can style by provenance
//!   without weft gaining a styling vocabulary. There is no glyph-set axis —
//!   ASCII is not a mode here.
//! - **Document-algebra reflow.** The filler is first-fit with no lookahead. The
//!   Oppen/Wadler optimal-break machinery swaps in under the filler without the
//!   node layouts noticing.
//! - **Incremental layout and any TUI.** Resize recomputes. A resize-responsive
//!   surface is a live design input, but incrementality is not the answer to it.
//! - **Selection.** [`tree::Criticality`] and [`tree::Register`] are recorded
//!   and ruled on in no way — nothing is dropped, shortened, or reordered by
//!   them. Marking can start before the renderer learns to select.
//!
//! # Known tension, flagged rather than resolved
//!
//! The consuming project holds a law that every displayed string is born as a
//! catalog or arrangement row, never a formatting literal. Weft mints a small
//! set of structural glyphs itself — section rules, gutter separators, quotes,
//! the truncation lead. They are honestly stamped (`Arrangement { key: None }`),
//! so they are always distinguishable, but they are weft's bytes and not a
//! consumer row. Whether the consumer should instead supply them as a skeleton
//! of arrangement runs is a cross-cutting call for the orchestrator, not one to
//! settle inside this crate. The change would be additive.

mod align;
pub mod frame;
pub mod provenance;
pub mod render;
mod sink;
pub mod tree;
mod wrap;

pub use frame::{Frame, Reservation, Side, Width};
pub use provenance::{Instance, Provenance, Run, Span};
pub use render::{Rendered, render, render_framed};
pub use tree::{
    Banner, Branch, CodeBlock, CodeCell, CodeLine, Criticality, Document, Join, LabeledRow,
    Literalness, Mark, Node, NodeKind, Paragraph, Payload, Placement, PointerLine, Quoting,
    Register, Section, SpeakerRow, Truncation,
};
