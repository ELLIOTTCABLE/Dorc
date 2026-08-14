---
name: verified-core-discipline
description: The theory and praxis of this project's multi-tier correctness architecture — the verified core (algebra, checkers, Lean model) and the check-ladder around it (types, Kani, property tests, DST, runtime certification, proofs). Exists chiefly for agents whose MAIN task is something else — it tells you when you have wandered near something correctness-critical, what the thing in your way actually guarantees, and how to proceed without weakening it. The one law above all: never weaken the question to pass a check.
when_to_use: Load BEFORE proceeding when any of these holds, even (especially) if your task is unrelated and the obstacle feels incidental — you are editing files in the verified core (lattice/solver/domain code, comparison or claim chokepoints, checker or reference implementations, the Lean model, Kani harnesses, DST pins); a mechanical check failed or flaked under you (a Kani harness, a certification `Refused`, a property or DST seed, a Lean build, a lexical-fence test); you are adding a dependency, collection, panic path, or public constructor to a core crate; a signature change ripples into proofs or harnesses elsewhere; you must decide where a new invariant gets enforced; or you notice an urge to edit a test, harness, cap, spec, or theorem statement to get unblocked. Skip for prose/docs-only work and for code wholly outside the core with all checks green.
---

# Working near the verified core

This project runs a layered correctness architecture. A small **verified core**
(the abstract-domain algebra: lattices and their laws, the fixpoint solver, the
coordinate-comparison and claim/license chokepoints, the sparing/composition
algebra) is surrounded by a **check-ladder** of increasingly strong instruments,
and the big, churny engine above it is deliberately *untrusted* — its answers
are checked, not believed. If your task brushes any of this, the rules below
apply to you, whatever you were originally asked to do.

Two facts orient everything:

- **The checks are product surfaces, not chores.** A silently-wrong analysis
  under-executes a user's command — this codebase's cardinal sin — and a
  wrongly-attributed failure is worse than the failure. Explanation output is
  held to the same bar as decision output: the two planes (license and
  narrative/aid) carry separate gates, and *either* can veto a kernel change.
- **Every instrument is deliberately placed.** A five-line harness or a
  one-line `⊑` check may be load-bearing for thousands of lines of untrusted
  engine. Small ≠ incidental. If something tiny and strict is in your way, it
  is in your way on purpose.

## Am I near it? (recognition, for the unrelated-task agent)

You are near the verified core if any of these is true:

- Your edit touches lattice/solver/domain definitions, a `*_covers`/`compare`/
  `classify_*`-style chokepoint, a claim/license type, a checker or reference
  implementation, an owned-collection facade, `.lean` files, `#[kani::proof]`
  harnesses, or DST pin/seed machinery.
- A check failed that you did not write: a Kani counterexample, a certification
  `Refused(witness)`, a property-test or DST seed failure, a broken Lean build,
  a lexical-fence test naming a function you touched.
- You are about to add to a core crate: a dependency, a std collection, an
  `unwrap`/`panic!`, a generic that crosses the verification boundary, or a
  constructor that bypasses a smart-constructor.
- Your rename/signature change breaks something in a directory you have never
  visited (harnesses and proofs reference core signatures by construction).

When in doubt, assume you are near it: the cost of loading this discipline is
minutes; the cost of routing around a load-bearing check is a silent
wrong-elision.

## The one law: never weaken the question

An agent under pressure to finish an unrelated task will be tempted to make a
failing check pass by changing *the check*. Every form of this is forbidden:

- No `sorry`/`assume`/`admit` added to make a proof or harness green.
- No editing a theorem STATEMENT, a spec, a harness's bounds, or an
  `Arbitrary` instance to dodge the failing case.
- No converting, re-wrapping, or laundering a value to satisfy a
  stricter-typed signature (if a signature demands an authority you do not
  hold, you have the wrong value, not the wrong type).
- No raising iteration caps, loosening tolerances, or narrowing a
  quantified pin's input space.
- No `--no-verify`, no deleting-and-regoldening a check you do not understand.

A check you cannot satisfy is a finding. The correct moves are exactly two:
*fix the code* (your change genuinely broke a guaranteed property), or
*escalate with the failure in hand* (the check may be wrong or the spec may be
moving — that call belongs to the conductor/human, never to a task-focused
agent). Checks here are chosen for deterministic failure signals precisely so
this law is cheap to follow: a counterexample or witness names its operands.

**Flaky vs broken:** deterministic tiers (types, Kani, property seeds, DST,
certification) cannot flake — a failure is real, reproduce it from its seed or
witness. Solver-backed tiers (refinement annotations, some proof automation)
can be timing/heuristic-sensitive: re-run once to classify; if unstable,
escalate it *as instability* — never iterate until green, which launders a
real signal into noise.

## The ladder (theory): who does the universal quantifier

Every instrument answers "for all inputs, does P hold?" — they differ in who
does the *for all*, and each has a distinct failure signal:

| Tier | The ∀ is done by | Guarantees | Cannot do | Failure signal |
|---|---|---|---|---|
| Types / smart constructors | the compiler, forever | possession proves the check ran; no unchecked path exists | validate data that arrives at runtime beyond the boundary check | compile error (best signal there is) |
| Kani harnesses | exhaustive enumeration at small bounds | real ∀ over the bounded universe; laws hold for EVERY small case | unbounded claims; generic (un-monomorphized) code | concrete counterexample |
| Property tests | randomized generation | large-universe confidence, structural cases | exhaustiveness; rare-corner certainty | failing seed (replayable) |
| DST | seeded whole-system simulation | ordering, fault, and permutation coverage incl. output-multiset stability | model-level wrongness it shares with the code | failing seed (replayable) |
| Runtime certification | a per-answer check in production | THIS answer is safe, however it was computed | quality/precision; model bugs it shares with transfers | `Refused(witness)` naming the exact edge/operands |
| Lean proofs | a constructed argument, kernel-checked | unbounded ∀ over the modeled algebra | truth of the STATEMENTS; anything outside the model | broken build / unprovable goal |

Placement guidance for a NEW invariant: data-shape at an intake boundary →
smart constructor. Equational/∀-law over small values (lattice laws, meet/
compare algebra) → Kani harness, plus a property test if the universe is
large. Ordering/concurrency/whole-system → DST pin. "Is this computed answer
safe" → the certification layer. Algebra meta-theory → a Lean statement.
Prefer the *lowest* tier that fully expresses the property — lower tiers have
better failure signals and lower churn cost — and note that tiers compose: the
certifier leans on `⊑`, so the lattice laws it trusts are exactly what the
Kani tier pins.

## Praxis per tier

**Core code-shape** (binds all code in the verified core, today): no `unsafe`,
no interior mutability, no concurrency, no panic paths on any input; total
functions with closed outcome types. Monomorphic at the verification boundary
— generics live outside or are instantiated concretely at the seam. Std
collections never appear raw inside the core: state lives behind the project's
owned facades (small total APIs over sorted vecs), which is what keeps the
core translatable, checkable, and provable. An invariant lives at exactly ONE
named seat (a constructor or chokepoint function) and is never re-derived
inline at use-sites. Newtypes validate in private-field smart constructors;
harness access is granted by hand-written `#[cfg(kani)] Arbitrary` impls,
never by widening constructor visibility. New dependencies in core crates
require conductor ack, always.

**Kani**: a harness is ordinary Rust in the harness lane — `kani::any()`
inputs, assert the law, exhaustive at the declared bounds. Keep bounds honest
and small; a counterexample is a real bug in code or law — never harness
noise. When you add a lattice/domain/compare operation, you add its law
harnesses in the same change.

**Property tests / DST**: seeds are replay handles — a failure replays
deterministically; debug from the seed, never shrug at it. Permutation pins
assert order-independence of results *including narrative/output multisets*
(explanation stability is product correctness — convergence checks are blind
to it by construction, so only these pins protect it). Never hand-inject a
value a check is supposed to produce (anti-masking). Kernel code stays pure:
clock, randomness, filesystem, network only through injected seams.

**Runtime certification**: the solver's every answer passes a per-edge
post-fixpoint check; every sparing verdict is re-derived through an
independent reference implementation. Outcomes are closed types —
`Certified | Refused(witness)` — and consuming code handles `Refused` as
degrade-to-the-safe-floor plus an operand-carrying narrative record. Never
read certification as a boolean, never bypass it, never special-case "just
this once". If refusals appear after your change, your change is implicated:
the witness names the edge and both values — start there. The certifier's
refusal direction is pessimistic by design (capped or partial work degrades
toward the floor, never toward trust).

**The Lean model**: a small model of the core algebra lives beside the code;
its theorem STATEMENTS are the human-review surface, its proofs are
machine-checked (nobody reviews proofs; everybody may review statements). When
a core change breaks the model or its proofs, that is the system working —
re-align the model, re-prove, and if a *statement* must change to re-prove,
stop: that is a spec change and belongs to the conductor/human. `sorry` is a
loud, counted TODO ledger — legal mid-work, never silently merged. The model
also runs as a differential oracle inside DST, so keeping it aligned is not
optional polish.

## Escalation

Blocked by any of this? Say so, with the artifact in hand (the witness, the
counterexample, the seed, the failing goal). Flag judgment calls upward rather
than resolving them locally — especially anything that would move an invariant
between tiers, change a statement, or widen what a check accepts. The person
who owns the spec decides spec questions; you carry them the evidence.

## In this repository

Binding local law outranks this skill's generic phrasing: `spike/CLAUDE.md`
and the per-crate `CLAUDE.md` files (notably `spike/crates/core/` and
`spike/crates/analysis/`) carry the project-specific invariants and always
win on conflict. The verified core lives in those two crates; the
sparing-algebra model and its report live under
`.claude/research/refinement-types-industrial-cost/spike-lean-sparing/`.
Deep background — the evidence base, tradeoff pricing, and design rulings
behind this architecture: `.claude/research/refinement-types-industrial-cost/`
(start at the latest `turnNN` synthesis note), with kernel-currency context in
`Research/notes/28Q` / `28R`.
