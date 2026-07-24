//! `source-unmodeled-inventory` (`27R` §2 item-2; rung-book): the static half of the plan-time hint
//! machinery. Per book, the unmodeled ⊤-walls (`inv-top-reject` nodes) and the first-wall position —
//! the `27R` §3 demonstrator (plan/apply-adjacent reasoning with ZERO probe input). A ⊤-wall degrades
//! Dorc from "full elision" to "runtime guard" for downstream commands (the poison-wall, DESIGN.md),
//! so surfacing them tells an oracle author exactly where an oracle would unlock elision.
//!
//! SPIKE SCOPE (`churn-avoidance-disclosure`): the PRECISE "downstream MODELED sites each wall
//! degrades" count (`27R` §2 item-2) needs the effect classification threaded with the loaded
//! oracles — deferred as `seam-unmodeled-degradation-count`. This light form reports the wall count,
//! the first-wall line, and an APPROXIMATE downstream-command count (all downstream commands, not
//! only the modeled ones), clearly framed so it never over-claims.

use dorc_analysis::cfg::CfgNodeKind;

use crate::finding::{Finding, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The unmodeled-wall inventory source. Deterministic (pure over the source bytes).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct UnmodeledInventory;

impl LintSource for UnmodeledInventory {
    fn name(&self) -> &'static str {
        "unmodeled-inventory"
    }

    fn describe(&self) -> &'static str {
        "per-book ⊤-wall inventory (first wall + downstream degradation)"
    }

    fn rung(&self) -> Rung {
        Rung::Book
    }

    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        for file in ctx.files {
            let parsed = dorc_syntax::parse(&file.src);
            let cfg = dorc_analysis::cfg::build(&parsed.value);
            let mut wall_offsets: Vec<u32> = Vec::new();
            let mut leaf_offsets: Vec<u32> = Vec::new();
            for (id, node) in cfg.value.iter() {
                let span = parsed.value.node(node.ast).span;
                match node.kind {
                    CfgNodeKind::Top => wall_offsets.push(span.lo.0),
                    CfgNodeKind::Command
                        if !cfg.value.is_expansion_internal(id) && !cfg.value.in_loop_body(id) =>
                    {
                        leaf_offsets.push(span.lo.0);
                    }
                    _ => {}
                }
            }
            let Some(&first_wall) = wall_offsets.iter().min() else {
                continue; // no walls ⇒ nothing to inventory (a clean book stays silent here)
            };
            let wall_count = wall_offsets.len();
            let downstream = leaf_offsets.iter().filter(|&&o| o > first_wall).count();
            let (line, col) = dorc_aid::diag::line_col(&file.src, first_wall as usize);
            let wall_word = if wall_count == 1 { "wall" } else { "walls" };
            let message = format!(
                "{wall_count} unmodeled ⊤-{wall_word} in this book; the first is here. Downstream \
                 commands (~{downstream} leaf site(s) after it) lose full-elision and fall back to \
                 runtime guards until each wall's tool has an oracle."
            );
            out.push(Finding {
                path: file.path.clone(),
                line: Some(u32::try_from(line).unwrap_or(u32::MAX)),
                col: Some(u32::try_from(col).unwrap_or(u32::MAX)),
                severity: dorc_aid::Severity::Note,
                source: self.name(),
                code: "unmodeled-wall-inventory".to_owned(),
                message,
                remap: RemapFidelity::Exact,
                provenance: None,
            });
        }
        SourceStatus::Ran
    }
}
