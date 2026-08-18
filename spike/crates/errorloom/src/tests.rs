use crate::{
    AlignmentLimit, DEFAULT_REMOVABLE_OCCURRENCES, DEFAULT_RENDER_SCALAR_CEILING, EditRefusal,
    EditRefusalClass, EditTransport, EditableFragment, EditableRender, EditableSection,
    RenderComponent, TransportLimits, VariableDrop, transport_edit, transport_edit_allow_removal,
    transport_edit_with_limits,
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

/// What an edit does to a variable is decided by the bytes ANCHORING it, not by the variable's
/// own bytes surviving. Editing an anchor destroys the attribution, and the unique remaining
/// interpretation demotes the occurrence — so the value silently becomes literal text and the
/// hole is gone. That is the whole residue the removal search leaves behind, and the four cells
/// below are the map of it: only the first loses a variable it could have kept.
#[test]
fn editing_a_variables_anchors_flattens_it_while_rewording_around_it_does_not() {
    let baseline = render(vec![
        EditableFragment::Text("Dorc cannot resolve `".to_owned()),
        variable(1, "apt-get"),
        EditableFragment::Text("`, so it runs.".to_owned()),
    ]);
    let surviving = |edited: &str| survivors(&baseline, edited);

    // Deleting the backticks touches the variable on both sides; dropping it is then the only
    // reading that aligns, and `apt-get` lands in the compiled text as an ordinary literal.
    assert!(surviving("Dorc cannot resolve apt-get, so it runs.").is_empty());

    // Rewording on both sides, anchors intact, keeps the attribution.
    assert_eq!(
        surviving("Dorc cannot resolve the command `apt-get`, so it runs on every apply."),
        vec![1]
    );

    // A genuine removal also drops the variable — indistinguishable from the first case by the
    // fragment list alone, which is why the value's reappearance is what separates them.
    assert!(surviving("Dorc cannot resolve it, so it runs.").is_empty());

    // Relocation across a substantial rewrite survives while the anchors travel with it.
    assert_eq!(
        surviving("Because `apt-get` is dynamic, Dorc cannot resolve it."),
        vec![1]
    );
}

/// Helper for the anchoring cells: which variables survive one edit, and what it gave up.
fn accepted(
    baseline: &EditableRender<u8, u16>,
    edited: &str,
) -> (Vec<u16>, Vec<VariableDrop<u16>>, String) {
    let (edit, drops) = match transport_edit_allow_removal(baseline, edited) {
        Ok(EditTransport::Edited(edit)) => (edit, Vec::new()),
        Ok(EditTransport::EditedWithDrops { edit, drops }) => (edit, drops),
        Ok(EditTransport::Unchanged) => panic!("expected an edit, got Unchanged"),
        Err(refusal) => panic!("expected an edit, got {:?}", refusal.class()),
    };
    let kept = edit
        .fragments()
        .iter()
        .filter_map(|fragment| match fragment {
            EditableFragment::Variable { id, .. } => Some(*id),
            EditableFragment::Text(_) => None,
        })
        .collect();
    let text =
        edit.fragments()
            .iter()
            .map(|fragment| match fragment {
                EditableFragment::Text(text)
                | EditableFragment::Variable { rendered: text, .. } => text.as_str(),
            })
            .collect();
    (kept, drops, text)
}

/// Helper for the anchoring cells: which variables survive one edit.
fn survivors(baseline: &EditableRender<u8, u16>, edited: &str) -> Vec<u16> {
    accepted(baseline, edited).0
}

/// Helper for the removal cells: what one edit gave up, and the evidence about each loss.
fn losses(baseline: &EditableRender<u8, u16>, edited: &str) -> Vec<VariableDrop<u16>> {
    accepted(baseline, edited).1
}

/// Helper for the refusal cells on the removal-allowing path.
fn removal_refusal(baseline: &EditableRender<u8, u16>, edited: &str) -> EditRefusal {
    match transport_edit_allow_removal(baseline, edited) {
        Ok(transport) => panic!("expected a refusal, got {transport:?}"),
        Err(refusal) => refusal,
    }
}

/// A variable delimited by ordinary whitespace is robust: both neighbouring words can be
/// rewritten, lengthened or shortened and the attribution holds, because the space itself is the
/// anchor and ordinary prose editing leaves it alone. This is the reassuring half of the picture
/// and the reason the machinery works at all in practice.
#[test]
fn whitespace_anchored_variables_survive_rewriting_either_neighbour() {
    let baseline = render(vec![
        EditableFragment::Text("The ".to_owned()),
        variable(1, "nginx"),
        EditableFragment::Text(" service is enabled.".to_owned()),
    ]);
    for edited in [
        "The nginx service is now enabled.",
        "Our nginx service is enabled.",
        "The nginx daemon is enabled.",
        "Our nginx daemon is enabled.",
        "A nginx service is enabled.",
    ] {
        assert_eq!(survivors(&baseline, edited), vec![1], "{edited}");
    }

    // Deleting the variable outright drops it, and its value does not come back as text.
    assert!(survivors(&baseline, "The service is enabled.").is_empty());

    // Gluing a suffix onto it consumes the right-hand anchor, and the attribution goes with it.
    assert!(survivors(&baseline, "The nginx-based service is enabled.").is_empty());
}

/// A SECTION EDGE IS NOT A VARIABLE'S ANCHOR. A variable's attribution rides the bytes
/// immediately beside it, but where it sits first or last in its section, one of those sides has
/// no bytes at all — nothing of the render lies out there. Treating that emptiness as an anchor
/// made the two commonest prose edits in existence, appending a clause and prefixing a word,
/// silently flatten the variable into literal text. Boundary holes are not hypothetical: the
/// shipped catalog has registers ending in `{{oracles}}` / `{{expected}}` / `{{output}}` and ten
/// beginning with a hole.
#[test]
fn a_section_edge_is_not_a_variables_anchor() {
    let trailing = render(vec![
        EditableFragment::Text("wrote: ".to_owned()),
        variable(1, "/etc/nginx/nginx.conf"),
    ]);
    // Rewriting the word before it is safe; the ": " anchor survives.
    assert_eq!(
        survivors(&trailing, "saved: /etc/nginx/nginx.conf"),
        vec![1]
    );
    // Even removing the colon is safe -- the space is the immediate anchor, not the punctuation.
    assert_eq!(survivors(&trailing, "wrote /etc/nginx/nginx.conf"), vec![1]);
    let (kept, drops, text) = accepted(&trailing, "wrote: /etc/nginx/nginx.conf successfully");
    assert_eq!((kept, drops.len()), (vec![1], 0));
    assert_eq!(text, "wrote: /etc/nginx/nginx.conf successfully");
    // Deleting it outright still drops it: the edge is open, the value is simply gone.
    assert!(survivors(&trailing, "wrote: nothing").is_empty());

    let leading = render(vec![
        variable(1, "nginx"),
        EditableFragment::Text(" is not installed.".to_owned()),
    ]);
    assert_eq!(survivors(&leading, "nginx is not present."), vec![1]);
    assert_eq!(survivors(&leading, "The nginx is not installed."), vec![1]);
    assert_eq!(survivors(&leading, "The nginx is not installed!"), vec![1]);
    // Gluing a word onto the open side keeps it too, and SHOULD: the author typed
    // `The{{command}} is not installed.`, which is a legal template and their own spelling.
    assert_eq!(survivors(&leading, "Thenginx is not installed."), vec![1]);
    // Replacing it outright is still a removal -- the value left with the words.
    assert!(survivors(&leading, "The service is not installed.").is_empty());

    // A section that is nothing BUT a variable has two open edges and no anchoring text at all.
    let alone = render(vec![variable(1, "nginx")]);
    assert_eq!(survivors(&alone, "The nginx service"), vec![1]);
}

/// TARGET 1, the one place a silent WRONG-variable drop could live: two variables rendering
/// identical bytes, one of them removed. Nothing in the edited text says which occurrence went,
/// so the alignment decides — and the answer splits on the surrounding prose.
///
/// Where the two sit in symmetric surroundings, both readings align and the transport refuses as
/// ambiguous, which is the safe answer. Where the surroundings differ — most sharply when one of
/// them sits at a section edge — exactly one reading survives and the transport ACCEPTS it,
/// naming a survivor the author may not have meant. The rendered bytes are identical either way,
/// so nothing downstream can notice; the drop report is the only thing that can say so, which is
/// why it is a typed result the consumer cannot match past.
#[test]
fn equal_valued_variables_refuse_symmetrically_and_pick_a_winner_at_an_edge() {
    let symmetric = render(vec![
        EditableFragment::Text("from ".to_owned()),
        variable(1, "same"),
        EditableFragment::Text(" to ".to_owned()),
        variable(2, "same"),
        EditableFragment::Text(" done".to_owned()),
    ]);
    for edited in [
        "from same done",
        "to same done",
        "from same-ish to same done",
    ] {
        assert_eq!(
            removal_refusal(&symmetric, edited).class(),
            EditRefusalClass::AmbiguousAttribution,
            "{edited}"
        );
    }
    // Editing BESIDE one of them touches neither, so equal values cost nothing here.
    assert_eq!(
        survivors(&symmetric, "from now same to same done"),
        vec![1, 2]
    );

    let trailing = render(vec![
        EditableFragment::Text("from ".to_owned()),
        variable(1, "same"),
        EditableFragment::Text(" to ".to_owned()),
        variable(2, "same"),
    ]);
    // The author deleted the SECOND clause; the only reading that aligns keeps the second
    // variable and reports the first as dropped. Accepted, not refused -- and the value is still
    // in the text, so the report says flattening rather than deletion.
    let (kept, drops, _) = accepted(&trailing, "from same");
    assert_eq!(kept, vec![2]);
    assert_eq!(drops.len(), 1, "{drops:?}");
    assert_eq!(drops.first().map(VariableDrop::id), Some(&1));

    let leading = render(vec![
        variable(1, "same"),
        EditableFragment::Text(" to ".to_owned()),
        variable(2, "same"),
        EditableFragment::Text(" done".to_owned()),
    ]);
    // Mirrored, and mirrored the same way: deleting the leading clause reports the TRAILING
    // variable gone.
    let (kept, drops, _) = accepted(&leading, "same done");
    assert_eq!(kept, vec![1]);
    assert_eq!(drops.first().map(VariableDrop::id), Some(&2));
}

/// TARGET 2: an edit that abuts one variable is contained to it. The removal search demotes only
/// what it must, and the abutment rule still guards every RETAINED occurrence inside the search —
/// so a mask that also gives up the untouched neighbour has no alignment advantage and loses.
#[test]
fn an_edit_abutting_one_variable_leaves_its_distinct_neighbour_alone() {
    let baseline = render(vec![
        EditableFragment::Text("run ".to_owned()),
        variable(1, "nginx"),
        EditableFragment::Text(" then ".to_owned()),
        variable(2, "apache"),
        EditableFragment::Text(" now".to_owned()),
    ]);
    assert_eq!(
        survivors(&baseline, "run nginx-ssl then apache now"),
        vec![2]
    );
    assert_eq!(survivors(&baseline, "run nginx then apache2 now"), vec![1]);
    // Rewording the text BETWEEN them touches neither.
    assert_eq!(
        survivors(&baseline, "run nginx and then apache now"),
        vec![1, 2]
    );
}

/// TARGET 3: with distinct values, the variable reported dropped is the one whose bytes left.
/// Attribution follows the bytes, and distinct bytes make the reading unique.
#[test]
fn deleting_one_of_two_distinct_variables_reports_the_one_that_went() {
    let baseline = render(vec![
        EditableFragment::Text("run ".to_owned()),
        variable(1, "nginx"),
        EditableFragment::Text(" then ".to_owned()),
        variable(2, "apache"),
        EditableFragment::Text(" now".to_owned()),
    ]);

    let (kept, drops, _) = accepted(&baseline, "run nginx now");
    assert_eq!(kept, vec![1]);
    assert_eq!(drops.first().map(VariableDrop::id), Some(&2));
    assert_eq!(drops.first().map(VariableDrop::rendered), Some("apache"));
    assert_eq!(
        drops.first().map(VariableDrop::value_reappears_as_text),
        Some(false),
        "a value that left with its words is a deletion, not a flattening"
    );
    // AS BUILT: the retention class does NOT separate a deletion from a flattening -- both
    // refuse the same way, and it is the reappearance fact above that tells them apart. The
    // class carries WHAT the edit contacted, and today a candidate section is the only thing it
    // can have contacted.
    assert_eq!(
        drops.first().map(VariableDrop::retention_refusal),
        Some(EditRefusalClass::EditableVariableTouched)
    );

    let (kept, drops, _) = accepted(&baseline, "run apache now");
    assert_eq!(kept, vec![2]);
    assert_eq!(drops.first().map(VariableDrop::id), Some(&1));
}

/// TARGET 4: deleting a clause that ABUTS a surviving variable does not cascade into it. The
/// anchor beside `{{v1}}` is one space; the edited text still has a space there, and an anchor is
/// matched by CONTENT, not by which byte of the original it used to be. Only the deleted clause's
/// own variable goes.
#[test]
fn deleting_a_clause_beside_a_variable_does_not_cascade_into_it() {
    let baseline = render(vec![
        EditableFragment::Text("Use ".to_owned()),
        variable(1, "apt-get"),
        EditableFragment::Text(" or ".to_owned()),
        variable(2, "dpkg"),
        EditableFragment::Text(" here.".to_owned()),
    ]);
    let (kept, drops, _) = accepted(&baseline, "Use apt-get here.");
    assert_eq!(kept, vec![1]);
    assert_eq!(drops.first().map(VariableDrop::id), Some(&2));
    assert_eq!(survivors(&baseline, "Use dpkg here."), vec![2]);
}

/// TARGET 5: the removal search is exponential in a section's variable count, so a section past
/// the bound refuses as a LIMIT and says which ceiling it hit -- distinguishable from an
/// ambiguity, which is a statement about the edit rather than about the machine. Exactly at the
/// bound it still answers.
#[test]
fn a_section_past_the_removable_occurrence_bound_refuses_as_a_limit() {
    let section = |count: u16| {
        render(
            (0..count)
                .flat_map(|id| [variable(id, "v"), EditableFragment::Text(" ".to_owned())])
                .collect(),
        )
    };
    let bound = u16::try_from(DEFAULT_REMOVABLE_OCCURRENCES).unwrap_or(u16::MAX);

    let refusal = removal_refusal(&section(bound.saturating_add(1)), "changed");
    assert_eq!(refusal.class(), EditRefusalClass::AlignmentLimitExceeded);
    assert_eq!(
        refusal.limit().map(|limit| limit.exceeded),
        Some(AlignmentLimit::RemovableOccurrences)
    );
    assert!(transport_edit_allow_removal(&section(bound), "changed").is_ok());
}

/// TARGET 6: an edit spanning two sections refuses on the removal path too. Removal widens which
/// interpretations are reachable, never which SECTION may own an edit -- a template's words
/// belong to exactly one register (`282:rul-variable-edit-section-scope`).
#[test]
fn a_cross_section_edit_refuses_even_when_removal_is_allowed() {
    let two = EditableRender::new(vec![
        RenderComponent::EditableSection(EditableSection::<u8, u16>::new(
            1,
            vec![
                EditableFragment::Text("alpha ".to_owned()),
                variable(1, "one"),
            ],
        )),
        RenderComponent::Structure(" | ".to_owned()),
        RenderComponent::EditableSection(EditableSection::new(
            2,
            vec![
                EditableFragment::Text("beta ".to_owned()),
                variable(2, "two"),
            ],
        )),
    ]);
    assert_eq!(
        removal_refusal(&two, "alpha  | beta ").class(),
        EditRefusalClass::CrossSection
    );
}

/// TARGET 7: a value the baseline prose ALREADY spelled out does not make its own deletion look
/// like a flattening. The evidence is an occurrence-count delta, so a value that was there before
/// and is there after has not reappeared -- where a bare `contains` would call every deletion of
/// such a variable a frozen world and teach the author to ignore the warning.
#[test]
fn a_value_the_baseline_text_already_carried_is_not_a_reappearance() {
    let baseline = render(vec![
        EditableFragment::Text("nginx: the ".to_owned()),
        variable(1, "nginx"),
        EditableFragment::Text(" service".to_owned()),
    ]);
    let (kept, drops, text) = accepted(&baseline, "nginx: the service");
    assert!(kept.is_empty());
    assert_eq!(text, "nginx: the service");
    assert_eq!(
        drops.first().map(VariableDrop::value_reappears_as_text),
        Some(false),
        "the surviving `nginx` is the one the prose always had: {drops:?}"
    );

    // The flattening the delta is FOR: the same value, still in the text, one occurrence more
    // than the baseline literal carried.
    let anchored = render(vec![
        EditableFragment::Text("Dorc cannot resolve `".to_owned()),
        variable(1, "apt-get"),
        EditableFragment::Text("`, so it runs.".to_owned()),
    ]);
    let drops = losses(&anchored, "Dorc cannot resolve apt-get, so it runs.");
    assert_eq!(
        drops.first().map(VariableDrop::value_reappears_as_text),
        Some(true),
        "{drops:?}"
    );
    assert_eq!(
        drops.first().map(VariableDrop::retention_refusal),
        Some(EditRefusalClass::EditableVariableTouched)
    );
}
