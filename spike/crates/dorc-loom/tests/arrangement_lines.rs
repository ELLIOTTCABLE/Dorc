//! The chrome-LINE edit loop: a value-interleaved line edits back out of a LAID-OUT transcript.
//!
//! The sibling `arrangement_prose` module proves the whole-PAGE loop, where the render is the
//! entry and nothing interleaves. This proves the shape that was refused until now
//! (`_w4-map-DRAFT:prop-one-section-many-fragments`, adopted as `28H` ruling 3): a line whose
//! bytes alternate registry prose and computed values is ONE editable section, and the fragment
//! series the render stamped is what re-splits an edit back into the entry's words.
//!
//! Each test drives the REAL layout engine rather than hand-building a part stream, because the
//! two things that can go wrong are both weft's doing: it tokenizes one run into many spans, and
//! at a break it drops the row's own space and mints its own newline+pad. A test over a
//! hand-built stream would prove the easy half.

#![expect(
    clippy::panic,
    reason = "fixture harness over the committed registry; the no-panic lints guard untrusted input"
)]

use dorc_aid::arrangement::OwnedWords;
use dorc_aid::said::Said;
use dorc_aid::weave::{Face, to_render_parts};
use dorc_loom::{DorcConsumer, compile_preview, to_editable_render};
use errorloom::Case;
use weft::{Document, Node, NodeKind, Paragraph, Run, render};

/// A minimal arrangement case: `baseline_from_render` reads only that the frontmatter declares an
/// arrangement, which is what says the payload inventory is empty by construction.
const SHELL: &str = "---\narrangement: cli-help-page\nwhen-used: harness.\nwhy: harness.\n---\n\
                     -- replay --\n$ dorc --help\n";

/// Lay one chrome line out at `width` and take it all the way to an editable baseline.
fn laid_out(said: &Said, width: usize) -> (String, dorc_loom::DorcEditableBaseline) {
    let runs: Vec<Run<Face>> = said.runs(&dorc_aid::RenderCtx::production(), "harness");
    let document = Document::new(vec![Node::new(NodeKind::Prose(Paragraph { runs }))]);
    let rendered = render(&document, width);
    let parts = to_render_parts(&rendered);
    assert_eq!(
        parts.text(),
        rendered.text(),
        "the bridge re-attributes bytes, it never rewrites them"
    );
    let case = Case::parse(SHELL).unwrap_or_else(|e| panic!("harness case: {e}"));
    let baseline = DorcConsumer::new()
        .baseline_from_render(&case, to_editable_render(&parts))
        .unwrap_or_else(|e| panic!("baseline: {e}"));
    (rendered.text().to_owned(), baseline)
}

/// The entry a slug resolves to after an edit compiled from `edited` is applied.
fn applied(
    baseline: &dorc_loom::DorcEditableBaseline,
    edited: &str,
    slug: &str,
) -> Result<OwnedWords, dorc_loom::DorcApplyRefusal> {
    let preview = compile_preview(baseline, edited).unwrap_or_else(|e| panic!("compile: {e:?}"));
    let mut consumer = DorcConsumer::new();
    consumer.apply_preview(&preview)?;
    let entry = consumer
        .arrangements()
        .iter()
        .find(|entry| entry.slug == slug)
        .unwrap_or_else(|| panic!("no entry for {slug}"));
    Ok(entry.words.clone())
}

fn sections(baseline: &dorc_loom::DorcEditableBaseline) -> usize {
    baseline
        .render()
        .components()
        .iter()
        .filter(|component| matches!(component, errorloom::RenderComponent::EditableSection(_)))
        .count()
}

/// SINGLE RUN. One value, a trailing empty word, no wrap: the simplest shape that the old
/// transport still refused, because the entry stored two words.
#[test]
fn a_single_value_line_edits_back_into_its_words() {
    let said = Said::words("why-receipt-oracles", &["firewall.oracle.sh"]);
    let (text, baseline) = laid_out(&said, 92);
    assert_eq!(text.trim_end(), "oracles: firewall.oracle.sh");
    assert_eq!(sections(&baseline), 1, "one line is one section");

    let edited = text.replace("oracles:", "loaded oracles:");
    match applied(&baseline, &edited, "why-receipt-oracles") {
        Ok(OwnedWords::Authored(words)) => assert_eq!(words, vec!["loaded oracles: ", ""]),
        other => panic!("expected the re-split words, got {other:?}"),
    }
}

/// MULTI RUN, and wrapped. Four values with prose between them, laid out narrow enough that weft
/// breaks the line — so the section has to absorb the newline+pad weft minted where it dropped
/// the row's own space, and the compile-back has to collapse it again.
#[test]
fn a_four_value_line_survives_a_wrap_and_edits_back() {
    let said = Said::words(
        "why-outcome-contrastive",
        &[
            "14|apt-get",
            "ran",
            "skipped",
            "no oracle vouched for its convergence on this host",
        ],
    );
    let (text, baseline) = laid_out(&said, 40);
    assert!(text.contains('\n'), "the fixture must wrap: {text:?}");
    assert_eq!(
        sections(&baseline),
        1,
        "a wrapped line is still one section"
    );
    assert!(
        text.contains("RATHER THAN"),
        "the registry words reached the render: {text:?}"
    );

    let edited = text.replace("RATHER THAN", "INSTEAD OF");
    match applied(&baseline, &edited, "why-outcome-contrastive") {
        Ok(OwnedWords::Authored(words)) => assert_eq!(
            words,
            vec!["", " ", " INSTEAD OF ", ": ", ""],
            "every word boundary the render stamped came back, whitespace collapsed"
        ),
        other => panic!("expected the re-split words, got {other:?}"),
    }
}

/// GLUED. The value abuts its closing word with no whitespace between them
/// (`28A:rul-glued-param-rehole-seam`) — the one place the bet that errorloom needs no change
/// could have broken, because the transport's anchors are single characters lifted off the text
/// beside a variable.
#[test]
fn a_value_glued_to_its_closing_word_edits_back() {
    let said = Said::words("why-item-pointer", &["webhost.sh:14"]);
    let (text, baseline) = laid_out(&said, 92);
    assert_eq!(
        text.trim_end(),
        "(dorc why webhost.sh:14)",
        "the closing paren is glued to the value"
    );
    assert_eq!(sections(&baseline), 1);

    let edited = text.replace("(dorc why ", "(ask dorc why ");
    match applied(&baseline, &edited, "why-item-pointer") {
        Ok(OwnedWords::Authored(words)) => assert_eq!(words, vec!["(ask dorc why ", ")"]),
        other => panic!("expected the re-split words, got {other:?}"),
    }
}

/// The NARROW refusal that replaced the blanket one: an edit may rephrase every word around a
/// value, but moving one is not rephrasing — the values are the render's account of the world and
/// their order says which word goes where.
#[test]
fn moving_a_value_refuses_by_name() {
    let said = Said::words(
        "why-outcome-contrastive",
        &["14|apt-get", "ran", "skipped", "nothing vouched"],
    );
    let (text, baseline) = laid_out(&said, 92);
    // `{{v1}} {{v0}}` swaps the first two values inside the section's interior.
    let edited = text.replace("14|apt-get ran", "{{v1}} {{v0}}");
    assert_ne!(edited, text, "the fixture rewrote the value order");
    match applied(&baseline, &edited, "why-outcome-contrastive") {
        Err(dorc_loom::DorcApplyRefusal::ArrangementValueSequenceChanged {
            slug,
            expected,
            found,
        }) => {
            assert_eq!(slug, "why-outcome-contrastive");
            assert_eq!(expected, vec!["v0", "v1", "v2", "v3"]);
            assert_ne!(found, expected, "the refusal names what moved");
        }
        other => panic!("a moved value must refuse, got {other:?}"),
    }
}
