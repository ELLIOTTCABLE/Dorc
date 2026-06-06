# 167 — adversarial review of the `effect` analysis (ambient gate)

> **Status (2026-06-05): spike, adversarial cross-check of `analysis::effect`.**
> Method (the `adversarial-crosscheck` pair): two clean-context, web-grounded
> subagents — one neutral ("assess correctness"), one disowned-adversarial ("a
> colleague I distrust wrote this skip-gate; find the wrong-skip"). Both read the
> human docs (README/DESIGN/spike-CLAUDE) + the actual code, **neither was given
> notes 160–166** (so they couldn't echo my framing). Both built the spike and
> drove `classify` on ~45 adversarial inputs. **Convergence is the signal**;
> verdicts/tone discarded. Two findings, both already FIXED in commit `18ad386`;
> this note persists the findings + the design problems they surface.

## 0. The result in one paragraph
The reachable-flow gen/kill core is **sound for the modeled sh subset** (two
independent break-attempts failed to find a wrong-skip in straight-line / branch /
loop / subshell / `$( )` / pipeline / `set -e` / here-doc flow — a real confidence
win, though empirical-not-proof). The two real holes were both at the **edge of
what the CFG models**: an establish in a **detached region** (a function body, no
call-edge — find-A) read as ambient, and a **non-converged solve** trusted in
release (find-B). Both fold the wrong way (toward *skip*), both were latent (no
`probe`/`plan` consumer exists yet), both are now gated to `MustRun`.

## 1. find-A (CONFIRMED wrong-skip, latent) — detached function body → `EstablishAmbient`
- **Input:** `p() { apt-get install nginx; }\np` — the in-body `apt-get install
  nginx` was classified `EstablishAmbient(package:nginx)` (probe-and-maybe-skip).
- **Mechanism** (both agents traced it; +SURE): `cfg::lower_funcdef` builds the
  body as a **detached sub-CFG** (its own pred-less `Entry`); the call site `p` is
  a separate `Opaque`/`MustRun` `Command` with no call-edge *into* the body. So the
  body's establish has a vacuous ⊥ reaching-in-state — indistinguishable from a
  genuinely clean "nothing upstream mutated this" — and the old `classify` emitted
  a class for *every* `Command` with no reachability filter. Same root cause as
  `cfg.rs`'s own `find-7` TODO (detached bodies seeded clean), but for
  effect-classification, not errexit inflow.
- **Why it's the cardinal sin:** the body runs under whatever state precedes the
  (unmodeled) call — possibly after an upstream purge, possibly in a loop. The
  analyzer has *zero* call-context, so per `inv-top-reject`/`inv-kfail` the
  establish must fold to ⊤, never be advertised skippable (`kFAIL-perform`).
- **Fix (`18ad386`):** `reachable_from_entry(cfg)` (forward BFS); a `Command`
  unreachable from `entry` can never be `EstablishAmbient`/`EstablishWritten` — it
  folds to `MustRun`. Regression test `detached_function_body_establish_is_not_
  ambient` (the body install + the `p` call ⇒ `[MustRun, MustRun]`; contrast
  `lone_install_is_ambient`, identical establish ambient at top level → proves the
  gate works).

## 2. find-B (latent-hygiene; dormant today) — release trusts a non-converged solve
- `classify` guarded `reach.converged` only with `debug_assert!` (compiled OUT in
  release) then read `reach.states` regardless — contradicting `solve`'s own doc
  ("a correctness-critical caller MUST check `converged`"). A capped solve returns
  a *partial under-approximation*: an upstream kill may not have propagated ⇒ a
  downstream establish wrongly ambient.
- **Dormant, not exploitable today** (both verified; +SURE): the `Reach` lattice
  is monotone + finite-height (facts are interned tokens from a finite script), so
  the cap (`n*1024+4096`) is unreachable — adv stress-tested an 803-node CFG → 803
  rounds. But the solver is *generic and reused* (backward apply-slice,
  ShellEnvState next), so the moment a future transfer is non-monotone / unbounded,
  release silently converts non-convergence into wrong-skips.
- **Fix (`18ad386`):** `let trust_reach = reach.converged;` gates the only
  ambient-producing arm; `!converged` ⇒ every establish folds to `MustRun`. Kept
  the `debug_assert` too (defense-in-depth: loud in test, safe-fold in release).
  Not unit-tested directly — unreachable via `classify`'s public API without a
  contrived non-monotone transfer; covered by the `solve` cap-test +
  by-inspection (a noisy-pointless test avoided, per the test-value rule).

## 3. The convergent NEGATIVE (confidence, not proof) — reachable flow is sound
Both agents tried hard and **could not break** the reachable control-flow (each
traced to real code): a purge correctly poisons a later install across `if`/
`elif`/`else`, `&&`/`||` *both* operands, subshell `( )` + nested, brace group,
command-substitution (assignment + argument position), pipeline stages, `set -e`
paths (the failure-edge only *adds* a `→exit` successor; the fall-through still
carries the kill); `case` arms correctly do **not** cross-poison (no inter-arm
edge — sound, not a leak); here-doc bodies stay *data* (`cat <<EOF…purge…EOF`
emits no purge node); loops ⊤-reject → `Top` → `Opaque` → poison. `Opaque`⇒`Top`
is monotone + absorbing + crosses merges. Determinism/totality confirmed. This
de-risks the gen/kill + `Powerset`-union design for the modeled subset (cf. DP-6).

## 4. Minor / known (safe-direction; mostly already O-3/O-4)
- **fs-2 pre-verb flags shift the verb slot** (`apt-get -t stable install nginx` ⇒
  `Opaque`/`MustRun`): no per-provider flag grammar; errs to run. Known (note 162
  O-3/O-4). Erodes value on a common idiom; sound.
- **fs-3 double-quoted literal not a fixed token** (`apt-get install "nginx"` ⇒
  `MustRun`): `word_literal` matches only `[Literal] | [SingleQuoted]`, not
  `[DoubleQuoted]`. Double-quoting args is *extremely common*, so this loses many
  legitimately-skippable installs. Safe (errs to run). **New, worth a precision
  fix later:** a double-quoted word that is a single expansion-free literal *is* a
  fixed token (no word-splitting) — accepting it is sound IF `ast` only emits
  `DoubleQuoted` for the expansion-free case (verify before touching; soundness
  rides on it). Deferred — calibrate-UP is on *correctness*, not precision.
- **fs-4 any un-oracled command poisons all downstream ambient-ness** (`echo`,
  `:`, `cat`, loops ⇒ `Opaque` ⇒ `Top`): by design (the safe direction); the
  author test `opaque_upstream_poisons_ambientness` pins it. Means *one* un-oracled
  command upstream disables skipping for the rest of the script — a real value
  cost, the precision/coverage pressure for the oracle library (DP-5 lineage).
- **fs-5 redirections contribute no fact** (`Redir` node ⇒ `Pure` in `classify`):
  inert *today* (oracle facts come only from `(provider,verb)`, so no fact's entity
  is a path a `> f` could collide with), but **latent**: the first file-content/
  existence kind whose entity is a path makes `: > /etc/X` followed by an
  oracle-establish on `/etc/X` a wrong `EstablishAmbient`. Also: `effect.rs`'s
  `word_literal` doc-comment cites `may_split` as the guard, but the code relies on
  the stricter single-literal match — misleading comment (harmless).

## 5. DESIGN PROBLEMS for the corpus (the deliverable)
- **DP-8 (detached-region reachability gate; new):** Tier-A intraprocedural
  soundness is *not* "straight-line + branches are fine" — any **detached region**
  (function body now; later `trap` handlers, sourced files, subshell-backgrounded
  `&`) has a vacuous-⊥ inflow that reads as ambient. Every consumer of
  effect-classification needs the entry-reachability gate (now in `classify`) until
  call/handler-edges land (Tier-B supergraph, 160 eng-ifds-supergraph / 163 §5).
  This is the *reachability* instance of note 165's "unmodeled edge ⇒
  under-approximation ⇒ wrong-skip direction" minefield — distinct from the errexit
  instance (note 166). +SURE this generalizes: **modeling a construct half-way
  (its definition but not its invocation edges) is more dangerous than ⊤-rejecting
  it**, because the half-model looks analyzable.
- **DP-9 (solve-convergence is a *consumer* contract; new):** note 164 §3 fix-1
  made `solve` *return* `converged`; find-B shows that is necessary-not-sufficient
  — **every** analysis on `solve` must fold `!converged` to its phase-safe ⊤
  (`MustRun` here). The producer can't enforce it (the type system can't — DP-2);
  it's a per-consumer obligation. Candidate for a typed wrapper when the orientation
  locks (note 165 L1) land: a `Converged<L>` that only yields states behind a
  convergence check. Reinforces DP-2 (un-type-enforceable contracts need backstops).
- **(positive) DP-6 reinforced:** the pure/finite/monotone kernel made the
  reachable-flow soundness *checkable* by two independent agents in one session —
  the DST-friendly, no-DI design pays off in reviewability, not just testability.

## 6. Caveats on the review itself
- The HotOS'25 "JIT and Back Again" shell-static-analysis PDF returned as binary
  and went **unread** by the neutral agent; its grounding is the classic
  reaching-defs/monotone-framework sources (sufficient for the gen/kill-vs-union
  distinction at issue, but the one shell-specific soundness paper is a gap). Flag
  for a future read if a deeper soundness question arises.
- **Cross-interner comparison footgun** (both noted): `FactKey`/`SkipClass` wrap
  interner-relative `Symbol`s, so comparing `classify` outputs from two
  independently-seeded interners spuriously differs. A test/consumer footgun, not
  an engine bug — but the probe/plan stage and any cross-script reasoning must
  share one interner (or re-key). Watch when wiring `cli`.

## 7. Next (where these latent findings go live)
The `probe`/`plan` stage (note 164 §7, note 165 calibrate-UP) is where
`EstablishAmbient` becomes an actual skip — via the `SkipLicense` witness (note 165
L2), mintable ONLY from `EstablishAmbient` ∧ a `Converged` host probe-verdict ∧
ambient. find-A/find-B are pre-gated upstream of that, so the witness sees only
trustworthy candidates. **NOTES INDEX:** …165 orientation-lockdown · 166 CFG
errexit · 167 (this — effect adversarial review).
