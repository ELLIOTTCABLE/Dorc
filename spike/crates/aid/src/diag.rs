//! The structured-diagnostic API spine — the round-22 arch-3 design (`Research/notes/22B`,
//! ratified; `plans/22A` concl-7/concl-8; `held-4` sanctioned design-for-keeps exception).
//!
//! This is the GOOD shape made the cheapest to write (`22B` §0): a [`Diag`] is a typed
//! [`DiagCode`] payload + a MANDATORY primary [`SpanLabel`] + ordered [`SubDiag`] children +
//! an optional [`Suggestion`]; severity comes ONLY from the [`registry`], never a constructor.
//! Cribs rustc's `Diag` data model (`crib-1`/`crib-2`) and Elm's narrative render tone
//! (`crib-6`); REFUSES rustc's Fluent/derive authoring DSL by name (`refuse-1` — also forced by
//! `inv-no-unsafe`: proc-macros forbidden workspace-wide). The *types* do the work the derive
//! DSL did, with the stock compiler as the only enforcement engine.
//!
//! # One catalog, no legacy (`27V:rul-kill-legacy-diagnostic`)
//!
//! This is the ONE diagnostics mechanism — the legacy string-slug `Diagnostic` is gone. Every
//! give-up/disclosure the analyzer emits is a typed [`DiagCode`] variant; all user-facing prose
//! lives in the committed [`crate::catalog`] keyed by slug, filled from the payload via
//! [`params_of`]. Message text is authored NOWHERE else (`AID-NEEDS:defining-case-catalog`).
//!
//! # Invariants honored here (cite the slug)
//!
//! * `inv-no-throw` — every constructor returns data; nothing panics (`22B` §3).
//! * `inv-determinism` — ordered collections only (`Vec`, never a hashed map iterated to
//!   output); [`registry`] is a pure `match`.
//! * `inv-no-unsafe` — stock `#[derive]`s only; no macros, no proc-macros.
//! * `inv-referent-agnostic` — a payload's text is display-only, never decoded for meaning; the
//!   [`ProvId`] cause is opaque and non-`Display`.
//! * `inv-site-keyed-results` — [`SiteId`] preserves command-site keying (promoted from the
//!   cli's `RecordKey`).

use crate::RenderCtx;
use crate::Severity;
use crate::arrangement::{ComponentText, component_text};
use crate::foreign::{ForeignBytes, ParamText};
use crate::said::Said;
use dorc_core::{Capability, EscalationDial, ProvId, Span, TopCause};

// ===========================================================================
// The catalog enum (exhaustive spine) + typed per-variant payloads (type-sketch-1)
// ===========================================================================

/// The exhaustive catalog of every diagnostic the analyzer emits *through this spine*. One
/// variant per give-up/disclosure class; the compiler enforces handle-every-code (`226` §12 /
/// `22A` concl-7) — every `match` on this enum (the [`registry`], [`render_cli`],
/// [`render_artifact_comment`], the OOB projection) breaks until a new variant is handled.
///
/// NO `#[non_exhaustive]` (conductor decision, verified against the workspace): `#[non_exhaustive]`
/// forces DOWNSTREAM-crate matches to add a wildcard arm — the exact opposite of the
/// workspace-wide handle-every-code the catalog exists for. Every consumer here is an internal
/// workspace crate, so exhaustiveness is the feature, not a hazard.
///
/// Each variant carries a TYPED payload demanding exactly the objects the diagnostic cites
/// (`22B` `type-sketch-1`, the capability instinct made structural): you cannot author the
/// diagnostic wrong because you cannot NAME the wrong objects. Adding a code is ONE variant
/// here + ONE [`registry`] arm + ONE [`params_of`] arm + ONE [`crate::catalog`] entry — the
/// `22B` §7 friction test, bounded and compiler-guided.
///
/// Scope: every give-up/disclosure the analyzer emits (`27V:rul-kill-legacy-diagnostic`); the
/// legacy string-slug mechanism is retired. Variant kinds: PASSTHROUGH codes carry a `detail`
/// the emit site fills (catalog message `sm {detail}`); TEMPLATIZED codes carry named params a
/// real `sm <template>` interpolates (`params_of`).
///
/// # Coming here from a `.loom` file
///
/// A defining case names its diagnostic by SLUG (`code: cli-file-not-found`). The slug and the
/// variant are the same name in two spellings — [`Self::slug`] is the whole mapping, one arm per
/// variant — and the variant's payload STRUCT (`CliFileNotFound`, declared beside this enum) is
/// the complete set of values that case's prose may interpolate. To make a NEW value available to
/// a loom:
///
/// 1. add the field to that payload struct;
/// 2. name it in the payload's [`params_of`] arm — the arm destructures exhaustively, so the
///    compiler stops you here rather than letting the value be silently loom-invisible;
/// 3. fill it in wherever the diagnostic is minted (the compiler names every site);
/// 4. rebuild, and `dorc-loom vars --all <case>` lists it.
///
/// Nothing in `dorc-loom` needs touching for any of that, and nothing there is an edit surface
/// (`28L:rul-rust-and-loom-are-the-only-edit-surfaces`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagCode {
    // ── round-22 §5 worked examples ─────────────────────────────────────────
    /// A `$(…)`/runtime-dynamic operand (or the command word itself) forced a command to ⊤
    /// (`Opaque` ⇒ it runs, never elided). The find-3 no-silent-phantoms disclosure.
    CmdsubOperandTop(CmdsubOperandTop),
    /// A probe could not ship a read-only check for this site ⇒ the apply runs it
    /// (`kFAIL-perform`). The cli-edge readout of `ProbePlan::unresolvable`.
    SiteUnresolvable(SiteUnresolvable),
    /// The leaf-exact render REFUSED to elide a licensed leaf because it carries a heredoc —
    /// its span covers the `<<` opener, not the body — so the leaf runs verbatim
    /// (`kFAIL-perform`; arch-1 d-6). An Error-class give-up (a broken artifact would ship
    /// otherwise).
    RenderHeredocRefused(RenderHeredocRefused),

    // ── B4 mechanical sweep: former `diag::legacy` survivors ────────────────
    /// A command runs inside a `$(…)` substitution body — effect-bearing but not independently
    /// elidable (it runs whenever its enclosing line runs). `219` q-1.f silent-1/silent-4.
    CmdsubInnerNonleaf(CmdsubInnerNonleaf),
    /// A WRITE-shaped redirect (`>`/`>>`) to a DYNAMIC/unresolved target joins ⊤ (y-1, `21F`
    /// imp-1). The redirect target is unresolvable so no per-path `file` cell can be keyed.
    RedirTargetTop(RedirTargetTop),
    /// A transitively-inlined (depth-2) call whose own call-argument references a positional
    /// that does not thread two inline levels (`216` §1.2 correction). The call runs verbatim.
    Depth2PositionalUnthreaded(Depth2PositionalUnthreaded),

    // ── B4 mechanical sweep: syntax/parser.rs ───────────────────────────────
    /// An unmodeled or out-of-scope sh construct (a ⊤-reject); the construct becomes an
    /// `Unsupported` node and parsing continues (`inv-top-reject`).
    SyntaxUnsupported(SyntaxUnsupported),
    /// A structurally malformed sh construct (a parse error); parsing continues fail-soft.
    SyntaxMalformed(SyntaxMalformed),

    // ── B4 mechanical sweep: analysis/cfg.rs ────────────────────────────────
    /// An `Unsupported` AST ⊤-node became a CFG `Top` node — any command that runs after
    /// it may mutate anything (the conservative ⊤-absorbing semantics).
    CfgTopNode(CfgTopNode),
    /// The errexit-region analysis encountered an unknown/unmodeled command; the `set -e`
    /// failure-edge is conservatively assumed.
    CfgErexitUnknown(CfgErexitUnknown),
    /// A call to a function could not be inlined (budget exceeded, recursive, or out-of-
    /// modeled-subset); the call runs as an ordinary unmodeled command.
    CfgInlineRefused(CfgInlineRefused),
    /// A book funcdef shadows a shell builtin the engine assumes resolves to the real builtin
    /// (`is_target_state_pure_builtin`, `standin_sh`); the assumption may be unsound (find-I).
    CfgBuiltinShadowed(CfgBuiltinShadowed),

    // ── B4 mechanical sweep: analysis/effect.rs ─────────────────────────────
    /// A check's declared kind annotation disagrees with the effect-map kind for the same verb;
    /// the annotation (declared identity) wins (204 §6 open seam).
    EffectKindDisagreement(EffectKindDisagreement),

    // ── B4 mechanical sweep: oracle/predict/parser.rs ─────────────────────────
    /// A check function body contains a construct outside the check dialect (the check
    /// dialect is a strict subset of sh; out-of-dialect input is a lift failure).
    PredictOutOfDialect(PredictOutOfDialect),
    /// A check function body is structurally unterminated (a missing `;;` or `esac` etc.).
    PredictUnterminated(PredictUnterminated),
    /// A role-funcdef the file DECLARES is absent from the lifted set and no other diagnostic said
    /// why — its binds and marks went inert while the file still parsed clean (`26G`'s
    /// silence-is-the-common-cause class). Cause-AGNOSTIC by design: it reports the LOSS, never a
    /// reason, so it catches drop paths and unrouted roles nobody has found yet.
    OracleRoleFnUnlifted(OracleRoleFnUnlifted),
    /// A trailing effect mark rode an item of an and-or list, where the rc it claims to read is
    /// the LIST's rather than the command's — so the mark is refused and the cell unminted.
    MarkOnAndOrList(MarkOnAndOrList),

    // ── oracle/reserved.rs (munge-reservation lint) ─────────────────────────
    /// An emitted `<munged>__<role>` funcname is not a legal sh NAME (leading digit / dot /
    /// non-ASCII) ⇒ REFUSED (a broken function name cannot ship).
    MungeNameInvalid(MungeNameInvalid),
    /// Two DISTINCT source names munge to one sh function name ⇒ REFUSED, never silently merged.
    MungeNameCollision(MungeNameCollision),
    /// A book funcdef squats the reserved `<x>__<role>` oracle namespace (a coincidental capture).
    ReservedNamespaceSquat(ReservedNamespaceSquat),

    // ── oracle/marker.rs (marker gate) ──────────────────────────────────────
    /// A dorc-lang dialect construct appears in a file lacking the version marker.
    MissingDialectMarker(MissingDialectMarker),
    /// A dorc-lang dialect construct appears in a file whose version marker names an unrecognized
    /// version (distinct from a wholly-missing marker).
    MarkerVersionUnrecognized(MarkerVersionUnrecognized),

    // ── oracle/load_inert.rs (the marked-file load-inertness gate) ──────────
    /// A marker-carrying file's top level holds something other than a function definition or a
    /// bare assignment, so LOADING it is not provably a no-op (`28K` §2a
    /// `rul-marked-file-is-load-inert`). Spanned at the offending top-level item.
    OracleFileNotLoadInert(OracleFileNotLoadInert),

    // ── cli (the cross-unit shadow refusal) ─────────────────────────────────
    /// One unit's role definition overrode a family a DIFFERENT unit defined, with no intervening
    /// `unset -f` (`28K` §1 `rul-silent-shadowing-refuses`). The family's licenses are withheld;
    /// its sites run. Spanned at the shadowing definition's name.
    RoleFamilyContested(RoleFamilyContested),
    /// A book defines a role function BELOW sites its family could otherwise have answered
    /// (`28K` §2 `rul-visibility-is-full-positional`): the definition licenses nothing above
    /// itself, and moving it up recovers those sites. Spanned at the definition's name.
    RoleDefinedBelowItsSites(RoleDefinedBelowItsSites),
    /// A book defines a KIND-OWNER role — the vocabulary tier, which loads from the ambient prefix
    /// only (`28M:obl-in-book-vocabulary-role-notice`). It is refused with a notice rather than
    /// silently ignored. Spanned at the definition's name.
    InBookVocabularyRole(InBookVocabularyRole),

    // ── oracle/entry.rs (tolerance vouch + corroboration) ───────────────────
    /// An unknown context-dimension token on a `tolerates:` vouch (walls that dimension).
    ToleratesUnknownDimension(ToleratesUnknownDimension),
    /// A `tolerates:user` vouch over a body that visibly reads identity (corroboration ask).
    ToleratesOverIdentityDependence(ToleratesOverIdentityDependence),
    /// A body reads identity but carries no tolerance vouch (the one-line adoption hint).
    HeavyContextNoTolerance(HeavyContextNoTolerance),

    // ── oracle/wrapper.rs (lend_map lift) ───────────────────────────────────
    /// An unknown `lend_map` dimension token (mints no lend; the dimension walls).
    LendMapUnknownDimension(LendMapUnknownDimension),

    // ── oracle/carry.rs (pure-predicate carry) ──────────────────────────────
    /// An `invariant:netns` claim on a per-netns `net-kernel` store — a contradiction, dropped.
    CarryNetnsOnNetKernelForbidden(CarryNetnsOnNetKernelForbidden),

    // ── oracle/predict/derive.rs (verdict-mark derivation) ──────────────────
    /// A brace-alternation `@{a,b}` on a single-cell verdict/observe mark (mints no cell).
    MarkBraceVerdictSingleCell(MarkBraceVerdictSingleCell),

    // ── oracle/predict (the `281` mark-grammar parse — new-grammar path) ─────
    /// A period-free head/continuation token that is not a known mark verb (`281` §4 rule-3
    /// miss) — the block drops to ⊤ (`inv-top-reject`).
    MarkUnknownVerb(MarkUnknownVerb),
    /// Two rc-consuming marks (`asserts`/`refutes`) in one block (`281` §7 rc-arity) — one
    /// exit code cannot witness two cells, so the block drops to ⊤.
    MarkRcArityExceeded(MarkRcArityExceeded),
    /// A standalone mark-block (no command to bind) carries an rc-consumer or `reads`
    /// (`28A:rul-continuation-attachment`) — nothing to measure/back, so it drops to ⊤.
    MarkStandaloneRcConsumer(MarkStandaloneRcConsumer),
    /// A `#:` comment looks like a mark-block but did not parse (`281` §9) — left a comment,
    /// diagnosed (the hash-colon carrier never silently mis-erases).
    MarkHashcolonMalformed(MarkHashcolonMalformed),

    // ── plan/records.rs (framed records deframer) ───────────────────────────
    /// A records stream carried no framing at all (headerless) — refused, the host runs.
    RecordsHeaderlessRefused(RecordsHeaderlessRefused),
    /// A records line carried bytes after its terminal token (two writes glued) — refused.
    RecordsGluedLine(RecordsGluedLine),
    /// A framed records stream carried no header (torn/absent) — refused, the host runs.
    RecordsHeaderMissing(RecordsHeaderMissing),
    /// The end-sentinel carried a nonce that is not this attempt's — ignored.
    RecordsSentinelNonce(RecordsSentinelNonce),
    /// The fact lane truncated: fewer site records received than declared (unseen sites run).
    RecordsFactTruncated(RecordsFactTruncated),
    /// The records header failed an integrity key (nonce/attempt/host/book) — refused.
    RecordsIntegrityRefused(RecordsIntegrityRefused),
    /// Torn (no terminal token) record lines discarded (counted, never folded).
    RecordsTornLine(RecordsTornLine),
    /// Alien (non-nonce) record lines discarded (counted, never folded).
    RecordsAlienLine(RecordsAlienLine),
    /// Late (after the end-sentinel) record lines discarded (counted, never folded).
    RecordsLateLine(RecordsLateLine),
    /// Controller admission refused hostile host evidence before it entered the decision plane.
    HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused),

    // ── cli/main.rs (footprint / escalation / carry disclosures) ────────────
    /// A `touches()` footprint is incoherent (omits its own effect coordinate, or a malformed
    /// derived coordinate) — footprint refused, the site walls.
    FootprintIncoherent(FootprintIncoherent),
    /// A payload-bound `touches()` escalated to host-derivation (the spike-only advisory).
    TouchesEscalated(TouchesEscalated),
    /// A derived footprint family did not close completely — footprint refused, the site walls.
    DerivFamilyIncomplete(DerivFamilyIncomplete),
    /// The authority-disclosure line for the probe-escalation policy (consent legibility).
    EscalationPolicy(EscalationPolicy),
    /// The pure-predicate-carry attribution chain, rendered at every carried elision.
    CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis),
    /// A wrapped BOOK site degraded on a missing `tolerates:` vouch (the adoption hint).
    WrappedSiteAdoptionHint(WrappedSiteAdoptionHint),

    // ── cli/main.rs (resolver / reaches confusability) ──────────────────────
    /// Two oracle files declare one kind's resolver — BOTH refused, token-equality kept.
    ResolverConflict(ResolverConflict),
    /// A resolver keyed to a name matching a known COMMAND provider (a likely mis-key).
    ResolverProviderCollision(ResolverProviderCollision),
    /// A coordinate resolved DANGLING (no such entity on an enumerable kind) — the site runs.
    DanglingReference(DanglingReference),
    /// Two or more sites measured ONE shared cell and reported different things, so the meet ⊤s
    /// the cell and de-licenses every site on it — including sites that reported cleanly. One
    /// line per cell, not per disagreeing pair (`26G:fnd-shared-auto-cell-collides`).
    SharedCellMeasurementsDisagree(SharedCellMeasurementsDisagree),
    /// Two oracle files declare one kind's reach-function — BOTH refused, no expansion.
    ReachesConflict(ReachesConflict),
    /// A reach-function keyed to a name matching a known COMMAND provider (a likely mis-key).
    ReachesProviderCollision(ReachesProviderCollision),

    // ── cli/main.rs (wrapper coherence fail-fast) ───────────────────────────
    /// A wrapper's `__enter` and `__lend_map` disagree on argv flow — static incoherence.
    WrapperEntryIncoherent(WrapperEntryIncoherent),
    /// A wrapper's `__predict` and `__lend_map` disagree on the peel tail — static incoherence.
    WrapperPeelIncoherent(WrapperPeelIncoherent),

    // ── cli/main.rs + plan/whylog.rs (`dorc why --last` durable reader — `27V` Lane B) ──────
    /// `dorc why --last` found a durable written by a format version this binary does not
    /// understand — refuse politely (never replay a format we cannot parse).
    WhylogVersionRefused(WhylogVersionRefused),
    /// The durable's recorded book/oracle digest (or its stored decision digest) diverges from
    /// the current on-disk inputs — the replay would not reconstruct the recorded run.
    WhylogBookDesync(WhylogBookDesync),
    /// `dorc why --last` found no durable to replay in the whylog directory.
    WhylogAbsent(WhylogAbsent),
    /// A durable was found but is truncated / unparseable — diagnostics, never a panic.
    WhylogCorrupt(WhylogCorrupt),
    /// The run's durable could not be persisted, so no receipt exists for it to be asked about
    /// later. Error-floor (`28F:rul-write-failure-is-error-floor`): the advisory plane is
    /// suppressed under `apply`, which is exactly the run whose receipt matters most.
    WhylogUnwritten(WhylogUnwritten),

    // ── cli/main.rs (aid hints) — `AID-NEEDS:aid-unloaded-sibling-oracle` (gap-5, ack-6) ──────
    /// Sibling `*.oracle.sh` files sit on disk beside the loaded set but were not loaded — a
    /// suggest-never-auto-load hint (`24H` ack-6). Advisory; the run is unchanged.
    AidUnloadedSiblingOracle(AidUnloadedSiblingOracle),

    // ── dorc-lint's own findings (`288` §5) — the lane-local namespace retired ────────────────
    /// The book carries unmodeled ⊤-walls; downstream sites lose full elision until each wall's
    /// tool has an oracle.
    UnmodeledWallInventory(UnmodeledWallInventory),
    /// A verdict body answers with a PIPELINE's tail status, so the rc may not be the described
    /// tool's (`rul-rc-partition`).
    VerdictTerminalPipeline(VerdictTerminalPipeline),
    /// A verdict arm authors a deliberate decline whose class was read statically; the site runs
    /// and the class routes the nags.
    AuthoredDeclineClass(AuthoredDeclineClass),
    /// A verdict arm authors a deliberate decline whose class is NOT statically readable — a
    /// different world-state, resolved only at runtime.
    AuthoredDeclineClassUnreadable(AuthoredDeclineClassUnreadable),
    /// A configured external linter is not on PATH, so its checks did not run.
    LintToolAbsent(LintToolAbsent),
    /// An external linter produced output the adapters could not parse at any tier.
    LintToolOutputUnparsable(LintToolOutputUnparsable),
    /// An external linter exited nonzero but produced no parseable findings.
    LintToolFailedWithoutFindings(LintToolFailedWithoutFindings),

    // ── invocation errors (`288` §6) — the `dorc: {msg}` family, now registry codes ───────────
    /// `dorc strip` was given no path.
    CliStripNeedsPath(CliStripNeedsPath),
    /// `dorc strip`'s sole positional was a flag, not a path.
    CliStripGotAFlag(CliStripGotAFlag),
    /// The leading mode token is not a mode, but is a near-miss for one.
    CliUnknownMode(CliUnknownMode),
    /// A flag that takes a value was given without one.
    CliFlagNeedsValue(CliFlagNeedsValue),
    /// An unrecognized flag, with no near-miss to suggest.
    CliUnknownFlag(CliUnknownFlag),
    /// An unrecognized flag that is a near-miss for a real one.
    CliUnknownFlagDidYouMean(CliUnknownFlagDidYouMean),
    /// A flag's value is outside its closed vocabulary.
    CliFlagValueNotRecognized(CliFlagValueNotRecognized),
    /// A flag wanting a number was given something else.
    CliFlagValueNotANumber(CliFlagValueNotANumber),
    /// No book was given, by positional or by flag.
    CliNoBookGiven(CliNoBookGiven),
    /// Two flags that cannot both be given were.
    CliFlagsMutuallyExclusive(CliFlagsMutuallyExclusive),
    /// A flag valid only under one mode was given under another.
    CliFlagRequiresMode(CliFlagRequiresMode),
    /// An input file does not exist.
    CliFileNotFound(CliFileNotFound),
    /// An input file exists but is not readable by this process.
    CliFilePermissionDenied(CliFilePermissionDenied),
    /// An input file failed to read for some other OS reason.
    CliFileUnreadable(CliFileUnreadable),
    /// `dorc lint` was given nothing lintable.
    LintNoLintableFiles(LintNoLintableFiles),
    /// The lintable-file count disagrees with `--expect-files`.
    LintFileCountDrift(LintFileCountDrift),
    /// `--require-tools` was given and a configured tool is absent.
    LintRequiredToolsMissing(LintRequiredToolsMissing),
    /// `dorc-sh` was invoked with no script.
    DorcShUsage(DorcShUsage),
    /// `dorc-sh` could not read its script.
    DorcShScriptUnreadable(DorcShScriptUnreadable),
    /// `dorc-sh` could not exec the stock shell.
    DorcShExecFailed(DorcShExecFailed),
    /// The per-run PATH shim directory could not be created or written.
    CliShimDirUnwritable(CliShimDirUnwritable),
    /// Bytes about to be shipped to a host carry a carriage return.
    TransportCrlfRefused(TransportCrlfRefused),
    /// A session ran and never reported completion, so the host's state is unknown.
    TransportSessionLost(TransportSessionLost),
    /// No session process could be created, so the host was never contacted.
    TransportSpawnRefused(TransportSpawnRefused),
    TransportMarkerUnusable(TransportMarkerUnusable),
    /// A remote apply ran to completion and its artifact exited non-zero.
    TransportApplyFailed(TransportApplyFailed),
}

impl DiagCode {
    /// The stable wire/grep slug for this code (`22B-fork-wire-code` = string slug; a
    /// WIRE-FORMAT COMMITMENT — flagged). Stable across variant reordering (unlike a numeric
    /// discriminant), greppable, and the key the OOB lane's `code=` field carries
    /// (`226` finding-6, TS's code-stable discipline). These slugs match the legacy
    /// `Diagnostic` strings the migrated sites used, so existing `expected-diagnostics`
    /// fixtures and the coverage bridge keep matching.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per code, and that ONE-TO-ONE shape is the property: a wildcard or a \
                  derived spelling would let a new code ship without a deliberate slug, which is a \
                  wire-format commitment (`288:rul-error-slugs-are-semantic`)"
    )]
    pub fn slug(&self) -> &'static str {
        match self {
            DiagCode::CmdsubOperandTop(_) => "cmdsub-operand-top",
            DiagCode::SiteUnresolvable(_) => "site-unresolvable",
            DiagCode::RenderHeredocRefused(_) => "render-heredoc-refused",
            DiagCode::CmdsubInnerNonleaf(_) => "cmdsub-inner-nonleaf",
            DiagCode::RedirTargetTop(_) => "redir-target-top",
            DiagCode::Depth2PositionalUnthreaded(_) => "depth-2-positional-unthreaded",
            DiagCode::SyntaxUnsupported(_) => "syntax-unsupported",
            DiagCode::SyntaxMalformed(_) => "syntax-malformed",
            DiagCode::CfgTopNode(_) => "cfg-top-node",
            DiagCode::CfgErexitUnknown(_) => "cfg-errexit-unknown",
            DiagCode::CfgInlineRefused(_) => "cfg-inline-refused",
            DiagCode::CfgBuiltinShadowed(_) => "cfg-builtin-shadowed",
            DiagCode::EffectKindDisagreement(_) => "effect-kind-disagreement",
            DiagCode::PredictOutOfDialect(_) => "predict-out-of-dialect",
            DiagCode::PredictUnterminated(_) => "predict-unterminated",
            DiagCode::OracleRoleFnUnlifted(_) => "oracle-role-fn-unlifted",
            DiagCode::MarkOnAndOrList(_) => "mark-on-and-or-list",
            DiagCode::MungeNameInvalid(_) => "munge-name-invalid",
            DiagCode::MungeNameCollision(_) => "munge-name-collision",
            DiagCode::ReservedNamespaceSquat(_) => "reserved-namespace-squat",
            DiagCode::OracleFileNotLoadInert(_) => "oracle-file-not-load-inert",
            DiagCode::RoleFamilyContested(_) => "role-family-contested",
            DiagCode::RoleDefinedBelowItsSites(_) => "role-defined-below-its-sites",
            DiagCode::InBookVocabularyRole(_) => "in-book-vocabulary-role",
            DiagCode::MissingDialectMarker(_) => "missing-dialect-marker",
            DiagCode::MarkerVersionUnrecognized(_) => "marker-version-unrecognized",
            DiagCode::ToleratesUnknownDimension(_) => "tolerates-unknown-dimension",
            DiagCode::ToleratesOverIdentityDependence(_) => "tolerates-over-identity-dependence",
            DiagCode::HeavyContextNoTolerance(_) => "heavy-context-no-tolerance",
            DiagCode::LendMapUnknownDimension(_) => "lend-map-unknown-dimension",
            DiagCode::CarryNetnsOnNetKernelForbidden(_) => "carry-netns-on-net-kernel-forbidden",
            DiagCode::MarkBraceVerdictSingleCell(_) => "mark-brace-verdict-single-cell",
            DiagCode::MarkUnknownVerb(_) => "mark-unknown-verb",
            DiagCode::MarkRcArityExceeded(_) => "mark-rc-arity-exceeded",
            DiagCode::MarkStandaloneRcConsumer(_) => "mark-standalone-rc-consumer",
            DiagCode::MarkHashcolonMalformed(_) => "mark-hashcolon-malformed",
            DiagCode::RecordsHeaderlessRefused(_) => "records-headerless-refused",
            DiagCode::RecordsGluedLine(_) => "records-glued-line",
            DiagCode::RecordsHeaderMissing(_) => "records-header-missing",
            DiagCode::RecordsSentinelNonce(_) => "records-sentinel-nonce",
            DiagCode::RecordsFactTruncated(_) => "records-fact-truncated",
            DiagCode::RecordsIntegrityRefused(_) => "records-integrity-refused",
            DiagCode::RecordsTornLine(_) => "records-torn-line",
            DiagCode::RecordsAlienLine(_) => "records-alien-line",
            DiagCode::RecordsLateLine(_) => "records-late-line",
            DiagCode::HostEvidenceAdmissionRefused(_) => "host-evidence-admission-refused",
            DiagCode::FootprintIncoherent(_) => "footprint-incoherent",
            DiagCode::TouchesEscalated(_) => "touches-escalated",
            DiagCode::DerivFamilyIncomplete(_) => "deriv-family-incomplete",
            DiagCode::EscalationPolicy(_) => "escalation-policy",
            DiagCode::CarriedAcrossSubstrateAxis(_) => "carried-across-substrate-axis",
            DiagCode::WrappedSiteAdoptionHint(_) => "wrapped-site-adoption-hint",
            DiagCode::ResolverConflict(_) => "resolver-conflict",
            DiagCode::ResolverProviderCollision(_) => "resolver-provider-collision",
            DiagCode::DanglingReference(_) => "dangling-reference",
            DiagCode::SharedCellMeasurementsDisagree(_) => "shared-cell-measurements-disagree",
            DiagCode::ReachesConflict(_) => "reaches-conflict",
            DiagCode::ReachesProviderCollision(_) => "reaches-provider-collision",
            DiagCode::WrapperEntryIncoherent(_) => "wrapper-entry-incoherent",
            DiagCode::WrapperPeelIncoherent(_) => "wrapper-peel-incoherent",
            DiagCode::WhylogVersionRefused(_) => "whylog-version-refused",
            DiagCode::WhylogBookDesync(_) => "whylog-book-desync",
            DiagCode::WhylogAbsent(_) => "whylog-absent",
            DiagCode::WhylogCorrupt(_) => "whylog-corrupt",
            DiagCode::WhylogUnwritten(_) => "whylog-unwritten",
            DiagCode::AidUnloadedSiblingOracle(_) => "aid-unloaded-sibling-oracle",
            DiagCode::UnmodeledWallInventory(_) => "unmodeled-wall-inventory",
            DiagCode::VerdictTerminalPipeline(_) => "verdict-terminal-pipeline",
            DiagCode::AuthoredDeclineClass(_) => "authored-decline-class",
            DiagCode::AuthoredDeclineClassUnreadable(_) => "authored-decline-class-unreadable",
            DiagCode::LintToolAbsent(_) => "lint-tool-absent",
            DiagCode::LintToolOutputUnparsable(_) => "lint-tool-output-unparsable",
            DiagCode::LintToolFailedWithoutFindings(_) => "lint-tool-failed-without-findings",
            DiagCode::CliStripNeedsPath(_) => "cli-strip-needs-path",
            DiagCode::CliStripGotAFlag(_) => "cli-strip-got-a-flag",
            DiagCode::CliUnknownMode(_) => "cli-unknown-mode",
            DiagCode::CliFlagNeedsValue(_) => "cli-flag-needs-value",
            DiagCode::CliUnknownFlag(_) => "cli-unknown-flag",
            DiagCode::CliUnknownFlagDidYouMean(_) => "cli-unknown-flag-did-you-mean",
            DiagCode::CliFlagValueNotRecognized(_) => "cli-flag-value-not-recognized",
            DiagCode::CliFlagValueNotANumber(_) => "cli-flag-value-not-a-number",
            DiagCode::CliNoBookGiven(_) => "cli-no-book-given",
            DiagCode::CliFlagsMutuallyExclusive(_) => "cli-flags-mutually-exclusive",
            DiagCode::CliFlagRequiresMode(_) => "cli-flag-requires-mode",
            DiagCode::CliFileNotFound(_) => "cli-file-not-found",
            DiagCode::CliFilePermissionDenied(_) => "cli-file-permission-denied",
            DiagCode::CliFileUnreadable(_) => "cli-file-unreadable",
            DiagCode::LintNoLintableFiles(_) => "lint-no-lintable-files",
            DiagCode::LintFileCountDrift(_) => "lint-file-count-drift",
            DiagCode::LintRequiredToolsMissing(_) => "lint-required-tools-missing",
            DiagCode::DorcShUsage(_) => "dorc-sh-usage",
            DiagCode::DorcShScriptUnreadable(_) => "dorc-sh-script-unreadable",
            DiagCode::DorcShExecFailed(_) => "dorc-sh-exec-failed",
            DiagCode::CliShimDirUnwritable(_) => "cli-shim-dir-unwritable",
            DiagCode::TransportCrlfRefused(_) => "transport-crlf-refused",
            DiagCode::TransportSessionLost(_) => "transport-session-lost",
            DiagCode::TransportSpawnRefused(_) => "transport-spawn-refused",
            DiagCode::TransportMarkerUnusable(_) => "transport-marker-unusable",
            DiagCode::TransportApplyFailed(_) => "transport-apply-failed",
        }
    }
}

/// The position in a command's argv that went ⊤ (`22B` `type-sketch-1`): the command word
/// itself, or a 1-based operand index (excluding the command word). A newtype over the bare
/// `&str` the legacy `cmdsub_operand_top` took — the value plane is cause-erased to ⊤ at this
/// point, so the diagnostic names the POSITION, never the original text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandPosition {
    /// `argv[0]` — the command word is itself a `$(…)`/dynamic value.
    CommandWord,
    /// Operand `n` (1-based, command word excluded) is a `$(…)`/dynamic value.
    Operand(u32),
}

impl OperandPosition {
    /// The words for this position (`the command word` / `operand N`), from the arrangement
    /// registry: one occurrence per form, the index interleaved as the seat's value.
    ///
    /// User-facing prose in a fact-plane type was the second, unnoticed instance of
    /// `289:finding-reason-opener-still-hardcoded` — every string a person reads is
    /// registry-homed (`288` §1), and a `describe()` on an enum is not an exemption.
    #[must_use]
    pub fn describe(self, ctx: &RenderCtx<'_>) -> String {
        match self {
            OperandPosition::CommandWord => {
                crate::said::words_text(ctx, POSITION_WORDS, Some(0), &[])
            }
            OperandPosition::Operand(n) => {
                crate::said::words_text(ctx, POSITION_WORDS, Some(1), &[&n.to_string()])
            }
        }
    }
}

/// The registry slug holding the ⊤ position's two forms (occurrence 0 = the command word,
/// occurrence 1 = a 1-based operand index).
const POSITION_WORDS: &str = "why-operand-position";

/// The command-word name a diagnostic carries for its `{command}` template param (`282` §12
/// item-6): a value-flow-derived, three-state name so a message can speak the command in the
/// caller's terms (the human's stated need across MANY future messages). Populated at the analysis
/// emit site where the resolved argv is known — never synthesized late.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandName {
    /// A statically-resolved literal command word — renders the bare name (`apt-get`).
    Literal(String),
    /// A dynamic command word that constant-propagation resolved to a known name — renders the
    /// "which resolves to" phrasing. The TYPE is shaped now; analysis-side population is a marked
    /// follow-up (it needs value-flow provenance the emit site does not yet distinguish).
    Resolved(String),
    /// No single clear command name (a ⊤ command word) — renders a name-free fallback.
    Unclear,
}

impl CommandName {
    /// The `{command}` fill text (`282` §12 item-6): the bare name for a literal, a resolves-to
    /// clause for a const-prop'd dynamic word, a neutral fallback when no single name is clear. The
    /// engine-owned canonical formatter for this param (the `describe()` family, `27V` §3).
    ///
    /// The two WORDED forms are registry rows, the third `289:finding-reason-opener-still-hardcoded`
    /// instance: a `describe()` on an enum is not an exemption from `288` §1. A LITERAL name stays
    /// out of the registry — it is the world's own word, not a sentence of ours.
    #[must_use]
    pub fn describe(&self, ctx: &RenderCtx<'_>) -> String {
        match self {
            CommandName::Literal(name) => name.clone(),
            CommandName::Resolved(name) => {
                crate::said::words_text(ctx, COMMAND_NAME_WORDS, Some(0), &[name])
            }
            CommandName::Unclear => crate::said::words_text(ctx, COMMAND_NAME_WORDS, Some(1), &[]),
        }
    }
}

/// The registry slug holding the `{command}` fill's two worded forms (occurrence 0 = a
/// const-propagated dynamic word, occurrence 1 = no clear name).
const COMMAND_NAME_WORDS: &str = "why-command-name";

/// Payload of [`DiagCode::CmdsubOperandTop`]: the ⊤-origin site, WHICH position went ⊤, an optional
/// ⊤-cause receipt (`228` dc-1 — the exempt-plane hook that links this origin to its poisoned
/// downstream consumers without each consumer emitting), and the command-word name for `{command}`.
/// The `cause` is EXEMPT-plane (it is a [`ProvId`], opaque and non-`Display`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdsubOperandTop {
    /// The command-site that went ⊤.
    pub site: SiteId,
    /// Which argv position is the `$(…)`/dynamic value.
    pub position: OperandPosition,
    /// The ⊤-cause origin (arch-1 `ProvId` arena), if minted. EXEMPT-plane (`Exempt::ReceiptId`):
    /// it rides the diagnostic for the why-lens/dashboard dedup but reaches no artifact and
    /// drives no decision.
    pub cause: Option<ProvId>,
    /// The category of ⊤-cause, for the template's `{cause}` fill (`top_cause.describe()`); the
    /// message-plane companion to the exempt-plane [`cause`](Self::cause) receipt.
    pub top_cause: TopCause,
    /// The command-word name for the `{command}` fill (`command.describe()`) — value-flow-derived
    /// at the emit site (`282` §12 item-6).
    pub command: CommandName,
}

/// Payload of [`DiagCode::SiteUnresolvable`]: the probe-unresolvable sites the apply will run.
///
/// De-passthrough'd (`282:rul-passthrough-type-gated`): the disclosure sentence was OURS and now
/// lives in the catalog register, with the two genuinely-foreign values — the named sites and the
/// quoted first command, both book bytes — sealed. The [`SiteId`] is the blamed handle the probe
/// record keys back to (`inv-site-keyed-results`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteUnresolvable {
    /// The probe-unresolvable command-site (the apply runs it).
    pub site: SiteId,
    /// How many sites are disclosed (`{count}`).
    pub count: String,
    /// `site` or `sites`, agreeing with `count` (`{site_word}`).
    pub site_word: &'static str,
    /// The named sites, each backticked with its give-up cause (`{names}`) — book bytes.
    pub names: ForeignBytes,
    /// The first site's source, as the caret's representative (`{excerpt}`) — book bytes.
    pub excerpt: ForeignBytes,
}

/// Payload of [`DiagCode::RenderHeredocRefused`]: the heredoc-bearing site the leaf-exact render
/// refused to elide (`22B` `worked-2`). The legacy form was an inline literal (`21Z`: "not even
/// a named const"); the typed payload makes it a first-class enum variant the grep gate sees
/// and the dashboard can group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderHeredocRefused {
    /// The heredoc-carrying command-site that runs verbatim instead of being elided.
    pub site: SiteId,
    /// The disposition verb the refusal names (`elide` for a Replace/Omit, `guard` for a Guard)
    /// — the template's `{verb}` fill.
    pub verb: &'static str,
    /// The one-line command text the refusal points at (display only) — the template's `{command}`.
    pub command: String,
}

// ===========================================================================
// B4 sweep payload structs — one per migrated legacy code
// ===========================================================================

/// Payload of [`DiagCode::CmdsubInnerNonleaf`]: a command inside a `$(…)` body is
/// effect-bearing but not an independent plan leaf. The `site` uses the CFG-node-id space
/// (pre-plan — same precedent as `CmdsubOperandTop`; flagged `tc-cmdsub-siteid`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmdsubInnerNonleaf {
    /// The CFG-node-space site of the inner command (pre-plan id space).
    pub site: SiteId,
    /// The inner command's resolved text (display only — `inv-referent-agnostic`).
    pub inner: String,
}

/// Payload of [`DiagCode::RedirTargetTop`]: a write-redirect to a dynamic/unresolved
/// target joins ⊤. The site uses the CFG-node-id space (pre-plan).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedirTargetTop {
    /// The CFG-node-space site of the redirect node (pre-plan id space).
    pub site: SiteId,
}

/// Payload of [`DiagCode::Depth2PositionalUnthreaded`]: a depth-2 inlined call whose
/// argument references a positional that does not thread two inline levels. The site
/// uses the CFG-node-id space (pre-plan).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Depth2PositionalUnthreaded {
    /// The CFG-node-space site of the refused call (pre-plan id space).
    pub site: SiteId,
    /// The refused call's function name (display only — `inv-referent-agnostic`).
    pub name: String,
}

/// Payload of [`DiagCode::SyntaxUnsupported`]: a parser-level ⊤-reject. No `SiteId` (the syntax
/// layer runs before CFG construction; `site()` returns `None` for this code).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxUnsupported {
    /// Which unmodeled construct the parser met.
    pub reason: SyntaxUnsupportedReason,
}

/// Which unmodeled construct the parser met (see [`CfgTopNodeReason`] for why the reason enums
/// live in this crate).
///
/// Deliberately NOT `dorc_syntax::UnsupportedReason`: that one classifies the ⊤-TRIGGER for the
/// AST node (and so for downstream analysis), while this one names the construct for the reader.
/// Several triggers share one classification and say different things about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxUnsupportedReason {
    /// The list-level anti-stall guard forced progress by rejecting one token.
    ParserStalled,
    /// The recursive-descent depth bound was reached.
    NestingBound,
    /// A reserved word appeared where a command was required.
    ReservedWordInCommandPosition,
    /// A redirection trails a compound construct.
    ConstructTrailingRedirection {
        /// The construct's opening keyword.
        construct: &'static str,
        /// The keyword that closes it.
        closer: &'static str,
    },
    /// `for` with no iteration-variable name.
    ForWithoutVariableName,
    /// `for NAME` with no `in LIST`, which iterates the runtime `"$@"`.
    ForWithoutInList,
    /// A `for` list word carries an effect-bearing expansion.
    ForListWordHasExpansion,
    /// A `for` list is not terminated where `do` is required.
    ForListNotTerminated,
    /// `break`/`continue` in a loop body.
    LoopJumpInBody,
    /// `break`/`continue` in a loop body or its condition.
    LoopJumpInBodyOrCondition,
    /// Background/async `&`.
    BackgroundAmp,
    /// A binary operator with no command in front of it.
    OperatorWithoutCommand,
    /// `;;` outside a case arm.
    DoubleSemicolonOutsideCase,
    /// A token where a command was required.
    ExpectedACommand,
    /// `$(( … ))` in command position.
    ArithmeticAsCommand,
    /// The command word is not a fixed literal.
    DynamicCommandName,
    /// `eval`.
    EvalConstructedCode,
    /// `.`/`source` of a target that is not a literal path.
    SourceOfNonLiteralTarget,
    /// `unset` of a dynamic lvalue.
    UnsetDynamicLvalue,
    /// `printf -v`, which writes to a variable lvalue.
    PrintfWritesLvalue,
    /// `test -v` / `[ -v ]`, which references a variable lvalue.
    TestReferencesLvalue,
    /// A token where a word was required.
    ExpectedAWord,
}

/// Payload of [`DiagCode::SyntaxMalformed`]: a parse error. No `SiteId` (pre-CFG).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxMalformed {
    /// Which structural expectation the source broke.
    pub reason: SyntaxMalformedReason,
}

/// Which structural expectation the source broke (see [`CfgTopNodeReason`] for why the reason
/// enums live in this crate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxMalformedReason {
    /// No `then` after an `if` condition.
    ExpectedThenAfterIf,
    /// No `then` after an `elif` condition.
    ExpectedThenAfterElif,
    /// No `fi` closing an `if`.
    ExpectedFiToCloseIf,
    /// No `in` after a `case` word.
    ExpectedInAfterCaseWord,
    /// No `esac` closing a `case`.
    ExpectedEsacToCloseCase,
    /// No `do` opening a loop body.
    ExpectedDoToOpenLoopBody,
    /// No `done` closing a loop.
    ExpectedDoneToCloseLoop,
    /// A `case` arm never closed.
    UnterminatedCaseArm,
    /// No `)` after a case pattern.
    ExpectedRparenAfterCasePattern,
    /// A subshell never closed.
    UnterminatedSubshell,
    /// A brace group never closed.
    UnterminatedBraceGroup,
}

/// Payload of [`DiagCode::CfgTopNode`]: an AST `Unsupported` node became a CFG `Top` node.
/// No `SiteId` at this level (the CFG node's own index is not surfaced to the CFG builder at
/// the point it emits this; `site()` returns `None`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgTopNode {
    /// Which ⊤-trigger fired.
    pub reason: CfgTopNodeReason,
}

/// Which ⊤-trigger minted a CFG `Top` node.
///
/// The first of the typed REASON enums (`28L:rul-reason-enums-not-sibling-codes`): a code whose
/// one `detail` hole carried N genuinely different sentences gets a typed reason instead, an
/// enum→slug map here, and one arrangement prose-component per reason — so every sentence lands
/// in a registry an author can edit, with zero new codes (the `TopCause`/`RemediationClass`
/// shape, one tier down).
///
/// The reason enums live HERE, beside the payloads they ride on, rather than in the crate that
/// decides which one to construct: `aid` depends on `core` and on nothing else
/// (`aid-is-the-describe-plane`), so a type named by a payload struct cannot live in an emitting
/// crate that already depends on `aid`. The DECISION still belongs to the emit site — which
/// variant fires is the analyzer's call, exactly as which payload struct fires already was;
/// `aid` only owns the vocabulary and its words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CfgTopNodeReason {
    /// An AST `Unsupported` node lowered to an absorbing ⊤ node.
    UnsupportedConstruct,
    /// The CFG builder's own nesting bound was reached.
    NestingBound,
}

/// Payload of [`DiagCode::CfgErexitUnknown`]: the errexit pass encountered an unknown command;
/// the failure-edge is conservatively assumed. No `SiteId` (`site()` returns `None`).
///
/// Field-less since the de-passthrough (`282:rul-passthrough-type-gated`): the reason was one
/// fixed sentence we wrote, so it belongs in the catalog register, not on the payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgErexitUnknown;

/// Payload of [`DiagCode::CfgInlineRefused`]: a call could not be inlined. No `SiteId`
/// (`site()` returns `None`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgInlineRefused {
    /// Which budget or constraint refused the inline.
    pub reason: CfgInlineRefusedReason,
}

/// Which budget or constraint refused an inline (see [`CfgTopNodeReason`] for why the reason
/// enums live in this crate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgInlineRefusedReason {
    /// The callee has more than one definition.
    Redefined {
        /// The called function's name.
        name: String,
    },
    /// The callee is already on the active inline stack.
    RecursiveCall {
        /// The called function's name.
        name: String,
    },
    /// The inline-depth budget is spent.
    DepthBudget {
        /// The called function's name.
        name: String,
        /// The depth budget.
        budget: u32,
    },
    /// The callee's body uses a positional construct outside the modeled subset.
    UnmodeledPositional {
        /// The called function's name.
        name: String,
        /// The construct the body used.
        construct: &'static str,
    },
    /// The callee's body carries a write-redirect inlining would expose as wrong-ambience.
    WriteRedirect {
        /// The called function's name.
        name: String,
        /// Which redirect fenced the call.
        redirect: UnmodeledWriteRedirect,
    },
    /// The callee's estimated node count exceeds the per-call splice budget.
    PerCallNodeBudget {
        /// The called function's name.
        name: String,
        /// The body's conservative node estimate.
        estimate: usize,
        /// The per-call budget.
        budget: usize,
    },
    /// The book's running splice tally plus this body exceeds the per-book budget.
    PerBookNodeBudget {
        /// The called function's name.
        name: String,
        /// Nodes already spliced across the book.
        spliced: usize,
        /// The body's conservative node estimate.
        estimate: usize,
        /// The per-book budget.
        budget: usize,
    },
}

/// Which write-redirect fenced an inline — the inner reason of
/// [`CfgInlineRefusedReason::WriteRedirect`], with its own components because it too was a
/// composed sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnmodeledWriteRedirect {
    /// A write to a resolved path.
    ToPath {
        /// The redirect target as the book spells it (display only).
        path: String,
    },
    /// A write to a target that does not resolve statically.
    ToDynamicTarget,
}

/// Payload of [`DiagCode::CfgBuiltinShadowed`]: a book funcdef shadows a shell builtin the
/// engine relies on. PASSTHROUGH (`sm {detail}`). Spanned (the funcdef `name_span`), so `site()`
/// returns `None` (no plan-`LeafId`) but the primary span points at the definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgBuiltinShadowed {
    /// The shadowing function name, which is also the builtin it shadows (`{name}`).
    pub name: String,
}

/// Payload of [`DiagCode::EffectKindDisagreement`]: a check's annotation kind disagrees with
/// the effect-map kind for the verb; the annotation wins. No `SiteId` (this fires mid-effect-
/// resolution with no plan leaf; `site()` returns `None`). Note: this code currently emits
/// with `span: None` at its legacy site (no span available at the annotation-vs-map check).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectKindDisagreement {
    /// The kind the check annotation declared, which wins (`{annotated}`).
    pub annotated: String,
    /// The kind the effect map holds for the verb (`{effect_map}`).
    pub effect_map: String,
}

/// Payload of [`DiagCode::PredictOutOfDialect`]: a check function body uses a construct outside
/// the check dialect. No `SiteId` (`site()` returns `None`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredictOutOfDialect {
    /// Which dialect rule the body broke.
    pub reason: PredictOutOfDialectReason,
}

/// Which check-dialect rule a body broke (see [`CfgTopNodeReason`] for why the reason enums live
/// in this crate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictOutOfDialectReason {
    /// The `name__predict` header is not followed by `()`.
    MalformedFunctionHeader,
    /// The function body does not open with `{`.
    FunctionBodyMustStartWithBrace,
    /// The whole check body failed to lift.
    CheckBodyOutOfDialect,
    /// An and-or list does not begin with a command.
    AndOrListNotLedByCommand,
    /// An and-or list item is not a command.
    AndOrListItemNotCommand,
    /// No `do` after a `while` test.
    ExpectedDoAfterWhileTest,
    /// No `then` after an `if` test.
    ExpectedThenAfterIfTest,
    /// No `in` after a `case` scrutinee.
    ExpectedInAfterCaseScrutinee,
    /// A `case` never closed.
    UnterminatedCaseExpectedEsac,
    /// No `|` or `)` in a case-arm pattern.
    ExpectedPipeOrRparenInCaseArmPattern,
    /// A case pattern outside the literal/`*` subset.
    CasePatternOutOfDialect,
    /// No case-arm pattern where one was required.
    ExpectedCaseArmPattern,
    /// A `shift` count that is not a literal integer.
    ShiftCountNotLiteralInteger,
    /// A statement that does not begin with a word.
    StatementDoesNotStartWithWord,
    /// An annotation with no value word after `=`.
    AnnotationNeedsValueWord,
    /// The lexer refused a token inside a command.
    OutOfDialectToken {
        /// What the lexer refused.
        lex: PredictLexError,
    },
    /// Any other unexpected token inside a command.
    UnexpectedTokenInCommand,
    /// A command with no words.
    EmptyCommand,
    /// A token where a word was required.
    ExpectedAWord,
    /// No `[` opening a test.
    ExpectedLbracketToOpenTest,
    /// A test operator outside the string-comparison subset.
    TestOperatorNotStringComparison,
    /// No `]` closing a test.
    ExpectedRbracketToCloseTest,
    /// A trailing `:=` bind mark.
    TrailingBindMarkWithValue,
    /// A mark with no verb or coordinate after its intro.
    MarkNeedsVerbOrCoordinate,
    /// A trailing `bind` mark.
    TrailingBindMarkWord,
    /// A mark with no payload after its verb.
    MarkNeedsPayload,
    /// A mark payload that does not split into a coordinate.
    MalformedMarkTarget,
    /// A selector outside the POSIX-name-in-spirit charset.
    SelectorNotPosixName,
}

/// What the check lexer refused — the inner clause of
/// [`PredictOutOfDialectReason::OutOfDialectToken`], and the check lexer's own error token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictLexError {
    /// A byte the dialect does not model.
    UnmodeledByte,
    /// A backtick command substitution.
    BacktickCommandSubstitution,
    /// A quote that never closed.
    UnterminatedQuote,
}

/// Payload of [`DiagCode::PredictUnterminated`]: a check function body is structurally
/// unterminated. No `SiteId` (`site()` returns `None`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredictUnterminated {
    /// Which block ran off the end of the input.
    pub reason: PredictUnterminatedReason,
}

/// Which block ran off the end of a check body (see [`CfgTopNodeReason`] for why the reason enums
/// live in this crate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictUnterminatedReason {
    /// The function body itself.
    FunctionBody,
    /// A keyword-closed block.
    Block {
        /// The keyword that would have closed it.
        keyword: &'static str,
    },
    /// A case arm.
    CaseArm,
    /// An `if`'s then-branch.
    IfThen,
}

/// Payload of [`DiagCode::OracleRoleFnUnlifted`] (TEMPLATIZED): the declared-but-unlifted
/// role-funcdef. Spanned at its own name, so the caret lands on the function the author wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleRoleFnUnlifted {
    /// The funcdef name as the file spells it (`{funcname}`, e.g. `wombat__is_converged`).
    pub funcname: String,
}

/// Payload of [`DiagCode::MarkOnAndOrList`]: none. Spanned at the refused mark, which is the whole
/// remediation — the author moves the marked command onto its own line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkOnAndOrList;

// ===========================================================================
// Sweep payload structs — oracle-lane (reserved / marker / entry / wrapper / carry / derive)
// ===========================================================================

/// Payload of [`DiagCode::MungeNameInvalid`] (TEMPLATIZED): the source name, its illegal munged
/// funcname, and the charclass problem. Spanned (the emitted-name span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MungeNameInvalid {
    /// The oracle source name (`{source}`).
    pub source: String,
    /// The munged sh funcname that is not a legal NAME (`{funcname}`).
    pub funcname: String,
    /// The charclass problem description (`{problem}` — `problem.describe()`).
    pub problem: String,
}

/// Payload of [`DiagCode::MungeNameCollision`] (TEMPLATIZED): one source, the shared funcname, the
/// collision count, and the colliding source names. Spanned; `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MungeNameCollision {
    /// One of the colliding source names (`{source}`).
    pub source: String,
    /// The shared munged funcname (`{funcname}`).
    pub funcname: String,
    /// The number of distinct source names sharing the funcname (`{count}`, twice in the template).
    pub count: usize,
    /// The colliding source names, comma-joined (`{names}`).
    pub names: String,
}

/// Payload of [`DiagCode::ReservedNamespaceSquat`] (TEMPLATIZED): the squatting book funcname and
/// the reserved role suffix. Spanned; `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservedNamespaceSquat {
    /// The book funcname squatting the namespace (`{name}`).
    pub name: String,
    /// The reserved role suffix it collides with (`{role}`, twice in the template).
    pub role: String,
}

/// Payload of [`DiagCode::OracleFileNotLoadInert`] (static): none. The SPAN is the whole
/// remediation — it lands on the FIRST top-level item that would run — and the shapes that trip it
/// (a command; an assignment whose value expands one) are one world-state, not two, so they share
/// one code (`AID-NEEDS:law-codes-vary-by-world-not-grammar`). Fires at most once per file: the
/// claim is about the file, so per-item mints would be a correlated cascade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OracleFileNotLoadInert;

/// Payload of [`DiagCode::RoleFamilyContested`] (TEMPLATIZED): the shadowed FAMILY, the role member
/// whose two definitions collided, and where the overridden one lives. Spanned at the shadowing
/// definition's name; `site()` returns `None` — a contest is about two DEFINITIONS, not about any
/// command site that later runs because of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleFamilyContested {
    /// The munged family base whose licenses are withheld (`{family}`).
    pub family: String,
    /// The role function both definitions bind (`{name}`).
    pub name: String,
    /// Where the OVERRIDDEN definition was authored, `file:line`-shaped (`{prior}`).
    pub prior: String,
}

/// Payload of [`DiagCode::RoleDefinedBelowItsSites`] (TEMPLATIZED): the move-it-up hint's
/// operands — the role function defined too late, and how many sites above it its family would
/// otherwise have answered. Spanned at the definition's name; `site()` returns `None`, because
/// the remediation is at the DEFINITION, not at any one site that lost its answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleDefinedBelowItsSites {
    /// The role function whose definition sits too low (`{name}`).
    pub name: String,
    /// How many command sites above it name this family (`{sites}`).
    pub sites: usize,
}

/// Payload of [`DiagCode::InBookVocabularyRole`] (TEMPLATIZED): the in-book kind-owner definition
/// the vocabulary tier refuses. Spanned at the definition's name; `site()` = `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InBookVocabularyRole {
    /// The kind-owner role function the book defined (`{name}`).
    pub name: String,
    /// Its role suffix — which member of the vocabulary tier it is (`{role}`).
    pub role: String,
}

/// Payload of [`DiagCode::MissingDialectMarker`] (static): the file-level marker refusal. The
/// marker text is inline in the template. Spanned (the first dialect construct); `site()` = `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingDialectMarker;

/// Payload of [`DiagCode::MarkerVersionUnrecognized`]: the unrecognized `# dorc-lang/vX.Y` version
/// tag read from the file, distinct from a wholly-missing marker. Spanned (the first dialect
/// construct); `site()` = `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerVersionUnrecognized {
    /// The unrecognized version marker text found (`{found}`).
    pub found: String,
}

/// Payload of [`DiagCode::ToleratesUnknownDimension`] (TEMPLATIZED): the unknown token and the
/// expected-dimension list. Spanned (the mark span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToleratesUnknownDimension {
    /// The unrecognized dimension token (`{token}`).
    pub token: String,
    /// The comma-joined list of known dimensions (`{expected}`).
    pub expected: String,
}

/// Payload of [`DiagCode::ToleratesOverIdentityDependence`] (static): the corroboration ask.
/// Spanned; `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToleratesOverIdentityDependence;

/// Payload of [`DiagCode::HeavyContextNoTolerance`] (static): the adoption hint. Spanned;
/// `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyContextNoTolerance;

/// Payload of [`DiagCode::LendMapUnknownDimension`] (TEMPLATIZED): the unknown token and the
/// expected-dimension list. Spanned (the mark span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LendMapUnknownDimension {
    /// The unrecognized dimension token (`{token}`).
    pub token: String,
    /// The comma-joined list of known dimensions (`{expected}`).
    pub expected: String,
}

/// Payload of [`DiagCode::CarryNetnsOnNetKernelForbidden`] (TEMPLATIZED): the kind whose
/// `net-kernel` store claimed `invariant:netns`. Spanned (the store `name_span`); `site()` = `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarryNetnsOnNetKernelForbidden {
    /// The munged kind name (`{kind_munged}`).
    pub kind_munged: String,
}

/// Payload of [`DiagCode::MarkBraceVerdictSingleCell`] (static): the single-cell brace refusal.
/// Spanned (the mark span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkBraceVerdictSingleCell;

/// Payload of [`DiagCode::MarkUnknownVerb`] (TEMPLATIZED): the unknown verb token and the known
/// verb vocabulary. Spanned (the token span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkUnknownVerb {
    /// The unrecognized period-free head/continuation token (`{token}`).
    pub token: String,
    /// The comma-joined list of known mark verbs (`{expected}`).
    pub expected: String,
}

/// Payload of [`DiagCode::MarkRcArityExceeded`] (static): a second rc-consumer on one block.
/// Spanned (the offending mark span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkRcArityExceeded;

/// Payload of [`DiagCode::MarkStandaloneRcConsumer`] (static): an rc-consumer/`reads` on a
/// standalone block. Spanned (the mark span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkStandaloneRcConsumer;

/// Payload of [`DiagCode::MarkHashcolonMalformed`] (static): a `#:` comment that looked like a
/// mark but did not parse. Spanned (the `#:` intro span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkHashcolonMalformed;

// ===========================================================================
// Sweep payload structs — plan/records.rs (the framed deframer's fault + integrity codes)
// ===========================================================================

/// Payload of [`DiagCode::RecordsHeaderlessRefused`] (static). Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsHeaderlessRefused;

/// Payload of [`DiagCode::RecordsGluedLine`] (static). Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsGluedLine;

/// Payload of [`DiagCode::RecordsHeaderMissing`] (static). Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsHeaderMissing;

/// Payload of [`DiagCode::RecordsSentinelNonce`] (static). Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsSentinelNonce;

/// Payload of [`DiagCode::RecordsFactTruncated`] (TEMPLATIZED): the received/declared/unseen site
/// counts. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsFactTruncated {
    /// Site records actually received (`{received}`).
    pub received: usize,
    /// Site records the header declared (`{declared}`).
    pub declared: usize,
    /// The unseen count that folds Unknown ⇒ run (`{unseen}`).
    pub unseen: usize,
}

/// Payload of [`DiagCode::RecordsIntegrityRefused`] (TEMPLATIZED): which integrity key mismatched.
/// Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsIntegrityRefused {
    /// The mismatched key's description (`{which}`).
    pub which: String,
}

/// Payload of [`DiagCode::RecordsTornLine`] (TEMPLATIZED): the discarded-line count. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsTornLine {
    /// The number of torn lines discarded (`{count}`).
    pub count: usize,
}

/// Payload of [`DiagCode::RecordsAlienLine`] (TEMPLATIZED): the discarded-line count. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsAlienLine {
    /// The number of alien lines discarded (`{count}`).
    pub count: usize,
}

/// Payload of [`DiagCode::RecordsLateLine`] (TEMPLATIZED): the discarded-line count. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordsLateLine {
    /// The number of late lines discarded (`{count}`).
    pub count: usize,
}

/// The closed controller-owned reason host evidence was refused before admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostEvidenceRefusalKind {
    IncompatibleVersion,
    StreamLimit,
    LineLimit,
    InvalidUtf8,
    ControlByte,
    Framing,
    Grammar,
    Numeric,
    RecordLimit,
    FieldLimit,
    RetainedLimit,
    CollectionLimit,
    Duplicate,
    ArithmeticOverflow,
}

/// Payload of [`DiagCode::HostEvidenceAdmissionRefused`]. Spanless and parameter-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostEvidenceAdmissionRefused {
    /// The closed admission category; no hostile host data crosses this boundary.
    pub kind: HostEvidenceRefusalKind,
}

// ===========================================================================
// Sweep payload structs — cli/main.rs (footprint / escalation / carry / resolver / wrapper)
// ===========================================================================

/// Payload of [`DiagCode::FootprintIncoherent`] (PASSTHROUGH `sm {detail}`): two emit sites (the
/// own-coordinate canary and the malformed-derived-coordinate refusal); BOTH now carry the escalated
/// book command's span (`aid-caret-span-precision`). `site()` returns `None` (no plan-`LeafId`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FootprintIncoherent {
    /// Which coherence check refused the footprint.
    pub reason: FootprintIncoherentReason,
}

/// Which coherence check refused a footprint (see [`CfgTopNodeReason`] for why the reason enums
/// live in this crate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FootprintIncoherentReason {
    /// The authored emission omits the site's own effect coordinate.
    OmitsOwnCoordinate,
    /// A derived emission carried a coordinate that does not parse.
    MalformedDerivedCoordinate,
}

/// Payload of [`DiagCode::TouchesEscalated`] (TEMPLATIZED): the escalated site number and the call.
/// Spanned (the escalated book command's span; `aid-caret-span-precision`); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TouchesEscalated {
    /// The escalated site's node id (`{site}`).
    pub site: u32,
    /// The escalated call text (`{call}`).
    pub call: String,
}

/// Payload of [`DiagCode::DerivFamilyIncomplete`] (TEMPLATIZED): the site number and the
/// incompleteness reason. Spanned (the escalated book command's span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivFamilyIncomplete {
    /// The site's node id (`{site}`).
    pub site: u32,
    /// The incompleteness reason (`{reason}` — the declared-vs-received or no-close-record match).
    pub reason: String,
}

/// Payload of [`DiagCode::EscalationPolicy`]: the authority-disclosure line. Spanless.
///
/// The dial IS the reason (`28L:rul-reason-enums-not-sibling-codes`), so this carries `core`'s own
/// [`EscalationDial`] rather than a parallel enum — the `TopCause` shape exactly. `capability` is
/// known at every dial and mentioned by two of the three sentences.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscalationPolicy {
    /// Which dial the admin set.
    pub dial: EscalationDial,
    /// The connection's entry capability.
    pub capability: Capability,
    /// The entry-capable wrappers loaded, `, `-joined (`{entry_forms}`).
    pub entry_forms: String,
}

/// Payload of [`DiagCode::CarriedAcrossSubstrateAxis`] (PASSTHROUGH `sm {detail}`): the carry
/// attribution chain. Spanned (the carried site's span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarriedAcrossSubstrateAxis {
    /// The crossed axes, `+`-joined (`{axes}`).
    pub axes: String,
    /// The read kinds and their invariant loci (`{kinds}`).
    pub kinds: String,
}

/// Payload of [`DiagCode::WrappedSiteAdoptionHint`] (PASSTHROUGH `sm {detail}`): the one-line
/// adoption hint. Spanned (the wrapped site's span); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedSiteAdoptionHint {
    /// The wrapper provider whose oracle could vouch (`{provider}`).
    pub provider: String,
    /// The dimension the vouch would name (`{dimension}`).
    pub dimension: String,
}

/// Payload of [`DiagCode::ResolverConflict`] (TEMPLATIZED): the kind and the resolver count.
/// Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverConflict {
    /// The kind name (`{kind}`).
    pub kind: String,
    /// The number of conflicting resolvers (`{count}`).
    pub count: usize,
}

/// Payload of [`DiagCode::ResolverProviderCollision`] (TEMPLATIZED): the colliding name. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverProviderCollision {
    /// The resolver name matching a known provider (`{name}`).
    pub name: String,
}

/// Payload of [`DiagCode::DanglingReference`] (TEMPLATIZED): the dangling coordinate. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DanglingReference {
    /// The rendered dangling coordinate (`{coord}`, display only).
    pub coord: String,
}

/// Payload of [`DiagCode::SharedCellMeasurementsDisagree`] (TEMPLATIZED): the shared cell whose
/// per-site measurements met to ⊤, and how many sites measured it. Spanless — a cell is a
/// cross-site coordinate with no one source point, and naming any single site as the location
/// would frame a shared collapse as one line's fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedCellMeasurementsDisagree {
    /// The rendered cell coordinate (`{cell}`, display only).
    pub cell: String,
    /// How many sites measured it (`{sites}`).
    pub sites: u32,
}

/// Payload of [`DiagCode::ReachesConflict`] (TEMPLATIZED): the kind and the reach-function count.
/// Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReachesConflict {
    /// The kind name (`{kind}`).
    pub kind: String,
    /// The number of conflicting reach-functions (`{count}`).
    pub count: usize,
}

/// Payload of [`DiagCode::ReachesProviderCollision`] (TEMPLATIZED): the colliding name. Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReachesProviderCollision {
    /// The reach-function name matching a known provider (`{name}`).
    pub name: String,
}

/// Payload of [`DiagCode::WrapperEntryIncoherent`] (PASSTHROUGH `sm {detail}`): the fold/entry
/// incoherence refusal. Spanned (the entry `name_span`); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrapperEntryIncoherent {
    /// The wrapper provider (`{wrapper}`).
    pub wrapper: String,
    /// How many leading args the entry form consumes (`{entry_shifts}`).
    pub entry_shifts: String,
    /// How many the lend-fold consumes (`{lend_shifts}`).
    pub lend_shifts: String,
}

/// Payload of [`DiagCode::WrapperPeelIncoherent`] (PASSTHROUGH `sm {detail}`): the peel-tail
/// incoherence refusal. Spanned (the predict `name_span`); `site()` returns `None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrapperPeelIncoherent {
    /// The wrapper provider (`{wrapper}`).
    pub wrapper: String,
    /// How many argv tokens `__predict` consumes before `"$@"` (`{predict_depth}`).
    pub predict_depth: String,
    /// How many `__lend_map` consumes (`{lend_map_depth}`).
    pub lend_map_depth: String,
}

/// Payload of [`DiagCode::WhylogVersionRefused`] (`27V` Lane B): the durable's format-version tag
/// this binary could not parse. `{found}` = the tag read from the file's header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhylogVersionRefused {
    /// The `dorc-whylog/N` tag found in the durable's header (`{found}`).
    pub found: String,
}

/// Payload of [`DiagCode::WhylogBookDesync`] (`27V` Lane B; the `22F` book-identity/desync guard):
/// which recorded digest diverged from the current on-disk inputs. `{which}` = `book` / an oracle
/// path / `decision-digest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhylogBookDesync {
    /// The diverged input's description (`{which}`).
    pub which: String,
}

/// Payload of [`DiagCode::WhylogAbsent`] (`27V` Lane B): `dorc why --last` found no durable to
/// replay. `{dir}` = the whylog directory searched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhylogAbsent {
    /// The whylog directory searched (`{dir}`).
    pub dir: String,
}

/// Payload of [`DiagCode::WhylogCorrupt`] (`27V` Lane B; `inv-no-throw`): a durable was found but is
/// truncated / unparseable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhylogCorrupt {
    /// Which parse check refused the durable.
    pub reason: WhylogCorruptReason,
}

/// Which parse check refused a durable (see [`CfgTopNodeReason`] for why the reason enums live in
/// this crate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhylogCorruptReason {
    /// The file is empty, or its first line is not a header.
    Headerless,
    /// The header line does not carry the format tag.
    HeaderTagMissing,
    /// The records block declares more bytes than the file holds.
    ResultsBlockOverruns,
    /// The end sentinel never arrived.
    EndSentinelMissing,
}

/// Payload of [`DiagCode::WhylogUnwritten`] (`28D:must-default-durable-lands-with-its-hardening`,
/// the visible-persistence-failure item): the run finished but its durable did not land. `{dir}` =
/// the whylog directory; `{reason}` = the closed refusal word (`directory` / `names-exhausted` /
/// `oversize` / `write` from the store, or `limit` / `grammar` / `numeric` / `digest` / `overflow`
/// from the serializer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhylogUnwritten {
    /// The whylog directory the durable was to land in (`{dir}`).
    pub dir: String,
    /// The closed refusal word (`{reason}`).
    pub reason: String,
}

/// Payload of [`DiagCode::AidUnloadedSiblingOracle`] (PASSTHROUGH `{detail}`; `AID-NEEDS:aid-unloaded-
/// sibling-oracle`, gap-5 / `24H` ack-6): the cli-edge scan builds `detail` listing the sibling
/// `*.oracle.sh` files found on disk but not loaded (suggest, never auto-load). Spanless.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AidUnloadedSiblingOracle {
    /// The unloaded sibling files, backticked and comma-joined (`{oracles}`).
    pub oracles: String,
}

/// Payload of [`DiagCode::UnmodeledWallInventory`] (`288` §5): the whole-book wall census the
/// `unmodeled-inventory` lint source takes. Counts, never identities — `inv-referent-agnostic`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnmodeledWallInventory {
    /// How many unmodeled ⊤-walls the book carries (`{wall_count}`).
    pub wall_count: usize,
    /// The count-agreeing noun for `wall_count` (`{wall_word}`) — pluralization is an engine-owned
    /// canonical formatter, never something the prose register hand-writes.
    pub wall_word: &'static str,
    /// How many leaf sites sit downstream of the FIRST wall (`{downstream}`).
    pub downstream: usize,
}

/// Payload of [`DiagCode::VerdictTerminalPipeline`] (`288` §5): a verdict body whose last
/// status-bearing statement is a pipeline. Carries no operands — the span says which body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdictTerminalPipeline;

/// Payload of [`DiagCode::AuthoredDeclineClass`] (`288` §5): one per-arm decline inventory entry
/// whose `<verb> <class>` header WAS statically readable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredDeclineClass {
    /// The statically-read class token (`{class}`).
    pub class: String,
}

/// Payload of [`DiagCode::AuthoredDeclineClassUnreadable`] (`288` §5): the SIBLING world-state —
/// a dynamic format or an unrecognized class token (`27W:rul-report-noise-tolerant`) leaves the
/// class unread until runtime. A sibling code, not a `{class}`-hole variant, because the two
/// differ in world-state and remediation, never in grammar
/// (`AID-NEEDS:law-codes-vary-by-world-not-grammar`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredDeclineClassUnreadable;

/// Payload of [`DiagCode::LintToolAbsent`] (`288` §5): a configured external linter missing from
/// PATH. `dir-absent-is-info` — advisory unless `--require-tools` raises it operationally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintToolAbsent {
    /// The tool's name (`{tool}`).
    pub tool: String,
}

/// Payload of [`DiagCode::LintToolOutputUnparsable`] (`288` §5): PASSTHROUGH `{output}` — the
/// tolerant adapters fell through every tier, so the tool's own bytes ride the payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintToolOutputUnparsable {
    /// The tool's name (`{tool}`).
    pub tool: String,
    /// The tool's own bytes (`{output}`; display only), sealed as not-ours at the capture edge.
    pub output: ForeignBytes,
}

/// Payload of [`DiagCode::LintToolFailedWithoutFindings`] (`288` §5): the exit-trichotomy's third
/// arm — nonzero rc, nothing parseable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintToolFailedWithoutFindings {
    /// The tool's name (`{tool}`).
    pub tool: String,
    /// The tool's exit status (`{rc}`).
    pub rc: i32,
}

// ===========================================================================
// Invocation errors (`288` §6): the `dorc: {msg}` family + `dorc-sh`. All SPANLESS (an argv has no
// span) and never `Structural` (an invocation error is always the user's to fix). The cut follows
// `AID-NEEDS:law-codes-vary-by-world-not-grammar` — grammar-fit takes a hole, world-state takes a
// sibling. The usage synopsis is print-seat chrome, not a register (`291` §5d).
// ===========================================================================

/// Payload of [`DiagCode::CliStripNeedsPath`]: `dorc strip` with no positional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliStripNeedsPath;

/// Payload of [`DiagCode::CliStripGotAFlag`]: the sibling world-state — a positional WAS given and
/// it is a flag, so the user meant something the surface does not offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliStripGotAFlag {
    /// The flag found where the path belongs (`{got}`).
    pub got: String,
}

/// Payload of [`DiagCode::CliUnknownMode`]: a leading token that is a near-miss for a mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliUnknownMode {
    /// The token given (`{mode}`).
    pub mode: String,
    /// The nearest real mode (`{suggestion}`).
    pub suggestion: String,
}

/// Payload of [`DiagCode::CliFlagNeedsValue`]: ONE code for every value-taking flag — the flags
/// differ, the failure does not (grammar-fit ⇒ a `{flag}` hole, never a sibling per flag).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFlagNeedsValue {
    /// The flag given without its value (`{flag}`).
    pub flag: String,
    /// What that flag wants, in the author's own words (`{wants}`) — "a path", "a directory", …
    pub wants: &'static str,
}

/// Payload of [`DiagCode::CliUnknownFlag`]: an unrecognized flag with no near neighbour.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliUnknownFlag {
    /// The unrecognized flag (`{flag}`).
    pub flag: String,
}

/// Payload of [`DiagCode::CliUnknownFlagDidYouMean`]: the SIBLING world-state — the flag table has
/// a near neighbour, so the remediation is "you meant this one", not "read the usage".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliUnknownFlagDidYouMean {
    /// The unrecognized flag (`{flag}`).
    pub flag: String,
    /// The nearest real flag (`{suggestion}`).
    pub suggestion: String,
}

/// Payload of [`DiagCode::CliFlagValueNotRecognized`]: a value outside a flag's closed vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFlagValueNotRecognized {
    /// The flag (`{flag}`).
    pub flag: String,
    /// The value given (`{got}`).
    pub got: String,
    /// The accepted vocabulary, `|`-joined (`{expected}`).
    pub expected: &'static str,
}

/// Payload of [`DiagCode::CliFlagValueNotANumber`]: a numeric flag given a non-number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFlagValueNotANumber {
    /// The flag (`{flag}`).
    pub flag: String,
    /// The value given (`{got}`).
    pub got: String,
}

/// Payload of [`DiagCode::CliNoBookGiven`]: an analysis invocation with nothing to analyze.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliNoBookGiven;

/// Payload of [`DiagCode::CliFlagsMutuallyExclusive`]: two flags that cannot both hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFlagsMutuallyExclusive {
    /// The first flag (`{first}`).
    pub first: &'static str,
    /// The second flag (`{second}`).
    pub second: &'static str,
}

/// Payload of [`DiagCode::CliFlagRequiresMode`]: a flag scoped to one mode, given under another.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFlagRequiresMode {
    /// The flag (`{flag}`).
    pub flag: &'static str,
    /// The invocation the flag belongs to (`{mode}`).
    pub mode: &'static str,
}

/// Payload of [`DiagCode::CliFileNotFound`]: the `NotFound` arm of the read-error trichotomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFileNotFound {
    /// What we were reading, in the caller's words (`{kind}`) — "source", "book", …
    pub kind: String,
    /// The path as the user gave it (`{path}`).
    pub path: String,
}

/// Payload of [`DiagCode::CliFilePermissionDenied`]: the `PermissionDenied` arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFilePermissionDenied {
    /// What we were reading (`{kind}`).
    pub kind: String,
    /// The path (`{path}`).
    pub path: String,
}

/// Payload of [`DiagCode::CliFileUnreadable`]: the RESIDUAL arm — PASSTHROUGH `{detail}` carries
/// the platform's own words, because there is nothing better to say about an unclassed OS error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliFileUnreadable {
    /// What we were reading (`{kind}`).
    pub kind: String,
    /// The path (`{path}`).
    pub path: String,
    /// The platform's own words (`{detail}`), sealed as not-ours at the edge that read them.
    pub detail: ForeignBytes,
}

/// Payload of [`DiagCode::LintNoLintableFiles`]: zero lintable files is OPERATIONAL, never clean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintNoLintableFiles;

/// Payload of [`DiagCode::LintFileCountDrift`]: the `--expect-files` CI assertion failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintFileCountDrift {
    /// What `--expect-files` declared (`{expected}`).
    pub expected: usize,
    /// What the invocation actually found (`{found}`).
    pub found: usize,
}

/// Payload of [`DiagCode::LintRequiredToolsMissing`]: `--require-tools` with an absent tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintRequiredToolsMissing {
    /// The absent tools, comma-joined (`{tools}`).
    pub tools: String,
}

/// Payload of [`DiagCode::DorcShUsage`]: `dorc-sh` with no script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DorcShUsage;

/// Payload of [`DiagCode::DorcShScriptUnreadable`]: `dorc-sh` could not read its script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DorcShScriptUnreadable {
    /// The script path (`{path}`).
    pub path: String,
    /// The platform's own words (`{detail}`), sealed as not-ours at the edge that read them.
    pub detail: ForeignBytes,
}

/// Payload of [`DiagCode::DorcShExecFailed`]: the exec of stock sh itself failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DorcShExecFailed {
    /// The platform's own words (`{detail}`), sealed as not-ours at the edge that read them.
    pub detail: ForeignBytes,
}

/// Payload of [`DiagCode::CliShimDirUnwritable`]: the `--shim-dir` materialization edge failed.
/// NOT in `291` §5a's mapped inventory — the one-error-type-through-`run` extraction surfaced it
/// as the last raw string on the surface, and `one-catalog-no-legacy` leaves it nowhere to hide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliShimDirUnwritable {
    /// The path we could not create or write (`{path}`).
    pub path: String,
    /// The platform's own words (`{detail}`), sealed as not-ours at the edge that read them.
    pub detail: ForeignBytes,
}

/// Payload of [`DiagCode::TransportCrlfRefused`]: bytes bound for a host are not LF-only.
///
/// A CRLF shebang is an exec failure the remote kernel reports before any shell of ours exists,
/// so no guard, oracle or diagnostic can catch it there (`plans/139` §5) — which is why this
/// refuses on the controller instead, before anything is shipped. It refuses rather than
/// repairs: silently rewriting bytes someone is about to run on a server would trade a loud
/// one-line fix for an invisible edit (`260` dec-26-crlf).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportCrlfRefused {
    /// Which artifact carried it (`{which}`).
    pub which: String,
    /// The 1-based line the first carriage return sits on (`{line}`).
    pub line: String,
}

/// Payload of [`DiagCode::TransportSessionLost`]: a session produced no completion marker.
///
/// The world's state is UNKNOWN, which is neither "clean" nor "failed"
/// (`rul-integrity-failure-withholds-mutation`). It is deliberately NOT a sibling of
/// [`TransportSpawnRefused`] by grammar but by WORLD STATE
/// (`AID-NEEDS:law-codes-vary-by-world-not-grammar`): there, nothing ran and the remedy is to fix
/// the invocation; here, something may have run and the remedy is to re-probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportSessionLost {
    /// The destination (`{host}`).
    pub host: String,
    /// How many attempts were made (`{attempts}`).
    pub attempts: String,
    /// A best-effort reading of what severed it — decision-inert (`{diagnosis}`).
    pub diagnosis: String,
}

/// Payload of [`DiagCode::TransportSpawnRefused`]: the platform refused to create the session
/// process, so nothing ran anywhere.
///
/// One of two codes licensed to say the host was untouched, because the failure is local. Its
/// sibling is [`TransportMarkerUnusable`]; the two were one code with a MIXED `detail` until
/// `296:tc-transport-not-attempted-is-two-worlds` — here the platform speaks and the remedy is
/// environmental, there we speak and the remedy is the invocation
/// (`AID-NEEDS:law-codes-vary-by-world-not-grammar`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportSpawnRefused {
    /// The host that went uncontacted (`{host}`).
    pub host: String,
    /// The platform's own words about the refused spawn (`{detail}`), sealed as not-ours.
    pub detail: ForeignBytes,
}

/// Payload of [`DiagCode::TransportMarkerUnusable`]: the run's nonce could not become a session
/// marker, so no artifact was shipped and the host was never contacted.
///
/// The sibling of [`TransportSpawnRefused`]: same untouched-host claim, a different world. Nothing
/// outside the controller participated, so there is nothing to relay — the whole sentence is ours
/// and lives in the code's register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportMarkerUnusable {
    /// The host that went uncontacted (`{host}`).
    pub host: String,
}

/// Payload of [`DiagCode::TransportApplyFailed`]: a remote apply completed with a non-zero status.
///
/// The third world state in this family, and the only one that is KNOWN: the artifact ran, it
/// finished, and it exited non-zero. The status is reproduced, never interpreted — Dorc measures
/// a tool's status and passes it through, it does not decide what one means
/// (`law-lane-discipline`). Note that a zero here is equally not a health claim: a plan exiting 0
/// does not prove the services it touched are well (`plans/252` §8), which is what a verify
/// re-probe is for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportApplyFailed {
    /// The destination (`{host}`).
    pub host: String,
    /// The status the artifact exited with (`{status}`).
    pub status: String,
}

// ===========================================================================
// First-class site identity (type-sketch-5) — the slot, not the fleet machinery
// ===========================================================================

pub use dorc_core::SiteId;

/// The hierarchical grouping keys for ⊤-cascade dedup and fleet aggregation (`22B`
/// `type-sketch-5`; `228` dc-3: `CodeChecker` context-free-v2 + Sentry match-either-hash). The
/// FINE key distinguishes per-host detail (the engineer debugging one host); the COARSE key
/// collapses M manifestations of one cause for the admin ("one rot, 12 hosts"). Both served, per
/// the AGENTS two-user exclusion-check.
///
/// This is a TRAIT SLOT, not a built subsystem (`22A` arch-2: "design the slot, don't build the
/// fleet machinery"): the fleet rollup that CONSUMES coarse keys is out of scope this round.
/// `22B-fork-scope-key` is STUBBED — [`coarse_key`](GroupingKey::coarse_key) degenerates to the
/// fine key, because a real coarse key needs an enclosing-structural-scope id the spike does not
/// yet surface as first-class. A degenerate coarse=fine is honest for now (it just means no
/// cross-site collapse happens yet); the trait shape is the deliverable.
pub trait GroupingKey {
    /// The fine key — distinguishes per-site detail. Today: `(code-slug, site)`.
    fn fine_key(&self) -> FineKey;
    /// The coarse key — collapses manifestations of one cause for fleet rollup. STUBBED to the
    /// fine key this round (`22B-fork-scope-key`): an honest degenerate that simply does no
    /// collapsing until enclosing-scope ids are surfaced.
    fn coarse_key(&self) -> CoarseKey;
}

/// The fine grouping key (`228` dc-3): a code slug paired with the site. Distinguishes the
/// per-host detail an engineer debugging one host wants. Ordered/`Hash` so a render can group by
/// it deterministically (`inv-determinism`).
///
/// `site` is `Option<SiteId>` because codes emitted before CFG construction (the `syntax-*`
/// codes) or mid-resolution without a plan leaf (oracle lifter codes, `effect-kind-disagreement`)
/// carry no natural site. For those codes the fine key degenerates to a code-slug-only key
/// (`site == None`), which is still correct for tidy/coverage purposes — it just does not
/// collapse per-site (which is fine since those codes have no per-site identity to collapse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FineKey {
    /// The diagnostic code's stable slug.
    pub code: &'static str,
    /// The originating site, if available (see type-level comment).
    pub site: Option<SiteId>,
}

/// The coarse grouping key (`228` dc-3): for fleet rollup, drops the call-site so M
/// manifestations of one cause collapse. STUBBED this round (`22B-fork-scope-key`): it wraps the
/// fine key unchanged (degenerate coarse=fine), so no collapse happens yet. When
/// enclosing-structural-scope ids are surfaced, this gains an `enclosing-scope` field and drops
/// `site.leaf`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoarseKey {
    /// The fine key, verbatim (the degenerate stub — `22B-fork-scope-key`).
    pub fine: FineKey,
}

impl GroupingKey for Diag {
    fn fine_key(&self) -> FineKey {
        FineKey {
            code: self.code.slug(),
            site: self.code.site(),
        }
    }
    fn coarse_key(&self) -> CoarseKey {
        CoarseKey {
            fine: self.fine_key(),
        }
    }
}

impl DiagCode {
    /// The originating [`SiteId`] this code's payload cites, if one is available.
    ///
    /// Codes that carry a natural `SiteId` (plan-`LeafId` or CFG-node-id-space standin) return
    /// `Some`; codes that emit before CFG construction (the `syntax-*` codes), mid-effect-
    /// resolution with no plan leaf (`effect-kind-disagreement`), or from the oracle lifter
    /// with no plan context return `None`. The grouping keys degenerate to a site-less form in
    /// that case (`FineKey::site` is `Option<SiteId>`).
    #[must_use]
    fn site(&self) -> Option<SiteId> {
        match self {
            DiagCode::CmdsubOperandTop(p) => Some(p.site),
            DiagCode::SiteUnresolvable(p) => Some(p.site),
            DiagCode::RenderHeredocRefused(p) => Some(p.site),
            DiagCode::CmdsubInnerNonleaf(p) => Some(p.site),
            DiagCode::RedirTargetTop(p) => Some(p.site),
            DiagCode::Depth2PositionalUnthreaded(p) => Some(p.site),
            // Every other code carries no plan-`LeafId` SiteId: pre-CFG, a source-span-only
            // oracle/cli site, or a spanless whole-stream/whole-file verdict.
            _ => None,
        }
    }
}

// ===========================================================================
// The Diag value (type-sketch-2): message + labeled spans + children + suggestion
// ===========================================================================

/// One diagnostic, ready to render three ways or ride the OOB lane (`22B` `type-sketch-2`). The
/// structured shape cribbed from rustc (`crib-1`): a typed [`DiagCode`] payload, a MANDATORY
/// primary [`SpanLabel`] (the region the render points at), a window of optional secondary
/// labels (a ⊤-cause-site and a poisoned-site live in ONE diagnostic — `228`), and ordered
/// [`SubDiag`] children. Severity is NOT a field — it is looked up from the [`registry`] by
/// `code` (`crib-4`), so it cannot drift per-site.
///
/// `inv-no-throw`: a `Diag` is data; constructing it never panics. The mandatory primary span
/// is the structural fix for `21Z` drop-A/drop-B together — there is no span-less `Diag`, so the
/// CLI render cannot drop what was never optional and an author cannot forget it.
#[derive(Debug, Clone)]
pub struct Diag {
    /// The catalog code, carrying its typed payload (`type-sketch-1`).
    pub code: DiagCode,
    /// The one span the region renders around — MANDATORY (drop-A/drop-B impossible).
    pub primary: SpanLabel,
    /// Additional labeled spans (the cause-site, the poisoned-site — `228`).
    pub secondary: Vec<SpanLabel>,
    /// Ordered notes/helps (`crib-1`/`crib-3`): facts then remediation.
    pub children: Vec<SubDiag>,
    /// The actionable fix, if any (`crib-2`).
    pub suggestion: Option<Suggestion>,
}

/// A span with an optional label — the rustc primary/secondary-label model (`crib-1`). Fixes
/// `21Z` drop-B: a span is no longer `Option`-on-the-whole-`Diag`; the PRIMARY span is mandatory,
/// secondaries are the optional extras.
///
/// The span slot is a [`SpanSite`], not a bare [`Span`], to carry the second-class spanless case
/// ([`SpanSite::Spanless`], arch-3-residual-2) WITHOUT making the field an `Option` everyone can
/// reach. The field is PRIVATE: it is constructed only inside this module (the two `Diag`
/// constructors and [`Diag::secondary`]), and read through [`span`](Self::span), so external code
/// cannot mint a spanless label — span-lessness is reachable solely via [`Diag::new_spanless_site`].
#[derive(Debug, Clone)]
pub struct SpanLabel {
    /// The source span slot (mandatory-real on every PRIMARY minted by [`Diag::new`] and on every
    /// secondary; [`SpanSite::Spanless`] only on a [`Diag::new_spanless_site`] primary). Private so
    /// the spanless variant is unconstructable outside this module — read it via [`span`](Self::span).
    span: SpanSite,
    /// The caret-label prose ("this went ⊤", "first poisoned here"), if any.
    pub label: Option<String>,
}

impl SpanLabel {
    /// This label's source span, or `None` when the label is the second-class spanless case
    /// ([`SpanSite::Spanless`], arch-3-residual-2). Ordinary readers see `Option<Span>` (the same
    /// shape the legacy `Diagnostic::span` carried); they still cannot CONSTRUCT a spanless
    /// label (the field is private).
    #[must_use]
    pub fn span(&self) -> Option<Span> {
        match self.span {
            SpanSite::At(s) => Some(s),
            SpanSite::Spanless => None,
        }
    }
}

/// The span slot of a [`SpanLabel`] — almost always [`SpanSite::At`] (a real source span).
/// [`SpanSite::Spanless`] is the deliberately SECOND-CLASS sentinel for the arch-3-residual-2 codes
/// whose emit context genuinely has no source location. The exact set is enforced by the
/// `SPANLESS_SITE_PAYLOADS` allow-list in `core/tests/diag_tidy.rs` (the gate, not this comment, is
/// the source of truth): the six codes `cfg-errexit-unknown`, `effect-kind-disagreement`, and the
/// four `oracle-*` whole-file verdicts. It lives INSIDE the span slot (not as an `Option` on the
/// whole primary) precisely so the mandatory-primary-span guarantee (`21Z` drop-B) stays intact for
/// every ordinary `Diag`: [`Diag::new`] takes a real [`Span`] and cannot produce `Spanless`; only
/// [`Diag::new_spanless_site`] can. Private — `Spanless` is unnameable and unconstructable outside
/// this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpanSite {
    /// A real source span (every primary from [`Diag::new`], every secondary).
    At(Span),
    /// No source span — the second-class arch-3-residual-2 case (see type docs).
    Spanless,
}

/// A note or help child (`crib-1`/`crib-3`). `Help` is remediation-facing (CLI-only,
/// fact-plane-exempt); `Note` is additional fact context. The split lets the render model drop
/// helps from artifact-eligible output while keeping notes (`22B` `type-sketch-2`).
#[derive(Debug, Clone)]
pub enum SubDiag {
    /// Additional fact context (the primary message states the fact; a Note adds to it).
    Note(String),
    /// Remediation guidance (`crib-3`: "only the help should suggest how to fix"). CLI-only.
    Help(String),
}

// ===========================================================================
// Suggestion + Applicability + RemediationClass (type-sketch-3)
// ===========================================================================

/// An actionable fix (`22B` `type-sketch-3`). Cribbed from rustc (`crib-2`): a message + a
/// confidence the tooling reads to decide auto-apply. Dorc adds the human-ratified
/// [`RemediationClass`] (ru-6): the [`Applicability`] says HOW confident, the class says WHAT
/// kind of fix. Together they drive a render's grouping and a future `dorc fix` story.
///
/// No machine-applicable SPAN-EDIT to a shipped `.sh` artifact this round: a suggestion is
/// admin-facing guidance (CLI), not an artifact rewrite — the artifact stays fact-plane (ru-12).
///
/// SEAM (gap-4, RE-PARKED d4b): `Suggestion` has NO production emitter yet — the type stands, the
/// wiring waits. The natural FIRST emitter is `missing-dialect-marker` (an honest
/// `Applicability::MachineApplicable` insert of `# dorc-lang/v0.2` into an oracle's first 10 lines),
/// which unparks WITH the `dorc fix` apply-story (`27S` §4 fix-modes, deferred) — its artifact-vs-
/// authoring-plane auto-apply boundary needs a human ruling before the first real `MachineApplicable`
/// lands. Until then the code's `message` already states the fix in prose.
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// The remediation prose ("declare nginx's `installed` selector in the oracle").
    pub message: String,
    /// How confident the fix is correct (`crib-2`, verbatim from rustc).
    pub applicability: Applicability,
    /// Which user action clears the origin (ru-6 — the render/grouping axis).
    pub remediation: RemediationClass,
}

/// rustc's confidence model, verbatim (`crib-2`, re-verified live against `rustc_lint_defs`). The
/// discipline that matters: a tool decides whether to auto-apply from the applicability, not from
/// the prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    /// Auto-apply; preserves meaning.
    MachineApplicable,
    /// Valid code but uncertain — consult the user.
    MaybeIncorrect,
    /// Contains `(…)`-style holes; cannot auto-apply.
    HasPlaceholders,
    /// Confidence unspecified.
    Unspecified,
}

/// The human-ratified render axis (ru-6, `224` §7), re-cut HOW-not-WHO per ru-27: classify every
/// remediable origin by the KIND OF FIX that clears it — not who does it — and rank/group the render
/// by that. The old who-decomposition (author-oracle / fix-book-line) collapsed the dev-vs-admin
/// distinction into the ACTION type, which is what the render actually groups on; the two-user
/// distinction (AGENTS) now rides the fix's phrasing at render time, not the class.
///
/// [`Structural`](Self::Structural) stays the honest "no user action clears this; it's a Dorc
/// limitation" bucket — load-bearing for not lying that a ⊤ that is really ours is fixable
/// (`271:rul-sin-ordering`: mis-attribution is the worst aid failure).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationClass {
    /// Resolve a dynamic/runtime value so Dorc can read it (make the operand a literal, drop the
    /// `$(…)`): the give-up is a value Dorc could not resolve, not a missing model.
    ResolveDynamism,
    /// Declare the missing identity: a kind/selector/coordinate/vouch/marker the analyzer needs to
    /// bind or license (an oracle- OR book-side declaration).
    DeclareIdentity,
    /// Provide or extend a model: author/fix an oracle (a read-only probe, a coherent wrapper, a
    /// well-formed check body) so the tool is no longer unmodeled.
    ProvideModel,
    /// No user action clears it — a Dorc-modeling limitation (an honest "it's ours", never a false
    /// "you can fix this").
    Structural,
}

// ===========================================================================
// The registry: severity-as-data with a floor tier (type-sketch-4)
// ===========================================================================

/// Per-code severity (`crib-4`) and un-overridable floor (`crib-5`). The ONLY place severity is
/// decided — never at a construction site (the new API has no severity constructor). A single
/// `match` keyed on the code's discriminant; adding a code adds one arm (the friction test).
///
/// The floor column is PROPOSED (`22B-fork-floor-membership` / `22A` gate2-ask-1): the human
/// disposes which codes pin to the floor at the PR. My per-code judgment, clearly marked:
/// * [`DiagCode::RenderHeredocRefused`] ⇒ Error + [`Floor::WarnOrDeny`] — a kFAIL-correctness
///   give-up: silencing it below a warning would hide a converged mutator running because the
///   render could not safely elide it (a broken-artifact-adjacent class).
/// * [`DiagCode::SiteUnresolvable`] / [`DiagCode::CmdsubOperandTop`] ⇒ Note + [`Floor::None`] —
///   pure disclosures (the apply runs the site either way; the floor would over-constrain a
///   benign "ran on every apply" note).
#[must_use]
#[expect(
    clippy::match_same_arms,
    clippy::too_many_lines,
    reason = "the catalog's friction test is one ROW PER CODE (22B §7) — adding a code adds one \
              arm; merging arms by `|` would hide that a code has a declared row and break the \
              per-code-grading shape, so each of the 47 codes keeps its own arm even when several \
              share a CodeSpec value (which also makes the fn necessarily long)"
)]
pub fn registry(code: &DiagCode) -> CodeSpec {
    match code {
        // ── round-22 §5 worked examples ─────────────────────────────────────
        DiagCode::CmdsubOperandTop(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ResolveDynamism,
        },
        DiagCode::SiteUnresolvable(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::RenderHeredocRefused(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor (22B-fork-floor-membership): a render-refusal that would otherwise
            // ship a broken artifact must never be silenced below a warning.
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::Structural,
        },
        // ── B4 sweep: former diag::legacy survivors ──────────────────────────
        // Pure disclosures (the apply runs these sites regardless) → Note + Floor::None.
        DiagCode::CmdsubInnerNonleaf(_) => CodeSpec {
            severity: Severity::Note,
            // PROPOSED floor: pure disclosure, no correctness floor needed.
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RedirTargetTop(_) => CodeSpec {
            severity: Severity::Note,
            // PROPOSED floor: pure disclosure.
            floor: Floor::None,
            remediation: RemediationClass::ResolveDynamism,
        },
        DiagCode::Depth2PositionalUnthreaded(_) => CodeSpec {
            severity: Severity::Note,
            // PROPOSED floor: pure disclosure of a depth-2 limitation.
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        // ── B4 sweep: syntax/parser.rs ───────────────────────────────────────
        // Syntax errors are Error-class correctness give-ups → WarnOrDeny floor.
        DiagCode::SyntaxUnsupported(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor: an unmodeled construct causes ⊤; silencing it would hide a
            // correctness give-up.
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::Structural,
        },
        DiagCode::SyntaxMalformed(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor: a parse error is a hard correctness boundary.
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::Structural,
        },
        // ── B4 sweep: analysis/cfg.rs ────────────────────────────────────────
        DiagCode::CfgTopNode(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor: a ⊤-reject is a correctness give-up.
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::Structural,
        },
        DiagCode::CfgErexitUnknown(_) => CodeSpec {
            severity: Severity::Warning,
            // PROPOSED floor: a conservative assumption, but silencing could mask a missed edge.
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::CfgInlineRefused(_) => CodeSpec {
            severity: Severity::Warning,
            // PROPOSED floor: a capability disclosure; the call runs as unmodeled (MustRun, safe).
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::CfgBuiltinShadowed(_) => CodeSpec {
            severity: Severity::Warning,
            // PROPOSED floor: a disclosure of an assumption that may be unsound.
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        // ── B4 sweep: analysis/effect.rs ─────────────────────────────────────
        DiagCode::EffectKindDisagreement(_) => CodeSpec {
            severity: Severity::Warning,
            // PROPOSED floor: the annotation wins; the warning is informational.
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        // ── B4 sweep: oracle/predict/parser.rs ─────────────────────────────────
        DiagCode::PredictOutOfDialect(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor: an out-of-dialect check cannot be lifted — correctness gap.
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::PredictUnterminated(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor: an unterminated check body cannot be lifted — correctness gap.
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        // The cause-agnostic backstop: WARNING, not Error. It fires where the engine cannot say
        // what went wrong, so refusing on it would refuse on our own ignorance; and the loss is
        // always safety-correct (an unlifted funcdef vouches for nothing, it only stops helping).
        DiagCode::OracleRoleFnUnlifted(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::MarkOnAndOrList(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        // ── sweep: severities preserve each emit site's CURRENT classification exactly.
        // Floor rule (as elsewhere): Error ⇒ WarnOrDeny (a refusal must not silence below Warning);
        // Warning/Note disclosures ⇒ None.
        DiagCode::MungeNameInvalid(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::MungeNameCollision(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::ReservedNamespaceSquat(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        // Refuses rather than degrades: a partial load is a WRONG environment, not a narrow one
        // (`inv-top-reject`; `oracle/CLAUDE.md declarations-only-files`).
        DiagCode::OracleFileNotLoadInert(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        // WARNING, not error: the refusal only WITHHOLDS, and failing the run would punish an
        // admin for a collision two upstream authors caused.
        DiagCode::RoleFamilyContested(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        // NOTE: nothing is wrong — the book is correct sh and applies unchanged; the aid plane is
        // naming value the admin could recover by moving one line.
        DiagCode::RoleDefinedBelowItsSites(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        // WARNING: the definition genuinely does not load, so silence would leave the author
        // wondering why their kind-owner body never answers.
        DiagCode::InBookVocabularyRole(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::MissingDialectMarker(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::MarkerVersionUnrecognized(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::ToleratesUnknownDimension(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::ToleratesOverIdentityDependence(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::HeavyContextNoTolerance(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::LendMapUnknownDimension(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::CarryNetnsOnNetKernelForbidden(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::MarkBraceVerdictSingleCell(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        // `281` marks ⇒ ⊤ (`inv-top-reject`): Error+WarnOrDeny, save `mark-hashcolon-malformed`
        // (Warning, `281` §9). Remediation DeclareIdentity tracks `mark-brace-verdict-single-cell`.
        DiagCode::MarkUnknownVerb(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::MarkRcArityExceeded(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::MarkStandaloneRcConsumer(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::MarkHashcolonMalformed(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::RecordsHeaderlessRefused(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsGluedLine(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsHeaderMissing(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsSentinelNonce(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsFactTruncated(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsIntegrityRefused(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsTornLine(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsAlienLine(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::RecordsLateLine(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::HostEvidenceAdmissionRefused(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::Pinned,
            remediation: RemediationClass::Structural,
        },
        DiagCode::FootprintIncoherent(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::TouchesEscalated(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::DerivFamilyIncomplete(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::EscalationPolicy(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::CarriedAcrossSubstrateAxis(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::WrappedSiteAdoptionHint(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::ResolverConflict(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::ResolverProviderCollision(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::DanglingReference(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        // The shared-cell meet: a Note on the advisory tier. The sites RUN (the safe direction),
        // so nothing rides on it; splitting the cell needs an authored coordinate, which is a
        // DeclareIdentity act by the oracle author.
        DiagCode::SharedCellMeasurementsDisagree(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
        DiagCode::ReachesConflict(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::ReachesProviderCollision(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::WrapperEntryIncoherent(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::WrapperPeelIncoherent(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::ProvideModel,
        },
        // `dorc why --last` refusals: pull-surface disclosures ⇒ Warning + Floor::None.
        DiagCode::WhylogVersionRefused(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::WhylogBookDesync(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::WhylogAbsent(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::WhylogCorrupt(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::Structural,
        },
        DiagCode::WhylogUnwritten(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::WarnOrDeny,
            remediation: RemediationClass::Structural,
        },
        // The unloaded-sibling hint: a Note (suggest, never auto-load); ProvideModel — the oracle
        // exists on disk, loading it provides the model that would lift the wall.
        DiagCode::AidUnloadedSiblingOracle(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        // dorc-lint's own findings (`288` §5). Severities are the ones the lane-local codes
        // carried, now sourced from HERE (`crib-4`) instead of a construction site.
        DiagCode::UnmodeledWallInventory(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::VerdictTerminalPipeline(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::AuthoredDeclineClass(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        DiagCode::AuthoredDeclineClassUnreadable(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ProvideModel,
        },
        // The external-tool trio: absent/unparsable/failed are the ADMIN's environment to fix,
        // never a Dorc-modeling limitation, so none of them is `Structural`.
        DiagCode::LintToolAbsent(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
            remediation: RemediationClass::ResolveDynamism,
        },
        DiagCode::LintToolOutputUnparsable(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ResolveDynamism,
        },
        DiagCode::LintToolFailedWithoutFindings(_) => CodeSpec {
            severity: Severity::Warning,
            floor: Floor::None,
            remediation: RemediationClass::ResolveDynamism,
        },
        // Uniformly Error + no floor + DeclareIdentity. The EXIT codes are unchanged — severity is
        // registry data and never decides one (`291` §5a step 3).
        DiagCode::CliStripNeedsPath(_)
        | DiagCode::CliStripGotAFlag(_)
        | DiagCode::CliUnknownMode(_)
        | DiagCode::CliFlagNeedsValue(_)
        | DiagCode::CliUnknownFlag(_)
        | DiagCode::CliUnknownFlagDidYouMean(_)
        | DiagCode::CliFlagValueNotRecognized(_)
        | DiagCode::CliFlagValueNotANumber(_)
        | DiagCode::CliNoBookGiven(_)
        | DiagCode::CliFlagsMutuallyExclusive(_)
        | DiagCode::CliFlagRequiresMode(_)
        | DiagCode::CliFileNotFound(_)
        | DiagCode::CliFilePermissionDenied(_)
        | DiagCode::CliFileUnreadable(_)
        | DiagCode::LintNoLintableFiles(_)
        | DiagCode::LintFileCountDrift(_)
        | DiagCode::LintRequiredToolsMissing(_)
        | DiagCode::DorcShUsage(_)
        | DiagCode::DorcShScriptUnreadable(_)
        | DiagCode::DorcShExecFailed(_)
        | DiagCode::TransportCrlfRefused(_)
        | DiagCode::TransportSessionLost(_)
        | DiagCode::TransportSpawnRefused(_)
        | DiagCode::TransportMarkerUnusable(_)
        | DiagCode::TransportApplyFailed(_)
        | DiagCode::CliShimDirUnwritable(_) => CodeSpec {
            severity: Severity::Error,
            floor: Floor::None,
            remediation: RemediationClass::DeclareIdentity,
        },
    }
}

/// A code's declared severity + floor + remediation class (the [`registry`] row). Severity comes
/// from HERE, never a constructor (`crib-4`); [`remediation`](Self::remediation) is the ru-27
/// HOW-not-WHO column (gap-4 — replacing the old `remediation_for` default-to-Structural stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeSpec {
    /// The declared severity (the gate-3 floor keys on `Error`).
    pub severity: Severity,
    /// The un-overridable floor (`crib-5`).
    pub floor: Floor,
    /// The HOW-not-WHO remediation class (ru-27): the kind of fix that clears this code's origin.
    pub remediation: RemediationClass,
}

/// The un-overridable floor (`crib-5`; rustc `future-incompatible` = a floor, not a level). When
/// admin override lands (NOT this round), the floor-pinned codes cannot be silenced — the
/// few-chosen non-negotiables rustc's `forbid`/`force-warn` protect (`226` sev-1).
///
/// NB `22B-fork-severity-help` = NO top-level `Severity::Help` (conductor decision): help is a
/// [`SubDiag::Help`] child, so the registry never returns a `Help` severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    /// No floor — an admin/oracle may silence the code freely.
    None,
    /// May raise to Error but NEVER drop below Warning.
    WarnOrDeny,
    /// The rustc `forbid`/`force-warn` analog: exactly this severity, no override.
    Pinned,
}

// ===========================================================================
// The builder / constructor API (type-sketch-7) — the friction surface
// ===========================================================================

impl Diag {
    /// The one mint (`22B` `type-sketch-7` layer A): name the code (with its typed payload
    /// constructed inline, where the give-up site already holds the objects), point at the
    /// primary span. There is no severity constructor — severity is [`registry`] data
    /// (`crib-4`). This is the cheapest authoring path and lands the GOOD shape by default.
    #[must_use]
    pub fn new(code: DiagCode, primary: Span) -> Self {
        Self {
            code,
            primary: SpanLabel {
                span: SpanSite::At(primary),
                label: None,
            },
            secondary: Vec::new(),
            children: Vec::new(),
            suggestion: None,
        }
    }

    /// The deliberately SECOND-CLASS mint for a diagnostic whose emit context genuinely has NO
    /// source span (arch-3-residual-2). It produces a [`Diag`] whose primary [`SpanLabel`] carries
    /// [`SpanSite::Spanless`] — `to_legacy` then lowers it to `span: None`, byte-identically to the
    /// pre-spine `Diagnostic::{warning,error}(code, None, msg)` form those sites used.
    ///
    /// This exists ONLY for the codes whose give-up site has no span to point at:
    /// [`DiagCode::CfgErexitUnknown`] (the errexit pass spans a region, not a point) and
    /// [`DiagCode::EffectKindDisagreement`] (the annotation-vs-effect-map check fires mid-resolution
    /// with no leaf).
    /// (The two check-dialect codes [`DiagCode::PredictUnterminated`] / [`DiagCode::PredictOutOfDialect`]
    /// are NOT here: their EOF give-up synthesizes a zero-width end-of-input span and lowers through
    /// [`new`](Self::new) — human ruling 22-q1.) It is NOT a general escape hatch: [`new`](Self::new)
    /// with a real [`Span`] stays the only ordinary path, and `crates/aid/tests/diag_tidy.rs`
    /// hard-codes the allow-list (`SPANLESS_SITE_PAYLOADS`, the source of truth for the exact set) —
    /// a spanless-mint site not on it fails the gate. Do NOT use it to dodge plumbing a span that
    /// exists.
    ///
    /// **Spell the payload literally at the call.** That gate is a lexical grep for this
    /// constructor's name immediately followed by the payload's own constructor, so a variant built
    /// into a local and passed in is invisible to it, and a shared helper minting several codes
    /// hides all of them. Two builders have hit this exact red; the compiler cannot warn you.
    ///
    /// `inv-no-throw`: returns data, never panics. The mandatory-primary-span guarantee (`21Z`
    /// drop-B) is preserved for everything else BECAUSE this is the lone, self-describing, gated
    /// door to the spanless case.
    #[must_use]
    pub fn new_spanless_site(code: DiagCode) -> Self {
        Self {
            code,
            primary: SpanLabel {
                span: SpanSite::Spanless,
                label: None,
            },
            secondary: Vec::new(),
            children: Vec::new(),
            suggestion: None,
        }
    }

    /// Label the primary span (`type-sketch-7` layer B — small-`f` fluent chaining, NOT
    /// Fluent-the-i18n-system). One obvious call; nothing beyond [`new`](Self::new) is mandatory.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.primary.label = Some(label.into());
        self
    }

    /// Add a labeled SECONDARY span — the cause-site or the poisoned-site, in ONE diagnostic
    /// (`228`; `crib-1`).
    #[must_use]
    pub fn secondary(mut self, span: Span, label: impl Into<String>) -> Self {
        self.secondary.push(SpanLabel {
            span: SpanSite::At(span),
            label: Some(label.into()),
        });
        self
    }

    /// Add a [`SubDiag::Note`] child (additional fact context).
    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.children.push(SubDiag::Note(note.into()));
        self
    }

    /// Add a [`SubDiag::Help`] child (remediation guidance; CLI-only).
    #[must_use]
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.children.push(SubDiag::Help(help.into()));
        self
    }

    /// Attach the actionable [`Suggestion`] (`crib-2`).
    #[must_use]
    pub fn suggest(mut self, suggestion: Suggestion) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// This diagnostic's registry-declared severity ([`registry`] keyed by `code`; `crib-4`).
    #[must_use]
    pub fn severity(&self) -> Severity {
        registry(&self.code).severity
    }
}

// ===========================================================================
// The render model (§4): one Diag value, three lanes, authored once
// ===========================================================================

/// The named template params a code's payload supplies, in `(name, value)` form — the closed set
/// of `{holes}` the catalog message/help may interpolate (`27V` §3 · `AID-NEEDS:law-trust-tier`).
/// The per-variant formatter: a PASSTHROUGH code yields a [`ParamText::Foreign`] hole (its payload
/// field is a sealed [`ForeignBytes`]); every other hole is [`ParamText::Ours`], filled by the
/// sanctioned engine formatters — `position.describe()`, `top_cause.describe(ctx)` — never by
/// hand-written values; static-message codes yield `[]`. The `interner` is threaded for
/// forward-compat (no payload resolves an interned handle at HEAD). Pure; `inv-no-throw`.
#[must_use]
pub fn params_of(
    ctx: &RenderCtx<'_>,
    code: &DiagCode,
    _interner: &dorc_core::Interner,
) -> Vec<(&'static str, ParamText)> {
    params_of_raw(ctx, code)
}

/// The budget for one passthrough value on a diagnostic line. Generous — a passthrough is usually
/// one line of somebody's error text, not a document — and bounded, because the length was decided
/// by a platform or a book, not by us.
pub(crate) const FOREIGN_PARAM_CAP: usize = 2048;

/// The per-code hole values.
///
/// The ONE seat both renders read — the string one and the parts one — so they cannot disagree
/// about a code's values or about which of them are somebody else's bytes.
///
/// **Every arm destructures its payload EXHAUSTIVELY, with no `..`.** That is the whole reason the
/// arms are written the long way: a field added to a payload struct is `E0027` here, at the seat
/// that decides whether the new value is loom-visible. Without it a new field compiled green and
/// was silently invisible to the case corpus — `{{new_field}}` refused as an unknown name with
/// nothing to say why. Name a field to publish it; bind it `_` to say, deliberately, that it is
/// engine bookkeeping rather than a value prose may interpolate.
#[expect(
    clippy::match_same_arms,
    clippy::too_many_lines,
    reason = "one arm PER CODE, like `registry` — merging the param-less arms by `|` would hide \
              which codes declare no holes, and the exhaustive destructuring is what makes a new \
              payload field a compile error here"
)]
fn params_of_raw(ctx: &RenderCtx<'_>, code: &DiagCode) -> Vec<(&'static str, ParamText)> {
    /// A hole filled with a value the engine computed.
    fn ours(name: &'static str, text: String) -> (&'static str, ParamText) {
        (name, ParamText::Ours(text))
    }
    /// A hole filled with somebody else's bytes — spelled differently from [`ours`] at every arm so
    /// the passthrough census is a grep rather than an inspection of param names.
    fn foreign(name: &'static str, bytes: &ForeignBytes) -> (&'static str, ParamText) {
        (
            name,
            ParamText::Foreign(bytes.on_plain_sink(FOREIGN_PARAM_CAP)),
        )
    }
    /// A hole filled with a whole prose-component. Its identity travels with it: the render seat
    /// needs the slug it resolved to stamp the component's own face on a register that is nothing
    /// but this hole (`28L:rul-empty-registers-for-pure-holes`).
    fn component(name: &'static str, text: ComponentText) -> (&'static str, ParamText) {
        (name, ParamText::Component(text))
    }
    match code {
        DiagCode::CmdsubOperandTop(CmdsubOperandTop {
            site: _,
            position,
            cause: _,
            top_cause,
            command,
        }) => vec![
            ours("position", position.describe(ctx)),
            ours(
                "cause",
                crate::arrangement::arrangement_text(
                    ctx.arrangements(),
                    top_cause_slug(*top_cause),
                    None,
                ),
            ),
            ours("command", command.describe(ctx)),
        ],
        DiagCode::RenderHeredocRefused(RenderHeredocRefused {
            site: _,
            verb,
            command,
        }) => vec![
            ours("verb", (*verb).to_owned()),
            ours("command", command.clone()),
        ],
        DiagCode::CmdsubInnerNonleaf(CmdsubInnerNonleaf { site: _, inner }) => {
            vec![ours("inner", inner.clone())]
        }
        DiagCode::Depth2PositionalUnthreaded(Depth2PositionalUnthreaded { site: _, name }) => {
            vec![ours("name", name.clone())]
        }
        DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: _,
            count,
            site_word,
            names,
            excerpt,
        }) => vec![
            ours("count", count.clone()),
            ours("site_word", (*site_word).to_owned()),
            foreign("names", names),
            foreign("excerpt", excerpt),
        ],
        DiagCode::SyntaxUnsupported(SyntaxUnsupported { reason }) => {
            vec![component("reason", syntax_unsupported_text(ctx, *reason))]
        }
        DiagCode::SyntaxMalformed(SyntaxMalformed { reason }) => {
            vec![component("reason", syntax_malformed_text(ctx, *reason))]
        }
        DiagCode::CfgTopNode(CfgTopNode { reason }) => {
            vec![component("reason", cfg_top_node_text(ctx, *reason))]
        }
        DiagCode::CfgErexitUnknown(CfgErexitUnknown) => vec![],
        DiagCode::CfgInlineRefused(CfgInlineRefused { reason }) => {
            vec![component("reason", cfg_inline_refused_text(ctx, reason))]
        }
        DiagCode::CfgBuiltinShadowed(CfgBuiltinShadowed { name }) => {
            vec![ours("name", name.clone())]
        }
        DiagCode::EffectKindDisagreement(EffectKindDisagreement {
            annotated,
            effect_map,
        }) => vec![
            ours("annotated", annotated.clone()),
            ours("effect_map", effect_map.clone()),
        ],
        DiagCode::PredictOutOfDialect(PredictOutOfDialect { reason }) => {
            vec![component(
                "reason",
                predict_out_of_dialect_text(ctx, *reason),
            )]
        }
        DiagCode::PredictUnterminated(PredictUnterminated { reason }) => {
            vec![component("reason", predict_unterminated_text(ctx, *reason))]
        }
        DiagCode::OracleRoleFnUnlifted(OracleRoleFnUnlifted { funcname }) => {
            vec![ours("funcname", funcname.clone())]
        }
        DiagCode::MarkOnAndOrList(MarkOnAndOrList) => vec![],
        DiagCode::FootprintIncoherent(FootprintIncoherent { reason }) => {
            vec![component("reason", footprint_incoherent_text(ctx, *reason))]
        }
        DiagCode::EscalationPolicy(EscalationPolicy {
            dial,
            capability,
            entry_forms,
        }) => vec![component(
            "reason",
            escalation_policy_text(ctx, *dial, *capability, entry_forms),
        )],
        DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis { axes, kinds }) => {
            vec![ours("axes", axes.clone()), ours("kinds", kinds.clone())]
        }
        DiagCode::WrappedSiteAdoptionHint(WrappedSiteAdoptionHint {
            provider,
            dimension,
        }) => vec![
            ours("provider", provider.clone()),
            ours("dimension", dimension.clone()),
        ],
        DiagCode::WrapperEntryIncoherent(WrapperEntryIncoherent {
            wrapper,
            entry_shifts,
            lend_shifts,
        }) => vec![
            ours("wrapper", wrapper.clone()),
            ours("entry_shifts", entry_shifts.clone()),
            ours("lend_shifts", lend_shifts.clone()),
        ],
        DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
            wrapper,
            predict_depth,
            lend_map_depth,
        }) => vec![
            ours("wrapper", wrapper.clone()),
            ours("predict_depth", predict_depth.clone()),
            ours("lend_map_depth", lend_map_depth.clone()),
        ],
        DiagCode::WhylogVersionRefused(WhylogVersionRefused { found }) => {
            vec![ours("found", found.clone())]
        }
        DiagCode::WhylogBookDesync(WhylogBookDesync { which }) => {
            vec![ours("which", which.clone())]
        }
        DiagCode::WhylogAbsent(WhylogAbsent { dir }) => vec![ours("dir", dir.clone())],
        DiagCode::WhylogCorrupt(WhylogCorrupt { reason }) => {
            vec![component("reason", whylog_corrupt_text(ctx, *reason))]
        }
        DiagCode::WhylogUnwritten(WhylogUnwritten { dir, reason }) => {
            vec![ours("dir", dir.clone()), ours("reason", reason.clone())]
        }
        DiagCode::AidUnloadedSiblingOracle(AidUnloadedSiblingOracle { oracles }) => {
            vec![ours("oracles", oracles.clone())]
        }
        DiagCode::UnmodeledWallInventory(UnmodeledWallInventory {
            wall_count,
            wall_word,
            downstream,
        }) => vec![
            ours("wall_count", wall_count.to_string()),
            ours("wall_word", (*wall_word).to_owned()),
            ours("downstream", downstream.to_string()),
        ],
        DiagCode::VerdictTerminalPipeline(VerdictTerminalPipeline) => vec![],
        DiagCode::AuthoredDeclineClass(AuthoredDeclineClass { class }) => {
            vec![ours("class", class.clone())]
        }
        DiagCode::AuthoredDeclineClassUnreadable(AuthoredDeclineClassUnreadable) => vec![],
        DiagCode::LintToolAbsent(LintToolAbsent { tool }) => vec![ours("tool", tool.clone())],
        DiagCode::LintToolOutputUnparsable(LintToolOutputUnparsable { tool, output }) => {
            vec![ours("tool", tool.clone()), foreign("output", output)]
        }
        DiagCode::CliStripNeedsPath(CliStripNeedsPath)
        | DiagCode::CliNoBookGiven(CliNoBookGiven)
        | DiagCode::LintNoLintableFiles(LintNoLintableFiles)
        | DiagCode::DorcShUsage(DorcShUsage) => vec![],
        DiagCode::CliStripGotAFlag(CliStripGotAFlag { got }) => vec![ours("got", got.clone())],
        DiagCode::CliUnknownMode(CliUnknownMode { mode, suggestion }) => {
            vec![
                ours("mode", mode.clone()),
                ours("suggestion", suggestion.clone()),
            ]
        }
        DiagCode::CliFlagNeedsValue(CliFlagNeedsValue { flag, wants }) => {
            vec![
                ours("flag", flag.clone()),
                ours("wants", (*wants).to_owned()),
            ]
        }
        DiagCode::CliUnknownFlag(CliUnknownFlag { flag }) => vec![ours("flag", flag.clone())],
        DiagCode::CliUnknownFlagDidYouMean(CliUnknownFlagDidYouMean { flag, suggestion }) => {
            vec![
                ours("flag", flag.clone()),
                ours("suggestion", suggestion.clone()),
            ]
        }
        DiagCode::CliFlagValueNotRecognized(CliFlagValueNotRecognized {
            flag,
            got,
            expected,
        }) => vec![
            ours("flag", flag.clone()),
            ours("got", got.clone()),
            ours("expected", (*expected).to_owned()),
        ],
        DiagCode::CliFlagValueNotANumber(CliFlagValueNotANumber { flag, got }) => {
            vec![ours("flag", flag.clone()), ours("got", got.clone())]
        }
        DiagCode::CliFlagsMutuallyExclusive(CliFlagsMutuallyExclusive { first, second }) => vec![
            ours("first", (*first).to_owned()),
            ours("second", (*second).to_owned()),
        ],
        DiagCode::CliFlagRequiresMode(CliFlagRequiresMode { flag, mode }) => {
            vec![
                ours("flag", (*flag).to_owned()),
                ours("mode", (*mode).to_owned()),
            ]
        }
        DiagCode::CliFileNotFound(CliFileNotFound { kind, path }) => {
            vec![ours("kind", kind.clone()), ours("path", path.clone())]
        }
        DiagCode::CliFilePermissionDenied(CliFilePermissionDenied { kind, path }) => {
            vec![ours("kind", kind.clone()), ours("path", path.clone())]
        }
        DiagCode::CliFileUnreadable(CliFileUnreadable { kind, path, detail }) => vec![
            ours("kind", kind.clone()),
            ours("path", path.clone()),
            foreign("detail", detail),
        ],
        DiagCode::LintFileCountDrift(LintFileCountDrift { expected, found }) => vec![
            ours("expected", expected.to_string()),
            ours("found", found.to_string()),
        ],
        DiagCode::LintRequiredToolsMissing(LintRequiredToolsMissing { tools }) => {
            vec![ours("tools", tools.clone())]
        }
        DiagCode::DorcShScriptUnreadable(DorcShScriptUnreadable { path, detail }) => {
            vec![ours("path", path.clone()), foreign("detail", detail)]
        }
        DiagCode::DorcShExecFailed(DorcShExecFailed { detail }) => vec![foreign("detail", detail)],
        DiagCode::CliShimDirUnwritable(CliShimDirUnwritable { path, detail }) => {
            vec![ours("path", path.clone()), foreign("detail", detail)]
        }
        DiagCode::TransportCrlfRefused(TransportCrlfRefused { which, line }) => {
            vec![ours("which", which.clone()), ours("line", line.clone())]
        }
        DiagCode::TransportSessionLost(TransportSessionLost {
            host,
            attempts,
            diagnosis,
        }) => vec![
            ours("host", host.clone()),
            ours("attempts", attempts.clone()),
            ours("diagnosis", diagnosis.clone()),
        ],
        DiagCode::TransportSpawnRefused(TransportSpawnRefused { host, detail }) => {
            vec![ours("host", host.clone()), foreign("detail", detail)]
        }
        DiagCode::TransportMarkerUnusable(TransportMarkerUnusable { host }) => {
            vec![ours("host", host.clone())]
        }
        DiagCode::TransportApplyFailed(TransportApplyFailed { host, status }) => {
            vec![ours("host", host.clone()), ours("status", status.clone())]
        }
        DiagCode::LintToolFailedWithoutFindings(LintToolFailedWithoutFindings { tool, rc }) => {
            vec![ours("tool", tool.clone()), ours("rc", rc.to_string())]
        }
        DiagCode::MarkerVersionUnrecognized(MarkerVersionUnrecognized { found }) => {
            vec![ours("found", found.clone())]
        }
        DiagCode::MungeNameInvalid(MungeNameInvalid {
            source,
            funcname,
            problem,
        }) => vec![
            ours("source", source.clone()),
            ours("funcname", funcname.clone()),
            ours("problem", problem.clone()),
        ],
        DiagCode::MungeNameCollision(MungeNameCollision {
            source,
            funcname,
            count,
            names,
        }) => vec![
            ours("source", source.clone()),
            ours("funcname", funcname.clone()),
            ours("count", count.to_string()),
            ours("names", names.clone()),
        ],
        DiagCode::ReservedNamespaceSquat(ReservedNamespaceSquat { name, role }) => {
            vec![ours("name", name.clone()), ours("role", role.clone())]
        }
        DiagCode::ToleratesUnknownDimension(ToleratesUnknownDimension { token, expected }) => {
            vec![
                ours("token", token.clone()),
                ours("expected", expected.clone()),
            ]
        }
        DiagCode::MarkUnknownVerb(MarkUnknownVerb { token, expected }) => {
            vec![
                ours("token", token.clone()),
                ours("expected", expected.clone()),
            ]
        }
        DiagCode::LendMapUnknownDimension(LendMapUnknownDimension { token, expected }) => {
            vec![
                ours("token", token.clone()),
                ours("expected", expected.clone()),
            ]
        }
        DiagCode::CarryNetnsOnNetKernelForbidden(CarryNetnsOnNetKernelForbidden {
            kind_munged,
        }) => vec![ours("kind_munged", kind_munged.clone())],
        DiagCode::RecordsFactTruncated(RecordsFactTruncated {
            received,
            declared,
            unseen,
        }) => vec![
            ours("received", received.to_string()),
            ours("declared", declared.to_string()),
            ours("unseen", unseen.to_string()),
        ],
        DiagCode::RecordsIntegrityRefused(RecordsIntegrityRefused { which }) => {
            vec![ours("which", which.clone())]
        }
        DiagCode::RecordsTornLine(RecordsTornLine { count }) => {
            vec![ours("count", count.to_string())]
        }
        DiagCode::RecordsAlienLine(RecordsAlienLine { count }) => {
            vec![ours("count", count.to_string())]
        }
        DiagCode::RecordsLateLine(RecordsLateLine { count }) => {
            vec![ours("count", count.to_string())]
        }
        DiagCode::TouchesEscalated(TouchesEscalated { site, call }) => {
            vec![ours("site", site.to_string()), ours("call", call.clone())]
        }
        DiagCode::DerivFamilyIncomplete(DerivFamilyIncomplete { site, reason }) => {
            vec![
                ours("site", site.to_string()),
                ours("reason", reason.clone()),
            ]
        }
        DiagCode::ResolverConflict(ResolverConflict { kind, count }) => {
            vec![ours("kind", kind.clone()), ours("count", count.to_string())]
        }
        DiagCode::ResolverProviderCollision(ResolverProviderCollision { name }) => {
            vec![ours("name", name.clone())]
        }
        DiagCode::DanglingReference(DanglingReference { coord }) => {
            vec![ours("coord", coord.clone())]
        }
        DiagCode::SharedCellMeasurementsDisagree(SharedCellMeasurementsDisagree {
            cell,
            sites,
        }) => {
            vec![ours("cell", cell.clone()), ours("sites", sites.to_string())]
        }
        DiagCode::ReachesConflict(ReachesConflict { kind, count }) => {
            vec![ours("kind", kind.clone()), ours("count", count.to_string())]
        }
        DiagCode::ReachesProviderCollision(ReachesProviderCollision { name }) => {
            vec![ours("name", name.clone())]
        }
        DiagCode::RoleFamilyContested(RoleFamilyContested {
            family,
            name,
            prior,
        }) => vec![
            ours("family", family.clone()),
            ours("name", name.clone()),
            ours("prior", prior.clone()),
        ],
        DiagCode::RoleDefinedBelowItsSites(RoleDefinedBelowItsSites { name, sites }) => {
            vec![ours("name", name.clone()), ours("sites", sites.to_string())]
        }
        DiagCode::InBookVocabularyRole(InBookVocabularyRole { name, role }) => {
            vec![ours("name", name.clone()), ours("role", role.clone())]
        }
        // Static-message codes (no interpolation): no params. Their payload fields are still named
        // here, so adding one is a compile error at this seat too.
        DiagCode::RedirTargetTop(RedirTargetTop { site: _ })
        | DiagCode::OracleFileNotLoadInert(OracleFileNotLoadInert)
        | DiagCode::MissingDialectMarker(MissingDialectMarker)
        | DiagCode::ToleratesOverIdentityDependence(ToleratesOverIdentityDependence)
        | DiagCode::HeavyContextNoTolerance(HeavyContextNoTolerance)
        | DiagCode::MarkBraceVerdictSingleCell(MarkBraceVerdictSingleCell)
        | DiagCode::MarkRcArityExceeded(MarkRcArityExceeded)
        | DiagCode::MarkStandaloneRcConsumer(MarkStandaloneRcConsumer)
        | DiagCode::MarkHashcolonMalformed(MarkHashcolonMalformed)
        | DiagCode::RecordsHeaderlessRefused(RecordsHeaderlessRefused)
        | DiagCode::RecordsGluedLine(RecordsGluedLine)
        | DiagCode::RecordsHeaderMissing(RecordsHeaderMissing)
        | DiagCode::RecordsSentinelNonce(RecordsSentinelNonce)
        | DiagCode::HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused { kind: _ }) => {
            vec![]
        }
    }
}

/// The aid connective word for a code's `= <word>:` catalog-help line (`282` §12 item-2): a
/// [`RemediationClass::ResolveDynamism`] remediation reads `repair` (the fix is to make the book
/// statically resolvable), every other class stays `help`. Keyed on the registry class — the human
/// tunes the fuller class→word map iteratively as error surfaces demand it, not up front.
fn help_connective(code: &DiagCode) -> &'static str {
    match registry(code).remediation {
        RemediationClass::ResolveDynamism => "repair",
        _ => "help",
    }
}

/// The filled catalog message for a code (`27V` §3): `fill_template(catalog message, params_of)`.
/// A code with no catalog entry (unreachable once the completeness gate is green) renders the
/// greppable `[unwritten: <slug>]` placeholder. Pure; `inv-no-throw`.
#[must_use]
pub fn render_message(code: &DiagCode, interner: &dorc_core::Interner) -> String {
    render_message_with(&RenderCtx::production(), code, interner)
}

/// The [`render_message`] seat parameterized by a [`RenderCtx`]
/// (`283:dec-mirror-via-catalog-lookup`): production passes the const catalog; promote passes its
/// mutable mirror so an edit renders before any rebuild. `None` from the lookup synthesizes the
/// `[unwritten: <slug>]` placeholder (both "no entry" and "unwritten message" fold here).
#[must_use]
pub fn render_message_with(
    ctx: &RenderCtx<'_>,
    code: &DiagCode,
    interner: &dorc_core::Interner,
) -> String {
    let params = params_of(ctx, code, interner);
    let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.text())).collect();
    match ctx.catalog().message(code.slug()) {
        Some(t) => crate::catalog::fill_template(t, &refs)
            .unwrap_or_else(|_| format!("[invalid catalog template: {}]", code.slug())),
        None => format!("[unwritten: {}]", code.slug()),
    }
}

/// The CLI narrative render (`22B` `render-1`, the render-plane half of rec-1 two-surfaces): the
/// `<severity>[<slug>]: <problem>` title and the message laid out together, a source frame per
/// span (`file:line:col` locus, source-line gutter, an underline beneath the span; ack-8), then
/// the catalog help and any authored notes/suggestion as labelled rows. `src`/`filename` resolve a
/// span to a framed source excerpt (rul24-lineno-identity).
///
/// The BYTES are [`render_cli_parts`]' bytes, by construction rather than by agreement: there is
/// one render form, and the string seat is its concatenation
/// (`28L:rul-editability-is-stamped-never-re-derived`).
#[must_use]
pub fn render_cli(
    diag: &Diag,
    src: &str,
    filename: &str,
    interner: &dorc_core::Interner,
) -> String {
    render_cli_parts(&RenderCtx::production(), diag, src, filename, interner).text()
}

/// The ordered-parts twin of [`render_cli`], laid out into `ctx`'s box from `ctx`'s tables.
///
/// Layout happens INSIDE the part stream: the seat composes a document, the layout engine wraps
/// it, and every wrapped run comes back stamped with the register it was born from
/// (`28L:rul-editability-is-stamped-never-re-derived`). Nothing downstream re-derives a register,
/// a word boundary, or a section from the SHAPE of these bytes.
#[must_use]
pub fn render_cli_parts(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    src: &str,
    filename: &str,
    interner: &dorc_core::Interner,
) -> crate::tagged::RenderParts {
    let document = diagnostic_document(None, ctx, diag, src, filename, interner);
    crate::weave::to_render_parts(&weft::render_framed(&document, ctx.frame()))
}

/// Render a source-staged diagnostic (`282` §4). The stage prefix is a run INSIDE the document, so
/// the layout accounts for the columns it occupies rather than overflowing past them.
#[must_use]
pub fn render_staged_cli_parts(
    stage: &str,
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    src: &str,
    filename: &str,
    interner: &dorc_core::Interner,
) -> crate::tagged::RenderParts {
    let document = diagnostic_document(Some(stage), ctx, diag, src, filename, interner);
    crate::weave::to_render_parts(&weft::render_framed(&document, ctx.frame()))
}

/// The diagnostic as a laid-out document: the title-and-message paragraph, a source frame per
/// span, then one labelled row per continuation register.
///
/// Every English word arrives as a run from here; the engine self-mints only wordless geometry
/// (`28F:rul-weft-geometry-vs-words`). Severity, the code brackets and the `= <connective>:` lead
/// are chrome this seat COMPUTED, so they are stamped immutable and no edit can reach them.
fn diagnostic_document(
    stage: Option<&str>,
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    src: &str,
    filename: &str,
    interner: &dorc_core::Interner,
) -> weft::Document<crate::weave::Face> {
    use crate::weave::{mark, to_runs};
    use weft::{Banner, LabeledRow, Node, NodeKind};

    let mut headline = Vec::new();
    if let Some(stage) = stage {
        headline.push(mark(format!("{stage}: "), "cli-stage-prefix"));
    }
    headline.push(mark(
        format!(
            "{}[{}]: ",
            severity_word(registry(&diag.code).severity),
            diag.code.slug(),
        ),
        "cli-title",
    ));
    headline.extend(to_runs(&message_parts(ctx, diag, interner)));

    let mut body = Vec::new();
    if let Some(primary) = diag.primary.span() {
        body.push(Node::new(NodeKind::Code(frame_block(
            primary, src, filename, None, true,
        ))));
    }
    for secondary in &diag.secondary {
        if let Some(span) = secondary.span() {
            body.push(Node::new(NodeKind::Code(frame_block(
                span,
                src,
                filename,
                secondary.label.as_deref(),
                false,
            ))));
        }
    }

    let mut row = |lead: String, slug: &'static str, words: Vec<weft::Run<crate::weave::Face>>| {
        body.push(Node::new(NodeKind::Labeled(LabeledRow {
            table: None,
            label: vec![mark(lead, slug)],
            body: words,
            attachments: Vec::new(),
        })));
    };
    if let Some(help) = help_parts(ctx, diag, interner) {
        row(
            format!("= {}:", help_connective(&diag.code)),
            "cli-help-connective",
            to_runs(&help),
        );
    }
    for child in &diag.children {
        let (lead, text) = match child {
            SubDiag::Note(note) => ("= note:", note),
            SubDiag::Help(help) => ("= help:", help),
        };
        row(
            String::from(lead),
            "cli-authored-lead",
            vec![crate::weave::value(
                text,
                "cli-authored-text",
                crate::weave::RENDER_VALUE_CAP,
            )],
        );
    }
    if let Some(suggestion) = &diag.suggestion {
        row(
            String::from("= help:"),
            "cli-authored-lead",
            vec![crate::weave::value(
                format!(
                    "{} [{}]",
                    suggestion.message,
                    remediation_tag(suggestion.remediation)
                ),
                "cli-authored-text",
                crate::weave::RENDER_VALUE_CAP,
            )],
        );
    }
    weft::Document::new(vec![Node::new(NodeKind::Banner(Banner { headline, body }))])
}

fn push_arrangement_part(parts: &mut crate::tagged::RenderParts, text: String, slug: &'static str) {
    if !text.is_empty() {
        parts.push(crate::tagged::RenderPart::Arrangement { text, slug });
    }
}

/// The full rendered MESSAGE TEXT of a diagnostic (`render-1`): the filled catalog message, then
/// the catalog help + any authored notes/suggestion as ` = help:`/` = note:` continuation lines.
/// The lint crate uses this verbatim as a finding's message; [`render_cli`] and the cli's
/// `report()` split its first line onto the title and place the rest after the region. Pure.
#[must_use]
pub fn render_body(diag: &Diag, interner: &dorc_core::Interner) -> String {
    render_body_with(&RenderCtx::production(), diag, interner)
}

/// The [`render_body`] seat parameterized by a [`RenderCtx`]
/// (`283:dec-mirror-via-catalog-lookup`): production passes the const catalog, promote its mirror.
/// Byte-identical to [`render_body`] under the production context (gate-pinned).
#[must_use]
pub fn render_body_with(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    interner: &dorc_core::Interner,
) -> String {
    use std::fmt::Write;
    let params = params_of(ctx, &diag.code, interner);
    let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.text())).collect();
    let slug = diag.code.slug();
    let mut out = match ctx.catalog().message(slug) {
        Some(t) => crate::catalog::fill_template(t, &refs)
            .unwrap_or_else(|_| format!("[invalid catalog template: {slug}]")),
        None => format!("[unwritten: {slug}]"),
    };
    match ctx.catalog().help(slug) {
        crate::catalog::HelpRegister::Absent => {}
        crate::catalog::HelpRegister::Unwritten => {
            let _ = write!(
                out,
                "\n  = {}: {}",
                help_connective(&diag.code),
                unwritten_help_placeholder(slug)
            );
        }
        crate::catalog::HelpRegister::Written(help) => {
            let _ = write!(
                out,
                "\n  = {}: {}",
                help_connective(&diag.code),
                crate::catalog::fill_template(help, &refs)
                    .unwrap_or_else(|_| format!("[invalid catalog template: {slug}]"))
            );
        }
    }
    // Authored children + suggestion: empty in production, exercised by the builder tests.
    for child in &diag.children {
        match child {
            SubDiag::Note(n) => {
                let _ = write!(out, "\n  = note: {n}");
            }
            SubDiag::Help(h) => {
                let _ = write!(out, "\n  = help: {h}");
            }
        }
    }
    if let Some(s) = &diag.suggestion {
        let _ = write!(
            out,
            "\n  = help: {} [{}]",
            s.message,
            remediation_tag(s.remediation)
        );
    }
    out
}

/// The ordered-parts twin of [`render_body`].
#[must_use]
pub fn render_body_parts(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    interner: &dorc_core::Interner,
) -> crate::tagged::RenderParts {
    render_body_parts_with(ctx, diag, interner)
}

/// The MESSAGE register as parts: the filled catalog template, or the greppable
/// `[unwritten: <slug>]` placeholder when no words are authored yet.
///
/// The placeholder wears the REGISTER's face (`28L:rul-placeholder-wears-the-register-face`): its
/// text stays computed and nothing is stored (`28F:rul-placeholders-are-computed`), but it is
/// stamped with the key an authored message would carry, so the transport opens the message
/// section over it and overtyping it is how a code acquires its first words.
fn message_parts(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    interner: &dorc_core::Interner,
) -> crate::tagged::RenderParts {
    let params = params_of(ctx, &diag.code, interner);
    let code = diag.code.slug();
    let mut parts = crate::tagged::RenderParts::new();
    match ctx.catalog().message(code) {
        Some(template) => match crate::catalog::fill_template_parts(
            template,
            &params,
            code,
            crate::tagged::Field::Message,
            0,
        ) {
            Ok(field_parts) => parts.append(field_parts),
            Err(_) => push_arrangement_part(
                &mut parts,
                format!("[invalid catalog template: {code}]"),
                "invalid-template",
            ),
        },
        None => parts.push(crate::tagged::RenderPart::TemplateLiteral {
            text: format!("[unwritten: {code}]"),
            code,
            field: crate::tagged::Field::Message,
            paragraph: 0,
            instance: 0,
        }),
    }
    parts
}

/// The greppable placeholder a code renders while its help register is seeded but unwritten.
///
/// Suffixed, unlike the message's, because both registers can appear in ONE render and two
/// identical placeholder strings would leave an author (and the edit transport) unable to say
/// which one they meant.
#[must_use]
pub fn unwritten_help_placeholder(slug: &str) -> String {
    format!("[unwritten: {slug}.help]")
}

/// The HELP register as parts, without its `= <connective>:` lead — the lead is the seat's chrome
/// and belongs to whichever arrangement the seat puts the register in.
///
/// `None` is the code carrying no help register at all; a SEEDED-but-unwritten one renders the
/// placeholder wearing the register's own face, exactly as the message does
/// (`28L:rul-placeholder-wears-the-register-face`).
fn help_parts(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    interner: &dorc_core::Interner,
) -> Option<crate::tagged::RenderParts> {
    let params = params_of(ctx, &diag.code, interner);
    let code = diag.code.slug();
    let mut parts = crate::tagged::RenderParts::new();
    match ctx.catalog().help(code) {
        crate::catalog::HelpRegister::Absent => return None,
        crate::catalog::HelpRegister::Unwritten => {
            parts.push(crate::tagged::RenderPart::TemplateLiteral {
                text: unwritten_help_placeholder(code),
                code,
                field: crate::tagged::Field::Help,
                paragraph: 0,
                instance: 0,
            });
        }
        crate::catalog::HelpRegister::Written(help) => {
            match crate::catalog::fill_template_parts(
                help,
                &params,
                code,
                crate::tagged::Field::Help,
                0,
            ) {
                Ok(field_parts) => parts.append(field_parts),
                Err(_) => push_arrangement_part(
                    &mut parts,
                    format!("[invalid catalog template: {code}]"),
                    "invalid-template",
                ),
            }
        }
    }
    Some(parts)
}

fn render_body_parts_with(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    interner: &dorc_core::Interner,
) -> crate::tagged::RenderParts {
    let mut parts = message_parts(ctx, diag, interner);
    if let Some(help) = help_parts(ctx, diag, interner) {
        push_arrangement_part(
            &mut parts,
            format!("\n  = {}: ", help_connective(&diag.code)),
            "help-connective",
        );
        parts.append(help);
    }
    for child in &diag.children {
        let (text, slug) = match child {
            SubDiag::Note(note) => (format!("\n  = note: {note}"), "authored-note"),
            SubDiag::Help(help) => (format!("\n  = help: {help}"), "authored-help"),
        };
        push_arrangement_part(&mut parts, text, slug);
    }
    if let Some(suggestion) = &diag.suggestion {
        push_arrangement_part(
            &mut parts,
            format!(
                "\n  = help: {} [{}]",
                suggestion.message,
                remediation_tag(suggestion.remediation)
            ),
            "authored-suggestion",
        );
    }
    parts
}

/// The artifact-comment render (`22B` `render-3`, the ru-12 weld). A shipped `.sh` artifact may
/// carry AT MOST a FACT-PLANE projection of a diagnostic — a provenance comment naming the
/// fact, never the narrative prose, the help/remediation, or any [`ProvId`]-derived receipt
/// text. The enforcement is THIS function's type: the exempt-plane fields (`suggestion`, helps,
/// `cause`) are simply not read here. Returns `None` when no fact-plane comment is warranted (a
/// disclosure that belongs only in the render plane).
///
/// rec-1 two-surfaces (round-22 standing ruling): this is the artifact surface; [`render_cli`] is
/// the render/overlay surface. The adversarial erasability gate asserts the artifact is
/// byte-identical with receipts stripped — this partition is what makes that TRUE by
/// construction (the receipt fields never reach the bytes). The Error-class render-refusal is the
/// one migrated code whose fact is artifact-relevant (a leaf ran verbatim); the Notes are
/// render-plane disclosures and return `None`.
#[must_use]
pub fn render_artifact_comment(diag: &Diag) -> Option<String> {
    match &diag.code {
        // A render-refusal is a fact about the artifact (this leaf was NOT elided). Surface the
        // fact-plane site, never the prose. (Today the cli does not weave this into the artifact —
        // the existing provenance comments cover the elided sites; this is the SLOT, fact-plane by
        // construction, for when refusals annotate the artifact.)
        DiagCode::RenderHeredocRefused(p) => Some(format!(
            "# render-refused (heredoc): site {}{} runs verbatim",
            p.site.leaf.0,
            p.site.member.map(|m| format!(".{m}")).unwrap_or_default()
        )),
        // All other codes: pure render-plane disclosures or give-ups; no fact-plane artifact
        // comment (the apply runs the site; the existing unresolvable-no-probe comment, if any,
        // is the cli's, not this projection's).
        _ => None,
    }
}

// ===========================================================================
// The why-lens (round-22 arch-2 / 22D): the first receipt-READER. Reads a diag's ⊤-cause +
// the arena → a per-line "why did this command run (never elided)?" explanation.
// ===========================================================================

/// A per-line why-lens explanation (`22D` stage-2): the smallest honest answer to "why did this
/// command RUN (never elided)?", paired with the remediation class that says WHICH user clears it.
///
/// This is the CONSUMER side of the arch-1 receipt plane — it READS the ⊤-cause minted in the
/// `ProvArena` and surfaced on a [`Diag`] (stage-1 wired [`CmdsubOperandTop::cause`]). It is
/// render-plane only (`dir-soundiness-ux`: frontload the unsoundness where the operator reads, at
/// the decision point); it reaches no artifact and (ru-11 WELD) drives no decision — it is pure
/// OUTPUT explanation, on the `Exempt::Explanation` plane.
#[derive(Debug, Clone)]
pub struct Explanation {
    /// The cause-derived reason this command was forced to run, as the ordered fragments it is
    /// composed of: registry words, the computed cause coordinate, the book's own bytes, and the
    /// remediation hint. PARTS AT BIRTH (`28G` Phase W4) — a reason that arrives as one flattened
    /// string can never tell a surface which registry row an edit would rewrite, and the two why
    /// surfaces then disagree about what the bytes even are.
    pub parts: Vec<Said>,
    /// Which user action clears the forced run (ru-6; PROPOSED per code — `tc-whylens-remediation`).
    pub remediation: RemediationClass,
}

impl Explanation {
    /// The whole reason as bytes — for a seat with no span map to hand the parts to.
    #[must_use]
    pub fn text(&self, ctx: &RenderCtx<'_>) -> String {
        self.parts.iter().map(|said| said.text(ctx)).collect()
    }
}

/// The why-lens primitive (`22D` stage-2): read a [`Diag`]'s ⊤-cause + the [`ProvArena`] →
/// a per-line [`Explanation`] of why the command was forced to run, or `None` when this
/// diagnostic carries no caused-⊤ the why-lens can honestly explain.
///
/// HONESTY (fd-G, the load-bearing scope): the why-lens covers CAUSED ⊤s only — today the
/// reliable-oracle value-⊤ case carried by [`DiagCode::CmdsubOperandTop`] with a wired
/// `cause: Some`. For every other code (the oracle-lifter give-up codes carry no cause and
/// `site() == None`; the other migrated codes have no cause field at HEAD) this returns `None`
/// — those codes keep their OWN existing message; the why-lens is ADDITIVE for caused ⊤s, and
/// must NOT overclaim "every forced-run has a why". A `CmdsubOperandTop` whose `cause` is
/// somehow `None`, or whose cause does not resolve in `arena`, also yields `None` (no fabrication).
///
/// THE WELD (ru-11 one-way): this READS the cause to render text; it makes no decision, keys no
/// disposition, and returns data, never panicking (`inv-no-throw`). The cause is a non-`Display`,
/// non-`Ord` [`ProvId`] resolved through the arena's sole reader ([`ProvArena::node`]).
///
/// Minimal-witness-first (228): the cause-site is shown ONCE, the smallest honest explanation.
/// `src` resolves the cause's origin span to source text for orientation (referent-agnostic —
/// shown, never decoded).
#[must_use]
pub fn why(
    ctx: &RenderCtx<'_>,
    diag: &Diag,
    arena: &dorc_core::ProvArena,
    src: &str,
) -> Option<Explanation> {
    // Only a CmdsubOperandTop carries a ⊤-cause at HEAD (stage-1). Other codes: no caused-⊤ to
    // read ⇒ no why-lens line (fd-G honesty — they keep their own message).
    let DiagCode::CmdsubOperandTop(payload) = &diag.code else {
        return None;
    };
    // The cause must be present (stage-1 wires it) AND resolve to a real arena origin. A `None`
    // cause or an unresolvable id yields no explanation — the why-lens never fabricates a why.
    let cause = payload.cause?;
    let origin = arena.node(cause)?;
    let remediation = remediation_for(&diag.code);
    // ack-4 (vocabulary): plain unambiguous English on this user-facing line — "couldn't be
    // resolved" / "to stay safe" instead of the "⊤"/"kFAIL-perform" jargon (which stays in
    // code/comments/corpus).
    let mut parts = vec![Said::words(
        "why-reason-cmdsub-opener",
        &[&payload.position.describe(ctx)],
    )];
    // The cause-site, shown once (minimal-witness): the give-up origin's source span, resolved for
    // orientation. A site-less origin (the defensive fallback cause) says so in its place.
    match origin.site {
        Some(span) => parts.extend(cause_locus(span, src)),
        None => parts.push(Said::words("why-reason-cmdsub-locus-absent", &[])),
    }
    parts.push(Said::words("why-reason-cmdsub-closer", &[]));
    parts.push(Said::words(remediation_hint_slug(remediation), &[]));
    Some(Explanation { parts, remediation })
}

/// The remediation class for a code — now the [`registry`] column (ru-27; gap-4), replacing the old
/// `remediation_for` default-to-Structural stub. The why-lens reads it through here so a caused-⊤
/// renders the right fix-kind.
fn remediation_for(code: &DiagCode) -> RemediationClass {
    registry(code).remediation
}

/// The arrangement-registry key holding one ?-cause's naming phrase.
///
/// The DECIDE plane holds no user-facing words (`aid-is-the-describe-plane`), so what `core` owns
/// is the enum and what this plane owns is the sentence — the `remediation_hint_slug` shape, one
/// tier down. Occurrence-less: one entry serves every site that reaches a cause.
fn top_cause_slug(cause: TopCause) -> &'static str {
    match cause {
        TopCause::UnmodeledExpansion => "why-top-cause-unmodeled-expansion",
        TopCause::UnresolvablePositional => "why-top-cause-unresolvable-positional",
        TopCause::DynamicParameter => "why-top-cause-dynamic-parameter",
        TopCause::DynamicValue => "why-top-cause-dynamic-value",
        TopCause::SplitOrGlob => "why-top-cause-split-or-glob",
        TopCause::NonConvergent => "why-top-cause-non-convergent",
        TopCause::WalledRead => "why-top-cause-walled-read",
    }
}

/// The arrangement-registry key holding a class's one-line remediation hint (the why-lens's
/// `<remediation hint>` tail). The hint PROSE lives in the registry
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`); what stays here is only the
/// class → key map, so the last hardcoded user-facing prose class in the crate has an editable
/// home like every other user-facing string (`288` §1). Occurrence-less: one entry serves every
/// site that reaches a given class.
fn remediation_hint_slug(class: RemediationClass) -> &'static str {
    match class {
        RemediationClass::ProvideModel => "why-remediation-provide-model",
        RemediationClass::DeclareIdentity => "why-remediation-declare-identity",
        RemediationClass::ResolveDynamism => "why-remediation-resolve-dynamism",
        RemediationClass::Structural => "why-remediation-structural",
    }
}

// ===========================================================================
// Reason → sentence maps (`28L:rul-reason-enums-not-sibling-codes`)
// ===========================================================================
//
// Each map below is the [`top_cause_slug`] shape extended to reasons that INTERPOLATE: one
// arrangement component per reason, and the reason's own fields as that component's values. The
// map is a single exhaustive `match`, so a new variant is a compile error here — at the seat that
// decides what the new world says.
// They answer with the resolved COMPONENT rather than its bytes, which is what lets a whole-hole
// register wear the component's face (`28L:rul-empty-registers-for-pure-holes`).

/// The registry sentence for one [`CfgTopNodeReason`].
fn cfg_top_node_text(ctx: &RenderCtx<'_>, reason: CfgTopNodeReason) -> ComponentText {
    let slug = match reason {
        CfgTopNodeReason::UnsupportedConstruct => "cfg-top-node-unsupported-construct",
        CfgTopNodeReason::NestingBound => "cfg-top-node-nesting-bound",
    };
    component_text(ctx.arrangements(), slug, None, &[])
}

/// The registry sentence for one [`CfgInlineRefusedReason`].
fn cfg_inline_refused_text(ctx: &RenderCtx<'_>, reason: &CfgInlineRefusedReason) -> ComponentText {
    let arrangements = ctx.arrangements();
    let (slug, values): (&'static str, Vec<String>) = match reason {
        CfgInlineRefusedReason::Redefined { name } => {
            ("cfg-inline-refused-redefined", vec![name.clone()])
        }
        CfgInlineRefusedReason::RecursiveCall { name } => {
            ("cfg-inline-refused-recursive-call", vec![name.clone()])
        }
        CfgInlineRefusedReason::DepthBudget { name, budget } => (
            "cfg-inline-refused-depth-budget",
            vec![name.clone(), budget.to_string()],
        ),
        CfgInlineRefusedReason::UnmodeledPositional { name, construct } => (
            "cfg-inline-refused-unmodeled-positional",
            vec![name.clone(), (*construct).to_owned()],
        ),
        CfgInlineRefusedReason::WriteRedirect { name, redirect } => (
            "cfg-inline-refused-write-redirect",
            vec![name.clone(), unmodeled_write_redirect_text(ctx, redirect)],
        ),
        CfgInlineRefusedReason::PerCallNodeBudget {
            name,
            estimate,
            budget,
        } => (
            "cfg-inline-refused-per-call-budget",
            vec![name.clone(), estimate.to_string(), budget.to_string()],
        ),
        CfgInlineRefusedReason::PerBookNodeBudget {
            name,
            spliced,
            estimate,
            budget,
        } => (
            "cfg-inline-refused-per-book-budget",
            vec![
                name.clone(),
                spliced.to_string(),
                estimate.to_string(),
                budget.to_string(),
            ],
        ),
    };
    component_text(arrangements, slug, None, &borrowed(&values))
}

/// The values a reason map collected, as the arity seat takes them.
fn borrowed(values: &[String]) -> Vec<&str> {
    values.iter().map(String::as_str).collect()
}

/// The registry sentence for one [`SyntaxUnsupportedReason`].
fn syntax_unsupported_text(ctx: &RenderCtx<'_>, reason: SyntaxUnsupportedReason) -> ComponentText {
    let none: Vec<&str> = Vec::new();
    let (slug, values) = match reason {
        SyntaxUnsupportedReason::ParserStalled => ("syntax-unsupported-parser-stalled", none),
        SyntaxUnsupportedReason::NestingBound => ("syntax-unsupported-nesting-bound", none),
        SyntaxUnsupportedReason::ReservedWordInCommandPosition => {
            ("syntax-unsupported-reserved-word-in-command-position", none)
        }
        SyntaxUnsupportedReason::ConstructTrailingRedirection { construct, closer } => (
            "syntax-unsupported-construct-trailing-redirection",
            vec![construct, closer],
        ),
        SyntaxUnsupportedReason::ForWithoutVariableName => {
            ("syntax-unsupported-for-without-variable-name", none)
        }
        SyntaxUnsupportedReason::ForWithoutInList => {
            ("syntax-unsupported-for-without-in-list", none)
        }
        SyntaxUnsupportedReason::ForListWordHasExpansion => {
            ("syntax-unsupported-for-list-word-has-expansion", none)
        }
        SyntaxUnsupportedReason::ForListNotTerminated => {
            ("syntax-unsupported-for-list-not-terminated", none)
        }
        SyntaxUnsupportedReason::LoopJumpInBody => ("syntax-unsupported-loop-jump-in-body", none),
        SyntaxUnsupportedReason::LoopJumpInBodyOrCondition => {
            ("syntax-unsupported-loop-jump-in-body-or-condition", none)
        }
        SyntaxUnsupportedReason::BackgroundAmp => ("syntax-unsupported-background-amp", none),
        SyntaxUnsupportedReason::OperatorWithoutCommand => {
            ("syntax-unsupported-operator-without-command", none)
        }
        SyntaxUnsupportedReason::DoubleSemicolonOutsideCase => {
            ("syntax-unsupported-double-semicolon-outside-case", none)
        }
        SyntaxUnsupportedReason::ExpectedACommand => {
            ("syntax-unsupported-expected-a-command", none)
        }
        SyntaxUnsupportedReason::ArithmeticAsCommand => {
            ("syntax-unsupported-arithmetic-as-command", none)
        }
        SyntaxUnsupportedReason::DynamicCommandName => {
            ("syntax-unsupported-dynamic-command-name", none)
        }
        SyntaxUnsupportedReason::EvalConstructedCode => {
            ("syntax-unsupported-eval-constructed-code", none)
        }
        SyntaxUnsupportedReason::SourceOfNonLiteralTarget => {
            ("syntax-unsupported-source-of-non-literal-target", none)
        }
        SyntaxUnsupportedReason::UnsetDynamicLvalue => {
            ("syntax-unsupported-unset-dynamic-lvalue", none)
        }
        SyntaxUnsupportedReason::PrintfWritesLvalue => {
            ("syntax-unsupported-printf-writes-lvalue", none)
        }
        SyntaxUnsupportedReason::TestReferencesLvalue => {
            ("syntax-unsupported-test-references-lvalue", none)
        }
        SyntaxUnsupportedReason::ExpectedAWord => ("syntax-unsupported-expected-a-word", none),
    };
    component_text(ctx.arrangements(), slug, None, &values)
}

/// The registry sentence for one [`SyntaxMalformedReason`].
fn syntax_malformed_text(ctx: &RenderCtx<'_>, reason: SyntaxMalformedReason) -> ComponentText {
    let slug = match reason {
        SyntaxMalformedReason::ExpectedThenAfterIf => "syntax-malformed-expected-then-after-if",
        SyntaxMalformedReason::ExpectedThenAfterElif => "syntax-malformed-expected-then-after-elif",
        SyntaxMalformedReason::ExpectedFiToCloseIf => "syntax-malformed-expected-fi-to-close-if",
        SyntaxMalformedReason::ExpectedInAfterCaseWord => {
            "syntax-malformed-expected-in-after-case-word"
        }
        SyntaxMalformedReason::ExpectedEsacToCloseCase => {
            "syntax-malformed-expected-esac-to-close-case"
        }
        SyntaxMalformedReason::ExpectedDoToOpenLoopBody => {
            "syntax-malformed-expected-do-to-open-loop-body"
        }
        SyntaxMalformedReason::ExpectedDoneToCloseLoop => {
            "syntax-malformed-expected-done-to-close-loop"
        }
        SyntaxMalformedReason::UnterminatedCaseArm => "syntax-malformed-unterminated-case-arm",
        SyntaxMalformedReason::ExpectedRparenAfterCasePattern => {
            "syntax-malformed-expected-rparen-after-case-pattern"
        }
        SyntaxMalformedReason::UnterminatedSubshell => "syntax-malformed-unterminated-subshell",
        SyntaxMalformedReason::UnterminatedBraceGroup => {
            "syntax-malformed-unterminated-brace-group"
        }
    };
    component_text(ctx.arrangements(), slug, None, &[])
}

/// The registry sentence for one [`FootprintIncoherentReason`].
fn footprint_incoherent_text(
    ctx: &RenderCtx<'_>,
    reason: FootprintIncoherentReason,
) -> ComponentText {
    let slug = match reason {
        FootprintIncoherentReason::OmitsOwnCoordinate => {
            "footprint-incoherent-omits-own-coordinate"
        }
        FootprintIncoherentReason::MalformedDerivedCoordinate => {
            "footprint-incoherent-malformed-derived-coordinate"
        }
    };
    component_text(ctx.arrangements(), slug, None, &[])
}

/// The registry sentence for one [`PredictOutOfDialectReason`].
fn predict_out_of_dialect_text(
    ctx: &RenderCtx<'_>,
    reason: PredictOutOfDialectReason,
) -> ComponentText {
    use PredictOutOfDialectReason as R;
    let arrangements = ctx.arrangements();
    let none: Vec<String> = Vec::new();
    let (slug, values) = match reason {
        R::MalformedFunctionHeader => ("predict-out-of-dialect-malformed-function-header", none),
        R::FunctionBodyMustStartWithBrace => {
            ("predict-out-of-dialect-body-must-start-with-brace", none)
        }
        R::CheckBodyOutOfDialect => ("predict-out-of-dialect-check-body", none),
        R::AndOrListNotLedByCommand => ("predict-out-of-dialect-and-or-list-not-led", none),
        R::AndOrListItemNotCommand => ("predict-out-of-dialect-and-or-item-not-command", none),
        R::ExpectedDoAfterWhileTest => ("predict-out-of-dialect-expected-do-after-while", none),
        R::ExpectedThenAfterIfTest => ("predict-out-of-dialect-expected-then-after-if", none),
        R::ExpectedInAfterCaseScrutinee => ("predict-out-of-dialect-expected-in-after-case", none),
        R::UnterminatedCaseExpectedEsac => ("predict-out-of-dialect-unterminated-case", none),
        R::ExpectedPipeOrRparenInCaseArmPattern => {
            ("predict-out-of-dialect-expected-pipe-or-rparen", none)
        }
        R::CasePatternOutOfDialect => ("predict-out-of-dialect-case-pattern", none),
        R::ExpectedCaseArmPattern => ("predict-out-of-dialect-expected-case-arm-pattern", none),
        R::ShiftCountNotLiteralInteger => ("predict-out-of-dialect-shift-count", none),
        R::StatementDoesNotStartWithWord => ("predict-out-of-dialect-statement-not-a-word", none),
        R::AnnotationNeedsValueWord => ("predict-out-of-dialect-annotation-value", none),
        R::OutOfDialectToken { lex } => (
            "predict-out-of-dialect-token-in-command",
            vec![predict_lex_error_text(ctx, lex)],
        ),
        R::UnexpectedTokenInCommand => ("predict-out-of-dialect-unexpected-token", none),
        R::EmptyCommand => ("predict-out-of-dialect-empty-command", none),
        R::ExpectedAWord => ("predict-out-of-dialect-expected-a-word", none),
        R::ExpectedLbracketToOpenTest => ("predict-out-of-dialect-expected-test-open", none),
        R::TestOperatorNotStringComparison => ("predict-out-of-dialect-test-operator", none),
        R::ExpectedRbracketToCloseTest => ("predict-out-of-dialect-expected-test-close", none),
        R::TrailingBindMarkWithValue => ("predict-out-of-dialect-trailing-bind-with-value", none),
        R::MarkNeedsVerbOrCoordinate => ("predict-out-of-dialect-mark-needs-verb", none),
        R::TrailingBindMarkWord => ("predict-out-of-dialect-trailing-bind-word", none),
        R::MarkNeedsPayload => ("predict-out-of-dialect-mark-needs-payload", none),
        R::MalformedMarkTarget => ("predict-out-of-dialect-malformed-mark-target", none),
        R::SelectorNotPosixName => ("predict-out-of-dialect-selector-charset", none),
    };
    component_text(arrangements, slug, None, &borrowed(&values))
}

/// The registry sentence for one [`PredictLexError`] — the inner clause of
/// [`PredictOutOfDialectReason::OutOfDialectToken`].
fn predict_lex_error_text(ctx: &RenderCtx<'_>, lex: PredictLexError) -> String {
    let slug = match lex {
        PredictLexError::UnmodeledByte => "predict-lex-unmodeled-byte",
        PredictLexError::BacktickCommandSubstitution => "predict-lex-backtick-substitution",
        PredictLexError::UnterminatedQuote => "predict-lex-unterminated-quote",
    };
    crate::arrangement::arrangement_text(ctx.arrangements(), slug, None)
}

/// The registry sentence for one [`PredictUnterminatedReason`].
fn predict_unterminated_text(
    ctx: &RenderCtx<'_>,
    reason: PredictUnterminatedReason,
) -> ComponentText {
    let none: Vec<&str> = Vec::new();
    let (slug, values) = match reason {
        PredictUnterminatedReason::FunctionBody => ("predict-unterminated-function-body", none),
        PredictUnterminatedReason::Block { keyword } => {
            ("predict-unterminated-block", vec![keyword])
        }
        PredictUnterminatedReason::CaseArm => ("predict-unterminated-case-arm", none),
        PredictUnterminatedReason::IfThen => ("predict-unterminated-if-then", none),
    };
    component_text(ctx.arrangements(), slug, None, &values)
}

/// The registry sentence for one [`WhylogCorruptReason`].
fn whylog_corrupt_text(ctx: &RenderCtx<'_>, reason: WhylogCorruptReason) -> ComponentText {
    let slug = match reason {
        WhylogCorruptReason::Headerless => "whylog-corrupt-headerless",
        WhylogCorruptReason::HeaderTagMissing => "whylog-corrupt-header-tag-missing",
        WhylogCorruptReason::ResultsBlockOverruns => "whylog-corrupt-results-block-overruns",
        WhylogCorruptReason::EndSentinelMissing => "whylog-corrupt-end-sentinel-missing",
    };
    component_text(ctx.arrangements(), slug, None, &[])
}

/// The registry sentence disclosing the escalation policy at one [`EscalationDial`].
fn escalation_policy_text(
    ctx: &RenderCtx<'_>,
    dial: EscalationDial,
    capability: Capability,
    entry_forms: &str,
) -> ComponentText {
    let arrangements = ctx.arrangements();
    let capability_word = crate::arrangement::arrangement_text(
        arrangements,
        match capability {
            Capability::Root => "escalation-policy-capability-root",
            Capability::NonRootNopasswd => "escalation-policy-capability-nonroot-nopasswd",
            Capability::Degraded => "escalation-policy-capability-degraded",
        },
        None,
    );
    let (slug, values) = match dial {
        EscalationDial::NoEscalation => ("escalation-policy-no-escalation", vec![entry_forms]),
        EscalationDial::VouchedOnly => (
            "escalation-policy-vouched-only",
            vec![capability_word.as_str(), entry_forms],
        ),
        EscalationDial::AnyProbe => (
            "escalation-policy-any-probe",
            vec![capability_word.as_str(), entry_forms],
        ),
    };
    component_text(arrangements, slug, None, &values)
}

/// The registry sentence for one [`UnmodeledWriteRedirect`] — the inner clause of
/// [`CfgInlineRefusedReason::WriteRedirect`].
fn unmodeled_write_redirect_text(ctx: &RenderCtx<'_>, redirect: &UnmodeledWriteRedirect) -> String {
    let arrangements = ctx.arrangements();
    match redirect {
        UnmodeledWriteRedirect::ToPath { path } => crate::arrangement::arrangement_sentence(
            arrangements,
            "cfg-inline-refused-redirect-to-path",
            None,
            &[path],
        ),
        UnmodeledWriteRedirect::ToDynamicTarget => crate::arrangement::arrangement_text(
            arrangements,
            "cfg-inline-refused-redirect-dynamic",
            None,
        ),
    }
}

/// The OOB-lane projection (`22B` `render-2`): the FACT-PLANE fields a diagnostic contributes to
/// the out-of-band site-keyed record lane — `{ site, code-slug, severity }`. No prose, no help,
/// no [`ProvId`]. The [`SiteId`] is already the lane's key, so a diagnostic and its OOB record
/// share identity for free. The `code=` field is the stable string slug (`22B-fork-wire-code` —
/// a WIRE-FORMAT COMMITMENT, flagged).
///
/// This is the slot (string-slug wire code), authored as a pure function of the `Diag`, not a
/// second authoring. The lane grammar's growth to actually CARRY this is the cli's, not built
/// here (the cli's record protocol is `site <key> effect=… rc=…` today).
#[must_use]
pub fn project_oob(diag: &Diag) -> OobProjection {
    OobProjection {
        site: diag.code.site(),
        code: diag.code.slug(),
        severity: registry(&diag.code).severity,
    }
}

/// The fact-plane fields a [`Diag`] projects to the OOB site-keyed lane ([`project_oob`]). A pure
/// projection — prose/help/receipts are NOT here (the OOB lane is fact-plane, `render-2`).
///
/// `site` is `Option<SiteId>` because some codes (pre-CFG `syntax-*`, oracle lifter codes)
/// carry no natural site. The lane key degenerates gracefully — these codes are not probe-result
/// codes and never key a site-keyed record anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OobProjection {
    /// The lane key (shared with the diagnostic's site identity), if available.
    pub site: Option<SiteId>,
    /// The stable wire slug (`22B-fork-wire-code`).
    pub code: &'static str,
    /// The registry severity.
    pub severity: Severity,
}

// ===========================================================================
// Small render helpers (pure, allocation-light)
// ===========================================================================

/// The severity word for the title line (matches the cli `report()` vocabulary so gate-3's
/// `<sev>[<code>]` floor keys identically).
fn severity_word(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    }
}

/// The `[remediation-class]` inline tag (`render-1`): a stable lowercase slug per class.
fn remediation_tag(class: RemediationClass) -> &'static str {
    match class {
        RemediationClass::ResolveDynamism => "resolve-dynamism",
        RemediationClass::DeclareIdentity => "declare-identity",
        RemediationClass::ProvideModel => "provide-model",
        RemediationClass::Structural => "structural",
    }
}

/// A ⊤-cause site as the fragments it renders from: the no-caret `<line>:<col>` locus, then the
/// book's own bytes quoted, when the span is in range (the drop-A fix — the span reaches the user).
/// Referent-agnostic: the source text is shown for orientation, never decoded.
///
/// The coordinate is [`line_col`]'s, exactly as [`frame_block`]'s locus is: this seat used to print
/// the span's raw BYTE OFFSETS in the same `N:M` shape the rest of the surface spells line:col in,
/// so a reader following the number landed on the wrong line and had no way to tell
/// (**rul24-lineno-identity**: one line-number space, the source file's).
///
/// The split is the point (`ask-why-lens-stderr-unencoded`): the coordinate is ours and computed,
/// the excerpt is the BOOK's, so it enters as a foreign fragment and is encoded at that mint —
/// which is what makes the stderr lens as safe as the weft-rendered report, rather than safe only
/// where somebody remembered.
fn cause_locus(span: Span, src: &str) -> Vec<Said> {
    let lo = span.lo.0 as usize;
    let hi = span.hi.0 as usize;
    let (line, col) = line_col(src, lo);
    let coordinate = Said::Value(format!("{line}:{col}"));
    match src.get(lo..hi) {
        Some(text) => vec![
            coordinate,
            Said::Mark(CAUSE_QUOTE, " `".to_owned()),
            Said::foreign(&ForeignBytes::from_io_edge(text), "the book"),
            Said::Mark(CAUSE_QUOTE, "`".to_owned()),
        ],
        None => vec![coordinate],
    }
}

/// The seat name the quotes around a cause excerpt wear in the span map. Punctuation, not words
/// (`layout-is-not-a-word`).
const CAUSE_QUOTE: &str = "why-cause-quote";

/// The 1-based `(line, column)` of a byte offset in `src` — the ack-8 `file:line:col` regions'
/// coordinate primitive, feeding **rul24-lineno-identity**: the ONE line-number space is the
/// SOURCE file's, so every printed `N |` gutter and every accepted `book.sh:N` address resolve
/// through this same function. A byte past the end clamps to the source end. Columns count BYTES
/// within the line (1-based) — a shell script is overwhelmingly ASCII, so byte-columns align the
/// fixed-width caret art; a multi-byte line mis-aligns the caret cosmetically only (never the
/// line number, which is the load-bearing identity). Pure; never panics (`inv-no-throw`).
/// (Saturating arithmetic throughout — the offsets are source-file-bounded, so it never wraps,
/// and clippy's `arithmetic_side_effects` floor stays satisfied without a bespoke `#[expect]`.)
#[must_use]
pub fn line_col(src: &str, byte: usize) -> (usize, usize) {
    let clamped = byte.min(src.len());
    let mut line = 1usize;
    let mut line_start = 0usize;
    for (i, b) in src.bytes().enumerate().take(clamped) {
        if b == b'\n' {
            line = line.saturating_add(1);
            line_start = i.saturating_add(1);
        }
    }
    (line, clamped.saturating_sub(line_start).saturating_add(1))
}

/// The under-span mark for a caret frame. A primary SPAN (a byte region, `run >= 2`) renders the
/// bracket form `\`+`_`…+`/` — it reads as "this whole extent", not "this point" (the `e6edf5e`
/// style). A single-column primary keeps a `^` (the door item-1 left open for a lexeme point); a
/// secondary span keeps its `-` underline. Pure; total.
fn span_underline(run: usize, primary: bool) -> String {
    if !primary {
        return "-".repeat(run);
    }
    if run < 2 {
        return "^".repeat(run);
    }
    let mut mark = String::with_capacity(run);
    mark.push('\\');
    mark.extend(core::iter::repeat_n('_', run.saturating_sub(2)));
    mark.push('/');
    mark
}

/// A caret frame for ONE span (ack-8, the diagnostics frame), as a laid-out code block: a
/// `file:line:col` locus, the source line in a gutter, and an underline (`\`…`/` primary span /
/// `-` secondary) beneath the span with an optional label.
///
/// Feeds **rul24-lineno-identity**: the gutter IS the SOURCE line (via [`line_col`]), so a number
/// the user reads here is the number they type back as `:N`. The gutter's WIDTH and the bar's
/// column are the layout engine's — a gutter is a lead folded into the block's first stop, not a
/// field this seat pads. A multi-line span renders EVERY source line it covers, each with its own
/// underline (the rustc continuation shape). `inv-referent-agnostic`: the source line is shown,
/// never decoded — it enters as somebody else's bytes and is encoded at that mint, which is what
/// keeps a control byte from corrupting both the geometry and the terminal.
fn frame_block(
    span: Span,
    src: &str,
    filename: &str,
    label: Option<&str>,
    primary: bool,
) -> weft::CodeBlock<crate::weave::Face> {
    use crate::weave::{RENDER_SOURCE_CAP, RENDER_VALUE_CAP, foreign, mark, value};
    use weft::{CodeCell, CodeLine, Literalness};

    let (line, col) = line_col(src, span.lo.0 as usize);
    let (hi_line, hi_col) = line_col(src, span.hi.0 as usize);
    let lines: Vec<&str> = src.lines().collect();
    let mut block_lines = Vec::new();
    for l in line..=hi_line {
        let line_text = lines.get(l.saturating_sub(1)).copied().unwrap_or("");
        // This line's slice of the span: from the start column on the FIRST line (0 on
        // continuation lines) to the end column on the LAST line (end-of-line on earlier lines).
        let start = if l == line { col.saturating_sub(1) } else { 0 };
        let end = if l == hi_line {
            hi_col.saturating_sub(1)
        } else {
            line_text.len()
        };
        block_lines.push(CodeLine {
            gutter: Some(value(l.to_string(), "cli-frame-gutter", RENDER_VALUE_CAP)),
            cells: vec![CodeCell::new(vec![foreign(
                &ForeignBytes::from_io_edge(line_text),
                filename.to_owned(),
                RENDER_SOURCE_CAP,
            )])],
        });
        let mut underline = vec![mark(
            format!(
                "{}{}",
                " ".repeat(encoded_width(line_text, 0, start)),
                span_underline(encoded_width(line_text, start, end).max(1), primary),
            ),
            "cli-frame-underline",
        )];
        if let Some(label) = label {
            underline.push(value(
                format!(" {label}"),
                "cli-frame-label",
                RENDER_VALUE_CAP,
            ));
        }
        block_lines.push(CodeLine {
            gutter: None,
            cells: vec![CodeCell::new(underline)],
        });
    }
    weft::CodeBlock {
        table: None,
        mode: Literalness::Literal,
        locus: Some(vec![value(
            format!("{filename}:{line}:{col}"),
            "cli-frame-locus",
            RENDER_VALUE_CAP,
        )]),
        lines: block_lines,
    }
}

/// How many COLUMNS `line[from..to]` occupies once encoded for display.
///
/// The span's coordinates are byte offsets into the author's own line, but the line reaches the
/// reader escaped, so an underline placed at a raw offset would drift the moment a line carried a
/// byte the encoder widened. Byte offsets that do not land on a character boundary fall back to
/// the whole line rather than panicking (`inv-no-throw`).
fn encoded_width(line: &str, from: usize, to: usize) -> usize {
    let slice = line.get(from..to).unwrap_or("");
    crate::display::encode_foreign(slice, usize::MAX).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{BytePos, Interner, LeafId};

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(BytePos(lo), BytePos(hi))
    }

    fn site(n: u32) -> SiteId {
        SiteId::leaf(LeafId(n))
    }

    /// Whitespace-collapsed, so an assertion about WORDS is blind to the wrap the seat chose.
    fn flattened(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// One register's committed words, asked of the registry rather than copied out of it.
    ///
    /// Every catalog register below is case-owned, so its bytes are edited through the loom flow
    /// (`mise run loom:compile`/`loom:promote`) by someone who will not be reading this file. A
    /// literal here turns their prose edit into a `dorc-aid` unit failure with no pointer to the
    /// flow that caused it, which is exactly what `render-form-unwelded` forbids; asking the
    /// registry keeps the structural claim (this seat renders THAT register) and drops the byte
    /// claim.
    fn register_words(slug: &str) -> String {
        flattened(
            crate::catalog::entry(slug)
                .and_then(|entry| entry.message)
                .unwrap_or_else(|| panic!("`{slug}` has no written message register")),
        )
    }

    /// A one-site [`SiteUnresolvable`] whose named sites and excerpt are `names` (test helper —
    /// the callers below care about the code, not about which command it disclosed).
    fn unresolvable(names: &str) -> SiteUnresolvable {
        SiteUnresolvable {
            site: site(0),
            count: "1".to_owned(),
            site_word: "site",
            names: ForeignBytes::from_io_edge(names),
            excerpt: ForeignBytes::from_io_edge(names),
        }
    }

    /// A [`CmdsubOperandTop`] payload with the sweep's `top_cause` field (test helper).
    fn cmdsub_top(pos: OperandPosition, cause: Option<ProvId>) -> CmdsubOperandTop {
        CmdsubOperandTop {
            site: site(0),
            position: pos,
            cause,
            top_cause: TopCause::UnmodeledExpansion,
            command: CommandName::Literal("apt-get".to_owned()),
        }
    }

    /// The mandatory primary span is structural for the ORDINARY path (`21Z` drop-A/drop-B):
    /// [`Diag::new`] always carries a real span. (Span-lessness is reachable ONLY via the gated
    /// [`Diag::new_spanless_site`]; see `spanless_site_renders_without_region`.)
    #[test]
    fn primary_span_is_mandatory_on_the_ordinary_path() {
        let d = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: site(3),
                verb: "elide",
                command: "cat <<EOF".to_owned(),
            }),
            span(10, 20),
        );
        assert_eq!(d.primary.span(), Some(span(10, 20)));
    }

    /// The second-class spanless mint (arch-3-residual-2): [`Diag::new_spanless_site`] produces a
    /// primary whose span is `None`; `render_cli` renders the title (catalog message) but OMITS the
    /// region (no location to point at) and never panics (`inv-no-throw`).
    #[test]
    fn spanless_site_renders_without_region() {
        let d = Diag::new_spanless_site(DiagCode::CfgErexitUnknown(CfgErexitUnknown));
        assert_eq!(
            d.primary.span(),
            None,
            "the spanless primary carries no span"
        );
        let i = Interner::default();
        let cli = render_cli(&d, "irrelevant source", "book.sh", &i);
        assert!(
            !cli.contains("book.sh:"),
            "no source frame when spanless: {cli}"
        );
        assert!(
            cli.starts_with("warning[cfg-errexit-unknown]: "),
            "title renders: {cli}"
        );
        // This code takes no params, so whatever the register holds IS the rendered body.
        assert!(
            flattened(&cli).contains(&register_words("cfg-errexit-unknown")),
            "the register's own words, not a payload passthrough: {cli}"
        );
    }

    /// Severity comes ONLY from the registry (`crib-4`): there is no severity constructor, and a
    /// Note-class and an Error-class code resolve their declared severities.
    #[test]
    fn severity_is_registry_data_not_a_constructor() {
        let note = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                ..unresolvable("x")
            }),
            span(0, 1),
        );
        assert_eq!(note.severity(), Severity::Note);
        let err = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: site(0),
                verb: "elide",
                command: "x".to_owned(),
            }),
            span(0, 1),
        );
        assert_eq!(err.severity(), Severity::Error);
    }

    /// The floor column (`22B-fork-floor-membership`): the render-refusal pins
    /// [`Floor::WarnOrDeny`]; the disclosures are floorless.
    #[test]
    fn floor_column() {
        let refused = DiagCode::RenderHeredocRefused(RenderHeredocRefused {
            site: site(0),
            verb: "elide",
            command: "x".to_owned(),
        });
        assert_eq!(registry(&refused).floor, Floor::WarnOrDeny);
        let unresolvable = DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: site(0),
            ..unresolvable("x")
        });
        assert_eq!(registry(&unresolvable).floor, Floor::None);
    }

    /// The unwritten placeholder is a RENDER mechanic, so it is stated over a synthesized register
    /// rather than over whichever committed code still has no words.
    ///
    /// Pointing it at a real slug made the test a second, invisible owner of that code's prose:
    /// authoring the first sentence for it — the whole point of the burn-down — turned this into a
    /// red unit test in a crate the author never opens (`prose-three-state` says `[unwritten:]` is
    /// a legal RESTING state, never a pinned one).
    #[test]
    fn an_unwritten_register_renders_the_greppable_placeholder() {
        let catalog = vec![crate::catalog::OwnedEntry {
            slug: "transport-crlf-refused".to_owned(),
            when_fires: String::new(),
            why: String::new(),
            message: None,
            help: crate::catalog::HelpRegister::Absent,
            params: Vec::new(),
        }];
        let arrangements = crate::arrangement::owned_arrangements();
        let diag = Diag::new_spanless_site(DiagCode::TransportCrlfRefused(TransportCrlfRefused {
            which: "book.sh".to_owned(),
            line: "3".to_owned(),
        }));
        assert_eq!(
            render_body_with(
                &RenderCtx::new(&catalog, &arrangements),
                &diag,
                &Interner::default()
            ),
            "[unwritten: transport-crlf-refused]"
        );
    }

    /// The transport family renders SOMETHING, always: a complete body, no unfilled hole, and no
    /// panic on the spanless mint (`inv-no-throw`). Whether any given register has words yet is the
    /// corpus-wide gate's business, not this seat's.
    #[test]
    fn the_transport_family_renders_completely_and_never_panics() {
        let codes = [
            DiagCode::TransportCrlfRefused(TransportCrlfRefused {
                which: "book.sh".to_owned(),
                line: "3".to_owned(),
            }),
            DiagCode::TransportSessionLost(TransportSessionLost {
                host: "web1".to_owned(),
                attempts: "3".to_owned(),
                diagnosis: "timed out after 120s".to_owned(),
            }),
            DiagCode::TransportSpawnRefused(TransportSpawnRefused {
                host: "web1".to_owned(),
                detail: ForeignBytes::from_os_error(&std::io::Error::from(
                    std::io::ErrorKind::NotFound,
                )),
            }),
            DiagCode::TransportMarkerUnusable(TransportMarkerUnusable {
                host: "web1".to_owned(),
            }),
            DiagCode::TransportApplyFailed(TransportApplyFailed {
                host: "web1".to_owned(),
                status: "2".to_owned(),
            }),
        ];
        for code in codes {
            let slug = code.slug();
            assert_eq!(registry(&code).severity, Severity::Error);
            let body = render_body(&Diag::new_spanless_site(code), &Interner::default());
            assert!(!body.trim().is_empty(), "`{slug}` rendered nothing");
            assert!(
                !body.contains("{{"),
                "`{slug}` left a hole unfilled: {body}"
            );
        }
    }

    /// The admission refusal's REGISTRY row, which is the load-bearing half: it is Error-severity,
    /// floor-Pinned, structurally-remediated, and payload-free, so no hole can leak a managed
    /// host's bytes into it. What words it carries is the prose loop's, not this test's.
    #[test]
    fn host_evidence_admission_refusal_is_pinned_and_payload_free() {
        let code = DiagCode::HostEvidenceAdmissionRefused(HostEvidenceAdmissionRefused {
            kind: HostEvidenceRefusalKind::Framing,
        });
        assert_eq!(code.slug(), "host-evidence-admission-refused");
        assert_eq!(registry(&code).severity, Severity::Error);
        assert_eq!(registry(&code).floor, Floor::Pinned);
        assert_eq!(registry(&code).remediation, RemediationClass::Structural);
        assert!(params_of(&RenderCtx::production(), &code, &Interner::default()).is_empty());
        let entry = crate::catalog::entry(code.slug()).expect("catalog entry");
        assert!(entry.params.is_empty());
        let body = render_body(&Diag::new_spanless_site(code), &Interner::default());
        assert!(!body.trim().is_empty() && !body.contains("{{"), "{body}");
    }

    /// The gate-3 interaction: the two ⊤-disclosures stay `Note` (they must never silently become
    /// Error and trip the e2e stderr error-floor undeclared), and the render-refusal is `Error`.
    #[test]
    fn gate3_floor_note_codes_stay_note_error_code_is_declared() {
        for code in [
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::CommandWord, None)),
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                ..unresolvable("x")
            }),
        ] {
            assert_eq!(
                registry(&code).severity,
                Severity::Note,
                "a disclosure code must stay Note or it trips the gate-3 error-floor undeclared: {}",
                code.slug()
            );
        }
        let refused = DiagCode::RenderHeredocRefused(RenderHeredocRefused {
            site: site(0),
            verb: "elide",
            command: "x".to_owned(),
        });
        assert_eq!(registry(&refused).severity, Severity::Error);
    }

    /// The render partition (`render-3`, the ru-12 weld): the artifact comment is fact-plane
    /// (`Some` only for the fact-relevant refusal, `None` for render-plane disclosures); `render_cli`
    /// carries the filled catalog message (the `.label` message-path is retired).
    #[test]
    fn render_partition_artifact_is_fact_plane() {
        let i = Interner::default();
        let note = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::Operand(1), None)),
            span(0, 4),
        );
        assert_eq!(
            render_artifact_comment(&note),
            None,
            "a disclosure contributes no fact-plane artifact comment"
        );
        // The CLI render carries the CATALOG message (filled from the payload), not a `.label`.
        // The run past the register's last hole is the longest stretch a filled render reproduces
        // verbatim, so it witnesses THAT register without pinning its words.
        let cli = render_cli(&note, "echo TAIL", "book.sh", &i);
        let tail = register_words("cmdsub-operand-top");
        let tail = flattened(tail.rsplit("}}").next().unwrap_or(&tail));
        assert!(flattened(&cli).contains(&tail), "catalog message: {cli}");
        assert!(cli.contains("operand 1 is"), "position param filled: {cli}");
        assert!(
            cli.starts_with("note[cmdsub-operand-top]: "),
            "title is severity-keyed: {cli}"
        );
        // An Error refusal: a fact-plane comment naming the site, no prose.
        let refused = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: site(7),
                verb: "elide",
                command: "cat <<EOF".to_owned(),
            }),
            span(0, 4),
        );
        let comment = render_artifact_comment(&refused).expect("a refusal is artifact-relevant");
        assert!(comment.starts_with('#'), "a comment: {comment}");
        assert!(
            comment.contains("site 7"),
            "names the fact-plane site: {comment}"
        );
        // The literal run before the register's first hole; none of the help is artifact-plane.
        let help = crate::catalog::entry("render-heredoc-refused")
            .and_then(|entry| entry.help.written().copied())
            .and_then(|template| template.split("{{").next())
            .expect("the help register is written");
        assert!(
            !comment.contains(help),
            "the help (exempt-plane) must NOT reach the artifact: {comment}"
        );
    }

    /// The message/help composition the render owes a payload: the REGISTER owns the sentence, the
    /// payload's sealed values fill its holes, and the help register follows on its own
    /// `  = help: ` continuation line.
    ///
    /// Driven through a SYNTHESIZED catalog, not the committed one. Pinning the real registers'
    /// bytes made this test the second owner of two case-owned sentences, so the sanctioned prose
    /// loop broke it from a crate its author never opens — `render-form-unwelded`'s own failure
    /// mode. The composition is what this seat is responsible for, and a two-row fixture states it
    /// exactly, holes and all. (Whether the COMMITTED registers still carry their `sm ` migration
    /// prefix is `message_registers_are_sm_or_unwritten`'s corpus-wide job, keyed to
    /// `is_case_owned` — never a byte copy here.)
    #[test]
    fn a_register_owns_the_sentence_and_the_payload_fills_its_holes() {
        let i = Interner::default();
        let catalog = vec![
            crate::catalog::OwnedEntry {
                slug: "site-unresolvable".to_owned(),
                when_fires: String::new(),
                why: String::new(),
                message: Some("harness {{count}} {{site_word}} <{{names}}>".to_owned()),
                help: crate::catalog::HelpRegister::Written(
                    "harness help <{{excerpt}}>".to_owned(),
                ),
                params: Vec::new(),
            },
            crate::catalog::OwnedEntry {
                slug: "render-heredoc-refused".to_owned(),
                when_fires: String::new(),
                why: String::new(),
                message: Some("harness {{verb}} (`{{command}}`)".to_owned()),
                help: crate::catalog::HelpRegister::Absent,
                params: Vec::new(),
            },
        ];
        let arrangements = crate::arrangement::owned_arrangements();
        let ctx = RenderCtx::new(&catalog, &arrangements);

        let pass = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                ..unresolvable("3 sites run unprobed")
            }),
            span(0, 1),
        );
        assert_eq!(
            render_body_with(&ctx, &pass, &i),
            concat!(
                "harness 1 site <3 sites run unprobed>\n",
                "  = help: harness help <3 sites run unprobed>"
            ),
            "the register owns the sentence; the sealed values fill its holes"
        );
        let tmpl = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: site(0),
                verb: "guard",
                command: "cat <<EOF".to_owned(),
            }),
            span(0, 1),
        );
        assert_eq!(
            render_body_with(&ctx, &tmpl, &i),
            "harness guard (`cat <<EOF`)",
            "an Absent help register adds no continuation line"
        );
    }

    /// The OOB projection (`render-2`) is fact-plane: site + slug + severity, no prose. The slug
    /// is the stable wire token (dq- dropped by the sweep).
    #[test]
    fn oob_projection_is_fact_plane_slug_keyed() {
        let d = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: SiteId {
                    leaf: LeafId(4),
                    member: Some(2),
                },
                ..unresolvable("make install")
            }),
            span(0, 12),
        );
        let p = project_oob(&d);
        assert_eq!(
            p.code, "site-unresolvable",
            "stable wire slug (dq- dropped)"
        );
        let site = p.site.expect("SiteUnresolvable always has a site");
        assert_eq!(site.leaf, LeafId(4));
        assert_eq!(site.member, Some(2));
        assert_eq!(p.severity, Severity::Note);
    }

    /// The builder chains the GOOD shape (`type-sketch-7`): a secondary cause-span, a note, and a
    /// remediation-classed suggestion — the message rides the catalog, the extras chain on.
    #[test]
    fn builder_assembles_the_good_shape() {
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::CommandWord, None)),
            span(0, 5),
        )
        .secondary(span(10, 20), "and so this command cannot be elided")
        .note("downstream commands run unconditionally")
        .suggest(Suggestion {
            message: "declare the kind's selector in its oracle".to_owned(),
            applicability: Applicability::MaybeIncorrect,
            remediation: RemediationClass::ProvideModel,
        });
        assert_eq!(d.secondary.len(), 1);
        assert_eq!(d.children.len(), 1);
        assert!(d.suggestion.is_some());
        let cli = render_cli(&d, "01234_56789poisoned_", "book.sh", &Interner::default());
        let tail = register_words("cmdsub-operand-top");
        let tail = flattened(tail.rsplit("}}").next().unwrap_or(&tail));
        assert!(flattened(&cli).contains(&tail), "{cli}");
        assert!(cli.contains("cannot be elided"), "{cli}");
        assert!(cli.contains("[provide-model]"), "{cli}");
    }

    /// ack-8 `line_col`: the SOURCE-file line-number space (rul24-lineno-identity). 1-based line
    /// and byte-column; a byte past end clamps to the last line; the first byte of a line is col 1.
    #[test]
    fn line_col_is_source_truth_one_based() {
        let src = "aa\nbbb\nc";
        assert_eq!(line_col(src, 0), (1, 1), "first byte ⇒ 1:1");
        assert_eq!(line_col(src, 1), (1, 2));
        assert_eq!(
            line_col(src, 3),
            (2, 1),
            "first byte after the newline ⇒ line 2, col 1"
        );
        assert_eq!(
            line_col(src, 7),
            (3, 1),
            "the 'c' line (byte 6 is line 2's trailing newline)"
        );
        assert_eq!(
            line_col(src, 999),
            (3, 2),
            "past-end clamps to the last line"
        );
    }

    /// One frame's rendered bytes, laid out on its own.
    fn framed(span: Span, src: &str) -> String {
        let document = weft::Document::new(vec![weft::Node::new(weft::NodeKind::Code(
            frame_block(span, src, "book.sh", None, true),
        ))]);
        weft::render(&document, crate::CANONICAL_TRANSCRIPT_WIDTH)
            .text()
            .to_owned()
    }

    /// ack-8 the caret frame: the block underlines the exact span on its source line, in a gutter
    /// whose number IS the SOURCE line (rul24-lineno-identity). The span's start column places the
    /// underline. Pins the flagship shape (a diagnostic points at the exact bytes it means).
    #[test]
    fn a_frame_underlines_the_span_on_its_source_line() {
        let src = "set -eu\napt-get install $(date)\n";
        // `$(date)` on line 2 (after "apt-get install "); a 7-byte span.
        let lo = u32::try_from(src.find("$(date)").unwrap()).unwrap();
        let hi = lo + u32::try_from("$(date)".len()).unwrap();
        let frame = framed(Span::new(BytePos(lo), BytePos(hi)), src);
        assert!(
            frame.contains("book.sh:2:17"),
            "file:line:col locator: {frame}"
        );
        assert!(
            frame.contains("2 | apt-get install $(date)"),
            "the source line in a gutter: {frame}"
        );
        assert!(
            frame.contains("\\_____/"),
            "a 7-wide span bracket under `$(date)`: {frame}"
        );
    }

    /// ack-8 the MULTI-LINE caret frame (`aid-caret-span-precision`): a span crossing lines renders
    /// EVERY covered source line, each in its own gutter with a per-line underline (the first line
    /// from its start column, continuation lines from column 0, the last line to its end column).
    /// The gutters right-align on the widest number, so the bar column is one column for the whole
    /// block. Here the span covers lines 9–10, so ` 9` aligns under `10`.
    #[test]
    fn a_frame_renders_every_line_of_a_multiline_span() {
        let src = "a\nb\nc\nd\ne\nf\ng\nh\niii\njjj\n";
        let lo = u32::try_from(src.find("iii").unwrap()).unwrap();
        let hi = u32::try_from(src.find("jjj").unwrap() + "jjj".len()).unwrap();
        let frame = framed(Span::new(BytePos(lo), BytePos(hi)), src);
        assert!(
            frame.contains("book.sh:9:1"),
            "locator on the first line: {frame}"
        );
        assert!(
            frame.contains("\n 9 | iii"),
            "line 9 right-aligned in the block's gutter: {frame}"
        );
        assert!(
            frame.contains("\n10 | jjj"),
            "line 10 in the same gutter: {frame}"
        );
        assert_eq!(
            frame.matches("\n   | \\_/").count(),
            2,
            "a per-line `\\_/` span bracket beneath BOTH covered lines (not just the first): {frame}"
        );
    }

    /// A source line reaches a laid-out surface ENCODED, so the underline has to be placed in the
    /// columns the reader actually sees. A raw-byte offset would drift left of the span the moment
    /// a line carried a tab.
    #[test]
    fn an_underline_lands_under_the_span_even_when_the_line_widens_on_encoding() {
        let src = "a\tb cmd\n";
        let lo = u32::try_from(src.find("cmd").unwrap()).unwrap();
        let hi = lo + 3;
        let frame = framed(Span::new(BytePos(lo), BytePos(hi)), src);
        let line = frame
            .lines()
            .find(|line| line.contains("a\\x09b"))
            .expect("the source line is escaped");
        let underline = frame
            .lines()
            .find(|line| line.contains("\\_/"))
            .expect("the span is underlined");
        assert_eq!(
            line.find("cmd"),
            underline.find("\\_/"),
            "the bracket sits under `cmd`:\n{frame}"
        );
    }

    /// ack-8 the MULTI-span model (228, the built-but-unwired `render_cli` machinery now realized):
    /// a diagnostic with a PRIMARY caret and a LABELED SECONDARY caret renders BOTH in one frame —
    /// cause+effect together. The primary underlines with `^`, the secondary with `-` and carries
    /// its label (the flagship being pipeline-stage precision: `a | b || c` underlining the exact
    /// opaque stage). This pins that the secondary caret + its label reach the render.
    #[test]
    fn render_cli_renders_primary_and_labeled_secondary_carets() {
        let src = "run_stage_a | grep -q x || install\n";
        let a_lo = 0u32;
        let a_hi = u32::try_from("run_stage_a".len()).unwrap();
        let stage_lo = u32::try_from(src.find("grep -q x").unwrap()).unwrap();
        let stage_hi = stage_lo + u32::try_from("grep -q x".len()).unwrap();
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::CommandWord, None)),
            Span::new(BytePos(a_lo), BytePos(a_hi)),
        )
        .secondary(
            Span::new(BytePos(stage_lo), BytePos(stage_hi)),
            "this stage is the opaque one",
        );
        let cli = render_cli(&d, src, "book.sh", &Interner::default());
        assert!(
            cli.contains("\\_________/"),
            "primary span bracket under `run_stage_a`: {cli}"
        );
        assert!(
            cli.contains("---------"),
            "secondary `-` caret under the opaque stage: {cli}"
        );
        assert!(
            cli.contains("this stage is the opaque one"),
            "the secondary label reaches the frame: {cli}"
        );
        // Both spans are on line 1 ⇒ both locators name line 1 (one SOURCE line-number space).
        assert_eq!(
            cli.matches("book.sh:1:").count(),
            2,
            "both carets locate on line 1: {cli}"
        );
    }

    /// `OperandPosition::describe` matches the legacy prose the migrated emit site produced (so
    /// the disclosure text is stable across the migration).
    #[test]
    fn operand_position_describe_matches_legacy_prose() {
        assert_eq!(
            OperandPosition::CommandWord.describe(&RenderCtx::production()),
            "the command word"
        );
        assert_eq!(
            OperandPosition::Operand(2).describe(&RenderCtx::production()),
            "operand 2"
        );
    }

    /// The grouping keys (`type-sketch-5`): fine keys on (slug, site); coarse STUBS to fine
    /// (`22B-fork-scope-key` — degenerate coarse=fine, no collapse yet).
    #[test]
    fn grouping_keys_fine_and_stubbed_coarse() {
        let d = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: site(5),
                verb: "elide",
                command: "x".to_owned(),
            }),
            span(0, 4),
        );
        let fine = d.fine_key();
        assert_eq!(fine.code, "render-heredoc-refused");
        assert_eq!(fine.site, Some(site(5)));
        // STUB: the coarse key wraps the fine key unchanged this round.
        assert_eq!(d.coarse_key().fine, fine);
    }

    /// STAGE-2 the why-lens (`22D` §1): `why` reads a [`CmdsubOperandTop`]'s wired ⊤-cause + the
    /// arena and renders the cause-derived "ran because … ⊤ originated at <site>; <remediation>"
    /// line. The cause-site is resolved from the arena origin's span (shown once, minimal-witness).
    /// Pins that the why-lens consumes the real receipt (a [`dorc_core::OriginKind::TopCause`] origin at
    /// a known span) and surfaces the position + remediation hint. The first real receipt-READER.
    #[test]
    fn why_lens_renders_cause_derived_reason() {
        let mut arena = dorc_core::ProvArena::new();
        // The ⊤-cause origin: a give-up minted at the operand's source span (as classify mints it).
        let cause = arena.leaf(dorc_core::OriginKind::TopCause, Some(span(11, 20)));
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::Operand(1), Some(cause))),
            span(0, 20),
        );
        let src = "apt-get install $(date)";
        let exp = why(&RenderCtx::production(), &d, &arena, src)
            .expect("a caused-⊤ has a why-lens explanation");
        let reason = exp.text(&RenderCtx::production());
        assert_eq!(
            exp.remediation,
            RemediationClass::ResolveDynamism,
            "the ru-27 HOW class for a dynamic-operand forced run (registry column)"
        );
        assert!(
            reason.contains("operand 1"),
            "names the ⊤ position: {reason}"
        );
        assert!(
            reason.contains("first seen at"),
            "names the cause-site (the receipt the why-lens READS): {reason}"
        );
        assert!(
            reason.contains("to stay safe"),
            "the why-lens explains the RUN in plain English (ack-4), never licenses a skip: {reason}"
        );
        assert!(
            reason.contains("[resolve-dynamism]"),
            "the remediation hint addresses the right user (admin): {reason}"
        );
    }

    /// The reason's BYTES, whole. Its sentence now comes from four registry rows with a coordinate
    /// and a book excerpt between them, and the migration that put them there was required to move
    /// nothing (`28G` Phase W4's churn contract): substring assertions cannot see a doubled space
    /// or a lost one at a fragment boundary, and this is the surface the e2e needles read.
    #[test]
    fn a_reason_reads_exactly_as_the_hardcoded_sentence_did() {
        let mut arena = dorc_core::ProvArena::new();
        let cause = arena.leaf(dorc_core::OriginKind::TopCause, Some(span(11, 20)));
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::Operand(1), Some(cause))),
            span(0, 20),
        );
        let exp = why(
            &RenderCtx::production(),
            &d,
            &arena,
            "apt-get install $(date)",
        )
        .expect("a caused-⊤ explains");
        assert_eq!(
            exp.text(&RenderCtx::production()),
            "ran because operand 1 is a command-substitution `$(...)` or runtime-dynamic value -- \
             its value couldn't be resolved (first seen at 1:12 `tall $(da`); so dorc runs it, to \
             stay safe (when unsure, run). to skip it, make the operand a literal Dorc can \
             resolve+probe [resolve-dynamism]"
        );
        // The site-less cause: the same sentence, saying so where the locus would be. Nothing else
        // reaches this row, and an unrendered row is one nobody can tell is wrong.
        let siteless = arena.leaf(dorc_core::OriginKind::TopCause, None);
        let bare = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::CommandWord, Some(siteless))),
            span(0, 4),
        );
        assert_eq!(
            why(&RenderCtx::production(), &bare, &arena, "date")
                .expect("a caused-⊤ explains")
                .text(&RenderCtx::production()),
            "ran because the command word is a command-substitution `$(...)` or runtime-dynamic \
             value -- its value couldn't be resolved (first seen at (no source site)); so dorc \
             runs it, to stay safe (when unsure, run). to skip it, make the operand a literal \
             Dorc can resolve+probe [resolve-dynamism]"
        );
    }

    /// The cause locus arrives as PARTS, and the book's own bytes wear the not-ours class inside
    /// them — which is what makes the plan-stderr lens as safe as the weft-rendered report, since
    /// both read the same fragments (`ask-why-lens-stderr-unencoded`).
    #[test]
    fn the_book_bytes_in_a_reason_are_a_foreign_fragment_encoded_at_mint() {
        let mut arena = dorc_core::ProvArena::new();
        let cause = arena.leaf(dorc_core::OriginKind::TopCause, Some(span(0, 8)));
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::Operand(1), Some(cause))),
            span(0, 8),
        );
        let exp = why(&RenderCtx::production(), &d, &arena, "run \u{1b}[31m x")
            .expect("a caused-⊤ explains");
        let foreign: Vec<&Said> = exp
            .parts
            .iter()
            .filter(|part| matches!(part, Said::Foreign { .. }))
            .collect();
        assert_eq!(foreign.len(), 1, "the excerpt is the one not-ours fragment");
        assert_eq!(
            foreign[0].text(&RenderCtx::production()),
            "run \\x1b[31",
            "the escape is already encoded in the fragment, not at some later seat"
        );
        assert!(exp.text(&RenderCtx::production()).is_ascii());
    }

    /// STAGE-2 honesty (fd-G): the why-lens covers CAUSED ⊤s ONLY. A code with no cause field
    /// (every code but [`CmdsubOperandTop`] at HEAD) ⇒ `why` returns `None` (it does NOT overclaim
    /// a why for a give-up that carries none). A [`CmdsubOperandTop`] with `cause: None` (the
    /// stage-1 hard-None that should no longer occur, but the type permits) ⇒ also `None`.
    #[test]
    fn why_lens_returns_none_without_a_caused_top() {
        let arena = dorc_core::ProvArena::new();
        let src = "irrelevant";
        // A cause-less code: SiteUnresolvable has no cause field ⇒ the why-lens explains nothing.
        let unresolvable = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                ..unresolvable("make install")
            }),
            span(0, 12),
        );
        assert!(
            why(&RenderCtx::production(), &unresolvable, &arena, src).is_none(),
            "a code with no ⊤-cause must NOT get a fabricated why (fd-G honesty)"
        );
        // A CmdsubOperandTop with cause: None ⇒ no fabrication either.
        let causeless_top = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::CommandWord, None)),
            span(0, 5),
        );
        assert!(
            why(&RenderCtx::production(), &causeless_top, &arena, src).is_none(),
            "a cause: None CmdsubOperandTop yields no why (no fabrication)"
        );
    }

    /// STAGE-3/4 the rec-1 two-surfaces WELD at the type level: a caused-⊤ has a why-lens
    /// explanation (the RENDER surface) BUT no artifact comment (the byte-floored `.sh` surface).
    /// The why-lens lives ONLY on the render plane; the cause/receipt never reaches the artifact.
    /// This is the partition the cli's stage-3 render relies on — the artifact stays receipt-free.
    #[test]
    fn why_lens_is_render_plane_artifact_is_receipt_free() {
        let mut arena = dorc_core::ProvArena::new();
        let cause = arena.leaf(dorc_core::OriginKind::TopCause, Some(span(11, 20)));
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(cmdsub_top(OperandPosition::Operand(1), Some(cause))),
            span(0, 20),
        );
        // RENDER surface: the why-lens explains it.
        assert!(
            why(
                &RenderCtx::production(),
                &d,
                &arena,
                "apt-get install $(date)"
            )
            .is_some(),
            "a caused-⊤ has a why-lens explanation (the render surface)"
        );
        // ARTIFACT surface: NO fact-plane comment for a CmdsubOperandTop ⇒ the cause/receipt
        // never reaches the byte-floored `.sh` (rec-1 — the artifact stays receipt-free).
        assert_eq!(
            render_artifact_comment(&d),
            None,
            "the why-lens (cause/receipt) must NEVER reach the artifact (rec-1 two-surfaces weld)"
        );
    }

    /// A passthrough value is somebody else's bytes reaching a terminal, so it wears the
    /// not-ours class AND arrives already encoded. Both renders read one seat
    /// ([`params_of`]), so the string form and the parts form can never disagree about what a
    /// reader is shown. The class is the VALUE's type now, so this code's two book-derived
    /// values are both not-ours and its counts and structure words are not.
    #[test]
    fn a_passthrough_value_reaches_both_renders_already_encoded() {
        let hostile = "make install \u{1b}[31m \u{202e}llatsni ekam\u{202c}\u{7}";
        let diag = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                ..unresolvable(hostile)
            }),
            span(0, 5),
        );
        let interner = Interner::default();
        let parts = render_body_parts(&RenderCtx::production(), &diag, &interner);
        let mut foreign = 0_usize;
        for part in parts.parts() {
            let text = part.text();
            if matches!(part, crate::tagged::RenderPart::ForeignText { .. }) {
                foreign = foreign.saturating_add(1);
                assert_eq!(
                    crate::display::encode_line(text, FOREIGN_PARAM_CAP),
                    text,
                    "a passthrough value reached the render un-encoded: {text:?}"
                );
                assert!(!text.trim().is_empty(), "the value must not be dropped");
            }
            for c in text.chars() {
                assert!(
                    c == '\n' || crate::display::is_display_safe(c),
                    "{c:?} reached a diagnostic render: {text:?}"
                );
            }
        }
        assert_eq!(
            foreign, 2,
            "the named sites and the quoted excerpt are the render's not-ours runs"
        );
        assert_eq!(
            render_body(&diag, &interner),
            parts.text(),
            "the two renders must agree byte for byte, encoding included"
        );
    }
}
