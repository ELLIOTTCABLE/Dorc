//! The honest-trigger invocation route (`289:rul-worldless-route-honest-trigger`; `291` §5a W2).
//!
//! An invocation-error case declares a code AND shows a command. The route runs the REAL argument
//! parser over that command and uses whatever diagnostic it actually produced — and REFUSES when
//! the slug disagrees. The refusal is the whole value (`291:rule-worldless-route-refuses-on-mismatch`):
//! without it the command would be decorative, free to drift from the code forever, on the surface
//! humans review errors through (`288:rul-errors-human-authored-review-surface`).

#![expect(
    clippy::panic,
    reason = "fixture loader over the committed corpus; the no-panic lints guard untrusted input"
)]

use std::path::{Path, PathBuf};

use dorc_loom::DorcConsumer;
use errorloom::{Case, CaseRenderer};

/// Every case whose world is its own argv — the route's real subjects.
const HONEST_CASES: [&str; 12] = [
    "cli-strip-needs-path.loom",
    "cli-strip-got-a-flag.loom",
    "cli-unknown-mode.loom",
    "cli-flag-needs-value.loom",
    "cli-unknown-flag.loom",
    "cli-unknown-flag-did-you-mean.loom",
    "cli-flag-value-not-recognized.loom",
    "cli-flag-value-not-a-number.loom",
    "cli-no-book-given.loom",
    "cli-flags-mutually-exclusive.loom",
    "cli-flag-requires-mode.loom",
    "dorc-sh-usage.loom",
];

fn cases_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("cases")
}

fn read(name: &str) -> Case {
    let text = std::fs::read_to_string(cases_dir().join(name))
        .unwrap_or_else(|e| panic!("read {name}: {e}"));
    Case::parse(&text).unwrap_or_else(|e| panic!("parse {name}: {e}"))
}

/// Every honest case's replay command REALLY fires its declared code — the parser, not a fixture,
/// decides. Anti-masking: nothing here constructs a payload; the render comes from the live parse.
#[test]
fn each_invocation_case_is_fired_by_its_own_command() {
    let consumer = DorcConsumer::new();
    for name in HONEST_CASES {
        let case = read(name);
        let slug = case
            .frontmatter()
            .scalar("code")
            .unwrap_or_else(|| panic!("{name} declares a code"));
        let rendered = consumer
            .render_case(&case)
            .unwrap_or_else(|e| panic!("{name} renders through the real parser: {e}"));
        assert!(
            rendered.contains(&format!("error[{slug}]")),
            "{name}'s own command fires {slug}"
        );
    }
}

/// The REFUSAL: point a case's command at an argv that fires a DIFFERENT code and the route must
/// not render it. Without this the honest route is world-as-payload with extra steps.
#[test]
fn a_command_that_fires_another_code_is_refused() {
    let case = read("cli-no-book-given.loom");
    let text =
        std::fs::read_to_string(cases_dir().join("cli-no-book-given.loom")).expect("read the case");
    // `dorc plan --wat` fires `cli-unknown-flag-did-you-mean`, never `cli-no-book-given`.
    let drifted = text.replace("$ dorc plan\n", "$ dorc plan --wat\n");
    assert_ne!(drifted, text, "the fixture still carries the bare command");
    let drifted = Case::parse(&drifted).expect("the drifted case still parses");

    let consumer = DorcConsumer::new();
    assert!(
        consumer.render_case(&case).is_ok(),
        "precondition: the committed command does fire its code"
    );
    assert!(
        consumer.render_case(&drifted).is_err(),
        "a command firing a DIFFERENT code must be refused, never rendered as if it matched"
    );
}

/// A command that parses CLEANLY is not an invocation error at all, so the route declines and the
/// case falls through to its ordinary world — which is how the world-as-payload cases still work.
#[test]
fn a_successful_command_falls_through_to_the_payload_world() {
    let case = read("cli-file-not-found.loom");
    let rendered = DorcConsumer::new()
        .render_case(&case)
        .expect("the payload world still renders");
    assert!(
        rendered.contains("error[cli-file-not-found]"),
        "an I/O-world code keeps its constructed stand-in: {rendered}"
    );
}
