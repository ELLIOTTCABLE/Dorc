# 30Xc — the tree reconciled against `30X` §11 (scout inventory, 2026-09-02)

> Tier: Sonnet scout inventory — COLLATED, NOT JUDGED — reconciling `30X` §11's ground truth
> against `ai/main` at `0ce6f90e` ("Tune conductor's usage of subagents and worktrees"). Names are
> stable; `file:line` numbers rot and are as-of that commit. Kept so a successor conductor need not
> re-scout; the conductor's reading of it is `30Xa` §3.

## 1. Corpus census (`spike/crates/*/tests/`)

| crate | X.loom | X/X.loom | X/cmd | X/book.sh+expected.out | X/book.sh alone | .rs |
|---|---|---|---|---|---|---|
| aid | 200 | 0 | 0 | 0 | 0 | 6 |
| cli | 108 | 0 | 0 | 109 | 2 (`lint-real-checkbashisms`, `lint-real-shellcheck`) | 11 |
| dorc-loom | 3 (+5 fixtures under `tests/fixtures/`) | 0 | 0 | 0 | 0 | 13 |
| lint | 0 | 0 | 1 (`loop-brace-range-runs-once/cmd`) | 0 | 0 | 3 |
| oracle | 0 | 0 | 0 | 0 | 0 | 2 |
| others | 0 | 0 | 0 | 0 | 0 | 1–13 each |

- `X/X.loom` multi-file shape: ZERO instances corpus-wide. No `*.sync-conflict-*` files.
- The two `lint-real-*` dirs are a third, undocumented dir shape, driven only under
  `DORC_E2E_REAL_TOOLS` (`e2e.rs:3040-3150,4100-4114`), not the round-trip battery.
- 311 corpus looms: `run:` present 109 (`round-trip` 105, `lint` 4); `fixpoint: executed` 109 (1:1
  with `run:`).
- cli dir-case markers (111 subdirs): `mocks/` 106 · `expected.ran` 106 · `expected.out` 109 ·
  `expected.emitted` 1 (`floor30-inline-dot-boundary`) · `ARTIFACT_SET` 7 · `XFAIL` 4 ·
  `DORC_FLAGS=` 13 · `tolerate=` 4 (all `pipe-stage-order`) · `PROBE_RESULTS=authored` 21 ·
  `DUAL_RAIL=multiline-argv` 1 (`inlined` 0) · `DORC_EXIT=` 2 · `expected-diagnostics` 9 ·
  `expected-why` 11 · `expected-hint` 2 · zero: `EXIT_RC`, `WHY_ADDR`, `expected-why-chain`,
  `expected-why-receipt`, `head-expected.ran`, `expected-rc`, `cmd`.
- The 3 `expected.out` dirs without `mocks/`: `floor30-inline-dot-boundary`,
  `load30-blind-act-withholds-its-carriage`, `load30-script-relative-lints`.

## 2. Frontmatter key usage (`dorc-loom/src/vocabulary.rs:36-170`; 24 keys; over 311 looms)

code 117 (run_lane) · arrangement 83 · when-fires 117 (rl) · when-used 9 · why 127 (rl) · owns 30
(rl) · todo 1 · run 109 (rl) · fixpoint 109 (rl) · flags 9 (rl) · exit 6 (rl) · apply-exit 1 (rl) ·
tolerate 2 (rl) · artifact-set 1 (rl) · probe-results 4 (rl) · dual-rail 1 (rl) · why-addr 6 (rl) ·
expect-diagnostic 19 (rl; 12 distinct slugs) · expect-why 8 (rl) · expect-hint 2 (rl) ·
expect-why-chain 6 (rl) · expect-why-receipt 1 (rl; `cli/tests/why30-receipt-rooted-surface.loom`)
· tests-critical-law 0 (rl) · envelope 7.

## 3. Seam seats (all exist unless marked)

`Harness` struct `cli/tests/e2e.rs:169`, `Harness::dorc` `:270` · `OWN_PROFILE_DIR` `e2e.rs:167`
(reads `:275`, `:1376`) · `DORC_FIXTURE_CLOCK_MS` const `cli/src/main.rs:1469`, read `:1476` in
`clock_for_invocation` `:1475`, `RunClock::Ticking` at `:1481-1484` · `DORC_STDOUT_POSTURE` const
`main.rs:1496`, read `:1507` in `stdout_posture` `:1503` · `DORC_FIXTURE_SOURCE_MATCH`
`cli/src/source_match.rs:32`, read `:78` · `run_replay_block` `e2e.rs:1601` · `run_loom`
`e2e.rs:1353` · `drive_extra_replays` `e2e.rs:1444` · `scan_why_receipt` `e2e.rs:2818` ·
`materialize_loom` `e2e.rs:1087` · `ProfileSandbox` `cli/tests/sandbox.rs:23` · `apply_roots_under`
`sandbox.rs:73` · `OsEntropy` `cli/src/receipt_edge.rs:168` (built `main.rs:422,2752`) ·
`OsKeyEntropy` `cli/src/durable.rs:652` (built `main.rs:417,2760`) · `ReceiptIdEntropy` trait
`receipt/src/ids.rs:140`, `EntropyReceiptIds::over` `:158` · `KeySecretEntropy` trait
`receipt-crypto/src/key_document.rs:481`, `EntropyKeysetGenerator::over` `:509` · `Framing::spike`
`plan/src/records.rs:193` (~35 call sites) · `admit_fixture_records` `cli/src/results.rs:758` ·
`run_engine` `dorc-loom/src/consumer.rs:998` · `run_receipt_store_why` `consumer.rs:984` ·
`ROOTLESS_WORLD` `consumer.rs:1284` (`"no-controller-root"`) · `LoomEngineEdges` `consumer.rs:1654`
(built with `RunClock::Absent` `:1076`) · `run_remote_apply` `consumer.rs:838` ·
`ship_consented_apply` `main.rs:2703` · `ConsentedApply` `cli/src/apply.rs:331` ·
`ApplyPlanNotDispatchable` `aid/src/diag.rs:2169` (variant `:412`) · `RootEnvironment` trait
`cli/src/durable.rs:60` (`var(&self, name) -> Option<String>`; impl `ProcessEnvironment`
`main.rs:842-846`) · `Posix::find` `internal-tooling/src/lib.rs:44` · `Posix::floor` `:92`.

NOT IN TREE (forward design): `Seams`, `Seams::from_env`, `EnvReader`, `bin/dorc-harness.rs`
(`cli/src/bin/` holds only `dorc-sh.rs`; `cli/Cargo.toml` bins: `dorc`, `dorc-sh`).

## 4. Environment reads reachable from the shipped `dorc` (no dependency crate reads any)

`APPDATA` `durable.rs:80` · `LOCALAPPDATA` `:81` · `HOME` `:85,91` · `XDG_CONFIG_HOME` `:93` ·
`XDG_STATE_HOME` `:97` (all via `RootEnvironment`) · `PATH` `main.rs:1272` · `PATHEXT`
`main.rs:1289` · `DORC_FIXTURE_CLOCK_MS` `main.rs:1476` · `DORC_STDOUT_POSTURE` `main.rs:1507` ·
`DORC_FIXTURE_SOURCE_MATCH` `source_match.rs:78` · `DORC_TRANSPORT` `cli/src/transport_edge.rs:87`
(debug builds only, `cfg!(debug_assertions)`) · `DORC_TRANSPORT_INTERPRETER` `transport_edge.rs:91`
(debug only) · `DORC_FIXTURE_NONCE` `transport_edge.rs:55`. `dorc-sh` reads no environment.
Dev tools (never reach `dorc`): internal-tooling reads `DORC_COVERAGE`, `NO_LINT_DOCIDS`,
`XDG_CACHE_HOME`, `HOME`, `LOCALAPPDATA`, `PATH`, `PATHEXT`, `CARGO_TARGET_DIR`, `DORC_PREFLIGHT`,
`WSL_DISTRO_NAME`, `SystemRoot`; the `dorc-loom` bin reads one variable at
`dorc-loom/src/bin/dorc-loom.rs:451`.

## 5. hostsim seed plumbing

Seed = `u64` into `Host::seeded(seed, …)` `hostsim/src/lib.rs:518`, `Lcg::new(seed)` `:53`,
`Host::with_sigpipe_race` `:546`. Generator `lcg-only-entropy` `lib.rs:38-64`, no `rand`. The
replay law (`hostsim/CLAUDE.md:42-44`: "a failing seed (+ commit) must deterministically reproduce;
surface the seed in every DST failure message") is enforced per-assertion (`lib.rs:1140,1146,1154`),
with no run-wide print. Seed constructors outside hostsim: `plan/tests/sparing_differential.rs:231,329`,
`sweep/src/scenario.rs:372`, `hostsim/examples/differential.rs` (CLI `--seed`/`--sweep`/`--start-seed`).

## 6. `receipt-local` `LocalIo` and the Cargo edges

`pub trait LocalIo: Sealed` `receipt-local/src/io.rs:510` (sealed `:26-31`: nameable outside,
implementable only inside). `NativeIo` `native.rs:75`, impl `:142`, re-exported at the root
(`lib.rs:223`). `ModelIo` `model.rs:151`, impl `:385`, `pub` in `pub mod model` (`lib.rs:209`) —
NOT `cfg(test)`, NOT feature-gated; reachable as `dorc_receipt_local::model::ModelIo`.
Edges: cli → core, aid, syntax, analysis, oracle, plan, lint, receipt, why, receipt-crypto,
receipt-local, transport (dev: internal-tooling, dorc-loom, errorloom) · dorc-loom → core, aid,
oracle, syntax, analysis, plan, lint, transport, cli (NO receipt / receipt-crypto / receipt-local /
why) · receipt → none · receipt-local → receipt, receipt-crypto · receipt-crypto → receipt · why →
receipt, aid · internal-tooling → none. cli↔dorc-loom is a deliberate dev-only cycle
(`cli/Cargo.toml:97-101`).

## 7. Invocation plumbing

mise (root `mise.toml`): `test` `:248` dir spike, run `["cargo test --workspace --doc {{filter}}",
"cargo nextest run --workspace --no-fail-fast {{filter}}"]` · `test:e2e` `:290` `cargo test -p
dorc-cli --test e2e --` · `test:looms` `:303` `… --test looms --` · `test:e2e-quiet` /
`test:looms-quiet` `:318/:328` (same, `DORC_E2E_QUIET=1`) · `test:real-tools` `:335`
(`DORC_E2E_REAL_TOOLS`) · `test:floor` `:342` (`DORC_E2E_FLOOR_SHELLS`) · `gate:full-quiet` `:269`
`[preflight gate, gate:floor, hk check --pr --profile medium --profile slow --no-fail-fast --quiet,
hk check --staged…, hk check --unstaged…]` · `gate:floor` `:421` · `gate:step` `:415` · `gate:arc`
`:430` · `test:hooks` `:441` · `bless` `:497` `[preflight bless, internal-tooling -- bless]` ·
`bless:dry` `:503` · `bless:case` `:515` (`BLESS=1`, `cargo test -p dorc-cli --test e2e --`) ·
`bless:floor` `:526` · `loom` `:509` (`cargo run -q -p dorc-loom --bin dorc-loom --`) ·
`prose:census` `:538` · `prose:orphans` `:544` · `spine:baseline` `:550` (`cargo test -p dorc-cli
--test spine_baseline -- --ignored --nocapture`) · `xfail:census` `:556` · `coverage` `:562` ·
`yardstick` `:574` `sh e2e/yardstick.sh` — BROKEN as of 2026-07-26.
hk.pkl corpora steps: `loom-hygiene` (pre-commit; glob `spike/crates/*/tests/*.loom,
spike/crates/*/tests/*/*.loom`; `cargo test -q -p dorc-cli --test looms …`) · `e2e` (glob
`spike/crates/*/tests/*/**, spike/crates/cli/tests/*.loom`; `cargo test -q -p dorc-cli --test e2e …
-- {{files}}`, env `DORC_E2E_QUIET=1`) · `minispec`. Slow profile: `test-floor`, `test-real-tools`,
`test-hooks`, `test-floor-suite` (`mise run test`), `verify-check`, `verify-kani-check`. Arc:
`verify-translate-check`, `verify-lean-badges`, `verify-kani`. pre-commit `stash="git"`, `fix=true`;
commit-msg: `sh .githooks/commit-msg`.
`internal-tooling/src/bless.rs`: preflights `mise --version` / `git rev-parse` `:59`; `mise run
gate:full-quiet` `:81`; `mise run test:floor` `:101` (floor); `git diff --stat -- crates/cli/tests
:!crates/cli/tests/*.rs` `:116-126`; `mise exec -- cargo test -p dorc-cli --test e2e [-- cases]`
env `BLESS=1 DORC_E2E_QUIET=1` (+ `BLESS_FLOOR=1 DORC_E2E_FLOOR_SHELLS=dash,posh`) `:176-190`.
`cli/Cargo.toml` `autotests=false` (:9); `[[test]]`: e2e (harness=false), looms (harness=false),
definition_frames, receipt_route, durable_route, recorded_facts_route, sh_parity, region_artifacts,
spine_baseline (stanza comment `:153-155`: DELETE at the fold review).

## 8. Line counts

cli/tests: sandbox 89 · region_artifacts 222 · looms 269 · spine_baseline 420 · support 478 ·
recorded_facts_route 612 · durable_route 872 · definition_frames 976 · sh_parity 1238 ·
receipt_route 1670 · e2e 4137. dorc-loom/src: consumer 2877 · staging 1538 · staging_store 987 ·
repository 949 · ownership 572 · generate 500 · invocation 500 · edit 443 · lib 352 · vocabulary
286 · usage 250 · preview 238 · compile 203 · refusal 194 · defect 172 · edge_fault 164 · roots 117
· inspect 95. cli/src: main 5456 · results 1071 · receipt_edge 842 · apply 842 · durable 765.
internal-tooling/src: xfail 775 · doctor 655 · preflight 611 · docids 546 · gate_floor 416 · bless
397 · precommit_gate 368 · hook_selftest 352 · lib 196 · arrangement_census 155 · step_globs 136 ·
coverage 96 · prose_census 85 · main 64 · fmt_detached 56 · livetest 42 · posix_script 41.

## 9. Test-helper duplication (75 test .rs files; 88 fn names in ≥2 files, incl. coincidental test names)

`next_receipt_id` ×10 (cli/tests/receipt_route, receipt/tests/{apply_projection, dispatch,
plain_narrowing, recorded_why_facts, support/mod}, receipt-crypto/tests/crypto_interop,
receipt-local/tests/{native_store, store_sweep}, why/tests/support/mod) · `new`/`drop`
sandbox-like ×10/×7 (cli/tests/{durable_route, e2e, recorded_facts_route, sandbox},
dorc-loom/tests/publish_write_path, lint/tests/adapters, plan/tests/sparing_differential,
receipt/tests/recorded_why_facts, receipt-local/tests/{native_keyset, native_store}) · `material` ×5
· `authored` ×5 · `vouch_all`/`is_replaced` ×4 (plan/tests) · `roots` ×4 (receipt-local/tests) ·
`corpus_dir` ×4 (dorc-loom/tests).
`mod support;` in cli/tests: definition_frames:48, e2e:34, looms:28, sh_parity:19,
spine_baseline:54. `mod sandbox;`: durable_route:31, e2e:33, receipt_route:27,
recorded_facts_route:36, spine_baseline:53. `receipt/tests/support/mod.rs` and
`why/tests/support/mod.rs` are separate crate-local dirs.

## 10. xfail registry (`internal-tooling/src/xfail.rs`; `CURRENT_ROUND = 30` at `:45`; 24 pins `:154-426`)

Reserved (no call site): `d-alpha-rename-equivalence` (end-of-r31),
`p-x-durable-account-export-is-enabled` (r31:kernel-punt-glance). Scheduled end-of-r30:
`p-x-front-hoist-lifts-a-clean-bundle` (`cli/src/artifact.rs:1704`),
`p-x-front-hoist-munges-a-colliding-role-name` (`:1735`). r31:book-load-acceptance: seven
load-operand pins at `cli/src/main.rs:2156-2401`. r31:kernel-punt-glance: five at
`analysis/src/funcenv.rs:5717-5823`. Unscheduled r31/end-of-r31: `p-x-intra-compound-plurality`
(`cli/tests/sh_parity.rs:830`), `p-x-placement-tuning-pair` (`plan/src/lib.rs:7504`),
`p-x-book-level-dot-locals` (`funcenv.rs:5392`), `p-x-blessed-toplevel-conditional`
(`sh_parity.rs:235`), four env-identity pins (`plan/tests/env_identity_pins.rs:173-274`).

## 11. The loom publish clean-tree gate

`dorc-loom/src/repository.rs::classify_prose_changes` `:330-421` — BLAST-RADIUS scoped, not
whole-tree (doc `:323-324`: "Dirt elsewhere in the repository no longer refuses it"). Predicate:
both generated locks byte-equal to HEAD (`:157-159`; refusals `:348-350`, `:373-376`); each
SELECTED case wholly unstaged ` M` or wholly staged `M ` (`:355-371`; untracked refused
distinctly); paths outside selected ∪ locks skipped (`:351-354`); a selected case's diff confined
to replay-output islands (`same_non_replay_output_bytes` `:391-401`). No hook refuses on a dirty
tree; pre-commit stashes (`stash="git"`). `CONTRIBUTING.md`'s "must have a clean worktree" text is
stale against this.

## 12. Doctests

No crate sets `doctest = false` or `doc = false` (0/21). `mise run test` runs `cargo test
--workspace --doc {{filter}}` THEN nextest (`mise.toml:262-265`; the `:47-51` comment says nextest
runs zero doctests, claims "12" doctests, names `aid/src/narrative.rs:23`'s `compile_fail` as
load-bearing). The true count needs `cargo test --doc -- --list`. `spike/verify/{aeneas,kani}` are
separate workspaces.

## 13. Batteries

| file | #[test] | spawns | ignore | note |
|---|---|---|---|---|
| durable_route.rs | 14 | `CARGO_BIN_EXE_dorc` `:85` | no | render-ish ~13 / state-ish ~15 |
| recorded_facts_route.rs | 12 | `:77,449` | no | render ~11 / state ~4; already asserts every `scan_why_receipt` property |
| receipt_route.rs | 20 | one spawn test `:1638,1659`, rest in-process | no | render ~12 / state ~1; stale "cannot sign" header `:19-20` |
| spine_baseline.rs | 1 | `:112` | YES (`:145`) | build-to-kill |
| definition_frames.rs | 5 | no | no | census |
| region_artifacts.rs | 4 | no | no | census over committed goldens |
| sh_parity.rs | 18 | no | no | pipeline tier (oracle-tier half at `oracle/tests/sh_parity.rs`) |

(The render/state split is a grep-based line-count proxy, not a per-assertion parse.)
