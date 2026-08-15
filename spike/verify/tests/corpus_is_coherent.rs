//! The binder's cheap gate, over the REAL corpus, riding the ordinary workspace suite.
//!
//! Placing it here rather than behind a mise task is what makes it run on both platform legs
//! for free, with no external toolchain: `cargo nextest run --workspace` is the gate, and
//! `301` §5 wants exactly these checks at that tier.

use dorc_verify::badge::{Badge, Evidence, Expectation};
use dorc_verify::catalogue_lock::LAWS;
use dorc_verify::evidence::{self, Tier};
use dorc_verify::{check, repo_root, seat, unit};

#[test]
fn the_catalogue_and_the_corpus_agree() {
    let findings = check::run(repo_root()).expect("the corpus must be readable");
    assert!(
        findings.failures.is_empty(),
        "dorc-verify check found {} failure(s):\n  {}",
        findings.failures.len(),
        findings.failures.join("\n  ")
    );
}

#[test]
fn every_cited_seat_resolves_to_a_real_function() {
    // The citation is a checked confirmation, not new information — which is precisely why it
    // earns its keep: this is the assertion that goes red when a rename moves a chokepoint out
    // from under a law, in a directory the renamer never opened.
    for law in &LAWS {
        seat::resolve(law.seat, repo_root()).unwrap_or_else(|why| panic!("{}: {why}", law.slug));
    }
}

#[test]
fn the_cheap_tier_never_mints_an_external_engine_badge() {
    // The whole tier split rests on this. A cheap run that answered `earned` for a Lean or
    // Kani badge would be laundering a committed expectation into an evidence claim — the
    // exact stale-coverage lie the catalogue exists to catch, wearing the catalogue's clothes.
    let units = unit::load_all(repo_root()).expect("the corpus must be readable");
    for law in &LAWS {
        let found = evidence::compute(
            law,
            units.iter().find(|u| u.slug == law.slug),
            repo_root(),
            Tier::Cheap,
        );
        for (badge, verdict) in Badge::ALL.iter().zip(&found) {
            if badge.needs_external_engine() {
                assert_ne!(
                    *verdict,
                    Evidence::Earned,
                    "{} minted `{badge}` without running its engine",
                    law.slug
                );
            }
        }
    }
}

#[test]
fn an_unlooked_at_badge_cannot_contradict_and_a_looked_at_one_can() {
    // `NotAtThisTier` agreeing with everything is load-bearing and easy to misread as
    // permissiveness, so it is pinned beside the cases that DO refuse.
    assert!(Evidence::NotAtThisTier.agrees_with(Expectation::Earned));
    assert!(Evidence::NotAtThisTier.agrees_with(Expectation::Todo));
    assert!(Evidence::Earned.agrees_with(Expectation::Earned));
    assert!(Evidence::Absent("x".to_owned()).agrees_with(Expectation::Todo));
    assert!(Evidence::Absent("x".to_owned()).agrees_with(Expectation::Excepted("why")));

    // Both directions refuse: rot (promoted earned, evidence gone) and ambition (promoted
    // todo, evidence present).
    assert!(!Evidence::Absent("x".to_owned()).agrees_with(Expectation::Earned));
    assert!(!Evidence::Earned.agrees_with(Expectation::Todo));
    assert!(!Evidence::Earned.agrees_with(Expectation::Excepted("why")));
}
