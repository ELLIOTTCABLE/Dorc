# 165 — locking down the "soundness is per-consumer" minefield

> **Status (2026-06-05): spike design, governs the upcoming analysis phase.**
> Human-flagged (the `set -e` "more arrows = safe" case): small CFG/analysis
> details are correctness-critical in *two* non-obvious directions at once, so a
> shared artifact has NO globally-safe choice — safety is per-consumer. This note
> theorizes how to lock that down hard (types + witnesses + calibration +
> concentration). Reasoning lives in the session transcript; this is the durable
> contract. Confidence-marked; the priority tension at the end needs the human.

## 0. The hazard, precisely
For a given CFG/analysis detail, BOTH over- and under-approximating it can be
unsound — for *different* consumers (extra `set -e` edge: safe for "can I skip
this?", hazard for "is this host converged?"; removing it flips which breaks).
So "the analyzer is conservative" is meaningless without "for which consumer, in
which direction." The bug class is **a correct value consumed in the wrong
orientation, silently** — no crash, no wrong number, just an answer sound under
assumptions the reader doesn't hold. Worst possible failure mode for a
lower-reasoning agent (or a tired human).

## 1. Governing principle
**A value must never exist without its soundness-orientation in its type.** No
bare `Powerset<Fact>`, no bare `bool`-skippable, no bare `Verdict`. Orientation
is a type parameter; mixing is a compile error; the only *unsound* coercion is
unwriteable (you may weaken `Must→May`; never `May→Must` without re-proof).

## 2. The layered lock (hardest → softest)
- **L1 compile-time (primary):** `May<L>` (truth ⊆ this, merges with ⊔) vs
  `Must<L>` (truth ⊇ this, merges with ⊓); the solver is parameterized by
  direction so it picks the merge — kills the union-where-you-needed-intersection
  bug. `Must::weaken()->May` exists; `May->Must` does NOT. Same for phase:
  `Verdict<Probe|Apply>` where a `Bias` trait forces `on_unknown()` (Probe⇒
  Withhold, Apply⇒Act) via an exhaustive match — no `_ => Skip` fallthrough is
  writeable, and a probe verdict can't be read as an apply verdict.
- **L2 witness:** eliding a command (the irreversible action) requires a
  `SkipLicense` with private fields, minted only by one reviewed
  `prove_skippable(must-established ∧ not-undone ∧ must-grade)`. The plan emitter
  has no other path to "skip." The license carries its `Derivation` (audit trail
  + the greyed-out-why UI).
- **L3 typed CFG views:** edges tagged `Certain|Speculative` (a `set -e`
  failure-edge is Speculative); expose `must_succ` (Certain only) vs `may_succ`
  (∪ Speculative), never raw `succ`. The flagged over-approx edge becomes
  structurally unmixable, and the tag is where `set -e` gets made precise later
  without touching consumers.
- **L4 runtime defense-in-depth:** re-check the skip-precondition at the
  irreversible boundary (and re-probe before apply, with a real host); `Unknown`
  is absorbing + loud + can never silently become "skip."
- **L5 calibration (types can't prove transfer soundness):** differential vs real
  shell (`May ⊇ observed`, `Must ⊆ observed`); metamorphic "adding a Speculative
  edge can only SHRINK the skip-set" (guards the flagged claim directly); plus the
  framework iteration-cap returning didn't-settle, not hanging.
- **L6 concentrate the judgment:** all direction/phase/approx decisions live in
  one tiny reviewed "analysis registry" (~"reaching-defs = Forward+May; skip-gate
  consumes Must-reachability; converged-verdict = Apply+Must"). Subagents get a
  transfer-function-shaped hole for a *declared* lattice — they can't choose
  orientation; wrong tag ⇒ won't compile; unsound transfer ⇒ differential test
  fails. The mine is never in the agent's blast radius.

## 3. Priority ruling (human, 2026-06-05): calibrate UP on correctness
**Decision:** lean *harder* into the type-locks than readability alone would
warrant. This is the throwaway landmine-finding stream, so the heavy-typed
approach at max cost, splattered everywhere, IS a goal — it surfaces what such a
codebase feels like in practice. Exact line left to me; deliberately **VARY** the
lock-style across modules (some hard-typed, some convention-fenced) as extra
state-space exploration. Watch-item: if it becomes *literally* thousands of
wrappers drowning agent context, switch to SHORT type-names — a mechanical change,
deferred until the first analysis files show the real density. My prior lean
below is now the FLOOR, not the cap.

*(Original tension, recorded:)*
Validation (priority-3) vs readability (priority-1): phantom-typing *everything*
maximizes the lock but drowns the code in `May<…>`/`Verdict<Apply>` noise, hurting
the top priority. ~SUSPECT the line: type-lock only the *catastrophic* axes — the
may/must split ON THE ELISION PATH, the phase split ON VERDICTS, and the
`SkipLicense` witness — and leave the rest as calibrated-convention inside the
concentrated registry (L6). Type-locking the whole engine is over-engineering
against the stated priorities. Also: `Must` needs a *meet* the current `Lattice`
trait lacks (only `join`) — real machinery, +SURE worth it on the elision path,
~SUSPECT not engine-wide. And L1's lock IS the one-way coercion; without
forbidding `May->Must` the wrappers are decoration.

## 4. The one-line rule for the build
Every analysis result type answers, in its type: *over or under? probe or apply?*
— and the one dangerous verb (elide) takes a witness, not a bool.
