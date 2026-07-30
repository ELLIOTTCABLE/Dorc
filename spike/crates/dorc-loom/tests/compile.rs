//! Public section compiler regressions.

use std::collections::BTreeMap;

use dorc_loom::{
    CompileRefusal, CompiledFragment, SectionVariableId, TemplateVariableName, compile_fragments,
};
use errorloom::EditableFragment;

fn name(name: &str) -> TemplateVariableName {
    TemplateVariableName(String::from(name))
}
fn values(values: &[(&str, &str)]) -> BTreeMap<TemplateVariableName, String> {
    values
        .iter()
        .map(|(key, value)| (name(key), String::from(*value)))
        .collect()
}
fn variable(key: &str, rendered: &str) -> EditableFragment<SectionVariableId> {
    EditableFragment::Variable {
        id: SectionVariableId {
            name: name(key),
            occurrence: 0,
        },
        rendered: String::from(rendered),
    }
}

#[test]
fn compiler_binds_exact_empty_and_nul_values() {
    let compiled = compile_fragments(
        &[EditableFragment::Text(String::from("{{empty}} {{nul}}"))],
        &values(&[("empty", ""), ("nul", "\0")]),
    )
    .unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(compiled.text(), " \0");
    assert_eq!(
        compiled.bindings(),
        &values(&[("empty", ""), ("nul", "\0")])
    );
}

#[test]
fn compiler_refuses_an_unknown_name_and_binds_adjacent_markers() {
    assert!(matches!(
        compile_fragments(
            &[EditableFragment::Text(String::from("{{missing}}"))],
            &values(&[])
        ),
        Err(CompileRefusal::UnknownVariable(_))
    ));
    let compiled = compile_fragments(
        &[EditableFragment::Text(String::from("{{a}}{{b}}"))],
        &values(&[("a", "a"), ("b", "b")]),
    )
    .unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(compiled.text(), "ab");
    assert_eq!(compiled.used(), &[name("a"), name("b")]);
}

#[test]
fn compiler_markers_override_and_respect_fragment_boundaries() {
    let replaced = vec![
        EditableFragment::Text(String::from("run ")),
        variable("command", "apt"),
        EditableFragment::Text(String::from(" ")),
        EditableFragment::Text(String::from(" {{command}} ")),
    ];
    let compiled = compile_fragments(&replaced, &values(&[("command", "apt")]))
        .unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(compiled.used(), &[name("command")]);
    assert_eq!(compiled.text(), "run   apt ");
    let glued = compile_fragments(
        &[
            variable("path", "/x"),
            EditableFragment::Text(String::from("{{command}}")),
        ],
        &values(&[("path", "/x"), ("command", "apt")]),
    )
    .unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(glued.text(), "/xapt");
    let accepted = compile_fragments(
        &[
            variable("path", "/x"),
            EditableFragment::Text(String::from(" {{command}} ")),
        ],
        &values(&[("path", "/x"), ("command", "apt")]),
    )
    .unwrap_or_else(|error| panic!("{error:?}"));
    assert_eq!(accepted.text(), "/x apt ");
    assert!(matches!(
        accepted.fragments()[0],
        CompiledFragment::Variable(_)
    ));
}
