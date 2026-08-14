//! The arrangement-registry prose loop, end to end
//! (`289:rul-arrangement-home-is-registry-plus-transcripts`).
//!
//! `282:rul-transcript-is-the-authoring-surface` says prose is authored by editing a committed
//! transcript and compiling it back. This pins that the CHROME register works the same way as
//! the catalog registers do, over the help-page pilot: an edit to a word of the committed
//! `$ dorc --help` transcript attributes to the registry entry the render read, lands on the
//! mirror, reaches the regenerated lock, and comes back out of the re-rendered case.
//!
//! It is deliberately the whole loop in one test rather than four unit assertions: every link
//! is trivial on its own, and the only thing worth pinning is that they compose — which is
//! exactly what the two fixpoint gates cannot see, since they check the RESTING state.

#![expect(
    clippy::panic,
    reason = "fixture loader over the committed corpus; the no-panic lints guard untrusted input"
)]

use std::path::Path;

use dorc_aid::prose::ProseTier;
use dorc_loom::{
    DorcConsumer, compile_preview, generate_arrangement_lock, load_arrangement_corpus,
};
use errorloom::{Case, CaseRenderer as _};

const CASE: &str = "cli-help-page.loom";
const SLUG: &str = "cli-help-page";

fn corpus_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests")
}

fn case() -> Case {
    let text = std::fs::read_to_string(corpus_dir().join(CASE))
        .unwrap_or_else(|e| panic!("read {CASE}: {e}"));
    Case::parse(&text).unwrap_or_else(|e| panic!("parse {CASE}: {e}"))
}

#[test]
fn an_edited_help_word_reaches_the_lock_and_the_re_rendered_case() {
    let case = case();
    let committed = case.replay().blocks()[0].output();
    let mut consumer = DorcConsumer::new();
    let baseline = consumer
        .editable_baseline(&case)
        .expect("the page case renders from the registry");
    assert_eq!(
        baseline.render().text(),
        committed,
        "the committed transcript IS the registry's render"
    );

    let edited = committed.replace("spec-mining", "specification-mining");
    assert_ne!(edited, committed, "the fixture still carries the old word");

    let preview = compile_preview(&baseline, &edited).expect("the help-page edit compiles");
    let section = match preview.sections() {
        [section] => section.section(),
        other => panic!("exactly one section changed, got {}", other.len()),
    };
    assert_eq!(section.owner, SLUG);
    assert_eq!(section.field, dorc_loom::ARRANGEMENT_FIELD);

    consumer
        .apply_preview(&preview)
        .expect("apply to the mirror");
    let entry = consumer
        .arrangements()
        .iter()
        .find(|entry| entry.slug == SLUG)
        .expect("the pilot entry");
    match &entry.words {
        Some(ProseTier::Slop(words)) => assert_eq!(
            words,
            std::slice::from_ref(&edited),
            "the entry now holds exactly the edited page"
        ),
        other => panic!("a human edit makes the entry authored, got {other:?}"),
    }

    let corpus = load_arrangement_corpus(&corpus_dir()).expect("load arrangement corpus");
    let lock = generate_arrangement_lock(&consumer, &corpus).expect("regenerate the lock");
    assert!(
        lock.contains("Some(ProseTier::Slop(&[\"dorc -- specification-mining"),
        "the regenerated lock carries the edited words, no longer marked migrated"
    );

    let rendered = consumer.render_case(&case).expect("re-render the case");
    assert!(
        rendered.contains("specification-mining"),
        "the re-rendered transcript comes back out of the edited registry"
    );
    assert_eq!(
        Case::parse(&rendered)
            .expect("the re-rendered case parses")
            .replay()
            .blocks()[0]
            .output(),
        edited,
        "the published transcript is exactly what the human wrote"
    );
}

/// The chain fence (`289:rider-arrangement-home-anticipates-chains`): storage is
/// sequence-shaped, but nothing re-splits an edited string into words, so an edit against a
/// multi-word entry REFUSES instead of guessing a boundary.
#[test]
fn an_edit_against_a_word_sequence_refuses() {
    let case = case();
    let mut consumer = DorcConsumer::new();
    let baseline = consumer
        .editable_baseline(&case)
        .expect("the page case renders from the registry");
    let committed = case.replay().blocks()[0].output();
    let preview = compile_preview(&baseline, &committed.replace("spec-mining", "spec mining"))
        .expect("the edit compiles");

    consumer.set_arrangement_words(
        SLUG,
        Some(ProseTier::Migrated(vec![
            "one ".to_owned(),
            "two".to_owned(),
        ])),
    );

    let refusal = consumer
        .apply_preview(&preview)
        .expect_err("a multi-word entry refuses the edit");
    assert_eq!(
        refusal,
        dorc_loom::DorcApplyRefusal::ArrangementIsSequenceStructured(SLUG.to_owned())
    );
}

/// The honest-trigger half (`289:rul-worldless-route-honest-trigger`): a page case whose replay
/// command renders a DIFFERENT page than it declares is refused, never quietly transcribed.
#[test]
fn a_page_case_declaring_another_arrangement_is_refused() {
    let text = std::fs::read_to_string(corpus_dir().join(CASE)).expect("read the pilot case");
    let mistaken = text.replace(
        "arrangement: cli-help-page",
        "arrangement: cli-usage-synopsis",
    );
    let case = Case::parse(&mistaken).expect("the mistaken case parses");
    assert!(
        DorcConsumer::new().render_case(&case).is_err(),
        "a command that renders another page cannot prove this case's transcript"
    );
}

/// The generation lag, stated: a case naming a slug with no committed row renders a refusal that
/// names the repair, rather than an empty page (the catalog's `is_case_owned` lag, same shape).
#[test]
fn a_case_naming_an_unregistered_arrangement_names_the_repair() {
    let case = Case::parse(
        "---\narrangement: cli-nonexistent-page\nwhen-used: nothing.\nwhy: nothing.\n---\n\
         -- replay --\n$ dorc --help\n",
    )
    .expect("the case parses");
    let error = DorcConsumer::new()
        .editable_baseline(&case)
        .expect_err("an unregistered arrangement has no render");
    assert!(error.contains("promote the case"), "{error}");
}

/// The corpus split: arrangement cases are keyed by `arrangement`, code cases by `code`, and the
/// two collections never see each other's members (a page case has no catalog row to generate).
#[test]
fn the_two_corpora_partition_the_collection() {
    let arrangements = load_arrangement_corpus(&corpus_dir()).expect("arrangement corpus");
    let codes = dorc_loom::load_corpus_by_slug(&corpus_dir()).expect("code corpus");
    assert!(arrangements.contains_key(SLUG));
    assert!(!codes.contains_key(SLUG));
    let shared: Vec<_> = arrangements
        .keys()
        .filter(|slug| codes.contains_key(*slug))
        .collect();
    assert!(shared.is_empty(), "no case defines both: {shared:?}");
}
