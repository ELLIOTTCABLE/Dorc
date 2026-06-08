# notes/180 — dq-substrate prior-art, wave 1

**Stamp:** round 17 · 2026-06-06 · interactive-research gather-and-grade, wave 1. The three substrate
families (IFDS/IDE · Datalog/Soufflé · the monotone-worklist baseline) + the recency/strong-update
precision keystone. Sources full-read and graded by clean-context subagents (`graded-by: subagent`,
provisional until main-context re-verify); excerpts below are verbatim. My own grounding read of the SPA
distributive chapter [B-moeller-spa-distributive-2024] is logged separately (turn note 181).

> Round-17 question (from `16Q §3`): decide **dq-substrate** (hand-rolled monotone worklist vs IFDS/IDE
> vs Datalog/Soufflé) × **dq-entity-algebra** (flat vs structured fact domain + strong/weak update) ×
> the **provenance/why-tree query model** (`kFIDELITY`), against Dorc's real needs. Thumb on the scale:
> the perf-inversion (`AGENTS`: network dominates → analyzer big-O ~free).

## Findings (most-attended first; certainty-marked)

- **fnd-1 — Dorc's effect-analysis is the *easiest* IFDS subclass, and the worklist it already built is
  in IFDS's precision class intraprocedurally.** +SURE. Oracle gen/kill of `(kind,entity)` facts =
  textbook locally-separable bit-vector problem (reaching-defs/liveness family); domain = powerset of a
  finite set, merge = union, transfers distribute. So **substrate choice is NOT a precision question for
  the current analysis** — a monotone worklist and IFDS compute the *same* meet-over-valid-paths answer
  for distributive bit-vector problems. The substrate question is really about *interprocedural reach*,
  *provenance*, *the precision keystone*, and *incrementality*. [A-ifds-demand-dataflow-1995]
  [A-ifds-practical-extensions-2010]

- **fnd-2 — the perf-inversion neutralizes every heavy substrate's headline cost; the ONE survivor is
  controller RAM** for a materialized fact-base. +SURE. Soufflé does OpenJDK7 context-insensitive
  points-to in 35s but the *context-sensitive* run materialized **206 GB**; the Datalog-scalability paper
  treats 50 GB as an OOM cap and calls the blowup cliff "unpredictable." ~SUSPECT this is moot for Dorc:
  shallow sh + referent-agnostic kinds keep the fact-base tiny (no points-to context explosion), so the
  cost that breaks Java analyses likely never reaches megabytes here — but a `limitsize`/Choice-Bound
  guard is cheap insurance against a pathological script. [A-souffle-synthesis-cav-2016]
  [A-datalog-scalability-walls-2025] [A-doop-datalog-pointsto-2009]

- **fnd-3 — the recency "non-monotonicity" scare is narrower than the `16Q`/`055` framing implies.**
  +SURE. What the 2017 paper proves non-monotone is the recency *constructor* as a function over the
  *lattice of address-abstractions* (refining the underlying partition need not improve recency's result
  — so you can't blindly swap in a "finer" entity-keying and assume more precision). It is **NOT** a claim
  that the per-program dataflow *transfer functions* are non-monotone, so a monotone-worklist least
  fixpoint over a *fixed* abstraction does not obviously break. ~SUSPECT residual hazard (from the 2006
  paper, not 2017): the recency *allocation transformer* demotes most-recent→summary, a destructive
  (non-join) step; B&R keep termination via flow-sensitivity + an ascending count lattice. Dorc must
  verify its flat worklist can host a strong-update transformer without losing the ascending-chain
  guarantee — plausibly yes-with-care. [A-recency-not-monotone-2017] [A-recency-abstraction-2006]

- **fnd-4 — strong updates (the precision keystone) look achievable on a FLAT domain, and may be ~free
  for Dorc.** +SURE on the mechanism: the 2017 "singleton abstraction" gets sound strong updates *without
  splitting nodes* — keep the underlying partition, attach a per-partition `{singleton, multiple}` tag,
  strong-update only when cardinality = 1; it preserves refinement (sidesteps the non-monotonicity) and
  matches recency's precision at lower cost. -GUESS the bigger point for Dorc: its entities
  (`package:nginx`) are mostly *statically-identifiable singletons* (one nginx per host), unlike
  malloc-in-a-loop — so strong updates may need only "flat key + a uniqueness bit," with the heavy
  recency machinery reserved for genuinely multiplicitous/dynamically-named entities (`for h in $hosts;
  do install…`, runtime-variable names). Actionable: enumerate which Dorc kinds can be non-singleton.
  [A-recency-not-monotone-2017] [A-recency-abstraction-2006]

- **fnd-5 — Datalog's real pull for Dorc is expressiveness+provenance, not speed.** +SURE. (a) Rules read
  as a near-transcription of declared semantics ("the Datalog code is almost an exact transcription of
  the [Java] specification… the 'must' property is ensured by the least-fixpoint") — maps directly onto
  oracle authors stating `(kind,provider,verb)→effect`; (b) mutual recursion + fixpoint are free; (c)
  Soufflé ships **lazy minimal-height proof-trees** (`-t explain/explore`) — the `kFIDELITY` why-trees
  ~free in dev-effort/query-time. Costs: NOT incremental (Soufflé core is full re-eval; incrementality
  lives in IncA/Flix/Differential-Dataflow); Rust embedding friction (Soufflé emits header-only **C++** →
  FFI, not a clean crate); and "Datalog is not a magic auto-optimizer, you hand-introduce indexes" —
  which is *moot under the perf-inversion* (the >1000× optimized-vs-unoptimized gap is exactly the cost
  Dorc doesn't pay). [A-doop-datalog-pointsto-2009] [B-datalog-easy-analysis-lessons-2011]
  [A-souffle-provenance-docs-2026] [A-souffle-synthesis-cav-2016]

- **fnd-6 — IFDS's distinctive value over the worklist is summary-reuse + a demand cost-guarantee; shallow
  sh undercuts the first.** +SURE. Summary edges amortize a procedure's distilled effect across many
  call-sites — pays off under *repeated* interprocedural reuse/recursion, which "embarrassingly shallow"
  sh (functions called a handful of times) largely lacks. BUT summaries are the right *correctness*
  mechanism to stop the spike's ⊤-forcing of function-bodies/sourced-files/traps (detached regions). If
  effects ever go richer than gen/kill (a fact produced only if A∧B — non-distributive), the documented
  next rung is **IDE** (facts = maps `D→L` over a finite-height semilattice), not a stretched IFDS.
  [A-ifds-demand-dataflow-1995] [A-ifds-practical-extensions-2010]

- **fnd-7 — "demand-driven" ≈ Dorc's query-planner framing, and it is NOT substrate-exclusive.** +SURE.
  Demand IFDS answers "does this fact hold at this point" with polynomial-per-query / same-worst-case
  over all queries, explicitly motivated by interactive goal-directed tools — Dorc's "do only the work the
  goal needs." And demand-IFDS is formally an analog of a *magic-sets-transformed* logic program, i.e.
  demand-Datalog and demand-IFDS are kin — so the demand property can be expressed on either substrate.
  [A-ifds-demand-dataflow-1995]

- **fnd-8 — Datalog expressivity caveats to check against Dorc's effect-lattice.** ~SUSPECT. Stratified
  negation only (no negation inside a recursive cycle); no constructors (context-depth must be bounded).
  A best-effort ⊤-collapse lattice with bounded interprocedural depth probably sidesteps both, but verify
  if any Dorc analysis needs non-monotone negation or aggregation. [A-doop-datalog-pointsto-2009]
  [B-datalog-easy-analysis-lessons-2011]

## Citations

### IFDS / IDE — fit, limits, demand

> [A-ifds-demand-dataflow-1995]:§2 (relevance: +1:SURE)
> "The algorithm … can be used to solve any interprocedural dataflow problem in which the dataflow facts
> form a finite set D, and the dataflow functions (which are of type 2^D→2^D) distribute over the meet
> operator (either union or intersection). We call this class … the interprocedural, finite, distributive,
> subset problems, or IFDS problems".

> [A-ifds-demand-dataflow-1995]:§2 (relevance: +1:SURE)
> "The IFDS problems include all locally separable problems — the interprocedural versions of classical
> 'bit-vector' or 'gen-kill' problems (e.g., reaching definitions, available expressions, and live
> variables) — as well as non-locally-separable problems such as truly-live variables, copy constant
> propagation … and possibly-uninitialized variables."

> [A-ifds-demand-dataflow-1995]:§3 (relevance: +1:SURE)
> "when the nodes of G* represent individual statements … and when there is no aliasing, we expect most
> distributive problems to be h-sparse (with h << D): Each statement changes only a small portion of the
> execution state … The dataflow functions … should be 'close to' the identity function." (gen/kill = the
> easiest IFDS subclass.)

> [A-ifds-demand-dataflow-1995]:§1 (relevance: +1:SURE)
> "a demand dataflow analysis algorithm determines whether a single given dataflow fact holds at a single
> given point." … "It has a polynomial worst-case cost for both a single demand and a sequence of all
> possible demands." (demand = the query-planner cost model.)

> [A-ifds-practical-extensions-2010]:§1 (relevance: +1:SURE)
> "The fundamental restrictions of the algorithm, which we do not seek to eliminate in this paper, are
> that the analysis domain must be a powerset of some finite set D, and that the dataflow functions must
> be distributive." (IFDS's hard boundary — richer needs ⇒ IDE.)

> [A-ifds-practical-extensions-2010]:§9 (relevance: +1:SURE)
> "The IDE algorithm generalizes IFDS … the dataflow facts [are] maps drawn from D→L, where D is a finite
> set and L is a finite-height semi-lattice … the IDE algorithm additionally evaluates functions L→L along
> those paths." (the documented next rung past gen/kill.)

> [A-ifds-practical-extensions-2010]:§2 (relevance: -0:SUSPECT)
> "every distributive function in P(D)→P(D) is uniquely defined by its value on the empty set and on every
> singleton subset of D … the function can be defined by a bipartite graph". (why distributivity makes
> summaries compact/cheap — the mechanism Dorc would buy.)

### Datalog / Soufflé — buys, costs, provenance, embedding

> [A-doop-datalog-pointsto-2009]:§3.3 (relevance: +1:SURE)
> "the Datalog code is almost an exact transcription of the Java specification. (The main difference is
> that the specification is written in a must style, whereas the Datalog code specifies which casts may
> happen. The 'must' property is ensured by the least-fixpoint evaluation of Datalog.)" (rules ≈ declared
> semantics — maps onto oracle authoring.)

> [A-doop-datalog-pointsto-2009]:§3.2 (relevance: +1:SURE)
> "The total size of the analysis logic in DOOP is less than 2500 lines of code (approximately 180 Datalog
> program rules) … These metrics include all pointer analysis variants". (concision.)

> [B-datalog-easy-analysis-lessons-2011]:§3 (relevance: +1:SURE)
> "Datalog is not an abstract logic and does not magically yield automatic programming capabilities … We
> needed to develop an optimization methodology for highly recursive programs and to introduce indexes
> manually, in order to attain optimal performance. The difference in performance between optimized and
> unoptimized DOOP rules is enormous." (the perf is hand-won — but moot under Dorc's inversion.)

> [B-datalog-easy-analysis-lessons-2011]:Abstract/§1 (relevance: +1:SURE)
> "Although this performance difference is largely attributable to architectural choices (e.g., the use of
> an explicit representation vs. BDDs) … This performance improvement is not caused by any major
> algorithmic innovation". (don't credit Datalog-the-language for the speed.)

> [A-souffle-provenance-docs-2026]:/provenance (relevance: +1:SURE)
> "Provenance is a way to explain the execution of a Soufflé program… These explanations come in the form
> of a proof tree. In Soufflé, for any tuple, these proof trees are of minimal height". (why-trees, first
> party.)

> [A-souffle-provenance-docs-2026]:/provenance#internals (relevance: +1:SURE)
> "The approach for provenance in Soufflé is a lazy one, where no proof trees are computed until the user
> queries for them. However … the system requires to keep track of some extra information during
> evaluation. In particular, for each tuple, the system tracks the rule producing the tuple, and the
> height of a minimal height proof tree". (lazy, but a standing per-tuple memory tax when enabled.)

> [A-souffle-synthesis-cav-2016]:Table 1 (relevance: +1:SURE)
> context-insensitive points-to "SOUFFLÉ 0:00:35, 8.5 GB"; context-sensitive points-to "6:44:08, 206.4
> GB"; Security analysis "14:45:01, 75.3 GB". (CPU trivial vs SSH; RAM is the sole survivor cost.)

> [A-souffle-synthesis-cav-2016]:§2.1/§4 (relevance: +1:SURE)
> "The generated C++ code is packaged in form of header files for a smooth integration with host
> applications." … "resulting analyzers may be directly included into host applications as a header-only
> library." (embedding = C++ FFI for a Rust host.)

> [A-datalog-scalability-walls-2025]:Abstract (relevance: +1:SURE)
> "Datalog evaluation is bottom-up, meaning that all inferences … are performed and all their conclusions
> are outputs … virtually every program analysis expressed in Datalog becomes unscalable for some inputs,
> due to the worst-case blowup of computing all results, even when a partial answer would have been
> perfectly satisfactory." (the counter-thesis; the unpredictable-cliff motivation for a size guard.)

> [A-doop-datalog-pointsto-2009]:§6 (relevance: -0:SUSPECT)
> "The engine we use also supports incremental evaluation after deletion and updates of facts using the
> DRed algorithm. Efficient incremental evaluation might make … analysis suitable for use in IDEs."
> (incremental is possible in some engines — Dorc's plan-preview-on-edit use case — but not in Soufflé
> core; see gaps.)

### Recency / strong-update / entity-algebra

> [A-recency-abstraction-2006]:§1 (relevance: +1:SURE)
> "A strong update overwrites the contents of an abstract object, and represents a definite change in
> value to all concrete objects that the abstract object represents. Strong updates cannot generally be
> performed on summary objects because a (concrete) update usually affects only one of the summarized
> concrete objects."

> [A-recency-abstraction-2006]:§1 (relevance: +1:SURE)
> "for an assignment … points-to-analysis algorithms are ordinarily forced to perform a weak update …
> the abstract execution of an assignment to a field of a summary node cannot kill the effects of a
> previous assignment … Because imprecisions snowball as additional weak updates are performed".

> [A-recency-abstraction-2006]:Abstract/§4 (relevance: +1:SURE)
> "This approach succeeded in resolving 55% of virtual-function call-sites, whereas previous tools for
> analyzing executables fail to resolve any". (the precision gain from strong updates — note the
> "87%→<2%" figure in Dorc docs is NOT here; it is TAJS — see gaps.)

> [A-recency-abstraction-2006]:§5 (relevance: +1:SURE)
> "if the recency-abstraction were used with a flow-insensitive algorithm, it would provide little
> additional precision over the allocation-site abstraction … the algorithm would have to perform weak
> updates for assignments to MRAB nodes". (strong-update precision REQUIRES flow-sensitivity.)

> [A-recency-not-monotone-2017]:Abstract (relevance: +1:SURE)
> "while recency abstraction enables more precise analysis results by allowing strong updates on recent
> objects, it is not monotone in the sense that it does not preserve the precision relationship between
> the underlying address abstraction techniques: for an address abstraction A and a more precise
> abstraction B, recency abstraction on B may not be more precise than recency abstraction on A."
> (the non-monotonicity is META-level, over the lattice of abstractions — NOT transfer-function
> monotonicity.)

> [A-recency-not-monotone-2017]:§4.2 (relevance: +1:SURE)
> "we decide not to divide a given partition but to simply perform strong updates on singleton objects …
> It distinguishes partitions with only one address as s and maps the other partitions to m … the
> singleton abstraction preserves the refinement relation of its underlying address abstraction because
> they use the same partition … singleton abstraction allows strong updates for address partitions that
> map to s." (strong updates on a FLAT domain + a cardinality tag — the Dorc-relevant fix.)

> [A-recency-not-monotone-2017]:§5 (relevance: -0:SUSPECT)
> "recency and singleton abstractions analyze about 42.39% and 33.63% of property loads more precisely on
> average, respectively." … "singleton abstraction does not incur much performance overhead like recency
> abstraction while providing comparable analysis precision." (singleton ≈ recency precision, cheaper.)

## Gaps / wave-2 targets

- **gap-1 — the "87%→<2%" precision claim is TAJS, not the recency papers.** Source it directly: TAJS
  (Jensen, Møller, Thiemann, "Type Analysis for JavaScript", SAS 2009) + the TAJS site. Confirm the exact
  figure and what disabling recency/strong-update costs there, since `055`/`16Q` lean on it as the
  keystone justification.
- **gap-2 — Salsa / rust-analyzer (the incremental/demand engine).** A Rust-native, demand-driven,
  *incremental* query engine — a candidate 4th "substrate" (or an orthogonal memoization layer over any
  of the three) that directly serves the `dorc try` diff-time hot-loop — exactly the incrementality
  Soufflé core lacks (fnd-5). Decide: substrate, or layer?
- **gap-3 — Rust-native Datalog (Ascent, Crepe, Datafrog, Differential Dataflow / DDlog).** Both Datalog
  subagents flagged embedding-in-Rust as out-of-scope-for-their-sources. If a clean Rust-embeddable
  Datalog exists, the "Soufflé→C++ FFI friction" objection (fnd-5) weakens sharply, and Differential
  Dataflow answers the incremental gap. This is decisive for whether "Datalog" means "external C++ engine"
  or "a Rust crate."
- **gap-4 — Dorc's OWN prior conclusions.** Read `plans/055` (the reference engine design: fact =
  opaque-token + source-expr; the recency strong/weak lever; the why-tree provenance design) and notes
  `050`–`054` (the project's analysis prior-art map — esp. `052` IFDS-engine + Datalog-bridge, `054`
  TAJS/recency, `053` PDG/SDG-vs-Datalog-vs-value-flow). Align wave-1 findings with what Dorc already
  decided/leaned, and surface where this round contradicts it.
- **gap-5 — IDE worked example + non-distributive escape.** If Dorc ever needs conjunctive/valued effects,
  confirm IDE's cost and whether a hand-worklist can host the D→L environment-transformer form, or whether
  that is the point where a real framework (Soufflé / an IFDS-IDE lib like Heros) earns its keep.
