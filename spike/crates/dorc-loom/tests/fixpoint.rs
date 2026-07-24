//! The render-level case fixpoint and hygiene gates.

#![expect(
    clippy::expect_used,
    reason = "corpus-loader helper over the committed tree; the no-panic lints guard untrusted input"
)]

use std::path::{Path, PathBuf};

use dorc_loom::{DorcConsumer, generate_catalog_lock, load_corpus_by_slug, replay_case};
use errorloom::{Case, CaseFile, RunEnv, fixpoint_check};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("cases")
}

fn committed_lock() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/src/catalog_lock.rs")
}

/// The generated-catalog byte-identity gate (`28A` §4 · `282:rul-catalog-lock-is-generated-whole`):
/// regenerating the whole lock from the committed corpus reproduces the committed `catalog_lock.rs`
/// byte-for-byte. This is the second fixpoint (the render-level case fixpoint is the first). A
/// hand-edit to the generated lock, or drift between a case and its generated row, trips here.
#[test]
fn generated_lock_reproduces_the_committed_bytes() {
    let consumer = DorcConsumer::new();
    let cases = load_corpus_by_slug(&corpus_dir()).expect("load corpus");
    let generated = generate_catalog_lock(&consumer, &cases).expect("generate lock");
    let committed = std::fs::read_to_string(committed_lock()).expect("read committed lock");
    assert_eq!(
        generated, committed,
        "the committed catalog_lock.rs is not a fixpoint of the generator"
    );
}

/// Every committed `cases/*.loom`, sorted for determinism (`inv-determinism`). A missing/empty dir
/// yields an empty corpus, so the gates pass vacuously until the pilots land.
fn load_corpus() -> Vec<CaseFile> {
    let mut cases: Vec<CaseFile> = Vec::new();
    let Ok(entries) = std::fs::read_dir(corpus_dir()) else {
        return cases;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "loom") {
            let name = path
                .file_name()
                .expect("case file has a name")
                .to_string_lossy()
                .into_owned();
            let text = std::fs::read_to_string(&path).expect("read case file");
            cases.push(CaseFile::new(name, text));
        }
    }
    cases.sort_by(|a, b| a.path().cmp(b.path()));
    cases
}

/// The exact direct-plan corpus specimens re-render from the current catalog.
/// Other committed commands now require the configured generic executor and are
/// intentionally outside this in-process renderer's authority.
#[test]
fn direct_plan_render_fixpoint() {
    const DIRECT_PLAN_CASES: [&str; 4] = [
        "cmdsub-operand-top.loom",
        // The external-linter relays (`288` §5) are world-as-payload for the same reason the
        // original specimen is: their honest world is an expensive one replay never enters.
        "lint-tool-absent.loom",
        "lint-tool-output-unparsable.loom",
        "lint-tool-failed-without-findings.loom",
    ];
    let consumer = DorcConsumer::new();
    let corpus: Vec<_> = load_corpus()
        .into_iter()
        .filter(|case| {
            DIRECT_PLAN_CASES
                .iter()
                .any(|name| case.path() == Path::new(name))
        })
        .collect();
    assert_eq!(
        corpus.len(),
        DIRECT_PLAN_CASES.len(),
        "every direct-plan specimen is committed"
    );
    fixpoint_check(&consumer, &corpus).expect("direct-plan cases reproduce from the catalog");
}

/// Every committed case is txtar/hygiene-clean and surfaces its own `code` slug in each replay block
/// (`282` §2 coherence gate) — the corpus can round-trip through the container.
#[test]
fn corpus_cases_are_hygienic() {
    for case_file in load_corpus() {
        let case = Case::parse(case_file.text())
            .unwrap_or_else(|e| panic!("case `{}` parses: {e}", case_file.path().display()));
        case.check_hygiene(Some("code"))
            .unwrap_or_else(|e| panic!("case `{}` hygiene: {e}", case_file.path().display()));
    }
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
