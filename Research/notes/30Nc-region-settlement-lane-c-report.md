# 30Nc — Lane C report: `plans/30L` stages 4–5 (settlement projection, render, why)

> Tier: builder lane report (Opus, worktree `agent-aa12db50a3fdc415b`, branch
> `ai/r30-region-settlement`, based on `ai/r30-conduct` at `f927fdcc`). Scope: `30L` §12 stages 4
> and 5, the census-opener completion (`30N:adj-census-openers-gap-is-lane-c-precondition`), and the
> drift enumeration. Stage 6 (the cross-platform corpus close and the `pin-loop-types-need-no-rekey`
> paper review) is NOT in this lane.
>
> HEADLINE, and it is the one number a fold reviewer should read first: the existing corpus drift is
> **ZERO**. 170/170 pre-existing e2e cases are byte-identical; `bless:dry` writes nothing and leaves
> a clean tree. Five NEW cases carry the behaviour. §7 is the enumeration the brief asked for, and
> it is empty by measurement rather than by assertion.

## §1 — The precondition: census openers, completed

`30L:rul-call-census-must-be-closed` names seven opener classes. Lane B built three (dynamic
execution, refused inlines, loop bodies) and left three — unresolved loads, the alias vector,
trap/string execution — as `30Nb:dev-census-openers-are-incomplete`, the one gap with a
wrong-direction failure mode.

All three are now wired, through ONE required value per `30N:rul-census-inputs-are-non-optional`:

```text
region::CensusOpeners::of(universe, unresolvable_loads, definition_vectors, string_execution)
region::census(ast, cfg, diags, openers, book)
```

`CensusOpeners` has no `Default`, no builder, and no optional field: a census cannot be constructed
without every opener, and a driver that acquires a new signal must visit that constructor to drop
it. The two genuinely-external signals are the ones defensive emission already reads at the same
edge (`dorc_oracle::closure::definition_vectors` · `funcenv::unresolvable_loads`) — the same
question, one layer out: can something outside the enumerated set change what a name means.

`region::StringExecutionSites::of_unit(ast)` is the third, minted by the driver. It is literal
command-WORD keyed (`trap`), never effect-keyed, which is the discrimination
`oracle/CLAUDE.md a-top-reject-is-not-a-definition-vector` records: a dynamic command position is
already a `DynamicExecution` ⊤-reject, and reading THAT as string execution would put every peeling
wrapper in the world into the trigger. Deliberately not narrowed to "names the function": deciding
which function a trap's action string reaches means parsing a string with no closed enumeration, and
an unenumerable happy path falls back to the defensive answer (`rul-happy-path-is-a-closed-set`).

`RegionUniverse` has its real producer at BOTH cli edges (`main.rs` and `world.rs`'s `WhyWorld`),
admitting exactly the loaded files `oracle::marker::has_marker` says are NOT dorc-lang.
`pin-region-universe-excludes-dorc-lang` is unchanged and still enforced at the mint.

**Battery, one cell per new opener class**, each asserting BOTH halves the brief demanded (the
population goes `Open` AND the region runs):
`an_unresolvable_load_opens_every_census_and_the_region_runs` ·
`a_definition_vector_opens_every_census_and_the_region_runs` ·
`a_trap_opens_every_census_and_the_region_runs`. Each also holds the quiet control beside it, so a
cell cannot pass by the census going Open for an unrelated reason.

## §2 — Stage 4 as built: the settlement seats

### The bridge (`30Nb` §11.1)

`DecisionConclusion::as_route` in `plan/src/lib.rs` — ONE total match, at the site seat, and the only
producer of a `region::RouteConclusion`. `RouteAdmission::project` remains the sole admission mint.

### The region round

`plan::settle::decide_regions`, inside the EXISTING settlement (`plan::settle_effective_world`;
`one-settlement-one-world` holds — there is no second settlement). Per `30L` §6's round shape:

1. the census is FROZEN in `SettleInputs.regions` beside the policy and the vouches;
2. per instance, `crate::decide_route` runs the ORDINARY site seat (`site_conclusion`) at the
   spliced body node, with that instance's own freshness;
3. proofs group by region and `region::decide_region` meets them universally;
4. `lower_shared_decision` mints the shared license and per-instance no-execution proofs, and ONLY
   the universally-agreed `Replace` arm reaches the proof mint;
5. those proofs enter the same grow-only ledger the site decisions do;
6. reach re-solves, records re-fold, and the loop repeats to quiescence;
7. the settled round writes `SpineRegionDecision` once, at quiescence.

### The self-suppressed solve (a substantive addition, flagged)

Sibling INSTANCES of one region wall each other along the ordinary sequence: `install_pkg nginx`'s
establish reaches `install_pkg curl`'s, so under the honest policy instance 2 is stale and the
universal meet could never agree. The region's own ATOMIC replacement is what removes those writes,
so its freshness is answered with the whole population silenced — the same fixed-point argument
`effect::self_reach_holds` already makes for an in-loop Members site, one level up.

`world::solve_reaching_walls` therefore takes a suppression SET rather than one node, and the
suppressed answer is read ONLY beside its own certification (`30Mb:fnd-members-floor-is-a-sentinel`;
BOTH the window's and the solo's floor the instance). Only a plural population pays for the extra
solve — a lone instance is never in its own in-state.

### The license mint (`30N:rul-license-mints-at-settlement-from-shared-conclusion`)

`ReplaceLicense::prove_shared_region_replaceable` (`LicenseVia::SharedRegion`) mints at the
settlement seat FROM the private `SharedConclusion` plus the cross-instance witness. `decide_region`
retains the private conclusion so the lowering reads the PREMISE and never the public
`SharedOutcome` (`pin-no-outcome-as-generator`); there is still no `From<SharedOutcome>` anywhere.

`pin-shared-witness-spans-instances` rides the EXISTING fence: the witness is
`AllEstablishesVouched::mint` over an `AggregateEstablishes` built from the exact ordered union of
every contributing instance's `(site, cell)`, identity- and cardinality-matched. A per-call witness
is unspellable — pinned by `a_shared_region_license_spans_every_contributing_instance`, which shows
one instance's vouch REFUSING a two-instance population.

`ReplacementDeathProof::mint`'s one-caller lexical fence is UNWIDENED: the region lowering routes
through the existing `settle::replacement_death` helper in the same file, and
`replacement_death_mint_has_exactly_one_caller` still passes unmodified.

### Effective reach

`world::effective_invalidators` gains one conjunct: an invalidator is effective iff neither the node
itself NOR its execution owner is proven un-runnable. The owner check is unchanged; the node check
is what lets a region's atomic edit retire each instance's mutation while the CALL that governs it
still runs. The two are different claims and neither substitutes for the other.

### The two §6 residue riders

- **`req-backings-freeze-at-probe-boundary`** — `settle_effective_world` captures round 1's
  `fact_backings` and every later round consumes that same account. Re-derived per round the backing
  would drift as the ledger erases, while the policy beside it is frozen.
- **`req-wall-narrative-gains-region-operand`** — `CollapseKind::WallFormation` gains
  `region: Option<ElisionRegion>`, threaded from the act's own `WallId` node through the census.
  STRICTLY operand-only: the population and the `accounts_survival` gate are byte-for-byte what they
  were, because whether a wall narrates at all is `30M` §3's owed human ruling and this rider has to
  be correct under either answer. Today every record the mint schedule produces carries `None`;
  `a_wall_names_the_authored_region_it_stands_in_and_nothing_else` pins both halves. See the `tc-`
  flag in §9.

### Spine

`SpineRegionDecision<P>` — a new species, `CensusArm::New`, keyed by `ElisionRegion` rather than
`SiteId`. `DecidePlane` gains `type RegionDecision` beside `Decision` rather than reusing it: the two
key by different identities, and a seam that let one be handed where the other was expected would
make today's coincidence (both instantiate to `Disposition`) load-bearing.

**The durable HARD STOP was honoured and never approached.** The species sits in the transitory arm;
no `DurableView` names it; `whylog.rs` is untouched; nothing in the replay intake reads it. The
census test's arm counts move from 11 to 12 in the New arm, and the durable arm stays at four.

`routes: Account<RegionRoute>` carries the attribution (`{ invocation: SiteId, ast: AstId }`),
capped like every operand account. Influence: the region decision carries the JOINED grade
(`30L` §4.4) via `decide_region`'s existing find-first-influence, and every contributing route stays
available on the decision. NO new grade-stamping behaviour was added — the Spine grade boundary
(`30M` §4.1) is untouched, and `SpineRegionDecision.grade` is stamped by `Spine::minted_at` exactly
as every other species is.

### Guards, and the divergent-instances valve

`p-x-divergent-routes-share-one-parametric-guard` is GREEN through the real path, and the pin is
RETIRED from `internal_tooling::xfail::PINS`. What it needed, per the lane-B handoff, was two things:

1. **A route must be able to ADMIT a guard it did not itself conclude.** `RouteAdmission::project`
   now takes the guard candidate BESIDE the conclusion, which is what makes the admission the
   PRODUCT `30L:rul-every-property-meets-universally` describes. Read only each route's preferred
   answer and a converged-and-fresh route (which would Replace) and a diverged sibling (which would
   Run) meet to Run — losing exactly the value §4.5 exists to recover.
2. **The guard's argv must be the SOURCE-level expression.** `crate::source_argv` slices the
   region's authored words after the command name, and `GuardLicense::mint_for_shared_region`
   re-spells the vouch's invocation with them. `install -y "$1"` re-binds per invocation inside sh;
   a per-call literal would install a check about the wrong operand into shared source.

`SharedGuard`'s identity is the canonical bytes ALONE — emitted name, source-level invocation,
preamble — which is `pin-guard-resolution-is-frame-live` exactly (two instances resolving different
live definitions ship different preamble bytes and refuse), and which is what lets one region serve
two operands. `guards-mint-no-values` is untouched, no dispatch is generated, and authored-oracle
bytes still precede untouched originals.

## §3 — Stage 5 as built: render and why

`rul-edit-authored-definition-once` is the render: a `RegionStep` carries the authored span and the
edit lands there, once, through the SAME `collect_edits`/`emit_span_edits` machinery a leaf's does.
Definitions stay in place, calls stay calls, no body is cloned, no name is renamed, no dispatch is
emitted. The measured artifact for the mixed case:

```sh
main() {
   true   # dorc: elided [apt-get install -y nginx] (already converged / dead branch)
   hork tune-packages
   ( apt_get__is_converged install -y curl ) || apt-get install -y curl   # dorc: guard [...]
}

main "$@"
```

`rul-attention-honesty`: every call line that may execute is visible and untouched; a Run region's
authored bytes ship verbatim; a Guard is visible; only universally-proved Replace/Omit saves
attention.

`pin-whole-helper-derived-only` has two halves and both are built. The DERIVATION is unchanged
(`prove_inline_replaceable` still demands every body establish converged plus the call's own
consumed observables). The RENDER half is `Plan::live_regions`: when every invocation of a
definition is itself neutralised, its body executes on no route, and §8's ruling is that the inert
definition keeps its AUTHORED TEXT. Conservative in exactly one direction — a TRUNCATED route
account cannot answer "every invocation", so it keeps its edit, because dropping an edit whose
region can still execute would leave a no-execution proof resting on bytes the artifact still runs.

`dorc why` is bidirectional (`30L` §9). A region is an addressable decision of its own
(`dorc why book.sh:<body line>` answers for the edit a reader can SEE there, marked universal over
its contributing invocations); a call names the shared edits it executes. Two new arrangement
components carry it, hand-seeded `words: None` and rendering `[unwritten: <slug>]` —
`why-reason-region-universal-over` and `why-reason-call-executes-shared-regions`. Builders author
zero user-facing prose; the words are a conductor/human act against a registered row. Proposed text
is in §10.

## §4 — Battery flip inventory

| cell | status |
|---|---|
| `p-x-divergent-routes-share-one-parametric-guard` | **GREEN through the real path; pin RETIRED**, test renamed `divergent_routes_share_one_parametric_guard` |
| `interim_divergent_route_facts_run_rather_than_guarding` | RETIRED and replaced by `divergent_route_facts_with_no_shared_guard_run` — the interim asserted "no guard exists"; the surviving cell asserts the floor that still holds (a route admitting no guard cannot be carried by a sibling that does) |
| `p-x-loop-population-closes-over-literal-members` | **still RED**, unchanged, `end-of-r30`, propagation lane's |
| every other lane-B cell | green, unchanged in meaning; call sites updated for the two-input `RouteAdmission::project` |

New in `plan/tests/region.rs` (32 trials, was 29): the three opener cells above.

New elsewhere:
- `plan/src/lib.rs` `a_shared_region_license_spans_every_contributing_instance` —
  `pin-shared-witness-spans-instances`.
- `plan/src/settle.rs` `only_the_universally_agreed_arm_retires_anything` —
  `pin-shared-edit-before-erasure` / `inv-no-posthoc-shared-demotion`, as a locality scan over the
  lowering seat (the property is about WHERE a proof can be minted, which no value-level test
  reaches; same shape as the file's existing `a_provisional_round_names_no_spine_setter`).
- `plan/src/settle.rs` `a_wall_names_the_authored_region_it_stands_in_and_nothing_else` —
  `req-wall-narrative-gains-region-operand`.
- `plan/src/certifier_trip.rs` `a_real_trip_evicts_a_shared_region_elision_too` — the `30L` §10
  exclusion "cap/failure takes Run", at the region grain.
- `cli/tests/region_artifacts.rs` (new target, 4 trials) —
  `no_region_artifact_carries_a_cloned_or_renamed_helper` (`pin-no-generated-specialization`) ·
  `a_shared_guard_carries_the_source_level_argv_not_a_resolved_operand`
  (`rul-edit-authored-definition-once`) · `a_wholly_elided_helpers_body_ships_verbatim`
  (`pin-whole-helper-derived-only`) · `a_why_report_walks_from_a_region_to_its_invocations_and_back`
  (`30L` §9).

## §5 — Certification

All shared decisions certify. Three seats hold it:

1. every instance's freshness is floored by BOTH the window's certification and (where one exists)
   the self-suppressed solo's;
2. a tripped run demotes every region Replace/Omit to Run through `demote_on_trip`, on the same
   syntactic occupancy-1 census a guard stands on, narrating against the contributing invocations;
3. no new plan-producing path was added, so `every_plan_producer_spends_its_certifier_trip`'s
   two-way roster is unchanged and still green.

## §6 — New e2e cases

Five, all dir-form, each carrying its invariant argument as a comment header in its own `book.sh`
(the argument therefore round-trips through the artifact, where a reader of the case meets it).

| case | shape | measured outcome |
|---|---|---|
| `region30-mixed-body-splits-the-decision` | `main() { converge; hork; converge }` + `main "$@"` | nginx region REPLACES, `hork` RUNS, curl region GUARDS (stale past hork's wall), the call runs. Three answers inside one definition. |
| `region30-twin-calls-share-one-region` | `install_pkg() { apt-get install -y "$1" }` ×2, both converged | both calls elide across settlement rounds; the inert definition ships verbatim |
| `region30-drifted-route-guards-the-shared-region` | same, nginx holds / curl absent | ONE parametric guard `( apt_get__is_converged install -y "$1" ) \|\| apt-get install -y "$1"` at the authored region; nginx's call elides, curl's runs and falls through |
| `region30-unmeasured-route-runs-the-shared-region` | same, nginx holds / curl cant-tell | the shared region RUNS — an UNKNOWN route admits no guard, so one route's convergence buys the region nothing |
| `region30-whole-helper-stays-authored-text` | `main() { converge; converge }` + `main "$@"`, all converged | the CALL elides; the definition keeps its authored bytes and acquires no stand-in |

The brief asked for a "drifted-day case where one route's staleness forces the shared region to
Run". It SPLIT into two, and the split is a finding rather than a substitution: a DIVERGED route
still admits the parametric guard (that is §4.5 working, and it is the more valuable outcome), while
an UNMEASURED route admits nothing and forces the Run. Both cells are committed.

## §7 — The drifted-case enumeration

**The enumeration is EMPTY.** Measured three ways at the final tip:

- `mise run test:e2e` — 175 passed / 0 failed (170 pre-existing + the 5 new);
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`, and `git status --porcelain`
  clean afterwards, so no golden would have been rewritten;
- `mise run test:looms` — 279 passed, no transcript moved.

The behaviour DID flip once, mid-lane, and the flip was measured and then repaired rather than
blessed. Recording it, because it is the one thing a fold reviewer should re-derive:

**`fnd-region-guard-economics-are-a-population-property`.** The first cut relaxed the region-tier
guard's `Converged` conjunct outright. That is safe (a guard falls through to the authored bytes
whatever the check says) but it is an ECONOMICS regression, and it drifted exactly one corpus case:
`aggregate30-inline-verdict-primacy`, whose two body establishes are both DIVERGED, started emitting
two guards whose checks are known to fail at every invocation — paying the check tax for nothing.
`jc-mint-policy m-a` ("a guard at a predicted-change site buys nothing") is a per-SITE reading, and
the region tier must not undercut it. The repair is that the guard's economics are a property of the
POPULATION, not of any route: candidates drop before the meet unless SOME route measured Converged.
With that, `30L` §4.5's actual licence — divergent instances — is what is served, and the case is
byte-identical again. Two further consequences: the UNMEASURED world stays byte-identical (every
cell is `Unknown`, so no region guards and no book acquires a preamble it did not have), and
`Unknown` refuses independently as the unsure direction (`inv-kfail`).

## §8 — Deviations, each OPEN, none self-endorsed

- **`dev-shared-guard-identity-drops-the-cell`** — lane B's `SharedGuard` carried a `FactKey` and
  compared it. That refuses exactly the case the parametric guard exists for: `install_pkg nginx`
  and `install_pkg curl` re-verify DIFFERENT cells through ONE authored region. The cell is dropped
  from the shared identity (which is now the canonical bytes alone) and stays on
  `RouteConclusion::Guard`, where it is that route's own truth. The safety chain is unchanged: every
  route has its OWN reached vouch over its OWN argv, so every operand the closed census enumerates
  passed the author's argparse.
- **`dev-region-guard-relaxes-the-converged-conjunct`** — `GuardLicense::mint_for_shared_region` is
  a second, region-tier-only mint. The per-site mint is untouched. The vouch demand and the
  consumption gate at ⊤ are identical; `Converged` relaxes to "a DEFINITE verdict", and the
  population gate above adds "some route converged". Believed correct, flagged because it is a mint
  policy and a mint policy is licence-review-tier.
- **`dev-route-admission-is-a-two-input-product`** — `RouteAdmission::project(conclusion, guard)`.
  Still ONE sole mint, but it now takes a second input the site seat computes beside the conclusion.
  Argued in §2; flagged because lane B's doc named the conclusion as the only source.
- **`dev-shared-omit-lowers-to-nothing`** — a region-tier `Omit` mints no ledger proof and no render
  edit. The arm is unreachable today (the fold's statuses key by the leaves it classified, and a body
  site is not one), and if it ever fires the region renders and retires nothing — the run floor,
  never a silent `:`. Named residue rather than a gap; building it needs a `DeadBranchProof` per
  instance and a controller the artifact really neutralises.
- **`dev-self-suppressed-solve-per-region`** — one extra certified solve per PLURAL closed region
  per round. Argued from the Members precedent and floored by its own certification; flagged because
  it is a new licensing surface and because it costs solves (bounded by the closed multi-instance
  region count, which is 0 or 1 across the whole corpus today).
- **`dev-live-regions-suppresses-inert-edits`** — the render drops a region edit when every
  contributing invocation is neutralised (§3). Justified by §8's ruling; flagged because it is a
  cross-identity render rule and because its conservative direction had to be chosen deliberately.
- **`dev-wall-region-operand-is-inert-today`** — see the `tc-` flag below.
- **`dev-dual-rail-gains-a-region-rail`** — `emit_debug_argv` emits a `region <call-leaf> <verb>
  <resolved argv>` line per spliced body site, and gate-6's `dual_rail_judge` reads it as a second
  ledger rail. This closes a PRE-EXISTING hole as well: an eliding inlined CALL licensed its body's
  mutations, and no ledger line said so, so any case reaching that shape would have read as an
  unattributed bare-only run. No existing case reached it; my new ones do.
- **`dev-new-e2e-goldens-minted-by-scoped-bless`** — the brief forbids golden re-blessing
  (orchestrator-only) and asks for new cases "with goldens you write". I minted the FIVE new cases'
  goldens with `mise run bless -- region30`, which rides the trial filter
  (`bless-honours-the-trial-filter`). Nothing was re-blessed: the five cases had no prior bytes, and
  `git status` after each run showed only `crates/cli/tests/region30-*`. Reported rather than
  assumed acceptable — hand-framing the records stream would have been the alternative, and I judged
  a mechanically-minted golden over a hand-framed one the lower-risk choice.
- **`dev-plan-summary-tally-stays-leaf-granular`** — `Plan::disposition_counts` still counts leaves
  only, so `dorc: plan-summary sites=N elide=…` does not see region decisions. Widening it changes
  what the WRITTEN word "sites" means under prose nobody re-authored. See the `tc-` flag.
- **`dev-arrangement-rows-hand-seeded-unwritten`** — two `words: None` rows hand-seeded into
  `arrangement_lock.rs`. Sanctioned by `aid/CLAUDE.md arrangement-lock-is-generated-too`, and the
  byte-identity fixpoint gate confirms the seed is a generator fixpoint.
- **`dev-effective-invalidators-gains-a-node-check`** — `world::effective_invalidators` now also
  asks whether the node ITSELF is proven un-runnable. A no-op for every pre-existing proof species
  (nothing minted one at a non-owner node); it is what carries the region's atomic retirement.

## §9 — `tc-*` judgment calls, flagged UP

- **`tc-wall-region-operand-population`** — the operand is built and truthful, and today it is
  `None` on every record, because the mint schedule was left exactly as-built per the brief. Whether
  the population should widen to non-leaf walls is `30M` §3's ratify-or-mint question's immediate
  neighbour, and it is what would make the operand non-vacuous. Not mine.
- **`tc-region-guard-economics-seat`** — "some route converged" is the population gate I chose. A
  sharper one would consider only routes whose CALL still executes: in
  `region30-drifted-route-guards-the-shared-region` the converged route's call independently elides,
  so the guard it justifies only ever runs for the diverged operand. Correct and safe as built,
  slightly over-generous on check tax. The sharper rule needs an intra-round ordering between call
  and region decisions that I did not want to introduce unilaterally.
- **`tc-plan-summary-counts-regions`** — should the push summary tally region decisions? It is a
  one-line change to `disposition_counts` and a semantic change to written prose.
- **`tc-region-tier-omit`** — build the shared-Omit lowering, or rule it excluded like the
  dorc-lang interiors are? Today it is neither: unreachable, and floored to Run if reached.
- **`tc-shared-guard-cell-identity`** — ratify dropping the cell from `SharedGuard`
  (`dev-shared-guard-identity-drops-the-cell`). It is the change that makes the parametric guard
  possible at all, so it wants an explicit look.
- **`tc-region-decision-influence-is-first-not-joined`** — `decide_region` carries the FIRST
  influenced route's grade, which under v0's positional-global flip is every route's grade, so the
  join is currently trivial. If per-record gradation ever lands (`306b` §1c), this seat needs a real
  join rather than a find-first.

## §10 — Proposed steering text (NOT applied — conductor's)

1. `spike/crates/plan/CLAUDE.md`, **Direction**, REPLACING the last sentence of the existing
   `region-decisions-meet-universally` bullet ("The decisions are computed and consumed by nothing
   until the settlement stage lands."):

   > The decisions are consumed by `plan::settle`, which is where the shared license mints.

2. `spike/crates/plan/CLAUDE.md`, **Direction**, append:

   > - **shared-edit-before-erasure** (`plans/30L` §6) — a region's per-instance no-execution proofs
   >   are minted at ONE seat (`settle::lower_shared_decision`'s `Replace` arm) and only after the
   >   universal meet agreed. Nothing may mint one per instance ahead of that: the ledger is
   >   grow-only and has no retraction, so a later Run meet would have to re-introduce a wall it had
   >   already retired for a mutation the artifact still executes. The witness the license carries is
   >   the exact ORDERED union of every contributing instance's establish
   >   (`pin-shared-witness-spans-instances`); a per-call witness never substitutes, and
   >   `AllEstablishesVouched::mint`'s identity/cardinality match is what makes that unspellable.
   >   Region freshness reads a SELF-SUPPRESSED solve over the whole population — the sibling
   >   instances of one region wall each other, and the region's own atomic replacement is what
   >   removes them (`effect::self_reach_holds`'s argument, one level up) — and that second answer is
   >   read only beside its OWN certification.

3. `spike/crates/plan/CLAUDE.md`, **Law — render**, append:

   > - **no-specialized-shell** (`30L:rul-edit-authored-definition-once` ·
   >   `pin-no-generated-specialization`) — a shared region's edit lands ONCE, at the authored
   >   function-body span; definitions stay in place and calls stay calls. Never a per-call
   >   specialized body, a cloned or renamed helper, or generated argument dispatch — there must
   >   always be one authored line, by one human, answerable for anything that runs. A shared GUARD
   >   therefore carries the region's SOURCE-level argv (`install -y "$1"`), never a site's resolved
   >   operands: positionals re-bind per invocation inside sh, and a per-call literal installed into
   >   shared source is a check about the wrong operand at every other invocation. Whole-helper
   >   elision stays DERIVED: when every invocation is itself neutralised the body executes on no
   >   route, so `Plan::live_regions` drops the edit and the inert definition keeps its authored
   >   bytes — conservatively, since a capped route account cannot answer "every invocation".

4. `spike/crates/cli/CLAUDE.md`, **Law**, append:

   > - **region-openers-are-demanded-not-defaulted** (`30N:rul-census-inputs-are-non-optional`) — the
   >   elision-region census is handed `region::CensusOpeners`, whose constructor requires EVERY
   >   opener signal the census cannot see for itself: `funcenv::unresolvable_loads`, the definition
   >   vectors, and the string-execution sites. An opener the census does not see is a population
   >   wrongly CLOSED, which is a wrong-elision one abstraction level up, so the shape is a required
   >   constructor rather than a defaulted parameter and a driver acquiring a new signal must visit
   >   that seat to drop it. Both drivers build it — the binary and `WhyWorld` — from the same frozen
   >   inputs, for the reason `one-definition-table-two-drivers` gives.

5. `spike/CLAUDE.md`, **Invariants — analysis boundaries**, amending `inv-leaf-seam` (the amendment
   `30L` §2 says lands with this stage). Append to that bullet:

   > Execution leaves and ELISION REGIONS are distinct identities (`30L:rul-two-identities-never-
   > conflated`): a `LeafId` is one execution, one probe record, one `Step`; an
   > `ElisionRegion` is one authored EDIT that many executions share. The Step-level map stays
   > injective and `inv-site-keyed-results` is unweakened — a region owns no leaf and mints none.

6. `ANALYZER-NEEDS.md` — rows for invocation-instance identity, elision-region identity, closed route
   populations, universal region decisions, and the iteration dimension (`30L` §13). Not drafted
   here; the register is conductor-tier.

7. `FORFEITS.md` — `30L` §13 asks for a row retiring "partial function-body elision remains
   forfeited" and adding "loop populations Open-until-propagation". The first half is now
   discharged; the second is the standing `p-x-loop-population-closes-over-literal-members` pin.

## §11 — Verification

- Comment budget: **27** added inline `//` lines (budget 30), counted with the briefed command
  `git diff f927fdcc..HEAD -- "*.rs" | grep -cE "^\+\s*//($|[^/])"` minus its 8 `//!` module-doc
  lines. The rationale that used to sit inline now lives in the `///` docs of the items it explains,
  which is where the style law wants it anyway.
- `mise run check` — green (Windows).
- `cargo clippy --workspace --all-targets` — green (Windows), and the arc-tier gate agrees.
- `mise run test` — **2468 passed, 0 failed, 2 skipped** (Windows);
  **2464 passed, 0 failed, 2 skipped** (WSL; the difference is pre-existing `cfg`-gated coverage).
- `mise run test:e2e` — **175/175**, of which 170 are the pre-existing corpus, byte-identical.
- `mise run test:looms` — **279/279**.
- `mise run both gate:full-quiet` — **rc=0, BOTH legs** (Windows leg first). No leg reported a
  drifted case, because there are none.
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`; clean tree afterwards.
- `mise run xfail:census` — 8 live pins, 1 reserved (was 9 live). No horizon expired.
  `p-x-loop-population-closes-over-literal-members` stays red at `end-of-r30`, as required.

## §12 — Handoff to the stage-6 close lane

1. **There is no drift to bless.** Stage 6's "corpus-wide golden flip" is a no-op as of this tip.
   What IS owed there is the cross-platform execution close over the five new cases (they already
   run under inert mocks on both legs) and the `pin-loop-types-need-no-rekey` paper review.
2. **For that paper review**, the identities to check against `30N` §2's `20S`-era commitments are
   unchanged from lane B: `RouteInstance.cfg_node` is the clone axis, `iteration` the overlay axis,
   `IterationSlot::member()` already speaks `SiteId`'s numbering. What stage 4 ADDED that the review
   should also check: `SpineRegionDecision.routes` is an `Account<RegionRoute>` keyed by invocation
   SITE, so a member route needs no new key; and the self-suppressed solve already takes a SET, so a
   closed member population suppresses exactly as a closed invocation population does.
3. **The one performance shape to watch** is the per-region self-suppressed solve. It is
   `O(plural closed regions × nodes × edges)` per round. The corpus reaches at most one such region;
   a genuinely wide book (twenty helpers each called twice) would pay twenty solves per round.
   Bounded and inside `perf-doctrine`, but it is the first thing to measure if a big fixture crawls.
4. **`tc-region-guard-economics-seat`** (§9) is the sharpest remaining precision item and the one
   most likely to change a golden when it lands.
5. **The two unwritten arrangement rows** (§3) are the prose act stage 6 or the conductor inherits.
   Proposed wording is deliberately absent — `error-authorship-tier` says builders author none.
