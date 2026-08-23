# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record; `Research/README.md`'s per-round map says what each closed document
> did) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/CLAUDE.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
> findings get a durable numbered-note home; this file carries pointers.
>
> **How to maintain:** update judiciously — direction-changes, discoveries, refutations,
> deferments; never per-turn chatter. Keep it NARROW: only in-flight work and what a
> near-future conductor must know; when an arc closes, its entry collapses to one pointer and
> its account moves to the README round-map. Reverse-chronological, always.

---

## CURRENT STATE (2026-08-22 — r30 CLOSE-OUT IN FLIGHT; resume from `notes/30Q` §5c)

**Where to start:** `notes/30O` (THE schedule: every owed r30 kernel stage, the lanes, the
fold order, the brief riders) and `plans/30P` (THE design for the emission planner, the
stream forms, and book-load principles — including its review stance on `notes/30Pb` and
the rulings of the 2026-08-22 sitting). Read both in full before dispatching anything.

**Branch state:** `ai/r30-conduct` (worktree `.claude/worktrees/r30-conduct`) carries, linearly:
the four kernel lanes, the fold repair, the loop-residue lane, the influence MAP (red cells
only), and this session's steering/ledger commits. ONE lane is still folding: the fruit lane,
via `ai/r30-lane-fruit-3` (a Sonnet regenerating the conflicting catalog locks; its gate is the
merged tip's first whole-gate). `ai/main` is the human's to fast-forward; the primary checkout
carries the human's uncommitted ruling work — radioactive. Lane table and the fold order:
`notes/30Q` §2/§5c. The xfail/atlas lane branch `ai/r30-lane-load-xfails` is CONTAINED and may
be `-d`'d once its harness worktree (`agent-a5aff50f562a53775`) is reaped; test `176e0818` is
recovered, so `worktree-sol-adversarial-30M` is deletable. Three stale review branches
(`ai/r30-loom-surface-build2`, `review-verify-adv`, `review-verify-neutral`) are not contained
— the human's sweep; the four folded lane branches and their harness worktrees likewise.

**Dispatch state, per `30O:the-schedule`:** `sched-parallel-disjoint-lanes` is BUILT and
folded (gate-green at the repair tip) → `sched-serial-constructor-reshape` =
`lane-influence-carriage` MAP FOLDED (rulings in `notes/30Q` §5d; two questions on the root
burndown), EXECUTE waits on a typed human ack →
`sched-round-close-ceremony` (crosscheck · the `307` §6 veto register · `gate:arc` ·
`CURRENT_ROUND` bump · prose queue) held. Every brief: the Safety block, step-zero worktree verify,
`AGENTS.for-builders-only.md` first, and `notes/30Pc` (the opaque review's builder-lane
half — unread by Fable-class conductors by law) where it names the lane.

**Human-gated (nothing dispatchable):** the root `_tmp-human-burndown.md` items; the
`30P:open-rulings` residue (the controller-expectation/host-check pattern's UX + opaque
review); the prose queue; the
non-blocking `30N:open-items-riding-this-conduct (née §4)` set (wall-narration mint · the
render↔spine feedback sitting · `tc-plan-owns-its-source` · book-level dot-locals domain ·
redirect dead-or-owed · `stop-spine-mode-is-durable` · certification row shape · the
prelude-floor veto).

**Standing truths a successor must not re-derive:** xfail horizons are ATTENTION-CALLS, not
completion dates — never re-horizon them as if "r31" were a plan; end-of-r30 is kernel
quiescence and unscheduled means unscheduled. `KNOBS:kBACKFLIPS` is welded: verbatim
relocation or refuse; the floor is uneven across emission forms. No floor-valid text is a
parse violation (`30P:rul-floor-valid-text-never-parse-fails`). Every FORFEITS row carries
reds (`30P:rul-forfeits-carry-reds`). A book-load head is EXACT or a point havoc, nothing
between — no snapshot-suffix sets, no singleton, no runtime-verified candidate
(`30P:rul-load-head-is-exact-or-havoc`); a command substitution in a load operand resolves
only through a statically-evaluable stdlib predict (`30P:rul-static-predict-sites-loads`).
Dorc never interprets what a convergence vouch checks; it only keeps its own movement and
renaming from producing bindings ordinary sh would not (`30P:rul-guard-resolves-like-its-mutation`).
The load plane's correctness posture is RULED (`30P:the-load-plane-stays-correct`, 2026-08-22):
a book's `.` line is rewritten only under derived permission (explicit reference + transformed
target — literal operands); EXACT-via-`$0` lines stay verbatim and are mirrored so they land;
controller-evaluated predicts are verified at probe standup, artifact integrity at apply standup,
and no per-line load verifier exists — the standups are tunnel negotiation, never plan lines.
`gate:full-quiet` routes `test:floor` when floor paths
are staged (a floor case must agree on both platform legs). The opaque-review gate is
builder-initiated (`AGENTS.for-builders-only.md`); no builder triggered it this sitting.

**Register debt still owed (conductor-tier, small):** ANALYZER-NEEDS `an-backing-selfframing`
("freezing remains owed" is stale) · `oracle/CLAUDE.md`'s dangling cite
`FORFEITS:forfeit-whole-file-inertness-refusal` · `cli/CLAUDE.md` harness-contract lines
(floor cases: top-level files only; `$0` shape platform-bound) · the `30Kb` residue.

## GENTLY HELD (deliberately waiting on the human)

- **block-stdlib** — zero non-fixture oracles; gated on `unblock-starter-oracle-library`
  (the dialect-reach decision) and on the `30D`+`30J` predict-contract arc landing first.
  On-ramp: `notes/27Q`.
- **`28Q:stage-iii-world-scopes`** — gated on `design-world-scope-surface`,
  `rule-incarnation-continuity-semantics`, and the ssh oracle (⇐ stdlib).
- **r26 reactive/capture + multi-host revival** — `26B`/`26C` + `260`–`262`; after the r30
  close. **`26K:fruit-arc (née §0a)`** is the one parallel-anytime slice and is scheduled.
- **The forfeited inclusion tiers** — `FORFEITS:forfeit-plain-sh-inclusion-analysis`; the
  next language-surface round's entry point.

## Conduct fences (standing; bind any successor)

Repo-durable conduct law lives in `spike/CLAUDE.md` (Safety · Boundaries · Spawning
subagents · Build/test/run) — read it there. Fences living only here: git surgery relaxed
2026-07-19 (branch-scoped, reflog-recoverable surgery is permitted in autonomous mode; push,
stash-drop/clear, `clean -f`, force-delete, tag-delete, filter-*, update-ref stay blocked;
the human reviews-and-rebases AI branches) · merges from `main` batch at round-close ·
silence ≠ ack (only what the human TYPED counts; keep an ack-ledger) · crosscheck adjudication
under maximum skepticism; adversarial framing = exclusions-not-inclusions · never
AskUserQuestion (ask in prose); dump the numbered task list on changes · Fable conducts, Opus
codes · the verify entrypoint is `mise run both gate:full-quiet`; `gate:arc` at arc close,
from the populated branch before folding · naming discipline (`270` §1, HIGH): hyphenated
full-word slugs, `docID:slug` cross-refs, subscript old labels once ("née P5") · note-ID
discipline: r28+ notes are LETTER-suffixed; never mint another `29x` ID (the quarantined r29).

## History

Closed arcs and their accounts: `Research/README.md` per-round map. r30's ledgers in order:
`notes/300` (first half) · `307` (wave two) · `30N` (second half) · `30O` (close-out).
