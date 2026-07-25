# 293 — the why-surface design sitting (the seat, the render model, the surfaces)

AI-authored (Fable conductor, sitting WITH the human, 2026-07-25). THE design durable
for the round opened at `289` §2w; it SUPERSEDES the in-ledger reaction banking
(`289` §2z — that ledger is the dev-arc conduct trail, this is the design summary of
record). Goal was one step toward settled, not settlement: render particulars stay
unstable-and-improving (`27V:rul-output-form-unwelded`); everything below is graded.
§7 is reserved for the prior-art round (in flight at this writing) and lands after
its adjudication.

Companions: `plans/288` (the unification arc) · `289` §2t–§2z (conduct trail:
hint-homing · gallery · strawmen · fold verifies · findings) · the corpus this
sitting stares at — five `spike/crates/cli/tests/whygallery-*.loom` (as-built
committed truth, incl. per-line and zero-arg `dorc why` transcripts) +
`notes/292-why-output-strawmen/` (conductor aspirations, written unpoisoned) + the
human's in-progress hand-edits of 292 · `plans/286` (the explain/teaching tier this
sitting leans on) · `plans/282` (loom transport) · root `AID-NEEDS.md` (surface law).

## §0 — Ledger

HUMAN-TYPED, firm:

- **`293:rul-ascii-output-forever`** — product output is pure ASCII: "no unicode,
  ever. period. anywhere... permanently." 90s-leaning by taste and by policy. Bites
  now: the as-built chain gutter prints a Unicode elbow (`top_run_reason`) — an
  ASCII respell is owed, cheap, rides any render lane; committed transcripts
  re-bless with it. Unifies with spike/docs' ASCII law; steering-sync owed.
- **`293:rul-trust-spent-first-argless-why`** — zero-arg `dorc why` leads with
  TRUST-SPENT, always, never capped; danger in the user's face first.
- **`293:rul-sh-rewrap-is-load-bearing-scope`** — an in-process sh-formatter WITH
  TEETH is in scope and load-bearing: it must render sh both LITERALLY (formatter
  duty: correctness-preserving rewrap, escape/continuation-aware, valid sh out) and
  AS DESCRIPTION (ellipsis-elision of uninteresting spans — "line 12 ... line 17" —
  with opinions about what matters), and stay meaningful at brutal width budgets
  (~40 chars after gutters/nesting). Probably lives outside Dorc's core.
- **`293:rul-reason-tail-is-the-minimal-register`** — the plan render's per-line
  reason-tail IS the most-compressed render of the same decision-DAG; acked hard
  that this constrains, shapes, and flavours the whole model.
- **`293:nack-whylog-stores-book-bytes`** — the whylog stays THIN; it never
  swallows book bytes. The drift/history answer is source-tracking integration
  (below), with a Dorc-owned book-cache only as the doubted fallback if git proves
  heavy/unreliable. The whole drift question is PARKED as a rabbit-hole; fence
  carried from the sitting: any byte-source integration is ANNOTATION-TIER only —
  it may say "this run's book is commit X, HEAD has drifted", it never substitutes
  bytes into the receipt render.
- **`293:rul-tree-render-is-a-firewalled-crate`** — nested code/prose/why-block
  rendering, reflow, and sh-highlighting are a segregated internal mini-product
  (errorloom-precedent; maybe published, maybe not), never polluting Dorc's
  internals; and we run a NEEDS INVENTORY before any library shopping — no bending
  the design to library requirements.

HUMAN-TYPED leans (banked, unwelded):

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
  need across the markup round before minting any role.
- Ordering tuning (the seam); mutual-awareness beyond model-facts; the enriched
  errorloom word-model API shape; the TUI itself.
- Phase 8 (prose burn-down) remains HELD on the human's ack, unchanged.

## §6 — In-flight at this writing

- Prior-art research round (Opus + /interactive-research): DAG-teaching in
  constrained text media, disproof-first; lands in `.claude/research/`; §7 below
  absorbs its adjudication.
- Janitor sweep #3 under the human's typed 2026-07-25 license (dead worktrees,
  cancelled r29 branches, sync-conflict files; rescue-hedge on the three held
  worktrees).

## §7 — Prior-art adjudication (RESERVED — lands after the research return)
