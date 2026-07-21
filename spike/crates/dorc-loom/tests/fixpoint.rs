//! The render-level case fixpoint and hygiene gates.

#![expect(
    clippy::expect_used,
    reason = "corpus-loader helper over the committed tree; the no-panic lints guard untrusted input"
)]

use std::path::{Path, PathBuf};

use dorc_loom::DorcConsumer;
use errorloom::{Case, CaseFile, fixpoint_check};

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

/// GATE 1 — the errorloom RENDER-LEVEL fixpoint (`283` §4a): every committed case re-renders from the
/// current catalog to its own committed bytes. Catches a prose hand-edit to `catalog.rs` (a message
/// change moves the transcript away from the committed one).
#[test]
fn render_level_fixpoint_over_the_corpus() {
    let consumer = DorcConsumer::new();
    let corpus = load_corpus();
    fixpoint_check(&consumer, &corpus)
        .expect("committed corpus reproduces from the catalog (render-level fixpoint)");
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
