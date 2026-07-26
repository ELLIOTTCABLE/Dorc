//! Byte-identity pins for the chrome migrated into the arrangement registry
//! (`289:rul-arrangement-home-is-registry-plus-transcripts`).
//!
//! The migration is a STORAGE move: every migrated line must render exactly the bytes its
//! hardcoded predecessor did. Most of the moved chrome is covered by the e2e/loom goldens
//! already, but three stderr lines — the plan-summary yardstick, the decision-digest, and the
//! why-pointer — are asserted by NOTHING in the corpus, so moving them would have been an
//! unwitnessed change. Each expectation below is the pre-migration literal, frozen.
//!
//! These are not render-form contracts (`27V:rul-output-form-unwelded` still holds): a
//! deliberate rewording updates the entry AND its pin together. What they catch is the accident —
//! a dropped separator, a lost space, a word-order slip in the ordered-word sequence.

use dorc_aid::arrangement::{CONST_ARRANGEMENTS, arrangement_sentence};

fn rendered(slug: &str, values: &[&str]) -> String {
    arrangement_sentence(&CONST_ARRANGEMENTS, slug, None, values)
}

/// `emit_plan_summary`'s yardstick line, previously one `format!` in `cli/src/main.rs`.
#[test]
fn the_plan_summary_line_is_byte_identical() {
    assert_eq!(
        rendered("cli-plan-summary-line", &["12", "5", "1", "3", "3", "0"]),
        "dorc: plan-summary sites=12 elide=5 omit=1 guard=3 run=3 may-alias=0"
    );
}

/// The decision-digest line that follows it.
#[test]
fn the_decision_digest_line_is_byte_identical() {
    assert_eq!(
        rendered("cli-decision-digest-line", &["deadbeef"]),
        "dorc: decision-digest deadbeef"
    );
}

/// The aggregate why-pointer a `dorc plan` preview closes with.
#[test]
fn the_why_pointer_line_is_byte_identical() {
    assert_eq!(
        rendered("cli-why-pointer-line", &["webhost.sh"]),
        "dorc: run `dorc why` for the per-site cause-chains, or `dorc why webhost.sh:N` to query a source line"
    );
}

/// The lint report's two sentences. Both ARE golden-covered, but pinning them here keeps the
/// value-interleaving contract (words = values + 1, in this order) readable in one place —
/// including the plural-suffix positions, which are the easiest thing to mis-order.
#[test]
fn the_lint_sentences_are_byte_identical() {
    assert_eq!(
        rendered("lint-clean-sentence", &["3", "s", "1", ""]),
        "dorc lint: clean — nothing found across 3 files, 1 source."
    );
    assert_eq!(
        rendered(
            "lint-summary-sentence",
            &["4", "s", "0", "s", "1", "", "1", ""]
        ),
        "dorc lint: 4 errors, 0 warnings, 1 info across 1 file."
    );
}

/// The four why-lens remediation hints, previously a class-keyed `&'static str` match in
/// `aid/src/diag.rs`. Like the three stderr lines above they are faceless — no transcript renders
/// them — so these literals are the whole net; the `[tag]` suffix is part of the prose
/// (`expected-why` needles substring-match it).
///
/// No longer the pre-migration bytes: `28G` Phase W1 respelled them twice, and the net now pins the
/// respell rather than the freeze. `elide` was ENGINE vocabulary reaching a user surface (the
/// admin-English carve, `28E` §8), and the em-dash violated `rul-ascii-output-forever`
/// (`28E` §0, human-typed). A migration net freezes bytes against ACCIDENTAL drift; it never
/// outranks a ruling that the frozen bytes were wrong.
#[test]
fn the_remediation_hints_are_byte_identical() {
    assert_eq!(
        rendered("why-remediation-provide-model", &[]),
        "to skip it, an oracle must declare a read-only probe for this kind [provide-model]"
    );
    assert_eq!(
        rendered("why-remediation-declare-identity", &[]),
        "to skip it, add the missing kind/selector/Query declaration [declare-identity]"
    );
    assert_eq!(
        rendered("why-remediation-resolve-dynamism", &[]),
        "to skip it, make the operand a literal Dorc can resolve+probe [resolve-dynamism]"
    );
    assert_eq!(
        rendered("why-remediation-structural", &[]),
        "no user fix -- Dorc cannot model this construct [structural]"
    );
}

/// An arity slip renders the greppable placeholder rather than a mangled line — the property that
/// makes a mis-wired seat loud instead of subtly wrong.
#[test]
fn a_seat_that_disagrees_with_its_entry_renders_unwritten() {
    assert_eq!(
        rendered("lint-summary-sentence", &["4"]),
        "[unwritten: lint-summary-sentence]"
    );
}
