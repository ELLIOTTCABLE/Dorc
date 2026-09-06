# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record; `Research/README.md`'s per-round map says what each closed document
> did) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/CLAUDE.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
> findings get a durable numbered-note home; this file carries pointers.
>
> **This file and root `ROADMAP.md` are mutually exclusive.** A piece of work appears here
> only once a conductor has been handed it and told to build it; until then it is on the
> roadmap (scheduled or owed), and when work is split on purpose only the handed-over half
> moves. So everything here has active, owned, in-flight work; an entry sitting here unowned
> means code-level work was dropped mid-build — never design residue that was punted, which
> belongs on the roadmap or in its record.
>
> **How to maintain:** update judiciously — direction-changes, discoveries, refutations,
> deferments; never per-turn chatter. Keep it NARROW: only in-flight work and what a
> near-future conductor must know; when an arc closes, its entry is deleted and its account
> moves to the README round-map. Reverse-chronological, always.
>
> *Never* update this file in a worktree; apply your edits directly in the
> project root, and if possible, commit it there by pathspec (check for
> tree-dirty status and/or recent commits.) This is effectively cross-arc
> collaboration, so be defensive about concurrent edits, and yield where
> appropriate. It need not be edited every time anything small changes; target
> end-of-arc and/or substantial-redirects.

---

## IN FLIGHT (2026-09-05 — the test-architecture REBUILD: BUILT and gated; the opaque review's NACK ADJUDICATED into `30C:accept-dangerous-developer-harness`; two repairs and the fold OWED to a successor conductor)

The rebuild is complete on `ai/r30-30X-test-infra-decruft-conductor` (worktree
`.tmp/trees/r30-30X-test-infra-decruft-conductor`, riding `ai/main`): the `Seams`/`HarnessSeams`
bundle and the `dorc-harness` sibling binary; looms as shell sessions with gates by block kind; the
in-process receipt world with `gate-two-drivers-agree`; seeds varied by default; one runner; thirteen
frontmatter keys and every dir-case converted; `dorc-testbed`; the shell-resolution seat in
`transport` consumed by `dorc-sh`; the sibling-oracle advisory retired; the apply identities line
authored; steering, registers, plans, and the round map current. Design of record `notes/30X`;
conductor ledger **`notes/30Xa`** — its §0 is the successor's complete on-ramp: the ruling's bounds,
the finding that `mise run install` currently installs `dorc-harness` alongside `dorc` and `dorc-sh`
(the repair and its objective check), the "runner-owned" re-cut list, the gate procedure under this
machine's memory limits, and the fold steps. The ephemeral handoff carrying the ruling's text sits
untracked in that worktree. No further opaque review was requested; the fold completes on the
existing procedure once the two repairs land green.

## Standing truths a successor must not re-derive (ruled; homes cited)

xfail horizons are ATTENTION-CALLS, not completion dates — never re-horizon them as if "r31"
were a plan; end-of-r30 is kernel quiescence and unscheduled means unscheduled
(`30O:the-measuring-stick`). `KNOBS:kBACKFLIPS` is welded: verbatim relocation or refuse; the
floor is uneven across emission forms. No floor-valid text is a parse violation
(`30P:rul-floor-valid-text-never-parse-fails`). Every FORFEITS row carries reds
(`30P:rul-forfeits-carry-reds`). A book-load head is EXACT or a point havoc, nothing between
(`30P:rul-load-head-is-exact-or-havoc`); a command substitution in a load operand resolves
only through a statically-evaluable stdlib predict (`30P:rul-static-predict-sites-loads`).
Dorc never interprets what a convergence vouch checks (`30P:rul-guard-resolves-like-its-mutation`).
The load plane's correctness posture is RULED (`30P:the-load-plane-stays-correct`): a book's
`.` line is rewritten only under derived permission; EXACT-via-`$0` lines stay verbatim and are
mirrored; controller-evaluated predicts are verified at probe standup, artifact integrity at
apply standup; no per-line load verifier exists. Below a BLIND ACT Dorc claims nothing
(`30P:law-no-unsoundness-below-a-blind-act`; built in `30Qf`). Influence is causal
accounting carried by every stable object (`30Qd`; `core/CLAUDE.md
the-influence-account-is-carried-never-stamped`), licenses nothing at v0, and its durable
export is built but DISABLED. `gate:full-quiet` routes `test:floor` when floor paths are
staged (a floor case must agree on both platform legs). The opaque-review gate is
builder-initiated (`AGENTS.for-builders-only.md`). Cross-kind sparing is licensed only by a
finished definition (`30U:rul-cross-kind-sparing-needs-a-finished-definition`, 2026-08-29;
the `an-kind-reach` row and steering carry it; the build is unscheduled). Any Dorc act not
provably caused by the author's code is withhold-shaped; only explicitly-authored, unmodified
lines Dorc can Must-conclude would have run under plain sh are perform-shaped
(`26N:rul-dorc-acts-are-withhold-shaped`, human-typed 2026-09-03). An unresolvable
book-custody load refuses the plan before network — never shipped on a guess, never left to
die halfway (`26N:rul-unresolvable-book-load-refuses`). The delivery shape is ruled (`26N`
§2), and `$0` under Dorc is the absolute path of the real plan file wherever a filesystem
exists (`26N:rul-dollar-zero-authority-spelling`). Every context Dorc stands up carries its
own scaffold and sink, and records never cross a link as records
(`26O:rul-every-context-carries-its-own-scaffold`); which of the four streams (control /
aid / raw-out / raw-err) rides which lane is planned per run after measurement and analysis,
never fixed by the language, with collapse a last resort (`26O:5-the-routing-planner`); the
probe lane never holds a pty.

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
discipline: r28+ notes are LETTER-suffixed; never mint another `29x` ID (the quarantined r29) ·
every `/opaque-review` DISPATCH — design-time or end-of-arc, initial or follow-up — needs the
human's TYPED ack first (human-typed 2026-09-02): loading the skill on a builder's instruction
licenses no dispatch by itself · `rNN:` xfail horizons are never minted to mirror a roadmap
row (human, 2026-09-02: mechanical horizons are expensive in human attention and must pass a
high bar).

## Round

r30 is OPEN. Its close ceremony (`30O:the-schedule`: the human-run gate → `gate:arc` → the
`CURRENT_ROUND` bump → the prose queue → ff `ai/main`) is the human's, once the rebuild above
folds. Closed arcs and their accounts: `Research/README.md` per-round map; r30's ledgers in
order: `notes/300` · `307` · `30N` · `30O` · `30Va` · `30Xa` (in flight).
