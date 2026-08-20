# 30Ka — Effective world reach: the as-built lane record

> Tier: r30 builder lane report for the `30K` conversion. `notes/30K` is the work order and
> stays ahistorical; THIS document is the as-built record — seats, deviations left OPEN,
> findings, next steps. Root docs and `spike/CLAUDE.md` outrank it. Grades: **[+SURE]**
> measured · **[~SUSPECT]** reasoned but unmeasured · **[-GUESS]**.
>
> Docid steer: this lane's report is `30Ka`, not a major docID (human, direct, mid-lane).

## §1 — `step-1-map-effective-invalidator-ownership`: the census

The scratch census the work order asks for, kept here because its OWNERSHIP half became a
built artifact (`cfg::ExecutionOwner`) rather than a throwaway list.

### 1a — Producers of the split answer, as found

| seat | what it answered | fate |
|---|---|---|
| `analysis::effect::Reach` (`Facts(BTreeSet<FactKey>) \| Top(ProvId)`) | per-cell "was my cell written upstream" | RETAINED, origin/probe only |
| `Reach::is_pristine` | the `QueryResolvable.valid` bit | retained for the FROZEN probe; retired as apply authority |
| `SkipClass::{EstablishAmbient,EstablishWritten}` | the elide/guard discriminator | RENAMED `EstablishProbe{Ambient,Written}`; no longer selects a tier |
| `ReplaceLicense::prove_replaceable(class, …)` | the "ambient gate" | takes a `FactKey`; freshness is the caller's conjunct |
| `plan::wall_walk_total` | flag-off total wall, `Replace -> Run` | DELETED |
| `plan::wall_walk_survival` | flag-on scoped wall + `SurvivalWitness` mint + re-derivation | DELETED; its body re-seats in `world::WallPolicy::freshness` |
| `build_plan_walled`'s `is_mutator` side channel | `class_is_establish_bearing \|\| kills.contains` | DELETED (the predicate too) |
| `cli::fixpoint::settle_validity_fixpoint` | the W-C erasure rounds | REPLACED by the one grow-only settlement |
| `cli::fixpoint::attribute_cascades` | which erasures revalidated a Query | narrowed + renamed `attribute_dead_branch_cascades` |

### 1b — The effective-invalidator population, by shape, with its explicit owner

`invalidators` is every CFG node whose post-erasure effect vector contains
`Establishes`/`Kills`/`Opaque` (`effect::gens_into_reach`). Enumerated from
`effect::node_effects`, which is total over `CfgNodeKind`:

| invalidator shape | plan leaf? | execution owner | why |
|---|---|---|---|
| `Command`, ordinary leaf | yes | `Leaf(self)` | its own decision governs it |
| `Command`, expansion-internal (`$( … )` body) | no | `Leaf(enclosing simple)` | the enclosing leaf's span carries the substitution; a replaced leaf takes it with it |
| `Command`, spliced-internal (per-call body copy) | no | `Leaf(call)` | the CALL is the render unit (`i-3`); an all-or-nothing call replacement neutralises the whole body |
| `Command`, detached funcdef body | no | `Leaf(self)`, never a plan leaf ⇒ never retired | unreachable from entry, so its gen reaches only its own island — exactly `Reach`'s own answer |
| `Command`, Members site | yes | `Leaf(self)` | one leaf, N member cells |
| `Redir` (write-shaped, under a `Simple`) | no | `Leaf(enclosing simple)` | the redirect is inside the leaf's byte span and is span-elided with it |
| `Redir` (group/subshell-level, `attach_redirs`) | no | `AlwaysAtNode` | no leaf span covers it, so no leaf decision can remove it |
| `Top` (unmodeled construct) | no | `AlwaysAtNode` | nothing decides it away |

Ownership is RECORDED at lowering (`cfg::Builder`), never re-derived from spans or
adjacency: `lower_simple` claims every node it allocates between its entry and its command
node; `splice_funcdef_body` overwrites its whole spliced range with the CALL, so a nested
splice resolves to the OUTERMOST call in one step and no consumer walks a chain. The
DEFAULT is `AlwaysAtNode` — the total-running floor — so a new node kind walls until
someone decides otherwise (`30K` §3.7: an ownerless invalidator never guesses).

## §2 — As-built seats

- `analysis::cfg::ExecutionOwner` + `Cfg::execution_owner` — the ownership census, built.
- `analysis::effect::Classification` — `classify` now answers with the invalidator set
  beside the classes, because a caller cannot re-derive it (non-leaf invalidators are
  invisible in `SkipClass`) and a plan built without it elides past real mutations.
- `plan::world` — `WallId`, `ReachingWalls` (`Powerset<WallId>`), `EffectiveAct`,
  `NoMutationProof`, `ReplacementDeathProof`, `NoExecutionLedger`, `Quiescence`,
  `WallPolicy`, `Freshness`/`StaleCause`, `effective_invalidators`,
  `solve_reaching_walls`, and `WallPolicy::freshness` (the survival tier's new seat).
- `plan::settle` — `RoundModel`, `SettleInputs`, `RoundClassification`,
  `ProvisionalEffectiveRound` (no Spine API), `SettledEffectiveAnalysis` (private
  constructor; takes a `Quiescence`), `settle_effective_world` (the one grow-only loop),
  `floor_uncertified`, `replacement_death`, and `write_spine`.
- `plan::decide_site` + `DecideSite`/`SiteDecision` — the ONE seat that mints a
  disposition and its semantic act together.
- `plan::attach_spine_probe_provenance` — post-settlement, over the SETTLED spine, so no
  round can reach the arena (and therefore no round can reach an output surface).
- `plan::build_plan` / `build_plan_walled` — thin drivers over one settlement with a
  constant classification; `cli::fixpoint::settle_world` supplies the reclassifying model.
- `aid::diag::SolvePass::EffectiveReach` — the effective solve reports under its own name.

### 2a — The domain, and why it needed no new verified-core surface

`ReachingWalls` is `dorc_analysis::lattice::Powerset<WallId>` — the EXISTING algebra, with
a plan-local element type. No new `Lattice` impl, no new law harness, no minispec or Kani
change, and none owed by analogy: the join/meet/⊥ laws in play are `Powerset`'s, already
pinned. `WallId` is minted in `plan::world` and nowhere else, which is what keeps the two
reach species un-interchangeable without a trait to erase the distinction.

## §3 — Deviations from `30K`, left OPEN for the conductor

Recorded as taken, never self-endorsed.

- `dev-replacement-death-does-not-erase-effects` — `30K` §3.5 asks for ONE `ErasedSites`
  overlay carrying both proof species at the analysis effect seam. Built otherwise: the
  DeadBranch species alone reaches that seam; the Replaced species suppresses the site's
  wall GEN and nothing else. Reason: the effect seam spells erasure as
  `CommandEffect::Pure`, which also destroys the site's own `SkipClass` — so a replaced
  site would classify `MustRun` on the next round, lose the license that replaced it, and
  settle as `Run` while every downstream decision had already been taken as if it would
  not execute. That is a wrong elision, not a precision loss. Both species still live in
  ONE `NoExecutionLedger` and one gen-suppression rule
  (`NoExecutionLedger::proves_no_execution`); the divergence is only in which consumer
  each species reaches.
- `dev-backings-ride-beside-the-policy` — `30K` §3.3 lists `fact_backings` among
  `RiskAccepted`'s fields. Built beside it instead: the three AUTHORITIES (footprints,
  resolutions, dialect) are frozen and inhabit the closed sum, while backings are derived
  per round from the residual model. Putting a round-derived value inside the authority
  type would have made the policy pretend to be frozen when it is not; the
  cannot-construct-without-every-authority property is unaffected.
- `dev-aggregates-take-one-position-freshness` — `InlineCall`/`EstablishMembers` consume
  the reaching walls at the aggregate's OWN node (Members through a self-suppressed
  re-solve, mirroring `effect::self_reach_holds`), and keep every one of today's
  per-body-site conditions unchanged. `30K` §5.4's universal per-erased-establish effective
  freshness is NOT built; the aggregate takes the conservative single position, and where
  the representation cannot express the universal statement the whole aggregate runs.
- `dev-wall-formation-account-stays-flag-gated` — `30K` §7 lists a wall-formation account
  per effective mutation act among the required final accounts. Built as it was: minted
  only under the risk-accepted policy. Nothing consumes the record yet
  (`289:seam-narrative-render-unconsumed`), so widening it to the honest path buys no
  account and costs every why-transcript an `[unnarrated: WallFormation]` line — measured:
  it moved `whygallery-decline-unsound-arm`, `why-reason-render-refused`, and
  `why-analysis-opener-guarded`. It should widen with its consumer, not ahead of one.
- `dev-replacement-death-cascades-are-unattributed` — a Query can now become valid because
  an upstream mutation was ELIDED, not only because a branch was proven dead. That cascade
  has no controller line, so `CascadeAttribution` cannot carry it; the function is narrowed
  and RENAMED `attribute_dead_branch_cascades` rather than broadened past what its operands
  can say (`30K` §8 step-3's "do not broaden the old function"). The elision-cascade chain
  needs a render shape that does not exist, and inventing one is prose work.
- `dev-effective-reach-component-prose-unminted` — `SolvePass::EffectiveReach` exists and
  the cli reports under it, but its arrangement component
  (`solver-consistency-failure-effective-reach`) is NOT in the lock: `dorc-loom publish`
  refuses an `owns:` slug no case renders, and the defining case is fixture-routed to ONE
  pass. Nothing in the corpus triggers it, so the resting state is honest; minting the
  component needs a fixture that renders the new pass, which is authoring-surface work.

## §4 — Findings

- `fnd-the-cap-was-never-the-leaf-count` **[+SURE]** — the retired fixpoint capped at
  `classes.len()`. The settlement's ledger holds CFG SITES (leaves AND non-leaves) and
  grows by at least one per non-quiescent round, so the bound is the node count plus the
  one settling round. The leaf count is both too small and one short; it tripped the
  monotonicity `debug_assert` on three corpus books before the bound was corrected. The
  retired loop was safe only because it never had non-leaf sites to prove.
- `fnd-positional-class-indexing-was-a-latent-mis-key` **[+SURE]** — the retired
  `validity_view`/`attribute_cascades` indexed `classes` POSITIONALLY and called the index
  a `LeafId`. Leaf ids are assigned in SPAN order (`site_order`); classes arrive in CFG
  ALLOCATION order. The two coincide for straight-line books, which is why the corpus never
  caught it. Both seats now go through the new `plan::leaf_ids`.
- `fnd-non-leaf-invalidators-were-invisible-to-every-non-cli-driver` **[+SURE]** —
  `build_plan`/`build_plan_walled` took only `classes`, so a caller could not have supplied
  the non-leaf invalidators even in principle. Under the retired split this was masked by
  origin `Reach` doing the per-cell invalidation; with effective reach as the sole
  authority it would have been a wrong elision, which is why `classify` now answers with
  the set and both entries demand it.
- `fnd-the-guard-tier-repair-is-broader-than-the-guard26-pair` **[+SURE]** — the same
  repair moved five `strawman24-*`/`pin28-*` survival cases from a bare RUN to a GUARD. The
  defect was never specific to classed declines: any site whose ONLY lost elision
  precondition was freshness used to run bare.
- `fnd-fail-fast-hid-four-real-failures-behind-the-golden-drift` **[+SURE]** — with eleven
  e2e cases red, nextest's fail-fast stopped `gate:full-quiet` at 611 of 2368 trials, so the
  whole unit tier went unrun and read as "green except the drift". It was not. Running
  `gate:quick` (lint + unit tier, no corpora) surfaced FOUR genuine failures the composite
  gate could not have shown while any case was red: both re-homed lexical fences, the lost
  cap-degrade narrative, and one behaviour move. **A conversion whose goldens are pending a
  bless MUST be gated tier-by-tier — `gate:quick`, `test:looms-quiet`, `test:e2e-quiet` —
  and never by the composite alone.**
- `fnd-a-lexical-fence-can-find-itself` **[+SURE]** — `licence_mint_has_exactly_one_caller`
  scans the workspace for a literal needle. Once the mint's sole caller moved out of the
  file holding the fence, the ONLY remaining match in that file was the fence's own needle
  string, and the census read 2. Both fences (this one and the new
  `replacement_death_mint_has_exactly_one_caller`) now spell the needle as `concat!(…)`, so
  the scan cannot match itself. A fence whose subject moves needs re-aiming AND re-checking
  that it still measures what it names.
- `fnd-the-funcenv-fence-stopped-covering-the-loop-body` **[+SURE]** — the fence scanned the
  driver FUNCTION body for env entry points. Under the settlement the real per-round work
  moved into `WorldRoundModel::classify`, outside that slice, so the fence would have been
  silently vacuous for the property it exists to hold. It now scans both regions.
- `fnd-a-notEffective-act-cannot-be-mutated-into-a-wall-removal` **[+SURE]**, methodology —
  the first attempt at the guard-walls-downstream mutation used
  `NoMutationProof::NotEffective`, which `record_round` deliberately drops, so the mutation
  was a no-op and the pin passed vacuously. The pin was ALSO passing for an unrelated reason
  (the running wall reached the third site directly). Both were repaired: the book now
  isolates the guard as the only possible wall, and the mutation mints a real proof. This is
  the second specimen in this lane found passing for the wrong reason.

## §5 — Next steps / residue

1. **The goldens.** ELEVEN cases move (§6) — all the repair or its direct consequence, none
   refactor churn. `mise run bless` is the conductor's; `bless:dry` names the exact scoped
   form (an unfiltered bless verifies before it writes, so it can never accept a sanctioned
   drift):

   ```
   mise run bless -- crates/cli/tests/exec-subst-body-nonleaf \
     crates/cli/tests/frame30-nested-region-inherits-the-outer-body.loom \
     crates/cli/tests/frame30-subshell-body-answers-inside-only.loom \
     crates/cli/tests/pin28-reach-arm-death-walls-total.loom \
     crates/cli/tests/pin28-survival-body-death-walls-total.loom \
     crates/cli/tests/strawman24-alias-provides \
     crates/cli/tests/strawman24-alias-symlink \
     crates/cli/tests/strawman24-reach-crossauthor \
     crates/cli/tests/strawman24-reach-static-service \
     crates/cli/tests/strawman24-survive-multiwall \
     crates/cli/tests/whygallery-webhost-whole.loom
   ```
2. **The three `guard26-*` books carry stale prose.** The two promoted cases still describe
   themselves as XFAIL-until-the-ruling, and the control still names the retired defect
   twin. Their headers were deliberately NOT rewritten: a book byte-change moves the
   embedded book text AND its content digest in the golden, which would have forced a bless
   mid-window. The prose refresh should ride the bless.
3. **Steering-law text is proposed, not applied** (§7) — `spike/CLAUDE.md`, `analysis`,
   `plan`, and `cli` crate `CLAUDE.md`s, plus `FORFEITS`/`ANALYZER-NEEDS` rows.
4. `Research/trial/r26/predictions.md` §7's closing paragraph names the retired defect twin
   as a live pin. It is a PRE-REGISTERED ledger whose own rule is that it is never edited to
   match reality, so it was left alone; the conductor may want a superseded-marker.
5. The residue `30K` §10 named and this lane did not touch: no Query footprint-sparing, no
   at-most completion speech, no durable growth, no `28Q:stage-iii-world-scopes`.

## §6 — Behaviour drift, enumerated

Every moved case, with the mechanism that moved it. Reviewed as behaviour before any bless.

**A. The repair itself — a site below a modeled running wall reaches the GUARD rung**
(was: bare `Run`). Ten cases:

| case | movement |
|---|---|
| `dorc-plan tests::fixture_install_on_realistic_book_still_runs_residual_poison` | run ⇒ guard on the realistic `pi-webhost` book; the datum (no elision) is unchanged, the honesty improved. Assertion updated in place |
| `guard26-classed-decline-guards-below` | XFAIL ⇒ XPASS, promoted (marker + `head-expected.ran` removed) |
| `guard26-diverged-wall-guards-below` | XFAIL ⇒ XPASS, promoted |
| `guard26-classed-decline-demotes-guard` | the defect twin — RETIRED (deleted); its lesson is now `a_modeled_running_wall_leaves_the_guard_tier_reachable_below_it` plus the promoted twin |
| `pin28-survival-body-death-walls-total` | `apt-get install -y nginx` run ⇒ guard |
| `pin28-reach-arm-death-walls-total` | content diff: the guard preamble arrives |
| `strawman24-alias-provides` | `apt-get install nginx-full` run ⇒ guard |
| `strawman24-alias-symlink` | same shape |
| `strawman24-survive-multiwall` | `apt-get install -y curl` run ⇒ guard |
| `strawman24-reach-crossauthor` | content diff: the guard preamble arrives |
| `strawman24-reach-static-service` | content diff: the guard preamble arrives |

**B. An elided upstream mutation retires its wall, so the site below it elides too**
(was: guard). Two cases — the `28Q` §1 frame goldens, where site 0 elides inside a subshell
region and site 1 measured the same cell:

| case | movement |
|---|---|
| `frame30-subshell-body-answers-inside-only` | site 1 guard ⇒ elide; `expected.ran` empties |
| `frame30-nested-region-inherits-the-outer-body` | same |

**C. A non-leaf mutation now walls through its owner** (was: invisible to the walk). One
case, and it is a closed wrong-elision:

| case | movement |
|---|---|
| `exec-subst-body-nonleaf` | `apt-get install -y curl` elide ⇒ guard — the `$(apt-get install -y nginx)` inside the `echo` REALLY RUNS, and the retired walk could not see it |

**D. Downstream of A/B/C.** One why-transcript re-renders because the dispositions it
explains moved: `whygallery-webhost-whole`.

Nothing outside these four classes moved. **[+SURE]** — measured over the full 168-case
e2e corpus and the 279-case loom corpus, identically on BOTH platform legs (the two
`gate:full-quiet` runs name the same eleven cases and differ in nothing else).

An earlier cut of this conversion also moved `whygallery-decline-unsound-arm`,
`why-reason-render-refused`, and `why-analysis-opener-guarded` by minting the
wall-formation account on the honest path. That is `dev-wall-formation-account-stays-flag-gated`:
the mint was scoped back to the flagged path and those three cases are byte-identical again.

## §7 — Proposed steering prose (text only; the conductor applies)

**`spike/crates/analysis/CLAUDE.md`**, replacing `origin-reach-is-never-final-freshness`
and `effective-reach-consumes-semantic-acts` with their as-built form:

> - **origin-reach-is-probe-only** (`30K`, BUILT) — `Reach`, `Reach::is_pristine`, and
>   `SkipClass::EstablishProbe{Ambient,Written}` answer ONE question: which check may ship,
>   and which cell the authored model names. They carry no apply-time authority and their
>   names say so. Apply-time freshness, effective Query validity, total walls, and
>   footprint survival read `plan::world::ReachingWalls` and nothing else. Never make the
>   two species generic behind one trait, and never convert effective reach back into probe
>   eligibility.
> - **classify-answers-with-its-invalidators** (`30K` §3.7) — `classify` returns
>   `Classification { value, diags, invalidators }` because the invalidator set is NOT
>   derivable from the classes: a `$( … )` body command, a write-shaped redirection, and an
>   unmodeled construct all gen into the world without being leaves. A caller that drops it
>   elides past a mutation nothing in its inputs can see. `cfg::ExecutionOwner` says whose
>   decision governs each of them, recorded at lowering and never re-derived.

**`spike/crates/plan/CLAUDE.md`**, replacing `effective-reach-replaces-wall-walks`:

> - **one-settlement-one-world** (`30K`, BUILT) — every apply-time answer derives from one
>   grow-only settlement (`plan::settle`) over one fact: which mutations may ACTUALLY
>   execute. A round applies the ledger, re-derives the model, solves `ReachingWalls`, folds
>   the frozen records through the validity that reach implies, decides every site, and
>   proves what cannot execute; a growing round discards every provisional product. Only the
>   quiescent round — sealed by a `Quiescence` the ledger alone mints — writes Spine, and a
>   `ProvisionalEffectiveRound` has no Spine API to reach for. Never add a second settlement.
> - **acts-and-dispositions-mint-together** — `decide_site` returns BOTH the `Disposition`
>   and the private `EffectiveAct`, from one pass over one set of conditions. There is no
>   `From<Disposition> for EffectiveAct` and there must never be one: the act is the other
>   half of the conclusion, not a reading of the outcome (`pin-no-outcome-as-generator`).
> - **only-a-proof-retires-a-wall** — a `Guard` walls exactly like a `Run` (its untouched
>   fallback is the authored mutation), and a `Replace` the RENDER will refuse walls too. A
>   wall is retired ONLY by a `NoMutationProof` in the ledger, and the two species reach
>   different consumers on purpose: a DeadBranch shrinks the analyzer's effect model, a
>   Replaced one must not (that spelling would also destroy the site's own class).

**`spike/crates/cli/CLAUDE.md`**, replacing
`the-fixpoint-owns-the-rounds-and-builds-nothing-else`'s W-C framing: the frozen set and
the never-survives property are unchanged; what moves is that the loop is now
`plan::settle_effective_world` driven by `fixpoint::WorldRoundModel`, and the settled round
writes Spine itself rather than a later `build_plan_walled` call doing it. Also: the
survival-tier lift (footprints, resolvers, dialect) now runs BEFORE the settlement, from
the ORIGIN classification, because the wall policy is one of the settlement's frozen
authorities. Erasure only removes sites, so the origin-lifted footprint set is a superset
whose extra entries belong to sites that gen no wall and are never looked up.

**`FORFEITS.md`** — one new row proposed:

> `forfeit-aggregate-single-position-freshness` — an inline call and a member loop take the
> reaching walls at the aggregate's OWN position rather than proving effective freshness per
> erased establish (`30K` §5.4). Conservative: where the aggregate's position is stale the
> whole aggregate runs. REVISIT when a typed per-member effective-freshness proof exists.

**`ANALYZER-NEEDS.md`** — `an-reaching-ambient` / `an-written-stale` / `an-wall-topology`
should read as DISCHARGED by `28Q:stage-effective-world-reach`, pointing at
`plan::world`/`plan::settle` rather than describing the three-mechanism split.

## §8 — Evidence

- Mutation results, all eight run and all eight reddening the named pin:

| pin | mechanism mutated away | result |
|---|---|---|
| `a_modeled_running_wall_leaves_the_guard_tier_reachable_below_it` | the Stale arm's guard mint | RED |
| `an_elided_upstream_mutation_removes_its_own_wall` | `Replace` ⇒ always `may_mutate` | RED |
| `a_guard_is_the_only_wall_the_third_site_can_be_stale_from` | `Guard` ⇒ a real no-execution proof | RED |
| `an_expansion_internal_mutation_walls_through_its_owner` | non-leaf invalidators dropped from the gen set | RED |
| `a_render_refused_replacement_never_retires_its_wall` | `replacement_renders_dead` ⇒ always true | RED |
| `a_colliding_footprint_demotes_to_the_guard_tier_not_to_a_bare_run` | the Poisoned arm ⇒ `FreshClean` | RED |
| `a_live_fallback_keeps_the_rung_below_it_invalid` | Query validity ignores walls | RED |
| `an_uncertified_effective_answer_makes_every_fact_stale` | `floor_uncertified` ⇒ passthrough | RED |

- Comment budget: 177 non-doc comment lines added over 3015 added lines (5.9%); doc
  comments billed separately at 550. Command:
  `git diff -U0 <base>..HEAD -- 'spike/**/*.rs' | grep -c '^+[[:space:]]*// '`.
- Gates, run TIER BY TIER on both legs because the composite fail-fasts past the unit tier
  while any case is red (`fnd-fail-fast-hid-four-real-failures-behind-the-golden-drift`).
  Windows first, then WSL, per `preflight-bounds-before-spend`:

| tier | Windows | WSL |
|---|---|---|
| `gate:quick` (four lint gates + unit tier) | 1455/1455 GREEN | 1451/1451 GREEN |
| `test:looms-quiet` | 279/279 GREEN | 279/279 GREEN |
| `test:e2e-quiet` | 156 pass, the 11 of §6 | 156 pass, the SAME 11 |

  The 1455/1451 delta is the `cfg`-gated platform tests, not a divergence: the two legs
  name the same eleven e2e cases and differ in nothing else. `test:floor` and
  `test:real-tools` re-run the same e2e corpus under extra env, so they fail on the same
  eleven; `bless:dry` refuses for the same reason and prints the scoped bless above.
  `verify:check` rides `gate:full-quiet` and is green on both.
