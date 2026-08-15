# 300 — Round-30 conduct ledger: the 28Q-arc implementation

> Tier: LLM-authored conductor working-ledger (Fable; minted 2026-08-14 at round-30 open,
> human-directed). AUTHORITY ORDER: root human docs > `plans/28Q` (THE kernel plan) +
> `notes/301` (THE minispec/dorc-verify spec) + `spike/CLAUDE.md` > this ledger. This file
> never duplicates those — it carries arc STATE: staffing, dispatch, ack-grades, gates,
> the census bank, and the conductor-handoff protocol. Grades: [TYPED] the human typed
> it · [ACKED] substance confirmed in dialogue · [CONDUCTOR] conductor adjudication,
> unratified unless a human reaction is recorded. Maintenance: compression-resistant;
> folded lanes collapse to a line; newest state at the top of §2.

## §1 — Arc shape and the handoff protocol

- Round 30 splits in two: the FIRST HALF is the correctness-tooling standup (this
  file's §2 + `notes/301`; it reshapes and supersedes the execution-plan half of
  `notes/28T`, which stays the evidence digest, marker-annotated); the SECOND HALF is
  `28Q` stages i–iii (stage-0 landed pre-arc). Ordering forced by `28Q` §8: every
  stage inherits the checker gates (certifier + sparing re-derivation green, both
  planes voting), so the checkers must exist first.
- [TYPED 2026-08-14] Conductor-context management: the arc expects sequential
  conductors and/or rewinds. THE NAMED STOPPING POINT is **wave-one-close** (§4). The
  human's rewind anchor is the 2026-08-14 plan-ack sitting (pre-dispatch, this ledger
  committed, zero subagents in flight). A post-rewind or successor conductor MUST
  distrust conversation memory of anything after that sitting — ground truth is this
  ledger + `notes/301` + `LIVING_STATUS.md` + `git log` (conductor branch:
  `ai/r30-conduct`, worktree `.claude/worktrees/r30-conduct`).
- [TYPED] Round-30 numbering: notes/300 = this ledger; notes/301 = the minispec/verify
  spec; `plans/302` is RESERVED for the solve-certifier mechanical spec
  (conductor-authored, pre-build). Never mint a 29x ID (quarantined round).

## §2 — The Wave-1 stage (the correctness-tooling standup): lanes, staffing, gates

[ACKED, as reshaped through the 2026-08-14 design sittings] Opus builders in isolated
worktrees; every brief carries the `spike/CLAUDE.md` safety block verbatim, step-zero
(reset to the conductor-stated `ai/main` tip + hash verify), step-one root-doc reads,
the no-subagent clamp, naming discipline (`270` §1), the `verified-core-discipline`
skill pointer, and flag-don't-resolve on every judgment call. PLUS the quarantined
builder-prerequisite read, per the conductor skill's quarantine section (durable,
human-committed 2026-08-14): Opus/Sonnet builders and foreign-lineage reviewers,
before any other work; never Fable-class subagents; the conductor never reads it. No lane carries
pilot/measure/kill staging [TYPED — velocity; the human inserts kills if needed].
Sequencing: facade solo FIRST → {derived-defs pipeline + minispec/verify standup} and
{kani, certifier} in parallel → rederivation integration → discipline-close.

- **lane-facade-std-dropping** — **FOLDED 2026-08-14 @ `601364f7`** (four commits;
  conductor-verified both legs green, zero golden drift, zero new deps, no split).
  What exists now: `core::sorted::{SortedSet, SortedMap}` (private-backed, one
  private `position`→`Slot` scan each; total, panic-free, index-walk bodies);
  `Powerset`/`MapL` re-seated on them (`Powerset`'s backing SEALED — the pub field
  died as the Aeneas-prep/refinement-enabling structural change); `Dialect`/
  `selector_covers` moved onto them; `solve.rs` production code untouched (its
  BTreeSets were test-only — census delta); VecDeque stays per lean-vecdeque-stays.
  §2a below banks the seat list + findings the later lanes consume.
- **lane-derived-definitions-pipeline** (after facade) — stand up Aeneas-translate +
  lake-build of the facade'd algebra as a MAINTAINED opt-in lane feeding
  `minispec/Generated/` (WSL leg; pinned per the nested-mise pattern in-repo,
  ratification below). Simply built, per no-pilots: regeneration breakage is the
  drift-alarm working, and translation/proof churn data falls out incidentally,
  gating nothing.
- **lane-minispec-verify-standup** — the `notes/301` build: the `minispec/` skeleton +
  the 2–3 remit claims + the dorc-verify binder v0 + the first bound demonstration
  loom + `minispec/CLAUDE.md`. Staffing split per `301` §0's access laws: builders
  build ALL harness/tooling; the spec content (units, claims, prose) is
  frontier-authored under explicit human authorization — in practice the conductor,
  with the human's ack, at dispatch.
- **lane-kani-harnesses** (after facade) — opt-in mise lane, real-tools-lane shape,
  expected Linux/WSL-only; harness home is `spike/verify/` (`301` §3). Targets: the
  lattice laws per combinator · `MapL` canonical-form · backing-set universal meet +
  non-empty-by-construction + ⊤-never-∅ · ternary consumer-map exhaustiveness ·
  span-edit non-overlap · the facade sortedness/canonicality seats. Tier placement per
  the `verified-core-discipline` ladder (the narrative-fold permutation pins may land
  property/DST-tier — placement flagged, never silently decided). Hand-written
  `#[cfg(kani)] Arbitrary`; checked code stays stable-toolchain, zero annotations.
  Conductor reviews the harness STATEMENTS (law, bounds, what is NOT pinned) at fold.
- **lane-solve-certifier** (after facade AND `plans/302`) — Opus implements against
  the conductor-authored mechanical spec; the spec carries the five crosscheck brief
  obligations verbatim + `28R:fnd-pessimistic-pass-shape` + the fresh-ack pedigree
  note. Shape: `Certified | Refused(EdgeWitness)`; `Refused` ⇒ degrade to the
  ⊤/stage-0 floor (license plane) + an operand-carrying narrative record (aid plane;
  `collapse-mints-narrative`); cap-trip certifies the PARTIAL solution; `Must<L>`
  duality, one checker; ships in the DEFAULT suite. Post-land: a cross-lineage review
  pass (codex-reviewer; cheap).
- **lane-sparing-rederivation** — (a) the naive reference model of the
  sparing/composition algebra, authored FROM the ratified English law-set under
  structural-simplicity constraints: the checker's value is STRUCTURAL difference —
  written under different constraints, from the machinery-free description of the
  goal, one pass, no worklist — never authorial lineage [TYPED — the
  independent-voices framing was deweighted; a foreign-model author (codex, ACKED
  available) is incidental, not load-bearing]. Zero shared code with `coord.rs`;
  statement-vs-spec disagreements FLAGGED, never resolved. (b) Opus integrates:
  DST-permutation internal differential + plan-time re-derivation of every survival
  verdict before a plan ships; disagreement ⇒ demote to guard/run + narrative record;
  the demote-only structure recorded explicitly (the `271:rul-net-quality-u-curve`
  pass condition).
- **lane-flux-engine-hardening** — [TYPED 2026-08-14: DEFERRED, penciled] not in
  Wave-1 (scoping it in would bloat; enough is on the table). Penciled MID-r30: after
  everything Lean-related is stood up (wave-one-close), before the proper kernel
  rewrite (28Q stage-i). Intent stands: another defense-in-depth instrument —
  ENGINE-tier (intake byte-budget, span/interval arithmetic; the churny tier no other
  instrument reaches at compile time), explicitly NOT part of the verification core
  (Kani+Lean+binder own the algebra; triple-covering rejected); nightly pin nested;
  meta-process learnings a deliverable. EXCEPTION [TYPED]: any typesystem or
  architecture change REQUIRED for Flux to be possible at all belongs in the
  Aeneas-prep work — the facade lane's scope, not the deferred lane's.
- **lane-discipline-close** (conductor) — the verified-core CLAUDE.md sections for
  `core`/`analysis` (incl. the `inv-determinism` sharpening: facade sortedness =
  named-seat + Kani-pin, the honour-system move stated in law text) · FORFEITS rows if
  any arise · prompt-review pass on all CLAUDE.md edits · ledger/LIVING_STATUS
  currency · the wave-one-close gate run (§4).

### §2a — Facade-fold bank (consumed by lane-kani, the derived-defs lane, and Flux)

Invariant seats (seat · invariant · pinning test; all tests in the default suite):
- `core::sorted::SortedSet::insert` — strictly-ascending, duplicate-free backing —
  `set_insert_sorts_and_dedups`; and structural `PartialEq` == semantic set equality
  (what `solve`'s `joined != state[w]` fixpoint test rests on) —
  `set_structural_eq_is_semantic_eq`.
- `core::sorted::SortedSet::position` — membership agrees with backing at every
  boundary — `set_contains_and_remove_agree_with_membership`.
- `SortedSet::union`/`intersection` — canonical results; ∪/∩ commute; ∅ identity/
  absorbing — `set_union_and_intersection_stay_canonical`.
- `core::sorted::SortedMap::insert` — ascending unique keys; rebind replaces+returns —
  `map_insert_sorts_keys_and_replaces_values`; structural==semantic Eq —
  `map_structural_eq_is_semantic_eq`.
- `SortedMap::remove`/`get_at` — order survives removal; `get_at` walks key order —
  `map_remove_and_get_at_keep_key_order`.
- `analysis::lattice::MapL::insert` (pre-existing) — no key maps to `V::bottom()` —
  `maplattice_is_pointwise_and_canonical`; plus insertion-order-independence of
  `Powerset`/`MapL` equality — `collection_domains_are_insertion_order_independent`.

Kani-lane guidance (builder-supplied, conductor-endorsed): the canonical predicate is
strict ascent — `∀i: get_at(i) < get_at(i+1)` (sortedness+dedup in one; maps over
`get_at(i).0`); NO `pub is_canonical()` was added (harnesses express it; add in the
Kani lane only if needed). `#[cfg(kani)] Arbitrary` homes in `core::sorted` (reaches
the private field, no widening) and must construct via arbitrary `Vec` +
`kani::assume(canonical)` — building via repeated `insert` would make the `insert`
harnesses circular. The asymmetric risk the harnesses exist to close: a bug making two
semantically-DIFFERENT values compare equal stops the solver's climb early
(under-approximated may-set ⇒ potential wrong elision, invisible to goldens); the
opposite bug only trips `converged: false`. Until Kani lands, the seat tests are the
whole net.

Findings + conductor adjudications:
- `fnd-reach-lattice-outside-scope` — `analysis::effect::Reach` (a `Lattice` impl in
  engine-tier `effect.rs`) still holds a raw `BTreeSet<FactKey>` + a hand-written
  cause-excluding `Eq`; the algebra tier is NOT BTree-free. ADJUDICATED: eviction
  deferred (not this wave; careful territory — the cause-excluding Eq is
  correctness-critical); the derived-defs lane EXCLUDES `Reach` from translation scope
  at v0 and says so in its config; revisit when a Lean statement first needs
  reaching-defs.
- `fnd-generic-ord-blocks-refinement` — facades stay generic over `T: Ord`; Flux needs
  concrete decidable orders, so the Flux lane (mid-r30) prices harness-side
  monomorphic instantiations (`SortedSet<SelectorId>` etc.), never product-code
  monomorphization.
- `fnd-iterator-exits-may-not-translate` — `iter()`/`IntoIterator`/`FromIterator` are
  grouped+commented as the translation boundary; the algebra proper avoids them. If
  the Aeneas pipeline chokes on the ALGEBRA (not the exits), the `while let
  Some(x) = v.get(i)` shape is unusable and the facade needs re-shaping — report,
  don't patch.
- `dec-shared-facade-home-in-core` — RATIFIED: one shared facade in `core` (both
  crates consume; core stays dependency-free) is the justified dislocation from
  `301` §3's crate-local default; dividend: one Kani harness set covers both crates.

[CONDUCTOR ratification, 2026-08-14] Nested-vs-root mise configs: toolchain-SHADOWING
pins live in nested configs (the in-repo Aeneas precedent); additive-only pins (elan)
may live at root.

[CONDUCTOR staffing, standing] Fable authors SPECS and reviews STATEMENTS
(`plans/302`; minispec content under the `301` access laws; harness-statement review
at fold); Opus authors bodies, tests, and toolchain wiring. Neither Kani nor the
certifier runs full-Fable or in-conductor-implementation.

## §3 — The Lean-tier vehicle (RESOLVED in substance, 2026-08-14)

- [TYPED] Aeneas is a must/of-course, if the tier exists at all: machine-correlation
  is the entire point where correlation is available, and the seam's brittleness under
  regeneration is the drift-alarm working, not a cost to engineer away.
- The maintained artifact is **minispec** (`notes/301`): hand-written statements +
  instances over Aeneas-DERIVED definitions, proofs where cheap. The earlier
  hand-model and Aeneas research spikes are QUARRY, never seed [TYPED].
- The recorded translation limits (sealed tiers, phantom `Must`/`May`,
  smart-constructor privacy do not cross) are compile-time discipline that rustc keeps
  enforcing over the real code — they never needed to cross; the Lean tier's job is
  equational law over the bodies, which translate faithfully.
- The churn-measurement question dissolved with no-pilots: the derived-defs pipeline
  is simply maintained; there is no vehicle decision left to gate on it.

## §4 — wave-one-close (the handoff gate)

All lanes folded to `ai/main` · `mise run both gate:full-quiet` green + `bless:dry`
clean · certifier + re-derivation live in the DEFAULT suite · the Kani lane opt-in and
documented · minispec standing (skeleton; the remit claims at their earned badge-sets;
binder v0 + the generated report; the first bound demonstration; `minispec/CLAUDE.md`)
· the derived-defs lane green · CLAUDE.md discipline sections landed + prompt-reviewed
· ledgers current (this file, `notes/301` if amended, LIVING_STATUS, FORFEITS) ·
conductor worktree/branch cleaned or handed over deliberately. Successor boot order:
LIVING_STATUS → this file → `notes/301` → `plans/28Q` → `spike/CLAUDE.md`. The NEXT
stage's first acts: a full root `ANALYZER-NEEDS.md` read (it owes the `an-flat-domain`
reconciliation paragraph, `28Q` §7), then 28Q stage-i's fixtures-first commissioning
per `28Q` §8.

## §5 — Ack-ledger (what the human has TYPED this arc; silence is never ack)

- 2026-08-14, the plan-ack sitting: the six-lane Wave-1 plan ACKED (with the stated
  leans: rederivation-in-scope · vecdeque-stays · needs-ledgers-deferred) · codex
  dispatch ACKED ("use as you see fit") · the research branches deleted by the human ·
  round-30 minted, notes/300 assigned · the sequential-conductor/rewind protocol
  directed (§1).
- 2026-08-14, the design sittings (the reshape this §2 reflects): Aeneas
  must/of-course · the small reviewable surface is a core product (literate
  colocation; the rationale is LLM attention-forcing, per the errorloom precedent) ·
  model-writing is design-work — the spike models are quarry; minispec's remit is the
  2–3-claim minimum; enrichment is a standalone human-led item · the runtime checkers
  and formalization-as-question-generator hard-ACKED · no pilots / no measure-kill
  stages, velocity · the independent-voices/lineage framing deweighted — a checker's
  value is structural asymmetry (finder/checker under different constraints) ·
  out-of-scope is a human judgment, never machinery (taxonomy/strength-axis repairs
  nacked) · mutation-testing is a gentle-must (badge defined day-one, `301` §5);
  property-testing stays the general check-ladder, never a spec badge · an automated
  performance-regression lane (CI graphs + hard gates) is banked for someday, out of
  scope · the whylog decision record is the assertion substrate, under the [TYPED]
  framing that huge amounts of Dorc are modelable as a deterministic mapping from
  source through probe-results to whylog result · doc routing: notes/301 minted as THE
  minispec/verify spec; this file carries the rest; `plans/28Q` edits minimal; `28T`
  markers-only; `plans/302` = the certifier spec (renumbered under the routing).
  The 301-interior rulings (access laws, remit, badges, bindings, naming, byte
  tripwire, local-homing default) live in `301` and are not duplicated here.
- 2026-08-14, session close: Flux DEFERRED [TYPED] — penciled mid-r30 (post-Lean
  standup, pre-stage-i), defense-in-depth intent standing, with the
  required-changes-ride-Aeneas-prep exception (§2's facade rider).
- 2026-08-14, the greenlight sitting (post-rewind): certification machinery ruled
  sketch-until-demanded (`301:post-certification-sketch-until-demanded` — architecture
  + cheap tooling + named seams; upfront depth is conductor/builder latitude) ·
  imported tools built general, never minispec-scoped
  (`301:law-imported-tools-built-general`) · proceed GREENLIT, conductor discretion
  ("I am here to work with you").
- 2026-08-14, the builder-prerequisite dictum [TYPED]: landed as durable law in the
  conductor skill itself (its quarantine section; human commit). §2 carries the
  pointer; successor conductors get it from the skill at boot.
- Standing carry-overs: the `KNOBS:kSURVIVAL` status-line edit remains the human's
  (28T inheritance); silence ≠ ack; only typed text counts.

## §6 — The settled-rules census (BANKED; the enrichment item's tabled menu)

Gathered 2026-08-14 by a criteria-driven scout over KNOBS · `spike/CLAUDE.md` ·
`crates/{core,analysis}/CLAUDE.md` · FORFEITS · `277` · `271` · `28Q`;
conductor-adjudicated. The MENU IS TABLED [TYPED] — selection happens at the
enrichment item, never before. Criteria: explicit ratification evidence ∧ statable as
value-algebra ∧ off 28Q's moving edge.

Passing (evidence as found):
- `ternary-compare-consumer-map` — acked (`271` task-12 closing sweep 2026-07-12;
  `277` §9). Caveat: the relation shape + consumer map only; the fuller generator
  registry is still conductor-proposed. Named in `28Q` §6's preserved wall.
- `set-lifting-universal-meet` · `pin-set-meet-order-independence` ·
  `pin-no-outcome-as-generator` — ACKED, typed, 2026-07-16 (`277` §5 / the `279f`
  ack batch). The first is named in `28Q` §6; the third is not individually
  (~SUSPECT rider).
- `inv-backing-set-nonempty-by-construction` · `inv-top-never-encoded-as-empty` —
  acked 2026-07-17 (`27Xf:cr-set-lifting-vacuous-at-empty`); the measured
  vacuous-∀ design-bug class.
- `never-derive-separation` — acked "spike-tier-because-foundational" (`271`,
  2026-07-12); named in `28Q` §6.
- `top-identifies-with-nothing` — WEAKEST evidence in the set: "unchanged" across
  three rounds, NO dated typed marker found anywhere. Candidate calibration probe for
  the enrichment item's question-router (it should ask for confirmation).
- `rul-coordinate-shape-flat-three-place` — typed (`271`, 2026-07-10); light moving
  edge only (`28Q` §3 extends the context slot; the flat shape itself unchanged).
- `silence-licenses-nothing` · `inv-top-reject` — named unchanged in `28Q` §6.
- Settled but STRONG moving-edge (excluded from near-term proving): `rul-family`
  (typed, but `28Q` §1/§2 reshape membership frame/closure-relative) ·
  `pure-predicate-carry` (human-opted 2026-07-17, but `28Q` §3 grows its axis
  vocabulary).
- Settled but not value-algebra: `empty-world-byte-identical` (whole-system
  differential property; its evidence stays the corpus differential).

Conductor adjudication deltas:
- `inv-must-may` SPLITS: the coercion ban is compiler-tier (evidence = the
  `compile_fail` seals); the `Must`-as-order-dual SEMANTICS is genuine value-algebra
  and underlies the certifier's one-checker duality.
- MapL canonical-form (structural-Eq = semantic-Eq) fails the scout's
  settledness-marker criterion but enters anyway via the facade lane as its
  honour-system invariant.
- `rul-rc-partition`'s ≥2-flat-sink ("flat FOREVER") is borderline algebra-content;
  benched.

Excluded as not-settled (soft/forfeit/refused): the sparing dialect-resolution core
(typed-spike-provisional + acked-SOFT + `pin-two-position-sparing` extremely-soft +
its FORFEITS row) · `forfeit-committee-fence-sparing-inert` (UNRATIFIED) ·
`kind-fence-movable` (a reserved seam, not a ruling) · the `275` transport
ratifications (REFUSED, `279f` §3).

Doc-coherence note, repaired in `28Q` §6 this arc: "the sparing algebra" in the
preserved wall means the set-meet SUBSTRATE (hard-acked, above); the
dialect-resolution rule is `28Q` §9 `pin-two-position-sparing` territory (soft).
