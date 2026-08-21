//! `dorc-plan` — the elision path: decide, per command, run-or-skip, behind the
//! orientation locks of `Research/notes/165`.
//!
//! The catastrophic bug this crate is built to make *unrepresentable* is a wrong
//! skip: eliding a command that actually needed to run (`kFAIL-perform`). Three
//! locks, hardest first:
//!
//! * **`PhasedVerdict<P>`** (note 165 L1) — a host verdict carries its phase in
//!   the type, so a probe verdict cannot be silently consumed as an apply verdict,
//!   and [`Bias`] forces the `Unknown`-fold per phase. No code path folds
//!   `Unknown` to a skip.
//! * **[`ReplaceLicense`]** (note 165 L2) — the witness for the one irreversible verb
//!   (*elide*). Its fields are private, so the only way to obtain one is
//!   [`ReplaceLicense::prove_replaceable`]; a plan emitter takes a `ReplaceLicense`, never
//!   a `bool`, so "skip" cannot be spelled without the proof.
//! * **`inv-must-may` + the freshness gate**, enforced inside `prove_replaceable` and by its
//!   caller: only a [`Grade::Must`] fact the host probe found `Converged`, whose site the
//!   settlement proved FRESH (no mutation that may actually execute reaches it — `crate::world`),
//!   may be elided.
//!
//! Determinism (`inv-determinism`): a pure function of its inputs; the host
//! verdict is injected (the real host / `hostsim` is a later seam).
//!
//! # The vouch demand, pinned (`297` phase-zero item 4)
//!
//! `core::claim` proves no laundering path exists between tiers. What only THIS crate can pin
//! is that its mints still *ask* for the tier: the regression that algebra cannot see is a
//! signature quietly weakening to take a bare [`VerdictVouch`], or an `Option` that a `None`
//! caller can satisfy where a vouch was meant. Naming the whole type pins it, with no values to
//! construct:
//!
//! ```
//! use dorc_core::claim::ByVouch;
//! use dorc_plan::{PhasedVerdict, Probe, ReplaceLicense, VerdictVouch};
//!
//! let _pinned: fn(
//!     dorc_core::FactKey,
//!     dorc_core::Grade,
//!     PhasedVerdict<Probe>,
//!     dorc_analysis::lattice::May<dorc_analysis::lattice::Powerset<dorc_core::Channel>>,
//!     dorc_core::Predicted<dorc_core::Rc>,
//!     Option<ByVouch<VerdictVouch>>,
//! ) -> Option<ReplaceLicense> = ReplaceLicense::prove_replaceable::<Probe>;
//! ```
//!
//! The aggregate paths (member-loop, inline-call) carry the same demand through a private,
//! non-empty, identity-matched proof. Its unconstructibility from outside is the property; the
//! failure it guards is narrow but real — a future builder exposing the proof or its mint "just
//! for a test", which would let a caller assemble an aggregate erasure without one reached vouch
//! per erased establish.
//!
//! ```compile_fail
//! let _ = dorc_plan::AllEstablishesVouched::mint(&[], &dorc_plan::Vouches::default());
//! ```

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; this crate-root expect
// ratchets away during the rebuild (an unfulfilled `expect` warns, so it
// self-removes as the seeded layer is replaced). It never relaxes the policy
// for new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::arithmetic_side_effects,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use core::marker::PhantomData;
use std::collections::{BTreeMap, BTreeSet};

use dorc_aid::diag::Diag;
use dorc_aid::narrative::{AuthoredReason, MintSpan, RenderRefusalTag};
use dorc_aid::{Carrier, CollapseKind, CollapseNarrative, SpeechAct};
use dorc_analysis::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use dorc_analysis::effect::{FactKey, InlineSite, SkipClass};
use dorc_analysis::lattice::{May, Powerset};
use dorc_analysis::value::{ValueFlow, ValueOf};
use dorc_core::{
    AstId, ByVouch, Channel, EntityRef, Grade, Interner, KindId, Observable, Predicted, Rc, Rung,
    SourceFileId, Symbol, Verdict,
};
use dorc_oracle::touches::DISTURBS_SUFFIX;
use dorc_oracle::verdict::VERDICT_SUFFIX;
use dorc_syntax::ast::{Ast, NodeKind, RedirOp, RedirTarget};

mod fold;
pub use fold::{AbstractRc, FoldResult};

pub mod erasability;

pub mod erase;

pub mod records;

pub mod whylog;

/// The pure, adapter-facing plan invocation boundary (`io-at-edges-only`).
pub mod invocation;

pub mod render;

/// The per-run PATH shim for `dorc-sh` (`274` §5): the pure model — host-independent shipped text
/// (`shim_script`), run-id-derived naming (`shim_dir_name`, no mktemp randomness), and the failure
/// lattice (`classify_shim_rc` / `smoke_degrades_session` — every shim/exec failure drains to the ≥2
/// sink ⇒ run; a failed preamble smoke degrades the session shimless). MODELS only — materialization
/// is the cli/hostsim I/O edge and probe-shipping is task-14-gated; the corpus stays byte-stable.
pub mod shim;

pub mod rederive;

pub mod certifier_trip;

pub mod survival;
use survival::{AggregateEstablish, AggregateEstablishes};
pub use survival::{
    AggregateMemberSurvival, AggregateSurvivalWitness, Backing, CanonicalCoord, Crossing,
    DisjointOutcome, DisjointnessProof, EntityCoord, Footprint, FootprintOrigin, MayAliasReason,
    ReachExpansion, Resolution, Resolutions, SurvivalAttribution, SurvivalWitness,
    TrustedFootprints, disjoint,
};

pub mod world;
pub use world::{NoExecutionLedger, ReachingWalls, WallId, WallPolicy};

pub mod settle;
use erase::DeadBranchProof;
use settle::SurvivalAccount;
pub use settle::{
    RoundClassification, RoundModel, SettleInputs, Settlement, settle_effective_world,
};
use world::{EffectiveAct, Freshness, NoMutationProof};

pub mod spine;
pub use spine::{Authorised, PlanAuthority, PlanPlane, Spine, project_plan};

pub mod region;

// ===========================================================================
// Phase markers + the Unknown-fold bias (note 165 L1)
// ===========================================================================

/// Type-level marker for the **probe** phase — distinct from the runtime
/// [`dorc_core::Phase`] enum. Uninhabited: it exists only to parameterise a type,
/// never to be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Probe {}

/// Type-level marker for the **apply** phase. See [`Probe`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Apply {}

/// The definite action a verdict folds to once `Unknown` is resolved per phase.
/// A plan may elide a command only when it holds a [`Resolved::Replaceable`], and
/// `Replaceable` is reachable ONLY from a definite [`Verdict::Converged`] — never
/// from `Unknown` (that is the wrong-skip this crate forbids).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolved {
    /// The command's effect is already established → it may be elided.
    Replaceable,
    /// The command must run (diverged, or the conservative fold of unknown).
    Run,
}

/// The phase-keyed safe default for an `Unknown` verdict (welded `kFAIL`). No
/// implementation may return [`Resolved::Replaceable`] — folding `Unknown` to a skip
/// is the catastrophic error (note 165). Keeping the rule in one trait, one impl
/// per phase, means it is reviewed in exactly one place instead of re-derived at
/// every `match` on a verdict.
pub trait Bias {
    /// What an `Unknown` verdict folds to in this phase. Must never be `Replaceable`.
    fn on_unknown() -> Resolved;
}

impl Bias for Probe {
    /// Probe phase (`kFAIL-withhold`): an `Unknown` means the read-only check could
    /// not confirm convergence → treat as not-established → [`Resolved::Run`].
    fn on_unknown() -> Resolved {
        Resolved::Run
    }
}

impl Bias for Apply {
    /// Apply phase (`kFAIL-perform`): never skip a needed mutation → an `Unknown`
    /// verdict [`Resolved::Run`]s.
    fn on_unknown() -> Resolved {
        Resolved::Run
    }
}

/// A host convergence [`Verdict`], tagged with the phase that produced it. The
/// phase tag is the lock: a `PhasedVerdict<Probe>` cannot be passed where a
/// `PhasedVerdict<Apply>` is wanted, and [`resolve`](PhasedVerdict::resolve)
/// folds `Unknown` through the phase's [`Bias`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhasedVerdict<P: Bias> {
    raw: Verdict,
    _phase: PhantomData<P>,
}

impl<P: Bias> PhasedVerdict<P> {
    /// Tag a raw host verdict with this phase.
    #[must_use]
    pub fn new(raw: Verdict) -> Self {
        Self {
            raw,
            _phase: PhantomData,
        }
    }

    /// Fold to a definite action; `Unknown` uses this phase's [`Bias`]. The only
    /// route to [`Resolved::Replaceable`] is a definite [`Verdict::Converged`].
    #[must_use]
    pub fn resolve(self) -> Resolved {
        match self.raw {
            Verdict::Converged => Resolved::Replaceable,
            Verdict::Diverged => Resolved::Run,
            Verdict::Unknown => P::on_unknown(),
        }
    }

    /// The underlying three-valued verdict (for display / provenance).
    #[must_use]
    pub fn raw(self) -> Verdict {
        self.raw
    }
}

// ===========================================================================
// The observable-consumption gate (16F / note 16J)
// ===========================================================================
//
// The un-collapsed consumption fact — which unvouched output observables a leaf's
// context consumes ([`Channel`]) — is computed by the ENGINE and emitted on the
// `Cfg` ([`dorc_analysis::cfg::Cfg::consumed_observables`]); `plan` collapses it
// (`inv-superposition`, note 16J). The `true`-stub defaults every observable
// (effect→none, status→0, stdout/stderr→empty); a default is sound only if
// *vouched* — effect by convergence (the forward gate), status by the `establishes`
// contract (free), stdout/stderr by NOTHING — so a consumed stdout/stderr is the
// one thing that forbids the stub. Per `inv-must-may`, that fact is read in the
// `May` (over-approximate) orientation, which can only ever *block* a license.

// ===========================================================================
// The replace witness (note 165 L2; "replace" — 16F)
// ===========================================================================

/// Which of the two value-preserving substitution paths licensed a replacement
/// (task-D2): a convergence-elision of an already-established mutator, or a
/// value-preserving substitution of a read-only Query guard. The two have genuinely
/// different preconditions (a mutator needs `Converged` + `Must`; a Query needs only
/// a valid, probe-sourced rc — it has no mutation to be already-done), so the witness
/// records which one it proved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseVia {
    /// Convergence-elision: an `EstablishProbeAmbient` mutator whose effect the host
    /// reports already holds (`Converged`), oracle-declared `Must`, ambient.
    ConvergedEstablish,
    /// Query-guard substitution (202 §2 / task-D2): a read-only guard with a valid,
    /// probe-sourced rc, replaced by the value-preserving [`StandIn`] reproducing that
    /// rc. Mutates nothing, so convergence does not gate it — only rule-query-validity
    /// + a known rc + the consumption gates do.
    QueryGuard,
    /// In-loop Members convergence-elision (task-L2 item-3, `209` brk-1(b)): an in-loop
    /// `EstablishMembers` body leaf whose EVERY member is Converged, that is self-reached
    /// (only its own per-member establishes reach it), and that passes the consumption
    /// gates. The all-or-nothing in-loop license — it lifts the in-loop render-floor for
    /// exactly this shape; any non-converged member, any non-self writer, or a consumed ⊤
    /// status refuses it (the whole leaf runs).
    MembersLoop,
    /// Inlined function-CALL convergence-elision (arch-2, brk-2, `i-3`): an
    /// [`SkipClass::InlineCall`] whose EVERY effect-bearing spliced body leaf licenses elision
    /// — each body Establish is `EstablishProbeAmbient` + Converged (a body Kill/Opaque/⊤/written
    /// establish, or a non-Converged one, blocks the WHOLE call), Queries pass their gates, and
    /// the CALL site's own consumed channels are reproduced. The all-or-nothing CALL license
    /// (the Members precedent): the CALL leaf's span substitutes to `true`; one non-licensing
    /// body leaf ⇒ the call RUNS (the real body executes). No partial-body render (`i-3`).
    InlineCall,
    /// Shared-region convergence-elision (`plans/30L` §5): ONE authored region inside a function
    /// body, whose EVERY statically possible invocation instance independently proved the same
    /// observable-preserving replacement. The license's witness spans every contributing instance's
    /// establish (`30L:pin-shared-witness-spans-instances`) — a per-call witness never stands in
    /// for it — and the edit lands once, at the authored definition
    /// (`30L:rul-edit-authored-definition-once`).
    SharedRegion,
}

/// Why a replacement was licensed — the audit trail a plan UI greys-out as the "why"
/// (note 165 L2). Readable, but only ever constructed inside
/// [`ReplaceLicense::prove_replaceable`], so every field reflects a checked condition.
#[derive(Debug, Clone)]
pub struct Derivation {
    /// The fact whose established-ness (or queried-ness) licenses the substitution.
    pub fact: FactKey,
    /// Which substitution path was proved ([`LicenseVia`]) — convergence-elision or a
    /// Query-guard value-preserving substitution.
    pub via: LicenseVia,
    /// `analysis` classified this command [`SkipClass::EstablishProbeAmbient`]: no
    /// upstream same-run mutation reaches it (the W5 ambient gate, note 162 O-1).
    /// Always `true` for [`LicenseVia::ConvergedEstablish`]; `false` for a Query guard
    /// (a Query has no ambient-establish gate — rule-query-validity gates it instead).
    pub ambient: bool,
    /// The fact is oracle-declared [`Grade::Must`] (a mined `May` never licenses —
    /// `inv-must-may`). [`Grade::Must`] for a converged-establish; for a Query guard
    /// this records the guard's grade (the guard's elision is not a mutation-elision,
    /// so `inv-must-may`'s mutation-licensing rule does not bind it).
    pub grade: Grade,
    /// The host probe verdict: [`Verdict::Converged`] for a converged-establish; for a
    /// Query guard, the guard's observed Effect verdict (`holds`/`absent` — the guard
    /// is substituted regardless, since it mutates nothing).
    pub verdict: Verdict,
    /// The FULL granted witness (arch-1 `vp-17`/`vp-18`, the uncapped license-tier of the
    /// two-tier receipts budget — [`dorc_core::Witness`]): the origin receipts that justified
    /// this license, stored EXACTLY (no k-cap, unlike a value join's [`dorc_core::Parents`]).
    ///
    /// THE WELD: this is pure OUTPUT provenance — `build_plan` computes it from the site the
    /// license already keys on, AFTER the mint decision, so it can never influence the
    /// decision. It is on the EXEMPT plane (`Exempt::ReceiptId`): the `erasability` gate omits
    /// it from the identity comparison, and the gate's run-B (adversarial arena) proves a
    /// different witness does not perturb any decision. Empty for a license minted without an
    /// arena (the tests' throwaway path); populated with the establish site's `BookSource`
    /// origin on the real `build_plan` path.
    pub witness: dorc_core::Witness,
    /// The SURVIVAL attribution (Stage 2 / TC-3), if this elision crossed ≥1 running wall under
    /// `--risk-faultless-skips`. `None` for every ordinary elision (pre-wall, or flag-off); `Some`
    /// names which walls it crossed + whose footprint licensed each. Attached post-mint by the
    /// effective freshness decision ([`ReplaceLicense::with_survival`]); read ONLY by the why-lens render (never the
    /// artifact — rec-1). NOT a proof of adequacy (converged≠no-op stays the vouch's) — see
    /// [`survival`].
    pub survival: Option<SurvivalAttribution>,
    /// The licensing vouch's DEFINING span + oracle-file id (C7 `27V:mech-minting-line-threading`),
    /// for the survival attribution's `file:line`. `Some` only on the [`LicenseVia::ConvergedEstablish`]
    /// (elide-weld) path, which consumes a [`ByVouch<VerdictVouch>`]; `None` for Query/loop/call
    /// elisions (no vouch consumed) and for vouches minted without a threaded span. Pure OUTPUT
    /// provenance — EXEMPT from the erasability identity comparison (like [`witness`]/[`survival`]):
    /// a vouch informs a license and never becomes a fact (TC-tier-3), and this reaches only the
    /// why-lens render, never the byte-floored artifact.
    pub vouch_span: Option<(dorc_core::Span, SourceFileId)>,
    /// Ordered attribution for every establish erased by an aggregate replacement.
    pub establish_vouches: Vec<EstablishVouchReceipt>,
    /// The REPORTED half of the chain, beside `vouch_span`'s VOUCHED half: which record measured
    /// this license's fact, when, with what tool-rc, and which funcdef reported it
    /// ([`ProbeAttribution`]). Attached post-mint by `build_plan`, exactly like `witness` —
    /// output-only provenance the erasability gate exempts. `None` for a license minted without a
    /// probe-attribution map (the tests' throwaway path, the flag-off/kill-unaware entry).
    pub probe: Option<ProbeAttribution>,
}

/// Decision-inert attribution for the ONE probe record that reported a licensing fact — everything
/// a why-chain's REPORTED row states beyond the payload: who reported it, when, and what the tool
/// exited with (`27V` Lane A · `AID-NEEDS:law-trust-tier-is-syntax`).
///
/// `Some` only when EXACTLY ONE record measured the fact. Two records are two events with no single
/// speaker, instant, or rc, so the honest answer is absence rather than a silent first-wins pick;
/// the joined [`dorc_core::ProvId`] on [`ProbeAttribution`] still carries both as receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportedObservation {
    /// The record's arrival ordinal + the instant the controller observed it (`None` with no clock).
    pub stamp: dorc_core::ProbeStamp,
    /// The tool-rc the record carried. Raw carriage, NEVER a decision input: an Establish site's
    /// rc is the PROBE command's, not the mutator's (`ProbeSiteKind`'s firewall), and the fold
    /// already decided separately whether it was admissible.
    pub tool_rc: Rc,
    /// The defining span (+ oracle file) of the funcdef whose body produced this observation —
    /// `ProbePredict::defining_span` carried through. `None` when the shipped body had no single
    /// defining funcdef (entry-composed, connected pipes).
    pub predict_span: Option<(dorc_core::Span, SourceFileId)>,
}

/// The probe-side provenance for one fact: its receipt origin plus, when a single record reported
/// it, that record's [`ReportedObservation`]. Built at the cli edge (where the arena and the
/// records live) and handed to [`build_plan_walled`], which attaches it to a licensing
/// disposition's [`Derivation`] AFTER the mint.
///
/// THE WELD: pure OUTPUT provenance. Both halves are EXEMPT from the erasability identity plane —
/// `origin` under `Exempt::ReceiptId`, the observation's instant under `Exempt::Timing` — so a
/// decision can never read either.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeAttribution {
    /// The `ProbeResult` origin receipt (a join, when several records measured the fact).
    pub origin: dorc_core::ProvId,
    /// The single reporting record's observation, when exactly one record reported this fact.
    pub reported: Option<ReportedObservation>,
}

/// Decision-inert attribution retained after an aggregate mutation-erasure mint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EstablishVouchReceipt {
    pub site: CfgNodeId,
    pub fact: FactKey,
    pub defining_span: Option<(dorc_core::Span, SourceFileId)>,
    pub probe: Option<ProbeAttribution>,
}

#[derive(Debug, Clone)]
struct EstablishVouch {
    site: CfgNodeId,
    fact: FactKey,
    vouch: ByVouch<VerdictVouch>,
}

#[derive(Debug, Clone)]
struct AllEstablishesVouched {
    head: EstablishVouch,
    tail: Vec<EstablishVouch>,
}

#[derive(Debug, Clone, Copy)]
struct ReadSubstitutionProof {
    fact: FactKey,
    status: Predicted<Rc>,
}

impl ReadSubstitutionProof {
    fn mint(
        sites: &[InlineSite],
        observe: &(impl Fn(FactKey) -> Observable + ?Sized),
    ) -> Option<Self> {
        let [site] = sites else { return None };
        let SkipClass::QueryResolvable { fact, valid: true } = site.class else {
            return None;
        };
        let status = observe(fact).status;
        (!matches!(status, Predicted::Top)).then_some(Self { fact, status })
    }
}

impl AllEstablishesVouched {
    fn mint(expected: &AggregateEstablishes, vouches: &Vouches) -> Option<Self> {
        if expected
            .iter()
            .any(|entry| vouches.is_duplicate(entry.site(), entry.fact()))
        {
            return None;
        }
        let involved: BTreeSet<CfgNodeId> = expected.iter().map(AggregateEstablish::site).collect();
        let supplied: Vec<_> = vouches
            .ordered_keys()
            .into_iter()
            .filter(|(site, _)| involved.contains(site))
            .collect();
        let identities: Vec<_> = expected
            .iter()
            .map(|entry| (entry.site(), entry.fact()))
            .collect();
        if supplied != identities {
            return None;
        }
        let mut entries = expected.iter().map(|entry| {
            Some(EstablishVouch {
                site: entry.site(),
                fact: entry.fact(),
                vouch: vouches.get(entry.site(), entry.fact())?.clone(),
            })
        });
        Some(Self {
            head: entries.next()??,
            tail: entries.collect::<Option<Vec<_>>>()?,
        })
    }

    fn representative(&self) -> FactKey {
        self.head.fact
    }

    fn into_receipts(self) -> Vec<EstablishVouchReceipt> {
        std::iter::once(self.head)
            .chain(self.tail)
            .map(|entry| EstablishVouchReceipt {
                site: entry.site,
                fact: entry.fact,
                defining_span: entry.vouch.vouch().defining_span(),
                probe: None,
            })
            .collect()
    }
}

/// The witness authorising the one irreversible verb — *elide a command*. Its
/// fields are private, so the ONLY way to obtain one is
/// [`prove_replaceable`](ReplaceLicense::prove_replaceable); a plan emitter accepts a
/// `ReplaceLicense`, never a `bool`, so a skip cannot be spelled without the proof
/// (note 165 L2). Carries its [`Derivation`] for provenance.
#[derive(Debug, Clone)]
pub struct ReplaceLicense {
    fact: FactKey,
    derivation: Derivation,
    /// WHOSE utterance this replacement rests on (`28M` §8 — the monologue, typed). Every mint
    /// stamps it, so "which author is this license speaking for" has an answer that is read off the
    /// value rather than re-derived from three unrelated mechanisms agreeing.
    custody: dorc_core::LicenseCustody,
}

impl ReplaceLicense {
    /// Mint a license iff EVERY condition holds; otherwise `None` — the
    /// conservative *run-it* direction (note 165 L2 / `inv-must-may` / the ambient
    /// gate):
    ///
    /// 1. the caller established the fact is FRESH — no mutation that may actually execute reaches
    ///    the site (`Freshness`). It is the CALLER's conjunct rather than a class
    ///    test on purpose: the origin classification answers which check could ship, and reading
    ///    its per-cell ambient-ness as apply-time freshness is exactly the split `30K` closed;
    /// 2. the fact is [`Grade::Must`] (oracle-declared; a `May` hint never licenses);
    /// 3. the probe verdict folds to [`Resolved::Replaceable`] — a definite
    ///    `Converged`; `Diverged` and (via [`Bias`]) `Unknown` do not.
    /// 4. no UNVOUCHED observable is consumed downstream. The consumption is the
    ///    engine's un-collapsed `May<Powerset<Channel>>` fact (`inv-superposition`,
    ///    note 16J); per `inv-must-may` a `May` value can only block. Branch-consumed
    ///    status comes in two engine variants by render-expressibility (`19D` / 19C
    ///    strain-D / `206` §3); both gate a *different* command's reachability, so a
    ///    *fabricated* rc-0 stand-in would destroy that decision. The unvouched set:
    ///    * `Stdout`/`Stderr` — the stub defaults them to empty, vouched by nothing
    ///      (16F §3); a consumed one ⇒ run (no in-spike bridge). A declared rc does
    ///      NOT vouch *output content*, so these block regardless of `observed_rc`.
    ///    * `StatusIterated` (a `while`/`until` condition) — blocks **unconditionally**.
    ///      The condition is re-evaluated per pass, so its consumed value is a SEQUENCE no
    ///      single predicted rc reproduces, and a constant-substituted loop condition is an
    ///      infinite/zero-iteration disaster (arch-1, note 214 — the honest successor to the
    ///      retired render-floor, keyed on iteration not render capability).
    ///    * `StatusRelaxable` (a `&&`/`||` left operand, an errexit-region command, a
    ///      `$?`-reader's predecessor, or — since arch-1 — an `if`/`elif` guard) — blocks
    ///      **only when the rc is ⊤** (`status == Predicted::Top`): then the stand-in would
    ///      default to
    ///      `true` (rc 0), a fabricated success that suppresses a `|| fallback` (the
    ///      `kFAIL-perform` under-execute — the round-19 adversarial trace). A
    ///      PROBE-SOURCED rc relaxes it (`status == Predicted::Value(N)` ⇒ the stand-in is
    ///      `StandIn::from_rc(N)`, reproducing the exact status, so the branch decides
    ///      identically — `inv-probe-sourced-values`; an Establish site's status is
    ///      withheld to ⊤ at intake, so only a Query's own measured rc ever arrives here).
    ///      The render CAN express this (operand+operator on one line; the fold +
    ///      omit-safety gate handle it).
    ///    * Errexit (`set -e`)-consumed status is NOT special-cased (honored
    ///      round-20 / 205 §2): the cfg pass marks errexit-region commands (and `$?`
    ///      readers' predecessors) `StatusRelaxable`-consumed, so they ride the same
    ///      measured-rc-or-block rule above. Under fork-mutator-rc a mutator's rc is
    ///      always ⊤ ⇒ converged mutators under `set -e` run (the 206 §2 headline cost).
    ///
    /// Generic over the phase `P` (`inv-superposition`): the engine never bakes a
    /// phase; the caller argues it. `build_plan` passes the verdict's own provenance
    /// (`Probe`) and the leaf's observed rc.
    ///
    /// task-D2 dispatch: this is the convergence-elision path; a read-only Query guard's
    /// substitution takes [`prove_query_replaceable`](ReplaceLicense::prove_query_replaceable)
    /// instead, and the two are separate entry points rather than one class match — a Query's
    /// precondition is its own measured rc's validity, not a mutation's freshness.
    ///
    /// **The elide-weld (24D §3 / rul24-vouch-is-verdict-authoring).** The full-skip (elide)
    /// license now DEMANDS the reached `vouch` — the SAME [`ByVouch<VerdictVouch>`] the guard mint
    /// consumes (TC-tier-2). It arrives as an `Option` (the caller's [`Vouches`] lookup); the
    /// `EstablishProbeAmbient` arm consumes it BY VALUE, and that consumption IS the tier check — a
    /// [`core::claim::ByObservation`] or [`core::claim::BySilence`] cannot inhabit
    /// `Option<ByVouch<_>>`, so a measurement alone can never license a mutation-skip
    /// (proviso-read-erasure, `24A §1c`: the fact tier licenses read-reproduction, NEVER a
    /// mutation-skip). **No vouch ⇒ `None` ⇒ run** — the safe direction (kFAIL-perform), closing
    /// the HEAD vouchless-elide gap. The Query-guard arm IGNORES `vouch`: a read-only substitution
    /// IS read-reproduction, licensed by the fact tier, never demanding a vouch.
    #[must_use]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "by-value verdict/consumed/vouch keeps this minting API a clean owned-args boundary; the vouch is CONSUMED as the tier check (24D §3), never needless"
    )]
    pub fn prove_replaceable<P: Bias>(
        fact: FactKey,
        grade: Grade,
        verdict: PhasedVerdict<P>,
        consumed: May<Powerset<Channel>>,
        status: Predicted<Rc>,
        vouch: Option<ByVouch<VerdictVouch>>,
    ) -> Option<ReplaceLicense> {
        // The elide-weld (TC-tier-2): consume the reached vouch BY VALUE — no vouch ⇒ run.
        // A `ByObservation`/`BySilence` cannot inhabit this `Option`, so a converged
        // measurement alone no longer elides (the vouchless-elide gap, closed).
        let vouch: ByVouch<VerdictVouch> = vouch?;
        // C7: read the vouch's defining span (display-only) BEFORE it drops, for the
        // survival render's `file:line` (a vouch informs, never becomes a fact — TC-tier-3).
        let vouch_span = vouch.vouch().defining_span();
        // Read off the CONSUMED vouch, never passed beside it (`28M` §8).
        let custody = dorc_core::LicenseCustody::Vouched(vouch.vouch().custody());
        if grade != Grade::Must {
            return None;
        }
        if verdict.resolve() != Resolved::Replaceable {
            return None;
        }
        consumption_ok(&consumed, status).then_some(ReplaceLicense {
            custody,
            fact,
            derivation: Derivation {
                fact,
                via: LicenseVia::ConvergedEstablish,
                ambient: true,
                grade,
                verdict: Verdict::Converged,
                // Empty at mint (the minter has no arena); `build_plan` attaches the
                // real witness post-mint via `with_witness` (arch-1, output-only/exempt).
                witness: dorc_core::Witness::empty(),
                survival: None,
                vouch_span,
                establish_vouches: Vec::new(),
                probe: None,
            },
        })
    }

    /// Mint a license for a read-only **Query guard**'s value-preserving substitution
    /// (202 §2 / task-D2 — Build 5). A Query mutates nothing, so convergence does NOT
    /// gate it (unlike a converged-establish): the guard is replaced by the
    /// [`StandIn`] reproducing its PROBED rc whenever
    ///
    /// 1. the guard is **valid** (rule-query-validity, 205 §2: no mutator/opaque
    ///    reached it from entry — else its resting rc is stale ⇒ run for real); AND
    /// 2. its rc is a **known** probe-sourced `Predicted::Value` (not ⊤) — the
    ///    stand-in needs a concrete rc to reproduce (`inv-probe-sourced-values`: no
    ///    fabricated rc-0); AND
    /// 3. the consumption gates pass ([`consumption_ok`]): a guard whose `Stdout`/
    ///    `Stderr` is consumed, or whose status is a `while`/`until` loop condition
    ///    (`StatusIterated`), still blocks. A `StatusRelaxable`-consumed status with a
    ///    *known* rc relaxes (the whole point — the fold reads the exact rc, substitutes it);
    ///    an `if`/`elif` guard is now `StatusRelaxable` too (arch-1), so a known-rc guard
    ///    Query is exactly this path.
    ///
    /// An INVALID guard arrives with `status == ⊤` from its phased caller (the cli
    /// withholds the stale rc), so condition (2) already blocks it — but we also gate
    /// on `valid` directly so a mis-wired caller cannot smuggle a stale rc through.
    #[must_use]
    pub(crate) fn prove_query_replaceable(
        fact: FactKey,
        valid: bool,
        verdict: Verdict,
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        query_substitutes(valid, consumed, status).then_some(ReplaceLicense {
            custody: dorc_core::LicenseCustody::MeasuredSelf,
            fact,
            derivation: Derivation {
                fact,
                via: LicenseVia::QueryGuard,
                ambient: false,
                grade: Grade::Must,
                verdict,
                witness: dorc_core::Witness::empty(),
                survival: None,
                vouch_span: None, // Query/loop/call elisions consume no vouch ⇒ no locus
                establish_vouches: Vec::new(),
                probe: None,
            },
        })
    }

    /// Mint a license for an in-loop **Members** body leaf's convergence-elision (task-L2
    /// item-3, `209` brk-1(b)) — the all-or-nothing in-loop license. Implemented EXACTLY as
    /// the four conjuncts of item-3, every ambiguity resolving to REFUSE:
    ///
    /// (a) EVERY member's fact is Converged — `member_verdicts` is the per-member host
    ///     verdict (Effect channel); a single non-Converged member refuses (the family is
    ///     all-or-nothing — partial-member elision is a deferred direction, not this).
    /// (b) `self_reached` (the engine's item-3(b) bit): the only in-script writers reaching
    ///     this site are its own per-member establishes (no pre-loop/sibling/Opaque). The
    ///     RATIONALE this preserves: the elision's own effect removes the body's writes, so
    ///     under the elision the resting probe stays authoritative (a fixed-point argument:
    ///     elide-all is self-consistent); ANY non-self writer breaks that argument ⇒ refuse.
    /// (c) the consumption gates pass ([`consumption_ok`]): the in-loop leaf's status is
    ///     errexit/`$?`-marked by the existing machinery — under fork-mutator-rc a mutator's
    ///     rc is ⊤, so a CONSUMED status (errexit-region, or a post-loop `$?` reading the
    ///     body, item-6a) blocks; a consumed Stdout/Stderr or render-floor blocks too.
    /// (d) per-member-resolvable (item-4): a member with no probe-sourced observation
    ///     arrives `Verdict::Unknown` ⇒ not Converged ⇒ (a) refuses it. So (d) is subsumed.
    ///
    /// The leaf still ITERATES N times over `true` (the render substitutes a `true` body —
    /// observable-preserving given (a)+(c)). `member_verdicts` empty ⇒ refuse (defensive;
    /// a Members site has ≥1 member). The witness records the FIRST member's fact as the
    /// representative `fact` (the family is the establish; provenance names one cell).
    #[must_use]
    fn prove_members_replaceable(
        all_vouched: AllEstablishesVouched,
        member_verdicts: &[Verdict],
        self_reached: bool,
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        let representative = all_vouched.representative();
        if !self_reached {
            return None;
        }
        // (a) all members Converged — a non-Converged (Diverged/Unknown) member refuses.
        if member_verdicts.is_empty() || !member_verdicts.iter().all(|v| *v == Verdict::Converged) {
            return None;
        }
        // (c) the consumption gates (the in-loop leaf's status is ⊤ for a mutator —
        // fork-mutator-rc — so a consumed status blocks; stdout/stderr/render-floor block).
        if !consumption_ok(consumed, status) {
            return None;
        }
        let establish_vouches = all_vouched.into_receipts();
        Some(ReplaceLicense {
            custody: dorc_core::LicenseCustody::VouchedSeverally,
            fact: representative,
            derivation: Derivation {
                fact: representative,
                via: LicenseVia::MembersLoop,
                ambient: true,
                grade: Grade::Must,
                verdict: Verdict::Converged,
                witness: dorc_core::Witness::empty(),
                survival: None,
                vouch_span: None,
                establish_vouches,
                probe: None,
            },
        })
    }

    /// Mint a license for an inlined function-CALL's convergence-elision (arch-2, brk-2,
    /// `i-3`) — the all-or-nothing CALL license (the Members precedent, 20S). The call's
    /// command word resolved to a same-file-earlier funcdef and its body was spliced; this
    /// mints a [`LicenseVia::InlineCall`] `Replace` (the CALL span → `true`) iff EVERY
    /// effect-bearing body leaf licenses elision, every ambiguity ⇒ REFUSE:
    ///
    /// * every body ESTABLISH — `EstablishProbeAmbient` and `EstablishProbeWritten` alike — has a
    ///   Converged fact (a single non-Converged ⇒ refuse; the whole call runs). The two origins are
    ///   NOT distinguished here and must not be: origin-reach answers which check may ship, never
    ///   whether a resting measurement is still good (`origin-reach-is-probe-only`). Staleness is
    ///   effective FRESHNESS, the caller's conjunct in `settle`, which this mint cannot see;
    /// * NO body site is a blocker — a `MustRun` (a body Kill, an Opaque/⊤ command, a multi-cell
    ///   verb, an unreachable establish), an in-loop `EstablishMembers` (an in-loop call body —
    ///   out of slice), or a nested `InlineCall` (defensive — transitive inlines are flattened to
    ///   leaves, so one should never appear here);
    /// * a body `QueryResolvable` does NOT block (it is read-only — the wrapper-pun's
    ///   `dpkg -s "$1"` guard); its convergence is irrelevant to the call's elision (the call
    ///   elides on the body's ESTABLISH facts, not the guard's rc);
    /// * the CALL site's own consumed channels are reproduced ([`consumption_ok`]): the call's
    ///   rc is ⊤ (a mutator-shaped aggregate, fork-mutator-rc), so a consumed status
    ///   (errexit-region, a `$?`-reader, a bare `||` operand) blocks; a consumed Stdout/Stderr
    ///   or a `while`/`until` condition blocks; door-3 (`call || true`) does NOT block (the
    ///   `StatusInvariant` channel) — the composition case (`i-5`).
    ///
    /// RATIONALE (the all-or-nothing fixed point, the i-3 weld): the elision removes the WHOLE
    /// body, so under the elision the resting probe stays authoritative only if every body
    /// effect is accounted Converged. A call whose body has NO converged establish to elide —
    /// a wrapper of pure builtins (`foo() { echo hi; }`), or a body whose only effects are
    /// Queries — RUNS (refuse): eliding it would gain nothing and would suppress whatever its
    /// pure leaves do (an `echo`'s stdout); the run-it floor is harmless (`kFAIL-perform`). The
    /// witness records the FIRST establishing body fact as the representative `fact` (the call
    /// IS the aggregate establish; provenance names one cell).
    #[must_use]
    fn prove_inline_replaceable(
        sites: &[InlineSite],
        all_vouched: AllEstablishesVouched,
        observe: &(impl Fn(FactKey) -> Observable + ?Sized),
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        let mut representative: Option<FactKey> = None;
        for site in sites {
            match &site.class {
                SkipClass::EstablishProbeAmbient(f) | SkipClass::EstablishProbeWritten(f) => {
                    if observe(*f).effect != Verdict::Converged {
                        return None; // a non-converged body establish ⇒ the whole call runs
                    }
                    representative.get_or_insert(*f);
                }
                // A read-only Query guard never blocks (the wrapper-pun's `dpkg -s "$1"`); its
                // own convergence does not gate the call's elision.
                SkipClass::QueryResolvable { .. } => {}
                // Every other class blocks the whole call (all-or-nothing): a MustRun
                // (Kill/Opaque/⊤), an in-loop Members body, or a nested InlineCall (defensive —
                // should be flattened).
                SkipClass::MustRun
                | SkipClass::EstablishMembers { .. }
                | SkipClass::InlineCall { .. } => return None,
            }
        }
        // A call with NO converged establish to elide runs (the run-it floor): there is no
        // mutation to be already-done, and eliding a pure-builtin wrapper would suppress its
        // observable (an `echo`'s stdout) for no gain.
        let fact = representative?;
        if fact != all_vouched.representative() {
            return None;
        }
        // The CALL site's own consumed channels (the aggregate rc is ⊤ — a mutator-shaped
        // call, fork-mutator-rc — so a consumed status blocks; door-3 `|| true` does not).
        if !consumption_ok(consumed, status) {
            return None;
        }
        let establish_vouches = all_vouched.into_receipts();
        Some(ReplaceLicense {
            custody: dorc_core::LicenseCustody::VouchedSeverally,
            fact,
            derivation: Derivation {
                fact,
                via: LicenseVia::InlineCall,
                ambient: true,
                grade: Grade::Must,
                verdict: Verdict::Converged,
                witness: dorc_core::Witness::empty(),
                survival: None,
                vouch_span: None,
                establish_vouches,
                probe: None,
            },
        })
    }

    /// Mint the license for ONE authored region that every invocation instance agreed to replace
    /// (`30L` §5/§6).
    ///
    /// The per-instance conditions are already discharged: each route ran the ordinary site
    /// decision, so each proved its own vouch, verdict, freshness, and consumption independently,
    /// and the meet then required them to reproduce the SAME stand-in (`30L`
    /// `rul-shared-edit-reproduces-every-route`). What this mint adds is the thing no route can
    /// carry — the CROSS-INSTANCE witness. `all_vouched` is built from the exact ordered union of
    /// every contributing instance's `(site, cell)`, identity- and cardinality-matched by
    /// [`AllEstablishesVouched::mint`], so a witness that spans fewer instances than the population
    /// cannot be spelled (`30L:pin-shared-witness-spans-instances`).
    ///
    /// `consumed` is the UNION over every instance's consumed channels, re-checked here rather than
    /// trusted: one authored edit answers for every call context at once, and a union can only
    /// block.
    #[must_use]
    fn prove_shared_region_replaceable(
        all_vouched: AllEstablishesVouched,
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        if !consumption_ok(consumed, status) {
            return None;
        }
        let fact = all_vouched.representative();
        let establish_vouches = all_vouched.into_receipts();
        Some(ReplaceLicense {
            custody: dorc_core::LicenseCustody::VouchedSeverally,
            fact,
            derivation: Derivation {
                fact,
                via: LicenseVia::SharedRegion,
                ambient: true,
                grade: Grade::Must,
                verdict: Verdict::Converged,
                witness: dorc_core::Witness::empty(),
                survival: None,
                vouch_span: None,
                establish_vouches,
                probe: None,
            },
        })
    }

    fn prove_inline_query_replaceable(
        proof: ReadSubstitutionProof,
        consumed: &May<Powerset<Channel>>,
    ) -> Option<ReplaceLicense> {
        consumption_ok(consumed, proof.status).then_some(ReplaceLicense {
            custody: dorc_core::LicenseCustody::MeasuredSelf,
            fact: proof.fact,
            derivation: Derivation {
                fact: proof.fact,
                via: LicenseVia::InlineCall,
                ambient: false,
                grade: Grade::Must,
                verdict: Verdict::Converged,
                witness: dorc_core::Witness::empty(),
                survival: None,
                vouch_span: None,
                establish_vouches: Vec::new(),
                probe: None,
            },
        })
    }

    /// Attach the FULL granted witness post-mint (arch-1 `vp-17`/`vp-18`) — pure OUTPUT
    /// provenance, set AFTER the decision so it cannot influence the mint (the WELD). Called by
    /// `build_plan` with the establish site's origin(s); the witness is on the EXEMPT plane
    /// (`Exempt::ReceiptId`), so the `erasability` gate proves it perturbs no decision.
    #[must_use]
    pub fn with_witness(mut self, witness: dorc_core::Witness) -> Self {
        self.derivation.witness = witness;
        self
    }

    /// Attach the SURVIVAL witness post-mint (Stage 2 / TC-3) — the attribution for an elision
    /// that crossed ≥1 running wall. Like [`with_witness`](Self::with_witness) it is pure OUTPUT
    /// provenance set AFTER the mint (the survival decision happens in effective freshness, downstream
    /// of the license mint), so it never influences whether the license was granted. Rides the
    /// render surface (the why-lens) only — never the byte-floored artifact (rec-1).
    #[must_use]
    pub fn with_survival(mut self, witness: SurvivalAttribution) -> Self {
        self.derivation.survival = Some(witness);
        self
    }

    fn with_aggregate_survival(mut self, witness: AggregateSurvivalWitness) -> Option<Self> {
        let receipts = self
            .derivation
            .establish_vouches
            .iter()
            .map(|receipt| (receipt.site, receipt.fact));
        let survived = witness
            .members()
            .map(|member| (member.site(), member.fact()));
        if !receipts.eq(survived) {
            return None;
        }
        self.derivation.survival = Some(SurvivalAttribution::Aggregate(witness));
        Some(self)
    }

    /// Attach the probe-side attribution post-mint (`27V` Lane A) — who reported the licensing
    /// fact, when, and with what tool-rc. Same posture as [`with_witness`](Self::with_witness):
    /// set AFTER the decision, read only by the why render, exempt from the identity plane.
    #[must_use]
    pub fn with_probe_attribution(mut self, attribution: Option<ProbeAttribution>) -> Self {
        self.derivation.probe = attribution;
        self
    }

    fn with_aggregate_probe_attribution(
        mut self,
        attributions: &BTreeMap<FactKey, ProbeAttribution>,
    ) -> Self {
        self.derivation.probe = attributions.get(&self.fact).copied();
        for receipt in &mut self.derivation.establish_vouches {
            receipt.probe = attributions.get(&receipt.fact).copied();
        }
        self
    }

    /// The fact whose established-ness licensed this skip.
    #[must_use]
    pub fn fact(&self) -> FactKey {
        self.fact
    }

    /// WHOSE utterance this license rests on (`28M` §8 — the monologue, typed).
    #[must_use]
    pub fn custody(&self) -> dorc_core::LicenseCustody {
        self.custody
    }

    /// The audit trail (the greyed-out "why" for the plan UI).
    #[must_use]
    pub fn derivation(&self) -> &Derivation {
        &self.derivation
    }
}

/// The shared consumed-observable gate for both substitution paths (the un-vouched
/// channel check, 16F §3 / 19C strain-D / 19D / `206` §3). The fact arrives un-collapsed
/// as a `May` (over-approximate consumption): per `inv-must-may` a `May` value can only
/// BLOCK a license, never grant one. Returns `true` iff NO unvouched observable
/// forbids the substitution:
/// * `Stdout`/`Stderr` — empty default vouched by nothing ⇒ a consumed one always
///   blocks (a declared/probed rc does NOT vouch output *content*);
/// * `StatusIterated` (a `while`/`until` condition) — blocks unconditionally: the
///   condition's per-iteration rc-sequence cannot be reproduced by one predicted rc, and a
///   constant-substituted loop condition is an infinite/zero-iteration disaster (arch-1,
///   note 214 — the honest successor to the retired `StatusRenderFloor`, keyed on iteration);
/// * `StatusRelaxable` (a `&&`/`||` left operand, an errexit-region command, a
///   `$?`-reader's predecessor, or — since arch-1 — an `if`/`elif` guard) — blocks ONLY
///   when the rc is ⊤ (a fabricated rc-0 `true` would suppress a `|| fallback`, the
///   `kFAIL-perform` under-execute); a known/probe-sourced rc relaxes it
///   (`StandIn::from_rc` reproduces the exact status);
/// * `StatusInvariant` (the `cmd || true` shape — door-3, `20V` §4) — NEVER blocks,
///   regardless of prediction (⊤ included): both `||` continuations rejoin with identical
///   observables, so any stand-in rc is extensionally faithful (`19D`'s under-execute
///   cannot arise — there is no `|| fallback` whose firing a fabricated rc-0 would
///   suppress; the fallback *is* `true`, observable-free). Still RECORDED in `consumed`
///   (disclosure/provenance sees the read); only the blocking judgment is "never".
///
/// Sound in BOTH phases; only what a blocked leaf *becomes* is phase-keyed (the
/// caller's collapse, `inv-superposition`).
/// THE definition of "this Query guard's probed rc substitutes for the guard" — one seat,
/// two readers (`26H` §4 `dec-no-duplicated-license-logic`).
///
/// [`ReplaceLicense::prove_query_replaceable`] mints the substitution license from it, and
/// [`crate::erase::prove_dead_branches`] requires it of every leaf under a fold controller
/// before that controller may back an erasure. The second reader is why this is factored:
/// the erasure ledger must know the controller will really be substituted away, and
/// re-deriving that rule beside the original would be exactly the drift that turns a
/// precision fix into a wrong-elision.
///
/// The three conjuncts: the guard is VALID (rule-query-validity — no mutator/opaque reached
/// it, else its resting rc is stale); its rc is a KNOWN probe-sourced value, never a
/// fabricated rc-0 (`inv-probe-sourced-values`), which is also what lets the fold resolve
/// the `&&`/`||` at all; and the consumption gates pass.
pub(crate) fn query_substitutes(
    valid: bool,
    consumed: &May<Powerset<Channel>>,
    status: Predicted<Rc>,
) -> bool {
    valid && !matches!(status, Predicted::Top) && consumption_ok(consumed, status)
}

fn consumption_ok(consumed: &May<Powerset<Channel>>, status: Predicted<Rc>) -> bool {
    let May(consumed) = consumed;
    if consumed.contains(&Channel::Stdout) || consumed.contains(&Channel::Stderr) {
        return false;
    }
    if consumed.contains(&Channel::StatusIterated) {
        // A `while`/`until` condition's per-iteration rc-sequence cannot be reproduced by a
        // single predicted rc, and a constant-substituted loop condition is an
        // infinite/zero-iteration disaster ⇒ blocks unconditionally (arch-1, note 214 — the
        // honest successor to the retired `StatusRenderFloor`, keyed on iteration).
        return false;
    }
    if consumed.contains(&Channel::StatusRelaxable) && matches!(status, Predicted::Top) {
        return false;
    }
    // `Channel::StatusInvariant` (door-3) is intentionally absent from every block above:
    // a site carrying ONLY it (its sole status-consumer is a `|| true`) passes even at ⊤.
    // A site that ALSO carries a blocking mark (`StatusRelaxable` from an inner `||` or an
    // if/elif guard, `StatusIterated` from a loop condition, a consumed `Stdout`) is still
    // blocked by that mark — Invariant never *un*-blocks, it only declines to block (the
    // d-3 mark-union rule).
    true
}

// ===========================================================================
// The plan: per-leaf run/skip + render-back-to-sh (the leaf-seam, dn-3)
// ===========================================================================

/// A stable identifier for one executable leaf in a plan (`dn-3`, the leaf-seam):
/// executable work is a list of *individually wrappable* leaves, each with a
/// stable back-map to its source — NEVER one opaque `sh -c "$bigscript"`. The
/// back-map is [`Step::ast`]; the id is this leaf's position in source order.
///
/// Defined in `core` (`dec-seam-ownership`, the `dac-B` shared vocabulary) and
/// re-exported here: the round-22 structured diagnostic ([`dorc_aid::diag::SiteId`])
/// keys on it, so the base crate owns it and `plan` shares the one type rather than a
/// parallel one (`inv-site-keyed-results`).
pub use dorc_core::LeafId;

/// The cheapest sh stand-in that reproduces a leaf's **exact** PROBE-MEASURED exit status
/// (DESIGN `16F`/`16P-T10`; `inv-probe-sourced-values`).
/// NOT always `:`: the value the downstream fold/guard reads must be preserved, so a
/// known-rc Query substitutes `(exit n)` rather than `true` — else its rc-0 stub would
/// suppress a `|| fallback` (the `kFAIL-perform` under-execute the round-19 adversarial
/// pass proved). An Establish site's status is withheld to ⊤ at intake, so it stands in
/// as `true` and reproduces nothing measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandIn {
    /// rc 0 — `true` (the human's choice over `:` for the common conforming case).
    True,
    /// rc 1 — `false`.
    False,
    /// any other rc `n` — `(exit n)` (a subshell so it reproduces the status without
    /// terminating the surrounding script).
    Exit(i32),
}

impl StandIn {
    /// The stand-in reproducing a concrete observed exit status.
    #[must_use]
    pub fn from_rc(rc: Rc) -> Self {
        match rc.0 {
            0 => StandIn::True,
            1 => StandIn::False,
            n => StandIn::Exit(n),
        }
    }

    /// The sh that reproduces the status — the value-preserving substitution bytes.
    /// Delegates to [`render::standin_sh`] (the artifact assembler, task-R): the
    /// `true`/`false`/`(exit n)` text lives in ONE audited home, with its
    /// `dash -n`-clean / subshell-non-abort guarantee documented there.
    #[must_use]
    pub fn sh(self) -> String {
        render::standin_sh(self)
    }
}

// ===========================================================================
// The guard tier (rul-ternary-verdict / rul-guard-license / 24D §2 — the third verb)
// ===========================================================================

/// The judgment-plane **vouch descriptor** — the payload a [`ByVouch<VerdictVouch>`] carries
/// (`core::claim`, TC-tier-4's `VouchAndRung<VerdictVouch>` inner). It is what the guard emitter needs
/// to ship the oracle's own verdict body strip-only and invoke it at position, plus the
/// attribution label — and NOTHING that is a fact-plane value (TC-tier-3: a vouch informs a
/// license, never becomes an ambient fact). Built by the cli edge from the lifted verdict function
/// (`dorc_oracle::verdict`); the kernel receives it as data (`inv-determinism` — no oracle lift in
/// the kernel). All fields are pre-resolved Strings so the byte-floored span render
/// ([`Plan::render_apply`]) needs no interner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdictVouch {
    /// The mangled verdict funcname (`apt_get__is_converged`) — the preamble-dedup key AND the
    /// invocation's word 0.
    fn_name: String,
    /// The oracle's own verdict body, shipped STRIP-ONLY (`dorc_oracle::predict::strip_verdict`) —
    /// the guard preamble def, the SAME authored bytes the probe lane self-vouches
    /// (rul-ternary-verdict: the `predict()` IS the oracle; no engine-synthesized sh).
    body: String,
    /// The SNAPSHOT the body needs beside it (helpers, file-level constants), as DECLARATIONS rather
    /// than a blob: the apply artifact hoists one shared preamble above the whole book, so two guards
    /// that reached the same helper must emit it once
    /// (`plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding`).
    closure: Vec<dorc_oracle::closure::ClosureDecl>,
    /// `closure` and `body` concatenated — DERIVED at the one seat that can set either
    /// ([`VerdictVouch::with_closure`]), never assignable on its own, so the two spellings of "what
    /// this guard runs" cannot drift apart.
    preamble: String,
    /// The full check invocation the guard runs at position (`apt_get__is_converged install -y
    /// curl`) — the cli builds it (funcname + the site's resolved argv). Ships as the `||`-LEFT.
    invocation: String,
    /// The fact's kind name (`package`) for the `# dorc: guard [<kind> converged-vouch; …]`
    /// attribution comment (cli-resolved; the render has no interner).
    kind_label: String,
    /// The verdict body's own literal check-commands (`dpkg-query`) — gate-6 attribution ONLY
    /// (`dorc_oracle::verdict::check_commands`). The cli emits a `guardcmd <argv0>` ledger line per
    /// entry so the widened dual-rail judge allowlists the guard's live check as a legitimate
    /// apply-only line. Display/attribution, NOT decision data: EXCLUDED from
    /// [`GuardInsert::canonical`] (it derives from `preamble`, which the canon already covers).
    check_cmds: Vec<String>,
    /// WHOSE utterance this is (`28M` §8 — the monologue, typed). Decision-plane, unlike
    /// `defining_span` beside it: the elide mint reads this to stamp the license's custody, so a
    /// future widening that reproduced another author's measured value cannot silently inherit
    /// this author's provenance.
    custody: dorc_core::DefinitionCustody,
    /// The vouch's DEFINING span (C7 `27V:mech-minting-line-threading`): the reached vouching
    /// arm + which oracle file it indexes (`tc-oracle-file-identity`), for the guard attribution's
    /// `file:line`. `None` when the caller did not thread it (the DST/test constructors) or the
    /// vouch located no check (an explicit `return 0` — the render falls back to no locus). Display
    /// tier only — decision-inert (a vouch informs a license and never becomes a fact, TC-tier-3).
    defining_span: Option<(dorc_core::Span, SourceFileId)>,
}

impl VerdictVouch {
    /// Build a vouch descriptor from the cli-resolved verdict-function data (the sole constructor;
    /// the cli edge owns the lift + strip + argv-render). `fn_name`/`invocation` are the mangled
    /// name and the full invocation; `preamble` is the stripped body; `kind_label` the fact's kind
    /// for attribution; `check_cmds` the verdict body's own command names (gate-6 attribution). The
    /// defining span is threaded separately by [`with_defining_span`](Self::with_defining_span)
    /// (only the plan driver has the oracle-file index; DST/test constructors omit it).
    ///
    /// `custody` is REQUIRED rather than threaded post-construction like the defining span, and the
    /// asymmetry is deliberate (`28M` §8): the span is display, custody is decision. Every
    /// constructor — DST and test constructors included — has to answer whose utterance this vouch
    /// is, because a vouch with no owner is exactly the composite-nobody-said that the monologue
    /// property exists to forbid.
    #[must_use]
    pub fn new(
        fn_name: String,
        body: String,
        invocation: String,
        kind_label: String,
        check_cmds: Vec<String>,
        custody: dorc_core::DefinitionCustody,
    ) -> Self {
        Self {
            fn_name,
            preamble: body.clone(),
            body,
            closure: Vec::new(),
            invocation,
            kind_label,
            check_cmds,
            custody,
            defining_span: None,
        }
    }

    /// Attach the snapshot the body ships with (`28R:rul-snapshot-transplant-emission`).
    ///
    /// The ONE seat that may write `preamble`, which it re-derives here — a caller that
    /// pre-concatenated its own prefix would leave the emission unable to dedup declarations across
    /// guards, and two same-named funcdefs in the hoisted preamble is the shape
    /// `pinned-definitions-are-the-artifact's-binding` forbids.
    #[must_use]
    pub fn with_closure(mut self, closure: &dorc_oracle::closure::Closure) -> Self {
        self.preamble = format!("{}{}", closure.sh(), self.body);
        self.closure = closure.decls().to_vec();
        self
    }

    /// The definition's own bytes, without its snapshot — the emission unit the tiering renames.
    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// The snapshot declarations, in emission order.
    #[must_use]
    pub fn closure(&self) -> &[dorc_oracle::closure::ClosureDecl] {
        &self.closure
    }

    /// WHOSE utterance this vouch is (`28M` §8). By construction the same custody the positional
    /// agreement gate compared when it admitted this definition — one question, one type, and now
    /// one answer rather than two spellings that could drift apart.
    #[must_use]
    pub fn custody(&self) -> dorc_core::DefinitionCustody {
        self.custody
    }

    /// Thread the vouch's defining span + oracle-file id post-construction (C7). Low-churn: only
    /// `build_vouches` (which holds the oracle-file index) calls it; every other constructor leaves
    /// it `None` and the render omits the locus.
    #[must_use]
    pub fn with_defining_span(mut self, arm: dorc_core::Span, file: SourceFileId) -> Self {
        self.defining_span = Some((arm, file));
        self
    }

    /// The vouch's defining span + oracle-file id (C7), if threaded — the guard render's `file:line`.
    #[must_use]
    pub fn defining_span(&self) -> Option<(dorc_core::Span, SourceFileId)> {
        self.defining_span
    }
}

/// The **guard insertion** the render overlays at a site (rul-guard-license / crisis-closure
/// carve-out, `inv-probe-sourced-values`): it mints NO values — no [`StandIn`], no [`Predicted`],
/// no [`Observable`]. On PASS the check's own live rc is the line's rc; on fall-through the
/// original command runs and its observables are genuine. Carries the consumed vouch's emitter
/// data plus the plan-time probe [`Verdict`] (attribution only — the guard re-decides LIVE at
/// apply, never trusting this stale prediction; that is the whole point, X-drift).
#[derive(Debug, Clone)]
pub struct GuardInsert {
    vouch: VerdictVouch,
    probe_verdict: Verdict,
}

impl GuardInsert {
    /// The mangled verdict funcname (the preamble-dedup key).
    #[must_use]
    pub fn fn_name(&self) -> &str {
        &self.vouch.fn_name
    }

    /// The guard preamble def to ship (deduped per [`fn_name`](Self::fn_name)) — the definition
    /// PLUS its snapshot, which is what the probe lane and the erasability canon read.
    #[must_use]
    pub fn preamble(&self) -> &str {
        &self.vouch.preamble
    }

    /// The definition's own bytes, without its snapshot — the unit the apply hoist tiers and renames.
    #[must_use]
    pub fn body(&self) -> &str {
        self.vouch.body()
    }

    /// The snapshot declarations this guard's body needs, in emission order.
    #[must_use]
    pub fn closure(&self) -> &[dorc_oracle::closure::ClosureDecl] {
        self.vouch.closure()
    }

    /// The vouch's defining span + oracle-file id (C7 `file:line`), if the plan driver threaded it.
    #[must_use]
    pub fn defining_span(&self) -> Option<(dorc_core::Span, SourceFileId)> {
        self.vouch.defining_span()
    }

    /// The verdict body's own check-commands (gate-6 `guardcmd` attribution; 23A §5). The cli
    /// emits one `guardcmd <argv0>` per entry so the dual-rail judge allowlists the guard's live
    /// check as a legitimate apply-only line — never an unrelated one (cf-5).
    #[must_use]
    pub fn check_cmds(&self) -> &[String] {
        &self.vouch.check_cmds
    }

    /// The erasability-IDENTITY canon (24D §2): the DECISION-relevant guard bytes (funcname,
    /// invocation, sense, preamble) — the artifact code. EXCLUDES the attribution (the probe word,
    /// the kind label): those OVERLAY the render as display and are erasability-EXEMPT, exactly as
    /// `Derivation.survival` is, so a plan differing only in guard attribution digests identically.
    /// A guard carries NO `ProvId`/arena witness (it mints no values — the carve-out), so there is
    /// no receipt to strip here; this canon is the whole guard.
    pub(crate) fn canonical(&self) -> String {
        format!(
            "fn={} inv={} preamble={}",
            self.vouch.fn_name, self.vouch.invocation, self.vouch.preamble
        )
    }

    /// The plan-time probe word for the attribution comment (`holds`/`absent`/`cant-tell`). NB:
    /// disclosure only — the guard's runtime rc, not this, decides the fall-through.
    fn probe_word(&self) -> &'static str {
        match self.probe_verdict {
            Verdict::Converged => "holds",
            Verdict::Diverged => "absent",
            Verdict::Unknown => "cant-tell",
        }
    }

    /// Render the guarded line: `( <check-invocation> ) || <original>   # dorc: guard [<kind>
    /// converged-vouch; probe: <word>]` (24D §2 / rul-ternary-verdict). The original bytes survive
    /// VERBATIM as the `||`-right (no code path removes them — the two never-clauses). The glue is
    /// always the direct `( f_is_converged args ) || <original>` (`24C:rul24-ditch-is-diverged`: the
    /// sole verdict role is `is_converged`; the inverted sense is now spelled with explicit-return
    /// manual inversion inside it). `original` is the site's verbatim command bytes.
    #[must_use]
    fn render_line(&self, original: &str, invoked: &str) -> String {
        format!(
            "{} || {original}   # dorc: guard [{} converged-vouch; probe: {}]",
            self.check_form(invoked),
            self.vouch.kind_label,
            self.probe_word(),
        )
    }

    /// The check as it ships: `( <emitted-fn> <site argv> )`.
    ///
    /// `invoked` is the name the ARTIFACT binds this body to, which is the authored funcname in the
    /// single-definition case and a hash-disambiguated one where the unit holds two distinct bodies
    /// under one name (`28K` §4 `rul-hash-munge-disambiguation`). The caller resolves it through
    /// [`Plan::pinned_definitions`], because it is a whole-artifact property no single insert knows.
    #[must_use]
    pub fn check_form(&self, invoked: &str) -> String {
        match self
            .vouch
            .invocation
            .strip_prefix(self.vouch.fn_name.as_str())
        {
            Some(argv) => format!("( {invoked}{argv} )"),
            None => format!("( {} )", self.vouch.invocation),
        }
    }

    /// The guarded line as the apply artifact carries it, MINUS its receipt comment — what the why
    /// surface shows when it says "here is what dorc shipped instead of a skip" (`28G` strawman
    /// `b-wide-guarded`).
    ///
    /// Display, never execution (`27W:rul-report-surface-massaging`): dropping the provenance
    /// comment is a repair-directing massage of bytes that are not ours, and the executable plane
    /// keeps reading [`render_line`]. The two-surfaces byte floor is untouched.
    /// Names the AUTHORED function, never a hash-disambiguated emission: the artifact's munged name
    /// is engine scaffolding, and the surface a human reads to answer "whose judgment is this?"
    /// must point at the body its author wrote (`28K` §4 — plan-render attribution names the
    /// authored function and its `file:line`).
    #[must_use]
    pub fn display_line(&self, original: &str) -> String {
        format!("{} || {original}", self.check_form(&self.vouch.fn_name))
    }
}

/// What the apply artifact hoists, and which name each guarded site's check invokes
/// (`28K` §4 — see [`Plan::pinned_definitions`], which is the only constructor).
#[derive(Debug, Default)]
pub struct PinnedDefinitions {
    hoisted: String,
    invoked: BTreeMap<AstId, String>,
}

impl PinnedDefinitions {
    /// The definitions to emit above the book, in a deterministic order. EMPTY when every pinned
    /// body is already in place, which is what keeps a guard-free — and an in-book-oracle — book
    /// byte-identical to its own text.
    #[must_use]
    pub fn hoisted(&self) -> &str {
        &self.hoisted
    }

    /// The funcname the guard at `ast` invokes. Absent for a site that guards nothing.
    #[must_use]
    pub fn invoked(&self, ast: AstId) -> Option<&str> {
        self.invoked.get(&ast).map(String::as_str)
    }
}

/// The short, deterministic disambiguator a hash-munged name carries (`28K` §4
/// `rul-hash-munge-disambiguation`).
///
/// SHA-256 of the definition BYTES, first 8 hex digits — the same digest `book_digest` uses, not an
/// FNV drift-detector, because this reaches the shipped artifact
/// (`rul-fixture-identity-never-production`). It answers "are these the same bytes" and nothing
/// else; uniqueness WITHIN one artifact is what matters and is checked at the emission seat.
fn short_digest(body: &str) -> String {
    invocation::book_digest(body)
        .get(..8)
        .unwrap_or_default()
        .to_owned()
}

/// Does the book's own text define `name` at top level at all, whatever the bytes?
///
/// Tier-2's collision rule (`28R:rul-instantiation-hash-dedup`): on a static collision with a book
/// name the BOOK's name survives and ours munges, always — the admin's bytes are never rewritten and
/// never copied, so the only thing left to move is our own emission. Deliberately TOP-LEVEL only: a
/// regional book definition competes at its own scope and sh keeps the two apart, which is the
/// emission `308:cr-artifact-two-funcdefs-letter` ratified. The custody DENIAL census reads the book
/// at every depth instead, because there the question is whether the admin's body can reach somebody
/// else's vouch, and a region can.
fn book_defines_at_top_level(ast: &Ast, name: &str) -> bool {
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return false;
    };
    items
        .iter()
        .any(|&item| matches!(&ast.node(item).kind, NodeKind::FuncDef { name: n, .. } if n == name))
}

/// Does the book's own text already define `name` at top level with exactly `body`'s bytes?
///
/// The test is BYTES, not identity: a definition that matches is the pinned one, sitting where its
/// author put it, so copying it above the book would put two same-named funcdefs in the shipped
/// artifact for no gain (`28K` §4 retires that shape by any route). A definition that does NOT
/// match is a different body under the same name, and the hoist-plus-munge path handles it.
fn book_already_defines(src: &str, ast: &Ast, name: &str, body: &str) -> bool {
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return false;
    };
    items.iter().any(|&item| {
        let node = ast.node(item);
        matches!(&node.kind, NodeKind::FuncDef { name: n, .. } if n == name)
            && src.get(node.span.lo.0 as usize..node.span.hi.0 as usize) == Some(body)
    })
}

/// The witness authorising a **guard** — the third verb of rul-ternary-verdict's {elide, guard,
/// run}. Mirrors [`ReplaceLicense`]'s private-fields / sole-mint pattern (TC-3-shaped): the ONLY
/// way to obtain one is [`GuardLicense::mint`], which DEMANDS a [`ByVouch<VerdictVouch>`] (the
/// vouch; TC-tier-2) — no vouch ⇒ no `GuardLicense` ⇒ run (rul-guard-license). A plan emitter
/// accepts a `GuardLicense`, never a `bool`, so a guard cannot be spelled without the judgment.
///
/// **Uncheckable invariant (rul24-overtype):** no vouch ⇒ no `GuardLicense` ⇒ run. The vouch is
/// judgment-tier and NEVER enters the fact-plane (rul-guard-license); the guard re-verifies LIVE
/// at apply and never trusts the plan-time verdict it carries (X-drift). Types protect the
/// plumbing — that the guard is licensed by an authored judgment — never the truth of that
/// judgment (a wrong verdict function still falls through to run, by the `||` glue).
#[derive(Debug, Clone)]
pub struct GuardLicense {
    fact: FactKey,
    insert: GuardInsert,
    /// The REPORTED half of a guarded site's chain, exactly as [`Derivation::probe`] is a skipped
    /// site's: which record measured the fact this guard re-verifies, when it was taken in, with
    /// what tool-rc, and which funcdef reported it.
    ///
    /// A guard's chain has the same three speakers a skip's does — the report, the author's vouch,
    /// and the walls between them (`28G` strawman `b-wide-guarded`) — and the render could name
    /// only the second of the three without this. Attached POST-mint, exactly like
    /// [`ReplaceLicense::with_probe_attribution`]: pure OUTPUT provenance keyed on a site the mint
    /// already decided, so no decision can read it, and `Eq`/identity treat it as absent.
    probe: Option<ProbeAttribution>,
}

impl GuardLicense {
    /// Mint a guard iff the plan-time probe [`Verdict`] is [`Verdict::Converged`] (jc-mint-policy
    /// m-a: converged-past-wall ONLY — a guard at a predicted-change site buys nothing, flagship
    /// site 3). CONSUMES the [`ByVouch<VerdictVouch>`] by value (TC-tier-2: a [`core::claim::ByObservation`]
    /// or a silence claim does not satisfy this signature). A diverged/unknown verdict ⇒ `None` ⇒
    /// the site runs (`inv-kfail`).
    ///
    /// AND iff no consumed channel would read a value the guard REPLACES. `guards-mint-no-values`
    /// is usually read as the guard's freedom — it reproduces nothing, so it needs no
    /// probe-provenance — but it has a second edge: on the PASS path the line's status is the
    /// CHECK's live rc and its output is the check's, not the original's. So a site whose Status,
    /// Stdout, or Stderr some consumer reads cannot be guarded at all: `apt-get install -y vim;
    /// rc=0` would capture `apt_get__is_converged`'s status, a value the authored program could
    /// never produce. A mutator's own status is ⊤ (`fork-mutator-rc`) and the check's is unknown
    /// until apply, so the gate is [`consumption_ok`] at ⊤ — which admits exactly
    /// `StatusInvariant` (the `cmd || true` left, consumed-in-form and dead-in-fact) and blocks
    /// every reader that could tell the difference.
    ///
    /// The conjunct lives in the MINT rather than beside its caller because it is a property of
    /// the verb, not of one decision path: before effective world reach the guard rung was
    /// reachable only from `EstablishProbeWritten`, whose rc-consuming population happened to be
    /// empty in the corpus, so the omission was invisible.
    #[must_use]
    pub fn mint(
        fact: FactKey,
        vouch: ByVouch<VerdictVouch>,
        probe_verdict: Verdict,
        consumed: &May<Powerset<Channel>>,
    ) -> Option<GuardLicense> {
        if probe_verdict != Verdict::Converged {
            return None;
        }
        if !consumption_ok(consumed, Predicted::Top) {
            return None;
        }
        Some(GuardLicense {
            fact,
            insert: GuardInsert {
                vouch: vouch.into_vouch(),
                probe_verdict,
            },
            probe: None,
        })
    }

    /// Mint the guard ONE ELISION REGION admits, over the region's AUTHORED argv expression
    /// (`30L` §4.5 — the divergent-instances valve).
    ///
    /// Two differences from [`mint`](Self::mint), and the asymmetry between them is the point.
    ///
    /// The INVOCATION is the region's source-level words rather than this instance's resolved
    /// operands, because one authored region has one set of bytes and there is no second author to
    /// answer for a specialized copy (`30L:rul-no-specialized-shell`). Every enumerated instance's
    /// own argv passed the author's argparse when its vouch was reached, and the census is CLOSED —
    /// so every value `"$1"` can hold at runtime is one the author already accepted.
    ///
    /// The `Converged` conjunct RELAXES to a DEFINITE measurement, and only that far. The vouch
    /// demand, the consumption gate at ⊤, and the `|| <original bytes>` fall-through are untouched,
    /// so nothing about what may execute changes; what changes is which region is worth paying a
    /// check for. See [`crate::region_guard_candidate`] for why that reads differently at a region
    /// than at a site.
    ///
    /// `Unknown` still refuses, and that boundary is where the widening stops. A DIVERGED fact is a
    /// measurement — the world answered, and it answered "not yet" — so a sibling invocation that
    /// answered "already" is real value the guard recovers. An UNKNOWN fact is the absence of an
    /// answer, and paying a check to discover what the probe could not is the unsure direction
    /// (`inv-kfail`). It is also what keeps the UNMEASURED world byte-identical: with no records
    /// every cell is `Unknown`, so no region guards and no book acquires a preamble it did not have.
    #[must_use]
    pub(crate) fn mint_for_shared_region(
        fact: FactKey,
        vouch: ByVouch<VerdictVouch>,
        probe_verdict: Verdict,
        consumed: &May<Powerset<Channel>>,
        source_argv: &str,
    ) -> Option<GuardLicense> {
        if probe_verdict == Verdict::Unknown {
            return None;
        }
        if !consumption_ok(consumed, Predicted::Top) {
            return None;
        }
        let mut vouch = vouch.into_vouch();
        vouch.invocation = if source_argv.is_empty() {
            vouch.fn_name.clone()
        } else {
            format!("{} {source_argv}", vouch.fn_name)
        };
        Some(GuardLicense {
            fact,
            insert: GuardInsert {
                vouch,
                probe_verdict,
            },
            probe: None,
        })
    }

    /// This guard's decision-relevant bytes — the identity a shared region meets on.
    pub(crate) fn canonical(&self) -> String {
        self.insert.canonical()
    }

    /// Re-stamp the plan-time probe word this guard DISCLOSES.
    ///
    /// Display-only (`GuardInsert::probe_word`): the guard re-decides live at apply and never trusts
    /// this. A shared region's instances can answer differently, and no single word is true of all
    /// of them, so the settlement stamps `Unknown` — "cant-tell" — rather than picking one
    /// instance's answer to speak for the rest.
    pub(crate) fn with_probe_verdict(mut self, verdict: Verdict) -> Self {
        self.insert.probe_verdict = verdict;
        self
    }

    /// Attach the probe-side attribution post-mint — the guard's twin of
    /// [`ReplaceLicense::with_probe_attribution`], with the same weld: set AFTER the decision, read
    /// only by the why render, never an input to anything.
    #[must_use]
    pub fn with_probe_attribution(mut self, attribution: Option<ProbeAttribution>) -> Self {
        self.probe = attribution;
        self
    }

    /// The record that reported the fact this guard re-verifies, when exactly one did.
    #[must_use]
    pub fn reported(&self) -> Option<ReportedObservation> {
        self.probe.and_then(|p| p.reported)
    }

    /// The fact this guard re-verifies (attribution / the why-lens).
    #[must_use]
    pub fn fact(&self) -> FactKey {
        self.fact
    }

    /// The guard insertion (the emitter data — the render reads it).
    #[must_use]
    pub fn insert(&self) -> &GuardInsert {
        &self.insert
    }
}

/// The per-site **vouch map** the guard mint consumes (rul-guard-license / rul24-vouch-is-verdict
/// -authoring, 24A §1c): each site whose provider authored a verdict function that REACHES a
/// vouching path for the site's constant-propagated argv (`evaluate_verdict` ⇒ `Vouched`) gets one
/// [`ByVouch<VerdictVouch>`], keyed by its [`CfgNodeId`]. A site ABSENT from the map has no
/// reached vouch ⇒ it never guards (no vouch ⇒ run — the map's judgment tier is exactly what
/// [`GuardLicense::mint`] DEMANDS, TC-tier-2; a fact or silence claim cannot populate it). The cli
/// edge builds it ALWAYS-ON — guards are the un-flagged baseline (rul24-mode-gate; NOT
/// `--risk-faultless-skips`-gated, which governs only the survival tier).
/// Reached verdict vouches indexed by their exact establish identity.
///
/// Production lifting happens in [`build_vouches`]; public construction remains test/harness
/// surface pending the phase-five production fence.
#[derive(Debug, Clone, Default)]
pub struct Vouches {
    by_establish: BTreeMap<(CfgNodeId, FactKey), ByVouch<VerdictVouch>>,
    order: Vec<(CfgNodeId, FactKey)>,
    duplicates: BTreeSet<(CfgNodeId, FactKey)>,
}

impl Vouches {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, site: CfgNodeId, fact: FactKey, vouch: ByVouch<VerdictVouch>) {
        let key = (site, fact);
        if let std::collections::btree_map::Entry::Vacant(entry) = self.by_establish.entry(key) {
            entry.insert(vouch);
            self.order.push(key);
        } else {
            self.duplicates.insert(key);
        }
    }

    #[must_use]
    pub fn get(&self, site: CfgNodeId, fact: FactKey) -> Option<&ByVouch<VerdictVouch>> {
        self.by_establish.get(&(site, fact))
    }

    #[must_use]
    pub fn contains_site(&self, site: CfgNodeId) -> bool {
        self.by_establish
            .keys()
            .any(|(candidate, _)| *candidate == site)
    }

    pub fn extend(&mut self, other: Self) {
        self.duplicates.extend(other.duplicates.iter().copied());
        for (site, fact) in other.order {
            if let Some(vouch) = other.by_establish.get(&(site, fact)) {
                self.insert(site, fact, vouch.clone());
            }
        }
    }

    fn ordered_keys(&self) -> Vec<(CfgNodeId, FactKey)> {
        self.order.clone()
    }

    fn is_duplicate(&self, site: CfgNodeId, fact: FactKey) -> bool {
        self.duplicates.contains(&(site, fact))
    }
}

fn resolve_vouch_operands(
    words: &[ValueOf],
    fact: FactKey,
    member_specialization: bool,
    interner: &Interner,
) -> Option<Vec<String>> {
    let dynamic_count = words
        .iter()
        .filter(|word| matches!(word, ValueOf::Top(_)))
        .count();
    words
        .iter()
        .map(|word| match word {
            ValueOf::Literal(symbol) => Some(interner.resolve(*symbol).to_owned()),
            ValueOf::Top(_) if member_specialization && dynamic_count == 1 => match fact.entity {
                EntityRef::Operand(token) => Some(interner.resolve(token.0).to_owned()),
                EntityRef::Singleton => None,
            },
            ValueOf::Top(_) => None,
        })
        .collect()
}

/// Lift the per-site VOUCHES (24D §3 elide-weld / rul-guard-license / rul24-vouch-is-verdict-
/// authoring) — the ONE home for the composition every driver shares (the cli, the sweep net, the
/// coverage dashboard, the hostsim DST). For each establish-bearing site whose provider authored a
/// verdict function (`<provider>.is_converged`) that REACHES a vouching path over
/// the site's constant-propagated argv (`evaluate_verdict` ⇒ `Vouched`), it builds one
/// [`ByVouch<VerdictVouch>`] keyed by the site's [`CfgNodeId`]. A site ABSENT from the map has no
/// reached vouch ⇒ it never elides (the elide-weld, [`ReplaceLicense::prove_replaceable`]) and
/// never guards (no vouch ⇒ run) — the judgment tier the map carries is exactly what the mints
/// DEMAND (TC-tier-2). A `return N` decline, a ⊤/non-literal argv, or no verdict function ⇒
/// absence. Fail-soft ([`Carrier`]): the verdict-lift diagnostics ride out for the caller to
/// surface (the cli's gate-3 error-floor; the DSTs drop them). `inv-referent-agnostic`: the kind
/// label + operands are resolved for the invocation/attribution, never decoded (the 24A §1b fence).
pub fn build_vouches(
    oracle_srcs: &[&str],
    oracle_paths: &[&str],
    helpers: &dorc_oracle::closure::HelperIndex,
    classes: &[(CfgNodeId, SkipClass)],
    value: &ValueFlow,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> (Carrier<Vouches>, VouchLiftAid) {
    let mut diags = Vec::new();
    let verdict_sets: Vec<dorc_oracle::verdict::VerdictSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = dorc_oracle::verdict::VerdictSet::lift(interner, src);
            diags.extend(lifted.diags);
            lifted.value
        })
        .collect();
    let (lifted, aid) = build_vouches_from_sets(
        oracle_srcs,
        oracle_paths,
        &verdict_sets,
        helpers,
        classes,
        value,
        interner,
        live,
    );
    diags.extend(lifted.diags);
    (Carrier::new(lifted.value, diags), aid)
}

/// What the vouch lift has to SAY beside the vouches it minted: the decision-inert narrative every
/// collapse owes (`law-collapse-mints-narrative`) and the composition suspensions a user must see.
///
/// The suspensions carry their oracle-FILE index beside the `Diag`, because a `Diag` holds a span and
/// spans are only file-qualified by their caller (`AID:law-lineno-identity`) — the same shape the
/// load-edge helper-collision report uses. Deduped by (name, reason, voucher): one composition, one
/// sentence, however many sites lost their license to it.
#[derive(Debug, Default)]
pub struct VouchLiftAid {
    /// One record per suspended SITE — the narrative plane counts collapses, not compositions.
    pub narrative: Vec<CollapseNarrative>,
    /// The reportable suspensions, in `(name, reason, oracle-file)` order.
    pub suspensions: Vec<(usize, Diag)>,
}

/// `path:line` for the declaration a shell would bind, or empty where the denial named no site.
///
/// The paths are the CALLER's, which is why they are threaded in rather than derived: one
/// line-number space, the source file's, everywhere (`AID:law-lineno-identity`). A lane that holds
/// no paths passes none and the operand stays empty — the two cross-custody reasons are the ones
/// `30I` §3.4 wants it for, and only the real drivers raise them.
fn live_locus(paths: &[&str], srcs: &[&str], sites: &[(usize, dorc_core::Span)]) -> String {
    let Some(&(file, span)) = sites.last() else {
        return String::new();
    };
    let (Some(path), Some(src)) = (paths.get(file), srcs.get(file)) else {
        return String::new();
    };
    let (line, _) = dorc_aid::diag::line_col(src, span.lo.0 as usize);
    format!("{path}:{line}")
}
/// The [`dorc_aid::diag::VouchedCompositionReason`] one closure denial maps to. Two vocabularies, one
/// crossing seat: `dorc-oracle` decides WHY a composition carries no license, `dorc-aid` owns how a
/// reason is said, and neither imports the other's enum.
fn suspension_reason(
    reason: dorc_oracle::closure::DenialReason,
) -> dorc_aid::diag::VouchedCompositionReason {
    use dorc_aid::diag::VouchedCompositionReason as Said;
    use dorc_oracle::closure::DenialReason as Found;
    match reason {
        Found::BookRedefinesHelper => Said::BookRedefinesHelper,
        Found::BookShadowsCommand => Said::BookShadowsCommand,
        Found::DependencySelectedButUnaligned => Said::DependencySelectedButUnaligned,
        Found::DependencyAmbientOrUntraceable => Said::DependencyAmbientOrUntraceable,
        Found::ContestedWithinCustody => Said::ContestedWithinCustody,
        Found::UnresolvedLoad => Said::UnresolvedLoad,
        Found::UnenumerableCall => Said::UnenumerableCall,
    }
}

/// [`build_vouches`] over already-lifted per-file
/// [`VerdictSet`](dorc_oracle::verdict::VerdictSet)s — the `VerdictIndex::from_sets` shape, for
/// the same reason it exists there and one more.
///
/// The driver's sets are the WITHDRAWN ones (contested families, and per `28M` §9 the definitions
/// the function environment proves are never live). Re-lifting here read a fourth population off
/// the raw source text, so a definition every other seat had dropped still won this seat's
/// whole-unit answer — and the positional gate below then refused the vouch, which is a silent
/// wall nothing else in the run agreed with (`28P:fnd-build-vouches-relifted-the-verdict-sets`).
#[expect(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    reason = "the ONE composition every driver shares (vouch lift + decline-narrative mint); \
              splitting it would scatter the single vouch-authoring path. Each argument is a \
              distinct world the lift reads, and the paths are the caller's by law \
              (`AID:law-lineno-identity`)"
)]
pub fn build_vouches_from_sets(
    oracle_srcs: &[&str],
    oracle_paths: &[&str],
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    helpers: &dorc_oracle::closure::HelperIndex,
    classes: &[(CfgNodeId, SkipClass)],
    value: &ValueFlow,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> (Carrier<Vouches>, VouchLiftAid) {
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    use dorc_oracle::verdict::{
        VERDICT_SUFFIX, VerdictResolution, VerdictSet, check_commands, classify_decline,
        evaluate_verdict, vouch_site,
    };

    let diags: Vec<Diag> = Vec::new();
    // C5 (`27V` Lane A): the decision-inert VerdictDecline narrative beside the no-vouch-⇒-run collapse.
    let mut collapse_narrative: Vec<CollapseNarrative> = Vec::new();
    let mut suspensions: BTreeMap<(String, dorc_aid::diag::VouchedCompositionReason, usize), Diag> =
        BTreeMap::new();

    let mut vouches = Vouches::new();
    // `leaf_idx` IS the site's `LeafId` — the SAME positional assignment `build_plan` makes, so a
    // `VerdictDecline` keys by the site a report-lane record re-keys to (pinned by the leaf-index test).
    let candidates: Vec<(usize, CfgNodeId, FactKey, bool)> = classes
        .iter()
        .enumerate()
        .flat_map(|(leaf_idx, (node, class))| match class {
            SkipClass::EstablishProbeAmbient(f) | SkipClass::EstablishProbeWritten(f) => {
                vec![(leaf_idx, *node, *f, false)]
            }
            SkipClass::EstablishMembers { members, .. } => members
                .iter()
                .map(|fact| (leaf_idx, *node, *fact, true))
                .collect(),
            SkipClass::InlineCall { sites } => sites
                .iter()
                .filter_map(|site| match site.class {
                    SkipClass::EstablishProbeAmbient(fact)
                    | SkipClass::EstablishProbeWritten(fact) => {
                        Some((leaf_idx, site.node, fact, false))
                    }
                    _ => None,
                })
                .collect(),
            SkipClass::QueryResolvable { .. } | SkipClass::MustRun => Vec::new(),
        })
        .collect();
    for (leaf_idx, node, fact, member_specialization) in candidates {
        // Resolve the site's argv → (provider, operands), all literal — a ⊤ word ⇒ no vouch.
        let argv = value.argv_values(node);
        let Some((first, rest)) = argv.split_first() else {
            continue;
        };
        let ValueOf::Literal(provider) = first else {
            continue;
        };
        let Some(op_texts) = resolve_vouch_operands(rest, fact, member_specialization, interner)
        else {
            continue;
        };
        let op_refs: Vec<&str> = op_texts.iter().map(String::as_str).collect();

        // Find the provider's verdict funcdef (shared hyphen↔underscore convention) and trace it.
        // The file INDEX rides along so an arm span crossing to the render carries its file
        // identity (`tc-oracle-file-identity`).
        // The LIVE verdict definition (`28K` §1) AT THIS SITE (`28K` §2), from the SHARED seats —
        // authoring the verdict IS the vouching act, so one a shell has not reached cannot vouch
        // for a line above it (`28M:fnd-verdict-resolution-duplicates-live-source`).
        let want = map_provider_name(interner.resolve(*provider));
        let verdict_name = format!("{want}{VERDICT_SUFFIX}");
        let named = |set: &VerdictSet| {
            set.providers()
                .find(|p| map_provider_name(interner.resolve(*p)) == want)
                .and_then(|p| set.get(p).cloned())
        };
        let found = dorc_core::answering_row(
            live.definition_before(node, &verdict_name),
            verdict_sets.len(),
            |i| {
                verdict_sets
                    .get(i)
                    .and_then(named)
                    .map(|v| dorc_analysis::funcenv::row_definition(i, v.span))
            },
        )
        .and_then(|i| {
            let set = verdict_sets.get(i)?;
            Some((i, *oracle_srcs.get(i)?, named(set)?))
        });
        let Some((file_idx, src, verdict)) = found else {
            continue;
        };
        let verdict = &verdict;
        let arm_file = SourceFileId(u32::try_from(file_idx).unwrap_or(u32::MAX));
        // The reached-path license (rul-guard-license): ONLY a Vouched resolution mints. A Declined
        // (unhandled path / an inert builtin / a non-converged `return` — hz-refusepath) or ⊤ ⇒ no
        // vouch ⇒ run.
        if !matches!(
            evaluate_verdict(verdict, &op_refs),
            VerdictResolution::Vouched
        ) {
            // Narrate a genuine DECLINE: the gate + precise arm span (C7; `Unreached` ⇒ name_span
            // fallback), and the tier-2 class + emitting-arm span if the reached path emitted one
            // (`27W` §3; a dynamic argv/format leaves `authored_reason` `None` ⇒ tier-3).
            if let Some(info) = classify_decline(verdict, &op_refs) {
                let authored_reason = info.emission.map(|(class, emit_span)| AuthoredReason {
                    class,
                    arm: MintSpan(emit_span),
                    arm_file,
                });
                collapse_narrative.push(CollapseNarrative::new(
                    SpeechAct::Vouched,
                    CollapseKind::VerdictDecline {
                        site: dorc_aid::diag::SiteId::leaf(LeafId(
                            u32::try_from(leaf_idx).unwrap_or(u32::MAX),
                        )),
                        arm: MintSpan(info.arm_span.unwrap_or(verdict.name_span)),
                        arm_file,
                        gate: info.gate,
                        authored_reason,
                    },
                ));
            }
            continue;
        }

        let fn_name = format!(
            "{}{VERDICT_SUFFIX}",
            dorc_oracle::to_funcname_segment(interner.resolve(verdict.provider)),
        );
        // `28K` §4: the guard runs the definition's bytes PLUS its closure. A contested closure
        // withholds the VOUCH — no guard, no elide, the site runs (`inv-kfail`).
        let stripped = strip_verdict(src, verdict, interner);
        let closure = match helpers.closure_for(file_idx, &stripped) {
            Ok(closure) => closure,
            // The composition that will RUN is not the one this author vouched
            // (`28R:rul-mixed-custody-suspends-vouch`): no elide, no guard, the site runs. Narrated at
            // every step, and reported once per (name, reason, voucher) — a per-SITE report would be a
            // correlated cascade over one collision (`28O:dec-one-diagnostic-per-file-not-per-item`).
            Err(denial) => {
                let reason = suspension_reason(denial.reason);
                collapse_narrative.push(CollapseNarrative::new(
                    SpeechAct::Declined,
                    CollapseKind::CompositionSuspended {
                        site: dorc_aid::diag::SiteId::leaf(LeafId(
                            u32::try_from(leaf_idx).unwrap_or(u32::MAX),
                        )),
                        vouching: MintSpan(verdict.name_span),
                        vouching_file: arm_file,
                        reason,
                    },
                ));
                suspensions.insert(
                    (denial.name.clone(), reason, file_idx),
                    Diag::new(
                        dorc_aid::diag::DiagCode::VouchedCompositionNotPresent(
                            dorc_aid::diag::VouchedCompositionNotPresent {
                                name: denial.name,
                                reason,
                                live: live_locus(oracle_paths, oracle_srcs, &denial.sites),
                            },
                        ),
                        verdict.name_span,
                    ),
                );
                continue;
            }
        };
        let invocation = if op_refs.is_empty() {
            fn_name.clone()
        } else {
            format!("{fn_name} {}", op_refs.join(" "))
        };
        let kind_label = interner.resolve(fact.kind.0).to_owned();
        // The dual-rail ledger allowlists what the SHIPPED check runs — the closure included.
        let mut check_cmds = check_commands(verdict);
        check_cmds.extend(closure.commands.iter().cloned());
        // C7: the reached vouching-arm span (or `name_span` for a check-less `return 0` vouch) +
        // its oracle-file id, for the guard render.
        let defining = vouch_site(verdict, &op_refs).unwrap_or(verdict.name_span);
        // The SAME index the agreement gate above admitted, so the two cannot disagree.
        let vouch = VerdictVouch::new(
            fn_name,
            stripped,
            invocation,
            kind_label,
            check_cmds,
            dorc_analysis::funcenv::custody_of_source_index(file_idx),
        )
        .with_closure(&closure)
        .with_defining_span(defining, arm_file);
        vouches.insert(node, fact, ByVouch::vouched(vouch, Rung::Both));
    }
    (
        Carrier::new(vouches, diags),
        VouchLiftAid {
            narrative: collapse_narrative,
            suspensions: suspensions
                .into_iter()
                .map(|((_, _, file), diag)| (file, diag))
                .collect(),
        },
    )
}

/// Mint the elide/guard VOUCHES for wrapped-ENTERING BOOK sites (`27C` §3 / lane-integration
/// `27N`). A wrapped site's argv[0] is the WRAPPER word (`sudo`), so [`build_vouches`] — which keys
/// the verdict on argv[0] — cannot vouch it. Here the vouch is minted from the INNER oracle's
/// verdict reached over the site's PEELED argv (the same reached-path license, `rul-guard-license`),
/// gated on the consent decision already having permitted entry ([`WrappedProbe::Enter`]). The
/// vouch's guard data is the ENTRY-COMPOSED invocation (`sudo__enter pipx__is_converged install
/// poddle`) so an in-context guard renders correctly; the elide (`Replace`) consumes only the
/// license. Merge the result into [`build_vouches`]'s map at the cli edge (wrapped nodes are
/// disjoint from ambient ones).
///
/// It takes the DRIVER's already-lifted sets for the reason
/// [`build_vouches_from_sets`] does: re-lifting from raw source read a population every other seat
/// had already narrowed, so this seat's whole-unit winner could be a definition the run had
/// withdrawn — and the positional gate would then withhold, a silent wall nothing else in the run
/// agreed with (`28P:fnd-build-vouches-relifted-the-verdict-sets`, the same fault one seat further
/// out).
///
/// Its resolution scanned FORWARD until `28P:fnd-the-wrapped-vouch-seat-resolved-forwards`
/// (first-definition-wins, the INVERSE of sh's answer) and is now NARROWED positionally as well —
/// the sixth and last seat to join the regime (`28P:tc-wrapped-vouch-seat-has-no-positional-gate`).
/// The whole-unit winner vouches here only where it is the definition a shell would have live AT
/// the wrapped site: the withhold-not-re-resolve shape bitem0 ruled
/// (`analysis/CLAUDE.md visibility-is-full-positional`), through bitem3's ONE custody crossing.
/// Custody was already honest about WHOSE judgment speaks; this makes it honest about WHERE.
#[must_use]
pub fn build_wrapped_vouches(
    verdict_sets: &[dorc_oracle::verdict::VerdictSet],
    classes: &[(CfgNodeId, SkipClass)],
    wrapped: &WrappedProbes,
    interner: &mut Interner,
    live: dorc_analysis::funcenv::LiveDefinitions<'_>,
) -> Vouches {
    use dorc_oracle::predict::map_provider_name;
    use dorc_oracle::verdict::{
        VERDICT_SUFFIX, VerdictResolution, check_commands, evaluate_verdict,
    };

    let mut vouches = Vouches::new();
    for (node, wp) in wrapped {
        // Enter and Carry both mint an elide/guard vouch from the inner verdict over the peeled argv
        // (`27C` §3/§4(a)). Carry's `composed.enter_defs` is empty ⇒ the guard shape is the AMBIENT
        // inner check guarding the book bytes (measure ambient, carry across the substrate boundary).
        let (WrappedProbe::Enter { provider, composed }
        | WrappedProbe::Carry { provider, composed }) = wp
        else {
            continue; // a Degrade site runs — no vouch
        };
        let fact = classes.iter().find_map(|(n, c)| match c {
            SkipClass::EstablishProbeAmbient(f) | SkipClass::EstablishProbeWritten(f)
                if n == node =>
            {
                Some(*f)
            }
            _ => None,
        });
        let Some(fact) = fact else { continue };
        // The inner verdict, reached over the PEELED argv (operands after the inner command word).
        let op_refs: Vec<String> = composed
            .inner_argv
            .iter()
            .map(|s| interner.resolve(*s).to_owned())
            .collect();
        let op_slices: Vec<&str> = op_refs.iter().map(String::as_str).collect();
        let verdict_name = format!(
            "{}{VERDICT_SUFFIX}",
            map_provider_name(interner.resolve(*provider))
        );
        let Some(file_idx) = dorc_core::answering_row(
            live.definition_before(*node, &verdict_name),
            verdict_sets.len(),
            |i| {
                verdict_sets
                    .get(i)
                    .and_then(|set| set.get(*provider))
                    .map(|v| dorc_analysis::funcenv::row_definition(i, v.span))
            },
        ) else {
            continue;
        };
        let Some(verdict) = verdict_sets
            .get(file_idx)
            .and_then(|set| set.get(*provider))
        else {
            continue;
        };
        if !matches!(
            evaluate_verdict(verdict, &op_slices),
            VerdictResolution::Vouched
        ) {
            continue; // the inner verdict declines over this argv ⇒ no elide license (run)
        }
        // The entry-composed guard shape (`27C` §5): `sudo__enter … pipx__is_converged <argv>`. The
        // preamble carries every shipped funcdef (enter forms + the inner verdict body), all oracle
        // bytes (`271:rul-only-oracle-bytes-ship`).
        let mut invocation: Vec<String> =
            composed.enter_defs.iter().map(|(f, _)| f.clone()).collect();
        invocation.push(composed.inner_fn.clone());
        invocation.extend(op_refs.iter().cloned());
        let mut preamble = String::new();
        for (_, def) in &composed.enter_defs {
            preamble.push_str(def);
            preamble.push('\n');
        }
        preamble.push_str(&composed.inner_sh);
        // The INNER author's: a wrapper entry form is TRANSPORT (it cannot enter ⇒ the site runs),
        // and only the inner body's JUDGMENT licenses — so `28M` §8's monologue holds by that.
        let vouch = VerdictVouch::new(
            composed.inner_fn.clone(),
            preamble,
            invocation.join(" "),
            interner.resolve(fact.kind.0).to_owned(),
            check_commands(verdict),
            dorc_analysis::funcenv::custody_of_source_index(file_idx),
        );
        vouches.insert(*node, fact, ByVouch::vouched(vouch, Rung::Both));
    }
    vouches
}

/// What the plan does with one leaf.
#[derive(Debug, Clone)]
pub enum Disposition {
    /// Run the leaf — its effect is needed, its convergence is unknown, or an
    /// unvouched observable it emits is consumed downstream.
    Run,
    /// Replace the leaf with a value-preserving [`StandIn`] reproducing its exact
    /// observed exit status — authorised by a [`ReplaceLicense`] (convergence-
    /// elision), the only way to reach here. The `StandIn` is the `19A §5`
    /// refinement: `true`/`false`/`(exit n)`, NOT always `:`.
    Replace(ReplaceLicense, StandIn),
    /// Omit the leaf: the apply abstract-interpreter (the fold) proved it lies in a
    /// **provably-dead** branch — a `&&`/`||`/`if`/`!` whose controlling leaf has a
    /// *known* exit status that short-circuits past this leaf (`19B` build-1, the
    /// fold). Distinct from [`Replace`](Disposition::Replace): a `Replace` reproduces
    /// a status a consumer reads; an `Omit`ted leaf is *unreachable*, so it has no
    /// status to reproduce. Carries the controlling leaf's [`AstId`] (the render gate
    /// looks up the controller's disposition by it; provenance only).
    ///
    /// `inv-kfail`: an `Omit` is minted ONLY when the controlling rc is KNOWN (a
    /// probed observable); an unknown/⊤ controller never folds (the branch stays
    /// live ⇒ run). Rendering an `Omit` is additionally gated on the controller being
    /// itself neutralised (Replace/Omit), so the artifact never re-evaluates a kept,
    /// possibly-stale guard against an omitted body (`render_apply`). "Kept" includes
    /// a render-REFUSED Replace (a heredoc-bearing controller): [`is_neutralised`]
    /// consults the same refusal predicate as the render, so a licensed-but-refused
    /// guard keeps its dependent body verbatim too.
    Omit { controller: AstId },
    /// **Guard** the leaf (rul-ternary-verdict's third verb) — insert the oracle's own verdict
    /// check before the original bytes: `( <check> ) || <original>`. The original command SURVIVES
    /// VERBATIM as the `||`-right; on a live PASS it is skipped, on fall-through it runs for real.
    /// Authorised by a [`GuardLicense`], the only way to reach here (no vouch ⇒ no license ⇒ run).
    /// Distinct from [`Replace`](Disposition::Replace): a `Replace` value-substitutes an
    /// already-proven elision; a `Guard` DEFERS the decision to a fresh apply-time re-check (the
    /// past-a-wall fallback — the plan-time convergence can no longer be trusted, so the guard
    /// re-reads live). Mints no values (the crisis-closure carve-out, `inv-probe-sourced-values`).
    Guard(GuardLicense),
}

/// One leaf of the plan: its stable id, its source back-map (`dn-3`), the verbatim
/// sh it would run, and the run/skip disposition.
#[derive(Debug, Clone)]
pub struct Step {
    pub leaf: LeafId,
    pub ast: AstId,
    pub sh: String,
    pub disposition: Disposition,
}

/// The survival-tier instrumentation for one plan run (24F §3a — the may-alias fire-rate). The
/// yardstick surfaces `may_alias_fires`: how many converged elisions demoted because a same-kind
/// pair could NOT be canonicalized (the resolver ⊤'d / dangled / was absent). A SWAMPED count is a
/// finding to REPORT (the resolver default is too weak / the resolver is broken), never a license
/// to silently flip the may-alias default back to token-equality (§3a). EXEMPT from the decision
/// digest — render-surface instrumentation, like the survival witness (the erasability canon reads
/// only `steps`).
#[derive(Debug, Clone, Default)]
pub struct SurvivalReport {
    may_alias_fires: u32,
    /// 24G Part B — the `reaches()` poison attributions: each `(demoted leaf, reach-function KIND)` where
    /// a converged elision demoted because a `<kind>.reaches()` EXPANSION coordinate HIT its backing
    /// (the cross-author demote). The why-lens surfaces "site N: poisoned via `<kind>.reaches()`".
    /// Empty when no reach expansion poisoned an elision.
    reach_poisonings: Vec<(LeafId, KindId)>,
    /// C5 aid plane (`27V` Lane A): the decision-inert `WallFormation` / `Demotion` narratives the
    /// survival walk mints beside its dispositions (`two-plane-aid-law`; steers nothing). Mint-pass
    /// ordered (`inv-determinism`); threaded to the why-lens seam by the cli (d4 renders).
    collapse_narrative: Vec<CollapseNarrative>,
    /// `300:lane-sparing-rederivation` — each `(demoted leaf, crossed-wall ordinal)` where the
    /// reference model declined to confirm a survival the wall walk had already minted. EMPTY is
    /// the healthy state and the corpus's standing expectation: a non-empty entry means our two
    /// implementations of one algebra disagreed, which is a finding about OUR engine, never the
    /// book's text. The cli renders it as `survival-rederivation-disagreement`.
    rederivation_demotions: Vec<(LeafId, u32)>,
}

impl SurvivalReport {
    /// How often a converged elision demoted to run because a same-kind pair could not be
    /// canonicalized (24F §3a). The yardstick reads this; a swamped value is a finding.
    #[must_use]
    pub fn may_alias_fires(&self) -> u32 {
        self.may_alias_fires
    }

    /// The `reaches()` poison attributions (24G Part B): `(demoted leaf, reach-function KIND)` per
    /// converged elision a reach-expanded coordinate demoted. The cli's why-lens names the
    /// reach-function for each ("…poisoned via `<kind>.reaches()`").
    pub fn reach_poisonings(&self) -> impl Iterator<Item = (LeafId, KindId)> + '_ {
        self.reach_poisonings.iter().copied()
    }

    /// The C5 wall/demotion collapse-narratives (`27V` Lane A): decision-inert records the cli unions
    /// onto the why-lens seam. Read-only display tier (`two-plane-aid-law`).
    #[must_use]
    pub fn collapse_narrative(&self) -> &[CollapseNarrative] {
        &self.collapse_narrative
    }

    /// The re-derivation demotions (`300:lane-sparing-rederivation`): `(demoted leaf, crossed-wall
    /// ordinal)` per survival the reference model declined to confirm. Empty is the healthy state.
    pub fn rederivation_demotions(&self) -> impl Iterator<Item = (LeafId, u32)> + '_ {
        self.rederivation_demotions.iter().copied()
    }
}

/// A whole-book plan: an ordered list of leaf [`Step`]s (the leaf-seam — never a
/// single opaque script). Render with [`render_sh`](Plan::render_sh). Carries the survival-tier
/// [`SurvivalReport`] (24F §3a instrumentation — the may-alias fire-rate; digest-exempt).
/// One AUTHORED REGION of the plan: the definition-keyed span, the verbatim sh inside it, the ONE
/// decision every invocation instance agreed to, and how many instances that is (`plans/30L` §8).
///
/// Not a [`Step`], and the separation is `30L:rul-two-identities-never-conflated`: a `Step` is
/// EXECUTION identity (one `LeafId`, one probe record, one run), while a region is EDIT identity —
/// many executions, exactly one authored span to rewrite. Folding regions into `steps` would either
/// mint leaf ids for spans that never execute as leaves, or collapse instances the site-keyed
/// results lane must keep apart (`inv-site-keyed-results` · `inv-leaf-seam`).
#[derive(Debug, Clone)]
pub struct RegionStep {
    pub region: dorc_core::region::ElisionRegion,
    pub ast: AstId,
    pub sh: String,
    pub disposition: Disposition,
    /// Every statically possible invocation instance this ONE edit is universal over — the route
    /// count the pull and why surfaces show, and the call identities `dorc why` walks
    /// (`30L` §8/§9).
    pub routes: dorc_core::spine::Account<dorc_core::spine::RegionRoute>,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub steps: Vec<Step>,
    /// The authored regions inside function bodies this plan edits (`plans/30L`). EMPTY for a book
    /// with no eligible calls, which is what keeps such a book byte-identical
    /// (`30L:pin-empty-function-world-parity`).
    pub regions: Vec<RegionStep>,
    /// The survival-tier instrumentation (24F §3a). Empty on the flag-off / no-resolver path.
    pub survival_report: SurvivalReport,
    /// DEFENSIVE emission (`28R:rul-defensive-mode-definition-vectors`): the unit carries an
    /// unresolved in-process definition vector, so every emitted name munges rather than trusting
    /// that a bare one still means what we emitted. Whole-artifact, hence a plan field rather than a
    /// per-insert one; `false` is the overwhelming case and is what keeps the artifact bare.
    pub defensive_emission: bool,
}

/// The per-disposition tally that backs the CLI plan-summary surface (plans/240 Stage-1
/// yardstick — the round's north-star metric, *elision frequency*, made CLI-visible).
/// `sites` is the leaf total; `elide` counts [`Disposition::Replace`] (a converged/dead
/// line value-substituted away — the golden-hill verb), `omit` counts [`Disposition::Omit`]
/// (a fold-proved-dead branch), `run` the rest. `guard` is the ternary tier's bucket
/// (rul-ternary-verdict's `{elide, guard, run}`): **0 at HEAD**, because no `Disposition`
/// mints a guard until the Stage-3 guard tier — the field exists now so the summary's
/// grammar is stable across that build (a parse target must not gain a column mid-round).
/// `sites == elide + omit + guard + run` by construction.
///
/// `elide` additionally SPLITS by what the skip rested on — the distinction a receipt header owes
/// its reader, since the two carry different risk: an `elide_by_proof` skip stands on a probed
/// fact, while an `elide_by_trusted_claim` skip was kept past a RUNNING wall on an author's at-most
/// claim under the consent flag (the design's one naked-trust cell — `survive-license`). Derived
/// from the survival witness the wall walk already attached, so it needs no new decision input.
/// `elide == elide_by_proof + elide_by_trusted_claim` by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DispositionCounts {
    pub sites: usize,
    pub elide: usize,
    /// Elisions resting on a probed fact alone (no wall crossed).
    pub elide_by_proof: usize,
    /// Elisions kept past ≥1 running wall on an at-most claim, under the consent flag.
    pub elide_by_trusted_claim: usize,
    pub omit: usize,
    pub guard: usize,
    pub run: usize,
}

// ===========================================================================
// The probe (apply-2's convergence check) — DESIGN "probing phase", note 163 §1.
// The FORWARD half of the compiler: what to check so the apply can elide. The
// apply ([`build_plan`]) is driven by this probe's (simulated/real) answers.
//
// Round-20 task-D1 re-key (the WIRE — `inv-site-keyed-results`, 202 §3 / 205 §1):
// the probe is now a real, runnable, SELF-REPORTING artifact, keyed by **command
// site** (the stable [`LeafId`] back-map), not by fact. Each resolvable site invokes
// the kind's check and emits a results-record on stdout (the round-trip's return
// channel). Two same-command sites stay DISTINCT (different `LeafId`s ⇒ two records);
// the per-fact dedup of spike-2 (which collapsed them) is gone.
// ===========================================================================

/// What kind of site a [`ProbePredict`] is — the discriminant the wrong-concrete
/// firewall keys on (202 §3 / 20C §2 / task-D2). The two site-classes carry
/// **different observables in their record-rc**, and conflating them is the
/// disaster class:
/// * an `Establish` site's record-rc is the PROBE command's rc (`dpkg-query`'s),
///   NOT the mutator's (`apt-get`'s) — feeding it to the fold's Status would be a
///   confidently-wrong concrete; it is carried on the wire but feeds the fold
///   NOTHING (status stays ⊤, unconditionally).
/// * a `Query` site's record-rc is the guard's OWN rc (`command -v`'s) — it IS the
///   value the `&&`/`||`/`if`/errexit consumer reads, so it is fold-usable as the
///   Status channel, but ONLY when [`valid`](ProbeSiteKind::Query::valid)
///   (rule-query-validity, 205 §2). This asymmetry is the heart of task-D2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeSiteKind {
    /// An establish-class site: record-rc is the probe-command's rc ⇒ never the fold
    /// Status (the firewall blocks it unconditionally).
    Establish,
    /// A read-only Query guard: record-rc is the guard's own rc ⇒ fold-usable as the
    /// Status channel IFF `valid` (rule-query-validity's pristine-prefix bit). When
    /// `!valid` an upstream mutator/opaque made the resting rc stale ⇒ the caller
    /// withholds it (status ⇒ ⊤) and the guard runs for real.
    Query { valid: bool },
}

/// One oracle body a ship seam resolved for a probe site, together with the DEFINING span of the
/// funcdef it was sliced out of (`27V:mech-minting-line-threading`, extended past vouches).
///
/// The span is the `file:line` a why-chain's REPORTED row points at — the answer to "who reported
/// this?" beyond a bare `<provider>__predict` funcname. It is file-qualified for the same reason a
/// vouch span is: a bare [`dorc_core::Span`] is ambiguous once more than one oracle is loaded
/// (`law-lineno-identity`). `None` is a legitimate answer and is never filled in by guessing —
/// an entry-composed or connected-pipe body has no single defining funcdef to point at.
///
/// Decision-inert: `defining_span` is display provenance only (the erasability gate exempts it),
/// while `sh`/`emits_report` remain probe-artifact identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShippedCheck {
    /// The stripped funcdef the site ships (strip-only — `271:rul-only-oracle-bytes-ship`).
    pub sh: String,
    /// The funcdef's defining span + which loaded oracle it indexes into; `None` when unthreaded.
    pub defining_span: Option<(dorc_core::Span, SourceFileId)>,
    /// `27W` §3 tier-3: the shipped body emits report-lane lines. Only the auto-cell verdict seam
    /// ever sets this (a `__predict` model never emits reports).
    pub emits_report: bool,
}

impl ShippedCheck {
    /// A `<provider>__predict` body — never report-emitting (`27W` §3 scopes tier-3 to auto-cell).
    #[must_use]
    pub fn predict(sh: String, defining_span: Option<(dorc_core::Span, SourceFileId)>) -> Self {
        Self {
            sh,
            defining_span,
            emits_report: false,
        }
    }

    /// A `24L` §2 auto-cell `<provider>__is_converged` body, which may emit report-lane lines.
    #[must_use]
    pub fn verdict(
        sh: String,
        defining_span: Option<(dorc_core::Span, SourceFileId)>,
        emits_report: bool,
    ) -> Self {
        Self {
            sh,
            defining_span,
            emits_report,
        }
    }
}

/// One read-only check the probe ships for a **command site**: the oracle's own
/// stripped `check()` funcdef plus the site's full verbatim argv (C-1), wrapped so the
/// rendered probe, when run, emits a results-record per site (`inv-site-keyed-results`).
///
/// R3 (23D §1 / rul-ternary-verdict — the check IS the oracle): `sh` is the STRIPPED
/// check funcdef (`<provider>__predict() { … }`), shipped strip-only (annotations removed,
/// nothing else changed — [`dorc_oracle::predict::strip_predict`]) and invoked per-site with
/// the site's resolved argv (`<provider>__predict install -y nginx`). The check's own
/// argparse resolves the entity from that argv (identical rc to a `dpkg-query` package
/// site, and the AUTHORED command where it diverges — `dpkg -s --`, the firewall's
/// non-pipeline re-spelling, per ask-probe-divergence RULED (b)). The check IS the
/// oracle: there is one shipped unit in both lanes (23D §1).
///
/// `site` is the stable [`LeafId`] (== the apply plan's leaf-id for the same source
/// command), so the results-record keys back to exactly this program point. `fact` is
/// the resolved cell (display/provenance + the cli's site→fact verdict re-key).
#[derive(Debug, Clone)]
pub struct ProbePredict {
    /// The stable command-site identity (`inv-site-keyed-results`): the same
    /// [`LeafId`] the apply plan assigns the source command. Two same-command sites
    /// carry distinct ids.
    pub site: LeafId,
    /// The MEMBER index, for an in-loop Members site (task-L2 item-4): `Some(idx)` ⇒ this
    /// check is member `idx` of a fact-FAMILY, emitting a sub-keyed record `site
    /// <leafid>.<idx>`; `None` ⇒ an ordinary single-fact site, record `site <leafid>`. The
    /// member index ranges over the loop's members in list order (duplicates kept).
    pub member: Option<u32>,
    /// The resolved cell this site (or member) establishes or queries (the probe checks
    /// whether it holds). For a Members site this is the per-member cell.
    pub fact: FactKey,
    /// Establish-class or Query-class — the firewall discriminant ([`ProbeSiteKind`]).
    pub site_kind: ProbeSiteKind,
    /// R3: the book command word (argv[0]) — the provider whose stripped `check()` this
    /// site ships. Its funcname (`<provider>__predict`, [`predict_fn_name`]) keys the render's
    /// wrapper dedup, re-emitted when the body changes (a provider with >1 check body —
    /// `apt-get` as both `package` and `pkgindex`).
    pub provider: Symbol,
    /// R3: the site's argv AFTER the command word (`install`, `-y`, `nginx`), each a
    /// resolved literal — the check's own argparse resolves the entity from it. F-quoted
    /// per word at render (`inv-kfail` both directions — one inert positional each).
    pub argv: Vec<Symbol>,
    /// R3: the STRIPPED check funcdef (`<provider>__predict() { … }`), shipped verbatim
    /// (strip-only — the check IS the oracle). Re-defined before an invocation whose
    /// provider's body differs from the last emitted (sh's last-writer-wins + top-to-bottom
    /// exec makes each invocation see its own body).
    pub sh: String,
    /// `24J` §2 (repaired, `271:rul-only-oracle-bytes-ship`) — the CONNECTED-probe body.
    /// `Some(composed)` ⇒ this governing site ships a *connected* probe whose stages are each
    /// replaced by their oracle's stripped predict, piped
    /// (`otelcol__predict '--version' | grep__predict '-q' '0.155.0'`), its governing rc captured
    /// by the record scaffold — ONLY oracle-authored bytes ship (never the raw book pipeline).
    /// `None` ⇒ the ordinary single-command shape (`sh` funcdef + `argv` invocation). The
    /// `fact`/`site_kind` re-key + the record grammar are unchanged either way.
    pub connected: Option<ComposedProbe>,
    /// `24L` §2 — the typeless-floor auto-cell probe. `true` ⇒ `sh` is the stripped VERDICT body
    /// (`<provider>__is_converged`, not `__predict`), invoked with the site argv; its rc maps to
    /// the Effect verdict through the SAME record-scaffold rc-partition (0=holds, 1=absent,
    /// else=cant-tell — exactly the verdict rc-partition). The probe IS the verdict for a markless
    /// oracle. `false` ⇒ the ordinary `__predict` shape. Steers only [`ProbePlan::render_sh`]'s
    /// funcname choice; the record grammar and site-keying are identical.
    pub verdict: bool,
    /// `27C` §3 / lane-integration `27N` — the ENTRY-COMPOSED probe for a wrapped site whose consent
    /// trace permits entry. `Some` ⇒ this site's check runs the inner oracle's body INSIDE the
    /// wrapper chain's context: the entry forms wrap the inner predict/verdict, invoked with the
    /// site's peeled argv (`sudo__enter pipx__is_converged install poddle`). ONLY oracle-authored
    /// bytes ship — never the raw book bytes (`271:rul-only-oracle-bytes-ship`, extended to entry
    /// composition). `fact` carries the composed [`Context`], so the record re-keys the context-
    /// qualified verdict exactly. `None` ⇒ the ordinary (ambient) shape.
    pub entry: Option<EntryComposed>,
    /// The DEFINING span (+ oracle file) of the funcdef whose body this site ships
    /// (`27V:mech-minting-line-threading`) — what a why-chain's REPORTED row names as its speaker,
    /// in place of the reconstructed `<provider>__predict` funcname. Decision-inert display
    /// provenance: the erasability gate exempts it, so two plans differing only here digest
    /// identically. `None` for a body with no single defining funcdef (entry-composed, connected
    /// pipes) — absence is typed, never guessed.
    pub defining_span: Option<(dorc_core::Span, SourceFileId)>,
    /// `27W` §3 tier-3 (C4) — this check's shipped body EMITS report-lane lines (a `decline <class>`
    /// on a declining path). ONLY the auto-cell verdict path can be `true` (a `__predict` model never
    /// emits reports; entry/connected bodies are out of the tier-3 scope this round). When `true`,
    /// [`ProbePlan::render_sh`] ships the DRAIN scaffold: the check runs with `DREP_V1` bound inside
    /// a scratch directory Dorc exclusively created, its emissions re-framed as `report site=<key> …`
    /// records. `false` ⇒ the ordinary scaffold, byte-identical (`empty-world-byte-identical`).
    pub emits_report: bool,
}

/// The entry-composed body of a wrapped-site probe (`27C` §3 / `27N`): the wrapper chain's entry
/// forms wrapping the inner oracle's stripped body, all oracle-authored, argv-flowing. Built at the
/// cli edge; rendered by [`ProbePlan::render_sh`].
///
/// Shim seam (`274` §5 / `27L` task-14, DISCLOSED deferral): the real `sudo` boundary crossing needs
/// the per-run PATH shim to materialize the oracle bytes as executables `sudo` can exec; that I/O
/// edge is deferred. At HEAD the composition ships strip-only funcdefs + a nested invocation, so a
/// real `sudo -n <fn>` cannot resolve the funcdef and the record lands can't-say ⇒ run (safe). The
/// emission + context-qualified readback are exercised via simulated results (`PROBE_RESULTS=authored`).
#[derive(Debug, Clone)]
pub struct EntryComposed {
    /// The chain's stripped entry-form funcdefs, `(funcname, funcdef)`, OUTERMOST-FIRST
    /// (`sudo__enter`, then `chroot__enter`). Each dedup-emitted like an ordinary check body.
    pub enter_defs: Vec<(String, String)>,
    /// The inner oracle's check funcname (`pipx__is_converged` / `pipx__predict`).
    pub inner_fn: String,
    /// The inner oracle's stripped check funcdef (strip-only, `271:rul-only-oracle-bytes-ship`).
    pub inner_sh: String,
    /// The site's PEELED argv (the inner command's args, F-quoted at render) — the admin's argv
    /// flowing THROUGH the inner oracle's argparse (`rul-argv-flows-bytes-do-not`).
    pub inner_argv: Vec<Symbol>,
}

/// The probe disposition of a wrapped BOOK site (`27C` §3 / `27N`), built at the cli edge from the
/// two-axis consent decision (dial × capability × vouch × entry-form) and consulted by
/// [`compile_probe`]. A wrapped site takes THIS path exclusively: the ordinary ship (keyed on the
/// wrapper word `sudo`) would mis-resolve to the wrapper's own model.
#[derive(Debug, Clone)]
pub enum WrappedProbe {
    /// The consent trace PERMITS entry: ship the entry-composed check (the inner oracle's body
    /// inside the wrapper chain's context). `provider` is the inner provider (display/debug).
    Enter {
        /// The inner provider symbol (the `ProbePredict::provider` field; display only for entry).
        provider: Symbol,
        /// The entry-composed body (enter forms + inner check).
        composed: EntryComposed,
    },
    /// PURE-PREDICATE CARRY (`27C` §4(a); steering `pure-predicate-carry`): entry degraded, but the
    /// crossed boundary is a SUBSTRATE axis, the fact's marked backing kinds carry `invariant:<axis>`
    /// (A), and the verdict body is read-set-closed (B) — so the AMBIENT measurement answers the
    /// wrapped site, UNFLAGGED. `composed` has EMPTY `enter_defs` (measure ambient, no entry form);
    /// the cli keys the fact `Context::HostDefault`, so this ships the plain inner check and the
    /// ambient verdict answers it. A DISTINCT licensed path — `compare` is untouched
    /// (`pin-no-outcome-as-generator`).
    Carry {
        /// The inner provider symbol (display only; the `ProbePredict::provider` field).
        provider: Symbol,
        /// The AMBIENT inner check (enter forms EMPTY — measured in the host-default world).
        composed: EntryComposed,
    },
    /// The consent trace REFUSES entry (dial forbids, unvouched, no capability, ⊤ dimension, no
    /// entry form, or a runtime degrade) AND pure-predicate carry does not apply ⇒ can't-say ⇒ the
    /// site runs (unresolvable in the probe).
    Degrade,
}

/// The wrapped BOOK sites of a run, keyed by [`CfgNodeId`] (`27N`). Empty for a wrapper-free run ⇒
/// [`compile_probe`] behaves byte-identically (`empty-world-byte-identical`).
pub type WrappedProbes = BTreeMap<CfgNodeId, WrappedProbe>;

/// A compiled probe: per-resolvable-site read-only checks whose answers drive the
/// apply's elision (apply-2), plus the un-resolvable sites recorded for transparency.
/// A site is **resolvable** iff its class is [`SkipClass::EstablishProbeAmbient`] (the
/// elidable establish — note 162 O-1) OR [`SkipClass::QueryResolvable`] (a read-only
/// guard whose check IS the probe — 202 §2 / task-D2), AND its kind has a *declared*
/// read-only probe; only resolvable sites get an invocation. An un-resolvable site (a
/// kill, an opaque command, a written establish, a `MustRun`, or a resolvable class whose
/// kind has no probe) appears in the rendered artifact as a `site:<id>
/// unresolvable-no-probe` comment, never as an invocation (`kFAIL-perform`: no convergence
/// knowledge ⇒ the apply runs it).
#[derive(Debug, Clone, Default)]
pub struct ProbePlan {
    /// The resolvable sites' checks, in site-id order.
    pub checks: Vec<ProbePredict>,
    /// The un-resolvable sites' ids (rendered as `unresolvable-no-probe` comments).
    pub unresolvable: Vec<LeafId>,
    /// Why an un-resolvable site's oracle check gave up, where the tracer had a reason to give
    /// (`26G:fnd-existence-gate-darkens-oracle`, the "say so" half of `inv-top-reject`). A
    /// DIAGNOSTICS-ONLY channel beside [`unresolvable`](ProbePlan::unresolvable): it feeds the
    /// stderr `site-unresolvable` note and nothing else. Nothing may branch on it for licensing or
    /// verdicts, and the rendered probe/apply artifacts are byte-identical with it empty — a site
    /// is un-resolvable for reasons the reason-map cannot see (a kill, a `MustRun`, a resolved
    /// check with no shippable body), so a missing entry is ordinary, never a signal.
    pub unresolvable_causes: BTreeMap<LeafId, dorc_oracle::predict::TopReason>,
}

/// The check-function name for a probed site's provider: `<provider>__predict` (R3 /
/// 23D §1 — the check IS the oracle, shipped strip-only under its own funcname). The
/// book command word is normalised through the hyphen↔underscore provider convention
/// ([`dorc_oracle::predict::map_provider_name`] then [`dorc_oracle::to_funcname_segment`]),
/// so it agrees byte-for-byte with the name
/// [`strip_predict`](dorc_oracle::predict::strip_predict) mangles the funcdef to (`apt-get` ⇒
/// `apt_get__predict`). Referent-agnostic: the name is passed to the host, never branched
/// on. Two providers ⇒ two names (`apt_get__predict` / `yum__predict`, the seam); one
/// provider with two check bodies (`apt-get` as `package` and `pkgindex`) shares the
/// name — the render re-emits the body per invocation ([`ProbePlan::render_sh`]).
fn predict_fn_name(interner: &Interner, provider: Symbol) -> String {
    format!(
        "{}__predict",
        dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(
            interner.resolve(provider)
        )),
    )
}

/// The verdict funcname a typeless-floor auto-cell probe defines + invokes (`24L` §2):
/// `<provider>__is_converged`, mangled through the SAME hyphen↔underscore convention as
/// [`predict_fn_name`] so it agrees byte-for-byte with the name
/// [`dorc_oracle::predict::strip_verdict`] gives the shipped funcdef (and with the guard emitter's
/// invocation, `build_vouches`). Referent-agnostic: passed to the host, never branched on.
fn verdict_fn_name(interner: &Interner, provider: Symbol) -> String {
    format!(
        "{}{}",
        dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(
            interner.resolve(provider)
        )),
        VERDICT_SUFFIX,
    )
}

impl ProbePlan {
    /// Attach classify's per-node give-up reasons to the un-resolvable sites
    /// ([`unresolvable_causes`](ProbePlan::unresolvable_causes)) — the DIAGNOSTICS-ONLY join, kept
    /// out of [`compile_probe`] because no compilation decision reads it and every caller but the
    /// cli renders no causes. `ast`/`cfg`/`classes` must be the SAME three the plan was compiled
    /// from: the join re-derives the site ordering, so a mismatched triple would mis-key reasons
    /// onto sites (harmless to the artifacts — nothing branches on them — but a wrong cause is a
    /// worse diagnostic than none).
    #[must_use]
    pub fn with_unresolvable_causes(
        mut self,
        ast: &Ast,
        cfg: &Cfg,
        classes: &[(CfgNodeId, SkipClass)],
        degrades: &BTreeMap<CfgNodeId, dorc_oracle::predict::TopReason>,
    ) -> Self {
        let node_of: BTreeMap<LeafId, CfgNodeId> = site_order(ast, cfg, classes)
            .into_iter()
            .map(|(site, node, _)| (site, node))
            .collect();
        self.unresolvable_causes = self
            .unresolvable
            .iter()
            .filter_map(|site| Some((*site, *degrades.get(node_of.get(site)?)?)))
            .collect();
        self
    }

    /// Render the probe as a shippable, read-only, **self-reporting** shell-script
    /// (the sanitised projection shipped to gather facts — DESIGN). The artifact, WHEN
    /// RUN, emits one results-record per resolvable site on stdout — the round-trip's
    /// return channel (202 §3). The record grammar (documented in the artifact header):
    ///
    /// ```text
    /// site <leafid> effect=<holds|absent|cant-tell> rc=<n>
    /// ```
    ///
    /// `effect` is the fact-probe's three-outcome observation derived from the probe
    /// command's exit status by the oracle's existing convention (`an-probe-shape`):
    /// `0 ⇒ holds`, `1 ⇒ absent`, anything else `⇒ cant-tell`. `rc` is the raw probe
    /// rc, carried for provenance. **No exit-code semantics for Dorc verdicts**: the rc
    /// is opaque (a standing human ruling) and the record IS the out-of-band lane.
    ///
    /// CRITICAL (the wrong-concrete firewall, 202 §3 / the cli re-key): the `rc` is the
    /// PROBE command's rc (`dpkg-query`'s), NOT the book command's (`apt-get`'s). For
    /// an establish-class site these are DIFFERENT observables, so the cli carries this
    /// rc but feeds it to NOTHING in the fold (only the legacy `fold-oror-guard` Query
    /// exception folds a probe-sourced rc, via its own `declared-rc` line — D2's Query
    /// class is what will legitimately equate a guard's probe-rc with its site status).
    ///
    /// Emitted-function shape (R3 / 23D §1 — the check IS the oracle): the oracle's own
    /// stripped `<provider>__predict` funcdef, invoked **per site** with the site's full
    /// resolved argv (`apt_get__predict install -y nginx`). The check's argparse resolves
    /// the entity from that argv, so a multi-selector kind self-discriminates by verb-arm
    /// at runtime (no per-`(kind, selector)` wrapper). One provider with two distinct
    /// check bodies (`apt-get` as both `package` and `pkgindex`) re-emits each body
    /// immediately before its own invocation (sh's last-writer-wins). The wrapper captures
    /// `$?` immediately after the check, maps it to the three-outcome word, and prints the
    /// record.
    #[must_use]
    pub fn render_sh(&self, framing: &records::Framing, interner: &Interner) -> String {
        let mut out = String::from(render::probe::header());
        // `262` §2 framing header — the artifact's FIRST OUTPUT line. `sites=` is the
        // fact-lane census (the resolvable site-record count), so a truncated fact lane is a
        // computable range at the deframer (`26A` amend-smalls). The end-sentinel is emitted
        // by the round-trip driver AFTER every record lane (`records::sentinel_line`).
        out.push_str(&records::header_line(framing, self.checks.len()));
        let nonce = framing.nonce();
        let drains = self.checks.iter().any(|c| c.emits_report);
        if drains {
            out.push_str(&render::probe::report_scratch_prologue(nonce));
        }
        // R3 (23D §1 — the check IS the oracle): emit each provider's stripped
        // `<provider>__predict` funcdef, then invoke it per SITE with the site's full argv +
        // the self-report wrapper. The funcdef is deduped per funcname but RE-EMITTED
        // whenever the needed body differs from the one currently in scope: one provider
        // with two check bodies (`apt-get` as both `package` and `pkgindex`) ships each
        // body immediately before its own invocation, so sh's last-writer-wins +
        // top-to-bottom exec makes every invocation see its own body. All sh-text assembly
        // routes through `render::probe` (task-R); this loop owns the re-emit bookkeeping.
        let mut defined: BTreeMap<String, &str> = BTreeMap::new();
        for check in &self.checks {
            // `24L` §2 — an auto-cell probe defines+invokes `<provider>__is_converged` (the shipped
            // verdict body), not `<provider>__predict`. The record scaffold + rc-partition are
            // identical; only the funcname (dedup key + invocation) differs.
            let fn_name = if check.verdict {
                verdict_fn_name(interner, check.provider)
            } else {
                predict_fn_name(interner, check.provider)
            };
            // The record's site key: `N` for a single-fact site, `N.M` for member M of an
            // in-loop Members family (item-4).
            let key = render::probe::site_key(check.site, check.member);
            out.push_str(&render::probe::site_comment(
                &key,
                &fact_label(interner, check.fact),
            ));
            // `24J` §2 (repaired, `271:rul-only-oracle-bytes-ship`) — a CONNECTED governing stage
            // ships the COMPOSED predicts: each stage's stripped `<provider>__predict` funcdef
            // (dedup-emitted like the ordinary path) piped, `stage0__predict a | stage1__predict b`.
            // ONLY oracle-authored bytes ship — never the raw book pipeline. Otherwise the ordinary
            // R3 shape: (re-)emit the stripped funcdef, then invoke it with the site's argv.
            let invocation = if let Some(entry) = &check.entry {
                // `27C` §3 / `27N` — the ENTRY-COMPOSED probe: emit the chain's entry-form funcdefs
                // (outermost-first) + the inner check funcdef (dedup like the ordinary path), then a
                // NESTED invocation `sudo__enter … pipx__is_converged <argv>`. Only oracle-authored
                // bytes ship (`271:rul-only-oracle-bytes-ship`); the admin's argv flows F-quoted.
                let mut prefix: Vec<String> = Vec::with_capacity(entry.enter_defs.len() + 1);
                for (fname, fdef) in &entry.enter_defs {
                    if defined.insert(fname.clone(), fdef.as_str()) != Some(fdef.as_str()) {
                        out.push_str(&render::probe::wrapper_def(fdef));
                    }
                    prefix.push(fname.clone());
                }
                if defined.insert(entry.inner_fn.clone(), entry.inner_sh.as_str())
                    != Some(entry.inner_sh.as_str())
                {
                    out.push_str(&render::probe::wrapper_def(&entry.inner_sh));
                }
                prefix.push(entry.inner_fn.clone());
                render::probe::invocation(&prefix.join(" "), &entry.inner_argv, interner)
            } else if let Some(composed) = &check.connected {
                let mut invs = Vec::with_capacity(composed.stages.len());
                for stage in &composed.stages {
                    let stage_fn = predict_fn_name(interner, stage.provider);
                    if defined.insert(stage_fn.clone(), stage.sh.as_str())
                        != Some(stage.sh.as_str())
                    {
                        out.push_str(&render::probe::wrapper_def(&stage.sh));
                    }
                    invs.push(render::probe::invocation(&stage_fn, &stage.argv, interner));
                }
                invs.join(" | ")
            } else {
                if defined.insert(fn_name.clone(), check.sh.as_str()) != Some(check.sh.as_str()) {
                    out.push_str(&render::probe::wrapper_def(&check.sh));
                }
                render::probe::invocation(&fn_name, &check.argv, interner)
            };
            // `27W` §3 C4: an emission-bearing auto-cell body drains; every other stays byte-identical.
            if check.emits_report {
                out.push_str(&render::probe::record_scaffold_draining(
                    &invocation,
                    &key,
                    nonce,
                ));
            } else {
                out.push_str(&render::probe::record_scaffold(&invocation, &key, nonce));
            }
        }
        // Un-resolvable sites are recorded as comments (never invoked): transparency
        // for the human reading the artifact and the D3 argv-echo differential.
        for site in &self.unresolvable {
            out.push_str(&render::probe::unresolvable_comment(*site));
        }
        if drains {
            out.push_str(render::probe::report_scratch_epilogue());
        }
        out
    }

    /// The per-run PATH shim FILE SET an entry-composed probe needs to resolve its inner check
    /// across the wrapper's exec boundary (`274` §5 / `27L` task-14 — the shim-materialization last
    /// mile). An entry form (`sudo__enter() { sudo -n "$@"; }`) EXECS its guest as a fresh process,
    /// and a shell function does not survive `exec`; so `sudo__enter hork__is_converged …` cannot
    /// resolve `hork__is_converged` at the guest position unless it is a real executable on PATH.
    /// This maps each EXEC'd guest funcname → a standalone dispatch script (`#!/bin/sh` + the oracle's
    /// stripped funcdef + `<fn> "$@"`), which the cli/e2e edge materializes into a PATH-prepend dir
    /// before running the probe. ONLY oracle-authored bytes ship (`271:rul-only-oracle-bytes-ship`):
    /// the funcdef is the shipped `inner_sh`/`enter_defs` verbatim; the shebang + dispatch line are
    /// synthesized scaffolding (`probe-composition-walls` — never book bytes).
    ///
    /// The exec'd guests are every entry form AFTER the outermost (the outermost runs as a funcdef
    /// in the probe shell) plus the inner check. A single-link `sudo__enter <inner>` yields exactly
    /// one shim (the inner check). `Carry` (ambient, empty `enter_defs`) and plain ambient checks
    /// (`entry: None`) cross no boundary ⇒ no shim. Deterministic content + `BTreeMap` ordering
    /// (`inv-determinism`); empty for a wrapper-free run (`empty-world-byte-identical` — no shim dir).
    #[must_use]
    pub fn shim_files(&self) -> BTreeMap<String, String> {
        let mut files = BTreeMap::new();
        for check in &self.checks {
            let Some(entry) = &check.entry else { continue };
            if entry.enter_defs.is_empty() {
                continue; // Carry / ambient — no exec boundary (see the doc-comment).
            }
            for (fname, fdef) in entry.enter_defs.iter().skip(1) {
                files.insert(fname.clone(), shim_dispatch_script(fname, fdef));
            }
            files.insert(
                entry.inner_fn.clone(),
                shim_dispatch_script(&entry.inner_fn, &entry.inner_sh),
            );
        }
        files
    }

    /// Did the probe compile a check for `fact`? The apply may only elide a fact the
    /// probe actually checks (the "can't-probe ⇒ can't-elide" link). (Fact-keyed, not
    /// site-keyed: the DST/unit tests ask "is this cell probed at all"; the site-keyed
    /// record lane is the cli's concern.)
    #[must_use]
    pub fn checks_fact(&self, fact: FactKey) -> bool {
        self.checks.iter().any(|c| c.fact == fact)
    }
}

/// A per-run shim file's text: the oracle's stripped funcdef followed by a dispatch call that
/// forwards the guest's argv (`274` §5). Run as `<fn> a b`, it defines the function then calls it
/// with `$@ = a b` — identical positional binding to the in-process funcdef, so the oracle body
/// reads its argv exactly as it does in the probe shell.
fn shim_dispatch_script(fname: &str, fdef: &str) -> String {
    format!("#!/bin/sh\n{fdef}\n{fname} \"$@\"\n")
}

/// A ship decision for one escalated derivation site (24E §2): the stripped `<provider>__disturbs`
/// funcdef + the host tool the body reached (display locus). Returned by the cli's derive-closure,
/// which owns the oracle sources + the `evaluate_touches` escalation check — so `plan` stays
/// oracle-free (the same seam-shape as [`compile_probe`]'s `ship_body`).
#[derive(Debug, Clone)]
pub struct DerivationShip {
    /// The stripped `<provider>__disturbs` funcdef (strip-only; `dorc_oracle::predict::strip_touches`).
    pub sh: String,
    /// The host tool the touches body reached (e.g. `dpkg -L`) — a display locus for the fork-4B
    /// advisory + the `Derived` footprint origin (24E §9). `inv-referent-agnostic`: never decoded.
    pub call: String,
}

/// One compiled derivation-probe query (24E §2/§5): an escalated wall-candidate site whose
/// `touches()` body ships to the host to derive its footprint. Site-keyed
/// (`inv-site-keyed-results`) — the `deriv <leafid> coord=…` records it emits key back to `site`,
/// and `node` re-keys the derived footprint into [`TrustedFootprints`] (`CfgNodeId`-keyed) at
/// readback.
#[derive(Debug, Clone)]
pub struct ProbeDerivation {
    /// The stable command-site identity (the same [`LeafId`] the apply plan assigns).
    pub site: LeafId,
    /// The CFG node — re-keys the readback footprint into the `CfgNodeId`-keyed [`TrustedFootprints`].
    pub node: CfgNodeId,
    /// The book command word (argv[0]) whose `__disturbs` this ships.
    pub provider: Symbol,
    /// The site's argv after the command word (F-quoted at render).
    pub argv: Vec<Symbol>,
    /// The stripped `<provider>__disturbs` funcdef.
    pub sh: String,
    /// The host tool the body reached (display locus for the `Derived` origin + advisory).
    pub call: String,
}

/// The compiled derivation-probe (24E §2 — the SECOND probe-shipping path, PARALLEL to
/// [`ProbePlan`] per fork-s4-compile): the wall-candidate sites whose `touches()` escalated to
/// host-derivation. It rides the SAME phase-1 artifact as the convergence probe; its stdout
/// coordinate-records are read back and built into `Derived` [`Footprint`]s before the survival
/// walk. Empty ⇒ nothing is appended to the probe artifact (goldens stay byte-identical).
#[derive(Debug, Default)]
pub struct DerivationPlan {
    /// The escalated sites' derivation queries, in site-id order.
    pub derivations: Vec<ProbeDerivation>,
}

/// One compiled resolver-probe query (24F §3 — the identity CANONICALIZATION lane): a
/// resolver-bearing coordinate whose `<kind>.resolve()` ships to the host to canonicalize its entity.
/// Keyed by the COORDINATE (not a site — resolution is a pure function of `(kind, entity)`; the same
/// coordinate at many sites resolves once). All fields are pre-resolved Strings so the render is
/// interner-free (the cli owns the interner + oracle sources; `inv-referent-agnostic` — the entity
/// text is F-quoted for the invocation, never decoded).
#[derive(Debug, Clone)]
pub struct ResolverProbe {
    /// The coordinate's `kind:entity` label — the `resolv` record key + display (cli-resolved).
    pub coord_label: String,
    /// The kind's display name (`package`) for the provenance comment.
    pub kind_label: String,
    /// The mangled resolver funcname (`package__resolve`) — the shipped def + the invocation agree.
    pub kind_fn: String,
    /// The entity text passed to the resolver invocation (F-quoted at render).
    pub entity_text: String,
    /// The stripped `<kind>__resolve` funcdef (strip-only; `dorc_oracle::predict::strip_resolve`).
    pub sh: String,
}

/// The compiled resolver-probe (24F §3 — the identity CANONICALIZATION lane, PARALLEL to
/// [`ProbePlan`]/[`DerivationPlan`]): the resolver-bearing coordinates whose `<kind>.resolve()` runs
/// host-side to canonicalize their entities. Rides the SAME phase-1 artifact; its `resolv` records
/// are read back into a [`Resolutions`] map consumed BEFORE the survival walk (both footprint and
/// backing coords canonicalized). Empty ⇒ nothing appended (goldens stay byte-identical).
#[derive(Debug, Default)]
pub struct ResolverPlan {
    /// The resolver-probe queries, deduplicated by coordinate, in `coord_label` order.
    pub probes: Vec<ResolverProbe>,
}

impl ResolverPlan {
    /// Render the resolver-probe as read-only, self-reporting sh, APPENDED to the earlier probes in
    /// the SAME phase-1 block (no shebang). Each kind's stripped `<kind>__resolve` funcdef is emitted
    /// once (deduped, re-emitted on a body change — sh last-writer-wins), then invoked per COORDINATE
    /// with the entity; its stdout is re-keyed to a `resolv <coord> canon=…` record (or `dangling`,
    /// §4). Empty ⇒ `""` (nothing appended). Interner-free (all fields pre-resolved).
    #[must_use]
    pub fn render_sh(&self, nonce: &records::Nonce) -> String {
        if self.probes.is_empty() {
            return String::new();
        }
        let mut out = String::from(render::resolv::header());
        let mut defined: BTreeMap<&str, &str> = BTreeMap::new();
        for p in &self.probes {
            out.push_str(&render::resolv::resolv_comment(
                &p.coord_label,
                &p.kind_label,
            ));
            if defined.insert(p.kind_fn.as_str(), p.sh.as_str()) != Some(p.sh.as_str()) {
                out.push_str(&render::resolv::kind_def(&p.sh));
            }
            out.push_str(&render::resolv::record_scaffold(
                &p.kind_fn,
                &p.entity_text,
                &p.coord_label,
                nonce,
            ));
        }
        out
    }
}

/// One compiled reach-probe query (24G §4 — a DYNAMIC `reaches()` arm shipped for a footprint coord):
/// per (reach-bearing footprint coordinate, dynamic arm) the coord's entity is passed to the arm's
/// stripped-clean per-arm wrapper, whose stdout lines are the RAW ENTITIES the arm emits — captured
/// PER-ARM (`reach <coord> arm=<index> entity=<line>`) so the controller joins arm→kind statically
/// (the arm index re-keys back to the arm's lifted kind). Interner-free (all fields pre-resolved; the
/// entity text is F-quoted for the invocation, never decoded — `inv-referent-agnostic`).
#[derive(Debug, Clone)]
pub struct ReachProbe {
    /// The source footprint coord's `kind:entity` label — the `reach` record key + display.
    pub coord_label: String,
    /// The reach-function KIND's display name (`package`) for the provenance comment.
    pub kind_label: String,
    /// The per-arm wrapper funcname (`package__disturbance_reaches_only_1`) — def + invocation agree.
    pub arm_fn: String,
    /// The arm index (readback demux — `reach <coord> arm=<index>`; the controller maps it to kind).
    pub arm_index: usize,
    /// The coord's entity text passed to the arm invocation (F-quoted at render).
    pub entity_text: String,
    /// The per-arm wrapper funcdef (`<arm_fn>() { <arm-command bytes> ; }`) — the arm's command
    /// span-slice (mark-free by construction) wrapped so `$1` binds the entity. Byte-exact author sh.
    pub arm_sh: String,
}

/// The compiled reach-probe (24G §4 — the `reaches()` EXPANSION lane, PARALLEL to
/// [`ProbePlan`]/[`DerivationPlan`]/[`ResolverPlan`]): the DYNAMIC reaches arms shipped for each
/// reach-bearing AUTHORED footprint coordinate. Rides the SAME phase-1 artifact; its `reach` records
/// are read back and unioned into the footprints (via `Footprint::add_reached`) BEFORE the survival
/// walk. STATIC reaches arms never ship (traced at the cli). Empty ⇒ nothing appended (goldens stay
/// byte-identical — a book with no reach-bearing wall is unchanged).
#[derive(Debug, Default)]
pub struct ReachPlan {
    /// The reach-probe queries, in `(coord_label, arm_index)` order.
    pub probes: Vec<ReachProbe>,
}

impl ReachPlan {
    /// Render the reach-probe as read-only, self-reporting sh, APPENDED to the earlier probes in the
    /// SAME phase-1 block (no shebang). Each per-arm wrapper is emitted once (deduped, re-emitted on a
    /// body change — sh last-writer-wins), then invoked per COORDINATE with the entity; its stdout is
    /// re-keyed per-line to a `reach <coord> arm=<index> entity=…` record. Empty ⇒ `""`. Interner-free.
    #[must_use]
    pub fn render_sh(&self, nonce: &records::Nonce) -> String {
        if self.probes.is_empty() {
            return String::new();
        }
        let mut out = String::from(render::reach::header());
        let mut defined: BTreeMap<&str, &str> = BTreeMap::new();
        for p in &self.probes {
            out.push_str(&render::reach::reach_comment(
                &p.coord_label,
                &p.kind_label,
                p.arm_index,
            ));
            if defined.insert(p.arm_fn.as_str(), p.arm_sh.as_str()) != Some(p.arm_sh.as_str()) {
                out.push_str(&render::reach::arm_def(&p.arm_sh));
            }
            out.push_str(&render::reach::record_scaffold(
                &p.arm_fn,
                &p.entity_text,
                &p.coord_label,
                p.arm_index,
                nonce,
            ));
        }
        out
    }
}

/// The `<provider>__disturbs` derivation funcname (24E §2/§9), mangled IDENTICALLY to
/// [`predict_fn_name`] and suffixed from the SAME constant `strip_touches` mangles the shipped
/// funcdef with, so a site's def and its invocation agree byte-for-byte. Referent-agnostic:
/// passed to the host, never branched on.
fn touches_fn_name(interner: &Interner, provider: Symbol) -> String {
    format!(
        "{}{DISTURBS_SUFFIX}",
        dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(
            interner.resolve(provider)
        )),
    )
}

impl DerivationPlan {
    /// Render the derivation-probe as read-only, self-reporting sh, APPENDED to the convergence
    /// probe in the SAME phase-1 block (no shebang — the e2e shebang-split keeps it in phase-1).
    /// Each provider's stripped `<provider>__disturbs` funcdef is emitted once (deduped, re-emitted
    /// on a body change — sh's last-writer-wins, exactly as [`ProbePlan::render_sh`] does for the
    /// multi-body provider), then invoked per SITE with the site's argv, its stdout coord-lines
    /// re-keyed to `deriv <leafid> coord=…` records. Empty ⇒ `""` (nothing appended).
    #[must_use]
    pub fn render_sh(&self, nonce: &records::Nonce, interner: &Interner) -> String {
        if self.derivations.is_empty() {
            return String::new();
        }
        let mut out = String::from(render::deriv::header());
        let mut defined: BTreeMap<String, &str> = BTreeMap::new();
        for d in &self.derivations {
            let fn_name = touches_fn_name(interner, d.provider);
            out.push_str(&render::deriv::deriv_comment(
                d.site,
                interner.resolve(d.provider),
                &d.call,
            ));
            if defined.insert(fn_name.clone(), d.sh.as_str()) != Some(d.sh.as_str()) {
                out.push_str(&render::probe::wrapper_def(&d.sh));
            }
            let invocation = render::deriv::invocation(&fn_name, &d.argv, interner);
            out.push_str(&render::deriv::record_scaffold(&invocation, d.site, nonce));
        }
        out
    }
}

/// Compile the derivation-probe (24E §2 / fork-s4-compile — a PARALLEL builder to
/// [`compile_probe`], deliberately NOT an extension: a different site-set [wall-candidates, not
/// elision-candidates], a different body-source [`touches` not `predict`], a different readback
/// [stdout coords not rc]). For each WALL-CANDIDATE site (an establish-bearing class or a kill —
/// the same candidate set [`crate::build_plan_walled`]'s footprints cover), `derive_body` decides
/// escalation + ships: it returns `Some(DerivationShip)` iff the site's `touches()` body ESCALATED
/// (reached a host query the static `evaluate_touches` could not resolve — a `NonPrintfCommand` ⊤),
/// else `None` (no touches, a statically-resolvable/non-escalating body, or a non-literal argv).
/// The escalation decision + the strip live in the closure (the cli owns the oracle sources +
/// `evaluate_touches`); `plan` stays oracle-free. Deterministic, non-mutating; the readback +
/// footprint-build are the cli's ([`ProbeDerivation::site`] keys them).
///
/// The closure takes the site's [`CfgNodeId`] because WHICH `disturbs` body ships is a question
/// about the asking frame (`28Q` §1.3), exactly as it is in [`compile_probe`]'s two ship closures.
pub fn compile_derivations(
    ast: &Ast,
    cfg: &Cfg,
    value: &ValueFlow,
    classes: &[(CfgNodeId, SkipClass)],
    kills: &BTreeSet<CfgNodeId>,
    derive_body: impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<DerivationShip>,
) -> DerivationPlan {
    let mut derivations = Vec::new();
    for (site, node, class) in site_order(ast, cfg, classes) {
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishProbeAmbient(_) | SkipClass::EstablishProbeWritten(_)
        ) || kills.contains(&node);
        if !is_wall_candidate {
            continue;
        }
        // R3-shape: split the resolved argv into (provider-word, operands); a ⊤ word ⇒ no
        // concrete invocation ⇒ the site stays walled (kFAIL-safe). The closure then decides
        // escalation (a `NonPrintfCommand` ⊤ from `evaluate_touches`) + ships the stripped body.
        let argv = value.argv_values(node);
        let Some((first, rest)) = argv.split_first() else {
            continue;
        };
        let ValueOf::Literal(provider) = first else {
            continue;
        };
        let mut operands = Vec::with_capacity(rest.len());
        let mut concrete = true;
        for w in rest {
            if let ValueOf::Literal(s) = w {
                operands.push(*s);
            } else {
                concrete = false;
                break;
            }
        }
        if !concrete {
            continue;
        }
        let Some(ship) = derive_body(node, *provider, &operands) else {
            continue;
        };
        derivations.push(ProbeDerivation {
            site,
            node,
            provider: *provider,
            argv: operands,
            sh: ship.sh,
            call: ship.call,
        });
    }
    DerivationPlan { derivations }
}

/// The canonical per-site ordering shared by [`compile_probe`] and [`build_plan`]
/// (`inv-site-keyed-results`, the load-bearing back-map): assign each classified
/// command a stable [`LeafId`] by sorting on its source span, so the probe's site-ids
/// and the apply plan's leaf-ids are the SAME id space. Two same-command sites get
/// distinct ids (their spans differ). Returned in site-id order, paired with the
/// node + class so a caller need not re-sort.
///
/// Deterministic (`inv-determinism`): a total sort by `(span.lo, span.hi)`. Classify
/// already excluded expansion-internal non-leaves (find-cli-1), so every entry is a
/// genuine plan/apply leaf.
/// Each classified site's stable [`LeafId`], in span order — the ONE id space the probe records,
/// the plan's steps, and every site-keyed diagnostic share (`inv-site-keyed-results`).
///
/// Exposed because a driver that indexes `classes` positionally is keying on CFG ALLOCATION order,
/// which coincides with span order for straight-line books and silently does not for others.
#[must_use]
pub fn leaf_ids(
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
) -> Vec<(LeafId, CfgNodeId)> {
    site_order(ast, cfg, classes)
        .into_iter()
        .map(|(leaf, node, _)| (leaf, node))
        .collect()
}

pub(crate) fn site_order<'a>(
    ast: &Ast,
    cfg: &Cfg,
    classes: &'a [(CfgNodeId, SkipClass)],
) -> Vec<(LeafId, CfgNodeId, &'a SkipClass)> {
    let mut ordered: Vec<(CfgNodeId, &SkipClass)> = classes.iter().map(|(n, c)| (*n, c)).collect();
    ordered.sort_by_key(|(node, _)| {
        let span = ast.node(cfg.node(*node).ast).span;
        (span.lo.0, span.hi.0)
    });
    ordered
        .into_iter()
        .enumerate()
        .map(|(i, (node, class))| (LeafId(u32::try_from(i).unwrap_or(u32::MAX)), node, class))
        .collect()
}

/// The **connected check-pipes** recognised in a book (24J — the pipe-guard MEDIUM core).
///
/// A check-pipeline `A | F [| F…]` whose EVERY stage is a vouched read-only Query (`A`'s
/// verb-arm via its own oracle, `F` via the stdlib grep oracle) is shipped as ONE *connected*
/// probe: the host runs the real pipe and the governing (last) stage's rc reads back keyed to
/// the governing site (24J §2). A lone `grep -q` has no independent fact (silence-is-wall), so
/// ONLY the connected form probes; the raw pipeline bytes are what run (24J: "the host runs the
/// real `A | F` … nothing needs reassembling" — a stage's `__predict` body cannot chain, e.g.
/// `otelcol --version >/dev/null` would starve `grep`).
///
/// NARROW FIRST (24J §2): simple all-vouched `A | F [| F…]` chains only — every stage a bare
/// `Simple` command with NO redirect, and every stage a [`SkipClass::QueryResolvable`]. Anything
/// else (a redirection, nesting, an unvouched/mutating stage) is NOT recognised and ⊤s to the
/// wall floor (today's behaviour — the negative control pins it).
#[derive(Debug, Clone, Default)]
pub struct ConnectedPipes {
    /// governing (last-stage) [`CfgNodeId`] → the COMPOSED probe: each stage substituted by its
    /// oracle's stripped `<provider>__predict` invoked with the stage's own argv
    /// (`271:rul-only-oracle-bytes-ship`; `24J`-repair). The probe scaffold pipes the composed
    /// predicts and captures the governing rc (`inv-site-keyed-results`, keyed to the governing
    /// site). ONLY compounds that PASS the coverage rule (every non-last stage produces real
    /// stdout — [`StageShip::produces_real_stdout`]) land here; a compound that fails is demoted
    /// wholesale to [`orphan_stages`](ConnectedPipes::orphan_stages) (can't-say ⇒ run).
    governing: BTreeMap<CfgNodeId, ComposedProbe>,
    /// non-last member [`CfgNodeId`] → its governing (last-stage) [`CfgNodeId`]. The member ships no
    /// separate probe (subsumed into the connected unit); at apply it OMITS controlled by the
    /// governing stage once that stage's connected verdict is converged (`build_plan_walled`).
    members: BTreeMap<CfgNodeId, CfgNodeId>,
    /// A Query stage of a pipeline that did NOT qualify as connected — a redirection, a nesting, an
    /// unvouched/mutating stage (the negative control), OR a recognized-but-COVERAGE-FAILING
    /// compound (a stage whose predict does not resolve, or a non-last stage that declines stdout,
    /// `271:rul-only-oracle-bytes-ship` rider 1). Such a stage is stdin-dependent inside its pipe
    /// (silence-is-wall — a lone `grep -q` has no independent fact), so it must NOT ship its
    /// context-free `__predict` (which would read the wrong stdin); it is UNRESOLVABLE ⇒ runs
    /// (`kFAIL-perform`). Only a CONNECTED governing stage that shipped ever probes a pipe-stage.
    orphan_stages: BTreeSet<CfgNodeId>,
}

/// The composed probe for one connected check-pipe (`271:rul-only-oracle-bytes-ship`; the `24J`
/// raw-ship repair): the ordered pipe stages, each replaced by its oracle's stripped predict. The
/// render pipes each stage's `<provider>__predict <argv>` — ONLY oracle-authored bytes ship; the
/// admin's book bytes NEVER do (they flow in as the predicts' arguments through each author's
/// argparse — `271:rul-argv-flows-bytes-do-not`).
#[derive(Debug, Clone)]
pub struct ComposedProbe {
    /// The pipe stages in order (governing stage last). Rendered as
    /// `stage0__predict a b | stage1__predict c | …` then the record scaffold.
    pub stages: Vec<ComposedStage>,
}

/// One stage of a [`ComposedProbe`]: the provider whose stripped predict stands in, that stage's
/// resolved argv (F-quoted at render, `inv-kfail`), and the stripped `<provider>__predict` funcdef
/// (emitted once, dedup by funcname). Mirrors the ordinary [`ProbePredict`] `provider`/`argv`/`sh`
/// fields — a stage IS an ordinary shipped predict, just piped into its successor.
#[derive(Debug, Clone)]
pub struct ComposedStage {
    /// The stage's command word (`otelcol`, `grep`) — keys [`predict_fn_name`] at render.
    pub provider: Symbol,
    /// The stage's argv after the command word, each a resolved literal.
    pub argv: Vec<Symbol>,
    /// The stripped `<provider>__predict` funcdef for this stage (strip-only — the check IS the
    /// oracle). Re-emitted only when its funcname's body changes (the render's dedup).
    pub sh: String,
}

/// What a stage-ship closure returns for one pipe stage (`271:rul-only-oracle-bytes-ship` rider 1):
/// the stripped `<provider>__predict` body PLUS its STDOUT coverage — whether the arm this argv
/// selects produces REAL bytes on the pipe. A non-last stage is model-substitutable iff
/// [`produces_real_stdout`](StageShip::produces_real_stdout); the coverage decision itself lives
/// in the oracle crate (`dorc_oracle::predict::predict_stage_stdout`), collapsed here to the
/// byte-consumer gate (`StageStdout::RealBytes`).
#[derive(Debug, Clone)]
pub struct StageShip {
    /// The stripped `<provider>__predict` funcdef (strip-only).
    pub sh: String,
    /// Whether this stage's predict arm produces REAL (delegation-produced, `271:rul-composed-bytes-
    /// defer-and-floor`) bytes on stdout — the coverage a downstream byte-consumer requires. A
    /// `printf`-asserted or `>/dev/null`-declined arm is `false` ⇒ refuses the compound if non-last.
    pub produces_real_stdout: bool,
}

impl ConnectedPipes {
    /// The composed probe for a governing site, if `node` governs a SHIPPABLE connected check-pipe.
    #[must_use]
    pub fn governing_composed(&self, node: CfgNodeId) -> Option<&ComposedProbe> {
        self.governing.get(&node)
    }

    /// The governing (last-stage) node for a non-last member, if `node` is subsumed into a
    /// shippable connected check-pipe.
    #[must_use]
    pub fn member_governor(&self, node: CfgNodeId) -> Option<CfgNodeId> {
        self.members.get(&node).copied()
    }

    /// Is `node` a stage of a pipeline that did NOT ship a connected probe (silence-is-wall — never
    /// probes, runs)? True for a non-recognized pipe's Query stages AND for every stage of a
    /// coverage-failing compound.
    #[must_use]
    pub fn is_orphan_stage(&self, node: CfgNodeId) -> bool {
        self.orphan_stages.contains(&node)
    }
}

/// Decide the [`ConnectedPipes`] of a book (`24J` §2 — the composed-predict repair,
/// `271:rul-only-oracle-bytes-ship`). Pure + deterministic (ordered maps, a single AST walk);
/// safe on ANY book — a book with no shippable all-Query pipe yields an empty map (the flag-off
/// equivalent).
///
/// A pipeline SHIPS as a composed probe iff: it has ≥2 stages, EVERY stage is a bare `Simple`
/// command with NO redirect (`24J` narrow-first) resolving to a [`SkipClass::QueryResolvable`]
/// leaf (the read-only vouch — the grep stdlib + the check tool's own `:?` arm), AND every stage's
/// predict RESOLVES its argv (`ship_stage` returns `Some`), AND every NON-LAST stage produces REAL
/// stdout bytes (the per-channel coverage rule, rider 1 — the byte the downstream stage consumes
/// must be world-spoken, not a `printf`-assert or a `>/dev/null`-decline). Each stage is then
/// replaced by its stripped `<provider>__predict <argv>`; ONLY oracle-authored bytes ship. Any
/// failure REFUSES the whole compound — every stage becomes an orphan ⇒ runs (can't-say ⇒ run,
/// always safe; no partial or mixed raw/composed emission). The pipeline's `negated` bit is
/// irrelevant here (the fold replays `!` over the captured rc).
#[must_use]
pub fn connected_check_pipes(
    ast: &Ast,
    cfg: &Cfg,
    value: &ValueFlow,
    classes: &[(CfgNodeId, SkipClass)],
    ship_stage: impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<StageShip>,
) -> ConnectedPipes {
    // AstId → (CfgNodeId, is-QueryResolvable). A simple-pipe stage is a single leaf, so the map is
    // 1:1 for the shapes we recognise; a stage whose AstId is absent (opaque/mutator/nested) fails
    // the all-Query gate below and the pipe is rejected (the safe direction).
    let mut leaf_of: BTreeMap<AstId, (CfgNodeId, bool)> = BTreeMap::new();
    for (node, class) in classes {
        let is_query = matches!(class, SkipClass::QueryResolvable { .. });
        leaf_of.insert(cfg.node(*node).ast, (*node, is_query));
    }
    let mut out = ConnectedPipes::default();
    for (_pipe_id, node) in ast.iter() {
        let NodeKind::Pipeline { stages, .. } = &node.kind else {
            continue;
        };
        if stages.len() < 2 {
            continue;
        }
        // Demote every Query stage of this pipe to an ORPHAN (runs) — the safe default this loop
        // OVERRIDES only for a fully-shippable compound. A rejected/refused pipe leaves the demotion
        // in place; a shippable one re-keys its stages into `governing`/`members` below.
        let demote_to_orphans = |out: &mut ConnectedPipes| {
            for &s in stages {
                if let Some((n, true)) = leaf_of.get(&s).copied() {
                    out.orphan_stages.insert(n);
                }
            }
        };
        // Every stage must be a bare Simple with no redirect AND an all-Query leaf.
        let stage_leaf = |stage: AstId| -> Option<CfgNodeId> {
            let NodeKind::Simple { redirs, .. } = &ast.node(stage).kind else {
                return None;
            };
            if !redirs.is_empty() {
                return None;
            }
            leaf_of.get(&stage).and_then(|&(n, q)| q.then_some(n))
        };
        let Some(nodes): Option<Vec<CfgNodeId>> = stages.iter().map(|&s| stage_leaf(s)).collect()
        else {
            // REJECTED (a stage is not a clean no-redir Simple Query — the negative control): the
            // whole pipe walls (silence-is-wall), stages recorded as orphans.
            demote_to_orphans(&mut out);
            continue;
        };
        // Resolve each stage to a composed predict + its stdout coverage. Any stage that fails to
        // resolve (⊤ argv, un-oracled provider) refuses the whole compound; a non-last stage that
        // does not produce REAL stdout (rider 1: `printf`-assert / `>/dev/null`-decline / rc-only)
        // refuses it too — its declined channel is exactly what the next stage consumes.
        let last_idx = nodes.len().saturating_sub(1);
        let mut composed = Vec::with_capacity(nodes.len());
        let mut refused = false;
        for (idx, &stage_node) in nodes.iter().enumerate() {
            let Some((provider, argv, ship)) =
                ship_stage_for_argv(&value.argv_values(stage_node), stage_node, &ship_stage)
            else {
                refused = true;
                break;
            };
            // rider 1 (per-channel coverage): a NON-LAST stage's stdout is consumed downstream, so
            // it MUST produce real bytes; the LAST (governing) stage's stdout is not piped onward
            // (only its rc is consumed), so it is exempt from the stdout gate.
            if idx != last_idx && !ship.produces_real_stdout {
                refused = true;
                break;
            }
            composed.push(ComposedStage {
                provider,
                argv,
                sh: ship.sh,
            });
        }
        // stages.len() >= 2 (checked above) mirrors into `nodes`, so split_last is present — the
        // `else` is unreachable, kept only to avoid an `expect` (no panic path).
        let Some((&governing, members)) = nodes.split_last() else {
            demote_to_orphans(&mut out);
            continue;
        };
        if refused {
            demote_to_orphans(&mut out);
            continue;
        }
        out.governing
            .insert(governing, ComposedProbe { stages: composed });
        for &m in members {
            out.members.insert(m, governing);
        }
    }
    out
}

/// Resolve one pipe stage's provider + argv-after-word0 + [`StageShip`] from its resolved argv
/// ([`ValueFlow::argv_values`]) — the composed-probe analogue of [`ship_for_argv`]. A ⊤ command
/// word or operand ⇒ no concrete stage ⇒ `None` (refuses the compound, `kFAIL-perform`).
fn ship_stage_for_argv(
    argv: &[ValueOf],
    node: CfgNodeId,
    ship_stage: &impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<StageShip>,
) -> Option<(Symbol, Vec<Symbol>, StageShip)> {
    let (first, rest) = argv.split_first()?;
    let &ValueOf::Literal(provider) = first else {
        return None;
    };
    let mut operands = Vec::with_capacity(rest.len());
    for w in rest {
        let &ValueOf::Literal(s) = w else {
            return None;
        };
        operands.push(s);
    }
    let ship = ship_stage(node, provider, &operands)?;
    Some((provider, operands, ship))
}

/// Compile the probe from the analysis result, keyed by command **site**
/// (`inv-site-keyed-results`): each [`SkipClass::EstablishProbeAmbient`] / resolvable-Query
/// site becomes one [`ProbePredict`] shipping its provider's stripped `<provider>__predict`
/// funcdef invoked with the site's argv (R3 / 23D §1 — the check IS the oracle). `ship_body`
/// maps a site's (provider-word, argv-after-word0) to that stripped funcdef (the oracle seam
/// the caller threads, so `plan` need not lift oracles itself); a site with a ⊤ argv word, or
/// whose provider's check does not resolve, becomes `unresolvable`. Two same-command
/// resolvable sites yield two distinct checks (distinct ids) — the per-fact dedup of spike-2
/// is gone.
///
/// `value` supplies each site's resolved argv ([`ValueFlow::argv_values`], or the per-member /
/// per-inline-body argv). `ast`/`cfg` compute the shared site-id ordering ([`site_order`] —
/// the same one [`build_plan`] uses), so the probe's site-ids equal the apply plan's leaf-ids.
/// Deterministic, non-mutating; the FORWARD half of the compiler (the apply is [`build_plan`]).
/// An un-shippable site yields no check ⇒ it cannot be elided downstream
/// (`can't-probe ⇒ can't-elide`, `kFAIL-perform`).
///
/// `is_vouched` closes strain-classify-coupling (24C): a **past-wall** site is
/// [`SkipClass::EstablishProbeWritten`] (an opaque upstream poisoned its resting probe), so at HEAD it
/// ships NO probe — but a guard needs its probe-verdict (the witness's probe half; plan-prediction
/// and apply-guard run the same check, 233 §guard-license). So a `EstablishProbeWritten` site the cli
/// reports VOUCHED (its provider authored a verdict function reaching a vouching path) DOES ship
/// its read-only Establish probe. An unvouched `EstablishProbeWritten` stays unresolvable (jc-probe-
/// scope: whether unvouched walled sites ship hint-probes is deliberately OPEN).
///
/// `connected` ([`connected_check_pipes`], 24J §2) re-routes a recognised connected check-pipe: the
/// GOVERNING (last) stage ships ONE *connected* probe (the raw pipeline bytes — "the host runs the
/// real `A | F`"); every non-last MEMBER is SUBSUMED (ships no separate record — a lone `grep -q`
/// has no independent fact, silence-is-wall). Off the connected path (`ConnectedPipes::default()`)
/// this is byte-identical to before.
///
/// The compiled plan's [`unresolvable_causes`](ProbePlan::unresolvable_causes) starts EMPTY; the
/// one caller that renders causes attaches them with
/// [`with_unresolvable_causes`](ProbePlan::with_unresolvable_causes).
#[must_use]
#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "the probe compiler threads the whole compiled context (ast/cfg/value/classes/connected) \
              plus the `27N` wrapped-site decisions plus THREE ship seams — the predict body, the \
              `24L` §2 verdict-lane body, and the vouch predicate; each is a distinct \
              caller-supplied input. The verdict-lane ship arm pushes the body just over the line \
              cap; the per-class dispatch is irreducibly flat"
)]
pub fn compile_probe(
    ast: &Ast,
    cfg: &Cfg,
    value: &ValueFlow,
    classes: &[(CfgNodeId, SkipClass)],
    wrapped: &WrappedProbes,
    connected: &ConnectedPipes,
    ship_body: impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<ShippedCheck>,
    ship_auto: impl Fn(CfgNodeId, &[FactKey], Symbol, &[Symbol]) -> Option<ShippedCheck>,
    is_vouched: impl Fn(CfgNodeId, FactKey) -> bool,
) -> ProbePlan {
    let mut checks = Vec::new();
    let mut unresolvable = Vec::new();
    for (site, node, class) in site_order(ast, cfg, classes) {
        // `27C` §3 / `27N` — a wrapped BOOK site takes the ENTRY path exclusively: ship the
        // entry-composed check when the consent trace permits (`WrappedProbe::Enter`), else degrade
        // to run (`WrappedProbe::Degrade`, or a non-fact-bearing inner). The ordinary ship (keyed on
        // the wrapper word) would mis-resolve to the wrapper's own model, so it is skipped here.
        if let Some(wp) = wrapped.get(&node) {
            let fact = match class {
                SkipClass::EstablishProbeAmbient(f)
                | SkipClass::EstablishProbeWritten(f)
                | SkipClass::QueryResolvable { fact: f, .. } => Some(*f),
                _ => None,
            };
            match (fact, wp) {
                // Enter (measure in-context, `fact` carries the Wrapped context) and Carry (measure
                // AMBIENT, `fact` carries HostDefault, `composed.enter_defs` empty) ship the SAME
                // entry-composed shape — the fact's own context steers the readback (`27C` §3/§4(a)).
                (
                    Some(fact),
                    WrappedProbe::Enter { provider, composed }
                    | WrappedProbe::Carry { provider, composed },
                ) => {
                    checks.push(ProbePredict {
                        site,
                        member: None,
                        fact,
                        site_kind: ProbeSiteKind::Establish,
                        provider: *provider,
                        argv: Vec::new(),
                        sh: String::new(),
                        defining_span: None,
                        connected: None,
                        verdict: false,
                        emits_report: false,
                        entry: Some(composed.clone()),
                    });
                }
                _ => unresolvable.push(site),
            }
            continue;
        }
        // item-6b (20O find-6 / 20M §7): an in-loop QUERY site stays render-floored this
        // round (`disposition_for` runs it regardless), so probing it is wasted remote
        // work — and with the member-precision wire (item-4) it would ship per-member. So
        // an in-loop Query is recorded unresolvable (never invoked). An in-loop MEMBERS
        // establish is the one in-loop shape that DOES ship a (per-member) check (item-4),
        // handled below; every other in-loop establish is single-fact and floored, so it
        // takes the ordinary resolvable path but is never elided (the floor in `plan`).
        if cfg.in_loop_body(node) && matches!(class, SkipClass::QueryResolvable { .. }) {
            unresolvable.push(site);
            continue;
        }
        // 24J §2 — a non-last stage of a connected check-pipe is SUBSUMED into the governing
        // stage's connected probe: it ships NO separate record (a lone `grep -q` predecessor has
        // no independent fact — silence-is-wall) and is NOT `unresolvable` either (the apply
        // OMITS it, it does not run, so the dq-site-unresolvable "runs unprobed" note would lie).
        if connected.member_governor(node).is_some() {
            continue;
        }
        // 24J §2 — a Query stage of a NON-connected pipeline (an unvouched middle stage, a redirect —
        // the negative control) is stdin-dependent with no independent fact: it must NOT ship its
        // context-free `__predict` (wrong stdin). UNRESOLVABLE ⇒ it runs (silence-is-wall,
        // `kFAIL-perform`); the whole pipe walls.
        if connected.is_orphan_stage(node) {
            unresolvable.push(site);
            continue;
        }
        // An in-loop MEMBERS establish ships ONE check PER MEMBER (item-4): each member is
        // a concrete per-member cell, all-or-nothing — if any member's probe has no body,
        // the WHOLE site is unresolvable (`can't-probe ⇒ can't-elide`, all members or
        // none). The records it emits are sub-keyed `site <leafid>.<member-idx>`. (The probe
        // queries every member regardless of `self_reached` — that bit gates the apply-side
        // license, not what the probe needs to learn.)
        if let SkipClass::EstablishMembers { members, .. } = class {
            push_member_checks(
                &mut checks,
                &mut unresolvable,
                site,
                node,
                members,
                value,
                &ship_body,
                &ship_auto,
                &is_vouched,
            );
            continue;
        }
        // arch-2 (i-4): an inlined CALL ships one `site N.M` check per spliced body establish
        // (see `push_inline_checks` for the all-or-nothing probe-ability).
        if let SkipClass::InlineCall { sites } = class {
            push_inline_checks(
                &mut checks,
                &mut unresolvable,
                site,
                sites,
                value,
                &ship_body,
                &ship_auto,
                &is_vouched,
            );
            continue;
        }
        // Both an EstablishProbeAmbient and a (resolvable) Query site ship a check — each is
        // probe-resolvable iff the provider's `<provider>__predict` resolves the site's argv
        // (R3 / 23D §1). The `site_kind` discriminant rides along so the cli's firewall
        // knows whether the record-rc is the probe command's (Establish ⇒ never fold) or
        // the guard's own (Query ⇒ fold iff valid). A written establish, an inverted
        // claim, opaque, pure, MustRun — none resolvable (`can't-probe ⇒ can't-elide`,
        // `kFAIL-perform`).
        let resolvable = match class {
            SkipClass::EstablishProbeAmbient(fact) => Some((*fact, ProbeSiteKind::Establish)),
            // strain-classify-coupling (24C): a vouched past-wall establish still probes (the
            // guard witness needs the verdict). Establish-class ⇒ its record-rc is the probe
            // command's, never fed to the fold (the firewall is unmoved).
            SkipClass::EstablishProbeWritten(fact) if is_vouched(node, *fact) => {
                Some((*fact, ProbeSiteKind::Establish))
            }
            SkipClass::QueryResolvable { fact, valid } => {
                Some((*fact, ProbeSiteKind::Query { valid: *valid }))
            }
            _ => None,
        };
        let Some((fact, site_kind)) = resolvable else {
            unresolvable.push(site);
            continue;
        };
        // `24J` §2 (repaired) — a GOVERNING connected check-pipe stage ships the COMPOSED probe:
        // each stage's stripped `<provider>__predict` piped (`271:rul-only-oracle-bytes-ship`); the
        // pipeline's governing rc is captured and keyed to this (last-stage) site. `fact`/`site_kind`
        // (the Query firewall) re-key exactly as a lone Query would. `connected_check_pipes` already
        // resolved + coverage-gated every stage, so a `Some` here is fully shippable (a refused
        // compound demoted its stages to orphans ⇒ they take the `is_orphan_stage` path above). The
        // governing site's own provider keys the record's dedup slot; the stages carry the bodies.
        if let Some(composed) = connected.governing_composed(node) {
            if let Some(ValueOf::Literal(provider)) = value.argv_values(node).first() {
                checks.push(ProbePredict {
                    site,
                    member: None,
                    fact,
                    site_kind,
                    provider: *provider,
                    argv: Vec::new(),
                    sh: String::new(),
                    defining_span: None,
                    connected: Some(composed.clone()),
                    verdict: false,
                    emits_report: false,
                    entry: None,
                });
            } else {
                unresolvable.push(site);
            }
            continue;
        }
        // `24L` §2 — a VERDICT-LANE site: the shipped probe is the STRIPPED VERDICT BODY itself
        // (there is no predict answering this cell), invoked with the site argv; its rc maps to
        // the Effect verdict through the record scaffold's existing rc-partition (0=holds, 1=absent,
        // else=cant-tell — the verdict rc-partition). `ship_auto` returns `Some` ONLY for a site
        // classify keyed through that lane, so a `Some` here IS the signal. Establish-class only
        // (a Query never keys through the verdict lane). GATED on the vouch: the verdict IS the
        // probe, so a DECLINED verdict (a refuse path — `return 2`, the R2-MULTIOP arity gate) has
        // nothing to measure and must not ship a record; the site runs
        // (`guard23-refusepath-rc0-never-passes`: a declined verdict never licenses, and never probes).
        // Precedes the predict lane below and MUST: a verdict-lane site can also carry a
        // resolvable predict, which would measure a different cell than this record keys.
        if matches!(site_kind, ProbeSiteKind::Establish)
            && is_vouched(node, fact)
            && let Some((provider, argv, shipped)) =
                ship_auto_for_argv(&value.argv_values(node), node, &[fact], &ship_auto)
        {
            checks.push(ProbePredict {
                site,
                member: None,
                fact,
                site_kind,
                provider,
                argv,
                sh: shipped.sh,
                defining_span: shipped.defining_span,
                connected: None,
                verdict: true,
                entry: None,
                emits_report: shipped.emits_report,
            });
            continue;
        }
        // R3: ship the provider's stripped `check()` invoked with the site's argv. A ⊤ command word or
        // operand, or no check resolving this argv, ⇒ un-shippable (no concrete invocation ⇒
        // `can't-probe ⇒ can't-elide`, `kFAIL-perform`).
        match ship_for_argv(&value.argv_values(node), node, &ship_body) {
            Some((provider, argv, shipped)) => checks.push(ProbePredict {
                site,
                member: None,
                fact,
                site_kind,
                provider,
                argv,
                sh: shipped.sh,
                defining_span: shipped.defining_span,
                connected: None,
                verdict: false,
                emits_report: false,
                entry: None,
            }),
            None => unresolvable.push(site),
        }
    }
    ProbePlan {
        checks,
        unresolvable,
        unresolvable_causes: BTreeMap::new(),
    }
}

/// R3: resolve the (provider-word, argv-after-word0, stripped `<provider>__predict` funcdef)
/// a resolvable site ships, from its resolved `argv` ([`ValueFlow::argv_values`], or a
/// per-member / per-inline-body argv). The command word and every operand must be a
/// concrete literal — a ⊤ command word (a cmdsub/dynamic provider) or a ⊤ operand yields
/// no concrete invocation, so the site is un-shippable (`None`) and therefore un-elidable
/// (`kFAIL-perform`). `ship_body` maps (provider-word, args) to the stripped funcdef; it
/// is the caller's seam onto the oracle sources + check-set, so `plan` needs no oracle
/// lift of its own. A provider whose check does not resolve this argv also yields `None`.
/// `24L` §2 — resolve the (provider, argv, stripped VERDICT body) a verdict-lane site ships.
/// Mirrors [`ship_for_argv`] but hands the SITE to `ship_auto`, which returns the stripped
/// `<provider>__is_converged` body ONLY for a site classify keyed through the verdict lane.
/// A ⊤ command word or operand ⇒ no concrete invocation ⇒ `None` (unshippable).
///
/// Site-keyed, not fact-keyed (`26H` §3.5): the lane is a property of how THIS site's cell was
/// decided, and one fact can be reached by both lanes across a book (one oracle's predict minting
/// the cell at one site, another provider's verdict body naming it at another).
fn ship_auto_for_argv(
    argv: &[ValueOf],
    node: CfgNodeId,
    subjects: &[FactKey],
    ship_auto: &impl Fn(CfgNodeId, &[FactKey], Symbol, &[Symbol]) -> Option<ShippedCheck>,
) -> Option<(Symbol, Vec<Symbol>, ShippedCheck)> {
    let (provider, operands) = literal_invocation(argv)?;
    let shipped = ship_auto(node, subjects, provider, &operands)?;
    Some((provider, operands, shipped))
}

/// `node` is the SITE, not a decoration: the ship closure resolves the body positionally
/// (`28K` §2 rul-visibility-is-full-positional), so it must ship the definition live at the line
/// the check will guard — the same answer the classify lane already read there.
fn ship_for_argv(
    argv: &[ValueOf],
    node: CfgNodeId,
    ship_body: &impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<ShippedCheck>,
) -> Option<(Symbol, Vec<Symbol>, ShippedCheck)> {
    let (provider, operands) = literal_invocation(argv)?;
    let shipped = ship_body(node, provider, &operands)?;
    Some((provider, operands, shipped))
}

/// The concrete `(provider-word, operands)` invocation a site ships, or `None` when any word is ⊤
/// (a cmdsub/dynamic provider or operand) ⇒ no concrete invocation ⇒ un-shippable ⇒ un-elidable
/// (`kFAIL-perform`).
fn literal_invocation(argv: &[ValueOf]) -> Option<(Symbol, Vec<Symbol>)> {
    let (first, rest) = argv.split_first()?;
    let &ValueOf::Literal(provider) = first else {
        return None;
    };
    let mut operands = Vec::with_capacity(rest.len());
    for w in rest {
        let &ValueOf::Literal(s) = w else {
            return None;
        };
        operands.push(s);
    }
    Some((provider, operands))
}

/// Compile one `site N.M` check per loop member and resolved cell.
/// An exact all-vouched population uses verdict bodies; otherwise predictions cannot authorize
/// replacement. Any unshippable selected body makes the whole site unresolvable.
#[expect(
    clippy::too_many_arguments,
    reason = "aggregate probing keeps ordered facts, both body lanes, and exact vouch identity explicit"
)]
fn push_member_checks(
    checks: &mut Vec<ProbePredict>,
    unresolvable: &mut Vec<LeafId>,
    site: LeafId,
    node: CfgNodeId,
    members: &[FactKey],
    value: &ValueFlow,
    ship_body: &impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<ShippedCheck>,
    ship_auto: &impl Fn(CfgNodeId, &[FactKey], Symbol, &[Symbol]) -> Option<ShippedCheck>,
    is_vouched: &impl Fn(CfgNodeId, FactKey) -> bool,
) {
    // R3: the per-member argvs (aligned with `members`, list order, dups kept —
    // [`ValueFlow::member_argv`]). Absent, or a length mismatch, means the Members
    // side-channel and the fact-family disagree ⇒ can't-probe ⇒ the whole site is
    // unresolvable (all-or-nothing, `kFAIL-perform`).
    let Some(member_argvs) = value.member_argv(node) else {
        unresolvable.push(site);
        return;
    };
    if member_argvs.len() != members.len() {
        unresolvable.push(site);
        return;
    }
    let all_vouched = members.iter().all(|fact| is_vouched(node, *fact));
    let mut staged = Vec::with_capacity(members.len());
    for (idx, (fact, argv)) in members.iter().zip(member_argvs).enumerate() {
        let shipped = if all_vouched {
            ship_auto_for_argv(argv, node, members, ship_auto)
        } else {
            ship_for_argv(argv, node, ship_body)
        };
        let Some((provider, args, shipped)) = shipped else {
            unresolvable.push(site);
            return;
        };
        staged.push(ProbePredict {
            site,
            member: Some(u32::try_from(idx).unwrap_or(u32::MAX)),
            fact: *fact,
            site_kind: ProbeSiteKind::Establish,
            provider,
            argv: args,
            sh: shipped.sh,
            defining_span: shipped.defining_span,
            connected: None,
            verdict: all_vouched,
            emits_report: all_vouched && shipped.emits_report,
            entry: None,
        });
    }
    checks.extend(staged);
}

/// Compile one `site N.M` check per probeable inlined body site.
/// An exact all-vouched establish population uses verdict bodies; otherwise predictions cannot
/// authorize replacement. Any unshippable selected establish rejects the whole call; queries stay
/// predict-sourced and cannot mint mutation vouches.
#[expect(
    clippy::too_many_arguments,
    reason = "aggregate probing keeps body sites, both body lanes, and exact vouch identity explicit"
)]
fn push_inline_checks(
    checks: &mut Vec<ProbePredict>,
    unresolvable: &mut Vec<LeafId>,
    site: LeafId,
    sites: &[InlineSite],
    value: &ValueFlow,
    ship_body: &impl Fn(CfgNodeId, Symbol, &[Symbol]) -> Option<ShippedCheck>,
    ship_auto: &impl Fn(CfgNodeId, &[FactKey], Symbol, &[Symbol]) -> Option<ShippedCheck>,
    is_vouched: &impl Fn(CfgNodeId, FactKey) -> bool,
) {
    let establishes: Vec<_> = sites
        .iter()
        .filter_map(|body| match body.class {
            SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
                Some((body.node, fact))
            }
            _ => None,
        })
        .collect();
    let all_vouched = !establishes.is_empty()
        && establishes
            .iter()
            .all(|(node, fact)| is_vouched(*node, *fact));
    let mut staged = Vec::new();
    for (idx, body) in sites.iter().enumerate() {
        let member = Some(u32::try_from(idx).unwrap_or(u32::MAX));
        // The spliced body site's argv, resolved with the call's positionals bound (`i-2`;
        // [`ValueFlow::argv_values`] returns the positional-bound form for a body node).
        let body_argv = value.argv_values(body.node);
        match &body.class {
            SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
                let shipped = if all_vouched {
                    ship_auto_for_argv(&body_argv, body.node, &[*fact], ship_auto)
                } else {
                    ship_for_argv(&body_argv, body.node, ship_body)
                };
                let Some((provider, args, shipped)) = shipped else {
                    // An un-shippable ESTABLISH ⇒ the whole call is unresolvable (all or none).
                    unresolvable.push(site);
                    return;
                };
                staged.push(ProbePredict {
                    site,
                    member,
                    fact: *fact,
                    site_kind: ProbeSiteKind::Establish,
                    provider,
                    argv: args,
                    sh: shipped.sh,
                    defining_span: shipped.defining_span,
                    connected: None,
                    verdict: all_vouched,
                    emits_report: all_vouched && shipped.emits_report,
                    entry: None,
                });
            }
            SkipClass::QueryResolvable { fact, valid } => {
                // A read-only guard: ship its check if resolvable (it does NOT gate the call's
                // elision, so an un-shippable guard is simply omitted, never a blocker).
                if let Some((provider, args, shipped)) =
                    ship_for_argv(&body_argv, body.node, ship_body)
                {
                    staged.push(ProbePredict {
                        site,
                        member,
                        fact: *fact,
                        site_kind: ProbeSiteKind::Query { valid: *valid },
                        provider,
                        argv: args,
                        sh: shipped.sh,
                        defining_span: shipped.defining_span,
                        connected: None,
                        verdict: false,
                        emits_report: false,
                        entry: None,
                    });
                }
            }
            // Not elision-gating ⇒ no record.
            SkipClass::MustRun
            | SkipClass::EstablishMembers { .. }
            | SkipClass::InlineCall { .. } => {}
        }
    }
    checks.extend(staged);
}

/// Build a plan from the analysis result + an injected host **observation** oracle.
///
/// `observe` is the host probe (the real host / `hostsim` is a later seam): it
/// answers, per fact, the [`Observable`] state — the convergence [`Verdict`] (the
/// elision gate) *and* the concrete probe-measured exit status (the fold +
/// value-preserving substitution input). `build_plan` is a pure function of
/// its inputs (deterministic given a deterministic `observe`).
///
/// Two collapses, both apply-phase (`inv-superposition` — the caller argues the
/// phase; the engine never bakes it):
/// 1. **convergence-elision** (the existing path): an `EstablishProbeAmbient` + `Must` +
///    `Converged` + no-unvouched-consumed leaf is `Replace`d by the value-preserving
///    [`StandIn`] reproducing its observed exit status (`true` for the conforming rc
///    0, `(exit 9)` for a non-conforming establish — NOT always `:`).
/// 2. **the fold** (`fold::fold`): a leaf the apply abstract-interpreter proved
///    lies in a provably-dead `&&`/`||`/`if`/`!` branch (from a *known* controlling
///    status) is `Omit`ted. Fold OMITS only from KNOWN observables; ⊤/unknown ⇒ no
///    fold ⇒ run (`inv-kfail`/`kFAIL-perform`).
///
/// A leaf that is neither folded-dead nor convergence-elidable **runs** (the
/// `kFAIL-perform` safe direction).
///
/// This is the kill-unaware entry (empty kill-set): its plan-time wall keys only on
/// establish-bearing classes, so a running `Kills`-only site (`apt-get purge`, classifies
/// `MustRun`) does NOT wall downstream. Callers that have the kill-node set (the cli, via
/// [`dorc_analysis::effect::classify_with_why_diags`]) use [`build_plan_walled`] to close
/// that gap (24A §3). Kept for the callers that do not thread kills (`hostsim`, tests).
///
/// Threads `vouches` (24D §3 elide-weld): a converged ambient site elides ONLY with a reached
/// vouch, so this entry now takes the [`Vouches`] map too (build it with [`build_vouches`]). The
/// survival tier is still off (`None`) — this entry stays kill-unaware AND flag-off.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "the compatibility entry accepts the complete kernel context before bundling the survival-only inputs for the shared settlement"
)]
pub fn build_plan(
    src: &str,
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    invalidators: &BTreeSet<CfgNodeId>,
    vouches: &Vouches,
    observe: impl Fn(FactKey) -> Observable,
    arena: &mut dorc_core::ProvArena,
) -> Plan {
    // `kills` and the backing map are the SURVIVAL lane's inputs and this entry is honest-walls, so
    // they are genuinely empty here rather than defaulted: nothing on the honest path reads either.
    // `invalidators` is NOT optional in the same way — it is the effective world itself, and a
    // caller that dropped it would elide past a mutation nobody could see (`30K` §3.7).
    let classification = RoundClassification {
        classes: classes.to_vec(),
        kills: BTreeSet::new(),
        invalidators: invalidators.clone(),
        fact_backings: BTreeMap::new(),
    };
    // The intakeless entry: this world was never measured, so there is no channel whose integrity
    // could have been lost (`spine::PlanAuthority::without_intake`).
    //
    // The latch is NAMED and spent below. This entry takes classes, not a `Classification`, so what
    // closes here is the SETTLEMENT's latch; a caller whose classification tripped threads its own
    // through `build_plan_walled` directly.
    let mut trip = dorc_analysis::certify::CertifierTrip::default();
    let mut spine = build_plan_walled(
        src,
        ast,
        cfg,
        &classification,
        WallPolicy::Honest,
        vouches,
        &ConnectedPipes::default(),
        // No probe-origin witnesses in this flag-off entry (C6): the Witness is EXEMPT.
        &BTreeMap::new(),
        observe,
        arena,
        &mut trip,
        // The intakeless entry reads no host bytes, so its records are authored-before-contact.
        None,
    );
    certifier_trip::project_censusless(&mut spine, &trip, &PlanAuthority::without_intake())
}

/// [`build_plan`] PLUS the run's wall POLICY.
///
/// `policy` is the closed authority (`30K` §3.3): [`world::WallPolicy::Honest`] is the default and
/// the only honest answer without the admin's typed consent, and
/// [`world::WallPolicy::RiskAccepted`] is constructible ONLY with every input the survival decision
/// needs — so a maintainer cannot reach a footprint the admin did not consent to, and there is no
/// `Option` pair a future caller could half-fill.
///
/// This entry runs ONE settlement (`settle::settle_effective_world`) whose classification is
/// CONSTANT: it has no records to re-fold and no dead-branch cascade to chase, so its rounds move
/// only as replacement deaths retire walls. The cli's driver supplies the reclassifying model, and
/// both go through the same loop — a second settlement implementation is how the two would drift.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "the kernel entry threads the whole compiled context; each argument is a distinct input, and the four model-derived ones are already bundled behind `RoundClassification`"
)]
pub fn build_plan_walled(
    src: &str,
    ast: &Ast,
    cfg: &Cfg,
    classification: &RoundClassification,
    policy: WallPolicy<'_>,
    vouches: &Vouches,
    connected: &ConnectedPipes,
    probe_origins: &BTreeMap<FactKey, ProbeAttribution>,
    observe: impl Fn(FactKey) -> Observable,
    arena: &mut dorc_core::ProvArena,
    trip: &mut dorc_analysis::certify::CertifierTrip,
    minted_at: dorc_core::spine::Grade,
) -> Spine {
    let mut model = FrozenRoundModel {
        classification,
        observe,
        trip,
    };
    // No loaded-source set here, so no openers and no universe. An EMPTY census decides no region.
    let regions = region::RegionCensus::default();
    let inputs = SettleInputs {
        src,
        ast,
        cfg,
        vouches,
        connected,
        policy,
        regions: &regions,
        minted_at,
    };
    // The ledger holds CFG SITES and grows by at least one per non-quiescent round, so the bound is
    // the node count plus the one round that proves nothing new. Leaf count is NOT the bound: a
    // `$( … )` body command, a redirection write, and an unmodeled construct are all sites.
    let cap = u32::try_from(dorc_analysis::solve::Graph::node_count(cfg).saturating_add(1))
        .unwrap_or(u32::MAX)
        .max(1);
    let mut spine = settle_effective_world(&inputs, &mut model, cap).spine;
    attach_spine_probe_provenance(&mut spine, ast, probe_origins, arena);
    spine
}

/// The settlement model for a caller with no records: one classification, computed once, and an
/// observer that answers from the caller's own oracle.
///
/// The overlay is IGNORED here, deliberately. This entry proves no dead branches of its own (its
/// caller holds no records to ground one), so the only ledger growth it can see is replacement
/// deaths, which never reach the analyzer's effect seam anyway (`world::NoExecutionLedger`).
struct FrozenRoundModel<'a, F: Fn(FactKey) -> Observable> {
    classification: &'a RoundClassification,
    observe: F,
    trip: &'a mut dorc_analysis::certify::CertifierTrip,
}

impl<F: Fn(FactKey) -> Observable> RoundModel for FrozenRoundModel<'_, F> {
    fn classify(&mut self, _erased: &dorc_analysis::erase::ErasedSites) -> RoundClassification {
        self.classification.clone()
    }

    fn fold(&mut self, _validity: &BTreeMap<LeafId, bool>) {}

    fn observe(&self, fact: FactKey) -> Observable {
        (self.observe)(fact)
    }

    fn trip(&mut self) -> &mut dorc_analysis::certify::CertifierTrip {
        self.trip
    }
}

/// arch-1 witness (`vp-17`/`vp-18`) + C6: the FULL granted witness for a licensed `Replace` — the
/// establish site's `BookSource` origin PLUS the `ProbeResult` origin of the record that measured
/// its fact converged (absent ⇒ book origin only) — and, beside it, that record's
/// [`ProbeAttribution`] for the why-chain's REPORTED row. Pure OUTPUT provenance attached AFTER the
/// mint (the WELD): the origins are sites the license already keys on, so they cannot influence the
/// decision; they are EXEMPT (`Exempt::ReceiptId`/`Exempt::Timing`) and the `erasability` gate
/// proves they perturb nothing.
/// Attach the post-mint probe provenance every LICENSING disposition carries — the why-chain's
/// REPORTED row, keyed on the fact the license already decided on.
///
/// Runs strictly after the decision, and both licenses treat it as exempt from their identity
/// planes, so nothing here can perturb what was decided.
/// Attach the post-mint probe provenance to every licensing decision on a SETTLED Spine.
///
/// A separate pass, over the settled result, for two reasons that agree: the provenance is EXEMPT
/// output-only material and must not be able to perturb a decision (the WELD), and a settlement
/// round that could reach the arena would be a round that could reach an output surface — which is
/// exactly what `309:law-spine-write-only-during-run` exists to make unrepresentable.
pub fn attach_spine_probe_provenance(
    spine: &mut Spine,
    ast: &Ast,
    probe_origins: &BTreeMap<FactKey, ProbeAttribution>,
    arena: &mut dorc_core::ProvArena,
) {
    for record in spine.dispositions_mut() {
        let span = ast.node(record.ast).span;
        let decision = std::mem::replace(&mut record.decision, Disposition::Run);
        record.decision = attach_probe_provenance(decision, span, probe_origins, arena);
    }
}

pub(crate) fn attach_probe_provenance(
    disposition: Disposition,
    site_span: dorc_core::Span,
    probe_origins: &BTreeMap<FactKey, ProbeAttribution>,
    arena: &mut dorc_core::ProvArena,
) -> Disposition {
    match disposition {
        Disposition::Replace(license, stand_in) => Disposition::Replace(
            attach_replace_provenance(license, site_span, probe_origins, arena),
            stand_in,
        ),
        Disposition::Guard(license) => {
            let attribution = probe_origins.get(&license.fact()).copied();
            Disposition::Guard(license.with_probe_attribution(attribution))
        }
        run_or_omit @ (Disposition::Run | Disposition::Omit { .. }) => run_or_omit,
    }
}

fn attach_replace_provenance(
    license: ReplaceLicense,
    site_span: dorc_core::Span,
    probe_origins: &BTreeMap<FactKey, ProbeAttribution>,
    arena: &mut dorc_core::ProvArena,
) -> ReplaceLicense {
    let book = arena.leaf(dorc_core::OriginKind::BookSource, Some(site_span));
    let attribution = probe_origins.get(&license.fact()).copied();
    let mut origins = vec![book];
    if license.derivation.establish_vouches.is_empty() {
        if let Some(measured) = attribution {
            origins.push(measured.origin);
        }
    } else {
        origins.extend(
            license
                .derivation
                .establish_vouches
                .iter()
                .filter_map(|receipt| probe_origins.get(&receipt.fact).map(|probe| probe.origin)),
        );
    }
    license
        .with_witness(dorc_core::Witness::of(origins))
        .with_aggregate_probe_attribution(probe_origins)
}

/// Everything one site's decision establishes: what the plan does with the leaf, whether its
/// original mutation can still execute, and what the survival tier concluded — all minted
/// TOGETHER, from one proof (`30K` §3.4 `constraint-semantic-acts-not-dispositions`).
///
/// The act is not read off the disposition. It is the other half of the same conclusion, which is
/// what lets effective reach be decision-fed while `pin-no-outcome-as-generator` holds: a rendered
/// outcome never re-enters analysis, because the analysis input was established at the same instant
/// the outcome was, from the same conditions.
pub(crate) struct SiteDecision {
    pub(crate) disposition: Disposition,
    pub(crate) act: EffectiveAct,
    pub(crate) survival: SurvivalAccount,
}

/// The private semantic conclusion from which both public output and effective analysis project.
enum DecisionConclusion {
    Run,
    Replace(ReplaceLicense, StandIn),
    Omit { controller: AstId },
    Guard(GuardLicense),
}

impl DecisionConclusion {
    /// THE BRIDGE into the region plane (`30Nb` §11.1): one total match, at the site seat, and the
    /// only sanctioned producer of a [`region::RouteConclusion`].
    ///
    /// The region plane needs a route's answer without being handed the route's LICENSE: a shared
    /// replacement's license must carry the cross-instance witness spanning every contributing
    /// establish (`30L:pin-shared-witness-spans-instances`), and a per-call license standing in for
    /// it is the exact substitution that pin forbids. So the shadow vocabulary crosses and the
    /// license does not; the settlement mints the real one from the shared conclusion
    /// (`30N:rul-license-mints-at-settlement-from-shared-conclusion`).
    fn as_route(&self) -> region::RouteConclusion {
        match self {
            DecisionConclusion::Run => region::RouteConclusion::Run,
            DecisionConclusion::Replace(_, stand_in) => region::RouteConclusion::Replace(*stand_in),
            DecisionConclusion::Omit { controller } => region::RouteConclusion::Omit {
                controller: *controller,
            },
            DecisionConclusion::Guard(license) => region::RouteConclusion::Guard {
                fact: license.fact(),
            },
        }
    }

    fn project(self, p: &DecideSite<'_>) -> (Disposition, EffectiveAct) {
        let not_effective = || EffectiveAct::NoMutation(NoMutationProof::NotEffective);
        match self {
            DecisionConclusion::Run => (
                Disposition::Run,
                if p.invalidator {
                    EffectiveAct::may_mutate(p.node)
                } else {
                    not_effective()
                },
            ),
            DecisionConclusion::Guard(license) => (
                Disposition::Guard(license),
                if p.invalidator {
                    EffectiveAct::may_mutate(p.node)
                } else {
                    not_effective()
                },
            ),
            DecisionConclusion::Replace(license, stand_in) => {
                let act = if p.invalidator {
                    match settle::replacement_death(p.ast, p.node, p.ast_id, &license) {
                        Some(proof) => EffectiveAct::NoMutation(NoMutationProof::Replaced(proof)),
                        None => EffectiveAct::may_mutate(p.node),
                    }
                } else {
                    not_effective()
                };
                (Disposition::Replace(license, stand_in), act)
            }
            DecisionConclusion::Omit { controller } => {
                let act = if p.invalidator {
                    match p.dead {
                        Some(proof) => {
                            EffectiveAct::NoMutation(NoMutationProof::DeadBranch(*proof))
                        }
                        None => EffectiveAct::may_mutate(p.node),
                    }
                } else {
                    not_effective()
                };
                (Disposition::Omit { controller }, act)
            }
        }
    }
}

/// The inputs one site's decision reads. A struct because the seat genuinely needs the whole
/// context and a twelve-argument function hides which of them are frozen.
pub(crate) struct DecideSite<'a> {
    pub(crate) cfg: &'a Cfg,
    pub(crate) ast: &'a Ast,
    pub(crate) fold: &'a FoldResult,
    pub(crate) node: CfgNodeId,
    pub(crate) ast_id: AstId,
    pub(crate) class: &'a SkipClass,
    pub(crate) freshness: &'a Freshness,
    pub(crate) vouches: &'a Vouches,
    pub(crate) connected: &'a ConnectedPipes,
    pub(crate) observe: &'a dyn Fn(FactKey) -> Observable,
    /// This round's EFFECTIVE Query validity, per node.
    pub(crate) valid_at: &'a BTreeMap<CfgNodeId, bool>,
    /// The fact each leaf establishes or reads, for the connected-pipe governor lookup.
    pub(crate) leaf_fact: &'a BTreeMap<AstId, FactKey>,
    /// This site's dead-branch derivation, if one exists this round.
    pub(crate) dead: Option<&'a DeadBranchProof>,
    /// Does this site gen into the effective world at all? A pure builtin never becomes a wall.
    pub(crate) invalidator: bool,
    /// Does the run's policy account for survivals? Honest walls record nothing.
    pub(crate) accounts_survival: bool,
    /// The one exact aggregate identity shared by freshness and vouch authorization.
    pub(crate) aggregate_establishes: Option<&'a AggregateEstablishes>,
}

/// One region INSTANCE's answer: what the route concluded alone, what edit it would admit as one of
/// many, and which establish it would erase.
///
/// The three travel together because they are one pass over one set of conditions, exactly as
/// [`SiteDecision`]'s three halves are — and because separating them is how a route's own preferred
/// answer would silently become the only thing the region meet could see
/// (`30L:rul-every-property-meets-universally`).
pub(crate) struct RouteDecision {
    pub(crate) conclusion: region::RouteConclusion,
    /// The parametric guard this route would admit, whatever it concluded alone.
    pub(crate) guard: Option<GuardLicense>,
    /// The `(site, cell)` this instance's replacement would erase — one member of the shared
    /// replacement's cross-instance witness.
    pub(crate) establish: Option<(CfgNodeId, FactKey)>,
    /// What the world said about this instance's cell. Carried because the shared guard's ECONOMICS
    /// are a property of the POPULATION, not of any one route: a region every one of whose routes
    /// measured DIVERGED gains nothing from a check that is known to fail everywhere, which is the
    /// site tier's own `jc-mint-policy m-a` reading one level up. The region seat drops the
    /// candidates when no route converged.
    pub(crate) verdict: Verdict,
}

/// Decide one region instance (`30L` §4) — the route's own conclusion, plus what it admits.
///
/// `source_argv` is the region's AUTHORED argv expression (`install "$1"`), which is what makes a
/// shared guard parametric: positional parameters re-bind naturally per invocation, so one authored
/// check serves every operand, and a per-call literal never installs into shared source that also
/// serves another (`30L` §4.5).
pub(crate) fn decide_route(p: &DecideSite<'_>, source_argv: Option<&str>) -> RouteDecision {
    let (conclusion, _) = site_conclusion(p);
    let establish = match p.class {
        SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
            Some((p.node, *fact))
        }
        _ => None,
    };
    RouteDecision {
        conclusion: conclusion.as_route(),
        guard: source_argv.and_then(|argv| region_guard_candidate(p, argv)),
        verdict: establish.map_or(Verdict::Unknown, |(_, fact)| (p.observe)(fact).effect),
        establish,
    }
}

/// The parametric guard one region instance ADMITS — the region tier's own guard question.
///
/// Every safety conjunct of [`GuardLicense::mint`] is here: a reached vouch (so the check is the
/// author's own body over argv their argparse accepted), and the consumption gate at ⊤ (so no reader
/// can tell the check's rc or output from the original's). What is NOT here is the mint's
/// `Converged` conjunct, and the difference is economics rather than safety: at a SITE a guard over
/// a diverged fact buys nothing, because the check is known to fall through to the command that was
/// going to run anyway. At a shared REGION serving many invocations it buys the converged ones,
/// because the guard re-decides per invocation inside sh — which is the whole of `30L` §4.5, and the
/// reason the ordinary site mint is left exactly as it is.
fn region_guard_candidate(p: &DecideSite<'_>, source_argv: &str) -> Option<GuardLicense> {
    if has_top_successor(p.cfg, p.node) || p.cfg.in_loop_body(p.node) {
        return None;
    }
    let fact = match p.class {
        SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => *fact,
        _ => return None,
    };
    let vouch = p.vouches.get(p.node, fact)?.clone();
    let consumed = May(p.cfg.consumed_observables(p.node).clone());
    GuardLicense::mint_for_shared_region(
        fact,
        vouch,
        (p.observe)(fact).effect,
        &consumed,
        source_argv,
    )
}

/// Decide one site (`30K` §5) — the disposition and the semantic act, from one pass.
pub(crate) fn decide_site(p: &DecideSite<'_>) -> SiteDecision {
    let (conclusion, survival) = site_conclusion(p);
    let (disposition, act) = conclusion.project(p);
    SiteDecision {
        disposition,
        act,
        survival: if p.accounts_survival {
            survival
        } else {
            SurvivalAccount::Silent
        },
    }
}

/// The per-leaf disposition: the connected-pipe collapse, then the aggregates, then the in-loop
/// floor, then the fold (a provably-dead leaf is `Omit`ted), then the freshness-gated ternary —
/// elide a fresh converged site, guard a stale one, run everything else.
///
/// The fold takes precedence over convergence-elision because a *dead* leaf has no status a
/// consumer reads — `Omit` is strictly the right disposition (vs `Replace`, which exists to
/// reproduce a status). Both are the apply collapse; a leaf that is neither runs (`kFAIL-perform`).
fn site_conclusion(p: &DecideSite<'_>) -> (DecisionConclusion, SurvivalAccount) {
    // 24J §2 — a SUBSUMED non-last stage of a connected check-pipe: OMIT it (controlled by the
    // governing last stage) once that governing stage's connected verdict is KNOWN. An unknown/⊤
    // governing verdict, or a ⊤-successor member, ⇒ RUN (`kFAIL-perform`).
    if let Some(gov_node) = p.connected.member_governor(p.node) {
        let gov_ast = p.cfg.node(gov_node).ast;
        let gov_known = p
            .leaf_fact
            .get(&gov_ast)
            .is_some_and(|f| matches!((p.observe)(*f).status, Predicted::Value(_)));
        let disposition = if gov_known && !has_top_successor(p.cfg, p.node) {
            DecisionConclusion::Omit {
                controller: gov_ast,
            }
        } else {
            DecisionConclusion::Run
        };
        return (disposition, SurvivalAccount::Silent);
    }
    // An in-loop Members site and an inlined CALL each take their own all-or-nothing license path
    // (the PER-MEMBER / PER-BODY-SITE observations) BEFORE the in-loop floor below.
    match p.class {
        SkipClass::EstablishMembers {
            members,
            self_reached,
        } => return members_disposition(p, members, *self_reached),
        SkipClass::InlineCall { sites } => return inline_disposition(p, sites),
        _ => {}
    }
    // (0) the in-loop render floor (task-L1, `209` brk-1): the line-granular render cannot elide
    // one iteration, and per-iteration deadness is not line-expressible.
    if p.cfg.in_loop_body(p.node) {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    }

    // (2) the fold: a provably-dead branch leaf is omitted. Minted ONLY from a known controlling
    // status (`fold` records `dead` only then) — `inv-kfail`. Top-containment still gates: a
    // ⊤-contaminated leaf is never folded away (context unmodeled).
    if !has_top_successor(p.cfg, p.node)
        && let Some(controller_ast) = p.fold.dead_controller(p.ast_id)
    {
        return (
            DecisionConclusion::Omit {
                controller: controller_ast,
            },
            SurvivalAccount::Silent,
        );
    }

    match p.class {
        SkipClass::QueryResolvable { fact, .. } if !has_top_successor(p.cfg, p.node) => {
            let observed = (p.observe)(*fact);
            let consumed = May(p.cfg.consumed_observables(p.node).clone());
            let valid = p.valid_at.get(&p.node).copied().unwrap_or(false);
            let disposition = match ReplaceLicense::prove_query_replaceable(
                *fact,
                valid,
                observed.effect,
                &consumed,
                observed.status,
            ) {
                Some(license) => DecisionConclusion::Replace(license, standin_for(observed.status)),
                None => DecisionConclusion::Run,
            };
            (disposition, SurvivalAccount::Silent)
        }
        SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact)
            if !has_top_successor(p.cfg, p.node) =>
        {
            establish_disposition(p, *fact)
        }
        _ => (DecisionConclusion::Run, SurvivalAccount::Silent),
    }
}

/// A vouched establish's fate — the whole ternary, in one place.
///
/// The elision license is minted FIRST and then gated on freshness, which is what makes the
/// demotion account exact: "an elision the walls refused" is precisely a site that held a license
/// and lost it to a wall — the same population the retired wall walk demoted. A stale site falls to
/// the guard tier, whose own conditions are unchanged: a reached vouch, a converged measurement,
/// and no consumed channel the insertion would answer for.
fn establish_disposition(
    p: &DecideSite<'_>,
    fact: FactKey,
) -> (DecisionConclusion, SurvivalAccount) {
    let observed = (p.observe)(fact);
    let vouch = p.vouches.get(p.node, fact);
    let verdict = PhasedVerdict::<Probe>::new(observed.effect);
    let consumed = May(p.cfg.consumed_observables(p.node).clone());
    let licensed = ReplaceLicense::prove_replaceable(
        fact,
        Grade::Must,
        verdict,
        consumed.clone(),
        observed.status,
        vouch.cloned(),
    );
    match (p.freshness, licensed) {
        (Freshness::FreshClean, Some(license)) => (
            DecisionConclusion::Replace(license, standin_for(observed.status)),
            SurvivalAccount::Clean,
        ),
        (Freshness::FreshSurvived(SurvivalAttribution::Standalone(witness)), Some(license)) => (
            DecisionConclusion::Replace(
                license.with_survival(SurvivalAttribution::Standalone(witness.clone())),
                standin_for(observed.status),
            ),
            SurvivalAccount::SurvivedStandalone,
        ),
        (Freshness::FreshSurvived(SurvivalAttribution::Aggregate(_)), Some(_))
        | (Freshness::FreshClean | Freshness::FreshSurvived(_), None) => {
            (DecisionConclusion::Run, SurvivalAccount::Silent)
        }
        // The guard tier (rul-ternary-verdict's third verb). A site whose ONLY lost elision
        // precondition is freshness re-decides LIVE at apply: `( check ) || <original>`, so the
        // stale plan-time convergence is never trusted. No vouch, or a diverged/unknown verdict,
        // ⇒ run — a guard at a predicted-change site buys nothing (`inv-kfail`).
        (Freshness::Stale(cause), licensed) => {
            let account = if licensed.is_some() {
                SurvivalAccount::Demoted(*cause)
            } else {
                SurvivalAccount::Silent
            };
            let disposition = match vouch {
                Some(v) => match GuardLicense::mint(fact, v.clone(), observed.effect, &consumed) {
                    Some(license) => DecisionConclusion::Guard(license),
                    None => DecisionConclusion::Run,
                },
                None => DecisionConclusion::Run,
            };
            (disposition, account)
        }
    }
}

/// The value-preserving stand-in for a substituted leaf's status.
///
/// An unpredicted status falls back to `true` (rc 0) in two cases, neither fabricating a value a
/// LIVE reader consumes: a converged-establish whose status is not branch-consumed (the mint blocks
/// a branch-consumed ⊤ via `StatusRelaxable`), and door-3's `cmd || true` left, where `true` is the
/// IDIOM rather than a predicted value — the mint is licensed by INVARIANCE, not by a claim that
/// the command exits 0.
fn standin_for(status: Predicted<Rc>) -> StandIn {
    match status {
        Predicted::Value(rc) => StandIn::from_rc(rc),
        Predicted::Top => StandIn::True,
    }
}

/// The disposition for an in-loop **Members** body leaf (task-L2 item-3, `209` brk-1(b)) — the
/// all-or-nothing in-loop license: every member Converged, `self_reached`, the consumption gates
/// pass, and the aggregate's own position effectively FRESH. The stand-in is always `true` (the
/// loop still iterates N times over it).
///
/// Reaching walls come from the existing self-suppressed solve; every member then crosses those
/// external walls independently. The aggregate's own writes remain absent because replacement
/// erases them atomically.
fn members_disposition(
    p: &DecideSite<'_>,
    members: &[FactKey],
    self_reached: bool,
) -> (DecisionConclusion, SurvivalAccount) {
    if has_top_successor(p.cfg, p.node) {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    }
    let member_verdicts: Vec<Verdict> = members.iter().map(|f| (p.observe)(*f).effect).collect();
    let consumed = May(p.cfg.consumed_observables(p.node).clone());
    // The in-loop body leaf's status: a mutator's rc is ⊤ (fork-mutator-rc), and a Members site is
    // a mutator, so ⊤. The consumption gate blocks a consumed ⊤.
    let status = Predicted::Top;
    let Some(establishes) = p.aggregate_establishes else {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    };
    let Some(all_vouched) = AllEstablishesVouched::mint(establishes, p.vouches) else {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    };
    let licensed = ReplaceLicense::prove_members_replaceable(
        all_vouched,
        &member_verdicts,
        self_reached,
        &consumed,
        status,
    );
    aggregate_outcome(p, licensed, StandIn::True)
}

/// The disposition for an inlined function-CALL leaf (arch-2, brk-2, `i-3`) — the all-or-nothing
/// CALL license (the CALL span → `true`) iff every effect-bearing body leaf licenses elision AND
/// the call's own position is effectively fresh. On refusal the call RUNS — the real function body
/// executes (the run-it floor, `kFAIL-perform`).
///
/// Freshness is read at the CALL node, where the body's own writes are not yet in the in-state: the
/// splice is wired AFTER the call, so an aggregate's own effects never stale it.
fn inline_disposition(
    p: &DecideSite<'_>,
    sites: &[InlineSite],
) -> (DecisionConclusion, SurvivalAccount) {
    // The in-loop render floor, EXPLICIT here (the Members precedent): an in-loop inlined call
    // never mints a license, robustly, rather than relying on the back-edge self-poison.
    if p.cfg.in_loop_body(p.node) || has_top_successor(p.cfg, p.node) {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    }
    let consumed = May(p.cfg.consumed_observables(p.node).clone());
    // The CALL aggregate's status: ⊤ (a mutator-shaped call's rc has no sanctioned source).
    let status = Predicted::Top;
    if p.aggregate_establishes.is_none() {
        let Some(proof) = ReadSubstitutionProof::mint(sites, p.observe) else {
            return (DecisionConclusion::Run, SurvivalAccount::Silent);
        };
        let stand_in = match proof.status {
            Predicted::Value(rc) => StandIn::from_rc(rc),
            Predicted::Top => return (DecisionConclusion::Run, SurvivalAccount::Silent),
        };
        let licensed = ReplaceLicense::prove_inline_query_replaceable(proof, &consumed);
        return aggregate_outcome(p, licensed, stand_in);
    }
    let Some(establishes) = p.aggregate_establishes else {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    };
    let Some(all_vouched) = AllEstablishesVouched::mint(establishes, p.vouches) else {
        return (DecisionConclusion::Run, SurvivalAccount::Silent);
    };
    let licensed =
        ReplaceLicense::prove_inline_replaceable(sites, all_vouched, p.observe, &consumed, status);
    aggregate_outcome(p, licensed, StandIn::True)
}

/// Gate an aggregate's minted license on effective freshness.
///
/// A survived aggregate can replace only when its cardinality-matched witness names the same exact
/// ordered establishes as its vouch receipts. Any mismatch takes the atomic run floor.
fn aggregate_outcome(
    p: &DecideSite<'_>,
    licensed: Option<ReplaceLicense>,
    stand_in: StandIn,
) -> (DecisionConclusion, SurvivalAccount) {
    match (p.freshness, licensed) {
        (Freshness::FreshClean, Some(license)) => (
            DecisionConclusion::Replace(license, stand_in),
            SurvivalAccount::Clean,
        ),
        (Freshness::FreshSurvived(SurvivalAttribution::Aggregate(witness)), Some(license)) => {
            match license.with_aggregate_survival(witness.clone()) {
                Some(license) => (
                    DecisionConclusion::Replace(license, stand_in),
                    SurvivalAccount::SurvivedAggregate {
                        establishes: u32::try_from(witness.members().count()).unwrap_or(u32::MAX),
                    },
                ),
                None => (DecisionConclusion::Run, SurvivalAccount::Silent),
            }
        }
        (Freshness::FreshSurvived(SurvivalAttribution::Standalone(_)), Some(_)) => {
            (DecisionConclusion::Run, SurvivalAccount::Silent)
        }
        (Freshness::Stale(cause), Some(_)) => {
            (DecisionConclusion::Run, SurvivalAccount::Demoted(*cause))
        }
        (_, None) => (DecisionConclusion::Run, SurvivalAccount::Silent),
    }
}

/// A leaf's source text flattened to one line (interior whitespace collapsed) for an
/// inline diagnostic message — a heredoc leaf's text spans lines, which would garble a
/// single-line `error[…]:` line otherwise.
fn command_text_oneline(sh: &str) -> String {
    sh.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A command's AUTHORED argv expression: its words after the command name, each verbatim from the
/// source, single-space joined.
///
/// This is what makes a shared region's guard PARAMETRIC (`30L` §4.5): `install "$1"` re-binds per
/// invocation inside sh, so one authored check serves every operand the closed census enumerated,
/// and no per-call literal ever installs into shared source that also serves another. Contrast the
/// ordinary site guard, whose invocation carries that site's RESOLVED operands — correct there,
/// because a top-level site has exactly one.
///
/// `None` for a node that is not a simple command: without an argv there is no parametric form, so
/// the region admits no shared guard. The join normalises inter-word whitespace, which is engine
/// glue rather than the author's preserved bytes — the `||`-right keeps those verbatim — and it is
/// what keeps an interleaved redirection out of the check's arguments.
pub(crate) fn source_argv(src: &str, ast: &Ast, id: AstId) -> Option<String> {
    let NodeKind::Simple { words, .. } = &ast.node(id).kind else {
        return None;
    };
    let operands: Vec<&str> = words
        .iter()
        .skip(1)
        .map(|word| {
            let span = ast.node(*word).span;
            src.get(span.lo.0 as usize..span.hi.0 as usize)
                .unwrap_or_default()
        })
        .collect();
    Some(operands.join(" "))
}

/// The verbatim source text of a node's `[lo, hi)` span — the exact sh the admin
/// wrote. Resolving a span for display is allowed under `inv-referent-agnostic`
/// (it is provenance, not a logic branch).
pub(crate) fn command_text(src: &str, ast: &Ast, id: AstId) -> String {
    let span = ast.node(id).span;
    src.get(span.lo.0 as usize..span.hi.0 as usize)
        .unwrap_or_default()
        .to_string()
}

/// Does this CFG node have a top (`Top`) node among its successors? Top-containment
/// (16G hole-5): a leaf whose own statement is top-contaminated — e.g. `cmd &`,
/// lowered as the leaf followed by a `Top` — is not safely replaceable.
/// Map each classified leaf's `AstId` → its fact (establish + query classes carry one).
///
/// The fold reaches over the AST and needs each leaf's observed status keyed by `AstId`, so
/// it asks this map, then the injected `observe`. A Query guard's fact is included so the
/// fold can read its (probe-sourced) Status channel — the rc that resolves the `&&`/`||`
/// branch (task-D2). An in-loop Members site, and an inlined CALL (arch-2), are never fold
/// controllers (a Members body is render-floored; a CALL is an aggregate whose own rc is ⊤),
/// so neither carries a fold status.
///
/// Factored because [`crate::erase::prove_dead_branches`] folds over the SAME mapping: if the
/// ledger's fold saw different observations than the plan's, a site could be erased on a
/// deadness the artifact never renders.
pub(crate) fn leaf_facts(
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
) -> BTreeMap<AstId, FactKey> {
    classes
        .iter()
        .filter_map(|(node, class)| {
            let fact = match class {
                SkipClass::EstablishProbeAmbient(f)
                | SkipClass::EstablishProbeWritten(f)
                | SkipClass::QueryResolvable { fact: f, .. } => *f,
                SkipClass::EstablishMembers { .. }
                | SkipClass::InlineCall { .. }
                | SkipClass::MustRun => return None,
            };
            Some((cfg.node(*node).ast, fact))
        })
        .collect()
}

pub(crate) fn has_top_successor(cfg: &Cfg, node: CfgNodeId) -> bool {
    cfg.succ_ids(node)
        .any(|s| cfg.node(s).kind == CfgNodeKind::Top)
}

impl Plan {
    /// Tally the plan's leaves by disposition for the plan-summary UI (plans/240 Stage-1
    /// yardstick). Pure over [`steps`](Plan::steps) — `inv-determinism`: the yardstick's
    /// elision-frequency metric is a function of the Plan value alone, with no clock, env,
    /// or iteration-order input. The `match` is deliberately exhaustive (no `_` arm): when
    /// the Stage-3 guard tier adds a `Disposition::Guard`, this stops compiling until the
    /// `guard` bucket counts it — so the summary's guard column becomes real, never silently
    /// lost to a catch-all.
    #[must_use]
    pub fn disposition_counts(&self) -> DispositionCounts {
        let mut c = DispositionCounts {
            sites: self.steps.len(),
            ..DispositionCounts::default()
        };
        for step in &self.steps {
            match &step.disposition {
                Disposition::Replace(license, _) => {
                    c.elide += 1;
                    if license.derivation().survival.is_some() {
                        c.elide_by_trusted_claim += 1;
                    } else {
                        c.elide_by_proof += 1;
                    }
                }
                Disposition::Omit { .. } => c.omit += 1,
                Disposition::Guard(_) => c.guard += 1,
                Disposition::Run => c.run += 1,
            }
        }
        c
    }

    /// Render the plan back as sh (the Terraform plan/apply UX, DESIGN): run leaves
    /// verbatim, skipped leaves as provenance comments carrying the why. Each leaf
    /// is emitted separately (the leaf-seam — never coalesced into one `sh -c`).
    ///
    /// *Known first-cut limitation (surfaced, not a bug):* leaves are emitted as a
    /// flat source-ordered sequence, so a leaf's enclosing guard (`if`/`case`) is
    /// NOT reproduced — the plan shows mutator dispositions, not a runnable rewrite
    /// of the original control flow. A faithful in-place rewrite (comment the
    /// elided span where it sits) is a later refinement; the flattening is the
    /// leaf-seam / wo-1 provenance tension made concrete.
    #[must_use]
    pub fn render_sh(&self, interner: &Interner) -> String {
        let mut out = String::from(render::apply::plan_header());
        for step in &self.steps {
            match &step.disposition {
                // A run leaf is emitted verbatim (the leaf-seam — never coalesced).
                Disposition::Run => {
                    out.push_str(&step.sh);
                    out.push('\n');
                }
                Disposition::Replace(license, stand_in) => {
                    out.push_str(&render::apply::flat_replace_block(
                        step.leaf.0,
                        &step.sh,
                        *stand_in,
                        &fact_label(interner, license.fact()),
                    ));
                }
                Disposition::Omit { .. } => {
                    out.push_str(&render::apply::flat_omit_block(step.leaf.0, &step.sh));
                }
                // A guard is inserted inline as real code: `( check ) || <original>` — the
                // original bytes survive verbatim as the `||`-right (rul-ternary-verdict). The
                // preamble defs are emitted once, up front, by [`guard_preamble`](Plan::guard_preamble).
                Disposition::Guard(license) => {
                    out.push_str(
                        &license
                            .insert()
                            .render_line(&step.sh, license.insert().fn_name()),
                    );
                    out.push('\n');
                }
            }
        }
        out
    }

    /// The artifact's **pinned definitions** (`28K` §4 `rul-runtime-resolution-never-load-bearing`):
    /// which body each guard invokes, and under what name.
    ///
    /// The property this exists to make STRUCTURAL: the name a guard calls is bound, at that point
    /// in the artifact, to exactly the bytes the analysis resolved — by construction, not by three
    /// unrelated mechanisms agreeing. A misalignment there could swap WHOSE judgment executes, which
    /// is pope-sin tier (`271:rul-sin-ordering`), so the emission decides the binding rather than
    /// leaving a shell to re-derive it.
    ///
    /// Three rules, in the order they apply:
    ///
    /// 1. **Content-dedup.** Byte-identical bodies are ONE definition however many sites reach them
    ///    (vendored copies are the commonest real collision, `28K` §4).
    /// 2. **Already-in-place wins.** A body the book's own text already defines at top level, under
    ///    the same name and the same bytes, is not copied: the artifact would otherwise carry two
    ///    same-named funcdefs, which is the shape `oracle/src/reserved.rs` refuses by another route
    ///    and which `28K` §4 retires by ANY route. Nothing is re-derived — the definition is the
    ///    pinned one, sitting where its author put it, and the positional regime already proved it
    ///    live at every site that guards (`rul-visibility-is-full-positional`: a vouch exists only
    ///    where the definition it comes from is the one live at the line, so a book-sited definition
    ///    always PRECEDES its guards).
    /// 3. **Hash-munge the rest.** Where one name still has two distinct bodies, each is emitted
    ///    once under `<name>_h<digest>` and the call sites carry the disambiguated name
    ///    (`rul-hash-munge-disambiguation`). Engine SCAFFOLDING around authored bytes — the same
    ///    sanctioned category as the guard glue — never a second source of convergence-truth. The
    ///    munged name cannot parse as a `__role` (the vocabulary is closed and suffix-keyed), so a
    ///    re-ingested artifact reads the guard as an opaque call ⇒ conservative run, the
    ///    `23A:P-reingest` floor.
    ///
    /// Deterministic throughout (`inv-determinism`): the digest is over the definition BYTES, never
    /// a runtime source, and both the hoist order and the name assignment iterate sorted maps.
    ///
    /// The SNAPSHOT (`28R:rul-snapshot-transplant-emission`) precedes every body: each declaration
    /// any pinned body reaches is emitted ONCE, keyed by the declaration site the resolution chose,
    /// so two guards reaching one helper share it instead of each carrying a copy. Bodies own only
    /// their own bytes, which is also what makes the munge rewrite a header-only edit by construction
    /// rather than by a comment asking the reader to trust it.
    #[must_use]
    pub fn pinned_definitions(&self, src: &str, ast: &Ast) -> PinnedDefinitions {
        let mut snapshot: BTreeMap<(usize, u32), &str> = BTreeMap::new();
        // Distinct bodies per funcname, first-seen order preserved within a name.
        let mut bodies: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for insert in self.rendered_guards(ast) {
            for decl in insert.closure() {
                snapshot.insert(decl.key(), decl.bytes());
            }
            let under = bodies.entry(insert.fn_name()).or_default();
            if !under.contains(&insert.body()) {
                under.push(insert.body());
            }
        }
        let mut emitted_names: BTreeMap<(&str, &str), String> = BTreeMap::new();
        let mut hoisted = String::new();
        for bytes in snapshot.into_values() {
            hoisted.push_str(bytes);
            hoisted.push('\n');
        }
        for (name, distinct) in &bodies {
            let book_claims = book_defines_at_top_level(ast, name);
            for body in distinct {
                if book_already_defines(src, ast, name, body) {
                    emitted_names.insert((name, body), (*name).to_owned());
                    continue;
                }
                let plural = distinct.len() > 1;
                if !(plural || book_claims || self.defensive_emission) {
                    hoisted.push_str(body);
                    hoisted.push('\n');
                    emitted_names.insert((name, body), (*name).to_owned());
                    continue;
                }
                let emitted = format!("{name}_h{}", short_digest(body));
                if plural {
                    hoisted.push_str(&render::apply::pinned_provenance(name));
                }
                let header = format!("{name}()");
                hoisted.push_str(&body.replacen(&header, &format!("{emitted}()"), 1));
                hoisted.push('\n');
                emitted_names.insert((name, body), emitted);
            }
        }
        let invoked = self
            .steps
            .iter()
            .map(RenderedEdit::of_step)
            .chain(
                self.live_regions(ast)
                    .into_iter()
                    .map(RenderedEdit::of_region),
            )
            .filter_map(|step| {
                let Disposition::Guard(license) = step.disposition else {
                    return None;
                };
                let insert = license.insert();
                let emitted = emitted_names.get(&(insert.fn_name(), insert.body()))?;
                Some((step.ast, emitted.clone()))
            })
            .collect();
        PinnedDefinitions { hoisted, invoked }
    }

    /// The regions whose edits the artifact still needs — those the artifact can still REACH.
    ///
    /// This is the other half of `30L:pin-whole-helper-derived-only`. When every invocation of a
    /// definition is itself neutralised, its body executes on no route, and `30L` §8's ruling is that
    /// the inert definition remains AUTHORED TEXT: editing it would put a stand-in where nothing
    /// runs, and — worse for a guard — hoist a preamble definition for a body no route reaches,
    /// taking an otherwise-untouched book off its byte floor.
    ///
    /// Conservative in exactly one direction. A route account that was CAPPED cannot answer "every
    /// invocation", so a truncated one keeps its edit: over-editing an inert body is noise, while
    /// dropping an edit whose region can still execute would leave the settlement's no-execution
    /// proof resting on bytes the artifact still runs — a wrong elision, not a cosmetic one.
    fn live_regions<'a>(&'a self, ast: &Ast) -> Vec<&'a RegionStep> {
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();
        self.regions
            .iter()
            .filter(|region| {
                region.routes.dropped() > 0
                    || region.routes.shown().is_empty()
                    || region
                        .routes
                        .shown()
                        .iter()
                        .any(|route| !is_neutralised(&by_ast, ast, route.ast, 0))
            })
            .collect()
    }

    /// The guard inserts whose line the render actually EMITS. A render-REFUSED guard (heredoc /
    /// blocking redirect) runs verbatim, so pinning its definition would hoist a dead one and take
    /// a guard-free book off its byte floor.
    fn rendered_guards<'a>(&'a self, ast: &Ast) -> impl Iterator<Item = &'a GuardInsert> {
        self.steps
            .iter()
            .map(RenderedEdit::of_step)
            .chain(
                self.live_regions(ast)
                    .into_iter()
                    .map(RenderedEdit::of_region),
            )
            .filter_map(move |step| {
                let Disposition::Guard(license) = step.disposition else {
                    return None;
                };
                // OOB-safe: a synthetic test Plan's `AstId`s may index no real node.
                (!(ast.len() > step.ast.0 as usize && guard_render_refused(ast, step.ast)))
                    .then(|| license.insert())
            })
    }

    /// The `AstId`s of `Guard` steps whose render is REFUSED ([`guard_render_refused`] — a heredoc
    /// or non-devnull output-redirect leaf): the guard degrades to run-verbatim. The cli guard
    /// why-lane consults this so a refused guard does NOT claim it "guarded" the site
    /// (rul-attention-honesty — the mutator actually RUNS); it discloses the refusal instead
    /// (gate-7 `refus`). Deterministic (`BTreeSet`).
    #[must_use]
    pub fn guard_refused_asts(&self, ast: &Ast) -> BTreeSet<AstId> {
        self.steps
            .iter()
            .filter(|s| {
                matches!(s.disposition, Disposition::Guard(_)) && guard_render_refused(ast, s.ast)
            })
            .map(|s| s.ast)
            .collect()
    }

    /// Render the apply as the ORIGINAL book with each elided leaf's **exact byte-span**
    /// substituted in-situ (arch-1, note 214 — the leaf-exact / span-based render). A
    /// `Replace`d leaf's command span becomes its value-preserving [`StandIn`]; a
    /// fold-dead `Omit` leaf (whose controller is itself neutralised) becomes `:`; a `Run`
    /// leaf gets NO edit (verbatim is the default, by construction). Every other byte —
    /// scaffolding keywords (`for`/`done`/`if`/`fi`/`then`/`case`/`esac`), the `pat)`/`;;`
    /// of a case arm, the `||`/`&&` of a list, blanks, comments — is kept verbatim, so the
    /// artifact preserves the book's control flow (contrast [`render_sh`](Plan::render_sh),
    /// the flat leaf-list). The leaf-exact render RETIRES the round-21 carve-out family
    /// (T14 case-arm, F2 scaffolding-shared, the group-closer) and the
    /// `StatusRenderFloor`: "the source line" was the wrong substitution unit; the leaf's
    /// byte-span is the right one.
    ///
    /// `ap-2` / `an-render-runnable`: each substitution is value-preserving — `true`
    /// (rc 0), `false` (rc 1), `(exit n)` (other), or `:` for a wholly-dead `Omit`. The
    /// stand-in is the substitution *itself*, not filler: a `Replace` reproduces the leaf's
    /// probe-measured status, so a known-rc-1 Query substitutes `false` and keeps its
    /// `|| fallback` live. Because the edit replaces ONLY the command span and leaves
    /// the surrounding keywords intact, no empty-clause `dash -n` error can arise (the
    /// trap the whole-line-comment form fell into). The leaf-exact render makes the door-3
    /// `cmd || true` payoff expressible (`true || true`) and lets an if/elif guard
    /// substitute in-situ (`if (exit 1); then`) — both unreachable under the line render.
    ///
    /// Edit-model invariants (asserted in [`collect_edits`]): edits never partially
    /// overlap; under full containment the OUTER edit wins (a folded construct's edit
    /// subsumes its interior leaves' — though no current shape produces a containing
    /// construct-edit). The omit-safety gate survives: an `Omit` leaf is edited to `:`
    /// ONLY when its controlling guard is itself neutralised ([`is_neutralised`]); a kept
    /// (`Run`) guard leaves the dead body verbatim (it runs; the runtime guard gates it —
    /// `kFAIL-perform`). The render-capability refusal (`20V` §4 d-6) refuses a leaf
    /// carrying a **heredoc** redirect (the AST span covers `<<EOF`, not the body lines, so
    /// substituting the command span would strand the body as stray artifact lines): such a
    /// leaf is run verbatim.
    #[must_use]
    pub fn render_apply(&self, src: &str, ast: &Ast) -> String {
        let pinned = self.pinned_definitions(src, ast);
        let edits = self.collect_edits(src, ast, &pinned);
        let artifact = emit_span_edits(src, &edits);
        // The GUARD PREAMBLE (24D §2 / rul-ternary-verdict): the pinned definitions the guarded
        // lines invoke, emitted ONCE between the apply header and the book (the defs must precede
        // their invocations — sh execs top-to-bottom, and the header is pure comments). Empty when
        // no site guards ⇒ a guard-free book stays byte-identical to HEAD. `emit_span_edits` emits
        // `apply_header()` as the artifact's verbatim prefix, so splicing after it lands the defs
        // above the whole book.
        let preamble = pinned.hoisted();
        if preamble.is_empty() {
            return artifact;
        }
        let header = render::apply::apply_header();
        format!(
            "{header}{}{preamble}\n{}",
            render::apply::guard_preamble_banner(),
            &artifact[header.len()..],
        )
    }

    /// The sparing re-derivation diagnostics (`300:lane-sparing-rederivation`): one `error` per
    /// site whose SURVIVAL the wall walk minted and the independent reference model then declined
    /// to confirm, so the site demoted to the guard/run tier.
    ///
    /// Empty is the healthy state, and the whole corpus is expected to keep it empty. A non-empty
    /// result is a finding about OUR engine — our two implementations of one algebra answered
    /// differently — never about the book's text; the plan is still valid and still safe, only
    /// poorer. Sited, because a survival verdict is about one line and the reader deserves to see
    /// which elision they did not get. The cli `report()`s these on stderr.
    #[must_use]
    pub fn rederivation_diagnostics(&self, ast: &Ast) -> Vec<Diag> {
        use dorc_aid::diag::{DiagCode, SiteId, SurvivalRederivationDisagreement};
        let spans: BTreeMap<LeafId, dorc_core::Span> = self
            .steps
            .iter()
            .map(|s| (s.leaf, ast.node(s.ast).span))
            .collect();
        self.survival_report
            .rederivation_demotions()
            .filter_map(|(leaf, wall)| {
                spans.get(&leaf).map(|span| {
                    Diag::new(
                        DiagCode::SurvivalRederivationDisagreement(
                            SurvivalRederivationDisagreement {
                                site: SiteId::leaf(leaf),
                                wall: wall.to_string(),
                            },
                        ),
                        *span,
                    )
                })
            })
            .collect()
    }

    /// The render-capability refusal diagnostics (arch-1 d-6): one `error` per leaf that the
    /// disposition layer LICENSED to elide (a `Replace`, or a fold-dead `Omit` whose
    /// controller is neutralised) but the leaf-exact render must REFUSE because its span
    /// cannot be safely edited. The refuse-set this round: a leaf carrying a **heredoc**
    /// redirect (`<<EOF`) — the AST span covers the operator, not the body lines, so
    /// substituting the command span would strand the body as stray artifact lines. Such a
    /// leaf runs verbatim (`kFAIL-perform` — over-executing an already-converged mutator is
    /// safe; a broken artifact is not), and this surfaces WHY (the apply silently running a
    /// converged mutator would otherwise be invisible). The cli `report()`s these on stderr;
    /// the e2e gate-3 floor requires a case exercising this path to declare the diagnostic.
    #[must_use]
    pub fn render_refusal_diagnostics(&self, ast: &Ast, _interner: &Interner) -> Vec<Diag> {
        use dorc_aid::diag::{DiagCode, RenderHeredocRefused, SiteId};
        self.refused_render_steps(ast)
            .into_iter()
            .map(|(step, verb, _cause)| {
                // The migrated `DiagCode::RenderHeredocRefused` spine (`22B` §5 worked-2 — the
                // most-improved case: an inline literal becomes a first-class typed variant the
                // grep gate sees and the registry pins Error+WarnOrDeny). Lowered to the legacy
                // stream, preserving `(code-slug, span, Error)` so the coverage span-bridge and
                // the erasability identity plane are unchanged. The interner resolves no excerpt
                // here (the payload carries only a site) but is threaded for the shared lowering.
                Diag::new(
                    DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                        site: SiteId::leaf(step.leaf),
                        verb,
                        command: command_text_oneline(&step.sh),
                    }),
                    ast.node(step.ast).span,
                )
            })
            .collect()
    }

    /// The `RenderRefusal` collapse narratives paired one-for-one with
    /// [`render_refusal_diagnostics`](Self::render_refusal_diagnostics)
    /// (`AID-NEEDS:law-collapse-mints-narrative`): refusing a LICENSED elision is a
    /// safety-narrowing, so it mints a decision-inert record like every other one. Pairing is by
    /// construction — both walk [`refused_render_steps`](Self::refused_render_steps) — and pinned
    /// by a cardinality gate, the same posture the merge mint carries.
    ///
    /// Decision-inert and, today, unconsumed by any render: the push disclosure is the
    /// `render-heredoc-refused` diagnostic, and the narrative exists for the why-chain that does
    /// not yet read narratives (`289:seam-narrative-render-unconsumed`).
    #[must_use]
    pub fn render_refusal_narratives(&self, ast: &Ast) -> Vec<CollapseNarrative> {
        self.refused_render_steps(ast)
            .into_iter()
            .map(|(step, _, cause)| {
                // Spelled literally, not through `render_refusal_heredoc`: the mint census is a
                // lexical grep for `CollapseKind::<Variant>` and cannot see a named constructor.
                CollapseNarrative::new(
                    SpeechAct::Derived,
                    CollapseKind::RenderRefusal {
                        site: dorc_core::SiteId::leaf(step.leaf),
                        cause,
                    },
                )
            })
            .collect()
    }

    /// [`refused_render_steps`](Self::refused_render_steps) by leaf, for the decision-plane record
    /// (`30E` §3 `dec-render-refusal`). Same seat, so a record and an artifact cannot disagree.
    #[must_use]
    pub fn refused_render_leaves(&self, ast: &Ast) -> Vec<(LeafId, &'static str)> {
        self.refused_render_steps(ast)
            .into_iter()
            .map(|(step, verb, _)| (step.leaf, verb))
            .collect()
    }

    /// Every `Omit` leaf with the render's neutralisation answer (`30E` §3
    /// `dec-omit-neutralisation`): `false` ⇒ the controller was not neutralised, so the body renders
    /// VERBATIM and runs behind a live guard. That is the wrong-yes fence
    /// (`erasure-demands-a-proof-and-a-rendered-death`) as a readable decision rather than a
    /// render-time branch nothing records.
    #[must_use]
    pub fn omit_neutralisations(&self, ast: &Ast) -> Vec<(LeafId, bool)> {
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();
        self.steps
            .iter()
            .filter_map(|step| match &step.disposition {
                Disposition::Omit { controller } => {
                    Some((step.leaf, is_neutralised(&by_ast, ast, *controller, 0)))
                }
                Disposition::Run | Disposition::Replace(..) | Disposition::Guard(_) => None,
            })
            .collect()
    }

    /// The leaves the disposition layer LICENSED to elide that the leaf-exact render must REFUSE,
    /// each with its disposition-aware verb and the CAUSE. A GUARD refusal says "guard"
    /// (X-heredoc's expected-diagnostics pins it), a Replace/Omit refusal says "elide".
    ///
    /// The predicate matches [`collect_edits`](Self::collect_edits)'s drop exactly: every
    /// disposition refuses a heredoc, and a GUARD additionally refuses a blocking output redirect
    /// (`guard_render_refused`). Reading only the heredoc half left a redirect-refused guard
    /// running verbatim with NO disclosure on any of the three surfaces (`30Mf` F2).
    fn refused_render_steps(&self, ast: &Ast) -> Vec<(&Step, &'static str, RenderRefusalTag)> {
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();
        let mut refused = Vec::new();
        for step in &self.steps {
            let is_guard = matches!(step.disposition, Disposition::Guard(_));
            let would_elide = match &step.disposition {
                // A Replace value-substitutes the span; a Guard EDITS it to `( check ) || <orig>` —
                // both strand a heredoc body, so a heredoc-bearing leaf of either must REFUSE and
                // run verbatim (X-heredoc: a vouched heredoc site stays RUN, loudly).
                Disposition::Replace(_, _) | Disposition::Guard(_) => true,
                Disposition::Omit { controller } => is_neutralised(&by_ast, ast, *controller, 0),
                Disposition::Run => false,
            };
            if !would_elide {
                continue;
            }
            // Heredoc first: it refuses under EVERY disposition, so a leaf carrying both reports
            // the cause that would have refused it anyway.
            let cause = if leaf_has_heredoc(ast, step.ast) {
                RenderRefusalTag::Heredoc
            } else if is_guard && leaf_has_blocking_output_redirect(ast, step.ast) {
                RenderRefusalTag::OutputRedirect
            } else {
                continue;
            };
            refused.push((step, if is_guard { "guard" } else { "elide" }, cause));
        }
        refused
    }

    /// Collect the span edits the leaf-exact render applies (arch-1) — one `(Span,
    /// replacement, original)` per elided leaf — and enforce the edit-model invariants.
    ///
    /// A `Replace`d leaf contributes its command-node span (which the parser sets to
    /// include the leaf's trailing redirects — d-2(a)) edited to its [`StandIn`]'s sh; a
    /// fold-dead `Omit` whose controller is neutralised contributes its span edited to `:`
    /// (the omit-safety gate — an un-neutralised controller leaves the body verbatim). A
    /// `Run` leaf contributes nothing (verbatim by default).
    ///
    /// REFUSE (d-6 render-capability): a leaf carrying a heredoc redirect is dropped (no
    /// edit ⇒ runs verbatim) — its AST span covers only `<<EOF`, not the body, so editing
    /// the command span would orphan the heredoc body as stray lines. Multi-line spans are
    /// NOT refused (a span edit may cover multiple lines — the line-render's old refusal
    /// retired); they collapse cleanly to the single-line replacement.
    fn collect_edits(&self, src: &str, ast: &Ast, pinned: &PinnedDefinitions) -> Vec<SpanEdit> {
        // Per-AstId disposition, so an `Omit`'s controller resolves for the omit-safety gate.
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();
        // The TOP-LEVEL Simple statements (`Script.items` that are simple commands) — the
        // provably-safe home of the commented-original elision render (below): a leaf here is
        // NEVER a `&&`/`||`/if/loop/case operand (those nest under AndOr/If/… nodes, not directly
        // under Script), so commenting it can neither empty a control arm nor kill following code.
        let top_level_simple: BTreeSet<AstId> = match &ast.node(ast.root()).kind {
            NodeKind::Script { items } => items
                .iter()
                .copied()
                .filter(|&id| matches!(ast.node(id).kind, NodeKind::Simple { .. }))
                .collect(),
            _ => BTreeSet::new(),
        };

        let mut edits: Vec<SpanEdit> = Vec::new();
        // A region's edit lands ONCE, at the authored definition; calls stay calls.
        for step in self.steps.iter().map(RenderedEdit::of_step).chain(
            self.live_regions(ast)
                .into_iter()
                .map(RenderedEdit::of_region),
        ) {
            let span = ast.node(step.ast).span;
            // d-6: a heredoc leaf refuses ANY neutralising edit (its span does not cover the body
            // lines, so substituting would strand them). A GUARD ALSO refuses a non-devnull output
            // redirect (`>>log`) — the guard's pass-direction would suppress the admin-spelled
            // side-effect (23C-fd10). Both run VERBATIM (kFAIL-perform; disclosed by
            // `render_refusal_diagnostics` + the cli guard why-lane).
            let is_guard = matches!(step.disposition, Disposition::Guard(_));
            if leaf_has_heredoc(ast, step.ast)
                || (is_guard && leaf_has_blocking_output_redirect(ast, step.ast))
            {
                continue;
            }
            let original = command_text(src, ast, step.ast);
            // `self_commented` = the replacement embeds its OWN disposition comment, so the
            // shared elided-provenance comment must NOT be appended on top (a guard would else
            // read `… # dorc: guard [...]   # dorc: elided [...]` — a double comment, and
            // "elided" is a lie: a guard's original bytes SURVIVE in the `||`-right). `comment_out`
            // = the elision render is the ORIGINAL BYTES commented-out (the human's round-24 lean),
            // so the shared BRACKETED provenance is replaced by a no-bracket one (the original is
            // already visible on the line).
            let (replacement, self_commented, comment_out): (String, bool, bool) =
                match &step.disposition {
                    // The elided-render lean (human, round-24): a top-level standalone Simple whose
                    // rc is UNCONSUMED (`StandIn::True` — a consumed-nonzero read is `False`/`Exit`,
                    // excluded; errexit-rc-0 is abort-equivalent) renders as `# <original bytes>`,
                    // not an opaque `true`. THREE safety conditions, all necessary: top-level Simple
                    // (never a `&&`/if/loop/case operand ⇒ commenting can't empty a control arm);
                    // ALONE on its line (a `#` reaching end-of-line can't kill a sibling — `cmd; for
                    // …` / `cmd; systemctl …` are two top-level items on one line, so the FIRST keeps
                    // its stand-in); and single-line span (a multi-line `#` would strand its tail as
                    // live code). Otherwise the stand-in.
                    Disposition::Replace(_, stand_in) => {
                        if matches!(stand_in, StandIn::True)
                            && top_level_simple.contains(&step.ast)
                            && !original.contains('\n')
                            && is_alone_on_line(src, span.lo.0 as usize, span.hi.0 as usize)
                        {
                            (format!("# {original}"), false, true)
                        } else {
                            (stand_in.sh(), false, false)
                        }
                    }
                    Disposition::Omit { controller }
                        if is_neutralised(&by_ast, ast, *controller, 0) =>
                    {
                        // A neutralised-controller dead body: `:` (a pure structural placeholder
                        // — its status is unreachable, never observed).
                        (":".to_string(), false, false)
                    }
                    // A guard edits the span to `( check ) || <original>`, the original bytes embedded
                    // VERBATIM as the `||`-right (rul-ternary-verdict; the guard preamble def is
                    // prepended once by the cli via [`guard_preamble`](Plan::guard_preamble)). The
                    // heredoc case is already refused at the top of the loop (span cannot cover the
                    // body) — X-heredoc. It carries its OWN `# dorc: guard …` comment ⇒ self-commented.
                    Disposition::Guard(license) => {
                        let invoked = pinned
                            .invoked(step.ast)
                            .unwrap_or(license.insert().fn_name());
                        (
                            license.insert().render_line(&original, invoked),
                            true,
                            false,
                        )
                    }
                    // A kept-controller `Omit` (the runtime guard gates it) and a `Run` leaf are
                    // both verbatim — no edit.
                    Disposition::Omit { .. } | Disposition::Run => continue,
                };
            edits.push(SpanEdit {
                lo: span.lo.0 as usize,
                hi: span.hi.0 as usize,
                replacement,
                original,
                self_commented,
                comment_out,
            });
        }
        normalise_edits(edits)
    }
}

/// What the span render needs from a decided unit, whichever identity it wears.
///
/// A `Step` and a `RegionStep` answer to different identities and cannot merge (`inv-leaf-seam`),
/// but the RENDER's question is the same for both: which authored span, and what does the decision
/// put there. This view is that question and nothing else — no leaf id, so nothing downstream can
/// key on an identity the unit may not have.
#[derive(Clone, Copy)]
struct RenderedEdit<'a> {
    ast: AstId,
    disposition: &'a Disposition,
}

impl<'a> RenderedEdit<'a> {
    fn of_step(step: &'a Step) -> Self {
        Self {
            ast: step.ast,
            disposition: &step.disposition,
        }
    }

    fn of_region(region: &'a RegionStep) -> Self {
        Self {
            ast: region.ast,
            disposition: &region.disposition,
        }
    }
}

/// One leaf-exact span edit (arch-1, note 214): replace `src[lo..hi]` with `replacement`,
/// disclosing the `original` command text in the line's provenance comment. Byte offsets
/// are absolute into the source.
#[derive(Debug, Clone)]
struct SpanEdit {
    lo: usize,
    hi: usize,
    replacement: String,
    original: String,
    /// The `replacement` embeds its own disposition comment (a `Guard`'s `# dorc: guard …`), so
    /// the shared elided-provenance comment is suppressed for this member (else a double comment,
    /// and "elided" misdescribes a guard whose original bytes survive in the `||`-right).
    self_commented: bool,
    /// The elision render is the ORIGINAL BYTES commented-out (`# <original>`, the human's round-24
    /// lean) rather than an opaque `true` — so the line gets a NO-BRACKET `# dorc: elided (…)`
    /// provenance (the original is already visible). Set only for a top-level standalone Simple
    /// leaf (single-member group by construction — [`Plan::collect_edits`]).
    comment_out: bool,
}

/// Enforce the edit-model invariants (arch-1 d-1) and return the surviving edits sorted by
/// `lo`: edits never PARTIALLY overlap (a `debug_assert` — the leaf-seam guarantees command
/// spans are disjoint-or-nested, never crossing); under full containment the OUTER edit
/// wins and the inner is DROPPED (a folded construct's edit subsumes its interior leaves').
/// No current shape produces a containing construct-edit (only leaf commands are edited, and
/// two leaf command spans are disjoint), so the containment branch is defensive; it keeps
/// the splice correct if a future construct-span edit lands.
fn normalise_edits(mut edits: Vec<SpanEdit>) -> Vec<SpanEdit> {
    edits.sort_by_key(|e| (e.lo, core::cmp::Reverse(e.hi)));
    let mut kept: Vec<SpanEdit> = Vec::with_capacity(edits.len());
    for e in edits {
        if let Some(prev) = kept.last()
            && e.lo < prev.hi
        {
            // Overlap of some kind. Full containment (e ⊆ prev) ⇒ the OUTER prev wins, drop e.
            // A PARTIAL overlap (e.lo < prev.hi < e.hi) is a leaf-seam violation — assert in
            // debug, and conservatively drop e in release (never produce a corrupt splice).
            debug_assert!(
                e.hi <= prev.hi,
                "partial span-edit overlap [{},{}) vs [{},{}) -- leaf-seam violated",
                prev.lo,
                prev.hi,
                e.lo,
                e.hi
            );
            continue;
        }
        kept.push(e);
    }
    kept
}

/// One line-overlap GROUP of span edits, spliced together as a single rendered line (arch-1
/// note 214 §9 hunt-7; P1 fix 21E). A group is the transitive closure of edits whose covered
/// source-line ranges intersect or ABUT (one edit's start line ≤ a prior edit's end line):
/// such edits collapse onto the SAME rendered line after splicing, so they must be processed
/// together. Keying edits by their lone start line (the pre-fix shape) ORPHANS an edit whose
/// start line falls inside a prior multi-line edit's consumed span — the line-walk skips that
/// line — corrupting the artifact (a half-spliced second command, a provenance comment landing
/// inside an open quote). The group's region spans the first member's start-line start → the
/// last member's end-line end; every member is spliced into it exactly once.
struct EditGroup<'a> {
    /// First source line the group covers (where the rendered line is emitted).
    first_line: usize,
    /// Last source line the group covers (lines `first_line+1..=last_line` are consumed).
    last_line: usize,
    /// The group's edits, in source order (`lo` ascending — `normalise_edits` guarantees they
    /// are span-disjoint, so the order is total and the disclosure reads left-to-right).
    members: Vec<&'a SpanEdit>,
}

/// Partition the normalised edits into line-overlap [`EditGroup`]s (P1 fix 21E f-1). `edits`
/// is sorted by `(lo, …)`, hence by start line; sweep left-to-right, extending the running
/// group while the next edit's start line is within the group's covered line span (intersect
/// or abut), else start a new group. Returns groups keyed by `first_line` so the emit walk can
/// look one up at each line. Every edit lands in EXACTLY ONE group's `members` (the f-1
/// no-edit-dropped invariant; the caller counts them).
fn group_edits<'a>(src: &str, edits: &'a [SpanEdit]) -> BTreeMap<usize, EditGroup<'a>> {
    let line_of = |byte: usize| -> usize {
        src.get(..byte)
            .map_or(0, |s| s.bytes().filter(|&b| b == b'\n').count())
    };
    let mut groups: BTreeMap<usize, EditGroup<'a>> = BTreeMap::new();
    let mut current: Option<EditGroup<'a>> = None;
    for e in edits {
        let start = line_of(e.lo);
        // The edit's last covered line: `hi` is exclusive, so the last byte is `hi-1`; an
        // empty/zero-width span (hi==lo) covers only its start line.
        let end = line_of(e.hi.saturating_sub(1).max(e.lo));
        match &mut current {
            // Abut/overlap test: the edit starts on or before the group's current last line ⇒
            // it shares the group's rendered line (`normalise_edits` already dropped fully
            // contained edits and asserts no PARTIAL overlap, so `start <= last_line` here
            // means "same rendered line", never a crossing splice).
            Some(g) if start <= g.last_line => {
                g.last_line = g.last_line.max(end);
                g.members.push(e);
            }
            // A gap ⇒ flush the running group and open a fresh one.
            _ => {
                if let Some(g) = current.take() {
                    groups.insert(g.first_line, g);
                }
                current = Some(EditGroup {
                    first_line: start,
                    last_line: end,
                    members: vec![e],
                });
            }
        }
    }
    if let Some(g) = current.take() {
        groups.insert(g.first_line, g);
    }
    groups
}

/// Emit the apply artifact by splicing the span edits into the source bytes (arch-1, note
/// 214; P1 fix 21E). Edits that collapse onto one rendered line are processed as a GROUP
/// ([`group_edits`]) and spliced **right-to-left** (highest `lo` first) so an earlier edit's
/// byte offsets stay valid as later ones splice. A provenance comment is appended to each
/// rendered line that carries ≥1 edit (d-3) — disclosing every group member's replaced
/// original — IFF the line end is comment-safe ([`comment_safe`]).
///
/// `edits` must be the normalised (sorted, non-partial-overlap) set from [`normalise_edits`].
fn emit_span_edits(src: &str, edits: &[SpanEdit]) -> String {
    // Group edits that share a rendered line (the P1 fix: a multi-line edit whose consumed span
    // contains a later edit's start line MUST splice both, or the second is orphaned and the
    // artifact corrupts — note 214 §9 hunt-7). Keyed by the group's first source line.
    let groups = group_edits(src, edits);

    // Byte offset of each source line's first byte (index = line number).
    let line_start: Vec<usize> = std::iter::once(0)
        .chain(
            src.bytes()
                .enumerate()
                .filter_map(|(i, b)| (b == b'\n').then_some(i + 1)),
        )
        .collect();

    let mut out = String::from(render::apply::apply_header());
    // f-1 invariant: every edit is applied EXACTLY once. Structurally guaranteed —
    // `group_edits` puts each edit in exactly one group, and the walk visits every group
    // (consecutive groups never share a line, so the post-group jump lands at or before the
    // next group's first line). `spliced_count` is the runtime tripwire (counted in release
    // too); the `debug_assert_eq!` below is the loud debug catch for a future regression.
    let mut spliced_count = 0usize;
    let total_lines = src.lines().count();
    let mut i = 0usize;
    while i < total_lines {
        match groups.get(&i) {
            None => {
                // No edit-group starts here ⇒ verbatim (the default, by construction).
                if let Some(line) = src.lines().nth(i) {
                    out.push_str(line);
                    out.push('\n');
                }
                i += 1;
            }
            Some(group) => {
                // The spliced region's source bytes: from the group's first line start to its
                // last line end (covering every member, multi-line ones included). Splice each
                // member right-to-left within it (offsets relative to the region start).
                let region_lo = line_start.get(group.first_line).copied().unwrap_or(0);
                let region_hi = line_start
                    .get(group.last_line + 1)
                    .copied()
                    .map_or(src.len(), |start| start.saturating_sub(1)); // exclude the '\n'
                let mut spliced = src
                    .get(region_lo..region_hi)
                    .unwrap_or_default()
                    .to_string();
                // Right-to-left so earlier offsets stay valid (members are span-disjoint).
                let mut ordered: Vec<&&SpanEdit> = group.members.iter().collect();
                ordered.sort_by_key(|e| core::cmp::Reverse(e.lo));
                for e in &ordered {
                    let lo = e.lo.saturating_sub(region_lo).min(spliced.len());
                    let hi = e.hi.saturating_sub(region_lo).min(spliced.len()).max(lo);
                    spliced.replace_range(lo..hi, &e.replacement);
                    spliced_count += 1;
                }
                out.push_str(&spliced);
                // d-3: append the provenance comment disclosing every member's replaced
                // original, IFF the post-splice line end is comment-safe. A SELF-COMMENTED member
                // (a `Guard`, whose `render_line` carries its own `# dorc: guard …`) is excluded —
                // it discloses itself, and it does not "elide" (its original bytes survive). If a
                // group is entirely self-commented, no shared comment is appended.
                if comment_safe(&spliced) {
                    // A comment-out group (a top-level standalone Replace rendered as `# <original>`,
                    // single-member by construction): the original is already on the line, so a
                    // NO-BRACKET provenance. Otherwise the shared bracketed disclosure over every
                    // non-self-commented member's original (source order, left-to-right).
                    if group.members.iter().any(|e| e.comment_out) {
                        out.push_str(render::apply::commented_original_provenance());
                    } else {
                        let mut es: Vec<&&SpanEdit> = group.members.iter().collect();
                        es.sort_by_key(|e| e.lo);
                        let originals: Vec<String> = es
                            .iter()
                            .filter(|e| !e.self_commented)
                            .map(|e| e.original.clone())
                            .collect();
                        if !originals.is_empty() {
                            out.push_str(&render::apply::provenance_comment(&originals));
                        }
                    }
                }
                out.push('\n');
                i = group.last_line + 1;
            }
        }
    }
    debug_assert_eq!(
        spliced_count,
        edits.len(),
        "span-edit count mismatch: {spliced_count} spliced vs {} collected -- an edit was \
         orphaned or double-applied (P1 21E f-1 invariant)",
        edits.len(),
    );
    out
}

/// Is `src[lo..hi]` (a command span) ALONE on its source line — nothing but whitespace before
/// it and nothing but whitespace / a trailing `#`-comment after it? The load-bearing precondition
/// for the commented-original elision render (the human's round-24 lean): only such a command can
/// be safely rewritten to `# <original>`, because a `#` runs to end-of-line — so a sibling
/// statement sharing the line (`apt-get …; systemctl …`, `cmd; for …`) would be silently killed.
/// Leading code is checked because it must not be swallowed either (though a leading sibling is
/// already excluded upstream — such a command is not a direct `Script.items` entry). Pure.
fn is_alone_on_line(src: &str, lo: usize, hi: usize) -> bool {
    let line_start = src
        .get(..lo)
        .and_then(|s| s.rfind('\n'))
        .map_or(0, |i| i + 1);
    let line_end = src
        .get(hi..)
        .and_then(|s| s.find('\n'))
        .map_or(src.len(), |i| hi.saturating_add(i));
    let leading = src.get(line_start..lo).unwrap_or("").trim();
    let trailing = src.get(hi..line_end).unwrap_or("").trim();
    leading.is_empty() && (trailing.is_empty() || trailing.starts_with('#'))
}

/// Is appending a ` # …` comment to this rendered line safe (d-3 SAFETY RULE; P1 fix 21E f-2)?
/// A trailing `#` begins a comment-to-end-of-line after a complete command, but NOT when the
/// line ends inside a shape where `#` is not a comment boundary:
///   - INSIDE AN OPEN QUOTE — the f-2 fix. A grouped/multi-line splice can leave the rendered
///     line ending mid-string (e.g. `… install -y "c` when a second leaf's quoted operand was
///     half-consumed by a now-fixed orphan, OR any genuinely quote-crossing rendered line). A
///     `#` there lands INSIDE the literal, not as a comment — silently corrupting the operand
///     (and, with an odd embedded quote, breaking `dash -n` outright). A minimal POSIX
///     quote-state machine ([`region_ends_in_quote`]) decides this.
///   - A BACKSLASH-CONTINUATION (`\` at end) ⇒ the next line continues the command, so `#`
///     would be appended mid-command.
///   - A HEREDOC operator (`<<`) ⇒ the following lines are the heredoc body; a `#` here is in
///     neither a command nor a comment. (A heredoc-bearing leaf is refused an edit upstream
///     — d-6 — so `<<` only reaches here on a verbatim line sharing the rendered line; we still
///     guard it. We keep the simple substring check: no heredoc parsing, per the f-2 scope.)
///
/// Conservative: when unsure, DROP the comment (artifact correctness over provenance prose; the
/// OOB verdict lane still discloses).
fn comment_safe(rendered_line: &str) -> bool {
    let trimmed = rendered_line.trim_end();
    if trimmed.ends_with('\\') {
        return false; // backslash-continuation: the command continues on the next line
    }
    if rendered_line.contains("<<") {
        return false; // a heredoc operator: following lines are the body, not commentable here
    }
    if region_ends_in_quote(rendered_line) {
        return false; // ends inside an open '…' or "…": a `#` would land inside the literal
    }
    true
}

/// Does this rendered line end INSIDE an open single- or double-quote (P1 fix 21E f-2)? A
/// minimal POSIX quote-state scan — single-quote (`'…'`: every byte literal until the next
/// `'`), double-quote (`"…"`: `\` escapes the next byte, `"` closes), and an unquoted backslash
/// (escapes the next byte, so `\'`/`\"` are literals, not quote toggles). NOT a lexer and NOT a
/// heredoc/expansion parser (the `<<` guard stays separate): it tracks ONLY the three quote
/// states needed to answer "is a trailing `#` inside a string literal?". Returns true when the
/// scan finishes still inside a quote (or on a dangling unquoted `\`, which is its own
/// continuation hazard the trailing-`\` check also covers). Mirrors the dash semantics
/// [`dorc_syntax::sem::single_quote`] encodes (single-quotes suppress all escaping).
fn region_ends_in_quote(line: &str) -> bool {
    #[derive(PartialEq)]
    enum Q {
        Out,
        Single,
        Double,
    }
    let mut state = Q::Out;
    let mut escaped = false; // previous byte was an unquoted/in-double `\`
    for ch in line.chars() {
        if escaped {
            // The `\` consumed this byte as a literal — no state change (a `\'` outside or
            // `\"` inside double never toggles a quote).
            escaped = false;
            continue;
        }
        match state {
            Q::Out => match ch {
                '\\' => escaped = true,
                '\'' => state = Q::Single,
                '"' => state = Q::Double,
                _ => {}
            },
            // Single-quotes suppress ALL escaping (dash): only a `'` closes, `\` is literal.
            Q::Single => {
                if ch == '\'' {
                    state = Q::Out;
                }
            }
            Q::Double => match ch {
                '\\' => escaped = true, // escapes the next byte (incl. a `"`)
                '"' => state = Q::Out,
                _ => {}
            },
        }
    }
    state != Q::Out || escaped
}

/// Does a leaf command carry a **heredoc** redirect (`<<EOF`)? The render-capability refusal
/// (d-6): the AST span covers the `<<EOF` operator, NOT the body lines (they are generated
/// content the parser captures separately), so editing the command span would strand the
/// body as stray artifact lines. Such a leaf refuses the edit and runs verbatim.
///
/// This predicate is the ONE refusal definition and must stay in lockstep across its three
/// consumers: [`Plan::collect_edits`] (drop the edit), [`Plan::render_refusal_diagnostics`]
/// (disclose it), and [`is_neutralised`] (a refused controller is KEPT, so it is not
/// neutralised — the omit-safety gate). A future refusal class must extend all three.
pub(crate) fn leaf_has_heredoc(ast: &Ast, leaf: AstId) -> bool {
    let (NodeKind::Simple { redirs, .. }
    | NodeKind::Subshell { redirs, .. }
    | NodeKind::Group { redirs, .. }) = &ast.node(leaf).kind
    else {
        return false;
    };
    redirs.iter().any(|&r| {
        matches!(
            &ast.node(r).kind,
            NodeKind::Redir {
                target: RedirTarget::HereDoc { .. },
                ..
            }
        )
    })
}

/// Does a leaf carry a non-`/dev/null` OUTPUT redirect (`>f` / `>>f` to a file word)? A GUARD on
/// such a leaf is a REFUSE-HOME (23C-fd10 / the redirect ruling h4): the guard's pass-direction
/// (`( check ) || <orig >>log>` — the redirect binds the mutator) SUPPRESSES the admin-spelled
/// side-effect on a converged pass (the append never happens), an effect corruption at the
/// consumer. So the guard is refused and the line runs VERBATIM (kFAIL-perform — over-running is
/// safe; suppressing a side-effect is not). GUARD-ONLY: a `Replace`/`Omit` refusal on redirects is
/// the elide tier's separate concern (Part B), untouched here.
///
/// **Devnull-exemption DEFERRED (ru-26 churn-avoidance, tc-guard-redirect-devnull):** the ruling
/// exempts `>/dev/null`, but resolving the target word to `/dev/null` needs the source/interner the
/// refusal predicates deliberately do not thread. This round refuses ALL output-file redirects
/// (Write/Append with a Word target) — strictly MORE conservative (a devnull guard runs verbatim
/// instead of guarding), which is kFAIL-safe; no corpus case guards a devnull-redirect line, so the
/// exemption is unobservable until one exists. Fd-dups (`>&2`) and input redirects (`<f`) never
/// block (no output side-effect).
fn leaf_has_blocking_output_redirect(ast: &Ast, leaf: AstId) -> bool {
    let (NodeKind::Simple { redirs, .. }
    | NodeKind::Subshell { redirs, .. }
    | NodeKind::Group { redirs, .. }) = &ast.node(leaf).kind
    else {
        return false;
    };
    redirs.iter().any(|&r| {
        matches!(
            &ast.node(r).kind,
            NodeKind::Redir {
                op: RedirOp::Write | RedirOp::Append,
                target: RedirTarget::Word(_),
                ..
            }
        )
    })
}

/// Is this leaf's GUARD render refused (run verbatim + disclosed)? A heredoc leaf (span can't cover
/// the body) OR a non-devnull output-redirect leaf (guarding suppresses the side-effect). The ONE
/// guard-refusal definition, kept in lockstep across [`Plan::collect_edits`] (drop the edit),
/// [`Plan::render_refusal_diagnostics`] (disclose it), [`Plan::guard_refused_asts`] (the why-lens
/// suppresses the "guarded" claim), and the cli's guard why-lane.
fn guard_render_refused(ast: &Ast, leaf: AstId) -> bool {
    leaf_has_heredoc(ast, leaf) || leaf_has_blocking_output_redirect(ast, leaf)
}

/// Is `node` neutralised (its rendered form reproduces its decision without running it)?
/// Used by [`Plan::collect_edits`]'s omit-safety gate: an `Omit` body may only be edited to
/// `:` if its controlling guard is neutralised — else a KEPT (`Run`) guard would re-decide
/// against a removed body (a `kFAIL-perform` under-execute), so a kept-guard `Omit` body
/// renders verbatim (it runs; the runtime guard gates it).
///
/// A `Replace` counts ONLY if the render will actually express it: a render-REFUSED
/// controller (heredoc-bearing, [`leaf_has_heredoc`]) is kept verbatim by `collect_edits`,
/// so its rendered form is the LIVE command — it re-decides at apply time, and the dead
/// body must stay verbatim behind it (it runs when the live guard says so —
/// `kFAIL-perform`). Without this check the licensed-but-refused guard pierced the gate:
/// `dpkg -s nginx <<EOF … || install` rendered a live guard over a `:`-substituted body,
/// exactly the kept-guard/omitted-body configuration this gate forbids (and in the `&&`
/// form its flipped-world list-rc became a fabricated success no probe ever sourced).
/// An `Omit`-disposed node carrying a heredoc is different: its verbatim text sits BEHIND
/// its own controller's frozen short-circuit, so the transitive controller check below is
/// the honest gate for it (no heredoc check on the `Omit` arm).
///
/// A `node` that is a plan LEAF (a [`Step`], so present in `by_ast`) is neutralised iff its
/// disposition is `Replace` (substituted to its stand-in) or an `Omit` whose own controller
/// is neutralised (transitive, depth-capped — `inv-no-throw`). A `node` that is NOT a leaf —
/// a COMPOUND controller (`if`'s condition node, a `! pipeline`, an `&&`/`||`) — is
/// neutralised iff EVERY `Simple` command leaf in its AST subtree is neutralised: a guard
/// whose every command is substituted reproduces the branch decision in the artifact, so the
/// dead body is safe to elide. (At HEAD this fell through to "not neutralised" because a
/// floored guard never elided; arch-1 makes a known-rc guard substitute, so the compound
/// case now matters — `guard-status`/`render21-if-guard-query-elides`.)
fn is_neutralised(
    by_ast: &BTreeMap<AstId, &Disposition>,
    ast: &Ast,
    node: AstId,
    depth: u32,
) -> bool {
    if depth > 64 {
        return false; // defensive: never loop; default to run-it
    }
    if let Some(disposition) = by_ast.get(&node) {
        return match disposition {
            Disposition::Replace(_, _) => !leaf_has_heredoc(ast, node),
            Disposition::Omit { controller } => is_neutralised(by_ast, ast, *controller, depth + 1),
            // A guard controller RUNS its check and MAY run the original ⇒ its decision is NOT
            // reproduced by a `:` body, so (like Run) it is not neutralised (the run-it direction).
            Disposition::Guard(_) | Disposition::Run => false,
        };
    }
    // Not a plan leaf ⇒ a compound controller. Neutralised iff every Simple leaf under it is.
    // An empty subtree (no command leaf — a bare structural node) is vacuously NOT a guard
    // whose decision we must reproduce, but it also reproduces nothing — treat as not
    // neutralised (the safe run-it direction; no current shape reaches it).
    let mut any_leaf = false;
    let all_leaves_neutralised =
        subtree_leaves_all(
            ast,
            node,
            &mut any_leaf,
            &mut |leaf| match by_ast.get(&leaf) {
                // Same render-refusal gate as the leaf arm: a heredoc-bearing Replace is
                // kept verbatim, so it does NOT reproduce the compound's decision.
                Some(Disposition::Replace(_, _)) => !leaf_has_heredoc(ast, leaf),
                Some(Disposition::Omit { controller }) => {
                    is_neutralised(by_ast, ast, *controller, depth + 1)
                }
                _ => false,
            },
        );
    any_leaf && all_leaves_neutralised
}

/// Walk every `Simple` command leaf in `node`'s AST subtree, returning whether `pred` holds
/// for ALL of them (short-circuit `false`). Sets `any` true if at least one leaf was seen.
/// A small recursive descent mirroring the modeled `NodeKind` set (the fold's `kill_rec`
/// shape) — used by [`is_neutralised`] to resolve a COMPOUND controller (an `if`-cond /
/// `! pipeline` / `&&`/`||`) to its guard leaves. Detached funcdef bodies and word/redir
/// nodes carry no command leaf the render reasons about, so they are skipped.
fn subtree_leaves_all(
    ast: &Ast,
    node: AstId,
    any: &mut bool,
    pred: &mut impl FnMut(AstId) -> bool,
) -> bool {
    match &ast.node(node).kind {
        NodeKind::Simple { .. } => {
            *any = true;
            pred(node)
        }
        NodeKind::Script { items } | NodeKind::List { items } => {
            let items = items.clone();
            items.iter().all(|&i| subtree_leaves_all(ast, i, any, pred))
        }
        NodeKind::Pipeline { stages, .. } => {
            let stages = stages.clone();
            stages
                .iter()
                .all(|&s| subtree_leaves_all(ast, s, any, pred))
        }
        NodeKind::AndOr { left, right, .. } => {
            let (left, right) = (*left, *right);
            // Evaluate both (no short-circuit on the AND of the two — both must hold).
            let l = subtree_leaves_all(ast, left, any, pred);
            let r = subtree_leaves_all(ast, right, any, pred);
            l && r
        }
        NodeKind::Subshell { body, .. } | NodeKind::Group { body, .. } => {
            subtree_leaves_all(ast, *body, any, pred)
        }
        NodeKind::If {
            cond,
            then_body,
            elifs,
            else_body,
        } => {
            let cond = *cond;
            let then_body = *then_body;
            let elifs: Vec<AstId> = elifs.iter().flat_map(|e| [e.cond, e.body]).collect();
            let else_body = *else_body;
            let mut ok = subtree_leaves_all(ast, cond, any, pred);
            ok = subtree_leaves_all(ast, then_body, any, pred) && ok;
            for e in elifs {
                ok = subtree_leaves_all(ast, e, any, pred) && ok;
            }
            if let Some(eb) = else_body {
                ok = subtree_leaves_all(ast, eb, any, pred) && ok;
            }
            ok
        }
        NodeKind::Case { arms, .. } => {
            let bodies: Vec<AstId> = arms.iter().map(|a| a.body).collect();
            bodies
                .iter()
                .all(|&b| subtree_leaves_all(ast, b, any, pred))
        }
        NodeKind::ForLoop { body, .. } => subtree_leaves_all(ast, *body, any, pred),
        NodeKind::WhileLoop { cond, body, .. } => {
            let (cond, body) = (*cond, *body);
            let c = subtree_leaves_all(ast, cond, any, pred);
            let b = subtree_leaves_all(ast, body, any, pred);
            c && b
        }
        // funcdef body is detached; word/assign/redir/unsupported carry no command leaf.
        _ => true,
    }
}

/// A round-trippable, unambiguous display label for a fact's re-keyed cell
/// (`notes/193` strain-4, K2's call). Resolves the interned names for
/// *display/provenance* only — never a logic branch (`inv-referent-agnostic`). The
/// cli matches host probe-result lines back to facts by this exact label (it keys a
/// map on the string, never decoding it), so the format is the cli's stdin grammar.
///
/// Two shapes, discriminated by the presence of a `:` *operand* segment:
/// * `kind:entity@selector` for [`EntityRef::Operand`] — `package:nginx@installed`;
/// * `kind@selector` for [`EntityRef::Singleton`] — `package-index@fresh`. A
///   singleton has no operand, so it carries NO `:`-segment (the bare `package-index@fresh`
///   the strain-4 note warned against is avoided — `:` present ⇔ an operand exists).
///
/// The selector is ALWAYS rendered (`@selector`, `281` §R4): it is the per-entity facet the
/// re-key added (`an-per-entity-selector`), and dropping it would let an `is-active`
/// probe-verdict discharge an unmet `@enabled` cell — a wrong-elision under apply's
/// `kFAIL` (`cli/CLAUDE.md` "stdin re-key gotcha"). The label is injective over
/// distinct `FactKey`s modulo a `:`/`@` collision in an interned name (a disposable-
/// parser limitation, `ch-scope`; book operands like `nginx` don't carry them).
#[must_use]
pub fn fact_label(interner: &Interner, fact: FactKey) -> String {
    let kind = interner.resolve(fact.kind.0);
    let selector = interner.resolve(fact.selector.0);
    match fact.entity {
        EntityRef::Operand(tok) => {
            format!("{kind}:{}@{selector}", interner.resolve(tok.0))
        }
        EntityRef::Singleton => format!("{kind}@{selector}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{Interner, KindId, OpaqueToken, ProviderId, SelectorId};
    use dorc_oracle::{KindIndex, ValueClaim};

    /// Corpus-shaped check dialect for the pipeline tests: the `apt-get` check
    /// (flag-strip → verb → `update` Singleton arm `package-index`; else single-operand
    /// `package` with a `[ "$2" = "" ]` multi-operand refusal). Annotation kinds match
    /// the effect-map's, so the kind-agreement rule never fires. Lifted with the test's
    /// interner so provider symbols match the book's command words (204 seam #2).
    const CORPUS_PREDICT_SRC: &str = r#"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      update) test -n fresh : sm.dorc.PkgIndex@fresh ;;
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ; fi ;;
   esac
}
"#;

    const CORPUS_VERDICT_SRC: &str = r"
apt_get__is_converged() { return 0; }
";

    /// R3 test seam: resolve+strip the corpus `apt_get__predict` for a site's (provider, argv),
    /// the same resolution the cli's `ship_predict_body` runs — the FIRST check whose provider
    /// matches and whose argparse resolves this argv, stripped to its runnable funcdef. Returns
    /// `None` when no check resolves (an un-oracled provider / a refused argv), so a test can
    /// spell "un-probeable" by giving a provider the corpus does not model.
    fn ship_corpus(
        checks: &[dorc_oracle::predict::PredictSet],
        interner: &Interner,
        provider: Symbol,
        argv: &[Symbol],
    ) -> Option<ShippedCheck> {
        use dorc_oracle::predict::{Resolution, evaluate, map_provider_name, strip_predict};
        let want = map_provider_name(interner.resolve(provider));
        let arg_texts: Vec<String> = argv
            .iter()
            .map(|s| interner.resolve(*s).to_owned())
            .collect();
        let arg_refs: Vec<&str> = arg_texts.iter().map(String::as_str).collect();
        for cs in checks {
            for cp in cs.providers() {
                if map_provider_name(interner.resolve(cp)) != want {
                    continue;
                }
                let Some(check) = cs.get(cp) else { continue };
                if matches!(evaluate(check, &arg_refs), Resolution::Resolved(_)) {
                    return Some(ShippedCheck::predict(
                        strip_predict(CORPUS_PREDICT_SRC, check, interner),
                        Some((check.name_span, SourceFileId(0))),
                    ));
                }
            }
        }
        None
    }

    /// `package:nginx@installed` — the cell `apt-get install nginx` gates. The
    /// re-key (`notes/193`) made the entity an [`EntityRef`] and added a selector.
    fn nginx_fact() -> FactKey {
        let mut i = Interner::default();
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: SelectorId(i.intern("installed")),
            context: dorc_core::Context::HostDefault,
        }
    }

    /// An empty (provably-quiet) consumption fact in the `May` orientation — the
    /// common case for the `prove_replaceable` unit tests.
    fn quiet() -> May<Powerset<Channel>> {
        May(Powerset::default())
    }

    /// A throwaway reached vouch for the elide-weld tests (24D §3) — the SAME shape
    /// [`Vouches`] carries. Its payload is inert here: the elide-weld consumes the vouch as the
    /// TIER CHECK, never reads its bytes (contrast the guard mint, which ships them).
    fn test_vouch() -> ByVouch<VerdictVouch> {
        ByVouch::vouched(
            VerdictVouch::new(
                "apt_get__is_converged".to_string(),
                "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
                "apt_get__is_converged install -y nginx".to_string(),
                "package".to_string(),
                vec!["dpkg-query".to_string()],
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            ),
            Rung::Both,
        )
    }

    /// Test convenience (elide-weld, 24D §3): vouch every AMBIENT establish site so the
    /// plan-mechanics helpers keep exercising ELISION. Deliberately NOT `EstablishProbeWritten` — a
    /// vouched+converged written site fires the GUARD tier (`Disposition::Guard`), which these
    /// elision/wall/fold tests do not expect (guards are pinned by the guard23 e2e + the guard
    /// unit tests). The vouch GATE is pinned by [`no_license_for_ambient_without_vouch`] + e2e +
    /// the FAITHFUL sweep/coverage lift; here a synthetic vouch (no oracle lift) keeps focus.
    fn vouch_all(classes: &[(CfgNodeId, SkipClass)]) -> Vouches {
        let mut vouches = Vouches::new();
        for (node, class) in classes {
            match class {
                // BOTH establish species: the origin ambient/written split no longer gates the tier,
                // so a scaffold that vouched only ambient sites would hide the guard rung from
                // every native test (`30K` §5.1).
                SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
                    vouches.insert(*node, *fact, test_vouch());
                }
                SkipClass::EstablishMembers { members, .. } => {
                    for fact in members {
                        vouches.insert(*node, *fact, test_vouch());
                    }
                }
                SkipClass::InlineCall { sites } => {
                    for site in sites {
                        if let SkipClass::EstablishProbeAmbient(fact)
                        | SkipClass::EstablishProbeWritten(fact) = site.class
                        {
                            vouches.insert(site.node, fact, test_vouch());
                        }
                    }
                }
                SkipClass::QueryResolvable { .. } | SkipClass::MustRun => {}
            }
        }
        vouches
    }

    // ---- the guard tier (24D §2): mint-policy + emitter shape (rul-ternary-verdict) ----

    #[test]
    fn guard_mints_only_on_a_converged_probe_verdict() {
        use dorc_core::{ByVouch, Rung};
        let vouch = || {
            VerdictVouch::new(
                "apt_get__is_converged".to_string(),
                "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
                "apt_get__is_converged install -y curl".to_string(),
                "package".to_string(),
                vec!["dpkg-query".to_string()],
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            )
        };
        // jc-mint-policy m-a: a diverged/unknown probe-verdict NEVER guards (a guard at a
        // predicted-change site buys nothing; `inv-kfail` → run). The mint DEMANDS a
        // `ByVouch<VerdictVouch>` (TC-tier-2) — a fact/silence claim would not typecheck here.
        assert!(
            GuardLicense::mint(
                nginx_fact(),
                ByVouch::vouched(vouch(), Rung::Both),
                Verdict::Diverged,
                &quiet(),
            )
            .is_none(),
            "a diverged probe-verdict must not mint a guard"
        );
        assert!(
            GuardLicense::mint(
                nginx_fact(),
                ByVouch::vouched(vouch(), Rung::Both),
                Verdict::Unknown,
                &quiet(),
            )
            .is_none(),
            "an unknown probe-verdict must not mint a guard"
        );
        let license = GuardLicense::mint(
            nginx_fact(),
            ByVouch::vouched(vouch(), Rung::Both),
            Verdict::Converged,
            &quiet(),
        )
        .expect("a converged probe-verdict + vouch mints a guard");
        assert_eq!(license.fact(), nginx_fact());
    }

    #[test]
    fn converged_guard_emitter_shape_obeys_the_two_never_clauses() {
        use dorc_core::{ByVouch, Rung};
        let vouch = VerdictVouch::new(
            "apt_get__is_converged".to_string(),
            "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
            "apt_get__is_converged install -y curl".to_string(),
            "package".to_string(),
            vec!["dpkg-query".to_string()],
            dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
        );
        let license = GuardLicense::mint(
            nginx_fact(),
            ByVouch::vouched(vouch, Rung::Both),
            Verdict::Converged,
            &quiet(),
        )
        .unwrap();
        // The guard_shape law: `( <check> ) || <original verbatim>   # dorc: guard [...]`.
        let line = license
            .insert()
            .render_line("apt-get install -y curl", "apt_get__is_converged");
        assert!(
            line.starts_with(
                "( apt_get__is_converged install -y curl ) || apt-get install -y curl"
            ),
            "converged direct glue + verbatim original: {line}"
        );
        assert!(
            line.contains("# dorc: guard [package converged-vouch; probe: holds]"),
            "attribution comment: {line}"
        );
        // The bytes after the FIRST ` || ` are the verbatim original (never-2 / bytes-verbatim —
        // exactly what guard_shape_check asserts).
        let (_lhs, rhs) = line.split_once(" || ").unwrap();
        let rhs_code = rhs.split("   #").next().unwrap();
        assert_eq!(
            rhs_code, "apt-get install -y curl",
            "original survives verbatim"
        );
    }

    /// Two sites, one funcname, one BODY ⇒ one hoisted definition under the plain name, and both
    /// guards invoke it. The content-dedup half of `28K` §4.
    #[test]
    fn guard_preamble_dedups_and_counts() {
        use dorc_core::{ByVouch, Rung};
        let mk = |leaf: u32| {
            let vouch = VerdictVouch::new(
                "apt_get__is_converged".to_string(),
                "apt_get__is_converged() { dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string(),
                "apt_get__is_converged install curl".to_string(),
                "package".to_string(),
                vec!["dpkg-query".to_string()],
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            );
            Step {
                leaf: LeafId(leaf),
                ast: AstId(leaf),
                sh: "apt-get install curl".to_string(),
                disposition: Disposition::Guard(
                    GuardLicense::mint(
                        nginx_fact(),
                        ByVouch::vouched(vouch, Rung::Both),
                        Verdict::Converged,
                        &quiet(),
                    )
                    .unwrap(),
                ),
            }
        };
        let plan = Plan {
            steps: vec![mk(0), mk(1)],
            regions: Vec::new(),
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        };
        // A throwaway (empty) Ast: the synthetic `AstId`s index no real node, and the OOB-safe
        // check treats an out-of-arena id as not-refused (so both guards pin).
        let ast = dorc_syntax::parse("").value;
        let pinned = plan.pinned_definitions("", &ast);
        assert_eq!(
            pinned.hoisted().matches("apt_get__is_converged()").count(),
            1,
            "one BODY ⇒ one hoist: {}",
            pinned.hoisted()
        );
        assert_eq!(
            pinned.invoked(AstId(0)),
            Some("apt_get__is_converged"),
            "the single-definition case keeps the authored name, byte-identical to strip"
        );
        assert_eq!(pinned.invoked(AstId(1)), pinned.invoked(AstId(0)));
        // The exhaustive `disposition_counts` match now feeds the guard bucket (the summary's
        // guard column becomes real — DispositionCounts forced this wiring).
        let counts = plan.disposition_counts();
        assert_eq!(counts.guard, 2);
        assert_eq!(counts.sites, 2);
    }

    /// Build a two-guard plan whose sites carry the given verdict BODIES under one funcname.
    #[cfg(test)]
    fn two_guard_plan(bodies: [&str; 2]) -> Plan {
        use dorc_core::{ByVouch, Rung};
        let step = |leaf: u32, preamble: &str| {
            let vouch = VerdictVouch::new(
                "apt_get__is_converged".to_string(),
                preamble.to_string(),
                "apt_get__is_converged install curl".to_string(),
                "package".to_string(),
                Vec::new(),
                dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
            );
            Step {
                leaf: LeafId(leaf),
                ast: AstId(leaf),
                sh: "apt-get install curl".to_string(),
                disposition: Disposition::Guard(
                    GuardLicense::mint(
                        nginx_fact(),
                        ByVouch::vouched(vouch, Rung::Both),
                        Verdict::Converged,
                        &quiet(),
                    )
                    .unwrap(),
                ),
            }
        };
        Plan {
            steps: vec![step(0, bodies[0]), step(1, bodies[1])],
            regions: Vec::new(),
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        }
    }

    /// `28K` §4 `rul-hash-munge-disambiguation`, and the pope-sin it closes.
    ///
    /// Two sites whose live definitions are DIFFERENT bodies under one funcname. The retired
    /// dedup-by-funcname emitted the first body only and let BOTH sites invoke it, so one site ran
    /// a judgment its author never made for that line — a mis-attribution
    /// (`271:rul-sin-ordering`'s worst class) that no golden could show. Now each body is emitted
    /// once under its own disambiguated name and each site invokes its own.
    #[test]
    fn two_distinct_bodies_under_one_name_are_hash_munged_apart() {
        let a = "apt_get__is_converged() { dpkg-query -W \"$1\" ; }";
        let b = "apt_get__is_converged() { dpkg-query -W --strict \"$1\" ; }";
        let plan = two_guard_plan([a, b]);
        let ast = dorc_syntax::parse("").value;
        let pinned = plan.pinned_definitions("", &ast);
        let (first, second) = (
            pinned.invoked(AstId(0)).expect("site 0 guards"),
            pinned.invoked(AstId(1)).expect("site 1 guards"),
        );
        assert_ne!(first, second, "distinct bodies never share a name");
        for name in [first, second] {
            assert!(
                name.starts_with("apt_get__is_converged_h"),
                "the authored name stays readable in the munged one: {name}"
            );
            assert!(
                dorc_oracle::reserved::role_family(name).is_none(),
                "a munged name must not parse as a role, or a re-ingested artifact would read the \
                 guard as a description instead of an opaque call (`23A:P-reingest`): {name}"
            );
            assert_eq!(
                pinned.hoisted().matches(&format!("{name}()")).count(),
                1,
                "each body emitted exactly once, under its own name:\n{}",
                pinned.hoisted()
            );
        }
        assert!(
            !pinned.hoisted().contains("apt_get__is_converged()"),
            "the plain name binds nothing when the unit holds two bodies:\n{}",
            pinned.hoisted()
        );
        assert_eq!(
            pinned
                .hoisted()
                .matches("# dorc: pinned definition of `apt_get__is_converged`")
                .count(),
            2,
            "each munged body names the AUTHORED function it is, or a reader cannot answer whose \
             judgment runs:\n{}",
            pinned.hoisted()
        );
    }

    /// The artifact never carries two same-named funcdefs BY ANY ROUTE (`28K` §4). When the pinned
    /// definition is the book's own — the stage-3 in-book oracle — the artifact already holds it at
    /// its authored position, so hoisting a copy would put the very shape `oracle/src/reserved.rs`
    /// refuses into the shipped bytes. Nothing is re-derived: the positional regime licenses a
    /// vouch only where its definition is live, so a book-sited definition always precedes its
    /// guards.
    #[test]
    fn a_definition_the_book_already_carries_is_not_copied_above_it() {
        let body = "apt_get__is_converged() { dpkg-query -W \"$1\" ; }";
        let src = format!("{body}\napt-get install curl\n");
        let ast = dorc_syntax::parse(&src).value;
        let plan = two_guard_plan([body, body]);
        let pinned = plan.pinned_definitions(&src, &ast);
        assert_eq!(
            pinned.hoisted(),
            "",
            "the book's own definition IS the pin — no second funcdef ships"
        );
        assert_eq!(pinned.invoked(AstId(0)), Some("apt_get__is_converged"));
    }

    /// The same law, with a SNAPSHOT riding along — the shape that was silently breaking it.
    ///
    /// Measured on this tree: while a vouch carried one blob of closure-plus-definition, the
    /// already-in-place test compared the book's funcdef span against that blob, never matched, and
    /// hoisted a COPY of the book's own body above the book. Two same-named top-level funcdefs in the
    /// emitted artifact, which is exactly what `28K` §4 retires by any route, and it reached the
    /// corpus (`pin28-helper-package-entrypoints-discarded`). Splitting the two makes the comparison
    /// answerable: the snapshot hoists, the book's body stays where its author put it.
    #[test]
    fn a_book_carried_definition_with_a_closure_hoists_only_the_closure() {
        use dorc_core::{ByVouch, Rung};
        let helper = "_apt_dest() { printf '%s\\n' \"$1\" ; }";
        let body = "apt_get__is_converged() { dpkg-query -W \"$(_apt_dest \"$1\")\" ; }";
        let src = format!("{body}\napt-get install curl\n");
        let ast = dorc_syntax::parse(&src).value;
        let helpers = dorc_oracle::closure::HelperIndex::build(&[helper], None);
        let closure = helpers
            .closure_for(0, body)
            .expect("one source declares one helper");
        let vouch = VerdictVouch::new(
            "apt_get__is_converged".to_string(),
            body.to_string(),
            "apt_get__is_converged install curl".to_string(),
            "package".to_string(),
            Vec::new(),
            dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
        )
        .with_closure(&closure);
        assert!(
            vouch.preamble.starts_with("_apt_dest() {"),
            "the probe lane still reads one concatenated unit: {}",
            vouch.preamble
        );
        let plan = Plan {
            steps: vec![Step {
                leaf: LeafId(0),
                ast: AstId(0),
                sh: "apt-get install curl".to_string(),
                disposition: Disposition::Guard(
                    GuardLicense::mint(
                        nginx_fact(),
                        ByVouch::vouched(vouch, Rung::Both),
                        Verdict::Converged,
                        &quiet(),
                    )
                    .unwrap(),
                ),
            }],
            regions: Vec::new(),
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        };
        let pinned = plan.pinned_definitions(&src, &ast);
        assert_eq!(
            pinned.hoisted(),
            format!("{helper}\n"),
            "the snapshot ships; the book's own body does not travel"
        );
        assert_eq!(pinned.invoked(AstId(0)), Some("apt_get__is_converged"));
    }

    /// Two guards reaching ONE helper emit it once. The snapshot is hoisted above the whole book, so
    /// a per-body copy would put two same-named funcdefs in the preamble — the same law, from the
    /// other side.
    #[test]
    fn one_helper_reached_by_two_guards_is_emitted_once() {
        use dorc_core::{ByVouch, Rung};
        let helper = "_apt_dest() { printf '%s\\n' \"$1\" ; }";
        let body = "apt_get__is_converged() { dpkg-query -W \"$(_apt_dest \"$1\")\" ; }";
        let helpers = dorc_oracle::closure::HelperIndex::build(&[helper], None);
        let closure = helpers.closure_for(0, body).expect("one source");
        let step = |leaf: u32| Step {
            leaf: LeafId(leaf),
            ast: AstId(leaf),
            sh: "apt-get install curl".to_string(),
            disposition: Disposition::Guard(
                GuardLicense::mint(
                    nginx_fact(),
                    ByVouch::vouched(
                        VerdictVouch::new(
                            "apt_get__is_converged".to_string(),
                            body.to_string(),
                            "apt_get__is_converged install curl".to_string(),
                            "package".to_string(),
                            Vec::new(),
                            dorc_core::DefinitionCustody::of_defining_file(SourceFileId(0)),
                        )
                        .with_closure(&closure),
                        Rung::Both,
                    ),
                    Verdict::Converged,
                    &quiet(),
                )
                .unwrap(),
            ),
        };
        let plan = Plan {
            steps: vec![step(0), step(1)],
            regions: Vec::new(),
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        };
        let ast = dorc_syntax::parse("").value;
        let pinned = plan.pinned_definitions("", &ast);
        let hoisted = pinned.hoisted();
        assert_eq!(
            hoisted.matches("_apt_dest() {").count(),
            1,
            "one declaration, one emission:\n{hoisted}"
        );
        assert_eq!(
            hoisted.matches("apt_get__is_converged()").count(),
            1,
            "and one body, since both guards resolved the same one:\n{hoisted}"
        );
    }

    /// `p-book-collision-forces-non-idiomatic` (`30A` §2 P-green) — a book funcdef sharing an emitted
    /// name under DIFFERENT bytes forces the ENGINE's definition to munge, and leaves the book's own
    /// binding alone. Two cells: the book only defines the name, and the book defines AND calls it.
    ///
    /// The sh fact: a top-level funcdef binds its name for every call below it, and the emitted
    /// preamble sits ABOVE the whole book, so whichever body the preamble bound would be rebound by
    /// the book's own definition for the book's own calls — and, if the preamble used the bare name,
    /// the book's definition would be a SECOND top-level funcdef under it. Both halves land on
    /// `rul-happy-path-is-a-closed-set`: idiomatic bare-name emission is licensed only where the
    /// engine has ENUMERATED the names in play, and a book that claims one is exactly the case that
    /// cannot be enumerated away.
    ///
    /// Why an engine choice depends on it: emitting the bare name here would put the guard's call and
    /// the book's call on the same name with two different bodies underneath, and which one ran would
    /// depend on where in the artifact the reader looked — pope-sin tier
    /// (`271:rul-sin-ordering`), and invisible to a text golden.
    ///
    /// NOTE the aid-plane gap this measurement surfaced, recorded not fixed: the provenance comment
    /// (`# dorc: pinned definition of …`) rides the PLURAL branch only, so a munge forced by a book
    /// collision emits a bare `<name>_h<digest>()` with nothing saying whose judgment it is.
    #[test]
    fn a_book_funcdef_under_an_emitted_name_forces_the_engine_to_munge() {
        let body = "apt_get__is_converged() { dpkg-query -W \"$1\" ; }";
        let book_body = "apt_get__is_converged() { printf 'always converged\\n' ; }";
        let plan = two_guard_plan([body, body]); // content-dedup ⇒ ONE engine definition

        let bare = dorc_syntax::parse("apt-get install curl\n").value;
        let unclaimed = plan.pinned_definitions("apt-get install curl\n", &bare);
        assert_eq!(
            unclaimed.invoked(AstId(0)),
            Some("apt_get__is_converged"),
            "control: with the name unclaimed the emission is idiomatic — the authored name, bare"
        );

        for (cell, src) in [
            (
                "defines only",
                format!("{book_body}\napt-get install curl\n"),
            ),
            (
                "defines and calls",
                format!("{book_body}\napt_get__is_converged install curl\napt-get install curl\n"),
            ),
        ] {
            let ast = dorc_syntax::parse(&src).value;
            let pinned = plan.pinned_definitions(&src, &ast);
            let invoked = pinned
                .invoked(AstId(0))
                .expect("the site still guards, under a disambiguated name");
            assert!(
                invoked.starts_with("apt_get__is_converged_h"),
                "{cell}: the ENGINE's definition munges — {invoked}"
            );
            assert!(
                !pinned.hoisted().contains("apt_get__is_converged()"),
                "{cell}: and the preamble binds the bare name NOWHERE, so the book's own definition \
                 is the artifact's only binding for it and its own calls are untouched:\n{}",
                pinned.hoisted()
            );
            assert!(
                pinned.hoisted().contains(&format!("{invoked}()")),
                "{cell}: the engine's body is still emitted, under its own name:\n{}",
                pinned.hoisted()
            );
        }
    }

    /// `p-defensive-forced-fallback` (`30A` §2 P-green) — under DEFENSIVE emission every emitted name
    /// munges, including a lone body no collision touches.
    ///
    /// The sh fact this is conservative about: a real definition vector (`alias`, an unresolvable
    /// load) can bind a name in the executing shell that no static walk saw, so the set of names in
    /// play is not enumerable — and `rul-happy-path-is-a-closed-set` says the idiomatic tier is
    /// licensed ONLY by enumeration. Not by absence of evidence: by proof. So the fallback is
    /// munge-everything, in all cases.
    ///
    /// Why an engine choice depends on it: the fallback is what makes idiomatic emission SAFE to build
    /// gradually. A defensive world that still emitted one bare name would be a name an `alias` could
    /// have rebound, and the guard would call the alias instead of the vouched body.
    ///
    /// The other half of the rule — that a WRAPPER's command-position `"$@"` must not trigger this,
    /// because the parser folds that ⊤-reject in with `eval` and keying on it would put every wrapper
    /// oracle in the world into defensive emission — is pinned where the detection lives
    /// (`dorc_oracle::closure`'s `definition_vectors_ignore_unmodeled_commands_and_top_rejects`).
    #[test]
    fn defensive_emission_munges_even_an_uncontested_singleton() {
        let body = "apt_get__is_converged() { dpkg-query -W \"$1\" ; }";
        let ast = dorc_syntax::parse("").value;
        let mut plan = two_guard_plan([body, body]);
        assert_eq!(
            plan.pinned_definitions("", &ast).invoked(AstId(0)),
            Some("apt_get__is_converged"),
            "control: one body, no collision, no vector ⇒ the authored name, bare"
        );

        plan.defensive_emission = true;
        let pinned = plan.pinned_definitions("", &ast);
        let invoked = pinned.invoked(AstId(0)).expect("the site still guards");
        assert!(
            invoked.starts_with("apt_get__is_converged_h"),
            "a definition vector anywhere in the unit munges every emitted name, collision or not — \
             {invoked}"
        );
        assert!(
            !pinned.hoisted().contains("apt_get__is_converged()"),
            "no bare name survives for an `alias` to rebind:\n{}",
            pinned.hoisted()
        );
        assert_eq!(
            pinned.invoked(AstId(1)),
            Some(invoked),
            "both sites still reach ONE definition — defensive emission renames, it does not split"
        );
    }

    /// A ONE-guard plan carrying `body` — the single-use half of the placement pair below.
    #[cfg(test)]
    fn one_guard_plan(body: &str) -> Plan {
        let mut plan = two_guard_plan([body, body]);
        plan.steps.truncate(1);
        plan
    }

    /// `p-x-placement-tuning-pair` — THE TARGET: a colliding body used ONCE is colocated at its site
    /// rather than munged and lifted above the whole book.
    ///
    /// The sh fact that makes colocation the idiomatic answer: a guard's check already runs inside
    /// `( … )` (`rul-ternary-verdict`'s shape), and a subshell is its own environment — so a definition
    /// placed there binds for that check and nothing else, and the collision with the book's name
    /// simply does not exist. Measured under `posh ∩ dash` by `floor28-subshell-scoped-re-source`. The
    /// munge is what you need when a body must be reachable from SEVERAL sites above the book; for one
    /// site it buys nothing and costs the reader a `_h<digest>` name.
    ///
    /// The pair, and why both halves are here. MANY-USE stays top-lifted and munged — that IS its
    /// idiomatic form, it is today's behaviour, and it is asserted below as the control so the target
    /// cannot be satisfied by simply ceasing to emit. ONCE-USED is the xfail.
    ///
    /// The assertion is deliberately about PLACEMENT and not about the emission API: whichever channel
    /// a colocated definition eventually travels on, the preamble must not name it, and the site must
    /// still guard. Asserting anything about `invoked()` would pin an API the planner has not designed.
    ///
    /// FAILS TODAY: the once-used collider munges and hoists exactly as the many-use one does, because
    /// `pinned_definitions` has one placement and no notion of use-count.
    #[test]
    fn a_once_used_colliding_body_is_colocated_rather_than_lifted() {
        let body = "apt_get__is_converged() { dpkg-query -W \"$1\" ; }";
        let book_body = "apt_get__is_converged() { printf 'always converged\\n' ; }";
        let src = format!("{book_body}\napt-get install curl\n");
        let ast = dorc_syntax::parse(&src).value;

        // Control (the many-use half): two sites reaching one body, colliding with the book's name.
        // Top-lift plus munge is the idiomatic answer here and must stay.
        let many = two_guard_plan([body, body]).pinned_definitions(&src, &ast);
        assert_eq!(
            many.hoisted().matches("apt_get__is_converged_h").count(),
            1,
            "the many-use half lifts ONE munged definition above the book:\n{}",
            many.hoisted()
        );

        // Setup outside the closure: a panic in there would read as the target still failing.
        let plan = one_guard_plan(body);
        let guards = plan.disposition_counts().guard;
        let once = plan.pinned_definitions(&src, &ast);
        let hoisted = once.hoisted().to_owned();
        internal_tooling::xfail::xfail_until("p-x-placement-tuning-pair", || {
            assert_eq!(guards, 1, "the single site still guards");
            assert!(
                !hoisted.contains("apt_get__is_converged"),
                "one site needs no name above the book at all — the check's own `( … )` is a scope, \
                 so the definition belongs there and the collision evaporates:\n{hoisted}"
            );
        });
    }

    /// Run the real pipeline (parse → cfg → value-flow → classify → `compile_probe`) on
    /// `src`; `probeable` gates whether the corpus apt check ships (R3: the whole stripped
    /// funcdef via [`ship_corpus`]). Returns the site-keyed [`ProbePlan`] + the interner (for
    /// `render_sh`). The corpus apt check resolves identity either way. This is the
    /// honest site-keyed shape (`inv-site-keyed-results`): the synthetic-`CfgNodeId`
    /// fact-keyed tests of spike-2 could not exercise `site_order`.
    ///
    /// `probeable` picks whether the corpus apt check ships (R3: the whole stripped
    /// `apt_get__predict` funcdef, invoked per-site with the site's argv). `true` ⇒ every
    /// site the apt argparse resolves is checked; `false` ⇒ the ship closure returns `None`
    /// for all, spelling "un-probeable" (⇒ all sites unresolvable). A provider the corpus
    /// does not model (`systemctl`) is un-probeable regardless (no check resolves it).
    fn probe_for_src(src: &str, probeable: bool) -> (ProbePlan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |_, provider, argv| {
                if probeable {
                    ship_corpus(&checks, &i, provider, argv)
                } else {
                    None
                }
            },
            |_, _, _, _| None,
            |_, _| false,
        );
        (probe, i)
    }

    /// `ship-seam-reads-the-lane-not-the-kind` (`26H` §3.5): where a site is in the VERDICT lane
    /// AND also carries a resolvable predict, the verdict body ships. The two closures are tried
    /// in order and that order is load-bearing — a verdict-lane site's cell is owned by the
    /// verdict body, so shipping the predict would MEASURE a different cell than the record KEYS.
    ///
    /// That failure is invisible to every golden in the corpus: the artifact still contains a
    /// well-formed check and a well-formed record, the site count is unchanged, and only the
    /// coordinate the measurement actually answers has moved. Nothing else pinned this ordering,
    /// so the fixture-level evidence for it was zero (`28K` build; pin added before the
    /// resolution rewiring touches these closures).
    #[test]
    fn a_verdict_lane_site_ships_the_verdict_body_over_a_resolvable_predict() {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse("apt-get install -y nginx\n");
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |_, _, _| Some(ShippedCheck::predict("PREDICT_BODY".to_owned(), None)),
            |_, _, _, _| {
                Some(ShippedCheck::verdict(
                    "VERDICT_BODY".to_owned(),
                    None,
                    false,
                ))
            },
            |_, _| true,
        );
        let shipped = probe
            .checks
            .iter()
            .find(|c| c.site_kind == ProbeSiteKind::Establish)
            .expect("the establish site ships a check");
        assert!(
            shipped.verdict,
            "the verdict lane must win the try-order: {shipped:?}"
        );
        assert_eq!(shipped.sh, "VERDICT_BODY", "{shipped:?}");
    }

    /// The same seam's negative half, and what makes the pin above non-vacuous: with the site NOT
    /// declared verdict-lane, the identical inputs ship the PREDICT. So the discriminator really is
    /// the caller's per-SITE lane declaration (`verdict-lane-is-site-keyed`) — not the fact's kind,
    /// and not which closure happens to answer first.
    #[test]
    fn the_same_site_ships_the_predict_when_it_is_not_verdict_lane() {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse("apt-get install -y nginx\n");
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |_, _, _| Some(ShippedCheck::predict("PREDICT_BODY".to_owned(), None)),
            |_, _, _, _| None,
            |_, _| true,
        );
        let shipped = probe
            .checks
            .iter()
            .find(|c| c.site_kind == ProbeSiteKind::Establish)
            .expect("the establish site ships a check");
        assert!(!shipped.verdict, "{shipped:?}");
        assert_eq!(shipped.sh, "PREDICT_BODY", "{shipped:?}");
    }

    #[test]
    fn disposition_counts_tally_bucketing_and_sites_invariant() {
        // plans/240 Stage-1 yardstick: the plan-summary's per-disposition tally. Pin
        // (1) each disposition lands in its own bucket, (2) `guard` is 0 at HEAD (no
        // `Disposition` mints one until the Stage-3 guard tier), and (3) the
        // `sites == elide + omit + guard + run` invariant the greppable grammar promises.
        let fact = nginx_fact();
        let license = ReplaceLicense::prove_replaceable::<Apply>(
            fact,
            Grade::Must,
            PhasedVerdict::new(Verdict::Converged),
            quiet(),
            Predicted::Top,
            Some(test_vouch()),
        )
        .expect("a converged, ambient, Must fact with no consumption mints a Replace license");
        let step = |leaf: u32, disposition: Disposition| Step {
            leaf: LeafId(leaf),
            ast: AstId(leaf),
            sh: String::new(),
            disposition,
        };
        let plan = Plan {
            steps: vec![
                step(0, Disposition::Replace(license.clone(), StandIn::True)),
                step(1, Disposition::Replace(license, StandIn::True)),
                step(
                    2,
                    Disposition::Omit {
                        controller: AstId(0),
                    },
                ),
                step(3, Disposition::Run),
            ],
            regions: Vec::new(),
            survival_report: SurvivalReport::default(),
            defensive_emission: false,
        };
        let c = plan.disposition_counts();
        assert_eq!(c.sites, 4, "four leaves");
        assert_eq!(c.elide, 2, "two Replace ⇒ elide=2");
        assert_eq!(c.omit, 1, "one Omit ⇒ omit=1");
        assert_eq!(c.guard, 0, "no Disposition mints a guard at HEAD");
        assert_eq!(c.run, 1, "one Run ⇒ run=1");
        assert_eq!(
            c.sites,
            c.elide + c.omit + c.guard + c.run,
            "the summary grammar's partition invariant"
        );

        // The empty plan tallies to all-zero (the yardstick's honest floor for a probe-only
        // or no-command book).
        assert_eq!(
            Plan {
                steps: vec![],
                regions: Vec::new(),
                survival_report: SurvivalReport::default(),
                defensive_emission: false,
            }
            .disposition_counts(),
            DispositionCounts::default()
        );
    }

    #[test]
    fn compile_probe_resolvable_sites_probed_unresolvable_recorded() {
        // The probe = EstablishProbeAmbient sites WITH a declared read-only probe. A site
        // whose kind has an effect but NO probe is un-checkable ⇒ NOT invoked, recorded
        // `unresolvable` (can't-probe ⇒ can't-elide, kFAIL-perform). A MustRun site
        // (the un-oracled `systemctl reload`) is likewise unresolvable. Here only
        // `package` has a probe, so `install nginx` is the one resolvable site; the
        // reload is unresolvable.
        let (probe, _i) = probe_for_src("apt-get install -y nginx\nsystemctl reload nginx\n", true);
        assert_eq!(probe.checks.len(), 1, "one resolvable site (the install)");
        assert_eq!(
            probe.checks[0].site,
            LeafId(0),
            "the install is the first source command ⇒ site 0"
        );
        assert!(
            !probe.unresolvable.is_empty(),
            "the un-oracled reload is recorded unresolvable: {probe:?}"
        );
    }

    #[test]
    fn verdict_decline_leaf_index_matches_build_plan() {
        // `tc-verdictdecline-site-leaf-source`: `build_vouches` keys a `VerdictDecline` by the
        // enumeration index over `classes` — the SAME positional leaf `compile_probe` assigns, so a
        // report record's `site=<key>` pairs to the right decline. Two declining sites pin it.
        let src = "apt-get install -y curl\napt-get install -y nginx\n";
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let verdict_src = "apt_get__is_converged() { return 2 ; }"; // always declines ⇒ two declines
        let (_vouches, narrative) = build_vouches(
            &[verdict_src],
            &[],
            &dorc_oracle::closure::HelperIndex::default(),
            &classes,
            &value,
            &mut i,
            dorc_analysis::funcenv::LiveDefinitions::unsolved(),
        );
        let mut decline_leaves: Vec<u32> = narrative
            .narrative
            .iter()
            .filter_map(|ev| match ev.kind() {
                CollapseKind::VerdictDecline { site, .. } => Some(site.leaf.0),
                _ => None,
            })
            .collect();
        decline_leaves.sort_unstable();
        assert_eq!(
            decline_leaves,
            vec![0, 1],
            "the two declining establish sites mint VerdictDecline at leaves 0 and 1 (positional)"
        );
        // The SAME classes slice → the probe's positional leaves agree with the declines'.
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |_, provider, argv| ship_corpus(&checks, &i, provider, argv),
            |_, _, _, _| None,
            |_, _| false,
        );
        let mut probe_leaves: Vec<u32> = probe.checks.iter().map(|c| c.site.0).collect();
        probe_leaves.sort_unstable();
        assert_eq!(
            probe_leaves, decline_leaves,
            "the probe checks key by the same positional leaves the declines do"
        );

        // The AGREEMENT direction (`289:rul-mint-hardening-package` item 4a): a body that REACHES
        // its check vouches rather than declines, so the same two sites mint no `VerdictDecline`.
        let vouching_src = "apt_get__is_converged() { dpkg -s \"$2\" : package:\"$2\"@installed ;}";
        let (_vouches, none) = build_vouches(
            &[vouching_src],
            &[],
            &dorc_oracle::closure::HelperIndex::default(),
            &classes,
            &value,
            &mut i,
            dorc_analysis::funcenv::LiveDefinitions::unsolved(),
        );
        assert!(
            !none
                .narrative
                .iter()
                .any(|ev| matches!(ev.kind(), CollapseKind::VerdictDecline { .. })),
            "a reached, vouching verdict body is no collapse and narrates nothing: {none:?}"
        );
    }

    #[test]
    fn compile_probe_no_probe_for_kind_makes_site_unresolvable() {
        // can't-probe ⇒ can't-elide: with NO probe body for any kind, an otherwise
        // ambient install site is unresolvable (not invoked) ⇒ the apply runs it.
        let (probe, _i) = probe_for_src("apt-get install -y nginx\n", false);
        assert!(probe.checks.is_empty(), "no probe ⇒ no resolvable site");
        assert_eq!(
            probe.unresolvable,
            vec![LeafId(0)],
            "the un-probeable site is recorded: {probe:?}"
        );
    }

    #[test]
    fn compile_derivations_ships_escalated_wall_candidate_and_renders_deriv_scaffold() {
        // 24E §2: a WALL-CANDIDATE site whose touches() ESCALATED (the closure returns a ship)
        // becomes one ProbeDerivation; render_sh appends the stripped __disturbs def + a per-site
        // `deriv N coord=` scaffold to the phase-1 probe (no second shebang). A non-escalating
        // provider (the un-oracled systemctl, not even a wall candidate) yields no derivation.
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse("apt-get install -y nginx\nsystemctl reload nginx\n");
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let (classes, _why, kills, _kill_coords, _fact_backings, _narrative, _invalidators) =
            dorc_analysis::effect::classify_with_why_diags(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &dorc_oracle::verdict::VerdictIndex::default(),
                &BTreeMap::new(),
                &dorc_analysis::erase::ErasedSites::none(),
                &mut i,
                &mut dorc_core::ProvArena::new(),
                &mut BTreeMap::new(),
                &mut BTreeMap::new(),
                &mut dorc_analysis::certify::CertifierTrip::default(),
                dorc_analysis::funcenv::LiveDefinitions::unsolved(),
            );
        let classes = classes.value;
        let derivations = compile_derivations(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &kills,
            |_node, provider, _argv| {
                // Escalate ONLY apt-get (the payload-bound install); everything else declines. The
                // forward munge keys the book word `apt-get` on the segment `apt_get`.
                (dorc_oracle::predict::map_provider_name(i.resolve(provider)) == "apt_get").then(
                    || DerivationShip {
                        sh: "apt_get__disturbs() { apt-manifest \"$1\"; }".to_string(),
                        call: "apt-manifest".to_string(),
                    },
                )
            },
        );
        assert_eq!(
            derivations.derivations.len(),
            1,
            "only the apt-get install (an EstablishProbeAmbient wall candidate) escalated"
        );
        assert_eq!(
            derivations.derivations[0].site,
            LeafId(0),
            "the install is site 0"
        );
        let sh = derivations.render_sh(&records::Nonce::spike_default(), &i);
        assert!(
            sh.contains("apt_get__disturbs() { apt-manifest"),
            "the stripped touches def ships verbatim: {sh}"
        );
        assert!(
            sh.contains("| { _n=0; while IFS= read -r _c; do printf 'dorc deriv 0 coord=%s"),
            "the per-site deriv readback scaffold renders (framed, counting subshell): {sh}"
        );
        assert!(
            sh.contains(
                "printf 'dorc deriv-end 0 n=%s body-rc=%s @@dorc@@\\n' \"$_n\" \"$_dr\"; }"
            ),
            "the at-most family closes with a count AND the emitting body's termination status \
             (262 §2 / 26A stop-1 + 28P dec-whole-body-atomic-refusal): {sh}"
        );
        assert!(
            sh.contains("_dr=$?"),
            "the body's status is captured BEFORE the record pipe — a pipeline's status is its \
             RHS's, so the pre-28P scaffold could not see a body death: {sh}"
        );
        assert!(
            !sh.starts_with("#!/bin/sh"),
            "no second shebang — the derivation-probe rides the SAME phase-1 block: {sh}"
        );
    }

    /// `289:rul-touches-mismatch-own-lane` — the shipped DEF name and the INVOKED name of the
    /// derivation lane must be one string. They drifted apart once (the strip mangled to
    /// `__disturbs` while the invocation still spelled `__touches`), so every shipped derivation
    /// probe hit rc 127, emitted nothing, and collapsed its footprint to a wall: safe, but the
    /// whole survival product silently dead. Driven through the REAL strip rather than a
    /// hand-written fixture, because a hand-written fixture is exactly what hid the bug.
    #[test]
    fn derivation_shipped_def_name_equals_the_invoked_name() {
        let authored = "\
apt_get__disturbs() {
   verb=$1; shift
   case $verb in
   install) apt-manifest \"$1\" ;;
   esac
}";
        let mut i = Interner::default();
        let lifted = dorc_oracle::touches::TouchesSet::lift(&mut i, authored);
        let provider = lifted.value.providers().next().expect("one provider");
        let body = lifted.value.get(provider).expect("the disturbs funcdef");
        let stripped = dorc_oracle::predict::strip_touches(authored, body, &i);
        let def_name = stripped
            .split_once('(')
            .expect("the stripped funcdef opens with `<name>(`")
            .0
            .to_owned();

        let book_word = i.intern("apt-get");
        assert_eq!(
            touches_fn_name(&i, book_word),
            def_name,
            "the derivation invocation must name the funcdef the strip actually ships"
        );

        let plan = DerivationPlan {
            derivations: vec![ProbeDerivation {
                site: LeafId(0),
                node: CfgNodeId(0),
                provider: book_word,
                argv: vec![i.intern("install")],
                sh: stripped,
                call: "apt-manifest".to_owned(),
            }],
        };
        let sh = plan.render_sh(&records::Nonce::spike_default(), &i);
        assert!(
            sh.contains(&format!("{def_name}() {{")) || sh.contains(&format!("{def_name}() \n")),
            "the def ships under the mangled name: {sh}"
        );
        assert!(
            sh.contains(&format!("_d=$({def_name} 'install');")),
            "the invocation calls that exact name: {sh}"
        );
    }

    /// The GUARD lane's half of the same law (`289:rul-touches-mismatch-own-lane`): the funcname
    /// `strip_verdict` mangles the shipped preamble to must equal the one the guard invokes.
    /// Both sides now read `VERDICT_SUFFIX`; this pins that they still meet after the strip, and
    /// covers both invocation spellings — `verdict_fn_name` (keyed on the BOOK word) and
    /// `build_vouches` (keyed on the lifted funcdef provider) must land on one string.
    #[test]
    fn verdict_shipped_def_name_equals_the_invoked_name() {
        let authored = "\
apt_get__is_converged() {
   case $1 in
   install) dpkg-query -W \"$2\" >/dev/null 2>&1 ;;
   *) return 2 ;;
   esac
}";
        let mut i = Interner::default();
        let lifted = dorc_oracle::verdict::VerdictSet::lift(&mut i, authored);
        let provider = lifted.value.providers().next().expect("one provider");
        let body = lifted.value.get(provider).expect("the verdict funcdef");
        let def_name = dorc_oracle::predict::strip_verdict(authored, body, &i)
            .split_once('(')
            .expect("the stripped funcdef opens with `<name>(`")
            .0
            .to_owned();

        let book_word = i.intern("apt-get");
        assert_eq!(
            verdict_fn_name(&i, book_word),
            def_name,
            "the guard invocation must name the funcdef the strip actually ships"
        );
        assert_eq!(
            format!(
                "{}{VERDICT_SUFFIX}",
                dorc_oracle::to_funcname_segment(i.resolve(provider))
            ),
            def_name,
            "build_vouches's own spelling must land on the same name"
        );
    }

    /// The reach lane's half of the same law (`289:rul-touches-mismatch-own-lane`): a dynamic
    /// `reaches()` arm ships as an engine-synthesized per-arm wrapper, so its def and invocation
    /// must both be the ONE `arm_fn` string the cli built — nothing may re-derive either side.
    #[test]
    fn reach_arm_shipped_def_name_equals_the_invoked_name() {
        let arm_fn = format!(
            "sm_dorc_Package{}_0",
            dorc_oracle::reaches::DISTURBANCE_REACHES_ONLY_SUFFIX
        );
        let plan = ReachPlan {
            probes: vec![ReachProbe {
                coord_label: "sm.dorc.Package:nginx".to_owned(),
                kind_label: "sm.dorc.Package".to_owned(),
                arm_fn: arm_fn.clone(),
                arm_index: 0,
                entity_text: "nginx".to_owned(),
                arm_sh: format!("{arm_fn}() {{ dpkg -L \"$1\" ; }}"),
            }],
        };
        let sh = plan.render_sh(&records::Nonce::spike_default());
        assert!(
            sh.contains(&format!("{arm_fn}() {{ dpkg -L")),
            "the per-arm wrapper ships under arm_fn: {sh}"
        );
        assert!(
            sh.contains(&format!("_r=$({arm_fn} 'nginx')")),
            "the invocation calls that exact name: {sh}"
        );
        // The arm's status is captured off the invocation itself, never off the record pipe — a
        // pipeline's status is its RHS's, which is what made the body's death unobservable
        // (`28P:fnd-the-reach-lane-has-no-completeness-gate-at-all`).
        assert!(
            sh.contains("); _rr=$?") && sh.contains("n=%s body-rc=%s"),
            "the arm closes with its count AND its own termination status: {sh}"
        );
    }

    #[test]
    fn two_same_command_sites_stay_distinct_sites() {
        // `inv-site-keyed-results` (the core of the re-key): two same-command sites are
        // NEVER collapsed (spike-2's per-fact dedup is gone). Two IDENTICAL `apt-get
        // install -y nginx` lines on the SAME cell: the SECOND sees the first establish
        // its cell upstream ⇒ EstablishProbeWritten ⇒ unresolvable (correct — its resting
        // probe is stale). So site 0 is resolvable (a check) and site 1 is recorded
        // unresolvable — distinct ids, no collapse. (A finding the test premise first
        // got wrong: same-cell re-establish is Written, NOT a second ambient site;
        // strain-D1-samecell.)
        let (probe, i) =
            probe_for_src("apt-get install -y nginx\napt-get install -y nginx\n", true);
        assert_eq!(probe.checks.len(), 1, "site 0 resolvable (ambient)");
        assert_eq!(probe.checks[0].site, LeafId(0));
        assert_eq!(
            probe.unresolvable,
            vec![LeafId(1)],
            "site 1 is a DISTINCT site, recorded unresolvable (same-cell Written), not collapsed"
        );
        let rendered = probe.render_sh(&records::Framing::spike(String::new()), &i);
        assert!(
            rendered.contains("printf 'dorc site 0 effect="),
            "site 0 record:\n{rendered}"
        );
        assert!(
            rendered.contains("# site:1 unresolvable-no-probe"),
            "site 1 comment:\n{rendered}"
        );
    }

    #[test]
    fn two_distinct_cell_sites_both_resolvable_distinct_ids() {
        // The clean half of `inv-site-keyed-results`: two installs of DIFFERENT packages
        // (distinct cells, neither poisons the other) are two resolvable sites with
        // distinct ids and distinct facts — two invocations, two records.
        let (probe, i) = probe_for_src("apt-get install -y nginx\napt-get install -y curl\n", true);
        assert_eq!(probe.checks.len(), 2, "two distinct-cell sites");
        assert_eq!(probe.checks[0].site, LeafId(0));
        assert_eq!(probe.checks[1].site, LeafId(1));
        assert_ne!(
            probe.checks[0].fact, probe.checks[1].fact,
            "distinct cells (nginx vs curl)"
        );
        let rendered = probe.render_sh(&records::Framing::spike(String::new()), &i);
        assert!(
            rendered.contains("printf 'dorc site 0 effect="),
            "site 0 record:\n{rendered}"
        );
        assert!(
            rendered.contains("printf 'dorc site 1 effect="),
            "site 1 record:\n{rendered}"
        );
    }

    #[test]
    fn probe_render_self_reports_and_binds_operand() {
        // The WIRE (R3 / 23D §1 — the check IS the oracle): the rendered probe is
        // SELF-REPORTING — each resolvable site invokes the provider's stripped
        // `<provider>__predict` funcdef with the site's FULL argv, F-QUOTE'd per word, and
        // emits `site <id> effect=… rc=…` on stdout. The nullary verb (`apt-get update`)
        // is invoked with just the verb as its argv (`apt_get__predict 'update'`) — the
        // check's own argparse resolves the Singleton. The three sites share one provider
        // (`apt-get`), so its funcdef is emitted once and re-used (same body ⇒ no re-emit).
        let (probe, i) = probe_for_src(
            "apt-get install -y nginx\napt-get install -y curl\napt-get update\n",
            true,
        );
        let rendered = probe.render_sh(&records::Framing::spike(String::new()), &i);

        // Full argv bound + single-quoted per word (F-QUOTE): the check argparses the entity.
        assert!(
            rendered.contains("apt_get__predict 'install' '-y' 'nginx'"),
            "nginx site's argv bound + quoted:\n{rendered}"
        );
        assert!(
            rendered.contains("apt_get__predict 'install' '-y' 'curl'"),
            "curl site's argv bound + quoted:\n{rendered}"
        );
        // The provider funcdef is emitted exactly ONCE — three same-provider sites, one body.
        assert_eq!(
            rendered.matches("apt_get__predict() {").count(),
            1,
            "apt-get's check funcdef emitted once, invoked per site:\n{rendered}"
        );
        // The nullary verb (`apt-get update`) is invoked with just the verb (no operand
        // exists) — the check argparse resolves the Singleton from `$verb`.
        assert!(
            rendered.contains("apt_get__predict 'update'; _rc=$?;"),
            "a Singleton (nullary) site invokes the check with just its verb:\n{rendered}"
        );
        // Self-reporting: a site-keyed record printf per resolvable site (3 of them).
        assert_eq!(
            rendered.matches("printf 'dorc site ").count(),
            3,
            "one record per resolvable site:\n{rendered}"
        );
        // The three-outcome derivation is present (holds/absent/cant-tell from rc).
        assert!(
            rendered.contains("_e=holds")
                && rendered.contains("_e=absent")
                && rendered.contains("_e=cant-tell"),
            "the wrapper maps rc to the three-outcome word:\n{rendered}"
        );
    }

    #[test]
    fn probe_render_quotes_operand_with_space_or_metachar() {
        // F-QUOTE (`notes/198`, `inv-kfail` both directions): the book operand is
        // interned POST-parse (quotes stripped, embedded chars preserved). A spaced or
        // metachar operand must render as exactly ONE inert single-quoted arg, never
        // splitting (TWO args ⇒ wrong entity, kFAIL-perform) or re-parsing (a `;` ⇒ a
        // SECOND command ⇒ kFAIL-withhold probe-mutation). Driven through the real
        // value-flow: a command-prefix assignment `PKG='my pkg'` flows the spaced
        // operand to the install site. (The behavioral `dash -n` + binding properties
        // are the e2e `probe-operand-quoting` case's job — "IN sh, FROM sh".)
        // Spaced operand via a flowed assignment. R3: the whole argv is F-quoted per word,
        // so the spaced operand renders as exactly one arg (`'my pkg'`).
        let (probe, i) = probe_for_src("PKG='my pkg'\napt-get install -y \"$PKG\"\n", true);
        let rendered = probe.render_sh(&records::Framing::spike(String::new()), &i);
        assert!(
            rendered.contains("apt_get__predict 'install' '-y' 'my pkg'"),
            "spaced operand single-quoted to one arg:\n{rendered}"
        );

        // Metachar operand: the `;` is INSIDE the quotes, so it cannot split.
        let (probe, i) = probe_for_src(
            "PKG='x; touch /tmp/PWNED'\napt-get install -y \"$PKG\"\n",
            true,
        );
        let rendered = probe.render_sh(&records::Framing::spike(String::new()), &i);
        assert!(
            rendered.contains("apt_get__predict 'install' '-y' 'x; touch /tmp/PWNED'"),
            "metachar operand single-quoted ⇒ the `;` cannot split:\n{rendered}"
        );
        // No UNQUOTED metachar invocation leaked (the `;` only ever appears quoted).
        assert!(
            !rendered.contains("'-y' x; touch"),
            "no unquoted metachar invocation:\n{rendered}"
        );
    }

    #[test]
    fn probe_site_id_equals_plan_leaf_id() {
        // `inv-site-keyed-results` (the load-bearing equivalence): the probe's site-id
        // for a source command == the apply plan's leaf-id for the SAME command. A
        // record `site N …` therefore keys back to plan leaf N. Drive both stages off
        // one classify result and cross-check the install's id. (`apt-get update` is a
        // modeled DISTINCT cell, so it does not poison the install's ambient-ness — the
        // install stays resolvable; it is the second source command ⇒ site/leaf 1.)
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let src = "apt-get update\napt-get install -y nginx\n";
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;

        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |_, provider, argv| ship_corpus(&checks, &i, provider, argv),
            |_, _, _, _| None,
            |_, _| false,
        );
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &invalidators,
            &vouch_all(&classes),
            |_f| Observable::verdict_only(Verdict::Diverged),
            &mut dorc_core::ProvArena::new(),
        );

        let install_site = probe
            .checks
            .iter()
            .find(|c| matches!(c.fact.entity, EntityRef::Operand(_)))
            .expect("the install is a resolvable site")
            .site;
        let install_leaf = plan
            .steps
            .iter()
            .find(|s| s.sh.contains("apt-get install"))
            .expect("the install is a plan leaf")
            .leaf;
        assert_eq!(
            install_site, install_leaf,
            "probe site-id and plan leaf-id are ONE id space"
        );
    }

    #[test]
    fn license_minted_for_ambient_must_converged() {
        // The one path that authorises a skip: classify said ambient, the oracle
        // declared Must, and the probe found it already holds.
        let f = nginx_fact();
        let Some(lic) = ReplaceLicense::prove_replaceable(
            f,
            Grade::Must,
            PhasedVerdict::<Probe>::new(Verdict::Converged),
            quiet(),
            Predicted::Value(Rc(0)),
            Some(test_vouch()),
        ) else {
            panic!("ambient + must + converged must license a skip");
        };
        assert_eq!(lic.fact(), f);
        assert!(lic.derivation().ambient);
        assert_eq!(lic.derivation().verdict, Verdict::Converged);
    }

    #[test]
    fn no_license_for_ambient_without_vouch() {
        // The elide-weld (24D §3 / rul24-vouch-is-verdict-authoring): a converged, ambient, Must
        // fact with NO reached vouch does NOT elide — it runs (kFAIL-perform). This is the HEAD
        // vouchless-elide gap, closed: the ONLY difference from
        // `license_minted_for_ambient_must_converged` is `None` vs `Some(test_vouch())`, so a
        // measurement alone can never license a mutation-skip (proviso-read-erasure). A
        // `ByObservation`/`BySilence` cannot even be PASSED here (the tier is the compile-check);
        // `None` is the run-it direction the same signature forces.
        let f = nginx_fact();
        assert!(
            ReplaceLicense::prove_replaceable(
                f,
                Grade::Must,
                PhasedVerdict::<Probe>::new(Verdict::Converged),
                quiet(),
                Predicted::Value(Rc(0)),
                None,
            )
            .is_none(),
            "a converged ambient Must fact WITHOUT a vouch must not elide (no vouch ⇒ run)"
        );
    }

    /// `28M` §8's monologue, now read off the value instead of argued from three mechanisms
    /// agreeing. An establish-elide speaks for the author whose vouch it consumed — and for nobody
    /// else — while a Query substitution speaks for no author at all, because its reproduced value
    /// is the probe's own measurement of the very command being substituted.
    ///
    /// Non-vacuous in the direction that matters: the custody is read from the CONSUMED vouch, so
    /// this fails the moment a mint stamps a license with a custody its vouch did not supply, which
    /// is precisely the shape a measured-value widening would take.
    #[test]
    fn an_establish_elide_speaks_for_its_vouching_author_and_a_query_for_none() {
        use dorc_core::{ByVouch, DefinitionCustody, LicenseCustody, Rung, SourceFileId};
        let f = nginx_fact();
        let vouch_from = |file: u32| {
            ByVouch::vouched(
                VerdictVouch::new(
                    "apt_get__is_converged".to_string(),
                    "apt_get__is_converged() { :; }".to_string(),
                    "apt_get__is_converged install -y nginx".to_string(),
                    "package".to_string(),
                    vec!["dpkg-query".to_string()],
                    DefinitionCustody::of_defining_file(SourceFileId(file)),
                ),
                Rung::Both,
            )
        };
        for file in [0_u32, 3] {
            let lic = ReplaceLicense::prove_replaceable(
                f,
                Grade::Must,
                PhasedVerdict::<Probe>::new(Verdict::Converged),
                quiet(),
                Predicted::Top,
                Some(vouch_from(file)),
            )
            .expect("a converged ambient Must fact WITH a vouch elides");
            assert_eq!(
                lic.custody(),
                LicenseCustody::Vouched(DefinitionCustody::of_defining_file(SourceFileId(file))),
                "the elision must speak for the author whose vouch licensed it, and follow it when \
                 that author changes"
            );
        }
        let query = ReplaceLicense::prove_query_replaceable(
            f,
            true,
            Verdict::Converged,
            &quiet(),
            Predicted::Value(Rc(0)),
        )
        .expect("a valid known-rc Query substitutes");
        assert_eq!(
            query.custody(),
            LicenseCustody::MeasuredSelf,
            "a read-substitution rests on no authored vouch — it reproduces the substituted \
             command's OWN measurement, so there is no second speaker to name"
        );
    }

    /// `28M` §8's hardening, the half that is about VALUES rather than authors: a split family —
    /// one author's `predict` resolving the site, another's `is_converged` vouching it — must not
    /// let the elision reproduce anything the predict measured. The firewall is upstream (an
    /// Establish site's status is withheld to ⊤ at intake, `results.rs`), so the mint sees
    /// `Predicted::Top` and the stand-in is `True`. Pinned HERE because the intake firewall and the
    /// mint are separate crates and neither one alone states the property.
    ///
    /// Read the two assertions together: the license exists, AND its stand-in carries no measured
    /// value. A split family therefore cannot smuggle author A's measurement into author B's
    /// sentence — the elision reproduces the vouch and nothing else.
    #[test]
    fn a_split_family_establish_elide_reproduces_nothing_predict_derived() {
        let f = nginx_fact();
        let lic = ReplaceLicense::prove_replaceable(
            f,
            Grade::Must,
            PhasedVerdict::<Probe>::new(Verdict::Converged),
            quiet(),
            // What an Establish site always arrives with: the rc firewall withheld it.
            Predicted::Top,
            Some(test_vouch()),
        )
        .expect("the establish elide mints");
        assert!(
            matches!(lic.custody(), dorc_core::LicenseCustody::Vouched(_)),
            "the licence is the verdict author's alone"
        );
        let stand_in = match Predicted::<Rc>::Top {
            Predicted::Value(rc) => StandIn::from_rc(rc),
            Predicted::Top => StandIn::True,
        };
        assert_eq!(
            stand_in,
            StandIn::True,
            "an establish-elide's stand-in reproduces NO measured value: `true` is the vouch \
             acting-as-succeeded, never a predict-derived rc (`guards-mint-no-values`' elide twin)"
        );
    }

    /// `28M` §8 `vouch-covers-the-stand-in-rc-0`. The rc-0 an establish-elide leaves behind is not
    /// a claim that the command exits 0 — it is the VOUCH's act-as-succeeded, which is why a
    /// consumer that would notice the difference blocks the mint instead of relaxing it.
    ///
    /// So the pin is the pair: rc-0 rides through for a consumer that cannot tell (`quiet`), and
    /// the SAME inputs with a ⊤-status branch consumer refuse. If the second half ever passed, the
    /// stand-in's rc-0 would be a fabricated success suppressing a `|| fallback` — the round-19
    /// under-execute, and the exact reason the vouch's coverage stops at the consumers it can speak
    /// for.
    #[test]
    fn the_vouch_covers_the_stand_in_rc_zero_only_where_no_consumer_can_tell() {
        let f = nginx_fact();
        let mint = |consumed| {
            ReplaceLicense::prove_replaceable(
                f,
                Grade::Must,
                PhasedVerdict::<Probe>::new(Verdict::Converged),
                consumed,
                Predicted::Top,
                Some(test_vouch()),
            )
        };
        assert!(
            mint(quiet()).is_some(),
            "with nothing consuming the status, the stand-in's rc 0 is inside what the vouch \
             covers — 're-running this is noise I accept'"
        );
        assert!(
            mint(May(Powerset::singleton(Channel::StatusRelaxable))).is_none(),
            "a branch that READS the status is outside the vouch's coverage at ⊤: the stand-in's \
             rc 0 would decide somebody else's line, so the site runs"
        );
    }

    #[test]
    fn no_license_when_unvouched_output_consumed() {
        // 16F/16G: a consumed stdout OR stderr makes the `true`-stub's empty default
        // unsound ⇒ no license (run), even with ambient + Must + Converged. Both
        // unvouched output observables block — the `Stderr` branch was formerly only
        // exercised end-to-end, pinned here so the matrix can drop its stderr cell.
        // A *declared* rc does NOT vouch output content, so passing `Predicted::Value(Rc(0))` must
        // STILL block (`19D`: the rc-relaxation is `Status`-only, never stdout/stderr).
        let f = nginx_fact();
        for obs in [Channel::Stdout, Channel::Stderr] {
            let consumed = May(Powerset::singleton(obs));
            assert!(
                ReplaceLicense::prove_replaceable(
                    f,
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(Verdict::Converged),
                    consumed,
                    Predicted::Value(Rc(0)),
                    Some(test_vouch()),
                )
                .is_none(),
                "a consumed {obs:?} must forbid the stub even with a declared rc"
            );
        }
    }

    #[test]
    fn relaxable_status_blocks_only_when_rc_undeclared() {
        // `19D` (the keystone of the kFAIL-perform fix): a `&&`/`||` left operand's
        // `StatusRelaxable` blocks the license iff the rc is UNDECLARED — then the stand-in
        // would default to `true`/rc-0, a fabricated success suppressing a `|| fallback`
        // (the round-19 under-execute). A *declared* rc relaxes it (the value-preserving
        // stand-in reproduces the exact status, preserving the branch).
        let f = nginx_fact();
        let consumed = || May(Powerset::singleton(Channel::StatusRelaxable));
        // Undeclared rc ⇒ BLOCK (the safe run-it floor).
        assert!(
            ReplaceLicense::prove_replaceable(
                f,
                Grade::Must,
                PhasedVerdict::<Probe>::new(Verdict::Converged),
                consumed(),
                Predicted::Top,
                Some(test_vouch()),
            )
            .is_none(),
            "`&&`/`||`-consumed status + undeclared rc must block (kFAIL-perform floor)"
        );
        // Declared rc (even a non-conforming 9) ⇒ RELAX (the stand-in is exact).
        for rc in [Rc(0), Rc(9)] {
            assert!(
                ReplaceLicense::prove_replaceable(
                    f,
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(Verdict::Converged),
                    consumed(),
                    Predicted::Value(rc),
                    Some(test_vouch()),
                )
                .is_some(),
                "`&&`/`||`-consumed status + declared rc {rc:?} licenses (value-preserving)"
            );
        }
    }

    #[test]
    fn iterated_status_blocks_unconditionally() {
        // arch-1 (note 214; successor to the retired `render_floor_status_blocks_unconditionally`):
        // a `while`/`until` condition's `StatusIterated` blocks the license EVEN with a
        // declared rc — the condition's per-iteration rc-sequence cannot be reproduced by one
        // predicted value, and a constant-substituted loop condition is an infinite/zero-
        // iteration disaster. Contrast `relaxable_status_blocks_only_when_rc_undeclared` (a
        // single-shot guard a known rc relaxes) — the if/elif guard moved to THAT channel.
        let f = nginx_fact();
        for rc in [
            Predicted::Top,
            Predicted::Value(Rc(0)),
            Predicted::Value(Rc(9)),
        ] {
            assert!(
                ReplaceLicense::prove_replaceable(
                    f,
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(Verdict::Converged),
                    May(Powerset::singleton(Channel::StatusIterated)),
                    rc,
                    Some(test_vouch()),
                )
                .is_none(),
                "a loop condition's StatusIterated blocks unconditionally (per-iteration sequence), rc={rc:?}"
            );
        }
    }

    #[test]
    fn no_license_when_verdict_not_converged() {
        // Diverged ⇒ run; Unknown ⇒ run (the Bias fold) — neither licenses.
        let f = nginx_fact();
        for v in [Verdict::Diverged, Verdict::Unknown] {
            assert!(
                ReplaceLicense::prove_replaceable(
                    f,
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(v),
                    quiet(),
                    Predicted::Value(Rc(0)),
                    Some(test_vouch()),
                )
                .is_none(),
                "verdict {v:?} must NOT license a skip"
            );
        }
    }

    #[test]
    fn no_license_for_may_grade() {
        // inv-must-may: a mined/distributional May-grade fact never authorises a skip.
        let f = nginx_fact();
        assert!(
            ReplaceLicense::prove_replaceable(
                f,
                Grade::May,
                PhasedVerdict::<Probe>::new(Verdict::Converged),
                quiet(),
                Predicted::Value(Rc(0)),
                Some(test_vouch()),
            )
            .is_none()
        );
    }

    #[test]
    fn unknown_folds_to_run_in_both_phases() {
        // The kFAIL fold: Unknown is never Replaceable, in either phase.
        assert_eq!(
            PhasedVerdict::<Probe>::new(Verdict::Unknown).resolve(),
            Resolved::Run
        );
        assert_eq!(
            PhasedVerdict::<Apply>::new(Verdict::Unknown).resolve(),
            Resolved::Run
        );
        // Sanity on the definite verdicts.
        assert_eq!(
            PhasedVerdict::<Probe>::new(Verdict::Converged).resolve(),
            Resolved::Replaceable
        );
        assert_eq!(
            PhasedVerdict::<Apply>::new(Verdict::Diverged).resolve(),
            Resolved::Run
        );
    }

    // --- end-to-end: the whole pipeline (parse → cfg → classify → plan) ---

    /// A package kind-index modeling `apt-get install → package@installed` AND
    /// `apt-get update → package-index@fresh` (the spike-2 re-key, `notes/193` §1).
    /// `update` now lands on a *distinct cell* from `install`, so it no longer
    /// poisons the install below it — the poison-wall fix. (Pre-key, `update` was
    /// left un-modeled ⇒ Opaque ⇒ `Reach::Top` ⇒ it poisoned everything downstream.)
    fn package_index(i: &mut Interner) -> KindIndex {
        let package = KindId(i.intern("package"));
        let package_index = KindId(i.intern("package-index"));
        let installed = SelectorId(i.intern("installed"));
        let fresh = SelectorId(i.intern("fresh"));
        let apt = ProviderId(i.intern("apt_get"));
        let install = i.intern("install");
        let update = i.intern("update");
        let mut idx = KindIndex::default();
        idx.add_effect(0, apt, install, package, installed, ValueClaim::Establish);
        idx.add_effect(0, apt, update, package_index, fresh, ValueClaim::Establish);
        idx
    }

    /// Run the pipeline on `src`, answering `package:nginx@installed` with
    /// `nginx_verdict` and every other fact `Unknown`.
    fn plan_for(src: &str, nginx_verdict: Verdict) -> (Plan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let target = FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: SelectorId(i.intern("installed")),
            context: dorc_core::Context::HostDefault,
        };
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        // fork-mutator-rc (202 §5 / 206 §3): a MUTATOR's status has no sanctioned source —
        // only its Effect channel (convergence) arrives from the probe, the rc is ⊤. So
        // `verdict_only` everywhere, never a fabricated `Rc(0)`.
        let observe = |f: FactKey| {
            if f == target {
                Observable::verdict_only(nginx_verdict)
            } else {
                Observable::verdict_only(Verdict::Unknown)
            }
        };
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &invalidators,
            &vouch_all(&classes),
            observe,
            &mut dorc_core::ProvArena::new(),
        );
        (plan, i)
    }

    /// Run the pipeline on `src`, answering each `package:<entity>@installed` cell with
    /// the verdict `verdict_of(entity)` returns (every non-package fact ⇒ Unknown). For the
    /// task-L2 member tests that need DIFFERENT verdicts per member (e.g. nginx converged,
    /// curl diverged). Status stays ⊤ (fork-mutator-rc), as `plan_for`.
    fn plan_for_pkgs(src: &str, verdict_of: impl Fn(&str) -> Verdict) -> (Plan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        // Resolve each package entity's verdict by its interned operand text. The closure
        // captures the entity strings it cares about; an unknown entity ⇒ Unknown.
        let observe = |f: FactKey| {
            if f.kind == package
                && f.selector == installed
                && let EntityRef::Operand(tok) = f.entity
            {
                return Observable::verdict_only(verdict_of(i.resolve(tok.0)));
            }
            Observable::verdict_only(Verdict::Unknown)
        };
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &invalidators,
            &vouch_all(&classes),
            observe,
            &mut dorc_core::ProvArena::new(),
        );
        (plan, i)
    }

    fn plan_with_duplicate_aggregate_vouch(src: &str) -> Plan {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        let mut vouches = build_vouches(
            &[CORPUS_VERDICT_SRC],
            &[],
            &dorc_oracle::closure::HelperIndex::default(),
            &classes,
            &value,
            &mut i,
            dorc_analysis::funcenv::LiveDefinitions::unsolved(),
        )
        .0
        .value;
        let supplied = classes.iter().find_map(|(node, class)| match class {
            SkipClass::EstablishMembers { members, .. } => {
                members.first().map(|fact| (*node, *fact))
            }
            SkipClass::InlineCall { sites } => sites.iter().find_map(|site| match site.class {
                SkipClass::EstablishProbeAmbient(fact) | SkipClass::EstablishProbeWritten(fact) => {
                    Some((site.node, fact))
                }
                _ => None,
            }),
            _ => None,
        });
        let (site, fact) = supplied.expect("the fixture has an aggregate establish");
        let duplicate = vouches
            .get(site, fact)
            .expect("build_vouches supplied the reached identity")
            .clone();
        vouches.insert(site, fact, duplicate);
        build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &invalidators,
            &vouches,
            |_| Observable::verdict_only(Verdict::Converged),
            &mut dorc_core::ProvArena::new(),
        )
    }

    /// Run the pipeline answering EVERY fact `Converged` — for the plan-level keystone tests
    /// where the upstream modeled mutator (`apt-get update`) must itself ELIDE (cast no shadow,
    /// silence=wall) for a downstream converged establish to elide *past* it. `plan_for` gives
    /// every non-nginx cell `Unknown` ⇒ `update` would RUN ⇒ wall the install; converging
    /// `update`'s own cell is what keeps the keystone (distinct-cell, no poison) demonstrable at
    /// the plan tier under the honest `23Ib-fd10` law.
    fn plan_all_converged(src: &str) -> (Plan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        let observe = |_f: FactKey| Observable::verdict_only(Verdict::Converged);
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &invalidators,
            &vouch_all(&classes),
            observe,
            &mut dorc_core::ProvArena::new(),
        );
        (plan, i)
    }

    fn find<'a>(plan: &'a Plan, needle: &str) -> &'a Step {
        match plan.steps.iter().find(|s| s.sh.contains(needle)) {
            Some(s) => s,
            None => panic!("no leaf containing {needle:?} in {:?}", plan.steps),
        }
    }

    #[test]
    fn converged_ambient_install_is_replaced_rest_runs() {
        // A lone install is ambient; a Converged probe licenses the skip. The
        // following un-oracled command runs (Opaque ⇒ MustRun).
        let (plan, interner) = plan_for(
            "apt-get install -y nginx\nsystemctl reload nginx\n",
            Verdict::Converged,
        );
        assert!(
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, _)
            ),
            "converged ambient install ⇒ skip"
        );
        assert!(
            matches!(
                find(&plan, "systemctl reload").disposition,
                Disposition::Run
            ),
            "opaque reload ⇒ run"
        );

        let sh = plan.render_sh(&interner);
        assert!(
            sh.contains("# replace["),
            "rendered plan comments the replaced leaf:\n{sh}"
        );
        assert!(
            sh.contains("package:nginx"),
            "replace provenance names the fact:\n{sh}"
        );
        assert!(
            sh.contains("systemctl reload nginx"),
            "run leaf rendered verbatim:\n{sh}"
        );
    }

    #[test]
    fn diverged_install_runs() {
        // The host says nginx is absent ⇒ the install must run (no license).
        let (plan, _) = plan_for(
            "apt-get install -y nginx\nsystemctl reload nginx\n",
            Verdict::Diverged,
        );
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "diverged ⇒ run"
        );
    }

    #[test]
    fn fixture_install_on_realistic_book_still_runs_residual_poison() {
        // THE poison-wall finding (`notes/193` strain-5, K2 — a DATUM, not a fail).
        // The keystone kills `apt-get update`'s poison SPECIFICALLY (proven at classify
        // level by `effect::tests::poison_wall_dies_modeled_update_does_not_poison_
        // install`, and at plan level by the `…_only_neighbour` test below). But on the
        // FULL realistic `pi-webhost.book.sh` the install STILL runs (and so does
        // `update` itself) — for a DIFFERENT, correct reason: TWO upstream un-oracled
        // neighbours each independently poison to Reach::Top (verified by isolating the
        // fragments, `notes/193` strain-5):
        //   1. `case "$(hostname)" in …` — the `$(hostname)` command-substitution is an
        //      un-oracled Command ⇒ Opaque ⇒ Top;
        //   2. `if ! command -v nginx …` — the guard's `command -v nginx` is likewise
        //      un-oracled Opaque ⇒ Top (the bitter irony: the admin wrote this guard AS
        //      an idempotency check, and it poisons the very block it guards).
        // Modeling `update` was NECESSARY but not SUFFICIENT to elide on this scrappy
        // book — a real measure of how much oracle coverage a realistic book needs to
        // elide *anything* (honest, not a green faked by deleting the neighbours).
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/pi-webhost.book.sh"
        ));
        // The residual poison costs the elision. The book's `set -e` also consumes the install's
        // status, so a guard would replace that status with the check's live answer; it must run.
        let (plan, _) = plan_for(fixture, Verdict::Converged);
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "install still does not elide: two upstream un-oracled neighbours ($(hostname) in the \
             case scrutinee, and `command -v nginx` in the if-guard) really run — `update` is no \
             longer the poison, but it is not the only one (notes/193 strain-5)"
        );
    }

    #[test]
    fn residual_poison_sources_isolated() {
        // The exclusion-check behind strain-5 (`notes/193`): pin the TWO residual
        // poison sources independently, so the finding survives as a regression and not
        // just a narrated comment. Each upstream un-oracled construct, alone, forces the
        // install to Written; with neither, it is Ambient (the keystone win). Uses
        // `plan_all_converged`: post-`23Ib-fd10` a RUNNING modeled `update` would WALL the
        // install (silence=wall), so to isolate the classify-level POISON gate the upstream
        // `update` must itself ELIDE (converged ⇒ casts no shadow) — then only a real poison,
        // not the wall, can force the install to run.
        let ambient = |src: &str| {
            let (plan, _) = plan_all_converged(src);
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, _)
            )
        };
        // Neither neighbour ⇒ ambient ⇒ elides (the clean keystone case): a converged `update`
        // elides (no shadow), so the converged install elides past it.
        assert!(
            ambient("apt-get update\napt-get install -y nginx\n"),
            "no poison ⇒ elides"
        );
        // `set -e` is a pure builtin (fs-4) — it must NOT POISON (the install stays
        // EstablishProbeAmbient at the EFFECT layer). But under C-3 (205 §2 / 206 §3),
        // `set -e` CONSUMES the install's status, which for a mutator is ⊤
        // (fork-mutator-rc), so the plan disposition is now Run — NOT elided. The old
        // `ambient(set -e …)` assert masked C-3 by feeding a fabricated rc-0 through
        // `plan_for`; with the faithful ⊤-rc the install RUNS. Pin the EFFECT-layer
        // non-poison (classify EstablishProbeAmbient) directly, separate from the plan-level
        // status block.
        {
            let mut i = Interner::default();
            let idx = package_index(&mut i);
            let src = "set -e\napt-get update\napt-get install -y nginx\n";
            let parsed = dorc_syntax::parse(src);
            let cfg = dorc_analysis::cfg::build(&parsed.value).value;
            let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
            let checks =
                vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
            let classification = dorc_analysis::effect::classify(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &dorc_oracle::verdict::VerdictIndex::default(),
                &mut i,
                &mut dorc_core::ProvArena::new(),
            );
            let classes = classification.value;
            assert!(
                classes
                    .iter()
                    .any(|(_, c)| matches!(c, SkipClass::EstablishProbeAmbient(_))),
                "fs-4: set -e does not poison ⇒ the install stays EstablishProbeAmbient: {classes:?}"
            );
        }
        // …but at the PLAN level the C-3 ⊤-rc status block makes it RUN (206 §3).
        assert!(
            !ambient("set -e\napt-get update\napt-get install -y nginx\n"),
            "C-3 (206 §3): set -e consumes the mutator's ⊤-rc status ⇒ the install RUNS"
        );
        // Each real upstream Opaque neighbour, alone, poisons (no elision).
        assert!(
            !ambient(
                "case \"$(hostname)\" in *) : ;; esac\napt-get update\napt-get install -y nginx\n"
            ),
            "the $(hostname) case-scrutinee substitution poisons the install"
        );
        assert!(
            !ambient("if ! command -v nginx; then apt-get install -y nginx; fi\n"),
            "the `command -v nginx` if-guard poisons the install it guards"
        );
    }

    #[test]
    fn fixture_install_elides_when_update_is_the_only_neighbour() {
        // THE keystone win at the PLAN level (`notes/193` strain-5 / acceptance §7.2), now
        // stated honestly under `23Ib-fd10` (this is the wall's (b)-direction — the
        // first-order escape "just elide yourself"): with `apt-get update` the ONLY upstream
        // neighbour (modeled, distinct cell) AND update ITSELF converged, update ELIDES ⇒ casts
        // no shadow ⇒ the converged install elides past it (`Disposition::Replace`). The poison
        // wall is dead end-to-end (not just at classify). Pre-key this was impossible: `update`
        // Opaque ⇒ Top ⇒ install forced Written ⇒ Run. Post-`fd10` the *running* case is the
        // opposite: a DIVERGED update runs ⇒ WALLS the install (the (a)-direction test
        // `running_modeled_mutator_walls_downstream_converged_establish`). `plan_all_converged`
        // makes update converge so it elides — the honest keystone.
        let (plan, _) = plan_all_converged("apt-get update\napt-get install -y nginx\n");
        assert!(
            matches!(
                find(&plan, "apt-get update").disposition,
                Disposition::Replace(_, _)
            ),
            "the upstream modeled `update` is itself converged ⇒ elides (casts no shadow)"
        );
        assert!(
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, _)
            ),
            "modeled `update` (distinct cell, and here ELIDED) does not wall ⇒ converged install elides"
        );
    }

    #[test]
    fn running_modeled_mutator_walls_downstream_converged_establish() {
        // The (a)-direction of the plan-time wall (silence=wall / `23Ib-fd10` / `23O` §2 —
        // the honest-baseline repair). Two installs of DIFFERENT packages ⇒ DISTINCT cells, so
        // the static ambient gate (same-cell reasoning) leaves BOTH `EstablishProbeAmbient` — no
        // same-cell poison rescues this. curl is DIVERGED ⇒ it RUNS ⇒ it is a modeled mutator
        // that runs, which by the frame problem (233) may touch anything it did not declare. So
        // the downstream CONVERGED nginx install — which the static gate would elide — loses its
        // elision (`inv-kfail`: when unsure, act) and falls to the GUARD rung, which re-decides
        // live after curl has run. No `set -e`, so the demotion is the wall's doing, not errexit
        // consuming the mutator's ⊤ status.
        let (plan, _) = plan_for_pkgs("apt-get install -y curl\napt-get install -y nginx\n", |e| {
            if e == "curl" {
                Verdict::Diverged
            } else {
                Verdict::Converged
            }
        });
        assert!(
            matches!(find(&plan, "install -y curl").disposition, Disposition::Run),
            "the diverged upstream mutator runs (no license)"
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "silence=wall: the converged install loses its elision past the running curl mutator, \n             and re-checks live rather than running bare"
        );
    }

    #[test]
    fn elided_upstream_mutator_casts_no_shadow() {
        // The (b)-direction — the first-order escape "just elide yourself", the value the whole
        // product rides on (`23Ib-fd10`; IMPLEMENTATION.md: "elision casts no poisoned shadow").
        // Same two distinct-cell installs, but BOTH converged: the upstream curl install ELIDES
        // (Replace) ⇒ it never runs ⇒ casts no shadow ⇒ the downstream converged nginx install
        // still ELIDES. An elided mutator is not a wall. (The e2e counterparts:
        // exec-poison-wall-dead / guard23-vouch-inert-pair, both converged-converged.)
        let (plan, _) = plan_for_pkgs(
            "apt-get install -y curl\napt-get install -y nginx\n",
            |_| Verdict::Converged,
        );
        assert!(
            matches!(
                find(&plan, "install -y curl").disposition,
                Disposition::Replace(_, _)
            ),
            "converged upstream install elides (casts no shadow)"
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Replace(_, _)
            ),
            "no wall (upstream elided) ⇒ the downstream converged install still elides"
        );
    }

    /// R3 (24A §3 — the kill gap) test seam: the kill-aware pipeline. Like [`plan_for_pkgs`] but
    /// drives [`classify_with_why_diags`] (for the kill-node set) + [`build_plan_walled`], and
    /// registers `apt-get purge` as an `EstablishInverted` claim ⇒ `CommandEffect::Kills` (the
    /// corpus `*` arm models any verb as a plain establish; a KILL needs the `!` polarity). With
    /// `walled=false` the found kill-set is dropped (empty) — the kill-UNAWARE `build_plan` path,
    /// for the differential that pins the wall is precisely kill-driven (a non-kill `MustRun`
    /// never walls). Status stays ⊤ (fork-mutator-rc), as `plan_for_pkgs`.
    fn kill_plan(
        src: &str,
        verdict_of: impl Fn(&str) -> Verdict,
        walled: bool,
    ) -> (Plan, Interner) {
        let mut i = Interner::default();
        let mut idx = package_index(&mut i);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let apt = ProviderId(i.intern("apt_get"));
        let purge = i.intern("purge");
        idx.add_effect(
            0,
            apt,
            purge,
            package,
            installed,
            ValueClaim::EstablishInverted,
        );
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        let (classified, _why, kills_found, _kill_coords, _fact_backings, _narrative, invalidators) =
            dorc_analysis::effect::classify_with_why_diags(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &dorc_oracle::verdict::VerdictIndex::default(),
                &BTreeMap::new(),
                &dorc_analysis::erase::ErasedSites::none(),
                &mut i,
                &mut arena,
                &mut BTreeMap::new(),
                &mut BTreeMap::new(),
                &mut dorc_analysis::certify::CertifierTrip::default(),
                dorc_analysis::funcenv::LiveDefinitions::unsolved(),
            );
        let classes = classified.value;
        let kills = if walled { kills_found } else { BTreeSet::new() };
        // Kill-UNAWARE means the caller never saw the kill at all: it is absent from the effective
        // world too, which is what makes the pair a real before/after of the closed gap.
        let invalidators = if walled {
            invalidators
        } else {
            BTreeSet::new()
        };
        let observe = |f: FactKey| {
            if f.kind == package
                && f.selector == installed
                && let EntityRef::Operand(tok) = f.entity
            {
                return Observable::verdict_only(verdict_of(i.resolve(tok.0)));
            }
            Observable::verdict_only(Verdict::Unknown)
        };
        let classification = RoundClassification {
            classes: classes.clone(),
            kills,
            invalidators,
            fact_backings: BTreeMap::new(),
        };
        let mut trip = dorc_analysis::certify::CertifierTrip::default();
        let mut spine = build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classification,
            WallPolicy::Honest,
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            &BTreeMap::new(),
            observe,
            &mut arena,
            &mut trip,
            None,
        );
        let plan =
            certifier_trip::project_censusless(&mut spine, &trip, &PlanAuthority::without_intake());
        (plan, i)
    }

    #[test]
    fn running_kill_walls_downstream_converged_establish() {
        // R3 (24A §3 — closing the kill gap fd10 left open). `apt-get purge oldpkg` is a Kill
        // (EstablishInverted ⇒ CommandEffect::Kills ⇒ classifies MustRun ⇒ ALWAYS runs). A
        // running kill mutates the world, so by the frame problem (233) it may touch anything it
        // did not declare — exactly like fd10's running modeled ESTABLISH. So the downstream
        // CONVERGED `apt-get install -y nginx` (a DIFFERENT cell the static ambient gate would
        // elide) is DEMOTED Replace→Run (`inv-kfail`). The kill-node set threaded to
        // build_plan_walled restores the wall the opaque `MustRun` SkipClass hid.
        let (plan, _) = kill_plan(
            "apt-get purge oldpkg\napt-get install -y nginx\n",
            |e| {
                if e == "nginx" {
                    Verdict::Converged
                } else {
                    Verdict::Unknown
                }
            },
            true,
        );
        assert!(
            matches!(find(&plan, "purge oldpkg").disposition, Disposition::Run),
            "the kill always runs (Kills ⇒ MustRun)"
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "silence=wall: the converged install loses its elision past the running kill, and \n             re-checks live rather than running bare"
        );
    }

    #[test]
    fn kill_unaware_plan_does_not_wall_the_gap() {
        // The DIFFERENTIAL that pins the wall is precisely kill-driven (not a blanket `MustRun`
        // wall — pure builtins/opaques are `MustRun` too and must NEVER wall, `exec-pure-builtin`).
        // The SAME book through the kill-UNAWARE `build_plan` (empty kill-set) does NOT wall: the
        // converged nginx install WRONGLY elides. This is the exact under-execution the e2e pin
        // `exec-kill-wall-runs` reds pre-fix; threading `kills` (test above) is what closes it.
        let (plan, _) = kill_plan(
            "apt-get purge oldpkg\napt-get install -y nginx\n",
            |e| {
                if e == "nginx" {
                    Verdict::Converged
                } else {
                    Verdict::Unknown
                }
            },
            false,
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Replace(_, _)
            ),
            "kill-unaware build_plan does NOT wall ⇒ the converged install elides (the gap)"
        );
    }

    // ── Stage 2: the survival tier (the golden hill) ────────────────────────────────────────

    // =======================================================================
    // Effective world reach (`30K`) — the ownership-seat pins
    //
    // One fact decides apply-time freshness: which mutations may ACTUALLY execute. These pin the
    // seats where that fact is produced and consumed, because the whole-product cases can only see
    // the composed answer. Each names the mechanism it observes, and each was checked by MUTATING
    // that mechanism away and confirming the pin reddens (the lane report records the results).
    // =======================================================================

    type FootprintChooser<'a> = &'a dyn Fn(&str) -> Option<String>;

    /// Build a plan over the package corpus with full control of the world (`30K` §4).
    ///
    /// `verdict_of` answers `package:<entity>@installed`; `footprints_of` decides the run's policy
    /// — `None` is honest-walls, `Some(f)` builds the risk-accepted policy from a per-node
    /// footprint chooser, which is what the flag buys and nothing else.
    fn effective_plan(
        src: &str,
        verdict_of: impl Fn(&str) -> Verdict,
        footprints_of: Option<FootprintChooser<'_>>,
    ) -> (Plan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let provider = i.intern("apt-get");
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut arena,
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        // `choose(entity)` answers which ENTITY a wall claims, or `None` for "no trustworthy
        // footprint" — which is the shape that makes an unfootprinted wall total.
        let footprints = footprints_of.map(|choose| {
            let mut tf = TrustedFootprints::new();
            let mut claims: Vec<(CfgNodeId, String)> = Vec::new();
            for (node, class) in &classes {
                let fact = match class {
                    SkipClass::EstablishProbeAmbient(f) | SkipClass::EstablishProbeWritten(f) => *f,
                    _ => continue,
                };
                let EntityRef::Operand(tok) = fact.entity else {
                    continue;
                };
                if let Some(claimed) = choose(i.resolve(tok.0)) {
                    claims.push((*node, claimed));
                }
            }
            for (node, claimed) in claims {
                let entity = EntityRef::Operand(OpaqueToken(i.intern(&claimed)));
                let coord = EntityCoord::new(package, entity);
                if let Some(fp) = Footprint::authored(provider, vec![coord]) {
                    tf.insert(node, fp);
                }
            }
            tf
        });
        let observe = |f: FactKey| {
            if f.kind == package
                && f.selector == installed
                && let EntityRef::Operand(tok) = f.entity
            {
                return Observable::verdict_only(verdict_of(i.resolve(tok.0)));
            }
            Observable::verdict_only(Verdict::Unknown)
        };
        let resolutions = Resolutions::none();
        let dialect = dorc_core::Dialect::empty();
        let policy = match footprints.as_ref() {
            Some(fp) => WallPolicy::RiskAccepted {
                footprints: fp,
                resolutions: &resolutions,
                dialect: &dialect,
            },
            None => WallPolicy::Honest,
        };
        let classification = RoundClassification {
            classes: classes.clone(),
            kills: BTreeSet::new(),
            invalidators,
            fact_backings: BTreeMap::new(),
        };
        let mut trip = dorc_analysis::certify::CertifierTrip::default();
        let mut spine = build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classification,
            policy,
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            &BTreeMap::new(),
            observe,
            &mut arena,
            &mut trip,
            None,
        );
        (
            certifier_trip::project_censusless(&mut spine, &trip, &PlanAuthority::without_intake()),
            i,
        )
    }

    /// A modeled mutator that RUNS walls exactly like an unmodeled one, and the sites below it
    /// reach the GUARD tier rather than running bare (`30K` §0's target).
    ///
    /// This is `fnd-classed-decline-unwalls-guard-tier` at the seat: before effective reach, a
    /// downstream converged site below a modeled running wall classified ambient, minted a
    /// `Replace`, and the wall walk could only turn that into `Run` — so an honest oracle produced
    /// a strictly worse plan than no oracle at all.
    #[test]
    fn a_modeled_running_wall_leaves_the_guard_tier_reachable_below_it() {
        let (plan, _) = effective_plan(
            "apt-get install -y oldpkg\napt-get install -y nginx\n",
            |e| {
                if e == "nginx" {
                    Verdict::Converged
                } else {
                    Verdict::Diverged
                }
            },
            None,
        );
        assert!(
            matches!(
                find(&plan, "install -y oldpkg").disposition,
                Disposition::Run
            ),
            "the diverged wall runs"
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "the converged, vouched site below it re-checks LIVE rather than running bare"
        );
    }

    /// An upstream site that ELIDES casts no wall, and the cascade is real: with both converged,
    /// the second site elides too rather than guarding behind the first (`30K` §4.3).
    #[test]
    fn an_elided_upstream_mutation_removes_its_own_wall() {
        let (plan, _) = effective_plan(
            "apt-get install -y oldpkg\napt-get install -y nginx\n",
            |_| Verdict::Converged,
            None,
        );
        for needle in ["install -y oldpkg", "install -y nginx"] {
            assert!(
                matches!(find(&plan, needle).disposition, Disposition::Replace(..)),
                "{needle}: a mutation nobody will run cannot invalidate anything below it"
            );
        }
    }

    #[test]
    fn a_replaced_inline_call_retires_every_owned_body_wall() {
        let (plan, _) = effective_plan(
            "install_both() { apt-get install -y nginx; apt-get install -y curl; }\n\
             install_both\n\
             apt-get install -y wombat\n",
            |_| Verdict::Converged,
            None,
        );
        assert!(
            matches!(
                find(&plan, "install_both").disposition,
                Disposition::Replace(..)
            ),
            "the all-converged inline aggregate replaces"
        );
        assert!(
            matches!(
                find(&plan, "install -y wombat").disposition,
                Disposition::Replace(..)
            ),
            "the replaced call owns both spliced establishes, so neither may wall the later site"
        );
    }

    /// A GUARD is a possible mutator downstream (`30K` §5.3): its check is read-only, but its
    /// untouched fallback is the authored mutation, so everything below it stays stale. And the
    /// recovery is spelled in ordinary forms only — no wall flags, no conditional tails, no
    /// controller bookkeeping in the reviewed plan (`constraint-plan-surface-stays-readable`).
    #[test]
    fn a_guard_stays_a_wall_for_everything_below_it() {
        let (plan, i) = effective_plan(
            "apt-get install -y oldpkg\napt-get install -y nginx\napt-get install -y curl\n",
            |e| {
                if e == "oldpkg" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
            None,
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "the first site below the running wall guards"
        );
        assert!(
            matches!(
                find(&plan, "install -y curl").disposition,
                Disposition::Guard(_)
            ),
            "and so does the site below THAT guard — a guard's fallback may still mutate"
        );
        let rendered = plan.render_sh(&i);
        for bookkeeping in ["_dorc_wall", "DORC_WALL", "_dorc_tail"] {
            assert!(
                !rendered.contains(bookkeeping),
                "the reviewed plan carries no generated wall state ({bookkeeping})"
            );
        }
    }

    /// The flag buys survival and NOTHING else: with a disjoint footprint the elision is kept past
    /// the running wall; with a colliding one the site falls to the guard tier rather than running
    /// bare, exactly as an unfootprinted wall leaves it (`30K` §5.1).
    #[test]
    fn a_colliding_footprint_demotes_to_the_guard_tier_not_to_a_bare_run() {
        let verdict = |e: &str| {
            if e == "nginx" {
                Verdict::Converged
            } else {
                Verdict::Diverged
            }
        };
        let src = "apt-get install -y oldpkg\napt-get install -y nginx\n";
        // Every wall claims its OWN entity ⇒ disjoint from a different entity's backing.
        let disjoint = |e: &str| Some(e.to_owned());
        let (survived, _) = effective_plan(src, verdict, Some(&disjoint));
        assert!(
            matches!(
                find(&survived, "install -y nginx").disposition,
                Disposition::Replace(..)
            ),
            "a disjoint footprint keeps the elision past the running wall"
        );
        // The wall claims the DOWNSTREAM entity's own cell ⇒ a proven collision.
        let collides = |_: &str| Some("nginx".to_owned());
        let (poisoned, _) = effective_plan(src, verdict, Some(&collides));
        assert!(
            matches!(
                find(&poisoned, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "a colliding footprint costs the elision — and lands on the guard rung, not a bare run"
        );
    }

    #[test]
    fn a_later_inline_establish_collision_keeps_the_call_running() {
        let (plan, _) = effective_plan(
            "install_both() { apt-get install -y nginx; apt-get install -y curl; }\n\
             apt-get install -y oldpkg\n\
             install_both\n",
            |entity| {
                if entity == "oldpkg" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
            Some(&|entity: &str| (entity == "oldpkg").then(|| "curl".to_owned())),
        );
        assert!(
            matches!(find(&plan, "install_both").disposition, Disposition::Run),
            "the exact upstream footprint misses representative nginx but hits later body fact \
             curl; replacing the call would under-execute"
        );
    }

    #[test]
    fn every_disjoint_inline_establish_survives_as_one_replacement() {
        let (plan, _) = effective_plan(
            "install_both() { apt-get install -y nginx; apt-get install -y curl; }\n\
             apt-get install -y oldpkg\n\
             install_both\n",
            |entity| {
                if entity == "oldpkg" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
            Some(&|entity: &str| (entity == "oldpkg").then(|| "oldpkg".to_owned())),
        );
        let aggregate = find(&plan, "install_both");
        assert!(
            matches!(aggregate.disposition, Disposition::Replace(..)),
            "the external footprint is disjoint from both body establishes, so the atomic call \
             replacement may survive"
        );
    }

    #[test]
    fn inline_survival_attributes_every_erased_establish_and_crossing() {
        let (plan, i) = effective_plan(
            "install_both() { apt-get install -y nginx; apt-get install -y curl; }\n\
             apt-get install -y oldpkg\n\
             install_both\n",
            |entity| {
                if entity == "oldpkg" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
            Some(&|entity: &str| (entity == "oldpkg").then(|| "oldpkg".to_owned())),
        );
        let Disposition::Replace(license, _) = &find(&plan, "install_both").disposition else {
            panic!("the universally spared aggregate must replace")
        };
        let Some(SurvivalAttribution::Aggregate(witness)) = &license.derivation().survival else {
            panic!("aggregate replacement must carry aggregate survival attribution")
        };
        let members: Vec<_> = witness.members().collect();
        assert_eq!(
            members.len(),
            2,
            "both erased establishes remain attributed"
        );
        assert_eq!(
            members
                .iter()
                .map(|member| i.resolve(match member.fact().entity {
                    EntityRef::Operand(token) => token.0,
                    EntityRef::Singleton => panic!("package facts are operand-keyed"),
                }))
                .collect::<Vec<_>>(),
            ["nginx", "curl"],
            "identity order is the body's establish order"
        );
        assert!(
            members
                .iter()
                .all(|member| member.survival().crossings().len() == 1),
            "each member independently records the load-bearing external crossing"
        );
    }

    #[test]
    fn wall_free_inline_aggregate_is_disposition_and_byte_identical() {
        let src = "install_both() { apt-get install -y nginx; apt-get install -y curl; }\n\
                   install_both\n";
        let (honest, honest_i) = effective_plan(src, |_| Verdict::Converged, None);
        let (risk, risk_i) = effective_plan(
            src,
            |_| Verdict::Converged,
            Some(&|entity: &str| Some(entity.to_owned())),
        );
        assert!(matches!(
            find(&honest, "install_both").disposition,
            Disposition::Replace(..)
        ));
        assert!(matches!(
            find(&risk, "install_both").disposition,
            Disposition::Replace(..)
        ));
        assert_eq!(honest.render_sh(&honest_i), risk.render_sh(&risk_i));
    }

    #[test]
    fn external_wall_stays_blocked_by_members_self_reach_gate() {
        let (plan, _) = effective_plan(
            "apt-get install -y oldpkg\n\
             for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\n",
            |entity| {
                if entity == "oldpkg" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
            Some(&|entity: &str| (entity == "oldpkg").then(|| "oldpkg".to_owned())),
        );
        assert!(
            matches!(
                find(&plan, "apt-get install -y \"$pkg\"").disposition,
                Disposition::Run
            ),
            "the existing Members self-reach gate rejects this external writer before aggregate \
             freshness can license anything; widening that gate is outside this repair"
        );
    }

    /// A GUARD is itself a wall for everything below it (`30K` §5.3), isolated so the pin can only
    /// pass for that reason: the running wall's footprint is disjoint from the third site, so the
    /// ONLY thing that can stale it is the guard in between — which carries no footprint, and so
    /// walls total.
    #[test]
    fn a_guard_is_the_only_wall_the_third_site_can_be_stale_from() {
        let (plan, _) = effective_plan(
            "apt-get install -y oldpkg\napt-get install -y nginx\napt-get install -y curl\n",
            |e| {
                if e == "oldpkg" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
            // oldpkg's wall claims nginx's cell (so nginx must guard) and nothing else; nginx's own
            // guard is UNFOOTPRINTED, so if a guard walls at all, curl is stale.
            Some(&|e: &str| match e {
                "oldpkg" => Some("nginx".to_owned()),
                "curl" => Some("curl".to_owned()),
                _ => None,
            }),
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "the colliding wall costs nginx its elision"
        );
        assert!(
            matches!(
                find(&plan, "install -y curl").disposition,
                Disposition::Guard(_)
            ),
            "curl is disjoint from the RUNNING wall, so only the guard in between can stale it — \
             a guard's untouched fallback is the authored mutation"
        );
    }

    /// A `$( … )` body command mutates without being a leaf, and its wall follows its OWNER: the
    /// enclosing line's decision is what can remove it (`30K` §3.7). Dropping the non-leaf
    /// invalidators would elide the site below against a mutation that really runs.
    #[test]
    fn an_expansion_internal_mutation_walls_through_its_owner() {
        let (plan, _) = effective_plan(
            "echo \"$(apt-get install -y oldpkg)\"\napt-get install -y nginx\n",
            |_| Verdict::Converged,
            None,
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "the substitution's install really runs when its `echo` runs, so the site below is stale"
        );
    }

    /// A replacement the RENDER will refuse keeps its wall (`30K` §2.4 — the wrong-yes fence). A
    /// heredoc-carrying leaf renders verbatim however licensed it was, so treating its `Replace`
    /// as proof of no-execution would license every downstream elision against a live mutation.
    #[test]
    fn a_render_refused_replacement_never_retires_its_wall() {
        let (plan, _) = effective_plan(
            "apt-get install -y oldpkg <<EOF\npayload\nEOF\napt-get install -y nginx\n",
            |_| Verdict::Converged,
            None,
        );
        assert!(
            matches!(
                find(&plan, "install -y nginx").disposition,
                Disposition::Guard(_)
            ),
            "the heredoc site's licensed Replace is refused at render, so its wall stands"
        );
    }

    /// The survival books the mode-gate equality test iterates (install-only shapes so the
    /// corpus predict resolves them without the purge effect-add; the purge/kill and cross-kind
    /// shapes ride the e2e cases). Each is a diverged wall (or two) plus a converged same-kind
    /// different-entity survivor.
    const SURVIVAL_BOOKS: &[&str] = &[
        "apt-get install -y oldpkg\napt-get install -y nginx\n",
        "apt-get install -y oldpkg\napt-get install -y badpkg\napt-get install -y nginx\n",
    ];

    /// Build a plan for an install-only book, answering `package:<entity>@installed` with
    /// `verdict_of(entity)`. `survival` selects the walk: `None` ⇒ Stage-1 total wall;
    /// `Some(self_footprints)` ⇒ every establish-bearing node footprints its OWN coordinate
    /// (coherent by construction — the well-authored oracle shape), so a wall is disjoint from
    /// any different-entity downstream.
    fn survival_plan(
        src: &str,
        verdict_of: impl Fn(&str) -> Verdict,
        self_footprints: bool,
    ) -> Plan {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let provider = i.intern("apt-get");
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut arena,
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        // Every establish-bearing node footprints its own coordinate (the coherent shape).
        let footprints = self_footprints.then(|| {
            let mut tf = TrustedFootprints::new();
            for (node, class) in &classes {
                let fact = match class {
                    SkipClass::EstablishProbeAmbient(f) | SkipClass::EstablishProbeWritten(f) => *f,
                    _ => continue,
                };
                let coord = EntityCoord::new(fact.kind, fact.entity);
                if let Some(fp) = Footprint::authored(provider, vec![coord]) {
                    tf.insert(*node, fp);
                }
            }
            tf
        });
        let observe = |f: FactKey| {
            if f.kind == package
                && f.selector == installed
                && let EntityRef::Operand(tok) = f.entity
            {
                return Observable::verdict_only(verdict_of(i.resolve(tok.0)));
            }
            Observable::verdict_only(Verdict::Unknown)
        };
        let classification = RoundClassification {
            classes: classes.clone(),
            kills: BTreeSet::new(),
            invalidators,
            fact_backings: BTreeMap::new(),
        };
        let resolutions = Resolutions::none();
        let dialect = dorc_core::Dialect::empty();
        let policy = match footprints.as_ref() {
            Some(fp) => WallPolicy::RiskAccepted {
                footprints: fp,
                resolutions: &resolutions,
                dialect: &dialect,
            },
            None => WallPolicy::Honest,
        };
        let mut trip = dorc_analysis::certify::CertifierTrip::default();
        let mut spine = build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classification,
            policy,
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            &BTreeMap::new(),
            observe,
            &mut arena,
            &mut trip,
            None,
        );
        certifier_trip::project_censusless(&mut spine, &trip, &PlanAuthority::without_intake())
    }

    /// rul24-mode-gate BOTH-SIDES pin (the PRIMARY unflagged-equality guard, per the human's
    /// testing-protocol trim): the flag-OFF plan must be byte-identical to the honest Stage-1
    /// total wall. Proven by comparing `None` (unflagged) against `Some(EMPTY footprints)` — with
    /// no footprints every running mutator is a TOTAL wall (silence=wall), so the survival walk's
    /// output must MATCH the Stage-1 (`None`) walk's on every survival book. The flag buys
    /// survivals ONLY where a real footprint licenses one; absent footprints, it changes nothing.
    #[test]
    fn flag_off_equals_stage1_total_wall_on_survival_books() {
        let verdict = |e: &str| {
            if e == "nginx" {
                Verdict::Converged
            } else {
                Verdict::Diverged // oldpkg / badpkg diverged ⇒ the walls RUN
            }
        };
        for src in SURVIVAL_BOOKS {
            let none = survival_plan(src, verdict, false);
            // Some(empty): the survival walk with NO footprints ⇒ every wall is total ⇒ Stage-1.
            let empty = survival_plan_empty_footprints(src, verdict);
            let tag = |d: &Disposition| match d {
                Disposition::Run => "run",
                Disposition::Replace(..) => "replace",
                Disposition::Omit { .. } => "omit",
                Disposition::Guard(_) => "guard",
            };
            let disp = |p: &Plan| -> Vec<&'static str> {
                p.steps.iter().map(|s| tag(&s.disposition)).collect()
            };
            assert_eq!(
                disp(&none),
                disp(&empty),
                "flag-off (None) must equal Some(empty footprints) — the Stage-1 total wall — on {src:?}"
            );
            // And the survivor DEMOTES (the honest baseline: no footprint ⇒ no survival) — onto the
            // guard rung, which is where an elision the walls refused now lands.
            assert!(
                matches!(
                    find(&none, "install -y nginx").disposition,
                    Disposition::Guard(_)
                ),
                "unflagged: the converged nginx loses its elision past the running wall on {src:?}"
            );
        }
    }

    /// The empty-footprints variant (a survival walk that finds no footprint for any wall ⇒
    /// every wall total ⇒ Stage-1 behaviour). Separate helper so the equality test can compare
    /// the two walk entries directly.
    fn survival_plan_empty_footprints(src: &str, verdict_of: impl Fn(&str) -> Verdict) -> Plan {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let mut arena = dorc_core::ProvArena::new();
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut arena,
        );
        let classes = classification.value;
        let invalidators = classification.invalidators;
        let empty = TrustedFootprints::new();
        let observe = |f: FactKey| {
            if f.kind == package
                && f.selector == installed
                && let EntityRef::Operand(tok) = f.entity
            {
                return Observable::verdict_only(verdict_of(i.resolve(tok.0)));
            }
            Observable::verdict_only(Verdict::Unknown)
        };
        let classification = RoundClassification {
            classes: classes.clone(),
            kills: BTreeSet::new(),
            invalidators,
            fact_backings: BTreeMap::new(),
        };
        let resolutions = Resolutions::none();
        let dialect = dorc_core::Dialect::empty();
        let mut trip = dorc_analysis::certify::CertifierTrip::default();
        let mut spine = build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classification,
            WallPolicy::RiskAccepted {
                footprints: &empty,
                resolutions: &resolutions,
                dialect: &dialect,
            },
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            &BTreeMap::new(),
            observe,
            &mut arena,
            &mut trip,
            None,
        );
        certifier_trip::project_censusless(&mut spine, &trip, &PlanAuthority::without_intake())
    }

    /// The FLAGGED survival path at the plan level: a converged install past a running DIVERGED
    /// install of a different package SURVIVES when the wall's footprint (package:oldpkg) is
    /// disjoint from the downstream backing (package:nginx) — same kind, different entity. Its
    /// `Replace` carries a survival witness naming the crossed wall.
    #[test]
    fn disjoint_footprint_survives_running_wall() {
        let verdict = |e: &str| {
            if e == "nginx" {
                Verdict::Converged
            } else {
                Verdict::Diverged
            }
        };
        let plan = survival_plan(
            "apt-get install -y oldpkg\napt-get install -y nginx\n",
            verdict,
            true,
        );
        assert!(
            matches!(
                find(&plan, "install -y oldpkg").disposition,
                Disposition::Run
            ),
            "the diverged oldpkg install RUNS = the footprinted wall"
        );
        match &find(&plan, "install -y nginx").disposition {
            Disposition::Replace(license, _) => {
                let witness = license
                    .derivation()
                    .survival
                    .as_ref()
                    .expect("a survived elision past a wall carries a survival witness");
                let SurvivalAttribution::Standalone(witness) = witness else {
                    panic!("one establish must carry standalone survival attribution")
                };
                assert_eq!(witness.crossings().len(), 1, "one wall crossed");
            }
            other => panic!("nginx must SURVIVE (Replace) past the disjoint wall, got {other:?}"),
        }
    }

    // (The non-disjoint HIT direction — a footprint intersecting the backing demotes even
    // flagged — is pinned by `survival::tests::poisoned_backing_demotes` + the
    // `strawman24-nonsurvive-hit` e2e case; no plan-level duplicate here.)

    #[test]
    fn elide_tally_splits_proof_from_trusted_claim_on_the_real_survival_path() {
        // The two halves carry different risk, so a receipt owes the split. Tallied over the SAME
        // fixture that mints a REAL witness — a hand-built one would let a mis-wired split pass.
        let verdict = |e: &str| {
            if e == "nginx" {
                Verdict::Converged
            } else {
                Verdict::Diverged
            }
        };
        let flagged = survival_plan(
            "apt-get install -y oldpkg\napt-get install -y nginx\n",
            verdict,
            true,
        )
        .disposition_counts();
        assert_eq!(flagged.elide, 1, "one line elided");
        assert_eq!(
            (flagged.elide_by_trusted_claim, flagged.elide_by_proof),
            (1, 0),
            "an elision kept past a RUNNING wall rests on the claim, never on proof alone"
        );

        let clean = survival_plan("apt-get install -y nginx\n", verdict, true).disposition_counts();
        assert_eq!(
            (clean.elide_by_trusted_claim, clean.elide_by_proof),
            (0, 1),
            "no wall crossed ⇒ the skip rests on the probed fact"
        );
        for c in [flagged, clean] {
            assert_eq!(
                c.elide,
                c.elide_by_proof + c.elide_by_trusted_claim,
                "the split partitions the elide bucket exactly"
            );
        }
    }

    #[test]
    fn survival_walk_mints_wall_and_demotion_narratives() {
        // C5 anti-masking (`AID-NEEDS:law-collapse-mints-narrative`): the running curl mutator mints
        // a WallFormation and the demoted nginx a Demotion — DERIVED from the collapse, all Derived.
        let plan = survival_plan_empty_footprints(
            "apt-get install -y curl\napt-get install -y nginx\n",
            |e| {
                if e == "curl" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
        );
        let ev = plan.survival_report.collapse_narrative();
        assert!(
            ev.iter()
                .any(|e| matches!(e.kind(), CollapseKind::WallFormation { .. })),
            "the running curl mutator mints a WallFormation"
        );
        assert!(
            ev.iter()
                .any(|e| matches!(e.kind(), CollapseKind::Demotion { .. })),
            "the demoted nginx install mints a Demotion"
        );
        assert!(
            ev.iter().all(|e| e.tier() == SpeechAct::Derived),
            "survival-walk narratives are engine-derived"
        );

        // The AGREEMENT direction (`289:rul-mint-hardening-package` item 4a): with nothing running
        // there is no wall and nothing to demote, so the same walk must mint neither class. Without
        // this, a mint that fired unconditionally would read as green above.
        let converged = survival_plan_empty_footprints(
            "apt-get install -y curl\napt-get install -y nginx\n",
            |_| Verdict::Converged,
        );
        assert!(
            !converged
                .survival_report
                .collapse_narrative()
                .iter()
                .any(|e| matches!(
                    e.kind(),
                    CollapseKind::WallFormation { .. } | CollapseKind::Demotion { .. }
                )),
            "an all-converged book forms no wall and demotes nothing, so it narrates neither: {:?}",
            converged.survival_report.collapse_narrative()
        );
    }

    #[test]
    fn in_loop_constant_establish_runs_even_when_converged() {
        // The in-loop render floor STILL holds (task-L1 / task-L2 item-3) for an in-loop
        // establish that is NOT a Members site — a CONSTANT install not referencing the
        // for-var. `for f in a b; do apt-get install -y nginx; done`: nginx is the same
        // cell every iteration (no member-family), so it takes the single-fact path and
        // the in-loop floor in `disposition_for` forces Run even when Converged. (task-L2
        // lifts the floor ONLY for the Members shape, below.)
        let (plan, _) = plan_for(
            "for f in a b; do apt-get install -y nginx; done\n",
            Verdict::Converged,
        );
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "a CONSTANT in-loop establish RUNS despite Converged (the floor still holds for non-Members)"
        );
    }

    #[test]
    fn in_loop_members_single_member_elides_when_converged() {
        // task-L2 item-3: a single-word for-loop's body install IS a (1-member) Members
        // site, so a Converged host + self-reach + no consumer ⇒ the in-loop floor LIFTS
        // and the body is Replaced by `true` (the loop still iterates once over `true`).
        // `for f in nginx; do apt-get install -y "$f"; done` ⇒ Replace. (Pre-L2 this was
        // the L1 floor's RUN; the member-elision slice unlocks it — the brk-1(b) payoff.)
        let (plan, _) = plan_for(
            "for f in nginx; do apt-get install -y \"$f\"; done\n",
            Verdict::Converged,
        );
        assert!(
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, StandIn::True)
            ),
            "a converged single-member in-loop install elides to `true` (item-3): {:?}",
            find(&plan, "apt-get install").disposition
        );
    }

    #[test]
    fn post_loop_install_elides_below_a_pure_loop() {
        // THE brk-1 value-unlock at the PLAN layer (the run-set-proven elision the e2e
        // `loop-post-elision-revives` case witnesses): a converged install BELOW a PURE
        // loop now ELIDES. Pre-L1 the loop was a ⊤ node whose ⊤-containment + havoc
        // killed this; with the loop lowered + a pure body, the post-loop install is
        // EstablishProbeAmbient and Converged ⇒ Replace.
        let (plan, _) = plan_for(
            "for f in a b; do echo \"$f\"; done\napt-get install -y nginx\n",
            Verdict::Converged,
        );
        assert!(
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, _)
            ),
            "a converged install below a pure loop ELIDES (the brk-1 value-unlock): {:?}",
            find(&plan, "apt-get install").disposition
        );
    }

    // --- task-L2 item-3: the all-or-nothing in-loop Members license (plan layer) -------

    #[test]
    fn members_all_converged_elides_to_true() {
        // THE item-3 payoff: `for pkg in nginx curl; do apt-get install -y "$pkg"; done`,
        // BOTH members converged ⇒ the body install is Replaced by `true` (the loop still
        // iterates twice over `true`). The brk-1(b) payoff at the plan layer.
        let (plan, _) = plan_for_pkgs(
            r#"for pkg in nginx curl; do apt-get install -y "$pkg"; done"#,
            |_| Verdict::Converged,
        );
        assert!(
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, StandIn::True)
            ),
            "both members converged ⇒ in-loop install elides to `true`: {:?}",
            find(&plan, "apt-get install").disposition
        );
    }

    #[test]
    fn members_partial_diverged_runs_whole_leaf() {
        // item-3(a) all-or-nothing: ONE member diverged ⇒ the WHOLE leaf runs (no
        // partial-member elision this slice). nginx converged, curl DIVERGED ⇒ Run.
        let (plan, _) = plan_for_pkgs(
            r#"for pkg in nginx curl; do apt-get install -y "$pkg"; done"#,
            |e| {
                if e == "curl" {
                    Verdict::Diverged
                } else {
                    Verdict::Converged
                }
            },
        );
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "one diverged member ⇒ the whole leaf RUNS (all-or-nothing): {:?}",
            find(&plan, "apt-get install").disposition
        );
    }

    #[test]
    fn members_external_writer_runs_despite_both_converged() {
        // item-3(b) self-reach: a PRE-LOOP `apt-get purge curl` writes a member cell ⇒
        // self-reach broken ⇒ NO license, EVEN with BOTH members converged (the bait). The
        // resting probe is no longer authoritative under the elision (the purge's effect is
        // a non-self writer). The install RUNS.
        let (plan, _) = plan_for_pkgs(
            "apt-get purge curl\nfor pkg in nginx curl; do apt-get install -y \"$pkg\"; done",
            |_| Verdict::Converged,
        );
        // The in-loop install (the SECOND `apt-get install` leaf — the purge is `apt-get
        // purge`) runs.
        let install = plan
            .steps
            .iter()
            .find(|s| s.sh.contains("apt-get install"))
            .expect("the in-loop install leaf");
        assert!(
            matches!(install.disposition, Disposition::Run),
            "a pre-loop purge of a member cell breaks self-reach ⇒ the install RUNS despite both-converged: {:?}",
            install.disposition
        );
    }

    #[test]
    fn members_in_loop_sibling_writer_runs_despite_both_converged() {
        // item-3(b) self-reach, the IN-LOOP-SIBLING-via-back-edge case (the adversarial
        // hunt the strain note flagged as the top crosscheck target): a sibling `apt-get
        // purge curl` INSIDE the loop body writes a member cell. The suppressed-solve must
        // catch it — the SIBLING's gen is NOT suppressed (only the install's own is), so the
        // purge's `curl@installed` reaches the install's in-state via the back-edge as a
        // NON-self writer ⇒ self-reach false ⇒ the install RUNS despite both members reported
        // converged. (Proves the suppressed-solve is sound against back-edge siblings, not
        // just pre-loop writers.)
        let (plan, _) = plan_for_pkgs(
            r#"for pkg in nginx curl; do apt-get install -y "$pkg"; apt-get purge -y curl; done"#,
            |_| Verdict::Converged,
        );
        let install = plan
            .steps
            .iter()
            .find(|s| s.sh.contains("apt-get install"))
            .expect("the in-loop install leaf");
        assert!(
            matches!(install.disposition, Disposition::Run),
            "an in-loop sibling purge of a member cell breaks self-reach ⇒ the install RUNS: {:?}",
            install.disposition
        );
    }

    #[test]
    fn members_var_reassign_body_runs() {
        // item-1 degrade at the plan layer: a body reassignment of the for-var ⇒ NOT a
        // Members site (the value-plane degraded to None) ⇒ the in-loop floor runs it.
        // `for pkg in nginx curl; do pkg=evil; apt-get install -y "$pkg"; done` ⇒ Run
        // (the install's argv is `evil`-or-⊤, never a converged member family).
        let (plan, _) = plan_for_pkgs(
            r#"for pkg in nginx curl; do pkg=evil; apt-get install -y "$pkg"; done"#,
            |_| Verdict::Converged,
        );
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "a body var-reassign ⇒ not a Members site ⇒ the floor runs it: {:?}",
            find(&plan, "apt-get install").disposition
        );
    }

    #[test]
    fn members_license_unit_all_conjuncts() {
        // The license minter (item-3) directly, each conjunct (anti-masking: a constructed
        // family + verdicts, not a hand-injected disposition). nginx+curl cells.
        let mut i = Interner::default();
        let kind = KindId(i.intern("package"));
        let selector = SelectorId(i.intern("installed"));
        let mut cell = |e: &str| FactKey {
            kind,
            entity: EntityRef::Operand(OpaqueToken(i.intern(e))),
            selector,
            context: dorc_core::Context::HostDefault,
        };
        let family = vec![cell("nginx"), cell("curl")];
        let both_converged = vec![Verdict::Converged, Verdict::Converged];
        let vouched = || {
            let site = CfgNodeId(17);
            let mut vouches = Vouches::new();
            for fact in &family {
                vouches.insert(site, *fact, test_vouch());
            }
            let expected = AggregateEstablishes::mint(
                family
                    .iter()
                    .map(|fact| AggregateEstablish::new(site, *fact))
                    .collect(),
            )
            .expect("the member identity is non-empty and unique");
            AllEstablishesVouched::mint(&expected, &vouches).expect("exact member vouches")
        };
        // All converged + self-reached + quiet ⇒ license.
        assert!(
            ReplaceLicense::prove_members_replaceable(
                vouched(),
                &both_converged,
                true,
                &quiet(),
                Predicted::Top,
            )
            .is_some(),
            "all-converged + self-reached + quiet ⇒ license"
        );
        // One diverged ⇒ no license (all-or-nothing).
        assert!(
            ReplaceLicense::prove_members_replaceable(
                vouched(),
                &[Verdict::Converged, Verdict::Diverged],
                true,
                &quiet(),
                Predicted::Top,
            )
            .is_none(),
            "one diverged member ⇒ no license"
        );
        // self-reach false ⇒ no license (even all-converged).
        assert!(
            ReplaceLicense::prove_members_replaceable(
                vouched(),
                &both_converged,
                false,
                &quiet(),
                Predicted::Top,
            )
            .is_none(),
            "self-reach false ⇒ no license"
        );
        // A consumed StatusRelaxable with the (⊤) mutator status ⇒ blocked (item-3(c) — the
        // errexit / post-loop-`$?` consumer with a ⊤ rc). This is why item-6a matters.
        assert!(
            ReplaceLicense::prove_members_replaceable(
                vouched(),
                &both_converged,
                true,
                &May(Powerset::singleton(Channel::StatusRelaxable)),
                Predicted::Top,
            )
            .is_none(),
            "a consumed status with a ⊤ mutator rc ⇒ blocked (item-3(c))"
        );
    }

    #[test]
    fn members_duplicate_supplied_vouch_runs_atomically() {
        let plan = plan_with_duplicate_aggregate_vouch(
            "for pkg in nginx curl; do apt-get install -y \"$pkg\"; done\n",
        );
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "a duplicate supplied member identity refuses the entire loop replacement"
        );
    }

    #[test]
    fn inline_duplicate_supplied_vouch_runs_atomically() {
        let plan = plan_with_duplicate_aggregate_vouch(
            "install() { apt-get install -y \"$1\"; }\ninstall nginx\n",
        );
        assert!(
            matches!(find(&plan, "install nginx").disposition, Disposition::Run),
            "a duplicate supplied inline identity refuses the whole call replacement"
        );
    }

    #[test]
    fn aggregate_vouches_require_exact_ordered_site_fact_pairs() {
        let mut i = Interner::default();
        let kind = KindId(i.intern("package"));
        let selector = SelectorId(i.intern("installed"));
        let fact = |entity| FactKey {
            kind,
            entity: EntityRef::Operand(OpaqueToken(entity)),
            selector,
            context: dorc_core::Context::HostDefault,
        };
        let nginx = fact(i.intern("nginx"));
        let curl = fact(i.intern("curl"));
        let wombat = fact(i.intern("wombat"));
        let first = CfgNodeId(41);
        let second = CfgNodeId(42);
        let expected = || {
            AggregateEstablishes::mint(vec![
                AggregateEstablish::new(first, nginx),
                AggregateEstablish::new(second, curl),
            ])
            .expect("the expected identity is non-empty and unique")
        };
        let exact = || {
            let mut vouches = Vouches::new();
            vouches.insert(first, nginx, test_vouch());
            vouches.insert(second, curl, test_vouch());
            vouches
        };
        assert!(AllEstablishesVouched::mint(&expected(), &exact()).is_some());

        let mut missing = Vouches::new();
        missing.insert(first, nginx, test_vouch());
        assert!(AllEstablishesVouched::mint(&expected(), &missing).is_none());

        let mut extra = exact();
        extra.insert(first, wombat, test_vouch());
        assert!(AllEstablishesVouched::mint(&expected(), &extra).is_none());

        let mut reordered = Vouches::new();
        reordered.insert(second, curl, test_vouch());
        reordered.insert(first, nginx, test_vouch());
        assert!(AllEstablishesVouched::mint(&expected(), &reordered).is_none());

        let mut wrong_site = Vouches::new();
        wrong_site.insert(first, nginx, test_vouch());
        wrong_site.insert(CfgNodeId(43), curl, test_vouch());
        assert!(AllEstablishesVouched::mint(&expected(), &wrong_site).is_none());

        let mut duplicate = exact();
        duplicate.insert(first, nginx, test_vouch());
        assert!(AllEstablishesVouched::mint(&expected(), &duplicate).is_none());

        assert!(
            AggregateEstablishes::mint(vec![
                AggregateEstablish::new(first, nginx),
                AggregateEstablish::new(first, nginx),
            ])
            .is_none(),
            "duplicate aggregate identities are unrepresentable before vouch matching"
        );
    }

    /// `30L:pin-shared-witness-spans-instances` — a SHARED replacement's witness spans every
    /// contributing instance, and a per-call one never stands in for it.
    ///
    /// The mistake-shape this forbids is the aggregate lane's own, one abstraction level up
    /// (`30Kb` §1): two invocations of one authored region erase two DIFFERENT cells, each licensed
    /// by its own reached vouch, so a license carrying only the first instance's establish would
    /// retire the second's mutation on a vouch nobody gave for it. The identity- and
    /// cardinality-match is what makes that unspellable, and the mint here is the same one the
    /// settlement's shared-region seat uses.
    #[test]
    fn a_shared_region_license_spans_every_contributing_instance() {
        let mut i = Interner::default();
        let kind = KindId(i.intern("package"));
        let selector = SelectorId(i.intern("installed"));
        let cell = |entity| FactKey {
            kind,
            entity: EntityRef::Operand(OpaqueToken(entity)),
            selector,
            context: dorc_core::Context::HostDefault,
        };
        let (nginx, curl) = (cell(i.intern("nginx")), cell(i.intern("curl")));
        let (first, second) = (CfgNodeId(11), CfgNodeId(12));
        let population = AggregateEstablishes::mint(vec![
            AggregateEstablish::new(first, nginx),
            AggregateEstablish::new(second, curl),
        ])
        .expect("two instances of one region");

        let mut per_call = Vouches::new();
        per_call.insert(first, nginx, test_vouch());
        assert!(
            AllEstablishesVouched::mint(&population, &per_call).is_none(),
            "one instance's vouch cannot license the edit the other instance also executes"
        );

        let mut spanning = Vouches::new();
        spanning.insert(first, nginx, test_vouch());
        spanning.insert(second, curl, test_vouch());
        let all_vouched = AllEstablishesVouched::mint(&population, &spanning)
            .expect("the cross-instance witness");
        let license = ReplaceLicense::prove_shared_region_replaceable(
            all_vouched,
            &May(Powerset::default()),
            Predicted::Top,
        )
        .expect("an all-vouched population with no consumed channel replaces");
        assert_eq!(
            license
                .derivation()
                .establish_vouches
                .iter()
                .map(|receipt| (receipt.site, receipt.fact))
                .collect::<Vec<_>>(),
            vec![(first, nginx), (second, curl)],
            "the witness carries the exact ORDERED union across instances, not a representative"
        );
        assert!(
            matches!(license.derivation().via, LicenseVia::SharedRegion),
            "and it names the region path, so a reader can tell it from a per-call aggregate"
        );
    }

    #[test]
    fn substitution_internal_command_is_not_a_plan_leaf() {
        // find-cli-1: the `$(uname)` body command must NOT be a plan Step (it runs
        // during word expansion, not as a leaf); the two top-level commands are the
        // only leaves. Before the fix this rendered a third, garbage step from the
        // substring-relative span of the subst body.
        let (plan, _) = plan_for(
            "echo $(uname)\napt-get install -y nginx\n",
            Verdict::Diverged,
        );
        assert_eq!(
            plan.steps.len(),
            2,
            "only the two top-level commands are leaves: {:?}",
            plan.steps.iter().map(|s| s.sh.clone()).collect::<Vec<_>>()
        );
        assert!(
            plan.steps.iter().any(|s| s.sh.starts_with("echo")),
            "echo is a leaf"
        );
        assert!(
            plan.steps.iter().any(|s| s.sh.contains("apt-get install")),
            "install is a leaf"
        );
    }

    #[test]
    fn consumption_fact_total_over_classify_leaves() {
        // def-5 (note 16J §4): consumption is computed in the single lowering
        // traversal and stored per node, so EVERY classify leaf has it defined — the
        // "absent leaf" that slipped the old plan-side dual-traversal (16I bug-c) is
        // structurally impossible. Cross-check the join: every leaf is queryable, the
        // group-redirected install is marked Stdout, and the lone install is quiet.
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let src = "{ apt-get install -y nginx; } > /tmp/out\napt-get install -y curl\n";
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let classification = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &dorc_oracle::verdict::VerdictIndex::default(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        );
        let classes = classification.value;
        assert!(!classes.is_empty(), "fixture has classify leaves");
        let (mut marked, mut quiet) = (0, 0);
        for (node, _) in &classes {
            // Total Vec ⇒ defined for every classify leaf (never an absent lookup).
            if cfg.consumed_observables(*node).contains(&Channel::Stdout) {
                marked += 1;
            } else {
                quiet += 1;
            }
        }
        assert!(marked >= 1, "the group-redirected install is marked Stdout");
        assert!(quiet >= 1, "the lone curl install is quiet");
    }

    // === P1 fix 21E: the comment-safety quote machine (f-2) + edit grouping (f-1) ===

    #[test]
    fn region_ends_in_quote_tracks_posix_quote_state() {
        // f-2: the trailing-`#` safety hinges on "does the rendered line end inside a string
        // literal?". These pin the minimal POSIX quote machine against dash's semantics.
        // OUTSIDE any quote ⇒ safe (false):
        assert!(!region_ends_in_quote("true; true"), "balanced, no quote");
        assert!(
            !region_ends_in_quote(r#"echo "closed""#),
            "closed double-quote"
        );
        assert!(
            !region_ends_in_quote("echo 'closed'"),
            "closed single-quote"
        );
        assert!(
            !region_ends_in_quote(r#"echo "a 'nested' b""#),
            "single inside a closed double does not leak"
        );
        assert!(
            !region_ends_in_quote(r#"echo \""#),
            "an unquoted-escaped quote is a literal, not an opener"
        );
        // INSIDE an open quote ⇒ unsafe (true) — a `#` would land in the literal:
        assert!(
            region_ends_in_quote(r#"apt-get install -y "c"#),
            "open double-quote"
        );
        assert!(
            region_ends_in_quote("apt-get install -y 'c"),
            "open single-quote"
        );
        assert!(
            region_ends_in_quote(r#"echo "a'b x"; foo "c"#),
            "the SECOND double-quote is still open (the P1 odd-quote shape)"
        );
        assert!(
            region_ends_in_quote(r#"echo "a\""#),
            "a `\\\"` inside double does NOT close it ⇒ still open"
        );
        // A single-quote suppresses escaping: `\` is literal, so the quote stays open.
        assert!(
            region_ends_in_quote(r"echo 'a\"),
            "backslash is literal inside single-quotes ⇒ quote still open"
        );
        // A dangling unquoted backslash is its own continuation hazard ⇒ unsafe.
        assert!(
            region_ends_in_quote(r"echo a\"),
            "dangling unquoted backslash"
        );
    }

    /// The load-bearing safety precondition of the commented-original elision render (human's
    /// round-24 lean): `is_alone_on_line` must REFUSE any command sharing its line with a sibling
    /// statement — else a `# <original>` rewrite (a `#` runs to end-of-line) silently kills the
    /// sibling. This is exactly the `pre-loop-shared-for-line` / `exec-multileaf-line-mixed`
    /// regression the whole-line check closes.
    #[test]
    fn is_alone_on_line_refuses_shared_lines() {
        // Alone on its own line ⇒ safe.
        let s = "apt-get install -y nginx\n";
        assert!(
            is_alone_on_line(s, 0, s.trim_end().len()),
            "a whole-line command"
        );
        // A trailing comment after the command is fine (still the line's last live token).
        let s2 = "apt-get install -y nginx  # note\n";
        let hi = s2.find("  #").unwrap();
        assert!(
            is_alone_on_line(s2, 0, hi),
            "a trailing comment does not disqualify"
        );
        // A SIBLING after `;` on the same line ⇒ REFUSE (commenting would kill it).
        let s3 = "apt-get install -y nginx; systemctl reload nginx\n";
        let hi3 = s3.find(';').unwrap();
        assert!(
            !is_alone_on_line(s3, 0, hi3),
            "`; systemctl …` follows ⇒ not alone"
        );
        // A `for …` continuation after `;` ⇒ REFUSE (the pre-loop-shared-for-line shape).
        let s4 = "apt-get install -y nginx; for x in a\ndo echo; done\n";
        let hi4 = s4.find(';').unwrap();
        assert!(
            !is_alone_on_line(s4, 0, hi4),
            "`; for …` follows ⇒ not alone"
        );
        // Leading code before the command on the line ⇒ REFUSE (would be swallowed).
        let s5 = "foo; apt-get install -y nginx\n";
        let lo5 = s5.find("apt-get").unwrap();
        assert!(
            !is_alone_on_line(s5, lo5, s5.trim_end().len()),
            "leading `foo; ` on the line ⇒ not alone"
        );
    }

    #[test]
    fn comment_safe_refuses_open_quote_line() {
        // The f-2 wiring: `comment_safe` now refuses a line that ends inside an open quote,
        // on TOP of the prior trailing-`\` and `<<` rejections.
        assert!(comment_safe("true; true"), "balanced ⇒ safe");
        assert!(
            !comment_safe(r#"true; apt-get install -y "c"#),
            "ends inside an open double-quote ⇒ refuse (the orphan-corruption shape)"
        );
        assert!(!comment_safe(r"echo foo \"), "trailing backslash ⇒ refuse");
        assert!(!comment_safe("cat <<EOF"), "heredoc operator ⇒ refuse");
    }

    // ── Span-edit non-overlap, exhaustively over a small interval universe ────────────────
    //
    // PLACEMENT, flagged: a ∀-law over small values is Kani-tier by shape. It is HERE because
    // `SpanEdit` lives inside `dorc-plan`, and the Kani verification unit
    // (`spike/verify/kani`) is a dependency-free `#[path]`-include of the algebra sources —
    // this crate's closure cannot enter it, and `String`-carrying values are the shape a
    // bounded model checker pays most for anyway. So the quantifier is a loop over every
    // interval collection a five-byte source can express, which is exhaustive over that
    // universe rather than over a bound.

    /// An edit's identity for these assertions. The strings are payload; the law is about the
    /// intervals and about which of them survive.
    fn span_of(edit: &SpanEdit) -> (usize, usize) {
        (edit.lo, edit.hi)
    }

    /// Disjoint-or-nested is `normalise_edits`'s PRECONDITION, guaranteed by the leaf seam
    /// (two leaf command spans never cross). Feeding it a partial overlap is a `debug_assert`
    /// by design, so the enumeration excludes those rather than asserting on them.
    fn disjoint_or_nested(a: (usize, usize), b: (usize, usize)) -> bool {
        let disjoint = a.1 <= b.0 || b.1 <= a.0;
        let nested = (a.0 <= b.0 && b.1 <= a.1) || (b.0 <= a.0 && a.1 <= b.1);
        disjoint || nested
    }

    fn edit_at(lo: usize, hi: usize) -> SpanEdit {
        SpanEdit {
            lo,
            hi,
            replacement: "true".into(),
            original: "cmd".into(),
            self_commented: false,
            comment_out: false,
        }
    }

    #[test]
    fn normalise_edits_yields_a_sorted_pairwise_disjoint_set() {
        // THE property `emit_span_edits` rests on. It splices a group right-to-left so that an
        // earlier edit's byte offsets stay valid as later ones land, and that is only sound if
        // the kept edits are sorted and share no byte. An overlapping survivor would splice
        // into bytes another edit had already rewritten — a corrupt artifact, and one that
        // `dash -n` would not necessarily catch.
        let universe: Vec<(usize, usize)> = (0..5)
            .flat_map(|lo| (lo + 1..=5).map(move |hi| (lo, hi)))
            .collect();

        let mut collections_checked = 0usize;
        for a in &universe {
            for b in &universe {
                for c in &universe {
                    let raw = [*a, *b, *c];
                    let legal = disjoint_or_nested(*a, *b)
                        && disjoint_or_nested(*a, *c)
                        && disjoint_or_nested(*b, *c);
                    if !legal {
                        continue;
                    }
                    collections_checked += 1;

                    let kept =
                        normalise_edits(raw.iter().map(|(lo, hi)| edit_at(*lo, *hi)).collect());

                    for pair in kept.windows(2) {
                        let (left, right) = (span_of(&pair[0]), span_of(&pair[1]));
                        assert!(left.0 <= right.0, "sorted by lo: {raw:?} kept {kept:?}");
                        assert!(
                            right.0 >= left.1,
                            "survivors share no byte: {raw:?} kept {kept:?}"
                        );
                    }
                    for edit in &kept {
                        assert!(
                            raw.contains(&span_of(edit)),
                            "a survivor is always an input: {raw:?} kept {kept:?}"
                        );
                    }
                    assert!(
                        !kept.is_empty(),
                        "a non-empty legal input never normalises to nothing: {raw:?}"
                    );
                }
            }
        }
        assert!(
            collections_checked > 100,
            "the enumeration must actually cover a universe, not fall through its filter"
        );
    }

    #[test]
    fn normalise_edits_keeps_the_outer_edit_of_a_nested_pair() {
        // The containment rule, and the direction matters: a folded construct's edit SUBSUMES
        // its interior leaves', so the outer wins and the inner is dropped. Keeping the inner
        // instead would splice a leaf's replacement into a region the construct's own edit was
        // about to rewrite wholesale.
        let kept = normalise_edits(vec![edit_at(2, 4), edit_at(0, 8)]);
        assert_eq!(kept.len(), 1, "one survivor: {kept:?}");
        assert_eq!(span_of(&kept[0]), (0, 8), "and it is the OUTER edit");

        // Equal spans are the degenerate containment: exactly one survives, never both.
        let identical = normalise_edits(vec![edit_at(1, 5), edit_at(1, 5)]);
        assert_eq!(identical.len(), 1, "duplicates collapse: {identical:?}");
    }

    #[test]
    fn group_edits_merges_abutting_multiline_edits() {
        // f-1: two edits whose line ranges ABUT (edit B starts on edit A's end line) must
        // land in ONE group — the pre-fix orphan. Source (4 lines, 0-indexed):
        //   0: apt-get install -y "a
        //   1: b"; apt-get install -y "c
        //   2: d"
        // Edit A spans lines 0-1, edit B spans lines 1-2. Build the two span edits by hand
        // (byte offsets into `src`) and assert they group into a single 0..2 region.
        let src = "apt-get install -y \"a\nb\"; apt-get install -y \"c\nd\"\n";
        let a_lo = 0;
        let a_hi = src.find("\";").unwrap() + 1; // through the closing `"` of operand a/b
        let b_lo = src.find("; ").unwrap() + 2; // the second `apt-get`
        let b_hi = src.rfind('"').unwrap() + 1; // through the closing `"` of operand c/d
        let edits = normalise_edits(vec![
            SpanEdit {
                lo: a_lo,
                hi: a_hi,
                replacement: "true".into(),
                original: "apt-get install -y \"a\nb\"".into(),
                self_commented: false,
                comment_out: false,
            },
            SpanEdit {
                lo: b_lo,
                hi: b_hi,
                replacement: "true".into(),
                original: "apt-get install -y \"c\nd\"".into(),
                self_commented: false,
                comment_out: false,
            },
        ]);
        let groups = group_edits(src, &edits);
        assert_eq!(
            groups.len(),
            1,
            "the two abutting edits form ONE group: {groups:#?}",
            groups = groups.keys()
        );
        let g = groups.get(&0).expect("group keyed by its first line (0)");
        assert_eq!(
            g.last_line, 2,
            "the group covers through line 2 (operand c/d's close)"
        );
        assert_eq!(g.members.len(), 2, "both edits are members (none orphaned)");
    }

    #[test]
    fn group_edits_keeps_disjoint_edits_separate() {
        // f-1 counter-check: edits on NON-adjacent lines stay in distinct groups (the fix
        // must not over-merge). Two single-line installs separated by a blank line.
        let src = "apt-get install -y nginx\n\napt-get install -y curl\n";
        let a_hi = src.find('\n').unwrap();
        let b_lo = src.rfind("apt-get").unwrap();
        let b_hi = src.rfind("curl").unwrap() + "curl".len();
        let edits = normalise_edits(vec![
            SpanEdit {
                lo: 0,
                hi: a_hi,
                replacement: "true".into(),
                original: "apt-get install -y nginx".into(),
                self_commented: false,
                comment_out: false,
            },
            SpanEdit {
                lo: b_lo,
                hi: b_hi,
                replacement: "true".into(),
                original: "apt-get install -y curl".into(),
                self_commented: false,
                comment_out: false,
            },
        ]);
        let groups = group_edits(src, &edits);
        assert_eq!(
            groups.len(),
            2,
            "disjoint-line edits are NOT merged: {:#?}",
            groups.keys()
        );
    }

    // ---- 24J §2: connected check-pipe recognition (`connected_check_pipes`) ----

    /// The `CfgNodeId` of the leaf `Command` whose Simple's first word is `word`. The recognition
    /// pins assert governing/member/orphan membership against these ids.
    fn pipe_node(cfg: &Cfg, ast: &Ast, word: &str) -> CfgNodeId {
        use dorc_syntax::ast::{NodeKind, WordPart};
        for (id, cnode) in cfg.iter() {
            if !matches!(cnode.kind, CfgNodeKind::Command) {
                continue;
            }
            if let NodeKind::Simple { words, .. } = &ast.node(cnode.ast).kind
                && let Some(&w) = words.first()
                && let NodeKind::Word { parts } = &ast.node(w).kind
                && matches!(parts.as_slice(), [WordPart::Literal(s)] if s == word)
            {
                return id;
            }
        }
        panic!("no leaf command `{word}` in the cfg");
    }

    /// Parse `src`, build its cfg + value-flow, and mark every leaf `Command` whose first word is in
    /// `queries` as a `QueryResolvable` (the read-only vouch the recognition requires — the fact's
    /// identity is irrelevant, the recognition keys on structure). A stage word NOT listed is absent
    /// from `classes`, so the decider sees it as non-Query (opaque/mutator). Returns the interner too
    /// (the value-flow interned the argv the decider resolves, and the ship closures resolve providers).
    fn pipe_fixture(
        src: &str,
        queries: &[&str],
    ) -> (Ast, Cfg, ValueFlow, Vec<(CfgNodeId, SkipClass)>, Interner) {
        use dorc_syntax::ast::{NodeKind, WordPart};
        let ast = dorc_syntax::parse(src).value;
        let cfg = dorc_analysis::cfg::build(&ast).value;
        let mut i = Interner::default();
        let value = dorc_analysis::value::analyze(&cfg, &ast, &mut i);
        let mut classes = Vec::new();
        for (node, cnode) in cfg.iter() {
            if !matches!(cnode.kind, CfgNodeKind::Command) {
                continue;
            }
            let NodeKind::Simple { words, .. } = &ast.node(cnode.ast).kind else {
                continue;
            };
            let Some(&w) = words.first() else { continue };
            let NodeKind::Word { parts } = &ast.node(w).kind else {
                continue;
            };
            let [WordPart::Literal(name)] = parts.as_slice() else {
                continue;
            };
            if queries.contains(&name.as_str()) {
                classes.push((
                    node,
                    SkipClass::QueryResolvable {
                        fact: FactKey {
                            kind: KindId(i.intern("grepmatch")),
                            entity: EntityRef::Operand(OpaqueToken(i.intern(name))),
                            selector: SelectorId(i.intern("matched")),
                            context: dorc_core::Context::HostDefault,
                        },
                        valid: true,
                    },
                ));
            }
        }
        (ast, cfg, value, classes, i)
    }

    /// A stub stage-ship: every stage resolves to a trivial delegation body producing REAL stdout
    /// (the coverage a downstream byte-consumer requires). The recognition tests key on membership,
    /// not the shipped bytes, so a constant stub suffices.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "must match the `ship_stage: Fn(..) -> Option<StageShip>` closure signature"
    )]
    fn ship_all_real(_n: CfgNodeId, _p: Symbol, _a: &[Symbol]) -> Option<StageShip> {
        Some(StageShip {
            sh: "stub__predict() { :; }".to_owned(),
            produces_real_stdout: true,
        })
    }

    #[test]
    fn connected_recognises_all_query_two_stage() {
        // The flagship shape: `A | F` with BOTH stages vouched read-only Queries + resolving predicts
        // ⇒ a COMPOSED probe (`271:rul-only-oracle-bytes-ship`). The LAST stage governs (carries the
        // composed body); the first stage is a subsumed member keyed to the governor. No orphans.
        let src = "otelcol --version | grep -q x || curl y\n";
        let (ast, cfg, value, classes, i) = pipe_fixture(src, &["otelcol", "grep"]);
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship_all_real);
        let gov = pipe_node(&cfg, &ast, "grep");
        let a = pipe_node(&cfg, &ast, "otelcol");
        let composed = c
            .governing_composed(gov)
            .expect("the governing (last) stage carries the composed probe");
        let providers: Vec<&str> = composed
            .stages
            .iter()
            .map(|s| i.resolve(s.provider))
            .collect();
        assert_eq!(
            providers,
            vec!["otelcol", "grep"],
            "the composed probe carries each stage's provider in pipe order (governing last)"
        );
        assert_eq!(
            c.member_governor(a),
            Some(gov),
            "the non-last stage is a member keyed to the governor"
        );
        assert!(
            !c.is_orphan_stage(a) && !c.is_orphan_stage(gov),
            "a shippable connected pipe has no orphans"
        );
    }

    #[test]
    fn connected_recognises_three_stage() {
        // `A | B | F` all-Query + resolving ⇒ two members (A, B) + the governor F.
        let src = "a p | b q | grep -q x\n";
        let (ast, cfg, value, classes, i) = pipe_fixture(src, &["a", "b", "grep"]);
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship_all_real);
        let gov = pipe_node(&cfg, &ast, "grep");
        assert_eq!(c.member_governor(pipe_node(&cfg, &ast, "a")), Some(gov));
        assert_eq!(c.member_governor(pipe_node(&cfg, &ast, "b")), Some(gov));
        let composed = c
            .governing_composed(gov)
            .expect("shippable three-stage pipe");
        let providers: Vec<&str> = composed
            .stages
            .iter()
            .map(|s| i.resolve(s.provider))
            .collect();
        assert_eq!(providers, vec!["a", "b", "grep"]);
    }

    #[test]
    fn connected_refuses_non_last_stage_without_real_stdout() {
        // rider 1 (per-channel coverage, `271:rul-only-oracle-bytes-ship`): a NON-LAST stage whose
        // predict does NOT produce real stdout (a `printf`-assert or a `>/dev/null`-decline — the arm
        // declines the very channel `grep` consumes) REFUSES the whole compound. Every stage becomes
        // an orphan ⇒ runs (can't-say ⇒ run — no partial/mixed emission). The STRUCTURAL pin that the
        // coverage rule bites: the FIRST stage `otelcol` declines stdout, so nothing ships.
        let src = "otelcol --version | grep -q x || curl y\n";
        let (ast, cfg, value, classes, i) = pipe_fixture(src, &["otelcol", "grep"]);
        let ship = |_n: CfgNodeId, p: Symbol, _a: &[Symbol]| {
            Some(StageShip {
                sh: "stub__predict() { :; }".to_owned(),
                // otelcol declines stdout (rc-only / redirect-void); grep is fine — but grep is LAST.
                produces_real_stdout: i.resolve(p) != "otelcol",
            })
        };
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship);
        let a = pipe_node(&cfg, &ast, "otelcol");
        let f = pipe_node(&cfg, &ast, "grep");
        assert!(
            c.governing_composed(f).is_none(),
            "an uncovered non-last stage refuses the whole compound (no composed probe ships)"
        );
        assert!(
            c.is_orphan_stage(a) && c.is_orphan_stage(f),
            "both stages become orphans ⇒ the pipe runs (can't-say ⇒ run)"
        );
        assert!(
            c.member_governor(a).is_none(),
            "no subsumed members on refusal"
        );
    }

    #[test]
    fn connected_refuses_when_a_stage_predict_does_not_resolve() {
        // A stage whose predict does not resolve (`ship_stage` ⇒ `None` — un-oracled, ⊤ argv) refuses
        // the compound: only oracle-authored bytes may ship, so a stage we cannot model-substitute
        // sinks the whole pipe to run. Here `grep` (the governor) has no shippable predict.
        let src = "otelcol --version | grep -q x\n";
        let (ast, cfg, value, classes, i) = pipe_fixture(src, &["otelcol", "grep"]);
        let ship = |_n: CfgNodeId, p: Symbol, _a: &[Symbol]| {
            (i.resolve(p) != "grep").then(|| StageShip {
                sh: "stub__predict() { :; }".to_owned(),
                produces_real_stdout: true,
            })
        };
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship);
        assert!(
            c.governing_composed(pipe_node(&cfg, &ast, "grep"))
                .is_none()
                && c.is_orphan_stage(pipe_node(&cfg, &ast, "otelcol")),
            "an unshippable stage refuses the compound; all stages run"
        );
    }

    #[test]
    fn connected_rejects_unvouched_middle_stage_as_orphans() {
        // The NEGATIVE CONTROL (silence-is-wall): an unvouched (non-Query) MIDDLE stage `cat`
        // disqualifies the pipe (NARROW FIRST). The Query stages become ORPHANS — they ship no
        // context-free probe and RUN; nothing is a governor/member.
        let src = "otelcol --version | cat | grep -q x || curl y\n";
        let (ast, cfg, value, classes, _i) = pipe_fixture(src, &["otelcol", "grep"]); // `cat` NOT a query
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship_all_real);
        let a = pipe_node(&cfg, &ast, "otelcol");
        let f = pipe_node(&cfg, &ast, "grep");
        assert!(
            c.is_orphan_stage(a) && c.is_orphan_stage(f),
            "both Query stages are orphans"
        );
        assert!(
            c.governing_composed(f).is_none(),
            "no governor — the pipe is not connected"
        );
        assert!(
            c.member_governor(a).is_none(),
            "no members — the pipe is not connected"
        );
    }

    #[test]
    fn connected_rejects_redirected_stage() {
        // A redirection on a stage ⊤s the pipe to the wall floor (24J narrow-first): NOT connected,
        // the Query stages are orphans. (`> /dev/null` on the first stage makes its Simple carry a
        // redirect ⇒ `stage_leaf` refuses it.)
        let src = "otelcol --version >/dev/null | grep -q x\n";
        let (ast, cfg, value, classes, _i) = pipe_fixture(src, &["otelcol", "grep"]);
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship_all_real);
        assert!(
            c.governing_composed(pipe_node(&cfg, &ast, "grep"))
                .is_none(),
            "a redirect on any stage disqualifies the connected pipe"
        );
        assert!(
            c.is_orphan_stage(pipe_node(&cfg, &ast, "grep")),
            "the last-stage Query is an orphan"
        );
    }

    #[test]
    fn connected_ignores_non_pipeline() {
        // A single command (no pipe) is never a connected check-pipe — nothing recorded.
        let src = "grep -q x || curl y\n";
        let (ast, cfg, value, classes, _i) = pipe_fixture(src, &["grep"]);
        let c = connected_check_pipes(&ast, &cfg, &value, &classes, ship_all_real);
        let g = pipe_node(&cfg, &ast, "grep");
        assert!(
            c.governing_composed(g).is_none()
                && !c.is_orphan_stage(g)
                && c.member_governor(g).is_none(),
            "a lone command is neither governor, member, nor orphan"
        );
    }

    #[test]
    fn composed_probe_renders_predicts_never_raw_book_bytes() {
        // The STRUCTURAL no-book-bytes pin (`271:rul-only-oracle-bytes-ship`, the cardinal probe-lane
        // law): a shipped connected probe carries ONLY oracle-authored bytes. Render a ComposedProbe
        // built from two stripped predicts and assert the artifact ships the COMPOSED invocation
        // (`otelcol__predict '--version' | grep__predict '-q' '0.155.0'`) and contains NO distinctive
        // RAW book spelling. The raw book pipe would ship `--version | grep -q` (bare `grep`, book
        // spacing); the composed form never does — this test FAILS if raw-shipping ever returns.
        let mut i = Interner::default();
        let fact = FactKey {
            kind: KindId(i.intern("grepmatch")),
            entity: EntityRef::Operand(OpaqueToken(i.intern("x"))),
            selector: SelectorId(i.intern("matched")),
            context: dorc_core::Context::HostDefault,
        };
        let otelcol = i.intern("otelcol");
        let grep = i.intern("grep");
        let version = i.intern("--version");
        let dashq = i.intern("-q");
        let pat = i.intern("0.155.0");
        let plan = ProbePlan {
            checks: vec![ProbePredict {
                site: LeafId(1),
                member: None,
                fact,
                site_kind: ProbeSiteKind::Query { valid: true },
                provider: grep,
                argv: Vec::new(),
                sh: String::new(),
                defining_span: None,
                connected: Some(ComposedProbe {
                    stages: vec![
                        ComposedStage {
                            provider: otelcol,
                            argv: vec![version],
                            sh: "otelcol__predict() { case $1 in --version) otelcol --version ;; esac; }".to_owned(),
                        },
                        ComposedStage {
                            provider: grep,
                            argv: vec![dashq, pat],
                            sh: "grep__predict() { pat=\"$1\"; grep -q -- \"$pat\"; }".to_owned(),
                        },
                    ],
                }),
                verdict: false,
                emits_report: false,
                entry: None,
            }],
            unresolvable: Vec::new(),
            unresolvable_causes: BTreeMap::new(),
        };
        let rendered = plan.render_sh(&records::Framing::spike(String::new()), &i);
        assert!(
            rendered.contains("otelcol__predict '--version' | grep__predict '-q' '0.155.0'"),
            "the composed predict invocation ships (oracle bytes, admin argv as arguments): {rendered}"
        );
        // The distinctive RAW book spelling `--version | grep -q` (bare grep, book spacing) must NOT
        // appear anywhere — its presence is the raw-ship debt returning (book bytes in the probe).
        assert!(
            !rendered.contains("--version | grep -q"),
            "NO raw book pipeline bytes may appear in a shipped connected probe: {rendered}"
        );
    }

    /// An auto-cell probe whose shipped verdict body EMITS report lines (`emits_report: true`).
    fn auto_cell_check(i: &mut Interner, emits_report: bool) -> ProbePredict {
        let fact = FactKey {
            kind: KindId(i.intern("dorc-auto")),
            entity: EntityRef::Operand(OpaqueToken(i.intern("vm.drop_caches"))),
            selector: SelectorId(i.intern("converged")),
            context: dorc_core::Context::HostDefault,
        };
        let provider = i.intern("sysctl");
        let key = i.intern("vm.drop_caches");
        ProbePredict {
            site: LeafId(0),
            member: None,
            fact,
            site_kind: ProbeSiteKind::Establish,
            provider,
            argv: vec![key],
            sh: "sysctl__is_converged() { printf \"$fmt\" \"$1\" >>\"${DREP_V1:-/dev/null}\"; \
                 return 2; }"
                .to_owned(),
            defining_span: None,
            connected: None,
            verdict: true,
            emits_report,
            entry: None,
        }
    }

    /// Every redirect target in `rendered`, quote-tracked, comment lines skipped. A SCAN, not a
    /// search for known-bad strings: a denylist forbids only the shapes someone already thought
    /// of, and this pin has to survive future additions to the lane.
    fn redirect_targets(rendered: &str) -> Vec<String> {
        let mut targets = Vec::new();
        for line in rendered.lines() {
            if line.trim_start().starts_with('#') {
                continue;
            }
            let chars: Vec<char> = line.chars().collect();
            let (mut in_s, mut in_d) = (false, false);
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];
                if c == '\'' && !in_d {
                    in_s = !in_s;
                } else if c == '"' && !in_s {
                    in_d = !in_d;
                } else if (c == '>' || c == '<') && !in_s && !in_d {
                    let mut j = i + 1;
                    if chars.get(j) == Some(&c) {
                        j += 1;
                    }
                    while matches!(chars.get(j), Some(' ' | '\t')) {
                        j += 1;
                    }
                    let mut word = String::new();
                    while let Some(&cc) = chars.get(j) {
                        if matches!(cc, ' ' | '\t' | ';' | '|' | '(' | ')' | '>' | '<')
                            || (cc == '&' && !word.is_empty())
                        {
                            break;
                        }
                        word.push(cc);
                        j += 1;
                    }
                    if !word.is_empty() {
                        targets.push(word);
                    }
                    i = j;
                    continue;
                }
                i += 1;
            }
        }
        targets
    }

    /// `rul-probe-writes-only-what-it-owns` — the structural net over the tier-3 report lane. The
    /// probe's pathname operations (create, truncate, read back, unlink, remove) are legitimate
    /// ONLY inside a container it exclusively created this run, so this asserts the ownership
    /// SHAPE rather than forbidding the vocabulary: one exclusive create, degrading on failure;
    /// every sink binding inside that container or `/dev/null`; the non-cascading cleanup pair; no
    /// host environment siting the scratch; and no scanned redirect escaping the owned set.
    #[test]
    fn emitting_auto_cell_owns_every_path_it_writes() {
        let mut i = Interner::default();
        let plan = ProbePlan {
            checks: vec![auto_cell_check(&mut i, true)],
            unresolvable: Vec::new(),
            unresolvable_causes: BTreeMap::new(),
        };
        let rendered = plan.render_sh(&records::Framing::spike(String::new()), &i);

        assert_eq!(
            rendered.matches("mkdir -m 700 ").count(),
            1,
            "the exclusive create happens ONCE per artifact — a per-site mkdir would be creating \
             inside a parent nobody proved was ours: {rendered}"
        );
        assert!(
            rendered.contains("|| _dsc=\n"),
            "a failed exclusive create must EMPTY the guard variable (the degradation signal every \
             later site reads); never retry, never fall back to a second name: {rendered}"
        );

        let bindings: Vec<&str> = rendered
            .match_indices("DREP_V1=")
            .map(|(at, m)| &rendered[at + m.len()..])
            .collect();
        assert_eq!(
            bindings.len(),
            2,
            "one drained site binds the sink exactly twice (owned file, degraded /dev/null): {rendered}"
        );
        for tail in &bindings {
            assert!(
                tail.starts_with("\"$_dsc/") || tail.starts_with("/dev/null"),
                "a sink binding is rooted in the owned scratch or is the inert sink, nothing else: {rendered}"
            );
        }

        assert!(
            rendered.contains("rmdir ") && rendered.contains("rm -f "),
            "cleanup is the non-cascading pair (per-file unlink inside the owned dir, then an \
             empty-only rmdir): {rendered}"
        );

        for hostile in ["TMPDIR", "HOME", "XDG_", "rm -rf"] {
            assert!(
                !rendered.contains(hostile),
                "the scratch root is a controller literal and cleanup never recurses ({hostile}): {rendered}"
            );
        }

        for target in redirect_targets(&rendered) {
            let owned = target == "/dev/null"
                || target == "\"$DREP_V1\""
                || target == "\"${DREP_V1:-/dev/null}\""
                || target.starts_with("\"$_dsc/")
                || target.starts_with('&');
            assert!(
                owned,
                "redirect target {target} is outside the owned set (/dev/null, the engine-supplied \
                 sink value, or the exclusively-created scratch): {rendered}"
            );
        }

        assert!(
            rendered.contains("printf 'dorc site 0 effect=%s rc=%s @@dorc@@\\n' \"$_e\" \"$_rc\""),
            "the effect record still ships byte-for-byte (the drain is additive): {rendered}"
        );
    }

    #[test]
    fn nonemitting_auto_cell_ships_the_ordinary_scaffold() {
        let mut i = Interner::default();
        let plan = ProbePlan {
            checks: vec![auto_cell_check(&mut i, false)],
            unresolvable: Vec::new(),
            unresolvable_causes: BTreeMap::new(),
        };
        let rendered = plan.render_sh(&records::Framing::spike(String::new()), &i);
        for absent in [
            "_dsc",
            "mkdir",
            "rmdir",
            "rm -f",
            "DREP_V1=",
            "while IFS= read",
            "report site=",
        ] {
            assert!(
                !rendered.contains(absent),
                "a report-free probe carries no scratch plumbing at all ({absent}) — this is what \
                 keeps `empty-world-byte-identical` and holds golden churn to the drained cases: \
                 {rendered}"
            );
        }
        assert!(
            rendered.contains("printf 'dorc site 0 effect=%s rc=%s @@dorc@@\\n' \"$_e\" \"$_rc\""),
            "the ordinary effect record ships: {rendered}"
        );
    }

    #[test]
    fn entry_composed_probe_renders_enter_forms_never_raw_book_bytes() {
        // The STRUCTURAL no-book-bytes pin EXTENDED to ENTRY composition (`27C` §3 / `27N`;
        // `271:rul-only-oracle-bytes-ship`): a wrapped site's entry-composed probe carries ONLY
        // oracle-authored bytes — the wrapper's `__enter` funcdef + the inner oracle's check body,
        // invoked with the site's PEELED argv (`sudo__enter hork__is_converged 'install' 'frob'`).
        // The raw book site `sudo hork install frob` must NEVER appear — its presence is book bytes
        // crossing the wrapper boundary (the very hole entry composition closes).
        let mut i = Interner::default();
        let fact = FactKey {
            kind: KindId(i.intern("dorc-auto:hork")),
            entity: EntityRef::Singleton,
            selector: SelectorId(i.intern("converged")),
            context: dorc_core::Context::Wrapped(dorc_core::ContextKey(
                i.intern("user=M:root;fs-view=F;netns=F"),
            )),
        };
        let hork = i.intern("hork");
        let install = i.intern("install");
        let frob = i.intern("frob");
        let plan = ProbePlan {
            checks: vec![ProbePredict {
                site: LeafId(1),
                member: None,
                fact,
                site_kind: ProbeSiteKind::Establish,
                provider: hork,
                argv: Vec::new(),
                sh: String::new(),
                defining_span: None,
                connected: None,
                verdict: false,
                emits_report: false,
                entry: Some(EntryComposed {
                    enter_defs: vec![(
                        "sudo__enter".to_owned(),
                        "sudo__enter() { sudo -n \"$@\"; }".to_owned(),
                    )],
                    inner_fn: "hork__is_converged".to_owned(),
                    inner_sh:
                        "hork__is_converged() { case $1 in install) hork query \"$2\" ;; esac; }"
                            .to_owned(),
                    inner_argv: vec![install, frob],
                }),
            }],
            unresolvable: Vec::new(),
            unresolvable_causes: BTreeMap::new(),
        };
        let rendered = plan.render_sh(&records::Framing::spike(String::new()), &i);
        assert!(
            rendered.contains("sudo__enter hork__is_converged 'install' 'frob'"),
            "the entry-composed invocation ships (enter form + inner oracle bytes, admin argv as \
             arguments): {rendered}"
        );
        assert!(
            rendered.contains("sudo__enter() { sudo -n \"$@\"; }"),
            "the wrapper's __enter funcdef ships (oracle bytes): {rendered}"
        );
        // The raw book site bytes `sudo hork install frob` must NOT appear — book bytes never cross
        // the wrapper boundary (`271:rul-only-oracle-bytes-ship`; `rul-argv-flows-bytes-do-not`).
        assert!(
            !rendered.contains("sudo hork install frob"),
            "NO raw book site bytes may appear in a shipped entry-composed probe: {rendered}"
        );
    }

    /// Build a one-check `ProbePlan` whose sole check carries the given `entry` — the shared
    /// fixture for the `shim_files` battery (the shim set is keyed only off `entry`).
    fn probe_plan_with_entry(entry: Option<EntryComposed>) -> ProbePlan {
        let mut i = Interner::default();
        ProbePlan {
            checks: vec![ProbePredict {
                site: LeafId(1),
                member: None,
                fact: FactKey {
                    kind: KindId(i.intern("dorc-auto:hork")),
                    entity: EntityRef::Singleton,
                    selector: SelectorId(i.intern("converged")),
                    context: dorc_core::Context::HostDefault,
                },
                site_kind: ProbeSiteKind::Establish,
                provider: i.intern("hork"),
                argv: Vec::new(),
                sh: String::new(),
                defining_span: None,
                connected: None,
                verdict: false,
                emits_report: false,
                entry,
            }],
            unresolvable: Vec::new(),
            unresolvable_causes: BTreeMap::new(),
        }
    }

    #[test]
    fn shim_files_wrap_the_exec_d_inner_check_as_a_dispatch_script() {
        // A single-link `sudo__enter <inner>` needs exactly ONE shim — the inner check `sudo -n` execs
        // — its bytes being shebang + the oracle funcdef verbatim + argv-dispatch (oracle bytes only).
        let inner_sh = "hork__is_converged() {\n   :\n   case \"$1\" in\n   install) hork query \"$2\" ;;\n   *) return 2 ;;\n   esac\n}";
        let plan = probe_plan_with_entry(Some(EntryComposed {
            enter_defs: vec![(
                "sudo__enter".to_owned(),
                "sudo__enter() { sudo -n \"$@\"; }".to_owned(),
            )],
            inner_fn: "hork__is_converged".to_owned(),
            inner_sh: inner_sh.to_owned(),
            inner_argv: Vec::new(),
        }));
        let files = plan.shim_files();
        assert_eq!(
            files.keys().cloned().collect::<Vec<_>>(),
            vec!["hork__is_converged".to_owned()],
            "the exec'd guest (inner check) is the sole shim; the outermost enter form runs in-process"
        );
        assert_eq!(
            files["hork__is_converged"],
            format!("#!/bin/sh\n{inner_sh}\nhork__is_converged \"$@\"\n"),
            "shim = shebang + verbatim oracle funcdef + argv-forwarding dispatch"
        );
    }

    #[test]
    fn shim_files_empty_for_ambient_and_carry_no_exec_boundary() {
        // Ambient (`entry: None`) and CARRY (empty `enter_defs`, `27C` §4(a)) cross no exec boundary —
        // the inner check runs in-process ⇒ no shim ⇒ `empty-world-byte-identical`.
        assert!(
            probe_plan_with_entry(None).shim_files().is_empty(),
            "an ambient check materializes no shim"
        );
        let carry = probe_plan_with_entry(Some(EntryComposed {
            enter_defs: Vec::new(),
            inner_fn: "hork__is_converged".to_owned(),
            inner_sh: "hork__is_converged() { :; }".to_owned(),
            inner_argv: Vec::new(),
        }));
        assert!(
            carry.shim_files().is_empty(),
            "a Carry check (empty enter_defs) crosses no exec boundary ⇒ no shim"
        );
        assert!(
            ProbePlan::default().shim_files().is_empty(),
            "an empty probe materializes no shim"
        );
    }

    #[test]
    fn shim_files_materialize_every_exec_d_guest_of_a_multi_link_chain() {
        // A chain execs its guests transitively (sudo execs chroot, chroot execs pipx), so every
        // enter form AFTER the outermost + the inner check needs a shim; keys sorted (`inv-determinism`).
        let plan = probe_plan_with_entry(Some(EntryComposed {
            enter_defs: vec![
                (
                    "sudo__enter".to_owned(),
                    "sudo__enter() { sudo -n \"$@\"; }".to_owned(),
                ),
                (
                    "chroot__enter".to_owned(),
                    "chroot__enter() { chroot / \"$@\"; }".to_owned(),
                ),
            ],
            inner_fn: "pipx__is_converged".to_owned(),
            inner_sh: "pipx__is_converged() { :; }".to_owned(),
            inner_argv: Vec::new(),
        }));
        assert_eq!(
            plan.shim_files().keys().cloned().collect::<Vec<_>>(),
            vec!["chroot__enter".to_owned(), "pipx__is_converged".to_owned(),],
            "every exec'd guest (inner enter forms + inner check) is materialized; the outermost is not"
        );
    }
}
