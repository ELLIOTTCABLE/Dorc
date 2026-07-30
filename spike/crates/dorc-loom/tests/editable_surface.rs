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

use dorc_loom::{DorcConsumer, DorcEditableBaseline, DorcSectionEditRefusal, compile_section_edit};
use errorloom::{Case, RenderComponent, RunEnv};

/// One case, replayed through `consumer`'s own mirror into its editable baseline and transcript.
fn drive(consumer: &DorcConsumer, case: &Case) -> (DorcEditableBaseline, String) {
    let replay = dorc_loom::replay_case(case, consumer, &RunEnv::new(), |command, _| {
        panic!("the in-process driver must claim {command:?}")
    })
    .expect("case replays")
    .pop()
    .expect("one replay block");
    let transcript = replay.output().to_owned();
    let baseline = consumer
        .baseline_from_render(
            case,
            replay.editable_render().cloned().expect("editable render"),
        )
        .expect("editable baseline");
    (baseline, transcript)
}

/// One committed case, replayed through the production seat into its editable baseline.
fn driven(text: &str) -> (Case, DorcConsumer, DorcEditableBaseline, String) {
    let case = Case::parse(text).expect("case parses");
    let consumer = DorcConsumer::new();
    let (baseline, transcript) = drive(&consumer, &case);
    (case, consumer, baseline, transcript)
}

fn help_of(consumer: &DorcConsumer, slug: &str) -> dorc_aid::catalog::HelpRegister<String> {
    consumer
        .mirror()
        .iter()
        .find(|entry| entry.slug == slug)
        .map(|entry| entry.help.clone())
        .expect("the mirror carries the code")
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

/// The silent-corruption path this pack closes: a `= help:` line the render never emitted used to
/// be absorbed into the message register, rewriting somebody's sentence with somebody else's line.
/// It refuses now, and the refusal names the command that mints the register instead.
#[test]
fn added_help_line_refuses_and_names_the_command() {
    let (_, _, baseline, transcript) =
        driven(include_str!("../../aid/tests/cli-no-book-given.loom"));
    let edited = format!("{transcript}  = help: pass --book=PATH\n");
    let refusal =
        compile_section_edit(&baseline, &edited).expect_err("an added line is not a prose edit");
    assert!(
        matches!(
            refusal,
            DorcSectionEditRefusal::AddedLine {
                laid_out: 0,
                edited: 1,
                ..
            }
        ),
        "{refusal:?}"
    );
    assert!(
        refusal
            .explain("crates/aid/tests/cli-no-book-given.loom")
            .contains("dorc-loom add-register crates/aid/tests/cli-no-book-given.loom help"),
        "the refusal must name the repair verbatim: {}",
        refusal.explain("crates/aid/tests/cli-no-book-given.loom")
    );
}

/// The affordance the refusal names: mint the register, and the ORDINARY loop fills it — the
/// placeholder the render then grows is an edit region like any other.
#[test]
fn help_register_edit_round_trips() {
    let case = Case::parse(include_str!("../../aid/tests/cli-no-book-given.loom")).expect("parses");
    let mut consumer = DorcConsumer::new();
    assert_eq!(
        help_of(&consumer, "cli-no-book-given"),
        dorc_aid::catalog::HelpRegister::Absent
    );
    consumer
        .seed_help_register("cli-no-book-given")
        .expect("the register is absent");

    let (baseline, transcript) = drive(&consumer, &case);
    assert!(
        transcript.contains("= help:  [unwritten: cli-no-book-given.help]"),
        "the seeded register renders its own placeholder: {transcript:?}"
    );
    let edited = transcript.replace(
        "[unwritten: cli-no-book-given.help]",
        "give a path, or --book=PATH",
    );
    let edit = compile_section_edit(&baseline, &edited).expect("the placeholder is editable");
    assert_eq!(edit.section().field, "help");
    consumer
        .apply_section_edit(&edit)
        .expect("the mirror takes it");
    assert_eq!(
        help_of(&consumer, "cli-no-book-given"),
        dorc_aid::catalog::HelpRegister::Written(String::from("give a path, or --book=PATH"))
    );
}

/// Seeding twice is a mistake worth naming rather than a no-op that quietly loses an edit.
#[test]
fn seeding_an_existing_register_refuses() {
    let mut consumer = DorcConsumer::new();
    consumer
        .seed_help_register("cli-no-book-given")
        .expect("absent");
    assert_eq!(
        consumer.seed_help_register("cli-no-book-given"),
        Err(dorc_loom::SeedRefusal::AlreadyPresent(String::from(
            "cli-no-book-given"
        )))
    );
    assert_eq!(
        consumer.seed_help_register("no-such-code"),
        Err(dorc_loom::SeedRefusal::MissingCode(String::from(
            "no-such-code"
        )))
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
