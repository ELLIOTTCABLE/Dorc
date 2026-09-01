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
>
> *Never* update this file in a worktree; apply your edits directly in the
> project root, and if possible, commit it there by pathspec (check for
> tree-dirty status and/or recent commits.) This is effectively cross-arc
> collaboration, so be defensive about concurrent edits, and yield where
> appropriate. It need not be edited every time anything small changes; target
> end-of-arc and/or substantial-redirects.

---

## NEXT (2026-09-01 — the test-architecture rip; human-typed "absolutely owed")

The why-surface arc's PRODUCT is sound; its receipt-route TEST ARCHITECTURE is cruft to
rip and rebuild: the `expect-why-receipt` needle gate (assertions hardcoded in the e2e
runner, case as costume) goes; in its place, harness-only determinism at the CLI's two
`getrandom` seats (the crates already carry the injection seam), sequence-shaped cases
(ordered invocations against one persistent world) as ordinary corpus data, byte-exact
goldens, the three bespoke process-restart `.rs` batteries migrated in, and the 37
total-surface labels thereby authorable. Full handoff, rip list, and the one owed design
sitting (the fence around a deterministic identity mode — cargo-feature, never env;
opaque-adjacent): **`notes/30Va` §HANDOFF**. Start there.

## CURRENT STATE (2026-08-31 — the r30/30R close arc is COMPLETE and folded; the round ceremony is the human's)

Arc conduct: **`notes/30Va`** (remit · rulings · acks · close). `ai/main` (`ea295267`)
carries the whole arc, linearly: the receipt residue lane (`notes/30Rk`), the
whylog→receipt steering/register recast (née-tagged renames), and the receipt-backed
why surface BUILT COMPLETE across three builder slices (`notes/30Vd` is the lane
ledger): the `dorc-why` reconstruction plane (`30V` §3 as types; `Known`/`Held`
wrappers), `dorc_receipt::report` widened EXHAUSTIVE over all fifteen persisted plan
families (opaque-ruled, `30Va:rul-report-projection-becomes-exhaustive`), the
intentionally-temporary total surface (`cli::why_total`) + `--json`, the receipt-rooted
`dorc why` wiring (listing seats deleted; `--receipt <file>` repaired; digest-matched
addresses), the one-CLI-seat source-comparison packet
(`30Va:rul-source-comparison-is-one-cli-seat`; `file.sh:N` renders where the visit
holds content), the publish→why e2e proof (`expect-why-receipt` run-lane key), the
report-only orphan-arrangement instrument (`mise run prose:orphans`), and the
conductor prose pass (help rows, receipt messages, register-guide recasts; the 37
total-surface labels rest `[unwritten:]` — the label-mint gap is banked in `30Va`).
`gate:arc` green from the populated lane before the fold.

**What remains is the human's:** the r30 round-close ceremony (below) · the
`why-surface-close-residue` row in TODO-ADDTL (unwitnessed branches, the held
`tc-address-refusal-is-a-datum-not-a-diagnostic`, five orphan rows awaiting a deletion
word) · the standing soft-acked root-identity fold. Report-only kernel re-derivation
remains a separate authorized round; the kernel stayed frozen throughout.

## CURRENT STATE (2026-08-23 — r30 kernel CLOSE-OUT: every lane BUILT and folded; the round ceremony is owed and is the human's)

**Where to start:** `notes/30O` (THE schedule) and `plans/30P` (THE design for the emission
planner, stream forms, book-load principles). The lane table and fold order: `notes/30Q`
§2/§5c–§5e. NO lane is open; every end-of-r30 pin is green (census: all `r31:*`). The
2026-08-30 sweep (`30Va`) reaped the residual lane branches/worktrees the 2026-08-23 sweep
left; nothing of that inventory remains.

**Dispatch state, per `30O:the-schedule`:** `sched-parallel-disjoint-lanes` and
`sched-serial-constructor-reshape` are BUILT and folded (`30Q` §5c–§5e; influence in `30Qd`) →
`sched-round-close-ceremony` is NEXT and is the human's/successor's: the human runs the gate
themselves; no crosscheck in the closing window (human-typed 2026-08-23); then the `307` §6
veto register · `gate:arc` · `CURRENT_ROUND` bump · the prose queue · the ff of `ai/main`.
The durable account export DIED with the old durable (`30Rk:the-account-export-died-with-its-lane`;
xfail pin `p-x-durable-account-export-is-enabled` parked `Reserved`) — rebuilding it against the
receipt durable clears `rul-durable-contents-reviewed-before-design` first.
Every brief: the Safety block, step-zero worktree verify,
`AGENTS.for-builders-only.md` first, and `notes/30Pc` (the opaque review's builder-lane
half — unread by Fable-class conductors by law) where it names the lane.

**Human-gated (nothing dispatchable):** `30O:human-gated-rulings` is the current list — the
burndown's remaining headers (the committee fence · world-scope surface · incarnation
continuity; all stage-iii) · `tc-dollar-zero-is-script-anchored` · the hoist ACTION's two calls
(`tc-hoisted-dot-line-spelling`, `tc-t2-is-narrower-than-the-ladder-says`) · the review before
any rebuilt account export is enabled · the prose queue (`mise run prose:census`) · the parked
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
  correctly starves the capture lane). **`26K:fruit-arc (née §0a)`** LANDED as r30's
  fruit lane (verified in-tree 2026-08-25; residue: the no-secrets authoring-doc
  paragraph, human-adjacent).
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
