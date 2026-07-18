# 16E — state vs CFG: the unified read/write model (working slug, actively updated)

> **Status (2026-06-05): spike, ACTIVE working-model — updated in place across the
> discussion (human asked for one living slug here, unlike the append-only round
> notes). Round 16: …16D → 16E.** Unifies the corpus's ambient/transient/
> save-restore/probe-ability talk with the recent skip-and-substitute thread into
> one model of state + control-flow. **VALIDATED 2026-06-05 WITH A CORRECTION — the
> §1 recant over-corrected; read §3a/§4 before §1.** The narrow forward fact holds;
> the slogan "the after-side does no work" wrongly deleted a co-equal *backward*
> (result-liveness) obligation. Confidence-marked.

## 0. The frame
Apply phase. Treat every piece of state opaquely (we do NOT track values for
data-flow): it holds *some* value; operations READ it or WRITE it. Goal: skip
WRITES already done (state already holds what the write would set) without breaking
any READ. A "read" (human's term, refined) = any point that consumes mutable
state: a non-elidable probe that must run before a mutation, or a mutation that
itself uses state.

## 1. The core model (corrected — supersedes the earlier 2×2 framing)
To skip a write `W` (sets fact `F := v`), you need: *is F already v when W would
run?* You learn it by probing F *before* the run (early). W runs later. The ONLY
thing that invalidates the early probe for W is **another write to F between the
probe and W** — a write *after* W cannot change F at W's moment.

⇒ **For skipping writes, only the "before" side matters; the "after" side does no
work.** That is exactly the reaching-definitions gate already built (a write to F
upstream of W ⇒ W not safely skippable by the early probe).

**Recant (what the prose discussion over-claimed):** the earlier 2×2 (change-
before-read × change-after-read) presented two co-equal axes. For the *core* job
(skip writes) the after-axis is irrelevant. The after-axis is real only for a
*different, deferred* job — skipping a READ and reconstructing its value
(skip-and-substitute, note 16C): there a later write destroys the value you'd need,
because you can't probe the past. So "no-before/yes-after is critically important"
and "≡ the temporal problem" are **retracted for the core model**; they hold only
inside the deferred read-substitution feature.

## 2. What actually survives
- **The trajectory, not the grid.** No separate "after" property of a point — the
  "after" of one read is the "before" of the next. The real object is the *sequence
  of writes along the path*; every decision is valid iff no write to that fact falls
  between the probe and the point that matters. The 2×2 is a two-event snapshot of
  that sequence (a useful human taxonomy, NOT the core type).
- **Two orthogonal questions, previously blurred:** (a) *is my probe still valid
  here?* — depends only on whether a write intervened (decidable structure,
  value-independent); (b) *is the state already the value I want?* — the actual host
  observation. Validity ≠ convergence.
- **The real asymmetry:** a *before*-change is recoverable (re-probe later, right
  before the point); you cannot probe the future, so a lost *past read-value* is
  unrecoverable. This is why skipping writes is tractable and substituting reads is
  hard — and it is NOT the (yes,no)-vs-(no,yes) cell pairing claimed earlier.
- **Transient / save-restore is not a separate category.** `setenforce 0; work;
  setenforce 1`: `work` reads F after the change ⇒ early probe stale ⇒ same single
  rule. The restore is only a trap for a naive net-change check; the reaching-writes
  rule never falls for it.

## 3. State vs CFG, and the three locations (the unification the human asked for)
- **CFG is partly a function of state-reads.** A guard (`if [ -f X ]`) and `set -e`
  are reads of state whose result selects a branch / decides an abort-edge. So
  "state affects the CFG" is just the *control-consumption* case of a read, governed
  by the same before-writes validity. State and control-flow aren't two things — the
  graph's shape is, in part, the output of reading state.
- **Three state locations, one model** (all "facts" with reads/writes):
  1. *script-local* (shell vars `$cid`): written by output-capture, read by `$var`.
  2. *script-global / shell-env* (errexit, cwd, traps, IFS): written by `set`/`cd`/
     `trap`, read implicitly; this is the location that most directly shapes the CFG.
  3. *target-system* (packages, files, services): written by mutators, read by
     probes/guards; what oracles observe.
- **Transitions are just commands that read one location and write another:**
  system→local (`cid=$(docker inspect …)`), local→system (`rm "$path"`),
  shell-env→control→system (`set -e` deciding whether a system-mutation aborts). The
  model treats all three uniformly; the transitions are ordinary read-then-write.
- **Taint (the oracle nod, kept orthogonal):** location 3 values come from oracle
  probes ⇒ *untrusted* (oracle may lie — the 16D lens, conditional + conservative-
  fold). Locations 1–2 are analyzer-derived ⇒ sounder, but bounded by our modeling
  (we don't track var values). Taint marks *which facts' values came from an
  untrusted source*; it rides alongside the read/write model, it isn't part of it.

## 3a. VALIDATION RESULT (adversarial pair, 2026-06-05) — the recant OVER-corrected
A clean neutral+adversarial pair checked §1–§4 against shell semantics and the real
kernel. Outcome: the *narrow* claim survives, the *generalization* does not.
- **Survives (+SURE):** "fact F's value at a point depends only on writes-to-F
  *before* it" is physics — true in every phase/direction. So the forward
  `EstablishAmbient`-vs-`Written` gate (reaching same-fact writes) is right.
- **Over-correction:** §1's slogan "the after-side does no work for the core" let a
  true-narrow *forward* fact (value-validity) stand in for the whole *skip
  decision*. The decision also needs **backward liveness**: is the command's
  **stdout / exit-status needed downstream?** That propagates consumer→producer — a
  *reverse* analysis I deleted. The 2×2's two axes were really **forward-reaching
  (value here) × backward-liveness (result needed)**; I dropped the backward one.
- **What's live vs latent (verified by tracing, NOT relaying — important):**
  - `apt-get install … && start` IS skipped by the kernel, but the skip is
    **benign**: a converged install exits 0, so `&& start` runs either way. (The
    adversarial agent over-claimed this as a wrong-skip; tracing it refutes that.)
  - **stdout-consumed** (`install | tee log`, `x=$(install …)`) — the skip drops the
    output → real divergence. This is exactly Owes::Output / skip-and-substitute
    (note 16C); the kernel mis-handles it now.
  - **status-consumed where converged⇒non-zero** (`mkdir d || handle`: `mkdir`
    errors when `d` exists) — real divergence, but **latent**: the package oracle
    has no converged-non-zero command, so not exhibited today.
- **The recurring anti-pattern (human, 2026-06-05):** excluding a quadrant as
  irrelevant because it's irrelevant in *one* cell, then it returns via the reverse
  direction / other phase / other user. This is the 3rd instance (errexit find-8;
  subst-internals "aren't leaves"; this). **Standing check:** before excluding any
  edge, re-test it under reverse-direction, the other phase, the other user, AND the
  reliable-oracle case; if irrelevant in only one cell it is deferred, not
  irrelevant. And: *verify a claimed failure by tracing it; don't relay it.*

## 3b. The lens-pair sweep (why the exclusion fails in every cell)
The quoted "for the core job only one edge carries weight" was tested against the
three lens-pairs. The *narrow* forward fact (F's value = before-writes-to-F) holds
in all cells — but the *decision-level* claim fails in each:
- **probe vs apply:** in *probe* it INVERTS — a probe exists only for its output,
  which the skip-decision consumes, so the backward/result edge is the whole point,
  not a second concern. In *apply* it fails via stdout-liveness (real — `install |
  tee log`) and converged-non-zero status-liveness (latent — `mkdir d || handle`).
- **admin vs engineer:** the admin's book carries all edges in its own script; for
  the engineer the backward edge IS their contribution — the gather/compute bridge
  (16C) exists precisely to *discharge* "your result is needed," making a
  result-live command skippable.
- **reliable-oracle vs blind:** blind ⇒ everything Opaque ⇒ nothing skips ⇒ the
  claim is vacuously "true" and useless; reliable ⇒ skips actually happen ⇒ every
  edge is load-bearing. The exclusion looks safe exactly where it doesn't matter and
  is most-wrong exactly where it does.
All three failures are the SAME excluded object — backward result-liveness —
reappearing through the reverse-direction, other-phase, and other-user doors. That
is why the §3a standing check must test all four (reverse / other-phase /
other-user / reliable-oracle), not just spot-check one.

## 4. The lattice / "what stays lifted" (VALIDATED)
The 2×2 does **not** become the core lattice type, and neither does a single
reaching-defs lattice. The core is a **product of two analyses in opposite
directions**:
- **forward** — reaching same-fact writes → value-validity (`EstablishAmbient` vs
  `Written`); the gate already built.
- **backward** — liveness of a command's **stdout and exit-status** (is the result
  consumed downstream?); **not built** — this is the hole, and it is the *same
  object* as skip-and-substitute's output-dependency (16C). Status-liveness only
  bites when converged⇒non-zero; stdout-liveness bites whenever output is captured/
  piped.

A skip is licensed only by discharging the **product** of co-equal obligations:
(i) forward value-validity + converged; (ii) backward result-liveness = dead, or
bridged by an oracle; (iii) Must grade. The kernel does (i)+(iii) only.

What stays **lifted** (the Must/May analog — the caller must collapse, the type
forbids the unsafe shortcut): **two** tokens, not one —
- a probed value's **freshness** (valid only until the next write to that fact;
  crossing a write is a type-level invalidation → re-probe or "unknown ⇒ do the
  work"); and
- a command's **result-liveness** (its stdout/status is dead, or discharged by a
  bridge) — a skip cannot be minted while a live result is undischarged.
This is the direct analog of `May ↛ Must`: don't collapse "I observed F" into "F is
globally v, and nobody needs this command's result"; keep both as separate proofs
the caller must present. The orientation (probe-withhold vs apply-perform) stays
lifted via the phase type as today. (The 2×2 is a derived human view; it
under-counted because it folded all downstream concerns into one "after" axis when
they are at least two backward properties — stdout-liveness and status-liveness.)

**STATUS: validated.** Safe to reason about type-system choices on §4. The concrete
spike consequence: `prove_skippable` needs a backward result-liveness obligation
(stdout+status dead, or bridged) added beside the forward gate; without it,
stdout-consumed establishes are mis-skipped (and converged-non-zero status ones
will be, once such an oracle exists).

**NOTES INDEX:** …16C skip-and-substitute · 16D degradation lens · 16E (this —
state/CFG unified read-write model; ACTIVE, pending validation).
