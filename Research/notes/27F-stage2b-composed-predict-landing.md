# 27F — block-rebuild stage-2b (composed-predict repair) landing + residue

AI-authored (Opus builder, r27 stage-2b session). Records what landed for the `24J` raw-ship
repair (`271:rul-only-oracle-bytes-ship`) + the coupled A6 otelcol retirement + the `279f` §5
SIGPIPE riders. Companion to `27D` (the conductor ledger) and `27E` (stage-2). Authority: root docs
+ `spike/CLAUDE.md` rulings + `271`/`273` outrank this.

## What landed (all green: 139 hostsim + 91 plan + … unit / 128 e2e; 4 gates clean)

- **The repair** — the connected-probe lane ships ONLY oracle-authored bytes. A probed pipeline
  `A | F` composes each stage's stripped predict (`a__predict argv | b__predict argv`); the raw
  book pipeline never ships. The admin's argv flows as the predicts' arguments through each
  author's argparse (`271:rul-argv-flows-bytes-do-not`). e2e delta: two cases' probe artifacts
  changed from the raw `otelcol --version | grep -q "0.155.0"` to the composed form; the elided
  apply is byte-identical; the record is parity-preserving (`site 1 effect=holds rc=0`, verified
  under mocks).

- **A6** — the three pipe-guard otelcol oracles' improvised empty-entity singleton bind
  (`v : io.opentelemetry.Collector` + `>/dev/null 2>&1` + `:? …:#v0155`) retired to a clean
  read-only OBSERVE delegation (`otelcol --version :? io.opentelemetry.Collector:"otelcol"#version`
  — real stdout, concrete entity). `if-form` HAD to convert too: its `>/dev/null` non-last stage
  declines the very channel `grep` consumes, so under the coverage rule it would refuse the
  compound and regress elide→run. `unvouched-mid` converted for consistency (its `cat` still walls
  the pipe ⇒ otelcol stays an orphan and runs — behaviour unchanged).

## The substitution-coverage rule's implemented shape (rider 1 — stage-4 consumes this)

- **Where the per-channel decision lives:** `dorc_oracle::predict::predict_stage_stdout(check, argv)
  → StageStdout {RealBytes | Asserted | Declined}` (the `273` §2 vocabulary). It reuses the predict
  EVALUATOR (`Evaluator::over` — the same argparse trace `evaluate` runs) and reports the STDOUT
  coverage of the LAST producing command reached on the selected path. The classifier: a
  `>/dev/null`-class stdout redirect ⇒ `Declined`; a `printf` head ⇒ `Asserted` (declared, not
  world-spoken — rider-3-refused for a knife-tier byte-consumer); anything else (delegation) ⇒
  `RealBytes`. Redirect detection is a new parse-time `Command.stdout_void` bit
  (`redirect_voids_stdout`: only an OUTPUT redirect on fd 1 voids stdout; `2>&1`/`2>/dev/null`/`<…`
  leave it on the pipe).

- **How declines flow / where the compound decision lives:** `plan::connected_check_pipes` is now
  the connected DECIDER (not a pure recogniser). The cli/coverage/sweep pass a `ship_stage` closure
  returning `StageShip {sh, produces_real_stdout}` (the cli collapses `StageStdout::RealBytes` to
  the bool). For each recognised all-Query pipe, the decider resolves every stage and applies the
  gate: a NON-LAST stage must be `RealBytes` (its stdout is the byte the next stage consumes); the
  LAST/governing stage's stdout is NOT piped onward (only its rc is consumed), so it is EXEMPT from
  the stdout gate. ANY stage that fails to resolve OR any non-last stage that is not `RealBytes`
  REFUSES the whole compound — every stage is demoted to an ORPHAN (reusing the existing
  negative-control mechanism ⇒ each runs; no partial/mixed emission). Shippable compounds key the
  governing node → `ComposedProbe`, non-last → members; refused → orphans only. The apply side is
  unchanged and safe by construction: a refused compound never enters `members`, so
  `build_plan_walled`'s member-OMIT never fires; the orphan stages take the ordinary
  can't-probe⇒run path.

- **The structural no-book-bytes pin** (the law machine-checked, not builder-memory):
  `plan::…composed_probe_renders_predicts_never_raw_book_bytes` renders a ComposedProbe and asserts
  the composed invocation ships AND the distinctive raw book spelling `--version | grep -q` NEVER
  appears. Two refusal pins: `connected_refuses_non_last_stage_without_real_stdout` (rider 1) and
  `connected_refuses_when_a_stage_predict_does_not_resolve`. Oracle-side: five `stage_stdout_tests`
  pin the `273` §2 classification (delegation / redirect-void / stderr-only / printf / unmatched).

## SIGPIPE riders (`279f` §5 / sigpipe-flap-class)

- hostsim `Host::with_sigpipe_race(seed, flappy)`: a raced fact observes `Unknown` (the ≥2 flat
  sink / cant-tell) instead of its true verdict — seeded, bit-reproducible (goldens cannot flap),
  `model-the-outcome` (inject at the seam, never real SIGPIPE). The DST
  `dst_composed_probe_under_sigpipe_race_lands_in_sink_without_flapping` builds a real composed
  probe, injects the race on the governing fact, and pins: (1) no flap (two builds at one seed
  render byte-identical); (2) the ≥2-sink landing runs the pipe (never a wrong elision); (3)
  reachability (fires for some seeds, not all — `sometimes-assert`).
- cli emits a why-lane note on every rc-141 record (advisory, feeds no decision).
- No `--exit-code` surface exists; the contract (compute from divergence-of-world, never raw
  sink-landings) is recorded in a doc-comment at the sink-landing site (`render::probe::record_scaffold`).

## tc-* / judgment calls flagged UP (never settled locally)

- **tc-last-reached-command-coverage** — `predict_stage_stdout` reports the LAST producing command's
  stdout coverage. For a single-delegation arm (the whole corpus) this is exact. A multi-command arm
  (`a; b` where both produce stdout) is NOT modelled precisely — the arm's true stdout is the
  concatenation, but the heuristic reports only `b`'s. Safe direction (a non-RealBytes tail refuses),
  but a `RealBytes; RealBytes` arm and a `printf; RealBytes` arm both read `RealBytes` — the latter
  is arguably `Asserted`+`Real` mixed. No corpus arm exercises this; flag for the value-recipe-reshape
  when per-channel Observable production lands (which subsumes this heuristic).
- **tc-stage-ship-triplication** — `ship_predict_stage` is duplicated across cli/coverage/sweep
  (mirroring the pre-existing `ship_predict_body` triplication for those harness crates). Not
  factored — consistent with the existing pattern; a shared extraction is a separate cleanup.
- **A6 entity choice** — the converted otelcol coordinate (`io.opentelemetry.Collector:"otelcol"#version`)
  is semantically arbitrary: the otelcol stage is SUBSUMED (the governing grep cell drives the
  verdict), so its own cell is never probed. Any resolving Observe coordinate works; I picked a clean
  one. Not a pin.
