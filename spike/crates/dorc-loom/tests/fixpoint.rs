//! The render-level case fixpoint and hygiene gates.

#![expect(
    clippy::expect_used,
    reason = "corpus-loader helper over the committed tree; the no-panic lints guard untrusted input"
)]

use std::path::{Path, PathBuf};

use dorc_loom::{DorcConsumer, replay_case};
use errorloom::{Case, CaseFile, RunEnv, fixpoint_check};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("cases")
}

/// Every committed `cases/*.txt`, sorted for determinism (`inv-determinism`). A missing/empty dir
/// yields an empty corpus, so the gates pass vacuously until the pilots land.
fn load_corpus() -> Vec<CaseFile> {
    let mut cases: Vec<CaseFile> = Vec::new();
    let Ok(entries) = std::fs::read_dir(corpus_dir()) else {
        return cases;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "txt") {
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

/// The exact direct-plan corpus specimen re-renders from the current catalog.
/// Other committed commands now require the configured generic executor and are
/// intentionally outside this in-process renderer's authority.
#[test]
fn direct_plan_render_fixpoint() {
    let consumer = DorcConsumer::new();
    let corpus: Vec<_> = load_corpus()
        .into_iter()
        .filter(|case| case.path() == Path::new("cmdsub-operand-top.txt"))
        .collect();
    assert_eq!(corpus.len(), 1, "the direct-plan specimen is committed");
    fixpoint_check(&consumer, &corpus).expect("direct-plan case reproduces from the catalog");
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
    const LINT_CASES: [&str; 8] = [
        "missing-dialect-marker.txt",
        "marker-version-unrecognized.txt",
        "mark-unknown-verb.txt",
        "mark-rc-arity-exceeded.txt",
        "mark-standalone-rc-consumer.txt",
        "mark-hashcolon-malformed.txt",
        "munge-name-invalid.txt",
        "tolerates-unknown-dimension.txt",
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
        ("mark-unknown-verb.txt", "frobnicate", "asserts"),
        ("mark-rc-arity-exceeded.txt", "\n: refutes sm.a.C@y", ""),
        (
            "mark-standalone-rc-consumer.txt",
            ": sm.a.B@x",
            "foo : sm.a.B@x",
        ),
        (
            "mark-hashcolon-malformed.txt",
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
        "missing-dialect-marker.txt",
        "marker-version-unrecognized.txt",
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
