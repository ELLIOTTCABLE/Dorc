# spike/crates/cli — CLAUDE.md

Role: the round-trip driver (book + oracles → read-only probe → results → eliding
apply) and the e2e acceptance harness's contract. The ONE place determinism is
relaxed — real I/O at the edges only. Read `spike/CLAUDE.md` first. Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law

- **io-at-edges-only** — keep I/O in `run()`/`main`; the pipeline
  (`parse → cfg → classify → compile_probe/build_plan`) stays a total
  `Carrier<T>` function of its inputs; never let a clock/RNG/env-read leak
  inward "to help".
- **stdout-contract** — stdout is EXACTLY probe-then-apply (split on shebangs);
  diagnostics go to stderr only — anything else breaks the e2e capture and any
  real downstream pipe.
- **probe-ships-oracle-bytes-only** — the compiled probe is synthesized
  scaffolding + oracle bodies, never book contents (it never inherits the
  book's `trap`s). ⚠ HEAD DEBT: the landed `24J` pipe-lift raw-ships book
  bytes — standing-law debt; read `24J`'s header correction and
  `271:rul-only-oracle-bytes-ship` (+ its build riders) before touching probe
  emission. Never imitate the landed shape.
- **results-fold-to-run** — a missing or unparseable fact folds to
  `Verdict::Unknown` ⇒ run (`kFAIL-perform`); keep that default, it is
  load-bearing. Never silently drop a selector on parse and widen a verdict to
  the whole entity — that is a wrong-elision under apply's fail-direction.
- **speculate-and-intercept** — the probe model resolves probe-gated branches by
  running the read-only check for real (oracles intercept; not Ansible
  check-mode blindness).

## The acceptance harness (`spike/e2e/run.sh`; sh-mechanized; this contract is law)

- **per-case gates** — `dash -n` on BOTH rendered artifacts (the load-bearing
  runnability gate; the historical trap was a text-only golden diff shipping a
  non-runnable empty `then`-clause green, twice) · exec-under-mocks for cases
  with a `mocks/` dir (sorted run-set asserted against `expected.ran`, which
  MUST exist — missing ⇒ loud fail, never empty-want) · crash/empty guard
  (dorc rc≠0 or empty output hard-fails before the xfail lens and before
  bless) · the content golden-diff as a secondary check (catches wrong-elision
  CONTENT, to which `-n` is blind) · XFAIL/XPASS pin machinery (XFAIL is
  golden-text-BLIND by design — structural gates only; a surprise pass is a
  loud XPASS-to-promote).
- **bless-never-first** — `BLESS=1` regenerates goldens; gates run before bless,
  but bless cannot prove an elision RIGHT: fresh verified binary,
  orchestrator-only, diff inspected case-by-case (BLESS exclusivity —
  `spike/CLAUDE.md`).
- **count-drifts** — the case-count drifts; count the dirs, never trust a
  literal.

## Direction

- **wire-records** — the ad-hoc stdin results format is replaced by the `262`
  §2 records lane at block-rebuild: framing header/sentinel · per-record
  terminal token · coordinate fields last-to-token · partial deriv-family ⇒
  wall-total · value stdout carries arbitrary single-line bytes (embedded
  spaces survive round-trip — `279f` rider).
- **probe-projection-second-caller** — the probe plan-builder is the only real
  SECOND phased caller of `inv-superposition` (the load-test of "engine emits,
  caller collapses"): build it as a genuine `Phase::Probe` caller; never bake a
  posture into the kernel to make it easier.
- **scope-boundary** — the real apply-executor, transport (`KNOBS:kCOMMS`), and
  multi-host fan-in stay out of spike scope. Keep the binary a thin driver:
  arg-parse, file-read, call the kernel, print. Resist absorbing pipeline
  logic.
