//! Plain narrowing over the two custody slots (`30Rb:book-content-and-locator-projection`).
//!
//! The narrowing walks the slot table, so these cases really check that the new slots JOINED that
//! table rather than being special-cased somewhere the narrow cannot see — which is how a plain
//! document would end up announcing content it does not carry.

#![expect(
    clippy::expect_used,
    reason = "fixture helpers beside the cases, where the in-tests allowance does not reach them"
)]

use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::{ABSENT, RecordKind};
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::projection::narrow_to_plain;

/// A counting identity source. The production edge fills these from the operating system.
struct Counter(u8);

impl ReceiptIdSource for Counter {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

fn record(kind: RecordKind, atoms: &[&str]) -> SkeletonRecord {
    SkeletonRecord::build(kind, atoms.iter().map(|atom| (*atom).to_owned()).collect())
        .expect("a row the grammar admits")
}

/// A rich skeleton whose source carries exact content and whose site carries a locator.
fn rich() -> Skeleton {
    Skeleton {
        receipt_id: "b".repeat(64),
        order: ReceiptOrderToken::of_controller_millis(1),
        signing_key_id: "c".repeat(64),
        encryption_key_id: Some("d".repeat(64)),
        records: vec![
            record(
                RecordKind::Source,
                &[
                    "0",
                    "book",
                    &"a".repeat(64),
                    "12",
                    "captured",
                    "uncollected",
                    "general-sh",
                    "captured",
                    "authored-before-contact",
                ],
            ),
            record(
                RecordKind::SiteDecision,
                &[
                    "0",
                    ABSENT,
                    "3",
                    "run",
                    "captured",
                    "captured",
                    "authored-before-contact",
                ],
            ),
        ],
    }
}

#[test]
fn plain_narrowing_withholds_source_content_and_the_site_locator() {
    let rich = rich();
    let plain = narrow_to_plain(&rich, &mut Counter(0), rich.order).expect("it narrows");

    assert_eq!(plain.records[0].atom("content"), Some("withheld-plain"));
    assert_eq!(plain.records[1].atom("locator"), Some("withheld-plain"));
    assert!(
        plain.encryption_key_id.is_none(),
        "a plain document names no encryption provider"
    );
}

/// Narrowing withholds what was CAPTURED and never overwrites an answer somebody else gave.
///
/// The excerpt slot is `uncollected` for its own reason — V1 selects no excerpts — and a narrow
/// that flattened every slot to `withheld-plain` would report a projection decision as a custody
/// one, which are different facts about why a reader has no bytes.
#[test]
fn a_slot_withheld_for_another_reason_keeps_its_own_word() {
    let rich = rich();
    let plain = narrow_to_plain(&rich, &mut Counter(0), rich.order).expect("it narrows");
    assert_eq!(plain.records[0].atom("excerpt"), Some("uncollected"));
}

/// The narrowed document is a REMINT: its own identity, never the rich one it came from.
#[test]
fn the_narrowed_document_takes_its_own_identity() {
    let rich = rich();
    let plain = narrow_to_plain(&rich, &mut Counter(0), rich.order).expect("it narrows");
    assert_ne!(
        plain.receipt_id, rich.receipt_id,
        "two documents under one identity would be unresolvable as the same or different"
    );
}
