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

pub use ast::{
    AndOrOp, Ast, AstBuilder, CaseArm, ElseIf, Node, NodeKind, RedirOp, RedirTarget,
    UnsupportedReason, Word, WordPart,
};

use dorc_core::{Carrier, DiagCode, Diagnostic};

/// Parse sh source into an arena AST paired with diagnostics.
///
/// Never panics on malformed input (`inv-no-throw`): unsupported or malformed
/// constructs become [`NodeKind::Unsupported`] nodes plus `Error` diagnostics, so
/// downstream stages can still surface *unrelated* problems.
///
/// NOTE: stub. The lexer + recursive-descent body is delegated; this returns an
/// empty script so the workspace stays green until it lands.
#[must_use]
pub fn parse(src: &str) -> Carrier<Ast> {
    let mut builder = AstBuilder::default();
    let len = u32::try_from(src.len()).unwrap_or(u32::MAX);
    let root = builder.alloc(Node::script(Vec::new(), len));
    let ast = builder.finish(root);
    Carrier::new(
        ast,
        vec![Diagnostic::warning(
            DiagCode("parse-unimplemented"),
            None,
            "parser body not yet implemented (stub)",
        )],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_parse_is_total_and_does_not_panic() {
        // The contract that must survive the real implementation: parse() always
        // returns, even on hostile input.
        for src in ["", "echo hi", "$(((", "\u{0}\u{0}", "if then fi |||"] {
            let parsed = parse(src);
            // root resolves; no panic.
            let _ = parsed.value.node(parsed.value.root());
        }
    }
}
