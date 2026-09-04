# 30Xa — the test-infrastructure decruft arc: conductor ledger

> Tier: conductor ledger (Fable, opened 2026-09-02); compression-resistant state for the arc
> that BUILDS `notes/30X`. Ahistorical except §2 (the human's typed record, kept verbatim). The
> design lives in `30X`; this file carries only what conducting the build needs: rulings typed
> since 30X, the tree-reconciliation deltas, the lane state, and the residue accounting.
> Authority: `30X` and everything it cites outrank this file.

## §0 — state (2026-09-03, end of the second sitting): lanes A, B, C1, C2, C3a, C3b BUILT and green on both legs; lane D (two sub-lanes) next, HELD for a harness restart; a rewound successor resumes HERE

Branch `ai/r30-30X-test-infra-decruft-conductor`; worktree
`C:\Users\ec\Sync\Code\Dorc\.tmp\trees\r30-30X-test-infra-decruft-conductor`. Lane briefs are
never committed (human ruling); this sitting's live in the conductor's scratchpad at
`C:\Users\ec\AppData\Local\Temp\claude\C--Users-ec-Sync-Code-Dorc\98f0da77-1d12-44a8-bc56-c82d8db17c2e\scratchpad\`
(`c1-brief.md` carries the invariants/OUT/process/report shape every later brief cites;
`c1-execute-brief.md`, `c1-prime-brief.md`, `c1-double-prime-brief.md`, `c2-brief.md`,
`c3a-brief.md`, `c3b-brief.md` record what landed; **`d-brief.md` is the next dispatch — D1 then D2,
one builder each**, its amendments from the C checkpoints already folded in). A successor in a new
session re-reads them there or re-cuts from §5 and the checkpoint tables. The branch RIDES
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

### Checkpoint C1-execute (`cc7af6fe`; both gate legs and `bless:dry` green; R1–R7 BUILT; the gate infrastructure built with its activation HELD; deliverables 5/7/9 blocked on one root cause)

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `tc-two-driver-records-intake-divergence` — in-process, a `--results -` block feeds raw fixture records through the fenced fixture intake (`admit_fixture_records` / `Framing::spike`), which the engine hard-codes non-receipt-eligible, so nothing publishes and all 26 `run:` looms diverge from their shell transcripts (records diagnostics, probe shipping) | RULED `rul-in-process-sessions-take-the-controller-intake`: the in-process driver frames a session's raw records into the controller wire format at ONE seat both drivers consume (B1's `session-stdin-is-the-framed-stream`, replicated in-process) and admits them through the controller intake with the seam's attempt nonce — for EVERY session line, never keyed on `run:`; the fixture seat's non-session users stay (lane D's question). The aid-loom re-render this causes is authorized for the records-diagnostic class only, each case listed. Built by C1′. | neither the scout nor the map censused the two intake paths; the C1 brief's "red under `DORC_KNOWN_BROKEN`" rule assumed one-off disagreements, not a systematic one. |
| `dev-gate-activation-held` — the executor committed the gate infrastructure but left the `run:` arm non-activating rather than redden 26 looms | ACCEPT as the lane's interim: a lane ends green, and 26 known-broken reds would have broken every downstream lane's hot loop; C1′ activates it. | the brief's disagreement rule, above. |
| `tc-gate-runerror-should-decline` — a line errorloom's grammar refuses surfaces as `RunError`, not a decline | ACCEPT: grammar refusals map to `LoomDecline::Unexpressible`; a session only the shell can express is a decline by definition. | — |
| `dev-only-one-loom-needs-declaration` — `apply-plan-not-dispatchable` reaches `no-controller-root` only in frontmatter and through the `--this defect` route | ACCEPT; the scout counted a frontmatter mention as a render. | — |
| `dev-r4-folded-into-r6` | fine; build order only. | — |
| nine re-rendered aid looms (the no-clock class: `[unwritten: why-receipt-when-undated]` → a seeded date; probe records gaining `received <time>`) | ACCEPT under R5; C1′ runs `mise run prose:orphans` and REPORTS whether `why-receipt-when-undated` lost its last renderer — never deletes. | — |

Built and verified, so no successor re-derives it: `dorc_loom::runner_seams` (`RUN_SEED`, `SESSION_ROOT = "/dorc-loom-session"`, `session_seams(ordinal)`) consumed by `e2e.rs` and the in-process driver; `dorc_cli::compose::{publish_rooted_receipt, publish_seamed_receipt, read_rooted_receipt, production_receipt_edge_over}` over `&mut dyn LocalIo`, `ProductionEdges` passing `NativeIo`; `LoomSession { store, edge }` in a `RefCell` per `DorcReplayDriver` (platform-shaped model store from one constructor); `LoomDecline` (RemoteApply · ApplyWithPlan · OracleDirs · ExplicitReceiptFile · ReceiptsOverride · ShellOrExternal · Unexpressible); `EdgeFault::ReceiptRead` and `durable-receipt-unreadable.loom`'s declaration; `render_run_loom_in_process` → `TwoDriverOutcome`; `crate_fences.rs` unedited and green; the shipped binary reaches no `ModelIo`.

### Checkpoint C1′ (`d2177928`; Windows leg green; the WSL leg UNRUN — the 2026-09-03 disk-full incident killed the lane mid-gate and its report with it; this entry is read from its commits)

Built: `dorc_loom::records_framing` — the framing seat both drivers consume (`cli/tests/support.rs` gave up its 178-line copy); in-process results routed through the CONTROLLER intake for every session line (`rul-in-process-sessions-take-the-controller-intake`); the records-diagnostic divergences are gone and NO aid transcript moved. HELD: `gate-two-drivers-agree` — five `run:` looms still disagree for reasons outside C1′'s scope, over its threshold of three: a cross-file helper-closure lift · `--format=jsonl` · a receipt source-comparison cwd. UNKNOWN (the report died): the why30 proof, `prose:orphans`. Operating constraints while the human recovers the disk (2026-09-03): native Windows only, no WSL, no `mise run both`, heavy gates skipped; the WSL leg and `gate:arc` are owed at close once cleared.

Ruling for C1″ (conductor, on paper): each disagreement is classified before it is touched — (a) an in-process FIDELITY gap, where the in-process world renders a different answer than the binary for the same case: a bug in `dorc-loom` or the shared cli seats, fixed so both agree, never by changing a product crate's semantics (a fix that needs one is a STOP); (b) a genuine inexpressibility: a typed `LoomDecline`, the set shrinking when the lane that owns it lands (the `run: lint` looms' `--format=jsonl` is lane D's fold — a decline until then is honest only if the in-process lint route is truly absent, not merely different); (c) the interleaving cell: reported with bytes, red under `DORC_KNOWN_BROKEN`. The gate activates when what remains red is ≤ three and every red is a named `tc-*`.

### Checkpoint C1″ (`1984a306`; both gate legs green — the WSL leg on a cold rebuild after the disk wipe; `bless:dry` clean; lane C1 COMPLETE)

The 2026-09-03 disk exhaustion was this suite's own doing: `e2e.rs`'s `Harness` owned its shim dir and profile parent, but `libtest_mimic::run(...).exit()` and the preflight's `process::exit` skip `Drop`, so every runner process left a full COPY of the harness binary (`dorc-e2e-shim-<pid>-<seq>/dorc.exe`) and a profile dir behind — thousands of each in the human's `%TEMP%`. Fixed at the root: the shim is a hard link (unix symlink, then copy, as cross-volume fallbacks), one per runner process; `Harness::reap()` is idempotent and called at every exit path, `Drop` included; measured net-zero temp entries across the whole suite on both legs. Every temp path the suite mints is inventoried in the lane's report with its reaper (all owned; `dorc-doctor-vhdx-test-<pid>` self-cleans at start only — minor).

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| the five disagreements (all class (a), all fixed): `lint-jsonl-envelope` (in-process `run_lint` ignored `--format=jsonl`; now dispatches on `args.format` through the shipped binary's own render seat) · `pin28`/`pin30` (the in-process snapshot followed only the BOOK's `.`-loads and dropped a helper `.`-sourced by a pre-sourced oracle; `dorc_cli::snapshot::root_dependencies` now keeps and positions them) · `why30` (the receipt source-comparison read the recorded path from real disk; an injectable in-memory current-source reader now serves the in-process world, the shipped binary passing `None`) · `report27-decline-static-classed` (a FIXTURE with a 16-char placeholder digest that only the shell runner's silent re-framing had ever repaired; corrected to the book's real digest, consistent with its 8 siblings) | ACCEPT all four. Two riders for C2: `rul-framing-is-one-rule-both-drivers` — the shared framing seat frames RAW fixtures and passes `dorc-records/`-headed ones verbatim on BOTH drivers (the shell runner stops repairing framed fixtures; a fixture that then fails in the shell path is rot, fixed as a fixture); and the new in-memory dependency walk must be the edge's own walk parameterized by its byte source, never a twin (`one-definition-table-two-drivers`'s shape) — C2 verifies and unifies. | C1′'s three named reasons were two short of the truth; the census re-take was the right instrument. |
| `OPEN-2 vars-still-refuses-run-looms` — `dorc-loom vars` refuses `run:` looms (`EXECUTED_ELSEWHERE`), a premise the live gate falsifies; `sections` lists why30's 23 `why-total-*` holes and a dry publish is a fixpoint | ACCEPT as-is: the authoring loop is closed through `sections` + `publish`; `vars`'s refusal and its first-block-only inventory die with the `run:` key in lane D (which folds session inventories into the derived driver). Recorded in D's row. | — |
| `OPEN-3 source-comparison-seat-extended` — the opaque-ruled `30Va` seat gained an injectable current-source reader | ACCEPT: `cli/CLAUDE.md source-comparison-is-one-cli-seat` names extension AT the seat as the only sanctioned path; the shipped read, bounds, non-following, and authentication asymmetry are unchanged; listed for the human's end-of-arc review decision. | — |
| the stream-interleaving cell | AGREES on every rendered session; the C1 brief's worry was unfounded — both drivers emit in engine order. | — |

Steering concepts owed at close (from C1 as a whole): `runner-reap-at-exit-not-drop` · `loom-shim-is-a-hard-link` · `in-process-snapshot-mirrors-source-acquisition` · `source-comparison-reads-the-in-process-world` · `in-process-driver-frames-records-through-the-controller-intake` · `rul-rootless-worlds-are-declared-faults` · `rul-in-process-sessions-take-the-controller-intake` · `LoomDecline`/`gate-two-drivers-agree` as the harness contract's new rows.

### Checkpoint C2 (`d3c479a9`; both gate legs green; `bless:dry` clean; no transcript moved)

Built: `dorc_loom::session_grammar` (one reading of a `$` line through `dorc_syntax::parse` → `SessionReading::{Line(SessionLine{head, argv, input, output}), Decline}`; heads Invocation · Export(Assign|Mark) · Cd · EchoStatus · Cat; errorloom's `ReplayCommand::parse` stays the outer gate and a cross-check — a disagreement declines; the corpus disagreement census is EMPTY) · `dorc_loom::session_env` (a CONCRETE dash environment, name → value + exported bit; the exported non-empty subset is the `SeamEnv` `HarnessSeams::from_env` reads; the per-block clock re-injection mirrors `drive_session`'s shadow line exactly) · `LoomSession.cwd: dorc_core::loadpath::Cwd` feeding the snapshot · `e2e.rs::block_argv` through the same grammar · the two C1″ riders (the framing is one rule; the sourced-oracle dependency walk is ONE byte-source-parameterized BFS in `snapshot.rs`, disk for the binary, sections in-process). `durable-receipt-ambiguous` renders in-process and agrees on both legs; the decline census is 108 agree · 1 declines (`emit30-multipart`, `$ARTIFACT_DIR` outside errorloom's grammar — honest).

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `tc-real-file-stdout-redirect-outside-ruled-set` — the brief's ruled output set (`> /dev/null` · `2>/dev/null` · `2>&1`) made `> <literal file>` + `cat` read-back a decline, though errorloom's grammar already admitted it and a pre-existing test used it; the builder split that test and pinned the decline | REVERSED: the in-process expressible set may never regress below what the outer gate already admits (`30X` §9 — the UNSUPPORTED set shrinks). C3a restores `> <literal>` (stdout to a file in the session's materialized dir) and recombines the split test; the cross-check gains a stdout-to-file concept beside `stdout_to_null`. | the brief enumerated the ruled set from the corpus survey, not from errorloom's admitted grammar. |
| `tc-echo-dollar-question-sees-the-injection-line` — the shell's injected clock line runs before each block, so `$?` seen by `echo $?` is the injection's status, not the previous block's; zero corpus users; latent two-driver divergence | ACCEPT the finding; C3a's rewrite of that line (it must also compute the clock from `$DORC_SEED`) preserves `$?` across the injection (`__rc=$?; …; (exit "$__rc")`-shaped), and a runner selftest pins it. | the B1 line was minted without asking what it did to `$?`; nobody's fault to find until `echo $?` existed. |
| the `cd` scaffold — `Cwd` moves on `cd`, but sections are flat-named, so a subdirectory cwd would misresolve `--book`/`<` operands silently | RULED: a non-root `cd` is a typed decline until the section-name/cwd reconciliation is built (zero corpus users); a scaffold that misresolves is worse than an honest decline. C3a. | the brief asked for the model without asking how flat sections meet a moved cwd. |
| the errorloom single-parse proposal (a `drive_case` generic over the parsed command type) | DECLINED, as the builder recommended: a genuine refactor, not an additive API (`rul-minimize-errorloom-changes`); the resting design is outer gate + dogfooded reading, cross-checked. | — |
| `front-dogfood-ceiling` (three HARD DEFERs, kernel-arc) | RECORDED in `30X` §10 by the conductor: (a) the value plane ⊤-clobbers `export NAME=word` and tracks no exported bit — modelling it is winner-shifting (a resolved variable resolves a load, a load binds definitions, definitions license: `28Q` §1); (b) no public per-word expansion seat; (c) with both, the session environment IS `ValueFlow::variable_before` over the session script's own CFG. The cwd half is already dogfooded. | — |

### Checkpoint C3a (`31852952`; both gate legs green; `bless:dry` clean; the builder stopped at its budget with a clean tree and two deferrals)

Built: **`dorc-testbed`** (`spike/crates/testbed`; dependency-free, std-only, `publish = false`) — the suite's shared substrate below `cli`, born with two tenants (`run_seed`: `DORC_SEED` from the runner's process environment, else one OS draw per process bounded below 2^62 for sh parity, memoized; and `seam_vars`, the `DORC_SEED`/`DORC_SEAM_*` names moved OUT of `dorc_cli::seam`, which imports them) and a charter naming three more (`xfail`, the LCG, the sandbox guard); consumers `cli` and `dorc-loom` (regular), `plan`/`sweep` (dev); `internal-tooling` gains NO dependent (the human's intent: it is the xtask binary's plumbing). The seed prints once at the start of each runner (`seed: <n> — replay this run with \`DORC_SEED=<n>\``) and once at the end of a failing run with both affordances (replay this run · pin this case with `$ export DORC_SEED=<n>`). The clock advances one day per INVOCATION block (`fold_clock_seed(seed, ordinal) = (seed % 36525) + ordinal`; exports, `cd`, `echo $?`, `cat` do not tick — both drivers through the one session grammar; the shell shadow line reads `$DORC_SEED` live in a `$((…))`), so a pin line is ordinal-neutral and seed 0 is the fold's identity for dates. Nine cases pinned `$ export DORC_SEED=0` (eight byte-identical plus the pin block; why30's receipt id re-rendered — the builder's uniform-pin choice, accepted). The e2e bless re-drives the candidate under a derived second seed in its own throwaway store and REFUSES a divergence naming the first differing block and the pin remedy (demonstrated). `sparing_differential` and `sweep`'s exploration loops offset from the run seed; the two runners green under ≥3 seeds on Windows and one on WSL.

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| the seat's home (`internal-tooling` first, then the leaf crate on the human's lean) | RULED `dorc-testbed` as above; `internal-tooling` keeps no dependents, and D2 moves `xfail`/`Posix` out of it so that becomes literally true. | the brief named `internal-tooling` as a candidate without knowing the human's no-dependents intent. |
| `tc-exploration-sweep-coverage-vacuity` — a varied base can make a rare-class reachability (`sometimes`) assertion vacuous under an unlucky seed | RULED `rul-coverage-assertions-are-base-robust-or-fixed`: a varied base tests INVARIANTS (`30X:seed-exploration-asserts-invariants`); a coverage/reachability assertion must either hold with margin over its trial count for ANY base (state the class probability and the miss bound at the assertion; raise the count, never pin the sweep) or keep a FIXED base named as a regression literal. Executed by D2 for hostsim's family. | the brief's deliverable 6 treated hostsim's sweeps as a routing question, not a statistical one. |
| deferred: the publish authority's second-seed refusal (5b) · hostsim's sweeps and `differential.rs` (6) · the seed on every FAIL line rather than once per failing run | ACCEPT the stops (a clean tree at budget beats a compacted lane); D1 takes 5b and the per-FAIL seed (it reworks the publish/inventory loop and the runner's failure composer anyway); D2 takes hostsim's family under the ruling above. | the C3a brief was a full context on its own before the leaf crate was added mid-lane. |
| the uniform `DORC_SEED=0` pin for why30 (its receipt id re-rendered, ~200 lines) over the old-seed pin (id kept, dates moved) | ACCEPT: one spelling for every pin is worth one re-render. | — |
| the WIP commit under `DORC_KNOWN_BROKEN` left in history (the `internal-tooling` siting, superseded two commits later) | correct — the honest ledger. | — |

### Checkpoint C3b (`397983ed`; both gate legs green; `bless:dry` clean; `xfail:census` unchanged; lane C COMPLETE)

Built: `TransportSeam::Scripted(dorc_transport::SimScript)` / `HarnessTransportSeam::Scripted`, constructible ONLY through `HarnessSeams::with_scripted_transport` (never `from_env`, never `Seams::os()` — the type is the fence), driving the existing DST `SimDriver` through the same marker scan the ssh and local drivers use; the case section `hosts/<name>/apply-outcome` (`status <n>` + captured stdout; strawman); ONE apply implementation `compose::dispatch_and_report_apply(io: &mut dyn LocalIo, …)` — production over `NativeIo` + disk, the loom's `run_apply` over the session store + a case section (`run_remote_apply`'s hand table DELETED; its two `--host` apply looms re-rendered to what the real route renders: `apply: error[transport-apply-failed]` plus the identities line, and `dorc: error[transport-crlf-refused]` plus the usage line its own `envelope: invocation` always claimed — the CR the LF-only case cannot carry is injected by doctoring the plan bytes so the route's own check fires; the three `plan --host` transport looms untouched; `marker-unusable` stays a decline, the seeded nonce being always marker-safe); `DiagCode::ApplyOutcomeUnwritten { intent, step: ApplyWriteStep }` (`apply-outcome-unwritten`, EMPTY prose, defining case in `aid/tests`; the write step discovered by a throwaway intact drive, never an op-count constant; the cli-edge conversion from `receipt::DurableFailure` a wildcard-free `match`); the arrangement row `cli-apply-identities-line` (unwritten; renders whenever BOTH documents were recorded, transport status regardless — the conductor's change to the builder's status-0 strawman); the intent-without-outcome witness in `compose.rs`'s tests over `ModelIo` + a `FailureSchedule` (homed in `cli`, keeping `receipt_state.rs`'s cross-process promise); `Op`/`Side`/`IoFault` re-exported from `dorc_cli::durable` (`dorc-loom` still names only cli).

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `deviation-new-looms-are-in-process-not-run-sessions` — no shell-driver census for the scripted-success half | HALF ACCEPT: the failure case is in-process-only by the checkpoint's own ruling; but the identities line CAN be rendered by a `run:` session (`owns:` exists — the builder's "must declare `arrangement:`" is wrong), and the true gap is upstream: the shell session sets NO transport seam, so a `--host` apply cannot run under the local fixture interpreter in the shell driver at all. D1: the session's runner-owned default `DORC_SEAM_TRANSPORT=local:<shell>` (`rul-runner-varies-only-what-it-set`), then ONE successful-apply session whose scripted `hosts/<name>/apply-outcome` declares what the interpreter really produced, proven by both drivers. | the C3b brief asked for shell-driver sessions without checking that the session could select a transport. |
| `deviation-identities-line-renders-unwritten-with-values-dropped` — an unwritten arrangement row emits `[unwritten: slug]` and drops its interleaved values, so the seeded ids do not render until words are authored | ACCEPT: the existing prose machinery's behaviour for every unwritten row (`prose-provenance-states`); whether an unwritten chrome row should still emit its computed values is `tc-identities-line-unwritten-drops-values`, listed for the close report — an aid-machinery question, not this arc's. | — |
| `deviation-frontmatter-metadata-authored-by-builder` (`when-fires`/`why`, `when-used`/`why`) | ACCEPT: registry metadata, not rendered prose (the B2a ruling); the conductor reviews at close. | — |
| `deviation-durable-failure-exit-is-complete` — an unrecorded outcome exits 0 with an Error diagnostic | ACCEPT: the apply happened and the RECORD failed; the exit code stays the apply's; whether "applied but unrecorded" earns a code is the open `aid-error-exit-code-family` question (the human's), listed for close. | — |
| `tc-apply-write-step-word-duplication` — `ApplyWriteStep::word()` (aid) and `apply::publication_refusal_word` (cli) spell the same five words | RULED one table: the words live in `aid` (`ApplyWriteStep::word`), and the pre-dispatch surface converts `receipt::DurableFailure` → `ApplyWriteStep` → `word()`; `publication_refusal_word` goes. D2. | the brief said "reusing `publication_refusal_word`'s words" — it should have said "moving them". |

Steering concepts banked for close: `rul-scripted-column-is-a-fenced-seam-variant` · `rul-outcome-fault-occurrence-is-discovered-not-constant` · `rul-unshell-reproducible-failures-are-in-process-only` · `rul-post-dispatch-durable-failure-is-a-sibling-code` (executed).

### Checkpoint D1 (`2950fc13`; both gate legs green; `bless:dry` clean; the builder stopped twice at green boundaries for rulings — correct both times)

Built: `30X:loom-one-runner` — `looms.rs` deleted; `e2e.rs` is the ONE `harness = false` runner over one walk of every `crates/*/tests`, minting one trial per case (426; the discovery floor now guards BOTH populations); the `looms` `[[test]]` stanza, the `test:looms`/`-quiet` tasks, and hk's `loom-hygiene` step are gone (the `e2e` step's glob widened to the union); two hook self-tests that encoded the two-step shape now track the merge. Rider (e): the run seed and both affordances ride every FAIL line. The binary/task keep the name `e2e` — the rename is the conductor's close-time re-cut.

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `fork-dual-rail-session-home` — `dual-rail` (one use) appears in neither the brief's mapping nor `30X` §5's | RULED: a GATE, never a key — `30X` §5 lists it among the artifact gates; it attaches to every artifact-producing block whose artifact earns it (the trigger derived from the rendered bytes; universal if not mechanical). | the brief's key mapping copied `30X` §5, which had omitted it. |
| `fork-tolerate-export-or-stays` | RULED: an export nothing in the session reads is sidecar config in an sh costume (`KNOBS:kOOB`'s spirit), so never a runner-read export; prefer removing the pipe-stage-order nondeterminism AT THE SOURCE in the exec rail (the key dies with it, four cases re-blessed and listed); else `tolerate` stays as the on-target tenth key. | — |
| `fork-gate-input-plumbing` | RULED shape (B): `run_round_trip` and the gates take a typed inputs struct; the loom path fills it from the session, the dir path from its markers — the one filler D2 deletes. No throwaway marker derivation. | the brief never said how the gates take inputs once markers die. |
| `tests-critical-law` (zero uses, reserved by the binder) | kept among the survivors. | — |
| the derivation blocker — "derive whole-product from `Mode`" routes ~200 erroring-`dorc plan` catalog looms into the artifact battery; no single static signal separates them (55 of 108 whole-product looms carry `mocks/`; 120 of 202 catalog looms render a shebang) | RULED `rul-drivers-decline-symmetrically`: EACH driver owns a typed decline set — in-process `LoomDecline` as built; the SHELL declines a case with an `edge-fault` section (a fault it cannot make happen) and any `dorc-loom` invocation (the in-process tooling's own binary), nothing else. A session is proven by every driver that does not decline it; both ⇒ they must agree (`gate-two-drivers-agree`); neither ⇒ the runner refuses. The `$ dorc-loom --this vars` block 161 catalog looms carry makes them in-process-only BY CONTENT; whole-product looms are shell-proven with the in-process witness — today's split, derived. `rul-gates-attach-to-what-a-block-produced`: exec-under-mocks iff the case carries `mocks/` (a mocks case the shell declines is a loud authoring refusal); the artifact gates (dash `-n`, redirect scan, guard-shape, argv-echo, dual-rail, the artifact-set generation rule) iff the block's `Mode` emits an executable artifact AND its stdout is one (non-empty, shebang-led, the probe/apply pair split); the crash/empty guard narrows to "an artifact-producing mode exited 0 with empty stdout". The `da-keep-an-explicit-signal` option is REJECTED: the signal is in the session's content. The trial reports `[proven by: shell+in-process]` / `[proven by: shell; in-process declined: <r>]` / `[proven by: in-process; shell declined: <r>]`. Newly disagreeing cases are fidelity findings under the C1″ threshold (≤ 3 red as named `tc-*`, else STOP with the census). | the brief's "derive from `Mode`" was a guess at a predicate the design had not spelled; the builder was right to refuse to run a corpus-wide converter on it. |

Praxis: two stops at green boundaries with a grounded resumption map each time beat one compacted lane; the map executes in a fresh builder (D1′) because ~260k of runway is not enough for a corpus-wide coupled change.

### Checkpoint D1′ (`1897d5f1`; both gate legs green except the three named `tc-*` under `DORC_KNOWN_BROKEN`; `bless:dry` no churn; the builder stopped at its budget with a clean resumption state)

Built: `rul-drivers-decline-symmetrically` — `ShellDecline { EdgeFault, LoomToolBlock }` beside `LoomDecline` in `dorc-loom`, one vocabulary the runner reads; a case is proven by every driver that does not decline it, both must agree, neither refuses loudly (`PROVEN_NEITHER`); `run:`/`fixpoint:` are no longer READ anywhere (the keys still sit in the vocabulary until the collapse); the proof label per trial. `rul-gates-attach-to-what-a-block-produced` — the artifact battery runs only on a CLEAN plan (exit 0, shebang-led stdout); the crash guard is "an artifact-producing mode exited 0 with empty stdout"; exec-under-mocks iff `mocks/`; the diagnostic needle gates are the DIR-case surface only — a loom's diagnostic assertion is the session compare plus `defined_code_fired` (which learned the lint-finding shape). Census over 318 looms: both 139 · shell-only 1 (`emit30-multipart`, the `$`-expansion residue) · in-process-only 173 (the `dorc-loom` blocks and the `edge-fault` cases — the predicted split, derived) · refused 0 · DISAGREEING 3. Rider (d): `dorc-loom publish` refuses under a second seed (`second_seed` lives once in `dorc_testbed::run_seed`; the seed threads `SessionEnv::seeded_with → LoomSession::new_seeded → DorcReplayDriver::new_seeded`; demonstrated). Rider (f) part 1: the session's runner-owned `DORC_SEAM_TRANSPORT=local:<shell>;<interp>` — which turned `cli-apply-identities-line` from a disagreement into a both-drivers proof. Wall-clock after: Windows ~23 s, WSL ~5 s for the corpus.

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `tc-dorc-sh-usage-shell-has-no-binary` — the shim provides only `dorc`; `dorc-sh` (the runtime object) has no harness twin and resolves `sh` by bare PATH, which the mocks-only rail cannot honour | RULED: `dorc-sh` blocks join the SHELL decline set, symmetric with `dorc-loom` (`ShellDecline::RuntimeObjectBlock`, name strawman); the three `dorc-sh-*` cases are in-process-only by content. D1″. | the decline set was ruled "nothing else" without listing the third non-shimmed binary. |
| `tc-lint-error-render-carries-slug-in-process-only` — the binary's top-level lint error omits the `error[slug]` header the in-process render carries | RULED: the BINARY is wrong — every invocation error is a registry code (`cli/CLAUDE.md invocation-errors-are-registry-codes`); D2's lint fold makes the binary's lint-error print seat render through the same staged-parts seat; the committed golden stands. A product-facing diagnostic change (a slug appears in the binary's lint error), listed for close. | — |
| `tc-artifact-form-fallback-note-binary-only` — the binary emits `emission: note[artifact-form-fallback]` on stderr for a script-relative-load book under `> /dev/null`; the in-process render does not | RULED: the shell proof is authoritative where it runs (`30X:loom-driver-is-derived-and-reported`); the in-process world's artifact `Selection` inputs differ — an in-process fidelity gap D2 fixes in `dorc-loom`/the shared cli seats, after which that golden GAINS the note line (a fidelity correction, listed). | — |
| `open-every-block-vars-reverted` — inventorying every editable block moves five `--this vars` transcripts | REVERSED: the every-block inventory stands and the five blocks re-render (listed) — `vars` output is derived editor aid (`282` §2), and a derived block growing because the tool got better is not a golden re-blessed to pass a gate; the `vars_answers_for_every_committed_case` tolerance is removed. D1″. | the brief's "no rendered bytes move" clause did not carve derived tool output. |
| `open-deferred-to-e2e-removed` (a dead predicate) | fine. | — |
| the collapse and the apply session undone at budget | ACCEPT the stop; D1″ from the resumption state. | the D1′ brief was a full context on its own. |

### Checkpoint D1″ (`9834a071`; both gate legs green except the two D2 fidelity reds; `bless:dry` no churn beyond the listed; lane D1 COMPLETE except the collapse, re-homed)

Built: `ShellDecline::RuntimeObjectBlock` for `dorc-sh` heads (census: both 139 · shell-only 1 · in-process-only 174 · refused 0 · the two D2 reds); every-block `vars` (`editable_renders()` collects every editable render; `Used` lists the distinct `(name, value)` pairs in block order; `used_variables_of` reads render fragments only, so `durable-receipt-ambiguous` now inventories; the five `--this vars` blocks gained 20 lines, listed in the lane's report); the `DORC_LOOM_DUMP` candidate write restored on the render-fixpoint failure path (a D1-merge regression of `aid/CLAUDE.md authoring-a-replay-block-is-blind`); `apply30-scripted-outcome-both-drivers.loom` (`--plan plan.sh`; `hosts/web1.example.net/apply-outcome` `status 0`; proven by both drivers, rendering the unwritten identities row).

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `ask-deliverable-three-survivor-set` — `probe-results: authored` (four looms, all with mocks) is a GATE OPT-OUT: it skips gate-1's mocked-probe reproduction compare for hand-authored records the mocks cannot produce; "→ the `<` redirect" keeps the feeding and loses the opt-out | RULED `rul-survivors-are-the-criterion-not-the-count`: `30X` §5's law is "frontmatter survives only for what is ABOUT THE CASE AS AN AUTHORING HOME"; nine was the tally under it. `probe-results: authored` and `tolerate: <class>` are case-level declarations no session line can honestly carry (a runner-read export is sidecar config in an sh costume), so the survivors are ELEVEN: `code` · `arrangement` · `owns` · `when-fires` · `when-used` · `why` · `envelope` · `tests-critical-law` · `todo` · `probe-results` (`authored` only) · `tolerate`. Gate-1 stays — it is what keeps a hand-authored fixture from silently contradicting its own mocks. D2a executes the collapse on that set. | the brief's mapping treated `probe-results` as a feed, never asking what the key gated. |
| `ask-vars-used-dedup-by-value` | ACCEPT: identical repeated blocks add no noise; distinct pairs in block order. | — |
| `ask-dump-rescue-into-render-fixpoint` | ACCEPT: a repair of D1's own regression, not new mechanism. | the D1 merge dropped a documented loop and no gate noticed — `authoring-a-replay-block-is-blind` is a doc, not a test. |
| `ask-apply-uses-plan-flag-not-positional` | ACCEPT (`--plan`); the brief's spelling was wrong. | — |
| the collapse unstarted | ACCEPT; re-homed into D2a with the dir-case conversion, the same converter family. | — |

### Checkpoint D2a (`68ccaa41`; both legs green except the two D2b reds; the builder stopped cleanly before the dir-case conversion with a grounded map)

Built: `RoundTripInputs` (shape B) with a `from_session` filler (flags and `--artifact-dir` from the artifact-producing block's argv through the session grammar; `probe-results`/`tolerate` from the surviving frontmatter; dual-rail derived from the book's own multiline argv; the exit from `$ echo $?`) and a `from_markers` filler the conversion deletes; the frontmatter collapse — 110 looms respelled by a throwaway converter (`flags`/`why-addr`/`artifact-set` were already on the session line; `dual-rail` and the four `expect-*` keys dropped, with a new runner check `transcript_slugs_are_catalog` validating every transcript diagnostic header against the catalog; `exit` → six `$ echo $?` blocks, the only rendered additions); `FRONTMATTER_KEYS` one list, `run_lane`/`is_run_lane_key`/`defining_form_refusal` gone. Rulings taken at the scouting stop: `rul-xfail-is-a-registry-keyed-key` (a twelfth key `xfail: <pin-slug>`, the pin registered in the one xfail registry; structural gates tolerated and reported, the transcript ENFORCED, XPASS loud — "a loom is never XFAIL" inverts under "no legacy e2e"); converted transcripts are BOTH streams via a scoped bless with two one-time checks that die with the converter (the blessed stdout projection equals the former `expected.out` byte for byte; every retired `expected-*` needle appears in the blessed stderr).

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `tc-apply-exit-has-no-session-spelling` — `apply-exit` is the exec rail's expected exit for the rendered apply under mocks, a runner-only execution no session line can spell; the builder dropped the loom-side assertion (one non-zero case) | REVERSED: `apply-exit` SURVIVES (default 0) — a case-level declaration about the exec rail's world, by `rul-survivors-are-the-criterion-not-the-count`; the exec_check rc assertion is restored for looms. THIRTEEN keys. D2a′. | the survivor ruling enumerated eleven from the mapping's blind spots and missed this one. |
| `dev-dual-rail-derived-from-book-quotes` (a multiline quoted string opens the rail; no escaped-quote handling) | ACCEPT: a fooled trigger loses a check, never fails falsely; noted. | — |
| `dev-eleven-not-twelve` (`xfail` joins with its lens and cases in the conversion) · `dev-binder-assert-deleted-not-swapped` · `dev-from-session-lands-in-deliverable-2` | fine. | — |
| deliverable 3 unstarted at ~660k | ACCEPT; D2a′ from the map, a fresh builder. | — |

### Checkpoint D2a′ (`c7aeffca`, `DORC_KNOWN_BROKEN`; the conversion executed and blessed; 105 of 109 green; the dir path still present; gates unrun)

Built: all 109 dir-cases converted in place to single-file looms by a throwaway converter (both-streams transcripts via a filtered `bless:case` over 108; check (i) as an ordered non-blank stdout subsequence — sound — held for 102 of 102 non-XFAIL cases; check (ii) landed every one of 26 needle patterns; the stderr churn is exactly the diagnostics the drives already produced); the `apply-exit` key restored with its `exec_check` assertion (`door1-and-form`); the XFAIL/XPASS lens moved onto the loom path; four dedicated pins registered and Live; THIRTEEN keys in `FRONTMATTER_KEYS`; the three case anchors the deletion would strand (`dorc_flags_selftest`, `dorc_sh_smoke`, `run_closed_loop`) re-homed through `materialize_anchor`. Proof census over the converted set: both 58 · shell-only 7 (`$` expansion) · in-process-only 0 · refused 0.

| item | ruling | the conductor's own mistake, named |
|---|---|---|
| `tc-loop30-xfail-has-no-loom-structural-expression` — a TARGET-TENSE golden, tolerated only because the dir lens skipped the content diff; every loom structural gate passes (XPASS) | RULED: a transcript is what the user saw, so a wished-for output has no loom home; `loop30` is an ordinary descriptive case (no `xfail:`); `p-x-loop-cell-disjoint-siblings-replace` stays registered RESERVED (no call site) with its horizon and a `Deferred.why` naming the target-tense golden, discharged when the engine lands the replacement and the case re-blesses. No new Rust test. D2a″. | the brief's premise "the four fail structural gates" was true of three. |
| `tc-load30-subshell-transcript-carries-a-machine-path` — `helper-declaration-contested` prints the absolute materialization path on stderr | RULED: the third seat this arc caught (after B1's why-lens and B2a's sibling advisory); it renders through the ONE relativizing seat (`why::relativize_for_display`) like the other two — a display change of the ruled class, listed; never a normalizer. D2b, with its fidelity items. | — |
| `tc-load30-script-relative-two-driver-disagreement` | the same `artifact-form-fallback` in-process gap D2b owns; same fix. | — |
| `tc-floor30-both-streams-transcript-vs-never-remeasure` — `bless:case` refuses floor cases; only `bless:floor` writes, and it re-measures | RULED: run `mise run bless:floor -- floor30-inline-dot-boundary` on the WSL leg; the re-measure is the design (`emitted-is-measure-once-ground-truth` names it the ONE write path, byte-stable over a correct case); the manifest must come back byte-identical, asserted in the report. D2a″. | the brief's "never re-measured" was over-cautious. |
| `open-checki-is-subsequence-not-byte` · `open-load30-empty-ran-normalized` (the bless's own fixpoint, one byte) · `open-dedicated-xfail-pins` (env30 at `r31` mirrors the existing `30S` attention-calls, not a roadmap row) · `open-two-dorc-exit-cases-skip-structural-gates` (their `expected.ran` sections are dead if unread — delete, reported) | ACCEPT all. | — |
| `open-pin-registry-drift` — `plans/30P` names two pins not in `PINS` | a plan-doc drift for the close batch (rewrite `30P`'s what-landed paragraph in place). | — |

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
| C1 `lane-c1-in-process-receipt-world` (medium) | `dorc-loom` reaches `ModelIo`/`LocalIo` through `dorc_cli::durable` re-exports (NO Cargo edge; `crate_fences.rs`'s human-ack rosters untouched — `Checkpoint C1-map`); the in-process driver composes the REAL `LocalReceiptEdgeV1` over `ModelIo` with the SAME seeded entropy and ticking clock `cli::seam` holds (one generator, reached through `dorc_cli`'s lib); `run_receipt_store_why` reads that store; a store that cannot be reached is a DECLARED `edge-fault` (`rul-rootless-worlds-are-declared-faults`) and the scripted `ROOTLESS_WORLD` seat retires; a multi-block session runs in-process as ONE world with the per-block clock; a closed typed-decline enum for what it cannot express (`export`, `cd`, pipes, `cat`, external tools, unmodelled durables, process exits) — one decline routes the WHOLE session to the process driver, the set only shrinks, no roster; `gate-two-drivers-agree` in `looms.rs` (a `run:` loom run in-process without decline must equal its committed transcript byte-for-byte; declined sessions are reported by reason, not failed; both-streams order verified against the process transcripts); proof that why30's `[unwritten: why-total-*]` holes resolve through `dorc-loom vars`/`sections` and a dry publish is a fixpoint — no prose authored. OUT: shell-line modelling (C2); seeds (C3); the durable-report surfaces (C3 — extending `run_remote_apply`'s scripted table is a STOP); durable CONTENTS changes are `rul-durable-contents-reviewed-before-design` territory, a hard STOP | COMPLETE at `1984a306` (`Checkpoint C1″`): the gate is LIVE — 107 `run:` looms agree byte-for-byte on both legs, 2 decline honestly, 0 disagree; the runners leak no disk; both gate legs green |
| C2 `lane-c2-dogfood-session-model` (medium) | the in-process driver models the session's `$` lines with our own `dorc_syntax` parser and env model (`30X` §8: HARD NACK if any kernel invariant softens; HARD DEFER, recorded under `30X:front-dogfood-ceiling`, if it needs invasive kernel change): `export` (seam variables through `HarnessSeams::from_env` over the MODELLED environment — the one parser), `cd`, `<` redirects, exact `echo $?`; everything else stays a typed decline; `rul-runner-varies-only-what-it-set` holds in-process exactly as in the shell | COMPLETE at `d3c479a9` (`Checkpoint C2`): 108 of 109 `run:` looms render in-process and agree on both legs; three small residues fold into C3a |
| C3 `lane-c3-seeds-affordances-durable-report` (medium) | `seed-varied-by-default` (every seeded seam takes a fresh run seed per run; the run-wide seed printed at the start of every run; every failure names the seed and the one-line pin spelling `$ export DORC_SEED=…`); bless REFUSES a transcript that does not reproduce under a second seed; seed-dependent renders (the three whygallery looms and any other) declare their pin in the case; `seed-declared-is-regression` — one spelling across unit/DST/loom/e2e, hostsim's seed constructors taking the same run seed; the post-dispatch durable report authored over the in-process world — a durable-failure diagnostic carrying the surviving intent (a seeded id) plus the closed write-step word, and the completed apply's intent/outcome identities chrome line — minted with EMPTY prose (`[unwritten:]`, `error-authorship-tier`), each witnessed by a state-only test in `receipt_state.rs` (the store holds an intent and no outcome / holds both) | SPLIT 2026-09-03 into C3a (seeds: the varied default, the two affordances, one spelling across every tier, bless refusing under a second seed; plus C2's three residues) and C3b (the transport seam's scripted column, success half, so an apply runs in-process; the consumer stops discarding; the durable-report surfaces) — each a builder's budget on its own. C3a COMPLETE at `31852952` (`Checkpoint C3a`; its two budget deferrals fold into D1/D2); C3b COMPLETE at `397983ed` (`Checkpoint C3b`; one shell-driver gap folds into D1, one word table into D2) |
| — checkpoint C3 | RULED `rul-post-dispatch-durable-failure-is-a-sibling-code` (conductor, 2026-09-03; the human's veto stands open at close): the post-dispatch durable failure is a SIBLING code of `durable-receipt-unwritten`, never a reason arm — `AID-NEEDS:law-codes-vary-by-world-not-grammar`: the plan-time world (no durable, nothing touched, re-plan) and the post-dispatch world (the intent published, the machine perhaps changed, the outcome unrecorded — check the host, keep the intent id) differ in world and in repair, and the why-lens must never let them be confused; WHICH write step failed is a typed reason enum within the one code (`28L:rul-reason-enums-not-sibling-codes`). C3b executes it. | — |
| D `lane-d-one-runner-and-frontmatter-collapse` | one runner; the driver derived and reported; `run:`/`fixpoint:` retired; frontmatter 24→9 (`tests-critical-law` has zero uses — drop unless `vocabulary.rs` reserves it for a reason); hk/mise/bless plumbing follows; the dir-case → loom conversion (ACKED: a one-off converter deleted after use; the round-trip runner's dir entry and its marker grammar die); `lint-real-*` re-homed as the real-tools test's fixture space; the four `run: lint` looms fold into the session driver; the doctest noise; yardstick. Dispatched as TWO serial sub-lanes: D1 (the runner merge, the derived driver, the frontmatter collapse) inheriting C1″'s `vars` over every session block and C3a's two residues (the publish authority's second-seed refusal; the run seed on every FAIL line); D2 (the dir-case conversion, the lint fold, the fixture seat's fate, `xfail` and `Posix` out of `internal-tooling` into `dorc-testbed` so it has no dependents, the doctest noise, yardstick) inheriting hostsim's sweeps under `rul-coverage-assertions-are-base-robust-or-fixed` | D1 §1 and rider (e) BUILT at `2950fc13`; D1′ BUILT the derived driver, rider (d), rider (f) part 1 at `1897d5f1` (`Checkpoint D1′`; three fidelity disagreements red as named `tc-*`); D1″ BUILT the `dorc-sh` decline, the every-block `vars`, and the apply session at `9834a071` (`Checkpoint D1″`); the collapse re-homes into D2a (the two converters: the frontmatter collapse and the dir-case conversion, one family), D2b the rest of D2 (the lint fold with its two fidelity fixes, the fixture seat, `xfail`/`Posix` out of `internal-tooling`, hostsim's sweeps, the word table, doctest noise, yardstick) |

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
