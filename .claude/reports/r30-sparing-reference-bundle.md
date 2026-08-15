# Dispatch bundle — r30 lane-sparing-rederivation (a): the reference model

Conductor: Fable, r30-conduct. One lane. Human standing ack: codex "use as you see fit"
(2026-08-14, plan-ack sitting). Base SHA: 5e6d6788 (verified green both legs).

=== DISPATCH: sol | mode=worker | base=5e6d6788 ===
You are building a small, standalone Rust crate in the Dorc repository:
`dorc-sparing-reference` — an obviously-correct, naive REFERENCE MODEL of Dorc's
sparing/composition algebra, authored from the project's ratified English law-set
ONLY. It will later be run differentially against the production implementation; its
entire value is STRUCTURAL DIFFERENCE — written under different constraints, from the
machinery-free description of the goal, in one pass, with no worklist, no caching, and
no cleverness. The dumbest correct spelling wins.

FIRST, before any other reading or work: read, in full,
`Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` (relative to the repo
root you are working in). This dispatch is the orchestrator's explicit pointer to that
one file; read nothing else under any quarantine directory; follow that file's own
guidance about anything it wants routed back to the conductor.

THEN read, in order: `README.md` and `DESIGN.md` (the human-written ground truth);
`spike/CLAUDE.md` (sections "The product, distilled" and "Invariants — separation,
worlds & survival" — your laws); `Research/notes/277-entity-algebra-design.md` §2, §3,
and §5 (THE spec for the compare relation, the sparing rule, and set-lifting);
`KNOBS.md` section "Named mechanisms".

HARD CONSTRAINT — the blindness that makes you valuable: you must NOT open, grep, or
otherwise study the production implementation of these laws. Specifically forbidden:
`spike/crates/core/src/coord.rs`, `spike/crates/plan/src/survival.rs`, and generally
ALL Rust source under `spike/` — the ONLY spike-tree file you may read is
`spike/CLAUDE.md`. Your model derives from the English, never from the code. Define
your OWN minimal input types (opaque token newtypes for kind/entity/selector/family;
your own coordinate, claim, backing-set, and dialect representations) — a later,
separate lane writes the adapter from production types to yours.

The laws your model must express (verbatim from the ratified corpus; the cited docs
elaborate):
- sparing-algebra: same-entity, a claim SPARES a backing iff BOTH sides carry minted
  selectors AND the claim-token is in the dialect of (the backing's minting family,
  kind) AND claim ≠ backing. Everything else COLLIDES: a selector-less/⊤ coordinate on
  EITHER side, unminted tokens, cross-dialect tokens.
- ternary-compare-consumer-map: compare(cellA, cellB) ∈ {same | provably-disjoint |
  unknown}; same feeds transport only; provably-disjoint feeds survival-sparing only;
  unknown is the safe bottom for BOTH consumers.
- set-lifting-universal-meet: consumers quantify UNIVERSALLY over backing-SETS —
  sparing requires EVERY footprint×backing pair provably-disjoint; any unknown member
  ⇒ collide, at every iteration, whatever the member-resolution order
  (pin-set-meet-order-independence). A compare-verdict never re-enters the relation as
  evidence for a later verdict (pin-no-outcome-as-generator).
- Backing-sets are NON-EMPTY by construction, and ⊤ is never encoded as the empty set
  (universal-over-∅ would vacuously spare).
- top-identifies-with-nothing: ⊤ identifies with nothing, including itself; cross-kind
  "same" does not exist.
- silence-licenses-nothing: every unmodeled/unknown case resolves toward collide.

Deliverable:
1. New workspace crate at `spike/crates/sparing-reference` (register it in
   `spike/Cargo.toml`'s members — the ONLY edit you make outside the new crate).
   Zero dependencies. The workspace lint table applies (no unsafe, no unwrap/expect
   in non-test code, no panics).
2. Pure functions, one pass, total: the compare relation over your coordinate type;
   the per-pair sparing rule; the set-lifted verdict over a footprint × a backing-set.
   Closed outcome enums, no booleans-with-meaning.
3. An instance battery as ordinary `#[test]`s: hand-worked positive AND negative
   examples for every law above, each test naming the law it demonstrates in its
   function name (this-style-of-naming: `an_unknown_member_collides_the_whole_set`).
4. A crate-root doc-comment stating provenance: authored from the ratified English,
   deliberately blind to the production implementation.
5. Where the English is AMBIGUOUS or underdetermined on a point you need: do NOT
   resolve it silently (silent resolution is unratified design laundered through a
   checker). Implement the conservative reading (collide), mark the site with a
   `// FLAGGED:` comment, and list every such point prominently in your report.
6. A report at `.claude/reports/r30-sparing-reference/report.md` inside your worktree,
   committed: what you built, the per-law example coverage, every flagged ambiguity,
   and anything the first-read file directed you to route to the conductor.

Work solo — do not spawn subagents or delegate. Commit granularly as you go, one
coherent commit per step, following the commit-message convention appended to this
prompt (the `AI` label is mandatory on every commit; single-line messages). If your
file-read tooling is unavailable or a required file is missing, FAIL FAST and report
the gap — never reconstruct the laws from memory or general knowledge.

Gates before you finish: `mise run check` clean; `cargo test -p dorc-sparing-reference`
green; `mise run gate:quick-quiet` green. (The conductor runs the full cross-platform
acceptance gate at fold; you do not.)
=== END DISPATCH: sol ===
