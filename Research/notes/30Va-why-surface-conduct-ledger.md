# 30Va — why-surface refresh: conduct ledger

> The r30 receipt arc's engineering close, plus the receipt-backed `dorc why`
> presentation work rehomed out of the quarantined phase. Conductor: Fable-class
> (quarantine-blind by law; builders read `AGENTS.for-builders-only.md` first).
> The reserved design-doc slot for anything directional this arc settles is 30V
> (plans-tier, minted late, only if refinement produces directional rulings).
> This ledger is state, not history; git carries chronology.

## Remit

1. Transition residue of the durable-receipt family (report-API tidy · inert
   legacy-whylog deletion + singular-implementation census · CLI
   vocabulary/help) — builder lane on `ai/r30-receipt`; the builder sites its
   own ledger at the lowest-unused 30R docID.
2. Receipt-backed `dorc why` arrangement/render + replacement user-facing
   cases over the sealed `dorc-receipt::report::RecordedWhyFacts` model —
   built TOWARD a conductor-authored plaintext spec (human-directed
   2026-08-30: the UX must not fall out of in-flight builder direction).
   The spec: `notes/30Vb` (the strawman surface set).
3. Arc close: `gate:arc` from the populated branch · LIVING_STATUS refresh
   (several entries stale: the "nothing upstreamed" line, the
   loom-production-path in-flight entry, the reaped branch inventory) ·
   worktree/branch cleanup.

Out of scope, by direction: everything security-shaped (addressed, closed);
report-only kernel re-derivation (separate authorized round; kernel frozen);
loaded-source locator composition (named uncollected V1 residue).

## State

- Worktree/branch cleanup: DONE (sonnet lane, 2026-08-30) — 9 worktrees
  removed, 12 dead branches deleted, 76 dead-husk directories dumped
  (human-authorized). The five survivors were then triaged (opus lane, same
  day): ALL-DIE, per-commit supersession evidence below; nothing salvaged;
  `ai/r30-salvage-sweep` minted empty and deleted. Deleting the five is the
  human's act (`-D` is hook-blocked for agents): `ai/r30-lane-fruit` ·
  `ai/r30-lane-fruit-2` · `ai/r30-lane-load` · `review-verify-adv` ·
  `review-verify-neutral`. The `30N` §4 hold on `review-verify-adv` is
  discharged and annotated in place.

### banked-branch-triage-dispositions (2026-08-30; every unique commit → DIE)

- `85808626` brace-range catalog entry → `DiagCode::ForLoopBraceRangeRunsOnce`
  + published lock + loom + lint case in main, which also absorbed the lane's
  own later oracle-shape fix and re-derived the lock.
- `73f34fce` zero-site-oracle warning → `DiagCode::OracleMatchedZeroSites` +
  loom + a whole-product round-trip case the lane never had.
- `f32ae2a3` lock re-publish → subsumed by the above re-derivation.
- `461b220a` paste-hygiene diagnostic → `DiagCode::EmittedLineUnsafeForPaste`
  + `plan::render::paste_hygiene_hazards` + cases, all in main.
- `3b2e72d4` five red pins (unknown-source/`$0`/computed operand) → all five
  PROMOTED in main (behavior built, tests green, each marked "né p-x-…,
  promoted").
- `176e0818` sentinel-literal drift demo → landed green with a corrected
  expectation (see the `30N` §4 annotation).
- `1dbca1ab` discarded certifier trips → built + fenced
  (`plan/CLAUDE.md certifier-trip-cleanup-runs-in-every-driver`, `TripSpent`
  witness, lexical producer roster).
- `5e614861` lost sourced assignments demo → same test in main, correctly
  re-housed under `xfail_until("p-x-book-level-dot-locals", …)`.
- `c304dc99` replayed pre-source authority demo → green in main, citing
  `30Mc:finding-transitive-pre-source-replays-as-root`.
- `d3388076` review synthesis ledger → strict subset of `notes/30Mc`, which
  carries citations/worlds/slugs the orphan lacks.
- Residue lane: COMPLETE — `ai/r30-receipt` at `4414af7a` (14 commits,
  +729/−4971), `mise run both gate:full-quiet` green both legs, tree clean,
  builder ledger `notes/30Rk`. Old whylog fully cut over (census: no live
  code, no loom replay, no flag table); report-API tidy landed; receipt flag
  family mechanics + selector-exclusion tests landed; the fence turned out
  never to apply (the mode field reaches no durable — breakpoint claim
  corrected in `30Rk`) and the acked rename landed as `Unstated` anyway.
- Standing rulings pending (human): the root-identity fold into closure and
  the ancestors-only closure walk (both stand as built); whether the now
  dead-ended `SpineInvocation::mode()` (zero callers) should die outright.
- Steering/register recast: LANDED 2026-08-31 (`b3552abd`/`99ebe1b1`/`b44bd5e0`).
  Verification first (the human's suspicion held): the `30Rj`-flagged
  receipt/receipt-local rows and `plan/CLAUDE.md`'s durable-replay section were
  ALREADY current — the genuinely-stale set was `spike/CLAUDE.md` (five spots),
  `core/CLAUDE.md` (the DurableAccount flattening cite), `cli/CLAUDE.md` (five
  spots), `core/src/spine.rs`'s module/`CensusArm`/`ExcludedContent` docs, and
  `plan/src/spine.rs`'s untracked-inventory doc (a dead `plan/src/whylog.rs`
  bullet + a stale "list is empty" sentence). All recast to receipt vocabulary;
  renames carry née tags (`receipt-durable-write-only-report-back` ·
  `receipts-not-a-cache` · `law-receipts-are-sensitive` · `aid-receipt-posthoc-why`
  · `an-receipt-durable` · `inv-receipt-collection-never-expands-observation`).
  The account-export expectation is re-homed in place: excluded-by-absence again,
  pin `p-x-durable-account-export-is-enabled` parked `Reserved`, rebuild =
  receipt-contents change, review-first.
- Fold-time queue (remaining): cli/src doc-comment citers of the née'd slugs
  (`cli/src/lib.rs:536` · `cli/src/results.rs:291`, plus the stale
  `whylog_store`-as-reference comments in `cli/src/artifact_store.rs` and kin —
  deferred while the render lane owns that tree) · prose massage (five unwritten
  receipt-flag help rows; two receipt placeholders; six stale `why:` metadata
  citations in the generated locks; slop-tier ACKED) · LIVING_STATUS final
  refresh · `gate:arc` · end-of-work review per root `AGENTS.md` · worktree
  cleanup (`r30-receipt`/`r30-conduct` now trail `ai/main`).
- Why-surface design: exploration CLOSED and banked — `notes/30V` (the
  re-exploration record: model, doctrine, rulings, §5 build direction,
  §6 owed-list) + `notes/30Vc` (four adjudicated render generations, raw);
  `30Vb` is gen-1, historical. Build-now per `30V` §5: the reconstruction
  plane + an INTENTIONALLY-TEMPORARY total surface (no drawn gutter, interim
  register, replaced without ceremony later); the graph library, settled
  register, and curated tiers deferred to proper process and attention.
- Why-surface build lane: DISPATCHED 2026-08-31 — Opus builder, branch
  `ai/r30-why-surface` (base `f6317f43`, deliberately behind `ai/main`; fold is
  conductor-side); builds `30V` §5 + the arrangement-orphan census rider +
  replacement receipt-rooted why cases; completion contract
  `mise run both gate:full-quiet`; builder ledger sites at lowest-unused
  30V-suffix. The structure checkpoint was adjudicated same-day: new pure crate
  `dorc-why` (edges → receipt + aid only; render/`--json` at the cli edge);
  the `Known`/`Held` wrapper pair; listing-surface retirement RULED (all four
  listing seats die); `Rooted::{Plan, OtherSpecies}` one-face RULED;
  `--receipt <file>` roots-a-question repair in scope; the `.rs` whole-product
  battery substitutes for a loom case (the loom `run:` vocabulary cannot drive
  a store-side route — a widening is a queued TODO row, not this lane).
  Builder finding of record: the receipt durable RECORDS fifteen row families
  but sealed `report::RecordedWhyFacts` PROJECTS seven — most of the
  durable-gap audit is a report-API gap, not a carrier gap
  (`fnd-report-api-carries-seven-of-fifteen`, its ledger).
- rul-report-projection-becomes-exhaustive (OPAQUE-RULED, relayed by the human
  2026-08-31; resolves `tc-model-families-reach-the-reconstruction`). The
  packet, verbatim (its `_tmp-extension-remit.md` carrier is ephemeral):
  "The correct repair is not more persistence, format work, or kernel work.
  Extend RecordedWhyFacts with typed report projections for the existing row
  families: closed recorded tokens remain typed; influence/material
  availability stays independent; opaque operands, shell, locators, and
  details use the existing RecordedValue encoder exit; no raw
  RecordedPlanReceipt or overlay accessor is exposed. Ideally the report
  projection becomes exhaustive over the durable model: every persisted family
  is either represented by a typed facts collection or explicitly classified
  as unavailable/not relevant. That gives 30V and later aid work one complete
  public read surface without reopening receipt internals." Relayed to the
  render lane with scope interpretation: read-surface projection work only
  (no grammar/writer/wire/projection-state/provider change); the fifteen plan
  families are the core mandate, intent/outcome rows in-scope where the
  `Rooted` face needs them; a closed no-wildcard family classification so an
  unclassified sixteenth cannot land; any need to reopen internals is a STOP.
  The cli-side decomposition route stays forbidden.
- Render lane, second builder (2026-08-31, wound down at budget, endorsed):
  `712ab177..8646a4f6` — ALL fifteen plan families projected
  (`FamilyCoverage::RecordedButUnprojected` unreachable except by removing a
  projection); `cli::why_total` (Coverage at the emit site; exclusion ledger
  empty by uninhabited type; weft-woven); `why_json`
  (`dorc-why-json/unstable`; state+value keys, never absent); 37 arrangement
  rows seeded unwritten; both legs green at tip. UNBUILT then, spec sharpened
  in `30Vd:what-remains`, since BUILT by the third builder: item 4 (cli wiring: facts_for production caller,
  listing-seat deletion, `ReceiptRoot::File` repair, whole-store-listing drop,
  digest-matched address resolution) + item 5 (publish→why e2e cases + the
  minimal runner extension). New flags: `tc-licensor-custody-speaks-its-own-act`
  (lane-ledger-minted; CONDUCTOR-RULED: custody→act mapping stands — engine-voice `Derived` would
  dress an authored judgment as derivation; refinement owed in the wiring lane:
  `vouched-severally` populates the voice-set as an inseparable committee of
  unnamed voices, not a single unnamed voice) ·
  `tc-nonplan-root-depth` re-sharpened (CONDUCTOR-RULED: shallow fill of
  `NonPlanRoot` from the already-public intent/outcome accessors now; deeper
  intent/outcome family projection deferred with a `NotRelevant`-tier
  classification and a flag — it would need new sealed accessors, a fresh act
  under the same law, not this arc's).
- rul-source-comparison-is-one-cli-seat (OPAQUE-RULED, relayed by the human
  2026-08-31; resolves the file-line address question — the
  `_tmp-recorded-source-path-handoff.md` carrier is ephemeral). Packet,
  verbatim: "dorc-receipt validates/classifies recorded source material, then
  releases one narrowly typed source-comparison packet to a single CLI seat.
  Comparison and filesystem behavior live outside receipt. Receipt API:
  RecordedSourceMaterial::visit_for_comparison(SourceComparisonConsumer). The
  visit carries exact recorded path bytes, source identity/digest, exact
  recorded general-sh content where held, locator spans, and
  authentication/material state. This is source-specific — not a generic raw
  RecordedValue accessor — and none of these values gain Display, revealing
  Debug, serde, or ambient formatting. One production consumer in CLI owns:
  platform-aware path rehydration; bounded, non-following regular-file reads;
  exact current/recorded path and content comparison; current
  same-physical-line policy; future diff/content-aware correspondence
  policies; destination encoding for anything displayed. The consumer must
  distinguish locally authenticated receipts from imported/self-asserted
  material. Receipt-provided paths never trigger implicit reads from the
  latter; an explicit user-named file may still be compared. Mechanically
  enumerate the one implementation and callsite as hygiene. Do not move
  filesystem I/O or matching policy into receipt, and do not widen arbitrary
  opaque-detail access. Future comparison features extend this one CLI seat
  rather than adding receipt APIs or scattered raw path exits." Relayed
  mid-lane to the wiring builder with: `file.sh:N` now BUILDS (path bytes +
  line computed from spans over exact recorded content, at the seat,
  display through destination encoding; ordinal-and-span stays the fallback
  where the visit holds no content); the authentication asymmetry as stated;
  the hygiene enumeration structural-first, lexical only as the
  packet-requested (hence human-ack'd) exception.
- Render lane, third builder (2026-08-31): COMPLETE — items 4+5 + both
  packets + the orphan instrument, eight commits, both legs green at tip. The
  lane is BUILT in full; rebased onto `ai/main` for the fold. Conductor
  adjudications of its disclosures: `--json`-beside-`--results` refusal
  ENDORSED (a silently-ineffective flag is exactly what the human would flag)
  · ambiguity-explains-nothing ENDORSED (30R's no-arbitrary-tie-break, a
  disclosed narrowing) · signing-key-off-the-surface ENDORSED (skeleton
  material, not report material) · `tc-correspondence-falls-back-to-content`
  ENDORSED (seat-owned policy per the comparison packet) ·
  `tc-file-root-order-comes-from-the-name` STANDS-AS-BUILT with a banked
  lean: the honest end-state is probably a typed order accessor on the read
  surface (projection of a persisted closed token, within the
  exhaustive-or-classified law) — a later small act, not this arc's ·
  `tc-address-refusal-is-a-datum-not-a-diagnostic` HELD FOR THE HUMAN
  (conductor recommendation: a NO-MATCH address is an answer, not ambiguity —
  the datum-row satisfies 30R's render-the-rest; the `[TYPED]`
  fails-fast-on-ambiguity rule should bind the MULTI-match case, which should
  stop-and-ask; split accordingly if the human agrees). Residue rows for
  TODO-ADDTL at close: the unwitnessed authentication-asymmetry and
  non-following-read branches (no fixture world reaches them) · the
  unwitnessed `vouched-severally` committee arm · the loom `run:` vocabulary
  widening (deferred) · the five orphan arrangement rows awaiting the human's
  deletion word (`syntax-unsupported-source-of-dynamic-target` ·
  `why-drift-address-unanswerable` · `why-drift-analysis-suppressed` ·
  `why-receipt-plan-tally-unsplit` · `why-receipt-when-replayed`).
- Conductor prose pass (2026-08-31, slop-tier per ack, on the lane): the six
  receipt/machine-register help rows, both receipt-code messages
  (`{{store}}`/`{{reason}}`-holed), three register-guide `why:` recasts (the
  dead "whylog is a deterministic reproducer" guidance now points at the
  published receipt), the dorc-loom usage example slug, and the two usage
  transcripts refreshed via the dump-rescue loop. Steering sited on the lane:
  `cli/CLAUDE.md source-comparison-is-one-cli-seat` ·
  `why/CLAUDE.md file-line-addresses-come-from-the-visit`. NOT authored: the
  37 `why-total-*` label rows — structural finding: their only rendering
  surface carries per-run entropy (receipt ids), so no committed transcript
  can be their fixpoint-authoring home and the loom words-mint cannot reach
  them; they rest `[unwritten:]` legally, and the clean future fix is a
  synthetic-world loom case rendering `why_total` over an entropy-free
  reconstruction (a later sitting's machinery). Two lock `why:` rows keep
  historical fence-names by their old spellings deliberately
  (`nack-whylog-stores-book-bytes`, the 27V clock cite) — history, not drift.
- Worktree-reap incident (2026-08-31, resolved; the `worktree-file-access-law`
  hazard fired as documented): the harness reaped the builder's managed
  worktree at its checkpoint turn-boundary; the orphaned skeleton's dead `.git`
  pointer made bare git answer FROM THE PRIMARY. The builder's step-zero
  discipline held — read-only commands only, stopped and reported; primary
  verified clean; zero lane commits existed so nothing was lost. Resolution:
  conductor-minted persistent worktree `.claude/worktrees/why-surface` (not
  harness-managed ⇒ immune to return-triggered reaping), residue verified
  zero-file and deleted. Lesson already durable in steering; no new law.
- Cleanup queue addendum: `.claude/worktrees/` still holds ~36 `agent-*` husks
  and ~14 named directories that are NOT registered worktrees (`git worktree
  list` knows only play/r30-conduct/r30-receipt/why-surface) — presumably the
  same empty-husk class the 2026-08-30 sweep dumped. Verify zero-file, then
  dump at arc close.

## Close (2026-08-31)

The arc is CLOSED: `gate:arc` green (rc=0; both-platform completion + arc-tier
verifiers + advisories) from the populated lane, then `ai/main` fast-forwarded
to the lane tip (`ea295267`). One post-gate red was caught and fixed in-lane
(the six worded help rows joined `cli-help-page.loom`'s `owns:`). The
opaque-review gate stayed builder-initiated: all three builders read
`AGENTS.for-builders-only.md` first and none instructed loading it. Remaining
human items ride `TODO-ADDTL:why-surface-close-residue` + the round-close
ceremony; worktrees/branches cleaned at close.

## HANDOFF — the test-architecture rebuild (design CONVERGED 2026-09-01 in a Fable⇄human sitting; NOT dispatched — the human rewinds/restarts before pursuing; the next conductor starts HERE)

The arc's PRODUCT is sound and folded; its TEST ARCHITECTURE is to be rebuilt, not patched. This
section is the brief. It supersedes the earlier handoff in full (that text's ground-truth errors are
corrected under `handoff-ground-truth`). Nothing was built under either.

### handoff-ground-truth (verified in code 2026-09-01, not from builder reports)

- **Sequences already half-exist.** `crates/cli/tests/e2e.rs::drive_extra_replays` drives replay
  blocks 1..N sequentially in the one materialized dir ("a run that publishes, and the later
  invocation that reads what it published"). What is narrow: (a) a case gets its OWN roots only
  when it carries `code:`; every other drive publishes into one suite-wide sandbox
  (`Harness::dorc`, `OWN_PROFILE_DIR`); (b) the clock pin `DORC_FIXTURE_CLOCK_MS` is one constant
  for every drive, so two publishes in one case share an order token and `--receipt-last` is
  ambiguous by design; (c) blocks ≥1 accept only `dorc …` + `< probe-results.txt` / `> /dev/null`,
  rc 0 required, stdout-only transcript (`run_replay_block`); (d) block 0 MUST equal the invocation
  the gate battery drives (`run_loom`) — an artifact of the battery being a fixed choreography.
- **"Profile"** is harness vocabulary only: a throwaway pair of per-user roots (config root → the
  receipt keyset; state root → the receipt store), pointed at by the PLATFORM's own variables
  (`XDG_*`/`APPDATA`/`LOCALAPPDATA`/`HOME`; `tests/sandbox.rs`). It exists because receipts are
  default-on (`28F:rul-w3-default-on-aim-high`) and a suite run once minted a real keyset into a
  developer's real `~/.config/dorc`.
- **The subprocess tier was never DST.** DST lives in-process (pure kernel, seeded hostsim, the
  loom driver's injected values). The e2e corpus spawns the real binary, whose `main.rs` re-pins
  every seam to the OS, and the harness reaches in only through env pins that accreted one leak at
  a time: clock (`DORC_FIXTURE_CLOCK_MS`, a FROZEN wallclock — `RunClock::Ticking{step_millis:0}`),
  terminal posture (`DORC_STDOUT_POSTURE`), git source-match (`DORC_FIXTURE_SOURCE_MATCH`); entropy
  never. Two `getrandom` seats: `cli/src/receipt_edge.rs` `OsEntropy` (receipt ids) and
  `cli/src/durable.rs` `OsKeyEntropy` (keyset generation), constructed in `cli/src/main.rs` at the
  plan/round-trip publish seat and the apply route; both feed trait injection points the crates
  already carry (`ReceiptIdEntropy` / `KeySecretEntropy`, `over(entropy)`).
- **The batteries were miscounted.** `receipt_route.rs` is in-process with injected capabilities
  (one spawn test; its header "the binary cannot sign" is STALE — `durable_route` proves it can);
  `spine_baseline.rs` is an `#[ignore]`d build-to-kill smoke instrument whose Cargo stanza says
  delete at the fold review; the genuine spawn-the-binary batteries are `durable_route.rs` and
  `recorded_facts_route.rs` — and the latter ALREADY asserts every property `scan_why_receipt`
  hardcodes (total surface · `--all` byte-identity · `--json` withhold markers · unmatched address
  refuses inside the answer · explicit file root · `file:line`). The needle gate was a third home.
- **Labels need more than determinism.** `dorc-loom publish` attributes prose edits through the
  IN-PROCESS driver's stamped provenance, and that driver answers every store-reading `dorc why`
  with a hard-coded `Unreadable(ROOTLESS_WORLD)` (`dorc-loom/src/consumer.rs::run_receipt_store_why`;
  `RunClock::Absent`; nothing is ever published in-process). Deterministic ids alone do not make
  the 37 `why-total-*` rows authorable; an in-process receipt WORLD does (lane C).
- **Six test shapes exist today**, two of them unnamed: unit · in-process looms · `run:` looms
  and dir-cases (e2e) · Rust-authored batteries · corpus-wide CENSUS tests (`definition_frames.rs`,
  `region_artifacts.rs`: universally-quantified properties over the case population — the
  executable form of `30A` quality-is-a-ratchet; legitimate, keep) · PIPELINE-TIER authored-world
  tests (`sh_parity.rs`, much of `receipt_route.rs`: an inline Rust world driven through the real
  pipeline, asserting a typed internal decision — legitimate today, with a standing pull toward
  looms as the `why`/`--json` surface becomes total over decisions).

### handoff-target-architecture (the human's typed rulings are in the ack-ledger)

- **`arch-one-world-many-drivers`** — a case IS a world (sources, declared facts, seams) plus an
  ordered shell session and its exact output. Every nondeterministic edge the engine consumes is a
  typed value in ONE bundle (strawman `Seams`: clock · id entropy · key entropy · stdout posture ·
  roots/env · transport · future columns — process supervisor, netns, sudo prompt …), and every
  seam independently selects an implementation — `Seeded(seed)` / `Pinned(value)` / `Os` /
  (transport) `Scripted(hosts/…)` / `Hostsim` / `RealSsh`. A tier is a ROW of that matrix, never a
  separate harness: unit = models everywhere; in-process loom = models + seeded; harness binary =
  real filesystem + seeded everything else; livetest = real ssh. A new seam is one column added
  once; every driver inherits it — the anti-accretion property the human asked for ("more abstract,
  more powerful; the general structure must COVER the multi-process/multi-host universe, not be
  built by accretion"). Build only the columns with implementations in hand; the STRUCTURE is the
  deliverable. Per-seam UNPEG is native: any seam may be set to `Os` by a case that owns the
  consequence (that value can no longer golden).
- **`arch-harness-binary-not-produced-cli`** — runtime injection into the binary-under-test is
  required (process exit is part of the property), but it lives in a SIBLING BUILD, never the
  shipped `dorc`: `cli/src/bin/dorc.rs` = `exit(compose::run(Seams::os()))`;
  `cli/src/bin/dorc-harness.rs` = `exit(compose::run(Seams::from_env(&real_env)))`, refusing loudly
  when no seam env is set. Same crate, same lib; all arg parsing, source acquisition, root
  resolution and the receipt edge live BELOW the seam. The e2e runner spawns `dorc-harness` (as a
  `dorc` PATH shim, so a transcript still reads `$ dorc …`). The shipped `dorc` LOSES its three env
  pins. Under the human's sharpened reading of `rul-fixture-identity-never-production` ("anything
  published" = PUBLIC INTERFACES: no fixture durable-state may be producible by any code path
  typeable into `cli/main.rs`), a second bin is the exact structural expression; a cargo feature
  also satisfies it but drags `--features` through six repo-owned call sites, and a forgotten flag
  either reddens id-bearing goldens or, under `required-features`, silently skips the corpus.
- **`inv-division-at-the-narrowest-edge`** (human-typed; becomes steering at close) — the shipped
  `main.rs` is edge VALUES plus one call; anything with a branch, a parse, or a decision belongs
  below the seam. Everything above the seam is testable only by the Rust-authored e2e over the
  shipped binary, so that remit is bounded (the ~ten lines + "the OS impls are live") and must
  never grow. Binds `dorc-sh` too. Site in `cli/CLAUDE.md`; a mechanical no-control-flow check
  only as a human-ack fence.
- **`arch-looms-are-shell-sessions`** — a loom's replay is a POSIX shell session and NOTHING about
  block position, ordering, or content is restricted; gates attach to blocks BY WHAT THE BLOCK IS
  (the product's own arg parser classifies each `$ dorc …` line; artifact-producing blocks get the
  artifact gates — dash `-n`, exec-under-mocks with a per-artifact run-set, guard-shape, redirects,
  argv-echo, dual-rail; every block gets the diagnostic gates), never by position. The PROCESS
  driver is literally a shell: materialize the case, start `sh` (via `internal_tooling::Posix`) in
  the temp dir with the harness env and `dorc`→`dorc-harness` on a shim PATH, feed the `$` lines
  through a sentinel-delimited persistent session (the records lane's own trick), capture per line.
  `export`, `cd`, pipes, `cat`, `echo $?`, `chroot` — all native. The IN-PROCESS driver is a perf
  OPTIMIZATION over a closed grammar it can run in memory (parse those lines with `dorc_syntax`,
  not a hand grammar — dogfood); a typed decline anywhere routes the WHOLE session to the shell.
  "Not supported" is a closed-and-shrinking set at the in-memory tier and empty at the loom tier
  (no meta-test for that; aim high, cede ground only as economically necessary).
- **`arch-seams-are-sh-lines`** — seam selection is spelled as sh IN the session:
  `$ export DORC_SEAM_CLOCK=seeded:7` (one var per seam; a `DORC_SEED` umbrella is fine). The
  harness binary reads the real environment; the in-process driver reads a MODELLED session
  environment; ONE parser (`Seams::from_env(&dyn EnvReader)` — the `RootEnvironment` DI reader is
  the precedent) serves both. This deliberately REVERSES `282` §2's "harness-only environment must
  not appear" for seam selection: seams are a documented typed surface, not fixture authority.
  Defaults for every loom: cwd = the materialized dir, stdin = the block's redirect or null,
  every seam seeded-varied unless exported otherwise.
- **`arch-transcript-is-what-the-user-saw`** — both streams, in order (`2>&1` at the session; the
  in-process driver already emits ordered events for both). `run:` looms commit stdout-only today,
  so all 27 re-bless (AUTHORIZED). The artifact gates re-drive their subject block with split
  streams for parsing — legitimate because a seeded invocation driven twice is byte-identical.
- **`arch-varied-seed-default-declared-seed-opt-in`** — the runner varies every seeded seam on
  every run (a fresh render is a free invariance test); a case that commits a transcript depending
  on any seeded value PINS it (`$ export DORC_SEED=7`) — regression is an opt-in, per case, in
  every tier (looms, e2e, unit: one seat, one spelling). World FACTS are declared case data and
  are never varied (a loom over hostsim-GENERATED facts has, by committing its transcript, already
  pinned). Two affordances so intermittent reds never become agent-retry fodder: the run-wide seed
  prints at the start and is named in every failure with the one-line pin/replay spelling
  (hostsim `replay-seed`), and bless REFUSES to write a transcript that did not reproduce under a
  second seed.
- **`arch-one-runner-driver-derived`** — `looms.rs` + `e2e.rs` merge into one runner over one walk;
  which driver proves a session is DERIVED (in-process when the closed grammar and the world allow,
  shell otherwise) and REPORTED, never declared. Where both can run a session, both do and must
  agree byte-for-byte (`gate-two-drivers-agree` — today nothing checks that the in-process render
  `publish` attributes against matches the bytes the binary proved). `one-fixpoint-authority-per-case`
  is superseded by "the shell proof is authoritative where it runs; the in-process render is a
  second witness."
- **`arch-frontmatter-collapses`** (only where CLEARLY better; stay on-target, no rip-and-tear) —
  `flags` → the `$ dorc` line · `exit`/`apply-exit` → `$ echo $?` · `probe-results` → the `<`
  redirect · `why-addr` → `$ dorc why book.sh:4` · `artifact-set` → `--artifact-dir` on the line ·
  `tolerate` → an export · `expect-diagnostic`/`expect-why`/`expect-hint`/`expect-why-chain` → the
  transcript (the catalog-validation of `[slug]` headers stays as a runner check over transcript
  bytes) · `run:`/`fixpoint:` → derived. Survivors are registry metadata: `code` · `arrangement` ·
  `owns` · `when-fires` · `when-used` · `why` · `envelope` · `tests-critical-law` · `todo`. 24 → 9.
- **`arch-e2e-is-the-tier`** — e2e = the product mechanized above unit level, with three shapes:
  looms (default — tests AND prose from one durable), dir-cases (legacy, converting), Rust-authored
  batteries (arbitrarily complex setup; STATE AND EXITS ONLY, never render bytes; may spawn the
  shipped `dorc` with OS seams for "the shipped binary did X" claims, or `dorc-harness` seeded).
  `durable_route.rs`/`recorded_facts_route.rs` split by assertion kind: render needles become loom
  goldens, filesystem/negative cells stay Rust e2e in one named home.
- **`arch-dogfood-the-sh-engine`** (human hard-ack, with two hard conditions) — use our own
  parser / env model / rho / const-prop for the session model where it does not chafe. HARD NACK if
  it softens any correctness invariant; HARD DEFER if it needs invasive kernel changes — in which
  case record the ideal picture for the next kernel arc rather than approximating it.

### handoff-lanes (serial; one Opus builder per lane; stop-and-report between lanes; every lane completes with `mise run both gate:full-quiet`; comment budget + rip-don't-update on every brief)

1. **`lane-a-seams-and-harness-binary`** (medium) — the `Seams` bundle with per-seam selection;
   `compose::run(Seams)` extracted from `main.rs`; `dorc-harness` bin; seeded id/key entropy (a
   dependency-free splitmix/LCG over the seed — hostsim's `lcg-only-entropy` posture) feeding the
   existing trait points; a TICKING harness clock (`step_millis` finally non-zero; per-block base
   offset derived from the block ordinal); `Seams::from_env` parser; the shipped `dorc` loses its
   three env pins; the e2e runner spawns the harness via a `dorc` shim. Goldens must stay
   byte-identical (`bless:dry` clean) — nothing in the current corpus renders an id.
   CHECKPOINT after A (the extraction is the risky refactor).
2. **`lane-b-session-driver-and-rip`** (large) — the shell-session process driver;
   gates-by-kind with NO position rules; own roots per session; the needle gate ripped whole
   (`expect-why-receipt` row + `materialize_loom` mapping + `scan_why_receipt` + its discovery
   floor); `why30-receipt-rooted-surface.loom` rewritten as an ordinary multi-block golden session;
   both-streams transcripts (27-loom re-bless); the batteries split by assertion kind;
   `spine_baseline.rs` + `mise run spine:baseline` DELETED; `receipt_route.rs` header corrected.
3. **`lane-c-in-process-receipt-world`** (medium; the opaque-ADJACENT lane) — the loom driver
   composes the REAL `LocalReceiptEdgeV1` over `receipt-local`'s deterministic `LocalIo` model
   (`inv-every-io-act-is-injected`; verify the model is exposable — if it is test-only, exposing it
   to `dorc-loom` is the boundary question) with seeded entropy and the ticking case clock, so
   receipt-rooted `why` is a fast editable loom and the 37 `why-total-*` rows become authorable
   through the existing publish loop; the varied-seed default with its two affordances;
   `gate-two-drivers-agree`.
4. **`lane-d-one-runner-and-frontmatter-collapse`** (medium-large, mostly mechanical once B's
   session driver exists) — merge the runners; derive the driver; collapse frontmatter per the
   list above; retire `run:`/`fixpoint:`; hk/mise routing follows (`hk.pkl` `e2e`/`loom-hygiene`
   steps; `internal-tooling` `bless.rs` spawns `cargo test -p dorc-cli --test e2e`).

**Lane law (human-typed):** D is IN SCOPE. Leave no cruft and no half-completed work. The ONLY
legal deferral is "clear improvement, deeply wanted, but needs kernel mutation" — recorded as the
ideal picture for the next kernel arc. Nothing else from this arc becomes a TODO row. Product work
and suite work stay fully separate (non-concurrent), which is what licenses the re-bless.

**Before any building step:** the human decides whether this design goes through
`/opaque-review` (it may be owed — the receipt family's identity/key semantics are brushed by
lane A, and lane C touches `receipt-local`'s boundary); the conductor does not concern itself
with anything opaque mid-design and may break invariants to reach excellent testing praxis; the
review, if owed, precedes building.

### handoff-open-fronts

- **`front-age-nondeterminism`** — the `age` crate draws its ephemeral key and nonce from its own
  RNG inside the adapter (`receipt-crypto` `inv-adapters-do-not-own-policy-or-io`); seeded seams
  make KEYS and IDS deterministic but rich receipt FILE BYTES stay nondeterministic. Renders print
  ids/key-ids/decrypted content, never ciphertext, so goldens should hold UNLESS a surface prints a
  digest over whole document bytes (the required-placement landing digest is one candidate) —
  the builder measures first. Every route is unattractive (stub the encryption · a production
  bypass · leave it untestable); the human carries it into review. An open front, not a TODO row.
- **`front-dogfood-ceiling`** — where using the kernel's env model / const-prop for the session
  model would need invasive kernel changes, record the ideal here for the next kernel arc.
- **`front-lexical-roster-stands`** — `the_source_comparison_seat_is_the_only_one`
  (`receipt/tests/crate_boundary.rs`) stands: "one implementation across two crates" is not
  type-expressible (sealing the trait in `receipt` would forbid `cli`'s own impl), and it was
  minted at explicit direction. Existing fences stand.

### handoff-rip-list (updated)

`expect-why-receipt` (the 24th `FRONTMATTER_KEYS` row · `materialize_loom`'s mapping ·
`e2e.rs::scan_why_receipt` · its key-specific discovery floor · the three needles in
`why30-receipt-rooted-surface.loom`) · the block-0-must-match rule in `run_loom` · the
`split_whitespace` mini-grammar in `run_replay_block` · the constant clock in `Harness::dorc` · the
shipped binary's three env pins · `spine_baseline.rs` + its task + its Cargo stanza · every
frontmatter key in the collapse list · the `looms.rs`/`e2e.rs` split.

### handoff-resumption-pointers

Seats by path (line numbers drift; names do not): `cli/src/main.rs` `clock_for_invocation` ·
`stdout_posture` · the two `Os*Entropy` construction sites; `cli/src/receipt_edge.rs` `OsEntropy`;
`cli/src/durable.rs` `OsKeyEntropy` + `standard_roots`/`RootEnvironment`; `cli/src/results.rs`
`RunClock` + `admit_fixture_records` (the `Framing::spike` substitution-point precedent);
`cli/tests/e2e.rs` `Harness::dorc` · `run_loom` · `drive_extra_replays` · `run_replay_block` ·
`scan_why_receipt` · `materialize_loom` · `bless_loom`; `cli/tests/sandbox.rs`; `cli/tests/looms.rs`;
`dorc-loom/src/consumer.rs` `run_receipt_store_why` · `run_engine` · `LoomEngineEdges`;
`dorc-loom/src/vocabulary.rs` `FRONTMATTER_KEYS`; `cli/Cargo.toml` `[[test]]` stanzas; `hk.pkl`
corpora steps; `internal-tooling/src/bless.rs`. Prior design authority the lanes must honor:
`plans/282` (the loom pipeline; §7 is the execution harness), `plans/128` (DST: Seam-1 is the
controller↔host session), `crates/cli/CLAUDE.md`'s harness contract, `crates/aid/CLAUDE.md`'s
catalog/ownership law, `spike/CLAUDE.md` `rul-fixture-identity-never-production` (to be re-cut with
the "public interfaces" reading at close). Lane ledger of what the arc built: `notes/30Vd`.

## Ack-ledger (only what the human has TYPED counts)

- ack-cleanup-authorized (2026-08-30): dead branches/worktrees deleted, husks
  dumped unceremoniously; human `-D`'d the two dangling receipt-arc branches
  themselves.
- ack-30v-docid-reservation (2026-08-30): 30V = final design doc for
  directional tweaks; 30Va = this ledger; builder residue ledger sites under
  30R at lowest-unused suffix.
- ack-conductor-authors-the-spec (2026-08-30): the conductor hand-builds the
  prospective `dorc why` surfaces as TUI-as-spec plaintext; builders build
  toward it.
- Everything in `30Vb` is UNACKED strawman until adjudicated; its ask-list is
  the adjudication queue. (Superseded 2026-08-30: the whole ask-list was
  overtaken by the sitting's rulings, banked with `[TYPED]` tags in
  `notes/30V` §2 — that section is now the authoritative ack record for the
  why-surface direction.)
- ack-slop-prose-authorized (2026-08-31): the conductor may author the owed
  prose at Slop tier (the five receipt-flag help rows, the two receipt
  placeholders, the six stale lock `why:` citations), full-loom reads owed.
- ack-ancestors-only-walk-correct-as-built (2026-08-31): receipts are
  never-append, so forward links cannot exist in the files; walking from the
  explicitly-named root toward causes is correct. Selector conveniences like
  `--receipt-last` may enumerate/search a bounded store; that is graph
  discovery, not a union of histories.
- ack-steering-suspicion-confirmed (2026-08-31): the human suspected the
  receipt-crate steering rows were already written and the `30Rk` note stale —
  verified true (see the recast bullet in State).
- Root docs: the human committed their own USER_STORY re-cut (`1fc95c45`) and
  TODO-ADDTL row (`f6317f43`) mid-arc; both root docs stay conductor-untouched.
- ack-root-identity-fold-soft (2026-08-31): stands as built; SOFT ack only —
  the human has not investigated deeply. Not settled law; revisitable.
- ack-mode-deletion (2026-08-31): `SpineInvocation::mode()` dies outright —
  landed conductor-side: `core::spine::InvocationMode` deleted, `minted()`
  lost its mode parameter, five callers fixed (incl. `cli/src/receipt_edge.rs`
  — a known trivial fold-conflict candidate with the render lane).
- rul-no-reflexive-lexical-fences (human-typed 2026-08-31; SCOPE SHARPENED
  same day): the target is grep-gates as BACKSTOPS TO TYPECHECKING —
  agent-maintainable ones are ~valueless (the agent edits both sides) and
  disincentivize proper typing discipline; the valuable ones are exactly the
  human-requested, human-ack-only ones. Scans over inherently-textual
  material (declared-source rosters, frontmatter, literals) are legitimate
  INSTRUMENTS — but instruments report; deleting a user-facing string on
  their say-so stays a conductor-at-least, human-maybe decision (human lean,
  typed). Prefer types/structure; flag up where only a lexical backstop
  would serve. NOT a removal directive — existing fences stand. Durable:
  `spike/CLAUDE.md lexical-fences-are-human-ack-instruments`; relayed to the
  render lane (structural mechanisms built). Consequence: the
  orphan-arrangement census is UN-WITHHELD, redirected to a report-only
  doctor-family instrument (never a gate, never a deleter), last in the
  wiring lane's priority order.
- rul-whole-product-proof-rides-the-e2e-corpus (human-typed 2026-08-31): no
  new test TYPE — the publish-then-why whole-product proof lives as e2e-corpus
  cases, with a minimal e2e runner/case-shape extension authorized if the
  grammar cannot yet express a two-invocation sequence; the standalone
  `why_surface_route.rs` battery is dropped. The loom `run:` widening stays
  deferred. The `crates/why` unit/integration gates remain ordinary Rust
  tests. (SUPERSEDED IN PART 2026-09-01 by the HANDOFF section: the builder
  satisfied this ruling's letter with a runner-hardcoded needle gate, which the
  human ruled cruft to rip.)
- ack-test-architecture-rip-owed (human-typed 2026-09-01): "I'm shocked sequences
  are not modeled. Yes, this is absolutely owed … a lot of cruft-rip is needed
  here." The HANDOFF section is the directive's record; the human clears context
  before the fix proceeds.
- The 2026-09-01 design sitting (Fable conductor; every item below TYPED by the human):
  - ack-published-means-public-interfaces: "anything published" in
    `rul-fixture-identity-never-production` = PUBLIC INTERFACES; the narrow reading is
    "a durable-file feature/state we never want for production users shouldn't be
    producible by any code-path typeable into `cli/main.rs`" — NOT that such state may
    not exist (that would rule out deterministic testing).
  - ack-separate-harness-binary: "ack on separate binary-name, I see no downsides; but
    the division must be right out at the furthest, narrowest edge, because everything
    before the division is effectively untestable without inventing a sixth kind of
    test. that needs to become an invariant."
  - ack-e2e-is-the-tier: the state-only Rust batteries are e2e (the catch-all for
    testing the product above unit level that isn't a loom); looms are the editable
    subset; dir-cases should slowly become rare.
  - ack-tooling-never-deferred: good tooling work is always owed and never deferred
    unless catastrophically huge; it just cannot be done in flight or in parallel — so
    finding cruft OUTSIDE an arc is the moment to do it. Lane D is in scope; the only
    legal cruft is "clear improvement, deeply wanted, needs kernel mutation"; nothing
    from a cleanup arc becomes a TODO row.
  - ack-breadth-more-abstract: the testing-target universe is every seam of multiple
    unix processes on machines Doing Things; the general structure must COVER it, not be
    built by accretion — "more abstract, not less; more powerful, not smaller."
  - ack-unpeg-per-seam: separate flags/env per determinism seam, so determinism can be
    selectively un-pegged.
  - ack-looms-are-shell-sessions: no artificial restrictions on structure, ordering, or
    content; in-memory execution is a suite-perf optimization that must not lead design;
    "not supporting" is a closed-and-shrinking set (no meta-test for it).
  - ack-seams-as-sh-lines: seam/env mapping set IN the loom as `export` lines, with
    defaults for all looms; "we express things as sh in this house."
  - ack-facts-are-data: world-facts have no seam because they are facts (declared case
    data); the conductor's seed "partition" was withdrawn as a muddle.
  - ack-varied-seed-default: declared seed = regression + output-stabilization, an
    opt-in per case across the ENTIRE testing infra (regression e2es, unit tests, looms
    alike); looms DEFAULT to varied seed; pinning is the one-line fix when a surface
    contains ND.
  - ack-dogfood-the-engine (hard): use our own parser/env-model/rho/const-prop for looms
    if it doesn't chafe; HARD NACK if it softens a correctness invariant; HARD DEFER if
    it needs invasive kernel changes (hold the ideal picture for the next kernel arc).
  - ack-rebless-authorized: the whole point of non-concurrent suite work; fully separate
    product work from suite work.
  - ack-frontmatter-reduction-on-target: rip frontmatter only where a CLEARLY better
    option exists; stay on the specific goals; no rip-and-tear of merely-messy things.
  - ack-age-is-an-open-front: no plan or lean; carried into opaque review; not a TODO.
  - ack-opaque-review-may-be-owed: the conductor ignores opaque concerns mid-design and
    may break invariants for excellent praxis; the human routes the plan through review
    if owed, BEFORE any building step.
  - call-spine-baseline-dies (conductor's call, delegated): delete it.
  - ack-husks-cleaned: the human removed the six husk dirs themselves.
  - ack-no-dispatch-rewind: no builder dispatched; the human rewinds/restarts before
    pursuing; this ledger's HANDOFF is the brief.
