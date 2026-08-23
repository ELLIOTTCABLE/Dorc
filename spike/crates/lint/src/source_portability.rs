//! `source-loop-brace-range` (`30Qe:fruit-loop-does-not-loop-lint`; rung-file): a plain-sh `for`
//! whose entire word-list is one bash/zsh brace-RANGE (`for i in {1..10}`) does not expand under
//! POSIX sh (dash, posh) -- the range stays one literal word, so the loop body runs exactly ONCE,
//! with the range text bound to the loop variable, never N times. A real, repeated corpus footgun
//! (k3s and the Kubernetes docs both ship it).
//!
//! Pure syntax, per the `26K` sect0a boundary law: parses the file with the SAME `dorc_syntax`
//! parser every other stage uses, walks the resulting AST once, and touches no
//! `dorc_analysis`/`dorc_plan` machinery -- no CFG, no value-flow, no new walking primitive.

use dorc_syntax::ast::NodeKind;

use crate::finding::{Finding, FrameChoice, NativeDiag, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The brace-range-for-loop source. Deterministic (pure over file bytes).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct LoopBraceRange;

impl LintSource for LoopBraceRange {
    fn name(&self) -> &'static str {
        "loop-brace-range"
    }

    fn describe_arrangement(&self) -> &'static str {
        "lint-source-loop-brace-range"
    }

    fn rung(&self) -> Rung {
        Rung::File
    }

    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        for file in ctx.files {
            let ast = dorc_syntax::parse(&file.src).value;
            for (_, node) in ast.iter() {
                let NodeKind::ForLoop { words, .. } = &node.kind else {
                    continue;
                };
                let [only] = words.as_slice() else {
                    continue;
                };
                let word_node = ast.node(*only);
                let NodeKind::Word { parts } = &word_node.kind else {
                    continue;
                };
                let word = dorc_syntax::ast::Word { parts };
                let Some(range) = word.as_literal().filter(|w| is_brace_range(w)) else {
                    continue;
                };
                let span = word_node.span;
                let (line, col) = dorc_aid::diag::line_col(&file.src, span.lo.0 as usize);
                let diag = dorc_aid::Diag::new(
                    dorc_aid::diag::DiagCode::ForLoopBraceRangeRunsOnce(
                        dorc_aid::diag::ForLoopBraceRangeRunsOnce {
                            range: range.to_owned(),
                        },
                    ),
                    span,
                );
                out.push(Finding {
                    path: file.path.clone(),
                    line: Some(u32::try_from(line).unwrap_or(u32::MAX)),
                    col: Some(u32::try_from(col).unwrap_or(u32::MAX)),
                    severity: diag.severity(),
                    source: self.name(),
                    code: diag.code.slug().to_owned(),
                    message: dorc_aid::diag::render_body(&diag, &dorc_core::Interner::default()),
                    remap: RemapFidelity::Exact,
                    provenance: Some(NativeDiag {
                        diag,
                        source: file.src.clone(),
                    }),
                    frame: FrameChoice::Compact,
                });
            }
        }
        SourceStatus::Ran
    }
}

/// Is `word` a bash/zsh brace-RANGE (`{1..10}`, `{10..1..-2}`) -- POSIX sh never expands it, so a
/// for-list containing exactly this one word iterates ONCE, over the literal text. Sequence
/// brace-LISTS (`{a,b,c}`) are a different, already-visibly-multi-word construct under bash and
/// out of scope here -- this checks only the `..`-range shape.
fn is_brace_range(word: &str) -> bool {
    let Some(inner) = word.strip_prefix('{').and_then(|s| s.strip_suffix('}')) else {
        return false;
    };
    let parts: Vec<&str> = inner.split("..").collect();
    if !(2..=3).contains(&parts.len()) {
        return false;
    }
    parts.iter().all(|p| {
        let digits = p.strip_prefix('-').unwrap_or(p);
        !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
    })
}

#[cfg(test)]
mod tests {
    use super::is_brace_range;

    #[test]
    fn recognizes_ranges_and_rejects_lookalikes() {
        assert!(is_brace_range("{1..10}"));
        assert!(is_brace_range("{10..1}"));
        assert!(is_brace_range("{1..10..2}"));
        assert!(is_brace_range("{-5..5}"));
        assert!(!is_brace_range("{a,b,c}"), "a brace LIST, not a range");
        assert!(!is_brace_range("$var"), "not brace syntax at all");
        assert!(!is_brace_range("{1..}"), "an incomplete range");
        assert!(!is_brace_range("{1..2..3..4}"), "too many `..` segments");
    }
}
