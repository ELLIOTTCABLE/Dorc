# Reusable structure (Q2) + scale mechanisms (Q3) — SDG/PDG, Datalog/Soufflé, sparse value-flow

Three alternative *reusable substrates*, all reducing analysis to **reachability/fixpoint over a graph or relation set**, plus the convergent analyzer factoring and the scaling levers.

## The convergent analyzer factoring (adopt this)
SURE: independent codebases converge on the same 3-way split for an *extensible* analyzer:
- **SVF**: `Graph` (abstraction extracted from IR — *where* to analyze) / `Rules` (how to derive facts per statement) / `Solver` (in what order to resolve).
- **Goblint**: `domain(s)` (lattice lib) / `solver` (fixpoint engines) / `framework`+`analyses` (rules as plug-ins).
- **Soufflé**: EDB (facts) / IDB (rules) / engine (semi-naïve solver).
→ Dorc's analyzer should factor the same way: **(IR/graph) + (effect/transfer rules, pluggable per command via oracles) + (solver)**. This is also how the **oracle library plugs in**: an oracle contributes *Rules* (effect-class + check) for a command; the engine (Graph+Solver) is fixed. Matches the CoLiS "engine generic over command specs" lesson (note 20) and the planning-log's engine/oracle split.

## Q2 — reusable structure: three substrates
1. **PDG / SDG (dependence graphs)** — Ferrante-Ottenstein-Warren + Horwitz-Reps-Binkley [A-horwitz-reps-binkley-sdg-slicing-toplas-1990]. Vertices = statements/predicates; edges = **control-dependence + data(flow)-dependence**. **Slicing = vertex reachability** over these edges (intraprocedural: linear time). Interprocedural needs **summary edges** (transitive actual-in→actual-out per call site) + **two-pass realizable-path reachability** to respect **calling context** (naive transitive-closure is imprecise — the Add-called-by-A-returns-to-Increment problem; same realizable-path/CFL insight as IFDS).
   - → Dorc: build the dependence graph *once*; then **backward slice from the probe/dirty set = "what feeds this probe"** (Q2 tracing) **and "drop everything not in the slice"** (1B precision). **Forward slice from a changed role = diff-time impact analysis** (what this edit affects). Many analyses reuse the one graph.
2. **Datalog fact base (Soufflé/Doop/CodeQL)** — extractor → input relations (EDB); analysis = **rules (IDB)**; engine = least-fixpoint (semi-naïve). **Adding an analysis = adding rules**, no pass-rewriting (the strongest "retain facts, query many ways, extend cheaply" answer). Soufflé compiles the rules to specialized parallel C++ (points-to on 1.4M vars/840M tuples in 35s). **IFDS ≡ Datalog** (note 52), so our reachability/effect analysis is expressible as rules over the fact base. Datalog **provenance** = "why is this probe needed" (explainability → output quality, Q2).
3. **Value-Flow Graph (SVF)** — def-use chains made explicit via (memory-)SSA; analysis propagates **sparsely** along value-flow edges, skipping irrelevant program points. Source→sink reachability is the dominant client.
- **Dorc choice (GUESS for synthesis):** a **hybrid** — a dependence/value-flow graph as the IR (carries control+data+effect edges, sliceable), exposed *as* a relational/Datalog fact base for queries (so new analyses = new rules + provenance). The graph gives slicing+sparsity; the fact base gives extensibility+explainability. The two are duals (graph edges ≅ relations).

## Q3 — scale levers (all compose)
- **Compositional summaries** (Facebook + IFDS summary edges + SDG summary edges + Salcianu-Rinard per-method summaries): analyze each role/module **once**, cache a summary, recompose. Ops corpus is embarrassingly modular ⇒ ideal. Naturally **incremental** (change one role ⇒ re-summarize only it).
- **Sparse value-flow** (SVF): propagate only along def-use, not across every CFG point. Big real-world speedup; directly serves "only probe what's touched" (1B).
- **Datalog-compiled-to-native** (Soufflé): the *whole-corpus* fact computation as compiled, indexed (auto index-selection), parallel relational algebra (B-trees/Tries, optimistic locking). Scales to billions of tuples.
- **Region/granularity partitioning** (SVF mem-regions): partition system-state into fact-domains (`pkg`/`file`/`svc`/`user`/`port`...) to tune scalability vs precision.
- **Demand-driven** (IFDS demand; Heintze-Tardieu; Reps-cc94): compute only what a query needs ("does this host need probing for fact F?") — precision + speed.
- **Incremental fixpoint** (Soufflé semi-naïve delta; IncA; Reviser for IFDS; Salsa-style memoized queries): the diff-time recompute. (Mechanism detail: note 54.)

## How this answers the three questions (forming)
- **Q1** = forward over-approximating may-mutate (MOD) analysis (Salcianu-Rinard domain) propagated via IFDS/IDE-style summaries over the dependence/value-flow graph; probe set = Reachable ∩ MayMutate; ⊤-on-unknown (un-probeable + can't-skip) for soundness (two: probe + elision, see `kFAIL`).
- **Q1B** = slicing (backward from probe set) + sparse value-flow + realizable-path/CFL precision + demand.
- **Q2** = the dependence/value-flow graph IS the retained reachability structure; expose as a Datalog fact base so new analyses = new rules; provenance = explainability.
- **Q3** = compositional per-role summaries + sparse + Datalog-native + region-partitioning + demand + incremental — every lever maps onto the modular ops-corpus structure.

## Still to cover before synthesis (honesty on the gate)
- **Incremental mechanism** (Reviser / IncA / differential-dataflow): how summaries/relations are *updated* on a diff (vs recomputed). Read one.
- **Sound AI of a dynamic language** (TAJS): our exact predicament (messy, dynamic, recalcitrant). What do you do when values/targets are unknown? (collapse to ⊤; coarse abstraction). Read it.
- Then: coverage check → synthesize.
