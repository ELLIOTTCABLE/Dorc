//! The measure pass: where every column stop comes from.
//!
//! Columns fall out of proximity easily enough — a run of adjacent speaker rows
//! measures itself, a code block measures its own cells. But the things a reader
//! wants squared up are not reliably siblings. Two `fix:` rows in different
//! branches of a join are the same kind of thing said twice; so are the trailing
//! comments of two separate excerpts. Under a purely structural rule none of
//! those can relate, because the tree puts them under different parents.
//!
//! So a member joins a TABLE by naming it, and everything sharing that name
//! resolves together. Naming rather than inheriting is the honest encoding:
//! these members relate because they are the same kind of thing repeated, which
//! is a claim about relevance that only the consumer can make.
//!
//! # Why a table and not a column
//!
//! A column's screen position is a prefix sum: where column `n` starts depends
//! on every column left of it. Sharing one column's width while its neighbours
//! resolve independently therefore produces silent one-column offsets — which is
//! how the first attempt at this failed, and why it was withdrawn rather than
//! patched. A table shares the whole prefix by construction:
//!
//! ```text
//! stop[0]   = max over members of (the member's left edge + its own lead)
//! width[n]  = max over members of (the width of its cell n)
//! stop[n+1] = stop[n] + width[n] + gap        (a zero-width column takes nothing)
//! ```
//!
//! One left-to-right pass, no fixpoint. It needs the tree walked with each
//! node's left edge in hand, but it needs neither the right edge for the stops
//! nor any wrapping, because a column's width is its UNWRAPPED content and
//! wrapping happens after the stops are fixed. The two-pass shape is an
//! ordering, not an iteration.
//!
//! # Anonymous tables
//!
//! Structural proximity is not a second mechanism: a run of adjacent rows, or
//! one code block, is simply a table nobody named. That keeps exactly one code
//! path resolving columns, so the ungrouped case cannot drift away from the
//! named one.
//!
//! # Degradation is a table-wide decision
//!
//! A table stacks as a UNIT, on its narrowest member (`28F:rul-table-degrades-
//! whole`). A wide member forced to stack by a narrow sibling is the correct
//! outcome: a table in which some rows hang and others stack reads as broken
//! rather than as adaptive, and the ragged result is worse than the uniform one.

use crate::frame::Frame;
use crate::tree::{CodeBlock, Document, LabeledRow, Node, NodeKind, Quoting, SpeakerRow};
use crate::wrap::runs_width;

/// The gap between two adjacent columns of a table.
const COLUMN_GAP: usize = 2;
/// Below this many columns of payload, a speaker table stacks instead of aligning.
const MIN_PAYLOAD: usize = 20;
/// Below this many columns of body, a labelled table stacks instead of hanging.
/// Higher than [`MIN_PAYLOAD`] on purpose: a speaker row's columns carry three
/// distinct things a reader scans down, and are worth crowding to keep; a
/// labelled row's column carries one short word, so prose wins sooner.
const MIN_LABELED_BODY: usize = 32;

/// The resolved geometry of one table.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Stops {
    /// Absolute columns: `stops[n]` is where cell `n` begins, and the last entry
    /// is where a member's free-flowing tail begins.
    stops: Vec<usize>,
    /// Whether the whole table gave up on columns.
    stacked: bool,
}

impl Stops {
    /// Where cell `index` begins, or the tail stop for an index past the last
    /// column.
    pub(crate) fn stop(&self, index: usize) -> usize {
        self.stops
            .get(index)
            .or_else(|| self.stops.last())
            .copied()
            .unwrap_or(0)
    }

    /// Where the free-flowing tail after the last column begins.
    pub(crate) fn tail(&self) -> usize {
        self.stops.last().copied().unwrap_or(0)
    }

    /// Whether members render stacked rather than in columns.
    pub(crate) fn stacked(&self) -> bool {
        self.stacked
    }
}

/// What the layout pass is owed for one member, in visit order.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Placement {
    /// The member's resolved table.
    pub(crate) stops: Stops,
    /// The left edge the measure pass predicted for this member.
    pub(crate) left: usize,
}

/// Every table in a document, resolved, plus the ordered placements the layout
/// pass consumes.
pub(crate) struct Measured {
    tables: Vec<Stops>,
    members: Vec<(usize, usize)>,
}

impl Measured {
    /// The placement of the `index`th member in visit order.
    ///
    /// A missing entry cannot happen while the two walks agree, and the layout
    /// pass asserts that they do; returning a degenerate placement rather than
    /// panicking keeps a release build rendering something honest if they ever
    /// stop agreeing.
    pub(crate) fn placement(&self, index: usize) -> Placement {
        let (table, left) = self.members.get(index).copied().unwrap_or((usize::MAX, 0));
        let stops = self.tables.get(table).cloned().unwrap_or(Stops {
            stops: vec![left],
            stacked: true,
        });
        Placement { stops, left }
    }
}

/// One member's contribution while its table is still being accumulated.
struct Contribution {
    left: usize,
    lead: usize,
    cells: Vec<usize>,
    demand: Demand,
}

/// What a member needs of the space after its last column, for the stacking
/// decision. A member with no minimum — a code block, whose lines overrun rather
/// than reflow — never votes to stack.
struct Demand {
    right: usize,
    minimum: usize,
    unwrapped: usize,
}

/// A table under construction.
struct Draft<K> {
    name: Option<K>,
    lead_stop: usize,
    widths: Vec<usize>,
    demands: Vec<Demand>,
}

impl<K: Clone + PartialEq> Draft<K> {
    fn absorb(&mut self, contribution: Contribution) {
        self.lead_stop = self
            .lead_stop
            .max(contribution.left.saturating_add(contribution.lead));
        for (index, width) in contribution.cells.into_iter().enumerate() {
            match self.widths.get_mut(index) {
                Some(current) => *current = (*current).max(width),
                None => self.widths.push(width),
            }
        }
        self.demands.push(contribution.demand);
    }

    fn resolve(&self) -> Stops {
        let mut stops = vec![self.lead_stop];
        for width in &self.widths {
            let previous = stops.last().copied().unwrap_or(0);
            let gap = if *width == 0 { 0 } else { COLUMN_GAP };
            stops.push(previous.saturating_add(*width).saturating_add(gap));
        }
        let tail = stops.last().copied().unwrap_or(0);
        // A member objects only when the tail is BOTH too narrow to read as
        // prose and actually forced to wrap; content that fits keeps its columns
        // however tight they look.
        let stacked = self.demands.iter().any(|demand| {
            demand.minimum > 0
                && tail.saturating_add(demand.minimum) > demand.right
                && tail.saturating_add(demand.unwrapped) > demand.right
        });
        Stops { stops, stacked }
    }
}

/// Whether two adjacent nodes are rows of one kind, and so belong to one
/// anonymous table with no blank line between them.
pub(crate) fn are_adjacent_rows<K>(left: &NodeKind<K>, right: &NodeKind<K>) -> bool {
    matches!(
        (left, right),
        (NodeKind::Speaker(_), NodeKind::Speaker(_)) | (NodeKind::Labeled(_), NodeKind::Labeled(_))
    )
}

/// Measures a document: resolves every table, and predicts every member's left
/// edge for the layout pass to check itself against.
pub(crate) fn measure<K: Clone + PartialEq>(document: &Document<K>, frame: &Frame) -> Measured {
    let mut walk = Walk {
        drafts: Vec::new(),
        members: Vec::new(),
    };
    walk.nodes(&document.nodes, frame);
    Measured {
        tables: walk.drafts.iter().map(Draft::resolve).collect(),
        members: walk.members,
    }
}

struct Walk<K> {
    drafts: Vec<Draft<K>>,
    members: Vec<(usize, usize)>,
}

impl<K: Clone + PartialEq> Walk<K> {
    fn open(&mut self, name: Option<K>) -> usize {
        self.drafts.push(Draft {
            name,
            lead_stop: 0,
            widths: Vec::new(),
            demands: Vec::new(),
        });
        self.drafts.len().saturating_sub(1)
    }

    /// The table a member joins: the one it named, or the anonymous table of the
    /// structural run it sits in, opened on first use.
    fn join(&mut self, name: Option<&K>, run: &mut Option<usize>, contribution: Contribution) {
        let table = match name {
            Some(name) => self
                .drafts
                .iter()
                .position(|draft| draft.name.as_ref() == Some(name))
                .unwrap_or_else(|| self.open(Some(name.clone()))),
            None => match *run {
                Some(index) => index,
                None => *run.insert(self.open(None)),
            },
        };
        let left = contribution.left;
        if let Some(draft) = self.drafts.get_mut(table) {
            draft.absorb(contribution);
        }
        self.members.push((table, left));
    }

    fn nodes(&mut self, nodes: &[Node<K>], frame: &Frame) {
        // One anonymous table per RUN of adjacent same-kind rows: proximity is
        // the case of naming where the name is the run itself.
        let mut run: Option<usize> = None;
        for (index, node) in nodes.iter().enumerate() {
            let continues = index
                .checked_sub(1)
                .and_then(|previous| nodes.get(previous))
                .is_some_and(|previous| are_adjacent_rows(&previous.kind, &node.kind));
            if !continues {
                run = None;
            }
            match &node.kind {
                NodeKind::Speaker(row) => {
                    let contribution = speaker_contribution(row, frame);
                    self.join(row.table.as_ref(), &mut run, contribution);
                    self.nodes(&row.attachments, &frame.inset(crate::render::INDENT));
                }
                NodeKind::Labeled(row) => {
                    let contribution = labeled_contribution(row, frame);
                    self.join(row.table.as_ref(), &mut run, contribution);
                    self.nodes(&row.attachments, &frame.inset(crate::render::INDENT));
                }
                NodeKind::Code(block) => {
                    let contribution = code_contribution(block, frame);
                    self.join(block.table.as_ref(), &mut None, contribution);
                }
                NodeKind::Section(section) => self.nodes(&section.body, frame),
                NodeKind::Banner(banner) => {
                    self.nodes(&banner.body, &frame.inset(crate::render::INDENT));
                }
                NodeKind::Join(join) => {
                    for branch in &join.branches {
                        self.nodes(&branch.nodes, &frame.inset(crate::render::INDENT));
                    }
                }
                NodeKind::Pointer(_) | NodeKind::Prose(_) | NodeKind::Truncation(_) => {}
            }
        }
    }
}

/// A speaker row's gutter glyph is a LEAD, not a column: it sits one space in
/// front of the table proper, so a row that carries one and a row that does not
/// still start their speakers in the same column.
fn speaker_contribution<K>(row: &SpeakerRow<K>, frame: &Frame) -> Contribution {
    let gutter = row.gutter.as_ref().map_or(0, |run| run.text.len());
    let quotes = if matches!(row.payload.quoting, Quoting::Quoted) {
        2
    } else {
        0
    };
    Contribution {
        left: frame.left(),
        lead: if gutter == 0 {
            0
        } else {
            gutter.saturating_add(1)
        },
        cells: vec![
            runs_width(&row.speaker),
            row.verb.as_ref().map_or(0, |runs| runs_width(runs)),
        ],
        demand: Demand {
            right: frame.right(),
            minimum: MIN_PAYLOAD,
            unwrapped: runs_width(&row.payload.runs).saturating_add(quotes),
        },
    }
}

fn labeled_contribution<K>(row: &LabeledRow<K>, frame: &Frame) -> Contribution {
    Contribution {
        left: frame.left(),
        lead: 0,
        cells: vec![runs_width(&row.label)],
        demand: Demand {
            right: frame.right(),
            minimum: MIN_LABELED_BODY,
            unwrapped: runs_width(&row.body),
        },
    }
}

/// A code block's lead is its gutter plus the separator; its cells are the
/// columns a multi-cell line asked to have squared up.
fn code_contribution<K>(block: &CodeBlock<K>, frame: &Frame) -> Contribution {
    let gutter = block
        .lines
        .iter()
        .map(|line| line.gutter.as_ref().map_or(0, |run| run.text.len()))
        .max()
        .unwrap_or(0);
    let separator = crate::render::separator(block.mode, gutter > 0);
    let columns = block
        .lines
        .iter()
        .map(|line| line.cells.len())
        .max()
        .unwrap_or(0);
    let cells = (0..columns)
        .map(|index| {
            block
                .lines
                .iter()
                .filter_map(|line| line.cells.get(index))
                .map(|cell| runs_width(&cell.runs))
                .max()
                .unwrap_or(0)
        })
        .collect();
    Contribution {
        left: frame.left(),
        lead: gutter.saturating_add(separator.len()),
        cells,
        demand: Demand {
            right: frame.right(),
            // Source is shown byte-honest and overruns rather than reflowing, so
            // a code block never votes to stack the table it joins.
            minimum: 0,
            unwrapped: 0,
        },
    }
}
