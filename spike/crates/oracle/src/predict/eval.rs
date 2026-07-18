//! The concrete evaluator — trace a known argv through a [`Predict`]'s argparse to
//! its kind-annotation (202 §1 face-check; 19H §2).
//!
//! This is *not* abstract interpretation. The book-side value-flow (task-A,
//! `analysis::value`) resolves a command-site's argv to a concrete `Vec<String>`;
//! this evaluator then runs the oracle's own `check()` control-flow over that known
//! argument list (`while` strips flags exactly as written, `case` selects the arm
//! the real shell would, `shift` consumes). The argparse loops terminate by
//! construction (each iteration consumes arguments), but a hostile or buggy check
//! could loop, so an iteration budget bounds it: budget-exceeded ⇒ [`Resolution::Top`].
//!
//! # Output (`inv-superposition`)
//!
//! A [`Resolution`] is a phase-/orientation-agnostic *fact*: which kind, which argv
//! element is the entity, the derived verb (if any), and which probe command(s) the
//! selected path reaches (as verbatim spans). The phased caller collapses it; this
//! module bakes no phase. Anything non-concrete ⇒ [`Resolution::Top`] with a reason
//! string (`inv-kfail`, both directions: nothing ships, nothing elides).

use super::ast::{Annotation, Command, Pattern, Predict, Stmt, Test, TestOp, Word};
use dorc_core::{Span, Symbol};
use dorc_syntax::sem::{self, UnsetPolicy};
use std::collections::BTreeMap;

/// How a predict arm covers its STDOUT channel, in the `273` §2 per-channel vocabulary
/// (`271:rul-only-oracle-bytes-ship` rider 1) — the coverage the composed-probe compiler
/// consumes to decide whether a pipe stage may be model-substituted. Only a NON-LAST stage's
/// stdout is consumed (piped into the next stage), so this gates exactly those.
///
/// The knife-tier floor (`271:rul-composed-bytes-defer-and-floor`): a byte-consumer downstream
/// needs REAL (world-spoken / delegation-produced) bytes, so only [`RealBytes`](StageStdout::RealBytes)
/// satisfies it — [`Asserted`](StageStdout::Asserted) (a `printf` claim) does NOT, and
/// [`Declined`](StageStdout::Declined) plainly does not. The compiler refuses the whole compound
/// unless every non-last stage is `RealBytes` (can't-say ⇒ run — always safe).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageStdout {
    /// A DELEGATION arm running the real read-only command with stdout on the pipe: faithful
    /// real bytes (§2 "delegation = faithful all-channel claim"). The only knife-tier-safe form.
    RealBytes,
    /// A `printf`-ASSERTED stdout (§2 "printf = asserted output claim"): covers the channel but
    /// with declared, not world-spoken, bytes — rider 3 (capture-ships-real-bytes) refuses it for
    /// a knife-tier byte-consumer. Distinguished from `Declined` for provenance / stage-4 use.
    Asserted,
    /// STDOUT declined: voided by a redirect (§2 "redirect-to-null = per-channel decline"), or the
    /// selected path reached no producing command (⊤ / whole-shape decline / `return 2`).
    Declined,
}

/// The result of evaluating a [`Predict`] over a concrete argv.
///
/// Either a concrete resolution or [`Top`](Resolution::Top) — a single safe
/// degrade for everything non-concrete (`inv-kfail`). A `Top` site stays
/// un-probeable and un-elidable; a *wrong* [`Resolved`](Resolution::Resolved) is the
/// disaster class (19H §1.3), so the evaluator biases every ambiguity to `Top`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// The argparse resolved concretely to an entity + kind (+ optional verb + the
    /// probe bodies the selected path reaches).
    Resolved(Resolved),
    /// Non-concrete, out-of-dialect-at-runtime, budget overrun, missing annotation,
    /// or an annotation never reached on the selected path. Carries a reason for
    /// diagnostics/provenance. Always the safe outcome.
    Top(TopReason),
}

/// A concrete resolution of a check over an argv.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    /// The reverse-DNS kind string from the annotation (opaque coordination handle;
    /// never decoded — `inv-referent-agnostic`).
    pub kind: String,
    /// The resolved entity: either a concrete operand the annotation denotes
    /// (`nginx`), or [`ResolvedEntity::Singleton`] for a nullary verb whose resource
    /// has no operand (`apt-get update`; 202 §2 / task-W §4).
    pub entity: ResolvedEntity,
    /// The derived verb, if the check binds one (the value bound to a variable the
    /// oracle named `verb`). `None` for a verbless check (`useradd` — 19H §2.3); the
    /// absence is a first-class outcome, not an error.
    pub verb: Option<String>,
    /// The probe command(s) the selected path reaches, as VERBATIM source spans into
    /// the oracle file (`Command::span`). A `systemctl`-style check carries a
    /// different probe per verb arm (19H §2.5); these are the ones the *selected*
    /// path actually runs, in execution order.
    pub probe_body: Vec<Span>,
}

/// The resolved entity of a [`Resolved`] — the operand the annotation denotes, or
/// the Singleton (no-operand) resource of a nullary verb.
///
/// Maps directly onto `core::EntityRef` at the wiring boundary
/// (`Operand(text)` → `EntityRef::Operand`, `Singleton` → `EntityRef::Singleton`),
/// preserving the existing Singleton semantics (`apt-get update` ⇒
/// `package-index#fresh`, no `:operand` segment in its `fact_label`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedEntity {
    /// A concrete operand argv element (`nginx`) the annotation's value resolved to.
    Operand(String),
    /// A nullary verb's singleton resource (no operand): the value-less annotation
    /// form (`index : pkgindex`). The wiring keys this on `EntityRef::Singleton`.
    Singleton,
}

/// Why an evaluation degraded to [`Resolution::Top`]. A closed enum so adding a new
/// degrade-reason breaks every exhaustive match (the compiler-as-checklist).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopReason {
    /// The argv was empty (no command for the argparse to consume).
    EmptyArgv,
    /// A word resolved to no concrete value (an unbound variable, an unmodeled
    /// expansion, `$0`, a positional past the end of the current argv).
    NonConcreteWord(&'static str),
    /// The check has no inline kind-annotation at all.
    MissingAnnotation,
    /// The annotation resolved, but the selected path reached no probe command — e.g.
    /// a `systemctl` check whose `case $verb` matched no arm (an unknown verb), so no
    /// `is-enabled`/`is-active` body ran. A probe-less resolution is not actionable;
    /// the conservative outcome is un-probeable ⇒ runs (`inv-kfail`). (19H §2.5: the
    /// probe lives in the verb arm, so no-arm ⇒ no probe.)
    NoProbeReached,
    /// The annotation's value-position did not resolve to a concrete argv element or
    /// literal (e.g. `pkg : Kind = "$3"` when argv has 2 elements).
    UnresolvedAnnotationValue,
    /// The iteration budget was exhausted (a loop did not terminate within bound).
    BudgetExceeded,
    /// The selected path reached a command PIPELINE (`cmd | cmd | …`, 24E §14) — ACCEPTED at parse
    /// (it ships byte-exact, the kLANG mirror-invariant), but a pipeline never statically resolves
    /// (the tracer cannot model its dataflow) ⇒ ⊤ ⇒ the site RUNS (the safe degrade,
    /// `kFAIL-perform`). Parse-permissively; trace-conservatively. (A `touches()` pipeline instead
    /// ESCALATES — see `touches::TouchesTop::NonPrintfCommand`.)
    Pipeline,
}

impl TopReason {
    /// A short human-readable form for diagnostics/provenance.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            TopReason::EmptyArgv => "empty argv",
            TopReason::NonConcreteWord(w) => w,
            TopReason::MissingAnnotation => "check has no kind-annotation",
            TopReason::NoProbeReached => "selected path reached no probe command",
            TopReason::UnresolvedAnnotationValue => {
                "annotation value did not resolve to an argv element"
            }
            TopReason::BudgetExceeded => "iteration budget exceeded",
            TopReason::Pipeline => {
                "selected path reached a command pipeline (out of dialect ⇒ runs)"
            }
        }
    }
}

/// Evaluate `check` over `argv` — the full, concrete, verbatim argument list of the
/// book's command, **not** including the command word itself (C-1: the oracle
/// receives full verbatim args, the engine parses nothing). Returns a
/// [`Resolution`].
///
/// # Determinism / no-throw
///
/// Pure and total (`inv-determinism`/`inv-no-throw`): no clock/RNG/IO, ordered
/// collections only, and every path returns a [`Resolution`] — never panics, even on
/// a pathological check (the budget bounds loops).
#[must_use]
pub fn evaluate(check: &Predict, argv: &[&str]) -> Resolution {
    if argv.is_empty() {
        return Resolution::Top(TopReason::EmptyArgv);
    }
    let mut ev = Evaluator::over(check, argv);
    match ev.run_block(&check.body) {
        Flow::Normal => ev.finish(),
        Flow::Top(reason) => Resolution::Top(reason),
    }
}

/// The STDOUT coverage a predict arm produces for a concrete `argv` ([`StageStdout`]) — the
/// composed-probe coverage decision (`271:rul-only-oracle-bytes-ship` rider 1). Traces the
/// SAME argparse [`evaluate`] does (one shared [`Evaluator::over`]), then reports the coverage
/// of the last producing command on the selected path. A path that degrades to ⊤ (unresolved,
/// budget, a pipeline arm, `return 2`) or reaches no command ⇒ [`StageStdout::Declined`] — the
/// whole-shape decline. Pure/total (`inv-determinism`/`inv-no-throw`).
#[must_use]
pub fn predict_stage_stdout(check: &Predict, argv: &[&str]) -> StageStdout {
    if argv.is_empty() {
        return StageStdout::Declined;
    }
    let mut ev = Evaluator::over(check, argv);
    match ev.run_block(&check.body) {
        Flow::Normal => ev.last_stdout.unwrap_or(StageStdout::Declined),
        Flow::Top(_) => StageStdout::Declined,
    }
}

/// Budget = `4 * argv.len() + BUDGET_CONSTANT`. Generous: a correct argparse takes
/// O(argv) steps; the constant covers fixed prologue/epilogue statements.
const BUDGET_CONSTANT: usize = 32;

struct Evaluator {
    /// Current `$1..$n` (1-based; index 0 of this vec is `$1`). Mutated by `shift`.
    positionals: Vec<String>,
    /// Variable bindings from `name=value` assignments.
    vars: BTreeMap<Symbol, String>,
    /// The interned symbol of the conventional verb-binding name (from the check).
    /// An assignment whose lvalue equals this symbol records the verb — a comparison
    /// of *symbols*, never decoding the variable's text (`inv-referent-agnostic`).
    verb_sym: Symbol,
    /// The derived verb (the value most recently bound to [`Evaluator::verb_sym`]),
    /// if any.
    verb: Option<String>,
    /// Probe command spans reached on the selected path, in execution order.
    probe_body: Vec<Span>,
    /// The first inline annotation reached, resolved to (kind, entity).
    annotation: Option<(String, ResolvedEntity)>,
    /// The STDOUT coverage of the LAST producing command reached on the selected path
    /// ([`StageStdout`]) — the composed-probe coverage rule reads it to gate a non-last pipe
    /// stage. `None` until a command is reached (⇒ [`StageStdout::Declined`], no producer).
    last_stdout: Option<StageStdout>,
    budget: usize,
    steps: usize,
}

/// Control-flow result of running a statement / block. The dialect has no `return`
/// (none of 19H §2's examples use one), so normal fall-through and a ⊤ degrade are
/// the only outcomes.
enum Flow {
    /// Fell through normally.
    Normal,
    /// Degraded to ⊤ — propagates out immediately.
    Top(TopReason),
}

impl Evaluator {
    /// Build a fresh evaluator over a concrete `argv` (the shared constructor for [`evaluate`]
    /// and [`predict_stage_stdout`], so both trace the SAME argparse). Caller has already ruled
    /// out an empty argv.
    fn over(check: &Predict, argv: &[&str]) -> Self {
        Evaluator {
            positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
            vars: BTreeMap::new(),
            verb_sym: check.verb_sym,
            verb: None,
            probe_body: Vec::new(),
            annotation: None,
            last_stdout: None,
            budget: argv.len().saturating_mul(4).saturating_add(BUDGET_CONSTANT),
            steps: 0,
        }
    }

    /// Charge one step against the budget; `Err` ⇒ budget exhausted.
    fn tick(&mut self) -> Result<(), TopReason> {
        self.steps = self.steps.saturating_add(1);
        if self.steps > self.budget {
            Err(TopReason::BudgetExceeded)
        } else {
            Ok(())
        }
    }

    fn run_block(&mut self, body: &[Stmt]) -> Flow {
        for stmt in body {
            match self.run_stmt(stmt) {
                Flow::Normal => {}
                top @ Flow::Top(_) => return top,
            }
        }
        Flow::Normal
    }

    fn run_stmt(&mut self, stmt: &Stmt) -> Flow {
        if let Err(reason) = self.tick() {
            return Flow::Top(reason);
        }
        match stmt {
            Stmt::Assign { name, value } => self.run_assign(*name, value),
            Stmt::Shift { count } => self.run_shift(count.unwrap_or(1)),
            Stmt::While { test, body } => self.run_while(test, body),
            Stmt::Case { scrutinee, arms } => self.run_case(scrutinee, arms),
            Stmt::If {
                test,
                then_body,
                else_body,
            } => self.run_if(test, then_body, else_body),
            Stmt::Annotation(anno) => self.run_annotation(anno),
            Stmt::Command(cmd) => {
                // 24E §14: a PIPELINE on the selected path is not a modeled probe (the tracer
                // cannot resolve its dataflow) ⇒ ⊤ ⇒ the site can't-resolve ⇒ it RUNS (the safe
                // degrade, `kFAIL-perform`). Parse-permissively (it lifted, ships byte-exact);
                // trace-conservatively (⊤ here). Checked BEFORE recording a probe span.
                if cmd.pipeline {
                    return Flow::Top(TopReason::Pipeline);
                }
                // a probe body on the selected path: record its verbatim span (we run
                // statically — the span ships into the probe artifact, C-1). A trailing
                // effect mark (`cmd.mark`) is metadata for the lift/strip only; it does
                // not change what the probe command DOES, so evaluation ignores it.
                self.probe_body.push(cmd.span);
                // §2 per-channel STDOUT coverage (composed-probe rule): the last producing
                // command reached IS the arm's stdout, so a later command overwrites this.
                self.last_stdout = Some(stage_stdout_of(cmd));
                Flow::Normal
            }
        }
    }

    fn run_assign(&mut self, name: Symbol, value: &Word) -> Flow {
        match self.resolve(value) {
            Ok(v) => {
                if name == self.verb_sym {
                    self.verb = Some(v.clone());
                }
                self.vars.insert(name, v);
                Flow::Normal
            }
            // A non-concrete rvalue makes the binding unknown. We do NOT bind it to a
            // bogus value (that would risk a wrong downstream resolution); the var
            // stays unbound, and any later use of it degrades to Top. If this was the
            // verb binding, the verb also stays absent (the safe outcome).
            Err(_reason) => Flow::Normal,
        }
    }

    fn run_shift(&mut self, count: u32) -> Flow {
        let n = count as usize;
        if n > self.positionals.len() {
            // `shift` past the end is a runtime error in sh; bias to Top.
            return Flow::Top(TopReason::NonConcreteWord("shift past end of argv"));
        }
        self.positionals.drain(0..n);
        Flow::Normal
    }

    fn run_while(&mut self, test: &Test, body: &[Stmt]) -> Flow {
        loop {
            if let Err(reason) = self.tick() {
                return Flow::Top(reason);
            }
            match self.eval_test(test) {
                Ok(true) => match self.run_block(body) {
                    Flow::Normal => {}
                    top @ Flow::Top(_) => return top,
                },
                Ok(false) => return Flow::Normal,
                Err(reason) => return Flow::Top(reason),
            }
        }
    }

    fn run_if(&mut self, test: &Test, then_body: &[Stmt], else_body: &[Stmt]) -> Flow {
        match self.eval_test(test) {
            Ok(true) => self.run_block(then_body),
            Ok(false) => self.run_block(else_body),
            Err(reason) => Flow::Top(reason),
        }
    }

    fn run_case(&mut self, scrutinee: &Word, arms: &[super::ast::CaseArm]) -> Flow {
        let value = match self.resolve(scrutinee) {
            Ok(v) => v,
            Err(reason) => return Flow::Top(reason),
        };
        for arm in arms {
            if arm.patterns.iter().any(|p| pattern_matches(p, &value)) {
                return self.run_block(&arm.body); // sh: first matching arm only
            }
        }
        // No arm matched and no `*` catch-all: real sh falls through with no effect.
        // We do the same (a flag-strip `case $1 in -v) …` legitimately falls through
        // when the flag is absent). When the fall-through means no probe command ran
        // (a `case $verb` selecting the probe body — an unknown verb), `finish` turns
        // the empty probe_body into Top(NoProbeReached); we do not special-case it
        // here, keeping the evaluator faithful to sh semantics.
        Flow::Normal
    }

    fn run_annotation(&mut self, anno: &Annotation) -> Flow {
        // A value-less annotation is the nullary/Singleton form (`index : pkgindex`):
        // the verb's resource has no operand. A valued annotation resolves the operand.
        let entity = match &anno.value {
            None => ResolvedEntity::Singleton,
            Some(value) => match self.resolve(value) {
                Ok(text) => ResolvedEntity::Operand(text),
                Err(_) => return Flow::Top(TopReason::UnresolvedAnnotationValue),
            },
        };
        // First annotation wins (a check declares one entity-of-interest per path); a
        // second is ignored. Record kind + resolved entity.
        if self.annotation.is_none() {
            self.annotation = Some((anno.kind.clone(), entity));
        }
        Flow::Normal
    }

    /// Resolve a [`Word`] in the **strict** context (annotation value, `case`
    /// scrutinee, assignment RHS): an unset positional is non-concrete ⇒ `Err`. See
    /// [`resolve_word`].
    fn resolve(&self, word: &Word) -> Result<String, TopReason> {
        resolve_word(word, &self.positionals, &self.vars, UnsetPolicy::Unresolved)
    }

    /// Evaluate a `[ LHS OP RHS ]` string-comparison test against this evaluator's state
    /// (delegates to the shared [`eval_test`]).
    fn eval_test(&self, test: &Test) -> Result<bool, TopReason> {
        eval_test(test, &self.positionals, &self.vars)
    }

    /// Assemble the final [`Resolution`] from accumulated state. Two degrade gates:
    /// no annotation reached ⇒ `MissingAnnotation`; an annotation but no probe
    /// command on the selected path ⇒ `NoProbeReached` (a probe-less resolution is
    /// not actionable — `inv-kfail`).
    fn finish(self) -> Resolution {
        match self.annotation {
            None => Resolution::Top(TopReason::MissingAnnotation),
            Some(_) if self.probe_body.is_empty() => Resolution::Top(TopReason::NoProbeReached),
            Some((kind, entity)) => Resolution::Resolved(Resolved {
                kind,
                entity,
                verb: self.verb,
                probe_body: self.probe_body,
            }),
        }
    }
}

/// The STDOUT coverage a single reached [`Command`] produces (`273` §2 vocabulary,
/// `271:rul-only-oracle-bytes-ship` rider 1). A `>/dev/null`-class stdout redirect DECLINES the
/// channel; a `printf` head is an ASSERTED (declared, not world-spoken) claim — knife-tier-refused
/// (rider 3); anything else is a DELEGATION producing REAL bytes. (A pipeline arm never reaches
/// here — it ⊤s first, `TopReason::Pipeline`.) `inv-referent-agnostic`: keys on the command's own
/// structure — the head word `printf`, the redirect fd — never on what an operand MEANS.
fn stage_stdout_of(cmd: &Command) -> StageStdout {
    if cmd.stdout_void {
        return StageStdout::Declined;
    }
    match cmd.words.first() {
        Some(Word::Literal(w) | Word::SingleQuotedLiteral(w)) if w == "printf" => {
            StageStdout::Asserted
        }
        _ => StageStdout::RealBytes,
    }
}

/// Does a [`Pattern`] match the scrutinee value? Literal ⇒ exact equality; wildcard
/// ⇒ always. (No globbing — the parser already rejected non-trivial globs.)
///
/// `pub(crate)`: the touches-footprint evaluator ([`crate::touches`]) dispatches `case`
/// arms with the SAME sh semantics (24A §1b — one dialect, two collectors), so it reuses
/// this rather than re-deriving arm-selection.
pub(crate) fn pattern_matches(pattern: &Pattern, value: &str) -> bool {
    match pattern {
        Pattern::Literal(lit) => lit == value,
        Pattern::Wildcard => true,
    }
}

/// Resolve a [`Word`] to a concrete string against `positionals` (`$1..$n`, 1-based) and
/// `vars` (name bindings) under a named [`UnsetPolicy`] (the single home of the
/// unset-parameter context fork, `sem::UnsetPolicy`), or `Err` with a reason if it is
/// non-concrete.
///
/// A past-the-end positional / `${N#prefix}` forks on `policy`:
/// [`ExpandEmpty`](UnsetPolicy::ExpandEmpty) (test context) ⇒ the empty string;
/// [`Unresolved`](UnsetPolicy::Unresolved) (strict context) ⇒ `Err`. A `$0` or an
/// unbound *variable* is non-concrete under *both* policies (the safe direction).
///
/// `pub(crate)`: extracted from the predict evaluator so the touches-footprint evaluator
/// reuses the exact same word-resolution (positional/var/prefix-strip/unmodeled) — the
/// vocabulary fence (24A §1b) requires footprint fragments resolve through the SAME
/// value-flow as predict, not a parallel one.
pub(crate) fn resolve_word(
    word: &Word,
    positionals: &[String],
    vars: &BTreeMap<Symbol, String>,
    policy: UnsetPolicy,
) -> Result<String, TopReason> {
    let positional = |n: u32| -> Option<&str> {
        let idx = (n as usize).checked_sub(1)?;
        positionals.get(idx).map(String::as_str)
    };
    match word {
        Word::Literal(s) | Word::SingleQuotedLiteral(s) => Ok(s.clone()),
        Word::Positional(0) => Err(TopReason::NonConcreteWord("`$0` is not modeled")),
        Word::Positional(n) => match positional(*n) {
            Some(v) => Ok(v.to_owned()),
            None => unset_positional(policy),
        },
        Word::PositionalStripPrefix { n, prefix } => match positional(*n) {
            // literal-prefix shortest-match == the literal (`sem::strip_prefix_literal`)
            Some(val) => Ok(sem::strip_prefix_literal(val, prefix).to_owned()),
            None => unset_positional(policy),
        },
        // `${N-default}` — the `-` explicitly requests the default when unset, so it
        // resolves independent of `policy` (never `Unresolved`): `$N` if present, else
        // the literal default (the `${2-}` nounset idiom, `24P` §2).
        Word::PositionalDefault { n, default } => {
            Ok(positional(*n).map_or_else(|| default.clone(), str::to_owned))
        }
        Word::Var(sym) => vars
            .get(sym)
            .cloned()
            .ok_or(TopReason::NonConcreteWord("unbound variable")),
        // `"$@"` — the positional LIST — is genuinely ⊤ in VALUE position (annotation RHS,
        // `[ ]` operand, `case` scrutinee): a multi-value list is not one value (`27H`
        // finding-positional-oracle-side-couples-founding-pin). Its concrete COMMAND-position
        // use (a peel's guest) is handled by the command-running callers (verdict `run_command`,
        // the predict `Command` handler), which never route through here.
        Word::PositionalArgs => Err(TopReason::NonConcreteWord(
            "`\"$@\"` is the positional list — not one value (⊤ in value position)",
        )),
        // Unmodeled expansions fail in every position — including `[ ]` tests:
        // evaluating them as text or guessing dash's glob semantics would be a
        // wrong concrete.
        Word::Unmodeled(_) => Err(TopReason::NonConcreteWord("unmodeled parameter expansion")),
    }
}

/// Evaluate a `[ LHS OP RHS ]` string-comparison test against `positionals` + `vars`.
///
/// In a `[ … ]` test, a past-the-end positional is the **empty string**, faithful to sh
/// (an unset parameter expands to empty), NOT a degrade — so the flag-strip
/// `while [ "${1#-}" != "$1" ]` terminates cleanly when the argv is exhausted. `pub(crate)`
/// for the shared touches evaluator (same reason as [`resolve_word`]).
pub(crate) fn eval_test(
    test: &Test,
    positionals: &[String],
    vars: &BTreeMap<Symbol, String>,
) -> Result<bool, TopReason> {
    let lhs = resolve_word(&test.lhs, positionals, vars, UnsetPolicy::ExpandEmpty)?;
    let rhs = resolve_word(&test.rhs, positionals, vars, UnsetPolicy::ExpandEmpty)?;
    Ok(match test.op {
        TestOp::Eq => lhs == rhs,
        TestOp::Ne => lhs != rhs,
    })
}

/// The value of an *unset* positional under the [`UnsetPolicy`] fork (the single home
/// of the unset-parameter context rule, `sem::UnsetPolicy`): test context ⇒ empty
/// string (dash-faithful), strict context ⇒ non-concrete `Err` (the soundness floor).
fn unset_positional(policy: UnsetPolicy) -> Result<String, TopReason> {
    match policy {
        UnsetPolicy::ExpandEmpty => Ok(String::new()),
        UnsetPolicy::Unresolved => Err(TopReason::NonConcreteWord("positional past end of argv")),
    }
}

#[cfg(test)]
mod stage_stdout_tests {
    //! The composed-probe per-channel STDOUT coverage rule (`273` §2 vocabulary,
    //! `271:rul-only-oracle-bytes-ship` rider 1): a NON-LAST pipe stage may be model-substituted
    //! only if its predict arm produces REAL bytes on stdout. These pin the classification the plan
    //! crate's `connected_check_pipes` gates on — the exact reason the landed `24J` raw-ship debt
    //! (an `>/dev/null`-redirected otelcol predict) refuses to compose. Process-evidence, not proof.
    use super::{StageStdout, predict_stage_stdout};
    use crate::predict::lift_predicts;
    use dorc_core::Interner;

    fn stdout_of(src: &str, provider: &str, argv: &[&str]) -> StageStdout {
        let mut i = Interner::default();
        let out = lift_predicts(&mut i, src);
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let p = i.intern(provider);
        let check = out.value.get(p).expect("a check for the provider");
        predict_stage_stdout(check, argv)
    }

    #[test]
    fn delegation_arm_produces_real_bytes() {
        // The A6-converted otelcol shape: a bare delegation to the real read-only command, stdout on
        // the pipe ⇒ REAL bytes. This is the ONLY form a non-last pipe stage may take (rider 1).
        let src = "otelcol__predict() { case $1 in --version) otelcol --version ;; esac }";
        assert_eq!(
            stdout_of(src, "otelcol", &["--version"]),
            StageStdout::RealBytes,
            "a delegation arm with stdout on the pipe covers the byte-consumer downstream"
        );
    }

    #[test]
    fn stdout_redirect_to_null_declines() {
        // The LANDED `24J` raw-ship debt's exact shape: `otelcol --version >/dev/null 2>&1` VOIDS
        // stdout (§2 redirect-to-null decline). A non-last stage like this starves the downstream
        // `grep` ⇒ the compound must NOT ship (can't-say ⇒ run). This is why A6 had to CONVERT it.
        let src = "otelcol__predict() { case $1 in --version) otelcol --version >/dev/null 2>&1 ;; esac }";
        assert_eq!(
            stdout_of(src, "otelcol", &["--version"]),
            StageStdout::Declined,
            "a `>/dev/null` stdout redirect declines the channel the next stage consumes"
        );
    }

    #[test]
    fn stderr_only_redirect_keeps_stdout_on_the_pipe() {
        // `2>&1` / `2>/dev/null` redirect STDERR, never fd 1 — stdout still reaches the pipe. The
        // fd-discrimination is load-bearing: a stderr redirect must not be mistaken for a decline.
        let src = "grep__predict() { grep -q -- \"$1\" 2>/dev/null ;}";
        assert_eq!(
            stdout_of(src, "grep", &["x"]),
            StageStdout::RealBytes,
            "a `2>/dev/null` redirect leaves stdout on the pipe (only fd 2 is voided)"
        );
    }

    #[test]
    fn printf_arm_is_asserted_not_real() {
        // A `printf` head is an ASSERTED output claim (§2), NOT world-spoken bytes — rider 3
        // (capture-ships-real-bytes) refuses it for a knife-tier byte-consumer. Distinguished from a
        // decline for provenance; the plan gate treats both as "not RealBytes" ⇒ refuse.
        let src = "faux__predict() { printf '%s\\n' \"$1\" ;}";
        assert_eq!(
            stdout_of(src, "faux", &["x"]),
            StageStdout::Asserted,
            "a printf-headed arm asserts stdout — not the real bytes a knife-tier consumer needs"
        );
    }

    #[test]
    fn unmatched_arm_declines_whole_shape() {
        // A verb the argparse selects no arm for reaches NO producing command ⇒ whole-shape decline
        // (⊤). The compound refuses (never a guess about an unmodeled shape).
        let src = "otelcol__predict() { case $1 in --version) otelcol --version ;; esac }";
        assert_eq!(
            stdout_of(src, "otelcol", &["--unknown-verb"]),
            StageStdout::Declined,
            "an argv the arms do not select reaches no producer ⇒ declined"
        );
    }
}
