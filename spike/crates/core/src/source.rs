//! What an acquired source IS to a run — the decide-plane half of acquisition.
//!
//! Lives here rather than beside the acquisition edge because two planes ask it. The edge
//! asks it to decide what loads before the book's first line and whose text the engine
//! models at all; a durable projection asks it to record, per source, what that source was.
//! A role the projection could not name would have to be recorded as some other role, and a
//! recorded load order that never happened is the one thing a receipt may not carry.

/// What a source IS in this run, independently of where it sits in any vector.
///
/// Role is CARRIED, never derived from position: a source is a book because it was named as
/// one, not because it sorts last. Reading role off an ordering fossilizes "exactly one book,
/// at the end" into every consumer that re-derives it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceRole {
    /// A root the invocation named as a book: admin-authored, chaotic, and the surface the
    /// plan renders. Its own `.` acts change visibility and mint no speaker.
    Book,
    /// A root the invocation named to LOAD — an oracle. Its definitions are ambient: they
    /// load before the book's first line, as the `.` prelude a pre-source is.
    NamedLoad,
    /// A root reached only from a book `.`. It loads AT that line and nowhere else, so its
    /// definitions license nothing above their own load point.
    BookSourced,
    /// A source acquired only because a NAMED root's top level sources it: it loads at that
    /// `.`, inside its sourcer's program, never before line 1 and never again as a root of
    /// its own.
    LoadDependency,
    /// An ordinary sh file a book `.` names, acquired for its BYTES and modelled NOT AT ALL
    /// (`30P:principle-book-code-source-is-inclusion`).
    ///
    /// It signed no dorc-lang contract, so nothing is lifted from it, nothing it declares
    /// binds, and its `.` site walls exactly as an unread one does
    /// (`FORFEITS:forfeit-plain-sh-inclusion-analysis`). What acquiring it buys is one thing:
    /// the artifact can mirror it beside the plan, so the author's own `.` finds it.
    PlainInclusion,
}

impl SourceRole {
    /// Every role, for a census walking the set.
    pub const ALL: [Self; 5] = [
        Self::Book,
        Self::NamedLoad,
        Self::BookSourced,
        Self::LoadDependency,
        Self::PlainInclusion,
    ];

    /// Does this source load before the book's first line?
    #[must_use]
    pub const fn is_ambient(self) -> bool {
        matches!(self, Self::NamedLoad)
    }

    /// Does the engine MODEL this source's text at all?
    ///
    /// The one predicate every LIFT and INDEX seat asks, so "acquired for bytes" cannot leak
    /// into a definition universe by a consumer forgetting.
    #[must_use]
    pub const fn is_modelled(self) -> bool {
        !matches!(self, Self::PlainInclusion)
    }
}
