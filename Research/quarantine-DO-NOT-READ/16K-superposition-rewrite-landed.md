# 16K — the superposition rewrite landed (consumption → engine, collapse → caller)

> **Status (2026-06-05): spike, round-16 implementation summary.** The def-4 rewrite
> specced in 16J is built and green. Key refinement to 16J's hedge recorded below
> (§1). Append-only (round 16: …16J → 16K). HEAD `eb5f864`. Confidence-marked.

## 0. What landed
- **`951fcbb` (the rewrite):** output-consumption moved from `plan`'s hand-rolled
  `walk_consumed` INTO the engine, computed during CFG lowering and stored per node
  as `Powerset<Observable>` on the `Cfg` ([`Cfg::consumed_observables`]), emitted
  **un-collapsed**. `plan` now collapses it: `prove_replaceable<P: Bias>` is generic
  over the phase (no hard `<Probe>`) and takes `May<Powerset<Observable>>`; per
  `inv-must-may` a `May` value can only **block** a license. `Observable` moved to
  `analysis::cfg`. Deleted `walk_consumed`/`consumed_output_leaves`/`has_output_redir`/
  `ObservableUse`/plan's `word_text` (kept `has_top_successor`).
- **`eb5f864` (tests):** 8 engine-level consumption tests (`tests/cfg.rs`) pinning the
  fact directly (own-redir stdout/stderr, `/dev/null` scalpel, non-last pipe stage,
  the two enclosing kill-shots, enclosing-`/dev/null` scalpel, lone-quiet) + a def-5
  totality cross-check (`plan`). Matrix unchanged + still green (20 + 1 ignored).

## 1. The refinement to 16J (record for any future def-4/engine work)
16J hedged "~SUSPECT a backward pass over the CFG via `solve` … -GUESS the minimal
version may be attach-during-lowering." Having read `cfg.rs`, the answer is firm:
**attach-during-lowering is correct; a backward dataflow fixpoint is the WRONG fit.**
+SURE of the reasons:
- `lower_node_inner` is **already an exhaustive match over all 14 `NodeKind`s — no
  `_ =>` catch-all** (def-2 satisfied by construction; adding a variant is a compile
  error there).
- The builder already propagates enclosing context to inner nodes via **arena-range
  marking** (`expansion_internal`, `clear_fallible_range`). Consumption uses the same
  idiom (`mark_consumed_range`): a pipeline non-last stage / redirected group/subshell
  marks every `Command` in its lexical range — exactly the enclosing case the old
  leaf-local gate missed (16G kill-shot).
- **The CFG flattens pipelines** (`lower_pipeline` sequences stages; it carries NO
  pipe-edges), so a CFG-dataflow liveness pass literally could not *see* pipe-
  consumption without first adding that structure. The lowering, which still has the
  `Pipeline` AST in hand, can. So the engine "owning completeness" here = the
  exhaustive lowering traversal, not a `solve` fixpoint.
- Result is a `Vec<Powerset<Observable>>` indexed by `CfgNodeId`, length = node_count
  ⇒ **total over nodes**: an empty set means examined-and-quiet, never un-examined.
  The "absent leaf" that slipped the old plan-side dual-traversal (16I bug-c) is now
  **structurally impossible**, not merely test-detected. (def-1's intent — "didn't
  look ≠ safe" — achieved by totality, not a flipped default.)

## 2. Where the phase-collapse actually lives (and what I did NOT do)
The 8-pole sweep (16I-followup) said the danger is baking a phase-collapse into the
engine. Built accordingly:
- **`prove_replaceable<P: Bias>`** forces the phase to be *argued* (exclude): you
  cannot mint a license without naming `P`. The engine fact is phase-agnostic.
- **The consumed-block is phase-INVARIANT-sound** (~SUSPECT, reasoned): a consumed
  unvouched output means the `true`-stub's empty default diverges from a real
  consumer — true in BOTH phases. So the block lives in `prove_replaceable`, not in
  `Bias`. The phase-keyed part is the **disposition** of a blocked leaf: apply ⇒ run
  (`build_plan`'s `None => Disposition::Run`, kFAIL-perform); probe-projection ⇒
  *withhold* (its own collapse, unbuilt). That site is commented as the apply collapse
  (`inv-superposition`).
- **I deliberately did NOT add `Bias::on_consumed_output`** (16J floated it). Reason:
  `Resolved` is `{Replaceable, Run}` — apply-shaped, no `Withhold`. A per-phase
  consumed-direction method would force the (unbuilt) probe impl to return `Run`,
  which is the *wrong* probe answer (probe withholds) — i.e. it would bake an
  apply-shaped answer into the type, the very thing the sweep warned against. The
  honest forcing-point for probe is `build_plan`'s phase-specificity: the probe
  projection needs its OWN plan-builder, and writing it forces the `None ⇒ ?` choice.
  ~SUSPECT this is the right call; flagged for re-litigation when the probe phase or a
  `Disposition`/`Resolved` with a `Withhold` arm lands.

## 3. `inv-superposition` (proposed registry entry — human's call)
Recorded in code comments + notes 16J/16K. The `inv-*` registry is `spike/CLAUDE.md`
(AI-authored, human-owned-by-convention), which does NOT yet list it. Proposed entry,
for the human to paste (not edited autonomously — that file steers all future agents):

> **`inv-superposition`** — the analyzer kernel emits phase-/orientation-agnostic
> lattice facts; only the phased caller collapses them, by arguing the phase
> (`Bias`/`PhasedVerdict<P>`) and orientation (`May`/`Must`). The engine must never
> fold `May`/`Must` or pick a phase default. Note 165-L1 generalized from the verdict
> to every phase-sensitive fact (DESIGN :83 "Same analysis, different fail-safe
> posture"). Grounds: a baked posture is a wrong-skip under the opposite phase's
> `kFAIL`.

## 4. State (network-free kernel; whole workspace green + clippy-clean)
`core` · `syntax` · `analysis(lattice,solve,cfg,effect)` · `oracle` · `plan` ·
`hostsim` · `cli`. Tests: analysis 18 · cfg **34** · core 5 · hostsim 6 · oracle 8 ·
plan-lib **11** · matrix 20 (+1 ignored) · syntax 2+16. Clippy clean.

## 5. Still deferred (unchanged by this round)
- The **probe projection** mechanism (16J §5) — unbuilt; its consumed-block
  disposition (withhold) and whether it reads this exact fact remain open.
- **fd-dup resolution** (`2>&1`, `>&3`) — the structural floor stands; `> /dev/null
  2>&1` stays replaceable (precision refinement).
- **HOLE#1** (16I) — substs in redirect-targets / case-patterns not lowered into the
  CFG, so their `Kill` doesn't poison — the CFG-lowering completeness item.
- **The apply executor + multi-host** (16A) — direction only.

**NOTES INDEX:** …16I core attack · 16J superposition spec · 16K (this — the
superposition rewrite landed; attach-during-lowering confirmed over a fixpoint).
