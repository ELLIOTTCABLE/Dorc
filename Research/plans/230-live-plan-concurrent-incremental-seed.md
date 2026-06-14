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

## §1 Monotonicity — stated CORRECTLY (the central r22 correction)

The original contract claimed "facts are per-command-site, independent across sites" and
"the plan only tightens, never un-replaces a shown line." BOTH were wrong framings; the
human corrected them and they are the load-bearing thing this seed fixes.

- **Cross-site coupling is the ENGINE, not a flaw.** It is a dataflow analysis — of course
  a guard controls a body (`dpkg -s nginx || apt-get install nginx`). The fold
  (`plan/src/fold.rs` `eval_and_or`/`eval_if`) reads a controller leaf's Status to mark a
  *different* leaf dead → `Disposition::Omit`, cross-site by construction
  (`dead: BTreeMap<AstId_body, AstId_controller>`), and that fold-Omit takes precedence
  over a site's own convergence-`Replace` (`build_plan`'s `disposition_for` runs the fold
  first). This is correct and expected.
- **The monotone quantity is the RUN-COUNT, not the per-line disposition.** With each
  probe-fact received, the NUMBER of commands that will RUN equals-or-reduces — never
  increases (+SURE, verified against the fold logic):
  - no fact ⇒ a command is conservatively RUN (kFAIL-perform: when unsure, run) — the max.
  - a fact moves a command run → not-run: `Replace` (its own state came back converged) or
    `Omit` (a controlling guard's fact resolved its branch dead).
  - once not-run, it STAYS not-run; reversing needs a CORRECTED fact (same site, a different
    value later) — which does NOT occur here (see the single-pass model below).
- **What is NOT monotone, and is FINE:** WHICH commands run, and HOW each not-run command
  renders. A line shown `Replace` (its own fact arrived first) can flip to `Omit` when its
  guard's fact lands; the display churns, the count only shrinks. As a higher CFG node
  resolves, the nodes beneath it necessarily change — that is the plan updating live, not a
  violation.
- **THE SINGLE-PASS / NO-CORRECTION MODEL (load-bearing; its absence made both the crosscheck
  and the conductor over-worry a non-issue).** During the plan-phase slurp, each site reports
  ONCE. There is NO re-probing/correction at this stage: the only re-probe ever considered is
  baked into the apply-script body (far-future, not-set-in-stone; the human leans toward
  STATIC apply-scripts). So no fact is ever corrected mid-stream ⇒ the run-count is strictly
  monotone non-increasing. (If apply-embedded re-probe ever lands, revisit — a corrected
  controller fact could un-Omit a body; out of scope now, tripwire registered.)

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
