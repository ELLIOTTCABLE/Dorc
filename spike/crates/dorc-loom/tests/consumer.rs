//! The Dorc [`DorcConsumer`] drives the WHOLE errorloom bless loop (`282` §2 · `283` §2) through the
//! public API only — the five `toy_consumer` scenarios ported onto real Dorc catalog codes with a
//! `FakeGit` (hermetic, no subprocess): run/fixpoint, prose-bless + re-render, structure-bless, and
//! the three refusals (both-classes, dirty-catalog, structure-drift), plus the fixpoint gate catching
//! a raw mirror hand-edit.
//!
//! `dangling-reference` (a spanless code with a SPACE-DELIMITED `{coord}` param) drives the prose
//! round-trip — a backtick-GLUED param would straddle the word/span boundary and mangle re-holing,
//! which is a `282` §5 sharp edge, not exercised here.

use std::path::Path;

use dorc_loom::DorcConsumer;
use errorloom::{
    BlessError, Case, CaseFile, Consumer, FakeGit, ModeRefusal, fixpoint_check, prose_bless,
    structure_bless,
};

const CASE_PATH: &str = "cases/dangling-reference.txt";
const CATALOG_PATH: &str = "crates/core/src/catalog.rs";
const CODE_PATH: &str = "crates/core/src/diag.rs";

/// The committed transcript for `slug`: render a skeleton case through a fresh consumer, so the
/// committed bytes ARE a fixpoint by construction (the render is deterministic).
fn committed(slug: &str, command: &str) -> String {
    let skeleton = format!("---\ncode: {slug}\n---\n-- replay --\n$ {command}\n");
    let case = Case::parse(&skeleton).expect("skeleton parses");
    DorcConsumer::new()
        .render_case(&case)
        .expect("skeleton renders")
}

fn message_of(consumer: &DorcConsumer, slug: &str) -> String {
    consumer
        .mirror()
        .iter()
        .find(|e| e.slug == slug)
        .and_then(|e| e.message.clone())
        .expect("mirror has the code's message")
}

#[test]
fn full_prose_bless_loop_then_structure_bless() {
    let committed = committed("dangling-reference", "dorc plan --book=book.sh");
    assert!(
        committed.contains("typo"),
        "the base render carries the word we edit: {committed}"
    );

    // run: the committed catalog reproduces the committed transcript (fixpoint).
    let corpus = vec![CaseFile::new(CASE_PATH, committed.clone())];
    fixpoint_check(&DorcConsumer::new(), &corpus).expect("committed corpus is a fixpoint");

    // The author edits prose in the transcript (typo -> mistake), far from the `{coord}` param.
    let edited = committed.replace("typo", "mistake");
    let git = FakeGit::new()
        .commit(CASE_PATH, committed.clone())
        .mark_dirty(CASE_PATH);

    let mut consumer = DorcConsumer::new();
    let edited_corpus = vec![CaseFile::new(CASE_PATH, edited)];
    let result = prose_bless(&mut consumer, &git, &edited_corpus, CATALOG_PATH.as_ref())
        .expect("prose-bless succeeds");

    // The mirror absorbed the edit (prose, with the `{coord}` hole re-held, not baked).
    let msg = message_of(&consumer, "dangling-reference");
    assert!(msg.contains("mistake"), "mirror absorbed the edit: {msg}");
    assert!(!msg.contains("typo"), "the old word is gone: {msg}");
    assert!(msg.contains("{coord}"), "the param stayed a hole: {msg}");

    // Every case re-rendered with the new prose; the regenerated corpus is itself a fixpoint.
    let regenerated = result
        .regenerated()
        .get(Path::new(CASE_PATH))
        .expect("case regenerated");
    assert!(regenerated.contains("mistake"));
    let after = vec![CaseFile::new(CASE_PATH, regenerated.clone())];
    fixpoint_check(&consumer, &after).expect("regenerated corpus is a fixpoint");

    // A code/arrangement change: structure-bless regenerates from the (clean, now-edited) catalog.
    let git = FakeGit::new()
        .commit(CASE_PATH, regenerated.clone())
        .mark_dirty(CODE_PATH);
    let structural = structure_bless(&consumer, &git, &after, CATALOG_PATH.as_ref())
        .expect("structure-bless succeeds");
    let restructured = structural
        .regenerated()
        .get(Path::new(CASE_PATH))
        .expect("case regenerated");
    assert!(restructured.contains("mistake"));
}

#[test]
fn both_classes_dirty_refuses() {
    let committed = committed("whylog-absent", "dorc why --last");
    let mut consumer = DorcConsumer::new();
    let git = FakeGit::new()
        .commit(CASE_PATH, committed.clone())
        .mark_dirty(CASE_PATH)
        .mark_dirty(CODE_PATH);
    let corpus = vec![CaseFile::new(CASE_PATH, committed)];
    let err = prose_bless(&mut consumer, &git, &corpus, CATALOG_PATH.as_ref()).unwrap_err();
    assert_eq!(err, BlessError::Mode(ModeRefusal::BothClasses));
}

#[test]
fn dirty_catalog_refuses() {
    let committed = committed("whylog-absent", "dorc why --last");
    let mut consumer = DorcConsumer::new();
    let git = FakeGit::new()
        .commit(CASE_PATH, committed.clone())
        .mark_dirty(CASE_PATH)
        .mark_dirty(CATALOG_PATH);
    let corpus = vec![CaseFile::new(CASE_PATH, committed)];
    let err = prose_bless(&mut consumer, &git, &corpus, CATALOG_PATH.as_ref()).unwrap_err();
    assert_eq!(err, BlessError::Mode(ModeRefusal::DirtyCatalog));
}

#[test]
fn structure_drift_within_prose_bless_refuses() {
    // HEAD's committed transcript carries a DRIFTED title arrangement (`[…-drift]`) while the current
    // render is `[dangling-reference]`; the author also edited prose, but only the case shows dirty —
    // the baseline-verify catches the structural drift and demands structure-bless first.
    let committed = committed("dangling-reference", "dorc plan --book=book.sh");
    let head_drift = committed.replace("[dangling-reference]", "[dangling-reference-drift]");
    let work = committed.replace("typo", "mistake");

    let mut consumer = DorcConsumer::new();
    let git = FakeGit::new()
        .commit(CASE_PATH, head_drift)
        .mark_dirty(CASE_PATH);
    let corpus = vec![CaseFile::new(CASE_PATH, work)];
    let err = prose_bless(&mut consumer, &git, &corpus, CATALOG_PATH.as_ref()).unwrap_err();
    assert!(
        matches!(err, BlessError::StructureDrift { .. }),
        "expected StructureDrift, got {err:?}"
    );
}

#[test]
fn world_as_pipeline_marker_pilot_fires_the_real_gate() {
    // The one real-fired proof (`28A` §2n): a wrong-version marked oracle drives the REAL in-process
    // marker gate, so the render is SPANNED (a caret frame into the materialized source), not the
    // spanless world-as-payload path — and it is what the binary actually produces.
    let case_text = "---\ncode: marker-version-unrecognized\n---\n\
                     -- oracle.sh --\n# dorc-lang/v0.1\n\
                     apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }\n\
                     -- replay --\n$ dorc lint oracle.sh\n";
    let case = Case::parse(case_text).expect("case parses");
    let rendered = DorcConsumer::new()
        .render_case(&case)
        .expect("pipeline render");
    assert!(
        rendered.contains(
            "error[marker-version-unrecognized]: [unwritten: marker-version-unrecognized]"
        ),
        "the unwritten render fires: {rendered}"
    );
    assert!(
        rendered.contains("--> oracle.sh:2:"),
        "a spanned caret frame from the real gate (not the spanless payload path): {rendered}"
    );
}

#[test]
fn fixpoint_gate_catches_a_catalog_hand_edit() {
    // Hand-edit the mirror (prose) without regenerating the committed transcript: the fixpoint gate
    // sees the committed transcript no longer reproduce.
    let committed = committed("whylog-absent", "dorc why --last");
    let mut consumer = DorcConsumer::new();
    consumer.set_message("whylog-absent", Some("sm tampered message".to_owned()));
    let corpus = vec![CaseFile::new(CASE_PATH, committed)];
    let err = fixpoint_check(&consumer, &corpus).unwrap_err();
    assert!(matches!(err, BlessError::Fixpoint { .. }), "got {err:?}");
}
