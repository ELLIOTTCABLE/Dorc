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
- **lane-w4-map LANDED** @ `fc56e973` on `ai/r28-w4-map` (read-only; stopped at
  checkpoint as briefed; `_w4-map-DRAFT.md`, 645 lines, conductor-read whole).
  Headline, counted in-tree: class (a) editable-today = 0 of 111 `why-*` rows;
  class (c) transport-blocked = 45 of 56 sentence-bearing rows (80%), 61% of
  reached prose chars — the watch item CONFIRMED, the deferred transport work
  stands directly between the human and W5. Secondary finds: the span map has no
  production consumer (`print_document` drops it); 26 print sites counted;
  `advisory: bool` is already kernel-clean (12 cli signatures only); the carrier
  lane is provably zero-re-bless (stderr is needle-pinned, never byte-goldened).
- **Conductor adjudication of the map's ASK list** (rulings; the map's §G carries
  the full arguments):
  1. `ask-pull-transport-into-this-arc` → **PULL IN** (as lane-w4-span leg A).
     Grounds: the 80%/61% measurement + the human's push-to-prose directive +
     AGENTS.md anti-deferral + `289:steer-errorloom-best-to-use`. Flagged to the
     human for cheap veto; conducting on it meanwhile.
  2. `ask-word-model-fix-lives-dorcside` → **ADOPT, proof-first**: build the
     dorc-side one-section-many-fragments shape (`RenderPart::ArrangementValue`;
     accumulate per `(slug, occurrence)`; `apply_arrangement_edit` re-splits on
     the compiled fragment series); PROVE it at the leg-A checkpoint on three
     unit-tier cases — single-run, multi-run, and a GLUED boundary (value abutting
     a word, no whitespace — the `28A:rul-glued-param-rehole-seam` shape, the one
     place ~SUSPECT errorloom-needs-no-change could break); touch errorloom only
     if a refusal class demands it, fix-in-place, no adapters. Supersedes
     `28E:prop-span-boundary-tokenization` (conductor proposal, never human-typed)
     as the first shape tried.
  3. `ask-lift-one-span-per-chrome-line` → **AMEND THE LAW** to "one editable
     SECTION per chrome line; the section may hold interleaved value fragments;
     nothing ever splits one chrome line across sections" — preserves the
     2026-07-24 lesson (many-sections was the breakage). Amendment lands with
     leg A: `aid/CLAUDE.md` rows + `plans/288` §7b closing text (conductor edit).
  4. `ask-hoist-said-into-aid` → **YES** (lane-w4-parts). `Said::Lens` deletion is
     the phase's point; `Said::Foreign` encodes at mint.
  5. `ask-all-flag-promises-exhaustive` → **the kTASTE residue model discharges
     it**: `--all` renders `links` unfiltered. Slightly exceeds types-only; a
     filter-read, not register machinery; kills a false printed promise without
     prose churn.
  6. `ask-emission-order-may-move` (tc, flagged up) → **preserve flush points,
     byte-identical stderr this arc**; re-ordering banked with `289` §2v's
     multi-host concurrency note.
  7. `ask-arrangement-normalization-fork` → **normalize only bridge-minted
     sections**; the arrangement-page path stays verbatim.
  8. `ask-why-case-collection-placement` (tc, flagged up) → **ADOPT a new
     in-process-driven why case shape in `crates/aid/tests/`** (registered-aid
     placement per `288:rul-slug-decides-loom-placement`, read broadly as its own
     text directs); whygallery whole-product cases retained unchanged as executed
     evidence. World-shape mechanics = builder latitude; NB `28F`'s banked
     `run: replay-only` case-shape want is probably this same need — converge on
     one shape, don't mint two.
  9. `ask-register-key-axis-reserved` → **ADOPT**: occurrence never carries
     register; the third-axis note goes in `aid/CLAUDE.md`; no machinery.
  10. `ask-chain-link-order-is-a-render-default` → **ADOPT**: one-function seat +
      doc-comment naming it a RENDER DEFAULT; no ordering machinery.
  11. `ask-why-lens-stderr-unencoded` → **fix in lane-w4-parts** via
      `Said::Foreign` (encode-at-mint), as law-compliance.
  Lane cut **ADOPTED as proposed**: three lanes, SERIAL (carrier → parts →
  span-with-mid-checkpoint); the parallel variant declined (one cross-merge not
  worth it, no time pressure). kTASTE `ChainModel`/`LinkSelection` shape adopted;
  one latitude note: `conclusion` may be a parts-stream rather than a single
  `Said` if the welded synthesis wants it. Zero-churn expectations + empty-diff
  proofs adopted as per-lane gates. Platform exposure none (confirmed); the
  `mise run both` leg stays owed at folds as cheap insurance. Rider for the span
  lane: when adding the `dorc why <addr>` arm to BOTH replay-arm chains
  (`replay` + `render_direct_replay`), add a cheap mechanical agreement guard if
  one falls out naturally (`fc56e973`'s §H item 3 — a recorded divergence class).
- **finding-map-builder-hook-bypass** (self-disclosed, self-corrected) — the map
  builder's FIRST commit passed `-c core.hooksPath=/dev/null` (effectively
  `--no-verify`, forbidden); it reset and re-committed with hooks live; the
  landed tip is clean. Standing brief line from here on: "if a hook refuses your
  commit, fix the message/content — NEVER bypass (no `--no-verify`, no
  `core.hooksPath` tricks, no env unset)". Datum for the Task-3 hook
  investigation: something DID fire in that agent worktree.
- **Map FOLDED** @ merge `86522ca2` (`_w4-map-DRAFT.md` on the mainline; the lane
  branch `ai/r28-w4-map` retained as its history).
- **lane-prose-onramp LANDED** (report-only; its worktree reset clean; scratch
  branch deleted by conductor — zero commits). Worklist: 209 prose rows = 47
  transcript-editable today / 162 lock-only; all 7 `[unwritten:]` catalog codes
  HAVE looms (the best first sitting); ALL arrangement rows are faceless except
  `cli-help-page` (`dorc-loom`'s corpus is hardcoded to `crates/aid/tests` —
  confirms the map's gap from the tooling side). Flow VERIFIED green end-to-end
  on the catalog-loom path (bare `mise run loom:compile` / `loom:promote`; no
  case list, no `--shell=` needed for the aid corpus — two banked frictions
  already fixed); the lock hand-seed path works and passes the byte-identity
  fixpoint but ends at an orchestrator-only bless (unverified by the lane, by
  design); why-transcripts correctly REFUSE edit authority
  (`282:rul-replay-editability-is-provenance` observed working). The L6
  arity-loud fix confirmed live (a deliberate one-word seed for a one-value row
  panicked `dorc why` in debug — the fix working, not a regression; release
  degrades to `[unwritten:]`). Six frictions banked in its report; sharpest:
  promote's word-diff preview is swallowed by `MISE_TASK_OUTPUT=timed`; a
  transcript edit appends a trailing `\n` to the stored register (the only such
  row in the lock — wants a trim-or-not ruling before the human's rows diverge
  in shape); no `loom:compile:verbose` task; `scaffold` has no mise task.
- **finding-hk-master-killswitch** (the Task-3 answer; human-owned fix) —
  `HK=0` is live in the agent-session environment, inherited from whatever
  launches the sessions (NOT repo `.claude/settings.json`, NOT mise config, NOT
  Windows user/machine env). The installed hook command is
  `test "${HK:-1}" = "0" || mise x -- hk run <hook> --from-hook`, so `HK=0`
  short-circuits EVERY hk hook — commit-msg included — in every agent shell.
  Installation is fine; the matcher is fine (refuses a trailer when invoked
  directly); `mise run test:hooks` is structurally BLIND to the gap (it spawns
  the matcher script directly and never exercises the git→hk wiring — green
  while the gate was dead). Explains finding-commit-trailer-slip completely, and
  moots finding-map-builder-hook-bypass (hooks were dead in that worktree too).
  Fixes: (1) HUMAN-OWNED — scrub `HK=0` from the session-launching environment;
  (2) repo-side PROPOSAL, wants a ruling — extend hook-selftest to assert
  WIRING reachability (`git hook list commit-msg` non-empty AND env not
  neutering), so a dead gate is red not green; (3) the human should check their
  own interactive shell (`echo $HK`) — live human commits may be bypassing too.
  RESOLVED (1): post-harness-restart probe 2026-07-26 — `HK` unset,
  `HK_SKIP_HOOK=pre-commit` intact; `git hook run commit-msg` REFUSES a
  trailer-carrying message (rc=1) and passes a lawful one (rc=0). The gate is
  live for conductor and builders alike. (2) remains open — the selftest is
  still wiring-blind; the gate died silently once.
- **Human items pulled into scope (typed 2026-07-26; weight-tags as given):**
  - `item-ran-log-retirement` (gentle lean, LOW priority, builder-driven —
    "don't churn over polishing right this second"): the `_dorc_logged` /
    `expected.ran` mock-run-log machinery should eventually die or fold into
    dorc-loom as a Rust-powered feature; minimize long-term loom noise; do NOT
    overengineer a testing-feature suite. Builder read-first:
    `.claude/research/loom-harness-alternatives/` (the 289-banked
    mock-stub-crate candidate) + this ledger's replay-only case-shape note.
    Shape: optional tail lane, investigation-first, skippable under pressure.
  - `item-probe-results-fixture-legibility` (product surface; conductor answered
    in-chat) — probe-results.txt is the probe→controller RETURN CHANNEL as a
    case fixture: bare `site <leafid> effect=… rc=…` body-lines (262 §2), whose
    VERSION/attribution lives in the `dorc-records/1` framing header + terminal
    token that the loom fixtures omit (the unframed harness lane;
    `SiteResults.framed` tracks which lane admitted). Banked ask:
    `ask-fixtures-exercise-framed-lane` — should case fixtures carry the frame,
    so replays exercise the strict bounded-admission path production uses?
  - `item-loom-shell-niceties` (nit, ONLY-if-trivially-cheap): round-trip a
    replay's `$ dorc …` argv through the shell-executor purely to print
    EVALUATED argv; the in-process catcher consumes that — looms become 1:1
    with shell semantics without double-invoking dorc. Span-lane builder
    assesses while inside the replay driver; declining is fine.
  - `item-missing-command-word` (bug, investigate in passing):
    `whygallery-webhost-whole` renders "on 's at-most claim" — an empty
    speaker/name before the possessive. Parts-lane rider.
  - `item-simpler-why-invocations` (LEANS, human-tagged "don't encode too
    strongly"): the loom prose says `dorc why book.sh:14` while the replay
    lines show flag-heavy harness invocations — telling. Floated thoughts,
    banked as leans only: oracles intelligently recoverable from the receipt's
    hashes/filenames; books as positionals not flag-with-payload;
    probe-results on stdin (NB stdin IS already the default outside `why`; W3
    respelled why's replays to explicit `--results`). Execute-phase design
    item; converges with `ask-why-case-collection-placement` — the new why case
    shape should drive the receipt-first path so transcripts SHOW the simple
    form.
- **Lane work-order deltas from the above**: parts lane + item-missing-command-
  word; span lane + item-loom-shell-niceties (assess-only) + the replay-arm
  agreement guard; optional tail lane-ran-retirement appended after span.
- **HOLD (human-directed)** — harness restart pending. Both lanes landed and
  banked; execute lanes NOT dispatched; resume post-restart from this ledger +
  `_w4-map-DRAFT.md` (§F lane cut as adjudicated above).
- **lane-w4-carrier LANDED + FOLDED** @ merge `062c4516` (5 commits, tip
  `f302e230`, builder-rebased onto `8281432d`; one file, 221+/214−). 26 sites
  confirmed → 24 post (the three whylog-loader seats collapse to one fold
  point); `advisory: bool` retreats from NINE signatures (map said 8 — builder
  recount stands) and survives at exactly `report_at` / `report_by_oracle_file`
  / `advisory_filter`; libtest red-frames dead by construction + PINNED
  (`resolver-conflict` asserted as a returned value); zero re-bless PROVEN
  (empty diff over `crates/*/tests`); comment budget net-new ≈2 of 10; both
  platform gates green at the builder's tip (1465 Windows / 1461 WSL, the cfg
  split). stderr byte-identity rests on a STRUCTURAL argument + the needle/gate
  net, not a captured byte-diff — accepted knowingly (the preserve ruling was
  conservative; stderr shape is render-form-unwelded territory anyway).
  Conductor fold-verify: own-hand `gate:full-quiet` on the merged tip (the
  arbiter for some post-report rust-analyzer E0308 noise in the agent worktree
  — `{unknown}`-typed slice-coercion complaints, ~SUSPECT r-a false positives
  given two green clippy legs).
- **Conductor rulings on the carrier flags:**
  `ask-diags-only-helpers-are-vec-not-carrier` → **`Vec<Diag>` ACCEPTED** for
  diags-only products (this file's own idiom; `Carrier<()>` is ceremony) ·
  `ask-kindlift-is-a-second-spine-type` → **ACCEPTED as-built** (two diag groups
  genuinely frame against different sources; unify-into-one-spine is noted as a
  parts-lane OPTION, never a mandate) · `ask-touches-set-is-lifted-twice` →
  **preserved-deliberately ACCEPTED**; banked as a cheap follow-on for whoever
  next owns that region (unifying moves where diags print — correctly out of a
  byte-preserving lane) · NEW bounded RIDER for the parts lane:
  `rider-carrier-must-use-on-carrier` — add `#[must_use]` to `Carrier<T>` in
  `aid` IF the `-D warnings` fallout is a handful of sites; else revert to a
  note and report (the silent-drop hazard the builder named is real and
  type-shaped). — value-bearing lines
  render as ONE span; multi-word value-interleaved entries are render-only under the
  current transport. The why-chain speaker rows are exactly that shape. The map lane
  measures how much of the why surface this fences off; if large, the deferred
  transport enrichment (`28G` §2) gets priced INTO this arc (per AGENTS.md's new
  anti-deferral law + `289:steer-errorloom-best-to-use`: errorloom is unpublished,
  fix the word-model in place, no adapters).
