# notes/181 — analyzer-needs (`ananeed`) extraction: method + provenance

**Stamp:** round 17 · 2026-06-06 · the build record for the root `ANALYZER-NEEDS.md` (the living
`ananeed-1`×`ananeed-2` table). Findings live in that table, not here; this is method + what the
adversarial pass caught, for traceability.

## Method (a sequenced pair, not blind-parallel)
- **Neutral wave (7 agents, one per corpus cluster):** human-docs+KNOBS · engine-core (021/055/050–054) ·
  state+specimens (090/099/09A/091–096) · perf+pluggability (064/076/077/083/078/070–075) ·
  security+error+provenance (100/102/110–113) · tdd+transport+platform+round-15 (120–128/140–142/130–139/150–151)
  · the spike (16P/16Q + a targeted recall over the quarantined 160–16O). Each returned terse pipe-rows;
  ~700 raw rows, heavy cross-agent overlap (the convergence = the recall signal).
- **Merge:** deduped to ~150 canonical `an-*` needs, grouped A–N + meta §Z, dual-aware, status-tagged
  (B/S/D/O/W). Written as `ANALYZER-NEEDS.md` v1.
- **Adversarial wave (3 gap-hunters fed the v1 table):** engine/state/substrate · provenance/verdict/transport
  · human-docs/oracle/cost/platform/modes. Each re-read its region *against* the draft, emitting only
  deltas (missing / corollary / dual-half / mis-stated). ~40 keepers integrated → v2 (~190 rows).

## What the adversarial pass caught (the recall payoff — these were genuinely missing/wrong)
- **Forgotten features / fact-kinds:** `an-word-expansion` (021§2 emphatic, survived only as a sub-clause);
  `an-version-applicability` (the MH2 layer; TODO-ADDTL "top, genuinely lost"); `an-host-identity-fact`
  (hostname/group/arch — the README *leads* with it); `an-privilege-fact`; `an-inventory-input`.
- **Dropped sub-fields:** the K8s `metav1.Condition` shape was harvested for three-valuedness only —
  `an-verdict-reason-id` / `-last-transition` / `-noop` / `-failsafe-default` / `an-report-correlation-id`
  were dropped; the locator-DAG flattened `an-loc-session` + `an-diag-accretion` + `an-graft-provenance`.
- **The recency keystone split (high-lock):** `an-fresh-vs-summary-entity` (the *representational shape*,
  retrofit-hostile) separated from `an-strong-weak-update` (the *mechanism*, low-lock) — the exact merge
  `16Q§1` says spike-1 made fatally.
- **Corrections:** `an-status-lane` conflated FIFO atomicity with `O_APPEND` (a latent corruption bug,
  150-R13 → new `an-append-corruption`); `an-effect-class` had folded Opaque into ⊤ and dropped the
  carried location-set; `an-distributive-split` undersold a *termination* hazard as mere precision.
- **"So obvious nobody wrote it":** `an-transfer-fn`, `an-merge-op`, `an-h-sparsity`, `an-build-then-solve-coupling`.

## Status distribution → where the lock-in is
The `O` (open, high-lock) rows cluster in §C (entity-algebra: `an-entity-shape`/`-strong-weak-update`/
`-fresh-vs-summary-entity`/`-uniqueness`/`-per-entity-selector`) and §M (substrate: `an-substrate`/
`-distributive-split`/`-ide-value-layer`/`-graph-type-agreement`/`-async-vs-statemachine`/`-context-key`/
`-flat-domain`). **These are exactly the `ananeed-3` targets** — the algorithm/data-structure choice is
forced by, and must be matched against, this `O`-cluster. Wave-1 substrate prior-art (`notes/180`:
IFDS/IDE · Datalog/Soufflé · recency/singleton) is the input.

## Next
- `ananeed-3` synthesis → `plans/170`: map the `O`-cluster needs onto the substrate candidates (worklist /
  IFDS-IDE / Datalog), using `notes/180`. Then the human-requested "explain for idiots" pass.
- The root doc name is provisional (`ANALYZER-NEEDS.md`); avoided `FACTS.md` (collides with `kFACTS` + the
  engine's runtime "facts").
