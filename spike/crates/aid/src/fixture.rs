//! Canonical stand-in payloads for the defining-case corpus (`283:dec-world-two-forms`,
//! world-as-payload).
//!
//! **This table lives here because it constructs the payload types next door.** A Rust author
//! adding a field to a payload in [`crate::diag`] gets an `E0063 missing field` here — in `aid`,
//! one file over — and never inside `dorc-loom`, whose internals are not an edit surface for
//! either persona (`28L:rul-rust-and-loom-are-the-only-edit-surfaces`). Fill the new field with a
//! plausible constant and the value flows to every loom consumer with no further step.
//!
//! **Fixture tier, never production** (`rul-fixture-identity-never-production`). These are canned
//! example worlds: a diagnostic a real run emits is built at its own emit site, from the world it
//! observed. Nothing under a crate's `src/` outside this module may call into here — that is a
//! gate, `fixture_payloads_are_unreachable_from_production`, not a comment.
//!
//! A slug earns an entry only while it has NO honest firing route. Every consumer tries the real
//! pipeline first (`289:rul-worldless-route-honest-trigger`), so an entry whose code gains a real
//! trigger becomes unreachable rather than wrong; the corpus's render fixpoint is what notices.

use dorc_core::{Capability, EscalationDial, LeafId, TopCause};

use crate::ForeignBytes;

use crate::diag::{
    AidUnloadedSiblingOracle, CarriedAcrossSubstrateAxis, CliFileNotFound, CliFilePermissionDenied,
    CliFileUnreadable, CliShimDirUnwritable, CmdsubOperandTop, CommandName, DanglingReference,
    DiagCode, DorcShExecFailed, DorcShScriptUnreadable, EscalationPolicy,
    HelperDeclarationContested, HostEvidenceAdmissionRefused, HostEvidenceRefusalKind,
    InBookVocabularyRole, LintFileCountDrift, LintNoLintableFiles, LintRequiredToolsMissing,
    LintToolAbsent, LintToolFailedWithoutFindings, LintToolOutputUnparsable,
    MarkHashcolonMalformed, MarkRcArityExceeded, MarkStandaloneRcConsumer, MarkUnknownVerb,
    MarkerVersionUnrecognized, MissingDialectMarker, MungeNameInvalid, OperandPosition,
    RecordsFactTruncated, RenderHeredocRefused, RoleDefinedBelowItsSites, RoleFamilyContested,
    SharedCellMeasurementsDisagree, SiteId, SiteUnresolvable, SyntaxUnsupported,
    SyntaxUnsupportedReason, ToleratesUnknownDimension, TransportApplyFailed, TransportCrlfRefused,
    TransportMarkerUnusable, TransportSessionLost, TransportSpawnRefused, WhylogAbsent,
    WhylogBookDesync, WhylogCorrupt, WhylogCorruptReason, WhylogUnwritten, WhylogVersionRefused,
    WrapperPeelIncoherent,
};

/// The canonical stand-in payload for `slug`, if one is registered.
#[must_use]
pub fn canonical_payload(slug: &str) -> Option<DiagCode> {
    canonical_payloads()
        .into_iter()
        .find(|(registered, _)| *registered == slug)
        .map(|(_, code)| code)
}

/// Every registered stand-in, slug-keyed, in table order.
///
/// Values are fixed so the renders they feed are deterministic (`inv-determinism`).
#[must_use]
#[expect(
    clippy::too_many_lines,
    reason = "one entry PER CODE, like the catalog registry it stands in for — merging entries would hide which codes still lack an honest trigger"
)]
pub fn canonical_payloads() -> Vec<(&'static str, DiagCode)> {
    vec![
        // Analyzer give-ups whose honest world is a whole probe round-trip.
        (
            "cmdsub-operand-top",
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: SiteId::leaf(LeafId(3)),
                position: OperandPosition::Operand(1),
                cause: None,
                top_cause: TopCause::UnmodeledExpansion,
                command: CommandName::Literal("apt-get".to_owned()),
            }),
        ),
        (
            "site-unresolvable",
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: SiteId::leaf(LeafId(4)),
                count: "2".to_owned(),
                site_word: "sites",
                names: ForeignBytes::from_io_edge("`make install`, `ldconfig`"),
                excerpt: ForeignBytes::from_io_edge("make install"),
            }),
        ),
        (
            "render-heredoc-refused",
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: SiteId::leaf(LeafId(7)),
                verb: "elide",
                command: "cat <<EOF".to_owned(),
            }),
        ),
        (
            "syntax-unsupported",
            DiagCode::SyntaxUnsupported(SyntaxUnsupported {
                reason: SyntaxUnsupportedReason::BackgroundAmp,
            }),
        ),
        (
            "records-fact-truncated",
            DiagCode::RecordsFactTruncated(RecordsFactTruncated {
                received: 3,
                declared: 5,
                unseen: 2,
            }),
        ),
        (
            "host-evidence-admission-refused",
            DiagCode::HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused {
                kind: HostEvidenceRefusalKind::Framing,
            }),
        ),
        (
            "escalation-policy",
            DiagCode::EscalationPolicy(EscalationPolicy {
                dial: EscalationDial::VouchedOnly,
                capability: Capability::Root,
                entry_forms: "sudo -n".to_owned(),
            }),
        ),
        (
            "carried-across-substrate-axis",
            DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                axes: "fs-view".to_owned(),
                kinds: "sm_dorc_File (invariant: line at certsync.oracle.sh:12)".to_owned(),
            }),
        ),
        (
            "wrapper-peel-incoherent",
            DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
                wrapper: "sudo".to_owned(),
                predict_depth: "1".to_owned(),
                lend_map_depth: "0".to_owned(),
            }),
        ),
        (
            "aid-unloaded-sibling-oracle",
            DiagCode::AidUnloadedSiblingOracle(AidUnloadedSiblingOracle {
                oracles: "`redis.oracle.sh`".to_owned(),
            }),
        ),
        (
            "dangling-reference",
            DiagCode::DanglingReference(DanglingReference {
                coord: "sm.dorc.Package:nginx".to_owned(),
            }),
        ),
        // World-as-payload by necessity: the trigger is a whole LOADED UNIT (two files defining one
        // role family), and a case materializes its world one source at a time.
        (
            "role-family-contested",
            DiagCode::RoleFamilyContested(RoleFamilyContested {
                family: "yum".to_owned(),
                name: "yum__is_converged".to_owned(),
                prior: "vendor/yum.oracle.sh:4".to_owned(),
            }),
        ),
        // Same necessity: the trigger is a whole BOOK read positionally.
        (
            "role-defined-below-its-sites",
            DiagCode::RoleDefinedBelowItsSites(RoleDefinedBelowItsSites {
                name: "yum__is_converged".to_owned(),
                sites: 2,
            }),
        ),
        (
            "in-book-vocabulary-role",
            DiagCode::InBookVocabularyRole(InBookVocabularyRole {
                name: "sm_dorc_Package__resolve".to_owned(),
                role: "__resolve".to_owned(),
            }),
        ),
        // Same necessity again: two loaded sources spelling one helper differently is a
        // two-file world, not a one-source one.
        (
            "helper-declaration-contested",
            DiagCode::HelperDeclarationContested(HelperDeclarationContested {
                name: "_yum_installed".to_owned(),
                prior: "vendor/yum.oracle.sh:4".to_owned(),
            }),
        ),
        // Read back from a RECORDS stream: a replay drives no host and admits no records.
        (
            "shared-cell-measurements-disagree",
            DiagCode::SharedCellMeasurementsDisagree(SharedCellMeasurementsDisagree {
                cell: "dorc-auto:cp@converged".to_owned(),
                sites: 2,
            }),
        ),
        // The external-linter trio: a replay never runs a foreign tool (`tools_enabled: false`).
        (
            "lint-tool-absent",
            DiagCode::LintToolAbsent(LintToolAbsent {
                tool: "shellcheck".to_owned(),
            }),
        ),
        (
            "lint-tool-output-unparsable",
            DiagCode::LintToolOutputUnparsable(LintToolOutputUnparsable {
                tool: "checkbashisms".to_owned(),
                output: ForeignBytes::from_io_edge("possible bashism in - line 4 (should be '.'):"),
            }),
        ),
        (
            "lint-tool-failed-without-findings",
            DiagCode::LintToolFailedWithoutFindings(LintToolFailedWithoutFindings {
                tool: "shellcheck".to_owned(),
                rc: 2,
            }),
        ),
        // Worlds that are an I/O FAILURE, not an argv: an honest trigger would need a real
        // unreadable file, a full disk, or an absent `sh`.
        (
            "cli-file-not-found",
            DiagCode::CliFileNotFound(CliFileNotFound {
                kind: "book".to_owned(),
                path: "webhost.sh".to_owned(),
            }),
        ),
        (
            "cli-file-permission-denied",
            DiagCode::CliFilePermissionDenied(CliFilePermissionDenied {
                kind: "oracle".to_owned(),
                path: "/etc/dorc/nginx.oracle.sh".to_owned(),
            }),
        ),
        (
            "cli-file-unreadable",
            DiagCode::CliFileUnreadable(CliFileUnreadable {
                kind: "results".to_owned(),
                path: "probe-results.txt".to_owned(),
                detail: ForeignBytes::from_io_edge("Is a directory (os error 21)"),
            }),
        ),
        (
            "cli-shim-dir-unwritable",
            DiagCode::CliShimDirUnwritable(CliShimDirUnwritable {
                path: "/run/dorc/shims".to_owned(),
                detail: ForeignBytes::from_io_edge("Read-only file system (os error 30)"),
            }),
        ),
        (
            "whylog-unwritten",
            DiagCode::WhylogUnwritten(WhylogUnwritten {
                dir: "/var/lib/dorc/whylog".to_owned(),
                reason: "directory".to_owned(),
            }),
        ),
        (
            "lint-no-lintable-files",
            DiagCode::LintNoLintableFiles(LintNoLintableFiles),
        ),
        (
            "lint-file-count-drift",
            DiagCode::LintFileCountDrift(LintFileCountDrift {
                expected: 12,
                found: 9,
            }),
        ),
        (
            "lint-required-tools-missing",
            DiagCode::LintRequiredToolsMissing(LintRequiredToolsMissing {
                tools: "checkbashisms, shellcheck".to_owned(),
            }),
        ),
        (
            "dorc-sh-script-unreadable",
            DiagCode::DorcShScriptUnreadable(DorcShScriptUnreadable {
                path: "webhost.sh".to_owned(),
                detail: ForeignBytes::from_io_edge("No such file or directory (os error 2)"),
            }),
        ),
        (
            "dorc-sh-exec-failed",
            DiagCode::DorcShExecFailed(DorcShExecFailed {
                detail: ForeignBytes::from_io_edge("No such file or directory (os error 2)"),
            }),
        ),
        // A SESSION, not bytes we parsed: the spike opens no sockets.
        (
            "transport-crlf-refused",
            DiagCode::TransportCrlfRefused(TransportCrlfRefused {
                which: "webhost.dorc-plan.sh".to_owned(),
                line: "1".to_owned(),
            }),
        ),
        (
            "transport-session-lost",
            DiagCode::TransportSessionLost(TransportSessionLost {
                host: "web1.example.net".to_owned(),
                attempts: "3".to_owned(),
                diagnosis: "the session ended without a status".to_owned(),
            }),
        ),
        (
            "transport-spawn-refused",
            DiagCode::TransportSpawnRefused(TransportSpawnRefused {
                host: "web1.example.net".to_owned(),
                detail: ForeignBytes::from_io_edge("program not found"),
            }),
        ),
        (
            "transport-marker-unusable",
            DiagCode::TransportMarkerUnusable(TransportMarkerUnusable {
                host: "web1.example.net".to_owned(),
            }),
        ),
        (
            "transport-apply-failed",
            DiagCode::TransportApplyFailed(TransportApplyFailed {
                host: "web1.example.net".to_owned(),
                status: "2".to_owned(),
            }),
        ),
        // Codes with an honest in-corpus trigger. They keep an entry because the render-equality
        // twins exercise every payload species; a consumer reaches the real firing first.
        (
            "missing-dialect-marker",
            DiagCode::MissingDialectMarker(MissingDialectMarker),
        ),
        (
            "munge-name-invalid",
            DiagCode::MungeNameInvalid(MungeNameInvalid {
                source: "9pkg".to_owned(),
                funcname: "9pkg".to_owned(),
                problem: "starts with a digit".to_owned(),
            }),
        ),
        (
            "tolerates-unknown-dimension",
            DiagCode::ToleratesUnknownDimension(ToleratesUnknownDimension {
                token: "netns2".to_owned(),
                expected: "user, netns, fs-view".to_owned(),
            }),
        ),
        (
            "marker-version-unrecognized",
            DiagCode::MarkerVersionUnrecognized(MarkerVersionUnrecognized {
                found: "# dorc-lang/v0.1".to_owned(),
            }),
        ),
        (
            "mark-unknown-verb",
            DiagCode::MarkUnknownVerb(MarkUnknownVerb {
                token: "frobnicate".to_owned(),
                expected: "asserts, refutes, reads, bind, safe-across, disturbs, lends, \
                           stored-in, undivided-by-transit-across"
                    .to_owned(),
            }),
        ),
        (
            "mark-rc-arity-exceeded",
            DiagCode::MarkRcArityExceeded(MarkRcArityExceeded),
        ),
        (
            "mark-standalone-rc-consumer",
            DiagCode::MarkStandaloneRcConsumer(MarkStandaloneRcConsumer),
        ),
        (
            "mark-hashcolon-malformed",
            DiagCode::MarkHashcolonMalformed(MarkHashcolonMalformed),
        ),
        (
            "whylog-version-refused",
            DiagCode::WhylogVersionRefused(WhylogVersionRefused {
                found: "dorc-whylog/2".to_owned(),
            }),
        ),
        (
            "whylog-book-desync",
            DiagCode::WhylogBookDesync(WhylogBookDesync {
                which: "book".to_owned(),
            }),
        ),
        (
            "whylog-absent",
            DiagCode::WhylogAbsent(WhylogAbsent {
                dir: "./.dorc/whylog".to_owned(),
            }),
        ),
        (
            "whylog-corrupt",
            DiagCode::WhylogCorrupt(WhylogCorrupt {
                reason: WhylogCorruptReason::EndSentinelMissing,
            }),
        ),
    ]
}
