//! The `dorc-receipt/1` grammar corpus: every committed vector, and the writer/reader
//! properties that make byte equality the format's equality relation.
//!
//! The vectors under `tests/vectors/` are reviewed fixtures, hand-written and hand-edited.
//! Nothing regenerates them: a corpus a tool can rewrite proves whatever the tool currently
//! does, which is the opposite of what a conformance corpus is for.
//!
//! Failures are collected and asserted together, so one run names every vector that moved
//! rather than stopping at the first.

use std::path::{Path, PathBuf};

use dorc_receipt::format::{self, RefusalReason};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::limits::{CountLimit, ReceiptLimits};
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Rich};

/// Vectors of one shape. The corpus holds two: `.skeleton` files are bare skeleton spans, and
/// `.receipt` files are whole signed documents. Feeding one to the other's reader proves
/// nothing, so the split is by extension rather than by directory.
fn vectors_named(kind: &str, extension: &str) -> Vec<(String, Vec<u8>)> {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("vectors")
        .join(kind);
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_owned();
            if !name.contains(extension) {
                return None;
            }
            Some((name, std::fs::read(&path).ok()?))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    // A corpus walk that finds nothing would otherwise pass silently, so the floor is
    // non-empty rather than an exact count, which drifts as vectors are added.
    assert!(
        !out.is_empty(),
        "no {kind} {extension} vectors under {}",
        root.display()
    );
    out
}

/// Bare skeleton spans.
fn vectors(kind: &str) -> Vec<(String, Vec<u8>)> {
    vectors_named(kind, ".skeleton")
}

/// Whole signed documents.
fn documents() -> Vec<(String, Vec<u8>)> {
    vectors_named("valid", ".receipt")
}

/// The species line of a document, as spelled.
fn species_line(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .lines()
        .nth(1)
        .unwrap_or_default()
        .to_owned()
}

/// Parse one plain body under whichever species its own header names. A vector whose species
/// line is itself the departure is read as the species the valid corpus uses, so the mismatch
/// is what the parser answers on.
fn parse_plain(body: &[u8], limits: &ReceiptLimits) -> Result<(), RefusalReason> {
    match species_line(body).as_str() {
        "species apply-intent" => {
            format::parse_skeleton_span::<ApplyIntent, Plain>(body, limits).map(|_| ())
        }
        "species apply-outcome" => {
            format::parse_skeleton_span::<ApplyOutcome, Plain>(body, limits).map(|_| ())
        }
        _ => format::parse_skeleton_span::<PlanReceipt, Plain>(body, limits).map(|_| ()),
    }
}

/// Parse then reserialize under the species the document names.
fn round_trip(body: &[u8], limits: &ReceiptLimits) -> Option<Result<String, RefusalReason>> {
    match species_line(body).as_str() {
        "species plan" => Some(
            format::parse_skeleton_span::<PlanReceipt, Plain>(body, limits)
                .and_then(|parsed| format::serialize_skeleton::<PlanReceipt, Plain>(&parsed)),
        ),
        "species apply-intent" => Some(
            format::parse_skeleton_span::<ApplyIntent, Plain>(body, limits)
                .and_then(|parsed| format::serialize_skeleton::<ApplyIntent, Plain>(&parsed)),
        ),
        "species apply-outcome" => Some(
            format::parse_skeleton_span::<ApplyOutcome, Plain>(body, limits)
                .and_then(|parsed| format::serialize_skeleton::<ApplyOutcome, Plain>(&parsed)),
        ),
        _ => None,
    }
}

#[test]
fn every_valid_vector_parses_and_reserializes_to_the_same_bytes() {
    // Byte equality is the format's equality relation, so a valid vector must survive the
    // round trip unchanged. A writer that normalized anything would show up right here.
    let limits = ReceiptLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    for (name, bytes) in vectors("valid") {
        match round_trip(&bytes, &limits) {
            Some(Ok(text)) if text.as_bytes() == bytes.as_slice() => {}
            Some(Ok(_)) => failures.push(format!("{name}: did not round-trip byte-for-byte")),
            Some(Err(reason)) => failures.push(format!("{name}: {reason:?}")),
            None => failures.push(format!("{name}: names no known species")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// A vector is a skeleton span; make it a whole document by appending a well-shaped trailer.
///
/// The trailer is syntactically valid and cryptographically meaningless, which is all the
/// locator needs: it checks shape and never checks a signature.
fn as_document(span: &[u8]) -> Vec<u8> {
    let mut out = span.to_vec();
    out.extend_from_slice(b"signature ");
    out.extend_from_slice("0".repeat(128).as_bytes());
    out.push(b'\n');
    out
}

/// Drive a vector through the real read order: locate, then parse the located skeleton span.
///
/// Routing through the locator is what exercises the byte-level guards. Those live in
/// `locate` alone, so a test that called the parser directly would never reach them and a
/// vector named for a line-ending departure would be refused — if at all — for some other
/// reason entirely.
fn locate_then_parse(span: &[u8], limits: &ReceiptLimits) -> Result<(), RefusalReason> {
    let document = as_document(span);
    let located = format::locate(&document, limits)?;
    parse_plain(&located.skeleton, limits)
}
#[test]
fn a_byte_level_departure_is_refused_by_the_locator_and_named_for_what_it_is() {
    // The point of these two vectors is the byte, so the refusal has to name the byte. Both
    // would otherwise be refused further in for an incidental reason — a stray carriage
    // return also derails the version line — and the vector would then be passing for a
    // reason other than the one it was written to prove.
    let limits = ReceiptLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    for (name, span) in vectors("invalid") {
        let want = match name.as_str() {
            "carriage-returns.skeleton.crlf" => b'\r',
            "tab-separator.skeleton" => b'\t',
            _ => continue,
        };
        match format::locate(&as_document(&span), &limits) {
            Err(RefusalReason::IllegalByte { byte }) if byte == want => {}
            other => failures.push(format!(
                "{name}: wanted IllegalByte {want:?}, got {other:?}"
            )),
        }
    }
    assert_eq!(failures.len(), 0, "{failures:#?}");
    assert!(
        vectors("invalid")
            .iter()
            .any(|(name, _)| name == "carriage-returns.skeleton.crlf"),
        "the wrong-line-ending vector is what keeps the locator's byte guard honest"
    );
}

#[test]
fn the_three_species_cover_the_valid_corpus() {
    // The corpus is only a conformance corpus if it exercises every species; one that
    // silently lost a species would keep passing.
    let seen: Vec<String> = vectors("valid")
        .into_iter()
        .map(|(_, bytes)| species_line(&bytes))
        .collect();
    for species in [
        "species plan",
        "species apply-intent",
        "species apply-outcome",
    ] {
        assert!(
            seen.iter().any(|line| line == species),
            "no {species} vector"
        );
    }
}

fn build(kind: RecordKind, atoms: &[&str]) -> Result<format::SkeletonRecord, RefusalReason> {
    format::SkeletonRecord::build(kind, atoms.iter().map(|a| (*a).to_owned()).collect())
}

fn skeleton_of(
    records: Vec<format::SkeletonRecord>,
    encryption: Option<String>,
) -> format::Skeleton {
    format::Skeleton {
        receipt_id: "a".repeat(64),
        signing_key_id: "c".repeat(64),
        encryption_key_id: encryption,
        records,
    }
}

#[test]
fn a_record_refuses_an_atom_its_field_does_not_admit() {
    // The writer's own acceptance check: a record that could not be read back is refused at
    // construction rather than emitted and discovered later.
    let good = ["whole-window", "yes", "no", "authored-before-contact"];
    assert!(build(RecordKind::SolveCertification, &good).is_ok());

    let bad_token = ["whole-window", "true", "no", "authored-before-contact"];
    assert!(matches!(
        build(RecordKind::SolveCertification, &bad_token),
        Err(RefusalReason::FieldAtom { key: "consistent" })
    ));

    assert!(matches!(
        build(RecordKind::SolveCertification, &["whole-window"]),
        Err(RefusalReason::FieldShape { .. })
    ));
}

#[test]
fn a_species_refuses_a_record_kind_it_does_not_admit() {
    // An apply-outcome row inside a plan document is not a field error but a species error:
    // the kind is not part of that document at all.
    let row = build(
        RecordKind::SiteOutcome,
        &[
            "0",
            "0",
            "0",
            "absent",
            "ran",
            "absent",
            "uncollected",
            "uncollected",
            "host-influenced",
        ],
    );
    let Ok(row) = row else {
        assert!(row.is_ok(), "a well-formed site-outcome row: {row:?}");
        return;
    };
    assert!(matches!(
        format::serialize_skeleton::<PlanReceipt, Plain>(&skeleton_of(vec![row], None)),
        Err(RefusalReason::UnknownRecordKind)
    ));
}

#[test]
fn a_plain_skeleton_cannot_carry_an_encryption_provider() {
    // The plain projection's refusal is structural rather than a field check: a plain
    // document has no region, so it has no provider line to name one.
    let skeleton = skeleton_of(Vec::new(), Some("d".repeat(64)));
    assert!(matches!(
        format::serialize_skeleton::<PlanReceipt, Plain>(&skeleton),
        Err(RefusalReason::OverlayPresence)
    ));
}

/// A one-record plain plan body, spelled literally so the bound tests own their input.
fn one_record_body(count: &str) -> String {
    format!(
        "dorc-receipt/1\nspecies plan\nprojection plain\nreceipt-id {}\nsigning-key-id {}\nrecords {count}\nrecord 0 projection-omission species=observation count=0 reason=unminted account=authored-before-contact\nskeleton-end\n",
        "a".repeat(64),
        "c".repeat(64)
    )
}

#[test]
fn the_record_bound_refuses_at_boundary_plus_one() {
    // Boundary-minus / at / plus on the declared record count, which is the one value a
    // document can use to ask for an allocation.
    let body = one_record_body("1");

    let mut at = ReceiptLimits::V1;
    at.records = CountLimit::of(1);
    assert!(format::parse_skeleton_span::<PlanReceipt, Plain>(body.as_bytes(), &at).is_ok());

    let mut under = ReceiptLimits::V1;
    under.records = CountLimit::of(0);
    assert!(matches!(
        format::parse_skeleton_span::<PlanReceipt, Plain>(body.as_bytes(), &under),
        Err(RefusalReason::OverBound { what: "records" })
    ));
}

#[test]
fn a_declared_count_never_allocates_before_the_bound_is_checked() {
    // A document may declare an enormous record count; the bound is consulted before any
    // record is read, so the declaration cannot drive an allocation on its own.
    let body = one_record_body("18446744073709551615");
    assert!(matches!(
        format::parse_skeleton_span::<PlanReceipt, Plain>(body.as_bytes(), &ReceiptLimits::V1),
        Err(RefusalReason::OverBound { what: "records" })
    ));
}

#[test]
fn a_rich_skeleton_parses_from_its_own_span_and_not_from_the_whole_signed_body() {
    // The signed body of a rich document is the skeleton span followed by the region. Only
    // the first of those is a skeleton, and handing the parser the whole body asks it to read
    // the region as records. This pins which span the reader takes: the two are the same
    // bytes for plain, so nothing else in the corpus can tell the difference, and the
    // substitution would go unnoticed until a rich document existed to fail on it.
    let limits = ReceiptLimits::V1;
    let Ok(row) = build(
        RecordKind::SolveCertification,
        &["whole-window", "yes", "no", "authored-before-contact"],
    ) else {
        panic!("the fixture row is well formed");
    };
    let skeleton = skeleton_of(vec![row], Some("d".repeat(64)));
    let Ok(span) = format::serialize_skeleton::<PlanReceipt, Rich>(&skeleton) else {
        panic!("the fixture skeleton serializes");
    };

    assert!(
        format::parse_skeleton_span::<PlanReceipt, Rich>(span.as_bytes(), &limits).is_ok(),
        "the skeleton span is what parses"
    );

    let body = format::signed_body(&span, Some("-----BEGIN AGE ENCRYPTED FILE-----"));
    assert!(
        body.len() > span.len(),
        "a rich body extends past its skeleton"
    );
    assert!(
        format::parse_skeleton_span::<PlanReceipt, Rich>(&body, &limits).is_err(),
        "the whole signed body is not a skeleton and must not parse as one"
    );
}

/// Every invalid vector, bound to the exact refusal its own departure must produce.
///
/// Binding the reason, not merely "it was refused", is what keeps a negative vector honest.
/// Several departures here fail closed in ways that are indistinguishable from the outside —
/// a wrong count and a stray line both stop the same parse — so a vector that drifted onto a
/// neighbouring reason would keep passing while testing nothing.
const EXPECTED: &[(&str, RefusalReason)] = &[
    (
        "bad-optional-token.skeleton",
        RefusalReason::FieldAtom { key: "started" },
    ),
    (
        "blank-line.skeleton",
        RefusalReason::Structure {
            what: "skeleton-end",
        },
    ),
    (
        "bytes-after-terminator.skeleton",
        RefusalReason::SignatureShape,
    ),
    (
        "carriage-returns.skeleton.crlf",
        RefusalReason::IllegalByte { byte: b'\r' },
    ),
    (
        "extra-field.skeleton",
        RefusalReason::FieldAtom { key: "account" },
    ),
    (
        "kind-not-in-species.skeleton",
        RefusalReason::UnknownRecordKind,
    ),
    ("leading-zero.skeleton", RefusalReason::RecordCount),
    (
        "missing-field.skeleton",
        RefusalReason::FieldShape { kind: "invocation" },
    ),
    (
        "missing-final-newline.skeleton",
        RefusalReason::Structure {
            what: "skeleton-end",
        },
    ),
    (
        "negative-integer.skeleton",
        RefusalReason::FieldAtom { key: "bytes" },
    ),
    (
        "no-terminator.skeleton",
        RefusalReason::Structure {
            what: "skeleton-end",
        },
    ),
    (
        "noncontiguous-record-id.skeleton",
        RefusalReason::RecordIdentity,
    ),
    (
        "projection-mismatch.skeleton",
        RefusalReason::DomainMismatch,
    ),
    ("record-count-too-high.skeleton", RefusalReason::RecordCount),
    ("record-count-too-low.skeleton", RefusalReason::RecordCount),
    (
        "reordered-fields.skeleton",
        RefusalReason::FieldShape { kind: "invocation" },
    ),
    (
        "short-digest.skeleton",
        RefusalReason::FieldAtom { key: "digest" },
    ),
    (
        "species-mismatch.skeleton",
        RefusalReason::UnknownRecordKind,
    ),
    (
        "tab-separator.skeleton",
        RefusalReason::IllegalByte { byte: b'\t' },
    ),
    (
        "trailing-space.skeleton",
        RefusalReason::Structure { what: "records" },
    ),
    ("unknown-kind.skeleton", RefusalReason::UnknownRecordKind),
    (
        "unsupported-version.skeleton",
        RefusalReason::UnsupportedVersion,
    ),
    (
        "wrong-token-case.skeleton",
        RefusalReason::FieldAtom { key: "account" },
    ),
];

#[test]
fn every_invalid_vector_is_refused_for_exactly_its_own_departure() {
    // Total in both directions: a vector with no row is unaccounted for, and a row with no
    // vector is a table that outlived the file it describes.
    let limits = ReceiptLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    let present: Vec<String> = vectors("invalid")
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    for (name, _) in EXPECTED {
        if !present.iter().any(|have| have == name) {
            failures.push(format!(
                "{name}: named by the table, absent from the corpus"
            ));
        }
    }
    for (name, bytes) in vectors("invalid") {
        let Some((_, want)) = EXPECTED.iter().find(|(row, _)| *row == name) else {
            failures.push(format!("{name}: in the corpus, absent from the table"));
            continue;
        };
        match locate_then_parse(&bytes, &limits) {
            Err(got) if got == *want => {}
            other => failures.push(format!("{name}: wanted {want:?}, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn a_nested_refusal_reaches_the_document_vocabulary_without_losing_its_detail() {
    // Three refusal families meet at one document-level vocabulary: the skeleton grammar's own
    // arms, the region's, and the image's. The two nested ones are carried whole rather than
    // flattened to a generic arm, because the sentence a reader needs — which slot, which
    // path, which bound — lives in the inner value and cannot be recovered once it is dropped.
    let overlay = RefusalReason::Overlay(dorc_receipt::overlay::OverlayFault::DuplicateKey);
    let image = RefusalReason::Image(dorc_receipt::image::ImageRefusal::SecondStream);
    assert_ne!(overlay, image, "the two families are distinguishable");

    match &image {
        RefusalReason::Image(inner) => assert_eq!(
            inner,
            &dorc_receipt::image::ImageRefusal::SecondStream,
            "the inner refusal survives the widening"
        ),
        other => panic!("the image arm did not carry its own value: {other:?}"),
    }

    // A document-level arm is not interchangeable with a nested one naming the same idea: an
    // over-bound skeleton and an over-bound image are different objects being refused.
    assert_ne!(
        RefusalReason::OverBound {
            what: "skeleton-bytes"
        },
        RefusalReason::Image(dorc_receipt::image::ImageRefusal::OverBound {
            what: "image-bytes"
        })
    );
}

#[test]
fn the_two_vector_shapes_partition_the_valid_corpus() {
    // A vector that matched neither loader would be committed, walked past, and prove nothing;
    // one that matched both would be read twice under two incompatible readers. Neither
    // failure announces itself, so the partition is asserted rather than assumed.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("vectors")
        .join("valid");
    let all: Vec<String> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| Some(entry.path().file_name()?.to_str()?.to_owned()))
        .collect();
    assert!(!all.is_empty(), "no valid vectors at all");

    let spans: Vec<String> = vectors("valid").into_iter().map(|(n, _)| n).collect();
    let docs: Vec<String> = documents().into_iter().map(|(n, _)| n).collect();
    for name in &all {
        let in_spans = spans.contains(name);
        let in_docs = docs.contains(name);
        assert!(in_spans || in_docs, "{name}: matched neither loader");
        assert!(!(in_spans && in_docs), "{name}: matched both loaders");
    }
}

#[test]
fn every_committed_rich_document_locates_and_reassembles_to_its_own_bytes() {
    // The frozen half of the corpus. Encryption is not reproducible, so a rich document cannot
    // be regenerated byte-for-byte and the writer cannot be asked to reproduce it. What can be
    // asserted is that the reader takes it apart into spans that put it back together exactly:
    // that pins the framing, the span arithmetic and the armor shape against a writer and a
    // reader drifting together, which is the one thing an in-process round trip cannot catch.
    let limits = ReceiptLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    let mut seen: Vec<String> = Vec::new();

    for (name, bytes) in documents() {
        seen.push(species_line(&bytes));
        let located = match format::locate(&bytes, &limits) {
            Ok(located) => located,
            Err(reason) => {
                failures.push(format!("{name}: did not locate: {reason:?}"));
                continue;
            }
        };
        let Some(armor) = located.armor.as_deref() else {
            failures.push(format!("{name}: carries no region"));
            continue;
        };
        if let Err(reason) = format::check_armor_shape(armor) {
            failures.push(format!("{name}: region shape: {reason:?}"));
        }
        let skeleton = String::from_utf8_lossy(&located.skeleton).into_owned();
        let rebuilt = format::assemble(&skeleton, Some(armor), &located.signature_hex);
        if rebuilt != bytes {
            failures.push(format!("{name}: did not reassemble to its own bytes"));
        }
        if format::signed_body(&skeleton, Some(armor)) != located.body {
            failures.push(format!(
                "{name}: the signed span is not skeleton plus region"
            ));
        }
    }

    for species in [
        "species plan",
        "species apply-intent",
        "species apply-outcome",
    ] {
        assert!(
            seen.iter().any(|line| line == species),
            "no rich {species} document"
        );
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
