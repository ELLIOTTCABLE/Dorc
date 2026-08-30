//! The receipt-owned locator: what its wire form preserves, and what it refuses.
//!
//! Every case here is about a document a reader did not write. The encoding is exercised through
//! its own round trip, and the refusals are exercised by CORRUPTING a real payload rather than by
//! hand-spelling one — a hand-spelled malformed input tests the test's idea of the format, and a
//! corrupted real one tests the format.

#![expect(
    clippy::expect_used,
    reason = "fixture helpers beside the cases, where the in-tests allowance does not reach them"
)]

use dorc_receipt::durable_locator::{
    DurableLocator, DurableStage, LocatorRefusal, RecordedStageKind, StageTextKind,
};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::rows::SourceOrdinal;

fn limits() -> ReceiptLimits {
    ReceiptLimits::V1
}

fn authored(lo: u64, hi: u64) -> DurableStage {
    DurableStage::in_source(
        RecordedStageKind::Authored,
        SourceOrdinal::of(0),
        (lo, hi),
        Vec::new(),
    )
    .expect("a forward span in a source")
}

/// THE SHAPE V1 ACTUALLY WRITES: one authored stage naming a byte range of one acquired source.
///
/// Round-tripped rather than byte-pinned, because the bytes are `rul-strawman-formats-no-compat`
/// material and re-spelling them is legal. What is NOT legal is losing the source ordinal or
/// moving the span, so those are what the assertion reads.
#[test]
fn one_authored_stage_round_trips_with_its_source_and_span() {
    let locator = DurableLocator::of(vec![authored(40, 80)], 0, &limits()).expect("a valid graph");
    let back = DurableLocator::decode(&locator.encode(), &limits()).expect("its own bytes read");
    assert_eq!(back, locator);
    assert_eq!(
        back.authored_origin(),
        Some((SourceOrdinal::of(0), (40, 80))),
        "the address a reader resolves against is the authored source and its exact range"
    );
}

/// A load act above authored bytes: two stages, one edge, and the chain reads generated-first.
///
/// The smallest graph that forces the DAG rather than a pair, and the one a `.`-reached site will
/// carry when the per-site source identity exists to build it from.
#[test]
fn a_load_act_and_the_bytes_it_named_survive_as_two_stages() {
    let loaded = DurableStage::in_source(
        RecordedStageKind::Loaded,
        SourceOrdinal::of(1),
        (10, 34),
        vec![0],
    )
    .expect("a load act citing the authored stage");
    let locator =
        DurableLocator::of(vec![authored(40, 80), loaded], 1, &limits()).expect("a valid graph");
    let back = DurableLocator::decode(&locator.encode(), &limits()).expect("its own bytes read");

    assert_eq!(
        back.chain(),
        vec![1, 0],
        "the load act first, its bytes behind"
    );
    assert_eq!(
        back.authored_origin(),
        Some((SourceOrdinal::of(0), (40, 80))),
        "an address resolves to the AUTHORED bytes, never to the `.` that pulled them in"
    );
}

/// Free bytes round-trip exactly, including a newline — which is what the length prefix buys.
///
/// An artifact label carrying the encoding's own separator is the case that would break any
/// line-oriented reader, and generated labels are engine-supplied strings nobody has promised are
/// newline-free.
#[test]
fn a_stage_text_carrying_the_separator_round_trips_byte_exact() {
    let hostile = b"deps/\na\xffb.sh".to_vec();
    let copied =
        DurableStage::in_artifact(RecordedStageKind::Copied, hostile.clone(), (0, 10), vec![0])
            .expect("a copied range of a generated artifact");
    let locator =
        DurableLocator::of(vec![authored(0, 10), copied], 1, &limits()).expect("a valid graph");
    let back = DurableLocator::decode(&locator.encode(), &limits()).expect("its own bytes read");

    let stage = back.stage(1).expect("the copied stage");
    assert_eq!(
        stage.text(),
        hostile.as_slice(),
        "not escaped, not transcoded"
    );
    assert_eq!(stage.text_kind(), StageTextKind::Artifact);
}

/// A FORWARD origin is refused, which is what makes acyclicity structural.
///
/// The live locator can only cite ids it already minted, so a payload citing a later stage did not
/// come from one. Admitting it would admit a cycle, and every walk over this graph would then have
/// to defend itself against one.
#[test]
fn an_origin_pointing_forward_is_refused() {
    let forward = DurableStage::in_source(
        RecordedStageKind::Authored,
        SourceOrdinal::of(0),
        (0, 1),
        vec![1],
    )
    .expect("the stage builds; the graph is what refuses");
    assert_eq!(
        DurableLocator::of(vec![forward, authored(2, 3)], 0, &limits()).unwrap_err(),
        LocatorRefusal::Origin
    );
}

/// A head naming no stage is refused, rather than resolving to an empty chain.
#[test]
fn a_head_past_the_end_is_refused() {
    assert_eq!(
        DurableLocator::of(vec![authored(0, 1)], 7, &limits()).unwrap_err(),
        LocatorRefusal::Head
    );
    assert_eq!(
        DurableLocator::of(Vec::new(), 0, &limits()).unwrap_err(),
        LocatorRefusal::Head,
        "an empty graph has no head to name"
    );
}

/// A stage may not carry an identity its kind does not take, in either direction.
///
/// The two halves are separate mistakes: an authored stage without a source names no document, and
/// a generated one WITH a source claims the engine wrote bytes into a file the controller read.
#[test]
fn a_stage_shape_that_contradicts_its_kind_is_refused() {
    assert_eq!(
        DurableStage::in_source(
            RecordedStageKind::Generated,
            SourceOrdinal::of(0),
            (0, 1),
            Vec::new()
        )
        .unwrap_err(),
        LocatorRefusal::StageShape,
        "generated scaffolding descends from no source the controller read"
    );
    assert_eq!(
        DurableStage::in_artifact(
            RecordedStageKind::Authored,
            b"x".to_vec(),
            (0, 1),
            Vec::new()
        )
        .unwrap_err(),
        LocatorRefusal::StageShape,
        "authored bytes live in a source, not in an artifact"
    );
}

/// A backwards span is refused at the constructor, before a graph can hold one.
#[test]
fn a_backwards_span_is_refused() {
    assert_eq!(
        DurableStage::in_source(
            RecordedStageKind::Authored,
            SourceOrdinal::of(0),
            (80, 40),
            Vec::new()
        )
        .unwrap_err(),
        LocatorRefusal::Span
    );
}

/// Corrupting a real payload refuses rather than panicking, at every byte position.
///
/// The decoder reads bytes a document carried, so the property that matters is TOTALITY: whatever
/// a truncation or a flipped byte produces, it is an answer and not a crash. Exhaustive over
/// truncation length and over one flipped byte per position, which is cheap at this size and
/// catches the indexing mistakes a hand-picked case would miss.
#[test]
fn every_truncation_and_byte_flip_answers_rather_than_panicking() {
    let locator = DurableLocator::of(vec![authored(40, 80)], 0, &limits()).expect("a valid graph");
    let bytes = locator.encode();

    for cut in 0..bytes.len() {
        let truncated = &bytes[..cut];
        assert!(
            DurableLocator::decode(truncated, &limits()).is_err(),
            "a payload cut at {cut} is not a shorter graph"
        );
    }
    for position in 0..bytes.len() {
        let mut flipped = bytes.clone();
        flipped[position] ^= 0xff;
        // The answer may legitimately be Ok — flipping a digit inside a span still parses — so
        // what is asserted is that decode RETURNED, which a panic would not have done.
        let _ = DurableLocator::decode(&flipped, &limits());
    }
}

/// Trailing bytes after the closing token are refused, so a payload cannot smuggle a tail.
#[test]
fn trailing_bytes_after_the_close_are_refused() {
    let locator = DurableLocator::of(vec![authored(0, 4)], 0, &limits()).expect("a valid graph");
    let mut bytes = locator.encode();
    bytes.extend_from_slice(b"stage authored 0 0 4 none 0 -\n\n");
    assert_eq!(
        DurableLocator::decode(&bytes, &limits()).unwrap_err(),
        LocatorRefusal::Count
    );
}

/// An unrecognized version line is refused before anything else is read.
#[test]
fn an_unknown_version_is_refused() {
    let locator = DurableLocator::of(vec![authored(0, 4)], 0, &limits()).expect("a valid graph");
    let bytes = locator.encode();
    let mut future = b"dorc-receipt-locator/2\n".to_vec();
    future.extend_from_slice(
        bytes
            .split(|byte| *byte == b'\n')
            .skip(1)
            .flat_map(|line| [line, b"\n"].concat())
            .collect::<Vec<u8>>()
            .as_slice(),
    );
    assert_eq!(
        DurableLocator::decode(&future, &limits()).unwrap_err(),
        LocatorRefusal::Version
    );
}

/// A graph over the stage bound is refused rather than allocated.
#[test]
fn a_graph_past_the_stage_bound_is_refused() {
    let mut narrow = ReceiptLimits::V1;
    narrow.locator_stages = dorc_receipt::limits::CountLimit::of(1);
    let two = vec![
        authored(0, 1),
        DurableStage::in_source(
            RecordedStageKind::Loaded,
            SourceOrdinal::of(0),
            (2, 3),
            vec![0],
        )
        .expect("a second stage"),
    ];
    assert_eq!(
        DurableLocator::of(two, 1, &narrow).unwrap_err(),
        LocatorRefusal::OverLimit
    );
}
