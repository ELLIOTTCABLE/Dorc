# 28H — W4 conduct ledger (parts-at-birth · carrier-to-edge · span coverage → a prose-ready why surface)

AI-authored (Fable implementation-conductor, seated 2026-07-26). Executes `plans/28G`
Phase W4 under the `28E` design record; the goal restated by the human at seat: push
through to the position where THE HUMAN can begin authoring prose (the `TODO-ADDTL`
W5⇄W4 interlock resolves W4-first; W5's executor is the human at the loom surface).
Authority: root docs, `spike/CLAUDE.md`, human-typed rulings outrank. Builders write NO
landing notes (27U/28A/28F precedent); as-built detail lives in granular commits + this
ledger. Conductor residency: `.claude/worktrees/r28-unify`, mainline `ai/r28-unify`
(base tip at seat: `d3c05a55`, human-fast-forwarded past the 28F arc close). `ai/main`
is the human's; untouched.

## §0 — Session directives (human-typed 2026-07-26, this seat; rewind-durable)

- Proceed with W4. The deliverable is the human positioned to author prose.
- Conductor reads/understands/reasons/plans/conducts — no churn, no code. CLI use
  ≈ `mise run bless` after final spot-check, plus minimal conduct git (ledger
  commits, lane folds).
- Implementors run in their own new worktrees and manage their own git-state
  faithfully; the conductor does not churn git on their behalf.
- Taste-bound arc: surface specific explore-commands to the human as work lands, for
  flavour/taste opining; do NOT assume interactive response (AFK at points); reach
  for the human as a resource where genuinely valuable, act autonomously otherwise.
- `plans/288` and `plans/286` remain live inputs, bits possibly superseded; updating
  `plans/` as appropriate is in-remit for this conductor.

## §1 — Lane plan (conductor's cut; map-then-execute per `28G` §3)

- **lane-w4-map** (Opus, read-only, own worktree, branch `ai/r28-w4-map` off
  `d3c05a55`) — the map half: parts-at-birth producer/consumer inventory · the
  print-in-place site census · the span-coverage path · THE MEASUREMENT (every
  why-surface string classed: editable-today / editable-with-spans /
  transport-blocked / computed-placeholder / foreign) · kTASTE type room · the
  execute-lane cut. Lands `_w4-map-DRAFT.md`; STOPS at checkpoint.
- **lane-prose-onramp** (parallel, scratch worktree `ai/r28-prose-onramp`) — what is
  authorable TODAY: worklist counts, an end-to-end smoke of the loom edit flow, the
  human's quickstart, the waits-on-W4 list. Report-only; nothing folds.
- Execute lanes cut after the map adjudication (§2 will accrete).

## §2 — Rulings / landings (accretes)

- (seat) Ledger ID 28H claimed (28G taken by the plan + strawmen; 289/290/291 taken).
  W4 accretes on `ai/r28-unify` — the human fast-forwarded this worktree for the
  purpose; one eventual fold, theirs.
- (seat) **ack-prose-pass-executor RESOLVED** — `288:rul-prose-pass-is-fable-this-
  arc` is superseded: prose authorship happens at the loom surface under the
  AGENTS.md looms-sacrosanct law, and authoring is underway in parallel with the
  W4 build. `plans/288` §0/§8 update accordingly at the plans refresh; W4's job is
  unchanged (give the why-surface rows transcript faces).
- (seat) **conduct mechanics: the human edits on `ai/main`**, atomically forwarding
  into `ai/r28-unify` only when it is quiet — the mainline tip may move under the
  conductor; the working-dir will not. Conductor commits stay explicit-pathspec;
  execute-lane folds re-verify the tip at fold time.
- **finding-stale-executor-in-safety-block** (human-caught, 2026-07-26) — the
  propagate-verbatim Safety block in `spike/CLAUDE.md` still named the raw
  `cargo test -p dorc-cli --test e2e` executor after the mise refresh; conductor
  briefs inherited it verbatim. Respelled onto `mise run test:e2e` / `mise run
  test` (+ the `aid/CLAUDE.md` DORC_LOOM_DUMP flow onto `mise run test:looms`).
  The BLESS/WSL bullets keep their documentary raw spellings (each names its
  mise wrap). In-flight lanes: the human redirected lane-w4-map directly; the
  conductor messaged lane-prose-onramp the corrected line.
- **finding-commit-trailer-slip** (human-caught, same sitting) — the conductor's
  seat commit carried harness-injected `Co-Authored-By`/`Claude-Session` trailers
  (forbidden; `.gitlabels` headliner is the only authorship spelling) AND the
  mechanical commit-msg gate did not refuse it. Reverted + re-issued clean by
  human direction; the why-didn't-the-hook-fire investigation rides
  lane-prose-onramp as its Task 3. Standing clamp: every brief now carries an
  explicit no-trailer line (subagent harnesses inject the same instruction).
- (seat) **watch: the 288 §7b one-span fence is the W4 crux** — value-bearing lines
  render as ONE span; multi-word value-interleaved entries are render-only under the
  current transport. The why-chain speaker rows are exactly that shape. The map lane
  measures how much of the why surface this fences off; if large, the deferred
  transport enrichment (`28G` §2) gets priced INTO this arc (per AGENTS.md's new
  anti-deferral law + `289:steer-errorloom-best-to-use`: errorloom is unpublished,
  fix the word-model in place, no adapters).
