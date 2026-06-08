# 17B — the kill-floor is *two* floors; coordination-failure is non-directional (round 17, 2026-06-07)

> Triggered by the human's adversarial challenge (a "colleague" disputing my "cross techniques off
> without fear" claim) + three nits + the expanded `IMPLEMENTATION.md` priority enumeration. This is a
> reasoning round (no new sources gathered); grounded in the in-corpus spines + `IMPLEMENTATION.md`'s
> four-outcome ordering. Certainty-marked; the human adjudicates. Strawmen are illustrative, NOT locked
> design (per AGENTS: don't lock-in via strawmen).

## Findings (lifted)
- **F1 · the colleague is substantially right; my kill-floor was over-scoped.** It holds for one axis
  and fails for another:
  - **depth axis** (how hard the analysis thinks — interproc, context, precision-mechanism): killing →
    less certainty → *unknown* → run. Drains to over-run (`unnecessarily-` / `over-execute`,
    `IMPLEMENTATION.md` priorities 2–3). Kill-floor HOLDS; killing judged on speed/value. +SURE.
  - **fidelity/coordination axis** (what states exist; selector granularity; when oracle-A's
    state-dependency is discharged by oracle-B's state-test): killing *by coarsening* → **over-correlation**
    → wrong-discharge → `under-execute` (priority **1**, the worst). Kill-floor does NOT hold. +SURE.
- **F2 · a coordination/fidelity feature fails non-directionally.** False-negative correlation (fail to
  match when you should) → A can't confirm its precondition → runs → `over-execute` (safe-ish).
  False-positive correlation (match when you shouldn't) → A wrongly confirms → elides → `under-execute`
  (dangerous). The human's wombat-frock example is the *false-negative* (safe) variant; the false-positive
  variant is equally reachable. So "simpler = safer" is false here; complexity is the wrong axis. +SURE.
- **F3 · directionality is RESTORABLE — but by a contract, not by the floor.** Make correlation
  **MUST-grade-to-correlate**: a discharge is licensed only by an explicit/structurally-anchored agreement
  that two operations touch the same (kind, selector); absence/ambiguity ⇒ no-match ⇒ run. Then failures
  are pushed to false-*negatives* (over-run, safe). Over-coarsening (boolean-where-enum-is-needed) is an
  *optimistic-correlation* move → false-positives → forbidden. (= Engler MUST/MAY, `096`, applied to the
  *correlation* itself, not just the anchor.) ~SUSPECT this is the clean resolution.
- **F4 · the boolean false-is-safe default does NOT cover this.** "Unsure ⇒ false ⇒ run" protects
  *within-oracle* uncertainty; it does nothing against *cross-oracle* over-correlation (B confidently
  returns *its* true, wrongly identified with A's fact). So the concern exists at **any cardinality ≥ 1**;
  enum just enlarges the mis-correlation surface. +SURE (answers the human's "true/false too?" — yes).
- **F5 · the fidelity-floor is set by the world, not by us (nit-1).** The state-model must distinguish at
  *least* the real entity's mutation-gating states. systemd alone forces ≥3 for a unit
  (installed / enabled / active are independently mutation-gating), so boolean is below-floor. The floor is
  empirical → corpus owns `dq-entity-algebra`'s lower bound; above it the kill-floor re-applies. +SURE.
- **F6 · re-run is not harmless (nit-3).** `over-execute` (run > once) is priority-2, *above* the
  value-prop, because it is the one outcome *worse than blind-running* (`IMPLEMENTATION.md` L92: blind-run
  never multi-executes). Idempotence is best-effort-hoped, never assumed. My "harmless" framing was wrong.
- **F7 · the safe direction is phase-keyed (nit-2).** Probe phase: withhold/skip-the-probe is safe (no
  pre-plan mutation — a categorical promise, `IMPLEMENTATION.md` L85/89). Apply phase: perform/run is safe.
  The analyzer + type-discipline mostly live in the *probe* stage, where "skip it" is the safe move; it
  only reduces to "run it" at the *whole-system* level (probe-withheld ⇒ no fact ⇒ apply runs).

## Strawmen (illustrative; real-world systemd multi-state)
- **selector-confusion ⇒ under-execute** (the dangerous kill). Model collapses service to boolean
  `service:nginx=up`. Probe runs `systemctl is-enabled nginx` → true; book is
  `systemctl is-active nginx || systemctl restart nginx`. "active" correlated to `up`=true ⇒ restart
  elided. Reality: enabled-but-crashed ⇒ service stays down ⇒ priority-1 failure. The enum
  {installed,enabled,active} prevents it (a distinct `active` selector that `is-enabled` cannot discharge).
- **still-wet / cross-facet** (cert→reload): `certbot certonly …` writes cert on-disk (wet);
  `systemctl reload nginx` is the maybe-elidable mutative. The real state is *loaded-matches-disk* (two
  facets). Coarsely correlating reload-dependency to on-disk-present ⇒ fresh cert (present, not-yet-loaded)
  ⇒ reload elided ⇒ stale cert served. The on-disk write must **kill** the loaded-facet (cross-oracle,
  cross-facet invalidation). = W5 ambient/transient, but spanning two authors.
- **cardinality: not-installed ≠ disabled.** `systemctl is-enabled nginx` returns enabled/disabled when
  the unit exists, *errors* when not-installed. Boolean `enabled:yes/no` can't hold the 3rd state ⇒ the
  error must be mapped, and the mapping is non-directional (mis-map ⇒ wrong-run for `enable`, or wrong-skip
  for a dependent guard).

## Coordination — author-obligation splits (theorized; the open design space)
- **split-1 · monolithic kind-owner** — one oracle owns `service` + its full enum + all probes/effects.
  Coordination = zero. Fails to compose (cross-manager apt-vs-brew; cross-concern systemd-active vs
  HTTP-healthy). Simplest, doesn't scale.
- **split-2 · kind-owner declares the enum; providers slot in per (state, provider)** — the Puppet RAL,
  extended to multi-state. Coordination = providers reference the kind's *state-vocabulary*. Risk
  (colleague): shared **name**, divergent **meaning** ("active"=systemd-active vs responds-to-HTTP) ⇒
  over-correlation. The contract must pin selector *meaning*, not just name — which bottoms into the
  state-grounding problem (K1-adjacent, for selectors). ~SUSPECT this is the RAL's real lesson + its floor.
- **split-3 · per-facet oracles + explicit cross-facet invalidation** — the cert/reload case. One author
  declares "writing facet X invalidates facet Y." Most expressive, highest coordination; the invalidation
  is itself a MUST-grade cross-oracle dependency.
- **unifying rule (proposed):** whatever the split, correlation is **MUST-grade-to-correlate** (F3) — the
  non-directional mechanism is forced to fail toward under-correlation (over-run, safe). The open
  author-obligation question = *who declares the correlation* (kind-owner / each provider / a
  relation-owner), minimizing coordination-surface while keeping the match MUST-grade.

## Implications for the K2 map
- The deliverable must **carry the depth-vs-fidelity axis distinction** front-and-center: the kill-floor
  licenses kills only on the depth axis; fidelity/coordination kills are judged on the fidelity-floor +
  fail-toward-under-correlation, not on complexity.
- **Re-split kill-5 (typestate):** keep the *state-distinctions* (fidelity — must-keep to ≥enum); kill only
  the *protocol-enforcement* (depth — we never reject anyway).
- **`dq-entity-algebra` lower bound is ≥enum** (nit-1), not boolean; stays bounded above that.

## Open questions for the human
- **oq-1** — is MUST-grade-to-correlate (F3) the intended resolution, i.e. correlation defaults to
  *no-match ⇒ run* and only explicit anchored agreement discharges? (This is the `dq-kOOB`/coordination
  contract shape, K2-side.)
- **oq-2** — which author-obligation split (1/2/3) is the lean, or is it per-kind (a mode)?
- **oq-3** — selector-*meaning* pinning (split-2 risk) is where K2's coordination touches K1's grounding;
  is that a firewall hand-off, or genuinely K2's to bound?

## Citations (in-corpus spines this reasoning rests on)
> [A-tobin-hochstadt-logical-types-2010] (relevance: +1:SURE)
> selectors (the "object" / `car(p)`) ⇒ kinds are *structured*; the latent proposition is per-(command,
> selector) — i.e. fidelity below the real selector-set cannot narrow correctly.

> [A-foster-flow-sensitive-qualifiers-2002] (relevance: +1:SURE)
> strong-update gated by *uniqueness*; a non-unique entity forces weak-update (⊤-ward). Over-coarsening
> manufactures false uniqueness ⇒ unlicensed strong-update = the over-correlation hazard, mechanized.
