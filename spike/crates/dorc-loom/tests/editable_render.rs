//! Core render-part adapter regressions.

use dorc_aid::tagged::{Field, RenderPart, RenderParts};
use dorc_loom::{SectionVariableId, TemplateVariableName, to_editable_render};
use errorloom::{EditableFragment, RenderComponent};

fn parts(parts: Vec<RenderPart>) -> RenderParts {
    let mut render = RenderParts::new();
    for part in parts {
        render.push(part);
    }
    render
}

fn literal(text: &str) -> RenderPart {
    RenderPart::TemplateLiteral {
        text: String::from(text),
        code: "code",
        field: Field::Message,
        paragraph: 0,
        instance: 0,
    }
}

fn variable(name: &'static str, text: &str) -> RenderPart {
    RenderPart::ParamValue {
        text: String::from(text),
        code: "code",
        field: Field::Message,
        param: name,
        instance: 0,
    }
}

#[test]
fn adapter_preserves_bytes_and_sections_around_foreign_data() {
    let parts = parts(vec![
        RenderPart::Arrangement {
            text: String::from("["),
            slug: "prefix",
        },
        literal("a"),
        RenderPart::ForeignText {
            text: dorc_aid::ForeignBytes::from_io_edge("").on_measured_sink(64),
            source: String::from("detail"),
        },
        literal("b"),
        RenderPart::Arrangement {
            text: String::from("]"),
            slug: "suffix",
        },
    ]);
    let render = to_editable_render(&parts);
    assert_eq!(render.text(), parts.text());
    assert!(matches!(render.components()[0], RenderComponent::Structure(ref text) if text == "["));
    assert!(matches!(
        render.components()[2],
        RenderComponent::FixedVariable { ref rendered, .. } if rendered.is_empty()
    ));
    let RenderComponent::EditableSection(first) = &render.components()[1] else {
        panic!("first editable section")
    };
    let RenderComponent::EditableSection(second) = &render.components()[3] else {
        panic!("second editable section")
    };
    assert_eq!(first.id().segment, 0);
    assert_eq!(second.id().segment, 1);
    assert_eq!(first.id().owner, second.id().owner);
    assert_eq!(first.id().field, second.id().field);
    assert_eq!(first.id().instance, second.id().instance);
}

#[test]
fn adapter_keeps_repeated_empty_variables_distinct() {
    let parts = parts(vec![
        literal("a"),
        variable("name", ""),
        variable("name", "x"),
    ]);
    let render = to_editable_render(&parts);
    assert_eq!(render.text(), "ax");
    let RenderComponent::EditableSection(section) = &render.components()[0] else {
        panic!("editable section")
    };
    assert_eq!(
        section.fragments(),
        [
            EditableFragment::Text(String::from("a")),
            EditableFragment::Variable {
                id: SectionVariableId {
                    name: TemplateVariableName(String::from("name")),
                    occurrence: 0,
                },
                rendered: String::new(),
            },
            EditableFragment::Variable {
                id: SectionVariableId {
                    name: TemplateVariableName(String::from("name")),
                    occurrence: 1,
                },
                rendered: String::from("x"),
            },
        ]
    );
}

#[test]
fn adapter_coalesces_adjacent_template_text() {
    let parts = parts(vec![literal("a"), literal("b")]);
    let render = to_editable_render(&parts);
    let RenderComponent::EditableSection(section) = &render.components()[0] else {
        panic!("editable section")
    };
    assert_eq!(
        section.fragments(),
        [EditableFragment::Text(String::from("ab"))]
    );
}
