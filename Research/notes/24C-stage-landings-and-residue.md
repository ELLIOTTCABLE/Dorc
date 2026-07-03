# 24C — round-24 stage landings + the residue ledger

AI-authored (Fable conductor), 2026-07-03, round 24. Build-evidence + residue, accreted
per stage as it merges (24A = the law; 24B = the test architecture; THIS = what landed and
what it leaves owed). Confidence-marked. Append per stage; never rewrite a landed section.

## Stage 2 — the frame-rule machine (LANDED 2026-07-03; the golden hill lit)

Merged to `ai/spike3-r23` (`5c7bae6`/`e105cd7`/`2465ed0`): the `touches()` at-most footprint
lift + the survival wall walk + `--trust-footprints` + attribution. Green: 549 unit tests,
143 e2e (9 standing guard23 xfails), all four gates. **The load-bearing number: on the
IDENTICAL book shape (`modeled-wall` ≡ `survive-simple`), post-wall elision went 0 → 1 under
the flag; unflagged stays 0.** A converged line now elides past a *running* wall, and the
differential proves it safe. The charter's three questions answered with evidence: disjointness
FIRES both directions; footprints are hand-authorable (one sharp pinch — §resid-argparse);
elisions stay correct (run-set differential + the frame rule, NOT gate-6, which attributes but
never judges licenses).

**Conductor review verdict: SOUND as designed.** Traced the four walk properties (survival-test-
then-wall-contribute ordering; a surviving elision casts no shadow; a demoted mutator flips to
Run and walls; `total_wall` is sticky so a later footprinted wall can't rescue what a bare wall
poisoned). The four TC type-contracts are genuinely realized (verified in `plan/src/survival.rs`:
private fields, sole constructors, no `From<effect>` into `Footprint`, `Option` not `bool`). The
**naming-and-docs gate PASSES** (rul24-overtype addendum): every compiler-invisible invariant
rides in prose — `Footprint` states "NO constructor taking establish-effects (the 233 sin must
not compile)", `disjoint` "argument ORDER is load-bearing and type-enforced", `SurvivalWitness`
"SURVIVAL IS NOT ADEQUACY" (twice), `TrustedFootprints` "data-absence, never a bool". Artifact
bytes stay byte-identical (attribution attached via `with_survival` to `Derivation`, exempt in
the erasability gate — rec-1 honored).

### The residue (mode-gated dangers this stage ships open — none are bugs; all are the honest boundary)

- **resid-aliasing (PRIMARY; the one dangerous cell, fails toward UNDER-execute).** `disjoint`
  is token-equality on interned `(kind, entity)`. Two entity tokens naming the SAME real referent
  (`nginx`/`nginx-full` via provides; symlinked paths) come up wrongly-disjoint ⇒ wrong-survival ⇒
  under-execute. This is 23M's "synonym/coherence" cell / 23N §6's within-kind observable aliasing
  = **must-not-alias-or-wall / dynamic points-to**, explicitly DEFERRED to **Stage 5**. Correct as
  designed (the code cites 23N §5 "sound modulo aliasing") — but it is a SILENT under-execute
  behind the flag, and per rul24-mode-gate + PRICE-the-residue it **MUST be professed at the
  horizon** in those words. The `strawman24-crosskind-residue` case documents the cross-KIND
  escape; the within-kind ALIASING residue is sharper and is NOT yet disclosed anywhere user-facing.
  OWED: a horizon-profession line (human voice / the eventual horizon doc); Stage-5 closes the
  mechanism. ~SUSPECT the chronology net (#9) can quantify the bite-rate with a lying/aliasing
  generator — optional Stage-5 preview.
- **resid-kill-coherence (narrow under-execute).** Establish-walls get the at-least ⊆ at-most
  coherence check; **kill-walls skip it** (no single establish cell available). A drifted kill
  `touches()` is therefore unchecked ⇒ a too-narrow kill footprint ⇒ a downstream fact on the
  really-killed entity wrongly survives. Builder-flagged; **Stage 3/4 threads the killed fact**
  to close it. Narrow + mode-gated.
- **resid-argparse-drift (CONTAINED — and the containment is a design win worth keeping).**
  `predict()` and `touches()` are separate functions sharing an argparse; drift silently
  mis-resolves the footprint entity (the builder's unit test caught a real instance — `-y`
  resolved instead of `nginx`). For establish-walls this is **contained by the coherence check**:
  a drifted footprint fails at-least ⊆ at-most ⇒ loud `footprint-incoherent` ⇒ walls (fail-safe,
  not wrong-survive). **Stage 4 (host-executed `touches()`) dissolves it entirely** — the tool
  emits its own footprint, no second argparse to drift. A concrete argument for derivation over
  authored footprints, and the coherence check earns its keep as the interim net.

### Handoffs (verified from the builder report + code)

- Stage 3 shares the plan crate + the why-lens (now two `why:` lanes — run-cause and
  survives-attribution; the guard tier adds a third). `DispositionCounts`' exhaustive match still
  forces wiring the `guard` bucket. A `Disposition::Guard` needs the same erasability-exempt
  classification `Derivation.survival` got.
- Stage 4 turns `evaluate_touches` (static) into host-executed (`dpkg -L`); the `kind:entity`
  parse + interning reuses verbatim; resid-argparse-drift vanishes; the coherence check + survival
  walk are unchanged, only the footprint SOURCE moves.
- `core/tests/diag_tidy.rs`: `footprint-incoherent` added to the legacy allow-list, tc-flagged
  pending the typed-diagnostic-spine migration — a small owed cleanup, not a blocker.
