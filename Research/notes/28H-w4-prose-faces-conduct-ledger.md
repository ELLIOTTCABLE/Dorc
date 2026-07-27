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
- (added mid-arc, human-typed) **product-vs-internal-carve** for ALL loom-infra
  work: errorloom is a product too — per change, ask "genuine quality-of-life
  for any errorloom user, or Dorc-specific bloat?"; Dorc-specifics live in
  dorc-loom/the consumer. Propagates into every loom-touching brief. (Full text
  + the webhost red-line distillation: `notes/28I`.)

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
  type-shaped).
- **lane-w4-parts LANDED + FOLDED** @ merge `2d3eec1d` (8 commits, tip
  `0c4db94c`, builder-rebased onto `435f961f`; own-hand gate running at fold).
  All nine items: `Said` hoisted into `aid/said.rs` (`Said::Lens` DELETED);
  `Explanation { parts, remediation }` with the four-part decomposition;
  5 entries / 4 slugs hand-seeded `Words::Migrated` (promote confirmed the seed
  IS the generator's fixpoint, zero writes); both consumer seats own skeletons;
  kTASTE `chain.rs` (`ChainModel`/`LinkSelection`/`Relevance`/`LinkRef`;
  `--all` reads `links` unfiltered — the printed promise is now true); the
  chain-row-order seat + render-default doc; `#[must_use]` on `Carrier<T>`
  (zero real dropped-carrier sites; 23 redundant attributes deleted). Byte-level
  verbatim-migration proof (`a_reason_reads_exactly_as_the_hardcoded_sentence_
  did`, 281 bytes, boundaries included). Gates 1469 Windows / 1465 WSL. Churn:
  exactly ONE test-tree file moved — the sanctioned rider-8 re-bless.
- **Conductor rulings on the parts flags:**
  `Said::Parts` + `Said::Mark` deviations ACCEPTED (Parts spares ~60 site
  reshapes; Mark carries punctuation without minting an editable row —
  layout-is-not-a-word). FENCE noted for successors: **`Said::Mark` must never
  carry English words** — glyphs/punctuation only, or it becomes a dodge of
  the every-string-is-a-row law. `conclusion: Option<Said>` ACCEPTED
  (plain_chain genuinely reaches no restatement). 5-rows-not-3 ACCEPTED (the
  closer row is arithmetically forced; `why-reason-cmdsub-locus-absent` was a
  third §0 violation the map missed).
  **rider-8 (`item-missing-command-word`) — FIXED AS A TYPE**: `ChainRender.
  trust: Option<TrustSpent>`, minted only by `survival_chain`; the two bogus
  TRUST SPENT items (guard/decline chains with empty claimants, wrong verbs)
  are gone from the webhost transcript (8-line drift, inspected, sanctioned).
  **`ask-aggregate-owes-declines-and-guards-a-section`** → ESCALATED TO THE
  HUMAN as a taste question (conductor read: post-fix quiet-by-default looks
  design-CORRECT — the anti-nag ruling + strawman-e restraint; the per-line
  surface is where declines narrate — but the argless aggregate's owed-a-line
  question is theirs).
  **`ask-because-clause-truncates-at-two-forty`** → banked as a SPAN-LANE
  CHECKPOINT ITEM: the addressed-why ⊤-run reason (281 bytes) renders
  truncated through the `WHY_VALUE_CAP=240` value slot until the transport
  work removes the flattening; span leg B must verify it un-truncated + pin it
  with a case.
  **`friction-bless-task-cannot-scope-a-sanctioned-drift` — FIXED IN LANE,
  ACCEPTED**: `mise run bless -- <substring>` now scopes the BLESS pass, and
  a SCOPED bless inverts to bless-then-gate (a sanctioned drift is red until
  blessed); unscoped unchanged. Dorc-internal tooling; errorloom untouched.
  Conductor doc-edit batch GROWS (owed at the span-leg-A fold, with the §7b
  amendment): aid/CLAUDE.md third-key-axis note (ruling 9) ·
  `a-registry-row-need-not-mint-a-span` under-describes the four-faceless-row
  reason · said/chain module mentions · spike/CLAUDE.md `test:looms` deferral
  caveat (from the probe) · the webhost-collision is now REAL (the human's
  `f4f48316` red-line rebases over the fixed transcript).
  Second datum banked for `item-loom-blast-radius-dirty-gate` (the builder's
  own uncommitted lock seed tripped the dirty gate — fixpoint confirmation is
  forcibly after-the-fact) · `friction-hand-seeded-row-has-no-reach-gate`
  banked (nothing asserts a seat reaches a hand-seeded slug until a transcript
  covers it; the span lane's why-case shape is the closure).
- **rul-selection-is-goal-derived RULED** (human-typed 2026-07-26, in-chat; now
  `AID-NEEDS:law-selection-is-goal-derived`, with `law-pull-runs-wide-open`
  reglossed to match and the `28E` §8 matrix annotated vibe→RULED). The three
  answers given: (1) tracks — one-model/surfaces-as-consent×goal was already
  durable (`28E` §4, AID-NEEDS surfaces block); (2) conductor agrees — the
  static-policy alternative cannot reach "deeply effective" across contexts,
  and the labeled `--all` floor makes curation failures visible-and-recoverable;
  reservations voiced: the derived goal must be inspectable DATA (a new
  mis-curation surface otherwise), and the wants-relation stays code-shape, not
  a catalog; (3) encoding was PARTIAL — the matrix was vibe-tier, the curation
  dichotomy implicit-only, the goal-driven backwards construction absent; now
  fixed durable. `ask-aggregate-owes-declines-and-guards-a-section` ANSWERED
  within it: decline stays quiet at default WHEN other content exists; the
  calculus is receipt-dynamic (an otherwise-quiet receipt may make quiet
  classes the story). Architecture consequence banked for W5/r30 selection
  work + the aid/CLAUDE.md doc batch: selection code derives a typed
  inspectable goal first, selects backwards from it, and can narrate it; the
  kTASTE `Relevance` room grows goal-conditioned variants, never a bool.
- **lane-precommit-loom-honesty LANDED + FOLDED** @ merge `0d0b5a58` (5 commits,
  tip `2c806075`; own-hand gate at fold). The glob hole closed
  (`spike/crates/cli/tests/*.loom` joins the e2e step; warm single-loom hook
  measured 1.40–1.50s, INSIDE the 3s law, cheaper than the dir-case it joins);
  hk step `looms` → `loom-hygiene` (three references, all in hk.pkl); the
  no-match floor is a three-way triage (`Runs`/`NoTrial`/`Unknown`) with both
  directions pinned in preflight; `step_globs` battery asserts
  seen/seen/unseen for the three representative paths (the cost-scoping
  decision pinned, not just coverage); `mise run both test:hooks` + `both
  gate:full-quiet` green. BONUS: a pre-existing false-failure found + fixed —
  the dir glob already reached weft `golden/` + dorc-loom `fixtures/`
  (`.rs`-fixture space), panicking the pre-commit e2e step on any commit
  touching them; now benign-with-note through the real hook.
- **Conductor rulings on the honesty flags:** `tc-unknown-name-always-fatal`
  ACCEPTED (loud is the safe direction for a harness; the staged-deletion
  measurement licenses it — noted dependency: if hk ever includes staged
  deletions in `{{files}}`, this becomes a false failure) ·
  `tc-benign-means-filesystem-presence` ACCEPTED (discovery-coupling refused;
  the `e2e.rs`-as-filter cost case is exotic and still prints the note) ·
  `tc-pin-is-the-decision-not-the-exit-code` ACCEPTED (process-level pin =
  self-spawn tax; hand-verified transcripts + the real-hook end-to-end proof
  are proportionate) · the `looms`-cargo-target residual naming collision
  ACCEPTED as residual (renaming churns `mise run test:looms` + docs for
  little; the step rename removed the collision that actually hid the hole).
  Frictions banked: `loom-hygiene`'s glob reaches dorc-loom fixtures (whole-
  corpus run on a fixture edit; sub-second, harmless, noted) · WSL env note:
  `wsl -- mise trust --all` where plain `mise trust` fails config parse.
- **rul-selection-is-goal-derived REFINED** (human-typed correction, same
  sitting): the conductor's first regloss of `law-pull-runs-wide-open`
  OVER-CORRECTED — "answer maximally" was always a REGISTER statement (any pull
  answer is generous next to the jealously-meted push surfaces; that contrast
  stands), not a density mandate, and the goal-law never forbade curating
  maximally — it forbids curating WRONG; density is a tuning within the goal.
  Second refinement: spike-era posture — the present concern is THE ARCHITECTURE
  to tune prosody/verbosity (generate more/better/extra/noisy output data; it
  forces the architecture to track it; down-tuning is cheap later, up-tuning is
  very hard — kWARN tune-high generalized), far more than where the tuning sits.
  Both edits applied to the two AID-NEEDS law entries in place.
- (seat) **watch: the 288 §7b one-span fence is the W4 crux** — value-bearing lines
  render as ONE span; multi-word value-interleaved entries are render-only under the
  current transport. The why-chain speaker rows are exactly that shape. The map lane
  measures how much of the why surface this fences off; if large, the deferred
  transport enrichment (`28G` §2) gets priced INTO this arc (per AGENTS.md's new
  anti-deferral law + `289:steer-errorloom-best-to-use`: errorloom is unpublished,
  fix the word-model in place, no adapters). [Confirmed by the map's class-(c)
  measurement; transport work pulled in — see the adjudication above.]
- **Human workflow items (typed 2026-07-26, mid-arc; conductor-checked):**
  - `item-loom-blast-radius-dirty-gate` + `item-input-edit-is-a-structure-bless`
    + `item-errorloom-obvious-flow-pass` — the human edited an INPUT section
    (re-hardwrapped a `-- book.sh --` comment) and `loom:compile` refused on
    dirty-tree. Logic CHECKED, holds with one caveat: the blunt gate exists
    because compile/promote is a render-back that OVERWRITES loom files — the
    real hazard is MIXED same-file edits (uncommitted prose + input edits in
    one loom), which is exactly what `282`'s prose-bless/structure-bless
    EXCLUSIVITY names (the human's recalled term is right). Correct scope:
    dirty-refusal only within the blast radius (touched looms + the two
    generated locks) — repo dirt outside it is ceremony, agreed; an
    input-section edit routes to the STRUCTURE-BLESS path (re-drive the
    replay, re-render, re-anchor surviving prose spans, show the diff) —
    designed in `282`, ~SUSPECT unbuilt/partial as a first-class compile path
    (today's route is the blind `DORC_LOOM_DUMP` two-step); mixed input+prose
    edits in ONE file refuse with the exclusivity message naming both paths.
    Plus the flow-legibility pass: every refusal names the next command; the
    edit/compile/review/promote/commit path obvious from any error. HOME: a
    loom-UX lane AFTER span leg A (leg A reworks `to_editable_render` /
    `apply_arrangement_edit`; building UX atop pre-rework shapes churns
    twice); sized at the span fold.
  - `rul-fixture-records-enriched-not-reduced` (human lean, direction-setting)
    — fixture/loom probe-records ENRICH rather than reduce: carry the full
    framed `dorc-records/1` form verbatim (version statement, DST-stabilized
    clocks/hashes and all), PLUS a very-much-not-on-the-wire debug/fixture
    mode appending a commented correlation form to each `site` line
    (`# 12|apt-get` — command word only, no argv, churn-minimal; the reader
    already tolerates `#` lines by design). Supersedes
    `ask-fixtures-exercise-framed-lane` in the ENRICH direction. New why-cases
    adopt at the span lane; corpus-wide fixture respell is a tail item, not
    mid-arc churn.
  - `item-webhost-redline-orphaned` — the human red-lined
    `whygallery-webhost-whole.loom` @ `f4f48316` (their branch), deliberately
    non-compiling; proper compiling prose edits to follow. COLLISION FLAGGED
    to the human: the parts lane's `item-missing-command-word` rider
    re-blesses that SAME loom (mechanical bug fix); recommendation — lane
    proceeds, the human's prose edits rebase over the fixed transcript.
  - `item-precommit-loom-step-overclaims` — RESOLVED by lane-loom-gate-probe
    (report-only; scratch cleaned): **VERDICT (a), tier confusion — the FULL
    gate goes red on the red-line** (e2e CATCH ×2: the block-0 command pin +
    — luck-only, not creditable — the ascii gate on an echoed em-dash; the
    content-diff/dash-n/needle gates are inferred-caught, short-circuited
    behind the command pin). BUT a real mechanical pre-commit gap: **the hk
    `e2e` step's glob is dir-shaped-only** (`spike/crates/*/tests/*/**`), so a
    single-file whole-product loom NEVER reaches the tier that proves its
    transcript; the `looms` step green-prints all 80 trials (hygiene-only for
    the 27 `run:` looms, indistinguishable from genuinely-fixpointed); hk
    prints NOTHING for a glob-missed step in normal mode. hk.pkl's own comment
    names the escape clause and walks past it. Probe empirically proved: the
    e2e runner ALREADY handles single-file loom paths; the 27 `run:`-bearing
    looms are exactly `crates/cli/tests/*.loom`. **lane-precommit-loom-honesty
    dispatched**: the glob line + the `looms`→`loom-hygiene` rename + honest
    comments + the no-match-floor benign/bug distinction (convention →
    mechanism); the `spike/CLAUDE.md` task-table caveat line is the
    conductor's at fold. Banked frictions: `fixpoint: executed` wants a
    visible `ok (deferred)` marker (libtest-mimic shape fights it — loom-UX
    lane candidate) · gate short-circuiting reports one failure where a
    design-red-line carries five (breadth-vs-first-failure, loom-UX lane) ·
    blocks 1..N replay commands are driven-verbatim, only block 0 is pinned
    against the driven invocation (seam note, defensible as-is).
