# 307b — lane-certifier-trip-policy: build report

> Opus builder lane against `Research/plans/302-solve-certifier-spec.md` §3
> `rul-certifier-trip-guard-only` [TYPED] and the two `FORFEITS.md` rows it minted. Branch
> `ai/r30-lane-trip-policy`, cut from `ai/r30-conduct` @ `e5741a7e`. Confidence marks per
> `spike/CLAUDE.md`.

## §1 — Per-item, against the ruling

- **one-boolean-per-spine** — BUILT. `dorc_analysis::certify::CertifierTrip`: a private-field
  monotone latch whose only mutator is `record(&SolveConsistency<L>)`. Nothing clears it, and the
  only way to set it is to hand it a real outcome — whose two mints are already lexically fenced
  to the checker (`the_outcome_has_exactly_one_mint`), so the trip inherits that fence rather than
  needing a second one. +SURE.
- **set-by-any-inconsistent** — BUILT, all four seats. The two pre-network ones latch at
  `dorc_cli::world::record_pre_network_trip` (value's own `consistency()`, and funcenv's
  `EnvFloor::SolverInconsistent`); the two in-classify ones latch inside
  `classify_with_why_diags` (reaching-defs) and `self_reach_pass` (per Members site).
- **latch-crosses-the-fixpoint-rounds** — the latch is an out-param through `classify_round` and
  `settle_validity_fixpoint` rather than a per-round return, and that is load-bearing: intermediate
  rounds are never observed (`cli/CLAUDE.md` the-fixpoint-owns-the-rounds-and-builds-nothing-else),
  so a round-2 reaching-defs failure is invisible to anything reading only the settled round. As an
  accumulator it cannot be forgotten by a caller. +SURE.
- **scope-per-host-plan** — width-one today, so the run: one latch per `run()` / per
  `WhyWorld::analyze`.
- **terminal-cleanup-pass** — BUILT: `dorc_plan::certifier_trip::demote_on_trip`, called from
  `dorc_cli::world::demote_on_certifier_trip` immediately after `build_plan_walled` in BOTH drivers
  (the binary's `run()` and `WhyWorld::analyze_measured`). Seat rationale: nothing between
  `build_plan_walled` and `render_apply` mutates a disposition, so "immediately before
  plan-emission" and "immediately after plan-construction" are the same seat — and only that one
  makes the digest, the why report, the plan summary and the artifact describe one plan.
- **elide/omit/survive all demote** — the three elision-family outcomes are two dispositions:
  `Replace` carries elide-by-proof AND survive (the split is the license's survival witness, not
  the verb — `DispositionCounts::elide_by_trusted_claim`), and `Omit` is the fold-proved-dead
  branch. Both demote unconditionally. `Run` steps are untouched: runs run. +SURE.
- **narrative-per-demotion, reason-arm not sibling** — `DemoteTag::CertifierTripped`, a new arm on
  the EXISTING `CollapseKind::Demotion` reason enum. Pull-tier, exactly as its five siblings are.
- **guards stand on the census** — BUILT; the fork's branch is in §2.
- **§3 consumer floors unchanged** — untouched. No floor was moved, weakened, or re-sited; the
  funcenv fold still BREAKS at the failing round with `folded_edges = ∅`. Verified by inspection and
  by the untouched `302` §6.8 battery in `analysis`.
- **aid: one plan-prominent banner** — new code `solver-consistency-plan-demoted`, Error /
  `Floor::WarnOrDeny`, spanless, one per tripped run, carrying `{demoted}`. Defining loom case at
  `spike/crates/aid/tests/solver-consistency-plan-demoted.loom`, fixture-routed like its cousin
  (`303:fnd-refusal-has-no-honest-trigger`). PROSE EXPLICITLY EMPTY (`message: None`, rendering
  `[unwritten: solver-consistency-plan-demoted]`) per `error-authorship-tier`; the `when-fires`/`why`
  metadata is engineering documentation and is authored, the user-facing words are not.
- **the boolean is a spine row** — the banner joins `identity_diags`, so `canon_diag` keys it into
  the decision digest by (slug, span, severity). NO new durable field: the digest is an existing
  whylog field whose VALUE moves. Deliberately via the diags plane rather than a new canon section,
  because a tripped run whose cleanup evicted nothing would otherwise digest identically to its
  clean twin — and because an unconditional new section would re-hash every golden in the corpus for
  nothing. The durable tripwire (`rul-durable-contents-reviewed-before-design`) was NOT fired and
  did not need to be. +SURE.
- **no recovery, no carves, no re-planning** — none built, none reachable.

## §2 — THE CENSUS FORK: it IS a lookup, and guards stand

The ruled conditional (`FORFEITS:forfeit-certifier-trip-demotes-guards`) asked whether the
plural-family census is trivially constructable at the cleanup seat. **It is.** The evidence:

1. `dorc_analysis::funcenv::DefinitionTable` already holds every role funcdef the run loaded, as
   `(file, name, span)` records — built by `dorc_cli::world::definition_table`, a plain syntactic
   walk over `dorc_syntax::parse` output for every input, book included.
2. The census is therefore `defs.iter().filter(|d| d.name == name).count()` — three lines, added as
   `DefinitionTable::occupancy`.
3. The table is IN SCOPE at the cleanup seat in both drivers: `run()` builds it at the funcenv seat
   and it lives for the whole function; `WhyWorld::analyze_measured` likewise.
4. A guard's verdict funcname is on the guard itself (`GuardInsert::fn_name()`), so the lookup key
   needs no derivation.

**Why occupancy-1 is the right conjunct, not merely a convenient one.** A guard is
`( check ) || <original bytes>`: the check re-verifies live on the host at apply time and the
author's bytes survive verbatim as the `||`-right, so every conjunct a guard rests on is re-measured
— except WHICH body the name resolves to, which the analysis chose. At occupancy 1 no choice was
made. The two residual failure modes are both safe: the positional gate withholds (⇒ no vouch ⇒ no
guard at all), or it names a definition the shell has not bound yet, in which case the check exits
non-zero (`command not found`) and falls through to the original command. At occupancy ≥2 a wrong
choice runs somebody ELSE's judgment and can answer 0 over a mutator that needed to run — the
under-execute direction, which is why that cell demotes.

**The admissibility argument, stated because it is the whole point:** the census consults NO solve.
A trip disqualifies the solver and the certifier together, so a census that itself depended on
either would be no census at all. This one depends on the parser. +SURE.

Two deliberate conservatisms, both in the over-execute direction:
- DECLARATIONS are counted, not distinct bodies. Content-dedup (two byte-identical vendored copies)
  would be sharper; the table holds spans, not bytes, so sharpening it would cost the lookup its
  triviality. ~SUSPECT this is the right trade for a policy whose charter word is "stupid".
- A family the BOOK also declares counts toward occupancy, so a book-shadowed oracle family's guards
  demote. That is the same direction.

**Conductor action:** the `forfeit-certifier-trip-demotes-guards` row's REVISIT condition ("remove
this row if the census proves a lookup") is MET. The wholesale branch is not dead code — it is what
a censusless caller gets, and `a_censusless_caller_demotes_guards_wholesale` pins that it stays safe.

## §3 — Posture by seat (`302` §4), as built

- value + funcenv: PRE-NETWORK. Latched before the probe is compiled; the existing tier-2 fail-fast
  report is unchanged, and the trip banner is a second, later line about the product.
- reaching-defs + self-reach: latched inside classify, in the origin round (pre-network) and in
  every fixpoint round (post-probe). Same latch, so a post-probe trip evicts the same way.
- `Mode::Probe`: returns before a plan exists, so no cleanup and no banner. Correct by the ruling's
  own words — a terminal pass cannot un-ship a probe; that is the §3 floors' job, and they fire.
  NOTED, not built around.
- apply-time: still vacuous as-built (the plan is frozen at consent).

## §4 — Tests

All outcomes come from the real `certify_solution` over a real perturbation (the F9-era pattern);
nothing hand-injects a verdict, and every fixture asserts its own non-vacuity before asserting
anything else.

`spike/crates/plan/src/certifier_trip.rs`:
- `a_real_trip_evicts_every_elision_family_outcome` — a REAL `Replace` (driven end-to-end through
  classify + `build_plan` over a converged, vouched `apt-get install`) plus an `Omit`, against a real
  trip: all demote, the step COUNT is unchanged (nothing is removed), and every record wears the
  reason arm.
- `a_census_unique_guard_stands_while_a_plural_one_demotes` — the fork, both cells in one test.
- `a_censusless_caller_demotes_guards_wholesale` — the forfeited branch stays reachable and safe.
- `a_passing_certification_never_latches` — the latch control.
- `a_later_consistent_answer_never_clears_the_latch` — monotonicity, both outcomes real.

`spike/crates/cli/src/world.rs`:
- `the_body_occupancy_census_decides_whether_a_guard_stands` — the fork over the REAL lookup: one
  oracle declaring the family vs. two, through `definition_table`.
- `a_trip_mints_one_spanless_banner_carrying_the_demoted_count` — the banner's structure: one diag,
  the right slug, spanless, count measured from the walk.
- `an_untripped_run_is_left_entirely_alone` — the SEAT control (same fixture, defect removed): no
  banner, no narrative, and a plural-census guard survives, so the trip is the whole trigger.

**DST: the hostsim cannot reach a trip, and I did not contort one.** The hostsim models the WORLD
(host state, seeded record-lane byte faults — `hostsim::fault` is a records vocabulary); a certifier
trip is an ENGINE defect, and no host state can make a correct solver return a non-post-fixpoint
answer. Reaching one from DST would need a solver fault-injection seam in the production kernel,
which is a second forgeable route to "broken solver" and a design act, not a lane item. The synthetic
perturbed-solve fixtures above are the same tier `302` §6 uses for the certifier itself.

## §5 — Gate evidence

| leg | result |
|---|---|
| `mise run check` (before each commit) | green; `check-quiet` prints 0 bytes on success |
| `mise run gate:full-quiet` (Windows) | 2068 tests run, 2068 passed, 1 skipped — 187s |
| `mise run both gate:full-quiet` (WSL leg) | 2064 tests run, 2064 passed, 1 skipped — 431s both-legs |
| `mise run bless:dry` | completed, **zero golden writes**; `git status` clean afterward |

**New goldens minted by this lane: ONE** — `spike/crates/aid/tests/solver-consistency-plan-demoted.loom`
(the new code's defining case) and its derived row in the generated
`spike/crates/aid/src/catalog_lock.rs`. No other golden moved a byte, which is the expected result:
the trip never fires in the corpus, so every existing case takes the untripped path.

## §6 — Comment budget

Counting commands, run over the lane diff `e5741a7e..HEAD` limited to `*.rs`:

```
git diff e5741a7e..HEAD -- '*.rs' | grep -c '^+[^+]'            # 818 added non-blank lines
git diff e5741a7e..HEAD -- '*.rs' | grep -cE '^\+\s*//[^/!]'    # 22 added plain // lines
git diff e5741a7e..HEAD -- '*.rs' | grep -cE '^\+\s*///'        # 150 added /// lines
git diff e5741a7e..HEAD -- '*.rs' | grep -cE '^\+\s*//!'        # 29 added //! lines
```

The two added test modules (`certifier_trip.rs:97-419`, `world.rs:758-942`) contribute 470 non-blank
lines, 0 plain `//`, and 43 `///`.

- **added non-test Rust lines: 348** (818 − 470)
- **plain `//` why-comments among them: 22 → 6.3%** (budget ≤10%) ✔
- mandated doc-comments, billed separately: **107 `///`** on non-test items, plus **29 `//!`** module
  docs (one new module header) — and **43 `///`** inside the test modules, which are the
  reasoned-argument-per-test the code style asks for.

## §7 — Flags, each phrased as its question

- **ask-remove-the-guard-forfeit-row** — the census proved a lookup (§2), so
  `FORFEITS:forfeit-certifier-trip-demotes-guards`'s REVISIT is discharged. I did not edit
  `FORFEITS.md` (the brief forbids it). Do you want the row removed at fold, or rewritten to record
  the two residual conservatisms (declaration-count rather than distinct-bytes; the book's own
  declaration counting) as a smaller forfeit?
- **ask-register-the-rule-in-the-crate-registries** — `spike/crates/{analysis,plan,cli}/CLAUDE.md`
  carry no bullet for this policy yet; `analysis/CLAUDE.md`'s "Law — the solve-certifier" section is
  the obvious home for the latch, and `plan/CLAUDE.md` for the cleanup's demote-only shape. Those
  files are conductor-managed, so I left them alone. Do you want me to draft the bullets, or will
  you site them at fold?
- **ask-is-a-sibling-code-right-here** — I minted `solver-consistency-plan-demoted` as a SIBLING of
  `solver-consistency-failure` rather than a reason arm of it. My argument: they differ by LICENSE
  variant, not grammar fit (`28L:rul-reason-enums-not-sibling-codes`'s sanctioned split) — the cousin
  is an engine event, one per failing pass; this is a product property, once per run, and a reader
  with only one of the two is missing something they need. ~SUSPECT this is right; it is the one
  place in the lane where I chose to grow the catalog rather than an enum, and the ruling's "never a
  sibling code" clause was about the per-demotion RECORD, which I did put on the existing enum. Is
  that reading of the clause the one you intended?
- **ask-should-probe-mode-carry-the-banner** — `dorc plan --probe`-shaped invocations return before a
  plan exists, so a trip evicts nothing and mints no banner there; the pre-network
  `solver-consistency-failure` line still fires. That matches the ruling (a terminal pass cannot
  un-ship a probe), but it means a probe-mode run tells the user a solve failed without telling them
  what it cost. Is that the resting state you want, or does the probe surface want its own line?
- **ask-out-param-versus-eighth-tuple-element** — I threaded the latch as a `&mut CertifierTrip`
  out-param on `classify_with_why_diags` (matching its existing `degrades` / `verdict_lane`
  out-params) rather than growing its 7-tuple return. The accumulator shape is what makes
  cross-round latching un-forgettable; the cost is one argument at nine call sites, and two lint
  suppressions it pushed over their caps (`settle_validity_fixpoint`'s `too_many_arguments`,
  `sweep::run_kernel`'s `too_many_lines` — both carry reasons naming this lane). Would you rather
  the return-tuple shape, or are the two `#[expect]`s acceptable?
- **note-no-flags-from-the-quarantine-lane** — the builders-only invariants were read first and
  applied; nothing in this lane's construction conflicted with them, and no stop was reached.
