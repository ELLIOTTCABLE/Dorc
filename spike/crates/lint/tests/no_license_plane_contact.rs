//! `dir-no-license-plane-contact`, made mechanical (`291` §4e, on `27U:finding-emission-would-vouch`).
//!
//! `dorc-lint` is advisory: a finding is a report line, never a claim, license, or fact. Three
//! module docs said so and nothing checked it — and `dorc-lint` depends on `dorc-core`, so the whole
//! `claim` module is reachable from here. Now that lint findings carry registry `DiagCode`s, the
//! crate LOOKS structurally like a decision-plane participant, which is exactly when a doc-only
//! claim starts rotting. This is the grep that keeps it honest.
//!
//! The named hazard class is EMISSION-VOUCHING: an aid-plane emission minting an elision license.
//! The historical instance was sh-side (a `27W` decline `printf` with no trailing `return 2` exiting
//! 0 and vouching), fixed by making recognized sink-emissions inert in the tracer; its regression
//! pins are `dorc_oracle::verdict`'s `emission_only_body_declines_never_vouches` and
//! `canonical_emission_then_return_two_declines_with_arm_captured`. This lane adds no sh-side
//! emission shape, so the pin here is the SHAPE-ANALOGUE, plus a both-ways check that the one shell
//! this lane DOES generate — the `dorc-loom scaffold` skeleton — is inert to the same tracer.

use std::path::{Path, PathBuf};

/// License-plane vocabulary. If any of these appears under `crates/lint/src`, the advisory crate has
/// reached into the decision plane — the exact conversion `claim-tier-gating` exists to stop.
const LICENSE_PLANE_TOKENS: &[&str] = &[
    "ByVouch",
    "ByObservation",
    "BySilence",
    "claim::",
    "Grade::Must",
    "mint_from_room",
    "RoomFact",
    "ReplaceLicense",
    "GuardLicense",
];

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn lint_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

#[test]
fn lint_source_never_names_the_license_plane() {
    let mut files = Vec::new();
    rs_files(&lint_src(), &mut files);
    assert!(!files.is_empty(), "the lint crate's src tree is readable");
    for file in files {
        let text = std::fs::read_to_string(&file).expect("read lint source");
        for token in LICENSE_PLANE_TOKENS {
            assert!(
                !text.contains(token),
                "{} names `{token}`: dorc-lint is ADVISORY (dir-no-license-plane-contact). A \
                 finding is a report line, never a claim/license/fact — lint-clean licenses \
                 nothing (two-plane-aid-law). Findings carrying `DiagCode`s does not change that.",
                file.display()
            );
        }
    }
}

/// The negative control: the scan can actually see a token, so a green run means absence, not a
/// broken reader.
#[test]
fn the_scan_would_see_a_license_token() {
    let mut files = Vec::new();
    rs_files(&lint_src(), &mut files);
    let corpus: String = files
        .iter()
        .filter_map(|file| std::fs::read_to_string(file).ok())
        .collect();
    assert!(
        corpus.contains("Severity"),
        "sanity: the scan reads real lint source (it finds an aid-plane token it SHOULD see)"
    );
}

/// `27U:finding-emission-would-vouch`, pinned BOTH ways over the one shell this lane generates.
///
/// `dorc-loom scaffold` writes a case skeleton. If that skeleton ever grew a verdict-shaped body
/// with a report-sink emission, a copy-pasted scaffold could carry an rc-0-on-the-printf vouch into
/// an author's oracle. Direction A: the skeleton contains no oracle-role funcdef and no report sink
/// at all. Direction B: the tracer that would be fooled still declines an emission-only body, so the
/// vouching boundary has not moved under us.
#[test]
fn the_scaffold_skeleton_carries_no_vouchable_shell() {
    let bin = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .join("dorc-loom/src/bin/dorc-loom.rs");
    let text = std::fs::read_to_string(&bin).expect("read the scaffold command");
    let start = text
        .find("let skeleton = format!(")
        .expect("the scaffold skeleton literal");
    let end = text[start..]
        .find(");")
        .map(|offset| start + offset)
        .expect("the skeleton literal terminates");
    let skeleton = &text[start..end];
    for forbidden in [
        "__is_converged",
        "__predict",
        "__disturbs",
        "DREP_V1",
        "printf",
    ] {
        assert!(
            !skeleton.contains(forbidden),
            "the scaffold skeleton contains `{forbidden}`: generated shell must stay inert to the \
             vouch tracer (27U:finding-emission-would-vouch) — a scaffold is a case skeleton, never \
             a starting point for oracle code"
        );
    }

    // Direction B: the sh-side inertness the oracle crate owns is still in force. Named here so a
    // lane touching aid-plane emission shows it did not move the boundary.
    let verdict = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .join("oracle/src/verdict.rs");
    let verdict = std::fs::read_to_string(&verdict).expect("read the verdict tracer");
    for pin in [
        "fn emission_only_body_declines_never_vouches",
        "fn canonical_emission_then_return_two_declines_with_arm_captured",
    ] {
        assert!(
            verdict.contains(pin),
            "the emission-inertness regression pin `{pin}` is gone — it is the only thing standing \
             between a recognized sink-emission and an rc-0 vouch"
        );
    }
}
