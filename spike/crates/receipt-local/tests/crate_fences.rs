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
/// `cli` alone: the production route's composition root is where a keyset and a store are
/// selected, and nothing else in the workspace has business reaching for either. Checked in both
/// directions below, so an entry that stopped depending fails as loudly as a crate depending
/// without one.
const MAY_NAME_IT: [&str; 1] = ["cli"];

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
    // Two-way: a crate naming it without an entry fails, and an entry that stopped naming it
    // fails too, so the list cannot rot into a description of what used to be true.
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

/// The authority-bearing entry points of this crate: everything that initializes keys, loads
/// them, publishes a receipt, or removes an object.
///
/// Checked below in BOTH directions — that every name here is still defined in this crate, and
/// that no production file outside it reaches one. A rename that left this list stale would
/// otherwise leave a fence matching nothing while reading like a fence.
const AUTHORITY_ENTRY_POINTS: [&str; 5] = [
    "open_or_initialize_for_write",
    "open_for_read",
    "open_or_create",
    "publish_required_v1",
    "remove_owned",
];

/// The production files permitted to reach any of them.
///
/// ONE file: the composition root. Every other production seat that wants a keyset or a store
/// asks that root for one, so this list stays a single line and a second entry is a governed
/// review rather than a call that appeared.
const MAY_REACH_THE_EDGE: [&str; 1] = ["cli/src/durable.rs"];

/// Every production `.rs` file in the workspace outside this crate, keyed by a slash-normalized
/// CRATE-RELATIVE path.
///
/// Production only: `tests/` and `benches/` are excluded, because a fixture reaching the edge is
/// exactly what the test layers are for.
///
/// Crate-relative rather than absolute, so an allow-list entry is a spelling a person can write
/// and a diff can be read: an absolute key would embed whoever's checkout produced it, which no
/// entry could ever match.
fn production_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let joined = format!("{prefix}/{name}");
            if path.is_dir() {
                walk(&path, &joined, out);
            } else if path.extension().is_some_and(|ext| ext == "rs")
                && let Ok(text) = std::fs::read_to_string(&path)
            {
                out.push((joined, text));
            }
        }
    }
    let root = crates_root();
    let mut found = Vec::new();
    for entry in std::fs::read_dir(&root).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "receipt-local" {
            continue;
        }
        walk(
            &entry.path().join("src"),
            &format!("{name}/src"),
            &mut found,
        );
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        found.len() > 50,
        "the source walk found {} files; it is looking in the wrong place",
        found.len()
    );
    found
}

/// Whether `text` carries `needle` as a WHOLE identifier.
///
/// Substring matching would count every identifier merely ending in those bytes, which is how a
/// fence quietly widens into one that matches things nobody meant.
fn names_identifier(text: &str, needle: &str) -> bool {
    let is_part = |byte: u8| byte == b'_' || byte.is_ascii_alphanumeric();
    text.match_indices(needle).any(|(at, _)| {
        let before = at
            .checked_sub(1)
            .and_then(|index| text.as_bytes().get(index))
            .copied();
        let after = at
            .checked_add(needle.len())
            .and_then(|index| text.as_bytes().get(index))
            .copied();
        !before.is_some_and(is_part) && !after.is_some_and(is_part)
    })
}

#[test]
fn every_authority_entry_point_this_fence_names_still_exists() {
    // The stale half. A rename that left an entry here pointing at nothing would leave the census
    // below walking for a name no code carries — a fence that passes because it matches nothing.
    let mut sources = String::new();
    for entry in std::fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("src"))
        .into_iter()
        .flatten()
        .flatten()
    {
        sources.push_str(&std::fs::read_to_string(entry.path()).unwrap_or_default());
    }
    assert!(
        !sources.is_empty(),
        "this crate's own sources were not read"
    );
    for entry_point in AUTHORITY_ENTRY_POINTS {
        assert!(
            names_identifier(&sources, &format!("fn {entry_point}")),
            "{entry_point} is named by this fence and defined nowhere in the crate"
        );
    }
}

#[test]
fn only_the_listed_production_files_reach_an_authority_entry_point() {
    // The census `30Rd` requires: every production call to key initialization, key loading, or
    // local receipt publication is enumerated, and adding one is a governed edit rather than
    // something that appears. A file has to name the crate to call into it, so naming the crate
    // is the precondition and the entry point is what makes the reach an authority-bearing one.
    let mut reaching_files: Vec<String> = Vec::new();
    for (path, text) in production_sources() {
        if !names_identifier(&text, "dorc_receipt_local") {
            continue;
        }
        let reached: Vec<&str> = AUTHORITY_ENTRY_POINTS
            .into_iter()
            .filter(|entry_point| names_identifier(&text, entry_point))
            .collect();
        if !reached.is_empty() {
            reaching_files.push(format!("{path} reaches {reached:?}"));
        }
    }
    for reaching_file in &reaching_files {
        assert!(
            MAY_REACH_THE_EDGE
                .iter()
                .any(|allowed| reaching_file.starts_with(allowed)),
            "{reaching_file}, and is not on the list"
        );
    }
    assert_eq!(
        reaching_files.len(),
        MAY_REACH_THE_EDGE.len(),
        "the files reaching an entry point are {reaching_files:?}; the list is {MAY_REACH_THE_EDGE:?}"
    );
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
