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

## Stage 3 — the guard tier + claim-tier algebra (PARTIAL 2026-07-03; 3 of 4 pieces landed)

The builder made a disciplined partial-stop: 3 clean green pieces landed
(`3726f3b`/`fd40349`/`8fb80e6` → merged `b047e16`), the 4th (mint-wiring + elide-weld) handed off
because it is large, cross-cutting, and coupled to corpus-wide churn that breaks the tree if
partial. Green: 25 suites, 145 e2e (9 guard23 XFAILs intact — no guard fires yet), all gates.
Landed: (1) the **claim-tier trust algebra** (`core/src/claim.rs`); (2) the **verdict-function
lift** (`is_converged`/`is_diverged`, `oracle/src/verdict.rs`); (3) the **guard tier
type-architecture** (`Disposition::Guard`, `GuardLicense`, the emitter — minted in tests only).

**Conductor review of the FOUNDATION (the claim-tier algebra — the round's earmarked-reviewable
decision): VERIFIED SOUND.** All four unrepresentability properties are genuine compile-errors,
mechanism-checked in `claim.rs`: TC-tier-1 `demote` is the sole tier transition (no inverse fn ⇒
upgrade un-spellable); TC-tier-2 the phantom tier makes `Fact<_>`/`Judgment<_>` non-unifiable at a
mint signature; TC-tier-3 `observation()` is `FactTier`-only (a judgment has no fact-plane exit, no
`From<vouch>` anywhere); TC-tier-4 `Rung` sits inside `Vouched<P>` (judgment-only payload),
reserving the ladder seam as an ADD-not-resign. Sealed trait blocks a 4th tier; the honest-bound
line is verbatim. Naming-and-docs gate PASSES. **NB (never-vouch discipline): this is conductor
process-evidence, NOT proof — the foundation stays earmarked for the human's own eyes (it is the
base the whole guard tier + corpus churn build on).**

**strain-classify-coupling (builder's design discovery; corrects a 24D under-spec — +SURE,
`--debug-argv`-verified).** The guard tier is NOT a localized plan change. Past-wall sites are
`EstablishWritten` (an opaque `hork` poisons them at classify time), so they **skip probes entirely
at HEAD** — they are NOT `EstablishAmbient`-walled `Replace`s. So the naive "convert a walled
Replace→Guard" model is WRONG: the guard mint requires `compile_probe` to SHIP probes for vouched
`EstablishWritten` sites, plus a `disposition_for` guard-arm for them — the guard reaches into the
classify/probe boundary, not just the wall-walk. 24D §2/§3 under-specified this; the builder's
hand-off is the corrected spec.

**The A/B split (conductor decision, human asleep — the remainder is two risk-classes):**
- **Part A — make guards fire (DISPATCHED as the continuation; additive, foundation-validating).**
  The mint-wiring: cli lifts verdict-sets + `evaluate_verdict` per site → a `Judgment<VerdictVouch>`
  map threaded into `build_plan_walled` (ALWAYS-ON — guards are the un-flagged baseline, NOT
  `--trust-footprints`-gated); `compile_probe` ships probes for vouched `EstablishWritten` sites;
  the guard mint (`EstablishWritten`+vouch+converged ⇒ `Disposition::Guard`); the guard23 fixtures
  gain verdict functions; gate-6 widening + `cf-6`. The 9 `guard23-*` XFAILs become PROMOTABLE —
  the builder PROPOSES (diffs), the conductor inspects + authorizes (promotion discipline). No
  corpus churn beyond the 9 pins. This VALIDATES the foundation by exercising it end-to-end.
- **Part B — the elide-weld (HELD for the human's foundation go/no-go).** Demanding a
  `Judgment<VerdictVouch>` on `prove_replaceable`'s `EstablishAmbient` arm (closing the HEAD
  vouchless-elide gap) requires churning EVERY converged oracle fixture to gain a verdict function
  + re-goldening — the expensive-if-the-foundation-is-wrong change. Held deliberately: it is the
  corpus-wide churn the "type-architecture is reviewable" earmark exists to gate. Un-hold on the
  human's nod to the foundation (24D + `claim.rs`).

### Part A LANDED (2026-07-03): the guard tier FIRES — 9/9 guard23 promoted

Merged (`86999d0`/`db7f0c8`/`acaeca3`): the mint-wiring — `Vouches` map (cli lifts verdict-sets
per site, always-on = un-flagged baseline), `compile_probe` ships probes for vouched
`EstablishWritten` sites (closes strain-classify-coupling), the guard mint
(`EstablishWritten`+reached-vouch+converged ⇒ `Disposition::Guard`), the `( check ) || <original>`
emitter + `guard_preamble`, the guard why-lane, gate-6 widened + `cf-6`. **All 9 `guard23-*` XFAILs
PROMOTED** (conductor-inspected the renders: the ternary map is clean — pre-wall elide / opaque-wall
run / past-wall-vouched-converged GUARD / past-wall-diverged run; the `( )` subshell isolates BOTH
the var-namespace clobber AND the `set -u` crash; the redirect line stays bare RUN with its loud
`expected-diagnostics: guard` refusal). 145/145 e2e, all gates. **The guard half of the two-halves
doctrine is now REAL and demonstrated** — Dorc produces `( oracle-check ) || command` guards on a
real book, past a real wall.

**Two emitter shape-law bugs found + fixed end-to-end (the first live exercise of the emitter, +SURE):**
(a) the span-edit provenance comment double-commented guards; (b) **the emitter refused only
heredocs, not non-`/dev/null` output redirects — a guard would have SUPPRESSED a `>>log` file
side-effect on a converged pass** (a real correctness bug, now a redirect refuse-home). This is
exactly the "build to surface where it breaks" payoff.

**find-return-vouches (LATENT SOUNDNESS GAP — conductor priority; touches USER_STORY).** The
verdict-fn lift treats a path that "reaches a command" as a vouch, but a bare `*) return 2 ;;`
parses AS a command, reaches `run_command`, and **wrongly VOUCHES** — where the author meant a
DECLINE (rc ≥2 = confused ⇒ run, rul-rc-partition; the hz-refusepath fence). In the GUARD tier this
is runtime-contained (a declined path's probe returns ≥2 = can't-tell ⇒ mint blocked; and even a
spurious `( check )` returns non-zero ⇒ `||` falls through ⇒ command runs) — so no under-execute
TODAY. **But it bites Part B**: once a vouch licenses ELISION (no runtime net), a decline-path
wrongly read as a vouch could wrongly ELIDE. AND **`*) return 2 ;;` is the canonical decline idiom I
put in USER_STORY stages 3/4** (human-vibed) — so the human-facing doc's decline idiom is exactly
the one the lift mis-reads. Fix (settled-law, not a design-open): model `return N`/`false`/`:` as
non-vouching declines in the verdict lift; this also SUBSUMES the builder's `tc-verdict-lift-warn`
workaround (⊤-reject downgraded to warning so the `return 2` floors don't trip gate-3 — unneeded
once return-is-decline is modeled). → task #12, folded into the Part B brief (Part B raises its
stakes, so it must land there or before).

**Coverage gaps (not defects, ~SUSPECT):** the declared-dual `is_diverged` sense-flip renders +
unit-tests but no guard23 case fires it end-to-end (the one `is_diverged` fixture has an
`EstablishAmbient`/`guard=0` site); the sweep mints no guard scenarios yet
(`tc-sweep-guard-scenarios`). Both want a case/scenario later. The strawman yardstick `guard=`
stays 0 (strawman oracles have no verdict functions — Part B territory); guard firing is measured
on the guard23 pin-set (`guard=1` each; flagship `sites=4 elide=1 guard=1 run=2`).

## Stage 3 Part B — the elide-weld (LANDED 2026-07-04; the vouchless-elide gap CLOSED)

Merged (`3fffb65`→`ff0cb4e`, cherry-picked; shared tip `26001ef`). **A full skip now DEMANDS a
reached vouch** — `prove_replaceable`'s `EstablishAmbient` arm consumes an `Option<ByVouch<VerdictVouch>>`
by value (the consumption IS the tier check; a `ByObservation`/`BySilence` can't inhabit it). No
vouch ⇒ run (`kFAIL-perform`). Rode along in the same pass (24D §6): the **rename**
(`Fact`→`ByObservation`, `Judgment`→`ByVouch`, add `BySilence`; `FactTier`→`ObservationTier`,
`JudgmentTier`→`VouchTier`; `Vouched`→`VouchAndRung`; minters `measured`→`observed`,
`authored`→`vouched`); the **self-framing doc honesty-fix** (survival.rs — the overstated language
was ONLY there, not claim.rs); the **`return-vouches` fix** (#12 — `return N`/`false`/`:`/`true`
now DECLINE, `tc-verdict-return` softening reverted); the **per-crate critical-types + when-blocked
doc lines**. **50 oracle fixtures gained an `is_converged()`.** Conductor review: **yardstick came
back FLAT at 0.27 (no drop) — the churn was complete, no converged site was secretly skipping**;
`build_plan(empty vouches)` can no longer elide, so every DST harness (sweep/coverage/hostsim/plan
tests) needed vouches threaded (`build_vouches` extracted to `dorc_plan` as the shared home — the
Stage-4/5 seam). All 145 e2e + gates green on a FRESH build. (A 3-case "failure" in first
verification was a STALE-BINARY artifact — `cargo test` hadn't rebuilt the `dorc` bin the e2e
harness uses; a `cargo build` fixed it. Lesson for the next conductor: force a `cargo build
--workspace` before trusting an e2e run after a cherry-pick.)

**The dead pin (human-ruled retirement, standalone attributed commit — in flight).**
`guard23-vouch-inert-pair` asserted the OLD law "a vouch never changes which sites elide," which the
weld deliberately OVERTURNED; the builder had left it green only vacuously (both halves run).
Per the human's ruling (*pins follow design, not implementation; an unpinning is its own attributed
commit*), it is being retired and replaced by **`guard23-vouch-gates-elision`** — one honest case
where the apt oracle is vouched (install elides) and the systemctl oracle is not (enable runs), the
vouch being the whole difference (contrast `guard23-no-vouch-runs`, all-runs). A narrow Opus is
executing the mechanical swap.

**Stage 3 is now COMPLETE** (guards fire + elide-weld + the blessed claim-tier foundation). The
two-halves doctrine is real end-to-end. Owed-durable for the next conductor: nothing from Part B
except the dead-pin commit landing (in flight) and the parked human threads (below).

## Stage 4 — derived footprints (LANDED 2026-07-04; elide past a payload-bound install)

AI-authored (Opus conductor), appended per the accrete-per-stage discipline. Spec: `24E` (+ §13
fork-resolutions, §14 pipes). Built by one Opus builder over three passes (recon → mechanism →
remainders), conductor-verified at each landing (fresh build + full gates + e2e run by the
conductor's own hand, per never-vouch — process-evidence, not proof).

**The mechanism (all seven 24E pieces + three remainders):** `strip_touches` · the
`Authored`/`Derived{call}` provenance tag on `Footprint` (origin-agnostic consumers) ·
`compile_derivations` (the parallel derivation-probe builder — fork-s4-compile) · the NEW cli
pipeline stage (corr-§2: compile → emit into phase-1 probe artifact → `deriv <leafid> coord=…`
per-site readback lane → all-or-nothing intern → coherence → `Footprint::derived` → merge into
`TrustedFootprints` before `build_plan_walled`) · `Host::derive` (the DECLARED derivation-answer;
no dpkg-simulation) · the `strawman24-derived-survive` flagship · the sweep lying-derived net ·
pipes-in-dialect (§14) · the kill-coherence close (§7).

**The load-bearing numbers (conductor-verified on a fresh build):** yardstick **0→1 derived** —
on `strawman24-derived-survive` flagged `sites=2 elide=1 run=1` (a converged `nginx` install
ELIDES past the RUNNING diverged `oldpkg` install because its backing is disjoint from the
footprint **derived at probe time** from the host-run `touches()` — the natural
`dpkg -L "$1" | sed 's|^|file:|'` idiom); unflagged stays `elide=0 run=2` (byte-identical
Stage-1). **The soundness net has teeth: `derived_lying_divergences=220`** over 3000 seeds (a
too-narrow `Host::derive` manifest ⊂ true `CellDelta` → wrong survival → end-state RED, all
caught + attributed; the fc-5 non-vacuity assertion holds). Final: 145/145 e2e, 25 suites
0-failed, all four gates, on the rebased tip.

**resid-derive-dialect (surfaced by the build, then CLOSED by §14 — the round's sharpest
strain-payoff).** The natural payload-bound idiom `dpkg -L | sed` did NOT lift (the dialect
⊤-rejected the pipe at parse), forcing a coord-emitting helper-tool shape. Closed by the
parse-permissively/trace-conservatively move (24E §14): the parser ACCEPTS a pipeline as one
span-covering `Command{pipeline}` shipping byte-exact (the kLANG mirror-invariant — valid sh
degrades, never hard-kills; pipes ONLY, subshells still parse-reject); the tracers ⊤ on it
(touches-pipe = the escalation trigger; predict-pipe = can't-resolve ⇒ run). The ⊤-bias moved to
the semantic layer where correctness lives; `printf … | sed` correctly still escalates (the
pipeline-⊤ fires before the printf model). This was the first deliberate lift of a spike-local
"refuse the unexpected" posture (human-directed, approaching the r25 real-machine trial).

### Stage-4 residue (accreted; none blocking)

- **resid-derive-coherence (NEW, ~SUSPECT sharp for oracle-authors).** A PURE file-level
  derivation (`dpkg -L | sed` alone) emits only `file:` coords — a different KIND than the wall's
  own `package:` establish — so the own-establish ⊆ footprint coherence check REFUSES it
  (fail-safe: walls). The touches() body must re-emit its own establish coordinate (the flagship
  does, via a leading `printf 'package:%s'`). Honest-but-unergonomic; a Stage-5/6 DX question
  (does every derived touches() carry this boilerplate, or does the engine contribute the site's
  own establish coordinate itself?).
- **strain-kill-coherence-no-net (owed).** The kill-wall coherence refuse (§7) has UNIT teeth
  (side-map + comparand-selection pinned) but no automated end-to-end net: no fixture ships a
  DRIFTED kill footprint, and the sweep ignores `kill_coords`. Closing wants a lying-kill-footprint
  sweep scenario (the kill analogue of the lying-derived net) or a too-narrow-kill fixture.
- **strain-derive-double-lift (minor).** The cli lifts `TouchesSet` twice under the flag (derivation
  lane + authored lane) — deliberate, noted in-code; a future refactor lifts once.
- **strain-derive-overship (minor, accepted).** `compile_derivations` ships a derivation-probe for
  EVERY escalated wall-candidate incl. ones with nothing downstream — wasted probe-phase work,
  fine per work-on-non-human-timescales.
- **resid-derive-adequacy (structural, the field-trial's).** Whether `dpkg -L` on a real box lists
  everything the install actually touches (maintainer-scripts, the cross-kind escape) is
  un-spike-testable (`128` se-2) — the spike proved the MECHANISM; derivation-to-reality fidelity
  is exactly the round-25 real-machine differential's primary target.
- **resid-argparse-drift: dissolved for the DERIVED lane** (the tool emits its own footprint — no
  second argparse), exactly as the Stage-2 handoff predicted; the authored lane keeps the
  coherence check as its net.

**The §8 boundary as built:** "body computes and emits; engine interns and intersects, never
bridges" — a touches() body may emit cross-kind coords its own sh computes (the `file:` lines);
any ENGINE-mediated kind-crossing (expansion bridges, co-reference, the resid-aliasing closure)
remains Stage 5, none built.

## Stage 5 Part A — the aliasing closure (LANDED 2026-07-04 late; resid-aliasing CLOSED for resolver-bearing kinds)

AI-authored (Opus conductor), accreted per-stage. Spec `24F` (+§10); built by one Opus in an
isolated worktree; conductor-verified on the merged tip by own hand (fresh build · clippy
`-D warnings` · 25 suites 0-failed · **147/147 e2e** · flagship inspected line-by-line).
Process-evidence, not proof (never-vouch).

**What landed (6 commits, cherry-picked to `2f8946c`):** the type-shapes (`CanonicalCoord`
private-mint / `Resolution{Canonical, MayAlias}` / `disjoint` re-signed over canonical coords — a
raw coord cannot reach the intersection in a resolver-bearing kind); the `resolve()` sibling
(**KIND-keyed** per §10 — `package.resolve()`, `FnRole::Resolve`, host-run strip-only on the
fork-4A rails, no static evaluator); the cli round-trip (`resolv` readback lane mirroring `deriv`;
§10 confusability — duplicate-kind = refuse-both ERROR, provider-collision = WARNING; the
`dangling-reference` diagnostic; `may-alias=N` on plan-summary; why-lens names the resolver); the
**lying-resolver DST net** (`AliasWall` topology; **`alias_lying_divergences=147`/3000 seeds**,
non-vacuous, resolver-attributed — the three lying-nets now coexist green: static 579 / derived
220 / alias 147); two fixtures (`strawman24-alias-provides` — a converged `nginx-full` victim
past a running `nginx` wall canonically HITS and DEMOTES where token-equality wrongly survived,
differential runs BOTH; `strawman24-alias-symlink` — the fs kind via a realpath-shaped resolver).
Resolver-less kinds stay byte-identical token-equality (the per-kind gradual floor).

### Stage-5A residue (accreted)

- **resid-resolve-derived (owed, narrow).** The round-trip resolves AUTHORED footprint + backing
  coords (phase-1-available); DERIVED-footprint coords (escalated walls, known only post-results)
  are NOT resolved — resolver+derived on one wall needs a second round-trip or resolution folded
  into the derivation readback. Noted in `collect_resolver_coords`; defer until a fixture needs both.
- **resolv-lane parity gap (minor).** gate-1 checks `site`-record parity, not `resolv` (mirrors
  the `deriv` lane's gap); fixture self-consistency is authored + conductor-inspected, not gated.
- **strain-coreference-crosskind (DESIGN INTEL — the §5 seed strained honestly, first-to-give as
  planned).** Two structural blockers for the post-trial co-reference contract: `disjoint`'s
  kind-fence short-circuits cross-kind pairs BEFORE canonicalization, and `CanonicalCoord` fixes
  the kind (a within-kind entity remap; cannot carry a target kind). Co-reference needs the fence
  moved after canonicalization + a kind-carrying canonical — a mechanism extension, correctly out
  of Part A's scope. Recorded, not built.
- **may-alias (§3a) instrumented:** `plan-summary may-alias=N`; fire-rate accrues toward the
  -GUESS default's confirm-or-flip.

**Design-round pointer:** the kind-owner-family design (unified reach-function `reaches()`, typed
emission, naming, error-posture) settled in live human dialogue after this landing — `24G` is the
thought-process record; `USER_STORY` stages 6–7 the surface story. The `touches()` stringly-
emission migration is human-sequenced LAST (after `reaches()` proves the typed shape).

## pipe-guard pins (LANDED 2026-07-05 — the r25 trial-shape question answered empirically)

The sibling conductor's question — does the spike handle the idiomatic version-guard
`otelcol --version | grep -q V || curl … | tar xz`? — answered with pins, not opinion
(fixtures `strawman24-pipe-guard-*`; conductor-verified 152/152 on the merged tree).
**The floor is SAFE** (GREEN pin: the exact 3-line trial book, no oracles — all 12 sites run,
apply verbatim). **The gap is precisely the check-side pipe:** a single-command check lifts
WHOLE today (GREEN contrast: `dpkg -s X || curl|tar` → run-set empty); a check-pipe with a
modeled first stage folds; the fallback pipe omits as a dead branch. The XFAIL (two-sided,
head-signature-guarded): the strongest authored oracle still can't lift the flagship — the `||`
reads the un-oracled last stage (`grep -q`) and rule-query-validity withholds it (opaque
first-stage through the pipe; classify unit-pins fix the through-pipe invalidation in both
directions). **Trial-ceiling note:** on a converged host the bare book's own `||` already
short-circuits the fallback ⇒ the owed value on this shape is check-tax + attention only.

**Gap-anatomy (sized, for Stage 6):** (e-1) pipeline-as-one-status-unit — MEDIUM, but founders
on *filter-blessing* (someone must own "grep -q's rc means match" as a claim; nobody wants to);
(e-2) the provider-less fallback — a non-problem (dead-branch omit works; its wall-footprint is
the existing hork story); (e-3) a LINE-level guard verdict — the first vouched tool in the
AND-OR list owns the line's guard, argv assembled across stages — MEDIUM-to-LARGE, the
plausible product answer. **Human forks parked for Stage 6:** `flag-pipe-status-unit`
(per-command vs per-line oracle-speaks-for) · `flag-filter-blessing`. Harness rider:
`head_ran_check` honors `RAN_ORDER=lax` (multiset equality; dropped/added lines stay RED —
only benign concurrent-stage reordering forgiven).
> **[SUPERSEDED next day — both forks RESOLVED, `24J`:** per-command CONNECTED PROBES chosen;
> per-line is DEAD (beautification-fragility/pseudo-argv/vouch-blast); filter-blessing was a
> PHANTOM (the engine measures rc, never interprets it — grep is ordinary stdlib purity-vouch
> material). Build = the pipefix pass; the XFAIL is the tripwire.]

## First-contact polish pass (LANDED 2026-07-05; merged tip `00664b1` lineage)

The `24H` charter applied in full minus enumerated deferrals (one Opus, 8 commits, 54 files;
conductor-verified: fresh build · clippy `-D warnings` · 25 suites · **152/152 e2e on the
polish tip by the conductor's own run**; merged-tree e2e re-verified post-merge). Landed:
**exit-code family** (⊤-reject fast-fails rc=10, artifact still ships; usage rc=2;
`--help`/`--version` stdout+0; the `DORC_EXIT=<n>` marker teaches run.sh's crash-guard) ·
**rustc-style diagnostic frames** (file:line:col + gutter + ASCII carets; `render_cli`
multi-span wired; byte-offsets dead) · **firehose fix** (structurally-unprobeable sites
suppressed; the rest aggregate to ONE line + a `dorc why` pointer) · **`dorc why`** (source-
line-keyed; unargumented = current-run problems; `rul24-lineno-identity` verified round-trip)
· **CLI ergonomics** (positional books — the day-one bug, multi-book concat, `--oracle-dir`,
`--results`, did-you-mean, humane file-errors) · **jargon pass** (⊤-class → plain English on
user-facing surfaces) · **anstream/anstyle adopted, exact-pinned, cli-edge only** (kernel
dep-free; plain-when-piped verified — no ANSI in captured stderr) · **the elision-render**
(human-leaned, greenlit mid-flight): elided lines render as ORIGINAL-BYTES-commented
(`# apt-get install -y nginx   # dorc: elided (…)`) — scoped provably-safe (top-level,
alone-on-line, single-line, StandIn::True only; two regressions caught in development drove
the `is_alone_on_line` guard); 41 goldens re-blessed, all diffs = the single format change.
**clap DEFERRED deliberately** (the hand-rolled surface now covers the need; a swap = pure
rework risk). **Deferred, enumerated:** production multi-span secondaries (the pipeline-stage
flagship needs a cross-crate structured-diagnostic thread — the frame itself is delivered);
plan-mode why-detail relocation (churns all 13 needles + gate-7); env-mirrors + oracle-dir
hint + `may-alias=` doc-line; analysis-stage label jargon. Residue: none new — the pass is
presentation-plane by construction (artifact bytes changed ONLY via the scoped elision-render).

## pipe-guard LIFT (LANDED 2026-07-05 — the XFAIL promoted; 24J built)

The `24J` connected-pipes design applied in full; the tripwire XFAIL is now GREEN (task #19 —
154/154, zero xfails remain). **Mechanism:** connected read-only check-pipes ship as ONE probe —
the raw book pipe runs *verbatim* on the host, forced not chosen: stage-predict chaining starves
stdin (a per-stage value-substitution has nothing to pipe onward), so the whole pipe is the probe
unit. The governing stage's rc folds the `||` via the existing `StatusRelaxable` path (no new
consumption semantic); the non-governing members **Omit**. The trial-shape line lifts whole
(`: | true || : | :`, run-set empty). Landed alongside: the **if-form beautification** case and the
**unvouched-mid negative control** (silence-is-wall — an orphan/unvouched stage ships no probe).
**Mechanism-3 needed ZERO classify code:** otelcol's `--version` arm is an `:?` Observe, and an
Observe gens nothing — the existing validity pin already covers it. **No new SkipClass / license /
tier** — a `ConnectedPipes` side-map only.

**Residuals flagged for Stage 6:**
- **non-quiet last stages** would leak stdout into the probe record — narrow-first, `-q`-class only
  today.
- **pure/builtin non-last stages don't connect** — a plausible widening, deferred.
- **no sweep scenario** — unit-pins only, deliberate: the sweep's nets are load-bearing.
- **the typos-allowlist commit fixed a PRE-EXISTING polish-base gap** (`aply`/`tust`/`boook`
  fuzzy-matcher fixtures) — flag as a polish-pass oversight, confirmed benign.

**Verified:** 154/154 e2e (freshly re-run by the close-out agent on the merged tip `cdca43b`) + all
gates, conductor-protocol.

## find-lcg-thinning CLOSED (LANDED 2026-07-05 — the §Stage-2b owed fix; wave-1 of the post-rotation queue)

AI-authored (Fable conductor), appended per the accrete discipline. One Opus builder, isolated
worktree; conductor-inspected the diff line-by-line, cherry-picked to `ai/spike3-r23` as
`e3f67a5`, and verified the merged tree by own hand (fresh build · fmt · clippy `-D warnings` ·
deny · typos · 25 suites 0-failed · **154/154 e2e**). Process-evidence, not proof (never-vouch).

**The fix (root cause, not call-site):** `Lcg::chance` now draws via the high-bit Lemire
`below()` instead of `next_u64() % den` — one line + doc-honesty edits, `hostsim/src/lib.rs`
only. Rationale verified: `chance` has exactly ONE live caller today (`Host::seeded`), but
`hostsim/CLAUDE.md` directs the future probe-flakiness fault-axis at `Lcg::chance`, so the
primitive had to be fixed or the bug would be re-inherited. Mirrors `differential.rs`'s own
already-correct `Rng::chance` (the 21D-triage fix); `differential.rs` itself untouched (frozen),
its isolation re-verified in code, not assumed from 24C.

**The concretization (sharper than the original finding):** with an odd multiplier + odd
increment the LCG's low bit STRICTLY ALTERNATES, so for a two-candidate `Host::seeded` every
seed produced exactly-one-of-two — the {both-converged} and {neither-converged} membership
cells were UNREACHABLE at every seed, and the in-crate DST loops' curl-elision-fires branch
(requires both converged) was structurally never exercised in-memory until this fix. Now
exercised and green. New flavour-A pin
`seeded_coins_decorrelate_so_the_full_subset_lattice_is_reachable` (128 seeds, asserts all four
membership cells reachable; a regression to the low-bit draw vanishes {both}/{neither} ⇒ red).

**No prize finding:** the un-thinned space exposed no latent engine bug — full workspace green
(624 tests / 25 suites), e2e untouched as predicted (in-memory-only change).

**Sweep provably unaffected (the brief's counters-will-shift expectation was wrong):**
`dorc-sweep` draws every axis via `below()` directly and builds S0 via `Host::new`, never
`Host::seeded` — counters byte-identical pre/post (3000 seeds: lying=641 / derived=220 /
alias=147 / reach=97; depth run SWEEP_SEEDS=100000 all-green, every non-vacuity gate nonzero,
all 9 topology classes reached by seed 69).

**resid-24C-counter-drift (flagged, not repaired):** HEAD's general lying-net counter is **641**
per 3000 seeds while this ledger's Stage-5A entry says 579 — ALREADY divergent before the lcg
fix. The other three counters match exactly. ~SUSPECT the general counter shifted when later
landings (reach/owncoord/pipe-guard) extended the generator and the 579 was never re-measured;
the documented per-3000-seed numbers herein are LANDING-TIME snapshots, not stable invariants —
read them that way. (The non-vacuity assertions, not the absolute counts, are the load-bearing
thing.)

**Process notes (bind future briefs):** (a) fresh worktrees have an UNTRUSTED `mise.toml` — a
piped `cargo build | tail` masks the mise trust-error silently; briefs gain a step-0.5
`mise trust`. (b) TWO of three wave-1 builders backgrounded the slow e2e then paused forever on
a completion-notification that never re-wakes a stopped agent — briefs now say: run the final
e2e in the FOREGROUND with a generous timeout, never backgrounded.

## Wave-1 queue landings (2026-07-05 — firstwall-hint + degraduation batches 1–2)

AI-authored (Fable conductor), appended per the accrete discipline. Two Opus builders, isolated
worktrees; conductor-inspected diffs, tip-gated cherry-picks, merged tree verified by own hand at
each landing (fresh build · fmt · clippy `-D warnings` · deny · typos · workspace suites · full
e2e). Never-vouch: process-evidence, not proof. (The wave's third member — find-lcg-thinning —
is ledgered in its own section above. All three builder worktrees audited post-landing: clean,
zero unmerged-by-patch-id commits, zero stashes — nothing in-flight lost to the concurrent
token-limit event that hit a sibling session.)

### firstwall-hint (LANDED `339189a` — the USER_STORY stage-3 nag, real; answers r25 B2)

ONE aggregated stderr hint (plan/round-trip lanes; suppressed on apply) for the FIRST unmodeled
wall in book order: `hint: 'hork' (line 22) is unmodeled: it is the first wall — an oracle
vouching its convergence would elide it when converged, and un-wall 1 downstream site` (plus
`; N more unmodeled walls — dorc why` when applicable; M=0 drops the un-wall clause; NEVER fires
for modeled/honest walls — those are not oracle-gaps). Source-line numbers round-trip through
`dorc why book.sh:N` (rul24-lineno-identity), which carries the per-site reasoning detail;
plain-English register per 24H ack-4; `hint:` severity never trips gate-3. CLI-edge only
(+448/−10 in cli/main.rs incl. 11 in-memory pins per the 24B filing rule); ZERO golden/needle
churn; 154/154 e2e byte-identical at its landing. Conductor eyeballed the live flagship render
(aggregated unprobeable note + guard attribution + the hint + `plan-summary sites=4 elide=1
omit=0 guard=1 run=2`).

**find-classify-forecloses-refold (the design finding; +SURE, corpus-consistent):** the honest
counterfactual ("re-fold the plan with the wall treated as elided") is STRUCTURALLY UNAVAILABLE
for opaque walls — their poison lands at classify (⊤-reach ⇒ downstream `EstablishWritten`),
not in the plan wall-walk, and past-wall unvouched sites never ship probes — so no plan-level
re-fold can un-poison (the strain-classify-coupling shape, third appearance). M is therefore the
sanctioned conservative window-count: `Disposition::Guard` sites strictly between the first
opaque wall and the NEXT wall of any kind (opaque or honest; transparent steps pass through).
Errs HIGH in one named shape (a guard whose EstablishWritten came from a same-cell in-script
write above the wall) and LOW versus a full re-classify counterfactual (past-wall unvouched
sites are uncountable — their probes never shipped). Both directions acceptable for an advisory
that licenses nothing.

Residue: **resid-hint-emission-unpinned-e2e** — gate-7 greps `why:` lines only, so the hint lane
has no e2e needle; the 11 unit pins + the conductor's live render check carry it (accepted;
cheap to revisit if the hint lane grows). Minor: the cli MIRRORS the plan crate's private
`class_is_establish_bearing` (3-line predicate, doc-noted as a kept-in-step mirror) — export it
if a third consumer appears.

### e2e de-graduation, wave 1 = 24I batches 1–2 (LANDED `817a4f7`..`a899fff` — 154→126, −28)

All 28 named REDUNDANTs deleted, 0 skipped, each twin EYEBALLED by the builder before deletion
(the 24I discipline); `exec-shimmed-query-fold` KEPT; no Rust, no run.sh/yardstick.sh, no
STAY-set, no `strawman24-*` touched. Conductor-verified on the stacked tree: **126/126 e2e
(exactly 154−28), all four gates, 14 test suites, 0 xfail / 0 XPASS / 0 red.** The case→twin
map (durable home — the builder report is ephemeral):

- converged → plan::converged_ambient_install_is_replaced_rest_runs
- diverged → plan::diverged_install_runs
- consumed-output → om::spec_converged_stdout_piped_to_grep_must_run + cfg::consumed_nonlast_pipeline_stage
- enclosing-group-redir → om::spec_converged_enclosing_group_redirect_must_run + cfg::consumed_enclosing_group_redirect_marks_inner_leaf
- redir-as-effect → effect::blessed_pure_colon_with_write_redirect_invalidates_downstream_query (+ converged baseline)
- background-amp-runs → syntax background-amp unsupported pin + cfg::unsupported_loop_becomes_top_node_with_diagnostic + om::spec_topcontext_background_leaf_must_run
- guard-status-blocks-elision → om::f1_status_consumed_by_if_guard_blocks_replacement + cfg::consumed_negated_if_guard_marks_relaxable
- andor-rc-undeclared-runs → cfg::consumed_oror_left_operand_marks_relaxable_status (byte-identical book) + om::andor_left_operand_undeclared_rc_runs_kfail_perform
- y1-devnull-exempt → effect::devnull_redirect_does_not_invalidate_query (+ converged baseline)
- y1-var-resolved-target-invalidates-query → effect::var_resolved_redirect_target_invalidates_query + effect::var_resolved_redirect_gens_concrete_cell_not_top
- y1-redirect-write-invalidates-query → effect::write_redirect_invalidates_downstream_query + effect::append_redirect_also_invalidates_query
- y1-top-target-poisons → effect::top_target_redirect_poisons_downstream_query + effect::top_target_redirect_discloses_not_silent + effect::opaque_upstream_poisons_ambientness
- no-oracle → plan::compile_probe_no_probe_for_kind_makes_site_unresolvable (+ coverage::runs_unprobed_when_no_results)
- toprejected → syntax::loop_shapes_outside_the_subset_stay_unsupported_loop (exact book) + cfg::unsupported_loop_becomes_top_node_with_diagnostic + cfg::unsupported_in_sequence_keeps_neighbours_live
- while-read-file-rejects → syntax::construct_trailing_redirection_is_loud_top_reject + syntax::construct_trailing_redirection_salvages_the_construct
- inline21-recursion-rejects → cfg::direct_recursion_refuses_with_cycle_diagnostic + effect::recursive_call_refuses_inline_and_poisons_the_body
- inline21-redirect-body-refuses → cfg::body_write_redirect_to_real_file_refuses_but_devnull_is_exempt
- inline21-overbudget-degrades → cfg::at_budget_body_inlines_over_budget_refuses
- door3-or-true-elides → om::door3_oror_true_converged_mutator_is_replaced + coverage::dead_invariant_door3_or_true (exact book incl. set -e)
- door3-or-true-diverged-runs → om::door3_oror_true_diverged_mutator_runs_effect_still_gates + coverage::dead_invariant_diverged_still_runs
- door3-and-true-blocks → cfg::consumed_andand_true_keeps_relaxable_not_invariant + om::pins_converged_status_via_andand_runs_mutator_rc_top
- exec-dollarq-blocks-elision → om::cmd_consuming_dollar_question_blocks_predecessor + cfg::consumed_dollar_question_marks_predecessor_c3
- exec-query-guard-composition → om::query_guard_holds_omits_install_and_substitutes_guard + om::clean_query_guard_still_renders_dead_body_as_colon
- exec-query-after-mutator-runs → om::query_guard_invalid_after_mutator_runs_for_real + effect::query_after_mutator_is_invalid
- door1-guard-below-mutators-invalid → effect::query_after_mutator_is_invalid + om::query_guard_invalid_after_mutator_runs_for_real
- fold-oror-guard-omits → om::query_guard_holds_omits_install_and_substitutes_guard (byte-identical book) + om::clean_query_guard_still_renders_dead_body_as_colon (pins the true-or-colon render)
- exec-converged → plan::converged_ambient_install_is_replaced_rest_runs (book byte-identical to converged)
- exec-diverged → plan::diverged_install_runs (byte-identical to diverged)

Strains (builder's, conductor-adjudicated; accepted with these dispositions):
- **st-1 — INHERITS INTO BATCH 3 AS A NAMED MUST-COVER:** door3-or-true-elides's leaf-exact
  `true || true` render + its dash-n cleanliness is no longer identically pinned — dispositions
  and attribution have twins, but the render STRING rides only the general span-render machinery
  plus the `strawman24-pipe-guard-*` STAY case's richer `: | true || : | :` collapse, and
  fold-oror-guard-omits's in-memory twin pins the `true || :` render text without dash-n.
  Narrow residual blindness, accepted; batch 3's one-shot `dash -n` per rendered artifact (THE
  24I ap-2 flag) must name this shape explicitly.
- st-2: five twins are composed/provider-swapped (mechanism-equivalent; enumerated in the map) —
  redir-as-effect, y1-devnull-exempt, y1-top-target-poisons (query-consumer vs
  establish-consumer), door1-guard-below-mutators-invalid, guard-status-blocks-elision
  (provider swaps).
- st-3: the set-e + query-fold composition (old exec-query-guard-composition) is now IMPLICITLY
  covered via the target-state-purity reduction (the same known Query rc relaxes the errexit
  StatusRelaxable mark); no dedicated in-memory twin.
- st-4 (positive): the coverage crate carries verbatim-book attribution twins for the door-3 and
  verdict-baseline deletions — stronger backing than 24I's om/plan/an mapping claimed.
- st-5: the DORC_EXIT=10 fast-fail-verbatim contract survives the ⊤-reject deletions via the
  staying `top-eval` + `guard23-background-not-guarded` (verified BEFORE deleting).
- st-6: 24I's accounting was slightly stale — the corpus was 154 not 152 (two pipe-guard cases
  post-date the audit), and guard-status-blocks-elision + andor-rc-undeclared-runs exec under
  mocks (not analysis-only). No twin decision changed.

Rider SKIPPED, correctly (**resid-guard23-stale-comments**): the stale "XFAIL until the guard
tier lands" book comments echo VERBATIM into 6 guard23 `expected.out` goldens (rec-1
comment-flow), so the "comment-only" cleanup actually cascades into protected STAY-set goldens —
owed to a future BLESS-and-inspect session, never a drive-by edit.

Remaining 24I work: batches 3–5 (batch 3 = the DEGRADE tier + the new `render_corpus.rs` home +
THE dash-n net + st-1's must-cover; batch 4 = guard23 no-mint floors → GuardLicense-absence
asserts; batch 5 = survival-tier vs sweep, per-topology assertion-depth verification FIRST).
