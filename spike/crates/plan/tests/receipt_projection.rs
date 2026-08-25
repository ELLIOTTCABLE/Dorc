//! The Spine → `PlanReceipt` projection, pinned where it can be silently wrong.
//!
//! Two of these guard failures that VALIDATE: a mis-numbered region reference and a dropped
//! population both produce a document the model accepts.
#![expect(
    clippy::panic,
    reason = "a test helper beside the cases, which the in-tests allowance does not reach"
)]

use dorc_core::influence::InfluenceAccount;
use dorc_core::region::{ElisionRegion, RegionUniverse};
use dorc_core::spine::{
    InvocationMode, RegionRoutes, RenderDecision, RunIdentity, SourceClaim, SpineInvocation,
    SpineRegionDecision, SpineRenderDecision, SpineSpecies,
};
use dorc_core::{AstId, BytePos, DefinitionId, SourceFileId, SourceRole, Span};
use dorc_plan::receipt::{ProjectionRefusal, project};
use dorc_plan::{Disposition, Spine};
use dorc_receipt::plan::RenderSubject;
use dorc_receipt::rows::RecordedProjectionOmission;
use dorc_receipt::tokens::{
    ClosedToken, RecordedInvocationMode, RecordedOmissionReason, RecordedRenderKind,
    RecordedSpineSpecies,
};

fn authored() -> InfluenceAccount {
    InfluenceAccount::authored_before_contact()
}

/// A Spine carrying the one record the projection demands, and nothing else.
fn spine_with_invocation() -> Spine {
    let mut spine = Spine::new();
    spine.set_invocation(SpineInvocation::minted(
        InvocationMode::WhylogReplay,
        vec![String::from("dorc"), String::from("plan")],
        vec![SourceClaim {
            path: String::from("book.sh"),
            digest: "a".repeat(64),
            role: SourceRole::Book,
            bytes: 12,
        }],
        RunIdentity {
            nonce: String::from("n"),
            attempt: 1,
            host: String::from("web1"),
            started_at: None,
        },
        authored(),
    ));
    spine
}

/// Two regions, and a render refusal belonging to the SECOND.
fn region_at(file: u32, lo: u32) -> ElisionRegion {
    let definition = DefinitionId::at(
        SourceFileId(file),
        Span::new(BytePos(lo), BytePos(lo.saturating_add(1))),
    );
    let universe = RegionUniverse::of_book_custody_files([SourceFileId(file)]);
    let span = Span::new(BytePos(lo), BytePos(lo.saturating_add(9)));
    match ElisionRegion::mint(&universe, definition, span) {
        Some(region) => region,
        None => panic!("a book-custody definition admits a region"),
    }
}

#[test]
fn a_render_row_names_the_region_the_run_decided_and_not_its_neighbour() {
    // THE TRAP this projection is most able to fall into. The model range-checks a region
    // ordinal and cannot range-check a WRONG one, so numbering the regions in one walk and
    // emitting the render rows from another would leave every region-keyed row describing a
    // different region — with the document still validating cleanly.
    //
    // Two regions, and the refusal belongs to the SECOND. A projection that numbered from a
    // second walk, or defaulted an unknown region to zero, reports the FIRST.
    let (first, second) = (region_at(0, 10), region_at(0, 40));
    assert_ne!(first, second);

    let mut spine = spine_with_invocation();
    for region in [first, second] {
        spine.push_region_decision(SpineRegionDecision::minted(
            region,
            AstId(3),
            String::from("install -y \"$1\""),
            Disposition::Run,
            RegionRoutes::default(),
            authored(),
        ));
    }
    spine.push_render_decision(SpineRenderDecision::minted(
        None,
        Some(second),
        RenderDecision::Refused {
            cause: dorc_core::spine::RefusalCause::Heredoc,
        },
        authored(),
    ));

    let model =
        project(&spine, RecordedInvocationMode::Plan, authored()).expect("the Spine projects");
    let rendered = model.renders();
    assert_eq!(rendered.len(), 1, "one refusal was recorded");
    let row = &rendered[0];
    assert_eq!(
        row.kind(),
        RecordedRenderKind::RefusedHeredocRegion,
        "a region-keyed refusal must not read as a leaf-keyed one"
    );
    let RenderSubject::Region(ordinal) = row.subject() else {
        panic!(
            "a region refusal must wear the region axis, got {:?}",
            row.subject()
        );
    };
    assert_eq!(
        ordinal.get(),
        1,
        "the refusal belongs to the SECOND region, and the ordinal space is the one the \
         region rows were numbered in"
    );
    // And the reference resolves inside the declared space, which is all the model can check.
    assert_eq!(model.regions().len(), 2);
}

#[test]
fn every_species_the_projection_declines_states_its_population() {
    // `quarantine/30Rb`: a nonzero population the projection does not carry mints an explicit
    // omission. The census is over EVERY species, so a population cannot vanish by nobody
    // having written a row for it.
    let mut spine = spine_with_invocation();
    // A record-stream population the projection deliberately folds into the admission row.
    spine.set_admission(dorc_core::spine::SpineAdmission::minted(
        dorc_core::spine::AdmissionOutcome::NoObservation,
        None,
        authored(),
    ));

    let model =
        project(&spine, RecordedInvocationMode::Plan, authored()).expect("the Spine projects");
    let omitted: Vec<RecordedSpineSpecies> = model
        .omissions()
        .iter()
        .map(RecordedProjectionOmission::species)
        .collect();

    // Carried species never appear; declined ones always do, whatever their population.
    assert!(
        !omitted.contains(&RecordedSpineSpecies::Invocation),
        "a carried species must not also be reported omitted"
    );
    for declined in [
        RecordedSpineSpecies::RecordStream,
        RecordedSpineSpecies::PresentedPlan,
        RecordedSpineSpecies::Vouch,
        RecordedSpineSpecies::Observation,
        RecordedSpineSpecies::ValidityRound,
        RecordedSpineSpecies::Outcome,
    ] {
        assert!(
            omitted.contains(&declined),
            "{} is not carried and must say so",
            declined.token()
        );
    }
    assert_eq!(
        omitted.len(),
        SpineSpecies::ALL.len() - 10,
        "ten species are carried; every other one mints a row"
    );

    // The approval-surface identities are not minted yet, so the row that would state them is
    // absent rather than half-filled.
    assert!(model.presented().is_none());
    let presented = model
        .omissions()
        .iter()
        .find(|row| row.species() == RecordedSpineSpecies::PresentedPlan)
        .expect("the presented-plan omission is recorded");
    assert_eq!(presented.reason(), RecordedOmissionReason::NotProjectedV1);
}

#[test]
fn a_spine_with_no_invocation_has_no_document_to_write() {
    // Not an empty document: a run that never minted an invocation has nothing to say, and the
    // projection refuses rather than emitting a shape whose required singleton is missing.
    let spine = Spine::new();
    assert_eq!(
        project(&spine, RecordedInvocationMode::Plan, authored()),
        Err(ProjectionRefusal::NoInvocation)
    );
}

#[test]
fn a_licensed_verb_is_attributed_and_an_unlicensed_one_mints_nothing() {
    // The licensor row exists to name what licensed an irreversible verb, so a `Run` — which
    // licensed nothing — must mint no row at all rather than a row claiming a verb.
    let mut spine = spine_with_invocation();
    spine.set_disposition(dorc_core::spine::SpineDisposition::minted(
        dorc_core::SiteId::leaf(dorc_core::LeafId(0)),
        AstId(1),
        String::from("apt-get install -y nginx"),
        Disposition::Run,
        authored(),
    ));

    let model =
        project(&spine, RecordedInvocationMode::Plan, authored()).expect("the Spine projects");
    assert_eq!(model.sites().len(), 1, "the decision itself is recorded");
    assert!(
        model.licensors().is_empty(),
        "a run licenses no irreversible verb, so it attributes none"
    );
}
