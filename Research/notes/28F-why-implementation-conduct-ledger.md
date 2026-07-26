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
