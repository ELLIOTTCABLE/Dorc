//! Assembling [`RecordedWhyFacts`](super::RecordedWhyFacts) from already-decoded receipt material.
//!
//! Everything here is a pure function of what it was handed. The edge has already resolved roots,
//! opened the keyset, walked the store, verified signatures and read whatever current source it
//! chose to; this seat receives the OUTCOMES as data and decomposes them. That split is what keeps
//! the crate free of filesystem, provider and key implementations while still being the home of
//! the decomposition.

use super::address::{self, AuthoredPlacement};
use super::states::{
    AuthenticationState, CurrentSourceState, DetailState, MaterialState, ProjectionState,
    RecordedDocumentId, SiblingState,
};
use super::value::{RecordedValue, ValueClass};
use super::{
    ClosureFacts, OmissionFacts, ReDerivationState, RecordedWhyFacts, RequestedAddress, RootFacts,
    SiteFacts, SourceFacts, StageFacts,
};
use crate::durable_locator::{DurableLocator, RecordedStageKind, StageTextKind};
use crate::model::{PlanReceipt, Rich};
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
    /// Its identity and store order.
    pub identity: RecordedDocumentId,
    /// The order token it was filed under, as spelled.
    pub order: String,
    /// What outer verification said.
    pub authentication: AuthenticationState,
    /// Whether its grouped detail region opened.
    pub detail: DetailState,
    /// Every other document the rooted question reached, in graph order.
    pub reached: Vec<RecordedDocumentId>,
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
            self.reached.len(),
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

    let mut reached = vec![input.identity.clone()];
    reached.extend(input.reached.iter().cloned());

    RecordedWhyFacts {
        root: RootFacts::of(
            input.identity.clone(),
            input.order.clone(),
            input.authentication,
            match detail {
                DetailState::NotCarried => ProjectionState::Plain,
                DetailState::Available | DetailState::Unavailable => ProjectionState::Rich,
            },
            detail,
        ),
        closure: ClosureFacts::of(reached, input.siblings.clone()),
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
                site: (site.leaf(), site.member()),
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
                leaf: site.site().leaf().get(),
                member: site.site().member().map(crate::rows::RecordedMember::get),
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
