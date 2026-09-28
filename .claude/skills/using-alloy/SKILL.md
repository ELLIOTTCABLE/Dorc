---
name: using-alloy
description: >-
  Praxis for writing, editing, checking and debugging Alloy 6 models so that a passing check means
  something: the authoring loop, the catalogue of ways a check passes for the wrong reason, the
  semantic traps, the behavioural idiom, test instances, scope and cost, and the failure modes
  specific to language models. Read in full before touching any .als or Alloy Markdown file.
when_to_use: >-
  Load whenever a task writes, edits, runs, or interprets Alloy: a new model from prose, a change to
  an existing model, a red counterexample, an unexpected green, a command with no instance, a
  timeout, or a scope question. Skip only for reading Alloy without changing or running it.
---

# Using Alloy without fooling yourself

Alloy's product is the counterexample. A `check` that finds nothing has not said anything: it is the
absence of a message, and there are at least eight ways to get that absence with a false claim.
Every rule below exists to make a green mean what it appears to mean.

## Load-bearing facts, repeated on purpose

- A `fact` removes worlds. Two facts that contradict remove every world, and from then on every
  `check` passes and every `run` fails, with no message. The only detector is a `run` that finds an
  instance.
- The solver fills every gap you leave. In a `run` it fills them in your favour; in a `check` it
  fills them against you. A relation you did not constrain takes any value; a mutable relation an
  event does not mention changes freely.
- "No counterexample for 3" is a fact about worlds of size three. Never write "proved".
- A plain `fact` binds the initial state only. Signature declarations, multiplicities and signature
  facts hold in every state. A quantifier outside `always` over a set that changes is evaluated
  at time zero.
- `in` is subset, not membership. `iden` covers every atom in the universe. `no`/`one`/`lone` over
  several variables is not the nested form. Quantified variables are not distinct unless you write
  `disj`.
- Analyzer options (recursion depth, overflow handling, warnings, skolem depth) change answers. A
  model whose verdict depends on an option is malformed. Keep the defaults and never cite an option
  in a fix.

## The loop

Every Alloy task is this loop, whether the model is new or fifty edits old. Do the steps in order;
each names what stops you.

1. **Declare.** Signatures, then fields, with every multiplicity written even when it is the
   default `one`. Use `extends` for disjoint kinds and `in` for overlapping roles; mark a parent
   `abstract` only when it has extensions (otherwise the keyword does nothing). No `Int`, no
   `String`, unless arithmetic is genuinely needed; identity is an uninterpreted atom, comparison
   is an explicit order.
2. **Inhabit before anything else.** Add `run example {}` and execute it. If it has no instance the
   declarations already contradict. Keep this run for the life of the model; it is the
   consistency probe you re-execute after every change to a fact.
3. **Constrain from instances.** Press through instances. Every fact is written in answer to a
   specific instance that exhibited something wrong, carries a name, and carries one English
   sentence above it saying what it forbids. A fact written from imagination rather than from an
   instance is the first place vacuity enters.
4. **Re-inhabit.** After any fact changes, execute the empty run again and page through a few
   instances. Facts that admit only trivial worlds (empty ones, one-atom ones) make every later
   check vacuous.
5. **Claim.** Write the property as a named `assert`, and the command as `check name for N` with
   the scope stated. Never `check name {}`, and never `run name { ... }` where `name` is a predicate:
   a command with a block is a label plus that block, the assertion or predicate of that name is not
   applied, and an empty block checks `true` under a reassuring name and can never fail.
6. **Witness the premise.** For every `check` whose assertion has the shape `premise implies
   conclusion`, add a `run` of the premise and confirm it is satisfiable at the same scope; ask
   that run for the strongest world the law is meant to cover, not merely any world. A check with
   an unsatisfiable premise is green forever and has never been read.
7. **Kill.** Delete or negate one constraint the claim depends on and confirm something goes red.
   The kills worth trying are the mutations a mutation tool tries: flip a multiplicity, swap
   `all`/`some`, swap `^`/`*`, swap `implies`/`iff` or `and`/`or`, insert or drop a `~`, reverse
   the operands of a join, swap the branches of an `else`, drop a prime, drop an `always`. A claim
   that nothing can kill is either a restated definition or protected by facts that are too
   strong. Export the counterexample as a predicate and keep it as a rejection run.
8. **Read the scopes.** The Analyzer reports the scopes it actually used; read them. Grow a scope
   only when a witness genuinely needs the atoms, and say why beside the command. A scope is never
   the fix for a red.
9. **Record.** Name every command, mark expected outcomes with `expect 1` or `expect 0`, and keep
   the model executable end to end with Execute All.

In the hot loop, execute one command, not the file. Every command has a name; the whole file is
for the end.

## Starting from prose

Classify every sentence before writing any Alloy:

- An **assumption** about the world becomes a `fact`. Be stingy: this is the knob that removes
  worlds, and a fact can be tested negatively only by an `expect 0` that scope starvation fakes;
  a rule you will ever reject instances against is better a predicate (see Testing).
- A **claim** you expect to follow becomes an `assert` with a `check`. When unsure whether a
  sentence is assumption or claim, make it a claim; a claim that turns out to be an assumption
  costs one check, an assumption that should have been a claim silently hides bugs. Never put in a
  fact the thing you are about to check (the leader-election model deliberately leaves `Elected`
  unrestricted because "at most one leader" is the property).
- A **definition** becomes a `fun` or `pred`.
- An **example** or **counter-example** in the prose becomes a scenario `run`, positive or
  negative, in the some/disj idiom (see Testing).
- **Configuration** that never changes during an execution is static; **state** is `var`.
  Separate them first; the static part is what a check quantifies over "for all configurations up
  to scope", which is Alloy's main advantage over tools that check one configuration at a time.

Abstract hard. Replace record-like signatures whose atoms are "created" during execution with
tuples in a higher-arity relation where the payload is small; replace stored state that is a
function of history with a `fun` over `once` and `before`; remove any signature whose only job is
to be counted.

## Editing an existing model

- Read every `fact` before anything else; they define the universe every command lives in, and an
  opened module's facts count.
- Before composing any formula, inventory the model's own `fun` and `pred` definitions and use the
  one that names the concept. A formula rebuilt from primitives beside an existing helper loses
  the name that carried the intent; two independent agents did exactly this on a memory-model
  repair and both missed the accepted fix.
- To restructure a fact or predicate, keep the old body under a new name and `check { old iff
  new }` at the working scope before deleting it. That is the unit-level form of a rewording
  moving nothing.
- Add a check before changing a fact, so the change has a witness in both directions.
- A rewording, a rename, or a restructuring must leave every command's outcome unchanged; a
  moved outcome is a finding, not churn.
- A new constraint goes where its unit lives: a rule about every world in a fact or signature
  fact; a rule about one situation in the scenario run; a rule that narrows one claim in that
  claim's premise. Adding a fact to make one check pass narrows every other check too.
- A new event needs a guard, effects written as equalities, frame conditions for every mutable
  relation it does not change, membership in the transitions disjunction, and an enabledness
  predicate if fairness is in play.
- A new signature changes scope arithmetic (see Scope and cost) and may make an existing
  overloaded field ambiguous; fix ambiguity with `S <: f`, never by renaming into the dark.
- After the edit: the empty run, the premise twins, one kill, then the full file.

## Vacuity and false confidence: the catalogue

Each entry: how it looks, why it happens, how you catch it.

- **Every check green, every run empty.** Inconsistent facts. Catch: the empty run has no
  instance.
- **A check is green no matter what you break.** The assertion restates a fact or a definition,
  or its premise is unsatisfiable. Catch: the kill step and the premise twin.
- **`check name {}` is green.** It checks `true`. Catch: grep for commands with an empty block that
  share a name with an assertion.
- **A run over a totally ordered signature finds nothing, or a check over it is green at a size
  where it should fail.** `util/ordering` makes its parameter's scope exact and non-empty; three
  atoms may not fit the constraints. Catch: the reported scopes; hand-write the order when smaller
  configurations matter.
- **No trace at all once the model is "complete".** Traces are infinite; without a stuttering step
  a model that runs out of enabled actions has no trace. Catch: the empty run on the behavioural
  model. Include `stutter` when idle behaviour is valid in the domain; when the transition
  relation is total by design, adding it changes the model (idle traces appear and every liveness
  claim then needs fairness), so leave it out and let the empty run prove traces exist.
- **A scenario has no instance.** The scope cannot seat the atoms it needs: a `one sig` consumes
  its parent's scope; when all but one extension of an abstract signature is scoped, the last gets
  the remainder, possibly zero; a record-like signature needs one atom per distinct value
  combination the trace uses. Catch: the reported scopes, then the smallest scope that seats the
  witness; a rejection run (see Testing) tells starvation from rejection outright.
- **A check over integers is green at a bitwidth its literals do not fit.** At the default
  bitwidth of four, `10` reads as `-6` (measured on the pinned jar: `10 = -6` is satisfiable and
  `10 > 5` is not), so `all f: File | f.size > 10` has no counterexample with the overflow option
  off, and turning it on hides more by discarding overflowing instances. Catch: a `but N Int`
  every literal and sum fits.
- **A recursive function agrees with you up to depth three and then returns nothing.** With the
  recursion-depth option on (off by default, when recursion is refused loudly), a recursive
  function is unrolled to that depth and returns `none` past it. Catch: do not recurse; memoize
  into a field with a fact.
- **A temporal assertion is green trivially.** The quantifier sits outside `always` over a
  mutable set that is empty at time zero, or a past operator sits at top level where there is no
  past. Catch: move quantifiers inside `always`; use past operators only under a future one.
- **An inductive step has a counterexample although the invariant holds.** Induction considers
  every state satisfying the invariant, reachable or not. Catch: strengthen the invariant or
  check it as `traces implies always inv` instead; do not "confirm unreachability" with a run,
  which searches forever.
- **A liveness check is green because the fairness premise excuses a stuck state.** The
  enabledness predicate was written from the guard while the event's effects contradict for some
  argument, so the event is never truly enabled. Catch: a `run` that demands the event fire for
  every argument shape, including degenerate ones (a ring of one node). Validate liveness with
  runs before believing checks.
- **A higher-order quantifier compiles and then errors, or a `some x: set A` in a check.**
  An existential over a set or relation solves in a `run`, and a universal in a `check`, when
  every quantifier above it is of the same kind; under a quantifier of the other kind it is
  refused. Catch: rewrite with an explicit subset signature or accept the error.

## Reading a red, and reading "no instance"

Assume the model, not the tool. In order of likelihood the counterexample is:

1. An object you did not pin: a key, a parent, a record the world needed and the solver dropped or
   invented. Fix by a fact in the scenario or a rule in the species, never by scope.
2. A direction you inverted: an order read backwards, `all` for `some`, `*` where `^` was meant, a
   relation that may be "separate" from itself because you never said `a != b`.
3. A constraint that says something you did not mean. Fix the constraint, then re-witness.
4. Silence: nothing in the model speaks to the shape shown. That is a design question; hold it as
   a named predicate excluded from the premise of the affected check, and only there.
5. The tool, last.

Use the evaluator on the instance: type any expression or formula, call the model's own functions
and predicates, and refer to atoms by their evaluator names (`Dir$0`, not the theme's label).
Skolemized variables appear as `$command_variable` and can be queried. Bounded checking returns
the shortest counterexample first, so a long trace means every shorter one was ruled out. The
smallest shape also draws the eye: a self-loop shown against a cycle law is the two-cycle's
degenerate case, so fix the class the instance belongs to, never the instance. Read a trace as
configuration, initial state, the events in order, the loop edge, and the first failing state.

"No instance" for a `run` means the facts plus the scope forbid it, never that the property is
false. Check the reported scopes, the ordering-exactness trap, and stuttering before anything else;
then bisect: move the facts into named predicates and add them back to the run one at a time
until it goes unsat.

## Semantic traps, each with its fix

- Membership: `x in S` is subset. For an optional singleton, write `some x and x in S`.
- Equality: `a.f = b.g` holds when both sides are empty; `some a.f & b.g` is the overlap test.
  `Door.state = Unlocked` fails when `Door` is empty where `Door.state in Unlocked` holds; choose
  by whether the empty case should pass.
- Reflexivity: `(A <: iden) in R`, not `iden in R`.
- Emptiness: `no R`. `R = none` is an arity error for a binary `R`.
- `*r` includes the identity on the whole universe; use it only immediately before a join.
- `disj` binds one group of variables drawn from one set; `all disj x, y: A, w, z: B` leaves `w`
  and `z` free to coincide.
- `one x, y: A | P` counts pairs; `one x: A | one y: A | P` counts something else. Prefer `some`
  and `all` with several variables.
- `a.b[c]` is `(a.b)[c]`. `implies` and `;` associate right; `else` binds the nearest `implies`.
- `always`, `eventually`, `after`, `historically`, `once`, `before` bind tighter than `and`, `or`,
  `implies`: `always (A or B)` needs the parentheses.
- Prime distributes: `(s.m)'` is `s.(m')`. Write the frame macro `x = (x)'` as the book does; the
  pinned jar substitutes a macro's argument already parsed, so the bare form tested equivalent.
- `init; always next` leaves the first transition unconstrained: `;` is `and after`, so the
  `always` starts at state one. Write `init and always next`.
- `F until G` also asserts that `G` eventually holds; the form that does not is `G releases F`.
- `let` is substitution, so `let t = x | eventually (x > t)` compares a value with itself; nothing
  freezes a value across a temporal operator. Compare adjacent states with `x'` against `x`, or
  look back with `before` under a future operator.
- Parameter declarations on a `pred` or `fun` are checked when it is run directly and ignored when
  it is invoked. They are documentation. Put the constraint in the body if it matters.
- Inside a signature fact, fields are already `this.f`; another atom's field is `y.@f`.
- An effect written as inclusion, `n.id in n.outbox'`, leaves the rest of the set free. Write
  `n.outbox' = n.outbox + n.id`.
- `=` on sets of integers compares sets; `=<` and `>=` sum first. `1 + 2` is the set `{1, 2}`;
  `1 - 1` is empty; `sum File.size` deduplicates, `sum f: File | f.size` does not; the `Int`
  scope is a bitwidth that the overall scope does not change.
- `seq` has its own scope (default four); `add` and `insert` on a full sequence silently return it
  unchanged; the `Int` bitwidth must cover the `seq` bound.
- A mutable signature extending a static one is static (a warning says so); atoms never move
  between top-level or sibling signatures across time; with a mutable top-level signature `univ`
  and `iden` are mutable too.
- `enum` and `util/ordering` impose a total order and an exact scope; two enums make `first`
  ambiguous without a module alias.
- Modules are found by file name only; a module opened twice with different parameters needs `as`
  aliases; `private` hides names but the atoms stay in `univ`.
- Alloy 6 reserved `'` and `;` as symbols and `after always before enabled event eventually
  historically invariant modifies once releases since steps triggered until var` as keywords; a
  hyphen is an operator, never part of an identifier.

## Scope and cost

- Default scope is three per top-level signature, applied to top-level signatures only. Subset
  signatures take no scope (bound them with `#S = n` in the command); relations take none.
- Cost grows faster than linearly in scope, in bitwidth, and steeply in relation arity (a field
  of arity n may need on the order of two to the n-squared booleans; arity above three is
  rarely worth it).
- A `univ`-typed field defeats the type-based bounding and is the first thing to remove from a
  slow model. A `var` relation carries one hidden extra column, and the encoder refuses outright
  when a relation's domain size to the power of its arity exceeds a 32-bit integer.
- The reported primary variables are the honest size readout; vars and clauses follow from them.
- A timeout is a result about the encoding or the scope, not a reason to wait longer. Look for
  closures taken per pair, higher-arity fields, and integers; lower the scope on that command only.
- Bounded temporal checking defaults to ten steps and returns the shortest counterexample;
  `for 1.. steps` is unbounded and needs an external complete model checker, which the pinned jar
  does not ship, so it errors here. Check bounded first, then unbounded only when bounded finds
  nothing and a checker exists.
- Inductive invariant checks run at one and two steps and are two orders faster than trace
  checks, at the price of refactoring `init` and `next` into predicates.
- SAT4J is the portable default and rarely the fastest; iterating instances needs an incremental
  solver; `parallel` decomposition speeds satisfiable runs and can slow unsatisfiable checks.

## Behavioural models

The idiom is a transition system spelled in temporal logic; every part is a plain predicate.

- State: `var` fields and `var sig X in Static {}`. Initial state: a fact with no temporal
  operator, or `all v: var$ | no v.value` using the meta signatures.
- One predicate per event, in this order inside the body: guard, effects as equalities on the
  primed relation, frame conditions for every other mutable relation (or the generic
  `all v: var$ - changed$ | v.value = v.value'`).
- A `stutter` predicate of frame conditions only, and a transitions fact
  `always (stutter or some x | event[x] or ...)`, with `some` and never `one`: `one` over-constrains
  and hides a missing frame condition, since two events with the same effect may no longer
  coincide. Without stutter there is no infinite trace once actions run out, and the model cannot
  compose with anything else.
- Facts for the initial state and the transitions bind every command to the system, which is what
  scenario runs want. When inductive checks or scenarios outside the system are wanted, `init`
  and `next` become predicates and `traces` the premise of every check; then every scenario run
  must state `traces` itself, or it constrains nothing and finds a garbage instance quietly.
- A history guard is one operator: `historically t not in File.shared`. Derived state that is a
  function of history is a `fun` over `once` and `before`, not a stored relation with effects.
- Safety: `always P`. "From then on": `always (E implies after always Q)`. Two states ahead:
  `x''`. Liveness: `eventually P`, and it needs fairness.
- Fairness lives in a predicate and is used as a premise, `check { fairness implies property }`,
  never as a fact: a fairness fact taxes every safety check too. Weak fairness for an event `A`
  with enabledness `en`: `always ((always en) implies always eventually A)`. Enabledness is the
  guard only when the guard is the whole story; a body with `some x: E | ...` needs `some E`, and a
  body whose effects can contradict for some argument is not enabled then.
- Past operators only under a future one; `before` is false at time zero; a formula in a command
  or fact is evaluated at time zero.
- Validate by scenario runs and by forking instances at chosen states before checking; a
  behavioural check with no runs behind it is untrusted.

## Testing and regression

- A concrete instance is a `run` in the some/disj idiom: `some disj d0, d1: Dir, disj f0, f1:
  File { Dir = d0 + d1  File = f0 + f1  entries = d0->e0 + ... }` with a scope that seats it.
  `disj` per group; every signature and field pinned to the union of its variables so no stray
  atom appears. Never introduce `one sig` atoms for a test: they exist in every command's
  universe and break symmetry.
- A negative instance is a rejection run, `run { valuation and not rule } expect 1`, whenever the
  rule is a predicate: satisfiable means rejected, and a scope that cannot seat the valuation
  comes back unsat and loud, so the test cannot pass by starvation. Write any rule you will ever
  reject instances against as a predicate for this reason. Only a fact forces the weaker form,
  the same valuation `expect 0`, which starvation fakes; that form needs the positive instance
  at the same scope beside it as its twin. `expect 1` disables symmetry breaking on that command.
- Coverage of a signature or relation is three instances: empty, one, two or more; of a formula,
  one instance where it holds and one where it fails. The empty case is the one nobody writes and
  the one a vacuous universal hides in.
- A counterexample the Analyzer found can be exported as a predicate in exactly this idiom; after
  the fix, keep it as a negative test.
- Trace scenarios: pin the configuration with some/disj, then states joined by `;` (lowest
  precedence; `p; q` is `p and after q`), closing with `always` on the last state if the
  continuation must be fixed; or the event-oriented form `upload[f]; share[f, t]; ...; always
  stutter`, which hides concrete values when events are non-deterministic.
- Event depiction: an `enum Event` and one derived relation per event, `fun upload_happens:
  Event -> File { { e: Upload, f: File | upload[f] } }`, lets `always some events` be the
  transitions fact and `always lone events` a check that finds steps where two events coincide
  (the book's own finds one and marks it `expect 1`), at no solver cost.
- Kill tests: for each law, delete one unit it rests on and watch it go red; a law nothing kills
  is decoration or restatement. Nothing in the tool catches a check that restates its own
  definition; only this habit does.

## Discipline specific to language models

Measured behaviour, not speculation: asked for twenty formulas for one-line properties, current
models produced syntax errors in up to fourteen of twenty and semantically wrong formulas in up to
eighteen of twenty, and one round of error feedback fixed the syntax errors. The Analyzer is the
only reliable reviewer in the loop, and it can only review what is asked of it.

- Never add a fact to make a check pass. A fact is a claim about every world. If a check is red for
  a case the design has not decided, exclude that case from that check's premise by a named
  predicate and stop there.
- Never widen a scope to make a premise twin satisfiable unless the witness truly needs the atoms,
  and never widen it as a fix for a red.
- Never write a check that restates a fact or a definition. Kill-test every claim once.
- Never report "proved". Report "no counterexample at scope N" verbatim, and report the scopes
  the Analyzer used with every green.
- Write test instances from the situation being modelled, never by reading the model; an instance
  derived from the model can only agree with it.
- Parse before you solve: a syntax error costs a solver launch; run the smallest command that
  parses the whole file first.
- Execute one named command in the loop; execute the file only at the end. Do not spend a solver
  run confirming what an instance you already have would show in the evaluator.
- A rewording must move nothing. If an outcome moved under a rename or a restructuring, stop and
  report it as a finding.
- Read the counterexample before touching anything. Rebuild it by hand from the printed instance;
  it is always small enough.
- Never rely on an Analyzer option, and never propose one as a fix.
- Never write the pre-6 idiom: no `sig Time` or `sig State`, no `util/ordering[Time]`, no
  `Time`-indexed fields. Training data is dominated by it, half of one 2024 cohort reached for it
  under Alloy 6, and it forfeits `var`, the temporal operators, and the trace semantics.

## Quick orientation

- Alloy Markdown: the Analyzer reads `.md` files whose first lines are a YAML header (`---`, then
  fields, then `---`) followed by fenced `alloy` blocks; a file without the header is not read.
- Modules resolve by file name relative to the main file's directory, then the library directory.
  On Windows the pinned jar refuses to parse `util/natural` (an alias clash inside the library
  file); run such a model under WSL.
- In this repository a model runs with `mise run alloy -- <file.als>`: one bounded JVM per
  command, `--command <name>` for one command, `--instances` to carry the counterexample in the
  row. A command carrying an `expect` is red only when its verdict disagrees; one without is red
  when a run is unsatisfiable or a check finds a counterexample; exit 75 is the machine-wide
  heavy-work lock, so do other work and retry.
- Higher-order quantification is accepted by the grammar and solved only where Skolemization
  applies; anything else is an error, not a slow run.
- Rare shapes worth copying (hand-written order without `util/ordering`, generator and uniqueness
  predicates for record-like signatures, messages as tuples, the inductive-invariant refactor,
  meta-signature frames, memoization instead of recursion, the two macros) are in `examples.md`
  beside this file; load it when a matching task appears, not before.
- First-party sources, in order of usefulness: Practical Alloy (practicalalloy.github.io), by the
  Alloy 6 designers, with every example in a runnable model repository; the language reference
  (alloytools.org/spec.html); Hillel Wayne's reference (alloy.readthedocs.io). A line-cited
  digest of what those sources warn about, the experiment behind the language-model numbers, and
  the practitioner prior art live in this repository under
  `.claude/research/design-model-mechanisation-prior-art/` (`turn06` for the first-party read,
  `turn05` for the field, `turn07` for the peer skills and the testing literature, `plan.md` for
  the map).
