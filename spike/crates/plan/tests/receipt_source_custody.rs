//! Which acquired bytes a plan receipt carries, and which it refuses to
//! (`30Ra:planning-book-bytes-and-durable-locators`).
//!
//! The custody rule is a PARTITION over the dialect classes, so every case here reads both halves:
//! a test that pinned only the carried half would pass over a projection that carried everything.

#![expect(
    clippy::expect_used,
    reason = "fixture helpers beside the cases, which the in-tests allowance does not reach"
)]

use std::collections::BTreeMap;

use dorc_core::Interner;
use dorc_core::SourceRole;
use dorc_core::influence::InfluenceAccount;
use dorc_core::spine::{
    InvocationMode, RunIdentity, SourceClaim, SpineInvocation, SpinePresentedPlan,
};
use dorc_plan::planning_input::{PlanningInputs, PlanningMode, PlanningPolicy};
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::receipt::{ProjectedPlan, RecordedInputs, SourceCustody, project};
use dorc_plan::{NO_ARTIFACT_FORM, Plan, ProbePlan, Spine, SurvivalReport};
use dorc_receipt::limits::{ByteLimit, ReceiptLimits};
use dorc_receipt::plan::RecordedSource;
use dorc_receipt::projection::OpaqueFieldTag;
use dorc_receipt::report::ByteAgreement;
use dorc_receipt::tokens::{OpaqueState, RecordedInvocationMode, RecordedSourceClass};

fn authored() -> InfluenceAccount {
    InfluenceAccount::authored_before_contact()
}

/// The book every witness here is settled over. Empty: these cases are about the SOURCE TABLE, and
/// a book with sites would add decisions none of them read.
const BOOK: &str = "";

/// Two acquired sources, so the partition has both sides to land on.
fn invocation_over(paths: [&str; 2]) -> SpineInvocation {
    SpineInvocation::minted(
        InvocationMode::WhylogReplay,
        vec![String::from("dorc"), String::from("plan")],
        paths
            .iter()
            .map(|path| SourceClaim {
                path: (*path).to_owned(),
                digest: "a".repeat(64),
                role: SourceRole::Book,
                bytes: 12,
            })
            .collect(),
        RunIdentity {
            nonce: String::from("n"),
            attempt: 1,
            host: String::from("web1"),
            started_at: None,
        },
        authored(),
    )
}

fn witness() -> FinalPresentation {
    let ast = dorc_syntax::parse(BOOK).value;
    let plan = Plan::decided(
        vec![],
        Vec::new(),
        SurvivalReport::default(),
        false,
        NO_ARTIFACT_FORM,
        BOOK,
        &ast,
        authored(),
    );
    FinalPresentation::of_settled(
        &plan,
        &ProbePlan::default(),
        BOOK,
        &ast,
        &Interner::default(),
        &[],
        PlanningInputs::of(
            "dorc/test",
            &invocation_over(["book.sh", "pkg.oracle.sh"]),
            None,
            None,
            PlanningPolicy::of(PlanningMode::Plan, false),
        ),
        None,
    )
}

fn spine_over(paths: [&str; 2]) -> Spine {
    let mut spine = Spine::new();
    spine.set_invocation(invocation_over(paths));
    spine.set_presented_plan(SpinePresentedPlan::minted(
        witness().presented_plan(),
        authored(),
    ));
    spine
}

fn projected(spine: &Spine, inputs: &RecordedInputs<'_>, limits: &ReceiptLimits) -> ProjectedPlan {
    project(
        spine,
        RecordedInvocationMode::Plan,
        authored(),
        &witness(),
        inputs,
        limits,
    )
    .expect("the Spine projects")
}

fn source_rows(
    spine: &Spine,
    inputs: &RecordedInputs<'_>,
    limits: &ReceiptLimits,
) -> Vec<RecordedSource> {
    projected(spine, inputs, limits).model().sources().to_vec()
}

fn custody(sources: [SourceCustody<'_>; 2]) -> RecordedInputs<'_> {
    RecordedInputs::of(sources.to_vec(), BTreeMap::new())
}

/// THE CUSTODY BOUNDARY: general sh keeps its exact bytes; valid `dorc-lang` keeps identity only.
#[test]
fn general_sh_keeps_its_bytes_and_dorc_lang_keeps_only_its_identity() {
    let spine = spine_over(["book.sh", "pkg.oracle.sh"]);
    let rows = source_rows(
        &spine,
        &custody([
            SourceCustody::general_sh("hork tune\n"),
            SourceCustody::dorc_lang(),
        ]),
        &ReceiptLimits::V1,
    );

    assert_eq!(rows[0].class(), RecordedSourceClass::GeneralSh);
    assert_eq!(
        rows[0].content(),
        OpaqueState::Captured,
        "general sh may mutate, so its exact bytes are what an address resolves against later"
    );
    assert_eq!(rows[1].class(), RecordedSourceClass::DorcLang);
    assert_eq!(
        rows[1].content(),
        OpaqueState::Uncollected,
        "a mutation-pure source is recoverable from its digest; carrying it would only multiply \
         the durable corpus"
    );
    // Non-vacuity: both rows still carry their identity half, so `uncollected` is a statement
    // about CONTENT rather than a row nobody projected.
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| !row.digest().is_empty()));
}

/// Exact bytes reach the overlay under their own tag, neither normalized nor transcoded.
///
/// CRLF is the case that matters: `30R` rules a newline conversion a SOURCE CHANGE, so a
/// projection that helpfully normalized it would make a drifted book read as unchanged.
#[test]
fn exact_source_bytes_ride_the_overlay_without_newline_normalization() {
    let crlf = "hork tune\r\nufw allow 443/tcp\r\n";
    let spine = spine_over(["book.sh", "pkg.oracle.sh"]);
    let document = projected(
        &spine,
        &custody([SourceCustody::general_sh(crlf), SourceCustody::dorc_lang()]),
        &ReceiptLimits::V1,
    );

    // Asked as a VERDICT: a detail hands out no plaintext, so exactness is proved by comparing
    // against the bytes this case supplied rather than by reading them back.
    let content: Vec<ByteAgreement> = document
        .details()
        .iter()
        .filter(|entry| entry.tag() == OpaqueFieldTag::SourceContent)
        .map(|entry| entry.agrees_with(crlf.as_bytes()))
        .collect();
    assert_eq!(content.len(), 1, "one general-sh source, one content entry");
    assert_eq!(
        content,
        vec![ByteAgreement::Identical],
        "every CR survives: a newline conversion is drift, not an equivalence"
    );
    let normalized = crlf.replace("\r\n", "\n");
    assert_eq!(
        document
            .details()
            .iter()
            .find(|entry| entry.tag() == OpaqueFieldTag::SourceContent)
            .map(|entry| entry.agrees_with(normalized.as_bytes())),
        Some(ByteAgreement::Differing),
        "and the comparison would notice the conversion this case exists to refuse"
    );
}

/// A source past the per-source bound records an omission rather than a shortened file.
///
/// Truncation is the failure worth refusing: a reader cannot tell a truncated book from a short
/// one, and every locator span would then index bytes that are no longer there.
#[test]
fn a_source_past_its_bound_records_omission_and_carries_nothing() {
    let mut narrow = ReceiptLimits::V1;
    narrow.source_content_bytes = ByteLimit::of(4);
    let spine = spine_over(["book.sh", "pkg.oracle.sh"]);
    let document = projected(
        &spine,
        &custody([
            SourceCustody::general_sh("far more than four bytes"),
            SourceCustody::dorc_lang(),
        ]),
        &narrow,
    );

    assert_eq!(
        document.model().sources()[0].content(),
        OpaqueState::OmittedLimit,
        "a bound that fired says so"
    );
    assert!(
        !document
            .details()
            .iter()
            .any(|entry| entry.tag() == OpaqueFieldTag::SourceContent),
        "and allocates nothing: no truncated body rides along beside the omission"
    );
}

/// The AGGREGATE bound is cumulative across the walk, not re-spent per source.
///
/// The failure this refuses is a document that admitted every source because each passed the
/// per-source bound alone. Two sources that individually fit and jointly do not is that shape.
#[test]
fn the_aggregate_content_bound_is_spent_across_the_whole_walk() {
    let mut narrow = ReceiptLimits::V1;
    narrow.source_content_aggregate_bytes = ByteLimit::of(12);
    let spine = spine_over(["book.sh", "helper.sh"]);
    let rows = source_rows(
        &spine,
        &custody([
            SourceCustody::general_sh("0123456789"),
            SourceCustody::general_sh("0123456789"),
        ]),
        &narrow,
    );

    assert_eq!(
        rows[0].content(),
        OpaqueState::Captured,
        "the first fits inside the aggregate"
    );
    assert_eq!(
        rows[1].content(),
        OpaqueState::OmittedLimit,
        "the second would take the total past it, and the budget is CUMULATIVE"
    );
}

/// Persistence expands no observation: a source nobody described carries nothing.
///
/// The projection holds a PATH for every source and must never turn one into a read. The
/// absent-entry default is what makes that structural — there is no branch that could open a file.
#[test]
fn a_source_the_caller_described_nothing_about_carries_no_bytes() {
    let spine = spine_over(["book.sh", "pkg.oracle.sh"]);
    let rows = source_rows(&spine, &RecordedInputs::default(), &ReceiptLimits::V1);
    assert!(
        rows.iter()
            .all(|row| row.content() == OpaqueState::Uncollected),
        "no custody described means no content, whatever path the row names"
    );
}
