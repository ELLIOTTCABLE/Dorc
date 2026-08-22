# 30O — r30 owed kernel work: the complete accounting, and its schedule

> Tier: conductor synthesis (Fable, third r30 conductor; worktree
> `.claude/worktrees/r30-conduct`, branch `ai/r30-conduct`). Minted 2026-08-21 as the
> owed-work accounting; re-cut 2026-08-22 after the emission/inclusion sitting (`plans/30P`)
> settled the lane shapes. Grades: +SURE / ~SUSPECT / -GUESS. Naming: every referent carries a
> slug; where the source only had a section number the slug is minted here, `née §N`.
> `plans/30P` outranks this note wherever they overlap: this note SCHEDULES, `30P` DESIGNS.

## the-measuring-stick — every r30 stage, and where it stands

r30 had two halves: `300:wave-one-tooling-standup (née §2)` stood up the correctness
tooling; the second half is the `28Q:staging-ladder (née §8)` plus the `309`/`30I`/`30L`
artifact close. End-of-r30 is kernel quiescence; everything not scheduled here is
unscheduled, not "later" [TYPED 2026-08-22]. Status per stage, +SURE unless marked:

| stage (charter) | status | residue scheduled here |
|---|---|---|
| `300:wave-one-tooling-standup` | LANDED; the `30B`/`30H` review residue repaired 2026-08-17 | none; `300:lane-flux-engine-hardening` punted to `TODO-ADDTL` |
| `28Q:stage-0-ship-seam` | LANDED; `30La` closed the aggregate residue | `FORFEITS:forfeit-wrapped-case-bodied-book-verdict` (named, not scheduled) |
| `28Q:stage-i-definition-factoring` | LANDED + crosschecked (`308`) | the three closure-custody precision xfails (`lane-load-plane-precision`) |
| `28Q:stage-emission-snapshot-transplant` | LANDED | the emission planner build (`lane-emission-planner`) |
| `28Q:stage-ii-closure-custody` | infrastructure LANDED; `30I` runtime projection LANDED through step 8 | policy half HUMAN (`ratify-committee-sparing-fence` · `28M:keep-lift-and-registration-verdicts (née §11)` · `28Q:pin-closure-membership-and-diamond`) |
| `28Q:stage-effective-world-reach` (`30K`) | BUILT, reviewed, repaired | `30Kb` non-blocking residue (`register-and-steering-debt`) |
| `28Q:stage-ii-bundle-and-artifact-close` (`30I` 5b–8 · `30L` · `30Ng`) | BUILT; corpus promoted | the handoff's lanes, re-shaped by `30P` (`the-build-backlog`) |
| `309:stage-spine-census` · `stage-spine-transition` | BUILT | — |
| `309:stage-306-accounting` | PARTIAL: typed authority-absences built; influence carriage design-closed, code still one-grade-per-Spine; report-only rendering unbuilt | `lane-influence-carriage`; report-only rendering HUMAN (`design-report-only-refusal-scope`) |
| `28Q:stage-iii-world-scopes` | NOT STARTED; human-gated at every edge | — |

**`fnd-handoff-undercounts-by-stage`** (+SURE): the `30N` handoff's three lanes were all
`28Q:stage-ii-bundle-and-artifact-close` residue; the loop xfail, the closure-custody pins,
`309:stage-306-accounting`'s remainder, and the unscheduled emission planner were outside it.

## the-build-backlog — designed, buildable now, no human gate

- **`lane-emission-planner`** (absorbs the handoff's `lane-bundle-front-lift`) — charter
  `30P:the-emission-planner`: ONE component, apply-side now, probe seam reserved; placement
  `hoist | in-place | sink` × naming `authored | munged`; first consumer the
  `30Ng:bundle-front-lift-ladder` (`tier-hoist-as-is` · `tier-hoist-munged` ·
  `tier-in-place-rewritten` · decline) with a `dorc why` reason arm per tier; second consumer
  `p-x-placement-tuning-pair` (the `sink` value). Riders from `30Pb`, ratified-in-stance: the
  hoist legality predicate covers every book OBSERVATION or MUTATION of a bound name above the
  `.` (`command -v`, `type`, `unset`, `alias`, variable reads — not calls only); emitted-name
  injectivity over the full emitted ∪ book namespace with detect-and-lengthen on digest
  collision; header-only renames only for definitions whose every reference is
  engine-emitted (helpers referenced from authored bodies stay in place or withhold —
  alpha-rename stays reserved); confirm whether today's preamble hoist of oracle constants
  can shadow a same-named book variable (a latent hole if so). The `30Nh` why/harness residue
  rides here (`dev-why-world-carries-no-import-edits`; the kept-stream expected-empty-stdout
  e2e). Touch: `plan/src/render.rs` · `plan/src/lib.rs` (preamble/hoist, hash-munge) ·
  `cli/src/artifact.rs` · `cli/src/why.rs` + `world.rs`. Size -GUESS L. Map-then-execute:
  tier predicates + red cells, checkpoint, then munge. Steering at fold: retire
  `cli/CLAUDE.md`'s "front-lifting waits on a licence" sentence (done in this re-cut).
- **`lane-load-plane-precision`** — charter `30P:the-load-principles` + the handoff's
  guarded-source charter (`30I:rul-load-semantics-stay-full-fidelity` ·
  `rul-guarded-source-speech-is-lossy`; greens `p-x-sentinel-value-conjunct`; recovers test
  `176e0818` from `worktree-sol-adversarial-30M`). Commit-series, in order: (1) point-havoc
  (`p-x-unknown-source-is-a-point-havoc`) over the v0 domains — function bindings (havoc at
  the line, last-wins re-binds), cwd ⊤ (later relative `.` unresolvable), shell options ⊤,
  positionals ⊤, termination May-reach; the unknown `.` stays an emission opener and an
  execution wall; (2) `${0%/*}` resolved from the AUTHORED book path, never a shell's `$0`
  (`p-x-load-operand-param-expansion-of-dollar-zero`; the no-slash and root-`/` traps
  measured); (3) `mech-acquire-and-ship-plain-sh`: a resolvable plain-sh `.` enters the load
  account as an occurrence, is mirrored beside the plan by the existing placement, and is
  analyzed NOT AT ALL (the site walls) — first confirm the ~SUSPECT that today the file is
  not shipped and the plan dies at that line on the host (a failed `.` is fatal, atlas-measured);
  mirror touch on `cli::artifact` expected minimal, coordinate with the planner lane if not;
  (4) the computed-`.` parse-tier refusal becomes a post-analysis pre-network fail-fast
  (`30P:rul-floor-valid-text-never-parse-fails`; easiest spelling); (5) the three
  closure-custody xfails `p-x-definition-grade-keying` · `p-x-helper-unset-f-across-files` ·
  `p-x-regional-helper` (`30O:dec-merge-guarded-source-with-closure-pins`, ACKED). Riders:
  funcenv precision is license-review-tier, forever — checkpoint after the seat map; the
  slashless-operand lint hint (`. helpers.sh` is a PATH search); `30Pb`'s three-state load
  identity (POSSIBLE / EXACT / ENGINE-SELECTED — only EXACT re-says or mints custody);
  `rul-guard-resolves-like-its-mutation` [PROPOSED, pending ratification] must not be
  violated by any seat touched. NOT included: `p-x-blessed-toplevel-conditional` (waits on
  `28Q:res-dot-blessing-is-engine-side`); `dirname`-headed operands beyond POSSIBLE (ship +
  wall). Touch: `analysis/src/funcenv.rs` · `analysis/src/value.rs` · `analysis/src/load.rs` ·
  `oracle/src/closure.rs` · `cli/src/sourcing.rs` · `cli/src/main.rs` (shared with the planner
  lane). Size -GUESS L.
- **`lane-loop-propagation`** — charter `30L:loop-propagation-staged-now (née §7)` +
  `30N:loop-propagation-prior-art (née §2)` + `30N:chk-loop-types-paper-review` PASSED. Greens
  `p-x-loop-population-closes-over-literal-members` (the end-of-r30 attention-call). Scope:
  census-mint for finite literal lists + the value plane (`ValueFlow::member_argv` exists from
  r21) + the consumer seam into `plan::region`/`plan::settle`; never re-key `ElisionRegion`,
  witness, or record identity; never drop members; `StatusIterated` untouched. IF room remains
  after it folds: globs as the second population source (`30P`: set-valued, order-unknown, a
  UNIVERSAL meet over every member order — members may `unset -f`, assign, `cd`, `exit`;
  no-match is fatal) — otherwise honestly unscheduled. Touch: `analysis/src/value.rs` ·
  `analysis/src/effect.rs` · `plan/src/region.rs` · `plan/src/settle.rs`. Size -GUESS M.
- **`lane-influence-carriage`** — charter `306b:influence-carriage-across-entities (née §10)`
  + `309:rul-spine-preserves-never-stamps` + `30L:rul-shared-influence-never-launders` +
  `an-host-influence-carriage`. Removes the one-grade-per-Spine stamp; every stable semantic
  object carries a private, immutable, non-optional influence account joined at its mint;
  unconverted seams are explicit maximally-influenced `untracked`. STAYS IN r30 [TYPED
  2026-08-22: critical for reasons outside the conductor's horizon; never a droppable lane].
  Touch: every constructor of a stable object (`core/src/influence.rs` · `core/src/spine.rs` ·
  `plan/src/settle.rs` · `plan/src/region.rs` · `plan/src/lib.rs` · `cli/src/artifact.rs` ·
  `cli/src/world.rs`) — SERIAL and alone. Gate: goldens AND `.whylog` byte-identical. Size
  -GUESS L; map-then-execute (constructor census first).
- **`lane-fruit-arc`** (`26K:fruit-arc (née §0a)`; non-kernel; Sonnet; anytime; S).

## deferred-by-ruling — design-closed, build deadline-triggered (not r30)

- `30D` + `30J`, the predict-contract arc (deadline: the stdlib revival, a survival-authoring
  trial, or third-party publication — `30J:rul-family-vocabulary-build-is-not-an-r30-blocker`).
- The planner's probe mode (`p-x-intra-compound-plurality`) and alpha-rename
  (`d-alpha-rename-equivalence`, reserved).
- `FORFEITS:forfeit-certifier-trip-evicts-elisions` capture (trips observed in the field).
- `FORFEITS:forfeit-plain-sh-inclusion-analysis` (`30P` tiers 1 and 3: the splice and the
  single-stream paste) — the obvious entry point of the next language-surface round.

## human-gated-rulings — what each unblocks

On the burndown (unchanged): `unblock-starter-oracle-library` · `ratify-committee-sparing-fence`
· `design-report-only-refusal-scope` · `design-atmost-completion-speech` ·
`design-world-scope-surface` · `rule-incarnation-continuity-semantics` ·
`sweep-conductor-veto-pile`. From `30P` (`30P:open-rulings` + `review-adjudication-inputs`):
ratify `rul-guard-resolves-like-its-mutation` · ack the POSSIBLE/EXACT amendment of
`rul-partly-dynamic-operand-is-a-set` · rule the relative=controller / absolute=target class
selector (acked as a lean) · the controller-expectation/host-check pattern's UX and opaque
review · `ask-authored-pure-predict-may-site-loads` (parked). Non-blocking, from `30N`:
`30M:ask-wall-narration-ratify-or-mint (née §3)` · `30Ng:attn-render-refusal-feeds-the-spine`
· `30Nd:tc-plan-owns-its-source` · `30Na:tc-book-level-dot-locals-domain` ·
`30Na:tc-redirect-refusal-dead-or-owed` · `30Na:stop-spine-mode-is-durable` ·
`30M:ask-certification-row-shape` · the prelude-floor veto · the prose queue. Prose findings
for the human: `30P:fnd-squat-warning-contradicts-in-book-lift` (stale warning on the stage-3
path; the lint wants re-scoping to "a loaded oracle also defines this name").

## out-of-round — nothing here is r30-owed

`28Q:stage-iii-world-scopes` · the r26 reactive/capture + multi-host revival · the
ANALYZER-NEEDS transport cluster gated on `22H` · book-code splice + paste (forfeited, with
reds) · globs if the loop lane leaves no room · `notes/30Pc` (the opaque review's
BUILDER-lane half, unread by the conductor by law: hand it to the lane it names when its
builder reads `AGENTS.for-builders-only.md`).

## register-and-steering-debt — state after this re-cut

Done in this re-cut: FORFEITS header (`30P:rul-forfeits-carry-reds`) · the
`forfeit-book-dynamic-load-analysis` rewrite · `forfeit-plain-sh-inclusion-analysis` ·
`cli/CLAUDE.md`'s front-lift sentence · `spike/CLAUDE.md`'s floor-routing correction ·
`Research/README.md`'s r30 pointers. Still owed: ANALYZER-NEEDS `an-backing-selfframing`
("freezing remains owed" is false since `30Nc:req-backings-freeze-at-probe-boundary`) ·
`oracle/CLAUDE.md`'s dangling cite `FORFEITS:forfeit-whole-file-inertness-refusal` ·
`cli/CLAUDE.md`'s harness-contract lines (floor cases: top-level files only; `$0` shape is
platform-bound) — conductor-tier, at the next CLAUDE.md edit · the `30Kb` residue (wall
narrative operand · final-round typestate · the effective-reach prose case).

## the-schedule

```
sched-parallel-disjoint-lanes   (cli/src/main.rs is the one shared file)
   lane-loop-propagation         [M; no checkpoint; re-keying = hard stop]
   lane-emission-planner         [L; checkpoint after the tier-predicate map]
   lane-load-plane-precision     [L; checkpoint after the seat map — license-review-tier]
   lane-fruit-arc                [S; Sonnet; anytime]
   fold order: loop-propagation → emission-planner → load-plane-precision
sched-serial-constructor-reshape (alone, over the merged tip)
   lane-influence-carriage       [L; map-then-execute; byte-identical goldens + whylog]
sched-round-close-ceremony
   second-half adversarial crosscheck (priorities: the self-suppressed solve · the
   effective_invalidators node-check · the hoist predicates · the closed loop populations ·
   point-havoc's domains · the influence constructor census) → `307:veto-sweep-pile (née §5)`
   (human) → `gate:arc` → `CURRENT_ROUND` bump → the prose queue → fold to ai/main.
```

Brief riders, all lanes (the `30N` standing corrections plus today's): acceptance criteria
for evidence tests state the CFG shape they exercise · budget inline `//` and `///`
separately · a case minted to demonstrate a capability must observe it · `mise run fmt`
stages what it rewrites (reset or `git commit -- <pathspec>`) · scoped NEW-case bless is
permitted with scope verification, existing goldens never · `gate:full-quiet` ROUTES
`test:floor` when floor paths are staged, so a floor case must agree on both platforms ·
`rul-forfeits-carry-reds` · no floor-valid text is a parse violation · builders read
`AGENTS.for-builders-only.md` first and `notes/30Pc` where it names their lane. Opus for the
kernel lanes; Sonnet with the no-subagent clamp for the fruit arc.

## refuted-scout-claims — verified stale, so nobody re-chases them

Certifier replay "unbuilt" (`SolveReplay` exists) · `308:cr-carry-proof-answers-from-the-wrong-definition`
"unaddressed" (closed at the `308` burndown) · `30Kb:finding-aggregate-backing-underchecks`
"routed around" (repaired; `30M:repair-verification (née §8)`) · minispec root-import break
(repaired; the root is import-free by design) · `dorc-verify promote` "missing" (exists) ·
the `.`-resolves-against-sourcing-file veto (superseded by `30I:rul-dot-resolves-as-sh`).
