# 27U — the user-aid build phase: landings, process report, meta-diagnosis

AI-authored (Fable conductor, 2026-07-18/19; the ONE note for the whole build phase by
human ruling — builders wrote no landing notes; their as-built detail lives in the
granular commits on `ai/r27-aid` and in this ledger). Executes `27V` (plan-of-record)
+ the `27W` §5 riders under root `AID-NEEDS.md` law. Authority: root docs,
`spike/CLAUDE.md`, `AID-NEEDS.md` outrank. Branch: **`ai/r27-aid`** (conductor stack;
every lane branch folded by conductor re-point, conductor-verified own-hand at every
advance). Tip at note-mint: `ac57b39` + close-out commits; 958 unit / 97 e2e / four
gates, `DRY=1 sh e2e/conduct-bless.sh` green.

## §1 — What landed (the dispatch ladder, as-executed)

- **d1 lane-catalog-kill** (two builders: map-and-rule → fresh executor): legacy
  string-slug `Diagnostic` KILLED workspace-wide (~124 refs / 21 files); `DiagCode`
  47 variants (32 minted, typed payloads); the ONE committed catalog
  (`core/src/catalog.rs`, const-table, compiler-is-the-parser, no build.rs/macros)
  with structured metadata fields + gate tests; `Carrier` re-carries structured
  `Diag`; slug continuity except `rul-drop-dq-prefix` (5 codes, né dq-*); the `sm `
  prose-wave; `e2e/conduct-bless.sh` (the conductor's one-shot verify+bless);
  the flagship XFAIL substrate.
- **d2 lane-evidence-plane** (proposal → core → continuation): sealed
  `core::evidence` (`CollapseEvidence`/`CollapseKind` all nine classes,
  `TrustTier` ×6, `Operands` k-capped, Eq-excluded-at-carrier, compile-fail seal on
  the `core::room` pattern); `OriginKind::{OracleClaim, ProbeResult(ProbeStamp)}`
  real; minting at every collapse class (C3 kernel post-pass, C4 edge
  measured-grade, C5 decline/wall/substitution/entry/demotion) threaded to the
  why-lens seam. Fact-merge honesty line: precedent-application, no invention —
  the human's Fable-apply-then-rewind offer went UNSPENT.
- **d3 lane-whylog**: the thin durable (`dorc-whylog/1`, line-framed on the
  records discipline, raw as-received stream, spike-tier `--whylog-dir` opt-in
  DISCLOSED in-code vs the product's zero-setup posture, N=5/1MB retention);
  `dorc why --last` replaying through the same kernel; version/desync/absent/
  corrupt refusals (4 codes); report-lane ingestion (noise-tolerant, sanitize+cap,
  dedup); C6 origins-on-the-record + witness attachment; runtime `EntryFailure`.
  apply-report lines are PREDICTED-marked (no spike executor; the reader never
  renders a prediction as an outcome).
- **span dispatch**: precise decline-arm spans; `OracleFileId` on every
  cross-file span carrier; `file:line` on guard + survival attributions; tier-1
  static sink recognition (`Command::report_sink`, engine-owned versioned name
  list) + tier-2 per-site classing + `authored_reason` population; **the
  emission-vouches license bug caught and fixed** (§3); multi-line/caret work
  re-scoped after an honest survey.
- **d4a lane-chain-render**: the arrangement walker (`tier_word` = the ONE
  epistemics-rendering seat; structured `ChainRender`); THE FLAGSHIP GREEN —
  `survivebite27-naked-trust-chain` renders the six-link chain
  (measured/vouched/ran/claimed/derived/consented) with both loci + the
  naked-trust epilogue (derived structurally from evidence-kind presence, never
  instance-guessed) + recovery moves, LIVE and REPLAYED (harness gate-8 pair).
  The subtler lie: cross-kind at-most omission — passes the own-coordinate canary
  BY CONSTRUCTION (the omitted cell is another kind's; the engine never supplies
  it) = the undetectable class USER_STORY's bought-unsoundness ledger names. The
  case asserts NARRATION-completeness, never world-breakage (DST cannot observe
  the counterfactual; that is the field trial's). Full chain = PULL-ONLY
  (`rul-chain-is-pull-only`: never pushed to plan stderr, even spike-tuned);
  survived elisions surface in unargumented `dorc why` as a TRUST-SPENDS section
  distinct from problems. `VerdictDecline.site`; disturbs-arm span threading (the
  leverage-point locus); carry render file:line.
- **d4b lane-catalog-era**: lint absorption (one `core::Severity` vocabulary,
  wire tokens kept, render-layer Finding fold, format-name ownership);
  `dorc_oracle::validate` book-free factoring → rung-oracle-solo UNLOCKED (+
  tier-1 decline-inventory lint source); `RemediationClass` HOW-re-cut
  (ResolveDynamism/DeclareIdentity/ProvideModel/Structural) as a registry column,
  all 51 classified; Suggestion RE-PARKED (seam: `missing-dialect-marker` first
  emitter when fix-apply unparks); defining cases — 17 covered (two-half regime:
  e2e trigger + unit triple-golden) + `DEFINING_CASE_RATCHET` = 35 shrink-only
  with per-entry injection-surface notes; the promote tool
  (`promote_is_a_prose_fixpoint` gate = prose-untouchability STRUCTURAL;
  orchestrator-only, never run by builders); `aid-unloaded-sibling-oracle`
  (found+fixed a Windows `\`/`/` path bug); `expected-hint` pinning (gate-hint);
  the tier-3 report-drain (per-site scratch, PIPE_BUF-atomic re-frame, full-corpus
  byte-stability HELD) + `decline27-tier3-dynamic`.
- **caret dispatch**: 7 spanless codes plumbed (allowlist 25→18; resolver/reaches
  bucketed per oracle file — `report_by_oracle_file`); the sub-line tighten survey
  (COMMITTED: `core/tests/span_precision_survey.rs`) found the cheap surface
  EMPTY — codes already anchor honest; multi-line caret frames (per-line gutter +
  underline; single-line byte-identical). Honest deferrals: `cmdsub-operand-top`
  operand tightening (analysis-kernel re-threading = scope), `dangling-reference`
  (needs a coord→origin back-map; pointing at the resolver would misattribute).
- **docs lane**: spike/docs + spike/skills/author-oracle re-synthesized for the
  aid machinery (decline classes, lint-in-the-hot-loop, the why surfaces as the
  author's feedback loop) — see its commits.
- **Conductor prose checkpoints** ×2: the 4 whylog codes + the sibling-oracle
  nudge — 5 slugs on the `CONDUCTOR_AUTHORED` roster; all other prose remains
  `sm `-marked builder text awaiting the human's slow rewrite; zero `[unwritten:]`
  at close.

## §2 — Incidents (all recovered; root causes)

1. **d1 builder committed to `ai/main` from the primary checkout** (its cds
   escaped the worktree); the human's `3d933b5` interleaved; recovered by
   cherry-pick (byte-identical, verified); human cleaned `ai/main` and ruled
   ai/main-is-the-danger-accepted-playground, no hook-narrowing wanted.
2. **d2 builder grounded its whole phase-1 proposal on the primary checkout via
   absolute READS** (step-zero switch was correct; file access was not). Caught
   at the checkpoint because its claims exactly matched the pre-d1 tree; verified
   before accusing. ONE root cause for both: absolute paths to the primary
   checkout. The brief rule that ended it: *every Read/Grep/Edit/cite lives under
   your own worktree; the primary checkout is radioactive for ANY access,
   read-only included* (§5 candidate law).
3. **SyncThing (PHNHRER) created `*.sync-conflict-*` files inside THREE
   worktrees mid-build** (d2-continuation's, d3's, the conductor's own e2e
   cases). The known `.claude/worktrees` exclusion-repair leftover is
   demonstrably ACTIVE, benign so far (untracked, uncompiled, gates unaffected).
   Cleanup + `.stignore` fix are human-owned; dispatches continued under the
   previously-accepted risk.
4. **Two builders stalled ending their turn awaiting their own backgrounded
   verification** — the notification model wakes the conductor, not them. Brief
   rule that ended it: final verification runs FOREGROUND.
5. **Conductor's own bless-script rc-capture double-bug**: `if ! cmd; then
   _rc=$?` captures the negation; the `if cmd; then return; fi; _rc=$?` "fix"
   reads the if-statement's 0. The only correct set-e-safe capture is
   `_rc=0; cmd || _rc=$?`. Both found LIVE on failing runs (the failure printed
   `exit 0`). Filed under sh-humility for the sh-analysis project's conductor.
6. **Transient `0xc0000142` linker storm** (parallel test-binary links,
   STATUS_DLL_INIT_FAILED) — environmental, cleared on retry; one retry then
   escalate is the right posture.

## §3 — Design findings with teeth (beyond process)

- **`finding-emission-would-vouch`** (span dispatch phase-1 review): a `27W`
  decline-emission `printf` with no trailing `return 2` exits 0 and VOUCHED —
  an aid-plane emission minting an elision license. Fixed: recognized
  sink-emissions join the tracer's inert set (never the vouching command);
  emission-only bodies now decline ⇒ run; test-pinned both ways. License-plane
  fix in the safe direction; the two-phase checkpoint caught it before it
  shipped.
- **`finding-flagship-lie-is-narrative`**: in DST the flagship can only assert
  the RENDER discloses claim-not-measurement + names the naked-trust link; the
  world-breakage half is unobservable (the field trial's). Stated in the case
  itself so nobody later "fixes" it toward an impossible assertion.
- **`finding-canary-blind-crosskind`** (the flagship's construction): the
  own-coordinate footprint canary is structurally blind to cross-kind omissions
  — the fixture IS the demonstration of USER_STORY's undetectable class.
- **`finding-spanless-mostly-honest`** (caret survey): most coarse spans were
  already honest; forcing tighter spans would have been precise-wrong. The
  survey artifact is committed so the conclusion doesn't get re-derived.
- **`finding-corpus-blind-edge-codes`**: no corpus case exercises the plumbed
  resolver/reaches/footprint give-up codes (verified only by out-of-corpus
  smoke); the defining-case ratchet's shrink is the coverage path.
- Named nits for the human: `skip-unresolvable` render vocabulary (round-20
  vintage, 5 goldens) vs the skip-ban's UI carve — rename is a cheap re-bless,
  prose-adjacent, their call; the door1 first-wall hint is suppressed when the
  probe-fold elides (fold-sensitive, ~SUSPECT pre-existing); the why-lens
  remediation-hint prose lives in `diag.rs` outside the catalog (pre-existing
  one-catalog violation; wants the prose-register schema design).

## §4 — The conductor-token protocol: verdict (the human's explicit experiment)

The design held. The prose flow performed exactly as specified on both live
exercises: one Read of the catalog (builder metadata was sufficient alone, both
times), N Edits, one verification command; zero compilation-churn or
test-spelunking on the conductor side; checkpoint #2 was three tool calls
end-to-end. The catalog-as-committed-Rust-const-table + structured metadata +
the three-state prose gate (`sm ` / `[unwritten:]` / `CONDUCTOR_AUTHORED`
roster) made the authorship boundary mechanical; d4b's promote fixpoint-gate
made prose-untouchability structural.

Where the churn actually went (the honest half): the VERIFICATION tooling, not
the prose flow — the rc-capture double-bug (§2.5), the discovery that BLESS
cannot rewrite hand-declared expectations (`expected-diagnostics` substrings,
lint `expected.out`s — six hand-edits, one time), a cwd slip, and a rustfmt trip
(refinement adopted: the conductor path RUNS `cargo fmt` before checking — raw
string prose edits predictably rewrap). All four are tool defects the first real
use flushed out; none recurred after their fixes. Secondary refinement: the
comment-budget counting command over-bills structural banners in data-table
diffs and re-counts moved comments in block rewrites; briefs now note banners
separately (and a net-new-functionality dispatch legitimately runs hotter than a
churn dispatch).

Dispatch-shape verdicts: the two-phase (proposal → go) checkpoint caught one
license bug (§3) and one wrong-tree grounding (§2.2) before either cost a
sweep — it pays for itself in kind, not just process. The map-and-rule →
fresh-executor split rescued d1 cleanly and is the standing pattern for any
big-bang. Serial dispatches + the human-reserved fold made lane branches
conflict-free all phase; the conductor stack (`ai/r27-aid` re-pointed at each
verified tip) folds with one human command.

## §5 — Candidate `spike/CLAUDE.md` law bullets (proposed at close; human deletes freely)

- **worktree-file-access-law** — worktree agents: every Read/Grep/Edit/cite
  lives under the agent's own worktree; the primary checkout is radioactive for
  ANY access, read-only included (two incidents, one root cause).
- **map-then-execute-split** — a big-bang dispatch splits map-and-rule (proposal,
  rulings, mechanical spec) from execution (fresh budget, no re-derivation);
  the checkpoint between them is where conductor review is cheap.
- **error-prose-conductor-flow** — the three-state catalog prose protocol
  (`sm ` migrated-verbatim builder text / `[unwritten: <slug>]` / roster-listed
  conductor-or-human prose); builders author ZERO user-facing strings; the
  conductor authors from catalog metadata alone (one Read, N Edits, one
  `conduct-bless`); the roster is the two-place-lie defense.
- **foreground-final-verification** — a builder's last verification runs
  foreground; ending a turn awaiting your own background task strands the lane.
- **conduct-bless-is-the-verify-entrypoint** — conductors verify landings via
  `sh e2e/conduct-bless.sh` (BLESS mode conductor-exclusive; `DRY=1` is the
  builder smoke); the tally line is run-backed (build · unit · e2e · gates).

## §6 — LSP/MCP duck dispositions (design-seams; from the 2026-07-19 sitting)

lsp-is-the-lint-lane-plus-a-loop (rung-file/book ≙ publishDiagnostics; the
catalog maps ~1:1 onto LSP wire types; AID-NEEDS's push/pull ≙ LSP's) ·
lsp-reactivity-is-the-r26-engine (one re-entrancy obligation, two hats; hygiene
adopted: bytes-not-paths reserve, generation-stamped outputs) ·
lsp-fatigue-inverts-the-kwarn-setting (an editor surface ships
precise-or-silent-leaning selection; legal under the weld's late-cheap-knobs) ·
probing-LSP re-scoped by the human to a probing editor-EXTENSION (consent via
extension config over the same LSP diagnostic lanes); cron ≡ ambient-probing as
ONE design class (timeouts, cost-classes, fragile-host opt-outs,
workspace-scoped consent, explicit-gesture gating); the half-typed-argv danger
UPGRADED by the human (half-typed argv reaching vouched bodies stretches
vouches past author-anticipated shapes — possibly fatal to ambient probing;
tabled whole) · MCP punted but recorded as the cheaper sibling (CI-mode posture
= the contract; lint/why/explain tools = the engine-in-the-authoring-loop
leverage; a container-target MCP probe tool sidesteps the standing-consent
hazard and serves the containerized-TDD dream) · simple no-probe LSP left
IN-PLAY as a spike candidate. Perf caveat (human): the codebase is +SURE slower
than it could be by deliberate priority; keystroke-gating is where that debt
would surface; punt stands until measured.

## §7 — Human close-out queue (asks; nothing here is self-ratified)

1. The FOLD: `ai/r27-aid` carries the whole phase; one ff/rebase pass, yours.
2. Floors ratification (prepared per-code in d4b's registry work; deltas
   proposed: records-refusal codes → WarnOrDeny; wrapper-incoherent pair →
   first real `Floor::Pinned`; the rest keep current).
3. The defining-case RATCHET (17/52 covered, shrink-only) tempers the letter of
   `law-one-defining-case-per-code` at spike tier — ack or direct full coverage
   (the records-* corruption tail is where the ~30h estimate lives).
4. `27W` §0's soft-acks want typed acks: the class starter-set
   {unsound, unmodeled, interactive, hazard} and the advise-verb deferral.
5. The `--risk-faultless-skips` flag rename re-ask (code says
   `--trust-footprints`; ack-3 said keep; gap-9 ruled re-ask at close).
6. C8 (operand-VALUE display in disagreement evidence) parked as a seam —
   veto if you want it built.
7. The prose-register schema (terse/deep/first-encounter) + migrating the
   class-level remediation-hint prose into a catalog home — wants a human/
   conductor design sitting (pairs with your slow `sm `-rewrite pass).
8. The `sm `-prose rewrite pass is yours, eventually; `grep '"sm '` in
   catalog.rs enumerates it; the 5 roster slugs are done.
9. Nits: `skip-unresolvable` render token; the door1 fold-sensitive hint;
   SyncThing `.stignore` + conflict-file cleanup (third worktree affected).
10. AID-NEEDS surface entries for LSP/MCP (future) mirroring "TUI (future)" —
    added this close-out; delete if unwanted.
