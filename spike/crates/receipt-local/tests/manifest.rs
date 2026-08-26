//! The keyset-manifest corpus: every committed vector, bound to the exact refusal it earns.
//!
//! The vectors under `tests/vectors/manifest/` are reviewed fixtures, hand-written. Nothing
//! regenerates them: a corpus a tool can rewrite proves whatever the tool currently does.
//!
//! Every negative is pinned to its OWN refusal rather than to "it was refused". Several of these
//! fail closed in ways that look alike from outside — a missing line and a reordered pair both
//! stop the same parse — so a vector that drifted onto a neighbouring reason would keep passing
//! while testing nothing.

use std::path::{Path, PathBuf};

use dorc_receipt_local::limits::LocalLimits;
use dorc_receipt_local::manifest::{KeyRole, KeysetManifest, ManifestRefusal};

fn vectors(kind: &str) -> Vec<(String, Vec<u8>)> {
    let root: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("vectors")
        .join("manifest")
        .join(kind);
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_owned();
            Some((name, std::fs::read(&path).ok()?))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    // A walk that found nothing would otherwise pass silently.
    assert!(
        !out.is_empty(),
        "no {kind} vectors under {}",
        root.display()
    );
    out
}

/// Every invalid vector, bound to the exact refusal its own departure must produce.
const EXPECTED: &[(&str, ManifestRefusal)] = &[
    (
        "blank-line.keyset",
        ManifestRefusal::Structure {
            what: "encryption-key-id",
        },
    ),
    ("bytes-after-end.keyset", ManifestRefusal::BytesAfterEnd),
    (
        "carriage-returns.keyset.crlf",
        ManifestRefusal::IllegalByte { byte: b'\r' },
    ),
    (
        "comment.keyset",
        ManifestRefusal::Structure {
            what: "signing-key-id",
        },
    ),
    (
        "double-space.keyset",
        ManifestRefusal::IdentityNotExactDigest {
            role: KeyRole::Signing,
        },
    ),
    (
        "encryption-identity-uppercase.keyset",
        ManifestRefusal::IdentityNotExactDigest {
            role: KeyRole::Encryption,
        },
    ),
    (
        "encryption-role-absent.keyset",
        ManifestRefusal::Structure {
            what: "encryption-key-id",
        },
    ),
    (
        "missing-final-newline.keyset",
        ManifestRefusal::BytesAfterEnd,
    ),
    (
        "no-terminator.keyset",
        ManifestRefusal::Structure { what: "keyset-end" },
    ),
    (
        "roles-reordered.keyset",
        ManifestRefusal::Structure {
            what: "signing-key-id",
        },
    ),
    (
        "signing-identity-short.keyset",
        ManifestRefusal::IdentityNotExactDigest {
            role: KeyRole::Signing,
        },
    ),
    (
        "tab-separator.keyset",
        ManifestRefusal::IllegalByte { byte: b'\t' },
    ),
    (
        "trailing-space.keyset",
        ManifestRefusal::IdentityNotExactDigest {
            role: KeyRole::Signing,
        },
    ),
    (
        "unsupported-version.keyset",
        ManifestRefusal::UnsupportedVersion,
    ),
];

#[test]
fn every_valid_vector_parses_and_reserializes_to_the_same_bytes() {
    // Byte equality is this format's equality relation too: one writer form, one reader form.
    let limits = LocalLimits::V1;
    let mut failures: Vec<String> = Vec::new();
    for (name, bytes) in vectors("valid") {
        match KeysetManifest::parse(&bytes, &limits) {
            Ok(manifest) if manifest.serialize().as_bytes() == bytes.as_slice() => {}
            Ok(_) => failures.push(format!("{name}: did not round-trip byte-for-byte")),
            Err(refusal) => failures.push(format!("{name}: {refusal:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn every_invalid_vector_is_refused_for_exactly_its_own_departure() {
    // Total in both directions: a vector with no row is unaccounted for, and a row with no vector
    // is a table that outlived the file it describes.
    let limits = LocalLimits::V1;
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
        match KeysetManifest::parse(&bytes, &limits) {
            Err(got) if got == *want => {}
            other => failures.push(format!("{name}: wanted {want:?}, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn the_manifest_claims_two_identities_and_never_one_under_two_names() {
    // The cross-file agreement's own precondition: the two roles are read from separate lines
    // into separate slots, so a manifest cannot claim one identity for both by accident of
    // parsing.
    let manifest = KeysetManifest::of(&"a".repeat(64), &"b".repeat(64)).expect("well formed");
    assert_ne!(
        manifest.claimed(KeyRole::Signing),
        manifest.claimed(KeyRole::Encryption)
    );
    assert_eq!(manifest.claimed(KeyRole::Signing), "a".repeat(64));
    assert_eq!(manifest.claimed(KeyRole::Encryption), "b".repeat(64));
}
