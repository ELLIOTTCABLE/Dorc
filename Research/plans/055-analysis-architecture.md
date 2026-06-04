# Dorc analysis architecture — sound probe reduction, reusable structure, corpus-scale performance

> ⟢ **SUPERSEDED-IN-PART (2026-06-01):** the firmest *engine design*, but its *weight* is gated on the `kDEPS` investment split (the unrun corpus go/no-go); the later perf/recovery rounds demoted how far static-derive alone reaches. Treat as the **reference design**, not committed MVP scope. Current synthesis: `Research/plans/083-synthesis-and-spike-charter.md`; design tensions: `KNOBS.md`.

Answering the three questions, assuming a reasonable parsetree over a corpus of files/modules. Grounded in the prior art (notes 50–54); confidence markers throughout; genuine design decisions flagged, not silently resolved.

## 0. One-paragraph thesis
Dorc's analysis is a **compositional, over-approximating may-mutate (MOD) abstract interpretation** over a CFG/supergraph, in which each command's effect is supplied by its **hand-authored oracle** (effect-class + check), propagated interprocedurally via **IFDS/IDE-style summaries** (precise realizable-path reachability), producing per-host / per-function **skip verdicts** plus a **minimal probe set**. The analysis state is retained as a **dependence/value-flow graph exposed as a queryable (Datalog-style) fact base**, so new analyses are new rules and every probe is explainable by provenance. It scales by **per-role summaries + sparsity + demand + incremental clear-and-propagate**, exploiting the embarrassingly-modular structure of an ops corpus (R roles × H hosts). In one line: **Infer's architecture (compositional summaries, diff-time deployment) running an over-approximating MOD analysis (Salcianu–Rinard shape, TAJS-style sound AI) instead of an under-approximating bug-finder.** The hand-authoring of per-command oracles is permanent and large; what the *engine derives is their composition* (the user's reframe), which is exactly what summary-based interprocedural analysis does.

---

## Q1 — soundly reduce parsetree → minimal-but-complete probe set

### 1A — the two soundnesses (probe-soundness + elision-soundness)
Two distinct obligations with *opposite* fail-safe actions (see `kFAIL`). **Probe-soundness**: the
read-only projection must never itself mutate — rests on correct inert-classification (`inert ⇒ really
read-only`); an uncertain leaf is *un-probeable* (defer to apply), never run raw. **Elision-soundness**
(the rest of this section): never omit a necessary probe/mutation. Its necessary-probe set is
`Reachable ∩ MayMutate`, both **over-approximated**:
- **Reachable**: forward reachability over the CFG/supergraph (which commands can execute). Over-approximate — unknown/dynamic control → assume reachable.
- **MayMutate**: a forward **may-modify (MOD) analysis** (the classical analog — Callahan's `may-use`/`must-modify`, expressible in IFDS per note 52; Salcianu–Rinard purity/side-effect per note 51). Each command's transfer function = its oracle's **effect class**:
  - `pure-query` → contributes ⊥ (no prestate mutation; scratch-it-owns is benign — Salcianu–Rinard "mutating only new objects is pure");
  - `mutating(F)` → establishes/kills specific facts `F` (`pkg:foo`, `file:/etc/x`, `svc:foo#enabled`);
  - `unknown` → ⊤.
- **Lattice**: ⊤ = "may mutate unknown state / must-probe"; ⊥ = "provably no prestate mutation". Meet = union (a *may* analysis). The over-approximation guarantee (Cousot AI; TAJS): everything **unmodeled collapses to ⊤** — `eval`, dynamic command names, `source "$dyn"`, an unrecognized command with no oracle, a non-distributive blow-up, an external/non-deterministic read (clock/`$RANDOM`/network). For elision, ⊤ = can't-skip (must-run); for probe-construction the *same* unmodeled leaf is **un-probeable** — excluded from the read-only projection and deferred to ordered apply, *not* "run it to observe." **This is the `unsafe` boundary made formal** — exactly TAJS's "`eval` → emit warning, treat as boundary," and corpus-validated as rare (eval ≈ 9%/4%).
- **The reconciliation with "bounded verification"**: the *model* is bounded (unmodeled constructs → ⊤); the *over-approximation discipline inside the model is absolute*. So "best-effort, not Coq" and "complete certainty no omitted probe" are the same stance on two axes. The cost asymmetry is **phase-dependent**: at apply, false-skip is dangerous and false-run is merely slow (idempotence absorbs it); during probing it *flips* — a mis-classified mutation *run* in the read-only pass is the catastrophe, while a pessimistic "can't-probe" is merely slow. Encoded by the lattice orientation per phase (⊤ is the safe default in each), not by a soundness proof.

### 1B — precision = tighten the over-approximation (few irrelevant probes)
Stacked, each from prior art:
- **Dead-code/reachability pruning** (SPA dataflow; TAJS reports dead code): don't probe unreachable commands.
- **Realizable-path / context-sensitivity** (IFDS/IDE; SDG summary edges; SVF balanced-parens): exclude infeasible call/return paths → tighter MayMutate (meet-over-*valid*-paths, not all-paths). This is the precision win that motivated IFDS.
- **Backward slicing from the skip-decision/dirty set** (Horwitz–Reps–Binkley): drop commands whose effect can't reach the decision = "drop irrelevant probes," computed as graph reachability.
- **Recency / strong-weak abstraction** (TAJS) — *the keystone precision lever*: strong-update the specific fresh state-entity a command touches ("pkg foo now exactly present@X"); weak-update summarized/unknown entities. Without it, "maybe-mutated" proliferates and nothing is skippable (TAJS: 87%→<2% precision when disabled). **Adopt a recency-style split in the effect/fact domain.**
- **Demand** (IFDS-demand; Heintze–Tardieu; Reps cc94): answer "does host *h* / role *r* need probing for fact *F*?" by computing only the queried slice.
- **Effect characterization** (Salcianu–Rinard regex of touched locations): probe only the *specific* state a command can touch (fact-granular), never "something changed."

### The engine for Q1
- **IFDS/IDE** for the distributive **fact layer** — establish/require/conflict facts are gen/kill, hence distributive, so IFDS applies and yields precise poly-time realizable-path **summaries** (note 52). For value-carrying refinements (versions), IDE (environment transformers).
- A **separate finite effect/fact lattice** (recency strong/weak) for the points-to-like **"which state is touched"** characterization — not purely distributive → coarser, finite, ⊤-on-unknown. Monotone-framework AI (TAJS/SPA).
- Both share one supergraph + fact base. **The hybrid (IFDS fact layer + AI effect layer) is a real design decision — flag it (§decisions).**
- **Cheap fast path**: Tier-A intra-function skip (the planning-log 90%) needs only plain intraprocedural monotone dataflow (SPA §5); IFDS's interprocedural machinery earns its cost only at the cross-function / `source` tier (Tier-B). The engine should special-case the intraprocedural majority.

---

## Q2 — retain reachability/tracing for evolving analyses + output quality

- **Retained structure = a dependence/value-flow graph** (PDG/SDG per Horwitz–Reps–Binkley + value-flow edges per SVF), built once over the IR, carrying control + data + effect dependences. Many analyses become **reachability/slicing queries** over it (HRB: slicing *is* reachability; SVF: source→sink *is* reachability). Build once, query many ways.
- **Expose it as a Datalog-style fact base** (Soufflé/Doop/QL model, note 53): facts = relations (`cfg_edge`, `may_mutate`, `establishes(cmd,fact)`, `requires(cmd,fact)`, `reachable`, …); **analyses = rules (IDB)**; **adding an analysis = adding rules**, no pass-rewriting. Because **IFDS ≡ Datalog** (note 52), the core analysis itself is expressible as rules — so the engine and the extensibility layer are one substrate. This is the strongest answer to "retain reachability info for *further* analyses as the tool evolves."
- **Explainability / output quality = provenance**: Datalog provenance (Soufflé why-trees) or the dependence-graph path answers "**why** is this probe necessary / why is this host not skippable" with a real derivation. This is the Terraform-plan "what will change and why," backed by a trace — surfaced in the plan/apply diff UI.
- **Forward slice from a diff = impact analysis** (HRB forward slicing): "this role edit affects these facts on these hosts" — the diff-time view.
- Graph ⇄ relations are duals; keep both views — the graph for slicing/sparsity, the relations for extensibility/provenance.

---

## Q3 — performance on an arbitrarily-large org corpus

The corpus is **embarrassingly modular** (R roles × H hosts, mostly independent — the planning-log "flat forest"). Every scaling lever from the literature maps onto this structure:
- **Compositional per-role summaries** (Facebook scale; IFDS/SDG summary edges; Salcianu–Rinard per-method summaries): analyze each role/function **once** → a summary (its require/establish/may-mutate facts, parameterized over calling context); recompose per host. Runtime ≈ Σ per-role costs; independent roles → parallel; "each procedure visited a few times" (Facebook). IDE's "compute the summary once, re-apply at every calling context" is exactly per-host reuse.
- **Per-host = summary instantiation + probe, not re-analysis**: instantiate cached role summaries against each host's facts → emit that host's probe set. H hosts ≈ H cheap instantiations.
- **Sparse value-flow** (SVF): propagate along def-use/effect edges only, skipping irrelevant program points.
- **Region / fact-domain partitioning** (SVF memory regions): partition system-state into fact-domains (`pkg`/`file`/`svc`/`user`/`port`/`mount`) — tune precision vs cost per domain; most commands touch one domain. This is also the planning-log's Tier-B canonical-fact set and the CoLiS UNIX-utils ontology seed.
- **Demand** (IFDS-demand / Heintze–Tardieu / Reps cc94): "does host *h* need probing?" computes only *h*'s relevant slice.
- **Datalog-compiled-to-native** (Soufflé): if the fact-base route is taken, the whole-corpus fact computation compiles to indexed, parallel relational algebra — billions of tuples (far beyond any plausible ops corpus; OpenJDK points-to in 35 s).
- **Incremental (the "evolve the tool / re-run on change" half of Q3)**:
  - **Compositional ⇒ naturally incremental** (Facebook): change one role → re-summarize only it + transitive callers/dependents; reuse the rest.
  - **Clear-and-propagate** (Reviser, for the IFDS/IDE route): diff the ICFG, mark affected = transitively-reachable-from-changed (over-approximate = safe), clear + re-propagate only affected jump/summary functions. ~80% saved, identical results.
  - **Semi-naïve delta** (Soufflé) / **DRedL lattice-Datalog** (IncA) for the Datalog route; **memoized demand queries** (Salsa / rust-analyzer) for the query route.
  - → **Diff-time deployment** (Facebook's 0%→70% fix-rate lesson): analyze the ops-repo diff, recompute only affected summaries, show what-changes-where fast. This is Dorc's primary workflow and the concrete thing that obsoletes Ansible's "re-run the whole fleet for one role."

---

## The architecture, assembled
```
parsetree
  → IR (lossless, schema-defined)
  → Graph builder      : CFG/supergraph + dependence/value-flow edges
  → Fact extraction    : EDB relations (cfg_edge, calls, establishes, requires, …)
  → Analysis (Rules)   : reachability + may-mutate over the effect lattice
        • per-command transfer/effect-class supplied by the ORACLE (the pluggable Rules)
        • IFDS/IDE summaries for the distributive fact layer (realizable paths)
        • recency strong/weak in the effect/characterization layer
        • ⊤-on-unknown (eval/dynamic/no-oracle/non-det) → un-probeable + can't-skip  ← the unsafe boundary
  → Per-role summaries (cached, composable)
  → Per-host instantiation + demand
  → Probe set + backward slice + provenance
  → ship read-only probes → plan/apply
```
**Factoring** (convergent across SVF / Goblint / Soufflé): **Graph / Rules(+oracles) / Solver**. The oracle library plugs in *as Rules* (a command's effect-class + check); the engine (Graph + Solver) is fixed — this is the CoLiS "engine generic over per-command specs," realized with our probe-the-host oracles instead of symbolic specs.
**Soundness** (two — *probe*: projection never mutates, via correct inert-classification; *elision*: never skip a needed mutation): over-approximate; ⊤-on-unknown ⇒ un-probeable + can't-skip. **Precision**: realizable-path + slice + recency + demand. **Scale**: per-role summaries + sparse + region-partition + demand + Datalog-native. **Incremental**: clear-and-propagate / semi-naïve / memoized, at diff-time.

---

## Genuine design decisions (flag, do NOT silently resolve)
1. **Engine substrate** `[DECISION]`: hand-rolled **IFDS/IDE solver** (arbitrary lattices incl. recency, precise summaries — but you build it; Heros/Goblint as references) **vs Datalog/Soufflé** (extensibility + provenance + scale for free — but relational domain makes recency/strong-weak awkward; IncA's lattice-Datalog narrows the gap) **vs hybrid** (IFDS/IDE core producing facts + a Datalog/relational query layer over them). SUSPECT the hybrid is the sweet spot; needs a spike. This interacts with the Phase-2 language choice.
2. **Distributive split** `[DECISION]`: confirm the fact layer (establish/require/conflict gen-kill) is IFDS-distributive while the effect-characterization (points-to-like) is the separate AI layer. Get this boundary right or the engine is either imprecise or non-terminating.
3. **Recency granularity** `[DECISION]`: what is a "singleton" state-entity — a specific `pkg@version`? a path? a `svc` unit? Determines strong-update precision.
4. **Context-sensitivity dial** `[DECISION]`: per-host? per-role-invocation? (TAJS: context-sensitivity dominates precision on the harder cases.)
5. **Fact-domain partition + canonical ontology** (Tier-B): the ~dozen high-fanout facts (planning-log) = the SVF region partition = the CoLiS UNIX-utils ontology seed. Deferred per planning-log, but reserve the representation now (fact = `(opaque-token, source-expr)` so aliasing canonical predicates later is non-migrating).
6. **Soundness calibration (no Coq, per the verdict)**: differential testing (analysis prediction vs actually-running-the-mutate on container fixtures) + property tests of **both** kernels — `inert-classified ⇒ provably no mutation` (probe-soundness: pure-marked leaves really are read-only) and *skip fires only on positive proof of convergence* (elision-soundness). The one place a *targeted* property/Why3 check could be justified (v2): the inert-classification kernel.

## Coverage honesty (the gate the user set)
Deep-read in full: IFDS (Reps–Horwitz–Sagiv), purity/MOD (Salcianu–Rinard), Facebook-scale (Distefano et al.), Soufflé (Scholz et al.), SVF (Sui–Xue), SDG-slicing (Horwitz–Reps–Binkley), TAJS (Jensen–Møller–Thiemann), Reviser (Arzt–Bodden) — plus the shell corpus (CoLiS/Morbig/Bash-in-the-Wild/Dozer) and the SPA textbook's structure. Concept-covered (read abstract + adjacent treatment, not in full; would refine sub-mechanisms, not change the architecture): CFL-reachability survey (subsumed by IFDS + SVF's use of it), Lucassen–Gifford effect systems, Heintze–Tardieu / Reps demand, IncA / differential-dataflow incremental-Datalog, QL/CodeQL productization, Cousot'77 original (image-only scan; theory covered by SPA). Cloned for reference: flow, infer, TAJS, WALA, souffle, doop, SVF, salsa, codeql, goblint (+ the shell repos). The Møller/Aarhus lineage (SPA textbook → TAJS → shstats) is the methodological throughline and the strongest single body to lean on, being both academic-rigorous and dynamic-language-pragmatic.
