# Cross-cutting — the cost model & tuning knobs (the user's `.perf-info` question)

> ⟢ 2026-06 — "how do oracles communicate cost without a YAML metadata blob" is the knob `kOOB` (minimize out-of-band; `Q-INFER` measures how much is inferable from sh-structure); the hoist-vs-guard dial is `kFLATTEN`, probe-vs-just-run is `kPROBING`. Grep the slug for current handling.

This is the newly-surfaced complication (note 70): the probe optimizer's hoist-and-batch vs keep-under-guard choice needs a **per-check cost signal**, but the UX constraint is hard — Dorc is *not* a YAML metadata blob and must not burden the engineer with mandatory annotation (that breaks the deployer gradient, AGENTS contract matrix). The good news: this exact problem is **solved prior art** in two mature fields. Sources: DB query-optimization literature (arXiv "Adaptive Cost Model for Query Optimization" 2409.17136, A; Oracle optimizer docs, B; query-optimization survey, B), PGO/AutoFDO (Google research, A), effect systems (Lucassen–Gifford, on disk, A).

## The two prior-art fields, and what each says

### DB query optimizers — the direct structural analog
A query optimizer does *precisely* Dorc's job: take a declarative-ish spec, decide an execution plan (which operators, what order, what to push down/batch), driven by a **cost model**.
- **Cost model = formulas estimating plan latency** (CPU + I/O) from statistics. Dorc's analog: estimate a check/check-tree's cost (local-stat ≪ local-exec ≪ network ≪ remote-rpc) to decide batch-vs-guard.
- **Cardinality estimation is the hard part and dominates quality** (survey, A): "cost-based optimization quality depends much on cardinality estimation quality." The estimate of *how much* (rows, here: how-expensive / how-often-reached) is harder and matters more than the per-operator cost formula. → **Dorc's hard problem isn't "what does a `curl` cost" (knowable); it's "how often is this subtree reached / how many hosts hit it / does the guard usually elide it" — a cardinality-estimation analog.** Get this from telemetry (below), not from annotation.
- **Adaptive query execution / statistics feedback** (Oracle, arXiv 2409.17136, A): "at the end of the first execution… the optimizer uses information gathered during execution to determine whether re-optimization has a cost benefit… statistics feedback automatically improves plans for repeated queries with cardinality misestimates." An **Adaptive Cost Model "dynamically optimizes plan cost parameters at runtime by continuously monitoring execution statistics."** → **This is the model for Dorc: cold-start with a coarse static estimate, then re-plan from observed probe costs across runs.** Ops is *repeated execution against an evolving fleet* — the ideal setting for feedback-driven re-planning (you run `dorc` against the same fleet over and over).

### PGO / AutoFDO — how to get the profile without burdening anyone
- **AutoFDO** (Google, A): **sampling-based profiling on production machines, no instrumentation, no source annotation** — collect execution statistics from normal runs, feed back to guide optimization. **85% of instrumented-PGO's gains despite sampling imprecision**; "all optimizations benefit… less reliant on heuristics." Profiles say which code is hot/cold.
- → **Dorc gets this for free**: the **realtime-output requirement (AGENTS hard requirement) means every probe already measures and streams its own timing.** That telemetry *is* the profile. No `.perf-info`, no annotation — the probe self-profiles, and the optimizer learns check costs from accumulated run history. This is the strongest answer to the user's UX question: **profile-guided cost, harvested from the realtime-output we're already required to produce.**

## The recommended cost-model design (synthesis; flagged as a design proposal, not settled)
A **three-tier cost signal**, conservative-by-default, annotation-optional:
1. **Default (zero buy-in): assume expensive → keep-under-guard.** Absent any information, a check is treated as costly, so the optimizer **keeps it under its guard** (never flattens a possibly-expensive thing into the even-probe bag). This is the **perf-safe default** — it can only cost *missed batching* (a perf loss, recoverable by feedback), never a thundering-herd surprise. Mirrors "un-annotated ⇒ conservative."
2. **Optional static cost-class hint (cheap engineer buy-in): a one-line coarse class**, not a separate `.perf-info` binary (that's over-engineering). Something like an oracle declaring `cost: local-stat | local-exec | network | remote-rpc` (4 classes, not a number). Cheap to author, optional, and it *only* improves cold-start; absent ⇒ tier-1 default. A *separate callable* `.perf-info` is rejected: it doubles the artifact count and the information is coarse enough for a declaration. (Open: is even 4 classes too much ceremony? Could the class be *inferred* from the oracle's own command — a `curl`/`ssh`/`nc` in the check body ⇒ `network` — so the engineer annotates *nothing*? SUSPECT yes, partially: static inspection of the check body gives a free first guess. This is the most elegant option and worth the spike.)
3. **Profile-guided (the winner at steady state): learn from probe telemetry.** Accumulate per-check timing from realtime-output across runs; re-plan batch-vs-guard from observed cost + observed reach-frequency (the cardinality analog). Cold-start uses tier 1/2; converges to accurate as the fleet is run repeatedly. Adaptive, zero annotation, matches DB statistics-feedback + AutoFDO.

**Plus a structural rule independent of cost** (note 73 / RCPSP): a guard that **gates a large subtree** is worth keeping *regardless* of its own cost (it elides much expected work) — an expected-work / critical-path calculation, not a flat cost threshold. And the **shared-resource dimension** (note 72): cost is not scalar — a check that hits a *shared backend* (mirror/registry/remote controller) has a **fleet-multiplied** cost (`task_duration × num_targets` if serialized) and a contention cost, so the cost model needs a `touches-shared-resource: <id>` tag (derivable from the fact-domain partition) to drive throttling, not just batch-vs-guard.

**Soundness rider** (carried from the probe-model discussion): keeping a guard to gate an expensive check is only valid if the *guard itself* is read-only **and** evaluable against initial host state (no dependence on an upstream unapplied mutation). The cost model must not retain an order-dependent guard that then mis-gates — purity + intra-run-independence are preconditions on *every* retained guard.

## The tuning knobs and where they live (the user's "perf is sensitive to a few tuning parameters")
The knobs cluster at the **intersections of the three dualities** (AGENTS contract matrix) — which is *why* they resisted a single setting:

| Knob | Phase | Set by / sensitive to |
|---|---|---|
| batch-vs-guard threshold | probe-construction | cost model (above); the core optimizer dial |
| probe fan-out width | probing | controller ceiling (async, fan-out tree); target `MaxStartups`; **mode** (`update`=narrow/deep, `reconcile`=wide) |
| connection reuse / multiplex | probing | `MaxSessions`; ControlMaster; same-host probe→apply→re-probe |
| apply parallelism (semaphore) | mutation | derived DAG width (note 73); **Graham anomaly** — not monotonic |
| rolling batch size / `serial` | mutation | blast-radius (audience: deployer "fire now" vs engineer "careful"); derived |
| resource throttle group | mutation | derived fact-domain contention (RCPSP capacity) |
| readiness-gate strictness | mutation | re-probe between batches; mode |
| context-sensitivity (k) | analysis | **EXPTIME cliff (note 71)** — keep 0; selective only |

**The two objectives that reset the defaults** (the duality intersections that matter most):
- **Mode**: `dorc update` (goal-framed) → small declared scope, **latency-bound**, few hosts deep, optimize time-to-first-action. `dorc reconcile` (state-framed) → whole fleet, **throughput-bound**, wide, optimize total makespan. *Different objective functions → different optimizer defaults, not just different scope.*
- **Audience/situation**: deployer "server's on fire, NOW" → minimize latency-to-first-mutation, accept imprecision (more probes). Engineer polishing a role → maximize precision/insight, accept slower. The cost model and these knobs are **one surface**; expose a coarse "urgency/thoroughness" intent, derive the rest.

## Observing effects cheaply — ptrace is a trap, eBPF is the answer (calibration + any effect-derivation)
Two places Dorc may want to *observe what a command actually did* (not analyze, but watch reality): (1) the **calibration harness** (differential testing: run the mutate on a container fixture, observe the state delta to validate the analysis prediction — static-analysis-engine §5); (2) any future **oracle-derivation / effect-discovery** à la Dozer (which traces syscalls to learn a command's effect). The measured cost gap is large (grade A/B):
- **ptrace/strace: 2–10× slowdown** (stops the process at every syscall, two context-switches per call; a `geteuid` microbench ran **102× slower**). Fine for offline one-shot fixture-recording; **fatal if ever on a hot/production path.**
- **eBPF/bpftrace: <1–2% overhead** at typical syscall rates (in-kernel, system-wide, no per-call stop). The modern answer; Dozer used strace (2022, offline) but a Dorc-era effect-observer should be **eBPF-based**.
- → **"Wish I'd known": if Dorc ever observes effects at syscall altitude (calibration deltas, oracle-derivation, drift-witnessing), use eBPF, not ptrace** — and keep it *offline/fixture-side* (the calibration container), never on the live probe path. The cheap on-probe self-profiling (timing, exit codes — note above) is plenty for the cost model; syscall-level observation is a *calibration/derivation* tool, not a per-run one. This also bounds the Dozer-style "derive the effect-class from the check body": offline eBPF tracing on fixtures can *suggest* an oracle's effect-class/cost-class, feeding the tier-2 hint without engineer annotation.

## Footgun list (cost-model "wish I'd known")
1. **Mandatory cost annotation** → breaks the deployer gradient (the whole product). Default-conservative + optional-hint + profile-guided; never required.
2. **Scalar cost** → misses the **fleet-multiplied** and **shared-resource-contention** costs; a "cheap" local check that hits a shared mirror is expensive ×N. Cost is a vector: (local-cost, reach-frequency, shared-resource-id).
3. **Cost without feedback** → cardinality misestimates persist (DB lesson: estimation quality dominates). Harvest probe telemetry (free from realtime-output); re-plan across runs.
4. **Optimizing for one objective** → `update` and `reconcile` have *different* objective functions (latency vs makespan); a single tuned default is wrong for one of them.
5. **A separate `.perf-info` artifact** → ceremony; prefer a one-line class, or infer the class from the check body (static inspection: a `curl` in the check ⇒ `network`). Spike the inference.
6. **Forgetting the guard-purity precondition** → the cost model retains an order-dependent guard to save cost, and it mis-gates. Every retained guard must be read-only + initial-state-evaluable.
