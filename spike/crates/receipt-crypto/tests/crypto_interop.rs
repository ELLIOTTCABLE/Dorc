//! The signed plain round trip for all three species, and one region seal/open, through the
//! real implementations.
//!
//! The pure crate's own corpus proves the grammar with a fixture signer; this proves the two
//! selected packages are being driven correctly and that the states they feed are minted only
//! on the pure side.

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
        // The pure crate owns the representation, so a test cannot fabricate one directly;
        // it drives the same seam production drives.
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

fn signer() -> Ed25519Signer {
    Ed25519Signer::of_secret(FIXTURE_SECRET)
}

fn verifier() -> Ed25519Verifier {
    Ed25519Verifier::of_public_material(signer().public_material()).expect("fixture material")
}

fn one_omission_skeleton(ids: &mut CountingIds, signing: SigningKeyId) -> Skeleton {
    let record = SkeletonRecord::build(
        RecordKind::ProjectionOmission,
        vec![
            "observation".to_owned(),
            "0".to_owned(),
            "unminted".to_owned(),
            "authored-before-contact".to_owned(),
        ],
    )
    .expect("a well-formed omission row");
    Skeleton {
        receipt_id: ids.next_receipt_id().hex(),
        signing_key_id: signing.hex(),
        encryption_key_id: None,
        records: vec![record],
    }
}

fn round_trip<D: Species>(name: &str) {
    let limits = ReceiptLimits::V1;
    let signer = signer();
    let mut ids = CountingIds(1);
    let skeleton = one_omission_skeleton(&mut ids, signer.signing_key_id());

    let signed = DraftReceipt::<D, Plain>::of(skeleton)
        .serialize()
        .expect("a plain draft serializes")
        .sign(&signer);
    let bytes = signed.bytes().to_vec();

    let mut sink = MemorySink::default();
    let published = signed.publish(name, &mut sink).expect("the sink placed it");
    assert_eq!(published.grade(), PublicationGrade::Volatile);
    assert_eq!(sink.placed.len(), 1);

    let resolver = PolicyNames(TrustedEd25519Key::of(verifier()));
    match read_plain::<D>(bytes.clone(), &limits, &resolver) {
        Ok(ReadPlain::Trusted(recorded)) => {
            assert_eq!(recorded.as_report().signer_provenance(), "trusted");
            assert_eq!(recorded.as_report().skeleton().records.len(), 1);
        }
        other => panic!("{name}: expected a trusted read, got {other:?}"),
    }

    // The same bytes under material policy does not name land in the other arm. The document
    // is identical; only the provenance of the material differs, and the type says so.
    let unnamed = PolicyDoesNotName(SelfAssertedEd25519Key::of(verifier()));
    match read_plain::<D>(bytes, &limits, &unnamed) {
        Ok(ReadPlain::SelfAsserted(recorded)) => {
            assert_eq!(recorded.as_report().signer_provenance(), "self-asserted");
        }
        other => panic!("{name}: expected a self-asserted read, got {other:?}"),
    }
}

#[test]
fn every_species_round_trips_as_a_signed_plain_document() {
    round_trip::<PlanReceipt>("plan");
    round_trip::<ApplyIntent>("apply-intent");
    round_trip::<ApplyOutcome>("apply-outcome");
}

#[test]
fn one_flipped_body_byte_fails_the_check() {
    // The signature covers the exact span the reader parses, so any edit to the document —
    // including one inside a field the reader would have accepted — stops the read.
    let limits = ReceiptLimits::V1;
    let signer = signer();
    let mut ids = CountingIds(9);
    let skeleton = one_omission_skeleton(&mut ids, signer.signing_key_id());
    let signed = DraftReceipt::<PlanReceipt, Plain>::of(skeleton)
        .serialize()
        .expect("serializes")
        .sign(&signer);

    let mut bytes = signed.bytes().to_vec();
    let at = bytes
        .windows(11)
        .position(|w| w == b"count=0 rea")
        .expect("the omission row is present");
    bytes[at + 6] = b'1';

    let resolver = PolicyNames(TrustedEd25519Key::of(verifier()));
    assert!(
        read_plain::<PlanReceipt>(bytes, &limits, &resolver).is_err(),
        "a mutated body must not read"
    );
}

#[test]
fn a_document_signed_for_one_species_does_not_read_as_another() {
    // The payload type is derived from the type parameters, so reading a plan document as an
    // apply intent changes the checked input and fails before the grammar is consulted.
    let limits = ReceiptLimits::V1;
    let signer = signer();
    let mut ids = CountingIds(3);
    let skeleton = one_omission_skeleton(&mut ids, signer.signing_key_id());
    let signed = DraftReceipt::<PlanReceipt, Plain>::of(skeleton)
        .serialize()
        .expect("serializes")
        .sign(&signer);
    let bytes = signed.bytes().to_vec();

    let resolver = PolicyNames(TrustedEd25519Key::of(verifier()));
    assert!(read_plain::<PlanReceipt>(bytes.clone(), &limits, &resolver).is_ok());
    assert!(
        read_plain::<ApplyIntent>(bytes, &limits, &resolver).is_err(),
        "the signature domain names the species"
    );
}

#[test]
fn material_the_resolver_does_not_hold_stops_the_read() {
    struct Empty;
    impl VerificationKeyResolver for Empty {
        fn trusted(&self, _: SigningKeyId) -> Option<&dyn TrustedReceiptVerificationKey> {
            None
        }
        fn self_asserted(
            &self,
            _: SigningKeyId,
        ) -> Option<&dyn SelfAssertedReceiptVerificationKey> {
            None
        }
    }
    let limits = ReceiptLimits::V1;
    let signer = signer();
    let mut ids = CountingIds(4);
    let skeleton = one_omission_skeleton(&mut ids, signer.signing_key_id());
    let signed = DraftReceipt::<PlanReceipt, Plain>::of(skeleton)
        .serialize()
        .expect("serializes")
        .sign(&signer);
    assert!(read_plain::<PlanReceipt>(signed.bytes().to_vec(), &limits, &Empty).is_err());
}

#[test]
fn a_failed_sink_mints_no_publication() {
    let signer = signer();
    let mut ids = CountingIds(5);
    let skeleton = one_omission_skeleton(&mut ids, signer.signing_key_id());
    let signed = DraftReceipt::<PlanReceipt, Plain>::of(skeleton)
        .serialize()
        .expect("serializes")
        .sign(&signer);
    let mut sink = MemorySink {
        refuse: true,
        ..MemorySink::default()
    };
    assert!(signed.publish("plan", &mut sink).is_err());
    assert!(sink.placed.is_empty());
}

#[test]
fn a_region_seals_and_opens_and_refuses_past_its_bound() {
    // The two package seams the rich projection will use, exercised end to end before any
    // rich document exists: canonical armor out, exact bytes back, and a bound that refuses
    // rather than truncating into something that looks whole.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let opener = AgeOpener::of(identity);

    let plaintext = b"one region's exact bytes".to_vec();
    let armor = sealer.seal(&plaintext).expect("seals");
    assert!(armor.starts_with("-----BEGIN AGE ENCRYPTED FILE-----"));
    assert!(armor.ends_with("-----END AGE ENCRYPTED FILE-----"));
    assert!(
        !armor.ends_with('\n'),
        "the format supplies the closing newline"
    );

    let opened = opener.open(&armor, 1024).expect("opens");
    assert_eq!(opened, plaintext);

    let plaintext_len = u64::try_from(plaintext.len()).expect("small");
    assert!(opener.open(&armor, plaintext_len).is_some(), "at the bound");
    assert!(
        opener.open(&armor, plaintext_len - 1).is_none(),
        "past the bound the region is refused, never truncated"
    );

    let mut damaged = armor.clone();
    damaged.push_str("ZZZZ");
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
        signer().signing_key_id().hex(),
        sealer.encryption_key_id().hex(),
        "the two provider roles never alias"
    );
}
