//! `rul-harness-is-developer-tooling-not-a-sandbox` bound 2, held objectively: `mise run install`
//! ships every production cli bin and never the developer harness. Bins come from `cargo metadata`,
//! not the `[[bin]]` stanzas — `autobins` discovers a bin no stanza names.
#![expect(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a test asserts by panicking; expect and index over cargo metadata and mise.toml so a malformed shape fails at the exact missing field"
)]

use std::collections::BTreeSet;
use std::process::Command;

fn cli_package_bins() -> BTreeSet<String> {
    let manifest = dorc_testbed::repo_root().join("spike").join("Cargo.toml");
    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(&manifest)
        .output()
        .expect("cargo metadata should spawn");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let meta: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("cargo metadata should be JSON");
    let package = meta["packages"]
        .as_array()
        .expect("metadata should list packages")
        .iter()
        .find(|p| p.get("name").and_then(serde_json::Value::as_str) == Some("dorc-cli"))
        .expect("the workspace should contain dorc-cli");
    package["targets"]
        .as_array()
        .expect("dorc-cli should have targets")
        .iter()
        .filter(|t| {
            t.get("kind")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|kinds| kinds.iter().any(|k| k.as_str() == Some("bin")))
        })
        .map(|t| {
            t.get("name")
                .and_then(serde_json::Value::as_str)
                .expect("a target should have a name")
                .to_owned()
        })
        .collect()
}

fn install_task_bins() -> BTreeSet<String> {
    let text = std::fs::read_to_string(dorc_testbed::repo_root().join("mise.toml"))
        .expect("root mise.toml should read");
    let doc: toml::Value = toml::from_str(&text).expect("root mise.toml should parse");
    let run = doc
        .get("tasks")
        .and_then(|t| t.get("install"))
        .and_then(|i| i.get("run"))
        .and_then(toml::Value::as_array)
        .expect("[tasks.install].run should be an array");
    let cargo_installs: Vec<&str> = run
        .iter()
        .filter_map(toml::Value::as_str)
        .filter(|line| line.contains("cargo install"))
        .collect();
    assert_eq!(
        cargo_installs.len(),
        1,
        "[tasks.install].run should carry exactly one `cargo install` line, found {}",
        cargo_installs.len()
    );
    cargo_installs[0]
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| pair[0] == "--bin")
        .map(|pair| pair[1].to_owned())
        .collect()
}

#[test]
fn the_install_task_names_the_cli_bins_minus_the_harness() {
    const HARNESS: &str = "dorc-harness";
    let package_bins = cli_package_bins();
    // Non-vacuous: subtracting the harness proves nothing unless it is a bin target — a renamed
    // harness must redden here, not slip through (rul-harness-is-developer-tooling-not-a-sandbox).
    assert!(
        package_bins.contains(HARNESS),
        "cargo metadata no longer reports a `{HARNESS}` bin for dorc-cli; a renamed harness must update this fence"
    );
    let installed = install_task_bins();
    let expected: BTreeSet<String> = package_bins
        .iter()
        .filter(|bin| bin.as_str() != HARNESS)
        .cloned()
        .collect();
    assert_eq!(
        installed, expected,
        "the `mise run install` --bin set {installed:?} must equal the dorc-cli bins minus the harness = {expected:?}; a new bin needs a distribution decision (rul-harness-is-developer-tooling-not-a-sandbox)"
    );
}
