# 300 — Round-30 conduct ledger: the 28Q-arc implementation

> Tier: LLM-authored conductor working-ledger (Fable; minted 2026-08-14 at round-30 open,
> human-directed). AUTHORITY ORDER: root human docs > `plans/28Q` (THE kernel plan) +
> `notes/301` (THE minispec/dorc-verify spec) + `spike/CLAUDE.md` > this ledger. This file
> never duplicates those — it carries arc STATE: staffing, dispatch, ack-grades, gates,
> the census bank, and the conductor-handoff protocol. Grades: [TYPED] the human typed
> it · [ACKED] substance confirmed in dialogue · [CONDUCTOR] conductor adjudication,
> unratified unless a human reaction is recorded. Maintenance: compression-resistant;
> folded lanes collapse to a line; newest state at the top of §2.

## §1 — Arc shape and the handoff protocol

- Round 30 splits in two: the FIRST HALF is the correctness-tooling standup (this
  file's §2 + `notes/301`; it reshapes and supersedes the execution-plan half of
  `notes/28T`, which stays the evidence digest, marker-annotated); the SECOND HALF is
  `28Q` stages i–iii (stage-0 landed pre-arc). Ordering forced by `28Q` §8: every
  stage inherits the checker gates (certifier + sparing re-derivation green, both
  planes voting), so the checkers must exist first.
- [TYPED 2026-08-14] Conductor-context management: the arc expects sequential
  conductors and/or rewinds. THE NAMED STOPPING POINT is **wave-one-close** (§4). The
  human's rewind anchor is the 2026-08-14 plan-ack sitting (pre-dispatch, this ledger
  committed, zero subagents in flight). A post-rewind or successor conductor MUST
  distrust conversation memory of anything after that sitting — ground truth is this
  ledger + `notes/301` + `LIVING_STATUS.md` + `git log` (conductor branch:
  `ai/r30-conduct`, worktree `.claude/worktrees/r30-conduct`).
- [TYPED] Round-30 numbering: notes/300 = this ledger; notes/301 = the minispec/verify
  spec; `plans/302` is RESERVED for the solve-certifier mechanical spec
  (conductor-authored, pre-build). Never mint a 29x ID (quarantined round).
- [ACKED 2026-08-15] **REWIND ANCHOR #2: the wave-one-close stamp** — `8889618f`
  folded, `notes/305` (the definition-factoring conversion work order) committed,
  zero subagents in flight. A post-rewind conductor dispatches the conversion lane
  FROM `305` verbatim (no re-derivation), conducts its checkpoint-1 → conversion →
  fold, then the snapshot-emission stage. THE NEXT NAMED STOPPING POINT: **the
  stage-i fold** (conversion landed · the differential cells' engine-agreement half
  green · `pin30` flipped with an AUTHORED record · both planes' checker gates
  green) — sited there because closure-custody's policy half waits on the
  human+sibling rulings anyway, so a conductor cycle at that boundary costs nothing.

## §2 — The Wave-1 stage (the correctness-tooling standup): lanes, staffing, gates

[ACKED, as reshaped through the 2026-08-14 design sittings] Opus builders in isolated
worktrees; every brief carries the `spike/CLAUDE.md` safety block verbatim, step-zero
(reset to the conductor-stated `ai/main` tip + hash verify), step-one root-doc reads,
the no-subagent clamp, naming discipline (`270` §1), the `verified-core-discipline`
skill pointer, and flag-don't-resolve on every judgment call. PLUS the quarantined
builder-prerequisite read, per the conductor skill's quarantine section (durable,
human-committed 2026-08-14): Opus/Sonnet builders and foreign-lineage reviewers,
before any other work; never Fable-class subagents; the conductor never reads it. No lane carries
pilot/measure/kill staging [TYPED — velocity; the human inserts kills if needed].
Sequencing: facade solo FIRST → {derived-defs pipeline + minispec/verify standup} and
{kani, certifier} in parallel → rederivation integration → discipline-close.

- **lane-facade-std-dropping** — **FOLDED 2026-08-14 @ `601364f7`** (four commits;
  conductor-verified both legs green, zero golden drift, zero new deps, no split).
  What exists now: `core::sorted::{SortedSet, SortedMap}` (private-backed, one
  private `position`→`Slot` scan each; total, panic-free, index-walk bodies);
  `Powerset`/`MapL` re-seated on them (`Powerset`'s backing SEALED — the pub field
  died as the Aeneas-prep/refinement-enabling structural change); `Dialect`/
  `selector_covers` moved onto them; `solve.rs` production code untouched (its
  BTreeSets were test-only — census delta); VecDeque stays per lean-vecdeque-stays.
  §2a below banks the seat list + findings the later lanes consume.
- **lane-derived-definitions-pipeline + lane-minispec-verify-standup** — **FOLDED
  2026-08-14 (combined lane, 13 commits; conductor-verified)**. As-built: the
  translation unit + fence (`spike/verify/aeneas/`; strict-translate 0 holes/40
  axioms; lake green 1707 jobs; byte-idempotent regeneration; hole+axiom census
  wired — a green translate proves nothing without a green lake build AND a census,
  the `304:fnd-mut-closure-emits-ill-typed-lean` law); `minispec/` skeleton (three
  unwritten unit stubs, Proofs/, committed `Generated/`, CLAUDE.md conductor-stub);
  the `dorc-verify` crate + nine `verify:*` tasks + the hk `minispec` step;
  `tests-critical-law` frontmatter key + two-way agreement, both directions. Badges
  REAL: proved · elaborated · interrogated · report/catalogue/mismatch-refusal;
  NAMED SEAMS (structural `needs_external_engine` tier split): `seam-kani-pairing-
  unbuilt` · `seam-decision-record-read-mode` · `seam-statement-mutation-unbuilt`.
  Verso NOT adopted (fallback: module docstrings; three costs in `304`; re-open at
  the first binding). Tripwire 8KB. Tools-built-general: NONE of the imported family
  stood up (would have been minispec-scoped at budget — the law's named-seam path
  taken, correctly). FINDINGS + the reshape table: **`notes/304`** (née 303-at-mint).
  OWED OUT OF THE FOLD: (a) the FIVE one-line `match` respells in the verified core
  (Option-combinator idioms choke Aeneas; proved cousins; `SortedSet::insert` — the
  canonical-form seat — is currently fenced as COLLATERAL of charon's
  inherent-impl-naming limitation, so no Lean law about set insertion until the
  reshape lands; remit-over-`Flat` unaffected) → the reshape rider below; (b) a
  plan-route decision dump emitting `(SiteId, decision)` pairs — the whylog's
  `ApplyLine` records `leaf: u32` only, collapsing in-loop member sites — a real
  product feature, its own dispatch, prerequisite to `demonstrated` bindings;
  (c) `spike/CLAUDE.md` Build/test/run grows the `verify:*` rows at discipline-close.
- **lane-facade-reshape** — **FOLDED 2026-08-15 @ `b9d91fec`** (three commits,
  rebased; conductor-verified both legs green + `verify:check`). The five `match`
  respells landed; the pipeline is end-to-end +SURE (translates AND typechecks);
  the fence is the permanent EXITS class only — `SortedSet::insert`, the
  canonical-form seat, now carries a real Lean body (the collateral cost
  discharged; set-insertion laws are stateable). Census 40→26 as published.
  Banked for law-authoring seat-picks: `alloc.vec.Vec.{remove,is_empty}` and the
  derived pair-`PartialEq` instance remain trusted-base axioms UNDER now-open
  bodies — remove-family and Eq-derived laws prove modulo those;
  insert/get/get_at/position/union/intersection are axiom-free below. Riders
  routed out: `304:fnd-axiom-census-double-counts` (Template files double the
  count; real unique = 13) → the kani lane; the
  keep-borrows-out-of-closure-returns discipline needs its durable home in
  `core`/`analysis` CLAUDE.mds → discipline-close (currently stated only in the
  aeneas Cargo.toml, where a facade editor will never look).
- **lane-kani-harnesses — FOLDED 2026-08-15 @ `3563584c`** (13 commits rebased onto
  the conduct branch — NOT `ai/main`, frozen as the human's QA point; conductor
  Windows-leg `gate:full-quiet` GREEN over the folded tip, 2020/2020 incl.
  clippy-from-clean; WSL leg DEFERRED to the close batch, justified by inspection:
  ZERO `cfg(unix)`/`cfg(windows)` additions in the lane diff, and WSL serialization
  behind the running definition-fixtures lane). Landed: the 37-harness battery
  (19 green at bounds · 18 over-budget/UNJUDGED · ZERO counterexamples); the
  `pinned`-badge machinery (`seam-kani-pairing-unbuilt` CLOSED — toolchain-resolved
  by-name pairing, three outcomes kept apart); the census double-count fix (REPORT
  axioms 26→13); the memory-gated driver with GATE-CHECK-BEFORE-VERDICT (the
  CBMC-prints-FAILED-after-its-own-OOM save, regression-pinned with the real bytes;
  no-verdict-no-gate = broken run, a third thing, never rounded either way).
  Conductor statement-review PASSED: laws faithful, bounds declared honestly per
  harness, the Arbitrary law held (arbitrary backing + assume-canonical, never
  build-by-insert), independent-walk second opinions share no code with the judged
  scans, all support `cfg(kani)`-gated (no production widening). Lane report:
  `notes/300a-kani-lane-report.md` (its §1 carries the over-budget shaping rule).
  ADJUDICATIONS at fold: root mise pin ENDORSED (additive `cargo:kani-verifier`,
  os-gated — the elan precedent) · the implicit rustup-nightly install CONFIRMED
  within the ruled kani-setup exception · `Dialect::any_minted` KEPT faithful; any
  reshape may add an assumed-canonical dialect generator ONLY paired with a
  mint-satisfies-the-invariant closing harness (the Arbitrary-law pattern; never a
  silent trade). BANKED `work-kani-battery-reshape` (mechanical, non-blocking,
  post-close): reshape the 18 over-budget harnesses to concrete lengths per the
  measured shaping rule (one harness per length/length-pair) · add a toolchain-less
  `cargo check` of the detached harness crate to the lane task (rot-visibility) ·
  the `verify:kani*` rows still owed to `spike/CLAUDE.md` Build/test/run at close.
- **lane-solve-certifier** — **FOLDED 2026-08-15 @ `a1535601`** (8 commits;
  conductor-verified both legs 1967/1963 green, zero golden drift). As-built per the
  settled `302` + the checkpoint rulings: `certify.rs` (~700 lines incl. tests;
  `SolveConsistency` private-mint; `core::sorted` throughout, no facade extension
  needed); the observer seam in `solve` (`run` + `Unobserved` ZST; loop extracted
  once); `trusted()` = certified at all nine call sites (the lane's ONE widening,
  spec-settled: consistent-at-cap is the lfp and is used); four named consumer floors
  (funcenv fold-BREAK with `folded_edges=∅` pinned); aid plane registered
  (`solver-consistency-failure` + `SolvePass` + `CollapseKind::SolverConsistencyFailure`,
  prose `[unwritten:]`); cli reports root-cause-only pre-network. Adjudications at
  fold: **F9 accepted-as-disclosed** — the reach/self-reach floors are gate-level
  (boolean, cardinal-sin-safe direction), end-to-end drive priced 1–2h, natural home
  = the classify rework 28Q stage-iii forces · F10 (`mise run fmt` refuses under
  agent env; working spelling `mise exec -- cargo fmt --all --manifest-path
  spike/Cargo.toml`) + F11 (WSL keeps a separate mise trust store; new worktrees need
  a WSL-side `mise trust`) → discipline-close Build/test/run lines · whylog-spine
  chafes banked for enrichment: `FailedCheck` carries run-scoped CFG node indices
  (partial node→`SiteId` mapping OR a distinct run-scoped row species — undecided) +
  `Inconsistency<Reach>` holds `ProvId` (resolve-or-drop at any future durable edge —
  spine boundary (1) doing its job). Open: `302:pin-blast-radius-escalation`
  [HUMAN?]. The post-land cross-lineage review RAN (Sol, read-only; raw report
  committed at `notes/303a-certifier-crosslineage-review.md`) and RETURNED FIVE
  FINDINGS, every one conductor-verified in the code before crediting (provenance:
  OpenAI lineage, adjudicated under maximum skepticism): (1) missing-states
  fail-open — `certify_solution`'s `let-else continue` lets a truncated/empty states
  vector certify clean with zero checks, contradicting `302` §2's pessimistic rider
  (production-latent only while `solve` is correct — which is the exact assumption
  the instrument exists to drop); (2) the `run`/`Unobserved` `pub(crate)` seam
  bypasses the `solve(`-needle lexical fence; (3) the origin-round reach/self-reach
  consistency diags surface AFTER the probe-mode return and probe shipping,
  violating the §4 pre-network posture (floors held; disclosure late/lost);
  (4) the SelfReach account is materially false (solve-count reported in the
  failing-checks field; fabricated advisory) — mis-attribution-adjacent under
  `271:rul-sin-ordering`; (5) the `effect.rs` debug_assert fires BEFORE
  demote-and-report, so debug builds (and DST) panic instead of exercising the
  machinery. The review also independently VERIFIED the funcenv grant-seal from the
  code, and REFUTED "raw solve unreachable" as an enforcement claim (true today,
  unenforced tomorrow). REPAIR LANE dispatched (the certifier builder resumed;
  branch `ai/r30-lane-certifier-repair` @ `da66a918`): R1 length-mismatch ⇒
  Inconsistent (edge-guard mirror stays) · R2 fence gains the `run(` needle ·
  R3 origin-round diags surface pre-network · R4 honest-or-typed-absent account ·
  R5 assert deleted · plus the F9 end-to-end floor drive folded in. Human
  push-notified 2026-08-15. **REPAIRS FOLDED 2026-08-15** (four commits; builder
  gates green both legs 1974/1970, zero drift, fences falsified both directions;
  conductor gate over the combined tip at fold). Execution found TWO findings worse
  than reviewed: the fence's `production_half` cut each file at its first
  `#[cfg(test)]`, blinding it to production items after a test module (both fences
  now scan whole files — simpler AND stronger); and the origin round's consistency
  diags were DROPPED entirely (`origin.diags` discarded for `round.diags`), never
  merely late. R1's landed shape separates graph-fact (solver-mirror stays) from
  solution-defect (missing state/seed ⇒ `failing.boundary`; `inconsistencies()` may
  be legitimately empty while `total > 0`). R4 gave the record real
  failing-check/solve counts + a measured `SolverRounds` advisory. F9's ×4 is
  complete (`a_real_reach_inconsistency` — genuine perturbation, genuine checker,
  `ProvId`-bearing — drives both floors; non-vacuity controls included). No de-dup
  between origin- and fixpoint-round failures: different solves, different events
  (builder call, endorsed).
- **lane-sparing-rederivation (a) — FOLDED 2026-08-15** (codex/Sol-authored,
  foreign lineage, shim-backstop-committed; three commits; conductor law-review
  passed; gate over the combined tip at fold). `dorc-sparing-reference`: 614 lines,
  zero deps, own opaque token vocabulary, `BackingSet` non-empty by construction,
  22 law-named tests; authored blind to production code as briefed. THE THREE
  MODEL-AMENDMENT RULINGS (conductor, from the fold review — two flagged by the
  model's author, one a bundle omission of MINE; all are additions to the ruled
  English, and lane-(b) applies them to the model with citations):
  `rul-reference-entity-name-floor` — within-kind unequal entities answer
  ProvablyDisjoint under the name-comparison floor (`canonical-coord-continuity`'s
  no-resolver floor; USER_STORY stage-6); the model stays resolver-blind and the
  (b)-adapter feeds canonicalized entities where a resolver exists. EPISTEMIC
  SHARPENING [human-corrected 2026-08-15]: "ProvablyDisjoint" is the algebra's
  verdict vocabulary — disjoint GIVEN the claims — never machine-proof of
  referent-inequality. Entity-disjointness rests on a SPEECH-ACT (the resolver
  author's claim, or the disclosed-weak name-floor), must wear its tier
  mechanically in every why-chain (`trust-tier-is-syntax`; a claim never renders
  in measurement's clothing), and no aid surface may spell it "proven".
  Discipline-close check item: verify the survival why-chain DISTINGUISHES
  resolver-claimed disjointness from name-floor-derived disjointness; a gap there
  is an aid-plane finding to ledger, never to silently build ·
  `rul-reference-kind-fence-disjoint` — cross-kind pairs short-circuit
  ProvablyDisjoint (the v1 kind fence, `kind-fence-movable`; my bundle stated only
  no-cross-kind-same, so the model answers Unknown and would demote every real
  cross-kind meet) · `rul-reference-empty-footprint-assert` — the model KEEPS its
  conservative empty-footprint collide; the (b)-adapter asserts production never
  feeds ∅ to the meet, and a violation is a differential FINDING, not a
  normalization. Shim anomalies logged: the Windows codex self-commit ACL grant was
  blocked by the permission classifier (backstop path used; a settings-allowlist
  question for the human someday); a sibling-codex-in-same-worktree observation
  (~SUSPECT process-ancestry misread; no corruption; logged only).
  Original (a) charter, for the record: the naive reference model of the
  sparing/composition algebra, authored FROM the ratified English law-set under
  structural-simplicity constraints: the checker's value is STRUCTURAL difference —
  written under different constraints, from the machinery-free description of the
  goal, one pass, no worklist — never authorial lineage [TYPED — the
  independent-voices framing was deweighted; a foreign-model author (codex, ACKED
  available) is incidental, not load-bearing]. Zero shared code with `coord.rs`;
  statement-vs-spec disagreements FLAGGED, never resolved. (b) Opus integrates:
  DST-permutation internal differential + plan-time re-derivation of every survival
  verdict before a plan ships; disagreement ⇒ demote to guard/run + narrative record;
  the demote-only structure recorded explicitly (the `271:rul-net-quality-u-curve`
  pass condition).
- **lane-flux-engine-hardening** — [TYPED 2026-08-14: DEFERRED, penciled] not in
  Wave-1 (scoping it in would bloat; enough is on the table). Penciled MID-r30: after
  everything Lean-related is stood up (wave-one-close), before the proper kernel
  rewrite (28Q stage-i). Intent stands: another defense-in-depth instrument —
  ENGINE-tier (intake byte-budget, span/interval arithmetic; the churny tier no other
  instrument reaches at compile time), explicitly NOT part of the verification core
  (Kani+Lean+binder own the algebra; triple-covering rejected); nightly pin nested;
  meta-process learnings a deliverable. EXCEPTION [TYPED]: any typesystem or
  architecture change REQUIRED for Flux to be possible at all belongs in the
  Aeneas-prep work — the facade lane's scope, not the deferred lane's.
- **lane-discipline-close** (conductor) — the verified-core CLAUDE.md sections for
  `core`/`analysis` (incl. the `inv-determinism` sharpening: facade sortedness =
  named-seat + Kani-pin, the honour-system move stated in law text) · FORFEITS rows if
  any arise · prompt-review pass on all CLAUDE.md edits · ledger/LIVING_STATUS
  currency · the wave-one-close gate run (§4).
- **lane-definition-fixtures — FOLDED 2026-08-15 (close batch)** — five `floor30-*`
  differential manifests + the `pin30` EXPECTED-TO-FLIP engine golden
  (`spike/crates/cli/tests/`). dash∩posh agreed on every cell (the plural-definition
  idioms are INSIDE the base dialect — the load-bearing null result); every manifest is
  the shells' own answer, hand-predicted then confirmed; gate-9 falsified both
  directions (live under opt-in, inert by default). Ground truths banked for the
  conversion: an in-subshell `unset -f` dies at the paren like a definition; deep-stack
  helper binding follows the frame at the CALL. Deviation adjudications
  (deviation-litmus, each re-derived): scoped-bless ENDORSED, conductor mistake named —
  no sanctioned builder path exists for new-case transcript minting; comment-budget
  REVERSED against the conductor's own brief-rule for never-churned calibration records
  (rationale re-widened this batch); the subshell-cell widening, the sibling-frames
  munge reading (the delivered cell IS the hash-munge activation shape; the alpha-rename
  premise is engine territory, banked for the snapshot-emission brief), and mocks-log
  byte-identity ENDORSED; `pin30`'s EMPTY probe-results ENDORSED — gate-1 itself is the
  flip alarm, and the conversion lane must AUTHOR the record, never bless past (rider
  carried to that brief). BANKED `work-bless-emitted-manifests` (post-close tooling):
  bless silently discards a floor cell's measured `expected.emitted` (scratch-written,
  never folded back) — loud-refusal lean; plus a sanctioned builder path for new-case
  transcript minting.
- **lane-audit-application — FOLDED 2026-08-15 (close batch)** — D1/B1/B2-B3/B6 applied
  per `300b`: solve docs read trusted()=CERTIFIED with `converged` advisory
  (`solve-is-certified-only` minted in analysis/CLAUDE.md); the `fmt` task itself
  repaired (task-env `HK_FIX = "1"`, load-bearing comment) and the taught-workaround
  bullet replaced; step-zero carries pwd-first + `git -C`-every-command with no named
  lineage branch; the verify:kani rows landed with the evidence-grounded cbmc-only
  reaper name (the audit's guessed extra process names REFUSED — fidelity, not
  deviation). B1 SHAPE deviation adjudicated ENDORSED with the conductor mistake named:
  the brief paraphrased the ruling ("add a task") instead of quoting it ("fix the fmt
  task"); prevention: briefs quote rulings verbatim. CORRECTION banked: F10's "HK_FIX=0
  refuses" was a misdiagnosis — `hk fix` under `HK_FIX=0` silently substitutes each
  step's CHECK command (exit-0 no-op on a clean tree), and hk's own error text taught
  the hand-derived invocation. B5/C1/D2/E1 adjudicated APPLY-with-amendments, landed
  this batch (B5 two-part lock restored · C1 scoped to the translated algebra tier
  pointing at `spike/verify/aeneas/src/lib.rs` · D2 = the analysis-half of the reshape
  rider, homed in the dangers section · E1 folded per registry discipline). The
  converged-wording sweep (5 sites) + the nine-crate-files count landed this batch.
  A1–A3 remain human-owned.

- **the close batch — FOLDED 2026-08-15 @ `8889618f`** (ff'd to conduct; all gates
  green: both-legs full gate 2032/2028 + both-legs floor 140/140 + `bless:dry`;
  zero drift). **WAVE-ONE-CLOSE IS STAMPED.** Flag adjudications (deviation-litmus):
  the five hand-edited floor30 transcripts (rationale re-widening changes committed
  transcript bytes + the `book=<sha256>` digest; bless was brief-forbidden)
  ACCEPTED-AS-VERIFIED — the e2e production-bytes gate proves them — with a DOUBLE
  praxis miss logged: the builder should have paused-and-asked before a functional
  hand-bless, AND the conductor's brief ordered golden-touching work while
  forbidding the only write authority (prevention: briefs name the sanctioned write
  path or pre-authorize scoped bless — done in `305`); THIRD instance of the
  floor-transcript tooling gap ⇒ `work-bless-emitted-manifests` priority raised.
  both-legs-run-separately ENDORSED — the TOOLING is wrong, not the builder
  (`mise run both` sets no WSL-local `CARGO_TARGET_DIR`, violating
  wsl-unix-leg-at-fold; `clippy:clean`'s task-level target-dir override wipes the
  shared lint dir over drvfs) ⇒ BANKED `work-both-task-wsl-target`. Disk exhaustion
  mid-gate: environmental, not Dorc-caused, self-cleared; HUMAN-attention items:
  NINETEEN stale `~/.cache/dorc-wsl-target-*` lane caches inside an 87.9GB
  ext4.vhdx (deletion + vhdx compaction are human acts; banked policy question:
  reap lane caches at fold?). The self-caught never-filter-a-task violation noted
  (re-run unfiltered; no result rests on a filtered run); placement latitude
  accepted; the builder's spot-check of the conductor's own ledger claims before
  landing them endorsed as the litmus's two-way form.

**The post-close debt-paying pass** [human-directed 2026-08-15, while the rewind
waits on their design-review thread]: "get the builds SAFE against RAM and
disk-space so these failures stop happening; pre-gate the build-scripts with
bounds-checking against expected usages and complain loudly before spending time
and resources" + general mess-cleanup; disk usage is "usually worktrees piling up".
Three lanes, conduct-branch-based:
- **lane-resource-safety** (FOLDED, this batch): the preflight bounds-check seat in
  `internal-tooling` (per-profile expected disk/RAM vs actual, LOUD pre-spend
  refusal, per-leg, wired into the heavy tasks; hot-loop stays unburdened) ·
  `work-both-task-wsl-target` (fix `mise run both`'s missing WSL-local target +
  `clippy:clean`'s shared-dir wipe) · a worktree/cache doctor inventory · reap of
  verified-contained-and-clean dead agent worktrees + OUR stale
  `dorc-wsl-target-*` caches (regenerable build artifacts; the vhdx compaction
  stays the human's) · unremovable branch refs listed for the human, never
  force-deleted.
- **lane-floor-transcript-tooling** (FOLDED, this batch): `work-bless-emitted-manifests` —
  bless LOUDLY REFUSES an `expected.emitted` case by default, plus an explicit
  opt-in re-measure-and-write path (floor shells set + a dedicated flag) so
  minting/amending floor transcripts stops being a hand-bless; measure-once stays
  the doctrine, the default path never writes.
- **lane-kani-battery-reshape** (queued behind the two above; WSL-heavy,
  serialized): the 18 over-budget harnesses to concrete lengths per `300a` §1's
  measured shaping rule · the `any_minted` assumed-canonical generator paired with
  a mint-satisfies-the-invariant closing harness (the adjudicated direction) · the
  toolchain-less `cargo check` of the detached harness crate in the lane task.
- **lane-floor-transcript-tooling — FOLDED 2026-08-15** (5 commits; both legs green,
  zero drift at its tip). Landed: bless LOUDLY REFUSES an `expected.emitted` case
  (naming `mise run bless:floor`); the double-opt-in mint (`BLESS_FLOOR=1` ∧ the floor
  shells) re-measures and writes manifest + transcript + `book=` digest in ONE act;
  between-binaries disagreement still refuses; a single-binary mint refuses (the
  manifest is the two-binary agreement record — so Windows never mints a half floor);
  `bless-folds-only-on-pass` landed after a full failure-site enumeration proved no
  workflow depends on partial folding (every golden-compare is bless-aware; XFAIL's
  blindness intact; XPASS folds nothing); landmine docs landed (empty-`expected.ran`
  two-spellings; posh-`echo` agrees with dash — no real dash∩posh divergence found for
  gate-9's disagreement branch, the test substitutes `dash,bash`). ADJUDICATIONS
  (deviation-litmus): the single-binary refusal and the appended `test:floor`
  verification are beyond-brief and ENDORSED, conductor gaps named — the mint's quorum
  and its post-write verification were doctrine-implied but unstated (prevention: state
  doctrine-implied invariants when building a new writer for an artifact class); the
  flagged-not-fixed handling of the partial-bless bug is the deviation-praxis MODEL CASE
  — question asked, conductor answered with a required dependency analysis, the analysis
  cleared it, the fix landed pinned.
- **lane-resource-safety — FOLDED 2026-08-15; ~67 GiB recovered.** Landed: `preflight`
  (per-profile disk/RAM bounds vs actual, per-leg, loud pre-spend refusal,
  `DORC_PREFLIGHT=skip` hatch, measured-with-provenance bounds; the kani RAM bound
  pinned above the driver's address-space cap BY A TEST) + `doctor` (read-only pile-up
  inventory) in `internal-tooling`, wired into the heavy tasks, hot loop untouched; the
  `both` task's WSL leg auto-gets a per-worktree ext4 target (the drvfs-clobber fixed,
  canary-proven) and `clippy:clean` composes onto it. Dep: `fs4` minimal — the builder
  REJECTED the brief-pre-authorized `sysinfo` on measurement (~150 MiB rlib in a
  disk-guarding tool) and hand-rolled RAM dep-free (`/proc/meminfo`; `wmic`→CIM fallback;
  understatement errs toward refusal; unmeasurable ⇒ warn-and-pass): ENDORSED, conductor
  mistake named — a dep pre-authorized by name, unsized (prevention: size before
  pre-authorizing). Reap: 3 verified-contained worktrees + 18 stale caches (~67 GiB;
  `df`-exact); the CONCURRENT floor lane's cache correctly excluded — the flag-not-delete
  posture validated, and a conductor sequencing rule minted: REAP LANES RUN SOLO or with
  an explicit exclusion list. Comment-density deviation (22%/17% vs the ~10% budget)
  ENDORSED — the brief mandated per-bound provenance while carrying the numeric budget,
  the SECOND budget-vs-mandate collision (prevention: a brief that mandates comment
  content adjusts the budget in the same breath). RULED conductor-side [human veto
  welcome]: lanes DELETE their own `dorc-wsl-target-<worktree>` cache at fold-complete —
  the banked reap-at-fold question resolved as self-cleanup of regenerable artifacts.
  HUMAN-ATTENTION list: the vhdx compaction; the `-D`-only branch refs
  (`ai/r30-lane-definition-fixtures` @1bb304bc · `ai/r30-lane-audit-application`
  @bd56ef68 · `ai/r30-lane-reports-cleanup` @d1097791 · `ai/r30-lane-kani` @bddead65 ·
  post-fold, the two debt-lane refs); the SyncThing conflict-file cleanup in the resource
  lane's worktree (three untracked `Cargo.sync-conflict-*` files — human-owned per
  standing law; that worktree reads dirty until cleared, so its reap waits on them).

### §2a — Facade-fold bank (consumed by lane-kani, the derived-defs lane, and Flux)

Invariant seats (seat · invariant · pinning test; all tests in the default suite):
- `core::sorted::SortedSet::insert` — strictly-ascending, duplicate-free backing —
  `set_insert_sorts_and_dedups`; and structural `PartialEq` == semantic set equality
  (what `solve`'s `joined != state[w]` fixpoint test rests on) —
  `set_structural_eq_is_semantic_eq`.
- `core::sorted::SortedSet::position` — membership agrees with backing at every
  boundary — `set_contains_and_remove_agree_with_membership`.
- `SortedSet::union`/`intersection` — canonical results; ∪/∩ commute; ∅ identity/
  absorbing — `set_union_and_intersection_stay_canonical`.
- `core::sorted::SortedMap::insert` — ascending unique keys; rebind replaces+returns —
  `map_insert_sorts_keys_and_replaces_values`; structural==semantic Eq —
  `map_structural_eq_is_semantic_eq`.
- `SortedMap::remove`/`get_at` — order survives removal; `get_at` walks key order —
  `map_remove_and_get_at_keep_key_order`.
- `analysis::lattice::MapL::insert` (pre-existing) — no key maps to `V::bottom()` —
  `maplattice_is_pointwise_and_canonical`; plus insertion-order-independence of
  `Powerset`/`MapL` equality — `collection_domains_are_insertion_order_independent`.

Kani-lane guidance (builder-supplied, conductor-endorsed): the canonical predicate is
strict ascent — `∀i: get_at(i) < get_at(i+1)` (sortedness+dedup in one; maps over
`get_at(i).0`); NO `pub is_canonical()` was added (harnesses express it; add in the
Kani lane only if needed). `#[cfg(kani)] Arbitrary` homes in `core::sorted` (reaches
the private field, no widening) and must construct via arbitrary `Vec` +
`kani::assume(canonical)` — building via repeated `insert` would make the `insert`
harnesses circular. The asymmetric risk the harnesses exist to close: a bug making two
semantically-DIFFERENT values compare equal stops the solver's climb early
(under-approximated may-set ⇒ potential wrong elision, invisible to goldens); the
opposite bug only trips `converged: false`. Until Kani lands, the seat tests are the
whole net.

Findings + conductor adjudications:
- `fnd-reach-lattice-outside-scope` — `analysis::effect::Reach` (a `Lattice` impl in
  engine-tier `effect.rs`) still holds a raw `BTreeSet<FactKey>` + a hand-written
  cause-excluding `Eq`; the algebra tier is NOT BTree-free. ADJUDICATED: eviction
  deferred (not this wave; careful territory — the cause-excluding Eq is
  correctness-critical); the derived-defs lane EXCLUDES `Reach` from translation scope
  at v0 and says so in its config; revisit when a Lean statement first needs
  reaching-defs.
- `fnd-generic-ord-blocks-refinement` — facades stay generic over `T: Ord`; Flux needs
  concrete decidable orders, so the Flux lane (mid-r30) prices harness-side
  monomorphic instantiations (`SortedSet<SelectorId>` etc.), never product-code
  monomorphization.
- `fnd-iterator-exits-may-not-translate` — `iter()`/`IntoIterator`/`FromIterator` are
  grouped+commented as the translation boundary; the algebra proper avoids them. If
  the Aeneas pipeline chokes on the ALGEBRA (not the exits), the `while let
  Some(x) = v.get(i)` shape is unusable and the facade needs re-shaping — report,
  don't patch.
- `dec-shared-facade-home-in-core` — RATIFIED: one shared facade in `core` (both
  crates consume; core stays dependency-free) is the justified dislocation from
  `301` §3's crate-local default; dividend: one Kani harness set covers both crates.

[CONDUCTOR ratification, 2026-08-14] Nested-vs-root mise configs: toolchain-SHADOWING
pins live in nested configs (the in-repo Aeneas precedent); additive-only pins (elan)
may live at root.

[CONDUCTOR staffing, standing] Fable authors SPECS and reviews STATEMENTS
(`plans/302`; minispec content under the `301` access laws; harness-statement review
at fold); Opus authors bodies, tests, and toolchain wiring. Neither Kani nor the
certifier runs full-Fable or in-conductor-implementation.

### §2b — Late-wave folds (2026-08-15)

- **lane-sparing-rederivation (b) — FOLDED @ `0c1cefcd`** (four commits; builder gates
  green both legs 2008/2004 + bless:dry, zero drift; conductor gate over the combined
  tip at fold). As-built: the three model amendments landed with flags retired and the
  epistemic rider applied (the "proves" wording in the model's own variant doc
  sharpened too); `plan::rederive` (adapter: canonicalized entities, typed refusal on
  unresolvables, ∅-footprint = named differential failure; kind-fence pairing leaves
  entities unread — a real adapter bug the differential caught at seed 3);
  `recheck_survival` seated INSIDE `wall_walk_survival`'s Survived arm (a post-pass
  demote would un-wall downstream sites — the lane's one genuine correctness trap,
  correctly dodged), by-value witness in / Confirmed-or-Demoted out, demote-only
  pinned three ways; 8000-seed differential (two tests + non-vacuity censuses + a
  mutant control): ZERO production↔model disagreements. FLAG ADJUDICATIONS
  (conductor): `flag-backing-mintedness-is-translated` ACCEPTED (the backing-side
  dialect-membership conjunct is adapter-computed — the differential's one disclosed
  coverage limit; documented in the test header; revisit if the model ever grows a
  dialect-lookup of its own) · `flag-plan-gains-a-dependency` ACKED (dorc-sparing-
  reference production + dorc-hostsim dev; both in-workspace zero-dep) ·
  `flag-diag-not-in-identity-diags` ACCEPTED (the demotion rides the decision digest;
  double-count avoided). The proposed plan/CLAUDE.md bullets ride discipline-close.
- **THE MINISPEC REMIT IS REAL (conductor-authored, 2026-08-15)** — `TrustedBase`
  vocabulary (LawfulClone/LawfulEq as named trusted-base entries + the lawful-by-
  construction U32 battery ground) + the three law units (JoinIsCommutative ·
  JoinIsIdempotent · JoinIsAssociative — the third renamed from the LeqIsReflexive
  placeholder; derived-leq reflexivity is idempotence in disguise), stated over the
  DERIVED `Flat` join with hypotheses in the statements, batteries + non-vacuity
  proven by reduction (`rfl`), lake green (the 4 dependency-closure holes are the
  known aeneas-own trusted base, unchanged). Badges: elaborated + interrogated EARNED
  ×3, claimed through the promote ceremony (silent-ambition refusal observed working,
  then satisfied); the verified boundary renders its first real seat
  (`dorc_analysis::lattice::Lattice::join`). TWO CHAFE FINDS from authoring (the
  harness's first real user): `fnd-vocabulary-home-was-unrepresentable` — the binder
  had no governed-vocabulary concept and demanded a law of TrustedBase; FIXED
  properly (`Minispec/Vocabulary/` walk: unit-contract-exempt, hole-censused — a
  vocabulary hole vacates every importing unit; fixtures + two tests) ·
  `fnd-promote-subcommand-missing` — `catalogue_lock.rs`'s header names a
  `dorc-verify promote` generator that DOES NOT EXIST; the promote act is currently a
  sanctioned hand-edit (the header's own review-is-the-git-diff rule); the generator
  is residue for the harness's next builder.

## §3 — The Lean-tier vehicle (RESOLVED in substance, 2026-08-14)

- [TYPED] Aeneas is a must/of-course, if the tier exists at all: machine-correlation
  is the entire point where correlation is available, and the seam's brittleness under
  regeneration is the drift-alarm working, not a cost to engineer away.
- The maintained artifact is **minispec** (`notes/301`): hand-written statements +
  instances over Aeneas-DERIVED definitions, proofs where cheap. The earlier
  hand-model and Aeneas research spikes are QUARRY, never seed [TYPED].
- The recorded translation limits (sealed tiers, phantom `Must`/`May`,
  smart-constructor privacy do not cross) are compile-time discipline that rustc keeps
  enforcing over the real code — they never needed to cross; the Lean tier's job is
  equational law over the bodies, which translate faithfully.
- The churn-measurement question dissolved with no-pilots: the derived-defs pipeline
  is simply maintained; there is no vehicle decision left to gate on it.

### §3b — SUCCESSOR HANDOFF (written 2026-08-15, second environment death; the
minting conductor is at token wind-down and the human is asleep — no adjudication
until they wake)

A SECOND harness death (same WSL/memory failure mode; the harness now runs OUTSIDE
WSL — cap slightly relaxed, problem unsolved) killed three in-flight agents. They are
being resumed with durability-first + memory-guard riders (per-harness `timeout` +
`ulimit -v` + exact-name reaping; commit-early-commit-often). The minting conductor's
remaining job is COLLATION ONLY: land their reports as durables. Everything below is
the SUCCESSOR's:

1. ~~Fold `ai/r30-lane-kani`~~ — **DONE 2026-08-15** (folded @ `3563584c`, conduct
   branch; statement-review passed; adjudications + `work-kani-battery-reshape`
   banked in §2's row. Historical handoff detail below stands as written:)
   — the lane was COMPLETE and awaiting fold only
   (branch tip `bddead65`, base `5e6d6788`, 12 commits; its own durable report at
   `notes/300a-kani-lane-report.md` IN-BRANCH, written cold). Landing
   facts: 19/37 harnesses verify green at bounds, 18 OVER-BUDGET (unjudged, not
   broken — recorded per the memory-gate discipline), ZERO counterexamples — green
   covers the ternary consumer-map + universal-meet law-set, the selector
   chokepoints, facade canonicality incl. across reallocation, and every lattice
   law over Flat/Product/May/Must (the Must pass IS the order-dual proof).
   `seam-kani-pairing-unbuilt` CLOSED (toolchain-resolved by-name pairing; three
   outcomes distinct). Census fix landed: REPORT axioms 26 → 13 as `304`
   predicted. CRITICAL process save, verify at fold: CBMC prints
   `VERIFICATION:- FAILED` after its OWN out-of-memory, and the first battery
   therefore reported THREE FALSE counterexamples against the coordinate algebra —
   fixed in `49dd1bca` with a regression test carrying the real bytes; the
   successor's statement review should re-confirm that gate ordering. Placements
   flagged (successor reviews): `Reach::eq` → exhaustive-small in effect.rs
   (BTree-backed, Kani-unreachable; migrates when Reach hits the facade) ·
   `normalise_edits` → exhaustive-small in plan · narrative-fold permutation pins
   NOT built (DST-shaped). Windows gate green 1951/1951 at final commit; WSL leg
   skipped under a stated allowance-read the successor should verify at fold
   (no cfg-gated additions claimed). Second-opinion asks the lane itself raised:
   the root-vs-nested Kani pin placement, and `Dialect::any_minted` building
   through real `mint` calls (faithful, but what pushes the three `compare_*`
   harnesses over the address-space gate). The rustup-nightly disclosure repeats
   the earlier-ruled kani-setup exception — same ruling extends. THE FOLD +
   Fable-tier harness-statement review (law · bounds · what is NOT pinned) remain
   the successor's, per the staffing split.
2. ~~Adjudicate the prompt-review audit~~ — **DONE 2026-08-15 (close batch)**; the
   adjudications and what landed are in §2's `lane-audit-application` row (A1–A3 stay
   human-owned). Historical detail below stands as written: the audit itself
   LANDED (report committed at
   `notes/300b-prompt-review-audit.md`; twelve priority-ordered proposals +
   seven reasoned rejections; criteria AGING at 103 days). The minting conductor
   applied ONLY the two factual path fixes (F1: the stale `Minispec/TrustedBase`
   paths in minispec/CLAUDE.md + the three unit docstrings, post-Vocabulary-move;
   B4: the spike/CLAUDE.md quarantine path corrected to
   `Research/quarantine-DO-NOT-READ/`). EVERYTHING ELSE awaits the successor's
   judgment — headline items: D1 (the analysis/CLAUDE.md + solve.rs
   "MUST check `converged`" lines now CONTRADICT the certifier law — the new law is
   verified true; reword the old) · B1 (the fmt-under-agent-env bullet teaches a
   hand-derived invocation against the never-hand-derive law — better outcome is
   fixing the `fmt` task and deleting the bullet) · B2/B3 (step-zero lacks
   `git -C`, names a seven-rounds-stale lineage branch) · B5/C1/D2/E1 (scope and
   placement of the new law bullets) · A1–A3 (conductor-skill wording; A2 — the
   AFK rule's unclear-trigger default — WIDENS human-authored text and wants a
   cheap explicit human ack; A3 — the skill's frontmatter description has ended
   mid-sentence forever) · B6 (rides the kani close-out) · and the out-of-remit
   catch: `verified-core-discipline/SKILL.md` still points at the research-spike
   Lean home, not `minispec/` — high-value, since that skill orients core-adjacent
   agents.
3. ~~The speech-act gap-check~~ — LANDED and LEDGERED (report committed at
   `notes/300c-speechact-gapcheck.md`): the resolver-claimed vs
   name-floor disjointness distinction is NOT rendered. It dies TWICE: structurally
   at `core::coord::Relation::ProvablyDisjoint` (a bare unit variant — the
   kind-fence, entity-inequality, and dialect-sparing generators merge with zero
   provenance; `EntityResolution::Canonical` likewise covers resolver-produced and
   identity-fallback in one shape), and as a wiring gap (`survival::Crossing::
   via_resolver` — itself only a coarse kind-HAS-a-resolver proxy — feeds one
   uncatalogued `format!` stderr line and is NEVER read by `why.rs::survival_chain`,
   whose derives link renders one undifferentiated `SpeechAct::Derived` template).
   FINDING ONLY, per the epistemic-sharpening ruling — nothing built. Successor
   notes: the structural fix is a provenance-carrying disjoint verdict (a `Relation`
   signature change — license-review-tier, `compare` being THE chokepoint;
   enrichment-era); the cheap wiring half alone is NOT safe first — the proxy's
   coarseness can MISLABEL a fence/dialect disjoint as resolver-claimed
   (pope-sin-adjacent). Open sub-adjudication: whether the uncatalogued stderr
   attribution line violates `one-catalog-no-legacy`.
4. ~~LIVING_STATUS refresh~~ — **DONE 2026-08-15 (close batch)**: the top block is
   rewritten to the wave-one-close-stamped state; see §2's `lane-audit-application`
   row for the batch it rode.
5. ~~The wave-one-close gate~~ (§4) — **DONE 2026-08-15 (close batch)**: gates run and
   green over the folded tip; the stamp is ledgered in §5. STILL OPEN: the human-QA
   list (§4's [TYPED] deliverable — judgment-tier items only, REASSIGNED to the sibling
   conductor + human; mechanical checks run by the successor or an idiot-review lane
   which is DEFERRED to end-of-r30).
6. Open human items unchanged: `302:pin-blast-radius-escalation` · records-8 ·
   kSURVIVAL status-line · the `.wslconfig` hardening option · the codex
   self-commit ACL allowlist.
7. NOT yet done, deliberately deferred by the wind-down: nothing else — every other
   Wave-1 item is folded and banked above.

## §4 — wave-one-close (the handoff gate)

[TYPED 2026-08-15] CLOSE DELIVERABLE, chat-tier: a brief **human-QA list** that
EXERCISES the arc's work — tests to break (and in what way), subtle incorrectness
to inject that must trip the new safeguards, specific CLI invocations
demonstrating the new tooling — optimized for minimal human effort × maximal
chance of surfacing unexpected choices, holes, mistakes, or underspecifications.
(Prior arcs smuggled sharp edges under flashy acceptance criteria via the human's
own limited exercise time; this is the counter.) Refined [human, 2026-08-15]:
items need NOT be single commands — fuzzier exercises are expected; anything
mechanically checkable the conductor runs itself pre-close, or routes through an
IDIOT-REVIEW lane (an unprompted agent, FORBIDDEN the docs, given only a goal,
recording what chafes — the loom blind-reviewer precedent generalized; plan one
over the dorc-verify/minispec flows at close). The human's list keeps only what
genuinely needs a human: judgment, taste, fuzzy-seam poking. The list itself is
conversation output at close, never a durable; THIS obligation note is what
survives compression. Conductor collects candidates per fold.

**QA-candidate bank** (collected per fold; HANDLING REASSIGNED 2026-08-15 to the
sibling conductor + human — this conductor stays on forward progress). Judgment-tier
exercises against the wave-1 instruments, each = the poke · what must happen · the
judgment sought:
- qa-certifier-trip-reads-right — locally perturb `solve` (e.g. skip the final
  worklist round); must: whole-window demotion + `solver-consistency-failure`
  surfacing PRE-network with real counts (the R4-repair area); judge: is the
  disclosure intelligible, non-fabricated, root-cause-only?
- qa-rederivation-demotes-only — flip the reference model's kind-fence arm; must:
  every survival demotes to guard/run + a narrative record, agreement never licenses;
  judge: rendered demotion quality.
- qa-promote-ceremony-refuses-both-ways — hand-claim an unearned badge in the
  catalogue lock (silent ambition), separately delete a proof file (silent demotion);
  must: `verify:check` refuses BOTH directions; judge: do refusals route the reader
  to the promote ceremony?
- qa-remit-units-review-surface — read the three `minispec/Minispec/JoinIs*.lean`
  units as the intended non-proof-literate review surface; judge: genuinely
  reviewable? do the TrustedBase hypotheses read as honest assumptions, not fine
  print?
- qa-kani-trichotomy-feel — WSL: `mise run verify:kani` (optionally
  `DORC_KANI_HARNESS_BUDGET_SECS=5` to watch over-budget classification); Windows:
  the one-line refusal; judge: does green/over-budget/failed read honestly?
- qa-why-chain-taste — plan + `dorc why` over a scratch book of the human's own;
  judge: survival-chain feel, knowing the ledgered speech-act gap
  (resolver-claimed vs name-floor disjointness renders undifferentiated — what ELSE
  chafes?).

All lanes folded to the conduct branch (`ai/main` promotion is a separate human-directed
act while the QA freeze holds) · `mise run both gate:full-quiet` green + `bless:dry`
clean · certifier + re-derivation live in the DEFAULT suite · the Kani lane opt-in and
documented · minispec standing (skeleton; the remit claims at their earned badge-sets;
binder v0 + the generated report; the first bound demonstration; `minispec/CLAUDE.md`)
· the derived-defs lane green · CLAUDE.md discipline sections landed + prompt-reviewed
· ledgers current (this file, `notes/301` if amended, LIVING_STATUS, FORFEITS) ·
conductor worktree/branch cleaned or handed over deliberately. Successor boot order:
LIVING_STATUS → this file → `notes/301` → `plans/28Q` → `spike/CLAUDE.md`. The NEXT
stage's first acts: a full root `ANALYZER-NEEDS.md` read (it owes the `an-flat-domain`
reconciliation paragraph, `28Q` §7), then 28Q stage-i's fixtures-first commissioning
per `28Q` §8.

## §5 — Ack-ledger (what the human has TYPED this arc; silence is never ack)

- 2026-08-14, the plan-ack sitting: the six-lane Wave-1 plan ACKED (with the stated
  leans: rederivation-in-scope · vecdeque-stays · needs-ledgers-deferred) · codex
  dispatch ACKED ("use as you see fit") · the research branches deleted by the human ·
  round-30 minted, notes/300 assigned · the sequential-conductor/rewind protocol
  directed (§1).
- 2026-08-14, the design sittings (the reshape this §2 reflects): Aeneas
  must/of-course · the small reviewable surface is a core product (literate
  colocation; the rationale is LLM attention-forcing, per the errorloom precedent) ·
  model-writing is design-work — the spike models are quarry; minispec's remit is the
  2–3-claim minimum; enrichment is a standalone human-led item · the runtime checkers
  and formalization-as-question-generator hard-ACKED · no pilots / no measure-kill
  stages, velocity · the independent-voices/lineage framing deweighted — a checker's
  value is structural asymmetry (finder/checker under different constraints) ·
  out-of-scope is a human judgment, never machinery (taxonomy/strength-axis repairs
  nacked) · mutation-testing is a gentle-must (badge defined day-one, `301` §5);
  property-testing stays the general check-ladder, never a spec badge · an automated
  performance-regression lane (CI graphs + hard gates) is banked for someday, out of
  scope · the whylog decision record is the assertion substrate, under the [TYPED]
  framing that huge amounts of Dorc are modelable as a deterministic mapping from
  source through probe-results to whylog result · doc routing: notes/301 minted as THE
  minispec/verify spec; this file carries the rest; `plans/28Q` edits minimal; `28T`
  markers-only; `plans/302` = the certifier spec (renumbered under the routing).
  The 301-interior rulings (access laws, remit, badges, bindings, naming, byte
  tripwire, local-homing default) live in `301` and are not duplicated here.
- 2026-08-14, session close: Flux DEFERRED [TYPED] — penciled mid-r30 (post-Lean
  standup, pre-stage-i), defense-in-depth intent standing, with the
  required-changes-ride-Aeneas-prep exception (§2's facade rider).
- 2026-08-14, the greenlight sitting (post-rewind): certification machinery ruled
  sketch-until-demanded (`301:post-certification-sketch-until-demanded` — architecture
  + cheap tooling + named seams; upfront depth is conductor/builder latitude) ·
  imported tools built general, never minispec-scoped
  (`301:law-imported-tools-built-general`) · proceed GREENLIT, conductor discretion
  ("I am here to work with you").
- 2026-08-14/15, the certifier duck-sitting (sibling-conductor context; product =
  the `plans/302` recut): NO RECOVERY [TYPED — "nothing here to salvage; it doesn't
  get a row": no FORFEITS entry; rationale distilled in `302` §9, the one recorded
  direction-not-taken] · aid-leads-the-engine [TYPED, standing, product-wide: aid is
  part of the correct output; the certifier triggers, the engine self-reports] ·
  "whole-window demotion" is the term (the conversational "kill" retired, never used
  in durables) · the naming set RULED: `SolveConsistency`/`Consistent | Inconsistent`
  · component stays "solve-certifier" · `Inconsistency{Boundary,Edge}` items ·
  `SolvePass` reason enum · `solver-consistency-failure` DiagCode ·
  first-break/unstable-component summaries · replay vocabulary · floors unchanged ·
  old notes left un-annotated by choice (rot endemic; r30 made current instead).
- 2026-08-14, the builder-prerequisite dictum [TYPED]: landed as durable law in the
  conductor skill itself (its quarantine section; human commit). §2 carries the
  pointer; successor conductors get it from the skill at boot.
- 2026-08-14, certifier-spec HOLD [TYPED]: the human is reworking `plans/302` with a
  sibling conductor. Everything certifier-shaped HOLDS until it settles: the phase-1
  proposal agent completes as read-only recon (its census/floor-table halves are
  spec-independent; its type-shape half will be re-cut against the settled spec), but
  NO phase-2 greenlight, and lane-sparing-rederivation stays queued behind it. The
  `302` §3 refusal-loudness [HUMAN?] flag is dropped as pending — it resolves inside
  the human's sitting. This conductor's `302` is superseded-in-place by whatever the
  sitting produces; re-read before any certifier act.
- 2026-08-14, phase-1 LANDED under the hold → **`notes/303`** (census · the four
  existing Refused-floors · eleven spec findings, several of which correct `302` as
  written — the sitting's input). Kani-lane list grows one row from it:
  `303:fnd-reach-equality-excludes-its-cause` (`Reach::eq` is trusted by the
  certifier with nothing pinning it). Lane branch `ai/r30-lane-certifier` exists,
  clean, zero commits — the phase-1 agent's context resumes for phase-2 if this
  session survives the hold; else redispatch fresh against settled-`302` + `303`.
- 2026-08-15, **rul-whylog-is-the-spine** [TYPED substance]: the whylog is the CORE
  PRODUCT — every engine decision passes through it, and every other surface (the
  plan render first among them) is a filtered VIEW over whylog × input-files. The
  standalone "decision-dump" DISSOLVES into whylog enrichment: full `SiteId` keying
  of the decision digest (the `leaf: u32` member-collapse fixed), phase-growth
  minting (a plan invocation mints the decision half; apply appends its report), and
  a read surface reachable from test/loom contexts. Conductor-stated boundaries,
  accepted as refinements not nacks: (1) the spine records DECISIONS and their
  scalar/interned accounts — never arena handles or working lattice state
  (`operands-are-pure-and-capped` generalizes to the file; the certifier's by-value
  items stay in-memory/pull-tier, F8); (2) `law-whylog-is-sensitive` stands —
  enrichment adds engine-derived rows only, host-sourced material stays in its typed
  intake lane within the file, the secrets-round re-grade before real estates still
  binds; (3) rec-5 stands — no whylog ever feeds the LICENSE plane of any run, its
  own included; re-ingestion is aid-plane only, and the spine never becomes the
  kSTATE cache by accretion. (Wording narrowed 2026-08-15: the earlier "consumers
  read the whylog OF THEIR OWN RUN, never a stored one" collapsed the CONCEPTUAL
  whylog — the engine's single output structure, of which every product including
  the `.whylog` durable is a projection — with that durable itself, and as written
  outlawed `dorc why --last`, which reads a stored durable by design.) Consequences
  banked: the records-8 pending decision leans WIRE (a spine wants its emitters);
  renderer-consumes-whylog is a banked refactor direction, opportunistic, never a
  big-bang; enrichment work stays demand-driven per sketch-until-demanded (the
  first product-behavior law binding is the natural trigger).
- 2026-08-15, certifier-lane worktree incident: the phase-2 builder's UNLOCKED
  harness worktree was reaped mid-task (the locked reshape worktree survived);
  zero commits lost, nothing mutated; the builder's near-miss (silent cwd
  fallback into the conductor tree with a `reset --hard` queued) is now law —
  `spike/CLAUDE.md` worktree-file-access-law sharpened (git -C absolute paths;
  verify before every mutating git command; stop-don't-improvise on a vanished
  tree). Builder resumed with a conductor-directed self-minted worktree.
- 2026-08-15, THE WSL-DEATH INCIDENT (post-mortem, cause settled): the harness
  process (claude.exe, child of a WSL-zsh) died with the human's terminals when the
  WSL2 VM OOM'd under the Kani lane's CBMC solver load — a measured 3.6GB/21min CBMC
  earlier in its window; SIX further runs stopped mid-climb; and the demonstrated
  hazard that **TaskStop does not kill WSL-side CBMC** (orphaned multi-GB solvers
  accumulate; reap explicitly with `pkill -9 -x cbmc` — exact-name, never `-f`,
  which once matched the lane's own wrapper shell). At death, an unmeasured
  37-harness battery was running into exactly the blow-up shape. No shutdown
  command was ever issued by any lane; the rederivation lane is exculpated (zero
  WSL contact; full clean ledger). RULINGS: the implicit
  `rustup toolchain install nightly-2025-11-21` into `~/.rustup` (fired by kani's
  own first-time setup, undisclosed-until-after by the tool) is WITHIN the
  pre-authorized kani-setup exception — disclosed properly, additive, reversible;
  keep it, document it in the lane's toolchain notes. Standing discipline from
  here: WSL-heavy lanes run SERIALIZED (one lane's WSL work at a time; conductor
  sequences); the Kani battery resumes only under a hard per-harness timeout + an
  explicit CBMC reaper (the lane's own proposal, made mandatory); its three probe
  cache dirs get deleted at resume. Discipline-close item: the
  TaskStop-does-not-kill-remote-children hazard + the exact-name-pkill rule belong
  in `spike/CLAUDE.md`'s build/run section. The blow-up fix itself is already in
  the lane's tree and measured (capacity-headroom generator: 21min/3.6GB → 2.2s).
  Forensics sharpening (scout, +SURE on kernel-log evidence): the fatal CBMC grew
  to ~15.16GB RSS — the ENTIRE WSL VM budget — and was the OOM-killer's sole
  victim (single-process blow-up, not a swarm); the terminals actually closed
  ~90s LATER via a `wsl --shutdown`-shaped teardown of the session tree, issuer
  unrecorded (~SUSPECT a reaction to the thrashed VM; Windows logs don't capture
  `wsl.exe` invocations). The box has NO `.wslconfig`, so the VM defaults to
  ~15GiB of the 31.7GiB host. OPTIONAL human hardening item: a `.wslconfig`
  memory raise buys headroom, but the lane disciplines are the real fix (a
  runaway solver eats any cap).
- 2026-08-15, the wave-2 ack sitting (successor conductor): the kernel-rewrite plan
  ACKED as explained; proceed on genuinely-INDEPENDENT work only (the
  definition-fixtures commissioning · the kani fold + statement review · the
  prompt-audit's builder-facing items · the ANALYZER-NEEDS read + owed paragraph ·
  wave-one-close + the QA list); never pre-do work that the human's concurrent input
  may change. **`ai/main` is FROZEN as the human's QA touching-point** — every fold
  lands on `ai/r30-conduct` until the human directs promotion. A SIBLING conductor
  concurrently runs the closure-custody sittings (née stage-ii) and the human-items
  queue (`302:pin-blast-radius-escalation` · records-8 · kSURVIVAL status-line ·
  `.wslconfig` · codex ACL — REASSIGNED off this conductor); this conductor stops
  wherever that sitting's feedback becomes load-bearing. Git-siting law: the
  conductor uses `git -C <path>` on every git command; builders are born in harness
  worktrees; never stomp sibling worktrees.
- 2026-08-15, [TYPED substance] **the flat-domain/k-CFA line DEMOTED**: it was a very
  early, prospective, pre-code feasibility weld — academic-research-based, predating
  the ruling performance posture (network + multi-host dominate hard; performance is
  USER-ATTENTION SCHEDULING first, wallclock second; once the user has stood up from
  the keyboard, the constraint is concurrency, pre-optimization/work-dropping, and
  user-tuning — not algorithmic complexity). The owed ANALYZER-NEEDS `an-flat-domain`
  paragraph must HONESTLY assess whether frames breach the old line — the human
  suspects the frame design does breach it and doubts they care; never contort to
  claim non-breach (the plan's "this is NOT context-sensitivity in the k-CFA sense"
  framing is suspect sycophancy-of-the-spec). BANKED, low-priority: a
  performance-choices analysis work-item under the attention-scheduling framing
  (`work-performance-posture-analysis`; human wants it eventually, explicitly not
  now). KNOBS `kCONTEXT`'s redline status-line is a candidate edit for the
  human/sibling sitting, deliberately not touched by this conductor.
- 2026-08-15, naming law (standing; propagate to every brief and durable): semantic,
  constant, descriptive name-slugs for stages/lanes/concepts — never bare
  "phase-N"/"pillar-N"/"stage-ii" as a primary handle; numerals only where genuinely
  ordered, and always beside a name. (Memory-persisted; the 28Q stage names to use:
  the definition-factoring stage · the snapshot-emission stage · closure-custody ·
  world-scopes.)
- 2026-08-15, [TYPED] **no parallel durable dirs**: the accreted `.claude/reports/`
  is a divergence — every durable lives in the single Research docID system.
  Sub-reports that logically parallel an existing doc and aren't worth a full docID
  get SUBSCRIPTED docIDs (the `notes/303a-foo-bar.md` shape); otherwise they are
  deleted. Sole exception: `.claude/research/` (deep per-topic research, summarized
  into a docID at completion, durable only for reference from that summary doc).
  Cleanup EXECUTED + FOLDED (Sonnet lane + conductor extras-pass, 2026-08-15); the
  landed mapping: kani lane report → `300a` · prompt-review audit → `300b` ·
  speech-act gap-check → `300c` · certifier cross-lineage review → `303a` (beside
  the certifier census note) · sparing-reference lane report → `300d` ·
  sparing-reference dispatch bundle → `300e` (KEPT deliberately: it is the
  independence RECEIPT — the evidence the reference model was authored blind to
  production code, which the checker's structural-difference value rests on) · the
  certifier review's dispatch bundle DELETED (a spent prompt; the review's remit
  self-describes in `303a`; git holds the bytes). The dir is gone. Future briefs
  site reports at subscripted docIDs directly, never under `.claude/reports/`.
- 2026-08-15, [TYPED] QA of the wave-1 instruments REASSIGNED to the sibling
  conductor + human; the candidate bank rides §4; this conductor stays on forward
  progress.
- 2026-08-15, [TYPED] **the deviation-litmus** — conductors are mildly skeptical of
  builders' REASONING about their own deviations, never of their factual claims:
  builder reports are prompts (bottom-up sycophancy is real); the litmus is "would the
  human, un-prompted, have asked for this?"; every endorsement is an APOLOGY whose
  conductor-mistake must be nameable (a scouting gap, or a missing pause-and-ask seam)
  — else the builder was probably wrong: reverse. Praxis misses count even when the
  product call was right. Pre-prompting deliberately NOT tightened. Encoded in the
  conductor skill ("Reviewing builder judgment", `8b01167d`); memory updated.
- 2026-08-15, **wave-one-close STAMPED** — contingent on this batch's gates: both-legs
  `gate:full-quiet` + both-legs `test:floor` + `bless:dry` green over the folded tip;
  the §4 gate is discharged on the conduct branch; `ai/main` promotion remains the
  human's act.
- Standing carry-overs: the `KNOBS:kSURVIVAL` status-line edit remains the human's
  (28T inheritance; now riding the sibling's queue); silence ≠ ack; only typed text
  counts.

## §6 — The settled-rules census (BANKED; the enrichment item's tabled menu)

Gathered 2026-08-14 by a criteria-driven scout over KNOBS · `spike/CLAUDE.md` ·
`crates/{core,analysis}/CLAUDE.md` · FORFEITS · `277` · `271` · `28Q`;
conductor-adjudicated. The MENU IS TABLED [TYPED] — selection happens at the
enrichment item, never before. Criteria: explicit ratification evidence ∧ statable as
value-algebra ∧ off 28Q's moving edge.

Passing (evidence as found):
- `ternary-compare-consumer-map` — acked (`271` task-12 closing sweep 2026-07-12;
  `277` §9). Caveat: the relation shape + consumer map only; the fuller generator
  registry is still conductor-proposed. Named in `28Q` §6's preserved wall.
- `set-lifting-universal-meet` · `pin-set-meet-order-independence` ·
  `pin-no-outcome-as-generator` — ACKED, typed, 2026-07-16 (`277` §5 / the `279f`
  ack batch). The first is named in `28Q` §6; the third is not individually
  (~SUSPECT rider).
- `inv-backing-set-nonempty-by-construction` · `inv-top-never-encoded-as-empty` —
  acked 2026-07-17 (`27Xf:cr-set-lifting-vacuous-at-empty`); the measured
  vacuous-∀ design-bug class.
- `never-derive-separation` — acked "spike-tier-because-foundational" (`271`,
  2026-07-12); named in `28Q` §6.
- `top-identifies-with-nothing` — WEAKEST evidence in the set: "unchanged" across
  three rounds, NO dated typed marker found anywhere. Candidate calibration probe for
  the enrichment item's question-router (it should ask for confirmation).
- `rul-coordinate-shape-flat-three-place` — typed (`271`, 2026-07-10); light moving
  edge only (`28Q` §3 extends the context slot; the flat shape itself unchanged).
- `silence-licenses-nothing` · `inv-top-reject` — named unchanged in `28Q` §6.
- Settled but STRONG moving-edge (excluded from near-term proving): `rul-family`
  (typed, but `28Q` §1/§2 reshape membership frame/closure-relative) ·
  `pure-predicate-carry` (human-opted 2026-07-17, but `28Q` §3 grows its axis
  vocabulary).
- Settled but not value-algebra: `empty-world-byte-identical` (whole-system
  differential property; its evidence stays the corpus differential).

Conductor adjudication deltas:
- `inv-must-may` SPLITS: the coercion ban is compiler-tier (evidence = the
  `compile_fail` seals); the `Must`-as-order-dual SEMANTICS is genuine value-algebra
  and underlies the certifier's one-checker duality.
- MapL canonical-form (structural-Eq = semantic-Eq) fails the scout's
  settledness-marker criterion but enters anyway via the facade lane as its
  honour-system invariant.
- `rul-rc-partition`'s ≥2-flat-sink ("flat FOREVER") is borderline algebra-content;
  benched.

Excluded as not-settled (soft/forfeit/refused): the sparing dialect-resolution core
(typed-spike-provisional + acked-SOFT + `pin-two-position-sparing` extremely-soft +
its FORFEITS row) · `forfeit-committee-fence-sparing-inert` (UNRATIFIED) ·
`kind-fence-movable` (a reserved seam, not a ruling) · the `275` transport
ratifications (REFUSED, `279f` §3).

Doc-coherence note, repaired in `28Q` §6 this arc: "the sparing algebra" in the
preserved wall means the set-meet SUBSTRATE (hard-acked, above); the
dialect-resolution rule is `28Q` §9 `pin-two-position-sparing` territory (soft).
