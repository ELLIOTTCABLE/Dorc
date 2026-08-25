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

## CURRENT STATE (2026-08-25 — the durable receipt family is IN FLIGHT on `ai/r30-receipt`; a fresh conductor resumes from `quarantine/30Rc`)

**Where to start:** `plans/30R` is the conductor-facing design. `quarantine/30Rc` is the live
conductor state — branch tips, what binds, what is owed, and an invariant corpus awaiting a
steering-prose synthesis. Read `30Rc` in full before dispatching anything; it carries the
carry-ins builders must not rediscover.

**Branch state:** `ai/r30-receipt` (worktree `.claude/worktrees/r30-receipt`) carries the arc
linearly — two new crates (a dependency-light receipt crate plus a sibling holding its provider
implementations), the exact grammar and its record families, recorded models, the report-only
graph, the plan-side projection, and the pre-dispatch authority chain. It is 85 commits over
`ai/main` and **8 behind it**; rebase before folding. `ai/r30-conduct` carries the ledger only.
Five folded lane branches/worktrees resist `-d` because the build branch was rebased mid-arc and
their commits are now copies — left for the human, do not force.

**Dispatch state:** stages 0–2 done; stage 3 substantive; **stage 4 partial** (authority chain
built — intent/outcome projection, the deterministic apply route, and gating the ship path are
owed); stages 5–6 unstarted. No lane is open; every builder stopped deliberately at a commit
boundary.

**Owed at close, hard:** the six why-chain e2e cases whose replay arm the old writer's removal
will empty (restored verbatim — and neither "disable" instrument reaches them); a steering-prose
synthesis from `30Rc`'s invariant corpus, since builders authored none this arc by ruling;
`plans/30R` reconciliation; `gate:arc`. Four departures await the human as the top entry of
`TODO-ADDTL`.

**One tooling change landed for everyone:** `gate:full-quiet` now opens with `gate:floor`, which
refuses a run that would check nothing. It exists because the gate was measured returning success
having executed zero checks — and a **pure-deletion changeset selects zero checks**, which matters
for any deletion-shaped stage. Read the floor's line, never the exit code alone. Unresolved and
the human's: `cargo test --no-run` intermittently cannot replace `spike/target/debug/dorc.exe` on
Windows (`Access is denied`), where `CARGO_TARGET_DIR` sits inside the synced tree.

## CURRENT STATE (2026-08-23 — r30 CLOSE-OUT: every lane BUILT and folded; the ceremony is owed; resume from the close entry at the end of `notes/30Q` §5e)

**Where to start:** `notes/30O` (THE schedule: every owed r30 kernel stage, the lanes, the
fold order, the brief riders) and `plans/30P` (THE design for the emission planner, the
stream forms, and book-load principles — including its review stance on `notes/30Pb` and
the rulings of the 2026-08-22 sitting). Read both in full before dispatching anything.

**Branch state:** `ai/r30-conduct` (worktree `.claude/worktrees/r30-conduct`) carries, linearly:
the four kernel lanes, the fold repair, the loop-residue lane, the influence MAP (red cells
only), the fruit lane, the human's blind-act law (`30P:law-no-unsoundness-below-a-blind-act`),
the blind-act retrofit and the third blind act (the load plane under that law: rewrite gate
EXACT ∧ explicit, nothing shipped below a clobber, the `[ -f ]` cwd gate, the CFG refusal set
as a clobber seed), the tooling lane, the influence EXECUTE (`30Qd`), the sibling sitting's
rulings, and this session's steering/ledger commits. NO lane is open. Every end-of-r30 pin is
green (census: all `r31:*`). `ai/main` is the human's to fast-forward; the primary checkout
carries the human's uncommitted ruling work — radioactive. Lane table and the fold order:
`notes/30Q` §2/§5c–§5e. Swept 2026-08-23 (contained-and-clean only): 14 finished worktrees and
43 branches reaped, ~26 GB of WSL caches returned. Left for the human: six CONTAINED lane
branches that `-d` refuses because of their configured upstream
(`ai/r30-lane-{influence-map,load-b,loop-2,load-a,fruit-3,planner-exec}`); five early-stage
lane worktrees whose history was folded as REBASED copies, so the containment proof fails
(`ai/r30-lane-{loop,planner,load,fruit,fruit-2}`); the dirty `r30-loading`
(`ai/r30-static-loading`); and the uncontained review branches (`review-verify-adv`,
`review-verify-neutral`, `ai/r30-loom-surface-build2`, `worktree-sol-adversarial-30M` — the
last sits on `review-verify-adv`'s tip).

**Dispatch state, per `30O:the-schedule`:** `sched-parallel-disjoint-lanes` and
`sched-serial-constructor-reshape` are BUILT and folded (`30Q` §5c–§5e; influence in `30Qd`) →
`sched-round-close-ceremony` is NEXT and is the human's/successor's: the human runs the gate
themselves; no crosscheck in the closing window (human-typed 2026-08-23); then the `307` §6
veto register · `gate:arc` · `CURRENT_ROUND` bump · the prose queue · the ff of `ai/main`.
Open before the durable account export is ENABLED: its review (`rul-durable-contents-reviewed-before-design`;
the switch is `plan::whylog::ACCOUNT_EXPORT`). Every brief: the Safety block, step-zero worktree verify,
`AGENTS.for-builders-only.md` first, and `notes/30Pc` (the opaque review's builder-lane
half — unread by Fable-class conductors by law) where it names the lane.

**Human-gated (nothing dispatchable):** `30O:human-gated-rulings` is the current list — the
burndown's remaining headers (the committee fence · world-scope surface · incarnation
continuity; all stage-iii) · `tc-dollar-zero-is-script-anchored` · the hoist ACTION's two calls
(`tc-hoisted-dot-line-spelling`, `tc-t2-is-narrower-than-the-ladder-says`) · the review before
`plan::whylog::ACCOUNT_EXPORT` flips · the prose queue (`mise run prose:census`) · the parked
`tc-plan-owns-its-source` and `tc-book-level-dot-locals-domain` sittings.

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
Below a BLIND ACT Dorc claims nothing (`30P:law-no-unsoundness-below-a-blind-act`, human-typed
2026-08-22/23): no cwd-dependent decision, no authority from definitions loaded below it, no
rewrite, nothing shipped, no elision, no engine-side recovery — guards survive and the
remedies are admin-sourced (`notes/30Pd`); built in `30Qf`. Influence is causal accounting
carried by every stable object (`30Qd`; `core/CLAUDE.md the-influence-account-is-carried-never-stamped`),
licenses nothing at v0, and its durable export is built but DISABLED.
`gate:full-quiet` routes `test:floor` when floor paths
are staged (a floor case must agree on both platform legs). The opaque-review gate is
builder-initiated (`AGENTS.for-builders-only.md`); no builder triggered it this sitting.

**Register debt:** `30O:register-and-steering-debt` is current (the `30Kb` residue · the
two-seat explicitness predicate · the frameless acquisition exposure · `doctor` over drvfs);
the conductor-closed residue rows live in `TODO-ADDTL` (`r30-close-out-residue`).

## GENTLY HELD (deliberately waiting on the human)

- **block-stdlib** — zero non-fixture oracles; the dialect-reach human gate is RULED (`30Q` §5g:
  POSIX `test` grammar + `$(…)`-as-⊤ in predict bodies, carry safe-list split, tracer continues
  past an unevaluable gate); remaining gates are engineering — that dialect-widening lane, the
  `30D`+`30J` predict-contract arc, the `30S` env-identity floor
  (`30S:seq-stdlib-gates-on-env-identity`: retarget-sensitive families never ship on
  name-blind fact keys), the `27Q` preconditions.
  On-ramp: `notes/27Q`.
- **`28Q:stage-iii-world-scopes`** — gated on `design-world-scope-surface`,
  `rule-incarnation-continuity-semantics`, and the ssh oracle (⇐ stdlib).
- **r26 reactive/capture + multi-host revival** — `26B`/`26C` + `260`–`262`; after the r30
  close. The 2026-08-24/25 sittings (`notes/26L`+`26Lb`, exploration/brainstorm-tier — cite
  no commitments) bear on its shape: conductor-book topology, punt-now-not-forever on
  cross-host influence, and capture soft-gated on `30D` stdout claims (unclaimed stdout
  correctly starves the capture lane). **`26K:fruit-arc (née §0a)`** is the one
  parallel-anytime slice and is scheduled.
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
