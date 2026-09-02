# 30Xa — the test-infrastructure decruft arc: conductor ledger

> Tier: conductor ledger (Fable, opened 2026-09-02); compression-resistant state for the arc
> that BUILDS `notes/30X`. Ahistorical except §2 (the human's typed record, kept verbatim). The
> design lives in `30X`; this file carries only what conducting the build needs: rulings typed
> since 30X, the tree-reconciliation deltas, the lane state, and the residue accounting.
> Authority: `30X` and everything it cites outrank this file.

## §0 — state (2026-09-02): resumed after the human's adjudication; `30-reviewB` gates lane A

`/opaque-review` `30-reviewA` (pass `initial`, range `3b999e47^..633fd954`) returned, verbatim:
`NACK Research/quarantine-DO-NOT-READ/30-reviewA-opaque-report.md`; the conductor never opened
it. The human adjudicated: the accepted tightening landed in `30X` as `8ab53489` ("Fence fixture
authority without narrowing sessions" — `RealSsh` and `Os` roots out of the ordinary harness's
reach; `HarnessSeams` as the constructor-side subtype; `loom-syntax-grants-no-production-authority`;
livetest as the explicit composition for ambient capability), then acked the conductor's five
clarity repairs and the added `rul-seam-columns-are-conductor-ruled`. The re-stabilized design
goes through `30-reviewB` (pass `initial`; kit `30Xd`) before lane A; ACK continues, anything
else returns to the human.

Successor on-ramp (everything a rewound conductor needs is on this branch): this ledger · `30X` as
amended at `633fd954` (§3a is the new non-loom law) · `30Xb` (the lane A brief, DRAFT, pre-NACK —
re-cut against the human's ruling before dispatch) · `30Xc` (the scout inventory: seat table,
corpus census, plumbing, xfail registry) · `30Xd` (the sealed-review dispatch kit; reuse verbatim
for `30-reviewB` with the identity/pass/range/report lines changed, and forbid `Monitor` in the
relay prompt). The review report itself is at
`C:\Users\ec\Sync\Code\Dorc\.tmp\trees\r30-30X-test-infra-decruft-conductor\Research\quarantine-DO-NOT-READ\30-reviewA-opaque-report.md`
(committed on this branch as `9b915238`); the conductor never opens it. `ai/main` carries two
status-only commits from this sitting (`74da908d`, `17403acd`) that this branch must be rebased
over before any fold.

## §1 — remit and standing rulings

- `remit-30X-is-the-floor` — build every 30X lane; mild exceedance licensed only toward a
  coherent, maintainable, future-proof corpus; no new questions to the human beyond those 30X
  pre-rules; "leave it as it is" is the default for anything non-obvious.
- `rul-minimize-errorloom-changes` `[TYPED 2026-09-02]` — NACK on deep changes to the
  loom-parsing architecture. `errorloom` may be edited, minimally: additive API only, no rework
  of its section / transcript / replay model. The session driver, gates-by-kind, and the
  frontmatter collapse live in `dorc-loom` and the cli runner. A lane that finds it needs more
  than a small additive errorloom change STOPS and reports.
- `rul-clean-tree-gate-is-out-of-scope` `[TYPED 2026-09-02: "a hard maybe … just about
  out-of-scope"]` — moot in any case: the publish gate is already blast-radius scoped
  (`dorc-loom/src/repository.rs::classify_prose_changes` — locks byte-equal to HEAD, the
  selected cases wholly staged or wholly unstaged, everything else ignored). Only the stale
  CONTRIBUTING sentence is corrected at close.
- `rul-non-loom-licensing` `[TYPED 2026-09-02]` — recorded as `30X` §3a: all prose is loomed
  (primacy); a non-loom test is licensed only by one of seven closed classes and asserts state,
  exits, structure or relations; every loom-escape impulse STOPS and is raised to at least a
  conductor; at most ONE new handbuilt test mechanism per arc. Propagates into every brief.
- `30X` §11's lane law stands: no cruft, no half-work; the only legal deferral is
  kernel-mutating and lands under `30X:front-dogfood-ceiling`.

## §2 — ack-ledger (only what the human TYPED counts)

| item | state |
|---|---|
| the plan (lanes A–D, checkpoints after A and C, the exceedances, serial builders in the conductor worktree) | ACKED 2026-09-02 |
| `ask-opaque-review-before-build` | RULED 2026-09-02: the stabilized plan clears `/opaque-review` BEFORE the first build; an ACK continues, anything else returns to the human. OUTCOME: `30-reviewA` → NACK (§0); held |
| `exceed-convert-dir-cases-to-looms` (109 cli dir cases → looms; one-off converter deleted after use; lane D tail) | ACKED 2026-09-02 ("no legacy e2e") |
| `amend-shape-c-assertion-rule` (§4) | ACKED 2026-09-02 in sharpened form — `30X` §3a (prose primacy · seven classes · stop-at-the-conductor · one mechanism per arc) |
| the reviewer-driven tightening (`8ab53489`) + the conductor's five clarity repairs + `rul-seam-columns-are-conductor-ruled` | ACKED 2026-09-02 ("Ack; proceed with your cleaning. I believe we're ready to deploy") |
| `ambiguity-persistence-means-what` — applied as "the production ROOT is excluded, never native I/O under a runner-owned root" (the only reading lane B survives) | applied under that ack; the human corrects in-chat if the other reading was meant |
| `exceed-silence-doctest-noise` | acked with the plan 2026-09-02 |
| `exceed-port-or-delete-yardstick` | acked with the plan 2026-09-02 |
| `note-stale-review-repair-worktree` (`.claude/worktrees/r30-review-repair1`, tip inside `ai/main`; not this arc's) | reported; untouched |

## §3 — tree reconciliation (2026-09-02; `ai/main` at "Tune conductor's usage of subagents and worktrees")

`30X` §11's ground truth holds, with these deltas:

- `delta-six-env-pins-not-three` — the shipped `dorc` reads SIX harness-shaped variables:
  `DORC_FIXTURE_CLOCK_MS`, `DORC_STDOUT_POSTURE`, `DORC_FIXTURE_SOURCE_MATCH`,
  `DORC_FIXTURE_NONCE` (`cli/src/transport_edge.rs`), and the debug-only `DORC_TRANSPORT` /
  `DORC_TRANSPORT_INTERPRETER`. Lane A retires all six under
  `30X:inv-fixture-state-never-typeable-into-main`; transport is the seam column 30X already
  lists. No dependency crate reads any environment; `dorc-sh` reads none; the remaining reads
  are platform roots (`APPDATA`, `HOME`, `XDG_*` via `RootEnvironment`) and `PATH`/`PATHEXT`.
- `delta-model-io-is-already-public` — `dorc_receipt_local::model::ModelIo` is ordinary public
  code (neither `cfg(test)` nor feature-gated; `LocalIo` is sealed but nameable). Lane C's
  boundary question is one new Cargo edge: `dorc-loom` depends on none of `receipt`,
  `receipt-crypto`, `receipt-local`, `why` today.
- `delta-corpus-shape-census` — `cli/tests`: 108 single-file looms + 109 dir round-trip cases
  (106 with `mocks/`; markers in use: `ARTIFACT_SET` 7, `XFAIL` 4, `DORC_FLAGS` 13, `tolerate=`
  4, `PROBE_RESULTS=authored` 21, `DUAL_RAIL` 1, `DORC_EXIT` 2, `expected.emitted` 1) + two
  `lint-real-*` dirs (a third, undocumented shape, driven only under `DORC_E2E_REAL_TOOLS`).
  `aid/tests`: 200 looms. `lint/tests`: one `cmd` case. The multi-file `X/X.loom` shape has ZERO
  instances. 109 looms carry `run:` (105 round-trip, 4 lint) and exactly those carry
  `fixpoint: executed`.
- `delta-frontmatter-usage` (24 keys, 311 looms) — `tests-critical-law` 0 · `expect-why-receipt`,
  `apply-exit`, `artifact-set`, `dual-rail`, `todo` 1 each · `tolerate`, `expect-hint` 2 ·
  `probe-results` 4 · `exit`, `why-addr`, `expect-why-chain` 6 · `envelope` 7 · `expect-why` 8 ·
  `flags`, `when-used` 9 · `owns` 30 · `arrangement` 83 · `run`, `fixpoint` 109 · `code`,
  `when-fires` 117 · `why` 127.
- `delta-batteries-verified` — 30X's classification of the seven cli batteries is confirmed,
  including the stale "cannot sign" header in `receipt_route.rs` and `spine_baseline.rs`'s
  build-to-kill Cargo stanza. `recorded_facts_route.rs` already asserts every property the
  needle gate `scan_why_receipt` hardcodes.
- `delta-yardstick-broken` — `mise run yardstick` (`sh e2e/yardstick.sh`) has been known-broken
  since 2026-07-26 and is the one task still spelling `sh` (`task-bodies-are-shell-free`).
- `delta-doctest-noise-source` — `mise run test` runs `cargo test --workspace --doc {{filter}}`
  before nextest; no crate sets `doctest = false`; the `compile_fail` doctest in
  `aid/src/narrative.rs` is load-bearing (`mise.toml` header).
- `delta-hostsim-seed-print` — hostsim surfaces its seed per-assertion, not run-wide; the
  seed-constructing sites outside hostsim are `plan/tests/sparing_differential.rs`,
  `sweep/src/scenario.rs`, and `hostsim/examples/differential.rs`. `30X:seed-two-affordances`
  is the widening.

## §4 — dir cases versus looms (conductor opinion, 2026-09-02; asked for by the human)

The human's instinct: the problem-space may need e2e tests inappropriate for looms AND for unit
tests, so shape (b) should not be deprecated merely because looms exist. Finding: the instinct
holds for the KIND of test, not for the dir SHAPE.

- `fnd-dir-case-is-a-loom-serialized` — a dir round-trip case and a `run: round-trip` loom are
  one test in two serializations: `run_loom` materializes the loom into the dir layout and runs
  the unchanged battery (`cli/CLAUDE.md loom-form-is-the-same-battery`). The dir shape has no
  expressive power the loom lacks, and under 30X the loom has strictly more. What a directory
  offers is serialization ergonomics — a browsable fixture tree, real file modes, bytes txtar
  cannot carry — which are reasons for a RUST battery to own a fixture dir (already licensed by
  `flat-test-tree-and-loom-placement`'s residual clause), never for a second corpus shape with
  its own marker grammar.
- The permanent non-loom residue, each in-tree today:
  - `counter-meta-tests-of-the-runner` — `bless_folds_only_on_pass_selftest`,
    `doctor_never_gains_the_power_to_delete`, the discovery floors, the two fixpoint gates. A
    loom cannot test the loom runner without regress; the observable is "nothing written,
    nonzero exit".
  - `counter-for-all-cases-census` — `region_artifacts.rs`, `definition_frames.rs`,
    `p-zero-munge-happy-corpus`: "for all cases" is not a case (`30X:tier-census`).
  - `counter-seed-swept-invariants` — `plan/tests/sparing_differential.rs`, hostsim sweeps:
    invariants over generated worlds; a loom pins one transcript
    (`30X:seed-exploration-asserts-invariants`).
  - `counter-platform-variant-state` — the `#[cfg(unix)]` keyset-permission asserts (run on
    `/tmp` because drvfs lies) and the Windows-only rename-backup path. One transcript serves
    both legs, and `30X:model-determinism-at-the-source` forbids normalizing after the fact, so
    where the property under test IS the platform difference no single golden exists.
  - `counter-foreign-and-live-output` — the real-tools lane must never golden shellcheck's text;
    livetest matches a baseline shape against a real host. Structural assertions over bytes
    nobody controls.
  - `counter-properties-over-os-seams` — `30X:inv-division-at-the-narrowest-edge`'s own remit
    (the shipped `dorc` really draws OS entropy and a live clock): inequalities and
    cross-invocation identities (`recorded_facts_route.rs` asserts `--all` byte-identity across
    two runs and the `--json` withhold markers). A session CAN spell
    `[ "$a" != "$b" ] && echo differ`, but a loom whose golden is the word `differ` has no prose
    to author and no byte-honest transcript to read — both reasons a loom exists are absent, so
    Rust is the honest home.
  - `counter-timing-and-interleaving` — BLESS exclusivity, the hook self-tests, the SIGPIPE flap
    class under a real shell: interleavings cannot be goldened; hostsim's answer is DST.
- `fnd-accidental-residue-shrinks` — several of today's "needs Rust" cases are artifacts of the
  narrow harness that the session model absorbs: the `expected-empty-stdout` harness gap
  (`cli/CLAUDE.md an-artifact-set-runs-from-its-own-generation`) closes with both-streams
  transcripts; `durable_route.rs`'s render needles become loom goldens once lane C exposes the
  receipt world in-process. Only the list above is essential.
- `amend-shape-c-assertion-rule` (PROPOSED; needs a typed ack) — shape (c)'s "state and exits
  only, never render bytes" is narrower than the tree already needs. The precise line:
  byte-exact render GOLDENS belong to looms (a golden is the prose-authoring surface; nothing
  else can be edited into prose); STRUCTURAL and RELATIONAL assertions over renders (a code slug
  present, a count, an inequality, two invocations byte-identical) belong to Rust, and are legal
  only where the bytes are un-goldenable (`Os` seams, foreign tools, platform variance) — anywhere
  else the case is a loom.
- Recommendation: convert shape (b) to looms (one corpus shape, one materialization path, the
  `NAME=value` marker grammar dies with `run_round_trip`'s dir entry); keep shape (c) under the
  sharpened rule; a Rust battery may own a fixture directory; the two `lint-real-*` dirs become
  the real-tools test's fixture space rather than corpus cases.

## §5 — lane plan and state

Worktree `.tmp/trees/r30-30X-test-infra-decruft-conductor`, branch
`ai/r30-30X-test-infra-decruft-conductor` from `ai/main`. One Opus builder per lane, serial,
working IN this worktree on this branch (no merges); conductor edits are committed between
lanes only, never while a lane is live; every lane ends green on `mise run both
gate:full-quiet`; `gate:arc` runs from this branch before the fold to `ai/main`. Every brief:
the Safety block, step-zero (`pwd` + branch + tip verify, `git -C` everywhere), step-0.5 (`mise
trust` on both legs), `AGENTS.for-builders-only.md` first, step-one reads, the comment budget
with rip-don't-update, the no-subagents clamp, the naming discipline, `rul-minimize-errorloom-changes`.

| lane | scope (`30X` §11 + §3 deltas) | state |
|---|---|---|
| A `lane-a-seams-and-harness-binary` | `Seams` + `HarnessSeams` (`30X` §4) + `HarnessSeams::from_env` on one env-reader footing; the session environment scrubbed; `compose::run(Seams)` extracted from `main.rs`; `bin/dorc-harness.rs` refusing with no seam set; seeded id/key entropy over a dependency-free generator; the ticking harness clock; ALL SIX pins retired from the shipped binary; the e2e runner spawns the harness through a `dorc` shim on PATH; `bless:dry` clean | not dispatched |
| — checkpoint A | `inv-division-at-the-narrowest-edge` judged on the extraction diff | — |
| B `lane-b-session-driver-and-rip` | the shell-session process driver; gates by kind, no position rules; own roots per session; the needle gate ripped whole; `why30-receipt-rooted-surface.loom` as an ordinary multi-block session; both-streams transcripts (every `run:` loom re-blesses — AUTHORIZED); batteries split by assertion kind; `spine_baseline.rs` + `mise run spine:baseline` deleted; `receipt_route.rs` header corrected | not dispatched |
| C `lane-c-in-process-receipt-world` | the in-process driver composes the real `LocalReceiptEdgeV1` over `ModelIo` (the new Cargo edge) with seeded entropy and the ticking case clock; the 37 `why-total-*` rows authorable; varied-seed default + the two affordances; `gate-two-drivers-agree`; the post-dispatch durable report authored over that world, witnessed by state-only e2es | not dispatched |
| — checkpoint C | the durable-failure diagnostic's shape (sibling code vs reason arm) is the conductor's product ruling | — |
| D `lane-d-one-runner-and-frontmatter-collapse` | one runner; the driver derived and reported; `run:`/`fixpoint:` retired; frontmatter 24→9 (`tests-critical-law` has zero uses — drop unless `vocabulary.rs` reserves it for a reason); hk/mise/bless plumbing follows; the dir-case → loom conversion (ACKED: a one-off converter deleted after use; the round-trip runner's dir entry and its marker grammar die); `lint-real-*` re-homed as the real-tools test's fixture space; the doctest noise; yardstick | not dispatched |

## §6 — residue accounting (empty, or kernel-only under `30X:front-dogfood-ceiling`, at close)

- none yet.

## §7 — steering and register edits owed at close (conductor voice, once)

Per `30X` §11, plus what this arc learned: `crates/cli/CLAUDE.md` (the harness contract re-cut
around sessions, gates-by-kind, the derived driver, the narrowest-edge invariant, the
apply-host driving route) · `spike/CLAUDE.md` (`rul-fixture-identity-never-production`'s
public-interfaces reading; the Safety block's "central e2e runner" sentence; the
`flat-test-tree-and-loom-placement` shape list losing `X/X.loom` and, if converted, the dir
shape; the Build/test/run task table) · `crates/aid/CLAUDE.md` (runner pointers; the
`seam-tolerated-nondeterminism` spelling) · `plans/282` §2/§7 (in-place correction of the two
superseded clauses) · `CONTRIBUTING.md` (the stale clean-worktree paragraph; the gate
description if the runner merge changes it) · `LIVING_STATUS.md` + the `Research/README.md`
DST topic row · `TODO-ADDTL` `why-surface-close-residue` (the migrated batteries) · `30X` §3
shape (c) if `amend-shape-c-assertion-rule` is acked.
