//! One deterministic production-shaped lint invocation for transcript replay.

use dorc_core::tagged::RenderParts;

use crate::{LintInput, LintOptions, LintReport, NoToolsRunner, lint, render};

/// Explicit policy for a replayable lint source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePolicy {
    /// External tools are disabled so replay never consults PATH or spawns a process.
    pub tools_enabled: bool,
}

/// The one report and exact tagged human render produced for one materialized source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionLintResult {
    report: LintReport,
    human: RenderParts,
}

impl ProductionLintResult {
    /// The deterministic report used by the CLI for its exit policy.
    #[must_use]
    pub fn report(&self) -> &LintReport {
        &self.report
    }

    /// Exact human bytes with renderer-owned editable provenance.
    #[must_use]
    pub fn human(&self) -> &RenderParts {
        &self.human
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
    let human = render::render_human_parts(&report);
    ProductionLintResult { report, human }
}
