//! `SLUGS.md`, a machine-generated index of every hyphenated slug the prose corpus defines or
//! cites. READ the file, never edit it.
//!
//! WHY THIS EXISTS (there is no design document for it; this is the durable home). The corpus
//! (`Research/plans`, `Research/notes`, the root and steering docs — hundreds of markdown files,
//! over a million words) names its rulings with hyphenated full-word slugs and cross-cites them
//! as `docID:slug`. "Where is X ruled, is it still live, and what else discusses it?" was a
//! whole-corpus re-read per question, and hand-maintained index documents rot because an agent
//! never updates a document it never saw. So this file is DERIVED from the corpus on every commit
//! and committed beside it: a row exists exactly when the corpus defines or cites its slug, so the
//! list and the corpus stay synonymous. It is an INSTRUMENT that reports, never a gate on content
//! (`spike/CLAUDE.md:lexical-fences-are-human-ack-instruments` — scans over textual material are
//! ordinary instruments). To "remove" a slug you mark it superseded where it is defined, and the
//! row then shows that. The way it earns its keep is greppability: a grep for a slug, or a word in
//! a slug, lands on the compact row beside the prose hits.
//!
//! Each row also carries an ADVISORY `near:` line — a few corpus spans that discuss the slug's
//! concept WITHOUT citing it, so a design effort that strayed into existing territory under a new
//! name surfaces next to the canonical key. The near line is allowed to go stale: it recomputes
//! only when the row's mechanical lines change, and is carried forward verbatim otherwise
//! (`slug_near`).
//!
//! Determinism: LF endings, `/` separators, rows and IDs sorted bytewise, identical bytes from
//! identical inputs on every platform — the gate runs Windows and WSL. The mechanical stage reads
//! the tree and writes one file; it never reads the clock, network, or randomness.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::ExitCode;

use crate::corpus::{ch, id_of, scanned, starts_with, take_id};
use crate::slug_near::{self, NearRanker};

/// The generated file, at the repo root. Excluded from its own scan.
const OUTPUT: &str = "SLUGS.md";

/// One place a slug is defined: a display path, a 1-based line, the heading/bold-lead text, and the
/// paragraph that follows (query material for the advisory `near:` line).
#[derive(Clone)]
struct DefSite {
    path: String,
    line: u32,
    text: String,
    para: String,
}

/// Everything the corpus says about one slug.
#[derive(Default)]
struct SlugRow {
    /// Definition sites, later sorted by `(path, line)`.
    defs: Vec<DefSite>,
    /// Old slugs renamed INTO this one, via a `(né …)` edge on a definition line.
    akas: BTreeSet<String>,
    /// A superseded marker within three lines of a definition, verbatim/trimmed/truncated.
    superseded: Option<String>,
    /// Distinct docIDs that cite this slug.
    citing: BTreeSet<String>,
    /// Total citation occurrences across the corpus.
    cite_count: u32,
    /// When this slug is itself an OLD name, the slug it was renamed TO.
    rename_to: Option<String>,
}

// ── slug and citation grammar (hand-rolled, dependency-free, mirroring `docids`) ─────────────────

/// A SLUG token: `[a-z][a-z0-9]*(-[a-z0-9]+){2,}` — three or more hyphen-joined lowercase parts.
/// The three-part floor is what keeps two-word emphasis (`well-tested`) and option names
/// (`--dry-run`) out while admitting real keys (`dec-timing-cache`).
fn take_slug(chars: &[char], i: usize) -> Option<(String, usize)> {
    if !ch(chars, i).is_some_and(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let mut end = i.saturating_add(1);
    while ch(chars, end).is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        end = end.saturating_add(1);
    }
    let mut hyphens = 0_u32;
    while ch(chars, end) == Some('-')
        && ch(chars, end.saturating_add(1))
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        hyphens = hyphens.saturating_add(1);
        end = end.saturating_add(1);
        while ch(chars, end).is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
            end = end.saturating_add(1);
        }
    }
    if hyphens < 2 {
        return None;
    }
    Some((chars.get(i..end)?.iter().collect(), end))
}

/// The end index past a citation's docID prefix (`271`, `30U`, `notes/307`, `KNOBS`,
/// `ANALYZER-NEEDS`), or `None`. Numeric IDs need three characters, matching `docids`, so `15:30`
/// stays out; uppercase stems need three too.
fn take_prefix(chars: &[char], i: usize) -> Option<usize> {
    for dir in ["notes/", "plans/"] {
        if starts_with(chars, i, dir)
            && let Some((_, end)) = take_id(chars, i.saturating_add(dir.len()))
        {
            return Some(end);
        }
    }
    if let Some((id, end)) = take_id(chars, i)
        && id.chars().count() >= 3
    {
        return Some(end);
    }
    take_stem(chars, i)
}

/// An uppercase root/steering stem: `[A-Z][A-Z0-9_-]*`, at least three characters
/// (`KNOBS`, `AID-NEEDS`, `FORFEITS`).
fn take_stem(chars: &[char], i: usize) -> Option<usize> {
    if !ch(chars, i).is_some_and(|c| c.is_ascii_uppercase()) {
        return None;
    }
    let mut end = i.saturating_add(1);
    while ch(chars, end)
        .is_some_and(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' || c == '-')
    {
        end = end.saturating_add(1);
    }
    (end.saturating_sub(i) >= 3).then_some(end)
}

/// A `docID:slug` citation starting at `i`, returning the cited slug and the span end.
fn take_citation(chars: &[char], i: usize) -> Option<(String, usize)> {
    let after = take_prefix(chars, i)?;
    if ch(chars, after) != Some(':') {
        return None;
    }
    take_slug(chars, after.saturating_add(1))
}

/// Every maximal slug-shaped token on one line, each flagged QUALIFIED when it is a `docID:slug`
/// citation and bare otherwise. Both forms count toward a row's `cited:` line; only the qualified
/// form mints a row, so the row set stays `defined ∪ qualified-cited` while a bare mention only
/// augments an existing row. A token never starts mid-word, keeping the scanner out of
/// `AES256:foo`-shaped interiors.
fn slug_occurrences(line: &str) -> Vec<(String, bool)> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0_usize;
    while i < chars.len() {
        let fresh = i == 0 || !ch(&chars, i.saturating_sub(1)).is_some_and(char::is_alphanumeric);
        if fresh {
            if let Some((slug, end)) = take_citation(&chars, i) {
                out.push((slug, true));
                i = end.max(i.saturating_add(1));
                continue;
            }
            if let Some((slug, end)) = take_slug(&chars, i) {
                out.push((slug, false));
                i = end.max(i.saturating_add(1));
                continue;
            }
        }
        i = i.saturating_add(1);
    }
    out
}

/// A DEFINITION on this line — a markdown heading (`## slug`, optionally backticked) or the bold
/// lead of a list item (`- **slug**`) — returning the slug and the trimmed/truncated trailing text.
fn definition(line: &str) -> Option<(String, String)> {
    let chars: Vec<char> = line.chars().collect();
    if let Some((slug, after)) = heading_def(&chars) {
        return Some((slug, rest(&chars, after)));
    }
    if let Some((slug, after)) = list_bold_def(&chars) {
        return Some((slug, rest(&chars, after)));
    }
    None
}

fn heading_def(chars: &[char]) -> Option<(String, usize)> {
    let mut i = 0_usize;
    let mut hashes = 0_u32;
    while ch(chars, i) == Some('#') {
        i = i.saturating_add(1);
        hashes = hashes.saturating_add(1);
    }
    if !(2..=6).contains(&hashes) || ch(chars, i) != Some(' ') {
        return None;
    }
    while ch(chars, i) == Some(' ') {
        i = i.saturating_add(1);
    }
    let backtick = ch(chars, i) == Some('`');
    if backtick {
        i = i.saturating_add(1);
    }
    let (slug, mut end) = take_slug(chars, i)?;
    if backtick {
        if ch(chars, end) != Some('`') {
            return None;
        }
        end = end.saturating_add(1);
    }
    Some((slug, end))
}

fn list_bold_def(chars: &[char]) -> Option<(String, usize)> {
    let mut i = 0_usize;
    while matches!(ch(chars, i), Some(' ' | '\t')) {
        i = i.saturating_add(1);
    }
    match ch(chars, i) {
        Some('-' | '*') => i = i.saturating_add(1),
        Some(c) if c.is_ascii_digit() => {
            while ch(chars, i).is_some_and(|c| c.is_ascii_digit()) {
                i = i.saturating_add(1);
            }
            if ch(chars, i) != Some('.') {
                return None;
            }
            i = i.saturating_add(1);
        }
        _ => return None,
    }
    if ch(chars, i) != Some(' ') {
        return None;
    }
    while ch(chars, i) == Some(' ') {
        i = i.saturating_add(1);
    }
    if !(ch(chars, i) == Some('*') && ch(chars, i.saturating_add(1)) == Some('*')) {
        return None;
    }
    i = i.saturating_add(2);
    let (slug, end) = take_slug(chars, i)?;
    if !(ch(chars, end) == Some('*') && ch(chars, end.saturating_add(1)) == Some('*')) {
        return None;
    }
    Some((slug, end.saturating_add(2)))
}

/// Old slugs named by a `(né …)` / `(née …)` / `(nee …)` / `(ne …)` edge on a definition line,
/// with or without backticks. All four spellings are equal — no accent is required. Scoped to
/// definition lines by the caller, so a prose `(nee 22F-fd6)` (not a slug anyway) never mints an
/// edge. Markers are matched longest-first (`nee` before `ne`) so the bare form never shadows it.
fn rename_edges(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0_usize;
    while i < chars.len() {
        let fresh = i == 0 || !ch(&chars, i.saturating_sub(1)).is_some_and(char::is_alphanumeric);
        if fresh {
            for marker in ["née", "né", "nee", "ne"] {
                if starts_with(&chars, i, marker) {
                    let mut j = i.saturating_add(marker.chars().count());
                    if ch(&chars, j) == Some(' ') {
                        while ch(&chars, j) == Some(' ') {
                            j = j.saturating_add(1);
                        }
                        if ch(&chars, j) == Some('`') {
                            j = j.saturating_add(1);
                        }
                        if let Some((old, _)) = take_slug(&chars, j) {
                            out.push(old);
                        }
                    }
                    break;
                }
            }
        }
        i = i.saturating_add(1);
    }
    out
}

/// A superseded marker within three lines of the definition at `n` (before or after, inclusive),
/// verbatim/trimmed/truncated. `supersedes` is deliberately NOT matched — the active verb marks the
/// LIVE slug, not a retired one.
fn superseded_near(lines: &[&str], n: usize) -> Option<String> {
    let lo = n.saturating_sub(3);
    let hi = n.saturating_add(3);
    (lo..=hi).find_map(|k| {
        let line = lines.get(k)?;
        line.to_lowercase()
            .contains("superseded")
            .then(|| truncate(line.trim()))
    })
}

/// Trim to at most 100 characters, appending `…` when the original was longer.
fn truncate(s: &str) -> String {
    let mut out: String = s.chars().take(100).collect();
    if s.chars().nth(100).is_some() {
        out.push('…');
    }
    out
}

fn rest(chars: &[char], after: usize) -> String {
    let tail: String = chars.get(after..).unwrap_or_default().iter().collect();
    truncate(tail.trim())
}

/// The paragraph immediately below a definition at line `n`: subsequent non-blank lines joined,
/// capped at ~600 characters. Query material for the advisory `near:` line only.
fn following_paragraph(lines: &[&str], n: usize) -> String {
    let mut para = String::new();
    let mut k = n.saturating_add(1);
    while let Some(line) = lines.get(k) {
        if line.trim().is_empty() {
            break;
        }
        if !para.is_empty() {
            para.push(' ');
        }
        para.push_str(line.trim());
        if para.chars().count() >= 600 {
            break;
        }
        k = k.saturating_add(1);
    }
    para.chars().take(600).collect()
}

// ── extraction ───────────────────────────────────────────────────────────────────────────────

/// The docID a corpus filename or root/steering stem encodes: `271-block-….md` → `271`, else the
/// display path with `.md` stripped (`KNOBS.md` → `KNOBS`, `spike/CLAUDE.md` → `spike/CLAUDE`).
pub(crate) fn doc_id_of(display: &str) -> String {
    let filename = display.rsplit('/').next().unwrap_or(display);
    id_of(filename).map_or_else(
        || display.strip_suffix(".md").unwrap_or(display).to_owned(),
        str::to_owned,
    )
}

/// The corpus files to scan: the `docids` universe minus quarantine (never opened by the walker),
/// minus `Research/corpora/` and `SLUGS.md` itself.
fn corpus_files(root: &Path) -> Vec<(String, String)> {
    let mut quarantined = Vec::new();
    scanned(root, &mut quarantined)
        .into_iter()
        .filter_map(|path| {
            let display = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            if display == OUTPUT || display.starts_with("Research/corpora/") {
                return None;
            }
            let text = std::fs::read_to_string(&path).ok()?;
            Some((display, text))
        })
        .collect()
}

/// Build the row set from `(display-path, contents)` pairs. Pure and disk-free, so the whole
/// extraction is unit-testable without a real corpus.
///
/// A slug earns a ROW iff it is DEFINED or QUALIFIED-cited (`docID:slug`). Its `cited:` line then
/// lists every document that qualified-cites OR bare-mentions it, EXCLUDING its own defining
/// documents, one docID per document, count = total occurrences of either form. Bare mentions only
/// augment an existing row — a token that is never defined or qualified-cited mints no row.
fn build_rows(files: &[(String, String)]) -> BTreeMap<String, SlugRow> {
    let mut rows: BTreeMap<String, SlugRow> = BTreeMap::new();
    // Per slug: occurrences (qualified + bare) keyed by document; whether it was ever qualified-cited
    // (a row-minter); and which documents define it (excluded from its `cited:` set).
    let mut occ: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
    let mut qualified: BTreeSet<String> = BTreeSet::new();
    let mut defined_by: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for (display, text) in files {
        let self_id = doc_id_of(display);
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            for (slug, is_qualified) in slug_occurrences(line) {
                let count = occ
                    .entry(slug.clone())
                    .or_default()
                    .entry(self_id.clone())
                    .or_insert(0);
                *count = count.saturating_add(1);
                if is_qualified {
                    qualified.insert(slug);
                }
            }
            if let Some((slug, text_after)) = definition(line) {
                let olds = rename_edges(line);
                let marker = superseded_near(&lines, n);
                let site = DefSite {
                    path: display.clone(),
                    line: u32::try_from(n.saturating_add(1)).unwrap_or(u32::MAX),
                    text: text_after,
                    para: following_paragraph(&lines, n),
                };
                defined_by
                    .entry(slug.clone())
                    .or_default()
                    .insert(self_id.clone());
                {
                    let row = rows.entry(slug.clone()).or_default();
                    row.defs.push(site);
                    if let Some(m) = marker {
                        row.superseded.get_or_insert(m);
                    }
                    for old in &olds {
                        row.akas.insert(old.clone());
                    }
                }
                for old in olds {
                    rows.entry(old)
                        .or_default()
                        .rename_to
                        .get_or_insert(slug.clone());
                }
            }
        }
    }

    // A qualified citation mints a row just as a definition does; ensure those rows exist.
    for slug in &qualified {
        rows.entry(slug.clone()).or_default();
    }
    // Fold both relations into `cited:` — but only for row slugs (defined or qualified-cited), so a
    // bare-only token, or a rename-only `aka`, never gains a row.
    for (slug, row) in &mut rows {
        if !(defined_by.contains_key(slug) || qualified.contains(slug)) {
            continue;
        }
        let Some(doc_counts) = occ.get(slug) else {
            continue;
        };
        let definers = defined_by.get(slug);
        for (doc, count) in doc_counts {
            if definers.is_some_and(|d| d.contains(doc)) {
                continue;
            }
            row.citing.insert(doc.clone());
            row.cite_count = row.cite_count.saturating_add(*count);
        }
    }

    for row in rows.values_mut() {
        row.defs
            .sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
    }
    rows
}

// ── render ───────────────────────────────────────────────────────────────────────────────────

fn header() -> String {
    "# SLUGS — generated index of every slug the corpus defines or cites\n\
     \n\
     INDEX, not prose: do NOT read this top to bottom — opening it shows only its first rows.\n\
     GREP it, for a slug or a word inside one, with a few lines of context to catch the whole\n\
     row: `grep -C5 touches-becomes SLUGS.md`. Each row is one slug; the `##` heading carries the\n\
     defining `docID:slug` (when the slug is defined in one document) so the whole heading pastes\n\
     as a citation. Machine-generated; do not edit.\n\
     \n\
     Generated by `cargo run -q -p internal-tooling --manifest-path spike/Cargo.toml -- slugs`\n\
     (the pre-commit hook regenerates it; `mise run slugs` regenerates it by hand). A row exists\n\
     while the corpus defines or cites its slug; to retire one, mark it superseded where it is\n\
     defined. Advisory `near:` lines (pointers to related discussion that does not cite the slug)\n\
     are NOT generated yet — the seam exists but no embedding backend is wired. When one lands, a\n\
     row's `near:` line refreshes only when that row's other lines change.\n"
        .to_owned()
}

/// The MECHANICAL lines of one row (everything but `near:`), in order. `--check` compares these.
fn mechanical_lines(slug: &str, row: &SlugRow) -> Vec<String> {
    let mut out = Vec::new();
    let is_rename_only = row.defs.is_empty() && row.rename_to.is_some();
    if is_rename_only {
        if let Some(to) = &row.rename_to {
            out.push(format!("- renamed: {to}"));
        }
    } else {
        if row.defs.is_empty() {
            out.push("- defined: —".to_owned());
        } else {
            for d in &row.defs {
                if d.text.is_empty() {
                    out.push(format!("- defined: {}:{}", d.path, d.line));
                } else {
                    out.push(format!("- defined: {}:{} — {}", d.path, d.line, d.text));
                }
            }
        }
        if !row.akas.is_empty() {
            let akas: Vec<&str> = row.akas.iter().map(String::as_str).collect();
            out.push(format!("- aka: {}", akas.join(" ")));
        }
        if let Some(s) = &row.superseded {
            out.push(format!("- superseded: {s}"));
        }
    }
    if !row.citing.is_empty() {
        let ids: Vec<&str> = row.citing.iter().map(String::as_str).collect();
        out.push(format!("- cited: {} ({})", ids.join(" "), row.cite_count));
    }
    let _ = slug;
    out
}

/// Whether a slug earns a row at all: it must be defined or cited. A bare rename target that is
/// never cited lives only as an `aka:` on the new slug's row.
fn has_row(row: &SlugRow) -> bool {
    !row.defs.is_empty() || !row.citing.is_empty()
}

/// The `## <docref>:<slug>` heading prefix for a row: the DEFINING document's ID (the same
/// `doc_id_of` derivation `cited:` uses, so a `docID:slug` grep lands on the heading too) when the
/// slug is defined in EXACTLY ONE document — several sites in one document still count as one.
/// `None` (⇒ a bare `## <slug>` heading, always acceptable) when the slug has no definition or is
/// defined across more than one document; no reference is ever invented.
fn heading_docref(row: &SlugRow) -> Option<String> {
    let mut docs = row.defs.iter().map(|d| doc_id_of(&d.path));
    let first = docs.next()?;
    docs.all(|d| d == first).then_some(first)
}

/// Render the whole file. `near` supplies the (already staleness-resolved) advisory line per slug.
fn render(rows: &BTreeMap<String, SlugRow>, near: &BTreeMap<String, String>) -> String {
    let mut out = header();
    for (slug, row) in rows {
        if !has_row(row) {
            continue;
        }
        out.push_str("\n## ");
        if let Some(docref) = heading_docref(row) {
            out.push_str(&docref);
            out.push(':');
        }
        out.push_str(slug);
        out.push('\n');
        for line in mechanical_lines(slug, row) {
            out.push_str(&line);
            out.push('\n');
        }
        if let Some(n) = near.get(slug) {
            out.push_str("- near: ");
            out.push_str(n);
            out.push('\n');
        }
    }
    out
}

// ── the committed file: parse it back for carry-forward and `--check` ───────────────────────────

/// The previous `SLUGS.md`, parsed into per-slug (mechanical-lines, near-line). Absent file ⇒ empty.
struct Previous {
    mechanical: BTreeMap<String, Vec<String>>,
    near: BTreeMap<String, String>,
}

fn parse_previous(text: &str) -> Previous {
    let mut mechanical: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut near: BTreeMap<String, String> = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            // The heading is `<docref>:<slug>` (or a bare `<slug>`); key by the bare slug — the part
            // after the last `:` — so it round-trips against `build_rows` (docrefs carry no `:`).
            let heading = rest.trim();
            current = Some(heading.rsplit(':').next().unwrap_or(heading).to_owned());
        } else if let Some(slug) = &current {
            if let Some(n) = line.strip_prefix("- near: ") {
                near.insert(slug.clone(), n.to_owned());
            } else if line.starts_with("- ") {
                mechanical
                    .entry(slug.clone())
                    .or_default()
                    .push(line.to_owned());
            }
        }
    }
    Previous { mechanical, near }
}

// ── entry point ────────────────────────────────────────────────────────────────────────────────

/// `slugs [--check] [--near-all] [--no-near] [--stats]`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    let check = args.iter().any(|a| a == "--check");
    let near_all = args.iter().any(|a| a == "--near-all");
    let no_near = args.iter().any(|a| a == "--no-near");
    let stats = args.iter().any(|a| a == "--stats");

    let root = internal_tooling::repo_root();
    let files = corpus_files(root);
    let rows = build_rows(&files);

    if stats {
        print_stats(&files, &rows);
        return ExitCode::SUCCESS;
    }

    let out_path = root.join(OUTPUT);
    let previous_text = std::fs::read_to_string(&out_path).unwrap_or_default();
    let previous = parse_previous(&previous_text);

    if check {
        return check_mechanical(&rows, &previous);
    }

    let ranker = if no_near {
        None
    } else {
        slug_near::build_ranker(&files)
    };
    let near = resolve_near(&rows, &previous, ranker.as_deref(), near_all, no_near);

    let rendered = render(&rows, &near);
    if let Err(e) = std::fs::write(&out_path, rendered.as_bytes()) {
        eprintln!("slugs: could not write {}: {e}", out_path.display());
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

/// Fail on the first slug whose mechanical lines differ from the committed file; near lines and the
/// header are ignored. This is what hk runs under `HK_FIX=0`.
fn check_mechanical(rows: &BTreeMap<String, SlugRow>, previous: &Previous) -> ExitCode {
    let fresh: BTreeMap<String, Vec<String>> = rows
        .iter()
        .filter(|(_, row)| has_row(row))
        .map(|(slug, row)| (slug.clone(), mechanical_lines(slug, row)))
        .collect();
    if fresh == previous.mechanical {
        return ExitCode::SUCCESS;
    }
    let first = fresh
        .iter()
        .find(|(slug, lines)| previous.mechanical.get(*slug) != Some(*lines))
        .map(|(slug, _)| slug.clone())
        .or_else(|| {
            previous
                .mechanical
                .keys()
                .find(|slug| !fresh.contains_key(*slug))
                .cloned()
        });
    match first {
        Some(slug) => {
            println!("slugs: SLUGS.md is stale (first at `{slug}`) — run `mise run slugs`");
        }
        None => println!("slugs: SLUGS.md is stale — run `mise run slugs`"),
    }
    ExitCode::from(1)
}

/// The `near:` line per slug, applying the staleness rule: recompute for a row that is new or whose
/// mechanical lines changed; carry the old line forward byte-for-byte otherwise. An absent backend
/// (or `--no-near`) carries EVERYTHING forward — the run behaves as `--no-near` and computes none,
/// so the near line is allowed to be stale and the hook never needs the model.
fn resolve_near(
    rows: &BTreeMap<String, SlugRow>,
    previous: &Previous,
    ranker: Option<&dyn NearRanker>,
    near_all: bool,
    no_near: bool,
) -> BTreeMap<String, String> {
    let carry_only = no_near || ranker.is_none();
    let mut out = BTreeMap::new();
    for (slug, row) in rows {
        if !has_row(row) {
            continue;
        }
        let mech = mechanical_lines(slug, row);
        let unchanged = previous.mechanical.get(slug) == Some(&mech);
        let recompute = !carry_only && (near_all || !unchanged);
        if recompute {
            if let Some(r) = ranker {
                let excluded = excluded_docs(row);
                if let Some(line) = r.near(&row_query(slug, row), &excluded) {
                    out.insert(slug.clone(), line);
                }
            }
        } else if let Some(old) = previous.near.get(slug) {
            out.insert(slug.clone(), old.clone());
        }
    }
    out
}

/// The query text a near computation scores against: the slug's words, its definition text(s), and
/// the paragraph following the first definition site.
fn row_query(slug: &str, row: &SlugRow) -> String {
    let mut q = slug.replace('-', " ");
    for d in &row.defs {
        q.push(' ');
        q.push_str(&d.text);
    }
    if let Some(first) = row.defs.first() {
        q.push(' ');
        q.push_str(&first.para);
    }
    q
}

/// Documents that must NOT surface as `near:` hits for this slug: the ones defining it and the ones
/// citing it. A near hit is by construction a document that neither defines nor cites the slug.
fn excluded_docs(row: &SlugRow) -> BTreeSet<String> {
    row.defs
        .iter()
        .map(|d| doc_id_of(&d.path))
        .chain(row.citing.iter().cloned())
        .collect()
}

// ── the measure-and-tune diagnostic (never wired to a hook or gate) ─────────────────────────────

fn print_stats(files: &[(String, String)], rows: &BTreeMap<String, SlugRow>) {
    let defined: Vec<&String> = rows
        .iter()
        .filter(|(_, r)| !r.defs.is_empty())
        .map(|(s, _)| s)
        .collect();
    let def_sites: usize = rows.values().map(|r| r.defs.len()).sum();
    let citations: u32 = rows.values().map(|r| r.cite_count).sum();
    let cited_undef: Vec<&String> = rows
        .iter()
        .filter(|(_, r)| r.defs.is_empty() && !r.citing.is_empty() && r.rename_to.is_none())
        .map(|(s, _)| s)
        .collect();
    let def_uncited: Vec<&String> = rows
        .iter()
        .filter(|(_, r)| !r.defs.is_empty() && r.citing.is_empty())
        .map(|(s, _)| s)
        .collect();
    let def_twice: Vec<&String> = rows
        .iter()
        .filter(|(_, r)| r.defs.len() >= 2)
        .map(|(s, _)| s)
        .collect();
    let renames: usize = rows.values().filter(|r| r.rename_to.is_some()).count();
    let superseded: usize = rows.values().filter(|r| r.superseded.is_some()).count();
    let rows_emitted = rows.values().filter(|r| has_row(r)).count();

    println!("files walked      {}", files.len());
    println!("rows emitted      {rows_emitted}");
    println!("slugs defined     {}", defined.len());
    println!("definition sites  {def_sites}");
    println!("citations         {citations}");
    println!("cited-undefined   {}", cited_undef.len());
    println!("defined-uncited   {}", def_uncited.len());
    println!("defined-twice     {}", def_twice.len());
    println!("rename edges      {renames}");
    println!("superseded        {superseded}");
    sample("cited-but-undefined", &cited_undef, rows);
    sample("defined-but-uncited", &def_uncited, rows);
    sample("defined-twice", &def_twice, rows);
}

fn sample(label: &str, slugs: &[&String], rows: &BTreeMap<String, SlugRow>) {
    println!("\n{label}:");
    for slug in slugs.iter().take(3) {
        if let Some(row) = rows.get(*slug) {
            let where_ = row.defs.first().map_or_else(
                || {
                    let mut ids: Vec<&str> = row.citing.iter().map(String::as_str).collect();
                    ids.truncate(4);
                    format!("cited by {}", ids.join(" "))
                },
                |d| format!("{}:{}", d.path, d.line),
            );
            println!("  {slug}  ({where_})");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DefSite, SlugRow, definition, heading_docref, parse_previous, rename_edges,
        slug_occurrences, superseded_near, take_slug,
    };

    fn def(path: &str) -> DefSite {
        DefSite {
            path: path.to_owned(),
            line: 1,
            text: String::new(),
            para: String::new(),
        }
    }

    fn slug(line: &str) -> Option<String> {
        definition(line).map(|(s, _)| s)
    }

    #[test]
    fn it_takes_three_part_slugs_only() {
        let chars: Vec<char> = "dec-timing-cache rest".chars().collect();
        assert_eq!(
            take_slug(&chars, 0).map(|(s, _)| s),
            Some("dec-timing-cache".to_owned())
        );
        let two: Vec<char> = "well-tested".chars().collect();
        assert_eq!(take_slug(&two, 0), None, "two parts is not a slug");
    }

    #[test]
    fn it_reads_the_three_definition_shapes() {
        assert_eq!(
            slug("### rul-touches-becomes-disturbs  (2026-07-11)"),
            Some("rul-touches-becomes-disturbs".to_owned())
        );
        assert_eq!(
            slug("- **what-dorc-is** — Dorc plans"),
            Some("what-dorc-is".to_owned())
        );
        assert_eq!(
            slug("## `rul-skip-ban-is-llm-facing`"),
            Some("rul-skip-ban-is-llm-facing".to_owned())
        );
        assert_eq!(
            slug("1. **step-zero-worktree-first** does X"),
            Some("step-zero-worktree-first".to_owned())
        );
    }

    #[test]
    fn a_paragraph_bold_is_not_a_definition() {
        // `**disowned-and-lazy** by design` is prose emphasis, not a list-item definition.
        assert_eq!(slug("**disowned-and-lazy** by design"), None);
    }

    #[test]
    fn the_definition_text_is_trimmed_and_kept() {
        let (_, text) = definition("### rul-net-quality-u-curve  (2026-07-11; TYPED)").unwrap();
        assert_eq!(text, "(2026-07-11; TYPED)");
    }

    #[test]
    fn it_flags_qualified_citations_and_counts_bare_mentions() {
        // A `docID:slug` is qualified (row-minting); an otherwise-identical bare token is a mention.
        assert_eq!(
            slug_occurrences("per `271:rul-sin-ordering` and a bare rul-touches-becomes-disturbs"),
            [
                ("rul-sin-ordering".to_owned(), true),
                ("rul-touches-becomes-disturbs".to_owned(), false),
            ]
        );
        assert_eq!(
            slug_occurrences("see AID-NEEDS:law-collapse-mints-narrative"),
            [("law-collapse-mints-narrative".to_owned(), true)]
        );
        // A two-part token and bare line-number pairs are not slugs at all.
        assert!(slug_occurrences("well-tested 15:30 and 100:1").is_empty());
        // A longer token is taken maximally, never as a shorter prefix.
        assert_eq!(
            slug_occurrences("rul-foo-bar-baz"),
            [("rul-foo-bar-baz".to_owned(), false)]
        );
    }

    #[test]
    fn a_rename_edge_names_the_old_slug_with_or_without_backticks() {
        assert_eq!(
            rename_edges("- **compare-consumer-map** (née ternary-compare-consumer-map; x)"),
            ["ternary-compare-consumer-map"]
        );
        assert_eq!(
            rename_edges("- **collapse-mints-narrative** (née `collapse-mints-evidence`)"),
            ["collapse-mints-evidence"]
        );
        assert!(
            rename_edges("(né touches): at-most claims").is_empty(),
            "two-part is not a slug"
        );
    }

    #[test]
    fn a_rename_edge_accepts_the_bare_ne_without_the_accent() {
        // `ne` with no accent is equal to `né`/`née`/`nee`; the bare form must not shadow `nee`.
        assert_eq!(
            rename_edges("- **compare-consumer-map** (ne ternary-compare-consumer-map)"),
            ["ternary-compare-consumer-map"]
        );
        assert_eq!(
            rename_edges("## `collapse-mints-narrative` (nee `collapse-mints-evidence`)"),
            ["collapse-mints-evidence"]
        );
    }

    #[test]
    fn a_superseded_marker_matches_only_the_exact_word() {
        // The retirement marker is the exact word `superseded`; `supersedes` (the active verb, one
        // letter off) marks the LIVE slug and must NOT match. That exactness is also what drops the
        // corpus's former misspelling now that the spelling pass has removed it — the stem branch
        // that used to tolerate it is gone.
        let live = ["## some-live-slug", "- superseded: gone 2026", "body"];
        assert_eq!(
            superseded_near(&live, 0).as_deref(),
            Some("- superseded: gone 2026")
        );
        let active = ["## some-live-slug", "this supersedes the old note", "body"];
        assert_eq!(
            superseded_near(&active, 0),
            None,
            "the active verb is one letter off and must not match"
        );
        let plain = ["## some-live-slug", "an ordinary body line", "body"];
        assert_eq!(superseded_near(&plain, 0), None);
    }

    #[test]
    fn a_heading_docref_is_the_lone_defining_document() {
        // Singly-defined ⇒ the defining doc's ID (the same `doc_id_of` derivation `cited:` uses):
        // a numeric corpus doc, a root stem, and a crate CLAUDE.md all round-trip.
        let numeric = SlugRow {
            defs: vec![def("Research/plans/271-block-settle-rulings-ledger.md")],
            ..Default::default()
        };
        assert_eq!(heading_docref(&numeric).as_deref(), Some("271"));

        let root = SlugRow {
            defs: vec![def("KNOBS.md")],
            ..Default::default()
        };
        assert_eq!(heading_docref(&root).as_deref(), Some("KNOBS"));

        let crate_claude = SlugRow {
            defs: vec![def("spike/crates/why/CLAUDE.md")],
            ..Default::default()
        };
        assert_eq!(
            heading_docref(&crate_claude).as_deref(),
            Some("spike/crates/why/CLAUDE")
        );

        // Several SITES in ONE document ⇒ still one docref.
        let twice = SlugRow {
            defs: vec![
                def("Research/notes/28T-correctness-tooling-synthesis.md"),
                def("Research/notes/28T-correctness-tooling-synthesis.md"),
            ],
            ..Default::default()
        };
        assert_eq!(heading_docref(&twice).as_deref(), Some("28T"));

        // Defined across TWO documents, and defined nowhere at all ⇒ a bare heading.
        let split = SlugRow {
            defs: vec![
                def("Research/plans/271-x.md"),
                def("Research/notes/30U-y.md"),
            ],
            ..Default::default()
        };
        assert_eq!(heading_docref(&split), None);
        assert_eq!(heading_docref(&SlugRow::default()), None);
    }

    #[test]
    fn parse_previous_keys_rows_by_the_bare_slug() {
        // A `## <docref>:<slug>` heading must round-trip to the BARE slug key so `--check` compares
        // against `build_rows`, which keys by the bare slug; a bare heading keys as itself.
        let prev = parse_previous(
            "## 271:foo-bar-baz\n- defined: x:1\n\n## plain-three-word-slug\n- cited: 30U (1)\n",
        );
        assert!(prev.mechanical.contains_key("foo-bar-baz"));
        assert!(prev.mechanical.contains_key("plain-three-word-slug"));
    }
}
