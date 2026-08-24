//! The dependency-edge fence: which crates may name which, read off the manifests.
//!
//! No type can express "this crate has no dependency", so this is a lexical gate over the
//! workspace's own manifests. It walks a non-empty set and names the crates it found, so a
//! walk that silently found nothing fails rather than passing.

use std::path::{Path, PathBuf};

fn crates_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/receipt has a parent")
        .to_path_buf()
}

fn manifests() -> Vec<(String, String)> {
    let root = crates_root();
    let mut found: Vec<(String, String)> = std::fs::read_dir(&root)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", root.display()))
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
            .expect("its own manifest");
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

#[test]
fn no_source_file_of_the_pure_crate_names_a_crypto_package() {
    // The manifest fence and this one answer different questions: one is what the crate may
    // link, the other is what its code reaches for.
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut checked = 0_u32;
    for entry in std::fs::read_dir(&src).expect("its own source").flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("readable source");
        for token in ["dorc_receipt_crypto", "ed25519_dalek", "age::"] {
            assert!(
                !text.contains(token),
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
    const MAY_NAME_IT: [&str; 2] = ["receipt-crypto", "cli"];
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
    let text = std::fs::read_to_string(&manifest).expect("the crypto manifest");
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
    let text = std::fs::read_to_string(&manifest).expect("the crypto manifest");
    let age_line = dependency_lines(&text)
        .into_iter()
        .find(|line| line.starts_with("age "))
        .expect("the manifest names age");
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
