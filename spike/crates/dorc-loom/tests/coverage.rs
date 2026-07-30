//! `RenderParts` coverage for the editable adapter.

use dorc_aid::catalog::{CATALOG, fill_template, fill_template_parts};
use dorc_aid::diag::{
    Diag, DiagCode, MissingDialectMarker, RenderHeredocRefused, SiteId, SiteUnresolvable,
    render_body, render_body_parts, render_cli, render_cli_parts, render_staged_cli_parts,
};
use dorc_aid::tagged::{Field, RenderPart};
use dorc_core::{BytePos, Interner, LeafId, Span as SourceSpan};
use dorc_loom::to_editable_render;
use errorloom::{EditableFragment, RenderComponent};

#[test]
fn every_catalog_template_maps_through_render_parts() {
    for entry in CATALOG {
        let values: Vec<(&'static str, String)> = entry
            .params
            .iter()
            .map(|param| (*param, format!("value-for-{param}")))
            .collect();
        let refs: Vec<(&'static str, &str)> = values
            .iter()
            .map(|(param, value)| (*param, value.as_str()))
            .collect();
        for (field, template) in [
            (Field::Message, entry.message),
            (Field::Help, entry.help.written().copied()),
        ] {
            let Some(template) = template else { continue };
            let parts = fill_template_parts(template, &refs, entry.slug, field, 0)
                .unwrap_or_else(|error| panic!("{}: {error:?}", entry.slug));
            assert_eq!(fill_template(template, &refs), Ok(parts.text()));
            assert_eq!(to_editable_render(&parts).text(), parts.text());
        }
    }
}

#[test]
fn production_render_parts_match_bytes_and_preserve_parameter_identity() {
    let interner = Interner::default();
    let src = "make install >/etc/motd\nldconfig\n";
    let span = SourceSpan::new(BytePos(0), BytePos(4));
    let diagnostics = [
        Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                site: SiteId::leaf(LeafId(7)),
                verb: "elide",
                command: "cat <<EOF".to_owned(),
            }),
            span,
        ),
        Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: SiteId::leaf(LeafId(4)),
                detail: "2 sites run unprobed: `make install`, `ldconfig`".to_owned(),
            }),
            span,
        ),
        Diag::new(DiagCode::MissingDialectMarker(MissingDialectMarker), span),
    ];

    for diag in &diagnostics {
        let body = render_body_parts(diag, &interner);
        let cli = render_cli_parts(
            &dorc_aid::catalog::CONST_CATALOG,
            diag,
            src,
            "book.sh",
            &interner,
            dorc_aid::diag::CANONICAL_TRANSCRIPT_WIDTH,
        );
        assert_eq!(body.text(), render_body(diag, &interner));
        assert_eq!(cli.text(), render_cli(diag, src, "book.sh", &interner));
        // The stage prefix occupies columns, so a staged render is laid out AROUND it rather than
        // being the unstaged bytes with a prefix glued on.
        let staged = render_staged_cli_parts(
            "whylog",
            &dorc_aid::catalog::CONST_CATALOG,
            diag,
            src,
            "book.sh",
            &interner,
            dorc_aid::diag::CANONICAL_TRANSCRIPT_WIDTH,
        )
        .text();
        assert!(staged.starts_with("whylog: "), "{staged}");
        assert!(
            staged.contains(&format!("[{}]: ", diag.code.slug())),
            "{staged}"
        );
        assert_eq!(to_editable_render(&body).text(), body.text());
        assert_eq!(to_editable_render(&cli).text(), cli.text());
    }

    let parts = render_body_parts(&diagnostics[0], &interner);
    let render = to_editable_render(&parts);
    assert!(render.components().iter().any(|component| matches!(
        component,
        RenderComponent::EditableSection(section)
            if section.fragments().iter().any(|fragment| matches!(
                fragment,
                EditableFragment::Variable { id, rendered }
                    if id.name.0 == "command" && rendered == "cat <<EOF"
            ))
    )));
    assert!(parts.parts().iter().any(|part| matches!(
        part,
        RenderPart::ParamValue {
            param: "command",
            instance: 0,
            ..
        }
    )));
}
