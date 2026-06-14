# 220 — research: engineering the value-provenance plane (receipts)
<!-- /* slug corrected 2026-06-11: drafted under provisional title "21H" before being
filed under the 22x research convention; the REAL 21H is the y-1/q-2 build note. Old
slug preserved here for grep. (note 217 §7) */ -->

> Deep-research round, 2026-06-11, serving the 21G §2 layer-2 direction (every value carries
> receipts: `value × set-of-contributing-origins`, built forward, queried backward, surfaced as
> "why is this ⊤? who contributed?"). **Extends** `plans/111` (round-11 error/provenance
> synthesis); deliberately does NOT re-cover its ground (PROV vocabulary, locator lists, OTel,
> Puppet/K8s/Terraform, EXPLAIN, parser-recovery). New ground: per-value receipt *representation*
> engineering, provenance query/UX engineering, the why/how/where formalism mapped to
> elision-justification, failure post-mortems, and prior art for the one-way safety rule.
> Findings slugged `vp-N`; sources `[grade-slug-year]`, graded list in §7. Confidence marks
> per project convention (+SURE / ~SUSPECT / -GUESS / --WONDER).

## §0 Conclusions up front

**r-1 (cheap receipts).** +SURE the engineering consensus across five independent system families
is one shape: *a constant-size per-value annotation pointing into an engine-owned, append-only,
shared side-structure; all richness recovered by lazy backward search at query time.* Soufflé
attaches exactly two extra numbers per tuple — (producing-rule, minimal-proof-height) — and
re-derives proof trees on demand, at 1.27× runtime / 1.45× memory over tens of millions of tuples
[A-souffle-prov-2020]; rustc packs location+hygiene+parent into an 8-byte `Span` with an interner
fallback hit <0.1% of the time — and its earlier 4-byte version was *slower* because only 80–90%
fit inline (over-compression backfires) [A-rustc-spanenc-2026]; ProvSQL stores one generic
provenance *circuit* (hash-shared DAG, memory-mapped) and specializes to any semiring lazily, at
2–3× [A-provsql-2025]; Smoke shows capture must live *inside* the engine's hot loop (a single
virtual call per tuple into a "lineage subsystem" costs 10×+) [A-smoke-2018], and Titian shows the
external-store alternative costs up to 86× (Newt/MySQL) vs <1.3× in-engine [A-titian-2016].
Taint systems shipped only by brutal boundary-coarsening: TaintDroid's whole receipt is a 32-bit
set, one tag per array, one per IPC message → 14% overhead [B-taintdroid-2010]. Salsa adds the
complement trick: a coarse *tier summary* (durability version-vector, derived as min-of-inputs)
that lets revalidation skip entire receipt subgraphs without touching them [B-ra-durability-2023].

**r-2 (query/UX).** +SURE the legible systems share five habits, and the noise-failures are
exactly their absences: (i) *minimal witness first* — Soufflé returns minimal-height proofs
specifically to minimize user interactions [A-souffle-prov-2020]; `nix why-depends` prints one
shortest path, `--all` is opt-in [B-nix-whydepends-2026]; (ii) *fragment-and-expand*, never the
whole graph — Soufflé `setdepth`+`subproof` labels, ProvSQL Studio frontier-expansion, Pernosco's
one-origin-hop-per-click [A-souffle-docs-2026][B-provsql-github-2026][B-pernosco-demo-2021];
(iii) *concrete evidence per edge, in the user's own artifact* — nix `--precise` shows the literal
file fragment causing each edge; (iv) *reason-category plus the delta* — Bazel `--explain` names
the category ("action command has changed") but users still flounder because the delta (which
input) needs `--verbose_explanations`, and ninja's `-d explain` dumps pages of cascade including
not-actually-dirty nodes [C-bazel-explain-2026][C-ninja-explain-2014]; (v) *suppression heuristics
are the deliverable* — Clang SA's condition-tracking exploded reports from ~10 to ~170 notes and
shipped only after don't-track-asserts / nested-calls-only / prune-trivial-functions heuristics
[B-szelethus-gsoc-2019]. Negative queries ("why NOT") resist automation everywhere (Soufflé's
`explainnegation` is user-guided); Dorc dodges this because refusals are *positive events* in the
analyzer (the license check that failed), already nameable — the round-21 refusal taxonomy is the
right substrate. ~SUSPECT that's a genuine structural advantage over the Datalog systems.

**r-3 (which formalism).** +SURE of the hierarchy (how ⇒ why ⇒ lineage by homomorphism; where is
orthogonal) [B-cheney-whw-2009][B-green-tannen-2007]; mapping to Dorc: *lineage* (flat origin set)
answers "who contributed to this ⊤" — cheapest, right default for every abstract value; *one
stored minimal witness* (why-provenance's unit) answers "this substitution is licensed because
probe-record X + oracle-claim Y" — store the witness the license was actually granted on, at the
license only; the *alternatives structure* (full why/PosBool) answers "does the elision survive if
claim Y is retracted?" — needed only at retraction/staleness time and replaceable by re-running
the analysis (over-invalidation is safe in the `kFAIL-perform` direction, it costs probes never
correctness); full *how*-semiring (ℕ[X] polynomials: multiplicity, exponents) answers no Dorc
question — avoid; *where*-provenance is Dorc's existing Span/source-map plane (which bytes did
this surfaced sh text copy from) — keep it a separate plane per 111's locator list, don't fuse it
into origin sets. The flat product `value × set-of-origins` from 21G is exactly *lineage* — +SURE
it under-serves precisely one consumer (retraction), and re-derivation covers that.

**r-4 (what drowned systems).** +SURE payload bloat per se killed almost nobody — everyone lands
1.3–3× and lives. The recurring killers: (1) *receipts outside the engine* — Newt's external
MySQL lineage store hit 86× and couldn't finish at 500 GB [A-titian-2016]; (2) *host-fork
packaging* — the uncertain-DB graveyard (Trio pinned to PostgreSQL 8.2/8.3; Perm and Orion
unmaintained forks of PG 8.3/8.4; ORCHESTRA's deps vanished; MystiQ needs Java 5) vs the
survivors, all extensions/middleware/libraries (ProvSQL, GProM, Titian) [A-provsql-2025];
(3) *capture without a consumer* — PASS's observed-provenance noise (output linked to every
Python module the interpreter loaded), the n-by-m black-box problem, storage pruning left as an
open challenge [B-pass-2006][B-carata-primer-2014]; (4) *unbounded precision* — DroidSafe, the
most precise of the Android taint tools, cannot analyze real Play-store apps at all; FlowDroid
survives via k-bounded access paths [B-tse-taint-compare-2021]; (5) *raw-dump UX* — Clang's note
explosion, ninja's cascade pages; users tune ignored channels out (corroborates round-11's
[B-darling-plan-warnings-2022]). Predictors, inverted, are the design rules.

**r-5 (one-way rule prior art).** +SURE the asymmetry is named, in three literatures. (a) IFC:
*declassification/endorsement* are the only sanctioned downgrades, everything else monotone;
Sabelfeld–Sands' "who" dimension — *robust declassification* — states that untrusted influence
must not control a release point [B-sabelfeld-sands-2009]; Flume operationalizes it: declassify/
endorse only via an explicitly-held capability [B-flume-2007]; Livshits–Chong document that
humans misplace these permit-points (hence: make them few, explicit, owned — Dorc's oracle-claim
is the declassifier, written in sh) [B-livshits-chong-2013]. (b) Capability systems: CHERI names
*provenance validity* + *monotonicity* + *intentionality* — but as the contrast case: CHERI's
provenance IS authority, sound only because every derivation is hardware-enforced with
machine-checked proofs [A-cheri-rules-2020]. Dorc's chain is best-effort/soundy, so receipts must
stay on the refuse/explain side — provenance-as-authority demands a sound chain Dorc explicitly
doesn't promise. CHERI's *intentionality* (use only the capability handed to you, though you hold
more) is the license discipline: elide only on the cited witness, never on ambient knowledge.
(c) Dorc already encodes the rule internally: `an-orientation-coercion` (Must→May legal, May→Must
a compile error). Receipts are may-side metadata; licenses are must-side facts; the type system
should make `ProvId → License` non-constructible. Corollary (inverting Sabelfeld–Sands'
*monotonicity of release*): adding/stripping receipts must never change a verdict — an
**erasability gate** (analyzer output invariant under receipt-stripping) is CI-checkable from day
one, the same shape as Racket's no-execution-dependency label edge from round-11.

---

## §1 r-1 evidence — representation engineering

- **vp-1 · the two-word receipt.** Soufflé's provenance mode stores, per tuple, only
  `(rule-id, min-proof-height)` as two extra columns (`path(1,3)` becomes `path(1,3, 2,4)`); the
  height transfer is `max(body heights)+1` computed inside normal semi-naïve evaluation; proof
  trees are *not stored* — a backward search guided by the annotations reconstructs one level at a
  time, lazily, per user query, with no re-evaluation of the program [A-souffle-prov-2020]
  [A-souffle-docs-2026]. Measured on Doop/DaCapo (tens of millions of tuples): 1.27× runtime,
  1.45× memory on average. +SURE this is the closest published match to Dorc's engine shape
  (bottom-up fixpoint over a lattice; recursive rules; minimal explanations wanted), and the
  annotation-not-structure trick is the load-bearing idea: *store enough to re-derive the next
  level of the explanation, nothing more.* The height-minimality also serves the lattice problem
  Dorc shares: in cyclic derivations (their Fig. 4) there are infinitely many proofs; the height
  annotation picks the shortest deterministically.
- **vp-2 · inline-or-intern, measured.** rustc's `Span` is 8 bytes (`lo:u32, len:u16-1bit,
  ctxt/parent:u16`) with four formats; anything that doesn't fit goes to an interner table and the
  span stores an index. 99.9%+ fit inline. The doc comment records two hard lessons: the earlier
  *4-byte* Span was **slower** end-to-end because only 80–90% of spans fit inline and the interner
  was hit too often; and field widths were chosen from measured distributions (len peaks at 3–4
  bits; `lo` kept at full 32 bits *specifically* so there is "no performance cliff if a crate
  exceeds a particular size") [A-rustc-spanenc-2026]. +SURE the meta-lessons transfer wholesale:
  measure the receipt-size distribution before packing; keep the common case inline; never create
  a size cliff. ~SUSPECT Dorc doesn't need the packing at all at script scale (vp-15).
- **vp-3 · chains as IDs into a global side-table.** rustc hygiene: a `SyntaxContext` is a u32
  naming a whole chain of `(ExpnId, Transparency)` marks; chain data lives in one global
  `HygieneData` table; `SyntaxContextData` is "mostly a cache for results of filtering that chain
  in different ways"; `Ident = Symbol + Span` — i.e. every identifier carries its full expansion
  provenance for the cost of a u32, because the structure is shared/interned once
  [A-rustc-hygiene-2026]. Three separate hierarchies (expansion order / definition / call-site)
  hang off the same `ExpnId`s — multiple edge-types over one node arena, which is round-11's di-2
  (≥3 edge-types) realized cheaply. Also: span accesses under incremental compilation *register a
  dependency* (`SPAN_TRACK`) — receipts participate in the dependency engine rather than floating
  beside it.
- **vp-4 · capture inside the hot loop or not at all.** Smoke (VLDB'18): even one virtual call per
  tuple into a separate lineage subsystem slows operators >10×; their design principles — P1
  tight integration (lineage logic compiled into the physical operator), P4 reuse (the hash table
  a join builds anyway *becomes* the lineage index), P2/P3 workload-awareness (if the consuming
  queries are known up front, don't materialize lineage they won't read; push their logic into
  capture) — got capture overhead to ~zero-ish and lineage queries to interactive (<150 ms)
  [A-smoke-2018]. +SURE P4 has a direct Dorc analog: the analyzer's own dataflow/CFG edges are the
  receipt edges (dac-B from 111, independently re-derived in the DISC world); ~SUSPECT P2/P3 too —
  Dorc's "consuming queries" are knowable up front (⊤-blame, license-witness, dashboard why-not),
  so the plane can be designed against that closed query set rather than "any future query."
- **vp-5 · external receipt stores are fatal; in-engine is cheap.** Titian: instrumenting Spark
  with Newt (lineage → MySQL cluster) cost up to 86× job time and could not complete at 500 GB —
  "MySQL could not sustain the data lineage throughput"; their RAMP-style reimplementation inside
  the engine: 2.3×; Titian proper (lineage as native RDDs/in-memory indexes, queried *in the same
  Spark terminal via the same RDD API*): rarely >30% [A-titian-2016]. Also a usability finding:
  with Newt, a trace was an iterative SQL loop in a Python script outside the host environment —
  receipts queried in a foreign language don't get queried.
- **vp-6 · coarsen at boundaries; cap the set.** TaintDroid: the entire per-value receipt is a
  32-bit bit-vector (32 possible origins, union = bitwise-or); arrays carry ONE tag for the whole
  array; IPC carries one tag per *message*; native code one tag per method — explicit
  granularity-coarsening at every boundary where precision would cost; result 14% on CPU-bound
  microbenchmarks, shippable on 2010 phones [B-taintdroid-2010]. FlowDroid's static equivalent is
  the k-bounded access path [C-flowdroid-2014]. +SURE the transferable rule: bound the per-value
  receipt size by construction (k-cap + explicit truncation marker), never let the origin-set
  domain become the complexity driver. The cost is documented too: coarse units over-taint
  (false positives) — for Dorc that direction is safe (over-attribution = more refusals/probes,
  `kFAIL`-aligned), unlike for TaintDroid where it meant false alarms.
- **vp-7 · one generic structure, many lazy views.** ProvSQL captures a single semiring-agnostic
  provenance *circuit* (gates shared across results, stored in memory-mapped files, UUID handles)
  and evaluates it in whichever semiring (Boolean, counting, probability, Shapley, where-) the
  user asks for *later*; provenance overhead ≈ constant 2–3×; scales past GProM via circuit
  compactness (sharing) [A-provsql-2025]. Two negative results worth keeping: standard
  multiset-difference (`EXCEPT ALL`) makes provenance computation intractable — they *changed the
  operator semantics* (to `NOT IN`) rather than pay it; and aggregation provenance (semimodules)
  is supported only at top level. +SURE the lesson: when a construct's provenance is intractable,
  redefine the construct's contract rather than approximate silently — Dorc analog: a sh construct
  whose receipt would be unbounded (e.g. `eval`-shaped flows) gets one Opaque receipt by contract,
  not a best-effort partial set.
- **vp-8 · a coarse tier-summary above the fine receipts.** Salsa durabilities: a version-vector
  with one counter per durability tier; each query stores its durability = min of its inputs'
  tiers; revalidation compares one counter and *skips the entire subgraph* when the tier hasn't
  moved — added because merely walking the fine-grained dependency graph of stdlib queries cost
  ~300 ms per keystroke [B-ra-durability-2023]. +SURE Dorc analog: origins fall into natural
  tiers (oracle-library claims ≪ book source ≪ per-host probe results ≪ per-run runtime facts);
  a per-tier epoch makes "did anything from tier X change" O(1), so `dorc bump` need not touch
  receipt structure for unchanged tiers. This is the receipts-equivalent of an index, and it's
  ~20 lines, not a subsystem.
- **vp-9 · identity discipline is the hard part of stable receipts.** Adapton: incremental reuse
  hinges on *first-class names* for cached computations; naming by global allocation counter is
  flagged "never appropriate for the archivist role — too sensitive to global allocation order"
  [B-adapton-docs-2026]. Dorc's `site N.M` records are exactly such names; +SURE receipts must key
  on stable site identities (survive reordering/edit), not on visit order — otherwise every
  cross-run feature (memoized verdicts under `kSTATE`, `dorc bump` diffing) inherits instability.
  ~SUSPECT this is the single likeliest place a naive r22 implementation goes wrong silently.
- **vp-10 · the overhead envelope.** Collected capture costs, all-in: Soufflé 1.27×t/1.45×m;
  ProvSQL 2–3×; Titian <1.3×; RAMP-style 2.3×; PASS <20%; TaintDroid ~14%; Flume 34–43%;
  Newt 86× (the anti-pattern). +SURE "dummy-thicc is sanctioned" is consistent with prior art
  *provided* the receipts live in-engine: the worst well-engineered case is ~3× of an
  engine-local cost that Dorc's wall-clock (network) doesn't even see.

## §2 r-2 evidence — query & explanation UX

- **vp-11 · minimal witness first, expand on demand.** Soufflé: minimal-height proofs explicitly
  to "minimize the number of user interactions"; real Doop proofs exceed height 200, so the UI is
  `setdepth N` fragments with `subproof` placeholders expanded interactively
  [A-souffle-prov-2020][A-souffle-docs-2026]. `nix why-depends`: one *shortest* path through the
  references graph by default; `--all` opt-in [B-nix-whydepends-2026]. Bazel query:
  `somepath` (one) vs `allpaths` (all) as separate verbs [C-bazel-explain-2026]. ProvSQL Studio:
  renders the circuit behind one result UUID with *frontier expansion* + an inspector
  [B-provsql-github-2026]. Pernosco: "just click on the NULL value" — each click is one backward
  dataflow hop to the origin write; navigation, not graph rendering [B-pernosco-demo-2021].
  +SURE the shared grammar: default = one minimal witness; structure = expandable frontier;
  never = the whole DAG.
- **vp-12 · evidence per edge, in the user's own artifact.** nix `--precise` prints, for every
  edge on the path, the file fragment (`lib/thunderbird/libxul.so: …/libX11-1.7.0/lib…`) that
  *caused* the edge — the receipt is shown as bytes the user can grep, not as abstract metadata
  [B-nix-whydepends-2026]. +SURE this is the Dorc rendering rule: a witness step surfaces as the
  sh text (or probe output line) that grounds it, which also serves `kOOB-in-band` taste and the
  off-ramp (explanations readable as shell, even though stored OOB).
- **vp-13 · category + delta, or it gets ignored.** Bazel `--explain` logs a reason category per
  re-run action; the documented failure mode (user thread): "the verbose explanation actually says
  'action command has changed' whenever we see these rebuilds" — category without the *which
  input/what diff* leaves users stuck; `--verbose_explanations` adds the full command but is
  off-by-default and floods [C-bazel-explain-2026]. ninja `-d explain`: "pages & pages of 'these
  things are dirty'… printed before ninja has realised a lot of those things aren't actually
  dirty (thanks to restat)" [C-ninja-explain-2014] — cascade-dumping plus
  emit-before-confirmation. +SURE Dorc's refusal diagnostics must carry (refusal-code, the
  specific origin delta, the witness that failed), and only for root causes (AGENTS.md's
  root-cause-only rule is the same lesson re-learned).
- **vp-14 · suppression heuristics are the shipped artifact.** Clang SA GSoC 2019: adding
  control-dependency condition-tracking to bug paths raised note counts from ~30→~70, worst
  ~10→~170 — "intolerable"; the project's deliverable became the heuristics (don't track assert
  conditions; only mention events in nested calls since the user can read the local function;
  prune notes about trivial functions) [B-szelethus-gsoc-2019]. The mechanism beneath: diagnostics
  are *constructed after analysis* by visitors re-walking the stored bug path (the exploded-graph
  receipts), so adding/removing explanation detail never touches the analysis — the same
  build-forward/query-backward split Dorc plans. Salsa's cycle UX adds a small twist worth
  stealing: the panic names the *cycle head*, and fixed-point recovery can embed a `salsa::Id`
  cycle-marker in the initial value so recovery code detects self-originated values — a receipt
  consumed by the *engine itself* for correctness, not by the user [B-salsa-cycles-2026].
- **vp-15 · "why not" doesn't automate; Dorc has an out.** Soufflé's `explainnegation` requires
  the user to pick the rule and bind free variables ("not technically feasible to automatically
  generate explanations for non-existence") [A-souffle-docs-2026]. For Dorc, "why was this NOT
  elided" is not a non-existence query: the license check that failed is a positive event with a
  site, a refusal-code, and the partial witness it rejected — the round-21 named-refusal
  diagnostics + per-site why-not attribution already have the right shape. +SURE keep refusals
  first-class events; never reconstruct them from absence.

## §3 r-3 evidence — the minimal formalism for an elision-justification engine

Definitions (standard; [B-cheney-whw-2009], unified by the semiring framework
[B-green-tannen-2007] as read through [A-provsql-2025]/[A-souffle-prov-2020]):
*lineage* = flat set of contributing source items; *why-provenance* = the witness basis (a set of
witnesses; each witness = a sufficient subset of sources); *minimal why* = only minimal
witnesses; *how-provenance* = the semiring polynomial (× joint use, + alternatives, exponents
multiplicity) — most general, with why/lineage as homomorphic images; *where-provenance* = which
source *location* a value was copied from (cell-level; annotation-propagation).

Mapping each to the concrete Dorc question it answers (+SURE on the mapping, the questions are
from 21G/219/21B):

- **vp-16 · "who contributed to this ⊤?" → lineage.** A flat origin set (k-capped, vp-6) on the
  abstract value answers the dashboard/⊤-blame question completely. Join = set-union = the
  lattice ⊔; monotone, cheap, interned. Nothing finer is needed *on values*.
- **vp-17 · "this substitution is licensed because probe-record X + oracle-claim Y" → one stored
  witness (why-provenance's unit).** The license verdict should persist the *specific minimal
  conjunction it was granted on* — not the full witness basis. This is also CHERI-intentionality
  operationalized (vp-26): the elision may use only its cited witness.
- **vp-18 · "does the elision survive if claim Y is retracted/stale?" → the alternatives
  structure, OR re-derivation.** Precise retraction-propagation needs all witnesses
  (PosBool[X]-shape); flat lineage over-approximates the support, so member-changed ⇒
  license-unknown. That over-invalidation is *safe* (costs a probe/re-analysis, never a wrong
  elision — the `kFAIL-perform` direction) and Soufflé-style re-derivation makes the precise
  answer recoverable on demand. +SURE ship lineage + single-witness; treat retraction by
  recompute; do NOT store DNF on every value. ~SUSPECT licenses are rare enough (per-site, not
  per-value) that even storing full witness sets *at licenses only* stays trivial — a later knob.
- **vp-19 · multiplicity (full ℕ[X] how-provenance) answers no Dorc question.** "How many
  derivations" has no consumer in probe/apply/bump/reconcile. The systems that stored rich
  polynomials eagerly are the dead ones (§4); the one that ships (ProvSQL) does it via shared
  circuits + lazy specialization, for *database* questions (probability, Shapley) Dorc doesn't
  ask. Avoid; don't even reserve.
- **vp-20 · "which bytes did this surfaced sh text copy from?" → where-provenance = the existing
  Span plane.** Dorc already carries it (Span/SpanEdit `original`; 111's loc-* list). Keep where
  (locator/copy plane) and why/lineage (justification plane) as *separate planes* on the same
  nodes — the survey keeps them distinct because they propagate differently (where follows
  copying; why follows logical dependence); fusing them recreates the coarsest-tier precision
  loss 111 warned about for source-map composition.
- **vp-21 · exclusion-check (the AGENTS.md four-by-two).** Reverse direction: forward-lineage
  ("what does this oracle-claim license downstream?") is the same indexes traversed forward —
  Smoke/Titian maintain both directions; build both query directions over one arena (111 di-3
  already requires this). Other phase: probe-safety vouching ("this probe is non-mutative because
  oracle O's claim") is the same witness shape as apply-elision licenses — one license record
  type, phase-keyed. Other user: the admin consumes rendered minimal witnesses; the engineer
  consumes expandable proof fragments — two depths of one DAG, not two systems. Other
  reliability: unreliable oracles are exactly the retraction consumer (vp-18) — deferred to
  re-derivation, *named* here so it doesn't sneak back as "we need DNF everywhere."

## §4 r-4 evidence — how provenance systems drown

- **vp-22 · the graveyard, and the packaging predictor.** ProvSQL's related-work section is a
  burial register: Trio "tied to specific and obsolete versions of PostgreSQL (8.2 or 8.3)"; Perm
  and Orion unmaintained *forks* of PG 8.3/8.4; ORCHESTRA "cannot be compiled because some of its
  dependencies are on servers that have disappeared"; MystiQ requires Java 5 [A-provsql-2025].
  Survivors: ProvSQL (extension), GProM (middleware), Titian (library inside Spark). +SURE the
  predictor is *coupling mode*, not formalism or overhead: provenance implemented as a fork of /
  sidecar to the host dies with its pin; implemented as a plane inside the host's own abstractions
  it survives. Dorc corollary: the receipts plane is a layer of the analyzer crates (dac-B), with
  no own store, no own query engine, no own serialization regime beyond the existing OOB lane.
- **vp-23 · capture-without-consumer.** PASS: observed-provenance noise (a Python script's output
  is linked to every module the interpreter loaded), the n-by-m black-box problem (n inputs × m
  outputs all falsely related), cycles needing node-merge workarounds, and "improving provenance
  pruning to manage storage growth" still listed as *future work* — i.e. collect-everything ran
  ahead of any bounded query workload [B-pass-2006][B-carata-primer-2014]. Scientific-workflow
  provenance shows the same capture≫query asymmetry (the primer's query section: exploratory
  graph-browsing vs directed query languages, neither widely exercised). +SURE the inoculation:
  design receipts against the closed consumer list (⊤-blame; license-witness; refusal-delta;
  dashboard why-not; erasability gate) and refuse speculative capture.
- **vp-24 · unbounded precision dies on real input.** TSE'21 controlled comparison: DroidSafe
  (highest accuracy on benchmarks) "failed to handle large real-world applications" outright;
  FlowDroid survives by being cheaper and k-bounded but misses ICC/reflection; none reliable on
  real Play-store apps [B-tse-taint-compare-2021]. The shipping systems (TaintDroid, FlowDroid)
  are the ones with hard caps (vp-6). For receipts: the cap belongs in the *representation*
  (bounded origin lists, truncation markers), so worst-case scripts degrade receipts gracefully
  instead of degrading the engine.
- **vp-25 · channel-credibility failures.** Clang's note-explosion (vp-14), ninja's
  cascade-dumps, Bazel's category-without-delta (vp-13) — and round-11's SQL-Server
  plan-warnings finding — converge: +SURE an explanation channel that over-produces or
  under-specifies gets *ignored*, which is indistinguishable from not having built it. The
  warning-fatigue clause already in AGENTS.md is the same budget; receipts rendering must be
  root-cause-only with suppression rules as first-class, tested code.
- (Cross-ref, not re-covered: 111 already banked source-map composition precision-loss and OTel
  import-the-idea-not-the-machinery; both consistent with the above.)

## §5 r-5 evidence — the one-way rule (refuse/explain, never permit)

- **vp-26 · IFC names it.** Sabelfeld–Sands: declassification dimensions (what/who/where/when) +
  four prudent principles — semantic consistency, conservativity (no-declassification programs
  get plain noninterference), *monotonicity of release* (adding declassifications cannot make a
  secure program insecure), non-occlusion (a release must not mask other leaks)
  [B-sabelfeld-sands-2009]. The "who" dimension's *robust declassification* (Zdancewic–Myers
  lineage, read here via the survey — B-grade access) is the closest formal statement of Dorc's
  rule: **untrusted influence must not control what gets released.** Dorc translation: inferred
  receipts are untrusted influence; the only permit-points are oracle-claims — explicit, owned,
  spelled in sh by the engineer, auditable. Conservativity translates too: a script with no
  oracle-claims gets the plain never-elide posture, exactly current behavior.
- **vp-27 · permits are capabilities, never inferences.** Flume: a process may declassify/endorse
  only if it *holds the capability* for that tag (t⁻/t⁺); creating a tag grants its capabilities;
  everything else flows monotonically [B-flume-2007]. Livshits–Chong: developer-placed
  sanitizers are error-prone at scale, motivating few/explicit/automatically-checked
  permit-points [B-livshits-chong-2013]. +SURE the design translation: the License type is
  constructible *only* from an oracle-claim value (capability-style), never from receipt
  inspection; 21G §2's make-bad-states-unrepresentable posture already aims here.
- **vp-28 · the capability contrast (when provenance MAY permit).** CHERI: "provenance validity
  ensures capabilities can be used only if derived via valid transformations of valid
  capabilities"; "monotonicity requires any capability derived from another cannot exceed the
  permissions and bounds of" its parent; *intentionality* prevents confused-deputy — the kernel
  uses only the capability the caller passed, though it holds greater authority
  [A-cheri-rules-2020]. CHERI's provenance is load-bearing authority and is sound only because
  every derivation step is hardware-enforced and machine-check-proved. +SURE the boundary rule
  this names for Dorc: provenance-as-authority requires a *sound* derivation chain; Dorc is
  capped at soundiness (README), therefore receipts must never be load-bearing for permission —
  they may only narrow (refuse) or explain. Intentionality survives the translation as license
  discipline: an elision executes against its cited witness only (vp-17), never against ambient
  analyzer knowledge ("we know better" is the confused deputy).
- **vp-29 · Dorc already has the internal weld; extend it to types + CI.** `an-orientation-
  coercion`: Must→May legal, May→Must a compile error (ANALYZER-NEEDS §A). Receipts ride the
  may-side. Two mechanical enforcements fall out: (i) no function from `ProvId`/origin-sets to
  `License` (type-level, the 21G layer-2(A) wrapper inverted); (ii) the **erasability gate** —
  strip the receipts plane, re-run the analyzer, assert verdict-identical output (the inverse of
  monotonicity-of-release; also Racket's tracking-edge-carries-no-execution-dependency from 111,
  now as a test). ~SUSPECT erasability also pins down a subtle bug-class early: any code path
  where a receipt *accidentally* becomes load-bearing (e.g. join order influenced by origin-set
  size) breaks determinism — Salsa's execution-order-independent cycle fallbacks
  [B-salsa-cycles-2026] are the same discipline.

## §6 Design consequences for Dorc (r22 seeding)

**Representation candidate** (+SURE on shape, ~SUSPECT on field details — measure first, vp-2):

- One append-only per-run arena of origin nodes; per-value receipt = one `ProvId(u32)`
  (niche-friendly for `Option`). Node ≈ `(kind: OriginKind, site: SiteId/Span,
  parents: bounded list of ProvId)`. Hash-cons nodes (the memory lever: fixpoint iteration
  re-derives identical origins constantly; sharing is the whole game — ProvSQL circuits, rustc
  hygiene chains). Resolve to human text lazily, controller-side (rustc Span→SourceMap; 111).
- `ValueOf::Top` gains a cause: `Top(ProvId)` — the 219 fork-cmdsub-top-cause lean, generalized.
  ⊤-absorption: once ⊤, joining stops accumulating (keep first-cause or a k-capped Join node) —
  the Clang RecoveryExpr cascade-suppression already welded, applied to receipts.
- Join nodes k-capped with an explicit `truncated` marker rendered as "…and N more" (vp-6).
  Licenses are exempt from the cap and store their full granted witness (vp-17/vp-18): values
  are many and capped; licenses are few and exact. Two-tier budget.
- A per-tier epoch vector over origin tiers (oracle-claim / book-source / probe-result /
  runtime), durability-style, for O(1) "anything changed in tier X?" (vp-8).
  <!-- /* demoted 2026-06-11 (round-22, ru-13 + RV3 find-1): rerun-to-fixpoint is the
  likely change-handling path, which removes this item's motivating use-case. Do NOT
  build load-bearing; at most a dashboard/why hint. 22A §1 arch-1 deliberately omits
  it. */ -->
- Receipts key on stable site identities (`site N.M`), never on visit order (vp-9).
- It is a *plane inside the analyzer crates* — no sidecar store, no own query engine; queries are
  Rust APIs the CLI/dashboard call; rendering passes through the DiagCode catalog with ProvIds as
  structured params (21G rq-1 compatible) (vp-5/vp-22).

**The memory knob.** This plane is `kFACTS` in miniature (KNOBS already names provenance in the
kFACTS-materialized pole): eager materialized arena ↔ Soufflé-style re-derivation (store only
(transfer-id, step)-grade annotations; reconstruct parents by guided re-query). Recommendation:
**eager arena + hash-consing + k-caps + ⊤-absorption now**; reserve the re-derivation door by
keeping `OriginKind` a closed enum and the query API opaque (`kLOCKIN-reserve`, cheap). Prior-art
envelope says in-engine eager lands ≤1.5–3× engine memory (vp-10); at Dorc scale (10³–10⁴-line
scripts, ~10⁴–10⁵ abstract values, 16–24 B nodes) that is single-digit MBs — -GUESS two orders
below where the knob would bite; the knob exists for the pathological-corpus cell, not the mean.

**What r22 builds first** (ordered):
1. `ProvId` arena + `Top(cause)` reshape, threaded through `ValueOf` and `Carrier` emission
   sites (the queued q-2/y-1 slice already leans this; rq-1..rq-3 stay as specified).
2. The **erasability gate** as a unit/CI test from the first commit (vp-29) — cheapest while the
   plane is small, load-bearing forever (same trick as the rq-2 catalog-completeness embryo).
3. One consumer end-to-end before widening capture (vp-23): the dashboard's per-site why-not
   attribution rendered as minimal-witness + expandable fragments + per-step sh-text evidence
   (vp-11/vp-12), with suppression rules written as code (vp-14).

**Avoid** (each tagged to its §4 predictor): external/persisted receipt stores (vp-5; `kSTATE`
unsettled — receipts stay per-run until kSTATE resolves); full how-polynomials or per-value DNF
(vp-19/vp-18); unbounded origin sets (vp-24); rendering receipts eagerly into strings (vp-1/
vp-14 — store structure, render late); explanation output without suppression/root-cause-only
rules (vp-25); any API where receipts can mint permission (vp-26..29); speculative capture for
consumers that don't exist yet (vp-23).

## §7 Graded sources

Full/substantially read (load-bearing claims rest on these):
- [A-souffle-prov-2020] · Zhao, Subotić, Scholz — "Debugging Large-scale Datalog: A Scalable Provenance Evaluation Strategy" (TOPLAS) · ar5iv.labs.arxiv.org/html/1907.05045 · A · the closest engine-shape match; (rule,height) annotations, lazy minimal proofs, 1.27×/1.45×.
- [A-souffle-docs-2026] · Soufflé manual, "Provenance" · souffle-lang.github.io/provenance · A · the shipped explain/explore UX: setdepth, subproof, explainnegation; annotation internals.
- [A-provsql-2025] · Sen, Maniu, Senellart — "ProvSQL: A General System…" · arxiv.org/abs/2504.12058 · A · circuits, mmap storage, lazy semiring views, 2–3×; the §2 graveyard; EXCEPT-ALL intractability.
- [A-smoke-2018] · Psallidas, Wu — "Smoke: Fine-grained Lineage at Interactive Speed" (VLDB'18) · ar5iv.labs.arxiv.org/html/1801.07237 · A · capture-in-hot-loop economics; P1–P4; workload-aware pruning.
- [A-titian-2016] · Interlandi et al. — "Titian: Data Provenance Support in Spark" (VLDB'16) · pmc.ncbi.nlm.nih.gov/articles/PMC4697929/ · A · Newt 86× external-store failure; in-engine <30%; query-in-host-language.
- [A-rustc-spanenc-2026] · rust-lang/rust `compiler/rustc_span/src/span_encoding.rs` doc comment · github.com/rust-lang/rust · A · 8-byte Span, 4 formats, 4-byte-was-slower, measured field widths, no-cliff rule.
- [A-rustc-hygiene-2026] · rustc-dev-guide, "Macro expansion" · rustc-dev-guide.rust-lang.org/macro-expansion.html · A · SyntaxContext chains in a global side-table; 3 hierarchies over one ExpnId arena; Span=location+ctxt.
- [A-cheri-rules-2020] · CHERI C/C++ Programming Guide, "Architectural rules for capability use" · ctsrd-cheri.github.io/cheri-c-programming/background/architectural-rules.html · A · provenance validity, monotonicity, intentionality; machine-checked basis.
- [B-ra-durability-2023] · matklad — "Durable Incrementality" (rust-analyzer blog) · rust-analyzer.github.io/blog/2023/07/24/durable-incrementality.html · B · durability version-vector; min-of-inputs; skip-subgraph revalidation.
- [B-salsa-cycles-2026] · Salsa book, "Cycle handling" · salsa-rs.github.io/salsa/cycles.html · B · cycle-head naming; fixpoint recovery; embedded cycle-marker Ids; order-independent fallbacks.
- [B-carata-primer-2014] · Carata, …, Seltzer, Hopper — "A Primer on Provenance" (ACM Queue) · queue.acm.org/detail.cfm?id=2602651 · B · granularity/layering/noise; n-by-m; disclosed vs observed; overhead tradeoffs. (Read ~⅔ direct + summarizer; site 403'd the tail.)
- [B-szelethus-gsoc-2019] · Umann — GSoC'19 final report, Clang SA bug-report enhancement · szelethus.github.io/gsoc2019/ · B · visitor-based path reconstruction; 10→170 note explosion; the suppression heuristics.
- [B-nix-whydepends-2026] · Nix manual — `nix why-depends` · nix.dev/manual/nix/2.26/command-ref/new-cli/nix3-why-depends · B · shortest-path default; --all; --precise per-edge file-fragment evidence.
- [B-pernosco-demo-2021] · Huey (O'Callahan's blog) — "Demoing the Pernosco omniscient debugger" · robert.ocallahan.org/2021/04/demoing-pernosco-omniscient-debugger.html · B · click-to-origin dataflow navigation as the consumption model.
- [B-adapton-docs-2026] · docs.rs/adapton crate docs · docs.rs/adapton · B · DCG; nominal memoization; global-counter naming "never appropriate" for cached roles.
- [B-livshits-chong-2013] · Livshits, Chong — "Towards Fully Automatic Placement of Security Sanitizers and Declassifiers" (POPL'13, abstract page) · people.seas.harvard.edu/~chong/abstracts/LivshitsC13.html · B · sanitizer misplacement risk; static-where-possible placement.

Read via summarizer (key claims corroborated by snippets/secondaries; graded for my access, not the venue):
- [B-taintdroid-2010] · Enck et al. — TaintDroid (OSDI'10) · usenix.org/legacy/event/osdi10/tech/full_papers/Enck.pdf · B · 32-bit tag vector; per-array/per-message coarsening; 14%; explicit flows only.
- [B-sabelfeld-sands-2009] · Sabelfeld, Sands — "Declassification: Dimensions and Principles" (JCS) · cse.chalmers.se/~andrei/sabelfeld-sands-jcs07.pdf · B · what/who/where/when; the four prudent principles; robust declassification (who).
- [B-cheney-whw-2009] · Cheney, Chiticariu, Tan — "Provenance in Databases: Why, How, and Where" (FnT-DB) · homepages.inf.ed.ac.uk/jcheney/publications/provdbsurvey.pdf · B · the three notions; witness basis; homomorphism hierarchy; eager-vs-lazy.
- [B-tse-taint-compare-2021] · Luo et al. — "Analyzing Android Taint Analysis Tools…" (TSE) · people.ece.ubc.ca/mjulia/publications/Analyzing_Android_Taint_Analysis_Tools_TSE_2021.pdf · B · DroidSafe fails real apps; FlowDroid k-bounded survival; none reliable real-world.
- [B-pass-2006] · Muniswamy-Reddy et al. — "Provenance-Aware Storage Systems" (USENIX ATC'06) · usenix.org/event/usenix06/tech/full_papers/muniswamy-reddy/muniswamy-reddy.pdf · B · <20% overhead; cycle node-merging; pruning as open problem.
- [B-flume-2007] · Krohn et al. — "Information Flow Control for Standard OS Abstractions" (SOSP'07) · pdos.csail.mit.edu/papers/flume-sosp07.pdf · B · tags+capabilities; endorsement requires held capability; 34–43%.

Search-surveyed (context / single claims; not load-bearing alone):
- [B-green-tannen-2007] · Green, Karvounarakis, Tannen — "Provenance Semirings" (PODS'07) · (via the two A-grade reads above; not directly read) · B · ℕ[X] generality; homomorphic images.
- [B-provsql-github-2026] · PierreSenellart/provsql README (Studio: circuit frontier-expansion, where-mode hover) · github.com/PierreSenellart/provsql · B.
- [C-bazel-explain-2026] · Bazel user manual (--explain/--verbose_explanations) + bazel-discuss "Help debugging spurious rebuilds" · bazel.build/docs/user-manual · C · category-without-delta failure mode.
- [C-ninja-explain-2014] · ninja-build list "Improving Ninja's -d explain output" (+ issue #2599) · groups.google.com/g/ninja-build/c/63PdL6xYS7Y · C · cascade-dump + pre-restat noise.
- [C-flowdroid-2014] · Arzt et al. — FlowDroid (PLDI'14) · bodden.de/pubs/far+14flowdroid.pdf · C (snippets) · k-bounded access paths.
- [C-phosphor-2014] · Bell, Kaiser — Phosphor (OOPSLA'14) · github.com/gmu-swe/phosphor · C (snippets) · per-value JVM tags portable at interpreter level.
- [C-trio-status-2009] · Stanford Trio project page + Trio-One paper · infolab.stanford.edu/trio/ · C · layered-on-conventional-DBMS design; activity ends ~2008-09 (status evidence for vp-22).
- [C-salsa-overview-2026] · Salsa book overview · salsa-rs.github.io/salsa/overview.html · C.
- [C-pernosco-hn-2020] · HN thread on Pernosco availability/pricing · news.ycombinator.com/item?id=25042827 · C · precompute-everything positioning.
- [C-widom-trio-talk-2007] · Widom — Trio overview talk · cs.stanford.edu/people/widom/trio-talk.pdf · C (not read; existence/status).
- [C-zhao-thesis-2021] · Zhao — PhD thesis (provenance/incremental Datalog) · ses.library.usyd.edu.au · C (snippet corroboration of height annotations).
- [C-tplp-rollback-2025] · "Provenance Guided Rollback Suggestions" (TPLP) · cambridge.org · C · provenance consumed to *suggest fixes* — a possible far-future consumer, noted only.
- [D-instatunnel-2025] · "DVR for Developers" substack overview of rr/Pernosco · instatunnel.substack.com · D · context only.

Unreachable / noted for the human: queue.acm.org 403'd mid-article and cacm.acm.org 403'd
entirely (Cloudflare-ish); the primer's tail (overhead-numbers section, SPADEv2 figures) was
taken via summarizer only. Zdancewic–Myers "Robust declassification" itself was not fetched
(read through the Sabelfeld–Sands survey); if r-5 gets welded into a design doc, that primary
deserves a direct read.
