# 302 — The solve-certifier: mechanical spec

> Tier: LLM-authored mechanical spec (Fable conductor, 2026-08-14; the `notes/300` §2
> staffing split routes this spec to the conductor and the implementation to an Opus
> builder). Subordinate to root docs, `spike/CLAUDE.md`, and the crate CLAUDE.mds.
> Grades: [SPEC] binding on the build · [BUILDER] confirmed/priced during the build ·
> [HUMAN?] a point the human may overrule at fold. Names STRAWMAN per
> `rul-strawman-formats-no-compat`. Naming note: "solve-certifier" always — bare
> "certifier" collides with round-16's per-leaf inertness certifier (`17O`).
>
> Standing obligations honored herein, cited once: the five crosscheck brief
> obligations (turn07 `adj-certifier-spike-brief-obligations`) · the fresh-ack
> pedigree note (`28T` §1 w1-solve-certifier — this build rests on the 28T checker-triad
> [TYPED] ack, never on `plans/021`/`055` pedigree) · `28R:fnd-pessimistic-pass-shape`.

## §0 — What it is, and the exact guarantee

A per-answer, post-fixpoint validator at the solve seam: after every production
`solve()` call, one flat pass re-checks that the returned states actually ARE a
post-fixpoint of the transfer system the solver was given. The solver stays untrusted;
its every answer is checked, not believed.

[SPEC] The guarantee, stated with its hypothesis (the domino-lemma obligation,
settled): **given the transfer function is monotone** — already `solve()`'s
documented caller-upheld precondition, so this leans on nothing new — a `Certified`
answer is a valid post-fixpoint of the seeded system, and therefore over-approximates
every abstract path from the seeded boundary. Concrete (γ-soundness) coverage is
EXPLICITLY out of scope: the certifier shares the transfer model with the solver and
is not foreign ground truth (`28T` §4) — it catches solver bugs (worklist, ordering,
convergence-detection, state-management), never model bugs. If the caller's transfer
is in fact non-monotone, `Certified` claims only the post-fixpoint inequalities
themselves; the certifier does not detect non-monotonicity.

## §1 — The checked property

[SPEC] Two families of inequalities, checked over exactly the inputs the solver
consumed (same graph, same `Direction`-oriented edge view, same transfer closure, same
initial/seed values) plus its output states:

1. **boundary, all nodes**: `∀ n: init[n] ⊑ state[n]`. Checking every node — not only
   entries — is deliberate: it is strictly stronger, it is what makes the check
   NON-VACUOUS for must-oriented boundary seeding (entry nodes have no in-edges, so a
   pure per-edge walk never sees the seed; this clause does), and for bottom-seeded
   non-entry nodes it degenerates to a trivially-true, still-executed case. This
   discharges the must-boundary-vacuity obligation structurally.
2. **per-edge**: `∀ solver-oriented edges (v → w): transfer(v, state[v]) ⊑ state[w]`.

`⊑` is the `Lattice`-derived order (`x ⊑ y ⟺ x ⊔ y = y`), which rests on semantic
`Eq` — the same precondition `solve()` already imposes, and precisely the invariant
the facade lane seats and the Kani lane pins (`MapL` canonical-form: structural Eq ==
semantic Eq). The certifier deliberately trusts the lattice laws; the Kani tier is
what earns that trust (`verified-core-discipline`: tiers compose).

[SPEC] **`Must<L>` duality costs zero branches**: the dual order is carried by the
`Must` lattice instance itself, so the same two inequality families cover both
orientations. The certifier is generic over `L: Lattice` and NEVER inspects
orientation, phase, or domain semantics — one checker, no orientation parameter
(`inv-superposition` friendly).

[SPEC] **Walk order is canonical and worklist-free**: a flat pass in (node index ×
successor index) order. Consequence: the witness set is DETERMINISTIC — turn07's
"first failing edge is traversal-order-dependent" hazard is dissolved by construction,
not mitigated. There is no early-exit on first failure (see §2).

[SPEC] **The `converged` flag is advisory, the states are what certify**: a cap-tripped
(`converged: false`) answer that happens to satisfy §1 is legitimately `Certified` —
the solver stopped without noticing it had landed; certification rescues it. A
converged-claiming answer with a failing edge is `Refused` — the bug class this whole
instrument exists to catch.

## §2 — Outcome type and witness discipline

[SPEC] Closed outcome, never a bool, no `is_ok()`-shaped accessor:

- `Certified` — carries nothing beyond (optionally) counts for the narrative plane.
- `Refused(witnesses)` — a NON-EMPTY, k-capped, canonically-ordered list of
  `EdgeWitness`es. An `EdgeWitness` names the failing clause: either
  `Boundary { node, init, state }` or `Edge { from, to, transferred, state }` —
  operands included by value (they are lattice values; the narrative plane needs
  them). The k-cap is a disclosed selection: the refusal records "first k of N
  failing checks in canonical order" with both k and N — deterministic + disclosed,
  matching the `28T` w1-kani narrative-pin discipline. On a cap-tripped solve, the
  full witness set IS the oscillation localization — collecting all (to k) rather
  than first-only is what makes the refusal diagnostic.
- [SPEC] **witness ≠ root cause**, priced into every rendering: a witness is evidence
  of non-certifiability at a named check, never a cause verdict. No narrative
  fragment may render a witness as "caused by"; the honest verb is "failed its
  post-fixpoint check at". (Turn07 obligation 3.)

[SPEC] **The certifier itself is pessimistic-shaped** (`28R:fnd-pessimistic-pass-shape`):
it has NO budget cap of its own at v0 (one flat pass, trivially cheap next to the
fixpoint it checks — O(E) transfer+join evaluations); if any future pressure ever caps
it, the unexamined region refuses, never certifies. Partial certification of the
SOLUTION exists (cap-tripped solve, §1); partial execution of the CERTIFIER does not.

## §3 — The seam and the consumer obligation

[SPEC] One seat: `certify_solution(...)` beside the solver
(`spike/crates/analysis/src/certify.rs` or in-module with `solve.rs` — [BUILDER]
taste, crate-local homing either way), plus a `solve_certified(...)` wrapper that
returns `(Solution<L>, SolveCertification)` so no call-site can obtain an answer
while forgetting to look at its certification.

[SPEC] Every PRODUCTION caller of `solve()` routes through the certified seam. The
builder enumerates the production call-sites in the build report, and for EACH
records its **Refused-floor**: the degraded value that licenses NOTHING for that
consumer — the ⊤/walls/stage-0 posture in that consumer's own domain terms. This
explicit mapping is the resolution of the degrade-to-⊤-is-unspellable-generically
obligation: `Lattice` has no `top()` and `Powerset`/`MapL` are deliberately not
`BoundedLattice`, so the FLOOR BELONGS TO THE CONSUMER, and the closed
`Refused` shape is what forces each consumer to supply one. No generic fallback
exists, on purpose. Raw `solve()` stays available to tests/DST only; [BUILDER] a
cheap lexical or review fence that production code does not call it bare.

[SPEC] A `Refused` is handled degrade-and-continue, never abort: the consumer takes
its floor (maximally conservative analysis ⇒ everything guards/runs; fail-toward-run
preserved), the plan still emits, and the failure lands tier-2 (pre-network, at plan
construction). [HUMAN?] It surfaces LOUDLY — a catalog `DiagCode` (structured, per
`one-catalog-no-legacy`; prose per `error-authorship-tier`: builder mints the code
with explicitly-empty prose) — because a Refused in the field means OUR solver has a
bug and the warning is how it routes home. kWARN-rich era: never silent.

## §4 — Two-plane integration

[SPEC] The certifier is license-plane-adjacent pure computation; it reads no aid
state. The DEGRADE act at each consumer is a safety-narrowing and therefore mints the
narrative record (`collapse-mints-narrative`), carrying the witness operands, the
disclosed k-cap facts, and the advisory `converged`/`rounds` context. The record is
decision-inert (sealed, aid-plane); the license plane consumes only the closed
outcome. Per `28Q` §7: once 28Q stages land, a certification `Refused` on the new
frame/closure shapes is a FINDING, never churn — that posture starts now.

## §5 — Test obligations (all in the default suite; anti-masking honored)

[SPEC, all]:
1. **fault-injection, solver-side**: perturb a correct solution (one state raised /
   lowered / swapped) ⇒ `Refused` with the exact expected witness(es), canonical
   order verified.
2. **boundary non-vacuity**: a violated seed (init ⋢ state at an entry AND at a
   non-entry seeded node), both orientations — the `Must`-wrapped case included, so
   the dual-order boundary check is demonstrably exercised (turn07 obligation 2
   witnessed by a test, not an argument).
3. **cap-trip localization**: a deliberately oscillating system under a round-cap ⇒
   `Refused` whose witness set covers the oscillating edge(s); plus the
   landed-on-fixpoint-at-cap case ⇒ `Certified` despite `converged: false`.
4. **duality**: one system certified under `L` and its `Must<L>` dual, one checker,
   no orientation flag anywhere in the call.
5. **determinism**: witness lists byte-identical across repeated runs and (DST)
   across input-permutation where the graph is permutation-invariant.
6. **anti-masking**: no test hand-injects a certification outcome; outcomes come from
   the real checker over real solutions (`anti-masking-tests`).
7. **narrative**: the consumer-side degrade mints the record; the record carries
   operands; goldens for any surfaced diagnostic follow `render-form-unwelded`.

## §6 — Explicitly not (scope fences)

- Not foreign ground truth; catches solver bugs only, never transfer-model bugs.
- No quality/precision claim: an everywhere-⊤ answer certifies.
- No per-phase-product results-checkers beyond this seat; no whole-chain elision
  re-walk; no runtime aid-plane checker (`28T` §4 checker-expansion rejection stands;
  the admission tests T1/T2 bind any future proposal).
- No caching of certifications, no persistence (rec-5 posture; recompute per run).
- The sparing reference re-derivation is a SEPARATE instrument (its own lane).

## §7 — Build notes

- [SPEC] **Reprice before building** (turn07 obligation 4): the folk "~50 lines /
  ~1 agent-day" estimate predates the closed-outcome shape, witness discipline,
  consumer enumeration + floors, narrative minting, and §5's battery. The builder
  re-estimates in its plan-first checkpoint and reports the delta; the checkpoint is
  map-then-execute (`27U` §4) — proposal to the conductor BEFORE the build half.
- Placement: `analysis` crate (crate-local homing). The generic checker takes the
  same type parameters as `solve` (`G: Graph`, `L: Lattice`, the transfer closure);
  monomorphic-boundary concerns are Kani-lane territory, not this seat's.
- Kani/property follow-ups belong to lane-kani (the checker's own inequality-walk is
  a candidate harness target once landed); minispec/`dorc-verify` badge wiring for
  this instrument is deferred to the enrichment era — nothing here blocks on it.
