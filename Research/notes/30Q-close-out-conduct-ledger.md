# 30Q — r30 close-out conduct ledger (the `30O` schedule, executed)

> Tier: conductor ledger (Fable, fourth r30 conductor; session opened 2026-08-22; seat
> `.claude/worktrees/r30-conduct`, branch `ai/r30-conduct`, lineage tip at open `aabcc2d9`
> `(AI doc) Drop the discharged owed-ledger lines and refresh the human-gated residue`).
> Remit: conduct `30O:the-schedule` — `sched-parallel-disjoint-lanes` → `sched-serial-
> constructor-reshape` → (probably a successor's) `sched-round-close-ceremony`. Compression-
> resistant: lanes collapse to one line as they fold; rulings and findings get durable homes
> in the lane reports (minted by each lane as a letter-suffixed sibling of this note; cited here once they exist), this file carries pointers. Grades +SURE / ~SUSPECT /
> -GUESS.

## §1 — Sizing rulings at open (2026-08-22; human-acked "proceed")

- **`dec-map-then-execute-for-large-lanes`** — onboarding ≈ 110–120k tokens per kernel builder;
  an L lane in one context is a compression risk (human: catastrophic). The two L lanes
  (`lane-emission-planner`, `lane-load-plane-precision`) and `lane-influence-carriage` run as
  MAP builder → conductor ruling → fresh EXECUTE builder(s). `lane-loop-propagation` is one
  builder with one return-and-resume checkpoint after census-mint (the `settle.rs` seam is
  where a wrong shape re-keys). `lane-fruit-arc` is one Sonnet shot.
- **`dec-load-lane-splits-into-two-executes`** — the scout measured the lane larger than `30O`
  sized it: `${0%/*}` needs SYNTAX-crate decoding (the lexer discards the expansion operator
  and pattern — `WordPart::ParamComplex` is opaque), point-havoc is a funcenv-lattice change
  with four of five v0 domains uncoded, and the computed-`.` parse refusal must be un-murdered.
  EXECUTE-A: havoc × five domains · `${0%/*}` + symbolic `$0` + off-ramp lint · parser
  acceptance + post-analysis complaint machinery. EXECUTE-B: the three closure-custody xfails ·
  sentinel-value fidelity (+ test `176e0818`) · `mech-acquire-and-ship-plain-sh` LAST (it
  touches the planner lane's `cli/src/artifact.rs`).
- **`dec-globs-unscheduled-this-round`** — `30O`'s "if room remains" is not a plan; globs need
  the load lane's EXACT heads and the universal-meet design; excluded from the loop lane
  outright. `p-x-glob-load-members-are-order-unknown`'s assertion is retargeted (stays red).
- **`fnd-thirty-o-undercounts-shared-files`** — `analysis/src/value.rs` is touched by both the
  loop and load lanes; `cli/src/artifact.rs` by both the load and planner lanes. Both briefs
  carry line-range ledgers; acquire-and-ship sequenced last.
- **`fnd-xfails-drifted-from-rulings`** (scout audits, conductor-read): `p-x-load-operand-param-
  expansion-of-dollar-zero` ASSERTION-INCOMPLETE (slash-bearing spelling only; no slashless
  spelling, no lint) · `p-x-glob-load-members-are-order-unknown` ASSERTION-STALE (collision-only
  vs the ruled universal meet) · `dirname`/`cd-pwd` pins TEXT-STALE (cite the retired `ask-…`
  slug) · `p-x-sentinel-value-conjunct` metadata stale (ruled 2026-08-21; moot when greened) ·
  both planner pins TEXT-STALE only (retired `top-lift`/`in-paren` names; alpha-rename phrased
  as live). All routed into the lanes; horizons untouched (`30P:scheduling-truth`).
- **`fnd-oracle-constant-hoist-hole-confirmed`** — `pin_definitions` hoists a dorc-lang file's
  top-level `FOO=bar` above the book with no check against a same-named book variable (the
  injectivity census is funcdef-only). A correctness repair for the planner lane, red-celled.
- **`dec-thirty-pc-routing`** — the conductor may not read `30Pc`; a Sonnet produced routing
  metadata only (one finding; names only planner-lane files); the planner MAP builder lands it
  first. No unowned items.
- **`fnd-thirty-o-riders-were-stale`** (human-caught 2026-08-22) — `30O:lane-load-plane-precision`'s
  rider list predated the `30P` re-cut (three-state load identity; `dirname` beyond POSSIBLE;
  `rul-guard-resolves-like-its-mutation` as PROPOSED). Rewritten in `30O` to current truth; the
  load MAP builder told directly. The brief itself was drafted from `30P` and did not inherit them.
- **`fnd-load-plane-ruled-mid-flight`** — the human landed `30P:the-load-plane-stays-correct` on
  `ai/main` (`dd78b192`) after wave-one dispatch. Absorbed: `rul-rewrite-permission-is-derived`
  as a rider into load EXECUTE-A (an `Explicitness::{Literal, Evaluated}` marker on every
  resolution), load EXECUTE-B and planner EXECUTE (import edits and hoists only for an explicit
  literal operand; an EXACT-via-`$0` `.` stays verbatim, mirrored at its authored path; pinned
  now while vacuous). Deferred to `30O`: the blessed lift, the two standups, the single-stream
  `$0`-paste. UNSURE, raised to the human, direction NOT changed in flight
  (`ask-ship-explicit-targets-below-a-clobber`): the per-form paragraph's "havoc ⇒ nothing
  shipped" read against `an-load-exactness-reads-binding-state` (a `cd`/havoc `.` above makes a
  `.` non-EXACT) would un-ship a literal relative oracle `.` below `. /etc/os-release`; the
  lanes build the conductor's re-cut (mirror at the authored path, claim nothing). The conduct
  branch is rebased onto `dd78b192`; lanes fold onto the ruled text.
- **Human advice carried into every brief (typed 2026-08-22):** naming argued only for overloads
  and cross-domain glosses (a 10% lean; two-to-three-word names acceptable as the fix);
  whole-product unrepresentability statements for new/reshaped types; parser work is cheap and
  throwaway — fix the obvious while fingers are in, keep the rich-AST/locator seam, redden nothing.

## §2 — Lanes

| lane | branch | note | builder shape | status |
|---|---|---|---|---|
| `lane-loop-propagation` | `ai/r30-lane-loop` → `-2` | loop lane report | Opus ×2 (census+value plane; then the licence) | FOLDED 2026-08-22 (rebased, ff; conduct tip `2ab9ba89`; steering + FORFEITS re-cut `cff47b42`). Licence built; five `loop30-*` goldens minted; zero existing drift. Two genuine holes found by the second builder: member binding under a rebinding loop extent (under-execution; fixed with `loop_extent_rebinds`) and `ship_auto`'s subject gate refusing every member ship (fixed with `node_subjects`). Deviations all ENDORSED (real backings map threaded — region facts reach survival; per-candidate argv on the vouch lift; XFAIL fixtures that were silently wrong, repaired). Both lane branches contained; their harness worktrees are the human's sweep. |
| `lane-emission-planner` | `ai/r30-lane-planner` → `-exec` | planner lane report | Opus MAP → ruling → EXECUTE | MAP FOLDED-IN-BRANCH `0c38045d` (30Pc repair + 2 evidence cells landed); RULED; EXECUTE DISPATCHED 2026-08-22 |
| `lane-load-plane-precision` | `ai/r30-lane-load` → `-a`, `-b` (parallel) | load lane report | Opus MAP → ruling → EXECUTE-A ∥ EXECUTE-B | MAP in-branch `d2b47654`. **B GREEN at `4d4bea06`, FOLD HELD** until the planner lands (B's `select`/`bundle_files`/`mirrored_files`/`inline_imports` now take the snapshot — four non-additive signatures in the planner's file; B rebases over the planner). Five pins promoted after falsification; plain-sh inclusion ships (`PlainInclusion` derived from marker absence; `Loadable::{Program, Included}`; single-stream refuses by name); sentinel VALUE compared; `unset -f` repaired in the index (endorsed: reaches the frameless consumers); regional pin re-homed at the cli tier (endorsed: anti-masking). OWED, banked: the six production resolution seats still enumerate candidates per FILE, so `p-x-definition-grade-keying`'s product value is not end-to-end — a red + possibly a narrow FORFEITS row at the load fold. A still out. |
| `lane-influence-carriage` | — | influence lane report | Opus EXECUTE over the merged tip (scout census banked) | WAITING on the fold |
| `lane-fruit-arc` | `ai/r30-lane-fruit` → `-2` | fruit lane report | Sonnet ×2 | FIRST SONNET HUMAN-STOPPED at `306a7a00` (items 1–3 landed; item 1 detector unwired; gate unconfirmed; item 4 paragraph → prose queue; FOUND a latent `dorc_oracle::predict` lexer bug misreading `{1..N}` as a group opener on the loom lift path — kernel batch); CLOSER DISPATCHED 2026-08-22 |

Briefs: scratchpad `brief-common.md` + `brief-<lane>.md`; scout maps `scout-<lane>.md` (session
scratchpad; the load-bearing findings are copied into the briefs and into this ledger).

Fold order: loop → planner → load, rebased onto `ai/r30-conduct`; then influence serially.

## §3 — Rulings owed at checkpoints (open)

- loop seam: RULED — member-major ordering everywhere observable (record index AND witness); `IterationSlot` keeps the member ordinal; duplicate `(site, fact)` keeps refusing; the count>1 trigger keys on closure; licence reds minted before the seam (`rul-forfeits-carry-reds`).
- planner map: RULED 2026-08-22 — `rul-a-loaded-definitions-placement-is-its-load-position` endorsed (book-loaded closure defaults in-place; no second hoist; residual withholds); T2 narrowed (T2a role-munge; T2b → T3 with the collider named) pending the HUMAN on the typed ladder vs reserved alpha-rename; `sink`-inside-the-paren ruled admissible (veto-eligible); tier cells = unit pins + disclosure needles; squat cell roster entry GRANTED; why-world re-derives the Selection, never a new durable field. FINDING for the human: the artifact already carries a book-loaded bundle at the `.`, so the preamble hoist was a SECOND copy — the hole is live (gate-6 dual-rail failure), fixed by step 3 of the execute order.
- load map: RULED 2026-08-22 — D1 pointwise havoc endorsed (frames = subshell scopes to confirm;
  a function-body `.` havocs the global frame); D2 cwd clobbers endorsed as machinery but RE-CUT:
  cwd-⊤ never costs acquisition or mirroring, only binding authority (the cwd-⊤ domain is a
  conductor-tier lean in `30P`'s review section, not typed law, and as mapped it would regress
  `. /etc/os-release` books to dying at their relative oracle `.` on the host); a subshell `cd`
  clobbers nothing outside; `$0`-headed operands get ONE cwd-sensitivity switch with the
  conservative default (the flagship `load30-point-havoc-and-script-relative` stays red) pending
  the HUMAN on `tc-dollar-zero-is-script-anchored` (invoke the shipped plan by absolute path so
  the host's `${0%/*}` is cwd-immune); the slashless `$0` spelling is a LINT spelling; the
  computed `.` keeps today's outcome at exit 17 with only the tier moved; the `case $0` form gets
  a new r31 red; the `30Pc` repair is skipped in EXECUTE-B (it landed on the planner lane).
- influence (before dispatch): rename the influence `Grade` (collides with claim-tier `core::Grade`) ·
  render/output sinks as explicit `untracked` adapters this lane (conductor lean) ·
  `certifier_trip` demotion path's influence meaning.

## §4 — Ack-ledger (only what the human TYPED counts)

2026-08-22: "Acked; proceed" on the four-lane wave-one shape + map-then-execute + ceremony
roll-over lean + the `30Pc` routing method (no objection raised). Naming/typing advice and the
parser lean as recorded in §1.

2026-08-22 (successor session, on the §5c plan): the root checkout's seven dirty files are the
HUMAN's in-flight ruling work — no touch · item 4 (steering/register edits) PROCEED, tooling
steer "fix, don't document … only if a Sonnet can fix it" · item 5 (loop residue) DISPATCH ·
item 6 (`lane-influence-carriage`) DISPATCH THE MAP ONLY; EXECUTE waits on a typed ack ·
everything unmentioned (the two folds, the execute-deviation adjudication, the ceremony) is a
do-nothing HOLD. Basing the two new lanes on the green repair tip was the conductor's read of
"dispatch", not a typed instruction.

## §5c — SUCCESSOR SESSION (2026-08-22; supersedes §5b's and §5a's order)

**Returns banked, UNADJUDICATED (held):** fold-repair `ai/r30-conduct-repair` @ `eae684b1`
(4 commits over `0c073fd2`; the three hand-merged hunks verified correct; the two diagnosed
reds plus two fold-mechanics reds the lib build hid — stale test seats for `select`/`closure_for`,
a full-profile-only `collapsible_if` — fixed; `both gate:full-quiet` green, `both test`
2642/2638, xfail census 13 live / 1 reserved, no expired group; the `load30-script-relative-lints`
golden moved exactly the ruled 9-out/1-in) · fruit closer `ai/r30-lane-fruit-2` @ `9b97c30d`
(green both legs foreground; its lane report's `§close` is on the lane branch; ONE deviation: a plain `~`-leading unmodeled line as
the paste-hygiene round-trip witness instead of the sketched oracle+mocks shape).

**Open adjudication register — the execute-lane deviations the collapse folded mechanically,
each still OPEN (the `30Q` §2 table's "endorsed" marks for load-B's two stand; nothing else is
ruled):** planner `30Qb:dev-four-goldens-moved-not-one` · `dev-residual-withholds-placement-not-the-vouch`
(= `tc-uncarried-source-still-guards`; conductor lean: as built) · `dev-hoist-action-not-built`
(T1/T2a red; the hoisted `.`-line spelling is `tc-hoisted-dot-line-spelling`, human) ·
`dev-a-tier-pins-not-in-the-type-commit` · `dev-tier-needles-are-not-expressible` ·
`dev-two-mise-tasks-added` · `fnd-book-set-roots-resolve-and-must-stay-explicit` (the `30P`
per-form gloss wants the definition's words) · load-A `30Qc:dev-prelude-floor-keeps-the-absorbing-top`
(rides the human's clobber sentence) · `dev-errexit-is-a-lexical-operand-test` ·
`dev-cwd-cell-re-pointed-to-the-ruling` · `dev-lint-severity-follows-the-map` (builder lean:
`script-relative-load-dies-slashless` at NOTE) · `dev-source-of-dynamic-target-is-retired` ·
`dev-parser-span-residue-not-taken` · load-B `dev-lift-arity-lands-but-the-seats-still-enumerate-per-file`
(a licensure act — the six per-FILE seats; wants a ruling or a red) ·
`dev-inclusion-role-is-derived-from-the-marker` · `dev-select-takes-the-snapshot` ·
`dev-book-reached-admits-marker-free-targets` · repair-lane: the header nit and the dangling pin
cite (both routed to `lane-loop-residue`) · fruit: the witness-shape deviation.

**Dispatched from `eae684b1` (not a fold — basing):** `lane-influence-carriage` MAP on
`ai/r30-lane-influence-map` (Opus; its lane report takes the next free `30Q` letter, `§map`; red cells only; the three conductor leans
it must confirm or refute: `lean-account-is-non-optional-with-a-typed-untracked` ·
`lean-render-sinks-are-untracked-adapters-this-lane` · `lean-certifier-trip-demotion-joins`;
`stop-spine-mode-is-durable` rides it and may trigger the quarantine file's review at MAP time)
· `lane-loop-residue` on `ai/r30-lane-loop-residue` (Sonnet; six tasks: the Replace-asserting
red for `forfeit-cell-blind-self-reach-walls-loop-siblings` · `plan::member_argv` →
`argv_of_inline_site` · the dangling `p-x-computed-dot-parses-and-havocs` cite · the
`load30-script-relative-lints` oracle header · the shared `~/.local/state/hk/output.log`
(per-worktree via `mise.toml` env iff one-line) · `oracle/src/closure.rs`'s stale `ParamComplex`
header sentence). Briefs: session scratchpad `brief-influence-map.md` · `brief-loop-residue.md`.

**Steering and registers applied on `ai/r30-conduct` (item 4):** `syntax/CLAUDE.md`
(`syntactic-top-triggers` source clause retired; `tn-coarse-subst-provenance` consumer) ·
`analysis/CLAUDE.md` (`funcenv-reads-source-literal-plane-only` rider ·
`rul-havoc-is-pointwise-never-the-stack` · `rul-exact-is-not-explicit` ·
`rul-acquiring-bytes-is-not-modelling-them` · `rul-guarded-source-compares-the-sentinel-value`) ·
`oracle/CLAUDE.md` (the closure rationale restated + `SiteFrame` rider · the dangling FORFEITS
cite dropped · `rul-a-removal-clears-what-is-indexed`) · `cli/CLAUDE.md` (`PlainInclusion`
rider · the two rewrite gates · `floor-cases-see-a-modelled-dollar-zero`) · `plan/CLAUDE.md`
(placement inheritance appended to `pinned-definitions-are-the-artifact's-binding`) ·
`spike/CLAUDE.md` task list (`bless:case`, `loom`) · ANALYZER-NEEDS (`an-backing-selfframing`
freeze; `an-name-observation-census`) · FORFEITS (the `case-over-dollar-zero` red; the plain-sh
row's landing evidence) · `30O:register-and-steering-debt`.

**Findings this session:** `fnd-one-explicitness-predicate-two-seats` — `cli::artifact::operand_is_explicit`
(AST word) and `funcenv::ResolvedHead::explicitness()` (resolution) both answer the rewrite
question after the fold; the cli seat should read the marker (`30O` debt) · the harness
auto-loads every `CLAUDE.md` under a tree on a conductor's first edit there (~40k tokens this
session) — edit steering in one sitting, never piecemeal · the Windows preflight reads
`vmmemWSL`'s unreturned page cache as pressure, so `both` runs the WSL leg FIRST on this box
(the repair builder measured it; `spike/CLAUDE.md preflight-bounds-before-spend` says the
opposite and is owed a one-line correction at the next steering edit).

**Held, in order, once the human acks:** (1) ff `ai/r30-conduct-repair` then rebase+ff
`ai/r30-lane-fruit-2` and the residue lane onto `ai/r30-conduct` (this session's doc commits
rebase on top of the repair tip) · (2) the adjudication register above, banked here · (3) rule
the MAP, then EXECUTE on the human's typed ack · (4) the ceremony per `30O`.

## §5d — the two lanes back; the MAP adjudicated (2026-08-22, conductor rulings, veto-eligible)

**`fnd-windows-disk-crisis-mid-lanes`** — `C:` hit 0 bytes free during both lanes (a
`mise run test` died in `rustc`'s LLVM and the harness temp fs `ENOSPC`'d); both Windows gate
legs were REFUSED by preflight (`1.9 GiB free … needs 4.0/14.0 GiB`), WSL legs green.
Inventory: 161.6 GiB in eighteen worktrees on this leg, fourteen of them folded `agent-*`
lanes at 7–10 GiB each. The conductor reclaimed ONLY its own worktree's cache (`mise run clean`
in `r30-conduct`, 16.3 GiB) and re-resumed both builders for their Windows legs; everything
else is the human's sweep (`doctor-inventories-never-reaps`). Steering correction owed:
`spike/CLAUDE.md preflight-bounds-before-spend` says "run the Windows leg first"; two builders
measured the opposite (the WSL leg's cache release is what let Windows recover) — re-say at
the next steering edit.

**`lane-loop-residue`** — `ai/r30-lane-loop-residue` @ `c5f63a72`, six commits: the XFAIL case
`loop30-cell-disjoint-siblings-would-replace` (target-tense `expected.ran`/`expected.out`
hand-authored from the green sibling; registers as expected-XFAIL) · `argv_of_inline_site` ·
the dangling pin cite · the fixture header · **`HK_STATE_DIR` per worktree in `mise.toml`
`[env]`, both legs** (a real hk knob, measured to relocate `hk.log` + `output.log`; needed a
`\`→`/` replace because `config_root` is backslashed on Windows and the existing
`CARGO_TARGET_DIR` `split('/')` silently no-ops there) · the `closure.rs` header. Deviations:
none. FORFEITS row `forfeit-cell-blind-self-reach-walls-loop-siblings` REDS → that case, at the
fold. Windows leg pending re-run.

**`lane-influence-carriage` MAP** — `ai/r30-lane-influence-map` @ `ba0feb05`; its report is
on the lane branch under the next free `30Q` letter (cited here once it folds). Rulings, each
re-derived:
- `fnd-the-conversion-changes-no-answers-at-v0` ACCEPTED — both drivers feed the phase
  unconditionally and every Spine record is minted downstream of intake, so the lane is
  plumbing + sealing + `untracked` + spelled joins. That is the SHAPE `306b` §10 asked for;
  byte-identity is near-trivial. `fnd-per-route-difference-is-unreachable-at-v0` ACCEPTED (the
  `by_fact` key-set derivation would LAUNDER through `freshness`; pinned red, not claimed).
- `lean-account-is-non-optional-with-a-typed-untracked` CONFIRMED as sharpened: `InfluenceAccount`
  + the paired `core::spine::Account<T>` → `OperandAccount<T>`; `Untracked` is an ARM of the
  account, never the reserved fourth `Grade` (`dec-untracked-is-not-gradation` ENDORSED);
  total chain `Authored ⊏ HostInfluenced ⊏ Untracked`, `join = max` — ENDORSED chain-now
  (`tc-untracked-sits-above-influenced` → chain; a product only if `306b` §1c lands).
- `lean-render-sinks-are-untracked-adapters-this-lane` REFUTED, REPLACEMENT ENDORSED: `Plan`
  CONVERTS (`project_plan` joins the accounts it reads), so `render_sh`/`render_apply` accept an
  accounted object with no adapter and no influence-aware render; the four probe-side plans
  RESTRICT; `Selection` RESTRICTS, `with_plan` JOINS. Conductor's mistake: the lean named the
  sinks without tracing that their input is the accounted `Plan` — a scouting gap the MAP
  checkpoint exists to catch; no recurrence guard beyond "trace the sink's input type".
- `lean-certifier-trip-demotion-joins` CONFIRMED; `tc-spine-record-mut-accessors-survive`
  RULED: the `&mut` accessors go; the demotion becomes a named Spine method taking the joined
  account (R8).
- `tc-accounting-reads-are-not-gating` RULED (conductor-tier): accounting is the ONE exempt
  consumer of influence reads — the account is decision-inert by TYPE (no conversion into any
  license-plane input compiles; the `two-plane-aid-law` posture), so reading which inputs were
  influenced selects no code path about the plan. Steering sentence at the fold.
- `tc-load-decisions-read-authored` — conductor LEAN, raised to the HUMAN (a typed `30I` rule
  against the human-ruled `306b` §10): the account joins at the SEMANTIC mint, and a load
  decision's semantic mint is `funcenv`, pre-contact — recording it onto the Spine inside
  `record_new_arm` is transcription, not a mint, so the intake-gated control path does not
  join. Lean: AUTHORED. One typed line settles it; EXECUTE's R4/R9 carry the ruling.
- `dec-horizon-is-scheduled-not-deferred` — the builder's reading of `Horizon` is right and the
  brief's was wrong (`Deferred` records a slip that has not happened); ENDORSED, conductor's
  brief error.
- RED-cell (a) lexical (ACCEPTED — the seat is private; the value pin waits on a settlement
  fixture) · RED-cell (b) sited in `plan` (ACCEPTED — `core` stays dependency-free) ·
  `fnd-two-drivers-compute-one-fact-twice` → the two-seat split in `results` ENDORSED ·
  `fnd-one-mint-fence-misses-a-qualified-spelling` → R3 rider: widen the needle ·
  `fnd-provenance-attach-raises-nothing` ACCEPTED as a doc-stated property.
- **`stop-spine-mode-is-durable` (R10) — GATED on the human.** The MAP found `mode` is an
  EQUALITY-CHECKED replay conjunct (`matches_claims` requires `whylog-replay`), so writing the
  truthful value un-replays every production durable unless the check moves in lockstep.
  Option (c) — narrow `SpineInvocation.mode` to a closed enum whose one inhabitant spells
  `whylog-replay`; zero durable bytes move, `mode_valid` and `matches_claims` untouched; the
  truthful value becomes a later reviewed one-arm widening — is the conductor's lean too
  (~SUSPECT it does not fire `rul-durable-contents-reviewed-before-design`, since neither what
  is persisted nor what re-ingestion consumes moves). Per the rule, the HUMAN rules by
  preference; `/opaque-review` only if they defer it. R1–R9 do not depend on it.
- Steering/register proposals 1–7 (two `core/CLAUDE.md` bullets, the `plan` mint rider, the
  cross-crate pointer, ANALYZER-NEEDS `S`→`B` with the residue stated, NO FORFEITS row, the
  `rul-rc-reaches-genkill-only-through-decisions` "expected out of the influence round" sentence
  to re-say) — APPLIED AT THE EXECUTE FOLD, when the design has firmed.
- EXECUTE shape: ONE Opus builder over the green merged tip, rows R1–R9 in the MAP's order
  (R2 before R3 is load-bearing), R10 only with the ruling; gate = goldens + the nine
  loom-embedded `.whylog` transcripts byte-identical + `bless:dry` clean + `spine:baseline`
  before/after handed to the conductor. Size -GUESS L; one return-and-resume checkpoint after
  R3 (the stamp gone, species sealed) is worth its cost.

## §5b — CONTEXT COLLAPSE (2026-08-22, after the restart; supersedes §5a's order)

The conductor spent its remaining window folding planner → load-A → load-B by hand (three
real conflicts; `nameuse`/`artifact`/`funcenv` repairs) and cannot conduct further. All four
kernel lanes are FOLDED on `ai/r30-conduct`; the merged tip is not yet gate-green (two
diagnosed fold reds). A fold-repair builder and the fruit closer are running; their reports go
to the SUCCESSOR. Everything the successor needs that is not here is in the root
`_tmp-r30-conduct-collapse.md` (builder IDs, the reds, the hand-merged hunks, the owed
steering list). Lesson for the record: a dispatch hold must not become "do the builders'
work yourself" — fold conflicts are builder work; a conductor's context is for adjudication.

## §5a — RESTART STATE (human hold, 2026-08-22: no further subagent wakes; harness restart pending)

In-flight lanes run to completion and are adjudicated as they land; NOTHING is dispatched or
resumed until the human restarts the harness. Dispatch-ready afterwards, in this order:

1. **fruit closer STRANDED** — `ai/r30-lane-fruit-2` (2 commits over `306a7a00`: the
   paste-hygiene diagnostic wired + its catalog entry) ended its turn awaiting its OWN
   backgrounded bless/gate (the `foreground-final-verification` failure mode). Its round-trip
   case `emitted-line-unsafe-for-paste-round-trip.loom` was UNTRACKED in a reapable harness
   worktree; a copy is banked in the session scratchpad (`fruit2-uncommitted-…loom`). A fresh
   Sonnet, branched from `ai/r30-lane-fruit-2`, commits that case (scoped bless), runs
   `mise run both gate:full-quiet` FOREGROUND, appends `# §close` to the fruit lane report (on the lane branch). Then the lane folds.
2. Fold whichever of planner-exec / load-a / load-b have reported green (loop is FOLDED; order:
   planner → load-a → load-b, each rebased onto `ai/r30-conduct`; load-b reconciles the
   `Explicitness` marker seam with load-a and the planner at fold). Small post-restart items
   banked from the loop fold: mint the Replace-asserting red for
   `forfeit-cell-blind-self-reach-walls-loop-siblings`; rename `plan::member_argv` →
   `argv_of_inline_site` (near-collision with `ValueFlow::member_argv`, flagged by the builder);
   `tc-disagreeing-region-renders-cant-tell` (a region whose routes disagreed renders
   `probe: cant-tell` — aid polish) → the prose queue.
3. `lane-influence-carriage` EXECUTE over the merged tip (scout census in the scratchpad
   `scout-influence-census.md`; rulings owed first: rename the influence `Grade`; renders as
   explicit `untracked` adapters; the certifier-trip demotion path).
4. The ceremony (`30O:sched-round-close-ceremony`) — likely a successor's.

Human-owned, unchanged: `tc-dollar-zero-is-script-anchored` · `ask-ship-explicit-targets-below-a-clobber`
· `tc-t2-is-narrower-than-the-ladder-says` · the prose queue (incl. the fruit lane's item-4 paragraph).

## §5 — Handoff pointers (for a successor, if the ceremony rolls over)

`30O:sched-round-close-ceremony` priorities stand; add: the loop lane's iteration-aware route
decision and the planner's hoist-legality predicate to the crosscheck list. Stale worktrees/
branches are the human's sweep (`LIVING_STATUS`); `worktree-sol-adversarial-30M` is deletable
once EXECUTE-B lands `176e0818`.
