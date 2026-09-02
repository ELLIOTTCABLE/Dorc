# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record; `Research/README.md`'s per-round map says what each closed document
> did) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/CLAUDE.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
> findings get a durable numbered-note home; this file carries pointers. Upcoming and owed
> work is NOT here either: that is root `ROADMAP.md` (scheduled · owed · the rulings the
> human owes), with `TODO-ADDTL.md` as its unacked porch.
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

## IN FLIGHT (2026-09-02 — the test-architecture REBUILD; design cleared on the second sealed review; lane A building)

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
branch, unopened by the conductor). Lane A (the `Seams`/`HarnessSeams` bundle, `compose::run`,
`dorc-harness`, seeded entropy, the six env pins retired) is BUILDING in the conductor
worktree; checkpoint after it. A rewound conductor resumes from the ledger's §0 on that branch,
where the lane briefs, the scout inventory, and the review dispatch kit are banked beside it.

## DESIGN SITTING (2026-08-31 → 09-02 — pivot-book language surface; OPEN, banked for a rewind)

Fable⇄human design sitting toward the human's concrete target (an idempotent Vultr
standup + first-config + apps book). Ledger and live queue: **`notes/26M`** (the
ack-ledger — only typed acks count; the tabled apply-side-transport exploration; the
payload-declaration decomposition; the recast trail). Design-of-record: **`plans/30W`**
(index-kinds in the context slot; region OVERLAP; the lifted ask-the-owner discipline; the
per-arm claim family; six rulings owed in its §10). Nothing welded; the license-consuming
half stays behind `kSURVIVAL` and the human's economic deferral. Resume from `26M`'s queue.
The sitting's output is now the spine of the announced next round:
`ROADMAP:round-r31-language-and-kernel` (candidates awaiting the human's in/out).

## CLOSED ARCS (pointers only)

The r30/30R close arc — receipt residue, the whylog→receipt recast, the receipt-backed
`dorc why` surface — is COMPLETE and folded (2026-08-31; conduct `notes/30Va`, lane ledger
`notes/30Vd`). The r30 kernel close-out — every `30O` lane BUILT and folded (2026-08-23;
schedule `notes/30O`, conduct `notes/30Q`, design `plans/30P`). Accounts: the README r30
entry. What remains of both is on the roadmap: `ROADMAP:ceremony-r30-close` (the human's) and
the rulings list there (formerly the "human-gated" and "gently held" sections of this file).

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
the `an-kind-reach` row and steering carry it; the build is unscheduled).

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

## History

Closed arcs and their accounts: `Research/README.md` per-round map. r30's ledgers in order:
`notes/300` (first half) · `307` (wave two) · `30N` (second half) · `30O` (close-out) ·
`30Va` (the receipt/why close) · `30Xa` (the test-architecture rebuild, in flight).
