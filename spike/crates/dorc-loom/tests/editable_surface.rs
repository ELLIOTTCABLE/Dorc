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
use errorloom::{Case, CaseRenderer, RenderComponent, RunEnv};

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
///
/// A genuine BLANK line (two newlines), not one: read-in normalization (`28L`/`282` §3) collapses a
/// single embedded newline to a space, so a soft rewrap can no longer trip this refusal — only a
/// real paragraph break still reads as "a line the render never emitted"
/// (`prose_re_wrap_compiles_with_no_stored_newline` pins the relaxed half).
#[test]
fn added_help_line_refuses_and_names_the_command() {
    let (_, _, baseline, transcript) =
        driven(include_str!("../../aid/tests/cli-no-book-given.loom"));
    let edited = format!("{transcript}\n  = help: pass --book=PATH\n");
    let refusal =
        compile_section_edit(&baseline, &edited).expect_err("an added line is not a prose edit");
    assert!(
        matches!(
            refusal,
            DorcSectionEditRefusal::AddedLine {
                laid_out: 0,
                edited: 2,
                ..
            }
        ),
        "{refusal:?}"
    );
    assert!(
        refusal
            .explain(std::path::Path::new(
                "crates/aid/tests/cli-no-book-given.loom"
            ))
            .contains("dorc-loom add-register crates/aid/tests/cli-no-book-given.loom help"),
        "the refusal must name the repair verbatim: {}",
        refusal.explain(std::path::Path::new(
            "crates/aid/tests/cli-no-book-given.loom"
        ))
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

/// The one-step loop (`28H:finding-why-render-reads-the-const-not-the-mirror`): a chrome row
/// edited in a transcript re-renders through the EDITED registry, with no intermediate rebuild —
/// the re-render reads the same context the edit landed in, not the compiled-in table.
#[test]
fn one_step_why_row_edit() {
    let case = Case::parse(include_str!(
        "../../aid/tests/why-drift-analysis-suppressed.loom"
    ))
    .expect("parses");
    let mut consumer = DorcConsumer::new();
    let (baseline, transcript) = drive(&consumer, &case);
    let edited = transcript.replace("   oracles: firewall", "   loaded oracles: firewall");
    assert_ne!(edited, transcript, "the fixture must actually edit a row");

    let preview = dorc_loom::compile_preview(&baseline, &edited).expect("the chrome line compiles");
    consumer
        .apply_preview(&preview)
        .expect("the registry mirror takes it");
    let rerendered = consumer.render_case(&case).expect("the case re-renders");
    assert!(
        rerendered.contains("loaded oracles: firewall.oracle.sh"),
        "the re-render must read the edited row: {rerendered}"
    );
    assert!(
        dorc_aid::arrangement::ARRANGEMENTS
            .iter()
            .any(|entry| entry.slug == "why-receipt-oracles"
                && entry.words.words() == Some(&["oracles: ", ""][..])),
        "the compiled-in table is untouched — nothing was rebuilt"
    );
}

/// `vars` reports the render an edit compiles against, for every committed case — including the
/// whylog, lint and invocation-error shapes the old second world-derivation could not reach at all
/// (`_loom-final-map` §2c). A floor, never a count: the corpus drifts.
#[test]
fn vars_answers_for_every_committed_case() {
    let corpus = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests");
    let consumer = DorcConsumer::new();
    let mut answered = 0usize;
    for entry in std::fs::read_dir(&corpus).expect("the corpus dir is readable") {
        let path = entry.expect("a corpus entry").path();
        if path.extension().is_none_or(|kind| kind != "loom") {
            continue;
        }
        let case = Case::parse(&std::fs::read_to_string(&path).expect("case is readable"))
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        let baseline = consumer
            .editable_baseline(&case)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        assert!(
            !baseline.render().components().is_empty(),
            "{}: an inventory over an empty render says nothing",
            path.display()
        );
        let _ = baseline.used_variables();
        let _ = baseline.all_variables();
        answered = answered.saturating_add(1);
    }
    assert!(
        answered > 50,
        "the corpus discovery floor: {answered} cases"
    );
}

/// The four whylog cases specifically: their render is the STAGED one, which the retired
/// second derivation had no arm for.
#[test]
fn vars_answers_the_whylog_cases() {
    for text in [
        include_str!("../../aid/tests/whylog-absent.loom"),
        include_str!("../../aid/tests/whylog-corrupt.loom"),
        include_str!("../../aid/tests/whylog-version-refused.loom"),
        include_str!("../../aid/tests/whylog-book-desync.loom"),
    ] {
        let case = Case::parse(text).expect("case parses");
        let baseline = DorcConsumer::new()
            .editable_baseline(&case)
            .expect("a whylog case has an inventory");
        assert!(
            baseline.render().text().starts_with("whylog: "),
            "the inventory reads the STAGED render: {:?}",
            baseline.render().text()
        );
    }
}

/// The container has no escape for a body line that reads back as a section header, so the render
/// seat refuses one by name rather than writing a case that re-parses into a different case
/// (`28L:residue-a-wrapped-line-can-look-like-a-txtar-header`).
#[test]
fn a_rendered_section_header_lookalike_refuses_by_name() {
    let case = Case::parse(include_str!("../../aid/tests/cli-help-page.loom")).expect("parses");
    let mut consumer = DorcConsumer::new();
    consumer.set_arrangement_words(
        "cli-help-page",
        dorc_aid::arrangement::OwnedWords::Authored(vec![String::from("-- book.sh --\n")]),
    );
    let error = consumer
        .render_case(&case)
        .expect_err("a header lookalike cannot be written");
    assert!(error.contains("-- book.sh --"), "{error}");
    assert!(
        error.contains("does not both begin `-- ` and end ` --`"),
        "the refusal names the repair: {error}"
    );
}

/// The four things an author does to a value, on ONE committed case (`282` §13): move it by
/// rephrasing around it, duplicate it, drop it, and introduce one the message did not use.
#[test]
fn variable_insert_move_delete_duplicate() {
    let case =
        Case::parse(include_str!("../../aid/tests/cmdsub-operand-top.loom")).expect("parses");
    let consumer = DorcConsumer::new();
    let replay = dorc_loom::replay_case(&case, &consumer, &RunEnv::new(), |command, _| {
        panic!("the in-process driver must claim {command:?}")
    })
    .expect("case replays")
    .swap_remove(0);
    let transcript = replay.output().to_owned();
    let baseline = consumer
        .baseline_from_render(
            &case,
            replay.editable_render().cloned().expect("editable render"),
        )
        .expect("editable baseline");

    let compiled_from = |edited: &str| {
        assert_ne!(edited, transcript, "the probe must actually edit");
        compile_section_edit(&baseline, edited)
            .unwrap_or_else(|error| panic!("{error:?}"))
            .compiled()
            .clone()
    };
    let compiled = |from: &str, to: &str| compiled_from(&transcript.replace(from, to));

    let moved = compiled_from(
        &transcript
            .replace("operand 3 is a", "the operand is a")
            .replace("to check.", "to check ({{position}})."),
    );
    assert!(
        moved.text().contains("to check (operand 3)."),
        "a marker relocates a value within its own section"
    );

    let duplicated = compiled("operand 3 is a", "{{position}} and {{position}} are a");
    assert!(duplicated.text().contains("operand 3 and operand 3 are a"));

    let dropped = compiled("operand 3 is a", "the operand is a");
    assert!(
        !dropped
            .used()
            .contains(&dorc_loom::TemplateVariableName(String::from("position"))),
        "omitting the value drops it from the used set"
    );

    let inserted = compiled("on the host", "on the host via {{command}}");
    assert!(
        inserted.text().contains("on the host via apt-get"),
        "a value the message did not use is reachable from the payload inventory"
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
