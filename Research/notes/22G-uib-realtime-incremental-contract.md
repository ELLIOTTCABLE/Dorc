# 22G — ui-B build-contract: realtime concurrent incremental re-analysis (multi-probe → multi-plan)

> Round-22 conductor, 2026-06-14. The ui-B build-contract, drafted on the ru-28
> REFRAME (ui-B is NOT a streaming-display feature; it is the realtime concurrent
> incremental re-analysis engine — the UI is the thin final surface) + the 22F-fd5
> finding (ui-B extends `plan`, reuses `advisory_filter`) + the ru-29 CLI-shape
> refinement (the real `plan` internalizes probe-dispatch+slurp → per-host files).
> AI-authored; +SURE/~SUSPECT/-GUESS/--WONDER. THIS IS THE CROSSCHECK INPUT — its
> load-bearing CLAIMS are marked LB-N for the adversarial pass to attack. NOT yet
> built; a builder implements the minimal slice (§6) AFTER the crosscheck lands.

## §0 What ui-B IS (and is NOT)

IS: the engine that runs the analyzer in a REALTIME scenario where probe results
arrive CONCURRENTLY from multiple hosts, and each arriving batch is folded into the
being-built plan to surface NEW elisions — producing per-host plan(s) that
progressively TIGHTEN as probes report. Grounded verbatim in DESIGN ("Dorc's
approach" §3: the plan is "dynamically updated in real-time as the probe-phase
asynchronously proceeds over-the-network, and uncovers elision-relevant state on
various targets"). It is the FIRST multi-party-adjacent work.

IS NOT: a pretty ANSI TUI (that is the thin FINAL surface — minimal here). NOT a full
multi-host DST fleet harness (the ru-25 named rabbit-hole). NOT a new analysis engine
(it re-uses the built one — see LB-1). NOT real network transport (out of scope; the
concurrent-arrival SOURCE is `hostsim`, the DST simulator).

## §1 Architecture — the static/dynamic split (LB-1)

**LB-1 (~SUSPECT; structurally +SURE, fold-internals unverified):** the analyzer
already splits a probe-INDEPENDENT static half from a fact-CONSUMING dynamic half.
In the current cli `run()`, `parse → cfg::build → value::analyze →
effect::classify_with_why_diags → compile_probe` ALL run BEFORE a single probe result
is read from stdin; only `build_plan` consumes the (re-keyed) probe observations. So
the realtime loop is: **classify ONCE**, then as probe facts stream in, **RE-FOLD the
plan** over the accumulated facts. NOT a full-analyzer rerun per batch.

The minimal-slice fold MAY simply re-run `build_plan(classes, accumulated_facts)` each
batch (cheap — `classes` is fixed, `build_plan` is a pure fn of classes+facts). A
truly INCREMENTAL fold (re-touch only the sites a new fact affects) is an
OPTIMIZATION, DEFERRED (we like extra work on non-human timescales; the network
dominates — but NB the human's correction: the *analysis* re-fold is NOT
network-masked, so if `build_plan` is expensive × many batches × many hosts it could
bite — measure before optimizing, do not pre-build incrementality).

CROSSCHECK TARGET: is LB-1 actually true — does anything probe-dependent leak into the
static half? If `classify`/`compile_probe` reads a fact, the "classify once" claim
breaks and the whole loop is wrong.

## §2 The load-bearing property — MONOTONICITY (LB-2)

**LB-2 (~SUSPECT — the single most load-bearing claim; the contract REQUIRES the
build to PROVE it, not assume it):** converged-facts only ever ADD elisions. The
default is kFAIL-perform (no fact ⇒ run, no elision); facts are site-keyed
(`inv-site-keyed-results` — a fact for site B does not touch site A's decision); and
⊤-status is STATIC (it comes from unmodeledness, computed in the static half — probe
facts answer *convergence*, never *modeledness*). Therefore the plan's elision-set
GROWS MONOTONICALLY as facts arrive; an arriving fact NEVER retracts a prior elision.

Consequence (the UX + the tractability): the streaming plan only ever TIGHTENS (more
commands elided) as hosts check in — no churn, no retraction, no "un-eliding" a line
already shown elided. This is what makes incremental folding safe AND the realtime
display coherent.

If LB-2 is FALSE (some fact retracts an elision), the whole streaming story acquires a
retraction problem (a shown-elided line must un-elide — a kFAIL-perform-DANGER if the
user already acted on the tightened plan). THE CROSSCHECK MUST ATTACK THIS HARDEST:
find any fact-arrival order, any cross-site fold interaction, any door/errexit/loop
case where accumulating a fact REMOVES an elision the prior fold had. Verification in
the build: a DST property test asserting the elision-set is monotone-nondecreasing
across the arrival sequence (§6).

## §3 Concurrency + DST — the arrival model (LB-3)

**LB-3 (-GUESS on hostsim's current capability; the SEAM is required regardless):**
N hosts' probe streams arrive concurrently; the realtime loop folds each arrival into
THAT host's plan. The concurrency source is `hostsim` (the DST simulator), driven by a
SEEDED LOGICAL CLOCK that orders arrivals deterministically (ru-25's "DST
timing/logical-clock dependency"). The clock is the ONLY nondeterminism, and it lives
behind the DI seam (`inv-determinism` — the kernel + fold stay pure; the clock is
injected/seeded).

OPEN (fork-hostsim-concurrency): does `hostsim` already model concurrent multi-host
arrival + a logical clock, or must the minimal seam be built? The builder determines
this FIRST and either uses the existing seam or builds the smallest one. This is where
the rabbit-hole lives — model concurrent ARRIVAL deterministically, NOT a full fleet.

CROSSCHECK TARGET: is a "logical clock over N arrival streams" the right determinism
model, or does it smuggle in ordering assumptions that make the test pass vacuously
(the elision result must be ARRIVAL-ORDER-INDEPENDENT if LB-2 holds — see LB-4)?

## §4 The surface — extend `plan`, reuse `advisory_filter` (LB-4, from 22F-fd5/fd6)

**LB-4 (+SURE on the surface, ~SUSPECT on order-independence):** ui-B extends the
`plan` mode (NOT `apply`, NOT the round-trip — 22F-fd5). The streaming UNIT is the
per-site advisory line; the logical clock is the per-site probe-return ordering. ui-B
REUSES the existing `advisory_filter` decision (22F-fd6 — do NOT invent a second
severity policy; that is the dac-B two-sources-of-truth hazard). The byte-floored
`apply` artifact is NEVER a streaming surface.

Order-independence corollary (the clean acceptance, ties to LB-2): if the elision-set
is monotone in facts AND facts are site-independent, the FINAL per-host plan is
INDEPENDENT of the arrival order — only the intermediate TIGHTENING sequence differs.
So a strong acceptance test: incremental-fold-final == single-shot-all-facts-at-once
(see §6). If the final result DOES depend on arrival order, LB-2 or
site-independence is false.

## §5 The CLI-shape it serves (ru-29)

The real `plan` (per ru-29) = dispatch probes + WAIT + slurp results (concurrent) +
build coherent per-host plan(s) → emit a few `.sh` files, ONE-PER-HOST. ui-B builds
THIS `plan`-phase realtime engine. ui-A's stdin-single-batch `plan` is the spike
stand-in; ui-B replaces the single-batch slurp with a CONCURRENT hostsim-driven stream
and writes PER-HOST plan files (uib-3 multi-file output). `apply` (take-in-files +
ship) is UNCHANGED by ui-B. The probe/plan split commands of ui-A stay; ui-B is about
what `plan` does INTERNALLY when fed a concurrent multi-host stream.

## §6 The minimal-viable slice + acceptance (uib-scope) — held APART from the full vision

MINIMAL SLICE (the smallest thing that proves the engine; build THIS):
- a `plan` path that accepts a CONCURRENT probe stream from a SMALL N (2–3) simulated
  hosts via `hostsim`, seeded arrival order;
- `classify` once (LB-1); re-fold per arrival (LB-2); write PER-HOST plan `.sh` files
  that TIGHTEN as facts arrive;
- minimal "streaming" = emit the per-site plan-deltas (or a fold-sequence log) as they
  happen — NOT an ANSI TUI (that is the deferred thin surface).

ACCEPTANCE (DST tests — the deliverable, not a pretty demo):
- ACC-1 (incremental == batch): the incremental-fold FINAL per-host plans are
  byte-identical to a single-shot analysis fed all facts at once (proves no
  information is lost by streaming; ties LB-4 order-independence).
- ACC-2 (monotone tightening): across the arrival sequence, each host's elision-set is
  monotone-NONDECREASING (proves LB-2 operationally; the property the crosscheck
  attacks).
- ACC-3 (determinism): same seed ⇒ same fold sequence + same final plans
  (`inv-determinism`; the logical clock is the only nondeterminism, DI'd).
- ACC-4 (per-host isolation): a fact for host A never changes host B's plan (the
  multi-party correctness floor; the parked cross-host dependency stays parked).

ESCAPE VALVE (ru-25): if the slice grows heavy, it becomes r23. The line to hold:
prove the ENGINE (ACC-1..4) on a tiny N with a minimal display; do NOT build the fleet
harness, the real network, or the rich TUI.

## §7 Welds it MUST honor

- `inv-determinism` — the logical clock is the ONLY nondeterminism, DI'd/seeded; the
  fold is a pure function of (classes, accumulated facts, arrival order); ordered
  collections only.
- `kFAIL-perform` — monotone tightening PRESERVES it: never elide-then-need. An
  un-reported site stays RUN. (LB-2 is the formal statement of this under streaming.)
- `rec-1` two surfaces — the per-host `.sh` files are byte-floored apply-class
  artifacts (receipt-free); the streaming advisory is the render surface
  (`plan`-tier, `advisory_filter`).
- `inv-site-keyed-results` — the per-site fold + the per-host isolation rest on this.
- `inv-superposition` — the realtime `plan` is a SECOND real phased caller (after
  apply) IF it collapses facts per-arrival; watch that it does not bake a phase
  default the engine should emit un-collapsed.

## §8 Scope cuts / forks (conductor dispositions; flag at build)

- PARKED (ru-28): cross-host plan dependency (one host's plan depending on another's
  facts) — EXTREMELY far-future, ZERO cycles; ACC-4 actively PINS the per-host
  independence the parking assumes.
- fork-hostsim-concurrency (§3): use the existing hostsim concurrency/clock seam or
  build the minimal one — builder determines, flags if it is more than minimal.
- fork-incremental-vs-rerun (§1): minimal slice may re-run `build_plan` per batch;
  true incrementality DEFERRED (measure first).
- DISPLAY: minimal (delta/log emission); the ANSI TUI (ru-20 ui-2 full pretty-mode) is
  DEFERRED — ui-B proves the engine, not the pixels.
- NO real network/transport (`kCOMMS`, plans/142) — out of scope; hostsim stands in.
- wire-format probe digest (ru-29 tc-probe-no-digest) + split-phase round-trip test
  (tc-probe-results-roundtrip) — task #19, round-end, NOT ui-B.

## §9 Dispatch shape + crosscheck targets

After the adversarial crosscheck lands + reconciles: ONE Opus builder, fb-19-clamped,
conductor-created worktree at the verified ai/spike3 tip, the §6 minimal slice, the §7
welds, granular commits, full gate chain per commit, NO BLESS, surface strain (a
notes/22x). The crosscheck (THIS contract's adversary) should hit, in priority order:
LB-2 (monotonicity — find a retraction), LB-1 (a probe-dependence leak into the static
half), LB-4 (arrival-order dependence of the final plan), LB-3 (a vacuous
determinism model), and the uib-scope line (is the "minimal slice" actually minimal,
or does ACC-1..4 smuggle in the fleet harness?).
