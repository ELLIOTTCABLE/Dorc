# 289 — the aid/loom unification conduct ledger (the `288`-execution arc)

AI-authored (Fable conductor, seated 2026-07-24). Executes `plans/288` whole. Split out
of `notes/28A` by human direction (28A grew too long; it stays closed). Authority: root
docs, `spike/CLAUDE.md`, human-typed rulings outrank. Builders write NO landing notes;
as-built detail lives in granular commits + this ledger. Conductor stack:
**`ai/r28-unify`** (worktree `.claude/worktrees/r28-unify`; base `fbbf88f1` = ai/main
tip at seat), re-pointed/merged at verified lane tips as folds land.

## §0 — Session directives (human-typed 2026-07-24, this sitting)

- Conduct the `288` execution; land it WHOLESALE, except **phase 8 (the Fable prose
  burn-down) is HELD for the human's attention and ack**.
- Conserve conductor context/tokens brutally; trust Opus builders to execute briefs
  faithfully; no conductor churn over CLI invocations/tests.
- Ledger lives in a fresh note (this file), not 28A.
- Conductor works IN A WORKTREE (never the primary checkout); builders get clean/fresh
  worktrees for any modification; conductor rebases/merges their work; a cleanup
  builder at arc end removes completed worktrees/branches (guarded, merge-checked,
  surface-never-delete-unmerged — the `28C` janitor posture).
- Reach the human via notification when genuinely needed; otherwise maintain momentum.

Standing orders still binding (from `28A` §6/§4b, survive rewind): opaque-review
DEFERRED (infrastructure non-functional; do not attempt, do not re-ask) ·
strawman-formats-never-compat-targets (no mapping layers pre-user) · cold-clippy rider
(`28A:finding-incremental-clippy-serves-stale`) in every worktree builder brief.

## §1 — Arc state

- Base: `fbbf88f1` (ai/main; carries the 288 promotion + .loom respell).
- Open ask (`288` §10, the only one): **prop-mint-completeness-hardening** — conductor
  default is BUILD it (cheap, fails-safe, quality-trumps-progress), riding whichever
  of phase 1-exec / phase 2 the map half says is cheapest, UNLESS the human vetoes
  before that dispatch.
- Phase plan per `288` §8 + the acked compression lean: 0∥1 (running) → 2–4 as one
  checkpointed lane → 5 serial (atomic path move) → 6 → 7 (arrangement-home design
  sitting + build; help-text pilot) → 8 HELD (human ack required) → cleanup builder.

## §2 — Rulings (conductor, this arc)

(accretes per checkpoint)

## §3 — Lane map (update on every change)

| lane | branch | shape | state |
|---|---|---|---|
| phase 0: lint-tally-pluralization | `ai/r28-unify-p0` | single dispatch, worktree | DISPATCHED 2026-07-24 |
| phase 1 map: aid-crate extraction spec (`notes/290`) | `ai/r28-unify-p1` | map half (map-then-execute), no engine edits | DISPATCHED 2026-07-24 |
| phase 1 exec: the extraction cutover | off p1 map | fresh executor after checkpoint | pending |
| phases 2–4: mint-seam+scaffold · lint-unification · cli-error-migration | — | one checkpointed lane | pending |
| phase 5: flat-tree move + run.sh retirement + safety-law edits | — | serial, atomic paths-only | pending |
| phase 6: e2e→loom conversion | — | serial | pending |
| phase 7: arrangement-home sitting + build (help-text pilot) | — | design sitting then lane | pending |
| phase 8: Fable prose burn-down | — | HELD — human attention + ack | held |
| cleanup: worktree/branch janitor | — | guarded, end of arc | pending |

## §4 — Ack-ledger (only human-TYPED items count)

- 2026-07-24 seat brief: execute 288; wholesale except phase 8; brutal token
  conservation; trust builders; notification tool available. (TYPED.)
- 2026-07-24 mid-turn: fresh ledger, not 28A. (TYPED.)
- 2026-07-24 mid-turn: conductor-in-worktree; fresh worktrees for builders; cleanup
  builder at arc end. (TYPED.)
- OUTSTANDING ASK → human: prop-mint-completeness-hardening (build-vs-accept-assert;
  conductor default BUILD, veto window open until the phase-1-exec/phase-2 dispatch).

## §5 — Dispatch log

- 2026-07-24: phase-0 (Opus, fresh worktree, bg) — lint tally pluralization + golden
  hand-edits.
- 2026-07-24: phase-1 MAP (Opus, fresh worktree, bg) — mechanical extraction spec →
  `notes/290` on `ai/r28-unify-p1`; no engine edits.
