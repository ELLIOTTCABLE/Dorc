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

## IN FLIGHT (2026-09-03 — the test-architecture REBUILD; lanes A, B, C1, C2, C3a built and green; C3b building; D next; the arc branch rides today's `ai/main`)

The why-surface arc's PRODUCT is sound; its test architecture is rebuilt, not patched, in a
non-concurrent suite-only arc (product work stays separate). Design converged in a
Fable⇄human sitting 2026-09-01: one `Seams` bundle with per-seam selection (a tier is a row
of the seam×implementation matrix, never a separate harness); a sibling `dorc-harness`
binary carrying runtime seam injection while the shipped `dorc` loses even its env pins
(`inv-division-at-the-narrowest-edge`); looms as unrestricted shell sessions with gates
attached by block KIND, seams spelled as `$ export` lines, transcripts carrying both streams;
varied seed by default, declared seed as per-case regression opt-in in every tier; one runner
with the driver derived; frontmatter 24→9; the needle gate ripped; the batteries split into
loom goldens vs state-only Rust e2e. Four serial lanes, all in scope, no deferrals except
kernel-mutating improvements. The design statement and its build-planning tail: **`notes/30X`**.
The arc is OPEN (2026-09-02): conductor worktree `.tmp/trees/r30-30X-test-infra-decruft-conductor`
on `ai/r30-30X-test-infra-decruft-conductor`; the conductor ledger sits beside `30X` on that
branch and folds with it (typed rulings · reconciliation deltas · the dir-cases-versus-looms
analysis · the lane table). The human's
2026-09-02 rulings are folded into `30X` §3/§3a (no legacy e2e — every dir-case converts to a
loom; all prose is loomed; a non-loom test is licensed only by seven closed classes; every
loom-escape impulse stops at a conductor; one handbuilt test mechanism per arc) and §11.
`/opaque-review` `30-reviewA` returned NACK; the human adjudicated it into `30X` (`RealSsh` and
`Os` roots out of the ordinary harness; a `HarnessSeams` subtype; the scrubbed session;
livetest as the explicit ambient composition; `rul-seam-columns-are-conductor-ruled`), and
`30-reviewB` over the re-stabilized design returned ACK (both reports committed on the arc
branch, unopened by the conductor). BUILT and green on both gate legs (as of the branch tip carrying the ledger's §0): lane A —
the `Seams`/`HarnessSeams` bundle, `compose::run`, `dorc-harness`, seeded entropy, the six env
pins retired, the scrubbed session, literal runner-owned roots; lane B — one persistent `sh`
session per `run:` loom with gates by kind and both-streams transcripts, the ticking per-block
clock default, the needle gate and the baseline scaffold gone, the receipt-rooted why loom a
four-block session, the receipt batteries split into one state-only home. Lane C ran as four
lanes, three COMPLETE and green on both legs: C1 (the in-process driver owns a real seeded
receipt world over the model store, reached through `dorc_cli::durable` re-exports; one
io-parameterized receipt edge shared with the shipped binary; the typed `LoomDecline`;
`gate-two-drivers-agree` LIVE with 107 `run:` looms agreeing byte-for-byte; the runners now reap
their temp after the 2026-09-03 disk-full incident their shim copies caused) · C2 (the session
grammar through `dorc_syntax`, a concrete dash environment feeding the one seam parser, the
kernel's own `Cwd`; 108 agree; the kernel-side dogfood ceiling recorded in `30X` §10) · C3a (the
dependency-free `dorc-testbed` crate holding the run seed — varied by default, printed, named in
failures — and the seam vocabulary; the clock ticking per invocation; nine cases pinned at
`DORC_SEED=0`; the e2e bless refusing under a second seed). C3b (the transport seam's scripted
column so an apply runs in-process; the consumer stops discarding its intent/outcome;
`apply-outcome-unwritten` as a sibling code) is BUILDING; then D1 (one runner, the derived
driver, the frontmatter collapse) and D2 (the dir-case conversion, the lint fold, `xfail` and
`Posix` out of `internal-tooling`, the doctest noise, yardstick). The arc branch is re-rebased
onto `ai/main` in every between-lanes gap, never mid-lane. Lane briefs are uncommitted, in the
conductor's scratchpad (absolute path in the ledger's §0). Two product rulings were taken inside
this suite arc and stand open to the human's veto (the why-lens relativizes `.`-sourced
dependency paths under the load cwd; the unloaded-sibling-oracle advisory reconciles by
canonical key); lane A's five fence re-targets await re-ack. No sealed review within the arc
(human, 2026-09-02). Builder briefs are never committed (human, same day); a rewound conductor
resumes from the ledger's §0 on that branch, which carries every lane's scope and every ruling.

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
exists (`26N:rul-dollar-zero-authority-spelling`). Every world Dorc stands up carries its own
scaffold and sink, and records never cross a link as records
(`26O:rul-every-world-carries-its-own-scaffold`); which of control / aid / raw rides which
lane is planned per run after measurement and analysis, never fixed by the language, with
collapse a last resort (`plans/26O` §5); the probe lane never holds a pty.

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
