# 307c — The definition-factoring conversion: lane report (**DRAFT**)

> Tier: builder lane report (Opus executor, 2026-08-16). **DRAFT — INCOMPLETE BY
> DESIGN.** It covers `305a` §2 plus the first two items of §3; the rest of §3 is
> unstarted and its scoping map is banked below verbatim so a fresh executor
> re-derives nothing. Completing this document is the §3 executor's duty.
> Authority: `notes/305` (work order) as amended by `notes/305a` (rulings);
> `plans/28Q` §1/§7/§8 rules both.

## §1 — What landed

Branch `ai/r30-lane-definition-factoring`, rebased onto `ai/r30-conduct`. Three
commits beyond the prior executor's seven:

- `559178a1` — the `305a` §2 atomic chunk (`KindIndex`/`VerdictIndex` re-key, the
  two `effect.rs` seats, `dialect_minting_source`, ripples).
- `ec0b6888` — the plan-crate test-target declaration (see
  `fnd-cargo-autodiscovery-ingests-sync-residue`).
- `3e01c87b` — the agreement veto retired; the three owed law edits.

### The seat table (as far as the conversion has run)

The six ROLE-lane seats `oracle/CLAUDE.md` enumerates are all converted. Seat
numbering follows `305` §1's enumeration.

| Seat | Before | After |
|---|---|---|
| 1 — effect-map lift (`lift_from_sets`) | `live_source` filter kept ONLY the whole-unit winner's rows | filter dropped; every file's rows survive, keyed `(file, provider, verb)` |
| 2 — `VerdictIndex::from_sets` | `live_source` filter kept one body per provider | filter dropped; keyed `(usize, ProviderId)` |
| 3 — `analysis::effect` predict lane (`live_predict_source`) | `live_source` scan + positional agreement gate + `idx.source_of == live` third condition | one `answering_file` call; third condition DELETED as structurally unnecessary |
| 4 — `analysis::effect` verdict lane (`verdict_cell_or_auto`) | `visible.role_answers(verdicts.source_of(p), …)` | `answering_file` over `verdicts.contains(file, p)` |
| 5 — the three cli ship closures (`shipping_source`) | `live_source(...).filter(answers_at)` | `answering_file` (prior executor, `6b00046c`) |
| 6 — `plan::build_vouches_from_sets` + `build_wrapped_vouches` | `live_source(...).filter(answers_at)` | `answering_file` (prior executor, `d1b2d723`) |
| 9 — survival/footprint `disturbs` lane | **UNCONVERTED** — three forward scans | see §4, unstarted |

Consequences, as `28Q` §1 predicted:

- **The veto is gone.** `LiveDefinitions::answers_at` had zero production callers
  once seats 1–6 resolved, and is deleted. Its three tests are repinned onto
  `definition_before`/`provenance_of`.
- **`live_source` has exactly one caller left**: `dialect_minting_source`. That is
  why the seat-law edit reads true today rather than aspirationally.
- **The chimera is unrepresentable, not gated.** A site's argparse and its cells
  are addressed by the same file index, so "identity through one author's arms,
  cells another author declared" (`271:rul-sin-ordering`, pope-sin tier) cannot be
  spelled. This is the whole point of the factoring and the reason seat 3's third
  condition could be deleted rather than kept as a belt.

### `dec-dialect-keeps-a-whole-unit-fold`

`build_dialect` mints only from `KindIndex::dialect_minting_source` — per provider,
the pre-conversion whole-unit winner. Now that every file's rows live in the index,
minting from all of them would ENLARGE the dialect, and a larger dialect spares
MORE: a silent liberalization of the design's one naked-trust tier, in the dangerous
direction (`28Q` §9 `pin-two-position-sparing`). The field is named for aggregation,
not resolution, and its doc says so; the seat law carries the parenthetical.

### Law edits landed

- `oracle/CLAUDE.md`: `live-source-is-the-only-resolution-seat` →
  **`the-frame-lookup-is-the-only-resolution-seat`**, carrying the
  `dialect_minting_source` carve and the winner-shifting rider.
- `analysis/CLAUDE.md` `visibility-is-full-positional`: mechanism restated as
  RESOLUTION rather than agreement; the retired `source_of` carriage recorded.
- `analysis/CLAUDE.md` + `plan/CLAUDE.md`: the two "built at stage-0" claims →
  "RULED at stage-0 and NOT YET BUILT", citing `fnd-stage-zero-is-not-built`.

The winner-shifting rider is propagated into doc-comments at
`core::definition` (module header), `VisibleRole::answering`, and the seat law.

## §2 — Findings

- **`fnd-stage-zero-is-not-built`** — stage-0 was never built; the commit that read
  as its landing restated LAW ONLY. `305` §2 item 8 (the stage-0 retroactive audit)
  is consequently STRUCK for this lane (`305a` §1) and stage-0 is its own queued
  lane. Full statement in `307` §2.
- **`fnd-reserved-name-error-does-not-refuse`** — the reserved-name error is
  MARKED, not REFUSED. +SURE for the lint trace; the ~SUSPECT can-answer-at-sites
  gap is disclosed and unproven by design. Landed as `5f9e5bc6`. Full statement in
  `307` §2. This is why the authored/munged join-miss maps to `Unkeyed` (the ruled
  permissive arm) rather than to `Ambiguous`.
- **`fnd-survival-footprint-lane-scans-forward`** — a live wrong-elision route.
  Scoping map in §4; UNFIXED.
- **`fnd-ship-predict-stage-is-not-in-world`** — DISCHARGED. Seat 6's composed-stage
  path now routes through the shared `shipping_source` seat instead of an
  open-coded twin.
- **`fnd-corpus-carries-twelve-plural-families`** — twelve textual plural cases;
  seven never load the second file; FIVE (`contest28-*` ×4,
  `guard23-reingest-collision-verbatim`) are held byte-stable ONLY by the
  contested-withdrawal. The tripwire note lives on `ContestedFamilies`' test
  (`4a1d8591`). **This dependency is load-bearing for the byte-identity gate**: any
  change that weakens the withdrawal moves those five cases.
- **`fnd-sweep-duplicates-the-footprint-resolution`** (NEW) — `sweep/src/drive.rs`
  carries its own `resolve_touches_footprint` (line ~510) alongside
  `cli::survival`'s. This is a second copy of the resolution rule: exactly the
  failure `the-frame-lookup-is-the-only-resolution-seat` names, and exactly how the
  verdict's winner once split from the predict's
  (`28M:fnd-verdict-resolution-duplicates-live-source`). Converting one copy and not
  the other would reintroduce that split. Seat 9's fix must address both or
  deliberately fold the duplicate.
- **`fnd-git-deny-hook-blind-to-midrebase`** (NEW, tooling) — `git-deny.mjs`'s
  `isAutonomous()` reads `git branch --show-current`, which returns EMPTY under a
  mid-rebase detached HEAD (**measured: `branch=[]`**). Neither autonomy clause can
  then match, so `git rebase --continue` is refused — and so is `git rebase
  --abort`, leaving a conflicted rebase resolvable only from outside the session.
  The same predicate gates `git commit`, so it also bites at a `rebase -i` edit stop
  and at cherry-pick conflicts. Not lane-specific; it will recur for any agent whose
  rebase stops on a conflict. Proposed fallback, unapplied (harness config is
  human-owned):

  ```js
  const rebaseBranch = () => {
     for (const p of ["rebase-merge/head-name", "rebase-apply/head-name"]) {
        const f = sh(`rev-parse --git-path ${p}`)
        if (f && existsSync(f))
           return readFileSync(f, "utf8").trim().replace(/^refs\/heads\//, "")
     }
     return ""
  }
  const branch = sh("branch --show-current") || rebaseBranch()
  ```

  `rev-parse --git-path` resolves per-worktree git dirs correctly. ~SUSPECT complete
  for the rebase case; detached-HEAD states that are NOT a rebase (a plain `git
  checkout <sha>`) still read empty and arguably should be handled too.
- **`fnd-cargo-autodiscovery-ingests-sync-residue`** (NEW, tooling; FIXED in
  `ec0b6888`) — `spike/crates/plan` had no `autotests = false`, so cargo
  autodiscovered `crates/plan/tests/*.rs` — including SyncThing
  `*.sync-conflict-*.rs` copies — as test targets. Such a copy's name is not a legal
  crate identifier, so the workspace lint gate fails and **every commit in the tree**
  is blocked until a human clears the residue. Cargo's autodiscovery is a corpus walk
  that does not honour `sync-residue-is-never-a-case`; `crates/cli` already declares
  its targets for the sibling reason. Fixed by mirroring that. The nine on-disk
  copies are left untouched (cleanup is human-owned).

## §3 — Scope notes and disclosures

- **`tc-lift-diags-now-span-every-file`** (ENDORSED by the conductor) — dropping
  `lift_from_sets`' `live_source` filter means `derive_predict`'s diagnostics are now
  surfaced for EVERY file's rows, not only the whole-unit winner's. Correct by
  consistency (retained rows deserve their diagnostics), aid-plane only, no license
  effect, and invisible on today's corpus (verified: zero e2e drift), because the
  twelve plural families either never load the second file or are contested-withdrawn.
- **`dec-plurality-withhold-repin`** — `effect.rs`'s
  `the_live_definitions_reason_is_the_one_reported` FAILED under the conversion:
  with no environment, two competing definitions now withhold instead of the last
  one winning. This is `answering_file`'s ruled `NoOpinion` arm (sole answers,
  plural withhold — load order may not adjudicate between two authors, `28K` §6).
  Repinned as `competing_definitions_without_an_environment_withhold_in_either_order`,
  asserting withhold IDENTICALLY under both load orders — a stronger invariant than
  either order-dependent pick, since an order-dependent expectation can only assert
  which expedient is in force. A sibling,
  `a_sole_definition_without_an_environment_still_answers`, keeps the pin a statement
  about PLURALITY rather than about the no-environment posture; without it the
  symmetry pin would pass under an unconditional-withhold regression that would wall
  every hand-built index. The behaviour change is real but narrow: production always
  solves a `DefinitionTable`, so it reaches only hand-built indices and the
  instrument/hint lanes (`coverage`/`sweep`/`lint`), which become more conservative —
  the safe direction.
- **`dis-solved-environment-cousin-dropped`** — the conditionally-authorized
  solved-environment cousin test (asserting "the reported reason follows the LIVE
  definition" under a real frame) was DROPPED, disclosed per its ruling.
  `effect.rs`'s test module has no environment-solving fixture at all — every test
  there is `LiveDefinitions::unsolved()` — and `funcenv`'s `solve_positional` mints
  its own `Interner`, so a cousin needs a shared-interner variant plus table
  construction. The invariant's coverage instead lives at the differential tier: §3's
  `definition_frames.rs` battery asserts file-of-answering-definition against the
  committed `expected.emitted` ground truths across all six plural idioms, which is
  strictly stronger.
- **Comment budget** (my three commits; doc-comments counted separately):
  **259** added lines in `spike/crates/*/src/*.rs`, of which **11** are plain `//`
  (**4.2%**, under the ≤10% budget) and **85** are `///`/`//!` doc-comments.
  Commands:
  ```
  R=559178a1^..HEAD
  git diff $R -- 'spike/crates/*/src/*.rs' | grep -c '^+[^+]'
  git diff $R -- 'spike/crates/*/src/*.rs' | grep -cE '^\+\s*//([^/!]|$)'
  git diff $R -- 'spike/crates/*/src/*.rs' | grep -cE '^\+\s*//(/|!)'
  ```

## §4 — Seat 9: the scoping map (BANKED VERBATIM; unstarted)

`fnd-survival-footprint-lane-scans-forward`. The `disturbs`/touches lane resolves by
FIRST-file-wins and, worse, by first-that-RESOLVES. Both are wrong twice over: sh's
answer is the definition live at the site, and a first-that-resolves scan falls
THROUGH a declining live body into a shadowed one's arms — `28K` §6
`rej-decline-fallthrough-cascade`, the expedient `analysis::effect` retired at stage
D and which survives here. It is a live wrong-elision route because a footprint that
answers from the wrong body can narrow an at-most claim, and narrow SPARES MORE.

Three scans, all in `spike/crates/cli/src/survival.rs`:

1. `resolve_touches_footprint` (~line 181) —
   `touches_sets.iter().enumerate().find_map(…)`. First-file-wins AND
   first-that-resolves (the `find_map` falls through when
   `evaluate_touches_located` does not emit). **Has `node`**, so it can take
   `LiveDefinitions` directly.
2. `touches_defining_span` (~line 236) — first-file-wins name lookup. **No `node`**;
   needs threading.
3. `ship_touches_body` (~line 273) — `touches_paired.iter().find_map(…)`,
   first-file-wins AND first-that-resolves. **No `node`**; needs threading.

Call sites to thread: `cli/src/main.rs:1143` (`derive` closure), `main.rs:2681`,
`survival.rs:90`, `:119`, `:1044`, `:1118`, `:1353`, `world.rs:301`, and
`sweep/src/drive.rs:338` — which calls sweep's OWN duplicate at `drive.rs:510`
(`fnd-sweep-duplicates-the-footprint-resolution`).

The withdrawal half is cheap: `TouchesSet::withdrawing` already exists (the
`28K` §1 forwarder family).

**Open question the §3 executor must get RULED before widening, not after.**
`cli/CLAUDE.md withdrawal-is-applied-once-never-consulted` currently records the
survival seats as known-outside-the-edge
(`28P:res-whyworld-and-survival-do-not-withdraw`), "benign only while those vectors
stay oracle-only", and says widening either seat to the source-wide list "must route
it through this edge first". `one-helper-index-two-lanes` likewise prices widening
the survival/kind lanes as "its own dispatch". `305a` §1 nonetheless names seat 9's
fix as in-scope with pinning coverage. Those read as reconcilable — routing through
the edge IS the specified fix — but the wording wants a conductor ruling before the
edit lands, because doing it wrong reintroduces a half-withdrawal.

Pinning coverage owed for the two-file `__disturbs` shape; unit/differential tier is
acceptable if a golden is disproportionate (`305a` §1), disclosed either way.

## §5 — What remains (the §3 executor's queue, in `305a` §3 order)

1. **Seat 9** — §4 above.
2. **The `never_live` retirement end-to-end** — funcenv + cli + tests. Note `305a`
   §1's rider: the funcenv certifier floor (`folded_edges = ∅`) must survive
   `never_live`'s deletion ON ITS OWN MERITS; add a seat test if none pins it
   independently.
3. **The allow-list plurality census** — asserts every REACHABLE plural family sits
   in an enumerated plural-idiom list; empty at landing, growing to this lane's six
   new cells. Prose-only fallback in this document if the load-set parsing balloons.
4. **The differential test + the six new cells** —
   `crates/cli/tests/definition_frames.rs` per the checkpoint-approved mechanism
   (sourced-files-as-inputs under exact path strings; file-of-answering-definition vs
   the committed `expected.emitted` lines; the two helper cells as WITHHOLD cells
   asserting the closure floor). The six new cells are the pre-authorized new-golden
   class and must be NAMED INDIVIDUALLY here.
5. **`task-verify-definition-vector-walls`** — per `notes/28R` §snapshot.
6. **The whyworld unification** — in scope; if it genuinely cannot land, STOP with
   evidence rather than silently deferring (`305a` §1).
7. **`mise run both gate:full-quiet` + `bless:dry`, FOREGROUND**, evidence per leg.
8. **Complete this document.**

Standing constraints unchanged: the byte-identity gate
(`syn-single-frame-byte-identical`) over the full corpus, both legs; golden drift
under it is a FINDING, never a re-bless; the only sanctioned golden changes are
`pin30`'s expected flip, the six new cells, and seat 9's pinning coverage, each named
here. `pin30-wrapped-case-bodied-in-book-verdict` is EXPECTED-TO-FLIP — when it goes
red, AUTHOR the record the flipped behaviour consumes; if it does NOT flip, that
refutes `28Q` §1's asserted cause, so stop and report rather than chase.

## §6 — Gate evidence (this draft's scope)

Per-leg full-gate evidence is OWED and belongs to the §3 executor; nothing below
substitutes for `mise run both gate:full-quiet`.

| Step | Result |
|---|---|
| `mise run check-quiet` | clean (after `ec0b6888`; before it, blocked by sync residue) |
| `mise run build` | clean |
| `mise run test:e2e` | 141 cases, **zero golden drift** — the byte-identity gate holds |
| `mise run gate:quick-quiet` | **1260 / 1260** |
| `mise run both gate:full-quiet` | **NOT RUN** — owed |
| `mise run bless:dry` | **NOT RUN** — owed |

No golden was re-blessed and no `expected.*` file was touched at any point in this
draft's scope.
