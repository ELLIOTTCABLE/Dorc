//! `RecordedWhyFacts`: what a receipt-rooted question can establish, and what it refuses to guess.
//!
//! Driven through a REAL published document — projected, sealed, signed, read back — rather than a
//! hand-built model, because the thing most worth pinning is that record ordinals, detail keys and
//! locator payloads still line up after a round trip. A hand-built model agrees with itself.

use dorc_receipt::durable_locator::RecordedStageKind;
use dorc_receipt::graph::{ReachedClosure, ReceiptGraph};
use dorc_receipt::ids::{PlanReceiptId, ReceiptId, ReceiptIdSource};
use dorc_receipt::order::ReceiptOrderToken;
use dorc_receipt::report::{
    AddressResolution, AuthenticationState, ClosureCompleteness, CurrentSourceReading,
    CurrentSourceState, DetailState, MaterialState, PlanFamily, ReDerivationState,
    RecordedDocumentId, RecordedSpecies, RequestedAddress, SiblingState, SourceObservation,
    UnresolvedReason, ValueClass, ValueEncoder, WhyFactsInput, derive,
};
use dorc_receipt::rows::{RecordedLeaf, RecordedSite};

mod support;

use support::{DocumentUnderTest, book, published};

/// An encoder that reveals what it was given, so a case can assert the ONE exit works.
///
/// Production supplies the real destination encoders from `aid`; this one is deliberately naive,
/// because what these cases pin is that the exit is encoder-MEDIATED, not what any encoder does.
struct Spy {
    seen: Vec<ValueClass>,
}

impl ValueEncoder for Spy {
    fn encode(&mut self, class: ValueClass, bytes: &[u8]) -> String {
        self.seen.push(class);
        String::from_utf8_lossy(bytes).into_owned()
    }
}

impl Spy {
    fn new() -> Self {
        Self { seen: Vec::new() }
    }
}

/// The address every exact-line case asks about: the book's second physical line.
const ADDRESSED_LINE: u32 = 2;

/// The site the fixture document records — leaf 0, no in-loop member.
fn recorded_site() -> RecordedSite {
    RecordedSite::of(RecordedLeaf::of(0), None)
}

/// The closure a plan-rooted question gets, from the graph rather than from this test.
///
/// The graph is empty on purpose: a plan root reaches nothing further whatever the store holds
/// (`30R:receipt-rooted-attention-and-cli`), and a helper that hand-built the membership would be
/// exercising a route the API no longer has.
fn closure(document: &DocumentUnderTest) -> ReachedClosure {
    ReceiptGraph::new().closure_from(&RecordedDocumentId::Plan(document.id))
}

fn input(
    document: &DocumentUnderTest,
    observations: Vec<SourceObservation>,
    address: Option<RequestedAddress>,
) -> WhyFactsInput<'_> {
    WhyFactsInput {
        root: &document.receipt,
        model: &document.model,
        order: document.order,
        authentication: AuthenticationState::Trusted,
        detail: DetailState::Available,
        reached: closure(document),
        siblings: Vec::new(),
        observations,
        address,
    }
}

fn matching(book: &str) -> Vec<SourceObservation> {
    vec![SourceObservation {
        ordinal: 0,
        reading: CurrentSourceReading::Read(book.as_bytes().to_vec()),
        matches_digest: true,
    }]
}

/// THE EXACT-LINE CASE: current and recorded line N are byte-identical, so the recorded site is
/// the answer to the address.
#[test]
fn a_byte_identical_line_resolves_to_the_site_recorded_there() {
    let document = published();
    let facts = derive(&input(
        &document,
        matching(book()),
        Some(RequestedAddress::of(0, ADDRESSED_LINE)),
    ));

    let address = facts.address().expect("the question named an address");
    assert_eq!(address.current(), CurrentSourceState::Matching);
    let AddressResolution::Resolved { site } = *address.resolution() else {
        panic!("an identical line resolves; got {:?}", address.resolution());
    };
    assert_eq!(site, recorded_site());
    assert!(
        facts.addressed_site().is_some(),
        "and the model can hand back the site it resolved to"
    );
}

/// A CHANGED line at the same coordinate refuses the address-specific answer, keeps the recorded
/// site as a statement about the past, and never searches for the line elsewhere.
#[test]
fn a_changed_line_refuses_the_address_and_never_looks_for_it_elsewhere() {
    let document = published();
    // The same command, MOVED: a third line inserted above it. A resolver willing to match on
    // content would find it one line down and answer confidently about the wrong coordinate.
    let moved = format!("#!/bin/sh\necho inserted\n{}", &book()[10..]);
    let facts = derive(&input(
        &document,
        vec![SourceObservation {
            ordinal: 0,
            reading: CurrentSourceReading::Read(moved.into_bytes()),
            matches_digest: false,
        }],
        Some(RequestedAddress::of(0, ADDRESSED_LINE)),
    ));

    let address = facts.address().expect("the question named an address");
    assert_eq!(address.current(), CurrentSourceState::Drifted);
    let AddressResolution::ChangedLine { recorded_site } = *address.resolution() else {
        panic!(
            "a changed line is an ambiguity; got {:?}",
            address.resolution()
        );
    };
    assert_eq!(
        recorded_site,
        Some(self::recorded_site()),
        "what the RECORDED line carried is still true about the past"
    );
    assert_eq!(
        address.resolved_site(),
        None,
        "but it does not answer the address"
    );
    // NON-VACUITY: every unrelated receipt fact still renders. One unanswerable address is not a
    // reason to stop explaining the rest.
    assert!(!facts.sites().is_empty() && !facts.sources().is_empty());
}

/// Missing current source leaves a recorded-only answer, qualified by the comparison that did not
/// happen — never presented as agreement.
#[test]
fn an_absent_current_source_yields_a_qualified_recorded_only_answer() {
    let document = published();
    let facts = derive(&input(
        &document,
        vec![SourceObservation {
            ordinal: 0,
            reading: CurrentSourceReading::Absent,
            matches_digest: false,
        }],
        Some(RequestedAddress::of(0, ADDRESSED_LINE)),
    ));

    let address = facts.address().expect("the question named an address");
    let AddressResolution::ComparisonUnavailable { recorded_site, why } = *address.resolution()
    else {
        panic!(
            "an absent source cannot be compared; got {:?}",
            address.resolution()
        );
    };
    assert_eq!(recorded_site, Some(self::recorded_site()));
    assert_eq!(why, CurrentSourceState::Absent);
    assert_eq!(
        address.resolved_site(),
        None,
        "an uncompared address is not a resolved one"
    );
}

/// A document whose region did not open yields PARTIAL facts, and says which material is missing
/// rather than reporting an empty chain as a short one.
#[test]
fn an_unavailable_detail_region_yields_explicitly_partial_material() {
    let document = published();
    let mut request = input(
        &document,
        matching(book()),
        Some(RequestedAddress::of(0, 2)),
    );
    request.detail = DetailState::Unavailable;
    let facts = derive(&request);

    assert_eq!(facts.root().detail(), DetailState::Unavailable);
    let site = &facts.sites()[0];
    assert_eq!(
        site.locator(),
        MaterialState::Undecodable,
        "a slot the skeleton says is captured, in a region that did not open, is undecodable"
    );
    assert!(site.chain().is_empty());
    assert_eq!(facts.sources()[0].content(), MaterialState::Undecodable);
    assert_eq!(
        *facts.address().expect("an address").resolution(),
        AddressResolution::Unresolved(UnresolvedReason::SourceContentUnavailable),
        "with no recorded bytes there is no line to compare"
    );
}

/// Closure membership is the GRAPH's answer, and its completeness is derived from the siblings the
/// question could not reach.
///
/// The shape this refuses is the one it replaced: a caller handed `reached` a hand-built vector, so
/// a plan-rooted question could be told it reached a later intent and outcome — the exact
/// pull-every-later-attempt reading `30R:receipt-rooted-attention-and-cli` forbids, and unfalsifiable
/// besides, since nothing checked those documents existed.
#[test]
fn closure_membership_comes_from_the_graph_and_a_plan_reaches_nothing_later() {
    let document = published();
    let intent = dorc_receipt::ids::ApplyIntentId::mint(&mut Counting(9));

    let facts = derive(&input(&document, Vec::new(), None));
    assert_eq!(
        facts.closure().completeness(),
        ClosureCompleteness::Complete
    );
    assert_eq!(
        facts.closure().reached().len(),
        1,
        "a plan root reaches itself and no later apply attempt"
    );
    assert_eq!(
        facts.closure().reached()[0].species(),
        RecordedSpecies::Plan
    );
    assert_eq!(
        facts.root().document(),
        &facts.closure().reached()[0],
        "the root is named once — the closure's head IS the document the facts are about"
    );

    let mut broken = input(&document, Vec::new(), None);
    broken.siblings = vec![SiblingState::Missing(RecordedDocumentId::ApplyIntent(
        intent,
    ))];
    let facts = derive(&broken);
    assert_eq!(
        facts.closure().completeness(),
        ClosureCompleteness::Partial,
        "a named sibling that is not in hand makes the closure partial"
    );
    assert_eq!(
        facts.closure().siblings()[0].document().species(),
        RecordedSpecies::ApplyIntent
    );
}

/// Authentication, completeness and influence are three independent answers.
///
/// The failure this refuses is a model that rounded them together — reporting an unauthenticated
/// document as incomplete, or a partial closure as unauthenticated. `30Ra` keeps them separate
/// because a reader must be able to hold one without inferring the others.
#[test]
fn authentication_completeness_and_influence_stay_independently_typed() {
    let document = published();
    let mut request = input(&document, Vec::new(), None);
    request.authentication = AuthenticationState::Failed;
    let facts = derive(&request);

    assert_eq!(facts.root().authentication(), AuthenticationState::Failed);
    assert!(!facts.root().authentication().is_authenticated());
    assert_eq!(
        facts.closure().completeness(),
        ClosureCompleteness::Complete,
        "a failed signature says nothing about whether the closure assembled"
    );
    assert!(
        !facts.sites().is_empty(),
        "and bounded recoverable structure still reaches the report"
    );
    // The influence grade is its own answer and travels with the site, never rehydrated.
    let _grade = facts.sites()[0].influence();
}

/// Re-derivation is explicitly pending, never an absence and never a fabricated disposition.
#[test]
fn re_derivation_is_explicitly_pending_rather_than_silent() {
    let document = published();
    let facts = derive(&input(&document, Vec::new(), None));
    assert_eq!(
        facts.rederivation(),
        ReDerivationState::PendingKernelSupport,
        "a reader must be able to tell nobody-checked from checked-and-agreed"
    );
}

/// Bytes leave ONLY through an encoder, and each arrives under its own class.
#[test]
fn recorded_bytes_exit_only_through_a_caller_supplied_encoder() {
    let document = published();
    let facts = derive(&input(&document, matching(book()), None));

    let mut spy = Spy::new();
    let source = facts.sources()[0]
        .text()
        .expect("the general-sh source carries its bytes");
    let rendered = source.render(&mut spy);

    assert_eq!(rendered, book(), "the encoder saw the exact acquired bytes");
    assert_eq!(spy.seen, vec![ValueClass::SourceText]);
    // The class is available WITHOUT rendering, so a caller can route to the right encoder.
    assert_eq!(source.class(), ValueClass::SourceText);
    assert_eq!(source.len(), book().len());
    // And the Debug says how much, never what.
    let shown = format!("{source:?}");
    assert!(
        shown.contains("SourceText") && !shown.contains("hork"),
        "Debug must not reveal content; got {shown}"
    );
}

/// THE SENTINEL: the encoder is the ONLY thing that emits recorded bytes.
///
/// A sealed value with no accessor is only half the property. The other half is that nothing
/// ELSE renders one by accident — a derived `Debug` on the document, on the model, on the input
/// struct, or on the current-source reading would put a book's own shell text into a panic
/// message, a log line or a test failure, none of which is a destination encoder
/// (`sinv-sink-encoding`). Every surface below is asked directly, and the positive control at the
/// end is what says the bytes were there to leak.
#[test]
fn no_debug_or_report_surface_carries_the_recorded_bytes() {
    // A run of the fixture book that appears nowhere in a type name, a token, or a state word,
    // so a hit is the CONTENT and never the frame around it.
    const SENTINEL: &str = "hork tune --profile web";
    assert!(
        book().contains(SENTINEL),
        "the sentinel must really be in the bytes under test"
    );

    let document = published();
    let observations = matching(book());
    let input = input(&document, observations.clone(), None);
    let facts = derive(&input);

    let surfaces = [
        ("the read-back document", format!("{:?}", document.receipt)),
        ("its sealed model", format!("{:?}", document.model)),
        ("the model input", format!("{input:?}")),
        ("the derived facts", format!("{facts:?}")),
        (
            "a current-source reading",
            format!("{:?}", observations[0].reading),
        ),
    ];
    for (what, shown) in &surfaces {
        assert!(
            !shown.contains(SENTINEL),
            "{what} rendered the recorded bytes: {shown}"
        );
        assert!(
            !shown.is_empty(),
            "{what} rendered nothing at all, so this proves nothing"
        );
    }

    // The positive control. The same bytes DO come out, through the one exit, so the silence
    // above is a seal rather than a document that turned out to be empty.
    let mut spy = Spy::new();
    let rendered = facts.sources()[0]
        .text()
        .expect("the general-sh source carries its bytes")
        .render(&mut spy);
    assert!(
        rendered.contains(SENTINEL),
        "the encoder is the exit, and it must still work"
    );
}

/// A locator round-trips through the document into the model's chain.
#[test]
fn a_site_carries_the_provenance_chain_its_document_recorded() {
    let document = published();
    let facts = derive(&input(&document, matching(book()), None));
    let site = &facts.sites()[0];

    assert_eq!(site.locator(), MaterialState::Held);
    let authored = site
        .authored_origin()
        .expect("the site records where its bytes were authored");
    assert_eq!(authored.kind(), RecordedStageKind::Authored);
    assert_eq!(authored.source(), Some(0));
    assert!(authored.span().is_some());
}

/// A deterministic identity source, for the sibling identities these cases name.
struct Counting(u8);

impl ReceiptIdSource for Counting {
    fn next_receipt_id(&mut self) -> ReceiptId {
        self.0 = self.0.wrapping_add(1);
        ReceiptId::of_source_bytes([self.0; 32])
    }
}

/// The public surface yields no live authority, and no raw bytes.
///
/// A LEXICAL check over this crate's own report module, on the same footing as the crate-boundary
/// fences beside it: the property is "the module cannot even spell it", which no type bound states.
#[test]
fn the_report_surface_names_no_live_authority_and_hands_out_no_raw_bytes() {
    let sources = [
        include_str!("../src/report.rs"),
        include_str!("../src/report/build.rs"),
        include_str!("../src/report/states.rs"),
        include_str!("../src/report/address.rs"),
        include_str!("../src/report/value.rs"),
    ];
    for source in sources {
        let code: String = source
            .lines()
            .filter(|line| !line.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in [
            "PlanAuthority",
            "ReplaceLicense",
            "GuardLicense",
            "VerdictVouch",
            "ByVouch",
            "dorc_plan",
            "dorc_aid",
            "dorc_cli",
            "std::fs",
            "std::net",
        ] {
            assert!(
                !code.contains(forbidden),
                "a report that can name `{forbidden}` is a report that can decide or read"
            );
        }
    }
    // And the one byte-bearing type offers no bare exit. Spelled as an absence of method names,
    // because that is exactly what the seal is.
    let value = include_str!("../src/report/value.rs");
    for escape in ["pub fn as_bytes", "pub fn as_str", "pub fn into_inner"] {
        assert!(
            !value.contains(escape),
            "`{escape}` would be an exit that never consults an encoder"
        );
    }
}

/// The crate's DEPENDENCY table names nothing that renders, decides, or reads a filesystem.
///
/// Scoped to the `[dependencies]` section rather than the whole file, because the manifest's own
/// prose explains which crates this one deliberately sits below — a whole-file grep fails on the
/// comment that documents the rule it is checking.
#[test]
fn the_receipt_crate_depends_on_nothing_that_renders_or_reads() {
    let manifest = include_str!("../Cargo.toml");
    let table: String = manifest
        .lines()
        .skip_while(|line| line.trim() != "[dependencies]")
        .skip(1)
        .take_while(|line| !line.trim_start().starts_with('['))
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !table.trim().is_empty(),
        "the section walk found no dependency table, so it proves nothing"
    );
    for forbidden in [
        "dorc-aid",
        "dorc-plan",
        "dorc-cli",
        "dorc-receipt-local",
        "dorc-receipt-crypto",
    ] {
        assert!(
            !table.contains(forbidden),
            "the receipt crate must not depend on `{forbidden}`; table:\n{table}"
        );
    }
}

/// Unused in the cases above but constructed here so the fixture's own shape stays honest: a
/// locator the document could not parse must not read as a held one.
#[test]
fn a_locator_payload_that_does_not_parse_reads_undecodable() {
    let mut ids = Counting(3);
    let id = PlanReceiptId::mint(&mut ids);
    let order = ReceiptOrderToken::of_controller_millis(1_700_000_000_000);
    let document = support::publish_with_locator(id, order, b"not a locator".to_vec());
    let facts = derive(&input(&document, Vec::new(), None));

    assert_eq!(
        facts.sites()[0].locator(),
        MaterialState::Undecodable,
        "an unparseable payload is not a site whose provenance was one stage long"
    );
    assert!(facts.sites()[0].chain().is_empty());
}

/// EVERY persisted family answers with typed facts, and the two singletons the fixture carries
/// prove the projection actually decomposed something.
///
/// The exhaustiveness half of `inv-report-projection-exhaustive-or-classified`. A count would drift;
/// what must hold is that no family answers `RecordedButUnprojected`, because that word now means a
/// projection somebody removed rather than one nobody has written.
#[test]
fn every_persisted_family_answers_with_typed_facts() {
    let document = published();
    let facts = derive(&input(&document, Vec::new(), None));

    let coverage = facts.coverage();
    assert_eq!(coverage.len(), PlanFamily::ALL.len());
    for (family, cover) in &coverage {
        assert!(
            cover.is_projected(),
            "family {} answers {cover:?}; the fixture carries one row of every family, so anything \
             but a projection is a decomposition that stopped working",
            family.token()
        );
    }

    assert_eq!(
        facts
            .admission()
            .expect("the fixture publishes an admission row")
            .records(),
        2
    );
    assert_eq!(facts.regions()[0].routes(), 3);
    assert_eq!(facts.loads()[0].ordinal(), 0);
    assert!(facts.classifications()[0].verdict_lane());
    assert!(facts.certifications()[0].consistent());
    assert_eq!(facts.survivals()[0].aggregate(), Some(2));
    assert_eq!(facts.licensors()[0].site(), recorded_site());
    assert!(
        facts
            .presented()
            .expect("the fixture publishes a presented-plan row")
            .planned_image()
            .is_some()
    );
}

/// Every family's OPAQUE slot leaves through the encoder, and never any other way.
///
/// One case over the whole widened surface rather than one per family: what `sinv-sink-encoding`
/// binds is that a recorded byte run reaches a destination only by being handed to an encoder, and a
/// family whose text slot bypassed that would show up here as a class the spy never saw.
#[test]
fn every_widened_family_releases_its_bytes_only_through_the_encoder() {
    let document = published();
    let facts = derive(&input(&document, Vec::new(), None));
    let mut spy = Spy::new();

    let rendered: Vec<String> = [
        facts.admission().and_then(|row| row.stream_text()),
        facts.regions().first().and_then(|row| row.shell_text()),
        facts.loads().first().and_then(|row| row.name_text()),
        facts.loads().first().and_then(|row| row.custody_text()),
        facts.ships().first().and_then(|row| row.source_text()),
        facts.survivals().first().and_then(|row| row.poison_text()),
        facts.renders().first().and_then(|row| row.detail_text()),
        facts.licensors().first().and_then(|row| row.locus_text()),
    ]
    .into_iter()
    .map(|value| {
        value
            .expect("the fixture captures every widened slot")
            .render(&mut spy)
    })
    .collect();

    assert_eq!(
        spy.seen.len(),
        rendered.len(),
        "every value that produced bytes went through the encoder exactly once"
    );
    assert!(
        spy.seen.contains(&ValueClass::EncodedStructure)
            && spy.seen.contains(&ValueClass::SourcePath)
            && spy.seen.contains(&ValueClass::Coordinate)
            && spy.seen.contains(&ValueClass::DiagnosticDetail),
        "the widened slots pose several DIFFERENT sink questions, and each arrives under its own \
         class: {:?}",
        spy.seen
    );
}
