//! The pure layout function.
//!
//! `render` is a pure function of `(tree, frame)` — no clock, no randomness, no
//! filesystem, no interior state surviving a call (`28E:prop-pure-layout-
//! function`). Resizing is recomputation, never incremental reflow, which is
//! what keeps the whole surface deterministic enough to golden and to fuzz, and
//! what structurally excludes the reflow-fixpoint problem the browser-class
//! layout engines fight.
//!
//! Rendering is the second of two passes. [`measure`](crate::measure) walks the
//! tree first, carrying each node's left edge, and resolves every table's column
//! stops; this pass then lays out against those stops in the same visit order.
//! The passes are an ORDERING, not an iteration — a column's width is its
//! unwrapped content, so nothing here can change a stop.
//!
//! Because the two walks must agree, they are two walks of one shape, and every
//! member checks itself: the layout pass asserts that the left edge it actually
//! has is the one the measure pass predicted (`28F:rul-layout-asserts-measure`).
//! Drift between the walks is otherwise silent, and shows up as columns that are
//! subtly wrong rather than as a failure.
//!
//! Everything in this module reads its geometry from a [`Frame`] and its words
//! from the consumer. Weft mints whitespace and a short, fixed set of structural
//! glyphs, and nothing else — no vocabulary, no numbering, no counts.

use crate::frame::{Frame, Reservation, Side, Width};
use crate::measure::{Placement, are_adjacent_rows, has_attachments, measure};
use crate::provenance::Span;
use crate::sink::Sink;
use crate::tree::{
    Banner, Branch, CodeBlock, CodeLine, Document, Join, LabeledRow, Literalness, Node, NodeKind,
    Paragraph, Placement as PointerPlacement, PointerLine, Quoting, Section, SpeakerRow,
    Truncation,
};
use crate::wrap::{emit_runs, runs_width, wrap};

/// The indentation unit, in columns.
pub(crate) const INDENT: usize = 3;

const SECTION_OPEN: &str = "=== ";
const SECTION_CLOSE: &str = " ===";
const COUNTS_OPEN: &str = " (";
const COUNTS_CLOSE: &str = ")";
const LOCUS_OPEN: &str = "[ ";
const GUTTER_RUNNABLE: &str = " | ";
const GUTTER_DESCRIPTIVE: &str = " ~ ";
const DESCRIPTIVE_BARE: &str = "~ ";
const TRUNCATION_LEAD: &str = "... ";
const QUOTE: &str = "\"";

/// A rendered document: the bytes, and where every one of them came from.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Rendered<K> {
    text: String,
    spans: Vec<Span<K>>,
}

impl<K> Rendered<K> {
    /// The rendered text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The span map, in output order, contiguous and covering the whole text.
    #[must_use]
    pub fn spans(&self) -> &[Span<K>] {
        &self.spans
    }

    /// Consumes the render, yielding both halves.
    #[must_use]
    pub fn into_parts(self) -> (String, Vec<Span<K>>) {
        (self.text, self.spans)
    }
}

/// Lays a document out at a given width.
#[must_use]
pub fn render<K: Clone + PartialEq>(
    document: &Document<K>,
    width: impl Into<Width>,
) -> Rendered<K> {
    render_framed(document, &Frame::of_width(width.into()))
}

/// Lays a document out into an explicit box.
///
/// The entry point for placing a document inside geometry the caller already
/// owns — a column, a pane, a region beside a float.
#[must_use]
pub fn render_framed<K: Clone + PartialEq>(document: &Document<K>, frame: &Frame) -> Rendered<K> {
    let mut painter = Painter {
        sink: Sink::new(),
        measured: measure(document, frame),
        member: 0,
    };
    painter.nodes(&document.nodes, frame);
    let (text, spans) = painter.sink.finish();
    Rendered { text, spans }
}

/// The layout pass: the output sink, the measured tables, and the cursor that
/// keeps this walk aligned with the measure walk.
struct Painter<K> {
    sink: Sink<K>,
    measured: crate::measure::Measured,
    member: usize,
}

/// The gutter separator for a mode, given whether the block has a gutter.
///
/// A descriptive block always carries its marker, gutter or not: the
/// non-runnable mark is load-bearing, so there is no shape in which it can go
/// missing.
pub(crate) fn separator(mode: Literalness, has_gutter: bool) -> &'static str {
    match (mode, has_gutter) {
        (Literalness::Descriptive, true) => GUTTER_DESCRIPTIVE,
        (Literalness::Descriptive, false) => DESCRIPTIVE_BARE,
        (_, true) => GUTTER_RUNNABLE,
        (_, false) => "",
    }
}

impl<K: Clone + PartialEq> Painter<K> {
    /// The next member's resolved table, checked against where layout actually
    /// put it (`28F:rul-layout-asserts-measure`).
    fn take_member(&mut self, left: usize) -> Placement {
        let placement = self.measured.placement(self.member);
        self.member = self.member.saturating_add(1);
        debug_assert_eq!(
            placement.left, left,
            "the measure and layout walks disagree about a member's left edge: \
             the two walks must visit the same members, in the same order, at the same insets"
        );
        placement
    }

    /// Advances to `column`, always leaving at least one space if the line has
    /// content — so an over-wide column never runs its neighbour together.
    fn advance_to(&mut self, column: usize) {
        let minimum = if self.sink.line_is_empty() {
            0
        } else {
            self.sink.column().saturating_add(1)
        };
        self.sink.pad_to(column.max(minimum));
    }

    fn nodes(&mut self, nodes: &[Node<K>], frame: &Frame) {
        for (index, node) in nodes.iter().enumerate() {
            if let Some(previous) = index.checked_sub(1).and_then(|i| nodes.get(i)) {
                self.separate(&previous.kind, &node.kind);
            }
            self.node(&node.kind, frame);
        }
    }

    /// Vertical separation between two nodes.
    ///
    /// A blank line belongs between unlike things. Two exceptions: a run of
    /// same-kind rows reads as one block, and a pointer refers to what precedes
    /// it, so separating it from that would be a lie about what it points at.
    ///
    /// A row whose attachments rendered is no longer tight against its
    /// successor: the material hanging under it has to end somewhere visible, or
    /// the next row reads as part of the attachment. Note that this is a
    /// SPACING judgment only — the interrupted rows still share their table, so
    /// the run resumes squared up after the interruption.
    fn separate(&mut self, previous: &NodeKind<K>, next: &NodeKind<K>) {
        if matches!(next, NodeKind::Pointer(_)) {
            return;
        }
        if are_adjacent_rows(previous, next) && !has_attachments(previous) {
            self.sink.end_line();
            return;
        }
        self.sink.blank_line();
    }

    fn node(&mut self, kind: &NodeKind<K>, frame: &Frame) {
        match kind {
            NodeKind::Section(section) => self.section(section, frame),
            NodeKind::Code(block) => self.code(block, frame),
            NodeKind::Pointer(pointer) => self.pointer(pointer, frame),
            NodeKind::Banner(banner) => self.banner(banner, frame),
            NodeKind::Prose(paragraph) => self.prose(paragraph, frame),
            NodeKind::Join(join) => self.join(join, frame),
            NodeKind::Truncation(truncation) => self.truncation(truncation, frame),
            NodeKind::Speaker(row) => self.speaker_row(row, frame),
            NodeKind::Labeled(row) => self.labeled_row(row, frame),
        }
    }

    fn section(&mut self, section: &Section<K>, frame: &Frame) {
        self.sink.end_line();
        self.sink.pad_to(frame.left());
        self.sink.layout(SECTION_OPEN);
        emit_runs(&mut self.sink, &section.header);
        if let Some(counts) = &section.counts {
            self.sink.layout(COUNTS_OPEN);
            emit_runs(&mut self.sink, counts);
            self.sink.layout(COUNTS_CLOSE);
        }
        self.sink.layout(SECTION_CLOSE);
        if !section.body.is_empty() {
            self.sink.blank_line();
            self.nodes(&section.body, frame);
        }
    }

    fn banner(&mut self, banner: &Banner<K>, frame: &Frame) {
        self.sink.end_line();
        wrap(&mut self.sink, &banner.headline, frame);
        if !banner.body.is_empty() {
            self.sink.end_line();
            self.nodes(&banner.body, &frame.inset(INDENT));
        }
    }

    fn prose(&mut self, paragraph: &Paragraph<K>, frame: &Frame) {
        self.sink.end_line();
        wrap(&mut self.sink, &paragraph.runs, frame);
    }

    fn truncation(&mut self, truncation: &Truncation<K>, frame: &Frame) {
        self.sink.end_line();
        self.sink.pad_to(frame.left());
        self.sink.layout(TRUNCATION_LEAD);
        wrap(
            &mut self.sink,
            &truncation.note,
            &frame.inset(TRUNCATION_LEAD.len()),
        );
    }

    /// A pointer is never wrapped: it must stay copy-paste-true.
    fn pointer(&mut self, pointer: &PointerLine<K>, frame: &Frame) {
        let width = runs_width(&pointer.target);
        let flush_right = frame.right().saturating_sub(width);
        match pointer.placement {
            PointerPlacement::Standalone => {
                self.sink.end_line();
                self.sink.pad_to(frame.left());
            }
            PointerPlacement::Trailing => {
                let fits_beside = !self.sink.line_is_empty() && self.sink.column() < flush_right;
                if !fits_beside {
                    self.sink.end_line();
                }
                self.sink.pad_to(flush_right.max(frame.left()));
            }
        }
        emit_runs(&mut self.sink, &pointer.target);
    }

    /// Branches are separated by the same rule as siblings: alternatives that
    /// are each one row read as one list interrupted by a connective, while
    /// branches carrying blocks need the air.
    fn join(&mut self, join: &Join<K>, frame: &Frame) {
        for (index, branch) in join.branches.iter().enumerate() {
            let previous = index
                .checked_sub(1)
                .and_then(|i| join.branches.get(i))
                .and_then(|branch| branch.nodes.last());
            if let Some(previous) = previous {
                match branch.nodes.first() {
                    Some(first) if are_adjacent_rows(&previous.kind, &first.kind) => {
                        self.sink.end_line();
                    }
                    _ => self.sink.blank_line(),
                }
            }
            self.branch(branch, frame);
        }
        if let Some(restatement) = &join.restatement {
            self.sink.blank_line();
            wrap(&mut self.sink, &restatement.runs, frame);
        }
    }

    fn branch(&mut self, branch: &Branch<K>, frame: &Frame) {
        if let Some(connective) = &branch.connective {
            self.sink.end_line();
            self.sink.pad_to(frame.left());
            emit_runs(&mut self.sink, connective);
        }
        if !branch.nodes.is_empty() {
            self.sink.end_line();
            self.nodes(&branch.nodes, &frame.inset(INDENT));
        }
    }

    fn attachments(&mut self, attachments: &[Node<K>], frame: &Frame) {
        if attachments.is_empty() {
            return;
        }
        self.sink.blank_line();
        self.nodes(attachments, &frame.inset(INDENT));
    }

    fn labeled_row(&mut self, row: &LabeledRow<K>, frame: &Frame) {
        let placement = self.take_member(frame.left());
        self.sink.end_line();
        self.sink.pad_to(placement.stops.stop(0));
        emit_runs(&mut self.sink, &row.label);
        let body_left = if placement.stops.stacked() {
            // A hanging indent charges the label's width to every continuation
            // line; stacking charges one line, once. The stacked indent is the
            // TABLE's, not the member's, so a degraded table stays square.
            self.sink.end_line();
            placement.stops.stop(0).saturating_add(INDENT)
        } else {
            self.advance_to(placement.stops.tail());
            placement.stops.tail()
        };
        wrap(
            &mut self.sink,
            &row.body,
            &frame.inset(body_left.saturating_sub(frame.left())),
        );
        self.attachments(&row.attachments, frame);
    }

    fn speaker_row(&mut self, row: &SpeakerRow<K>, frame: &Frame) {
        let placement = self.take_member(frame.left());
        let stops = &placement.stops.clone();
        self.sink.end_line();
        self.sink.pad_to(frame.left());
        if let Some(gutter) = &row.gutter {
            self.sink.run(&gutter.text, &gutter.provenance);
            self.advance_to(stops.stop(0));
        } else {
            self.sink.pad_to(stops.stop(0));
        }
        emit_runs(&mut self.sink, &row.speaker);

        let payload_left = if stops.stacked() {
            if let Some(verb) = &row.verb {
                let beside = self.sink.column().saturating_add(1);
                self.advance_to(beside);
                emit_runs(&mut self.sink, verb);
            }
            self.sink.end_line();
            stops.stop(0).saturating_add(INDENT)
        } else {
            if let Some(verb) = &row.verb {
                self.advance_to(stops.stop(1));
                emit_runs(&mut self.sink, verb);
            }
            self.advance_to(stops.tail());
            stops.tail()
        };

        self.payload(row, frame, payload_left);
        self.attachments(&row.attachments, frame);
    }

    fn payload(&mut self, row: &SpeakerRow<K>, frame: &Frame, payload_left: usize) {
        let quoted = matches!(row.payload.quoting, Quoting::Quoted);
        let mut payload_frame = frame.inset(payload_left.saturating_sub(frame.left()));
        if quoted {
            // The closing quote must land inside the box, so it is withheld from
            // every line rather than allowed to overrun the last one.
            payload_frame =
                payload_frame.reserving(Reservation::all_lines(Side::Right, QUOTE.len()));
            self.sink.pad_to(payload_left);
            self.sink.layout(QUOTE);
        }
        wrap(&mut self.sink, &row.payload.runs, &payload_frame);
        if quoted {
            self.sink.layout(QUOTE);
        }
        if !row.payload.trailer.is_empty() {
            wrap(
                &mut self.sink,
                &row.payload.trailer,
                &frame.inset(payload_left.saturating_sub(frame.left())),
            );
        }
    }

    /// A code block lays out against its table's stops like any other member,
    /// and ignores the table's stacking decision: source is shown byte-honest
    /// and overruns, so there is no stacked form of it to fall back to.
    fn code(&mut self, block: &CodeBlock<K>, frame: &Frame) {
        let placement = self.take_member(frame.left());
        let stops = placement.stops.clone();
        self.sink.end_line();
        if let Some(locus) = &block.locus {
            self.sink.pad_to(frame.left().saturating_add(INDENT));
            self.sink.layout(LOCUS_OPEN);
            emit_runs(&mut self.sink, locus);
            self.sink.end_line();
        }
        let gutter_width = block
            .lines
            .iter()
            .map(|line| line.gutter.as_ref().map_or(0, |gutter| gutter.text.len()))
            .max()
            .unwrap_or(0);
        let separator = separator(block.mode, gutter_width > 0);
        // Gutters are right-aligned so that the separators of every block in one
        // table land in the same column, whatever their line numbers measure.
        let separator_left = stops.stop(0).saturating_sub(separator.len());

        for line in &block.lines {
            self.sink.end_line();
            self.sink.pad_to(frame.left());
            if let Some(gutter) = &line.gutter {
                self.sink
                    .pad_to(separator_left.saturating_sub(gutter.text.len()));
                self.sink.run(&gutter.text, &gutter.provenance);
            } else {
                self.sink.pad_to(separator_left);
            }
            self.sink.layout(separator);
            self.cells(line, block.mode, frame, &stops);
        }
    }

    fn cells(
        &mut self,
        line: &CodeLine<K>,
        mode: Literalness,
        frame: &Frame,
        stops: &crate::measure::Stops,
    ) {
        for (index, cell) in line.cells.iter().enumerate() {
            let left = stops.stop(index);
            if index > 0 {
                self.advance_to(left);
            }
            match mode {
                // Byte-honest modes never rewrap; an overrun is preferable to
                // implying a line break the source does not contain.
                Literalness::Literal | Literalness::Formatted => {
                    emit_runs(&mut self.sink, &cell.runs);
                }
                Literalness::Descriptive => wrap(
                    &mut self.sink,
                    &cell.runs,
                    &frame.inset(left.saturating_sub(frame.left())),
                ),
            }
        }
    }
}
