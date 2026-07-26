# 28G — the why-surface implementation plan (phased; post-sitting)

PLANS-TIER, ahistorical, kept-current: if wrong, rewrite it. AI-authored (Fable
conductor, session close 2026-07-25). Phases the `28E` design record (+ its §8
convergence round, the `28G` strawman corpus, `28D`'s gates, and the
`.claude/research/dag-explanation-ux/` adjudication at `28E` §7) toward code.
Authority: root docs, `spike/CLAUDE.md`, human-typed rulings outrank. The human's
framing at close: reports and errors ARE the product in many ways; some of this
is slightly blocking for first-blooding — worth effort to get right, up to a
point of happiness, not perfection.

## §0 — The one law that binds every phase

**Everything stays rendering-assembled from editable spans.** Every string any
phase introduces is born as a catalog row, an arrangement-registry row, or a
class-keyed row — NEVER a `format!` literal — span-stamped through the tagged
render, so the outputs remain reverse-inferable from loom text (the `282`
pipeline extended, not bypassed). Per-phase acceptance: the new renders either
round-trip prose-bless, or are lock-edit-only with the cause documented (the
faceless/sequence-structured classes). One class of displayed string is NOT ours
and must not be treated as if it were: source inlined for display — oracle arms,
their authors' comments, as-shipped guard sh (W2's show-the-code rows; the
strawmen's `as-written:` gutters). It rides the tagged render's foreign-text
region rather than a row, and is escaped at the terminal edge like any other
not-ours bytes (`28D:must-encode-per-surface`) — W2 is its first consumer, so
the tagging lands there rather than being retrofitted. The `28G` strawmen are the DESIGN TARGETS
the renderer converges toward; divergences get flagged in landing reports, never
silently absorbed.

## §1 — Phase map (sequenced; each one conductor-checkpointed lane)

### Phase W1 — the honest words (IMMEDIATE; first-blooding-blocking; small)

All within today's walker/catalog/arrangement machinery; goldens re-bless freely
(render-form-unwelded). One Opus lane over aid/cli render code + registry rows +
transcript re-bless:

- Vocabulary respell: `measured` → `reported` everywhere user-facing
  (rul-reported-never-measured); admin-English outcome voice (skipped/guarded;
  the round-20 `skip-unresolvable` render-token nit folds in here); engine
  vocabulary never leaks.
- ASCII respell: the Unicode chain gutter dies (rul-ascii-output-forever);
  committed transcripts re-bless.
- Two-rank marks: `*` runtime-backed / `!` covers-unmeasured on chain rows
  (completion-class axis); six TrustTiers stay typed underneath, untouched.
- Quoted-speakers rows: speaker-first, tier-word-as-verb, past-for-events
  present-for-text, payloads as quoted coordinates. This is an arrangement-shape
  change to the existing walker, not the parts model.
- The triptych: `=== OUTCOME / ANALYSIS / NEXT STEPS ===` skeleton for `why N`;
  contrastive OUTCOME sentence (foil = the other disposition); structural
  suspect/fix/verify/repair/review rows; the receipt footer (filtered-disclosure
  + `--all` labeled exhaustive + reading-direction line).
- The argless aggregate: TRUST SPENT first (uncapped) · SURPRISES (divergence
  class) · IMPROVEMENTS (quantified, cheapest-first); PROBLEMS retired as a
  section name.
- File-qualified addresses (`dorc why web.sh:9`) + `N|command` refs.
- New headers/labels/connectives land as arrangement-registry rows
  (`Words::Unwritten` or conductor-authored; NEVER literals) — §0's law.

### Phase W2 — the missing narrations (IMMEDIATE; closes the gallery's findings)

Consumes already-minted data; still pre-parts-model; same-or-next lane:

- Decline narration on pull: `why N` on a declined site renders the class, the
  arm inlined as-written with the author's comment, the anti-nag statement for
  `unsound`, the model-offer where the catalog knows one
  (`289`:fnd-decline-class-is-push-only fixed; `27W`'s design finally on the pull
  surface).
- The wall link in guarded chains (`289`:fnd-guarded-chain-omits-the-wall): guarded
  `why N` names its wall(s) as `!` rows and shows the guard's as-shipped sh.
- The receipt header: invocation record rendered (book digest + git-match
  annotation line, oracle inventory, risk-profile, plan tally, addressability
  line) — `28D:need-exact-input-identity`'s cheap half from existing whylog data.
- The replay/receipt voice: the footer's receipt-analysis framing;
  `289:fnd-replayed-voice-is-byte-identical` dissolves into the receipt-first model.
- `[unnarrated: <class>]` at max verbosity only (prop-unnarrated-is-visible),
  WITH the whylog/narrative version-coupling the caveat demands.
- The participating-lines block: presence-complete over the asked question's
  CFG/value closure, gutter rows; ANALYSIS selects (presence-complete,
  density-selected). The block names the closure it is complete OVER — read at
  03:40, an unqualified "participating lines" becomes "nothing else was
  involved", which is a claim about the world rather than about the closure
  (`28D:must-never-assert-a-negative`).

### Phase W3 — the surface fold, gated (NEXT; medium; the hardening bill)

- `dorc why` defaults to the whylog (receipt-reconciliation); `dorc plan --why`
  becomes the remediation verb (re-measure under plan's standing consent, the
  asked question carried inline). Records-from-argv survives as harness posture.
- HARD GATE (`28D:must-default-durable-lands-with-its-hardening`): the
  default-on whylog ships WITH exclusive creation · restrictive mode · atomic
  replacement · reads bounded independently of the writer · trusted-directory
  rule · visible persistence failure · a stated sensitivity contract — or the
  fold ships opt-in. No partial credit.
- The argv bugs ride along: `289:rider-why-last-address-order` (silent
  wrong-surface at rc 0) · `289:rider-sibling-note-false-fires-relative`.
- Annotation-tier git integration (lean-git-source-tracking-secondary):
  digest-keyed exact-or-absent commit-match line; never substitutes bytes
  (the `28E` nack's fence).

### Phase W4 — parts at birth, carrier to the edge (NEXT+1; the structural middle)

- `Explanation.reason: String` → parts; the why-lens gains a real seat; both
  consumers own their skeletons; the hint rows and the reason opener gain
  transcript faces (`289:seam-whylens-render-seat` closes).
- The ~25 cli print-in-place sites → `Carrier` accumulation; `advisory: bool`
  retreats to the edge (`289:seam-diagnostics-print-not-carried` closes; the
  libtest red-frame noise dies).
- Span coverage extends over the full why render, so the W1/W2 shapes become
  loom-round-trippable (§0's acceptance turns ON for the why surface).
- kTASTE data-model seed honored in the types: the model retains the welded
  conclusion, the residue, AND the selection metadata (relevance / superseded-by
  / implied-by) — both registers computable from one model. No register
  machinery built yet; the type room is what W4 must not foreclose.

### Phase W5 — prose burn-down (HELD; human ack required, unchanged)

The standing phase-8: the Fable prose pass over `sm `/`[unwritten:]` + the
arrangement `Migrated` rows — sequenced AFTER W1/W2 so the words aren't wet
cement. Entry point: `spike/_prose-worklist.sh`. Still gated on the human's ack.

## §2 — Deferred, with re-entry pointers (build NONE of these now)

- **The render crate**: the SKELETON is BUILT (human-directed pull-forward,
  2026-07-25; `spike/crates/weft` — zero-dep firewalled box-model layout,
  total-cover provenance spans, 80/40 goldens; conduct record `notes/28F`).
  Its named-table cross-box alignment design lands with the W2 adapter
  (`28F:rul-weft-table-lands-with-adapter`). Still deferred from the original
  entry: the sh-formatter with teeth · own-lexer highlighting · doc-algebra
  reflow optimization · TUI. The needs-inventory-before-library-shopping rule
  stands for those (rul-tree-render-is-a-firewalled-crate).
- **DAG join machinery** (join-only numbering, restatement joins): current
  chains are linear-with-one-join; build when real multi-claim joins appear
  (likely stdlib/field-trial era). The render vocabulary stays born-DAG-shaped
  in types only.
- **Emergency-distrust levers** (`28D` macro-target #1): its own round —
  product-macro, sits below oracle loading, subtractive-only law. Recommend the
  r30 opener, alongside…
- **The retention design** (`28D:must-retention-is-one-decision`): ONE design
  preceding the whole forensic tier (fleet diff, trends, stdout capture). r30.
- **Transport enrichment** (span-boundary tokenization in errorloom; multi-word
  re-splitting; glued-params): after the W1–W4 render shapes settle, so the
  word-model is cut against real needs (`289:steer-errorloom-best-to-use`
  lifts the compat objection).
- `why --probe` (question-scoped live consent) · the durable-question float ·
  `sworn` and the deep-register tier table · the `__describe` cell-gloss member
  (execute-vs-static precondition first) · `dorc teach` rename (rides `286`'s
  unpark) · TUI. All banked in `28E`.

## §3 — Execution status (arc closed 2026-07-26; conduct record `notes/28F`)

**W1, W2, and W3 are EXECUTED** (one implementation-conductor sitting, eleven
lanes; `28F` holds every ruling and landing). W3 ran AIM-HIGH by human
mid-arc redirect: default-on whylog with the full hardening bill, not the
opt-in fold this plan's earlier text assumed; the drifted morning renders a
D1 degraded receipt (`28F:rul-drift-replay-d1` — the chain is NOT
durable-derivable, so drifted receipts carry header/tally/drift-line only,
honestly). The formatting-engine SKELETON (`spike/crates/weft`) was
human-pulled-forward into the arc (§2's first entry). W5 remains HELD on the
human's ack, its queue grown (the drift rows; the jargon glyphs). **W4 is
the r30 opener** (structural, cross-crate, map-then-execute), alongside the
levers/retention pair, the whylog drifted-wording walk, and the
desync-transition machinery.

## §4 — Acceptance, stated once

A first-blooding-ready why-surface means: the five `28G` shapes render from the
real engine over real fixtures with the W1/W2 voice; every string loom-homed
(§0); the flagship naked-trust case reads like `a-fire-morning` modulo
render-form drift; and the human, running `dorc why` on a receipt they did not
stage, is happy up to a point. Perfection is explicitly not the bar
(`27V:rul-output-form-unwelded` stands; kTASTE stays unresolved forever).
