use crate::{
    AlignmentLimit, DEFAULT_RENDER_SCALAR_CEILING, EditRefusalClass, EditTransport,
    EditableFragment, EditableRender, EditableSection, RenderComponent, TransportLimits,
    transport_edit, transport_edit_allow_removal, transport_edit_with_limits,
};

fn render(fragments: Vec<EditableFragment<u16>>) -> EditableRender<u8, u16> {
    EditableRender::new(vec![RenderComponent::EditableSection(
        EditableSection::new(1, fragments),
    )])
}

fn variable(id: u16, rendered: &str) -> EditableFragment<u16> {
    EditableFragment::Variable {
        id,
        rendered: rendered.to_owned(),
    }
}

fn refusal(render: &EditableRender<u8, u16>, edited: &str) -> EditRefusalClass {
    transport_edit(render, edited)
        .expect_err("must refuse")
        .class()
}

#[test]
fn seeded_disjoint_text_edits_preserve_opaque_variables() {
    let values = ["", "same", "a-b", "…", "猫"];
    for seed in 0..64u8 {
        let baseline = render(vec![
            EditableFragment::Text(format!("left-{seed} ")),
            variable(1, values[usize::from(seed % 5)]),
            EditableFragment::Text(format!(" middle-{seed} ")),
            variable(2, values[usize::from((seed + 1) % 5)]),
            EditableFragment::Text(format!(" right-{seed}")),
        ]);
        let edited = baseline
            .text()
            .replacen("left", "start", 1)
            .replacen("right", "end", 1);
        let EditTransport::Edited(edit) =
            transport_edit(&baseline, &edited).expect("disjoint edit")
        else {
            panic!("edit")
        };
        assert_eq!(
            edit.fragments()
                .iter()
                .filter_map(|f| match f {
                    EditableFragment::Variable { id, .. } => Some(*id),
                    EditableFragment::Text(_) => None,
                })
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            edit.fragments()
                .iter()
                .map(|f| match f {
                    EditableFragment::Text(t) | EditableFragment::Variable { rendered: t, .. } =>
                        t.as_str(),
                })
                .collect::<String>(),
            edited
        );
    }
}

#[test]
fn equal_values_keep_identity_and_ambiguous_removal_refuses() {
    let baseline = render(vec![
        EditableFragment::Text("from ".into()),
        variable(1, "same"),
        EditableFragment::Text(" to ".into()),
        variable(2, "same"),
    ]);
    let EditTransport::Edited(edit) =
        transport_edit(&baseline, "copy from same to same").expect("text edit")
    else {
        panic!("edit")
    };
    assert_eq!(
        edit.fragments()
            .iter()
            .filter_map(|f| match f {
                EditableFragment::Variable { id, .. } => Some(*id),
                EditableFragment::Text(_) => None,
            })
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    let removable = render(vec![variable(1, "same"), variable(2, "same")]);
    assert_eq!(
        transport_edit_allow_removal(&removable, "same")
            .expect_err("ambiguous removal")
            .class(),
        EditRefusalClass::AmbiguousAttribution
    );
}

#[test]
fn boundaries_and_immutable_components_refuse() {
    let section = render(vec![
        EditableFragment::Text("a".into()),
        variable(1, "value"),
        EditableFragment::Text("b".into()),
    ]);
    assert_eq!(
        refusal(&section, "a valueb"),
        EditRefusalClass::EditableVariableTouched
    );
    let outer = EditableRender::new(vec![
        RenderComponent::Structure("[".into()),
        RenderComponent::EditableSection(EditableSection::<u8, u16>::new(
            1,
            vec![EditableFragment::Text("ok".into())],
        )),
        RenderComponent::FixedVariable {
            id: 2,
            rendered: "]".into(),
        },
    ]);
    assert_eq!(
        transport_edit(&outer, "{ok]")
            .expect_err("structure")
            .class(),
        EditRefusalClass::StructureTouched
    );
    assert_eq!(
        transport_edit(&outer, "[ok}").expect_err("fixed").class(),
        EditRefusalClass::FixedVariableTouched
    );
    let two = EditableRender::new(vec![
        RenderComponent::EditableSection(EditableSection::<u8, u16>::new(
            1,
            vec![EditableFragment::Text("one".into())],
        )),
        RenderComponent::Structure("|".into()),
        RenderComponent::EditableSection(EditableSection::new(
            2,
            vec![EditableFragment::Text("two".into())],
        )),
    ]);
    assert_eq!(
        transport_edit(&two, "ONE|TWO")
            .expect_err("cross section")
            .class(),
        EditRefusalClass::CrossSection
    );
}

#[test]
fn bounded_alignment_refuses_scalar_and_work_exhaustion() {
    let over = DEFAULT_RENDER_SCALAR_CEILING.saturating_add(9);
    let huge = render(vec![EditableFragment::Text("x".repeat(over))]);
    let limit = transport_edit(&huge, "changed")
        .expect_err("must refuse")
        .limit()
        .cloned()
        .expect("a limit refusal carries its resource metadata");
    assert_eq!(limit.exceeded, AlignmentLimit::RenderScalars);
    // The whole point of the metadata: HOW FAR over. A `ceiling + 1` sentinel here would
    // tell a reader only what they already knew from the ceiling field.
    assert_eq!(limit.baseline_scalars, over);
    assert_eq!(limit.scalar_ceiling, DEFAULT_RENDER_SCALAR_CEILING);

    let fragments = (0..700)
        .flat_map(|id| [variable(id, "x"), EditableFragment::Text(" ".into())])
        .collect();
    let baseline = render(fragments);
    let edited = baseline.text().replacen(' ', "!", 1);
    let limit = transport_edit(&baseline, &edited)
        .expect_err("must refuse")
        .limit()
        .cloned()
        .expect("a limit refusal carries its resource metadata");
    assert_eq!(
        limit.exceeded,
        AlignmentLimit::AlignmentWork,
        "a render well inside the scalar ceiling refused for COST, and says so"
    );
    assert!(limit.baseline_scalars < limit.scalar_ceiling);
}

/// The ceilings are the caller's: an embedder on a smaller budget can refuse renders the
/// default accepts, and the refusal reports the ceiling that was actually in force.
#[test]
fn caller_supplied_limits_replace_the_defaults() {
    let baseline = render(vec![EditableFragment::Text("x".repeat(64))]);
    let edited = "y".repeat(64);
    let limit = transport_edit_with_limits(
        &baseline,
        &edited,
        TransportLimits {
            scalar_ceiling: 16,
            ..TransportLimits::default()
        },
    )
    .expect_err("over the caller's ceiling")
    .limit()
    .cloned()
    .expect("resource metadata");
    assert_eq!(limit.scalar_ceiling, 16);
    assert_eq!(limit.baseline_scalars, 64);
    assert!(
        transport_edit(&baseline, &edited).is_ok(),
        "the same edit is comfortably inside the default ceiling"
    );
}
