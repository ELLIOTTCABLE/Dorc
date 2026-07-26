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

- (HUMAN-TYPED, 2026-07-26) **ack-decline-tier-ratified** — `TrustTier::Declined`
  RATIFIED; the tc-flag doc-comment in `narrative.rs` retired to a ratification
  note. The tier-ORDERING question is open in-chat (the set is kinds, not a
  monotonic scale; code `Ord` is mechanical map-key determinism only; semantic
  ordering stays the `28E:lean-ordering-is-a-seam` seam).
- **lane-speechact-rename LANDED + FOLDED** @ merge `268b6c36` (5 commits;
  own-hand workspace tests green post-merge). Better than briefed: the
  cli-local hand-stamped `RowRank` field is DELETED — `ChainLink` carries
  `tier: SpeechAct` only and both consumers derive via
  `SpeechAct::knowability()` (kind-constant over all 12 as-built sites:
  Measured/Vouched/Derived→Witnessed; Ran/Claimed/Declined→CoversUnmeasured;
  no consumer disagreement found). Zero transcript churn confirmed.
  spike/CLAUDE.md's spellings list was 6 — `declined` added (7) + the
  SpeechAct/Knowability pointer; law slugs untouched.
  **OPEN JUDGMENT, awaiting first render**: `Consented` has no as-built
  chain render; its `Knowability::Witnessed` assignment is BY ANALOGY with
  Derived (engine's closed-world decisions), documented as judgment-not-fact
  in the method doc — a real ruling is owed when a consent row first renders.
- **W3 EXECUTE GO issued** (same builder resumed, CP-D pattern; merges
  mainline `268b6c36` first; step 11 flag-rename SKIPPED pending the human's
  ack; drifted-wording walk + desync redesign + retention + default-on all
  explicitly excluded as r30).
- **lane-w3-fold MAP LANDED + FOLDED** @ `16cdfb1e` (`_w3-map-DRAFT.md`;
  read-only, stopped at checkpoint as briefed). Bottom line ACCEPTED as ruled
  below: **OPT-IN FOLD, with the whole hardening bill landed anyway** — the
  seven bill items are ≈ one lane, but the default-on flip activates MORE
  than the seven: the FNV-1a-64 book digest (law-named:
  rul-fixture-identity-never-production forbids FNV reaching default
  persistence), unscoped retention (r30-owned by
  `28D:must-retention-is-one-decision`), Windows restrictive-mode honesty
  (real DACLs need FFI, forbidden workspace-wide; the honest mitigation is a
  per-user profile root + a stated contract), and the desync-refusal eating
  the receipt on the edited-book morning. Surface folds now; durable stays
  flag-gated; r30's flip = one line + digest + retention design.
- **Conductor rulings on the W3 map (execute-half charter):**
  **rul-w3-optin-fold-full-bill** — as above; the `28D` gate's opt-in branch
  is the sanctioned outcome, no partial credit taken.
  **rul-safe-store-is-cli-local** — the `FsReceiptStore` reuse is blocked by
  the `dorc-loom → dorc-cli` dep cycle; the safe-write shapes (exclusive
  create, unix mode, capped index) land as a cli-local module citing
  `FsReceiptStore` as reference with a churn-avoidance-disclosure note; a
  shared crate only when a third consumer appears (simplicity > structure;
  ~100 lines of well-understood duplication, disclosed).
  **rul-write-failure-is-error-floor** — whylog persistence failure is an
  ERROR-floor code family, visible on the apply console (advisory would be
  suppressed under `apply` — exactly the run whose receipt matters most;
  22F's error floor is the sanctioned channel).
  **rul-path-hint-must-match-its-doc** — `tc-path-hint-capability-widening`
  is a REAL invariant lie (doc says "never a source-loading capability"
  twice; `load_whylog_replay` uses it as one, unbounded, pre-check): the
  execute half makes code match doc — bounded read, digest-checked before
  any trust, or the hint stops proposing loads.
  **rul-digest-lands-now** — FNV→dependency-free SHA-256 rides the execute
  half (M): law-named surface; landing it now removes an r30 flip-blocker
  (old durables breaking is sanctioned by strawman-formats).
  **Accepted as proposed**: explicit `--results` respell of the 8 in-corpus
  why invocations (transcripts unchanged; ambient stdin/tty alternatives
  rejected) · the sibling-note relative-path fix (~10 lines) · the git
  MATCHES-HEAD line in its S form (Repository-shaped trait in main.rs, pure
  `SourceMatch` crossing, off/injected under test, hung-git falls to
  absence); the "HEAD has drifted" capped history walk is DEFERRED (M) ·
  sensitivity-contract mechanism lands with `sm `-tier prose (the prose act
  stays conductor/human).
  **DEFERRED to r30, banked**: the desync-eats-receipt redesign
  (replay-with-drift-disclosure is design work coupled to the git line's
  drifted wording + retention) · retention · default-on.
  **ACKED (human-typed, 2026-07-26)**: the `--trust-footprints` →
  `--risk-faultless-skips` rename is a GO ("pending for ages, no reason to
  wait") — step 11 un-skipped mid-lane, builder messaged; priced S: one const
  + three parser lines, 6 looms re-blessed in lockstep, ~6 rendered lines,
  one help-page lock row via its loom; sequenced late per the map. The
  long-standing rider-flag-rename-re-ask is thereby DISCHARGED.
- **OPAQUE W25 STAGE COMPLETE + BLIND-FOLDED** @ merge `15edfb05` (5 commits,
  no STOP; per protocol the report carried gates only — all green at its tip,
  1441 tests — and the conductor inspected nothing; merge output suppressed;
  own-hand gate verify run post-fold). Proceeding per the human's directive.
- **lane-speechact-rename DISPATCHED** (Sonnet, hard-clamped, off `15edfb05`)
  + **lane-w3-fold MAP HALF DISPATCHED** (Opus, read-only, STOPS at the
  checkpoint; its map note lands as `_w3-map-DRAFT.md` on its branch) — in
  parallel; the rename folds before W3's execute go.
- (conductor, under human directive 2026-07-26) **rul-speechact-rename** — the
  human directed a quick mechanical rename of `TrustTier` ("tier" squats
  genuinely-ordered critical vocabulary — `claim-tier-gating`'s license tiers
  ARE ordered with serious effects; "trust" misrepresents an unordered
  kind-set as a correctness scale). Conductor name choice: **`SpeechAct`**
  (collision-checked free) — the seven are speech-act KINDS (who speaks,
  what act), matching quoted-speakers (the render verb IS the act);
  inherently unordered; squats nothing ("voice" was rejected — squatted by
  receipt-voice/admin-English-voice; "evidence" was renamed away to
  narrative in 288). `tier_word()` → `verb_word()`. The ordered SUPER-LAYER
  the human named gets TYPED (it has ≥2 consumers: the `*`/`!` marks + the
  naked-trust epilogue): **`Knowability`** (human nit accepted same sitting:
  "completion" reads as sh/tab-completion in a shell tool; "completeness" is
  squat-adjacent to the catalog-completeness gates; `Knowability` is free,
  distinctive, and names the frame-problem semantic — the `!` class speaks
  for what no runnable command COULD witness. The ruled slug
  rul-danger-axis-is-completion-class stays the historical anchor; variant
  names anchor to the ruled phrase — e.g. `Witnessed` / `CoversUnmeasured` —
  builder latitude), genuinely `Ord`, ONE derivation seat
  (a method on `SpeechAct` if the mapping is kind-constant, else the typed
  render-seat fn — builder discovers as-built truth); the kind-enum's derived
  `Ord` stays (map-key determinism) doc'd MECHANICAL-ONLY. Law SLUGS
  (`law-trust-tier-is-syntax` etc.) stay unchanged — historical anchors;
  type-name MENTIONS in spike/aid CLAUDE.mds update. Sonnet-tier dispatch,
  hard-clamped, AFTER the opaque fold (no parallel churn against an
  uninspectable diff); before W3's execute half.
- (HUMAN-TYPED, 2026-07-26; corrected same sitting) **strawmen-b-through-e-are-
  a-gloss** — only `a-fire-morning` is deeply reviewed; `b`–`e` are UNREVIEWED
  gloss: a weight-grading, NOT a work-cut ("I wasn't cutting any work in
  particular"). Build cheap machinery toward them where easy; expect further
  markup layers to re-shape them. Priority: the opaque stage → **W3** →
  pushing forward (the human's first message said W4 — misspoke, corrected).
  Sequencing unchanged from the plan: W3 next after opaque, W4 after.

- **lane-w2b-narrations LANDED + FOLDED** @ merge `c24c5242` (12 commits) —
  PHASE W2 COMPLETE. All ten items: decline triptych on pull + the anti-nag
  routing (`an_oracle_could_still_answer`; the wrong sysctl nag is gone) ·
  guarded chains name their walls + show the as-shipped guard
  (`GuardLicense.probe`, post-mint, canon-exempted exhaustively) · the receipt
  header with the skipped SPLIT · receipt/replay voice (durable's own
  `started_at`, never the replay clock; every receipt states
  apply-report-is-prediction) · `--all` + `[unnarrated:]` with the
  PLANE_VERSION×RECORD_STREAM_VERSION coupling gate · participating-lines
  block naming its closure · the `instant ordinal= at=` line-kind
  (writer+reader one commit; `observed_at`→`received_at`; replay re-stamps
  from the durable) · `reported_speaker` from `predict_span` + the
  `(received HH:MM:SS, rc N)` trailer · per-arm disturbs spans (as-written
  shows the MATCHED ARM; leverage points at the widenable line) ·
  brace-selector display grouping (render-seat only, FactKey untouched).
- **Conductor rulings on the w2b flags:**
  **`(received …)` over `(ran …)` ACCEPTED** — direct consequence of
  rul-probe-instants-host-says-no-times; `ran` would date a host event nobody
  reported. Render-word latitude was explicitly the seat's.
  **rul-fixture-clock-env-accepted** — `DORC_FIXTURE_CLOCK_MS` (read once at
  `main()`, harness-set, real clock default) is the DI-seam-at-the-edge
  pattern; the rejected spike-fixed clock would make every receipt timestamp
  a lie; case files stay user-shaped (the 282 harness-env law binds replay
  COMMANDS, which carry nothing). Flagged onward to the human as a taste nit
  (env vs flag), not a blocker.
  **rul-placeholders-are-computed** — greppable machine placeholders
  (`[unwritten: <slug>]`, `[unnarrated: <class>]`) are COMPUTED, not
  registry rows, by design — they are self-advertising machine states, not
  prose; precedent: the catalog's render-time `[unwritten:]`. This is the
  stated carve on `28G` §0's every-string law.
  **`tc-decline-is-a-seventh-tier` → ESCALATED TO THE HUMAN** (not
  self-ratified): the builder added `TrustTier::Declined` because every
  existing tier word misstates a decline (`vouches` worst — declining IS
  refusing to vouch), and mis-attribution tops the sin-ordering. The tier
  SET is law (`AID-NEEDS:law-trust-tier-is-syntax`) and the human actively
  curates this vocabulary (`28E` §8: `reported`, the `sworn` audition).
  Conductor recommendation: RATIFY (a decline is epistemically distinct;
  the code as-built stays meanwhile — reverting would misattribute).
  AWAITING TYPED ACK.
- **Strawman-distance residuals banked** (honest, caused, none blocking):
  `b`'s value-flow participant (`PORT=443`) absent — no reaching-definitions
  query is exposed to the render seat (engine gap; the closure row discloses
  it) · `c`'s model-offer needs the `__describe` cell-gloss member (deferred
  by design, `28G` §2) · `d`'s SURPRISES/then-vs-now cannot exist without an
  apply executor (real-executor era; `tc-apply-report-is-prediction`) ·
  `e`'s guard-subsumption attribution — the admin's OWN lifted guard as a
  chain SPEAKER — unbuilt (the Half-B reward surface wearing why-clothes;
  candidate for W3/r30) · `trigger:`/oracle-versions/`why --probe` footer
  sentence: no data or deferred, correctly omitted over invented.
- **W5 prose queue, sharpest first** (from w2b): `why-next-step-describe-
  walls` SINGULAR case renders `[unwritten:]` — blanks the next-step of the
  COMMON one-wall guarded case; then the six other Unwritten rows + the
  non-unsound decline-class occurrence prose.
- **Loom-cleanup order accreted (w2b)**: arrangement-fixpoint failures should
  name the first differing slug (a ~120KB assert dump hid a field-order slip)
  · hand-seeded row FIELD ORDER is an unstated invariant — wants a line in
  aid's CLAUDE.md or a targeted assert · `tolerate:` normalizers reach the
  run-log but NOT the replay transcript (future rendered-output
  nondeterminism has no declared-class escape hatch — design gap) ·
  replay-block authoring is a blind two-step (document it) · scoped-bless +
  LCS-diff debts confirmed live again.
- **lane-w2a-adapter LANDED + FOLDED** @ merge `46ca50b6` (7 commits; clean
  cross-merge with w2-data; 1417 tests at its tip). Leg 1: the named table
  built as ruled (measure walk + prefix-sum stops; three ad-hoc measurement
  paths deleted; assert verified by injected-drift red; ragged NEXT-STEPS
  resolved; gutter-is-a-lead). Leg 2: adapter sited `aid::weave` (keys +
  constructors) / cli (composition; only aid+cli dep weft); `Said`
  provenance-typed (`Words(slug)|Value|Lens`); W1's leftover string literals
  gone; six headings' `===` moved to weft geometry. Leg 3: `as-written:`
  foreign block live (comment-walk + cap + counted truncation between
  same-table code blocks; encode-at-mint `\xNN`). Payload trailer plumbed,
  `None` until W2b fills it. Nine divergences flagged-not-absorbed (D1 rides
  the disturbs-arm span gap; D2–D9 accepted: table-limits/judgment tier).
  Weft crate `CLAUDE.md` MINTED at this fold (its F4 flag; the
  firm invariants now sit crate-adjacent).
- **Conduct lesson (from w2a's F1): ledger edits COMMIT IMMEDIATELY** — the
  three weft rulings existed only in my working tree when w2a dispatched;
  builders can only cite committed law. Practice corrected as of `f2fdac14`.
- **W2b scope additions from w2a**: `tc-disturbs-span-threading` (per-arm span
  for disturbs claims — makes `as-written:` show the matched arm + author
  comment instead of the funcdef line; engine-side, plan/survival) ·
  `predict_speaker` defining_span flip (F3, was already banked) ·
  `ChainLink.event` trailer fill. Standing seams re-confirmed, NOT W2b's:
  span-map-unconsumed/loom-round-trip (W4; `Said` slugs make the future
  bridge a mapping not a re-derivation) · `Said::Lens` pre-flattened (named
  in-code).
- **Loom-cleanup work order accreted (w2a's L1–L7)**: `BLESS=<substring>`
  scoped bless (all-or-nothing rides drift in silently) · dry-run candidate
  transcripts to a scratch path (render-lane iteration cost) · LCS failure
  diff (line-paired diff is noise once line counts change) · needle-freeness
  documented (needles survived a wholesale re-home — correct but worth
  stating) · record the sanctioned hand-seed path against the lock's
  DO-NOT-EDIT banner · **L6, the sharpest: arity-mismatch degrades to a
  silent `[unwritten:]` render** — editing a Migrated row's word boundaries
  is invisible to every check but the transcript fixpoint; wants a loud
  pre-render gate (`error-prose-conductor-flow`'s failure taste) · the two
  bless paths' corpus-reach split needs an ordering statement.

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

## §3 — macOS verification round (human-offered 2026-07-26; NON-blocking)

The human can dispatch a macOS round at leisure. Genuine need: W3's unix
restrictive-mode work is `#[cfg(unix)]` — neither type-checked nor executed by
any gate on this Windows box (a known incident class). Plan: the W3 builder
keeps cfg-gated regions trivially thin and LISTS them all, marked unverified;
at the W3 fold the conductor prepares a self-contained macOS packet (checkout ·
mise · full gate suite · the listed unix-path tests to watch) banked HERE for
whenever the human runs it. Findings become riders; nothing in the pipeline
waits on it.
