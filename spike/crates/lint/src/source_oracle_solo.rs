//! `rung-oracle-solo` sources (`27R` §8b; UNLOCKED by `27S:seam-oracle-validate-factoring`): oracle
//! files linted with NO book present. Two sources ride the same factored surface —
//!
//! * `oracle-validate` — the book-free oracle-side validation (`dorc_oracle::validate`): the
//!   effect-map lift, per-file check-dialect lift, dual-peel + fold-entry coherence, munge-reservation
//!   lint, and the marker gate, lowered from structured [`dorc_core::Diag`]s to findings. The SAME
//!   diagnostics the cli routes to stderr, surfaced here for the author's hot loop.
//! * `oracle-declined-inventory` — the tier-1 authored-decline inventory (`27W` §3
//!   `rul-static-first-three-tier`; `AID-NEEDS:aid-authored-decline-classes`): each verdict body's
//!   per-arm `decline <class>` emissions, listed advisory (never gates, never probes).
//!
//! Advisory-only (`dir-no-license-plane-contact`): neither mints/reads a license; the decline classes
//! route AID only.

use dorc_core::Interner;
use dorc_core::evidence::DeclineClass;
use dorc_oracle::verdict::VerdictSet;

use crate::finding::{Finding, NativeDiag, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The book-free oracle-validation source: lowers `dorc_oracle::validate`'s stage-diags to findings.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct OracleValidate;

impl LintSource for OracleValidate {
    fn name(&self) -> &'static str {
        "oracle-validate"
    }

    fn describe(&self) -> &'static str {
        "book-free oracle-side validation (lift/check/coherence/reserved/marker)"
    }

    fn rung(&self) -> Rung {
        Rung::OracleSolo
    }

    fn run(&self, ctx: &LintContext<'_>, out: &mut Vec<Finding>) -> SourceStatus {
        let mut interner = Interner::default();
        let srcs: Vec<&str> = ctx.oracles.iter().map(|o| o.src.as_str()).collect();
        let validation = dorc_oracle::validate::validate(&mut interner, &srcs);
        for stage in &validation.stages {
            // A per-file stage frames into THAT oracle's path/source; a unit-wide stage (no single
            // file) reports against the first oracle's path as a stable, deterministic anchor.
            let anchor = match stage.file {
                Some(i) => ctx.oracles.get(i),
                None => ctx.oracles.first(),
            };
            let Some(o) = anchor else { continue };
            for diag in &stage.diags {
                out.push(diag_to_finding(&o.path, &o.src, diag, self.name()));
            }
        }
        for oracle in ctx.oracles {
            let verdicts = VerdictSet::lift(&mut interner, &oracle.src).value;
            for provider in verdicts.providers() {
                if let Some(verdict) = verdicts.get(provider) {
                    let (_, diags) = dorc_oracle::entry::lift_tolerance(verdict);
                    for diag in diags {
                        out.push(diag_to_finding(
                            &oracle.path,
                            &oracle.src,
                            &diag,
                            self.name(),
                        ));
                    }
                }
            }
        }
        SourceStatus::Ran
    }
}

/// Lower one oracle-validation [`dorc_core::Diag`] into a finding (the same bridge shape as
/// `source-analysis-diagnostics`): the span resolves to the source `(line, col)`; a spanless diag
/// yields a whole-file finding. Native ⇒ always `RemapFidelity::Exact`.
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
        severity: diag.severity(),
        source,
        code: diag.code.slug().to_owned(),
        message: dorc_core::diag::render_body(diag, &Interner::default()),
        remap: RemapFidelity::Exact,
        provenance: Some(NativeDiag {
            diag: diag.clone(),
            source: src.to_owned(),
        }),
    }
}

/// The tier-1 authored-decline inventory source (`27W` §3; `AID-NEEDS:aid-authored-decline-classes`).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct OracleDeclinedInventory;

impl LintSource for OracleDeclinedInventory {
    fn name(&self) -> &'static str {
        "oracle-declined-inventory"
    }

    fn describe(&self) -> &'static str {
        "authored decline classes per verdict arm (tier-1, oracle-solo)"
    }

    fn rung(&self) -> Rung {
        Rung::OracleSolo
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
                for arm in dorc_oracle::report::report_inventory(verdict) {
                    let (line, col) = dorc_core::diag::line_col(&oracle.src, arm.arm.lo.0 as usize);
                    out.push(Finding {
                        path: oracle.path.clone(),
                        line: Some(u32::try_from(line).unwrap_or(u32::MAX)),
                        col: Some(u32::try_from(col).unwrap_or(u32::MAX)),
                        // Advisory disclosure — an inventory listing, never gates.
                        severity: dorc_core::Severity::Note,
                        source: self.name(),
                        code: "authored-decline-class".to_owned(),
                        message: decline_message(arm.class),
                        remap: RemapFidelity::Exact,
                        provenance: None,
                    });
                }
            }
        }
        SourceStatus::Ran
    }
}

/// The inventory finding message for a per-arm decline: the classed form when the `<verb> <class>`
/// header was recognized, else the generic degrade-note (`27W:rul-report-noise-tolerant`).
fn decline_message(class: Option<DeclineClass>) -> String {
    match class {
        Some(c) => format!(
            "this verdict arm authors a deliberate decline classed `{}` (a `decline {}` report \
             emission) — the site will run; the class routes the enhancement nags (advisory only).",
            c.token(),
            c.token(),
        ),
        None => "this verdict arm authors a deliberate decline whose class is not statically \
                 readable (a dynamic format or an unrecognized class token) — the site will run; \
                 the class resolves at runtime (advisory only)."
            .to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use crate::source::{LintInput, LintOptions, registry};
    use crate::{NoToolsRunner, lint};

    fn oracle(path: &str, src: &str) -> LintInput {
        LintInput {
            path: path.to_owned(),
            src: src.to_owned(),
        }
    }

    /// Both oracle-solo sources are registered and selectable by name (`27R` §8
    /// delta-named-sources-selectable), and sit on the `oracle-solo` rung.
    #[test]
    fn oracle_solo_sources_are_registered_on_their_rung() {
        let names: Vec<&str> = registry().iter().map(|s| s.name()).collect();
        assert!(names.contains(&"oracle-validate"), "{names:?}");
        assert!(names.contains(&"oracle-declined-inventory"), "{names:?}");
        for s in registry() {
            if s.name() == "oracle-validate" || s.name() == "oracle-declined-inventory" {
                assert_eq!(s.rung().label(), "oracle-solo");
            }
        }
    }

    /// The tier-1 inventory lists each classed decline arm as an advisory Note (`AID-NEEDS:
    /// aid-authored-decline-classes`) — the `sysctl` write-only-trigger idiom (`27W:rul-strawman-
    /// tool-set`). Anti-masking: the source reads the oracle's REAL emission, not an injected class.
    #[test]
    fn declined_inventory_lists_classed_arms() {
        let sysctl = "\
sysctl__is_converged() {
   key=$1
   case $key in
   vm.drop_caches)
      printf 'decline unsound %s is a write-only trigger key\\n' \"$key\" >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   *) sysctl -n -- \"$key\" >/dev/null 2>&1 ;;
   esac
}";
        let report = lint(
            &[],
            &[oracle("sysctl.oracle.sh", sysctl)],
            LintOptions::default(),
            &NoToolsRunner,
            Some(&["oracle-declined-inventory".to_owned()]),
        );
        assert_eq!(report.findings.len(), 1, "one classed decline arm");
        let f = &report.findings[0];
        assert_eq!(f.code, "authored-decline-class");
        assert_eq!(f.severity, dorc_core::Severity::Note, "never gates");
        assert!(f.message.contains("`unsound`"), "the class: {}", f.message);
        assert_eq!(f.line, Some(5), "the emitting arm is on line 5");
    }

    /// `oracle-validate` lifts the per-file check-dialect give-up: a check body using `[[ … ]]` (out
    /// of the check dialect) surfaces `predict-out-of-dialect` as an oracle-solo finding, proving the
    /// factored `dorc_oracle::validate` reaches the lint lane (the `27S` seam's whole point).
    #[test]
    fn oracle_validate_surfaces_a_check_dialect_giveup() {
        // A backtick command-substitution is out of the check dialect (`predict/lexer.rs`).
        let out_of_dialect = "# dorc-lang/v0.2\nfoo__predict() {\n   foo --check `date`\n}\n";
        let report = lint(
            &[],
            &[oracle("foo.oracle.sh", out_of_dialect)],
            LintOptions::default(),
            &NoToolsRunner,
            Some(&["oracle-validate".to_owned()]),
        );
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.code == "predict-out-of-dialect"),
            "the check-dialect lift reaches the oracle-solo lane: {:?}",
            report.findings
        );
    }
}
