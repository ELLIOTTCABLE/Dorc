# 30O — r30 owed kernel work: the complete accounting, and its schedule

> Tier: conductor synthesis (Fable, third r30 conductor, session opened 2026-08-21; worktree
> `.claude/worktrees/r30-conduct`, branch `ai/r30-conduct`). Remit from the human: the `30N`
> handoff names three owed lanes; produce a MORE COMPLETE accounting of still-not-done kernel
> work measured against every r30 stage, and map it to parallel/serial build steps for the
> human to size to builders before dispatch. Nothing here is acked; §4 lists what blocks
> dispatch. Grades: +SURE / ~SUSPECT / -GUESS per the house rule. Method: the reading-guide
> set + `28Q`/`309`/`306b`/`30I`/`30L`/`30J`/`30D`/`30N`/`30Ng` read in full by the conductor;
> the remaining r30 ledgers and the two needs-registers swept by three read-only scouts whose
> open-item claims were then VERIFIED against code/text (five were stale — §6 lists them so
> no successor re-chases them).

## §1 — The measuring stick: every r30 stage, and where it stands

r30 had two halves. The FIRST HALF (`300` §2) stood up the correctness tooling; the SECOND
HALF is the `28Q` kernel ladder plus the `309`/`30I`/`30L` artifact close. Status per stage,
+SURE unless marked:

| stage (charter) | status | residue this note schedules |
|---|---|---|
| first-half tooling standup (`300` §2: facades · derived-defs · minispec · Kani · certifier · sparing re-derivation) | LANDED; the 2026-08-17 push repaired the `30B`/`30H` review residue (root module now import-free, drift alarm mechanized, `dorc-verify promote` exists) | none kernel-tier; `lane-flux-engine-hardening` was PENCILED "post-Lean, pre-stage-i" and never dispatched — its window passed silently (§4 q-flux) |
| `28Q` stage-0 ship-seam (verdict primacy) | LANDED 2026-08-16; `30La` closed the aggregate residue | `forfeit-wrapped-case-bodied-book-verdict` (pin30; two seats, cheap chase — not scheduled, named) |
| `28Q` stage-i definition-factoring (P1) | LANDED + crosschecked (`308`) | the three `r31:closure-custody` xfails (§2 A2) |
| `28Q` stage-emission-snapshot-transplant | LANDED | `pin-emission-planner-universal` direction-ruled, build unscheduled (§4 q-planner) |
| `28Q` stage-ii-closure-custody (P2) | infrastructure LANDED; `30I` runtime projection LANDED through step 8 | policy half is HUMAN (fence permanence · keep/lift · closure membership — burndown items 2, `28Q` §9 pins 5/6/10) |
| `28Q` stage-effective-world-reach (`30K`) | BUILT, reviewed (`30Kb`), repairs landed | `30Kb` non-blocking residue: final-round typestate · effective-reach prose defining case (§2 E) |
| `28Q` stage-ii-bundle-and-artifact-close (`30I` 5b–8 · `30L` · `30Ng` rework) | BUILT; corpus promoted | the `30N` handoff's three lanes + loop propagation + `30Nh` residue (§2 A/B) |
| `309` stage-spine-census · stage-spine-transition | BUILT (`30E`/`30F`) | — |
| `309` stage-306-accounting | PARTIAL: typed authority-absences built; influence carriage DESIGN-closed (`306b` §10) but the one-grade-per-Spine stamp is still the code; forgiving-parser re-home + full report-only rendering NOT built | influence carriage (§2 B1); report-only rendering is HUMAN-gated (burndown item 3) |
| `28Q` stage-iii-world-scopes (P3) | NOT STARTED | fully human-gated: `28Q` §10 authored surface (burndown 5) · incarnation door (burndown 6) · ssh oracle ⇐ stdlib ⇐ dialect-reach decision (burndown 1) |

`fnd-handoff-undercounts-by-stage` (+SURE): the `30N` handoff's three lanes are all
stage-ii-bundle-close residue. Against the ladder, r30 also owes the end-of-r30 xfail (loop
propagation), the three r31 closure-custody precision pins that share lane 2's file surface,
`309`'s accounting remainder, the lapsed Flux pencil, and the unscheduled emission-planner
build. Stage-iii is not a gap in the handoff — it is correctly absent, being human-gated at
every edge.

## §2 — The owed work, classified

### A — Designed, buildable now, no human gate (the build backlog)

- **A1 `lane-bundle-front-lift`** — charter `30Ng` §7 (human-typed ladder: T1 lift-as-is ·
  T2 lift-and-munge · T3 positional-with-rewrite · decline). What it greens: no xfail; the
  battery is four new single-stream round-trip cases (one per tier + decline) asserting layout
  AND `expected.ran` unmoved. Folded in as the same artifact-plane surface (small, and a
  separate lane would re-load the same context): the `30Nh` residue `WhyWorld` not carrying
  import edits (`dev-why-world-carries-no-import-edits` — a why-plane completeness gap against
  the human's entire-DAG directive, `30Ng` §2) and the kept-stream cell's missing e2e (an
  expected-empty-stdout harness lane, `cli/CLAUDE.md an-artifact-set-runs-from-its-own-generation`).
  Touch surface: `plan/src/render.rs` · `plan/src/lib.rs` (the preamble/hoist + hash-munge
  seat, `pinned-definitions-are-the-artifact's-binding`) · `cli/src/artifact.rs` · `cli/src/why.rs`
  + `world.rs` · `cli/tests/e2e.rs`. Size -GUESS M. Licence-relevant: T1's condition list IS a
  `rul-happy-path-is-a-closed-set` enumeration proof, so the tier predicates are license-review
  material — map-then-execute split (predicates + red cells, checkpoint, then T2). Steering
  lag to fix at fold: `cli/CLAUDE.md` still says front-lifting "waits on a licence
  (`30Nh:tc-bundle-lift-needs-the-spine`)" — that framing is RETIRED by `30Ng` §7.
- **A2 `lane-load-plane-precision`** — ONE lane, not two (`dec-merge-guarded-source-with-closure-pins`,
  conductor, veto-eligible): the handoff's `lane-guarded-source-fidelity` (charter
  `30I:rul-load-semantics-stay-full-fidelity` + `rul-guarded-source-speech-is-lossy` + the
  rewritten `step-4-recognize-exact-guarded-source`; greens `p-x-sentinel-value-conjunct`;
  recovers evidence test `176e0818` from `worktree-sol-adversarial-30M`, a 29-line funcenv
  test) PLUS the three `r31:closure-custody` xfails the census already carries:
  `p-x-definition-grade-keying` (`PredictSet`/`VerdictSet` keep one row per `(file, role)`, so
  the earlier of two within-file definitions produces no row), `p-x-helper-unset-f-across-files`
  (`HelperIndex` resolves last-declaration-wins over load-inert sources and asks the
  environment nothing — `30Ib` §5.7 prices it as one filter drop), `p-x-regional-helper`
  (`closure_for` takes no site; the book census is depth-blind). Ground for merging: all four
  are funcenv/`HelperIndex` precision, all four are WINNER-SHIFTING and therefore
  license-review-tier forever (`28Q` §1; `oracle/CLAUDE.md the-frame-lookup-is-the-only-resolution-seat`),
  and the review that the tier demands is cheaper once than four times. Touch surface:
  `analysis/src/funcenv.rs` · `analysis/src/value.rs` (sentinel value flow) ·
  `oracle/src/closure.rs` · `cli/src/sourcing.rs` · `cli/src/main.rs` (shared with A1 — the one
  conflict file `30N` named). Size -GUESS M–L. Checkpoint mandatory after the map (which seats
  move, which cells go red) — funcenv precision is never ordinary value-add. NOT included
  (§4 q-blessed-toplevel): `p-x-blessed-toplevel-conditional`.
- **A3 `lane-loop-propagation`** — charter `30L` §7 + `30N` §2 (the `20S` digest) +
  `chk-loop-types-paper-review` PASSED (`30N` §3, six points: member = `RouteInstance` with
  `iteration: Member(u32)` on the same lowered node; ordered no-dedup; `site N.M` numbering
  kept; witness keys already carry the member; per-member self-reach via the suppression-set
  solve; `StatusIterated` untouched). Greens `p-x-loop-population-closes-over-literal-members`
  — THE end-of-r30 horizon: at round-close the conductor bumps `CURRENT_ROUND` and this pin
  reddens `xfail_census` unless landed or re-horizoned with a written `why`. Scope (the `30N`
  §2 caveat, endorsed): census-mint for finite fully-enumerated literal lists + the value
  plane (`ValueFlow::member_argv` already exists from r21 — `effect.rs` consumes it for
  `EstablishMembers`; the lane extends it to close the population) + the consumer seam into
  `plan::region`/`plan::settle`. It may NEVER re-key `ElisionRegion`, witness, or record
  identity, never drop members from a closed set, and never weaken `StatusIterated`. Touch
  surface: `analysis/src/value.rs` · `analysis/src/effect.rs` · `plan/src/region.rs` ·
  `plan/src/settle.rs` · `core/src/region.rs` (read-only by intent). Size -GUESS M. Round-close
  crosscheck priority (the lane mints a new license-bearing population).
- **B1 `lane-influence-carriage`** — charter `306b` §10 (seven rulings) + `309` §§1–2/5/7 +
  `30L` §4.4's amendment + ANALYZER-NEEDS `an-host-influence-carriage`. Removes the landed
  one-grade-per-Spine stamp; every stable semantic object (analyzer conclusions, decisions,
  licenses, Spine events, `Selection`/`ArtifactSet`, region decisions, routing/output choices)
  gains a private, immutable, non-optional influence account joined at its own mint;
  unconverted seams become explicit maximally-influenced `untracked`. Closes
  `tc-region-decision-influence-is-first-not-joined` (`30N` §4.10). Deadline per `306b` §10:
  before influence-aware render work or any durable-grade lift reads the current fields —
  neither is scheduled, so this COULD slip to r31 without breaking a typed deadline (§4
  q-influence-this-round). Touch surface: `core/src/influence.rs` · `core/src/spine.rs` ·
  `plan/src/settle.rs` · `plan/src/region.rs` · `plan/src/lib.rs` (`Plan::decided`) ·
  `cli/src/artifact.rs` · `cli/src/world.rs` — today 15 files reference influence across
  core/analysis/plan/cli/lint; the conversion touches every constructor of a stable object,
  which is why it is SERIAL and alone. Gate: goldens AND `.whylog` byte-identical (the grade
  is excluded from the durable by `DurableView` and influenced values never gate control
  flow — `306b` §6b — so a byte-move is a finding). Size -GUESS L; map-then-execute (a
  census of stable-object constructors first; checkpoint; then conversion).
- **A4 `lane-fruit-arc`** (`26K` §0a — NOT kernel; listed because it is "parallel-anytime,
  zero code" since 2026-07-28 and keeps being deferred): four render/lint-plane items under
  the human-typed boundary law ("the moral equivalent of adding a printf"). Sonnet-tier,
  any time, zero conflict with A1–A3. Size S.

### B — Design-closed, build deferred BY RULING (deadline-triggered; not r30 unless a trigger fires)

- `30D` + `30J` — the predict-contract arc: replace `return 2`-as-predict-decline atomically
  with exact Status + authored DREP channel speech (`30D`, 15 acceptance obligations, 10
  builder-tier deferred decisions in its §10), then predict-qualified family vocabulary
  (`30J`, 12 acceptance worlds). Deadline: the earliest of the stdlib revival, a real
  survival-authoring trial, or third-party publication (`30J:rul-family-vocabulary-build-is-not-an-r30-blocker`).
  Rider: `30D` §10's Spine projection of prediction-control records is behind
  `rul-durable-contents-reviewed-before-design` (opaque-review gate).
- `pin-emission-planner-universal` — human-ruled DIRECTION 2026-08-16, build unscheduled;
  end-of-r31 pins `p-x-intra-compound-plurality` · `p-x-placement-tuning-pair`;
  `d-alpha-rename-equivalence` reserved. See §4 q-planner for its relation to A1.
- `forfeit-certifier-trip-evicts-elisions` capture (the second, super-dumb mini-analyzer) —
  REVISIT trigger is trips observed in the field; not r30.

### C — Blocked on a human ruling (what each unblocks)

Already on the burndown (unchanged; listed for completeness of the map): 1
`unblock-starter-oracle-library` (dialect-reach; gates stdlib ⇒ ssh oracle ⇒ stage-iii) · 2
`ratify-committee-sparing-fence` (`28Q` §9 pins 5/6; stage-ii policy half) · 3
`design-report-only-refusal-scope` (`306b` §4c; unblocks the forgiving-parser re-home +
full report-only rendering — `309` stage-306-accounting's last third) · 4
`design-atmost-completion-speech` (`an-atmost-completion-signal`) · 5
`design-world-scope-surface` (`28Q` §10; stage-iii's authored input) · 6
`rule-incarnation-continuity-semantics` (`28Q` §9 pin 4) · `sweep-conductor-veto-pile`
(round-close).

In `30N` §4 / `TODO-ADDTL`, not on the burndown (non-blocking for A/B): the wall-narration
mint (`30M` §3 — unblocks `WallFormation` narrative in honest mode and makes
`tc-wall-region-operand-population` non-vacuous; small) · `attn-render-refusal-feeds-the-spine`
(a sitting with a termination argument; hard to unbake; A1 is explicitly NOT a consumer) ·
`tc-plan-owns-its-source` · `tc-book-level-dot-locals-domain` (end-of-r31 xfail) ·
`tc-redirect-refusal-dead-or-owed` · `stop-spine-mode-is-durable` (opaque-review gate
first) · `rule-certification-row-shape` · the prelude-floor veto · the prose queue.

NEW from this accounting (§4 carries the questions): the emission-planner scoping for A1 ·
whether the typed INERTNESS-IS-DYING direction suffices to build
`p-x-blessed-toplevel-conditional` · Flux's lapsed pencil · whether B1 is this round.

### D — Out of round (next-round candidates; nothing here is r30-owed)

stage-iii-world-scopes (after burndown 1/5/6) · the r26 reactive/capture + multi-host
revival (`26B`/`26C`, `260`–`262`; waits behind the `28Q` push — which is now essentially
this note's §2 A/B) · the `an-*` transport/scheduling cluster gated on `22H` (five registry
rows cite it verbatim) · book-code load acquisition (`30Nh:tc-book-code-loads-are-not-in-the-model`,
`30I:pin-complex-book-source-render` — the human's worked example needs it; design-shaped,
not ruled).

### E — Register and steering currency debt (conductor-owed, non-kernel, cheap)

- ANALYZER-NEEDS `an-backing-selfframing` still says "explicit freezing remains owed" —
  `30Nc:req-backings-freeze-at-probe-boundary` landed it. Rewrite the row.
- `oracle/CLAUDE.md only-load-inert-sources-contribute` cites
  `FORFEITS:forfeit-whole-file-inertness-refusal`, which no longer exists in FORFEITS (removed
  under the 2026-08-21 scope sharpening, ~SUSPECT). Either re-point the cite at the
  `p-x-blessed-toplevel-conditional` pin or restore a row; dangling cites are the
  `lint:docids`-invisible class.
- `cli/CLAUDE.md artifact-forms-derive-from-one-structure`: the front-lift "waits on a
  licence" sentence — retire at A1's fold.
- `30Kb` residue unchanged and unscheduled: the honest/non-leaf wall narrative operand
  (rides the wall-narration ruling) · `WorldRoundModel::classify_origin`'s impossible-state
  fallback pending final-round typestate · the effective-reach prose defining case (prose
  queue).
- The branch set: `ai/r30-loom-surface-build2` (5 ahead), `review-verify-adv` (3),
  `review-verify-neutral` (4) are NOT contained in conduct — not mine to `-D`, listed for the
  human's sweep; `worktree-sol-adversarial-30M` must survive until A2 recovers `176e0818`.

## §3 — The schedule

```
WAVE A (parallel; three builder worktrees; cli/src/main.rs is the one shared file)
   A1 lane-bundle-front-lift      [M; checkpoint after tier-predicate map]
   A2 lane-load-plane-precision   [M–L; checkpoint after seat map — license-review-tier]
   A3 lane-loop-propagation       [M; no checkpoint; re-keying = hard stop]
   A4 lane-fruit-arc              [S; Sonnet; anytime; non-kernel]
   ── fold order: A3, then A1, then A2 (A2 last because its review is the expensive one and
      its conflict surface with A1 is one file; A3 first because it touches nothing A1 edits)
WAVE B (serial, alone, over the merged A tip)
   B1 lane-influence-carriage     [L; map-then-execute; byte-identical gate on goldens + whylog]
ROUND-CLOSE (conductor + human)
   second-half adversarial crosscheck over A+B (priorities: the self-suppressed solve ·
   the effective_invalidators node-check · A1's T1/T2 predicates · A3's closed populations ·
   B1's constructor census) → `307` §5 veto sweep (human) → `gate:arc` → CURRENT_ROUND bump
   (A3 must be landed or re-horizoned first) → the prose queue → fold to ai/main.
HUMAN-GATED, unschedulable from here: stage-iii · report-only rendering · at-most speech ·
   fence ratification · the predict-contract arc (deadline-triggered) · Flux.
```

Why this shape: A1–A3 are disjoint in files (verified by the touch-surface map in §2) and
disjoint in what they license (artifact layout · load-plane resolution · loop populations),
so a wrong fold in one cannot masquerade as the other's breakage. B1 reshapes constructors
across all of them, so running it beside any of them is the conflict machine `30N` named.
The crosscheck goes AFTER B1 so it reviews the constructors B1 produced rather than the
interim it replaced.

Sizing hints for the human's dispatch: Opus builders for A1/A2/A3/B1; A4 Sonnet with the
no-subagent clamp; every brief carries the Safety block, step-zero worktree verify, the
`AGENTS.for-builders-only.md` pointer, the comment budget split (`//` vs `///`), the
scoped-new-case-bless permission, and "reset or `git commit -- <pathspec>` after
`mise run fmt`" (the `30N` standing corrections).

## §4 — Questions that block dispatch (answer in chat or the burndown)

- **q-planner-scope-for-front-lift** (blocks A1's brief; recommendation inline): build the
  `30Ng` §7 ladder on the EXISTING defensive-emission/hash-munge machinery (T2 = "munge
  everything the lifted bundle binds, consistently, in the generated plan"), and leave
  `pin-emission-planner-universal` as the named follow-on that absorbs A1's placement code
  when it is built. Reason: the human said "sooner than later"; every tier's condition is
  computable from the definition table + census openers today; the universal planner is a
  separate direction-ruled refactor whose other consumers are end-of-r31 pins. The RISK I am
  flagging, not hiding: this is piecemeal on `28Q` territory, against the standing
  NO-MORE-PIECEMEAL order — if you would rather A1 BE the planner's first consumer, A1 grows
  to L and should run serial after A2/A3.
- **q-merge-closure-pins-into-lane-two** (blocks A2's brief): ack or veto
  `dec-merge-guarded-source-with-closure-pins` (§2 A2). Veto ⇒ the three pins wait for a
  separate r31 lane and A2 shrinks to the handoff's charter.
- **q-blessed-toplevel-direction-suffices** (non-blocking; scopes A2): does the typed
  INERTNESS-IS-DYING direction (`oracle/CLAUDE.md`, 2026-08-16) suffice to build the
  MAY-grade binding `p-x-blessed-toplevel-conditional` demands, or does it wait on
  `28Q:res-dot-blessing-is-engine-side` (your pin 2)? My read: it waits — "blessing must
  supply a real MAY-grade binding, never widen the allow-list" is a funcenv DOMAIN change,
  winner-shifting, and pin 2 is yours. Excluded from A2 unless you say otherwise.
- **q-influence-carriage-this-round** (blocks Wave B): B1 has no typed deadline that fires
  in r30. Recommendation: this round, because every lane since `30L` has been accreting
  "interim" grade shapes (`30L` §4.4, `30Nd` left the slice read-only, A1–A3 will mint more
  stable objects under the old shape), and the round-close crosscheck is the cheapest place
  to review the constructor discipline once. If r30 is to close sooner, B1 is the lane to
  drop, not A1–A3.
- **q-flux-pencil-lapsed** (non-blocking): `lane-flux-engine-hardening` was typed DEFERRED
  to "post-Lean, pre-stage-i" and never dispatched; stage-i is long landed. Re-pencil (where?)
  or drop to the correctness-tooling backlog. Not a kernel lane; raised because a typed
  pencil lapsing silently is the kind of thing you asked this accounting to catch.

## §5 — What this note does NOT do

It schedules nothing human-gated, re-litigates no ruling, and sizes nothing to a builder
count — that is the next sitting's product. It does not re-enumerate the ~150 `S`-status
ANALYZER-NEEDS rows: they are the product backlog, not r30 stages (the scout report that
lists them sits in the session scratchpad and is deliberately not a durable).

## §6 — Scout claims refuted by verification (so nobody re-chases them)

- "certifier replay mechanism unbuilt" — `analysis/src/certify.rs` carries `SolveReplay`,
  `ReplayUpdate`, `replay_solve`, `REPLAY_UPDATE_CAP`. Built.
- "`308:cr-carry-proof-answers-from-the-wrong-definition` unaddressed" — the wrapper/entry/
  carry lane joined the stage-i conversion at the `308` burndown; `try_carry` takes the
  resolved body and cannot reach a second definition (`28Q` §1; `cli/CLAUDE.md`). Closed.
- "`30Kb:finding-aggregate-backing-underchecks` routed around, not repaired" — the `30Kb`
  repair set built per-establish effective freshness (`rul-every-erased-establish-is-vouched`'s
  second paragraph; `30N` §1 `dec-skip-30Ka-forfeit-row`; `30M` §8 verified it). Closed.
- "minispec root imports a nonexistent module" — `minispec/Minispec.lean` is now deliberately
  import-free; the lakefile glob builds every unit. Closed.
- "`dorc-verify promote` missing" — `spike/CLAUDE.md verify-lane-family`: it exists and is the
  only sanctioned lock-writer. Closed.
- "`.`-resolves-against-the-sourcing-file deviation awaiting veto" — superseded by the
  human-ruled cwd parity (`30I` §3.2). Closed.
