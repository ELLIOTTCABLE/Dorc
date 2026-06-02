# Analysis prior-art acquisition map (the campaign plan)

> ⟢ 2026-06 — the engine this maps (IFDS / Datalog / recency / context-sensitivity) is the **reference design**, not committed MVP scope; its substrate tensions are `kFACTS` / `kCONTEXT` / `kPRECISION`, and its *weight* is gated on the `kDEPS` investment split. Grep the slug for current handling.

Parsing is treated as **solved/pluggable** (user directive); assume a reasonable parsetree over a corpus of files/modules. This map organizes the *analysis* prior art to slurp up **before** synthesizing. Synthesis is gated on coverage.

## The reframe (what "soundness" means here)
> **Update (two-soundness standard):** this round modeled only what's now called *elision-soundness* (don't omit a necessary probe/mutation). The co-equal *probe-soundness* (the read-only projection must not itself mutate; opposite fail-action) was left implicit. See AGENTS §1.
- **(1A) elision-soundness = sound over-approximation of the necessary-probe set.** Probe(c) is *necessary* iff c is *reachable* AND c *may-mutate*. So the probe set = `Reachable ∩ MayMutate`, and 1A demands we over-approximate both (never omit). Unmodeled constructs → ⊤ (un-probeable + can't-skip) — sound by construction; this is abstract interpretation's safe direction.
- **(1B) precision = tighten the over-approximation** (best-effort, undecidable in the limit): dead-code/reachability pruning, purity/MOD precision, slice-to-relevance, path/flow sensitivity.
- **(2) retain reachability/tracing** = build a *reusable* dependence graph + queryable fact base (+ provenance for explainability), not a one-shot pass.
- **(3) scale to arbitrarily-large org corpus** = compositional summaries + incremental recomputation + demand-driven + sparse representations.

## Question 1 — sound reduction parsetree → minimal-but-complete probe set
Canonical bodies of work (acquire each):
- **CFL-reachability** (Reps) — the *unifying* framework: dataflow, slicing, points-to are all graph-reachability over a context-free language of paths. THE backbone for 1A+1B together. → Reps "Program Analysis via Graph Reachability" (1998 survey).
- **IFDS / IDE** (Reps–Horwitz–Sagiv POPL'95; Sagiv–Reps–Horwitz) — precise interprocedural distributive dataflow as reachability; the practical algorithm for "which facts hold where" with a guaranteed-precise, polynomial result. (SPA §9 covers it; get the source papers.)
- **Abstract interpretation** (Cousot & Cousot POPL'77) — the soundness *foundation* (Galois connections, over-approximation). Get the paper + a modern tutorial.
- **Dataflow lattice frameworks** (Kildall'73; Sharir–Pnueli'81 call-strings vs functional). 
- **Effect systems** (Gifford–Lucassen POPL'88; Talpin–Jouvelot) — per-command *effect class* is literally a type-and-effect annotation; our oracle's effect-class = an effect type. Region/effect inference is how you *infer* effects compositionally.
- **MOD/REF & side-effect/purity analysis** (Banning'79; Cooper–Kennedy PLDI'88 "interprocedural side-effect analysis in linear time"; Salcianu–Rinard VMCAI'05 purity for Java) — *literally* "which commands mutate, and what." Closest classical analog to our may-mutate.
- **Taint analysis** — structurally identical to "which state reaches which probe / which mutation"; the JS/Java industrial lineage (FlowDroid is IFDS-based) is a direct template.

## Question 2 — retain reachability/tracing for evolving analyses + output quality
- **Program/System Dependence Graphs** (Ferrante–Ottenstein–Warren'87 PDG; Horwitz–Reps–Binkley TOPLAS'90 SDG) — the *reusable* control+data-dependence structure many analyses share (slicing, info-flow, change-impact). This is the "retain it once, query many ways" answer.
- **Program slicing** (Weiser'81; Tip'95 survey; Reps et al. "speeding up slicing") — backward slice from the dirty/probe set = exactly 1B "drop irrelevant" + 2 "what feeds this."
- **Datalog / declarative analysis as a queryable fact base** (CodeQL/QL — Avgustinov et al. ECOOP'16; Doop — Bravenboer–Smaragdakis OOPSLA'09; Soufflé — Scholz et al. CC'16; bddbddb — Whaley–Lam PLDI'04). The "analysis = queries over a relational/graph DB of program facts" model is the strongest realization of "retain info, add analyses later without rewriting passes." **Provenance** (Soufflé provenance / why-trees) = explainability ("why is this probe needed?") for output quality.

## Question 3 — performance on arbitrarily-large org corpus
- **Compositional / summary-based analysis** (Calcagno–Distefano–O'Hearn–Yang bi-abduction POPL'09 — have it; Distefano et al. "Scaling Static Analyses at Facebook" CACM'19; Infer/Pulse). Analyze each module/role *once*, cache a summary, recompose — maps perfectly onto the planning-log's "flat forest of mostly-independent roles" and onto a per-host fleet.
- **Incremental computation** (Salsa — rust-analyzer's query engine; Adapton; differential dataflow — McSherry CIDR'13; incremental IFDS — Arzt–Bodden "Reviser" ICSE'14; IncA — Szabó et al. incremental Datalog; Goblint incremental mode). The "evolve the tool / re-analyze a changed corpus cheaply" answer.
- **Demand-driven analysis** (Horwitz–Reps–Sagiv demand interprocedural; Heintze–Tardieu demand points-to) — compute only what a query needs (precision + speed).
- **Sparse analysis** (SVF — Sui–Xue CC'16 sparse value-flow; staged/sparse points-to) — skip irrelevant program points via value-flow graphs; major real-world speedup.

## Acquisition targets — repos to clone (graded; license noted where known)
| Repo | Lang | Why |
|---|---|---|
| facebook/flow | **OCaml** | flow-sensitive type inference + control-flow narrowing at industrial scale (the user's "narrowing" framing, in OCaml). |
| facebook/infer | **OCaml** | compositional (bi-abduction) + incremental + Pulse; the scale reference, in OCaml. |
| cs-au-dk/TAJS | Java | Møller's abstract-interpretation analyzer for JS — sound-ish AI of a messy dynamic language (our exact predicament). |
| wala/WALA | Java | IBM framework: call graphs, **IFDS/tabulation**, slicing, demand. The "analysis-framework" reference. |
| souffle-lang/souffle | C++ | compiled Datalog for large-scale analysis + provenance. |
| plast-lab/doop | Datalog | declarative points-to at scale (the Soufflé client). |
| SVF-tools/SVF | C++ | sparse value-flow + on-demand; the performance reference. |
| salsa-rs/salsa | Rust | incremental query framework (rust-analyzer's engine) — the incrementality reference. |
| goblint/analyzer | OCaml | (already cloned) abstract interpretation + incremental, OCaml. |
| github/codeql (docs/std lib) | QL | the queryable-fact-base model; lib + docs (engine is closed). |
| (maybe) microsoft/TypeScript | TS | control-flow-based narrowing — the canonical "narrowing" implementation; large, may take docs only. |

## Papers to download (free PDFs; grade A unless noted) — see status in `000-source-manifest.md`
Reps CFL-reachability survey; Reps–Horwitz–Sagiv IFDS; Cousot AI'77 (+ tutorial); Sharir–Pnueli; Weiser slicing; HRB SDG slicing; Tip slicing survey; Gifford–Lucassen effects; Cooper–Kennedy side-effects; Salcianu–Rinard purity; Distefano "Scaling at Facebook"; Arzt–Bodden Reviser (incremental IFDS); Whaley–Lam bddbddb; Bravenboer–Smaragdakis Doop; Scholz Soufflé; Sui–Xue SVF; Jensen–Møller TAJS; Avgustinov QL/CodeQL; McSherry differential dataflow.

## Synthesis (DO NOT START until coverage convinces) — will answer:
Q1 (sound+precise probe reduction), Q2 (reusable reachability/fact structure), Q3 (compositional+incremental+demand+sparse scaling) — as a concrete architecture + data-structures recommendation for Dorc.
