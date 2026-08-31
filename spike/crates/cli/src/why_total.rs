//! The TOTAL surface (`30V` §5): everything one reconstruction holds, exactly once, zero selection.
//!
//! Product-face, not a debug artifact `[TYPED]` — the "I have grep, give me my data" half of the
//! userbase is served better by the tool collating receipts than by them `jq`-ing raw ones. And
//! deliberately TEMPORARY: the settled register is deferred (`30V` §7), so this cut is replaced
//! without ceremony when it arrives.
//!
//! # The register, for this cut
//!
//! Canonical content-derived ordering (the reconstruction's own walk), stable labels, no prosodic
//! header or summary, no drawn gutter, printable ASCII. Readability affordances are STRUCTURE and
//! LABELS only — glossing re-enters the deferred territory `30V` §1 fenced off.
//!
//! # Why the coverage ledger is appended at the emit site
//!
//! A second walk that counted what "should" have been rendered would agree with itself. The ledger
//! is appended where a datum's bytes are actually produced, so a datum an early return drops is
//! caught by a permutation check over the population rather than by a second opinion.
//!
//! # Encoding
//!
//! Every recorded byte run leaves through the caller's [`ValueEncoder`] and then through the weave
//! seat, which encodes again at mint (`sinv-sink-encoding`: a render seat weaves or encodes at
//! mint). Registry words are ours and are never encoded; only values are.

use dorc_aid::RenderCtx;
use dorc_aid::said::Said;
use dorc_aid::tagged::RenderParts;
use dorc_aid::weave::Face;
use dorc_receipt::report::{RecordedValue, ValueClass, ValueEncoder};
use dorc_receipt::rows::RecordedSite;
use dorc_receipt::tokens::ClosedToken;
use dorc_why::known::{CantTell, CarrierAbsence, Held, Known, WithholdReason};
use dorc_why::{
    Carrier, CarrierRole, CorrelationFact, Datum, DatumId, Delivery, IdentityFact, Moment,
    NegativeKind, Payload, Reconstruction, RecordedFlag, RecordedToken, Separability, Speaker,
    StateFact, Subject, Voice, VoiceSet,
};
use weft::{LabeledRow, Node, NodeKind, Section};

/// The terminal's destination encoder for recorded values — THE seat where a recorded byte run
/// becomes characters somebody is shown.
///
/// The match is exhaustive by NAME rather than by a wildcard, and that is the whole of what it buys:
/// today every class answers the same question — foreign bytes bound for a terminal, capped — and a
/// new class landing tomorrow reddens this seat instead of quietly taking a neighbour's encoding
/// (`sinv-sink-encoding`).
#[derive(Debug, Clone, Copy)]
pub struct TerminalValues {
    cap: usize,
}

impl Default for TerminalValues {
    fn default() -> Self {
        Self {
            cap: dorc_aid::said::WHY_VALUE_CAP,
        }
    }
}

impl ValueEncoder for TerminalValues {
    fn encode(&mut self, class: ValueClass, bytes: &[u8]) -> String {
        match class {
            ValueClass::ShellText
            | ValueClass::SourceText
            | ValueClass::SourcePath
            | ValueClass::ArtifactLabel
            | ValueClass::OriginClaim
            | ValueClass::Argv
            | ValueClass::TargetName
            | ValueClass::HostOutput
            | ValueClass::Coordinate
            | ValueClass::EncodedStructure
            | ValueClass::DiagnosticDetail => {
                dorc_aid::display::encode_foreign(&String::from_utf8_lossy(bytes), self.cap)
            }
        }
    }
}

/// Which data reached the render, and which the render left out.
///
/// The exclusion half is empty BY CONSTRUCTION at this cut ([`ExclusionReason`] is uninhabited), so
/// "no curation" is a fact about the type rather than a claim about the code
/// (`30V` §2 rul-types-first-lossy-at-display: the absent-curation tier is literally the empty
/// exclusion set). The claim that carries weight is the permutation over `reached`.
#[derive(Debug, Default)]
pub struct Coverage {
    reached: Vec<DatumId>,
    excluded: Vec<Exclusion>,
}

impl Coverage {
    /// Every datum whose bytes this render produced, in emit order.
    #[must_use]
    pub fn reached(&self) -> &[DatumId] {
        &self.reached
    }

    /// Every datum the render deliberately left out.
    #[must_use]
    pub fn excluded(&self) -> &[Exclusion] {
        &self.excluded
    }

    pub(crate) fn saw(&mut self, datum: DatumId) {
        self.reached.push(datum);
    }
}

/// One datum a curated surface would leave out, and why.
#[derive(Debug)]
pub struct Exclusion {
    /// Which datum.
    pub datum: DatumId,
    /// Why it was left out.
    pub reason: ExclusionReason,
}

/// Why a datum was excluded.
///
/// UNINHABITED: the total surface excludes nothing, and the first arm belongs to whichever curated
/// surface first decides to. An empty ledger that could not be otherwise is the honest shape for a
/// no-decisions render.
#[derive(Debug)]
pub enum ExclusionReason {}

/// Render one reconstruction as the total surface.
///
/// Pure: the encoder is the only thing that can produce bytes from recorded material, and it is the
/// caller's.
#[must_use]
pub fn why_total(
    reconstruction: &Reconstruction,
    ctx: &RenderCtx<'_>,
    encoder: &mut dyn ValueEncoder,
) -> (RenderParts, Coverage) {
    let mut coverage = Coverage::default();
    let mut nodes = Vec::new();

    nodes.push(section(
        ctx,
        "why-total-section-carriers",
        reconstruction
            .carriers()
            .iter()
            .map(|carrier| carrier_row(ctx, carrier))
            .collect(),
    ));

    nodes.push(section(
        ctx,
        "why-total-section-data",
        reconstruction
            .data()
            .iter()
            .enumerate()
            .map(|(index, datum)| {
                coverage.saw(DatumId::of(index));
                datum_row(ctx, datum, encoder)
            })
            .collect(),
    ));

    nodes.push(section(
        ctx,
        "why-total-section-correlations",
        reconstruction
            .structure()
            .receipts()
            .iter()
            .map(|correlation| {
                labeled(
                    ctx,
                    "why-total-label-correlation",
                    &Said::Value(correlation_text(correlation)),
                )
            })
            .collect(),
    ));

    let loci = reconstruction.structure().loci();
    let mut locus_rows: Vec<Node<Face>> = loci
        .nodes()
        .iter()
        .map(|locus| {
            labeled(
                ctx,
                "why-total-label-locus",
                &Said::Parts(vec![
                    Said::Value(locus_text(locus)),
                    Said::Mark("why-total-gap", " ".to_owned()),
                    address_said(&locus.address),
                ]),
            )
        })
        .collect();
    locus_rows.extend(loci.edges().iter().map(|edge| {
        labeled(
            ctx,
            "why-total-label-locus-edge",
            &Said::Value(format!("{}>{}", edge.from, edge.to)),
        )
    }));
    nodes.push(section(ctx, "why-total-section-loci", locus_rows));

    (crate::why_parts(nodes, 0), coverage)
}

/// One titled division, whose header is a registry row and whose body is its rows.
fn section(ctx: &RenderCtx<'_>, slug: &'static str, body: Vec<Node<Face>>) -> Node<Face> {
    Node::new(NodeKind::Section(Section {
        header: Said::words(slug, &[]).runs(ctx, slug),
        counts: None,
        body,
    }))
}

/// One labelled row: a registry label and a computed body.
fn labeled(ctx: &RenderCtx<'_>, slug: &'static str, body: &Said) -> Node<Face> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table: None,
        label: Said::words(slug, &[]).runs(ctx, slug),
        body: body.runs(ctx, slug),
        attachments: Vec::new(),
    }))
}

/// One labelled row with its own hanging rows — the shape one datum and one carrier take.
fn labeled_with(
    ctx: &RenderCtx<'_>,
    slug: &'static str,
    body: &Said,
    attachments: Vec<Node<Face>>,
) -> Node<Face> {
    Node::new(NodeKind::Labeled(LabeledRow {
        table: None,
        label: Said::words(slug, &[]).runs(ctx, slug),
        body: body.runs(ctx, slug),
        attachments,
    }))
}

/// One carrier: its identity, and the three standings `30R:standing-invariants` keeps independent.
fn carrier_row(ctx: &RenderCtx<'_>, carrier: &Carrier) -> Node<Face> {
    labeled_with(
        ctx,
        "why-total-label-document",
        &Said::Value(carrier.document.hex()),
        vec![
            labeled(
                ctx,
                "why-total-label-species",
                &Said::Value(carrier.species.token().to_owned()),
            ),
            labeled(ctx, "why-total-label-role", &role_said(&carrier.role)),
            labeled(
                ctx,
                "why-total-state-authentication",
                &known_said(&carrier.authentication, |state| {
                    Said::Value(format!("{state:?}"))
                }),
            ),
            labeled(
                ctx,
                "why-total-state-projection",
                &known_said(&carrier.projection, |state| {
                    Said::Value(format!("{state:?}"))
                }),
            ),
            labeled(
                ctx,
                "why-total-state-detail",
                &known_said(&carrier.detail, |state| Said::Value(format!("{state:?}"))),
            ),
        ],
    )
}

/// One datum: its subject heads the row, and the other four axes hang under it, each labelled.
fn datum_row(ctx: &RenderCtx<'_>, datum: &Datum, encoder: &mut dyn ValueEncoder) -> Node<Face> {
    let world = datum.world();
    labeled_with(
        ctx,
        "why-total-label-subject",
        &known_said(datum.subject(), |subject| {
            Said::Value(subject_text(subject))
        }),
        vec![
            labeled(
                ctx,
                "why-total-label-speaker",
                &known_said(datum.speaker(), speaker_said),
            ),
            labeled(
                ctx,
                "why-total-label-payload",
                &known_said(datum.payload(), |payload| payload_said(payload, encoder)),
            ),
            labeled(
                ctx,
                "why-total-label-world-moment",
                &known_said(world.moment(), |moment| match moment {
                    Moment::Filed(order) => Said::Value(order.clone()),
                    Moment::Undated => Said::Value("undated".to_owned()),
                }),
            ),
            labeled(
                ctx,
                "why-total-label-world-host",
                &known_said(world.host(), |host| {
                    Said::Value(host.value().render(encoder))
                }),
            ),
            labeled(
                ctx,
                "why-total-label-world-lineage",
                &known_said(world.lineage(), |lineage| {
                    let dorc_why::AttemptLineage::Document(document) = lineage;
                    Said::Value(document.hex())
                }),
            ),
            labeled(
                ctx,
                "why-total-label-delivery",
                &Said::Value(match datum.delivery() {
                    Delivery::Recorded(reference) => reference.get().to_string(),
                    Delivery::Live => "live".to_owned(),
                }),
            ),
        ],
    )
}

/// One wrapped slot: the value, or the registry word for the absence it came back as.
fn known_said<T>(known: &Known<T>, present: impl FnOnce(&T) -> Said) -> Said {
    match known.value() {
        Some(value) => present(value),
        None => Said::words(absence_slug(known), &[]),
    }
}

/// Which absence a wrapped slot came back as, as the registry row naming it.
///
/// Total over BOTH wrappers and no-wildcard, so every state `30V` §3 separates has its own word
/// here. Collapsing any pair would let a bound that fired read as a run that held no value. Its
/// machine twin is [`absence_word`], and both being no-wildcard is what stops them drifting.
fn absence_slug<T>(known: &Known<T>) -> &'static str {
    match known {
        Known::Knowable(Held::Present(_)) => "why-total-present",
        Known::Knowable(Held::AbsentFromCarrier(absence)) => match absence {
            CarrierAbsence::RunHeldNoValue => "why-total-absent-run-held-no-value",
            CarrierAbsence::ProjectionUncollected => "why-total-absent-projection-uncollected",
            CarrierAbsence::ReportApiLacks => "why-total-absent-report-api-lacks",
        },
        Known::Knowable(Held::Withheld(reason)) => match reason {
            WithholdReason::PlainProjection => "why-total-withheld-plain",
            WithholdReason::BoundRefused => "why-total-withheld-bound",
            WithholdReason::RegionUnavailable => "why-total-withheld-region",
            WithholdReason::EncoderGated => "why-total-withheld-encoder",
        },
        Known::Knowable(Held::CouldNotTell(cause)) => match cause {
            CantTell::ComparisonNotMade => "why-total-cant-tell-no-comparison",
            CantTell::Truncated => "why-total-cant-tell-truncated",
        },
        Known::KnowableNYI => "why-total-not-yet-piped",
        Known::Unknowable => "why-total-unknowable",
    }
}

/// The same answer as one MACHINE word, or `None` where the slot holds a value.
///
/// Its own no-wildcard match rather than a transformation of [`absence_slug`]: the machine surface's
/// vocabulary is hardcoded and out of the registry (`30V` §5), so deriving one from the other would
/// couple a wire word to a render slug's spelling.
pub(crate) const fn absence_word<T>(known: &Known<T>) -> Option<&'static str> {
    Some(match known {
        Known::Knowable(Held::Present(_)) => return None,
        Known::Knowable(Held::AbsentFromCarrier(absence)) => match absence {
            CarrierAbsence::RunHeldNoValue => "absent-run-held-no-value",
            CarrierAbsence::ProjectionUncollected => "absent-projection-uncollected",
            CarrierAbsence::ReportApiLacks => "absent-report-api-lacks",
        },
        Known::Knowable(Held::Withheld(reason)) => match reason {
            WithholdReason::PlainProjection => "withheld-plain",
            WithholdReason::BoundRefused => "withheld-bound",
            WithholdReason::RegionUnavailable => "withheld-region",
            WithholdReason::EncoderGated => "withheld-encoder",
        },
        Known::Knowable(Held::CouldNotTell(cause)) => match cause {
            CantTell::ComparisonNotMade => "cant-tell-no-comparison",
            CantTell::Truncated => "cant-tell-truncated",
        },
        Known::KnowableNYI => "not-yet-piped",
        Known::Unknowable => "unknowable",
    })
}

/// Who spoke, in what act. The verb comes from the ONE seat that renders a [`SpeechAct`]
/// (`AID-NEEDS:law-trust-tier-is-syntax`); a second mapping here would be a second vocabulary.
fn speaker_said(speaker: &Speaker) -> Said {
    Said::Parts(vec![
        crate::why::verb_said(speaker.act()),
        Said::Mark("why-total-gap", " ".to_owned()),
        known_said(speaker.voices(), voice_said),
    ])
}

/// Which voices performed the act.
fn voice_said(voices: &VoiceSet) -> Said {
    match voices {
        VoiceSet::Mine => Said::words("why-total-voice-mine", &[]),
        VoiceSet::One(Voice::AuthoredIn(source)) => Said::Parts(vec![
            Said::words("why-total-voice-authored-in", &[]),
            Said::Mark("why-total-gap", " ".to_owned()),
            Said::Value(source.get().to_string()),
        ]),
        VoiceSet::Committee {
            voices,
            separability,
        } => Said::Parts(vec![
            Said::words("why-total-voice-committee", &[]),
            Said::Mark("why-total-gap", " ".to_owned()),
            Said::Value(format!(
                "{} {}",
                voices.len(),
                match separability {
                    Separability::Separable => "separable",
                    Separability::Inseparable => "inseparable",
                }
            )),
        ]),
    }
}

/// Why a document is in the closure.
fn role_said(role: &CarrierRole) -> Said {
    match role {
        CarrierRole::Root => Said::words("why-total-role-root", &[]),
        CarrierRole::Reached => Said::words("why-total-role-reached", &[]),
        CarrierRole::Sibling(state) => Said::Parts(vec![
            Said::words("why-total-role-sibling", &[]),
            Said::Mark("why-total-gap", " ".to_owned()),
            Said::Value(format!("{state:?}")),
        ]),
    }
}

/// What a datum is about, in the identities the document itself numbers.
pub(crate) fn subject_text(subject: &Subject) -> String {
    match subject {
        Subject::Site(site) => format!("site {}", site_text(*site)),
        Subject::Source(source) => format!("source {}", source.get()),
        Subject::Stage { site, index } => format!("stage {} {index}", site_text(*site)),
        Subject::Document(document) => format!("document {}", document.hex()),
        Subject::Address(address) => format!("address {} {}", address.source.get(), address.line),
        Subject::Narrative(ordinal) => format!("narrative {ordinal}"),
        Subject::Region(ordinal) => format!("region {ordinal}"),
        Subject::Load(ordinal) => format!("load {ordinal}"),
        Subject::Family(family) => format!("family {}", family.token()),
    }
}

/// A site, with the in-loop member index where it has one — two same-command sites never collapse
/// (`inv-site-keyed-results`).
pub(crate) fn site_text(site: RecordedSite) -> String {
    match site.member() {
        Some(member) => format!("{}.{}", site.leaf().get(), member.get()),
        None => site.leaf().get().to_string(),
    }
}

/// What was said. The one arm that can carry bytes hands them to the caller's encoder.
fn payload_said(payload: &Payload, encoder: &mut dyn ValueEncoder) -> Said {
    match payload {
        Payload::Decision(disposition) => Said::Value(disposition.token().to_owned()),
        Payload::Influence(grade) => Said::Value(grade.token().to_owned()),
        Payload::Identity(identity) => Said::Value(identity_text(identity)),
        Payload::State(state) => Said::Value(state_text(*state)),
        Payload::Text(value) => Said::Value(text_of(value, encoder)),
        Payload::Correlation(correlation) => Said::Value(correlation_text(correlation)),
        Payload::Collapse(kind) => Said::Value(kind.token().to_owned()),
        Payload::Token(token) => Said::Value(token_text(*token)),
        Payload::Flag(flag) => Said::Value(flag_text(*flag)),
        Payload::NegativeSpace(space) => Said::Parts(vec![
            Said::words(
                match space.kind {
                    NegativeKind::ReportApiGap => "why-total-absent-report-api-lacks",
                    NegativeKind::CarrierGap => "why-total-absent-run-held-no-value",
                },
                &[],
            ),
            Said::Mark("why-total-gap", " ".to_owned()),
            Said::Value(space.family.token().to_owned()),
        ]),
    }
}

/// THE exit for recorded bytes: the caller's destination encoder, and nothing else.
fn text_of(value: &RecordedValue, encoder: &mut dyn ValueEncoder) -> String {
    value.render(encoder)
}

pub(crate) fn identity_text(identity: &IdentityFact) -> String {
    match identity {
        IdentityFact::Document(document) => document.hex(),
        IdentityFact::Species(species) => species.token().to_owned(),
        IdentityFact::Digest(digest) => digest.clone(),
        IdentityFact::Bytes(bytes) => bytes.to_string(),
        IdentityFact::Count(count) => count.to_string(),
        IdentityFact::Operands(operands) => {
            format!("{}+{}", operands.shown(), operands.dropped())
        }
        IdentityFact::UncarriedSpecies(species) => species.token().to_owned(),
        IdentityFact::SourceClass(class) => class.token().to_owned(),
        IdentityFact::Ast(ast) => ast.to_string(),
        IdentityFact::InvocationMode(mode) => mode.token().to_owned(),
    }
}

/// One of the report's own closed states.
///
/// Spelled through `Debug` rather than a minted word: these are report-plane states with no wire
/// token, and a token minted here would be user-facing prose a builder authored
/// (`error-authorship-tier`). The interim spelling is machine-shaped and greppable, and the settled
/// register replaces it (`30V` §7).
pub(crate) fn state_text(state: StateFact) -> String {
    match state {
        StateFact::Authentication(value) => format!("{value:?}"),
        StateFact::Projection(value) => format!("{value:?}"),
        StateFact::Detail(value) => format!("{value:?}"),
        StateFact::Closure(value) => format!("{value:?}"),
        StateFact::CurrentSource(value) => format!("{value:?}"),
        StateFact::ReDerivation(value) => format!("{value:?}"),
    }
}

pub(crate) fn correlation_text(correlation: &CorrelationFact) -> String {
    match correlation {
        CorrelationFact::PlanToIntent { plan, intent } => {
            format!("{} {}", plan.hex(), intent.hex())
        }
        CorrelationFact::IntentToOutcome { intent, outcome } => {
            format!("{} {}", intent.hex(), outcome.hex())
        }
        CorrelationFact::Finding(kind) => format!("{kind:?}"),
    }
}

/// One word of a recorded closed vocabulary, in the document's own spelling.
pub(crate) fn token_text(token: RecordedToken) -> String {
    match token {
        RecordedToken::AdmissionOutcome(value) => value.token().to_owned(),
        RecordedToken::LoadOutcome(value) => value.token().to_owned(),
        RecordedToken::SiteClass(value) => value.token().to_owned(),
        RecordedToken::SolvePass(value) => value.token().to_owned(),
        RecordedToken::ShipLane(value) => value.token().to_owned(),
        RecordedToken::SurvivalOutcome(value) => value.token().to_owned(),
        RecordedToken::RenderKind(value) => value.token().to_owned(),
        RecordedToken::LicenseVerb(value) => value.token().to_owned(),
        RecordedToken::LicenseCustody(value) => value.token().to_owned(),
    }
}

/// One named predicate and its answer, keyed by the grammar's own field name.
pub(crate) fn flag_text(flag: RecordedFlag) -> String {
    let (name, value) = match flag {
        RecordedFlag::VerdictLane(value) => ("verdict-lane", value),
        RecordedFlag::Invalidator(value) => ("invalidator", value),
        RecordedFlag::SolveConsistent(value) => ("consistent", value),
        RecordedFlag::SolveTripped(value) => ("tripped", value),
    };
    format!("{name} {value}")
}

/// One locus of the provenance DAG.
pub(crate) fn locus_text(locus: &dorc_why::Locus) -> String {
    format!(
        "{} {} {:?} {:?} {:?}",
        site_text(locus.site),
        locus.index,
        locus.stage,
        locus.namespace,
        locus.agreement
    )
}

/// A locus address, or the absence the carrier left in its place.
///
/// Ordinal-and-span rather than `file.sh:N` (`30Vd:fnd-addresses-cannot-be-spelled-file-line`):
/// neither half of the path-and-line form is derivable from this read surface, and guessing one
/// would be the mis-attribution `271:rul-sin-ordering` ranks worst.
fn address_said(address: &Known<dorc_why::LocusAddress>) -> Said {
    known_said(address, |value| {
        Said::Value(format!(
            "{} {}..{}",
            value.source.get(),
            value.span.0,
            value.span.1
        ))
    })
}
