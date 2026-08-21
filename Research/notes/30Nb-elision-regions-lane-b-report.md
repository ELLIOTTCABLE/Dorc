# 30Nb — Lane B report: `plans/30L` stages 0–3 (elision regions + loop staging)

> Tier: builder lane report (Opus, worktree `agent-ae1a99e81828d832b`, branch
> `ai/r30-elision-regions`, based on `ai/r30-conduct` at `7f7cb359`). Scope: `30L` §12
> stages 0, 1, 2, 3 — the ground-truth battery, the identities, and the universal meet.
> Stages 4–6 (settlement projection, effective reach, render, Spine) are NOT in this lane
> and nothing here is wired into them.

## §1 — Stage 0: the verdict-primacy repair is folded (evidence)

**VERIFIED at this lane's base.** `30La`'s member/inline verdict-primacy repair is present at
`7f7cb359` and both product-facing cases are promoted, not xfailed: `4a993667`
("(AI test) Promote verdict-body round trips") DELETES the `XFAIL` marker files from
`crates/cli/tests/aggregate30-{inline,member}-verdict-primacy` and folds the real transcripts, and
`09045ea0` ("(AI fix ana) Bind aggregate measurements to reached verdict bodies") is the engine
half — `compile_probe`'s member and inline lanes are re-cut so `is_vouched` takes `(node, fact)`
per member, an exact `all_vouched` population selects `ship_auto` (the VERDICT body) over
`ship_body` (the predict), and `verdict`/`emits_report` ride that same population. Both cases run
GREEN at my tip (`mise run test:e2e-quiet -- aggregate30`, 2/2), and their committed
`expected.out` ships `apt_get__is_converged` — the verdict body — as the `site 0.0`/`site 0.1`
checks, with the authored diverged records (`effect=absent rc=1`) keeping both mutations runnable
in the apply. So the member/inline lanes license through shipped verdict bodies and not predict
measurements, which is what §4.1 `req-member-lane-ruling-precedes-consumption` required before
this plan consumed the primitive. No stop.

## §2 — Stage 1a: the sizing measurement, and the constants chosen

### The measurement (corpus-shaped strawmen, AST-subtree counts — the unit the per-site check bills)

| strawman | largest funcdef body estimate | outcome under the INHERITED budgets |
|---|---|---|
| `fixtures/pi-webhost.book.sh`'s body wrapped in `main() { … }` (15 commands, ~45 lines) | **63** | admitted, with ONE node of headroom under 64 |
| that body ×3 (43 commands) | **185** | REFUSED, per-call budget |
| `main → task_fn → helper` (the motivating floor) | 9 | REFUSED, depth budget |
| a realistically factored book, 5 tasks × 1 helper each | 12 | REFUSED, depth budget, on every helper |
| 20 independent top-level helper calls (140 commands) | 16 | admitted; 120 spliced nodes, far under the 1024 per-book cap |

Two readings, both load-bearing. First, realistic sh runs **~4.2 AST nodes per command**, so the
inherited 64-node per-call budget admitted exactly one whole-book wrapper — the pi-webhost
strawman, at 98% of budget — and refused every book larger than fifteen commands. The
`main() { whole book }` shape this stage exists to descend into was one command away from
impossible. Second, the depth budget refused the motivating factoring outright, exactly as `30L`
§3.4 predicted.

Wall-clock (debug build, parse+build, this workstation): 200 calls / 1005 spliced nodes = 6.0ms;
500 calls / 2505 = 27ms; 1000 calls / 5005 = 101ms. `build` is O(calls × AST size) because every
splice ATTEMPT walks the whole AST for its estimate — pre-existing, unchanged by the re-size, and
comfortably inside "fail on human timescales" at any book a human writes.

### The constants, set deliberately (`analysis/src/cfg.rs` `inline_budget`)

| | was | now | why |
|---|---|---|---|
| `MAX_DEPTH` | 2 | **4** | `main → task_fn → helper` needs 3; 4 leaves one level for `main → task → helper → leaf`, and stops there because each level multiplies clone count by the fan-out |
| `MAX_NODES_PER_SITE` | 64 | **1024** | at the measured ~4 nodes/command this admits a ~250-command whole-book wrapper — past the top of the hand-written range, while still refusing a machine-generated body |
| `MAX_NODES_PER_BOOK` | 1024 | **4096** | 4× the per-site cap: one whole-book wrapper plus three more full bodies of factored-helper splices. This is the multiplicative backstop — per-site bounds ONE body, only this bounds fan-out^depth |

Over-budget stays proportional refusal with its named diagnostic; the depth-5 chain still refuses
by name, and the two extremes tests now find the boundary by walking outward rather than by naming
a constant, so they survive the next re-size.

**These are licensure-widening knobs, not perf knobs** — the doc comment now says so. An un-spliced
call is `Opaque` ⇒ ⊤ ⇒ a poison wall; raising a budget makes mutations VISIBLE that were hidden
behind a wall and makes downstream elisions available. The corpus is byte-identical under the
raise (measured: 170/170 e2e cases unchanged), because no committed case carries a depth-3 chain
or an over-64-node body — but the next lane should read the raise as a licence surface, not a tidy-up.

A coupling that was a comment became a derivation: `value::MAX_INLINE_PASSES` (the bounded
positional-binding pass) was `3` with a doc saying "depth ≤ 2 ⇒ at most 3 passes". It now reads
`inline_budget::MAX_DEPTH + 1`. Under-iterating there is safe (the pass only ADDS bindings and a
missing one leaves the ordinary ⊤-positional argv), but the stale sentence would have been a lie.

### A defect the re-size exposed, and fixed

`splice_funcdef_body`'s body-leaf flatten **double-counted at depth ≥ 3**. Measured on
`main → task_fn → helper`: `main`'s `call_body_sites` came out `[apt-get, systemctl, apt-get, ufw]`
— the bottom mutation twice. The retired dedup subtracted already-flattened LEAVES, which hides a
nested leaf but not the nested CALL that produced it, so the middle call re-flattened its own body.
Now the scan takes an inner call's list and skips its whole arena REGION (a new builder-local
`spliced_ranges` map). The population was unreachable under the old depth budget, and its failure
direction was safe (a duplicate member rejects the whole aggregate — `rul-every-erased-establish-is-vouched`),
but it would have silently forfeited every depth-3 elision this stage exists to deliver.

## §3 — Stage 1b: clone-vs-overlay (DECIDED; OPEN for fold-review)

**Decision: keep per-invocation body CLONES, and keep (and now name) member OVERLAYS. The answer
is a split, not a choice.**

Reasoning, in the order it bound:

1. **Per-instance CFG position is load-bearing.** §4.2 requires each route instance to consume the
   effective reaching-wall set *at its own CFG position*. With one lowered body there is no such
   position; recovering it means per-instance in-states on a shared node — which is
   context-sensitivity in the k-CFA sense, explicitly redlined by `28Q` §1 (`an-flat-domain`:
   "intervals in one linear program order plus a statically-known fork tree — no call-strings, flat
   domain values").
2. **The cited precedent is not an alternative to cloning.** `value.rs`'s positional overlay
   (`positional_argv`) is keyed BY the spliced body site's own `CfgNodeId` — it is a side-channel
   layered ON clones, not a replacement for them.
3. **Loops already ARE the overlay case.** A loop body is lowered once with a real back-edge;
   `ValueFlow::member_argv` + `SkipClass::EstablishMembers { members }` are per-member overlays on
   one node. So the engine already runs both representations, for the two different reasons.
4. Re-keying to overlays would move `ExecutionOwner::Leaf`, `call_body_sites`, `site N.M` records,
   and per-instance freshness — contradicting `pin-probe-site-identity-unchanged`.
5. Cost is bounded by the (now deliberate) per-book budget and measured in milliseconds;
   `perf-doctrine` says spend it.

Consequence, and why it is more than bookkeeping: `RouteInstance` carries BOTH axes — `cfg_node`
identifies the clone (invocation instances), `iteration: IterationSlot` identifies the overlay
member (iteration instances) — and a population may mix them without either axis meaning the other.
That is precisely why §2's struct lists `iteration` separately from `cfg_node`; it is the overlay
axis, not decoration.

## §4 — Stages 2–3: what was built

**`core::region`** (new): `ElisionRegion` (definition-keyed authored span; private fields; ONE
mint, gated on the universe) · `RegionUniverse` (pure data built at the driver edge, the
`CustodyClosures` shape; `admits(DefinitionCustody)` is the sanctioned custody-file read, the same
carve `custody::custody_reaches` takes) · `IterationSlot { NotIterated, Member(u32) }` whose
`member()` is exactly what `SiteId.member` carries.

**`plan::region`** (new): `InvocationId` · `RouteInstance` · `ClosedRoutes` (head+tail, non-empty by
construction) · `RoutePopulation = Closed | Open` · `RegionCensus` + `census(ast, cfg, diags,
universe, book)` · `RouteConclusion` (the region plane's input vocabulary) · `RouteAdmission`
(private fields, sole mint `project`) · `SharedStandIn` · `SharedGuard` · `RouteRegionProof` ·
private `SharedConclusion` · `SharedOutcome` + `SharedRegionAct` · `SharedRegionDecision` ·
`decide_region`.

Census closedness fails toward OPEN on three signals: a shell-level DYNAMIC-EXECUTION construct
anywhere in the unit (keyed on the syntax reason, so an unmodeled EXTERNAL command never opens a
census — external commands cannot invoke shell functions); a refused inline naming the region's
function; an instance inside a loop body. Route instances are additionally filtered to nodes
REACHABLE from entry — see the finding in §7.

The meet tries Replace → Omit → Guard → Run, quantifying universally at each arm, with semantic
(not tag) equivalence: two routes both tagged `Replace` still meet to Run unless they reproduce the
same stand-in. `SharedConclusion::project` mints the public outcome and the settlement-facing act
TOGETHER from one conclusion; there is no `From<SharedOutcome> for SharedRegionAct` and the doc
says why.

## §5 — Battery inventory (cell → status)

`plan/tests/region.rs` (29 trials) + two analysis-tier cells. Census cells drive the REAL analyzer;
meet cells synthesize admissions (the site-conclusion bridge is stage 4's one-line match).

| §12 battery cell | test | status |
|---|---|---|
| mixed Replace/Run bodies | `a_mixed_body_decides_its_regions_independently` | green, target |
| agreeing twin calls | `agreeing_twin_calls_meet_to_one_replacement` | green, target |
| divergent-facts-one-guard | `interim_divergent_route_facts_run_rather_than_guarding` | green, INTERIM (named) |
| " (target) | `p_x_divergent_routes_share_one_parametric_guard` | **red-first xfail** |
| disagreeing-transformations Run | `differing_reproduced_statuses_run` | green, target |
| branch-join one-arm failure | `branch_arms_are_separate_regions_and_one_arms_failure_stays_there` | green, target |
| nested static calls | `the_wrapped_factoring_censuses_every_body_region` | green, target |
| call status consumed vs dead | `one_region_serves_a_status_consuming_call_and_a_bare_one` | green, target |
| stdout consumed on one route | folded into the cell above + `one_failing_route_forces_run_for_the_whole_region` | green, target |
| dynamic-call census poison | `a_dynamic_execution_construct_opens_every_census` (+ the exclusion twin) | green, target |
| EXPECTED-OPEN literal loop | `a_literal_loop_population_is_open_today` | green, CURRENT truth |
| " (target) | `p_x_loop_population_closes_over_literal_members` | **red-first xfail** |
| influence divergence | `one_influenced_route_influences_the_shared_decision` | green, target |
| whole-helper derivation | `a_wholly_replaceable_helper_is_all_replace_regions_and_no_call_decision` | green, target |

Pins this lane owns, each a test: `pin-no-singleton-special-case`
(`cardinality_one_falls_out_of_the_general_meet`) · `pin-open-route-runs`
(`an_open_population_runs_whatever_the_proofs_say`) · `pin-every-route-meets`
(`one_failing_route_forces_run_for_the_whole_region`) · `pin-common-replacement-observables`
(`differing_reproduced_statuses_run`) · `pin-errexit-rides-status-law`
(`analysis/tests/cfg.rs consumed_errexit_inside_a_spliced_body_matches_top_level`) ·
`pin-influence-joins-most` (`one_influenced_route_influences_the_shared_decision`) ·
`pin-guard-resolution-is-frame-live` (`divergent_live_guard_definitions_refuse_the_shared_guard`) ·
`pin-census-is-execution-not-scope`
(`the_census_counts_every_executing_invocation_whatever_was_checked`) ·
`pin-region-universe-excludes-dorc-lang` (`core::region` unit + `a_book_outside_the_universe_yields_no_regions`) ·
`pin-loop-population-open-until-proven` (`a_literal_loop_population_is_open_today` + the xfail) ·
`pin-definition-not-name` (`core::region` unit + `same_named_definitions_never_share_a_population`) ·
`pin-empty-function-world-parity` (`a_book_with_no_calls_has_no_regions` + zero corpus drift) ·
`pin-probe-site-identity-unchanged` (`route_identity_is_stable_and_carries_no_dispatch_dimension`) ·
`inv-closed-route-set-never-empty` (`no_proofs_at_all_is_run_never_a_vacuous_yes` +
`proofs_that_do_not_cover_the_population_run`) · `req-census-admits-the-wrapped-book`
(`the_motivating_wrapped_factoring_is_admitted`).

Two xfail pins registered in `internal_tooling::xfail::PINS`, both `Horizon::Unscheduled` at
`end-of-r30` with reasons: `p-x-loop-population-closes-over-literal-members` (trigger: the
loop-propagation lane) and `p-x-divergent-routes-share-one-parametric-guard` (trigger: the `30L`
stage-4/5 settlement-and-render lane).

## §6 — Deviations, each OPEN, none self-endorsed

- **`dev-region-decision-is-not-plan-disposition`** — the brief asked the private conclusion to
  project "the public `Disposition`". It projects a region-level `SharedOutcome` instead.
  `Disposition::Replace` carries a `ReplaceLicense`, and the license a SHARED replacement must
  carry is the cross-instance witness `pin-shared-witness-spans-instances` mints in stage 4;
  putting one route's per-call license into a region-level `Disposition` would be exactly the
  per-call-witness substitution that pin forbids. The act half is likewise a region-level
  `SharedRegionAct` rather than `world::EffectiveAct`, because minting `NoMutation(Replaced(..))`
  requires `ReplacementDeathProof::mint`, which is lexically fenced to ONE caller in the settlement
  path. The projects-twice DISCIPLINE is honoured verbatim (one conclusion, two projections, no
  outcome→act conversion). Fold-review should confirm the substitution or direct the unification.
- **`dev-route-conclusion-is-a-shadow-vocabulary`** — `RouteAdmission::project` takes a public
  `RouteConclusion` rather than the private `DecisionConclusion`, because a `pub` seat cannot name a
  private type and an unused `pub(crate)` seat is dead code the workspace policy refuses. Stage 4
  writes the bridge as a total match at the site seat. The doc says the ONE sanctioned producer is
  that match; nothing else may populate it. This is one vocabulary plus one future bridge, not
  per-seat hand population — but it IS a second name for the same four arms, so it is flagged.
- **`dev-elision-region-sited-in-core`** — `ElisionRegion`/`IterationSlot`/`RegionUniverse` live in
  `core`, not `plan`, because stage 4's `req-wall-narrative-gains-region-operand` puts a region
  operand on an `aid`-plane narrative and `aid` cannot see `plan`. `RouteInstance` stays in `plan`
  (it carries a `CfgNodeId`). If the narrative operand ends up needing the full instance, that is a
  siting question stage 4 must re-open.
- **`dev-omit-requires-equal-controllers`** — the shared Omit arm requires every route's controller
  `AstId` to be equal. Clones of one body share their controller's `AstId` by construction, so this
  is near-free where it is right and refuses where a shared render would have no single provenance.
  It is a conservative reading of §5's "all routes prove source-level Omit"; a widening would need a
  render-side answer to "whose controller does the region's provenance name".
- **`dev-census-openers-are-incomplete`** — the census's opener detection covers dynamic execution,
  refused inlines, and loop bodies. `rul-call-census-must-be-closed`'s list also names unresolved
  source/load, unresolved callback/alias, and trap-or-string execution naming the function. Those
  signals live at the driver edge (`funcenv::unresolvable_loads`, the alias vector) and the census
  takes no driver input yet, so they are NOT yet openers. Stage 4 wires the driver and must add
  them; until then a book carrying an unresolved load could census Closed where it should be Open.
  **This is the one gap in this lane with a wrong-direction failure mode**, and it is contained
  only by nothing consuming these decisions yet.
- **`dev-guard-identity-is-canonical-bytes`** — `pin-guard-resolution-is-frame-live` is enforced by
  comparing `GuardInsert::canonical()` (emitted name + invocation + preamble bytes) rather than the
  guard's `DefinitionId`. `GuardInsert::defining_span` is documented display-tier and
  decision-inert, so keying a licence decision on it would promote a display value. Two instances
  resolving different live definitions ship different preamble bytes and compare unequal; two
  resolving byte-identical bodies are one definition under the artifact's own content-dedup rule.
  Believed correct, flagged because it is a substitution of mechanism.
- **`dev-loom-fixtures-refreshed`** — the budget re-size invalidated three `crates/aid/tests/`
  defining cases (`cfg-inline-refused-{depth,per-call,per-book}-budget`); their fixtures were
  rescaled and their transcripts refreshed through the sanctioned `DORC_LOOM_DUMP` loop. No
  `dorc-loom publish` was run and no prose register was touched — only fixture bytes and
  engine-rendered transcript bytes moved. Outside `crates/cli/tests`, so the "zero golden drift" and
  "NO bless" contract is intact, but the conductor should confirm the refresh path was the intended
  one.

## §7 — Findings

- **`fnd-nested-flatten-double-counted`** — see §2. Real, measured, fixed in this lane.
- **`fnd-spliced-internal-flags-detached-bodies`** — `Cfg::is_spliced_internal` is TRUE for a
  funcdef's own detached body lowering, not only for a call's spliced copy. Both merely mean "not a
  plan leaf". The census filters by reachability-from-entry, and the errexit pin filters by
  execution ownership (`ExecutionOwner::Leaf(call)` with `call != id`), which is the precise
  discriminator. This cost a false red before it was understood, and the flag's doc comment invites
  the misreading. Proposed sharpening in §9.
- **`fnd-depth2-diagnostic-name-is-now-stale`** — the `depth-2-positional-unthreaded` code refuses
  ANY nested call whose argument references a positional; the refusal is nesting-keyed, not
  depth-2-keyed. With `MAX_DEPTH = 4` its slug and its authored prose ("down a **second** level of
  function calls") are inaccurate. **Builders author no prose**; exact proposed replacement text is
  in §9 and it needs a conductor/human act. The refusal's SEMANTICS are unchanged.
- Errexit inside spliced bodies is already CORRECT: a spliced body command under `set -e` carries
  `StatusRelaxable` exactly as at top level, and `|| true` releases it identically. No optimistic
  exemption exists. Pinned.

## §8 — `tc-*` flags UP (never resolved here)

- **`tc-region-decision-license-siting`** — where does a shared Replace's LICENSE live, and does
  `SharedOutcome` collapse into `Disposition` when the cross-instance witness exists? This is the
  same question as `dev-region-decision-is-not-plan-disposition` and it is cross-cutting between
  `plan::region`, `plan::settle`, and `spine`.
- **`tc-census-driver-inputs`** — the census currently takes only `(ast, cfg, diags, universe,
  book)`. The remaining openers, and the universe itself, come from the driver. Deciding what the
  census is allowed to be handed is a licensure-surface decision (an opener the census does not see
  is a population wrongly Closed), not a plumbing choice.
- **`tc-splice-budget-is-licensure-review-tier`** — the budgets are now documented as
  licence-widening. Should a future change to them carry the same review posture as a funcenv
  precision change (`28Q` §1's winner-shifting rider)? I believe yes; it is not mine to rule.

## §9 — Proposed steering text (NOT applied — conductor's)

1. `spike/crates/analysis/CLAUDE.md`, **Direction** section, append:

   > - **splice-budgets-are-licensure-not-perf** (`30L:req-census-admits-the-wrapped-book`) — the
   >   `cfg::inline_budget` constants are a LICENCE surface, not a performance knob: an un-spliced
   >   call is `Opaque` ⇒ ⊤ ⇒ a poison wall, so raising a budget makes mutations visible that were
   >   hidden behind a wall and makes downstream elisions available. They were re-sized against
   >   measured corpus-shaped strawmen (a 15-command wrapped book is 63 AST nodes; the inherited
   >   per-call 64 admitted exactly one such book and nothing larger), and each carries its
   >   measurement in its own doc comment. Move one deliberately, with the corpus re-measured, never
   >   to make a fixture fit.

2. `spike/crates/analysis/CLAUDE.md`, **Law — the dangers**, append:

   > - **spliced-internal-covers-detached-bodies** — `Cfg::is_spliced_internal` is true for a
   >   funcdef's OWN detached body lowering as well as for a call's spliced copy; both mean only
   >   "not a plan leaf". A consumer that needs "this is an execution" must ALSO check
   >   reachability-from-entry, or `ExecutionOwner::Leaf(call)` with `call != node`, which is the
   >   exact discriminator. Reading a detached body's vacuous-⊥ state as ambient is a wrong-elision
   >   (`vacuous-entry-fold`), and the flag alone does not stop you.

3. `spike/crates/plan/CLAUDE.md`, **Direction**, append:

   > - **region-decisions-meet-universally** (`plans/30L` §5) — `plan::region` groups per-instance
   >   answers by the authored span they would edit and meets them: Replace → Omit → Guard → Run,
   >   universally quantified, semantic (never tag-level) equivalence, biased to Run. Nothing
   >   branches on route-set cardinality; an `Open` population runs without consulting a proof; and
   >   a proof list that does not correspond exactly to the census's population runs. The decisions
   >   are computed and consumed by nothing until the settlement stage lands.

4. The `depth-2-positional-unthreaded` catalog entry needs a conductor/human prose act: the slug's
   "depth-2" and the message's "down a **second** level of function calls" are inaccurate now that
   the depth budget is 4 — the refusal fires for ANY nested call whose argument references a
   positional. Proposed replacement message, for a human to accept or rewrite:
   *"Dorc did not inline the call to `{{name}}`: its argument passes a positional (`$1`..`$9`/`$#`)
   into a nested function call, which Dorc does not model, so the call runs as an ordinary
   unmodeled command."* A slug rename to `nested-positional-unthreaded` is the cleaner fix and is
   free pre-publication (`rul-strawman-formats-no-compat`), but it moves the catalog lock and the
   defining case, so it is squarely a conductor act.

## §10 — Verification

- Comment budget: **29** added bare inline `//` narration lines (the briefed counting command
  reports **102**, of which 69 are `//!` module doc-comments and 4 are structural banners — the
  same billing miscalibration `30N` §3 `adj-endorse-comment-budget-overage` records; the
  `///`/`//!` doc-comments carrying each public item's why are required by style law).
- Corpus: `git diff 7f7cb359..HEAD --stat -- "spike/crates/cli/tests"` is **EMPTY**. Zero golden
  drift, no bless, no new e2e case (this lane's battery is unit/integration tier).
- `mise run both gate:full-quiet` at the final tip: **rc=0, both legs green** (Windows leg first;
  WSL leg preflight warm).
- `mise run test` at the final tip: **2452 passed, 0 failed, 2 skipped**.
- `mise run xfail:census`: 9 live pins, 1 reserved; the two new pins group under `end-of-r30`, no
  horizon expired.

## §11 — Handoff to the stages-4–6 lane

Read this section first; it is written for a builder who has not seen this lane.

1. **The bridge you write first.** `plan::region::RouteConclusion` is the region plane's input
   vocabulary. Write the total match from `plan::lib`'s private `DecisionConclusion` into it at the
   site seat (`site_conclusion`/`decide_site`), and delete nothing: `RouteAdmission::project` is
   already the sole mint and its doc names you as the one sanctioned producer.
2. **What the census still needs.** `dev-census-openers-are-incomplete` (§6) is the gap with a
   wrong-direction failure mode. Wire the driver's `funcenv::unresolvable_loads()` and the
   definition-vector signals in as census openers BEFORE any region decision reaches settlement. The
   `RegionUniverse` also has no producer yet — the cli edge must build it from the loaded source
   set, admitting exactly the files `oracle::marker::has_marker` says are NOT dorc-lang.
3. **What `SharedRegionDecision` gives you.** `contributing()` is the exact ordered population, in
   census order — that is what `pin-shared-witness-spans-instances` builds its cross-instance
   witness from. `act()` is `RetiresEveryInstance` / `MayMutateEveryInstance`; lowering it into
   per-instance `world::EffectiveAct` goes through the existing fenced mints
   (`ReplacementDeathProof::mint`, whose one-caller lexical fence you must not widen — extend it
   deliberately or route through it).
4. **Do not re-key anything.** `RouteInstance`'s `cfg_node` is the clone axis and `iteration` is the
   overlay axis; the propagation lane fills `IterationSlot::Member` and must not touch
   `ElisionRegion`, any witness, or `SiteId`. `IterationSlot::member()` already speaks `SiteId`'s
   numbering, pinned.
5. **The two red-first pins are yours to green** (or to re-horizon with a reason):
   `p-x-divergent-routes-share-one-parametric-guard` needs a diverged route to be able to ADMIT a
   guard at all and needs the guard's argv to be the SOURCE-level expression rather than each
   site's resolved operands. `p-x-loop-population-closes-over-literal-members` is the propagation
   lane's.
6. **Budgets.** They are now licence surface (§9 item 1). If a corpus case starts inlining that did
   not before, that is a real decision change, not drift.
