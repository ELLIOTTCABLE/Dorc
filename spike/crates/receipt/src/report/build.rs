//! Assembling [`RecordedWhyFacts`](super::RecordedWhyFacts) from already-decoded receipt material.
//!
//! Everything here is a pure function of what it was handed. The edge has already resolved roots,
//! opened the keyset, walked the store, verified signatures and read whatever current source it
//! chose to; this seat receives the OUTCOMES as data and decomposes them. That split is what keeps
//! the crate free of filesystem, provider and key implementations while still being the home of
//! the decomposition.

use super::address::{self, AuthoredPlacement};
use super::families::{
    AdmissionFacts, CertificationFacts, ClassificationFacts, InvocationFacts, LicensorFacts,
    LoadFacts, NarrativeFacts, PresentedPlanFacts, RegionFacts, RenderFacts, ShipFacts,
    SurvivalFacts,
};
use super::states::{
    AuthenticationState, CurrentSourceState, DetailState, MaterialState, ProjectionState,
    SiblingState,
};
use super::value::{RecordedValue, ValueClass};
use super::{
    ClosureFacts, OmissionFacts, ReDerivationState, RecordedWhyFacts, RequestedAddress, RootFacts,
    SiteFacts, SourceFacts, StageFacts,
};
use crate::durable_locator::{DurableLocator, RecordedStageKind, StageTextKind};
use crate::graph::ReachedClosure;
use crate::model::{PlanReceipt, Rich};
use crate::order::ReceiptOrderToken;
use crate::plan::RecordedPlanReceipt;
use crate::projection::OpaqueFieldTag;
use crate::reader::Receipt;
use crate::reingested::Reingested;

/// What the edge saw when it went looking for one acquired source in the current tree.
///
/// Mirrors `source::CurrentSource` in shape, owned rather than borrowed, because the model outlives
/// the read: the edge opens a file, hands over what it found, and closes it.
#[derive(Clone)]
pub enum CurrentSourceReading {
    /// The edge read something at the recorded path; these are its exact bytes.
    Read(Vec<u8>),
    /// Nothing is at the recorded path.
    Absent,
    /// Something is there and the edge could not read it.
    Unreadable,
    /// No path was recorded, or none was looked for.
    NotLookedFor,
}

/// Names the arm and, for the one that holds bytes, how many — never the bytes.
///
/// Hand-written for [`super::RecordedValue`]'s reason: these are a whole source file off somebody's
/// filesystem, and a derived `Debug` would put them into a panic message, a log line, or a test
/// failure, none of which is a destination encoder.
impl core::fmt::Debug for CurrentSourceReading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read(bytes) => write!(f, "Read({} bytes)", bytes.len()),
            Self::Absent => f.write_str("Absent"),
            Self::Unreadable => f.write_str("Unreadable"),
            Self::NotLookedFor => f.write_str("NotLookedFor"),
        }
    }
}

impl CurrentSourceReading {
    /// How this reading stands against a recorded digest.
    ///
    /// The digest comparison is the caller's — it owns the hash — so this takes the ANSWER rather
    /// than the function, which keeps the crate free of a digest implementation on this path.
    const fn standing(&self, matches_digest: bool) -> CurrentSourceState {
        match self {
            Self::Read(_) if matches_digest => CurrentSourceState::Matching,
            Self::Read(_) => CurrentSourceState::Drifted,
            Self::Absent => CurrentSourceState::Absent,
            Self::Unreadable => CurrentSourceState::Unreadable,
            Self::NotLookedFor => CurrentSourceState::NotCompared,
        }
    }

    const fn bytes(&self) -> Option<&Vec<u8>> {
        match self {
            Self::Read(bytes) => Some(bytes),
            Self::Absent | Self::Unreadable | Self::NotLookedFor => None,
        }
    }
}

/// One acquired source's current-tree observation, as the edge supplies it.
#[derive(Debug, Clone)]
pub struct SourceObservation {
    /// Which acquired source, by ordinal.
    pub ordinal: u32,
    /// What the edge found.
    pub reading: CurrentSourceReading,
    /// Whether what it found still hashes to the recorded digest.
    pub matches_digest: bool,
}

/// Everything the edge hands the model.
///
/// A struct rather than a long argument list, because five of these are `Vec`s and two are
/// `Option`s: an argument order somebody has to remember is an argument order somebody transposes.
///
/// `Debug` is redacted rather than derived: this value holds the decoded document AND the current
/// source readings, so a derived one would print both halves of everything the model exists to
/// release only through an encoder.
pub struct WhyFactsInput<'a> {
    /// The selected root document, decoded and sealed.
    pub root: &'a Reingested<Receipt<PlanReceipt, Rich>>,
    /// Its own model, closed over itself.
    pub model: &'a Reingested<RecordedPlanReceipt>,
    /// The order token it was filed under.
    pub order: ReceiptOrderToken,
    /// What outer verification said.
    pub authentication: AuthenticationState,
    /// Whether its grouped detail region opened.
    pub detail: DetailState,
    /// The rooted question's causal closure, minted by the graph's own walk.
    ///
    /// Carries the ROOT identity too, so the root is named exactly once: an `identity` field beside
    /// this one could disagree with the closure it was supposed to be the head of.
    pub reached: ReachedClosure,
    /// What is wrong with each required sibling that is not in hand.
    pub siblings: Vec<SiblingState>,
    /// Per-source current-tree observations, for the sources the edge looked at.
    pub observations: Vec<SourceObservation>,
    /// The address the question asked about, where it asked about one.
    pub address: Option<RequestedAddress>,
}

/// Names the type and the shape of what it carries, never the material.
impl core::fmt::Debug for WhyFactsInput<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "WhyFactsInput({:?}, {:?}, {} reached, {} siblings, {} observations)",
            self.authentication,
            self.detail,
            self.reached.documents().len(),
            self.siblings.len(),
            self.observations.len()
        )
    }
}

/// Decompose one rooted question's material into the inert model.
#[must_use]
pub fn derive(input: &WhyFactsInput<'_>) -> RecordedWhyFacts {
    let detail = input.detail;
    let sources = source_facts(input, detail);
    let sites = site_facts(input, detail);
    let address = input.address.map(|requested| {
        let source = sources
            .iter()
            .find(|source| source.ordinal == requested.source());
        let observation = input
            .observations
            .iter()
            .find(|observation| observation.ordinal == requested.source());
        address::resolve(
            requested,
            source.and_then(SourceFacts::text),
            observation
                .and_then(|observation| observation.reading.bytes())
                .map(Vec::as_slice),
            source.map_or(CurrentSourceState::NotCompared, SourceFacts::current),
            &placements(&sites),
        )
    });

    RecordedWhyFacts {
        invocation: invocation_facts(input, detail),
        narratives: narrative_facts(input.model),
        admission: admission_facts(input, detail),
        presented: presented_facts(input.model),
        regions: region_facts(input, detail),
        loads: load_facts(input, detail),
        classifications: classification_facts(input.model),
        certifications: certification_facts(input.model),
        ships: ship_facts(input, detail),
        survivals: survival_facts(input, detail),
        renders: render_facts(input, detail),
        licensors: licensor_facts(input, detail),
        root: RootFacts::of(
            input.reached.root().clone(),
            input.order,
            input.authentication,
            match detail {
                DetailState::NotCarried => ProjectionState::Plain,
                DetailState::Available | DetailState::Unavailable => ProjectionState::Rich,
            },
            detail,
        ),
        closure: ClosureFacts::of(input.reached.clone(), input.siblings.clone()),
        address,
        sites,
        sources,
        omissions: omission_facts(input.model),
        // The kernel seat that would answer this is deliberately not built in this arc, and saying
        // so is the honest answer. Never a fabricated current disposition.
        rederivation: ReDerivationState::PendingKernelSupport,
    }
}

/// Every recorded site's authored placement, for the address resolver.
fn placements(sites: &[SiteFacts]) -> Vec<AuthoredPlacement> {
    sites
        .iter()
        .filter_map(|site| {
            let authored = site.authored_origin()?;
            Some(AuthoredPlacement {
                site: site.site(),
                source: authored.source()?,
                span: authored.span()?,
            })
        })
        .collect()
}

fn source_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<SourceFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::Source);
    input
        .model
        .sources()
        .iter()
        .enumerate()
        .map(|(position, source)| {
            let content = MaterialState::of(source.content(), detail);
            let observation = input
                .observations
                .iter()
                .find(|observation| observation.ordinal == source.ordinal());
            SourceFacts {
                ordinal: source.ordinal(),
                class: source.class(),
                digest: source.digest(),
                bytes: source.bytes(),
                content,
                path: MaterialState::of(source.path(), detail),
                current: observation.map_or(CurrentSourceState::NotCompared, |observation| {
                    observation.reading.standing(observation.matches_digest)
                }),
                text: content
                    .is_held()
                    .then(|| ordinals.get(position).copied())
                    .flatten()
                    .and_then(|record| {
                        detail_value(
                            input.root,
                            record,
                            OpaqueFieldTag::SourceContent,
                            ValueClass::SourceText,
                        )
                    }),
            }
        })
        .collect()
}

/// The record ordinals of every row of one kind, in document order.
///
/// Read off the document's OWN record stream rather than derived by counting which species the
/// projection emits first. A detail entry is keyed by record POSITION, so a consumer that
/// re-derived that position would be a second copy of the projection's ordering — and when the two
/// copies disagreed, every enrichment would land on whichever row shared its integer, with the
/// document still validating cleanly.
fn ordinals_of(
    root: &Reingested<Receipt<PlanReceipt, Rich>>,
    kind: crate::grammar::RecordKind,
) -> Vec<u64> {
    root.record_kinds()
        .into_iter()
        .enumerate()
        .filter(|(_, found)| *found == kind)
        .filter_map(|(position, _)| u64::try_from(position).ok())
        .collect()
}

fn site_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<SiteFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::SiteDecision);
    input
        .model
        .sites()
        .iter()
        .enumerate()
        .map(|(position, site)| {
            let record = ordinals.get(position).copied();
            let locator_state = MaterialState::of(site.locator(), detail);
            let shell_state = MaterialState::of(site.shell(), detail);
            let chain = locator_state
                .is_held()
                .then_some(record)
                .flatten()
                .and_then(|record| decoded_locator(input.root, record))
                .map(|locator| chain_facts(&locator))
                .unwrap_or_default();
            SiteFacts {
                site: site.site(),
                ast: site.ast(),
                disposition: site.disposition(),
                influence: site.account(),
                shell: shell_state,
                shell_text: shell_state
                    .is_held()
                    .then_some(record)
                    .flatten()
                    .and_then(|record| {
                        detail_value(
                            input.root,
                            record,
                            OpaqueFieldTag::Shell,
                            ValueClass::ShellText,
                        )
                    }),
                // A locator the skeleton says is captured and the payload will not parse is
                // UNDECODABLE, not held: the chain below would be empty and a reader would have no
                // way to tell that from a site whose provenance was one stage long.
                locator: if locator_state.is_held() && chain.is_empty() {
                    MaterialState::Undecodable
                } else {
                    locator_state
                },
                chain,
            }
        })
        .collect()
}

fn decoded_locator(
    root: &Reingested<Receipt<PlanReceipt, Rich>>,
    record: u64,
) -> Option<DurableLocator> {
    let bytes = root.detail(record, OpaqueFieldTag::SiteLocator)?;
    DurableLocator::decode(bytes, &crate::limits::ReceiptLimits::V1).ok()
}

fn chain_facts(locator: &DurableLocator) -> Vec<StageFacts> {
    locator
        .chain()
        .into_iter()
        .filter_map(|index| locator.stage(index))
        .map(|stage| StageFacts {
            kind: stage.kind(),
            source: stage.source().map(crate::rows::SourceOrdinal::get),
            span: stage.span(),
            text: match stage.text_kind() {
                StageTextKind::None => None,
                StageTextKind::Artifact => Some(RecordedValue::sealed(
                    ValueClass::ArtifactLabel,
                    stage.text().to_vec(),
                )),
                StageTextKind::Claim => Some(RecordedValue::sealed(
                    ValueClass::OriginClaim,
                    stage.text().to_vec(),
                )),
            },
        })
        .collect()
}

fn detail_value(
    root: &Reingested<Receipt<PlanReceipt, Rich>>,
    record: u64,
    tag: OpaqueFieldTag,
    class: ValueClass,
) -> Option<RecordedValue> {
    root.detail(record, tag)
        .map(|bytes| RecordedValue::sealed(class, bytes.to_vec()))
}

/// The invocation singleton, with its host destination taken from the region under its own record
/// ordinal.
///
/// The ordinal comes off the document's OWN record stream for `ordinals_of`'s reason: a consumer
/// that re-derived which record the invocation is would be a second copy of the projection's
/// ordering, and the two disagreeing enriches whichever row shares the integer.
fn invocation_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> InvocationFacts {
    let record = ordinals_of(input.root, crate::grammar::RecordKind::Invocation)
        .first()
        .copied();
    let target = MaterialState::of(input.model.invocation_target(), detail);
    InvocationFacts {
        mode: input.model.mode(),
        started: input.model.invocation_started(),
        attempt: input.model.invocation_attempt(),
        argv: MaterialState::of(input.model.invocation_argv(), detail),
        target,
        target_text: target
            .is_held()
            .then_some(record)
            .flatten()
            .and_then(|record| {
                detail_value(
                    input.root,
                    record,
                    OpaqueFieldTag::TargetName,
                    ValueClass::TargetName,
                )
            }),
        influence: input.model.invocation_account(),
    }
}

fn narrative_facts(model: &Reingested<RecordedPlanReceipt>) -> Vec<NarrativeFacts> {
    model
        .narratives()
        .iter()
        .map(|narrative| NarrativeFacts {
            ordinal: narrative.ordinal(),
            speech: narrative.speech(),
            kind: narrative.kind(),
            operands: narrative.operands(),
            influence: narrative.account(),
        })
        .collect()
}

/// The detail value one row of `kind` carries in `tag`, where its slot is held.
///
/// The record ordinal comes off the document's OWN record stream, exactly as `ordinals_of`'s doc
/// requires: a projection that re-derived which record a row is would be a second copy of the
/// emission order, and the two disagreeing enriches whichever row shares the integer.
fn row_detail(
    root: &Reingested<Receipt<PlanReceipt, Rich>>,
    ordinals: &[u64],
    position: usize,
    state: MaterialState,
    tag: OpaqueFieldTag,
) -> Option<RecordedValue> {
    state
        .is_held()
        .then(|| ordinals.get(position).copied())
        .flatten()
        .and_then(|record| detail_value(root, record, tag, ValueClass::of_tag(tag)))
}

fn admission_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Option<AdmissionFacts> {
    let admission = input.model.admission()?;
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::Admission);
    let stream = MaterialState::of(admission.stream(), detail);
    Some(AdmissionFacts {
        outcome: admission.outcome(),
        records: admission.records(),
        bytes: admission.bytes(),
        stream,
        stream_text: row_detail(
            input.root,
            &ordinals,
            0,
            stream,
            OpaqueFieldTag::RecordStream,
        ),
        influence: admission.account(),
    })
}

fn presented_facts(model: &Reingested<RecordedPlanReceipt>) -> Option<PresentedPlanFacts> {
    let presented = model.presented()?;
    Some(PresentedPlanFacts {
        planning_input: presented.planning_input(),
        presented_plan: presented.presented_plan(),
        planned_image: presented.planned_image(),
        influence: presented.account(),
    })
}

fn region_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<RegionFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::RegionDecision);
    input
        .model
        .regions()
        .iter()
        .enumerate()
        .map(|(position, region)| {
            let shell = MaterialState::of(region.shell(), detail);
            RegionFacts {
                region: region.region(),
                ast: region.ast(),
                disposition: region.disposition(),
                routes: region.routes(),
                shell,
                shell_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    shell,
                    OpaqueFieldTag::Shell,
                ),
                influence: region.account(),
            }
        })
        .collect()
}

fn load_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<LoadFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::LoadDecision);
    input
        .model
        .loads()
        .iter()
        .enumerate()
        .map(|(position, load)| {
            let name = MaterialState::of(load.name(), detail);
            let custody = MaterialState::of(load.custody(), detail);
            LoadFacts {
                ordinal: load.ordinal(),
                outcome: load.outcome(),
                name,
                name_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    name,
                    OpaqueFieldTag::ImportPath,
                ),
                custody,
                custody_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    custody,
                    OpaqueFieldTag::Custody,
                ),
                influence: load.account(),
            }
        })
        .collect()
}

fn classification_facts(model: &Reingested<RecordedPlanReceipt>) -> Vec<ClassificationFacts> {
    model
        .classifications()
        .iter()
        .map(|classification| ClassificationFacts {
            site: classification.site(),
            ast: classification.ast(),
            class: classification.class(),
            verdict_lane: classification.verdict_lane(),
            invalidator: classification.invalidator(),
            cells: classification.cells(),
            influence: classification.account(),
        })
        .collect()
}

fn certification_facts(model: &Reingested<RecordedPlanReceipt>) -> Vec<CertificationFacts> {
    model
        .certifications()
        .iter()
        .map(|certification| CertificationFacts {
            pass: certification.pass(),
            consistent: certification.consistent(),
            tripped: certification.tripped(),
            influence: certification.account(),
        })
        .collect()
}

fn ship_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<ShipFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::ProbeShip);
    input
        .model
        .ships()
        .iter()
        .enumerate()
        .map(|(position, ship)| {
            let source = MaterialState::of(ship.source(), detail);
            ShipFacts {
                site: ship.site(),
                lane: ship.lane(),
                source,
                source_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    source,
                    OpaqueFieldTag::Shell,
                ),
                influence: ship.account(),
            }
        })
        .collect()
}

fn survival_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<SurvivalFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::Survival);
    input
        .model
        .survivals()
        .iter()
        .enumerate()
        .map(|(position, survival)| {
            let poison = MaterialState::of(survival.poison(), detail);
            SurvivalFacts {
                site: survival.site(),
                outcome: survival.outcome(),
                wall: survival.wall(),
                aggregate: survival.aggregate(),
                poison,
                poison_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    poison,
                    OpaqueFieldTag::Locator,
                ),
                influence: survival.account(),
            }
        })
        .collect()
}

fn render_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<RenderFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::RenderDecision);
    input
        .model
        .renders()
        .iter()
        .enumerate()
        .map(|(position, render)| {
            let carried = MaterialState::of(render.detail(), detail);
            RenderFacts {
                subject: render.subject(),
                kind: render.kind(),
                detail: carried,
                detail_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    carried,
                    OpaqueFieldTag::DiagnosticOperand,
                ),
                influence: render.account(),
            }
        })
        .collect()
}

fn licensor_facts(input: &WhyFactsInput<'_>, detail: DetailState) -> Vec<LicensorFacts> {
    let ordinals = ordinals_of(input.root, crate::grammar::RecordKind::Licensor);
    input
        .model
        .licensors()
        .iter()
        .enumerate()
        .map(|(position, licensor)| {
            let locus = MaterialState::of(licensor.locus(), detail);
            LicensorFacts {
                site: licensor.site(),
                license: licensor.license(),
                custody: licensor.custody(),
                locus,
                locus_text: row_detail(
                    input.root,
                    &ordinals,
                    position,
                    locus,
                    OpaqueFieldTag::Locator,
                ),
                influence: licensor.account(),
            }
        })
        .collect()
}

fn omission_facts(model: &Reingested<RecordedPlanReceipt>) -> Vec<OmissionFacts> {
    model
        .omissions()
        .iter()
        .map(|omission| OmissionFacts {
            species: omission.species(),
            count: omission.count(),
        })
        .collect()
}

/// The one stage kind an address resolves to, re-stated so this module reads without the reader
/// having to hold the locator's own vocabulary.
const _AUTHORED: RecordedStageKind = RecordedStageKind::Authored;
