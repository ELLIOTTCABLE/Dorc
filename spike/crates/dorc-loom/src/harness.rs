//! Typed loom-only edge outcomes that cannot be portably materialized in txtar.

use dorc_aid::ForeignBytes;
use dorc_aid::diag::{
    AidUnloadedSiblingOracle, ArtifactPublishRefused, CarriedAcrossSubstrateAxis, CliFileNotFound,
    CliFilePermissionDenied, CliFileUnreadable, CliShimDirUnwritable, DanglingReference, Diag,
    DiagCode, DorcShExecFailed, DorcShScriptUnreadable, HostEvidenceAdmissionRefused,
    HostEvidenceRefusalKind, LintFileCountDrift, LintNoLintableFiles, LintRequiredToolsMissing,
    LintToolFailedWithoutFindings, LintToolOutputUnparsable, PlanImportRewritten,
    RenderHeredocRefused, RenderRegionRefused, SharedCellMeasurementsDisagree, SiteId,
    TransportApplyFailed, TransportCrlfRefused, TransportMarkerUnusable, TransportSessionLost,
    TransportSpawnRefused, WhylogUnwritten,
};
use dorc_core::LeafId;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum HarnessPresentation {
    Invocation,
    Shim,
    Body,
    Stage(&'static str),
}

pub(crate) struct HarnessEvent {
    pub(crate) diagnostic: Diag,
    pub(crate) presentation: HarnessPresentation,
    pub(crate) status: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum HarnessScenario {
    AidUnloadedSiblingOracle,
    CarriedAcrossSubstrateAxis,
    DanglingReference,
    CliFileNotFound,
    CliFilePermissionDenied,
    CliFileUnreadable,
    CliShimDirUnwritable,
    DorcShScriptUnreadable,
    DorcShExecFailed,
    LintNoLintableFiles,
    LintFileCountDrift,
    LintRequiredToolsMissing,
    LintToolOutputUnparsable,
    LintToolFailedWithoutFindings,
    HostEvidenceAdmissionRefused,
    PlanImportRewritten,
    RenderHeredocRefused,
    RenderRegionRefused,
    SharedCellMeasurementsDisagree,
    ArtifactPublishRefused,
    WhylogUnwritten,
    TransportCrlfRefused,
    TransportSessionLost,
    TransportSpawnRefused,
    TransportMarkerUnusable,
    TransportApplyFailed,
}

impl HarnessScenario {
    pub(crate) fn from_slug(slug: &str) -> Option<Self> {
        Some(match slug {
            "aid-unloaded-sibling-oracle" => Self::AidUnloadedSiblingOracle,
            "carried-across-substrate-axis" => Self::CarriedAcrossSubstrateAxis,
            "dangling-reference" => Self::DanglingReference,
            "cli-file-not-found" => Self::CliFileNotFound,
            "cli-file-permission-denied" => Self::CliFilePermissionDenied,
            "cli-file-unreadable" => Self::CliFileUnreadable,
            "cli-shim-dir-unwritable" => Self::CliShimDirUnwritable,
            "dorc-sh-script-unreadable" => Self::DorcShScriptUnreadable,
            "dorc-sh-exec-failed" => Self::DorcShExecFailed,
            "lint-no-lintable-files" => Self::LintNoLintableFiles,
            "lint-file-count-drift" => Self::LintFileCountDrift,
            "lint-required-tools-missing" => Self::LintRequiredToolsMissing,
            "lint-tool-output-unparsable" => Self::LintToolOutputUnparsable,
            "lint-tool-failed-without-findings" => Self::LintToolFailedWithoutFindings,
            "host-evidence-admission-refused" => Self::HostEvidenceAdmissionRefused,
            "plan-import-rewritten" => Self::PlanImportRewritten,
            "render-heredoc-refused" => Self::RenderHeredocRefused,
            "render-region-refused" => Self::RenderRegionRefused,
            "shared-cell-measurements-disagree" => Self::SharedCellMeasurementsDisagree,
            "artifact-publish-refused" => Self::ArtifactPublishRefused,
            "whylog-unwritten" => Self::WhylogUnwritten,
            "transport-crlf-refused" => Self::TransportCrlfRefused,
            "transport-session-lost" => Self::TransportSessionLost,
            "transport-spawn-refused" => Self::TransportSpawnRefused,
            "transport-marker-unusable" => Self::TransportMarkerUnusable,
            "transport-apply-failed" => Self::TransportApplyFailed,
            _ => return None,
        })
    }

    pub(crate) fn accepts(self, argv: &[String]) -> bool {
        match self {
            Self::DorcShScriptUnreadable | Self::DorcShExecFailed => {
                argv.first().map(String::as_str) == Some("dorc-sh")
            }
            Self::LintNoLintableFiles
            | Self::LintFileCountDrift
            | Self::LintRequiredToolsMissing
            | Self::LintToolOutputUnparsable
            | Self::LintToolFailedWithoutFindings => {
                argv.get(0..2) == Some(["dorc".to_owned(), "lint".to_owned()].as_slice())
            }
            Self::TransportApplyFailed => {
                argv.get(0..2) == Some(["dorc".to_owned(), "apply".to_owned()].as_slice())
            }
            _ => argv.first().map(String::as_str) == Some("dorc"),
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "one closed typed edge-state table; factoring by payload would recreate an arbitrary diagnostic fixture API"
    )]
    pub(crate) fn event(self) -> HarnessEvent {
        let (code, presentation, status) = match self {
            Self::AidUnloadedSiblingOracle => (
                DiagCode::AidUnloadedSiblingOracle(AidUnloadedSiblingOracle {
                    oracles: "`redis.oracle.sh`".to_owned(),
                }),
                HarnessPresentation::Stage("oracle"),
                0,
            ),
            Self::CarriedAcrossSubstrateAxis => (
                DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                    axes: "fs-view".to_owned(),
                    kinds: "sm_dorc_File (invariant: line at certsync.oracle.sh:12)".to_owned(),
                }),
                HarnessPresentation::Stage("carry"),
                0,
            ),
            Self::DanglingReference => (
                DiagCode::DanglingReference(DanglingReference {
                    coord: "sm.dorc.Package:nginx".to_owned(),
                }),
                HarnessPresentation::Stage("resolve"),
                0,
            ),
            Self::CliFileNotFound => (
                DiagCode::CliFileNotFound(CliFileNotFound {
                    kind: "book".to_owned(),
                    path: "webhost.sh".to_owned(),
                }),
                HarnessPresentation::Invocation,
                2,
            ),
            Self::CliFilePermissionDenied => (
                DiagCode::CliFilePermissionDenied(CliFilePermissionDenied {
                    kind: "oracle".to_owned(),
                    path: "/etc/dorc/nginx.oracle.sh".to_owned(),
                }),
                HarnessPresentation::Invocation,
                2,
            ),
            Self::CliFileUnreadable => (
                DiagCode::CliFileUnreadable(CliFileUnreadable {
                    kind: "results".to_owned(),
                    path: "probe-results.txt".to_owned(),
                    detail: ForeignBytes::from_io_edge("Is a directory (os error 21)"),
                }),
                HarnessPresentation::Invocation,
                2,
            ),
            Self::CliShimDirUnwritable => (
                DiagCode::CliShimDirUnwritable(CliShimDirUnwritable {
                    path: "/run/dorc/shims".to_owned(),
                    detail: ForeignBytes::from_io_edge("Read-only file system (os error 30)"),
                }),
                HarnessPresentation::Invocation,
                2,
            ),
            Self::DorcShScriptUnreadable => (
                DiagCode::DorcShScriptUnreadable(DorcShScriptUnreadable {
                    path: "webhost.sh".to_owned(),
                    detail: ForeignBytes::from_io_edge("No such file or directory (os error 2)"),
                }),
                HarnessPresentation::Shim,
                2,
            ),
            Self::DorcShExecFailed => (
                DiagCode::DorcShExecFailed(DorcShExecFailed {
                    detail: ForeignBytes::from_io_edge("No such file or directory (os error 2)"),
                }),
                HarnessPresentation::Shim,
                2,
            ),
            Self::LintNoLintableFiles => (
                DiagCode::LintNoLintableFiles(LintNoLintableFiles),
                HarnessPresentation::Body,
                3,
            ),
            Self::LintFileCountDrift => (
                DiagCode::LintFileCountDrift(LintFileCountDrift {
                    expected: 12,
                    found: 9,
                }),
                HarnessPresentation::Body,
                3,
            ),
            Self::LintRequiredToolsMissing => (
                DiagCode::LintRequiredToolsMissing(LintRequiredToolsMissing {
                    tools: "checkbashisms, shellcheck".to_owned(),
                }),
                HarnessPresentation::Body,
                3,
            ),
            Self::LintToolOutputUnparsable => (
                DiagCode::LintToolOutputUnparsable(LintToolOutputUnparsable {
                    tool: "checkbashisms".to_owned(),
                    output: ForeignBytes::from_io_edge(
                        "possible bashism in - line 4 (should be '.'):",
                    ),
                }),
                HarnessPresentation::Body,
                1,
            ),
            Self::LintToolFailedWithoutFindings => (
                DiagCode::LintToolFailedWithoutFindings(LintToolFailedWithoutFindings {
                    tool: "shellcheck".to_owned(),
                    rc: 2,
                }),
                HarnessPresentation::Body,
                1,
            ),
            Self::HostEvidenceAdmissionRefused => (
                DiagCode::HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused {
                    kind: HostEvidenceRefusalKind::Framing,
                }),
                HarnessPresentation::Stage("records"),
                12,
            ),
            Self::PlanImportRewritten => (
                DiagCode::PlanImportRewritten(PlanImportRewritten {
                    verb: "repointed",
                    names: "./wombat.dorc-bundle.sh".to_owned(),
                    reason: "shape-unmeasured",
                }),
                HarnessPresentation::Stage("emission"),
                0,
            ),
            Self::RenderHeredocRefused => (
                DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                    site: SiteId::leaf(LeafId(7)),
                    verb: "elide",
                    command: "cat <<EOF".to_owned(),
                }),
                HarnessPresentation::Stage("render"),
                0,
            ),
            Self::RenderRegionRefused => (
                DiagCode::RenderRegionRefused(RenderRegionRefused {
                    verb: "elide",
                    command: "cat <<EOF".to_owned(),
                    routes: 2,
                }),
                HarnessPresentation::Stage("render"),
                0,
            ),
            Self::SharedCellMeasurementsDisagree => (
                DiagCode::SharedCellMeasurementsDisagree(SharedCellMeasurementsDisagree {
                    cell: "dorc-auto:cp@converged".to_owned(),
                    sites: 2,
                }),
                HarnessPresentation::Stage("records"),
                0,
            ),
            Self::ArtifactPublishRefused => (
                DiagCode::ArtifactPublishRefused(ArtifactPublishRefused {
                    reason: "directory",
                }),
                HarnessPresentation::Stage("emission"),
                16,
            ),
            Self::WhylogUnwritten => (
                DiagCode::WhylogUnwritten(WhylogUnwritten {
                    dir: "/var/lib/dorc/whylog".to_owned(),
                    reason: "directory".to_owned(),
                }),
                HarnessPresentation::Stage("whylog"),
                0,
            ),
            Self::TransportCrlfRefused => (
                DiagCode::TransportCrlfRefused(TransportCrlfRefused {
                    which: "webhost.dorc-plan.sh".to_owned(),
                    line: "1".to_owned(),
                }),
                HarnessPresentation::Stage("transport"),
                13,
            ),
            Self::TransportSessionLost => (
                DiagCode::TransportSessionLost(TransportSessionLost {
                    host: "web1.example.net".to_owned(),
                    attempts: "3".to_owned(),
                    diagnosis: "the session ended without a status".to_owned(),
                }),
                HarnessPresentation::Stage("transport"),
                14,
            ),
            Self::TransportSpawnRefused => (
                DiagCode::TransportSpawnRefused(TransportSpawnRefused {
                    host: "web1.example.net".to_owned(),
                    detail: ForeignBytes::from_io_edge("program not found"),
                }),
                HarnessPresentation::Stage("transport"),
                13,
            ),
            Self::TransportMarkerUnusable => (
                DiagCode::TransportMarkerUnusable(TransportMarkerUnusable {
                    host: "web1.example.net".to_owned(),
                }),
                HarnessPresentation::Stage("transport"),
                13,
            ),
            Self::TransportApplyFailed => (
                DiagCode::TransportApplyFailed(TransportApplyFailed {
                    host: "web1.example.net".to_owned(),
                    status: "2".to_owned(),
                }),
                HarnessPresentation::Stage("transport"),
                15,
            ),
        };
        HarnessEvent {
            diagnostic: Diag::new_spanless_site(code),
            presentation,
            status,
        }
    }
}
