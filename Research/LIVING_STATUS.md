# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record; `Research/README.md`'s per-round map says what each closed document
> did) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/AGENTS.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
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

## IN FLIGHT (2026-10-04)

The identity dependency-order run (Fable conductor, autonomous, the human away; opened
2026-10-04): worktree `.tmp/trees/r31-identity-order-conductor`, branch
`ai/r31-identity-order-conductor`, ledger `notes/313e` § 4 onward. Its remit is the order of
human design sittings that would firm the identity, relation, and effects model, measured with
scouts over the corpus and small Alloy probes. It edits nothing under `specs/` and rules nothing;
every model it writes is quarry (`30Z:form-strawmen-are-quarry-never-seed`). Its report is not
yet minted. Until that lands, `notes/313` and the mechanised specification's held holes are
unreconciled alternatives, and neither is the path.

The assay progress lane (`notes/30Yg`, Fable conductor, worktree
`.tmp/trees/r30-alloy-praxis-conductor` kept) folded into `ai/main` the evening of 2026-09-29
with both gate legs green, in two folds: a `--check`/`--write` reports itself on stderr as
`tracing` events (a begin line, a plan line, each fresh solve's start, translation, five-minute
still-alive with sat4j's conflicts, restarts, learned clauses, and decisions once solving, and
result, an end line on every exit path, all stamped `assay +<elapsed>`; `mise run assay-quiet`
is the hook and agent spelling and silences exactly that); its JSON report goes to
`.tmp/assay/<stem>-<UTC stamp>.json` (`--json <path|->`) and stdout is a summary ending in that
path, carrying the `HEAD` commit; `--write` refuses uncommitted document or shared-half text
(exit 2) and writes the commit into the lock's header (`{"schema": 2, "commit": …}`); `--help`
lists the flags; the official tier has an eight-hour batch cap and is not resumable by ruling;
preflight is sized from the tier's heap; the tooling binary builds under `--profile tooling` so
no workspace build can replace a running tooling exe (`spike/AGENTS.md`
`tooling-runs-under-its-own-profile`). The adapter's digest moved with the counters, so the
committed 311 lock (`(test new) Lock the specs for the first time`, written by the human's
1 h 34 m official pass on the pre-lane binary) no longer matches any key; the 311 mechanization
conductor owns regenerating it from scratch on the landed adapter, together with its own spec
changes [human-typed]. Posed, unfixed: `30Yg:finding-gate-e2e-harness-uplift-race` (the
completion gate's concurrent steps can re-uplift `dorc-harness.exe` under the e2e tests once in
a while), `30Yg:finding-fmt-runs-a-jvm` (two hk steps racing on the tooling exe after a
tooling-source edit; an hk `depends` would close it), and
`30Yg:open-respawn-parse-eats-wall-budget`. The lane's second tune, from the 311
mechanization conductor's needs list (a shared world module for books; an instance text form;
two lints), is not yet started; its three small chafes (`--help`, deferral notes naming their
measurement, `expect` on report rows) landed here.

The assay build closed 2026-09-28 and its hot-loop lane 2026-09-29, every lane folded (ledgers
`notes/30Yc`, § 7 the close, and `notes/30Yf`, the hot-loop rulings; as built `notes/30Yd`,
`notes/30Ye`; what is, `notes/30Y`; the praxis `plans/30Z`). The design-tier checker exists:
`mise run assay -- <spec.assay.md> [--parse | --staged | --check | --write] [--hot | --gate |
--official] [--module <m> | --only <[module.]command>] [--quiet]` compiles a specification into
`target/alloy/<stem>/` and answers every row whose key is unchanged straight from
`<stem>.lock.json`, so a `--check` with nothing changed costs a parse; a set of reds the lock
records is still a pass. The tiers are `--hot` (120 CPU s per command, deferring what last timed
out or is large), `--gate` (600 s; the default and the `assay-lock` completion step) and
`--official` (1800 s and 4096 MB, from scratch); exit 4 means no mismatch but rows unmeasured or
owed a run at the ceiling, which the official tier answers. The 311 specification's lock is
schema 1: its first `--check` re-solves every row, and a row that times out against it reads as
unmeasured (exit 4) until a definite result or an `--official` run records it. Pre-commit's hk
`assay` step runs `--staged` (compile, parse, and a key-diff warning); `mise run alloy --
--timeout <s> <dir>` runs a directory bounded. Every heavy task
(`alloy`, the three gates, the four `bless` tasks, the three `verify:*` lanes) runs under the
machine-global heavy-work lock; **exit 75 is contention, do other work**. The one open design
item, what an author owes and when, grows `plans/30Z` § 6 (the locks and gates are glossed there)
and sits in `notes/30Yc` § 5 as the conductor's draft. Strawmen under `notes/30Ya-*` are frozen.
The Alloy authorship praxis for agents is `.claude/skills/using-alloy/` (its `examples.md` executed
against the pinned jar and repaired 2026-09-28; evidence in
`.claude/research/design-model-mechanisation-prior-art/`). The runner refuses a `--command` that
names nothing (exit 2), judges a command carrying `expect` by agreement with it, reports the
`expect` in the row, and keeps kodkod's native-library probes off stderr.

The test-architecture rebuild folded into `ai/main` on 2026-09-05 (its account: the r30 row of
`Research/README.md`; its ledger `notes/30Xa`; its design of record `notes/30X`). What remains of
r30 is the human's close ceremony (`## Round`).

## Standing truths a successor must not re-derive (ruled; homes cited)

`plans/30D` is the prediction-contract design of record (promoted from notes): literal
model status, mandatory shell/DREP decline agreement, authored stream checkpoints, and
separate execution integrity. It is not an as-built assertion. The reasoning is `notes/311a`
§§11–13; `310` and `ROADMAP` remain deliberately unreconciled pending the human's phasing
sitting. No prediction implementation is in flight.

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
accounting carried by every stable object (`30Qd`; `core/AGENTS.md
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

Repo-durable conduct law lives in `spike/AGENTS.md` (Safety · Boundaries · Spawning
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
high bar) · no TODO of any kind escapes into a `CLAUDE.md` — build it now or let it get lost
(human-typed 2026-09-04) · no durable to-ack or owed list anywhere, in any document — lose the
work rather than mint pending work (human-typed 2026-09-04: "I'd rather lose work than mint
pending-work").

## Round

r30 is OPEN. Its close ceremony (`30O:the-schedule`: the human-run gate → `gate:arc` → the
`CURRENT_ROUND` bump → the prose queue → ff `ai/main`) is the human's; the test-architecture
rebuild, r30's last conductor arc, folded 2026-09-05. Closed arcs and their accounts:
`Research/README.md` per-round map; r30's ledgers in order: `notes/300` · `307` · `30N` · `30O`
· `30Va` · `30Xa`.
