//! `rul-ascii-output-forever` (`28E` §0, human-typed: "no unicode, ever. period. anywhere...
//! permanently") over the two generated registries AND the committed rendered corpus.
//!
//! Two independent surfaces, because they fail independently:
//!
//! * REGISTRY — the catalog's `example`/`message`/`help` and the arrangement registry's `words`.
//!   Dev-facing metadata (`when_fires`/`when_used`/`why`) is deliberately out: it is read
//!   in-tree, never printed to a user. Residue is named per ROW in [`ASCII_SWEEP_ALLOWLIST`],
//!   shrink-only in BOTH directions: an unlisted row may not carry non-ASCII, and a listed row
//!   that has been swept must leave.
//! * CORPUS — every byte the engine MINTED into a committed transcript or golden. This is the
//!   surface a registry gate structurally cannot see: a `{{detail}}` payload built at an emit
//!   site, an artifact banner, a lint envelope. It is separated from echoed fixture bytes by the
//!   byte-floor itself (`two-surfaces`): the artifact reproduces the book verbatim, so a
//!   non-ASCII line that also appears in the case's own inputs is the AUTHOR's, not ours.
//!
//! `28G` Phase W1's `the_why_surface_renders_pure_ascii` (in `dorc-cli`) stays as the why-lane's
//! own narrower pin.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

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
    (
        "cli-help-page",
        "JARGON ⊤ (arrangement): `a syntax-level ⊤-reject / CFG ⊤-node`",
    ),
    (
        "lint-source-unmodeled-inventory",
        "JARGON ⊤ (arrangement): `book ⊤-wall inventory`",
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
            rows.push((entry.slug, "message", *message.text()));
        }
        if let Some(help) = entry.help.written() {
            rows.push((entry.slug, "help", *help.text()));
        }
    }
    for entry in ARRANGEMENTS {
        for word in entry.words.map_or(&[][..], |tier| *tier.text()) {
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
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spike = manifest
        .parent()
        .and_then(Path::parent)
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

// ===========================================================================
// The corpus surface: engine-minted bytes in the committed transcripts/goldens
// ===========================================================================

/// The glyphs a rendered surface may still carry, corpus-wide. Both stand for a CONCEPT whose
/// replacement is authored English rather than a substitution (`⊤` the wall, `⊄` not-a-subset),
/// so they are the plain-language prose pass's to spend
/// (`AID-NEEDS:law-plain-language-surfaces`), not a sweep's. Everything the mechanical
/// punctuation map covers is absent by construction, which is exactly what makes this list a
/// gate: adding a glyph here is a visible act.
const CORPUS_JARGON_GLYPHS: &[char] = &['⊤', '⊄'];

/// Cases whose RENDER is non-ASCII because the thing under test is non-ASCII. A row here is a
/// statement that the bytes are the fixture's, not the engine's voice.
///
/// Empty, and the emptiness is load-bearing: a quotation of somebody else's bytes reaches a
/// laid-out surface through the display seat, which escapes anything outside printable ASCII. A
/// case that quotes an illegal name therefore renders the escape, not the name.
const CORPUS_CASE_ALLOWLIST: &[(&str, &str)] = &[];

/// One committed case: its engine-rendered region and the input bytes that region may legally
/// echo. Splitting the two is the whole trick — see the module header.
struct CorpusCase {
    name: String,
    rendered: String,
    inputs: String,
}

/// Every case in the two central runners' flat collections (`crates/*/tests/`), classified by
/// SHAPE exactly as `flat-test-tree-and-loom-placement` classifies it. A nested `tests/fixtures/`
/// tree is an `.rs` test's fixture space, never a case, and is deliberately not scanned: its
/// transcripts are synthetic inputs to the loom compiler, not renders anyone reads.
fn corpus_cases(spike: &Path) -> Vec<CorpusCase> {
    let mut cases = Vec::new();
    let Ok(crates) = std::fs::read_dir(spike.join("crates")) else {
        return cases;
    };
    let mut roots: Vec<PathBuf> = crates
        .flatten()
        .map(|entry| entry.path().join("tests"))
        .filter(|dir| dir.is_dir())
        .collect();
    roots.sort();
    for root in roots {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths {
            if path.extension().is_some_and(|ext| ext == "loom") {
                if let Some(case) = loom_case(&path) {
                    cases.push(case);
                }
            } else if path.is_dir() {
                cases.extend(dir_case(&path));
            }
        }
    }
    cases
}

/// A single-file loom: the transcript is everything from the `-- replay --` section on, and the
/// txtar sections before it are the case's own inputs. A loom with no replay section renders
/// nothing and contributes nothing.
fn loom_case(path: &Path) -> Option<CorpusCase> {
    let text = std::fs::read_to_string(path).ok()?;
    let cut = text.find("\n-- replay --\n")?;
    Some(CorpusCase {
        name: case_name(path),
        rendered: text[cut..].to_owned(),
        inputs: text[..cut].to_owned(),
    })
}

/// A round-trip case dir: `expected.out` is the render, and every other file in the tree
/// (`book.sh`, oracles, `mocks/`, a nested multi-file loom) is input the artifact may echo.
fn dir_case(dir: &Path) -> Option<CorpusCase> {
    let golden = dir.join("expected.out");
    let rendered = std::fs::read_to_string(&golden).ok()?;
    let mut inputs = String::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(next) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&next) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path != golden {
                inputs.push_str(&std::fs::read_to_string(&path).unwrap_or_default());
                inputs.push('\n');
            }
        }
    }
    Some(CorpusCase {
        name: case_name(dir),
        rendered,
        inputs,
    })
}

fn case_name(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

/// THE corpus gate: a non-ASCII line in a rendered surface is legal only when it is an echo of
/// the case's own input bytes, or carries nothing but named jargon. The failure prints the line,
/// because the repair differs by origin: an echoed line means the FIXTURE wants sweeping (and
/// the byte-floor means the render will follow), while a minted one means an EMITTER does.
///
/// The floors are the discovery-floor law (`count-drifts`): both runners can be pointed at a
/// wrong root and find nothing, and a gate over zero cases passes vacuously. Non-empty is the
/// floor; a count would drift.
#[test]
fn rendered_corpus_carries_no_minted_non_ascii() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spike = manifest
        .parent()
        .and_then(Path::parent)
        .expect("crates/aid -> crates -> spike");
    let excused: BTreeSet<&str> = CORPUS_CASE_ALLOWLIST
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let cases = corpus_cases(spike);
    let mut rendered_lines: usize = 0;
    for case in &cases {
        let echoed: BTreeSet<&str> = case.inputs.lines().map(str::trim_end).collect();
        for line in case.rendered.lines() {
            rendered_lines = rendered_lines.saturating_add(1);
            if line.is_ascii() || excused.contains(case.name.as_str()) {
                continue;
            }
            let unexplained: String = line
                .chars()
                .filter(|c| !c.is_ascii() && !CORPUS_JARGON_GLYPHS.contains(c))
                .collect();
            if unexplained.is_empty() || echoed.contains(line.trim_end()) {
                continue;
            }
            panic!(
                "`{}` renders engine-minted non-ASCII {unexplained:?} \
                 (`28E:rul-ascii-output-forever`): {line:?}\n\
                 The line is not an echo of this case's own inputs, so an EMITTER produced it: \
                 sweep the emit site with the punctuation map, then re-bless. If the glyph needs \
                 authored English instead, it belongs to the prose pass, not here.",
                case.name
            );
        }
    }
    assert!(
        cases.len() > 100,
        "only {} cases discovered -- the walk is not looking at the flat case collections",
        cases.len()
    );
    assert!(
        rendered_lines > 2000,
        "only {rendered_lines} rendered lines reached -- the case shapes are being misread"
    );
}

/// The excuse-list stays honest in the same shrink-only direction the registry list does: a case
/// that no longer renders non-ASCII must leave, and a row naming no discovered case is stale.
#[test]
fn no_corpus_allowlist_entry_is_stale() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spike = manifest
        .parent()
        .and_then(Path::parent)
        .expect("crates/aid -> crates -> spike");
    let cases = corpus_cases(spike);
    for (name, why) in CORPUS_CASE_ALLOWLIST {
        let case = cases.iter().find(|case| case.name == *name);
        let Some(case) = case else {
            panic!("CORPUS_CASE_ALLOWLIST names `{name}`, which is not a discovered case (stale)");
        };
        assert!(
            !case.rendered.is_ascii(),
            "CORPUS_CASE_ALLOWLIST still excuses `{name}` ({why}) but its render is already pure \
             ASCII -- the list is shrink-only; delete the entry"
        );
    }
}
