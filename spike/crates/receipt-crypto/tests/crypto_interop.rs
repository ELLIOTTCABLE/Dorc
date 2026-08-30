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
    ReceiptVerificationKey, VerificationKeyResolver,
};
use dorc_receipt::format::{Skeleton, SkeletonRecord};
use dorc_receipt::grammar::RecordKind;
use dorc_receipt::ids::{ReceiptId, ReceiptIdSource, SigningKeyId};
use dorc_receipt::limits::ReceiptLimits;
use dorc_receipt::model::{ApplyIntent, ApplyOutcome, Plain, PlanReceipt, Rich, Species};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::overlay::OverlayEntry;
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::reader::{read_plain, read_rich};
use dorc_receipt::writer::{DraftReceipt, OverlayPlaintext};
use dorc_receipt_crypto::{AgeOpener, AgeSealer, Ed25519Signer, Ed25519Verifier};

/// A fixed secret, so the corpus is reproducible. Test-only by construction: an integration
/// test is not compiled into the library.
const FIXTURE_SECRET: [u8; 32] = [7_u8; 32];

/// The instant every fixture document is stamped with, so a committed rich vector carries a
/// fixed reviewed order rather than one a run happened to observe.
const FIXTURE_MILLIS: u64 = 1_700_000_000_000;

/// The order a remint is stamped with: a second document written at a second moment, never the
/// moment its origin was written.
fn remint_order() -> ReceiptOrderToken {
    ReceiptOrderToken::of_controller_millis(FIXTURE_MILLIS + 1)
}

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

struct PolicyNames(Ed25519Verifier);
struct PolicyHoldsNothing;

impl VerificationKeyResolver for PolicyNames {
    fn material(&self, id: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
        (ReceiptVerificationKey::signing_key_id(&self.0) == id)
            .then_some(&self.0 as &dyn ReceiptVerificationKey)
    }
}

impl VerificationKeyResolver for PolicyHoldsNothing {
    fn material(&self, _: SigningKeyId) -> Option<&dyn ReceiptVerificationKey> {
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
        order: ReceiptOrderToken::of_controller_millis(FIXTURE_MILLIS),
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
    let named = PolicyNames(named_material);
    match read_plain::<D>(bytes.clone(), &limits, &named) {
        Ok(recorded) => {
            if recorded.record_count() != 1 {
                failures.push(format!("{name}: the read reported wrongly"));
            }
        }
        Err(_) => failures.push(format!("{name}: expected a completed read")),
    }

    // The same bytes under material the resolver does not hold do not read at all. There is no
    // second, differently-labelled arm to land in: whose key it is is not a question this crate
    // answers, so "held" and "not held" is the whole of the distinction.
    match read_plain::<D>(bytes, &limits, &PolicyHoldsNothing) {
        Err(partial) => {
            if *partial.reason() != dorc_receipt::RefusalReason::KeyUnavailable {
                failures.push(format!(
                    "{name}: unheld material refused for the wrong reason"
                ));
            }
        }
        Ok(_) => failures.push(format!("{name}: unheld material must not read")),
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
    let named = PolicyNames(held);
    assert!(
        read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &named).is_err(),
        "a mutated body must not read"
    );
}

#[test]
fn one_edited_order_digit_fails_the_check() {
    // The order line is INSIDE the signed body, which is the whole of what stops a local name
    // and a document disagreeing usefully: a store that trusted a filename could be handed one
    // spelling in the name and another in the header, and the reader's comparison would be
    // between two values the same party chose. Editing one digit is the smallest form of that,
    // and it has to fail the signature rather than the grammar — the edited spelling below is
    // still exactly twenty digits, so nothing downstream of the check would object to it.
    let document = signed_bytes::<PlanReceipt>(11);
    assert!(document.is_some(), "the fixture document did not sign");
    let Some(mut bytes) = document else { return };
    let unedited = bytes.clone();

    let spelled = ReceiptOrderToken::of_controller_millis(FIXTURE_MILLIS).spelled();
    let needle = format!("\norder {spelled}\n");
    let found = bytes
        .windows(needle.len())
        .position(|window| window == needle.as_bytes());
    assert!(
        found.is_some(),
        "the fixture document carries no order line"
    );
    let Some(at) = found else { return };

    // The last digit of the order value: still a digit afterwards, so the departure is the
    // signature and nothing else.
    let slot = at
        .checked_add(needle.len())
        .and_then(|end| end.checked_sub(2))
        .filter(|index| *index < bytes.len());
    assert!(
        slot.is_some(),
        "the order line is shorter than its spelling"
    );
    let Some(index) = slot else { return };
    if let Some(byte) = bytes.get_mut(index) {
        assert!(byte.is_ascii_digit(), "the edited byte is an order digit");
        *byte = if *byte == b'9' { b'8' } else { b'9' };
    }

    let held = material();
    assert!(held.is_some(), "fixture material did not load");
    let Some(held) = held else { return };
    let named = PolicyNames(held);
    // The positive control, so the refusal below is caused by the edit rather than by the
    // fixture never having read in the first place.
    assert!(
        read_plain::<PlanReceipt>(unedited, &ReceiptLimits::V1, &named).is_ok(),
        "the unedited fixture reads"
    );
    // Pinned to the EXACT refusal, not merely to a refusal: an edited digit that still spelled
    // twenty digits would also be refused if the grammar happened to reject it for some other
    // reason, and the test would then pass with the order line outside the signed span, which is
    // the one thing it exists to prove.
    match read_plain::<PlanReceipt>(bytes, &ReceiptLimits::V1, &named) {
        Err(partial) => assert_eq!(
            *partial.reason(),
            dorc_receipt::RefusalReason::SignatureCheck,
            "an edited order fails the CHECK, which is what makes it authenticated rather than \
             advisory metadata a store could be handed two spellings of"
        ),
        Ok(_) => panic!("an edited order must not read"),
    }
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

    let named = PolicyNames(held);
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
        order: ReceiptOrderToken::of_controller_millis(FIXTURE_MILLIS),
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
    let resolver = PolicyNames(verifier);

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
        Ok(_) => {}
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
    let resolver = PolicyNames(verifier);

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
    match outcome {
        // The region is inside the signed body, so a swap is caught by the outer check before
        // the region binding is even consulted. The binding itself is pinned at the validator,
        // over inert bytes, where no signature can mask it.
        Err(partial) => assert_eq!(
            partial.reason(),
            &dorc_receipt::format::RefusalReason::SignatureCheck
        ),
        Ok(_) => panic!("a region written for another document released something"),
    }
}

#[test]
fn a_damaged_region_is_refused_by_the_signature_because_the_signature_covers_it() {
    // Naming the layer matters: the region is inside the signed body, so a flipped byte is a
    // signature failure and never reaches the opener at all. A test that accepted either
    // answer would keep passing if that ordering were ever reversed.
    let (bytes, opener, resolver) = rich_document(82, b"dorc plan book.sh");
    let text = String::from_utf8(bytes).expect("text");
    let at = text.find("-----BEGIN").expect("a region") + 40;
    let mut damaged = text.into_bytes();
    damaged[at] = if damaged[at] == b'A' { b'B' } else { b'A' };

    match read_rich::<PlanReceipt>(damaged, &ReceiptLimits::V1, &resolver, &opener) {
        Err(partial) => assert_eq!(
            partial.reason(),
            &dorc_receipt::format::RefusalReason::SignatureCheck
        ),
        Ok(_) => panic!("a damaged region read as whole"),
    }
}

#[test]
fn a_damaged_region_under_a_matching_signature_is_refused_by_the_opener() {
    // The other half of the same statement. Re-signing the damaged bytes gets past the outer
    // check, and what refuses then is the region's own authentication — so the two layers are
    // separately load-bearing rather than one masking the other.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let opener = AgeOpener::of(identity);
    let signer = signing_key();
    let verifier = material().expect("the fixture verification material is well formed");
    let resolver = PolicyNames(verifier);

    let mut ids = CountingIds(85);
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
        &[OverlayEntry::of(0, OpaqueFieldTag::Argv, b"argv".to_vec())],
    );
    let whole = DraftReceipt::<PlanReceipt, Rich>::of(skeleton)
        .serialize(plaintext, &sealer)
        .expect("a rich document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();

    let located = dorc_receipt::format::locate(&whole, &ReceiptLimits::V1).expect("it locates");
    let armor = located.armor.expect("it carries a region");
    let mut damaged: Vec<u8> = armor.clone().into_bytes();
    let at = 40;
    damaged[at] = if damaged[at] == b'A' { b'B' } else { b'A' };
    let damaged = String::from_utf8(damaged).expect("still text");

    let body = dorc_receipt::format::signed_body(&span, Some(&damaged));
    let signature = signer.sign(&dorc_receipt::ids::pae(
        &dorc_receipt::model::payload_type::<PlanReceipt, Rich>(),
        &body,
    ));
    let resigned = dorc_receipt::format::assemble(
        &span,
        Some(&damaged),
        &dorc_receipt::ids::to_hex(&signature),
    );

    match read_rich::<PlanReceipt>(resigned, &ReceiptLimits::V1, &resolver, &opener) {
        Err(partial) => assert_eq!(
            partial.reason(),
            &dorc_receipt::format::RefusalReason::RegionUnopenable
        ),
        Ok(_) => panic!("a damaged region opened"),
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

#[test]
fn rich_narrows_to_plain_by_reminting_and_never_by_stripping_text() {
    // Two halves. The remint produces a document that says what it is and carries a signature
    // over its own plain bytes. The strip produces bytes nobody signed, and the reader says so
    // rather than accepting a rich document with its region deleted.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let signer = signing_key();
    let verifier = material().expect("the fixture verification material is well formed");
    let resolver = PolicyNames(verifier);

    let mut ids = CountingIds(90);
    let rich = rich_skeleton(
        &mut ids,
        signer.signing_key_id(),
        &sealer.encryption_key_id().hex(),
    );
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&rich)
        .expect("the fixture skeleton serializes");
    let plaintext = OverlayPlaintext::canonical(
        &rich.receipt_id,
        PlanReceipt::TOKEN,
        span.as_bytes(),
        &[OverlayEntry::of(0, OpaqueFieldTag::Argv, b"argv".to_vec())],
    );
    let rich_bytes = DraftReceipt::<PlanReceipt, Rich>::of(rich.clone())
        .serialize(plaintext, &sealer)
        .expect("a rich document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();

    let mut remint_ids = CountingIds(180);
    let plain = dorc_receipt::projection::narrow_to_plain(&rich, &mut remint_ids, remint_order())
        .expect("the narrowing holds");
    assert_eq!(
        plain.encryption_key_id, None,
        "plain names no encryption provider"
    );
    assert_ne!(
        plain.receipt_id, rich.receipt_id,
        "the narrowed document is its own document and mints its own identity"
    );
    let plain_bytes = DraftReceipt::<PlanReceipt, Plain>::of(plain)
        .serialize()
        .expect("the narrowed document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();
    let narrowed = String::from_utf8(plain_bytes.clone()).expect("text");
    assert!(
        narrowed.contains("argv=withheld-plain"),
        "a captured slot narrows to a withheld one: {narrowed}"
    );
    assert!(
        !narrowed.contains("captured"),
        "no slot still claims capture"
    );
    assert!(
        read_plain::<PlanReceipt>(plain_bytes, &ReceiptLimits::V1, &resolver).is_ok(),
        "the reminted document reads as plain under its own signature"
    );

    // Now the strip: delete the region from the rich bytes and relabel the projection, which
    // is what a textual narrowing would amount to.
    let rich_text = String::from_utf8(rich_bytes).expect("text");
    let start = rich_text.find("opaque-overlay\n").expect("a region opens");
    let end = rich_text.find("opaque-end\n").expect("a region closes") + "opaque-end\n".len();
    let stripped = format!(
        "{}{}",
        rich_text[..start].replace("projection rich", "projection plain"),
        &rich_text[end..]
    );
    assert!(
        read_plain::<PlanReceipt>(stripped.into_bytes(), &ReceiptLimits::V1, &resolver).is_err(),
        "a rich document with its region deleted must not read as a plain one"
    );
}

/// The age identity that seals the committed rich vectors, and nothing else, ever.
///
/// Generated fresh for this corpus and reused from nowhere. It exists so a frozen rich
/// document stays openable: age encryption is not reproducible, so those vectors cannot be
/// regenerated and must be readable years from now with the material committed beside them.
/// It is fixture material by construction — an integration test is not compiled into any
/// library, and `the_fixture_identity_is_unreachable_from_production` holds it there.
const FIXTURE_ONLY_AGE_IDENTITY_SEALS_COMMITTED_VECTORS_ONLY: &str =
    "AGE-SECRET-KEY-1WRNNRELNXYYJLWD2WTKAYUDDQRDP76K0QU4FCLT05LRYKTJ47JSQ6J8N8H";

fn committed_rich_vectors() -> Vec<(String, Vec<u8>)> {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../receipt/tests/vectors/valid");
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_owned();
            if !name.contains(".receipt") {
                return None;
            }
            Some((name, std::fs::read(&path).ok()?))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "no committed rich vectors");
    out
}

#[test]
fn every_committed_rich_vector_reads_back_whole_under_the_fixture_material() {
    // The frozen corpus, driven through the entire order with the real packages: verify, parse,
    // open, validate. An in-process round trip proves the writer and the reader agree with each
    // other; this proves they still agree with bytes neither of them just produced.
    let identity: age::x25519::Identity = FIXTURE_ONLY_AGE_IDENTITY_SEALS_COMMITTED_VECTORS_ONLY
        .parse()
        .expect("the fixture identity parses");
    let opener = AgeOpener::of(identity);
    let verifier = material().expect("the fixture verification material is well formed");
    let resolver = PolicyNames(verifier);

    let mut failures: Vec<String> = Vec::new();
    for (name, bytes) in committed_rich_vectors() {
        let species = String::from_utf8_lossy(&bytes)
            .lines()
            .nth(1)
            .unwrap_or_default()
            .to_owned();
        let outcome = match species.as_str() {
            "species plan" => {
                read_rich::<PlanReceipt>(bytes, &ReceiptLimits::V1, &resolver, &opener)
                    .map(|_| ())
                    .map_err(|partial| format!("{:?}", partial.reason()))
            }
            "species apply-intent" => {
                read_rich::<ApplyIntent>(bytes, &ReceiptLimits::V1, &resolver, &opener)
                    .map(|_| ())
                    .map_err(|partial| format!("{:?}", partial.reason()))
            }
            "species apply-outcome" => {
                read_rich::<ApplyOutcome>(bytes, &ReceiptLimits::V1, &resolver, &opener)
                    .map(|_| ())
                    .map_err(|partial| format!("{:?}", partial.reason()))
            }
            other => Err(format!("unknown species line {other}")),
        };
        if let Err(reason) = outcome {
            failures.push(format!("{name}: {reason}"));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn a_plain_remint_is_a_second_document_with_its_own_identity() {
    // The case that would catch a regression to treating a remint as the same document. A
    // narrowed document is minted, not relabelled: it gets its own identity from the injected
    // source, so nothing downstream has to reason about one identity spanning two byte-images.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let signer = signing_key();

    let mut ids = CountingIds(95);
    let rich = rich_skeleton(
        &mut ids,
        signer.signing_key_id(),
        &sealer.encryption_key_id().hex(),
    );
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&rich)
        .expect("the fixture skeleton serializes");
    let plaintext = OverlayPlaintext::canonical(
        &rich.receipt_id,
        PlanReceipt::TOKEN,
        span.as_bytes(),
        &[OverlayEntry::of(0, OpaqueFieldTag::Argv, b"argv".to_vec())],
    );
    let rich_bytes = DraftReceipt::<PlanReceipt, Rich>::of(rich.clone())
        .serialize(plaintext, &sealer)
        .expect("a rich document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();

    let mut remint_ids = CountingIds(200);
    let plain = dorc_receipt::projection::narrow_to_plain(&rich, &mut remint_ids, remint_order())
        .expect("the narrowing holds");
    assert_ne!(
        plain.receipt_id, rich.receipt_id,
        "a remint mints its own identity like any other document"
    );
    assert_ne!(
        plain.order, rich.order,
        "and takes the order of the moment it was written, not its origin's — two documents \
         sharing a store position would read as a tie between one document and its own remint"
    );
    let plain_bytes = DraftReceipt::<PlanReceipt, Plain>::of(plain)
        .serialize()
        .expect("the narrowed document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();
    assert_ne!(rich_bytes, plain_bytes);

    // One identity, differing bytes, stays a finding with no carve-out to escape through.
    let mut forged = rich_bytes.clone();
    let last = forged.len().saturating_sub(2);
    forged[last] = if forged[last] == b'a' { b'b' } else { b'a' };
    assert_eq!(
        dorc_receipt::projection::same_identity_pair(&rich_bytes, &forged),
        dorc_receipt::projection::SameIdentityPair::Divergent
    );
    assert_eq!(
        dorc_receipt::projection::same_identity_pair(&rich_bytes, &rich_bytes),
        dorc_receipt::projection::SameIdentityPair::Identical
    );
}

#[test]
fn a_rich_document_and_its_plain_remint_are_two_nodes_and_no_finding() {
    // A remint is a new document with its own identity, so the graph holds two nodes and owes
    // a reader nothing: no collision, because nothing collided, and no correlation, because
    // the two carry no recorded link to each other.
    //
    // Its sibling in the pure crate's graph corpus
    // (`two_documents_claiming_one_identity_are_both_retained_as_a_finding`) is the case that
    // DOES fire: one identity, differing bytes. The two differ by whether the second document
    // minted its own identity, which is the only thing that ever made them different cases.
    let identity = age::x25519::Identity::generate();
    let sealer = AgeSealer::of(identity.to_public());
    let opener = AgeOpener::of(identity);
    let signer = signing_key();
    let verifier = material().expect("the fixture verification material is well formed");
    let resolver = PolicyNames(verifier);

    let mut ids = CountingIds(150);
    let rich = rich_skeleton(
        &mut ids,
        signer.signing_key_id(),
        &sealer.encryption_key_id().hex(),
    );
    let span = dorc_receipt::format::serialize_skeleton::<PlanReceipt, Rich>(&rich)
        .expect("the fixture skeleton serializes");
    let plaintext = OverlayPlaintext::canonical(
        &rich.receipt_id,
        PlanReceipt::TOKEN,
        span.as_bytes(),
        &[OverlayEntry::of(0, OpaqueFieldTag::Argv, b"argv".to_vec())],
    );
    let rich_bytes = DraftReceipt::<PlanReceipt, Rich>::of(rich.clone())
        .serialize(plaintext, &sealer)
        .expect("a rich document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();

    let mut remint_ids = CountingIds(210);
    let plain = dorc_receipt::projection::narrow_to_plain(&rich, &mut remint_ids, remint_order())
        .expect("the narrowing holds");
    let plain_bytes = DraftReceipt::<PlanReceipt, Plain>::of(plain)
        .serialize()
        .expect("the narrowed document serializes")
        .sign(&signer)
        .bytes()
        .to_vec();
    assert_ne!(
        rich_bytes, plain_bytes,
        "the two projections must really differ, or this case proves nothing"
    );

    let mut graph = dorc_receipt::graph::ReceiptGraph::new();
    let trust = dorc_receipt::tokens::RecordedSignerTrust::Trusted;
    let rich_document =
        read_rich::<PlanReceipt>(rich_bytes.clone(), &ReceiptLimits::V1, &resolver, &opener)
            .expect("the rich document reads");
    graph.ingest_plan(&rich_document, trust, &rich_bytes);
    let plain_document =
        read_plain::<PlanReceipt>(plain_bytes.clone(), &ReceiptLimits::V1, &resolver)
            .expect("the reminted document reads");
    graph.ingest_plan(&plain_document, trust, &plain_bytes);

    assert_eq!(
        graph.plans().len(),
        2,
        "a remint is a second document, so it is a second node"
    );
    assert!(
        graph.collisions().is_empty(),
        "two identities cannot collide"
    );
    assert!(
        graph.findings().is_empty(),
        "and it owes a reader no finding: {:?}",
        graph.findings()
    );
    assert!(graph.faults().is_empty());
}
