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

use core::marker::PhantomData;
use std::collections::BTreeSet;

use dorc_analysis::cfg::{Cfg, CfgNodeId, CfgNodeKind, Observable};
use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_analysis::lattice::{May, Powerset};
use dorc_core::{AstId, Grade, Interner, KindId, Verdict};
use dorc_syntax::ast::Ast;

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
        Self { raw, _phase: PhantomData }
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
// context consumes ([`Observable`]) — is computed by the ENGINE and emitted on the
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
    /// 4. no UNVOUCHED observable (stdout/stderr) is consumed downstream. The
    ///    consumption is the engine's un-collapsed `May<Powerset<Observable>>` fact
    ///    (`inv-superposition`, note 16J); per `inv-must-may` a `May` value can only
    ///    block. The stub defaults stdout/stderr to empty, vouched by nothing (16F
    ///    §3); a consumed one ⇒ run (no in-spike bridge).
    ///
    /// Generic over the phase `P` (`inv-superposition`): the engine never bakes a
    /// phase; the caller argues it. `build_plan` passes the verdict's own provenance
    /// (`Probe`).
    #[must_use]
    pub fn prove_replaceable<P: Bias>(
        class: &SkipClass,
        grade: Grade,
        verdict: PhasedVerdict<P>,
        consumed: May<Powerset<Observable>>,
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
        // No UNVOUCHED observable (stdout/stderr) consumed downstream. The fact
        // arrives un-collapsed as a `May` (over-approximate consumption): per
        // `inv-must-may` a `May` value can only BLOCK a license, never grant one — a
        // may-consumed stdout/stderr forbids the `true`-stub's empty default (16F §3
        // / note 16J; no in-spike bridge discharges it). The block is sound in BOTH
        // phases; only what a blocked leaf's disposition *becomes* is phase-keyed —
        // the caller's collapse (`inv-superposition`), not here.
        let May(consumed) = &consumed;
        if consumed.contains(&Observable::Stdout) || consumed.contains(&Observable::Stderr) {
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

/// What the plan does with one leaf.
#[derive(Debug, Clone)]
pub enum Disposition {
    /// Run the leaf — its effect is needed, its convergence is unknown, or an
    /// unvouched observable it emits is consumed downstream.
    Run,
    /// Replace the leaf with a stand-in (`true` is the degenerate stand-in) —
    /// authorised by a [`ReplaceLicense`], the only way to reach here.
    Replace(ReplaceLicense),
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

impl ProbePlan {
    /// Render the probe as a shippable, read-only shell-script (the sanitised
    /// projection shipped to gather facts — DESIGN). Provenance comments name the
    /// fact (display only — `inv-referent-agnostic`).
    #[must_use]
    pub fn render_sh(&self, interner: &Interner) -> String {
        let mut out = String::from(
            "#!/bin/sh\n# dorc probe (read-only): reports per-fact convergence, mutates nothing.\n\n",
        );
        for check in &self.checks {
            out.push_str(&format!("# probe: {}\n{}\n", fact_label(interner, check.fact), check.sh));
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
        if let SkipClass::EstablishAmbient(fact) = class {
            if seen.insert(*fact) {
                if let Some(sh) = probe_body(fact.kind) {
                    checks.push(ProbeCheck { fact: *fact, sh });
                }
            }
        }
    }
    ProbePlan { checks }
}

/// Build a plan from the analysis result + an injected host probe oracle.
///
/// `verdict_of` is the host probe (the real host / `hostsim` is a later seam): it
/// answers, per fact, whether it already holds. `build_plan` is a pure function of
/// its inputs (deterministic given a deterministic `verdict_of`). Every command
/// leaf becomes a [`Step`]; a leaf is `Skip` ONLY when
/// [`ReplaceLicense::prove_replaceable`] mints a license — every other leaf runs (the
/// `kFAIL-perform` safe direction).
#[must_use]
pub fn build_plan(
    src: &str,
    ast: &Ast,
    cfg: &Cfg,
    classes: &[(CfgNodeId, SkipClass)],
    verdict_of: impl Fn(FactKey) -> Verdict,
) -> Plan {
    let mut steps: Vec<Step> = classes
        .iter()
        .map(|(node, class)| {
            let ast_id = cfg.node(*node).ast;
            let sh = command_text(src, ast, ast_id);
            let disposition = match class {
                // Top-containment (16G hole-5): a leaf whose own statement is
                // top-contaminated — e.g. a backgrounded `cmd &`, lowered as the leaf
                // followed by a `Top` node — must not be replaced (its execution
                // context is unmodeled). Conservative: a Top successor => run.
                SkipClass::EstablishAmbient(fact) if !has_top_successor(cfg, *node) => {
                    // `Probe` tags the verdict's provenance (a host probe);
                    // prove_replaceable is generic over the phase (inv-superposition:
                    // the caller argues it, the engine never bakes it).
                    let verdict = PhasedVerdict::<Probe>::new(verdict_of(*fact));
                    // The engine's un-collapsed consumption fact, read in the `May`
                    // (over-approximate) orientation: if it MAY be consumed, block.
                    let consumed = May(cfg.consumed_observables(*node).clone());
                    // An EstablishAmbient fact comes from the oracle effect-map by
                    // construction, so its grade is Must (note 160 `must-may`).
                    match ReplaceLicense::prove_replaceable(class, Grade::Must, verdict, consumed) {
                        Some(license) => Disposition::Replace(license),
                        // None ⇒ run: the APPLY collapse of "no license"
                        // (kFAIL-perform). The probe projection, when built, maps
                        // "no license" to *withhold* — its own collapse (note 16J §5,
                        // inv-superposition). Don't reuse this arm for probe.
                        None => Disposition::Run,
                    }
                }
                _ => Disposition::Run,
            };
            Step { leaf: LeafId(0), ast: ast_id, sh, disposition }
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
    cfg.succ_ids(node).any(|s| cfg.node(s).kind == CfgNodeKind::Top)
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
                Disposition::Replace(license) => {
                    out.push_str(&format!(
                        "# replace[{}]: {}\n#   \u{21b3} {} already holds (probe: converged \u{b7} must \u{b7} ambient)\n",
                        step.leaf.0,
                        step.sh,
                        fact_label(interner, license.fact()),
                    ));
                }
            }
        }
        out
    }

    /// Render the apply as the ORIGINAL book with elided (`Replace`) command-lines
    /// commented out — "a copy of book.sh with the safe-to-omit lines commented", the
    /// CLI's final artifact. Line-granular (the spike's books are ~one-command-per-line):
    /// a source line is commented iff a `Replace` leaf lies on it and no `Run` leaf
    /// does. Everything else (guards, blanks, comments, multi-leaf lines) passes
    /// verbatim, so the output stays a runnable shell-script with the original
    /// structure intact (contrast [`render_sh`](Plan::render_sh), the flat leaf-list).
    #[must_use]
    pub fn render_apply(&self, src: &str, ast: &Ast) -> String {
        let line_of = |byte: u32| -> usize {
            src.get(..byte as usize).map_or(0, |s| s.bytes().filter(|&b| b == b'\n').count())
        };
        let (mut elided, mut run) = (BTreeSet::new(), BTreeSet::new());
        for step in &self.steps {
            let span = ast.node(step.ast).span;
            let last_byte = span.hi.0.saturating_sub(1).max(span.lo.0);
            let lines = line_of(span.lo.0)..=line_of(last_byte);
            let target = match &step.disposition {
                Disposition::Replace(_) => &mut elided,
                Disposition::Run => &mut run,
            };
            target.extend(lines);
        }
        let mut out = String::from(
            "#!/bin/sh\n# dorc apply: the book, with already-converged lines elided (commented).\n\n",
        );
        for (i, line) in src.lines().enumerate() {
            if elided.contains(&i) && !run.contains(&i) {
                out.push_str("# ");
                out.push_str(line);
                out.push_str("   # dorc: elided (already converged)\n");
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
}

/// `kind:entity` for a fact, resolving the interned names for *display/provenance*
/// only (never a logic branch — `inv-referent-agnostic`). Public so the CLI can match
/// probe-result lines back to facts by the same label the probe emitted.
///
// TODO(K2): the K1 re-key made `fact.entity` an `EntityRef` (Operand/Singleton) and
// added `fact.selector` (`core::FactKey`). This must resolve the richer key for
// display — e.g. `package:nginx#installed` (Operand) / `package-index#fresh`
// (Singleton). The exact format is a K2 call because it COUPLES to the cli round-trip:
// `cli::parse_results`'s stdin grammar keys verdicts by this label and `hostsim`'s
// fact-store keys on the same `FactKey` (see crates/cli/CLAUDE.md "stdin re-key
// gotcha" — do NOT drop the selector on parse and widen a verdict to the whole
// entity, a wrong-elision under apply's kFAIL). K1 owns core+oracle+analysis only and
// leaves this break for K2 (the body below still reads the old flat `.entity.0`).
#[must_use]
pub fn fact_label(interner: &Interner, fact: FactKey) -> String {
    format!(
        "{}:{}",
        interner.resolve(fact.kind.0),
        interner.resolve(fact.entity.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{Interner, KindId, OpaqueToken, ProviderId};
    use dorc_oracle::{KindIndex, Polarity};

    fn nginx_fact() -> FactKey {
        let mut i = Interner::default();
        FactKey {
            kind: KindId(i.intern("package")),
            entity: OpaqueToken(i.intern("nginx")),
        }
    }

    /// An empty (provably-quiet) consumption fact in the `May` orientation — the
    /// common case for the `prove_replaceable` unit tests.
    fn quiet() -> May<Powerset<Observable>> {
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
        let nginx = OpaqueToken(i.intern("nginx"));
        let p80 = OpaqueToken(i.intern("80"));
        let classes = vec![
            (CfgNodeId(0), SkipClass::EstablishAmbient(FactKey { kind: package, entity: nginx })),
            (CfgNodeId(1), SkipClass::EstablishAmbient(FactKey { kind: port, entity: p80 })),
            (CfgNodeId(2), SkipClass::MustRun),
        ];
        // Only `package` has a declared probe.
        let probe = compile_probe(&classes, |k| {
            (k == package).then(|| "dpkg-query -W \"$1\"".to_string())
        });
        assert!(probe.checks_fact(FactKey { kind: package, entity: nginx }), "package probed");
        assert!(
            !probe.checks_fact(FactKey { kind: port, entity: p80 }),
            "port has no probe ⇒ excluded (can't elide)"
        );
        assert_eq!(probe.checks.len(), 1, "only the probeable EstablishAmbient fact is in the probe");
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
        let f = nginx_fact();
        for obs in [Observable::Stdout, Observable::Stderr] {
            let consumed = May(Powerset::singleton(obs));
            assert!(
                ReplaceLicense::prove_replaceable(
                    &SkipClass::EstablishAmbient(f),
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(Verdict::Converged),
                    consumed,
                )
                .is_none(),
                "a consumed {obs:?} must forbid the stub"
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
        assert!(ReplaceLicense::prove_replaceable(
            &SkipClass::EstablishAmbient(f),
            Grade::May,
            PhasedVerdict::<Probe>::new(Verdict::Converged),
            quiet(),
        )
        .is_none());
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
                )
                .is_none(),
                "{class:?} must not license a skip"
            );
        }
    }

    #[test]
    fn unknown_folds_to_run_in_both_phases() {
        // The kFAIL fold: Unknown is never Replaceable, in either phase.
        assert_eq!(PhasedVerdict::<Probe>::new(Verdict::Unknown).resolve(), Resolved::Run);
        assert_eq!(PhasedVerdict::<Apply>::new(Verdict::Unknown).resolve(), Resolved::Run);
        // Sanity on the definite verdicts.
        assert_eq!(PhasedVerdict::<Probe>::new(Verdict::Converged).resolve(), Resolved::Replaceable);
        assert_eq!(PhasedVerdict::<Apply>::new(Verdict::Diverged).resolve(), Resolved::Run);
    }

    // --- end-to-end: the whole pipeline (parse → cfg → classify → plan) ---

    /// A package kind-index with `apt-get install → establish`. Deliberately no
    /// `apt-get update` effect — it stays Opaque, which is the fixture's poison.
    fn package_index(i: &mut Interner) -> KindIndex {
        let package = KindId(i.intern("package"));
        let apt = ProviderId(i.intern("apt-get"));
        let install = i.intern("install");
        let mut idx = KindIndex::default();
        idx.add_effect(apt, install, package, Polarity::Establish);
        idx
    }

    /// Run the pipeline on `src`, answering `package:nginx` with `nginx_verdict`
    /// and every other fact `Unknown`.
    fn plan_for(src: &str, nginx_verdict: Verdict) -> (Plan, Interner) {
        let mut i = Interner::default();
        let idx = package_index(&mut i);
        let target = FactKey {
            kind: KindId(i.intern("package")),
            entity: OpaqueToken(i.intern("nginx")),
        };
        let parsed = dorc_syntax::parse(src);
        let cfg = dorc_analysis::cfg::build(&parsed.value).value;
        let classes = dorc_analysis::effect::classify(&cfg, &parsed.value, &idx, &mut i);
        let plan = build_plan(src, &parsed.value, &cfg, &classes, |f| {
            if f == target {
                nginx_verdict
            } else {
                Verdict::Unknown
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
        let (plan, interner) =
            plan_for("apt-get install -y nginx\nsystemctl reload nginx\n", Verdict::Converged);
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Replace(_)),
            "converged ambient install ⇒ skip"
        );
        assert!(
            matches!(find(&plan, "systemctl reload").disposition, Disposition::Run),
            "opaque reload ⇒ run"
        );

        let sh = plan.render_sh(&interner);
        assert!(sh.contains("# replace["), "rendered plan comments the replaced leaf:\n{sh}");
        assert!(sh.contains("package:nginx"), "replace provenance names the fact:\n{sh}");
        assert!(sh.contains("systemctl reload nginx"), "run leaf rendered verbatim:\n{sh}");
    }

    #[test]
    fn diverged_install_runs() {
        // The host says nginx is absent ⇒ the install must run (no license).
        let (plan, _) =
            plan_for("apt-get install -y nginx\nsystemctl reload nginx\n", Verdict::Diverged);
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "diverged ⇒ run"
        );
    }

    #[test]
    fn fixture_install_runs_despite_converged_probe() {
        // TODO(K2): the keystone INVERTS this test. The poison was `apt-get update`
        // being un-oracled ⇒ Opaque ⇒ poisoning the `install -y nginx` below it to
        // EstablishWritten. K1 (notes/193) makes `update` MODELED on a distinct cell
        // (`package-index#fresh`), so it no longer poisons the install — proven at the
        // classify level by `effect::tests::poison_wall_dies_modeled_update_does_not_
        // poison_install`. K2 must: (a) make `package_index(&mut i)` here model
        // `apt-get update → (package-index, #fresh, establish)` (it currently models
        // only `install`), then (b) FLIP this assertion to `Disposition::Replace` on a
        // Converged host. CAVEAT (exclusion-check): the fixture's install sits in an
        // `if ! command -v nginx` guard with other un-oracled neighbors (`ufw allow`,
        // `systemctl enable --now`, the `case "$(hostname)"` scrutinee). Verify NONE of
        // those still poison the install's cell before asserting Replace — if one does,
        // the right assertion may stay Run for a DIFFERENT (correct) reason, which is
        // itself a finding to record in notes/193. The keystone kills `update`'s
        // poison specifically, not all poison on the book.
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/pi-webhost.book.sh"
        ));
        let (plan, _) = plan_for(fixture, Verdict::Converged);
        assert!(
            matches!(find(&plan, "apt-get install").disposition, Disposition::Run),
            "poisoned install must run even with a converged probe"
        );
    }

    #[test]
    fn substitution_internal_command_is_not_a_plan_leaf() {
        // find-cli-1: the `$(uname)` body command must NOT be a plan Step (it runs
        // during word expansion, not as a leaf); the two top-level commands are the
        // only leaves. Before the fix this rendered a third, garbage step from the
        // substring-relative span of the subst body.
        let (plan, _) = plan_for("echo $(uname)\napt-get install -y nginx\n", Verdict::Diverged);
        assert_eq!(
            plan.steps.len(),
            2,
            "only the two top-level commands are leaves: {:?}",
            plan.steps.iter().map(|s| s.sh.clone()).collect::<Vec<_>>()
        );
        assert!(plan.steps.iter().any(|s| s.sh.starts_with("echo")), "echo is a leaf");
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
        let classes = dorc_analysis::effect::classify(&cfg, &parsed.value, &idx, &mut i);
        assert!(!classes.is_empty(), "fixture has classify leaves");
        let (mut marked, mut quiet) = (0, 0);
        for (node, _) in &classes {
            // Total Vec ⇒ defined for every classify leaf (never an absent lookup).
            if cfg.consumed_observables(*node).contains(&Observable::Stdout) {
                marked += 1;
            } else {
                quiet += 1;
            }
        }
        assert!(marked >= 1, "the group-redirected install is marked Stdout");
        assert!(quiet >= 1, "the lone curl install is quiet");
    }
}
