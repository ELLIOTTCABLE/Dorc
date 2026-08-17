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

use dorc_aid::prose::ProseTier;
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
    laid_out_runs(
        said.runs(&dorc_aid::RenderCtx::production(), "harness"),
        width,
    )
}

/// The same, from runs the test minted itself — so a fixture that has to be LONG enough to wrap
/// carries its own length rather than borrowing a registry entry whose prose may be reworded.
fn laid_out_runs(runs: Vec<Run<Face>>, width: usize) -> (String, dorc_loom::DorcEditableBaseline) {
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
) -> Result<Option<ProseTier<Vec<String>>>, dorc_loom::DorcApplyRefusal> {
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
        Ok(Some(ProseTier::Slop(words))) => assert_eq!(words, vec!["loaded oracles: ", ""]),
        other => panic!("expected the re-split words, got {other:?}"),
    }
}

/// MULTI RUN, and wrapped. Several values with prose between them, laid out narrow enough that
/// weft breaks the line — so the section has to absorb the newline+pad weft minted where it
/// dropped the row's own space, and the compile-back has to collapse it again.
#[test]
fn a_multi_value_line_survives_a_wrap_and_edits_back() {
    let said = Said::words(
        "why-outcome-contrastive",
        &[
            "14|apt-get",
            "ran",
            "skipped for want of a vouched convergence check on this host",
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
        Ok(Some(ProseTier::Slop(words))) => assert_eq!(
            words,
            vec!["", " ", " INSTEAD OF ", ":"],
            "every word boundary the render stamped came back, whitespace collapsed"
        ),
        other => panic!("expected the re-split words, got {other:?}"),
    }
}

/// The registry slug the words-only fixtures below rewrite. Any real slug does — they carry their
/// own text and only need an entry for an applied edit to land on.
const WORDS_ONLY_SLUG: &str = "lint-source-analysis-diagnostics";

/// A words-only chrome component, long enough that every width below the literal wraps it.
fn words_only(text: &str) -> Vec<Run<Face>> {
    vec![dorc_aid::weave::words(
        text.to_owned(),
        WORDS_ONLY_SLUG,
        None,
    )]
}

const LONG_LINE: &str =
    "engine parse and control-flow-graph diagnostics over each file, computed with no world at all";

/// DIAGNOSIS for the reported section-per-chunk gap: it does not happen. A words-only component
/// long enough to wrap is ONE editable section at EVERY width — weft's newline+pad is pure layout
/// lying between two stretches of one row, which the bridge's absorption rule folds back into the
/// row (`aid::weave::to_render_parts`). Measured corpus-wide too: of 247 committed sections the
/// only split pair is a `{{detail}}` passthrough, which is a foreign VALUE splitting its register,
/// not a wrap.
///
/// What actually refused a reworded long line is one layer down: the added-line guard counted the
/// RENDERER's soft wrap as an authored break.
#[test]
fn a_wrapped_words_only_line_is_one_section_at_every_width() {
    for width in [30, 40, 55, 80] {
        let (text, baseline) = laid_out_runs(words_only(LONG_LINE), width);
        assert!(text.contains('\n'), "width {width} must wrap: {text:?}");
        assert_eq!(sections(&baseline), 1, "width {width}: {text:?}");
    }
}

/// The real gap, and the round-trip that closes it: a reword that MOVES the break point — here by
/// making the sentence long enough to need one more laid-out line than the render produced — is an
/// ordinary prose edit, because where a register's words wrap is the renderer's and never the
/// author's (`28H` ruling 7). It used to refuse as an added line and point at
/// `add-register … help`, a register a chrome component cannot even have.
#[test]
fn a_reword_across_the_break_point_compiles() {
    let (text, baseline) = laid_out_runs(words_only(LONG_LINE), 40);
    let tail = " whatsoever, and nothing else besides";
    let longer = format!("{LONG_LINE}{tail}");
    let rewrapped = laid_out_runs(words_only(&longer), 40).0;
    assert!(
        rewrapped.lines().count() > text.lines().count(),
        "the fixture must need an extra laid-out line or it proves nothing"
    );

    let edited = format!("{}{tail}\n", text.trim_end_matches('\n'));
    match applied(&baseline, &edited, WORDS_ONLY_SLUG) {
        Ok(Some(ProseTier::Slop(words))) => {
            assert_eq!(words, vec![longer], "the wrap never reaches storage");
        }
        other => panic!("a reword across the break must compile, got {other:?}"),
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
        Ok(Some(ProseTier::Slop(words))) => assert_eq!(words, vec!["(ask dorc why ", ")"]),
        other => panic!("expected the re-split words, got {other:?}"),
    }
}

/// The NARROW refusal that replaced the blanket one: an edit may rephrase every word around a
/// value, but moving one is not rephrasing — the values are the render's account of the world and
/// their order says which word goes where.
#[test]
fn moving_a_value_refuses_by_name() {
    let said = Said::words("why-outcome-contrastive", &["14|apt-get", "ran", "skipped"]);
    let (text, baseline) = laid_out(&said, 92);
    // `{{v1}} {{v0}}` swaps the first two values inside the section's interior.
    let edited = text.replace("14|apt-get ran", "{{v1}} {{v0}}");
    assert_ne!(edited, text, "the fixture rewrote the value order");
    match applied(&baseline, &edited, "why-outcome-contrastive") {
        Err(refusal) => {
            let dorc_loom::DorcApplyRefusal::ArrangementValueSequenceChanged {
                ref slug,
                ref expected,
                ref found,
                ..
            } = refusal
            else {
                panic!("a moved value must refuse by name, got {refusal:?}")
            };
            assert_eq!(slug, "why-outcome-contrastive");
            assert_eq!(*expected, vec!["v0", "v1", "v2"]);
            assert_ne!(found, expected, "the refusal names what moved");
            // The author sees an ordinary English word; the refusal has to say it was computed and
            // which words on the line are theirs to change.
            let explained = refusal.explain(std::path::Path::new("crates/aid/tests/x.loom"));
            assert!(
                explained.contains("computed") && explained.contains("edit are"),
                "the refusal must name the computed values AND the editable words: {explained}"
            );
            assert!(
                explained.contains("dorc-loom compile crates/aid/tests/x.loom"),
                "every refusal ends in its next command: {explained}"
            );
        }
        other => panic!("a moved value must refuse, got {other:?}"),
    }
}
