//! The persistent-name corpus: the exact spellings this crate mints, and the exact refusal every
//! departure earns.
//!
//! A filename is a SELECTION HINT and never authority — every value it spells also sits inside the
//! signed body, and those are the ones that count. What this corpus pins is that a name means one
//! thing: nothing is repaired, nothing is normalized, and a departure is named for what it is so a
//! store can report a malformed entry under a recognized prefix differently from an entry that is
//! simply something else.

use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt_local::limits::LocalLimits;
use dorc_receipt_local::names::{
    ENCRYPTION_PRIVATE_FILE, KEY_DIR, KEYSET_DIR, KEYSET_MANIFEST_FILE, NameRefusal, NamedSpecies,
    RECEIPT_EXTENSION, ReceiptFileName, SIGNING_PRIVATE_FILE, STORE_DIR,
};

/// A fixed instant, so a committed name is a fixed byte string.
const FIXTURE_MILLIS: u64 = 1_700_000_000_000;

/// Its one spelling.
const FIXTURE_ORDER: &str = "00000001700000000000";

/// A fixed identity, likewise.
const FIXTURE_ID: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn order() -> ReceiptOrderToken {
    ReceiptOrderToken::of_controller_millis(FIXTURE_MILLIS)
}

#[test]
fn every_persistent_directory_and_file_name_carries_its_version() {
    // The redundancy is the point (`30Rd:versioned-persistent-names`): the format inside a file
    // already says `/1`, and the NAME says it too so an operator can tell an era apart without
    // opening anything. Spelled out here rather than only at the declarations, so an edit that
    // dropped a version from one of them is a red test rather than a quiet re-era.
    assert_eq!(KEY_DIR, "receipt-keys-v1");
    assert_eq!(KEYSET_DIR, "keyset-v1");
    assert_eq!(SIGNING_PRIVATE_FILE, "signing-private-v1.pk8");
    assert_eq!(ENCRYPTION_PRIVATE_FILE, "encryption-private-v1.age");
    assert_eq!(KEYSET_MANIFEST_FILE, "keyset-manifest-v1.txt");
    assert_eq!(STORE_DIR, "receipts-v1");
    assert_eq!(RECEIPT_EXTENSION, ".dorc-receipt");
    for name in [
        KEY_DIR,
        KEYSET_DIR,
        SIGNING_PRIVATE_FILE,
        ENCRYPTION_PRIVATE_FILE,
        KEYSET_MANIFEST_FILE,
        STORE_DIR,
    ] {
        assert!(name.contains("v1"), "{name} carries no era");
    }
}

/// Every species, and the exact filename it mints for the fixture values.
const COMMITTED_NAMES: &[(NamedSpecies, &str)] = &[
    (
        NamedSpecies::Plan,
        "plan-v1-00000001700000000000-0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.dorc-receipt",
    ),
    (
        NamedSpecies::ApplyIntent,
        "apply-intent-v1-00000001700000000000-0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.dorc-receipt",
    ),
    (
        NamedSpecies::ApplyOutcome,
        "apply-outcome-v1-00000001700000000000-0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.dorc-receipt",
    ),
];

#[test]
fn every_species_mints_exactly_its_committed_name_and_reads_it_back() {
    // Both directions over the same committed bytes. A mint the parser cannot read back would be
    // a store that publishes entries it can never enumerate.
    let limits = LocalLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    for (species, spelled) in COMMITTED_NAMES {
        let Some(name) = ReceiptFileName::of(*species, order(), FIXTURE_ID) else {
            failures.push(format!("{species:?}: did not mint"));
            continue;
        };
        if name.spelled() != *spelled {
            failures.push(format!("{species:?}: minted {}", name.spelled()));
        }
        match ReceiptFileName::of_entry(spelled, &limits) {
            Ok(read) => {
                if read.species() != *species
                    || read.order() != order()
                    || read.receipt_id() != FIXTURE_ID
                {
                    failures.push(format!("{species:?}: read back as {read:?}"));
                }
            }
            Err(refusal) => failures.push(format!("{species:?}: {refusal:?}")),
        }
    }
    // Every species is covered, so a table that lost one keeps failing rather than shrinking.
    assert_eq!(COMMITTED_NAMES.len(), NamedSpecies::ALL.len());
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Every departure, bound to the exact refusal it earns.
///
/// The two that matter most sit at the top: an entry that is simply not a receipt, and an entry
/// that wears a recognized species and is then wrong. A store reports those differently — only the
/// second says anything about this store — so collapsing them would lose the finding.
fn departures() -> Vec<(&'static str, String, NameRefusal)> {
    let head = format!("plan-v1-{FIXTURE_ORDER}");
    vec![
        (
            "no receipt extension",
            format!("{head}-{FIXTURE_ID}"),
            NameRefusal::NotAReceiptExtension,
        ),
        (
            "an unversioned species stem",
            format!("plan-{FIXTURE_ORDER}-{FIXTURE_ID}{RECEIPT_EXTENSION}"),
            NameRefusal::UnknownSpecies,
        ),
        (
            "a species this version does not know",
            format!("apply-report-v1-{FIXTURE_ORDER}-{FIXTURE_ID}{RECEIPT_EXTENSION}"),
            NameRefusal::UnknownSpecies,
        ),
        (
            "a sync client's conflict copy",
            format!(
                "plan-v1-{FIXTURE_ORDER}-{FIXTURE_ID}.sync-conflict-20260101-000000-AAAAAAA{RECEIPT_EXTENSION}"
            ),
            NameRefusal::IdentityNotExactDigest,
        ),
        (
            "nothing after the species",
            format!("plan-v1{RECEIPT_EXTENSION}"),
            NameRefusal::UnknownSpecies,
        ),
        (
            "no identity component",
            format!("{head}{RECEIPT_EXTENSION}"),
            NameRefusal::MalformedUnderKnownSpecies,
        ),
        (
            "an order one digit short",
            format!("plan-v1-0000001700000000000-{FIXTURE_ID}{RECEIPT_EXTENSION}"),
            NameRefusal::OrderNotExactWidth,
        ),
        (
            "an order without its leading zeroes",
            format!("plan-v1-1700000000000-{FIXTURE_ID}{RECEIPT_EXTENSION}"),
            NameRefusal::OrderNotExactWidth,
        ),
        (
            "an identity in uppercase",
            format!("{head}-{}{RECEIPT_EXTENSION}", FIXTURE_ID.to_uppercase()),
            NameRefusal::IdentityNotExactDigest,
        ),
        (
            "an identity one character short",
            format!("{head}-{}{RECEIPT_EXTENSION}", &FIXTURE_ID[..63]),
            NameRefusal::IdentityNotExactDigest,
        ),
        (
            "an entry past the name bound",
            format!(
                "plan-v1-{FIXTURE_ORDER}-{}{RECEIPT_EXTENSION}",
                "a".repeat(200)
            ),
            NameRefusal::OverNameBound,
        ),
    ]
}

#[test]
fn every_departure_is_refused_for_exactly_its_own_reason() {
    let limits = LocalLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    for (what, entry, want) in departures() {
        match ReceiptFileName::of_entry(&entry, &limits) {
            Err(got) if got == want => {}
            other => failures.push(format!("{what}: wanted {want:?}, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn an_uppercase_identity_is_refused_rather_than_folded() {
    // The reason the alphabet is lowercase-only. Some filesystems call two entries differing in
    // case one entry, so admitting uppercase would let one identity spell two names that
    // sometimes collide and sometimes do not — and folding it here would let a store answer for a
    // document whose name it had quietly rewritten.
    assert_eq!(
        ReceiptFileName::of(NamedSpecies::Plan, order(), &FIXTURE_ID.to_uppercase()),
        None
    );
}

#[test]
fn the_order_a_name_carries_is_the_order_a_reader_gets_back() {
    // The selection hint's whole job. Two names differing only in order must read back in the
    // same relation their tokens are in, or a store selecting the greatest would answer with the
    // older document.
    let limits = LocalLimits::V1;
    let earlier = ReceiptFileName::of(
        NamedSpecies::Plan,
        ReceiptOrderToken::of_controller_millis(9),
        FIXTURE_ID,
    )
    .expect("minted");
    let later = ReceiptFileName::of(
        NamedSpecies::Plan,
        ReceiptOrderToken::of_controller_millis(10),
        FIXTURE_ID,
    )
    .expect("minted");
    let read_earlier = ReceiptFileName::of_entry(&earlier.spelled(), &limits).expect("read");
    let read_later = ReceiptFileName::of_entry(&later.spelled(), &limits).expect("read");
    assert!(read_earlier.order() < read_later.order());
    // And the fixed width is what makes the BYTE order agree with it, which is what a listing
    // sorts by before anything is opened.
    assert!(earlier.spelled() < later.spelled());
}
