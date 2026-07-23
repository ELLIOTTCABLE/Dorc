//! Dorc-specific case rendering, editable baselines, compiled application, and fixpoint coverage.

#![expect(
    clippy::expect_used,
    reason = "committed-case helpers over the known-good test tree; the no-panic lints guard untrusted input"
)]

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use dorc_core::diag::render_staged_cli_parts;
use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, TemplateVariableName, compile_section_edit, replay_case,
    replay_case_with_inputs,
};
use errorloom::{
    BlessError, Case, CaseFile, CaseRenderer, FakeGit, ReplayInput, ReplayResult, RunEnv, RunError,
    fixpoint_check, structure_bless,
};

const CASE_PATH: &str = "cases/dangling-reference.txt";
const CATALOG_PATH: &str = "crates/core/src/catalog_lock.rs";
const CODE_PATH: &str = "crates/core/src/diag.rs";

fn message_of(consumer: &DorcConsumer, slug: &str) -> String {
    consumer
        .mirror()
        .iter()
        .find(|e| e.slug == slug)
        .and_then(|e| e.message.clone())
        .expect("mirror has the code's message")
}

fn whylog_absent_case() -> Case {
    Case::parse(include_str!("../cases/whylog-absent.txt")).expect("case parses")
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
fn host_evidence_admission_refusal_case_renders_the_unwritten_placeholder() {
    let case = Case::parse(include_str!("../cases/host-evidence-admission-refused.txt"))
        .expect("case parses");
    let rendered = DorcConsumer::new()
        .render_case(&case)
        .expect("canonical payload renders");
    assert_eq!(
        rendered,
        include_str!("../cases/host-evidence-admission-refused.txt")
    );
}

#[test]
fn editable_baseline_renders_a_defining_case_with_help() {
    let case = Case::parse(include_str!("../cases/whylog-book-desync.txt")).expect("case parses");
    let consumer = DorcConsumer::new();
    let replay = replay_case(&case, &consumer, &RunEnv::new(), |_command, _context| {
        panic!("exact whylog replay must not fall back")
    })
    .expect("exact replay")
    .pop()
    .expect("one replay");
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
    assert!(baseline.variables().values().any(|variables| {
        variables.get(&TemplateVariableName(String::from("which"))) == Some(&String::from("book"))
    }));
}

#[test]
fn whylog_cases_use_exact_fixture_bytes_and_production_provenance() {
    for text in [
        include_str!("../cases/whylog-absent.txt"),
        include_str!("../cases/whylog-corrupt.txt"),
        include_str!("../cases/whylog-version-refused.txt"),
        include_str!("../cases/whylog-book-desync.txt"),
    ] {
        let case = Case::parse(text).expect("case parses");
        let consumer = DorcConsumer::new();
        let replay = replay_case(&case, &consumer, &RunEnv::new(), |_command, _context| {
            panic!("exact whylog replay must not fall back")
        })
        .expect("case replays")
        .pop()
        .expect("one replay");
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
        let diag = dorc_plan::whylog::inspect(
            raw,
            ".whylog",
            book.map(|book| dorc_plan::whylog::WhylogCurrent {
                book: Some(book),
                oracles: &[],
            }),
        )
        .diagnostics
        .into_iter()
        .next()
        .expect("fixture produces one whylog diagnostic");
        let interner = dorc_core::Interner::default();
        assert_eq!(
            replay.output(),
            render_staged_cli_parts(
                "whylog",
                &dorc_core::catalog::CONST_CATALOG,
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
         $ dorc why --last --whylog=.whylog | wombat\nold\n",
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
    for result in &results[1..] {
        assert!(result.editable_render().is_none());
        assert!(result.output().starts_with("fallback:"));
    }
    assert_eq!(
        calls.into_inner(),
        [
            "dorc why --last --whylog=.whylog --whylog=.other",
            "dorc why --last --whylog=.whylog --unknown",
            "dorc why --last --whylog=../whylog",
            "dorc why --last --whylog=.whylog | wombat",
        ]
    );
}

#[test]
fn lint_driver_claims_only_the_exact_no_tools_shape() {
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
    for result in &results[1..] {
        assert!(result.editable_render().is_none());
        assert!(result.output().starts_with("fallback:"));
    }
    assert_eq!(
        calls.into_inner(),
        [
            "dorc lint oracle.sh",
            "dorc lint --no-tools oracle.sh",
            "dorc lint oracle.sh --no-tools --no-tools",
            "dorc lint ../oracle.sh --no-tools",
        ]
    );
}

#[test]
fn fixpoint_gate_catches_a_catalog_hand_edit() {
    let case = whylog_absent_case();
    let committed = DorcConsumer::new()
        .render_case(&case)
        .expect("case renders");
    let mut consumer = DorcConsumer::new();
    consumer.set_message("whylog-absent", Some("sm tampered message".to_owned()));
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

#[test]
fn explicit_marker_can_introduce_an_unused_typed_payload_value() {
    let case = Case::parse("---\ncode: cmdsub-operand-top\n---\n-- book.sh --\n#!/bin/sh\n-- replay --\n$ dorc plan --book=book.sh\n")
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
    assert_eq!(entry.message.as_deref(), Some("run {{command}}"));
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
    let cmdsub = Case::parse("---\ncode: cmdsub-operand-top\n---\n-- replay --\n$ dorc plan\n")
        .expect("case parses");
    let baseline = DorcConsumer::new()
        .editable_baseline(&cmdsub)
        .expect("baseline renders");
    assert_eq!(
        baseline.used_variables(),
        vec![
            (
                TemplateVariableName(String::from("position")),
                String::from("operand 1")
            ),
            (
                TemplateVariableName(String::from("cause")),
                String::from(
                    "a command-substitution `$(…)` / arithmetic / operator-form expansion"
                ),
            ),
        ]
    );
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

    let foreign = Case::parse("---\ncode: site-unresolvable\n---\n-- replay --\n$ dorc plan\n")
        .expect("case parses");
    let baseline = DorcConsumer::new()
        .editable_baseline(&foreign)
        .expect("foreign baseline");
    assert!(baseline.all_variables().is_empty());
}

#[test]
fn exact_replays_keep_editability_with_provenance_and_route_all_declines_to_the_injected_fallback()
{
    let source = "#!/bin/sh\napt-get install -y \"$(cat /etc/webhost/pkgset)\"\n";
    let case = Case::parse(&format!(
        "---\ncode: cmdsub-operand-top\n---\n-- book.sh --\n{source}-- probe.txt --\nprobe bytes\n-- replay --\n$ dorc plan --book=book.sh < probe.txt\nold\n$ dorc plan --book=book.sh --format=jsonl < probe.txt\nold\n$ dorc lint book.sh\nold\n$ dorc why --last\nold\n$ dorc plan --book=book.sh | jq --pretty\nold\n$ dorc plan --book=missing.sh\nold\n$ dorc plan --book=book.sh --book=book.sh\nold\n$ dorc plan --book=book.sh --unknown\nold\n$ dorc plan --book=../book.sh\nold\n$ dorc plan --book=./book.sh\nold\n"
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
    assert_eq!(
        results[1].output(),
        "{\"code\":\"cmdsub-operand-top\",\"severity\":\"note\"}\n"
    );
    for result in &results[2..] {
        assert!(result.editable_render().is_none());
        assert!(result.output().contains("{{command}}"));
    }
    assert_eq!(
        calls.into_inner(),
        [
            "dorc lint book.sh",
            "dorc why --last",
            "dorc plan --book=book.sh | jq --pretty",
            "dorc plan --book=missing.sh",
            "dorc plan --book=book.sh --book=book.sh",
            "dorc plan --book=book.sh --unknown",
            "dorc plan --book=../book.sh",
            "dorc plan --book=./book.sh",
        ]
    );
}

#[test]
fn replay_with_a_fake_fallback_leaves_case_catalog_and_source_bytes_unchanged() {
    let case_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cmdsub-command.txt");
    let catalog_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../core/src/catalog_lock.rs");
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
        "---\ncode: cmdsub-operand-top\n---\n-- replay --\n\
         $ dorc-loom vars --all self.txt\nold\n\
         $ dorc-loom vars --used other.txt\nold\n\
         $ dorc-loom vars --all missing.txt\nold\n\
         $ dorc-loom vars --all ../self.txt\nold\n\
         $ dorc-loom vars --all /self.txt\nold\n\
         $ dorc-loom vars --all self.txt extra\nold\n\
         $ dorc-loom vars --all 'self.txt'\nold\n",
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

    assert!(results[0].output().contains("{{command}} = \"apt-get\""));
    assert!(results[1].output().contains("{{verb}} = \"elide\""));
    assert!(!results[1].output().contains("{{command}} = \"apt-get\""));
    for result in &results[2..] {
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
            "dorc-loom vars --all 'self.txt'",
        ]
    );
}
