# 309 — Spine reification: the one output structure, its projections, and the 306 accounting

> Tier: LLM-authored plan (Fable conductor, from the 2026-08-15 human design sitting,
> session `r30-conductor-4`); DRAFT pre-ack as a document — the individual rulings
> inside carry their own grades and many are [TYPED]. Subordinate to root docs and
> `spike/CLAUDE.md`. Grades: **[TYPED]** human typed it · **[ACKED]** substance
> confirmed in dialogue · **[PROPOSED]** conductor-derived, awaiting ratification.
> Softness the human voiced is preserved inline ("tentative", "gut feel", "for now").
>
> Relationship to prior documents: `notes/306b` stays AUTHORITATIVE for influence
> vocabulary/laws; this plan is where `plans/306c`'s STOPPED remainder re-plans
> (§3a report-only output · the forgiving-parser re-home + fence · §4c) — 306c's §2
> and amended §3b landed via the r30 seams lane and are not re-planned here.
> `300:rul-whylog-is-the-spine` LIVES, re-read (§1). The `28Q` kernel arc is
> untouched by this plan except for the sequencing edge in §6.

## §0 — The design in one screen

**`Spine`** [TYPED naming, 2026-08-15]: the global in-memory structure everything
hangs off — every decision, its inputs (as capped accounts), its influence grade,
its narration. It is REIFIED from today's scatter (the classify tuple, ad-hoc cli
assembly, the decision digest, witnesses, the narrative arena). Every apparent
product is a **projection** of Spine × input-files: the executable artifact, the
plan render, the orchestrator's connections, and the `.whylog` durable itself.
The literal `.whylog` is ONE view — closely tracked, lossy by exclusion (§3) —
because some Spine content is in-memory-only and deliberately never durable.

The round-recast [TYPED]: *reify Spine; abstract `Plan` out of it and render Plan
from Spine as a lossy transformation; install the 306 accounting into Spine.*
The report-only output (`306b` §4b, FIRM) lands as a Spine state whose
authority-bearing projections are structurally ABSENT — a typed absence, not a
guard (`tc-report-substrate-is-the-plan` was an as-built accident: `WhyReport`
borrows a `Plan` only because nothing better was reified).

## §1 — The ruling re-read, and what Spine is for

**`rul-whylog-is-the-spine` LIVES** [TYPED]: "the whylog IS the authoritative
rendering of the Spine. We build FOR the whylog." It is a *mechanism* to keep
builders aid-first, explain-first, plan-later. Exceptions (Spine content withheld
from the whylog-rendering intent) are vanishingly few, each bought at great price
through opaque-review; they are not conductor/builder concerns.

**Spine×influence are one substrate** [ACKED; `307` §2b carries the argument]:
`306b` §2a's "complete in-memory structure — every decision, its inputs, its
grade, its narration" IS the Spine with the grade column drawn in. v0 influence
(the positional flip; the seams lane's `core::influence`) is spine-independent
and already in flight; every refinement past v0 — the contingency axis (a
differential over decisions), the §3d four-state replay discipline, the §6c
marking frontier, durable grades — is an operation over reified Spine records.

## §2 — The Spine type: position and laws

- **`law-spine-outside-the-kernel`** [TYPED, human-marked *tentative*]: Spine lies
  slightly outside the analysis kernel. One-way discipline: the engine WRITES
  events, facts, and finalized decisions onto Spine; **Spine never feeds back
  into the analysis loop** — a firewall between the analysis-loop and live
  dataflow structures, exactly the standing aid-plane pattern extended to
  finalized decisions of every type. Supports the existing decide-once-never-
  relitigate leans. **Watch-item `watch-firewall-is-default-not-dogma`** [TYPED
  worry]: the seal must not rot into dogma — if a case ever appears where
  re-reading a finalized decision is genuinely right, that is a deliberate,
  dearly-bought design act (opaque-review-tier), not a workaround; the watch-item
  exists so the pressure toward exclusion stays visible.
- **`law-spine-write-only-during-run`** [ACKED]: the purity half — Spine records
  (and references to them) never enter anything the solver compares; two
  semantically-equal analysis states must compare equal whatever their narration
  (the `CollapseNarrative` Eq-exclusion precedent, `22W` §2, generalized).
- **`law-spine-operands-capped`** [ACKED, "for now"]: the content half — records
  carry small, by-value, interned/scalar ACCOUNTS of operands, k-capped where
  unbounded (k exemplars + a count); never arena handles, never working lattice
  state (`operands-are-pure-and-capped` promoted from the durable-file rule to a
  construction law on the type).
- Grade-stamping [ACKED]: every Spine record stamps the v0 influence phase marker
  at mint (`authored-before-contact` pre-ingestion; `host-influenced` after).
  Calibration: this is v0 fidelity — full `306b` §1b generality (passes-ran,
  arrival timing as graded facts) rides the open gradation pin, not this plan.
- Crate home + arena/interning mechanics: [PROPOSED, undesigned] — decided
  against real code at the census stage; the `core::prov`/`DefinitionCustody`
  cross-crate patterns are the candidates.

## §3 — Projections, the durable exclusion census, and drop-accounting

- **`rul-durable-by-exclusion`** [TYPED]: the `.whylog` projection is defined by
  an EXCLUSION set, not an inclusion list. We are not choosing what new things to
  make durable; we invert the existing durable list into exclusions over a
  totalistically-tracking Spine. Later enrichment = LIFTING one exclusion
  ("this thing we already track gets written too"), each lift a deliberate act.
- **`mech-census-three-states`** [TYPED, the human's nit]: a no-wildcard census
  over Spine record species (the `CollapseKind` completeness-census pattern),
  three arms: **`durable`** (written to `.whylog`; ENTERING this arm is the
  tripwire firing — human and/or opaque-review, always) · **`excluded`** (ruled
  non-durable) · **`new`** (transitory: non-durable in production but not RULED
  non-durable — the legal resting state for in-flight work; dumps durably only in
  a project-internal debug mode that must be STRUCTURALLY unable to ship to user
  machines — gate per `rul-fixture-identity-never-production`'s discipline:
  absence of a constructor, never an env check [PROPOSED mechanics]). A new
  species fails the census until classified; silent inclusion and silent omission
  are both unrepresentable; `law-whylog-is-sensitive`'s secrets-round re-grade
  becomes a review of one list.
- **`rul-drop-accounting-completes-the-narrative-law`** [TYPED]: a projection is
  itself a narrowing function, so projection-drops become new COLLAPSE CLASSES
  minted through `collapse-mints-narrative`'s existing discipline and covered by
  its existing completeness census — "the durable is not permitted to be poor; it
  may be forced to be poor" (`306b` §2a) becomes mechanical: excluded/dropped
  species are countable at projection time, and the durable says what it chose
  not to keep. The standing consume-half gap
  (`289:seam-narrative-render-unconsumed`) begins discharging when the plan
  surface renders from Spine, rather than deepening.
- **Authority-exit enumeration** [ACKED]: `306b` §4b closes not one variant but a
  LIST — every projection carrying mutation authority (the plan render's apply
  artifact first; the orchestrator's connections; any future executor feed) gets
  the refused-target typed absence, and the census proves the list closed. The
  open `306b` §4c question (whole-target or narrower) becomes "which projections
  vanish under refusal" — deferred, both directions still live.
- Rehydration (`306b` §3a/§3b) stays UNBUILT: nothing new reaches disk this
  round, so nothing new rehydrates. Fences untouched: rec-5 (never a cache;
  re-ingestion aid-plane only) · two-plane law · `306b` §6b (influenced values
  never gate engine control flow; enforcement mechanism stays an open pin).

## §4 — Migration shape: big-bang [TYPED lean, "gut feel, not dictating"]

Census → transition → tests go RED → red stays red through the burn-down → GREEN
byte-exact. No transitional double-path states (the never-big-bang line in `300`
§5 was conductor-stated, not human law; overridden here [TYPED]). One builder,
one window; a second builder inherits the red state knowingly. What makes
big-bang safe HERE specifically: the end-gate needs no judgment — the
pre-refactor goldens ARE the spec (byte-identity of every render), plus the
standing checker gates (certifier · sparing re-derivation · both planes' votes)
over the green tip. Cost held consciously: during the red window the instruments
are blind, so everything rides the end-gates and the CENSUS's accuracy — which is
why the census is a checkpoint-tier deliverable (§5) and the window opens only on
a quiesced kernel (§6).

## §5 — Staging

1. **stage-spine-census** (checkpoint-tier; map-then-execute): enumerate every
   decision-site (the hidden-decision audit — decisions currently made at
   plan-build/render time hoist into Spine; `pinned-definitions-are-the-
   artifact's-binding` is the exemplar of a license-relevant "render-time"
   decision); enumerate every projection and its authority class; propose the
   Spine type + record species + census arms + crate home. Conductor+human ack
   before any conversion commit.
2. **stage-spine-transition** (the big-bang window): reify Spine; hoist the
   census's decision-sites; re-derive Plan (and the artifact) as projections;
   land the three-state durable census with today's `.whylog` contents exactly
   reproduced (exclusion-set = the complement of today's list — the durable's
   BYTES are inside the end-gate, so the tripwire is not fired by the
   transition itself). Gate: every golden byte-identical; both legs; checker
   gates green.
3. **stage-306-accounting**: grade-stamp records from the phase marker; the
   report-only Spine state + typed projection-absences for the enumerated
   authority exits; re-home the forgiving parser onto the report-only consumer +
   its governed lexical fence (the `306c` §3b steps 2–3 that stopped); the
   projection-tier collapse classes + drop-accounting.
4. Deferred beyond this plan: exclusion-lifts (each a tripwire event) ·
   rehydration + durable grades · the §6c marking discriminator / dualistic
   render · gradation past v0 · §6b enforcement mechanics.

## §6 — Sequencing (where this lands in the r30 arc)

After the stage-i definition-factoring fold + its adversarial crosscheck +
burndown (a quiesced kernel is the precondition for the red window); **before
world-scopes design settles** (stage-iii's availability computation is
influenced-by-construction AND mints new decision classes — building it pre-Spine
mints retrofit debt twice). Relative to closure-custody: plausibly parallel
(plan/cli/aid-side vs oracle/analysis-side) — ~SUSPECT; merge-risk decided at
dispatch. The snapshot-emission stage is unaffected and precedes this.

## §7 — Open pins (complete list)

1. `pin-spine-crate-home` — type home + arena/interning mechanics (§2; census
   stage decides against real code).
2. `pin-debug-dump-gating` — the `new`-arm debug dump's structural
   cannot-ship mechanism (§3; compile-gated per the fixture-identity discipline,
   exact shape undesigned).
3. `watch-firewall-is-default-not-dogma` — §2's watch-item; re-read at any
   proposal to feed a finalized decision back into analysis.
4. `pin-authority-exit-list` — the enumerated authority-bearing projections and
   the §4c which-projections-vanish question (§3).
5. `306b` §1c gradation axis · §6b enforcement · §6c marking discriminator —
   inherited open, unmoved.
6. `pin-census-new-arm-hygiene` — whether the `new` arm needs an accretion
   instrument (a census count surfaced like the prose burn-down) so transitory
   never becomes permanent-by-neglect [PROPOSED, cheap, undecided].
