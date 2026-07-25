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
    DorcConsumer, generate_arrangement_lock, generate_catalog_lock, load_arrangement_corpus,
    load_corpus_by_slug, replay_case,
};
use errorloom::{Case, RunEnv};

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
    assert_eq!(
        generated, committed,
        "the committed catalog_lock.rs is not a fixpoint of the generator"
    );
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
    assert_eq!(
        generated, committed,
        "the committed arrangement_lock.rs is not a fixpoint of the generator"
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
            production.human().text(),
            replay[0].output(),
            "lint case `{filename}` uses the production render"
        );
    }
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
