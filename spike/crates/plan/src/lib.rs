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
    AstId, Channel, EntityRef, Grade, Interner, KindId, Observable, Predicted, Rc, Verdict,
};
use dorc_syntax::ast::Ast;

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

/// Why a replacement was licensed — the audit trail a plan UI greys-out as the "why"
/// (note 165 L2). Readable, but only ever constructed inside
/// [`ReplaceLicense::prove_replaceable`], so every field reflects a checked condition.
#[derive(Debug, Clone)]
pub struct Derivation {
    /// The fact whose established-ness licenses the skip.
    pub fact: FactKey,
    /// `analysis` classified this command [`SkipClass::EstablishAmbient`]: no
    /// upstream same-run mutation reaches it (the W5 ambient gate, note 162 O-1).
    pub ambient: bool,
    /// The fact is oracle-declared [`Grade::Must`] (a mined `May` never licenses —
    /// `inv-must-may`).
    pub grade: Grade,
    /// The host probe found the fact already holds ([`Verdict::Converged`]).
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
    ///    strain-D); both gate a *different* command's reachability, so a *fabricated*
    ///    rc-0 stand-in would destroy that decision. The unvouched set:
    ///    * `Stdout`/`Stderr` — the stub defaults them to empty, vouched by nothing
    ///      (16F §3); a consumed one ⇒ run (no in-spike bridge). A declared rc does
    ///      NOT vouch *output content*, so these block regardless of `observed_rc`.
    ///    * `Status` (an `if`/`elif` guard) — blocks the license **unconditionally**.
    ///      The line-granular render cannot substitute a guard on its `if`/`then`/`fi`
    ///      line, so even a declared rc cannot be applied in-situ (the disposition would
    ///      be sound, but the render breaks `dash -n`). The block is the render floor;
    ///      full retirement waits on the leaf-exact render (`C-5`).
    ///    * `AndOrStatus` (a `&&`/`||` left operand) — blocks **only when the rc is
    ///      undeclared** (`observed_rc == None`): then the stand-in defaults to `true`
    ///      (rc 0), a fabricated success that suppresses a `|| fallback` (the
    ///      `kFAIL-perform` under-execute — the round-19 adversarial trace). A
    ///      *declared* rc relaxes it (`observed_rc == Some(N)` ⇒ the stand-in is
    ///      `StandIn::from_rc(N)`, reproducing the exact status, so the branch decides
    ///      identically — the fold's declared-rc opt-in, `19A §5`). The render CAN
    ///      express this (operand+operator on one line; the fold + omit-safety gate
    ///      handle it). (`tc-mint`/`tc-reliability`: the rc is a *declared observable*,
    ///      not inferred; an un-declared rc on a non-conforming establish is an
    ///      oracle-quality defect — build-2's contract, `19C` strain-B.)
    ///    * Errexit (`set -e`)-consumed status is NOT special-cased (19A C-3, honored
    ///      round-20 / 205 §2): the cfg pass marks errexit-region commands (and `$?`
    ///      readers' predecessors) as `AndOrStatus`-consumed, so they ride the same
    ///      declared-rc-or-block rule above. Under fork-mutator-rc a mutator's rc is
    ///      always ⊤ ⇒ converged mutators under `set -e` run (the 206 §2 headline cost).
    ///
    /// Generic over the phase `P` (`inv-superposition`): the engine never bakes a
    /// phase; the caller argues it. `build_plan` passes the verdict's own provenance
    /// (`Probe`) and the leaf's observed rc.
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
        let SkipClass::EstablishAmbient(fact) = class else {
            return None;
        };
        if grade != Grade::Must {
            return None;
        }
        if verdict.resolve() != Resolved::Replaceable {
            return None;
        }
        // No UNVOUCHED observable consumed downstream. The fact arrives un-collapsed
        // as a `May` (over-approximate consumption): per `inv-must-may` a `May` value
        // can only BLOCK a license, never grant one. (The block is sound in BOTH
        // phases; only what a blocked leaf *becomes* is phase-keyed — the caller's
        // collapse, `inv-superposition`.)
        let May(consumed) = &consumed;
        // `Stdout`/`Stderr`: empty default vouched by nothing (16F §3). A declared rc
        // does not vouch output *content* ⇒ always block.
        if consumed.contains(&Channel::Stdout) || consumed.contains(&Channel::Stderr) {
            return None;
        }
        // `Status` (an `if`/`elif` guard): blocks unconditionally — the render floor
        // (19C strain-D; even a declared rc can't be substituted in-situ on the
        // `if`/`then`/`fi` line). Errexit-consumption does NOT land here — it is
        // marked as the value-relaxing `AndOrStatus` below (19A C-3 / 205 §2).
        if consumed.contains(&Channel::Status) {
            return None;
        }
        // `AndOrStatus` (a `&&`/`||` left operand): blocks ONLY when the rc is undeclared
        // (`19D`). With no declared rc the stand-in defaults to `true` (rc 0) — a
        // fabricated success that suppresses a `|| fallback` (the `kFAIL-perform`
        // under-execute). A declared rc relaxes it: `StandIn::from_rc` reproduces the
        // exact status, so the branch decides as the real command would (`19A §5`).
        if consumed.contains(&Channel::AndOrStatus) && matches!(status, Predicted::Top) {
            return None;
        }
        Some(ReplaceLicense {
            fact: *fact,
            derivation: Derivation {
                fact: *fact,
                ambient: true,
                grade,
                verdict: Verdict::Converged,
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
// ===========================================================================

/// One read-only check the probe ships: "does `fact` already hold?", carried as the
/// oracle's verbatim probe-sh (non-mutating by contract — `kFAIL-withhold`). The
/// host's answer ([`Verdict`]) is what licenses the apply to elide the establishing
/// leaf.
///
/// `sh` is the oracle's `oracle_probe_<kind>` body (a brace-group taking the entity
/// as `$1`). Half-B (`notes/197` §2 / `notes/198`) binds the book's operand into it
/// at render: the FLAT interceptor model — define the kind's check once, invoke it
/// per-entity with the operand bound (the "$1 unbound" Half-A degeneracy is gone).
#[derive(Debug, Clone)]
pub struct ProbeCheck {
    pub fact: FactKey,
    pub sh: String,
}

/// A compiled probe: the read-only fact-checks whose answers drive the apply's
/// elision (apply-2). It holds every [`SkipClass::EstablishAmbient`] fact (the only
/// elidable class — note 162 O-1) whose kind has a *declared* read-only probe. A
/// fact whose kind has NO probe is deliberately ABSENT — it is un-checkable, so the
/// apply cannot elide it (`kFAIL-perform`: no convergence knowledge ⇒ run it). This
/// is the "can't-probe ⇒ can't-elide" link.
#[derive(Debug, Clone, Default)]
pub struct ProbePlan {
    pub checks: Vec<ProbeCheck>,
}

/// The check-function name for a kind: `<kind>__check` (the strawman's `id__check`
/// shape — `notes/197` §2). Resolving the kind name for the shipped artifact is
/// referent-agnostic (it is passed through to the host, never branched on).
fn check_fn_name(interner: &Interner, kind: KindId) -> String {
    format!("{}__check", interner.resolve(kind.0))
}

/// POSIX single-quote a string so it becomes exactly **one** literal argument when
/// the rendered probe is parsed by `sh` — the F-QUOTE fix (`notes/198`, `inv-kfail`
/// both directions). The book operand is interned **post-parse** (quotes already
/// stripped, embedded metachars preserved — `effect::word_literal` reads
/// `WordPart::SingleQuoted`'s inner text), so an operand like `my pkg` or
/// `x; touch /tmp/PWNED` would otherwise interpolate raw into `package__check my pkg`
/// (TWO args ⇒ probes the wrong entity, a `kFAIL-perform` wrong-elision) or
/// `package__check x; touch …` (the `;` parses as a SECOND command ⇒ a `kFAIL-withhold`
/// probe-mutation). Wrapping in `'…'` (with the `'\''` escape for an embedded quote)
/// makes the value inert and exactly one positional arg, in every `sh`.
///
/// This is a pass-through byte-transform, NOT a decode (`inv-referent-agnostic`): the
/// quoting never branches on what the operand *means*, only on which bytes need
/// escaping to survive the shell verbatim — the same latitude render already takes to
/// pass the operand through to the check.
fn sh_single_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        if ch == '\'' {
            out.push_str("'\\''"); // close-quote, escaped literal quote, re-open
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

impl ProbePlan {
    /// Render the probe as a shippable, read-only shell-script (the sanitised
    /// projection shipped to gather facts — DESIGN). Provenance comments name the
    /// fact (display only — `inv-referent-agnostic`).
    ///
    /// Half-B FLAT interceptor model (`notes/197` §2, variant A): each kind's
    /// `oracle_probe_<kind>` body is wrapped once into a `<kind>__check()` function,
    /// then invoked **per entity with the book's operand bound** (`$1`) — so a probe
    /// for `package:nginx#installed` runs `package__check nginx`, not the Half-A
    /// `dpkg-query -W ""` with `$1` empty. A [`EntityRef::Singleton`] fact (no
    /// operand, `package-index#fresh`) is invoked with no args. The interceptors are
    /// independent ⇒ a real dispatcher runs them concurrently (`an-leaf-seam`); the
    /// CFG-preserved variant B (`an-maintain-cfg`) is deferred (`notes/198`).
    ///
    /// Binding the operand into the shipped artifact resolves the operand token to
    /// text — legitimate (it is passed *through* to the check, never branched on —
    /// `inv-referent-agnostic`), the same latitude `fact_label` uses for display.
    #[must_use]
    pub fn render_sh(&self, interner: &Interner) -> String {
        let mut out = String::from(
            "#!/bin/sh\n# dorc probe (read-only): reports per-fact convergence, mutates nothing.\n\n",
        );
        // Each kind's check fn is emitted once (first-seen ⇒ deterministic), then
        // invoked per fact with the operand bound.
        let mut defined = BTreeSet::new();
        for check in &self.checks {
            let fn_name = check_fn_name(interner, check.fact.kind);
            out.push_str(&format!("# probe: {}\n", fact_label(interner, check.fact)));
            if defined.insert(check.fact.kind) {
                // body is a brace-group, so `name() <group>` is a valid POSIX funcdef.
                out.push_str(&format!("{fn_name}() {}\n", check.sh));
            }
            match check.fact.entity {
                EntityRef::Operand(tok) => {
                    // Single-quote so the operand is always exactly one inert positional
                    // arg, never split or re-parsed (F-QUOTE, `notes/198`).
                    let operand = sh_single_quote(interner.resolve(tok.0));
                    out.push_str(&format!("{fn_name} {operand}\n"));
                }
                EntityRef::Singleton => out.push_str(&format!("{fn_name}\n")),
            }
        }
        out
    }

    /// Did the probe compile a check for `fact`? The apply may only elide a fact the
    /// probe actually checks (the "can't-probe ⇒ can't-elide" link).
    #[must_use]
    pub fn checks_fact(&self, fact: FactKey) -> bool {
        self.checks.iter().any(|c| c.fact == fact)
    }
}

/// Compile the probe from the analysis result: every [`SkipClass::EstablishAmbient`]
/// fact whose kind has a declared read-only probe, supplied by `probe_body` (the
/// oracle seam, threaded by the caller so `plan` need not depend on `oracle`).
/// Deterministic, non-mutating; the FORWARD half of the compiler (the apply is
/// [`build_plan`]). A kind without a probe yields no check ⇒ its facts cannot be
/// elided downstream.
#[must_use]
pub fn compile_probe(
    classes: &[(CfgNodeId, SkipClass)],
    probe_body: impl Fn(KindId) -> Option<String>,
) -> ProbePlan {
    let mut checks = Vec::new();
    let mut seen = BTreeSet::new();
    for (_, class) in classes {
        if let SkipClass::EstablishAmbient(fact) = class
            && seen.insert(*fact)
            && let Some(sh) = probe_body(fact.kind)
        {
            checks.push(ProbeCheck { fact: *fact, sh });
        }
    }
    ProbePlan { checks }
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
    // Map each classified leaf's AstId → its fact (only establish classes carry one).
    // The fold reaches over the AST and needs each leaf's observed status keyed by
    // AstId, so it asks this map, then the injected `observe`.
    let leaf_fact: BTreeMap<AstId, FactKey> = classes
        .iter()
        .filter_map(|(node, class)| {
            let fact = match class {
                SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => *f,
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
                SkipClass::EstablishAmbient(f) | SkipClass::EstablishWritten(f) => {
                    Some(observe(*f))
                }
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
    // reading), then assign stable leaf ids.
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

    // (1) convergence-elision (the existing path, refined to a value-preserving
    // stand-in). Top-containment (16G hole-5): a ⊤-successor leaf is never replaced.
    match class {
        SkipClass::EstablishAmbient(_) if !has_top_successor(cfg, node) => {
            let verdict =
                PhasedVerdict::<Probe>::new(observed.map_or(Verdict::Unknown, |o| o.effect));
            let consumed = May(cfg.consumed_observables(node).clone());
            let status = observed.map_or(Predicted::Top, |o| o.status);
            match ReplaceLicense::prove_replaceable(class, Grade::Must, verdict, consumed, status) {
                Some(license) => {
                    // The value-preserving stand-in reproduces the predicted Status channel.
                    // An unpredicted status (`Predicted::Top`) falls back to the conforming
                    // `true` (rc 0) — reached ONLY where the status is not branch-consumed
                    // (`prove_replaceable` blocks a branch-consumed `Top`, `19D`), so the
                    // rc-0 placeholder is never read by a branch.
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
        // Per-AstId disposition, so an `Omit`'s controller can be resolved for the
        // omit-safety gate.
        let by_ast: BTreeMap<AstId, &Disposition> =
            self.steps.iter().map(|s| (s.ast, &s.disposition)).collect();

        // Per source line: does a Run leaf sit on it (⇒ verbatim)? and the surviving
        // value-preserving stand-in for a neutralised line.
        let mut run_lines: BTreeSet<usize> = BTreeSet::new();
        let mut neutral_lines: BTreeSet<usize> = BTreeSet::new();
        let mut line_standin: BTreeMap<usize, StandIn> = BTreeMap::new();
        for step in &self.steps {
            let span = ast.node(step.ast).span;
            let last_byte = span.hi.0.saturating_sub(1).max(span.lo.0);
            let lines: Vec<usize> = (line_of(span.lo.0)..=line_of(last_byte)).collect();
            match &step.disposition {
                Disposition::Run => run_lines.extend(&lines),
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
            if neutral_lines.contains(&i) && !run_lines.contains(&i) {
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

    #[test]
    fn compile_probe_includes_probeable_excludes_unprobeable() {
        // The probe = EstablishAmbient facts WITH a declared read-only probe. A kind
        // with an effect but NO probe is un-checkable ⇒ excluded ⇒ the apply cannot
        // elide it (can't-probe ⇒ can't-elide, kFAIL-perform). MustRun is never probed.
        let mut i = Interner::default();
        let package = KindId(i.intern("package"));
        let port = KindId(i.intern("port"));
        let installed = SelectorId(i.intern("installed"));
        let open = SelectorId(i.intern("open"));
        let nginx = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let p80 = EntityRef::Operand(OpaqueToken(i.intern("80")));
        let pkg_nginx = FactKey {
            kind: package,
            entity: nginx,
            selector: installed,
        };
        let port_80 = FactKey {
            kind: port,
            entity: p80,
            selector: open,
        };
        let classes = vec![
            (CfgNodeId(0), SkipClass::EstablishAmbient(pkg_nginx)),
            (CfgNodeId(1), SkipClass::EstablishAmbient(port_80)),
            (CfgNodeId(2), SkipClass::MustRun),
        ];
        // Only `package` has a declared probe.
        let probe = compile_probe(&classes, |k| {
            (k == package).then(|| "dpkg-query -W \"$1\"".to_string())
        });
        assert!(probe.checks_fact(pkg_nginx), "package probed");
        assert!(
            !probe.checks_fact(port_80),
            "port has no probe ⇒ excluded (can't elide)"
        );
        assert_eq!(
            probe.checks.len(),
            1,
            "only the probeable EstablishAmbient fact is in the probe"
        );
    }

    #[test]
    fn probe_render_binds_operand_flat_interceptor() {
        // Half-B FLAT interceptor model (`notes/197` §2 / `notes/198`): the probe
        // render wraps each kind's body into `<kind>__check()` ONCE and invokes it
        // per-entity with the book's operand BOUND — not the Half-A `$1`-unbound stub.
        let mut i = Interner::default();
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let nginx = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let curl = EntityRef::Operand(OpaqueToken(i.intern("curl")));
        let pkg_index = KindId(i.intern("package-index"));
        let fresh = SelectorId(i.intern("fresh"));
        let cell = |kind, entity, selector| FactKey {
            kind,
            entity,
            selector,
        };
        let classes = vec![
            (
                CfgNodeId(0),
                SkipClass::EstablishAmbient(cell(package, nginx, installed)),
            ),
            (
                CfgNodeId(1),
                SkipClass::EstablishAmbient(cell(package, curl, installed)),
            ),
            (
                CfgNodeId(2),
                SkipClass::EstablishAmbient(cell(pkg_index, EntityRef::Singleton, fresh)),
            ),
        ];
        let probe = compile_probe(&classes, |k| {
            if k == package {
                Some("{ dpkg-query -W \"$1\"; }".to_string())
            } else if k == pkg_index {
                Some("{ test fresh; }".to_string())
            } else {
                None
            }
        });
        let rendered = probe.render_sh(&i);

        // Operand bound + single-quoted (F-QUOTE): `package__check 'nginx'` AND
        // `package__check 'curl'`. A metachar-free operand quotes to the same single
        // arg, just wrapped (render-only; the host strips the quotes).
        assert!(
            rendered.contains("package__check 'nginx'"),
            "operand `nginx` bound + quoted into the invocation:\n{rendered}"
        );
        assert!(
            rendered.contains("package__check 'curl'"),
            "operand `curl` bound + quoted (per-entity invocation):\n{rendered}"
        );
        // The function body is defined exactly ONCE per kind (FLAT dedup), even with
        // two `package` entities.
        assert_eq!(
            rendered.matches("package__check() {").count(),
            1,
            "package's check fn defined once, invoked per entity:\n{rendered}"
        );
        // Singleton: no operand argument (`package-index__check` on its own line).
        assert!(
            rendered.contains("package-index__check\n"),
            "a Singleton fact invokes the check with NO operand:\n{rendered}"
        );
        // No `$1`-unbound leftover invocation (the Half-A degeneracy is gone): every
        // `package__check` line either defines the fn or carries an operand.
        assert!(
            !rendered.contains("package__check\n"),
            "no bare `package__check` (operand kinds must bind an operand):\n{rendered}"
        );
    }

    #[test]
    fn probe_render_quotes_operand_with_space_or_metachar() {
        // F-QUOTE (`notes/198`, `inv-kfail` both directions): the book operand is
        // interned POST-parse (quotes stripped, embedded chars preserved), so a
        // `'my pkg'` operand interns as `my pkg` and a `'x; touch /tmp/PWNED'` operand
        // interns as `x; touch /tmp/PWNED`. Unquoted these would render `package__check
        // my pkg` (TWO args ⇒ probes the WRONG entity, kFAIL-perform) and
        // `package__check x; touch …` (the `;` ⇒ a SECOND command ⇒ kFAIL-withhold
        // probe-mutation). Single-quoting makes each render as exactly ONE inert arg.
        //
        // This asserts the render-level structure (always runs, no shell). The
        // *behavioral* properties the prompt names — the rendered probe is `dash -n`-
        // clean AND `$1` binds to the whole operand — are exercised end-to-end against a
        // real POSIX shell by `e2e/cases/probe-operand-quoting` (the `-n` gate + the
        // exec gate prove binding under mocks); the spike keeps shell execution in the
        // sh harness, never in kernel unit tests (`run.sh` "IN sh, FROM sh").
        let mut i = Interner::default();
        let package = KindId(i.intern("package"));
        let installed = SelectorId(i.intern("installed"));
        let spaced = EntityRef::Operand(OpaqueToken(i.intern("my pkg")));
        let inject = EntityRef::Operand(OpaqueToken(i.intern("x; touch /tmp/PWNED")));
        // An embedded single-quote pins the `'\''` escape (the only non-trivial case).
        let quoted = EntityRef::Operand(OpaqueToken(i.intern("a'b")));
        let cell = |entity| FactKey {
            kind: package,
            entity,
            selector: installed,
        };
        let classes = vec![
            (CfgNodeId(0), SkipClass::EstablishAmbient(cell(spaced))),
            (CfgNodeId(1), SkipClass::EstablishAmbient(cell(inject))),
            (CfgNodeId(2), SkipClass::EstablishAmbient(cell(quoted))),
        ];
        let probe = compile_probe(&classes, |k| {
            (k == package).then(|| "{ dpkg-query -W \"$1\" >/dev/null 2>&1; }".to_string())
        });
        let rendered = probe.render_sh(&i);

        // The space-operand is ONE arg: the whole `my pkg` inside one `'…'`.
        assert!(
            rendered.contains("package__check 'my pkg'\n"),
            "spaced operand is single-quoted to one arg:\n{rendered}"
        );
        // The metachar-operand is inert: the `;` is INSIDE the quotes, so it cannot
        // start a second command. (The literal `touch /tmp/PWNED` survives only as data
        // passed to the check, never executed.)
        assert!(
            rendered.contains("package__check 'x; touch /tmp/PWNED'\n"),
            "metachar operand is single-quoted ⇒ the `;` cannot split:\n{rendered}"
        );
        // The embedded-quote case uses the POSIX `'\''` idiom (close, escaped quote,
        // reopen) — `a'b` ⇒ `'a'\''b'`.
        assert!(
            rendered.contains(r"package__check 'a'\''b'"),
            "embedded single-quote escaped as '\\'':\n{rendered}"
        );
        // Defence-in-depth against the raw-injection regression: the unquoted, bare
        // `; touch` sequence must NOT appear at the start of a rendered line (it only
        // ever appears inside the single-quoted operand above).
        assert!(
            !rendered.contains("\npackage__check x; touch"),
            "no UNQUOTED metachar invocation leaked:\n{rendered}"
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
    fn andor_status_blocks_only_when_rc_undeclared() {
        // `19D` (the keystone of the kFAIL-perform fix): a `&&`/`||` left operand's
        // `AndOrStatus` blocks the license iff the rc is UNDECLARED — then the stand-in
        // would default to `true`/rc-0, a fabricated success suppressing a `|| fallback`
        // (the round-19 under-execute). A *declared* rc relaxes it (the value-preserving
        // stand-in reproduces the exact status, preserving the branch).
        let f = nginx_fact();
        let consumed = || May(Powerset::singleton(Channel::AndOrStatus));
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
    fn if_guard_status_blocks_unconditionally() {
        // `19D` / 19C strain-D: the `if`/`elif`-guard `Status` is the render floor — it
        // blocks the license EVEN with a declared rc (the line-granular render cannot
        // substitute a guard on its `if`/`then`/`fi` line; a declared-rc relaxation
        // would break `dash -n`). Contrast `andor_status_blocks_only_when_rc_undeclared`.
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
                    May(Powerset::singleton(Channel::Status)),
                    rc,
                )
                .is_none(),
                "an if-guard's Status blocks unconditionally (render floor), rc={rc:?}"
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
            // (AndOrStatus), and a declared rc-0 would relax-and-elide it; with the
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
