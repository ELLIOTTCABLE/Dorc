//! Dorc-specific case rendering, editable baselines, compiled application, and fixpoint coverage.

#![expect(
    clippy::expect_used,
    reason = "committed-case helpers over the known-good test tree; the no-panic lints guard untrusted input"
)]

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use dorc_aid::diag::render_staged_cli_parts;
use dorc_aid::prose::{Mint, ProseTier};
use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, TemplateVariableName, compile_section_edit, replay_case,
    replay_case_with_inputs,
};
use errorloom::{
    BlessError, Case, CaseFile, CaseRenderer, FakeGit, ReplayInput, ReplayResult, RunEnv, RunError,
    fixpoint_check, structure_bless,
};

const CASE_PATH: &str = "cases/dangling-reference.loom";
const CATALOG_PATH: &str = "crates/aid/src/catalog_lock.rs";
const CODE_PATH: &str = "crates/aid/src/diag.rs";

fn message_of(consumer: &DorcConsumer, slug: &str) -> String {
    consumer
        .mirror()
        .iter()
        .find(|e| e.slug == slug)
        .and_then(|e| e.message.as_ref())
        .map(|tier| tier.text().clone())
        .expect("mirror has the code's message")
}

fn whylog_absent_case() -> Case {
    Case::parse(include_str!("../../aid/tests/whylog-absent.loom")).expect("case parses")
}

#[test]
fn world_as_pipeline_marker_pilot_fires_the_real_gate() {
    // The one real-fired proof (`28A` §2n): a wrong-version marked oracle drives the REAL in-process
    // marker gate, so the render is SPANNED (a caret frame into the materialized source), not the
    // spanless world-as-payload path — and it is what the binary actually produces.
    let case_text = "---\ncode: marker-version-unrecognized\n---\n\
                     -- oracle.sh --\n# dorc-lang/v0.1\n\
                     apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }\n\
                     -- replay --\n$ dorc plan --book=oracle.sh\n";
    let case = Case::parse(case_text).expect("case parses");
    let rendered = DorcConsumer::new()
        .render_case(&case)
        .expect("pipeline render");
    // The SLUG, not the words: prose burn-down must not break a test about the gate firing.
    assert!(
        rendered.contains("error[marker-version-unrecognized]: "),
        "the real gate fires: {rendered}"
    );
    assert!(
        rendered.contains("oracle.sh:2:"),
        "a spanned caret frame from the real gate (not the spanless payload path): {rendered}"
    );
}

#[test]
fn host_evidence_admission_refusal_case_renders_the_unwritten_placeholder() {
    let case = Case::parse(include_str!(
        "../../aid/tests/host-evidence-admission-refused.loom"
    ))
    .expect("case parses");
    let rendered = DorcConsumer::new()
        .render_case(&case)
        .expect("canonical payload renders");
    assert_eq!(
        rendered,
        include_str!("../../aid/tests/host-evidence-admission-refused.loom")
    );
}

#[test]
fn editable_baseline_renders_a_defining_case_with_help() {
    let case =
        Case::parse(include_str!("../../aid/tests/whylog-book-desync.loom")).expect("case parses");
    let consumer = DorcConsumer::new();
    let replay = replay_case(&case, &consumer, &RunEnv::new(), |_command, _context| {
        panic!("exact whylog replay must not fall back")
    })
    .expect("exact replay")
    .into_iter()
    .rev()
    .find(|result| result.editable_render().is_some())
    .expect("an editable replay");
    let baseline = consumer
        .baseline_from_render(
            &case,
            replay
                .editable_render()
                .cloned()
                .expect("editable provenance"),
        )
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
    // The `{which}` hole carries the case's own diverged input. Since the durable grammars merged,
    // the case renders the ORACLE arm — a book-drifted durable that ADMITS takes the degraded-receipt
    // route instead (`28F:rul-drift-replay-d1`), so the book arm is unreachable from `--last`.
    assert!(baseline.variables().values().any(|variables| {
        variables.get(&TemplateVariableName(String::from("which")))
            == Some(&String::from("oracle firewall.oracle.sh"))
    }));
}

#[test]
fn whylog_cases_use_exact_fixture_bytes_and_production_provenance() {
    for text in [
        include_str!("../../aid/tests/whylog-absent.loom"),
        include_str!("../../aid/tests/whylog-corrupt.loom"),
        include_str!("../../aid/tests/whylog-version-refused.loom"),
        include_str!("../../aid/tests/whylog-book-desync.loom"),
    ] {
        let case = Case::parse(text).expect("case parses");
        let consumer = DorcConsumer::new();
        // The FIRST block: a case is free to carry further ones after its whylog render.
        let replay = replay_case(&case, &consumer, &RunEnv::new(), |_command, _context| {
            panic!("exact whylog replay must not fall back")
        })
        .expect("case replays")
        .into_iter()
        .next()
        .expect("a replay");
        let raw = case
            .sections()
            .iter()
            .find(|section| section.name() == ".whylog")
            .map(errorloom::Section::content);
        let book = case
            .sections()
            .iter()
            .find(|section| section.name() == "book.sh")
            .map(errorloom::Section::content);
        let diag = dorc_plan::whylog::inspect(raw, ".whylog", book, |path| {
            case.sections()
                .iter()
                .find(|section| section.name() == path)
                .map(|section| section.content().to_owned())
        })
        .into_iter()
        .next()
        .expect("fixture produces one whylog diagnostic");
        let interner = dorc_core::Interner::default();
        assert_eq!(
            replay.output(),
            render_staged_cli_parts(
                "whylog",
                &dorc_aid::RenderCtx::production(),
                &diag,
                "",
                "",
                &interner,
            )
            .text()
        );
        assert_eq!(
            replay
                .editable_render()
                .map(errorloom::EditableRender::text),
            Some(replay.output().to_owned())
        );
        let prefix = replay
            .editable_render()
            .and_then(|render| render.components().first());
        assert!(
            matches!(prefix, Some(errorloom::RenderComponent::Structure(text)) if text == "whylog: ")
        );
        let baseline = consumer
            .baseline_from_render(
                &case,
                replay
                    .editable_render()
                    .cloned()
                    .expect("editable provenance"),
            )
            .expect("editable baseline");
        let rejected_prefix_edit = replay.output().replacen("whylog: ", "rewrite: ", 1);
        assert!(
            compile_section_edit(&baseline, &rejected_prefix_edit).is_err(),
            "the source-stage prefix is immutable structure, not editable prose"
        );
        assert_eq!(
            consumer.render_case(&case).expect("case regenerates"),
            case.to_text()
        );
    }
}

#[test]
fn whylog_driver_claims_only_the_exact_single_file_shape() {
    let case = Case::parse(
        "---\ncode: whylog-absent\n---\n-- replay --\n\
         $ dorc why --last --whylog=.whylog\nold\n\
         $ dorc why --last --whylog=.whylog --whylog=.other\nold\n\
         $ dorc why --last --whylog=.whylog --unknown\nold\n\
         $ dorc why --last --whylog=../whylog\nold\n\
         ",
    )
    .expect("case parses");
    let calls = RefCell::new(Vec::new());
    let results = replay_case(
        &case,
        &DorcConsumer::new(),
        &RunEnv::new(),
        |command, _context| {
            calls.borrow_mut().push(command.to_owned());
            Ok(ReplayResult::bytes(format!("fallback: {command}\n")))
        },
    )
    .expect("replays route");
    assert!(results[0].editable_render().is_some());
    assert!(results[1].editable_render().is_none());
    assert!(results[1].output().starts_with("fallback:"));
    assert!(results[2].editable_render().is_some());
    assert!(results[2].output().starts_with("dorc: error["));
    assert!(results[3].editable_render().is_none());
    assert!(results[3].output().starts_with("fallback:"));
    assert_eq!(
        calls.into_inner(),
        [
            "dorc why --last --whylog=.whylog --whylog=.other",
            "dorc why --last --whylog=../whylog",
        ]
    );

    let pipeline = Case::parse(
        "---\ncode: whylog-absent\n---\n-- replay --\n\
         $ dorc why --last --whylog=.whylog | wombat\nold\n",
    )
    .expect("case parses");
    assert!(matches!(
        replay_case(
            &pipeline,
            &DorcConsumer::new(),
            &RunEnv::new(),
            |_command, _context| Ok(ReplayResult::bytes(String::new()))
        ),
        Err(RunError::UnsupportedReplayGrammar { .. })
    ));
}

/// Two lint shapes are claimed and nothing else is: `dorc lint P` (external tools enabled, the
/// injected runner answering every one absent) and `dorc lint P --no-tools`. A flag order, a
/// repeat, or a path outside the case is somebody else's command.
#[test]
fn lint_driver_claims_exactly_two_shapes() {
    let case = Case::parse(
        "---\ncode: marker-version-unrecognized\n---\n\
         -- oracle.sh --\n# dorc-lang/v0.1\n\
         foo__predict() { pkg : sm.dorc.Package = \"$1\"; }\n\
         -- replay --\n\
         $ dorc lint oracle.sh --no-tools\nold\n\
         $ dorc lint oracle.sh\nold\n\
         $ dorc lint --no-tools oracle.sh\nold\n\
         $ dorc lint oracle.sh --no-tools --no-tools\nold\n\
         $ dorc lint ../oracle.sh --no-tools\nold\n",
    )
    .expect("case parses");
    let calls = RefCell::new(Vec::new());
    let results = replay_case(
        &case,
        &DorcConsumer::new(),
        &RunEnv::new(),
        |command, _context| {
            calls.borrow_mut().push(command.to_owned());
            Ok(ReplayResult::bytes(format!("fallback: {command}\n")))
        },
    )
    .expect("replays route");
    assert!(results[0].editable_render().is_some());
    assert!(results[1].editable_render().is_some());
    // Tools enabled reports both configured linters absent; the airgapped spelling reports neither.
    assert!(results[1].output().contains("lint-tool-absent"));
    assert!(!results[0].output().contains("lint-tool-absent"));
    for result in &results[2..] {
        assert!(result.editable_render().is_none());
        assert!(result.output().starts_with("fallback:"));
    }
    assert_eq!(
        calls.into_inner(),
        [
            "dorc lint --no-tools oracle.sh",
            "dorc lint oracle.sh --no-tools --no-tools",
            "dorc lint ../oracle.sh --no-tools",
        ]
    );
}

/// The tier an edit MINTS and the demotion it records — stated at the mirror, where the mark is
/// actually made and everything downstream (notice, refusal, census) reads it from.
#[test]
fn an_edit_mints_its_tier_and_names_what_it_re_marked() {
    const BEFORE: &str = "a sentence somebody typed";
    const AFTER: &str = "a sentence somebody reworked";
    let case = whylog_absent_case();
    let overtyped = |mint: Mint, start: ProseTier<String>| {
        let mut consumer = DorcConsumer::new().minting(mint);
        consumer.set_message("whylog-absent", Some(start));
        let baseline = consumer
            .editable_baseline(&case)
            .expect("editable baseline");
        let edited = baseline.render().text().replace(BEFORE, AFTER);
        let edit = compile_section_edit(&baseline, &edited).expect("the overtype compiles");
        consumer
            .apply_section_edit(&edit)
            .expect("the mirror takes");
        let tier = consumer
            .mirror()
            .iter()
            .find(|entry| entry.slug == "whylog-absent")
            .and_then(|entry| entry.message.clone())
            .expect("the mirror carries the register");
        (tier, consumer.demoted().to_vec())
    };
    let human = || ProseTier::WrittenByHumanOnly(BEFORE.to_owned());

    let (tier, demoted) = overtyped(Mint::Slop, human());
    assert_eq!(tier, ProseTier::Slop(AFTER.to_owned()));
    assert_eq!(demoted, ["whylog-absent"]);

    let (tier, demoted) = overtyped(Mint::Human, human());
    assert_eq!(tier, ProseTier::WrittenByHumanOnly(AFTER.to_owned()));
    assert!(demoted.is_empty(), "`--human` re-marks nothing");

    let (tier, demoted) = overtyped(Mint::Slop, ProseTier::Migrated(BEFORE.to_owned()));
    assert_eq!(tier, ProseTier::Slop(AFTER.to_owned()));
    assert!(
        demoted.is_empty(),
        "only a HUMAN register is worth naming — migrated and slop words are re-marked in silence"
    );
}

#[test]
fn fixpoint_gate_catches_a_catalog_hand_edit() {
    let case = whylog_absent_case();
    let committed = DorcConsumer::new()
        .render_case(&case)
        .expect("case renders");
    let mut consumer = DorcConsumer::new();
    consumer.set_message(
        "whylog-absent",
        Some(ProseTier::Migrated("sm tampered message".to_owned())),
    );
    let corpus = vec![CaseFile::new(CASE_PATH, committed)];
    let err = fixpoint_check(&consumer, &corpus).unwrap_err();
    assert!(matches!(err, BlessError::Fixpoint(_)), "got {err:?}");
}

#[test]
fn structure_bless_regenerates_a_dorc_case() {
    let case = whylog_absent_case();
    let committed = DorcConsumer::new()
        .render_case(&case)
        .expect("case renders");
    let corpus = vec![CaseFile::new(CASE_PATH, committed.clone())];
    let git = FakeGit::new().mark_dirty(CODE_PATH);
    let result = structure_bless(&DorcConsumer::new(), &git, &corpus, CATALOG_PATH.as_ref())
        .expect("structure bless succeeds");
    assert_eq!(
        result.regenerated().get(Path::new(CASE_PATH)),
        Some(&committed)
    );
}

/// The message register's rendered bytes, as the render laid them out.
fn message_section_text(baseline: &dorc_loom::DorcEditableBaseline) -> String {
    baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            errorloom::RenderComponent::EditableSection(section)
                if section.id().field == "message" =>
            {
                Some(
                    section
                        .fragments()
                        .iter()
                        .map(|fragment| match fragment {
                            errorloom::EditableFragment::Text(text)
                            | errorloom::EditableFragment::Variable { rendered: text, .. } => {
                                text.clone()
                            }
                        })
                        .collect::<String>(),
                )
            }
            _ => None,
        })
        .expect("message section")
}

#[test]
fn applied_template_regenerates_complete_multi_replay_case() {
    let text = "---\ncode: dangling-reference\nwhen-fires: preserved frontmatter\n---\n\
                -- input.txt --\nsource bytes stay unchanged\n\
                -- replay --\n\
                $ dorc plan --book=input.txt\nstale human bytes\n\
                $ dorc plan --book=input.txt > /dev/null\nstale second render\n";
    let case = Case::parse(text).expect("case parses");
    let mut consumer = DorcConsumer::new();
    let baseline = consumer
        .editable_baseline(&case)
        .expect("editable baseline");
    // Taken from the render rather than spelled here: the seat owns the wrap and the words are
    // loom-editable, so a literal copy would go stale the first time either moved.
    let original = message_section_text(&baseline);
    assert!(
        original.contains("sm.dorc.Package:nginx"),
        "the message section carries the payload's coordinate: {original:?}"
    );
    let dirty = baseline.render().text().replace(
        &original,
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
    assert!(regenerated.contains("$ dorc plan --book=input.txt > /dev/null"));
    assert_eq!(
        regenerated
            .matches("sm.dorc.Package:nginx is dangling")
            .count(),
        2
    );
    assert!(!regenerated.contains("stale human bytes"));
    assert!(!regenerated.contains("stale second render"));

    let reparsed = Case::parse(regenerated).expect("regenerated case parses");
    assert_eq!(reparsed.replay().blocks().len(), 2);
    fixpoint_check(&consumer, &[CaseFile::new(CASE_PATH, regenerated.clone())])
        .expect("mutated consumer reproduces regenerated case");
}

#[test]
fn explicit_marker_can_introduce_an_unused_typed_payload_value() {
    let case = Case::parse("---\ncode: cmdsub-operand-top\n---\n-- book.sh --\n#!/bin/sh\napt-get install -y \"$(cat /etc/pkgset)\"\n-- replay --\n$ dorc plan --book=book.sh > /dev/null\n")
        .expect("case parses");
    let mut consumer = DorcConsumer::new();
    let baseline = consumer.editable_baseline(&case).expect("baseline renders");
    let section_text = baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            errorloom::RenderComponent::EditableSection(section)
                if section.id().field == "message" =>
            {
                Some(
                    section
                        .fragments()
                        .iter()
                        .map(|fragment| match fragment {
                            errorloom::EditableFragment::Text(text)
                            | errorloom::EditableFragment::Variable { rendered: text, .. } => {
                                text.clone()
                            }
                        })
                        .collect::<String>(),
                )
            }
            _ => None,
        })
        .expect("message section");
    let dirty = baseline
        .render()
        .text()
        .replacen(&section_text, "run {{command}}", 1);
    let edit = compile_section_edit(&baseline, &dirty).expect("typed marker compiles");
    assert_eq!(edit.compiled().text(), "run apt-get");
    assert_eq!(
        edit.compiled().used(),
        &[TemplateVariableName(String::from("command"))]
    );
    consumer.apply_section_edit(&edit).expect("apply");
    let entry = consumer
        .mirror()
        .iter()
        .find(|entry| entry.slug == "cmdsub-operand-top")
        .expect("entry");
    assert_eq!(
        entry.message.as_ref().map(|tier| tier.text().as_str()),
        Some("run {{command}}")
    );
    assert_eq!(entry.params, ["command"]);
    assert!(
        consumer
            .render_case(&case)
            .expect("re-render")
            .contains("run apt-get")
    );
}

#[test]
fn payload_inventory_excludes_unknown_and_foreign_values() {
    let cmdsub = Case::parse(
        "---\ncode: cmdsub-operand-top\n---\n-- book.sh --\n#!/bin/sh\napt-get install -y \"$(cat /etc/pkgset)\"\n-- replay --\n$ dorc plan --book=book.sh > /dev/null\n",
    )
    .expect("case parses");
    let baseline = DorcConsumer::new()
        .editable_baseline(&cmdsub)
        .expect("baseline renders");
    assert!(baseline.used_variables().contains(&(
        TemplateVariableName(String::from("position")),
        String::from("operand 3")
    )));
    let cause = baseline
        .used_variables()
        .into_iter()
        .find(|(name, _)| name.0 == "cause")
        .map(|(_, value)| value)
        .expect("the rendered diagnostic uses its cause");
    assert!(
        cause.contains("a command-substitution `$(...)`"),
        "{cause:?}"
    );
    assert!(cause.contains("operator-form expansion"), "{cause:?}");
    assert!(
        !baseline
            .used_variables()
            .iter()
            .any(|(name, _)| name.0 == "command")
    );
    assert_eq!(
        baseline
            .all_variables()
            .get(&TemplateVariableName(String::from("command"))),
        Some(&String::from("apt-get"))
    );
    assert!(
        baseline
            .all_variables()
            .contains_key(&TemplateVariableName(String::from("position")))
    );
    let section_text = baseline
        .render()
        .components()
        .iter()
        .find_map(|component| match component {
            errorloom::RenderComponent::EditableSection(section)
                if section.id().field == "message" =>
            {
                Some(
                    section
                        .fragments()
                        .iter()
                        .map(|fragment| match fragment {
                            errorloom::EditableFragment::Text(text)
                            | errorloom::EditableFragment::Variable { rendered: text, .. } => {
                                text.clone()
                            }
                        })
                        .collect::<String>(),
                )
            }
            _ => None,
        })
        .expect("message section");
    let dirty = baseline
        .render()
        .text()
        .replacen(&section_text, "{{unknown}}", 1);
    assert!(matches!(
        compile_section_edit(&baseline, &dirty),
        Err(DorcSectionEditRefusal::UnknownVariable(_))
    ));

    let foreign = Case::parse(
        "---\ncode: site-unresolvable\n---\n-- book.sh --\n#!/bin/sh\nmake install\nldconfig\n-- replay --\n$ dorc plan --book=book.sh > /dev/null\n",
    )
    .expect("case parses");
    let baseline = DorcConsumer::new()
        .editable_baseline(&foreign)
        .expect("foreign baseline");
    // The inventory offers the values a loom author may move, and withholds the two the code
    // relays from the book — foreignness is the VALUE's type now, not the hole's name
    // (`282:rul-passthrough-type-gated`).
    assert!(
        baseline
            .all_variables()
            .contains_key(&TemplateVariableName(String::from("count")))
    );
    assert!(
        baseline
            .all_variables()
            .contains_key(&TemplateVariableName(String::from("site_word")))
    );
    assert!(
        !baseline
            .all_variables()
            .contains_key(&TemplateVariableName(String::from("names")))
    );
    assert!(
        !baseline
            .all_variables()
            .contains_key(&TemplateVariableName(String::from("excerpt")))
    );
}

#[test]
fn exact_replays_keep_editability_with_provenance_and_route_edges_to_the_injected_fallback() {
    let source = "#!/bin/sh\napt-get install -y \"$(cat /etc/webhost/pkgset)\"\n";
    let case = Case::parse(&format!(
        "---\ncode: cmdsub-operand-top\n---\n-- book.sh --\n{source}-- probe.txt --\nprobe bytes\n-- replay --\n$ dorc plan --book=book.sh < probe.txt\nold\n$ dorc --version\nold\n$ dorc lint book.sh\nold\n$ dorc why --last\nold\n$ dorc plan --book=missing.sh\nold\n$ dorc plan --book=book.sh --book=book.sh\nold\n$ dorc plan --book=book.sh --unknown\nold\n$ dorc plan --book=../book.sh\nold\n$ dorc plan --book=./book.sh\nold\n"
    ))
    .expect("case parses");
    let env = RunEnv::new()
        .path_dir("built-tools")
        .path_dir("mocks-only")
        .var("DORC_TEST_MODE", "controlled")
        .shell("configured-shell");
    let calls = RefCell::new(Vec::new());
    let cwd = RefCell::new(None::<PathBuf>);
    let scratch = RefCell::new(None::<PathBuf>);
    let results = replay_case(&case, &DorcConsumer::new(), &env, |command, context| {
        assert_eq!(context.env(), &env);
        let prior_cwd = cwd.replace(Some(context.cwd().to_path_buf()));
        let prior_scratch = scratch.replace(Some(context.scratch().to_path_buf()));
        if let Some(prior) = prior_cwd {
            assert_eq!(prior, context.cwd());
        }
        if let Some(prior) = prior_scratch {
            assert_eq!(prior, context.scratch());
        }
        let marker = context.scratch().join("fallback-order");
        let order = calls.borrow().len();
        if order == 0 {
            std::fs::write(&marker, "first fallback").map_err(RunError::from)?;
        } else {
            assert_eq!(
                std::fs::read_to_string(&marker).ok().as_deref(),
                Some("first fallback")
            );
        }
        calls.borrow_mut().push(command.to_owned());
        Ok(ReplayResult::bytes(format!(
            "note[cmdsub-operand-top]: {{{{command}}}} fallback {order}\n"
        )))
    })
    .expect("replays route");

    assert!(results[0].editable_render().is_some());
    assert_eq!(
        results[0]
            .editable_render()
            .map(errorloom::EditableRender::text),
        Some(results[0].output().to_owned())
    );
    assert!(results[1].editable_render().is_none());
    assert_eq!(results[1].output(), "dorc 0.0.0\n");
    assert!(results[2].editable_render().is_some());
    for declined in [3usize, 4, 7, 8] {
        assert!(results[declined].editable_render().is_none());
        assert!(results[declined].output().contains("{{command}}"));
    }
    for invocation_error in [5usize, 6] {
        assert!(results[invocation_error].editable_render().is_some());
        assert!(
            results[invocation_error]
                .output()
                .starts_with("dorc: error[")
        );
    }
    assert_eq!(
        calls.into_inner(),
        [
            "dorc why --last",
            "dorc plan --book=missing.sh",
            "dorc plan --book=../book.sh",
            "dorc plan --book=./book.sh",
        ]
    );
}

#[test]
fn replay_with_a_fake_fallback_leaves_case_catalog_and_source_bytes_unchanged() {
    let case_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cmdsub-command.loom");
    let catalog_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/src/catalog_lock.rs");
    let case_before = std::fs::read(&case_path).expect("case reads");
    let catalog_before = std::fs::read(&catalog_path).expect("catalog reads");
    let case = Case::parse(std::str::from_utf8(&case_before).expect("case is UTF-8"))
        .expect("case parses");

    let results = replay_case(
        &case,
        &DorcConsumer::new(),
        &RunEnv::new(),
        |_command, _context| Ok(ReplayResult::bytes(String::from("fake bytes\n"))),
    )
    .expect("direct replay needs no fallback");

    assert_eq!(results.len(), 1);
    assert_eq!(std::fs::read(&case_path).ok(), Some(case_before));
    assert_eq!(std::fs::read(&catalog_path).ok(), Some(catalog_before));
}

#[test]
fn vars_replay_reads_only_the_named_materialized_case() {
    let outer = Case::parse(
        "---\ncode: cmdsub-operand-top\n---\n-- book.sh --\n#!/bin/sh\napt-get install -y \"$(cat /etc/pkgset)\"\n-- replay --\n\
         $ dorc plan --book=book.sh > /dev/null\nold\n\
         $ dorc-loom vars --all self.txt\nold\n\
         $ dorc-loom vars --used other.txt\nold\n\
         $ dorc-loom vars --all missing.txt\nold\n\
         $ dorc-loom vars --all ../self.txt\nold\n\
         $ dorc-loom vars --all /self.txt\nold\n\
         $ dorc-loom vars --all self.txt extra\nold\n",
    )
    .expect("outer case parses");
    let other = Case::parse(
        "---\ncode: render-heredoc-refused\n---\n-- replay --\n$ dorc plan --book=book.sh\nold\n",
    )
    .expect("other case parses");
    let inputs = [
        ReplayInput::new("self.txt", outer.to_text()).expect("self input"),
        ReplayInput::new("other.txt", other.to_text()).expect("other input"),
    ];
    let calls = RefCell::new(Vec::new());
    let results = replay_case_with_inputs(
        &outer,
        &DorcConsumer::new(),
        &RunEnv::new(),
        &inputs,
        |command, _context| {
            calls.borrow_mut().push(command.to_owned());
            Ok(ReplayResult::bytes(format!("fallback: {command}\n")))
        },
    )
    .expect("replays route");

    assert!(results[1].output().contains("{{command}} = \"apt-get\""));
    assert!(results[2].output().contains("{{verb}} = \"elide\""));
    assert!(!results[2].output().contains("{{command}} = \"apt-get\""));
    for result in &results[3..] {
        assert!(result.editable_render().is_none());
        assert!(result.output().starts_with("fallback: dorc-loom vars"));
    }
    assert_eq!(
        calls.into_inner(),
        [
            "dorc-loom vars --all missing.txt",
            "dorc-loom vars --all ../self.txt",
            "dorc-loom vars --all /self.txt",
            "dorc-loom vars --all self.txt extra",
        ]
    );

    let quoted = Case::parse(
        "---\ncode: cmdsub-operand-top\n---\n-- replay --\n\
         $ dorc-loom vars --all 'self.txt'\nold\n",
    )
    .expect("quoted case parses");
    assert!(matches!(
        replay_case_with_inputs(
            &quoted,
            &DorcConsumer::new(),
            &RunEnv::new(),
            &inputs,
            |_command, _context| Ok(ReplayResult::bytes(String::new()))
        ),
        Err(RunError::UnsupportedReplayGrammar { .. })
    ));
}

/// A case's inventory OF ITSELF, on both chains, under the `--this` selector.
///
/// A case cannot contain itself, so the block a case carries about its own values names no target
/// at all, and both chains resolve `--this` against the case being rendered. What makes it
/// terminate is not a depth count: the baseline seat — the one place an inventory comes from —
/// declines the block, so the question is asked exactly once.
#[test]
fn a_case_answers_its_own_inventory_on_both_chains() {
    for block in ["--this vars", "--this vars --all", "--this sections"] {
        let case = Case::parse(&format!(
            "---\ncode: render-heredoc-refused\n---\n-- replay --\n\
             $ dorc plan --book=book.sh\nold\n\
             $ dorc-loom {block}\nold\n"
        ))
        .expect("case parses");
        let consumer = DorcConsumer::new();
        let results = replay_case(&case, &consumer, &RunEnv::new(), |command, _context| {
            panic!("the in-process driver must claim {command:?}")
        })
        .expect("replays route");
        let inventory = results[1].output();
        assert!(
            inventory.starts_with("case: render-heredoc-refused\n"),
            "`{block}`: the header names the case's own slug, never a spelling in the command: \
             {inventory}"
        );
        assert!(
            inventory.contains("elide"),
            "`{block}`: the inventory is the payload's, not an empty answer: {inventory}"
        );
        let rendered = consumer.render_case(&case).expect("the case re-renders");
        assert_eq!(
            rendered.matches("case: render-heredoc-refused").count(),
            1,
            "`{block}`: the fixpoint chain answers the same block once: {rendered}"
        );
    }
}

/// The stratification the self-referring block rests on: the EDITABLE-BASELINE seat declines it,
/// every other seat answers it.
///
/// This is a correctness property, not a nicety. The inventory is derived from the render an edit
/// compiles against, and that render comes from driving the case — so a baseline drive that
/// answered the block would ask its own question forever. The `--this` selector changed which
/// SPELLING reaches the gate; it must not have changed the gate.
#[test]
fn the_editable_baseline_seat_declines_a_case_s_own_inventory() {
    let case = Case::parse(
        "---\ncode: render-heredoc-refused\n---\n-- replay --\n\
         $ dorc-loom --this vars\nold\n\
         $ dorc plan --book=book.sh\nold\n",
    )
    .expect("case parses");
    let consumer = DorcConsumer::new();

    // The baseline exists at all only because the block declined: it is the FIRST replay, so a
    // seat that answered it would have to answer it while computing the answer.
    let baseline = consumer
        .editable_baseline(&case)
        .expect("the baseline comes from the second replay");
    assert!(
        baseline
            .used_variables()
            .iter()
            .any(|(name, _)| name.0 == "verb"),
        "the baseline is the plan replay's render, not the declined block's"
    );

    // The ordinary seat, gate Allowed, answers it — through the very baseline that just declined.
    let answered = consumer
        .vars_inventory(&case, dorc_loom::Breadth::Used)
        .expect("the inventory derives");
    assert!(answered.contains("{{verb}} = \"elide\""), "{answered}");
}

/// The two chains a case travels — `DorcConsumer::replay` (the EDIT chain, which `publish`
/// drives) and `CaseRenderer::render_case` (the FIXPOINT chain, which the looms runner
/// drives) — must claim exactly the same invocations.
///
/// This is the guard `28F` bought with a real divergence. Nothing structural relates the two arms:
/// a shape only ONE answers is a case that render-fixpoints green and refuses every edit, or edits
/// cleanly and then fails its own fixpoint. The table IS the relation.
///
/// A claimed shape must ALSO answer with the same bytes on both arms. There is one render form —
/// the committed transcript IS what the seat printed
/// (`28L:rul-editability-is-stamped-never-re-derived`) — so a byte difference here means a
/// transcript and the bytes an edit compiles against have silently parted ways again.
#[test]
fn both_replay_chains_claim_the_same_invocation_shapes() {
    let fixtures = concat!(
        "-- book.sh --\n#!/bin/sh\nprintf '%s\\n' current\n\n",
        "-- oracle.sh --\n# dorc-lang/v0.1\nfoo__predict() { pkg : sm.dorc.Package = \"$1\"; }\n\n",
        "-- .whylog --\n",
        "dorc-whylog/2 nonce=dorc attempt=0 host=web1 target=width-one generation=width-one",
        " mode=whylog-replay started=1784944837000 @@dorc@@\n",
        "book digest=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        " path=book.sh @@dorc@@\n",
        "argv value=dorc @@dorc@@\n",
        "digest decision=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb @@dorc@@\n",
        "apply leaf=0 disposition=run predicted=1 @@dorc@@\n",
        "results bytes=0 @@dorc@@\n",
        "dorc-whylog-end/2 @@dorc@@\n\n",
    );

    // (invocation, both chains claim it)
    let shapes = [
        ("dorc why --last --whylog=.whylog", true),
        ("dorc why book.sh:5 --last --whylog=.whylog", true),
        ("dorc --help", true),
        ("dorc lint oracle.sh --no-tools", true),
        ("dorc why --last --whylog=absent.whylog", true),
        ("dorc why book.sh:5 --last --whylog=absent.whylog", false),
        ("dorc why --last --whylog=../escape", false),
        ("dorc plan --book=book.sh", true),
        ("dorc wombat --hork", true),
    ];

    for (command, claimed) in shapes {
        let arrangement = (command == "dorc --help").then_some("arrangement: cli-help-page\n");
        let case = Case::parse(&format!(
            "---\n{}---\n{fixtures}-- replay --\n$ {command}\nold\n",
            arrangement.unwrap_or_default()
        ))
        .expect("case parses");
        let consumer = DorcConsumer::new();

        let declined = RefCell::new(false);
        let edit_chain = replay_case(&case, &consumer, &RunEnv::new(), |_command, _context| {
            *declined.borrow_mut() = true;
            Ok(ReplayResult::bytes(String::from("fallback\n")))
        })
        .expect("replays route")
        .pop()
        .expect("one replay");
        let edit_claims = !declined.into_inner();
        let fixpoint_chain = consumer.render_case(&case);

        assert_eq!(
            edit_claims,
            fixpoint_chain.is_ok(),
            "`{command}`: one chain claims it and the other does not, so a case on this shape \
             would edit and fixpoint against different answers"
        );
        assert_eq!(edit_claims, claimed, "`{command}`: unexpected claim");
        if !claimed {
            continue;
        }
        let rendered = Case::parse(&fixpoint_chain.expect("claimed"))
            .expect("regenerated case parses")
            .replay()
            .blocks()
            .first()
            .expect("one block")
            .output()
            .to_owned();
        assert_eq!(
            rendered,
            edit_chain.output(),
            "`{command}`: the two chains printed different bytes"
        );
        assert!(
            edit_chain.editable_render().is_some(),
            "`{command}`: a claimed shape carries edit provenance"
        );
    }
}

#[test]
fn source_backed_plan_replays_the_complete_engine_invocation_and_redirects_its_artifact() {
    let case = Case::parse(
        "---\ncode: cmdsub-operand-top\n---\n\
         -- book.sh --\n#!/bin/sh\nhork \"$(wombat)\"\n\n\
         -- replay --\n\
         $ dorc plan --book=book.sh > plan.sh\nold\n\
         $ cat plan.sh\nold\n",
    )
    .expect("case parses");

    let results = replay_case(&case, &DorcConsumer::new(), &RunEnv::new(), |command, _| {
        panic!("the direct driver declined {command:?}")
    })
    .expect("the production engine runs");

    assert!(
        results[0].output().contains("cmdsub-operand-top"),
        "stderr stays in the natural transcript: {}",
        results[0].output()
    );
    assert!(results[0].editable_render().is_some());
    assert!(
        results[1].output().contains("#!/bin/sh"),
        "the redirected stdout artifact is observable through native cat"
    );
    assert!(!results[1].output().contains("cmdsub-operand-top"));
}
