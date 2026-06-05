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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_is_total_and_does_not_panic() {
        // The contract that must survive the real implementation (`inv-no-throw`):
        // parse() always returns and the root resolves, even on hostile input.
        // (The brutal totality table lives in tests/parse.rs.)
        for src in ["", "echo hi", "$(((", "\u{0}\u{0}", "if then fi |||"] {
            let parsed = parse(src);
            let _ = parsed.value.node(parsed.value.root());
        }
    }

    #[test]
    fn parse_well_formed_command_is_clean() {
        // A trivially-modeled command must parse with no diagnostics and a single
        // top-level item — the stub used to emit a `parse-unimplemented` warning,
        // so this pins that the real parser replaced it.
        let parsed = parse("echo hi");
        assert!(!parsed.has_errors());
        assert!(parsed.diags.is_empty(), "no spurious diagnostics: {:?}", parsed.diags);
        match &parsed.value.node(parsed.value.root()).kind {
            NodeKind::Script { items } => assert_eq!(items.len(), 1),
            other => panic!("root not Script: {other:?}"),
        }
    }
}
