# 302 — The solve-certifier: rationale, mechanics, build spec

> Tier: LLM-authored mechanical spec (Fable conductor, from the 2026-08-14/15 certifier
> design sittings with the human; census input `notes/303`). Subordinate to root docs,
> `spike/CLAUDE.md`, and the crate CLAUDE.mds; `notes/300` §2 carries staffing (spec →
> conductor; implementation → an Opus builder, map-then-execute). Grades: [SPEC] binding
> on the build · [BUILDER] confirmed/priced during the build · [HUMAN?] the human may
> overrule at fold · [TYPED] the human typed it. Names STRAWMAN per
> `rul-strawman-formats-no-compat`; the vocabulary set herein is human-ruled
> (2026-08-15). Naming: "solve-certifier" always — bare "certifier" collides with
> round-16's per-leaf inertness certifier (`17O`).
>
> Standing obligations honored herein, cited once: the five crosscheck brief obligations
> (turn07 `adj-certifier-spike-brief-obligations`) · the fresh-ack pedigree note (this
> build rests on the `28T` checker-triad [TYPED] ack, never on `plans/021`/`055`
> pedigree) · `28R:fnd-pessimistic-pass-shape` · the aid-leads direction ([TYPED]: aid is
> part of the correct output — the deterministic source → probe-results → whylog mapping
> includes consistency-failure behavior, and its aid quality is a correctness property).

## §0 — What it is, and the exact guarantee

A per-answer, post-fixpoint validator at the solve seam: after every production
`solve()` call, one flat pass re-checks that the returned states actually ARE a
post-fixpoint of the transfer system the solver was given. The solver stays untrusted;
its every answer is checked, not believed. The admission argument is the find/check
asymmetry (`28T` §4 T1): the worklist iteration is where an implementation bug — a
dropped re-queue, a mis-scheduled update, a premature convergence claim — yields a
silently STALE answer that goldens can bless right past, and downstream a stale answer
is a wrong elision; the finished answer, by contrast, has a purely local
characterization checkable in one sweep.

- **rul-certifier-value-is-stupidity** [TYPED substance] — the instrument is admissible
  because it is strictly simpler than what it checks: small (the checker body is ~60
  lines; `303` §3), total, panic-free, single-sweep, reviewable in one sitting.
  Minimalness, robustness, correctness, and reviewability are paramount. Every pressure
  toward cleverness resolves by routing it elsewhere (aid → the engine, §5) or refusing
  it (recovery, §9).
- [SPEC] The guarantee, stated with its hypothesis (the domino-lemma obligation,
  settled): **given the transfer function is monotone** — already `solve()`'s
  documented caller-upheld precondition, so this leans on nothing new — a `Consistent`
  answer is a valid post-fixpoint of the system and therefore over-approximates every
  abstract path from the boundary. Concrete (γ-soundness) coverage is EXPLICITLY out of
  scope: the certifier shares the transfer model with the solver and is not foreign
  ground truth — it catches solver bugs (worklist, ordering, convergence-detection,
  state-management), never model bugs. Non-monotone transfers are not detected; a
  `Consistent` answer then claims only the inequalities themselves. [Rider, ACKED: if
  the domino lemma is ever authored as a minispec statement, the monotonicity
  hypothesis is settled FIRST, in the statement.]
- Trust chain (tiers compose): the checker's `⊑` is join + structural equality, so its
  soundness leans on the facade canonicality seats and the Kani-pinned lattice laws —
  pinned independently, at compile time, exhaustively at bounds. Known shared-substrate
  limit, accepted with eyes open: a broken `Eq`/canonical form can blind this
  instrument. Census-found instance routed to the Kani lane: `Reach`'s hand-written
  `PartialEq` excludes its `Top(ProvId)` cause and nothing else pins it
  (`303:fnd-reach-equality-excludes-its-cause`).

## §1 — The checked property

[SPEC] Two families of inequalities, checked over exactly the inputs the solver
consumed (same graph, same `Direction`-oriented edge view, same transfer closure) plus
its output states:

1. **boundary, all nodes**: `∀ n: init[n] ⊑ state[n]`. `certify_solution` takes
   `init: &[L]` even though `solve` today seeds nothing (all-⊥ unconditionally, both
   production seeders seeding INSIDE their Entry transfers, which the per-edge family
   checks — `303:fnd-solver-takes-no-seed-at-all`); the wrapper passes all-⊥, the
   clause executes as the trivially-true-still-executed case, and the day the solver
   grows real seeding the check is already live. Checking every node — not only
   entries — is deliberate: entry nodes have no in-edges, so a pure per-edge walk never
   sees a seed; this clause does, in both orientations (`Must::bottom()` is the dual's
   ⊥). This discharges the must-boundary-vacuity obligation structurally.
2. **per-edge**: `∀ solver-oriented edges (v → w): transfer(v, state[v]) ⊑ state[w]`.

`⊑` is the `Lattice`-derived order (`x ⊑ y ⟺ x ⊔ y = y`), resting on semantic `Eq` —
the same precondition `solve()` already imposes, and precisely what the facade seats
and the Kani lane pin.

[SPEC] **`Must<L>` duality costs zero branches**: the dual order is carried by the
`Must` lattice instance itself, so the same two inequality families cover both
orientations. The checker is generic over `L: Lattice` (`SolveConsistency<L>` —
inconsistency items hold values by value), never inspects orientation, phase, or
domain semantics — one checker, no orientation parameter. Coverage honesty
(`303:fnd-no-production-must-or-backward-caller`): no production `Must` or `Backward`
caller exists today; the duality is exercised on synthetic domains in §6.

[SPEC] **Walk order is canonical and worklist-free**: a flat pass in (node index ×
successor index) order, transfer evaluated once per node, no early exit. The solver's
release-mode out-of-range guard is mirrored verbatim — mirror the solver, never
validate the graph (`303:fnd-mirror-the-out-of-range-skip`).

[SPEC] **The `converged` flag is advisory; the states are what certify.** A
cap-tripped (`converged: false`) answer that satisfies §1 is legitimately
`Consistent` — and under monotone ascent from ⊥ it is exactly the least fixpoint (an
ascent state is always ⊑ lfp; a post-fixpoint that is ⊑ lfp equals it), so nothing is
lost: the solver merely stopped without noticing it had landed. The advisory mismatch
mints a curiosity-tier narrative note, and `effect.rs:1615`'s `reach.converged`
debug-assert re-cuts to ask certification instead
(`303:fnd-converged-debug-assert-is-now-the-wrong-question`). The reverse mismatch —
`converged: true` with a failing check — is the defect class this instrument exists to
catch, and is simply `Inconsistent`. A cap-trip whose states do NOT certify is the
oscillation case: the failing set's summaries (§2) name the un-stabilized region, for
the narrative plane only.

## §2 — Outcome type and its evidence

[SPEC] Closed outcome, never a bool, no `is_ok()`-shaped accessor, non-empty by
private mint:

- `Consistent { checks }` — counts for the narrative plane, nothing more.
- `Inconsistent { failing, inconsistencies, shown, total, first_break_edges,
  unstable_components }`:
  - **the complete failing-check INDEX set** (`failing`: boundary nodes; edge
    (from, to) pairs) is always carried whole — scalars, cheap, canonical, and the
    substrate every downstream computation reads;
  - **by-value items** — `Inconsistency<L>{ Boundary{node, init, state} |
    Edge{from, to, transferred, state} }` — are the first `INCONSISTENCY_CAP`(=8) in
    canonical order, with `shown`/`total` disclosed (deterministic + disclosed k-cap,
    the house discipline);
  - **summaries**, computed from the COMPLETE index set before any cap
    (**rul-first-break-and-unstable-components**): the FIRST-BREAK edges — failing
    checks whose source node is itself fully clean (every incoming edge passes,
    boundary holds) — name where consistency first breaks along the flow; where none
    exist (every node in a cycle failing — the oscillation shape), the UNSTABLE
    COMPONENTS (strongly-connected components containing failing checks) name the
    un-stabilized region. Summaries are narrative input only; they never scope the
    demotion (§3).
- [SPEC] **an inconsistency is not a cause**, priced into every rendering: an
  `Inconsistency` is evidence that a named check failed, and the first-break edges are
  the earliest OBSERVABLE inconsistency — the actual cause is a code defect no runtime
  artifact can name. The honest verbs are "failed its post-fixpoint check at" and
  "first breaks at"; never "caused by".

[SPEC] **The certifier itself degrades toward `Inconsistent` under any interruption**
(`28R:fnd-pessimistic-pass-shape` applied to the checker): it has no budget cap of its
own at v0 (one flat pass, O(E) transfer+join evaluations — `303` §3 prices the
production total at roughly one extra solver sweep per seat, noise under
perf-doctrine); if any future pressure caps it, the unexamined region reports failing,
never passing. Partial execution of the certifier never yields `Consistent` for
anything unexamined.

[SPEC] **Certifiability and interruption-safety are different properties** — never
conflate the instruments: this checker detects INCONSISTENCY (implementation bugs);
safety-under-interruption of a capped SOLVE comes from pass shape (a pessimistic pass
starts at walls and proves survival, so every stopping point is safe-side by
construction, no certificate involved — and its mid-descent states may honestly fail
edge checks while being perfectly safe). Cappable passes owe the pessimistic shape as
standing doctrine; the certifier licenses no partial result in either shape.

## §3 — The seam, the whole-window demotion, and the consumer floors

[SPEC] One seat: `certify_solution(graph, direction, init, transfer, solution)`
beside the solver, plus a `solve_certified(...)` wrapper returning
`(Solution<L>, SolveConsistency<L>)` so no call-site can obtain an answer while
forgetting its certification. Raw `solve` demotes to `pub(crate)` with an in-crate
lexical fence (non-empty assertion) so production code cannot call it bare; tests and
DST may (`303` §3).

[SPEC, TYPED substance] **rul-whole-window-demotion** — an `Inconsistent` verdict
demotes the ENTIRE product of that solve's analysis window: every consumer of that
answer takes its floor, every license fed by it lapses — general, approaching global,
never partial. No per-node trust, no region carve, no cone; the summaries exist to
explain, never to scope. (The one recorded direction-not-taken and its reasons: §9.)

[SPEC] "Degrade to ⊤" is unspellable generically — `Lattice` has no `top()`, and
`Powerset`/`MapL` are deliberately not `BoundedLattice` (`must-lattice-by-type`) — so
**the floor belongs to the consumer**, and the closed `Inconsistent` shape is what
forces each consumer to supply one. All four production floors already exist as the
named, exercised non-convergence degrade paths (the `16P` DP-9 bargain); the certifier
reuses them, inventing no new posture (`303` §1/§2, binding here):

1. **value** (`value.rs:241`) — `converged=false` ⇒ the five converged-gated passes
   answer all-⊤ ⇒ every command `Opaque` ⇒ `MustRun`; `SourceLiteralPlane::converged()`
   goes false, cascading funcenv to its own floor (dependency-ordered flooring falls
   out of the existing gates).
2. **funcenv** (`funcenv.rs:557`, ≤9 solves across the fold rounds) — the
   `funcenv.rs:544–550` floor value: all-Top states, `converged=false`, **and
   `folded_edges=∅`, with the fold loop BREAKING to the floor at the failing round**.
   This is a hard [SPEC] rider, not an implementation detail: `never_live` subtracts
   exactly and SHIFTS WINNERS (`28P:adj-never-live-exactness-accepted`), so an
   inconsistent solution that still feeds subtraction or folding GRANTS on unchecked
   states — the one place a sloppy floor converts a consistency failure into a license
   (`303:fnd-never-live-is-the-grant-shifting-consumer`).
3. **self_reach** (`effect.rs:1174`, per Members site) — the answer is `false` (the
   existing conservative refuse); the `sol.converged && …` gate becomes the
   certification gate. Its closure seat has no diagnostic channel, so per-site answers
   hoist into a pre-pass ([BUILDER], priced; reshapes a `too_many_lines` function —
   `303:fnd-self-reach-has-no-diagnostic-channel`).
4. **reach** (`effect.rs:1612`) — `trust_reach=false` ⇒ every site `SkipClass::MustRun`
   — the stage-0/⊤ posture, safe under both phases.

Each floor is a NAMED function, so §6's consumer tests can exercise it without
violating anti-masking. Guards everywhere stay fail-safe under any floor by
construction: `( check ) || original` falls through to the authored bytes.

- **pin-blast-radius-escalation** [HUMAN?] — per-inconsistent-solve flooring is what
  this spec builds (consistency failures are engine defects, expected vanishingly
  rare; uniform per-solve demotion is predictable, honest, and the value floor already
  cascades funcenv along the real dependency). Whether any single consistency failure
  should further escalate to a whole-plan stage-0 posture (one detected inconsistency
  taints the shared machinery) is an open human call; if ruled, it lands as a thin
  policy above the per-solve floors, not a reshape.

## §4 — Posture by seat (the fail-fast ↔ best-effort calibration)

[SPEC] The certifier fires wherever `solve` fires, and the failure posture is the
WINDOW's, not the certifier's. [BUILDER] classifies every driver from `303` §1's list
(cli `main`/`world`/`survival`, the loom consumer) into these rows and records the
mapping in the build report:

- **Pre-network solves** (analysis before any probe ships; the user present, waiting):
  tier-2 fail-fast — loud, on human timescales. The product is still a plan (the
  honest floor is a valid plan, never worse than no-Dorc) with the diagnostic
  front-and-center.
- **Post-probe solves** (folding probe results; mid-window, possibly unattended):
  best-effort — batch the demotion, keep extracting UNRELATED value, surface
  everything at the single approval moment. No new interaction moments.
- **Apply-time**: vacuous as-built (the plan is frozen at consent; no solve runs
  during apply) — stated so the posture is pre-committed: any future recompute whose
  consistency failure touches in-flight mutation authority is the second fail-fast
  regime, the `rul-integrity-failure-withholds-mutation` cousin — not
  world-uncertainty (⇒ run), but lost trust in our own computation (⇒ withhold further
  mutation).

## §5 — Two-plane integration and the aid surface (aid leads)

[SPEC] The certifier is license-plane-adjacent pure computation; it reads no aid
state, and the license plane consumes only the closed outcome.

- **rul-certifier-never-reports** — the checker's entire output is the verdict and its
  data (indices, by-value items, summaries). It renders nothing, authors no prose, and
  never re-enters the engine. Its aid contract, whole: "rerun and do extra work — you
  are now a self-report engine."
- **Narrative records carry scalars only** — the DEGRADE act at each consumer is the
  safety-narrowing and mints the record (`collapse-mints-narrative`):
  `CollapseKind::SolverConsistencyFailure`, `SpeechAct::Derived`, operands per
  `aid/CLAUDE.md:operands-are-pure-and-capped` — stage, indices, shown/total, advisory
  `converged`/`rounds` — never lattice values, never `ProvId`-bearing types
  (`303:fnd-witness-operands-cannot-enter-narrative`); full-value items live in the
  in-memory `SolveConsistency` and reach people through pull surfaces,
  display-rendered at the edge. ONE catalog code, `solver-consistency-failure`, with a
  `SolvePass{ValueFlow, FunctionEnvironment, ReachingDefs, SelfReach}` reason enum
  (`28L:rul-reason-enums-not-sibling-codes`); spanless; prose explicitly empty at mint
  (`error-authorship-tier`), authored through the standard pipeline; the defining loom
  case is fixture-routed (`303:fnd-refusal-has-no-honest-trigger` —
  `289:rul-worldless-route-honest-trigger`). Mint seats follow the
  `funcenv::unresolvable_loads` precedent: kernels record the failure as data, cli
  drivers mint via `report_at`, `effect.rs` mints in place.
- **rul-rerun-is-the-self-report-engine** — on a consistency failure, the ENGINE
  re-runs the identical solve with instrumentation on. `inv-determinism` is what makes
  this sound: a pure kernel replays the IDENTICAL trajectory, defect included, now
  narrated — a per-update replay log (node, old value, new value, causing edge, round)
  sliced to the failing region ([BUILDER] picks the cheapest honest slice; the full
  replay stays available at maximum verbosity). Against the replay, a first-break edge
  becomes a genuine observable-level account ("this node last moved in round 3 via
  that edge; the wall's erasing transfer landed in round 5; no re-queue followed").
  The replay
  is evidence of what happened, never a trusted computation — the checker remains the
  judge — and it is pull-tier (`rul-chain-is-pull-only`): the push surface carries the
  compact record above. In-lane and required, priced at the checkpoint ([BUILDER]); it
  may land as the lane's second commit-series but never falls out of the lane.
- **Admin-facing honesty**, rendered plainly: this is OUR defect, not the book's; the
  plan is safe but poorer (N sites demoted); and — the kernel being pure — the whylog
  IS a deterministic reproducer bundle for the report home.

## §6 — Test obligations (default suite; anti-masking honored throughout)

[SPEC, all]:
1. **fault-injection, solver-side**: perturb a correct solution (one state raised /
   lowered / swapped) ⇒ `Inconsistent` with the exact expected items, canonical order
   verified.
2. **boundary non-vacuity**: violated seeds at an entry AND at a non-entry node, both
   orientations, the `Must<Flat<u8>>`-style dual included — the dual-order boundary
   check demonstrably exercised, witnessed by a test rather than an argument.
3. **cap-trip both ways**: a deliberately oscillating system under a round-cap ⇒
   `Inconsistent` whose complete failing set covers the oscillating edges and whose
   unstable components name the region; and the landed-on-fixpoint-at-cap case ⇒
   `Consistent` despite `converged: false` (fixture hand-writes the ADVISORY FLAG
   only; states come from a real solve — `303` §4).
4. **first-break summaries**: a single-defect fixture where exactly one edge fails
   ⇒ the first-break set names it; a two-stage staleness where it excludes the
   downstream casualty.
5. **duality**: one system certified under `L` and under `Must<L>`, one checker, no
   orientation flag anywhere in the call.
6. **determinism**: repeat-run byte-identity of the whole outcome; under
   edge-insertion permutation, verdict + failing SET + summaries compare equal
   (canonical order legitimately reorders the item list —
   `303:fnd-permutation-pin-is-set-not-sequence`).
7. **anti-masking**: no test hand-injects a certification outcome; outcomes come from
   the real checker over real solutions; a lexical gate pins that no production code
   constructs the outcome type by hand.
8. **floor-and-narrate, ×4 consumers**: each named floor function reached via a real
   `Inconsistent`, the funcenv fold demonstrably BREAKING at the failing round with
   `folded_edges=∅`, the record minted with scalar operands, and the value→funcenv
   cascade observed.
9. **replay pins**: the instrumented replay reproduces the identical trajectory
   (state-sequence equality against the original solve), the slice covers the failing
   region, and the replay surface honors the disclosed cap.

## §7 — Explicitly not (scope fences)

- Not foreign ground truth; catches solver bugs only, never transfer-model bugs.
- No quality/precision claim: an everywhere-⊤ answer certifies.
- No per-phase-product results-checkers beyond this seat; no whole-chain elision
  re-walk; no runtime aid-plane checker (`28T` §4's admission tests T1/T2 bind any
  future proposal).
- No caching of certifications, no persistence (rec-5 posture; recompute per run).
- The sparing reference re-derivation is a SEPARATE instrument (its own lane).

## §8 — Build notes

- [SPEC] **The reprice is real and stands** (turn07 obligation 4, discharged with
  numbers): ≈800–900 lines across ~10 files + one loom case + lock regen, 8–13
  agent-hours — ~16× the folk "~50 lines", dominated by aid-plane registration and
  §6's battery; the checker body itself is ~60 lines (`303` §3). §5's replay
  instrumentation prices separately at the checkpoint. Map-then-execute stands:
  phase-2 dispatches fresh against THIS spec + `notes/303`; proposal to the conductor
  before the build half.
- Placement: `analysis` crate, `certify.rs` beside `solve.rs` (crate-local homing);
  generic over the same parameters as `solve`; monomorphic-boundary concerns are
  Kani-lane territory, not this seat's.
- Comment budget: ≤45 non-doc comment lines across changed files (counting command:
  `rg -n '^\s*//' <changed files>`); public-item doc-comments billed separately per
  `spike/CLAUDE.md` code style.
- Kani/property follow-ups route to lane-kani (the checker's own inequality walk, and
  `Reach::eq`'s cause-exclusion, join its target list once landed); minispec/
  `dorc-verify` badge wiring is deferred to the enrichment era — nothing here blocks
  on it.

## §9 — Why there is no recovery

The certifier deliberately ships with no value-recovery mode, and none is planned: an
`Inconsistent` verdict is a general — approaching global — demotion of everything that
fed the certifier's analysis window, never a piecemeal elision-recovery mechanism to
buy value back from. The economics are against it twice over: what recovery would buy
back is priority-three value (unnecessary execution avoided), while every recovery
mechanism's own failure lands in priority-one territory (silent under-execution) — and
those failures are CORRELATED with the trigger, not independent of it: the defect
classes that fire consistency failures in practice (broken canonical forms and `Eq`,
non-monotone transfers — increasingly dominant as ordinary QA extinguishes the benign
scheduling class) are the same classes that void a recovery mechanism's own premises,
so a "screened" recovery could only ever license on absence-of-detected-wrongness — a
silence-shaped license, which this project categorically refuses
(`silence-licenses-nothing`). Minimalness, robustness, correctness, and reviewability
are paramount here, and the checker's admissibility rests on being strictly simpler
than what it checks — every recovery design erodes exactly that. For the same reasons
the aid machinery stays in the engine: the certifier says "rerun and do extra work —
you're now a self-report engine"; the certifier does not do the reporting.
