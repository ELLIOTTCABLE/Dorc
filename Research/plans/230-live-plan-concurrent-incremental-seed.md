# 230 — live-plan: concurrent-probe streaming → incremental per-host re-analysis (r23 seed)

> r23 SEED (forward-looking; the dense starting point, not a build-contract — a
> build-contract gets drafted from this in r23, then adversarial-crosschecked before
> building). Renamed from the misnomer "ui-B" (it expanded far past a streaming-UI
> proof). This seed is the OUTPUT of an r22 design pass: a first contract draft
> (the removed notes/22G), a two-pass adversarial crosscheck on it, and the human's
> correction of its central premise. DEFERRED out of r22 deliberately (see §8). Marks:
> +SURE / ~SUSPECT / -GUESS / --WONDER.

## §0 What it IS (the reframe — ru-28)

The analyzer running in a REALTIME, CONCURRENT regime: probes run on N hosts and their
results stream back asynchronously; each arriving batch is folded into the being-built
plan to surface newly-provable replacements; the output is per-host plan(s) that tighten
as hosts report. It is the FIRST multi-party-adjacent work. The live DISPLAY is the thin
FINAL surface — the substance is the concurrent incremental re-analysis ENGINE.

Grounded verbatim in DESIGN ("Dorc's approach" §3): the plan is "dynamically updated in
real-time as the probe-phase asynchronously proceeds over-the-network, and uncovers
elision-relevant state on various targets." WHY: realtime feedback (the operator isn't
blocked on the slowest host; the plan visibly resolves) + per-host plans (a command
replaceable on host A may still run on host B).

## §1 Replacement-stability — a best-effort property to INVESTIGATE, not a proven invariant

(Downgraded per the human, 2026-06-14.) The first draft centered a critical-tier claim —
"facts are per-site independent; the plan only tightens; run-count monotone non-increasing."
That was wrong on premise, certainty, AND unit, and it is NOT the center of this engine (§2
is). What's true, carefully:

- **Cross-site coupling is the ENGINE, not a flaw.** It is a dataflow analysis — of course a
  guard controls a body (`dpkg -s nginx || apt-get install nginx`). The fold
  (`plan/src/fold.rs` `eval_and_or`) reads a controller leaf's Status to mark a *different*
  leaf dead → `Disposition::Omit`, cross-site by construction
  (`dead: BTreeMap<AstId_body, AstId_controller>`), precedence over a site's own
  convergence-`Replace`. Expected.
- **The unit is REPLACEMENT, not run/not-run.** What "not run" MEANS changes (Replace-with-v
  / Replace-with-v′ / Omit), so a "run-count" is the wrong thing to call monotone — the
  replacement's CONTENT is what moves.
- **Replacement is NOT provably stable (the over-claim, retracted).** A replacement reproduces
  a command's consumed observables, each a `Predicted<T>` = `Value(v)` | `Top`; a `Top` on a
  consumed channel FORBIDS the substitution (`core/lib.rs:362`, kFAIL-perform). Across arrivals
  one cell goes `Top` (no fact) → `Value` (a fact lands) → and a DISAGREEING second fact merges
  BACK to `Top` (`merge_observable` = meet-toward-⊤), flipping a consumer `Replace` → `Run`.
  That is a replace-then-re-replace-in-a-different-direction, needing NO TOCTOU — just two
  sites disagreeing on one cell, in sequence. Reachability in single-pass (two establishers
  genuinely disagreeing on one host-cell) is UNVERIFIED (~SUSPECT); correction/TOCTOU breaks it
  further (out of scope). So replacement-stability is a best-effort GOAL ("mostly don't
  introduce new changes"), NOT a proven monotone invariant.
- **CERTAINTY IS TIERED BY PROVENANCE — but the type to do it with is welded OUT of decisions
  (the load-bearing tension).** Trustworthy-monotone: the pure-CFG-structural tier (control-flow
  + data-dependencies — fully-trusted immutable input; folds kill, never revive). NOT: the
  replacement-content riding on probe/oracle-tainted `Predicted` values. The type that
  distinguishes these EXISTS — `OriginKind` (`core/prov.rs:76`): `BookSource` / `OracleClaim` /
  `ProbeResult` / `TopCause` / `Join` — but (a) ru-11 welds it DECISION-INERT (it may "influence
  nothing — not a license, not a fold, not a disposition"; the kind grounds the why-lens
  EXPLANATION, never a decision), and (b) `OracleClaim`/`ProbeResult` are RESERVED-not-yet-minted.
  The decision-plane certainty type `Predicted<T>` (`Value`/`Top`) IS read by decisions but is
  SOURCE-BLIND. So an algorithmic "pure-CFG-can-only-downgrade vs probe/oracle-tainted"
  certainty-claim has NO clean home today: the source-type is decision-inert, the decision-type
  is source-blind. **r23 design question (do NOT pre-decide):** does the engine need a NEW
  ru-11-compatible decision-plane source-distinction to make replacement-stability a TYPED
  guarantee, or does it stay best-effort + explanation-tiered? Either way, any certainty-tier
  claim must rest on that typed analysis, never on a hand-waved "monotone."
- The single-pass / no-correction model (each site reports once in the plan-phase; re-probe is
  apply-script-embedded, far-future, human leans STATIC apply-scripts) removes the
  correction-retraction but NOT the same-cell-disagreement retraction above — necessary, not
  sufficient.

Terminology: run / `Replace` / `Omit`. Avoid "elide".

## §2 What must be BUILT (honest engine-sizing — the crosscheck's surviving finding)

The original contract leaned on `hostsim` for "a seeded logical clock ordering concurrent
arrivals." It does not exist (+SURE, both crosscheck passes + the crate docs): `hostsim` is a
synchronous, SINGLE-host, CLOCKLESS set-membership oracle; its seeded PRNG seeds initial host
STATE, not arrival TIME (`hostsim/CLAUDE.md`); no async, no streams, no per-host plan emission
(`cli` emits one `Plan`; `cli/CLAUDE.md`: multi-host fan-in is out of *spike* scope). So this
is real engine-building, NOT a thin surface — r23 must build:
- a deterministic ARRIVAL-ORDERING seam — a DI'd, seeded logical clock over N arrival streams,
  under `inv-determinism` (the only nondeterminism; its own fuzz/`an-sometimes-assert` coverage);
- PER-HOST accumulators carrying each host's facts-so-far across batches (the cli today reads
  stdin to EOF once and builds ONE plan — no per-host state-carrier exists);
- the session loop: classify once, re-fold per host as that host's facts arrive, emit per-host
  plan files (the `Disposition` set per host) + minimal live deltas.

## §3 What's REUSABLE (survives — both passes agree)

- The per-CALL static/dynamic split is real (+SURE): `parse → cfg → value → classify →
  compile_probe` are probe-INDEPENDENT and run before any fact is read; only `build_plan`
  consumes facts (a pure fn of classes + an `observe` closure + arena). "Classify once,
  re-fold per batch" is sound — BUT note `build_plan` re-runs the WHOLE fold + re-derives every
  `LeafId` from a span-sort each call: it is a full fold over the cached static half, not an
  incremental delta. Cheap enough (the static half is skipped; network dominates), but do not
  oversell it as incremental; a truly incremental fold is a deferred optimization (measure
  first — the re-fold is NOT network-masked).
- The same-cell `merge_observable` is conservative meet-toward-⊤, commutative + idempotent
  (+SURE) ⇒ accumulation order into one cell is safe.
- Terminal determinism (+SURE): the incremental-FINAL per-host plan is byte-identical to a
  single-shot analysis fed that host's full fact-set at once (because `build_plan` is pure +
  the merge is order-independent). This is a real acceptance anchor — but it tests the trivial
  (idempotent-recompute) direction; see §4.
- Surface reuse (+SURE): `render_apply` (byte-floored artifact) + `advisory_filter` (the
  receipt-free severity cut) are reused; per-host files are `Apply`-class artifacts (rec-1).

## §4 Acceptance — target the REAL accumulator (not the vacuous tautology)

The original ACC-4 ("a fact for host A never changes host B's plan") is VACUOUS: `build_plan` is
pure, so N per-host runs are independent BY CONSTRUCTION — the test cannot fail and proves
nothing (the spike-1 self-confirmation trap). r23's acceptance must drive the REAL concurrent
per-host accumulator and assert: (a) RUN-COUNT monotone non-increasing across each host's arrival
sequence (§1 — the property, on a guard+body book, under a multi-arrival trace the current
all-facts-at-once DST cannot exercise); (b) per-host partition holds under the shared accumulator
(facts route to the right host); (c) terminal == single-shot per host (§3); (d) determinism under
the seeded clock (same seed ⇒ same fold sequence + same final plans).

## §5 The CLI-shape it serves (ru-29)

The real `plan` = dispatch probes + WAIT + slurp results (concurrent) + build coherent per-host
plan(s) → a few `.sh` files, ONE-PER-HOST. `apply` = take in those (possibly user-edited) files +
ship. {probe&plan} → wait-for-user-action → {apply}. ui-A's stdin-single-batch `plan` is the
spike stand-in; the live-plan engine replaces the single-batch slurp with the concurrent stream +
per-host file output. TUI mode may collapse to one interface + an "apply?" button.

## §6 Scope cuts / parked

- PARKED (ru-28, EXTREMELY far-future, zero cycles): cross-host plan dependency (one host's plan
  depending on another's facts — rolling-update/quorum/drain class). Per-host independence is the
  working assumption; acceptance (b) pins it.
- The probe-RESULTS wire-format wants a drift/identity digest + the split-phase round-trip wants a
  book-hash guard (r22 tc-probe-no-digest / tc-probe-results-roundtrip, task #19) — these cross-lock
  with this engine (the per-host streams are the same records); fold in here.
- No real network/transport (`kCOMMS`, plans/142) — `hostsim` stands in, but `hostsim` must FIRST
  gain the concurrency/clock seam (§2). Minimal live display (deltas/log); the rich ANSI TUI
  (ru-20 ui-2) is deferred.

## §7 Process (the r22 → r23 handoff)

This seed is the output of the design-crosscheck-before-build loop working: the first contract's
naive per-site-independence premise + its over-stated monotonicity were caught (by the human's
correction more than the agents — the agents inherited the contract's under-specification of the
single-pass model), and the one finding that survived (the concurrency engine must be built) is
correctly load-bearing. r23: draft a build-contract FROM this seed (nailing §1's single-pass model
+ §2's seam + §4's real-accumulator acceptance up front), then re-run the adversarial crosscheck on
THAT, then build. Deferred from r22 because the durable probe-tape (arch-4) and this engine both key
on the probe-result stream — freezing either against the single-shot model before this engine exists
is the rework/cruft the deferral avoids.
