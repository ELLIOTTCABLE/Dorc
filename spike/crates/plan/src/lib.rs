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
//! * **`inv-must-may` + the ambient gate**, enforced inside `prove_replaceable`:
//!   only a [`Grade::Must`] fact that `analysis` classified [`SkipClass::EstablishAmbient`]
//!   (no upstream same-run mutation reaches it — note 162 O-1) and that the host
//!   probe found `Converged` may be elided.
//!
//! Determinism (`inv-determinism`): a pure function of its inputs; the host
//! verdict is injected (the real host / `hostsim` is a later seam).

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

use dorc_analysis::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use dorc_analysis::effect::{FactKey, InlineSite, SkipClass};
use dorc_analysis::lattice::{May, Powerset};
use dorc_analysis::value::{ValueFlow, ValueOf};
use dorc_core::{
    AstId, ByVouch, Carrier, Channel, Dialect, EntityRef, FactBacking, Grade, Interner, KindId,
    Observable, Predicted, Rc, Rung, Symbol, Verdict,
};
use dorc_oracle::verdict::VERDICT_SUFFIX;
use dorc_syntax::ast::{Ast, NodeKind, RedirOp, RedirTarget};

mod fold;
pub use fold::{AbstractRc, FoldResult};

pub mod erasability;

pub mod records;

pub mod render;

/// The per-run PATH shim for `dorc-sh` (`274` §5): the pure model — host-independent shipped text
/// (`shim_script`), run-id-derived naming (`shim_dir_name`, no mktemp randomness), and the failure
/// lattice (`classify_shim_rc` / `smoke_degrades_session` — every shim/exec failure drains to the ≥2
/// sink ⇒ run; a failed preamble smoke degrades the session shimless). MODELS only — materialization
/// is the cli/hostsim I/O edge and probe-shipping is task-14-gated; the corpus stays byte-stable.
pub mod shim;

pub mod survival;
pub use survival::{
    Backing, CanonicalCoord, Crossing, DisjointOutcome, DisjointnessProof, EntityCoord, Footprint,
    FootprintOrigin, MayAliasReason, Resolution, Resolutions, SurvivalWitness, TrustedFootprints,
    disjoint,
};

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
    /// Convergence-elision: an `EstablishAmbient` mutator whose effect the host
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
    /// — each body Establish is `EstablishAmbient` + Converged (a body Kill/Opaque/⊤/written
    /// establish, or a non-Converged one, blocks the WHOLE call), Queries pass their gates, and
    /// the CALL site's own consumed channels are reproduced. The all-or-nothing CALL license
    /// (the Members precedent): the CALL leaf's span substitutes to `true`; one non-licensing
    /// body leaf ⇒ the call RUNS (the real body executes). No partial-body render (`i-3`).
    InlineCall,
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
    /// `analysis` classified this command [`SkipClass::EstablishAmbient`]: no
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
    /// `--trust-footprints`. `None` for every ordinary elision (pre-wall, or flag-off); `Some`
    /// names which walls it crossed + whose footprint licensed each. Attached post-mint by the
    /// wall walk ([`ReplaceLicense::with_survival`]); read ONLY by the why-lens render (never the
    /// artifact — rec-1). NOT a proof of adequacy (converged≠no-op stays the vouch's) — see
    /// [`survival`].
    pub survival: Option<SurvivalWitness>,
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
}

impl ReplaceLicense {
    /// Mint a license iff EVERY condition holds; otherwise `None` — the
    /// conservative *run-it* direction (note 165 L2 / `inv-must-may` / the ambient
    /// gate):
    ///
    /// 1. the command's effect is [`SkipClass::EstablishAmbient`] (classify proved
    ///    no upstream same-run mutation reaches it — else its resting state is
    ///    stale and the probe is not authoritative);
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
    ///      *declared/probe-sourced* rc relaxes it (`status == Predicted::Value(N)` ⇒ the
    ///      stand-in is `StandIn::from_rc(N)`, reproducing the exact status, so the branch
    ///      decides identically — the fold's declared-rc opt-in, `19A §5`). The render CAN
    ///      express this (operand+operator on one line; the fold + omit-safety gate
    ///      handle it). (`tc-mint`/`tc-reliability`: the rc is a *declared observable*,
    ///      not inferred; an un-declared rc on a non-conforming establish is an
    ///      oracle-quality defect — build-2's contract, `19C` strain-B.)
    ///    * Errexit (`set -e`)-consumed status is NOT special-cased (19A C-3, honored
    ///      round-20 / 205 §2): the cfg pass marks errexit-region commands (and `$?`
    ///      readers' predecessors) `StatusRelaxable`-consumed, so they ride the same
    ///      declared-rc-or-block rule above. Under fork-mutator-rc a mutator's rc is
    ///      always ⊤ ⇒ converged mutators under `set -e` run (the 206 §2 headline cost).
    ///
    /// Generic over the phase `P` (`inv-superposition`): the engine never bakes a
    /// phase; the caller argues it. `build_plan` passes the verdict's own provenance
    /// (`Probe`) and the leaf's observed rc.
    ///
    /// task-D2 dispatch: an [`SkipClass::EstablishAmbient`] takes the
    /// convergence-elision precondition above; a [`SkipClass::QueryResolvable`] takes
    /// the Query-guard path ([`prove_query_replaceable`](ReplaceLicense::prove_query_replaceable)).
    /// Any other class never licenses.
    ///
    /// **The elide-weld (24D §3 / rul24-vouch-is-verdict-authoring).** The full-skip (elide)
    /// license now DEMANDS the reached `vouch` — the SAME [`ByVouch<VerdictVouch>`] the guard mint
    /// consumes (TC-tier-2). It arrives as an `Option` (the caller's [`Vouches`] lookup); the
    /// `EstablishAmbient` arm consumes it BY VALUE, and that consumption IS the tier check — a
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
        class: &SkipClass,
        grade: Grade,
        verdict: PhasedVerdict<P>,
        consumed: May<Powerset<Channel>>,
        status: Predicted<Rc>,
        vouch: Option<ByVouch<VerdictVouch>>,
    ) -> Option<ReplaceLicense> {
        match class {
            SkipClass::EstablishAmbient(fact) => {
                // The elide-weld (TC-tier-2): consume the reached vouch BY VALUE — no vouch ⇒ run.
                // A `ByObservation`/`BySilence` cannot inhabit this `Option`, so a converged
                // measurement alone no longer elides (the vouchless-elide gap, closed).
                let _vouch: ByVouch<VerdictVouch> = vouch?;
                if grade != Grade::Must {
                    return None;
                }
                if verdict.resolve() != Resolved::Replaceable {
                    return None;
                }
                consumption_ok(&consumed, status).then_some(ReplaceLicense {
                    fact: *fact,
                    derivation: Derivation {
                        fact: *fact,
                        via: LicenseVia::ConvergedEstablish,
                        ambient: true,
                        grade,
                        verdict: Verdict::Converged,
                        // Empty at mint (the minter has no arena); `build_plan` attaches the
                        // real witness post-mint via `with_witness` (arch-1, output-only/exempt).
                        witness: dorc_core::Witness::empty(),
                        survival: None,
                    },
                })
            }
            SkipClass::QueryResolvable { fact, valid } => {
                Self::prove_query_replaceable(*fact, *valid, verdict.raw(), &consumed, status)
            }
            _ => None,
        }
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
    fn prove_query_replaceable(
        fact: FactKey,
        valid: bool,
        verdict: Verdict,
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        if !valid {
            return None;
        }
        // The guard needs a concrete probe-sourced rc to reproduce — a ⊤ status forbids
        // the mint (`inv-probe-sourced-values`: never fabricate rc-0). This also covers
        // the "branch-decision fully resolved" gate (Build 5): a known rc is exactly
        // what lets the fold resolve the `&&`/`||` AND lets the stand-in reproduce it.
        if matches!(status, Predicted::Top) {
            return None;
        }
        consumption_ok(consumed, status).then_some(ReplaceLicense {
            fact,
            derivation: Derivation {
                fact,
                via: LicenseVia::QueryGuard,
                ambient: false,
                grade: Grade::Must,
                verdict,
                witness: dorc_core::Witness::empty(),
                survival: None,
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
        members: &[FactKey],
        member_verdicts: &[Verdict],
        self_reached: bool,
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        let representative = *members.first()?;
        if !self_reached {
            return None;
        }
        // (a) all members Converged — a non-Converged (Diverged/Unknown) member refuses.
        if member_verdicts.is_empty() || !member_verdicts.iter().all(|v| *v == Verdict::Converged) {
            return None;
        }
        // (c) the consumption gates (the in-loop leaf's status is ⊤ for a mutator —
        // fork-mutator-rc — so a consumed status blocks; stdout/stderr/render-floor block).
        consumption_ok(consumed, status).then_some(ReplaceLicense {
            fact: representative,
            derivation: Derivation {
                fact: representative,
                via: LicenseVia::MembersLoop,
                ambient: true,
                grade: Grade::Must,
                verdict: Verdict::Converged,
                witness: dorc_core::Witness::empty(),
                survival: None,
            },
        })
    }

    /// Mint a license for an inlined function-CALL's convergence-elision (arch-2, brk-2,
    /// `i-3`) — the all-or-nothing CALL license (the Members precedent, 20S). The call's
    /// command word resolved to a same-file-earlier funcdef and its body was spliced; this
    /// mints a [`LicenseVia::InlineCall`] `Replace` (the CALL span → `true`) iff EVERY
    /// effect-bearing body leaf licenses elision, every ambiguity ⇒ REFUSE:
    ///
    /// * every body site that is an `EstablishAmbient` has a Converged fact (`observe`(fact)
    ///   reports the Effect channel; a single non-Converged ⇒ refuse — the whole call runs);
    /// * NO body site is a blocker — an `EstablishWritten` (stale resting probe), a `MustRun`
    ///   (a body Kill, an Opaque/⊤ command, a multi-cell verb, an unreachable establish), an
    ///   in-loop `EstablishMembers` (an in-loop call body — out of slice), or a nested
    ///   `InlineCall` (defensive — transitive inlines are flattened to leaves, so one should
    ///   never appear here);
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
        observe: &impl Fn(FactKey) -> Observable,
        consumed: &May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        let mut representative: Option<FactKey> = None;
        for site in sites {
            match &site.class {
                SkipClass::EstablishAmbient(f) => {
                    if observe(*f).effect != Verdict::Converged {
                        return None; // a non-converged body establish ⇒ the whole call runs
                    }
                    representative.get_or_insert(*f);
                }
                // A read-only Query guard never blocks (the wrapper-pun's `dpkg -s "$1"`); its
                // own convergence does not gate the call's elision.
                SkipClass::QueryResolvable { .. } => {}
                // Every other class blocks the whole call (all-or-nothing): a written establish
                // (stale probe), a MustRun (Kill/Opaque/⊤), an in-loop Members body, or a
                // nested InlineCall (defensive — should be flattened).
                SkipClass::EstablishWritten(_)
                | SkipClass::MustRun
                | SkipClass::EstablishMembers { .. }
                | SkipClass::InlineCall { .. } => return None,
            }
        }
        // A call with NO converged establish to elide runs (the run-it floor): there is no
        // mutation to be already-done, and eliding a pure-builtin wrapper would suppress its
        // observable (an `echo`'s stdout) for no gain.
        let fact = representative?;
        // The CALL site's own consumed channels (the aggregate rc is ⊤ — a mutator-shaped
        // call, fork-mutator-rc — so a consumed status blocks; door-3 `|| true` does not).
        if !consumption_ok(consumed, status) {
            return None;
        }
        Some(ReplaceLicense {
            fact,
            derivation: Derivation {
                fact,
                via: LicenseVia::InlineCall,
                ambient: true,
                grade: Grade::Must,
                verdict: Verdict::Converged,
                witness: dorc_core::Witness::empty(),
                survival: None,
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
    /// provenance set AFTER the mint (the survival decision happens in the wall walk, downstream
    /// of the license mint), so it never influences whether the license was granted. Rides the
    /// render surface (the why-lens) only — never the byte-floored artifact (rec-1).
    #[must_use]
    pub fn with_survival(mut self, witness: SurvivalWitness) -> Self {
        self.derivation.survival = Some(witness);
        self
    }

    /// The fact whose established-ness licensed this skip.
    #[must_use]
    pub fn fact(&self) -> FactKey {
        self.fact
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
/// re-exported here: the round-22 structured diagnostic ([`dorc_core::diag::SiteId`])
/// keys on it, so the base crate owns it and `plan` shares the one type rather than a
/// parallel one (`inv-site-keyed-results`).
pub use dorc_core::LeafId;

/// The cheapest sh stand-in that reproduces a leaf's **exact** observed exit status
/// (`19A §5` observable-value-MAINTAINING substitution / DESIGN `16F`/`16P-T10`).
/// NOT always `:`: the value the downstream fold/guard reads must be preserved, so a
/// converged non-conforming establish (`useradd`, rc 9) becomes `(exit 9)`, never
/// `true` — else its rc-0 stub would suppress a `|| fallback` (the `kFAIL-perform`
/// under-execute the round-19 adversarial pass proved).
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
}

impl VerdictVouch {
    /// Build a vouch descriptor from the cli-resolved verdict-function data (the sole constructor;
    /// the cli edge owns the lift + strip + argv-render). `fn_name`/`invocation` are the mangled
    /// name and the full invocation; `preamble` is the stripped body; `kind_label` the fact's kind
    /// for attribution; `check_cmds` the verdict body's own command names (gate-6 attribution).
    #[must_use]
    pub fn new(
        fn_name: String,
        preamble: String,
        invocation: String,
        kind_label: String,
        check_cmds: Vec<String>,
    ) -> Self {
        Self {
            fn_name,
            preamble,
            invocation,
            kind_label,
            check_cmds,
        }
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

    /// The guard preamble def to ship (deduped per [`fn_name`](Self::fn_name)).
    #[must_use]
    pub fn preamble(&self) -> &str {
        &self.vouch.preamble
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
    fn render_line(&self, original: &str) -> String {
        let check = format!("( {} )", self.vouch.invocation);
        format!(
            "{check} || {original}   # dorc: guard [{} converged-vouch; probe: {}]",
            self.vouch.kind_label,
            self.probe_word(),
        )
    }
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
}

impl GuardLicense {
    /// Mint a guard iff the plan-time probe [`Verdict`] is [`Verdict::Converged`] (jc-mint-policy
    /// m-a: converged-past-wall ONLY — a guard at a predicted-change site buys nothing, flagship
    /// site 3). CONSUMES the [`ByVouch<VerdictVouch>`] by value (TC-tier-2: a [`core::claim::ByObservation`]
    /// or a silence claim does not satisfy this signature). A diverged/unknown verdict ⇒ `None` ⇒
    /// the site runs (`inv-kfail`).
    #[must_use]
    pub fn mint(
        fact: FactKey,
        vouch: ByVouch<VerdictVouch>,
        probe_verdict: Verdict,
    ) -> Option<GuardLicense> {
        if probe_verdict != Verdict::Converged {
            return None;
        }
        Some(GuardLicense {
            fact,
            insert: GuardInsert {
                vouch: vouch.into_vouch(),
                probe_verdict,
            },
        })
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
/// `--trust-footprints`-gated, which governs only the survival tier).
pub type Vouches = BTreeMap<CfgNodeId, ByVouch<VerdictVouch>>;

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
#[must_use]
pub fn build_vouches(
    oracle_srcs: &[&str],
    classes: &[(CfgNodeId, SkipClass)],
    value: &ValueFlow,
    interner: &mut Interner,
) -> Carrier<Vouches> {
    use dorc_oracle::predict::{map_provider_name, strip_verdict};
    use dorc_oracle::verdict::{VerdictResolution, VerdictSet, check_commands, evaluate_verdict};

    let mut diags = Vec::new();
    let verdict_sets: Vec<VerdictSet> = oracle_srcs
        .iter()
        .map(|src| {
            let lifted = VerdictSet::lift(interner, src);
            diags.extend(lifted.diags);
            lifted.value
        })
        .collect();

    let mut vouches = Vouches::new();
    for (node, class) in classes {
        // A vouch is consumed only at an establish-bearing site (elide: EstablishAmbient; guard:
        // EstablishWritten). Computing both is future-proof and inert where unused.
        let fact = match class {
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => *f,
            _ => continue,
        };
        // Resolve the site's argv → (provider, operands), all literal — a ⊤ word ⇒ no vouch.
        let argv = value.argv_values(*node);
        let Some((first, rest)) = argv.split_first() else {
            continue;
        };
        let ValueOf::Literal(provider) = first else {
            continue;
        };
        let mut op_texts = Vec::with_capacity(rest.len());
        let mut has_top = false;
        for w in rest {
            match w {
                ValueOf::Literal(s) => op_texts.push(interner.resolve(*s).to_owned()),
                ValueOf::Top(_) => {
                    has_top = true;
                    break;
                }
            }
        }
        if has_top {
            continue;
        }
        let op_refs: Vec<&str> = op_texts.iter().map(String::as_str).collect();

        // Find the provider's verdict funcdef (shared hyphen↔underscore convention) and trace it.
        let want = map_provider_name(interner.resolve(*provider));
        let found = verdict_sets.iter().zip(oracle_srcs).find_map(|(set, src)| {
            set.providers()
                .find(|p| map_provider_name(interner.resolve(*p)) == want)
                .and_then(|p| set.get(p))
                .map(|verdict| (*src, verdict))
        });
        let Some((src, verdict)) = found else {
            continue;
        };
        // The reached-path license (rul-guard-license): ONLY a Vouched resolution mints. A Declined
        // (unhandled path / an inert builtin / a non-converged `return` — hz-refusepath) or ⊤ ⇒ no
        // vouch ⇒ run.
        if !matches!(
            evaluate_verdict(verdict, &op_refs),
            VerdictResolution::Vouched
        ) {
            continue;
        }

        let fn_name = format!(
            "{}{VERDICT_SUFFIX}",
            dorc_oracle::to_funcname_segment(interner.resolve(verdict.provider)),
        );
        let preamble = strip_verdict(src, verdict, interner);
        let invocation = if op_refs.is_empty() {
            fn_name.clone()
        } else {
            format!("{fn_name} {}", op_refs.join(" "))
        };
        let kind_label = interner.resolve(fact.kind.0).to_owned();
        let check_cmds = check_commands(verdict);
        let vouch = VerdictVouch::new(fn_name, preamble, invocation, kind_label, check_cmds);
        vouches.insert(*node, ByVouch::vouched(vouch, Rung::Both));
    }
    Carrier::new(vouches, diags)
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
#[must_use]
pub fn build_wrapped_vouches(
    oracle_srcs: &[&str],
    classes: &[(CfgNodeId, SkipClass)],
    wrapped: &WrappedProbes,
    interner: &mut Interner,
) -> Vouches {
    use dorc_oracle::verdict::{VerdictResolution, VerdictSet, check_commands, evaluate_verdict};

    let verdict_sets: Vec<VerdictSet> = oracle_srcs
        .iter()
        .map(|src| VerdictSet::lift(interner, src).value)
        .collect();
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
            SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) if n == node => {
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
        let inner_verdict = verdict_sets.iter().find_map(|set| set.get(*provider));
        let Some(verdict) = inner_verdict else {
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
        let vouch = VerdictVouch::new(
            composed.inner_fn.clone(),
            preamble,
            invocation.join(" "),
            interner.resolve(fact.kind.0).to_owned(),
            check_commands(verdict),
        );
        vouches.insert(*node, ByVouch::vouched(vouch, Rung::Both));
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
}

/// A whole-book plan: an ordered list of leaf [`Step`]s (the leaf-seam — never a
/// single opaque script). Render with [`render_sh`](Plan::render_sh). Carries the survival-tier
/// [`SurvivalReport`] (24F §3a instrumentation — the may-alias fire-rate; digest-exempt).
#[derive(Debug, Clone)]
pub struct Plan {
    pub steps: Vec<Step>,
    /// The survival-tier instrumentation (24F §3a). Empty on the flag-off / no-resolver path.
    pub survival_report: SurvivalReport,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DispositionCounts {
    pub sites: usize,
    pub elide: usize,
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
/// A site is **resolvable** iff its class is [`SkipClass::EstablishAmbient`] (the
/// elidable establish — note 162 O-1) OR [`SkipClass::QueryResolvable`] (a read-only
/// guard whose check IS the probe — 202 §2 / task-D2), AND its kind has a *declared*
/// read-only probe; only resolvable sites get an invocation. An un-resolvable site (a
/// kill, an opaque command, a written establish, a `MustRun`, or a resolvable class whose
/// kind has no probe) appears in the rendered artifact as a `site:<id>
/// skip-unresolvable` comment, never as an invocation (`kFAIL-perform`: no convergence
/// knowledge ⇒ the apply runs it).
#[derive(Debug, Clone, Default)]
pub struct ProbePlan {
    /// The resolvable sites' checks, in site-id order.
    pub checks: Vec<ProbePredict>,
    /// The un-resolvable sites' ids (rendered as `skip-unresolvable` comments).
    pub unresolvable: Vec<LeafId>,
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
        let nonce = &framing.nonce;
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
            out.push_str(&render::probe::record_scaffold(&invocation, &key, nonce));
        }
        // Un-resolvable sites are recorded as comments (never invoked): transparency
        // for the human reading the artifact and the D3 argv-echo differential.
        for site in &self.unresolvable {
            out.push_str(&render::probe::unresolvable_comment(*site));
        }
        out
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

/// A ship decision for one escalated derivation site (24E §2): the stripped `<provider>__touches`
/// funcdef + the host tool the body reached (display locus). Returned by the cli's derive-closure,
/// which owns the oracle sources + the `evaluate_touches` escalation check — so `plan` stays
/// oracle-free (the same seam-shape as [`compile_probe`]'s `ship_body`).
#[derive(Debug, Clone)]
pub struct DerivationShip {
    /// The stripped `<provider>__touches` funcdef (strip-only; `dorc_oracle::predict::strip_touches`).
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
    /// The book command word (argv[0]) whose `__touches` this ships.
    pub provider: Symbol,
    /// The site's argv after the command word (F-quoted at render).
    pub argv: Vec<Symbol>,
    /// The stripped `<provider>__touches` funcdef.
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
    /// The per-arm wrapper funcname (`package__reaches_1`) — the shipped def + invocation agree.
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

/// The `<provider>__touches` derivation funcname (24E §2/§9), mangled IDENTICALLY to
/// [`predict_fn_name`] so a site's shipped def (via `strip_touches`) and its invocation agree
/// byte-for-byte. Referent-agnostic: passed to the host, never branched on.
fn touches_fn_name(interner: &Interner, provider: Symbol) -> String {
    format!(
        "{}__touches",
        dorc_oracle::to_funcname_segment(&dorc_oracle::predict::map_provider_name(
            interner.resolve(provider)
        )),
    )
}

impl DerivationPlan {
    /// Render the derivation-probe as read-only, self-reporting sh, APPENDED to the convergence
    /// probe in the SAME phase-1 block (no shebang — the e2e shebang-split keeps it in phase-1).
    /// Each provider's stripped `<provider>__touches` funcdef is emitted once (deduped, re-emitted
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
pub fn compile_derivations(
    ast: &Ast,
    cfg: &Cfg,
    value: &ValueFlow,
    classes: &[(CfgNodeId, SkipClass)],
    kills: &BTreeSet<CfgNodeId>,
    derive_body: impl Fn(Symbol, &[Symbol]) -> Option<DerivationShip>,
) -> DerivationPlan {
    let mut derivations = Vec::new();
    for (site, node, class) in site_order(ast, cfg, classes) {
        let is_wall_candidate = matches!(
            class,
            SkipClass::EstablishAmbient(_) | SkipClass::EstablishWritten(_)
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
        let Some(ship) = derive_body(*provider, &operands) else {
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
fn site_order<'a>(
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
    ship_stage: impl Fn(Symbol, &[Symbol]) -> Option<StageShip>,
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
                ship_stage_for_argv(&value.argv_values(stage_node), &ship_stage)
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
    ship_stage: &impl Fn(Symbol, &[Symbol]) -> Option<StageShip>,
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
    let ship = ship_stage(provider, &operands)?;
    Some((provider, operands, ship))
}

/// Compile the probe from the analysis result, keyed by command **site**
/// (`inv-site-keyed-results`): each [`SkipClass::EstablishAmbient`] / resolvable-Query
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
/// [`SkipClass::EstablishWritten`] (an opaque upstream poisoned its resting probe), so at HEAD it
/// ships NO probe — but a guard needs its probe-verdict (the witness's probe half; plan-prediction
/// and apply-guard run the same check, 233 §guard-license). So a `EstablishWritten` site the cli
/// reports VOUCHED (its provider authored a verdict function reaching a vouching path) DOES ship
/// its read-only Establish probe. An unvouched `EstablishWritten` stays unresolvable (jc-probe-
/// scope: whether unvouched walled sites ship hint-probes is deliberately OPEN).
///
/// `connected` ([`connected_check_pipes`], 24J §2) re-routes a recognised connected check-pipe: the
/// GOVERNING (last) stage ships ONE *connected* probe (the raw pipeline bytes — "the host runs the
/// real `A | F`"); every non-last MEMBER is SUBSUMED (ships no separate record — a lone `grep -q`
/// has no independent fact, silence-is-wall). Off the connected path (`ConnectedPipes::default()`)
/// this is byte-identical to before.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "the probe compiler threads the whole compiled context (ast/cfg/value/classes/connected) \
              plus the `27N` wrapped-site decisions plus THREE ship seams — the predict body, the \
              `24L` §2 auto-cell verdict body, and the vouch predicate; each is a distinct \
              caller-supplied input. The auto-cell ship arm pushes the body just over the line cap; \
              the per-class dispatch is irreducibly flat"
)]
pub fn compile_probe(
    ast: &Ast,
    cfg: &Cfg,
    value: &ValueFlow,
    classes: &[(CfgNodeId, SkipClass)],
    wrapped: &WrappedProbes,
    connected: &ConnectedPipes,
    ship_body: impl Fn(Symbol, &[Symbol]) -> Option<String>,
    ship_auto: impl Fn(FactKey, Symbol, &[Symbol]) -> Option<String>,
    is_vouched: impl Fn(CfgNodeId) -> bool,
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
                SkipClass::EstablishAmbient(f)
                | SkipClass::EstablishWritten(f)
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
                        connected: None,
                        verdict: false,
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
            push_member_predicts(
                &mut checks,
                &mut unresolvable,
                site,
                node,
                members,
                value,
                &ship_body,
            );
            continue;
        }
        // arch-2 (i-4): an inlined CALL ships one `site N.M` check per spliced body establish
        // (see `push_inline_predicts` for the all-or-nothing probe-ability).
        if let SkipClass::InlineCall { sites } = class {
            push_inline_predicts(
                &mut checks,
                &mut unresolvable,
                site,
                sites,
                value,
                &ship_body,
            );
            continue;
        }
        // Both an EstablishAmbient and a (resolvable) Query site ship a check — each is
        // probe-resolvable iff the provider's `<provider>__predict` resolves the site's argv
        // (R3 / 23D §1). The `site_kind` discriminant rides along so the cli's firewall
        // knows whether the record-rc is the probe command's (Establish ⇒ never fold) or
        // the guard's own (Query ⇒ fold iff valid). A written establish, an inverted
        // claim, opaque, pure, MustRun — none resolvable (`can't-probe ⇒ can't-elide`,
        // `kFAIL-perform`).
        let resolvable = match class {
            SkipClass::EstablishAmbient(fact) => Some((*fact, ProbeSiteKind::Establish)),
            // strain-classify-coupling (24C): a vouched past-wall establish still probes (the
            // guard witness needs the verdict). Establish-class ⇒ its record-rc is the probe
            // command's, never fed to the fold (the firewall is unmoved).
            SkipClass::EstablishWritten(fact) if is_vouched(node) => {
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
                    connected: Some(composed.clone()),
                    verdict: false,
                    entry: None,
                });
            } else {
                unresolvable.push(site);
            }
            continue;
        }
        // `24L` §2 — the typeless-floor auto-cell: the shipped probe is the STRIPPED VERDICT BODY
        // itself (a markless oracle has no `predict`), invoked with the site argv; its rc maps to
        // the Effect verdict through the record scaffold's existing rc-partition (0=holds, 1=absent,
        // else=cant-tell — the verdict rc-partition). `ship_auto` returns `Some` ONLY for an
        // auto-cell fact (the edge closure keys on the reserved auto-kind), so a `Some` here IS the
        // auto-cell signal. Establish-class only (a Query is never an auto-cell). GATED on the
        // vouch: the verdict IS the probe, so a DECLINED verdict (a refuse path — `return 2`, the
        // R2-MULTIOP arity gate) has nothing to measure and must not ship a record; the site runs
        // (`guard23-refusepath-rc0-never-passes`: a declined verdict never licenses, and never probes).
        if matches!(site_kind, ProbeSiteKind::Establish)
            && is_vouched(node)
            && let Some((provider, argv, sh)) =
                ship_auto_for_argv(&value.argv_values(node), fact, &ship_auto)
        {
            checks.push(ProbePredict {
                site,
                member: None,
                fact,
                site_kind,
                provider,
                argv,
                sh,
                connected: None,
                verdict: true,
                entry: None,
            });
            continue;
        }
        // R3: ship the provider's stripped `check()` invoked with the site's argv. A ⊤ command word or
        // operand, or no check resolving this argv, ⇒ un-shippable (no concrete invocation ⇒
        // `can't-probe ⇒ can't-elide`, `kFAIL-perform`).
        match ship_for_argv(&value.argv_values(node), &ship_body) {
            Some((provider, argv, sh)) => checks.push(ProbePredict {
                site,
                member: None,
                fact,
                site_kind,
                provider,
                argv,
                sh,
                connected: None,
                verdict: false,
                entry: None,
            }),
            None => unresolvable.push(site),
        }
    }
    ProbePlan {
        checks,
        unresolvable,
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
/// `24L` §2 — resolve the (provider, argv, stripped VERDICT body) an auto-cell site ships. Mirrors
/// [`ship_for_argv`] but hands the resolved `fact` to `ship_auto`, which returns the stripped
/// `<provider>__is_converged` body ONLY when `fact` is an auto-cell (the edge closure keys on the
/// reserved auto-kind). A ⊤ command word or operand ⇒ no concrete invocation ⇒ `None` (unshippable).
fn ship_auto_for_argv(
    argv: &[ValueOf],
    fact: FactKey,
    ship_auto: &impl Fn(FactKey, Symbol, &[Symbol]) -> Option<String>,
) -> Option<(Symbol, Vec<Symbol>, String)> {
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
    let sh = ship_auto(fact, provider, &operands)?;
    Some((provider, operands, sh))
}

fn ship_for_argv(
    argv: &[ValueOf],
    ship_body: &impl Fn(Symbol, &[Symbol]) -> Option<String>,
) -> Option<(Symbol, Vec<Symbol>, String)> {
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
    let sh = ship_body(provider, &operands)?;
    Some((provider, operands, sh))
}

/// Compile the per-member checks for an in-loop MEMBERS establish site (item-4): one
/// [`ProbePredict`] per member, each carrying its `member` index and per-member cell. ALL
/// members must have a declared probe body, or the WHOLE site is unresolvable — the
/// all-or-nothing in-loop license (item-3) cannot elide a partial-member set, so a
/// missing probe on any member kills the site (`can't-probe ⇒ can't-elide`). The records
/// these emit are sub-keyed `site <leafid>.<member-idx>` ([`ProbePredict::member`]).
fn push_member_predicts(
    checks: &mut Vec<ProbePredict>,
    unresolvable: &mut Vec<LeafId>,
    site: LeafId,
    node: CfgNodeId,
    members: &[FactKey],
    value: &ValueFlow,
    ship_body: &impl Fn(Symbol, &[Symbol]) -> Option<String>,
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
    let mut staged = Vec::with_capacity(members.len());
    for (idx, (fact, argv)) in members.iter().zip(member_argvs).enumerate() {
        let Some((provider, args, sh)) = ship_for_argv(argv, ship_body) else {
            // One member un-shippable ⇒ the whole site is unresolvable (all or none).
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
            sh,
            connected: None,
            verdict: false,
            entry: None,
        });
    }
    checks.extend(staged);
}

/// Compile the per-body-site checks for an inlined function-CALL (arch-2, brk-2, `i-4`): one
/// [`ProbePredict`] per effect-bearing/probeable spliced body site, each carrying its body-site
/// index as `member` (the `site N.M` sub-record, M = the index into the call's body-site list)
/// and the body site's resolved cell (positionals bound at the call, `i-2`). An `EstablishAmbient`
/// body site is an Establish-class record; a `QueryResolvable` body site is a Query-class record
/// (its rc is fold-usable per its `valid` bit, the wrapper-pun's `dpkg -s "$1"`); a Pure/MustRun/
/// Written body site ships nothing (not elision-gating).
///
/// ALL-OR-NOTHING on probe-ability (the call's all-or-nothing license cannot elide a partial
/// body): if any ESTABLISH body site has no declared probe body, the WHOLE call is unresolvable
/// (`can't-probe ⇒ can't-elide`). A Query body site with no probe body is NOT a blocker (it does
/// not gate the call's elision — the call elides on the body's establishes), so it is simply
/// omitted; the records are staged and committed only if no establish is un-probeable.
fn push_inline_predicts(
    checks: &mut Vec<ProbePredict>,
    unresolvable: &mut Vec<LeafId>,
    site: LeafId,
    sites: &[InlineSite],
    value: &ValueFlow,
    ship_body: &impl Fn(Symbol, &[Symbol]) -> Option<String>,
) {
    let mut staged = Vec::new();
    for (idx, body) in sites.iter().enumerate() {
        let member = Some(u32::try_from(idx).unwrap_or(u32::MAX));
        // The spliced body site's argv, resolved with the call's positionals bound (`i-2`;
        // [`ValueFlow::argv_values`] returns the positional-bound form for a body node).
        let body_argv = value.argv_values(body.node);
        match &body.class {
            SkipClass::EstablishAmbient(fact) => {
                let Some((provider, args, sh)) = ship_for_argv(&body_argv, ship_body) else {
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
                    sh,
                    connected: None,
                    verdict: false,
                    entry: None,
                });
            }
            SkipClass::QueryResolvable { fact, valid } => {
                // A read-only guard: ship its check if resolvable (it does NOT gate the call's
                // elision, so an un-shippable guard is simply omitted, never a blocker).
                if let Some((provider, args, sh)) = ship_for_argv(&body_argv, ship_body) {
                    staged.push(ProbePredict {
                        site,
                        member,
                        fact: *fact,
                        site_kind: ProbeSiteKind::Query { valid: *valid },
                        provider,
                        argv: args,
                        sh,
                        connected: None,
                        verdict: false,
                        entry: None,
                    });
                }
            }
            // Not elision-gating ⇒ no record.
            SkipClass::EstablishWritten(_)
            | SkipClass::MustRun
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
/// elision gate) *and* the concrete observed exit status (the fold + value-preserving
/// substitution input, `19A §5` / `19B` build-1). `build_plan` is a pure function of
/// its inputs (deterministic given a deterministic `observe`).
///
/// Two collapses, both apply-phase (`inv-superposition` — the caller argues the
/// phase; the engine never bakes it):
/// 1. **convergence-elision** (the existing path): an `EstablishAmbient` + `Must` +
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
pub fn build_plan(
    src: &str,
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    vouches: &Vouches,
    observe: impl Fn(FactKey) -> Observable,
    arena: &mut dorc_core::ProvArena,
) -> Plan {
    build_plan_walled(
        src,
        ast,
        cfg,
        classes,
        &BTreeSet::new(),
        None,
        None,
        &Dialect::empty(),
        // Survival is off (`None`) here, so the backing map is never consulted — the empty map is
        // the honest floor for this kill-unaware / flag-off entry.
        &BTreeMap::new(),
        vouches,
        &ConnectedPipes::default(),
        observe,
        arena,
    )
}

/// [`build_plan`] PLUS the **kill-node set** (R3 / 24A §3 — the kill gap). `kills` is the set
/// of leaf [`CfgNodeId`]s the analysis flagged `Kills` (`apt-get purge` — an `EstablishInverted`
/// claim ⇒ `CommandEffect::Kills` ⇒ classifies `MustRun`). A `MustRun` is opaque to the wall
/// predicate — a pure builtin, an opaque, and a kill all classify `MustRun` — but a kill is a
/// real mutator: a RUNNING kill may touch anything it did not declare (the frame problem, 233),
/// so it must WALL downstream different-cell converged establishes, exactly like a modeled
/// establish (the same under-execute shape fd10 closed). Pure builtins stay out of `kills` and
/// never wall (`exec-pure-builtin`); opaque handling is unchanged (an opaque already ⊤-poisons
/// downstream statically). Demotion stays Replace→Run only (`inv-kfail`). This is BASELINE
/// ground-truth behaviour — never flag-gated (rul24-mode-gate governs the survival tier, not
/// wall honesty). Deterministic (`kills` is a `BTreeSet`; `inv-determinism`).
///
/// # Survival tier (Stage 2 — the golden hill; mode-gate `survival`, TC-1)
///
/// `survival` is the mode-gate DATA (`--trust-footprints`): `None` ⇒ the honest Stage-1 wall
/// (a running mutator is a TOTAL wall — every downstream converged `Replace` demotes), the
/// byte-identical baseline. `Some(footprints)` ⇒ the frame-rule walk: a running mutator WITH a
/// lifted footprint scopes its wall (accumulates its coordinates) instead of totalising it, and
/// a downstream converged `Replace` SURVIVES (elides past the running wall) iff its backing is
/// disjoint from every accumulated footprint (`survival::wall_verdict`). A running mutator
/// WITHOUT a footprint (silence, a ⊤ lift, a refused coherence check) still totalises the wall.
/// Survival only ever *keeps* a `Replace`; demotion stays Replace→Run (`inv-kfail`). The
/// survival arm is structurally unreachable when `survival` is `None` — the footprints were
/// never lifted, so no maintainer can consult them unflagged (data-absence, not a checked bool).
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "the kernel entry threads the whole compiled context (src/ast/cfg/classes/kills/survival/fact-backings/observe/arena); each is a distinct input, not a bundle-able struct — widening it once here is clearer than a params object that hides the seams"
)]
pub fn build_plan_walled(
    src: &str,
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    kills: &BTreeSet<CfgNodeId>,
    survival: Option<&TrustedFootprints>,
    resolutions: Option<&Resolutions>,
    dialect: &Dialect,
    fact_backings: &BTreeMap<FactKey, FactBacking>,
    vouches: &Vouches,
    connected: &ConnectedPipes,
    observe: impl Fn(FactKey) -> Observable,
    arena: &mut dorc_core::ProvArena,
) -> Plan {
    // Map each classified leaf's AstId → its fact (establish + query classes carry
    // one). The fold reaches over the AST and needs each leaf's observed status keyed
    // by AstId, so it asks this map, then the injected `observe`. A Query guard's fact
    // is included so the fold can read its (probe-sourced) Status channel — the rc that
    // resolves the `&&`/`||` branch (task-D2).
    let leaf_fact: BTreeMap<AstId, FactKey> = classes
        .iter()
        .filter_map(|(node, class)| {
            let fact = match class {
                SkipClass::EstablishAmbient(f)
                | SkipClass::EstablishWritten(f)
                | SkipClass::QueryResolvable { fact: f, .. } => *f,
                // An in-loop Members site, and an inlined CALL (arch-2), are never fold
                // controllers (a Members body is render-floored; a CALL is an aggregate whose
                // own rc is ⊤), so neither carries a fold status.
                SkipClass::EstablishMembers { .. }
                | SkipClass::InlineCall { .. }
                | SkipClass::MustRun => return None,
            };
            Some((cfg.node(*node).ast, fact))
        })
        .collect();

    // Run the apply fold. A leaf's fold-status is its injected observation; a leaf
    // with no fact (MustRun / opaque / query without an oracle effect) is ⊤ ⇒ no fold
    // through it (`inv-kfail`).
    let fold = fold::fold(ast, |leaf| leaf_fact.get(&leaf).map(|f| observe(*f)));

    // Each step is paired with the wall predicate (`class_is_establish_bearing`): is this a
    // modeled MUTATOR whose run would invalidate downstream elide-licenses (silence=wall,
    // `23Ib-fd10`)? Computed here where the `SkipClass` is in scope (a `Step` does not carry
    // it) and consumed by the plan-time wall walk after the span-sort below.
    // Each entry: the step, its wall-bearing bit, and its `CfgNodeId` (the footprint-lookup
    // key for the survival walk — `TrustedFootprints` is node-keyed).
    let mut steps: Vec<(Step, bool, CfgNodeId)> = Vec::with_capacity(classes.len());
    for (node, class) in classes {
        let ast_id = cfg.node(*node).ast;
        let sh = command_text(src, ast, ast_id);
        // 24J §2 — a SUBSUMED non-last stage of a connected check-pipe: OMIT it (controlled by
        // the governing last stage) once that governing stage's connected verdict is KNOWN — a
        // known rc (converged OR diverged) lets the whole READ-ONLY pipe be substituted, saving
        // the check-tax; the governing stage reproduces the rc, so the member's own status/stdout
        // never escape the collapsed unit. An unknown/⊤ governing verdict, or a ⊤-successor member,
        // ⇒ RUN (`kFAIL-perform`). The Omit render is gated on the governing stage neutralising
        // (`is_neutralised` walks the pipe's leaves), so a governing stage that fails to Replace
        // keeps this member verbatim too — the safe direction.
        let mut disposition = if let Some(gov_node) = connected.member_governor(*node) {
            let gov_ast = cfg.node(gov_node).ast;
            let gov_known = leaf_fact
                .get(&gov_ast)
                .is_some_and(|f| matches!(observe(*f).status, Predicted::Value(_)));
            if gov_known && !has_top_successor(cfg, *node) {
                Disposition::Omit {
                    controller: gov_ast,
                }
            } else {
                Disposition::Run
            }
        }
        // An in-loop Members site and an inlined CALL each take their own all-or-nothing
        // license path (the PER-MEMBER / PER-BODY-SITE observations); every other class
        // takes the single-fact `disposition_for`.
        else {
            match class {
                SkipClass::EstablishMembers {
                    members,
                    self_reached,
                } => members_disposition(cfg, *node, members, *self_reached, &observe),
                // arch-2 (`i-3`): the CALL aggregates its body sites' observations.
                SkipClass::InlineCall { sites } => inline_disposition(cfg, *node, sites, &observe),
                _ => {
                    let observed = match class {
                        SkipClass::EstablishAmbient(f)
                        | SkipClass::EstablishWritten(f)
                        | SkipClass::QueryResolvable { fact: f, .. } => Some(observe(*f)),
                        SkipClass::EstablishMembers { .. }
                        | SkipClass::InlineCall { .. }
                        | SkipClass::MustRun => None,
                    };
                    disposition_for(
                        cfg,
                        &fold,
                        *node,
                        class,
                        ast_id,
                        observed,
                        vouches.get(node),
                    )
                }
            }
        };
        // arch-1 witness (`vp-17`/`vp-18`): a licensed `Replace` records its FULL granted
        // witness — the establish site's `BookSource` origin — uncapped (the license tier).
        // Pure OUTPUT provenance attached AFTER the mint (the WELD): the origin is the site
        // the license already keys on, so it cannot influence the decision; it is EXEMPT
        // (`Exempt::ReceiptId`) and the `erasability` gate proves it perturbs nothing.
        if let Disposition::Replace(license, stand_in) = disposition {
            let origin = arena.leaf(
                dorc_core::OriginKind::BookSource,
                Some(ast.node(ast_id).span),
            );
            disposition = Disposition::Replace(
                license.with_witness(dorc_core::Witness::of(vec![origin])),
                stand_in,
            );
        }
        // Wall-bearing = an establish-bearing class OR a flagged kill (R3 / 24A §3): a running
        // kill mutates but classifies `MustRun`, invisible to `class_is_establish_bearing`, so
        // the threaded `kills` set restores it. A pure builtin / opaque `MustRun` is NOT in
        // `kills`, so it still never walls.
        let is_mutator = class_is_establish_bearing(class) || kills.contains(node);
        steps.push((
            Step {
                leaf: LeafId(0),
                ast: ast_id,
                sh,
                disposition,
            },
            is_mutator,
            *node,
        ));
    }

    // Source order (classify yields CFG-alloc order; sort by span for a faithful
    // reading). This sort MUST stay byte-identical to [`site_order`]'s sort+enumerate: the
    // probe's site-ids and the leaf-ids assigned below are ONE id space
    // (`inv-site-keyed-results`), so a record `site N …` keys back to leaf N.
    // `probe_site_id_equals_plan_leaf_id` pins the equivalence. Sort BEFORE the wall walk so
    // the walk sees steps in execution order (book order IS execution order — order-is-sacred).
    steps.sort_by_key(|(s, _, _)| (ast.node(s.ast).span.lo.0, ast.node(s.ast).span.hi.0));

    // Assign stable leaf ids in span order (the `inv-site-keyed-results` equivalence a record
    // `site N …` depends on). Done BEFORE the wall walk so the survival walk can name the wall
    // leaf a downstream elision crossed (the attribution witness, TC-3). Leaf-id assignment is
    // a pure function of span order — the walk only reads/rewrites dispositions, never ids — so
    // moving it earlier is byte-neutral for the flag-off path.
    for (i, (step, _, _)) in steps.iter_mut().enumerate() {
        step.leaf = LeafId(u32::try_from(i).unwrap_or(u32::MAX));
    }

    // The plan-time WALL (silence=wall / `23Ib-fd10` / `23O` §2 settled law). A MODELED mutator
    // that will RUN at apply may touch anything it did not declare (the frame problem, `233`),
    // so silence licenses nothing. `survival` selects HOW a running mutator walls (TC-1):
    let mut survival_report = SurvivalReport::default();
    match survival {
        // Flag-off (BASELINE, byte-identical to Stage-1): a running mutator is a TOTAL wall.
        None => wall_walk_total(&mut steps),
        // Flag-on (the golden hill): a running FOOTPRINTED mutator scopes its wall; a downstream
        // converged `Replace` survives iff its backing SET is disjoint from every footprint (TC-3).
        Some(footprints) => {
            survival_report =
                wall_walk_survival(&mut steps, footprints, resolutions, dialect, fact_backings);
        }
    }

    // Drop the wall bookkeeping; the leaf ids are already assigned.
    let steps: Vec<Step> = steps.into_iter().map(|(step, _, _)| step).collect();
    Plan {
        steps,
        survival_report,
    }
}

/// The BASELINE wall walk (flag-off / Stage-1 / `23Ib-fd10`): walk once in execution order
/// maintaining a single `walled` flag. An elided/omitted mutator casts NO shadow (it never
/// runs), so only a *running* mutator walls; once walled, every later establish-bearing
/// `Replace` demotes to `Run`. Demotion is ONLY ever Replace→Run (`inv-kfail`). Structurally
/// identical to the pre-Stage-2 inline walk — kept a `bool`, no footprints in sight.
fn wall_walk_total(steps: &mut [(Step, bool, CfgNodeId)]) {
    let mut walled = false;
    for (step, is_mutator, _node) in steps {
        if walled && *is_mutator && matches!(step.disposition, Disposition::Replace(..)) {
            step.disposition = Disposition::Run;
        }
        if *is_mutator && matches!(step.disposition, Disposition::Run) {
            walled = true;
        }
    }
}

/// The SURVIVAL wall walk (flag-on / Stage 2 / the golden hill). In execution order, maintain a
/// `total_wall` flag (set by a running FOOTPRINT-LESS mutator — silence = wall, unchanged) plus
/// the accumulated running-wall footprints. A downstream establish-bearing `Replace` is put
/// through the ONE total [`survival::wall_verdict`]: it survives (stays `Replace`) iff no total
/// wall stands AND its backing is disjoint from every accumulated footprint; a crossing of ≥1
/// wall attaches the attribution witness (TC-3). A running mutator (whether it just demoted, or
/// was never converged) then contributes: WITH a lifted footprint it scopes the wall (union its
/// coordinates); WITHOUT one it totalises the wall. Demotion stays Replace→Run only (`inv-kfail`).
fn wall_walk_survival(
    steps: &mut [(Step, bool, CfgNodeId)],
    footprints: &TrustedFootprints,
    resolutions: Option<&Resolutions>,
    dialect: &Dialect,
    fact_backings: &BTreeMap<FactKey, FactBacking>,
) -> SurvivalReport {
    // `None` resolvers ⇒ the token-equality floor (24F §3): the empty map, every kind resolver-less.
    let empty = Resolutions::none();
    let resolutions = resolutions.unwrap_or(&empty);
    let mut report = SurvivalReport::default();
    let mut total_wall = false;
    let mut accumulated: Vec<survival::AccumulatedWall> = Vec::new();
    for (step, is_mutator, node) in steps {
        // 1. Survival test for a converged mutator's `Replace` against the walls so far — both the
        //    backing and each accumulated footprint canonicalized through the resolvers (24F §3).
        if *is_mutator && let Disposition::Replace(license, _) = &step.disposition {
            // `277` §5 backing-SETS: build the fact's backing SET — its own cell plus the
            // observe-backing-widening siblings, carrying the THREADED minting family. A map-MISS
            // (a file-write / auto-cell / Members fact, or a caller with no threaded map) falls to
            // the singleton `Backing::of_fact` (the reverse-lookup floor — today's behavior).
            let fact = license.fact();
            let backing = match fact_backings.get(&fact) {
                Some(fb) => Backing::widened(fact, fb.family, fb.observed.clone()),
                None => Backing::of_fact(fact),
            };
            match survival::wall_verdict(total_wall, &accumulated, &backing, resolutions, dialect) {
                // Crossed no wall — an ordinary pre-wall elision; leave it exactly as the
                // flag-off world would (no witness, `Replace` untouched).
                survival::WallVerdict::SurvivedClean => {}
                // Crossed ≥1 running wall, all disjoint — survives WITH attribution. Rebind the
                // disposition to carry the witness (pure output provenance, post-mint).
                survival::WallVerdict::Survived(witness) => {
                    if let Disposition::Replace(license, stand_in) =
                        std::mem::replace(&mut step.disposition, Disposition::Run)
                    {
                        step.disposition =
                            Disposition::Replace(license.with_survival(witness), stand_in);
                    }
                }
                // A total wall stands, the backing hit a footprint, or a same-kind pair could not be
                // canonicalized (§3a may-alias) — demote (`inv-kfail`, fail toward run). A may-alias
                // demote is instrumented (24F §3a — the yardstick shows the fire-rate).
                survival::WallVerdict::Demoted(reason) => {
                    match reason {
                        survival::DemoteReason::MayAlias => {
                            report.may_alias_fires = report.may_alias_fires.saturating_add(1);
                        }
                        // 24G Part B: a reach-expanded coordinate poisoned this elision — attribute
                        // the reach-function KIND for the why-lens ("…poisoned via <kind>.reaches()").
                        survival::DemoteReason::Poisoned {
                            via_reach: Some(kind),
                        } => report.reach_poisonings.push((step.leaf, kind)),
                        survival::DemoteReason::TotalWall
                        | survival::DemoteReason::Poisoned { via_reach: None } => {}
                    }
                    step.disposition = Disposition::Run;
                }
            }
        }
        // 2. Wall contribution: a RUNNING mutator walls — scoped if it has a footprint, total
        // otherwise (silence = wall). An elided/omitted mutator (survived, or converged away)
        // casts no shadow, so it is skipped here.
        if *is_mutator && matches!(step.disposition, Disposition::Run) {
            match footprints.get(*node) {
                Some(footprint) => accumulated.push(survival::AccumulatedWall {
                    wall_leaf: step.leaf,
                    footprint: footprint.clone(),
                }),
                None => total_wall = true,
            }
        }
    }
    report
}

/// The per-leaf disposition: the fold first (a provably-dead leaf is `Omit`ted), then
/// convergence-elision (`Replace` with the value-preserving stand-in), else `Run`.
///
/// The fold takes precedence over convergence-elision because a *dead* leaf has no
/// status a consumer reads — `Omit` is strictly the right disposition (vs `Replace`,
/// which exists to reproduce a status). Both are the apply collapse; a leaf that is
/// neither runs (`kFAIL-perform`).
fn disposition_for(
    cfg: &Cfg,
    fold: &FoldResult,
    node: CfgNodeId,
    class: &SkipClass,
    ast_id: AstId,
    observed: Option<Observable>,
    vouch: Option<&ByVouch<VerdictVouch>>,
) -> Disposition {
    // (0) the in-loop render floor (task-L1, `209` brk-1): a leaf inside a loop body or
    // condition is MustRun — UNLESS it is the in-loop Members shape, which is routed to
    // `members_disposition` BEFORE this function (task-L2 item-3 lifts the floor for
    // exactly that shape). For every OTHER in-loop leaf (a single-fact establish, an
    // in-loop Query, the loop condition) the floor stands: the line-granular render still
    // cannot elide a single iteration, and per-iteration `&&`/`||` deadness is not
    // line-expressible. POST-loop leaves are NOT in-loop, so the value below a converged
    // loop unlocks normally (the brk-1 value-unlock).
    if cfg.in_loop_body(node) {
        return Disposition::Run;
    }

    // (2) the fold: a provably-dead branch leaf is omitted. Minted ONLY from a known
    // controlling status (`fold` records `dead` only then) — `inv-kfail`. The fold
    // reached the deadness via the controller leaf's AstId; resolve its fact for
    // provenance + the render's neutralised-controller gate. Top-containment still
    // gates: a ⊤-contaminated leaf is never folded away (context unmodeled).
    if !has_top_successor(cfg, node)
        && let Some(controller_ast) = fold.dead_controller(ast_id)
    {
        return Disposition::Omit {
            controller: controller_ast,
        };
    }

    // (1) value-preserving substitution: convergence-elision of a converged-establish,
    // OR a Query-guard substitution (task-D2 — both minted through `prove_replaceable`,
    // which dispatches on the class). Reached only for a leaf the fold did NOT omit
    // (its branch stays live). Top-containment (16G hole-5): a ⊤-successor leaf is
    // never replaced.
    match class {
        SkipClass::EstablishAmbient(_) | SkipClass::QueryResolvable { .. }
            if !has_top_successor(cfg, node) =>
        {
            let verdict =
                PhasedVerdict::<Probe>::new(observed.map_or(Verdict::Unknown, |o| o.effect));
            let consumed = May(cfg.consumed_observables(node).clone());
            let status = observed.map_or(Predicted::Top, |o| o.status);
            // The elide-weld (24D §3): thread the reached vouch from the `Vouches` map (Part A's
            // `build_vouches` already populates ambient sites — no re-lift). An ambient site with
            // no vouch runs; a Query site is never in the map (`None`) and its arm ignores it.
            match ReplaceLicense::prove_replaceable(
                class,
                Grade::Must,
                verdict,
                consumed,
                status,
                vouch.cloned(),
            ) {
                Some(license) => {
                    // The value-preserving stand-in reproduces the predicted Status channel.
                    // An unpredicted status (`Predicted::Top`) falls back to `true` (rc 0) in
                    // two cases, neither fabricating a value a LIVE reader consumes: (a) a
                    // converged-establish whose status is not branch-consumed (`prove_replaceable`
                    // blocks a branch-consumed `Top` via `StatusRelaxable`, `19D`; a Query guard
                    // always carries a known rc) — the rc-0 placeholder is never read by a branch;
                    // (b) door-3 (`20V` §4): a `cmd || true` left whose ⊤ status is `StatusInvariant`
                    // -consumed. There `true` is the IDIOM, not a predicted value — the mint is
                    // licensed by INVARIANCE (both `||` continuations rejoin identically, so any rc
                    // is extensionally faithful), NOT by a claim cmd exits 0. This keeps weld-5 (no
                    // fabricated values for LIVE reads) intact: the `||` read is dead-in-fact.
                    // (A book defining a `true()` function never reaches this arm — door-3
                    // refuses at the cfg mark, find-I: the stand-in word would resolve to the
                    // function, not the builtin.)
                    let stand_in = match status {
                        Predicted::Value(rc) => StandIn::from_rc(rc),
                        Predicted::Top => StandIn::True,
                    };
                    Disposition::Replace(license, stand_in)
                }
                None => Disposition::Run,
            }
        }
        // The guard tier (rul-ternary-verdict's third verb — rul-guard-license). A past-a-wall
        // `EstablishWritten` site (an opaque upstream poisoned its resting probe, so it can no
        // longer ELIDE) with a REACHED vouch and a CONVERGED probe-verdict mints a `Guard`: the
        // oracle's own verdict check re-decides LIVE at apply (`( check ) || <original>`), so the
        // stale plan-time convergence is never trusted (X-drift). No vouch, or a diverged/unknown
        // verdict, ⇒ run — a guard at a predicted-change site buys nothing (`inv-kfail`;
        // `GuardLicense::mint` returns `None` off `Verdict::Converged`). Top-containment: a
        // ⊤-successor site (`cmd &`) never guards, exactly as it never Replaces (P-background).
        SkipClass::EstablishWritten(fact) if !has_top_successor(cfg, node) => match vouch {
            Some(v) => {
                let verdict = observed.map_or(Verdict::Unknown, |o| o.effect);
                match GuardLicense::mint(*fact, v.clone(), verdict) {
                    Some(license) => Disposition::Guard(license),
                    None => Disposition::Run,
                }
            }
            None => Disposition::Run,
        },
        _ => Disposition::Run,
    }
}

/// The disposition for an in-loop **Members** body leaf (task-L2 item-3, `209` brk-1(b)) —
/// the all-or-nothing in-loop license. Observe EVERY member's host verdict (the Effect
/// channel), then mint a [`LicenseVia::MembersLoop`] `Replace` via
/// [`ReplaceLicense::prove_members_replaceable`] iff all are Converged, the site is
/// `self_reached`, and the consumption gates pass. The stand-in is always `true` (the body
/// is replaced by a `true` that the loop still iterates N times over — observable-
/// preserving given all-converged + the consumed-status gate). On refusal the leaf runs.
///
/// Top-containment (16G hole-5): a ⊤-successor leaf is never replaced (a loop body leaf
/// with a `cmd &` shape, say). The in-loop leaf's status is ⊤ for a mutator (fork-mutator-
/// rc), so a consumed status (errexit-region, or a post-loop `$?` reading the body —
/// item-6a) blocks via the consumption gate, exactly as the single-fact path.
fn members_disposition(
    cfg: &Cfg,
    node: CfgNodeId,
    members: &[FactKey],
    self_reached: bool,
    observe: &impl Fn(FactKey) -> Observable,
) -> Disposition {
    if has_top_successor(cfg, node) {
        return Disposition::Run;
    }
    let member_verdicts: Vec<Verdict> = members.iter().map(|f| observe(*f).effect).collect();
    let consumed = May(cfg.consumed_observables(node).clone());
    // The in-loop body leaf's status: a mutator's rc is ⊤ (fork-mutator-rc), and a Members
    // site is a mutator (an establish), so ⊤. The consumption gate blocks a consumed ⊤.
    let status = Predicted::Top;
    match ReplaceLicense::prove_members_replaceable(
        members,
        &member_verdicts,
        self_reached,
        &consumed,
        status,
    ) {
        // The body is substituted by `true` (the loop still iterates N times over it).
        Some(license) => Disposition::Replace(license, StandIn::True),
        None => Disposition::Run,
    }
}

/// The disposition for an inlined function-CALL leaf (arch-2, brk-2, `i-3`) — the
/// all-or-nothing CALL license. Observe each spliced body Establish site's host verdict, then
/// mint a [`LicenseVia::InlineCall`] `Replace` (the CALL span → `true`) via
/// [`ReplaceLicense::prove_inline_replaceable`] iff every effect-bearing body leaf licenses
/// elision. On refusal the call RUNS — the real function body executes (the run-it floor,
/// `kFAIL-perform`).
///
/// The CALL leaf's own status is ⊤ (a mutator-shaped aggregate, fork-mutator-rc), so a consumed
/// status (errexit-region, a `$?`-reader, a bare `||` operand) blocks via the consumption gate
/// — exactly the single-fact path. Top-containment (16G hole-5): a ⊤-successor CALL (e.g. a
/// `prov &` background) is never replaced. The body sites are NOT render-edited (`i-3`); only
/// the CALL span is. `observe` reads the same per-fact host oracle the rest of the plan uses.
fn inline_disposition(
    cfg: &Cfg,
    node: CfgNodeId,
    sites: &[InlineSite],
    observe: &impl Fn(FactKey) -> Observable,
) -> Disposition {
    // The in-loop render floor (task-L1, `209` brk-1): an inlined CALL inside a loop body is
    // MustRun this round — the line/span render cannot elide a single iteration of a call, and
    // a member-precision path for inlined calls is not built (it would compose the Members
    // value with the call's positionals, a deferred multi-leaf case). EXPLICIT here (not
    // relying on the back-edge self-poison that incidentally tends to make an in-loop body
    // establish `EstablishWritten`): an in-loop inlined call NEVER mints a license, robustly.
    // (`inline_disposition` runs BEFORE `disposition_for`'s floor — the Members precedent — so
    // the floor must be re-checked here, like `members_disposition` re-checks `has_top_successor`.)
    if cfg.in_loop_body(node) {
        return Disposition::Run;
    }
    if has_top_successor(cfg, node) {
        return Disposition::Run;
    }
    let consumed = May(cfg.consumed_observables(node).clone());
    // The CALL aggregate's status: ⊤ (a mutator-shaped call's rc has no sanctioned source,
    // fork-mutator-rc). A consumed ⊤ status blocks via the consumption gate (door-3 `|| true`
    // does not — `StatusInvariant`).
    let status = Predicted::Top;
    match ReplaceLicense::prove_inline_replaceable(sites, observe, &consumed, status) {
        // The whole CALL span substitutes to `true` (the body is gone — observable-preserving
        // given every body establish is converged + the consumed-status gate).
        Some(license) => Disposition::Replace(license, StandIn::True),
        None => Disposition::Run,
    }
}

/// A leaf's source text flattened to one line (interior whitespace collapsed) for an
/// inline diagnostic message — a heredoc leaf's text spans lines, which would garble a
/// single-line `error[…]:` line otherwise.
fn command_text_oneline(sh: &str) -> String {
    sh.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The verbatim source text of a node's `[lo, hi)` span — the exact sh the admin
/// wrote. Resolving a span for display is allowed under `inv-referent-agnostic`
/// (it is provenance, not a logic branch).
fn command_text(src: &str, ast: &Ast, id: AstId) -> String {
    let span = ast.node(id).span;
    src.get(span.lo.0 as usize..span.hi.0 as usize)
        .unwrap_or_default()
        .to_string()
}

/// Does this CFG node have a top (`Top`) node among its successors? Top-containment
/// (16G hole-5): a leaf whose own statement is top-contaminated — e.g. `cmd &`,
/// lowered as the leaf followed by a `Top` — is not safely replaceable.
fn has_top_successor(cfg: &Cfg, node: CfgNodeId) -> bool {
    cfg.succ_ids(node)
        .any(|s| cfg.node(s).kind == CfgNodeKind::Top)
}

/// The plan-time wall predicate (silence=wall / `23Ib-fd10`): is this class a modeled
/// **mutator** — an establish-bearing site whose *running* would invalidate downstream
/// elide-licenses? A running such site walls; a walled such site's `Replace` is demoted.
///
/// Establish-bearing = `EstablishAmbient`/`EstablishWritten`/`EstablishMembers`, and an
/// `InlineCall` any of whose body sites establish (a spliced body mutation runs when the call
/// runs). Deliberately NOT establish-bearing, so they never wall:
/// * `QueryResolvable` — a declared read-only guard; a read kills nothing (and a downstream
///   Query of any upstream mutator is *already* run by rule-query-validity, so it never
///   reaches a post-wall `Replace` to demote);
/// * `MustRun` — the lossy residue. A pure builtin (`:`/`echo`/`cd`) is `MustRun` and must NOT
///   wall (see `exec-pure-builtin`: `cd /tmp` runs, the install below it still elides). An
///   *opaque* is also `MustRun`, but it already `⊤`-poisons every downstream fact in `classify`
///   (⇒ they are `EstablishWritten` ⇒ never elide), so not walling it here is harmless
///   redundancy. A *kill* (`apt-get purge`) is `MustRun` too and DOES mutate — this predicate
///   cannot see it (the `CommandEffect` is not in the `SkipClass`), so the R3 kill gap (24A §3)
///   is closed one layer up: [`build_plan_walled`] ORs this predicate with a threaded
///   kill-node set, so a running kill walls without a `SkipClass` change. Kill-unaware
///   [`build_plan`] passes an empty set (unchanged behaviour for `hostsim`/tests).
fn class_is_establish_bearing(class: &SkipClass) -> bool {
    match class {
        SkipClass::EstablishAmbient(_)
        | SkipClass::EstablishWritten(_)
        | SkipClass::EstablishMembers { .. } => true,
        SkipClass::InlineCall { sites } => sites.iter().any(|s| {
            matches!(
                s.class,
                SkipClass::EstablishAmbient(_)
                    | SkipClass::EstablishWritten(_)
                    | SkipClass::EstablishMembers { .. }
            )
        }),
        SkipClass::QueryResolvable { .. } | SkipClass::MustRun => false,
    }
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
            match step.disposition {
                Disposition::Replace(_, _) => c.elide += 1,
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
                    out.push_str(&license.insert().render_line(&step.sh));
                    out.push('\n');
                }
            }
        }
        out
    }

    /// The guard **preamble** (24D §2 / rul-ternary-verdict): the verdict-function defs the guarded
    /// lines invoke, each emitted ONCE (deduped by funcname; sh's last-writer-wins + top-to-bottom
    /// exec means every invocation sees its own def). Empty when no site guards (so HEAD is byte-
    /// unchanged). The cli prepends this to the apply artifact, above the guarded lines — the guard
    /// lane's analogue of the probe's wrapper-def emission. The bodies are shipped STRIP-ONLY (the
    /// oracle's own bytes; no engine-synthesized sh — the two never-clauses).
    ///
    /// A funcname appearing twice is emitted once; a provider with TWO distinct verdict bodies
    /// under one funcname (the probe's `apt-get`-as-package-and-pkgindex shape) is not modeled here
    /// (a verdict function has one body; tc-guard-preamble-reemit flags the re-emit case deferred).
    #[must_use]
    pub fn guard_preamble(&self, ast: &Ast) -> String {
        let mut defined: BTreeSet<&str> = BTreeSet::new();
        let mut out = String::new();
        for step in &self.steps {
            // A render-REFUSED guard (heredoc/redirect) emits no invocation, so its preamble def
            // would be dead — skip it, so a book whose only guard is refused stays byte-clean. The
            // OOB-safe check tolerates a synthetic test Plan whose `AstId`s index no real node.
            if let Disposition::Guard(license) = &step.disposition
                && !(ast.len() > step.ast.0 as usize && guard_render_refused(ast, step.ast))
                && defined.insert(license.insert().fn_name())
            {
                out.push_str(license.insert().preamble());
                out.push('\n');
            }
        }
        out
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
    /// observed status (`19A §5`), so `useradd[rc9] || mkdir` would substitute `(exit 9)`,
    /// keeping `|| mkdir` live. Because the edit replaces ONLY the command span and leaves
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
        let edits = self.collect_edits(src, ast);
        let artifact = emit_span_edits(src, &edits);
        // The GUARD PREAMBLE (24D §2 / rul-ternary-verdict): the verdict-function defs the guarded
        // lines invoke, emitted ONCE between the apply header and the book (the defs must precede
        // their invocations — sh execs top-to-bottom, and the header is pure comments). Empty when
        // no site guards ⇒ a guard-free book stays byte-identical to HEAD. `emit_span_edits` emits
        // `apply_header()` as the artifact's verbatim prefix, so splicing after it lands the defs
        // above the whole book.
        let preamble = self.guard_preamble(ast);
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
    pub fn render_refusal_diagnostics(
        &self,
        ast: &Ast,
        interner: &Interner,
    ) -> Vec<dorc_core::Diagnostic> {
        use dorc_core::diag::{Diag, DiagCode, RenderHeredocRefused, SiteId};
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();
        let mut diags = Vec::new();
        for step in &self.steps {
            let would_elide = match &step.disposition {
                // A Replace value-substitutes the span; a Guard EDITS it to `( check ) || <orig>` —
                // both strand a heredoc body, so a heredoc-bearing leaf of either must REFUSE and
                // run verbatim (X-heredoc: a vouched heredoc site stays RUN, loudly).
                Disposition::Replace(_, _) | Disposition::Guard(_) => true,
                Disposition::Omit { controller } => is_neutralised(&by_ast, ast, *controller, 0),
                Disposition::Run => false,
            };
            if would_elide && leaf_has_heredoc(ast, step.ast) {
                // The migrated `DiagCode::RenderHeredocRefused` spine (`22B` §5 worked-2 — the
                // most-improved case: an inline literal becomes a first-class typed variant the
                // grep gate sees and the registry pins Error+WarnOrDeny). Lowered to the legacy
                // stream, preserving `(code-slug, span, Error)` so the coverage span-bridge and
                // the erasability identity plane are unchanged. The interner resolves no excerpt
                // here (the payload carries only a site) but is threaded for the shared lowering.
                // The verb is disposition-aware: a GUARD refusal says "guard" (X-heredoc's
                // expected-diagnostics pins `guard`), a Replace/Omit refusal says "elide".
                let verb = if matches!(step.disposition, Disposition::Guard(_)) {
                    "guard"
                } else {
                    "elide"
                };
                let diag = Diag::new(
                    DiagCode::RenderHeredocRefused(RenderHeredocRefused {
                        site: SiteId::leaf(step.leaf),
                    }),
                    ast.node(step.ast).span,
                )
                .label(format!(
                    "leaf-exact render refuses to {verb} a heredoc-bearing command (`{}`): its \
                     span covers the `<<` operator, not the body lines, so substituting it would \
                     strand the heredoc body — it runs verbatim",
                    command_text_oneline(&step.sh),
                ))
                .help("split the heredoc body to its own leaf, or mark the kind un-elidable");
                diags.push(diag.to_legacy(interner));
            }
        }
        diags
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
    fn collect_edits(&self, src: &str, ast: &Ast) -> Vec<SpanEdit> {
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
        for step in &self.steps {
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
                        (license.insert().render_line(&original), true, false)
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
                "partial span-edit overlap [{},{}) vs [{},{}) — leaf-seam violated",
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
        "span-edit count mismatch: {spliced_count} spliced vs {} collected — an edit was \
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
fn leaf_has_heredoc(ast: &Ast, leaf: AstId) -> bool {
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
/// * `kind:entity#selector` for [`EntityRef::Operand`] — `package:nginx#installed`;
/// * `kind#selector` for [`EntityRef::Singleton`] — `package-index#fresh`. A
///   singleton has no operand, so it carries NO `:`-segment (the bare `package-index:#fresh`
///   the strain-4 note warned against is avoided — `:` present ⇔ an operand exists).
///
/// The selector is ALWAYS rendered (`#selector`): it is the per-entity facet the
/// re-key added (`an-per-entity-selector`), and dropping it would let an `is-active`
/// probe-verdict discharge an unmet `#enabled` cell — a wrong-elision under apply's
/// `kFAIL` (`cli/CLAUDE.md` "stdin re-key gotcha"). The label is injective over
/// distinct `FactKey`s modulo a `:`/`#` collision in an interned name (a disposable-
/// parser limitation, `ch-scope`; book operands like `nginx` don't carry them).
#[must_use]
pub fn fact_label(interner: &Interner, fact: FactKey) -> String {
    let kind = interner.resolve(fact.kind.0);
    let selector = interner.resolve(fact.selector.0);
    match fact.entity {
        EntityRef::Operand(tok) => {
            format!("{kind}:{}#{selector}", interner.resolve(tok.0))
        }
        EntityRef::Singleton => format!("{kind}#{selector}"),
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
    const CORPUS_PREDICT_SRC: &str = r##"
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      update) idx : package-index; test -n fresh : package-index:#fresh ;;
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg"#installed ; fi ;;
   esac
}
"##;

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
    ) -> Option<String> {
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
                    return Some(strip_predict(CORPUS_PREDICT_SRC, check, interner));
                }
            }
        }
        None
    }

    /// `package:nginx#installed` — the cell `apt-get install nginx` gates. The
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
            ),
            Rung::Both,
        )
    }

    /// Test convenience (elide-weld, 24D §3): vouch every AMBIENT establish site so the
    /// plan-mechanics helpers keep exercising ELISION. Deliberately NOT `EstablishWritten` — a
    /// vouched+converged written site fires the GUARD tier (`Disposition::Guard`), which these
    /// elision/wall/fold tests do not expect (guards are pinned by the guard23 e2e + the guard
    /// unit tests). The vouch GATE is pinned by [`no_license_for_ambient_without_vouch`] + e2e +
    /// the FAITHFUL sweep/coverage lift; here a synthetic vouch (no oracle lift) keeps focus.
    fn vouch_all(classes: &[(CfgNodeId, SkipClass)]) -> Vouches {
        let mut vouches = Vouches::new();
        for (node, class) in classes {
            if matches!(class, SkipClass::EstablishAmbient(_)) {
                vouches.insert(*node, test_vouch());
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
            )
        };
        // jc-mint-policy m-a: a diverged/unknown probe-verdict NEVER guards (a guard at a
        // predicted-change site buys nothing; `inv-kfail` → run). The mint DEMANDS a
        // `ByVouch<VerdictVouch>` (TC-tier-2) — a fact/silence claim would not typecheck here.
        assert!(
            GuardLicense::mint(
                nginx_fact(),
                ByVouch::vouched(vouch(), Rung::Both),
                Verdict::Diverged
            )
            .is_none(),
            "a diverged probe-verdict must not mint a guard"
        );
        assert!(
            GuardLicense::mint(
                nginx_fact(),
                ByVouch::vouched(vouch(), Rung::Both),
                Verdict::Unknown
            )
            .is_none(),
            "an unknown probe-verdict must not mint a guard"
        );
        let license = GuardLicense::mint(
            nginx_fact(),
            ByVouch::vouched(vouch(), Rung::Both),
            Verdict::Converged,
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
        );
        let license = GuardLicense::mint(
            nginx_fact(),
            ByVouch::vouched(vouch, Rung::Both),
            Verdict::Converged,
        )
        .unwrap();
        // The guard_shape law: `( <check> ) || <original verbatim>   # dorc: guard [...]`.
        let line = license.insert().render_line("apt-get install -y curl");
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
                    )
                    .unwrap(),
                ),
            }
        };
        let plan = Plan {
            steps: vec![mk(0), mk(1)],
            survival_report: SurvivalReport::default(),
        };
        // A throwaway (empty) Ast: the synthetic `AstId`s index no real node, and the OOB-safe
        // check in `guard_preamble` treats an out-of-arena id as not-refused (so both guards emit).
        let ast = dorc_syntax::parse("").value;
        // Two guards sharing one funcname ⇒ ONE preamble def (sh last-writer-wins; the invocation
        // sees its own def).
        assert_eq!(
            plan.guard_preamble(&ast)
                .matches("apt_get__is_converged()")
                .count(),
            1,
            "preamble deduped by funcname: {}",
            plan.guard_preamble(&ast)
        );
        // The exhaustive `disposition_counts` match now feeds the guard bucket (the summary's
        // guard column becomes real — DispositionCounts forced this wiring).
        let counts = plan.disposition_counts();
        assert_eq!(counts.guard, 2);
        assert_eq!(counts.sites, 2);
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;
        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |provider, argv| {
                if probeable {
                    ship_corpus(&checks, &i, provider, argv)
                } else {
                    None
                }
            },
            |_, _, _| None,
            |_| false,
        );
        (probe, i)
    }

    #[test]
    fn disposition_counts_tally_bucketing_and_sites_invariant() {
        // plans/240 Stage-1 yardstick: the plan-summary's per-disposition tally. Pin
        // (1) each disposition lands in its own bucket, (2) `guard` is 0 at HEAD (no
        // `Disposition` mints one until the Stage-3 guard tier), and (3) the
        // `sites == elide + omit + guard + run` invariant the greppable grammar promises.
        let fact = nginx_fact();
        let license = ReplaceLicense::prove_replaceable::<Apply>(
            &SkipClass::EstablishAmbient(fact),
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
            survival_report: SurvivalReport::default(),
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
                survival_report: SurvivalReport::default(),
            }
            .disposition_counts(),
            DispositionCounts::default()
        );
    }

    #[test]
    fn compile_probe_resolvable_sites_probed_unresolvable_recorded() {
        // The probe = EstablishAmbient sites WITH a declared read-only probe. A site
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
        // becomes one ProbeDerivation; render_sh appends the stripped __touches def + a per-site
        // `deriv N coord=` scaffold to the phase-1 probe (no second shebang). A non-escalating
        // provider (the un-oracled systemctl, not even a wall candidate) yields no derivation.
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse("apt-get install -y nginx\nsystemctl reload nginx\n");
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::predict::lift_predicts(&mut i, CORPUS_PREDICT_SRC).value];
        let (classes, _why, kills, _kill_coords, _fact_backings) =
            dorc_analysis::effect::classify_with_why_diags(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &BTreeSet::new(),
                &BTreeMap::new(),
                &mut i,
                &mut dorc_core::ProvArena::new(),
            );
        let classes = classes.value;
        let derivations = compile_derivations(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &kills,
            |provider, _argv| {
                // Escalate ONLY apt-get (the payload-bound install); everything else declines. The
                // forward munge keys the book word `apt-get` on the segment `apt_get`.
                (dorc_oracle::predict::map_provider_name(i.resolve(provider)) == "apt_get").then(
                    || DerivationShip {
                        sh: "apt_get__touches() { apt-manifest \"$1\"; }".to_string(),
                        call: "apt-manifest".to_string(),
                    },
                )
            },
        );
        assert_eq!(
            derivations.derivations.len(),
            1,
            "only the apt-get install (an EstablishAmbient wall candidate) escalated"
        );
        assert_eq!(
            derivations.derivations[0].site,
            LeafId(0),
            "the install is site 0"
        );
        let sh = derivations.render_sh(&records::Nonce::spike_default(), &i);
        assert!(
            sh.contains("apt_get__touches() { apt-manifest"),
            "the stripped touches def ships verbatim: {sh}"
        );
        assert!(
            sh.contains("| { _n=0; while IFS= read -r _c; do printf 'dorc deriv 0 coord=%s"),
            "the per-site deriv readback scaffold renders (framed, counting subshell): {sh}"
        );
        assert!(
            sh.contains("printf 'dorc deriv-end 0 n=%s @@dorc@@\\n' \"$_n\"; }"),
            "the at-most family closes with a `deriv-end` count record (262 §2 / 26A stop-1): {sh}"
        );
        assert!(
            !sh.starts_with("#!/bin/sh"),
            "no second shebang — the derivation-probe rides the SAME phase-1 block: {sh}"
        );
    }

    #[test]
    fn two_same_command_sites_stay_distinct_sites() {
        // `inv-site-keyed-results` (the core of the re-key): two same-command sites are
        // NEVER collapsed (spike-2's per-fact dedup is gone). Two IDENTICAL `apt-get
        // install -y nginx` lines on the SAME cell: the SECOND sees the first establish
        // its cell upstream ⇒ EstablishWritten ⇒ unresolvable (correct — its resting
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
            rendered.contains("# site:1 skip-unresolvable"),
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;

        let probe = compile_probe(
            &parsed.value,
            &cfg,
            &value,
            &classes,
            &BTreeMap::new(),
            &ConnectedPipes::default(),
            |provider, argv| ship_corpus(&checks, &i, provider, argv),
            |_, _, _| None,
            |_| false,
        );
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
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
            &SkipClass::EstablishAmbient(f),
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
                &SkipClass::EstablishAmbient(f),
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
                    &SkipClass::EstablishAmbient(f),
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
                &SkipClass::EstablishAmbient(f),
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
                    &SkipClass::EstablishAmbient(f),
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
                    &SkipClass::EstablishAmbient(f),
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
                    &SkipClass::EstablishAmbient(f),
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
                &SkipClass::EstablishAmbient(f),
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
    fn no_license_for_written_or_mustrun_class() {
        // Only EstablishAmbient is elidable. EstablishWritten (an upstream same-run
        // mutation reaches it) and MustRun must run even with a Converged probe.
        let f = nginx_fact();
        for class in [SkipClass::EstablishWritten(f), SkipClass::MustRun] {
            assert!(
                ReplaceLicense::prove_replaceable(
                    &class,
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(Verdict::Converged),
                    quiet(),
                    Predicted::Value(Rc(0)),
                    Some(test_vouch()),
                )
                .is_none(),
                "{class:?} must not license a skip"
            );
        }
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

    /// A package kind-index modeling `apt-get install → package#installed` AND
    /// `apt-get update → package-index#fresh` (the spike-2 re-key, `notes/193` §1).
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
        idx.add_effect(apt, install, package, installed, ValueClaim::Establish);
        idx.add_effect(apt, update, package_index, fresh, ValueClaim::Establish);
        idx
    }

    /// Run the pipeline on `src`, answering `package:nginx#installed` with
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;
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
            &vouch_all(&classes),
            observe,
            &mut dorc_core::ProvArena::new(),
        );
        (plan, i)
    }

    /// Run the pipeline on `src`, answering each `package:<entity>#installed` cell with
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;
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
            &vouch_all(&classes),
            observe,
            &mut dorc_core::ProvArena::new(),
        );
        (plan, i)
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;
        let observe = |_f: FactKey| Observable::verdict_only(Verdict::Converged);
        let plan = build_plan(
            src,
            &parsed.value,
            &cfg,
            &classes,
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
        let (plan, _) = plan_for(fixture, Verdict::Converged);
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "install still runs: two upstream un-oracled neighbours ($(hostname) in the \
             case scrutinee, and `command -v nginx` in the if-guard) poison it — `update` \
             is no longer the poison, but it is not the only one (notes/193 strain-5)"
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
        // EstablishAmbient at the EFFECT layer). But under C-3 (205 §2 / 206 §3),
        // `set -e` CONSUMES the install's status, which for a mutator is ⊤
        // (fork-mutator-rc), so the plan disposition is now Run — NOT elided. The old
        // `ambient(set -e …)` assert masked C-3 by feeding a fabricated rc-0 through
        // `plan_for`; with the faithful ⊤-rc the install RUNS. Pin the EFFECT-layer
        // non-poison (classify EstablishAmbient) directly, separate from the plan-level
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
            let classes = dorc_analysis::effect::classify(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &BTreeSet::new(),
                &mut i,
                &mut dorc_core::ProvArena::new(),
            )
            .value;
            assert!(
                classes
                    .iter()
                    .any(|(_, c)| matches!(c, SkipClass::EstablishAmbient(_))),
                "fs-4: set -e does not poison ⇒ the install stays EstablishAmbient: {classes:?}"
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
        // the static ambient gate (same-cell reasoning) leaves BOTH `EstablishAmbient` — no
        // same-cell poison rescues this. curl is DIVERGED ⇒ it RUNS ⇒ it is a modeled mutator
        // that runs, which by the frame problem (233) may touch anything it did not declare. So
        // the downstream CONVERGED nginx install — which the static gate would elide — is
        // DEMOTED Replace→Run (`inv-kfail`: when unsure, act). At HEAD nginx wrongly elides past
        // the running curl; the wall closes that under-execution. No `set -e`, so the demotion
        // is the wall's doing, not errexit consuming the mutator's ⊤ status.
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
                Disposition::Run
            ),
            "silence=wall: the converged install is demoted to Run past the running curl mutator"
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
        let (classified, _why, kills_found, _kill_coords, _fact_backings) =
            dorc_analysis::effect::classify_with_why_diags(
                &cfg,
                &value,
                &parsed.value,
                &idx,
                &checks,
                &BTreeSet::new(),
                &BTreeMap::new(),
                &mut i,
                &mut arena,
            );
        let classes = classified.value;
        let kills = if walled { kills_found } else { BTreeSet::new() };
        let observe = |f: FactKey| {
            if f.kind == package
                && f.selector == installed
                && let EntityRef::Operand(tok) = f.entity
            {
                return Observable::verdict_only(verdict_of(i.resolve(tok.0)));
            }
            Observable::verdict_only(Verdict::Unknown)
        };
        let plan = build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &kills,
            None,
            None,
            &Dialect::empty(),
            &BTreeMap::new(),
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            observe,
            &mut arena,
        );
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
                Disposition::Run
            ),
            "silence=wall: the converged install is demoted to Run past the running kill"
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

    /// The survival books the mode-gate equality test iterates (install-only shapes so the
    /// corpus predict resolves them without the purge effect-add; the purge/kill and cross-kind
    /// shapes ride the e2e cases). Each is a diverged wall (or two) plus a converged same-kind
    /// different-entity survivor.
    const SURVIVAL_BOOKS: &[&str] = &[
        "apt-get install -y oldpkg\napt-get install -y nginx\n",
        "apt-get install -y oldpkg\napt-get install -y badpkg\napt-get install -y nginx\n",
    ];

    /// Build a plan for an install-only book, answering `package:<entity>#installed` with
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut arena,
        )
        .value;
        // Every establish-bearing node footprints its own coordinate (the coherent shape).
        let footprints = self_footprints.then(|| {
            let mut tf = TrustedFootprints::new();
            for (node, class) in &classes {
                let fact = match class {
                    SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => *f,
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
        build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &BTreeSet::new(),
            footprints.as_ref(),
            None,
            &Dialect::empty(),
            &BTreeMap::new(),
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            observe,
            &mut arena,
        )
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
            // And the survivor DEMOTES (the honest baseline: no footprint ⇒ no survival).
            assert!(
                matches!(
                    find(&none, "install -y nginx").disposition,
                    Disposition::Run
                ),
                "unflagged: the converged nginx demotes past the running wall on {src:?}"
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut arena,
        )
        .value;
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
        build_plan_walled(
            src,
            &parsed.value,
            &cfg,
            &classes,
            &BTreeSet::new(),
            Some(&empty),
            None,
            &Dialect::empty(),
            &BTreeMap::new(),
            &vouch_all(&classes),
            &ConnectedPipes::default(),
            observe,
            &mut arena,
        )
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
                assert_eq!(witness.crossings().len(), 1, "one wall crossed");
            }
            other => panic!("nginx must SURVIVE (Replace) past the disjoint wall, got {other:?}"),
        }
    }

    // (The non-disjoint HIT direction — a footprint intersecting the backing demotes even
    // flagged — is pinned by `survival::tests::poisoned_backing_demotes` + the
    // `strawman24-nonsurvive-hit` e2e case; no plan-level duplicate here.)

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
        // EstablishAmbient and Converged ⇒ Replace.
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
        // purge's `curl#installed` reaches the install's in-state via the back-edge as a
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
        // All converged + self-reached + quiet ⇒ license.
        assert!(
            ReplaceLicense::prove_members_replaceable(
                &family,
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
                &family,
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
                &family,
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
                &family,
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
        let classes = dorc_analysis::effect::classify(
            &cfg,
            &value,
            &parsed.value,
            &idx,
            &checks,
            &BTreeSet::new(),
            &mut i,
            &mut dorc_core::ProvArena::new(),
        )
        .value;
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
    fn ship_all_real(_p: Symbol, _a: &[Symbol]) -> Option<StageShip> {
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
        let ship = |p: Symbol, _a: &[Symbol]| {
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
        let ship = |p: Symbol, _a: &[Symbol]| {
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
                entry: None,
            }],
            unresolvable: Vec::new(),
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
                connected: None,
                verdict: false,
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
}
