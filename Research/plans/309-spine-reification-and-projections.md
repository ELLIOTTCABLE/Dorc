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
- **`law-spine-write-only-during-run`** [ACKED; reworded position-derived per
  `_tmp-309` lesser-2]: Spine is written post-decision, from OUTSIDE anything the
  solver compares — Spine values never enter compared state at all, so no
  Eq-exclusion is ever needed anywhere. The `CollapseNarrative` Eq-exclusion
  (`22W` §2) is cited as the failure-mode this positioning AVOIDS, never a
  technique to generalize: that precedent is safe only because narratives are
  decision-inert, and Eq-excluding a license-bearing record from machinery that
  guards it would be a hole.
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
- **`mech-census-three-states`** [TYPED, the human's nit; the `durable` arm's
  View mechanics are [CONDUCTOR — human-deferred 2026-08-15: "I trust your
  engineering judgement; proceed as planned", explicitly NOT an ack]]: a
  no-wildcard census over Spine record species (the `CollapseKind`
  completeness-census pattern), three arms: **`durable-via-View`** (written to
  `.whylog` exclusively through a per-species `DurableView` projection type
  whose fields ARE the durable subset; records themselves never implement
  serialization — so field-level exclusion is STRUCTURAL: the influence grade is
  excluded by not existing in any View, silent field-growth is unrepresentable,
  and lifting a field exclusion = adding one field to one View, a diff that IS
  the tripwire's mechanical form. Resolves `_tmp-309` critical-2: a
  species-arity census cannot express a durable species carrying an excluded
  field, which the grade makes the FIRST post-transition state, not an edge
  case. ENTERING the arm, or growing a View, is the tripwire firing — human
  and/or opaque-review, always) · **`excluded`** (ruled
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
pre-refactor goldens ARE the spec (byte-identity of every render AND of the
`.whylog` durable), plus the standing checker gates (certifier · sparing
re-derivation · both planes' votes) over the green tip. Cost held consciously:
during the red window the instruments are blind, so everything rides the
end-gates and the CENSUS's accuracy — which is why the census is a
checkpoint-tier deliverable (§5) and the window opens only on a quiesced kernel
(§6).

**The decision-state smoke-diff** (`_tmp-309` critical-1, folded; the diff's
STATUS is [TYPED, ruled 2026-08-15]): byte gates are known-vacuous at
byte-lossy seams on exactly this refactor class (the stage-0 retroactive-audit
precedent, `28Q` §8: outcomes hold while records silently change; here: the
leaf-granular digest collapses member sites, and a binding can move between
byte-identical definition bodies with different custody — attribution changes
no byte gate sees, with no bisection point under big-bang). Mechanism:
BEFORE the window, a deliberately hack-tier dump tool walks today's scattered
decision-state (site-keyed decisions · definition-identity + custody per
binding · witness sets · digest at SITE granularity, computed from the old code
precisely so the known keying change needs no whitelist), deterministic and
sorted, frozen ONCE at the base commit; after green, a Spine projection
reproduces the schema, and the DIFF goes to the conductor's fold sitting.
**It is a smoke-testing machine, NOT an acceptance gate** [TYPED]: a review
tool for the very final round; non-empty output is material for judgment by
eye, never an auto-fail — "trying to game it will lead to weird backflips; a
cheap tool but a blunt one." BUILD-TO-KILL [TYPED]: tool, baseline, and
projection-shim all die after the fold review. Two fences: it is never the
whylog (no durable-tripwire contact; migration scaffolding only, never
shipped) and never conflates with the census `new`-arm debug-dump (different
mechanism, different lifetime); its schema INFORMS the owed SiteId
decision-dump product feature and must never become it. Honest residual: the
dump covers only decision-state the old code makes explicit enough to walk —a
fully-implicit decision is invisible to the baseline too, which is why the
census's hidden-decision audit stays checkpoint-tier and the diff is never
sold as total.

## §5 — Staging

1. **stage-spine-census** (checkpoint-tier; map-then-execute): enumerate every
   decision-site (the hidden-decision audit — decisions currently made at
   plan-build/render time hoist into Spine; `pinned-definitions-are-the-
   artifact's-binding` is the exemplar of a license-relevant "render-time"
   decision); enumerate every projection and its authority class; propose the
   Spine type + record species + census arms + the per-species `DurableView`
   shapes + the smoke-diff dump schema + crate home. Conductor+human ack before
   any conversion commit. The dump TOOL lands at the end of this stage, before
   the window opens (§4).
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
mints retrofit debt twice). Relative to closure-custody: BUILD phases SERIAL —
custody builds never overlap the red window (`_tmp-309` lesser-1, folded: a
parallel lane cannot distinguish its own breakage from the window's, and both
share `target/`); custody SITTINGS/rulings (design work, no code) free-run
throughout. The snapshot-emission stage is unaffected and precedes this.

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

## §8 — Review disposition

Reviewed 2026-08-15 by an opaque Opus-class reviewer working from the 306-series
sitting (`_tmp-309-review-notes.md`, project root, human-relayed; the file is the
human's to delete). Two critical + two lesser findings, ALL FOLDED above:
critical-1 → §4's smoke-diff (with the conductor's correction that stage-2's
durable-byte gate already stood, narrowing the exposure to byte-lossy seams) ·
critical-2 → §3's `DurableView` structural resolution · lesser-1 → §6 serial ·
lesser-2 → §2's position-derived rewording. Conductor-adjudicated under maximum
skepticism; every credited claim verified against in-context project text
(`28Q` §8's stage-0 audit language confirms the vacuity precedent independently
of the reviewer's withheld context).
