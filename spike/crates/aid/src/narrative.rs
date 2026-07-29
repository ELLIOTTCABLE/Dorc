//! `aid::narrative` — the **aid/explanation narrative plane** (`27V` Lane A;
//! `AID-NEEDS:law-collapse-mints-narrative`). A [`CollapseNarrative`] is the pure-data record a
//! safety-narrowing collapse mints, carrying the collapse's OPERANDS so the why-lens / report
//! surfaces can narrate WHY the engine gave up — without ever letting that narration steer a
//! decision.
//!
//! # THE LAW — decision-inert, one-way (`two-plane-aid-law`, `26C` §5b, human hard-ack)
//!
//! The license plane fails toward unsureness; this aid plane fails toward narration with
//! attributed confidence. The two planes are welded APART at the type level: **license values
//! flow INTO narration freely, never back**. There is no method on any type in this module that
//! yields a license-plane input (no `ByVouch`, no `Must`, no `RoomFact<Invited,_>`, no verdict);
//! every accessor returns display-tier data (a [`SpeechAct`], a tag, a [`Span`], a site handle).
//! "Lint-clean licenses nothing" (`AID-NEEDS:law-two-planes-opposite-fail`) runs in this
//! direction: nothing here can license.
//!
//! The seal is the [`core::room`](dorc_core::room) pattern (`27L`): the structural half is that no
//! license-consuming signature accepts a [`CollapseNarrative`]. The module-level `compile_fail`
//! doctest below pins it against a REAL license consumer ([`dorc_core::room::mint_from_room`]); the
//! positive doctest shows a vouch INFORMING a tier (license flowing in) while never being
//! retained as a license (it is consumed by-reference, only its existence read).
//!
//! ```compile_fail
//! use dorc_aid::narrative::{CollapseKind, CollapseNarrative, SpeechAct};
//! use dorc_core::room::mint_from_room;
//!
//! let kind = CollapseKind::render_refusal_heredoc(dummy());
//! let narrative = CollapseNarrative::new(SpeechAct::Derived, kind);
//! // A license mint demands a `RoomFact<Invited, _>`. A narrative is decision-inert: it can NEVER
//! // be surrendered to a license input — "collapse-mints-narrative" is a one-way street, a compile
//! // fact, not a discipline (`two-plane-aid-law`).
//! let _ = mint_from_room(narrative);
//! # fn dummy() -> dorc_core::SiteId { unimplemented!() }
//! ```
//!
//! # Kernels stay pure (`27V:rul-collapse-mints-narrative` (né -evidence), `22D` stage-1)
//!
//! Every operand is a `Copy` scalar ([`Span`], [`LeafId`], [`Channel`], …) or an interned handle
//! ([`OutBytes`]) — NO [`ProvId`](dorc_core::ProvId), NO `&mut ProvArena`, NO arena registration
//! inside a [`CollapseNarrative`]. A collapse CONSTRUCTOR demands this pure payload at the VALUE
//! level; assigning it an arena receipt (for the why-lens) is a SEPARATE post-pass, exactly as
//! `analysis::effect::mint_top_causes` mints causes apart from the pure transfer.
//!
//! # Eq is at the carrier, not here (`Reach::Top` precedent, fixpoint termination)
//!
//! [`CollapseNarrative`] derives `Eq` (unit tests compare it). Where a narrative rides a
//! fixpoint-iterated lattice value, that CARRIER hand-writes `PartialEq` to EXCLUDE the narrative
//! — the `analysis::effect::Reach` precedent: `solve`'s convergence test is `joined != state[w]`,
//! so a narrative-sensitive lattice `Eq` would re-derive-as-changed forever and never terminate.
//! Nothing in THIS module iterates a fixpoint, so its own `Eq` is free.

use dorc_core::SiteId;
use dorc_core::{Channel, JOIN_PARENT_CAP, LeafId, OutBytes, SourceFileId, Span};

/// The typed speech-act kind of a rendered link (`27V:mech-trust-tier-typed`;
/// `AID-NEEDS:law-trust-tier-is-syntax`). Rendered UNIFORMLY by arrangement code (d4) — prose
/// fragments never hand-write epistemics, so a claim can never be dressed as a measurement
/// (`271:rul-sin-ordering`: mis-attribution is the worst aid failure). A closed enum (the
/// `OriginKind` posture: adding a kind must break every exhaustive match). Spellings are STRAWMAN
/// and ride `27V:rul-output-form-unwelded` — the kind SET and its typed rendering are the law; the
/// words are unwelded pending real generated output.
///
/// Deliberately UNORDERED (`28F:rul-speechact-rename`): the seven kinds are WHO speaks and WHAT
/// act they perform (a measurement reported, a vouch reached, a claim asserted, …), never a
/// correctness scale. The derived `Ord` below is mechanical map-key determinism only
/// (`inv-determinism`), never a semantic ranking — the one genuine semantic ordering over these
/// kinds is the projection [`Knowability`], via [`SpeechAct::knowability`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpeechAct {
    /// A host measurement (a probe actually observed it).
    Measured,
    /// An author's vouch (a reached `is_converged()` answered in the named sense).
    Vouched,
    /// A command actually ran and its observable is genuine (a guard's live rc).
    Ran,
    /// An oracle declaration (a `predict` printf/return claim).
    Claimed,
    /// An engine derivation (a disjointness proof, a join).
    Derived,
    /// A context-entry consent decision (the escalation dial × capability).
    Consented,
    /// An author's DECLINE — they looked at this shape and ruled it unanswerable
    /// (`decline-class-emission`; `28G` strawman `c-declined-unsound`).
    ///
    /// RATIFIED (human-typed ack 2026-07-26, `28F`): the tier-SET widening is settled. The
    /// alternative was rendering a decline row under an existing tier word, and every one of
    /// them misstates it — `vouches` most of all, since declining is precisely the author refusing
    /// to vouch. Mis-attribution is the worst aid failure (`271:rul-sin-ordering`), so the honest
    /// tier is a new one rather than the nearest old one.
    Declined,
}

impl SpeechAct {
    /// Derive a tier from the EXISTENCE of a vouch (`two-plane-aid-law`: a license flows INTO the
    /// aid plane, informing the tier, and is never retained as a license). Consumes the vouch by
    /// reference and reads nothing but that it is present — the one-way flow made a signature.
    #[must_use]
    pub fn from_vouch<P>(_vouch: &dorc_core::ByVouch<P>) -> Self {
        SpeechAct::Vouched
    }

    /// The ordered [`Knowability`] projection (`28F:rul-speechact-rename`): KIND-CONSTANT over the
    /// as-built chain renders (`spike/crates/cli/src/main.rs`'s survival/guard/decline/plain chain
    /// builders), so this is the ONE derivation seat — both the `*`/`!` mark render and the
    /// naked-trust epilogue's which-link derivation route through it rather than re-deriving a rank
    /// per row.
    ///
    /// `Declined`'s placement here is HUMAN-RULED (`28F:rul-declined-is-bottom-rung`), not by
    /// analogy: machine-uncheckability is not the axis (both rungs hold human-trusted claims no
    /// command can check) — the axis is danger-INTRODUCTION, and a decline can only lose the reader
    /// value, never introduce it, so it sits at the bottom rung outright.
    ///
    /// `Consented` has no as-built [`CollapseKind::EntryDenial`] render today (`narrative-mints-
    /// outrun-renders` — the class mints but nothing consumes it yet); it is assigned `Witnessed`
    /// here by analogy with `Derived` (both are the engine's own closed-world decisions, never an
    /// author's open-world at-most claim) rather than from any observed row. Flagged for conductor
    /// review when a consent row first renders. (This analogy note is unrelated to `Declined`'s
    /// placement above, and remains UNRULED.)
    #[must_use]
    pub const fn knowability(self) -> Knowability {
        match self {
            SpeechAct::Measured
            | SpeechAct::Vouched
            | SpeechAct::Derived
            | SpeechAct::Consented
            | SpeechAct::Declined => Knowability::Witnessed,
            SpeechAct::Ran | SpeechAct::Claimed => Knowability::CoversUnmeasured,
        }
    }
}

/// The ordered super-layer of the danger axis (`28E` §7 adapt-two-rank-default-render, sharpened by
/// §8 `rul-danger-axis-is-completion-class`, further sharpened by `28F:rul-declined-is-bottom-rung`):
/// the seven [`SpeechAct`] kinds stay typed law underneath and are UNTOUCHED by this projection —
/// `Knowability` is a RENDER-relevant partition OVER them. GENUINELY ordered (unlike `SpeechAct`'s
/// own derived `Ord`, which is mechanical map-key determinism only): the axis is danger-
/// INTRODUCTION to the reader, NOT machine-uncheckability — both rungs can hold a
/// machine-uncheckable, human-trusted claim, and that is not what separates them.
/// [`CoversUnmeasured`](Knowability::CoversUnmeasured) speaks for what no runnable command COULD
/// witness — a WIDE at-most claim the reader is asked to trust, which INTRODUCES danger (drawing the
/// `!` mark's extra attention), or the run of a command nobody has described (which claims nothing,
/// and therefore covers everything) — and always outranks
/// [`Witnessed`](Knowability::Witnessed), the class of size N every member of which was observed, OR
/// whose only open-world claim is a NARROW decline that can only lose the reader value, never
/// introduce danger, and so sits at the bottom rung rather than the top.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Knowability {
    /// `*` — the row rests on something that actually happened (a probe report, a real run) or on a
    /// derivation over such things (class of size N, every member observed), OR on an author's
    /// narrow decline (`28F:rul-declined-is-bottom-rung`) — which is why this variant's name now
    /// under-covers what it holds; a rename candidate only if it ever grates.
    Witnessed,
    /// `!` — the row speaks for universe-minus-N: an author's at-most claim, or the run of a command
    /// nobody has described (which claims nothing, and therefore covers everything).
    CoversUnmeasured,
}

/// The k-cap on a [`CollapseNarrative`] operand list through DEEP merges (the [`JOIN_PARENT_CAP`]
/// precedent — `notes/220` §6 / `vp-6`: values are many and capped). Operands past the cap are
/// dropped with a truncation count rendered "…and N more"; a collapse's own operands are few, but
/// nested collapses merging their narratives must stay bounded.
pub const NARRATIVE_OPERAND_CAP: usize = JOIN_PARENT_CAP;

/// A bounded operand list (the [`dorc_core::Parents`]-shaped value tier): the retained operands plus a
/// count of those the cap dropped. An explicit struct, never a bare `Vec`, so truncation is part
/// of the type and never a silently-lossy `Vec::truncate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operands<T> {
    kept: Vec<T>,
    truncated: u32,
}

/// Hand-written (not derived) so an empty operand list needs no `T: Default` bound — the derived
/// `Default` would spuriously require it.
impl<T> Default for Operands<T> {
    fn default() -> Self {
        Self {
            kept: Vec::new(),
            truncated: 0,
        }
    }
}

impl<T: Clone> Operands<T> {
    /// Cap a list of operands at [`NARRATIVE_OPERAND_CAP`], recording the overflow as the
    /// truncation count. The caller offers operands in a stable, site-derived order (never visit
    /// order), so which survive is a deterministic function of the program (`vp-9`).
    #[must_use]
    pub fn capped(operands: Vec<T>) -> Self {
        let total = operands.len();
        let kept: Vec<T> = operands.into_iter().take(NARRATIVE_OPERAND_CAP).collect();
        Self {
            truncated: u32::try_from(total.saturating_sub(NARRATIVE_OPERAND_CAP))
                .unwrap_or(u32::MAX),
            kept,
        }
    }

    /// Merge two operand lists and RE-CAP (deep-merge safety): concatenate the retained operands,
    /// carry both truncation counts, then cap the union so the result never exceeds the bound.
    #[must_use]
    pub fn merge(mut self, mut other: Self) -> Self {
        let carried = self.truncated.saturating_add(other.truncated);
        self.kept.append(&mut other.kept);
        let mut merged = Self::capped(self.kept);
        merged.truncated = merged.truncated.saturating_add(carried);
        merged
    }

    /// The retained operands.
    #[must_use]
    pub fn kept(&self) -> &[T] {
        &self.kept
    }

    /// How many operands the cap dropped (`0` ⇒ none; the "…and N more" count).
    #[must_use]
    pub fn truncated(&self) -> u32 {
        self.truncated
    }
}

/// One disagreeing operand at a fact-merge collapse (`aid-why-disagreement-narration`): the site
/// that established a value, its minting line, and a display of the value it established. All pure
/// / interned — `shown` is `None` when the value has no readily-interned display (the c1 static
/// operand-value recovery lands its own commit; the site+line always carry).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueOperand {
    /// The establisher site (`inv-site-keyed-results`).
    pub site: SiteId,
    /// The defining source line of the establisher (`27V:mech-minting-line-threading`).
    pub minting_line: Option<Span>,
    /// A display of the established value (interned, resolved lazily controller-side), or `None`.
    pub shown: Option<OutBytes>,
}

/// The defining source span of a claim / vouch / emission arm, threaded end-to-end so attribution
/// renders `file:line` (`27V:mech-minting-line-threading`; the `27Q` §2 stdlib precondition;
/// `AID-NEEDS:law-lineno-identity`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MintSpan(pub Span);

/// Which decline shape a verdict body took (`rul-vouch-is-verdict-authoring`): an explicit
/// `return ≥2`, a reached-no-command arm, or an inert fixed-rc builtin. Names the GATE, never the
/// license (the rc-partition stays a flat sink; the license plane never reads this).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclineGate {
    /// An explicit `return N` with `N ≥ 2` (confused ⇒ run).
    Return,
    /// A reached arm that ran no command (`if`-false with no `else`, an empty arm).
    Unreached,
    /// An inert fixed-rc builtin (`false`/`:`/`true`) — runs no check.
    InertBuiltin,
}

/// The author's DELIBERATE decline class (`27W` `rul-class-starter-set`, soft-acked): the closed
/// v1 set. Engine-owned, append-only. Routes AID only (`decline-class-emission`): the rc-partition
/// weld is untouched, the license plane never reads a class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclineClass {
    /// Permanently unprobeable (write-only trigger keys, nondeterministic reads).
    Unsound,
    /// Not-yet-implemented; a better oracle could cover it.
    Unmodeled,
    /// Prompts by construction.
    Interactive,
    /// The author's tool-editorial claim (deprecated/discouraged usage).
    Hazard,
}

impl DeclineClass {
    /// Parse an engine-owned decline-class token (`27W:rul-class-starter-set`, soft-acked; the
    /// closed, append-only v1 set). An unknown token ⇒ `None` — the report lane DEGRADES it to a
    /// generic author-note, never an error (`27W:rul-report-noise-tolerant`).
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "unsound" => Some(Self::Unsound),
            "unmodeled" => Some(Self::Unmodeled),
            "interactive" => Some(Self::Interactive),
            "hazard" => Some(Self::Hazard),
            _ => None,
        }
    }

    /// Whether a BETTER ORACLE could still answer this shape — the one thing the aggregate's
    /// improvements section has to know about a decline.
    ///
    /// `Unmodeled` is the author saying "not yet"; the rest are the author saying "not ever, by
    /// anyone" — a write-an-oracle nag against one of those tells the person who already answered
    /// the question to go answer it (`28G` strawman `c-declined-unsound`: "dorc will not suggest an
    /// oracle for this shape"). Routing only, on the aid plane; no license reads a class
    /// (`decline-class-emission`).
    #[must_use]
    pub fn an_oracle_could_still_answer(self) -> bool {
        match self {
            Self::Unmodeled => true,
            Self::Unsound | Self::Interactive | Self::Hazard => false,
        }
    }

    /// The occurrence discriminator a class-keyed arrangement row is stamped with, so one slug can
    /// carry four class-specific lines and the unwritten ones stay greppable per class.
    #[must_use]
    pub fn occurrence(self) -> usize {
        match self {
            Self::Unsound => 0,
            Self::Unmodeled => 1,
            Self::Interactive => 2,
            Self::Hazard => 3,
        }
    }

    /// The engine-owned display/wire token (the reverse of [`from_token`](Self::from_token)); the one
    /// home for the class spellings (display only, `inv-referent-agnostic`; the words ride
    /// `27V:rul-output-form-unwelded`). `from_token(c.token()) == Some(c)` for every class.
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            Self::Unsound => "unsound",
            Self::Unmodeled => "unmodeled",
            Self::Interactive => "interactive",
            Self::Hazard => "hazard",
        }
    }
}

/// The authored reason a decline carries (`27W` §3): the class plus the emitting arm's source
/// span. The FIELD lands NOW; its POPULATION lands with d3's report-lane ingestion (until then a
/// decline mints `authored_reason: None` — a silent decline stays legal, classing is enhancement).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoredReason {
    /// The author-declared decline class.
    pub class: DeclineClass,
    /// The declining arm's defining source span (`mech-minting-line-threading`).
    pub arm: MintSpan,
    /// Which oracle file `arm` indexes into (`tc-oracle-file-identity`): the arm span crosses out
    /// of its owning file's context to the render, so it carries the same file id the vouch span
    /// does — a bare span is ambiguous once >1 oracle is loaded.
    pub arm_file: SourceFileId,
}

/// One role-function DEFINITION, as a narrative operand: which input file authored it and where
/// its name token sits (`28K` §2a Provenance — a bare [`Span`] is ambiguous once >1 file is
/// loaded). Pure `Copy` scalars, per `operands-are-pure-and-capped`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefinitionSite {
    /// Which input the definition was authored in.
    pub file: SourceFileId,
    /// The function-name span within that input.
    pub name: MintSpan,
}

/// Which channel-coverage failure formed a wall (`rul-only-oracle-bytes-ship` per-channel
/// coverage): the compound consumed a channel the walled participant could not cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelCoverage {
    /// The consumed channel whose coverage failed.
    pub channel: Channel,
}

/// A STATIC entry-consent denial rung (`27C` §3 `EntryDegrade`): every rung lands can't-say ⇒
/// guard/run; the tag drives the disclosure only. Mirrors `oracle::entry::EntryDegrade`'s static
/// arms (the dimension it names rides alongside as an interned display where relevant).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryDegradeTag {
    /// A crossed dimension the connection cannot mechanically effect (`hole-static-identity`).
    NoCapability,
    /// The dial is `--no-probe-escalation` (no oracle code shifts, ever).
    DialForbids,
    /// The default dial, but the executed function is not tolerance-vouched (`hole-unvouched-oracles`).
    Unvouched,
    /// A crossed dimension the wrapper chain WALLS (a missing `lend_map` key / unresolved target).
    TopDimension,
    /// The wrapper has no `__enter` form: its contexts are never entered.
    NoEntryForm,
}

/// A RUNTIME entry-failure class (`27C` §3): all land `Unknown`/run through the rc-partition;
/// named for the disclosure only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryFailureTag {
    /// Entry refused (`sudo -n` failure).
    Refused,
    /// Entry impossible (chroot target missing).
    Impossible,
    /// Missing deps in the view (rc 127).
    MissingDeps,
    /// An in-context decline (rc ≥ 2).
    InContextDecline,
}

/// Why a converged mutator's `Replace` demoted to `Run` (`survival::DemoteReason`, plus the
/// resolver/reaches conflict reclassifications). `inv-kfail`: a demote only ever fails toward run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoteTag {
    /// A running footprint-less mutator upstream (silence = wall).
    TotalWall,
    /// A footprint coordinate canonicalized to the SAME referent as the backing (a proven hit).
    Poisoned,
    /// A same-kind pair could not be canonicalized (the resolver ⊤'d / dangled / was absent).
    MayAlias,
    /// A resolver/reaches provider reclassification collision (`resolver-conflict`/`reaches-conflict`).
    Reclassified,
}

/// Why the leaf-exact render refused to elide/guard a licensed leaf (c8). `render-heredoc-refused`
/// is the sole v1 cause and STAYS a [`crate::diag::DiagCode`] (already push-surfaced) — this tag
/// exists only so the why-lens chain (d4) can reference the refusal if it needs to, without a
/// second render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderRefusalTag {
    /// The leaf's span covers a `<<` heredoc opener, so substituting it would strand the body.
    Heredoc,
}

/// The RESERVED cancellation-narrative marker (`27V` Lane A: cancellation is an r26 narrative kind
/// the type must not FORECLOSE). Uninhabited — [`CollapseKind::Cancellation`] cannot be constructed
/// at v1, but the variant holds the slot so no consumer's exhaustive match forecloses it. The r26
/// explanation-lane feeders (`26C` §5b) extend [`CollapseKind`] against this reservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reserved {}

/// The collapse CLASS and its operands (`AID-NEEDS:law-collapse-mints-narrative`: every
/// safety-narrowing mints a narrative carrying its OPERANDS). Deliberately NOT `#[non_exhaustive]`
/// (the `DiagCode` posture — every consumer is an internal workspace crate, so adding the r26
/// feeder variants breaks every match as a compiler checklist, never silently defaults).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollapseKind {
    /// Two establishers disagreed on one cell ⇒ ⊤ (`aid-why-disagreement-narration`). Carries the
    /// disagreeing operands (site + line + shown value), k-capped. Both the static value merge
    /// (`Reach`/value-plane, tier `Derived`) and the probe merge (`merge_observable`, tier
    /// `Measured`) mint this.
    FactMergeDisagreement {
        cell: SiteId,
        operands: Operands<ValueOperand>,
    },
    /// A verdict body declined (`rul-vouch-is-verdict-authoring`) ⇒ the site runs. Carries the
    /// declining `site` (aligning with its narrative siblings — `WallFormation`/`SubstitutionRefusal`/
    /// `Demotion` all key by site), the declining arm (with its file id — `tc-oracle-file-identity`)
    /// + the gate; `authored_reason` populated by the report-lane pairing keyed on `site` (`27W` §3).
    VerdictDecline {
        site: SiteId,
        arm: MintSpan,
        arm_file: SourceFileId,
        gate: DeclineGate,
        authored_reason: Option<AuthoredReason>,
    },
    /// A running mutator formed a wall (`rul-only-oracle-bytes-ship`) ⇒ downstream survival is
    /// constrained. Names the participant + the channel-coverage failure.
    WallFormation {
        participant: LeafId,
        channel: ChannelCoverage,
    },
    /// A substitution refused because a CONSUMED channel predicted ⊤ (`inv-probe-sourced-values`)
    /// ⇒ the leaf runs. Names the site + the ⊤ channel.
    SubstitutionRefusal { site: SiteId, top_channel: Channel },
    /// A STATIC context-entry consent denial (`27C` §3 `EntryDegrade`) ⇒ guard/run.
    EntryDenial { rung: EntryDegradeTag },
    /// A RUNTIME context-entry failure (`27C` §3) ⇒ guard/run.
    EntryFailure {
        site: SiteId,
        class: EntryFailureTag,
    },
    /// A converged mutator's `Replace` demoted to `Run` (`survival::WallVerdict::Demoted`).
    Demotion { site: SiteId, reason: DemoteTag },
    /// The leaf-exact render refused to elide/guard (c8; stays a [`crate::diag::DiagCode`]).
    RenderRefusal {
        site: SiteId,
        cause: RenderRefusalTag,
    },
    /// The validity fixpoint hit its iteration cap and DEGRADED TO ORIGIN (`26H` §4.4): every
    /// erasure discarded, the whole run recomputed with nothing erased. A safety-narrowing —
    /// elisions the fixpoint had licensed are withdrawn — so it narrates like any other
    /// (`law-collapse-mints-narrative`), even though the path is unreachable while erasure stays
    /// monotone. Carries how many rounds ran and how many erasures were thrown away.
    FixpointCapDegrade { rounds: u32, discarded: u32 },
    /// A role FAMILY's licenses were withheld because one unit's definition provably shadowed a
    /// DIFFERENT unit's, unblessed by an intervening `unset -f` (`28K` §1
    /// `rul-silent-shadowing-refuses`) ⇒ every site of that family runs. Carries BOTH definitions,
    /// because the whole repair is choosing between them: neither one alone names the conflict.
    RoleFamilyShadowed {
        prior: DefinitionSite,
        shadowing: DefinitionSite,
    },
    /// RESERVED (r26): the cancellation narrative. Unconstructable at v1 (holds the slot only).
    Cancellation(Reserved),
}

impl CollapseKind {
    /// A render-refusal collapse (the one c8 shape) — for the module doctest and d4's chain.
    #[must_use]
    pub fn render_refusal_heredoc(site: SiteId) -> Self {
        CollapseKind::RenderRefusal {
            site,
            cause: RenderRefusalTag::Heredoc,
        }
    }

    /// The class's greppable name — what a `[unnarrated: <class>]` disclosure prints
    /// (`28E:prop-unnarrated-is-visible`).
    ///
    /// Deliberately the VARIANT name rather than user prose: the disclosure exists so a maintainer
    /// can grep from the rendered output straight to the mint site, and it is a placeholder for
    /// words nobody has written yet, not words anybody chose.
    #[must_use]
    pub const fn class_name(&self) -> &'static str {
        match self {
            CollapseKind::FactMergeDisagreement { .. } => "FactMergeDisagreement",
            CollapseKind::VerdictDecline { .. } => "VerdictDecline",
            CollapseKind::WallFormation { .. } => "WallFormation",
            CollapseKind::SubstitutionRefusal { .. } => "SubstitutionRefusal",
            CollapseKind::EntryDenial { .. } => "EntryDenial",
            CollapseKind::EntryFailure { .. } => "EntryFailure",
            CollapseKind::Demotion { .. } => "Demotion",
            CollapseKind::RenderRefusal { .. } => "RenderRefusal",
            CollapseKind::FixpointCapDegrade { .. } => "FixpointCapDegrade",
            CollapseKind::RoleFamilyShadowed { .. } => "RoleFamilyShadowed",
            CollapseKind::Cancellation(_) => "Cancellation",
        }
    }
}

/// The narrative plane's version, which MUST move in lockstep with the whylog's record-stream
/// version (`dorc_plan::whylog::WHYLOG_V2_TAG`).
///
/// The coupling is what keeps `[unnarrated: <class>]` honest across a replay
/// (`28E:prop-unnarrated-is-visible`'s in-sitting caveat). The census is a claim about which
/// classes THIS binary's renders consume; asserted over a durable written by a binary whose plane
/// held different classes, it would be a confident statement about a run it cannot see. Keyed on
/// the durable's declared version, a mismatch withholds the census instead of lying about it.
pub const PLANE_VERSION: u32 = 2;

/// One decision-inert narrative record minted at a safety-narrowing collapse (`27V` Lane A). Pure
/// data (see module docs): a [`SpeechAct`] plus the [`CollapseKind`] carrying the collapse's
/// operands. SEALED decision-inert — no method yields a license-plane input (the `compile_fail`
/// doctest is the structural pin; this is the load-bearing law of the whole dispatch).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollapseNarrative {
    tier: SpeechAct,
    kind: CollapseKind,
}

impl CollapseNarrative {
    /// Mint a narrative at a collapse. The collapse CONSTRUCTOR demands this at the value level
    /// (`27V:rul-collapse-mints-narrative`, né -evidence); the caller supplies the tier from what
    /// the site knows
    /// (a probe merge → [`SpeechAct::Measured`], a vouch decline → [`SpeechAct::Vouched`], …).
    #[must_use]
    pub fn new(tier: SpeechAct, kind: CollapseKind) -> Self {
        Self { tier, kind }
    }

    /// The epistemic tier (display read — `mech-trust-tier-typed`). Rendered by d4 only.
    #[must_use]
    pub fn tier(&self) -> SpeechAct {
        self.tier
    }

    /// The collapse class + operands (display read for the why-lens / report surfaces).
    #[must_use]
    pub fn kind(&self) -> &CollapseKind {
        &self.kind
    }

    /// This narrative's class as a greppable name — the `<class>` of a `[unnarrated: <class>]`
    /// disclosure (`28E:prop-unnarrated-is-visible`).
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        self.kind.class_name()
    }

    /// Reconstruct a [`CollapseKind::VerdictDecline`] narrative with its `authored_reason` populated
    /// (`27W` §3 · d3): the report-lane ingestion resolves the decline CLASS + emitting-arm span
    /// AFTER the static decline mint (a dynamic-argv decline is classed only at runtime), so the
    /// narrative gains its authored reason WITHOUT field mutation (immutable records — the
    /// tc-authored-reason-immutability ruling: a narrow reconstructor, never a mutable setter). A
    /// A non-decline narrative, or one already carrying a reason, is returned unchanged (idempotent —
    /// the tier-2 static class already populated it, so a tier-3 runtime echo never overwrites).
    #[must_use]
    pub fn with_authored_reason(self, reason: AuthoredReason) -> Self {
        match self.kind {
            CollapseKind::VerdictDecline {
                site,
                arm,
                arm_file,
                gate,
                authored_reason: None,
            } => Self {
                tier: self.tier,
                kind: CollapseKind::VerdictDecline {
                    site,
                    arm,
                    arm_file,
                    gate,
                    authored_reason: Some(reason),
                },
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{ByVouch, BytePos, LeafId, Rung};

    /// A fixed non-zero oracle-file id for the tests (the id disambiguates >1 loaded oracle; a
    /// single-file test just needs a stable value).
    const F: SourceFileId = SourceFileId(1);

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(BytePos(lo), BytePos(hi))
    }

    fn site(leaf: u32) -> SiteId {
        SiteId {
            leaf: LeafId(leaf),
            member: None,
        }
    }

    #[test]
    fn a_vouch_informs_a_tier_but_is_never_retained() {
        // two-plane-aid-law: a license flows INTO the aid plane (informing the tier) and never
        // back. `from_vouch` reads only that the vouch exists — the vouch is consumed by-ref and
        // dropped; the narrative holds a plain `SpeechAct`, not a license.
        let vouch = ByVouch::vouched(7u32, Rung::Both);
        let tier = SpeechAct::from_vouch(&vouch);
        assert_eq!(tier, SpeechAct::Vouched);
        let ev = CollapseNarrative::new(
            tier,
            CollapseKind::VerdictDecline {
                site: site(1),
                arm: MintSpan(span(4, 9)),
                arm_file: F,
                gate: DeclineGate::Return,
                authored_reason: None,
            },
        );
        assert_eq!(ev.tier(), SpeechAct::Vouched);
    }

    #[test]
    fn operands_cap_with_a_truncation_marker() {
        // The JOIN_PARENT_CAP precedent: operands past the cap are dropped with an "…and N more"
        // count, never a silently-lossy truncate.
        let ops: Vec<ValueOperand> = (0..u32::try_from(NARRATIVE_OPERAND_CAP + 3).unwrap())
            .map(|i| ValueOperand {
                site: site(i),
                minting_line: Some(span(i, i + 1)),
                shown: None,
            })
            .collect();
        let capped = Operands::capped(ops);
        assert_eq!(
            capped.kept().len(),
            NARRATIVE_OPERAND_CAP,
            "retained ops are capped"
        );
        assert_eq!(capped.truncated(), 3, "the remainder is '…and 3 more'");
    }

    #[test]
    fn operand_merge_recaps_and_carries_both_truncations() {
        // A deep merge of two already-truncated operand lists stays bounded and sums the drops
        // (no unbounded growth through nested collapses).
        let mk = |base: u32, n: u32| -> Operands<ValueOperand> {
            Operands::capped(
                (0..n)
                    .map(|i| ValueOperand {
                        site: site(base + i),
                        minting_line: None,
                        shown: None,
                    })
                    .collect(),
            )
        };
        let a = mk(0, u32::try_from(NARRATIVE_OPERAND_CAP + 2).unwrap()); // truncated 2
        let b = mk(100, u32::try_from(NARRATIVE_OPERAND_CAP + 1).unwrap()); // truncated 1
        let merged = a.merge(b);
        assert_eq!(
            merged.kept().len(),
            NARRATIVE_OPERAND_CAP,
            "merged union stays capped"
        );
        // 2 (a) + 1 (b) + the union overflow past the cap (a.kept CAP + b.kept CAP − CAP = CAP).
        assert!(
            merged.truncated() >= 3,
            "both source truncations are carried, plus union overflow"
        );
    }

    #[test]
    fn cancellation_is_reserved_unconstructable_but_matchable() {
        // The r26 reservation: `Cancellation(Reserved)` holds the slot (no consumer forecloses it)
        // yet cannot be constructed at v1 (Reserved is uninhabited). A match still handles it.
        let ev = CollapseNarrative::new(
            SpeechAct::Measured,
            CollapseKind::FactMergeDisagreement {
                cell: site(1),
                operands: Operands::default(),
            },
        );
        if let CollapseKind::Cancellation(r) = ev.kind() {
            match *r {}
        }
    }

    #[test]
    fn decline_class_from_token_is_the_closed_v1_set() {
        assert_eq!(
            DeclineClass::from_token("unsound"),
            Some(DeclineClass::Unsound)
        );
        assert_eq!(
            DeclineClass::from_token("hazard"),
            Some(DeclineClass::Hazard)
        );
        assert_eq!(
            DeclineClass::from_token("bogus"),
            None,
            "unknown ⇒ degrade-generic"
        );
        assert_eq!(DeclineClass::from_token(""), None);
    }

    #[test]
    fn with_authored_reason_populates_a_decline_without_mutation_and_is_idempotent() {
        // d3 / `27W` §3: a narrow reconstructor populates the reason without mutation; idempotent —
        // a tier-2 static reason is not overwritten by a tier-3 runtime echo.
        let reason = AuthoredReason {
            class: DeclineClass::Unsound,
            arm: MintSpan(span(4, 9)),
            arm_file: F,
        };
        let ev = CollapseNarrative::new(
            SpeechAct::Vouched,
            CollapseKind::VerdictDecline {
                site: site(3),
                arm: MintSpan(span(0, 3)),
                arm_file: F,
                gate: DeclineGate::Return,
                authored_reason: None,
            },
        );
        let populated = ev.with_authored_reason(reason);
        assert!(
            matches!(
                populated.kind(),
                CollapseKind::VerdictDecline { authored_reason: Some(r), .. } if r.class == DeclineClass::Unsound
            ),
            "the reconstructor populates the class"
        );
        let other = AuthoredReason {
            class: DeclineClass::Hazard,
            arm: MintSpan(span(1, 2)),
            arm_file: F,
        };
        let again = populated.with_authored_reason(other);
        assert!(
            matches!(
                again.kind(),
                CollapseKind::VerdictDecline { authored_reason: Some(r), .. } if r.class == DeclineClass::Unsound
            ),
            "a populated reason is never overwritten (tier-2 wins over a tier-3 echo)"
        );
        let refusal = CollapseNarrative::new(
            SpeechAct::Derived,
            CollapseKind::render_refusal_heredoc(site(2)),
        );
        assert_eq!(
            refusal.clone().with_authored_reason(reason),
            refusal,
            "non-decline unchanged"
        );
    }

    #[test]
    fn narrative_eq_is_structural_here_carrier_excludes_it_elsewhere() {
        // CollapseNarrative derives Eq (tests compare it); the Reach-style EXCLUSION is a CARRIER
        // property proven where a narrative rides a fixpoint value (analysis::effect), not here.
        let a = CollapseNarrative::new(
            SpeechAct::Derived,
            CollapseKind::render_refusal_heredoc(site(2)),
        );
        let b = CollapseNarrative::new(
            SpeechAct::Derived,
            CollapseKind::render_refusal_heredoc(site(2)),
        );
        assert_eq!(a, b);
        let c = CollapseNarrative::new(
            SpeechAct::Ran,
            CollapseKind::render_refusal_heredoc(site(2)),
        );
        assert_ne!(a, c, "a different tier is a different record");
    }
}
