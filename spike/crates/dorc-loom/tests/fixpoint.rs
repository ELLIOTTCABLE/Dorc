//! The two phase-4 fixpoint gates (`283` §4a · `28A` §2g — BOTH required) plus the promote-v2
//! orchestrator entry. GIT-FREE CI gates (`283:dec-gates-are-git-free`): the render-level fixpoint
//! and hygiene run over the committed case corpus in-process; only the interactive BLESS touches git
//! (`SubprocessGit`, gated). At this checkpoint the corpus is the two phase-4 pilots (`283` §1d);
//! before step 6 populates it the gates pass vacuously.

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

/// The promote-v2 ORCHESTRATOR entry (`283:dec-promote-v2-composes-errorloom`; BLESS-law — the builder
/// BUILDS this, the ORCHESTRATOR runs it with `DORC_CATALOG_PROMOTE=1` from a fresh binary and inspects
/// the diff). It composes errorloom's interactive bless (`prose_bless` driving the [`DorcConsumer`]
/// over `SubprocessGit`) with `core::catalog::serialize` of the resulting mirror, written to
/// `target/` for the splice + `cargo fmt`. A no-op without the env, so the ordinary suite is inert;
/// `SubprocessGit` rides ONLY this bless, never the git-free CI gates above.
#[test]
fn promote_v2_writer_gated() {
    if std::env::var("DORC_CATALOG_PROMOTE").as_deref() != Ok("1") {
        return;
    }
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/dorc-loom -> crates -> spike");
    let catalog = Path::new("crates/core/src/catalog.rs");
    let git = errorloom::SubprocessGit::new(repo);

    let mut consumer = DorcConsumer::new();
    let corpus = load_corpus();
    let result = errorloom::prose_bless(&mut consumer, &git, &corpus, catalog)
        .expect("prose-bless over the corpus");

    // Overwrite the corpus with the re-rendered transcripts, then codegen the catalog from the
    // now-edited mirror — the orchestrator diffs, splices into catalog.rs, and `cargo fmt`s.
    for (path, text) in result.regenerated() {
        std::fs::write(corpus_dir().join(path), text).expect("write regenerated case");
    }
    let promoted = dorc_core::catalog::serialize(consumer.mirror());
    let out = repo.join("target/catalog-promoted.rs");
    std::fs::write(&out, promoted).expect("write promoted catalog");
    eprintln!(
        "promote-v2: wrote {} (diff, splice into catalog.rs, cargo fmt)",
        out.display()
    );
}
