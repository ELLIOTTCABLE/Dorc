//! The adapter coverage proof (`282` §4 · `28A` §2c): every rendered catalog code
//! maps to a VALID `errorloom::TaggedRender` (gap-free total cover through
//! `TaggedRender::new`), and the region classes round-trip against the catalog's
//! own template/param structure. Two halves:
//!
//! * `every_catalog_code_...` sweeps the WHOLE catalog with synthetic distinctive
//!   param values — no `DiagCode` payload needed, so it covers every code
//!   (superset of the defining-case set), exercising template/param structure and
//!   the passthrough `ForeignText` split.
//! * `representative_real_renders_...` drives the REAL [`render_body_tagged`] seat
//!   on one payload per species (templatized / passthrough / static / help /
//!   children+suggestion), proving `text()` is byte-identical to [`render_body`]
//!   and the real `params_of` values attribute correctly.

use dorc_core::catalog::{CATALOG, CONST_CATALOG, fill_template_tagged};
use dorc_core::diag::{
    Applicability, Diag, DiagCode, MissingDialectMarker, MungeNameInvalid, RemediationClass,
    RenderHeredocRefused, SiteId, SiteUnresolvable, Suggestion, WhylogVersionRefused, params_of,
    render_body, render_body_tagged, render_cli, render_cli_tagged,
};
use dorc_core::tagged::{Field, Region, Span, TaggedRender};
use dorc_core::{BytePos, Interner, LeafId, Span as SourceSpan};
use dorc_loom::to_errorloom;
use errorloom::Region as LoomRegion;

/// Assert each `ParamValue`/`ForeignText` span points at exactly its expected
/// value (`value_of(param)`), the attribution round-trip.
fn assert_param_spans_attribute(
    render: &errorloom::TaggedRender<dorc_loom::FieldKey>,
    slug: &str,
    value_of: impl Fn(&str) -> Option<String>,
) {
    for span in render.spans() {
        let (LoomRegion::ParamValue { param, .. } | LoomRegion::ForeignText { param }) =
            &span.region
        else {
            continue;
        };
        let name = param.as_str();
        assert_eq!(
            render.text().get(span.range.clone()).map(str::to_owned),
            value_of(name),
            "code `{slug}` param `{name}`: span text disagrees with the interpolated value"
        );
    }
}

#[test]
fn every_catalog_code_maps_to_a_valid_tagged_render() {
    for e in CATALOG {
        let vals: Vec<(&'static str, String)> = e
            .params
            .iter()
            .map(|p| (*p, format!("valFor-{p}")))
            .collect();
        let refs: Vec<(&'static str, &str)> = vals.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let mut out = String::new();
        let mut spans: Vec<Span> = Vec::new();
        // An unwritten (`None`) message tags its synthesized placeholder WHOLE as Arrangement — the
        // same arm `render_body_tagged` takes (`283:dec-message-becomes-option`).
        match e.message {
            Some(message) => fill_template_tagged(
                &mut out,
                &mut spans,
                message,
                &refs,
                e.slug,
                Field::Message,
                0,
            ),
            None => {
                let placeholder = format!("[unwritten: {}]", e.slug);
                out.push_str(&placeholder);
                spans.push(Span {
                    range: 0..out.len(),
                    region: Region::Arrangement {
                        slug: "unwritten-placeholder",
                    },
                });
            }
        }
        if let Some(help) = e.help {
            let start = out.len();
            out.push_str("\n  = help: ");
            spans.push(Span {
                range: start..out.len(),
                region: Region::Arrangement {
                    slug: "help-connective",
                },
            });
            fill_template_tagged(&mut out, &mut spans, help, &refs, e.slug, Field::Help, 0);
        }

        let core = TaggedRender::new(out, spans);
        let render = to_errorloom(&core)
            .unwrap_or_else(|err| panic!("code `{}` failed the total-cover check: {err}", e.slug));
        assert_param_spans_attribute(&render, e.slug, |name| {
            vals.iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.clone())
        });
    }
}

#[test]
fn representative_real_renders_are_byte_unchanged_and_valid() {
    let interner = Interner::default();
    let span = SourceSpan::new(BytePos(0), BytePos(1));

    let templatized = DiagCode::RenderHeredocRefused(RenderHeredocRefused {
        site: SiteId::leaf(LeafId(7)),
        verb: "elide",
        command: "cat <<EOF".to_owned(),
    });
    let passthrough = DiagCode::SiteUnresolvable(SiteUnresolvable {
        site: SiteId::leaf(LeafId(4)),
        detail: "2 sites run unprobed: `make install`, `ldconfig`".to_owned(),
    });
    let statik = DiagCode::MissingDialectMarker(MissingDialectMarker);
    let multi = DiagCode::MungeNameInvalid(MungeNameInvalid {
        source: "9pkg".to_owned(),
        funcname: "9pkg".to_owned(),
        problem: "starts with a digit".to_owned(),
    });
    let authored = DiagCode::WhylogVersionRefused(WhylogVersionRefused {
        found: "dorc-whylog/2".to_owned(),
    });

    for code in [templatized, passthrough, statik, multi, authored] {
        let diag = Diag::new(code.clone(), span);
        let core = render_body_tagged(&diag, &interner);
        assert_eq!(
            core.text(),
            render_body(&diag, &interner),
            "code `{}`: the tagged twin drifted from render_body",
            code.slug()
        );
        let render = to_errorloom(&core).unwrap_or_else(|err| {
            panic!("code `{}` failed the total-cover check: {err}", code.slug())
        });
        let params = params_of(&code, &interner);
        assert_param_spans_attribute(&render, code.slug(), |name| {
            params
                .iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.clone())
        });
    }

    // A diag carrying emit-site notes + a suggestion (empty in production): the
    // tagged twin must still be byte-identical and valid — the Arrangement branch.
    let noisy = Diag::new(
        DiagCode::RenderHeredocRefused(RenderHeredocRefused {
            site: SiteId::leaf(LeafId(7)),
            verb: "elide",
            command: "cat <<EOF".to_owned(),
        }),
        span,
    )
    .note("an added fact")
    .help("a manual hint")
    .suggest(Suggestion {
        message: "split the heredoc body".to_owned(),
        applicability: Applicability::Unspecified,
        remediation: RemediationClass::Structural,
    });
    let core = render_body_tagged(&noisy, &interner);
    assert_eq!(
        core.text(),
        render_body(&noisy, &interner),
        "notes+suggestion: the tagged twin drifted from render_body"
    );
    to_errorloom(&core).expect("notes+suggestion render is a valid total cover");
}

/// The FULL-transcript tagged twin (`282` §2 · `28A` §2m): [`render_cli_tagged`] must be byte-
/// identical to [`render_cli`] and map to a VALID `errorloom::TaggedRender` (the gap-free total cover
/// through the real adapter) — the title-split relocation validated end-to-end, including a spanned
/// caret frame and a `detail` passthrough whose value embeds a `\n  = note:` fold (the straddle case
/// the split must handle).
#[test]
fn cli_tagged_twin_is_byte_unchanged_and_covers_through_the_adapter() {
    let interner = Interner::default();
    let src = "make install >/etc/motd\nldconfig\n";
    let span = SourceSpan::new(BytePos(0), BytePos(4));

    let spanned = DiagCode::RenderHeredocRefused(RenderHeredocRefused {
        site: SiteId::leaf(LeafId(7)),
        verb: "elide",
        command: "cat <<EOF".to_owned(),
    });
    let folded_detail = DiagCode::SiteUnresolvable(SiteUnresolvable {
        site: SiteId::leaf(LeafId(4)),
        detail:
            "2 sites run unprobed: `make install`, `ldconfig`\n  = note: site runs `make install`"
                .to_owned(),
    });

    for code in [spanned, folded_detail] {
        let diag = Diag::new(code.clone(), span);
        let core = render_cli_tagged(&CONST_CATALOG, &diag, src, "book.sh", &interner);
        assert_eq!(
            core.text(),
            render_cli(&diag, src, "book.sh", &interner),
            "code `{}`: the cli tagged twin drifted from render_cli",
            code.slug()
        );
        to_errorloom(&core).unwrap_or_else(|err| {
            panic!(
                "code `{}` cli render is not a total cover: {err}",
                code.slug()
            )
        });
    }
}
