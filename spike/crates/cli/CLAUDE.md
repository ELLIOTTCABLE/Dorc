# spike/crates/cli — CLAUDE.md

The round-trip: book + oracles → read-only probe → results → eliding apply. Read `spike/CLAUDE.md` and `Research/plans/190-spike2-keystone-charter.md`.

This is the one place determinism is relaxed (real I/O); `inv-determinism` exempts the `cli` edge *only*. Keep the I/O in `run()`/`main` and the pipeline (`parse → cfg::build → effect::classify → plan::{compile_probe, build_plan}`) a total `Carrier<T>` function of its inputs — don't let a clock/RNG/env-read leak inward to "help". Diagnostics go to stderr (`report`); stdout stays exactly probe-then-apply, or the e2e capture (and any real downstream pipe) breaks.

## The ap-2 acceptance harness — the cli's headline spike-2 job

The harness lives at `spike/e2e/run.sh` (sh-mechanized, not a Rust/cargo harness — `16P T16`). Today it golden-diffs stdout *text* (`[ "$got" = "$want" ]`) and never executes nor `sh -n`-checks the rendered artifact. That is the `ap-2` defect, live and reproduced on the fork: `cases/guarded/expected.out` ships

```sh
if true; then
#    apt-get install -y nginx   # dorc: elided (already converged)
fi
```

— an empty `then`-clause, which is a **POSIX syntax error** (`fi` where a command-list is required), shipped *green* because the harness only string-compares (`16Q ap-2`). For a tool whose whole contract is "the output is just shell you can run," the acceptance test must **execute or `sh -n`-check** the rendered apply (and the probe) — never text-diff it (`an-render-executability-check`, `an-render-runnable`). +SURE this is day-1 for a "functioning" goal; the fix has two halves and `cli` owns the harness half:
- harness (`cli`): add an `sh -n`/`dash -n` gate on the emitted probe and apply per case (the existing text golden-diff may stay as a *secondary* check, but the runnability gate is the load-bearing one). ~SUSPECT this can stay sh-side in `run.sh`; a thin Rust integration test under `cli/tests/` that shells out to `dash -n` is also fine if it reads cleaner — either way, **assert runnability, not bytes**.
- emitter (`plan`): `render_apply` must emit `sh -n`-clean POSIX (the empty `then`-body fix). That's the `plan` crate's job — see `spike/crates/plan/CLAUDE.md` — but the harness here is what *catches* a regression, so the two move together.

Build first (mise keys trust by path, one-time after the round-19 fork): `mise trust mise.toml; mise trust spike/mise.toml`, then from `spike/` run `mise exec -- cargo build -p dorc-cli` and `sh e2e/run.sh` (it auto-locates `target/{debug,release}/dorc[.exe]`, or take `DORC=…`). `BLESS=1 sh e2e/run.sh` regenerates goldens — but blessing a non-runnable artifact is exactly the `ap-2` trap, so the runnability gate must run *before/independent of* bless.

## The probe-projection edge — the second phased caller (`F-FW3`)

The probe today emits the oracle body with `$1` **unbound** — illustrative, no per-entity binding, and there is no separate *probe* plan-builder yet (`16P T16`/`T11`, `q1-probe-projection`). When that gets built, `cli` is where it surfaces: the probe-plan-builder is the **only** place `inv-superposition` ever gets a real *second* phased caller (`17O F-FW3`) — until now `build_plan` (apply) has stood in alone as if phase-agnostic. Building a real `Phase::Probe` caller here is the load-test of "engine emits, caller collapses": if `May`/`Must` superposition survives two real phased callers it earns its locks; if it breaks, they were premature (`inv-superposition`, `inv-kfail` — probe withholds on ⊤, apply performs on ⊤, never traded).

The probe model `cli` drives is **speculate-and-intercept** (`17O R2-PROBEGATE`), not Ansible check-mode: oracles intercept (an `id__check` ships and replaces `id`), and a probe-gated branch is resolved by *running the read-only probe for real*. The probe is compiled from oracle bodies + minimal CFG fragments, **never the book's contents** — so it never inherits the book's ambient `trap`s. (`hostsim` answers it; `cli` just compiles + ships + reads back.)

## The stdin re-key gotcha (entity-algebra)

`parse_results` keys verdicts by a flat `kind:entity` string (`fact_label`), and a missing fact folds to `Verdict::Unknown ⇒ run` (`kFAIL-perform` — keep that default, it's load-bearing). The keystone re-key (`an-entity-shape`, `ap-1`) makes verdicts **per-selector** (`package:nginx#installed` vs `#version`), not one bit per kind/entity — so this stdin format and `fact_label` move *with* the re-key. ~SUSPECT the line grammar will need a selector field; don't quietly drop a selector on parse and silently widen a verdict to the whole entity (that's a wrong-elision under apply's `kFAIL`).

## Scope boundary (don't build toward these)

The real apply-executor over time, transport/`kCOMMS` (`plans/142`), and multi-host fan-in are **out of spike scope** (`ch-scope`, charter §6). `cli` *compiles* a probe and an apply and runs neither; the host's answers arrive on stdin (a stand-in for running the probe remotely). The only executor extension in scope is the thin backward / `dorc bump` apply-3 skeleton (`ch-scope`, `an-apply-3`) — and that mostly lands in `plan`; `cli` just needs a flag/path to drive it. Keep the binary a thin driver: arg-parse, file-read, call the kernel, print. Resist absorbing pipeline logic.

## Tension to surface, not resolve

`ap-2`-runnability (`sh -n`) and the existing **text golden-diff** are in mild tension, and which is canonical isn't obviously settled. `sh -n` proves the artifact *parses* but not that it elided the *right* lines (a `render_apply` that comments out *everything* is `sh -n`-clean and useless); the text golden catches wrong-elision content but is structurally blind to non-runnable output (the `ap-2` defect). --WONDER whether the honest harness needs *both* — a runnability gate (catches `ap-2`) plus a content assertion that isn't a brittle full-text diff (e.g. assert specific leaves are/aren't commented, or execute the apply against a fixture and check observable effects per `an-render-executability-check`). Flagging rather than picking: the charter mandates the runnability gate (`ap-2`), but silently deleting the content check to get there would re-open the wrong-elision blind spot from the other side.
