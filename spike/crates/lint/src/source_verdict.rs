//! `source-verdict-body-status-flattening` (`27R` §2 item-3; rung-file): the oracle-contract §3
//! mechanicals inside verdict-bearing (`__is_converged`) bodies. FALSIFICATION-FIRST subset only
//! (`rul-unprovable-rides-the-vouch` posture — the check may UNDER-report, never gate): a body ending
//! in a PIPELINE answers with the tail's status, which may not be the tool-under-description's
//! (oracle-contract §3 "Mind pipeline tails"). That flattens `≥2 = cannot-say` and `1 = complement`
//! into whatever the tail produced — a verdict-correctness hazard.
//!
//! SPIKE SCOPE (`churn-avoidance-disclosure`): the sibling `!`-on-the-answering-status and `|| true`
//! flattening mechanicals are NOT detected here — those constructs are OUT of the verdict dialect, so
//! they never lift and instead surface through `source-analysis-diagnostics` as `predict-out-of-dialect`
//! give-ups. A dedicated precise lint for them needs lexer-level bang/or-list support the dialect does
//! not expose; named `seam-verdict-bang-and-ortrue-flattening`.

use dorc_core::Interner;
use dorc_oracle::predict::Stmt;
use dorc_oracle::verdict::VerdictSet;

use crate::finding::{Finding, FrameChoice, NativeDiag, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The verdict-body flattening source. Deterministic (pure over the oracle bytes).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct VerdictBodyFlattening;

impl LintSource for VerdictBodyFlattening {
    fn name(&self) -> &'static str {
        "verdict-body"
    }

    fn describe_arrangement(&self) -> &'static str {
        "lint-source-verdict-body"
    }

    fn rung(&self) -> Rung {
        Rung::File
    }

    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        for oracle in ctx.oracles {
            let mut interner = Interner::default();
            let verdicts = VerdictSet::lift(&mut interner, &oracle.src).value;
            let providers: Vec<_> = verdicts.providers().collect();
            for provider in providers {
                let Some(verdict) = verdicts.get(provider) else {
                    continue;
                };
                if let Some(span) = terminal_pipeline_span(&verdict.body) {
                    let (line, col) = dorc_aid::diag::line_col(&oracle.src, span.lo.0 as usize);
                    let diag = dorc_aid::Diag::new(
                        dorc_aid::diag::DiagCode::VerdictTerminalPipeline(
                            dorc_aid::diag::VerdictTerminalPipeline,
                        ),
                        span,
                    );
                    out.push(Finding {
                        path: oracle.path.clone(),
                        line: Some(u32::try_from(line).unwrap_or(u32::MAX)),
                        col: Some(u32::try_from(col).unwrap_or(u32::MAX)),
                        severity: diag.severity(),
                        source: self.name(),
                        code: diag.code.slug().to_owned(),
                        message: dorc_aid::diag::render_body(&diag, &Interner::default()),
                        remap: RemapFidelity::Exact,
                        provenance: Some(NativeDiag {
                            diag,
                            source: oracle.src.clone(),
                        }),
                        frame: FrameChoice::Compact,
                    });
                }
            }
        }
        SourceStatus::Ran
    }
}

/// The span of the body's last STATUS-BEARING statement iff it is a terminal PIPELINE (the
/// falsification-first case). Status-bearing = a command / case / if / while (an `Assign`/`Shift`/
/// `Annotation` does not set the answering rc). Returns `None` unless that last statement is a
/// `Command` whose `pipeline` flag is set — conservative (never descends into `case`/`if` arms, so it
/// may under-report, per the posture).
fn terminal_pipeline_span(body: &[Stmt]) -> Option<dorc_core::Span> {
    let last_status = body.iter().rev().find(|s| {
        matches!(
            s,
            Stmt::Command(_) | Stmt::Case { .. } | Stmt::If { .. } | Stmt::While { .. }
        )
    })?;
    match last_status {
        Stmt::Command(cmd) if cmd.pipeline => Some(cmd.span),
        _ => None,
    }
}
