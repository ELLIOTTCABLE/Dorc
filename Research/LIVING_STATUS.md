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

## R27 ONBOARDING (2026-07-10 — the single current view; supersedes the r24 onboarding, whose content now lives durably in `notes/24U` + `plans/270`)

**The state in one paragraph:** round 24 is CLOSED by reshuffle (`notes/24U` — the
abandonment accounting; read it before anything else here). The field trial (né r25) and
multi-host (né r26) are TABLED with revival conditions (`270` §5). The live arc is
**round 27, the consolidation round** (`plans/270`): block-settle (design-pass) →
block-rebuild (one conductor, the whole reimplementation block) → block-context
(wrapper/payload/read-value) → block-stdlib (stdlib-authoring + yardstick-measurement)
→ field-trial revival → multi-host resumption.

**⚠ IMMEDIATE next activity (human-acked 2026-07-10): the `270:block-settle`
design-pass**, human-in-the-loop — the entity-algebra design note (reserving the
`24S:A7` seams) + the adjudication agenda (`270` §3: adj-entity-algebra ·
adj-capture-claim · adj-trichotomy-spelling/adj-axis-vocabulary ·
adj-survival-flag-outcome · adj-small-homes · adj-stopping-point). NO builders dispatch
before block-settle closes: the corpus-respell deliberately WAITS on the entity-algebra
authored-spelling ack so the fixture sweep happens exactly once (`270` §2 records the
rationale — this supersedes the old "respell is dispatch-ready, fire on d8" posture;
d8 itself was typed-acked 2026-07-10, so the specimen gate is clear).

**⚠ NAMING DISCIPLINE (human-ruled 2026-07-10, HIGH priority, binds every brief):**
hyphenated full-word slugs; outside-document references as `docID:slug`
(round-ID dedupes into the prefix — `24C:rul-selector-pre-stdlib`); subscript old
labels once ("né P5"); progressive de-naming-debt, judgment-bound. Full text:
`270` §1. Propagate verbatim into subagent briefs.

**Read-first on arrival:** root docs AT HEAD → `spike/CLAUDE.md` (all rulings blocks) →
`notes/23O` (settled law + history) → **`notes/24U`** (the round-24 close-out + the
reshuffle map) → **`plans/270`** (the charter — blocks, adjudications, fences) →
`notes/24C` INCLUDING its full accreted tail (build evidence + residue) → `notes/24P`
(the respell spec-by-example; §8/§9 riders + bless flow) → per-task: `plans/24S`/`24T`
(the wrapper/payload keystones — proposal-tier, prime adversarial-analysis targets) ·
`notes/24M` (language rulings) · `plans/262` §2 (the wire contract block-rebuild
partially imports) · `notes/219` (the capture lane behind adj-capture-claim) ·
`plans/24R` (secondary positions; the why-run impossibility ledger).

**Where the build stands (tip of `ai/spike3-r23`, conductor-verified 2026-07-10):**
e2e **all 126 pass** — 121 live + 4 declared-XFAIL respell specimens (stale-old goldens
BY DESIGN, the staged failing spec; implementor flow in `24P` §9b) +
pipe-guard-oracle-converged live with its re-keyed golden. The `240` ladder Stages 1–5
LANDED + polish + pipe lift + wave-1 (evidence: `24C`). The respell specimens are
COMMITTED at tip; `dorc_flags_selftest` anchors survive-multiwall. Working tree carries
only the human's own TODO.md edit — leave it.

**Branch map:** `ai/spike3-r23` = the live lineage (r24 history + round-27 forward).
`ai/spike3-r25` = field-trial tooling (P1/P3/P6 + the salvaged observer harness;
dormant; owed-on-revival items banked in `24U` §6 / `270` §5; any Vultr work re-reads
`252` §5.1's guardrails first — tag-scoped destroy only). `ai/spike3-r26` = plans-only
(ZERO build commits, verified at close; resumes post-trial by rebase; its
merge-disjointness contract is dormant until then — round-27 golden churn needs no
r26 flagging, but the `262` §7 extractables ride block-rebuild).

**Conduct fences (standing; bind any successor):** never edit
README/DESIGN/IMPLEMENTATION/USER_STORY/TODO/AGENTS/root-CLAUDE (human-only) · KNOBS
carve-out: conductor MAY edit, leaves edits UNCOMMITTED for human ack · the §1 naming
discipline (`270` §1) · builder briefs touching tests/fixtures carry
rider-comment-budget (`24P` §8: rip-don't-update + the hard byte budget + counting
command) · HARD QUARANTINE on `quarantine-DO-NOT-READ/` + `Research/corpora/` · check
the tree before minting note/plan IDs · word-slugs in full words · silence ≠ ack (only
what the human TYPED counts; keep an ack-ledger) · crosscheck adjudication under
maximum skepticism; adversarial framing = exclusions-not-inclusions · never
AskUserQuestion (ask in prose); dump the numbered task list on changes · Fable
conducts, Opus codes; sonnet no-subagent clamp propagates one tier down ·
code-modifying agents → isolated worktrees: step-zero `git switch -C <worktree-branch>
ai/spike3-r23` + tip-hash verify + `pwd` verify, step-0.5 `mise trust`, step-ONE an
EXPLICIT root-docs read before task material · builders: granular `(AI …)` commits,
never push, four gates before every commit, final e2e FOREGROUND with generous timeout,
force `cargo build --workspace` before trusting e2e, BLESS exclusive + diff-inspected ·
conductor: tip-gate every ref-move IN THE COMMAND; verify merges by own hand
(never-vouch) · the deferred-work ledger lives in `23O` §5; residue lives in `24C`.

---

## Round 24 (2026-07-03 → 2026-07-10 — CLOSED; full accounting `notes/24U`)

The empirical build round that landed the elide-past-a-running-wall machine (ladder
Stages 1–5 + guard tier + derived footprints + aliasing closure, `24C`), was
intercepted by the language arc (`24M` rulings; respell staged as spec-by-example,
`24P`), and closed when the late design keystones (`24R`/`24S`/`24T` + the r26 wire
adjudication) reshaped the remaining work into `plans/270`. Everything not completed
moved — the reshuffle map is `24U` §4. Still-live one-liners from the 2026-07-07
settled-list not homed elsewhere: the dotfiles acceptance-day stays TABLED; the
first-wall hint's missing e2e needle stays ACCEPTED-as-residue (`24C`).

## Round 25 (field trial — TABLED; revival = `270` §5)

Protocol + guardrails + pre-registered questions stand unchanged (`252`; adjudication
`254`; validated book + dry-run `255`). Trial tooling lives on `ai/spike3-r25`.
Owed-on-revival banked in `24U` §6. The trial strengthens under round-27: its two
permanent walls (`su - postgres -c`, the `$(hostname)` host-guard) become in-scope
machinery (`270:block-context`).

## Ancient (pre-round-24)

See `Research/README.md`'s per-round map and `notes/23O` (round 23: the
oracle-contract crisis + settled law).
