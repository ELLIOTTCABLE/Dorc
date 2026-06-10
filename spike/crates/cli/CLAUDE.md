# spike/crates/cli — CLAUDE.md

The round-trip: book + oracles → read-only probe → results → eliding apply. Read `spike/CLAUDE.md` and `Research/plans/191-spike2-keystone-charter.md`.

This is the one place determinism is relaxed (real I/O); `inv-determinism` exempts the `cli` edge *only*. Keep the I/O in `run()`/`main` and the pipeline (`parse → cfg::build → effect::classify → plan::{compile_probe, build_plan}`) a total `Carrier<T>` function of its inputs — don't let a clock/RNG/env-read leak inward to "help". Diagnostics go to stderr (`report`); stdout stays exactly probe-then-apply, or the e2e capture (and any real downstream pipe) breaks.

## The ap-2 acceptance harness — current state (round-20)

The harness lives at `spike/e2e/run.sh` (sh-mechanized, not a Rust/cargo harness — `16P
T16`) and carries, per case: a **`dash -n` gate on BOTH rendered artifacts** (probe and
apply — the load-bearing `ap-2` runnability gate; the historical trap was a text-only
golden-diff that shipped a non-runnable empty `then`-clause green, twice); an
**exec-under-mocks gate** for cases with a `mocks/` dir (the rendered apply runs under
`PATH=mocks-only` inert shims; the sorted run-set is asserted against `expected.ran`,
which must EXIST for a mocks case — missing ⇒ loud fail, never empty-want); a
**crash/empty guard** (dorc's exit status is captured un-piped; rc≠0 or empty output
hard-fails the case before the xfail lens and before bless — a dead engine is never
green and never blesses); the **content golden-diff** as a secondary check (catches
wrong-elision *content*, to which `-n` is blind); and the **XFAIL/XPASS** pin
machinery (an XFAIL case asserts the safe behavior of a known defect; a surprise pass
is a loud XPASS-to-promote). The cli's contract with all of this: stdout is EXACTLY
probe-then-apply (split on `#!/bin/sh` shebangs), diagnostics to stderr only.

Build: from `spike/`, `mise exec -- cargo build -p dorc-cli`, then `sh e2e/run.sh`
(auto-locates `target/{debug,release}/dorc[.exe]`, or take `DORC=…`). `BLESS=1`
regenerates goldens; the runnability + crash gates run before bless, but bless still
cannot prove an elision RIGHT — review blessed diffs by eye, and never bless while any
engine behavior is in doubt.

## The probe-projection edge — the second phased caller (`F-FW3`)

The probe today emits the oracle body with `$1` **unbound** — illustrative, no per-entity binding, and there is no separate *probe* plan-builder yet (`16P T16`/`T11`, `q1-probe-projection`). When that gets built, `cli` is where it surfaces: the probe-plan-builder is the **only** place `inv-superposition` ever gets a real *second* phased caller (`17O F-FW3`) — until now `build_plan` (apply) has stood in alone as if phase-agnostic. Building a real `Phase::Probe` caller here is the load-test of "engine emits, caller collapses": if `May`/`Must` superposition survives two real phased callers it earns its locks; if it breaks, they were premature (`inv-superposition`, `inv-kfail` — probe withholds on ⊤, apply performs on ⊤, never traded).

The probe model `cli` drives is **speculate-and-intercept** (`17O R2-PROBEGATE`), not Ansible check-mode: oracles intercept (an `id__check` ships and replaces `id`), and a probe-gated branch is resolved by *running the read-only probe for real*. The probe is compiled from oracle bodies + minimal CFG fragments, **never the book's contents** — so it never inherits the book's ambient `trap`s. (`hostsim` answers it; `cli` just compiles + ships + reads back.)

## The stdin re-key gotcha (entity-algebra)

`parse_results` keys verdicts by a flat `kind:entity` string (`fact_label`), and a missing fact folds to `Verdict::Unknown ⇒ run` (`kFAIL-perform` — keep that default, it's load-bearing). The keystone re-key (`an-entity-shape`, `ap-1`) makes verdicts **per-selector** (`package:nginx#installed` vs `#version`), not one bit per kind/entity — so this stdin format and `fact_label` move *with* the re-key. ~SUSPECT the line grammar will need a selector field; don't quietly drop a selector on parse and silently widen a verdict to the whole entity (that's a wrong-elision under apply's `kFAIL`).

## Scope boundary (don't build toward these)

The real apply-executor over time, transport/`kCOMMS` (`plans/142`), and multi-host fan-in are **out of spike scope** (`ch-scope`, charter §6). `cli` *compiles* a probe and an apply and runs neither; the host's answers arrive on stdin (a stand-in for running the probe remotely). The only executor extension in scope is the thin backward / `dorc bump` apply-3 skeleton (`ch-scope`, `an-apply-3`) — and that mostly lands in `plan`; `cli` just needs a flag/path to drive it. Keep the binary a thin driver: arg-parse, file-read, call the kernel, print. Resist absorbing pipeline logic.

## Tension to surface, not resolve

`ap-2`-runnability (`sh -n`) and the existing **text golden-diff** are in mild tension, and which is canonical isn't obviously settled. `sh -n` proves the artifact *parses* but not that it elided the *right* lines (a `render_apply` that comments out *everything* is `sh -n`-clean and useless); the text golden catches wrong-elision content but is structurally blind to non-runnable output (the `ap-2` defect). --WONDER whether the honest harness needs *both* — a runnability gate (catches `ap-2`) plus a content assertion that isn't a brittle full-text diff (e.g. assert specific leaves are/aren't commented, or execute the apply against a fixture and check observable effects per `an-render-executability-check`). Flagging rather than picking: the charter mandates the runnability gate (`ap-2`), but silently deleting the content check to get there would re-open the wrong-elision blind spot from the other side.
