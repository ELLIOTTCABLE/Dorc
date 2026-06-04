# plans/111 — error / provenance / reporting — round-11 conclusion

Round 11, 2026-06-03. **The round's conclusion** (supersedes the interim "synthesis + research plan"
framing this file opened with — all five facets are now gathered + graded). Raw material:
`notes/110` (domain-1 + bridges A/B, f1–f47), `notes/112` (domain-3 ops, f48–f52),
`notes/113` (domain-2 query-planner, f53–f65). 31 sources, all `top-level-agent`-graded on a full
read. The highest-value bits are already consolidated by the human into AGENTS.md (the perf
two-angles bullet; the "error-recovery / fail-fast" terminology foot-gun).

## 0. Conclusion — the answer, up front

Dorc's error / provenance / reporting is **one cross-cutting spine, not a pile of subsystems**: a
**PROV-shaped derivation DAG** [B-prov-primer-2013] of *located-nodes + typed-edges*, in which —
- every stage yields **`(best-effort result × accumulated diagnostics)`** and never throws [A-bour-merlin-2018];
- every node carries a **compact origin-handle resolved lazily** against a controller-side map (rustc `Span`→`SourceMap`) [B-rustc-diagnostics-2024];
- the graph is **built-forward, queried-backward** [B-prov-aq-2013].

+SURE the single most important result of the round: **across five independent prior-art domains** —
parsing/recovery, repair, static-transform provenance, distributed provenance, ops orchestration, and
RDBMS query-planners — *the same data-shape recurs*. Nothing here demands a new data-structure; the
work is to *adopt* the shape and weld it through every component.

Four findings shape Dorc's version — three that make it *exceed* the single-machine prior-art, one
that makes it *easier* than first feared:

- **dac-A — N-tier and per-host-forking.** The parser/compiler prior-art tops out at one tier-hop on
  one machine (GCC's macro dual-location is the best, still 2 points / 1 hop) [B-gcc-libcpp-location-2024];
  Dorc composes `source → oracle-lift → per-host-probe → runtime → surface`, and the *per-host*
  transform **forks** one source line into N host-specific images. The static half is a composed
  source-map / `inlinedAt` chain [B-mozilla-sourcemap-2024] [B-llvm-dilocation-2024]; the fork has a
  prior-art name — the *discriminator* [B-llvm-dilocation-2024]. +SURE
- **dac-B — the spine is the analyzer's own graph, not a second system.** The `depends-on` edges *are*
  the dependency/taint/CFG output; the error/provenance layer **reads** the analyzer's graph and hangs
  payloads on it. The Nature distributed-PROV "backbone" (one traversal algorithm + attached domain
  content) is the same idea [A-wittner-distributed-prov-2022]. +SURE the highest-leverage crosscutting
  finding: analyzer and error/reporting **must agree the graph types first**, or build two
  incompatible graphs. (AGENTS.md-in-the-analyzer-subdir.)
- **dac-C — it lives controller-side; hosts stay dumb.** The controller compiled the probe, so it
  holds the probe↔source map and reconstructs rich provenance from *delimited host output*; no
  `dorc`-evaluator on the foreign host, `kAGENTLESS-push` preserved. +SURE (the compiled-probe is
  ~O(hosts) tunnels, ~2/host — a leaf is **not** a network op; it runs as `sh` inside the streamed
  batch, so the only per-leaf provenance cost is in-stream marker *bytes*, and the lever is marker
  *compactness*, not opt-in instrumentation.)
- **dac-D — it is NOT distributed consensus.** Single controller, fan-in of N independent SSH results
  (OTel's "gather span" *links* its sources, never votes) [B-otel-spec-overview-2024]; a missing host
  is *just another diagnostic*. The initial "scary distributed merge" framing was wrong
  (human-corrected); the residual hard part is *correlation + async streaming*, and *partial-chain
  traversal* — which the Nature model already solves (fault-tolerant to missing components)
  [A-wittner-distributed-prov-2022]. +SURE

**Load-bearing design implications** (weld in from day 1):
- **di-1 — `kFIDELITY`'s 1:1 leaf↔source seam is necessary but not the whole record** (the real record
  is the N-tier forking chain) — **but it is NOT lock-in-y**: probes are *ephemeral*, so
  crude-locations-now → rich-later costs only "worse errors now," never a stuck artifact. Build
  error/provenance well *early* for **dev-velocity** (it pays the humans *and agents* building Dorc)
  + the shared substrate, **not** for lock-in. (human-corrected)
- **di-2 — ≥3 edge-types, never conflated:** `derived-from` (transform) · `ran-on` (distribution) ·
  `depends-on` (dataflow/surfacing). OTel span-link-vs-parent [B-otel-spec-overview-2024] and Racket's
  `label`-phase (a tracking edge carrying no execution dependency) [A-racket-syntax-model-2024] converge.
- **di-3 — bidirectional resolution, cheaply:** forward (source → host-probes) + backward (host-stderr
  → editable-source + dependent-surface); two sorted indexes over compact relative maps, nothing
  exotic [B-mozilla-sourcemap-2024] [B-prov-aq-2013].
- **locators are a variable-length LIST of typed loci** (`loc-host` / `loc-user-src` / `loc-probe` /
  `loc-surface` / …), not a fixed-slot struct; kept *separate per tier*, **not pre-flattened** —
  composition loses precision to the coarsest tier [B-mozilla-sourcemap-2024]. (human-flagged)
- **the verdict is three-valued (ok / fail / unknown) + staleness-versioned**, distinct from the
  transient diagnostic *stream* (K8s Events-vs-Conditions; `observedGeneration`) [B-k8s-api-conventions-conditions-2024].
- **provenance is OOB *metadata* (fine), not user-config** (which must be `sh`) — the `kOOB` redline is
  config-*form*, clarified in KNOBS; live sub-question is how much renders as in-band readable `sh`
  annotation vs a controller-side blob (lean: OOB; in-band balloons O(lines×tiers×hosts)).
- **diagnostics are security-sensitive in transit** — runtime stderr leaks secrets/paths; the
  controller aggregating every host's stderr *is* the security round's whole-fleet target [→ `plans/102`].
- **suboptimal-warnings must be precise or silent** — SQL Server's plan-warning channel is widely
  *ignored* because false-positive-prone (incomplete/local analysis); over-conservative warnings
  (Dorc's natural regime) train users to tune them out [B-darling-plan-warnings-2022].
- **fail-fast = report-well-then-still-fail; never recover into a broken outcome** (terminology
  foot-gun, now in AGENTS.md).
- **perf is a non-constraint for this machinery** — cost is O(hosts) network + (dominating) slow
  remote commands; controller-local provenance work is lost in the noise (AGENTS.md perf bullet). So
  *always* carry a compact marker; the only lever is marker compactness.

## 1. The recurring data-shape (the per-tier toolkit)
One node-shape and one process recur across parsing, repair, and provenance prior-art (notes/110 f21):
> a node = `(best-effort value | explicit error-node)` + a compact origin-handle + a confidence/trust
> tag + optional grafted-through-transform provenance; the process **always** yields
> `(result × [diagnostics])`, never throws; the message-catalog is decoupled from the engine and kept
> complete by a coverage check.

Mechanisms, each a candidate Dorc primitive:
- **`(result × [diagnostics])`, not `result | error`** [A-bour-merlin-2018]; **fail-fast detection ≠
  fail-fast reporting** (detect early, keep-everything-so-far, fill forward) [A-bour-merlin-2018].
- **one explicit error/poison node-kind, not an `invalid` flag everywhere** [B-clang-recoveryexpr-2019] [B-matklad-resilient-ll-2023];
  **cascade-suppression in the lattice** (⊤/"unknown" type absorbs downstream checks) [B-clang-recoveryexpr-2019].
- **cheap interned handle → lazy resolve** [B-rustc-diagnostics-2024]; **relative loci, derive absolute
  lazily** (DAG-share + edit-reuse; diagnostics on the immutable core) [B-roslyn-redgreen-2024];
  **compact packed location + ad-hoc escape** [B-gcc-libcpp-location-2024].
- **detachable/transplantable metadata** (a transform grafts input-provenance onto output) [A-racket-syntax-model-2024].
- **separate diagnostic catalog + completeness gate** [A-pottier-reachability-2016]; **confidence on
  every *suggestion*** [B-rustc-diagnostics-2024]; **regional ranking to kill cascades** [A-diekmann-cpct-2020] [B-tratt-error-recovery-2020].
- **non-correcting: the error *is* the rejected diff** + per-node change-bits (the `dorc try` prior-art) [A-wagner-history-recovery-1997];
  **one diagnostic spine** (fold lexer errors into parser errors) [B-tratt-error-recovery-2020] [A-bour-merlin-2018].

## 2. Cross-domain validation — what each front added
- **bridge-B (distributed provenance):** PROV = the formal vocabulary (Entity/Activity/Agent +
  `wasDerivedFrom`/`used`/`wasGeneratedBy` + Plan) [B-prov-primer-2013]; OTel independently reached
  di-2 + fork/join ("links not parent for the aggregator") [B-otel-spec-overview-2024]; SLSA
  schematizes the runtime tier and its trusted/untrusted split lands on the oracle boundary
  [B-slsa-provenance-2023]; Trace-Context = the minimal correlation-id [B-w3c-tracecontext-2021];
  Bazel `ActionResult` = the result transport (worker + timestamps + content-digest)
  [B-bazel-reapi-actionresult-2024]; Cramer = import-the-idea-not-the-machinery [B-cramer-otel-critique-2024];
  the Nature model = thin-backbone + missing-component tolerance [A-wittner-distributed-prov-2022].
- **bridge-A (static-transform provenance):** composition is real (`applySourceMap` / `inlinedAt`) and
  **loses precision to the coarsest tier** [B-mozilla-sourcemap-2024] [B-llvm-dilocation-2024]; the
  per-host fork = a *discriminator*; backward resolution = binary-search over a sorted table
  [B-swatinem-dwarf-lines-2023]; corroborates the compact-relative encoding (GCC/Roslyn).
- **domain-3 (ops):** the push/agentless tool *closest to Dorc* — Ansible — is the **worst**
  (errors-as-control-flow, no provenance/aggregation) [B-ansible-error-handling-2026]; the *structured*
  tools independently **converge** on the model — Puppet `Transaction::Report` (correlation-UUIDs +
  per-resource status + dependency-restart + fail-safe + pluggable processors) [B-puppet-transaction-report-2024],
  K8s three-valued staleness-aware Conditions [B-k8s-api-conventions-conditions-2024], Terraform
  DAG-grounded errors [B-terraform-graph-2024]. Dorc's job = bring that model to Ansible's niche.
- **domain-2 (query-planner):** the planner is the closest mature mirror of Dorc's analyzer. EXPLAIN =
  the plan/apply preview (plan + cost-estimates + actuals; estimate-vs-actual = the deopt signal)
  [B-postgres-explain-2024]; **PlanBouquet = probe-to-discover beats estimate-statically, with provable
  bounds** — the strongest external justification for the probe-first architecture [B-haritsa-robust-qp-2020];
  cardinality dominates the cost-formula (keep the formula simple, invest in inputs) [A-leis-query-optimizers-2015].
  **Caveat (human-corrected):** EXPLAIN's *instrumentation implementation* corroborates the data-shape
  (a separate accumulator [B-postgres-instrument-2026], a named aggregate-up, reason-tagged
  diagnostics — the elision-why record), but its *perf optimizations are a **disanalogy*** (PG's
  per-tuple CPU loop vs Dorc's O(hosts) batched-probe; a leaf is not a network op), **not** a validation
  of Dorc's perf knobs; and per-loop *averaging* is a cautionary contrast — Dorc's heterogeneous
  per-host fan-out keeps the breakdown, never averages. The cardinality/robustness *theory* belongs to
  the cost-model/perf round, not this one.

## 3. The frontier — competing options & why-not
- **in-band readable-`sh` annotation vs OOB controller-side blob** (for provenance): leaned **OOB**
  (in-band balloons; even JS "inline" source-maps are a compact blob, not per-line comments). Open:
  how much of a *surfaced diagnostic* renders as readable `sh` for the off-ramp.
- **a new KNOB for di-2/di-3?** Held — not proposed until the ≥3-edge-graph or bidirectional-resolution
  firms into a genuine A-vs-B tension (currently they read as requirements, not dials).
- **the `kFAIL` over/under-approximation ↔ static-vs-dynamic mapping** (Pottier f7) is ~SUSPECT —
  plausible but unverified against the actual probe/apply phase semantics.
- **always-instrument vs faithful-mode opt-in** for per-leaf *timing* actuals: leaning always-on (the
  cost is a host-side clock-read per leaf in a batch — tiny), but `kFIDELITY-faithful` could gate it.

## 4. Quarantine — banked / deferred (for human triage)
- **SQL-Server-warnings deep-dive** (battle-tested suboptimal-warning precision) — human-flagged "later." [TaskList #4]
- **the learn-and-act feedback loop** — Oracle SQL-Plan-Directives (persist-a-correction-note keyed to
  a misestimate), SQL Server auto-plan-correction (detect+act on regression), CE/memory-grant feedback
  ≈ Dorc PGO [`notes/074`]. [TaskList #4]
- **deepen-domain-1** (tree-sitter/Lezer/SwiftSyntax exhaustive placeholders) — lowest value. [TaskList #5]
- **content-hash version-drift spike** — SLSA `ResourceDescriptor.digest` + canonical-serialization-
  before-hashing is direct prior-art [already in `../TODO.md`].
- **cardinality-estimation + robust-QP theory** (Leis/Haritsa) — belongs to the *cost-model/perf*
  round; keep PlanBouquet's "discover-don't-estimate" as the cross-link.

## 5. Status & sources
Fronts: domain-1 ✓ (+deepened) · bridge-B ✓ · bridge-A ✓ · domain-3 ✓ · domain-2 ✓ (EXPLAIN-impl done;
warnings + learn-and-act loop banked). Sources (31, all full-read, `top-level-agent`):
- domain-1 — [A-bour-merlin-2018] [A-pottier-reachability-2016] [A-racket-syntax-model-2024] [A-diekmann-cpct-2020] [A-wagner-history-recovery-1997] [B-matklad-resilient-ll-2023] [B-tratt-error-recovery-2020] [B-rustc-diagnostics-2024] [B-gcc-libcpp-location-2024] [B-clang-recoveryexpr-2019] [B-roslyn-redgreen-2024]
- bridge-B — [B-prov-primer-2013] [B-cramer-otel-critique-2024] [B-otel-spec-overview-2024] [B-slsa-provenance-2023] [B-prov-aq-2013] [B-w3c-tracecontext-2021] [A-wittner-distributed-prov-2022] [B-bazel-reapi-actionresult-2024]
- bridge-A — [B-mozilla-sourcemap-2024] [B-llvm-dilocation-2024] [B-swatinem-dwarf-lines-2023]
- domain-3 — [B-ansible-error-handling-2026] [B-terraform-graph-2024] [B-puppet-transaction-report-2024] [B-k8s-api-conventions-conditions-2024]
- domain-2 — [A-leis-query-optimizers-2015] [B-haritsa-robust-qp-2020] [B-postgres-explain-2024] [B-darling-plan-warnings-2022] [B-postgres-instrument-2026]
