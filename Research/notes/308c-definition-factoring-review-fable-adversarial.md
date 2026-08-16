# 308c — Adversarial review: stage-i definition-factoring (`feb2305f..083efd8a`)

> Tier: independent adversarial review (Fable-class, clean context). Charter: distrust the
> landed `28Q` §1 / §8 stage-i-definition-factoring lane; find where a site resolves to the
> wrong authored definition's judgment, where a license is minted against ruled behavior or
> withheld where ruled behavior answers, and where the gates go vacuous. Constraints honored:
> static reading only (no builds, no test runs — every "green" claim below is therefore the
> lane's, not mine); independence from the lane's own conduct record (`notes/307c*`, `30*`,
> `28R*` unread by rule); `[TYPED]`/`[ACKED]` items attacked only for implementation fidelity.
> Certainty markers per house rule: +SURE / ~SUSPECT / -GUESS / --WONDER.
>
> Inputs read in full: `28Q` (the plan), README/DESIGN/IMPLEMENTATION, `spike/CLAUDE.md`,
> the five touched crate CLAUDE.mds (post-state and removed-law diff), FORFEITS.md, and the
> complete range diff minus the excluded lane report. Code walked seat-by-seat:
> `core/src/definition.rs`, `analysis/src/{funcenv,effect}.rs`, `oracle/src/{lib,verdict,
> closure,entry}.rs`, `cli/src/{main,world,survival}.rs`, `plan/src/lib.rs`,
> `sweep/src/drive.rs`, `cli/tests/definition_frames.rs`, the four `frame30-*` looms, the
> `floor30-*`/`contest28-*`/`pin28-*`/`pin30-*` fixtures.

## §1 — Findings, by severity

### find-1-expected-to-flip-diagnostic-unread — the stage's one planted diagnostic produced an answer nobody consumed (HIGH, diagnostic-integrity; behavior itself is conservative)

`28Q` §1 rules that `tc-wrapped-lane-drops-a-case-bodied-in-book-verdict` "was measured,
never diagnosed — the oracle-only-vector reading is a HYPOTHESIS, so the case-bodied
in-book wrapped fixture rides stage-i as an EXPECTED-TO-FLIP cell (the asserted cause gets
tested, not trusted)"; §8 stage-i repeats the rider. The cell is
`spike/crates/cli/tests/pin30-wrapped-case-bodied-in-book-verdict.loom`.

What the range shows, all +SURE:

- The loom is **byte-identical across the whole range** (`git diff feb2305f..083efd8a` on it
  is empty). Its committed transcript still reads `sites=0` / `# site:0
  unresolvable-no-probe`, and its own header still says "answers nothing there today."
- Its sibling `pin28-wrapped-vouch-answers-at-a-live-site.loom` — also unchanged, and
  predating the range — **already ships and elides an in-book, book-sited verdict at a
  wrapped site** (transcript lines 105–115: the probe carries `sudo__enter` + the book's
  `hork__is_converged`, the apply elides the site). The wrapper oracle is byte-identical
  between the two cases; the only difference is the verdict body (straight-line vs
  case-with-decline-arm). So the "oracle-only-vector" hypothesis was falsifiable from
  in-tree evidence **before** the lane ran: the wrapped seat consulted book definitions for
  straight-line bodies already.
- I traced every seat on pin30's path under the landed code and found **no mechanism that
  distinguishes it from pin28**: `build_wrapped_analysis` (cli/src/survival.rs:612) is
  handed the source-wide vectors (main.rs:945–958); `resolve_inner_check`
  (survival.rs:999) resolves the inner verdict through `shipping_source` →
  `dorc_core::answering_file` over source-wide `verdict_sets`, and the book's definition is
  Live at the site; `closure_for` is trivially Ok (no helpers in the world);
  `lift_tolerance` (oracle/src/entry.rs:289) treats the top-of-body `: safe-across user` as
  unconditional for case bodies too (entry.rs:333–340); `decide_entry`'s inputs are
  byte-identical between the two cases; `evaluate_verdict_coord` returns None (no marks) in
  both, minting the same auto-cell. Every input differing between the cases feeds a seat
  whose outcome I could show identical.

Consequently one of two things is true, and both are findings:

- (a) ~SUSPECT: the cell **does not flip** for a cause that lives outside every seat this
  lane touched and outside the plan's asserted cause — in which case the EXPECTED-TO-FLIP
  diagnostic has delivered its answer (*the hypothesis is falsified; the real cause is still
  unknown*) and that answer is recorded **nowhere a reviewer may read**: `28Q` §1/§8 still
  present the hypothesis as pending (plans are ahistorical and must be rewritten to current
  truth — AGENTS.md), `FORFEITS.md:forfeit-wrapped-case-bodied-book-verdict` still carries
  `REVISIT: the stage-i fold` although the stage-i fold is exactly what commits
  `4ac940d2`/`083efd8a` closed, and no crate CLAUDE.md nor any `.rs` mentions the cell
  (grep: zero hits for `case-bodied`/`pin30` outside the fixture).
- (b) --WONDER: my trace is right, the cell **does** flip under the landed binary, and the
  committed transcript is stale — i.e. the corpus is red at the review tip on this case
  (`run: round-trip` / `fixpoint: executed` cases are executed by the e2e gate; a shipped
  check would fail gate-1 against the pinned `sites=0` transcript). I could not run the gate
  to discharge this branch; the lane's own green claim is the only thing against it.

Either way the first post-review action is mechanical: run the pin30 trial alone and read
the answer out — into the FORFEITS row (rewrite or discharge), into `28Q` §1/§8 (rewrite to
truth), and, if (b), treat it as a broken-gate close. The offense is against `28Q` §1's own
"the asserted cause gets tested, not trusted", the FORFEITS discipline ("a captured
forfeit's row is rewritten or removed"), and `Research/plans` ahistoricity. The behavior
itself fails safe (the site runs), which is why this is a diagnostic-integrity finding and
not a wrong-elision.

### find-2-two-position-sparing-floor-not-built-while-plural-worlds-unlock (MEDIUM; flag-gated, corpus-invisible, constructible)

`28Q` §9.12 `pin-two-position-sparing` records an ACKED (extremely soft) build FLOOR:
"collide unless both positions agree on the backing family's closure and dialect", logged as
`FORFEITS.md:forfeit-two-position-sparing-collide` — whose RULE line states, present-tense,
"the frame-relative sparing meet COLLIDES whenever claim-position and backing-position
disagree about the backing family's … dialect."

As built, no such collide exists anywhere. The landed choice
(`oracle/src/lib.rs:dialect_minting_source`, `build_dialect` at lib.rs:260-269) keeps ONE
whole-unit minting winner per family — deliberately, to hold the stage-i byte-identity gate,
and it is honestly disclosed in code (`opt-dialect-keeps-a-whole-unit-winner`, a ruling I
cannot verify from permitted surfaces; -GUESS it lives in the 30* conduct record). The
problem is the composition with what stage-i simultaneously does: **retiring the agreement
veto is precisely what makes plural-frame worlds licensable** (the `frame30-*` cells
celebrate it), and in exactly those worlds the global dialect is now consulted at position
pairs where the frame-true dialects differ — the cell the ACKED floor says must collide.

Construction (the book is the last source-wide file, so its predict wins the minting fold —
`lift_from_sets`' `live_source` over `binds_somewhere`; the subshell definition draws no
contest, per `28K` §1's sanctioned idiom):

```sh
# certs.oracle.sh — mints only @synced for org.foob.Certs
foobar__predict() { case "$1" in
   sync-certs) foobar status -- "$2" : org.foob.Certs:"$2"@synced ;; esac ; }
```
```sh
# book.sh (run with --risk-faultless-skips)
(
foobar__predict() { case "$1" in
   sync-certs) foobar quick -- "$2" : org.foob.Certs:"$2"@extra ;; esac ; }
foobar sync-certs /etc/nginx/certs        # regional frame; dies at the paren
)
… : org.foob.Certs:"$dest"@extra …        # an ambient claim spelled with the REGIONAL token
```

The family's sparing dialect becomes the book-file's `{extra}` (or `{synced, extra}` if the
regional body repeats the oracle's arm), and an **ambient** claim@p × oracle-minted
backing@q meet can now SPARE on the strength of a token that only the regional author —
whose definition is live at *neither* position — ever minted. `sparing-algebra` requires
claim-token ∈ dialect(backing's minting family); with one global dialect that check cannot
see that the token's mint is frame-dead at both positions. Under the ACKED floor this meet
collides.

Mitigations, stated fairly: +SURE this is byte-identical to pre-conversion behavior (the
minting winner is preserved exactly, and the lane's
`a_never_live_definition_mints_no_dialect_tokens` test pins the dead-polyfill half); +SURE
it is `--risk-faultless-skips`-gated, inside the design's one consented naked-trust cell;
+SURE the plurality census (`definition_frames.rs::
every_reachable_plural_family_is_an_enumerated_plural_idiom`) proves no committed world
reaches it — which is also exactly why no gate can see it. And the plan defers the floor's
build to `28T`'s sparing mini-model. So this is not a stage-i regression; it is a
**pre-existing looseness whose blast radius stage-i deliberately widened** (veto retired ⇒
plural frames licensed) while the register text claims the floor already rules. Two owed
repairs: rewrite the FORFEITS RULE line to the as-built truth (one whole-unit dialect; the
collide floor is CAPTURE, not RULE), and treat the `28T` mini-model as blocking for any
promotion of plural-idiom books beyond the census's enumerated five.

### find-3-artifact-carries-two-same-named-funcdefs (LOW; law text vs. new behavior)

`plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding` states "the artifact never
carries two same-named funcdefs by ANY route." The new
`frame30-subshell-body-answers-inside-only.loom` apply transcript (lines 102–132) carries
`foobar__is_converged` **twice**: the guard preamble's oracle body at artifact top, and the
book's regional body inside the parens (book bytes, untouchable). Sh scoping makes the
binding correct at every guard site (+SURE — the regional definition shadows the preamble
only within the region), so this is not a wrong-binding; but the law's letter is now false,
and the consequence it was guarding — `23A:P-reingest` — changes shape: re-ingesting this
artifact yields a within-file plural (`DefinitionProvenance::Ambiguous` /
`helper`-adjacent `216` e-1 territory), which answers nowhere ⇒ conservative run. Reword
the rule to bind EMITTED definitions, and pin the re-ingest of a region-plural artifact as
a case (currently untested; ~SUSPECT it degrades honestly but nothing proves it).

### find-4-contested-census-arithmetic (LOW; doc accuracy in law-adjacent comments)

`core/src/contested.rs` (new comment, commit `33298818`) claims "the remaining FIVE
(`contest28-*`, `guard23-reingest-collision-verbatim`) really do load both and are held
byte-stable by exactly this withdrawal." But the plurality census enumerates
`contest28-polyfill-guard-defers-to-the-oracle` and `contest28-unset-f-blesses-elision` as
BLESSED plural idioms — reachable, licenses intact, *not* withheld (that is why they sit in
`PLURAL_IDIOM_CASES`). A family cannot be both held-by-withdrawal and
reachable-with-licenses-intact; the glob `contest28-*` over-counts (~SUSPECT the true
withdrawal-held set is three `contest28-*` plus `guard23-*`; I could not run the census to
count). Small, but these comments are what future agents cite as population facts.

### Observations (compliant, worth a line each)

- obs-no-opinion-plural-withhold: the `NoOpinion` arm's sole-answers/plural-withholds rule
  (`core/src/definition.rs:153`) is a genuine disposition change for hand-built/hint lanes
  (was last-wins). Conservative, order-symmetric, well-argued against the `28K` §6
  load-order-as-adjudicator fence, and pinned both ways
  (`competing_definitions_without_an_environment_withhold_in_either_order` +
  `a_sole_definition_without_an_environment_still_answers`). `[PROPOSED]`-tier mechanics,
  legitimately open; no complaint.
- obs-hint-lane-loses-plural-hints: `survival_diagnostics` now runs `unsolved()` +
  `ContestedFamilies::none()` (survival.rs:1128-1140), so plural-declaring worlds withhold
  hints they used to first-file. Aid-plane only; consistent with the model.
- obs-book-disturbs-walls: a site whose live `__disturbs` is book-defined now walls instead
  of consuming an oracle's footprint (survival.rs:touches_answering_source doc) — the old
  first-file scan could ship the *wrong author's* at-most claim, which NARROWS, which spares
  more. This is a real wrong-elision route **fixed** by the range and order-symmetrically
  pinned (`the_footprint_answers_from_the_definition_the_frame_names`). Credit where due.
- obs-reserved-name-population-licensable: the two-parser-disagreement rows (`Unkeyed`)
  still answer on their own provenance under `NoOpinion` — preserved, ruled
  (`28P:dec-the-gate-applies-only-to-names-the-unit-knows`), and the census now makes the
  population enumerable with a live-specimen vacuity floor. The lint is a REPORT, not a
  refusal (`bd3ce686` corrected the record) — anyone hardening this later must start from
  that fact.
- obs-loom-consumer-second-lift: `dorc-loom/src/consumer.rs:1872` (`fire_book_analysis`)
  lifts oracles through plain `lift` — the `binds_somewhere = true` posture, no contested
  withdrawal, no environment. Pre-existing, outside the range, fixture-firing only; but it
  is a second copy of the driver wiring of exactly the kind the lane's own
  `the-frame-lookup-is-the-only-resolution-seat` law warns about. Watch it when defining
  cases grow plural-definition worlds.

## §2 — What I verified and found sound (the load-bearing positives)

- +SURE **the chimera is structurally closed at the converted seats**: argparse resolution
  and cell reads are addressed by one file index (`effect.rs:command_effect` — `keyed =
  resolved.zip(live)`, `effect_of(live_file, …)`, `widening_of(live_file, …)`); the old
  third-condition agreement check is gone because it is unspellable, exactly as `28Q` §1
  promises. Same at both vouch seats (`plan/src/lib.rs:1535-1623, 1653-1744`), the three
  ship closures (`world.rs:765-847`, `main.rs:ship_predict_stage`), and the survival lane's
  three scans (one shared `touches_answering_source`).
- +SURE **index alignment holds end-to-end**: `source_table` = oracles-in-load-order + book;
  `definition_table` assigns `SourceFileId(idx)` on the same enumeration with the book last
  (world.rs:633-688); the lifted `checks`/`verdict_sets` are built from the same
  `source_refs`; the oracle-only survival vectors align because oracles are the prefix, and
  a frame-named book `__disturbs` falls off the end of the candidate vector ⇒ wall (the safe
  half, documented as its own dispatch).
- +SURE **`answering_file`'s three arms are individually sound** and the dangerous mixed
  cell is unreachable: `Ambiguous` rows require the table to know `(file, name)` (so the
  name is known ⇒ never `NoOpinion` when solved), and the unsolved posture maps *every* row
  `Unkeyed` (`LiveDefinitions::provenance_of` on `bound = None`), so `Ambiguous` can never
  leak into a sole-candidate answer. Every seat's candidate predicate gates on
  role-presence first (`has(i).then(|| provenance)`), so role-absent files are `None`,
  never `Unkeyed`.
- +SURE **probe emission needs no hash-munge for plural frames**: the probe defines each
  site's body immediately before that site's call (frame30 transcript, lines 84-97), so
  define-call-define-call sequencing carries per-site binding; the guard/apply side is
  covered by the pre-existing `pinned-definitions` machinery, and the frame30 apply
  transcript shows preamble-vs-regional scoping doing the right thing.
- +SURE **`never_live` exactness survives as data, not as a withdrawal**: the retired
  per-file set-withdrawal is replaced by `binds_somewhere` feeding only the dialect fold
  (`world.rs:never_live_predict_rows`, keyed per predict member, not per family), with the
  both-halves test (`a_never_live_definition_mints_no_dialect_tokens`) pinning that a dead
  last-declarer does not shift the minting winner. The funcenv floor's `folded_edges = ∅`
  break was re-justified against the new consumer set rather than silently kept.
- +SURE **the withdrawal edge is genuinely closed**: contested withdrawal now reaches every
  lifted vector including the disturbs sets (`lift_touches_sets`/`pair_touches_sets` take
  the fact by parameter), and `WhyWorld` mirrors the binary's exact mint order and
  source-wide vectors, retiring the book-sited-one-past interim whose safety was
  coincidence (`one-definition-table-two-drivers`, post-state).
- +SURE **the differential battery is built to be non-vacuous**: `floor30-*` manifests are
  shell-measured ground truth (landed `1cdd8020`, an ancestor of the range start — the
  measure-first staging requirement was honored, contra my initial suspicion), the engine
  half asserts against them with discovery/vacuity/coverage floors, the plurality census is
  two-way (unlisted-reachable fails AND stale-listed fails), and the join census demands
  its exception branch have a live specimen. This is the right shape for a gate the
  byte-identity check cannot see past.
- +SURE the helper-closure WITHHOLD floor of `28Q` §1.1 is implemented as the whole-unit
  `HelperIndex` refusal (differing bytes anywhere refuse for everyone) — a superset of the
  ruled floor, i.e. conservative, and pinned at the consuming seat by
  `a_contested_helper_closure_withholds_the_role_body`.
- +SURE stage-0's record drift is honestly carried: both crate CLAUDE.mds now read
  "RULED at stage-0 and NOT YET BUILT", matching `28Q` §8's corrected entry.

## §3 — did not hold (suspicions raised and killed by scrutiny)

- dnh-fixtures-after-conversion: the plan's "differential cells land BEFORE the conversion"
  looked violated by the commit order inside the range; it is not — the `floor30-*`
  manifests landed in `1cdd8020`, an ancestor of `feb2305f`. The `frame30-*` goldens landing
  after the behavior is the plan's own lean (pin the future, never the hole).
- dnh-ambiguous-lends-sole-answer: the `NoOpinion` + `Ambiguous` mixed-candidate hazard
  (an ambiguous file silently ceding "sole" to a neighbor) is unreachable — see §2, third
  bullet. The three-state design is tighter than it first reads.
- dnh-tolerance-misses-top-mark: `lift_tolerance` on case-bodied verdicts does honor a
  top-of-body `: safe-across` as unconditional (entry.rs:333-340); my candidate root cause
  for find-1's non-flip died there.
- dnh-provenance-absence-as-unkeyed: no converted seat maps "file lacks the role" to
  `Some(Unkeyed)`; all gate on presence first, so the sole-candidate arm cannot be fooled
  by role-absent files.
- dnh-binds-somewhere-name-mismatch: `never_live_predict_rows` keys by the munged predict
  member name, which matches the table's authored names for every legal-NAME author; the
  residue is exactly the pre-existing reserved-name-marked population, where the old
  withdrawal missed identically (behavior preserved, not regressed).
- dnh-probe-needs-munge: two frames' bodies in one probe do not collide — emission order is
  program order, define-before-call (§2).
- dnh-whyworld-divergence: the why report answering a different world than the run — the
  interim that made this reviewable — is retired by this very lane; binary and `WhyWorld`
  now share vectors, table, contested mint, and seats.
- dnh-verdict-source-count-mismatch: `verdict_cell_or_auto` scanning
  `verdicts.source_count()` while the vouch seats scan `verdict_sets.len()` is inelegant
  but sound — indices beyond `source_count` can hold no row, so the candidate sets are
  identical.

## §4 — Ordered next actions (for the lane's owner, not this reviewer)

1. Run the pin30 trial in isolation; read the EXPECTED-TO-FLIP answer out into `28Q` §1/§8
   and the FORFEITS row (find-1). If it flips: the close was red; re-open the lane.
2. Rewrite `forfeit-two-position-sparing-collide`'s RULE line to the as-built truth and
   name the `28T` mini-model as the gate on plural-idiom sparing (find-2).
3. Reword `pinned-definitions-are-the-artifact's-binding`'s "by ANY route" and pin a
   region-plural artifact re-ingest case (find-3).
4. Fix the `contested.rs` five-case enumeration or make it name files, not globs (find-4).
