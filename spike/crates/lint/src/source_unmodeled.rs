//! `source-unmodeled-inventory` (`27R` §2 item-2; rung-book): the static half of the plan-time hint
//! machinery. Per book, the unmodeled ⊤-walls (`inv-top-reject` nodes) and the first-wall position —
//! the `27R` §3 demonstrator (plan/apply-adjacent reasoning with ZERO probe input). A ⊤-wall degrades
//! Dorc from "full elision" to "runtime guard" for downstream commands (the poison-wall, DESIGN.md).
//! "Unmodeled" is NARROW here: a ⊤-node mints from a construct the PARSER does not model, or from
//! the nesting bound — never from an ordinary command nobody wrote an oracle for, which goes opaque
//! in the effect lattice and is invisible to this census. A wall names a construct to respell.
//!
//! SPIKE SCOPE (`churn-avoidance-disclosure`): the PRECISE "downstream MODELED sites each wall
//! degrades" count (`27R` §2 item-2) needs the effect classification threaded with the loaded
//! oracles — deferred as `seam-unmodeled-degradation-count`. This light form reports the wall count,
//! the first-wall line, and an APPROXIMATE downstream-command count (all downstream commands, not
//! only the modeled ones), clearly framed so it never over-claims.

use dorc_analysis::cfg::CfgNodeKind;

use crate::finding::{Finding, FrameChoice, NativeDiag, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The unmodeled-wall inventory source. Deterministic (pure over the source bytes).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct UnmodeledInventory;

impl LintSource for UnmodeledInventory {
    fn name(&self) -> &'static str {
        "unmodeled-inventory"
    }

    fn describe_arrangement(&self) -> &'static str {
        "lint-source-unmodeled-inventory"
    }

    fn rung(&self) -> Rung {
        Rung::Book
    }

    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        for file in ctx.files {
            let parsed = dorc_syntax::parse(&file.src);
            let cfg = dorc_analysis::cfg::build(&parsed.value);
            let mut wall_spans: Vec<dorc_core::Span> = Vec::new();
            let mut leaf_offsets: Vec<u32> = Vec::new();
            for (id, node) in cfg.value.iter() {
                let span = parsed.value.node(node.ast).span;
                match node.kind {
                    CfgNodeKind::Top => wall_spans.push(span),
                    CfgNodeKind::Command
                        if !cfg.value.is_expansion_internal(id) && !cfg.value.in_loop_body(id) =>
                    {
                        leaf_offsets.push(span.lo.0);
                    }
                    _ => {}
                }
            }
            let Some(&first) = wall_spans.iter().min_by_key(|span| span.lo.0) else {
                continue; // no walls ⇒ nothing to inventory (a clean book stays silent here)
            };
            let first_wall = first.lo.0;
            let wall_count = wall_spans.len();
            let downstream = leaf_offsets.iter().filter(|&&o| o > first_wall).count();
            let (line, col) = dorc_aid::diag::line_col(&file.src, first_wall as usize);
            let diag = dorc_aid::Diag::new(
                dorc_aid::diag::DiagCode::UnmodeledWallInventory(
                    dorc_aid::diag::UnmodeledWallInventory {
                        wall_count,
                        wall_word: if wall_count == 1 { "wall" } else { "walls" },
                        downstream,
                    },
                ),
                first,
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
        SourceStatus::Ran
    }
}
