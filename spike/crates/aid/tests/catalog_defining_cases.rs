//! Per-code **defining-case** coverage (`27V` §3 · the `282`/`283` generation flip). Prose ownership
//! and the committed render TRANSCRIPT now live in the
//! dorc-loom case corpus (`crates/dorc-loom/cases/<slug>.loom`), guarded by the errorloom render-level
//! `fixpoint_check` (`283` §4a). Phase 5 (`283` §5.9) backported the covered codes to those case files
//! and retired the old per-register fragment goldens (`tests/defining_cases/*` + `DORC_DEFINING_BLESS`);
//! this file keeps what stays in `aid`:
//!
//! * **production byte equality** — `render_body_parts` / `render_cli_parts` reproduce their product
//!   render bytes over every payload in [`dorc_aid::fixture`].
//! * **completeness + the coverage RATCHET** (`tc-defining-case-coverage-ratchet`) — every catalog
//!   code is EITHER case-owned (a dorc-loom case file exists, [`is_case_owned`]) OR on the shrink-only
//!   [`DEFINING_CASE_RATCHET`]; the partition is `case-owned ∪ ratchet == every catalog slug`. The
//!   "fires" half stays `diag_tidy::every_catalog_variant_is_constructed` (the emit-site backstop):
//!   delete a code's sole emit and that gate fails.
//!
//! Every ratchet entry carries a one-line trigger surface so a future case is mechanical, not
//! re-derived (conductor rider). The stand-in payload table used to be duplicated here and in
//! `dorc-loom`, kept honest by a drift guard (`28A:rul-keep-covered-with-drift-guard`); both now read
//! the ONE table in `aid`, so there is nothing left to drift.

use dorc_aid::RenderCtx;
use dorc_aid::diag::{self, Diag, DiagCode, WhylogVersionRefused};
use dorc_aid::fixture::canonical_payloads;
use dorc_aid::tagged::RenderPart;
use dorc_core::{BytePos, Interner, Span};

/// The SHRINK-ONLY not-yet-covered allowlist (`tc-defining-case-coverage-ratchet`). Each entry's note
/// is the corruption/trigger surface a future defining case injects, so coverage is mechanical
/// (conductor rider). A code may leave this list ONLY by gaining a defining case; adding a NEW code
/// without one must add it here with its surface note (the completeness gate enforces the
/// partition, `ratchet_only_shrinks` guards the direction against the committed baseline).
const DEFINING_CASE_RATCHET: &[(&str, &str)] = &[
    // ── analyzer give-ups: trigger via a small book/oracle through the pipeline ──
    (
        "cmdsub-inner-nonleaf",
        "book: an effect command inside `$(…)`, e.g. `x=$(id -u)`",
    ),
    (
        "redir-target-top",
        "book: a write-redirect to a dynamic target, `echo x >\"$f\"`",
    ),
    (
        "cfg-top-node",
        "book: an unsupported construct that lowers to a CFG ⊤ node",
    ),
    (
        "cfg-errexit-unknown",
        "book: `set -e` with an unmodeled command in the errexit region",
    ),
    (
        "effect-kind-disagreement",
        "oracle: a check annotation kind ≠ the effect-map kind for a verb",
    ),
    (
        "predict-unterminated",
        "oracle: a check `case` body missing its `esac`",
    ),
    (
        "munge-name-collision",
        "oracle: two source names munging to one sh funcname",
    ),
    (
        "reserved-namespace-squat",
        "book: a funcdef named `nginx__predict` squatting the role namespace",
    ),
    (
        "tolerates-over-identity-dependence",
        "oracle: `tolerates:user` over a body reading `id`/`$USER`",
    ),
    (
        "heavy-context-no-tolerance",
        "oracle: an is_converged reading identity with no tolerance vouch",
    ),
    (
        "lend-map-unknown-dimension",
        "oracle: a `__lend_map` line with an unknown dimension token",
    ),
    (
        "carry-netns-on-net-kernel-forbidden",
        "oracle: `invariant:netns` on a per-netns net-kernel store",
    ),
    (
        "mark-brace-verdict-single-cell",
        "oracle: a brace-alternation `@{a,b}` on a verdict/observe mark",
    ),
    (
        "footprint-incoherent",
        "oracle: a touches() footprint omitting its own effect coordinate",
    ),
    (
        "touches-escalated",
        "book+flag: a payload-bound touches() escalating to host-derivation",
    ),
    (
        "deriv-family-incomplete",
        "probe-results: a derived footprint family missing its deriv-end record",
    ),
    (
        "wrapped-site-adoption-hint",
        "book: a `sudo`-wrapped site whose is_converged lacks tolerates",
    ),
    (
        "resolver-conflict",
        "oracle: two oracle files declaring one kind's resolver",
    ),
    (
        "resolver-provider-collision",
        "oracle: a resolver keyed to a known COMMAND provider name",
    ),
    (
        "reaches-conflict",
        "oracle: two oracle files declaring one kind's reach-function",
    ),
    (
        "reaches-provider-collision",
        "oracle: a reach-function keyed to a known COMMAND provider name",
    ),
    (
        "wrapper-entry-incoherent",
        "oracle: a wrapper whose __enter and __lend_map disagree on argv flow",
    ),
    // ── records deframer: inject a mangled `probe-results.txt` frame ──
    (
        "records-headerless-refused",
        "probe-results: a stream with NO `dorc-records/1` framing at all",
    ),
    (
        "records-glued-line",
        "probe-results: a record line with bytes after its terminal token",
    ),
    (
        "records-header-missing",
        "probe-results: a framed stream whose header line is torn/absent",
    ),
    (
        "records-sentinel-nonce",
        "probe-results: an end-sentinel carrying a foreign nonce",
    ),
    (
        "records-integrity-refused",
        "probe-results: a header failing an integrity key (host/book/attempt)",
    ),
    (
        "records-torn-line",
        "probe-results: a record fragment that lost its terminating write",
    ),
    (
        "records-alien-line",
        "probe-results: a non-nonce record line mixed into the stream",
    ),
    (
        "records-late-line",
        "probe-results: a record line after the end-sentinel",
    ),
];

#[test]
fn defining_case_parts_match_product_renders() {
    let interner = Interner::default();
    let src = "make install >/etc/motd\nldconfig\n";
    for (slug, code) in canonical_payloads() {
        let diag = Diag::new(code, Span::new(BytePos(0), BytePos(4)));
        assert_eq!(
            diag::render_body_parts(&RenderCtx::production(), &diag, &interner).text(),
            diag::render_body(&diag, &interner),
            "defining case `{slug}`: body parts drifted"
        );
        assert_eq!(
            diag::render_cli_parts(&RenderCtx::production(), &diag, src, "book.sh", &interner)
                .text(),
            diag::render_cli(&diag, src, "book.sh", &interner),
            "defining case `{slug}`: cli parts drifted"
        );
    }
}

#[test]
fn body_parts_keep_empty_parameter_identity() {
    let diag = Diag::new(
        DiagCode::WhylogVersionRefused(WhylogVersionRefused {
            found: String::new(),
        }),
        Span::new(BytePos(0), BytePos(1)),
    );
    let parts = diag::render_body_parts(&RenderCtx::production(), &diag, &Interner::default());
    assert_eq!(parts.text(), diag::render_body(&diag, &Interner::default()));
    assert!(parts.parts().iter().any(|part| matches!(
        part,
        RenderPart::ParamValue {
            text,
            param: "found",
            ..
        } if text.is_empty()
    )));
}

/// Whether `slug` is CASE-OWNED: a defining case file exists in the dorc-loom corpus (mirrors the
/// private predicate in `catalog.rs`). Ownership moved to those files at the `283` flip; phase 5
/// backported the covered codes, so completeness keys to real case files, not the `covered()` list.
fn is_case_owned(slug: &str) -> bool {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(format!("{slug}.loom"))
        .exists()
}

/// The mint recipe, named VERBATIM so a red gate hands the reader the command that repairs it
/// (`288:rul-loom-mint-guarantee`).
const REPAIR_HINT: &str = "Mint its prose home: `dorc-loom scaffold <slug>`, author the case's \
                           when-fires/why and a replay whose output carries the slug, then have \
                           the orchestrator run `dorc-loom promote <case>`.";

/// A stand-in exists to give a defining case a world; completeness (g12) keys to case FILES, so
/// nothing else forces the fixture table's slugs to stay case-owned — a code could leave the case
/// corpus while `aid::fixture` still claimed to construct it (`28A` §2u).
#[test]
fn every_fixture_slug_is_case_owned() {
    for (slug, _) in canonical_payloads() {
        assert!(
            is_case_owned(slug),
            "`aid::fixture` constructs `{slug}` but no defining case owns it — delete the entry, \
             or mint the case"
        );
    }
}

/// COMPLETENESS (`AID-NEEDS:law-one-defining-case-per-code`, ratchet-tempered): every catalog slug is
/// EITHER case-owned (a dorc-loom case file, fixpoint-protected) OR on the shrink-only
/// [`DEFINING_CASE_RATCHET`] — never silently uncovered. Also: no slug is in BOTH (a case-owned code
/// must leave the ratchet). This is the phase-5 collapse of the transient `fragment-covered` third
/// state (`283` §4d): the gate now trusts real case files, not the `covered()` payload table.
#[test]
fn every_code_is_case_owned_or_ratcheted() {
    use std::collections::BTreeSet;
    assert!(
        REPAIR_HINT.contains("dorc-loom scaffold"),
        "the completeness failure must name the repair command verbatim; a reword may not drop it"
    );
    let ratchet: BTreeSet<&str> = DEFINING_CASE_RATCHET.iter().map(|(s, _)| *s).collect();
    let catalog: BTreeSet<&str> = dorc_aid::catalog::CATALOG.iter().map(|e| e.slug).collect();
    for e in dorc_aid::catalog::CATALOG {
        let owned = is_case_owned(e.slug);
        assert!(
            owned || ratchet.contains(e.slug),
            "catalog code `{}` has no dorc-loom case file and is not on DEFINING_CASE_RATCHET — \
             {REPAIR_HINT} (Or, for a legacy code only, add a ratchet entry with its trigger \
             surface; the ratchet is shrink-only. Silent partial coverage is not acceptable, \
             27V §3.)",
            e.slug
        );
        assert!(
            !(owned && ratchet.contains(e.slug)),
            "`{}` is BOTH case-owned and ratcheted — a case-owned code must leave the ratchet",
            e.slug
        );
    }
    // The ratchet may not name a retired code (keeps it honest as the catalog shrinks).
    for slug in &ratchet {
        assert!(
            catalog.contains(slug),
            "DEFINING_CASE_RATCHET names `{slug}`, which is not a catalog code (stale entry — remove it)"
        );
    }
}

/// The ratchet is SHRINK-ONLY (`tc-defining-case-coverage-ratchet`): it may never GROW past its
/// committed baseline (a new code must ship a covered case, not pad the ratchet). Best-effort against
/// `git show HEAD` — skipped when git is unavailable, like the `diag_tidy` retire-guard.
#[test]
fn ratchet_only_shrinks() {
    use std::process::Command;
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let rel = "crates/aid/tests/catalog_defining_cases.rs";
    let spike = manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("crates/aid -> crates -> spike");
    let committed = ["HEAD:spike/", "HEAD:"].iter().find_map(|prefix| {
        let out = Command::new("git")
            .arg("-C")
            .arg(spike)
            .arg("show")
            .arg(format!("{prefix}{rel}"))
            .output()
            .ok()?;
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
    });
    let Some(committed) = committed else {
        eprintln!("ratchet_only_shrinks: no committed baseline (new file / no git) — skipping");
        return;
    };
    let baseline = count_ratchet_entries(&committed);
    assert!(
        DEFINING_CASE_RATCHET.len() <= baseline,
        "DEFINING_CASE_RATCHET GREW ({} entries vs the committed {baseline}) — it is shrink-only; a \
         new code must ship a covered defining case, never pad the ratchet (27V §3)",
        DEFINING_CASE_RATCHET.len()
    );
}

/// Count the `("slug", "note")` rows inside the committed `DEFINING_CASE_RATCHET` literal by shape
/// (bounded to the `const DEFINING_CASE_RATCHET` block), so the guard reads the baseline without
/// importing the committed source. Robust to rustfmt wrapping: an entry opener is a line whose
/// trimmed form is either the wrapped `(` or the single-line `("…`.
fn count_ratchet_entries(src: &str) -> usize {
    let Some(start) = src.find("const DEFINING_CASE_RATCHET") else {
        return usize::MAX; // unreadable ⇒ never trips the <= assert (conservative)
    };
    let body = &src[start..];
    let end = body.find("];").map_or(body.len(), |i| i);
    body[..end]
        .lines()
        .filter(|l| {
            let t = l.trim();
            t == "(" || t.starts_with("(\"")
        })
        .count()
}

/// Unwritten prose is `None` and renders GREPPABLY (`27V` §3 · `283:dec-message-becomes-option`): an
/// unwritten `message` is stored as `None` (never a near-miss string), and the render seat synthesizes
/// EXACTLY `[unwritten: <slug>]` — the defining-case prose goldens pin that synthesized render
/// byte-for-byte, so this gate only has to count-and-pin the debt. The count shrinks as prose is
/// authored and never silently grows (a bump is a conscious conductor act).
#[test]
fn unwritten_renders_are_greppable_and_pinned() {
    let unwritten: Vec<&str> = dorc_aid::catalog::CATALOG
        .iter()
        .filter(|e| e.message.is_none())
        .map(|e| e.slug)
        .collect();
    // Ceiling 15 = the prior 6 + the 7 lint codes `288` §5 moved into the registry + 2 headroom
    // (`289:rul-unwritten-ceiling-one-bump`, the lane's ONE conscious bump). All seven `sm `-migrated
    // (`289:rul-sm-where-ancestor-exists`), so this is expected to sit slack at 6, never met.
    assert!(
        unwritten.len() <= 15,
        "more unwritten (`None`) messages ({}) than the pinned ceiling — each is a conductor prose \
         debt; bump this ceiling consciously when a new code lands unwritten: {unwritten:?}",
        unwritten.len()
    );
}
