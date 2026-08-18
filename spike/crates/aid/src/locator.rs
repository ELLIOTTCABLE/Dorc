//! `aid::locator` — where a byte CAME FROM, as an arbitrary multi-stage DAG
//! (`30I:rul-source-maps-are-rich-and-early`).
//!
//! The compiler this serves aims to stay stupid: exact copied ranges plus generated scaffolding.
//! The CONSUMER may not, and that asymmetry is the ruling. A user reading an error about a line in
//! a generated artifact wants the artifact's own locus AND the authored bytes it was copied from —
//! and, once bundles nest, every stage in between. A hard-coded `(generated, original)` pair
//! answers today's one hop and has to be re-cut at the first two-hop chain, so the representation
//! is a DAG from the start even while the compiler that fills it is trivial.
//!
//! ```text
//! book source span -> planned source replacement -> bundle load span
//!    -> bundle segment -> nested bundle segment -> original oracle span
//! ```
//!
//! Fan-in is real, not decorative: one generated line can descend from a copied range AND from the
//! load act that pulled it in, and a later flattening COMPOSES another edge rather than
//! overwriting what is already there (`30I` §9.2 item 6).
//!
//! # Decision-inert, structurally
//!
//! Locators are the describe plane (`two-plane-aid-law`). Nothing here may enter solver or lattice
//! equality, and nothing here may reach an authority mint — a license that compared provenance
//! would make fixpoint termination depend on narration and would let a claim about where bytes
//! came from decide what they are allowed to do. The fence is placement plus one lexical test:
//! this module names no decide-plane authority type at all.

use dorc_core::{SourceFileId, Span};

/// A point in one authored source: the file the controller read, and a byte range within it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceLocus {
    /// Which loaded source, in the run's one `SourceFileId` space.
    pub file: SourceFileId,
    /// The byte range within it.
    pub span: Span,
}

impl SourceLocus {
    /// Name a range of one loaded source.
    #[must_use]
    pub const fn at(file: SourceFileId, span: Span) -> Self {
        Self { file, span }
    }
}

/// A point in something the engine GENERATED — a bundle, a plan, a preamble.
///
/// The artifact is named by a label rather than a `SourceFileId` because a generated file is not a
/// loaded source and must never be mistaken for one: the id space this borrows from is the
/// controller's account of what it READ, and a generated artifact was never read.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeneratedLocus {
    /// Which generated artifact, by the emitter's own label.
    pub artifact: String,
    /// The byte range within it.
    pub span: Span,
}

impl GeneratedLocus {
    /// Name a range of one generated artifact.
    #[must_use]
    pub fn at(artifact: impl Into<String>, span: Span) -> Self {
        Self {
            artifact: artifact.into(),
            span,
        }
    }
}

/// One stage of a locator chain.
///
/// The set is closed and grows by NEW NAME only: a consumer that renders stages exhaustively is
/// what makes a new stage a visible edit rather than a silent omission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    /// Bytes exactly as an author wrote them, in a source the controller read.
    Authored(SourceLocus),
    /// The `.` act that brought the next stage's source into the unit — the book or package line a
    /// reader would edit to change WHICH file is involved, as opposed to what it says.
    Loaded(SourceLocus),
    /// Bytes copied verbatim into generated output. Its origins say where from.
    Copied(GeneratedLocus),
    /// Scaffolding the engine wrote itself, descending from no authored bytes.
    Generated(GeneratedLocus),
    /// A generated artifact's own CLAIM about where its bytes came from, read back from a comment.
    /// Narrative, never identity — see [`BundleOriginClaim`].
    Claimed(BundleOriginClaim),
}

/// An index into a [`Locator`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StageId(u32);

/// The DAG: stages plus the origins each descends from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Locator {
    stages: Vec<Stage>,
    origins: Vec<Vec<StageId>>,
}

impl Locator {
    /// Record a stage descending from `origins`, and return its id.
    ///
    /// Origins are ids this same locator already minted; an id from elsewhere names nothing here
    /// and is dropped, which keeps the graph closed rather than dangling.
    pub fn push(&mut self, stage: Stage, origins: &[StageId]) -> StageId {
        let id = StageId(u32::try_from(self.stages.len()).unwrap_or(u32::MAX));
        self.stages.push(stage);
        self.origins.push(
            origins
                .iter()
                .copied()
                .filter(|from| (from.0 as usize) < self.stages.len().saturating_sub(1))
                .collect(),
        );
        id
    }

    /// The stage `id` names.
    #[must_use]
    pub fn stage(&self, id: StageId) -> Option<&Stage> {
        self.stages.get(id.0 as usize)
    }

    /// How many stages this locator holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.stages.len()
    }

    /// Whether this locator holds nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stages.is_empty()
    }

    /// Every stage reachable from `head`, `head` first and then its origins outward, breadth-first
    /// and deduplicated — the order a consumer renders in, most-generated first and most-editable
    /// last.
    ///
    /// Total over a malformed graph: a cycle cannot arise (an origin is always an EARLIER id), and
    /// the visited set is what makes fan-in report each stage once however many paths reach it.
    #[must_use]
    pub fn chain_from(&self, head: StageId) -> Vec<StageId> {
        let mut seen = std::collections::BTreeSet::new();
        let mut order = Vec::new();
        let mut queue = std::collections::VecDeque::from([head]);
        while let Some(id) = queue.pop_front() {
            if (id.0 as usize) >= self.stages.len() || !seen.insert(id) {
                continue;
            }
            order.push(id);
            if let Some(origins) = self.origins.get(id.0 as usize) {
                queue.extend(origins.iter().copied());
            }
        }
        order
    }

    /// The chain from `head` as stages, in the same order.
    #[must_use]
    pub fn resolve(&self, head: StageId) -> Vec<&Stage> {
        self.chain_from(head)
            .into_iter()
            .filter_map(|id| self.stage(id))
            .collect()
    }
}

/// What a generated artifact SAYS about where its bytes came from (`30I:rul-bundle-origin-is-aid-only`).
///
/// A bundle read back in is arbitrary sh: its bytes alone feed analysis, and its comments are a
/// claim by whoever wrote them. So this type carries text and NOTHING ELSE — no `SourceFileId`, no
/// `DefinitionId`, no span into a loaded source, no custody, no dialect, no vouch. There is no
/// accessor that yields one and no constructor that takes one, which is what makes "a comment
/// cannot become identity" unrepresentable rather than merely unimplemented: removing or editing
/// every comment in a bundle must leave the analytic answer byte-identical, and the way to
/// guarantee that is for the parsed claim to have nowhere to go.
///
/// A claim becomes a RESOLVED origin only where matching source bytes are actually available and
/// content identity agrees — a separate act, performed by a consumer holding both, which mints an
/// ordinary [`Stage::Authored`] beside this one rather than converting it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleOriginClaim {
    claimed: String,
}

impl BundleOriginClaim {
    /// Take a claim from generated text. The bytes are retained as written and interpreted by
    /// nothing.
    #[must_use]
    pub fn of(claimed: impl Into<String>) -> Self {
        Self {
            claimed: claimed.into(),
        }
    }

    /// The claim as written, for display beside a resolved locus. Display is its ONLY exit.
    #[must_use]
    pub fn as_claimed(&self) -> &str {
        &self.claimed
    }
}

#[cfg(test)]
mod tests {
    use dorc_core::{BytePos, SourceFileId, Span};

    use super::{BundleOriginClaim, GeneratedLocus, Locator, SourceLocus, Stage};

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(BytePos(lo), BytePos(hi))
    }

    /// THE CHAIN THAT EXISTS TODAY, and the smallest one that forces the representation: a book's
    /// `.` act, and the authored bytes it pulled in. Two stages, one edge — and a pair type would
    /// have answered it, which is precisely why the pair had to be refused before the third stage
    /// arrived rather than after.
    #[test]
    fn a_load_act_and_the_bytes_it_named_are_two_stages() {
        let mut locator = Locator::default();
        let authored = locator.push(
            Stage::Authored(SourceLocus::at(SourceFileId(1), span(40, 80))),
            &[],
        );
        let via = locator.push(
            Stage::Loaded(SourceLocus::at(SourceFileId(2), span(10, 34))),
            &[authored],
        );
        assert_eq!(
            locator.resolve(via),
            vec![
                &Stage::Loaded(SourceLocus::at(SourceFileId(2), span(10, 34))),
                &Stage::Authored(SourceLocus::at(SourceFileId(1), span(40, 80))),
            ],
            "the load act first, the bytes it named behind it"
        );
    }

    /// A later stage COMPOSES an edge rather than replacing history (`30I` §9.2 item 6): flattening
    /// a bundle into a plan adds a stage in front of the bundle's own, and every earlier locus
    /// stays reachable.
    #[test]
    fn a_later_stage_composes_rather_than_overwriting() {
        let mut locator = Locator::default();
        let authored = locator.push(
            Stage::Authored(SourceLocus::at(SourceFileId(1), span(0, 10))),
            &[],
        );
        let bundled = locator.push(
            Stage::Copied(GeneratedLocus::at("deps/alpha.sh", span(20, 30))),
            &[authored],
        );
        let flattened = locator.push(
            Stage::Copied(GeneratedLocus::at("plan.sh", span(100, 110))),
            &[bundled],
        );
        assert_eq!(
            locator.resolve(flattened).len(),
            3,
            "plan, bundle, and the author's own bytes — all still reachable"
        );
    }

    /// FAN-IN: one generated line can descend from copied bytes AND from the load act that pulled
    /// them in. Each origin is reported once however many paths reach it, which is what keeps a
    /// diamond from rendering a stage twice.
    #[test]
    fn fan_in_reports_each_origin_once() {
        let mut locator = Locator::default();
        let authored = locator.push(
            Stage::Authored(SourceLocus::at(SourceFileId(1), span(0, 10))),
            &[],
        );
        let load_a = locator.push(
            Stage::Loaded(SourceLocus::at(SourceFileId(2), span(0, 5))),
            &[authored],
        );
        let load_b = locator.push(
            Stage::Loaded(SourceLocus::at(SourceFileId(3), span(0, 5))),
            &[authored],
        );
        let generated = locator.push(
            Stage::Copied(GeneratedLocus::at("plan.sh", span(0, 10))),
            &[load_a, load_b],
        );
        let chain = locator.resolve(generated);
        assert_eq!(chain.len(), 4);
        assert_eq!(
            chain
                .iter()
                .filter(|stage| matches!(
                    stage,
                    Stage::Authored(locus) if locus.file == SourceFileId(1)
                ))
                .count(),
            1,
            "the shared origin appears once, not once per path"
        );
    }

    /// Generated scaffolding descends from nothing, and says so: a chain that bottoms out in
    /// `Generated` is the honest answer for a line no author wrote.
    #[test]
    fn generated_scaffolding_has_no_origin() {
        let mut locator = Locator::default();
        let head = locator.push(
            Stage::Generated(GeneratedLocus::at("plan.sh", span(0, 12))),
            &[],
        );
        assert_eq!(locator.resolve(head).len(), 1);
    }

    /// A claim is TEXT, and the type is what stops it becoming anything else
    /// (`30I:rul-bundle-origin-is-aid-only`). If this ever compiles into a `SourceFileId`, a
    /// `DefinitionId`, or a span into a loaded source, the guarantee that deleting every bundle
    /// comment leaves the analytic answer byte-identical is gone.
    #[test]
    fn a_bundle_claim_carries_text_and_nothing_else() {
        let claim = BundleOriginClaim::of("org.example.common/entry.oracle.sh");
        assert_eq!(claim.as_claimed(), "org.example.common/entry.oracle.sh");
        let mut locator = Locator::default();
        let head = locator.push(Stage::Claimed(claim), &[]);
        assert!(matches!(locator.stage(head), Some(Stage::Claimed(_))));
    }

    /// The lexical half of the decision-inertness fence (`two-plane-aid-law`): this module names no
    /// decide-plane authority type, so a locator cannot reach a license, a claim tier, or a
    /// definition identity even by accident. The placement argument is what makes it safe; this is
    /// what keeps the placement honest.
    #[test]
    fn this_module_names_no_authority_type() {
        let source = include_str!("locator.rs");
        // CODE, not prose: the doc comments name several of these in order to say the module may
        // not, and a fence that could not tell the two apart would forbid explaining itself.
        let body: String = source
            .split_once("mod tests {")
            .map_or(source, |(head, _)| head)
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                !trimmed.starts_with("//")
            })
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in [
            "ReplaceLicense",
            "GuardLicense",
            "VerdictVouch",
            "ByVouch",
            "ByObservation",
            "DefinitionId",
            "DefinitionCustody",
            "FactKey",
            "Verdict",
        ] {
            assert!(
                !body.contains(forbidden),
                "a locator that can name `{forbidden}` is a narration that can decide"
            );
        }
    }
}
