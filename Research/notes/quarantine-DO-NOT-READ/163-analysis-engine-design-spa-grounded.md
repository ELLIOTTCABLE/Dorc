# 163 — analysis-engine design, grounded in SPA §4–§5

> **Status (2026-06-05): spike, pre-`analysis`-crate.** Synthesis of Møller–
> Schwartzbach *Static Program Analysis* §4 (lattices) + §5 (monotone frameworks),
> read in full, mapped onto Dorc's Tier-A analyzer. Persists the grounding so the
> raw textbook can compact. Source: `Research/sources/B-moller-schwartzbach-…txt`;
> learning-path map `Research/learning-path/README.md`. Confidence: the SPA
> machinery is +SURE (textbook, 30–50yr stable); the Dorc mapping is mine.

## 0. The shape in one paragraph
Dorc's analyzer is a **generic monotone-dataflow framework** (SPA §5): a complete
lattice `L` of finite height + a per-CFG-node **transfer function** `t_v : L → L`,
solved to the **least fixed point** by a **propagation worklist** (§5.3/§5.10).
Everything is parameterized over ⟨`L`, `t`, **direction**, the lattice's `⊔`⟩, so
forward/backward and may/must are *configuration, not separate engines* (§5.8).
Dorc's "may-mutate" is a textbook **forward-may** analysis; its ambient∧invariant
hoist-gate is **reaching-definitions** (§5.7) over the system-state fact store;
its apply-phase minimization is a **backward** slice from the dirty set. The
unmodeled collapses to an absorbing ⊤ (`inv-top-reject`).

## 1. The generic framework (what `analysis::{lattice,solve}` build)
- **`trait Lattice: Clone + Eq`** — `fn bottom() -> Self; fn join(&self, &Self) ->
  Self;`. `leq` derivable (`x ⊑ y ⟺ x ⊔ y == y`, SPA Exercise 4.2). Contract
  (not type-enforceable, so a doc-invariant + property-tests): `join` is
  associative/commutative/idempotent, `bottom` is its identity, and the lattice
  has **finite height** (guarantees the Kleene chain `⊥ ⊑ f(⊥) ⊑ f²(⊥) …`
  terminates — SPA §4.4). Transfer fns must be **monotone** (more-precise-in ⇒
  more-precise-out); property-test it (SPA Exercise 5.2 monotonicity check).
- **Lattice combinators** (SPA §4.3 — build domains compositionally, don't hand-
  roll each): `Powerset<T>(BTreeSet<T>)` join=∪ (a *may* domain; reverse-order
  ⊇/∩ for *must*); `Flat<T>` = `{Bottom, Elem(T), Top}` (height 2, e.g. the
  Sign/constant pattern, and our per-fact qualifier); `Product<A,B>` componentwise;
  `MapL<K, V: Lattice>(BTreeMap<K,V>)` pointwise (the `A → L` map-lattice — Dorc's
  fact-store is `Map<Fact, Qualifier>`); `Lift<L>` adds a fresh ⊥ (reachability).
  **`BTreeSet`/`BTreeMap` not Hash** — iteration is observable in output, must be
  deterministic (`inv-determinism`).
- **`solve(cfg, transfer, direction) -> IndexVec<CfgNodeId, L>`** — the
  PropagationWorkListAlgorithm (SPA §5.10, the efficient variant): init all ⊥,
  worklist = all nodes; pop `v`, compute `y = t_v(state[v])`, for each `w ∈ dep(v)`
  set `state[w] ⊔= y` and re-enqueue `w` if it changed. **`dep = succ` for
  forward, `pred` for backward** (§5.8) — the *only* thing direction changes.
  Terminates: each step either climbs `L` (finite height) or shrinks the worklist.
  Complexity **O(n·h·k)** (n nodes, h height, k transfer-cost). [Network reminder:
  this is dwarfed by the SSH round-trips that follow — do not micro-optimize it.]

## 2. Dorc's analyses on the SPA quartet (§5.8)
| analysis | dir | may/must | lattice | transfer = | purpose |
| --- | --- | --- | --- | --- | --- |
| **may-mutate / effect** | fwd | may (∪/⊆) | `Powerset<EffectFact>` | command's oracle effect-class (gen) | what state each point may have touched |
| **ambient∧invariant gate** | fwd | (reaching-defs, ∪) | `Powerset<FactDef>` | oracle effect map gen/kill of fact F | "does any in-script def/kill of F reach here?" → not-hoistable (note 162 O-1) |
| **apply-minimization slice** | **bwd** | may | `Powerset<MutationId>` | dirty-set seed + dep edges | given what changed, what must re-run (Tier-B/§9; framework is direction-generic now) |
| **ShellEnvState** | fwd | (flat per option) | `Product<errexit:Flat, cwd, traps…>` | `set`/`cd`/`trap` nodes | the haz-seterr/subshell model |

- **The two soundnesses orient ⊤ per phase** (`kFAIL`, note 162): probe-phase ⊤
  = un-probeable (withhold); apply-phase ⊤ = must-run (perform). Same lattice,
  opposite safe default per `core::Phase`.
- **Reaching-definitions IS the ambient gate** (the key reuse): SPA §5.7
  `[[v]] = JOIN(v) ↓X ∪ {X=E}` — substitute "assignment to var X" with "oracle
  effect on fact F". A book mutator that establishes `package:nginx` is skippable
  only if `package:nginx` is *not* re-defined/killed upstream in-script (reaching-
  defs says so) AND the probe says it holds AND it's hermetic-ambient. The
  `purge…install` wrong-skip (note 162 O-1 / break-10) is caught precisely because
  the purge's kill of `package:nginx` *reaches* the install.

## 3. CFG construction (SPA §2.5) + the hazard set (the coupling)
- **Nodes**: one per simple-command / condition / redirection-site; merge nodes at
  branch joins (SPA Exercise 5.19 — a no-op merge node bounds `|pred|,|succ| ≤ 2`,
  keeping the worklist cheap). Each node references its `AstId` (provenance,
  dac-B). An `Unsupported` AST node → a CFG node whose transfer is **const ⊤**
  (absorbing; `inv-top-reject`).
- **`set -e`/`errexit` is NOT a pre-pass** (haz-seterr, the subtlest): the exit-
  edge after a fallible command exists *iff* `errexit` holds there, and `errexit`
  is itself a forward dataflow fact (it can be toggled `set +e`/`set -e`, even via
  `$-`). So CFG-edge existence is partly an analysis *output* → build a base CFG,
  then let the ShellEnvState analysis *add* the conditional exit-edges (or model
  the edge as guarded). **Do not assume a clean build-then-solve split** — this is
  the one place the spike must couple them. v1: compute `errexit ∈ {on,off,⊤}`
  first (a tiny forward analysis), then materialize exit-edges; ⊤ ⇒ assume the
  edge may exist (conservative).
- **Subshell `( )` / `$( )` scope** (haz-concurrency): a sub-CFG whose env/var
  effects are *projected out* on exit (don't escape) but whose FS effects do. The
  ShellEnvState push/pops a frame at the boundary.
- **Redirections are their own effect-bearing nodes** (haz-redir-as-mutation),
  not cosmetic children — `: > /etc/x` mutates regardless of the command word.
- **`trap`** registers a handler-edge (contract, not detector — 09A); v1 may join
  the handler's effects into the function effect-set conservatively.

## 4. Rust module shape (`analysis` crate)
```
analysis::lattice   — trait Lattice + Powerset/Flat/Product/MapL/Lift + property-tests
analysis::solve     — solve(cfg, transfer, Direction) worklist; Direction{Forward,Backward}
analysis::cfg       — Ast → Cfg (nodes, pred/succ); ShellEnvState; hazard edges; ⊤-nodes
analysis::effect    — the may-mutate + ambient-gate analyses (instantiate the framework)
```
`solve` and `lattice` are **pure + analysis-agnostic** (testable with a toy
sign-analysis, like SPA's running example — a good first test that validates the
solver before any Dorc-specific analysis). `cfg`/`effect` carry the sh-specific
modeling. The oracle effect-class (note 162's `EffectMap`) feeds `effect`'s
transfer functions; the framework is generic over it.

## 5. Tier-A now / Tier-B reserved (SPA §8–§9)
- **Tier-A (build now)**: intraprocedural monotone dataflow over one script's CFG
  — the ~90% case (055). The framework above. k=0 context-insensitive (the
  EXPTIME redline, 071/kCONTEXT): no call-context, flat fact domain.
- **Tier-B (reserve seam, don't build)**: §8 interprocedural (functions / `.`-
  source → an inter-procedural CFG / supergraph with call/return edges) and §9
  IFDS/IDE (precise distributive interprocedural facts + the backward program-
  slice for sub-host minimization). The CFG must be **supergraph-addressable** (a
  call node can name a callee) and the fact domain **finite + distributive** (gen/
  kill qualifies — SPA Exercise 5.26/5.34) so IFDS slots in later without a
  substrate re-pour. Read SPA §8/§9 when building Tier-B; not now.
- **Not for Dorc** (map's skip list): §6 widening (no infinite-height/interval
  analysis), §10 CFA closure-analysis depth, §11 precise pointer analysis (we
  ⊤-approximate aliasing, W2), §12 AI Galois formalism (we disclaimed soundness-
  as-goal — lattice intuition suffices).

## 6. Direction-genericity is a day-1 requirement (the human's flag)
Build `solve` parameterized by `Direction` from its first commit — backward is
just `dep=pred`/`JOIN over succ`. Retrofitting direction touches the worklist
core. Forward carries the may-mutate + ambient-gate + ShellEnvState; backward
carries the apply-minimization slice + dead-probe elimination (liveness-of-facts,
SPA §5.4 is the template). Both ride one engine.
```
