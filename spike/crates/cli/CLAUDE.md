# spike/crates/cli — CLAUDE.md

Role: the round-trip driver (book + oracles → read-only probe → results → eliding
apply) and the e2e acceptance harness's contract. The ONE place determinism is
relaxed — real I/O at the edges only. Read `spike/CLAUDE.md` first. Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law

- **lib-target-is-a-loom-seam** (`289:rul-worldless-route-honest-trigger`; widened at the W4
  drifted-driver fold, `28H`; the loom-final arc moved SIX regions across it — `28L`) — the lib
  target is the INTERNAL invocation-and-render surface: usage text, `Args`/`LintArgs`/`Mode`, the
  parsers, `humane_read_error`, the drifted-why seat, and the extracted modules `why.rs`
  (`WhyReport` + `why_report_parts`) · `world.rs` (`WhyWorld::analyze`/`analyze_measured` +
  the shared ship-body helpers) · `kinds.rs` (resolver/reaches) · `survival.rs` (footprints/
  wrapped-analysis/carry) · `results.rs` (the intake segment: scope types, `parse_admitted_results`,
  `admit_controller_records` vs the fenced `admit_fixture_records`) · `fixpoint.rs` (validity
  rounds + cascade attribution). One implementation — `main.rs` keeps call sites and every I/O
  edge (clock readers, git, terminal width). It exists so `dorc-loom` can drive REAL invocations
  in-process; it is NEVER a public API — `publish = false`, nothing outside `dorc-loom` and the
  two bins may depend on it. VALUES cross the seam, QUERIES do not. If something in the lib
  starts wanting a clock, a file, or an env read, it is on the wrong side of the seam.
  (`RunClock::Absent` on every loom path — a committed transcript must be a fixpoint.)
- **invocation-errors-are-registry-codes** (`288` §6) — the parsers return typed `Diag`s, never
  strings. The `dorc: ` / `dorc: lint: ` / `dorc-sh: ` prefixes and the usage synopsis are print-seat
  CHROME the three report seats own, never catalog prose. Exit codes are unchanged and never read
  severity. A new invocation error mints a code + a defining case like any other surface.
- **chrome-comes-from-the-registry** (`289:rul-arrangement-home-is-registry-plus-transcripts`) —
  the help page and the seat-appended usage synopsis are arrangement-registry entries
  (`dorc_cli::help_text` / `usage_text`), not consts; their words are edited through
  `crates/aid/tests/cli-help-page.loom`, never in source. A new user-facing chrome string mints a
  registry entry, not a `const`.
- **io-at-edges-only** — keep I/O in `run()`/`main`; the pipeline
  (`parse → cfg → classify → compile_probe/build_plan`) stays a total
  `Carrier<T>` function of its inputs; never let a clock/RNG/env-read leak
  inward "to help".
- **stdout-contract** — stdout is EXACTLY probe-then-apply (split on shebangs);
  diagnostics go to stderr only — anything else breaks the e2e capture and any
  real downstream pipe.
- **probe-ships-oracle-bytes-only** — the compiled probe is synthesized
  scaffolding + oracle bodies, never book contents (it never inherits the
  book's `trap`s). The `24J` raw-ship debt is REPAIRED and machine-pinned
  (probe-render tests assert the raw book site cannot appear in emitted bytes);
  their failure means it returned.
- **results-fold-to-run** — a missing or unparseable FACT folds to
  `Verdict::Unknown` ⇒ run (`kFAIL-perform`); keep that default, it is
  load-bearing. Never silently drop a selector on parse and widen a verdict to
  the whole entity — that is a wrong-elision under apply's fail-direction.
- **admission-precedes-the-fold** — the fold-to-run default answers a MISSING
  FACT, never a broken CHANNEL. Bytes reach the fold only through the bounded
  intake, whose outcome is three-way: `Admitted` proceeds; `NoObservation`
  (well-owned attempt, no usable fact) takes the fold above; `Refused` (framing,
  bounds, attribution, or integrity failure) returns BEFORE plan construction,
  artifact rendering, or whylog writing, and emits no plan carrying mutation
  authority. Never collapse `Refused` into `Unknown` and continue — "run
  everything" is the safe answer to not knowing the WORLD, and the wrong answer
  to not knowing whether we are still talking to the world we think we are
  (`rul-integrity-failure-withholds-mutation`).
- **attribution-is-controller-minted** — the run's framing/scope is minted HERE,
  at this edge, from controller-owned values; an incoming payload frame is
  CHECKED against it and never mints it. The width-one scope types are private
  and deliberately unshared. When a second scope first becomes representable —
  real transport, concurrency, retry, cross-host reuse, saved approval — carrying
  the scope has to become checking it, and this is the choke point where that
  lands.
- **the-fixpoint-owns-the-rounds-and-builds-nothing-else** (`26H` §4/§4¾ — W-C) —
  `settle_validity_fixpoint` re-derives classify + the records fold against the residual model
  until a round proves no further branch dead. Three things bind. FROZEN: book/CFG/value-flow, the
  ADMITTED records (admission runs ONCE, before the loop — no re-probe, no re-admission), the
  vouches, and the compiled probe. Probe EMISSION is untouched because there is exactly ONE
  `ProbePlan`, built from round 1 and never rebuilt; what moves per round is a validity VIEW over
  it, and nothing else about the record intake moves — an erased site KEEPS contributing its
  measurement, because the deadness of the line that measured the world does not un-measure the
  world. NEVER-SURVIVES: intermediate rounds are unobservable not because they are discarded but
  because they are never built — the loop body constructs a classification and a fold, and every
  plan, narrative, render, whylog write, and `report_at` sits outside it. The sole deliberate
  exception is the round-tagged derivation link (`attribute_cascades`), durable so `dorc why` can
  answer a cascaded elision; that is a HARD requirement, not polish. Cap-hit is unreachable
  (erasure is monotone, bound = site count) and DISCARDS the ledger to re-derive from origin
  rather than ship a partial fixpoint, so the degraded answer is exactly the pre-W-C one —
  `solve`'s own unenforceable-termination bargain, `debug_assert`-loud in dev and under DST.
- **speculate-and-intercept** — the probe model resolves probe-gated branches by
  running the read-only check for real (oracles intercept; not Ansible
  check-mode blindness).

## The acceptance harness (`tests/e2e.rs` + `tests/looms.rs`; this contract is law)

- **runners-live-here-cases-are-peers** (`288:phase-flat-tree-move`) — this crate owns the
  two central `harness = false` runners and the round-trip / lint case collections that sit
  beside them in `tests/`. `sh e2e/run.sh` is RETIRED; every gate below moved into
  `tests/e2e.rs` unchanged, and `cargo test --workspace` now runs the corpus. `autotests =
  false` + explicit `[[test]]` targets is what lets case DATA share the dir with the
  runners; the shape rules that classify a dir are `spike/CLAUDE.md`'s
  flat-test-tree-and-loom-placement.
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
- **loom-form-is-the-same-battery** (`288:phase-e2e-loom-conversion`) — a whole-product case may be
  a single-file `.loom`: frontmatter instead of `NAME=value` markers, txtar sections instead of a
  fixture dir (`mocks/` included, dotfiles included, `expected.ran` as a byte section), and the
  committed transcript instead of `expected.out`. It is NOT a second harness — `run_loom`
  MATERIALIZES the case into exactly the dir shape and runs the unchanged gate battery over it, so
  a conversion cannot quietly drop a check. TWO closed key vocabularies exist, and an unread key
  is refused in both: the e2e runner's `LOOM_KEYS` (run-lane keys + `owns`, listed there because
  ownership is corpus-wide even though no e2e gate reads it) and the looms runner's
  `FRONTMATTER_KEYS` (the full ~22-key set: `code`/`arrangement`/`owns`/`edit-loop`/`envelope`/
  `tolerate`/`run`/`fixpoint`/…). A new key joins the vocabulary in the same commit that mints it,
  or its cases go red. The replay COMMAND is compared against the invocation the runner actually
  drives, so a transcript can never show one command while the gates run another. Corpus walks
  skip `*.sync-conflict-*` (sync residue is never a case).
- **one-fixpoint-authority-per-case** — `crates/cli/tests/looms.rs` render-fixpoints every committed
  loom through the in-process consumer; a whole-product loom declares `fixpoint: executed` instead,
  because its transcript is proven by running the REAL binary here (the stricter proof, and the only
  one the sanctioned-executor law allows for a case that materializes mocks). `fixpoint: executed`
  without a `run:` key is refused in the looms runner — otherwise the transcript is proven by
  nothing. The old 4-case `DIRECT_PLAN_CASES` gate in `dorc-loom` is GONE
  (`289:rider-fixpoint-gate-rationalize`); do not re-mint a second render-fixpoint authority.
- **tolerate-is-a-closed-vocabulary** (`288:prop-normalizer-closed-vocabulary`) — a case DECLARES the
  named nondeterminism class it tolerates (`tolerate=<class>` marker / `tolerate:` frontmatter) from
  an engine-owned vocabulary, and the named normalizer is applied to the CAPTURE at bless AND at
  check, so the committed bytes are the canonical form. Never a free regex; never a check-only
  relaxation (the retired `RAN_ORDER=lax` shape blessed raw bytes and compared sorted ones, so the
  committed file recorded an interleaving nothing asserted). Current vocabulary: `pipe-stage-order`.
- **needles-are-structural** (`288:prop-structural-needles-only`) — `expected-diagnostics` /
  `expect-diagnostic:` is a list of code SLUGS; the `error[<slug>]` needle is DERIVED and every slug
  is validated against the generated catalog, so a dead slug is REFUSED and a declaration is an
  ASSERTION (a declared-but-unfired code is red). The why/hint/why-chain needles stay free text —
  legal, rare, and carrying real semantic content rather than catalog prose.
- **count-drifts** — the case-count drifts; count the dirs, never trust a
  literal. The runners pin only a NON-EMPTY discovery floor (a zero-trial suite would
  exit green — the one failure their own path constants can cause and not report).
  RESIDUAL, unchanged from the sh harness: deleting ONE case dir shrinks the suite
  silently. That deletion is visible in the diff; a broken root is not, which is why the
  floor guards the root and nothing guards the count.

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
