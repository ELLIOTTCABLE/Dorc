//! The reverse-overlay validator, exercised over inert bytes.
//!
//! No encryption appears here. The validator is a pure function of the region's plaintext,
//! the skeleton it claims to enrich, and the bounds policy, so the whole failure family is
//! reachable by hand-authoring plaintext — which is why it is built and proved before
//! anything seals a region.

#![expect(
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{PlanReceipt, Rich, Species};
use dorc_receipt::overlay::{
    DecryptedOpaqueOverlay, OverlayEntry, OverlayFault, ValidatedOpaqueOverlay, captured_slots,
    serialize,
};
use dorc_receipt::projection::OpaqueFieldTag;

const RECEIPT: &str = "aa11bb22cc33dd44ee55ff66aa77bb88cc99dd00ee11ff22aa33bb44cc55dd66";

fn record(kind: RecordKind, atoms: &[&str]) -> SkeletonRecord {
    SkeletonRecord::build(kind, atoms.iter().map(|a| (*a).to_owned()).collect()).unwrap()
}

/// A skeleton with three captured slots across two records, and one record with none.
fn skeleton() -> Skeleton {
    Skeleton {
        receipt_id: RECEIPT.to_owned(),
        signing_key_id: "c".repeat(64),
        encryption_key_id: Some("d".repeat(64)),
        records: vec![
            record(
                RecordKind::Invocation,
                &[
                    "plan",
                    "absent",
                    "captured",
                    "withheld-plain",
                    "0",
                    "authored-before-contact",
                ],
            ),
            record(
                RecordKind::Source,
                &[
                    "0",
                    "book",
                    &"e".repeat(64),
                    "12",
                    "captured",
                    "captured",
                    "authored-before-contact",
                ],
            ),
            record(
                RecordKind::SolveCertification,
                &["whole-window", "yes", "no", "authored-before-contact"],
            ),
        ],
    }
}

fn span() -> Vec<u8> {
    dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&skeleton())
        .unwrap()
        .into_bytes()
}

fn entries() -> Vec<OverlayEntry> {
    vec![
        OverlayEntry::of(0, OpaqueFieldTag::Argv, b"dorc plan book.sh".to_vec()),
        OverlayEntry::of(1, OpaqueFieldTag::SourcePath, b"/etc/book.sh".to_vec()),
        OverlayEntry::of(1, OpaqueFieldTag::SourceExcerpt, b"set -eu\n".to_vec()),
    ]
}

fn good() -> Vec<u8> {
    serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &entries())
}

fn validate(bytes: Vec<u8>) -> Result<ValidatedOpaqueOverlay, OverlayFault> {
    DecryptedOpaqueOverlay::of(bytes).validate(
        &skeleton(),
        &span(),
        PlanReceipt::TOKEN,
        &ReceiptLimits::V1,
    )
}

/// Replace the first line starting with `prefix`.
fn replace_line(bytes: &[u8], prefix: &str, with: &str) -> Vec<u8> {
    let text = String::from_utf8_lossy(bytes).into_owned();
    let mut out = String::new();
    let mut done = false;
    for line in text.split_inclusive('\n') {
        if !done && line.starts_with(prefix) {
            out.push_str(with);
            out.push('\n');
            done = true;
        } else {
            out.push_str(line);
        }
    }
    assert!(done, "no line began with {prefix}");
    out.into_bytes()
}

#[test]
fn a_canonical_region_validates_and_yields_exactly_what_was_put_in() {
    let validated = validate(good()).expect("the canonical region validates");
    assert_eq!(
        validated.value(0, OpaqueFieldTag::Argv),
        Some(b"dorc plan book.sh".as_slice())
    );
    assert_eq!(
        validated.value(1, OpaqueFieldTag::SourceExcerpt),
        Some(b"set -eu\n".as_slice()),
        "a payload containing a newline frames on its declared length, not on the newline"
    );
    assert_eq!(
        validated.value(0, OpaqueFieldTag::TargetName),
        None,
        "a slot the skeleton withheld has no value to read"
    );
    assert_eq!(validated.entries().len(), 3);
}

#[test]
fn the_captured_account_is_computed_from_the_skeleton_alone() {
    // The skeleton is what says which slots exist. If this account could be influenced by the
    // region, the region could describe the set it satisfies.
    assert_eq!(
        captured_slots(&skeleton()),
        vec![
            (0, OpaqueFieldTag::Argv),
            (1, OpaqueFieldTag::SourcePath),
            (1, OpaqueFieldTag::SourceExcerpt),
        ]
    );
}

#[test]
fn a_region_naming_another_document_or_species_releases_nothing() {
    let other = serialize(&"9".repeat(64), PlanReceipt::TOKEN, &span(), &entries());
    assert_eq!(validate(other), Err(OverlayFault::DocumentMismatch));

    let wrong_species = serialize(RECEIPT, "apply-intent", &span(), &entries());
    assert_eq!(validate(wrong_species), Err(OverlayFault::DocumentMismatch));

    let wrong_projection = replace_line(&good(), "projection ", "projection plain");
    assert_eq!(
        validate(wrong_projection),
        Err(OverlayFault::DocumentMismatch)
    );
}

#[test]
fn a_region_bound_to_a_different_skeleton_releases_nothing() {
    // The digest is what stops a region validating against a skeleton it was not written for,
    // which is the case the outer signature alone cannot distinguish.
    let elsewhere = serialize(RECEIPT, PlanReceipt::TOKEN, b"some other span", &entries());
    assert_eq!(
        validate(elsewhere),
        Err(OverlayFault::SkeletonDigestMismatch)
    );
}

#[test]
fn a_missing_entry_releases_nothing_rather_than_enriching_partially() {
    let short: Vec<OverlayEntry> = entries().into_iter().take(2).collect();
    let region = serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &short);
    assert_eq!(validate(region), Err(OverlayFault::MissingRequired));
}

#[test]
fn an_entry_the_skeleton_does_not_account_for_is_refused() {
    // Record 0's target says withheld-plain, so a value for it is a claim the skeleton does
    // not make.
    let mut extra = entries();
    extra.push(OverlayEntry::of(
        0,
        OpaqueFieldTag::TargetName,
        b"web1".to_vec(),
    ));
    let region = serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &extra);
    assert_eq!(validate(region), Err(OverlayFault::Unaccounted));
}

#[test]
fn a_duplicate_key_is_refused_rather_than_letting_the_second_win() {
    let mut aliased = entries();
    aliased.push(OverlayEntry::of(
        0,
        OpaqueFieldTag::Argv,
        b"something else".to_vec(),
    ));
    let region = serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &aliased);
    assert_eq!(validate(region), Err(OverlayFault::DuplicateKey));
}

#[test]
fn an_entry_naming_a_record_or_field_that_is_not_there_is_refused() {
    let mut dangling = entries();
    dangling.push(OverlayEntry::of(9, OpaqueFieldTag::Argv, b"x".to_vec()));
    assert_eq!(
        validate(serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &dangling)),
        Err(OverlayFault::DanglingRecord)
    );

    let mut wrong_field = entries();
    wrong_field.push(OverlayEntry::of(2, OpaqueFieldTag::Stdout, b"x".to_vec()));
    assert_eq!(
        validate(serialize(
            RECEIPT,
            PlanReceipt::TOKEN,
            &span(),
            &wrong_field
        )),
        Err(OverlayFault::WrongFieldForKind)
    );
}

#[test]
fn a_declared_count_that_disagrees_with_the_entries_present_is_refused() {
    let high = replace_line(&good(), "entries ", "entries 4");
    assert_eq!(validate(high), Err(OverlayFault::EntryCount));

    let low = replace_line(&good(), "entries ", "entries 2");
    assert_eq!(validate(low), Err(OverlayFault::EntryCount));
}

#[test]
fn a_payload_shorter_or_longer_than_its_declared_length_is_refused() {
    let long = replace_line(&good(), "entry 0 argv ", "entry 0 argv 999999");
    assert!(matches!(
        validate(long),
        Err(OverlayFault::EntryShape { .. })
    ));

    let short = replace_line(&good(), "entry 0 argv ", "entry 0 argv 4");
    assert!(matches!(
        validate(short),
        Err(OverlayFault::EntryShape { .. })
    ));
}

#[test]
fn bytes_after_the_terminator_are_refused_rather_than_ignored() {
    let mut trailing = good();
    trailing.extend_from_slice(b"extra\n");
    assert_eq!(validate(trailing), Err(OverlayFault::Trailing));
}

#[test]
fn a_version_line_this_reader_does_not_implement_is_refused() {
    let other = replace_line(&good(), "dorc-receipt-overlay/", "dorc-receipt-overlay/2");
    assert!(matches!(validate(other), Err(OverlayFault::Header { .. })));
}

#[test]
fn an_unknown_tag_token_is_refused_even_though_it_matches_the_tag_alphabet() {
    // Matching `[a-z][a-z0-9-]*` is not membership. A token outside the closed table names no
    // slot, so it can only ever be refused.
    let unknown = replace_line(&good(), "entry 0 argv ", "entry 0 arg-v 17");
    assert!(matches!(
        validate(unknown),
        Err(OverlayFault::EntryShape { what: "tag" })
    ));
}

#[test]
fn a_zero_length_payload_is_legal_and_distinct_from_an_absent_one() {
    let empty = vec![
        OverlayEntry::of(0, OpaqueFieldTag::Argv, Vec::new()),
        OverlayEntry::of(1, OpaqueFieldTag::SourcePath, Vec::new()),
        OverlayEntry::of(1, OpaqueFieldTag::SourceExcerpt, Vec::new()),
    ];
    let validated = validate(serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &empty))
        .expect("empty payloads are legal where the schema allows them");
    assert_eq!(
        validated.value(0, OpaqueFieldTag::Argv),
        Some(b"".as_slice())
    );
}

#[test]
fn a_region_past_a_bound_is_refused_before_it_is_read() {
    let mut narrow = ReceiptLimits::V1;
    narrow.overlay_entries = dorc_receipt::limits::CountLimit::of(2);
    let refused = DecryptedOpaqueOverlay::of(good()).validate(
        &skeleton(),
        &span(),
        PlanReceipt::TOKEN,
        &narrow,
    );
    assert_eq!(
        refused,
        Err(OverlayFault::OverBound {
            what: "overlay-entries"
        })
    );

    let mut tiny = ReceiptLimits::V1;
    tiny.overlay_bytes = dorc_receipt::limits::ByteLimit::of(8);
    assert_eq!(
        DecryptedOpaqueOverlay::of(good()).validate(
            &skeleton(),
            &span(),
            PlanReceipt::TOKEN,
            &tiny
        ),
        Err(OverlayFault::OverBound {
            what: "overlay-bytes"
        })
    );
}

#[test]
fn the_canonical_serializer_orders_entries_however_they_were_supplied() {
    let mut shuffled = entries();
    shuffled.reverse();
    assert_eq!(
        serialize(RECEIPT, PlanReceipt::TOKEN, &span(), &shuffled),
        good(),
        "one form, whatever order a caller happened to build in"
    );
}

/// Emit a region with entries in exactly the order given, bypassing the canonical sort.
///
/// The framing is written here rather than recovered by splitting a canonical region on
/// newlines: a payload may itself contain newlines, so splitting one is not the inverse of
/// writing one.
fn emit_in_order(order: &[OverlayEntry]) -> Vec<u8> {
    let canonical = good();
    let header_end = canonical
        .iter()
        .enumerate()
        .filter(|(_, byte)| **byte == b'\n')
        .nth(4)
        .map(|(at, _)| at + 1)
        .expect("five header lines precede the count");
    let mut out = canonical[..header_end].to_vec();
    // The count is written for what is actually emitted, so a permutation test is not
    // also silently a count test.
    out.extend_from_slice(b"entries ");
    out.extend_from_slice(order.len().to_string().as_bytes());
    out.push(b'\n');
    for entry in order {
        out.extend_from_slice(b"entry ");
        out.extend_from_slice(entry.record().to_string().as_bytes());
        out.push(b' ');
        out.extend_from_slice(entry.tag().token().as_bytes());
        out.push(b' ');
        out.extend_from_slice(entry.bytes().len().to_string().as_bytes());
        out.push(b'\n');
        out.extend_from_slice(entry.bytes());
        out.push(b'\n');
    }
    out.extend_from_slice(b"overlay-end\n");
    out
}

#[test]
fn entries_out_of_canonical_order_are_refused_even_when_the_set_is_right() {
    // The canonical order is part of the form. Accepting a permuted region would be a second
    // grammar carrying the same values, and byte equality would stop being document equality.
    assert_eq!(
        emit_in_order(&entries()),
        good(),
        "emitting in canonical order reproduces the canonical region"
    );

    let mut reversed = entries();
    reversed.reverse();
    assert_eq!(
        validate(emit_in_order(&reversed)),
        Err(OverlayFault::Ordering)
    );
}

#[test]
fn a_region_repeating_one_slot_is_refused_before_the_second_can_win() {
    // Adjacent duplicates are ordering-equal rather than ordering-decreasing, so they need
    // their own arm; a bare monotonicity check would let them through.
    let doubled = vec![
        OverlayEntry::of(0, OpaqueFieldTag::Argv, b"first".to_vec()),
        OverlayEntry::of(0, OpaqueFieldTag::Argv, b"second".to_vec()),
    ];
    assert_eq!(
        validate(emit_in_order(&doubled)),
        Err(OverlayFault::DuplicateKey)
    );
}
