# 28E — the why-surface design sitting (the seat, the render model, the surfaces)

AI-authored (Fable conductor, sitting WITH the human, 2026-07-25; né `293`,
human-moved into the 28-series same day). THE design durable for the round opened at
`289` §2w; it SUPERSEDES the in-ledger reaction banking (`289` §2z — that ledger is
the dev-arc conduct trail, this is the design summary of record). Goal was one step
toward settled, not settlement: render particulars stay unstable-and-improving
(`27V:rul-output-form-unwelded`); everything below is graded.

Companions: `plans/288` (the unification arc) · `289` §2t–§2z (conduct trail:
hint-homing · gallery · strawmen · fold verifies · findings) · the corpus this
sitting stares at — five `spike/crates/cli/tests/whygallery-*.loom` (as-built
committed truth, incl. per-line and zero-arg `dorc why` transcripts) +
`notes/292-why-output-strawmen/` (conductor aspirations, written unpoisoned) + the
human's in-progress hand-edits of 292 · `plans/286` (the explain/teaching tier this
sitting leans on) · `plans/282` (loom transport) · root `AID-NEEDS.md` (surface law).

## §0 — Ledger

HUMAN-TYPED, firm:

- **`rul-ascii-output-forever`** — product output is pure ASCII: "no unicode,
  ever. period. anywhere... permanently." 90s-leaning by taste and by policy. Bites
  now: the as-built chain gutter prints a Unicode elbow (`top_run_reason`) — an
  ASCII respell is owed, cheap, rides any render lane; committed transcripts
  re-bless with it. Unifies with spike/docs' ASCII law; steering-sync owed.
- **`rul-trust-spent-first-argless-why`** — zero-arg `dorc why` leads with
  TRUST-SPENT, always, never capped; danger in the user's face first.
- **`rul-sh-rewrap-is-load-bearing-scope`** — an in-process sh-formatter WITH
  TEETH is in scope and load-bearing: it must render sh both LITERALLY (formatter
  duty: correctness-preserving rewrap, escape/continuation-aware, valid sh out) and
  AS DESCRIPTION (ellipsis-elision of uninteresting spans — "line 12 ... line 17" —
  with opinions about what matters), and stay meaningful at brutal width budgets
  (~40 chars after gutters/nesting). Probably lives outside Dorc's core.
- **`rul-reason-tail-is-the-minimal-register`** — the plan render's per-line
  reason-tail IS the most-compressed render of the same decision-DAG; acked hard
  that this constrains, shapes, and flavours the whole model.
- **`nack-whylog-stores-book-bytes`** — the whylog stays THIN; it never
  swallows book bytes. The drift/history answer is source-tracking integration
  (below), with a Dorc-owned book-cache only as the doubted fallback if git proves
  heavy/unreliable. The whole drift question is PARKED as a rabbit-hole; fence
  carried from the sitting: any byte-source integration is ANNOTATION-TIER only —
  it may say "this run's book is commit X, HEAD has drifted", it never substitutes
  bytes into the receipt render.
- **`rul-tree-render-is-a-firewalled-crate`** — nested code/prose/why-block
  rendering, reflow, and sh-highlighting are a segregated internal mini-product
  (errorloom-precedent; maybe published, maybe not), never polluting Dorc's
  internals; and we run a NEEDS INVENTORY before any library shopping — no bending
  the design to library requirements.
- **`rul-never-a-dinna-do-it-layer`** (hard rule, human-typed on the sibling
  round's kill) — Dorc NEVER synthesizes a negative claim ("Dorc did not touch X";
  "not Dorc's fault") on top of anything other than ENGINE-PROVED derivations:
  never user input, never oracle vouches or at-most claims, never
  machine-response data. Negatives may render only as tier-labeled
  absence-of-evidence with the frame-problem ceiling named in-line; counterfactual
  blind-run prediction is out PERMANENTLY (a second unmeasured semantics). Rests
  directly on suspectness-tracking: our logic is always suspect and says so. The
  POSITIVE pointer half survives (symptom-indexed "these executed lines carry
  claims/proofs touching X, tier-labeled" — attribution wearing an index).
  Subsumes `28D:must-never-assert-a-negative`.
- **`rul-macro-attention-to-footguns`** — taste/marketing/delight drive MICRO
  design; MACRO design-attention allocates to holes, footguns, and dangers to the
  user we protect. `why` earned its attention BECAUSE of the 233-class
  unsoundness purchase; other firefighter dangers (the `28D` A-grades) earn
  corresponding attention. Impact-projection gets CO-FLAGSHIP billing with the
  provenance chain on exactly this doctrine; advertising emphasis falls out much
  later.
- **`ask-tasty-productive-knob`** (human-typed direction; KNOBS mint + naming
  = the human's act, pending) — preference and effectiveness are in PERMANENT
  tension ("tasty" vs "productive"), maintained as a KNOBS-tier deliberate
  un-weld across the whole product surface, refused resolution on principle.
  The data-model law underneath: the model retains BOTH the welded synthesis (the
  productive pole's direction-setting render) AND the narrative-rich residue (the
  tasty pole's material) PLUS the selection metadata relating them (relevance
  grades, superseded-by, implied-by) as first-class data — collapsing either way
  puts the wrong information at the wrong resolution. Upgrades
  prop-register-per-node: registers are two GOALS x densities, both computable
  from one model. Field-trial consequence: measure preference AND outcome
  separately, forever (both poles need signal).

HUMAN-TYPED leans (banked, unwelded):

- **lean-rename-explain-to-teach** — `plans/286`'s surface renames toward
  `dorc teach` (and `dorc why --teach`), hard-dislocating the teaching register
  from firefighting; dissolves `286:fork-register-flag-naming`; makes the
  prior-art teaching-register threat legible in the product's own grammar (it
  bears on `teach`'s economics, not `why`'s). Rides `286`'s unpark;
  strawman-formats makes it free.

- **lean-prose-down-one-step** — the aspiration corpus reads ~7/10 prose-y; the
  annoyed-debugging register wants ~6/10: mechanical explanation over flowing
  prose; "what would bring this back" / "to fix" become labeled STRUCTURAL
  sections, not paragraphs. Prose remains present.
- **lean-why-is-whylog-reconciliation** — `dorc why` collapses toward
  always-reconciling-the-receipt ("why did that happen" — the question the user is
  actually asking); the live/fresh-measurement render becomes `dorc plan --why` (a
  fuller render of a consented probe). why/plan stop being meaningfully different
  surfaces beyond consent + ask over the one global model. Consequences named in
  §5. (Conductor correction absorbed in-sitting: as-built live why never probes —
  it recomputes from supplied records; the fold is surface simplification, not a
  safety repair.)
- **lean-git-source-tracking-secondary** — whylog reconciliation gains minimal
  add-on source-tracking awareness: if the book is in git/hg, mechanically pull the
  run-matching world-state out of the VCS (digest-keyed, exact-or-absent — the
  whylog's stored content digests make the lookup exact when it hits, silent when
  it misses), historical whylogs included. Stays secondary/extra; must never block
  the plain "I slept, why did it break overnight" path.
- **lean-reflow-and-tui-are-live** — a TUI is coming-soon (soft-deferred) and must
  be resize-responsive; reflow is therefore a live design input, not deferred-hard.
- **lean-ordering-is-a-seam** — chain/report ordering (incl. which link to
  distrust first, and aggregate item-order) needs thought and will receive tuning:
  keep it an open seam, potentially configurable, definitely ours to tweak late;
  nack on welding any ordering now. Nothing in the render model may bake
  chain-position order as semantic.
- **lean-start-without-mutual-awareness** — cross-fragment prose awareness starts
  OFF; it is gorgeous when it works, and the machinery + prose-authoring cost may
  be significant. (Sharpened in-sitting, unopposed: fragments may condition on
  MODEL facts — counts, uniqueness, absence, walker-derived — but never on which
  other fragments rendered. "The only unverified claim in this chain" is a model
  query, not prose knowing prose; the flagship epilogue already works this way.)
- **nit-why-steps-are-a-dag** — the numbering must express join shape
  (1a, 1b, 1c; 2. 1a&1b&1c -> consequence), never force a false total order. The
  render vocabulary is born DAG-shaped (retrofit is high-lock-in).
- **ask-cell-human-description** — why-surfaces badly want "what is this cell
  tracking, in user terms"; possibly a first-class oracle-language display surface
  (richer user-facing oracle metadata generally); comment-mining is the unloved
  fallback. NOT settled; shape-candidate in §6.
- **improvements-not-problems** — the aggregate splits three sections: TRUST-SPENT
  (first, loud, uncapped — disclosure), PROBLEMS (genuine breakage, rare),
  IMPROVEMENTS (dorc-could-if-you-did; capped, quantified, cheapest-first, calm).

CONDUCTOR proposals, standing unopposed (offered, not acked; the human's markup
round and the prior-art return both bear on them):

- **prop-parts-at-birth** — aid output stays structured until the rim: seats
  produce parts streams (computed layout runs · registry word-runs · param values ·
  code-block nodes), edges own skeletons; `Explanation.reason: String` dies. Gives
  the hint rows and the reason opener transcript faces; the substrate chain
  narration wants anyway.
- **prop-carrier-to-the-edge** — the ~25 print-in-place cli sites accumulate
  `Carrier` instead; `advisory: bool` retreats from analysis signatures to the
  render edge (`289:seam-diagnostics-print-not-carried`). Design-free churn; land
  opportunistically.
- **prop-unnarrated-is-visible** — an unrendered narrative class renders a
  greppable `[unnarrated: <class>]` line at the deepest pull tier only (the aid
  plane fails toward narration; the `[unwritten:]` burn-down precedent). CAVEAT
  minted in-sitting: with why folding onto the whylog, the durable's record stream
  and the narrative plane must version together or the placeholder lies about old
  receipts.
- **prop-register-per-node** — a consequence of rul-reason-tail: every DAG node
  carries renders down to ~one word; truncation must be legal at ANY link. Register
  is a node property, not a surface property.
- **prop-three-literalness-modes** — shown sh is exactly one of: LITERAL
  (byte-honest), FORMATTED (rewrapped, still valid/runnable, breaks only where our
  own grammar licenses), DESCRIPTIVE (ellipsized, non-runnable, visually marked —
  the display-sh-never-masquerades law makes the mode a rendered property).
- **prop-consumed-set-drives-elision** — the descriptive mode's "what matters"
  opinion is DERIVED, not aesthetic: elide what the decision never read; keep what
  it consumed (entity-bound operands, consumed channels, the reached arm).
  Explanation-printer, not pretty-printer; the part no shelf library has.
- **prop-pure-layout-function** — layout is a pure function of (tree, width):
  resize = recompute, no incremental reflow, DST-clean, goldens pin at the one
  canonical width (`282`). The browser-class fixpoint problem is structurally
  excluded by the document-algebra lineage (Oppen/Wadler; Rust `pretty` family) —
  the shopping-round's anchor prior-art, subject to the needs inventory.
- **prop-own-lexer-highlighting** — sh syntax-highlighting rides Dorc's own lexer
  (token classes -> ANSI at the edge); never a foreign grammar (dialect drift by
  construction). Color is layout-tier: arrangement-owned, never words, absent from
  committed transcripts.
- **prop-span-boundary-tokenization** — the loom transport tokenizes the BASELINE
  at span boundaries (the map is already the attribution authority; whitespace was
  a shortcut), dissolving glued-param mangling and converting multi-word
  sequence-entry refusals into per-run editability. Reframed by
  `289:steer-errorloom-best-to-use` (human-typed): errorloom is unpublished, zero
  compat weight — fix the word-model IN errorloom, possibly as an enriched mode.
- **prop-distrust-order-default** — inside the ordering seam, the default
  which-link-to-doubt presentation sorts by verification-distance
  (consented < claimed < vouched < measured), walker-derived. A DEFAULT only; the
  seam stays tunable per lean-ordering-is-a-seam.

## §1 — The synthesis frame (why this is one design, not five)

Aid output has a lifecycle: MINT (a decision-fact becomes data) -> CARRY (it
travels to the edge) -> COMPOSE (a seat arranges it) -> RENDER (an edge prints).
The standing seams are the stages' debts, one each:

- `289:seam-diagnostics-print-not-carried` — CARRY: cli helpers print mid-body;
  diagnostics never exist as values past birth.
- `289:seam-whylens-render-seat` — COMPOSE: the reason is a pre-composed String
  fragment; no seat owns anchors, so registry words there can never be stamped
  editable (the faceless-row class).
- `289:seam-narrative-render-unconsumed` — RENDER: nine collapse classes minted,
  one rendered; the richest data never looked at.
- The transport siblings (`28A:rul-glued-param-rehole-seam`,
  `289:seam-multiword-chrome-render-only`) are the same flattening on the EDITING
  path.

One diagnosis covers all five: prose flattened to bytes earlier than its last
consumer needed. One direction dissolves them: output stays structured until the
rim (prop-parts-at-birth + prop-carrier-to-the-edge + a walker that consumes
narratives + a span-aware transport).

## §2 — The evidence corpus (what the sitting stared at)

- AS-BUILT committed truth: the five gallery looms now carry real captured
  `dorc why` transcripts (per-line on elided/guarded/survived/declined + zero-arg),
  landed via the multi-replay drive-and-bless completion (`282`'s letter, built at
  `289` §2y). Sharpest as-built gaps, all committed as bytes: the guarded chain
  never mentions its wall; the declined chain shows none of the class/arm that the
  narrative plane holds (and nags against an author's `unsound` ruling — two
  design-intent inversions at once); `--last` has no replayed voice; the elided
  chain is a single line.
- ASPIRATION: `notes/292-why-output-strawmen/` (unpoisoned; invented-capability
  index in its README) — now under the human's hand-editing.
- The human's markup round over both corpora is the sitting's next input; the
  deltas between the three versions of one case are the requirements extractor.

## §2b — Sibling clean-context rounds (2026-07-25, folded)

- **`28D`** (the annoyed-admin firefight survey + its `opaque-approve` seat;
  human-marked-up at `dcbb714f`) is a sitting input of record. The human-ACKED
  payoffs bind sequencing: `28D:pay-attribution-spine-is-one-build` (input
  identity + withheld-action ledger + dependency provenance + partial-apply
  geometry are ONE build — the spine the elide-half already owes — and LEAD the
  forensic tier) · `28D:pay-levers-are-subtractive` (the emergency-distrust
  levers are macro-attention target #1 under `rul-macro-attention-to-
  footguns`: subtractive-only, NO widening sibling ever, and functional while the
  analysis is distrusted — which architecturally sites the levers BELOW oracle
  loading) · `28D:pay-parts-to-the-rim-pays-forward` (outside endorsement of
  prop-parts-at-birth/prop-carrier-to-the-edge; land them wherever ordering is
  free).
- `28D`'s constraints absorbed as GATES (human: gentle understand,
  trust-but-verify): `must-retention-is-one-decision` — ONE retention design
  precedes the whole forensic tier, and retention policy keys on what the data
  COULD HAVE CAUSED (a consumed observable ≠ a fact boolean ≠
  opt-in-stdout-held-for-debugging; never conflated — human STRONG ack) ·
  `must-default-durable-lands-with-its-hardening` — GATES this document's §4
  fold: why-defaults-to-whylog ships WITH the hardening bill (exclusive
  creation · restrictive mode · atomic replacement · bounded reads ·
  trusted-directory rule · visible persistence failure · stated sensitivity
  contract) or ships opt-in · `must-type-the-shell-we-emit` — independently
  CONVERGES with the prior-art round's dont-let-the-readability-transform-be-
  unsound (two blind lanes, one constraint: FORMATTED-mode sh carries
  grammar-typed quoting; DESCRIPTIVE's non-runnable marking is load-bearing) ·
  `must-encode-per-surface` (no universal sanitize; machine envelopes are sinks
  too) · `must-not-acquire-cross-run-state-incidentally` (fleet/trend views =
  retained receipts diffed by an EXTERNAL tool, never engine state; the `292`
  blast-radius capability SCOPES to the current invocation's loaded oracles) ·
  `must-split-the-bundled-entries` (per-line stdout capture and on-disk artifact
  retention never default on, never described as scrubbed) ·
  `must-keep-the-two-planes-typed-apart` (any aid-to-license path is a design
  event). The human's adjacent lean, near-ruling: artifacts/plans/whylogs
  pure-function + digest-verified promote toward build-now/core-promise.
- The second sibling round's export (human-summarized): the
  exoneration/counterfactual KILL — absorbed as `rul-never-a-dinna-do-it-
  layer` above.

## §3 — The needs inventory (seed; grows with the markup round)

Node vocabulary observed so far: chain-link (tier-worded) · join-node (DAG) ·
code-block (three literalness modes, gutter, source locus) · cell-reference (with
the description ask) · labeled structural section (bring-back/fix) · pointer line
(explain cross-refs) · section header with counts · banner (replay voice) ·
truncation-at-link. Constraints: pure (tree,width) layout · ASCII glyph set only ·
editable-span boundaries must coincide with layout boundaries (transport
coupling) · width-40 acceptance case · register-per-node · ANSI color as optional
edge decoration. Non-goals: grammatical composition across fragments (kFLOW's
refused extreme); prose-aware-of-prose.

## §4 — Surface fold (why / plan / whylog)

Three record-sources, one model, one renderer: records-from-argv (tooling/harness
posture) · the whylog (the user's "why did that happen" — the default `dorc why`)
· fresh consented probe (`dorc plan --why`). Hardens the always-on whylog
requirement (the `--whylog-dir` opt-in was a disclosed spike-cut; why-defaults-to-
whylog makes zero-setup load-bearing; sensitivity fence rides along). Surfaces
keep OPPOSITE selection-policy defaults (plan = pre-consent offer, push-budgeted;
why = post-hoc receipt, pull, wide-open) — fold the machinery, never the policies.
Drift: PARKED (nack above); as-built 22F desync-refusal stands meanwhile. Known
argv bugs banked at `289` §2y (why-last address order; sibling-note false-fire).

## §5 — Deferred / parked (named)

- The drift/book-byte rabbit-hole (nack'd; annotation-tier fence stands).
- ask-cell-human-description — shape-candidate: a per-kind display-tier emission
  member (`__describe`-shaped, engine-owned grammar, decision-inert, strip-
  surviving working sh = documentation-that-executes). TRAP fenced: a description
  explaining a confusing cell is often a missing model distinction (the sysctl
  @value/@persisted lesson generalized — the 27W modeling-crutch caution). Measure
  need across the markup round before minting any role. PRECONDITION from `28D`'s
  seat, absorbed: decide execute-on-host vs read-statically-from-source BEFORE the
  mint (execution = a new probe-time surface inheriting the read-only contract +
  encoding obligations; the `27W` static-first precedent makes static-read the
  lean, with execution-never-required as the candidate mint condition).
- Ordering tuning (the seam); mutual-awareness beyond model-facts; the enriched
  errorloom word-model API shape; the TUI itself.
- Phase 8 (prose burn-down) remains HELD on the human's ack, unchanged.

## §6 — Round state

- Prior-art research round COMPLETE: `.claude/research/dag-explanation-ux/` —
  100 manifest rows (30A/47B/19C/3D + the Lee addendum at B), synthesis +
  addendum + the researcher's own §A0 prediction-correction (the "most likely to
  change the design" billing was wrong, recorded visibly). Reddit unread
  (robots.txt honored); the five §5-flagged gatherer-graded claims (PubGrub
  spellings; Miller passages; Clang commits; Nix pair; Soufflé heights) carry a
  SPOT-VERIFY DEBT before any weld cites them externally.
- Janitor sweep #3 complete under the typed license (`289`-trail material);
  `_branch-purge.sh` handed to the human; SyncThing ignore-config remains the
  human-owned root cause.
- The human's markup round over the strawmen corpora remains the sitting's next
  input; phase 8 (prose burn-down) remains HELD on the human's ack.

## §7 — Prior-art adjudication (conductor WITH the human, 2026-07-25)

Dispositions below are adoption-grades over the round's findings, not source
grades; the round's own transfer caveats (CS1-novice lab base; revealed
preference weighted higher for our population) are carried, not repeated.

ADOPTED:

- **adopt-outcome-metrics-for-the-trial** (dont-validate-by-preference) — the
  field trial measures did-they-act / how-fast / was-it-correct AND preference,
  SEPARATELY (both knob poles need signal); `252`'s "was it illuminating"
  question dies. Protocol amendment banked for the trial revival.
- **adopt-contrastive-first** — the chain's mid-register answers "why THIS
  disposition rather than the other one"; the foil is the line's other
  disposition (free); the full derivation sits one step behind it.
- **adopt-endpoint-excerpts** (+ head-and-tail) — excerpt the site and the
  leverage point in LITERAL bytes (recognition is the point); text-only middles;
  elided middles counted and named (the Clang shape). The massaging license
  (`27W:rul-report-surface-massaging`) becomes exception-for-middles, never the
  anchor default.
- **adopt-reading-direction-line** — cheapest fix in the corpus; state it.
- **adopt-question-relative-informativeness** (Lee, the one portable idea) —
  demote links that only restate what the asker's invocation already fixed (the
  line, the disposition); three traditions converge here (Grice, the contrastive
  do-do, Lee's subtract-the-asker's-constants metric).
- The Bazel cash-out audit PASSES today by construction
  (law-collapse-mints-evidence); named gaps: C8 operand values (parked rider),
  apply-report PREDICTED-marking (real-executor era).

ADAPTED:

- **adapt-two-rank-default-render** — the six TrustTiers stay typed law (the
  honesty/liability property; untouched); the DEFAULT render carries TWO ranks —
  machine-verified vs human-claimed, the only cut the naked-trust epilogue
  needs — as glyph/emphasis, not vocabulary; the six words surface at depth from
  a closed, documented, STACKING table (the Gradle shape; hosts oracle-author
  `because`-prose someday). The round's most-threatened-element finding, answered
  at the render, never the type.
- **adapt-join-only-numbering** — number only nodes referenced non-adjacently
  (PubGrub); linear chains render as numberless prose; joins render
  branch-blank-branch with a prose RESTATEMENT at the join (the restatement is
  the load-bearing half Cargo's bare `(*)` lacks); the epilogue points BY
  TIER-NAME, not index. lean-ordering-is-a-seam untouched.
- **adapt-pull-wide-open-reglossed** — pull answers are complete over the ASKED
  question; the full closure stays one step away and is LABELED
  exhaustive/unselected (the dilution effect dissolves under that label);
  Kulesza's completeness-counterweight noted, independently supported by Lee's
  collective-failure passage.
- **adapt-chain-as-residue-after-labels** — everything label-able onto the
  anchor excerpt (primary/secondary, rustc-style) demotes to a label; the chain
  carries only what code cannot. Strawmen-round-2 tests this shape.
- **adapt-conciseness-as-cap** (Lee) — both poles degenerate (the full closure
  AND the bare tail); the middle registers are the product; k-choice is a render
  default tuned later by field data.

HELD / PUSHBACK:

- **held-placement-reread** — the 0%-to-70% placement result names OUR plan
  surface as the diff-time analog: priority flows to the reason-tail (converging
  with `rul-reason-tail-is-the-minimal-register`) and pointer lines must be
  copy-paste-true; `rul-chain-is-pull-only` SURVIVES.
- **held-teaching-threat-to-286** — rustc-`--explain`-died + preferred-not-
  effective carried to `286`'s unpark gate (its economics were designed against
  the abandonment cause); instrument ladder-climbing in the trial
  (ask-do-ladders-get-climbed — no such telemetry exists anywhere; we can be
  first).
- **held-lee-algorithm-not-imported** — pattern summarization needs a homogeneous
  population to generalize over; our ~6 heterogeneous links have none. Sharper:
  Dorc chains are depth-BOUNDED by the license taxonomy (one link per license
  component), unlike recursive Datalog derivations — what grows in the wild is
  join WIDTH and site COUNT, measured at the field trial.
- **dropped-measure-chain-length-now** — conceded to the human's sideeye (a
  histogram over our own hand-authored fixtures is circular); re-homed to trial
  instrumentation, where Lee's you-cannot-choose-k-without-it point re-attaches
  honestly.
- **resolved-impact-vs-provenance** — by `rul-macro-attention-to-footguns`:
  co-flagship billing; provenance keeps the naked-trust receipt duty; the impact
  projection (the positive symptom-index; the improvements section) grows under
  the same attention doctrine.

## §8 — The strawman-one convergence round (2026-07-25, session close)

The human worked `28G`'s flagship case over two markup layers; the deltas are
design rulings extracted by demonstration (the loom's git history is the record):

HUMAN-DEMONSTRATED (via markup; conductor-confirmed in the fan-out):
- UI voice is admin-English: "skipped" / "guarded" / "guard-clause" on user
  surfaces (the skip-ban was always LLM-facing; the carve is now exercised
  deliberately). Engine vocabulary (elide/replace) never leaks into renders.
- The argless aggregate is TWO sections: TRUST SPENT (first, when present) and
  IMPROVEMENTS ("dorc can do better, with your help") — PROBLEMS dissolved into
  improvements; genuine breakage would surface as SURPRISES (the divergence
  case's leading section when no trust was spent).
- Row shape: `file:N | command` items; `N|command` inline refs; `=== SECTION ===`
  dividers; three-space render indentation; `risk-profile:` labeling in the
  receipt header.
- Speaker rows quote their payloads ("Package:nginx@{enabled,active}" — brace
  selector display); the claims row carries its explanation as an indented
  paragraph below the quote, then the as-written excerpt.
- NEXT STEPS grew alternatives and a full remediation arc: suspect → fix (widen
  the claim OR re-plan flagless — the first emergency-distrust lever appearing
  organically, `28D`'s family) → verify (`plan --why`) → repair (`dorc apply` —
  the handoff to action) → review (`--all`).

RULED/BANKED this round:
- **rul-reported-never-measured** (acked, recurring — the reasoning restated
  because 'measure' has research-ocean momentum): we NEVER know what was
  measured; we know what the author's check REPORTED when it ran. The runtime
  tier-word is `reported`, permanently.
- **rul-danger-axis-is-completion-class** (nack on my reported/written split) —
  the firefighter's axis is not where words were written but what a claim
  COVERS: reported things are a class of size N; the at-most claim speaks for
  universe-minus-N. `!` marks covers-unmeasured. Tier word stays `claims`
  (momentum); `sworn` auditioned as the stellar-term candidate (testimony
  register: reported = eyewitness event, sworn = affidavit about everything
  else), undecided.
- **quoted-speakers ADOPTED** — every chain row is speaker-first with the
  tier-word as the sentence's verb; past tense for run events (`reported`),
  present for standing text (`vouches`, `claims`, `declines`); dorc asserts no
  world-fact in its own voice — it quotes speakers and vouches only for the run
  record and its own derivations.
- **the goals/consent/format matrix** (human vibe, not ruled) — one world of
  facts; verbs state GOALS and imply consent+format defaults, both tweakable
  within the verb. `why --probe` is TEACHING (the consent you could have given);
  `plan --why` is REMEDIATION (re-measures under plan's standing consent,
  carries the asked question inline, poised for apply). Float, weight-tagged
  curiosity: a durable ask-record so plan can highlight "the line you asked
  about is still about to go un-run".
- **rul-renderer-owns-layout** — semantics marks critical-vs-summarizable;
  the reflow engine RULES layout (columns, blocks, wrapping). Never lower
  rendering decisions into the semantics engine; enrich the renderer to
  understand semantics instead.
- **presence-complete, density-selected** — the participating-lines block lists
  every participant of the asked question (completeness lives in code-gutter
  rows, cheap to scan); ANALYSIS selects; value/CFG-tracing decides how MUCH
  each participant gets, never whether it appears (a dropped participant is a
  false provenance claim).

The strawman corpus of record: `notes/28G-why-strawmen-v2/` — `a-fire-morning`
(flagship; human-marked twice, conductor-retold) · `b-wide-guarded`
(healthy-guard, two walls, value-flow, no-suspect steps) · `c-declined-unsound`
(authored refusal as answer; anti-nag; model-offer) · `d-guard-fell-through`
(SURPRISES; then-vs-now; positive-pointer suspicion with the ceiling named) ·
`e-skipped-quiet` (restraint; the admin's own guard as speaker; triptych
collapse). The 292 corpus is superseded by 28G.

LIVING GOAL, restated so it survives the session boundary: every render above
stays RENDERING-ASSEMBLED FROM EDITABLE SPANS — reverse-inferred from loom text
through the tagged-render/span-map machinery. Section headers, tier verbs,
connectives, and footers are arrangement-registry rows; chain prose is
catalog/class-keyed rows; speaker rows are fixed-runs-interleaved-with-values.
Nothing above may ever be born as a `format!` literal.

Implementation phasing: **`plans/294`** (minted at session close) is the
plan-of-record; this document stays the design record.
