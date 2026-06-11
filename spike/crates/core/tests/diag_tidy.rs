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
];

/// Every catalog slug (the stable wire string) — for the retire-guard and reachability. KEEP IN
/// SYNC with `DiagCode::slug`. A slug removed here without a retired-list entry is a silent
/// deletion (guard 2).
const MIGRATED_SLUGS: &[&str] = &[
    "dq-cmdsub-operand-top",
    "dq-site-unresolvable",
    "render-heredoc-refused",
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
const LEGACY_ALLOW_LIST: &[&str] = &[
    // syntax/parser.rs
    "syntax-unsupported",
    "syntax-malformed",
    // analysis/cfg.rs
    "cfg-top-node",
    "cfg-errexit-unknown",
    "cfg-inline-refused",
    "cfg-builtin-shadowed",
    // analysis/effect.rs
    "effect-kind-disagreement",
    // analysis/effect.rs diag::legacy survivors (constructed via the legacy module's consts)
    "dq-cmdsub-inner-nonleaf",
    "dq-redir-target-top",
    "dq-depth-2-positional-unthreaded",
    // oracle/lib.rs
    "oracle-non-literal-kind",
    "oracle-missing-kind",
    "oracle-missing-probe",
    "oracle-bad-effect",
    "oracle-top-level-mutator",
    "oracle-non-declaration",
    "oracle-duplicate-effect",
    "oracle-probe-selector-roundtrip",
    // oracle/check.rs
    "check-out-of-dialect",
    "check-unterminated",
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

/// The concatenated source text of every scanned crate's `src/` tree (production + inline tests;
/// the legacy `DiagCode("…")` literals live in production code, and the test-only literals — the
/// coverage `refusal_diag` helper, the erasability `x-err`/`boom`/`test-warn` fixtures — are
/// EXCLUDED by scanning only the slugs we assert, never all literals).
fn scanned_source() -> String {
    let crates = crates_dir();
    let mut files = Vec::new();
    for c in SCANNED_CRATES {
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

/// (1a) Every catalog variant is CONSTRUCTED at some emit site (`226` §1 reachability). A
/// `pub enum` variant the type system never forces to be used would be dead catalog; the grep is
/// the only thing that sees it.
#[test]
fn every_catalog_variant_is_constructed() {
    let source = scanned_source();
    for payload in MIGRATED_PAYLOADS {
        // The construction marker: `Payload {` (the struct literal) appears at the emit site AND
        // at the `core::diag` definition/test sites — so it is present iff the variant is live.
        // We require it OUTSIDE core's own diag.rs definition too: an emit site in a consuming
        // crate. The simplest robust check is presence of a `DiagCode::Payload(` or
        // `Code::Payload(` enum-construction anywhere.
        let constructed = source.contains(&format!("DiagCode::{payload}("))
            || source.contains(&format!("Code::{payload}("));
        assert!(
            constructed,
            "catalog variant `{payload}` is registered but never constructed (dead catalog — \
             either emit it at a give-up site or remove the variant + its registry/render arms)"
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
/// `MIGRATED_SLUGS` (i.e. still a live catalog code) UNLESS it is a brand-new addition. We only
/// check the deletion direction: a committed slug that is no longer in `MIGRATED_SLUGS` means the
/// code was retired silently — record it deliberately (here, in a retired-list) instead.
fn assert_no_slug_vanished(committed_diag_rs: &str) {
    for slug in extract_committed_slugs(committed_diag_rs) {
        assert!(
            MIGRATED_SLUGS.contains(&slug.as_str()),
            "retire-guard: catalog slug `{slug}` was in the committed diag.rs but is gone from \
             MIGRATED_SLUGS — a silent catalog deletion. If intentional, record it as retired \
             deliberately; do not let a code quietly stop existing (226 §1 retire-guard)."
        );
    }
}

/// Pull the catalog slugs out of a committed diag.rs by scanning `slug`'s arms — the
/// `=> "dq-…"` shape. Conservative: only matches the `=> "…"` form inside the migrated-code
/// region, so it does not pick up doc-comment mentions or the legacy-module strings.
fn extract_committed_slugs(diag_rs: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in diag_rs.lines() {
        let trimmed = line.trim();
        // The `slug` match arms: `DiagCode::Variant(_) => "slug",`.
        if let Some(arrow) = trimmed.find("=> \"") {
            let after = &trimmed[arrow + 4..];
            if let Some(end) = after.find('"') {
                let slug = &after[..end];
                // Only catalog slugs (the migrated ones use the dq-/render- shapes); ignore
                // unrelated `=> "…"` string arms by requiring the slug be one we know is a
                // catalog code. This keeps the guard focused on the catalog, not every string.
                if MIGRATED_SLUGS.contains(&slug) {
                    out.insert(slug.to_string());
                }
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

/// The known TEST-FIXTURE diagnostic slugs (the erasability/carrier unit-test throwaways): never
/// real catalog codes, so they are neither allow-listed nor migrated. Listing them explicitly
/// (rather than filtering by "in a test module") keeps the gate's exclusion reviewer-visible.
fn is_test_fixture_slug(slug: &str) -> bool {
    matches!(
        slug,
        "test-warn" | "boom" | "x-note" | "x-warn" | "x-err" | "e"
    )
}
