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
- `288` §10's sole open ask is CLOSED: `289:rul-mint-hardening-package` (§2). No open
  asks to the human; phase 8 remains the one held item.
- Phase plan per `288` §8 + the acked compression lean: 0∥1 (running) → 2–4 as one
  checkpointed lane → 5 serial (atomic path move) → 6 → 7 (arrangement-home design
  sitting + build; help-text pilot) → 8 HELD (human ack required) → cleanup builder.

## §2 — Rulings (conductor, this arc)

- **`289:rul-mint-hardening-package`** (HUMAN-ACKED 2026-07-24, closes `288` §10) —
  build prop-mint-completeness-hardening as the four-part gate package, NOT the
  type-level/value-carriage kernel refactor: (1) a no-wildcard exhaustive
  `match CollapseKind` inside the completeness gate (compiler forces every new
  collapse class to visit the pairing site); (2) a tidy-style census over
  collapse-constructor call-sites (`error_codes.rs` shape); (3) the merge-mint
  pairing `debug_assert` promoted to a release-mode test gate; (4) DST
  fault-injection assertions per collapse class (force the seam, assert the
  narrative minted AND the chain renders it) — item 4 leans MANDATORY per the
  human's correctness-over-machinery lean ("if you consider 50% and 60%, choose
  60%, don't force 98%"), and is hard-mandatory if the builder finds a missing
  narrative silently OMITS a chain link rather than rendering `Unexplained`
  (builder must check + report which is as-built). Named escalation seam, build
  NOTHING now: if the census gate ever leaks a real under-narration,
  value-carriage-in-the-join is the priced next rung. Rides phase-1-exec or the
  2–4 lane, whichever `notes/290` says is cheapest.

## §2b — Phase-0 landing (conductor, 2026-07-24)

- LANDED @ `2bf13785` on `ai/r28-unify-p0` (2 commits): count-conditional `plural()`
  helper; BOTH lint sentences fixed — the tally (`render.rs:58`, the `288` §6 phase-0
  item) and the clean sentence (`render.rs:36`, conductor pull-forward under the
  human's correctness lean; its phase-7 arrangement-home disposition unmoved). 10
  expectations hand-edited (8 `.loom` cases + 2 e2e lint `expected.out`); no BLESS;
  no catalog/lock coupling. Builder gates cold-green (1163 unit / 97 e2e / four
  gates). Folded @ `c45be8b8`; conductor own-hand verification: cold `-p dorc-lint`
  rebuild + full clippy + `DRY=1 conduct-bless`.
- **`289:dec-clean-render-net-rides-loom`** — the builder's coverage flag (the clean
  render is untested below the substring level) is answered by a RIDER, not a unit
  pin: pinning full bytes in `report.rs` welds an explicitly-unwelded surface
  (`27V:rul-output-form-unwelded`) and would churn at phase 7 anyway. RIDER on
  phase 6: the lint-case loom conversion adds a CLEAN-run case (transcript pin,
  re-blesses freely) alongside the findings cases.

## §2c — Phase-1 MAP checkpoint rulings (conductor, 2026-07-24; on `notes/290`'s flags)

- Map LANDED @ `00155aaf` on `ai/r28-unify-p1` (notes-only; folded to the stack). The
  §3a rename decisions are ACCEPTED WHOLESALE — specifically dec-collapse-kind-kept +
  dec-trust-tier-kept (rename buys nothing, forces law-slug churn) and
  dec-hostevidence-untouched + dec-errorloom-untouched (genuinely different concepts;
  renaming would be a semantic error).
- **`289:rul-carrier-collision-path-rewrites-only`** (flag 1) — oracle's local
  `enum Carrier` in `predict/mark_grammar.rs` (the `#:`/`:` mark carrier) keeps its
  name; every rewrite touching that file is import-path-based, never bare-identifier;
  the §4d bare-identifier step EXCLUDES it. Correction accepted: cli (44 sites) is the
  heaviest file, not oracle.
- **`289:rul-user-aid-block-stays-pointered`** (flag 2) — the POINTER reading: the
  spike/CLAUDE.md User-aid block STAYS at root (it binds emission sites in every
  crate — plan/cli/analysis builders never auto-load `crates/aid/CLAUDE.md`); the new
  aid CLAUDE.md carries the crate-local registry plus duplicated deeply-critical
  bullets (AGENTS.md sanctions repetition for exactly this); the root block gains one
  relocation-pointer sentence for the moved types.
- **`289:rul-law-reslug-rides-phase-one`** (flag 3) — re-slug at this lane's
  steering-sync: `AID-NEEDS:law-collapse-mints-narrative` (née
  law-collapse-mints-evidence) and spike/CLAUDE.md `collapse-mints-narrative` (née
  collapse-mints-evidence), body wording updated, in-code citation comments grepped
  and updated mechanically; historical docs untouched. Motive: "evidence" was
  deliberately RESERVED for possible future correctness-plane use
  (`288:rul-narrative-layer-naming`); a kept-current law squatting on the word
  defeats the reservation.
- SiteId (flag 4): **TAKE**, as the map's §6 additive prelude, exactly as specified.
- Dead doc-links (flag 5): FIX the two dead links in a prelude commit (stale refs to
  killed types); NO cargo-doc gate this arc (machinery past the lean).
- Ratchet baseline skip-once (flag 6): accepted as designed; must NOT be "fixed";
  executor reports that it fired.
- **`289:rider-diag-tidy-scan-set`** (flag 7) — `SCANNED_CRATES` omits lint +
  dorc-loom (pre-existing): widen it in PHASE 3 when lint findings join the registry.
- Flags 8/9/12 (prov-stays-core · compile_fail doctest · catalog_lock wiring):
  confirmed clean, banked so no one re-derives. Flags 10 (plain-`.rs` test moves, no
  pre-created case dirs) and 11 (hostsim/sweep stay aid-free; investigate-don't-add):
  accepted as proposed.
- Hardening disposition: `289:rul-mint-hardening-package` RIDES PHASE 2 (the map's
  reasoning accepted — a behavior change would destroy the cutover's
  clean-diff-is-the-correctness-proof property).

## §3 — Lane map (update on every change)

| lane | branch | shape | state |
|---|---|---|---|
| phase 0: lint-tally-pluralization | `ai/r28-unify-p0` | single dispatch, worktree | LANDED+FOLDED 2026-07-24 @ `2bf13785` → merge `c45be8b8` |
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
- 2026-07-24: "Ack and approve" on the hardening package + the standing correctness
  lean (prefer 60% over 50%, never force 98%) — binds this arc's judgment calls.
  (TYPED; → `289:rul-mint-hardening-package`.)

## §5 — Dispatch log

- 2026-07-24: phase-0 (Opus, fresh worktree, bg) — lint tally pluralization + golden
  hand-edits.
- 2026-07-24: phase-1 MAP (Opus, fresh worktree, bg) — mechanical extraction spec →
  `notes/290` on `ai/r28-unify-p1`; no engine edits.
