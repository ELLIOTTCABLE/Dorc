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

#![expect(
    clippy::expect_used,
    reason = "the corpus scan is the gate's own precondition; a broken corpus must abort it loudly"
)]

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
/// EMPTY since the records lane gained its production emitter (`306b` §6e): every catalog slug is
/// now case-owned, and this list has nowhere left to shrink to. Keep it — a new code with no case
/// must declare itself here with its trigger surface, and the shrink-only guard then pins the
/// direction from a baseline of zero.
const DEFINING_CASE_RATCHET: &[(&str, &str)] = &[];

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

/// Whether `slug` is CASE-OWNED: some case in the collection is its authoring home — the case
/// named for it, or a multi-component case declaring it in `owns:`
/// (`28L:rul-ownership-declaration-adopted`). Ownership moved to those files at the `283` flip;
/// phase 5 backported the covered codes, so completeness keys to real cases, not `covered()`.
fn is_case_owned(slug: &str) -> bool {
    static OWNERSHIP: std::sync::OnceLock<dorc_loom::CaseOwnership> = std::sync::OnceLock::new();
    OWNERSHIP
        .get_or_init(|| {
            let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
            dorc_loom::corpus_ownership(&dir).expect("the committed corpus resolves one home each")
        })
        .owns(slug)
}

/// The mint recipe, named VERBATIM so a red gate hands the reader the command that repairs it
/// (`288:rul-loom-mint-guarantee`).
const REPAIR_HINT: &str = "Mint its prose home: `dorc-loom scaffold <slug>`, author the case's \
                           when-fires/why and a replay whose output carries the slug, then have \
                           the orchestrator run `dorc-loom publish <case>`.";

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
    // Ceiling 21 = 19 (`28K`'s loading refusals, spending `289:rul-unwritten-ceiling-one-bump`,
    // plus `28K` §2's two positional notices and §4's closure refusal) + the apply lane's two
    // refusals: an invocation that cannot record its own intent, and bytes that cannot be bound
    // to one. Unwritten because a builder authors ZERO prose
    // (`27V:rul-error-authorship-tier`); both carry a `why` naming the remediation register the
    // words are owed.
    assert!(
        unwritten.len() <= 21,
        "more unwritten (`None`) messages ({}) than the pinned ceiling — each is a conductor prose \
         debt; bump this ceiling consciously when a new code lands unwritten: {unwritten:?}",
        unwritten.len()
    );
}
