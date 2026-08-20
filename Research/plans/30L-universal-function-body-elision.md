# 30L - Universal function-body elision across static routes

> Tier: focused design plan, minted from the human's direct ruling on 2026-08-20.
> Root docs and `spike/CLAUDE.md` outrank it. **[TYPED]** means human-stated in the
> design sitting; **[DERIVED]** is this plan's mechanical consequence; **[PROPOSED]**
> remains implementation latitude.
>
> Scope: book/helper functions only. Oracle functions remain authored probe/guard
> programs, not apply bodies. This plan adds no function specialization, cloning,
> renaming, generated dispatch, or other generated shell. The only executable edits
> remain the ordinary Dorc forms: retain original bytes, insert one shared guard, or
> replace/comment one authored source line.
>
> Parent architecture: `28Q:stage-effective-world-reach` supplies per-instance CFG
> effects, effective walls, ownership, and settlement; `30I` supplies static loading,
> definition identity, source loci, bundle projection, and later artifact forms.

## 0. Ruling in one screen

**`30L:rul-shared-line-needs-universal-must` [TYPED]** - a source line inside a
function may be elided only when BOTH universals hold:

1. every CFG route to every statically possible invocation-instance of that line is
   closed and known; and
2. every license-bearing property holds on every such route.

Any dynamic/unresolved/unbounded route, any failed property, or any disagreement about
the one source edit means **Run** for that shared source line. There is no `May`
transformation and no per-invocation specialization.

```text
source_decision(line) = meet_Must {
   route_decision(line, route)
   | route in every_static_route_to(line)
}

meet_Must contains any unknown/failure/disagreement => Run
```

**`30L:rul-helper-elision-is-derived-linewise` [TYPED]** - full elision of an
entire helper is derived only when every executable line in its body, through every
incoming invocation edge and CFG route, admits elision. Partial body elision is the
ordinary case: agreeing lines replace/guard independently while any failed line remains
runnable.

**`30L:rul-no-specialized-shell` [TYPED]** - never clone a function per call, rewrite
call names, inline executable source at call sites, or emit argument-dispatch branches.
One authored source line receives at most one shared transformation valid for every
invocation.

## 1. The gap

Today the analyzer already creates per-call CFG instances for supported static function
calls. Positional arguments flow into the spliced body, internal commands classify and
probe separately, and every internal mutation participates in effective reach. But the
plan collapses those facts into one all-or-nothing CALL disposition because the call is
the only render leaf.

```sh
main() {
   converge_a
   mutate_b
   converge_c
}
main
```

As built:

```text
all body establishes removable => Replace(call)
any body site not removable     => Run(call)
```

Required:

```sh
main() {
   true       # converge_a replaced
   mutate_b
   true       # converge_c replaced
}
main
```

This is a precision/value gap, not a current under-execution: the all-or-nothing floor
runs too much. It is severe for the common `main() { whole book; }; main "$@"` shape,
where one live or unknown command otherwise forfeits nearly the whole attention product.

## 2. Identities: invocation instance versus shared source region

**`30L:rul-two-identities-never-conflated` [DERIVED]** - analysis and rendering ask
different identity questions:

```text
InvocationBodySite
   = one analyzed execution instance of a body statement

SharedBodyRegion
   = the one authored source span all those instances would edit
```

A body instance is approximately:

```text
BodyInstance {
   definition: DefinitionId,
   invocation: InvocationId,
   cfg_node: CfgNodeId,
   region: SharedBodyRegion,
}
```

`DefinitionId` prevents same-named definitions from combining. `InvocationId` separates
argument/context instances. `CfgNodeId` remains analysis-local. `SharedBodyRegion`
names the one editable authored span, including source-file/locator identity once `30I`
projects sourced books.

`LeafId` remains the outer call's executable/probe namespace. Existing `site N.M`
records remain per-call/member evidence. This plan does not widen `LeafId` or let
analysis-internal nodes masquerade as plan leaves.

**`30L:inv-closed-route-set-never-empty` [DERIVED]** - route populations are typed:

```text
RoutePopulation = Closed(NonEmpty<RouteInstance>) | Open
```

`Open` always runs. Universal quantification over an empty set is forbidden; an
unreached definition does not acquire vacuous authority.

## 3. What “every route” means

There are two distinct universal meets.

### 3.1 Within one invocation

The ordinary CFG/dataflow solver already meets predecessor states at branches and loops.
A body site's per-invocation proof is `Must` only if the property holds through every CFG
route that can reach that site. Implementations need not enumerate paths exponentially;
the lattice meet is the path quantifier.

```sh
f() {
   if condition; then prepare_a; else prepare_b; fi
   converge_x
}
```

`converge_x` is transformable only from the joined `Must` answer. A property held on one
arm and unknown on the other meets to failure/Run.

### 3.2 Across invocation instances

All statically possible calls resolving to the same definition/source region contribute
one per-instance answer:

```sh
install_pkg() { apt-get install "$1"; }
install_pkg nginx
install_pkg curl
```

The source line `apt-get install "$1"` receives one shared edit only if both invocation
instances admit exactly that edit. A proof for `nginx` alone says nothing about the shared
line.

**`30L:rul-call-census-must-be-closed` [TYPED]** - “all static invocations agree”
requires a closed invocation census. Any construct that may invoke the definition outside
the enumerated call set makes its body regions `Open`:

- dynamic command position;
- unresolved source/load;
- authored `eval`/opaque code-carrier;
- unresolved callback or alias;
- recursion or an exceeded call/splice bound;
- unbounded/unresolved loop invocation;
- trap/string execution that may name the function.

Silence never means “no other calls.”

## 4. The per-route property product

**`30L:rul-every-property-meets-universally` [TYPED]** - the shared line's decision
does not meet only a final `Disposition`. Every license-bearing property must be `Must`
through each route before the shared source edit mints.

Per route, retain at least:

```text
RouteLineProof {
   reachable_and_closed,
   exact_definition_and_body_site,
   phase_appropriate_measurement,
   reached_vouch_or_read_substitution_proof,
   effective_freshness,
   effect_ownership,
   consumed_observables,
   replacement_or_guard_bytes,
   render_feasibility,
   influence,
}
```

Any absent, unknown, stale, declined, conflicted, render-refused, or differently keyed
member rejects the shared edit.

### 4.1 Effects and freshness

Each body instance consumes the effective reaching-wall set at its own CFG position.
Replacement of one shared source region retires all invalidators owned by every instance
of that region only after universal agreement mints the shared no-execution proof.

One invocation's clean world cannot erase a line that is stale in another invocation.
Aggregate member freshness from `30K` remains usable evidence within each instance; it
never substitutes for the across-invocation meet.

### 4.2 Observables

**`30L:rul-shared-edit-reproduces-every-route` [TYPED]** - one shared replacement
must preserve the consumed observable tuple on every route:

```text
{Effect, Status, Stdout, Stderr}
```

If status is consumed and two invocations require different stand-ins, no one source
replacement exists: Run. If stdout/stderr is consumed on any route and no common real-
provenance stand-in reproduces it, Run.

Errexit receives no special escape. It is status consumption plus the CFG failure edge;
the existing `StatusRelaxable` / `StatusInvariant` / `StatusIterated` rules apply per
route and meet universally.

Function return status is compositional only when transformed body lines reproduce every
status consumed by body control flow and by each call context. A call used in `if f; then`
therefore contributes a stricter route than a call whose status is dead.

### 4.3 Influence

**`30L:rul-shared-influence-never-launders` [DERIVED]** - a shared decision carries
the most influenced grade among all contributing route proofs. One uninfluenced route
cannot cleanse a host-influenced sibling. Every contributing route remains available to
the narrative/locator plane even when the decision stores a joined grade.

### 4.4 Guards

A shared guard is permitted only when every route admits the same invocation-parametric
guard at the authored source line:

```sh
f() {
   ( tool__is_converged "$1" ) || tool "$1"
}
```

All routes must resolve the same guard definition/custody and the same source-level argv
expression. A per-call literal guard (`... nginx`) cannot be installed into shared source
that also serves `curl`. If no one common guard exists, Run.

Guards remain ordinary authored-oracle bytes plus original command bytes. No generated
dispatch or call specialization is licensed.

## 5. Shared source decision algebra

**`30L:rul-source-edit-is-one-must-result` [DERIVED]** - group route proofs by
`SharedBodyRegion`, then mint exactly one source decision:

```text
SharedDecision = Replace(CommonStandIn)
               | Guard(CommonGuard)
               | Omit
               | Run
```

The meet is deliberately biased to Run:

```text
all routes prove the same observable-preserving Replace => Replace
all routes prove source-level Omit                        => Omit
all routes admit exactly one common parametric Guard      => Guard
anything else                                             => Run
```

`Replace`/`Omit` equivalence is semantic, not merely equal enum tags: replacement bytes,
stand-in observables, effect identity, and render span must agree. `Guard` equivalence
includes guard body identity/custody and source-level invocation bytes.

No `May` result is rendered or fed into effective reach.

## 6. Settlement, not a render post-pass

**`30L:rul-shared-agreement-precedes-wall-retirement` [TYPED]** - per-instance
decisions may not be merged after settlement. If one route forces shared Run, the line's
effects remain walls and can invalidate downstream facts in every relevant invocation.

Required round shape:

1. derive per-instance route proofs from the frozen invocation/CFG population;
2. group by shared source region;
3. meet every property to one shared decision;
4. mint no-execution proofs only for universally agreed shared Replace/Omit;
5. project that shared act onto every owned instance invalidator;
6. solve effective reach and fold records;
7. repeat until the grow-only no-execution ledger is quiescent;
8. write shared source decisions and route attribution to Spine once.

The route/call population is frozen before the first round. Settlement may prove more
shared regions non-executing; it never discovers a new invocation or changes definition
binding.

**`30L:inv-no-posthoc-shared-demotion` [DERIVED]** - a per-instance replacement may
not enter the ledger before shared-source agreement. Otherwise a later Run meet would
need to reintroduce walls, breaking the grow-only proof and potentially leaving a wrong
downstream elision.

## 7. Loops and future constant propagation

**`30L:rul-one-call-site-is-not-one-evaluation` [TYPED]** - a syntactically singular
call inside a loop is not singleton evidence.

```sh
for pkg in nginx curl; do
   install_pkg "$pkg"
done
```

Representation reserves an iteration/member dimension in `RouteInstance`; it does not
re-key `SharedBodyRegion` when loop constant propagation arrives.

For a finite literal loop whose members are fully enumerated and propagated, each member
contributes a route instance and all meet universally. Until that mechanism is built, the
population is `Open` and the shared body line runs.

`while`/`until`, unresolved iteration counts, or loop-carried values remain Open unless a
future analysis proves a finite closed population. `StatusIterated` continues to block a
single-status substitution.

No loop-specific optimistic default is permitted. The future value-plane may turn Open
into Closed; it may never reinterpret an already-closed set by dropping members.

## 8. Rendering and plan honesty

**`30L:rul-edit-authored-definition-once` [TYPED]** - render the shared transformation
at the authored function-body source line. The function definition remains in place and
the original calls remain calls. This preserves shell scoping, positional parameters,
`local`, `return`, cwd/options, and ordinary off-ramp behavior.

Never render per-call specialized bodies.

The plan surface must show:

- the transformed body line at its definition locus;
- that its decision is universal over all contributing static invocations;
- the call sites/route count on pull/why surfaces;
- every call line that may itself execute or whose call-level observables remain live.

A line that is Run on any route remains visible and executable in the shared body. A Guard
remains visible. Only universally proved Replace/Omit saves attention at the definition
line.

**`30L:rul-whole-helper-elision-is-derived` [TYPED]** - after every executable body
region is universally non-executing and each call's consumed call-level observables are
reproducible, each call may independently replace with the ordinary stand-in. The inert
function definition remains authored text; no special “delete helper” mechanism exists.

## 9. Probe, evidence, and attribution

Existing per-call body records (`site N.M`) remain the evidence namespace. A shared source
decision consumes a closed exact set of these invocation/member records; one record never
speaks for another invocation.

Spine needs both levels:

```text
SharedBodyDecision {
   region,
   disposition,
   contributing_routes: NonEmpty<RouteProofRef>,
}
```

The representative source line is not an authority substitute for route proofs. `dorc why`
must be able to move both directions:

```text
definition line -> every invocation proof that licensed this edit
call instance   -> the shared body edits it executes
```

This is a first forcing consumer of the non-`LeafId` participant/locator distinction:
`LeafId` remains call/execution identity; `SharedBodyRegion` is authored edit identity;
`RouteInstance` is analysis identity.

## 10. Supported floor and exclusions

Initial support may include every statically resolved, finitely instantiated call graph
already admitted by the existing splice/inlining analysis, provided the invocation census
is Closed. It must not be phrased as “single invocation support.” Cardinality one falls out
of the general meet.

Run-floor exclusions remain:

- dynamic/unresolved calls;
- recursion and exceeded splice depth/budget;
- open loop populations;
- source regions shared with any unenumerated invocation;
- irreconcilable route observables;
- disagreeing guard definitions/argv forms;
- render-overlapping source edits;
- unknown function binding or definition custody;
- any route whose analysis/certification fails.

This plan does not add cross-run state, repeated probing, call specialization, generated
dispatch, reordering, or non-sh metadata.

## 11. Implementation stages

**`30L:stage-pin-shared-route-worlds` [PROPOSED]** - land red-first ground truth before
representation changes:

1. one top-level thunk with mixed Replace/Run body lines;
2. two calls whose body-line decisions agree;
3. two calls whose facts differ but admit one parametric guard;
4. two calls whose transformations disagree and therefore Run;
5. branch-join agreement and one-arm failure;
6. nested static calls;
7. call status consumed versus dead;
8. stdout/stderr consumed on only one invocation;
9. dynamic call poisoning the census;
10. literal-loop members, initially expected Open/Run until propagation lands;
11. influence grade differing across invocations;
12. full-helper elision derived from every body line.

**`30L:stage-mint-route-and-region-identities` [PROPOSED]** - add frozen invocation
instances, shared body regions, and `Closed(NonEmpty)|Open` route populations. Preserve
`DefinitionId` and source/locator identity; do not reuse `LeafId` or raw `AstId` alone.

**`30L:stage-build-universal-source-decisions` [PROPOSED]** - retain per-body-instance
facts rather than collapsing immediately into `InlineCall`; mint exact route proofs and
the universal shared decision. No execution proof enters the ledger before shared
agreement.

**`30L:stage-project-effective-acts` [PROPOSED]** - project one shared decision onto
every body-instance invalidator owner and integrate it into the existing certified
effective-world settlement. Preserve origin probe bytes and result IDs.

**`30L:stage-render-authored-body-regions` [PROPOSED]** - teach Spine/Plan/render to edit
definition regions once using only Replace/Omit/Guard/Run; thread definition+invocation
locator DAGs into why. No function cloning.

**`30L:stage-close-cross-platform-corpus` [PROPOSED]** - execute transformed books under
inert mocks; prove output/status preservation, order, source-map identity, deterministic
route ordering, empty-world parity, and both-platform gates.

## 12. Sequencing with 30I

`30I`'s complete load-occurrence account, bundle projection, and locator composition are
substrate for editing function definitions in sourced book files. This plan should not be
implemented as an incidental patch inside the closed `30K` stage.

Recommended boundary:

1. continue `30I` through bundle projection and real locator consumption;
2. stop before final artifact-form closure hardens Plan around leaf-only edits;
3. execute `30L` as its own reviewed stage;
4. close multipart/flattened/preserved-tree artifact forms over shared body regions.

If project scheduling closes r30 before implementation, the representation and forfeiture
must remain explicit; artifact APIs may not assume every editable/causal site is a `LeafId`.

## 13. Acceptance laws

The stage is complete only when all hold:

- `30L:pin-no-singleton-special-case` - no branch or API keys behavior on route-set
  cardinality one;
- `30L:pin-open-route-runs` - one unenumerated invocation forces Run for every shared
  region it may execute;
- `30L:pin-every-route-meets` - mutating any one route property to failure reddens the
  shared-elision pin;
- `30L:pin-shared-edit-before-erasure` - no per-instance effect retires before universal
  source agreement;
- `30L:pin-common-replacement-observables` - differing consumed statuses/bytes Run;
- `30L:pin-errexit-is-status` - no separate optimistic errexit path exists;
- `30L:pin-influence-joins-most` - shared decisions never lower influence;
- `30L:pin-loop-population-open-until-proven` - one syntactic loop call never reads as one
  evaluation;
- `30L:pin-definition-not-name` - same-name definitions never share route populations;
- `30L:pin-no-generated-specialization` - emitted artifacts contain no cloned/renamed helper
  or generated invocation dispatch;
- `30L:pin-whole-helper-derived-only` - a call elides only after every body region and every
  call-level observable admits it;
- `30L:pin-empty-function-world-parity` - books with no eligible calls remain byte-identical;
- `30L:pin-probe-site-identity-unchanged` - the transformation never re-keys frozen probe
  records;
- all shared source decisions certify; cap/failure takes Run; both-platform gates green.

## 14. Required ledger updates when built

At implementation open, update:

- `ANALYZER-NEEDS`: invocation-instance identity, shared render-region identity, closed
  route population, universal source decision, loop iteration dimension;
- `FORFEITS`: partial function-body elision remains forfeited until the stage lands;
- `spike/CLAUDE.md`: `inv-leaf-seam` must distinguish executable call leaves from shared
  editable body regions without weakening site-keyed results;
- `plan`/`analysis`/`cli` steering: shared edit before erasure, no specialization, and
  universal route Must;
- `28Q`/`30I` status: exact stage boundary relative to artifact closure.

No root human-authored document changes are implied without human authorship.
