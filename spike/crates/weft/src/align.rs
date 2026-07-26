//! Column widths shared between nodes that are not siblings.
//!
//! Columns fall out of proximity easily enough: a run of adjacent speaker rows
//! measures itself, a code block measures its own cells. But the things a
//! reader wants squared up are not reliably siblings. Two `fix:` rows sitting
//! in different branches of a join are the same kind of thing said twice; so
//! are the trailing comments of two separate excerpts, and a row of chain
//! evidence that resumes after an attachment interrupts it. Under a purely
//! structural rule none of those can relate, because the tree puts them under
//! different parents.
//!
//! So the relationship is *named* rather than inherited. Any element may carry
//! an opaque group key, and every element sharing that key shares a column
//! width wherever it sits. Measurement is a whole-document pass that runs
//! before layout, which is what lets a width be shared backwards — a row near
//! the top can be widened by one near the bottom it will never meet. Naming is
//! also the honest encoding: these elements relate because they are the *same
//! kind of thing repeated*, which is a claim about relevance, not structure.
//! The consumer knows which repetitions matter; weft cannot infer it.
//!
//! # This shares width, NOT position — read before relying on it
//!
//! A column's screen position is a prefix sum:
//!
//! ```text
//! position(column n) = enclosing indent
//!                    + gutter width + separator width
//!                    + sum of every column left of n, plus their gaps
//! ```
//!
//! A group shares ONE term of that sum. Two members land in the same place
//! only when every other term already agrees, which means:
//!
//! - **every column to the left must also be grouped** — sharing a comment
//!   column while leaving the code column ungrouped just moves the mismatch;
//! - **members must sit at the same nesting depth** — enclosing indent is not
//!   groupable at all, so a row inside a join branch stays offset by the
//!   branch's indent no matter what it names;
//! - **code blocks must share a literalness mode** — a descriptive block's
//!   marker is a different width from a runnable block's separator.
//!
//! Miss one and the result is a silent one- or two-column offset, not an error.
//!
//! The successor is column STOPS: the measure pass learns each member's
//! absolute left, a group resolves to `max(start)` across its members, and
//! every member pads to that stop — which also makes narrow-width degradation
//! a per-group decision rather than a per-local-run one. That needs the measure
//! pass to walk geometry rather than widths alone, i.e. a genuine dry run of
//! the layout pass (the scan half of the Oppen two-pass shape). Deliberately
//! not built here: the resolver's hard cases — what a stop does when it cannot
//! fit the box, whether a group spanning two depths aligns outward or refuses —
//! want real consumer demands rather than invented ones.

use crate::tree::{Document, Node, NodeKind};
use crate::wrap::runs_width;

/// Column indices within a group, so several columns can share one group key.
pub(crate) const COLUMN_PRIMARY: usize = 0;
pub(crate) const COLUMN_SECONDARY: usize = 1;
pub(crate) const COLUMN_TERTIARY: usize = 2;

/// Every named alignment group in a document, measured.
pub(crate) struct Alignments<K> {
    entries: Vec<Entry<K>>,
}

struct Entry<K> {
    group: K,
    column: usize,
    width: usize,
}

impl<K: Clone + PartialEq> Alignments<K> {
    /// Measures the whole document before any of it is laid out.
    pub(crate) fn measure(document: &Document<K>) -> Self {
        let mut alignments = Self {
            entries: Vec::new(),
        };
        alignments.walk(&document.nodes);
        alignments
    }

    /// The width a column should occupy: the group's shared width, never
    /// narrower than the element's own.
    ///
    /// An ungrouped element simply gets its own width back, so a document that
    /// names no groups lays out exactly as it would without this machinery.
    pub(crate) fn shared(&self, group: Option<&K>, column: usize, own: usize) -> usize {
        let Some(group) = group else {
            return own;
        };
        self.entries
            .iter()
            .find(|entry| entry.column == column && entry.group == *group)
            .map_or(own, |entry| entry.width.max(own))
    }

    fn record(&mut self, group: Option<&K>, column: usize, width: usize) {
        let Some(group) = group else {
            return;
        };
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.column == column && entry.group == *group)
        {
            entry.width = entry.width.max(width);
            return;
        }
        self.entries.push(Entry {
            group: group.clone(),
            column,
            width,
        });
    }

    fn walk(&mut self, nodes: &[Node<K>]) {
        for node in nodes {
            match &node.kind {
                NodeKind::Section(section) => self.walk(&section.body),
                NodeKind::Banner(banner) => self.walk(&banner.body),
                NodeKind::Join(join) => {
                    for branch in &join.branches {
                        self.walk(&branch.nodes);
                    }
                }
                NodeKind::Labeled(row) => {
                    self.record(row.align.as_ref(), COLUMN_PRIMARY, runs_width(&row.label));
                    self.walk(&row.attachments);
                }
                NodeKind::Speaker(row) => {
                    let gutter = row.gutter.as_ref().map_or(0, |run| run.text.len());
                    let verb = row.verb.as_ref().map_or(0, |runs| runs_width(runs));
                    self.record(row.align.as_ref(), COLUMN_PRIMARY, gutter);
                    self.record(
                        row.align.as_ref(),
                        COLUMN_SECONDARY,
                        runs_width(&row.speaker),
                    );
                    self.record(row.align.as_ref(), COLUMN_TERTIARY, verb);
                    self.walk(&row.attachments);
                }
                NodeKind::Code(block) => {
                    for line in &block.lines {
                        let gutter = line.gutter.as_ref().map_or(0, |run| run.text.len());
                        self.record(block.align.as_ref(), COLUMN_PRIMARY, gutter);
                        for (index, cell) in line.cells.iter().enumerate() {
                            self.record(cell.align.as_ref(), index, runs_width(&cell.runs));
                        }
                    }
                }
                NodeKind::Pointer(_) | NodeKind::Prose(_) | NodeKind::Truncation(_) => {}
            }
        }
    }
}
