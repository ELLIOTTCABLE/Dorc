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
    SpinePresentedPlan, SpineRegionDecision, SpineRenderDecision, SpineSpecies,
};
use dorc_core::{AstId, BytePos, DefinitionId, Interner, SourceFileId, SourceRole, Span};
use dorc_plan::planning_input::{PlanningInputs, PlanningMode, PlanningPolicy};
use dorc_plan::presentation::FinalPresentation;
use dorc_plan::receipt::{ProjectionRefusal, project};
use dorc_plan::{Disposition, NO_ARTIFACT_FORM, Plan, ProbePlan, Spine, SurvivalReport};
use dorc_receipt::plan::RenderSubject;
use dorc_receipt::rows::RecordedProjectionOmission;
use dorc_receipt::tokens::{
    ClosedToken, RecordedInvocationMode, RecordedOmissionReason, RecordedRenderKind,
    RecordedSpineSpecies,
};

fn authored() -> InfluenceAccount {
    InfluenceAccount::authored_before_contact()
}

/// The book every baseline witness is settled over.
const BASELINE_BOOK: &str = "";

fn invocation() -> SpineInvocation {
    SpineInvocation::minted(
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
    )
}

/// A witness over one settled surface. Two different books settle to two different surfaces.
fn witness_over(book: &str) -> FinalPresentation {
    let ast = dorc_syntax::parse(book).value;
    let plan = Plan::decided(
        vec![],
        Vec::new(),
        SurvivalReport::default(),
        false,
        NO_ARTIFACT_FORM,
        book,
        &ast,
        authored(),
    );
    FinalPresentation::of_settled(
        &plan,
        &ProbePlan::default(),
        book,
        &ast,
        &Interner::default(),
        &[],
        PlanningInputs::of(
            "dorc/test",
            &invocation(),
            None,
            None,
            PlanningPolicy::of(PlanningMode::Plan, false),
        ),
        None,
    )
}

fn witness() -> FinalPresentation {
    witness_over(BASELINE_BOOK)
}

/// A Spine carrying the two records the projection demands, and nothing else.
fn spine_with_invocation() -> Spine {
    let mut spine = Spine::new();
    spine.set_invocation(invocation());
    spine.set_presented_plan(SpinePresentedPlan::minted(
        witness().presented_plan(),
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

    let projected = project(&spine, RecordedInvocationMode::Plan, authored(), &witness())
        .expect("the Spine projects");
    let model = projected.model();
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

    let projected = project(&spine, RecordedInvocationMode::Plan, authored(), &witness())
        .expect("the Spine projects");
    let model = projected.model();
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
        SpineSpecies::ALL.len() - 11,
        "eleven species are carried; every other one mints a row"
    );

    // The reason axis carries real distinctions, so one is pinned: the stream is not "not yet"
    // work, it is content that rides the admission row as an opaque slot and never gets a row.
    let stream = model
        .omissions()
        .iter()
        .find(|row| row.species() == RecordedSpineSpecies::RecordStream)
        .expect("the record-stream omission is recorded");
    assert_eq!(stream.reason(), RecordedOmissionReason::ContentExcluded);

    // The approval surface is CARRIED now, so it must neither be reported omitted nor be absent.
    assert!(
        !omitted.contains(&RecordedSpineSpecies::PresentedPlan),
        "the approval surface is carried and must not also say it was omitted"
    );
    let presented = model.presented().expect("the surface projects a row");
    assert_eq!(presented.planning_input(), witness().planning_input().hex());
    assert_eq!(presented.presented_plan(), witness().presented_plan().hex());
    // No image is built at plan time, so the optional field reads absent rather than inventing one.
    assert_eq!(presented.planned_image(), None);
}

#[test]
fn a_spine_with_no_invocation_has_no_document_to_write() {
    // Not an empty document: a run that never minted an invocation has nothing to say, and the
    // projection refuses rather than emitting a shape whose required singleton is missing.
    let spine = Spine::new();
    assert_eq!(
        project(&spine, RecordedInvocationMode::Plan, authored(), &witness()),
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

    let projected = project(&spine, RecordedInvocationMode::Plan, authored(), &witness())
        .expect("the Spine projects");
    let model = projected.model();
    assert_eq!(model.sites().len(), 1, "the decision itself is recorded");
    assert!(
        model.licensors().is_empty(),
        "a run licenses no irreversible verb, so it attributes none"
    );
}

#[test]
fn a_witness_from_another_surface_cannot_supply_this_one_s_identities() {
    // THE substitution trap. A witness carries two identities the Spine has no copy of, so nothing
    // in the document could contradict them — a witness from a different plan would supply this
    // run's `planning-input` and its own image, and the receipt would validate cleanly while
    // naming inputs this run never consumed. The one identity BOTH hold is what makes the swap
    // detectable, so the projection compares it and refuses.
    let other = witness_over("true\n");
    assert_ne!(
        other.presented_plan(),
        witness().presented_plan(),
        "the two books must settle to different surfaces, or this case proves nothing"
    );
    let spine = spine_with_invocation();
    assert_eq!(
        project(&spine, RecordedInvocationMode::Plan, authored(), &other),
        Err(ProjectionRefusal::PresentationMismatch)
    );
}

#[test]
fn a_run_that_recorded_no_surface_has_nothing_for_a_witness_to_answer_to() {
    // The other half: with no recorded surface there is nothing to compare against, and accepting
    // the witness anyway would let it vouch for itself — which is exactly the swap above, minus
    // the evidence that would catch it.
    let mut spine = Spine::new();
    spine.set_invocation(invocation());
    assert_eq!(
        project(&spine, RecordedInvocationMode::Plan, authored(), &witness()),
        Err(ProjectionRefusal::NoPresentedPlan)
    );
}

#[test]
fn the_projected_order_is_the_canonical_one() {
    // LOAD-BEARING, not tidiness. A detail entry is keyed by its record's POSITION, and the model
    // re-emits records in `PlanReceipt::KINDS` order. If the projection's own walk drifted from
    // that order, every detail would enrich whichever row happened to share its integer — and the
    // document would still validate, because a position is range-checked and never sense-checked.
    // This is the same hazard the region-ordinal walk carries, one level up.
    let mut spine = spine_with_invocation();
    spine.set_disposition(dorc_core::spine::SpineDisposition::minted(
        dorc_core::SiteId::leaf(dorc_core::LeafId(0)),
        AstId(1),
        String::from("apt-get update"),
        Disposition::Run,
        authored(),
    ));
    spine.set_disposition(dorc_core::spine::SpineDisposition::minted(
        dorc_core::SiteId::leaf(dorc_core::LeafId(1)),
        AstId(2),
        String::from("cp ./a ./b"),
        Disposition::Run,
        authored(),
    ));

    let projected = project(&spine, RecordedInvocationMode::Plan, authored(), &witness())
        .expect("the Spine projects");
    assert_eq!(
        projected.records(),
        projected
            .model()
            .to_records()
            .expect("the model re-emits")
            .as_slice(),
        "the walk that numbered the details and the walk that emits the document must agree"
    );
}

#[test]
fn a_detail_is_offered_for_every_slot_the_row_marked_captured_and_no_other() {
    // The writer's half of the two-way account. The reader recomputes the required set from the
    // skeleton alone, so a projection offering a value for an unmarked slot, or marking a slot it
    // cannot fill, produces a document its own reader refuses.
    let mut spine = spine_with_invocation();
    spine.set_disposition(dorc_core::spine::SpineDisposition::minted(
        dorc_core::SiteId::leaf(dorc_core::LeafId(0)),
        AstId(1),
        String::from("apt-get update"),
        Disposition::Run,
        authored(),
    ));

    let projected = project(&spine, RecordedInvocationMode::Plan, authored(), &witness())
        .expect("the Spine projects");
    let skeleton = dorc_receipt::format::Skeleton {
        receipt_id: "a".repeat(64),
        signing_key_id: "b".repeat(64),
        encryption_key_id: Some("c".repeat(64)),
        records: projected.records().to_vec(),
    };
    let mut offered: Vec<(u64, dorc_receipt::projection::OpaqueFieldTag)> = projected
        .details()
        .iter()
        .map(|entry| (entry.record(), entry.tag()))
        .collect();
    offered.sort_by_key(|(record, tag)| (*record, tag.order()));
    assert_eq!(
        offered,
        dorc_receipt::overlay::captured_slots(&skeleton),
        "the offered details and the skeleton's own captured account must agree exactly"
    );
    assert!(
        !offered.is_empty(),
        "a run holding a target, a source path and a site's shell offers details, so an empty \
         account here would make the comparison vacuous"
    );
}
