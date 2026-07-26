//! The box model: the geometry a node lays out into.
//!
//! Weft is deliberately not a paragraph-wrapper with an indent argument. The
//! target shapes nest — a code excerpt inside an attached explanation inside a
//! chain row inside a section — and the eventual shapes float: an annotation
//! placed beside a code excerpt, outside it, narrowing the excerpt for exactly
//! the lines it sits beside. A renderer that threads `(width, indent)` around
//! cannot grow into that; one that threads a *box* can.
//!
//! So every layout decision reads its geometry from a [`Frame`]: a left edge, a
//! right edge, and a set of per-line [`Reservation`]s that carve columns out of
//! specific line ranges. Nesting is [`Frame::inset`]. A hanging indent is a box
//! whose first line begins further right. A float is a reservation. One
//! mechanism, three jobs — which is the point, because the fourth job (real
//! float placement) has to land in the same mechanism later without a rewrite.
//!
//! What is deliberately NOT here: deciding *where* a float goes. Reservations
//! are declared, never solved for — no collision resolution, no packing, no
//! reflow-until-fixpoint. Placement is the later engine; this is the seam it
//! plugs into.

/// A rendering width, in columns.
///
/// Columns are bytes: the crate's printable-ASCII contract makes the two the
/// same thing, which is why `rul-ascii-output-forever` is load-bearing for
/// layout and not merely a taste ruling.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Width(usize);

impl Width {
    /// The narrowest width weft will lay out into.
    pub const MINIMUM: usize = 1;

    /// Constructs a width, clamped to at least [`Width::MINIMUM`].
    ///
    /// Clamping rather than refusing: a zero width is a caller's arithmetic
    /// slip, and errors are data here rather than a panic on an input path.
    #[must_use]
    pub fn new(columns: usize) -> Self {
        Self(columns.max(Self::MINIMUM))
    }

    /// The width in columns.
    #[must_use]
    pub fn columns(self) -> usize {
        self.0
    }
}

impl From<usize> for Width {
    fn from(columns: usize) -> Self {
        Self::new(columns)
    }
}

/// Which edge a reservation eats into.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    /// Columns taken from the left edge.
    Left,
    /// Columns taken from the right edge.
    Right,
}

/// Columns withheld from a box over a range of its lines.
///
/// This is the float seam. A reservation says only "these lines are this much
/// narrower on this side"; it does not say what occupies the withheld columns,
/// and weft does not fill them. The caller draws the floated material itself,
/// which keeps placement policy outside the layout kernel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reservation {
    /// Which edge is eaten into.
    pub side: Side,
    /// First affected line, indexed within the box.
    pub first_line: usize,
    /// How many lines are affected. Zero affects nothing.
    pub line_count: usize,
    /// How many columns are withheld.
    pub columns: usize,
}

impl Reservation {
    /// A reservation covering every line of the box.
    #[must_use]
    pub fn all_lines(side: Side, columns: usize) -> Self {
        Self {
            side,
            first_line: 0,
            line_count: usize::MAX,
            columns,
        }
    }

    /// Whether this reservation covers a given line of the box.
    #[must_use]
    pub fn covers(&self, line: usize) -> bool {
        line >= self.first_line && line < self.first_line.saturating_add(self.line_count)
    }
}

/// The box a node lays out into: absolute left and right column bounds, plus
/// any per-line reservations narrowing them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Frame {
    left: usize,
    right: usize,
    reservations: Vec<Reservation>,
}

impl Frame {
    /// The root box: the full width, flush left, unreserved.
    #[must_use]
    pub fn of_width(width: Width) -> Self {
        Self {
            left: 0,
            right: width.columns(),
            reservations: Vec::new(),
        }
    }

    /// A nested box, inset from the left and dropping the outer box's
    /// reservations.
    ///
    /// Reservations are dropped rather than inherited because their line
    /// indices are relative to the box that declared them; carrying them inward
    /// would silently reinterpret them against a different line numbering. A
    /// caller that wants an inner box to respect an outer float re-declares it
    /// in the inner box's own coordinates, which is the honest spelling.
    #[must_use]
    pub fn inset(&self, columns: usize) -> Self {
        let left = self.left.saturating_add(columns);
        Self {
            left: left.min(self.right),
            right: self.right,
            reservations: Vec::new(),
        }
    }

    /// The same box with a reservation added.
    #[must_use]
    pub fn reserving(&self, reservation: Reservation) -> Self {
        let mut next = self.clone();
        next.reservations.push(reservation);
        next
    }

    /// The box's unreserved left edge.
    #[must_use]
    pub fn left(&self) -> usize {
        self.left
    }

    /// The box's unreserved right edge.
    #[must_use]
    pub fn right(&self) -> usize {
        self.right
    }

    /// The usable `(left, right)` bounds for one line of the box.
    ///
    /// Always returns a non-inverted pair: reservations that would eat past
    /// each other collapse to an empty-but-valid span rather than producing
    /// nonsense for the wrapper to divide by.
    #[must_use]
    pub fn usable(&self, line: usize) -> (usize, usize) {
        let mut left = self.left;
        let mut right = self.right;
        for reservation in &self.reservations {
            if !reservation.covers(line) {
                continue;
            }
            match reservation.side {
                Side::Left => left = left.saturating_add(reservation.columns),
                Side::Right => right = right.saturating_sub(reservation.columns),
            }
        }
        (left.min(right), right)
    }
}
