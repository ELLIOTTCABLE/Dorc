//! `rul-ascii-output-forever` (`28E` §0, human-typed: "no unicode, ever. period. anywhere...
//! permanently") over BOTH generated registries.
//!
//! The swept surface is the product-prose registers — the catalog's `example`/`message`/`help`
//! and the arrangement registry's `words`. Dev-facing metadata (`when_fires`/`when_used`/`why`)
//! is deliberately out: it is read in-tree, never printed to a user.
//!
//! What survives the sweep is named, one row at a time, in [`ASCII_SWEEP_ALLOWLIST`], and the
//! gates below make that list shrink-only in BOTH directions: an unlisted row may not carry
//! non-ASCII, and a listed row that has been swept must leave. `28G` Phase W1's
//! `the_why_surface_renders_pure_ascii` (in `dorc-cli`) stays as the why-lane's own narrower pin.

use std::collections::BTreeSet;

use dorc_aid::arrangement::ARRANGEMENTS;
use dorc_aid::catalog::CATALOG;

/// The SHRINK-ONLY residue of the mechanical ASCII sweep. Two reasons appear, and neither is a
/// licence to leave new non-ASCII behind:
///
/// * JARGON — the glyph stands for a concept (`⊤` the wall, `⊄` not-a-subset) whose replacement
///   is authored English, not a substitution. Owned by the plain-language prose pass
///   (`AID-NEEDS:law-plain-language-surfaces`), which several `why` registers already cite.
/// * PAYLOAD — the row's `example` is a filled sample whose non-ASCII arrives in a runtime VALUE.
///   Only the deliberately-illegal fixture name survives here: every other payload-borne glyph
///   was swept AT ITS EMITTER (`28F` lane-ascii-emitters) and the sample followed in lockstep,
///   which is the only honest order — a sample swept alone would misquote its own emitter.
///
/// A row leaves by being swept, not by being re-noted; `no_allowlist_entry_is_stale` fails the
/// moment an entry stops being needed.
const ASCII_SWEEP_ALLOWLIST: &[(&str, &str)] = &[
    // ── JARGON: ⊤/⊄ in user prose, awaiting authored words ──
    ("redir-target-top", "JARGON ⊤: `the write joins ⊤`"),
    (
        "depth-2-positional-unthreaded",
        "JARGON ⊤: `the inner body's positional is ⊤`",
    ),
    (
        "lend-map-unknown-dimension",
        "JARGON ⊤: `the dimension it meant to answer stays ⊤ (walls)`",
    ),
    (
        "unmodeled-wall-inventory",
        "JARGON ⊤: `N modeled ⊤-walls in this book`",
    ),
    (
        "cli-help-page",
        "JARGON ⊤ (arrangement): `a syntax-level ⊤-reject / CFG ⊤-node`",
    ),
    (
        "lint-source-unmodeled-inventory",
        "JARGON ⊤ (arrangement): `book ⊤-wall inventory`",
    ),
    // ── JARGON + PAYLOAD: a `{{detail}}` sample that also carries jargon ──
    (
        "cfg-top-node",
        "JARGON ⊤ + PAYLOAD: analysis/src/cfg.rs lower_top detail",
    ),
    (
        "cfg-errexit-unknown",
        "JARGON ⊤ + PAYLOAD: analysis/src/cfg.rs errexit detail",
    ),
    (
        "footprint-incoherent",
        "JARGON ⊄ + PAYLOAD: the touches()-footprint detail",
    ),
    // ── PAYLOAD: the fixture owns the words ──
    (
        "munge-name-invalid",
        "PAYLOAD: the deliberately-illegal non-ASCII name the case lints",
    ),
];

/// Every product-prose register in both generated locks, as `(slug, field, text)`. Nothing here
/// reads a dev-facing metadata field: those stay free to carry the corpus's own notation.
fn product_prose() -> Vec<(&'static str, &'static str, &'static str)> {
    let mut rows: Vec<(&str, &str, &str)> = Vec::new();
    for entry in CATALOG {
        rows.push((entry.slug, "example", entry.example));
        if let Some(message) = entry.message {
            rows.push((entry.slug, "message", message));
        }
        if let Some(help) = entry.help {
            rows.push((entry.slug, "help", help));
        }
    }
    for entry in ARRANGEMENTS {
        for word in entry.words.words().unwrap_or(&[]) {
            rows.push((entry.slug, "words", word));
        }
    }
    rows
}

fn slugs_carrying_non_ascii() -> BTreeSet<&'static str> {
    product_prose()
        .into_iter()
        .filter(|(_, _, text)| !text.is_ascii())
        .map(|(slug, _, _)| slug)
        .collect()
}

/// The gate itself: a product-prose register is pure ASCII unless its row is named above. The
/// failure names the field and the offending characters, because the repair is either the
/// punctuation map or an authored respell and the glyph decides which. The closing count is the
/// vacuity floor both registries need — they are generated, and an empty read would pass.
#[test]
fn product_prose_registers_are_pure_ascii() {
    let allowed: BTreeSet<&str> = ASCII_SWEEP_ALLOWLIST
        .iter()
        .map(|(slug, _)| *slug)
        .collect();
    let mut checked: usize = 0;
    for (slug, field, text) in product_prose() {
        checked = checked.saturating_add(1);
        if text.is_ascii() || allowed.contains(slug) {
            continue;
        }
        let glyphs: String = text.chars().filter(|c| !c.is_ascii()).collect();
        panic!(
            "`{slug}` {field} carries non-ASCII {glyphs:?} — product output is pure ASCII \
             (`28E:rul-ascii-output-forever`). Apply the punctuation map (em/en dash, ellipsis, \
             curly quotes, section sign, arrows, NBSP), or, if the glyph needs authored English, \
             add a row to ASCII_SWEEP_ALLOWLIST with its reason."
        );
    }
    assert!(
        checked > 100,
        "only {checked} product-prose registers reached — the registries are not where this gate looks"
    );
}

/// The shrink direction the list count cannot express: an entry that is no longer needed must be
/// DELETED, not left as a permanent excuse. Also refuses an entry naming no row at all, so a
/// retired code cannot park here.
#[test]
fn no_allowlist_entry_is_stale() {
    let carrying = slugs_carrying_non_ascii();
    let known: BTreeSet<&str> = CATALOG
        .iter()
        .map(|entry| entry.slug)
        .chain(ARRANGEMENTS.iter().map(|entry| entry.slug))
        .collect();
    for (slug, why) in ASCII_SWEEP_ALLOWLIST {
        assert!(
            known.contains(slug),
            "ASCII_SWEEP_ALLOWLIST names `{slug}`, which is neither a catalog code nor an \
             arrangement slug (stale entry — remove it)"
        );
        assert!(
            carrying.contains(slug),
            "ASCII_SWEEP_ALLOWLIST still excuses `{slug}` ({why}) but its prose is already pure \
             ASCII — the list is shrink-only; delete the entry"
        );
    }
}

/// The count half of shrink-only, against the committed baseline (`ratchet_only_shrinks` is the
/// model, git-absence included): a fresh non-ASCII register must be swept, never excused.
#[test]
fn ascii_allowlist_only_shrinks() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let spike = manifest
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/aid -> crates -> spike");
    let rel = "crates/aid/tests/ascii_output.rs";
    let committed = ["HEAD:spike/", "HEAD:"].iter().find_map(|prefix| {
        let out = std::process::Command::new("git")
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
        eprintln!(
            "ascii_allowlist_only_shrinks: no committed baseline (new file / no git) — skipping"
        );
        return;
    };
    let baseline = count_allowlist_entries(&committed);
    assert!(
        ASCII_SWEEP_ALLOWLIST.len() <= baseline,
        "ASCII_SWEEP_ALLOWLIST GREW ({} entries vs the committed {baseline}) — it is shrink-only; \
         new product prose ships ASCII (`28E:rul-ascii-output-forever`)",
        ASCII_SWEEP_ALLOWLIST.len()
    );
}

/// Count `("slug", "why")` rows in the committed literal by shape, bounded to the const's block,
/// so the guard reads its own baseline without importing committed source. Robust to rustfmt
/// wrapping: an entry opener is a trimmed `(` or `("…`.
fn count_allowlist_entries(src: &str) -> usize {
    let Some(start) = src.find("const ASCII_SWEEP_ALLOWLIST") else {
        return usize::MAX; // unreadable => never trips the <= assert (conservative)
    };
    let body = &src[start..];
    let end = body.find("];").unwrap_or(body.len());
    body[..end]
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed == "(" || trimmed.starts_with("(\"")
        })
        .count()
}
