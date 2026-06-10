//! The concrete evaluator — trace a known argv through a [`Check`]'s argparse to
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

use super::ast::{Annotation, Check, Pattern, Stmt, Test, TestOp, Word};
use dorc_core::{Span, Symbol};
use dorc_syntax::sem::{self, UnsetPolicy};
use std::collections::BTreeMap;

/// The result of evaluating a [`Check`] over a concrete argv.
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
pub fn evaluate(check: &Check, argv: &[&str]) -> Resolution {
    if argv.is_empty() {
        return Resolution::Top(TopReason::EmptyArgv);
    }
    let budget = argv.len().saturating_mul(4).saturating_add(BUDGET_CONSTANT);
    let mut ev = Evaluator {
        positionals: argv.iter().map(|s| (*s).to_owned()).collect(),
        vars: BTreeMap::new(),
        verb_sym: check.verb_sym,
        verb: None,
        probe_body: Vec::new(),
        annotation: None,
        budget,
        steps: 0,
    };
    match ev.run_block(&check.body) {
        Flow::Normal => ev.finish(),
        Flow::Top(reason) => Resolution::Top(reason),
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
                // a probe body on the selected path: record its verbatim span (we run
                // statically — the span ships into the probe artifact, C-1)
                self.probe_body.push(cmd.span);
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
    /// [`Evaluator::resolve_with`].
    fn resolve(&self, word: &Word) -> Result<String, TopReason> {
        self.resolve_with(word, UnsetPolicy::Unresolved)
    }

    /// Resolve a [`Word`] to a concrete string against the current positionals and
    /// bindings under a named [`UnsetPolicy`] (the single home of the unset-parameter
    /// context fork, `sem::UnsetPolicy`), or `Err` with a reason if it is non-concrete.
    ///
    /// A past-the-end positional / `${N#prefix}` forks on `policy`:
    /// [`ExpandEmpty`](UnsetPolicy::ExpandEmpty) (test context) ⇒ the empty string;
    /// [`Unresolved`](UnsetPolicy::Unresolved) (strict context) ⇒ `Err`. A `$0` or an
    /// unbound *variable* is non-concrete under *both* policies (the safe direction).
    fn resolve_with(&self, word: &Word, policy: UnsetPolicy) -> Result<String, TopReason> {
        match word {
            Word::Literal(s) | Word::SingleQuotedLiteral(s) => Ok(s.clone()),
            Word::Positional(0) => Err(TopReason::NonConcreteWord("`$0` is not modeled")),
            Word::Positional(n) => match self.positional(*n) {
                Some(v) => Ok(v.to_owned()),
                None => unset_positional(policy),
            },
            Word::PositionalStripPrefix { n, prefix } => match self.positional(*n) {
                // literal-prefix shortest-match == the literal (`sem::strip_prefix_literal`)
                Some(val) => Ok(sem::strip_prefix_literal(val, prefix).to_owned()),
                None => unset_positional(policy),
            },
            Word::Var(sym) => self
                .vars
                .get(sym)
                .cloned()
                .ok_or(TopReason::NonConcreteWord("unbound variable")),
            // Unmodeled expansions fail in every position — including `[ ]` tests:
            // evaluating them as text or guessing dash's glob semantics would be a
            // wrong concrete.
            Word::Unmodeled(_) => Err(TopReason::NonConcreteWord("unmodeled parameter expansion")),
        }
    }

    /// `$n` (1-based) of the current positionals, if in range.
    fn positional(&self, n: u32) -> Option<&str> {
        let idx = (n as usize).checked_sub(1)?;
        self.positionals.get(idx).map(String::as_str)
    }

    /// Evaluate a `[ LHS OP RHS ]` string-comparison test.
    ///
    /// In a `[ … ]` test, a past-the-end positional is the **empty string**, faithful
    /// to sh (an unset parameter expands to empty), NOT a degrade — so the flag-strip
    /// `while [ "${1#-}" != "$1" ]` terminates cleanly when the argv is exhausted, and
    /// an "is there a second operand?" guard `[ "$2" = "" ]` reads true at the end.
    /// (The ANNOTATION value-position stays strict — past-end ⇒ Top — because an
    /// entity must resolve concretely; only the *test* context takes sh's unset-empty
    /// semantics. A `$0`/unbound-var is still non-concrete ⇒ Top, the safe direction.)
    fn eval_test(&self, test: &Test) -> Result<bool, TopReason> {
        let lhs = self.resolve_with(&test.lhs, UnsetPolicy::ExpandEmpty)?;
        let rhs = self.resolve_with(&test.rhs, UnsetPolicy::ExpandEmpty)?;
        Ok(match test.op {
            TestOp::Eq => lhs == rhs,
            TestOp::Ne => lhs != rhs,
        })
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

/// Does a [`Pattern`] match the scrutinee value? Literal ⇒ exact equality; wildcard
/// ⇒ always. (No globbing — the parser already rejected non-trivial globs.)
fn pattern_matches(pattern: &Pattern, value: &str) -> bool {
    match pattern {
        Pattern::Literal(lit) => lit == value,
        Pattern::Wildcard => true,
    }
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
