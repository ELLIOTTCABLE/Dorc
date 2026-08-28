//! The diagnostic-catalog **tidy gate** — the cheap structural half of `22A` concl-7 /
//! `226` §1 (rustc's `tidy error_codes.rs`), the half the Rust type system cannot see.
//!
//! The exhaustive [`dorc_aid::diag::DiagCode`] enum already buys "every variant HANDLED" for
//! free: the `registry`, every render arm, and `slug` are exhaustive `match`es that will not
//! compile with a variant missing. This test covers the THREE things the compiler can't:
//!
//! 1. **bidirectional reachability** (`226` §1) — every catalog variant is CONSTRUCTED at some
//!    emit site (the type system never forces a `pub enum` variant to be used), AND every
//!    structured construction site names a real catalog variant (no orphan emit). A variant
//!    with a registry row but no emit site is dead catalog; an emit with no row cannot exist
//!    (it would not compile), so this direction is a belt-and-braces grep.
//! 2. **a git-diff retire-guard** (`226` §1, the `error_codes.rs` deletion guard) — a catalog
//!    slug removed from `diag.rs` without being recorded on `RETIRED_SLUGS` is a SILENT variant
//!    deletion (a code that quietly stopped existing). Caught by diffing the committed `diag.rs`
//!    against the working tree for removed `slug` arms. Best-effort: skipped (not failed) when
//!    git is unavailable, so the gate never blocks a non-git checkout.
//! 3. **the catalog COMPLETENESS bijection** (`27V:rul-kill-legacy-diagnostic`) — every `DiagCode`
//!    variant has EXACTLY ONE `catalog::CatalogEntry`, and every entry names a real variant. The
//!    legacy string-slug scan is retired with the legacy `Diagnostic` mechanism (no `DiagCode("…")`
//!    form remains); the bijection is what now keeps a code from rendering `[unwritten:]`.
//!
//! It NEVER touches message prose (`crib-7` / `refuse-5`): it polices registration and
//! reachability, never quality. The scan is a plain lexical pass over the workspace's own crate
//! sources (a structural advantage — Dorc's give-up sites are nameable source points, `22A`
//! concl-7), not a build-graph automaton.

// A grep-the-source tidy gate is inherently test-harness code: it indexes into byte slices it
// just located, does index arithmetic on `find` offsets, and `expect`s the manifest layout (a
// missing crate dir IS a harness bug worth a loud panic). The workspace no-panic/no-indexing
// lints target untrusted-INPUT paths; this code's "input" is the repo's own source tree.
#![expect(
    clippy::expect_used,
    clippy::arithmetic_side_effects,
    reason = "tidy-gate harness over the repo's own source: index arithmetic on located \
              byte-offsets and an expect on the known crate layout are correct here (the no-panic \
              lints guard untrusted-input paths, not a build-time source scanner)"
)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The mint recipe, named VERBATIM so a red gate hands the reader the command that repairs it
/// (`288:rul-loom-mint-guarantee`). The lock is `@generated` and hand-rows are refused, so the ONLY
/// repair is a defining case plus a publish.
const REPAIR_HINT: &str = "Mint its prose home: `dorc-loom scaffold <slug>`, author the case's \
                           when-fires/why and a replay whose output carries the slug, then have \
                           the orchestrator run `dorc-loom publish <case>`. The lock is generated \
                           — a hand-written row is refused.";

/// Every migrated catalog variant's PAYLOAD-struct name — the spine's construction marker. Each
/// variant wraps a uniquely-named payload struct that is constructed ONLY at an emit site (the
/// `DiagCode::Variant(Payload { … })` form), so grepping the struct literal is robust to the
/// `DiagCode`-vs-`Code`-alias the emit crates use. KEEP IN SYNC with [`dorc_aid::diag::DiagCode`]
/// — a new variant adds one entry here (the same one-edit friction the catalog promises).
const MIGRATED_PAYLOADS: &[&str] = &[
    "CmdsubOperandTop",
    "SiteUnresolvable",
    "RenderHeredocRefused",
    "ArtifactFormRefused",
    "ArtifactFormFallback",
    "ArtifactPublishRefused",
    "PlanImportRewritten",
    "CmdsubInnerNonleaf",
    "RedirTargetTop",
    "Depth2PositionalUnthreaded",
    // syntax/parser.rs
    "SyntaxUnsupported",
    "SyntaxMalformed",
    // analysis/cfg.rs
    "CfgTopNode",
    "CfgErexitUnknown",
    "CfgInlineRefused",
    "CfgBuiltinShadowed",
    // analysis/effect.rs
    "EffectKindDisagreement",
    "SolverConsistencyFailure",
    // plan/lib.rs (the sparing re-derivation seat)
    "SurvivalRederivationDisagreement",
    // oracle/predict.rs
    "PredictOutOfDialect",
    "PredictUnterminated",
    // oracle/reserved.rs
    "MungeNameInvalid",
    "MungeNameCollision",
    "ReservedNamespaceSquat",
    // oracle/marker.rs
    "MissingDialectMarker",
    "MarkerVersionUnrecognized",
    // oracle/entry.rs
    "ToleratesUnknownDimension",
    "ToleratesOverIdentityDependence",
    "HeavyContextNoTolerance",
    // oracle/wrapper.rs
    "LendMapUnknownDimension",
    // oracle/carry.rs
    "CarryNetnsOnNetKernelForbidden",
    // oracle/predict/derive.rs
    "MarkBraceVerdictSingleCell",
    // oracle/predict/mark_grammar.rs (the `281` new-grammar parse)
    "MarkUnknownVerb",
    "MarkRcArityExceeded",
    "MarkStandaloneRcConsumer",
    "MarkHashcolonMalformed",
    // plan/records.rs
    "RecordsHeaderlessRefused",
    "RecordsGluedLine",
    "RecordsHeaderMissing",
    "RecordsSentinelNonce",
    "RecordsFactTruncated",
    "RecordsIntegrityRefused",
    "RecordsTornLine",
    "RecordsAlienLine",
    "RecordsLateLine",
    "HostEvidenceAdmissionRefused",
    // cli/main.rs
    "FootprintIncoherent",
    "TouchesEscalated",
    "DerivFamilyIncomplete",
    "EscalationPolicy",
    "CarriedAcrossSubstrateAxis",
    "WrappedSiteAdoptionHint",
    "ResolverConflict",
    "ResolverProviderCollision",
    "DanglingReference",
    "ReachesConflict",
    "ReachesProviderCollision",
    "WrapperEntryIncoherent",
    "WrapperPeelIncoherent",
    // plan/whylog.rs + cli/main.rs (`dorc why --last` durable reader — `27V` Lane B)
    "WhylogVersionRefused",
    "WhylogBookDesync",
    "WhylogAbsent",
    "WhylogCorrupt",
    // cli/main.rs (aid hint) — AID-NEEDS:aid-unloaded-sibling-oracle (gap-5 / 24H ack-6)
    "AidUnloadedSiblingOracle",
    // lint — the lane-local namespace retired (`288` §5)
    "UnmodeledWallInventory",
    "VerdictTerminalPipeline",
    "AuthoredDeclineClass",
    "AuthoredDeclineClassUnreadable",
    "LintToolAbsent",
    "LintToolOutputUnparsable",
    "LintToolFailedWithoutFindings",
    // invocation errors (`288` §6) — the `dorc: {msg}` family + dorc-sh
    "CliStripNeedsPath",
    "CliStripGotAFlag",
    "CliUnknownMode",
    "CliFlagNeedsValue",
    "CliUnknownFlag",
    "CliUnknownFlagDidYouMean",
    "CliFlagValueNotRecognized",
    "CliFlagValueNotANumber",
    "CliNoBookGiven",
    "CliSeveralMainBooks",
    "CliStdinClaimedTwice",
    "CliFlagsMutuallyExclusive",
    "CliFlagRequiresMode",
    "CliModeNeedsFlag",
    "CliFileNotFound",
    "CliFilePermissionDenied",
    "CliFileUnreadable",
    "CliShimDirUnwritable",
    "LintNoLintableFiles",
    "LintFileCountDrift",
    "LintRequiredToolsMissing",
    "DorcShUsage",
    "DorcShScriptUnreadable",
    "DorcShExecFailed",
];

/// Every catalog slug (the stable wire string) — for the retire-guard and reachability. KEEP IN
/// SYNC with `DiagCode::slug`. A slug removed here without a retired-list entry is a silent
/// deletion (guard 2).
const MIGRATED_SLUGS: &[&str] = &[
    "cmdsub-operand-top",
    "site-unresolvable",
    "render-heredoc-refused",
    "artifact-form-refused",
    "artifact-form-fallback",
    "artifact-publish-refused",
    "plan-import-rewritten",
    "emitted-line-unsafe-for-paste",
    "cmdsub-inner-nonleaf",
    "redir-target-top",
    "depth-2-positional-unthreaded",
    "syntax-unsupported",
    "syntax-malformed",
    "cfg-top-node",
    "cfg-errexit-unknown",
    "cfg-inline-refused",
    "cfg-builtin-shadowed",
    "effect-kind-disagreement",
    "solver-consistency-failure",
    "solver-consistency-plan-demoted",
    "survival-rederivation-disagreement",
    "predict-out-of-dialect",
    "predict-unterminated",
    "oracle-role-fn-unlifted",
    "mark-on-and-or-list",
    "munge-name-invalid",
    "munge-name-collision",
    "reserved-namespace-squat",
    "oracle-file-not-load-inert",
    "role-family-contested",
    "role-defined-below-its-sites",
    "in-book-vocabulary-role",
    "helper-declaration-contested",
    "vouched-composition-not-present",
    "script-relative-load-dies-slashless",
    "load-carriage-withheld-under-unknown-cwd",
    "slashless-source-searches-path",
    "computed-source-operand",
    "missing-dialect-marker",
    "marker-version-unrecognized",
    "tolerates-unknown-dimension",
    "tolerates-over-identity-dependence",
    "heavy-context-no-tolerance",
    "lend-map-unknown-dimension",
    "carry-netns-on-net-kernel-forbidden",
    "mark-brace-verdict-single-cell",
    "mark-unknown-verb",
    "mark-rc-arity-exceeded",
    "mark-standalone-rc-consumer",
    "mark-hashcolon-malformed",
    "records-headerless-refused",
    "records-glued-line",
    "records-header-missing",
    "records-sentinel-nonce",
    "records-fact-truncated",
    "records-integrity-refused",
    "records-torn-line",
    "records-alien-line",
    "records-late-line",
    "host-evidence-admission-refused",
    "footprint-incoherent",
    "touches-escalated",
    "deriv-family-incomplete",
    "escalation-policy",
    "carried-across-substrate-axis",
    "wrapped-site-adoption-hint",
    "resolver-conflict",
    "resolver-provider-collision",
    "dangling-reference",
    "shared-cell-measurements-disagree",
    "reaches-conflict",
    "reaches-provider-collision",
    "wrapper-entry-incoherent",
    "wrapper-peel-incoherent",
    "whylog-version-refused",
    "whylog-book-desync",
    "whylog-absent",
    "whylog-corrupt",
    "whylog-unwritten",
    "durable-receipt-unwritten",
    "aid-unloaded-sibling-oracle",
    "oracle-matched-zero-sites",
    "unmodeled-wall-inventory",
    "verdict-terminal-pipeline",
    "for-loop-brace-range-runs-once",
    "authored-decline-class",
    "authored-decline-class-unreadable",
    "lint-tool-absent",
    "lint-tool-output-unparsable",
    "lint-tool-failed-without-findings",
    "cli-strip-needs-path",
    "cli-strip-got-a-flag",
    "cli-unknown-mode",
    "cli-flag-needs-value",
    "cli-unknown-flag",
    "cli-unknown-flag-did-you-mean",
    "cli-flag-value-not-recognized",
    "cli-flag-value-not-a-number",
    "cli-no-book-given",
    "cli-several-main-books",
    "cli-stdin-claimed-twice",
    "cli-mode-needs-flag",
    "cli-flags-mutually-exclusive",
    "cli-flag-requires-mode",
    "apply-intent-not-publishable",
    "apply-plan-not-dispatchable",
    "cli-file-not-found",
    "cli-file-permission-denied",
    "cli-file-unreadable",
    "cli-shim-dir-unwritable",
    "lint-no-lintable-files",
    "lint-file-count-drift",
    "lint-required-tools-missing",
    "dorc-sh-usage",
    "dorc-sh-script-unreadable",
    "dorc-sh-exec-failed",
    "transport-crlf-refused",
    "transport-session-lost",
    "transport-spawn-refused",
    "transport-marker-unusable",
    "transport-apply-failed",
];

/// Deliberately RETIRED/RENAMED slugs (`27V`): the `dq-` prefix drop on the five value-plane
/// codes. Recorded so the git-diff retire-guard reads a rename as intentional, not a silent
/// deletion (`assert_no_slug_vanished` accepts a committed slug here).
const RETIRED_SLUGS: &[&str] = &[
    "unannounced-cross-custody-call",
    "transport-not-attempted",
    "dq-cmdsub-operand-top",
    "dq-site-unresolvable",
    "dq-cmdsub-inner-nonleaf",
    "dq-redir-target-top",
    "dq-depth-2-positional-unthreaded",
];

/// The SPANLESS-MINT allow-list (arch-3-residual-2): EXACTLY the codes permitted to construct a
/// diagnostic with no primary span, via [`dorc_aid::diag::Diag::new_spanless_site`]. Every other
/// code MUST point at a real source span ([`dorc_aid::diag::Diag::new`] takes a mandatory
/// [`dorc_core::Span`] — `21Z` drop-B). These are the give-up sites whose emit context genuinely
/// has no location: the errexit-region pass, the effect-map kind-disagreement check, every framed
/// records fault/integrity code, and the cli-edge whole-stream/whole-plan verdicts. Entries are
/// PAYLOAD-struct names (the `Code::<Payload>(` construction marker the grep sees).
/// Two directions are enforced by [`spanless_mint_allow_list_is_exact`] (the "structural enforce"):
/// * a `new_spanless_site(Code::X(…))` in PRODUCTION source whose `X` is NOT here ⇒ FAIL (a new
///   spanless mint must be justified and declared, or given a real span);
/// * an `X` here that no longer appears at a production `new_spanless_site` site ⇒ FAIL (the entry
///   is stale — the code stopped minting spanless; remove it). Self-cleaning.
const SPANLESS_SITE_PAYLOADS: &[&str] = &[
    "CfgErexitUnknown",         // cfg-errexit-unknown      (analysis/cfg.rs)
    "EffectKindDisagreement",   // effect-kind-disagreement (analysis/effect.rs)
    "SolverConsistencyFailure", // solver-consistency-failure (analysis + cli; `plans/302`)
    // solver-consistency-plan-demoted (cli; `302:rul-certifier-trip-guard-only`) — whole-run scope.
    "SolverConsistencyPlanDemoted",
    // plan/records.rs — every framed-deframer fault/integrity code is spanless.
    "RecordsHeaderlessRefused",
    "RecordsGluedLine",
    "RecordsHeaderMissing",
    "RecordsSentinelNonce",
    "RecordsFactTruncated",
    "RecordsIntegrityRefused",
    "RecordsTornLine",
    "RecordsAlienLine",
    "RecordsLateLine",
    "HostEvidenceAdmissionRefused",
    // cli/main.rs — whole-stream/whole-plan verdicts with no single source point.
    // cli/main.rs — a shared CELL is a cross-site coordinate: no one line is its location, and
    // pointing the caret at one would blame it for a collapse every site on the cell shares.
    "SharedCellMeasurementsDisagree",
    "DanglingReference", // the dangling coord's book-origin site is not in the emit scope (deferred, not synthesized)
    "EscalationPolicy",
    // plan/whylog.rs + cli/main.rs — `--last` reader refusals: about the durable FILE ⇒ spanless.
    "WhylogVersionRefused",
    "WhylogBookDesync",
    "WhylogAbsent",
    "WhylogCorrupt",
    "WhylogUnwritten",
    "DurableReceiptUnwritten",
    // cli/main.rs — the unloaded-sibling hint is a whole-run disclosure with no source point.
    "AidUnloadedSiblingOracle",
    // cli/main.rs — a zero-matched-sites verdict is a claim about the whole ORACLE FILE, not any
    // one book command site.
    "OracleMatchedZeroSites",
    // cli/main.rs — a FORM is a whole-run property; a caret on a book command would blame the
    // admin's text for a v0 limit (`271:rul-sin-ordering`).
    "ArtifactFormRefused",
    "ArtifactFormFallback",
    "ArtifactPublishRefused",
    // cli/main.rs — a paste hazard is a claim about a FINALIZED ARTIFACT's rendered physical line,
    // which has no book-AST span (`plan::render::PasteHygieneHazard`'s own doc: durable/paste-facing
    // surfaces, never a source-text property).
    "EmittedLineUnsafeForPaste",
    // lint — the external-tool trio is ABOUT a foreign process, not about any dorc bytes, so its
    // emit context genuinely has no span. The four dorc-native lint codes DO carry real spans.
    "LintToolAbsent",
    "LintToolOutputUnparsable",
    "LintToolFailedWithoutFindings",
    // cli — every INVOCATION error: an argv has no span at all, so the family is spanless by
    // construction, not by omission.
    "CliStripNeedsPath",
    "CliStripGotAFlag",
    "CliUnknownMode",
    "CliFlagNeedsValue",
    "CliUnknownFlag",
    "CliUnknownFlagDidYouMean",
    "CliFlagValueNotRecognized",
    "CliFlagValueNotANumber",
    "CliNoBookGiven",
    "CliSeveralMainBooks",
    "CliStdinClaimedTwice",
    "CliFlagsMutuallyExclusive",
    "CliFlagRequiresMode",
    "CliModeNeedsFlag",
    "CliFileNotFound",
    "CliFilePermissionDenied",
    "CliFileUnreadable",
    "CliShimDirUnwritable",
    "LintNoLintableFiles",
    "LintFileCountDrift",
    "LintRequiredToolsMissing",
    "DorcShUsage",
    "DorcShScriptUnreadable",
    "DorcShExecFailed",
    // apply — about an INVOCATION and its authority, not about bytes we parsed. Both are decided
    // before any book is read, and an apply reads none at all, so there is no AST to point at.
    "ApplyIntentNotPublishable",
    "ApplyPlanNotDispatchable",
    // transport — about a SESSION, not about bytes we parsed. The CRLF refusal can fire on a
    // rendered plan no parser of ours saw, so its line is a payload value, not an AST span.
    "TransportCrlfRefused",
    "TransportSessionLost",
    "TransportSpawnRefused",
    "TransportMarkerUnusable",
    "TransportApplyFailed",
];

/// The crate-`src` roots scanned (the emit surface). The workspace's analyzer crates; `aid`
/// itself is included for the `diag.rs` retire-guard.
const SCANNED_CRATES: &[&str] = &[
    "aid",
    "syntax",
    "analysis",
    "oracle",
    "plan",
    "cli",
    "coverage",
    "hostsim",
    // `289:rider-diag-tidy-scan-set` — widened ahead of the lint codes joining the registry, so
    // their emit sites satisfy the constructed-scan instead of reading as dead catalog.
    "lint",
    "dorc-loom",
];

/// The `spike/crates` dir (this test runs with cwd = `crates/aid`).
fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/aid has a parent (crates/)")
        .to_path_buf()
}

/// Recursively collect every real `.rs` source under `dir`.
fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs")
            && path
                .file_name()
                .is_none_or(|name| !name.to_string_lossy().contains(".sync-conflict-"))
        {
            out.push(path);
        }
    }
}

/// The concatenated source text of the named crates' `src/` trees (production + inline tests).
fn concat_crate_src(crate_names: &[&str]) -> String {
    let crates = crates_dir();
    let mut files = Vec::new();
    for c in crate_names {
        rs_files(&crates.join(c).join("src"), &mut files);
    }
    let mut out = String::new();
    for f in files {
        if let Ok(text) = std::fs::read_to_string(&f) {
            out.push_str(&text);
            out.push('\n');
        }
    }
    out
}

/// The concatenated source text of every scanned crate's `src/` tree (production + inline tests;
/// the legacy `DiagCode("…")` literals live in production code, and the test-only literals — the
/// coverage `refusal_diag` helper, the erasability `x-err`/`boom`/`test-warn` fixtures — are
/// EXCLUDED by scanning only the slugs we assert, never all literals).
fn scanned_source() -> String {
    concat_crate_src(SCANNED_CRATES)
}

/// The PRODUCTION emit surface for the spanless-mint gate: every scanned crate EXCEPT `aid`. The
/// six real `new_spanless_site` sites live in `analysis`/`oracle`; `aid` only DEFINES the
/// constructor and exercises it in its own `#[cfg(test)]` module — excluding `aid` keeps the
/// self-cleaning direction honest (a removed production site is not masked by aid's test usage).
///
/// ru-26 residual-b (B8 disclosure): this excludes `aid` but NOT the `#[cfg(test)]` modules of the
/// OTHER scanned crates — `rs_files` collects every `.rs` under `src/`, test modules included. So a
/// test-only construction of a payload in a non-aid crate (a `DiagCode::X(…)` inside that crate's
/// own `#[cfg(test)]`) would satisfy `every_catalog_variant_is_constructed` for code `X` even if its
/// PRODUCTION emit were deleted — the grep cannot tell a `#[cfg(test)]` line from a real one. The
/// PART C must-emit per-code pins are the real liveness instrument (they FAIL when a production emit
/// dies); this grep is a cheap belt-and-braces backstop, not a soundness guarantee. Greenfield wants
/// cfg-aware source partitioning (or an emit-side registration) rather than a whole-`src/` scan.
fn production_emit_source() -> String {
    let non_aid: Vec<&str> = SCANNED_CRATES
        .iter()
        .copied()
        .filter(|c| !NON_EMIT_CRATES.contains(c))
        .collect();
    concat_crate_src(&non_aid)
}

/// Scanned, but NOT part of the production emit surface. `aid` only DEFINES the codes (its own
/// match arms and tests would satisfy the grep for every variant — the act-3 vacuity). `dorc-loom`
/// is the same category one layer out: its typed edge-state table constructs CASE FIXTURES, so
/// counting them as emits would mask a dead catalog entry whose real emit died.
/// Both stay in [`SCANNED_CRATES`] for the scans that legitimately want the whole tree.
const NON_EMIT_CRATES: &[&str] = &["aid", "dorc-loom"];

/// The FIXTURE FENCE (`rul-fixture-identity-never-production`). `aid::fixture` holds canned
/// stand-in worlds for aid-local tests. A real diagnostic is built at its emit site out of the world
/// that site observed, so reaching the canned table from an emit would ship a fabricated path or
/// host inside a genuine refusal. The fence is that only the crate defining the table may name it.
#[test]
fn fixture_payloads_are_unreachable_from_production() {
    const ALLOWED: &[&str] = &["aid"];
    let crates = crates_dir();
    let entries = std::fs::read_dir(&crates).expect("crates/ is readable");
    let mut scanned = 0usize;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !entry.path().is_dir() || ALLOWED.contains(&name.as_str()) {
            continue;
        }
        let mut files = Vec::new();
        rs_files(&entry.path().join("src"), &mut files);
        for file in files {
            scanned += 1;
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            for needle in ["canonical_payload(", "canonical_payloads("] {
                assert!(
                    !text.contains(needle),
                    "{} names `{needle}` — the fixture stand-in worlds are for the defining-case \
                     corpus only. Build the payload from the world the emit site observed \
                     (rul-fixture-identity-never-production).",
                    file.display()
                );
            }
        }
    }
    assert!(
        scanned > 0,
        "the fixture fence scanned no files — its crate walk is broken, and a broken walk passes \
         vacuously"
    );
}

/// Extract every payload-struct name constructed at a `new_spanless_site(…::<Payload>(…))` call in
/// `source` — the spanless-mint marker. Matches both the `Code::` alias (the emit crates) and the
/// fully-qualified `DiagCode::` form; the payload name is the identifier between `::` and the `(`.
fn spanless_site_payloads(source: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let needle = "new_spanless_site(";
    let mut rest = source;
    while let Some(i) = rest.find(needle) {
        let after = &rest[i + needle.len()..];
        rest = after;
        // After `new_spanless_site(` comes `Code::` or `DiagCode::`; take the segment up to the
        // payload's own `(`, then keep the identifier after the LAST `::` (the payload name).
        let Some(open) = after.find('(') else { break };
        let head = &after[..open];
        if let Some(sep) = head.rfind("::") {
            let name = head[sep + 2..].trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                out.insert(name.to_string());
            }
        }
    }
    out
}

/// (1a) Every catalog variant is CONSTRUCTED at some PRODUCTION emit site (`226` §1 reachability).
/// A `pub enum` variant the type system never forces to be used would be dead catalog; the grep is
/// the only thing that sees it.
///
/// REWRITTEN (x3a-B/t-1 fix, `224` §10 act-3): the prior scan used `scanned_source()`, which
/// INCLUDES `aid` — so `diag.rs`'s OWN `match` arms (every `DiagCode::Variant(_) =>` in `slug`/
/// `site`/`registry`/the renders) and `aid`'s `#[cfg(test)]` constructions satisfied the grep for
/// EVERY variant. The result: deleting a sole PRODUCTION emit left the test green (proven twice — a
/// dead-catalog variant the gate was built to catch sailed through). The fix scans
/// `production_emit_source()` (every crate EXCEPT `aid`), so only a real emit in a consuming crate
/// counts — exactly what `spanless_mint_allow_list_is_exact` already does for the same reason.
///
/// NEEDLE-SHAPE LIMIT (ru-26 disclosure; t-4): this is a grep for the LITERAL forms
/// `DiagCode::Payload(` / `Code::Payload(`. It CANNOT see a construction that builds the variant
/// into a variable and passes it on (`let c = Code::Payload(..); Diag::new(c, ..)`) or any
/// `DiagCode(expr)` indirection — such an emit is invisible here and would read as dead catalog. So
/// production emits must spell the variant literally at the `Diag::new`/`new_spanless_site` site
/// (act-1's `lift_failure` does, deliberately, for precisely this reason). A non-literal emit is a
/// spike-scoped blind spot, not a general guarantee; greenfield needs a real reachability pass
/// (an emit-side registration call, or a derive) rather than a source grep.
#[test]
fn every_catalog_variant_is_constructed() {
    let source = production_emit_source();
    for payload in MIGRATED_PAYLOADS {
        // The construction marker, in PRODUCTION (non-aid) source only: a real `DiagCode::Payload(`
        // or `Code::Payload(` emit at a give-up site, NOT diag.rs's own match arms or aid's tests.
        let constructed = source.contains(&format!("DiagCode::{payload}("))
            || source.contains(&format!("Code::{payload}("));
        assert!(
            constructed,
            "catalog variant `{payload}` is registered but never constructed at a PRODUCTION emit \
             site (dead catalog — either emit it at a give-up site in a consuming crate, or remove \
             the variant + its registry/render arms). NB the scan excludes aid, so diag.rs's own \
             match arms do not count (act-3); a non-literal emit is invisible to it (needle-shape)."
        );
    }
}

/// (1b) Every structured emit site names a real catalog variant (the reverse direction). The
/// compiler already enforces this (an unknown variant does not compile), so this is a
/// belt-and-braces assertion that the grep markers and the enum agree — it catches a
/// `MIGRATED_PAYLOADS` entry that was renamed in the enum but not here.
#[test]
fn every_migrated_payload_name_is_a_real_variant() {
    // The `aid::diag::DiagCode` source must define each payload-named variant. Read diag.rs and
    // assert the variant line exists (the enum arm `Payload(Payload)`).
    let diag_src = std::fs::read_to_string(crates_dir().join("aid/src/diag.rs"))
        .expect("aid/src/diag.rs is readable");
    for payload in MIGRATED_PAYLOADS {
        assert!(
            diag_src.contains(&format!("{payload}({payload})")),
            "`{payload}` is listed as a migrated payload but the DiagCode enum has no \
             `{payload}({payload})` variant (rename drift between this gate and the enum)"
        );
    }
    // And every slug in MIGRATED_SLUGS appears in `slug`'s match (the wire token is live).
    for slug in MIGRATED_SLUGS {
        assert!(
            diag_src.contains(&format!("\"{slug}\"")),
            "migrated slug `{slug}` is not present in diag.rs (slug-vs-enum drift)"
        );
    }
}

/// The other direction, and a separate test on purpose: the retire-guard reads `HEAD`, so it
/// cannot see a slug ADDED to `diag.rs` without a matching `MIGRATED_SLUGS` entry until that change
/// is already committed — it goes green on the offending commit and red on the next unrelated run,
/// which reads as a spurious failure in whatever landed after.
///
/// This reads the WORKING TREE, so a desync fails on the change that caused it. It cannot replace
/// the retire-guard: working-tree-vs-list catches ADDITIONS, and only committed-vs-current catches
/// a DELETION — a slug dropped from both the file and the list vanishes from a working-tree scan
/// entirely, which is the tautology the retire-guard was rewritten to escape.
#[test]
fn every_working_tree_slug_is_listed() {
    let diag = std::fs::read_to_string(crates_dir().join("aid/src/diag.rs")).expect("read diag.rs");
    let slugs = committed_slug_arms(&diag);
    assert!(
        !slugs.is_empty(),
        "the `slug()` shape-scan matched nothing in the working tree — fix the scan rather than \
         letting this pass vacuously"
    );
    for slug in slugs {
        assert!(
            MIGRATED_SLUGS.contains(&slug.as_str()) || RETIRED_SLUGS.contains(&slug.as_str()),
            "`{slug}` is in `DiagCode::slug()` but not in MIGRATED_SLUGS — that list is documented \
             as kept in sync with it, and the retire-guard cannot report it until the next commit."
        );
    }
}

/// (2) The git-diff RETIRE-GUARD (`226` §1): a catalog slug removed from the committed `diag.rs`
/// without being recorded as retired is a SILENT variant deletion. We diff the committed
/// `diag.rs` against the working tree and fail if a `MIGRATED_SLUGS` entry's literal was deleted.
/// Best-effort: SKIPPED (passes) when git is unavailable or the file is untracked, so a non-git
/// checkout never blocks — the guard is a CI aid, not a hard dependency.
#[test]
fn retire_guard_no_silent_slug_deletion() {
    let crates = crates_dir();
    // `git show HEAD:<path>` — the committed diag.rs. Path is relative to the repo root; compute
    // it from the worktree root via `git rev-parse --show-prefix` would be ideal, but the simpler
    // robust form is `git show :crates/aid/src/diag.rs` (the index) restricted to the spike dir.
    let diag_rel = "crates/aid/src/diag.rs";
    let spike_dir = crates
        .parent()
        .expect("crates/ has a parent (spike/)")
        .to_path_buf();
    let Ok(output) = Command::new("git")
        .arg("-C")
        .arg(&spike_dir)
        .arg("show")
        .arg(format!("HEAD:spike/{diag_rel}"))
        .output()
    else {
        eprintln!("retire-guard: git unavailable — skipping (CI aid only)");
        return;
    };
    if !output.status.success() {
        // The path may be `spike/crates/...` or `crates/...` depending on repo layout / first
        // commit; try the un-prefixed form before giving up.
        let Ok(alt) = Command::new("git")
            .arg("-C")
            .arg(&spike_dir)
            .arg("show")
            .arg(format!("HEAD:{diag_rel}"))
            .output()
        else {
            eprintln!("retire-guard: git show failed — skipping");
            return;
        };
        if !alt.status.success() {
            eprintln!("retire-guard: diag.rs not found at HEAD (new file / untracked) — skipping");
            return;
        }
        let committed = String::from_utf8_lossy(&alt.stdout);
        assert_no_slug_vanished(&committed);
        return;
    }
    let committed = String::from_utf8_lossy(&output.stdout);
    assert_no_slug_vanished(&committed);
}

/// The retire-guard's core assertion: every slug the COMMITTED diag.rs carried must still be in
/// the CURRENT `MIGRATED_SLUGS` (i.e. still a live catalog code). The deletion direction only: a
/// committed slug no longer in `MIGRATED_SLUGS` means the code was retired — record it deliberately
/// (a retired-list) instead of letting it quietly stop existing (`226` §1 retire-guard).
///
/// REWRITTEN (x3a-E/t-2 fix, `224` §10 act-2): the prior guard was TAUTOLOGICAL — its extractor
/// pre-filtered committed slugs by membership in the CURRENT `MIGRATED_SLUGS`, then asserted that
/// same membership, so the assertion could never fail (a slug deleted from BOTH `diag.rs` and the
/// list vanished from the extracted set and was never checked; a full silent catalog retirement
/// stayed green, proven twice). The fix is a real committed-source→current-list direction:
/// [`committed_slug_arms`] reads the slugs the committed `slug()` carried by their SHAPE alone
/// (bounded to the `fn slug` body, never gated on the current list), so a slug the working tree
/// dropped from `MIGRATED_SLUGS` is still extracted from the committed source and trips the assert.
fn assert_no_slug_vanished(committed_diag_rs: &str) {
    let committed = committed_slug_arms(committed_diag_rs);
    // Sanity: the committed `slug()` body MUST yield at least one arm, or the shape-scan silently
    // matched nothing and the guard would be vacuous again (a refactor of `slug()` that this scan
    // can't read must fail loudly, not pass empty). The catalog is never empty at any real HEAD.
    assert!(
        !committed.is_empty(),
        "retire-guard: the committed diag.rs `slug()` body yielded no slug arms — the shape-scan \
         matched nothing (a `slug()` refactor this guard cannot read, or a corrupt fetch). Fix the \
         scan; do not let the retire-guard pass vacuously (the x3a-E/t-2 tautology class)."
    );
    for slug in committed {
        assert!(
            MIGRATED_SLUGS.contains(&slug.as_str()) || RETIRED_SLUGS.contains(&slug.as_str()),
            "retire-guard: catalog slug `{slug}` was in the committed diag.rs `slug()` but is gone \
             from MIGRATED_SLUGS — a silent catalog deletion. If intentional (a rename/retirement), \
             record it on RETIRED_SLUGS deliberately; do not let a code quietly stop existing \
             (226 §1 retire-guard)."
        );
    }
}

/// Pull the catalog slugs out of a committed diag.rs by their SHAPE inside the `slug()` function
/// body — the `=> "…"` arms — WITHOUT consulting the current `MIGRATED_SLUGS` (that circular
/// pre-filter was the t-2 tautology). The scan is bounded to the `fn slug(` body (from its header
/// to the next top-level `fn ` at the same indentation) so it never picks up doc-comment mentions
/// or unrelated `=> "…"` string arms elsewhere in the file. Every arm slug inside that body is a
/// real catalog wire token by construction.
fn committed_slug_arms(diag_rs: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    // Locate the `slug` method body. `pub fn slug(` is unique in diag.rs (the one wire-token fn);
    // scan from there to the next line that begins a sibling item (`    fn ` / `    pub fn ` /
    // a `}` at column 0 closing the impl) — generous enough to cover the whole match, tight enough
    // to exclude the rest of the file.
    let Some(start) = diag_rs.find("fn slug(") else {
        // No `slug` fn in the committed source at all — return empty; `assert_no_slug_vanished`'s
        // non-empty guard turns this into a loud failure (the scan could not read the catalog).
        return out;
    };
    let body = &diag_rs[start..];
    for line in body.lines().skip(1) {
        // Stop at the next `fn` after `slug`'s body — `slug`'s arms are all before it, so a stray
        // `=> "…"` in a later method (`OperandPosition::describe`, `remediation_tag`, the renders)
        // is excluded. The skip(1) drops the `fn slug(` header line itself.
        let trimmed = line.trim_start();
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
            break;
        }
        // ru-26 residual-a (B8 disclosure): this reads only single-line `=> "…"` arms. An arm
        // formatted exotically (the `=>` and the string literal split across lines, or a slug
        // built by an expression) is INVISIBLE to this scan ⇒ a silent retirement of such an arm
        // would slip the retire-guard. rustfmt keeps `slug()`'s arms single-line today, so this
        // holds at HEAD; greenfield needs a real token-aware scan (or an emit-side registration),
        // not a line-shape grep, if the catalog ever carries multi-line arms.
        if let Some(arrow) = trimmed.find("=> \"") {
            let after = &trimmed[arrow + 4..];
            if let Some(end) = after.find('"') {
                out.insert(after[..end].to_string());
            }
        }
    }
    out
}

/// (3) The catalog COMPLETENESS bijection (`27V:rul-kill-legacy-diagnostic` / `defining-case-
/// catalog`): every `DiagCode` variant has EXACTLY ONE [`dorc_aid::catalog::CatalogEntry`], and
/// every entry names a real variant. `MIGRATED_SLUGS` mirrors the exhaustive `DiagCode::slug`
/// match (its own gate, `every_migrated_payload_name_is_a_real_variant`, pins that mirror), so the
/// slug set stands in for the variant set. This is what makes a code with no prose home — or an
/// orphan catalog row — a loud test failure rather than a silent `[unwritten:]` render.
#[test]
fn every_variant_has_exactly_one_catalog_entry() {
    use dorc_aid::catalog::CATALOG;
    assert!(
        REPAIR_HINT.contains("dorc-loom scaffold"),
        "the completeness failure must name the repair command verbatim; a reword may not drop it"
    );
    let catalog_slugs: BTreeSet<&str> = CATALOG.iter().map(|e| e.slug).collect();
    // No duplicate entries (catalog's own gate also checks this; belt-and-braces here).
    assert_eq!(
        catalog_slugs.len(),
        CATALOG.len(),
        "the catalog carries a duplicate slug (two entries for one code)"
    );
    // Every variant slug has an entry (no `[unwritten:]`-rendering hole).
    for slug in MIGRATED_SLUGS {
        assert!(
            catalog_slugs.contains(slug),
            "DiagCode variant slug `{slug}` has no CatalogEntry — every code needs exactly one \
             prose home (27V:rul-kill-legacy-diagnostic). {REPAIR_HINT}"
        );
    }
    // Every catalog entry names a real variant (no orphan row).
    for e in CATALOG {
        assert!(
            MIGRATED_SLUGS.contains(&e.slug),
            "CatalogEntry `{}` names no DiagCode variant (orphan catalog row) — remove it or add \
             the variant + its slug arm.",
            e.slug
        );
    }
}

/// (4) The SPANLESS-MINT allow-list is EXACT (arch-3-residual-2). The mandatory-primary-span
/// guarantee (`21Z` drop-B) means `Diag::new` cannot produce a span-less diagnostic; only the
/// gated, second-class `Diag::new_spanless_site` can. This gate makes that privilege STRUCTURAL:
/// the set of codes that actually mint spanless in PRODUCTION source must equal
/// [`SPANLESS_SITE_PAYLOADS`] exactly. Two directions (self-cleaning, like the legacy list):
/// * a `new_spanless_site(Code::X(…))` whose `X` is NOT allow-listed ⇒ FAIL (a new spanless mint
///   slipped in without review — give it a real span, or justify and declare it here);
/// * an allow-listed `X` that no longer appears at a production `new_spanless_site` site ⇒ FAIL
///   (the entry is stale; the code now carries a span or was removed — delete it from the list).
///
/// Scans `production_emit_source` (excludes `aid`): the six real sites live in `analysis`/
/// `oracle`, and excluding aid's own definition + `#[cfg(test)]` exercise keeps direction B from
/// being masked by the test's construction.
#[test]
fn spanless_mint_allow_list_is_exact() {
    let found = spanless_site_payloads(&production_emit_source());

    // Direction A: every production spanless mint is allow-listed.
    for payload in &found {
        assert!(
            SPANLESS_SITE_PAYLOADS.contains(&payload.as_str()),
            "`Diag::new_spanless_site(Code::{payload}(…))` mints a span-less diagnostic but \
             `{payload}` is NOT on SPANLESS_SITE_PAYLOADS — every code must point at a real span \
             (Diag::new, 21Z drop-B) UNLESS its emit context genuinely has none. If this is a \
             true no-span site, justify it and add `{payload}` to the allow-list; otherwise plumb \
             a span and use Diag::new (arch-3-residual-2)."
        );
    }

    // Direction B (self-cleaning): every allow-list entry is still a live production spanless site.
    for &payload in SPANLESS_SITE_PAYLOADS {
        assert!(
            found.contains(payload),
            "SPANLESS_SITE_PAYLOADS entry `{payload}` no longer appears at a production \
             `new_spanless_site(Code::{payload}(…))` site — it now carries a span or was removed; \
             delete it from the allow-list (the list is self-cleaning, arch-3-residual-2)."
        );
    }

    // Belt-and-braces: the allow-list and the found set are EXACTLY equal (no duplicates-or-typos
    // path the two directional loops could individually miss).
    let allow: BTreeSet<String> = SPANLESS_SITE_PAYLOADS
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(
        found, allow,
        "the production spanless-mint set must equal SPANLESS_SITE_PAYLOADS exactly"
    );
}

// ===========================================================================
// Negative controls — prove the rewritten guards can actually FIRE (the t-1/t-2 class)
// ===========================================================================

/// act-2 NEGATIVE CONTROL (x3a-E/t-2): the rewritten retire-guard must TRIP on a silent catalog
/// retirement. We synthesize a committed-`diag.rs` `slug()` body carrying a ghost slug that is NOT
/// in the current `MIGRATED_SLUGS`, and assert (a) the shape-scan extracts it ANYWAY — the property
/// the old circular pre-filter destroyed — and (b) `assert_no_slug_vanished` panics on it. The old
/// guard passed this same input green (the ghost was filtered out before the assert); this pins
/// that it no longer can.
#[test]
fn retire_guard_negative_control_trips_on_silent_retirement() {
    // A minimal stand-in for a COMMITTED diag.rs `slug()` whose source still carried a code that
    // the working tree has since silently dropped from MIGRATED_SLUGS.
    let committed = r#"
        pub fn slug(&self) -> &'static str {
            match self {
                DiagCode::SomeLiveCode(_) => "site-unresolvable",
                DiagCode::GhostRetired(_) => "ghost-retired-code",
            }
        }
        fn something_else(&self) -> u32 { 0 }
    "#;
    let extracted = committed_slug_arms(committed);
    // (a) The anti-tautology property: the scan sees the ghost regardless of MIGRATED_SLUGS.
    assert!(
        extracted.contains("ghost-retired-code"),
        "the shape-scan must extract a committed slug even when it is absent from the current \
         MIGRATED_SLUGS — that independence is exactly what kills the t-2 tautology"
    );
    assert!(
        !MIGRATED_SLUGS.contains(&"ghost-retired-code"),
        "precondition: the ghost slug is genuinely not a current catalog code"
    );
    // (b) The guard panics on this committed source (a silent retirement is caught).
    let result = std::panic::catch_unwind(|| assert_no_slug_vanished(committed));
    assert!(
        result.is_err(),
        "assert_no_slug_vanished MUST panic when a committed slug vanished from MIGRATED_SLUGS \
         (silent retirement) — if it passes, the guard is vacuous again (t-2)"
    );
}

/// act-2 companion: the non-empty guard inside `assert_no_slug_vanished` must itself fire when the
/// shape-scan reads nothing (a `slug()` refactor this scan cannot parse). Proves the guard cannot
/// pass vacuously on an empty extraction — the other half of the t-2 class (a scan that silently
/// matches nothing is as bad as a circular filter).
#[test]
fn retire_guard_negative_control_trips_on_unreadable_slug_body() {
    let no_slug_fn = "pub fn unrelated(&self) -> u32 { match self { _ => 0 } }";
    assert!(
        committed_slug_arms(no_slug_fn).is_empty(),
        "precondition: a source with no `fn slug(` yields no arms"
    );
    let result = std::panic::catch_unwind(|| assert_no_slug_vanished(no_slug_fn));
    assert!(
        result.is_err(),
        "assert_no_slug_vanished MUST panic on an empty extraction (an unreadable `slug()` body), \
         never pass vacuously"
    );
}

/// act-3 NEGATIVE CONTROL (x3a-B/t-1): the constructed-scan must scan PRODUCTION emits only, so a
/// variant satisfied solely by `diag.rs`'s own match arms (or aid's tests) would NOT pass. We
/// cannot delete a real production emit inside a test, so we pin the load-bearing PROPERTY directly:
/// the scan basis (`production_emit_source`) excludes `aid`'s `diag.rs`, while the OLD basis
/// (`scanned_source`) included it. A unique `diag.rs`-only marker present in the old basis and
/// ABSENT from the new one proves diag.rs's own arms can no longer satisfy the grep — which is
/// exactly why deleting a sole production emit now fails the scan (the t-1 vacuity is closed).
#[test]
fn constructed_scan_negative_control_excludes_aid_diag_arms() {
    // A token that exists ONLY in aid/src/diag.rs (the spanless-mint constructor's name). It is a
    // `aid`-internal definition, never written in a consuming crate's emit.
    let aid_only_marker = "pub fn new_spanless_site(";
    let old_basis = scanned_source();
    let new_basis = production_emit_source();
    assert!(
        old_basis.contains(aid_only_marker),
        "precondition: the old `scanned_source` basis DID include aid/diag.rs (that inclusion was \
         the t-1 vacuity — diag.rs's own arms satisfied the grep for every variant)"
    );
    assert!(
        !new_basis.contains(aid_only_marker),
        "the rewritten scan basis must EXCLUDE aid/diag.rs, so a variant constructed only in \
         diag.rs's own match arms / aid tests is NOT seen — deleting its sole production emit now \
         trips `every_catalog_variant_is_constructed` (act-3, t-1 closed)"
    );
    // Belt-and-braces: confirm a fabricated payload name appearing ONLY as a diag.rs-style arm is
    // not findable in the production basis (the scan cannot be satisfied by an arm).
    assert!(
        !new_basis.contains("DiagCode::ThisVariantDoesNotExistAnywhere("),
        "sanity: a non-emitted variant is absent from the production basis"
    );
    // The SAME vacuity one layer out: `dorc-loom`'s case fixtures would stand in for dead emits.
    let loom_only_marker = "pub(crate) enum EdgeFault";
    assert!(
        scanned_source().contains(loom_only_marker),
        "precondition: the widened scan set does include dorc-loom's source"
    );
    assert!(
        !new_basis.contains(loom_only_marker),
        "the production basis must EXCLUDE dorc-loom: its case-fixture constructors are not emits"
    );
}

/// The FIXTURE-INTAKE FENCE (`28L:rul-records-seam-approved`, rider (b)).
///
/// `dorc_cli::results::admit_fixture_records` mints a `Framing::spike` scope internally so a loom
/// case can drive the REAL host-evidence admission over its own committed records. Its SIGNATURE
/// already fences the identity — it takes no `Framing`, host, nonce or attempt, and none can be
/// added by a caller — so no fixture caller can name a managed host. What a signature cannot fence
/// is who calls it: `dorc-cli`'s lib, its bin, and `dorc-loom` are three separate crates, and no
/// type can privilege one over another. So the production side is pinned lexically here, exactly as
/// [`fixture_payloads_are_unreachable_from_production`] pins the stand-in worlds.
///
/// The production intake is `admit_controller_records`, which takes the controller's own framing;
/// that is the seat where the re-entry trigger will bite when a second scope first becomes
/// representable (`rul-attribution-is-controller-minted`).
#[test]
fn fixture_intake_is_unreachable_from_production() {
    /// The lib that DEFINES it, and the harness that is licensed to drive it.
    const ALLOWED_CRATES: &[&str] = &["cli", "dorc-loom"];
    /// Within `cli`, only the defining module may name it — never the binary.
    const ALLOWED_CLI_FILES: &[&str] = &["results.rs"];

    let crates = crates_dir();
    let entries = std::fs::read_dir(&crates).expect("crates/ is readable");
    let mut scanned = 0usize;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !entry.path().is_dir() {
            continue;
        }
        let mut files = Vec::new();
        rs_files(&entry.path().join("src"), &mut files);
        for file in files {
            let base = file
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let licensed = match name.as_str() {
                "dorc-loom" => true,
                "cli" => ALLOWED_CLI_FILES.contains(&base.as_str()),
                _ => false,
            };
            if licensed {
                continue;
            }
            scanned += 1;
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            assert!(
                !text.contains("admit_fixture_records("),
                "{} names `admit_fixture_records(` — that entry point mints a FIXTURE framing \
                 (`Framing::spike`) and exists so a loom case can drive the real admission over \
                 its own bytes. Production intake goes through `admit_controller_records` with \
                 the framing this run's controller minted \
                 (rul-attribution-is-controller-minted).",
                file.display()
            );
        }
    }
    assert!(
        ALLOWED_CRATES.len() == 2 && scanned > 0,
        "the fixture-intake fence scanned no files — its crate walk is broken, and a broken walk \
         passes vacuously"
    );
}

/// The FOREIGN-EDGE FENCE (`282:rul-passthrough-type-gated`, the `admit_fixture_records`
/// precedent). `ForeignBytes::from_os_error` needs no fence — its argument type already says the
/// words came from the platform — but `from_io_edge` takes a bare `&str`, and an unfenced bare-str
/// constructor is exactly the hole the seal exists to close: any sentence we composed could reach
/// it and arrive wearing the not-ours badge, un-editable at the loom and indistinguishable from a
/// host's bytes to anything reasoning about provenance downstream.
///
/// So the constructor is spellable only at the files listed here, each a genuine relay of bytes
/// somebody else wrote. Adding a file to this list is a claim about that file, reviewed as one.
#[test]
fn foreign_edge_constructor_is_fenced() {
    /// Path suffixes (`/`-spelled) permitted to name the bare-str edge constructor, and why.
    const ALLOWED: &[(&str, &str)] = &[
        (
            "aid/src/diag.rs",
            "quotes book and oracle source into carets and cause loci",
        ),
        (
            "aid/src/fixture.rs",
            "the canned stand-in worlds, which stand in for real edges",
        ),
        (
            "aid/src/foreign.rs",
            "the seal itself: the constructor's own definition and its tests",
        ),
        ("aid/src/said.rs", "an inline test's book-line excerpt"),
        ("aid/src/weave.rs", "an inline test's oracle-arm excerpt"),
        (
            "cli/src/lib.rs",
            "seals a transport driver's platform spawn-refusal detail at the shared production mapping",
        ),
        (
            "cli/src/why.rs",
            "quotes book, oracle and shipped-guard source into the why report",
        ),
        (
            "lint/src/source_external.rs",
            "captures an external linter's own output",
        ),
    ];
    const NEEDLE: &str = "from_io_edge(";

    let crates = crates_dir();
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&crates)
        .expect("crates/ is readable")
        .flatten()
    {
        rs_files(&entry.path().join("src"), &mut files);
    }
    assert!(!files.is_empty(), "the foreign-edge fence scanned no files");
    let mut seen = 0usize;
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        if !text.contains(NEEDLE) {
            continue;
        }
        let path = file.display().to_string().replace('\\', "/");
        let allowed = ALLOWED.iter().find(|(suffix, _)| path.ends_with(suffix));
        assert!(
            allowed.is_some(),
            "{path} names `{NEEDLE}` — that constructor declares bytes to be somebody else's, and \
             a sentence composed here is OURS. Move the words into the code's catalog register \
             with typed holes (282:rul-passthrough-type-gated); if this really is an I/O relay, \
             add it to ALLOWED in this test with its reason."
        );
        seen += 1;
    }
    assert_eq!(
        seen,
        ALLOWED.len(),
        "an ALLOWED entry no longer names the edge constructor — drop it rather than leaving the \
         fence wider than the code"
    );
}
