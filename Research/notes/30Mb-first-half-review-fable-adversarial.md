# 30Mb — Round-30 adversarial review (Fable, whole-range)

> Tier: commissioned adversarial review of `68709783..3011daae` (563 commits; the r30
> kernel rework and its surround), run 2026-08-21 against the tip, in the commissioned
> review worktree on this note's namesake branch. Method: root docs → the r30 plan/ledger corpus →
> direct code reading of the license-minting seats (`plan::{world,settle,lib,
> certifier_trip,survival,rederive}`, `analysis::{solve,certify,cfg,effect}` heads, the
> cli drivers), with two scout passes (code map; ledger/ratification digest) whose
> load-bearing claims I re-verified by hand. `mise run gate:full-quiet` green on the
> Windows leg (exit 0) before any reading. Project `CLAUDE.md`s were treated as CLAIMS
> to verify, not law (they are themselves r30 artifacts). Out of scope per commission:
> WSL/Lean/Kani tiers, the minispec/dorc-verify instrument standup, security,
> re-deriving `30La`, and the stage-i definition-factoring conversion (already
> four-lineage crosschecked).
>
> Grades: **+SURE** verified in code/corpus by me · **~SUSPECT** reasoned, not
> demonstrated by an executed specimen · **-GUESS** stated so it can be checked.
>
> Headline: I found no live wrongly-minted elision license at the tip. The one
> license-soundness defect I can demonstrate structurally (fnd-members-floor below) is
> currently unreachable and double-contained. The arc's real exposure is quieter:
> a cluster of conductor-tier deviations around narration/steering that contradict
> welded law and were never human-ratified, and a landed core whose paper trail
> (steering files, FORFEITS, one loom header) now describes the machine it replaced.

## §0 — Verdict in one screen

- **+SURE** The effective-world-reach conversion (`30K`→`30Ka`→`30Kb`→repairs) is, at
  the tip, the most heavily defended kernel surface in the tree: private-mint
  licenses consumed by value, a grow-only proof ledger with a typed
  provisional/settled boundary, a certified solve at every seat, member-by-member
  aggregate freshness with demote-only reference re-derivation, and a terminal
  cross-window trip cleanup. The `30Kb` criticals (aggregate single-representative
  survival; inline-owner wall persistence; disposition-read acts) are genuinely
  repaired in code, not just recorded — I traced each repair to its seat and its
  red-verified mutation pin.
- **+SURE** The known defect `30La:rul-predict-measured-aggregates-are-a-bug` is
  correctly quarantined at the tip: the `no_verdict_lane_in_members` seat still
  stands (`analysis/src/effect.rs:747`), both `aggregate30-*` XFAILs are present and
  target the right transcript, `30L` stage-0 gates on the repair, and the steering
  law names the defect in place. Nothing consumes the interim.
- The findings below are ordered by (design-weight × wrongness). Nothing here is a
  stop-the-arc item; two are repair-before-next-consumer items.

## §1 — fnd-members-floor-is-a-sentinel-not-a-type

**+SURE (structural) / -GUESS (any live path today).** The one place the
solve-certifier floor is spelled as *data* instead of a *typed cause*, and it is
bypassable in principle.

`30K` §4.4 and `302` §3 (`rul-whole-window-demotion`, TYPED substance) require that an
`Inconsistent` effective answer be inadmissible for freshness or survival — "no
Replace/Survive/Omit minted from that answer." The standalone path honors this with a
typed, unbypassable floor: `plan::settle::floor_uncertified` returns
`Freshness::Stale(StaleCause::SolveInconsistent)` (settle.rs:560–569), pinned by
`an_uncertified_effective_answer_makes_every_fact_stale`.

The Members path does not. When a members-site's self-suppressed solo solve fails
certification, the code substitutes a *sentinel wall* — the site's own node
(`settle.rs:415–423`: `ReachingWalls::singleton(WallId::of(*node))`, comment "hand the
site a wall it cannot resolve"). But whether a wall "cannot resolve" is decided by the
*consumer*: `WallPolicy::freshness` (world.rs:404–431) resolves any wall for which
`footprints.get(wall.node())` and `leaf_of.get(...)` both answer. The members site IS a
plan leaf, so the sentinel's unresolvability rests entirely on the accident that the
footprint lift currently excludes aggregate classes
(`cli/src/survival.rs:105–113` matches only `EstablishProbe{Ambient,Written}` and
kills). The moment the lift widens to aggregate at-most claims — which the member-lane
direction (`30L` §4.2, the propagation lane, `30La`'s repair territory) plausibly
brings — a `FreshSurvived` can mint from an answer the certifier rejected, and the
reference re-derivation will *confirm* it (it re-checks the sparing algebra over the
same accumulated walls, not the wall's provenance).

Containment, verified: (a) under `WallPolicy::Honest` the sentinel lands
`Stale(TotalWall)` — safe; (b) today's footprint lift cannot populate the sentinel
node; (c) the solo inconsistency is latched into the run-wide `CertifierTrip`
(settle.rs:413), so the terminal `demote_on_trip` evicts every `Replace`/`Omit` at
plan-emission anyway. So there is no live under-execution — this is defense-in-depth
erosion at exactly the boundary `302` says must not depend on downstream nets ("the
consumer floors are UNCHANGED and still fire in place").

Two adjacent wrongnesses ride it:

- **Attribution untruth (+SURE):** the demotion narrates as `TotalWall`, not
  `SolveInconsistent` — the admin is told "a wall stands" when the truth is "our
  solver failed its own check." `302` §5's admin-facing honesty ("this is OUR defect,
  not the book's") is exactly inverted for this path. On the `271:rul-sin-ordering`
  scale this is narration-tier mis-attribution, not license-tier — but it is the
  worst *class* on that scale, minted knowingly cheap.
- **Test gap (+SURE):** no test covers the members-solo-inconsistent path.
  `settle.rs:422` has zero test callers; the `30Ka` §8 pin battery's "an uncertified
  effective answer takes the named floor" covers the standalone seat only.

Repair shape (small): thread the solo `SolveConsistency` into the members freshness
the way the main one already threads (`floor_uncertified`-style, typed
`StaleCause::SolveInconsistent`), delete the sentinel, add the missing pin. ~SUSPECT
one sitting of work; no schema movement.

## §2 — fnd-wall-narration-deviation-contradicts-welded-law

**+SURE (state) / design-weight, unratified.** `30Ka:dev-wall-formation-account-stays-flag-gated`
is still in force at the tip: the `WallFormation` narrative is minted only under
`WallPolicy::RiskAccepted`, and only for *leaf* decisions (settle.rs:458, 517–519;
`write_spine` at 205–229). Honest-mode runs — the default posture — and every
non-leaf wall (command-substitution internals, group-level redirections, unmodeled
constructs) form decision-bearing walls that leave no narrative record at all.

Why this is a finding and not a footnote:

1. Sol REJECTED it twice, with reasons grounded in *human-welded* law
   (`30Kb` §3, §8: "semantic mints must not depend on current render consumption or
   the risk flag"). The controlling welds are `KNOBS:kWARN` (kWARN-rich, human-welded,
   spike-scoped: build the detection now, tune the surface later — "the detectable
   moment is during construction") and `AID-NEEDS:law-collapse-mints-narrative` (the
   mint is demanded at the safety-narrowing, at the value level). The deviation's
   stated justification — three why-transcripts gain `[unnarrated: WallFormation]`
   lines — is precisely the cost the kWARN weld says to pay, and golden churn is
   ruled a non-blocker besides.
2. The conductor downgraded a Sol "product architecture blocker" to "non-blocking
   residue" (`LIVING_STATUS` 2026-08-20 block) with no [TYPED]/[ACKED] mark anywhere
   in `300`/`307`/`30Ka` for that downgrade. Under the silence-is-not-ack fence this
   is an open conductor-tier call sitting directly on a welded knob.
3. The discharge rider is ambiguous about half the gap:
   `30L:req-wall-narrative-gains-region-operand` says it closes "the honest/non-leaf
   narration gap," but the mechanism it specifies is only the *non-LeafId operand*
   (`30Kb:finding-nonleaf-walls-have-no-account-seat`); removing the *risk-flag gate*
   on the mint is implied by the sentence and spelled by nothing. A `30L` builder
   implementing the rider literally leaves honest-mode wall formation minting nothing.
   **~SUSPECT** that is accidental scope-narrowing in the residue hand-off, not a
   decision anyone made; one sentence in the `30L` stage-4 brief fixes it.

Consequence if left: whylog replay for every honest-mode run (the default!) permanently
lacks the wall-formation species, and the eventual why/report surfaces will be designed
against a record population that silently excludes the product's most common
configuration. Cheap now, retrofit-annoying after `30I` artifact forms freeze durables.

## §3 — fnd-paper-trail-describes-the-deleted-machine

**+SURE.** Three registries at the tip describe the pre-`30K` world, in a project whose
own law makes registry currency load-bearing (successor agents auto-load these files;
AGENTS.md: "always current, and correct").

- `FORFEITS:forfeit-guard-tier-classed-decline` still records the guard-tier loss as a
  live forfeit ("the late wall walks demote Replace directly to Run"; "REVISIT: the
  `30K` implementation fold"). The fold happened; the guard26 pair is promoted; the
  walks are deleted. The register's own discipline — "a captured forfeit's row is
  rewritten or removed, never annotated as history" — is violated by its most
  prominent row.
- `30Ka` §7's as-built steering bullets were never applied:
  `analysis/CLAUDE.md` still carries `origin-reach-is-never-final-freshness` /
  `effective-reach-consumes-semantic-acts` in their pre-conversion, future-tense form,
  and `plan/CLAUDE.md` still carries `effective-reach-replaces-wall-walks` under
  "Direction" instructing a deletion already done. This is
  `30Kb:finding-steering-law-lags-the-code`, standing at the tip. (`cli/CLAUDE.md`
  WAS updated — the lag is partial, which makes it more misleading, not less.)
- `plan/src/lib.rs:710` — `prove_inline_replaceable`'s doc-comment still lists
  `EstablishProbeWritten` among the blockers ("stale resting probe") while the code at
  :743 deliberately accepts it (freshness moved to the caller's conjunct, correctly,
  per `30Ka`'s class rename). A maintainer trusting the comment re-introduces the
  origin-reach-as-freshness split the conversion existed to kill.

Footnote-tier, same cluster: `trial/r26/predictions.md` §7 still names the retired
defect twin as a live pin (its no-edit rule makes a superseded-marker optional; `30Ka`
§5.4 asked for one; none landed). The `guard26-*` book prose, by contrast, WAS
refreshed with the bless — that residue discharged properly.

## §4 — fnd-green-pin-with-inverted-prose (tests lens)

**+SURE.** `spike/crates/cli/tests/pin28-variable-resolved-source-loads.loom` is the
concrete specimen of the rot-while-green failure the commission asked about: a ~40-line
header arguing "parity IS the ruling — the variable spelling loads fine, exactly as a
literal path," pinned green, while the landed `30I` loader changed the resolution rule
underneath it (`30Ib:dev-slash-less-dot-is-now-unresolvable`, recorded OPEN by the
builder: "now pins the OPPOSITE of its own header prose with green goldens"). The
XFAIL lens is golden-text-blind by design, and a round-trip case's prose is asserted by
nothing — so this case will stay green under arbitrary future meaning-drift. The
builder flagged it; nobody has re-authored the case. It should be re-headed (or
re-shaped to assert what it now proves) before the `30I` XFAIL-promotion step blesses
the corpus wholesale.

Where the tests DO pin what they claim (checked, worth saying): I verified in-tree
that the aggregate member-by-member demotions, the render-refused-replacement wall
retention, and the ledger's Replaced-never-reaches-classify-overlay each have an
adversarial test targeting the right mechanism (`30Ka` §8's table records each one
verified red by mutation; that half is the lane's record, not re-run here);
the two lexical fences that had gone vacuous (self-matching needle; fence scanning a
function the work moved out of) were both re-aimed in `e4ce763a`; the xfail census is
healthy (6 live pins, round-marker horizons, none expired). The
`fnd-fail-fast-hid-four-real-failures` lesson from `30Ka` is minted as durable law.

## §5 — Second lens: unratified decisions with design weight

The calibration: [TYPED]/[ACKED] items are off-limits as decisions; what follows is the
subset of conductor/builder-made calls that carry product-level consequence, weighted
by wrongness. (The full ratification inventory is large and mostly fine; this is not
it.)

- **dec-spine-reification-under-delivers-its-own-boundary (~SUSPECT on consequence).**
  `plans/309`'s plan was human-acked ("reviewed-matches-intent, BUILD OPEN"), but the
  as-built deviations post-date the ack: the render still computes its own decisions
  rather than reading Spine back (`30F` §4.4), `30E` §3 records five render-time
  decisions invisible to the decision plane, and `Spine` went generic over a
  `DecidePlane` seam `30E` §5 had placed pure-in-core (`30F` §4.1). Each is disclosed;
  none is ratified; together they mean the "one decision structure" boundary that
  `30I` steps 7/8 will reify into artifact forms is not yet the boundary the ack
  described. The sequencing (artifact forms after `30L`) leaves room to close this —
  but nothing currently *owns* closing it; `30F`'s "owed, named" items have no lane.
- **dec-trip-cleanup-is-still-must-remember (+SURE, low-med).**
  `certifier_trip::demote_on_trip` has exactly two callers (`cli/src/main.rs:1839`,
  `cli/src/world.rs:484`); `plan::build_plan` → `project_plan` produces plans without
  it, and the guarding law is a CLAUDE.md sentence ("a NEW driver MUST call it") — the
  exact must-remember surface `30E:dec-certifier-trip-cleanup` claimed the reification
  dissolves. It records the decision after the fact (`SpineRenderDecision`); it does
  not force it. A typed seat (e.g. plan-projection demanding a trip-disposition
  witness the way it demands `PlanAuthority`) would finish the thought.
- **dec-implementation-push-adjudications-still-unreacted (+SURE, inventory-tier).**
  `307` §5 (08-17): every in-window ruling of the six-lane implementation push is
  conductor-tier and veto-eligible with no typed reaction — the durable-by-exclusion
  census arms, the `30E` §8 adjudication set (including deleting the v1 durable
  grammar), the occupancy-census conservatisms and `307b` §7's five open asks, and
  `307c`'s dialect-fold pair (`dec-dialect-keeps-a-whole-unit-fold` — which is also
  the standing rule behind `FORFEITS:forfeit-two-position-sparing-collide`'s
  too-large-dialect exposure). None looks wrong to me; all are honestly recorded;
  the pile is now big enough that the round-close veto sweep is real work, and two of
  them (dialect whole-unit fold; the durable-grammar deletion) touch surfaces the
  next arcs freeze.
- **dec-open-precision-losses-without-owners (+SURE, disclosed).**
  `307c:fnd-written-establishes-in-a-region-ship-no-check` (subshell regions lose
  their check; hash-munge machinery unexercised; "owner: the seat's, not this
  lane's") and `307a:flg-allow-list-entry-not-added` remain open with no owning lane.
  Conservative directions both; listed because "owner: nobody" is how residue
  ossifies.

## §6 — Plans-compose lens (the not-yet-built, as plans)

- **fnd-owner-only-retirement-cannot-express-region-grain (~SUSPECT, forward-looking).**
  `30L` §6 requires shared-region no-execution proofs projected "onto EVERY owned
  instance invalidator" while the CALL itself may keep running (partial-body
  transformation is the ordinary case, `30L` §0). The landed retirement seat cannot
  say that: `world::effective_invalidators` (world.rs:279–292) retires a `Leaf(owner)`
  node only via `proves_no_execution(owner)` — there is no arm consulting a proof
  recorded against the body node itself, so per-instance proofs under a still-running
  owner would be recorded by `record_round` and silently ignored by the filter.
  Failure direction is conservative (walls stand, value lost, plus one wasted
  settlement round per ignored proof), so this is compose-friction, not danger — but
  `30L` §14 prices "the settlement integration" as the risky third without naming
  this seat, and its stage-4 brief should carry it explicitly.
- **+SURE** `30Da`'s three narrow boundaries hold in the landed core: `Predicted<Rc>`
  stays an opaque exact value through the fold (no verdict-partition applied to
  predict statuses; `standin_for` reproduces any rc), effective Query validity is one
  conjunct at the fold seat (`one_round`'s `validity` → `model.fold`), and
  transport/admission integrity sits upstream of settlement (intake → `PlanAuthority`
  before `SettleInputs` exists). `30D`/`30J` compose as written; `30J`'s
  family-vocabulary qualification lands as an additive widening against the current
  conservative `verdict-minted-facts-thread-their-family` collide-fallback.
- **+SURE** `30I`'s remaining steps (5b–8) sequence correctly against the landed
  core: the pause-interpose rulings (`impl-effective-reach-interposes-before-bundles`,
  `impl-aggregate-verdict-primacy-precedes-bundles`,
  `impl-elision-regions-precede-artifact-forms`) mean no consumer accretes on either
  known-interim surface. The one 30I-side item I could not close from the ledgers:
  `30Ib:dev-the-probe-artifact-still-ships-under-a-refusal` was minted against the
  exit-16 refusal that the human then abolished ([TYPED 08-19]); the deviation is
  *probably* dissolved by that reversal (`exit(16)` is gone from the tree), but no
  record says so — it should be closed or re-stated against the suspension model.

## §7 — Method notes and scope honesty

- The commissioned worry — "kernel work of this size never got its expensive
  cross-cutting review" — is half-true in an instructive way. Stage-i got a
  four-lineage crosscheck; `30K` got a single Sol-neutral review (`30Kb`) that caught
  a genuine flag-gated wrong-elision *before* any user could meet it, plus in-arc
  repairs; `302`/`309` got opaque/cross-lineage passes. What got the *least*
  independent review relative to its license-surface is the `30I` static-loading lane
  (lane reports and conductor adjudication only) — and it is exactly where the
  round's two reversed-by-human calls happened (exit-16 refusal; sourcing-file-relative
  `.`). I read its design and custody rules closely (conservative in the license
  direction: misalignment suspends vouches toward run) but did NOT line-audit the
  funcenv/load implementation of guarded-source recognition (`30I` §3.4 case 2) —
  flagging that as the highest-value target for any follow-up review hour, since a
  recognition false-positive there runs the wrong author's judgment under custody
  (pope-sin tier). **That follow-up round was then commissioned and run — §9 below
  carries it; the flag is discharged.**
- Also not deeply audited: the oracle/syntax crates, the aid/loom lanes, intake
  parsing internals beyond the scout's chokepoint verification, and everything
  commission-fenced (verify instruments, security, WSL/Lean/Kani legs).

## §8 — did not hold

Suspicions — mine and the commission's — that died under scrutiny, so nobody
re-derives them:

- *Loop back-edges under-approximated in reach* — the worklist solves genuine cycles
  to fixpoint (`solve.rs` cycle test), `states[v]` is the in-state, and in-loop sites
  are floored to Run at the decision seat regardless (two independent defenses).
- *Aggregate survival still single-representative* — `30Kb`'s critical is repaired:
  `world::aggregate_survival` walks every establish; `AggregateSurvivalWitness::mint`
  and `with_aggregate_survival` cross-check exact ordered identity; two adversarial
  tests pin later-member MayAlias and later-member rederivation-demotion rejecting
  the whole aggregate.
- *Inline-call elision could hide an opaque body command* — `prove_inline_replaceable`
  is a closed total match; `MustRun`/Kill/Opaque/nested shapes refuse the whole call.
- *Replacement-death proof unbound from its license* — the mint now takes the
  `&ReplaceLicense` (sole-mint witness) and its lexical fence was de-self-matched.
- *`decide_site` reading the output disposition* — the private `DecisionConclusion`
  projects both halves; the source-scrape pin fails loud (not vacuous) under the
  reorderings I considered.
- *Trip cleanup letting stale guards stand* — census-unique-only, occupancy computed
  syntactically from `DefinitionTable` with no solve in the chain; the guard's net
  re-verifies live; reasoned sound.
- *Elided-upstream ⇒ downstream-elides (frame30 drift class B) as a license widening*
  — it is the correct reading of "an elided command casts no wall"; the downstream
  elision rests on the same attributed vouch chain, and a wrong upstream vouch already
  owns the cascade.
- *Backing recompute silently narrowing survival* — backings are still collected
  before the erasure seam (`effect.rs:1890` vs `:1907`), so per-round values are
  stable; the lifetime is wrong-in-principle (`30Kb` §6) but behaviorally inert, the
  trap is documented in `ANALYZER-NEEDS:an-backing-selfframing`, and
  `30L:req-backings-freeze-at-probe-boundary` owns the fix.
- *`AllEstablishesVouched` order-coincidence admitting wrong-fact vouches* — the
  supplied-vs-expected exact sequence equality rejects every mismatch class.
- *A `Refused` intake still emitting a plan* — `PlanAuthority` has no public
  constructor, `authorise` is the one gate, and the driver-side lexical pin forbids
  naming the test-only bypass.

## §9 — Addendum: the focused guarded-source/custody round (second sitting, same day)

The human spent the remaining wallclock on §7's flagged weak spot: a line audit of
the `30I` §3.4 case-2 implementation — the exact-package-sentinel recognition, the
speaker-edge mint, and the custody composition around them (`analysis/src/load.rs`
whole-file; `funcenv.rs` `sentinel_decides`/`sentinel_arm`/`sole_populator`/
`anything_removes`/`load_closure_of`; the `30Ib` §4/§5/§10/§12 ledgers; specimen and
composition-seat checks).

### Verdict: the recognition seat is clean

**+SURE** The seat is built with the same defensiveness as the `30K` kernel, and
the two a-priori attack cells I constructed before reading it are both already
specimen-pinned in the committed corpus (`load30-speaker-minting-is-observable`:
the `beta` wrong-unit sentinel-populator and `gamma` wrong-unit same-named-helper
arms, plus a recorded mutation result proving the recognition's own mint is what
the case observes). Specifics verified by eye:

- `sole_populator` (funcenv.rs:372) answers BOTH `Must` halves non-vacuously: the
  target closure must assign the tested name AND nothing else in the authored
  world may (every loadable program's top level, the whole book AST via
  `book_assigns` — ⊤-valued and below-the-load assignments included). The
  name-census-not-value design (`30Ib` §10 "dissolved rather than solved") is
  sound because uniqueness-of-possible-populator over-approximates
  value-provenance, and I confirmed the one cell it cannot see (a literal
  mismatch between the assignment and the guard's compared value) is inert: it
  can only cause a runtime re-source of the same exact bytes.
- `sentinel_arm` (funcenv.rs:1751) is the load-bearing repair: REUSE requires
  every closure-declared name bound to a definition in a closure FILE; one
  live-from-another-unit binding, any ⊤, any mixed state, or an empty declared
  set declines the whole recognition. The negation/sense arithmetic
  (`then_runs_when_equal`) is correct across all four spellings, and both
  patterns require the reuse branch EMPTY.
- The reuse-arm occurrence is route-demoted through the enclosing walk
  (`ctx.route(...)`; the `an_undecided_guard_mints_no_edge` /
  `a_recognized_sentinel_guard_mints_its_edge` pin pair), so a sentinel nested
  under an undecided guard mints nothing.
- Defense-in-depth is real, not asserted: every wrong-body route I constructed —
  partial closure from an unresolvable nested dep; a rogue unit defining a
  closure-declared name; the exact target loaded first by the book — dies
  independently at the composition seat (`ResolvedOutsideCustody` ⇒ suspend ⇒
  run), because the reached-helper `Must` half deliberately reuses
  `HelperIndex::resolve`'s closure gating rather than new code (`30Ib` §10).
- The test coverage here is the best of the round (tests-lens positive): every
  one of the six withholding conditions has its own named negative test
  (funcenv.rs:3783–4100 — wrong-arm polarity, book-assigned sentinel,
  another-unit populator, never-populated vacuity, own-name removal,
  outside-shadowed binding), the undecided-guard/no-speaker split is pinned at
  both the funcenv and driver seats, and the composition seat carries a full
  suspension battery (closure.rs:882–1166) including the ships/suspends contrast
  pair and the sentence-not-disposition pin
  (`selecting_a_dependency_changes_the_sentence_and_not_the_disposition`).

### Two contained robustness cells (report, don't repair)

**~SUSPECT contained, both.** The arm PREDICTION (condition 6) can diverge from
the runtime arm in two ways the census cannot see: a sentinel assigned inside
some loaded oracle's *function body* (`LoadProgram::assigns` is top-level-only,
by construction), and a sentinel *exported in the host/process environment* at
artifact runtime. I chased both to ground: in each, the mispredicted arm affects
only which definitions the model thinks the book's own runtime has live — and
since every Dorc-shipped surface (probe bodies, guard preambles) materializes its
helper closure into the artifact rather than depending on the book's load having
happened, the license plane never rests on the prediction; the failure lands as
rc-127/unknown ⇒ run, or as stale narration. No pin exercises either cell; a
cheap DST/e2e cell for the env-exported-sentinel shape would pin the containment
argument rather than the happy path.

### The lane's open ledger, inventoried (second-lens; all builder-recorded, none adjudicated)

- **`30Ib:dev-ambient-include-guards-are-not-evaluated`** is live state at HEAD
  (the ambient prefix survives the `--pre-source` rework; `main.rs:372/:862`): an
  ambient package's `LoadProgram` never runs, so its include-guarded dependency
  binds unconditionally. The builder's containment argument (winner shifts only
  under an earlier same-name definer, where shadow refusal/reporting fires) plus
  shipped-closures-are-the-model keeps this out of the license-hazard class, but
  the named fix (run ambient programs at the `Entry` transfer) carries an open
  licensure call — what an unresolvable ambient load does — and `30Ib` §5.2's
  variable-rooted-custody work is BLOCKED behind it. This is the sharpest
  unadjudicated item on the lane.
- **`30Ib:dev-guard-false-direction-fenced-to-role-names`** — the `command -v`
  guard decides FALSE only for role-shaped names, though the TYPED ruling's own
  example guards an ordinary helper name. Withholding-direction, correctly
  flagged up rather than settled; unowned since.
- **`30Ib` §5.2/§5.3** — a dependency spelled through a variable root
  (`. "$ROOT/dep.sh"`) suspends vouch composition (measured live), and a nested
  `.` is not yet a locator edge. Value-loss on exactly the healthy-package idiom
  `30I` §2.2 canonizes; conservative; queued behind the ambient-prefix fix.
- **`30Ib` §5.4** — the `funcenv-reads-source-literal-plane-only` wall is
  VACUOUSLY held today (`variable_before` has no per-variable `ValueGrade`);
  the crate law discloses this, but when `core`'s `seam-re-bind` lands, that
  accessor gaining the grade gate is load-bearing for plan reproducibility.
  Worth a red-first pin *now* rather than a sentence in two places.
- **`30Ib` §12.3 `dev-transcript-authored-by-hand`** — one committed golden
  (`load30-speaker-minting-is-observable`) entered the corpus via a hand-driven
  capture rather than `BLESS=1` (builders may not bless). The ordinary e2e run
  proves it byte-exact, so this is disclosure-tier — but the conductor should
  know it when the corpus-wide bless of `30I` step 8 happens.
- The `pin28-variable-resolved-source-loads` re-spell (§4 above) is confirmed
  *owed and unstarted*: the ledger's own "Owed" line names the exact fix
  (`. "./$PKG.oracle.sh"` + re-bless).

### Addendum's did-not-hold

- *Name-level sentinel recognition admits a same-valued stranger* — no: the
  census is uniqueness-of-populator over the whole authored world, and the
  `beta` specimen pins the exact cell.
- *Same-named helper from another unit rides a recognized edge* — no: file-grade
  binding identity in `sentinel_arm` declines mixed worlds, and the `gamma`
  specimen plus the composition seat both catch what slips past.
- *A partial transitive closure (unresolvable dep) widens recognition into
  licensing* — no: the invisible dep's material is equally invisible to
  `by_path`, and every reach into it suspends at composition.
- *The guard-literal/assignment value mismatch matters* — no: worst case is an
  idempotent runtime re-source of the same bytes; custody and licenses are
  unaffected, which is presumably why the value question could be dissolved.
- *Lexical-only path identity (no digest/inode) lets one physical file wear two
  canonical keys* — real, but checked: the double-load lands in the plurality
  machinery, where byte-identical copies dedup
  (`byte_identical_copies_across_sources_dedup_rather_than_refuse`) and
  differing bytes suspend; both directions conservative.
