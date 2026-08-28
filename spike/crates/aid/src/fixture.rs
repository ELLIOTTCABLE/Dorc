//! Typed payload fixtures for catalog structural tests.
//!
//! This table exercises payload construction and render equality. Defining-case replay never reads
//! it; cases must emit through production semantics or the three authorized defects.
//!
//! **Fixture tier, never production** (`rul-fixture-identity-never-production`). These are canned
//! example worlds: a diagnostic a real run emits is built at its own emit site, from the world it
//! observed. Nothing under a crate's `src/` outside this module may call into here — that is a
//! gate, `fixture_payloads_are_unreachable_from_production`, not a comment.
//!
//! Entries exist only where structural tests need a typed sample.

use dorc_core::{Capability, EscalationDial, LeafId, TopCause};

use crate::ForeignBytes;

use crate::diag::{
    ArtifactFormFallback, ArtifactFormRefused, CmdsubOperandTop, CommandName,
    ComputedSourceOperand, DiagCode, EmittedLineUnsafeForPaste, EscalationPolicy,
    HelperDeclarationContested, InBookVocabularyRole, LintToolAbsent,
    LoadCarriageWithheldUnderUnknownCwd, MarkHashcolonMalformed, MarkRcArityExceeded,
    MarkStandaloneRcConsumer, MarkUnknownVerb, MarkerVersionUnrecognized, MissingDialectMarker,
    MungeNameInvalid, OperandPosition, OracleMatchedZeroSites, PasteHygieneHazardReason,
    RecordsAlienLine, RecordsFactTruncated, RecordsGluedLine, RecordsHeaderMismatch,
    RecordsHeaderMissing, RecordsHeaderlessRefused, RecordsIntegrityRefused, RecordsLateLine,
    RecordsSentinelNonce, RecordsTornLine, RoleDefinedBelowItsSites, RoleFamilyContested,
    ScriptRelativeLoadDiesSlashless, SiteId, SiteUnresolvable, SlashlessSourceSearchesPath,
    SyntaxUnsupported, SyntaxUnsupportedReason, ToleratesUnknownDimension,
    VouchedCompositionNotPresent, VouchedCompositionReason, WhylogAbsent, WhylogBookDesync,
    WhylogCorrupt, WhylogCorruptReason, WhylogVersionRefused, WrapperPeelIncoherent,
};

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
            "artifact-form-refused",
            DiagCode::ArtifactFormRefused(ArtifactFormRefused {
                form: "flattened",
                cause: "inlining-unproven",
                loads: 1,
            }),
        ),
        (
            "artifact-form-fallback",
            DiagCode::ArtifactFormFallback(ArtifactFormFallback {
                form: "preserved-book-tree",
                cause: "inlining-unproven",
                loads: 1,
            }),
        ),
        (
            "emitted-line-unsafe-for-paste",
            DiagCode::EmittedLineUnsafeForPaste(EmittedLineUnsafeForPaste {
                line: 1,
                reason: PasteHygieneHazardReason::LeadingTilde,
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
        // The records lane's other eight. Their honest world is a whole probe round-trip whose
        // product is a REFUSAL — no artifact, and the diagnostic on stderr — which no in-process
        // route can render: the loom consumer's own seat turns a refused stream into an error
        // rather than a world, deliberately. The real path is pinned by the two whole-product e2e
        // cases instead (`crates/cli/tests/records28-*`).
        (
            "records-headerless-refused",
            DiagCode::RecordsHeaderlessRefused(RecordsHeaderlessRefused),
        ),
        (
            "records-glued-line",
            DiagCode::RecordsGluedLine(RecordsGluedLine),
        ),
        (
            "records-header-missing",
            DiagCode::RecordsHeaderMissing(RecordsHeaderMissing),
        ),
        (
            "records-sentinel-nonce",
            DiagCode::RecordsSentinelNonce(RecordsSentinelNonce),
        ),
        (
            "records-integrity-refused",
            DiagCode::RecordsIntegrityRefused(RecordsIntegrityRefused {
                which: RecordsHeaderMismatch::Host,
            }),
        ),
        (
            "records-torn-line",
            DiagCode::RecordsTornLine(RecordsTornLine { count: 2 }),
        ),
        (
            "records-alien-line",
            DiagCode::RecordsAlienLine(RecordsAlienLine { count: 1 }),
        ),
        (
            "records-late-line",
            DiagCode::RecordsLateLine(RecordsLateLine { count: 1 }),
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
            "wrapper-peel-incoherent",
            DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
                wrapper: "sudo".to_owned(),
                predict_depth: "1".to_owned(),
                lend_map_depth: "0".to_owned(),
            }),
        ),
        // World-as-payload: reads the run's WHOLE final `Vouches` set, unreachable to a
        // single-file consumer; real firing route: `cli/tests/oracle-matched-zero-sites-round-trip.loom`.
        (
            "oracle-matched-zero-sites",
            DiagCode::OracleMatchedZeroSites(OracleMatchedZeroSites {
                oracle: "hork.oracle.sh".to_owned(),
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
        // Same necessity: one helper spelled two ways is a two-file world.
        (
            "helper-declaration-contested",
            DiagCode::HelperDeclarationContested(HelperDeclarationContested {
                name: "_yum_installed".to_owned(),
                prior: "vendor/yum.oracle.sh:4".to_owned(),
            }),
        ),
        // Same necessity, one seat BACK: the load-head family is minted by the binary's own
        // load-edge driver, past everything the in-process book pipeline composes.
        (
            "script-relative-load-dies-slashless",
            DiagCode::ScriptRelativeLoadDiesSlashless(ScriptRelativeLoadDiesSlashless),
        ),
        (
            "slashless-source-searches-path",
            DiagCode::SlashlessSourceSearchesPath(SlashlessSourceSearchesPath),
        ),
        (
            "load-carriage-withheld-under-unknown-cwd",
            DiagCode::LoadCarriageWithheldUnderUnknownCwd(LoadCarriageWithheldUnderUnknownCwd {
                line: 1,
            }),
        ),
        (
            "computed-source-operand",
            DiagCode::ComputedSourceOperand(ComputedSourceOperand),
        ),
        // Same necessity, one seat further in: the suspension is decided inside the vouch lift and
        // reported at the binary's own load-edge seat, which no in-process consumer reaches.
        (
            "vouched-composition-not-present",
            DiagCode::VouchedCompositionNotPresent(VouchedCompositionNotPresent {
                name: "_yum_installed".to_owned(),
                reason: VouchedCompositionReason::BookRedefinesHelper,
                live: String::new(),
            }),
        ),
        // The external-linter trio: a replay never runs a foreign tool (`tools_enabled: false`).
        (
            "lint-tool-absent",
            DiagCode::LintToolAbsent(LintToolAbsent {
                tool: "shellcheck".to_owned(),
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
