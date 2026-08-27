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

use dorc_loom::{DorcConsumer, DorcEditableBaseline, SectionPreview, compile_section_edit};
use errorloom::{Case, CaseRenderer, RenderComponent, RunEnv};

/// One case, replayed through `consumer`'s own mirror into its editable baseline and transcript.
fn drive(consumer: &DorcConsumer, case: &Case) -> (DorcEditableBaseline, String) {
    let replay = dorc_loom::replay_case(case, consumer, &RunEnv::new(), |command, _| {
        panic!("the in-process driver must claim {command:?}")
    })
    .expect("case replays")
    .into_iter()
    .rev()
    .find(|result| result.editable_render().is_some())
    .expect("an editable replay");
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

/// The primary collection, read at RUN time — see [`a_case_whose_blanked_message_renders`].
fn corpus_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests")
}

/// A committed, in-process-drivable case for a code that has no `help` register yet.
///
/// CHOSEN rather than named: the mechanism under test is the ABSENT register, not any particular
/// code, so the fixture is whichever code still lacks one. Unlike a message register, this pool is
/// not the prose burn-down's to empty — a help register is minted deliberately, one code at a time.
fn a_case_whose_help_register_is_absent(consumer: &DorcConsumer) -> (String, Case) {
    let candidates: Vec<String> = consumer
        .mirror()
        .iter()
        .filter(|entry| matches!(entry.help, dorc_aid::catalog::HelpRegister::Absent))
        .map(|entry| entry.slug.clone())
        .collect();
    for slug in &candidates {
        let path = corpus_dir().join(format!("{slug}.loom"));
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(case) = Case::parse(&text) else {
            continue;
        };
        if drivable_output(consumer, &case).is_some() {
            return (slug.clone(), case);
        }
    }
    panic!(
        "no committed case names a code that still lacks a `help` register and that the in-process \
         driver can reach (candidates: {candidates:?})"
    )
}

/// A committed case whose render — with its own message register BLANKED in the mirror — takes the
/// shape a placeholder mechanic needs, handed back with that blanking already applied.
///
/// SYNTHESIZED, not found. `[unwritten:]` is a legal resting state rather than a pin, and
/// `aid/CLAUDE.md`'s `prose-pins-live-where-the-prose-does` puts the placeholder MECHANIC on a
/// synthesized row for exactly this reason: hunting the corpus for a code nobody has words for yet
/// made these tests hostages of the prose burn-down, and when its last unwritten message register
/// was authored the candidate pool went empty and both of them panicked. Blanking a mirror row
/// keeps the fixture CHOSEN rather than named — no slug is pinned, so writing prose still cannot
/// redden this crate — while the pool it chooses from is every drivable case in the collection.
fn a_case_whose_blanked_message_renders(
    shaped: impl Fn(&str, &str) -> bool,
) -> (String, Case, DorcConsumer) {
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(corpus_dir())
        .expect("the corpus dir is readable")
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.extension().is_some_and(|kind| kind == "loom"))
        .collect();
    // Directory order is not an order, and a fixture that flaps between runs is worse than none.
    paths.sort();
    for path in &paths {
        // `sync-residue-is-never-a-case`: a conflict copy keeps the extension.
        if path
            .to_str()
            .is_some_and(|name| name.contains(".sync-conflict-"))
        {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let Ok(case) = Case::parse(&text) else {
            continue;
        };
        // An `arrangement:` case owns chrome, which has no message register to blank.
        let Some(slug) = case.frontmatter().scalar("code").map(str::to_owned) else {
            continue;
        };
        let mut consumer = DorcConsumer::new();
        consumer.set_message(&slug, None);
        if drivable_output(&consumer, &case).is_some_and(|output| shaped(&slug, &output)) {
            return (slug, case, consumer);
        }
    }
    panic!(
        "no case in the collection renders a blanked message register in the shape this test needs \
         ({} cases tried)",
        paths.len()
    )
}

/// The transcript [`drive`] would hand a test, or `None` where it would panic instead.
///
/// The LAST block, because that is the one `drive` pops: a case closing on a machine format has no
/// editable render there and is not a fixture for anything here.
fn drivable_output(consumer: &DorcConsumer, case: &Case) -> Option<String> {
    dorc_loom::replay_case(case, consumer, &RunEnv::new(), |_, _| {
        Err(errorloom::RunError::ShellNotConfigured)
    })
    .ok()?
    .iter()
    .rev()
    .find(|result| result.editable_render().is_some())
    .map(|result| result.output().to_owned())
}

fn help_of(
    consumer: &DorcConsumer,
    slug: &str,
) -> dorc_aid::catalog::HelpRegister<dorc_aid::prose::ProseTier<String>> {
    consumer
        .mirror()
        .iter()
        .find(|entry| entry.slug == slug)
        .map(|entry| entry.help.clone())
        .expect("the mirror carries the code")
}

/// The CURRENT bytes of one editable section, taken from the render.
///
/// Every fixture below that edits a committed sentence starts here rather than from a literal copy
/// of it. A copy makes the test a second owner of prose the loom flow exists to let someone rewrite,
/// so authoring better words reddens this crate with no pointer to the flow that did it
/// (`render-form-unwelded`). What these tests mean is "take whatever it says now and change it",
/// which is what this expresses.
fn section_text(baseline: &DorcEditableBaseline, owner: &str, field: &str) -> String {
    baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            RenderComponent::EditableSection(section)
                if section.id().owner == owner && section.id().field == field =>
            {
                Some(
                    section
                        .fragments()
                        .iter()
                        .map(|fragment| match fragment {
                            errorloom::EditableFragment::Text(text)
                            | errorloom::EditableFragment::Variable { rendered: text, .. } => {
                                text.as_str()
                            }
                        })
                        .collect::<String>(),
                )
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no `{owner}`/`{field}` section in {:?}", sections(baseline)))
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
    let (slug, case, mut consumer) = a_case_whose_blanked_message_renders(|slug, transcript| {
        transcript.contains(&format!("[unwritten: {slug}]"))
    });
    let (baseline, transcript) = drive(&consumer, &case);
    assert!(
        sections(&baseline).contains(&(slug.clone(), "message")),
        "the placeholder wears the message register's face: {:?}",
        sections(&baseline)
    );

    let words = "the run finished but its why durable did not land";
    let edited = transcript.replace(&format!("[unwritten: {slug}]"), words);
    let edit = compile_section_edit(&baseline, &edited).expect("the overtype compiles");
    assert_eq!(edit.section().field, "message");
    consumer
        .apply_section_edit(&edit)
        .expect("the mirror takes it");
    assert_eq!(
        consumer
            .mirror()
            .iter()
            .find(|entry| entry.slug == slug)
            .and_then(|entry| entry.message.clone()),
        Some(dorc_aid::prose::ProseTier::Slop(String::from(words)))
    );
}

/// Typing over a value's TEXT is legal and destructive, and the two together are why it is
/// DISCLOSED rather than refused: omission is how `282` §13 removes a variable, so refusing it
/// would take the removal mechanism away — but an author who meant to reword has just frozen the
/// world into the register, and the compile view is where they can still see it.
#[test]
fn overtyping_a_value_discloses_the_dropped_variable() {
    let (_, _, baseline, transcript) = driven(include_str!(
        "../../aid/tests/cli-unknown-flag-did-you-mean.loom"
    ));
    assert!(
        transcript.contains("--wat"),
        "the fixture must interpolate its flag value: {transcript:?}"
    );
    let preview = dorc_loom::compile_preview(&baseline, &transcript.replace("--wat", "--wut"))
        .expect("overtyping a value is an ordinary edit");
    let dropped: Vec<String> = preview
        .sections()
        .iter()
        .flat_map(|section| section.dropped().iter().map(|hole| hole.name.0.clone()))
        .collect();
    assert_eq!(dropped, vec![String::from("flag")], "{preview:?}");
    let rendered = dorc_loom::render_publish_diff(&preview);
    assert!(
        rendered.contains("- ") && rendered.contains("{{flag}}"),
        "the diff must show the hole leaving the register: {rendered}"
    );
    // Replacing the value with a DIFFERENT one is not a bake-in: `--wat` is gone from the section,
    // so there is nothing frozen for a warning to point at.
    assert!(
        preview
            .sections()
            .iter()
            .flat_map(SectionPreview::dropped)
            .all(|hole| !hole.value_reappears_as_text),
        "{preview:?}"
    );
}

/// The bake-in the human asked to be warned about (`30C` item 2), and its exact evidence: the
/// variable is gone AND its rendered bytes are still sitting in the section as text.
///
/// Stripping the backticks around `--wat` destroys the anchors the transport aligns a variable by,
/// so the only interpretation left removes the occurrence — while the value the author retyped
/// stays put. That is the shape a plain reword can reach by accident, which is why it is disclosed.
#[test]
fn a_value_retyped_where_its_variable_stood_is_flagged_as_baked_in() {
    let (_, _, baseline, transcript) = driven(include_str!(
        "../../aid/tests/cli-unknown-flag-did-you-mean.loom"
    ));
    let edited = transcript.replace("`--wat`", "--wat");
    assert_ne!(edited, transcript, "the fixture must carry the anchors");
    let preview = dorc_loom::compile_preview(&baseline, &edited).expect("the removal interprets");
    let baked: Vec<String> = preview
        .sections()
        .iter()
        .flat_map(SectionPreview::dropped)
        .filter(|hole| hole.value_reappears_as_text)
        .map(|hole| hole.name.0.clone())
        .collect();
    assert_eq!(baked, vec![String::from("flag")], "{preview:?}");
}

/// One component, one home, per EDIT — not only per declaration. Eleven invocation-error cases
/// render the usage synopsis; an edit through any of the ten that are not its home would rewrite an
/// entry whose own case still shows the old words, and nothing would say so until somebody read two
/// transcripts side by side.
#[test]
fn editing_a_component_another_case_owns_refuses_by_name() {
    let borrowing = corpus_dir().join("cli-unknown-flag.loom");
    let (_, _, baseline, transcript) =
        driven(&std::fs::read_to_string(&borrowing).expect("read the borrowing case"));
    assert!(
        transcript.contains("usage: dorc "),
        "the fixture must render the synopsis it does not own: {transcript:?}"
    );
    let preview = dorc_loom::compile_preview(
        &baseline,
        &transcript.replace("usage: dorc ", "invocation: dorc "),
    )
    .expect("the edit itself compiles");

    let ownership = dorc_loom::corpus_ownership(&corpus_dir()).expect("the corpus resolves");
    let refusal = dorc_loom::refuse_foreign_components(&ownership, &borrowing, &preview)
        .expect_err("a foreign component refuses");
    let explained = refusal.explain(&borrowing);
    assert!(
        explained.contains("cli-usage-synopsis") && explained.contains("cli-no-book-given.loom"),
        "the refusal names the component AND its home: {explained}"
    );
    assert!(
        explained.contains("dorc-loom publish"),
        "every refusal ends in its next command: {explained}"
    );
}

/// The mint path stays open: a component nobody owns is not foreign, or a freshly scaffolded case
/// could never author its own first words.
#[test]
fn an_unowned_component_is_not_foreign() {
    let case = corpus_dir().join("cli-no-book-given.loom");
    let (_, _, baseline, transcript) =
        driven(&std::fs::read_to_string(&case).expect("read the owning case"));
    let preview = dorc_loom::compile_preview(
        &baseline,
        &transcript.replace("usage: dorc ", "invocation: dorc "),
    )
    .expect("the edit compiles");
    let ownership = dorc_loom::corpus_ownership(&corpus_dir()).expect("the corpus resolves");
    dorc_loom::refuse_foreign_components(&ownership, &case, &preview)
        .expect("its own home may edit it");
}

/// The affordance the refusal names: mint the register, and the ORDINARY loop fills it — the
/// placeholder the render then grows is an edit region like any other.
#[test]
fn help_register_edit_round_trips() {
    let mut consumer = DorcConsumer::new();
    let (slug, case) = a_case_whose_help_register_is_absent(&consumer);
    consumer
        .seed_help_register(&slug)
        .expect("the register is absent");

    let (baseline, transcript) = drive(&consumer, &case);
    let placeholder = format!("[unwritten: {slug}.help]");
    assert!(
        transcript.contains(&format!("= help:  {placeholder}")),
        "the seeded register renders its own placeholder: {transcript:?}"
    );
    let words = "give a path, or --book=PATH";
    let edit = compile_section_edit(&baseline, &transcript.replace(&placeholder, words))
        .expect("the placeholder is editable");
    assert_eq!(edit.section().field, "help");
    consumer
        .apply_section_edit(&edit)
        .expect("the mirror takes it");
    assert_eq!(
        help_of(&consumer, &slug),
        dorc_aid::catalog::HelpRegister::Written(dorc_aid::prose::ProseTier::Slop(String::from(
            words
        )))
    );
}

/// Seeding twice is a mistake worth naming rather than a no-op that quietly loses an edit.
#[test]
fn seeding_an_existing_register_refuses() {
    let mut consumer = DorcConsumer::new();
    let (slug, _) = a_case_whose_help_register_is_absent(&consumer);
    consumer.seed_help_register(&slug).expect("absent");
    assert_eq!(
        consumer.seed_help_register(&slug),
        Err(dorc_loom::SeedRefusal::AlreadyPresent(slug.clone()))
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
    let original = section_text(&baseline, "cli-flag-requires-mode", "message");
    let flag = baseline
        .all_variables()
        .get(&dorc_loom::TemplateVariableName(String::from("flag")))
        .expect("the payload carries a flag")
        .clone();
    // The glued spelling is the thing under test; the words either side of it are the register's.
    let edited = transcript.replace(&original, &original.replacen(&flag, "`{{flag}}`", 1));
    let edit = compile_section_edit(&baseline, &edited).expect("a glued marker compiles");
    assert_eq!(
        edit.compiled().text(),
        original.replacen(&flag, &format!("`{flag}`"), 1),
        "the marker renders back to its own value, backticks and all"
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

/// `28L:rul-empty-registers-for-pure-holes`, end to end: a code whose whole message is its
/// `{{reason}}` hole renders the COMPONENT's face, so the sentence in the transcript is editable
/// prose and the edit lands on the component's own registry entry.
///
/// Before the component carried its identity to the render seat, that register rendered as ONE
/// variable fragment and the same keystrokes were an attempt to rewrite a value —
/// `EditableVariableTouched`, the refusal for lying about the world. Eighty-odd variant sentences
/// were rendered-but-uneditable that way, which is the gap this closes.
#[test]
fn a_lone_reason_hole_edits_at_the_components_own_entry() {
    let case = Case::parse(include_str!(
        "../../aid/tests/predict-out-of-dialect-shift-count.loom"
    ))
    .expect("parses");
    let mut consumer = DorcConsumer::new();
    let (baseline, transcript) = drive(&consumer, &case);
    assert!(
        sections(&baseline).contains(&(
            String::from("predict-out-of-dialect-shift-count"),
            dorc_loom::ARRANGEMENT_LINE_FIELD
        )),
        "the section wears the COMPONENT's face, not the code's register: {:?}",
        sections(&baseline)
    );

    // Whatever the component says now, said differently.
    let original = section_text(
        &baseline,
        "predict-out-of-dialect-shift-count",
        dorc_loom::ARRANGEMENT_LINE_FIELD,
    );
    let rewritten = format!("{original}, rephrased");
    let edited = transcript.replace(&original, &rewritten);
    assert_ne!(edited, transcript, "the fixture must edit the sentence");
    let preview =
        dorc_loom::compile_preview(&baseline, &edited).expect("the component's words compile");
    consumer
        .apply_preview(&preview)
        .expect("the registry mirror takes it");

    let stored = consumer
        .arrangements()
        .iter()
        .find(|entry| entry.slug == "predict-out-of-dialect-shift-count")
        .and_then(|entry| entry.words.as_ref())
        .map(|tier| tier.text().clone())
        .expect("the component has an entry");
    assert_eq!(stored.len(), 1, "a pure-hole face stores one word run");
    assert!(
        stored[0].ends_with(", rephrased"),
        "the COMPONENT's entry is what moved: {stored:?}"
    );
    assert_eq!(
        consumer
            .mirror()
            .iter()
            .find(|entry| entry.slug == "predict-out-of-dialect")
            .and_then(|entry| entry.message.as_ref())
            .map(|tier| tier.text().as_str()),
        Some("{{reason}}"),
        "the register that is nothing but a hole stays nothing but a hole"
    );
    assert!(
        consumer
            .render_case(&case)
            .expect("the case re-renders")
            .contains("rephrased"),
        "the one-step loop holds for a component face too"
    );
}

/// The exclusion the same ruling names: a register with words of ITS OWN around the hole keeps the
/// component as an ordinary VALUE.
///
/// Facing an interior hole would split its register into sections fenced by the component — the
/// priced-and-declined remedy — so the section here is the code's, and the component's own words
/// are edited through the case that homes them.
#[test]
fn a_reason_inside_a_sentence_stays_a_value() {
    let case = Case::parse(include_str!("../../aid/tests/whylog-corrupt.loom")).expect("parses");
    let consumer = DorcConsumer::new();
    let (baseline, _) = drive(&consumer, &case);
    let sections = sections(&baseline);
    assert!(
        sections.contains(&(String::from("whylog-corrupt"), "message")),
        "the code's own register owns the section: {sections:?}"
    );
    assert!(
        !sections
            .iter()
            .any(|(owner, _)| owner.starts_with("whylog-corrupt-")),
        "no component fences the register into pieces: {sections:?}"
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
                && entry.words.map(|tier| *tier.text()) == Some(&["oracles: ", ""][..])),
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
        if path.extension().is_none_or(|kind| kind != "loom")
            || path
                .file_name()
                .is_some_and(|name| name.to_string_lossy().contains(".sync-conflict-"))
        {
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
    let original = section_text(&baseline, "cmdsub-operand-top", "message");
    let value = |name: &str| {
        baseline
            .all_variables()
            .get(&dorc_loom::TemplateVariableName(String::from(name)))
            .unwrap_or_else(|| panic!("the payload carries `{name}`"))
            .clone()
    };
    let (position, command) = (value("position"), value("command"));
    let rewritten = |section: &str| compiled_from(&transcript.replace(&original, section));

    let moved = rewritten(&format!(
        "{} ({{{{position}}}})",
        original.replacen(&position, "the operand", 1)
    ));
    assert!(
        moved.text().ends_with(&format!("({position})")),
        "a marker relocates a value within its own section: {}",
        moved.text()
    );

    let duplicated = rewritten(&original.replacen(&position, "{{position}} and {{position}}", 1));
    assert!(
        duplicated
            .text()
            .contains(&format!("{position} and {position}"))
    );

    let dropped = rewritten(&original.replacen(&position, "the operand", 1));
    assert!(
        !dropped
            .used()
            .contains(&dorc_loom::TemplateVariableName(String::from("position"))),
        "omitting the value drops it from the used set"
    );

    let inserted = rewritten(&format!("{original} via {{{{command}}}}"));
    assert!(
        inserted.text().ends_with(&format!("via {command}")),
        "a value the message did not use is reachable from the payload inventory: {}",
        inserted.text()
    );
}

/// A placeholder long enough to WRAP is still one section: the break weft minted inside it is the
/// register's own space wearing the renderer's clothes, and a second section here would leave half
/// the placeholder unaddressable.
///
/// The fixture is CHOSEN, like every other blanked-register one here: naming a slug would pin where
/// that code's placeholder happens to break, so a rename or a width change would redden this crate
/// from a distance.
#[test]
fn a_wrapped_placeholder_is_one_section() {
    let (slug, case, consumer) =
        a_case_whose_blanked_message_renders(|_, transcript| transcript.contains("[unwritten:\n"));
    let (baseline, transcript) = drive(&consumer, &case);
    // Indent-agnostic: the break carries whatever continuation indent the seat lays out, and which
    // seat the chosen case renders through is not this test's business.
    assert!(
        transcript
            .split_once("[unwritten:\n")
            .is_some_and(|(_, tail)| tail.trim_start().starts_with(&format!("{slug}]"))),
        "the chosen fixture wraps inside its placeholder: {transcript:?}"
    );
    let sections = sections(&baseline);
    assert_eq!(
        sections
            .iter()
            .filter(|(owner, field)| owner == &slug && *field == "message")
            .count(),
        1,
        "the wrapped placeholder is ONE section, not one per laid-out line: {sections:?}"
    );
}
