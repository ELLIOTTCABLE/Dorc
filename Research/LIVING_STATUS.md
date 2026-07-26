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

## THE WHY-SURFACE DESIGN SITTING (closed 2026-07-25) → THE `28G` IMPLEMENTATION QUEUE

**Design CLOSED, implementation PENDING a fresh conductor.** The record is
**`notes/28E`** (rulings §0/§8 · the mint/carry/compose/render synthesis · the
prior-art adjudication §7); the phased plan-of-record is **`plans/28G`** — read
BOTH before any why/render/aid work. The strawman corpus of record (design
targets, human-marked): **`notes/28G-why-strawmen-v2/`** (five cases; the human
is still working them over slowly — expect further markup layers; `292` is
superseded). Sibling inputs: `notes/28D` (firefight needs + the opaque-approve
gates, human-marked) · `.claude/research/dag-explanation-ux/` (100 graded
sources). New knob: `KNOBS:kTASTE` (delightful ↔ productive; deliberately
permanent). Headline rulings a successor must not re-derive: reported-never-
measured · quoted-speakers rows · completion-class danger axis (`*`/`!`) ·
never-a-dinna-do-it-layer (negatives only on engine-proved derivations) ·
trust-spent-first · renderer-owns-layout · ASCII-forever · everything
rendering-assembled-from-editable-spans (`28G` §0 — the living goal). Sizing
recc (`28G` §3): ONE more r28 implementation-conductor for phases W1→W2(→W3);
r30 for W4 + levers/retention. Phase W5 (= the held phase-8 prose pass) still
awaits the human's ack. First-blooding is slightly BLOCKED on W1/W2 per the
human ("the reports and errors ARE the product").

## THE AID/LOOM UNIFICATION ARC (`plans/288` EXECUTED WHOLE — closed 2026-07-24)

**BUILD COMPLETE, awaiting the human's fold of `ai/r28-unify` and the HELD phase-8
prose pass.** One sitting, ten lanes, all conductor-cold-verified (final:
`unit 1200 | e2e 98 | looms 73 | gates ok`). THE durable is **`notes/289`** (rulings,
landings, seams, the phase-8 package); the specs it executed: `plans/288` (+ maps
`notes/290`/`291`). What landed: `crates/aid` extracted (describe plane;
Narrative rename; two-plane seal intact) · the mint-seam + `dorc-loom scaffold` +
the mint-hardening gate package (caught a never-minted collapse class day one) ·
lint findings + ~21 CLI/dorc-sh errors as registry codes (all `sm `-migrated) ·
the flat test tree + central Rust runners (`run.sh` RETIRED; sanctioned-executor
law re-worded) · e2e→loom convergence machinery (closed normalizer vocabulary;
catalog-validated needles — which caught a rounds-dead fixture) · the arrangement
registry + help-page pilot (17 chrome rows loom-editable; artifact-plane fenced
OUT) · the disturbs-family def↔invocation LIVE BUG fixed + pinned · cross-platform
gate repairs (a never-type-checked `#[cfg(unix)]` path; WSL git≥2.48 wall;
conduct-bless pre-flights). Research banked: `.claude/research/loom-harness-
alternatives/` (errorloom publish-narrow lean; mock-stub crate candidate).
**Human queue:** the phase-8 prose pass (catalog `sm `/6-unwritten + 17 arrangement
rows; two transport seams pair with it) · WSL git upgrade + mise activation ·
SyncThing-writes-into-`.git` (4 conflict refs surfaced) · three dirty worktrees
(incl. uncommitted 279a/b Sol-N reports) · errorloom LICENSE/publish forks.

## ERRORLOOM TEMPLATE COMPILER (post-round-28; phase-three CLOSED 2026-07-23)

**MERGE:** the round-29 ingress `ai/main`@`b6fde355` is folded into `ai/r28-phase3-close`
(merge `5d74bfcc`), awaiting human review; `ai/main` was live-advancing at merge time —
re-verify before fold. Host-evidence catalog surface was bootstrapped into the case-owned
lock (case frontmatter + generated entry); r29's whylog bounded-admission flow was taken
whole and the `--whylog=` single-file selection woven in front of it.

**PHASE THREE UNITS 1–3 COMPLETE on `ai/r28-phase3-close` (unfolded).** The durable
transcript-case prose pipeline is built end-to-end: the committed transcript CASE is
the authoring surface and the generated `catalog_lock.rs` is DERIVED from it. As-built
ledger: **`notes/287`** (re-homed from the `28A` rewrite; the historical `28A` conduct
ledger is restored in place). Design authority: `plans/282`.

- **Unit 1** (inherited, `3e980b86`): consumer-neutral replay-driver/result seam +
  reusable generic executor; Dorc exact-shape in-process dispatch + explicit generic
  fallback; read-only `compile`/`vars` inspection over exact replay-result provenance.
- **Unit 2** (inherited, `a3dcc3d0`): content-bound plain-text receipt (exact case +
  catalog bytes + typed inspection); promote re-runs the inspection and requires exact
  packet equality; git touched-set gate (prose-only; dirty generated catalog refuses).
- **Unit 3** (`ai/r28-phase3-close`): the case-first lock generator (`dorc_loom::generate`
  `generate_catalog_lock`) + `core::catalog` `LockRow`/`serialize_lock` (the old
  catalog-first `serialize`/`promote_catalog_source`/`schematic_example` retired). Case
  frontmatter bootstrapped with `when-fires`/`why` (22 case-owned + 35 ratcheted = 57).
  `catalog_lock.rs` regenerated WHOLE (`#[rustfmt::skip]`, single-line, no dividers;
  case-owned `example` = concrete payload render). `promote` computes the whole candidate
  set (lock + touched cases) + BOTH fixpoints before any write, then publishes lock-first
  + affected cases in lexical order by per-target temp-file-and-rename. `DORC_CATALOG_PROMOTE`
  and every env-gated/splicing promote path are gone. Second fixpoint gate
  (`generated_lock_reproduces_the_committed_bytes`) added.

**DOGFOOD PROVEN + REVERTED:** `{{command}}` inserted through the flagship
`cmdsub-operand-top` transcript alone regenerated the lock (params/message/example) and
re-rendered the case, both fixpoints green (proof `25e1bee7`, reverted `b2d9261a`).

**DEFERRED (documented, `287` §11):** the committed `dorc-loom vars --used CASE` blocks in
case files (additive editor-aid; needs `render_direct_replay` + whylog `editable_baseline`
generalization) · adjacent/glued markers (`282:phase-adjacent-fragment-followup`; the
dogfood used a standalone marker).

**NEXT (human):** fold `ai/r28-phase3-close`; the prose-quality pass over `[unwritten:]`/`sm `
text; the deferred `vars --used` blocks + adjacent-marker phase.

## ROUND 28 (seeded 2026-07-19 — the current view)

**BUILD COMPLETE (2026-07-20) — awaiting the human's fold of `ai/r28-impl` @ `77ebd8e`.**
The human-directed flagship render-polish (`282` §12 / `28A` §2z-post-2) is FOLDED at
`77ebd8e` (2-parent merge of `ai/r28-flagship-polish`), conductor-cold-verified
(cargo clean → build → cold-clippy-0 → workspace tests → fmt → e2e 97/97); its
builder-flagged deferrals + two conduct loose ends (the trailer commits, the offered
`commit-msg` hook) are banked in `28A` §2z-post-2.
The whole `280` charter landed on the conductor stack, each lane conductor-cold-verified
(the incremental-clippy-serves-stale finding `28A:finding-incremental-clippy-serves-stale`
made cold verification mandatory): **errorloom** the standalone crate (d1 transport + d2
runner/orchestration/CLI + the `285` DeepSeek review → last-polish pass); the **syntax v0.2
respell** (the `281` mark-grammar cutover — `@` selectors, word verbs, `safe-across`/
`undivided-by-transit-across`, unified `disturbs`, `#:` carrier, `# dorc-lang/v0.2`); and the
**errorloom→workflow integration** (the `282` generation flip: catalog case-derived,
promote-v2, roster retired, + phase-5 backport of all covered codes; de-passthrough KILLED,
handed to the opaque sibling lane). Docs/steering/registry re-synthesized to v0.2. The ONE
durable ledger is **`notes/28A`** (all rulings, the ack-ledger, the deferred-queue). Sibling
durables: `28B` (respell map) · `28C` (janitor sweep) · `283` (gen-flip map) · `285a–d`
(errorloom review + adjudication) · `284` (taint hand-off, quarantine). **Human queue
(all banked in `28A`, none blocking):** the Fable `sm `-prose pass (the `[unwritten:]`
+ `sm ` codes) · the catalog canonicalization · the glued-param re-hole seam · the
`covered()⊆case-owned` drift guard (cheap) · the errorloom LICENSE/publish + Cargo metadata
+ the non_exhaustive-error-enum publish-taste fork. Opaque-review NOT run this round
(human-typed exemption, `28A` §4b). Root-doc v0.2 spelling: human-done.

**Where we are:** the user-aid build phase is COMPLETE (below), and round 28 opens
on two seed documents. **`plans/281`** (the annotation mark-grammar — THE spec of
the line-annotation surface: one-intro+sugar grammar, the word-verb vocabulary,
the `@` selector, the `#:` comment carrier, rc-arity, the salience design-goal;
supersedes `notes/277` §4's worked minimum, takes over the grammar `278` §6
deferred; its closing grep-map drives the corpus respell, which must land BEFORE
block-stdlib stamps prior spellings into the seed corpus). And **`plans/282`**
(the transcript-case prose pipeline, working name `wordloom`): user-facing prose
is authored at the *transcript* surface — executable case files showing exactly
what a user sees — and the compiled catalog is DERIVED from them
(txtar+frontmatter cases · words-and-paragraphs prose model · tagged-render +
word-diff transport · prose-bless/structure-bless exclusivity · git-gated
promote · type-gated passthrough), reversing the aid phase's catalog-first
as-built. **`plans/280` (the charter) is MINTED and human-acked (2026-07-19):**
`lane-errorloom-crate` ∥ `lane-syntax-unification-respell`, then the serial
`lane-errorloom-unify`; block-stdlib waits on the respell at minimum; the `#:`
carrier is acked and `KNOBS:kSALIENCE` registered (`7851eeb`, ai/main); the
pending-ruling riders are banked at `TODO-ADDTL.md`'s tail (blockers-only
discipline: the charter names none).

**The aid build phase (CLOSED 2026-07-19; the ONE durable = `notes/27U`):** the
`27V` plan executed whole across seven serial dispatches on **`ai/r27-aid`** (base
`380f2fa`, off `ai/main`): legacy-`Diagnostic` killed → the one catalog
(`core/src/catalog.rs`; three-state prose protocol; defining-case ratchet 17/52) →
sealed evidence plane (all nine collapse classes; minting-line/file:line
attribution — the `27Q` §2 stdlib precondition DISCHARGED) → whylog durable +
`dorc why --last` replay → the `27W` report lane (all three tiers) → the
arrangement walker + THE FLAGSHIP GREEN (`survivebite27-naked-trust-chain`, live
AND replayed) → lint absorption + rung-oracle-solo → caret plumbing → docs/skill
refresh. 958 unit / 97 e2e / four gates, conductor-verified at every advance.
Registry + law: root `AID-NEEDS.md` · `spike/CLAUDE.md` User-aid block · design
notes `27V`/`27W`. Incidents + protocol verdict: `27U` §2/§4 (worktree-file-access
law candidates in §5). NOTE: `282` supersedes-in-part the catalog-pipeline shape
described in `27V` §3 and built by `27U` d1/d4b — read `282` §0/§8 for what changes.

**Open human queue:** (1) the fold of `ai/r27-aid` (one command; carries the aid
phase + the round-28 seed) · (2) the **rider dump at `TODO-ADDTL.md`'s tail**
(2026-07-19, thirteen slugged items: floors, ratchet, soft-acks, flag rename,
prose registers, lint tc-* leans, small fixes, SyncThing) — to be dug into with a
fresh conductor, interactively; none blocks the 280 lanes.

**Round-28 read-first:** root docs AT HEAD → `spike/CLAUDE.md` (steering law; the
User-aid block; NB the authored-surface block still teaches the pre-`281` mark
spellings until the respell lands) → **`plans/280`** (the charter: lanes, bless
discipline, horizon) → **`plans/281`** + **`plans/282`** (the seed specs) →
`notes/27U` (the aid as-built) → for block-stdlib: `notes/27Q` (preconditions;
read §2 before ANY oracle is authored) → per-task pointers via
`Research/README.md`'s topic index.

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

**Branch map:** **`ai/r28-phase3-close`** = phase-three CLOSE (units 1–3; the durable
promote loop + generated-lock cutover; unfolded, awaiting the human's fold) · based on
`ai/r28-phase3-unit3-lock` @ `2cdfdfbc` · `ai/r28-phase3-unit1-cli` / `-unit2-receipt` /
`-unit3-lock` = the superseded per-unit lanes, folded into the close · `ai/r28-errorloom-phase2`
= accepted transport/template-compiler base ·
**`ai/r27-aid`** = the older conduct stack — the whole aid phase +
the round-28 seed (`281`/`282`/this trim); awaiting the human's single fold ·
`ai/main` = the human's integration playground; carries everything through
`a651fe8` (2026-07-18) plus their interleaved commits; residual `main`-vs-`ai/main`
topology is theirs · `ai/r27-aid-*` + `ai/r27-lint*` = lane branches, contained in
the stack · `ai/spike3-r23` = the old r24–r27 lineage (historical) ·
`ai/spike3-r27` = FROZEN ref at `1aecaa3` (human's) · `ai/spike3-r25` (field-trial
tooling) + `ai/spike3-r26` (multi-host plans; revival bank `26B`/`26C`) = dormant,
revival conditions at `270` §5.

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
(`27R`/`27S`/`27T`; opt-in real-tools lane; advisory-only by construction) · **the
user-aid design sitting** minted root `AID-NEEDS.md` + `27V`/`27W` + USER_STORY's
"Recovery" section · **human-facing docs** (`spike/docs/` +
`spike/skills/author-oracle/`) minted 2026-07-18 · the human merged the whole r27
stack into `ai/main` at `a651fe8`, conductor-verified. **NEXT per `270` §2 (as
amended):** block-stdlib under a NEW human-led conductor (on-ramp `notes/27Q`),
then yardstick-measurement, then the r25 field-trial revival
(+ `26B:ask-trial-counts-capture-walls`), then the r26 resumption — subject to
re-sequencing by the pending `plans/280` charter.

## Older

Round 24 (CLOSED by reshuffle): `notes/24U` is the full accounting; the round-23
oracle-contract crisis + settled law: `notes/23O`. Everything else: the per-round
map in `Research/README.md`.
