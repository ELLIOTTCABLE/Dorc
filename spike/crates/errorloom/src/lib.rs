//! errorloom — the transcript-case prose pipeline.
//!
//! errorloom makes the executable transcript case the authoring surface for a
//! CLI tool's user-facing prose (`282:rul-transcript-is-the-authoring-surface`):
//! authors edit what a user actually sees, and the compiled prose catalog is
//! DERIVED from those edits. This crate is the layer-1 transport engine: given a
//! machine-produced *tagged render* (bytes plus a [`Span`] map classifying every
//! run) and an author's *edited* text, it word-diffs the two, attributes each
//! change through the span map, re-holes instantiated param values, and yields
//! per-field prose edits — or a blunt [`Refusal`] (`282` §5).
//!
//! It is generic over an opaque consumer key (see [`ConsumerKey`]); Dorc is the
//! first consumer, but the crate holds no Dorc types. The container/runner,
//! bless orchestration, git trait, and CLI are separate layers (`28A` §1).
//!
//! Status: pre-1.0, `publish = false`. The one hard-tested guarantee (`282` §5):
//! an edit confined to one template region round-trips exactly, modulo
//! whitespace normalization.

use std::fmt::Debug;

mod diff;
mod promote;
mod prose;
mod span;

pub use crate::promote::{
    AttributedToken, ParamTables, ParamValues, PromoteOutcome, Refusal, RefusalClass, promote,
};
pub use crate::prose::{
    FieldTemplate, Fragment, Paragraph, ParamName, Prose, Token, Word, tokenize,
};
pub use crate::span::{ArrangementSlug, Region, Span, TaggedRender, TaggedRenderError};

/// What a consumer's opaque field key must satisfy. errorloom groups, sorts, and
/// compares by the key but never inspects it (Dorc's key is `(code, field)`).
/// The blanket impl covers any suitable type.
pub trait ConsumerKey: Clone + Ord + Debug {}

impl<T: Clone + Ord + Debug> ConsumerKey for T {}

#[cfg(test)]
mod tests;
