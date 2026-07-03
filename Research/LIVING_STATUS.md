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

## NOW (2026-07-03, late — round-24 Stage 1 COMPLETE; Stage 2 in flight; the test-suite arc commissioned)

**Stage 1 fully built, merged, green** (`ai/spike3-r23`; e2e **135/135**, 9 standing guard23
xfails; family elide-fraction **0.32**, post-wall elisions = 0 — the charter's honest
baseline, mechanically true). The three sub-stages: 1a yardstick (CLI `dorc: plan-summary`
stderr line + `strawman24-*` family + `sh e2e/yardstick.sh`); 1b fd10/silence=wall (plan-time
wall walk in `build_plan_walled`; running modeled mutators wall downstream, elided cast no
shadow); 1c riders (strip-fidelity — fixed a REAL latent rc-clobber bug; kill-wall; the first
verdict-function-carrying floors; errexit-honesty row). Full evidence: **`notes/24A` §3**.

**The round's durable homes (rotation discipline — 24A is CLOSED as the early-rulings ledger;
new chunks get fresh notes):**
- **`notes/24A`** — Stage-1 rulings + evidence + the 231 disposal. The typed rulings:
  rul24-wall-placement · rul24-threefunc-monotonic (the `touches()` 3rd role-sibling;
  supersedes the two-contract sentence) · **rul24-mode-gate** (survival tier FLAG-GATED
  `--trust-footprints`, never default; marketing-at-best/theatre-at-worst) ·
  rul24-divergence-is-the-game (license-site≠elision-site + claim-subject≠blast-subject;
  attribution co-primary for Stage 2; MH2 version-gate = the missing tether, seeded
  not-this-spike) · **rul24-vouch-is-verdict-authoring** (§1c — the tilde is DEAD; authoring
  `is_converged()`/`is_diverged()` IS the vouch; provisos: read-erasure, family-open,
  marks-survive) · **rul24-overtype + the ARC-WIN** (the claim-tier trust algebra:
  `Claim<Tier,Payload>` Fact/Judgment/Silence; license-mints DEMAND tiers in signatures;
  births Stage 3, pays hardest Stage 5; "uncheckable invariants ride WITH the types").
- **`notes/24B`** — the TESTING ARCHITECTURE: three flavours (kernel-unit / real-path corpus
  / the new in-memory sweep); battle-oracle corpus = STANDALONE fixtures, not e2e case-dirs;
  the sweep is a NEW composition-root crate (hostsim stays the model). §5 reconciles against
  the round-12 DST conclusion (`plans/128`): the net is elision-soundness DST at the
  fact-verdict seam (NOT round-12 coordination-DST — that's round-25/22H); the determinism
  guard is mandatory; approach-#3/cross-platform-by-purity; coverage humility.
- **`notes/24C`** — STAGE LANDINGS + RESIDUE ledger (accretes per stage). Stage 2 (golden
  hill lit 0→1) + its residue (resid-aliasing = the Stage-5 under-execute cell that MUST be
  professed at the horizon; kill-coherence; argparse-drift contained). Stage 2b (the sweep
  has teeth — 3 planted bugs caught) + two findings: **find-lcg-thinning** (Host::seeded's
  low-bit coin correlates → existing DST tests under-explored; cheap fix owed, task #11) and
  **find-net-covers-what** (survival-tier coverage rests entirely on the lying-attribution
  net — honest survival is provably sound, so the lying scenarios are load-bearing).
- **`notes/24D`** — the STAGE-3 TYPE-ARCHITECTURE SPEC (conductor-authored, rul24-overtype):
  the claim-tier trust algebra `Claim<Tier,_>` (Fact/Judgment/Silence; 4 unrepresentability
  props: one-way demotion, mints-demand-tiers, no-judgment→fact-plane, the OPEN rung reserve)
  + the guard tier + the elide-weld + verdict-fn lift. **The round's most foundational
  type-decision — the reviewable one.**
- **`ORACLE_PROVIDES.md`** (NEW root doc, pending human audit) — the ledger of information
  shapes an oracle hands Dorc (provides-decoding/vocabulary/reading/binding/behavior/
  convergence/license/margins); the license-LADDER (rung-0 display / rung-1 guard / rung-2
  elide) is OPEN — the wary-engineer hatch, sibling of the admin's mode-gate flag.

**Landed since the Stage-1 refresh:** Stage 2 (frame-rule machine — merged, sound-reviewed,
type-contracts + docs pass) · Stage 2b (the chronology net / `dorc-sweep` crate — merged,
teeth-proven). **In flight:** **#5 Stage 3** (guard tier + the claim-tier algebra — the
foundational arc-win; spec in `24D`; build claim-tier types FIRST so they're reviewable).
**Pipeline:** Stage 3 → **#10 Stage 2c** (battle-oracle, now blocked on Stage 3 to exercise
guards + dodge a sweep-crate collision) ∥ **#6 Stage 4** (derived footprints) → **#7 Stage 5**
(grounding — where the claim-tier pays hardest) → **#8 Stage 6** (measure adequacy-bite /
conclude). Loose: **#11** (the Lcg fix, between stages, non-blocking).

**Conduct addenda this session:** worktree-isolated agents base on ancient `main` — brief
step-zero `git switch -C <branch> ai/spike3-r23` (`reset --hard` hook-blocked) + tip verify ·
Fable conducts / Opus codes (type-contract design is the ONE conductor-code carve-out;
mechanical verification rides Opus) · research grounds in-corpus first (the round-12 DST round
is authoritative + current; no fresh Kagi) · new test coverage is in-memory/Rust/
performant-by-design, NOT more e2e case-dirs.

## Earlier today (2026-07-03 — round 23 CLOSED; round 24 charter + Stage 1)

**Round 23 is closed.** Its complete, durable, single-narrative history is **`notes/23O`** (the
closeout — read it first). The crisis (233 = the frame problem, permanent) resolved to the
**ternary verdict {elide, guard, run}** with the **converged-vouch** license and **silence = wall**;
the interface was settled (**role-split** `predict`/`is_converged`/`is_diverged` + **rc-partition**
0/1/≥2 + strip-fidelity); the spike was **realigned** to the design (marker fiction retired, 123/9/0/0
green); the guard tier was **pinned** (24 `guard23-*` cases); and the elide-half design reached a
**permanent floor** (`23M`/`23N`): the mechanism is the **separation-logic frame rule through a
dynamic frame** = **lazy code motion for shell**, and its one live risk is the converged-vouch's
**adequacy** (converged≠no-op), calibrated-never-proven. All settled law lives in `23O` §2 +
`spike/CLAUDE.md`'s rulings blocks. Do not relitigate it.

**Round 24 = "head off 233 by building something and seeing what happens"** — the plan is
**`plans/240`**. The theory is exhausted at its floor; the work is now empirical. Build the
**elide machine** (the golden hill — the attention product Dorc exists for) FIRST, on hand-authored
**strawman** books, because in the spike the **DST exec-differential is the correctness net** (run
the elided plan under mocks, diff the bare book; a wrong elision goes red). The guard is the
*production* net — deferrable. The yardstick, CLI-runnable at every stage: **elision frequency on
a strawman family, differential-verified.**

The six-stage ladder (full detail in `plans/240`; golden hill lights up at Stage 2):
1. **yardstick + honest baseline** — build the elision-count-plus-differential mode; land the
   `fd10` fix so *silence = wall* actually holds (the "dangerous middle" is still live at HEAD).
   Baseline: post-wall elisions = 0.
2. **the frame-rule machine — first line vanishes past a wall.** authored footprint + backing +
   disjointness + `elide-when-disjoint-else-run` (no guard; differential is the net). Yardstick 0→N.
3. **the guard tier** (the 9 `guard23-*` xfails) — the production net; a side-quest off the golden
   hill's critical path, slotted here per the human's ordering.
4. **derived footprints** (`dpkg -L`) — elide past payload-bound tools.
5. **grounding + collaboration** — coordinate-kinds, bridges, the `scan_cve` story; synonym =
   dynamic-points-to-or-wall.
6. **maximize + measure + conclude** — the ~80% question on strawmen; extract conclusions to the
   human docs; then the spike can die.

**Build on Opus** (mechanical-ish; reserve cheap-Fable for breadth + the round-25 reactivity
design). The spike is freshly realigned — build now, before it drifts.

**Live task state (reconciled at round-23 close):**
- DONE: #7 pins · #14 spike reconciliation · #17 interface rulings · #18 rename · #19 golden-hill
  design (floor reached → `23M`/`23N`/`23O`) · #20 closeout · #21 `plans/240`.
- CARRIES INTO ROUND 24: **#15** — the repair pass; its `fd10`/silence=wall fix IS Stage 1, the
  strip-fidelity implementation (ruled: bare marks deleted whole) rides along, plus the small pins
  + the 231 disposal paragraph. **#16** — the human's root-doc queue (line-fixes + adopt the `23N`
  vocabulary + the "lazy code motion for shell" README line).
- TABLED: **#11** — the placement-spectrum / barrier round = the *performance* product; parked to
  round-25+ (by the consent-wall it offers the attention goal nothing; `23O` §4).

**Deferred-work ledger** (durable now in `23O` §5 — 22H reactivity is round-25 and wants Fable
ASAP; provenance-DAG is reorderable and may ride this spike; MH2 versioning; the language +
`unsafe` hatch; kSTATE; DX tooling; `.diff`; the deferred surfaces incl. lane-privilege).

**Conduct fences (standing; bind any successor):** word-slugs in full words, explain prior-art
inline (the human is often on mobile); silence ≠ ack (only what he TYPED counts); **HARD
QUARANTINE on corpus/H2SaLS** (the `quarantine-DO-NOT-READ/` dir + `Research/corpora/` stay
unread; strawman measurement only, never the corpus); crosscheck adjudication under maximum
skepticism (convergence = signal; a corpus doc's *existence* is never authority — reverse-
sycophancy is a live failure mode); adversarial framing = exclusions-not-inclusions; Fable
dispatch = ask-first, goals-not-instructions (Opus gets full enumeration); code-modifying agents
→ isolated worktrees with a baseline-check + explicit-pathspec commits; never edit
README/DESIGN/IMPLEMENTATION/TODO/AGENTS/root-CLAUDE (human-only); notes are append-only EXCEPT
this file; **never use the AskUserQuestion tool** (his vi-mode breaks it — ask in prose); dump
the full numbered TaskList when it changes or when he's remote; the method is now
**build → measure the yardstick → let the evidence pick the next stage.**

**On-ramp order for a fresh round-24 conductor:** root docs (`README`/`DESIGN`/`IMPLEMENTATION`) →
`spike/CLAUDE.md` → **`notes/23O`** (the closeout — everything that happened + the settled law) →
**`plans/240`** (the round-24 plan) → THIS FILE → then, as the build needs them: `23M`/`23N` (the
elide mechanism's landmines + vocabulary), `23A`+`23G` + `spike/e2e/run.sh` (the guard-tier spec),
`23H` (the spike's reconciliation record + the strip-fidelity residue for #15).

---

## Yesterday-scale (round 23, 2026-06-15 → 2026-07-03 — compressed; full record `notes/23O`)

Opened on `plans/230` (best-effort), intercepted by the human's `plans/233` crisis (the oracle
poison contract was broken — the frame problem). Resolved to the ternary verdict + converged-vouch
(crosschecked `236a/b/c` → `237`; ceiling `238`; signed closure `239` + the two-halves doctrine).
Interface settled (`23K`/`23L`: role-split, rc-partition, strip-fidelity) after the rc-soundness
cluster surfaced in the direction-crosscheck (`23I`/`23J`). Spike realigned to the design over five
sessions (`23E`/`23H`: marker fiction retired). Guard tier pinned + reviewed (`23A`–`23G`). Elide
half worked to its floor (`23M`/`23N`: 233 permanent; frame-rule/dynamic-frames/lazy-code-motion;
consent-wall; survival-settled-vs-adequacy-is-the-risk). Turned empirical → `plans/240`.

## Ancient (pre-round-23)

See `Research/README.md`'s per-round map (rounds 1–22).
