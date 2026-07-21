//! The tagged-render primitive gates (`282` §4): [`fill_template_tagged`] must be
//! byte-identical to the product [`fill_template`] (the twin never drifts the
//! bytes), and its span map must be a gap-free cover that classifies holes as
//! `ParamValue`/`ForeignText` and literal prose as `TemplateLiteral`.

use dorc_core::catalog::{
    CATALOG, TemplatePart, fill_template, fill_template_tagged, is_foreign_param, parse_template,
};
use dorc_core::tagged::{Field, Region, Span};

/// Fill `template` through the tagged twin with an empty offset base, returning
/// the bytes and the span map (the standalone shape the corpus sweep uses).
fn tag(template: &str, params: &[(&'static str, &str)]) -> (String, Vec<Span>) {
    let mut out = String::new();
    let mut spans = Vec::new();
    assert_eq!(
        fill_template_tagged(
            &mut out,
            &mut spans,
            template,
            params,
            "the-code",
            Field::Message,
            0,
        ),
        Ok(())
    );
    (out, spans)
}

/// The spans are a gap-free, non-overlapping cover of exactly `0..len`.
fn assert_total_cover(spans: &[Span], len: usize) {
    let mut expected = 0;
    for s in spans {
        assert_eq!(s.range.start, expected, "non-contiguous span {s:?}");
        assert!(s.range.end > s.range.start, "empty span {s:?}");
        expected = s.range.end;
    }
    assert_eq!(
        expected, len,
        "spans cover {expected} bytes but the text is {len}"
    );
}

#[test]
fn fill_template_tagged_is_byte_identical_to_fill_template() {
    for e in CATALOG {
        let vals: Vec<(&'static str, String)> = e
            .params
            .iter()
            .map(|p| (*p, format!("valFor-{p}")))
            .collect();
        let refs: Vec<(&'static str, &str)> = vals.iter().map(|(k, v)| (*k, v.as_str())).collect();
        for (field, template) in [("message", e.message), ("help", e.help)] {
            let Some(template) = template else { continue };
            let (tagged_text, spans) = tag(template, &refs);
            assert_eq!(
                fill_template(template, &refs),
                Ok(tagged_text.clone()),
                "code `{}` {field}: tagged fill drifted from fill_template",
                e.slug
            );
            assert_total_cover(&spans, tagged_text.len());
        }
    }
}

#[test]
fn parse_template_returns_ordered_parts_and_literal_single_braces() {
    assert_eq!(
        parse_template("before {literal} {{name}} after"),
        Ok(vec![
            TemplatePart::Literal(String::from("before {literal} ")),
            TemplatePart::Hole(String::from("name")),
            TemplatePart::Literal(String::from(" after")),
        ]),
    );
}

#[test]
fn fill_template_tagged_classifies_holes_literals_and_foreign() {
    let (text, spans) = tag(
        "start {brace} {{name}} mid {{detail}} end",
        &[("name", "NN"), ("detail", "DD")],
    );
    assert_eq!(text, "start {brace} NN mid DD end");
    assert_total_cover(&spans, text.len());

    let kinds: Vec<&Region> = spans.iter().map(|s| &s.region).collect();
    // Regions in order: literal, ParamValue(name), literal, ForeignText(detail), literal.
    assert!(matches!(
        kinds[0],
        Region::TemplateLiteral { paragraph: 0, .. }
    ));
    assert!(matches!(kinds[1], Region::ParamValue { param: "name", .. }));
    assert!(matches!(kinds[2], Region::TemplateLiteral { .. }));
    assert!(matches!(kinds[3], Region::ForeignText { param: "detail" }));
    assert!(matches!(kinds[4], Region::TemplateLiteral { .. }));
    // The param spans point at exactly the interpolated values.
    assert_eq!(&text[spans[1].range.clone()], "NN");
    assert_eq!(&text[spans[3].range.clone()], "DD");

    assert!(is_foreign_param("detail"));
    assert!(!is_foreign_param("command"));
}

#[test]
fn fill_template_tagged_skips_empty_values_and_refuses_unknown_holes() {
    let (text, spans) = tag("a{{x}}b", &[("x", "")]);
    assert_eq!(text, "ab");
    assert_total_cover(&spans, text.len());
    assert!(
        spans
            .iter()
            .all(|s| matches!(s.region, Region::TemplateLiteral { .. })),
        "no ParamValue span for an empty hole: {spans:?}"
    );
    let mut out = String::new();
    let mut spans = Vec::new();
    assert_eq!(
        fill_template_tagged(
            &mut out,
            &mut spans,
            "a{{gap}}b",
            &[],
            "the-code",
            Field::Message,
            0,
        ),
        Err(dorc_core::catalog::TemplateRefusal::UnknownParam(
            String::from("gap")
        )),
    );
}
