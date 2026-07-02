//! The R2 corpus-wide differential gate (23E §3 / 23H §5) — the FLIP-GATE.
//!
//! For every e2e oracle fixture, the inline-dialect derivation ([`derive_check`]) must
//! reproduce EXACTLY the effect-map the retired markers lift ([`lift`]). The fixtures are
//! additive during the transition (they carry BOTH the `oracle_*` markers AND the inline
//! `case $verb` arms + trailing marks), so a single file feeds both sides, and this test
//! pins that swapping the effect-map source (marker → derivation) changes no behaviour.
//! It must be green before the wiring flips (P4); it is deleted with the marker lift (P5).
//!
//! Never-vouch: this is machine-run process-evidence, not a correctness proof. Its value is
//! catching a wrong-derivation (a dropped/extra cell) at CORPUS scale, before any golden or
//! wiring churns — the same DST differential discipline the project already uses.
#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "test harness: a fixture that cannot be read/parsed is a loud test failure, not production code"
)]

use dorc_core::Interner;
use dorc_oracle::check::{ValueClaim, derive_check, lift_checks};
use dorc_oracle::{Polarity, lift};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// One effect cell, normalized to strings so both sides compare identically:
/// `(provider, verb, kind, selector, claim-label)`. The ε-verb is the empty string.
type Cell = (String, String, String, String, &'static str);

/// The differential-comparison label — the ONLY place the polarity-free [`ValueClaim`]
/// is mapped back onto the old `Polarity` vocabulary. The lifted end-state carries no
/// polarity (jc-polarity-vs-rc); the `!`-inverted claim maps to the former `kill`.
fn claim_label(c: ValueClaim) -> &'static str {
    match c {
        ValueClaim::Establish => "establish",
        ValueClaim::EstablishInverted => "kill",
        ValueClaim::Observe => "query",
    }
}

fn polarity_label(p: Polarity) -> &'static str {
    match p {
        Polarity::Establish => "establish",
        Polarity::Kill => "kill",
        Polarity::Query => "query",
    }
}

/// The cell-set the inline-dialect derivation produces for every check in `src`.
fn derived_set(src: &str) -> BTreeSet<Cell> {
    let mut i = Interner::default();
    let cs = lift_checks(&mut i, src);
    let mut out = BTreeSet::new();
    for provider_sym in cs.value.providers() {
        let check = cs
            .value
            .get(provider_sym)
            .expect("provider from providers()");
        let provider = i.resolve(check.provider).to_owned();
        let (effects, _vouches) = derive_check(check);
        for e in effects {
            out.insert((
                provider.clone(),
                e.verb.unwrap_or_default(),
                e.kind,
                e.selector,
                claim_label(e.claim),
            ));
        }
    }
    out
}

/// The cell-set the OLD marker lift produces for `src`, enumerated over every declared
/// `(provider, verb)` key (both directions — a marker cell the derivation misses shows up
/// here even if the derivation never named that verb).
fn oldlift_set(src: &str) -> BTreeSet<Cell> {
    let mut i = Interner::default();
    let idx = lift(&mut i, &[src]);
    let mut out = BTreeSet::new();
    for (&(provider, verb), cells) in idx.value.effects_iter() {
        let provider_s = i.resolve(provider.0).to_owned();
        let verb_s = i.resolve(verb).to_owned();
        for cell in cells {
            out.insert((
                provider_s.clone(),
                verb_s.clone(),
                i.resolve(cell.kind.0).to_owned(),
                i.resolve(cell.selector.0).to_owned(),
                polarity_label(cell.polarity),
            ));
        }
    }
    out
}

/// Recursively collect `*.oracle.sh` under `root`, sorted (`inv-determinism` for tests).
fn collect_oracle_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries =
            std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".oracle.sh"))
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// THE flip-gate: `derive(inline) == lift(markers)` for every e2e oracle fixture. On any
/// mismatch, report EVERY offending file with its symmetric set-difference (derived-only
/// and marker-only cells) so a wrong conversion is diagnosed at a glance, not one-at-a-time.
#[test]
fn derivation_matches_markers_across_the_corpus() {
    let cases = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../e2e/cases");
    let files = collect_oracle_files(&cases);
    assert!(
        files.len() >= 100,
        "expected the full e2e oracle corpus, found {} files under {}",
        files.len(),
        cases.display()
    );

    let mut failures = Vec::new();
    for file in &files {
        let src = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
        let derived = derived_set(&src);
        let markers = oldlift_set(&src);
        if derived != markers {
            let derived_only: Vec<_> = derived.difference(&markers).collect();
            let marker_only: Vec<_> = markers.difference(&derived).collect();
            failures.push(format!(
                "\n  {}\n    derived-only (markers lack): {:?}\n    marker-only  (derive misses): {:?}",
                file.strip_prefix(&cases).unwrap_or(file).display(),
                derived_only,
                marker_only,
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "the inline derivation diverged from the marker effect-map in {} of {} fixtures:{}",
        failures.len(),
        files.len(),
        failures.join(""),
    );
}
