//! Reconstructing from what a receipt-rooted question read back.
//!
//! One walk, one canonical order, no filesystem and no second traversal: the edge has already
//! selected a root, walked the graph and read whatever current source it chose to, and this seat
//! receives the OUTCOMES as data (`inv-receipt-collection-never-expands-observation` — reading a
//! receipt back performs no additional observation).
//!
//! # Both root species, one product face
//!
//! `dorc_receipt::report::RecordedWhyFacts` is plan-typed, so an intent- or outcome-rooted question
//! has no sealed model. [`Rooted`] is what keeps that from becoming two product surfaces: a
//! non-plan root reconstructs its own root facts, its graph correlations, and typed carrier-absence
//! for everything a plan root would have carried. The DEPTH differs; the shape does not.

use dorc_aid::narrative::SpeechAct;
use dorc_receipt::plan::RenderSubject;
use dorc_receipt::report::{
    AddressResolution, AuthenticationState, ClosureCompleteness, CurrentSourceState, DetailState,
    FamilyCoverage, PlanFamily, ProjectionState, RecordedDocumentId, RecordedSpecies,
    RecordedWhyFacts, SiblingState, SiteFacts, StageFacts,
};
use dorc_receipt::tokens::{
    RecordedApplyPolicy, RecordedLicenseCustody, RecordedOriginState, RecordedSpeechAct,
    RecordedTerminalState,
};

use crate::datum::{
    AddressSubject, AttemptLineage, CarrierRef, CorrelationFact, Datum, Delivery, HostName,
    IdentityFact, Moment, NegativeKind, NegativeSpace, Payload, RecordedFlag, RecordedToken,
    Separability, Speaker, StateFact, Subject, UnplaceableAddress, Voice, VoiceSet,
    WorldCoordinate,
};
use crate::known::{CantTell, CarrierAbsence, Held, Known};
use crate::structure::{
    Locus, LocusAddress, LocusDag, LocusEdge, Namespace, SourceAgreement, Structure,
};
use crate::{Carrier, CarrierRole, Reconstruction};

/// Which species of root a question is about.
///
/// The non-plan arm carries only what the edge can establish WITHOUT a sealed model, and that is
/// the honest ceiling: promoting an intent's own records into plan-shaped facts would be inventing
/// a projection nobody wrote.
#[derive(Debug)]
pub enum Rooted<'a> {
    /// A plan receipt, with its sealed report model.
    Plan(&'a RecordedWhyFacts),
    /// An apply intent or outcome: identity, standing, and correlations only.
    OtherSpecies(&'a NonPlanRoot),
}

/// What the edge established about a root the report model does not cover.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonPlanRoot {
    /// Which document.
    pub document: RecordedDocumentId,
    /// What outer verification said.
    pub authentication: AuthenticationState,
    /// Which projection it is.
    pub projection: ProjectionState,
    /// Whether its grouped detail region opened.
    pub detail: DetailState,
    /// The store order it was filed under, as spelled; `None` where it carries the undated token.
    pub order: Option<String>,
    /// The typed correlations the graph walk produced from this root.
    pub correlations: Vec<CorrelationFact>,
    /// What is wrong with each required sibling not in hand.
    pub siblings: Vec<SiblingState>,
    /// An intent root's own shallow facts, where the root is one.
    pub intent: Option<ShallowIntent>,
    /// An outcome root's own shallow facts, where the root is one.
    pub outcome: Option<ShallowOutcome>,
}

/// An apply intent's own facts, at the depth the already-public read-back accessors answer.
///
/// SHALLOW by disclosure (`30Vd:tc-nonplan-root-depth`): per-assignment rows would need sealed
/// accessors nobody has written, so what is here is what a reader can have today and the deeper
/// projection stays a named, standing flag rather than an implied promise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShallowIntent {
    /// Which publication route authorized the apply.
    pub policy: RecordedApplyPolicy,
    /// Whether any assignment names an originating plan.
    pub origin_state: RecordedOriginState,
    /// How many assignments the intent carries.
    pub assignment_count: u64,
    /// Every originating plan this intent names, duplicates retained as the document spells them.
    pub origin_receipts: Vec<RecordedDocumentId>,
}

/// An apply outcome's own facts, at the same shallow depth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShallowOutcome {
    /// The graceful terminal state the apply reached.
    pub terminal: RecordedTerminalState,
    /// How many site rows the outcome carries.
    pub site_count: u64,
    /// The intent this outcome answers, where it names a readable one.
    pub intent: Option<RecordedDocumentId>,
}

/// What the EDGE could do with the address the question named.
///
/// A plan root's placed address travels inside `RecordedWhyFacts`, so the only thing this adds is
/// the case the report model cannot represent: a request the edge could not turn into an ordinal at
/// all. That is a fact about the QUESTION rather than about any document, which is why it arrives
/// beside the root rather than inside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddressStanding {
    /// No address was named, or the root's own facts already answer the one that was.
    #[default]
    AsRecorded,
    /// The question named an address this route could not place at all.
    Unplaceable(UnplaceableAddress),
}

/// Reconstruct one rooted question.
#[must_use]
pub fn reconstruct(rooted: &Rooted<'_>, address: AddressStanding) -> Reconstruction {
    match rooted {
        Rooted::Plan(facts) => from_plan(facts, address),
        Rooted::OtherSpecies(root) => from_non_plan(root, address),
    }
}

/// The question's own unplaceable address, as one datum.
///
/// Its delivery names the ROOT carrier because the root is the document the address was matched
/// against — the recorded source table it failed to find a digest in — while its subject is the
/// question, because nothing the document says is what went wrong.
fn push_unplaceable_address(
    data: &mut Vec<Datum>,
    address: AddressStanding,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let AddressStanding::Unplaceable(why) = address else {
        return;
    };
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        Known::present(Subject::Question),
        Known::present(Payload::Unplaceable(why)),
        here,
    ));
}

/// The engine speaking in its own voice — the terminal attribution link
/// (`30V` §2 rul-first-person-register): a claim grounded in nobody else's speech is ours.
fn ours(act: SpeechAct) -> Known<Speaker> {
    Known::present(Speaker::of(act, Known::present(VoiceSet::Mine)))
}

/// An author, named by the source their bytes sit in.
fn authored(source: crate::datum::SourceRef) -> Known<Speaker> {
    Known::present(Speaker::of(
        SpeechAct::Claimed,
        Known::present(VoiceSet::One(Voice::AuthoredIn(source))),
    ))
}

/// A recorded speech act whose SPEAKER-SET the document does not name.
///
/// The act is real — it is on the narrative row — and the voices are not: narrative operands are
/// not durable, so nobody can be named. Two leaves, two answers, which is exactly why the voice-set
/// carries its own wrapper rather than riding the act.
fn spoke(act: SpeechAct) -> Known<Speaker> {
    Known::present(Speaker::of(act, Known::report_api_lacks()))
}

/// The engine's own speech-act vocabulary, for one recorded act.
///
/// One-to-one and no-wildcard: the two vocabularies are the same seven kinds by construction, and a
/// widening on either side must visit this seat rather than silently mapping to a neighbour — which
/// would dress one act as another, the worst aid failure (`271:rul-sin-ordering`).
const fn speech_of(recorded: RecordedSpeechAct) -> SpeechAct {
    match recorded {
        RecordedSpeechAct::Measured => SpeechAct::Measured,
        RecordedSpeechAct::Vouched => SpeechAct::Vouched,
        RecordedSpeechAct::Ran => SpeechAct::Ran,
        RecordedSpeechAct::Claimed => SpeechAct::Claimed,
        RecordedSpeechAct::Derived => SpeechAct::Derived,
        RecordedSpeechAct::Consented => SpeechAct::Consented,
        RecordedSpeechAct::Declined => SpeechAct::Declined,
    }
}

/// The world-coordinate every datum of one document shares.
///
/// The host leaf comes off the invocation projection: the destination is recorded in the region and
/// the projection now carries it, so a run against a named host says so instead of reading as a
/// hole. A document that withheld or never collected it keeps its own absence word.
fn coordinate(
    moment: Known<Moment>,
    document: &RecordedDocumentId,
    host: Known<HostName>,
) -> WorldCoordinate {
    WorldCoordinate::of(
        moment,
        host,
        Known::present(AttemptLineage::Document(document.clone())),
    )
}

/// The recorded destination, where the document released it.
fn host_of(facts: &RecordedWhyFacts) -> Known<HostName> {
    let invocation = facts.invocation();
    invocation.target_text().map_or_else(
        || from_material(invocation.target()),
        |value| Known::present(HostName::of(value.clone())),
    )
}

fn from_plan(facts: &RecordedWhyFacts, address: AddressStanding) -> Reconstruction {
    let root = facts.root();
    let document = root.document().clone();
    let order = root.order().spelled();
    let moment = if order.bytes().all(|byte| byte == b'0') {
        Known::present(Moment::Undated)
    } else {
        Known::present(Moment::Filed(order))
    };

    let mut carriers = vec![Carrier {
        document: document.clone(),
        species: root.species(),
        role: CarrierRole::Root,
        authentication: Known::present(root.authentication()),
        projection: Known::present(root.projection()),
        detail: Known::present(root.detail()),
    }];
    for reached in facts.closure().reached() {
        if reached == &document {
            continue;
        }
        carriers.push(reached_carrier(reached, CarrierRole::Reached));
    }
    for sibling in facts.closure().siblings() {
        carriers.push(reached_carrier(
            sibling.document(),
            CarrierRole::Sibling(sibling.clone()),
        ));
    }

    let here = Delivery::Recorded(CarrierRef::of(0));
    let world = coordinate(moment, &document, host_of(facts));
    let mut data = Vec::new();
    push_root_data(&mut data, facts, &world, here);
    push_invocation_data(&mut data, facts, &world, here);
    push_source_data(&mut data, facts, &world, here);
    push_site_data(&mut data, facts, &world, here);
    push_narrative_data(&mut data, facts, &world, here);
    push_admission_data(&mut data, facts, &world, here);
    push_presented_data(&mut data, facts, &world, here);
    push_region_data(&mut data, facts, &world, here);
    push_load_data(&mut data, facts, &world, here);
    push_classification_data(&mut data, facts, &world, here);
    push_certification_data(&mut data, facts, &world, here);
    push_ship_data(&mut data, facts, &world, here);
    push_survival_data(&mut data, facts, &world, here);
    push_render_data(&mut data, facts, &world, here);
    push_licensor_data(&mut data, facts, &world, here);
    push_omission_data(&mut data, facts, &world, here);
    push_address_data(&mut data, facts, &world, here);
    push_unplaceable_address(&mut data, address, &world, here);
    push_correlation_data(&mut data, &carriers, &world, here);
    push_uncovered_families(&mut data, facts, &world, here);

    // A PLAN root reaches nothing later (`30R:receipt-rooted-attention-and-cli` walks toward
    // causes), so its correlation family is empty by the walk's own direction, not by omission.
    Reconstruction::of(carriers, data, Structure::of(Vec::new(), loci_of(facts)))
}

fn reached_carrier(document: &RecordedDocumentId, role: CarrierRole) -> Carrier {
    Carrier {
        document: document.clone(),
        species: document.species(),
        // A reached or missing sibling's own standing is the EDGE's to establish per document, and
        // this question read only its root: saying anything else would promote a hope.
        authentication: Known::report_api_lacks(),
        projection: Known::report_api_lacks(),
        detail: Known::report_api_lacks(),
        role,
    }
}

fn push_root_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let root = facts.root();
    let subject = Known::present(Subject::Document(root.document().clone()));
    let states = [
        StateFact::Authentication(root.authentication()),
        StateFact::Projection(root.projection()),
        StateFact::Detail(root.detail()),
        StateFact::Closure(facts.closure().completeness()),
        StateFact::ReDerivation(facts.rederivation()),
    ];
    for state in states {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::State(state)),
            here,
        ));
    }
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        subject.clone(),
        Known::present(Payload::Identity(IdentityFact::Species(root.species()))),
        here,
    ));
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        subject,
        Known::present(Payload::Identity(IdentityFact::Document(
            root.document().clone(),
        ))),
        here,
    ));
}

fn push_source_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for source in facts.sources() {
        let reference = crate::datum::SourceRef::of(source.ordinal());
        let subject = Known::present(Subject::Source(reference));
        for identity in [
            IdentityFact::SourceClass(source.class()),
            IdentityFact::Digest(source.digest().to_owned()),
            IdentityFact::Bytes(source.bytes()),
        ] {
            data.push(Datum::minted(
                ours(SpeechAct::Derived),
                world.clone(),
                subject.clone(),
                Known::present(Payload::Identity(identity)),
                here,
            ));
        }
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::State(StateFact::CurrentSource(source.current()))),
            here,
        ));
        data.push(Datum::minted(
            authored(reference),
            world.clone(),
            subject,
            source.text().map_or_else(
                || from_material(source.content()),
                |text| Known::present(Payload::Text(text.clone())),
            ),
            here,
        ));
    }
}

/// One `report::MaterialState`, as this model's absence vocabulary.
///
/// Total and one-to-one: the four absences stay four, because merging any pair invents a cause the
/// document never stated.
pub(crate) fn from_material<T>(state: dorc_receipt::report::MaterialState) -> Known<T> {
    use crate::known::WithholdReason;
    use dorc_receipt::report::MaterialState;
    match state {
        // `Held` reaching here means the skeleton says a value rides the region and none was handed
        // over — a truncation of the material, never an unbuilt slot. Answering `nyi` would put a
        // caller's inconsistency into the census that tracks OUR unbuilt work.
        MaterialState::Held => Known::Knowable(Held::CouldNotTell(CantTell::Truncated)),
        MaterialState::WithheldPlain => {
            Known::Knowable(Held::Withheld(WithholdReason::PlainProjection))
        }
        MaterialState::OmittedByLimit => {
            Known::Knowable(Held::Withheld(WithholdReason::BoundRefused))
        }
        MaterialState::Undecodable => {
            Known::Knowable(Held::Withheld(WithholdReason::RegionUnavailable))
        }
        MaterialState::Unavailable => Known::absent(CarrierAbsence::RunHeldNoValue),
        MaterialState::Uncollected => Known::absent(CarrierAbsence::ProjectionUncollected),
    }
}

fn push_site_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for site in facts.sites() {
        let subject = Known::present(Subject::Site(site.site()));
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Decision(site.disposition())),
            here,
        ));
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Influence(site.influence())),
            here,
        ));
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Identity(IdentityFact::Ast(site.ast().get()))),
            here,
        ));
        let speaker = site
            .authored_origin()
            .and_then(StageFacts::source)
            .map_or_else(Known::report_api_lacks, |ordinal| {
                authored(crate::datum::SourceRef::of(ordinal))
            });
        data.push(Datum::minted(
            speaker,
            world.clone(),
            subject.clone(),
            site.shell_text().map_or_else(
                || from_material(site.shell()),
                |text| Known::present(Payload::Text(text.clone())),
            ),
            here,
        ));
        for (index, _) in site.chain().iter().enumerate() {
            data.push(Datum::minted(
                ours(SpeechAct::Derived),
                world.clone(),
                Known::present(Subject::Stage {
                    site: site.site(),
                    index: u32::try_from(index).unwrap_or(u32::MAX),
                }),
                stage_payload(site, index),
                here,
            ));
        }
    }
}

/// One provenance stage's own payload: its generated label or origin claim where it carries one.
fn stage_payload(site: &SiteFacts, index: usize) -> Known<Payload> {
    site.chain().get(index).map_or_else(
        || Known::Knowable(Held::CouldNotTell(CantTell::Truncated)),
        |stage| {
            stage.text().map_or_else(
                || Known::present(Payload::Identity(IdentityFact::Count(0))),
                |text| Known::present(Payload::Text(text.clone())),
            )
        },
    )
}

fn push_omission_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for omission in facts.omissions() {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            Known::present(Subject::Document(facts.root().document().clone())),
            Known::present(Payload::Identity(IdentityFact::UncarriedSpecies(
                omission.species(),
            ))),
            here,
        ));
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            Known::present(Subject::Document(facts.root().document().clone())),
            Known::present(Payload::Identity(IdentityFact::Count(u64::from(
                omission.count(),
            )))),
            here,
        ));
    }
}

fn push_address_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let Some(address) = facts.address() else {
        return;
    };
    let subject = Known::present(Subject::Address(AddressSubject {
        source: crate::datum::SourceRef::of(address.requested().source()),
        line: address.requested().line(),
    }));
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        subject.clone(),
        Known::present(Payload::State(StateFact::CurrentSource(address.current()))),
        here,
    ));
    // A resolution that ADMITS a site names it; every other arm is an affirmative not-knowing, and
    // the model must not let the recorded-site-that-did-not-answer read as the answer.
    let payload = match address.resolution() {
        AddressResolution::Resolved { site } => {
            Known::present(Payload::Identity(IdentityFact::Ast(site.leaf().get())))
        }
        AddressResolution::ChangedLine { .. } | AddressResolution::ComparisonUnavailable { .. } => {
            Known::Knowable(Held::CouldNotTell(CantTell::ComparisonNotMade))
        }
        AddressResolution::Unresolved(_) => Known::absent(CarrierAbsence::RunHeldNoValue),
    };
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        subject,
        payload,
        here,
    ));
}

fn push_correlation_data(
    data: &mut Vec<Datum>,
    carriers: &[Carrier],
    world: &WorldCoordinate,
    here: Delivery,
) {
    for carrier in carriers {
        if let CarrierRole::Sibling(state) = &carrier.role {
            data.push(Datum::minted(
                ours(SpeechAct::Derived),
                world.clone(),
                Known::present(Subject::Document(state.document().clone())),
                Known::present(Payload::Identity(IdentityFact::Species(carrier.species))),
                here,
            ));
        }
    }
}

/// The invocation singleton: what the run was, and how much of itself it recorded.
fn push_invocation_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let invocation = facts.invocation();
    let subject = Known::present(Subject::Document(facts.root().document().clone()));
    for identity in [
        IdentityFact::InvocationMode(invocation.mode()),
        IdentityFact::Count(u64::from(invocation.attempt())),
    ] {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Identity(identity)),
            here,
        ));
    }
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        subject,
        Known::present(Payload::Influence(invocation.influence())),
        here,
    ));
}

/// Every decision-inert narrative, wearing the act the document recorded.
///
/// This family is why the speaker axis is real rather than uniformly first-person: the engine's own
/// derivations still speak as us, and a vouch, a decline or a measurement recorded here speaks in
/// its own act, with its voices honestly unnamed.
fn push_narrative_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for narrative in facts.narratives() {
        let subject = Known::present(Subject::Narrative(narrative.ordinal()));
        data.push(Datum::minted(
            spoke(speech_of(narrative.speech())),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Collapse(narrative.kind())),
            here,
        ));
        data.push(Datum::minted(
            spoke(speech_of(narrative.speech())),
            world.clone(),
            subject,
            Known::present(Payload::Influence(narrative.influence())),
            here,
        ));
    }
}

/// One datum the engine speaks in its own derived voice, at the document's shared coordinate.
fn derived(
    data: &mut Vec<Datum>,
    world: &WorldCoordinate,
    subject: &Known<Subject>,
    payload: Known<Payload>,
    here: Delivery,
) {
    data.push(Datum::minted(
        ours(SpeechAct::Derived),
        world.clone(),
        subject.clone(),
        payload,
        here,
    ));
}

/// A recorded value where the document released it, and the absence its state names where it did
/// not — never an empty payload standing in for either.
fn text_or_absence(
    text: Option<&dorc_receipt::report::RecordedValue>,
    state: dorc_receipt::report::MaterialState,
) -> Known<Payload> {
    text.map_or_else(
        || from_material(state),
        |value| Known::present(Payload::Text(value.clone())),
    )
}

/// A recorded count, or the affirmative absence of one an optional slot leaves.
fn count_or_absence(count: Option<u32>) -> Known<Payload> {
    count.map_or_else(
        || Known::absent(CarrierAbsence::RunHeldNoValue),
        |value| Known::present(Payload::Identity(IdentityFact::Count(u64::from(value)))),
    )
}

/// The records-admission singleton: what the intake edge answered, and how much it accounted for.
fn push_admission_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let Some(admission) = facts.admission() else {
        return;
    };
    let subject = Known::present(Subject::Document(facts.root().document().clone()));
    derived(
        data,
        world,
        &subject,
        Known::present(Payload::Token(RecordedToken::AdmissionOutcome(
            admission.outcome(),
        ))),
        here,
    );
    derived(
        data,
        world,
        &subject,
        Known::present(Payload::Identity(IdentityFact::Count(admission.records()))),
        here,
    );
    derived(
        data,
        world,
        &subject,
        Known::present(Payload::Identity(IdentityFact::Bytes(admission.bytes()))),
        here,
    );
    derived(
        data,
        world,
        &subject,
        text_or_absence(admission.stream_text(), admission.stream()),
        here,
    );
    derived(
        data,
        world,
        &subject,
        Known::present(Payload::Influence(admission.influence())),
        here,
    );
}

/// The presented-plan singleton: the three identities of one approval surface.
fn push_presented_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let Some(presented) = facts.presented() else {
        return;
    };
    let subject = Known::present(Subject::Document(facts.root().document().clone()));
    for digest in [presented.planning_input(), presented.presented_plan()] {
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Identity(IdentityFact::Digest(digest.to_owned()))),
            here,
        );
    }
    derived(
        data,
        world,
        &subject,
        presented.planned_image().map_or_else(
            || Known::absent(CarrierAbsence::RunHeldNoValue),
            |digest| Known::present(Payload::Identity(IdentityFact::Digest(digest.to_owned()))),
        ),
        here,
    );
    derived(
        data,
        world,
        &subject,
        Known::present(Payload::Influence(presented.influence())),
        here,
    );
}

/// Every authored region's shared outcome, keyed by REGION and never by one of its executions.
fn push_region_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for region in facts.regions() {
        let subject = Known::present(Subject::Region(region.region()));
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Decision(region.disposition())),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Identity(IdentityFact::Ast(region.ast().get()))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Identity(IdentityFact::Count(region.routes()))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            text_or_absence(region.shell_text(), region.shell()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(region.influence())),
            here,
        );
    }
}

/// Every definition-plane decision — what a load did to the function environment.
fn push_load_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for load in facts.loads() {
        let subject = Known::present(Subject::Load(load.ordinal()));
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Token(RecordedToken::LoadOutcome(load.outcome()))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            text_or_absence(load.name_text(), load.name()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            text_or_absence(load.custody_text(), load.custody()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(load.influence())),
            here,
        );
    }
}

/// Every site classification — what the analysis took each site to BE.
fn push_classification_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for classification in facts.classifications() {
        let subject = Known::present(Subject::Site(classification.site()));
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Token(RecordedToken::SiteClass(
                classification.class(),
            ))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Identity(IdentityFact::Ast(
                classification.ast().get(),
            ))),
            here,
        );
        for flag in [
            RecordedFlag::VerdictLane(classification.verdict_lane()),
            RecordedFlag::Invalidator(classification.invalidator()),
        ] {
            derived(
                data,
                world,
                &subject,
                Known::present(Payload::Flag(flag)),
                here,
            );
        }
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Identity(IdentityFact::Operands(
                classification.cells(),
            ))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(classification.influence())),
            here,
        );
    }
}

/// Every dataflow certification — the solver's second opinion about itself, per pass.
fn push_certification_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    let subject = Known::present(Subject::Document(facts.root().document().clone()));
    for certification in facts.certifications() {
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Token(RecordedToken::SolvePass(
                certification.pass(),
            ))),
            here,
        );
        for flag in [
            RecordedFlag::SolveConsistent(certification.consistent()),
            RecordedFlag::SolveTripped(certification.tripped()),
        ] {
            derived(
                data,
                world,
                &subject,
                Known::present(Payload::Flag(flag)),
                here,
            );
        }
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(certification.influence())),
            here,
        );
    }
}

/// Every probe shipment — which authored body each site sent to the host.
fn push_ship_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for ship in facts.ships() {
        let subject = Known::present(Subject::Site(ship.site()));
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Token(RecordedToken::ShipLane(ship.lane()))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            text_or_absence(ship.source_text(), ship.source()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(ship.influence())),
            here,
        );
    }
}

/// Every survival-tier outcome — whether a fact reached its site, and what stopped it.
fn push_survival_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for survival in facts.survivals() {
        let subject = Known::present(Subject::Site(survival.site()));
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Token(RecordedToken::SurvivalOutcome(
                survival.outcome(),
            ))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            count_or_absence(survival.wall()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            count_or_absence(survival.aggregate()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            text_or_absence(survival.poison_text(), survival.poison()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(survival.influence())),
            here,
        );
    }
}

/// Every render-time decision — the edits Dorc made to the artifact it generated.
///
/// The subject follows the row's own axis rather than a chosen one: a leaf-keyed edit is about a
/// site, a region-keyed edit about the authored region, and an unkeyed one about the document.
fn push_render_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for render in facts.renders() {
        let subject = Known::present(match render.subject() {
            RenderSubject::Leaf(site) => Subject::Site(site),
            RenderSubject::Region(ordinal) => Subject::Region(ordinal.get()),
            RenderSubject::None => Subject::Document(facts.root().document().clone()),
        });
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Token(RecordedToken::RenderKind(render.kind()))),
            here,
        );
        derived(
            data,
            world,
            &subject,
            text_or_absence(render.detail_text(), render.detail()),
            here,
        );
        derived(
            data,
            world,
            &subject,
            Known::present(Payload::Influence(render.influence())),
            here,
        );
    }
}

/// Every licensor of an irreversible verb.
///
/// The ONE family whose rows do not speak in the engine's own voice: recorded custody says WHOSE
/// utterance a license rested on, and `30V` §2 rul-first-person-register puts the tool's "I" only
/// where no more-correct register exists. The voices stay unnamed — the authoring locus is an
/// opaque slot, so the act is knowable and the speaker-set is not (`a-voice-set-is-its-own-leaf`).
fn push_licensor_data(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for licensor in facts.licensors() {
        let subject = Known::present(Subject::Site(licensor.site()));
        let speaker = custody_speaker(licensor.custody());
        for payload in [
            Payload::Token(RecordedToken::LicenseVerb(licensor.license())),
            Payload::Token(RecordedToken::LicenseCustody(licensor.custody())),
            Payload::Influence(licensor.influence()),
        ] {
            data.push(Datum::minted(
                speaker.clone(),
                world.clone(),
                subject.clone(),
                Known::present(payload),
                here,
            ));
        }
        data.push(Datum::minted(
            speaker.clone(),
            world.clone(),
            subject,
            text_or_absence(licensor.locus_text(), licensor.locus()),
            here,
        ));
    }
}

/// The act a recorded custody was performed in.
///
/// No-wildcard, so a widened custody vocabulary visits this seat. `VouchedSeverally` answers the
/// same ACT as a single vouch — several authors each vouching is still vouching — and the
/// severally-ness rides its own payload token rather than being folded into the act, which is what
/// keeps `speech_of`'s one-to-one discipline from being quietly widened here.
const fn custody_act(custody: RecordedLicenseCustody) -> SpeechAct {
    match custody {
        RecordedLicenseCustody::Vouched | RecordedLicenseCustody::VouchedSeverally => {
            SpeechAct::Vouched
        }
        RecordedLicenseCustody::MeasuredSelf => SpeechAct::Measured,
    }
}

/// Who performed a recorded custody's act, at the cardinality the custody word states.
///
/// `vouched-severally` populates an INSEPARABLE committee whose members are unnamed, rather than
/// the absent voice-set its siblings get: the document says several authors each vouched, and a
/// remedy against that license has to reach every one of them (`30V` §2 rul-remedies-may-fork).
/// Spelling it as one unnamed voice would understate the fork by exactly the thing the custody word
/// exists to say. The members stay EMPTY because narrative operands are not durable — the act and
/// its cardinality are knowable and the names are not (`a-voice-set-is-its-own-leaf`).
fn custody_speaker(custody: RecordedLicenseCustody) -> Known<Speaker> {
    let act = custody_act(custody);
    match custody {
        RecordedLicenseCustody::VouchedSeverally => Known::present(Speaker::of(
            act,
            Known::present(VoiceSet::Committee {
                voices: Vec::new(),
                separability: Separability::Inseparable,
            }),
        )),
        RecordedLicenseCustody::Vouched | RecordedLicenseCustody::MeasuredSelf => spoke(act),
    }
}

/// One datum per family this read surface does not carry typed facts for, so a hole is a NAMED row
/// of the total surface rather than a silence somebody has to notice.
///
/// Driven by the report's OWN coverage answer rather than a list kept here: two lists of the same
/// families would disagree the moment one is projected, and the disagreement would read as a hole
/// that no longer exists.
fn push_uncovered_families(
    data: &mut Vec<Datum>,
    facts: &RecordedWhyFacts,
    world: &WorldCoordinate,
    here: Delivery,
) {
    for (family, coverage) in facts.coverage() {
        let kind = match coverage {
            FamilyCoverage::Projected(_) => continue,
            FamilyCoverage::RecordedButUnprojected => NegativeKind::ReportApiGap,
            FamilyCoverage::NotCarried | FamilyCoverage::NotRelevant => NegativeKind::CarrierGap,
        };
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            Known::present(Subject::Family(family)),
            Known::present(Payload::NegativeSpace(NegativeSpace { kind, family })),
            here,
        ));
    }
}

/// Every recorded site's provenance chain, flattened into one walkable DAG.
fn loci_of(facts: &RecordedWhyFacts) -> LocusDag {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for site in facts.sites() {
        let first = nodes.len();
        for (index, stage) in site.chain().iter().enumerate() {
            let position = nodes.len();
            if position > first {
                edges.push(LocusEdge {
                    from: position.saturating_sub(1),
                    to: position,
                });
            }
            nodes.push(Locus {
                site: site.site(),
                stage: stage.kind(),
                index: u32::try_from(index).unwrap_or(u32::MAX),
                namespace: Namespace::Recorded,
                address: address_of(facts, stage),
                agreement: agreement_of(facts, stage),
            });
        }
    }
    LocusDag::of(nodes, edges)
}

fn address_of(_facts: &RecordedWhyFacts, stage: &StageFacts) -> Known<LocusAddress> {
    match (stage.source(), stage.span()) {
        (Some(source), Some(span)) => Known::present(LocusAddress {
            source: crate::datum::SourceRef::of(source),
            span,
        }),
        _ => Known::absent(CarrierAbsence::RunHeldNoValue),
    }
}

fn agreement_of(facts: &RecordedWhyFacts, stage: &StageFacts) -> SourceAgreement {
    stage
        .source()
        .map_or(SourceAgreement::NotCompared, |source| {
            facts
                .sources()
                .iter()
                .find(|candidate| candidate.ordinal() == source)
                .map_or(SourceAgreement::NotCompared, |candidate| {
                    match candidate.current() {
                        CurrentSourceState::Matching => SourceAgreement::Agrees,
                        CurrentSourceState::Drifted => SourceAgreement::Differs,
                        CurrentSourceState::Absent
                        | CurrentSourceState::Unreadable
                        | CurrentSourceState::NotCompared => SourceAgreement::NotCompared,
                    }
                })
        })
}

fn from_non_plan(root: &NonPlanRoot, address: AddressStanding) -> Reconstruction {
    let mut carriers = vec![Carrier {
        document: root.document.clone(),
        species: root.document.species(),
        role: CarrierRole::Root,
        authentication: Known::present(root.authentication),
        projection: Known::present(root.projection),
        detail: Known::present(root.detail),
    }];
    for sibling in &root.siblings {
        carriers.push(reached_carrier(
            sibling.document(),
            CarrierRole::Sibling(sibling.clone()),
        ));
    }

    let here = Delivery::Recorded(CarrierRef::of(0));
    let moment = root.order.clone().map_or_else(
        || Known::present(Moment::Undated),
        |spelled| Known::present(Moment::Filed(spelled)),
    );
    let world = coordinate(moment, &root.document, Known::report_api_lacks());
    let subject = Known::present(Subject::Document(root.document.clone()));

    let mut data = vec![
        Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Identity(IdentityFact::Species(
                root.document.species(),
            ))),
            here,
        ),
        Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Identity(IdentityFact::Document(
                root.document.clone(),
            ))),
            here,
        ),
    ];
    for state in [
        StateFact::Authentication(root.authentication),
        StateFact::Projection(root.projection),
        StateFact::Detail(root.detail),
        StateFact::Closure(if root.siblings.is_empty() {
            ClosureCompleteness::Complete
        } else {
            ClosureCompleteness::Partial
        }),
    ] {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::State(state)),
            here,
        ));
    }
    for correlation in &root.correlations {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(Payload::Correlation(correlation.clone())),
            here,
        ));
    }
    for payload in shallow_payloads(root) {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            subject.clone(),
            Known::present(payload),
            here,
        ));
    }
    push_unplaceable_address(&mut data, address, &world, here);
    // EVERY plan-shaped family is absent here, and for a different reason than on a plan root: the
    // report model does not cover this species at all. Same vocabulary, honestly caused.
    for family in PlanFamily::ALL {
        data.push(Datum::minted(
            ours(SpeechAct::Derived),
            world.clone(),
            Known::present(Subject::Family(*family)),
            Known::present(Payload::NegativeSpace(NegativeSpace {
                kind: NegativeKind::CarrierGap,
                family: *family,
            })),
            here,
        ));
    }

    Reconstruction::of(
        carriers,
        data,
        Structure::of(root.correlations.clone(), LocusDag::default()),
    )
}

/// A non-plan root's own recorded facts, at the depth its public read-back accessors answer.
///
/// One flat vector rather than a nested projection: the depth here is deliberately shallow
/// (`30Vd:tc-nonplan-root-depth` stands), and a shape that looked like a family projection would
/// suggest per-assignment and per-site rows this read surface does not have.
fn shallow_payloads(root: &NonPlanRoot) -> Vec<Payload> {
    let mut out = Vec::new();
    if let Some(intent) = &root.intent {
        out.push(Payload::Token(RecordedToken::ApplyPolicy(intent.policy)));
        out.push(Payload::Token(RecordedToken::OriginState(
            intent.origin_state,
        )));
        out.push(Payload::Identity(IdentityFact::Count(
            intent.assignment_count,
        )));
        for plan in &intent.origin_receipts {
            out.push(Payload::Identity(IdentityFact::Document(plan.clone())));
        }
    }
    if let Some(outcome) = &root.outcome {
        out.push(Payload::Token(RecordedToken::TerminalState(
            outcome.terminal,
        )));
        out.push(Payload::Identity(IdentityFact::Count(outcome.site_count)));
        if let Some(intent) = &outcome.intent {
            out.push(Payload::Identity(IdentityFact::Document(intent.clone())));
        }
    }
    out
}

/// Whether a species is the one the sealed report model covers.
#[must_use]
pub const fn is_modelled(species: RecordedSpecies) -> bool {
    matches!(species, RecordedSpecies::Plan)
}
