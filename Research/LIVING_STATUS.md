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

## THE LOOM-FINAL ARC — CLOSED (2026-07-31, branch `ai/r28-loom-final`; awaiting the human's fold ack)

The human's now-or-never order (2026-07-29) executed whole: looms are the working
prose-edit surface, end-to-end, empirically gated. Conduct ledger **`notes/28L`**
(every directive, ruling, and lane landing); the arc-close accounting of EVERY prose
surface **`notes/28N`** (transcript-faced ~181 components + 87/95 catalog codes ·
lock-tier 31 with the remedy priced · never-loom law-cited · records-8 pending the
human's emitter decision). Headline machinery: the stamped-provenance boundary weld
(byte-shape re-detection DELETED; `transcript_bytes_equal_production_bytes` standing) ·
placeholder-overtype IS the words-mint path · the Rust persona never touches loom
internals (compile-forced params; fixture worlds beside payloads) · ownership
declarations · the records seam (opaque-ACK'd, 28-reviewA) · the foreign-text seal
(`is_foreign_param` dead) · reason enums (`detail: String` extinct) · 176-case corpus
incl. measured/survival why-worlds · six `main.rs` lib-seam extractions · SyncThing
residue excluded from every corpus walk. The two-render-chain diagnosis and the whole
map: `_loom-final-map-DRAFT.md` in the arc's git history. `28J`'s false
"editable-today" claim carries its supersession note. Blind-reviewer round 1 chafe
built in; round 2 pending at close. Superseded/backup branches for the human's
force-delete are listed in the close report.

## ops-glue-residue RESEARCH ROUND -- CLOSED (2026-07-28, on `ai/main`; interactive, human-adjudicated throughout)

Direction-setting round on the glue phase (pre-ssh lifecycle  /  transport reach  / 
pivot/topology  /  dorc-inside). Deliverables LANDED: **`KNOBS.md:kBOOT`** (transport
floor; capability-probing-per-feature human-ruled)  /  root **`SIBLINGS.md`**
(three-posture framing table, audit-hardened by four column-advocates,
human-review-in-place)  /  **`plans/26K`** = THE plan + synthesis (its §0 is
the actionable head: the fruit arc; then THE kernel sitting -- titular =
local-exec-as-supported-mode, FIRST because local-exec needs epochs, one
design+build batch rolling in scope-typing and wait-loops; stdin-amendment
banked design-first)  /  six imagination-tier books at
`notes/r26-glue-strawmen/` (frozen evidence, never execute). Full adjudication
ledger + 183-source base: `.claude/research/ops-glue-residue/round-charter.md`.
Headline rulings (human-typed): pivot must-support  /  splice-is-the-floor  / 
paste-tier bottom rung  /  rung-zero paradigm-unification  /  transit-relative epoch
law (elision crosses CONVERGED transits). Strawman lanes folded and deletable
(`ai/r26-strawmen-{cloud,osnix,k8s}`); opaque-review gate: standing deferral
stands (human 2026-07-24). NB untracked `dorc-temp-key{,.pub}` +
`spike/mykey-known-hosts` flagged to human (pre-existing, uncommitted).

## r26 LIVE-EXECUTION — ROUND CLOSED (2026-07-29; branch `ai/r26-unify` @ `2248a1ba`, conductor-blessed `gates ok | suite 1605 | e2e 112 blessed`)

Dorc ran against a real machine for the first time, and the numbers held: real-ssh probe
(~1s) → real apply (nginx installed on the r26 Vultr box) → converged re-plan byte-identical
to prediction → ceiling `elide=4` matching the hermetic baseline EXACTLY. `mise run
livetest` = the containerized acceptance loop (green ~32s; wslc/docker-generic seam);
`livetest:remote` = the same loop vs any host. Conduct ledger + full pointer index:
**`notes/26F`**. THE open human ruling: **`fnd-classed-decline-unwalls-guard-tier`**
(`trial/r26/predictions.md` §7; `guard26-*` case pair) — guard-tier class-widening,
licensing-tier, absorbs the W-B ambient-past-wall bank. Morning queue: the uncommitted
`CONTRIBUTING.md` draft in the primary checkout (voice + snippet-vs-inline) · `git branch
-D ai/r26-executor-blocked` (hook-reserved) · `model-local` next increment ·
`TODO-ADDTL.md` top section re-cut. The box idles at `140.82.10.231` (~$0.0137/hr,
teardown human-ack-only, `26E`).

## r26 UNIFIED (branch `ai/r26-unify`, off `ai/r28-unify` @ `9050248e`)

All four r26 lanes folded onto one branch, both legs green (Win 1592 / WSL 1588, 1 skipped
each; `bless:dry` clean): analyzer-findings (ff) · adversarial-review (`26I`) · executor
(the transport seam) · live-target (`trial/r26` + `26E`). Only two conflicts, both expected:
the generated `catalog_lock.rs` (union-resolved, then verified AT the loom generator's
fixpoint — `loom:compile` reports 62 cases / 0 touched) and one `dorc-loom` import block.
**ZERO pre-existing goldens moved** — every transcript change in the arc is an ADD, so the
feared cross-lane interaction (the executor's T1 closed-loop running over the kernel arc's
changed dispositions) did not materialize.

Hygiene settled on the branch: **the `260:dec-26-exit-codes` collision is closed** — 11 was
never free (`EXIT_WRAPPER_INCOHERENT`), so the transport family renumbered to one code per
world-state (12 ingress-refused · 13 host-not-reached · 14 session-lost · 15 apply-failed),
matching the diag plane's cut; `260` §6/§9 rewritten to match. **D9 (real apply outcomes into
the whylog) was LEFT, not partially built**: `run()` short-circuits to `ship_consented_apply`
before the pipeline, so the remote-apply path never reaches `write_whylog`, and
`WhylogDoc.apply` stays plan-time prediction — `26D` d3's human ruling is still unspent.
STILL OPEN: deleting the superseded `ai/r26-executor-blocked` (@ `4acd4543`) is human-gated
by the branch-deletion hook.

**CLOSING LANE (branch `ai/r26-guard-fix`, off `ai/r26-unify` @ `bcd685a6`).** Folded
`ai/r26-livetest` in — clean, no conflicts (its stamps live in `trial/r26/`, not here). Both legs
green (Win 1605 / WSL 1601, 1 skipped), `bless:dry` clean, `mise run livetest` green in 33 s with
both container baselines matched. The 22-head builtin-deny and the r26 smoke-kit do NOT interact:
re-deriving the kit's hermetic baselines under the deny moves no disposition (its oracles use no
denied head; `shift` stays a modeled keyword). Also landed: `ControlMaster=no` + `ControlPath=none`
pinned into the ssh driver's non-negotiables (`260` §5 rewritten — it had proposed the opposite),
and a `livetest:baselines` task so the renders are re-derivable in one command.

**`del-authored-coordinate-voids-guard` IS MISDIAGNOSED — do not fix it as reported.** The
authored coordinate is innocent and W-B's keying is symmetric (mint and consumption read the same
`SkipClass` fact; the two cases below emit byte-identical probe artifacts and differ only in the
apply). The real defect is **`fnd-classed-decline-unwalls-guard-tier`**, bisected and written up in
`trial/r26/predictions.md` §7: an unmodeled command is `Opaque` and WALLS, but one bearing a verdict
function establishes a cell instead (`effect::verdict_cell_or_auto` rows 1/2) and stops walling, so
vouched drops below it fall from `EstablishWritten` to `EstablishAmbient`. They still sit below a
live mutator, so elision is correctly refused — but the guard tier, whose whole purpose is that
case, is keyed to `EstablishWritten` and unreachable. So **classing an honest decline yields a
strictly worse plan than shipping no oracle at all**, for every vouched site below it. The §6
strip-the-mark A/B was confounded: removing the mark collapses both sites onto the shared auto-cell,
so the second is stomped by the first and recovers a guard through an accidental wall. Pinned as a
whole-product pair (`guard26-unmodeled-wall-guards-below` control, two guards ·
`guard26-classed-decline-demotes-guard` defect, two mutating drops ship), which also closes §6's
corpus gap. The repair is a licensing-tier change — which class may reach the guard mint — so it
WIDENS what guards and is deliberately unmade: it wants a human ruling, not an overnight patch.

## THE r26 KERNEL ARC (2026-07-27/28 overnight — CLOSED; branch `ai/r26-analyzer-findings` @ `02ccf6e1`)

Human-escalated mid-round: four analyzer findings from the live-execution prep became a
checkpointed kernel-fix arc under Fable-conductor protocol (findings-only diagnosis →
by-hand scout+plan → map-then-execute per workstream → conductor bless/inspection each).
THE durables: **`notes/26G`** (adjudication + THREE appended corrections — read only WITH
them) · **`notes/26H`** (plan-of-record; its header stamps the closing status) · the conduct
detail in the session scratchpad ledger (temp-durable). LANDED, each both-legs green:
loud-degrades (W-A) · and-or lists (W-D — THREE wrong-yes defects closed: `&&`-swallow,
lone-`&`, `||`-true forged-vouch; `[T]||return N` + `cmd||return N` closed forms per human
ack) · verdict-mark keying (W-B — authored coordinates split cells; registry law in
core/analysis/plan CLAUDE.mds) · validity fixpoint via erasure (W-C — ladders cascade;
type-gated ledger; cap-degrades-to-origin). **ADVERSARIAL REVIEW (authorized single
Fable pass): `notes/26I-adversarial-kernel-review.md` on `ai/r26-adversarial-review`
@ `ef8b47fa`** — headline: `26I:fnd-state-builtins-silently-mis-key`, a LIVE
wrong-yes-capable hole (oracle-body `set --`/`unset`/`eval` ship-but-don't-model at the
plain-command fallthrough; same family as the arc's three; conductor-verified at the
committed-evidence tier; cheap fix sketched in the note) — **the successor's first kernel
item** — plus the unpinned W-C-monotonicity⇄merge-⊤-paranoia coupling (wants registry
bullets + an agreeing-sibling tripwire case) and three lesser findings; nine suspicions
cleared with evidence, incl. the erasure-fence/render weld under directed attack.
Adjudicate all of it under standing maximum-skepticism law; single-pass, no neutral
corroboration. r26 LIVE-EXECUTION state (executor landed, box up, acceptance lane held,
CONTRIBUTING deliverable pending) is in the scratchpad ledger + `notes/26E`; the human
resumes it in a fresh window.

**`26I:fnd-state-builtins-silently-mis-key` — CLOSED** on `ai/r26-builtin-deny` @ `fb53b859`
(off `ai/r26-unify`), durable **`notes/26J-builtin-deny.md`**. Human-framed side-lane: DENY the
unmodeled heads, do not model them. 22 heads refused at the two tracers' plain-command arm
(`predict/eval.rs` `run_stmt`, `verdict.rs` `run_command`), riding the seats' existing ⊤ channels
— new `TopReason::StateMutatingBuiltin` surfaces through `site-unresolvable`'s named-cause render;
`VerdictTop::StateMutatingBuiltin` never vouches. `26I`'s evidence reproduced BEFORE the fix
(resolved `Operand("install")` while the shipped `probe_body` carried the `set -- alpha` span;
whole-product `site 0 effect=holds` on a cell never measured), pinned after at both tiers. The
adjudication's load-bearing corrections: `:` is the MARK CARRIER (26 in-role uses — denying it
would delete `state_stored_only_in`/`lend_map`/`safe-across`), keyword-`shift` is a modeled
`Stmt::Shift` (460 uses), and `command` must stay (the contract's own gate). Zero golden movement,
both legs (1602/1598). Residue is needs-human, in `26J`: the authoring-side lint stays unbuilt
(it needs a which-roles-to-scan ruling `__resolve` complicates), verdict-lane ⊤s reach no surface
at all (pre-existing, general), and book-side `set --` is an unmodeled sub-form gap.

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
- **`28J`'s "7 unwritten catalog codes, all loom-editable today" is WRONG** (+SURE, all 7
  spot-checked 2026-07-27: `marker-version-unrecognized` · `mark-unknown-verb` ·
  `mark-rc-arity-exceeded` · `mark-standalone-rc-consumer` · `mark-hashcolon-malformed` ·
  `host-evidence-admission-refused` · `whylog-unwritten`, all refuse `MarkerOutsideEditableSection`
  identically; `transport-session-lost`, same shape, matches too). Root cause: catalog
  `message: None` renders `[unwritten: <slug>]` as chrome (`push_arrangement_part` →
  `RenderComponent::Structure`, `aid/diag.rs:2786-2790`), never a catalog `EditableSection` —
  -GUESS `28J`'s author conflated this with the ARRANGEMENT registry's same-spelled but
  structurally opposite `Words::Unwritten` (genuinely editable, `28H` span ruling 4). Separate
  friction on the WRITTEN side: `unreflow`'s `join_continuations` (`dorc-loom.rs:796-811`)
  excludes only `-->` caret-frame lines from its title-join, not `= help:`/`= note:`, so revising
  an already-`Some` message with no caret frame before its help block also refuses — ~SUSPECT hits
  4 of the 5 case-owned written entries (all `whylog-*`; only `whylog-absent` directly
  reproduced). Only confirmed-working shape: written message WITH a caret frame separating it
  from help (`cmdsub-operand-top`, `compile`-clean, not promoted). W5-as-scoped needs a re-audit
  before more human time goes into "type over the placeholder" on the unwritten codes.
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

## R28 residuals - THE W4 ARC — PROSE FACES FOR THE WHY SURFACE (EXECUTED — closed 2026-07-26)

**BUILD COMPLETE on `ai/r28-unify` (awaiting the human's fold).** THE durable is
**`notes/28H`** (every ruling); the human's W5 worklist home is **`notes/28J`**;
the red-line distillation `notes/28I`. Five execute lanes + two probe/inventory
lanes, each conductor-verified at fold: **map** (counted: 0/111 why rows
editable, 80% of sentences transport-blocked → the `28G` §2 transport deferral
PULLED IN) · **carrier** (26 print sites → `Carrier`; `advisory` retreats to
three edge seats; libtest red-frames dead) · **parts** (`Said` hoisted,
`Said::Lens` dead, `Explanation`-as-parts, kTASTE `ChainModel` room, `--all`
honestly exhaustive, the webhost trust-spent bug fixed as a type) · **span**
(THE crux: one-section-many-fragments transport landed DORC-SIDE — errorloom
untouched, glued case green; occurrence threading; the weft→parts bridge with a
corpus-wide byte-identity proof; the 240-byte truncation dead) ·
**drifted-driver** (~285-line lib extraction; the drifted-receipt replay arm in
BOTH chains + a mechanical agreement guard; the FIRST TWO why-faced looms,
`why-drift-*.loom`; the edit loop PROVEN end-to-end; divergent-edit last-wins →
refuse). Also this seat: the hk `HK=0` master-killswitch found + fixed
(env-side; commit-msg gate live); the pre-commit loom glob hole closed
(`loom-hygiene` rename; three-way runner-floor triage); scoped bless works
(`mise run bless -- <substring>`). RULED: `AID-NEEDS:law-selection-is-goal-
derived` (+ the pull-register regloss + the spike-era tune-high posture) · the
one-section law (`aid/CLAUDE.md` a-chrome-line-is-one-section) ·
render-surface-instability as conduct law · the shared-lexical-rulebook ask
(discipline now, inventory then decide). **W5 authoring UNDERWAY** (human, loom
surface; start = the 7 `[unwritten:]` catalog looms, `28J`). **r30 charters
banked in `28H`**: the loom-UX lane (top item: the const-vs-mirror why-render
lag making why-row loom-edits two-step) · the full-driver extraction (chain-row
faces; `ask-full-driver-this-arc-or-r30` open with the human) · the
lexical-judgment inventory · levers/retention (unchanged from `28G` §2).

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
