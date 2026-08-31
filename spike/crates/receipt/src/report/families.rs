//! Which persisted family is projected, and which is not — the read surface made EXHAUSTIVE.
//!
//! `RecordedWhyFacts` began as a projection of three families out of the fifteen a plan document
//! persists, and a consumer had no way to tell a family that is genuinely absent from one nobody
//! had projected yet. That is the gap this closes: every persisted family is either a typed facts
//! collection on the model or explicitly classified, and the classification is CLOSED, so a
//! sixteenth family cannot land unclassified.
//!
//! # What this is not
//!
//! Not a widening of what the durable persists, nor of the grammar, the writer, the wire, the
//! projection states, or the providers. Nothing new is recorded and nothing new is read from a
//! document — this is read-surface projection over material the reader ALREADY validated. Every
//! standing law binds unchanged: closed recorded tokens stay typed
//! (`inv-identities-never-cross-domains`), arbitrary values leave only through the class-aware
//! encoder (`inv-report-is-the-public-read-boundary`), and nothing here converts to a live claim
//! (`inv-recorded-values-stay-recorded`). No raw model accessor and no overlay accessor is exposed.

use crate::reingested::RecordedInfluence;
use crate::rows::RecordedOperands;
use crate::tokens::{RecordedInvocationMode, RecordedNarrativeKind, RecordedSpeechAct};

use super::states::MaterialState;
use super::value::RecordedValue;

/// One family of rows a plan document persists.
///
/// CLOSED and exhaustive over the recorded plan model. The point of naming them here is that
/// [`super::RecordedWhyFacts::coverage`] must answer for every one, so a family reaching the
/// durable without reaching this list is a compile error rather than a silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlanFamily {
    /// The invocation singleton.
    Invocation,
    /// The acquired-source table.
    Sources,
    /// The records-admission singleton.
    Admission,
    /// The presented-plan singleton.
    PresentedPlan,
    /// Per-site decisions.
    Sites,
    /// Per-region decisions.
    Regions,
    /// Per-load decisions.
    Loads,
    /// Per-site classifications.
    Classifications,
    /// Solve certifications.
    Certifications,
    /// Probe ships.
    Ships,
    /// Survivals.
    Survivals,
    /// Render decisions.
    Renders,
    /// Decision-inert narratives.
    Narratives,
    /// Licensors.
    Licensors,
    /// Projection omissions.
    Omissions,
}

impl PlanFamily {
    /// Every family, in the recorded model's own field order.
    pub const ALL: &'static [Self] = &[
        Self::Invocation,
        Self::Sources,
        Self::Admission,
        Self::PresentedPlan,
        Self::Sites,
        Self::Regions,
        Self::Loads,
        Self::Classifications,
        Self::Certifications,
        Self::Ships,
        Self::Survivals,
        Self::Renders,
        Self::Narratives,
        Self::Licensors,
        Self::Omissions,
    ];

    /// The word a report names it by.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Invocation => "invocation",
            Self::Sources => "sources",
            Self::Admission => "admission",
            Self::PresentedPlan => "presented-plan",
            Self::Sites => "sites",
            Self::Regions => "regions",
            Self::Loads => "loads",
            Self::Classifications => "classifications",
            Self::Certifications => "certifications",
            Self::Ships => "ships",
            Self::Survivals => "survivals",
            Self::Renders => "renders",
            Self::Narratives => "narratives",
            Self::Licensors => "licensors",
            Self::Omissions => "omissions",
        }
    }
}

/// What this model can say about one family.
///
/// FOUR states, and they are different facts: a family with a typed projection, one the document
/// carries that nobody has projected yet, one this document genuinely does not carry, and one that
/// does not apply to the question. Merging any pair would let a reader infer a durable gap from a
/// read-surface gap, which are repaired in completely different places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyCoverage {
    /// A typed facts collection carries it, with this many members.
    Projected(usize),
    /// The document persists it and this read surface does not project it yet. NOT a durable
    /// question: closing it is projection work in this module.
    RecordedButUnprojected,
    /// This document carries no rows of the family.
    NotCarried,
    /// The family does not apply to the rooted question.
    NotRelevant,
}

impl FamilyCoverage {
    /// A projected family whose collection may legitimately be empty.
    pub(crate) const fn of(members: usize) -> Self {
        Self::Projected(members)
    }

    /// Whether a consumer can read typed facts for this family today.
    #[must_use]
    pub const fn is_projected(self) -> bool {
        matches!(self, Self::Projected(_))
    }
}

/// The invocation singleton, as the document recorded it.
#[derive(Debug, Clone)]
pub struct InvocationFacts {
    pub(crate) mode: RecordedInvocationMode,
    pub(crate) started: Option<u64>,
    pub(crate) attempt: u32,
    pub(crate) argv: MaterialState,
    pub(crate) target: MaterialState,
    pub(crate) target_text: Option<RecordedValue>,
    pub(crate) influence: RecordedInfluence,
}

impl InvocationFacts {
    /// What the run was doing.
    #[must_use]
    pub const fn mode(&self) -> RecordedInvocationMode {
        self.mode
    }

    /// The controller's own start reading, where one was taken. Controller-minted: a managed host
    /// never contributes an instant.
    #[must_use]
    pub const fn started(&self) -> Option<u64> {
        self.started
    }

    /// Which attempt of its target this was.
    #[must_use]
    pub const fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Whether the argument vector is in the document.
    #[must_use]
    pub const fn argv(&self) -> MaterialState {
        self.argv
    }

    /// Whether the host destination is in the document.
    #[must_use]
    pub const fn target(&self) -> MaterialState {
        self.target
    }

    /// That destination, where it is. Encoder-mediated like every other recorded value.
    #[must_use]
    pub const fn target_text(&self) -> Option<&RecordedValue> {
        self.target_text.as_ref()
    }

    /// Where the run stood relative to host contact.
    ///
    /// Read straight off the recorded grade, whose own seat reads an absent or unrecognised token
    /// as the MOST-influenced grade. Never re-derived here, and never rounded downward.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}

/// One decision-inert narrative the run minted.
///
/// The family that carries the recorded SPEECH ACT, which is why projecting it matters out of
/// proportion to its size: without it a reconstruction can say what the engine decided and not in
/// what act anybody spoke. The row identifies no site — narrative operands are not durable — so a
/// reader learns that N collapses of a class occurred and never which line each was about, and
/// this projection must not suggest otherwise.
#[derive(Debug, Clone, Copy)]
pub struct NarrativeFacts {
    pub(crate) ordinal: u32,
    pub(crate) speech: RecordedSpeechAct,
    pub(crate) kind: RecordedNarrativeKind,
    pub(crate) operands: RecordedOperands,
    pub(crate) influence: RecordedInfluence,
}

impl NarrativeFacts {
    /// Where this narrative sat in mint order.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// The typed speech act.
    #[must_use]
    pub const fn speech(&self) -> RecordedSpeechAct {
        self.speech
    }

    /// Which collapse class narrowed.
    #[must_use]
    pub const fn kind(&self) -> RecordedNarrativeKind {
        self.kind
    }

    /// How many operands were kept, and how many the cap dropped.
    #[must_use]
    pub const fn operands(&self) -> RecordedOperands {
        self.operands
    }

    /// Where the collapse stood relative to host contact.
    #[must_use]
    pub const fn influence(&self) -> RecordedInfluence {
        self.influence
    }
}
