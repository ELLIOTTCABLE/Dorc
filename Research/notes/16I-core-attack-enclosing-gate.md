# 16I — attacking THE CORE: enclosing-scope observable gate + ⊤-containment

> **Status (2026-06-05): spike, round-16 — core-focused adversarial round + fix.**
> Per the steer ("attack *the core*, not unimplemented side-aspects — it's a
> dismissal codebase"), I re-ran the `adversarial-crosscheck` pair aimed squarely at
> the *implemented* replace decision (`EstablishAmbient ∧ Must ∧ Converged ∧
> reachable ∧ no-consumed-unvouched-output ∧ not-⊤`), with the deferred features
> (oracle output-bridge, `Grounded<T>`, cross-host, rc/stdout *value* analysis)
> explicitly fenced off so the agents couldn't burn the round on known-parked work.
> It found a **convergent soundness kill-shot in the gate I had just landed (16H)**.
> Fixed; the deferred `#[ignore]` containment spec is now green; one new hole surfaced
> and deferred with a spec. Append-only (round 16: …16H → 16I). HEAD `4cbd1b1`.
> Confidence-marked.

## 0. The kill-shot — the 16H gate was leaf-local; enclosing scope bypassed it
16H's observable-liveness gate was computed **leaf-locally**: `observable_use`
inspected only *that* `Simple` node's own `redirs` and its membership in a
`non_last_pipeline_stages` set. +SURE that is unsound, because output-consumption is
a property of the *enclosing* construct, not just the leaf:

- `{ apt-get install -y nginx; } > /tmp/out` then `cat /tmp/out` — the redirect is on
  the **group**, not the `install` leaf. 16H saw a bare leaf (no own redirs, not in a
  pipeline) ⇒ **REPLACED** ⇒ the stub drops the group's stdout ⇒ `/tmp/out` empty ⇒
  `cat` diverges. A converged-and-replaced establish corrupted a downstream read.
- `( apt-get install -y nginx ) | grep -q nginx && echo present` — the **subshell** is
  the non-last pipeline stage; the leaf inside it is not. Same bypass ⇒ REPLACED.

This is the *exact* same class as the original stdout gate, displaced up one syntactic
level. The leaf-local surrogate was a hammer-shaped hole in a scalpel.

### Fix — propagate enclosing output-consumption down to the leaves
Replaced `observable_use` + `non_last_pipeline_stages` (both leaf-local) with a
**top-down `walk_consumed`** (`plan/src/lib.rs`) carrying an `enclosing_consumed`
flag that becomes `true` when an ancestor consumes output, then `OR`s with the leaf's
own redirs at the bottom:

- `Pipeline`: every stage but the last gets `enclosing_consumed = true` (piped into
  the next); the last inherits the pipeline's own context.
- `Subshell`/`Group`: `enclosing_consumed |= has_output_redir(redirs)` — a real
  (non-`/dev/null`) fd-1/2 write on the group flows the body's output to that sink.
- `Simple` leaf: inserted iff `enclosing_consumed || has_output_redir(own redirs)`.
- `If`/`Case`/`AndOr`/`List`/`Script`: recurse, threading the flag unchanged.

The scalpel is preserved: `has_output_redir` keeps `/dev/null` exempt, so
`( apt-get install -y nginx ) > /dev/null` **stays replaced** (new pin
`pins_converged_enclosing_subshell_devnull_replaced`). New specs that now pass:
`spec_converged_enclosing_group_redirect_must_run`,
`spec_converged_enclosing_subshell_pipe_must_run`.

~SUSPECT this top-down walk is also *more correct than 16H even on the leaf-local
cases*, because it is one coherent structural pass instead of two ad-hoc sets that had
to agree.

## 1. ⊤-containment (hole-5 / g-toptop) — deferred in 16H, fixed here
16G/16H left `spec_topcontext_background_leaf_must_run` (`apt-get install -y nginx &`
then `echo done`) `#[ignore]`d: the `&` ⊤-rejects loudly, but `build_plan` never
consulted diagnostics, so the converged install was still REPLACED — an
`inv-top-reject` breach at the plan layer. Fixed cheaply: a `SkipClass::EstablishAmbient`
arm now carries an `if !has_top_successor(cfg, node)` guard; a leaf whose CFG node has
a `Top` successor folds to `Run`. Spec un-ignored, green.

-GUESS this is *slightly* over-refusing in the safe direction: it refuses any leaf
whose node lands immediately before *any* `Top` node, not strictly a leaf whose own
statement is ⊤-contaminated. The precise fix (parser binds `&` to its command so the
⊤ is *inside* the leaf's construct, and the plan checks containment not adjacency) is
deferred — the over-refusal is sound (`kFAIL` for the probe phase: when unsure, don't
replace), just imprecise.

## 2. HOLE#1 (NEW, deferred with a spec) — substs in redirect-targets don't poison
The adversarial surfaced, and I **traced**, a real effect-completeness gap:
`apt-get install -y nginx < "$(apt-get purge nginx)"`. The `$(apt-get purge nginx)`
*runs* (and purges nginx — a `Kill`), but it sits in a **redirect target**, a position
the CFG lowering never descends into. So its `Kill` never enters reaching-defs, never
poisons the `install`'s ambient-ness ⇒ the install is wrongly `EstablishAmbient` ⇒
REPLACED. +SURE it is real; ~SUSPECT it is contrived (nobody writes a purge in a
redirect target), so it is deferred as `#[ignore]`d
`spec_converged_subst_in_redir_target_poisons`. The clean fix is **CFG-lowering
completeness**: lower command-substitutions wherever they appear (redirect targets,
case patterns, assignment RHS, word parts) so their effects always reach the dataflow
— a single principled pass, the right next correctness item, broader than this one
spec.

## 3. Latent / unverified (flagged, not specced)
- **cd-blessing (latent):** `cd` is blessed `Pure` (16H fix B) so it doesn't poison
  ambient-ness. --WONDER whether a relative-path oracle that `cd`s then probes a
  relative fact could diverge between probe and apply; no in-spike divergence exists
  (the hostsim facts are path-agnostic `FactKey`s), so it is un-exercisable here.
- **fd-dup resolution:** `2>&1`, `>&3` are still not resolved (deliberate floor, 16G);
  `> /dev/null 2>&1` stays replaceable. A precision refinement, not a soundness hole.
- **if/case trailing redirect:** `If`/`Case` `NodeKind`s carry **no `redirs` field**
  (only `Simple`/`Subshell`/`Group` do; loops/`while` are ⊤-rejected wholesale). So a
  trailing redirect on a compound (`if …; fi > f`) is a *parser-modeling* question,
  not a `walk_consumed` gap — there is no AST slot for the walk to consult.
  --WONDER whether the parser silently drops such a redirect (latent hole) or
  ⊤-rejects it (safe); unverified, a future trace. The walk already covers every
  construct that *can* hold an output redirect today.

## 4. State (network-free kernel; whole workspace green + clippy-clean)
`core` · `syntax` · `analysis(lattice,solve,cfg,effect)` · `oracle` · `plan` ·
`hostsim` · `cli`. Tests: analysis 18 · cfg 26 · core 5 · hostsim 6 · oracle 8 ·
plan-lib 10 · matrix **20 (+1 deferred = HOLE#1)** · syntax 2+16. Clippy clean.

## 5. Method (held — note 16G §6)
`adversarial-crosscheck` skill, clean-context pair, un-seeded by 16C–16H, no third
optimistic pass. The steer narrowed the *target* to the implemented core and fenced
the parked features, so the round spent itself on the live decision surface rather
than re-discovering known-deferred work. Convergence = trustworthy; the kill-shot was
**convergent** (both agents, independently) — the strongest signal in the method, and
it was a hole I had *just* introduced, which is exactly the value the un-seeded pair
buys. Every "REPLACED today" was produced by tracing the real pipeline, not relayed.

**NOTES INDEX:** …16F observable/replace model · 16G coverage audit + build scope ·
16H gate landed · 16I (this — core attack: enclosing-scope gate + ⊤-containment).
