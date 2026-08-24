//! The signed plain round trip for all three species, and one region seal and open, through
//! the real implementations.
//!
//! The pure crate's corpus proves the grammar; this proves the two selected packages are
//! being driven correctly, and that the states they feed are minted only on the pure side.
//!
//! Every test collects its failures and asserts them together, so one run names everything
//! that moved rather than stopping at the first.

use dorc_receipt::capability::{
    OverlayOpener, OverlaySealer, PublicationGrade, ReceiptSigner, ReceiptSink,
    SelfAssertedReceiptVerificationKey, TrustedReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Species};
use dorc_receipt::reader::{ReadPlain, read_plain};
use dorc_receipt::writer::DraftReceipt;
use dorc_receipt_crypto::{
    AgeOpener, AgeSealer, Ed25519Signer, Ed25519Verifier, SelfAssertedEd25519Key, TrustedEd25519Key,
};

/// A fixed secret, so the corpus is reproducible. Test-only by construction: an integration
/// test is not compiled into the library.
const FIXTURE_SECRET: [u8; 32] = [7_u8; 32];

/// A deterministic identity source. The production edge supplies one backed by the operating
/// system; nothing in the kernel reaches for either.
struct CountingIds(u8);

impl ReceiptIdSource for CountingIds {
    fn next_receipt_id(&mut self) -> ReceiptId {
        let mut raw = [0_u8; 32];
        if let Some(slot) = raw.first_mut() {
            *slot = self.0;
        }
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes(raw)
    }
}

struct PolicyNames(TrustedEd25519Key);
struct PolicyDoesNotName(SelfAssertedEd25519Key);
struct PolicyHoldsNothing;

impl VerificationKeyResolver for PolicyNames {
    fn trusted(&self, id: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey> {
        (self.0.signing_key_id() == id).then_some(&self.0 as &dyn TrustedReceiptVerificationKey)
    }
    fn self_asserted(&self, _: SigningKeyId) -> Option<&dyn SelfAssertedReceiptVerificationKey> {
        None
    }
}

impl VerificationKeyResolver for PolicyDoesNotName {
    fn trusted(&self, _: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey> {
        None
    }
    fn self_asserted(&self, id: SigningKeyId) -> Option<&dyn SelfAssertedReceiptVerificationKey> {
        (self.0.signing_key_id() == id)
            .then_some(&self.0 as &dyn SelfAssertedReceiptVerificationKey)
    }
}

impl VerificationKeyResolver for PolicyHoldsNothing {
    fn trusted(&self, _: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey> {
        None
    }
    fn self_asserted(&self, _: SigningKeyId) -> Option<&dyn SelfAssertedReceiptVerificationKey> {
        None
    }
}

/// Holds nothing but the bytes it was handed, and reports the grade it actually achieved.
#[derive(Default)]
struct MemorySink {
    placed: Vec<(String, Vec<u8>)>,
    refuse: bool,
}

impl ReceiptSink for MemorySink {
    fn publish(&mut self, name: &str, bytes: &[u8]) -> Option<PublicationGrade> {
        if self.refuse {
            return None;
        }
        self.placed.push((name.to_owned(), bytes.to_vec()));
        Some(PublicationGrade::Volatile)
    }
}

fn signing_key() -> Ed25519Signer {
    Ed25519Signer::of_secret(FIXTURE_SECRET)
}

fn material() -> Option<Ed25519Verifier> {
    Ed25519Verifier::of_public_material(signing_key().public_material())
}

fn one_row_skeleton(ids: &mut CountingIds, provider: SigningKeyId) -> Option<Skeleton> {
    let row = SkeletonRecord::build(
        RecordKind::ProjectionOmission,
        ["observation", "0", "unminted", "authored-before-contact"]
            .iter()
            .map(|atom| (*atom).to_owned())
            .collect(),
    )
    .ok()?;
    Some(Skeleton {
        receipt_id: ids.next_receipt_id().hex(),
        signing_key_id: provider.hex(),
        encryption_key_id: None,
        records: vec![row],
    })
}

/// Sign one minimal document of the given species, answering its exact bytes.
fn signed_bytes<D: Species>(seed: u8) -> Option<Vec<u8>> {
    let key = signing_key();
    let mut ids = CountingIds(seed);
    let skeleton = one_row_skeleton(&mut ids, key.signing_key_id())?;
    Some(
        DraftReceipt::<D, Plain>::of(skeleton)
            .serialize()
            .ok()?
            .sign(&key)
            .bytes()
            .to_vec(),
    )
}

fn round_trip<D: Species>(name: &str, failures: &mut Vec<String>) {
    let limits = ReceiptLimits::V1;
    let key = signing_key();
    let mut ids = CountingIds(1);
    let Some(skeleton) = one_row_skeleton(&mut ids, key.signing_key_id()) else {
        failures.push(format!("{name}: could not build a skeleton"));
        return;
    };
    let Ok(serialized) = DraftReceipt::<D, Plain>::of(skeleton).serialize() else {
        failures.push(format!("{name}: a plain draft did not serialize"));
        return;
    };
    let signed = serialized.sign(&key);
    let bytes = signed.bytes().to_vec();

    let mut sink = MemorySink::default();
    match signed.publish(name, &mut sink) {
        Ok(published) => {
            if published.grade() != PublicationGrade::Volatile {
                failures.push(format!("{name}: unexpected publication grade"));
            }
        }
        Err(_) => failures.push(format!("{name}: the sink refused")),
    }
    if sink.placed.len() != 1 {
        failures.push(format!("{name}: the sink placed {}", sink.placed.len()));
    }

    let Some(named_material) = material() else {
        failures.push(format!("{name}: fixture material did not load"));
        return;
    };
    let named = PolicyNames(TrustedEd25519Key::of(named_material));
    match read_plain::<D>(bytes.clone(), &limits, &named) {
        Ok(ReadPlain::Trusted(recorded)) => {
            let report = recorded.as_report();
            if report.signer_provenance() != "trusted" || report.skeleton().records.len() != 1 {
                failures.push(format!("{name}: a trusted read reported wrongly"));
            }
        }
        _ => failures.push(format!("{name}: expected a trusted read")),
    }

    // The same bytes under material policy does not name land in the other arm. The document
    // is identical; only the provenance of the material differs, and the type says so.
    let Some(unnamed_material) = material() else {
        return;
    };
    let unnamed = PolicyDoesNotName(SelfAssertedEd25519Key::of(unnamed_material));
    match read_plain::<D>(bytes, &limits, &unnamed) {
        Ok(ReadPlain::SelfAsserted(recorded)) => {
            if recorded.as_report().signer_provenance() != "self-asserted" {
                failures.push(format!("{name}: a self-asserted read reported wrongly"));
            }
        }
        _ => failures.push(format!("{name}: expected a self-asserted read")),
    }
}

#[test]
fn every_species_round_trips_as_a_signed_plain_document() {
    let mut failures: Vec<String> = Vec::new();
    round_trip::<PlanReceipt>("plan", &mut failures);
    round_trip::<ApplyIntent>("apply-intent", &mut failures);
    round_trip::<ApplyOutcome>("apply-outcome", &mut failures);
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn one_flipped_body_byte_fails_the_check() {
    // The signature covers the exact span the reader parses, so any edit to the document —
    // including one inside a field the reader would otherwise accept — stops the read.
    let document = signed_bytes::<PlanReceipt>(9);
    assert!(document.is_some(), "the fixture document did not sign");
    let Some(mut bytes) = document else { return };

    let found = bytes.windows(7).position(|window| window == b"count=0");
    assert!(
        found.is_some(),
        "the omission row is missing from the fixture"
    );
    let Some(at) = found else { return };

    let slot = at.checked_add(6).filter(|index| *index < bytes.len());
    assert!(
        slot.is_some(),
        "the row is shorter than the fixture promises"
    );
    let Some(index) = slot else { return };
    if let Some(byte) = bytes.get_mut(index) {
        *byte = b'1';
    }

    let held = material();
    assert!(held.is_some(), "fixture material did not load");
    let Some(held) = held else { return };
    let named = PolicyNames(TrustedEd25519Key::of(held));
    assert!(
        read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &named).is_err(),
        "a mutated body must not read"
    );
}

#[test]
fn a_document_signed_for_one_species_does_not_read_as_another() {
    // The payload type is derived from the type parameters, so reading a plan document as an
    // apply intent changes the checked input and fails before the grammar is consulted.
    let document = signed_bytes::<PlanReceipt>(3);
    assert!(document.is_some(), "the fixture document did not sign");
    let Some(bytes) = document else { return };

    let held = material();
    assert!(held.is_some(), "fixture material did not load");
    let Some(held) = held else { return };

    let named = PolicyNames(TrustedEd25519Key::of(held));
    let limits = ReceiptLimits::V1;
    assert!(read_plain::<PlanReceipt>(bytes.clone(), &limits, &named).is_ok());
    assert!(
        read_plain::<ApplyIntent>(bytes, &limits, &named).is_err(),
        "the signature domain names the species"
    );
}

#[test]
fn material_the_resolver_does_not_hold_stops_the_read() {
    let document = signed_bytes::<PlanReceipt>(4);
    assert!(document.is_some(), "the fixture document did not sign");
    let Some(bytes) = document else { return };
    assert!(read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &PolicyHoldsNothing).is_err());
}

#[test]
fn a_failed_sink_mints_no_publication() {
    let key = signing_key();
    let mut ids = CountingIds(5);
    let built = one_row_skeleton(&mut ids, key.signing_key_id());
    assert!(built.is_some(), "could not build a skeleton");
    let Some(skeleton) = built else { return };

    let serialized = DraftReceipt::<PlanReceipt, Plain>::of(skeleton).serialize();
    assert!(serialized.is_ok(), "the draft did not serialize");
    let Ok(serialized) = serialized else { return };

    let mut sink = MemorySink {
        refuse: true,
        ..MemorySink::default()
    };
    assert!(serialized.sign(&key).publish("plan", &mut sink).is_err());
    assert!(sink.placed.is_empty());
}

#[test]
fn a_region_seals_and_opens_and_refuses_past_its_bound() {
    // The two package seams the rich projection will use, exercised end to end before any
    // rich document exists: canonical armor out in the format's own line ending, exact bytes
    // back, and a bound that refuses rather than truncating into something that looks whole.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let opener = AgeOpener::of(identity);

    let plaintext = b"one region's exact bytes".to_vec();
    let region = sealer.seal(&plaintext);
    assert!(region.is_some(), "the region did not seal");
    let Some(armor) = region else { return };

    assert!(armor.starts_with("-----BEGIN AGE ENCRYPTED FILE-----"));
    assert!(armor.ends_with("-----END AGE ENCRYPTED FILE-----"));
    assert!(
        !armor.contains('\r'),
        "the stored region is LF-only, like every other line of the format"
    );
    assert!(
        !armor.ends_with('\n'),
        "the format supplies the newline that closes the region"
    );

    assert_eq!(opener.open(&armor, 1024), Some(plaintext.clone()));

    let exact = u64::try_from(plaintext.len());
    assert!(exact.is_ok(), "the fixture plaintext is small");
    let Ok(exact) = exact else { return };
    assert!(opener.open(&armor, exact).is_some(), "at the bound");
    assert!(
        opener.open(&armor, exact.saturating_sub(1)).is_none(),
        "past the bound the region is refused, never truncated"
    );

    let damaged = format!("{armor}ZZZZ");
    assert!(
        opener.open(&damaged, 1024).is_none(),
        "damaged armor refuses"
    );
}

#[test]
fn the_two_provider_roles_derive_separate_identities() {
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    assert_ne!(
        signing_key().signing_key_id().hex(),
        sealer.encryption_key_id().hex(),
        "the two provider roles never alias"
    );
}
