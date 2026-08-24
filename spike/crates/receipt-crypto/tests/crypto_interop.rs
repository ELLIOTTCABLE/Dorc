//! The signed plain round trip for all three species, and one region seal and open, through
//! the real implementations.
//!
//! The pure crate's corpus proves the grammar; this proves the two selected packages are
//! being driven correctly, and that the states they feed are minted only on the pure side.
//!
//! Every test collects its failures and asserts them together, so one run names everything
//! that moved rather than stopping at the first.

#![expect(
    clippy::expect_used,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use dorc_receipt::capability::{
    OverlayOpener, OverlaySealer, PublicationGrade, ReceiptSigner, ReceiptSink,
    SelfAssertedReceiptVerificationKey, TrustedReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Rich, Species};
use dorc_receipt::overlay::OverlayEntry;
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::{ReadPlain, ReadRich, read_plain, read_rich};
use dorc_receipt::writer::{DraftReceipt, OverlayPlaintext};
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

/// A rich skeleton whose invocation row captures its argv, bound to one encryption provider.
fn rich_skeleton(ids: &mut CountingIds, signing: SigningKeyId, encryption: &str) -> Skeleton {
    let row = SkeletonRecord::build(
        RecordKind::Invocation,
        [
            "plan",
            "absent",
            "captured",
            "withheld-plain",
            "0",
            "authored-before-contact",
        ]
        .iter()
        .map(|atom| (*atom).to_owned())
        .collect(),
    )
    .expect("the fixture row is well formed");
    Skeleton {
        receipt_id: ids.next_receipt_id().hex(),
        signing_key_id: signing.hex(),
        encryption_key_id: Some(encryption.to_owned()),
        records: vec![row],
    }
}

#[test]
fn a_rich_document_round_trips_through_both_real_packages() {
    // The whole order in one run: seal, serialize, sign, then locate, verify, parse, open and
    // validate. Every step is real; nothing here is a stand-in.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let opener = AgeOpener::of(identity);
    let signer = signing_key();
    let Some(verifier) = material() else {
        panic!("the fixture verification material is well formed")
    };
    let resolver = PolicyNames(TrustedEd25519Key::of(verifier));

    let mut ids = CountingIds(70);
    let skeleton = rich_skeleton(
        &mut ids,
        signer.signing_key_id(),
        &sealer.encryption_key_id().hex(),
    );
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&skeleton)
        .expect("the fixture skeleton serializes");
    let entries = vec![OverlayEntry::of(
        0,
        OpaqueFieldTag::Argv,
        b"dorc plan book.sh web1".to_vec(),
    )];
    let plaintext = OverlayPlaintext::canonical(
        &skeleton.receipt_id,
        PlanReceipt::TOKEN,
        span.as_bytes(),
        &entries,
    );

    let serialized = DraftReceipt::<PlanReceipt, Rich>::of(skeleton)
        .serialize(plaintext, &sealer)
        .expect("a rich document serializes");
    let bytes = serialized.sign(&signer).bytes().to_vec();

    let text = String::from_utf8(bytes.clone()).expect("the document is text");
    assert!(
        !text.contains('\r'),
        "the stored document is LF-only throughout"
    );
    assert!(
        text.contains("-----BEGIN AGE ENCRYPTED FILE-----"),
        "the region is stored in canonical armor"
    );

    match read_rich::<PlanReceipt>(bytes, &ReceiptLimits::V1, &resolver, &opener) {
        Ok(ReadRich::Trusted(_)) => {}
        other => panic!("the rich document did not read back: {other:?}"),
    }
}

#[test]
fn a_real_armored_region_satisfies_the_shape_the_locator_requires() {
    // The locator's shape check is written against this writer's output. Measuring the real
    // region against it is what keeps the two from drifting apart silently: if the package
    // ever changes its wrapping, this fails rather than every rich document failing to locate.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let region = sealer.seal(b"some region bytes").expect("the region seals");
    assert_eq!(dorc_receipt::format::check_armor_shape(&region), Ok(()));
}

/// Build one signed rich document, and hand back its bytes and the material to read it.
fn rich_document(seed: u8, argv: &[u8]) -> (Vec<u8>, AgeOpener, PolicyNames) {
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let opener = AgeOpener::of(identity);
    let signer = signing_key();
    let verifier = material().expect("the fixture verification material is well formed");
    let resolver = PolicyNames(TrustedEd25519Key::of(verifier));

    let mut ids = CountingIds(seed);
    let skeleton = rich_skeleton(
        &mut ids,
        signer.signing_key_id(),
        &sealer.encryption_key_id().hex(),
    );
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&skeleton)
        .expect("the fixture skeleton serializes");
    let plaintext = OverlayPlaintext::canonical(
        &skeleton.receipt_id,
        PlanReceipt::TOKEN,
        span.as_bytes(),
        &[OverlayEntry::of(0, OpaqueFieldTag::Argv, argv.to_vec())],
    );
    let bytes = DraftReceipt::<PlanReceipt, Rich>::of(skeleton)
        .serialize(plaintext, &sealer)
        .expect("a rich document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();
    (bytes, opener, resolver)
}

#[test]
fn a_region_from_another_document_releases_nothing() {
    // Both documents are validly signed and both regions open. What refuses the swap is the
    // region's own binding to the skeleton it was written for.
    let (mine, _, _) = rich_document(80, b"mine");
    let (theirs, their_opener, their_resolver) = rich_document(81, b"theirs");

    let text = String::from_utf8(mine).expect("text");
    let start = text.find("-----BEGIN").expect("a region");
    let end = text.find("opaque-end").expect("a terminator");
    let foreign_region = &text[start..end];

    let target = String::from_utf8(theirs).expect("text");
    let t_start = target.find("-----BEGIN").expect("a region");
    let t_end = target.find("opaque-end").expect("a terminator");
    let swapped = format!(
        "{}{}{}",
        &target[..t_start],
        foreign_region,
        &target[t_end..]
    );

    let outcome = read_rich::<PlanReceipt>(
        swapped.into_bytes(),
        &ReceiptLimits::V1,
        &their_resolver,
        &their_opener,
    );
    assert!(
        outcome.is_err(),
        "a region written for another document released something"
    );
}

#[test]
fn a_damaged_region_releases_nothing_and_names_the_step_that_stopped() {
    let (bytes, opener, resolver) = rich_document(82, b"dorc plan book.sh");
    let text = String::from_utf8(bytes).expect("text");
    // Flip one base64 character inside the region, leaving every marker and width intact, so
    // the shape still locates and the failure is the package's own authentication.
    let at = text.find("-----BEGIN").expect("a region") + 40;
    let mut damaged = text.clone().into_bytes();
    damaged[at] = if damaged[at] == b'A' { b'B' } else { b'A' };

    let outcome = read_rich::<PlanReceipt>(damaged, &ReceiptLimits::V1, &resolver, &opener);
    match outcome {
        Err(partial) => assert!(
            matches!(
                partial.reason(),
                dorc_receipt::format::RefusalReason::RegionUnopenable
                    | dorc_receipt::format::RefusalReason::SignatureCheck
            ),
            "unexpected reason: {:?}",
            partial.reason()
        ),
        Ok(_) => panic!("a damaged region read as whole"),
    }
}

#[test]
fn the_two_projections_refuse_each_other_at_the_reader() {
    // Plain and rich are separate documents, not one document read two ways.
    let (rich, opener, resolver) = rich_document(83, b"dorc plan book.sh");
    assert!(
        read_plain::<PlanReceipt>(rich, &ReceiptLimits::V1, &resolver).is_err(),
        "a rich document did not read as plain"
    );

    let plain = signed_bytes::<PlanReceipt>(84).expect("a plain document");
    assert!(
        read_rich::<PlanReceipt>(plain, &ReceiptLimits::V1, &resolver, &opener).is_err(),
        "a plain document did not read as rich"
    );
}
