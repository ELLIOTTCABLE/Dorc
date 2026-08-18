# 30Ia — The static-loading/bundle-emission conduct ledger

> Conductor ledger for the `Research/plans/30I` build lane. `30I` is the semantic
> authority and the work order; this file is *state* — who is running, what was
> adjudicated, what is owed. Crash-resistant by design: compress or delete rows
> that stop mattering. Nothing important may live ONLY here.

## §1 — The seat

- Conductor: Opus-class, session opened 2026-08-18, dispatched from the primary
  checkout; conductor worktree `.claude/worktrees/r30-loading`, branch
  `ai/r30-static-loading`, cut from `ai/r30-loader-bundle-design` @ `80faf71d`
  ("(AI dsn) Turn static loading into a one-builder dispatch").
- Remit, as given: implement `30I`. The top implementation section's *phrasing*
  is conductor-tunable; the product spec is fixed, and difficulty in reaching it
  is a stop-and-report.
- Sibling lanes, untouched: `ai/r30-conduct` (kernel wave-two),
  `ai/r30-loom-surface` (loom authoring). `ai/r30-loader-bundle-design` is the
  lineage this branch folds back to.

## §2 — Baseline

`mise run gate:quick-quiet` measured GREEN by the conductor's own hand in this
exact worktree at `80faf71d`: 1354/1354, 0 skipped. Any red a builder reports is
theirs, not inherited.

## §3 — Dispatch decision (conductor call, 2026-08-18)

`dec-dispatch-one-builder-not-in-window` — the human's mild lean was in-window
work, conditioned on "if you think it can all fit without context-collapse". The
call went the other way, on scale measured rather than assumed:

- the lane's seats span `plan` (25k LOC), `cli` (23k), `oracle` (20k),
  `analysis` (17k), `aid` (15k), `core` (6k);
- three of its seven work items are NEW subsystems, not edits: the bundle
  projection, the multipart artifact set with atomic publication, and the
  generalized locator DAG. Scouted: today's artifact is stdout-only, and no
  bundle or artifact-directory machinery exists anywhere in the tree;
- iterative compile/gate churn on a lane this wide is exactly the traffic that
  costs a WINDOW rather than a budget, and a collapsed conductor cannot
  adjudicate the deviations the lane is certain to produce.

`30I:impl-one-builder-one-lane` is typed, so the split is one builder end-to-end,
NOT phased sub-lanes; the conductor seat stays clean for the fold. Brief at
`_tmp-30I-brief.md` (untracked, worktree-local, regenerable from `30I` + §4).

## §4 — Conductor scouting, banked (the four crossings handed to the builder)

Measured in-tree at `80faf71d`. These are the value the conductor seat added over
handing `30I` across bare; if a successor re-derives them, that is wasted time.

1. `fnd-30G-b8-gap-is-closed-by-replacement` — `30G` §4 item 8 STOPPED believing
   the only exits were "keep requiring the sourced tree" or "neutralise a book
   `.` line, which is a `rul-attention-honesty` ruling no component may make".
   `30I` §7.1/§7.3 rules a third that `30G` did not have: the book `.` naming a
   contracted dorc-lang root is REPLACED at its own source position by a `.` of
   the generated bundle — the ordinary replace mechanism, with `plan.sh` visibly
   naming every bundle at its original source point. Attention-honesty holds by
   construction, not by exception. `FORFEITS:forfeit-book-sourcing-walls` stops
   binding on the dorc-lang half; the non-dorc-lang half stays walled and falls
   to `30I` §7.1 mode 3.
2. `fnd-command-v-walls-and-loads-on-one-line` — the same construct answers two
   questions and neither answer may contaminate the other. `command` is an
   explicitly punted oracle, so `FORFEITS:forfeit-command-v-poison-wall` is
   half-irrelevant to the LOADER-ANALYSIS machinery — a `command -v` at marked-file
   top level feeding an include guard is load-time control flow the loader
   evaluates (`30I:force-guarded-fallback`). But it remains TRUE that a
   BOOK-position `command -v` walls, preventing elision on all content under that
   command's branch [human-typed correction, 2026-08-18]; `load30-subshell-errexit-fallback`'s
   book spells exactly that. The wall is correct and `silence-licenses-nothing`
   forbids un-walling it; needing to un-wall `command` to green a specimen is a
   licensure widening routed to the human (that forfeit's CAPTURE row), never lane
   work. Specimens are safe either way: `expected.ran` asserts run SETS, and a
   walled line still runs. `load30-two-point-frames` is the loader-side
   discriminator — one guard must answer FALSE ambiently and TRUE inside the
   subshell that pre-defines `sm_pick`.
3. `fnd-e2e-analysis-cwd-is-unresolved` — the specimens spell
   `SM_ORACLE_ROOT=crates/cli/tests/load30-…`, relative to `spike/`; the harness's
   `dorc` invocation (`e2e.rs` `fn dorc`) sets no `current_dir` and so inherits
   Cargo's PACKAGE-root cwd (`spike/crates/cli`); the execution rail deliberately
   runs artifacts from a throwaway sandbox. Under `30I:rul-dot-resolves-as-sh` the
   analysis cwd and the execution sandbox cwd are separate modeled questions.
   Absolute roots are not an escape — transcripts may not carry machine-specific
   absolute paths (`282` §7). The builder owes a stated pin and its reason.
4. `fnd-sourcing-seat-is-pre-factored` — `spike/crates/cli/src/sourcing.rs`
   names `resolve_against` in its own doc-comment as the single function to change
   for sh parity, and argues the sourcing-file-relative case at length. `30I` §3.2
   rejects that argument; the rule and its test are removed rather than kept for
   compatibility with an unreleased mistake, and the module doc is rewritten to
   current truth (`plans-current-tense-only`, no supersession annotation). The
   doc's own measured objection — that `28M` §7's helpers-plus-thin-entrypoints
   package becomes unreachable under cwd parity unless a cwd is pinned — IS
   crossing 3, now owned rather than routed around.

## §4a — The handoff authorization [human-typed, 2026-08-18]

`dec-one-builder-order-softened-to-a-seam` — the human read the brief, judged one
context-window unlikely to carry the lane, and AUTHORIZED (not dictated) a
mid-lane handoff: builder 1 targets a completion-of-work and a durable product;
builder 2 boots from that product and closes.

What is relaxed and what is not. `30I:impl-one-builder-one-lane`'s INTENT still
binds absolutely — no second resolver, no second source-order model, no
decorative source map. What is relaxed is only the claim that one window must
carry it. The seam is placed to serve the intent rather than to fight it: a
successor consuming a typed, documented load answer is LESS likely to re-resolve
than one exhausted builder improvising an emitter at the end of its window.

The seam, as issued to builder 1:

- FLOOR — never before work-order items 2–4 are landed, natively tested, and
  committed (one static snapshot; the healthy-library surface; the locator DAG
  generalized). Below that the load model is not yet one model and a successor
  would be guessing at it.
- PREFERENCE — also item 5 (the bundle projection existing, consumed by at least
  one entrypoint). The cleaner semantic cut: everything
  `rul-one-loader-many-projections` governs is then behind the seam, and the
  successor adds only PLACEMENT.
- CEILING — a handing-off builder does NOT start item 7. Promotion, e2e lowering,
  and golden-drift enumeration are atomic and belong to whoever closes the lane;
  half-done is worse than not-started.
- TRIGGER — declare at the nearest coherent boundary past roughly 60–70% context,
  never at exhaustion.

Handoff artifact: `Research/notes/30Ib-static-loading-lane-report.md`, maintained
LIVE from the first slice and committed with it — as-built seats and their
consumption API, what is pinned where, deviations left OPEN, open questions, exact
next steps. Not a re-plan of `30I`, not a chronology.

Conductor's read, for the record: ~70% that one builder cannot close cleanly. The
volume sits in items 6–7 (a new artifact subsystem in a 23k-LOC crate whose only
artifact surface is stdout today, then golden churn across a corpus whose quick
tier alone is 1354 tests, twice, on two platforms). The discipline costs builder 1
almost nothing if it turns out unnecessary. Its one real cost is a successor
re-paying onboarding — which is exactly what `30Ib` is for.

## §5 — Live state

- **Builder lane `lane-30I-static-loading`: DISPATCHED 2026-08-18.** One
  Opus-class builder, working in this worktree on `ai/r30-static-loading`,
  committing granularly. Its brief forbids sub-spawning, `CLAUDE.md` edits, and
  self-endorsed deviations.
- Conductor hands are OFF this worktree while the lane runs (shared `target/`).

## §6 — Owed at the fold

- Adjudicate every disclosed deviation as an OPEN item, re-derived from the
  global picture — never as a resolved footnote (conductor-skill law).
- Conductor's own-hand `mise run both gate:full-quiet` over the folded tip
  (never-vouch: a builder's green is reported, not borrowed).
- Steering prose: the builder PROPOSES `CLAUDE.md` text; the conductor judges
  whether it earns a permanent seat in every future context, and writes it once.
- `LIVING_STATUS` CURRENT STATE block; `FORFEITS` rows that this lane discharges
  or rewrites (at minimum `forfeit-book-sourcing-walls`); `30I` §14 pin status.
- The human queue items `30I` does NOT close stay open — `LIVING_STATUS`'s
  2026-08-17 queue minus items 2 and 3.
