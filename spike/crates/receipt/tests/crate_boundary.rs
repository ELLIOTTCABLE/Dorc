//! The dependency-edge fence: which crates may name which, read off the manifests.
//!
//! No type can express "this crate has no dependency", so this is a lexical gate over the
//! workspace's own manifests. It walks a non-empty set and names the crates it found, so a
//! walk that silently found nothing fails rather than passing.

use std::path::{Path, PathBuf};

fn crates_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(PathBuf::new, Path::to_path_buf)
}

/// The crates permitted to name the implementation crate, which carries the randomness.
///
/// Checked BOTH ways: a crate outside this list naming it fails, and an entry that stops
/// naming it fails too. An anticipatory entry is the failure mode the second direction
/// exists for — it reads as a fence while permitting a crate nothing has yet checked.
///
/// EMPTY today: the implementation crate exists and nothing depends on it yet, its own tests
/// being its only consumer. The stage that first wires a production caller adds its entry
/// here in the same commit as the manifest line, which is the act being made visible.
const MAY_NAME_IT: [&str; 0] = [];

fn manifests() -> Vec<(String, String)> {
    let root = crates_root();
    let mut found: Vec<(String, String)> = std::fs::read_dir(&root)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let manifest = entry.path().join("Cargo.toml");
            let name = entry.path().file_name()?.to_str()?.to_owned();
            let text = std::fs::read_to_string(&manifest).ok()?;
            Some((name, text))
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        found.len() > 5,
        "the manifest walk found {} crates; it is looking in the wrong place",
        found.len()
    );
    found
}

/// The dependency lines of one manifest, ignoring comment text so a rule that is merely
/// described does not read as a rule that is declared.
fn dependency_lines(manifest: &str) -> Vec<&str> {
    manifest
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect()
}

#[test]
fn the_pure_receipt_crate_names_no_other_workspace_crate() {
    // Its whole reason for existing is that the kernel may depend on it, which holds only
    // while it depends on nothing of ours and nothing nondeterministic.
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .unwrap_or_default();
    for line in dependency_lines(&manifest) {
        assert!(
            !line.starts_with("dorc-")
                && !line.starts_with("weft")
                && !line.starts_with("errorloom"),
            "the pure receipt crate named a workspace crate: {line}"
        );
    }
    for forbidden in ["age", "ed25519", "rand", "getrandom"] {
        assert!(
            !dependency_lines(&manifest)
                .iter()
                .any(|line| line.starts_with(forbidden)),
            "the pure receipt crate named {forbidden}"
        );
    }
    assert!(
        dependency_lines(&manifest)
            .iter()
            .any(|line| line.starts_with("sha2")),
        "the one dependency it does carry went missing"
    );
}

/// Whether `text` reaches into `krate`'s namespace, matching the crate name as a WHOLE
/// identifier.
///
/// A bare `contains("age::")` also matches every type whose name merely ends in those letters —
/// `ApplyArtifactImage::`, `Message::`, `Storage::` — so the fence would refuse ordinary code
/// while still catching nothing it does not catch here. A fence that fires on a name is a fence
/// people route around.
fn names_crate(text: &str, krate: &str) -> bool {
    let needle = format!("{krate}::");
    let mut from = 0_usize;
    while let Some(offset) = text.get(from..).and_then(|rest| rest.find(&needle)) {
        let at = from.saturating_add(offset);
        let preceded = text
            .get(..at)
            .and_then(|before| before.bytes().next_back())
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
        if !preceded {
            return true;
        }
        from = at.saturating_add(needle.len());
    }
    false
}

#[test]
fn the_namespace_fence_matches_a_crate_and_not_a_name_that_merely_ends_in_one() {
    // The fence's own regression: it exists to catch a reach into `age`, and a type called
    // `ApplyArtifactImage` is not one.
    assert!(names_crate("use age::Encryptor;", "age"));
    assert!(names_crate("let x = age::armor();", "age"));
    assert!(names_crate("age::Foo", "age"));
    assert!(!names_crate("ApplyArtifactImage::of_parts(...)", "age"));
    assert!(!names_crate("Message::new()", "age"));
    assert!(!names_crate("crate::storage::of()", "age"));
    assert!(names_crate("(ed25519_dalek::Signature)", "ed25519_dalek"));
}

#[test]
fn no_source_file_of_the_pure_crate_names_a_crypto_package() {
    // The manifest fence and this one answer different questions: one is what the crate may
    // link, the other is what its code reaches for.
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut checked = 0_u32;
    for entry in std::fs::read_dir(&src).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        for token in ["dorc_receipt_crypto", "ed25519_dalek", "age"] {
            assert!(
                !names_crate(&text, token),
                "{} reaches for {token}",
                path.display()
            );
        }
        checked = checked.saturating_add(1);
    }
    assert!(checked > 5, "the source walk checked only {checked} files");
}

#[test]
fn only_the_edge_may_name_the_crypto_implementation_crate() {
    // The implementation crate carries the randomness, so the set of crates allowed to name
    // it is an allow-list rather than a convention. Adding an entry here is the visible act.
    let mut named_by: Vec<String> = Vec::new();
    for (crate_dir, manifest) in manifests() {
        if dependency_lines(&manifest)
            .iter()
            .any(|line| line.starts_with("dorc-receipt-crypto"))
        {
            named_by.push(crate_dir);
        }
    }
    for name in &named_by {
        assert!(
            MAY_NAME_IT.contains(&name.as_str()),
            "{name} names the crypto implementation crate; only {MAY_NAME_IT:?} may"
        );
    }
}

#[test]
fn the_crypto_crate_names_the_pure_crate_and_no_other_workspace_crate() {
    let manifest = crates_root().join("receipt-crypto").join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).unwrap_or_default();
    let lines = dependency_lines(&text);
    assert!(
        lines.iter().any(|line| line.starts_with("dorc-receipt ")),
        "the crypto crate must name the pure crate"
    );
    for line in lines {
        if line.starts_with("dorc-") {
            assert!(
                line.starts_with("dorc-receipt "),
                "the crypto crate named another workspace crate: {line}"
            );
        }
    }
}

#[test]
fn the_armor_feature_is_named_explicitly() {
    // Canonical armor is not a default feature of that package, and the grammar requires it,
    // so a manifest that stopped naming it would fail at seal time rather than here.
    let manifest = crates_root().join("receipt-crypto").join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).unwrap_or_default();
    let age_line = dependency_lines(&text)
        .into_iter()
        .find(|line| line.starts_with("age "))
        .unwrap_or_default();
    assert!(
        age_line.contains("\"armor\""),
        "armor must be named: {age_line}"
    );
    assert!(
        age_line.contains("default-features = false"),
        "the unused surfaces stay off: {age_line}"
    );
    for surface in ["plugin", "ssh", "async", "cli-common", "unstable"] {
        assert!(
            !age_line.contains(&format!("\"{surface}\"")),
            "{surface} must stay off: {age_line}"
        );
    }
}

/// The files permitted to carry fixture key material, by crate-relative path.
///
/// Narrow to the file, not the crate, and two-way: an entry naming a file that no longer
/// carries any is as much a failure as material in a file with no entry. Adding a row is the
/// visible act.
const MAY_CARRY_FIXTURE_KEY_MATERIAL: [&str; 1] = ["receipt-crypto/tests/crypto_interop.rs"];

/// Every `.rs` file under the workspace's crates, as crate-relative paths.
fn workspace_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, root, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let relative = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");
                out.push((relative, std::fs::read_to_string(&path).unwrap_or_default()));
            }
        }
    }
    let root = crates_root();
    let mut out = Vec::new();
    walk(&root, &root, &mut out);
    out
}

#[test]
fn the_fixture_identity_is_unreachable_from_production() {
    // A private key committed to seal frozen vectors has exactly one legitimate home. No type
    // can say "this constant is not reachable from a shipped path", so the fence is lexical:
    // find every file carrying key material and require it to be one of the listed test files.
    // The walk asserts it found files, because a walk that matched nothing would pass while
    // proving nothing — the same failure shape as a corpus that loads zero vectors.
    let sources = workspace_sources();
    assert!(
        sources.len() > 20,
        "the source walk found only {} files; it is looking in the wrong place",
        sources.len()
    );

    let carriers: Vec<String> = sources
        .iter()
        // Spelled in halves so this file does not match its own search and report itself.
        .filter(|(_, text)| text.contains(concat!("AGE-SECRET", "-KEY-")))
        .map(|(path, _)| path.clone())
        .collect();

    for path in &carriers {
        assert!(
            MAY_CARRY_FIXTURE_KEY_MATERIAL.contains(&path.as_str()),
            "{path} carries fixture key material and is not on the list"
        );
    }
    for allowed in MAY_CARRY_FIXTURE_KEY_MATERIAL {
        assert!(
            carriers.iter().any(|path| path == allowed),
            "{allowed} is listed but carries no key material; the entry is stale"
        );
    }
}

/// Every production source file in the workspace, keyed by a slash-normalized relative path.
///
/// Walks `crates/*/src` only: a test target is not a production boundary, and the whole point of
/// several fences below is the distinction. The walk asserts it found sources, so a fence that
/// silently looked in the wrong place fails rather than passing over an empty set.
fn production_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let joined = if prefix.is_empty() {
                name.to_owned()
            } else {
                format!("{prefix}/{name}")
            };
            if path.is_dir() {
                walk(&path, &joined, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push((joined, std::fs::read_to_string(&path).unwrap_or_default()));
            }
        }
    }

    let root = crates_root();
    let mut out: Vec<(String, String)> = Vec::new();
    for entry in std::fs::read_dir(&root).into_iter().flatten().flatten() {
        let src = entry.path().join("src");
        if !src.is_dir() {
            continue;
        }
        let Some(krate) = entry
            .path()
            .file_name()
            .and_then(|n| n.to_str().map(str::to_owned))
        else {
            continue;
        };
        walk(&src, &format!("{krate}/src"), &mut out);
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        out.len() > 20,
        "the production source walk found {} files; it is looking in the wrong place",
        out.len()
    );
    out
}

/// Assert the set of production files mentioning `needle` is exactly `allowed`, both ways.
///
/// Two-way on purpose: a file that gains the mention fails, and an entry that no longer has it
/// fails too, so the list cannot rot into a description of what used to be true.
fn fence(needle: &str, allowed: &[&str]) {
    let found: Vec<String> = production_sources()
        .into_iter()
        .filter(|(_, text)| text.contains(needle))
        .map(|(path, _)| path)
        .collect();
    for path in &found {
        assert!(
            allowed.contains(&path.as_str()),
            "{path} names `{needle}`; only {allowed:?} may"
        );
    }
    for entry in allowed {
        assert!(
            found.iter().any(|path| path == entry),
            "the allow-list entry {entry} no longer names `{needle}`; remove it rather than \
             leaving a fence describing what used to be true"
        );
    }
}

#[test]
fn every_crate_allowed_to_name_the_crypto_implementation_still_names_it() {
    // The other direction of `only_the_edge_may_name_the_crypto_implementation_crate`. A stale
    // entry there would keep passing while quietly widening what the fence permits.
    let named_by: Vec<String> = manifests()
        .into_iter()
        .filter(|(_, manifest)| {
            dependency_lines(manifest)
                .iter()
                .any(|line| line.starts_with("dorc-receipt-crypto"))
        })
        .map(|(krate, _)| krate)
        .collect();
    assert_eq!(
        named_by.len(),
        MAY_NAME_IT.len(),
        "the set naming the crypto implementation crate is {named_by:?}; the allow-list is \
         {MAY_NAME_IT:?}, and the two must agree exactly"
    );
    for entry in MAY_NAME_IT {
        assert!(
            named_by.iter().any(|krate| krate == entry),
            "{entry} is allowed to name the crypto implementation crate and does not; the \
             allow-list has gone stale"
        );
    }
}

#[test]
fn the_identity_mint_is_reachable_from_one_production_file() {
    // A document identity is controller-minted per document. The seam is honest, but nothing in
    // the type stops a production file calling it with fixed bytes, so the gate over its callers
    // is lexical. `ids.rs` both declares it and drives it from its own fixture source, which
    // lives behind `#[cfg(test)]`.
    fence("of_source_bytes", &["receipt/src/ids.rs"]);
}

#[test]
fn verification_material_is_supplied_from_one_production_file() {
    // The resolver is the seam through which a permissive verifier could reach the reader, and a
    // fence covering only the crypto crate's NAME would not see one written elsewhere. Implementing
    // any of these in a production file is the visible act.
    for needle in [
        "impl VerificationKeyResolver",
        "impl TrustedReceiptVerificationKey",
        "impl SelfAssertedReceiptVerificationKey",
    ] {
        let found: Vec<String> = production_sources()
            .into_iter()
            .filter(|(_, text)| text.contains(needle))
            .map(|(path, _)| path)
            .collect();
        for path in &found {
            assert!(
                path == "receipt-crypto/src/lib.rs",
                "{path} carries `{needle}`; only the implementation crate may"
            );
        }
    }
    // Two-way: the implementation crate must still carry the two it is allowed to.
    fence(
        "impl TrustedReceiptVerificationKey",
        &["receipt-crypto/src/lib.rs"],
    );
    fence(
        "impl SelfAssertedReceiptVerificationKey",
        &["receipt-crypto/src/lib.rs"],
    );
}

#[test]
fn the_fixture_signature_stand_in_never_reaches_a_production_file() {
    // The graph corpus signs its documents with an inert deterministic stand-in. It is confined
    // by living in a test target; this asserts that rather than trusting it.
    for needle in ["inert_signature", "InertSigner", "InertKey"] {
        let found: Vec<String> = production_sources()
            .into_iter()
            .filter(|(_, text)| text.contains(needle))
            .map(|(path, _)| path)
            .collect();
        assert!(
            found.is_empty(),
            "the fixture signature stand-in reached production: {found:?}"
        );
    }
}
