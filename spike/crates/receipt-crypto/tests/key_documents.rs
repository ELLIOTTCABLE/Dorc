//! The two canonical private key documents: their exact encodings, their refusals, and the
//! independence of the two roles.
//!
//! The signing half is anchored on a PUBLISHED standard test vector — RFC 8032's first Ed25519
//! case, whose secret, public key, and signature over the empty message are printed in the
//! standard and reproduced in the signature package's own suite. Nothing here was generated to
//! be goldened: a vector minted by this project would prove only that the project agrees with
//! itself, which is exactly what a canonical-encoding test must not rest on.
//!
//! The encryption half has no comparable published identity vector, so it is anchored the other
//! way: every identity it uses is generated inside the test, and what is asserted is the SHAPE
//! the package writes and the byte equality of a round trip. No ciphertext and no generated
//! secret is committed anywhere in this file.

#![expect(
    clippy::expect_used,
    reason = "an integration test crate is an ordinary crate to clippy, so the central \
              allow-in-tests keys do not reach it; see spike/clippy.toml"
)]

use dorc_receipt::capability::{
    OverlayOpener as _, OverlaySealer as _, ReceiptSigner as _, ReceiptVerifier as _,
};
use dorc_receipt::ids::from_hex_32;
use dorc_receipt_crypto::{
    EncryptionPrivateDocument, EntropyKeysetGenerator, KeyDocumentBound, KeyDocumentRefusal,
    KeySecretEntropy, KeysetGenerator, LineDeparture, SigningPrivateDocument,
};

/// The V1 bound both documents are read under.
const BOUND: KeyDocumentBound = KeyDocumentBound::of(256);

/// RFC 8032 section 7.1, TEST 1. A PUBLISHED standard vector, not material minted here: it is
/// printed in the standard, shipped in the signature package's own test corpus, and reproduced
/// in every Ed25519 implementation's suite. Its whole value is that this project did not choose
/// it, so agreeing with it is evidence about the library rather than about ourselves.
const RFC_8032_TEST_1_PUBLISHED_SECRET: &str =
    "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";

/// The public key RFC 8032 publishes for that secret.
const RFC_8032_TEST_1_PUBLISHED_PUBLIC: &str =
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";

/// The signature RFC 8032 publishes for that key over the EMPTY message.
const RFC_8032_TEST_1_PUBLISHED_SIGNATURE: &str = concat!(
    "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e0652249015",
    "55fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
);

/// The ASN.1 prologue of an RFC 8410 Ed25519 private key: `SEQUENCE`, then the version integer,
/// then the algorithm identifier naming OID 1.3.101.112 with absent parameters, then the OCTET
/// STRING that wraps a second OCTET STRING around the seed.
///
/// Spelled out rather than goldened as an opaque blob, because the point of the case is that the
/// bytes the library writes are the STANDARD's bytes: a reader can check these against RFC 8410
/// and a golden would only say that today matches today.
const DER_VERSION_TWO: [u8; 3] = [0x02, 0x01, 0x01];
const DER_VERSION_ONE: [u8; 3] = [0x02, 0x01, 0x00];
const DER_ED25519_ALGORITHM: [u8; 7] = [0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70];
const DER_NESTED_SEED_OCTETS: [u8; 4] = [0x04, 0x22, 0x04, 0x20];
const DER_IMPLICIT_PUBLIC_BIT_STRING: [u8; 3] = [0x81, 0x21, 0x00];

fn secret() -> [u8; 32] {
    from_hex_32(RFC_8032_TEST_1_PUBLISHED_SECRET).expect("the published secret is 32 bytes")
}

fn public() -> [u8; 32] {
    from_hex_32(RFC_8032_TEST_1_PUBLISHED_PUBLIC).expect("the published public key is 32 bytes")
}

fn published_signature() -> [u8; 64] {
    let hex = RFC_8032_TEST_1_PUBLISHED_SIGNATURE.as_bytes();
    assert_eq!(hex.len(), 128, "a signature is 64 bytes of hexadecimal");
    let mut out = [0_u8; 64];
    for (slot, pair) in out.iter_mut().zip(hex.chunks_exact(2)) {
        let text = core::str::from_utf8(pair).expect("hexadecimal is ascii");
        *slot = u8::from_str_radix(text, 16).expect("the published signature is hexadecimal");
    }
    out
}

/// A `SEQUENCE` header for `content`, and the content.
fn der_sequence(content: &[u8]) -> Vec<u8> {
    assert!(content.len() < 128, "the fixture bodies are short-form");
    let mut out = vec![0x30, u8::try_from(content.len()).expect("short form")];
    out.extend_from_slice(content);
    out
}

/// The version-2 document: seed and public key, which is what the library writes.
fn published_document() -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&DER_VERSION_TWO);
    body.extend_from_slice(&DER_ED25519_ALGORITHM);
    body.extend_from_slice(&DER_NESTED_SEED_OCTETS);
    body.extend_from_slice(&secret());
    body.extend_from_slice(&DER_IMPLICIT_PUBLIC_BIT_STRING);
    body.extend_from_slice(&public());
    der_sequence(&body)
}

/// The version-1 document: seed alone. Valid RFC 8410, and NOT what the library writes.
fn published_document_without_public_key() -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&DER_VERSION_ONE);
    body.extend_from_slice(&DER_ED25519_ALGORITHM);
    body.extend_from_slice(&DER_NESTED_SEED_OCTETS);
    body.extend_from_slice(&secret());
    der_sequence(&body)
}

#[test]
fn the_signing_document_is_the_standard_encoding_of_a_published_key() {
    // Three claims in one run, and each is about the LIBRARY rather than about this code: the
    // encoding it writes is the standard structure, the key it derives from the standard's seed
    // is the standard's public key, and reading a document back gives the same material. The
    // parse is itself evidence of the second: the package checks the embedded public key against
    // the one it derives, so a document carrying a public key the seed does not produce is
    // refused rather than accepted with the wrong half.
    let expected = published_document();
    let document = SigningPrivateDocument::parse(&expected, BOUND).expect("the vector parses");
    assert_eq!(document.verification_material(), public());
    let written = document
        .with_canonical_bytes(<[u8]>::to_vec)
        .expect("the document encodes");
    assert_eq!(written, expected, "the library writes the standard's bytes");
}

#[test]
fn the_signature_over_the_published_message_is_the_published_signature() {
    // The interop claim proper. Structure round-tripping proves the container; this proves the
    // primitive underneath it is the one the standard describes, over the standard's own input.
    let document =
        SigningPrivateDocument::parse(&published_document(), BOUND).expect("the vector parses");
    let signature = document.sign(b"");
    assert_eq!(signature, published_signature());

    let verifier = document.verifier().expect("the public half loads");
    assert!(verifier.verify(b"", &signature));

    // The negative half, pinned to the same key and message so the refusal is caused by the
    // edited signature and by nothing else about the fixture.
    let mut edited = signature;
    if let Some(first) = edited.first_mut() {
        *first ^= 0x01;
    }
    assert!(!verifier.verify(b"", &edited));
    assert!(
        !verifier.verify(b"x", &signature),
        "the signature is over the empty message and no other"
    );
}

#[test]
fn a_valid_document_the_library_would_write_differently_is_refused_as_non_canonical() {
    // The sharpest refusal on this side, and the one a permissive reader would let through: the
    // version-1 form is legal RFC 8410 and the package PARSES it. What refuses it is the round
    // trip — the package would write the version-2 form — so the two refusals stay separate,
    // because "damaged" and "a shape we do not write" are different things to have found on
    // disk. The positive control below is the same seed in the form the package does write.
    let v1 = published_document_without_public_key();
    assert_eq!(
        SigningPrivateDocument::parse(&v1, BOUND).err(),
        Some(KeyDocumentRefusal::NonCanonical)
    );
    assert!(
        SigningPrivateDocument::parse(&published_document(), BOUND).is_ok(),
        "the same key in the written form is accepted, so the refusal above is the form"
    );
}

#[test]
fn a_damaged_signing_document_is_refused_as_malformed() {
    let whole = published_document();
    let mut truncated = whole.clone();
    truncated.pop();
    for (what, bytes) in [
        ("truncated", truncated),
        ("trailing byte", [whole.clone(), vec![0x00]].concat()),
        ("empty", Vec::new()),
        ("not der at all", b"-----BEGIN PRIVATE KEY-----\n".to_vec()),
    ] {
        assert_eq!(
            SigningPrivateDocument::parse(&bytes, BOUND).err(),
            Some(KeyDocumentRefusal::Malformed),
            "{what}"
        );
    }
}

#[test]
fn a_signing_document_whose_public_half_disagrees_with_its_seed_is_refused() {
    // The check belongs to the package and this asserts we are getting it: a document pairing a
    // real seed with somebody else's public key is the shape that would otherwise let a caller
    // sign under one identity while a manifest recorded another.
    let mut forged = published_document();
    if let Some(last) = forged.last_mut() {
        *last ^= 0x01;
    }
    assert_eq!(
        SigningPrivateDocument::parse(&forged, BOUND).err(),
        Some(KeyDocumentRefusal::Malformed)
    );
}

#[test]
fn the_bound_is_consulted_before_the_document_is_decoded() {
    // Boundary-at and boundary-plus. A document may be arbitrarily large, and the refusal has to
    // be the bound rather than whatever a decoder would have said about its first kilobyte.
    let whole = published_document();
    let exact = KeyDocumentBound::of(whole.len());
    assert!(SigningPrivateDocument::parse(&whole, exact).is_ok(), "at");
    let under = KeyDocumentBound::of(whole.len().saturating_sub(1));
    assert_eq!(
        SigningPrivateDocument::parse(&whole, under).err(),
        Some(KeyDocumentRefusal::OverBound),
        "one byte over"
    );
    assert_eq!(
        EncryptionPrivateDocument::parse(b"anything\n", KeyDocumentBound::of(3)).err(),
        Some(KeyDocumentRefusal::OverBound),
        "the same rule on the other role"
    );
}

/// A source answering fixed bytes, so a case can hold the signing half still while the
/// encryption half moves. Fixture material by construction: an integration test is not compiled
/// into any library.
struct FixedSecret(u8);

impl KeySecretEntropy for FixedSecret {
    fn fill(&mut self, raw: &mut [u8; 32]) -> bool {
        raw.fill(self.0);
        true
    }
}

/// One generated keyset.
fn generated(seed: u8) -> dorc_receipt_crypto::GeneratedKeysetV1 {
    EntropyKeysetGenerator::over(FixedSecret(seed))
        .generate()
        .expect("the fixture source answers")
}

#[test]
fn the_encryption_document_round_trips_through_its_canonical_line() {
    let keyset = generated(11);
    let written = keyset.encryption().with_canonical_bytes(<[u8]>::to_vec);

    let text = String::from_utf8(written.clone()).expect("the document is text");
    assert!(text.ends_with('\n'), "one final newline");
    assert!(!text.contains('\r'), "no carriage return");
    assert_eq!(text.lines().count(), 1, "exactly one line");
    let line = text.trim_end_matches('\n');
    assert_eq!(line, line.trim(), "no surrounding whitespace");
    assert_eq!(
        line,
        line.to_uppercase(),
        "the package writes the identity in one case, and a second case would spell one \
         identity two ways on a filesystem that folds them"
    );

    let read = EncryptionPrivateDocument::parse(&written, BOUND).expect("it reads back");
    assert_eq!(
        read.recipient_text(),
        keyset.encryption().recipient_text(),
        "the same public half"
    );
    assert_eq!(
        read.encryption_key_id().hex(),
        keyset.encryption().encryption_key_id().hex()
    );
    assert_eq!(
        read.with_canonical_bytes(<[u8]>::to_vec),
        written,
        "and writes the same bytes again"
    );
}

#[test]
fn a_reloaded_encryption_document_opens_what_its_own_sealer_wrote() {
    // The semantic half of the round trip: the reloaded document is not merely byte-equal, it is
    // the same key. Sealing is randomized, so nothing here pins ciphertext or its length — three
    // seals of one payload are three different regions of three different sizes.
    let keyset = generated(12);
    let written = keyset.encryption().with_canonical_bytes(<[u8]>::to_vec);
    let read = EncryptionPrivateDocument::parse(&written, BOUND).expect("it reads back");

    let payload = b"one region's exact bytes".to_vec();
    let region = read.sealer().seal(&payload).expect("it seals");
    assert_eq!(read.open(&region, 1024), Some(payload.clone()));
    assert_eq!(
        keyset.encryption().open(&region, 1024),
        Some(payload),
        "the document it was loaded from opens it too"
    );

    let other = generated(13);
    assert!(
        other.encryption().open(&region, 1024).is_none(),
        "and another keyset's identity does not"
    );
}

#[test]
fn a_reloaded_signing_document_signs_as_the_key_it_was_written_from() {
    let keyset = generated(14);
    let written = keyset
        .signing()
        .with_canonical_bytes(<[u8]>::to_vec)
        .expect("it encodes");
    let read = SigningPrivateDocument::parse(&written, BOUND).expect("it reads back");
    assert_eq!(
        read.signing_key_id().hex(),
        keyset.signing().signing_key_id().hex()
    );
    // Signing is deterministic for this scheme, so equality of the two signatures is a real
    // statement that the two documents hold one key rather than two that both verify.
    assert_eq!(read.sign(b"body"), keyset.signing().sign(b"body"));
}

#[test]
fn every_departure_from_one_canonical_line_is_refused_with_its_own_name() {
    let keyset = generated(15);
    let canonical = keyset.encryption().with_canonical_bytes(<[u8]>::to_vec);
    let line = String::from_utf8(canonical.clone())
        .expect("text")
        .trim_end_matches('\n')
        .to_owned();

    for (departure, bytes) in [
        (
            LineDeparture::CarriageReturn,
            format!("{line}\r\n").into_bytes(),
        ),
        (LineDeparture::NoFinalNewline, line.clone().into_bytes()),
        (
            LineDeparture::MoreThanOneLine,
            format!("{line}\n{line}\n").into_bytes(),
        ),
        (
            LineDeparture::MoreThanOneLine,
            format!("\n{line}\n").into_bytes(),
        ),
        (LineDeparture::Empty, b"\n".to_vec()),
        (LineDeparture::Comment, format!("# {line}\n").into_bytes()),
        (
            LineDeparture::SurroundingWhitespace,
            format!(" {line}\n").into_bytes(),
        ),
        (
            LineDeparture::SurroundingWhitespace,
            format!("{line} \n").into_bytes(),
        ),
        (LineDeparture::NotText, vec![0xff, b'\n']),
    ] {
        assert_eq!(
            EncryptionPrivateDocument::parse(&bytes, BOUND).err(),
            Some(KeyDocumentRefusal::NotOneCanonicalLine { departure }),
            "{departure:?}"
        );
    }
    assert!(
        EncryptionPrivateDocument::parse(&canonical, BOUND).is_ok(),
        "the unedited document reads, so every refusal above is the edit"
    );
}

#[test]
fn a_case_folded_identity_is_refused_rather_than_normalized() {
    // The encryption role's version of the non-canonical case. The identity alphabet folds, so a
    // lowercase spelling names the same key; accepting it would put two spellings of one
    // identity into a store that has to compare them, and repairing it would be this crate
    // deciding what a key document says.
    let keyset = generated(16);
    let canonical = keyset.encryption().with_canonical_bytes(<[u8]>::to_vec);
    let folded = String::from_utf8(canonical)
        .expect("text")
        .to_lowercase()
        .into_bytes();
    assert_eq!(
        EncryptionPrivateDocument::parse(&folded, BOUND).err(),
        Some(KeyDocumentRefusal::NonCanonical)
    );
}

#[test]
fn a_damaged_identity_is_refused_as_malformed() {
    let keyset = generated(17);
    let line = String::from_utf8(keyset.encryption().with_canonical_bytes(<[u8]>::to_vec))
        .expect("text")
        .trim_end_matches('\n')
        .to_owned();
    for (what, text) in [
        ("one character changed", {
            let mut edited = line.clone().into_bytes();
            if let Some(last) = edited.last_mut() {
                *last = if *last == b'Q' { b'P' } else { b'Q' };
            }
            String::from_utf8(edited).expect("text")
        }),
        ("truncated", {
            let mut shortened = line.clone();
            shortened.truncate(line.len().saturating_sub(4));
            shortened
        }),
        ("some other word", "NOT-AN-IDENTITY".to_owned()),
    ] {
        assert_eq!(
            EncryptionPrivateDocument::parse(format!("{text}\n").as_bytes(), BOUND).err(),
            Some(KeyDocumentRefusal::Malformed),
            "{what}"
        );
    }
}

#[test]
fn the_two_roles_are_generated_independently_and_neither_follows_the_other() {
    // The load-bearing separation claim, measured rather than asserted. The signing half is held
    // FIXED across two generations by a source answering the same bytes; if the encryption half
    // were derived from it, or from a shared root, the two encryption identities would agree.
    // They must not, and the signing ones must.
    let first = generated(21);
    let second = generated(21);
    assert_eq!(
        first.signing().signing_key_id().hex(),
        second.signing().signing_key_id().hex(),
        "the source answered the same signing bytes, so this half is genuinely held still"
    );
    assert_ne!(
        first.encryption().encryption_key_id().hex(),
        second.encryption().encryption_key_id().hex(),
        "one fixed signing secret must not determine the encryption identity"
    );

    // And the reverse direction, which no single generation can show: a different signing secret
    // beside independently drawn encryption halves.
    let third = generated(22);
    assert_ne!(
        first.signing().signing_key_id().hex(),
        third.signing().signing_key_id().hex()
    );
    assert_ne!(
        first.encryption().encryption_key_id().hex(),
        third.encryption().encryption_key_id().hex()
    );
}

#[test]
fn the_two_role_identities_never_collide() {
    // Value-level non-aliasing, beside the type-level pins in the crate's own documentation. The
    // two identities are derived under separate domains, so even material that somehow agreed
    // would not produce one digest.
    for seed in 0_u8..8 {
        let keyset = generated(seed);
        assert_ne!(
            keyset.signing().signing_key_id().hex(),
            keyset.encryption().encryption_key_id().hex()
        );
    }
}

#[test]
fn no_document_prints_its_own_material() {
    // The check the fixture rules ask for directly: a document's rendered form carries its public
    // identity and nothing else. Measured against the real canonical bytes rather than against a
    // pattern, so a future field that happened to include them fails here.
    let keyset = generated(31);
    let signing_bytes = keyset
        .signing()
        .with_canonical_bytes(<[u8]>::to_vec)
        .expect("it encodes");
    let identity = String::from_utf8(keyset.encryption().with_canonical_bytes(<[u8]>::to_vec))
        .expect("text")
        .trim_end_matches('\n')
        .to_owned();
    let secret_hex = dorc_receipt::ids::to_hex(&signing_bytes);

    for rendered in [
        format!("{:?}", keyset.signing()),
        format!("{:?}", keyset.encryption()),
        format!("{keyset:?}"),
    ] {
        assert!(!rendered.contains(&identity), "{rendered}");
        assert!(!rendered.contains(&secret_hex), "{rendered}");
        assert!(
            rendered.contains(&keyset.signing().signing_key_id().hex())
                || rendered.contains(&keyset.encryption().encryption_key_id().hex()),
            "a redacted rendering still names which keyset it is: {rendered}"
        );
    }
}

#[test]
fn a_generated_keyset_is_the_material_its_own_documents_encode() {
    // The whole-generation round trip, so nothing in the sequence D2 will run depends on a
    // document being re-derivable from something other than the bytes it wrote.
    let keyset = generated(41);
    let signing_id = keyset.signing().signing_key_id().hex();
    let encryption_id = keyset.encryption().encryption_key_id().hex();
    let (signing, encryption) = keyset.into_parts();

    let signing_bytes = signing
        .with_canonical_bytes(<[u8]>::to_vec)
        .expect("it encodes");
    let encryption_bytes = encryption.with_canonical_bytes(<[u8]>::to_vec);
    assert!(
        signing_bytes.len() <= BOUND.bytes() && encryption_bytes.len() <= BOUND.bytes(),
        "both documents fit the bound they are read under: {} and {}",
        signing_bytes.len(),
        encryption_bytes.len()
    );

    assert_eq!(
        SigningPrivateDocument::parse(&signing_bytes, BOUND)
            .expect("it reads")
            .signing_key_id()
            .hex(),
        signing_id
    );
    assert_eq!(
        EncryptionPrivateDocument::parse(&encryption_bytes, BOUND)
            .expect("it reads")
            .encryption_key_id()
            .hex(),
        encryption_id
    );
}

#[test]
fn the_signing_identity_is_derived_from_public_material_and_not_from_the_document() {
    // Where a provider identity comes from is a security property rather than a convenience: an
    // identity derived from private serialization would put a function of the secret into a
    // manifest that is deliberately world-readable structural metadata. This pins it positively
    // — the same public material derives the same identity through the pure crate's own mint,
    // reached without any document at all.
    let keyset = generated(51);
    assert_eq!(
        keyset.signing().signing_key_id().hex(),
        dorc_receipt::ids::SigningKeyId::of_public_material(
            &keyset.signing().verification_material()
        )
        .hex()
    );
    assert_eq!(
        keyset.encryption().encryption_key_id().hex(),
        dorc_receipt::ids::EncryptionKeyId::of_recipient_material(
            keyset.encryption().recipient_text().as_bytes()
        )
        .hex()
    );
}
