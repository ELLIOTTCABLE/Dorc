//! The loom-only authority for internally-caused defect diagnostics.

use dorc_aid::diag::{
    ApplyPlanNotDispatchable, Diag, DiagCode, SiteId, SolvePass, SolverConsistencyFailure,
    SolverConsistencyPlanDemoted, SurvivalRederivationDisagreement,
};
use dorc_core::LeafId;

/// Which report seat really prints a defect's code, and what the binary exits when it does.
///
/// Carried by the scenario rather than read off the case, because a bypass render has no
/// invocation whose shape a seat could be picked from: the registry is the only thing that can
/// say. A member whose seat disagrees with the product's would commit a transcript of a render
/// the product never emits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum DefectSeat {
    /// A source-staged diagnostic — `<stage>: error[...]` — after which the run carries on.
    Staged(&'static str),
    /// The invocation-error seat: the `dorc: ` prefix and the trailing usage synopsis, after
    /// which the binary exits.
    Invocation {
        /// The exit status the binary leaves with.
        status: i32,
    },
}

/// Human-authorized closed exception. Add a variant only with explicit human authorization, and
/// only for a correctness-critical internal failure impractical or impossible to induce through an
/// external production round trip. Any diagnostic with a causative external scenario MUST use the
/// normal honest loom route; this is never a convenience escape from constructing that scenario.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum DefectScenario {
    SolverConsistencyFailure,
    SolverConsistencyPlanDemoted,
    SurvivalRederivationDisagreement,
    /// Admitted by explicit human authorization, and exceptional against the criterion above:
    /// this one's causative scenario EXISTS, but no corpus route reaches it.
    ///
    /// The reason this member stands for is `image-not-recordable` — a consented plan past the
    /// per-entry apply-image bound. That bound is reachable only through `dorc apply --host`,
    /// which needs a live remote session the suite structurally does not have; the replay harness
    /// separately refuses to read a file anywhere near the bound's size.
    ///
    /// Scope, because the code is wider than this member: `apply-plan-not-dispatchable` carries
    /// three unrelated reasons and only this one is host-only. The code is owed a split into
    /// per-world codes, at which point this member narrows to the split-off reason or goes away
    /// with it. It was never about the other two.
    ApplyPlanImageNotRecordable,
}

impl DefectScenario {
    pub(crate) const ALL: [Self; 4] = [
        Self::SolverConsistencyFailure,
        Self::SolverConsistencyPlanDemoted,
        Self::SurvivalRederivationDisagreement,
        Self::ApplyPlanImageNotRecordable,
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
            Self::ApplyPlanImageNotRecordable => "apply-plan-not-dispatchable",
        }
    }

    pub(crate) const fn seat(self) -> DefectSeat {
        match self {
            Self::SolverConsistencyFailure | Self::SolverConsistencyPlanDemoted => {
                DefectSeat::Staged("solve")
            }
            Self::SurvivalRederivationDisagreement => DefectSeat::Staged("rederive"),
            // `run` hands this one back as `Err`, so the binary prints it through the invocation
            // seat and leaves with its usage status.
            Self::ApplyPlanImageNotRecordable => DefectSeat::Invocation { status: 2 },
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
            Self::ApplyPlanImageNotRecordable => {
                DiagCode::ApplyPlanNotDispatchable(ApplyPlanNotDispatchable {
                    reason: "image-not-recordable",
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
                "apply-plan-not-dispatchable",
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
    fn only_the_four_authorized_defects_construct_diagnostic_payloads() {
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
        assert_eq!(include_str!("defect.rs").matches(needle).count(), 4);
    }
}
