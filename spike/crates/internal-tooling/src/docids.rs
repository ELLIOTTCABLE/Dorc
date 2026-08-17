//! Dangling-docID lint over the prose corpus. READ-ONLY — it opens no file for writing, ever.
//!
//! The corpus cross-references documents by ID (`28Q`, `27Xf`, `notes/307`, `271:rul-sin-ordering`),
//! and a renumber or a retirement is a hand-grep affair, so a missed site rots in silence: the
//! reference still READS fine, and nothing but a human following it discovers the target is gone.
//!
//! PRECISION OVER RECALL, deliberately. Only three high-confidence spellings are matched, because
//! the alternative is an ocean of false positives — bare three-digit runs are line numbers, counts,
//! byte budgets, test tallies and version fragments far more often than they are docIDs, and a lint
//! that cries wolf is one people stop reading. A citation this misses is a citation the corpus
//! survives; a citation this wrongly flags costs a human's attention every run. Every filter below
//! was derived from a hand-triaged full-corpus run, and each one names the shape that motivated it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Cited IDs with no document, and the in-corpus evidence that each absence is legitimate.
///
/// Three families sit here, all attested at the citing line: slugs a note was DRAFTED under before
/// being renamed, documents that were minted and later REMOVED, and planned successors that were
/// announced and never written. Their citations are historical prose inside `Research/notes/`,
/// which is append-only by law — the reference is correct about what happened, so the fix is to
/// record why, not to rewrite history.
///
/// A stale entry is worse than no entry: it silently excuses a real dangle. The lint fails on an
/// entry that has stopped being cited, so this list cannot quietly outlive its reasons.
/// One entry per line, each under its own reason — `rustfmt` otherwise packs several onto a line
/// and leaves the comments attached to the wrong IDs, which is a worse lie than no comment.
#[rustfmt::skip]
const RETIRED: &[&str] = &[
    // A quarantined note, deliberately cited by a slug that does not resolve; plans/16P's own
    // §Citations note says so in as many words.
    "16X",
    // The parallel K1 agent's synthesis, quarantined out of the tree (notes/17A).
    "177",
    // Announced in plans/175 as "a later step", and never minted.
    "17Y",
    "17Z",
    // Named only in the round-21 plan-of-attack's roll-call of notes it intended to write.
    "21A",
    "21C",
    // Provisional drafting slugs. Each renamed note records its own correction in an HTML comment
    // at the top of the file: 21I became 221, 21J became 222.
    "21I",
    "21J",
    // Drafted and then REMOVED — notes/224 says exactly that at the citing line.
    "22G",
    // Listed beside 23J in the round-23 closeout roll-call; 23J exists, this one never landed.
    "23I",
    // The round-23 resumption ledger. Ephemeral by design and deleted when the round closed, which
    // is why fifteen documents of that era still point at it.
    "23Z",
    // Not a citation at all: the endpoint of the prose range "28A…28V" in notes/307.
    "28V",
    // A drafting slug, like 21I/21J above: the closure/custody work order was authored as 30C and
    // renumbered to 30G at the fold. Its own §8 deviation records the correction at the citing line.
    // Two floor30 cells still carry the old slug in a book comment, deliberately: their book bytes
    // are digest-pinned by their transcripts, so sweeping a comment would re-open a floor
    // measurement (`emitted-is-measure-once-ground-truth`) to no end. This lint reads markdown only.
    "30C",
];

/// Where the corpus keeps documents whose filename encodes their ID. Walked RECURSIVELY, and
/// directory names count: `plans/deferred/078-…` and `notes/28G-why-strawmen-v2/` are both real
/// targets, and missing the subdirectories reported fourteen live documents as dangling.
const DOC_DIRS: [&str; 2] = ["Research/notes", "Research/plans"];

/// Never descended into for content, wherever it appears; its filenames alone answer existence.
/// Quarantined material is off-limits to content reads, and this lint has no business being the
/// exception.
const QUARANTINE_DIR: &str = "quarantine-DO-NOT-READ";

/// Which spelling produced a citation. Only the bare form needs the corpus-shaped guard below —
/// a `notes/` path and a `:slug` tail each disambiguate themselves.
#[derive(PartialEq, Eq)]
enum Shape {
    Qualified,
    Bare,
}

fn ch(chars: &[char], i: usize) -> Option<char> {
    chars.get(i).copied()
}

fn starts_with(chars: &[char], i: usize, want: &str) -> bool {
    want.chars()
        .enumerate()
        .all(|(off, c)| ch(chars, i.saturating_add(off)) == Some(c))
}

/// A docID token: 1–3 digits then up to 2 ASCII letters (`307`, `306b`, `27Xf`).
fn take_id(chars: &[char], i: usize) -> Option<(String, usize)> {
    let mut end = i;
    let mut digits = 0_u8;
    while digits < 3 && ch(chars, end).is_some_and(|c| c.is_ascii_digit()) {
        end = end.saturating_add(1);
        digits = digits.saturating_add(1);
    }
    if digits == 0 {
        return None;
    }
    let mut letters = 0_u8;
    while letters < 2 && ch(chars, end).is_some_and(|c| c.is_ascii_alphabetic()) {
        end = end.saturating_add(1);
        letters = letters.saturating_add(1);
    }
    Some((chars.get(i..end)?.iter().collect(), end))
}

/// `notes/307`, `plans/309`, `Research/notes/306b` — an explicit path is unambiguous whatever the
/// ID's shape, which is what lets the digits-only IDs be checked at all.
fn take_path(chars: &[char], i: usize) -> Option<(String, usize)> {
    let dir = ["notes/", "plans/"]
        .into_iter()
        .find(|dir| starts_with(chars, i, dir))?;
    let (id, end) = take_id(chars, i.saturating_add(dir.len()))?;
    (!ch(chars, end).is_some_and(char::is_alphanumeric)).then_some((id, end))
}

/// `271:rul-sin-ordering` — the slug is the disambiguator, so a digits-only ID is safe here too.
///
/// The lowercase start and the mandatory hyphen keep `15:30` and `100:1` out; the three-character
/// floor keeps SECTION references out, because the corpus also writes `§0:v-no-reprobe-needed` and
/// every real docID is at least three characters.
fn take_colon_slug(chars: &[char], i: usize) -> Option<(String, usize)> {
    let (id, after) = take_id(chars, i)?;
    if id.len() < 3 || ch(chars, after) != Some(':') {
        return None;
    }
    let mut end = after.saturating_add(1);
    if !ch(chars, end).is_some_and(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let mut hyphens = 0_u32;
    while let Some(c) = ch(chars, end) {
        if c == '-' {
            hyphens = hyphens.saturating_add(1);
        } else if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
            break;
        }
        end = end.saturating_add(1);
    }
    (hyphens > 0).then_some((id, end))
}

/// A bare letter-bearing ID: two digits, an uppercase letter, optionally one lowercase (`28Q`,
/// `27Xf`, `24Kc`). The letter is what makes it safe to match unadorned.
///
/// Three guards, each answering a shape the corpus really contains:
/// * no backtracking — `15GiB` consumes the `i` and fails on `B` rather than falling back to `15G`;
/// * no `:` predecessor — an ISO timestamp ends `09:14:02Z`, and `02Z` is otherwise a perfect match;
/// * no `.` predecessor — `1.35M` and `1.50M` are magnitudes, not documents.
///
/// The predecessor rules live here rather than at the call site because a relative path
/// (`./notes/307`) must keep matching.
fn take_lettered(chars: &[char], i: usize) -> Option<(String, usize)> {
    if i > 0 && matches!(ch(chars, i.saturating_sub(1)), Some(':' | '.')) {
        return None;
    }
    if !ch(chars, i).is_some_and(|c| c.is_ascii_digit())
        || !ch(chars, i.saturating_add(1)).is_some_and(|c| c.is_ascii_digit())
        || !ch(chars, i.saturating_add(2)).is_some_and(|c| c.is_ascii_uppercase())
    {
        return None;
    }
    let mut end = i.saturating_add(3);
    if ch(chars, end).is_some_and(|c| c.is_ascii_lowercase()) {
        end = end.saturating_add(1);
    }
    if ch(chars, end).is_some_and(char::is_alphanumeric) {
        return None;
    }
    Some((chars.get(i..end)?.iter().collect(), end))
}

/// Every docID citation on one line. A match consumes its whole span, so nothing inside a matched
/// slug or path is re-read as a second citation.
fn references(line: &str) -> Vec<(String, Shape)> {
    let chars: Vec<char> = line.chars().collect();
    let mut found = Vec::new();
    let mut i = 0_usize;
    while i < chars.len() {
        // A citation never starts mid-word: this is what keeps the scanner out of the interior of
        // `2026-08-16`, `0.14.1` and `AES256`, where every shape below would otherwise get a turn.
        let fresh = i == 0 || !ch(&chars, i.saturating_sub(1)).is_some_and(char::is_alphanumeric);
        let hit = if fresh {
            take_path(&chars, i)
                .or_else(|| take_colon_slug(&chars, i))
                .map(|(id, end)| (id, end, Shape::Qualified))
                .or_else(|| take_lettered(&chars, i).map(|(id, end)| (id, end, Shape::Bare)))
        } else {
            None
        };
        match hit {
            Some((id, end, shape)) => {
                found.push((id, shape));
                i = end.max(i.saturating_add(1));
            }
            None => i = i.saturating_add(1),
        }
    }
    found
}

/// The ID a corpus filename encodes: everything before the first hyphen, if it starts with a digit.
fn id_of(name: &str) -> Option<&str> {
    let id = name.split('-').next()?;
    id.starts_with(|c: char| c.is_ascii_digit()).then_some(id)
}

/// Does the corpus have a series at this ID's leading digits?
///
/// The one corpus-shaped guard, and it applies to the bare form ALONE. `95K`, `85K`, `50M`, `35M`
/// and `47B` all pass every syntactic test and are token counts, magnitudes and manifest-row codes;
/// what disqualifies them is that no document numbered `95…`/`85…`/`47…` has ever existed. A
/// citation into a series the corpus never had is not a citation. This can only suppress a report,
/// never invent one, which is the safe direction for a precision-first lint.
fn series_exists(id: &str, known: &BTreeSet<String>) -> bool {
    id.get(..2)
        .is_some_and(|series| known.iter().any(|doc| doc.starts_with(series)))
}

fn dir_entries(dir: &Path) -> Vec<(String, bool)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            Some((name, entry.file_type().is_ok_and(|kind| kind.is_dir())))
        })
        .collect()
}

/// Every document ID under `dir`, files and directories alike, all the way down.
fn doc_ids(dir: &Path, out: &mut BTreeSet<String>) {
    for (name, is_dir) in dir_entries(dir) {
        if let Some(id) = id_of(&name) {
            out.insert(id.to_owned());
        }
        if is_dir {
            doc_ids(&dir.join(name), out);
        }
    }
}

/// Quarantine filenames, gathered by NAME ONLY — no entry is ever opened.
fn quarantine_ids(dir: &Path, out: &mut Vec<String>) {
    for (name, is_dir) in dir_entries(dir) {
        out.push(name.clone());
        if is_dir {
            quarantine_ids(&dir.join(name), out);
        }
    }
}

/// Markdown under `dir`. A quarantine is harvested for names and never descended into for content,
/// wherever in the tree it turns up.
fn markdown_under(dir: &Path, files: &mut Vec<PathBuf>, quarantined: &mut Vec<String>) {
    for (name, is_dir) in dir_entries(dir) {
        let path = dir.join(&name);
        if is_dir {
            if name == QUARANTINE_DIR {
                quarantine_ids(&path, quarantined);
            } else {
                markdown_under(&path, files, quarantined);
            }
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
}

/// The three scanned surfaces: the corpus, the steering files, and the root docs.
fn scanned(root: &Path, quarantined: &mut Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    markdown_under(&root.join("Research"), &mut files, quarantined);
    let mut spike = Vec::new();
    markdown_under(&root.join("spike"), &mut spike, quarantined);
    files.extend(
        spike
            .into_iter()
            .filter(|path| path.file_name().is_some_and(|name| name == "CLAUDE.md")),
    );
    files.extend(
        dir_entries(root)
            .iter()
            .map(|(name, _)| root.join(name))
            .filter(|path| path.extension().is_some_and(|ext| ext == "md")),
    );
    files.sort();
    files
}

pub(crate) fn run() -> ExitCode {
    let root = internal_tooling::repo_root();
    let mut known = BTreeSet::new();
    for dir in DOC_DIRS {
        doc_ids(&root.join(dir), &mut known);
    }
    if known.is_empty() {
        eprintln!("docids: found no corpus documents under {}", root.display());
        return ExitCode::from(2);
    }
    let mut quarantined = Vec::new();
    let files = scanned(root, &mut quarantined);

    let mut dangling: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut cited: BTreeSet<String> = BTreeSet::new();
    let mut checked = 0_u32;
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let shown = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        for (number, line) in text.lines().enumerate() {
            for (id, shape) in references(line) {
                // `notes/06x-*` is the corpus's own glob for a whole series, not a document.
                if id.ends_with('x') || (shape == Shape::Bare && !series_exists(&id, &known)) {
                    continue;
                }
                checked = checked.saturating_add(1);
                cited.insert(id.clone());
                if known.contains(&id)
                    || quarantined.iter().any(|name| name.starts_with(&id))
                    || RETIRED.contains(&id.as_str())
                {
                    continue;
                }
                let at = format!("{shown}:{}", number.saturating_add(1));
                dangling.entry(id).or_default().push(at);
            }
        }
    }

    let unused: Vec<&&str> = RETIRED.iter().filter(|id| !cited.contains(**id)).collect();
    if dangling.is_empty() && unused.is_empty() {
        return ExitCode::SUCCESS;
    }
    for (id, sites) in &dangling {
        println!("{id} — no document, {} citation(s)", sites.len());
        for site in sites {
            println!("    {site}");
        }
    }
    for id in &unused {
        println!("{id} — allowlisted in RETIRED but no longer cited; drop the entry");
    }
    println!(
        "docids: {} unresolved of {checked} citations — fix the reference, or add the ID to \
         RETIRED in docids.rs with its reason",
        dangling.len()
    );
    ExitCode::from(1)
}

#[cfg(test)]
mod tests {
    use super::{Shape, references, series_exists};
    use std::collections::BTreeSet;

    fn ids(line: &str) -> Vec<String> {
        references(line).into_iter().map(|(id, _)| id).collect()
    }

    /// The three sanctioned spellings, each in the punctuation the corpus actually wraps them in.
    #[test]
    fn it_reads_the_three_citation_shapes() {
        assert_eq!(ids("see 28Q and 27Xf."), ["28Q", "27Xf"]);
        assert_eq!(ids("(`Research/notes/306b` §2)"), ["306b"]);
        assert_eq!(ids("per plans/309 and notes/27Xf"), ["309", "27Xf"]);
        assert_eq!(ids("`271:rul-sin-ordering` holds"), ["271"]);
        assert_eq!(ids("24Kc:rul-a-b-c"), ["24Kc"]);
    }

    /// The false-positive ocean this lint exists to stay out of. Every line here is quoted from the
    /// corpus, and any one of them matching would make the lint unreadable.
    #[test]
    fn it_stays_out_of_the_false_positive_ocean() {
        for quiet in [
            "measured 2026-08-16 against hk 1.53.0",
            "a 15GiB cap and 62 MiB of rlib",
            "302 lines, 306 files, exit 141",
            "the r28 lane and round29/opaque-accrual",
            "AES256 at 15:30, ratio 100:1",
            "posh 0.14.1 ∩ dash 0.5.12",
            "`279f` §3 and 28-reviewA",
            "dorc-run seed-tiles.sh 2026-07-28T09:14:02Z headless",
            "(`§0:v-no-reprobe-needed`): invalid-Query checks ship",
            "1.35M GitHub bash scripts, R1' 179K, ~1.50M plus",
        ] {
            assert_eq!(ids(quiet), Vec::<String>::new(), "matched in {quiet:?}");
        }
    }

    /// A matched citation consumes its span, so a slug's own digits cannot mint a second hit.
    #[test]
    fn a_matched_span_is_not_rescanned() {
        assert_eq!(ids("281:rul-verbs-30-dotless"), ["281"]);
    }

    /// The bare form's corpus guard: same syntax, opposite verdicts, decided by whether the corpus
    /// ever numbered that series. `95K` is a token count; `28V` is a citation into a live series.
    #[test]
    fn the_series_guard_separates_counts_from_citations() {
        let known: BTreeSet<String> = ["280", "28Q", "309"].map(str::to_owned).into();
        assert!(!series_exists("95K", &known));
        assert!(series_exists("28V", &known));
        assert_eq!(
            references("~95K tokens")
                .first()
                .map(|(_, s)| s == &Shape::Bare),
            Some(true)
        );
    }
}
