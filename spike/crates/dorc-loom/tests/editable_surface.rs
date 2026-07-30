//! The editable-surface pack: what an author can reach from a committed transcript, and what the
//! tool refuses when they reach for something the render never emitted (`28L` §2 D2).
//!
//! Every fixture is an in-memory copy of a committed case, so nothing here writes to the corpus:
//! the dogfood landings are the conductor's rehearsal, not this suite's.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "fixture harness over the committed corpus; the no-panic lints guard untrusted input"
)]

use dorc_loom::{DorcConsumer, DorcEditableBaseline, compile_section_edit};
use errorloom::{Case, RenderComponent, RunEnv};

/// One committed case, replayed through the production seat into its editable baseline.
fn driven(text: &str) -> (Case, DorcConsumer, DorcEditableBaseline, String) {
    let case = Case::parse(text).expect("case parses");
    let consumer = DorcConsumer::new();
    let replay = dorc_loom::replay_case(&case, &consumer, &RunEnv::new(), |command, _| {
        panic!("the in-process driver must claim {command:?}")
    })
    .expect("case replays")
    .pop()
    .expect("one replay block");
    let transcript = replay.output().to_owned();
    let baseline = consumer
        .baseline_from_render(
            &case,
            replay.editable_render().cloned().expect("editable render"),
        )
        .expect("editable baseline");
    (case, consumer, baseline, transcript)
}

fn sections(baseline: &DorcEditableBaseline) -> Vec<(String, &'static str)> {
    baseline
        .render()
        .components()
        .iter()
        .filter_map(|component| match component {
            RenderComponent::EditableSection(section) => {
                Some((section.id().owner.clone(), section.id().field))
            }
            _ => None,
        })
        .collect()
}

/// The crux of the arc: a code with no authored words renders a COMPUTED placeholder, and
/// overtyping that placeholder in the transcript is how the code acquires its first message —
/// no catalog hand-edit, no stored placeholder row.
#[test]
fn overtype_placeholder_mints_words() {
    let (_, mut consumer, baseline, transcript) =
        driven(include_str!("../../aid/tests/whylog-unwritten.loom"));
    assert_eq!(
        sections(&baseline),
        vec![(String::from("whylog-unwritten"), "message")],
        "the placeholder wears the message register's face"
    );
    assert_eq!(
        consumer
            .mirror()
            .iter()
            .find(|entry| entry.slug == "whylog-unwritten")
            .and_then(|entry| entry.message.clone()),
        None,
        "nothing is stored before the edit"
    );

    let edited = transcript.replace(
        "[unwritten: whylog-unwritten]",
        "the run finished but its why durable did not land",
    );
    let edit = compile_section_edit(&baseline, &edited).expect("the overtype compiles");
    assert_eq!(edit.section().field, "message");
    consumer
        .apply_section_edit(&edit)
        .expect("the mirror takes it");
    assert_eq!(
        consumer
            .mirror()
            .iter()
            .find(|entry| entry.slug == "whylog-unwritten")
            .and_then(|entry| entry.message.clone()),
        Some(String::from(
            "the run finished but its why durable did not land"
        ))
    );
}

/// The house idiom: twenty-six committed messages backtick-quote a value, and until now that was
/// the one spelling an author could not newly write (`28L:rul-attached-markers-land`).
#[test]
fn a_backticked_marker_compiles() {
    let (_, _, baseline, transcript) =
        driven(include_str!("../../aid/tests/cli-flag-requires-mode.loom"));
    let edited = transcript.replace(
        "sm --whylog is only valid",
        "sm the flag `{{flag}}` is only valid",
    );
    let edit = compile_section_edit(&baseline, &edited).expect("a glued marker compiles");
    assert_eq!(
        edit.compiled().text(),
        "sm the flag `--whylog` is only valid with dorc why"
    );
    assert_eq!(
        edit.compiled().used(),
        &[
            dorc_loom::TemplateVariableName(String::from("flag")),
            dorc_loom::TemplateVariableName(String::from("mode")),
        ],
        "the retyped marker binds and the untouched variable is preserved"
    );
}

/// A placeholder long enough to WRAP is still one section: the break weft minted inside it is the
/// register's own space wearing the renderer's clothes, and a second section here would leave half
/// the placeholder unaddressable.
#[test]
fn a_wrapped_placeholder_is_one_section() {
    let (_, _, baseline, transcript) = driven(include_str!(
        "../../aid/tests/host-evidence-admission-refused.loom"
    ));
    assert!(
        transcript.contains("[unwritten:\nhost-evidence-admission-refused]"),
        "the fixture must actually wrap: {transcript:?}"
    );
    assert_eq!(
        sections(&baseline),
        vec![(String::from("host-evidence-admission-refused"), "message")]
    );
}
