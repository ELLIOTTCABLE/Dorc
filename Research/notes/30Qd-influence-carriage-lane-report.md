# 30Qd — `lane-influence-carriage`: the MAP

> Tier: LLM-authored lane report (Opus MAP builder; seat
> `.claude/worktrees/agent-a23da68137754c338`, branch `ai/r30-lane-influence-map`, lineage
> `ai/r30-conduct-repair` @ `eae684b1`). Charter: `306b:influence-carriage-across-entities (née §10)`
> · `309:rul-spine-preserves-never-stamps` · `30L:rul-shared-influence-never-launders` ·
> `ANALYZER-NEEDS:an-host-influence-carriage`. This half MAPS and RULES NOTHING: every
> item below is a proposal for the conductor, and the EXECUTE half is a fresh builder.
> Grades: +SURE / ~SUSPECT / -GUESS / --WONDER.

## §0 — the honest headline, before the tables

**`fnd-the-conversion-changes-no-answers-at-v0`** (+SURE). Removing the object-global Spine
stamp does not change what any Spine record answers on the production driver. The stamp is fed
`Some(scoped_results.influence())` at `cli/src/main.rs` L1965 and
`Some(influence_after_reaching_for_host_bytes())` at `cli/src/world.rs` L484, and BOTH are
unconditional: `admit_controller_records` mints the phase on the admitted path,
`results::no_observation` mints it on the quiet path, and `replayed_records` mints it on the
replay path. Every production run's Spine is therefore host-influenced today, and every
per-object join this lane installs will still answer host-influenced, because every Spine record
is minted downstream of intake and — under `306b:rul-semantic-mints-join-influence`'s "every
contributing data **and control** input" — the control condition that reached the mint is itself
intake-gated (`record_durable_arm` is literally inside `let Some(records) = admitted_records`,
`main.rs` L2369).

So what the lane buys is not new answers. It is four things, and they should be sold as exactly
these:

1. **the seat moves** — each mint states its own account, so a NEW record species cannot
   silently inherit a run-wide scalar (today's 18 setters overwrite whatever a mint supplied);
2. **`untracked` becomes expressible and is USED** — an `Open`/non-corresponding region
   population and every unconverted seam say so, where today they spell `None` and read
   authored (`306b:rul-untracked-is-not-authored`);
3. **the genuinely pre-contact objects acquire accounts they never had** —
   `plan::placement::PlacementDecision`, `cli::artifact::Selection`, `plan::ImportEdit`, the four
   probe-side plans — which is where `30I:rul-load-decisions-are-authored-before-contact` [TYPED]
   finally has a type to live in;
4. **the per-input join is SPELLED at each seat**, so a later per-cell derivation is a change to
   one conjunct rather than a re-architecture.

**`fnd-per-route-difference-is-unreachable-at-v0`** (+SURE, and this is the load-bearing
negative). The brief asks what feeds per-route influence once the global scalar is gone. Traced:
the ONE `HostReported` mint is `plan::records::admit_unscoped_host_records` (`records.rs` L1123);
its payload converts through `Influenced::map` into `SiteResults` and exits via `into_read()`
as an `InfluencePhase` whose payload is `()`. **The phase carries no per-cell information at
all.** The only per-cell discriminator that exists today is the KEY SET of `facts_from_sites`'
`by_fact` map — "did an admitted record speak about this cell". That is genuinely per-site, but
it is not sufficient: a route's decision also consumes `freshness`, which reads
`ReachingWalls` over the decision-fed effective set, which is influenced whenever ANY site in
the window read a record. Deriving the account from the cell alone would therefore LAUNDER —
exactly what `30L:rul-shared-influence-never-launders` forbids. The conservative v0 answer is
that the world conjunct dominates and every post-settlement record stays influenced. The
capability is pinned RED (§map-7) rather than claimed.

## §map-1 — refreshed constructor census over `eae684b1`

Baseline check: `git diff --stat aabcc2d9..eae684b1 -- spike/crates/core` is EMPTY, so the scout
census's §1 (`core/src/influence.rs`, 279 lines) and §2 (`core/src/spine.rs`, 1453 lines, 16
species, `pub type Grade = Option<InfluencePhase>` at L145) hold verbatim (+SURE). `SpineSpecies`
is still `[Self; 16]`; `census_arm` is still 4 durable / 0 excluded / 12 new.

Conversion classes: **CONVERT** joins real per-input accounts this lane · **ADAPT** carries an
explicit `untracked` · **RESTRICT** carries a typed authored-before-contact-only account ·
**OUT** is not a stable semantic object.

### The 16 Spine species (`core/src/spine.rs`)

| species | takes influence today | bypass | binding rule | class |
|---|---|---|---|---|
| `SpineInvocation` L319 | N (`pub grade`, always written `None`, stamped) | every field `pub`, struct-literal only | `rul-spine-preserves-never-stamps` | CONVERT |
| `SpineRecordStream` L356 | N | ditto | ditto | CONVERT |
| `SpineDisposition` L375 | N | ditto | + `rul-consequential-sinks-require-influence` (license-bearing) | CONVERT |
| `SpineDigest` L395 | N | ditto | ditto | CONVERT |
| `SpineLoadDecision` L408 | N | ditto | + `30I:rul-load-decisions-are-authored-before-contact` | CONVERT (authored-side; see `tc-load-decisions-read-authored`) |
| `SpineSiteClassification` L442 | N | ditto | ditto | CONVERT |
| `SpineSolveCertification` L469 | N | ditto | ditto | CONVERT |
| `SpineVouch` L492 | N | ditto | ditto — **NOT MINTED** (`30F` §4.5) | CONVERT (representation only) |
| `SpineProbeShip` L507 | N | ditto | ditto | CONVERT |
| `SpineAdmission` L538 | N | ditto | ditto | CONVERT |
| `SpineObservation` L565 | N | ditto | **NOT MINTED** | CONVERT (representation only) |
| `SpineValidityRound` L584 | N | ditto | **NOT MINTED** | CONVERT (representation only) |
| `SpineSurvival` L595 | N | ditto | ditto | CONVERT |
| `SpineRenderDecision` L651 | N | ditto | `rul-projections-continue-influence-flow` | CONVERT |
| `SpineRegionDecision` L834 | N | ditto | + `30L:pin-influence-joins-most` | CONVERT (the threading fix, §map-3) |
| `SpineOutcome` L858 | N | ditto | **NOT MINTED** (`30Nd`) | CONVERT (representation only) |

All sixteen share one bypass: **every field on every one of them is `pub` and there is no smart
constructor**, so a caller may hand-build any record with any grade. That is the direct
violation of `306b:rul-semantic-mints-join-influence`'s "no public field … may manufacture
lesser influence", and sealing them is the largest mechanical row (§map-8 R2).

### `plan` — the license and decision plane

| constructor | site | takes influence | bypass | class |
|---|---|---|---|---|
| `ReplaceLicense::prove_replaceable` | `plan/src/lib.rs` L565 | N | none (private fields, sole mint) | CONVERT |
| `ReplaceLicense::prove_query_replaceable` | L629 | N | none | CONVERT |
| `ReplaceLicense::prove_members_replaceable` | L678 | N | none | CONVERT |
| `ReplaceLicense::prove_shared_region_replaceable` | L824 | N | none | CONVERT |
| `GuardLicense::mint` | L1496 | N | none | CONVERT |
| `GuardLicense::mint_for_shared_region` | L1542 | N | none | CONVERT |
| `Disposition::{Run,Replace,Omit,Guard}` | L2148 | N (enum, no account) | plain enum | OUT-as-enum / the account rides `SiteDecision` (see below) |
| `SiteDecision` (private) | L4467 | N | crate-private | CONVERT — the natural per-site join seat |
| `RouteDecision` (private) | L4600 | N | crate-private | CONVERT — the natural per-route join seat |
| `Plan::decided` | L5070 | N | sole constructor already | CONVERT (joins the records it projects) |
| `Step` L2188 / `RegionStep` L2263 | — | N | `pub` fields | OUT (projections of an account-bearing record; the account lives on `Plan`) |
| `RefusedEdit` L2282 | — | N | `pub` fields | OUT (a render-plane detail of an accounted `Plan`) |
| `ImportEdit` L2313 | — | N | plain enum | RESTRICT (settled pre-contact at `Selection`) |
| `SurvivalReport` | L2634 | N | private fields | OUT (instrumentation, digest-exempt) |
| `ProbePlan`/`ResolverPlan`/`ReachPlan`/`DerivationPlan` `render_sh` | L2977/3214/3280/3328 | N | — | RESTRICT (compiled pre-intake; FROZEN by `the-fixpoint-owns-the-rounds-and-builds-nothing-else`) |
| `Plan::render_sh` L5131 · `Plan::render_apply` L5465 | — | N | — | **no adapter needed** — their input is an account-bearing `Plan` (see lean-2, §map-10) |
| `project_plan` | `plan/src/spine.rs` L138 | N | — | CONVERT (the projection joins; `rul-projections-continue-influence-flow`) |
| `record_render_decisions` | `plan/src/spine.rs` L233 | N | 4× `grade: None` literals | CONVERT |
| `ProvisionalSiteDecision` L134 · `ProvisionalRegionDecision` L168 | `settle.rs` | **no field at all** | crate-private | CONVERT |
| `SettledEffectiveAnalysis::write_spine` L250 | `settle.rs` | N (`grade: None` ×3) | — | CONVERT |
| `record_survival` L289 | `settle.rs` | N (`grade: None`) | — | CONVERT |
| `RouteRegionProof::new` | `region.rs` L551 | **Y** — `Option<InfluencePhase>` | public mint | CONVERT (widen the parameter) |
| `SharedRegionDecision` L612 / `decide_region` L739 | `region.rs` | **Y** — the ONE real join | private fields | CONVERT (+ `Untracked` for `Open`) |
| `certifier_trip::demote_on_trip` L109 | | N — rewrites `record.decision` post-mint | `&mut` via `dispositions_mut` | CONVERT (lean-3, §map-10) |
| `attach_spine_probe_provenance` L4400 | `plan/src/lib.rs` | N — rewrites `record.decision` post-mint | `&mut` | OUT — see `fnd-provenance-attach-raises-nothing` below |

**`fnd-provenance-attach-raises-nothing`** (~SUSPECT). `attach_spine_probe_provenance` is a THIRD
post-construction Spine mutation beside the trip demotion. Under "the account is immutable" it
cannot raise a record's account — but it need not: what it attaches is the probe attribution for
the very fact the license already decided on, so a `Replace`/`Guard` record's account has already
absorbed that measurement at its mint, and the `Run`/`Omit` arms are no-ops. State this as a
property in the doc; do not build a re-join for it.

### What the four folded lanes added

| constructor | site | takes influence | class |
|---|---|---|---|
| `PlacementDecision::new` | `plan/src/placement.rs` L167 | N | RESTRICT — its own doc already says this lane converts it |
| `Placement`/`EmittedName`/`PlacementReason`/`DefinitionKey` | `placement.rs` L48/65/96/200 | N | OUT (operands of `PlacementDecision`) |
| `PlacedSources::{carried,uncarried}` | `placement.rs` L251/256 | N | RESTRICT (a map of `PlacementDecision`s) |
| `EmittedNames::mint` | `placement.rs` L328 | N | OUT (a name allocator, not a semantic object) |
| `ArtifactEmission::of` | `placement.rs` L367 | N | RESTRICT (a borrowed view over two accounted values) |
| `Selection` / `select` / `select_for_terminal_render` | `cli/src/artifact.rs` L827/930/894 | N | RESTRICT — structurally pre-contact by its own doc (L823-826) |
| `Selection::with_plan` → `ArtifactSet` | `artifact.rs` L873 | N | CONVERT — it binds pre-contact form to the plan's bytes, so it JOINS the plan's account |
| `RouteInstance`/`IterationSlot` member routes | `region.rs` L72, `core::region` | N | OUT (identities, not conclusions) |
| `ClosedRoutes`/`RoutePopulation`/`RegionCensus` | `region.rs` L118/139/152 | N | OUT (the census is pre-contact; its `Open`-ness feeds `Untracked` at `decide_region`) |
| `ResolvedHead`/`OperandAnswer`/`HavocCause`/`Explicitness`/`LoadHead` | `analysis/src/funcenv.rs` L2225/2208/2265/2253/2280 | N | **OUT** — see below |
| `Loadable`/`LoadProgram` | `analysis/src/load.rs` L202/222 | N | **OUT** — same reason |
| `SiteFrame` | `oracle/src/closure.rs` L226 | N | OUT (a lookup closure) |
| `NameUseCensus`/`LoopEvaluations`/`ScriptSpellings`/`Spelling` | `analysis/src/{nameuse,cfg,funcenv}.rs` | N | OUT (kernel dataflow values) |

**`rul-the-account-never-enters-compared-state`** (proposed, +SURE about the hazard). The load-plane
and CFG types above are OUT for one reason that must be stated as a rule rather than left as a
judgment: they are KERNEL values that participate in lattice `Eq` and fixpoint comparison
(`funcenv::analyze` re-solves under a mask; `inv-determinism`'s "semantic `Eq`" precondition).
Putting an account inside one would put a non-semantic field into compared state and risk exactly
the failure `309:law-spine-write-only-during-run` cites `CollapseNarrative`'s Eq-exclusion for.
They are additionally authored-before-contact by construction
(`analysis/CLAUDE.md funcenv-reads-source-literal-plane-only` admits only `ValueGrade::ProgramText`),
so nothing is lost. **The conversion boundary is the decision plane and outward, never inward.**

## §map-2 — the type design

### The account type, and its name

Proposed: `core::influence::InfluenceAccount`, a private-field newtype over a three-point
standing. It lives in `core::influence` (dependency-free, already the vocabulary home) rather
than `core::spine`, because projections in `plan` and `cli` mint accounts too and
`309:pin-spine-crate-home` is still open.

**Name argued** (this is an OVERLOAD, so the discipline demands it):

- `Grade` is squatted twice over — `core::Grade` is the claim-tier `Must`/`May` enum, imported
  UNQUALIFIED at `plan/src/lib.rs` L78 and used at L582/643/706/797/841/863, while
  `core::spine::Grade` (the influence alias) is used fully-qualified at L4324. The census called
  this headline risk #1; it is real, and the fix is to stop spelling influence as "grade" at all.
- `provenance` is squatted by `core::prov` (the derivation DAG/arena); `placement`, `layout` and
  `lift` are spent by `30P:rul-emission-is-the-umbrella-name`; `derivation` is `plan`'s.
- `account` is the corpus's own word for this concept — `306b` §10 says "influence account" three
  times and `ANALYZER-NEEDS:an-host-influence-carriage` says it twice. Two-to-three words beat a
  squatting single word, so `InfluenceAccount` it is.
- **Paired rename, and it is not optional**: `core::spine::Account<T>` (the k-capped operand
  account, `law-spine-operands-capped`) sits in the same module and would read as this type's
  sibling. Rename it `OperandAccount<T>`, which is what `309` §2 calls it in prose anyway. Cost
  measured: SIX call sites across TWO files (`core/src/spine.rs`, `cli/src/main.rs`).

### The shape

```text
InfluenceAccount(Standing)                      // private field; no Default; no Serde
   Standing = AuthoredBeforeContact             // a NAMED POSTURE, never a default
            | HostInfluenced(InfluencePhase)    // the marker IS evidence: it is obtainable only
                                                //   by having READ host-reported material
            | Untracked                         // an unconverted or unenumerable contributor

mints:   authored_before_contact()  ·  of_phase(InfluencePhase)  ·  untracked()
reads:   join(self, other) -> Self  ·  is_influenced(&self) -> bool  (TRUE for both upper points)
```

**The lattice, and its order-independence** (the census found NO permutation pin; this closes the
gap). The three points form a TOTAL CHAIN
`AuthoredBeforeContact ⊏ HostInfluenced ⊏ Untracked`, and `join = max`. A total order gives
commutativity, associativity and idempotence for free, so order-independence is a property of the
shape rather than of a proof — which is what `core/CLAUDE.md pin-set-meet-order-independence`
asks of every universal meet. `Untracked` sits ABOVE `HostInfluenced` because "we did not compute
it" is strictly less informative than "we computed it and it is influenced", and the safe reading
of less-informative is more-influenced; joining `Untracked` with `AuthoredBeforeContact` therefore
yields `Untracked`, which IS `306b:rul-untracked-is-not-authored` spelled as an algebra.

**`Untracked` is NOT the reserved fourth grade** (`dec-untracked-is-not-gradation`, proposed).
`core/src/influence.rs` L85-87 reserves the sealed `Grade` trait's fourth inhabitant for
`306b` §1c GRADATION — how MUCH influence (intensity, distance, contingency). `untracked` is a
different axis entirely: how much of the derivation we BOTHERED to compute. Spending the reserved
slot on it would foreclose the open question and would additionally make
`Influenced<Untracked, P>` constructible with a payload, inviting untracked VALUES to be carried
around rather than untracked SEAMS to be declared. Keep the sealed trait untouched; `Untracked`
is an arm of the account. (If §1c ever lands, the natural home is a product —
`{standing, intensity}` — and the chain above is the `standing` half unchanged.)

### The sealing mechanism, three-legged

`306b:rul-consequential-sinks-require-influence`'s last sentence asks for a sealed contract plus
an exhaustive census that makes a NEW species fail until it joins. No single mechanism does that
in Rust; three together do, and each is already an established idiom in this tree:

1. **TYPE leg** — every `SpineXxx` gains private fields and ONE `minted(<inputs>, InfluenceAccount)`
   constructor, plus a SEALED `InfluenceBearing { fn account(&self) -> InfluenceAccount }`
   implemented once per species inside `core::spine`. `Spine`'s setters read the account through
   the trait and there is no `grade` parameter and no overwrite. A new species must be authored
   inside that module, where the discipline is visible.
2. **CENSUS leg** — a second no-wildcard classification beside `SpineSpecies::census_arm`, e.g.
   `const fn account_carriage(self) -> AccountCarriage` with arms `{Joined, UntrackedAdapter}`.
   A new variant stops the build until classified, exactly as `census_arm` does today, and the
   counts get an assertion on
   `the_census_classifies_every_species_and_the_durable_arm_holds_exactly_four`'s precedent.
3. **LEXICAL leg** — a non-empty-walk gate for the CONSUMER half, which no type expresses across
   crates (`licence_mint_has_exactly_one_caller` and
   `fixture_intake_is_unreachable_from_production` are the same shape). Honest disclosure: this
   leg is coarse, and it should say so in its own doc.

The construction half needs no fourth leg: once the fields are private, a struct literal outside
`core::spine` simply does not compile, which is `rul-fixture-identity-never-production`'s
"comments are not a fence — absence of a constructor is" applied here.

### Product-wide unrepresentability statement (the standing TYPES rider)

MADE UNREPRESENTABLE by the proposal:

- a stable semantic object with no influence answer (non-optional field, no `Default`);
- "absent account = authored" — there is no `Option`, and `Untracked` is a distinct inhabitant
  that reads influenced;
- a caller choosing a grade word (mints take contributing accounts or an `InfluencePhase`
  witness; there is no `from_grade`);
- LOWERING an account (no `From`, no setter, no public field; `join` is monotone by construction,
  mirroring `core::influence`'s three existing `compile_fail` doctests);
- a Spine setter overwriting a record's account (setters take sealed records; `Spine.grade` and
  `Spine::minted_at` are gone);
- a NEW Spine species landing unaccounted (the sealed trait + the no-wildcard census);
- hand-building a Spine record at all, from outside `core::spine`.

STILL ADMITTED, deliberately:

- an EXPLICIT `untracked` seam — that is the staging mechanism, not a hole;
- a WRONG `authored_before_contact()` claim at a seat that does read influenced material. Only the
  influenced answer is evidence (the phase marker is minted by reading); the authored answer stays
  an assertion. Mitigation is that it is a NAMED POSTURE a seat must spell, on
  `plan::NO_ARTIFACT_FORM`'s precedent, never a default;
- v0's coarse positional flip — two routes with genuinely different provenance receive the same
  phase (§0, and RED-cell (a));
- gradation (`306b` §1c) — three points, no intensity;
- durable rehydration (`306b` §3a/§3b) — nothing persists an account, so nothing rehydrates.

## §map-3 — the threading fix for the region join

Exact seats, in flow order:

1. `settle.rs` L742-758 — `RouteRegionProof::new(*route, RouteAdmission::project(…), round.inputs.minted_at)`.
   The third argument is the SAME scalar for every route in every region in the run. It becomes the
   route's own account (§map-3b).
2. `region.rs` L551 — `RouteRegionProof::new(instance, admits, influence: Option<InfluencePhase>)`
   → `account: InfluenceAccount`; the private field L546 follows.
3. `region.rs` L739/745 — `decide_region`'s `proofs.iter().find_map(|proof| proof.influence)`
   becomes a fold over `InfluenceAccount::join`. **Two new arms**: a `RoutePopulation::Open`
   population and a non-corresponding proof list both join `Untracked` in, because routes the
   census could not enumerate (or could not match) have UNKNOWN influence, not absent influence.
   This is the one place the lane delivers a genuinely better answer than the stamp did, and it is
   RED-celled (§map-7c).
4. `region.rs` L612/657 — `SharedRegionDecision.influence: Option<InfluencePhase>` → `account`;
   `.influence()` → `.account()`.
5. `settle.rs` L761/912 — `lower_shared_decision` never reads `decision.influence()` and cannot,
   because `ProvisionalRegionDecision` (L168) has no field. Simplest correct shape: `decide_regions`
   reads `decision.account()` directly at L762 when building the provisional record; the lowering
   seat's signature need not widen at all.
6. `settle.rs` L168 — `ProvisionalRegionDecision` gains `account: InfluenceAccount`.
7. `settle.rs` L264-273 — `write_spine`'s region arm's `grade: None` becomes `region.account`.
8. `settle.rs` L134 — `ProvisionalSiteDecision` gains an account, minted by `decide_site`
   (`plan/src/lib.rs` L4683) beside the disposition and the act, per
   `acts-and-dispositions-mint-together`; `write_spine` L274-283 passes it.
9. `settle.rs` L125 — `SettleInputs.minted_at: Grade` → `world_account: InfluenceAccount`. This is
   a "restricted dependency account" in `306b` §10's sense: the caller passes an account it holds,
   never a grade word.

### §map-3b — what feeds a route's account

Traced to the one `HostReported` mint (§0). At v0 the join is:

```text
account(route) = world_account                       // the run's phase, or authored for the
                                                     //   intakeless entries
              ⊔ untracked-if-the-population-is-open  // the new, real conjunct
```

and the per-cell conjunct that would make two routes DIFFER is deliberately NOT built, because
building it from `by_fact`'s key set alone would launder (§0). The `///` on `decide_region` must
say so in its own words — this is a `churn-avoidance-disclosure`-class scope cut and it must be
stated where it bites, not only here.

Which route inputs are host-influenced, for whoever builds the per-cell conjunct later:
`observe(fact)` (the records fold — the only DIRECT one) · `valid_at` · `dead`
(`DeadBranchProof`, records-grounded by `erasure-is-records-grounded-only`) · `freshness`, via
`ReachingWalls` over the decision-fed effective set (the TRANSITIVE one, and the reason the
cheap derivation is wrong). Authored-before-contact: `vouches`, `connected`, `leaf_fact`,
`argv`, `class` topology, the census.

## §map-4 — the two drivers

`one-definition-table-two-drivers` binds: the binary and `WhyWorld` must convert in lockstep.

- `cli/src/main.rs` L1965 — `minted_at: Some(scoped_results.influence())`.
- `cli/src/world.rs` L484 — `minted_at: Some(crate::results::influence_after_reaching_for_host_bytes())`.

**`fnd-two-drivers-compute-one-fact-twice`** (+SURE, pre-existing). These are two independent
derivations of what is one fact. They agree today only because both are unconditionally `Some`.
The conversion is the moment to give them one seat: propose `results::ScopedHostEvidence::account()`
returning `InfluenceAccount` for the driver, and `results::account_after_reaching_for_host_bytes()`
for the paths that hold no carrier — the same split `results.rs` L194/L206 already has, one
vocabulary up. Do NOT unify them into one function: the why-driver genuinely holds no carrier, and
`influence_after_reaching_for_host_bytes`'s doc argues honestly for why widening is the right
answer there.

`world.rs` writes a strictly narrower Spine (only `SpineRenderDecision` L531 and, in tests,
`SpineDisposition` L1310) and reaches the settlement through the same `SettleInputs`, so the
divergence is population, not mechanism. Both also reach `project_plan` and
`spend_certifier_trip`, so §map-3's items 5-9 cover both by construction.

The FIVE censusless/kernel entries pass `None` today and become the named authored posture:
`plan::build_plan` (`lib.rs` L4290), `coverage/src/lib.rs` L661, `hostsim/src/lib.rs` L1513,
`sweep/src/drive.rs` L237, and `main.rs`'s two test seats L7646/L7819. Each already carries a
comment saying "No intake"; the account makes the comment a type.

## §map-5 — `30Na:stop-spine-mode-is-durable` (GATED — the conductor must act before EXECUTE)

**What was found, precisely** (+SURE, and it is worse than `30Na` recorded):

- `record_durable_arm` hard-codes `mode: "whylog-replay"` (`main.rs` L2911) on the LIVE
  plan/apply path, so the field describes neither producing invocation. Its own doc at
  `core/src/spine.rs` L320-327 already says so and says not to tidy it.
- `whylog::view::Invocation` NAMES `mode` (L462), so it reaches the durable header bytes
  (`whylog.rs` L861-862).
- `mode_valid` (L1318) accepts `{plan, apply, roundtrip, whylog-replay}` — so the truthful value
  needs no grammar change.
- **The new finding**: `mode` is not merely persisted, it is an EQUALITY-CHECKED CONJUNCT of the
  replay claims match — `WidthOneAttemptScope::matches_claims` requires
  `envelope.mode() == "whylog-replay"` (`cli/src/results.rs` L688), consumed at `main.rs` L2541,
  whose failure is `AdmissionRefusal::Framing`. Writing the truthful value without changing that
  check would make every production durable UNREPLAYABLE.
- The fixtures split accordingly: the three looms that reach the claims check spell
  `mode=whylog-replay` (`aid/tests/why-drift-address-unanswerable.loom`,
  `why-drift-analysis-suppressed.loom`, `cli/tests/whygallery-drifted-book-degraded-receipt.loom`);
  the three that refuse earlier spell `mode=plan` (`whylog-book-desync.loom`, `whylog-corrupt.loom`,
  `whylog-corrupt-results-block-overruns.loom`). `30Na`'s "the fixtures already spell `mode=plan`"
  is half true and the half that matters is the other one.

**The option space** (mapped, NOT settled — `rul-durable-contents-reviewed-before-design` governs):

- **(a) write the truth** — `mode` becomes the producing invocation's mode, and `matches_claims`
  stops comparing it (or compares it against the controller's own expected value). CHANGES the
  durable's bytes AND what re-ingestion consumes. Fully gated.
- **(b) doc-only** — re-document `mode` as "the mode a reader should replay this under". No byte
  change, but the field still contradicts its own enumerated vocabulary. Not a fix.
- **(c) NARROW THE TYPE** — `SpineInvocation.mode: String` becomes a closed enum whose single
  inhabitant today spells `whylog-replay`, so the record can no longer CLAIM to be the producing
  invocation's mode. Durable bytes unchanged; `mode_valid` unchanged; `matches_claims` unchanged.
  This is exactly `core/CLAUDE.md a-record-says-what-its-population-holds`'s prescription — "a
  field that cannot carry its documented claim is narrowed rather than left aspirational" — and it
  turns (a) into a later, reviewed, one-arm widening.
- **(d) drop the field** — durable content change. Fully gated.

**My lean** (-GUESS, and it is the conductor's call): (c). ~SUSPECT that (c) does not fire the
durable tripwire, because neither what the durable persists nor what re-ingestion consumes moves
by a single byte — only the in-memory record's type does. But I am deliberately not settling that:
`rul-durable-contents-reviewed-before-design` says the review clears FIRST, and "I think this one
does not count" is precisely the local, obviously-correct-looking judgment the rule exists to
refuse.

**What the conductor must do, before EXECUTE opens this row** (this is the instruction the brief
asked me to hand back): obtain the ruling on the durable surface — ask the human by preference,
`/opaque-review` when they are away — and hand EXECUTE the ruled option. If the ruling does not
arrive, EXECUTE ships §map-8 rows R1-R9 and leaves R10 unopened; nothing else in this lane depends
on it. Two riders to carry into whichever option is ruled, both already stated out-quarantine in
`306b` §3: a persisted derivation-grade must rehydrate without laundering, and an absent,
unverifiable, or recomputation-failing one reads MOST-influenced
(`306b:rul-missing-influence-grade-reads-highest`); and nothing re-read from a durable may drive
an action (`306b:rul-reingestion-drives-no-action`), which is NOT subsumed by the first because
uninfluenced persisted material is exactly the gap.

## §map-6 — the byte-identity gate plan

`ExcludedContent::InfluenceGrade` (`core/src/spine.rs` L285) keeps the account out of every
`DurableView` STRUCTURALLY: records implement no serialization, and a field no View names cannot
reach disk. So the `.whylog` half should hold trivially. The exact exposure, enumerated:

| `DurableView` | fields it names | touched by the type change | value moves? |
|---|---|---|---|
| `view::Invocation` (`whylog.rs` L461) | `mode`, `argv`, `book`, `oracles`, `nonce`, `attempt`, `host`, `started_at` | YES — all eight become accessors under sealing | NO (unless §map-5 is ruled (a)/(d)) |
| `view::Digest` L503 | `digest` | YES — accessor | NO |
| `view::RecordStream` L520 | `instants` (+ the borrowed `records`) | YES — accessors | NO |
| `view::disposition` L534 | `record.site` → `.leaf`, `record.decision` → `tag()` | YES — accessors | NO |
| `DurableProjection::project` L576 | `spine.{invocation,digest,record_stream,dispositions}()` | no change | NO |
| `drop_account` | `spine.population(species)` | no change | NO |

So `SpineDisposition.decision` and `SpineDisposition.site` ARE durable-named (via `tag()` and
`.leaf`), which is the brief's concern, confirmed — but only through accessors, so no value moves.
`SpineRegionDecision` reaches no View at all (its species is `CensusArm::New`), so §map-3's
threading is durable-invisible.

How EXECUTE proves nothing moved:

1. `mise run bless:dry` — the acceptance summary with zero golden writes; a drift is a finding.
2. `mise run both gate:full-quiet` — the nine loom-embedded `.whylog` cases ride
   `test:looms`/`test:e2e` as byte-exact transcripts (`crates/aid/tests/{why-drift-address-unanswerable,
   why-drift-analysis-suppressed,whylog-book-desync,whylog-corrupt,whylog-corrupt-headerless,
   whylog-corrupt-header-tag-missing,whylog-corrupt-results-block-overruns,whylog-version-refused}.loom`
   + `crates/cli/tests/whygallery-drifted-book-degraded-receipt.loom`). There is no bespoke
   `expected.whylog` golden format; the loom transcripts ARE the durable's byte gate.
3. EXISTING goldens never move (`30O`'s standing rider): drift anywhere is a finding, not churn.
4. OPTIONAL, and worth it once: `mise run spine:baseline` before and after, and hand the diff to
   the conductor. It is `309` §4's smoke-diff, already built, explicitly NOT an acceptance gate,
   and it is the only instrument that sees a decision-state change no byte gate can.

## §map-7 — red cells, committed

Three pins registered in `internal_tooling::xfail::PINS` with live call sites, plus one green
companion. Horizon `end-of-r30` for all three.

- **(a) `p-x-region-account-reaches-the-spine-record`** — `plan/src/settle.rs`, test
  `the_region_record_carries_the_account_its_meet_joined`. Scans `write_spine`'s region arm and
  requires it to name the region decision's OWN account. Observes: feature-off, the arm writes a
  literal and the global stamp decides, so the meet's join is computed and discarded; feature-on,
  a route-level difference can reach the record at all. Deliberately the COARSE half — the seat is
  private and its input is a whole `RegionRound`, so no value-level assertion over a public API
  reaches it. Same shape as the file's own `only_the_universally_agreed_arm_retires_anything`,
  and it says so.
- **(b) `p-x-spine-record-keeps-its-mints-account`** — `plan/src/spine.rs`, test
  `a_spine_record_keeps_the_account_its_mint_supplied`. Value pin: a record whose mint answered
  authored, stored on an influenced Spine, must still answer authored. Fails today because every
  setter overwrites. **`core/src/spine.rs`'s
  `the_spine_stamps_the_grade_so_a_mint_site_cannot_forget_it` (L535) pins the FORBIDDEN behaviour
  and is this cell's direct contradiction — EXECUTE rewrites it rather than leaving it passing.**
  Sited in `plan` rather than `core` on purpose: `core` is dependency-free by design and adding
  `internal-tooling` as even a dev-dependency there is a decision this half declines to take.
- **(c) `p-x-unenumerated-population-is-not-authored`** — `plan/tests/region.rs`, test
  `an_open_population_does_not_read_as_authored_before_contact`. Value pin: a
  `RoutePopulation::Open` region whose enumerated proofs are all authored must not read authored.
  Fails today (`find_map` over all-`None` is `None`). Greens with `Untracked`.
- **GREEN companion** — `the_shared_account_does_not_depend_on_which_route_carries_influence`
  (`plan/tests/region.rs`): the join answers the same whether the influenced route is the head or
  the tail. Green today because the landed representation has two points; committed now so the
  fold that replaces `find_map` cannot regress order-independence while widening it. This is the
  permutation pin the brief asked for, in the only form that is expressible before the type exists
  — the three-point version is (c)'s sibling and greens with it.

**DEVIATION, reported** — the brief specified `Deferred.why` on these rows. `Horizon::Deferred`
records a slip that has not happened (its `was`/`now` pair is the whole point), and
`Horizon::Unscheduled` records an absent schedule that is present (this lane IS r30). All three
rows are `Horizon::Scheduled("end-of-r30")` with the full greening condition in the required
`trigger` field. Conductor may overrule.

## §map-8 — the EXECUTE order

One commit per row, conservative-first, each independently green. Sizes -GUESS.

| # | row | touch (line ranges at `eae684b1`) | size |
|---|---|---|---|
| R1 | mint `InfluenceAccount` + `join` + the `compile_fail` doctests; rename `Account<T>` → `OperandAccount<T>`. No consumers. | `core/src/influence.rs` +~120 · `core/src/spine.rs` L92-132, 138, 6 sites | S |
| R2 | SEAL the 16 species: private fields, one `minted` constructor each, the sealed `InfluenceBearing` trait, the `account_carriage` census arm. Setters STILL overwrite ⇒ no behaviour change. | `core/src/spine.rs` L308-548, L834-876, L953-1171 · every construction site in `settle.rs`/`certifier_trip.rs`/`plan/src/spine.rs`/`main.rs`/`world.rs` | **L** |
| R3 | REMOVE the stamp: `Spine::minted_at`, `Spine.grade`, the 18 `record.grade = self.grade;` lines, `Spine::grade()`. Rewrite `the_spine_stamps_the_grade_so_a_mint_site_cannot_forget_it`. RED-cell (b) greens. | `core/src/spine.rs` L145, L878-951, L953-1171, L535-563 | M |
| R4 | convert the settlement: `SettleInputs.world_account`; accounts on `SiteDecision`/`RouteDecision`/`ProvisionalSiteDecision`/`ProvisionalRegionDecision`; `write_spine` + `record_survival` pass them. | `plan/src/settle.rs` L110-330, L560-600, L693-812 · `plan/src/lib.rs` L4312-4353, L4467-4640, L4683+ | M |
| R5 | thread the region join: `RouteRegionProof`/`SharedRegionDecision`/`decide_region`; `Open` and non-corresponding ⇒ `Untracked`. RED-cells (a) and (c) green. | `plan/src/region.rs` L535-760 · `plan/src/settle.rs` L742-770 | S–M |
| R6 | convert the licence mints + the projection: the six `prove_*`/`mint*` seats, `Plan::decided`, `project_plan`, `record_render_decisions`. | `plan/src/lib.rs` L559-880, L1493-1580, L5070-5100 · `plan/src/spine.rs` L138-290 | M |
| R7 | RESTRICT the pre-contact side: `PlacementDecision`, `PlacedSources`, `ArtifactEmission`, `Selection`, `ImportEdit`, the four probe-side plans; `Selection::with_plan` JOINS the plan's account into `ArtifactSet`. | `plan/src/placement.rs` L156-400 · `cli/src/artifact.rs` L820-900 | M |
| R8 | `certifier_trip` demotion JOINS (lean-3): the `&mut record.decision` pokes become a Spine method taking the joined account. | `plan/src/certifier_trip.rs` L109-167 · `core/src/spine.rs` L995-1004, L1153-1158 | S |
| R9 | the two drivers in lockstep + the five named authored postures. | `cli/src/main.rs` L1965, L2758-2960, L7646, L7819 · `cli/src/world.rs` L484, L531, L1310 · `cli/src/results.rs` L162-210 · `coverage/src/lib.rs` L661 · `hostsim/src/lib.rs` L1513 · `sweep/src/drive.rs` L237 | S |
| R10 | **GATED** — `stop-spine-mode-is-durable`, whichever option §map-5 is ruled. Do not open without the ruling. | `core/src/spine.rs` L319-333 · `plan/src/whylog.rs` L461-500, L1318 · `cli/src/results.rs` L688 · `cli/src/main.rs` L2911 | S–M |

R2 before R3 is load-bearing: removing the stamp before sealing would leave sixteen public
`grade` fields anyone could set, which is a strictly worse state than today's.

## §map-9 — exclusions, confirmed

- **No influence-aware render.** `309` §5/§7 defers it and this lane honours that. §map-10's
  lean-2 refutation is about what the sinks ACCEPT, not about what they print: no render output
  changes.
- **No durable change** beyond §map-5's ruled option. Any other durable touch is a STOP.
- **No gradation past the three points.** `306b` §1c stays open; the reserved fourth `Grade`
  inhabitant stays reserved (`dec-untracked-is-not-gradation`).
- **No `core::sorted` backing.** Confirmed against the moving authority: `spike/verify/aeneas/src/lib.rs`
  `#[path]`-includes exactly `core::sorted` and `analysis::lattice` and nothing else, and neither
  `core::spine` nor `core::influence` appears in any Kani harness, in `minispec/`, or in
  `plan/src/rederive.rs`. **This lane has ZERO verified-core ripple as designed** — the account is
  a three-point enum in a newtype, needs no ordered collection, and the `verified-core-discipline`
  skill is loaded only if a mechanical check fails under EXECUTE.
- **No `tc-*` resolved locally.** §map-10.
- **No re-derivation of the load plane.** `rul-the-account-never-enters-compared-state` (§map-1)
  keeps the conversion outside the kernel's compared state.

## §map-10 — `tc-*` flags, names argued, proposed edits

### `tc-*` flags (flagged, NOT resolved)

- **`tc-accounting-reads-are-not-gating`** — `306b` §6b says values derived from host-reported
  material "may not … select a code path in the engine". But a per-object join CANNOT be computed
  without reading which of its inputs were influenced, so a literal reading of §6b makes §10's
  per-object carriage impossible. My lean: §6b binds control flow that reaches a DECISION about
  the plan, and accounting is the one exempt consumer — but the exemption must be TYPED and
  STATED, not assumed. Product surface: none (the account is decision-inert). Strawman of the
  hazard §6b actually guards, in sh terms: an engine that counted how many sites a host reported
  on and elided more when the count was high would be gating on host-shaped material; an engine
  that records "this decision read a host record" is not. Winner-shifting: no. Cross-cutting: yes
  — it binds every future account consumer.
- **`tc-load-decisions-read-authored`** — `30I:rul-load-decisions-are-authored-before-contact`
  [TYPED] says load decisions wear authored, and `SpineLoadDecision`'s inputs are all pre-contact.
  But the record is minted inside `record_new_arm`, on a path the intake reached. Does the account
  join the CONTROL condition that reached the mint (⇒ influenced) or only the DATA (⇒ authored)?
  `306b` §10 says "every contributing data and control input", which argues influenced; `30I`
  argues authored. The two typed directions are jointly unrepresentable in the landed type, which
  `core/src/spine.rs` L324-331's own comment already records as pending. This is the residue of
  `30M:ask-spine-grade-boundary` that `306b` §10 did NOT close. Winner-shifting: no (nothing reads
  the account). Product surface: what a future frontier render marks as measured vs authored.
- **`tc-untracked-sits-above-influenced`** — the three-chain puts `Untracked` at the top, so
  `join(HostInfluenced, Untracked) = Untracked` and the fact that a decision was ALSO genuinely
  influenced is lost to a reader. The alternative is a product `{influenced: bool, complete: bool}`.
  I lean chain-now (simplest thing that satisfies `rul-untracked-is-not-authored`, and no
  authority is lost since `Untracked` reads maximally influenced anyway), product-later-if-§1c-lands.
- **`tc-spine-record-mut-accessors-survive`** — `Spine::{disposition_mut,dispositions_mut,region_decisions_mut}`
  exist for the two post-construction rewrites. Under sealing they should become named methods
  that take the joined account, which removes the `&mut` surface entirely. That is a wider API
  change than "carry an account", so it is flagged rather than assumed.

### The conductor's three leans, adjudicated

- **`lean-account-is-non-optional-with-a-typed-untracked` — CONFIRMED**, with two sharpenings:
  the name is `InfluenceAccount` and it PAIRS with renaming `core::spine::Account<T>` →
  `OperandAccount<T>` (six sites); and `Untracked` is an arm of the account, NOT the reserved
  fourth `Grade` (`dec-untracked-is-not-gradation`, §map-2).
- **`lean-render-sinks-are-untracked-adapters-this-lane` — REFUTED, with a better shape.**
  `306b:rul-consequential-sinks-require-influence` binds what a sink ACCEPTS. `Plan::render_sh`
  and `Plan::render_apply` take `&Plan`, and a `Plan` is a projection of Spine records, so under
  `rul-projections-continue-influence-flow` it should CONVERT — `project_plan` joins the accounts
  of the dispositions, regions and render decisions it read. The two render sinks are then
  compliant for free, with no adapter, no `untracked`, and no influence-aware render (so `309`
  §5/§7's deferral is untouched). The FOUR probe-side `render_sh`s
  (`ProbePlan`/`ResolverPlan`/`ReachPlan`/`DerivationPlan`) RESTRICT instead: the compiled probe
  is frozen before intake. `Selection`/`ArtifactSet` are as the conductor guessed — RESTRICT,
  except `with_plan`, which joins the plan's account because the artifact bytes ARE the plan.
- **`lean-certifier-trip-demotion-joins` — CONFIRMED.** Seat found:
  `certifier_trip::demote_on_trip` at `plan/src/certifier_trip.rs` L121 and L141 rewrites
  `record.decision = Disposition::Run` through `dispositions_mut`/`region_decisions_mut`. Under
  an IMMUTABLE account that is a NEW semantic mint whose inputs are the certifier's witness (a
  `SolveConsistency` over solver state that is decision-fed and therefore records-influenced
  whenever anything was admitted) and the original record. Its account is the JOIN of both —
  never a reset, never a pass-through that ignores the trip. At v0 both operands carry the same
  value so the join is a no-op today; the SHAPE is what matters, and the mechanical consequence
  is that `record.decision = …` stops being a bare field poke (see
  `tc-spine-record-mut-accessors-survive`).

### Names argued (overloads and cross-domain glosses only)

- `InfluenceAccount` — argued in full at §map-2 (`Grade` is a claim-tier overload used
  unqualified in the same file; `provenance`/`placement`/`layout`/`lift`/`derivation` all
  squatted; `account` is the corpus's own word).
- `OperandAccount<T>` — the paired rename that keeps the two "account"s apart;
  `309:law-spine-operands-capped` already calls it an operand account in prose.
- `Standing` (the private inner enum) — chosen because `core::influence`'s own module doc already
  says "how far a value stands from host-produced bytes". Private, so it costs nothing if the
  conductor prefers another word.
- Everything else stands unceremonied per the human's 10% lean: `join`, `is_influenced`,
  `authored_before_contact`, `of_phase`, `untracked`, `minted`, `InfluenceBearing`.

### Proposed steering / register edits (conductor applies; this half edits no `CLAUDE.md`)

1. `spike/crates/core/CLAUDE.md`, new bullet under "Law — vocabulary discipline":
   **`the-influence-account-is-carried-never-stamped`** — every stable semantic object carries a
   private, immutable, non-optional `InfluenceAccount` joined at its own mint from every
   contributing data and control input; Spine STORES it and never computes, overwrites, or fills
   one; an unconverted seam is explicit `untracked` and reads maximally influenced; the join is a
   total chain so order-independence is a property of the shape. Cite `306b` §10 +
   `309:rul-spine-preserves-never-stamps`.
2. `spike/crates/core/CLAUDE.md`, same bullet or its own:
   **`the-account-never-enters-compared-state`** — the conversion boundary is the decision plane
   and outward; a kernel value that participates in lattice `Eq` or fixpoint comparison never
   gains an account (`inv-determinism`'s semantic-`Eq` precondition; the `CollapseNarrative`
   Eq-exclusion is the failure this positioning avoids, not a technique to copy).
3. `spike/crates/plan/CLAUDE.md`, under "Law — the license mints": a rider on
   `sole-mint-witnesses` that both irreversible-verb mints take an `InfluenceAccount` beside the
   vouch, and that `certifier_trip`'s demotion is a re-mint that JOINS rather than a field poke.
4. `spike/CLAUDE.md`, under "Invariants — host evidence & controller attribution": one bullet
   naming the carriage law and pointing at the two `core` bullets, since it is cross-crate.
5. `ANALYZER-NEEDS.md` row `an-host-influence-carriage`: `st` moves `S` → `B` at the EXECUTE fold,
   and the "Full threading/gradation remains staged" clause gains "per-route difference is
   unexpressible at v0's positional flip; pinned red" so the register states the residue rather
   than implying it landed.
6. `FORFEITS.md`: **no row is owed by this lane.** The scope sharpening [human-typed 2026-08-21]
   says a row is an ANALYSIS limitation that would yield better or more-correct ELISIONS. Influence
   carriage licenses nothing and changes no elision, so the per-route residue belongs in the xfail
   census (where it now is) and in `30O`'s schedule, not in FORFEITS. Flagging this explicitly
   because a builder reading `30P:rul-forfeits-carry-reds` might reach for a row reflexively.
7. `spike/CLAUDE.md rul-rc-reaches-genkill-only-through-decisions` says in so many words that "the
   wider permanent law is expected out of the influence implementation round and supersedes this
   in place". This lane does NOT produce that law: it carries accounting, and the rc-vs-gen/kill
   species separation is orthogonal. Recommend the conductor either re-horizon that sentence or
   strike the expectation — leaving it pointing at a lane that will not discharge it is the
   register drift `30O:register-and-steering-debt` already tracks.

### Context other lanes must maintain

- `fnd-one-mint-fence-misses-a-qualified-spelling` (+SURE, minor, pre-existing):
  `plan/src/records.rs`'s `the_influence_grade_has_exactly_one_mint` greps for
  `Influenced::host_reported(` and `::<HostReported, ()>::host_reported(`, but
  `core/src/spine.rs` L541 spells it `Influenced::<crate::influence::HostReported, ()>::host_reported(())`
  and slips both needles. The fence is not currently lying (that IS `core`, which the retain step
  would have dropped had the path matched), but any test that mints a phase under a qualified
  path is invisible to it. Whoever rewrites that test in R3 should either use the unqualified
  spelling or widen the needle.
- `30Pc:bug-assignment-bearing-dot-is-inlineable` is REPAIRED on this lineage
  (`cli/src/artifact.rs` L426 requires `assigns.is_empty()`, with its focused test at L1692). No
  action owed by this lane.
- Anyone touching `plan/src/settle.rs` after this MAP: three lexical seat-scans now live in its
  test module (`only_the_universally_agreed_arm_retires_anything`,
  `a_provisional_round_names_no_spine_setter`, and RED-cell (a)); renaming
  `fn write_spine` or `for decision in self.decisions` moves a fence.

## §close — deviations, and what stopped

Deviations from the brief, each an OPEN item for the conductor:

1. **Horizon spelling** — `Scheduled("end-of-r30")` rather than `Deferred.why` (§map-7).
2. **lean-2 refuted** rather than confirmed; the replacement shape is narrower and cheaper
   (§map-10).
3. **RED-cell (a) is lexical, not value-level.** The seat between `decide_region` and Spine is
   entirely private and its input is a whole `RegionRound`; a value pin would need a settlement
   fixture this half declined to build. Stated in the test's own doc as the coarse half.
4. **RED-cell (b) sited in `plan`, not `core`.** `core` is dependency-free by design and
   `internal-tooling` reads the filesystem; adding it as a `core` dev-dependency is a decision
   above a MAP builder's authority. The consequence is that this MAP touches ZERO `core` files.
5. **The note ID the brief expected to be occupied was not.** Only `30Qa`–`30Qc` existed when
   this was minted; `30Qd` was free and is taken.

**What was verified.** `mise run check-quiet` GREEN at the lineage tip before any edit. All four
new cells measured GREEN as pins (a pin PASSES while its target assertion fails):
`settle::tests::the_region_record_carries_the_account_its_meet_joined` ·
`spine::tests::a_spine_record_keeps_the_account_its_mint_supplied` ·
`region an_open_population_does_not_read_as_authored_before_contact` ·
`region the_shared_account_does_not_depend_on_which_route_carries_influence` (the green
companion). `internal-tooling xfail::tests::xfail_census_is_coherent` PASSES, so the two-way
registry↔call-site check accepts all three new rows. Comment budget: **0 net new inline `//`**
(budget 10), 54 net new `///` (counted separately; every pin's doc plus the two report-bearing
tests' acceptance criteria).

**BLOCKER — the machine, not the work.** `C:` hit 100% (0 bytes free) mid-verification: a
`mise run test` died with `LNK1318 … LIMIT/FILE_SYSTEM` and
`rustc-LLVM ERROR: IO failure on output stream: no space on device`, and the harness's own temp
filesystem began failing writes with `ENOSPC`. Space partially recovered (to ~2 GiB) as the
failed build released its own artifacts, which is what let the targeted runs above complete.
**The completion contract could not be met**: `mise run preflight gate` REFUSES —
"1.9 GiB free … needs 4.0 GiB (warm cache)" — so `mise run both gate:full-quiet` never ran on
either leg, and `DORC_PREFLIGHT=skip` is forbidden by this lane's brief. `mise run xfail:census`
was likewise not rendered (its GATE, `xfail_census`, did run and passed).

Nothing was reaped. `doctor-inventories-never-reaps` says a reap needs a containment proof this
lane has no business making, and `30Q` §5 already names stale worktrees as the human's sweep.
The read-only inventory, for whoever does sweep: **161.6 GiB at risk in worktrees on this leg**,
across eighteen — `Code/Dorc [ai/main]` 23.6 GiB DIRTY · `worktrees/r30-conduct` 16.3 GiB ·
`r30-loading` 9.1 GiB DIRTY · and fourteen `agent-*` lane trees at 7–10 GiB each, all CLEAN, most
of them lanes `30Q` §2 records as already FOLDED (loop, planner, load-a, load-b, fruit). This
worktree's own `spike/target` is 7.4 GiB and is the only disk this lane created.

**What that leaves owed**: a both-leg `mise run both gate:full-quiet` over this branch once the
disk is recovered, before the red cells are treated as landed. The risk it covers is narrow — the
four cells are unit-tier and were measured directly, and no production code changed — but the
WSL leg has caught a Linux-only failure in this project before, and the whole-workspace clippy
that rides `gate:full-quiet` has not seen these files.

## §execute-checkpoint — R1–R3 landed, and where the rows moved

> Tier: LLM-authored EXECUTE report (Opus builder; seat
> `.claude/worktrees/agent-ab4cb8131f42ba75b`, branch `ai/r30-lane-influence-exec`, lineage
> `ai/r30-conduct` @ `66954cc2`). Return-and-resume checkpoint: R4–R11 are NOT started.

### Commits, in order

| commit | row | what |
|---|---|---|
| `9f77ef43` | gate | the pre-conversion `spine:baseline`, frozen at `Research/notes/30Qd-spine-baseline-before.txt` (291 cases) |
| `de18560a` | R1 | `core::influence::InfluenceAccount` + the `Account<T>` → `OperandAccount<T>` rename + three lexical fences |
| `fd65b15b` | R2 | the sixteen species SEALED; `InfluenceBearing`; the `account_carriage` census; the run-wide account threaded |
| `09cc0cfd` | R3 | the object-global stamp REMOVED; the stamp test rewritten; RED-cell (b) promoted; the mint fence's needle widened |
| `9a3463ca` | — | comment-budget trim |

`mise run test` GREEN at every one of the three row tips (2687 trials), `mise run check-quiet`
GREEN before each commit. The whole e2e + loom corpus rides `mise run test`, so every golden and
all nine loom-embedded `.whylog` transcripts are byte-identical as of `9a3463ca` — no bless, no
re-bless, no drift.

### `rul-row-boundaries-moved-under-the-sealing` — the one thing to rule on

**The MAP's row split does not survive private fields, and R2/R3 absorbed parts of R4, R8 and R9.**
+SURE, and it is forced rather than chosen: sealing a record's fields means every seat that MUTATED
one and every seat that SUPPLIED one has to change in the same commit that seals it, or the row is
not compilable — and the brief's "each row independently green" outranks the row boundaries.

What actually landed where, against `§map-8`:

- **R2 absorbed R8 entirely.** `Spine::{disposition_mut, dispositions_mut, region_decisions_mut}`
  are GONE, as `tc-spine-record-mut-accessors-survive` rules. Private fields left no third option:
  a `&mut SpineDisposition` can no longer write `record.decision`, so the two post-construction
  rewrites had to become named methods in the same commit. They are
  `Spine::demote_dispositions(witness, stands, demoted) -> Vec<SiteId>` and
  `Spine::demote_region_decisions(...) -> Vec<RegionRoutes>` (each demoted record re-mints as
  `own_account ⊔ witness`), plus `Spine::reattach_dispositions(attach)` for
  `attach_spine_probe_provenance`, which touches NO account and says why in its own doc
  (`fnd-provenance-attach-raises-nothing`, stated as the property the MAP asked for).
  `demote_on_trip` / `spend_certifier_trip` / `project_censusless` /
  `cli::world::demote_on_certifier_trip` all gained a `witness: InfluenceAccount` parameter,
  supplied by the caller because the two drivers reach their account differently.
- **R2 absorbed R9's driver seats and the five authored postures.** `Spine::minted_at` had to take
  an `InfluenceAccount` rather than a `Grade` at R2, or the conversion would have had to happen
  inside `core::spine` — which would have put `of_phase` in the kernel and made the ruled
  "one phase→account transition" false for two rows. So `results::ScopedHostEvidence::account()`
  and `results::account_after_reaching_for_host_bytes()` (the ruled two-seat split) landed at R2,
  the binary reads `world_account` ONCE beside `_scope`, and the five intakeless entries
  (`plan::build_plan`, `coverage`, `hostsim`, `sweep::drive`, the two `main.rs` test seats) spell
  `InfluenceAccount::authored_before_contact()`.
- **R2 absorbed R4's TYPE change** (not its semantics): `SettleInputs.minted_at` and
  `build_plan_walled`'s parameter are `InfluenceAccount`, and `RouteRegionProof`/
  `SharedRegionDecision` carry `account` rather than `Option<InfluencePhase>`. `decide_region`'s
  `find_map` became a `fold` over `join` — semantically identical today over a homogeneous
  population, and the shape R5's `Untracked` arms need.
- **R3 absorbed a slice of R6.** With the stamp gone, `record_render_decisions` had no account to
  read, so `project_plan` gained a `world: InfluenceAccount` parameter and passes it down. `Plan`
  itself does NOT carry an account yet — `Plan::decided`'s join is still R6's, and it will replace
  this parameter.

**What is left for R4–R9, unchanged in substance**: R4's per-object accounts on
`SiteDecision`/`RouteDecision`/`ProvisionalSiteDecision`/`ProvisionalRegionDecision` and the
`minted_at` → `world_account` rename; R5's `Untracked` arms and cells (a)/(c); R6's six licence
mints and `Plan::decided`'s joined account; R7's RESTRICT side; R9's `tc-load-decisions-read-authored`
lean plus whatever naming remains.

### The sealing's exact shape

Three legs, as `§map-2` proposed, with one sharpening.

1. **TYPE.** All sixteen `SpineXxx` have private fields and exactly one `minted(<inputs>, account)`
   constructor; every read is an accessor. `InfluenceBearing { fn account(&self) -> InfluenceAccount }`
   is sealed by a private supertrait in `core::spine` and implemented once per species there. There
   is no account setter, no `&mut` route to one, and no `Default` on the account.
2. **CENSUS.** `SpineSpecies::account_carriage() -> AccountCarriage`, no wildcard, asserted by
   `every_species_declares_how_its_writer_reaches_an_account`: **12 `Joined` / 0 `UntrackedAdapter`
   / 4 `Unminted`**.
3. **LEXICAL.** Three fences in `plan/src/spine.rs`'s test module over a shared workspace walk with
   a non-empty-walk floor: `the_phase_to_account_transition_lives_at_one_seat` (`of_phase`),
   `every_authored_before_contact_posture_is_enumerated`, `every_untracked_adapter_is_enumerated`.

**`dev-carriage-census-needs-a-third-arm`** (deviation, conductor's to ratify). The MAP proposed
`{Joined, UntrackedAdapter}`. I built `{Joined, UntrackedAdapter, Unminted}`, because four species
(`Vouch`, `Observation`, `ValidityRound`, `Outcome`) have NO WRITER at all — `30F` §4.5 and `30Nd`
already disclose that — and classifying them `Joined` would be a claim about a mint that does not
exist, which `core/CLAUDE.md a-record-says-what-its-population-holds` forbids in so many words. The
count assertion is what makes a species moving between arms a diff.

### `dev-run-identity-grouped-out-of-the-invocation-mint` — a design choice the MAP did not foresee

`SpineInvocation` has eight fields; `minted(…)` plus the account would be a nine-argument function,
which `clippy::too_many_arguments` refuses. Rather than reach for an `#[expect]` on fresh code I
grouped the four controller-minted fields into `core::spine::RunIdentity { nonce, attempt, host,
started_at }` — which is what the type's own doc already called them ("controller-minted run
identity plus what it was pointed at"), and which `rul-attribution-is-controller-minted` treats as
one thing. `view::Invocation::of` reads them through `record.identity()`; **no durable byte moves.**
Flagged because it is a public shape change the MAP did not price.

### Surprises, each an OPEN item

- **`fnd-the-mint-fence-had-a-second-blind-spelling`.** The R3 rider said to widen
  `the_influence_grade_has_exactly_one_mint`'s needle. I did — it is now the bare `host_reported(`,
  which no qualified path can slip — and in the same commit I stopped `core::spine`'s rewritten test
  MINTING a phase at all: it widens an authored one instead. So the fence's population is genuinely
  one file again (`plan/src/records.rs`) rather than one file plus a tolerated test.
- **The three lexical fences enumerate TEST files too**, and cannot distinguish them lexically from
  production. `of_phase` lists `cli/src/results.rs` (the one production seat) plus three test seats;
  `authored_before_contact` lists twelve files. I chose enumerate-and-name over filter-and-guess,
  because a filter that guessed would be the fence's own blind spot. The lists move as rows land,
  which is the fence doing its job, but it does mean a conductor reading a row diff will see them
  churn.
- **`every_untracked_adapter_is_enumerated` is currently EMPTY**, asserted so. R5 is what first
  populates it, and the human's stated purpose for this lane — force the discipline, then WATCH
  whether holes appear — now has a mechanical instrument rather than only the fold report. Flagged
  as an addition the brief did not ask for.
- **`Spine::debug_dump` lost its run-wide `grade=` header line** (there is no run-wide account any
  more) and gained per-record `account=` on the load-decision and region rows. It has no production
  caller and no golden, so nothing moved.

### Budget and evidence at the checkpoint

- Inline `//`: **+22 added, −5 removed = 17 net new** against the lane budget of 30. Three of the 22
  are the structural banner around the sealed-contract block, matching the file's existing style.
- `///`: **+258**, counted separately (sixteen species × their accessors, the account type, the
  three fences).
- `mise run test`: 2687 trials, 2687 passed, 2 skipped. `mise run check-quiet`: clean.
- `mise run xfail:census`: `p-x-spine-record-keeps-its-mints-account` is GONE from the registry
  (promoted — its assertion survives as an ordinary test in `plan/src/spine.rs`); (a) and (c) are
  still live and still red, as they should be until R5.
- NOT yet run at this checkpoint: `bless:dry`, `mise run both gate:full-quiet`, and the AFTER
  `spine:baseline`. All three are end-of-lane obligations.
