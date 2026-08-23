//! `dorc-lint` — the `dorc lint` machinery (`27R` §1): an oracle-author-focused (not
//! oracle-exclusive) doctor/lint grab-bag, ala `brew doctor` / `mise doctor`.
//!
//! Shape (`27R` §1): a dumb [`LintSource`](source::LintSource) registry (trait + `Vec`, no discovery,
//! no config — `dir-registry-stays-dumb`); one finding model ([`Finding`](finding::Finding)); two
//! renderers ([`render`]); external-tool invocation behind ONE injected runner trait
//! ([`ExternalToolRunner`](runner::ExternalToolRunner), `dir-runner-is-the-di-seam`). Given the runner,
//! this crate is a pure, deterministic function of its inputs (`inv-determinism` posture): no clock,
//! RNG, filesystem, or network — the real subprocess impl lives at the cli edge only.
//!
//! ADVISORY-ONLY (`dir-no-license-plane-contact`): nothing here mints, widens, or influences a
//! claim/license/fact; lint never touches `core::claim`. `silence-licenses-nothing` runs both ways —
//! a lint-clean run licenses nothing either. And `dir-lint-never-probes`: lint contacts no hosts and
//! reads nothing but the files given (plus the sanctioned local read-only tool spawns at the edge).

#![forbid(unsafe_code)]

pub mod finding;
pub mod json;
pub mod production;
pub mod render;
pub mod runner;
pub mod source;

mod source_analysis;
mod source_external;
mod source_oracle_solo;
mod source_portability;
mod source_unmodeled;
mod source_verdict;

pub use finding::{
    Coverage, Finding, LintReport, RemapFidelity, SourceCoverage, SourceStatus, severity_token,
};
pub use production::{ProductionLintResult, SourcePolicy, lint_materialized_source};
pub use runner::{ExternalToolRunner, NoToolsRunner, ToolRun};
pub use source::{LintContext, LintInput, LintOptions, LintSource, Rung, registry};

/// Run every selected lint source over the inputs and return the sorted report (`27R` §1). Pure given
/// `runner` (`inv-determinism`): sources emit findings in a fixed order, then the whole set is sorted
/// by `(path, line, source, code)` (`dir-deterministic-output`). `only` restricts the run to the named
/// sources (`27R` §8 delta-named-sources-selectable — naming a source runs a subset); `None` runs all.
#[must_use]
pub fn lint(
    files: &[LintInput],
    oracles: &[LintInput],
    options: LintOptions,
    runner: &dyn ExternalToolRunner,
    only: Option<&[String]>,
) -> LintReport {
    let ctx = LintContext {
        files,
        oracles,
        options,
        runner,
    };
    let mut findings: Vec<Finding> = Vec::new();
    let mut sources: Vec<SourceCoverage> = Vec::new();
    for src in registry() {
        if let Some(names) = only
            && !names.iter().any(|n| n == src.name())
        {
            continue;
        }
        let status = src.run(&ctx, &mut findings);
        sources.push(SourceCoverage {
            name: src.name(),
            status,
        });
    }
    findings.sort_by_key(Finding::sort_key);
    LintReport {
        findings,
        coverage: Coverage {
            files: files.iter().map(|f| f.path.clone()).collect(),
            sources,
        },
    }
}

/// One registry entry's public descriptor for `--list-sources` (`27R` §8 delta-named-sources-selectable).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceInfo {
    /// The stable source name (also the value to pass as a subset selector).
    pub name: &'static str,
    /// The one-line description's ARRANGEMENT SLUG — the print seat resolves the words
    /// (`289:rul-arrangement-home-is-registry-plus-transcripts`).
    pub describe_arrangement: &'static str,
    /// The input rung it sits on (`27R` §8b): `file` or `book`.
    pub rung: &'static str,
}

/// Enumerate the registered sources for `--list-sources` (`27R` §8). Deterministic registry order.
#[must_use]
pub fn list_sources() -> Vec<SourceInfo> {
    registry()
        .iter()
        .map(|s| SourceInfo {
            name: s.name(),
            describe_arrangement: s.describe_arrangement(),
            rung: s.rung().label(),
        })
        .collect()
}
