//! The loom-only authority for internally-caused defect diagnostics.

use dorc_aid::diag::{
    Diag, DiagCode, SiteId, SolvePass, SolverConsistencyFailure, SolverConsistencyPlanDemoted,
    SurvivalRederivationDisagreement,
};
use dorc_core::LeafId;

/// Human-authorized closed exception. Add a variant only with explicit human authorization, and
/// only for a correctness-critical internal failure impractical or impossible to induce through an
/// external production round trip. Any diagnostic with a causative external scenario MUST use the
/// normal honest loom route; this is never a convenience escape from constructing that scenario.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum DefectScenario {
    SolverConsistencyFailure,
    SolverConsistencyPlanDemoted,
    SurvivalRederivationDisagreement,
}

impl DefectScenario {
    pub(crate) const ALL: [Self; 3] = [
        Self::SolverConsistencyFailure,
        Self::SolverConsistencyPlanDemoted,
        Self::SurvivalRederivationDisagreement,
    ];

    pub(crate) fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|scenario| scenario.slug() == slug)
    }

    pub(crate) const fn slug(self) -> &'static str {
        match self {
            Self::SolverConsistencyFailure => "solver-consistency-failure",
            Self::SolverConsistencyPlanDemoted => "solver-consistency-plan-demoted",
            Self::SurvivalRederivationDisagreement => "survival-rederivation-disagreement",
        }
    }

    pub(crate) const fn stage(self) -> &'static str {
        match self {
            Self::SolverConsistencyFailure | Self::SolverConsistencyPlanDemoted => "solve",
            Self::SurvivalRederivationDisagreement => "rederive",
        }
    }

    pub(crate) fn diagnostic(self) -> Diag {
        let code = match self {
            Self::SolverConsistencyFailure => {
                DiagCode::SolverConsistencyFailure(SolverConsistencyFailure {
                    pass: SolvePass::ReachingDefs,
                    failing: "3".to_owned(),
                })
            }
            Self::SolverConsistencyPlanDemoted => {
                DiagCode::SolverConsistencyPlanDemoted(SolverConsistencyPlanDemoted {
                    demoted: "4".to_owned(),
                })
            }
            Self::SurvivalRederivationDisagreement => {
                DiagCode::SurvivalRederivationDisagreement(SurvivalRederivationDisagreement {
                    site: SiteId::leaf(LeafId(4)),
                    wall: "1".to_owned(),
                })
            }
        };
        Diag::new_spanless_site(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AUTHORIZATION_FAILURE: &str = "the harness defect list may grow only with explicit human \
        authorization; an externally causative diagnostic must use an honest production loom";

    #[test]
    fn the_human_authorized_scenario_set_is_exact() {
        assert_eq!(
            DefectScenario::ALL.map(DefectScenario::slug),
            [
                "solver-consistency-failure",
                "solver-consistency-plan-demoted",
                "survival-rederivation-disagreement",
            ],
            "{AUTHORIZATION_FAILURE}"
        );
        for scenario in DefectScenario::ALL {
            assert_eq!(scenario.diagnostic().code.slug(), scenario.slug());
        }
    }

    #[test]
    fn production_cannot_name_harness_defect_authority() {
        let production = include_str!("../../cli/src/main.rs");
        for forbidden in ["DefectScenario", "dorc_loom", "--this defect"] {
            assert!(
                !production.contains(forbidden),
                "production names harness defect authority `{forbidden}`; {AUTHORIZATION_FAILURE}"
            );
        }
    }

    #[test]
    fn only_the_three_authorized_defects_construct_diagnostic_payloads() {
        let needle = concat!("Diag", "Code::");
        let source_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        for entry in std::fs::read_dir(source_dir).expect("source directory") {
            let path = entry.expect("source entry").path();
            if path.file_name().is_some_and(|name| {
                name == "defect.rs" || name.to_string_lossy().contains(".sync-conflict-")
            }) || path.extension().is_none_or(|extension| extension != "rs")
            {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("source reads");
            assert!(
                !source.contains(needle),
                "{} directly constructs a diagnostic payload; only defect.rs may do that",
                path.display()
            );
        }
        assert_eq!(include_str!("defect.rs").matches(needle).count(), 3);
    }
}
