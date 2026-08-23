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
- **Human advice carried into every brief (typed 2026-08-22):** naming argued only for overloads
  and cross-domain glosses (a 10% lean; two-to-three-word names acceptable as the fix);
  whole-product unrepresentability statements for new/reshaped types; parser work is cheap and
  throwaway — fix the obvious while fingers are in, keep the rich-AST/locator seam, redden nothing.

## §2 — Lanes

| lane | branch | note | builder shape | status |
|---|---|---|---|---|
| `lane-loop-propagation` | `ai/r30-lane-loop` → `-2` | loop lane report | Opus ×2 (census+value plane; then the licence) | FIRST BUILDER STOPPED GREEN at `69977343` (census, member value plane, member-keyed routes, four `loop30-*` reds; licence unbuilt — stop ENDORSED: the floor may move only after per-member facts exist); LICENCE BUILDER DISPATCHED 2026-08-22 |
| `lane-emission-planner` | `ai/r30-lane-planner` → `-exec` | planner lane report | Opus MAP → ruling → EXECUTE | MAP FOLDED-IN-BRANCH `0c38045d` (30Pc repair + 2 evidence cells landed); RULED; EXECUTE DISPATCHED 2026-08-22 |
| `lane-load-plane-precision` | `ai/r30-lane-load` → `-a`, `-b` (parallel) | load lane report | Opus MAP → ruling → EXECUTE-A ∥ EXECUTE-B | MAP FOLDED-IN-BRANCH `d2b47654` (5 red pins, glob retarget, plain-sh e2e red); RULED; A and B DISPATCHED 2026-08-22 |
| `lane-influence-carriage` | — | influence lane report | Opus EXECUTE over the merged tip (scout census banked) | WAITING on the fold |
| `lane-fruit-arc` | `ai/r30-lane-fruit` | fruit lane report | Sonnet, one shot | DISPATCHED 2026-08-22 |

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

## §5 — Handoff pointers (for a successor, if the ceremony rolls over)

`30O:sched-round-close-ceremony` priorities stand; add: the loop lane's iteration-aware route
decision and the planner's hoist-legality predicate to the crosscheck list. Stale worktrees/
branches are the human's sweep (`LIVING_STATUS`); `worktree-sol-adversarial-30M` is deletable
once EXECUTE-B lands `176e0818`.
