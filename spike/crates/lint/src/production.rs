//! One deterministic production-shaped lint invocation for transcript replay.

use dorc_aid::{RenderCtx, tagged::RenderParts};

use crate::{LintInput, LintOptions, LintReport, NoToolsRunner, lint, render};

/// Explicit policy for a replayable lint source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePolicy {
    /// External tools are disabled so replay never consults PATH or spawns a process.
    pub tools_enabled: bool,
}

/// The one report produced for one materialized source, plus the render seat over it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionLintResult {
    report: LintReport,
}

impl ProductionLintResult {
    /// The deterministic report used by the CLI for its exit policy.
    #[must_use]
    pub fn report(&self) -> &LintReport {
        &self.report
    }

    /// Exact human bytes with renderer-owned editable provenance, through the caller's tables.
    ///
    /// The context is demanded HERE, at the render, rather than baked in when the report was
    /// produced: a stored render can only carry the tables whoever ran the lint happened to hold,
    /// and `dorc-loom` re-renders a lint-route case against its EDITED mirror before anything is
    /// rebuilt (`28L:rul-render-context-struct`). Rendering eagerly against the compiled-in catalog
    /// left promote publishing a lock and a transcript that disagreed
    /// (`28L:fnd-lint-route-rerender-reads-const-not-mirror`).
    #[must_use]
    pub fn human(&self, ctx: &RenderCtx<'_>) -> RenderParts {
        render::render_human_parts(ctx, &self.report)
    }
}

/// Run the production lint/oracle-validation pipeline over one exact source. The
/// source is both a lint target and an oracle candidate because this is the
/// author-facing `dorc lint oracle.sh` lane.
#[must_use]
pub fn lint_materialized_source(
    path: String,
    source: String,
    policy: SourcePolicy,
) -> ProductionLintResult {
    let input = LintInput { path, src: source };
    let report = lint(
        std::slice::from_ref(&input),
        std::slice::from_ref(&input),
        LintOptions {
            tools_enabled: policy.tools_enabled,
        },
        &NoToolsRunner,
        None,
    );
    ProductionLintResult { report }
}
