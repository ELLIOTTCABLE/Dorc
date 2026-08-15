# 303 — Certifier phase-1: call-site census, floors, and spec findings

> Tier: LLM-authored recon (an Opus phase-1 agent under map-then-execute, read-only,
> worktree clean, @ `da81f822`, 2026-08-14; banked verbatim-in-substance by the Fable
> conductor). STATUS: the certifier lane is HELD — the human is reworking `plans/302`
> with a sibling conductor; this note is INPUT to that sitting (the census and floor
> table are spec-independent; the findings name where the current `302` text conflicts
> with as-built law). No code was written; no phase-2 exists. If the hold outlasts the
> minting session, phase-2 redispatches fresh against the settled spec + this note.
> Conductor leans on the decision-requests are marked [CONDUCTOR-LEAN] — offered to
> the sitting, not rulings.

## §1 — Call-site census (`census-of-production-solve-callers`)

`analysis::solve::solve` is the ONLY fixpoint entry in the workspace (+SURE; grepped).
FOUR production call-sites, all in `analysis`; no caller outside the crate
(`cli/src/fixpoint.rs:165` and `analysis/tests/cfg.rs:28` import the `Graph` trait
only).

1. `value.rs:241` `analyze` — `ValueEnv = MapL<String, Flat<String>>`, Forward,
   `&Cfg`, all-⊥ init (the "entry ⊤-seed" lives INSIDE the transfer at
   `CfgNodeKind::Entry`, ignoring `incoming`). Consumed by the five passes
   (`site_argv` · `members_pass` · `inline_pass` · `redir_pass` ·
   `provenance_pass`), each taking `(&solution.states, solution.converged)`.
2. `funcenv.rs:557` `analyze`, inside `solve_pruned` — invoked 1 + up to
   `FOLD_ROUNDS_CAP`(=8) times over DIFFERENT `PrunedCfg`s — `EnvStack`, Forward,
   all-⊥ (`EnvStack::Bottom`; ambient-prefix seeding again inside the Entry
   transfer). Consumed by `before`/`binding_before`/`unprovable`/`never_live`/
   `contests`/`LiveDefinitions` + the fold's own `dead_edges(…, converged)`.
3. `effect.rs:1174` `self_reach_holds` — once per Members site — `Reach`, Forward,
   all-⊥, transfer with the site's own gen suppressed. Consumed as
   `sol.converged && sol.states[site].is_pristine()` → `EstablishMembers.self_reached`.
4. `effect.rs:1612` `classify_with_why_diags` (the main reaching-defs) — `Reach`,
   Forward, all-⊥. `trust_reach = reach.converged`; every `reach.states` read sits
   inside `trust_reach &&` arms (+SURE).

Downstream drivers: `cli/src/main.rs:831,845` · `cli/src/world.rs:131,146` ·
`cli/src/survival.rs:989` (value only) · `dorc-loom/src/consumer.rs:1887`.
`plan/src/erase.rs:381`, `cli/src/why.rs:2923` are `#[cfg(test)]` (~SUSPECT the
latter). `coverage`/`sweep`/`hostsim` are instruments/DST.

## §2 — Per-consumer Refused-floor table

All four floors ALREADY EXIST as named, exercised degrade paths (the non-convergence
bargain, `16P` DP-9/find-B); the certifier reuses them, inventing no new posture:

1. **value** — `ValueFlow.converged = false`; the five passes are converged-gated and
   return all-⊤ ⇒ every command `Opaque` ⇒ `MustRun`; `SourceLiteralPlane::converged()`
   goes false, cascading funcenv to ITS floor.
2. **funcenv** — the value already built at `funcenv.rs:544–550`:
   `FuncEnv{ states: all-Top, converged: false, …, folded_edges: ∅ }`; the fold loop
   must BREAK to the floor at the refusing round, not merely stop folding. Yields:
   `before`⇒⊤ · `unprovable` names every role · `never_live`⇒∅ · `contests`⇒∅.
3. **self_reach** — `false` (the existing conservative-refuse answer); the
   `sol.converged && …` line becomes the certification check.
4. **reach** — `trust_reach = false` (one line at `effect.rs:1629`) ⇒ every site
   `SkipClass::MustRun` — the stage-0/⊤ posture, safe under both phases.

## §3 — Proposed implementation shape (phase-2 material; re-cut against the settled spec)

New `analysis/src/certify.rs`. `EdgeWitness<L>{ Boundary{node,init,state} |
Edge{from,to,transferred,state} }`; `SolveCertification<L>{ Certified{checks} |
Refused{witnesses, shown, total} }` — non-empty by private mint, `WITNESS_CAP=8`,
no bool accessor. `certify_solution(graph, direction, init, transfer, solution)` +
`solve_certified(...)` wrapper (`&transfer` reborrow; solver signature untouched).
Walk: canonical (node × successor index), transfer once per node, E joins, NO early
exit; mirror the solver's release-mode out-of-range `continue` verbatim. Cost ≈ one
solver sweep (value 1× · funcenv ≤9× · reach 1× · self-reach 1×/Members site) —
negligible under perf-doctrine.

Aid plane: ONE code `solve-certification-refused` + reason enum
`SolveStage{ValueFlow, FunctionEnvironment, ReachingDefs, SelfReach}`
(`28L:rul-reason-enums-not-sibling-codes`); spanless mint; prose explicitly empty;
fixture-routed (`289:rul-worldless-route-honest-trigger` — a Refused has no honest
book trigger; joins the fixture-driven population beside the records-8 pending
decision). Narrative: `CollapseKind::SolveCertificationRefused`, `SpeechAct::Derived`,
SCALAR operands only (see finding below). Mint seats follow the
`funcenv::unresolvable_loads` precedent: kernels record refusal as data; cli drivers
mint via `report_at`; `effect.rs` mints in place. Raw-`solve` fence: demote to
`pub(crate)` (zero-cost workspace compile fence) + an in-crate lexical walk with a
non-empty assertion. Each floor becomes a NAMED function (what makes §5.7 testable
without violating §5.6).

Test plan: one-to-one against `302` §5 (perturbation×3 with exact witnesses ·
boundary violations entry+interior+Must-dual over `Must<Flat<u8>>` · oscillation
localization + landed-at-cap certifies · duality with no orientation argument ·
determinism repeat + permutation · a lexical no-outcome-construction gate ·
four floor-and-narrate consumer tests + the loom defining case).

REPRICE: ≈800–900 lines across ~10 files + 1 loom case + lock regen — ~16× the folk
"~50 lines"; 8–13 agent-hours (~2× folk time). Dominated by aid-plane registration
+ the test battery; the checker body itself really is ~60 lines.

## §4 — Spec findings (flagged, unresolved; the sitting's input)

- **`fnd-solver-takes-no-seed-at-all`** — `solve` has NO init parameter (all-⊥
  unconditionally), so `302` §1's boundary clause is trivially true at every
  production site, in both orientations (`Must::bottom()` IS the dual's ⊥); the
  non-vacuity rationale describes a solver that seeds, and `lattice.rs` records
  must-boundary seeding as deliberately unbuilt. No coverage is actually lost (both
  seeding callers seed INSIDE the Entry transfer, checked by the per-edge family).
  Options: (a) `certify_solution` takes `init: &[L]`, wrapper passes all-⊥ — executes
  as the trivially-true-still-executed case, ready the day seeding lands; (b) derive
  ⊥ internally; (c) add seeding to `solve` (scope creep, declined by lattice.rs).
  [CONDUCTOR-LEAN: (a).]
- **`fnd-witness-operands-cannot-enter-narrative`** — `302` §4's "record carries the
  witness operands" conflicts with `aid/CLAUDE.md:operands-are-pure-and-capped`
  (scalars/interned handles only; `ProvId` forbidden — and `Reach::Top(ProvId)`
  carries exactly that). Proposed: narrative carries scalars only (stage, indices,
  shown/total, advisory converged/rounds); full-value witnesses stay in the
  `SolveCertification` and never cross into aid. [CONDUCTOR-LEAN: yes — aid law wins;
  `302` §2/§4 text amends.]
- **`fnd-certification-must-be-generic-over-lattice`** — by-value operands force
  `SolveCertification<L>`; trivial spec-text correction.
- **`fnd-never-live-is-the-grant-shifting-consumer`** — the sharpest census fact:
  `funcenv::never_live` subtracts EXACTLY and shifts winners
  (`28P:adj-never-live-exactness-accepted`), gated only on `env.converged()`. The
  funcenv floor MUST set `converged=false` and empty `folded_edges`, else a refused
  solution still GRANTS (subtraction/fold against refused states).
- **`fnd-self-reach-has-no-diagnostic-channel`** — call-site 3 sits under a `Fn`
  closure; refusal can be taken but not narrated in place ⇒ hoist per-site answers
  into a pre-pass (priced; reshapes a `too_many_lines` function).
- **`fnd-no-production-must-or-backward-caller`** — zero production `Must` or
  `Backward` solves exist; the duality claims are exercised on synthetic domains
  only. Not a conflict; a coverage-honesty note.
- **`fnd-reach-equality-excludes-its-cause`** — `Reach` hand-writes `PartialEq`
  excluding the `Top(ProvId)` cause (necessarily — cause-sensitive Eq never
  terminates); semantic under the intended reading but NOT covered by the facade's
  structural-Eq seats, and out of translation scope. The certifier trusts
  `Reach::eq` with nothing else pinning it ⇒ CANDIDATE FOR THE KANI LANE list.
- **`fnd-refusal-has-no-honest-trigger`** — the defining case routes through
  `aid::fixture::canonical_payloads` (sanctioned; fixture-only-while-no-honest-route).
- **`fnd-mirror-the-out-of-range-skip`** — copy `solve`'s debug-assert +
  release-`continue` guard verbatim; mirror the solver, don't validate the graph.
- **`fnd-permutation-pin-is-set-not-sequence`** — canonical order is (node ×
  successor index), so edge-insertion permutation legitimately reorders witnesses;
  the §5.5 permutation pin must compare verdict + witness SET (repeat-run
  byte-identity holds unconditionally). [CONDUCTOR-LEAN: set.]
- **`fnd-converged-debug-assert-is-now-the-wrong-question`** — `effect.rs:1615`
  asserts `reach.converged`; under certification the honest dev assertion becomes
  certified (landed-at-cap is legitimate).
- §5.3's landed-at-cap fixture hand-writes the ADVISORY FLAG only (states from a
  real solve; checker runs for real) — the alternative, an injectable solver cap, is
  a verified-core signature change the agent deliberately did not propose.

## §5 — Phase-2 posture

Phase-2 proposal accepted-in-shape by the conductor but NOT greenlit: it re-cuts
against the settled `302` after the human's sitting, consuming this note. The
phase-1 agent offered a phase-2 comment budget of ≤45 non-doc comment lines
(counting command: `rg -n '^\s*//' <changed files>`), public-item doc-comments
billed separately per `spike/CLAUDE.md` code style.
