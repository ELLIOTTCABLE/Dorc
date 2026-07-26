//! The output accumulator that keeps the span accounting honest.
//!
//! Every byte weft emits passes through exactly one method here, and every one
//! of those methods records a span. That is the whole enforcement mechanism for
//! total cover: there is no path from a layout routine to the output buffer
//! that bypasses attribution, so a new node kind cannot quietly emit
//! unattributed bytes.

use crate::provenance::{Provenance, Span};

pub(crate) struct Sink<K> {
    text: String,
    spans: Vec<Span<K>>,
    line_start: usize,
}

impl<K: Clone> Sink<K> {
    pub(crate) fn new() -> Self {
        Self {
            text: String::new(),
            spans: Vec::new(),
            line_start: 0,
        }
    }

    /// The current column, which is the byte offset within the current line.
    pub(crate) fn column(&self) -> usize {
        self.text.len().saturating_sub(self.line_start)
    }

    pub(crate) fn line_is_empty(&self) -> bool {
        self.column() == 0
    }

    /// Emits renderer-minted layout: indentation, padding, separators, glyphs.
    pub(crate) fn layout(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        // Layout bytes coalesce with their neighbours; consumer runs never do,
        // since two adjacent runs may carry distinguishable keys.
        if let Some(last) = self.spans.last_mut()
            && matches!(last.provenance, Provenance::Arrangement { key: None })
        {
            last.len = last.len.saturating_add(text.len());
            self.text.push_str(text);
            return;
        }
        self.spans.push(Span {
            provenance: Provenance::Arrangement { key: None },
            start: self.text.len(),
            len: text.len(),
        });
        self.text.push_str(text);
    }

    /// Emits consumer text under its own authorship.
    pub(crate) fn run(&mut self, text: &str, provenance: &Provenance<K>) {
        if text.is_empty() {
            return;
        }
        self.spans.push(Span {
            provenance: provenance.clone(),
            start: self.text.len(),
            len: text.len(),
        });
        self.text.push_str(text);
    }

    /// Pads with spaces up to `column`; a no-op if already at or past it.
    pub(crate) fn pad_to(&mut self, column: usize) {
        let deficit = column.saturating_sub(self.column());
        if deficit > 0 {
            self.layout(&" ".repeat(deficit));
        }
    }

    pub(crate) fn newline(&mut self) {
        self.layout("\n");
        self.line_start = self.text.len();
    }

    /// Ends the current line if it has content.
    pub(crate) fn end_line(&mut self) {
        if !self.line_is_empty() {
            self.newline();
        }
    }

    /// Ends the current line and leaves one wholly empty line behind it.
    ///
    /// Idempotent, and a no-op at the start of the output: separators are
    /// emitted before the thing they separate, so this must never open a
    /// document with a blank or stack two blanks together.
    pub(crate) fn blank_line(&mut self) {
        if self.text.is_empty() {
            return;
        }
        self.end_line();
        if !self.text.ends_with("\n\n") {
            self.newline();
        }
    }

    pub(crate) fn finish(mut self) -> (String, Vec<Span<K>>) {
        self.end_line();
        (self.text, self.spans)
    }
}
