//! The MODEL of a why-chain: everything a walker derived, the welded conclusion it reached, and
//! what a render selected out of it — plus why.
//!
//! The three-part shape is `28E:ask-tasty-productive-knob`'s demand: a surface that answers one
//! question well (the productive pole) and a surface that shows the whole derivation (the tasty
//! pole) are the SAME model at two densities, so the model has to retain the conclusion, the
//! narrative residue, and the selection metadata relating them. As built, every walker selects
//! everything it derived — there is no residue yet — so this is room, not machinery. The room is
//! what makes `--all` able to promise exhaustiveness and mean it
//! (`ask-all-flag-promises-exhaustive`).
//!
//! Sited in the describe plane beside [`Said`](crate::said::Said) (`aid-is-the-describe-plane`):
//! these are the shapes a render reads. The WALKERS that build them stay where the plan lives.
//!
//! MUST NOT FORECLOSE (the constraints the shape is chosen to keep open):
//!
//! * [`Relevance`] never becomes a `bool`. kTASTE is two GOALS × densities, not one axis, and a
//!   boolean would decide that question by accident.
//! * The residue is stored as PARTS ([`Said`](crate::said::Said)), never as pre-rendered strings.
//!   Flattening it puts the tasty pole's material at the productive pole's resolution, which is
//!   the exact collapse the ruling forbids.
//! * [`LinkSelection::superseded_by`] and [`LinkSelection::implied_by`] are DAG edges by
//!   [`LinkRef`], never a sorted position (`28E:lean-ordering-is-a-seam`).
//! * The arrangement registry's `occurrence` never carries a density register — it is already
//!   spent on position discriminators, and a register needs a third key axis
//!   (`ask-register-key-axis-reserved`). No register machinery, no density selection, no
//!   `--terse`, lives here or anywhere else yet.

use crate::narrative::SpeechAct;
use crate::said::Said;

/// One quoted-speakers row of a `dorc why <addr>` ANALYSIS panel (`28E` §8 quoted-speakers,
/// ADOPTED): speaker first, the tier word as the sentence's verb, the payload as the speaker's own
/// quoted words. Dorc asserts no world-fact in its own voice — it QUOTES speakers, and vouches
/// only for the run record and for its own derivations (which is why an engine row's payload is
/// unquoted).
#[derive(Clone, Debug)]
pub struct ChainLink {
    /// The epistemic act this row performs (`trust-tier-is-syntax`).
    pub tier: SpeechAct,
    /// Who is speaking: an oracle `file:line`, a book site's `N|command`, or the engine. `None`
    /// when the model carries no locus for this speaker (rendered as an empty column, never faked).
    pub speaker: Option<String>,
    /// The payload's own words, and the registry row they came from — so the render can stamp the
    /// span with the entry an edit would rewrite rather than with the seat that assembled it.
    pub payload: Said,
    /// Whether the payload is the speaker's own words (quoted) or dorc's narration of them (bare).
    pub quoted: bool,
    /// Metadata about the SPEAKING rather than the thing said — when a check ran and what it
    /// exited with (`28G` strawman `a-fire-morning`'s `(ran 01:59:52, rc 0)`). It renders OUTSIDE
    /// the quotation, because attributing the circumstances to the speaker puts words in their
    /// mouth.
    pub event: Option<Said>,
    /// The indented paragraph carried below the quote — today only the at-most claim's
    /// covers-unmeasured disclosure.
    pub explanation: Option<Said>,
    /// The speaker's own source, inlined beneath the explanation: the arm plus the author's
    /// adjacent comment (`27W:rul-report-surface-massaging`). Not our bytes.
    pub excerpt: Option<Excerpt>,
}

/// A speaker's own source, inlined beneath their row: the file it came from, the numbered lines,
/// and whether a middle was cut out of them.
#[derive(Clone, Debug)]
pub struct Excerpt {
    /// The file the lines were taken from.
    pub path: String,
    /// The retained head, as `(line number, text)`.
    pub head: Vec<(usize, String)>,
    /// The retained tail, when a middle was cut. Empty when the excerpt is contiguous.
    pub tail: Vec<(usize, String)>,
    /// How many lines the cut dropped. Zero when the excerpt is contiguous.
    pub elided: usize,
}

/// Everything a walker derived, plus what a render selected out of it and why.
#[derive(Clone, Debug)]
pub struct ChainModel {
    /// EVERY link the walk produced — the residue included. A render narrows; the model never
    /// does, which is what lets `--all` be exhaustive rather than merely wordier.
    pub links: Vec<ChainLink>,
    /// The welded synthesis the chain arrives at: the ANALYSIS panel's numberless restatement.
    /// `None` when the shape reaches no conclusion worth restating.
    pub conclusion: Option<Said>,
    /// Parallel to [`links`](Self::links) — index `i` selects link `i`. A missing entry reads as
    /// selected, so a walker that has nothing to say about selection is not thereby hiding rows.
    pub selection: Vec<LinkSelection>,
}

impl ChainModel {
    /// The chain as every walker builds one TODAY: each derived link selected, nothing superseded,
    /// nothing implied. Naming the state explicitly is the point — a later walk that drops a link
    /// from a render has to say so here rather than by not pushing it.
    #[must_use]
    pub fn all_selected(links: Vec<ChainLink>, conclusion: Option<Said>) -> Self {
        let selection = links.iter().map(|_| LinkSelection::selected()).collect();
        Self {
            links,
            conclusion,
            selection,
        }
    }

    /// The links a render shows: the selected ones, or ALL of them when the reader asked for the
    /// deepest tier (`dorc why … --all`, whose printed promise is "every link, unselected,
    /// exhaustive" — `28E` §7 held-placement-reread: a pointer line must be copy-paste-true).
    #[must_use]
    pub fn rendered(&self, exhaustive: bool) -> Vec<&ChainLink> {
        self.links
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                exhaustive
                    || self
                        .selection
                        .get(*index)
                        .is_none_or(|selection| selection.relevance.is_selected())
            })
            .map(|(_, link)| link)
            .collect()
    }
}

/// Why one link is where the render put it.
#[derive(Clone, Debug)]
pub struct LinkSelection {
    /// How this link bears on the question asked.
    pub relevance: Relevance,
    /// The link that says the same thing better, if one does — a DAG EDGE, never an order.
    pub superseded_by: Option<LinkRef>,
    /// The links this one follows from — DAG EDGES, never an order.
    pub implied_by: Vec<LinkRef>,
}

impl LinkSelection {
    /// Selected, unsuperseded, standing on nothing else.
    #[must_use]
    pub fn selected() -> Self {
        Self {
            relevance: Relevance::Selected,
            superseded_by: None,
            implied_by: Vec::new(),
        }
    }
}

/// How a link bears on the question the reader asked.
///
/// ONE variant today, deliberately (`weft::Register`'s precedent): the kinds a walk could
/// eventually distinguish — restatement, background, contradiction — are a design question, and an
/// enum with one arm keeps the answer open where a `bool` would close it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Relevance {
    /// The default render shows this link.
    #[default]
    Selected,
}

impl Relevance {
    /// Whether the DEFAULT render shows a link of this relevance.
    #[must_use]
    pub fn is_selected(self) -> bool {
        matches!(self, Relevance::Selected)
    }
}

/// An edge target: an index into [`ChainModel::links`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkRef(pub usize);
