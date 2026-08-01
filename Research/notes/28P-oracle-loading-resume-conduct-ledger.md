# 28P — Oracle loading and resolution: the resume conduct-ledger

Conductor ledger for the post-checkpoint half of the `plans/28K` lane (branch
`ai/r28-oracle-loading`, worktree `r28-oracle-loading`), session
`r28-megamerge-continuation-impl`, resumed 2026-07-31. Predecessor build ledger:
`notes/28O` (historical; stages A/B/G/D/E + the rebase). The ONLY live implementation
plan is `28K` §10 (bitem0–bitem9 + fold checklist); on committee-corner conflict,
`28M` governs (§7 ack-ledger, §8 license-plane ground truth). Confidence marks per
`spike/CLAUDE.md`.

## Standing state at resume

- Lane re-rebased onto `ai/main` tip "(AI dsn re) Rewrite the build shape into the
  slugged resume plan; bank the demotion-is-not-deletion lean" — 38 commits replayed,
  zero conflicts; main-side delta was docs-only (28K §10 rewrite, 28M §7/§8 growth,
  LIVING_STATUS, TODO, CONTRIBUTING, USER_STORY line). Quick-gate verification run
  post-rebase.
- E→F checkpoint CLEARED per LIVING_STATUS (28K §9 fully typed-closed; full-positional
  regime ACKED spike-tier; committee fence unresolved-but-motion-authorized,
  build-as-spiked, marked unratified in code + ledger).
- Stage-F riders banked in `28M` §7 carried into the bitem briefs: the
  `WhyReport.oracle_paths`/`oracle_srcs` rename (bitem7) · fold-glance at the
  load-inert refusal newly firing on three loom-era lint cases · the pre-existing
  Windows-only `mise run loom:compile` stack overflow on
  `syntax-unsupported-nesting-bound` (NOT this lane's).

## Dispatch log (compressed as lanes land)

- builder-1: bitem0 positional-regime conversion (LANDED) → bitem2 resolution-seat unification
  (LANDED, taken before bitem1 because the gate needs both indices' provenance) → bitem1
  pin-by-definition-bytes (handed off unstarted).
- builder-2: bitem1 pin-by-definition-bytes (LANDED).

## Findings / deviations

### Conductor adjudications at the bitem0/2 close (2026-07-31)

- **adj-positional-plural-value-loss-carries-to-bitem4** — builder-1's flagged
  `tc-positional-plural-value-loss` (the withhold-not-re-resolve cut dilutes BOTH sanctioned
  plural idioms at the verdict tier: sites above a blessing `unset -f`, and inside a
  regional-preference subshell, answer NOTHING rather than answering from the positionally-live
  file). Adjudicated: the withhold STANDS spike-tier — conservative direction, zero corpus
  coverage, softening is additive under `rul-strawman-formats-no-compat` — and the question
  carries into bitem4's brief, whose committee-fence surface (per-family/per-file maps) is the
  same seam; build the per-file effect map only if the fence work makes it near-free. DISCLOSED
  TENSION for the human: `28K` §1 `rul-scope-by-subshell-resource` sells the subshell idiom as
  answering from the re-sourced file, and `28M` §3 `wall-verdict-tier-sovereignty` premises
  per-position sovereignty — the spike under-delivers both until the seam closes. ~SUSPECT
  acceptable; awaiting typed ack or overrule.
- **adj-positional-gate-is-bitem3s-seam** — builder-1's §8 doubt (the agreement-gate is a
  FOURTH untyped mechanism holding the license-plane monologue) rides into builder-2's brief:
  bitem3's `SourceFileId`-into-`ReplaceLicense` threading should SUBSUME the gate, not sit
  beside it.
- **bank-decidable-fold-sited-post-bitem1** — `28M` §9 landed (design settled, sibling
  conductor); the fold runs as its own small lane after bitem1, before bitem3–5 (funcenv is
  quiet then, and bitem4/5 want the un-poisoned polyfill world as input). The
  function-only-definedness scout came back NEGATIVE (human-relayed 2026-07-31): all
  alternatives are shell-specific (`read`/`declare`/grepping `-V`); the `command -v`
  fn-definedness contract stands alone, decidable-set v0 is final, and bitem8's reserved
  battery case is THE pin for the contract's one divergence cell.

### Conductor adjudications at the bitem1 close (2026-07-31)

- **adj-lint-heuristic-widening-kept** — builder-2's `tc-lint-dialect-heuristic-widened`
  KEPT: the old gate error'd every ordinary book helper-function (false, error-severity,
  corpus-wide) and refused the helpers half of `28M` §8's commissioned two-file package
  shape outright — bitem6 cannot run without the fix. Residual detection loss accepted and
  named: a stray mark in a book that also defines functions no longer trips the mark-subset
  lint (~SUSPECT small; the load-inert gate still covers marked files). Precise re-cut
  (lint per-statement rather than per-file) is a named small item, not commissioned. The 8
  re-blessed looms merge into the fold-checklist eyeball-glance.
- **adj-survival-closure-gap-measure-then-fix** — builder-2's
  `tc-survival-lanes-ship-closure-less-bodies` routes into builder-3's lane (bitem3–5), as
  a MEASURE-FIRST item ahead of bitem3: determine whether a survival-lane body
  (`disturbs`/`resolve`/`reaches`) that dies mid-emission on an unbound helper is consumed
  whole-body-atomically (failure ⇒ no claim ⇒ wall stands, safe) or as
  emitted-lines-then-error (a PARTIAL at-most claim ⇒ wrongly-narrow footprint ⇒
  under-execute at the survival tier — worse than the guard lane's closed hole, because
  it corrupts the one naked-trust cell mechanically). If partial: closing it is the lane's
  FIRST item, before bitem3. If atomic: extend closure capture there as ordinary value-add,
  riding bitem4's surfaces.
- **adj-hash-munge-unit-pinned-accepted** — the agreement gate makes hash-munge unreachable
  today (only the ambient winner ships); built anyway per §4, unit-pinned. Correct call:
  bitem4's per-file surface is what makes it reachable, and funcname-dedup would then
  silently mis-bind. No corpus pin owed until then.

## bitem0 — the full-positional regime (LANDED)

`28K` §2 `rul-visibility-is-full-positional`. Every site-keyed consuming act now reads the
function environment AS OF THE SITE'S POSITION; vocabulary acts stay ambient; the two named aid
surfaces are minted. Golden churn across the whole conversion: **ZERO** outside the one new cell —
the prediction held exactly, and the reason is stage G's respell (every corpus unit is
single-definition per role name and define-before-use, so positional and ambient are the same
answer at every corpus site).

### dec-the-gate-is-agreement-not-re-resolution

The conversion's shape, and the decision worth defending. A site-keyed act could implement
positionality two ways: (A) resolve the act's inputs FROM the positionally-live file, or (B)
compute the ambient answer as today and WITHHOLD wherever it is not the one live at the site.
(B) landed, at every seat.

The reason is the effect map. `analysis::effect` resolves a site's identity through one file's
argparse and reads its cells out of `KindIndex`, which is merged whole-unit. Under (A) those two
could come from DIFFERENT files — the identity traced through one author's arms, the cells
declared by another — which measures one cell while keying the record to another: pope-sin tier
(`271:rul-sin-ordering`), and invisible to every golden. Under (B) the act either answers exactly
as before (all inputs from one file) or answers nothing. Withholding is the one direction that
cannot widen a license.

The under-approximation this buys is REAL and named: where two files both define one role and the
positional winner is not the ambient one, (B) withholds where (A) would have answered. That world
is reachable only in the two sanctioned plural idioms — the `unset -f`-blessed override, above the
`unset -f`; and the subshell re-source, outside the subshell — both already taxed by `28M` §6
("regional-preference admins: … extra guards on drifted days"). +SURE the direction is safe;
~SUSPECT the value loss is small enough to leave standing (`res-plural-families-withhold-off-peak`,
below).

The agreement is CHECKED, not assumed: `KindIndex` and `VerdictIndex` now record the source index
each provider's rows came from (`source_of`), and `analysis::effect::live_predict_source` refuses
to answer when that disagrees with the resolved file. Both indices and the `checks` vector are
built by one seat over one ordered list, so today they cannot disagree — which is exactly why the
check is cheap and exactly why it should exist before something reorders one of them.

### dec-the-gate-applies-only-to-names-the-unit-knows

`LiveDefinitions::answers_at` answers `true` for a name the `DefinitionTable` has no definition of.
That is the one permissive answer in the mechanism and it is deliberate: the environment's universe
IS the table's names, so a name outside it has no positional opinion, and manufacturing one would
wall every hand-built `KindIndex` in the workspace (the kernel unit tests build indices from no
source text at all). In production the only names outside the universe are the ones
`dorc_syntax::parse` and the dialect parser disagree about — `28O:fnd-two-parsers-disagree-on-funcdefs`
— and `reserved.rs` refuses that class at Error severity before it can ship. Pinned both ways by
`an_unknown_name_is_not_gated_and_a_known_one_is`, and the containment half by the pre-existing
`a_charclass_refused_name_produces_no_binding`.

`LiveDefinitions::unsolved()` is the second, LOUDER escape: an explicitly-named "no environment was
solved for this unit", used by `classify` (the thin wrapper), the two instrument crates, and the
survival HINT lane. Both real drivers construct a solved one.

### fnd-decline-fallthrough-was-still-live-in-the-ship-lane

`28O:fnd-decline-fallthrough-was-live-in-tree` recorded stage D retiring the
first-that-RESOLVES scan in `analysis::effect`. It did not retire it in the SHIP lane:
`ship_predict_body` / `ship_predict_stage` scanned `.rev()` for the first file whose check both
matched the provider AND resolved the argv, so a DECLINING live body fell through into a shadowed
one's arms — `28K` §6 `rej-decline-fallthrough-cascade`, still shipping, one layer past where it
was found. bitem2 retired it (below); recorded here because it is the same fence's SECOND confirmed
real-world instance, and because "we fixed the analyzer seat" was not the same claim as "we fixed
the rule".

### res-plural-families-withhold-off-peak (disclosed under-approximation)

Named, not chased. Under `dec-the-gate-is-agreement-not-re-resolution`, a family with live
definitions in two files answers only at sites where the ambient winner is also the positional one.
Concretely: above a blessing `unset -f`, and outside a regional-preference subshell, the family
answers nothing rather than answering from the file that IS live there. Conservative (walls, never
a wrong license), zero corpus coverage, and it sits inside the population `28M` §6 already prices.
Closing it means a per-file effect map, which is `28M` §4's committee-fence surface (bitem4) rather
than this item's.

### res-why-world-lifts-no-book-definitions

`WhyWorld` builds its `checks`/`verdict_sets` from the ORACLE vector only, so an in-book role
function does not lift on the loom seat at all — pre-existing, and the gap `28M` §7's
`WhyReport.oracle_paths` rename rider (bitem7) points at. The positional oracle is now solved there
on the same rule the binary uses, with the book's `SourceFileId` sited one PAST that vector, so a
site a book definition owns WITHHOLDS rather than answering from an oracle a shell would no longer
call. That is the honest reading of the gap, not a workaround for it; the shadow refusal itself is
still absent from that seat.

### res-instrument-lanes-stay-ambient

`coverage`, `sweep`, and `survival_diagnostics` pass `LiveDefinitions::unsolved()`. The first two
are INSTRUMENTS (they measure analyzer reach; a gate there would make the dashboard report
something other than what it measures) and the third is the HINT lane, where narrating a shape
whose license the positional regime withholds is the aid plane failing in its own safe direction
(`two-plane-aid-law`). Each carries the reason inline.

### The two aid surfaces (both `message: None`; ceiling 16 → 18)

- **`role-defined-below-its-sites`** (Note) — the move-it-up hint. Fires on a book definition of a
  COMMAND role with sites above it that nothing answers; `{sites}` counts them. Deliberately
  silent when another unit's definition IS live at those sites: that world is either served or
  contested, and the note would be noise in both.
- **`in-book-vocabulary-role`** (Warning) — `28M:obl-in-book-vocabulary-role-notice`. Fires on a
  book definition of a KIND-OWNER role (`__resolve` / `__disturbance_reaches_only` /
  `__state_stored_only_in`; the species split now has a named home,
  `oracle::reserved::is_vocabulary_role`). Warning rather than Note because something the author
  wrote genuinely has no effect — unlike the hint, where the book is simply correct sh.

Both are payload-rendered defining cases (`crates/aid/tests/*.loom` + a `fixture.rs` stand-in), on
the `role-family-contested` precedent: their honest trigger is a whole positional BOOK, which a
one-source case cannot materialize. `unwritten_renders_are_greppable_and_pinned` bumped 16 → 18,
a deliberate two-line spend — `28O:res-unwritten-ceiling-spent` said the prior headroom was gone,
and it was.

### Golden churn, case by case

Predicted: none outside the new cells. Actual: exactly that. 1832 → 1836 trials, all additive (two
loom cases, one e2e cell, and the funcenv table-4 unit tests are in-crate). No existing `expected.out`,
`expected.ran`, or loom transcript moved a byte across bitem0 or bitem2. The generated catalog lock
gained its two rows through `loom:compile`/`promote` at the generator fixpoint.

### The behaviour pin: `contest28-late-definition-licenses-nothing-above`

The sharpened consequence cell, end to end, and it reads better than the in-memory table because
the PROBE ARTIFACT shows the property directly: `sites=1`, with `# site:0 unresolvable-no-probe`
where the site above the definition would otherwise have shipped a check. The site below still
guards (its predecessor is an unmodeled establish that really runs, so it walls —
`opaque-poison-is-the-product`), which makes the pin stronger rather than weaker: the guard is
minted from the book's own verdict body, the definition live at THAT line. The case declares
`role-defined-below-its-sites`, so the hint's firing is asserted structurally.

Non-vacuous by construction: under the ambient regime the book's definition was the answer at
every site, so the first site would have shipped and guarded. The `site:0 unresolvable-no-probe`
line is the difference.

## bitem2 — one seat, and the five sites genuinely agree (LANDED)

`28M:fnd-verdict-resolution-duplicates-live-source`. `VerdictIndex::from_sets` consulted iteration
order rather than `live_source`; the three cli ship closures and `build_vouches` each open-coded a
backwards scan. All five now call `dorc_oracle::live_source`, and the "ONE seat… the resolution
sites must agree" doc on it is true rather than aspirational.

### dec-has-defines-never-has-answers

The load-bearing detail of the ship-seat rewrite. The predicate handed to `live_source` asks only
"does file `i` DEFINE this role for this provider", never "does its body answer this argv". The old
scans asked the second question, which is what made them a fallthrough cascade
(`fnd-decline-fallthrough-was-still-live-in-the-ship-lane`). Resolution now picks the winner FIRST
and evaluates only there; a decline by the winner returns `None` and the site runs. Zero churn,
which is the expected result for a single-definition corpus and would NOT have been for a plural
one — so this seat is under-covered by the corpus and its pin is the unit tier plus the reasoning
above (~SUSPECT worth a plural fixture when bitem6's suites land).

### tbl-the-five-seats-after-the-conversion

| seat | whole-unit answer | positional narrowing |
|---|---|---|
| `oracle::lift` (effect map) | `live_source` | n/a — records `source_of` for the consumer to check |
| `VerdictIndex::from_sets` | `live_source` (was iteration order) | n/a — records `source_of` |
| `analysis::effect::live_predict_source` | `live_source` | `answers_at` + `idx.source_of` agreement |
| `verdict_cell_or_auto` | `VerdictIndex` | `answers_at` against `verdicts.source_of` |
| `world::ship_predict_body` / `ship_verdict_body` / `ship_predict_stage` | `live_source` via `shipping_source` | `answers_at` |
| `plan::build_vouches` | `live_source` | `answers_at` |

### Flagged upward

- **`tc-positional-plural-value-loss`** — whether `res-plural-families-withhold-off-peak` is the
  right resting point, or whether the two sanctioned plural idioms deserve a per-file effect map so
  they answer positionally rather than withhold. It is a cross-cutting judgment (it trades a named
  value loss against the committee-fence surface bitem4 owns), so it is flagged, not settled.
- **`res-syntax-owes-a-loud-unsupported`** (28O, unchanged) is now load-bearing in a second place:
  it is the containment argument for `dec-the-gate-applies-only-to-names-the-unit-knows`.

## bitem1 — pin-by-definition-bytes (LANDED)

`28K` §4. The emission half of `rul-runtime-resolution-never-load-bearing`: what a guard invokes is
now decided by the artifact, not re-derived by a landing shell. Predecessor's handoff held — the
seats were landed and the corpus byte-stable, so this started clean.

### fnd-a-shipped-body-was-never-self-contained

The measurement that reframed the item, taken before any code was written. A verdict body calling a
helper LIFTS cleanly today and ships ALONE, in both lanes:

```sh
# dorc-lang/v0.2
WOMBAT_ROOT=/etc/wombat
_wombat_check() { wombat cmp -- "$1" "$WOMBAT_ROOT/$1" ;}
wombat__is_converged() { _wombat_check "$1" ;}
```
⇒ the probe emits `wombat__is_converged() { _wombat_check "$1"; }` and nothing else. +SURE
(measured through the built binary).

Usually that fails safe (`_wombat_check` unbound ⇒ rc 127 ⇒ cant-tell/decline ⇒ the site runs), and
that is what makes it easy to shrug at. It is not RELIABLY safe: a body that IGNORES a helper's
status and answers 0 from a later test reports converged off a helper that never ran, which is the
priority-1 under-execute. So `28K` §4's closure clause is not an ergonomic nicety — it closes a live
under-execute route, and the same route exists in the probe lane.

### dec-the-closure-is-shared-machinery-and-both-lanes-take-it

`28K` §4's Artifact-surface asymmetry says the probe artifacts "were already pinned by composition
and are unchanged by this plan". That is true of PINNING and false of CLOSURE: the probe composes
the resolved body but not its dependencies, so it shipped the same un-runnable bodies. The brief's
"machinery shared with probe composition where genuinely shareable" is read as licensing the shared
seat, and the seat is wired into both: the guard lane (`plan::build_vouches`) and the probe's three
ship seats. Zero corpus churn either way (below), so the reach cost nothing and the alternative was
knowingly leaving one lane broken.

Line held: the SURVIVAL/kind lanes (`touches`, resolvers, reaches) ship closure-less bodies still.
They read the oracle-only vectors, are a deliberately un-widened lane
(`28O:res-in-book-survival-roles-not-lifted`), and widening them is "its own dispatch" per
`cli/CLAUDE.md`. Named, not chased.

### dec-constants-ride-per-contributing-file

The one place the design had to be sharpened rather than implemented. A reference-driven constant
capture cannot prove itself complete: the lexer collapses every operator form of parameter expansion
to one opaque `ParamComplex` and discards the name (`28O:res-load-inert-conservatism`), so
`${ROOT%/}` names a constant the pass cannot see, and a missed constant expands empty — the
under-execute direction again. Two escapes were priced and both lost: refusing on any `ParamComplex`
kills the entire corpus (`case "${1-}" in` is THE canonical idiom), and teaching the lexer to retain
the name is syntax-crate surgery this item does not own.

Landed instead: a file's constants travel whenever that file's CODE travels. Complete for any
expansion form, no analysis hole, no refusal. Its accepted residue is `res-constants-of-non-
contributing-files`, below.

### dec-already-in-place-beats-hoisting

`28K` §4 says two things that collide for one cell, and the collision is worth recording because the
resolution is what moved the only golden. "In the single-definition common case the plain name is
emitted, byte-identical to strip" reads as a mandate to hoist; "the shipped artifact never again
carries two same-named funcdefs by ANY route" forbids doing so when the book's own text already
defines that name — the stage-3 in-book oracle, live in the corpus as
`contest28-late-definition-licenses-nothing-above`, whose artifact carried the hoisted copy AND the
book's own definition.

Ruled: a body the book already defines at top level, byte-identical, is NOT copied. Read carefully,
this is not the rejected no-preamble design (`28K` §4 rejects "guard as bare call, bound by the
book's own sourcing at runtime"): nothing is re-derived, because after the pass exactly ONE funcdef
in the shipped bytes binds that name, so no shell can resolve it differently. And the ordering it
depends on is guaranteed, not hoped for — `rul-visibility-is-full-positional` mints a vouch only
where the definition is the one live at the line, so a book-sited definition always PRECEDES its
guards. +SURE.

### fnd-hash-munge-has-no-reachable-input-today-and-is-built-anyway

Scouted before building it: under `28P:dec-the-gate-is-agreement-not-re-resolution`, `live_source`
answers the WHOLE-UNIT winner and `answers_at` withholds wherever the site disagrees, so at most ONE
body per name can reach the artifact. Two distinct bodies under one name is currently unreachable;
the retired dedup-by-funcname could not actually mis-bind today. +SURE.

Built regardless, and the reason is the item's whole point: the old emission was safe by a
conspiracy of three unrelated mechanisms, exactly the "emergent, not typed" shape `28M` §8 complains
about. If bitem4 closes `res-plural-families-withhold-off-peak` with a per-file effect map, plural
bodies become reachable and the old code would silently emit one and let both sites invoke it — a
site running a judgment its author never made for that line, invisible to every golden. The
mechanism is unit-pinned (`two_distinct_bodies_under_one_name_are_hash_munged_apart`), including the
`23A:P-reingest` floor (a munged name must not parse as a `__role`), and its corpus-unreachability is
recorded here rather than in a comment.

### fnd-a-helpers-only-file-was-refused-out-of-dialect

Found by the contested-helper fixture, not by reasoning. `validate` gated `lint_mark_subset` on
`!src.contains("__")`, reading any `__`-free file as a bare fragment of marked STATEMENTS — so the
HELPERS half of `28M` §8's REQUIRED package shape (bulk logic, non-role names ⇒ no `__` anywhere)
errored `predict-out-of-dialect` at its first funcdef. The cross-file closure this item exists to
build is unusable in its designed shape while that stands.

Sharpened to `!src.contains("__") && !declares_functions(src)`. A file that DEFINES FUNCTIONS is a
definitions file whatever its names look like; the fragment reading is for files that define none,
which is structurally what the `mark-*` cases are. Eight corpus cases moved and every one was
carrying the SAME false error over an ordinary BOOK's function (`install_one() { hork add nginx; }`
⇒ `error[predict-out-of-dialect]`), so the churn is a false-positive burn-down, not a relaxation.
Flagged upward all the same (`tc-lint-dialect-heuristic-widened`, below): it removes an
error-severity diagnostic corpus-wide and it is outside §4.

### res-constants-of-non-contributing-files (disclosed under-approximation)

A body reading a constant from a file that contributes NO code to its closure is not captured —
nothing ties that file to that definition, and shipping every loaded oracle's constants would put
the whole stdlib's variable namespace above the admin's book. Conservative in the value direction
(a missing constant expands empty ⇒ the check answers falsely-diverged ⇒ the site runs, in the
common shape), but not provably so in every shape. ~SUSPECT it never bites before the stdlib exists;
the honest repair is either the lexer retaining `ParamComplex` names or an explicit dependency
spelling, and both are someone else's lane.

### res-hoisted-constants-widen-the-variable-namespace

A hoisted constant lands at the top of the apply artifact and can shadow a book variable of the same
name. Real, and NOT a new hazard class: `guard23-var-namespace-isolated` already pins that a guard
body's own assignments clobber the book's (`pkg=` there), accepted-and-disclosed. What is new is the
surface — the FUNCTION namespace has a loud-friend lint (`reserved-namespace-squat`) and the
variable namespace has none. Named, not built.

### res-helper-bodies-ship-unstripped

Closure helpers ship VERBATIM, which is exactly what `dorc strip` leaves of a non-role top-level item
(`collect_strip_edits` collects from role `Predict`s only). So the byte floor holds by construction.
The pre-existing hole it inherits: a mark inside a HELPER body survives `dorc strip` too, and
`lint_mark_subset` never sees a file that has `__` in it. Out of lane; recorded because the closure
now makes those bytes executable in two more places.

### The aid surface (`message: None`; ceiling 18 → 19)

- **`helper-declaration-contested`** (Warning) — two loaded sources declare one non-role name with
  differing bytes. WARNING on `role-family-contested`'s footing: the refusal only withholds, and
  erroring would punish an admin for a collision two upstream authors caused. Defining case is the
  fixture-payload form (a two-file world a one-source case cannot materialize), with the honest
  firing route pinned separately at e2e (`pin28-contested-helper-withholds-the-pin`). Ceiling bumped
  18 → 19, deliberately: `289:rul-unwritten-ceiling-one-bump`'s headroom was already spent, so this
  is the second conscious bump in the lane.

### Golden churn, case by case

Predicted before building: closure work ⇒ ZERO (a corpus survey over BOTH shapes — case dirs and
`.loom` txtar sections, per `28O:fnd-loom-cases-are-invisible-to-directory-surveys` — found zero
oracle helpers and zero top-level constants anywhere); the in-place rule ⇒ exactly the one in-book
oracle case; hash-munge ⇒ zero (no plural-body case exists).

Actual, by cause:

| cause | cases | delta |
|---|---|---|
| closure capture (both lanes) | 0 | corpus has no helpers/constants to capture |
| already-in-place suppression | 1 (`contest28-late-definition…`) | the duplicated funcdef + its banner removed; run-set, verdict, license, guard all identical |
| hash-munge | 0 | unreachable today (see above) |
| the mark-fragment sharpening | 8 aid looms | one bogus `predict-out-of-dialect` per case, tally `1 error` → `0 errors`; NOT predicted, and the finding is why |
| net-new cases | 2 e2e + 1 aid loom + 12 unit | additive |

No site's verdict, license, or disposition moved anywhere in the corpus. 1836 → 1852 trials.

### Flagged upward

- **`tc-lint-dialect-heuristic-widened`** — the `declares_functions` sharpening removes an
  error-severity diagnostic from every book carrying a plain shell function, and it lands in
  `oracle::validate` rather than in §4's emission surface. My read: it is a false-positive burn-down
  (books are plain sh always, so a book's funcdef is never out of dialect), no real check is lost
  (files WITH `__` never ran the mark-subset lint, and the dialect checking that matters is
  `lift_predicts`/`lift_verdicts_converged`, untouched), and it is a hard blocker for the item's
  headline capability. But it changes `dorc lint`'s exit code for a real input class and touches
  eight cases owned by another concern, so it is the conductor's to keep or revert.
- **`tc-survival-lanes-ship-closure-less-bodies`** — `touches`/`resolve`/`disturbance_reaches_only`
  bodies still ship without their closure, so the under-execute route
  `fnd-a-shipped-body-was-never-self-contained` names is closed in two lanes of five. Held to the
  brief's line (those lanes are oracle-only and their widening is its own dispatch), but the
  asymmetry is now a property of the tree rather than of the design, and somebody should own it.
- **`res-book-span-consumers-arrive-in-stages-d-to-f` is discharged** — the guard attribution's
  `file:line` locus already resolves through the SOURCE-wide table, so a book-sited vouch names the
  book. No new consumer was needed; recorded so the item is not re-opened.

## decidable-condition fold — the polyfill lattice, READ (LANDED)

`28M` §9, sited into this lane by the conductor (`bank-decidable-fold-sited-post-bitem1`). Closes
BOTH halves of `28O:res-polyfill-binding-tops-pending-fold`: the under-complaint half (a
define-if-PRESENT override is now a PROVABLE shadow and draws the refusal) and the larger half (a
guard loaded after a real oracle no longer poisons the family it deferred to). `28K` §3's as-built
paragraph is rewritten in place to describe what is; `28O` is untouched (historical).

The mechanism as built: `funcenv::analyze` is pessimistic conditional-constant-propagation over
its own domain — solve, decide the conditions the solved environment makes decidable, mask the
arms those decisions prove dead, re-solve under a capped monotone mask (`FOLD_ROUNDS_CAP`). Arms
come from `cfg::Branch`, recorded by the lowering that wired the arm edges rather than re-derived
from adjacency order. Decidable-set v0 is built exactly as ruled and no wider.

### fnd-the-fold-alone-cures-the-binding-and-nothing-else

The measurement that reframed the item, taken before the second half was written. With only the
domain-side fold in, the P1 book's exit binding was `Defined(oracle)` and `unprovable` named
nothing — and the site still shipped no check, ran raw, and looked byte-identical to the pre-fold
world. +SURE, measured through the built binary.

The cause is that `dorc_oracle::live_source` answers the whole-unit winner by taking the LAST file
that DECLARES the role, which counts text and not bindings. The dialect parser is line-oriented
and lifts a funcdef nested inside an `if` (measured: `VerdictSet::lift` returns `["foobar"]` with
zero diagnostics for the guarded shape), so the guard's file still won that answer, and
`dec-the-gate-is-agreement-not-re-resolution`'s agreement gate then withheld at every site. The
fold moved the withhold's REASON from ⊤ to disagreement and delivered nothing.

This is why the brief's "never fold one consumer's view without the other's" is the load-bearing
sentence of the item: the two consumers are not both `FuncEnv` readers. One of them is a text scan.

### dec-subtract-the-never-live-rather-than-re-resolve-positionally

The repair, and the alternative it refuses. Making the site-keyed seats resolve FROM the
positionally-live file is option (A), rejected at bitem0 as pope-sin tier — the identity traced
through one author's argparse while the cells come from another's, invisible to every golden. That
ruling stands and this does not touch it.

Instead `funcenv::never_live` names every `(role name, source file)` the environment proves binds
at NO program point, and the cli subtracts those per file, beside the contested withdrawal. Nothing
becomes positional: the whole-unit answer stays whole-unit, it simply stops counting definitions no
execution can call, so `KindIndex`, `VerdictIndex`, the ship seats, `build_vouches` and
`answers_at` all resolve over one population and AGREE. Two reads of one environment, as ruled.

Load-bearing distinction, and the reason this needs care: unlike every other withdrawal in the
tree, this one is EXACT rather than conservative. Removal SHIFTS the winner to a different file
rather than merely withholding, so a wrong answer here grants a license rather than losing one.
The justification is that the fact is a proof, not an estimate — a definition no program point
binds is one no shell execution reaches — and it is empty by construction whenever the solve did
not converge (every binding ⊤ ⇒ `unprovable` withholds the family outright).

### fnd-build-vouches-relifted-the-verdict-sets

Found by the P1 fixture, not by reasoning, and it cost most of the item's debugging budget. With
`checks`/`verdict_sets`/`KindIndex` all withdrawn correctly, the site STILL shipped nothing:
`dorc_plan::build_vouches` re-lifted the verdict sets from raw source text itself, so its own
`live_source` read a FOURTH population — the un-withdrawn one — picked the guard's file, and its
positional filter then refused the vouch. No vouch, no license, no probe candidate.

Retired the same way `VerdictIndex::from_sets` was: `build_vouches_from_sets` takes the driver's
sets, `build_vouches` stays the re-lifting wrapper the DSTs use, and `dorc_oracle::lift_from_sets`
gives the effect map the same treatment. Zero churn at the nine-plus call sites.
`oracle/CLAUDE.md live-source-is-the-only-resolution-seat` complained about the RULE being spelled
twice; this was the LIFT being spelled twice, one layer under it, which produces the identical
failure and is harder to see.

### dec-an-unreached-node-produces-bottom

The one transfer change, and the one with reach beyond the fold. `CfgNodeKind::Top` and
`command_transfer`'s two havoc paths returned ⊤ unconditionally, ignoring their in-state — so a
masked-dead region containing an unmodeled construct or an unresolvable `.` would still have
poisoned the join it never reaches, and the mask would have bought nothing in exactly the books
that need it most. Now every non-`Entry` node with a ⊥ in-state produces ⊥ (`Entry` is exempt:
minting the boundary state out of ⊥ is its whole job). Monotone (⊥ ⊑ everything), and it also
corrects a pre-existing case nobody had priced — code after an `exit` is an island wired to the
program exit, so a ⊤ node there used to poison the exit binding. Zero corpus churn either way.

### res-command-v-is-its-own-poison-wall (disclosed, not chased)

The delivered P1 cell GUARDS rather than elides, and the reason is orthogonal to this item: the
polyfill's own `command -v` line is an unmodeled command that really runs, so it walls
(`opaque-poison-is-the-product`) and nothing below it can elide. The fold's delivery is that the
family answers AT ALL — before it, the site ran raw with no check. Blessing `command -v` as a pure
builtin would convert the guard into an elision for every polyfilled book; that is a
decidable-set-adjacent widening, NOT ruled, and deliberately left alone. ~SUSPECT worth a cheap
look when the stdlib revival makes polyfills common.

### res-the-file-test-head-reads-program-text

`[` carries a glob metacharacter, so the value plane holds every `[ … ]` HEAD at ⊤ — its correct,
conservative pathname-expansion posture at a use site. Honouring only `test -f` would have left
exactly the confusingly-similar middle the U-shape law warns against, so `command_head` reads the
head word from the AST as program text. The narrowness is the safety: only the HEAD, only to name
which builtin the command is; every OPERAND still resolves through `SourceLiteralPlane`, so
`funcenv-reads-source-literal-plane-only`'s actual subject — a value a HOST spoke siting a load —
is untouched. Recorded because it is the one place the fold reads outside the plane.

### Stage-E re-check (the six cells, under the fold)

No outcome regressed. Cell 3 now holds TWO ways and both are pinned separately, per the brief:
`a_guarded_define_if_absent_draws_no_complaint` (the decidable subcase — exempt by dead-edge
PROOF, so `28K` §1's "exempt as a consequence, not a blessing" is delivered as written rather than
reached by abstention) and `an_undecidable_guard_draws_no_complaint_by_joining_to_top` (the ⊤
subcase, with the ⊤ itself asserted so the twin cannot silently become the only route). Cells
1/2/4/5/6a/6b are byte-unchanged. `contest28-top-licenses-nothing` does not move, and the
unit-tier `a_file_test_on_an_unresolved_path_never_folds` asserts `folded_edges().is_empty()` for
that shape directly — a sharper pin than the case, because it fails if the decidable set is ever
widened to decide an unresolved path FALSE.

New alongside them: `a_define_if_present_guard_proves_the_shadow` — the under-complaint half
closing.

### Golden churn, predicted vs actual

Predicted: ZERO outside the one new cell. The corpus is single-definition-per-role and
define-before-use (stage G's respell), and the fold only fires where a decidable condition guards
a definition — no corpus book has one. The never-live subtraction is empty unless something is
provably dead, which needs the fold.

Actual: exactly that. No `expected.out`, `expected.ran`, or loom transcript moved a byte. 1852 →
1865 trials, all additive.

| cause | cases | delta |
|---|---|---|
| the fold masking an arm | 0 | no corpus book guards a definition on a decidable condition |
| never-live subtraction | 0 | nothing is provably dead without the fold |
| ⊥-preserving transfer | 0 | no corpus book has a ⊤ node in unreachable code |
| net-new | 1 e2e loom + 12 unit | additive |

Comment budget: 15 net-new non-doc `//` lines against a cap of 15, after moving the load-bearing
reasoning onto the items' doc-comments where it belongs.

### Flagged upward

- **`tc-command-v-blessing-would-convert-guards-to-elisions`** — see
  `res-command-v-is-its-own-poison-wall`. Cross-cutting (it touches the blessed-builtin table,
  which is a licensure surface), so flagged rather than taken.
- **`res-28m-pre-fold-wording-is-stale`** — `28M` §5.4 ("exempt today by ⊤-abstention, properly by
  the banked fold") and §6's "polyfill authors, pre-fold" rent line both describe the pre-fold
  world. NOT edited here: this worktree's `28M` predates its own §9/§10 by three commits on
  `ai/main`, so editing it would fight the fold rather than help it. Conductor's, at the merge.
- **`res-whyworld-and-survival-do-not-withdraw`** — `WhyWorld` and `survival` build their own
  lifted sets and apply NEITHER withdrawal (contested or never-live). Pre-existing for the
  contested one (`res-why-world-lifts-no-book-definitions`); the never-live one inherits the same
  gap. Benign today because both seats lift from the ORACLE-only vector, so a book-sited guard is
  invisible to them and they happen to agree — but that is a coincidence of the current vector
  choice, not a property, and it breaks the day either widens to the source-wide list (which
  bitem7's rename rider contemplates). Named, not chased.
