# 27V — the user-aid machinery build phase: whylog, evidence plane, one-catalog

AI-authored (Fable, design-rubber-duck sitting WITH the human, 2026-07-18). PLAN-OF-RECORD
for the engine-internals work the aid design demands *now* — before/alongside block-stdlib,
whose oracle-emission growth multiplies everything here. Authority: root docs,
`spike/CLAUDE.md`, `AID-NEEDS.md` (minted same sitting — the Law section there binds this
doc; laws are not restated here, only cited) outrank this. Companion durables: `27R`/`27S`
(the lint lane this composes with) · `26B`/`26C` (the reactive era + §5b two-lane
direction) · `22A`/`22W` (the r22 spine this completes) · `24H` (the polish acks several
riders discharge). Gap numbers (gap-N) cite §0b below.

## §0 — Charter and the human-typed rulings (this sitting's ledger)

The standing situation: the *live* half of user-aid is built (why-lens, attribution
lanes, first-wall hint, `dorc why` current-run, the lint lane); the *posthoc* half — the
sacred promise "`dorc why` will tell you why that went wrong," which countless design
decisions bottom out on — is fiction at HEAD, and the aid plane under-carries evidence.
Rulings, all human-typed or human-acked 2026-07-18:

- **`27V:rul-whylog-build-now`** — the whylog/posthoc-why is "potentially the most
  important" surface; stream-format churn under the r26 reactive rework is accepted;
  build now. The maximal-coherence goal: ONE machinery serving all phases, whose maximal
  mode is the posthoc-whylog-driven `dorc why` — the mode with the MOST information,
  answering the MOST questions, facing the user at their MOST annoyed.
- **`27V:rul-kill-legacy-diagnostic`** — one mechanism for the whole problem-space:
  proactively rip out the legacy string-slug `Diagnostic` (don't mandate the new one —
  remove the old one). (`AID-NEEDS` gap-3.)
- **`27V:rul-collapse-mints-evidence`** — `AID-NEEDS:law-collapse-mints-evidence`,
  value-level constructors / arena-backfill split as acked. The human's constructor nit
  ("require production of a why-step to merge-to-⊤?") resolves as: YES at the value level
  (collapse constructors demand a `CollapseEvidence` payload — pure data), NO at the
  arena level (kernels stay pure; `22D` stage-1 posture unchanged).
- **`27V:rul-aid-survives-the-spike`** — human lean: the warning-surface is the
  second-most-likely thing to survive the spike; this work is HIGHER-criticality and
  higher-lock-in than spike-median, with human hands expected in it. The "spike code,
  meh" discount does not apply here. (Rides ru-17/held-4: the diag API was already the
  design-for-keeps exception; this phase extends that exception's scope to the evidence
  types + catalog pipeline, and NOTHING else gains the status by analogy.)
- **`27V:rul-error-authorship-tier`** — builders mint codes + defining-case structure
  with explicitly-empty prose blocks; a high-reasoning conductor or the human issues the
  prose from the builder's when/why/how report. Destined for `spike/CLAUDE.md`
  (registered there this sitting).
- Catalog architecture rulings: colocated triple render · world-not-grammar sibling
  codes · params-only templates · one-defining-case-per-code · committed catalog
  intermediate promoted-never-auto-tracked · per-code-full-prose-no-fallback — all in
  `AID-NEEDS` Law; binding here.
- **`27V:rul-output-form-unwelded`** (human-typed 2026-07-18) — the PARTICULARS of
  rendered output (tier-word spellings, numbering, connective phrasing, arrangement
  shape) stay UNWELDED pending implementation. Whether outputs "flow" across the real
  failure-mode corpus — and how much mechanism is worth accreting to improve flow — is
  decidable only from generated output, never upfront, and never locked in by an example
  render in an LLM-authored document. The governing tension is `KNOBS:kFLOW`
  (authorable-mechanism ↔ polished-report; the refused extreme is mechanical
  English-grammar composition from fragments). Defining-case goldens pin CONTENT and
  STRUCTURE; arrangement/wording churn re-blesses freely (goldens-churn-freely).

## §0b — The gap ledger (built-vs-designed, from the 2026-07-18 code inventory)

1. **gap-no-durable-why** — no whylog, no `--last`, no durable reader; USER_STORY's
   headless story is fiction at HEAD (decision digest evaporates on stderr;
   `dorc-records/1` is wire-only). → Lane B.
2. **gap-claim-vs-receipt-unminted** — `OriginKind::{OracleClaim, ProbeResult}` reserved
   since r22, never minted; only `TopCause` is. → Lane A.
3. **gap-two-diag-systems** — legacy string-slug `Diagnostic` coexists with the
   battlefield-bound catalog; the newest lanes (escalation, wrapped, munge) went legacy.
   → Lane C (kill).
4. **gap-suggestion-unwired** — `Suggestion`/`Applicability` zero production emits;
   `RemediationClass` not a registry column; floors never ratified; `Floor::Pinned`
   unused. → Lane C riders.
5. **gap-ack6-sibling-hint-absent** — the ruled unloaded-sibling-oracle hint has no
   emitter. → §5 rider.
6. **gap-hints-unpinned** — zero `hint:` e2e expectations; the first-wall hint is
   unasserted. → §5 rider.
7. **gap-minting-line-threading** — claims/vouches carry no source line; blocks
   stdlib-era attribution (`27Q` §2 precondition, previously unowned). → Lane A.
8. **gap-why-surface-sanitization** — `an-output-sanitization` unbuilt while why/hint
   lanes print host-derived text. → security round (OUT here; fence named).
9. **gap-smalls** — `--risk-faultless-skips` ruled name unparsed (code:
   `--trust-footprints`); `--exit-code` unbuilt; a diag.rs header claims 20 codes over
   an enum of 15. → §5 riders / root-doc queue.

## §1 — Lane A: the evidence plane (the two-plane audit made structural)

The under-appreciation the human suspected is real and verified (2026-07-18 inventory):
the fences against receipts-driving-decisions are strong (`ProvId` `!Ord`; sealed
`core::room`), but nothing *mandates writing* evidence, and not-creating-it is the
cheapest compliance — only `TopCause` is minted; `OracleClaim`/`ProbeResult` reserved
since r22; fact-merge discards its disagreeing operands; declines record nothing.

Build:

- **`CollapseEvidence`** value type(s), demanded by collapse constructors. Known
  landmines, both with in-tree precedent: Eq-EXCLUDED from lattice-value equality
  (fixpoint termination — the `Reach::Top` cause carve, `22W` §2) and k-capped through
  deep merges (the `Join` precedent).
- **The collapse-point audit checklist** (each mints evidence carrying its operands):
  fact-merge disagreement (the two values + minting sites) · verdict-body decline
  (which arm / which gate; feeds `aid-why-decline-narration`) · wall formation (which
  participant, which channel-coverage failure) · substitution refusal (which consumed
  channel was ⊤) · entry-consent denial + entry failure classes (`27C`) · demotions
  (uniqueness/reclassification) · render refusals (already coded) · cancellation
  (RESERVED — r26; the type must not foreclose it).
- **Mint the reserved origins**: `OriginKind::{OracleClaim, ProbeResult}` become real at
  the two obvious sites (a lifted vouch/claim enters the fact plane; a record binds to a
  site), with timestamps/iteration-nonce riding `ProbeResult` for the whylog's benefit.
- **`27V:mech-trust-tier-typed`** — the tier field (`AID-NEEDS:law-trust-tier-is-syntax`)
  on every evidence node; rendered by arrangement code only.
- **`27V:mech-minting-line-threading`** — discharge the unowned `27Q` §2 precondition
  here: claims, vouches, and emission arms carry their defining source span end-to-end,
  so attribution renders file:line (the whole flagship output depends on it). MUST land
  before block-stdlib mints selector-bearing `disturbs` claims.
- **Sealing**: the evidence types get the `27L` sealed treatment from day one — no
  conversion into any license-plane input compiles. The r26 explanation-lane feeders
  (`26C` §5b) later *extend* this plane; nothing here may foreclose them.

## §2 — Lane B: the whylog (the thin durable + replay)

Shape per `22A:concl-10`, updated for the landed records lane: the durable is THIN —
what cannot be recomputed — and the full narration is a rendering of a re-run.

- **Contents (per run, one JSONL-ish file, version-tagged `dorc-whylog/1`-style,
  additive-only fields, NO byte-stability promise):** the invocation record (argv, flags
  — consent flags are chain-links; book+oracle content digests) · the records stream
  as-received (`dorc-records/1` frames; iteration-keyed when r26 arrives) · the decision
  digest · the apply report (per-line outcomes: ran / guard-passed / guard-fell-through /
  replaced; divergence flags; rcs) · the seed. NOT contents: renderings, prose,
  derivations, receipts-for-reuse.
- **One reader**: the why-report is a pure function over
  `(plan × evidence-plane × records × apply-report)`; `dorc why` feeds it live state;
  `dorc why --last` replays the durable through the SAME kernel (determinism is the
  license; the binary-vs-durable version check refuses politely — the `22F`
  book-identity/desync guard, cer-2-shaped, lands here for free).
- **Retention**: N-last + size-cap, local; siting per the embedded-transparency posture
  (headless `dorc-run` writes it quietly beside its work; TODO.md's happy-sibling item).
  rec-5 untouched: write-only, nothing re-ingests, never a verdict cache; kSTATE stays
  parked.
- **Sensitivity (NOTE only, human-directed; do not build):** whylog contents are
  host-metadata-sensitive even secret-free. Fences named at
  `AID-NEEDS:law-whylog-is-sensitive`; the secrets round owns the work.
- ~SUSPECT the r26 reactive rework reshapes the record framing (iteration keys, `26C`
  §4); the format's declared instability is the answer — do not defensively pre-build
  r26 shapes.

## §3 — Lane C: the one-catalog (kill the legacy; defining cases; promote)

- **Kill `dorc_core::Diagnostic`** (legacy string-slug): migrate the escalation lane,
  wrapped-analysis hints, and munge/squat lints onto structured codes; delete the type.
  One catalog, one completeness regime, before stdlib growth multiplies emissions.
- **Defining cases**: per-code case dir (trigger book + oracles + world + auto-comment
  stating when-it-fires + typed param list + the colocated triple render: machine line ·
  terse line · prose registers terse/deep/first-encounter). Builder latitude on siting
  (e2e corpus proper vs a crate-adjacent `aid-catalog/` run AS e2e cases) — build-graph
  hygiene decides; one-line rationale in the landing note.
- **The promote pipeline**: defining cases → (explicit promote, BLESS-law inherited:
  orchestrator-only, fresh binary, diff inspected) → the ONE committed catalog file →
  build.rs parse into the static table (no authored macros — the `inv-no-unsafe` family
  stands; this is the Clang-tablegen/Postgres-errcodes shape). Build NEVER auto-tracks
  the case files — the lag is the assertion (`AID-NEEDS:law-one-defining-case-per-code`).
- **Assertions**: defining case = byte-compare of all three renders; non-defining cases =
  structure + instance-of-template. Completeness gates re-cut from `diag_tidy`: every
  code has a defining case; a defining case that stops triggering fails loud; empty prose
  renders as greppable `[unwritten: <code>]` (legal, ugly, conductor-swept — the
  authorship-tier workflow's mechanical half).
- **Registry re-cut riders**: `RemediationClass` becomes a registry column (ru-27's
  HOW-not-WHO re-cut applies at the re-mint — resolve-dynamism / declare-identity /
  provide-model / structural, not who-fixes); floors get their overdue human ratification
  pass (`22A:gate2-ask-1`); `Suggestion`/`Applicability` either gains its first real
  emitter (the fix-suggestion tier) or is explicitly re-parked with a seam note.
- **Arrangement code**: the why-report walker (numbering, indentation, list-joins,
  tier-word rendering, engine-owned value formatters). Chain-position siblings only per
  world-state variants (`AID-NEEDS:law-codes-vary-by-world-not-grammar`); the walker's
  output shape rides `27V:rul-output-form-unwelded` — build the simple thing, let the
  generated corpus argue for more.
- **`27V:rider-lint-lane-absorption`** (`27R` §8e, absorbed): the one-catalog also
  subsumes (a) the lint crate's lane-local `Finding`/`LintSeverity` model (one
  structured-diagnostic type, many renders — lint was built registry-thin precisely for
  this swap); (b) the inline oracle-validation emissions, factored book-free
  (`27S:seam-oracle-validate-factoring` — simultaneously the rung-oracle-solo unlock);
  (c) ownership of the machine-format name (`dorc-lint-format/1` folds under the unified
  machine-diagnostics envelope while renaming is still free); (d) ONE severity
  vocabulary (lint's {Error,Warn,Info} + `--fail-on` maps onto the catalog's tiering).

## §4 — The flagship acceptance case (build FIRST, TDD-style)

The survival-bite scenario (worked in-session 2026-07-18; USER_STORY "bought
unsoundness" instantiated): drifted world · a deliberately-lying `disturbs` claim (the
sweep's `Lying-*` split exists for exactly this) · `--trust-footprints` set · the wall
really runs · the downstream elision survives · the world breaks · `dorc why 9` (live)
and `dorc why --last 9` (replayed) both produce the full chain render: numbered links,
tier words (measured/vouched/ran/claimed/derived/consented), file:line on every artifact,
the naked-trust epilogue stating the DESIGN truth (which link is unverified by
construction — never an instance guess), re-measure + leverage-point recovery moves.
Render particulars are ILLUSTRATIVE per `27V:rul-output-form-unwelded`: the case asserts
evidence-COMPLETENESS and structure; its byte-golden re-blesses freely as the
arrangement evolves. This case cannot pass until Lanes A+B+C all exist; it is the
phase's acceptance test and the defining case for the arrangement. Its render shares
source material with USER_STORY's "Recovery" section (whose render is likewise
illustrative, not a target).

## §5 — Sequencing, dispatch shape, riders

- **Order**: Lane C's legacy-kill + Lane A's evidence types land BEFORE block-stdlib
  authoring begins (stdlib emissions must be born on the one catalog;
  minting-line threading is a stdlib precondition). Lane B parallels; the flagship
  closes the phase. The lint lane (`27R`/`27S`) is untouched except that its finding
  model joins the one catalog when the legacy dies.
- **Dispatch**: conductor-led lanes, Opus builders, standing law (safety block ·
  step-zero/0.5/one · sonnet clamp · comment budget · four gates + foreground e2e ·
  granular commits). Builders mint codes with EMPTY prose per
  `27V:rul-error-authorship-tier`; prose emissions are conductor/human acts at
  checkpoint. Landing notes: `27V`+.
- **Riders** (cheap, attach to whichever lane touches the file): e2e `hint:` pinning
  (gap-6; `expected-hint` needle files, kWARN keepalive) · the ack-6 unloaded-sibling
  emitter (gap-5) · the diag.rs 20-vs-15 doc-count fix (gap-9) · the
  `--risk-faultless-skips` flag rename lands ONLY with a human ack (ack-3 said keep
  current names; re-ask at this phase's close).
- **Explicitly OUT**: TUI/streaming (r26) · the refutation-rerun dedup mechanism-2
  (deferred unless disclosure-noise demands it) · why-surface sanitization + whylog
  sensitivity (security round; fences named) · the `26C` §5b feeder classes themselves
  (r26 revival; this phase only guarantees the sealed plane they extend) ·
  `--exit-code` (rides `24R` cheap-adds; contract already pinned to divergence-of-world).

## §6 — Confidence

+SURE: the gap inventory (code-verified 2026-07-18); the ruling ledger (human-typed in
session). ~SUSPECT: the defining-case siting call (builder latitude); promote-tooling
cost (scaffold + collate + verify — estimated small, unmeasured); that Lane A's
constructor-demand refactor stays localized (the collapse sites are enumerable, but the
fact-merge touch is in the kernel's hot center — the builder must flag scope growth).
-GUESS: ~10³ codes at production grade (human's estimate; drives the tooling-over-
ceremony bias but nothing structural).
