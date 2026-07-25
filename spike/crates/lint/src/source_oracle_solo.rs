//! `rung-oracle-solo` sources (`27R` §8b; UNLOCKED by `27S:seam-oracle-validate-factoring`): oracle
//! files linted with NO book present. Two sources ride the same factored surface —
//!
//! * `oracle-validate` — the book-free oracle-side validation (`dorc_oracle::validate`): the
//!   effect-map lift, per-file check-dialect lift, dual-peel + fold-entry coherence, munge-reservation
//!   lint, and the marker gate, lowered from structured [`dorc_aid::Diag`]s to findings. The SAME
//!   diagnostics the cli routes to stderr, surfaced here for the author's hot loop.
//! * `oracle-declined-inventory` — the tier-1 authored-decline inventory (`27W` §3
//!   `rul-static-first-three-tier`; `AID-NEEDS:aid-authored-decline-classes`): each verdict body's
//!   per-arm `decline <class>` emissions, listed advisory (never gates, never probes).
//!
//! Advisory-only (`dir-no-license-plane-contact`): neither mints/reads a license; the decline classes
//! route AID only.

use dorc_aid::narrative::DeclineClass;
use dorc_core::Interner;
use dorc_oracle::verdict::VerdictSet;

use crate::finding::{Finding, FrameChoice, NativeDiag, RemapFidelity, SourceStatus};
use crate::source::{LintContext, LintSource, Rung};

/// The book-free oracle-validation source: lowers `dorc_oracle::validate`'s stage-diags to findings.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct OracleValidate;

impl LintSource for OracleValidate {
    fn name(&self) -> &'static str {
        "oracle-validate"
    }

    fn describe_arrangement(&self) -> &'static str {
        "lint-source-oracle-validate"
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

/// Lower one oracle-validation [`dorc_aid::Diag`] into a finding (the same bridge shape as
/// `source-analysis-diagnostics`): the span resolves to the source `(line, col)`; a spanless diag
/// yields a whole-file finding. Native ⇒ always `RemapFidelity::Exact`.
fn diag_to_finding(path: &str, src: &str, diag: &dorc_aid::Diag, source: &'static str) -> Finding {
    let (line, col) = match diag.primary.span() {
        Some(span) => {
            let (l, c) = dorc_aid::diag::line_col(src, span.lo.0 as usize);
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
        message: dorc_aid::diag::render_body(diag, &Interner::default()),
        remap: RemapFidelity::Exact,
        provenance: Some(NativeDiag {
            diag: diag.clone(),
            source: src.to_owned(),
        }),
        frame: FrameChoice::Framed,
    }
}

/// The tier-1 authored-decline inventory source (`27W` §3; `AID-NEEDS:aid-authored-decline-classes`).
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct OracleDeclinedInventory;

impl LintSource for OracleDeclinedInventory {
    fn name(&self) -> &'static str {
        "oracle-declined-inventory"
    }

    fn describe_arrangement(&self) -> &'static str {
        "lint-source-oracle-declined-inventory"
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
                    let (line, col) = dorc_aid::diag::line_col(&oracle.src, arm.arm.lo.0 as usize);
                    let diag = dorc_aid::Diag::new(decline_code(arm.class), arm.arm);
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

/// The registry code for a per-arm decline: SIBLING codes, not one `{class}`-hole code — a
/// statically-read class and an only-at-runtime one are different world-states with different
/// remediations (`AID-NEEDS:law-codes-vary-by-world-not-grammar`; `27W:rul-report-noise-tolerant`).
fn decline_code(class: Option<DeclineClass>) -> dorc_aid::diag::DiagCode {
    match class {
        Some(c) => {
            dorc_aid::diag::DiagCode::AuthoredDeclineClass(dorc_aid::diag::AuthoredDeclineClass {
                class: c.token().to_owned(),
            })
        }
        None => dorc_aid::diag::DiagCode::AuthoredDeclineClassUnreadable(
            dorc_aid::diag::AuthoredDeclineClassUnreadable,
        ),
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
        assert_eq!(f.severity, dorc_aid::Severity::Note, "never gates");
        // Structural, not prose (`288:prop-structural-needles-only`): the class is a TYPED payload
        // param now, so it survives every re-wording of the catalog register.
        let code = &f.provenance.as_ref().expect("native provenance").diag.code;
        assert!(
            matches!(
                code,
                dorc_aid::diag::DiagCode::AuthoredDeclineClass(p) if p.class == "unsound"
            ),
            "the read class rides the payload: {code:?}"
        );
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

    #[test]
    fn oracle_validate_only_diagnoses_marks_at_parsed_carriers() {
        let ordinary_shell = r#"# dorc-lang/v0.2
while_probe__predict() {
   while :; do wombat; done
}
null_probe__predict() {
   :
}
case_probe__predict() {
   case "$1" in
   http:*) printf '%s\n' "${x:-default}" "${x:=value}" ":" escaped\:colon https://host/path /tmp:state ;;
   esac
}
bind_probe__predict() {
   item : sm.dorc.Item = "$1"
   hork --check "$item"
}
systemctl__is_converged() {
   case "${1-}" in
   enable) systemctl is-enabled --quiet -- "${2-}" : sm.dorc.Service:"$2"@enabled ;;
   *) return 2 ;;
   esac
}
"#;
        let report = lint(
            &[],
            &[oracle("ordinary-shell.oracle.sh", ordinary_shell)],
            LintOptions::default(),
            &NoToolsRunner,
            Some(&["oracle-validate".to_owned()]),
        );
        let mark_codes: Vec<&str> = report
            .findings
            .iter()
            .map(|finding| finding.code.as_str())
            .filter(|code| code.starts_with("mark-"))
            .collect();
        assert!(
            mark_codes.is_empty(),
            "ordinary shell syntax is never a mark candidate: {mark_codes:?}"
        );
    }

    #[test]
    fn oracle_validate_preserves_the_four_production_mark_diagnostics() {
        let malformed = r"# dorc-lang/v0.2
unknown__predict() { hork --check : frobnicate sm.dorc.X; }
arity__predict() { hork --check : sm.dorc.X@first; : sm.dorc.X@second; }
standalone__predict() { :? sm.dorc.X@seen; }
hash__predict() { #: frobnicate; }
";
        let report = lint(
            &[],
            &[oracle("malformed-marks.oracle.sh", malformed)],
            LintOptions::default(),
            &NoToolsRunner,
            Some(&["oracle-validate".to_owned()]),
        );
        let mut mark_codes: Vec<&str> = report
            .findings
            .iter()
            .map(|finding| finding.code.as_str())
            .filter(|code| code.starts_with("mark-"))
            .collect();
        mark_codes.sort_unstable();
        assert_eq!(
            mark_codes,
            [
                "mark-hashcolon-malformed",
                "mark-rc-arity-exceeded",
                "mark-standalone-rc-consumer",
                "mark-unknown-verb",
            ]
        );
    }
}
