# 196 — Round-19 handoff + the Half-B (guard-subsumption) insight (the seed for the next round)

> Written deliberately near a context-compaction boundary to preserve the round's state and its
> single most valuable output (§2) at full fidelity. **This doubles as the seed for a re-seeded
> orchestrator round** — read this first, then `notes/193`/`194`/`195`, then the human docs
> (README/DESIGN/IMPLEMENTATION/KNOBS — the *only* human-voice root docs) and `plans/191`. AI-authored,
> confidence-marked. Trust R/D/I/K over everything here.
>
> **Doc-trust correction (human, this round):** `ANALYZER-NEEDS.md` is **agent-written**, not human
> voice. Do NOT treat it as canonical. The authoritative analyzer surface is **R/D/I/K + the built code**
> (the `16P` built-ledger, the actual `spike/crates/**` source, the `spike/CLAUDE.md` `inv-*` list).
> ANALYZER-NEEDS is a convenient but fallible *index* to cross-check, nothing more.

## 0. Working state (git)

`.claude/worktrees/round19` on `ai/round19-keystone` (off `main` @ `357efdd`). ~14 commits; `main`
untouched; never pushed. Tree clean except untracked `mkmocks.sh` (dev-scratch). A background `hk`
subagent (auto-fmt pre-commit) was in flight at write time — confirm it landed + the tree is clean
before more commits. **Commit discipline:** the `git-deny` hook allows a commit only when the shell's
cwd is rooted on the `ai/*` worktree branch — so `cd <worktree>` (standalone) *then* `git commit` (a
compound `cd && commit` fails: the hook reads the pre-cd cwd). Subagents self-commit via that technique.

## 1. What round-19 built (the keystone is real; two priority-1 wrong-elisions found)

- **K1+K2 — the entity-algebra keystone** (`notes/193`): the flat `FactKey{kind,entity}` became
  `core::FactKey{ kind, entity: EntityRef(Operand|Singleton), selector: SelectorId }`, threaded through
  the whole engine. Killed the poison wall (`apt-get update` → distinct `package-index#fresh` cell).
  Workspace green; a runnable `ap-2` harness (`sh -n` + execute-under-mocks).
- **K3 — adversarial-crosscheck** (`notes/193` strain-8): validated the keystone is real (not
  scaffolding), found **strain-8** — a priority-1 wrong-elision (the `EntityRef::Singleton` inference
  fired on `install $PKG`, a non-literal operand) — **fixed**. Then hardened structurally:
  `resolve_entity` (total, ⊤-contagious — `notes/194`: the lesson is "⊤-contagion must be *structural*",
  a corollary of `inv-top-reject`, load-bearing as the entity-algebra grows more ⊤-sources).
- **Corpus** (`notes/195`): 26 e2e cases (was 9), 13 *executed* under `PATH=mocks-only` (inert shims;
  safe by construction). Found **F1** — a *second* priority-1 wrong-elision (guard-status elision) —
  pinned `xfail`, NOT fixed (it's the `tc-*` call below). Headline: a *bare-mutation* pi-webhost elides
  6 mutations, verified by running. **Note it had to be bare-mutation — no guards — precisely because
  guards trigger F1 (§2).**

## 2. THE INSIGHT — Half B (guard-subsumption) is the missing core, and F1 is the spike screaming it

+SURE this is the round's most important finding; it reframes "what the spike is for." The human surfaced
it; this section is the durable version.

**"Probe → converge" has two halves; the spike built one.**
- **Half A (built):** `compile_probe` ships the *oracle's* read-only fact-probes (per ambient establish);
  `build_plan` elides a **mutator** when its fact is converged. The model knows exactly one operation:
  *elide a converged mutator.* The probe is *simulated* (verdicts injected; `16P` T16 — oracle bodies with
  `$1` unbound, a stand-in).
- **Half B (UNBUILT, the deferred core):** the probe *projection* — compile the probe **from the book's
  own CFG**, lifting the book's own read-only **checks/guards** (`command -v nginx`, `dpkg -s`) as
  read-only *interceptors* run in the probe, and **subsume the guarded branch** from the probe's answer.
  This is exactly `notes/17x-strawmen/adversarial/compiled-probe.straw.sh` (`R2-PROBEGATE`). The guard is
  never "elided" — it's *executed read-only* to gather state. **This is the half that handles the dominant
  real idiom (check-then-execute guards — DESIGN's whole pitch), and it is unbuilt.**

**Why the spike "drops a guard at all" (F1):** the engine has **no guard/check category.** `command_effect`
emits only `Pure / Establishes / Kills / Opaque`. A read-only check is therefore either un-oracled →
`Opaque → Reach::Top → MustRun` (runs, but poisons — strain-5) *or* oracled-as-`establish` → treated as a
*mutator* → `EstablishAmbient` → **`:`-stubbed when converged** (F1: forces the branch; block-`if` orphans
`then` — the `ap-2` gate catches that shape; the inline shape is render-masked). It is *not* sane that it
drops a guard — that's the bug. The correct behaviors (RESPECT the guard = run it, no elision; or SUBSUME
it = lift into the probe) are **both unbuilt**.

**The entanglement (the sharp part):** *the poison wall was accidentally protecting guards.* Pre-keystone,
every guard was un-oracled ⇒ ⊤ ⇒ `MustRun` (safe). The keystone's entire job is to remove that ⊤-forcing so
things become elidable — and the instant it does, it exposes that the elision model has no guard-handling.
So "the keystone works" and "guards are now wrongly elidable" are the **same event**; Half A and Half B
cannot be cleanly separated. strain-8 was a *new* wrong-elision the re-key introduced (fixed); **F1 is a
*pre-existing* one the keystone *exposed*.** ~SUSPECT durable law: *every increment of elision-power
surfaces latent wrong-elisions the poison was masking.*

**Verdict + recommendation (human concurs):** Half B (probe-projection / guard-subsumption) is **core to
the value-prop, not a refinement**, and should be **promoted — arguably ahead of seam-interproc** — because
without it the keystone's elision only reaches *unguarded* books (the easy slice), not the guarded common
case. The keystone-first sequencing was still correct (genuine prerequisite — nothing elided before it),
but the deferral's cost is now legible. F1 has a **two-layer response**: (i) *stopgap* — branch-consumed
`Observable::Status` blocks the `ReplaceLicense` (like consumed stdout/stderr), distinct from
errexit-consumed status (which stays vouched by the establishes-contract) ⇒ Dorc *runs* guards, safe but
value-less (the off-ramp); (ii) *real* — build Half B and subsume the guard. The corpus question ("is it
rich enough?") and this question are the same from two sides: **the corpus is thin on guarded books because
the engine can't do them yet.**

## 3. Corpus-enrichment design (directive #2 cont'd — for the next round to execute)

Goal (human's "rich test-base = spike value" lesson, refined): the corpus should **map the strain-frontier**
(charter's "what strains and where"), not just happy-paths — a broad, grounded, **mostly-xfail** corpus is
the deliverable *and* the anti-self-confirmation guard. Highest-value cells: **built-but-maybe-wrong** (where
strain-8 and F1 hid) + **interactions**; lowest-value: blanket unbuilt→xfail (they just restate status).
Discipline: **exhaustive on the built surface, one-xfail-per-cluster on the unbuilt frontier.**

**The surface to cover — grounded in R/D/I/K + the `16P` ledger + source (NOT ANALYZER-NEEDS as gospel):**
the gaps the current 26 cases miss, roughly:
- **shell-execution-environment state** (`16P` T9 / DESIGN; the "model-or-be-silently-unsound" cluster):
  errexit precise-edges (pruned: `!`-pipeline, whole `if`/`while` cond, `&&`/`||`-left, `|| true`; extended:
  failing redirection), scope-containment (`( )`/`$()`/`{ }` — env/var/cwd don't escape, FS does),
  `$()`-body-as-non-leaf (`is_expansion_internal`), redirection-as-effect (`: >/etc/x`), `& wait`/background.
  Almost none exercised.
- **observable-liveness enclosing variants** (`16P` T10 / 16I — the spike-1 *kill-shot*): `{ install; } >f`,
  `( install ) | grep` — consumption is a property of the *enclosing* construct; `/dev/null` exempt. Only
  the leaf-local case is tested.
- **render fidelity / leaf-seam** (`16P` T14): `render_sh`-flat vs `render_apply`-line-granular; the
  empty-clause hazard (the `:`-stub fix); multi-leaf lines blurring `LeafId↔AstId`.
- **parser ⊤-triggers** (`16P` T2 / `inv-top-reject`): eval, loops, dynamic command names, recursive
  `$(( ))`, lvalue builtins, background — `toprejected` hits one; sweep the set.
- **multi-oracle / the Seam** (`17N`): two oracles grounding one kind; cross-oracle coherence.
- **guard handling** (§2): the F1 family — guards in `if`/`while`/`&&`/`||`/`!` — the whole class, pinned
  xfail until Half B / the stopgap lands.
- (interproc / detached bodies = the seam-interproc work; partial-convergence `install a b c`;
  volatile/transient — mark as frontier, don't over-invest.)

**Execution plan (for the next round, kept controlled after the last subagent's scope-creep):** (1) the
orchestrator writes the coverage matrix itself (surface × the 26 cases → covered/gap/out-of-scope) so the
design stays grounded; (2) a tightly-scoped subagent writes e2e cases for the gaps — **`e2e/` only, never
`crates/**` (no engine edits), via the safe mock harness, never executing a raw strawman**; (3) an
adversarial *un-seeded* pair audits the matrix + corpus for blind-spots / what's mis-marked "covered" (the
`ap-3` target-rotation onto *coverage*). The whole thing is the frontier-map deliverable.

## 4. Open decisions handed forward (for the human / the re-seeded orchestrator)

- **next-phase order:** §2 argues **probe-projection / guard-subsumption (Half B) ahead of seam-interproc.**
  The human raised it; not yet ruled. (seam-interproc = un-⊤ detached function bodies — `q1-interproc`,
  charter §4; still wanted, but secondary to Half B if §2 holds.)
- **F1:** fix the stopgap now (branch-status block — safe floor) regardless; build Half B as the real fix.
  Currently `xfail`-pinned in the corpus (honest).
- **strain-1 / `tc-uniqueness`:** `Singleton`-ness is still *analyzer-inferred*; the principled fix is an
  oracle-*declared* cardinality bit (`R-strongweak`/T5), which also subsumes the inference.
- the richness phases (R-typestate inc-7, R-strongweak, R-recursion-smoke/`seam-finite`, R-backward/`ch-scope`)
  remain pending; the three seams are still **floor-only / not pressed** (K3 finding).
- **probe-execution / `kFAIL-withhold`:** the corpus deliberately did NOT execute the probe (it's the
  $1-unbound stub; `an-withhold-sandbox` unbuilt) — `notes/195` C-4. The real probe-plan-builder is `F-FW3`.

## 5. Process notes (durable, for any orchestrator)

- The `/adversarial-crosscheck` (`kp-1`) paid off twice this round (strain-8 sharpening; the corpus's F1) —
  un-seeded, target-rotated (`ap-3`). Keep using it at junctures, aimed off-core (harness, coverage,
  charter-adherence), not only at soundness.
- **Subagent safety (human flagged twice):** fixtures/strawmen contain real mutators; *never execute a raw
  strawman*; executed cases use inert mocks under `PATH=mocks-only`. A subagent that "explores" by running a
  fixture is the hazard. Propagate the SAFETY block + this verbatim.
- Subagents self-commit in stages (rich bisectable history) — but a corpus subagent scope-crept into a
  workspace-wide `cargo fmt` (committed separately as `(AI fmt)`) + left it uncommitted; review subagent
  *diffs*, not just reports. The `hk` auto-fmt pre-commit hook (being set up) prevents the stray-fmt recur.
