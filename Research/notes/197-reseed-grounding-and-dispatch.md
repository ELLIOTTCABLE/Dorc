# 197 — Round-19 re-seed: theory grounding, the corrected Half-B model, and the dispatch plan

> Re-seeded orchestrator kickoff (continues 193→196; 196 is the seed). I read R/D/I/K, the
> compiled-probe strawman, charter 191, notes 193–196, 16P/16Q, **SPA ch. 4/5/6/8/9/11/12 in
> full**, 17N, and ANALYZER-NEEDS §C/§M. The SPA cut is a reusable derived file at the **repo-root**
> `Research/sources/B-moller-schwartzbach-SPA-2025-spike2-cut.txt` — **gitignored** alongside the full
> (copyrighted) book, NOT committed, NOT present in the worktree; read it by absolute path. Baseline
> re-verified green this session: `cargo test --workspace` exit 0, `sh e2e/run.sh` 26/26 (F1 honestly
> `xfail`-pinned). AI-authored, confidence-marked. Trust R/D/I/K over this.

## 1. SPA → spike grounding map (what the theory pins, verified against source)

The engine *is* SPA ch. 4–5 (lattices + monotone worklist), and the keystone *is* ch. 11. Confirmed
mappings (so a future agent need not re-derive them):
- **§4.3 constructions** (powerset/flat/product/map/lift) = `analysis::lattice` `Powerset`/`Flat`/
  `Product`/`MapL`. **§5.3/§5.10 worklist** (Simple + Propagation, `dep` = CFG succ/pred) =
  `analysis::solve`. **§5.7 reaching-definitions** (powerset of defs, JOIN over pred, `↓X` kill + gen)
  = `effect::Reach`, the ambient gate (T8) — Dorc keys `(kind,entity,selector)` cells, not assignments.
- **§12.5 (verified, note-193 anchor):** *join (⊔) is always a sound **and complete** abstraction of
  concrete ⊔* — "joining abstract information when branches merge is **never** to blame for precision
  loss." ⇒ the poison wall is a **lattice-shape defect (too few cells), not a merge defect**; the
  selector re-key (more cells) is the *correct* fix, not a better join. (K1 already did this.)
- **§11 strong/weak update = the keystone mechanism.** Field-sensitivity (`[[c.f]]`, `Cell×Field`) =
  the **selector** facet; field-*insensitive* (flat key) = the poison. **Strong update (overwrite) is
  sound only on a provably-unique/singleton cell** (§11.5/11.6; Ex 11.17 shows an unsound strong-update
  on a multi-cell ⇒ wrong result) = `tc-uniqueness` / `EntityRef::Singleton` / `an-strong-weak-update`.
  strain-1/strain-8 (wrong `Singleton` on `install $PKG`) is *exactly* this unsound-strong-update class.
- **§9 IFDS/IDE = the deliberately-NOT-taken substrate**, and §11 says why we can't take it: IFDS needs
  `P(D)` finite **+ all transfers distributive**; gives poly-time full context-sensitivity + function
  summaries + meet-over-valid-paths (and the exploded-supergraph path-edges = provenance) ~free. **But
  the keystone's strong-update is non-distributive** (Ex 11.6: Andersen `*x=y`/`x=*y`), so it breaks
  IFDS's core requirement. So: the **gen/kill ambient gate alone IS IFDS-able** (reaching-defs/taint are
  distributive — §9.1 possibly-uninit "≈ taint analysis"), but the precision keystone is not. That is
  the `an-substrate` decision's theoretical spine, and the `an-distributive-split` (§M) hazard verbatim:
  *"get it wrong and the engine is imprecise **or non-terminating**."* `re-eval-trigger`: IFDS reopens
  only if a seam is built on the distributive sub-layer (seam-prov/seam-interproc *on gen/kill only*).
- **§6 widening = seam-finite.** Finite-height is the Kleene-termination precondition; the recursive
  kind-embedding threatens infinite height. **Depth-bound = simple widening ω** (finite-image, monotone,
  extensive ⇒ Tarski-converges). The spike's `solve` convergence-cap (DP-2, the 435/783-CPU-s hangs) is
  the loud-fail backstop; widening is the *principled* fix for the recursion case.
- **§8 interproc = seam-interproc** (call/after-call/entry/exit + the special call→after-call edge for
  locals). Context-**insensitive** is the `kCONTEXT` default (call-strings `Call^≤k` and functional
  `Context=State` are both the EXPTIME options the redline forbids unless the domain stays flat).
- **§12 = the `kFAIL` frame.** Soundness `α({[P]}) ⊑ [[P]]` (over-approximate) = **apply-side /
  `kFAIL-perform`**; the probe-side (`kFAIL-withhold`) is the *dual* under-approximation. §12.3
  **soundness-testing** (run concretely, check the static result over-approximates) is exactly Dorc's
  DST/differential harness — book-endorsed `kVERIFY-calibrate` ("TypeScript, not Coq").

## 2. The corrected Half-B / probe-projection model (human correction, this session)

My first synthesis overstated it. The corrected, locked-for-this-spike model:
- **Nothing un-oracled is magically lifted.** `compile_probe` emits the **union** of (1) *ordering +
  arguments from the book* and (2) *probe-bodies from oracles' declared `.check`s*. Both legs are
  oracle-anchored. The book gives the call-site (where + with-what); the oracle gives how-to-check-
  read-only. Today `compile_probe` does only a degenerate slice of this — it ships the oracle body per
  ambient fact with the **operand unbound** (`$1` empty; 16P T16 "Half A"). Half B = bind the book's
  operand/args into the **interceptor** (`command -v nginx` → `command__check -v nginx` + the body of
  `command.check()`, exactly the `id__check` strawman) and **subsume the guarded branch** from the
  probe verdict instead of `:`-stubbing it.
- **CFG-preservation (strawman variant B) is deferred, not this round.** Default = variant A (FLAT:
  independent interceptors dispatched concurrently). Variant B (`an-maintain-cfg`) is the rare case: a
  probe with a *probe-correctness precondition* on another fact ("unsafe to run as a probe unless
  `foo#defrocked`"), or the same probe guarding the runtime mutation. Build only if a corpus case forces
  it — and that forcing *is* a strain-log finding.
- **F3 hazard (17N — sharp, hand to gw-1):** a guard's operand is **not** necessarily its body's operand
  (`if ! dpkg -s conflicting_pkg; then apt-get install something`). So subsuming a guarded branch from a
  probe verdict is licensed only when the probed fact actually gates the body — co-reference is
  **may-grade**, never a manufactured instance-link (SF-1: identity is declared, the probe confirms
  *state*, not identity). Getting this wrong is an **under-execute** (priority-1, `kFAIL-perform`).
- **F1 is the same thread from the safe end.** Root cause (`plan/CLAUDE.md` + `tests/observable_matrix.rs`):
  a consumed *status* does **not** block the `ReplaceLicense` (rc-0 vouched by the establishes-contract)
  — sound for a *post-condition* consumer (`install && start`), **unsound for a guard/pre-condition
  consumer** (`if ! command -v`). Not a missing effect *category* (I retracted that framing): it is the
  observable-consumption rule mis-scoped on `Observable::Status`. **Stopgap (lands regardless):** status
  consumed by a *branch* (`if`/`while`/`&&`/`||`/`!`) blocks the license; errexit-consumed status stays
  vouched (so a converged install under `set -e` still elides). **Real fix:** Half B subsumes the guard
  into the probe. Stopgap = the off-ramp floor (run it, value-less); Half B = recover the value.

## 3. The apply-3 may/must subtlety (for gw-3 — verified against §5.8)

§5.8's four-quadrant table: reaching-defs = forward+may (what's instantiated); the never-run machinery
is backward and/or must (T4). apply-3 relevance-reduction has **both orientations at once**, and this is
the load-test of the `May`/`Must` locks (`F-FW3`): **relevance propagates backward-MAY** (live-variables-
shaped — keep anything that *might* affect the dirty/target set, over-approximate), while the **elision-
license is MUST-gated** (elide only *provably*-irrelevant — under-approximate the elidable set; the
one-way `May→Must` coercion forbids promoting). This exactly mirrors apply-2 (may-reaching-defs ambient +
`Must`-converged license). ~SUSPECT a *naïve* "instantiate a backward analysis" that is backward-**may**
only would exercise `Backward` but **not** the `Must`/`BoundedLattice` tower — so gw-3 must deliberately
route the elision-license through `Must` to actually load-test it (else ch-scope is half-satisfied).

## 4. Dispatch plan — five tracks (keystone-first means Half-B-first; the K1 selector re-key is done)

The charter's literal §4 "three seams" framing is partially superseded by 196's own discovery: **Half B
(probe-projection / guard-subsumption) is the now-promoted core** (human-ruled this session, ahead of
seam-interproc). The seams are *where the underpowered substrate strains*; Half B is a *capability* gap —
different axes. Tracks (task-ids in the TaskList):
- **gw-1 (#2) — Half-B probe-projection + F1 stopgap.** The centerpiece. round19 worktree. Owns
  `crates/{oracle,analysis,plan,cli}` + guard/probe e2e cases. `tc-*` escalate to me.
- **gw-4 (#3) — corpus strain-frontier (non-guard surface).** `e2e/`-only, isolated worktree, never
  `crates/**`. Maps shell-env-state / observable-liveness / parser-⊤ / render-fidelity (196 §3),
  exhaustive-on-built / one-xfail-per-cluster. Avoids the guard cluster (gw-1 owns it).
- **gw-3 (#4, blocked by #2) — backward/apply-3 skeleton + `#changed` scaffolding** (§3 above; ch-scope).
- **recursion-smoke (#5, blocked by #2) — depth-bounded recursive kind-embedding** (seam-finite via §6
  widening; human upgraded it modestly from first-to-give to an explicit exploration track).
- **adversarial-crosscheck (#6, blocked by #2) — at the Half-B juncture**, target-rotated (ap-3) onto the
  probe-compiler soundness (F3 under-execute), the harness, keystone-adherence, the seams.

First wave: gw-1 (round19, background) + gw-4 (isolated, background), clean file-split. gw-3/recursion/
adversarial gate on gw-1's shape stabilizing.

## 5. Coverage-matrix skeleton (196 §3 directive — orchestrator owns it; gw-4 fills the non-guard rows)

Surface × the 26 current e2e cases → {covered · gap · out-of-scope}. Rows (from R/D/I/K + 16P ledger +
ANALYZER-NEEDS §B/§C, NOT ANALYZER-NEEDS as gospel):
- **entity-resolution** (`command_effect`): Operand / nullary-Singleton / `$PKG`⇒Opaque / multi⇒Opaque /
  pure-builtin — *covered* (exec-* cases, strain-8 regression executed).
- **ambient gate** (`classify`/`Reach`): lone-establish / same-cell-kill / opaque-neighbour / distinct-
  selectors / poison-wall-dead / detached-fn — *covered*.
- **prove_replaceable**: ambient+Must+Converged+quiet⇒Replace / Diverged⇒Run / consumed-stdout⇒Run /
  /dev/null-exempt / `&`-⊤-contained — *covered*.
- **guard handling (the F1 family)**: `if`/`while`/`&&`/`||`/`!` — *gap, gw-1 owns* (xfail until stopgap;
  then Half-B subsumption cases).
- **shell-execution-environment state** (16P T9 / §B): errexit precise-edges (`!`-pipeline, whole if/while
  cond, `&&`/`||`-left, `|| true`, failing-redirection), scope-containment (`( )`/`$()`/`{ }`),
  `$()`-body-non-leaf, redirection-as-effect (`: >/etc/x`), `& wait` — **gap, gw-4 owns** (almost none
  exercised).
- **observable-liveness enclosing variants** (16P T10): `{ install; } >f`, `( install ) | grep` — **gap,
  gw-4** (only leaf-local tested).
- **render fidelity / leaf-seam** (16P T14): flat vs line-granular, empty-clause `:`-stub, multi-leaf-line
  `LeafId↔AstId` blur — **gap, gw-4**.
- **parser ⊤-triggers** (16P T2 / inv-top-reject): eval, loops, dynamic command names, recursive `$(())`,
  lvalue builtins, background — **gap, gw-4** (`toprejected` hits one; sweep the set).
- **multi-oracle / Seam** (17N): two oracles grounding one kind — *partial* (`two-oracles`).
- **interproc / detached bodies, partial-convergence, volatile** — frontier, mark-don't-over-invest
  (gw-3 / later).
