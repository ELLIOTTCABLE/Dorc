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

## CURRENT STATE (measured 2026-07-31)

**The loom-final arc is CLOSED and FOLDED**: `ai/r28-loom-final` == `ai/main` (the earlier
"awaiting the human's fold ack" claim here was stale — the fold happened). Conduct ledger
**`notes/28L`**; the arc-close accounting of every prose surface **`notes/28N`**. Still pending
from that arc: the **records-8 emitter decision** (delete-under-no-compat vs the r26-revival
wiring) — human's.

**THE kernel plan: `plans/28Q` — context-kernel unification (minted 2026-08-01).** The
single home for the analysis-kernel refactor, by human order NO MORE PIECEMEAL: P1
definition-factored positional indices (frames) · P2 entry-closure as the speaker · P3
universal context-availability, staged i–iii. It supersedes the `28P` bitem4/bitem5 hold,
absorbs `26K` §0b (local-exec/scopes/wait-loops → stage-iii), and discharges the
terminology rider; §11 (the authored surface — how users spell these concepts in oracles)
is RESERVED for the next human-led design dig; two term slots ([TERM-A]/[TERM-B], né
epoch/transit) await a dispatched terminology survey. Execution awaits the human's ack.

**`ai/r28-oracle-loading`: the `28K` lane is EXECUTED and CLOSED on its branch** (resume
conduct-ledger **`notes/28P`**; build ledger `notes/28O`): bitem0–3 and 6–9 LANDED, both
legs green; bitem4/bitem5/withhold-softening/meet-direction-registry HELD by human order
and now inherited by `28Q` stages i–ii. Live rulings routed OUT of the lane:
`tc-split-family-elides-on-two-authors` (composite-license admissibility — the
committee-corner sitting's) · `tc-inert-mocks-rail-is-dash-shaped` (posh has never
executed a corpus emitter body; separate lane) · the blessing pair — `command -v`
RESOLVED 2026-08-01 as a stdlib-oracle answer (`28Q` §5), `.`-of-proven-load-inert still
human-owned (`28Q:res-dot-blessing-is-engine-side`). `ai/r28-cli-inputs`' two commits
were cherry-picked into the lane (branch tip-redundant → advisory-delete queue).

**NOT STARTED**: the `26K` §0a fruit arc (still zero code; parallel-anytime). The §0b
kernel sitting is no longer a separate item — superseded into `28Q` stage-iii.

## CLEANUP QUEUE (branch deletion is human force-delete gated; verdicts below are advisory)

- **Four `*.sync-conflict-*-PHNHRER` branches** (SyncThing incursions, no unique content): the
  `ai/r26-strawmen-{k8s,osnix}` twins are already reachable from main; the
  `ai/r28-declined-rerank` and `worktree-agent-aba0f…` twins are tip-identical to their base
  branches.
- **13 orphan `worktree-agent-*` branches** (round-23/24/27-era; 6 reachable from main, 7 not —
  ~SUSPECT superseded rebase leftovers, unverified; verdict-free, pre-established).
- Loom-final backup/review branches: NONE remain (`git branch --list '*backup*' '*review1*'
  '*review2*'` is empty).
- `git worktree prune` DONE 2026-07-31 (removed the 4 stale entries: r26-accept, r28-impl,
  spike3-r26, spike3-r27). `ai/r26-executor-blocked` no longer exists (human deleted).
- Untracked in the primary checkout, flagged to the human: `dorc-temp-key{,.pub}` +

## r26 — CLOSED (live-execution + kernel arc + glue-residue research; compressed 2026-07-31)

Dorc ran against a real machine and the numbers held (real-ssh probe → real apply → converged
re-plan byte-identical; `mise run livetest` = the containerized acceptance loop). What must
survive the close:

- **THE open human ruling: `fnd-classed-decline-unwalls-guard-tier`** (`trial/r26/predictions.md`
  §7; the `guard26-*` case pair) — classing an honest decline yields a strictly worse plan than
  shipping no oracle at all (vouched sites below a classed decline lose the guard tier). The
  repair is licensing-tier — it WIDENS what guards — and is deliberately unmade.
- **The unspent `26D` d3/D9 ruling** — real apply outcomes into the whylog: the remote-apply
  path short-circuits before `write_whylog`, so `WhylogDoc.apply` stays plan-time prediction.
- Pointers: conduct ledger `notes/26F` (live-execution close) · `notes/26G`+`26H` (kernel arc;
  26G only WITH its three appended corrections) · `notes/26I` (adversarial kernel review;
  maximum-skepticism law applies) · `notes/26J` (builtin-deny; its residue is needs-human) ·
  ops-glue-residue round: `KNOBS.md:kBOOT` · root `SIBLINGS.md` · **`plans/26K`** (THE plan;
  §0 is the actionable head) · `notes/r26-glue-strawmen/` (frozen evidence, never execute) ·
  full adjudication ledger `.claude/research/ops-glue-residue/round-charter.md`.

## BRANCH / FOLD STATE (re-measured 2026-07-31)

- **Live, unmerged:** `ai/r28-oracle-loading` (38 ahead; rebased, gate-green, parked at E→F) ·
  `ai/r28-cli-inputs` (2 ahead; rebased clean).
- **Queued for human deletion:** the cleanup-queue branches above (siblings-audit, the three
  strawmen lanes, the four sync-conflict twins, the worktree-agent orphans).
- **Unreviewed residue, disposition with the human:** `ai/r28-declined-rerank` (3 ahead) · the
  five `ai/r28-xcheck-*` report branches · relics `ai/24Ka-langreview`, `ai/r22-x3a`/`-x3fix`/
  `-xcheck1`, `ai/r27-review-base`, `ai/snapshot`, `ai/spike3-r23-*` (report/review branches).
- **Everything else is folded** (loom-final, the whole r26 chain, r28-impl, spike3-r26/r27).

## GENTLY HELD (live work, deliberately waiting on the human's live experimentation)

- **block-stdlib** — zero non-fixture oracles exist; human-ruled 2026-07-27 pending-NOT-
  blocking. On-ramp when revived: `notes/27Q` (§2 preconditions discharged); prioritization
  `27Yb`. NOW ALSO GATED on a dialect-reach decision:
  **`28O:fnd-dialect-tests-admit-only-string-comparison`** — the R2-SHADOW quality bar needs
  the unary file-test family (`-x`/`-f`) that the check dialect refuses, so an author cannot
  write an R2-SHADOW-clean existence check that also lifts.
- **why/loom prose tails** — largely SUPERSEDED by the loom-final arc (the loom is now the
  working prose-edit surface end-to-end; post-arc residue is enumerated in `28N` §3 and
  nowhere else). Still standing: the W5 `sm `-corpus burn-down is human-owned and its `28J`
  worklist needs a RE-AUDIT before more human time goes in (28J's "7 unwritten codes
  loom-editable today" was verified WRONG 2026-07-27); the small-sittings ruling queue
  (floors-ratification `27U` §7 · decline-class starter-set `27W` §0 · C8 operand display ·
  lint tc-leans `27S` §5/`27T`).
- **r26 reactive/capture + multi-host revival** — `26B`/`26C` + `260`/`261`/`262`; revival
  conditions `270` §5; inherits the records-8 decision and whatever SessionDriver seam r26
  landed.
- **r25** — the first-blood protocol is superseded; the tooling is salvage at `Research/trial/`
  (`notes/26D` §4; NB `255` §5.1: as-written the book measures elide=0).

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

## R28 (CLOSED as an arc — compressed; the durables carry everything)

Six sub-arcs, all BUILT: **the `280` charter** (errorloom standalone · `281` mark-grammar v0.2 ·
the `282` generation flip) — ledger `notes/28A`; **errorloom phase-three** — ledger `notes/287`;
**the aid/loom unification** (`plans/288`: `crates/aid` extracted · flat test tree + central
runners · the arrangement registry) — ledger `notes/289` (+ maps `290`/`291`); **the
why-surface sitting + W1→W4 build** (`notes/28E` rulings · `plans/28G` · ledgers `notes/28F`
and `notes/28H`, red-lines `28I`, W5 worklist `28J` — re-audit caveat above); **THE LOOM-FINAL
ARC** (2026-07-29→31: the stamped-provenance boundary weld · placeholder-overtype as the
words-mint path · compile-forced params · ownership declarations · the foreign-text seal ·
reason enums · ~176-case corpus · six main.rs lib-seam extractions) — conduct ledger
`notes/28L`, prose accounting `notes/28N`; and **the r28 name-resolution sitting** that minted
`plans/28K` + `plans/28M` (the live lane above). r29 is a quarantined lane
(`quarantine-DO-NOT-READ/`; off-limits, do not ask). r26+ tails beyond the live-execution
remit, banked: emergency-distrust levers · retention design · whylog drifted-wording walk ·
desync-transition machinery (`28G` §2).

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
