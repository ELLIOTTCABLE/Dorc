//! The committed identity corpus: exact canonical bytes, and the exact identity each one has.
//!
//! These vectors are reviewed fixtures over the ENCODING, not over any run. That is deliberate:
//! the two mints take bare bytes, so what has to be pinned is the relation between a canonical
//! byte string and its identity — the domain it is separated under, the envelope it goes through,
//! and the digest that comes out. A vector minted from a live plan would pin whatever the planner
//! did that day and would churn whenever an unrelated render moved.
//!
//! `identity_table` is the reviewed statement of what a canonical byte string may contain; these
//! are instances of it, and the digests below are what the mints answer for them.

use dorc_receipt::ids::{PlanningInputId, PresentedPlanId};

/// The canonical planning-input bytes, valid under `identity_table::PLANNING_INPUT_COMPONENTS`.
const PLANNING_INPUT: &[u8] = include_bytes!("vectors/identity/planning-input.canonical");

/// The same inputs, except that `semantics` carries bytes SPELLING the line that follows it.
///
/// The cross-field substitution vector. Under an encoding that separated fields by their
/// separator alone, this and the vector above would be indistinguishable readings of one byte
/// string; the declared length is what makes them two documents that cannot be confused, and the
/// two identities below are what says so.
const PLANNING_INPUT_SPELLING_A_TAG: &[u8] =
    include_bytes!("vectors/identity/planning-input-value-spells-a-tag.canonical");

/// The canonical presented-plan bytes: every component, in order, each declaring its length,
/// with `regions` spelled absent because the fixture has none.
const PRESENTED_PLAN: &[u8] = include_bytes!("vectors/identity/presented-plan.canonical");

#[test]
fn every_committed_vector_has_exactly_its_committed_identity() {
    // The whole point of a conformance vector for a hash: the bytes are frozen and so is the
    // answer, so a change to the domain string, the envelope, or the digest is a red test rather
    // than a silent re-identification of every document already written.
    assert_eq!(
        PlanningInputId::of_canonical_inputs(PLANNING_INPUT).hex(),
        "24280d95cade8646dd512d482d8dc61f0879a456f2447a4b8ad6717424d44da5",
    );
    assert_eq!(
        PlanningInputId::of_canonical_inputs(PLANNING_INPUT_SPELLING_A_TAG).hex(),
        "09847e59764819adf1b2f2f98230bc6a5f3274d19d045bcc87307461428c44ea",
    );
    assert_eq!(
        PresentedPlanId::of_canonical_decision(PRESENTED_PLAN).hex(),
        "fbf2ae3c99ce3c048a3def6a7a46f137be2fe38e80de1048268cc311e3a2fac7",
    );
}

#[test]
fn a_framed_value_spelling_its_neighbour_is_a_different_document() {
    // The substitution the length framing exists to refuse, at the byte level. The two vectors
    // carry the same payload bytes split differently across one boundary, and they must not
    // present one identity.
    assert_ne!(
        PlanningInputId::of_canonical_inputs(PLANNING_INPUT),
        PlanningInputId::of_canonical_inputs(PLANNING_INPUT_SPELLING_A_TAG),
    );
}

#[test]
fn one_byte_string_under_two_domains_is_two_identities() {
    // Domain separation, over bytes rather than over a live value: the same canonical string
    // presented to each mint answers differently, so an identity minted for one surface cannot be
    // read as an identity of the other even where the material happens to coincide.
    assert_ne!(
        PlanningInputId::of_canonical_inputs(PLANNING_INPUT).hex(),
        PresentedPlanId::of_canonical_decision(PLANNING_INPUT).hex(),
    );
    assert_ne!(
        PlanningInputId::of_canonical_inputs(PRESENTED_PLAN).hex(),
        PresentedPlanId::of_canonical_decision(PRESENTED_PLAN).hex(),
    );
}
