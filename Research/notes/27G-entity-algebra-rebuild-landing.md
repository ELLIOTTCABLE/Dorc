# 27G — block-rebuild stage-3 (entity-algebra-rebuild) landing + residue

AI-authored (Opus builder, r27 stage-3 session). Records what landed for `270:block-rebuild`
stage 3 (the entity-algebra re-key: the coordinate comparison chokepoints, the selector dialect,
the context slot) against the `notes/277` spec. Companion to `27D` (the conductor ledger),
`27E`/`27F` (stages 2/2b). Authority: root docs + `spike/CLAUDE.md` rulings + `271`/`277` outrank
this. Branch: `ai/r27-entity-algebra` off `ai/spike3-r27` (base `d524c27`).

## What landed (all green: 679 unit + 128 e2e; four gates clean; ZERO golden churn)

- **The two chokepoints in `core` (`277` §§1–3; the deliverable).** `core::coord`:
  - `selector_covers(claim, backing, dialect) -> bool` — the ONE selector comparison. `true` =
    collide (the conservative floor); `false` = SPARE iff both carry minted selectors (∈ `dialect`)
    and `claim ≠ backing` (§3, as amended by `279f:fix-spare-top-backing` — a ⊤ on EITHER side
    collides, the backing side especially). Empty dialect ⇒ never spares ⇒ entity-granular.
  - `compare(claim, backing, claim_canon, backing_canon, dialect, backing_family) -> Relation` — the
    ONE whole-coordinate chokepoint, ternary `{ Same | ProvablyDisjoint | Unknown }`. Composes:
    context, the movable kind-fence, entity-canonicalization (fed as `EntityResolution` from the
    resolve generator — the §2 registry model), then `selector_covers`. Consumer map welded.
  - `selector_identifies(a, b) -> bool` — the transport-direction primitive (`top-identifies-with-
    nothing`: ⊤ never identifies, including itself). Built now, consumed by the transport consumer
    at block-context; `compare`'s `Same` is the survival-collide reading (overlap, not identity).
  - `Coord { kind, entity, selector: Option<SelectorId>, context: Context }` — the comparison
    representation holding the reserved **context slot** (opaque `Context::HostDefault`, name
    unminted, populated by nothing yet — §5 `seam-context-qualifier-slot`).
  - `Dialect` — per-`(family, kind)` minted-selector sets + provenance recovery (`sole_family`;
    ambiguous ⇒ `None` ⇒ safe floor, `fence-divergent-meaning`). Built from the lift
    (`oracle::build_dialect` over the `KindIndex` verdict/observe cells — the exact minting set;
    disturbs never enter the index).

- **Routing (`plan::survival`).** `disjoint` now goes THROUGH `core::compare` per footprint
  coordinate; the crate no longer compares axes inline. Mapping preserves byte-identity +
  demote-reasons: `ProvablyDisjoint`→clear(spare), `Same`→`Hit`(Poisoned), `Unknown`→`MayAlias`.
  The auto-cell `fence-no-disjoint` stays a pre-`compare` short-circuit (auto kind ⇒ MayAlias).
  `Footprint` carries the disturbs-emission selector (a side-table, keeping the entity-granular
  render/canon untouched); `Backing` carries the fact selector; the backing family is recovered
  from the dialect. `Dialect` threaded `build_plan_walled → wall_walk_survival → wall_verdict →
  disjoint`.

- **The lift.** `EmittedCoord` gains `selector` (from `mark.target.prop`). `split_mark_target` now
  extracts the emission `KIND#SELECTOR` form (`rul-emission-selector-on-mark` — previously the
  disturbs selector absorbed into the kind). Riders 1/2: a non-name selector is a LOUD ⊤-reject
  (`277` §4b); a claim-emission brace `#{a,b}` EXPANDS to one coord per token (§4c); a
  verdict/observe brace mints no cell (single-cell law).

- **Why byte-identical.** The whole re-key is DORMANT on the corpus: every corpus disturbs emission
  is whole-entity (⊤), so `selector_covers` always collides ⇒ the entity-granular floor
  (`empty-world-byte-identical`). The dialect fires only when an oracle authors a selector-bearing
  disturbs mark. 128/128 e2e passed WITHOUT re-blessing (no golden delta).

## Chokepoint signatures (report item 2 — the value-recipe-reshape stage consumes backing-SETS through these)

```rust
pub fn selector_covers(claim: Option<SelectorId>, backing: Option<SelectorId>,
                       dialect: &BTreeSet<SelectorId>) -> bool;
pub fn compare(claim: Coord, backing: Coord, claim_canon: EntityResolution,
               backing_canon: EntityResolution, dialect: &Dialect,
               backing_family: Option<ProviderId>) -> Relation;
pub fn selector_identifies(a: Option<SelectorId>, b: Option<SelectorId>) -> bool;
```

## Fences/pins landed as tests

- `core::coord`: 279f top-backing regression · empty-dialect-never-spares · cross-family-monotone ·
  top-identifies-with-nothing (selector_identifies) · cross-kind-has-no-same-generator ·
  never-derives-separation-from-unknown · same-token-divergent-meaning differential (the priced
  footgun) · universal-meet-any-unknown-collides-order-independent (§5, SYNTHETIC set —
  pin-set-meet-order-independence + pin-no-outcome-as-generator) · sole-family disambiguation.
- `plan::survival`: `dialect_selector_bearing_disturbs_spares_sibling_cell` (the REAL disturbs ×
  dialect-selector DST case, end-to-end — `#active` spares `#enabled`; empty dialect collides; ⊤
  backing collides) · `synthetic_cross_generator_consumer_map_holds`.
- lift: emission-selector-rides-the-mark · brace-alternation-disturbs-expands · non-name-selector-
  loud-reject · brace-selector-parses.

**DST cross-generator cases (`279f` §5):** disturbs × dialect-selector = REAL (survival test).
mapped-lend × keyed kind + full-lend × invariant kind = SYNTHETIC (the lend + invariance generators
arrive at block-context; pinned as the registry-shape consumer map, clearly labeled synthetic).

## tc-* / judgment calls flagged UP (never settled locally)

- **tc-context-slot-on-coord-not-factkey** — the context slot lives on `core::Coord` (the comparison
  representation), NOT on `FactKey`. Rationale: `FactKey` is a `BTreeMap` key across ~47 sites; a
  slot that is always `HostDefault` would perturb hashing/ordering for zero behavior. `Coord`
  injects `HostDefault` at construction; the `FactKey`-level slot lands when the wrapper machinery
  populates it (block-context). The brief lists "facts" as a coordinate site — confirm this
  placement or direct the `FactKey` field.
- **tc-resolutions-stays-in-plan** — `Resolutions`/`CanonicalCoord`/`Resolution`/`MayAliasReason`
  stay in `plan::survival`; `compare` takes the resolve generator's OUTPUT (`EntityResolution`)
  rather than the map. This keeps the `CanonicalCoord` private-mint intact and avoids a large code
  motion, matching §2's generator-registry model (each generator feeds the chokepoint its licensed
  evidence). `core/CLAUDE.md`'s `canonical-coord-continuity` reads as if these are core-law —
  confirm plan is the right home, or direct the move.
- **tc-backing-family-via-dialect-reverse-lookup** — the backing's minting family is RECOVERED from
  the dialect (`sole_family(kind, selector)`), not threaded as provenance onto the fact. The spec
  says "backing provenance carried into the comparison." Reverse-lookup is exact for the one-family
  case and falls to `None` (safe collide) on ambiguity (`fence-divergent-meaning`). If the fact must
  carry its true minting family (e.g. to spare under divergent-meaning per the flagged footgun), the
  fact/license path needs a `ProviderId` slot — deferred.
- **tc-same-is-overlap-not-identity** — `compare`'s `Relation::Same` means "not provably cell-
  separable" (the survival collide, includes ⊤-overlap and unminted-different tokens), NOT "same
  referent". This preserves byte-identity (same-entity collides → `Poisoned`) but means a future
  transport consumer must additionally require concrete-selector identity via `selector_identifies`
  (⊤ never transports). Documented on `Relation::Same`.
- **tc-brace-verdict-silent-skip** — a brace-alternation on a verdict/observe mark mints NO cell
  (safe: the arm declines ⇒ run), but SILENTLY (`derive_predict` is total/diagnostic-free by
  design). The `277` §4c "reject multi-cell there with a loud diagnostic" wants LOUD; the loud form
  needs a derive-side diagnostic channel — deferred.

## Deferred (with rationale)

- **rider-internal-rust-naming-debt** (rider 4) — the internal `touches`/`reaches` Rust names
  (`TouchesSet`, `evaluate_touches`, `strip_touches`, `TouchesResolution`/`TouchesTop`,
  `lift_touches`, `resolve_touches_footprint`, `ship_touches_body`, the `touches.rs` module) are NOT
  renamed to the `disturbs`-family. It is a large mechanical rename across oracle/cli/sweep/plan for
  purely-cosmetic internal naming (the AUTHORED `__disturbs` surface is already correct and the
  machinery is doc-commented as the disturbs lift). Deferred to avoid churn/risk near the re-key
  landing — "in passing" was not cheap here.
- **rider-resolver-coverage-watch** (rider 3) — RESOLVED as verified-sound, no code change: the
  collected-kinds set (`collect_coord_kinds`) is exactly the backing + footprint population the
  survival comparison ever canonicalizes, so there is no silent under-cover. Converting to a
  structural (not collection-based) coverage is a cli-edge resolver-shipping refactor the
  comparison-layer re-key does not subsume; documented at `collect_coord_kinds`.
- **The `FactKey`-level context slot** and the **backing-SET consumers** (`277` §5) stay reserved —
  the representation room + the universal-meet law are pinned; the consumers arrive with the
  value-recipe-reshape (backing SETS) and block-context (wrapper context, lend/invariance
  generators).

## Commits (on `ai/r27-entity-algebra`)

1. `c519d52` core chokepoints (selector_covers, compare, Dialect, Context, Coord, selector_identifies).
2. `8113302` route survival disjoint through core::compare (footprint/backing selectors, dialect wiring).
3. `a684a8a` pin §6 fences + the selector-sparing DST case.
4. `09f969f` selector charset diagnostic + brace-alternation (riders 1/2).
5. `4e9131a` document rider-resolver-coverage-watch soundness.
