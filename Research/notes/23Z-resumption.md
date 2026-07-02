# 23Z — round-23 resumption (LIVING doc — state, bindings, the arc; nothing else)

> Living doc: update judiciously — direction-changes, discoveries, refutations, deferments;
> never per-turn chatter. The bar: nothing lost to a context-collapse. Restructured 2026-07-02
> per the human: this file is value-heavy and temporally-bound; the detailed session rulings +
> design seeds were harvested to **`notes/23D`** (the ledger), the chronology lives in the
> numbered notes. AI-authored; trust root docs + stamped `plans/233` + signed `notes/239` over
> this where they conflict.

## State (as of 2026-07-02, end of the crisis arc)

The 233 design-crisis is **formally closed**: the verdict architecture is ternary —
**{elide, guard, run}** — where a `guard` inserts the oracle's own stripped check in front of
the original bytes (`check || command`), silence licenses *nothing* (it only fails to upgrade
guard→elide), and the frontloaded trade stands (attention surrendered where the world is
undescribed; guards rescue perf/safety/monotonicity at new cost). The rulings are welded into
**`spike/CLAUDE.md`'s round-23 standing-rulings block** (rul-ternary-verdict · rul-guard-license
· rul-attention-honesty · rul-divergence-proceed) plus the TOCTOU identified-cause clarifier and
the inv-probe-sourced-values guard carve-out; KNOBS carries the kELISION naming-caution and the
kSILO round-23 shove note. Everything is committed on `ai/spike3-r23`.

Guard-tier behaviour is **pinned before build**: 19 e2e cases (`spike/e2e/cases/guard23-*`, 6
xfail + 13 floors; register `notes/23A`), conductor-verified green (118 pass / 6 xfail / 0 red).
A neutral+adversarial crosscheck **pair over the pin-set is IN FLIGHT** (isolated worktrees;
notes-slugs 23B/23C; asymmetric context — the adversary is barred from 23A/23Z). The human will
**clear context and stand up a fresh conductor** once the pair is adjudicated and repairs land.

## The bindings (hold these; full text at the cited homes)

- **The four round-23 rulings** — `spike/CLAUDE.md` (the signed design; guard sourcing
  principle: the check IS the oracle, shipped strip-only, whole-body default, lifted forms
  byte-identical substrings; two nevers).
- **Two-halves doctrine + anti-creep** — `notes/239` §1: full elision is THE GOAL (never
  aspirational-tier language anywhere); the guard-half is sister + permanent fallback, equal
  attention; no guard-half decision quietly discards an elision-goal constraint.
- **Oracle ground-truth** — `23D` §1 (strip-only; arbitrary sh; analyzer-trick-not-language).
- **Plan-surface + attention-chronology + atomic-command axiom + can't-serve rulings** —
  `23D` §2–§4 (plan-is-the-code render; no late attention-demands ever; no command disassembly).
- **Interim rc-consumer posture** (deferred experiment; runs during the build phase): guards
  mint only where NO explicit status reader exists; errexit-implicit is OPEN, unpinned both
  ways — the human suspects painful breakage under either default (`23D` §3).
- **The vocabulary law + elide-half seeds** — `23D` §5 (positive rides open vocabulary;
  negative needs ownership/consent; grounding-bridges; entity-aliasing fence) — ~SUSPECT tier,
  adversarial pass OWED before any of it welds.
- **Statuses:** `plans/233` is STAMPED (frozen; end-annotation is the correction channel).
  KNOBS was conductor-editable-with-human-review THIS session only — a fresh conductor
  re-confirms before touching it. DESIGN/IMPLEMENTATION edits remain human-only (his rewrite
  pending, low-spoons — do not nag; `239` delta-5 deferred).

## The arc ahead (ordered; who/when/why)

0. **IMMEDIATE (this conductor, pre-clear):** DONE — pair adjudicated (**`notes/23F`**: both
   passes say build-against-the-set; the demonstrated adversarial composition holes adv-1
   variable-clobbering / adv-2 nounset; the convergent build-window blindness conv-1; the
   h1–h4 asks) and notes harvested (23B/23C committed). FULLY DONE 2026-07-02: rulings h1–h5
   landed (23F addendum), the repair pass executed (Opus agent; 9 commits cherry-picked;
   register **`notes/23G`**), conductor-verified: **123 e2e round-trips / 9 xfail / 0 XPASS /
   0 red**. The pin-set now includes the composition pins (variable-capture, set-u), the
   scoping floors (cross-oracle vouch, cant-tell), the redirect refuse-home, two-sided xfail
   markers, tripwire shims, the guard-shape floor + selftest confounds, and the
   diff-before-bless rule. ONE design flag for the build round (23G `jc-nounset-desired`): the
   set-u end-state is mechanism-SENSITIVE — subshell-wrap survives-but-defeats-the-guard vs
   `local`/`${n:-}` hygiene survives-and-works; bare-survival is pinned for now; rule which
   end-state is desired when the mechanism lands. Also 23G `jc-rust-diff`: the hostsim
   differential judge needs the same guard-disposition widening as gate-6 when the tier
   builds (deferred, correctly). Task #7 CLOSED; the round-23 conductor handoff is complete.
0.5. **SPIKE-vs-DESIGN RECONCILIATION — PARTIALLY LANDED 2026-07-02** (Opus agent, isolated
   worktree; its 3 commits cherry-picked onto main, conflict-free; merged tree verified by the
   conductor: all unit tests + 118 e2e / 6 designed xfails / 0 red). **DONE: R1** — strip-only
   parsing of the full inline dialect (period-named funcdefs; ESTABLISH/`!`/`= value`, OBSERVE
   `:?`, ACK `~`, POISON, the `: provider:verb~` vouch placeholder; a byte-stable strip pass;
   16 new tests; additive — old spellings still parse, nothing regressed, no goldens touched)
   + the st-2 correction notes in spike/CLAUDE.md and oracle/CLAUDE.md (originals kept as
   history). **DESIGNED-AND-DEFERRED: R2/R3/R4** (marker retirement, emitter ships the
   stripped check body, fixture conversion) — BLOCKED on a crisp human ruling (**ask h5**):
   the changes propagate into `crates/coverage/`, which is H2SaLS-entangled; the quarantine's
   LETTER excludes only its two H2SaLS-referencing files, its SPIRIT arguably the crate —
   option (A) coverage/lib.rs+main.rs call-sites are fair game (mechanical, unblocks
   everything) vs (B) freeze the crate (human edits the call-sites or sanctions a dual-mode
   transition). Full design + blocker + the design-shaped `jc-*` flags (dpkg-i effect
   derivation; the multi-selector resolve_probe floor evaporating under check-as-oracle;
   `!`-polarity vs rc-inversion semantics): **`notes/23E`**. NB a tooling quirk for future
   dispatches: the isolation worktree branched from the DEFAULT branch, not this ai-branch's
   HEAD — the agent noticed and self-corrected (23E §0); check new agent-worktree baselines.
   Original content of this step: correct the spike where it is *genuinely wrong* against the design, so the guard
   work lands on corrected substrate. Known items: the **fake probe/check division** (the st-2
   `oracle_probe_*`-vs-`check()` split is spike-internal fiction — design truth per `23D` §1:
   the check IS the oracle, strip-only, whole stripped body ships in both lanes; rewrite the
   st-2 ruling text in spike/CLAUDE.md to match, rework the emitters/fixtures that assume the
   split); and the **drifted strawman spellings** (`oracle_effect` marker rows and kin have
   wandered far from the design conversation's direction — re-ground fixtures against 233's
   inline vocabulary while keeping every spelling byte-cheap to swap and loudly marked
   strawman). Sweep for further genuine-incorrectness while in there; flag borderline cases up
   rather than deciding.
1. **THE BUILD SLICE — the next conductor's spine:** make the 6 xfails flip green, honestly.
   Sequence per `23A` §5: widen gate-6 (the dual-rail judge has no license class for apply-only
   check commands / guard-suppressed mutators) FIRST; then the guard emitter per the round-21
   door-4 mechanics (`notes/218a`: `||`-form errexit-exemption, preamble functions, call-site
   silencing; collision-refusal falls out of strip-only sourcing); the GuardLicense witness
   (mint only from call-site × reached-vouch × probe-verdict — mind **hz-refusepath**:
   corpus-standard check bodies exit 0 on refuse paths, so the reached-path component is
   load-bearing against vacuously-passing guards); the strawman vouch lift. Build-phase notes
   (no longer tasks): run the rc-consumer experiment here (both postures against realistic
   set-e books — the deferred split, interim posture in the bindings above); settle guard
   render-forms as they materialize (the one-body-two-lanes candidate is nearly automatic under
   whole-body shipping); update ANALYZER-NEEDS rows for the guard tier as they appear.
2. **The placement-spectrum design round (task #11, COMMISSIONED):** the re-verification
   placement dial (per-site guard ↔ hoisted post-wall wave). Wall-density cost-model FIRST
   (density is endogenous — falls as core-oracle coverage grows); single-approval,
   no-late-attention, no mid-apply re-planning; quiescence-window named; adversarial-crosscheck
   before ANY re-weld. The round CONCLUDES with the wave-related re-welds (all that remained of
   the old re-weld task) and the wave half of the pin plan.
3. **The elide-half design round (equal standing per two-halves; not before the human wants
   it):** seeds = `238` (horizon, derivation gradient) + `23D` §5 (footprints, demand-
   disjointness, namespace convention, grounding bridges, entity-aliasing) + the refiled 234
   items (demand-anchored poisoning; provider-equivalence; traced footprints). EVERYTHING here
   is ~SUSPECT-tier until its own adversarial pass runs.
4. **Parked, human-keyed (NOTES, not tasks — the human de-tasked these 2026-07-02):** the
   escape-hatches (`notes/235`; un-park signals: admin-recourse pressure or bump-mode work; the
   236 convergence supports it); check-cost banding (needs a sanctioned data source; corpus is
   QUARANTINED); the vouch-spelling family (the dq-kOOB/kTYANNOT cluster — gates replacing the
   strawman with real spelling); the 22H live-plan arc (r22-deferred, foundational; its
   Research/README entry carries the guards-pin-book-order composition note).
5. **Gentle notes for the next conductor (not tasks):** a `[REVISED]`-style annotation on
   `notes/093` recording that round-9's closed-world frame axiom was consciously revoked for
   the elide tier (236a's archaeology; 237's erratum list); slow walk-throughs (self-reach
   fixed-point, the prefix rule, door-4 mechanics) are explain-on-demand duty, not work items.

## Conduct fences (binding on any successor conductor)

- Word-slugs only (`convergence-3`, never `C3`); explain prior-art INLINE in conversation — the
  human is often on his phone and cannot open files; unroll, then cite parenthetically.
- Silence ≠ ack: only what the human has TYPED counts as agreed; keep an ack-ledger; restate
  critical claims for cheap explicit acks.
- **HARD QUARANTINE: corpus / H2SaLS topics** — never route sizing or measurement through
  them; hard-defer; do not ask why.
- Crosscheck adjudication: maximum skepticism; kill-shots and new-directions are rare;
  adversarial-only findings suspect-until-checked; never credulous toward hostility.
- Adversarial framing: exclusions-not-inclusions; strip authors' self-flagged worry-lists from
  what hostile agents may read. Fable-tier: ask-first (ru-24); the human dictates or reviews
  prompts (Fable wants goals/desires/position, NOT instruction-lists — his repeated correction).
- Firewall-breach discipline (a noted Fable tendency): when a proposal dissolves a welded
  separation, lead with the breach, price containment first, offer the non-breaching cousin.
- Strawman sh liberally in conversation; never save strawmen into durable docs as design.
- Never edit README/DESIGN/IMPLEMENTATION/TODO/AGENTS/root-CLAUDE. Notes are append-only —
  new-numbered-note per chunk — EXCEPT this file. Echo the TaskList to the human each round.
- The method: xfail-first → design → adversarial-crosscheck → build; behaviour pins before
  types; tc-shaped judgment calls flag UP, never settle silently.

## Pointers (the on-ramp for a fresh conductor, in order)

Root docs (README → DESIGN → IMPLEMENTATION; AGENTS for process) → `spike/CLAUDE.md` (invariants
+ the round-23 rulings block) → **`plans/233`** whole incl. end-annotation → **`notes/239`** →
this file → `23D` (rulings/seeds ledger) → `23A` (pin register) + `spike/e2e/run.sh` → `237`
(crosscheck adjudication, incl. its four corrections) → `238` (the ceiling) → `235` (hatches) →
Research/README (the corpus map). Build reality: `plans/16P` §3 + `16Q` + closes `20K`/`21W`/
`22W` + the `guard23-*` cases.
