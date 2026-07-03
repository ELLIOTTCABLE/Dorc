# 24D — Stage 3 type-architecture: the claim-tier algebra + the guard tier (conductor spec)

AI-authored (Fable conductor), 2026-07-03, round 24. The type-contract SPEC for Stage 3,
authored by the conductor per rul24-overtype (type-contract design is the sanctioned
conductor-code domain). This is the arc-win (`24A`, the claim-tier trust algebra) made
concrete + the guard tier the 9 `guard23-*` xfails pin. The Stage-3 builder implements the
SHAPES + unrepresentability PROPERTIES here; mechanics are the builder's (the strawman-teaches
ethos applies to code, not to these type-contracts). **This is the round's most foundational
type-decision — the reviewable one.** Confidence-marked.

## What Stage 3 builds (four welded pieces)

1. the **claim-tier trust algebra** (`Claim<T: Tier, _>`; the foundation everything downstream
   inherits — births here, pays hardest at Stage 5);
2. the **verdict-function lift** (`is_converged()`/`is_diverged()` — the corpus's first
   consumed verdict functions; predict-only keying gains verdict-fn keying);
3. the **elide-weld** (the elide-license now DEMANDS a judgment-tier claim — closing the
   HEAD vouchless-elide gap; the LICENSE-SOURCE ruling made a signature);
4. the **guard tier** (emitter + `GuardLicense` + gate-6 widening — the 9 `guard23-*` xfails
   turn XPASS).

## §1. The claim-tier trust algebra (the arc-win, concrete)

The shape (adjust idiom to the codebase; keep the properties):

```
// A tier is a zero-size phantom marker. Sealed (no external impls).
trait Tier: private::Sealed { }
enum FactTier {}      // probe-measured / derived — licenses READ-reproduction only
enum JudgmentTier {}  // authored acceptance (verdict-fn authoring) — licenses per rung
enum SilenceTier {}   // absence-of-claim — licenses NOTHING (representable, useless)

// A payload wrapped with the tier of authority it carries.
struct Claim<T: Tier, P> { payload: P, _tier: PhantomData<T> }
```

The **four unrepresentability properties** (the five hand-maintained prose disciplines collapse
into this compile-error family):

- **TC-tier-1 — demotion is one-way, toward display.** There is a `Claim<T,P> →
  Claim<SilenceTier,P>` (or "toward display") coercion and NO inverse: no `FactTier →
  JudgmentTier`, no `SilenceTier → anything`, no upgrade. `Silence` is a genuine tier (so
  "default"/"unmarked" is *spellable and useless*, never absent-and-ambiguous — the anti-233
  move made typed). +SURE this is the load-bearing one.
- **TC-tier-2 — license-mints DEMAND their tier in the signature.** The elide-license
  constructor takes a `Claim<JudgmentTier, _>` (the vouch); a read-reproduction/StatusRelaxable
  mint takes a `Claim<FactTier, _>`. A `SilenceTier` claim satisfies NEITHER signature ⇒
  silence-licenses-nothing is a *type error*, not a runtime check. **The elide-weld IS this
  signature change** (see §3).
- **TC-tier-3 — no function from JudgmentTier claims into fact-plane value types.**
  vouch-never-enters-the-fact-plane (rul-guard-license) becomes: there is no
  `Claim<JudgmentTier,_> → {fact-plane types}` path. A judgment can inform a license; it can
  never become an ambient fact another site's reasoning reads.
- **TC-tier-4 — the rung parameter lives INSIDE JudgmentTier, and is OPEN.** See §4: build it
  as a single "both-rungs" today; the type must have a *place* for the rung without prejudging
  the split.

**Honest bound — put it in the module doc, verbatim intent:** *types protect the PLUMBING (no
claim is ever consumed above its authority); they do NOT and cannot make a judgment TRUE. 233
stays permanent — a footprint or vouch can be honestly-authored and still wrong; the algebra
guarantees only that a wrong judgment is consumed at the tier it was offered, blamed to its
author, never silently promoted.* (The naming-and-docs gate WILL check this line exists.)

**Wrapping Stage-2's embryos (NOT a rewrite — `24A` arc-win):** Stage-2's `Footprint`
(no-`From<effect>`), the `SurvivalWitness`, and `Option<TrustedFootprints>` are already
tier-shaped embryos — a footprint/witness is judgment-tier evidence, `TrustedFootprints`'
absence is silence. The algebra WRAPS them (a `Footprint` becomes/carries a
`Claim<JudgmentTier,_>` where it feeds a license; the survival mint that currently rides the
existing elide license now rides the *tier-demanding* one). Do NOT rewrite survival.rs's
internals; re-sign its boundary. ~SUSPECT the wrap is boundary-only; if it forces internal
churn, tc-flag and stop.

## §2. The guard tier (the 9 `guard23-*` xfails; rul-ternary-verdict / rul-guard-license)

- **The emitter.** A guarded site renders `( <check-invocation> ) || <original bytes>` — the
  check is the oracle's own verdict-function body shipped STRIP-ONLY (bare marks deleted whole
  per strip-fidelity; `name.is_converged()` → `name_is_converged()`, nothing else changed), the
  original command's bytes survive VERBATIM (never engine-synthesized sh; never declared output
  in guard position — rul-ternary-verdict's two nevers). Declared-dual glue is the engine-emitted
  lossless sense-flip `( f_is_diverged args; [ $? -eq 1 ] ) || <original>` (rul-rc-partition).
- **`GuardLicense` (the witness; TC-3-shaped like `ReplaceLicense`).** Private fields; sole mint
  from a `(call-site, reached converged-vouch = the verdict-function's reached-path partition
  result, probe-verdict)` triple — and the vouch arrives as a `Claim<JudgmentTier,_>` (§1). No
  vouch ⇒ no `GuardLicense` ⇒ run. rc-partition: 0 = named sense holds, 1 = complement, ≥2 =
  confused ⇒ run. The mint carries NO `StandIn`/`Predicted`/`Observable` (the crisis-closure
  carve-out — a `GuardInsert` mints no values; on pass the check's live rc is the line's rc, on
  fall-through the original runs).
- **Disposition.** Add `Disposition::Guard` (the ternary verdict's third arm). `DispositionCounts`'
  exhaustive match already forces wiring the `guard` bucket (Stage-2 left it forced). Classify it
  erasability-EXEMPT the same way `Derivation.survival` is (artifact bytes unaffected by the
  attribution the render overlays; rec-1). The plan-summary `guard=` field becomes non-zero.
- **gate-6 widening (`e2e/run.sh`).** The `cf-5` forward-lock already pins the shape: a `guard`
  disposition may allowlist ONLY its own check-command as an apply-only line — NEVER an unrelated
  one. Add the paired negative control `cf-6` (a guard licensing its OWN suppressed mutator ⇒ must
  NOT scream) that `cf-5`'s comment says to add once the judge learns guard semantics.
- **Promotion discipline (welded).** When a `guard23-*` xfail turns XPASS: DIFF the engine's
  stdout against the hand-authored `expected.out` line-by-line and inspect BEFORE deleting the
  XFAIL or blessing — never bless-first. A shape-law divergence (engine-synth sh in guard
  position, mutated fall-through bytes, a dropped probe) is a STOP. (This is orchestrator-gated:
  the builder proposes the promotion + the diff; the conductor inspects.)

## §3. The verdict-function lift + the elide-weld

- **Lift `is_converged()`/`is_diverged()`.** Today the lift keys predict-only; add verdict-function
  lifting (the same period-form → mangled-name machinery; `FnRole` already parametrizes the
  parser per the Stage-1c/2 work). The verdict function's reached-path partition result is the
  license source (rul-role-split LICENSE-SOURCE: ONE convergence source at vouched sites; fact-plane
  ambience serves display, never a second license-source).
- **The elide-weld (the semantic change).** At HEAD the elide-tier mints vouchlessly (a converged
  ambient Must fact elides with no vouch consulted — the gap `24A §1c` names). Stage 3 welds:
  the elide-license mint now DEMANDS the `Claim<JudgmentTier,_>` from the reached verdict-function
  (TC-tier-2). Consequence: a converged site whose oracle authored NO verdict function no longer
  elides — it runs (or guards, if past a wall with a vouch). **This CHANGES existing green
  behaviour** (the `strawman24-all-converged-clean` / `converged` cases elide today vouchlessly).
  Handle exactly as the fd10 churn: the affected cases' oracles GAIN a verdict function (making
  the elision licensed), re-authored consciously, enumerated in the stage note, never blanket-
  blessed. ~SUSPECT this is the churniest part of Stage 3; the yardstick should be ~unchanged after
  (the same lines elide, now licensed) — if the yardstick DROPS, that is the vouchless-elide gap
  measured, and a finding.

## §4. The OPEN license-ladder (build both-rungs; do NOT prejudge)

`ORACLE_PROVIDES` provides-license is a LADDER (rung-0 display / rung-1 in-position=guard / rung-2
carried=elide), and the rung-SPLIT (the wary-engineer hatch — an author who wants to license
guards but NOT elisions) is OPEN, human-reserved. The current ruling
(rul24-vouch-is-verdict-authoring) welds authoring a verdict function to BOTH rungs at once.
**Stage 3 builds the current ruling: one verdict-function authoring act licenses both guard and
elide.** The type must RESERVE the rung distinction (TC-tier-4: a rung place inside JudgmentTier,
currently a single "both" value) so the future split is an ADD, not a re-architecture — but must
NOT invent a rung-selection spelling (that is the human's open call). Doc-comment the reserved
place as "the license-ladder rung — OPEN (ORACLE_PROVIDES provides-license); currently always
Both; a future rung-1-only hatch slots here without re-signing the mints." Do NOT block on the
human; build Both, reserve the seam.

## §5. What Stage 3 must NOT do

No probe-time/derived footprints (Stage 4). No grounding bridges (Stage 5). No new footprint
machinery. No rung-selection spelling (§4). No artifact-byte changes beyond the guard render
(which is a NEW disposition, not a mutation of existing bytes). Do not relitigate the settled law.
The battle-oracle suite (#10) is BLOCKED on this stage (it exercises guards) — leave it.

## Confidence

+SURE: the four welded pieces; TC-tier-1/2/3 as the compile-error family; the guard emitter shape
(pinned by the 9 xfails); the elide-weld demands a judgment-tier claim. ~SUSPECT: the embryo-wrap
is boundary-only (tc-flag if it churns survival.rs internals); the elide-weld churn leaves the
yardstick ~unchanged. -GUESS: the exact placement of the rung reserve (TC-tier-4) — builder's
call, low lock-in, just reserve SOMETHING. OPEN (human, not builder): the rung-split spelling.
