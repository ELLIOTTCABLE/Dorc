//! `dorc-why` — the RECONSTRUCTION plane a receipt-rooted `why` question is rebuilt onto
//! (`30V` §3), and nothing else.
//!
//! ```text
//! sealed receipt facts + typed current-source observations  ->  Reconstruction
//!                                                                    |
//!                                        (cli) total surface / --json sibling
//! ```
//!
//! # What this is not
//!
//! Not a renderer: nothing here lays anything out, and the only way a recorded byte leaves is
//! through a `ValueEncoder` the CALLER supplies. Not kernel-resident: "kernel" stays reserved for
//! the apply-correctness-critical core, and nothing here may reach a decision — every value is
//! report-plane, and `inv-reingested-material-never-authorizes-action` binds it exactly as it binds
//! the sealed model this is built from.
//!
//! # The two audit causes
//!
//! `30V` §5's durable-gap audit falls out of the model rather than being written beside it: a slot
//! nothing populates is a [`Datum`] whose payload is an affirmative absence, and its
//! [`CarrierAbsence`] says WHOSE — the carrier never recorded it, or the document holds it and
//! `dorc_receipt::report` does not project it. Only the first is a durable question; the second
//! needs no durable change at all. Widening the durable is out of scope either way
//! (`30R:governed-change-process`).
//!
//! # Determinism
//!
//! Pure, ordered, and free of clock, RNG, filesystem and network by construction — the crate's two
//! dependencies are pure data. Every collection is an ordered `Vec` built in one canonical walk, so
//! a reconstruction is byte-stable across input permutations that carry the same facts.

pub mod datum;
pub mod known;
pub mod recorded;
pub mod structure;

pub use datum::{
    AddressSubject, AttemptLineage, CarrierRef, CorrelationFact, Datum, Delivery, FindingKind,
    HostName, IdentityFact, Moment, NegativeKind, NegativeSpace, Payload, Separability, Speaker,
    StateFact, Subject, Voice, VoiceSet, WorldCoordinate,
};
pub use known::{CantTell, CarrierAbsence, Held, Known, UnknowableReason, WithholdReason};
pub use structure::{
    Locus, LocusAddress, LocusDag, LocusEdge, Namespace, SourceAgreement, Structure,
};

use dorc_receipt::report::{
    AuthenticationState, DetailState, PlanFamily, ProjectionState, RecordedDocumentId,
    RecordedSpecies, SiblingState,
};

/// Everything one rooted question was reconstructed into.
///
/// ONE flat datum population plus the structure it is navigated by. Flat because the totality
/// census has to be able to say "every datum reached the render exactly once", and a nested
/// population makes that claim a walk somebody can get wrong rather than a permutation check.
#[derive(Debug, Clone)]
pub struct Reconstruction {
    carriers: Vec<Carrier>,
    data: Vec<Datum>,
    structure: Structure,
}

impl Reconstruction {
    /// Bind one question's carriers, data and structure.
    #[must_use]
    pub const fn of(carriers: Vec<Carrier>, data: Vec<Datum>, structure: Structure) -> Self {
        Self {
            carriers,
            data,
            structure,
        }
    }

    /// Every carrier the rooted question reached, root first.
    #[must_use]
    pub fn carriers(&self) -> &[Carrier] {
        &self.carriers
    }

    /// Every datum, in canonical content-derived order.
    #[must_use]
    pub fn data(&self) -> &[Datum] {
        &self.data
    }

    /// One datum by position — the identity the totality census counts.
    #[must_use]
    pub fn datum(&self, id: DatumId) -> Option<&Datum> {
        self.data.get(id.get())
    }

    /// The edge-families.
    #[must_use]
    pub const fn structure(&self) -> &Structure {
        &self.structure
    }

    /// The carrier a datum was delivered by, where it was delivered by one.
    #[must_use]
    pub fn carrier_of(&self, datum: &Datum) -> Option<&Carrier> {
        match datum.delivery() {
            Delivery::Recorded(reference) => self.carriers.get(reference.get()),
            Delivery::Live => None,
        }
    }

    /// The durable-gap audit (`30V` §5), DERIVED rather than stored: every slot that came back an
    /// affirmative absence, with the cause that says whose hole it is.
    ///
    /// Derived so it cannot drift from what the surface actually renders — an audit assembled
    /// beside the data would be a second claim about the same population.
    #[must_use]
    pub fn audit(&self) -> Vec<Hole> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(index, datum)| {
                let subject = match datum.subject() {
                    Known::Knowable(Held::Present(Subject::Family(family))) => Some(*family),
                    _ => None,
                }?;
                let cause = match datum.payload() {
                    Known::Knowable(Held::AbsentFromCarrier(cause)) => Some(*cause),
                    Known::Knowable(Held::Present(Payload::NegativeSpace(space))) => {
                        Some(match space.kind {
                            NegativeKind::ReportApiGap => CarrierAbsence::ReportApiLacks,
                            NegativeKind::CarrierGap => CarrierAbsence::RunHeldNoValue,
                        })
                    }
                    _ => None,
                }?;
                Some(Hole {
                    datum: DatumId::of(index),
                    family: subject,
                    cause,
                })
            })
            .collect()
    }
}

/// One datum's position in the flat population — the identity the totality census counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DatumId(usize);

impl DatumId {
    /// Name one position.
    #[must_use]
    pub const fn of(index: usize) -> Self {
        Self(index)
    }

    /// The position.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

/// One axis this reconstruction could not populate, and whose hole it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hole {
    /// The datum that carries the absence.
    pub datum: DatumId,
    /// Which recorded family it is about.
    pub family: PlanFamily,
    /// Whether the carrier lacks it or the report API does.
    pub cause: CarrierAbsence,
}

/// One document the rooted question reached, and its standing.
///
/// Standing lives HERE and never on the datum: `30V` §3 has a datum name its carrier BY REFERENCE
/// precisely so authentication and completeness are looked up on the carrier entity rather than
/// copied onto every datum, where two copies could disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carrier {
    /// Which document.
    pub document: RecordedDocumentId,
    /// Its species.
    pub species: RecordedSpecies,
    /// Why it is in the closure.
    pub role: CarrierRole,
    /// What outer verification said. Independent of every other field here
    /// (`30R:standing-invariants`).
    pub authentication: Known<AuthenticationState>,
    /// Which projection it is.
    pub projection: Known<ProjectionState>,
    /// Whether its grouped detail region opened.
    pub detail: Known<DetailState>,
}

/// Why a document is in the rooted closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarrierRole {
    /// The selected root.
    Root,
    /// Reached by the question-directed causal walk.
    Reached,
    /// Required and not in hand. Carries what is wrong with it; a missing edge is INCOMPLETENESS
    /// and never a history (`30R`).
    Sibling(SiblingState),
}
