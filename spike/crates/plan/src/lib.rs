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
    clippy::format_push_string,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use core::marker::PhantomData;
use std::collections::{BTreeMap, BTreeSet};

use dorc_analysis::cfg::{Cfg, CfgNodeId, CfgNodeKind};
use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_analysis::lattice::{May, Powerset};
use dorc_core::{
    AstId, Channel, EntityRef, Grade, Interner, KindId, Observable, Predicted, Rc, SelectorId,
    Verdict,
};
use dorc_syntax::ast::{Ast, NodeKind};
use dorc_syntax::sem;

mod fold;
pub use fold::{AbstractRc, FoldResult};

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
    ///    * `StatusRenderFloor` (an `if`/`elif` guard) — blocks the license
    ///      **unconditionally**. The line-granular render cannot substitute a guard on its
    ///      `if`/`then`/`fi` line, so even a declared rc cannot be applied in-situ (the
    ///      disposition would be sound, but the render breaks `dash -n`). The block is the
    ///      render floor; full retirement waits on the leaf-exact render (`C-5`).
    ///    * `StatusRelaxable` (a `&&`/`||` left operand, an errexit-region command, or a
    ///      `$?`-reader's predecessor — the four `206` §3 sources) — blocks **only when
    ///      the rc is ⊤** (`status == Predicted::Top`): then the stand-in would default to
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
    #[must_use]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "by-value verdict/consumed keeps this minting API a clean owned-args boundary; not widened speculatively (plan/CLAUDE.md)"
    )]
    pub fn prove_replaceable<P: Bias>(
        class: &SkipClass,
        grade: Grade,
        verdict: PhasedVerdict<P>,
        consumed: May<Powerset<Channel>>,
        status: Predicted<Rc>,
    ) -> Option<ReplaceLicense> {
        match class {
            SkipClass::EstablishAmbient(fact) => {
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
    ///    `Stderr` is consumed, or whose status is an `if`/`elif` guard
    ///    (`StatusRenderFloor`), still blocks. A `StatusRelaxable`-consumed status with a
    ///    *known* rc relaxes (the whole point — the fold reads the exact rc, substitutes it).
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
            },
        })
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
/// * `StatusRenderFloor` (an `if`/`elif` guard) — blocks unconditionally (the render
///   floor: the line-granular render cannot substitute a guard on its `if`/`then`/`fi`
///   line; retired only by a guard-capable leaf-exact render, not by the rc value);
/// * `StatusRelaxable` (the four `206` §3 sources: a `&&`/`||` left operand, an
///   errexit-region command, or a `$?`-reader's predecessor) — blocks ONLY when the rc is
///   ⊤ (a fabricated rc-0 `true` would suppress a `|| fallback`, the `kFAIL-perform`
///   under-execute); a known/probe-sourced rc relaxes it (`StandIn::from_rc` reproduces
///   the exact status).
///
/// Sound in BOTH phases; only what a blocked leaf *becomes* is phase-keyed (the
/// caller's collapse, `inv-superposition`).
fn consumption_ok(consumed: &May<Powerset<Channel>>, status: Predicted<Rc>) -> bool {
    let May(consumed) = consumed;
    if consumed.contains(&Channel::Stdout) || consumed.contains(&Channel::Stderr) {
        return false;
    }
    if consumed.contains(&Channel::StatusRenderFloor) {
        return false;
    }
    if consumed.contains(&Channel::StatusRelaxable) && matches!(status, Predicted::Top) {
        return false;
    }
    true
}

// ===========================================================================
// The plan: per-leaf run/skip + render-back-to-sh (the leaf-seam, dn-3)
// ===========================================================================

/// A stable identifier for one executable leaf in a plan (`dn-3`, the leaf-seam):
/// executable work is a list of *individually wrappable* leaves, each with a
/// stable back-map to its source — NEVER one opaque `sh -c "$bigscript"`. The
/// back-map is [`Step::ast`]; the id is this leaf's position in source order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeafId(pub u32);

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

    /// The sh that reproduces the status. `(exit n)` runs in a subshell so a
    /// non-zero stand-in sets `$?` without aborting the script (a bare `exit n`
    /// would terminate it).
    #[must_use]
    pub fn sh(self) -> String {
        match self {
            StandIn::True => "true".to_string(),
            StandIn::False => "false".to_string(),
            StandIn::Exit(n) => format!("(exit {n})"),
        }
    }
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
    /// possibly-stale guard against an omitted body (`render_apply`).
    Omit { controller: AstId },
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

/// A whole-book plan: an ordered list of leaf [`Step`]s (the leaf-seam — never a
/// single opaque script). Render with [`render_sh`](Plan::render_sh).
#[derive(Debug, Clone)]
pub struct Plan {
    pub steps: Vec<Step>,
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

/// What kind of site a [`ProbeCheck`] is — the discriminant the wrong-concrete
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

/// One read-only check the probe ships for a **command site**: the oracle's verbatim
/// probe-sh plus the site's full verbatim argv (C-1), wrapped so the rendered probe,
/// when run, emits a results-record per site (`inv-site-keyed-results`).
///
/// `sh` is the resolved `oracle_probe_*` body (a brace-group taking the entity as
/// `$1`) — the SIMPLER sanctioned emitted-function shape (205 §1 / st-2, 20B §3): a
/// wrapper per `(kind, selector)` cell from the real declared probe
/// ([`dorc_oracle::KindIndex::resolve_probe`] resolves per-selector-else-kind-default,
/// task-P/find-1), invoked per-site with the resolved entity bound. NOT the check's
/// argparse skeleton (the placeholder check probe-bodies must not ship — `pkgindex`'s
/// tautological `test -n fresh` is the named hazard, 20B §3); the `<provider>__check`
/// argparse is the engine's entity-resolver, never the shipped probe body.
///
/// `site` is the stable [`LeafId`] (== the apply plan's leaf-id for the same source
/// command), so the results-record keys back to exactly this program point. `fact` is
/// the resolved cell (display/provenance + the cli's site→fact verdict re-key).
#[derive(Debug, Clone)]
pub struct ProbeCheck {
    /// The stable command-site identity (`inv-site-keyed-results`): the same
    /// [`LeafId`] the apply plan assigns the source command. Two same-command sites
    /// carry distinct ids.
    pub site: LeafId,
    /// The resolved cell this site establishes or queries (the probe checks whether
    /// it holds).
    pub fact: FactKey,
    /// Establish-class or Query-class — the firewall discriminant ([`ProbeSiteKind`]).
    pub site_kind: ProbeSiteKind,
    /// The oracle's `oracle_probe_<kind>` body (a brace-group), shipped verbatim.
    pub sh: String,
}

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
    pub checks: Vec<ProbeCheck>,
    /// The un-resolvable sites' ids (rendered as `skip-unresolvable` comments).
    pub unresolvable: Vec<LeafId>,
}

/// The check-function name for a probed cell: `<kind>_<selector>__check` (task-P /
/// find-1; the strawman's `id__check` shape evolved — `notes/197` §2). The scheme is
/// keyed per `(kind, selector)` so a multi-selector kind shipping DISTINCT probe bodies
/// (`service` via `is-enabled` for `#enabled`, `is-active` for `#active`) emits two
/// distinct, non-colliding wrappers (`service_enabled__check` / `service_active__check`).
/// Both segments route through the hyphen↔underscore funcname mapping
/// ([`dorc_oracle::to_funcname_segment`]) so a hyphenated kind/selector (`package-index`)
/// yields a valid POSIX function name. Resolving the names for the shipped artifact is
/// referent-agnostic (passed through to the host, never branched on).
fn check_fn_name(interner: &Interner, kind: KindId, selector: SelectorId) -> String {
    format!(
        "{}_{}__check",
        dorc_oracle::to_funcname_segment(interner.resolve(kind.0)),
        dorc_oracle::to_funcname_segment(interner.resolve(selector.0)),
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
    /// Emitted-function shape (205 §1 / st-2 ruling, task-P/find-1 per-selector): one
    /// `<kind>_<selector>__check()` wrapper per **`(kind, selector)`** cell (first-seen
    /// ⇒ deterministic), built from the resolved `oracle_probe_*` body
    /// ([`dorc_oracle::KindIndex::resolve_probe`] picks per-selector-else-kind-default),
    /// invoked **per site** with the resolved entity bound (`$1`). Keying per cell (not
    /// per kind) is the find-1 fix: a multi-selector kind ships DISTINCT probe bodies
    /// per selector (`service_enabled__check` runs `is-enabled`, `service_active__check`
    /// runs `is-active`), which a per-kind name would collide. A
    /// [`EntityRef::Singleton`] fact (no operand, `package-index#fresh`) is invoked with
    /// no args. The wrapper captures `$?` immediately after the check, maps it to the
    /// three-outcome word, and prints the record.
    #[must_use]
    pub fn render_sh(&self, interner: &Interner) -> String {
        let mut out = String::from(PROBE_HEADER);
        // Each `(kind, selector)` check fn is emitted once (first-seen ⇒ deterministic),
        // then invoked per SITE with the operand bound + the self-report wrapper. Keying
        // the dedup per CELL (not per kind) is task-P/find-1: a multi-selector kind ships
        // DISTINCT probe bodies per selector (`service` is-enabled vs is-active), so two
        // such bodies must NOT collide under one `<kind>__check` name.
        let mut defined = BTreeSet::new();
        for check in &self.checks {
            let fn_name = check_fn_name(interner, check.fact.kind, check.fact.selector);
            out.push_str(&format!(
                "# site {}: {}\n",
                check.site.0,
                fact_label(interner, check.fact)
            ));
            if defined.insert((check.fact.kind, check.fact.selector)) {
                // body is a brace-group, so `name() <group>` is a valid POSIX funcdef.
                out.push_str(&format!("{fn_name}() {}\n", check.sh));
            }
            // The invocation: run the check (operand bound + single-quoted, F-QUOTE),
            // capture its rc, map to the three-outcome word, emit the site-keyed record.
            // `_e` chosen as a probe-local var name unlikely to clash with a check body.
            let invocation = match check.fact.entity {
                EntityRef::Operand(tok) => {
                    // F-QUOTE: single-quote the operand so it is exactly one inert arg
                    // in any sh (`sem::single_quote`, `inv-kfail` both directions).
                    format!("{fn_name} {}", sem::single_quote(interner.resolve(tok.0)))
                }
                EntityRef::Singleton => fn_name,
            };
            out.push_str(&format!(
                "{invocation}; _rc=$?; \
                 if [ \"$_rc\" -eq 0 ]; then _e=holds; \
                 elif [ \"$_rc\" -eq 1 ]; then _e=absent; \
                 else _e=cant-tell; fi; \
                 printf 'site {} effect=%s rc=%s\\n' \"$_e\" \"$_rc\"\n",
                check.site.0
            ));
        }
        // Un-resolvable sites are recorded as comments (never invoked): transparency
        // for the human reading the artifact and the D3 argv-echo differential.
        for site in &self.unresolvable {
            out.push_str(&format!("# site:{} skip-unresolvable\n", site.0));
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

/// The probe artifact's header — documents the results-record grammar (205 §2
/// rule-probe-exec-gate consumers, and the human reading the artifact, depend on it).
///
/// `stdout=`/`stderr=` are RESERVED record keys (`19F` §3 one-Observable tuple): the cli
/// parser accepts-and-stores them ([`crate`]'s consumer, `parse_results`), but PRODUCING
/// them is FUTURE WORK — this probe emits only `effect=`/`rc=`, so the EMITTED header text
/// stays unchanged (the reserved keys live in the cli parser's doc + the record type, not
/// in the shipped artifact bytes — which keeps every golden byte-identical). A consumed
/// `Stdout`/`Stderr` blocks elision unconditionally regardless (16F §3), so reserving the
/// keys is a SHAPE completion, not a behavior change.
///
/// Wrapper naming (task-P/find-1, documented here per the artifact-header convention,
/// kept out of the EMITTED bytes to honor zero-extra-golden-churn — same posture 20H
/// took for the reserved keys): each probed cell's wrapper is named
/// `<kind>_<selector>__check` ([`check_fn_name`]), one definition per `(kind, selector)`.
/// The selector segment is what lets a multi-selector kind ship two distinct bodies
/// without collision.
const PROBE_HEADER: &str = "#!/bin/sh\n\
    # dorc probe (read-only): checks per-SITE convergence, mutates nothing.\n\
    # When run, emits one results-record per site on stdout (the return channel):\n\
    #   site <leafid> effect=<holds|absent|cant-tell> rc=<n>\n\
    # effect is derived from the probe command's rc (0=holds, 1=absent, else cant-tell);\n\
    # rc is the raw PROBE-command status (opaque to Dorc — the record is the out-of-band lane).\n\n";

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

/// Compile the probe from the analysis result, keyed by command **site**
/// (`inv-site-keyed-results`): each [`SkipClass::EstablishAmbient`] site whose kind
/// has a declared read-only probe (`probe_body`, the oracle seam threaded by the
/// caller so `plan` need not depend on `oracle`) becomes one [`ProbeCheck`] with the
/// site's stable [`LeafId`]; every other site is recorded as `unresolvable`. Two
/// same-command resolvable sites yield two distinct checks (distinct ids) — the
/// per-fact dedup of spike-2 is gone.
///
/// `ast`/`cfg` are threaded only to compute the shared site-id ordering ([`site_order`]
/// — the same one [`build_plan`] uses), so the probe's site-ids equal the apply plan's
/// leaf-ids. Deterministic, non-mutating; the FORWARD half of the compiler (the apply
/// is [`build_plan`]). A kind without a probe yields no check ⇒ its site cannot be
/// elided downstream (`can't-probe ⇒ can't-elide`).
#[must_use]
pub fn compile_probe(
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    probe_body: impl Fn(KindId, SelectorId) -> Option<String>,
) -> ProbePlan {
    let mut checks = Vec::new();
    let mut unresolvable = Vec::new();
    for (site, _node, class) in site_order(ast, cfg, classes) {
        // Both an EstablishAmbient and a (resolvable) Query site ship a check — each
        // is probe-resolvable iff its `(kind, selector)` cell resolves to a declared
        // probe (task-P/find-1: a per-selector probe, or the kind-default ONLY when the
        // kind is single-selector — `KindIndex::resolve_probe`). The `site_kind`
        // discriminant rides along so the cli's firewall knows whether the record-rc is
        // the probe's (Establish ⇒ never fold) or the guard's own (Query ⇒ fold iff
        // valid). A written establish, a kill, opaque, pure, MustRun — none resolvable
        // (`can't-probe ⇒ can't-elide`, `kFAIL-perform`).
        let resolvable = match class {
            SkipClass::EstablishAmbient(fact) => Some((*fact, ProbeSiteKind::Establish)),
            SkipClass::QueryResolvable { fact, valid } => {
                Some((*fact, ProbeSiteKind::Query { valid: *valid }))
            }
            _ => None,
        };
        match resolvable {
            // A multi-selector kind site whose specific selector has no per-selector
            // probe resolves to `None` here (the F-BLESSED floor) ⇒ un-checkable ⇒ runs.
            Some((fact, site_kind)) => match probe_body(fact.kind, fact.selector) {
                Some(sh) => checks.push(ProbeCheck {
                    site,
                    fact,
                    site_kind,
                    sh,
                }),
                None => unresolvable.push(site),
            },
            None => unresolvable.push(site),
        }
    }
    ProbePlan {
        checks,
        unresolvable,
    }
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
#[must_use]
pub fn build_plan(
    src: &str,
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    observe: impl Fn(FactKey) -> Observable,
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
                SkipClass::MustRun => return None,
            };
            Some((cfg.node(*node).ast, fact))
        })
        .collect();

    // Run the apply fold. A leaf's fold-status is its injected observation; a leaf
    // with no fact (MustRun / opaque / query without an oracle effect) is ⊤ ⇒ no fold
    // through it (`inv-kfail`).
    let fold = fold::fold(ast, |leaf| leaf_fact.get(&leaf).map(|f| observe(*f)));

    let mut steps: Vec<Step> = classes
        .iter()
        .map(|(node, class)| {
            let ast_id = cfg.node(*node).ast;
            let sh = command_text(src, ast, ast_id);
            let observed = match class {
                SkipClass::EstablishAmbient(f)
                | SkipClass::EstablishWritten(f)
                | SkipClass::QueryResolvable { fact: f, .. } => Some(observe(*f)),
                SkipClass::MustRun => None,
            };
            let disposition = disposition_for(cfg, &fold, *node, class, ast_id, observed);
            Step {
                leaf: LeafId(0),
                ast: ast_id,
                sh,
                disposition,
            }
        })
        .collect();

    // Source order (classify yields CFG-alloc order; sort by span for a faithful
    // reading), then assign stable leaf ids. This MUST stay byte-identical to
    // [`site_order`]'s sort+enumerate: the probe's site-ids and these leaf-ids are ONE
    // id space (`inv-site-keyed-results`), so a record `site N …` keys back to leaf N.
    // `probe_site_id_equals_plan_leaf_id` pins the equivalence.
    steps.sort_by_key(|s| (ast.node(s.ast).span.lo.0, ast.node(s.ast).span.hi.0));
    for (i, step) in steps.iter_mut().enumerate() {
        step.leaf = LeafId(u32::try_from(i).unwrap_or(u32::MAX));
    }
    Plan { steps }
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
) -> Disposition {
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
            match ReplaceLicense::prove_replaceable(class, Grade::Must, verdict, consumed, status) {
                Some(license) => {
                    // The value-preserving stand-in reproduces the predicted Status channel.
                    // An unpredicted status (`Predicted::Top`) falls back to the conforming
                    // `true` (rc 0) — reached ONLY for a converged-establish whose status is
                    // not branch-consumed (`prove_replaceable` blocks a branch-consumed `Top`,
                    // `19D`; a Query guard always carries a known rc, never ⊤). So the rc-0
                    // placeholder is never read by a branch.
                    let stand_in = match status {
                        Predicted::Value(rc) => StandIn::from_rc(rc),
                        Predicted::Top => StandIn::True,
                    };
                    Disposition::Replace(license, stand_in)
                }
                None => Disposition::Run,
            }
        }
        _ => Disposition::Run,
    }
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

impl Plan {
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
        let mut out = String::from(
            "#!/bin/sh\n# dorc plan (apply phase). Replaced leaves are already converged.\n\n",
        );
        for step in &self.steps {
            match &step.disposition {
                Disposition::Run => {
                    out.push_str(&step.sh);
                    out.push('\n');
                }
                Disposition::Replace(license, stand_in) => {
                    out.push_str(&format!(
                        "# replace[{}]: {}  (\u{2192} {})\n#   \u{21b3} {} already holds (probe: converged \u{b7} must \u{b7} ambient)\n",
                        step.leaf.0,
                        step.sh,
                        stand_in.sh(),
                        fact_label(interner, license.fact()),
                    ));
                }
                Disposition::Omit { .. } => {
                    out.push_str(&format!(
                        "# omit[{}]: {}\n#   \u{21b3} dead branch: a guard's known status proves it never runs\n",
                        step.leaf.0, step.sh,
                    ));
                }
            }
        }
        out
    }

    /// Render the apply as the ORIGINAL book with elided (`Replace`) command-lines
    /// replaced by their no-op stand-in — "a copy of book.sh with the safe-to-omit
    /// lines commented", the CLI's final artifact. Line-granular (the spike's books
    /// are ~one-command-per-line): a source line is elided iff a `Replace` leaf lies
    /// on it and no `Run` leaf does. Everything else (guards, blanks, comments,
    /// multi-leaf lines) passes verbatim, so the output keeps the original control
    /// flow (contrast [`render_sh`](Plan::render_sh), the flat leaf-list).
    ///
    /// `ap-2` / `an-render-runnable` (`notes/193` strain-6 + `19C`): a neutralised
    /// line emits a provenance comment **then its value-preserving [`StandIn`]** at the
    /// original indentation — `true` (rc 0), `false` (rc 1), `(exit n)` (other), or `:`
    /// for a wholly-dead (`Omit`) line whose status is unreachable. The stand-in is the
    /// substitution *itself*, not filler: a `Replace` reproduces the leaf's observed
    /// status (`19A §5`); a comment *alone* deletes the command, so commenting the lone
    /// body of an `if`/`while`/`case` arm leaves an empty clause — a `sh -n` syntax
    /// error — which the stand-in (valid in *every* context a command was) prevents.
    ///
    /// Two new gates over the round-16 line-render:
    /// * **value-preserving stand-in** — a line's stand-in reproduces its folded exit
    ///   status (the surviving `Replace` leaf's, or the sequence's last), not a blanket
    ///   `:`. This is what makes `useradd[rc9] || mkdir` safe end-to-end: were that line
    ///   ever neutralised, its stand-in would be `(exit 9)`, not `true` (so `|| mkdir`
    ///   still fires).
    /// * **omit-safety** — an `Omit` (fold-dead) leaf is neutralised **only when its
    ///   controlling guard is itself neutralised** (`is_neutralised`). If the guard is
    ///   kept (`Run` — e.g. an `if`/`elif` guard held by `mark_status`, which the
    ///   line-render cannot substitute in-situ), omitting the body would let the kept,
    ///   possibly-stale guard re-decide against a removed body (a `kFAIL-perform`
    ///   under-execute). So such an `Omit` body is rendered **verbatim** (it runs; the
    ///   runtime guard gates it — the F1 floor). Coherent in-situ guard substitution is
    ///   the deferred leaf-exact / structural render (`C-5`/`seam-prov`).
    #[must_use]
    pub fn render_apply(&self, src: &str, ast: &Ast) -> String {
        let line_of = |byte: u32| -> usize {
            src.get(..byte as usize)
                .map_or(0, |s| s.bytes().filter(|&b| b == b'\n').count())
        };
        // Byte offset of each source line's first byte (index = line number). Lets an
        // absolute leaf span be mapped to an in-line byte column for the in-situ
        // substitution (the T14 case-arm path below).
        let line_start: Vec<usize> = std::iter::once(0)
            .chain(
                src.bytes()
                    .enumerate()
                    .filter_map(|(i, b)| (b == b'\n').then_some(i + 1)),
            )
            .collect();
        // Per-AstId disposition, so an `Omit`'s controller can be resolved for the
        // omit-safety gate.
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();

        // The T14 fix (leaf-exact case-arm render): leaves that are the body of a
        // one-liner `case` arm (`pat) cmd ;;` all on one line) cannot be whole-line
        // commented without swallowing the `pat)`/`;;` scaffolding. They are substituted
        // IN-SITU instead (only the command span is replaced). Detected AST-structurally.
        let arm_inline_leaves = case_arm_oneliner_leaves(ast, &line_of);

        // Per source line: does a Run leaf sit on it (⇒ verbatim)? and the surviving
        // value-preserving stand-in for a neutralised line.
        let mut run_lines: BTreeSet<usize> = BTreeSet::new();
        let mut neutral_lines: BTreeSet<usize> = BTreeSet::new();
        let mut line_standin: BTreeMap<usize, StandIn> = BTreeMap::new();
        // line → (in-line byte start, in-line byte end, stand-in) for an in-situ
        // case-arm substitution. The whole-line path skips these lines.
        let mut inline_subst: BTreeMap<usize, (usize, usize, StandIn)> = BTreeMap::new();
        for step in &self.steps {
            let span = ast.node(step.ast).span;
            let last_byte = span.hi.0.saturating_sub(1).max(span.lo.0);
            let lines: Vec<usize> = (line_of(span.lo.0)..=line_of(last_byte)).collect();
            match &step.disposition {
                Disposition::Run => run_lines.extend(&lines),
                // A one-liner case-arm body `Replace`: substitute in-situ (keep `pat)`/`;;`).
                Disposition::Replace(_, stand_in) if arm_inline_leaves.contains(&step.ast) => {
                    let l = line_of(span.lo.0);
                    let lo = line_start.get(l).copied().unwrap_or(0);
                    let start = (span.lo.0 as usize).saturating_sub(lo);
                    let end = (span.hi.0 as usize).saturating_sub(lo);
                    inline_subst.insert(l, (start, end, *stand_in));
                }
                Disposition::Replace(_, stand_in) => {
                    for l in &lines {
                        neutral_lines.insert(*l);
                        // A `Replace` leaf's stand-in is the line's surviving value
                        // (the short-circuit survivor / sequence tail). Last writer in
                        // source order wins (the sequence tail).
                        line_standin.insert(*l, *stand_in);
                    }
                }
                Disposition::Omit { controller } => {
                    if is_neutralised(&by_ast, *controller, 0) {
                        // The guard is neutralised ⇒ safe to omit the dead body. A dead
                        // body is unreachable, so it contributes NO status — its line's
                        // stand-in stays whatever a surviving `Replace` set, else `:`.
                        neutral_lines.extend(&lines);
                    } else {
                        // The guard is kept (`Run`) ⇒ the F1 floor: render the body
                        // verbatim (it runs; the runtime guard gates it). Treat as Run.
                        run_lines.extend(&lines);
                    }
                }
            }
        }
        let mut out = String::from(
            "#!/bin/sh\n# dorc apply: the book, with already-converged/dead lines elided (value-preserving stand-in).\n\n",
        );
        for (i, line) in src.lines().enumerate() {
            if let Some((start, end, stand_in)) = inline_subst.get(&i).copied()
                && !run_lines.contains(&i)
            {
                // T14 in-situ: keep the `pat)` prefix and ` ;;` suffix, replace only the
                // command span with its value-preserving stand-in (`nginx) true ;;`). The
                // provenance comment trails the arm (a `# …` after `;;` is a valid arm end).
                let prefix = line.get(..start).unwrap_or(line);
                let suffix = line.get(end..).unwrap_or_default();
                out.push_str(prefix);
                out.push_str(&stand_in.sh());
                out.push_str(suffix);
                out.push_str(
                    "   # dorc: elided (already converged) — case-arm body substituted in situ\n",
                );
            } else if neutral_lines.contains(&i) && !run_lines.contains(&i) {
                let indent: String = line
                    .chars()
                    .take_while(|c| *c == ' ' || *c == '\t')
                    .collect();
                // A surviving `Replace` leaf reproduces the line's exact status; a
                // wholly-dead (`Omit`-only) line is unreachable code, so `:` (a pure
                // structural placeholder — status never observed) is the honest filler.
                let filler = match line_standin.get(&i) {
                    Some(stand_in) => stand_in.sh(),
                    None => ":".to_string(),
                };
                out.push_str("# ");
                out.push_str(line.trim_start());
                out.push_str("   # dorc: elided (already converged / dead branch)\n");
                out.push_str(&indent);
                out.push_str(&filler);
                out.push('\n');
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}

/// Is `leaf` neutralised (its line will be commented out)? Used by `render_apply`'s
/// omit-safety gate: an `Omit` body may only be neutralised if its controlling guard
/// also is. A `Replace` is neutralised; a `Run` is not; an `Omit` is iff *its*
/// controller is (transitively, with a small depth cap to defeat any pathological
/// cycle — `inv-no-throw`). A missing controller folds to "not neutralised" (the safe
/// run-it direction).
fn is_neutralised(by_ast: &BTreeMap<AstId, &Disposition>, leaf: AstId, depth: u32) -> bool {
    if depth > 64 {
        return false; // defensive: never loop; default to run-it
    }
    match by_ast.get(&leaf) {
        Some(Disposition::Replace(_, _)) => true,
        Some(Disposition::Omit { controller }) => is_neutralised(by_ast, *controller, depth + 1),
        _ => false, // Run, or an un-classified controller ⇒ not neutralised
    }
}

/// The leaf [`AstId`]s that are a body command of a **one-liner `case` arm** — an arm
/// whose pattern (`pat)`) and whose body command sit on the SAME source line (the T14
/// render defect, `notes/199` cluster-C). Such a leaf cannot be whole-line-commented:
/// the comment would also swallow the structural `pat)` / `;;` scaffolding, leaving an
/// arm with no `pat)` (a `dash -n` syntax error). `render_apply` instead substitutes
/// these leaves IN-SITU on the line (replacing only the command span), keeping the
/// arm structure intact (`nginx) true ;;`).
///
/// Detection is AST-structural (not text-scanning for `)`/`;;`, which a command's own
/// `)` would defeat): walk every [`NodeKind::Case`] arm, and if a body-`List` item's
/// line equals the arm's first-pattern line, that item is a same-line arm body. Only
/// the *direct* body items are collected — a leaf nested in a sub-group keeps the
/// whole-line form (it has its own enclosing tokens, not the arm's). Scoped this
/// narrowly so the ordinary whole-line path is untouched (zero golden churn elsewhere).
fn case_arm_oneliner_leaves(ast: &Ast, line_of: &impl Fn(u32) -> usize) -> BTreeSet<AstId> {
    let mut leaves = BTreeSet::new();
    for (_id, node) in ast.iter() {
        let NodeKind::Case { arms, .. } = &node.kind else {
            continue;
        };
        for arm in arms {
            let Some(&first_pat) = arm.patterns.first() else {
                continue;
            };
            let pat_line = line_of(ast.node(first_pat).span.lo.0);
            // The arm body is always a `List` (the parser wraps even a single command);
            // its direct items are the candidate same-line leaves.
            let NodeKind::List { items } = &ast.node(arm.body).kind else {
                continue;
            };
            for &item in items {
                if line_of(ast.node(item).span.lo.0) == pat_line {
                    leaves.insert(item);
                }
            }
        }
    }
    leaves
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
    use dorc_oracle::{KindIndex, Polarity};

    /// Corpus-shaped check dialect for the pipeline tests: the `apt-get` check
    /// (flag-strip → verb → `update` Singleton arm `package-index`; else single-operand
    /// `package` with a `[ "$2" = "" ]` multi-operand refusal). Annotation kinds match
    /// the effect-map's, so the kind-agreement rule never fires. Lifted with the test's
    /// interner so provider symbols match the book's command words (204 seam #2).
    const CORPUS_CHECK_SRC: &str = r#"
apt_get__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      update) idx : package-index; probe-fresh ;;
      *)
         while [ "${1#-}" != "$1" ]; do shift; done
         pkg : package = "$1"
         if [ "$2" = "" ]; then probe-pkg "$pkg"; fi ;;
   esac
}
"#;

    /// `package:nginx#installed` — the cell `apt-get install nginx` gates. The
    /// re-key (`notes/193`) made the entity an [`EntityRef`] and added a selector.
    fn nginx_fact() -> FactKey {
        let mut i = Interner::default();
        FactKey {
            kind: KindId(i.intern("package")),
            entity: EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
            selector: SelectorId(i.intern("installed")),
        }
    }

    /// An empty (provably-quiet) consumption fact in the `May` orientation — the
    /// common case for the `prove_replaceable` unit tests.
    fn quiet() -> May<Powerset<Channel>> {
        May(Powerset::default())
    }

    /// Run the real pipeline (parse → cfg → value-flow → classify → `compile_probe`) on
    /// `src`, with `probe_body` supplying each kind's `oracle_probe_*` body. Returns the
    /// site-keyed [`ProbePlan`] + the interner (for `render_sh`). The corpus apt check
    /// resolves identity; `probe_body` decides which kinds are probeable. This is the
    /// honest site-keyed shape (`inv-site-keyed-results`): the synthetic-`CfgNodeId`
    /// fact-keyed tests of spike-2 could not exercise `site_order`.
    ///
    /// `probe_body` is kind-keyed for these tests (every modeled kind here is
    /// single-selector, so the per-selector resolution rule is exercised at the oracle
    /// layer, not here); we adapt it to `compile_probe`'s `(kind, selector)` closure.
    fn probe_for_src(
        src: &str,
        probe_body: impl Fn(KindId) -> Option<String>,
    ) -> (ProbePlan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::check::lift_checks(&mut i, CORPUS_CHECK_SRC).value];
        let classes = dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut i).value;
        let probe = compile_probe(&parsed.value, &cfg, &classes, |k, _sel| probe_body(k));
        (probe, i)
    }

    #[test]
    fn compile_probe_resolvable_sites_probed_unresolvable_recorded() {
        // The probe = EstablishAmbient sites WITH a declared read-only probe. A site
        // whose kind has an effect but NO probe is un-checkable ⇒ NOT invoked, recorded
        // `unresolvable` (can't-probe ⇒ can't-elide, kFAIL-perform). A MustRun site
        // (the un-oracled `systemctl reload`) is likewise unresolvable. Here only
        // `package` has a probe, so `install nginx` is the one resolvable site; the
        // reload is unresolvable.
        let package = {
            let mut i = Interner::default();
            KindId(i.intern("package"))
        };
        let (probe, _i) =
            probe_for_src("apt-get install -y nginx\nsystemctl reload nginx\n", |k| {
                (k == package).then(|| "dpkg-query -W \"$1\"".to_string())
            });
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
        let (probe, _i) = probe_for_src("apt-get install -y nginx\n", |_k| None);
        assert!(probe.checks.is_empty(), "no probe ⇒ no resolvable site");
        assert_eq!(
            probe.unresolvable,
            vec![LeafId(0)],
            "the un-probeable site is recorded: {probe:?}"
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
        let package = {
            let mut i = Interner::default();
            KindId(i.intern("package"))
        };
        let (probe, i) = probe_for_src(
            "apt-get install -y nginx\napt-get install -y nginx\n",
            |k| (k == package).then(|| "{ dpkg-query -W \"$1\"; }".to_string()),
        );
        assert_eq!(probe.checks.len(), 1, "site 0 resolvable (ambient)");
        assert_eq!(probe.checks[0].site, LeafId(0));
        assert_eq!(
            probe.unresolvable,
            vec![LeafId(1)],
            "site 1 is a DISTINCT site, recorded unresolvable (same-cell Written), not collapsed"
        );
        let rendered = probe.render_sh(&i);
        assert!(
            rendered.contains("printf 'site 0 effect="),
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
        let package = {
            let mut i = Interner::default();
            KindId(i.intern("package"))
        };
        let (probe, i) =
            probe_for_src("apt-get install -y nginx\napt-get install -y curl\n", |k| {
                (k == package).then(|| "{ dpkg-query -W \"$1\"; }".to_string())
            });
        assert_eq!(probe.checks.len(), 2, "two distinct-cell sites");
        assert_eq!(probe.checks[0].site, LeafId(0));
        assert_eq!(probe.checks[1].site, LeafId(1));
        assert_ne!(
            probe.checks[0].fact, probe.checks[1].fact,
            "distinct cells (nginx vs curl)"
        );
        let rendered = probe.render_sh(&i);
        assert!(
            rendered.contains("printf 'site 0 effect="),
            "site 0 record:\n{rendered}"
        );
        assert!(
            rendered.contains("printf 'site 1 effect="),
            "site 1 record:\n{rendered}"
        );
    }

    #[test]
    fn probe_render_self_reports_and_binds_operand() {
        // The WIRE: the rendered probe is SELF-REPORTING — each resolvable site invokes
        // the kind's `oracle_probe_*` wrapper (defined ONCE per kind, F-QUOTE-bound
        // operand) and emits `site <id> effect=… rc=…` on stdout. The Singleton site
        // (`apt-get update` ⇒ pkgindex#fresh) is invoked with no operand. The emitted
        // wrapper body is the DECLARED probe, NOT the check's argparse skeleton (st-2):
        // the placeholder `test -n fresh` etc. must not ship — here we feed real bodies.
        let (package, pkgindex) = {
            let mut i = Interner::default();
            (
                KindId(i.intern("package")),
                KindId(i.intern("package-index")),
            )
        };
        // Two package sites (nginx, curl) + one Singleton (update).
        let (probe, i) = probe_for_src(
            "apt-get install -y nginx\napt-get install -y curl\napt-get update\n",
            |k| {
                if k == package {
                    Some("{ dpkg-query -W \"$1\"; }".to_string())
                } else if k == pkgindex {
                    Some(
                        "{ test -n \"$(find /var/lib/apt/lists -newermt '-1 hour')\"; }"
                            .to_string(),
                    )
                } else {
                    None
                }
            },
        );
        let rendered = probe.render_sh(&i);

        // Operand bound + single-quoted (F-QUOTE): the wrapper name is per-(kind,selector)
        // now (task-P) — `package_installed__check 'nginx'` AND 'curl'.
        assert!(
            rendered.contains("package_installed__check 'nginx'"),
            "operand nginx bound + quoted:\n{rendered}"
        );
        assert!(
            rendered.contains("package_installed__check 'curl'"),
            "operand curl bound + quoted:\n{rendered}"
        );
        // The wrapper fn is defined exactly ONCE per (kind, selector) cell (FLAT dedup),
        // two package:#installed sites.
        assert_eq!(
            rendered.matches("package_installed__check() {").count(),
            1,
            "package#installed's check fn defined once, invoked per site:\n{rendered}"
        );
        // Singleton: invoked with NO operand (the bare fn name, then the record wrapper).
        // The hyphenated kind `package-index` maps to funcname `package_index` (task-P).
        assert!(
            rendered.contains("package_index_fresh__check; _rc=$?;"),
            "a Singleton site invokes the check with NO operand:\n{rendered}"
        );
        // Self-reporting: a site-keyed record printf per resolvable site (3 of them).
        assert_eq!(
            rendered.matches("printf 'site ").count(),
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
        let package = {
            let mut i = Interner::default();
            KindId(i.intern("package"))
        };
        let body = |k: KindId| {
            (k == package).then(|| "{ dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string())
        };

        // Spaced operand via a flowed assignment.
        let (probe, i) = probe_for_src("PKG='my pkg'\napt-get install -y \"$PKG\"\n", body);
        let rendered = probe.render_sh(&i);
        assert!(
            rendered.contains("package_installed__check 'my pkg'"),
            "spaced operand single-quoted to one arg:\n{rendered}"
        );

        // Metachar operand: the `;` is INSIDE the quotes, so it cannot split.
        let (probe, i) = probe_for_src(
            "PKG='x; touch /tmp/PWNED'\napt-get install -y \"$PKG\"\n",
            body,
        );
        let rendered = probe.render_sh(&i);
        assert!(
            rendered.contains("package_installed__check 'x; touch /tmp/PWNED'"),
            "metachar operand single-quoted ⇒ the `;` cannot split:\n{rendered}"
        );
        // No UNQUOTED metachar invocation leaked (the `;` only ever appears quoted).
        assert!(
            !rendered.contains("\npackage_installed__check x; touch"),
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
        let checks = vec![dorc_oracle::check::lift_checks(&mut i, CORPUS_CHECK_SRC).value];
        let classes = dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut i).value;

        let package = KindId(i.intern("package"));
        let probe = compile_probe(&parsed.value, &cfg, &classes, |k, _sel| {
            (k == package).then(|| "{ dpkg-query -W \"$1\"; }".to_string())
        });
        let plan = build_plan(src, &parsed.value, &cfg, &classes, |_f| {
            Observable::verdict_only(Verdict::Diverged)
        });

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
        ) else {
            panic!("ambient + must + converged must license a skip");
        };
        assert_eq!(lic.fact(), f);
        assert!(lic.derivation().ambient);
        assert_eq!(lic.derivation().verdict, Verdict::Converged);
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
                )
                .is_some(),
                "`&&`/`||`-consumed status + declared rc {rc:?} licenses (value-preserving)"
            );
        }
    }

    #[test]
    fn render_floor_status_blocks_unconditionally() {
        // `19D` / 19C strain-D / `206` §3: the `if`/`elif`-guard `StatusRenderFloor`
        // blocks the license EVEN with a declared rc (the line-granular render cannot
        // substitute a guard on its `if`/`then`/`fi` line; a declared-rc relaxation
        // would break `dash -n`). Contrast `relaxable_status_blocks_only_when_rc_undeclared`.
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
                    May(Powerset::singleton(Channel::StatusRenderFloor)),
                    rc,
                )
                .is_none(),
                "an if-guard's StatusRenderFloor blocks unconditionally (render floor), rc={rc:?}"
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
        let apt = ProviderId(i.intern("apt-get"));
        let install = i.intern("install");
        let update = i.intern("update");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, installed, Polarity::Establish);
        idx.add_effect(apt, update, package_index, fresh, Polarity::Establish);
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
        };
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let value = dorc_analysis::value::analyze(&cfg, &parsed.value, &mut i);
        let checks = vec![dorc_oracle::check::lift_checks(&mut i, CORPUS_CHECK_SRC).value];
        let classes = dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut i).value;
        let plan = build_plan(src, &parsed.value, &cfg, &classes, |f| {
            // fork-mutator-rc (202 §5 / 206 §3): a MUTATOR's status has no sanctioned
            // source — only its Effect channel (convergence) arrives from the probe, the
            // rc is ⊤. So `verdict_only` everywhere, never a fabricated `Rc(0)`. The
            // earlier `Rc(0)` masked C-3: under `set -e` the install's status is consumed
            // (StatusRelaxable), and a declared rc-0 would relax-and-elide it; with the
            // faithful ⊤-rc it correctly RUNS (see `residual_poison_sources_isolated`).
            if f == target {
                Observable::verdict_only(nginx_verdict)
            } else {
                Observable::verdict_only(Verdict::Unknown)
            }
        });
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
        // install to Written; with neither, it is Ambient (the keystone win). Host
        // verdict is irrelevant here — this is the classify-level ambient gate.
        let ambient = |src: &str| {
            let (plan, _) = plan_for(src, Verdict::Converged);
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, _)
            )
        };
        // Neither neighbour ⇒ ambient ⇒ elides (the clean keystone case).
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
            let checks = vec![dorc_oracle::check::lift_checks(&mut i, CORPUS_CHECK_SRC).value];
            let classes =
                dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut i).value;
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
        // THE keystone win at the PLAN level (`notes/193` strain-5 / acceptance §7.2):
        // with `apt-get update` the ONLY upstream neighbour (modeled, distinct cell)
        // and the host Converged, the install is now `Disposition::Replace` — the
        // poison wall is genuinely dead end-to-end, not just at classify. This is the
        // `update → install` core of the realistic book with the un-oracled scrutinee/
        // guard stripped (the residual poison the full-fixture test documents). Pre-key
        // this was impossible: `update` Opaque ⇒ Top ⇒ install forced Written ⇒ Run.
        let (plan, _) = plan_for(
            "apt-get update\napt-get install -y nginx\n",
            Verdict::Converged,
        );
        assert!(
            matches!(
                find(&plan, "apt-get install").disposition,
                Disposition::Replace(_, _)
            ),
            "modeled `update` (distinct cell) no longer poisons ⇒ converged install elides"
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
        let checks = vec![dorc_oracle::check::lift_checks(&mut i, CORPUS_CHECK_SRC).value];
        let classes = dorc_analysis::effect::classify(&cfg, &value, &idx, &checks, &mut i).value;
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
}
