//! The `dorc-receipt/1` grammar corpus: every committed vector, and the writer/reader
//! properties that make byte equality the format's equality relation.
//!
//! The vectors under `tests/vectors/` are reviewed fixtures, hand-written and hand-edited.
//! Nothing regenerates them: a corpus a tool can rewrite proves whatever the tool currently
//! does, which is the opposite of what a conformance corpus is for.

use std::path::{Path, PathBuf};

use dorc_receipt::format::{self, RefusalReason};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt};

fn vectors(kind: &str) -> Vec<(String, Vec<u8>)> {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("vectors")
        .join(kind);
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", root.display()))
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_owned();
            Some((name, std::fs::read(&path).ok()?))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    // A corpus walk that finds nothing would otherwise pass silently, so the floor is
    // non-empty rather than an exact count, which drifts as vectors are added.
    assert!(
        !out.is_empty(),
        "no {kind} vectors under {}",
        root.display()
    );
    out
}

/// Parse one plain body under whichever species its own header names.
fn parse_plain(body: &[u8], limits: &ReceiptLimits) -> Result<(), RefusalReason> {
    let text = core::str::from_utf8(body).map_err(|_| RefusalReason::IllegalByte { byte: 0 })?;
    let species = text.lines().nth(1).unwrap_or_default();
    match species {
        "species plan" => format::parse_body::<PlanReceipt, Plain>(body, limits).map(|_| ()),
        "species apply-intent" => {
            format::parse_body::<ApplyIntent, Plain>(body, limits).map(|_| ())
        }
        "species apply-outcome" => {
            format::parse_body::<ApplyOutcome, Plain>(body, limits).map(|_| ())
        }
        // A vector whose species line is itself the departure is read as the species the
        // valid corpus uses, so the mismatch is what the parser answers on.
        _ => format::parse_body::<PlanReceipt, Plain>(body, limits).map(|_| ()),
    }
}

#[test]
fn every_valid_vector_parses_and_reserializes_to_the_same_bytes() {
    // Byte equality is the format's equality relation, so a valid vector must survive the
    // round trip unchanged. A writer that normalized anything would show up right here.
    let limits = ReceiptLimits::V1;
    for (name, bytes) in vectors("valid") {
        let text = core::str::from_utf8(&bytes).expect("vectors are ASCII");
        let species = text.lines().nth(1).unwrap_or_default();
        let reserialized = match species {
            "species plan" => {
                let parsed = format::parse_body::<PlanReceipt, Plain>(&bytes, &limits)
                    .unwrap_or_else(|e| panic!("{name} did not parse: {e:?}"));
                format::serialize_skeleton::<PlanReceipt, Plain>(&parsed)
            }
            "species apply-intent" => {
                let parsed = format::parse_body::<ApplyIntent, Plain>(&bytes, &limits)
                    .unwrap_or_else(|e| panic!("{name} did not parse: {e:?}"));
                format::serialize_skeleton::<ApplyIntent, Plain>(&parsed)
            }
            "species apply-outcome" => {
                let parsed = format::parse_body::<ApplyOutcome, Plain>(&bytes, &limits)
                    .unwrap_or_else(|e| panic!("{name} did not parse: {e:?}"));
                format::serialize_skeleton::<ApplyOutcome, Plain>(&parsed)
            }
            other => panic!("{name} names an unknown species line: {other}"),
        }
        .unwrap_or_else(|e| panic!("{name} did not reserialize: {e:?}"));
        assert_eq!(
            reserialized.as_bytes(),
            bytes.as_slice(),
            "{name} did not round-trip byte-for-byte"
        );
    }
}

#[test]
fn every_invalid_vector_is_refused() {
    // One departure per vector, so a refusal here is attributable to that departure alone.
    let limits = ReceiptLimits::V1;
    for (name, bytes) in vectors("invalid") {
        let outcome = parse_plain(&bytes, &limits);
        assert!(
            outcome.is_err(),
            "{name} was accepted; the grammar admits exactly one form"
        );
    }
}

#[test]
fn the_three_species_cover_the_valid_corpus() {
    // The corpus is only a conformance corpus if it exercises every species; a corpus that
    // silently lost one would keep passing.
    let seen: Vec<String> = vectors("valid")
        .into_iter()
        .filter_map(|(_, bytes)| {
            let text = String::from_utf8(bytes).ok()?;
            text.lines().nth(1).map(str::to_owned)
        })
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

#[test]
fn a_record_refuses_an_atom_its_field_does_not_admit() {
    // The writer's own acceptance check: a record that could not be read back is refused at
    // construction rather than emitted and discovered later.
    let ok = format::SkeletonRecord::build(
        RecordKind::SolveCertification,
        vec![
            "whole-window".to_owned(),
            "yes".to_owned(),
            "no".to_owned(),
            "authored-before-contact".to_owned(),
        ],
    );
    assert!(ok.is_ok());

    let bad_token = format::SkeletonRecord::build(
        RecordKind::SolveCertification,
        vec![
            "whole-window".to_owned(),
            "true".to_owned(),
            "no".to_owned(),
            "authored-before-contact".to_owned(),
        ],
    );
    assert!(matches!(
        bad_token,
        Err(RefusalReason::FieldAtom { key: "consistent" })
    ));

    let wrong_arity = format::SkeletonRecord::build(
        RecordKind::SolveCertification,
        vec!["whole-window".to_owned()],
    );
    assert!(matches!(wrong_arity, Err(RefusalReason::FieldShape { .. })));
}

#[test]
fn a_species_refuses_a_record_kind_it_does_not_admit() {
    // An apply-outcome row inside a plan document is not a field error but a species error:
    // the kind is not part of that document at all.
    let record = format::SkeletonRecord::build(
        RecordKind::SiteOutcome,
        vec![
            "0".to_owned(),
            "0".to_owned(),
            "0".to_owned(),
            "absent".to_owned(),
            "ran".to_owned(),
            "absent".to_owned(),
            "uncollected".to_owned(),
            "uncollected".to_owned(),
            "host-influenced".to_owned(),
        ],
    )
    .expect("a well-formed site-outcome row");
    let skeleton = format::Skeleton {
        receipt_id: "a".repeat(64),
        signing_key_id: "c".repeat(64),
        encryption_key_id: None,
        records: vec![record],
    };
    assert!(matches!(
        format::serialize_skeleton::<PlanReceipt, Plain>(&skeleton),
        Err(RefusalReason::UnknownRecordKind)
    ));
}

#[test]
fn a_plain_skeleton_cannot_carry_an_encryption_provider() {
    // The plain projection's refusal is structural rather than a field check: a plain
    // document has no region, so it has no provider line to name one.
    let skeleton = format::Skeleton {
        receipt_id: "a".repeat(64),
        signing_key_id: "c".repeat(64),
        encryption_key_id: Some("d".repeat(64)),
        records: Vec::new(),
    };
    assert!(matches!(
        format::serialize_skeleton::<PlanReceipt, Plain>(&skeleton),
        Err(RefusalReason::OverlayPresence)
    ));
}

#[test]
fn the_record_bound_refuses_at_boundary_plus_one() {
    // Boundary-minus / at / plus on the declared record count, which is the one value a
    // document can use to ask for an allocation.
    let head = format!(
        "dorc-receipt/1\nspecies plan\nprojection plain\nreceipt-id {}\nsigning-key-id {}\n",
        "a".repeat(64),
        "c".repeat(64)
    );
    let row = "record 0 projection-omission species=observation count=0 reason=unminted account=authored-before-contact\n";
    let body = format!("{head}records 1\n{row}skeleton-end\n");

    let mut at = ReceiptLimits::V1;
    at.records = dorc_receipt::limits::CountLimit::of(1);
    assert!(format::parse_body::<PlanReceipt, Plain>(body.as_bytes(), &at).is_ok());

    let mut under = ReceiptLimits::V1;
    under.records = dorc_receipt::limits::CountLimit::of(0);
    assert!(matches!(
        format::parse_body::<PlanReceipt, Plain>(body.as_bytes(), &under),
        Err(RefusalReason::OverBound { what: "records" })
    ));
}

#[test]
fn a_declared_count_never_allocates_before_the_bound_is_checked() {
    // A document may declare an enormous record count; the bound is consulted before any
    // record is read, so the declaration cannot drive an allocation on its own.
    let body = format!(
        "dorc-receipt/1\nspecies plan\nprojection plain\nreceipt-id {}\nsigning-key-id {}\nrecords 18446744073709551615\nskeleton-end\n",
        "a".repeat(64),
        "c".repeat(64)
    );
    assert!(matches!(
        format::parse_body::<PlanReceipt, Plain>(body.as_bytes(), &ReceiptLimits::V1),
        Err(RefusalReason::OverBound { what: "records" })
    ));
}
