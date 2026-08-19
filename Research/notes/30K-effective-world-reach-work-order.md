# 30K - Effective world reach: remove the split wall machinery

> Tier: r30 implementation work order, analogous to `30G`. `plans/28Q` remains the
> architectural parent; this note does not reopen its product design. Human-directed
> rulings from the 2026-08-19 sitting are marked **[TYPED]**. Measured implementation
> facts are **[+SURE]**. Concrete type/module choices are **[PROPOSED]** unless a law
> below constrains their shape. Root docs and `spike/CLAUDE.md` outrank this note.
>
> Scope: replace the analyzer's three answers to one staleness question with one
> apply-effective reach analysis, while preserving a separate origin analysis for
> probe construction. This is implementation unification, not a narrow guard mint.
> No backwards-compatibility obligation exists: obsolete internal variants and walks
> are deleted rather than adapted.
>
> Sequencing: `30I` seam 4 first finishes the one complete static-load-occurrence
> account and removes the superseded ambient-dependency refusal. This stage then lands
> before `30I` writes bundle projection code, artifact forms, or final XFAIL/golden
> promotions. `30I` builders do not absorb this work.

## 0. The target in one screen

`target-effective-reach-replaces-walks` **[human-directed target]** - apply-time freshness,
Query-status validity, total walls, and footprint-scoped survival derive from one
CFG-aware set of mutations that may actually execute. `plan::wall_walk_total` and
`plan::wall_walk_survival` are deleted. A modeled running mutation invalidates
downstream resting measurements exactly as an unmodeled one does; a vouched,
probe-converged site whose only lost elision precondition is freshness guards in
position rather than running bare.

`constraint-origin-effective-reach-separate` **[derived implementation constraint]** - origin
analysis remains the pre-contact, pessimistic input to probe construction. Effective
reach begins only after the one frozen probe has returned. No effective decision may
change which bytes shipped, add a probe, or re-key a probe record.

`constraint-plan-surface-stays-readable` **[human-directed constraint]** - precision recovery never
emits wall flags, conditional-tail state, generated bookkeeping branches, or other
controller machinery into reviewed `plan.sh`. A Guard is conservatively a possible
mutator for downstream analysis. The plan uses only the settled ordinary forms:
verbatim run, ordinary `( check ) || original` guard, omission, or replacement.
Private probe emission may remain mechanically complex; that grants no analogous
license on the approval surface.

`constraint-semantic-acts-not-dispositions` **[PROPOSED, required by accepted
goals]** - the reach transfer never consumes `Disposition`. A private semantic
site-act and the eventual disposition are minted from the same proof. This preserves
`pin-no-outcome-as-generator`: a rendered/output outcome never returns as analysis
evidence, while satisfying `rul-rc-reaches-genkill-only-through-decisions`: the typed
decision first chooses whether the original mutation may execute, then that semantic
act feeds effective reach.

`309:law-spine-write-only-during-run` **[existing law, applied here]** - provisional rounds
cannot construct Spine, Plan, a render, a digest, a whylog view, or durable content.
One certified quiescent result converts to final dispositions and writes Spine once.
The existing terminal certifier-trip cleanup remains after that write and before every
projection.

`constraint-wall-policy-is-typed` **[derived from existing mode-gate law]** -
honest-walls and risk-accepted footprint narrowing are different inhabited types, not
a boolean beside optional data. In the honest mode every effective mutator walls total.
The risk-accepted mode is constructible only with the `TrustedFootprints`, resolutions,
dialect, and backing inputs its decision needs. A missing/refused footprint remains a
total wall.

## 1. Why this work exists

The product rule is simple:

```text
elide = reached vouch + converged measurement + reproducible observables + fresh fact
guard = reached vouch + converged measurement + guard-safe rendering + stale fact
run   = no adequate license/check, or a guard/replacement cannot render safely
```

The current implementation answers `fresh fact` in three places:

1. `analysis::effect::Reach` maps `Opaque` to `Top` and exact known writes to a
   `FactKey` set. `classify_one_site` then emits `EstablishAmbient` or
   `EstablishWritten`.
2. `Reach::is_pristine` separately decides the `valid` bit embedded in
   `QueryResolvable`, and W-C re-solves that bit as dead branches are erased.
3. `plan::wall_walk_total` / `wall_walk_survival` examine final `Run` dispositions
   after `disposition_for`, then demote downstream `Replace` directly to `Run`.

The split loses the guard tier. An unmodeled upstream command is `Opaque`, poisons
origin `Reach`, and makes downstream establishes `EstablishWritten`; the existing guard
mint fires. Add an honest verdict function that declines, or one whose probe reports
divergence: the upstream site now gens only its own measurement/establish cell into
origin reach, while its final disposition still runs. Downstream sites first mint
`Replace`; the late wall walk can only turn it into `Run`. The whole-product witnesses
are `guard26-classed-decline-guards-below` and
`guard26-diverged-wall-guards-below`; the control is
`guard26-unmodeled-wall-guards-below` (`trial/r26/predictions.md` section 7).

The flaw is wider than those fixtures. Flag-on footprint collision follows the same
late `Replace -> Run` path, and Query validity answers the same effective-staleness
question from an origin-only state. A narrow late guard mint would preserve all three
mechanisms and add authority-minting to a pass whose current safety property is
demote-only. It is rejected.

## 2. Measured implementation map

### 2.1 Origin and probe

**[+SURE]** `cli::fixpoint::FrozenModel` holds CFG, value flow, AST, `KindIndex`,
predict sets, verdict index, peeled wrappers, and positional live definitions. The
driver builds it once in `cli/src/main.rs`, invokes `classify_round` with an empty
erasure overlay, clones the origin classes/kills, builds vouches, and compiles the
single frozen probe.

**[+SURE]** `analysis::effect::classify_with_why_diags` resolves every node's
`CommandEffect`, applies `ErasedSites` at one seam, derives the invalidator set, then
runs certified forward `Reach`. `CommandEffect::{Establishes,Kills,Opaque}` gen;
`Pure` and `Queries` do not. `Reach` is `Facts(BTreeSet<FactKey>) | Top(ProvId)`;
its exempt `Top` receipt is excluded from equality for termination.

**[+SURE]** `plan::compile_probe` explicitly accepts:

- `EstablishAmbient`;
- vouched `EstablishWritten` (the guard needs its measurement);
- `QueryResolvable`, carrying the origin `valid` bit;
- the existing aggregate and wrapped forms under their own rules.

Verdict-lane choice is site-keyed and frozen. `ship_auto` precedes predict shipping.
This path is not rewritten by the effective analysis.

### 2.2 W-C and admitted records

**[+SURE]** `settle_validity_fixpoint` is a grow-only erasure fixpoint over a frozen
record world. Every round:

1. reclassifies origin effects under `ErasureLedger::overlay()`;
2. re-folds the same admitted probe records with that round's Query validity;
3. mints new `DeadBranchProof`s;
4. appends them to the ledger;
5. stops when the ledger does not grow.

The only cross-round state is the proof-carrying ledger. Intermediate rounds build no
plan or narrative surface. A cap discards the ledger and re-derives the origin answer.
Every reach solve passes `solve_certified`; `CertifierTrip` latches across unseen
rounds.

**[+SURE]** `results::facts_from_sites` always admits a record's Effect verdict to
its site fact, but admits Status only for a non-conflicted Query that the current
validity view marks valid. Establish check status never masquerades as mutator status.

### 2.3 Plan, walls, and Spine

**[+SURE]** `build_plan_walled` currently calls `disposition_for`, pairs each Step
with an `is_mutator` boolean (`class_is_establish_bearing || kills.contains(node)`),
sorts and assigns leaf IDs, then runs one wall walk. Only after the walk does it mint
Spine dispositions. `project_plan` later reads those dispositions; no Spine value feeds
classification.

**[+SURE]** flag-off `wall_walk_total` stores one sticky boolean. Flag-on
`wall_walk_survival` stores a total-wall bit plus ordered `AccumulatedWall`s. The
latter mints `SurvivalWitness`, immediately sends it through
`rederive::recheck_survival`, and only then preserves `Replace`. A re-derivation
failure turns the site to `Run` inside the walk so it walls downstream.

**[+SURE]** `TrustedFootprints` already encodes the admin flag by data absence:
the CLI constructs the value only on `--risk-faultless-skips`; a missing site entry
means total wall. The unified policy must preserve and strengthen this shape rather
than introducing a flag that every consumer remembers to inspect.

**[+SURE]** final certifier-trip cleanup mutates only settled Spine dispositions.
It globally removes `Replace`/`Omit`; a census-unique Guard may stand because it
rechecks live. This remains safe after effective unification: every elision disappears,
all original mutations run, and every standing guard still rechecks at position.

### 2.4 A newly-visible wrong-yes boundary

**[+SURE]** a provisional `Replace` does not itself prove the artifact kills the
original effect. The render may refuse a replacement (the known heredoc class) and
leave the original bytes runnable; `30E` reified this as a render decision but did not
move it before world analysis. W-C's `DeadBranchProof` already carries the stronger
law: no invalidator is erased until `controller_substitutes_away` proves rendered death.

The effective ledger must generalize that law. A candidate replacement contributes
`NoMutation` only after a private proof establishes both its license and its actual
span-render neutralisation. Reading `Disposition::Replace` or trusting a later render
repair is forbidden. This may require hoisting the pure render-feasibility predicate to
the decision seat; it does not authorize a second rendering implementation.

## 3. Required type shape

Concrete names below are illustrative. A builder may improve them while preserving every
unrepresentability property.

### 3.1 Two reach species

**[REQUIRED]** Origin reach and apply-effective reach are distinct Rust types with no
conversion from effective to origin:

```text
OriginReach
   answers which checks may ship and which cells the authored model names

ReachingWalls
   answers which mutation-capable acts may execute before each CFG position
```

`compile_probe` accepts only origin classifications. The final decision builder accepts
effective freshness. No generic `ReachLike` trait should erase this distinction merely to
share a helper.

### 3.2 Effective walls as handles, not embedded evidence

**[PROPOSED preferred shape]** `ReachingWalls` is a finite canonical set of wall
handles (`CfgNodeId` or a stronger `WallId`) with union join. A transfer gens a handle
only for an effective mutation act. The wall policy resolves a handle to total-wall or
trusted-footprint data at the consumer. Benefits:

- the lattice remains small, finite, deterministic, and certifiable;
- attribution identity is decision-relevant and therefore correctly participates in Eq;
- footprint objects, dialect, resolvers, and narrative never enter solver state;
- one reaching set can feed establishment freshness and Query validity under different
  conservative consumer rules;
- the flag-gated authority stays in one policy value outside the lattice.

Embedding `Footprint` or `SurvivalWitness` in the lattice is disfavored: it couples the
solver's equality/termination to narration and knife-tier claims. Reusing origin `Reach`
is forbidden: a `FactKey` set cannot represent a total running wall, ordered wall
attribution, aliases, backing sets, or footprint dialect comparison.

### 3.3 One closed wall policy

**[REQUIRED]** use a closed sum, approximately:

```text
EffectiveWallPolicy::Honest
EffectiveWallPolicy::RiskAccepted {
   footprints,
   resolutions,
   dialect,
   fact_backings,
}
```

No constructor may produce `RiskAccepted` without every authority/input needed by the
survival decision. No `Option<Footprints>` plus independent `Option<Resolutions>` pair
may survive the conversion. A footprint map miss and every refused expansion resolve to
total wall.

### 3.4 Semantic acts, never output dispositions

**[REQUIRED]** the effective transfer consumes a private semantic act, not
`Disposition` and not a rendered tag. The minimum semantic vocabulary is:

```text
NoMutation(NoMutationProof)
MayMutate(WallHandle)
```

The distinction between final `Run` and `Guard` does not belong in reach: both permit the
original mutation to execute. `Replace` and `Omit` become `NoMutation` only through their
own proof variants. Statically pure/query nodes never become wall handles.

The candidate round should mint a private value approximately shaped as:

```text
ProvisionalSiteDecision {
   eventual_disposition_material,
   effective_act,
}
```

There is no `From<Disposition> for EffectiveAct`. The same constructor that checks the
license/render-death conditions returns both products. This is how the old
"outcome is not a premise" law and the new decision-fed reach law coexist.

### 3.5 Proof-carrying no-execution ledger

**[PROPOSED strong lean]** generalize the current erasure chain rather than add a
second bare site set:

```text
NoExecutionProof::DeadBranch(DeadBranchProof)
NoExecutionProof::Replaced(ReplacementDeathProof)
   -> EffectErasureEntry(round, proof)
   -> ErasureLicense
   -> ErasedSites
```

`ReplacementDeathProof` must require a real `ReplaceLicense` plus the one pure proof that
the emitted artifact neutralizes the original effect. Its fields are private; the sole mint
lives beside the provisional decision constructor. The analysis crate continues receiving
only opaque licensed node IDs and never learns why a site disappeared.

If retaining two proof-ledgers makes termination or attribution substantially clearer, that
is builder latitude, but both must combine into ONE `ErasedSites` overlay at the existing
effect seam and both must reset on record-world change. No consumer-specific mask is allowed.

### 3.6 Provisional and settled are different types

**[REQUIRED]** a provisional round cannot call any Spine setter. A settled wrapper is
the only type accepted by the Spine mint/projection adapter:

```text
ProvisionalEffectiveRound   // no Spine/Plan/render API
SettledEffectiveAnalysis   // private constructor: quiescent + certified
SettledEffectiveAnalysis::write_spine(...)
```

A boolean `settled` field is insufficient. Avoid implementing `IntoIterator<Step>` on the
provisional type; convenience there recreates the forbidden projection path.

### 3.7 Execution ownership is explicit

**[REQUIRED census before build]** effective invalidators include CFG nodes that are not
plan leaves: command-substitution internals, redirection writes, and spliced function-body
sites. Their execution is controlled by an enclosing render unit. The present wall walk sees
only leaf classes/kills and cannot express this dependency.

Before implementation, enumerate every `invalidators` member shape and provide one explicit
owner answer:

```text
EffectiveOwner::AlwaysAtNode
EffectiveOwner::Leaf(CfgNodeId)
EffectiveOwner::InlineMember { call, member }
```

Names are latitude; an implicit span/adjacency re-derivation is not. When a replacement or
omission neutralizes a render unit, every owned internal effect must disappear with it. An
ownerless invalidator takes the total-running floor and blocks model shrink; it never guesses.

## 4. Settlement algorithm

### 4.1 Frozen inputs

The complete settlement freezes before its first round:

- source/AST/CFG/value flow and positional live definitions;
- origin effect resolution, backings, verdict lane, vouches, connected/wrapped choices;
- the already-compiled `ProbePlan` and admitted `SiteResults`;
- the record-world influence/scope;
- effective ownership;
- the closed wall policy, including all flag-gated footprint/resolver/dialect data;
- pure render-feasibility inputs needed to prove original-byte death.

No round re-runs funcenv, re-lifts oracles, re-admits records, recompiles a probe, reads I/O,
or mutates those values.

### 4.2 One grow-only outer loop

**[PROPOSED algorithm; required behavior]** replace `settle_validity_fixpoint` with a
settlement whose only cross-round authority is a grow-only no-execution ledger:

1. Apply the ledger's one `ErasedSites` overlay at `resolve_node_effects`' existing seam.
2. Recompute the residual origin classification from scratch and certify every solve.
3. Solve `ReachingWalls` from the residual effective invalidator set and certify it.
4. Derive effective Query validity from reaching walls. V0 keeps the conservative floor:
   any reaching effective wall invalidates Query Status; footprint disjointness does not
   newly license Query substitution in this stage.
5. Fold the frozen records through that validity view.
6. Derive effective freshness for each establish from reaching walls plus wall policy.
7. Build provisional decisions and semantic acts, including guard/run on stale facts.
8. Mint every newly proven dead-branch and rendered-dead replacement proof.
9. Append only new proofs. If the ledger grew, discard all provisional products and repeat.
10. If it did not grow, seal the certified round as `SettledEffectiveAnalysis`, then and only
    then write dispositions, survival outcomes, and final narratives to Spine.

This is one settlement even if implementation uses two inner certified solves. A design in
which W-C "settles", then effective reach makes another Query valid without returning to the
dead-branch proof step is incomplete.

### 4.3 Monotonicity argument

The finite ledger contains CFG invalidators. Each growing round proves at least one new
original mutation cannot execute in this fixed record-world. Effects only disappear:

- reaching-wall sets can only shrink;
- a Query can move invalid -> valid, never back;
- a fact can move stale -> fresh, never back;
- a site may improve Run/Guard -> Replace or live -> Omit, never lose a proved
  no-execution entry;
- a survival is admitted to the ledger only after independent re-derivation confirms it,
  so no later recheck retracts an erasure.

The bound is the number of effective invalidators, not an arbitrary constant. Cap hit or any
monotonicity violation discards the entire ledger and derives the maximal-effects answer:
all mutation-capable originals remain active; downstream sites guard where independently
licensed and otherwise run. No partial ledger or intermediate decision survives.

### 4.4 Effective-solve inconsistency floor

Every effective reach solve routes through `solve_certified` and records into the existing
run-wide `CertifierTrip`. Add a closed `SolvePass::EffectiveReach` (or its exact current
equivalent) so diagnostics and Spine certification do not call it generic "reach".

An inconsistent effective answer is inadmissible for freshness or survival. Its named local
floor is all facts stale across potential mutations: no Replace/Survive/Omit minted from that
answer; a census-valid live Guard may still be constructed from the independent vouch and
probe measurement, otherwise Run. Terminal trip cleanup remains the final global net.

## 5. Freshness and disposition

### 5.1 Establish sites

For an establish fact at site S, consume the reaching wall handles at S:

- empty set -> `FreshClean`;
- honest policy + non-empty set -> `Stale(TotalWall)`;
- risk policy + any missing footprint -> `Stale(TotalWall)`;
- risk policy + footprint/backing collision -> `Stale(Poisoned|MayAlias)`;
- risk policy + every wall disjoint and reference-confirmed ->
  `FreshSurvived(SurvivalWitness)`.

`Fresh*` may feed the ordinary replacement mint. `Stale` may feed only the existing guard
mint, which still demands a reached `ByVouch<VerdictVouch>`, a converged measurement, and
guard-safe rendering. Unknown/diverged/unvouched remains Run. The mutation's own fact and
the verdict measurement remain separate values throughout.

### 5.2 Query sites

Origin validity still rides the frozen probe record because that is what shipped. Effective
validity overrides it only for folding the already-admitted Status. At v0:

```text
effective validity = no effective wall reaches this Query
```

This deliberately preserves the existing `is_pristine` conservatism; using footprint/backing
disjointness to relax a Query is a separate license widening. Delete or retire
`Reach::is_pristine` as a final apply authority once the effective consumer owns the answer.

### 5.3 Guards and downstream walls

A Guard's read-only check is not the wall; its untouched fallback mutation may execute, so
its semantic act remains `MayMutate`. It participates in downstream `ReachingWalls` exactly
like Run. This can produce ordinary guard cascades. It never emits conditional-tail state or
special plan plumbing (`constraint-plan-surface-stays-readable`).

### 5.4 Aggregates and non-leaves

Member-loop and inline-call replacements erase multiple establishes. Preserve
`rul-every-erased-establish-is-vouched`: every erased mutation must carry its own reached
vouch and fresh/effect proof, identity- and cardinality-matched. Never use one representative
fact to erase an aggregate's effective walls. Where the current aggregate representation
cannot express universal effective freshness, the whole aggregate runs; add the typed proof
before recovering value.

Connected-pipe and expansion-internal effects follow their explicit effective owner. A
subsumed stage whose governing replacement really neutralizes it may disappear only through
that owner's proof.

## 6. Survival and independent re-derivation

`survival::wall_verdict`'s production relation and `rederive::wall_spares`'s reference
relation keep their semantics and independence. What changes is their seat:

1. Effective freshness gathers reaching wall handles from the certified CFG solve.
2. The risk policy resolves handles to trusted footprints and constructs the target Backing.
3. Production proves every crossing disjoint and mints a candidate witness.
4. `recheck_survival` consumes that witness by value before any no-execution proof can mint.
5. Confirmation returns the same witness; disagreement yields `Stale`, never a weaker
   survival, and therefore no effect erasure.

Move the `SurvivalWitness` private mint from "only wall walk" to "only settled effective
freshness"; keep the lexical gates that prevent `rederive` from reaching it or production
compare helpers. A reference disagreement is recorded on final Spine and naturally cascades:
the site remains Guard/Run, its wall handle remains active in the next round, and downstream
freshness is recomputed before settlement.

`SpineSurvival` can remain the final account shape. It may need wording/name changes from
"walk" to "effective reach"; that is not durable schema growth. Do not add durable fields in
this stage. If exact attribution cannot fit existing in-memory records, stop at
`rul-durable-contents-reviewed-before-design` before changing a View or whylog content.

## 7. Spine, narratives, and rendering

The settlement accumulates only semantic proofs and compact cause handles. Narrative records
are minted from the settled round, never appended per provisional round. Required final
accounts:

- each final disposition, as today;
- each clean/survived/demoted/re-derivation survival result, as today;
- wall formation for each effective mutation act that reaches a dependent decision;
- Query invalidation and effective-reach collapse operands, through existing narrative
  species where truthful;
- the certifier result and terminal cleanup, as today.

Do not put narrative values in `ReachingWalls` or its equality. Attribution handles that
change the semantic footprint policy are decision data and belong in the wall set; prose,
counts, examples, and collapse explanations are post-settlement output.

The final plan surface gains no new shell form. Expected behavior changes are ordinary
`Run -> Guard` and, where a formerly late wall was genuinely absent, `Guard/Run -> Replace`.
Every such change remains visible in the disposition smoke diff and e2e run set.

## 8. Implementation work order

### Stage 0 - census and red-first specification

Before conversion:

1. Enumerate every producer/consumer of `Reach`, `SkipClass::{EstablishAmbient,
   EstablishWritten,QueryResolvable}`, `invalidators`, `kills`, wall walks,
   `WallVerdict`, `SurvivalWitness`, `facts_from_sites` validity, and certifier cleanup.
2. Enumerate every invalidator's effective owner, including non-leaves. A non-empty census
   is a gate; source-shape allow-lists are file-local and two-way if typing cannot express it.
3. Freeze a deterministic decision-state dump over current dispositions, Query validity,
   wall/survival outcomes, and effective invalidators. Review-only, build-to-kill, following
   `309`'s Spine migration precedent.
4. Confirm the two `guard26-*` XFAILs fail for the named reason and the unmodeled control
   remains green.
5. Add red-first ownership-seat tests for:
   - modeled decline and modeled divergence -> downstream Guard;
   - flag-off total wall;
   - flag-on disjoint survival and collision -> Guard;
   - Guard remains a downstream wall without emitted bookkeeping;
   - upstream Replace and Omit remove their effective walls and cascade;
   - effective Query validity flips only as walls disappear;
   - replacement render refusal never erases its effect;
   - internal/spliced/redirection effects follow their owner;
   - provisional rounds cannot write Spine (compile-fail or lexical fence);
   - an effective solve inconsistency takes the named floor.

### Stage 1 - types and certified domain

Add the distinct origin/effective types, closed wall policy, semantic site-act,
proof-carrying no-execution entry, explicit ownership, and provisional/settled boundary.
Implement and unit-test `ReachingWalls` plus its certified solve before changing plan output.
If the domain touches translated algebra files, STOP for the verified-core access rules;
the preferred plan-local domain should not require minispec or Kani changes merely by analogy.

### Stage 2 - unified settlement

Replace `settle_validity_fixpoint` with the grow-only settlement, retaining frozen records,
origin probe, erasure provenance, cap-to-origin discipline, and `CertifierTrip`. Move Query
validity, effective freshness, guard fallback, and survival recheck into the loop. Keep every
intermediate product private and output-free.

### Stage 3 - delete the split machinery

Delete `wall_walk_total`, `wall_walk_survival`, their `is_mutator` side channel, and any final
apply authority still read from origin `Reach::is_pristine`. Rename or reshape
`EstablishAmbient`/`EstablishWritten` so their remaining role is unambiguously origin/probe
classification; do not leave names that imply they are final apply freshness.

Re-home `attribute_cascades`: its cause population is no longer only dead-branch erasures.
Final attribution comes from effective wall/no-execution proofs. Do not broaden the old
function until its name and operands tell the truth.

### Stage 4 - behavior close and cleanup

Promote `guard26-classed-decline-guards-below` and
`guard26-diverged-wall-guards-below`; retire the defect twin after its negative lesson has an
ownership-seat test. Run a complete disposition/run-set diff. Every movement outside the
enumerated effective-wall population is a finding, never refactor churn.

Delete the temporary state dump after review. Update `FORFEITS`, `ANALYZER-NEEDS`, current
steering law, and `28Q` stage status to the as-built truth. Do not leave a compatibility wall
walk or an old/new switch.

## 9. Gates and review

Hot-loop and completion use the current mise lifecycle tasks; never hand-derive invocations.
Required close evidence:

- focused native tests for the domain, ownership, proof mints, and provisional/settled fence;
- both guard26 XFAIL promotions and the unmodeled control;
- query-validity and W-C corpus unchanged except enumerated cascades;
- survival differential and re-derivation demote-only gates;
- solve-certifier fault injection for effective reach;
- Spine projection/digest/why agreement;
- full e2e/loom corpus and `mise run both gate:full-quiet` foreground;
- `mise run gate:arc` at conductor close, from the populated branch;
- adversarial review centered on wrong erasure, owner loss, flag-off footprint access,
  provisional output leakage, and reference-check bypass.

No golden bless proves a guard or elision correct. Behavior drift is reviewed from the
decision/run-set diff first, then blessed only through the sanctioned path.

## 10. Boundaries and stop conditions

Out of scope:

- conditional-tail or generated wall-state plan emission (NACKED);
- new oracle syntax or metadata;
- stage-iii lifecycle/context authored surface;
- new Query footprint-sparing authority;
- changes to coordinate compare, dialect, resolver, or backing semantics;
- changes to at-most completion speech;
- durable/whylog content growth;
- bundle/artifact implementation from `30I`;
- recovery after a certifier disagreement.

Stop and report if:

- some invalidator has no statically representable execution owner;
- replacement death cannot be decided before the effect is erased;
- a monotone no-execution ledger cannot express an existing aggregate/survival behavior;
- effective reach requires changing a minispec statement, comparison law, or reference-model
  semantics rather than only reseating its consumer;
- independent re-derivation cannot demote before the no-execution proof mints;
- an effective solve cannot use the existing certifier without weakening its question;
- truthful attribution requires whylog/DurableView growth;
- an expected pure refactor changes probe bytes, site IDs, vouch identity, or definition
  binding.

Builder latitude:

- exact type/module names and whether `ReachingWalls` uses `CfgNodeId` or a stronger handle;
- one combined loop implementation versus a visibly nested pair of certified solves, provided
  the outer settlement is one grow-only fixpoint and Query/effect cascades close;
- one generalized proof ledger versus two proof ledgers feeding one overlay;
- the compact settled attribution representation, within existing in-memory/durable laws;
- test allocation between native, DST, and irreducible e2e seats.

The acceptance criterion is semantic and structural: one effective wall fact reaches every
apply-time freshness consumer; no output disposition feeds analysis; no provisional value can
reach Spine; and no obsolete wall mechanism survives beside the new one.
