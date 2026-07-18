//! `source-analysis-diagnostics` (`27R` §2 item-1; rung-book — `27R` §8b): run the existing pure
//! pipeline prefix (`parse → cfg`) over each file with NO probe results and NO world, and surface
//! the accumulated `Carrier` diagnostics as findings. Nearly free, and the sanctioned direction —
//! the structured diagnostic API is the design-for-keeps exception (`diag-api-design-for-keeps`),
//! so building on it is correct. This is `27R` §3's factoring made concrete: the same diagnostics
//! that `plan`/`apply` emit with MORE inputs, emitted here with fewer (no world) — every pass whose
//! inputs exist fires; passes needing probe facts simply never run (they are not stubbed or faked).

use dorc_core::{Interner, Severity};

use crate::finding::{Finding, LintSeverity, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The analysis-diagnostics source. Deterministic (`inv-determinism`): the pipeline is a pure
/// function of the source bytes, and findings are emitted in `(file, diagnostic)` order then
/// globally re-sorted by `crate::lint`.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct AnalysisDiagnostics;

impl LintSource for AnalysisDiagnostics {
    fn name(&self) -> &'static str {
        "analysis-diagnostics"
    }

    fn describe(&self) -> &'static str {
        "engine parse/cfg diagnostics over each file (no world)"
    }

    fn rung(&self) -> Rung {
        Rung::Book
    }

    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        for file in ctx.files {
            // A fresh interner per file: the sources are independent (nothing keys symbols across
            // them), so this keeps the crate a pure function of its inputs without a shared mutable.
            let mut interner = Interner::default();
            let parsed = dorc_syntax::parse(&file.src);
            let cfg = dorc_analysis::cfg::build(&parsed.value);
            for diag in parsed.diags.iter().chain(cfg.diags.iter()) {
                out.push(diag_to_finding(&file.path, &file.src, diag, self.name()));
            }
            // The interner is not consumed by parse/cfg here (they take `&str`); held only to make
            // the "each source mints its own" discipline explicit and future-proof.
            let _ = &mut interner;
        }
        SourceStatus::Ran
    }
}

/// Lower one engine `Diagnostic` (the `dn-7` legacy stream both `parse` and `cfg` emit) into a lint
/// [`Finding`]. The span resolves to a 1-based `(line, col)` via `dorc_core::diag::line_col`
/// (rul24-lineno-identity — the SOURCE line space); a span-less diagnostic (the pre-CFG codes) yields
/// a whole-file finding (`line: None`). Native findings are always `RemapFidelity::Exact` (real span).
fn diag_to_finding(
    path: &str,
    src: &str,
    diag: &dorc_core::Diagnostic,
    source: &'static str,
) -> Finding {
    let (line, col) = match diag.span {
        Some(span) => {
            let (l, c) = dorc_core::diag::line_col(src, span.lo.0 as usize);
            (
                Some(u32::try_from(l).unwrap_or(u32::MAX)),
                Some(u32::try_from(c).unwrap_or(u32::MAX)),
            )
        }
        None => (None, None),
    };
    Finding {
        path: path.to_owned(),
        line,
        col,
        severity: map_severity(diag.severity),
        source,
        code: diag.code.0.to_owned(),
        message: diag.message.clone(),
        remap: RemapFidelity::Exact,
    }
}

/// Map the engine's three-value `core::Severity` onto the lint tier: `Note` is an advisory
/// disclosure ⇒ `Info` (never gates), `Warning ⇒ Warn`, `Error ⇒ Error`.
fn map_severity(sev: Severity) -> LintSeverity {
    match sev {
        Severity::Error => LintSeverity::Error,
        Severity::Warning => LintSeverity::Warn,
        Severity::Note => LintSeverity::Info,
    }
}
