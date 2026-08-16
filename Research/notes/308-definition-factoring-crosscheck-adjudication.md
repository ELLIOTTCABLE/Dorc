# 308 — The stage-i crosscheck: batched adjudication

> Tier: conductor adjudication record (Fable, r30-conductor-5, 2026-08-16). Inputs: the five
> review reports `308b` (Fable neutral) · `308c` (Fable adversarial) · `308d` (Sol/GPT-5.6
> neutral, foreign) · `308e` (Sol adversarial, foreign — NO completion marker, treated as
> possibly-truncated) · `308f` (Sol adversarial CLEAN RE-RUN, foreign; human-authorized,
> same kit section, fresh context). All five reviewed `feb2305f..083efd8a` static-only.
> Method: maximum skepticism per the standing crosscheck law; the two load-bearing code
> claims were verified by the conductor's own read of `cli/src/survival.rs` BEFORE
> crediting; every remaining concrete claim is batched into the burndown lane's
> checkpoint-1 verification (the human's batching steer, §5). Provenance is labeled per
> finding; nothing below quotes foreign output as adjudication — the verdicts are mine.

## §0 — Verdict in one screen

The conversion's nine seats are sound — every lane, both lineages, independently verified
the chimera unrepresentable at the converted seats, the withdrawal edge closed for the
role lanes, and the differential battery non-vacuous. What the crosscheck caught is that
**the wrapper/entry/carry lane is a tenth seat-family the conversion never enumerated**:
`build_wrapper_index` (peel model, entry-form bytes, lend map, tolerance vouch) and
`try_carry` (the pure-predicate-carry closure proof) still answer whole-unit,
first-file-wins, frame-blind, and partly from raw re-lifted source that bypasses the
contested withdrawal. Three independent lanes converged on it from two lineages; the
carry-proof half is a constructible wrong-elision route (cardinal-sin tier). The stage-i
stamp WAITS on the burndown. Everything else credited is record-repair: the plan and two
law texts over-claim what landed.

## §1 — Credited, code tier (the burndown lane's build items)

- **`cr-wrapper-entry-folds-frame-blind`** [308b F1/F2 (Fable) + 308f finding 2 (Sol,
  independent) — cross-lineage convergence; conductor-VERIFIED in code: the three
  `or_insert` folds at `survival.rs:963/965/979`, the raw `lift_predicts(interner, src)`
  re-lift at :942–943 iterating `oracle_refs`, no `LiveDefinitions` input to the builder,
  and `decide_entry` consuming `tolerance.get(&inner_provider)` at :735–738]. Failure
  classes, each pinnable: (a) the CONSENT CHIMERA — the frame-live body ships while the
  FIRST author's `safe-across` mark licenses the context shift, breaching `27C`'s
  both-sides consent (a `28Q` §6 preserved invariant); (b) a CONTESTED wrapper family —
  withdrawn from every ordinary seat — still peels, still supplies entry-form bytes, and
  can still reach `EntryDecision::Enter`; (c) the REGIONAL variant (Sol's construction): a
  subshell-local wrapper redefinition that declines is bypassed in favor of the ambient
  first-loaded model, so an engineer's explicit in-frame decline licenses nothing it
  should. Mostly pre-existing (the range never touched the builder), but stage-i widened
  the plural population these folds disagree across AND landed the law text that claims
  seat closure — so it blocks the stamp either way.
  **RULING `rul-wrapper-lane-joins-the-conversion`** [CONDUCTOR]: convert, do not carve.
  The property: every wrapper-lane consuming act — peel model, entry-form bytes, lend map,
  tolerance vouch — answers from the definition live AT the consuming site, resolved
  through the same frame machinery as the converted seats, over the driver's
  already-lifted, already-WITHDRAWN vectors; raw-source re-lifts in this lane die. A
  contested wrapper family peels nothing and enters nothing. Last-wins is NOT a fix (Sol,
  endorsed: it still fails subshell/re-source/`unset -f` worlds).
- **`cr-carry-proof-answers-from-the-wrong-definition`** [308f finding 1, Sol,
  CRITICAL-severity; found ONLY by the re-run — pending checkpoint verification of the one
  load-bearing claim: `try_carry` at `survival.rs:865` selecting its verdict body by
  `verdict_sets.iter().find_map(...)`, first match in load order, then proving THAT body
  read-set-closed while `resolve_inner_check` ships the frame-live body]. If the code
  shape verifies (the conductor's own read reached the `try_carry` call at :762 and it is
  consistent), this decouples pure-predicate-carry's license from the measured body: an
  earlier read-set-closed definition can license ambient measurement of a later, frame-live
  body with an unmarked context-sensitive read, and a positive ambient answer can elide a
  wrapped mutation — under-execution, the one cardinal sin, on the UNFLAGGED carry path.
  **RULING `rul-carry-proof-is-same-definition`** [CONDUCTOR]: the closure proof and the
  shipped body come from ONE resolved definition; a site whose frame-live verdict is not
  the body proven closed never carries. Pin: prefer an end-to-end cell; if the licensing
  world is not spellable today, a seat-level test on `try_carry` plus the coupling fix
  suffices, disclosed.
- **`cr-license-chain-untested-end-to-end`** [308d finding 2, Sol neutral] — the battery
  proves frame identity but never drives a two-live-`__predict` world (different argparse,
  different declared cells) through fact-mint → probe → disposition. Structurally the
  seats are keyed (both Fable lanes verified `keyed = resolved.zip(live)`), so this is
  coverage, not a bug: ONE new cell rides the burndown; report-if-unspellable latitude.
- **`cr-small-textual-in-code`** — (a) `contested.rs`'s five-case census comment
  over-counts (308c find-4: two `contest28-*` cases are BLESSED shapes, not
  withdrawal-held — verify then fix, name files not globs); (b) `funcenv.rs:191/:196`
  doc-comment still cites the deleted `answers_at` and the corrected refuses-wording
  (308b F7); (c) one comment line at the dialect fold naming the `map_provider_name`
  normalization asymmetry (308b F7).

## §2 — Credited, record tier (conductor-authored at the burndown fold)

- **`cr-pin30-record-stale`** [308b F3 + 308c find-1, convergent] — the lane measured
  `fnd-pin30-did-not-flip` honestly (`307c`), but `28Q` §1/§8 still carry the
  EXPECTED-TO-FLIP hypothesis as pending, `FORFEITS`'s wrapped-case-bodied row still says
  REVISIT-at-stage-i-fold, and the loom's own header says "answers nothing there today"
  as if pre-stage-i. Both Fable lanes ALSO produced a better diagnosis lead than the lane
  had: `pin28-wrapped-vouch-answers-at-a-live-site` already ships an in-book verdict at a
  wrapped site, so the discriminator is the verdict body's SHAPE (case-with-decline-arm
  drops; straight-line ships), not vector membership — the oracle-only-vector hypothesis
  was falsifiable in-tree before the lane ran. Owed: rewrite plan + FORFEITS row to
  measured truth naming the body-shape lead; the diagnosis CHASE stays banked (the site
  runs; value-loss only). NB the wrapper-lane conversion (§1) may move this cell — the
  burndown reports either way.
- **`cr-two-position-forfeit-overstates`** [308c find-2] — `FORFEITS`'s
  two-position-sparing row states the collide floor in present-tense RULE voice; as built
  the dialect is deliberately whole-unit (`307c:dec-dialect-keeps-a-whole-unit-fold`,
  conductor-ruled, byte-identity-preserving) and no such collide exists. Rewrite the row:
  the floor is CAPTURE, owed at the sparing mini-model, not RULE; the veto-retirement
  widened the population that will eventually need it. No code change — the reviewer's
  construction is flag-gated, census-invisible, and pre-existing behavior.
- **`cr-plan-keying-letter-vs-ruled`** [308d finding 1, Sol neutral] — mechanically TRUE:
  indices key `(source-index, provider)` with a `(file, name)` provenance join, custody
  derived — not the stored `(SourceFileId, span, custody)` triple `28Q` §1 spells. But
  that shape was RULED at the lane checkpoint (`307` §2: stored-triple was PROPOSED-tier;
  derived custody + join-totality census + within-file-plural ⇒ `Ambiguous`/withhold,
  riding the pre-existing same-file refusal). Adjudication: implementation stands; the
  PLAN owes the rewrite (plans are ahistorical) so §1's letter stops promising a
  representation the ruling re-cut. The within-file span-granularity residue (308b F5 is
  the same observation) stays conservative and BANKED-low.
- **`cr-auto-cell-shared-coordinate`** [308b F4] — post-conversion, two frames' different
  authors legitimately mint ONE `dorc-auto:<provider>` cell; the reviewer traced the
  containment (cross-site disagreement meets to Unknown; same-cell never
  provably-disjoint; auto kinds fence-blind ⇒ may-touch) and it holds. One law sentence
  owed at `core/CLAUDE.md`'s auto-cell bullet so "chimera unrepresentable" doesn't travel
  further than the mechanism; the `probe_origins` why-chain join (a wrong-body record
  citable for the other author's site — mis-attribution-adjacent, aid-plane) is BANKED.
- **`cr-artifact-two-funcdefs-letter`** [308c find-3] — `frame30-subshell-*`'s apply
  artifact correctly carries a preamble body AND a regional book body under one name;
  `plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding`'s "never two same-named
  funcdefs by ANY route" is now letter-false. Reword to bind EMITTED (preamble)
  definitions; the region-plural re-ingest pin BANKED (Ambiguous-withhold covers it).
- **`cr-whyworld-wrapped-residue-line`** [308b F7] — `WhyWorld` models no wrapped sites
  (`peeled = BTreeMap::new()`), so a why report over a wrapped book explains a narrower
  world than the run. Pre-existing, out of range-scope; one named residue line owed at
  `cli/CLAUDE.md one-definition-table-two-drivers`.
- **`cr-hash-munge-claim-unwitnessed`** [308b F6, corroborating `307c`'s
  `fnd-written-establishes-in-a-region-ship-no-check`] — `28Q` §1's "munge becomes
  reachable" has no witness cell and the reviewer's static walk suggests near-vacuity at
  stage-i; soften the plan claim to as-measured, keep the machinery as belt-and-braces.

## §3 — Not credited / no action

- **308e's null verdict** — superseded by `308f`; the original run was possibly-truncated
  AND returned no findings where the clean re-run returned a Critical. Lesson banked in §5.
- **308c obs-loom-consumer-second-lift** (`dorc-loom` fires book analysis through plain
  `lift`, no withdrawal) — real, pre-existing, fixture-only; BANKED as a watch item for
  when defining cases grow plural worlds; not burndown scope.
- **308b F5 / within-one-file two-frame differential blindness** — bounded by the
  Ambiguous withhold; banked-low with `cr-plan-keying-letter-vs-ruled`'s residue.
- Everything in the reviews' own did-not-hold ledgers stands as exculpatory; no action.

## §4 — What the crosscheck independently verified sound (the positive record)

All five lanes, summarized without re-quoting: seat conversion chimera-free at the nine
converted seats (`keyed = resolved.zip(live)`; both vouch seats; three ship closures;
footprint scans through one shared resolution) · the withdrawal edge genuinely closed for
role/disturbs lanes in both drivers · `never_live` correctly reduced to the dialect fold
with the polyfill direction argued and pinned both ways · the `NoOpinion` plural withhold
order-symmetric and safe · driver unification real (one table, one mint order) · the
floor30-before-conversion staging honored · the plurality/join censuses two-way and
non-vacuous · stage-0's record drift honestly re-marked. The footprint forward-scan fix
(`fnd-survival-footprint-lane-scans-forward`) was independently re-derived as a real
pre-existing wrong-elision route correctly fixed in-lane (308c credits it explicitly).

## §5 — Process notes (the co-equal deliverable)

- **`prax-adversarial-null-needs-a-completion-marker`** — the one lane without a
  completion marker was also the one lane returning "no findings survived"; its clean
  re-run surfaced the arc's only Critical. A null adversarial result without provenance of
  completion is NOT evidence of absence; re-run before crediting a null. (The re-run cost
  cents and one shim; the human's pre-written-kit design meant zero contamination risk.)
- **`prax-conductor-verifies-drivers-opus-verifies-the-rest`** [human steer, 2026-08-16] —
  the conductor hand-verifies only the claims that DRIVE scope decisions (here: the
  wrapper-fold code shape, one read); every remaining concrete claim batches into ONE
  Opus verification unit riding the burndown lane's checkpoint, saving frontier tokens
  and giving the verifier a coherent unit.
- **`prax-fenced-reviewers-find-recorded-things`** — two findings (pin30 staleness;
  "recorded nowhere") were artifacts of the reviewers' deliberate `30*`-notes fence: the
  lane HAD recorded the measurements. The fence is still correct (independence beats
  omniscience); the adjudicator just has to reconcile against the lane record before
  crediting "undisclosed".
- Shim mechanics: the codex re-run shim ended its turn with the call in flight; the
  harness's background-completion rewake recovered it cleanly, and the report was filed
  and committed with only tool-mechanics friction (disclosed upward, properly). One
  hygiene miss: the shim COMMITTED its `.claude-commit` sentinel; caught at cherry-pick,
  amended out. Worth one line in the shim agent-defs someday.

## §6 — Destinations

- **Burndown lane** (Opus, one dispatch, checkpoint-1 = the claim-verification batch):
  §1 entire.
- **Conductor at the burndown fold** (one steering-prose/plan editorial pass): `28Q`
  §1/§8 rewrites (keying-as-ruled · pin30-refuted with the body-shape lead ·
  munge-claim softening · the wrapper-lane seat joining the inventory) · the two FORFEITS
  row rewrites · `plan/CLAUDE.md` pinned-definitions reword · `core/CLAUDE.md` auto-cell
  sentence · `cli/CLAUDE.md` whyworld residue line + withdrawal-law truth-check against
  the landed conversion · `oracle/CLAUDE.md` seat-law wording if the checkpoint alters
  the seat inventory.
- **Banked** (no owner change): the pin30 diagnosis chase (body-shape lead recorded) ·
  `probe_origins` aid-plane join · region-plural re-ingest pin · loom-consumer second
  lift · within-file span-granularity residue.
