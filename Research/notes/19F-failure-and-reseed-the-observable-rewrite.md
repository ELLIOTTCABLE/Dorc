# 19F — The failure (an incoherent `Observable`) + the re-seed for a clean rewrite

> Handoff/re-seed note (like `196` was for this round), written near context-exhaustion to preserve the
> failure-analysis and shape the next iteration. The round's CODE hit a representational wall; its **NOTES
> (193–19F) are the deliverable and MUST survive any branch action.** AI-authored, confidence-marked.
> Trust R/D/I/K + `19A §5`/`19B`/`19E` over this.

## 0. TL;DR

The spike carries **three incoherent representations of what is really one thing — a command's
observables** — and that disconnect is the root of the round's recurring confusion *and* the strain-B
under-execute. **Priority-1 for the next iteration (confirmed with the human): a clean rip-and-replace so
the codebase has exactly ONE coherent `Observable`.** Keep the notes + the sound substrate + the
keystone's *cell model* + the 43-case corpus; rip-and-replace the observable/verdict/fold/elision layer.
**Do NOT wipe to `main`** — `main`@357efdd predates the keystone, the corpus, and these notes (it is the
bare round-16 spike); wiping discards real value. Fork from *this* branch (or cherry-pick the keep-set +
notes onto a fresh branch).

## 1. The failure — the incoherent `Observable` (the +SURE root)

Three overlapping-but-separate types name facets of one concept:
- **`analysis::cfg::Observable`** — enum `{Status, Stdout, Stderr, AndOrStatus}`, used *only* as
  **consumption-liveness channels** ("which outputs does this leaf's enclosing context read?"), feeding
  the replace-gate.
- **`core::Verdict{Converged/Diverged/Unknown}`** — the keystone's **fact-cell state** from the probe.
- **`Observed{verdict, rc}`** (build-1) — the verdict with a **bolted-on `Option<rc>`**.

They are three names for facets of **one** thing: a command's **observable-tuple `{effect, status(rc),
stdout, stderr}`** — which (i) the check PREDICTS, (ii) an enclosing context CONSUMES (liveness — which
channels it reads), and (iii) a substitution REPRODUCES. Because they are *separate types*, the rc (a
status-observable) was sourced *beside* the verdict, could go absent, was defaulted to `0`, and silently
under-executed non-conforming establishes (`useradd … || mkdir` — strain-B, `19D`). **The fold direction
(`19A §5`) and the observable-predictor check (`19B`) are RIGHT; the *representation* is the failure.**

## 2. Why it kept getting lost (the meta-failure)

The conceptual clarity lived in the *notes* (`19A §5`: "rc is just an observable; convergence is the
engine's derived effect-state"), but the *code* never unified the three types — so every sub-build
re-introduced the split (the keystone's `Verdict`, build-1's bolted rc, the cfg consumption-enum), and
every explanation/crosscheck had to re-derive the distinction from scratch. **A disconnect in the *types*
defeats clarity in the *prose*.** ⇒ the fix must be in the types, and the invariant must live in
`spike/CLAUDE.md` (loaded every session), not only in a note. (Done this turn: `inv-one-observable`.)

## 3. Priority-1 (confirmed, human): ONE coherent `Observable`

A single `Observable` notion = a command's **output over channels** `{Effect, Status, Stdout, Stderr}`
(extensible). The model, end to end:
- the **check** (oracle `.check()`, the observable-PREDICTOR, `19B`) yields, per channel, either a
  predicted **value** or ⊤ / a loud OOB "can't-predict" — atomically for the command;
- **consumption-liveness** = which channels the enclosing context reads (the existing replace-gate idea,
  but now over the *same* channel set the check predicts);
- **elision/substitution** = replace a command with the cheapest stand-in that REPRODUCES the *consumed*
  channels' predicted values (exact rc, captured stdout, …) — and is licensed only when the **Effect**
  channel predicts *no-mutation*;
- **"convergence" is the DERIVED state of the `Effect` channel** (no-mutation), refined by the ambient
  gate for in-script staleness — **not** a separate probe-reported `Verdict`, and **not** the check's
  concern. The keystone's `Verdict{Converged}` folds into the `Effect` channel's predicted value.

No separate `Verdict` / `Observed{rc}` / consumption-only-`Observable` triple. One type, one flow.

## 4. Keep vs wipe (the human's direct question)

**KEEP (sound; re-deriving wastes effort):**
- the substrate: `syntax` (lexer/parser/ast), `analysis::lattice`, `analysis::solve`, `analysis::cfg`
  construction (incl. the precise errexit pass), `core` newtypes/`Carrier`/`Interner`, `hostsim` (the DST
  host + `kFAIL-withhold` monitor), the `cli` round-trip, and `e2e/run.sh` (the BLESS/xfail/exec gates).
- the **43-case e2e corpus** — much of the round's value (`195`/`199` + the wave cases); the test-base for
  the rewrite.
- the keystone's **cell model**: `FactKey{kind, entity, selector}` + the ambient gate (reaching-defs over
  the effect-map). This is the right direction; only its `Verdict` *representation* folds into the unified
  `Effect` channel (a keep-with-refactor, not a rip).

**RIP-AND-REPLACE (the disconnect):** `cfg::Observable`, `core::Verdict`, `Observed`, `plan::fold`,
`prove_replaceable`'s gate, the rc machinery, and the `cli`/`hostsim` rc-sourcing — unify into the one
`Observable`.

**On wiping to `main`:** don't. `main`@357efdd lacks the keystone, the corpus, and notes 193–19F. The
disconnect is localized to the elision/observable *layer*, not the substrate, so a targeted rewrite beats
a from-scratch restart. **Whatever you decide, preserve notes 193–19F** (they are the round's deliverable;
cherry-pick them wherever the rewrite lands). The committed code (keystone, fold, rc-fix) is a fine
*disposable record* of the exploration even if the layer is rewritten.

## 5. The replacement-prompt (the next iteration's brief — from "rip and replace, not a half-fix")

Shape the next re-seed prompt around:
- **Goal:** rip out the three-concept observable mess (§1) and replace it with the **one coherent
  `Observable`** (§3). Not a patch — a clean re-key of the elision/observable layer, the keystone's cell
  model retained (its `Verdict` folded into `Effect`).
- **The check is the observable-PREDICTOR** producing the atomic tuple (or per-channel ⊤/OOB-fail), full
  command args (`C-1`), command-keyed (`19A §5`/`19B`). Convergence is derived (`Effect`=no-mutation).
- **DISTRUST, explicitly, in the current impl:** `core::Verdict` and `Observed{verdict, rc}` (the bolted
  split); the `cli`/`hostsim` rc-*default* history (it fabricated `rc=0` — `19D`); the **masking tests**
  (`andor-rc-vouch-wrong`'s hand-injected `rc=9`, the promoted `…substitutes_exact_rc` test, `fold-oror`'s
  added `rc=0`) — they pass by *declaring* an rc the model should *predict*; treat them as suspect until
  re-grounded.
- **Encode the invariant in `spike/CLAUDE.md` FIRST** (`inv-one-observable`, added this turn) so the
  rewrite can't re-fragment it.
- **Use subagents** (control context); the orchestrator stays clear and adjudicates the `tc-*` calls.
- **Process keepers from this round:** crosscheck *with the full neutral+adversarial pair* (the comparison
  is the signal — a single pass + a seeded self-review missed strain-B until the adversarial pass drove it
  through the CLI); isolated worktrees check out a STALE base and can't `git commit` here (build agents run
  in the worktree; read-only crosscheck agents isolate fine); `SendMessage` is unavailable to course-correct.

## 6. The acceptance test-suite (passes ONLY if the model is coherent — the anti-masking contract)

The rewrite is "right" iff these pass with the observable VALUES flowing from the check (the predictor),
end-to-end — **no test may hand-inject an rc the check itself should predict** (that injection *is* the
masking that hid strain-B):
- `useradd deploy || mkdir …` with `useradd` a **non-conforming** establish (rc 9 when the user exists):
  `mkdir` **RUNS** — because the *check* predicts `useradd`'s rc=9, not because a fixture declared it. (The
  strain-B redline; the canonical regression.)
- `apt-get install … && systemctl start …` (conforming, rc 0 when installed): the install elides, `start`
  runs — the check predicts rc=0.
- `command -v X || install X` (the idempotency idiom): elides when present — the check predicts the
  guard's rc; the read-only guard is the easy predictor.
- `set -e; <converged non-conforming establish>; needed_cmd`: define + pin the correct behavior — the
  coherent model lets the check's predicted rc drive `set -e` (today this is a documented priority-2
  over-execute, `19E`; the unified model should make it fall out of the `Effect`/`Status` channels).
- **anti-masking meta-test:** a check that returns an **OOB "can't-predict"** for a command ⇒ that command
  RUNS (⊤ ⇒ `kFAIL-perform`); and removing a check's rc prediction must flip its dependent case to *run*,
  never silently elide.
