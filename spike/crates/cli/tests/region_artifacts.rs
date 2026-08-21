//! What a shared-region edit may put in a shipped artifact (`plans/30L` §8, §11).
//!
//! An ordinary-harness `.rs` test, like `definition_frames`: it READS the committed round-trip
//! goldens and asserts STRUCTURE over them. That is deliberate. The pins here are about what the
//! artifact may contain — a cloned helper, a renamed one, a generated dispatch, a per-call
//! specialization — and only the emitted bytes can answer that. Wording churns freely and gets
//! re-blessed (`render-form-unwelded`); the shapes below must survive every such churn, so nothing
//! here compares whole bytes.

#![expect(
    clippy::expect_used,
    clippy::panic,
    reason = "a fixture this battery cannot read is a broken corpus, not a case answer: the in-tests allowance the policy intends"
)]

use std::path::{Path, PathBuf};

/// Every region case's committed apply artifact, by case name.
fn region_artifacts() -> Vec<(String, String)> {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&tests)
        .expect("the case collection is readable")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("region30-"))
        })
        .collect();
    cases.sort();
    assert!(
        !cases.is_empty(),
        "the walk found no region cases, so every assertion below would pass vacuously"
    );
    cases
        .into_iter()
        .map(|dir| {
            let name = dir
                .file_name()
                .expect("a case dir has a name")
                .to_string_lossy()
                .into_owned();
            let out = std::fs::read_to_string(dir.join("expected.out"))
                .unwrap_or_else(|_| panic!("{name} has a committed transcript"));
            let apply = out
                .rfind("#!/bin/sh")
                .map_or_else(String::new, |at| out[at..].to_owned());
            (name, apply)
        })
        .collect()
}

/// `30L:pin-no-generated-specialization` — an emitted artifact holds no cloned or renamed helper and
/// no generated invocation dispatch.
///
/// The counted thing is FUNCTION HEADERS, because that is what a specialization would have to be:
/// per-call bodies mean two headers under related names, and generated dispatch means a body the
/// engine wrote. One header per name is the whole claim, and it is exactly what
/// `30L:rul-no-specialized-shell` demands — there must always be one authored line, by one human,
/// answerable for anything that runs.
#[test]
fn no_region_artifact_carries_a_cloned_or_renamed_helper() {
    for (name, apply) in region_artifacts() {
        let mut headers: Vec<&str> = apply
            .lines()
            .filter_map(|line| line.trim().strip_suffix("() {"))
            .collect();
        let before = headers.len();
        headers.sort_unstable();
        headers.dedup();
        assert_eq!(
            before,
            headers.len(),
            "{name}: a name is defined twice in one artifact — a per-call clone is the shape this \
             forbids:\n{apply}"
        );
        assert!(
            !headers.iter().any(|header| header.contains("_h")),
            "{name}: a munged emission means two bodies under one name, which no region case \
             should produce: {headers:?}"
        );
    }
}

/// `30L:rul-edit-authored-definition-once` — the shared transformation lands at the AUTHORED
/// function-body region, once, and calls stay calls.
///
/// The parametric guard is the sharpest witness available: its argv must be the SOURCE-level
/// expression, so one edit serves every operand. A resolved operand in guard position would mean
/// the engine had specialized shared source to one call's arguments — installing, into a body that
/// also serves the other operand, a check about something else entirely.
#[test]
fn a_shared_guard_carries_the_source_level_argv_not_a_resolved_operand() {
    let cases = region_artifacts();
    let (name, apply) = cases
        .iter()
        .find(|(name, _)| name.contains("drifted-route-guards"))
        .expect("the divergent-instances valve case");
    let guards: Vec<&str> = apply
        .lines()
        .filter(|line| line.contains("__is_converged") && line.contains("||"))
        .collect();
    assert_eq!(
        guards.len(),
        1,
        "{name}: ONE authored region, ONE edit — never one per invocation: {guards:?}"
    );
    let guard = guards[0];
    assert!(
        guard.contains("\"$1\""),
        "{name}: the shared guard must re-bind per invocation inside sh: {guard}"
    );
    for resolved in ["nginx", "curl"] {
        assert!(
            !guard.contains(resolved),
            "{name}: a per-call literal in shared source serves the wrong operand at every other \
             invocation: {guard}"
        );
    }
}

/// `30L` §9 — `dorc why` walks BOTH directions across the two identities.
///
/// A definition region has to be addressable in its own right (a reader who sees an edit at a body
/// line asks about THAT line, and gets a row marked universal over the invocations that licensed
/// it), and a call has to name the shared edits it executes (otherwise it reads as a bare `run`
/// with every real decision invisible one indent in).
///
/// The components render `[unwritten: <slug>]` on purpose: builders author no user-facing prose
/// (`27V:rul-error-authorship-tier`), so what is pinned here is that the STRUCTURE reaches the
/// surface at all — the words are a conductor/human act against a registered row.
#[test]
fn a_why_report_walks_from_a_region_to_its_invocations_and_back() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("region30-twin-calls-share-one-region");
    let read = |name: &str| std::fs::read_to_string(dir.join(name)).expect("a case fixture");
    let mut oracle_paths = Vec::new();
    let mut oracle_srcs = Vec::new();
    for oracle in ["pkgindex-predict.oracle.sh", "pkgindex-verdict.oracle.sh"] {
        oracle_paths.push(oracle.to_owned());
        oracle_srcs.push(read(oracle));
    }
    let book = read("book.sh");
    let snapshot = dorc_cli::snapshot::StaticLoadSnapshot::over(
        dorc_core::loadpath::Cwd::default(),
        oracle_paths.clone(),
        oracle_srcs,
        &dorc_cli::snapshot::LoadPositions::roots_only(),
        "book.sh",
        &book,
    );
    // The UNMEASURED world is enough: a running region is the one a reader most needs explained.
    let world = dorc_cli::world::WhyWorld::analyze(&snapshot);
    let framing = dorc_plan::records::Framing::spike(dorc_plan::invocation::book_digest(&book));
    let receipt = dorc_cli::Receipt {
        at: None,
        replayed: false,
        host: framing.host().to_owned(),
        book: "book.sh".to_owned(),
        book_digest: framing.book_digest().to_owned(),
        at_head: None,
        oracles: oracle_paths,
        risk_profile: None,
        tally: dorc_cli::PlanTally::Derived(world.disposition_counts()),
        deepest_tier: true,
        narratable: true,
    };
    let ctx = dorc_aid::RenderCtx::production();
    let line_of = |needle: &str| {
        let at = book.find(needle).expect("the fixture carries this line");
        format!("book.sh:{}", book[..at].lines().count() + 1)
    };
    // ADDRESSED: both rows are quiet, and the aggregate curates toward surprises.
    let asked = |address: String| {
        dorc_cli::why::why_report_parts(&ctx, &world.report(Some(&address), &receipt)).text()
    };
    let at_region = asked(line_of("   apt-get install -y \"$1\""));
    assert!(
        at_region.contains("why-reason-region-universal-over"),
        "the body line must answer for its own shared edit:\n{at_region}"
    );
    let at_call = asked(line_of("install_pkg nginx"));
    assert!(
        at_call.contains("why-reason-call-executes-shared-regions"),
        "and the call must name the shared edits it executes:\n{at_call}"
    );
}

/// `30L:pin-whole-helper-derived-only` (§8) — when every invocation of a definition elides, the
/// inert definition keeps its AUTHORED bytes.
///
/// No delete-helper mechanism exists, and no stand-in is written into a body no route reaches. The
/// book's own text is the comparand, so this cannot pass by the artifact and the golden drifting
/// together.
#[test]
fn a_wholly_elided_helpers_body_ships_verbatim() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("region30-whole-helper-stays-authored-text");
    let book = std::fs::read_to_string(dir.join("book.sh")).expect("the case's book");
    let apply = std::fs::read_to_string(dir.join("expected.out")).expect("the case's transcript");
    let body: Vec<&str> = book
        .lines()
        .skip_while(|line| !line.starts_with("main() {"))
        .take_while(|line| *line != "}")
        .collect();
    assert!(body.len() > 1, "the fixture must really carry a body");
    for line in body {
        assert!(
            apply.contains(line),
            "the inert definition must ship verbatim; missing {line:?}"
        );
    }
    assert!(
        !apply.contains("   true"),
        "and it must acquire no stand-in — nothing in it can execute:\n{apply}"
    );
}
