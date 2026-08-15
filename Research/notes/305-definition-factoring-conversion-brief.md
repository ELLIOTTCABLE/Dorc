# 305 — The definition-factoring conversion: work order (stage-i, second half)

> Tier: conductor-authored work order (Fable, 2026-08-15; wave-one-close stamped).
> AUTHORITY: `plans/28Q` §§0–1/§7/§8 rule; this brief CONCRETIZES, never re-derives —
> where it quotes, the quote governs; where it paraphrases and the source disagrees,
> the source wins and the divergence is a conductor bug to report. Written pre-rewind
> so the dispatching conductor (possibly context-fresh) issues it without
> re-derivation. The fixtures half of stage-i is LANDED (`300` §2
> lane-definition-fixtures; ground truth `spike/crates/cli/tests/floor30-*` +
> `pin30-*`); THIS lane is the conversion itself.

## §0 — Shape of the lane

One Opus builder, isolated worktree, MAP-THEN-EXECUTE: checkpoint-1 (sizing + plan)
is a REPORT to the conductor and an explicit pause for ack before any conversion
commit. This is the arc's biggest lane and the checkpoint is where grounding errors
and license bugs get caught cheaply. Standard riders (verbatim from house law, the
dispatching conductor copies them into the prompt): the `spike/CLAUDE.md` safety
block · the quarantine builder-prerequisite read FIRST · step-zero (pwd-first,
`git -C` every command, branch from the conductor-stated tip + hash verify, `mise
trust` both sides) · step-one reads (below) · no subagents · flag-don't-resolve ·
naming discipline · granular `.gitlabels` commits · `verified-core-discipline` binds
(the lane lives in the kernel). Deviation praxis, said plainly in the prompt: a
brief-contradicting or brief-exceeding move is a QUESTION at a checkpoint, not an
act with a justification in the report.

Step-one reads, in order: `plans/28Q` §0–§1 + §7 + §8 (the stage-i entry) →
`plans/28M` §§7–11 → `plans/28K` (+ `notes/28P` §the-bitem-ledger) → `spike/CLAUDE.md`
(whole) → `spike/crates/{core,analysis,oracle,plan}/CLAUDE.md` → this brief →
`notes/300` §2a (facade seat bank) → the six `floor30-*`/`pin30-*` cases themselves.

## §1 — Checkpoint-1: the sizing (FIRST deliverable, no conversion before ack)

`28Q` §7, verbatim: "The expense center is stage-i's TOUCH-COUNT (the resolution
seats + `build_vouches`' map shape), not asymptotics — O(frames) is bounded by
env-mutating statements and the corpus's common case is one frame. Size this first
in the stage-i brief."

Deliver: (a) the enumerated resolution-seat list as-built (the six seats routed
through `live_source`/`answers_at`, the seventh — `build_wrapped_vouches`, per
`28P:tc-wrapped-vouch-seat-has-no-positional-gate` — and the whyworld/survival
seats, whose unification `28P` priced as "re-lifting that seat's whole world", a
dispatch not a rename); (b) per-seat touch assessment; (c) the conversion plan in
mechanical steps with the DefinitionId type-shape proposed (the conductor reviews
type contracts per house practice); (d) anything in the as-built code that
contradicts this brief or `28Q` — reported, not resolved.

## §2 — The conversion (post-ack; `28Q` §8 stage-i, quoted where ruled)

1. **DefinitionId keying**: every derived row — checks, cell declarations, argparse
   arm-models, enrolled dialect tokens, footprint claims — keyed by the DefinitionId
   that produced it "(SourceFileId, span, custody). Computed once, whole-unit,
   exactly as today. No index multiplication."
2. **The frame indirection**: "The ONLY per-frame structure is the frame →
   live-definition map, which `funcenv` already computes (positional, scope-stacked,
   frozen). A query at site S = `live_definition(frame(S), name)` → read THAT
   definition's rows."
3. **Retire as separate mechanisms**: the agreement veto
   (`28P:dec-the-gate-is-agreement-not-re-resolution` retires in favor of true
   resolution), `live_source`'s whole-unit text-scan winner, and the fold's
   `never_live` subtraction ("a never-live definition is simply live at no frame").
   `oracle/CLAUDE.md live-source-is-the-only-resolution-seat` becomes "the
   frame-lookup is the only resolution seat" (edit it in the same fold).
4. **Helper closures — the WITHHOLD floor ONLY**: "a frame whose live definition
   closes over a helper name that is plural-with-differing-bytes across frames
   withholds; `helper-declaration-contested` stands; constants per
   `28P:dec-constants-ride-per-contributing-file` likewise." The
   snapshot-transplant emission is NOT this lane (its own stage, already ruled).
5. **The hash-munge becomes reachable** exactly as bitem1's ledger predicted (two
   frames, two live bodies, two munged names); `plan/CLAUDE.md
   pinned-definitions-are-the-artifact's-binding` already handles per-guard-site
   binding — consume, don't redesign.
6. **`pin30-wrapped-case-bodied-in-book-verdict` is EXPECTED-TO-FLIP** — it is the
   diagnostic on `28Q` §1's asserted cause ("the oracle-only-vector reading is a
   HYPOTHESIS, so the case-bodied in-book wrapped fixture rides stage-i as an
   EXPECTED-TO-FLIP cell (the asserted cause gets tested, not trusted)"). Its
   probe-results file is deliberately EMPTY so gate-1 itself is the flip alarm:
   when it goes red, AUTHOR the record the flipped behavior consumes — never bless
   past, never mark `PROBE_RESULTS=authored`. If it does NOT flip, that refutes the
   hypothesis: stop and report, do not chase.
7. **`task-verify-definition-vector-walls`** — the `28R:§snapshot` residue item;
   read its definition in `notes/28R` (grep the slug) and land it as specified there.
8. **The stage-0 retroactive audit**: "a records/fact-set diff over the corpus —
   site outcomes can hold while a named predict cell silently becomes an unmeasured
   auto-cell, and backings, survival, and why-chains consume those records; a lost
   measurement is a finding."
9. **The differential cells' engine-agreement half activates**: each `floor30-*`
   cell's committed shells-own-answer becomes an ENGINE assertion (the engine's
   positional resolution must agree with dash∩posh's measured behavior at every
   emitter site). Mechanism is the builder's to propose AT CHECKPOINT-1 (options:
   assertions in the cells' expected.out, a dedicated differential test reading the
   manifests, or gate-9 growth) — conductor adjudicates there.

## §3 — Constraints (quoted; violations are findings, not judgment calls)

- **Gate**: "`syn-single-frame-byte-identical` — a single-frame, single-closure
  world (today's entire corpus) produces byte-identical output. This is the
  migration gate for every stage-i commit." Full corpus, BOTH legs, plus the
  `28T`-era checker gates ("certifier + sparing re-derivation green over the full
  corpus, both planes' votes"); "a certification `Refused` on the new shapes is a
  finding, never churn." Golden drift under the byte-identity gate is a FINDING —
  never re-bless (the sole sanctioned golden changes: `pin30`'s flip, and any NEW
  cells this lane mints; scoped bless is PRE-AUTHORIZED for exactly those, named in
  the report).
- **`funcenv-reads-source-literal-plane-only` preserved**: "frame identity is
  ProgramText-graded; probe-provenance values never site a load decision;
  host-conditional loading stays ⊤ (`rul-unloadable-is-unlicensed` untouched)."
- **`vocabulary-acts-stay-ambient` carve preserved**: "the kind-owner trio answers
  world-noun questions, not book-region questions; their keying is §2's (closure),
  never frame. In-book vocabulary roles keep refuse-with-notice."
- **The winner-shifting rider** (permanent, propagate into the code's doc-comments
  at the seats): "under true resolution every funcenv precision bug is
  winner-shifting (it selects whose judgment governs a site, with no agreement veto
  behind it), so the whole frame solver is license-review-tier forever; funcenv
  precision work is never ordinary value-add."
- **`28P:res-plural-families-withhold-off-peak` CLOSES**: "the blessed-override
  (above `unset -f`) and subshell re-source idioms ANSWER from their
  positionally-live definitions instead of going value-dead" — the floor30 ground
  truths are the reference: a later unblessed redefinition wins from its line;
  `unset -f` removes (and dies at a subshell paren); a re-source restores; sibling
  frames hold distinct bodies; deep-stack binding follows the CALL frame.
- **Two-planes fence**: the load plane never grows a probe-data input; nothing this
  lane builds persists frame state (rec-5; kSTATE parked).

## §4 — Fold gates and report

`mise run check` · both-legs `gate:full-quiet` (separate invocations with a
WSL-local `CARGO_TARGET_DIR` until `work-both-task-wsl-target` lands — the `both`
task currently violates the WSL-target law) · both-legs `test:floor` with
`DORC_E2E_FLOOR_SHELLS=dash,posh` on the WSL leg · `bless:dry`. Report: per-step
one-liners · the seat-conversion table (seat → before/after mechanism) · pin30's
flip evidence + the authored record · the stage-0 audit findings (each lost
measurement enumerated or "none") · gate evidence per leg · every flag. The
conductor's fold review covers: the DefinitionId type contract, the seat table
against §1's enumeration, the pin30 record authorship, and every disclosed
deviation under the deviation-litmus.

## §5 — Explicitly out of scope

Snapshot-transplant emission (own stage) · closure-custody anything (`28Q` §8
stage-ii; awaiting rulings + the sibling conductor's sittings) · world-scopes
anything (stage-iii) · `Reach`'s facade eviction (`300:fnd-reach-lattice-outside-
scope` stands deferred) · the kani battery reshape (`work-kani-battery-reshape`,
post-close) · minispec content (frontier+human only — chafe is report-and-stop).
