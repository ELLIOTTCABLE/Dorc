//! The generated-lock fixpoint and the class-specific lint batteries.
//!
//! The RENDER-level fixpoint used to live here too, over a hand-listed four-case allow-list.
//! It does not any more: `crates/cli/tests/looms.rs` fixpoint-checks and hygiene-checks EVERY
//! committed loom in every collection, so exactly one corpus-level render-fixpoint authority
//! exists (`289:rider-fixpoint-gate-rationalize`). Two gates over one property meant the
//! narrower one silently rotted — 48 of 51 cases already held while the list still named
//! four. What stays here is what the runner cannot hold: the corpus-GLOBAL generated-lock
//! byte-identity gate (the second of the two fixpoints `defining-case-catalog` names) and the
//! class batteries that assert production-route identity rather than transcript bytes.
//!
//! Cost of the move, stated: `cargo test -p dorc-loom` alone no longer covers render fixpoint.
//! `cargo test --workspace` does, and that is every builder's standard gate.

use std::path::{Path, PathBuf};

use dorc_loom::{
    DorcConsumer, compile_section_edit, generate_arrangement_lock, generate_catalog_lock,
    load_arrangement_corpus, load_corpus_by_slug, replay_case,
};
use errorloom::{Case, CaseRenderer, RunEnv};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests")
}

fn committed_lock() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/src/catalog_lock.rs")
}

fn committed_arrangement_lock() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/src/arrangement_lock.rs")
}

/// The generated-catalog byte-identity gate (`28A` §4 · `282:rul-catalog-lock-is-generated-whole`):
/// regenerating the whole lock from the committed corpus reproduces the committed `catalog_lock.rs`
/// byte-for-byte. This is the second fixpoint (the render-level case fixpoint is the first). A
/// hand-edit to the generated lock, or drift between a case and its generated row, trips here.
#[test]
fn generated_lock_reproduces_the_committed_bytes() {
    let consumer = DorcConsumer::new();
    let cases = load_corpus_by_slug(&corpus_dir()).expect("load corpus");
    // The vacuity floor this crate's cross-crate corpus reach needs (`paths-are-manifest-
    // relative`): an empty read would otherwise generate a stub lock and diff loudly for the
    // wrong reason. Non-empty, never a count (`count-drifts`).
    assert!(
        !cases.is_empty(),
        "no `.loom` cases under {} — the corpus is not where this crate reaches",
        corpus_dir().display()
    );
    let generated = generate_catalog_lock(&consumer, &cases).expect("generate lock");
    let committed = std::fs::read_to_string(committed_lock()).expect("read committed lock");
    if let Some(report) = lock_difference(&generated, &committed, "CatalogEntry") {
        panic!("the committed catalog_lock.rs is not a fixpoint of the generator\n{report}");
    }
}

/// The arrangement registry's half of the same byte-identity gate
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`): regenerating the whole
/// `arrangement_lock.rs` from the committed registry + arrangement cases reproduces the committed
/// bytes. A hand-edit to the generated lock, or drift between a case's frontmatter and its
/// generated row, trips here — exactly as it does for the catalog.
#[test]
fn generated_arrangement_lock_reproduces_the_committed_bytes() {
    let consumer = DorcConsumer::new();
    let cases = load_arrangement_corpus(&corpus_dir()).expect("load arrangement corpus");
    assert!(
        !cases.is_empty(),
        "no arrangement cases under {} — the corpus is not where this crate reaches",
        corpus_dir().display()
    );
    let generated = generate_arrangement_lock(&consumer, &cases).expect("generate lock");
    let committed =
        std::fs::read_to_string(committed_arrangement_lock()).expect("read committed lock");
    if let Some(report) = lock_difference(&generated, &committed, "ArrangementEntry") {
        panic!("the committed arrangement_lock.rs is not a fixpoint of the generator\n{report}");
    }
}

/// Both lock gates' failure report. A whole-file `assert_eq!` over a generated lock dumps ~120 KB
/// of identical rows around the one that moved, and a reader has to diff it by eye — a field-order
/// slip once hid in exactly that dump. This locates the FIRST differing row, names its slug and
/// field, and shows only the differing neighbourhood of that field's value.
///
/// Returns `None` when the two are byte-identical.
fn lock_difference(generated: &str, committed: &str, entry: &str) -> Option<String> {
    if generated == committed {
        return None;
    }
    let marker = format!("    {entry} {{\n");
    // Each chunk after a marker runs to the NEXT marker, so it trails the row's own closing brace
    // plus whatever separates the rows; cut there, or a row-count change reads as a field change.
    let rows = |text: &str| -> Vec<String> {
        text.split(&marker)
            .skip(1)
            .map(|chunk| {
                chunk
                    .find("\n    },")
                    .and_then(|end| chunk.get(..end))
                    .unwrap_or(chunk)
                    .to_owned()
            })
            .collect()
    };
    let (left, right) = (rows(generated), rows(committed));
    for index in 0..left.len().max(right.len()) {
        match (left.get(index), right.get(index)) {
            (Some(l), Some(r)) if l == r => {}
            (Some(l), Some(r)) => return Some(row_difference(index, l, r)),
            (Some(l), None) => {
                return Some(format!(
                    "row {index} `{}` is GENERATED but absent from the committed lock",
                    row_slug(l)
                ));
            }
            (None, Some(r)) => {
                return Some(format!(
                    "row {index} `{}` is COMMITTED but the generator no longer produces it",
                    row_slug(r)
                ));
            }
            (None, None) => break,
        }
    }
    // Equal row bodies but unequal files: the difference is in the header or the trailer.
    Some(format!(
        "every row matches — the difference is in the generated header/trailer ({} vs {} bytes)",
        generated.len(),
        committed.len()
    ))
}

/// The first differing FIELD of one row, with the value neighbourhood around the first differing
/// character. Field values reach thousands of characters (a whole help page is one row's word), so
/// a bare pair of values would restore the haystack this report exists to remove.
fn row_difference(index: usize, generated: &str, committed: &str) -> String {
    const CONTEXT: usize = 60;
    let (left, right): (Vec<&str>, Vec<&str>) =
        (generated.lines().collect(), committed.lines().collect());
    for line in 0..left.len().max(right.len()) {
        let (l, r) = (left.get(line).copied(), right.get(line).copied());
        if l == r {
            continue;
        }
        let (l, r) = (l.unwrap_or("<row ends>"), r.unwrap_or("<row ends>"));
        let field = l.split_once(':').map_or("<field>", |(name, _)| name.trim());
        let at = l
            .chars()
            .zip(r.chars())
            .position(|(a, b)| a != b)
            .unwrap_or_else(|| l.chars().count().min(r.chars().count()));
        let from = at.saturating_sub(CONTEXT);
        return format!(
            "row {index} `{}` field `{field}` diverges at character {at}\n  generated: {:?}\n  committed: {:?}",
            row_slug(generated),
            window(l, from, at.saturating_add(CONTEXT)),
            window(r, from, at.saturating_add(CONTEXT)),
        );
    }
    format!(
        "row {index} `{}` differs only in line count",
        row_slug(generated)
    )
}

fn row_slug(row: &str) -> &str {
    row.lines()
        .find_map(|line| line.trim().strip_prefix("slug: \""))
        .and_then(|rest| rest.split_once('"'))
        .map_or("<unnamed>", |(slug, _)| slug)
}

fn window(text: &str, from: usize, to: usize) -> String {
    let clipped: String = text
        .chars()
        .skip(from)
        .take(to.saturating_sub(from))
        .collect();
    format!(
        "{}{clipped}{}",
        if from > 0 { "…" } else { "" },
        if text.chars().count() > to { "…" } else { "" }
    )
}

/// The report itself, over injected drift — a failure reporter nothing exercises is a failure
/// reporter nobody can trust at 3am. Each case pins the one fact that makes the report worth
/// having: the slug, the field name, and that the value window is bounded.
#[test]
fn the_lock_report_names_the_row_and_the_field() {
    let lock = |why: &str, words: &str| {
        format!(
            "// @generated\n    CatalogEntry {{\n        slug: \"first\",\n        why: \"y\",\n    }},\n    \
             CatalogEntry {{\n        slug: \"second\",\n        why: {why:?},\n        words: {words:?},\n    }},\n];\n"
        )
    };
    assert_eq!(
        lock_difference(&lock("y", "w"), &lock("y", "w"), "CatalogEntry"),
        None
    );

    let report = lock_difference(&lock("changed", "w"), &lock("y", "w"), "CatalogEntry")
        .expect("a differing row reports");
    assert!(report.contains("row 1 `second`"), "{report}");
    assert!(report.contains("field `why`"), "{report}");
    assert!(!report.contains("slug: \"first\""), "{report}");

    let long = "x".repeat(4_000);
    let report = lock_difference(&lock("y", &long), &lock("y", "w"), "CatalogEntry")
        .expect("a differing row reports");
    assert!(report.contains("field `words`"), "{report}");
    assert!(
        report.len() < 400,
        "a 4000-character field value must not restore the haystack: {} bytes",
        report.len()
    );

    let dropped = "// @generated\n    CatalogEntry {\n        slug: \"first\",\n        why: \"y\",\n    },\n];\n";
    let report =
        lock_difference(&lock("y", "w"), dropped, "CatalogEntry").expect("a dropped row reports");
    assert!(report.contains("`second`"), "{report}");
}

/// The static half of the arity net (`aid::arrangement`'s debug assertion is the dynamic half):
/// no committed transcript may show `[unwritten: <slug>]` for an arrangement row that HAS words.
/// That combination is only reachable by degradation — a row whose word count stopped matching its
/// seat — and re-blessing bakes it in quietly, since the placeholder re-renders as a fixpoint.
///
/// FULLY written, occurrence by occurrence: the rendered placeholder carries the slug alone, so a
/// slug with a written occurrence 0 and an unwritten occurrence 1 is INDISTINGUISHABLE from the
/// degradation this looks for — and the honest reading of an unwritten sibling rendering its own
/// placeholder is that it is unwritten. Keying on any-written false-fired the moment a transcript
/// first reached a partially-written family (the four decline classes).
#[test]
fn no_committed_transcript_shows_a_written_arrangement_as_unwritten() {
    let unwritten: std::collections::BTreeSet<&str> = dorc_aid::arrangement::ARRANGEMENTS
        .iter()
        .filter(|entry| entry.words.words().is_none())
        .map(|entry| entry.slug)
        .collect();
    let written: std::collections::BTreeSet<&str> = dorc_aid::arrangement::ARRANGEMENTS
        .iter()
        .filter(|entry| entry.words.words().is_some())
        .map(|entry| entry.slug)
        .filter(|slug| !unwritten.contains(slug))
        .collect();
    let mut degraded = Vec::new();
    let mut scanned = 0usize;
    for entry in std::fs::read_dir(corpus_dir()).expect("read corpus dir") {
        let path = entry.expect("corpus entry").path();
        if path.extension().is_none_or(|extension| extension != "loom") {
            continue;
        }
        scanned = scanned.saturating_add(1);
        let text = std::fs::read_to_string(&path).expect("read case");
        for slug in &written {
            if text.contains(&format!("[unwritten: {slug}]")) {
                degraded.push(format!("{}: {slug}", path.display()));
            }
        }
    }
    assert!(
        scanned > 0,
        "no cases scanned — the gate would pass vacuously"
    );
    assert!(
        degraded.is_empty(),
        "a written arrangement row renders as unwritten — its word count no longer serves its \
         seat: {degraded:?}"
    );
}

/// A WORLD-AS-PAYLOAD case — one whose replay is `dorc plan --book=book.sh` with no materialized
/// `book.sh` — must reach the driver's editable route, exactly as `render_case` already does. When
/// the driver declined these, `compile`/`promote` saw bytes-only results, so their prose was
/// editable nowhere but the generated lock: the corpus contradicted its own loom-is-the-home claim.
/// Discovered rather than listed, so a new case of this shape joins the gate on arrival.
#[test]
fn world_as_payload_cases_reach_the_editable_route() {
    let cases = load_corpus_by_slug(&corpus_dir()).expect("load corpus");
    let mut reached = 0usize;
    for (slug, case) in &cases {
        let has_book = case.sections().iter().any(|s| s.name() == "book.sh");
        let plan_blocks: Vec<_> = case
            .replay()
            .blocks()
            .iter()
            .map(errorloom::ReplayBlock::command)
            .filter(|command| *command == "dorc plan --book=book.sh")
            .collect();
        if has_book || plan_blocks.is_empty() {
            continue;
        }
        let results = replay_case(case, &DorcConsumer::new(), &RunEnv::new(), |command, _| {
            panic!("world-as-payload case `{slug}` declined `{command}` to the generic executor")
        })
        .unwrap_or_else(|error| panic!("replay `{slug}`: {error}"));
        for (block, routed) in case.replay().blocks().iter().zip(&results) {
            if block.command() != "dorc plan --book=book.sh" {
                continue;
            }
            assert_eq!(
                routed
                    .editable_render()
                    .map(errorloom::EditableRender::text)
                    .as_deref(),
                Some(routed.output()),
                "`{slug}` carries exact renderer provenance for its payload world"
            );
            reached = reached.saturating_add(1);
        }
    }
    assert!(
        reached > 0,
        "no world-as-payload case was found — this gate would pass vacuously"
    );
}

/// Every lint case drives its declared `dorc lint oracle.sh --no-tools` shape through the same
/// production report and tagged renderer as the CLI route. The defining code must be real: the
/// frontmatter slug alone cannot manufacture an output or editable provenance.
#[test]
fn lint_cases_replay_the_complete_production_report() {
    const LINT_CASES: [&str; 12] = [
        "missing-dialect-marker.loom",
        "marker-version-unrecognized.loom",
        "mark-unknown-verb.loom",
        "mark-rc-arity-exceeded.loom",
        "mark-standalone-rc-consumer.loom",
        "mark-hashcolon-malformed.loom",
        "munge-name-invalid.loom",
        "tolerates-unknown-dimension.loom",
        // dorc-lint's own findings, now registry codes (`288` §5) — honest-trigger for free.
        "unmodeled-wall-inventory.loom",
        "verdict-terminal-pipeline.loom",
        "authored-decline-class.loom",
        "authored-decline-class-unreadable.loom",
    ];
    for filename in LINT_CASES {
        let text = std::fs::read_to_string(corpus_dir().join(filename))
            .unwrap_or_else(|error| panic!("read lint case `{filename}`: {error}"));
        let case = Case::parse(&text)
            .unwrap_or_else(|error| panic!("parse lint case `{filename}`: {error}"));
        let slug = case
            .frontmatter()
            .scalar("code")
            .unwrap_or_else(|| panic!("lint case `{filename}` has code"));
        let source = case
            .sections()
            .iter()
            .find(|section| section.name() == "oracle.sh")
            .unwrap_or_else(|| panic!("lint case `{filename}` has oracle source"))
            .content();
        let production = dorc_lint::lint_materialized_source(
            String::from("oracle.sh"),
            String::from(source),
            dorc_lint::SourcePolicy {
                tools_enabled: false,
            },
        );
        assert!(
            production
                .report()
                .findings
                .iter()
                .any(|finding| finding.code == slug),
            "lint case `{filename}` must fire `{slug}`: {:?}",
            production.report().findings
        );
        let replay = replay_case(
            &case,
            &DorcConsumer::new(),
            &RunEnv::new(),
            |_command, _context| {
                panic!("lint case `{filename}` must take the direct production route")
            },
        )
        .unwrap_or_else(|error| panic!("replay lint case `{filename}`: {error}"));
        assert_eq!(replay.len(), 1, "lint case `{filename}` has one replay");
        let expected = case.replay().blocks()[0].output();
        assert_eq!(replay[0].output(), expected, "lint case `{filename}` bytes");
        assert_eq!(
            replay[0]
                .editable_render()
                .map(errorloom::EditableRender::text),
            Some(replay[0].output().to_owned()),
            "lint case `{filename}` keeps exact renderer provenance"
        );
        assert_eq!(
            production.human(&dorc_aid::RenderCtx::production()).text(),
            replay[0].output(),
            "lint case `{filename}` uses the production render"
        );
    }
}

/// `28L:fnd-lint-route-rerender-reads-const-not-mirror`, reproduced end-to-end by the conductor's
/// rehearsal: promote flips a register in the MIRROR and re-renders the case's transcript from it,
/// so a re-render that reads the compiled-in catalog publishes a lock and a transcript that
/// disagree — green at promote, red at the next rebuild. The why and arrangement routes always
/// threaded the mirror through `RenderCtx`; the lint route rendered eagerly inside
/// `lint_materialized_source`, before any caller could hand it one.
///
/// Both directions are STAGED on the mirror rather than borrowed from whichever code still happens
/// to be unwritten (`28L:fnd-shared-fixture-collision`): first the placeholder a `None` register
/// renders, then the words an overtype mints — each visible in the re-rendered transcript with no
/// rebuild in between, which is the promote-then-immediate-fixpoint shape.
#[test]
fn a_lint_route_re_render_reads_the_edited_mirror() {
    const CASE: &str = "mark-unknown-verb.loom";
    let text = std::fs::read_to_string(corpus_dir().join(CASE))
        .unwrap_or_else(|error| panic!("read `{CASE}`: {error}"));
    let case = Case::parse(&text).unwrap_or_else(|error| panic!("parse `{CASE}`: {error}"));
    let slug = case
        .frontmatter()
        .scalar("code")
        .unwrap_or_else(|| panic!("`{CASE}` declares a code"))
        .to_owned();
    let placeholder = format!("[unwritten: {slug}]");

    let mut consumer = DorcConsumer::new();
    consumer.set_message(&slug, None);
    let emptied = consumer.render_case(&case).expect("the case re-renders");
    assert!(
        emptied.contains(&placeholder),
        "an emptied register must re-render as its placeholder, not as the compiled-in words:\n\
         {emptied}"
    );

    let words = "fixture words the mirror alone carries";
    let baseline = consumer
        .editable_baseline(&case)
        .expect("the lint render carries editable provenance");
    let edited = baseline.render().text().replace(&placeholder, words);
    let edit = compile_section_edit(&baseline, &edited).expect("the overtype compiles");
    consumer
        .apply_section_edit(&edit)
        .expect("the mirror takes it");

    let overtyped = consumer.render_case(&case).expect("the case re-renders");
    assert!(
        overtyped.contains(words),
        "the re-render must carry the edited words with no rebuild:\n{overtyped}"
    );
    assert!(
        !overtyped.contains(&placeholder),
        "the placeholder must not survive its own overtype:\n{overtyped}"
    );
}

/// The production mark validator receives source bytes, not a defining-case slug. Removing each
/// defining carrier removes or changes its code while valid binds remain invisible to unknown-verb
/// recognition; every route is explicitly `--no-tools`.
#[test]
fn lint_mark_diagnostics_require_their_defining_source_shape() {
    const MUTATIONS: [(&str, &str, &str); 4] = [
        ("mark-unknown-verb.loom", "frobnicate", "asserts"),
        ("mark-rc-arity-exceeded.loom", "\n: refutes sm.a.C@y", ""),
        (
            "mark-standalone-rc-consumer.loom",
            ": sm.a.B@x",
            "foo : sm.a.B@x",
        ),
        (
            "mark-hashcolon-malformed.loom",
            "#: frobnicate",
            "# ordinary comment",
        ),
    ];
    for (filename, before, after) in MUTATIONS {
        let case = Case::parse(
            &std::fs::read_to_string(corpus_dir().join(filename))
                .unwrap_or_else(|error| panic!("read lint case `{filename}`: {error}")),
        )
        .unwrap_or_else(|error| panic!("parse lint case `{filename}`: {error}"));
        let slug = case.frontmatter().scalar("code").expect("case code");
        let source = case
            .sections()
            .iter()
            .find(|section| section.name() == "oracle.sh")
            .expect("oracle source")
            .content();
        let changed = source.replacen(before, after, 1);
        assert_ne!(changed, source, "mutation applies to `{filename}`");
        let report = dorc_lint::lint_materialized_source(
            String::from("oracle.sh"),
            changed,
            dorc_lint::SourcePolicy {
                tools_enabled: false,
            },
        );
        assert!(
            report
                .report()
                .findings
                .iter()
                .all(|finding| finding.code != slug),
            "removing `{before}` removes `{slug}`: {:?}",
            report.report().findings
        );
    }

    for filename in [
        "missing-dialect-marker.loom",
        "marker-version-unrecognized.loom",
    ] {
        let case = Case::parse(
            &std::fs::read_to_string(corpus_dir().join(filename))
                .unwrap_or_else(|error| panic!("read bind case `{filename}`: {error}")),
        )
        .unwrap_or_else(|error| panic!("parse bind case `{filename}`: {error}"));
        let source = case
            .sections()
            .iter()
            .find(|section| section.name() == "oracle.sh")
            .expect("oracle source")
            .content();
        let report = dorc_lint::lint_materialized_source(
            String::from("oracle.sh"),
            String::from(source),
            dorc_lint::SourcePolicy {
                tools_enabled: false,
            },
        );
        assert!(
            report
                .report()
                .findings
                .iter()
                .all(|finding| finding.code != "mark-unknown-verb"),
            "valid bind in `{filename}` is not a mark verb: {:?}",
            report.report().findings
        );
    }
}

/// The metadata-REGRESSION gate (`28L:fnd-case-frontmatter-overwrites-lock-metadata`): where a case
/// declares `when-fires`/`when-used`/`why` for a component that already carries committed metadata,
/// the two say the same thing.
///
/// The BELT to promote's braces: `dorc-loom promote` refuses the same drift before it writes
/// anything, and this catches the state promote never sees — a frontmatter edit sitting in the
/// worktree that nobody has promoted, which is the shape an author leaves behind when they meant
/// the prose edit and not the metadata one. Omitting a key means "keep the committed words"
/// (`kept_or_declared`), so reaching either gate means somebody TYPED different words. The repair
/// is either direction: restore the committed words, or promote with `--accept-metadata`.
#[test]
fn a_case_never_silently_rewrites_committed_metadata() {
    let arrangements = load_arrangement_corpus(&corpus_dir()).expect("load arrangement corpus");
    for entry in dorc_aid::arrangement::ARRANGEMENTS {
        let Some(case) = arrangements.get(entry.slug) else {
            continue;
        };
        for (key, committed) in [("when-used", entry.when_used), ("why", entry.why)] {
            assert_metadata_agrees(entry.slug, key, case, committed);
        }
    }

    let codes = load_corpus_by_slug(&corpus_dir()).expect("load catalog corpus");
    for entry in dorc_aid::catalog::CATALOG {
        let Some(case) = codes.get(entry.slug) else {
            continue;
        };
        for (key, committed) in [("when-fires", entry.when_fires), ("why", entry.why)] {
            assert_metadata_agrees(entry.slug, key, case, committed);
        }
    }
}

fn assert_metadata_agrees(slug: &str, key: &str, case: &Case, committed: &str) {
    let Some(declared) = case.frontmatter().scalar(key) else {
        return;
    };
    assert_eq!(
        declared, committed,
        "`{slug}`: the case's `{key}:` and the committed entry disagree.\n  case:      \
         {declared:?}\n  committed: {committed:?}\nOmit `{key}:` from the case to keep the \
         committed words, or run `mise run loom:compile crates/aid/tests/{slug}.loom && \
         mise run loom:promote crates/aid/tests/{slug}.loom` to publish the new ones."
    );
}
