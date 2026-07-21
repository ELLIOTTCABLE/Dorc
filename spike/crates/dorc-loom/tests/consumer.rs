//! Dorc-specific case rendering, editable baselines, compiled application, and fixpoint coverage.

#![expect(
    clippy::expect_used,
    reason = "committed-case helpers over the known-good test tree; the no-panic lints guard untrusted input"
)]

use std::path::Path;

use dorc_loom::{DorcConsumer, TemplateVariableName, compile_section_edit};
use errorloom::{
    BlessError, Case, CaseFile, CaseRenderer, FakeGit, fixpoint_check, structure_bless,
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
fn editable_baseline_renders_a_defining_case_with_help() {
    let case = Case::parse(include_str!("../cases/whylog-book-desync.txt")).expect("case parses");
    let baseline = DorcConsumer::new()
        .editable_baseline(&case)
        .expect("editable baseline");
    assert!(baseline.render().text().contains("= help:"));
    assert!(
        baseline
            .render()
            .components()
            .iter()
            .filter_map(|component| match component {
                errorloom::RenderComponent::EditableSection(section) => Some(section.id().field),
                _ => None,
            })
            .any(|field| field == "message")
    );
    assert!(
        baseline
            .render()
            .components()
            .iter()
            .filter_map(|component| match component {
                errorloom::RenderComponent::EditableSection(section) => Some(section.id().field),
                _ => None,
            })
            .any(|field| field == "help")
    );
    assert!(baseline.variables().values().any(|variables| {
        variables.get(&TemplateVariableName(String::from("which"))) == Some(&String::from("book"))
    }));
}

#[test]
fn fixpoint_gate_catches_a_catalog_hand_edit() {
    let committed = committed("whylog-absent", "dorc why --last");
    let mut consumer = DorcConsumer::new();
    consumer.set_message("whylog-absent", Some("sm tampered message".to_owned()));
    let corpus = vec![CaseFile::new(CASE_PATH, committed)];
    let err = fixpoint_check(&consumer, &corpus).unwrap_err();
    assert!(matches!(err, BlessError::Fixpoint { .. }), "got {err:?}");
}

#[test]
fn structure_bless_regenerates_a_dorc_case() {
    let committed = committed("whylog-absent", "dorc why --last");
    let corpus = vec![CaseFile::new(CASE_PATH, committed.clone())];
    let git = FakeGit::new().mark_dirty(CODE_PATH);
    let result = structure_bless(&DorcConsumer::new(), &git, &corpus, CATALOG_PATH.as_ref())
        .expect("structure bless succeeds");
    assert_eq!(
        result.regenerated().get(Path::new(CASE_PATH)),
        Some(&committed)
    );
}

#[test]
fn applied_template_regenerates_complete_multi_replay_case() {
    let text = "---\ncode: dangling-reference\nwhen-fires: preserved frontmatter\n---\n\
                -- input.txt --\nsource bytes stay unchanged\n\
                -- replay --\n\
                $ dorc plan --book=input.txt\nstale human bytes\n\
                $ dorc plan --book=input.txt --format=jsonl\nstale machine bytes\n";
    let case = Case::parse(text).expect("case parses");
    let mut consumer = DorcConsumer::new();
    let baseline = consumer
        .editable_baseline(&case)
        .expect("editable baseline");
    let original = "sm coordinate sm.dorc.Package:nginx resolved DANGLING — the kind's resolver reports no such entity (a likely typo / stale name); it degrades to may-alias (the site runs)";
    assert!(baseline.render().text().contains(original));
    let dirty = baseline.render().text().replace(
        original,
        "{{coord}} is dangling; inspect {{coord}} before applying",
    );
    let edit = compile_section_edit(&baseline, &dirty).expect("strict markers compile");
    consumer.apply_section_edit(&edit).expect("apply edit");

    assert_eq!(
        message_of(&consumer, "dangling-reference"),
        "{{coord}} is dangling; inspect {{coord}} before applying"
    );
    let regenerated = consumer.render_cases(&[case]).expect("render cases");
    assert_eq!(regenerated.len(), 1);
    let regenerated = &regenerated[0];
    assert!(regenerated.contains("when-fires: preserved frontmatter"));
    assert!(regenerated.contains("-- input.txt --\nsource bytes stay unchanged\n"));
    assert!(regenerated.contains("$ dorc plan --book=input.txt"));
    assert!(regenerated.contains("$ dorc plan --book=input.txt --format=jsonl"));
    assert!(regenerated.contains("sm.dorc.Package:nginx is dangling"));
    assert!(regenerated.contains("{\"code\":\"dangling-reference\",\"severity\":\"note\"}"));
    assert!(!regenerated.contains("stale human bytes"));
    assert!(!regenerated.contains("stale machine bytes"));

    let reparsed = Case::parse(regenerated).expect("regenerated case parses");
    assert_eq!(reparsed.replay().blocks().len(), 2);
    fixpoint_check(&consumer, &[CaseFile::new(CASE_PATH, regenerated.clone())])
        .expect("mutated consumer reproduces regenerated case");
}
