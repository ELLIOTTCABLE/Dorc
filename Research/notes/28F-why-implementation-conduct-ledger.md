# 28F — why-surface implementation conduct ledger (W1→W2→W3)

AI-authored (Fable implementation-conductor, seated 2026-07-25). Executes portions of
`plans/28G` under the `28E` design record. Authority: root docs, `spike/CLAUDE.md`,
human-typed rulings outrank. Builders write NO landing notes (27U/28A precedent);
as-built detail lives in granular commits + this ledger. Conductor residency:
`.claude/worktrees/r28-unify`, mainline `ai/r28-unify` (base tip at seat: `0355484b`).
`ai/main` is the human's; untouched.

## §0 — Session directives (human-typed 2026-07-25, this seat; rewind-durable)

- Conserve conductor context brutally; trust Opus builders on execution minutiae.
- Conductor does the final rebase/merge of lane work into `ai/r28-unify`; builders
  manage all other git state in their own worktrees.
- The `28G` phasing is malleable by conductor reasoning ("all the capabilities that
  conductor had").
- NORTH-STAR: `notes/28G-why-strawmen-v2/a-fire-morning.loom` — most heavily
  human-reviewed artifact of the round. Goal is NOT perfect reproduction: it is a
  strawman; get the large-scale machinery into the correct homes; agonize over the
  fundamental machinery behind mismatches, never over formatting-match churn.
- Biggest wants (verbatim-tier): (1) correct rendering in correct places, correctly
  segregated; (2) types tracking the correct info and surfacing it into the correct
  places; (3) missing information collected and typed; (4) a SKELETON of the
  formatting-engine — separate crate/sub-library, substantial firewall: design/printing
  can never drive engine-logic; engine-logic forced to surface well-structured,
  homogeneous, totalistic data to the printer. (This pulls the `28G` §2 render-crate
  deferral partially FORWARD: skeleton now, doc-algebra/sh-formatter still deferred.)
- Style nits: ship the cheap ones; the remit to pass over them applies only when they
  cause churn or block momentum.
- **OPAQUE-BUILDER STAGE (must run after W2 lands, before W3):** dispatch one builder
  whose entire brief is: read and execute
  `Research/quarantine-DO-NOT-READ/29C-w25-opaque-builder-prompt.md` (exists in the
  PRIMARY checkout; existence verified, never opened by conductor). Conductor must not
  open that file nor inspect the resulting commits' subject matter; no review owed. If
  the builder reports STOP: halt all work, notify the human. Otherwise ignore its
  results and proceed (presumably W3).

## §1 — Lane plan (conductor's cut of `28G` §1, amended per §0)

- **lane-w1-voice** (`ai/r28-w1-voice`, off `0355484b`) — 28G Phase W1 whole: the
  honest words in the EXISTING walker/arrangement/catalog machinery. Rider:
  `289:rider-why-last-address-order` if in reach. As-written source-excerpt blocks
  EXCLUDED (W2 owns show-the-code + ForeignText tagging).
- **lane-weft-skeleton** (`ai/r28-weft-skeleton`, off same base; PARALLEL,
  file-disjoint) — the firewalled formatting-engine skeleton crate
  `spike/crates/weft` (name = conductor pick, rename-latitude flagged): generic node
  vocabulary (28E §3 inventory), word-run provenance spans, pure (tree,width)→
  (ASCII, total-cover span map) render, zero deps, no Dorc wiring this lane.
- **lane-w2-narrations** (after both fold) — 28G Phase W2 + re-home the why render
  composition onto weft via an aid-side adapter; ForeignText tagging lands here.
- Then: opaque-builder stage (§0) → W3 sizing (hardening bill; ship only if small,
  else bank for r30).

## §2 — Rulings / landings (accretes)

- (human-typed, mid-W1, delivered by the human DIRECTLY to the weft builder;
  banked here for durability — none critical, all early-architecture steers so
  the skeleton doesn't weld against needed richness):
  **steer-weft-box-model** — a box model, not simple indents.
  **steer-box-model-descends-into-code** — the sh-formatter is NOT a separate
  inline handoff (block in, finished code out); box layout descends INTO code
  blocks — e.g. an end-of-line comment is HIGHLIGHTED by the sh-formatting
  detail but ALIGNED by the parent box model; such elements belong equally to
  the formatter and the page render.
  **steer-cross-box-gutter-alignment** — some alignment is not tree-local:
  gutter columns size dynamically to content and must align BETWEEN separate
  boxes that do not share a parent. (Implies a measure/arrange separation and
  shared alignment-scopes in the model; the W2 adapter + any W4 work must
  honor these, skeleton-tier now.)
- (human-typed, mid-W1) **weft cribs prior-art shapes liberally**: battle-tested
  open-source libraries/formats are the reference for types/interfaces/APIs/
  user-patterns — general SHAPE only, never code; permissive licenses only,
  checked and reported; the builder's no-subagent clamp lifted narrowly for
  read-only scout subagents so licensed source never enters the builder's own
  context. Design still outranks any library's shape (needs-inventory law).

- (seat) Ledger ID 28F claimed (28E/28G taken; 28-lettered series is the human's
  demonstrated preference for this arc).
- **lane-w1-voice LANDED + FOLDED** @ merge `5dde4d30` (6 commits, base `7e1755f6`
  — the human had advanced `ai/r28-unify` by one docs-only commit post-seat; the
  builder verified clean-ancestor and proceeded correctly). Builder gates all
  green (cold clippy; 62 suites; e2e 103/103; looms 26/26; arrangement-lock
  fixpoint holds). Conductor altitude checks: §0-law grep clean (one machine
  token in the probe artifact, sanctioned); own-hand cold verify running at fold.
  ~45 new arrangement rows, all `Words::Migrated` strawman-transcribed;
  faceless-but-storage-homed (edit-home = lock until W4 span coverage), per the
  documented carve.
- **Conductor rulings on the W1 flags:**
  **rul-w1-freeze-net-respell-approved** — retiring the remediation-hint
  byte-freeze (engine `elide` → admin `skip` in the `why-remediation-*` rows) is
  APPROVED: the admin-English carve is human-typed (`28E` §8); a migration
  freeze-net never outranks a ruling that the frozen bytes were wrong.
  **rul-w1-bare-positional-is-address** — in `why` mode a bare positional is an
  ADDRESS; `dorc why book.sh` (book intended) errors loudly. Accepted, pinned.
  **rul-w1-two-rank-partition** — `!` iff covers-unmeasured, `*` otherwise
  (vouches/derives included) — the builder's correction of the brief's
  "runtime-backed" gloss, accepted; matches all five strawmen and
  `rul-danger-axis-is-completion-class`.
  **`--trust-footprints`** stays printed (copy-paste-true pointers); the
  `--risk-faultless-skips` rename stays OWED (standing rider), isolated at
  `CONSENT_FLAG`.
  Kind-prefix display kept (strawmen inconsistent; stripping is lossy).
  `dorc teach walls` pointer correctly omitted (lean unbuilt; copy-paste-truth).
  Rider bonus: TWO silent-wrong-surface why-address bugs fixed + pinned
  (`289:rider-why-last-address-order` + a qualified-address file-part-ignored
  sibling the builder found).
- **gap-bank-for-w2** (from the W1 report; "collected and typed" is a top want):
  (1) predict-defining span — `ProbePredict` carries no source span; the
  minting-line threading covers vouches only. BIGGEST render gap
  (`service.oracle.sh:12 reported` renders as a funcname instead). (2) no clock:
  whylog/`WhylogV2Metadata` are clock-free; OUTCOME timestamp + `(ran HH:MM:SS,
  rc N)` need a DI-seam clock recorded at the cli edge into the durable.
  (3) per-record tool-rc not threaded to the chain builder. (4) receipt-header
  fields absent: host, trigger, book digest, oracle inventory, skipped-count
  split (git-match line stays W3). (5) decline-vs-undescribed indistinguishable
  at the IMPROVEMENTS seat — the anti-nag (strawman c) needs `VerdictDecline`
  narratives there; W1 reworded the nag to engine-backable words meanwhile.
  (6) brace-selector aggregation (one FactKey = one selector today).
  (7) participating-lines block unbuilt (W2 owns). (8) `claims`-row seam left
  for W2's `as-written:` ForeignText excerpt block.
- **lane-weft-skeleton LANDED + FOLDED** @ merge `11c12e9c` (8 commits + lockfile;
  the human steered it directly mid-lane — box model, descend-into-code,
  cross-box alignment, prior-art cribbing via scouts). Crate `spike/crates/weft`:
  zero-dep, one-way firewall stated in crate docs, box-model layout with
  per-line cells reaching into code blocks, total-cover provenance spans,
  goldens at 80/40, 1403 tests green at its tip. The half-built cross-box
  column-sharing was WITHDRAWN in-history (add-then-withdraw, recoverable by
  `git show`) in favour of a stated NAMED-TABLE design: rows join a named
  table; the table resolves all column stops in one prefix-sum pass (sharing
  one column's width while neighbours resolve independently is structurally a
  silent-offset bug — the table shares the whole prefix by construction); no
  fixpoint, only ordering (measure needs left edges only; wrap strictly after
  stops fix).
- **Conductor rulings on the weft flags:**
  **rul-weft-table-lands-with-adapter** — the named-table build goes to the
  W2 adapter lane (first real consumer demands in hand; builder's own
  recommendation), with both its design questions RULED now:
  **rul-table-degrades-whole** — a table stacks/hangs as a unit on its
  narrowest member; a narrow member forcing a wide one to degrade is the
  honest behaviour (coherence is the table's purpose; degradation policy is
  the renderer's per `28E:rul-renderer-owns-layout`; looks-bad cases are
  kTASTE tuning, not structure).
  **rul-layout-asserts-measure** — the layout pass asserts its actual left
  edge equals the measure pass's prediction (the drift class pinned as an
  invariant, debug-assert + test).
  **rul-weft-geometry-vs-words** — weft may self-mint WORDLESS geometry only
  (ASCII punctuation frames `===`/` | `/quotes, separators, truncation
  glyphs), honestly stamped `Arrangement{key: None}`; any English WORD —
  including the `OR` join connective — arrives from the consumer as a
  row-backed run. This is the line reconciling weft's glyphs with `28G` §0's
  every-string-is-a-row law.
  **Vocabulary banked for the W2 adapter** (builder's corpus-fit findings):
  speaker-row PAYLOAD TRAILER for event metadata (`(ran 01:59:52, rc 0)`)
  sits OUTSIDE the quotation — attributing run metadata to the speaker puts
  words in their mouth; `OR`-branch indent diverges one level from the
  strawman (formatting-tier, tolerated pending the table).
  `plans/28G` §2's render-crate-deferred entry rewritten (kept-current) to
  reflect the skeleton landing.
- (note-to-successors) cold clippy on this box is legitimately ~6s wall —
  check-only work on a dep-light graph; the slow tier is test LINKING. Don't
  re-derive the false alarm; the `28A` incremental-stale hazard still stands.
- **rul-ascii-debt-split** (conductor): product-wide ASCII debt found by W1 (the
  locks carry `—`/`…`/`§`/`⇒`/`⊤` etc. in message/help prose) splits two ways:
  the MECHANICAL punctuation map is a small lane dispatched now
  (lane-ascii-sweep, parallel with weft, loom-workflow prose edits + promote);
  the JARGON class (`⊤`/`⇒` needing word replacements) is prose-AUTHORING —
  W5-held territory, left + reported, flagged to the human.
- **lane-ascii-sweep LANDED + FOLDED** @ merge `57403966` (4 commits): 108
  replacements across both locks' product prose, dev-metadata verified
  byte-identical, prose-state ledger preserved (74 `sm ` / 76 Migrated / 0
  Authored unchanged); guard tests shrink-only in BOTH directions
  (`ascii_output.rs` ×3, negative-controlled, >100-register vacuity floor).
  THREE residue classes now precisely named: (1) JARGON — `⊤`/`⊄` in 9 registry
  rows, allowlisted, held for the human prose pass; (2) PAYLOAD-borne — 7 rows
  whose glyphs arrive as runtime `{{detail}}`-class values (sweeping the sample
  would falsify it against its emitter); (3) EMITTER-side — ~960 non-ASCII
  chars across 746 lines of `crates/**/*.rs` (detail-emitters, reason-strings,
  test literals) needing a product-vs-scaffolding judgment →
  **lane-ascii-emitters scheduled post-W2b** (after renders settle, avoiding
  double churn on emit sites W2b rewrites).
- **loom-tooling debts banked** (ascii lane's pipeline findings; pre-W5 repair
  candidates — the human's prose sprint hits every one of these):
  (a) promote's `replay` route cannot reach world-as-payload cases (4 named
  rows edit lock-direct today, contradicting `_prose-worklist.sh`'s
  loom-is-the-home claim); (b) `unreflow` injects a trailing `\n` into any
  register whose edited section ends the transcript (silent re-introduced
  drift; fix in the compile path); (c) lint-route cases need
  promote→rebuild→re-edit two-pass (compiled-in CATALOG lag, undocumented
  cost); (d) `apply_arrangement_edit` unconditionally flips
  Migrated→Authored — a mechanical edit falsely claims human authorship and
  drops the row from the awaiting-prose worklist (wants an edit-mode);
  (e) `run_lint` has no `BLESS=1` write-back (two loom expectations hand-
  re-blessed). None forced or worked around; fixpoint gates prove integrity.
- (human-typed) **lane-loom-cleanup AUTHORIZED, pushed LATE** — a cleanup pass
  over the loom/errorloom tooling (the five banked debts + accumulated
  builder-reported frictions) runs toward the arc's end so it doesn't churn;
  errorloom is meant to grow battle-tested. STANDING BRIEF LINE from here on:
  every builder reports loom/errorloom friction upward as a distinct report
  section (observations only; no in-lane tooling fixes). In-flight w2a lane
  messaged with the addendum.
- **lane-w2-data LANDED + FOLDED** @ merge `c851f30f` (7 commits; zero render
  churn PROVEN — empty diff over looms/goldens/render fns; 66 suites green at
  its tip). Now typed and populated: predict-defining spans
  (`ShippedCheck.defining_span`, `ProbeAttribution`/`ReportedObservation
  { stamp, tool_rc, predict_span }` reachable from `survival_chain`;
  entry-composed/connected-pipe sites honestly absent) · the DI clock
  (`RunClock` seam cli-edge-only, `RunInstant` pure data — no kernel crate can
  even carry a clock type; per-record `ProbeStamp.observed_at`; run-level
  `started=` in the whylog v2 header, round-trips the real binary) · per-record
  tool-rc (single-record facts only — two records report NO single observation,
  picking a winner would fabricate a measurement) · the skipped-count split
  (`DispositionCounts.elide_by_proof`/`.elide_by_trusted_claim`, recomputed
  never stored — the durable stays thin) · host/book-digest/oracle-inventory
  verified already-present.
- **Conductor rulings on the w2-data flags:**
  **rul-test-rationale-comments-exempt** (conduct refinement, standing) — the
  8-line budget collided with the steering-mandated per-test reasoned-argument
  comments; the builder compressed 41→20 and reported instead of deleting
  mandated content — correct. Henceforth test-rationale comments join
  doc-comments as budget-EXEMPT, billed separately in reports.
  **Whylog reader change accepted** (additive `started=`; old durables no
  longer parse) — sanctioned by strawman-formats-never-compat + the declared
  format instability; writer validates through the reader's own predicate.
  **`AID-NEEDS:aid-loaded-oracle-inventory` marked B** (conductor edit, this
  commit); the unverified-caveat line struck.
- **rul-probe-instants-host-says-no-times** (HUMAN-TYPED, 2026-07-25; the firm
  boundary clarified same sitting) — the RULING is exactly: the HOST says no
  times; host-reported timestamps are unnecessary — the controller has all the
  info and mints every instant on its own clock. WITHIN that: dispatch-time,
  received-at, or BOTH are all fine (builder/render-seat latitude; "sketch
  them in" = build proceeds at sketch tier). Consequences for W2b: stamp at
  controller seams (dispatch and/or ingest, both are controller-owned); name
  fields for what they actually are (`dispatched_at`/`received_at` — the
  current `observed_at` implies a host-side observation moment and
  over-claims); a batched wave honestly shares one dispatch instant.
  Render-word nit noted not ruled: `(ran HH:MM:SS)` renders a controller
  instant — wording precision is render-seat latitude under
  output-form-unwelded.
- **W2b scope bank** (accreted from w2-data's flags, joins the W2b brief):
  per-record instants DON'T survive the durable (controller-minted; replay
  stamps Absent) — W2b adds the per-record-instant line-kind to the v2
  ordered state machine (writer+reader one commit) so the receipt view can
  honestly render `(ran HH:MM:SS, rc N)`, under
  rul-probe-instants-are-dispatch-times above · `GuardLicense` carries no
  Derivation/reported row — extend for the wall-link work · `predict_speaker()`
  still fabricates `<provider>__predict` — swap to `defining_span` at the
  render seat · brace-selector display aggregation (from W1's gap 6) ·
  `trigger:` field STAYS a design question for the human (no signal exists;
  nothing invented).
- **lane-w2a-adapter dispatched** (post-fold): the named-table build in weft
  (under rul-table-degrades-whole + rul-layout-asserts-measure) + the aid-side
  adapter re-homing the why render onto weft + the `as-written:` ForeignText
  excerpt block (28G §0's first consumer) + the payload-trailer vocabulary
  (node only; data arrives from lane-w2-data). Runs parallel with lane-w2-data
  (zero-churn guarantee keeps them disjoint on transcripts).
