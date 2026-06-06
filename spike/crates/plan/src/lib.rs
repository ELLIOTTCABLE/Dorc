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

use dorc_analysis::cfg::{Cfg, CfgNodeId};
use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_core::{AstId, Grade, Interner, Verdict};
use dorc_syntax::ast::{Ast, NodeKind, RedirOp, RedirTarget, WordPart};

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
// Observables + the unvouched-output gate (16F / note 16G)
// ===========================================================================

/// A thing a downstream consumer can detect about a leaf having run (16F). The
/// `true`-stub defaults each — effect→none, status→0, stdout/stderr→empty — so the
/// only question per observable is whether that default is *vouched*: **effect** by
/// convergence (the forward gate), **status** by the `establishes` contract (a
/// converged establish exits 0 — free), **stdout/stderr** by NOTHING.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observable {
    Effect,
    Status,
    Stdout,
    Stderr,
}

/// Which of a leaf's **unvouched** observables (stdout/stderr) are consumed in a
/// value-bearing position downstream. Effect and status are vouched elsewhere, so
/// only an unvouched-and-consumed observable forbids the stub (16F §3 — one
/// observable-liveness obligation; status is establish-discharged). Set by a
/// conservative *structural* surrogate (16F §5; the analyzer has no value-plane to
/// prove a consumer is actually value-sensitive — 16C brk-1): a non-last pipeline
/// stage, or an output redirect of fd 1/2 to a non-`/dev/null` sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ObservableUse {
    pub stdout_consumed: bool,
    pub stderr_consumed: bool,
}

impl ObservableUse {
    /// Does a consumed unvouched observable forbid the `true`-stub's empty default?
    /// (Effect and status are vouched; only stdout/stderr being consumed forbids it.)
    #[must_use]
    pub fn forbids_stub(self) -> bool {
        self.stdout_consumed || self.stderr_consumed
    }
}

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
    /// 4. no UNVOUCHED observable (stdout/stderr) is consumed downstream — the stub
    ///    defaults them to empty and `establishes` vouches only status, not output
    ///    (16F §3 / note 16G); a consumed one ⇒ run (no in-spike bridge).
    #[must_use]
    pub fn prove_replaceable(
        class: &SkipClass,
        grade: Grade,
        verdict: PhasedVerdict<Probe>,
        output: ObservableUse,
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
        // No unvouched observable (stdout/stderr) consumed downstream: the `true`
        // stub defaults them to empty, vouched by nothing (16F §3 / note 16G); a
        // consumed one would diverge ⇒ run (no in-spike bridge discharges it).
        if output.forbids_stub() {
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
    let non_last_stages = non_last_pipeline_stages(ast);
    let mut steps: Vec<Step> = classes
        .iter()
        .map(|(node, class)| {
            let ast_id = cfg.node(*node).ast;
            let sh = command_text(src, ast, ast_id);
            let disposition = match class {
                SkipClass::EstablishAmbient(fact) => {
                    let verdict = PhasedVerdict::<Probe>::new(verdict_of(*fact));
                    let output = observable_use(ast, ast_id, &non_last_stages);
                    // An EstablishAmbient fact comes from the oracle effect-map by
                    // construction, so its grade is Must (note 160 `must-may`).
                    match ReplaceLicense::prove_replaceable(class, Grade::Must, verdict, output) {
                        Some(license) => Disposition::Replace(license),
                        None => Disposition::Run,
                    }
                }
                SkipClass::EstablishWritten(_) | SkipClass::MustRun => Disposition::Run,
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

/// AST ids of every command that is a **non-last stage of a pipeline** — its stdout
/// is piped into the next stage, hence value-bearingly consumed (16F / 16G). One
/// pass over the arena (no parent pointers, so collect from each `Pipeline`'s
/// `stages[..last]`).
fn non_last_pipeline_stages(ast: &Ast) -> BTreeSet<AstId> {
    let mut out = BTreeSet::new();
    for (_, node) in ast.iter() {
        if let NodeKind::Pipeline { stages, .. } = &node.kind {
            if let Some((_last, leading)) = stages.split_last() {
                out.extend(leading.iter().copied());
            }
        }
    }
    out
}

/// The unvouched-output consumption of the leaf whose `Simple` node is `simple_id`
/// — the conservative structural surrogate (16F §5; no value-plane, 16C brk-1):
/// stdout is consumed if the leaf is a non-last pipeline stage or redirects fd 1
/// (or the default) to a non-`/dev/null` sink; stderr if it redirects fd 2 there.
/// fd-dups (`2>&1`, `>&3`) are deliberately NOT resolved here (a deferred
/// refinement — 16G): the floor already runs any piped or file-redirected establish,
/// and not resolving dups keeps `> /dev/null 2>&1` correctly replaceable.
fn observable_use(ast: &Ast, simple_id: AstId, non_last_stages: &BTreeSet<AstId>) -> ObservableUse {
    let mut used = ObservableUse {
        stdout_consumed: non_last_stages.contains(&simple_id),
        stderr_consumed: false,
    };
    let NodeKind::Simple { redirs, .. } = &ast.node(simple_id).kind else {
        return used;
    };
    for &r in redirs {
        let NodeKind::Redir { op, fd, target } = &ast.node(r).kind else {
            continue;
        };
        if !matches!(op, RedirOp::Write | RedirOp::Append) {
            continue; // input / fd-dup / here-doc: not an output-write sink
        }
        let RedirTarget::Word(w) = target else { continue };
        if word_text(ast, *w) == Some("/dev/null") {
            continue; // discard sink ⇒ not consumed
        }
        match fd {
            None | Some(1) => used.stdout_consumed = true,
            Some(2) => used.stderr_consumed = true,
            _ => {}
        }
    }
    used
}

/// The single-literal text of a word node, if it is one (mirrors the analyzer's
/// `word_literal`; used only to recognise the `/dev/null` redirect sink).
fn word_text(ast: &Ast, id: AstId) -> Option<&str> {
    match &ast.node(id).kind {
        NodeKind::Word { parts } => match parts.as_slice() {
            [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s.as_str()),
            _ => None,
        },
        _ => None,
    }
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
                        fact_display(interner, license.fact()),
                    ));
                }
            }
        }
        out
    }
}

/// `kind:entity` for a fact, resolving the interned names for *display only*
/// (provenance, never a logic branch — `inv-referent-agnostic`).
fn fact_display(interner: &Interner, fact: FactKey) -> String {
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

    #[test]
    fn license_minted_for_ambient_must_converged() {
        // The one path that authorises a skip: classify said ambient, the oracle
        // declared Must, and the probe found it already holds.
        let f = nginx_fact();
        let Some(lic) = ReplaceLicense::prove_replaceable(
            &SkipClass::EstablishAmbient(f),
            Grade::Must,
            PhasedVerdict::<Probe>::new(Verdict::Converged),
            ObservableUse::default(),
        ) else {
            panic!("ambient + must + converged must license a skip");
        };
        assert_eq!(lic.fact(), f);
        assert!(lic.derivation().ambient);
        assert_eq!(lic.derivation().verdict, Verdict::Converged);
    }

    #[test]
    fn no_license_when_unvouched_output_consumed() {
        // 16F/16G: a consumed stdout/stderr makes the `true`-stub's empty default
        // unsound ⇒ no license (run), even with ambient + Must + Converged.
        let f = nginx_fact();
        let consumed = ObservableUse { stdout_consumed: true, stderr_consumed: false };
        assert!(
            ReplaceLicense::prove_replaceable(
                &SkipClass::EstablishAmbient(f),
                Grade::Must,
                PhasedVerdict::<Probe>::new(Verdict::Converged),
                consumed,
            )
            .is_none(),
            "a consumed unvouched observable must forbid the stub"
        );
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
                    ObservableUse::default(),
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
            ObservableUse::default(),
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
                    ObservableUse::default(),
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
        // fs-4 on the REAL book: `apt-get update` is un-oracled ⇒ Opaque ⇒ poisons
        // downstream ambient-ness, so the `apt-get install -y nginx` after it is
        // EstablishWritten, not EstablishAmbient — and prove_replaceable refuses a
        // Written leaf. So even a Converged probe cannot license the skip; the
        // install runs. (Surfaced design problem: to recover the skip the oracle
        // must model `apt-get update` as package-state-pure; the spike's does not.)
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
}
