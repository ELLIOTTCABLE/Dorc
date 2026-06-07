# 16M — apply-2: the eliding-apply compiler (probe → simulate → elide)

> **Status (2026-06-06): spike, round-16 — apply-2 first pass.** The "wrap up the
> analysis engine" milestone: the compiler now produces BOTH a probe and an
> eliding-apply, and the full chain is e2e-tested against the host unit. Append-only
> (round 16: …16L → 16M). HEAD `9a9d663`. Confidence-marked. (The grand cross-round
> synthesis is deferred to a fresh agent per the human; this note is just the
> apply-2 round.)

## 0. The three applies (human's taxonomy, 2026-06-06 — sharper than DESIGN)
- **apply-1 — full unconditional:** just ssh + a wrapper; run everything. (DESIGN's
  "dumbest way", lines 47–52. Trivial; the fallback.)
- **apply-2 — converge + safe-elide (THE DEFAULT, built this round):** probe the
  host, then elide anything we can prove safe to omit (already-converged). Forward
  only.
- **apply-3 — targeted desired-set (`dorc bump`, DEFERRED):** apply the user's set,
  eliding what we can't prove relevant. The human's key insight: **apply-3 ⊃ apply-2**
  — it is apply-2 *plus* a backward relevance-reduction (probe → apply → further-
  reduce), a strict superset of the effort, not a separate path. Needs the backward
  engine (the invertible direction, still only unit-tested).

## 1. What was already there vs. what landed
- **Already built (the eliding-apply itself):** `build_plan(verdicts) → Plan{Run|Replace}`
  + `Plan::render_sh` = a valid sh with the safe-to-omit leaves commented out
  (`# replace[…]: <sh>  ↳ <fact> already holds`). So apply-2's *elision* half existed;
  this round did NOT rebuild it.
- **Landed (`9a9d663`) — the missing probe half + the chain link:**
  - **`compile_probe(classes, probe_body) -> ProbePlan`** — the FORWARD half of the
    compiler: the read-only fact-checks to ship (every `EstablishAmbient` fact whose
    kind has a *declared* probe), as a renderable read-only sh (`ProbePlan::render_sh`).
    The oracle seam is a closure (`Fn(KindId) -> Option<String>`) so `plan` keeps no
    `oracle` dep. (Named `ProbePlan`, not `Probe` — `Probe` is the phase-marker enum.)
  - **The "can't-probe ⇒ can't-elide" link** (+SURE, the real new soundness bit): a
    kind with an effect but NO declared probe is un-checkable ⇒ absent from the probe
    ⇒ the apply runs it (`kFAIL-perform`), even on a host that holds the fact. Pinned
    by `apply2_unprobeable_fact_is_not_elided`.
  - **The assembled chain, e2e vs `hostsim`:** `dst_apply2_chain_probe_simulate_elide_over_seeds`
    drives source → cfg → classify → `compile_probe` → SIMULATE (host answers each
    checked fact; an unchecked fact ⇒ `Unknown`) → `build_plan` → render, over 64
    seeds: install elided ⟺ host holds it; un-oracled reload always runs; probe
    renders read-only.

## 2. Scope / what this is NOT (the human's split)
- Still entirely **inside the analyzer/compiler unit** — it *compiles* a probe and an
  apply; it does NOT execute. No executor, no host mutation over time, no
  re-probe-before-apply (TOCTOU), no idempotence-by-execution — those are the **apply
  EXECUTOR (Option C)**, the next round.
- **Forward only.** apply-2 needs no backward analysis; the invertible engine's
  backward direction stays exercised only by its `solve` unit test until apply-3.

## 3. DESIGN holes surfaced (suggested tightenings for the human — NOT edited)
- **hole-1 (load-bearing): the probe→apply contract is implicit.** DESIGN says probe
  under-approximates / apply over-approximates but never pins *what the probe yields
  and exactly what gates an elision*. The contract apply-2 implements: **elide leaf L
  iff probe(L.fact)=Converged ∧ ambient ∧ Must ∧ no consumed unvouched observable.**
  Lives only in code + 16F/16J, not DESIGN.
- **hole-2: "no probe ⇒ no elision" unstated.** Elision is gated on probe-availability
  (this round's link); DESIGN assumes probes exist but never says a probe-less fact
  must run.
- **hole-3: the two elisions are blurred; the three applies aren't enumerated.** DESIGN
  line 81 runs "already-correct (by-probing)" and "no-interdependency (partial)"
  together as alternatives; they are two distinct elision KINDS, and apply-3 ⊃ apply-2.
  `KNOBS kELISION` half-distinguishes them; DESIGN should split line 81 + cross-ref.

## 4. State (network-free kernel; whole workspace green + clippy-clean)
core 4 · analysis 19 · cfg 31 · oracle 8 · plan-lib 12 · matrix 9 (+1 ignored) ·
hostsim 8 · parse 17 · syntax-lib 0. The compiler now emits a probe (forward) and an
eliding-apply (forward), chained + e2e-tested against `hostsim`.

## 5. Next (per the human's plan)
- **(now, human-kicked) "half-assed Option B":** minimum-effort weirdo inputs/host-
  states to confirm the architecture *handles* weirdos (not polished).
- **(later) Option C — the apply executor / simulated orchestrator** (TOCTOU,
  idempotence-by-execution, mutative/unreliable oracles, multi-host/CAP) — note 16A.
- **(later) apply-3** — the backward relevance-reduction on top of apply-2.

**NOTES INDEX:** …16K superposition rewrite · 16L test-suite audit · 16M (this —
apply-2 eliding-apply compiler: probe → simulate → elide).
