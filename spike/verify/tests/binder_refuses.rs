//! What the binder REFUSES — driven off throwaway fixture laws, never off `minispec/`.
//!
//! The corpus itself cannot exercise a refusal: every unit there is a well-formed stub, which
//! is the point. So the failure directions are pinned against fixtures under this crate's own
//! test space, where a builder may freely author a "law" (`301` §0's access law binds the spec
//! surface, and these files are deliberately outside it).
//!
//! Every case here asserts a REFUSAL. A checker whose passing case is tested and whose
//! refusing case is not has been tested for the half that cannot hurt anyone.

use std::path::Path;

use dorc_verify::badge::{Badge, Evidence, Expectation};
use dorc_verify::binding::{self, Disagreement, Proposal};
use dorc_verify::catalogue::{Binding, LawRow};
use dorc_verify::evidence::{self, Tier};
use dorc_verify::unit::{self, Statement};

const FIXTURES: &str = "tests/fixtures";

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(FIXTURES)
        .join(name)
}

fn row(slug: &'static str, bindings: &'static [Binding]) -> LawRow {
    LawRow {
        slug,
        seat: "dorc_core::sorted::SortedSet::insert",
        proof: None,
        harness: None,
        bindings,
        expected: [Expectation::Todo; 6],
    }
}

#[test]
fn a_marked_stub_and_a_silent_file_are_different_states() {
    // The distinction is the difference between a legal resting state and a broken unit. A
    // binder that collapsed them would either nag about every stub or accept a unit file that
    // states nothing at all.
    let stub = unit::read(&fixture("StubLawIsUnwritten.lean")).expect("fixture readable");
    assert_eq!(stub.statement, Statement::Unwritten);

    let silent = unit::read(&fixture("SilentLawStatesNothing.lean")).expect("fixture readable");
    assert_eq!(
        silent.statement,
        Statement::Missing,
        "a file that neither states a law nor admits it is unwritten is broken"
    );

    let stated = unit::read(&fixture("StatedLawHasAProp.lean")).expect("fixture readable");
    assert_eq!(stated.statement, Statement::Stated);
    assert!(stated.has_nonvacuity_probe);
    assert!(stated.battery_entries >= 1);
}

#[test]
fn a_unit_carrying_a_proof_hole_is_seen() {
    // A hole in a STATEMENT is worse than a hole in a proof: it makes the law itself vacuous,
    // and it typechecks, so nothing downstream complains.
    let holed = unit::read(&fixture("HoledLawIsVacuous.lean")).expect("fixture readable");
    assert!(holed.has_hole);
}

/// What `interrogated` says about a fixture unit, with Lean reported GREEN so the badge's other
/// half cannot be what answers.
fn interrogated_verdict(slug: &'static str, unit: &unit::Unit) -> Vec<Evidence> {
    evidence::compute(
        &row(slug, &[]),
        Some(unit),
        Path::new("."),
        Tier::WithEngines {
            lean_built: Some(true),
            kani: None,
        },
    )
}

#[test]
fn a_battery_that_never_instantiates_its_own_law_is_not_interrogated() {
    // The ratchet, exercised on a FIXTURE rather than by removing a line from a real unit: a
    // battery of concrete facts stays green whether or not the statement above it still says
    // anything about them, so without the coupling the badge measures the neighbourhood of a
    // law and not the law (`30B:fnd-battery-never-instantiates-its-own-law`).
    let uncoupled = unit::read(&fixture("StatedLawHasAProp.lean")).expect("fixture readable");
    assert!(uncoupled.has_nonvacuity_probe && uncoupled.battery_entries > 0);
    assert!(!uncoupled.has_coupling);
    let verdicts = interrogated_verdict("StatedLawHasAProp", &uncoupled);
    let found = &verdicts[1];
    assert!(
        matches!(found, Evidence::Absent(why) if why.contains(unit::COUPLING_INFIX)),
        "a battery with a probe and no coupling still earned the badge: {found:?}"
    );

    // …and the same shape WITH the coupling earns, so the refusal above is about the coupling
    // and not about something else the fixture happens to lack.
    let coupled = unit::read(&fixture("CoupledLawSpecializes.lean")).expect("fixture readable");
    assert!(coupled.has_coupling);
    assert_eq!(
        interrogated_verdict("CoupledLawSpecializes", &coupled)[1],
        Evidence::Earned
    );
    assert_eq!(
        Badge::ALL[1],
        Badge::Interrogated,
        "the index both assertions read"
    );
}

#[test]
fn an_unaccepted_proposal_is_refused() {
    // A loom that declares itself law evidence while no catalogue row accepts it is an author
    // who believes they armed something. Silence here would make the alarming key decorative.
    let laws = [row("ProposalWithoutAcceptance", &[])];
    let proposals = [Proposal {
        case: "spike/crates/cli/tests/nowhere.loom".to_owned(),
        slug: "ProposalWithoutAcceptance".to_owned(),
    }];
    let found = binding::disagreements(&laws, &proposals, Path::new("."));
    assert_eq!(
        found,
        vec![Disagreement::Unaccepted {
            case: "spike/crates/cli/tests/nowhere.loom".to_owned(),
            slug: "ProposalWithoutAcceptance".to_owned(),
        }]
    );
}

#[test]
fn an_accepted_binding_whose_case_vanished_is_refused() {
    // The other direction, and the one rot actually takes: the catalogue keeps claiming a
    // demonstration after somebody moved or deleted the case.
    static BINDINGS: [Binding; 1] = [Binding {
        case: "spike/crates/cli/tests/deleted-case.loom",
        assertions: &[],
    }];
    let laws = [row("AcceptanceWithoutCase", &BINDINGS)];
    let found = binding::disagreements(&laws, &[], Path::new("."));
    assert_eq!(
        found,
        vec![Disagreement::Missing {
            case: "spike/crates/cli/tests/deleted-case.loom".to_owned(),
            slug: "AcceptanceWithoutCase".to_owned(),
        }]
    );
}

#[test]
fn the_binding_key_is_in_the_closed_loom_vocabulary() {
    // `301` §2: the key joins FRONTMATTER_KEYS in the same commit that mints it. Both runners
    // refuse a key outside that set, so a binding declared before the key is known would be a
    // case that simply stops parsing.
    assert!(dorc_loom::is_frontmatter_key(
        dorc_verify::catalogue::BINDING_KEY
    ));
    assert!(
        dorc_loom::is_run_lane_key(dorc_verify::catalogue::BINDING_KEY),
        "a bound loom is a whole-product case; a non-run-lane key would refuse there"
    );
}

#[test]
fn the_vocabulary_home_is_not_walked_as_units() {
    // The governed vocabulary (`301` §1) is spec surface but not a law unit: no slug law, no
    // Prop contract, no catalogue row. A walk that swept it up would demand a law of a file
    // that deliberately is not one.
    let root = fixture("vocab-corpus");
    let units = unit::load_all(&root).expect("fixture corpus readable");
    assert_eq!(units.len(), 1, "only the top-level unit is a unit");
    assert_eq!(units[0].slug, "StubLawIsUnwritten");
}

#[test]
fn a_vocabulary_hole_is_seen() {
    // The one check vocabulary DOES owe: a hole in shared vocabulary vacates every importing
    // unit at once — a worse halo than a holed unit.
    let root = fixture("vocab-corpus");
    let vocab = unit::load_vocabulary(&root).expect("fixture vocabulary readable");
    assert_eq!(vocab.len(), 1);
    assert!(vocab[0].has_hole, "the planted hole must be seen");
}
