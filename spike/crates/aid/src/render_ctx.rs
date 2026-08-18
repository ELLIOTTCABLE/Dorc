//! What a render seat needs besides the thing it is rendering: which prose tables to read, and
//! what box to lay out into (`28L:rul-render-context-struct`).
//!
//! Two facts travel together here because a seat that got one without the other would be a bug
//! that compiles. The tables are const in production and a MUTABLE MIRROR under
//! `dorc-loom publish`, which is the whole reason an edited row can re-render before anyone
//! rebuilds (`28H:finding-why-render-reads-the-const-not-the-mirror`); a seat holding the
//! arrangement mirror and the const catalog would show an author half of their own edit. The box
//! rides along because it is the other thing every laid-out seat had to be handed, and threading
//! two parameters through the same call graph twice is churn twice (the resolved
//! `tc-lookup-parameter-vs-render-context`).
//!
//! Deliberately a PARAMETER and never ambient: a thread-local would put hidden state in a render
//! seat, which `inv-determinism` and this project's taste both rule out.

use crate::arrangement::{ArrangementLookup, CONST_ARRANGEMENTS, ConstArrangements};
use crate::catalog::{CONST_CATALOG, CatalogLookup, ConstCatalog};

/// The width committed transcripts — and the deterministic no-terminal fallback — lay out at.
///
/// A width, not THE width: [`RenderCtx::at_width`] takes one, so a surface that knows its own box
/// passes that instead. Detecting a terminal's width is an I/O-edge concern and never reaches a
/// render seat (`inv-determinism`).
pub const CANONICAL_TRANSCRIPT_WIDTH: usize = 80;

/// The tables a render reads and the box it lays out into.
#[derive(Clone)]
pub struct RenderCtx<'a> {
    catalog: &'a dyn CatalogLookup,
    arrangements: &'a dyn ArrangementLookup,
    frame: weft::Frame,
}

impl std::fmt::Debug for RenderCtx<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RenderCtx")
            .field("frame", &self.frame)
            .finish_non_exhaustive()
    }
}

/// The two compiled-in tables, kept as values so [`RenderCtx::production`] can borrow them.
const PRODUCTION_CATALOG: ConstCatalog = CONST_CATALOG;
const PRODUCTION_ARRANGEMENTS: ConstArrangements = CONST_ARRANGEMENTS;

impl RenderCtx<'static> {
    /// The compiled-in tables at the canonical width — what every production seat renders through.
    #[must_use]
    pub fn production() -> Self {
        RenderCtx::new(&PRODUCTION_CATALOG, &PRODUCTION_ARRANGEMENTS)
    }
}

impl<'a> RenderCtx<'a> {
    /// A context over caller-chosen tables at the canonical width — `dorc-loom`'s seat, where the
    /// tables are its editable mirrors.
    #[must_use]
    pub fn new(catalog: &'a dyn CatalogLookup, arrangements: &'a dyn ArrangementLookup) -> Self {
        RenderCtx {
            catalog,
            arrangements,
            frame: weft::Frame::of_width(CANONICAL_TRANSCRIPT_WIDTH.into()),
        }
    }

    /// The same tables in a box `width` columns wide.
    #[must_use]
    pub fn at_width(self, width: usize) -> Self {
        RenderCtx {
            frame: weft::Frame::of_width(width.into()),
            ..self
        }
    }

    /// The same tables in a box whose leftmost `indent` columns are already spent — a seat that
    /// owns geometry (a lint report's finding list) hands one of these, so the indent is laid out
    /// THROUGH the render rather than glued on in front of it. Bytes glued on in front are the
    /// ones a wrap cannot see.
    #[must_use]
    pub fn inset(self, indent: usize) -> Self {
        RenderCtx {
            frame: self.frame.inset(indent),
            ..self
        }
    }

    /// The diagnostic prose table.
    #[must_use]
    pub fn catalog(&self) -> &dyn CatalogLookup {
        self.catalog
    }

    /// The chrome prose table.
    #[must_use]
    pub fn arrangements(&self) -> &dyn ArrangementLookup {
        self.arrangements
    }

    /// The box to lay out into.
    #[must_use]
    pub fn frame(&self) -> &weft::Frame {
        &self.frame
    }
}
