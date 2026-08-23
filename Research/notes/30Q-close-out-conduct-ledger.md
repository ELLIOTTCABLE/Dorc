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

2026-08-22 (later, both lanes green): NACK on `wsl --shutdown` (the human cycles WSL on their
own time; the suggested compaction invocation was also wrong for Windows Home) — the old WSL
caches MAY be cleared, by a Sonnet · tooling fixes are SAVED UP for ONE tooling Sonnet later
(cross-platform — the human also builds on macOS; Windows/WSL magic capability-detected, mildly
dumb, minimal upfront effort; scheduling is the conductor's) · design-level questions go to the
root `_tmp-human-burndown.md`, not ruled in chat · the FOLDS are ACKED (the human swept ~90 GB
on `C:`) · the influence EXECUTE ack is still pending.

2026-08-23: the human landed `30P:law-no-unsoundness-below-a-blind-act` and ff'd `ai/main`;
"if you still think we're ready for influence, you're green to dispatch; else … retrofit the
unsoundness" — conductor chose the retrofit (dispatched). Then, TYPED: the influence EXECUTE
is HELD for an explicit ack; everything not behind it (the retrofit lane, its fold, the
tooling Sonnet, ceremony prep) stays greenlit.

2026-08-23, TYPED: "`. /etc/os-release; . ./relative.sh` is inherently, unsolvably unsound …
The only sound remedies are admin-sourced." — `30Q:ask-ship-explicit-targets-below-a-clobber`
is SETTLED: nothing shipped below a blind act; the retrofit's E2 stands as built; no further
notes or churn on it.

2026-08-23, RELAYED by the sibling session `r30-sit-blind-loads-unsoundness` (human-typed in
that window; banked here as rulings for the influence EXECUTE — the EXECUTE ACK ITSELF is still
owed as a line typed to THIS session): R10 `mode` — "fix it, I don't care how", not an
opaque-review trigger (format, not content); (c) stays the lean · the durable account export is
BUILT ALL THE WAY (View + writer + reader + e2es) then DISABLED behind one switch with those
e2es swapped to xfails before lane-close; the human enables it post-review · "influence never
decides whether a line elides" NACKED as law — true at v0 (no consumer), a future decision
consumer is a typed human act (per-host contamination gating a revived cross-host planner is the
strawman); never say "decision-inert by type" · the §6b exemption is a NARROW window:
`InfluenceAccount::of_phase` is the ONE phase→account transition, caller-count fenced ·
no gradation, and the fold report enumerates every `untracked` adapter seat (the discipline
test's product) · no render change · per-host influence NOT built; `HostInfluenced` keeps the
phase, its `()` payload reserved as the host-scope slot · no affine clean-of-host witness; a
lexical fence on `authored_before_contact()` callers · load decisions: LEAN — move the
`SpineLoadDecision` write pre-intake if cheap, else Authored DERIVED from funcenv inputs,
reported as lean-applied · the `rul-rc-reaches-genkill-only-through-decisions` sentence is NOT
struck; re-homed at the fold's steering edit to the sibling's TODO-ADDTL row
`rc-vs-genkill-permanent-law` (uncommitted in the root checkout, the human's to commit) ·
FLATTENING: post-reingest influence is report/why-plane only; durable ingestion never
rehydrates an account into a decision; the why-driver's widen is the DEFINITION. All eleven are
in the EXECUTE brief (scratchpad `brief-influence-execute.md`, rows R10–R11 added).

2026-08-23, TYPED to this session: "You're clear to proceed; dispatch influence-execute when you
please" — the EXECUTE ACK. Terms: the conductor's only job is DISPATCH, RECEIPT, and LEDGERING
until told otherwise — no tests or gates run by the conductor, bless/`gate:arc`/substantial
edits held for a successor or a later ack; context suspected heavy. Then: influence EXECUTE
dispatches SERIAL — after `lane-third-blind-act` returns and folds — to keep the history
linear (no merges while context-constrained). The sibling's `§5g` (read by focused diff) stands
as banked; the retrofit's steering re-says are committed with this entry.

**`lane-third-blind-act` FOLDED (2026-08-23; conduct `66954cc2`)** — `Cfg::splice_refused` is
recorded at the ONE seat that mints the refusal diagnostic (`Builder::refuse_splice`; EIGHT arms,
not four — seven `CFG_INLINE_REFUSED` + the depth-2 positional Note, all recorded; the silent
`?`-returns stay ordinary `Opaque`, which is what keeps the two goldens still); the clobber seed
reads it; `p-x-an-unspliceable-call-havocs-the-cwd` promoted; a control cell pins the WRONG seed
(`call_body_sites(id).is_none()`) red; zero goldens moved; both legs green; census 21 live /
1 reserved, no expired group — the end-of-r30 pin set is green. Deviations all ACCEPTED (the
Note arm joins the set — withholding direction; the arms refactored onto one seat; the control
cell; the stale `load_sites` doc re-said; the base two doc commits ahead). OWED, conductor-tier,
one line: `analysis/CLAUDE.md rul-havoc-is-pointwise-never-the-stack` still says the third act
"is pinned red until the CFG's refusal set is queryable" — now false; `30Qf` §third-blind-act
carries the replacement. `tc-*` for the successor/human, none blocking: a HELD cross-file oracle
body is modelled as text, not shell state (its own `cd` is as invisible as a refused call's —
oracle-contract residue, or `seam-interproc`'s cross-file half) · an over-budget wrapper is now
also a CARRIAGE knob (`main() {…300 lines…}; main; . ./x.dorc.sh` ⇒ nothing shipped — rides the
typed ship-nothing ruling; zero corpus books) · a refused body's OWN `.` lines keep their
authority (the clobber closure walks from `cfg.entry()`; the pre-existing `vacuous-entry-fold`
seam). **`lane-influence-carriage` EXECUTE DISPATCHED** from `66954cc2` on
`ai/r30-lane-influence-exec` (Opus; checkpoint after R3; R10 = narrow `mode`; R11 = the durable
export built-then-disabled). **R3 CHECKPOINT (2026-08-23; `4d802e94`; `30Qd` §execute-checkpoint):**
R1–R3 green, every golden byte-identical; the BEFORE `spine:baseline` committed. Rulings: the
row boundaries moved (sealing forces the mutating seats into R2: R8, R9's two driver seats, the
five authored postures, R4's type change; R3 absorbed a slice of R6) — ACCEPTED, each row
independently green; `dev-carriage-census-needs-a-third-arm` (`Unminted` for the four
writer-less species) ENDORSED; `dev-run-identity-grouped-out-of-the-invocation-mint`
(`core::spine::RunIdentity`; no durable byte moves) ENDORSED; the builder's extra fence
`every_untracked_adapter_is_enumerated` ENDORSED as an INVENTORY (asserts the named list,
never emptiness forever). RED-cell (b) promoted at R3. R11's durable review: the human's typed
flow (build all the way → disable → reviewers chew on it → human enables) is the review
ordering for that row; built last on that basis. Resumed for R4–R11.

**`lane-influence-carriage` FOLDED (2026-08-23; `08027756` rebased over the sibling's four
ruling commits, ff'd; conduct `1d4ba081`; report `30Qd` §execute).** R4–R11 one commit each;
`spine-baseline-{before,after}.txt` byte-identical over 291 cases — `fnd-the-conversion-changes-no-answers-at-v0`
measured; nothing blessed; `bless:dry` clean; both legs green; census 19 live / 1 reserved.
Adjudications: `fnd-the-licence-census-undercounted-by-two` (EIGHT licence seats, not six —
`prove_inline_replaceable`/`prove_inline_query_replaceable`; the private field found them) —
the `plan/CLAUDE.md` rider says eight · the `untracked` INVENTORY = three seats
(`decide_region`'s open/non-corresponding population; `DurableAccount::rehydrated`'s
absent/unknown floor, ruled; that floor's test) — the discipline test's product, for the human
· `tc-load-decisions-read-authored`: the STRUCTURAL half landed (`mint_load_decisions` runs
pre-intake; `record_new_arm` only transcribes) — lean-applied, ENDORSED · R10 built as (c) ·
R11 built whole and shipped OFF (`ACCOUNT_EXPORT: bool = false`; `DurableAccount` is the
flattening as a type with no accessor back to a live account; the round-trip pinned
`p-x-durable-account-export-is-enabled` at `r31:kernel-punt-glance`; absent/unrecognised read
HIGHEST) — deviation `native test, not a loom` ACCEPTED (a loom would bless switch-ON bytes
production never emits) · `fnd-the-lint-tier-that-check-does-not-cover` (seven whole-workspace
clippy errors surfaced only at `bless:dry`; one a genuinely dead field) → a brief rider for
long conversion lanes: `mise run clippy` per row. **OWED TO THE SUCCESSOR/HUMAN** (conductor
under a no-substantial-edits hold): the eight steering deltas in `30Qd` §execute (the two
`core/CLAUDE.md` bullets with "no consumer at v0; a future one is a typed human act" — never
"decision-inert by type"; the `plan` mint rider at eight seats; the `spike/CLAUDE.md` pointer;
ANALYZER-NEEDS `an-host-influence-carriage` `S`→`B` with the per-route residue stated; the
rc/genkill re-home is done) · the one-line `analysis/CLAUDE.md` third-blind-act correction ·
R11's review must clear BEFORE `ACCOUNT_EXPORT` flips (the pin holds the door) · the ceremony
(`30O:sched-round-close-ceremony`: crosscheck with the priorities in `30O` + the hoist
predicates + the loop lane's iteration-aware route + the influence constructor census + the
blind-act seed; the `307` §6 veto register; `gate:arc` from the populated branch; the
`CURRENT_ROUND` bump; the prose queue; the human's ff of `ai/main`).

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
else is the human's sweep (`doctor-inventories-never-reaps`). ROOT CAUSE
(`fnd-wsl-preflight-reads-the-vhdx-not-the-host`, measured): the WSL leg's preflight reads
the sparse `ext4.vhdx`'s ADVERTISED capacity (`/dev/sdd` 1007G, "789G free") while that file
sits on `C:` at 182 GB and grows with every cold WSL build and never shrinks on its own; it
holds ~24 `dorc-wsl-target-*` caches at ~6 GB (nine orphans from reaped worktrees, ten more
from folded r30 lanes). Fix owed (a Sonnet, on the human's word): under `WSL_DISTRO_NAME`
bound on the MIN of the cache volume and the host volume holding the vhdx; `doctor` reports
the vhdx's on-host size. Recovery is the human's: delete the orphan caches inside WSL, then
`wsl --shutdown` + compact the vhdx. CONDUCTOR MISTAKE, recorded: the two briefs' "WSL leg
first" rider was BACKWARDS — `preflight-bounds-before-spend`'s Windows-first law stands (the
RAM refusal a builder then hit was its own WSL build's `vmmemWSL`); the rider was written off
one builder's anecdote against a measured law. Recurrence guard: a steering law is overridden
only by a measurement that names the law, never by a report that happens to contradict it.
The law's parenthetical "(no `.wslconfig`, no auto-reclaim)" IS stale — `~/.wslconfig`
(2026-08-18) sets `memory=20GB` + `autoMemoryReclaim=gradual`, which is why the refusal
drained in ~5 min; re-say as "gradual reclaim, minutes" at the next steering edit.

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
- Gate: `ai/r30-lane-influence-map` @ `ba0feb05` GREEN on both legs (the red cells' xfail
  semantics measured on both platforms); its worktree cache returned. `lane-loop-residue`
  @ `c5f63a72` GREEN on both legs, cache returned.
**Folds (acked 2026-08-22):** conduct's doc commits rebased onto the repair tip; residue and
MAP rebased and ff'd (linear; tip `e30910e6` before this commit). Fruit CONFLICTED on the
generated `catalog_lock.rs` (both sides published codes): aborted, handed to a Sonnet fold
builder on `ai/r30-lane-fruit-3` — regenerate both locks from the committed cases, scoped
re-bless of drifted FRUIT cases only, then `both gate:full-quiet`, which is the merged tip's
first whole-gate. The two design questions are in the root `_tmp-human-burndown.md`.

**Tooling-Sonnet queue (saved up per the human; ONE lane, later; cross-platform, macOS-safe,
capability-detected, mildly dumb):** (1) the WSL preflight bounds on MIN(cache volume, the host
volume holding the vhdx — `/mnt/<drive>` of the worktree) · (2) `doctor` reports the vhdx's
on-host size instead of "Windows keeps none of these" · (3) clear the orphaned
`dorc-wsl-target-*` caches (worktree gone) and the folded-lane caches, with a containment check
per cache, never `wsl --shutdown` · (4) `wsl --cd X -- mise …` misses PATH (builders needed
`bash -lc`; `sh -lc` dies on the zsh profile) — settle the one agent spelling and say it in
`spike/CLAUDE.md wsl-cd-not-bash-lc-cd` · (5) the WSL-leg `doctor:unused` took >2 min on a
warm-cached worktree — measure, fix if cheap · (6) [human-typed 2026-08-22] `lint:docids`
resolves citations over the WHOLE tree, so a complete commit is blocked when a neighbour's
work is incomplete (this session: `30Qd`/`30Qe` cited from the ledger before those notes were
on the branch — worked around by rewording, the wrong fix). It must map both FROM and TO
staged files only — run inside hk, which owns staging — and carry an escape hatch
(`NO_LINT_DOCIDS=1` or similar) honoured by the step itself, so `--no-verify` is never the
reach.

## §5e — the blind-act law lands; the retrofit precedes influence (2026-08-22)

The human landed `30P:law-no-unsoundness-below-a-blind-act` (+ `notes/30Pd`, four FORFEITS rows
with pencilled reds, the `[ -f ]` cwd-gate text in `analysis/CLAUDE.md`) on top of the conduct
tip and fast-forwarded `ai/main`. Against it, verified at the seats: (1) +SURE
`cli::artifact` re-points/inlines on `load.explicit` ALONE — no EXACT/havoc check in the file —
so a literal `.` below a `cd`/havoc is rewritten; (2) ~SUSPECT the decidable set's `[ -f ]`
entry reads no cwd state and can decide TRUE under cwd-⊤; (3) a non-EXACT `.` is still
MIRRORED (the conductor's D2 re-cut, which the law's LEAN reverses: nothing shipped on a guess
— veto-eligible by one human line); (4) the six pencilled reds are unminted. The prior
conductor's `30Q` §3 D2 re-cut ("cwd-⊤ costs authority only; acquisition and mirroring
untouched") is REVERSED by the law; the steering bullets this session wrote from it
(`rul-havoc-is-pointwise-never-the-stack`'s mirroring clause; `rul-exact-is-not-explicit`'s
"explicitness governs rewriting") are re-said at the retrofit fold as EXACT ∧ explicit /
nothing shipped. DECISION: `lane-blind-act-retrofit` (Opus; map-then-execute in one builder,
checkpoint after the seat map + red cells; branch `ai/r30-lane-blind-act-retrofit` from
`60f3955b`; report the next free `30Q` letter) runs FIRST; the influence EXECUTE is sequenced
serially AFTER it folds (a live wrong-elision route outranks plumbing; both touch
`cli/src/artifact.rs`/`main.rs`). Fruit FOLDED: `ai/r30-lane-fruit-3` (17 commits; two
additive import-list conflicts resolved under widened authority; the lock regenerated by one
`loom publish`; gate green both legs at its content) rebased over the human's tip and ff'd —
conduct `60f3955b`.

**Retrofit checkpoint rulings (2026-08-23; the map is the lane report's `§retrofit-map`):**
`fnd-clobber-seed-keyed-on-the-operand-not-the-target` (builder, +SURE measured) — `load_sites`
seeds cwd-⊤ from an UNEVALUABLE operand, so `. /etc/os-release` (evaluable, unheld) seeded
nothing and a relative `.` below it still resolved and bound: the hole was wider than the
conductor's two suspicions; the seed becomes UNRESOLVABILITY of the site (unheld target, plain-sh
inclusion) — ENDORSED, a fifth red cell. Measured on a scratch tree: seed + `[ -f ]` gate move
ZERO goldens (only the D2 re-cut's own test message re-says). Names ENDORSED:
`FuncEnv::load_certainty(node) -> Result<&ResolvedHead, HavocCause>` (the one composed seat — a
cwd-havoc'd site sits in both maps) · `BookLoad::permits: LoadPermission` (`may_rewrite` =
explicit ∧ exact; `may_ship` = exact); three cells: EXACT∧explicit ⇒ rewrite+bundle ·
EXACT∧inexplicit ⇒ verbatim+mirror · ¬EXACT ⇒ verbatim+nothing. RULED IN narrowly: an
UNSPLICEABLE call (over-budget/recursive/unresolvable callee) seeds the havoc too — the law's
third species; removing licence is the sound direction (E3c; stop-and-report if it moves a
golden). `tc-acquisition-outlives-the-clobber` — acquisition kept (`Withheld`-over-`NoOpinion`
is safe; dropping it stalls the fixpoint); the frameless `build_dialect`/`HelperIndex` exposure
(an early-round file read reaches the whole-unit dialect scan and custody, flag-bounded) is
routed to the next lane touching `build_dialect`. `tc-explicitness-seat-unification` → the
`30O` debt. `p-x-subshell-contains-a-blind-load` already green (the load lane's D1) — not
minted; `forfeit-shell-parity-immunity-model` re-said at the fold (REDS = the `$0` pin alone;
the paren example is stale). Five `r31:kernel-punt-glance` pins minted (census 26 live / 1
reserved). THE VETO-ELIGIBLE TRADE (`tc-nothing-shipped-costs-the-common-book-its-plan`): under
"nothing shipped", the common `. /etc/os-release` + relative-oracle book's apply DIES at the
second `.`; mirroring was inert-never-wrong. Built as the human's lean; the trade is on the
root burndown (`rule-ship-nothing-below-a-blind-act`); E2 is two seats in both directions.

- EXECUTE shape: ONE Opus builder over the green merged tip, rows R1–R9 in the MAP's order
  (R2 before R3 is load-bearing), R10 only with the ruling; gate = goldens + the nine
  loom-embedded `.whylog` transcripts byte-identical + `bless:dry` clean + `spine:baseline`
  before/after handed to the conductor. Size -GUESS L; one return-and-resume checkpoint after
  R3 (the stamp gone, species sealed) is worth its cost.

## §5g — the burndown sitting (2026-08-23; sibling session `r30-sit-blind-loads-unsoundness`; typed rulings, handed across to the running conductor)

Ack-ledger — TYPED unless marked LEAN. Rubber-duck sitting over the root burndown; no dispatch
from that session (one read-only Sonnet scout, `scout-predict-undecidable-test`).

- **`lane-influence-carriage`** (binds the EXECUTE brief; the EXECUTE ack itself must be typed to
  the RUNNING conductor's session — the sitting's "proceed" does not discharge it): R10
  (`stop-spine-mode-is-durable`) → fix at the conductor's choice; format-only, NOT opaque-review
  · durable grade: build to a FUNCTIONING export + e2es, then DISABLE it and swap those e2es to
  xfails before lane-close; the human enables post-review · NACK as welded law: "influence never
  decides whether a line elides" — a v0 FACT (no consumer exists), never law; a future decision
  consumer (e.g. per-host contamination gating revived cross-host planning) is a typed human act;
  the proposed steering must not say "decision-inert by type" · `306b` §6b exemption: the
  conductor's reading stands but stays a NARROW window — single seat, single transition
  (`InfluenceAccount::of_phase`), a caller-count fence, builder latitude, never spread · no
  gradation (ACK; purpose: force the threading and the type-discipline, then WATCH for holes over
  the next rounds — the fold report enumerates every seat that needed an `untracked` adapter) ·
  no render change (ACK) · per-host NOT built; RESERVE the slot: `HostInfluenced` keeps carrying
  the `InfluencePhase`, whose `()` payload is the future host-scope identity (account becomes
  set-of-hosts, join = union, at width two); never simplify to a unit variant · the affine
  "clean-of-host" witness for the Authored mint: NOT built ("not just yet"); this lane a lexical
  fence on `authored_before_contact()` callers · `tc-load-decisions-read-authored`: LEAN (not
  ruling) — move the `SpineLoadDecision` Spine write to the pre-intake seat if cheap, else
  Authored DERIVED from funcenv's inputs, never an asserted label; report as lean-applied ·
  `rul-rc-reaches-genkill-only-through-decisions`'s "expected out of the influence round": never
  strike → `TODO-ADDTL:rc-vs-genkill-permanent-law`; re-home the sentence at the fold ·
  flattening: post-reingest influence is why/report-plane ONLY; no durable ingestion rehydrates
  influence into a decision; whylog ingestion never drives decision — the why-driver's
  widen-to-influenced is the DEFINITION, not an approximation.
- **`rule-ship-nothing-below-a-blind-act`** — (A), confirmed in both sessions: a pre-network
  refusal (B) and mirror-claim-nothing (C) are BOTH unsound — Dorc cannot know the host lacks the
  file at the runtime cwd, so either is a guess in some direction; "not within our remit to make
  guesses, or to ship files around hosts speculatively." No further churn.
- **`unblock-starter-oracle-library` — RULED; the human gate is CLOSED.** Three parts, one
  lane: (1) the predict dialect admits the POSIX `test` grammar (unary string/file, numeric;
  `-a`/`-o` stay out per `276`), WITH the carry safe-list split riding — string/numeric ops stay
  on `carry.rs`'s pure list, file ops (`-f -x …`) become unmarked external reads that the carry
  pass default-disqualifies (`-x` is the EACCES identity case) — acked "as long as it doesn't
  invalidate the carry"; (2) `$(…)` admitted as an opaque ⊤ word (parser-tier — the lexer already
  passes it; `27Q:teach-marked-command-not-cmdsub` unchanged: still not carry-closable); (3) the
  argparse tracer CONTINUES past an unevaluable GATE in the closed `[ COND ] || return N` shape
  (`eval.rs` `run_and_or` / `recognize_gate`) instead of Topping — both outcomes name the same
  entity and the host declines at probe time; `if`/`while` on host-dependent conditions stay
  fail-to-Top. Scout-measured (+SURE): today `run_if`/`run_while`/`run_and_or` short-circuit every
  `Err(TopReason)` to `Flow::Top` and `run_block` stops, so (1)+(2) without (3) leave every
  R2-SHADOW-clean oracle un-probeable by construction; no `for` exists in the dialect. Ground:
  `rul-unsure-falls-toward-sh-parity` · `native-sh-or-break-loudly`; a user-surface/dialect
  decision, never an engine blessing of `command -v`. The stdlib `command` oracle answers the
  WORLD question (an executable on PATH; bare-name floors ⇒ can't-say ⇒ run, `30Ic`), never the
  LOAD question (`30I:pin-command-v-load-model`). Residue, trivial, at authoring time: the
  executable kind's name (bootstrap `sm.dorc.*`) and `-v`-only (lean yes). Any durable strawman
  body must honour the nix-`return 2`/DREP decline ruling (`27W`); none was written to disk.
  Remaining stdlib gates are ENGINEERING: the dialect-widening lane · `30D`/`30J` · the `27Q`
  preconditions. `FORFEITS:forfeit-command-v-poison-wall`'s CAPTURE re-said accordingly.
- **Burndown housekeeping**: the wave-one/veto-sweep block dropped (discharged/triaged);
  `_tmp-r30-conduct-collapse.md` deleted; the three conductor questions and item 1 closed; items
  2–6 untouched. Tooling-Sonnet queue += (7) `lint:docids` is under rework in another lane —
  the human authorized `--no-verify` for doc-only commits while it fights.

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

## §5h — `ratify-committee-sparing-fence` dissolved (2026-08-23; sibling sitting, Fable rubber-duck; branch `ai/r30-dialect-ruling` off `ai/r30-conduct`)

Finding first (+SURE, measured at `oracle::build_dialect` · `core::coord::compare` · `core::contested`
and in `28P`'s lane-close): the `28M` §4 fence was HELD by the human and never built; the FORFEITS
row's "built-as-spiked" was wrong. As-built: same-name cross-unit shadowing withdraws the family
(`ContestedFamilies`); the dialect is one whole-unit last-wins set per family. What `28M` framed
as three verdicts was two already ruled (speech-through-the-dependency-tree: `30I` §2.3/§3.3/§3.4
+ `28Q:syn-closure-is-the-speaker`; keep/lift: `30J` P2) and one live residue — two MINTING
speakers in one family at one frame (only reachable below an admin bless, since a second
`__predict` is a same-name shadow). A conductor mis-step recorded: the first read claimed the
fence was a corollary of `pin-two-position-sparing`; it is orthogonal (one family drifting across
frames vs two speakers at one frame), withdrawn in-sitting.

**RULED [gentle ack, human-typed understanding]** `30J:rul-dialect-is-the-live-speakers-at-the-backing-frame`:
a dialect is a property of one speaker closure per family; a comparison consults exactly the
closure whose definition is live at the backing's frame; the claim contributes a token only;
swapping the live definition swaps the dialect; two closures' words never pool. No fence; the
two-position pin is subsumed (the backing's frame decides). Build rides `30J` §10's trigger; the
`28T` mini-model is the proof home. Still open, unscheduled: cross-family registration (`30J` §6.5).

Encoded: `30J` §12 (+ header, §0, §2.4, §3.1, §6.3, §8, §9, §10) · `28Q` §2/§5/§8/§9 (pins 5, 6,
12 retired) · `28M` (header, §4 rewritten, §6 re-priced, §7, §8, §10, §11 → registration only) ·
`28K` bitem4 + exclusions · `30I` neighbouring-work · FORFEITS (both sparing-dialect rows
dropped) · ANALYZER-NEEDS `an-selector-dialect` · `spike/CLAUDE.md sparing-algebra` ·
`analysis/CLAUDE.md` · `oracle/CLAUDE.md` · `30O` · `Research/README` · the root burndown item
removed.

## §5i — `design-report-only-refusal-scope` ruled (2026-08-23; same sitting; direct on `ai/main`)

**TYPED:** `309:rul-refusal-takes-the-whole-target-down` — scope of refusal = scope of
destruction: the target's planned collection finishes, then no apply-able plan and no further
on-host activity this invocation until a user pull; analysis continues to completion into report
projections; every OPERATION endpoint (each plan mode; any future non-debugging Spine consumer)
takes the one `PlanAuthority` witness, debug/explain/aid the only bypass. HARD RULE: the intake
floor is STOP, never raised to Guard — the malleable part is only which conditions sit at the
floor vs below it (`306c` §3b); identity/framing failures build first, most defensively.
Refused targets WILL write whylogs (a degrading host may leave the durable as the only record;
"push little, pull more" needs something to pull) — can-they-today UNVERIFIED; contents ride the
round's pending opaque-review whylog question (`rul-durable-contents-reviewed-before-design`).
**PUNTED, not ruled:** continued multi-round probing of a refused target (`306b` §5b) vs
stop-poking — the leans cancel; least work wins; not revisited soon. Conductor finding encoded:
`306c` §3a's certifier-trip siting was wrong (that floor is a guard-only plan); re-sited on the
`Refused` arm. As-built gap named for the builder: the cli's `Refused` arm still prints one
diagnostic and returns `IngressRefused` (exit 12); the forgiving-parser re-home and its
allow-list entry (`307a:flg-allow-list-entry-not-added`) stay owed, unscheduled.

## §5j — `design-atmost-completion-speech` ruled (2026-08-23; same sitting; on `ai/main`)

**TYPED:** every completing path carries the sentinel ("it's not a completion sentinel if it's
not completely present"; simple bodies write it once at the end, branching bodies push it into
the branches — the author's burden, unsolvable for them). Spelling: the simple, stupid, obvious
one — the same shape as the `return 2` companion, a `printf` to the engine-supplied report sink;
the human's lean moved since 2026-07-31 because `__predict` already forced oracles onto that
lane, so a second ceremony would be cruft. **Correction, human-typed:** the report lane is
LOAD-BEARING by design — closed-set structured head + end-sentinel + freeform tail LAST; the
head may feed decisions (Influenced), only the tail is aid-only. `spike/CLAUDE.md
decline-class-emission`'s "classes route AID only / the license plane never reads a class" was
miswired and is re-said. Acked riders: exactly one sentinel per body execution (0 and >1 both
refuse the whole footprint); complements `deriv-end … body-rc`, never replaces it; pipeline-tail
swallow is the blessed-pipefail spelling's job (stdlib teaching line); `…_only` bodies owe it
too; a refused-for-no-sentinel footprint mints its hint. Encoded in
`ANALYZER-NEEDS:an-atmost-completion-signal` (the durable home) · `28Q` §9 pin 8 · the steering
bullet · `30O` · the burndown. Build rides the stdlib, unscheduled.

## §5 — Handoff pointers (for a successor, if the ceremony rolls over)

`30O:sched-round-close-ceremony` priorities stand; add: the loop lane's iteration-aware route
decision and the planner's hoist-legality predicate to the crosscheck list. Stale worktrees/
branches are the human's sweep (`LIVING_STATUS`); `worktree-sol-adversarial-30M` is deletable
once EXECUTE-B lands `176e0818`.
