# 230 — best-effort: which collapsed booleans must become gradients (r23 seed)

> r23 SEED. r23's focus is the **best-effort / soundiness** design the corpus circled
> before "ui-B" scope-crept it away — now the round's whole subject. Grounded in the
> human-authored `../IMPLEMENTATION.md` "Correctness vs. best-effort: a band". This is
> BROAD: not just the trust-taint lattice — a deep pass to find WHICH gradients got
> accidentally collapsed to booleans during the early implementation rounds, and which
> core components we need to actually *reach* best-effort in code. Opens with research
> (don't assume the answer is "just user-trustingness"). The orthogonal "time axis"
> (concurrent/incremental analysis) is DEFERRED to a later round (`plans/22H`). Output of
> the r22 wrapup design dialogue (`notes/224` §10). Marks: +SURE/~SUSPECT/-GUESS/--WONDER.

## §0 The thesis: best-effort = graceful degradation = gradients, not booleans

`IMPLEMENTATION.md` states the spec verbatim: "as either admin-user-behaviour or
oracle-author-behaviour degrades, **our behaviour degrades only as much as necessary**,
in the precise ways forced by the user error/omission, and no further." To degrade *only
as much as necessary*, the engine must KNOW how-far each input is degraded — i.e. a
**gradient**. The early rounds instead baked BOOLEAN assumptions ("oracle present ⇒
oracle complete"; "we can just depend on the oracle for that") that **cliff-edge** —
full-trust or ⊤ — rather than degrade gracefully. r23 = find those collapsed gradients
and walk them back.

This is the **decision-side** of r22's errors+provenance thesis. The why-lens built the
*reporting* side ("tell the user what happened / what to fix"); gradients are how the
engine *reasons* best-effort. `IMPLEMENTATION.md`'s "two angles on provenance" —
*where-from* + *how-much-we-trust*, the latter across **competence** AND
**security-privilege** — already says trust alone is ≥2 axes; and there are likely
*other* collapsed gradients beyond trust entirely.

**The CRITERION for which booleans become gradients** (human, `IMPLEMENTATION.md`
2026-06-14 — the load-bearing test): a gradient exists **iff partial-benefit /
partial-failure exists**. Best-effort IS precisely a *failure gradient* — "we only fail
you as much as you've already failed yourself." Two questions we ask the oracle-author
show the asymmetry, and they bound the whole r23 hunt:
- **"Does your oracle ever MUTATE?" (probe-safety) — NO gradient, by nature.** The
  contract (oracle mutation-free; no mutation-on-probe) either holds or **fully
  collapses** — there is no "partially mutated" state to aspire to, nor a "partially
  mutating" one to reach toward. So it stays BINARY, and that is *why* `dc-probe-NOT`
  (§3) is a hard, **principled** exclusion — not an arbitrary weld.
- **"How COMPLETELY have you modeled the command?" (apply-time coverage) — a gradient,
  by nature.** Partial benefit exists: Dorc can elide many commands, or fewer-but-still-
  some; an under-modeled / low-resolution oracle should reach toward the half-beneficial
  outcome rather than cliff to ⊤. The canonical gradient (`dc-elide-on-trusted-default`, §3).
So §1's sweep has a sharp test: hunt for booleans where *partial benefit genuinely exists
but we collapsed it* — and do NOT gradient where the failure mode is total (mutation, and
anything else with no partial state). This also re-frames `inv-kfail` (§5): the
execute-priority ladder (never under-execute > avoid over-execute > avoid
unnecessary-execute) is the *direction* the apply-coverage gradient may move (toward
over-/unnecessary-execute), never the mutation contract.

## §1 r23 OPENS with research — the collapsed-gradient sweep (fan-out + main-context adjudicated)

A fan-out sweep of the planning corpus + the spike source for every place a decision
quietly **collapsed a gradient to a boolean** — heavily the "oracle is perfect/complete"
marriages, but NOT assuming it is only trust. Subagents return CITATIONS; read the raw
hits + adjudicate in main context into a **walk-back map** (what to un-collapse, ranked by
lock-in / "hard-to-unbake"). This sweep is also the place to surface the TODO-ADDTL
"oracle-contract sh-spelling" + "verb-set / fail-fast contract" items, which are where the
completeness gradient (§4) will have to be spelled.

Candidate collapsed gradients to SEED the hunt (the sweep finds the real set — do not
treat this as the answer):
- trust/**COMPETENCE** (oracle thoroughness/completeness) + trust/**PRIVILEGE** (security)
  — the two `IMPLEMENTATION.md` axes.
- oracle **COVERAGE** — which aspects of a command are modeled. Per-channel ⊤ already
  gradients the *observables*; but effect-**completeness** ("did the oracle declare ALL
  this command's effects, or just the headline one?") is boolean-assumed today.
- **CARDINALITY** (singleton vs multiple, `an-entity-uniqueness`) — drives strong/weak
  update; is the "is this a singleton" answer boolean or confidence-graded?
- **CONVERGENCE-confidence**, **REACHABILITY**, the `May`/`Must` coarseness, … — open.

## §2 The worked exemplar: trust-taint as an information-flow analysis

The most-developed instance (from the r22 dialogue), as the pattern the others follow:
- Trust-taint is a per-value lattice **cell** — NOT the receipts chain — tracking
  source/coverage, flowing into BOTH the engine arena (graceful degradation) AND the
  receipts (attribution). It **rides the existing value-flow**: trust flows where
  *information* flows.
- Types today: `OriginKind` (`core/prov.rs`; the receipts; ru-11 **DECISION-INERT**;
  `OracleClaim`/`ProbeResult` tiers RESERVED-not-minted) has the SOURCE but cannot decide.
  `Predicted<T>` (the decision plane) has the decision-power but is SOURCE-BLIND. Neither
  is the decision-plane trust-taint; it is a NEW cell the receipts *reference*.
- **ru-11-ORTHOGONAL** (the convergence we reached): a separate taint cell driving
  decisions is NOT a weld breach. ru-11 welds the *receipts chain* inert/erasable; the
  taint is a decision input. Strip the receipts → the taint remains → the erasability gate
  still passes. Discipline: the taint lives on the decision plane; the receipts hold a
  reference, never the reverse.
- **Counter-example refinement (you CAN gain certainty by computing):** `flaky-⊤ || true`
  has a provably-`0` rc (`StatusInvariant`/door-3) — output MORE trusted than its input.
  So trust is NOT a naive meet-over-inputs; it is a **meet over the output's SEMANTIC
  dependencies** (where information doesn't flow — the `|| true` left w.r.t. the rc — no
  taint). Corroboration (independent agreeing sources) is a second up-trust path. The
  lattice is subtler than monotone-downgrade; the structural/pure-CFG tier can *recover*
  certainty.

## §3 The decision-surface — where trust-level CHANGES an analyzer decision (the core r23 question)

- **dc-elide-on-trusted-default (headline).** An unpredicted *consumed* observable is
  ⊤⇒run today (`inv-probe-sourced-values`: a ⊤ consumed channel forbids the mint). IF the
  author **vouched** the default complete (trust) it could replace; IF it is a lazy gap it
  must run. Same source line, OPPOSITE elision, keyed only on trust. = the completeness
  tension (§4).
- **dc-disagreement.** `merge_observable` collapses a same-cell disagreement to ⊤⇒run
  today; a trust-aware merge could prefer a probe-*observation* over an oracle-*claim* ⇒
  replace. A conservatism-FOR-precision trade — a deliberate, human-blessed **edge**, not a
  default (the project leans conservative-⊤).
- **dc-probe-NOT (the exclusion-check).** Probe *shipping* must stay BOOLEAN self-vouch —
  "no analysis-confidence threshold ever makes a probe safe" (standing ruling). The
  trust-gradient must NOT leak into probe-safety. A must-stay-boolean pin.
- Honest sizing: if this list stays SHORT, trust-taint is mostly a *reporting* concern with
  a thin decision-edge — which right-sizes the effort. The research measures it; do not
  pre-assume a large decision-surface.

## §4 The hard open tension: knowing how good an oracle is

"How good is this oracle" is **totally unspecified** today, and the crux is that **absence
is ambiguous**: a missing channel-prediction could be *"author reviewed it, the default is
correct"* (trusted) or *"author was lazy, it's genuinely unknown"* (gap) — and nothing in
the source distinguishes them. Direct design-tension: the magic/lazy/default value says
DON'T force a `<key: nothing>` annotation on every imaginable channel; the soundiness value
says DO track completeness as a known, trustworthy thing.

The only shape that serves both (~SUSPECT, to be designed): **gradual-enhancement** —
DEFAULT conservative (absence = ⊤ = run; lazy authors stay safe-but-pessimistic), with an
**opt-in, sh-spelled completeness vouch** that diligent authors add to unlock
`dc-elide-on-trusted-default`. But *how you spell "I vouch this default is intentional" in
idiomatic sh* (annotation-by-narrowing, NOT a Dorc YAML key — AGENTS "how would you write
it in sh?") is itself unsolved and hard, and ties directly to the still-open
oracle-contract sh-spelling (TODO-ADDTL #1). Write it as the open design question.

## §5 Welds + constraints (re-test EVERY proposed gradient against these)

- **inv-kfail.** A gradient may only add precision **in the safe direction** (toward run /
  toward conservative) or feed reporting — it may NEVER license a decision *less* safe than
  the boolean floor already permits. Soundiness degrades toward over-execute, never toward
  under-execute.
- **ru-11.** Receipts stay decision-inert; the taint is a SEPARATE decision-plane cell
  (orthogonal, §2). Erasability holds.
- **exclusion-check (AGENTS).** Probe-safety stays boolean (`dc-probe-NOT`). For every
  proposed gradient, re-test: does it leak into a place that must stay boolean for
  soundness? The lattice **rungs** (esp. probe-observed-vs-oracle-claimed ordering) are a
  SOUNDNESS call — wrong rungs = a soundness error, not churn. Design them adversarially.

## §6 Method: xfail-first (the human's), then design → crosscheck → build

r23's FIRST build-step is to encode the desired gradient behaviours as FAILING tests
(xfail pins), because getting the rungs right is a soundness question and behaviour should
define them before any type exists:
- the `dc-*` cases (`dc-elide-on-trusted-default` run-today / elide-under-vouch; `dc-disagreement`),
- the `|| true` certainty-recovery (does the engine keep rc-certainty through a ⊤ left, per §2?),
- the `dc-probe-NOT` **must-stay-boolean** pin (a gradient that ever made a probe ship is a red XPASS).
Then: design the gradient(s)/lattice → **adversarial crosscheck the design** (the
design→crosscheck→build loop that caught the real errors in r22) → build, walking back the
collapsed booleans per §1's map.

## §7 Sequencing + scope

r23 = the **CERTAINTY axis** (gradients). The **TIME axis** (concurrent/incremental
analysis — the live-plan engine) is DEFERRED (`plans/22H`, top of TODO-ADDTL); they are
orthogonal, and the §1 research decides whether/how later rounds interleave them. This is
**almost certainly multi-round** — a corpus-wide soundness walk-back + a gradient redesign
+ (later) the live-plan engine is not one round; r23 opens the arc (research + the trust
exemplar + the walk-back map), it does not finish it. This is the single most important
thing r22 *surfaced*: the why-lens was best-effort's reporting half; this is its
engine-reasoning half.
