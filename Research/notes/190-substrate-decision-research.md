# 190 — substrate decision research (round 19)

**Stamp:** round 19 · 2026-06-08 · bounded confirmatory interactive-research pass (wave 2). Closes the
three gaps left open by `notes/180` (gap-1 TAJS figure, gap-2 Salsa, gap-3 Rust-native lattice-Datalog
currency) against the spike-2 decision to KEEP + extend the hand-rolled monotone-worklist substrate
(rather than IFDS/IDE or Datalog/Soufflé). Sources full-read and graded `graded-by: subagent`; excerpts
verbatim. The decision lean held: nothing found moves it, though gap-3 (Ascent) is a genuine, now-current
Rust-embeddable lattice-Datalog that weakens one specific objection (Soufflé-means-C++-FFI). SPA textbook
TOC reported for chapter assignment (already registered as `[B-moller-schwartzbach-static-program-analysis-2025]`;
not re-registered).

## Findings

- **fnd-1 — gap-1 CONFIRMED verbatim: the "87% → <2%" precision figure is TAJS, exactly as Dorc docs
  claim.** +SURE. Disabling recency abstraction (which is what enables strong updates) on `richards.js`
  drops "constant property is present" guarantees from 87% to 2-of-156 (<2%), and null/undefined deref
  warnings rise 19→90. It is a per-benchmark figure for the fixed-property-read category, not a
  whole-analysis headline. Slug `[A-jensen-moller-tajs-type-analysis-javascript-sas-2009]` already
  registered + claim already audited; quoted below, not re-registered. [A-jensen-moller-tajs-type-analysis-javascript-sas-2009]

- **fnd-2 — gap-2 SETTLED: Salsa is an incremental/demand MEMOIZATION layer, NOT a substrate, and is
  orthogonal to a pure DST batch kernel.** +SURE. Salsa = "a generic framework for on-demand,
  incrementalized computation" over memoized queries (`K→V`) of Inputs + pure Functions; it re-uses
  memoized values when inputs change. It has no fact lattice, no fixpoint/relational engine, no structured
  fact domain. A from-scratch DST-fuzzed batch analysis gains nothing from incremental memoization, so
  Salsa would sit ABOVE the kernel to serve the `dorc try` diff-time hot-loop — it does not replace or
  compete with the worklist. Verdict: layer, not substrate; out-of-scope for the kernel. [B-salsa-incremental-framework-2026]

- **fnd-3 — gap-3 KEY RESULT: a clean Rust-embeddable LATTICE-Datalog now exists (Ascent), and it
  weakens the "Soufflé means a C++ FFI dependency" objection — but does NOT, on net, move the decision.**
  +SURE. Ascent (CC 2022, OOPSLA 2023 for BYODS; v0.8.0, actively maintained to 2026) "extend[s] Datalog
  semantics with non-powerset lattices, much like Flix, and with user-defined data types much like
  Formulog and Souffle," compiled via proc-macros uniformly with host Rust — no FFI, no C++. It
  re-implements the Rust borrow checker and benchmarks comparable to Datafrog and Soufflé. So objection
  (a) genuinely weakens. [A-ascent-seamless-deductive-macros-2022] [A-ascent-repo-docs-2026]

- **fnd-4 — gap-3(b): Ascent's lattice domain composes from exactly the combinators Dorc's fact-algebra
  needs — including the structured/recursive shape.** +SURE. `ascent_base::lattice` ships `Product`
  (product-order over tuples/arrays = "kind embeds kinds as typed fields"), `Lattice for Box<T>`/`Rc`/`Arc`
  (recursion through indirection), `Set` (BTreeSet powerset), `Option`, `Dual`/`Reverse`. The `lattice`
  keyword joins the final column on duplicate-key insert. BYODS (`#[ds(..)]`) lets any relation be backed
  by a custom data structure provider (union-find etc.), changing complexity AND merge-semantics — the
  hook by which a recursive kind-typed domain and strong-update-style custom merge could be hosted.
  So the "fights relational Datalog" half of the decision's premise is only true of FLAT relational
  Datalog (Crepe/Datafrog); a lattice-Datalog (Ascent/Flix lineage) does host the structured algebra.
  [A-ascent-repo-docs-2026] [A-ascent-seamless-deductive-macros-2022]

- **fnd-5 — gap-3(c): Ascent does NOT give provenance/why-trees cheaply (unlike Soufflé). This is the
  decisive cut against adopting it.** ~SUSPECT (strong; from code+README+doc search, not an exhaustive
  proof). Code search for `provenance`/`proof` over s-arash/ascent returned zero; README/BYODS/CHANGELOG
  have no proof-tree facility. Soufflé's first-party lazy minimal-height proof-trees
  (`[A-souffle-provenance-docs-2026]`) have no Ascent equivalent. So the `kFIDELITY` why-tree lever — a
  primary reason Datalog looked attractive (`180 fnd-5c`) — is NOT free on the Rust-embeddable option; you
  would hand-roll provenance on Ascent just as on the worklist. This neutralizes the strongest pull toward
  switching. [A-ascent-repo-docs-2026]

- **fnd-6 — gap-3 DST hazard: Ascent's DEFAULT `par` feature breaks hermeticity; serial Ascent is
  deterministic but still drags a dep tail.** +SURE on the mechanism. Default serial relation type is
  `std::vec::Vec` (insertion-deterministic) with `RelFullIndexType`; lattice values use `BTreeSet`
  (ordered). BUT `default = ["par"]` pulls `rayon` + `dashmap` (parallel, nondeterministic order) —
  must build `default-features=false` for a pure DST kernel. Even serial, the runtime tail is
  `hashbrown` + `rustc-hash` (FxHash — fixed-seed, deterministic, favorable) + `boxcar` + `pastey`;
  proc-macro deps (`syn`/`quote`/`petgraph`/`itertools`) are compile-time-only. Net: lighter than a C++
  FFI, but NOT dependency-free — directly in tension with `AGENTS` "correctness-critical kernels must stay
  clean of nondeterministic deps (or deps at all)." A proc-macro-generated engine is also harder to fuzz
  at the kernel boundary than hand-written code. [A-ascent-repo-docs-2026]

- **fnd-7 — gap-3 comparators: Crepe and Datafrog are LESS fit than Ascent; Datafrog is essentially
  Dorc's status quo.** +SURE. Crepe = proc-macro relational Datalog (semi-naive, stratified negation,
  Soufflé-comparable speed) but PURE POWERSET — no lattices, no structured domain, no provenance; it is
  the flat-finite model the decision already rejects. Datafrog (rust-lang, 882⭐, the polonius engine) is
  "a lightweight Datalog engine… with no runtime, relies on you to build and repeatedly apply the update
  rules" — a manual `from_join`/`while iteration.changed()` fixpoint TOOLKIT, no lattices, deterministic,
  near-zero deps. That is almost exactly what Dorc's hand-rolled monotone worklist already is — so it
  corroborates the worklist as a legitimate, rust-lang-blessed design point rather than a reason to
  switch. [B-crepe-datalog-macro-2026] [B-datafrog-engine-2026]

- **fnd-8 — currency: incremental Datalog-on-Differential-Dataflow is an active 2026 frontier (FlowLog),
  but heavier and not DST-clean.** -0:SUSPECT. FlowLog (VLDB '25/'26; repo pushed the day of this pass)
  compiles Soufflé-compatible `.dl` to standalone Rust executables on Timely + Differential Dataflow — a
  codegen-to-executable model (like Soufflé, emitting Rust+DD not C++), not an embedded library. Pulls the
  heavy Timely/DD stack; not obviously DST-pure. Confirms the differential-dataflow strand
  (`[A-abadi-mcsherry-foundations-differential-dataflow-esop-2015]`, IncA) is alive,
  but FlowLog is a heavier substrate than Ascent and does not move the worklist decision.
  [C-flowlog-datalog-dd-2026] [A-szabo-inca-incremental-datalog-lattices-oopsla-2018]

- **fnd-9 — NET on the decision:** +SURE. The worklist lean HOLDS. gap-3's strongest finding (Ascent is a
  real, current Rust lattice-Datalog) removes the FFI objection and shows the structured domain is
  hostable, but (i) the provenance lever it was wanted for is absent (fnd-5), (ii) it injects
  nondeterministic deps + a proc-macro engine into a kernel that `AGENTS` demands stay dep-clean and
  DST-fuzzable (fnd-6), and (iii) Datafrog shows the hand-worklist is itself the rust-lang-blessed shape
  for this niche (fnd-7). Under the perf-inversion the heavy-substrate scale benefits remain moot. No
  finding rises to "threatens the lean." The one thing worth flagging: if the why-tree/provenance
  requirement (`kFIDELITY`) is ever weighted heavily AND a structured lattice domain is needed AND DST
  could tolerate `default-features=false` Ascent, then Ascent-over-worklist becomes a live re-eval — but
  all three would have to hold at once.

## SPA textbook — chapter list (for chapter assignment)

Møller & Schwartzbach, *Static Program Analysis*, 2025-04-29 (sha256 `19f7dcfb…` matches the registered
copy). Already graded `[B-moller-schwartzbach-static-program-analysis-2025]` — TOC reported, not re-registered.

- **1 Introduction** (1) — 1.1 Applications · 1.2 Approximative Answers · 1.3 Undecidability of Program Correctness
- **2 A Tiny Imperative Programming Language** (9) — 2.1 Syntax of TIP · 2.2 Example Programs · 2.3 Normalization · 2.4 Abstract Syntax Trees · 2.5 Control Flow Graphs
- **3 Type Analysis** (19) — 3.1 Types · 3.2 Type Constraints · 3.3 Solving Constraints with Unification · 3.4 Record Types · 3.5 Limitations
- **4 Lattice Theory** (37) — 4.1 Motivating Example: Sign Analysis · 4.2 Lattices · 4.3 Constructing Lattices · 4.4 Equations, Monotonicity, and Fixed Points
- **5 Dataflow Analysis with Monotone Frameworks** (51) — 5.1 Sign Analysis Revisited · 5.2 Constant Propagation · 5.3 Fixed-Point Algorithms · 5.4 Live Variables · 5.5 Available Expressions · 5.6 Very Busy Expressions · 5.7 Reaching Definitions · 5.8 Forward, Backward, May, and Must · 5.9 Initialized Variables · 5.10 Transfer Functions
- **6 Widening** (79) — 6.1 Interval Analysis · 6.2 Widening and Narrowing
- **7 Path Sensitivity and Relational Analysis** (89) — 7.1 Control Sensitivity using Assertions · 7.2 Paths and Relations
- **8 Interprocedural Analysis** (99) — 8.1 Interprocedural CFGs · 8.2 Context Sensitivity · 8.3 Context Sensitivity with Call Strings · 8.4 Context Sensitivity with the Functional Approach
- **9 Distributive Analysis Frameworks** (111) — 9.1 Motivating Example: Possibly-Uninitialized Variables · 9.2 An Alternative Formulation · 9.3 Compact Representation of Distributive Functions · 9.4 The IFDS Framework · 9.5 Copy-Constant Propagation · 9.6 The IDE Framework
- **10 Control Flow Analysis** (135) — 10.1 Closure Analysis for the λ-calculus · 10.2 The Cubic Algorithm · 10.3 TIP with First-Class Functions · 10.4 Control Flow in Object-Oriented Languages
- **11 Pointer Analysis** (147) — 11.1 Allocation-Site Abstraction · 11.2 Andersen's Algorithm · 11.3 Steensgaard's Algorithm · 11.4 Interprocedural Pointer Analysis · 11.5 Null Pointer Analysis · 11.6 Flow-Sensitive Pointer Analysis · 11.7 Escape Analysis
- **12 Abstract Interpretation** (163) — 12.1 A Collecting Semantics for TIP · 12.2 Abstraction and Concretization · 12.3 Soundness · 12.4 Optimality · 12.5 Completeness · 12.6 Trace Semantics
- Index of Notation (195) · Bibliography (197)

Most load-bearing for Dorc's substrate work: ch.4 (Lattice Theory), ch.5 (Monotone Frameworks — esp. 5.3
Fixed-Point Algorithms, 5.8 Forward/Backward/May/Must), ch.8 (Interprocedural), ch.9 (Distributive — 9.3
compact distributive-function representation, 9.4 IFDS, 9.6 IDE), ch.12 (Abstract Interpretation —
soundness/over-approximation). NB the book has NO slicing chapter (a `021`/`055` co-mention is imprecise;
the real slicing source is Horwitz-Reps-Binkley, `[A-horwitz-reps-binkley-sdg-slicing-toplas-1990]`).

## Citations

### gap-1 — TAJS recency / strong-update precision (the keystone figure)

> [A-jensen-moller-tajs-type-analysis-javascript-sas-2009]:§5 (relevance: +1:SURE)
> "We can disable various features in the analysis to obtain a rough measure of their effect… Using
> recency abstraction is crucial: With this technique disabled, the analysis of richards.js can only
> guarantee that a constant property is present in 2 of the 156 read-property nodes (i.e. less than 2%,
> compared to 87% before) and the number of warnings about potential dereferences of null or undefined
> rises from 19 to 90. These numbers confirm our hypothesis that recency abstraction is essential to the
> precision of the analysis."

> [A-jensen-moller-tajs-type-analysis-javascript-sas-2009]:§5 (relevance: +1:SURE)
> "For 87% of the 156 read-property operations where the property name is a constant string, the property
> is guaranteed to be present." (the 87% baseline; Fig. 2 lists richards.js fixed-property-read = 87%.)

> [A-jensen-moller-tajs-type-analysis-javascript-sas-2009]:§4.2 (relevance: +1:SURE)
> "The effect of incorporating recency abstraction on the analysis precision is substantial, as shown in
> Section 5." … "[the most-recent allocation] object permits strong updates."

### gap-2 — Salsa (incremental memoization layer, not substrate)

> [B-salsa-incremental-framework-2026]:README (relevance: +1:SURE)
> "A generic framework for on-demand, incrementalized computation."

> [B-salsa-incremental-framework-2026]:README#key-idea (relevance: +1:SURE)
> "The key idea of salsa is that you define your program as a set of queries. Every query is used like
> function K -> V that maps from some key of type K to a value of type V… Inputs: the base inputs to your
> system… Functions: pure functions (no side effects) that transform your inputs into other values. The
> results of queries are memoized to avoid recomputing them a lot. When you make changes to the inputs,
> we'll figure out (fairly intelligently) when we can re-use these memoized values and when we have to
> recompute them." (memoization/incremental layer — no lattice, no fixpoint engine, no fact domain.)

### gap-3 — Ascent (Rust-embeddable lattice-Datalog)

> [A-ascent-seamless-deductive-macros-2022]:Abstract (relevance: +1:SURE)
> "We present an approach to integrating state-of-art bottom-up logic programming within the Rust
> ecosystem, demonstrating it with Ascent, an extension of Datalog… Rust's powerful macro system permits
> Ascent to be compiled uniformly with the Rust code it's embedded in and to interoperate with arbitrary
> user-defined components written in Rust." (embedded via macros — no FFI, no serialization boundary.)

> [A-ascent-seamless-deductive-macros-2022]:Abstract (relevance: +1:SURE)
> "We leverage Rust's trait system to extend Datalog semantics with non-powerset lattices, much like Flix,
> and with user-defined data types much like Formulog and Souffle." (the gap-3(b) answer: non-powerset
> lattices + user-defined data types are first-class.)

> [A-ascent-seamless-deductive-macros-2022]:Abstract (relevance: +1:SURE)
> "We use Ascent to re-implement the Rust borrow checker, a static analysis required by the Rust
> compiler. We evaluate our performance against Datafrog, Flix, and Souffle… observing comparable
> performance to Datafrog and Souffle, and speedups of around two orders of magnitude compared to Flix."
> (production-grade; perf-competitive with Soufflé.)

> [A-ascent-seamless-deductive-macros-2022]:§1 (relevance: -0:SUSPECT)
> "a few recent languages (such as Datafun and Flix) further extend Datalog with support for fixed-point
> computations over non-powerset lattices." (Ascent's lineage = the lattice-Datalog family, distinct from
> flat relational Datalog.)

> [A-ascent-repo-docs-2026]:README#Lattices (relevance: +1:SURE)
> "Ascent supports computing fixed points of user-defined lattices. The `lattice` keyword defines a
> lattice in Ascent. The type of the final column of a `lattice` must implement the `Lattice` trait. A
> `lattice` is like a relation, except that when a new `lattice` fact (…, vₙ) is discovered, and a fact
> (…, v'ₙ) is already present in the database, vₙ and v'ₙ are `join`ed together to produce a single fact.
> This feature enables writing programs not expressible in Datalog."

> [A-ascent-repo-docs-2026]:ascent_base/src/lattice/product.rs (relevance: +1:SURE)
> "A wrapper for tuple types and arrays that implements PartialOrd using product-order semantics. Lattice
> and BoundedLattice traits are also implemented." — plus `impl<T: Lattice> Lattice for Box<T>` /
> `Rc<T>` / `Arc<T>` and `Set<T>(BTreeSet<T>)` in the same module. (the structured + recursive + powerset
> lattice combinators Dorc's domain composes from.)

> [A-ascent-repo-docs-2026]:README#BYODS (relevance: +1:SURE)
> "BYODS (Bring Your Own Data Structures to Datalog) is a feature of Ascent that enables relations to be
> backed by custom data structures. This feature allows improving the algorithmic complexity of Ascent
> programs by optimizing the data structures used to back relations… The `#[ds(trrel_uf)]` attribute
> directs the Ascent compiler to use the data structure provider defined in the module `trrel_uf`."
> (custom relation backing = the hook for a recursive kind-typed domain / custom merge.)

> [A-ascent-repo-docs-2026]:ascent/Cargo.toml + ascent/src/rel.rs (relevance: +1:SURE)
> default = ["par"]; par = ["dashmap", "hashbrown/rayon", "once_cell", "rayon"]. Runtime deps: hashbrown
> 0.14, rustc-hash 2.0, boxcar, pastey, web-time. Default serial relation type: `::std::vec::Vec<…>`;
> parallel: `ascent::boxcar::Vec<…>`. (DST: serial Vec-backed = deterministic; the default `par`/rayon
> path is NOT — must disable. rustc-hash = fixed-seed FxHash, deterministic.)

> [A-ascent-repo-docs-2026]:code-search (relevance: +1:SURE)
> Code search for `provenance` and `proof` over s-arash/ascent returned ZERO matches; README/BYODS/
> CHANGELOG describe no proof-tree facility. (Ascent has no Soufflé-style why-trees — the kFIDELITY lever
> is not free here.)

### gap-3 comparators — Crepe, Datafrog, FlowLog

> [B-crepe-datalog-macro-2026]:README (relevance: +1:SURE)
> "Crepe is a library that allows you to write declarative logic programs in Rust, with a Datalog-like
> syntax. It provides a procedural macro that generates efficient, safe code… Features: Semi-naive
> evaluation; Stratified negation; Automatic generation of indices for relations…" — and: "Variants of
> transitive closure for large graphs (~10⁶ relations) run at comparable speed to compiled Souffle, and
> use a fraction of the compilation time." (NB: no `lattice` construct anywhere — pure relational/powerset.)

> [B-datafrog-engine-2026]:README (relevance: +1:SURE)
> "Datafrog is a lightweight Datalog engine intended to be embedded in other Rust programs. Datafrog has
> no runtime, and relies on you to build and repeatedly apply the update rules." (a manual fixpoint
> toolkit — `while iteration.changed() { nodes_var.from_join(…) }` — i.e. essentially Dorc's existing
> hand-worklist, rust-lang-blessed.)

> [C-flowlog-datalog-dd-2026]:README#Architecture (relevance: -0:SUSPECT)
> "Composable Datalog engine that compiles programs into efficient and scalable Differential Dataflow
> executables." … "codegen — Emits the plan as Rust code using Timely + Differential Dataflow." …
> "flowlog-build — Library form. Use from build.rs to compile .dl programs into Rust at build time."
> (codegen-to-executable on the heavy Timely/DD stack — a Soufflé-class substrate, not an embedded lib.)
