//! The dependency fences around the local durable edge.
//!
//! No type can express "this crate has no dependency" or "the analyzer cannot reach this", so
//! these are lexical gates over the workspace's own manifests. Each walks a non-empty set and
//! names what it found, so a walk that silently matched nothing fails rather than passing.
//!
//! Both directions, always. An entry naming a crate that has stopped depending on this one is as
//! much a failure as a crate depending on it without an entry — an anticipatory entry reads as a
//! fence while permitting a crate nothing has checked.

use std::path::{Path, PathBuf};

fn crates_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(PathBuf::new, Path::to_path_buf)
}

/// The crates permitted to name this one.
///
/// Empty TODAY, and that is the stage rather than an oversight: nothing selects a production key
/// provider or store yet, so nothing depends on the crate that will own them. `cli` joins this
/// list at the stage that wires the production route, and the emptiness is asserted below rather
/// than assumed — an entry appearing early would read as a fence while permitting a reach nobody
/// reviewed.
const MAY_NAME_IT: [&str; 0] = [];

/// The crates that must NEVER name it, whatever else changes.
///
/// The analyzer's own graph. Filesystem and key availability cannot enter an analysis decision,
/// and the only way to keep that true as the tree grows is for the edge to be unspellable from
/// there.
const MUST_NEVER_NAME_IT: [&str; 5] = ["core", "analysis", "syntax", "plan", "aid"];

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

/// The dependency lines of one manifest, ignoring comment text so a rule that is merely described
/// does not read as a rule that is declared.
fn dependency_lines(manifest: &str) -> Vec<&str> {
    manifest
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect()
}

fn names_local(manifest: &str) -> bool {
    dependency_lines(manifest)
        .iter()
        .any(|line| line.starts_with("dorc-receipt-local"))
}

#[test]
fn the_analyzer_graph_cannot_spell_the_local_edge() {
    // The load-bearing one. A filesystem answer reaching a planning decision would make what the
    // analyzer concludes depend on whether a key file happened to be readable.
    let mut failures: Vec<String> = Vec::new();
    let mut checked = 0_usize;
    for (name, manifest) in manifests() {
        if MUST_NEVER_NAME_IT.contains(&name.as_str()) {
            checked = checked.saturating_add(1);
            if names_local(&manifest) {
                failures.push(format!("{name} names the local durable edge"));
            }
        }
    }
    assert_eq!(
        checked,
        MUST_NEVER_NAME_IT.len(),
        "the walk did not find every crate the list names"
    );
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn only_the_listed_crates_name_the_local_edge_and_every_listed_one_does() {
    // Two-way. The list is empty at this stage, which the second half asserts as a fact rather
    // than leaving as a vacuous pass.
    let mut namers: Vec<String> = Vec::new();
    for (name, manifest) in manifests() {
        if name != "receipt-local" && names_local(&manifest) {
            namers.push(name);
        }
    }
    for name in &namers {
        assert!(
            MAY_NAME_IT.contains(&name.as_str()),
            "{name} names the local durable edge and is not on the list"
        );
    }
    for allowed in MAY_NAME_IT {
        assert!(
            namers.iter().any(|name| name == allowed),
            "{allowed} is listed and names nothing; the entry is stale"
        );
    }
    assert_eq!(
        namers.len(),
        MAY_NAME_IT.len(),
        "the namers are {namers:?} and the list is {MAY_NAME_IT:?}"
    );
}

#[test]
fn the_local_edge_names_only_the_receipt_crates() {
    // What it may reach, from its own side. It stores signed bytes and supplies validated
    // capabilities; a dependency on the analyzer, the describe plane, or the transport would mean
    // it had started doing something else.
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .unwrap_or_default();
    let permitted = ["dorc-receipt ", "dorc-receipt-crypto "];
    let mut named = 0_usize;
    for line in dependency_lines(&manifest) {
        if !line.starts_with("dorc-") && !line.starts_with("weft") && !line.starts_with("errorloom")
        {
            continue;
        }
        named = named.saturating_add(1);
        assert!(
            permitted.iter().any(|prefix| line.starts_with(prefix)),
            "the local durable edge named a workspace crate it may not: {line}"
        );
    }
    assert!(named > 0, "it names no workspace crate at all");
}

#[test]
fn the_local_edge_carries_no_cryptographic_package_of_its_own() {
    // The crypto crate owns generation, parsing, and serialization of private key documents. This
    // one persists what it is handed, so a package here would be a second place able to decide
    // what a key document IS.
    let manifest =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
            .unwrap_or_default();
    for forbidden in ["age ", "age =", "ed25519", "rand", "getrandom", "sha2"] {
        assert!(
            !dependency_lines(&manifest)
                .iter()
                .any(|line| line.starts_with(forbidden)),
            "the local durable edge named {forbidden}"
        );
    }
}
