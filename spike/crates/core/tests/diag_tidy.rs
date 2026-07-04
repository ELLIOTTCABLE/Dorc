//! The diagnostic-catalog **tidy gate** — the cheap structural half of `22A` concl-7 /
//! `226` §1 (rustc's `tidy error_codes.rs`), the half the Rust type system cannot see.
//!
//! The exhaustive [`dorc_core::diag::DiagCode`] enum already buys "every variant HANDLED" for
//! free: the `registry`, every render arm, and `slug` are exhaustive `match`es that will not
//! compile with a variant missing. This test covers the THREE things the compiler can't:
//!
//! 1. **bidirectional reachability** (`226` §1) — every catalog variant is CONSTRUCTED at some
//!    emit site (the type system never forces a `pub enum` variant to be used), AND every
//!    structured construction site names a real catalog variant (no orphan emit). A variant
//!    with a registry row but no emit site is dead catalog; an emit with no row cannot exist
//!    (it would not compile), so this direction is a belt-and-braces grep.
//! 2. **a git-diff retire-guard** (`226` §1, the `error_codes.rs` deletion guard) — a catalog
//!    slug removed from `diag.rs` without being added to the retired-list is a SILENT variant
//!    deletion (a code that quietly stopped existing). Caught by diffing the committed `diag.rs`
//!    against the working tree for removed `slug` arms. Best-effort: skipped (not failed) when
//!    git is unavailable, so the gate never blocks a non-git checkout.
//! 3. **a self-cleaning allow-list** (`226` §1, the hardcoded grandfathered gaps) — every
//!    legacy give-up site NOT yet migrated onto the spine is named here, reviewer-visible. The
//!    list SHRINKS as the B4 sweep migrates codes; a legacy `DiagCode("…")` string-construction
//!    that is NOT on the list fails the gate (a new un-migrated code must be declared, or
//!    migrated). "Self-cleaning": a slug on the list that no longer appears in the source also
//!    fails (the list must not rot with stale entries).
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

/// Every migrated catalog variant's PAYLOAD-struct name — the spine's construction marker. Each
/// variant wraps a uniquely-named payload struct that is constructed ONLY at an emit site (the
/// `DiagCode::Variant(Payload { … })` form), so grepping the struct literal is robust to the
/// `DiagCode`-vs-`Code`-alias the emit crates use. KEEP IN SYNC with [`dorc_core::diag::DiagCode`]
/// — a new variant adds one entry here (the same one-edit friction the catalog promises).
const MIGRATED_PAYLOADS: &[&str] = &[
    "CmdsubOperandTop",
    "SiteUnresolvable",
    "RenderHeredocRefused",
    // B4 sweep: former diag::legacy survivors
    "CmdsubInnerNonleaf",
    "RedirTargetTop",
    "Depth2PositionalUnthreaded",
    // B4 sweep: syntax/parser.rs
    "SyntaxUnsupported",
    "SyntaxMalformed",
    // B4 sweep: analysis/cfg.rs
    "CfgTopNode",
    "CfgErexitUnknown",
    "CfgInlineRefused",
    "CfgBuiltinShadowed",
    // B4 sweep: analysis/effect.rs
    "EffectKindDisagreement",
    // B4 sweep: oracle/predict/parser.rs
    "PredictOutOfDialect",
    "PredictUnterminated",
];

/// Every catalog slug (the stable wire string) — for the retire-guard and reachability. KEEP IN
/// SYNC with `DiagCode::slug`. A slug removed here without a retired-list entry is a silent
/// deletion (guard 2).
const MIGRATED_SLUGS: &[&str] = &[
    "dq-cmdsub-operand-top",
    "dq-site-unresolvable",
    "render-heredoc-refused",
    // B4 sweep: former diag::legacy survivors
    "dq-cmdsub-inner-nonleaf",
    "dq-redir-target-top",
    "dq-depth-2-positional-unthreaded",
    // B4 sweep: syntax/parser.rs
    "syntax-unsupported",
    "syntax-malformed",
    // B4 sweep: analysis/cfg.rs
    "cfg-top-node",
    "cfg-errexit-unknown",
    "cfg-inline-refused",
    "cfg-builtin-shadowed",
    // B4 sweep: analysis/effect.rs
    "effect-kind-disagreement",
    // B4 sweep: oracle/predict/parser.rs
    "predict-out-of-dialect",
    "predict-unterminated",
];

/// The self-cleaning ALLOW-LIST (`226` §1): every legacy give-up code still on
/// [`dorc_core::Diagnostic`] (the string-`DiagCode("…")` form), NOT yet migrated onto the spine.
/// Reviewer-visible and SHRINKING — the B4 mechanical sweep empties it. Each entry is a legacy
/// code's stable string slug. Two directions are enforced (the "self-cleaning" half):
/// * a legacy `DiagCode("X")` construction in the source whose `X` is NOT here ⇒ FAIL (a new
///   un-migrated code must be declared or migrated);
/// * an `X` here that no longer appears in the source ⇒ FAIL (a stale allow-list entry — it was
///   migrated or deleted but left rotting on the list).
///
/// Seeded at this HEAD by inventorying every `DiagCode("…")` literal across the crate sources
/// (the conductor's re-inventory mandate — `21Z`/`22B` site counts were stale). The migrated
/// three are deliberately ABSENT (they moved to the spine).
// B4 sweep complete: all 20 codes migrated onto the Diag spine. Self-cleaning: a new
// un-migrated code that introduces a DiagCode("…") literal must be declared here immediately.
// * `footprint-incoherent` (Stage 2 / 24A §1b coherence check): the loud refusal when a wall's
//   touches() footprint omits its own establish coordinate (at-least ⊄ at-most). A cli-edge
//   Warning; PENDING typed-spine migration (tc-footprint-diag — the diagnostic wants a
//   registry-declared home like the predict-dialect codes, deferred with the guard-tier
//   diagnostics it will share a render pass with).
// * `touches-escalated` (Stage 4 / 24E §4 fork-4B): the SPIKE-ONLY (ru-26) escalation advisory —
//   a cli-edge Note surfacing that a payload-bound touches() shipped to host-derivation. Shares
//   the deferred typed-spine migration (tc-footprint-diag); spike-instrumentation, not a
//   permanent greenfield requirement.
const LEGACY_ALLOW_LIST: &[&str] = &["footprint-incoherent", "touches-escalated"];

/// The SPANLESS-MINT allow-list (arch-3-residual-2): EXACTLY the codes permitted to construct a
/// diagnostic with no primary span, via [`dorc_core::diag::Diag::new_spanless_site`]. Every other
/// code MUST point at a real source span ([`dorc_core::diag::Diag::new`] takes a mandatory
/// [`dorc_core::Span`] — `21Z` drop-B). These SIX are the give-up sites whose emit context
/// genuinely has no location: the errexit-region pass, the effect-map kind-disagreement check, and
/// the four whole-file oracle-lifter contract verdicts. Entries are PAYLOAD-struct names (the
/// `Code::<Payload>(` construction marker the grep sees), paired with the wire slug for reviewers.
/// Two directions are enforced by [`spanless_mint_allow_list_is_exact`] (the "structural enforce"):
/// * a `new_spanless_site(Code::X(…))` in PRODUCTION source whose `X` is NOT here ⇒ FAIL (a new
///   spanless mint must be justified and declared, or given a real span);
/// * an `X` here that no longer appears at a production `new_spanless_site` site ⇒ FAIL (the entry
///   is stale — the code stopped minting spanless; remove it). Self-cleaning, like the legacy list.
const SPANLESS_SITE_PAYLOADS: &[&str] = &[
    "CfgErexitUnknown",       // cfg-errexit-unknown      (analysis/cfg.rs)
    "EffectKindDisagreement", // effect-kind-disagreement (analysis/effect.rs)
];

/// The crate-`src` roots scanned (the emit surface). The workspace's analyzer crates; `core`
/// itself is included for the `diag.rs` retire-guard + the `legacy` module's consts.
const SCANNED_CRATES: &[&str] = &[
    "core", "syntax", "analysis", "oracle", "plan", "cli", "coverage", "hostsim",
];

/// The `spike/crates` dir (this test runs with cwd = `crates/core`).
fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/core has a parent (crates/)")
        .to_path_buf()
}

/// Recursively collect every `.rs` file under `dir`.
fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
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

/// The PRODUCTION emit surface for the spanless-mint gate: every scanned crate EXCEPT `core`. The
/// six real `new_spanless_site` sites live in `analysis`/`oracle`; `core` only DEFINES the
/// constructor and exercises it in its own `#[cfg(test)]` module — excluding `core` keeps the
/// self-cleaning direction honest (a removed production site is not masked by core's test usage).
///
/// ru-26 residual-b (B8 disclosure): this excludes `core` but NOT the `#[cfg(test)]` modules of the
/// OTHER scanned crates — `rs_files` collects every `.rs` under `src/`, test modules included. So a
/// test-only construction of a payload in a non-core crate (a `DiagCode::X(…)` inside that crate's
/// own `#[cfg(test)]`) would satisfy `every_catalog_variant_is_constructed` for code `X` even if its
/// PRODUCTION emit were deleted — the grep cannot tell a `#[cfg(test)]` line from a real one. The
/// PART C must-emit per-code pins are the real liveness instrument (they FAIL when a production emit
/// dies); this grep is a cheap belt-and-braces backstop, not a soundness guarantee. Greenfield wants
/// cfg-aware source partitioning (or an emit-side registration) rather than a whole-`src/` scan.
fn production_emit_source() -> String {
    let non_core: Vec<&str> = SCANNED_CRATES
        .iter()
        .copied()
        .filter(|c| *c != "core")
        .collect();
    concat_crate_src(&non_core)
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

/// Extract every `DiagCode("X")` legacy-string slug constructed in `source` (the migration-debt
/// surface). A simple lexical scan for the `DiagCode("` … `")` form; the const-definition sites
/// (`const FOO: DiagCode = DiagCode("x")`) and the emit sites both match, which is what we want
/// (a const defined but never emitted is still catalog debt). Test-fixture slugs (the
/// erasability/coverage `x-err`-style throwaways) are filtered by asserting only against the
/// allow-list's known slugs, never the raw set.
fn legacy_string_slugs(source: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let needle = "DiagCode(\"";
    let mut rest = source;
    while let Some(i) = rest.find(needle) {
        let after = &rest[i + needle.len()..];
        if let Some(end) = after.find('"') {
            out.insert(after[..end].to_string());
            rest = &after[end..];
        } else {
            break;
        }
    }
    out
}

/// (1a) Every catalog variant is CONSTRUCTED at some PRODUCTION emit site (`226` §1 reachability).
/// A `pub enum` variant the type system never forces to be used would be dead catalog; the grep is
/// the only thing that sees it.
///
/// REWRITTEN (x3a-B/t-1 fix, `224` §10 act-3): the prior scan used `scanned_source()`, which
/// INCLUDES `core` — so `diag.rs`'s OWN `match` arms (every `DiagCode::Variant(_) =>` in `slug`/
/// `site`/`registry`/the renders) and `core`'s `#[cfg(test)]` constructions satisfied the grep for
/// EVERY variant. The result: deleting a sole PRODUCTION emit left the test green (proven twice — a
/// dead-catalog variant the gate was built to catch sailed through). The fix scans
/// `production_emit_source()` (every crate EXCEPT `core`), so only a real emit in a consuming crate
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
        // The construction marker, in PRODUCTION (non-core) source only: a real `DiagCode::Payload(`
        // or `Code::Payload(` emit at a give-up site, NOT diag.rs's own match arms or core's tests.
        let constructed = source.contains(&format!("DiagCode::{payload}("))
            || source.contains(&format!("Code::{payload}("));
        assert!(
            constructed,
            "catalog variant `{payload}` is registered but never constructed at a PRODUCTION emit \
             site (dead catalog — either emit it at a give-up site in a consuming crate, or remove \
             the variant + its registry/render arms). NB the scan excludes core, so diag.rs's own \
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
    // The `core::diag::DiagCode` source must define each payload-named variant. Read diag.rs and
    // assert the variant line exists (the enum arm `Payload(Payload)`).
    let diag_src = std::fs::read_to_string(crates_dir().join("core/src/diag.rs"))
        .expect("core/src/diag.rs is readable");
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
    // robust form is `git show :crates/core/src/diag.rs` (the index) restricted to the spike dir.
    let diag_rel = "crates/core/src/diag.rs";
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
            MIGRATED_SLUGS.contains(&slug.as_str()),
            "retire-guard: catalog slug `{slug}` was in the committed diag.rs `slug()` but is gone \
             from MIGRATED_SLUGS — a silent catalog deletion. If intentional, record it as retired \
             deliberately; do not let a code quietly stop existing (226 §1 retire-guard)."
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

/// (3) The self-cleaning ALLOW-LIST (`226` §1). Two directions:
/// * every legacy `DiagCode("X")` slug in the source is EITHER a migrated slug (a leftover
///   reference, e.g. a test fixture or the slug constant) OR on the allow-list — a new
///   un-migrated code that is neither fails;
/// * every allow-list entry still appears in the source (no stale entry rotting the list).
#[test]
fn legacy_allow_list_is_complete_and_self_cleaning() {
    let source = scanned_source();
    let found = legacy_string_slugs(&source);

    // Direction A: every legacy slug found is accounted for (allow-listed, or a migrated slug
    // whose string still appears — e.g. the coverage `refusal_diag` test helper or the slug
    // const). A slug that is neither is an undeclared un-migrated code.
    for slug in &found {
        let accounted = LEGACY_ALLOW_LIST.contains(&slug.as_str())
            || MIGRATED_SLUGS.contains(&slug.as_str())
            // test-fixture throwaways (erasability/carrier unit tests) — never real codes.
            || is_test_fixture_slug(slug);
        assert!(
            accounted,
            "legacy `DiagCode(\"{slug}\")` is constructed but is neither on the LEGACY_ALLOW_LIST \
             nor a migrated/ fixture slug — declare it on the allow-list (un-migrated) or migrate \
             it onto the spine (226 §1)"
        );
    }

    // Direction B (self-cleaning): every allow-list entry must still appear in the source. A
    // stale entry means the code was migrated/deleted but left here — the list must shrink, not
    // rot.
    for &slug in LEGACY_ALLOW_LIST {
        assert!(
            found.contains(slug),
            "LEGACY_ALLOW_LIST entry `{slug}` no longer appears as a `DiagCode(\"…\")` in the \
             source — it was migrated or removed; delete it from the allow-list (the list is \
             self-cleaning, 226 §1)"
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
/// Scans `production_emit_source` (excludes `core`): the six real sites live in `analysis`/
/// `oracle`, and excluding core's own definition + `#[cfg(test)]` exercise keeps direction B from
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

/// The known TEST-FIXTURE diagnostic slugs (the erasability/carrier unit-test throwaways): never
/// real catalog codes, so they are neither allow-listed nor migrated. Listing them explicitly
/// (rather than filtering by "in a test module") keeps the gate's exclusion reviewer-visible.
fn is_test_fixture_slug(slug: &str) -> bool {
    matches!(
        slug,
        "test-warn" | "boom" | "x-note" | "x-warn" | "x-err" | "e"
    )
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
                DiagCode::SomeLiveCode(_) => "dq-site-unresolvable",
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
/// variant satisfied solely by `diag.rs`'s own match arms (or core's tests) would NOT pass. We
/// cannot delete a real production emit inside a test, so we pin the load-bearing PROPERTY directly:
/// the scan basis (`production_emit_source`) excludes `core`'s `diag.rs`, while the OLD basis
/// (`scanned_source`) included it. A unique `diag.rs`-only marker present in the old basis and
/// ABSENT from the new one proves diag.rs's own arms can no longer satisfy the grep — which is
/// exactly why deleting a sole production emit now fails the scan (the t-1 vacuity is closed).
#[test]
fn constructed_scan_negative_control_excludes_core_diag_arms() {
    // A token that exists ONLY in core/src/diag.rs (the spanless-mint constructor's name). It is a
    // `core`-internal definition, never written in a consuming crate's emit.
    let core_only_marker = "pub fn new_spanless_site(";
    let old_basis = scanned_source();
    let new_basis = production_emit_source();
    assert!(
        old_basis.contains(core_only_marker),
        "precondition: the old `scanned_source` basis DID include core/diag.rs (that inclusion was \
         the t-1 vacuity — diag.rs's own arms satisfied the grep for every variant)"
    );
    assert!(
        !new_basis.contains(core_only_marker),
        "the rewritten scan basis must EXCLUDE core/diag.rs, so a variant constructed only in \
         diag.rs's own match arms / core tests is NOT seen — deleting its sole production emit now \
         trips `every_catalog_variant_is_constructed` (act-3, t-1 closed)"
    );
    // Belt-and-braces: confirm a fabricated payload name appearing ONLY as a diag.rs-style arm is
    // not findable in the production basis (the scan cannot be satisfied by an arm).
    assert!(
        !new_basis.contains("DiagCode::ThisVariantDoesNotExistAnywhere("),
        "sanity: a non-emitted variant is absent from the production basis"
    );
}
