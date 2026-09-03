# 30Xa — the test-infrastructure decruft arc: conductor ledger

> Tier: conductor ledger (Fable, opened 2026-09-02); compression-resistant state for the arc
> that BUILDS `notes/30X`. Ahistorical except §2 (the human's typed record, kept verbatim). The
> design lives in `30X`; this file carries only what conducting the build needs: rulings typed
> since 30X, the tree-reconciliation deltas, the lane state, and the residue accounting.
> Authority: `30X` and everything it cites outrank this file.

## §0 — state (2026-09-03, the second sitting): lanes A, B1, B2a, B2b BUILT and green; lane C1 executing from its map; a rewound successor resumes HERE

Branch `ai/r30-30X-test-infra-decruft-conductor`; worktree
`C:\Users\ec\Sync\Code\Dorc\.tmp\trees\r30-30X-test-infra-decruft-conductor`. The branch RIDES
`ai/main`: rebased 2026-09-03 before lane C rather than at close, so the slug-index and spelling
hooks that landed on `ai/main` govern the remaining lanes as they build (the slug index regenerates
as one `(- AI tool)` commit at each rebase; `mise run both gate:full-quiet` was green at the
rebase); it is re-rebased in every between-lanes gap in which `ai/main` has moved, never while a
lane is live. Built: the `Seams`/`HarnessSeams` bundle and `dorc-harness` (A, A′); the
shell-session process driver with gates by kind, both-streams transcripts, the ticking per-block
clock default under `rul-runner-varies-only-what-it-set`, the why-lens relativization (B1, B1′);
the needle gate and the baseline scaffold gone, `why30-receipt-rooted-surface` a four-block
session, the sibling-oracle advisory reconciling by canonical key (B2a); the receipt batteries
split into `receipt_state.rs` — the ONE state-only home, twenty tests, the shipped-binary liveness
witness among them — and two pipeline-tier residues (B2b). Every checkpoint ruling is in §2a; the
lane scopes a successor briefs from, including the C1/C2/C3/D split, are in §5; the steering edits
owed at close are in §7. Briefs are never committed (human ruling): a successor writes each lane's
brief into its own scratchpad from §5's scope + §2a's rulings + `spike/CLAUDE.md`'s spawning law
(the Safety block verbatim; step zero naming the tip; step one with
`AGENTS.for-builders-only.md` FIRST; the no-subagents clamp; the comment budget with its counting
command; the report shape), and hands it by absolute path in the dispatch message. Sizing: keep a
lane well under one Opus context — earlier lanes ran to the edge at ~830k tokens; ~500k remits
finished cleanly. Builders serial, in this worktree; the conductor touches nothing here while a
lane is live.

Open, the human's: the re-ack of lane A's five fence re-targets (§2a); the veto window on the two
product rulings taken inside this suite arc, `rul-why-lens-relativizes-under-the-load-cwd` and
`rul-sibling-oracle-scan-reconciles-by-canonical-key` (both §2a, both built); the `dorc-sh` bare-`sh`
finding's roadmap placement (§6). Fold procedure at arc close: rebase this branch over `ai/main`
(the human rewrites `ai/main` under conductor status commits — expect new hashes), `mise run
gate:arc` from the populated branch BEFORE folding, the §7 steering edits in conductor voice, then
`git merge --ff-only`; delete the `LIVING_STATUS` entry (its account moves to the `Research/README.md`
round map) and reap this worktree and branch only after `merge-base --is-ancestor` proves
containment.

### How the design got here (the sitting's review trail)

`/opaque-review` `30-reviewA` (pass `initial`, range `3b999e47^..633fd954`) returned, verbatim:
`NACK Research/quarantine-DO-NOT-READ/30-reviewA-opaque-report.md`; the conductor never opened
it. The human adjudicated: the accepted tightening landed in `30X` as `8ab53489` ("Fence fixture
authority without narrowing sessions" — `RealSsh` and `Os` roots out of the ordinary harness's
reach; `HarnessSeams` as the constructor-side subtype; `loom-syntax-grants-no-production-authority`;
livetest as the explicit composition for ambient capability), then acked the conductor's five
clarity repairs and the added `rul-seam-columns-are-conductor-ruled`. `30-reviewB` (pass `initial`, range
`3b999e47^..914c49a8`, the same relay kit) returned `ACK` verbatim; its report is committed as `653bddb5`.
Standing fence, human-typed the same day: NO sealed-review dispatch of any kind without the
human's typed ack (`LIVING_STATUS` conduct fences). Lanes were then dispatched serially from
this branch's tip with uncommitted briefs.

Successor on-ramp (everything a rewound conductor needs is on this branch): this ledger and `30X`
as amended (§3a is the non-loom law; §2/§4/§5 carry the post-review tightening). Builder briefs,
the scout inventory, and the sealed-review dispatch kit are NOT committed (human ruling
2026-09-02: `Research/notes/` holds design work; briefs go to the conductor's scratchpad or the
dispatch message; the rulings they produced live here). The two review reports are on this
branch under `Research/quarantine-DO-NOT-READ/` (`30-reviewA` at `9b915238`, `30-reviewB` at
`653bddb5`); the conductor never opens them. No further sealed review within the arc (human,
2026-09-02); an end-of-arc review is the human's call. `ai/main` carries status-only commits from
this sitting that this branch rebases over before any fold.

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
| `ask-opaque-review-before-build` | RULED 2026-09-02: the stabilized plan clears `/opaque-review` BEFORE the first build; an ACK continues, anything else returns to the human. OUTCOME: `30-reviewA` → NACK, adjudicated by the human; `30-reviewB` → ACK (§0) |
| `exceed-convert-dir-cases-to-looms` (109 cli dir cases → looms; one-off converter deleted after use; lane D tail) | ACKED 2026-09-02 ("no legacy e2e") |
| `amend-shape-c-assertion-rule` (§4) | ACKED 2026-09-02 in sharpened form — `30X` §3a (prose primacy · seven classes · stop-at-the-conductor · one mechanism per arc) |
| the reviewer-driven tightening (`8ab53489`) + the conductor's five clarity repairs + `rul-seam-columns-are-conductor-ruled` | ACKED 2026-09-02 ("Ack; proceed with your cleaning. I believe we're ready to deploy") |
| `ambiguity-persistence-means-what` — applied as "the production ROOT is excluded, never native I/O under a runner-owned root" (the only reading lane B survives) | applied under that ack; the human corrects in-chat if the other reading was meant |
| the five fence re-targets (`main.rs` → `compose.rs`; §2a last row) | awaiting the human's re-ack |
| sealed review over lane A's implementation (builder-flow relay, §2a) | DECLINED by the human 2026-09-02: no opaque review within the arc; end-of-arc is their call |
| `exceed-silence-doctest-noise` | acked with the plan 2026-09-02 |
| `exceed-port-or-delete-yardstick` | acked with the plan 2026-09-02 |
| `note-stale-review-repair-worktree` (`.claude/worktrees/r30-review-repair1`, tip inside `ai/main`; not this arc's) | reported; untouched |

## §2a — checkpoint A rulings (2026-09-02; the builder's report is the source, each deviation re-derived)

Lane A is BUILT to `4ee6aeca` (six commits; Windows `gate:full-quiet` green; the WSL leg and
`bless:dry` unrun; the builder hit its budget and was stopped). Rulings:

| deviation (builder's slug) | ruling | the conductor's own mistake, named |
|---|---|---|
| `dev-per-block-SEED-not-per-block-CLOCK` — shared clock base, per-block seed offset, to keep `durable-receipt-ambiguous` demonstrable | ACCEPT as lane-A interim. Lane B re-authors that case as a session that PINS the clock across its two publishes (`$ export DORC_SEAM_CLOCK=pinned:…`) with per-block seeds, and the runner default becomes the ticking per-block clock `30X` §11 intends. | the brief never censused `--receipt-last` dependents; the scout was not asked. |
| `dev-clock-absent-value` — `ClockSeam::Absent` | ACCEPT: a product clock state (`RunClock::Absent`), a variant of the existing column, not a column. | the brief copied the clock's implementation set from `30X` without checking the product's own variants. |
| `dev-inspection-drives-use-throwaway-stores` + `dev-per-case-profile-not-suite-wide` | ACCEPT both. The general rule is `inspection-redrives-carry-no-durable` (→ `cli/CLAUDE.md` at close; `30X` §5 rider applied). | `30X` §5 claimed a seeded double-drive is side-effect-free; it is byte-identical in RENDER only. |
| `dev-batteries-switched-and-seeded` — both batteries now spawn the harness | ACCEPT the direction. RIDER: the shape-(c) remit over the SHIPPED binary (`30X:inv-division-at-the-narrowest-edge`: the OS implementations are live) keeps ONE witness — the completion pass added it. | the brief did not say which battery keeps the shipped-binary witness. |
| `dev-roots-nominal-fence` — `Pinned` resolves through the platform variables | REJECT. `rul-roots-pinned-is-a-literal`: `Pinned` carries an absolute runner-owned path and never consults `APPDATA`/`HOME`/`XDG_*`; a nominal fence is no fence once the scrub is absent. Built in the completion pass. | the brief said "keep the platform-variable route" (from `30X` §11) without saying the pinned variant must itself be a literal. |
| `dev-scrub-not-applied` | INCOMPLETE, not a deviation; built in the completion pass. | the remit was sized to the builder's whole budget; the human's steer: size remits slightly smaller. |
| direct spawn of `dorc-harness`, no PATH shim | fine; lane B adds the shim with the shell session. | — |
| `tc-durable-receipt-ambiguity-needs-session-seams` | ruled with the first row. | — |
| five production-source fences re-pointed `main.rs` → `compose.rs` | maintenance under `lexical-fences-are-human-ack-instruments`; listed for the human's re-ack in §2. | — |

The builders-only relay asked for a sealed review over the lane; the human ruled no review within
the arc (2026-09-02).

Completion pass (`413e2e49`; both gate legs green, the WSL leg's first run over lane A;
`bless:dry` clean; the three whygallery transcripts byte-stable): four open items, all
ACCEPTED — an absent `DORC_SEAM_ROOTS` refuses (the runner always supplies it; a silent default
would contradict `30X:loom-seams-are-sh-lines`); the scrub forwards the runner's `PATH` until
lane B1 substitutes the shim/mocks path; the receipt gate re-pins its own root; the seam unit
tests track the parser. Measured: on Windows the harness needs none of `SystemRoot`/`ComSpec`/
`PATHEXT` (seeded entropy, no `cmd`-hosted child, absolute-path launch). The shipped-binary
witness is `durable_route::the_shipped_binary_draws_live_os_identities_and_ignores_harness_seams`.

### Checkpoint B1 (`10debadd`; the session driver built — one persistent `sh` per round-trip loom, sentinel-framed both-streams capture, gates by kind through the product's arg parser; 101 of 105 round-trip looms re-blessed; four floor looms red under `DORC_KNOWN_BROKEN`; gate legs unrun)

| item | ruling |
|---|---|
| `dev-lint-looms-stay-single-invocation` — the four `run: lint` looms keep `run_lint` over the shipped binary | ACCEPT for B1; lane D folds lint looms into the session driver (`$ dorc lint …` through the harness) so one driver remains. |
| `open-dot-sourced-dependency-paths-leak-into-both-streams` — four floor looms' stderr why-lens prints a `.`-sourced dependency's ABSOLUTE materialization path, invisible until stderr was transcripted | RULED `rul-why-lens-relativizes-under-the-load-cwd`: the live why-lens renders a `.`-sourced dependency's path relative to the load cwd when it lies under it, exactly as `--pre-source` oracle paths already render; canonical keys stay absolute (`need-controller-paths-never-cross-hosts` governs placement, not display). A product display change inside a suite arc, taken because the alternatives are four permanently un-goldenable cases or a forbidden normalizer; FLAGGED to the human for veto. Built in the B1 completion pass. |
| `concept-session-stdin-is-the-framed-stream` · `concept-block-argv-classifier-is-read-only` | accepted as steering concepts (`cli/CLAUDE.md` at close). |
| the runner frames the raw fixture records for the session (the shipped intake is exercised; the nonce rides the seeded `attempt_nonce` seam) and restores them raw for gate-1 | accepted. |
| gate legs and `bless:dry` unrun; comment budget at its ceiling (+10) | the completion pass. |

Completion pass (`5ed5e156`): `rul-why-lens-relativizes-under-the-load-cwd` BUILT at one shared seat
(`why::relativize_for_display` over `Cwd::relativize`; the compact plan/round-trip lens and
`why_report_parts` — so the receipt-rooted `dorc why` surface followed for free); the two
`emit30-*` looms re-blessed; every other transcript byte-identical. Two cases stay red on a
SECOND, distinct leak — `tc-sibling-oracle-diagnostic-leaks-absolute-scan-path`: the
`aid-unloaded-sibling-oracle` advisory (`cli::unloaded_sibling_oracle_diagnostics`) fires FALSELY
on a `--pre-source`d `*.oracle.sh` because it compares the discovered absolute path against the
loaded RELATIVE operand, and prints the absolute path. Pre-existing product bug, invisible until
stderr was transcripted; a wrong "not loaded" hint is the mis-attributed tier of
`271:rul-sin-ordering`. RULED `rul-sibling-oracle-scan-reconciles-by-canonical-key`: the
advisory reconciles loaded-vs-discovered by canonical key, never by spelling, and renders the
path through the same relativizing seat. A product correctness change inside a suite arc,
FLAGGED to the human for veto; built at the head of lane B2a with the two re-blesses. Residue
folded there too: the carry-attribution locus in `survival::build_wrapped_analysis` still calls
`oracle_locus` unrelativized (no committed case renders it absolute).

### Checkpoint B2a (`49626ae1`; both gate legs and `bless:dry` green)

Deliverable 0 built `rul-sibling-oracle-scan-reconciles-by-canonical-key` across all three drivers
(the shipped binary's reconciler keyed by `cwd.resolve_operand`, the in-process consumer, and the
survival carry-attribution locus, all rendering through `why::relativize_for_display`);
`pin28`/`pin30` re-blessed, the known-broken state cleared, `aid-unloaded-sibling-oracle`'s own
defining case byte-identical. Then: `expect-why-receipt` gone from `FRONTMATTER_KEYS` (24 → 23;
the e2e run-lane set filters that list, so nothing else changed), `scan_why_receipt` and its
key-specific floor gone (the general non-empty discovery floor stands), `spine_baseline.rs` with
its stanza and task gone, `receipt_route.rs`'s header corrected. `why30-receipt-rooted-surface`
is a four-block session (plan → `why --receipt-last` → `--json` → `--all`, the last
byte-identical to the second) needing no clock pin; its ~1,481-line transcript carries ~37
`[unwritten: why-total-*]` rows — the legal resting state lane C makes authorable. Rulings: the
rewritten `why:` frontmatter is case metadata, not rendered prose — ACCEPTED; the reconciliation
rule joins `cli/CLAUDE.md` at close beside `rul-why-lens-relativizes-under-the-load-cwd`.

### Checkpoint B2b (`90cec4cd`; both gate legs and `bless:dry` green at `459ea02b`, the final commit a transcript-neutral `why:` retarget)

`durable_route.rs` → `receipt_state.rs`, the one state-only home (twenty tests: the fifteen
durable-route state/exit/relation tests, four recorded-facts relations — `--all` byte-identity,
unmatched-address refusal slugs, explicit-file-root identity, source-drift — and the plan-mode
refusal test from `receipt_route.rs`); `recorded_facts_route.rs` reduced to five pipeline-tier
typed-derivation tests whose harness spawn is fixture setup (its header says so);
`receipt_route.rs` stays pipeline-tier minus the moved test. Three render tests DROPPED as exact
duplicates of `why30-receipt-rooted-surface`'s blocks; no loom minted. Rulings: keeping `--all`
byte-identity as a Rust RELATION is right (two independently goldened blocks cannot enforce
equality; §3a class 6) — ACCEPTED; the `catalog_lock` path citation moved at its authoring surface
(loom `when-fires` + `--accept-metadata` republish, two commits by the tool's own refusal to bundle)
— ACCEPTED; the why30 `why:` note retarget — case metadata, ACCEPTED. No fence edited; the spawn
census fence passed unchanged.

### Checkpoint C1-map (2026-09-03; the first C1 builder stopped at the MAP with nothing built — a correct `map-then-execute-split` call. The conductor's sizing mistake: a ten-deliverable remit over a twelve-item seat table spent the builder's context on mapping. Execution is a second, fresh dispatch from the map; lanes with a reading list this long are briefed map-then-execute from the start)

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `ask-cargo-route-b-vs-fence-edit` — a `dorc-receipt-local` line in `dorc-loom/Cargo.toml` reddens `receipt-local/tests/crate_fences.rs` (`MAY_NAME_IT = ["cli"]`, a human-ack roster) | RULED route B: `dorc_cli::durable` re-exports `ModelIo`, `FailureSchedule`, `DirectorySync`, `LocalIo` beside its existing `NativeIo` re-export; `dorc-loom` never spells `dorc_receipt_local` and never calls an authority entry point, only the shared cli helpers; both rosters untouched. FLAGGED to the human as `ask-route-b-reexport-or-roster-extend` (two lines to flip). | the scout was never asked to census the fences; the brief called the Cargo edge "the ONLY boundary change". |
| `tc-model-shape-tracks-host-platform` — `pinned_roots` hardcodes `host_platform()` and the store's baseline check refuses a mismatched `directory_sync()` | ACCEPT: one constructor in `dorc-loom` selects the platform-shaped model by `cfg!(windows)`; renders carry no platform bytes, so both legs agree with one transcript. | the brief said "pick ONE shape" without reading the baseline check. |
| the rootless answer (`ROOTLESS_WORLD`; exactly two non-`run:` looms render it) | RULED `rul-rootless-worlds-are-declared-faults`, superseding this ledger's earlier "only for a session that never published": in-process, a store that cannot be reached is a DECLARED `edge-fault` (the `ReceiptPublish` precedent; a read-side twin), never a session-history flag; every undeclared session owns a real model store from block 0, and an empty store answers as production does; the two cases declare their fault and stay byte-identical. | — (the predecessor's interim rule, replaced: a flag is a hidden scripted world, and the design's idiom already exists). |
| the interleaving cell (both streams in-process versus the shell's `2>&1`) | unverified until run; the executor runs it; a disagreement is red under `DORC_KNOWN_BROKEN` with the bytes in the report. | — |
| `RunClock::Absent` → the ticking seam clock, and a real store, for EVERY in-process loom | golden movement AUTHORIZED narrowly: only lines of the no-clock/no-publish class, each listed; plan digests are clock-independent and must not move. | — |

Verified by the mapper, so no successor re-derives it: the block ordinal is errorloom's `ReplayContext::block()` (0-based, the same index `drive_session` enumerates); `seam_env`'s `ordinal` parameter is dead and its doc describes the retired per-block-SEED scheme; the receipt-edge cores to share are the bodies behind `publish_receipt` and `read_rooted_receipt` (the apply route's site is C3's); the `FnMut` closure `drive_case` takes carries session state without an `errorloom` change.

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
| A `lane-a-seams-and-harness-binary` | `Seams` + `HarnessSeams` (`30X` §4) + `HarnessSeams::from_env` on one env-reader footing; the session environment scrubbed; `compose::run(Seams)` extracted from `main.rs`; `bin/dorc-harness.rs` refusing with no seam set; seeded id/key entropy over a dependency-free generator; the ticking harness clock; ALL SIX pins retired from the shipped binary; the e2e runner spawns the harness through a `dorc` shim on PATH; `bless:dry` clean | BUILT to `4ee6aeca` (§2a); completion → A′ |
| A′ lane A completion | roots `Pinned` carries a literal path; the session scrub; the shipped-binary liveness witness; the WSL leg; `bless:dry` | BUILT to `413e2e49`; lane A COMPLETE |
| — checkpoint A | `inv-division-at-the-narrowest-edge` judged on the extraction diff | — |
| B1 `lane-b1-session-driver` (split from B after the human's sizing steer) | the shell-session process driver over the `dorc` PATH shim; gates by kind, no position rules; both-streams transcripts (every `run:` loom re-blesses — AUTHORIZED); the runner default becomes the ticking per-block clock; `durable-receipt-ambiguous` re-authored as a clock-pinned session | BUILT to `10debadd` (four floor looms red, known-broken); completion → B1′ |
| B1′ lane B1 completion | `rul-why-lens-relativizes-under-the-load-cwd`; the four floor looms re-blessed; the known-broken state cleared; both gate legs; `bless:dry` | BUILT to `5ed5e156` (two `emit30-*` green; `pin28`/`pin30` red on the sibling-oracle leak, known-broken); the remainder heads B2a |
| B2a `lane-b2a-needle-rip-and-baseline-delete` | head: `rul-sibling-oracle-scan-reconciles-by-canonical-key` + the survival locus + the two re-blesses + the gates; then the needle gate ripped whole; `why30-receipt-rooted-surface.loom` as an ordinary multi-block session; `spine_baseline.rs` + `mise run spine:baseline` deleted; `receipt_route.rs` header corrected | BUILT to `49626ae1`; COMPLETE |
| B2b `lane-b2b-battery-split` | batteries split by assertion kind (loom goldens vs the one state-only home, keeping the shipped-binary witness) | BUILT to `90cec4cd`; COMPLETE |
| C1 `lane-c1-in-process-receipt-world` (medium) | `dorc-loom` reaches `ModelIo`/`LocalIo` through `dorc_cli::durable` re-exports (NO Cargo edge; `crate_fences.rs`'s human-ack rosters untouched — `Checkpoint C1-map`); the in-process driver composes the REAL `LocalReceiptEdgeV1` over `ModelIo` with the SAME seeded entropy and ticking clock `cli::seam` holds (one generator, reached through `dorc_cli`'s lib); `run_receipt_store_why` reads that store; a store that cannot be reached is a DECLARED `edge-fault` (`rul-rootless-worlds-are-declared-faults`) and the scripted `ROOTLESS_WORLD` seat retires; a multi-block session runs in-process as ONE world with the per-block clock; a closed typed-decline enum for what it cannot express (`export`, `cd`, pipes, `cat`, external tools, unmodelled durables, process exits) — one decline routes the WHOLE session to the process driver, the set only shrinks, no roster; `gate-two-drivers-agree` in `looms.rs` (a `run:` loom run in-process without decline must equal its committed transcript byte-for-byte; declined sessions are reported by reason, not failed; both-streams order verified against the process transcripts); proof that why30's `[unwritten: why-total-*]` holes resolve through `dorc-loom vars`/`sections` and a dry publish is a fixpoint — no prose authored. OUT: shell-line modelling (C2); seeds (C3); the durable-report surfaces (C3 — extending `run_remote_apply`'s scripted table is a STOP); durable CONTENTS changes are `rul-durable-contents-reviewed-before-design` territory, a hard STOP | MAPPED 2026-09-03 (the first builder stopped at the map, nothing built; rulings in `Checkpoint C1-map`); EXECUTING from the map, fresh builder |
| C2 `lane-c2-dogfood-session-model` (medium) | the in-process driver models the session's `$` lines with our own `dorc_syntax` parser and env model (`30X` §8: HARD NACK if any kernel invariant softens; HARD DEFER, recorded under `30X:front-dogfood-ceiling`, if it needs invasive kernel change): `export` (seam variables through `HarnessSeams::from_env` over the MODELLED environment — the one parser), `cd`, `<` redirects, exact `echo $?`; everything else stays a typed decline; `rul-runner-varies-only-what-it-set` holds in-process exactly as in the shell | not dispatched |
| C3 `lane-c3-seeds-affordances-durable-report` (medium) | `seed-varied-by-default` (every seeded seam takes a fresh run seed per run; the run-wide seed printed at the start of every run; every failure names the seed and the one-line pin spelling `$ export DORC_SEED=…`); bless REFUSES a transcript that does not reproduce under a second seed; seed-dependent renders (the three whygallery looms and any other) declare their pin in the case; `seed-declared-is-regression` — one spelling across unit/DST/loom/e2e, hostsim's seed constructors taking the same run seed; the post-dispatch durable report authored over the in-process world — a durable-failure diagnostic carrying the surviving intent (a seeded id) plus the closed write-step word, and the completed apply's intent/outcome identities chrome line — minted with EMPTY prose (`[unwritten:]`, `error-authorship-tier`), each witnessed by a state-only test in `receipt_state.rs` (the store holds an intent and no outcome / holds both) | not dispatched |
| — checkpoint C3 | whether the durable failure is a sibling of `durable-receipt-unwritten` or a reason arm widening it is the CONDUCTOR's product ruling (`30X` §11), never the builder's | — |
| D `lane-d-one-runner-and-frontmatter-collapse` | one runner; the driver derived and reported; `run:`/`fixpoint:` retired; frontmatter 24→9 (`tests-critical-law` has zero uses — drop unless `vocabulary.rs` reserves it for a reason); hk/mise/bless plumbing follows; the dir-case → loom conversion (ACKED: a one-off converter deleted after use; the round-trip runner's dir entry and its marker grammar die); `lint-real-*` re-homed as the real-tools test's fixture space; the four `run: lint` looms fold into the session driver; the doctest noise; yardstick | not dispatched |

## §6 — residue accounting (empty, or kernel-only under `30X:front-dogfood-ceiling`, at close)

- Not this arc's: `dorc-sh` resolves `sh` by a bare `Command::new("sh")` (a latent
  `one-shell-answer` gap on Windows; pre-existing product behaviour, found by lane A). Roadmap
  placement is the human's.

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
DST topic row · `TODO-ADDTL` `why-surface-close-residue` (the migrated batteries) · from checkpoint A:
`cli/CLAUDE.md` gains `inspection-redrives-carry-no-durable` and `rul-roots-pinned-is-a-literal`,
its `lib-target-is-a-loom-seam` bullet is re-cut for `compose.rs` and the `Seams`/`HarnessSeams`
shape, and the five re-pointed fences are named. From B1/B2: `rul-runner-varies-only-what-it-set`
(the runner's per-block injection yields to any author assignment), `session-stdin-is-the-framed-stream`
(a `--results -` block reads the framed records from the session's fd 0; the runner frames raw
fixture records and restores them for gate-1), `block-argv-classifier-is-read-only` (classification
hands argv to the product parser, never drives), `rul-why-lens-relativizes-under-the-load-cwd` +
`rul-sibling-oracle-scan-reconciles-by-canonical-key` (display re-spells and identity reconciles by
canonical key, at one seat, across the three drivers), and `receipt_state.rs` as the one state-only
home whose header states the `30X` §3 rule (goldens in looms; state, exits, structure and relations
in Rust; typed internal decisions stay pipeline-tier). The `run_loom`/`run_round_trip` prose in
`cli/CLAUDE.md`'s harness section is stale in every bullet that names `run_replay_block`,
`drive_extra_replays`, `scan_why_receipt`, `expect-why-receipt`, block-0-must-match, or the
constant fixture clock — re-cut them around the session.
