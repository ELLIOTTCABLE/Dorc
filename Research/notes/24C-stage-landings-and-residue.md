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

## Stage 2b — the chronology net (LANDED 2026-07-03; the DST net has teeth)

Merged (`0455178`/`1ecba50`/`d018bd3`): a NEW `dorc-sweep` crate (24B flavour C) — in-memory
seeded elision-soundness DST. 3000 seeds in the default `cargo test` (~1.3s), `SWEEP_SEEDS=n`
for depth (100k/15.6s). Green: 25 suites, 145 e2e (2 state-bearing tie-downs), all gates.
**Conductor review verdict: SOUND, strong work.** Verified the two load-bearing claims in code:
(1) `TrueEffect`-isolation is enforced BY SIGNATURE — `run_kernel(&DeclaredScenario, &Host,
flag_on, &mut Interner)` has no `GroundTruth` parameter, so the analyzer physically cannot see a
command's true effect; (2) the attribution-under-lies assertion is strong AND non-vacuous
(`tests/sweep.rs` asserts `lying_divergences > 0`, so the branch can never greenwash — the fc-5
discipline applied to the assertion itself).

**The net has teeth (builder planted three bugs, all caught + reverted):** a diverged-elides bug
→ honest end-state RED (seed 22); a broken `total_wall` demotion → lying-attribution RED (witness
named the wrong leaf, seed 11); a `HashMap`-order fingerprint → determinism-guard RED (seed 0).

**Two conductor-surfaced findings (my domain — cross-cutting):**
- **find-lcg-thinning (a REAL pre-existing bug the sweep exposed; fc-5 made concrete).**
  `hostsim::Lcg::chance(1,2)` uses `% 2` on an odd-multiplier LCG ⇒ the low bit is periodic ⇒
  consecutive coins CORRELATE. The builder hit this in the sweep (it silently erased the entire
  MissConverged+Lying topology cell — 0 lying divergences until fixed) and fixed the SWEEP's draws
  via a high-bit `below()`. BUT it is UNFIXED in `Host::seeded` (`hostsim/lib.rs:199`,
  `.filter(|_| rng.chance(1,2))`) — so `Host::seeded`'s "random ½-subset" is a PATTERNED,
  internally-correlated subset, and **every existing in-memory DST test that loops `Host::seeded`
  over seeds has been exploring a THINNED slice of the 2^N initial-state space** (inherit humility
  toward those "green across N seeds" results — `128` rg-1). Blast radius bounded: the frozen
  subprocess `differential.rs` has its OWN `Rng` whose `chance` already routes through the high-bit
  `below()` (the "21D triage" fix), so it is SAFE — only `Host::seeded` is affected. NOT a sweep
  blocker (the sweep uses the correct path). Cheap fix owed (route `Host::seeded` through `below`;
  re-run its consumers, confirm still-green — a state-CHANGE, do it consciously). → follow-up task.
- **find-net-covers-what (structural; do not lose).** The builder's ~SUSPECT is correct and
  sharp: in an HONEST world a wall that could invalidate the victim's cell makes the victim
  non-ambient (`EstablishWritten`) ⇒ never a survival candidate ⇒ the frame rule is provably
  sound. So the honest end-state net catches CORE wrong-elisions (eliding a diverged command) but
  is BLIND to survival-tier bugs BY CONSTRUCTION — those are reachable ONLY via lying scenarios,
  and the lying-attribution net is what carries them. Consequence: the survival tier's entire test
  coverage rests on the (verified strong, non-vacuous) lying-attribution net; the lying scenarios
  are load-bearing, never decorative. Keep this when reasoning about what "the sweep is green"
  buys.

**Tie-down faithfulness (partial — honest residual).** The 2 state-bearing tie-downs agree with
real dash+emitter (gate-1 probe parity from a file-backed world; gate-6 elision attribution), but
run.sh separates probe from apply and the books have no state-reading control flow, so a true
bare-state-vs-apply-state end-state DIFF isn't exercised by real dash — the in-memory sweep IS
that differential; a real-dash end-state-diff harness was deliberately NOT added (freezing run.sh
per the testing refinement). ~SUSPECT a future `e2e/` end-state-diff gate is the clean closure;
deferred.

**Stage-6 adequacy substrate is READY (builder-confirmed + verified):** a converged≠no-op world
is a `Host` whose mutator's `TrueEffect` exceeds what its probe reports converged — the SAME
declared/ground-truth split already built, so the honesty enum generalizes {Honest, Lying-footprint}
→ {…, Lying-adequacy}, keyed on the claim-tier once Stage 3 lands it. The dangerous residual the
whole round exists to measure now has its measuring instrument.
