//! The pure layout function.
//!
//! `render` is a pure function of `(tree, frame)` — no clock, no randomness, no
//! filesystem, no interior state surviving a call (`28E:prop-pure-layout-
//! function`). Resizing is recomputation, never incremental reflow, which is
//! what keeps the whole surface deterministic enough to golden and to fuzz, and
//! what structurally excludes the reflow-fixpoint problem the browser-class
//! layout engines fight.
//!
//! Everything in this module reads its geometry from a [`Frame`] and its words
//! from the consumer. Weft mints whitespace and a short, fixed set of structural
//! glyphs, and nothing else — no vocabulary, no numbering, no counts.

use crate::frame::{Frame, Reservation, Side, Width};
use crate::provenance::Span;
use crate::sink::Sink;
use crate::tree::{
    Banner, Branch, CodeBlock, CodeLine, Document, Join, LabeledRow, Literalness, Node, NodeKind,
    Paragraph, Placement, PointerLine, Quoting, Section, SpeakerRow, Truncation,
};
use crate::wrap::{emit_runs, runs_width, wrap};

/// The indentation unit, in columns.
const INDENT: usize = 3;
/// The gap between a speaker row's columns.
const COLUMN_GAP: usize = 2;
/// Below this many columns of payload, a speaker row stacks instead of aligning.
const MIN_PAYLOAD: usize = 20;
/// Below this many columns of body, a labelled row stacks instead of hanging.
/// Higher than [`MIN_PAYLOAD`] on purpose: a speaker row's columns carry three
/// distinct things a reader scans down, and are worth crowding to keep; a
/// labelled row's column carries one short word, so prose wins sooner.
const MIN_LABELED_BODY: usize = 32;

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
pub fn render<K: Clone>(document: &Document<K>, width: impl Into<Width>) -> Rendered<K> {
    render_framed(document, &Frame::of_width(width.into()))
}

/// Lays a document out into an explicit box.
///
/// The entry point for placing a document inside geometry the caller already
/// owns — a column, a pane, a region beside a float.
#[must_use]
pub fn render_framed<K: Clone>(document: &Document<K>, frame: &Frame) -> Rendered<K> {
    let mut sink = Sink::new();
    render_nodes(&mut sink, &document.nodes, frame);
    let (text, spans) = sink.finish();
    Rendered { text, spans }
}

/// Advances to `column`, always leaving at least one space if the line has
/// content — so an over-wide column never runs its neighbour together.
fn advance_to<K: Clone>(sink: &mut Sink<K>, column: usize) {
    let minimum = if sink.line_is_empty() {
        0
    } else {
        sink.column().saturating_add(1)
    };
    sink.pad_to(column.max(minimum));
}

fn render_nodes<K: Clone>(sink: &mut Sink<K>, nodes: &[Node<K>], frame: &Frame) {
    let mut index = 0usize;
    while let Some(node) = nodes.get(index) {
        if index > 0 {
            separate(sink, &node.kind);
        }
        let rest = nodes.get(index..).unwrap_or_default();
        let consumed = match &node.kind {
            NodeKind::Speaker(_) => render_speaker_group(sink, rest, frame),
            NodeKind::Labeled(_) => render_labeled_group(sink, rest, frame),
            other => {
                render_single(sink, other, frame);
                1
            }
        };
        index = index.saturating_add(consumed.max(1));
    }
}

/// Vertical separation between two nodes.
///
/// Grouped kinds are consumed whole by their group renderers, so this only ever
/// sees a boundary between unlike things — which is exactly where a blank line
/// belongs. A pointer is the exception: it refers to what precedes it, so
/// separating it from that would be a lie about what it points at.
fn separate<K: Clone>(sink: &mut Sink<K>, next: &NodeKind<K>) {
    if matches!(next, NodeKind::Pointer(_)) {
        return;
    }
    sink.blank_line();
}

fn render_single<K: Clone>(sink: &mut Sink<K>, kind: &NodeKind<K>, frame: &Frame) {
    match kind {
        NodeKind::Section(section) => render_section(sink, section, frame),
        NodeKind::Code(block) => render_code(sink, block, frame),
        NodeKind::Pointer(pointer) => render_pointer(sink, pointer, frame),
        NodeKind::Banner(banner) => render_banner(sink, banner, frame),
        NodeKind::Prose(paragraph) => render_prose(sink, paragraph, frame),
        NodeKind::Join(join) => render_join(sink, join, frame),
        NodeKind::Truncation(truncation) => render_truncation(sink, truncation, frame),
        NodeKind::Speaker(row) => {
            render_speaker_row(sink, row, frame, &SpeakerColumns::stacked());
        }
        NodeKind::Labeled(row) => render_labeled_row(sink, row, frame, None),
    }
}

fn render_section<K: Clone>(sink: &mut Sink<K>, section: &Section<K>, frame: &Frame) {
    sink.end_line();
    sink.pad_to(frame.left());
    sink.layout(SECTION_OPEN);
    emit_runs(sink, &section.header);
    if let Some(counts) = &section.counts {
        sink.layout(COUNTS_OPEN);
        emit_runs(sink, counts);
        sink.layout(COUNTS_CLOSE);
    }
    sink.layout(SECTION_CLOSE);
    if !section.body.is_empty() {
        sink.blank_line();
        render_nodes(sink, &section.body, frame);
    }
}

fn render_banner<K: Clone>(sink: &mut Sink<K>, banner: &Banner<K>, frame: &Frame) {
    sink.end_line();
    wrap(sink, &banner.headline, frame);
    if !banner.body.is_empty() {
        sink.end_line();
        render_nodes(sink, &banner.body, &frame.inset(INDENT));
    }
}

fn render_prose<K: Clone>(sink: &mut Sink<K>, paragraph: &Paragraph<K>, frame: &Frame) {
    sink.end_line();
    wrap(sink, &paragraph.runs, frame);
}

fn render_truncation<K: Clone>(sink: &mut Sink<K>, truncation: &Truncation<K>, frame: &Frame) {
    sink.end_line();
    sink.pad_to(frame.left());
    sink.layout(TRUNCATION_LEAD);
    wrap(sink, &truncation.note, &frame.inset(TRUNCATION_LEAD.len()));
}

/// A pointer is never wrapped: it must stay copy-paste-true.
fn render_pointer<K: Clone>(sink: &mut Sink<K>, pointer: &PointerLine<K>, frame: &Frame) {
    let width = runs_width(&pointer.target);
    let flush_right = frame.right().saturating_sub(width);
    match pointer.placement {
        Placement::Standalone => {
            sink.end_line();
            sink.pad_to(frame.left());
        }
        Placement::Trailing => {
            let fits_beside = !sink.line_is_empty() && sink.column() < flush_right;
            if !fits_beside {
                sink.end_line();
            }
            sink.pad_to(flush_right.max(frame.left()));
        }
    }
    emit_runs(sink, &pointer.target);
}

fn render_join<K: Clone>(sink: &mut Sink<K>, join: &Join<K>, frame: &Frame) {
    for (index, branch) in join.branches.iter().enumerate() {
        if index > 0 {
            sink.blank_line();
        }
        render_branch(sink, branch, frame);
    }
    if let Some(restatement) = &join.restatement {
        sink.blank_line();
        wrap(sink, &restatement.runs, frame);
    }
}

fn render_branch<K: Clone>(sink: &mut Sink<K>, branch: &Branch<K>, frame: &Frame) {
    if let Some(connective) = &branch.connective {
        sink.end_line();
        sink.pad_to(frame.left());
        emit_runs(sink, connective);
    }
    if !branch.nodes.is_empty() {
        sink.end_line();
        render_nodes(sink, &branch.nodes, &frame.inset(INDENT));
    }
}

fn render_attachments<K: Clone>(sink: &mut Sink<K>, attachments: &[Node<K>], frame: &Frame) {
    if attachments.is_empty() {
        return;
    }
    sink.blank_line();
    render_nodes(sink, attachments, &frame.inset(INDENT));
}

// ---- labelled rows -------------------------------------------------------

/// Whether a group of labelled rows keeps its label column, and how wide it is.
///
/// `Some(width)` hangs each body off its label; `None` puts every label on its
/// own line with the body indented beneath. The decision is per GROUP rather
/// than per row, because a table in which some rows hang and others stack reads
/// as broken rather than as adaptive.
fn label_column<K: Clone>(rows: &[&LabeledRow<K>], frame: &Frame) -> Option<usize> {
    let width = rows
        .iter()
        .map(|row| runs_width(&row.label))
        .max()
        .unwrap_or(0);
    let body_left = frame.left().saturating_add(width).saturating_add(1);
    let usable = frame.right().saturating_sub(body_left);
    let anything_wraps = rows.iter().any(|row| runs_width(&row.body) > usable);
    // A hanging indent is alignment when the box is roomy and a tax when it is
    // not: every continuation line pays the label's width forever, while
    // stacking pays one line once. So it survives only while the body stays
    // wide enough to read as prose, and only while it is buying something —
    // stacking rows that already fit on one line just spends lines.
    let hanging_is_affordable = body_left.saturating_add(MIN_LABELED_BODY) <= frame.right();
    if hanging_is_affordable || !anything_wraps {
        Some(width)
    } else {
        None
    }
}

fn render_labeled_group<K: Clone>(sink: &mut Sink<K>, nodes: &[Node<K>], frame: &Frame) -> usize {
    let rows: Vec<&LabeledRow<K>> = nodes
        .iter()
        .map_while(|node| match &node.kind {
            NodeKind::Labeled(row) => Some(row),
            _ => None,
        })
        .collect();
    let column = label_column(&rows, frame);
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            sink.end_line();
        }
        render_labeled_row(sink, row, frame, column);
    }
    rows.len()
}

fn render_labeled_row<K: Clone>(
    sink: &mut Sink<K>,
    row: &LabeledRow<K>,
    frame: &Frame,
    column: Option<usize>,
) {
    sink.end_line();
    sink.pad_to(frame.left());
    emit_runs(sink, &row.label);
    let body_left = if let Some(label_width) = column {
        let left = frame
            .left()
            .saturating_add(label_width.max(runs_width(&row.label)))
            .saturating_add(1);
        advance_to(sink, left);
        left
    } else {
        sink.end_line();
        frame.left().saturating_add(INDENT)
    };
    wrap(
        sink,
        &row.body,
        &frame.inset(body_left.saturating_sub(frame.left())),
    );
    render_attachments(sink, &row.attachments, frame);
}

// ---- speaker rows --------------------------------------------------------

/// The column geometry shared by one run of adjacent speaker rows.
struct SpeakerColumns {
    mark: usize,
    speaker: usize,
    payload: usize,
    stacked: bool,
}

impl SpeakerColumns {
    /// The degenerate geometry for a lone row rendered outside a group.
    fn stacked() -> Self {
        Self {
            mark: 0,
            speaker: 0,
            payload: 0,
            stacked: true,
        }
    }

    /// Measures a group, and decides whether its columns fit the box at all.
    ///
    /// Column widths are derived from the rows present, never supplied — the
    /// answer the whole diagnostic-renderer family converged on, and the reason
    /// a group renders as one visually coherent table rather than as rows that
    /// happen to be near each other.
    fn measure<K: Clone>(rows: &[&SpeakerRow<K>], frame: &Frame) -> Self {
        let glyph = rows
            .iter()
            .map(|row| row.gutter.as_ref().map_or(0, |gutter| gutter.text.len()))
            .max()
            .unwrap_or(0);
        let mark = if glyph == 0 {
            0
        } else {
            glyph.saturating_add(1)
        };
        let speaker = rows
            .iter()
            .map(|row| runs_width(&row.speaker))
            .max()
            .unwrap_or(0);
        let verb = rows
            .iter()
            .map(|row| row.verb.as_ref().map_or(0, |runs| runs_width(runs)))
            .max()
            .unwrap_or(0);
        let verb_column = if verb == 0 {
            0
        } else {
            verb.saturating_add(COLUMN_GAP)
        };
        let payload = frame
            .left()
            .saturating_add(mark)
            .saturating_add(speaker)
            .saturating_add(COLUMN_GAP)
            .saturating_add(verb_column);
        let stacked = payload.saturating_add(MIN_PAYLOAD) > frame.right();
        Self {
            mark,
            speaker,
            payload,
            stacked,
        }
    }
}

fn render_speaker_group<K: Clone>(sink: &mut Sink<K>, nodes: &[Node<K>], frame: &Frame) -> usize {
    let rows: Vec<&SpeakerRow<K>> = nodes
        .iter()
        .map_while(|node| match &node.kind {
            NodeKind::Speaker(row) => Some(row),
            _ => None,
        })
        .collect();
    let columns = SpeakerColumns::measure(&rows, frame);
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            sink.end_line();
        }
        render_speaker_row(sink, row, frame, &columns);
    }
    rows.len()
}

fn render_speaker_row<K: Clone>(
    sink: &mut Sink<K>,
    row: &SpeakerRow<K>,
    frame: &Frame,
    columns: &SpeakerColumns,
) {
    sink.end_line();
    sink.pad_to(frame.left());
    if let Some(gutter) = &row.gutter {
        sink.run(&gutter.text, &gutter.provenance);
    }
    sink.pad_to(frame.left().saturating_add(columns.mark));
    emit_runs(sink, &row.speaker);

    let payload_left = if columns.stacked {
        if let Some(verb) = &row.verb {
            let beside = sink.column().saturating_add(1);
            advance_to(sink, beside);
            emit_runs(sink, verb);
        }
        sink.end_line();
        frame.left().saturating_add(INDENT)
    } else {
        if let Some(verb) = &row.verb {
            advance_to(
                sink,
                frame
                    .left()
                    .saturating_add(columns.mark)
                    .saturating_add(columns.speaker)
                    .saturating_add(COLUMN_GAP),
            );
            emit_runs(sink, verb);
        }
        advance_to(sink, columns.payload);
        columns.payload
    };

    render_payload(sink, row, frame, payload_left);
    render_attachments(sink, &row.attachments, frame);
}

fn render_payload<K: Clone>(
    sink: &mut Sink<K>,
    row: &SpeakerRow<K>,
    frame: &Frame,
    payload_left: usize,
) {
    let quoted = matches!(row.payload.quoting, Quoting::Quoted);
    let mut payload_frame = frame.inset(payload_left.saturating_sub(frame.left()));
    if quoted {
        // The closing quote must land inside the box, so it is withheld from
        // every line rather than allowed to overrun the last one.
        payload_frame = payload_frame.reserving(Reservation::all_lines(Side::Right, QUOTE.len()));
        sink.pad_to(payload_left);
        sink.layout(QUOTE);
    }
    wrap(sink, &row.payload.runs, &payload_frame);
    if quoted {
        sink.layout(QUOTE);
    }
    if !row.payload.trailer.is_empty() {
        wrap(
            sink,
            &row.payload.trailer,
            &frame.inset(payload_left.saturating_sub(frame.left())),
        );
    }
}

// ---- code blocks ---------------------------------------------------------

/// The gutter separator for a mode, given whether the block has a gutter.
///
/// A descriptive block always carries its marker, gutter or not: the
/// non-runnable mark is load-bearing, so there is no shape in which it can go
/// missing.
fn separator(mode: Literalness, has_gutter: bool) -> &'static str {
    match (mode, has_gutter) {
        (Literalness::Descriptive, true) => GUTTER_DESCRIPTIVE,
        (Literalness::Descriptive, false) => DESCRIPTIVE_BARE,
        (_, true) => GUTTER_RUNNABLE,
        (_, false) => "",
    }
}

/// The left edge of each content cell, measured across the whole block.
///
/// A block's cells are columns, exactly as a run of speaker rows is: column `n`
/// starts where the widest cell `n-1` ends. One-cell lines therefore measure to
/// a single column and are emitted untouched, while multi-cell lines square up.
fn cell_columns<K: Clone>(block: &CodeBlock<K>, content_left: usize) -> Vec<usize> {
    let cell_count = block
        .lines
        .iter()
        .map(|line| line.cells.len())
        .max()
        .unwrap_or(0);
    let mut columns = Vec::with_capacity(cell_count);
    let mut left = content_left;
    for index in 0..cell_count {
        columns.push(left);
        let widest = block
            .lines
            .iter()
            .filter_map(|line| line.cells.get(index))
            .map(|cell| runs_width(&cell.runs))
            .max()
            .unwrap_or(0);
        left = left.saturating_add(widest).saturating_add(COLUMN_GAP);
    }
    columns
}

fn render_code<K: Clone>(sink: &mut Sink<K>, block: &CodeBlock<K>, frame: &Frame) {
    sink.end_line();
    if let Some(locus) = &block.locus {
        sink.pad_to(frame.left().saturating_add(INDENT));
        sink.layout(LOCUS_OPEN);
        emit_runs(sink, locus);
        sink.end_line();
    }
    let gutter_width = block
        .lines
        .iter()
        .map(|line| line.gutter.as_ref().map_or(0, |gutter| gutter.text.len()))
        .max()
        .unwrap_or(0);
    let separator = separator(block.mode, gutter_width > 0);
    let content_left = frame
        .left()
        .saturating_add(gutter_width)
        .saturating_add(separator.len());
    let columns = cell_columns(block, content_left);

    for line in &block.lines {
        sink.end_line();
        sink.pad_to(frame.left());
        if let Some(gutter) = &line.gutter {
            sink.pad_to(
                frame
                    .left()
                    .saturating_add(gutter_width.saturating_sub(gutter.text.len())),
            );
            sink.run(&gutter.text, &gutter.provenance);
        } else {
            sink.pad_to(frame.left().saturating_add(gutter_width));
        }
        sink.layout(separator);
        render_cells(sink, line, block.mode, frame, &columns);
    }
}

fn render_cells<K: Clone>(
    sink: &mut Sink<K>,
    line: &CodeLine<K>,
    mode: Literalness,
    frame: &Frame,
    columns: &[usize],
) {
    for (index, cell) in line.cells.iter().enumerate() {
        let left = columns.get(index).copied().unwrap_or_else(|| sink.column());
        if index > 0 {
            advance_to(sink, left);
        }
        match mode {
            // Byte-honest modes never rewrap; an overrun is preferable to
            // implying a line break the source does not contain.
            Literalness::Literal | Literalness::Formatted => emit_runs(sink, &cell.runs),
            Literalness::Descriptive => wrap(
                sink,
                &cell.runs,
                &frame.inset(left.saturating_sub(frame.left())),
            ),
        }
    }
}
