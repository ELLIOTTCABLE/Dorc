//! errorloom — executable transcript cases as the authoring surface for prose.
//!
//! errorloom makes the executable transcript case the authoring surface for a
//! CLI tool's user-facing prose (`282:rul-transcript-is-the-authoring-surface`):
//! authors edit what a user actually sees, and the compiled prose catalog is
//! DERIVED from those edits. It is generic over an opaque consumer key (see
//! [`ConsumerKey`]); Dorc is the first consumer, but the crate holds no Dorc types.
//!
//! The pieces (`28A` §1):
//! - the [`Case`] container — txtar sections with flat-YAML frontmatter;
//! - the replay runner ([`run_case`], [`check_run`], [`bless_structure`]) — runs a
//!   case's `$ ` command blocks in a caller-injected sandbox ([`RunEnv`]);
//! - the transport engine ([`promote`]) — word-diffs a [`TaggedRender`] against an
//!   edited transcript, attributes each change, and yields per-field prose edits or
//!   a blunt [`Refusal`];
//! - the bless orchestration ([`prose_bless`], [`structure_bless`],
//!   [`fixpoint_check`]) over the [`Consumer`] and [`Git`] traits — the two bless
//!   modes, their exclusivity, and the CI fixpoint gate.
//!
//! The one hard-tested guarantee (`282` §5): an edit confined to one template
//! region round-trips exactly, modulo whitespace normalization.
//!
//! Status: pre-1.0, `publish = false`. Sharp edges are intentional
//! (`282:rul-internal-tool-sharp-edges`); refusals are blunt.
//!
//! # Examples
//! Parse a case and run the required-token coherence gate — every replay block
//! must surface the frontmatter `code` value:
//! ```
//! use errorloom::Case;
//!
//! # fn main() -> Result<(), errorloom::CaseError> {
//! let text = "---\ncode: motd-refused\n---\n\
//!             -- replay --\n\
//!             $ mytool explain motd-refused\n\
//!             error[motd-refused]: refusing to elide the heredoc\n";
//! let case = Case::parse(text)?;
//! assert_eq!(case.frontmatter().scalar("code"), Some("motd-refused"));
//! case.check_hygiene(Some("code"))?;
//! # Ok(())
//! # }
//! ```

// Fully documented today; fail the build if a new public item lacks docs (taste-F8).
#![warn(missing_docs)]

use std::fmt::Debug;

mod bless;
mod container;
mod diff;
mod promote;
mod prose;
mod runner;
mod span;

pub use crate::bless::{
    BlessError, BlessMode, BlessResult, CaseFile, Consumer, FakeGit, Git, GitError, ModeRefusal,
    SubprocessGit, TaggedBaseline, fixpoint_check, infer_mode, prose_bless, structure_bless,
};
pub use crate::container::{
    Case, CaseError, Frontmatter, FrontmatterValue, REPLAY_SECTION, ReplayBlock, ReplaySection,
    Section,
};
pub use crate::promote::{
    AttributedToken, ParamTables, ParamValues, PromoteOutcome, Refusal, RefusalClass, promote,
};
pub use crate::prose::{
    FieldTemplate, Fragment, Paragraph, ParamName, Prose, Token, Word, tokenize,
};
pub use crate::runner::{
    Drift, ReplayCapture, RunEnv, RunError, RunReport, bless_structure, check_run, run_case,
};
pub use crate::span::{ArrangementSlug, InstanceId, Region, Span, TaggedRender, TaggedRenderError};

/// What a consumer's opaque field key must satisfy. errorloom groups, sorts, and
/// compares by the key but never inspects it (Dorc's key is `(code, field)`).
/// The blanket impl covers any suitable type.
pub trait ConsumerKey: Clone + Ord + Debug {}

impl<T: Clone + Ord + Debug> ConsumerKey for T {}

#[cfg(test)]
mod container_tests;
#[cfg(test)]
mod tests;
