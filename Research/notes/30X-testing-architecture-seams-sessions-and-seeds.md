# 30X — the testing architecture: seams, sessions, and seeds

> Tier: conductor-authored DESIGN STATEMENT (Fable, 2026-09-01), distilled from a sitting with the
> human; every `[TYPED]` item is the human's own ruling, everything tagged `lean-` is the
> conductor's and stands unvetoed. Ahistorical: this says how the suite is meant to WORK, not
> how it got here. Authority: root `IMPLEMENTATION.md` (DST + judicious integration tests as the
> correctness discipline for an agent-built codebase) and `spike/CLAUDE.md` outrank it; it
> composes with `plans/128` (DST: the controller↔host session is the network's one seam) and
> `plans/282` (the loom pipeline), and SUPERSEDES three narrower rules in place: `282` §2's
> "harness-only environment must not appear", `crates/cli/CLAUDE.md`'s
> `one-fixpoint-authority-per-case`, and the block-0 clause of `loom-form-is-the-same-battery`.
> The build-planning tail (§11) is the only implementation-focused section; a successor
> conductor reconciles it against the tree and starts there.

## §1 — purpose and priorities

- **`pri-order`** `[TYPED]` — the best product, maintainable tests, and safety; tertiarily
  velocity, cleanliness, simplicity. When a testing choice trades among these, that is the order.
- **`pri-breadth-is-the-constraint`** `[TYPED]` — Dorc is an sh analyzer AND an orchestrator: its
  testing-target universe is every seam and aspect of multiple unix processes, on machines Doing
  Things, mattering to each other. That is beyond a normal library's testing discipline, and the
  general structure must COVER it rather than be assembled by accretion as each need arrives.
  Lean: more abstract, not less; more powerful, not smaller — the inverse of the project's usual
  taste, deliberately.
- **`pri-tooling-is-never-deferred`** `[TYPED]` — good tooling work is always owed and almost
  never blocks product work in the age of LLMs; it is deferred only when catastrophically huge.
  It cannot be done in flight or in parallel with product lanes, so suite work is its own
  non-concurrent arc, and product work stays fully separate from it — which is exactly what
  licenses a suite arc to re-bless freely.
- **`pri-one-durable-yields-tests-and-prose`** — a loom is one committed file that is
  simultaneously an executable test and the authoring surface for every user-facing sentence it
  renders (`282:rul-transcript-is-the-authoring-surface`). Looms are therefore the DEFAULT shape
  for anything that renders; every other shape justifies itself by what a loom cannot assert.

## §2 — the world model

- **`model-a-case-is-a-world`** — a case is a WORLD (sources · declared facts · seam selections)
  plus an ordered shell SESSION and its exact OUTPUT. Nothing about a case is a property of a
  driver; drivers are interchangeable executors of the same case.
- **`model-seams-are-one-bundle`** — every nondeterministic edge the engine consumes is a typed
  member of ONE bundle (strawman `Seams`): clock · receipt-id entropy · key entropy · stdout
  posture · roots/environment · transport · and future columns (a process supervisor, netns, a
  sudo prompt, …). Each ordinary-harness seam independently selects a model implementation:
  `Seeded(seed)` · `Pinned(value)` · `Os`; transport additionally offers `Scripted(case sections)`
  and `Hostsim`. `RealSsh` is NOT in that cross-product: it belongs only to the explicit livetest
  composition and is unconstructible from loom session configuration. A tier is a ROW of its
  permitted matrix, never a separate harness; a new seam is one column added once, and every driver
  inherits it. Only columns with an implementation in hand are built — the STRUCTURE is the
  deliverable.
- **`model-determinism-at-the-source`** — no driver asks the operating system for a value unless
  the case set that seam to `Os`. Values are typed, injected at the composition root, and
  derived from case data plus an ordinal; nothing is normalized after the fact
  (`crates/aid/CLAUDE.md` seam-tolerated-nondeterminism-stops-at-the-run-log stands: rendered
  output is made deterministic at the source or not at all).
- **`model-unpeg-per-seam`** `[TYPED]` — determinism is selectively un-peggable: a case may set
  any single seam to `Os` and owns the consequence (that value can no longer be golden'd). The
  spelling is the same as pegging (§5).
- **`model-facts-are-data`** `[TYPED]` — world facts (probe answers, host state, transport
  outcomes) have no seam because they are FACTS: declared case data, never varied. A case that
  wants GENERATED facts (hostsim) selects the transport seam's generator and, by committing a
  transcript over it, has pinned it.

## §3 — tiers, as rows of the matrix

- **`tier-unit`** — kernel functions over values; every model seam, seeded. Fast, plentiful,
  seeded; done; keep. Assertions: anything.
- **`tier-in-process-loom`** — the real engine plus the real EDGE IMPLEMENTATIONS over
  deterministic MODELS (an in-memory `LocalIo` for the receipt store; seeded entropy; a ticking
  case clock; scripted observation), driven in one process. Fast, editable, render-fixpointed.
  This is a PERFORMANCE OPTIMIZATION of the e2e tier, not a different kind of test (§5); its
  correctness claim is `dorc-replay-is-production-semantics`. Its gap is never determinism (it is
  deterministic by construction) but COVERAGE: a world it cannot yet express is a typed decline,
  and the session runs in the process driver instead.
- **`tier-e2e`** `[TYPED]` — THE tier for the product mechanized above unit level, in two
  shapes: (a) looms as shell sessions driven through the harness binary (the default — tests
  AND prose); (c) Rust-authored batteries — arbitrarily complex setup, spawning either the
  seeded harness binary or the shipped `dorc` with `Os` seams, asserting STATE, EXITS,
  STRUCTURE and RELATIONS (files that exist or do not, a keyset that was or was not created, a
  refusal code, a code slug present, two invocations byte-identical, an inequality) and never a
  render GOLDEN. A byte-exact render belongs in a loom, always; a structural assertion over a
  render is legal in (c) only where the bytes are un-goldenable (`Os` seams, foreign tools,
  platform variance). Which cases may take shape (c) AT ALL is §3a's closed license, and that
  license is subordinate to §3a's prose law `[TYPED 2026-09-02]`. The former shape (b), the
  dir-case, was (a) serialized as files — one test in two spellings, since the loom runner
  materializes a loom into exactly the dir layout — and is CONVERTED, not kept
  (`[TYPED 2026-09-02: "no legacy e2e"]`; the seven in-tree counterexamples that drove the
  ruling are `30Xa` §4).
- **`tier-census`** — corpus-wide property tests: universally-quantified assertions over the
  whole case population ("no happy-path artifact contains a munged name"; "every lifted role row
  carries its parsed span"). A per-case golden cannot say "for all cases", so this is a real,
  settled shape — the executable form of `30A` quality-is-a-ratchet — and it never becomes a loom.
- **`tier-pipeline`** — authored-world tests: a world written inline in Rust, driven through the
  real pipeline in-process, asserting a typed INTERNAL decision (did this site keep its license).
  Legitimate today, with a standing pull: as the `why`/`--json` surface becomes total over
  decisions, "site 3 was licensed" is a loom golden and the inline world becomes a loom world.
- **`tier-livetest`** — real ssh, real hosts, the `Os`/`RealSsh` row; gate/bless-tier, never the
  hot loop (`notes/26D`). It is a separate explicit composition root, not a seam selection available
  to ordinary looms; privileged, `chroot`, real-mutator, and other intentionally ambient exercises
  belong here rather than teaching restrictions to the loom language.

## §3a — what may be non-loom, and when `[TYPED 2026-09-02]`

- **`law-all-prose-is-loomed`** `[TYPED]` — every user-facing sentence a surface renders has a
  loom that renders it, and every other rule in this section is SUBORDINATE to this one. "My
  testing situation is complex" never makes a surface loom-less: a complex situation grows a
  loom for the prose AND, in extremis, a narrow specialized harness for its one un-loomable
  assertion — additive, never instead. The loom carries every rendered byte; the harness carries
  only the assertion a loom cannot state.
- **`rul-non-loom-is-licensed-by-class`** — a test that is neither a loom nor a unit test is
  legal only when what it asserts falls in one of seven classes, and then it asserts STATE,
  EXITS, STRUCTURE or RELATIONS, never a render golden: (1) the suite's own machinery — runner,
  harness, bless, hooks, discovery floors, fixpoint gates (a loom cannot test the loom runner
  without regress); (2) a universal over the corpus, "for every case …" (`tier-census`); (3) an
  invariant over seed-generated worlds (`seed-exploration-asserts-invariants`); (4)
  platform-variant state where the property under test IS the platform difference (one
  transcript serves both legs, and nothing is normalized after the fact); (5) bytes nobody
  controls — a real tool's text, a real host's timings — asserted structurally, never goldened;
  (6) a relation over un-goldenable bytes from `Os` seams — inequality across runs,
  cross-invocation identity, a slug present; (7) timing and interleaving. A case outside these
  is a loom. A session that would have to become a shell program whose golden is the word `ok`
  is the signature of class (6), not a licence to write that program.
- **`rul-loom-escape-stops-at-the-conductor`** `[TYPED]` — no builder mints a new non-loom test,
  harness, driver, runner, gate family, marker, frontmatter key, or fixture shape on its own
  judgment — "this one case needs a tiny special runner" included. Every such impulse STOPS and
  is raised to at least a conductor, who rules it against the seven classes; anything the
  classes do not cover is the human's. The repeated loom-escapes of past arcs are the failure
  this rule ends; the detritus they left is what this arc removes.
- **`rul-one-handbuilt-mechanism-per-arc`** `[TYPED]` — at most ONE new bespoke test mechanism
  per arc (r30, r31, …): a harness, a driver, a runner, a gate family, a fixture shape, an
  assertion vocabulary. A second in the same arc is refused by default — fold it into the arc's
  one, or wait for the next. A new `#[test]` in an existing licensed battery is a test, not a
  mechanism: growth WITHIN the inventory below is free, growth OF the inventory is what this
  caps. The mechanisms are extremely fragile and problematic to maintain; that is the whole
  reason.
- **`inv-non-loom-inventory`** (lean-; the grandfathered set at this arc's close, kept current
  here) — the one runner (looms, and the Rust batteries of shape (c)) · hostsim DST and the seed
  sweeps · the floor lane (`test:floor`, measure-once manifests) · the real-tools lane
  (`DORC_E2E_REAL_TOOLS`) · livetest · the census tests · the harness selftests (bless, hooks,
  doctor, discovery floors, fixpoint gates). Each is one mechanism; a member added to one is a
  test; a row added to this list is an arc's one mechanism and a conductor's ruling.

## §4 — the binary under test

- **`bin-harness-sibling-not-produced-cli`** `[TYPED ack]` — runtime injection into the
  binary-under-test is REQUIRED: a process exiting, a second process reading what the first
  wrote, a keyset created on disk, are the properties, and an in-process
  `act_like_the_product(argv)` cannot stand in for them. But the injection lives in a SIBLING
  BUILD, never in the shipped `dorc`: one crate, one lib, two composition roots —
  `bin/dorc.rs` = `exit(compose::run(Seams::os()))`, `bin/dorc-harness.rs` =
  `exit(compose::run(HarnessSeams::from_env(&real_env)))`, the latter refusing loudly when no seam
  is set and structurally unable to construct real transport or production roots/persistence.
  Arg parsing, source acquisition, root resolution, the receipt edge, the engine — all BELOW the
  seam, shared byte for byte.
- **`inv-division-at-the-narrowest-edge`** `[TYPED]` — the shipped `main.rs` is edge VALUES plus
  one call; anything with a branch, a parse, or a decision belongs below the seam. Everything
  above the seam is testable only by shape (c) of the e2e tier over the shipped binary, so that
  remit is bounded — the composition lines plus "the OS implementations are live" — and must
  never grow. Binds `dorc-sh` equally. A mechanical no-control-flow check over `main.rs` is
  legal only as a human-ack fence (`lexical-fences-are-human-ack-instruments`).
- **`inv-fixture-state-never-typeable-into-main`** `[TYPED]` — the sharpened reading of
  `rul-fixture-identity-never-production`: "anything published" means PUBLIC INTERFACES. A
  durable-file feature or state we never want a production user to see must not be producible
  by any code path typeable into `cli/main.rs` — not "must not exist", which would rule out
  deterministic testing altogether. Corollary: the shipped `dorc` reads NO harness-shaped
  environment at all; its former pins (`DORC_FIXTURE_CLOCK_MS`, `DORC_STDOUT_POSTURE`,
  `DORC_FIXTURE_SOURCE_MATCH`) move into the harness composition as ordinary seams. A cargo
  feature would also satisfy the invariant, and is not the chosen spelling: it drags `--features`
  through every repo-owned test invocation, and a forgotten flag either reddens id-bearing goldens
  or — under `required-features` — silently skips the corpus, the zero-trials-exit-green shape.
- **`bin-seeded-entropy-is-dependency-free`** — the harness's seeded entropy is a small
  hand-rolled generator over the seed (hostsim's `lcg-only-entropy` posture), feeding the trait
  points the crates already carry (`ReceiptIdEntropy`, `KeySecretEntropy`). Keys and ids are
  then deterministic per (case, seed); the keyset files, store names and every rendered
  identity are stable.

## §5 — looms are shell sessions

- **`loom-is-a-shell-session`** `[TYPED]` — a loom's replay is a POSIX shell session and NOTHING
  about block position, ordering, or content is restricted. In-memory execution is a suite
  performance optimization and must not lead design; a restriction adopted for the optimizer's
  convenience is a restriction to be walked back the day a loom needs to `chroot` first.
  "Not supported" is a closed-and-shrinking set at the in-memory tier and EMPTY at the loom
  tier; no meta-test polices that — aim high, cede ground only as economically necessary.
- **`loom-process-driver-is-a-real-shell`** — materialize the case into a throwaway dir; start
  `sh` (`internal_tooling::Posix`, the one-shell-answer seat) there with the harness environment
  and `dorc` on a shim PATH resolving to `dorc-harness`; feed the `$` lines through one
  persistent, sentinel-delimited session (the records lane's own framing trick) and capture per
  line. `export`, `cd`, pipes, `cat`, `echo $?`, redirects — everything the shell does is
  native, and the artifact-execution rail (`PATH=<mocks>` only, `env -i`, a throwaway cwd,
  `umask 022`) stays a separate, runner-owned process exactly as today.
- **`loom-syntax-grants-no-production-authority`** `[TYPED after 30-reviewA]` — unrestricted shell
  GRAMMAR stays the point; it does not imply inherited production authority. The ordinary runner
  starts from a scrubbed environment with runner-owned cwd/profile roots and no inherited credential
  variables, and its harness composition cannot select `RealSsh` or production roots/persistence.
  This is deliberately NOT a claim that arbitrary shell is contained from the developer machine;
  the harness remains a powerful developer subsystem. A test that intentionally needs real-host,
  privileged, `chroot`, or other ambient capability uses the explicit livetest composition without
  narrowing the loom language. This is the complete tightening accepted in response to
  `Research/quarantine-DO-NOT-READ/30-reviewA-opaque-report.md`; do not rebuild the retired closed
  command grammar or accrete per-case escape machinery in its name.
- **`loom-in-process-driver-is-a-closed-grammar`** — the in-process driver runs the sessions it
  can express in memory and declines the rest with a TYPED decline (an external tool, a
  filesystem durable it has no model for, a process exit, an unmodelled construct); one decline
  anywhere routes the WHOLE session to the process driver, because a session has state and
  cannot change drivers midway. Lean, human hard-acked with conditions (§8): the closed grammar
  is recognized by our own `dorc_syntax`, and the session's environment/cwd model is our own env
  model, not a hand grammar.
- **`loom-driver-is-derived-and-reported`** — which driver proves a session is DERIVED from the
  session's needs and REPORTED in the trial name, never declared in the case (`run:` and
  `fixpoint:` retire). Where both drivers can run a session, BOTH do and must agree byte for
  byte (`gate-two-drivers-agree`); the process proof is authoritative where it runs and the
  in-process render is a second witness. This replaces `one-fixpoint-authority-per-case`, and it
  closes a real hole: `dorc-loom publish` attributes prose edits through the in-process render,
  which nothing previously checked against the bytes the binary proved.
- **`loom-gates-attach-by-kind`** — gates belong to what a block IS, never where it sits. The
  runner hands each `$ dorc …` line to the product's own arg parser: an artifact-producing block
  gets the artifact gates (dash `-n`, exec-under-mocks with its own run-set, guard-shape,
  redirect scan, argv-echo, dual-rail); every block gets the diagnostic gates. A dir-case is a
  one-block session. The gates re-drive their subject block with split streams for parsing — a
  seeded invocation driven twice is byte-identical, so the double drive is honest.
- **`loom-transcript-is-what-the-user-saw`** — both streams, in the order the user saw them
  (`2>&1` at the session; the in-process driver already emits ordered events for both). A
  diagnostic is then a transcript line, and `expect-*` needle keys have no job.
- **`loom-seams-are-sh-lines`** `[TYPED]` — seam selection is spelled as sh IN the session:
  `$ export DORC_SEAM_CLOCK=seeded:7`, one variable per seam (a `DORC_SEED` umbrella is fine).
  The harness binary reads the real environment; the in-process driver reads the MODELLED
  session environment; ONE parser (`Seams::from_env(&dyn EnvReader)`, on the `RootEnvironment`
  reader's footing) serves both. "We express things as sh in this house, and we have an sh
  engine the size of God." Defaults for every loom, so a bare session works: cwd = the
  materialized dir; stdin = the block's redirect or nothing; every seeded seam varied (§6).
  This deliberately reverses `282` §2's "harness-only environment must not appear": seams are a
  documented typed surface of the harness binary, not fixture authority leaking in.
- **`loom-frontmatter-is-registry-metadata-only`** `[TYPED lean: reduce only where CLEARLY
  better; stay on-target]` — frontmatter survives only for what is ABOUT THE CASE AS AN AUTHORING
  HOME: `code` · `arrangement` · `owns` · `when-fires` · `when-used` · `why` · `envelope` ·
  `tests-critical-law` · `todo`. Everything that was really a run knob or an assertion is
  spelled in the session instead: `flags` → the `$ dorc` line · `exit`/`apply-exit` →
  `$ echo $?` · `probe-results` → the `<` redirect · `why-addr` → `$ dorc why book.sh:4` ·
  `artifact-set` → `--artifact-dir` on the line · `tolerate` → an export · the four `expect-*`
  keys → the transcript (the catalog-validation of `[slug]` headers stays as a runner check
  over transcript bytes — `needles-are-structural` in its honest form) · `run:`/`fixpoint:` →
  derived. Twenty-four keys become nine.
- **`loom-one-runner`** — `looms.rs` and `e2e.rs` are one runner over one walk of every
  `crates/*/tests`, minting one named trial per case, with the discovery floor and the
  sync-residue skip unchanged.

## §6 — seeds

- **`seed-varied-by-default`** `[TYPED]` — every seeded seam takes a fresh seed on every run. A
  render that does not depend on entropy, clock, or ordering reproduces under any seed, and
  running it under a new one each time is a free invariance test; fixing the seed everywhere
  would throw that CPU work away.
- **`seed-declared-is-regression`** `[TYPED]` — a declared seed (`$ export DORC_SEED=7`) exists
  for two purposes only: regression, and stabilizing an output that legitimately shows
  seed-dependent bytes (a receipt id, a date). It is an OPT-IN, per case, with one spelling
  across the entire suite — regression looms, regression e2es, regression unit tests alike. When
  a varied surface churns because it turns out to contain nondeterminism, pinning it is the
  one-line fix.
- **`seed-two-affordances`** — so an intermittent red never becomes agent-retry fodder: the
  run-wide seed prints at the start of every run and is named in every failure together with
  the one-line pin/replay spelling (hostsim's `replay-seed` law, widened to the whole suite); and
  bless REFUSES to write a transcript that did not reproduce under a second seed, so
  nondeterminism cannot be baked into a golden silently.
- **`seed-exploration-asserts-invariants`** — hostsim's world-GENERATING use of a seed (forged
  verdicts, probe flakiness, transport faults) is the same mechanism under a different assertion
  mode: invariants, not bytes. It is not a loom, and a loom over a generated world has pinned its
  generator by committing its transcript (§2 `model-facts-are-data`).

## §7 — prose, provenance, and what the in-process world must express

- **`prose-editability-rides-the-in-process-driver`** — `dorc-loom publish` compiles a prose
  edit from the stamped provenance the in-process driver emits for the exact invocation
  (`282:rul-replay-editability-is-provenance`). Consequence, stated as a requirement rather than
  a gap: every prose-bearing surface a loom renders must be EXPRESSIBLE in the in-process world.
  The receipt-rooted `why` surface is the first surface to fail that test; the in-process
  receipt world (§11 lane C) is what makes its thirty-seven `[unwritten:]` label rows authorable.
- **`prose-laws-unchanged`** — `render-form-unwelded`, `error-authorship-tier`,
  `prose-pins-live-where-the-prose-does`, and the ownership-union rule all stand. This document
  changes where a case's RUN is spelled, never who may write a sentence.

## §8 — dogfood

- **`dogfood-the-sh-engine`** `[TYPED hard ack, two hard conditions]` — the session model
  (parsing the `$` lines, the exported environment, `cd`, redirects, const-propagated values)
  uses our own parser / env model / rho / const-prop wherever that does not chafe. HARD NACK if it
  would soften any correctness invariant of the kernel; HARD DEFER if it would need invasive
  kernel changes — in which case the ideal picture is RECORDED (§10 `front-dogfood-ceiling`)
  for the next kernel arc rather than approximated.

## §9 — what "not supported" means here

- At the loom tier: nothing. A session line the in-process driver cannot run is not "unsupported";
  it is a reason the session runs in a shell.
- At the in-memory tier: a closed set, named by typed declines, that only ever SHRINKS.
- No list, roster, or lexical meta-test enforces either statement `[TYPED]`; the discipline is
  editorial — aim high, and cede ground only when the economics force it.

## §10 — open fronts (not TODO rows; carried into the next review or the next kernel arc)

- **`front-age-nondeterminism`** — the `age` crate draws its ephemeral key and nonce from its own
  RNG inside the adapter (`receipt-crypto` `inv-adapters-do-not-own-policy-or-io`, quarantined
  rationale). Seeded seams make keys and ids deterministic; rich receipt FILE BYTES stay
  nondeterministic. Renders print ids, key-ids and decrypted content, never ciphertext, so goldens
  hold unless a surface prints a digest over whole document bytes (the required-placement landing
  digest is one candidate) — measure first. Every route is unattractive (stub the encryption · a
  production bypass · leave it untestable); the human carries it into review `[TYPED]`.
- **`front-dogfood-ceiling`** — where §8 defers, the ideal is written here.
- **`front-lexical-roster-stands`** — `the_source_comparison_seat_is_the_only_one` stands: "one
  implementation across two crates" is not type-expressible (sealing the trait in `receipt` would
  forbid `cli`'s own impl), and it was minted at explicit direction; existing fences stand.
- **`front-transport-scripted-column`** — the transport seam's `Scripted(case sections)`
  implementation (per-host sections, `282` §2's `hosts/<name>/…` convention) is the natural home
  for multi-host sessions when the r26 revival needs it; only the column exists today.

## §11 — build planning (the implementation-focused tail; a successor reconciles this against the tree and starts)

### ground truth (verified in code 2026-09-01; names are stable, line numbers are not)

- `crates/cli/tests/e2e.rs::drive_extra_replays` already drives replay blocks 1..N sequentially
  in one materialized dir. Narrow today: a case gets its own roots only when it carries `code:`
  (`Harness::dorc`, `OWN_PROFILE_DIR`; otherwise one suite-wide sandbox); `Harness::dorc` sets one
  constant `DORC_FIXTURE_CLOCK_MS` for every drive (two publishes in one case share an order token,
  so `--receipt-last` is ambiguous by design); `run_replay_block` accepts only `dorc …` with
  `< probe-results.txt` / `> /dev/null`, rc 0, stdout-only; `run_loom` requires block 0 to equal
  the battery's own invocation.
- "Profile" is harness vocabulary for a throwaway roots pair (`tests/sandbox.rs`:
  `ProfileSandbox`, `apply_roots_under`, set through the PLATFORM's own variables). Keep the
  platform-variable route; it is already a seam.
- Entropy seats: `cli/src/receipt_edge.rs` `OsEntropy` (receipt ids) and `cli/src/durable.rs`
  `OsKeyEntropy` (keyset generation), constructed in `cli/src/main.rs` at the plan/round-trip
  publish seat and the apply route; both feed existing trait points (`ReceiptIdEntropy`,
  `KeySecretEntropy`; `EntropyReceiptIds::over`, `EntropyKeysetGenerator::over`). Clock:
  `main.rs::clock_for_invocation` → `RunClock::Ticking { at, step_millis: 0 }`
  (`cli/src/results.rs`; the step field already exists). Posture: `main.rs::stdout_posture`.
  The `Framing::spike` substitution point (`results.rs::admit_fixture_records`) is the precedent
  for a type-fenced fixture seat.
- The in-process driver: `dorc-loom/src/consumer.rs` — `run_engine` (injects `LoomEngineEdges`
  with `RunClock::Absent`), `run_receipt_store_why` (answers every store-reading `why` with
  `Unreadable(ROOTLESS_WORLD)`). The frontmatter vocabulary: `dorc-loom/src/vocabulary.rs`
  `FRONTMATTER_KEYS` (24; `run_lane` flags).
- The batteries, classified: `durable_route.rs` and `recorded_facts_route.rs` spawn the shipped
  binary — the latter already asserts every property the needle gate `scan_why_receipt`
  hardcodes (total surface · `--all` byte-identity · `--json` withhold markers · unmatched address
  refuses inside the answer · explicit file root · `file:line`); both mix state assertions
  (keep, shape (c)) with render needles (become loom goldens). `receipt_route.rs` is in-process
  over injected capabilities (one spawn test; header claim "the binary cannot sign" is stale —
  it can). `spine_baseline.rs` is an `#[ignore]`d build-to-kill instrument (its Cargo stanza says
  delete at the fold review). `definition_frames.rs`, `region_artifacts.rs` are census tests;
  `sh_parity.rs` is pipeline-tier.
- The receipt store's deterministic model: `receipt-local` `inv-every-io-act-is-injected`
  (production and a deterministic model implement one sealed `LocalIo` vocabulary). Verify it is
  exposable to `dorc-loom`; if it is test-only, exposing it is the one boundary question of lane C.
- Invocation plumbing that must follow a runner change: mise `test:e2e`/`test:looms`(+`-quiet`),
  `hk.pkl`'s `e2e` and `loom-hygiene` steps, `internal-tooling/src/bless.rs` (spawns
  `cargo test -p dorc-cli --test e2e`), `cli/Cargo.toml`'s `[[test]]` stanzas.
- `cli/src/apply.rs` `ship_consented_apply` computes `ConsentedApply.{intent, outcome,
  durable_failure}` and the production consumer DISCARDS all three; the pre-dispatch refusal
  words and store locus do survive (`ApplyPlanNotDispatchable { reason, store }`, reusing an
  existing case). The two post-dispatch surfaces — the durable-failure diagnostic and the
  completed apply's intent/outcome identities line — are product behaviour with NO driving
  route: a `dorc apply --host` diagnostic can be transcripted today only through
  `consumer.rs run_remote_apply`, a scripted table keyed on `edge-fault` words that never runs
  an apply. Extending that table is a STOP, never a build — it is the accretion this document
  removes. Both surfaces are lane C's.
- Reconciled 2026-09-02 (`30Xa` §3): the shipped `dorc` reads SIX harness-shaped variables, not
  three — the three above plus `DORC_FIXTURE_NONCE` and the debug-only `DORC_TRANSPORT` /
  `DORC_TRANSPORT_INTERPRETER` (`cli/src/transport_edge.rs`); lane A retires all six, transport
  being a listed seam column. `dorc_receipt_local::model::ModelIo` is ordinary public code, so
  lane C's boundary question is one new Cargo edge (`dorc-loom` depends on no receipt crate
  today). `cli/tests` holds 108 looms and 109 dir-cases (106 with `mocks/`) beside two
  `lint-real-*` dirs driven only by the real-tools lane; the multi-file `X/X.loom` shape has zero
  instances; `tests-critical-law` has zero uses among the 24 keys.

### lanes (serial; one Opus builder per lane; stop-and-report between lanes; each lane ends green on `mise run both gate:full-quiet`; every brief carries the Safety block, step-zero, the comment budget with rip-don't-update, and `AGENTS.for-builders-only.md` first)

1. **`lane-a-seams-and-harness-binary`** (medium) — the `Seams` bundle with per-seam selection
   and one shared environment parser; `compose::run(Seams)` extracted from `main.rs`;
   `bin/dorc-harness.rs`; seeded id/key entropy over a dependency-free generator; the ticking
   harness clock (per-block base offset from the block ordinal, non-zero step); the shipped `dorc`
   loses all six env pins; the ordinary harness type excludes `RealSsh` and production
   roots/persistence and rejects those requests; the e2e runner supplies scrubbed credentials and
   runner-owned roots, then spawns the harness through a `dorc` shim on PATH. Goldens must stay
   byte-identical (`bless:dry` clean — nothing in the current corpus renders an id). CHECKPOINT after
   A: the extraction is the risky refactor and the invariants `inv-division-at-the-narrowest-edge`
   and `loom-syntax-grants-no-production-authority` are judged here.
2. **`lane-b-session-driver-and-rip`** (large) — the shell-session process driver; gates by
   kind, no position rules; own roots per session; the needle gate ripped whole
   (`expect-why-receipt` row, `materialize_loom`'s mapping, `scan_why_receipt`, its discovery
   floor); `why30-receipt-rooted-surface.loom` rewritten as an ordinary multi-block golden
   session; both-streams transcripts (every `run:` loom re-blesses — AUTHORIZED); the batteries
   split by assertion kind into loom goldens and one named state-only home; `spine_baseline.rs`
   and `mise run spine:baseline` deleted; `receipt_route.rs`'s header corrected.
3. **`lane-c-in-process-receipt-world`** (medium; the opaque-adjacent lane) — the in-process
   driver composes the REAL `LocalReceiptEdgeV1` over the deterministic `LocalIo` model with
   seeded entropy and the ticking case clock, so receipt-rooted `why` is a fast editable loom and
   the thirty-seven `why-total-*` rows become authorable through the existing publish loop; the
   varied-seed default with its two affordances; `gate-two-drivers-agree`; and the post-dispatch
   durable report, minted here as the first product surface authored over that world — a
   durable-failure diagnostic carrying the surviving intent (a seeded id, never a fixture
   literal) plus the closed write-step word, and the completed apply's intent/outcome
   identities chrome line — each authored through the ordinary publish loop and each also
   witnessed by a state-only e2e (the store holds an intent and no outcome / holds both) per §3
   shape (c). Whether the failure is a sibling of `durable-receipt-unwritten` or a reason arm
   widening it is a product choice the conductor rules at this lane's checkpoint, never a
   builder call.
4. **`lane-d-one-runner-and-frontmatter-collapse`** (large; mostly mechanical once B exists)
   — merge the runners; derive and report the driver; retire `run:`/`fixpoint:`; collapse
   frontmatter per §5; hk/mise/bless plumbing follows; convert every remaining dir-case to a
   loom through a one-off converter deleted after use, and delete the round-trip runner's dir
   entry with its `NAME=value` marker grammar (`[TYPED 2026-09-02: "no legacy e2e"]`); the
   `lint-real-*` dirs become the real-tools test's own fixture space; the `mise run test`
   doctest noise and the known-broken `yardstick` task are tidied (conductor exceedances,
   acked with the plan).

### lane law `[TYPED]`

All four lanes are in scope. Leave no cruft and no half-completed work; the ONLY legal deferral
is "a clear improvement, deeply wanted, that needs kernel mutation", recorded under
`front-dogfood-ceiling` for the next kernel arc. Nothing else from a cleanup arc becomes a TODO
row. The human ruled (2026-09-02) that the stabilized design clears `/opaque-review` BEFORE
the first build (lane A brushes the receipt family's identity/key semantics; lane C touches
`receipt-local`'s boundary): an ACK continues, anything else returns to the human; the
conductor does not concern itself with anything opaque during design and may break invariants
to reach excellent praxis.

### rip list

`expect-why-receipt` (the 24th `FRONTMATTER_KEYS` row · `materialize_loom`'s mapping ·
`e2e.rs::scan_why_receipt` · its key-specific discovery floor · the three needles in
`why30-receipt-rooted-surface.loom`) · the block-0-must-match rule in `run_loom` · the
`split_whitespace` mini-grammar in `run_replay_block` · the constant clock in `Harness::dorc` ·
the shipped binary's six env pins · the dir-case shape with its marker grammar and the
round-trip runner's dir entry · the `X/X.loom` shape (zero instances) · `spine_baseline.rs`
with its task and Cargo stanza · every
frontmatter key in the §5 collapse · the `looms.rs`/`e2e.rs` split · the stale "cannot sign"
header · the `tolerate`/`RAN_ORDER`-era normalizer vocabulary only if lane D finds it fully
expressible as an export (otherwise it stays, on-target rule).

### steering edits owed at close (Fable-authored, once, in conductor voice)

`crates/cli/CLAUDE.md`: the harness contract re-cut around sessions, gates-by-kind, the derived
driver, `inv-division-at-the-narrowest-edge`, and the apply-host driving route (receipt-side
diagnostics driven over the real edge in-process; scripted `edge-fault` rows are transport-only). `spike/CLAUDE.md`:
`rul-fixture-identity-never-production` re-cut with the public-interfaces reading; the Safety
block's "central e2e runner" sentence renamed to the one runner. `crates/aid/CLAUDE.md`: the
runner pointers and the `seam-tolerated-nondeterminism` rule's spelling. `plans/282` §2/§7:
in-place correction of the two superseded clauses (plans are ahistorical). `spike/CLAUDE.md
flat-test-tree-and-loom-placement`: the dir and `X/X.loom` shapes leave the shape list.
`crates/cli/CLAUDE.md`: §3a lands as law. `CONTRIBUTING.md`: the stale clean-worktree
paragraph.
