# 30O — r30 owed kernel work: the complete accounting, and its schedule

> Tier: conductor synthesis (Fable, third r30 conductor, session opened 2026-08-21; worktree
> `.claude/worktrees/r30-conduct`, branch `ai/r30-conduct`). Remit from the human: the `30N`
> handoff names three owed lanes; produce a MORE COMPLETE accounting of still-not-done kernel
> work measured against every r30 stage, and map it to parallel/serial build steps for the
> human to size to builders before dispatch. Nothing here is acked; `ask-dispatch-gating-questions`
> lists what blocks dispatch. Grades: +SURE / ~SUSPECT / -GUESS per the house rule. Method: the
> reading-guide set + `28Q`/`309`/`306b`/`30I`/`30L`/`30J`/`30D`/`30N`/`30Ng` read in full by
> the conductor; the remaining r30 ledgers and the two needs-registers swept by three read-only
> scouts whose open-item claims were then VERIFIED against code/text (six were stale —
> `refuted-scout-claims` lists them so no successor re-chases them).
>
> Naming: every referent below carries a slug; where the source only had a section number
> the slug is minted here retroactively, `née §N`, so the original stays greppable.

## the-measuring-stick — every r30 stage, and where it stands

r30 had two halves: `300:wave-one-tooling-standup (née §2)` stood up the correctness
tooling; the second half is the `28Q:staging-ladder (née §8)` plus the `309`/`30I`/`30L`
artifact close. Status per stage, +SURE unless marked:

| stage (charter) | status | residue this note schedules |
|---|---|---|
| `300:wave-one-tooling-standup` (facades · derived-defs · minispec · Kani · certifier · sparing re-derivation) | LANDED; the 2026-08-17 push repaired the `30B`/`30H` review residue (root module now import-free, drift alarm mechanized, `dorc-verify promote` exists) | none kernel-tier. `300:lane-flux-engine-hardening` (the penciled refinement-type instrument) is PUNTED to `TODO-ADDTL` by the human 2026-08-21 — unscheduled, not an r30 item |
| `28Q:stage-0-ship-seam` (verdict primacy) | LANDED 2026-08-16; `30La` closed the aggregate residue | `FORFEITS:forfeit-wrapped-case-bodied-book-verdict` (pin30; two seats, cheap chase — named, not scheduled) |
| `28Q:stage-i-definition-factoring` (P1) | LANDED + crosschecked (`308`) | the three closure-custody precision xfails (`lane-load-plane-precision`) |
| `28Q:stage-emission-snapshot-transplant` | LANDED | `28Q:pin-emission-planner-universal` direction-ruled, build unscheduled (`ask-planner-scope-for-front-lift`) |
| `28Q:stage-ii-closure-custody` (P2) | infrastructure LANDED; the `30I` runtime projection LANDED through `30I:step-8-promote-executable-specification` | the policy half is HUMAN: `ratify-committee-sparing-fence` (burndown) · `28M:keep-lift-and-registration-verdicts (née §11)` · `28Q:pin-closure-membership-and-diamond` |
| `28Q:stage-effective-world-reach` (`30K`) | BUILT, reviewed (`30Kb`), repairs landed | `30Kb` non-blocking residue (`register-and-steering-debt`) |
| `28Q:stage-ii-bundle-and-artifact-close` (`30I` steps 5b–8 · `30L` · the `30Ng` rework) | BUILT; corpus promoted | the `30N` handoff's three lanes + loop propagation + the `30Nh` residue (`the-build-backlog`) |
| `309:stage-spine-census` · `309:stage-spine-transition` | BUILT (`30E`/`30F`) | — |
| `309:stage-306-accounting` | PARTIAL: typed authority-absences built; influence carriage DESIGN-closed (`306b:influence-carriage-across-entities (née §10)`) but the one-grade-per-Spine stamp is still the code; the forgiving-parser re-home + full report-only rendering NOT built | `lane-influence-carriage`; report-only rendering is HUMAN-gated (`design-report-only-refusal-scope`, burndown) |
| `28Q:stage-iii-world-scopes` (P3) | NOT STARTED | fully human-gated: `28Q:authored-world-scope-surface (née §10)` (`design-world-scope-surface`) · `28Q:res-incarnation-correlation-door` (`rule-incarnation-continuity-semantics`) · the ssh oracle ⇐ stdlib ⇐ `unblock-starter-oracle-library` |

**`fnd-handoff-undercounts-by-stage`** (+SURE): the `30N` handoff's three lanes are all
`28Q:stage-ii-bundle-and-artifact-close` residue. Against the ladder, r30 also owes the
end-of-r30 xfail (loop propagation), the three closure-custody precision pins that share
`lane-guarded-source-fidelity`'s file surface, `309:stage-306-accounting`'s influence
remainder, and the unscheduled emission-planner build. `28Q:stage-iii-world-scopes` is not a
gap in the handoff — it is correctly absent, being human-gated at every edge.

## the-build-backlog — designed, buildable now, no human gate

- **`lane-bundle-front-lift`** — charter `30Ng:bundle-front-lift-ladder (née §7)` (human-typed:
  `tier-lift-as-is` · `tier-lift-and-munge` · `tier-positional-with-rewrite` · decline). What
  it greens: no xfail; the battery is four new single-stream round-trip cases (one per tier +
  decline) asserting layout AND `expected.ran` unmoved. Folded in as the same artifact-plane
  surface (small; a separate lane would re-load the same context): the `30Nh` residue
  `30Nh:dev-why-world-carries-no-import-edits` (a why-plane completeness gap against the
  human's entire-DAG directive, `30Ng:directive-narrative-carries-the-whole-dag (née §2)`)
  and `30Nh:dev-piped-cell-has-no-e2e` (an expected-empty-stdout harness lane,
  `cli/CLAUDE.md an-artifact-set-runs-from-its-own-generation`). Touch surface:
  `plan/src/render.rs` · `plan/src/lib.rs` (the preamble/hoist + hash-munge seat,
  `plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding`) · `cli/src/artifact.rs` ·
  `cli/src/why.rs` + `world.rs` · `cli/tests/e2e.rs`. Size -GUESS M. Licence-relevant: the
  lift-as-is tier's condition list IS a `rul-happy-path-is-a-closed-set` enumeration proof,
  so the tier predicates are license-review material — map-then-execute (predicates + red
  cells, checkpoint, then munge). Steering lag to fix at fold: `cli/CLAUDE.md
  artifact-forms-derive-from-one-structure` still says front-lifting "waits on a licence
  (`30Nh:tc-bundle-lift-needs-the-spine`)" — RETIRED by `30Ng:bundle-front-lift-ladder`.
- **`lane-load-plane-precision`** — ONE lane, not two (**`dec-merge-guarded-source-with-closure-pins`**,
  conductor, veto-eligible): the handoff's `lane-guarded-source-fidelity` (charter
  `30I:rul-load-semantics-stay-full-fidelity` + `30I:rul-guarded-source-speech-is-lossy` +
  the rewritten `30I:step-4-recognize-exact-guarded-source`; greens `p-x-sentinel-value-conjunct`;
  recovers evidence test `176e0818` from `worktree-sol-adversarial-30M`, a 29-line funcenv
  test) PLUS the three closure-custody precision xfails the census already carries under the
  horizon label `r31:closure-custody` — GLOSS: that label is a census EXPIRY marker
  (`internal_tooling::xfail::PINS` keys horizons by round-marker, never by date), not a lane
  and not a round that exists; it means "redden at the closure-custody work of the round
  after this one". The three: `p-x-definition-grade-keying` (`PredictSet`/`VerdictSet` keep
  one row per `(file, role)`, so the earlier of two within-file definitions produces no row),
  `p-x-helper-unset-f-across-files` (`HelperIndex` resolves last-declaration-wins over
  load-inert sources and asks the environment nothing — `30Ib:helper-unset-f-is-one-filter-drop
  (née §5.7)` prices it), `p-x-regional-helper` (`closure_for` takes no site; the book census
  is depth-blind). Ground for merging: all four are funcenv/`HelperIndex` precision, all four
  are WINNER-SHIFTING and therefore license-review-tier forever
  (`28Q:syn-definition-factored-indices`; `oracle/CLAUDE.md the-frame-lookup-is-the-only-resolution-seat`),
  and the review the tier demands is cheaper once than four times. Touch surface:
  `analysis/src/funcenv.rs` · `analysis/src/value.rs` (sentinel value flow) ·
  `oracle/src/closure.rs` · `cli/src/sourcing.rs` · `cli/src/main.rs` (shared with
  `lane-bundle-front-lift` — the one conflict file `30N` named). Size -GUESS M–L. Checkpoint
  mandatory after the map (which seats move, which cells go red) — funcenv precision is never
  ordinary value-add. NOT included (`ask-blessed-toplevel-direction-suffices`):
  `p-x-blessed-toplevel-conditional`.
- **`lane-loop-propagation`** — charter `30L:loop-propagation-staged-now (née §7)` +
  `30N:loop-propagation-prior-art (née §2)` (the `20S` digest) + `30N:chk-loop-types-paper-review`
  PASSED (six points: member = `RouteInstance` with `iteration: Member(u32)` on the same
  lowered node; ordered no-dedup; `site N.M` numbering kept; witness keys already carry the
  member; per-member self-reach via the suppression-set solve; `StatusIterated` untouched).
  Greens `p-x-loop-population-closes-over-literal-members` — THE end-of-r30 horizon: at
  round-close the conductor bumps `CURRENT_ROUND` and this pin reddens `xfail_census` unless
  landed or re-horizoned with a written `why`. Scope (the `30N` caveat, endorsed): census-mint
  for finite fully-enumerated literal lists + the value plane (`ValueFlow::member_argv` already
  exists from r21 — `effect.rs` consumes it for `EstablishMembers`; the lane extends it to
  close the population) + the consumer seam into `plan::region`/`plan::settle`. It may NEVER
  re-key `ElisionRegion`, witness, or record identity, never drop members from a closed set,
  and never weaken `StatusIterated`. Touch surface: `analysis/src/value.rs` ·
  `analysis/src/effect.rs` · `plan/src/region.rs` · `plan/src/settle.rs` · `core/src/region.rs`
  (read-only by intent). Size -GUESS M. Round-close crosscheck priority (the lane mints a new
  license-bearing population).
- **`lane-influence-carriage`** — charter `306b:influence-carriage-across-entities (née §10)`
  (seven rulings, `306b:rul-influence-carried-by-entities` through
  `306b:rul-untracked-is-not-authored`) + `309:rul-spine-preserves-never-stamps` +
  `30L:rul-shared-influence-never-launders` + ANALYZER-NEEDS `an-host-influence-carriage`.
  Removes the landed one-grade-per-Spine stamp; every stable semantic object (analyzer
  conclusions, decisions, licenses, Spine events, `Selection`/`ArtifactSet`, region decisions,
  routing/output choices) gains a private, immutable, non-optional influence account joined
  at its own mint; unconverted seams become explicit maximally-influenced `untracked`. Closes
  `30N:tc-region-decision-influence-is-first-not-joined`. Deadline per the charter: before
  influence-aware render work or any durable-grade lift reads the current fields — neither is
  scheduled, so this COULD slip a round without breaking a typed deadline
  (`ask-influence-carriage-this-round`). Touch surface: `core/src/influence.rs` ·
  `core/src/spine.rs` · `plan/src/settle.rs` · `plan/src/region.rs` · `plan/src/lib.rs`
  (`Plan::decided`) · `cli/src/artifact.rs` · `cli/src/world.rs` — today 15 files reference
  influence across core/analysis/plan/cli/lint; the conversion touches every constructor of a
  stable object, which is why it is SERIAL and alone. Gate: goldens AND `.whylog`
  byte-identical (the grade is excluded from the durable by `DurableView`, and
  `306b:rul-influenced-values-never-gate-engine-control-flow` means a byte-move is a finding).
  Size -GUESS L; map-then-execute (a census of stable-object constructors first; checkpoint;
  then conversion).
- **`lane-fruit-arc`** (`26K:fruit-arc (née §0a)` — NOT kernel; listed because it has been
  "parallel-anytime, zero code" since 2026-07-28 and keeps being deferred): four render/lint-plane
  items under the human-typed boundary law ("the moral equivalent of adding a printf").
  Sonnet-tier, any time, zero conflict with the kernel lanes. Size S.

## deferred-by-ruling — design-closed, build deadline-triggered (not r30 unless a trigger fires)

- `30D` + `30J` — the predict-contract arc: replace `return 2`-as-predict-decline atomically
  with exact Status + authored DREP channel speech (`30D`, fifteen acceptance obligations, ten
  builder-tier deferred decisions in `30D:deferred-implementation-decisions (née §10)`), then
  predict-qualified family vocabulary (`30J`, twelve acceptance worlds). Deadline: the earliest
  of the stdlib revival, a real survival-authoring trial, or third-party publication
  (`30J:rul-family-vocabulary-build-is-not-an-r30-blocker`). Rider: `30D`'s Spine projection
  of prediction-control records is behind `rul-durable-contents-reviewed-before-design`.
- `28Q:pin-emission-planner-universal` — GLOSS: the human-ruled DIRECTION (2026-08-16) for ONE
  abstract planner that decides, for every shipped definition, its PLACEMENT (top-lifted ·
  adjacent · colocated inside the paren) and its NAME (authored · munged), shared by probe and
  apply behind a policy toggle (probe: verbose-tolerant, no book namespace; apply:
  idiomatic-first, attention-priced), licensed above by `rul-happy-path-is-a-closed-set`.
  Today that logic is scattered: the preamble hoist + hash-munge in `plan/src/lib.rs`, the
  snapshot transplant, the defensive-emission regime. Build unscheduled; its census pins are
  `p-x-intra-compound-plurality` · `p-x-placement-tuning-pair` (horizon end-of-r31) and
  `d-alpha-rename-equivalence` (reserved). See `ask-planner-scope-for-front-lift`.
- `FORFEITS:forfeit-certifier-trip-evicts-elisions` capture (the second, super-dumb
  mini-analyzer) — REVISIT trigger is trips observed in the field; not r30.

## human-gated-rulings — what each unblocks

Already on the burndown (unchanged; listed for completeness of the map):
`unblock-starter-oracle-library` (dialect-reach; gates stdlib ⇒ ssh oracle ⇒
`28Q:stage-iii-world-scopes`) · `ratify-committee-sparing-fence` (`28Q:stage-ii-closure-custody`'s
policy half) · `design-report-only-refusal-scope` (`306b:refusal-whole-target-or-narrower
(née §4c)`; unblocks the forgiving-parser re-home + full report-only rendering —
`309:stage-306-accounting`'s last third) · `design-atmost-completion-speech`
(`an-atmost-completion-signal`) · `design-world-scope-surface`
(`28Q:authored-world-scope-surface`) · `rule-incarnation-continuity-semantics`
(`28Q:res-incarnation-correlation-door`) · `sweep-conductor-veto-pile` (round-close).

In `30N:open-items-riding-this-conduct (née §4)` / `TODO-ADDTL`, not on the burndown
(non-blocking for the build backlog): `30M:ask-wall-narration-ratify-or-mint (née §3)`
(unblocks `WallFormation` narrative in honest mode; makes
`30Nc:tc-wall-region-operand-population` non-vacuous; small) ·
`30Ng:attn-render-refusal-feeds-the-spine` (a sitting with a termination argument; hard to
unbake; `lane-bundle-front-lift` is explicitly NOT a consumer) · `30Nd:tc-plan-owns-its-source` ·
`30Na:tc-book-level-dot-locals-domain` (end-of-r31 xfail) · `30Na:tc-redirect-refusal-dead-or-owed` ·
`30Na:stop-spine-mode-is-durable` (opaque-review gate first) · `30M:ask-certification-row-shape`
· the prelude-floor veto (`an_unresolvable_prelude_load_floors_the_rest_of_the_prelude`) ·
the prose queue.

NEW from this accounting (`ask-dispatch-gating-questions` carries them): the emission-planner
scoping for the front-lift · whether the typed INERTNESS-IS-DYING direction suffices to build
`p-x-blessed-toplevel-conditional` · whether `lane-influence-carriage` is this round.

## out-of-round — next-round candidates; nothing here is r30-owed

`28Q:stage-iii-world-scopes` (after its three human gates) · the r26 reactive/capture +
multi-host revival (`26B`/`26C`, `260`–`262`; waits behind the `28Q` push — which is now
essentially this note's build backlog) · the ANALYZER-NEEDS transport/scheduling cluster
gated on `22H` (`an-dorc-exec` · `an-marker-protocol` · `an-comms-pole` ·
`an-async-vs-statemachine` · `an-narrowed-plan` cite it verbatim) · book-code load acquisition
(`30Nh:tc-book-code-loads-are-not-in-the-model`, `30I:pin-complex-book-source-render` — the
human's worked example needs it; design-shaped, not ruled) · `300:lane-flux-engine-hardening`
(punted to `TODO-ADDTL`).

## register-and-steering-debt — conductor-owed, non-kernel, cheap

- ANALYZER-NEEDS `an-backing-selfframing` still says "explicit freezing remains owed" —
  `30Nc:req-backings-freeze-at-probe-boundary` landed it. Rewrite the row.
- `oracle/CLAUDE.md only-load-inert-sources-contribute` cites
  `FORFEITS:forfeit-whole-file-inertness-refusal`, which no longer exists in FORFEITS (removed
  under the 2026-08-21 scope sharpening, ~SUSPECT). Either re-point the cite at
  `p-x-blessed-toplevel-conditional` or restore a row; dangling cites are the
  `lint:docids`-invisible class.
- `cli/CLAUDE.md artifact-forms-derive-from-one-structure`: the front-lift "waits on a
  licence" sentence — retire at `lane-bundle-front-lift`'s fold.
- `30Kb` residue unchanged and unscheduled: the honest/non-leaf wall narrative operand (rides
  `30M:ask-wall-narration-ratify-or-mint`) · `WorldRoundModel::classify_origin`'s
  impossible-state fallback pending final-round typestate · the effective-reach prose
  defining case (prose queue).
- The branch set: `ai/r30-loom-surface-build2` (5 ahead), `review-verify-adv` (3),
  `review-verify-neutral` (4) are NOT contained in conduct — not mine to `-D`, listed for the
  human's sweep; `worktree-sol-adversarial-30M` must survive until `lane-load-plane-precision`
  recovers `176e0818`.

## the-schedule

```
sched-parallel-disjoint-lanes   (three builder worktrees; cli/src/main.rs is the one shared file)
   lane-bundle-front-lift        [M; checkpoint after the tier-predicate map]
   lane-load-plane-precision     [M–L; checkpoint after the seat map — license-review-tier]
   lane-loop-propagation         [M; no checkpoint; any re-keying = hard stop]
   lane-fruit-arc                [S; Sonnet; anytime; non-kernel]
   fold order: loop-propagation → front-lift → load-plane-precision (the last because its
   review is the expensive one and its conflict surface with front-lift is one file; the
   first because it touches nothing front-lift edits)
sched-serial-constructor-reshape (alone, over the merged tip)
   lane-influence-carriage       [L; map-then-execute; byte-identical gate on goldens + whylog]
sched-round-close-ceremony       (conductor + human)
   second-half adversarial crosscheck over both tiers (priorities: the self-suppressed solve ·
   the effective_invalidators node-check · the front-lift tier predicates · the closed loop
   populations · the influence constructor census) → `307:veto-sweep-pile (née §5)` (human)
   → `gate:arc` → `CURRENT_ROUND` bump (loop propagation landed or re-horizoned first)
   → the prose queue → fold to ai/main.
human-gated, unschedulable from here: `28Q:stage-iii-world-scopes` · report-only rendering ·
   at-most speech · fence ratification · the predict-contract arc (deadline-triggered).
```

Why this shape: the three kernel lanes are disjoint in files (the touch-surface map above)
and disjoint in what they license (artifact layout · load-plane resolution · loop
populations), so a wrong fold in one cannot masquerade as another's breakage.
`lane-influence-carriage` reshapes constructors across all of them, so running it beside
any of them is the conflict machine `30N` named. The crosscheck goes AFTER it so it reviews
the constructors that lane produced rather than the interim it replaced.

Sizing hints for the human's dispatch: Opus builders for the four kernel lanes;
`lane-fruit-arc` Sonnet with the no-subagent clamp; every brief carries the Safety block,
step-zero worktree verify, the `AGENTS.for-builders-only.md` pointer, the comment budget
split (`//` vs `///`), the scoped-new-case-bless permission, and "reset or
`git commit -- <pathspec>` after `mise run fmt`" (the `30N` standing corrections).

## ask-dispatch-gating-questions — answer in chat or the burndown

- **`ask-planner-scope-for-front-lift`** (blocks the front-lift brief; recommendation
  inline): build the `30Ng:bundle-front-lift-ladder` on the EXISTING
  defensive-emission/hash-munge machinery (lift-and-munge = "munge everything the lifted
  bundle binds, consistently, in the generated plan"), and leave
  `28Q:pin-emission-planner-universal` as the named follow-on that absorbs the lane's
  placement code when it is built. Reason: the human said "sooner than later"; every tier's
  condition is computable from the definition table + census openers today; the universal
  planner is a separate direction-ruled refactor whose other consumers are end-of-r31 pins.
  The RISK flagged, not hidden: this is piecemeal on `28Q` territory, against the standing
  NO-MORE-PIECEMEAL order — if the human would rather the front-lift BE the planner's first
  consumer, the lane grows to L and runs serial after the other two.
- **`ask-merge-closure-pins-into-load-plane-lane`** (blocks that brief): ack or veto
  `dec-merge-guarded-source-with-closure-pins`. Veto ⇒ the three pins wait for a separate
  later lane and the lane shrinks to the handoff's `lane-guarded-source-fidelity` charter.
- **`ask-blessed-toplevel-direction-suffices`** (non-blocking; scopes the load-plane lane):
  does the typed INERTNESS-IS-DYING direction (`oracle/CLAUDE.md only-load-inert-sources-contribute`,
  2026-08-16) suffice to build the MAY-grade binding `p-x-blessed-toplevel-conditional`
  demands, or does it wait on `28Q:res-dot-blessing-is-engine-side`? Conductor read: it
  waits — "the blessing must supply a real MAY-grade binding, never widen the allow-list" is
  a funcenv DOMAIN change, winner-shifting, and that pin is the human's. Excluded unless
  the human says otherwise.
- **`ask-influence-carriage-this-round`** (blocks the serial tier): no typed deadline fires
  in r30. Recommendation: this round, because every lane since `30L` has been accreting
  "interim" grade shapes (`30L:rul-shared-influence-never-launders`'s amendment paragraph;
  `30Nd` left the slice read-only; the parallel lanes will mint more stable objects under
  the old shape), and the round-close crosscheck is the cheapest place to review the
  constructor discipline once. If r30 is to close sooner, this is the lane to drop, not the
  parallel three.

## what-this-note-does-not-do

It schedules nothing human-gated, re-litigates no ruling, and sizes nothing to a builder
count — that is the next sitting's product. It does not re-enumerate the ~150 `S`-status
ANALYZER-NEEDS rows: they are the product backlog, not r30 stages (the scout report that
lists them sits in the session scratchpad and is deliberately not a durable).

## refuted-scout-claims — verified stale, so nobody re-chases them

- "certifier replay mechanism unbuilt" — `analysis/src/certify.rs` carries `SolveReplay`,
  `ReplayUpdate`, `replay_solve`, `REPLAY_UPDATE_CAP`. Built.
- "`308:cr-carry-proof-answers-from-the-wrong-definition` unaddressed" — the wrapper/entry/
  carry lane joined the stage-i conversion at the `308` burndown; `try_carry` takes the
  resolved body and cannot reach a second definition (`28Q:syn-definition-factored-indices`;
  `cli/CLAUDE.md rul-wrapper-members-resolve-independently`). Closed.
- "`30Kb:finding-aggregate-backing-underchecks` routed around, not repaired" — the `30Kb`
  repair set built per-establish effective freshness (`spike/CLAUDE.md
  rul-every-erased-establish-is-vouched`, second paragraph; `30N:dec-skip-30Ka-forfeit-row`;
  `30M:repair-verification (née §8)` verified it). Closed.
- "minispec root imports a nonexistent module" — `minispec/Minispec.lean` is now deliberately
  import-free; the lakefile glob builds every unit. Closed.
- "`dorc-verify promote` missing" — `spike/CLAUDE.md verify-lane-family`: it exists and is the
  only sanctioned lock-writer. Closed.
- "`.`-resolves-against-the-sourcing-file deviation awaiting veto" — superseded by the
  human-ruled cwd parity (`30I:rul-dot-resolves-as-sh`). Closed.
