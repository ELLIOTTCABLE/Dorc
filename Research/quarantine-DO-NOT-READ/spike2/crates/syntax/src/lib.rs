//! `dorc-syntax` — a hand-rolled recursive-descent parser for the narrow slice of
//! POSIX sh that the Dorc analyzer models. String in → arena [`Ast`] + diagnostics
//! out, deterministic and panic-free (`inv-determinism`, `inv-no-throw`).
//!
//! The grammar is grown **demand-driven**: it parses only the constructs the
//! analyzer currently exercises (start: the `pi-webhost` book), and emits
//! [`ast::NodeKind::Unsupported`] for everything else (`inv-top-reject`). The
//! parser is intentionally *boring* — its only job is to hand the analyzer a
//! faithful, lossless-enough tree (see `ast` for the shape invariants).

#![forbid(unsafe_code)]

pub mod ast;
mod lexer;
mod parser;

pub use ast::{
    AndOrOp, Ast, AstBuilder, CaseArm, ElseIf, Node, NodeKind, RedirOp, RedirTarget,
    UnsupportedReason, Word, WordPart,
};

use dorc_core::Carrier;

/// Parse sh source into an arena AST paired with diagnostics.
///
/// Never panics on malformed input (`inv-no-throw`): unsupported or malformed
/// constructs become [`NodeKind::Unsupported`] nodes plus `Error` diagnostics, so
/// downstream stages can still surface *unrelated* problems. Pure and deterministic
/// (`inv-determinism`): same bytes in ⇒ same arena + diagnostics out, no I/O.
#[must_use]
pub fn parse(src: &str) -> Carrier<Ast> {
    parser::parse(src)
}

// Unit tests intentionally omitted: `parse` is a thin re-export of `parser::parse`,
// and the brutal integration suite (`tests/parse.rs`) covers totality, ⊤-reject,
// quoting, idioms, determinism, and the fixture shape — a lib smoke-test here would
// only duplicate it (test value-pass, 2026-06).
