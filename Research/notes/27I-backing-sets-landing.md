# 27I — block-rebuild stage-4b (effect-plane backing-SETS) landing + residue

AI-authored (Opus builder, r27 stage-4b session). Records what landed for `270:block-rebuild`
stage 4b — the effect-plane backing SET: `Backing`→coordinate-SET, observe-backing-widening
PRODUCTION, the survival universal-meet migration, and threaded minting-family — against `277` §5
(`seam-backing-sets`, the set-lifting law, the fixpoint-soundness clause) + `271`
observe-backing-widening. This is the design's ONE naked-trust cell (survival sparing); every
ambiguity resolved toward collide/run. Companion to `27D` (the conductor ledger), `27G`/`27H`
(stages 3/4). Authority: root docs + `spike/CLAUDE.md` + `271`/`277` outrank this. Branch:
`ai/r27-backing-sets` off `ai/spike3-r27` (base `ab740a3`). This landing is
finding-observe-backing-widening-production-is-effect-plane (`27H`) discharged.

## What landed (all green: 691 unit + 128/128 e2e; four gates clean; ZERO golden churn)

Byte-identity was the gate and it HELD — both new mechanisms are DORMANT on the corpus by two
independent facts: every corpus `:?` observe is STANDALONE (no co-occurring verdict ⇒ no
widening); every corpus disturbs emission is whole-entity ⊤ (`27G` — so `selector_covers` gets a
⊤ claim and collides regardless of the family, keeping the family-thread dormant too). The
mechanisms are PRODUCING (not a reserved seam), proven by targeted unit tests, not by corpus
behavior change.

- **Part B — observe-backing-widening derivation (`oracle::lift`).** `KindIndex` gains
  `widenings: BTreeMap<(ProviderId, Symbol), BTreeSet<SelectorId>>` + `widening_of`. `lift` groups
  each provider's `derive_predict` cells BY VERB (an arm = a verb — `derive.rs` binds one verb per
  literal-pattern arm): a verb with ≥1 VERDICT (`:`/`:!`) mark routes its co-occurring OBSERVE
  (`:?`) selectors to `add_widening` INSTEAD of an effect cell (a mixed `[Establishes, Queries]`
  slice would fall to `MustRun`); a verb with NO verdict keeps its observes as `Queries` cells
  (standalone — "widens nothing, its own row"). The observe→verdict linkage lives at the
  `(provider, verb)`/arm granularity because the `derive`/`cell_effect` model forces every cell of
  one `(provider, verb)` at a book site to ONE `(kind, entity)` (the mark's own `target.kind`/
  `target.entity` are discarded; only `target.prop`=selector survives), so a widening is a SIBLING
  cell — same kind+entity, this selector.

- **Part A — `core::FactBacking` + the threading.** `core::FactBacking { family: Option<ProviderId>,
  observed: BTreeSet<SelectorId> }`, keyed `BTreeMap<FactKey, FactBacking>`, built in
  `analysis::command_effect` (a new `&mut` out-param, `record_backing`) for every Establish fact:
  `family = Some(this site's provider)` (EXACT — not the `sole_family` reverse-lookup), `observed =
  widening_of(provider, verb)`. Cross-provider collision (same fact, two providers) merges family→
  `None` (safe floor) + unions observed. `classify_with_why_diags` returns it as a 5th tuple
  element; the `classify` wrapper discards it; the cli/sweep thread it into `build_plan_walled`
  (coverage/hostsim/plan-tests pass an empty map — survival-off there).

- **Part A/C — `plan::survival` (the naked-trust cell).** `Backing` is now a SET: its own cell +
  the observe-widened siblings, carrying the threaded family. `disjoint` runs the `277` §5
  UNIVERSAL MEET — an outer loop over `member_selectors()` inside the footprint loop; spared iff
  EVERY (footprint coord × backing member) pair is `ProvablyDisjoint`; ANY member `Overlaps` ⇒ Hit
  (collide), ANY `Unknown` ⇒ may-alias. Each member's family is the threaded `Some(P)` (authoritative
  past `fence-divergent-meaning`) or, `None`, recovered per-member-selector via `sole_family` (the
  map-miss floor = today's behavior — file-writes / auto-cells / Members facts). The auto-cell
  `fence-no-disjoint` and the `279f` ⊤-either-side pin hold member-wise (every member shares the
  fact's kind/entity, so the one kind-check + the `selector_covers` ⊤ arm cover the set).

## Final shapes (report item 2/3)

```rust
// core
pub struct FactBacking { pub family: Option<ProviderId>, pub observed: BTreeSet<SelectorId> }
// plan::survival — Backing is no longer Copy (holds a BTreeSet)
pub struct Backing { coord: EntityCoord, selector: Option<SelectorId>,
                     family: Option<ProviderId>, widen: BTreeSet<SelectorId> }
pub fn Backing::widened(fact: FactKey, family: Option<ProviderId>, observed: BTreeSet<SelectorId>) -> Backing
fn   Backing::member_selectors(&self) -> impl Iterator<Item = Option<SelectorId>>  // own, then widen (BTreeSet-ordered)
pub struct DisjointnessProof { backing: EntityCoord, family: Option<ProviderId> }   // .family() cites the minting family
```

The observe→verdict linkage (report item 3): built in `oracle::lift`, per `(provider, verb)`
group of `derive_predict` rows — `has_verdict` gates whether the group's observe selectors widen
(`add_widening`) or stay `Queries` cells. Threaded fact→`FactBacking` through
`command_effect`→`resolve_node_effects`→`classify_with_why_diags`→cli→`build_plan_walled`→
`wall_walk_survival`, where `Backing::widened` (map-hit) / `Backing::of_fact` (map-miss) is built.

## Part-D enumeration (report item 4): NONE

ZERO corpus behavior changes; 128/128 e2e passed UNBLESSED (byte-identity). No demotion, no golden
churn, no bless commit. The argument: (1) no corpus `:?` observe co-occurs with a verdict, so no
establish fact is widened — every corpus observe is a verbless ε-verb standalone (`dpkg -s`,
`grep -q`, `otelcol --version`, `command -v`) staying its own `Queries` cell; (2) no corpus
disturbs emission carries a `#selector` (all whole-entity ⊤ — `27G`), so `selector_covers` sees a
⊤ claim and collides for every backing member regardless of family, making the threaded family
observationally inert. The two facts are independent, so byte-identity is robust, not coincidental.

## Pins extended to REAL sets (report item 5)

Both `27G` pins were SYNTHETIC (a `Vec<Relation>` fold in `core::coord`). Now exercised on REAL
widened `Backing`s in `plan::survival` (`inv-referent-agnostic`, DST-clean):
- **pin-set-meet-order-independence** — `observe_widened_backing_collides_where_the_bare_fact_would_spare`
  (a `#enabled` fact spares a `#active` disturbs bare, but the observe-widened `#active` member
  collides the SET) + `widened_backing_survives_when_every_member_is_disjoint` (all members
  disjoint ⇒ spare). The meet is a pure fold over `member_selectors()`; any member collides the set.
- **pin-no-outcome-as-generator** — structural: `disjoint` never stores a member's spare/collide
  outcome to feed a later member's compare (a `compare` verdict feeds only the survival consumer).
  Cited in the `disjoint` doc; the core synthetic pin still guards the law with reversed order.
- Plus `threaded_family_spares_where_the_reverse_lookup_would_collide` (the `277` §3 provenance
  improvement — two families minting the same tokens ⇒ `sole_family` None ⇒ collide, but the
  threaded family spares) and `widened_backing_with_an_auto_member_kind_may_aliases_member_wise`
  (the `fence-no-disjoint` member-wise).

The fixpoint-soundness clause (`277` §5, human-banked): the universal meet is the fixpoint-robust
form — a not-yet-licensed member reads unknown ⇒ the set collides, evaluation-order-independent.
The spike at HEAD has NO probe-re-entrant back-edge (the value plane runs strictly before the
probe; plans mint once), so `pin-no-outcome-as-generator` holds trivially here; the clause is the
standing law the post-probe re-bind re-reads the day it is designed.

## tc-* / judgment calls flagged UP (never settled locally) — report item 6

- **tc-backing-family-via-dialect-reverse-lookup** (from `27G`/`27H`): RESOLVED for the FAMILY —
  threaded `Some(provider)` is now authoritative for oracle-establish facts, replacing the
  reverse-lookup on the threaded path. The reverse-lookup SURVIVES as the map-MISS floor
  (file-write / auto-cell / Members facts — which mint no marked selector, so `sole_family` is
  `None` for them anyway; inert). The **minting LINE** half is NOT threaded (see deferrals).
- **tc-context-slot-on-coord-not-factkey** / **tc-resolutions-stays-in-plan** (carried from
  `27G`/`27H`): UNTOUCHED here — this change adds no context slot and does not move `Resolutions`.
  Still open; carried to block-context planning.
- No NEW cross-cutting `tc-*` judgment was settled locally. Two design choices made (documented,
  not punted): (a) a co-occurring observe becomes a widening and does NOT also stay a `Queries`
  cell — correct because it is an oracle-internal read of the verdict's probe, not a book command,
  and keeping it a `Queries` cell would regress the establish site to `MustRun`; (b) an observe
  widens ALL verdict facts in its arm (multi-cell verb) — the safe direction (each fact's
  kill-surface only grows).

## Deferrals (with rationale)

- **rider-minting-line-attribution** — the DisjointnessProof cites the minting FAMILY
  (`ProviderId`), not the oracle mark's SOURCE SPAN. Threading the span would push spans through
  the effect map (`EffectCell`) + `FactBacking` + the survival attribution — a large surface with
  ZERO corpus-observable effect (sparing never fires on the corpus, so no sparing verdict ever
  renders it). Deferred to the day a sparing verdict actually ships (block-context / a
  selector-bearing-disturbs corpus). The family is the load-bearing provenance.
- **rider-member-widening-for-members-sites** — a Members site's facts fall to the singleton
  `Backing::of_fact` (a throwaway backing map in `member_family`), so an in-loop establish is not
  observe-widened. Safe: a Members site is render-floored (every member RUNS), so its survival is
  already the conservative case. Deferred, corpus-absent.
- **cross-kind observe widening** — the current `derive`/`cell_effect` model discards a mark's
  `target.kind`/`target.entity`, so a `:?` observe can only widen a SIBLING cell (same kind+entity,
  different selector). An observe genuinely reading a DIFFERENT kind/entity is not representable
  today; if a future model captures it, the widening representation (`FactBacking.observed:
  BTreeSet<SelectorId>`) must widen to carry full coords. Flagged for that model change.

## Commits (on `ai/r27-backing-sets`)
1. `e4c6da9` oracle: observe-backing-widening derivation (co-occurring `:?` widens; standalone stays a Query cell).
2. `9bc0b4f` core+analysis: `FactBacking` + threading from classify (minting family + widening selectors).
3. `0214c84` plan: `Backing`→SET; survival universal-meet; widening + threaded family PRODUCING.
