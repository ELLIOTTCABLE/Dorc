//! `check` — the command-keyed `check()` contract (19H §2 / 202 §1 face-check).
//!
//! An oracle ships one sh function per command-family, `<provider>__check`, that
//! argparses the command the way the real tool does, inline-annotates which value
//! is which named kind, and is itself the read-only probe body. This module is the
//! *static half* (202 §2.7's "read twice"): a dedicated mini-parser for the
//! constrained contract dialect plus a concrete evaluator that traces a known argv
//! through a check's argparse to its kind-annotation, yielding a [`Resolution`].
//!
//! # Why a separate parser (`adj-dialect-parser`, note 203 §4)
//!
//! The contract dialect is *not* arbitrary sh and is *not* the book dialect. The
//! book parser (`dorc-syntax`) ⊤-rejects loops by design; a `check()`'s argparse
//! *needs* `while`. Extending the book parser would drag its CFG-lowering, errexit
//! and consumption machinery along. Instead this module owns a small parser whose
//! grammar *is* the contract: anything it cannot parse is a loud per-function lift
//! failure (`inv-no-throw`: a diagnostic, never a panic; the file's other checks
//! still lift). The dialect parser structurally enforces "the dialect is NOT
//! arbitrary sh" (19G §2) — outside-dialect input fails to parse ⇒ unresolvable.
//!
//! # Soundness posture (`inv-kfail`, both directions)
//!
//! The evaluator never guesses. A flag the argparse does not consume, an arm `case`
//! cannot select concretely, an annotation whose value-position is not a positional
//! or known binding, a missing annotation, a budget overrun — every one is
//! [`Top`](Resolution::Top) with a reason. `Top` is always safe (the site stays
//! un-probeable and un-elidable). A *wrong* resolution is the disaster class (19H
//! §1.3: propagation-correctness has no floor), so every ambiguity biases to `Top`.
//!
//! `inv-referent-agnostic`: the evaluator resolves which **argv element** is the
//! entity (by tracing the oracle's own argparse); it never branches on what the
//! entity's text *means*. Kind strings are opaque coordination handles.

use dorc_core::{DiagCode, Diagnostic, Span};

mod ast;
mod eval;
mod lexer;
mod parser;

pub use ast::{Check, CheckSet};
pub use eval::{Resolution, Resolved, TopReason, evaluate};
pub use parser::lift_checks;

/// Diagnostic codes the dialect parser emits (greppable; `ch-catalog`).
const OUT_OF_DIALECT: DiagCode = DiagCode("check-out-of-dialect");
const UNTERMINATED: DiagCode = DiagCode("check-unterminated");

/// The conventional local variable name an oracle assigns the verb to (`verb=$1`,
/// 19H §2.1/§2.5). Recognizing it is a *structural convention in the oracle's own
/// code* (like the `__check` suffix or the annotation shape), NOT decoding entity
/// text — so it does not breach `inv-referent-agnostic`. Whether `verb` should be a
/// reserved dialect name is a `tc-*`-shaped cross-cutting question (flagged in the
/// build report); the conservative local choice is: if the oracle does not use this
/// name, the [`Resolution`] simply carries no verb (always safe).
const VERB_BINDING: &str = "verb";

/// A per-function lift failure: the named function is in the file but its body is
/// out of dialect. Fail-soft (`inv-no-throw`): the function contributes no [`Check`]
/// and the rest of the file still lifts.
fn lift_failure(code: DiagCode, span: Option<Span>, message: impl Into<String>) -> Diagnostic {
    Diagnostic::error(code, span, message)
}
