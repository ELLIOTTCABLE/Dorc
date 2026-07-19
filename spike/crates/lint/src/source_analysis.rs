//! `source-analysis-diagnostics` (`27R` §2 item-1; rung-book — `27R` §8b): run the existing pure
//! pipeline prefix (`parse → cfg`) over each file with NO probe results and NO world, and surface
//! the accumulated `Carrier` diagnostics as findings. Nearly free, and the sanctioned direction —
//! the structured diagnostic API is the design-for-keeps exception (`diag-api-design-for-keeps`),
//! so building on it is correct. This is `27R` §3's factoring made concrete: the same diagnostics
//! that `plan`/`apply` emit with MORE inputs, emitted here with fewer (no world) — every pass whose
//! inputs exist fires; passes needing probe facts simply never run (they are not stubbed or faked).

use crate::finding::{Finding, RemapFidelity, SourceStatus};
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
            let parsed = dorc_syntax::parse(&file.src);
            let cfg = dorc_analysis::cfg::build(&parsed.value);
            for diag in parsed.diags.iter().chain(cfg.diags.iter()) {
                out.push(diag_to_finding(&file.path, &file.src, diag, self.name()));
            }
        }
        SourceStatus::Ran
    }
}

/// Lower one engine `Diagnostic` (the `dn-7` legacy stream both `parse` and `cfg` emit) into a lint
/// [`Finding`]. The span resolves to a 1-based `(line, col)` via `dorc_core::diag::line_col`
/// (rul24-lineno-identity — the SOURCE line space); a span-less diagnostic (the pre-CFG codes) yields
/// a whole-file finding (`line: None`). Native findings are always `RemapFidelity::Exact` (real span).
fn diag_to_finding(path: &str, src: &str, diag: &dorc_core::Diag, source: &'static str) -> Finding {
    let (line, col) = match diag.primary.span() {
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
        // The one severity vocabulary (`27V` §3 rider-d): a native finding carries the engine's own
        // `core::Severity` verbatim — no remap, the swap the registry-thin design was built for.
        severity: diag.severity(),
        source,
        code: diag.code.slug().to_owned(),
        // The catalog-rendered message (default interner — no payload resolves an interned handle;
        // MINIMAL re-bridge, `27V`).
        message: dorc_core::diag::render_body(diag, &dorc_core::Interner::default()),
        remap: RemapFidelity::Exact,
    }
}
