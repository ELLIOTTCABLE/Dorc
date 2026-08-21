# 30L - Elision regions: universal descent into function bodies, staged for loop propagation

> Tier: focused design plan, minted from the human's direct rulings in the 2026-08-20
> design sitting. Root docs and `spike/CLAUDE.md` outrank it. **[TYPED]** means
> human-stated; **[DERIVED]** is a mechanical consequence of typed law; **[PROPOSED]**
> remains implementation latitude. Every type/field name herein renames freely
> (`rul-strawman-formats-no-compat`).
>
> Scope: elision descends into function bodies across the book-custody surface, under
> one universal decision per authored region. Loop propagation is NOT implemented here,
> but staging it — the types, the reserved dimensions, and the red-first pins, in every
> place this work touches — is a co-equal deliverable with the green function-descent
> work (§7). This plan adds no function specialization, cloning, renaming, generated
> dispatch, or other generated shell; the executable edits remain the ordinary Dorc
> forms (retain original bytes; insert one authored guard; replace/comment one authored
> region).
>
> Parent architecture: `28Q:stage-effective-world-reach` supplies per-instance CFG
> effects, effective walls, execution ownership, and the grow-only settlement; `30I`
> supplies static loading, definition identity, custody, source loci, and (later)
> bundle projection and artifact forms.

## 0. The rulings in one screen

**`rul-elision-region-is-the-unit` [TYPED 2026-08-20]** - the **elision region**
is the minimum granular unit Dorc is willing to elide. The name is stable; its extent
may evolve. Current extent: the command-leaf shape — a pipeline, or an
observable-consuming chain, stays one region (its interior byte-streams and statuses
interlock; sub-region replacement is not offered). This plan changes the region's
VENUE, not its GRAIN: regions now live inside function bodies, and one region may
serve many invocation instances. Nothing here shrinks the grain below the leaf shape.

**`rul-shared-region-needs-universal-must` (né rul-shared-line-needs-universal-must)
[TYPED]** - a region inside a function body may transform only when BOTH universals
hold:

1. every CFG route to every statically possible invocation instance of that region is
   closed and known; and
2. every license-bearing property holds, at `Must` grade, on every such route.

Any dynamic/unresolved/unbounded route, any failed property, or any disagreement about
the one source edit means **Run** for that region. There is no `May` transformation
and no per-invocation specialization.

```text
region_decision(r) = meet_Must {
   route_decision(r, route)
   | route in every_static_route_to(r)
}

meet_Must containing any unknown/failure/disagreement => Run
```

**`rul-region-universe-is-book-custody` [TYPED 2026-08-20]** - elision regions
exist ONLY on the book-custody surface: the book plus its non-dorc-lang sourced tree.
dorc-lang files are contracted non-mutative, so their interiors are not
attention-product surface and receive no elision work — ever, at any tier. The
attention product exists to review DEFINITELY-MUTATIVE code the admin wrote, right
now; "is this supposedly-non-mutative code secretly mutative" is a different review
category that Dorc's contract structure deliberately separates out. A book call INTO
an oracle-file helper remains an ordinary book region and takes ordinary call-level
disposition; the helper's interior does not.

**`rul-no-specialized-shell` [TYPED]** - never clone a function per call, rewrite
call names, inline executable source at call sites, or emit argument-dispatch
branches. One authored region receives at most one shared transformation valid for
every invocation. (Consequence of the authorship law: there must always be a
human-authored line answerable for anything that runs — `rul-ternary-verdict`,
IMPLEMENTATION.md.)

**`rul-whole-helper-is-derived` [TYPED]** - full elision of a helper is derived,
never primitive: it falls out when every executable region in the body, through every
route, is universally non-executing AND each call's consumed call-level observables
are reproducible. Partial body transformation is the ordinary case: agreeing regions
replace/guard independently while any failed region remains runnable.

**`dir-loop-propagation-is-staged-here` [TYPED 2026-08-20, titular]** - every
builder and conductor on this plan carries a second, co-equal deliverable: the
representation and acceptance battery must be loop-propagation-READY when this stage
closes. The iteration/member dimension exists in the route types; loop populations are
typed `Open` with red-first cells whose greening trigger is the propagation lane; no
seat, API, or witness shape may need re-keying when propagation arrives. Propagation
itself (turning a literal `for` list into a closed member population) is a later lane
— wanted soon — and must land as a value-plane change only.

## 1. Why this design exists

The analyzer already creates per-call CFG instances for supported static function
calls: positional arguments bind into the spliced body, internal commands classify and
probe separately (`site N.M` records), and every internal mutation participates in
effective reach. But the plan collapses those facts into one all-or-nothing CALL
disposition, because the call is the only render unit:

```sh
main() {
   converge_a
   mutate_b
   converge_c
}
main "$@"
```

```text
as-built:  all body establishes removable => Replace(call)
           anything live in the body      => Run(call)
required:  converge_a and converge_c replace at their own regions;
           mutate_b runs; the call runs the residual body
```

The all-or-nothing floor runs too much — a precision/value gap, never an
under-execution. It is severe for the `main() { whole book; }; main "$@"` shape, which
is what the most disciplined authors write (style guides mandate it), and for factored
helpers generally: one live or unknown command inside the wrapper forfeits the
attention product for the entire book.

Two standing facts bound what descent alone recovers, and belong in every value
conversation about this stage:

- **`ctx-errexit-is-authored-speech` [TYPED 2026-08-20]** - `set -e` is honored as
  authored speech: it consumes every command's status, so a bare converged mutator
  under errexit does not elide, inside a body exactly as at top level. `|| true` is
  the authored, per-line consent that releases that observable
  (`StatusInvariant`); a marked verdict declares its tool-rc and substitutes from
  probe provenance (`StatusRelaxable`). This is a design decision, not a forfeit: the
  experienced author already knows how to type the inversion, and we honor the
  strict-mode book's demand that every status matter. Descent raises the wrapped
  book's ceiling to match top level; it does not exempt anyone from errexit's speech.
- The census must actually admit realistically-factored books (§3.4): budgets sized
  for an earlier era's hint machinery are re-cut deliberately at this stage, or the
  machinery is universal in theory and refuses the motivating book in practice.

## 2. Identities

**`rul-two-identities-never-conflated` [DERIVED]** - analysis and rendering ask
different identity questions:

```text
ElisionRegion   = the one authored source span all instances would edit
                  (definition-keyed; file+span; book-custody only)

RouteInstance   = one analyzed execution instance of a body region
```

```text
RouteInstance {
   definition: DefinitionId,        # never a name; same-named defs never combine
   invocation: InvocationId,        # separates argument/context instances
   cfg_node: CfgNodeId,             # analysis-local
   region: ElisionRegion,
   iteration: IterationSlot,        # reserved dimension, typed NOW (§7)
}
```

`DefinitionId` and frame-live resolution come from `28Q` stage-i; source/file identity
composes with `30I`'s loci once sourced books project. `LeafId` remains the outer
call's execution/probe identity, untouched: existing `site N.M` records stay the
per-call/member evidence namespace, and this plan neither widens `LeafId` nor lets
analysis-internal nodes masquerade as plan leaves. `spike/CLAUDE.md inv-leaf-seam`
gains the matching amendment when this lands: execution leaves and elision regions are
distinct identities; the Step-level map stays injective.

**`inv-closed-route-set-never-empty` [DERIVED]** - route populations are typed:

```text
RoutePopulation = Closed(NonEmpty<RouteInstance>) | Open
```

`Open` always runs. Universal quantification over an empty set is forbidden; an
unreached definition acquires no vacuous authority.

**`fence-decisions-are-plan-scoped` [DERIVED]** - every shared decision is minted
per-(target, plan). Plans are per-`TargetedBook` (`30I` §7.6), so this holds by
construction; the fence exists so no future artifact or cache ever carries a shared
decision across targets, where the meet would silently need a host dimension.

## 3. The census: what "every route" means

Two distinct universal meets.

### 3.1 Within one invocation

The ordinary CFG/dataflow solver already meets predecessor states at branches and
loops; the lattice meet IS the path quantifier — no exponential enumeration. A body
region's per-invocation proof is `Must` only if the property holds through every CFG
route that can reach it; a property held on one arm and unknown on the other meets to
failure.

```sh
f() {
   if condition; then prepare_a; else prepare_b; fi
   converge_x
}
```

### 3.2 Across invocation instances

All statically possible calls resolving to one definition contribute one per-instance
answer each:

```sh
install_pkg() { apt-get install "$1"; }
install_pkg nginx
install_pkg curl
```

The region `apt-get install "$1"` receives one shared edit only if both instances
admit exactly that edit. A proof for `nginx` alone says nothing about the shared
region.

**`rul-call-census-must-be-closed` [TYPED]** - "all static invocations agree"
requires a closed invocation census. Any construct that may invoke the definition
outside the enumerated call set makes its body regions `Open`:

- dynamic command position;
- unresolved source/load;
- authored `eval`/opaque code-carrier;
- unresolved callback or alias;
- recursion, or an exceeded call/splice budget;
- unbounded/unresolved loop invocation (§7);
- trap or string execution that may name the function.

Silence never means "no other calls." External commands cannot invoke shell functions,
so unmodeled COMMANDS do not open the census; only shell-level dynamic constructs do.

**`rul-census-is-execution-not-mode` [DERIVED]** - the route population is defined
over everything that may EXECUTE in the produced program — never over what a mode
chose to check. Future partial-scope emissions (`kSCOPE-asked`, `dorc bump`-shaped
minimal programs) derive their own census over the program they actually emit; a
checking-scope filter is never a census filter. Stated now, in execution terms, so the
law is stable under whatever shape that later work takes.

**`pin-book-argv-value-plane` [PROPOSED, named-not-built]** - book positionals
(`"$@"`/`$1`…) currently read ⊤ (`UnresolvablePositional`) in book flow, so `main "$@"`
threads ⊤ into the body wherever positionals are consumed. Book argv is
controller-known, authored-before-contact input (`30I` §2.4), so admitting it to the
static value plane is a legitimate future widening — but it is winner-shifting
licensure surface and gets its own ruling; nothing here assumes it. Body regions that
do not consume positionals (the overwhelming majority) are unaffected either way.

### 3.3 Probing stays dispatch-agnostic

Measurements key to site/instance identity (`site N.M`), never to a dispatch batch.
Nothing in this plan may assume probe results arrive as one shipped set: the probing
phase remains a single phase between dispatch and plan-quiesce, but may someday
comprise multiple dispatch sets, later ones answering questions raised by earlier
ones. Equally, nothing here designs that — repeated/multi-dispatch probing carries its
own review gate (`spike/CLAUDE.md rul-repeated-probing-reviewed-before-design`). The
obligation on this stage is purely representational: dispatch-count never appears in a
route proof, witness, or record key.

### 3.4 Budgets sized to the product, not inherited

**`req-census-admits-the-wrapped-book` [DERIVED]** - the supported floor is every
statically resolved, finitely instantiated call graph the splice analysis admits, over
a Closed census — and that admission must be RE-SIZED at this stage, not inherited.
The standing budgets (`analysis/src/cfg.rs` inline_budget: depth 2, 64 nodes/site,
1024 nodes/book) predate this stage's purpose and refuse `main → task_fn → helper` at
depth 3 — the motivating shape. The stage brief sizes, before the meet machinery is
built:

- realistic wrapped-book depth and node counts (measure against corpus-shaped
  strawmen, then set the constants deliberately);
- whether per-invocation body CLONES remain the representation, or route instances
  become overlays on one lowered body (the positional-overlay machinery in
  `analysis/src/value.rs` is the precedent) — the meet multiplies instances, so this
  decision precedes it;
- over-budget stays proportional refusal, never a cliff: `Opaque` with the diagnostic
  naming the exceeded budget, exactly as today.

Cardinality one falls out of the general meet; no branch or API keys on route-set
cardinality (`pin-no-singleton-special-case`).

## 4. The per-route property product

**`rul-every-property-meets-universally` [TYPED]** - the shared decision does not
meet only a final `Disposition`. Every license-bearing property must be `Must` through
each route before the shared edit mints. Per route, the proof covers at least:

```text
RouteRegionProof {
   reachable_and_closed,
   exact_definition_and_body_site,      # frame-live resolution; DefinitionId-keyed
   phase_appropriate_measurement,       # verdict-primacy-consistent (§4.1)
   reached_vouch_or_read_substitution_proof,
   effective_freshness,                 # this instance's reaching-wall crossing
   effect_ownership,
   consumed_observables,                # status/stdout/stderr/effect, incl. bindings
   replacement_or_guard_bytes,
   render_feasibility,
   influence,
}
```

Any absent, unknown, stale, declined, conflicted, render-refused, or differently keyed
member rejects the shared edit for that region.

**`rul-route-proofs-are-projections` [DERIVED]** - the proof product is a
PROJECTION of per-instance facts the engine already computes (spliced argv, effects,
reach, vouches), never a second bookkeeping plane populated by hand at each seat. Only
the meet result materializes. And the mint discipline follows
`30Kb:required-private-decision-projects-twice`: one private decision
conclusion/witness is computed from license + freshness + render-death, and BOTH the
public `Disposition` and the settlement-facing semantic act project from it; the
effective side never accepts the public output enum as input.

### 4.1 Measurement and primacy

Verdict-primacy (`28Q` §4, `307:rul-primacy-moves-the-body-never-the-cell`) governs
which body measures at a vouched site, inside a function body exactly as at top level.
**`req-member-lane-ruling-precedes-consumption` [DERIVED, SATISFIED 2026-08-20]** -
`30La` repaired the member-loop and inline-call lanes before this plan consumes them.
Predict-derived cells and topology remain fixed, while an exact all-vouched establish
population ships reached verdict bodies as its ordered measurements; a partially vouched
population cannot replace, selected verdict shipping is all-or-nothing, and query-only
substitution stays separate. Shared-region
decisions consume this repaired primitive, never the former predict-measured interim.

### 4.2 Effects and freshness

Each route instance consumes the effective reaching-wall set at its own CFG position.
Replacement of one region retires the invalidators owned by EVERY instance of that
region, and only after universal agreement mints the shared no-execution proof (§6).
One invocation's clean world cannot erase a region that is stale in another
invocation. Per-instance aggregate freshness machinery (the `30K` member-by-member
crossings) is usable evidence WITHIN each instance; it never substitutes for the
across-invocation meet.

### 4.3 Observables

**`rul-shared-edit-reproduces-every-route` [TYPED]** - one shared replacement must
preserve the consumed observable tuple `{Effect, Status, Stdout, Stderr}` on every
route, from probe provenance only (`inv-probe-sourced-values`):

- Status: the existing trichotomy applies per route and meets universally —
  `StatusRelaxable` substitutes a probe-sourced tool-rc exactly (⊤ blocks);
  `StatusInvariant` (`|| true`) is the authored consent and never blocks;
  `StatusIterated` blocks unconditionally. Errexit receives no special path: it is
  status consumption plus the CFG failure edge (`pin-errexit-rides-status-law`).
  If two routes consume two different measured statuses, no single stand-in exists:
  Run.
- Stdout/stderr consumed on any route without a common real-provenance stand-in: Run.
- Shell-state bindings (`x=$(cmd)` consumed downstream) are value-predictions
  (`275`); a consumed binding whose value cannot be reproduced with probe provenance
  blocks the region.
- Function composite status: transformed body regions must reproduce every status
  consumed by body control flow AND by each call context — a call in `if f; then`
  contributes a stricter route than a call whose status is dead.

### 4.4 Influence

**`rul-shared-influence-never-launders` [DERIVED]** - a shared decision carries
the most-influenced grade among all contributing routes. One uninfluenced route never
cleanses a host-influenced sibling. Every contributing route stays available to the
narrative/locator plane even where the decision stores the joined grade.

### 4.5 Guards: the divergent-instances valve

Where route facts DIVERGE but every route admits the same invocation-parametric guard
at the region, Guard absorbs what Replace cannot — the runtime dispatch happens per
invocation, inside sh, authored:

```sh
f() {
   ( tool__is_converged "$1" ) || tool "$1"
}
```

This is why the universal meet is not lossy in practice: Replace demands universality;
Guard covers the mixed-world case at the cost of attention only (`KNOBS:kHALVES` —
the guard-half is the permanent sister fallback). Constraints:

- all routes resolve the SAME guard definition and custody, frame-live at every
  instance (`funcenv::LiveDefinitions`; `pin-guard-resolution-is-frame-live` — a
  subshell re-source giving two instances two live verdict definitions refuses the
  shared guard);
- the same source-level argv expression (positional parameters re-bind naturally per
  invocation; a per-call literal guard never installs into shared source that also
  serves other operands);
- guards remain authored-oracle bytes ahead of untouched original bytes; they mint no
  values (`guards-mint-no-values`); no generated dispatch.

## 5. The decision algebra

**`rul-region-edit-is-one-must-result` [DERIVED]** - group route proofs by
`ElisionRegion`, then mint exactly one decision:

```text
SharedDecision = Replace(CommonStandIn) | Guard(CommonGuard) | Omit | Run
```

The meet is biased to Run:

```text
all routes prove the same observable-preserving Replace  => Replace
all routes prove source-level Omit                        => Omit
all routes admit exactly one common parametric Guard      => Guard
anything else                                             => Run
```

Replace/Omit equivalence is semantic, not tag equality: replacement bytes, stand-in
observables, effect identity, and render span must agree. Guard equivalence includes
guard body identity/custody and source-level invocation bytes. No `May` result is
rendered or fed into effective reach, anywhere.

## 6. Settlement, not a render post-pass

**`rul-shared-agreement-precedes-wall-retirement` [TYPED]** - per-instance
decisions may never be merged after settlement. If one route forces Run, the region's
effects remain walls in every relevant invocation. Round shape:

1. derive per-instance route proofs from the frozen invocation/CFG population;
2. group by elision region;
3. meet every property to one shared decision;
4. mint no-execution proofs only for universally agreed Replace/Omit;
5. project each shared act onto EVERY owned instance invalidator, atomically;
6. solve effective reach; fold records;
7. repeat until the grow-only no-execution ledger is quiescent;
8. write shared decisions and route attribution to Spine once, at quiescence.

The population freezes before round 1; settlement may prove more regions
non-executing, never discover an invocation or change binding. The fixpoint is
monotone by construction: proofs only grow, and
**`inv-no-posthoc-shared-demotion` [DERIVED]** forbids any per-instance
replacement entering the ledger before shared agreement — otherwise a later Run meet
would need to re-introduce walls, breaking the grow-only proof.

**`pin-shared-witness-spans-instances` [DERIVED]** - the shared replacement's
witness carries the exact ordered union of erased establishes across ALL contributing
instances, identity- and cardinality-matched (the `rul-every-erased-establish-is-
vouched` discipline, one level up). A per-call/per-member witness never stands in for
the shared one; the vouch proof and the freshness proof stay aligned without
conflating. This is the same mistake-shape the aggregate lane already had to repair
once (`30Kb` §1) — the pin exists so it cannot recur an abstraction level higher.

Two settled-residue riders discharge in this stage, because it rebuilds their seats:

- **`req-backings-freeze-at-probe-boundary`** - fact backings freeze with the
  probe/model inputs before settlement; every round consumes the same backing account
  (`30Kb:required-backing-is-frozen-beside-policy`).
- **`req-wall-narrative-gains-region-operand`** - wall-formation narrative gains
  the truthful non-`LeafId` operand (region/instance identity through
  `ExecutionOwner`), closing the honest/non-leaf narration gap
  (`30Kb:finding-nonleaf-walls-have-no-account-seat`).

## 7. Loop propagation: staged now, implemented soon (titular deliverable)

What ships IN this stage, co-equal with the green descent work:

- `IterationSlot` in `RouteInstance` — the member/iteration dimension exists in the
  types, so a syntactically singular call inside a loop is representable as many route
  instances without re-keying `ElisionRegion` or any witness
  (**`rul-one-call-site-is-not-one-evaluation` [TYPED]**);
- loop-generated populations typed `Open` today: `for`/`while`/`until` bodies invoking
  functions contribute `Open` (region runs), with no loop-specific optimistic default
  anywhere;
- red-first cells for the literal-list shape, committed EXPECTED-OPEN with the named
  greening trigger being the propagation lane (`xfail_until`, round-marker horizon):

```sh
for pkg in nginx curl; do
   install_pkg "$pkg"
done
```

  expected now: population Open, shared region Run; expected after propagation: two
  member route instances, universal meet over both;
- `StatusIterated` continues to block single-status substitution regardless of
  propagation.

What the future propagation lane MAY do: turn `Open` into `Closed(members)` for
finite, fully enumerated, propagated literal populations — a value-plane change only.
What it may NEVER do: reinterpret an already-closed set by dropping members, or touch
any identity/witness shape (that immutability is what this stage's staging buys). The
`30K` member-loop aggregate machinery (ordered member identity, member-by-member wall
crossing) is the settlement precedent the propagation lane extends.

Relation to standing law: the r23 atomic-command axiom stands — multi-entity
granularity comes from authored loops, never from splitting one command. Propagation
over authored loops is precisely what makes that answer pay; this stage makes it
representable.

## 8. Rendering and plan honesty

**`rul-edit-authored-definition-once` [TYPED]** - render the shared transformation
at the authored function-body region. The definition stays in place; calls stay calls.
This preserves shell scoping, positional parameters, `local`, `return`, cwd/options,
and the off-ramp. Never render per-call specialized bodies.

The plan surface shows:

- the transformed region at its definition locus, its decision marked as universal
  over its contributing invocations (route count on pull/why surfaces);
- every call line that may execute, or whose call-level observables remain live —
  visible, always (`rul-attention-honesty`);
- a region Run on any route: visible and executable in the shared body; a Guard:
  visible. Only universally proved Replace/Omit saves attention at the definition.

Whole-helper elision stays derived (§0): when every body region is universally
non-executing and a call's consumed call-level observables are reproducible, that call
independently replaces with the ordinary stand-in; the inert definition remains
authored text; no special delete-helper mechanism exists.

Single-stream presentation note (owed to the artifact-forms work, NOT this stage): on
a fully flattened plan, dorc-lang material is literally present in the one stream, but
it is not attention surface (§0 region-universe ruling). The flattened render should
separate "pay attention here" from the would-be-bundled non-mutative miscellany —
lifting oracle material together (top-lift + munge per the emission planner's
placement vocabulary, `28Q:pin-emission-planner-universal`), section boundary
comments, or similar. Shape NYI; the obligation is recorded here so the artifact work
inherits it.

## 9. Probe, evidence, and attribution

Existing per-call body records (`site N.M`) remain the evidence namespace; a shared
decision consumes a closed exact set of instance records, and one record never speaks
for another invocation (`pin-probe-site-identity-unchanged` — the transformation
never re-keys frozen probe records; §3.3's dispatch-agnosticism rides the same pin).

Spine carries both levels:

```text
SharedRegionDecision {
   region,
   disposition,
   contributing_routes: NonEmpty<RouteProofRef>,
}
```

`dorc why` moves both directions:

```text
definition region -> every invocation proof that licensed this edit
call instance     -> the shared region edits it executes
```

This is the first forcing consumer of the participant/locator distinction beyond
`LeafId`: `LeafId` is call/execution identity; `ElisionRegion` is authored edit
identity; `RouteInstance` is analysis identity. Locators compose through `30I`'s DAG
(definition loci in sourced book files included).

## 10. Supported floor and exclusions

Supported: every statically resolved, finitely instantiated call graph admitted by the
(re-sized, §3.4) splice analysis, over the book-custody region universe, with a Closed
census. Never phrased as "single invocation support" — cardinality one falls out of
the meet.

Run-floor exclusions:

- dynamic/unresolved calls; recursion; exceeded (deliberately re-sized) budgets;
- open loop populations (§7);
- regions shared with any unenumerated invocation;
- irreconcilable route observables; disagreeing guard definitions/argv forms;
- render-overlapping region edits;
- unknown function binding or definition custody;
- any route whose analysis or certification fails (cap/failure takes Run — the
  certifier's question is never weakened).

Not built here: cross-run state, repeated/multi-dispatch probing, call
specialization, generated dispatch, reordering, non-sh metadata, dorc-lang-interior
elision, loop propagation itself.

## 11. Acceptance laws

The stage is complete only when all hold:

- `pin-no-singleton-special-case` - no branch or API keys on route-set
  cardinality one;
- `pin-open-route-runs` - one unenumerated invocation forces Run for every region
  it may execute;
- `pin-every-route-meets` - mutating any one route property to failure reddens
  the shared-elision pin;
- `pin-shared-edit-before-erasure` - no per-instance effect retires before
  universal agreement;
- `pin-shared-witness-spans-instances` - the witness carries the exact ordered
  cross-instance establish union; a per-call witness never substitutes (§6);
- `pin-common-replacement-observables` - differing consumed statuses/bytes Run;
- `pin-errexit-rides-status-law` - no separate errexit path exists in either
  direction (no optimistic exemption; no special block beyond status consumption);
- `pin-influence-joins-most` - shared decisions never lower influence;
- `pin-guard-resolution-is-frame-live` - divergent live guard definitions across
  instances refuse the shared guard (§4.5);
- `pin-census-is-execution-not-scope` - the census quantifies over what may
  execute in the produced program (§3.2);
- `pin-region-universe-excludes-dorc-lang` - no elision region is ever minted
  inside a dorc-lang file; the book call-site disposition is unchanged by the
  exclusion;
- `pin-loop-population-open-until-proven` + the committed EXPECTED-OPEN
  literal-loop cells with the propagation lane as greening trigger (§7);
- `pin-loop-types-need-no-rekey` - the propagation lane's arrival is
  representable without touching `ElisionRegion`, witness, or record identity
  (reviewed as a paper exercise against the landed types at stage close);
- `pin-definition-not-name` - same-named definitions never share route
  populations;
- `pin-no-generated-specialization` - emitted artifacts contain no cloned/renamed
  helper or generated invocation dispatch;
- `pin-whole-helper-derived-only` - a call elides only after every body region
  and every call-level observable admits it;
- `pin-empty-function-world-parity` - books with no eligible calls remain
  byte-identical;
- `pin-probe-site-identity-unchanged` - frozen probe records never re-key;
  nothing keys on dispatch batch;
- `req-census-admits-the-wrapped-book` discharged: the depth-3 wrapped strawman
  is admitted (measured), budgets re-set deliberately, over-budget still refuses
  proportionally with its named diagnostic;
- all shared decisions certify; both-platform gates green.

## 12. Implementation stages

0. **`stage-member-lane-ruling` - SATISFIED 2026-08-20.** `30La`'s member/inline
   verdict-ship repair and both product-facing acceptance cases are folded; downstream
   stages consume the repaired aggregate primitive.
1. **`stage-size-census-and-pin-battery`** - budget/representation sizing (§3.4:
   clone vs overlay decided; constants re-set against measured wrapped-book shapes);
   then the red-first battery: mixed Replace/Run bodies; agreeing twin calls;
   divergent-facts-one-guard; disagreeing-transformations Run; branch-join one-arm
   failure; nested static calls; call status consumed vs dead; stdout consumed on one
   route; dynamic-call census poison; the EXPECTED-OPEN literal-loop cells; influence
   divergence; whole-helper derivation. Ground truth precedes representation change.
2. **`stage-mint-region-and-route-identities`** - `ElisionRegion`,
   `RouteInstance` (with `IterationSlot`), `Closed(NonEmpty)|Open` populations;
   book-custody universe check; `DefinitionId`/loci preserved.
3. **`stage-build-universal-region-decisions`** - retain per-instance facts
   instead of collapsing into the call disposition; route proofs as projections
   (§4.0); the meet; the private decision projecting `Disposition` and semantic act.
4. **`stage-project-effective-acts`** - shared acts onto every owned instance
   invalidator inside the certified settlement; the two §6 residue riders (backing
   freeze; wall-narrative region operand); origin probe bytes and result IDs
   preserved.
5. **`stage-render-and-why`** - Spine/Plan/render edit definition regions once
   (Replace/Omit/Guard/Run only); bidirectional why; locator threading.
6. **`stage-close-cross-platform-corpus`** - transformed books executed under
   inert mocks; output/status preservation, order, source-map identity, deterministic
   route ordering, empty-world parity; both-platform gates; the pin-loop-types paper
   review.

## 13. Ledger updates at implementation-open

- `ANALYZER-NEEDS`: rows for invocation-instance identity, elision-region identity,
  closed route populations, universal region decisions, the iteration dimension;
- `FORFEITS`: partial function-body elision remains forfeited until the stage lands;
  after staging, a row for loop populations Open-until-propagation (capture: the
  propagation lane); aggregate verdict primacy is a correctness repair, not a row;
- `spike/CLAUDE.md`: the `inv-leaf-seam` amendment (execution leaves vs elision
  regions, site-keyed results unweakened);
- crate steering: `analysis/CLAUDE.md seam-interproc`'s all-or-nothing CALL license
  sentence rewritten to the region truth; `plan`/`cli` steering gains
  shared-edit-before-erasure and no-specialization;
- `28Q`/`30I` status: the exact stage boundary relative to artifact-form closure.

## 14. Footnotes: paths not taken, and scheduling

Paths not taken, each deliberately: per-invocation specialization and call-site
inlining (dead under the authorship law — generated shell has no answerable author,
and the plan would stop being the user's book); a `May`/partial decision tier
(un-renderable without specialization; the parametric guard is the sound version of
the same value); extending region descent into pipelines (the byte-stream interlock
makes mid-stage replacement valueless while a downstream stage runs — the parked
per-stage status question, `flag-pipe-status-unit`, is a why-plane matter, not an
edit-identity one); implementing loop propagation now (staged instead, §7 — the value
plane arrives on stable types rather than forcing a second identity churn); elision
work inside dorc-lang files (excluded by ruling, §0 — not deferred, excluded).

Scheduling, tersely: this stage sits after `30I`'s bundle projection and locator
consumption and BEFORE artifact-form reification and the corpus-wide XFAIL/golden
promotion — those two are the accretion surfaces that would harden leaf-only edit
identity into the Plan/Spine boundary and the blessed corpus. The artifact work must
CONSUME region identity, not reserve a seat for it. Where the round boundary falls
within that sequence is a conductor/human energy call; the sequence itself is not.
Sizing: comparable to the effective-world-reach stage — identities and census lean on
landed frames/custody; route proofs are retention, not new analysis; the settlement
integration is the risky third, halved by the fresh aggregate-witness precedent whose
discipline this plan reuses at the shared level.
