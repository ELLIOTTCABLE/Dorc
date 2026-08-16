---
name: verified-core-discipline
description: The theory and praxis of this project's multi-tier correctness architecture — the verified core (the sorted facades, the lattice/solver algebra, the compare and claim chokepoints, the solve-certifier, the sparing reference model, the minispec law corpus) and the check-ladder around it (types, Kani, property tests, DST, runtime certification, Lean-over-derived-definitions). Exists chiefly for agents whose MAIN task is something else — it tells you when you have wandered near something correctness-critical, what the thing in your way actually guarantees, which instrument owns which kind of correctness, and how to proceed without weakening anything. The one law above all: never weaken the question to pass a check.
when_to_use: Load BEFORE proceeding when any of these holds, even (especially) if your task is unrelated and the obstacle feels incidental — you are editing files in the verified core (core::sorted, coord/claim chokepoints, analysis lattice/solve/certify, the sparing-reference crate, plan::rederive, anything under minispec/ or spike/verify/); a mechanical check failed or flaked under you (a Kani harness, a SolveConsistency Inconsistent, a re-derivation demotion, a property or DST seed, a lake build, the dorc-verify gate, a lexical fence); you are adding a dependency, collection, panic path, or public constructor to a core crate; a signature change ripples into harnesses, the derived Lean definitions, or the catalogue lock; you must decide where a NEW invariant gets enforced (the which-instrument section is the map); or you notice an urge to edit a test, harness, bound, fence, catalogue expectation, spec, or theorem statement to get unblocked. Skip for prose/docs-only work and for code wholly outside the core with all checks green.
---

# Working near the verified core

This project runs a layered correctness architecture. A small **verified core** —
the owned sorted facades (`core::sorted`), the abstract-domain algebra (lattices,
the fixpoint solver), the coordinate-comparison and claim/license chokepoints, the
sparing/composition algebra and its naive reference model, the solve-certifier —
is surrounded by a **check-ladder** of increasingly strong instruments, and the
big, churny engine above it is deliberately *untrusted*: its answers are checked,
not believed. A separate literate spec corpus (**minispec/**) states the few laws
the project opts to verify, over machine-derived Lean definitions of the shipping
code. If your task brushes any of this, the rules below apply to you, whatever you
were originally asked to do.

Two facts orient everything:

- **The checks are product surfaces, not chores.** A silently-wrong analysis
  under-executes a user's command — this codebase's cardinal sin — and a
  wrongly-attributed failure is worse than the failure. Explanation output is held
  to the same bar as decision output: the license and narrative/aid planes carry
  separate gates, and *either* can veto a kernel change.
- **Every instrument is deliberately placed, and they compose upward.** The
  certifier trusts `⊑`; `⊑` rests on the facade canonicality seats; the seats are
  pinned by Kani; the minispec laws are stated over Lean definitions derived from
  these exact files. A five-line harness or a one-line expectation in the
  catalogue lock may be load-bearing for thousands of lines of untrusted engine.
  Small ≠ incidental. If something tiny and strict is in your way, it is in your
  way on purpose.

## Am I near it? (recognition, for the unrelated-task agent)

You are near the verified core if any of these is true:

- Your edit touches: `core::sorted` or the `coord`/`claim` chokepoints;
  `analysis`'s `lattice.rs`/`solve.rs`/`certify.rs`; `dorc-sparing-reference` or
  `plan::rederive`; anything under `minispec/` (spec surface — see the access
  laws) or `spike/verify/` (the binder, the Kani harnesses, the Aeneas pipeline);
  a `*_covers`/`compare`/`classify_*`-style chokepoint; a checker or reference
  implementation; `#[kani::proof]` harnesses; DST pin/seed machinery; the
  catalogue lock.
- A check failed that you did not write: a Kani counterexample, a
  `SolveConsistency::Inconsistent`, a re-derivation demotion, a failing property
  or DST seed, a broken lake build, a `dorc-verify` refusal, a lexical-fence test
  naming a function you touched.
- You are about to add to a core crate: a dependency, a raw std collection, an
  `unwrap`/`panic!`, a generic that crosses the verification boundary, or a
  constructor that bypasses a smart-constructor.
- Your rename/signature change breaks something in a directory you have never
  visited (harnesses, derived Lean, and the catalogue reference core signatures
  by construction).

When in doubt, assume you are near it: the cost of loading this discipline is
minutes; the cost of routing around a load-bearing check is a silent
wrong-elision.

## The one law: never weaken the question

An agent under pressure to finish an unrelated task will be tempted to make a
failing check pass by changing *the check*. Every form of this is forbidden:

- No `sorry`/`assume`/`admit` to green a proof or harness; no editing a theorem
  STATEMENT, a law unit, a harness's bounds, or an `Arbitrary` instance to dodge
  the failing case.
- No editing a catalogue-lock expectation outside the promote ceremony (the gate
  refuses computed-vs-committed mismatch in EITHER direction — silent demotion
  and silent ambition are both loud, on purpose).
- No converting, re-wrapping, or laundering a value to satisfy a stricter-typed
  signature (if a signature demands an authority you do not hold, you hold the
  wrong value, not the wrong type).
- No raising iteration caps, loosening tolerances, widening a fence's allow-list,
  or narrowing a quantified pin's input space.
- No `--no-verify`, no deleting-and-regoldening a check you do not understand,
  and no re-blessing golden drift under a pure refactor (drift there is a
  finding: behavior changed).

A check you cannot satisfy is a finding. The correct moves are exactly two: *fix
the code*, or *escalate with the failure in hand* (the check may be wrong or the
spec may be moving — that call belongs to the conductor/human, never to a
task-focused agent).

**Flaky vs broken vs over-budget:** deterministic tiers (types, Kani at bounds,
property seeds, DST, certification) cannot flake — a failure is real; reproduce
it from its seed or witness. Solver-backed tiers can be resource-sensitive, and
there is a named trap: **CBMC prints `VERIFICATION:- FAILED` after its own
out-of-memory**, so a "failed" harness under memory pressure is NOT a
counterexample until reproduced under headroom — the harness driver gates verdict
on the memory result (keep it gated; a regression test pins it), and an
over-budget harness is *recorded as over-budget*, never reported refuted and
never waited out. Heavy solver work runs memory-gated (`timeout` + `ulimit -v` +
exact-name reaping, serialized across lanes — `spike/CLAUDE.md`
background-wsl-children-outlive-taskstop).

## Which instrument, when (the decision map)

Every instrument answers "for all inputs, does P hold?" — they differ in who does
the *for all*, what they can reach, and what they cost. Pick the LOWEST tier that
fully expresses the property; escalate only for reach, never for prestige.

| You have… | Reach for | Not |
|---|---|---|
| a data-shape rule at an intake boundary | a smart constructor (possession proves the check ran) | a runtime assert |
| an equational/∀-law over small algebra values (lattice laws, facade canonicality, meet/compare) | a **Kani harness**, added in the SAME change as the operation; a property-test twin if the universe is large | a hand-picked example test |
| an algebra-SEMANTICS statement other components lean on (what compare/join/sparing MEAN) | a **minispec law unit** — the expensive, human-gated lane; see below | a comment, or a Kani harness pretending to be a spec |
| a churny engine-tier numeric/structural invariant (byte budgets, span arithmetic, weft numerics) | today: checked arithmetic + ONE named seat + a property test; **Flux** is the penciled mid-r30 future here — keep the invariant at one seat so refinement lands mechanically | Kani or Lean (the algebra instruments do not reach the churny tier, by design — triple-covering was rejected) |
| "is this computed production answer safe?" | the EXISTING certification instruments only: the solve-certifier (every solver answer) and the sparing re-derivation (every survival). Adding a NEW runtime checker requires the admission tests: T1 a large find/check asymmetry in the producer (only fixpoints qualify), or T2 maximal severity × invisibility (only the survival lane qualified) | a checker added by symmetry — N-version programming with correlated blind spots |
| ordering/permutation/fault/whole-system behavior, incl. narrative-record multisets | **DST pins** (seeds are replay handles; permutation pins protect explanation stability, which convergence checks cannot see) | unit tests with hand-ordered inputs |
| product behavior — renders, plans, CLI contracts, oracle corpora | ordinary integration/e2e/goldens (the e2e runner is the SOLE fixture executor); goldens re-bless freely on arrangement churn, NEVER under pure refactors | any of the above; this is where traditional testing rules |

**The minispec lane, priced honestly.** minispec is the project's reviewable
statement of the few laws it opts to verify: English-authoritative prose + a
`Prop` over `minispec/Generated/` (Lean definitions machine-derived from the
shipping Rust — zero transcription drift) + a concretely-evaluated instance
battery, badge-tracked by `dorc-verify`. It is deliberately expensive:

- **Content is frontier+human only** (`minispec/CLAUDE.md` access laws). A
  builder NEVER edits a unit, the Vocabulary, or the catalogue expectations —
  builders build tooling, surface chafe, and stop. This is what makes it an
  acceptance surface the worker cannot game.
- **Spec leads the build.** When a design change moves algebra semantics
  (compare/dialect/backing/join — anything a law unit states), the spec changes
  FIRST, through the authorized lane, and code builds to spec-green. If your
  build looks right and the spec disagrees: STOP and report; that disagreement
  is the system working.
- **When does a law EARN a unit?** By deliberate human increment (the enrichment
  item), never as a side-effect of a lane. The trigger worth escalating: you find
  a semantic property that multiple components silently assume. Report it; do
  not mint it.
- **A green translate proves nothing alone.** The pipeline's truth is
  translate-STRICT + lake build + the hole/axiom census (lenient translation
  emits SILENT `sorry`s; one emission was silently ill-typed until lake caught
  it). Regeneration diffs of `Generated/` are the drift alarm — a breakage there
  after your refactor is the alarm working, not an obstacle.
- **Trusted-base hypotheses live IN the statements** (`Minispec/Vocabulary/
  TrustedBase.lean`): the derived generic dictionaries are lawless, so every
  generic law says what it assumes (`LawfulClone`/`LawfulEq`); concrete battery
  dictionaries prove them outright. The translated-code discipline that keeps the
  pipeline alive: keep borrows out of closure returns and Option-combinators off
  the algebra path — spell the `match` cousin (`core/CLAUDE.md`).

## Praxis per tier

**Core code-shape** (binds all verified-core code): no `unsafe`, no interior
mutability, no concurrency, no panic paths on any input; total functions with
closed outcome types. Raw std collections never appear — ordered state lives
behind `core::sorted::{SortedSet, SortedMap}` (canonical form is honour-system:
ONE named seat per invariant + seat tests now, Kani pins as the closing net).
Monomorphic at the Kani boundary (generics are fine for Aeneas; harnesses
instantiate concretely). An invariant lives at exactly ONE named seat and is
never re-derived inline. Newtypes validate in private-field smart constructors;
harness access via hand-written `#[cfg(kani)] Arbitrary` impls homed beside the
type — constructed via arbitrary backing + `kani::assume(canonical)`, NEVER via
repeated `insert` (that makes the insert harnesses circular). New dependencies in
core crates require conductor ack, always.

**Kani — authorship discipline** (measured, r30; the 107/107-green battery is
its evidence): `kani::any()` inputs, assert the law; harness home
`spike/verify/`; the lane is opt-in (`verify:kani`, Linux/WSL). When you add a
lattice/domain/compare/facade operation, you add its law harnesses in the same
change. Shape the FORMULA, never the budget — over-budget is a finding about
the question's shape, and raising a cap is the banned weaken-the-question move.
The measured shaping rules:

- CONCRETE lengths, one harness per length or length-pair, the size in the
  harness NAME (`…_at_length_2`) and as a const generic — never a number in
  prose (measured drift: doc-comments understating their own bounds). Each
  harness declares exactly the universe it verified; a gestured-at bound never
  covered is worse than a small one that was.
- The two unaffordable input shapes: a growing mutation at symbolic length, and
  two symbolic-length collections in one harness (measured: 2 s concrete vs
  21 min/3.6 GB symbolic on the same law).
- Concrete inputs are necessary, NOT sufficient — count the inserts INSIDE the
  operators. An operator that grows a collection element-by-element goes
  symbolic after its first insert, so a merge is affordable only when it
  performs ≤1 insert, and a law that composes one merge into another has NO
  affordable shape at any size. Such laws live at the property-test/seat-test
  tier instead; say so where the harness would have been, and do not chase.
- Generators: never build values by the mutation under test (circular). Draw
  arbitrary backing + `kani::assume(<invariant>)` — and every assumed invariant
  is PAIRED with a closing harness proving the real producer maintains it (an
  unpaired assume is a hole). Guard generator vacuity with an in-generator
  `assert!` (Kani proves it; an unsatisfiable assume greens everything).
- Escalate for reach, never prestige: large or two-collection universes →
  property tests; ordering/multiset stability → DST; raw-BTree-backed types →
  exhaustive-small beside the type until it sits on the facade.

A counterexample is a real bug in code or law — but apply the over-budget/OOM
verdict discipline above before believing a FAILED.

**Runtime certification**: the solver's every production answer passes the
post-fixpoint check (`solve_certified`; raw `solve`/`run` are `pub(crate)` and
lexically fenced); every survival verdict re-derives through the independent
reference model (demote-only — agreement licenses nothing new). Outcomes are
closed types; consumers take NAMED floors; `trusted()` means *certified* — the
`converged` flag is advisory (consistent-at-cap is the least fixpoint and is
used). An `Inconsistent` demotes the ENTIRE analysis window; summaries explain,
never scope; there is no recovery mode and none may be added (`302` §9: recovery
failures correlate with the trigger's causes). If refusals appear after your
change, your change is implicated — the witness names the edge and both values;
start there. And never gate a graceful-degrade path behind a `debug_assert`: DST
runs debug, and an assert that fires before the floor leaves the real path
tested only in release (a landed bug of exactly this shape was repaired).

**Property tests / DST**: seeds replay deterministically — debug from the seed,
never shrug. Permutation pins assert order-independence *including
narrative/output multisets*. Never hand-inject a value a check should produce
(anti-masking). Kernel code stays pure: clock, randomness, filesystem, network
only through injected seams.

**minispec/dorc-verify mechanics** (tooling side, builder-legal): the cheap gate
(`verify:check`) rides the ordinary suite — catalogue coherence, unit contracts,
slug law, hole censuses (the Vocabulary is unit-contract-exempt but NEVER
hole-exempt); Lean-derived badges recompute at the opt-in tier
(`verify:report -- --with-lean`). Badge expectations move ONLY through the
promote ceremony (currently a hand-edit of the catalogue lock whose review is the
git diff; the `dorc-verify promote` generator is owed). Proofs live in
`Minispec/Proofs/` — the tactic-churn zone, structurally unable to touch a
statement; `sorry` there is a loud counted TODO, never silently merged.

## Escalation

Blocked by any of this? Say so, with the artifact in hand (the witness, the
counterexample, the seed, the failing goal, the refusing gate line). Flag
judgment calls upward rather than resolving them locally — especially anything
that would move an invariant between tiers, change a statement or expectation, or
widen what a check accepts. Spec questions route to the human through the
conductor; minispec chafe is a report-and-stop, never a workaround.

## In this repository

Binding local law outranks this skill's generic phrasing: `spike/CLAUDE.md` and
the per-crate `CLAUDE.md` files (notably `core`, `analysis`, `plan`) plus
`minispec/CLAUDE.md` carry the project-specific invariants and always win on
conflict. The verified core spans `core`/`analysis` plus the checker/reference
surfaces (`analysis/src/certify.rs`, `crates/sparing-reference`,
`plan/src/rederive.rs`); the spec corpus is `minispec/` (the Lean model's home —
the research-spike models under `.claude/research/…` are QUARRY, superseded);
the harness/binder home is `spike/verify/`. Deep background: `Research/notes/300`
(the r30 conduct ledger: as-built state, post-mortems, adjudications),
`notes/301` (THE minispec/dorc-verify spec), `plans/302` (the solve-certifier
spec), with the evidence base in
`.claude/research/refinement-types-industrial-cost/`.
