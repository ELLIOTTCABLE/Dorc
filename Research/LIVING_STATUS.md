# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/CLAUDE.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
> findings get a durable numbered-note home; this file carries pointers.
>
> **How to maintain:** update judiciously — direction-changes, discoveries, refutations,
> deferments; never per-turn chatter. The bar: nothing lost to a context-collapse. Density is
> **vaguely logarithmic in age**: the newest work sits at the TOP, rich enough for a new
> conductor to skill up on; day-old context compresses to a paragraph; older history decays to
> a line, a pointer (a note slug, `git log`), or deletion. Reverse-chronological, always.

---

## ROUND 26 — MINTED 2026-07-27: live execution (the next round; planning pending)

**The human's re-cut (typed 2026-07-27):** two months in, the tool has never run against a
real machine; that ends now. r26 = everything to do with piping things *through operating
systems* — the ssh executor (pipe-completeness: `dorc apply host.tld <plan.sh` does its own
ssh'ing, probe side included), the gate/bless-tier live-acceptance loop (real ssh, eventually
real apt-get; never hot-loop), and the Vultr experimentation kit — at SKELETON-TIER
completeness. **THE seed is `notes/26D`**: remit · the settled law to compose
(`142:Resolution` mechanize-ssh + `plans/260` §5 as the adjudicated transport spec, consumed
at N=1) · the as-built inventory (the probe→results→apply chain has NEVER been closed, even
test-only; records `Expect` identity is spike-constants) · the `Research/trial/` salvage map
(`apply-run.sh` · `vultr.sh` · the usekeychain scar on this Windows controller) · first-run
sharp edges (the `255` host-guard elide=0 finding; CRLF) · the open human decisions d1–d5.
A fresh conductor charters from it. Explicitly OUT of r26: stdlib · multi-host · the r25
ceremony · the why/loom prose tails (all gently held, below).

## GENTLY HELD (live work, deliberately waiting on the human's live experimentation)

- **block-stdlib** — zero non-fixture oracles exist; human-ruled 2026-07-27 pending-NOT-
  blocking ("stdlib, multihost, and the r25-first-blood protocols have mostly stood in the
  way of actually experimenting"; scrappy hand-oracles are part of the experiment itself).
  On-ramp when revived: `notes/27Q` (§2 preconditions discharged); prioritization `27Yb`.
- **the why/loom prose tails** — W5 (the `sm `-corpus burn-down; worklist `28J`, 47/209 rows
  transcript-editable at last count) + the `28F`/`28H` human queues (drift-row prose · jargon
  glyphs · `Consented`-knowability at first render · `--no-whylog` spelling · the loom-UX
  friction bank) + the small-sittings ruling queue (floors-ratification `27U` §7 ·
  decline-class starter-set `27W` §0 · C8 operand display `27U` §7 · prose-register schema
  `282` §10 — W4 landed, so that sitting now has transcript faces · lint tc-leans `27S`
  §5/`27T`). All live; none blocks r26.
- **r26 reactive/capture + multi-host revival** — `26B`/`26C` + `260`/`261`/`262`; revival
  conditions `270` §5. NB r26 consumes `260` §5/§2 transport law at N=1 WITHOUT building the
  fleet kernel; the revival inherits whatever SessionDriver seam r26 lands.
- **r25** — the first-blood *protocol* is superseded (the human runs an informal live session
  instead); the tooling is salvage at `Research/trial/` (`notes/26D` §4); the `255` book +
  prediction ledger remain the best live-run instrument (NB `255` §5.1: as-written the book
  measures elide=0 — the `$(hostname)` host-guard walls everything).

## BRANCH / FOLD STATE (measured 2026-07-27 — re-verify before minting r26 branches)

- **Merged into `ai/main`:** the W1–W3 why-surface lanes (w1-voice · w2-data · w2a-adapter ·
  w2b-narrations · w3-fold) · weft-skeleton · ascii-emitters/-sweep · loom-cleanup ·
  opaque-w25 · speechact-rename · d1-drift · r28-impl · r29-catchup · spike3-r26 · spike3-r27.
- **UNMERGED: `ai/r28-unify`** — carries the W4 arc (parts-at-birth/carrier/span-coverage →
  loom-round-trippable why; lanes w4-carrier/-map/-parts/-span/-drifted-driver;
  conductor-blessed @ `747ab48d` per the r28-unify worktree's copy of this file, which stays
  the richer one until the fold). Pre-banked fold conflict: the human's `f4f48316` webhost
  redline (`28H:item-webhost-redline-orphaned`); `28H`/`28I`/`28J` are worktree-resident
  notes until folded. Also unmerged: r28-declined-rerank · r28-precommit-honesty (small
  lanes; disposition with the human) · xcheck/report/relic branches · three
  `*.sync-conflict-*-PHNHRER` twins (SyncThing incursions; `.stignore` repair human-owned,
  `27U` §2).
- **Pruned (measured absence; content presumed folded):** ai/r27-aid · ai/r28-phase3-close ·
  ai/r28-errorloom-phase2 · ai/spike3-r23 · ai/spike3-r25 (r25's content confirmed
  in-mainline at `Research/trial/`, merge `2d5176dd`).

## R28 (CLOSED as an arc — compressed; the durables carry everything)

Five sub-arcs, all BUILT and conductor-cold-verified: **the `280` charter** (errorloom the
standalone crate · the `281` mark-grammar v0.2 corpus respell · the `282` generation flip) —
ledger `notes/28A`; **errorloom phase-three** (the transcript-case prose pipeline end-to-end;
case-first lock generation; durable promote) — ledger `notes/287`; **the aid/loom
unification** (`plans/288` executed whole: `crates/aid` extracted · flat test tree + central
runners · CLI/dorc-sh/lint errors as registry codes · the arrangement registry + help-page
pilot) — ledger `notes/289` (+ maps `290`/`291`); **the why-surface design sitting**
(`notes/28E` rulings · `plans/28G` phased plan · the `28G-why-strawmen-v2/` target corpus ·
`KNOBS:kTASTE`) and **its build, W1→W3** (honest words · the `weft` firewalled box-model
formatting crate · data+narrations · DEFAULT-ON whylog + hardening + SHA-256 · receipt-first
`dorc why` · the `--risk-faultless-skips` rename · D1 drift-disclosed receipts · ASCII
sweeps · loom-cleanup) — ledger `notes/28F`; and **W4** — worktree-resident ledger `28H`,
inside the unfolded `ai/r28-unify` (above). r29 is a quarantined lane
(`quarantine-DO-NOT-READ/`; off-limits, do not ask). r26+ tails beyond the live-execution
remit, banked: emergency-distrust levers · retention design · whylog drifted-wording walk ·
desync-transition machinery (`28G` §2).

**Conduct fences (standing; bind any successor):** repo-durable conduct law lives
in `spike/CLAUDE.md` (Boundaries · Spawning-subagents · Build/test/run) — read it
there. Fences living only here: **git surgery relaxed 2026-07-19** (human-directed:
the deny-hook now permits branch-scoped, reflog-recoverable surgery — rebase /
merge / `reset --hard` / safe branch-delete — in autonomous mode (`ai/*` /
worktree / sentinel); push, stash-drop/clear, `clean -f`, force-delete, tag-delete,
filter-*, update-ref stay blocked everywhere; the human still reviews-and-rebases
AI branches) · merges from `main` batch at round-close · silence ≠ ack (only what
the human TYPED counts; keep an ack-ledger) · crosscheck adjudication under maximum
skepticism; adversarial framing = exclusions-not-inclusions · never AskUserQuestion
(ask in prose); dump the numbered task list on changes · Fable conducts, Opus
codes · conductor: verify merges by own hand (never-vouch); `sh e2e/conduct-bless.sh`
is the verify entrypoint · a promised clean-room re-derivation gets a slugged ledger
entry naming who ran it (`27Xf` §4) · naming discipline (`270` §1, HIGH): hyphenated
full-word slugs; `docID:slug` cross-refs; subscript old labels once ("nee P5") ·
the deferred-work ledger lives in `23O` §5; residue in `24C`.

---

## R27 (CLOSED as an arc 2026-07-18 — compressed; evidence in git + the named durables)

The consolidation round, per `plans/270`: **block-settle** CLOSED (`plans/271`
rulings ledger; durables `272`–`278`; standing rider: netns-ahead-of-fs-view) ·
**block-rebuild** CLOSED (`notes/27D` + `27E`–`27I`: dorc-lang v0.1 corpus
end-to-end, typeless floor, composed-predict probes — the `24J` debt repaired,
entity algebra + backing-SETS, `dorc-records/1`, e2e de-graduation) ·
**block-context** CLOSED (`27K`/`27L`/`27N`/`27O`/`27P`: wrapper-peel, payload-v1,
context-entry + shim materialization, pure-predicate carry; **`plans/27C`** = THE
kept-current wrapper/context spec) · **read-value/capture** STRUCK to the r26
revival (`26B` reactive plan-construction + `26C` fixpoint semantics; secrets-seam
deadline moved to `26B:need-scrub-before-freeze`) · **`dorc lint`** landed
(`27R`/`27S`/`27T`) · **the user-aid design sitting** minted root `AID-NEEDS.md` +
`27V`/`27W` + USER_STORY's "Recovery" section, then the aid build phase executed
whole (`notes/27U` = the as-built ledger) · **human-facing docs** (`spike/docs/` +
`spike/skills/author-oracle/`) minted 2026-07-18.

## Older

Round 24 (CLOSED by reshuffle): `notes/24U` is the full accounting; the round-23
oracle-contract crisis + settled law: `notes/23O`. Everything else: the per-round
map in `Research/README.md`.
