//! The compact-line prose route, end to end (`289:rul-reflow-fix-in-phase-four`).
//!
//! `282:rul-transcript-is-the-authoring-surface` says prose is authored by editing a committed
//! transcript and compiling it back. That route was broken for every finding rendered as a COMPACT
//! line (`  2:1 info [source:code] <message>`) — the shape `288` §5's lint codes use — and the
//! breakage was silent: it surfaced only as `MarkerOutsideEditableSection`, which reads like an
//! authoring mistake rather than a harness bug. Two independent causes, both in the bin's
//! transcript un-reflow:
//!
//! 1. the caret-frame gutter re-indent (`  9 | src` → ` 9 | src`, for a frame right-aligned on a
//!    wider line number) also matched a compact finding line, eating one of the renderer's own two
//!    leading spaces;
//! 2. `str::lines()` drops a trailing newline, and the edit compiler strips the baseline's final
//!    STRUCTURE component off the dirty text as an exact suffix — so the last component never
//!    matched. That one refused EVERY case edit; a compact-line case was simply the first to reach
//!    `compile_preview` at all (framed cases were byte-identical to HEAD, so the compiler was never
//!    called on them).
//!
//! This test drives the real library path over a real compact-line render, so a regression in
//! either cause fails here instead of showing up as an unexplained refusal during a prose sitting.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "fixture loader over the committed corpus; the no-panic lints guard untrusted input"
)]

use std::path::Path;

use dorc_loom::{DorcConsumer, compile_section_edits, to_editable_render};
use errorloom::Case;

/// A book whose only finding is the compact-rendered wall inventory.
const CASE: &str = "unmodeled-wall-inventory.loom";

fn case() -> Case {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../aid/tests")
        .join(CASE);
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {CASE}: {e}"));
    Case::parse(&text).unwrap_or_else(|e| panic!("parse {CASE}: {e}"))
}

/// The renderer's own bytes for the case's world — the baseline an edit is attributed against.
fn baseline(case: &Case) -> dorc_loom::DorcEditableBaseline {
    let source = case
        .sections()
        .iter()
        .find(|section| section.name() == "oracle.sh")
        .expect("the case carries its world");
    let result = dorc_lint::lint_materialized_source(
        "oracle.sh".to_owned(),
        source.content().to_owned(),
        dorc_lint::SourcePolicy {
            tools_enabled: false,
        },
    );
    DorcConsumer::new()
        .baseline_from_render(
            case,
            to_editable_render(&result.human(&dorc_aid::RenderCtx::production())),
        )
        .expect("the lint render carries editable provenance")
}

#[test]
fn an_edited_compact_line_compiles_and_re_holes_its_params() {
    let case = case();
    let baseline = baseline(&case);
    let committed = case.replay().blocks()[0].output();

    // Edit ONLY the compact line's prose, exactly as a prose sitting would.
    let edited = committed.replace("cannot analyze", "cannot see");
    assert_ne!(
        edited, committed,
        "the fixture still carries the expected fixed words"
    );

    let edits = compile_section_edits(&baseline, &edited).expect("the compact-line edit compiles");
    let edit = match edits.as_slice() {
        [edit] => edit,
        other => panic!(
            "exactly one section changed, got {}: {other:?}",
            other.len()
        ),
    };
    assert_eq!(edit.section().owner, "unmodeled-wall-inventory");
    assert_eq!(edit.section().field, "message");

    // The re-holing is the payoff: the edited sentence is a TEMPLATE again, not one world's
    // frozen instance.
    let used: Vec<&str> = edit
        .compiled()
        .used()
        .iter()
        .map(|name| name.0.as_str())
        .collect();
    assert_eq!(
        used,
        ["wall_count", "wall_word", "downstream"],
        "every rendered value re-holed, in first-use order"
    );
}

/// The trailing-newline half, isolated: dropping it refuses at the LAST structure component, which
/// is why the whole route looked like an attribution failure.
#[test]
fn a_transcript_missing_its_trailing_newline_is_refused() {
    let case = case();
    let baseline = baseline(&case);
    let committed = case.replay().blocks()[0].output();
    assert!(
        committed.ends_with('\n'),
        "the committed transcript keeps the renderer's trailing newline"
    );
    let truncated = committed.trim_end_matches('\n');
    assert!(
        compile_section_edits(&baseline, truncated).is_err(),
        "a suffix-truncated transcript must not silently compile"
    );
}
