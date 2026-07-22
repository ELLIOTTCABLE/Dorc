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
//! - the nested editable-section transport ([`EditableRender`]);
//! - structure regeneration and the [`fixpoint_check`] gate over [`CaseRenderer`].
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

#![warn(missing_docs)]

use std::fmt::Debug;

mod bless;
mod container;
mod editable;
mod runner;

pub use crate::bless::{
    BlessError, BlessResult, CaseFile, CaseRenderer, FakeGit, Git, GitError, fixpoint_check,
    structure_bless,
};
pub use crate::container::{
    Case, CaseError, CaseReadError, Frontmatter, FrontmatterValue, MAX_CASE_BYTES,
    MAX_REPLAY_BLOCKS, MAX_REPLAY_COMMAND_BYTES, MAX_REPLAY_OUTPUT_BYTES, MAX_SECTION_BYTES,
    MAX_SECTION_COUNT, REPLAY_SECTION, ReplayBlock, ReplaySection, Section, read_case,
    read_case_text,
};
pub use crate::editable::{
    AlignmentLimitMetadata, EditRefusal, EditRefusalClass, EditTransport, EditableFragment,
    EditableRender, EditableSection, RenderComponent, SectionEdit, transport_edit,
    transport_edit_allow_removal,
};
pub use crate::runner::{
    Drift, MAX_CAPTURE_BYTES, ReplayCapture, ReplayContext, ReplayDriver, ReplayResult, RunEnv,
    RunError, RunReport, bless_structure, check_run, drive_case, execute_generic, run_case,
};

/// What a consumer's opaque field key must satisfy. errorloom groups, sorts, and
/// compares by the key but never inspects it (Dorc's key is `(code, field)`).
/// The blanket impl covers any suitable type.
pub trait ConsumerKey: Clone + Ord + Debug {}

impl<T: Clone + Ord + Debug> ConsumerKey for T {}

#[cfg(test)]
mod container_tests;
#[cfg(test)]
mod tests;
