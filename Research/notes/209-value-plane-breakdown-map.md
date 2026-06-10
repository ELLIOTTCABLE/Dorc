# 209 — Where the two-machine value-plane breaks down (capability map, pre-strain)

> Round-20, written mid-round in conversation with the human ("how far can that approach be
> pushed; what common constructs are unrepresentable; when do we need a 'real' engine?").
> Predictive, not measured — the corpus contains none of these shapes yet (a measured gap;
> task-D's floor-boundary strawmen make the stick see the boundary). AI-authored,
> confidence-marked. Companion to 202 §1 (the architecture) and 204 (the dialect).

## §0 The headline answers

- The worklist substrate likely NEVER gets thrown out: every identified break except one is a
  grammar-coverage or domain-richness gap, fixable by enrichment on the same engine (more
  parser, richer-but-finite lattices from the existing combinators, budgeted inlining).
- The one structural exception is deep-call-graph precision — IDE's territory (see §2) — which
  the shallow-books corpus bet says we never hit. -GUESS: never installed.
- For everything data-dependent, the permanent answer is the PROBE, not static machinery:
  runtime values are probe-sourced concretes (inv-probe-sourced-values' sanctioned source),
  which is the capability PLT tools structurally lack. "Our real values engine is the probe."

## §1 Breaks, ranked by (real-world frequency × elision-value lost)

- **brk-1 · `for x in <literal list>` loops** — the single highest-cost miss. Today:
  parser-⊤ ⇒ one opaque box ⇒ havoc + poison; a book installing its package list in a loop is
  invisible. Fix decomposes: (a) parser/CFG models for/while (back-edges — the worklist
  handles cycles by construction, never yet fed one); (b) domain: a flat join (`nginx ⊔
  postgresql = ⊤`) is uselessly weak — loop vars want a bounded **Powerset** of the literal
  list ⇒ the site touches an enumerable cell-family, weak-updated. Render direction
  (~SUSPECT, novel): elide list MEMBERS, not lines — rewrite the iteration list to the
  diverged members (`for pkg in postgresql`) — observable-preserving replacement applied to
  data. Unfixable sibling: `while read < file` — runtime data, ⊤ under any static engine,
  forever (probe/trace territory; kDEPS-accept-partial boundary).
- **brk-2 · user-defined functions** — made urgent by 207 §4/§4b (wrapper-pun oracles REQUIRE
  call-edges). Today: detached bodies, MustRun, no value flow. Fix: **budget-bounded
  inlining** (clone the body's analysis per call site; depth/site budget; ⊤ past it) — gets
  what IDE would get for the shallow shapes users write, on the existing worklist, while
  keeping IFDS's correctness discipline (call/return matching) for free by construction.
  IDE stays the labeled fire-escape iff a real corpus shows deep call graphs.
- **brk-3 · deliberate word-splitting** (`PKGS="a b c"; cmd $PKGS`) — high value, LOW cost:
  with a literal value + untouched IFS the split is computable on paper (one word-node ⇒ N
  argv slots); any IFS write ⇒ havoc. First domain enrichment to schedule.
- **brk-4 · `$(cmd)` substitution of read-only commands** — NOT an engine gap: value
  synthesis stays refuted; the answer is Query-class + the stdout channel — probe it, get the
  concrete, fold it. Same eventual story for `case "$(hostname)"` (probe + collapse).
- **brk-9 · partial-⊤ argv** (`install -y "$dyn"`) — where the clean concrete/abstract split
  erodes first: face-check with ⊤-HOLES (derive verb, entity-⊤ ⇒ kind-wide weak effect
  instead of Opaque poison). Honest hazard: ⊤ in a branch-condition position forces both-arms
  joins = the evaluator quietly becomes an abstract interpreter. Bounded version: ⊤ permitted
  in operand positions only. Watch this one for creak-vs-coverage.
- Lesser, mechanical: parameter-expansion operators on knowns (`${x%s}`, `${x:-d}`) —
  concrete computation, cheap; literal `. ./lib.sh` — parse-level splice + a DST-clean
  file-read seam; arithmetic on knowns; occurrence-typing of case-scrutinees (inc-6) — more
  transfer rules on existing flow-sensitivity.

## §2 The academic ladder, plainly (for future briefs; idiot-level versions in the chat log)

- **Worklist/monotone framework** (what we have): re-walk nodes whose inputs changed until
  fixpoint; knowledge degrades monotonically over a finite-height lattice ⇒ terminates.
  Handles loops natively; intraprocedural.
- **IFDS**: for finite, distributive yes/no facts, a function's effect precomputes into a
  reachability table spliced per call site, with call/return edges matched like balanced
  parens ("realizable paths") ⇒ context-sensitivity at polynomial cost. Cannot carry string
  VALUES (not finite facts; constant-prop is the textbook non-distributive case).
- **IDE**: IFDS edges upgraded to carry small value-TRANSFORMERS (identity / constant /
  concat) from a closed composing family ⇒ per-call-site-precise values through deep graphs,
  still polynomial. Our string flows fit the family; it solves a SCALE problem our corpus
  shape doesn't have.
- **Datalog**: facts + rules, bottom-up total derivation. Buys provenance-for-free (the
  why-elided derivation trees kFIDELITY wants), easy rule extension. Costs: external engine
  in the dep-clean DST kernel (near-weld), flat-relational model vs structured cells,
  materialize-everything memory. Already adjudicated (191 §4): worklist committed; Ascent
  mapped; re-eval trigger = why-trees + structured-domain + DST-tolerance coinciding.

## §3 Sequencing implication (predictive)

Spike corpus: no pressure. First realistic dogfood book: brk-1 + brk-2 bite immediately ⇒ a
near-future round wants parser-loops + Powerset loop-domain + budgeted inlining (one
coherent "grammar+domain" round); brk-3 rides along cheaply. brk-4 lands as a consequence of
task-D's Query/stdout machinery, not as engine work. brk-9 is the deliberate-creep decision
to take LAST, with the operand-only bound.
