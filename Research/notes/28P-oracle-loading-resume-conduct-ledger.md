# 28P — Oracle loading and resolution: the resume conduct-ledger

Conductor ledger for the post-checkpoint half of the `plans/28K` lane (branch
`ai/r28-oracle-loading`, worktree `r28-oracle-loading`), session
`r28-megamerge-continuation-impl`, resumed 2026-07-31. Predecessor build ledger:
`notes/28O` (historical; stages A/B/G/D/E + the rebase). The ONLY live implementation
plan is `28K` §10 (bitem0–bitem9 + fold checklist); on committee-corner conflict,
`28M` governs (§7 ack-ledger, §8 license-plane ground truth). Confidence marks per
`spike/CLAUDE.md`.

## LANE CLOSE (2026-07-31)

The `28K` §10 arc is executed: bitem0–3 and 6–9, the `28M` §9 fold, and the survival
atomicity fixes are LANDED; bitem4/bitem5/withhold-softening/meet-direction registry are
HELD by human order for the parallel committee-corner re-design. Fold checklist discharged:
loom-glance (scout-extracted, all refusals honest; the 8 lint-fix cases confirmed
false-positive-only) · `res-strip-leaves-a-bare-colon` REFUTED by direct strip inspection
(body-statement and case-arm shapes both erase clean) · `mise run test:floor` green on WSL
(134 passed, real dash+posh) · decidable-set widening warning stamped in
`analysis/CLAUDE.md` · the bitem9 trigger shrink ratified into `syntax/CLAUDE.md` ·
28O supersession markers placed · fold-routed items promoted (oracle/CLAUDE.md
bind-principle; TODO-ADDTL posh-leg entry; main.rs apostrophe mangle fixed) ·
`ai/r28-cli-inputs`' two commits cherry-picked (branch tip-redundant, advisory-delete) ·
LIVING_STATUS re-measured. Final acceptance: `mise run both gate:full-quiet` at the close
commit. Open human queue: collated in LIVING_STATUS's CURRENT STATE.

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

### Conductor adjudications at the fold close (2026-07-31)

- **amend-plural-value-loss-hold** — supersedes the routing half of
  `adj-positional-plural-value-loss-carries-to-bitem4` (the ruling half stands): the human
  corrected the seam's granularity (typed, in-chat) — there is NO whole-file analysis unit;
  the ask is indexed per-environment-frame (borderline per-line; abstractable at most to the
  piecewise-constant intervals between env-mutating statements), and the corner is under
  PARALLEL human adjudication (`28M` §10 `dir-ownership-is-transitive-inclusion`, unruled,
  re-keys the fence's `SourceFileId` span proxy to entry-closure identity). Standing order:
  bitem4 + bitem5 + any withhold-softening/re-resolution work are HELD until the human
  replies; nothing builds toward the restructure.
- **adj-never-live-exactness-accepted** — the fold's second seat (`funcenv::never_live`
  subtracting provably-never-live definitions from the whole-unit resolution population) is
  EXACT, not conservative: a wrong subtraction would SHIFT a winner, i.e. grant, not lose.
  Accepted because it is a closed-form proof over decidable-set v0 only, empty-by-construction
  on non-convergence, and wrong-but-consistent holds through bitem1's pinning (the artifact
  carries the answer; runtime cannot re-derive). Standing consequence, permanent: any future
  widening of `28M:dec-decidable-set-v0` now widens a WINNER-SHIFTING surface, not merely a
  disclosure surface — decidable-set growth is license-review-tier, never a convenience patch.
  Fold-close checklist gains: confirm this sentence is in `analysis/CLAUDE.md`, add if not.
- **adj-command-v-blessing-routed-to-human** — builder-3's… builder-fold's
  `tc-command-v-blessing-would-convert-guards-to-elisions` NOT taken: the delivered P1 cell
  guards rather than elides because the polyfill's own `command -v` line is an unmodeled
  running command (an honest wall). Blessing it target-state-pure would upgrade guard→elide —
  a licensure widening. Routed to the human as one design question COVERING ITS FAMILY:
  engine-blessed target-state-pure builtins in BOOK position (`command -v`; also
  `28O:res-book-sourcing-walls-at-the-site`'s `.`-of-a-proven-load-inert-file). Not built.
- **bank-28m-stale-wording-at-merge** — `res-28m-pre-fold-wording-is-stale` (§5.4/§6 pre-fold
  lines) is the SIBLING's document; flagged to the human/sibling rather than edited from this
  lane. The lane's own `28K` §3 disclosure was rewritten in place by the fold builder.

### Conductor adjudications at the builder-3 close (2026-07-31)

- **adj-reach-atomicity-fix-commissioned** — builder-3's `tc-reach-expansion-has-no-atomicity`
  is TAKEN as build work (builder-4): the dynamic `disturbance_reaches_only` lane has NO
  body-death gate, and a truncated reach-closure is the SAME wrongly-narrow-survey hazard the
  human confirmed critical-tier — a downstream fact whose backing lives in an uncovered file
  wrongly survives. The fix does not overrule `an-kind-reach`'s "widens claims only" row:
  that row describes the operator's direction WHEN COMPLETE; atomicity-on-death is
  orthogonal, and the fix moves survival outcomes only toward walls (conservative,
  spelling-free, `body-rc`-shaped like item0's). The exit-0-truncation residue stays the
  human's completion-signal design, unchanged.
- **route-split-family-two-author-elision-to-sitting** — builder-3's
  `tc-split-family-elides-on-two-authors` is COMMITTEE-CORNER ADJUDICATION MATERIAL, routed
  to the human's parallel sitting, nothing changed in-lane: bitem6 measured that a
  member-complementary split family's elision rests on author-1's measurement (predict) plus
  author-2's vouch (verdict) — a two-author license. `28M` §8's "monologue everywhere" was
  scouted in a single-author world and its per-utterance reading survives (each utterance
  has one author; no value crosses provenance); whether the COMPOSITE license needs
  `28M:rul-composite-meets-toward-guard-run` treatment (grant-tier ⇒ one speaker or
  per-author entailment) is exactly the fence sitting's question. The custody typing now
  makes the two-author case visible at the type level — load-bearing input for the re-key.
- **adj-meet-direction-registry-deferred-to-fence-revival** — builder-3's stop CONFIRMED:
  an unwired registry in the license plane reads like a gate, and wiring it spans three
  mechanisms across two crates — past "small" per the brief's own rule. It is `28M` §8's
  machinery and lands naturally with the held fence work; deferred there by name.
- **adj-wrapped-vouch-positional-gate-to-builder-4** — `tc-wrapped-vouch-seat-has-no-
  positional-gate`: the sixth (wrapped-vouch) seat joins the positional regime in builder-4's
  lane — withhold-only, the bitem0 gate's shape, no new mechanism.

### Conductor adjudications at the builder-4 close (2026-07-31)

- **adj-floor-lane-bullet-kept; route-posh-printf-coverage-to-human** — builder-4's
  `spike/CLAUDE.md` floor-differential bullet KEPT verbatim (load-bearing). The
  `tc-inert-mocks-rail-is-dash-shaped` finding routes to the human as a standing gap
  BEYOND this lane: `printf` is not a posh 0.14.1 builtin, so under `PATH=mocks-only` no
  shipped oracle body's emissions have ever been exercised under posh — the corpus half of
  the `kWHICHSH` weld's "dash, posh, and our evaluator perform identically" promise is
  dash-shaped. Gate-9 (`mise run test:floor`) closes it for its own six manifests only;
  corpus-wide posh coverage of emitter bodies is un-commissioned, sized beyond lane scope.
- **res-case-bodied-wrapped-verdict-coverage** — builder-4's measured
  `tc-wrapped-lane-drops-a-case-bodied-in-book-verdict` banked as named residue (coverage
  loss only, never a license); candidate small item for the fence-revival lane.
- **fold-checklist additions** — verify `res-strip-leaves-a-bare-colon-for-a-standalone-mark`
  (low-confidence; if real it violates `strip-is-pure-erasure`'s marks-erase-to-NOTHING
  clause — a stripped-in `:` clobbers the tool-rc to 0; one `dorc strip` inspection settles
  it) · run `mise run test:floor` once on the WSL leg (gate-9 is opt-in; the committed
  `expected.emitted` bytes are otherwise unproven) · confirm the decidable-set-widening
  warning reached `analysis/CLAUDE.md`.
- **adj-wrapped-gate-unreachable-accepted** — the wrapped-vouch gate is unreachable on the
  current corpus (the ship seat degrades first); built anyway on the hash-munge precedent,
  unit-pinned. Accepted.

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

## item0 — the survival-closure consumption measurement (MEASURED; deriv half LANDED)

`28P:adj-survival-closure-gap-measure-then-fix`, the measure-first item ahead of bitem3. bitem1
closed the closure gap in the guard/probe lanes; the survival lanes still ship closure-less bodies,
so the question was what a survival-lane body's death mid-emission does to the at-most claim it was
writing. Measured through the built engine on fixtures, never reasoned: a `disturbs` body that emits
from a payload-bound pipe and then calls a helper the ship lane does not carry.

### tbl-the-four-tiers-measured

The brief's required split — TRANSPORT atomicity (`an-derived-footprint`'s all-or-nothing readback:
did the record stream arrive whole) vs BODY-DEATH atomicity (did the body that wrote those records
finish its survey). They are independent, and only the first was ever gated.

| tier | transport | body-death | as-built |
|---|---|---|---|
| static per-arm `disturbs` | n/a (never leaves the controller) | **ATOMIC by construction** | safe |
| dynamic derived `disturbs` (the deriv lane) | ATOMIC (`deriv-end n=<K>`) | **PARTIAL — measured** | FIXED here |
| `kind__resolve` (the resolv lane) | n/a (one value, not a set) | **ATOMIC by construction** | safe |
| dynamic `disturbance_reaches_only` (the reach lane) | **NO GATE AT ALL** | **PARTIAL — measured** | REPORTED, not built |

Static tier: `evaluate_touches` tops the WHOLE body on any non-printf command (TC-4), so a body that
statically emits two coordinates and then calls a helper discards both and escalates — a partial
static footprint is unrepresentable, not merely unobserved. +SURE (the property is pinned by
`multi_stage_pipeline_and_leading_printf_arm_escalate`, which asserts exactly the printf-then-opaque
shape).

Resolv tier: its scaffold already captures the body's status (`_rr=$?` off a `$()` capture) and
answers `dangling` on non-zero or empty, which canonicalizes to `MayAlias(Unresolved)` then demotes
to run. A resolver that dies fails toward run, loudly. +SURE. Worth noting for the next reader that
this lane was the PRECEDENT for the deriv repair below — the shape already existed one module away.

### fnd-the-count-gate-cannot-see-a-body-death

The measurement, and the reason the deriv lane looked safe. `deriv-end <site> n=<K>` reads as a
completeness gate, and `26A` stop-1 describes it as one — but K is computed by the SCAFFOLD from the
lines it received, not declared by the body about its own survey. The pre-repair emission was a
single pipeline: the body on the left, and on the right a counting group that printed one record per
line received and then closed the family at its own running total.

So a body that emits one coordinate and then dies on an unbound helper closes at `n=1` and AGREES
WITH ITSELF. Transport is intact; the survey is not. And the body's own status was not merely
ignored, it was unreachable: the body sat on the LHS of a pipeline, whose status is its RHS's.

Measured, through `mise run test:e2e`, in two halves. First the runtime half — the shipped
closure-less body produced exactly its pre-helper coordinate and nothing else (gate-1(d) reported
`produced: deriv 0 coord=sm.dorc.File:/etc/oldpkg.conf` against an authored complete set naming the
helper's `sm.dorc.Package:nginx` too). Then the consumer half — feeding back exactly those truncated
records, the wall's footprint was ACCEPTED at its narrow width and the converged `apt-get install -y
nginx` SURVIVED past the running wall and elided, where the complete footprint collides and runs it.
A wrongly-NARROW at-most claim spares more; the site that should have run did not. +SURE, and it is
the priority-1 under-execute inside the survival tier's one naked-trust cell.

### dec-whole-body-atomic-refusal

The repair, deliberately the smallest one that closes the measured cell. The scaffold captures the
emission body's status BEFORE the record pipe (the resolv lane's capture shape) and carries it on the
close record, which becomes `deriv-end <site> n=<K> body-rc=<R>`; `merge_derived_footprints` refuses
the WHOLE family on non-zero `R`, on the same path and with the same outcome as the
malformed-coordinate refusal — no footprint, the site walls total. Whole-body-atomic, failing toward
the wall, and it asks the oracle author for NO new spelling.

Three things it deliberately is not. It is not a completion SIGNAL: a body that truncates its survey
and still exits 0 stays invisible, and that residue is the human's open design
(`ANALYZER-NEEDS:an-atmost-completion-signal`) — nothing here builds toward it. It is not a verdict
rc: `rul-rc-partition`'s 0/1/>=2 table binds verdict functions, and this reads a binary
did-the-body-finish, which is why the wire spells it `body-rc=` rather than joining the site record's
`rc=` (`rc-naming-discipline` — two different rcs one field-name apart is exactly the confusion that
ban exists for). And it is not a closure fix: the helper still does not travel. Closing THAT is the
value-add half of the adjudication and is now unblocked, but it is a widening, and a widening is
never what should land in front of a measured under-execute.

Honest cost, disclosed: the refusal also fires where a body legitimately finished its survey and
merely ended on a failing command (an author whose last pipeline stage exits non-zero). That loses
sparing — walls — never a license. Conservative direction, zero corpus instances.

The two gates are now independent and both necessary, which is the shape the measurement asked for:
`count` proves the stream, `body_rc` proves the body. `results::DerivClose` carries them as one typed
close record so a future reader cannot satisfy one and believe both.

### fnd-the-reach-lane-has-no-completeness-gate-at-all (REPORTED, not built)

Measured on the same footing and NOT repaired — flagged below as `tc-reach-expansion-has-no-atomicity`
because repairing it would overrule a built doctrine, which is not a builder's call.

A DYNAMIC `disturbance_reaches_only` arm runs under the identical pre-repair scaffold — the arm piped
into a per-line record printer, its status discarded by the same pipeline rule — and its consumer
(`expand_footprints_via_reaches`) reads the readback through `.unwrap_or_default()`. There is no close
record, no count, and therefore not even the transport-tier gate the deriv lane had. Measured: with
the reach family absent from the stream — the wire shape a truncated or aborted arm produces —
`strawman24-reach-crossauthor`'s downstream `installfile /etc/nginx/nginx.conf` stopped running and
elided, silently, with no diagnostic. That is the same cardinal sin: reaches EXPAND a footprint, so a
missing expansion narrows it, and narrow spares more.

Why it is not repaired here. `ANALYZER-NEEDS:an-kind-reach` is a BUILT (status B) doctrine reading
"widens claims only — the safe direction", and the reach scaffold's own doc-comment documents an
un-shimmed arm's silent empty expansion as safe on that basis. The measurement says that reading is
wrong whenever the disturbs claim is not independently total — which is precisely when a kind-owner
`reaches_only` is needed at all. Correcting it changes survival OUTCOMES for existing corpus cases
(every arm that fails or 127s would begin refusing its footprint) and contradicts a ruled row, so it
is a doctrine correction rather than a bug fix (`inv-superposition`: a cross-cutting judgment is
flagged UP, never settled inside a component).

Sizing for whoever takes it, since the mechanism is now built once and can be copied: the scaffold
edit and a `reach-end <coord> arm=<n> n=<K> body-rc=<R>` record are the same shape as this item's,
but the REFUSAL SEAT does not exist — `TrustedFootprints::expand_reaches` hands its closure no node
key and offers no removal, so the closure cannot wall the footprint it is expanding. That is the real
cost (a small signature change in `plan::survival` plus the cli consumer), and it is why this is a
lane item rather than a rider.

### Golden churn, predicted vs actual

Predicted before building: the scaffold bytes move in every case that compiles a derivation, and
nothing else — the repair only ever ADDS a refusal, and no corpus body terminates abnormally.

Actual: exactly that. Two cases, both content-only, each blessed scoped and inspected:
`strawman24-derived-survive` (the transcript's two scaffold lines; its apply artifact is
byte-identical, correctly — its body exits 0 and its footprint is complete) and the new pin. No
site's verdict, license, or disposition moved anywhere in the corpus.

| cause | cases | delta |
|---|---|---|
| scaffold bytes in a committed transcript | 1 | probe bytes only; apply artifact byte-identical |
| net-new | 1 e2e loom + 1 unit cell | additive |
| behavioural drift elsewhere | 0 | full e2e: 118 passed, 2 content diffs, nothing else |

### The behaviour pin: `pin28-survival-body-death-walls-total`

The consequence cell end to end, and it is worth reading over the unit test because the committed
transcript SHOWS the two atomicities pulling apart: the probe carries `deriv-end 0 n=1 body-rc=127`
— a family whose count is perfectly self-consistent and whose body never finished — and the apply
artifact runs BOTH installs. Non-vacuous by construction: before the repair this exact fixture elided
the nginx install, which is how the hole was measured in the first place.

The unit-tier twin extends `pin_partial_deriv_family_demotes_to_wall_total` with a
transport-perfect/body-dead cell, deliberately sharing the stream builder with the count cells so the
two gates are asserted over identical bytes.

### The aid surface (`Words::Unwritten`; no new code, no ceiling bump)

- **`footprint-incoherent-emitting-body-died-mid-survey`** — a new `FootprintIncoherentReason`
  variant beside `MalformedDerivedCoordinate`, carrying the body's termination status (127 = a helper
  the shipped body did not carry — the actionable datum). It mints NO new `DiagCode`: the refusal is
  a footprint-coherence failure, which `footprint-incoherent` already owns, and the typed-reason enum
  is the shape `28L:rul-reason-enums-not-sibling-codes` requires. So the unwritten CEILING is
  untouched — `28O:res-unwritten-ceiling-spent` and bitem1's second bump both stand unspent by this
  item. Words are `Unwritten` (hand-seeded arrangement row, the sanctioned carve; builders author
  zero prose).

  Sited under `footprint-incoherent` rather than `deriv-family-incomplete` ON PURPOSE, and the choice
  is the measurement's whole point: `deriv-family-incomplete` is the TRANSPORT code, and filing a
  body-death under it would tell the author their stream was cut when their stream was perfect.
  Mis-attribution is the sin that outranks the others (`271:rul-sin-ordering`).

### Flagged upward

- **`tc-reach-expansion-has-no-atomicity`** — `fnd-the-reach-lane-has-no-completeness-gate-at-all`,
  above. Measured under-execute; unrepaired because the repair overrules `an-kind-reach`'s built
  "widens claims only — the safe direction" row and moves existing survival outcomes. Wants a
  conductor ruling, then the mechanism copies from this item.
- **`res-survival-lanes-still-ship-closure-less`** — the adjudication's value-add half
  (`adj-survival-closure-gap-measure-then-fix`: "if atomic, extend closure capture there") is NOT
  taken, because the answer was PARTIAL, not atomic, and the refusal was the ruled first item. The
  helper still does not travel with a `disturbs`/`resolve`/`reaches` body; what changed is that its
  absence now walls loudly instead of silently narrowing an at-most claim. Extending bitem1's
  `HelperIndex` to the three survival ship seats stays cheap and stays somebody's dispatch
  (`cli/CLAUDE.md one-helper-index-two-lanes` already names it).

## bitem3 — custody and monologue pins (LANDED, one sub-thread STOPPED)

`28K` §10 / `28M` §8's "Hardening [PROPOSED]", made real. Strictly pin-shaped: typing a monologue
that already exists, never extending resolution.

### dec-custody-is-one-newtype-and-one-crossing

`28M` §8 found the license plane monologue EVERYWHERE but emergently — three unrelated mechanisms
agreeing (lane exclusivity · the establish-⊤ firewall · consumed-⊤ forbids-mint), none of which
names custody. `core::DefinitionCustody` is that name: a newtype over `SourceFileId` with ONE
constructor, and consumers that only ever COMPARE custodies, never read the file id to decide
anything. That is the whole design constraint, and it is `28M` §10's doing rather than tidiness — if
`dir-ownership-is-transitive-inclusion` re-keys custody from the defining file to an entry file's
transitive sourcing-closure, the re-key is a change to this type's internals and nothing else.
Nothing NEW keys off a raw `SourceFileId`; `defining_file()` exists for provenance and display and
says so.

The second half is `funcenv::custody_of_source_index` — the ONE crossing from a positional
vector-index into the custody vocabulary. The agreement gate had been comparing bare `usize`
indices, which is why `28P:adj-positional-gate-is-bitem3s-seam` called it a FOURTH untyped mechanism
holding the monologue: the gate and the license mint were two spellings of one question — whose
definition is speaking here — with nothing making them the same question. They are now one type and
one comparison, and `build_vouches` takes the license's custody from the SAME index the gate
admitted, so the two cannot drift.

### dec-license-custody-names-what-a-widening-would-have-to-add

`core::LicenseCustody` on `ReplaceLicense`, stamped by every mint. Three variants, and the third is
the one that does the work:

- `Vouched(DefinitionCustody)` — a converged-establish elision, read OFF the consumed vouch rather
  than passed beside it, so a license cannot be stamped with a custody its vouch did not supply.
- `VouchedSeverally` — an aggregate erasure (member-loop, inlined call): several establishes vanish
  under one license, each carrying its own author's reached vouch, cardinality-matched
  (`rul-every-erased-establish-is-vouched`). A CONJUNCTION of monologues, admissible for that
  reason, and named apart anyway because the day anything reads ACROSS those establishes rather than
  conjoining them, that read is a dialogue and it will be sitting here.
- `MeasuredSelf` — a read-only Query substitution, which rests on no authored vouch at all: its
  reproduced value is the probe's own measurement OF THE SUBSTITUTED COMMAND, so there is no second
  speaker. Documented as unusable the moment reproduction reaches beyond that cell.

This is where "re-entry becomes a type error" actually lives. A widening that reproduced a value
measured by a DIFFERENT author's `predict` under this author's license fits NONE of the three, so it
cannot be written without adding a variant — a visible, reviewable act in the one file that defines
what custody means, rather than a quiet edit at a mint site. The named re-entry routes (`28M` §8's
unground declared-rc opt-in comment, and any widening of measured-value reproduction) both land
exactly there.

### fnd-the-wrapped-vouch-seat-resolved-forwards

Found while threading custody, not by reasoning about it, and it is the same fault one seat further
out than bitem2 reached. `build_wrapped_vouches` picked the inner verdict with
`verdict_sets.iter().find_map(...)` — a FORWARD scan, i.e. FIRST-definition-wins, which is the
INVERSE of sh's own answer and precisely
`28M:fnd-verdict-resolution-duplicates-live-source` at a sixth seat that
`oracle/CLAUDE.md live-source-is-the-only-resolution-seat` does not list. Retired for
`dorc_oracle::live_source`, which also asks only whether a file DEFINES the role rather than whether
its body answers this argv (the retired decline-fallthrough cascade, `28K` §6).

Unreachable today for the same reason bitem1's hash-munge is: a contested family is withdrawn before
it arrives, and a non-contested plural one needs the fold to leave two live definitions standing.
+SURE it could not mis-bind on the current corpus; the point is that it was one silent seat away
from doing so, and the class had already been found twice.

Still MISSING at that seat and flagged below: the positional agreement gate. `build_wrapped_vouches`
receives no `LiveDefinitions`, so it resolves whole-unit and never asks whether the winner is live
AT the wrapped site. Custody there is honest about WHO but not about WHERE.

### dec-the-stdout-firewall-is-structural-too

`28M` §8 named the stdout parallel of the rc firewall ~SUSPECT and untraced; the brief asked to
trace it and pin only what the trace found missing. The trace: the property HOLDS — an establish
site's out-channels never carry a believed value — but not by the rc firewall's mechanism. The rc is
withheld structurally, keyed on `ProbeSiteKind::Establish`. The out-channels were held by two
accidents instead: nothing emits `stdout=` today, and `consumption_ok` blocks a consumed stdout
unconditionally without ever reading the value. So what was missing is the firewall itself, and that
is what landed — an Establish site's out-channels are ⊤ by CONSTRUCTION now, on the same line and
for the same reason as its rc (the probe never ran the mutator, so the mutator's own observables
cannot have probe-provenance). Inert today; the point is that it stays true when the values stop
being ⊤.

The whole firewall also moved into one named seat (`measured_channels`) rather than two `match`es on
one discriminant forty lines apart — a reader could previously satisfy themselves about the rc
without ever meeting the out-channels.

### The three commissioned pins

`an_establish_elide_speaks_for_its_vouching_author_and_a_query_for_none` (custody follows the vouch,
across two different authors, and a Query names none) · `a_split_family_establish_elide_reproduces_
nothing_predict_derived` (the elide's stand-in is `True`, reproducing no measured value — pinned at
the mint because the intake firewall and the mint are separate crates and neither alone states it) ·
`the_vouch_covers_the_stand_in_rc_zero_only_where_no_consumer_can_tell` (the pair: rc-0 rides where
nothing reads it, and the SAME inputs refuse with a ⊤-status branch consumer — if that half ever
passed, the stand-in's rc 0 would be a fabricated success suppressing a `|| fallback`).

### STOPPED: the meet-direction registry

`28M:rul-composite-meets-toward-guard-run`'s buildable piece, invoked under the brief's own
stop-rule rather than built. A `MeetDirection` table over properties is cheap to WRITE and worthless
unless it is CONSULTED where claims meet, and the meet sites are `Must`/`May` (`analysis/lattice.rs`),
the `Flat<T>` ⊤-domain, and `core::coord::compare` — three different mechanisms in two crates whose
meet-direction is currently convention-held in their type choices. Wiring a registry through them is
a lattice refactor, not a table; leaving it unwired would put a decorative structure in the license
plane that READS like a gate, which is worse than the convention it replaces. Flagged below with
that sizing rather than half-built.

## bitem6 — the commissioned composition suites (BOTH LANDED; the verdict is COMPOSES, with one finding)

`28M` §7/§8, human-typed. These MEASURE; the answers matter more than the cases.

### res-cross-file-helper-composition-works

The conductor's standing ~SUSPECT — "the current check-dialect / lift refuses or tops non-role calls
inside verdict bodies" — is REFUTED, measured through the built engine in both lanes.
`pin28-helper-package-entrypoints-lift` is the required package shape (a helpers file carrying the
bulk of the logic under non-role names, plus ONE thin entrypoints file carrying the `__role`
collision surface) and it lifts with its cross-file closure intact: the constant and BOTH helper
hops ship above the check in the probe and above the book in the apply preamble, and the mocked run
reaches `wombat cmp` through two hops, which an unresolved helper at either hop would have turned
into an rc 127 and an empty run-log.

Three things make the result stronger than the bare pass. The closure is TRANSITIVE (the entrypoint
calls `_wombat_same`, which calls `_wombat_dest`) — resolution is not one hop deep. The load order
is adversarial: the entrypoints file sorts BEFORE the helpers file, so the helper is declared in a
file loaded AFTER the body that calls it, which a prefix-scanning or defining-file-only closure
would miss. And the helpers file has no `__` anywhere and defines only functions, which is exactly
the shape that errored `predict-out-of-dialect` at its first funcdef until bitem1's
`fnd-a-helpers-only-file-was-refused-out-of-dialect`; this case is now that fix's corpus guard.

`pin28-helper-package-entrypoints-discarded` proves the other half the human demanded — that the
entrypoint file can be discarded without losing the helpers' function. The spelling chosen is the
bluntest of the three offered: the entrypoints file is simply not loaded, and the BOOK supplies its
own `wombat__is_converged` over the package's helpers. That is stricter than swapping the file,
since the replacement is not an oracle file at all, and it lands two properties at once — a book is
a first-class definition source whose closure resolves into a DIFFERENT author's file, and the
helpers cannot collide with what the admin wrote because they carry no `__` names. The guard mints
from the book's entrypoint with the package's helpers in its preamble.

So the packaging shape that makes custody cheap works, in both directions, and it is now standing
corpus rather than an argument.

### fnd-a-split-family-elides-on-two-authors

<!-- /* superseded: resolved at the license tier by 28Q §4 rul-verdict-primacy-at-the-ship-seat (stage-0: the verdict body ships and answers — a monologue); sparing-tier residue stays the fence sitting's; see Research/plans/28Q §4 */ -->

The other commissioned suite, and it did NOT confirm what it was written to confirm — which is why
it was worth commissioning. `pin28-split-family-lane-separation` puts `apt_get__predict` in one
author's file and `apt_get__is_converged` in another's (different MEMBERS, no shadowed name, so
nothing contests — `28M:fnd-split-family-composes-unrefused`) and gives the site a resolvable predict
AND a reached vouch.

Measured: the PREDICT lane ships. The verdict lane is reached only as a FALLBACK
(`verdict-lane-is-site-keyed`: nothing resolved the argv, or something resolved it but declared no
cells for the verb), and here author one's predict resolves the cell — so author one's body runs, its
rc becomes the `effect=holds`, and author TWO's verdict function is never executed at all. Its
contribution is the vouch, admitted because their own argparse accepted this argv statically. The
site elides.

The elision therefore rests on two authors: a measurement from one, a permission from the other.
`28M` §8's scouted "Ship lanes are mutually exclusive per site: a vouched site ships the VERDICT
body, and convergence derives solely from that body's own rc" is true of the single-author world it
was scouted in and NOT of this one. That is the configuration the suite was commissioned to create,
so the finding is the deliverable rather than a failure of it.

+SURE of the mechanism (the transcript ships the predict body; the run-log never reaches the
verdict's `aptcheck`; a vouch was nonetheless consumed, since `no_license_for_ambient_without_vouch`
pins that an unvouched ambient establish does not elide). NOT ruled on admissibility: it is not
obviously wrong — author two consented to this argv through their own arms, and author one's model
is factual, which is the ordinary fact-plus-vouch architecture. But it is not a MONOLOGUE, and
bitem3's custody now says so out loud: the license reads `Vouched(author two)` over a fact author one
measured. Flagged as `tc-split-family-elides-on-two-authors`.

<!-- /* superseded: resolved at the license tier by 28Q §4 rul-verdict-primacy-at-the-ship-seat (stage-0: the verdict body ships and answers — a monologue); sparing-tier residue stays the fence sitting's; see Research/plans/28Q §4 */ -->

## bitem7 — renames and small riders (LANDED)

`WhyReport.oracle_paths`/`oracle_srcs` → `source_paths`/`source_srcs`. The rebase widened them
source-wide (the binary fills them from `source_table`, book included, since `28K` §2a made a book a
first-class definition source), so the names understated their contents. No aliases
(`rul-strawman-formats-no-compat`).

Module-driven, not grep-driven (`rul-host-evidence-is-not-the-narrative-plane` demands it, and the
demand earned itself here): of 187 workspace occurrences of those two names, most are NOT this
vector. `RunSources.oracle_paths` and the whylog/attempt-scope record are genuinely oracle-only —
they are about the oracle SET as such — and a blind rename would have made them lie. The rename is
confined to the `WhyReport` field family and its one construction site.

### res-the-why-world-cut-is-now-visible

bitem7's second rider, and the rename is what made it cheap: `WhyWorld` fills those two fields
ORACLE-only while the binary fills them source-wide, so after the rename the field names say
`source_` and the values at that seat are not. The mismatch is now annotated in place as the
disclosed cut (`churn-avoidance-disclosure`), with the coincidence stated: the seat agrees with the
binary today only because nothing in the corpus resolves a locus to a book-sited definition, so it
withholds where the binary would answer — safe direction, identical output on everything that
exists, and a property of the current vector choice rather than of the design. Full unification was
priced and declined: it means re-lifting that seat's whole world, which is a dispatch and not a
rename (`28P:res-why-world-lifts-no-book-definitions` · the fold builder's
`res-whyworld-and-survival-do-not-withdraw`).

The optional `entrypoint-only-constants-under-deep-require` lint is SKIPPED, per the brief's
"skip if it drags" — it drags: the constant-capture story is `dec-constants-ride-per-contributing-file`,
whose whole point is that a reference-driven capture cannot prove itself complete while
`ParamComplex` discards the name, and a lint over that surface would be the imperfect mechanical net
`271:rul-net-quality-u-curve` warns against.

## bitem8 — the differential load-order battery (NOT STARTED)

Ruled stage H, untouched: no sentinel-body load-order manifests, no `command -v` PATH-reach-vs-
fn-definedness case, no `||`-operand funcdef parse question. It is the lane's remaining commissioned
work and it is intact — nothing here half-built it, and nothing here depends on it. Note for whoever
takes it that its `command -v` case is now the SOLE pin of `28M:rul-command-v-reads-fn-definedness`'s
divergence cell (the function-only-spelling scout came back negative), so it carries more weight than
its size suggests.

## Flagged upward (bitem3/6/7)

- **`tc-split-family-elides-on-two-authors`** — `fnd-a-split-family-elides-on-two-authors`. A
  measured, standing configuration in which an establish-elide's convergence FACT and its VOUCH come
  from different authors, contradicting `28M` §8's scouted monologue claim. Not adjudicated here:
  whether the composite is admissible is a license-plane ruling, and `28M` §8 explicitly reserved
  the custody-vs-coherence question for the human ("why custody rather than a coherence check, if
  reproduction ever widens"). The custody type makes it visible; it does not decide it.
  <!-- /* superseded: resolved at the license tier by 28Q §4 rul-verdict-primacy-at-the-ship-seat (stage-0: the verdict body ships and answers — a monologue); sparing-tier residue stays the fence sitting's; see Research/plans/28Q §4 */ -->
- **`tc-wrapped-vouch-seat-has-no-positional-gate`** — `fnd-the-wrapped-vouch-seat-resolved-forwards`
  closed the forward-scan half at that seat, but `build_wrapped_vouches` still takes no
  `LiveDefinitions` and so never asks whether its whole-unit winner is the definition live AT the
  wrapped site. Every other resolution seat does (`28P:tbl-the-five-seats-after-the-conversion`).
  Threading it is a parameter and a filter, i.e. small; it is flagged rather than done because it
  makes a SIXTH seat of the positional regime and `bitem0` deliberately enumerated its seats.
- **`tc-meet-direction-registry-not-built`** — see `STOPPED: the meet-direction registry`. The
  buildable piece is a table; the VALUE is in consulting it at the meet sites, which are three
  mechanisms across two crates. Wants either a real (lattice-tier) dispatch or an explicit ruling
  that a convention-held direction is the resting point.

## reach-atomicity — the second consumer of item0's mechanism (LANDED)

`28P:adj-reach-atomicity-fix-commissioned`. The dynamic `kind__disturbance_reaches_only()` lane had
no body-death gate — and, measured by builder-3, no transport gate either — so an arm that died or
was cut simply contributed no expansion, and the un-widened at-most footprint spared a site that
should have run. Repaired on item0's shape, deliberately not a second mechanism.

### dec-reach-expansion-refuses-whole-footprint

The repair, and the two decisions inside it worth defending.

WHAT CLOSES: the scaffold captures the arm's status BEFORE the record pipe (`_r=$(<arm> <entity>);
_rr=$?`) and closes with `reach-end <coord> arm=<n> n=<K> body-rc=<R>` — the deriv close's grammar
at a second key. Three refusal triggers, all independent and all typed: no close record at all
(the wire shape builder-3 actually measured), a count that disagrees with the records received, and
a non-zero body-rc. Framed streams only, exactly as the deriv gate is; the legacy unframed fixtures
carry no closes and stay trusted-complete.

WHAT IS REFUSED: the WHOLE footprint, not the coordinate. A `reaches_only` survey is
complete-by-contract, so an arm that cannot show it finished leaves the claim wrongly NARROW, and
narrow SPARES MORE — refusing only the arm's own coordinate would leave exactly the partial at-most
claim the whole gate exists to forbid. Refusal is immediate per footprint (the remaining coords are
not expanded either) and removes the entry from `TrustedFootprints`, so the site walls total on the
one path where absence already means wall.

`DerivClose` became `EmissionClose` and both lanes read it. The brief's "one mechanism, two
consumers, don't fork it" is a claim about the TYPE as much as the shape: two structurally
identical closes under two names is how the two gates drift apart later.

### dec-the-refusal-seat-is-a-return-value-not-a-node-lookup

The seat builder-3 sized as "a small signature change in `plan::survival` plus the cli consumer".
Built slightly smaller than that sizing: `expand_reaches`'s closure now answers a typed
`ReachExpansion` (`Expanded(coords)` | `Refused`) and the node key is threaded ALONGSIDE it rather
than instead of it. The closure never removes anything — `TrustedFootprints` owns its own map and
does the removal after the walk — so the one seat that can shrink the survival tier's data stays
inside the type that holds it. The node is threaded only so the cli can SPAN its diagnostic at the
wall site (`aid-caret-span-precision`), which is the same reason `merge_derived_footprints` takes a
`node_spans` map; `TrustedFootprints::nodes()` is the small accessor that feeds it.

An empty `Expanded` therefore means "this coordinate reaches nothing", an ANSWER, and is now
type-level distinct from "the survey could not be trusted". Before the repair those were the same
value, which is precisely why the hole was invisible.

### The aid surface (`Words::Unwritten`; no new `DiagCode`, no ceiling bump)

Three new `FootprintIncoherentReason` variants beside item0's — `ReachArmNeverClosed`,
`ReachArmStreamCut`, `ReachArmDiedMidSurvey` — each with its own hand-seeded arrangement row. NO new
code, on item0's own reasoning: the refusal is a footprint-coherence failure and
`footprint-incoherent` already owns that, and N same-world reason-sentences are COMPONENTS
(`28L:rul-reason-enums-not-sibling-codes`). So the unwritten CEILING is untouched at 19 — neither of
the lane's two conscious bumps is spent by this item.

They are three variants rather than one because the attribution differs, and mis-attribution
outranks the other sins (`271:rul-sin-ordering`): telling an author their arm body died when their
stream was cut is the same error item0 avoided by siting body-death under `footprint-incoherent`
rather than `deriv-family-incomplete`. Note the reach lane has no transport-tier CODE of its own and
does not want one — a reach arm is not a family the site owns, it is a kind-owner's survey the
engine applied to somebody else's footprint, and refusing that footprint is the whole consequence.

### Survival-churn accounting

Predicted before building: the scaffold bytes move in every case that compiles a reach probe, and
nothing else — the repair only ever ADDS a refusal, and the corpus's one reach arm exits 0.

Actual: exactly that, and the corpus has exactly ONE such case.

| cause | cases | delta |
|---|---|---|
| scaffold bytes in a committed golden | 1 (`strawman24-reach-crossauthor`) | probe bytes only; its apply artifact is BYTE-IDENTICAL, and its downstream site still demotes |
| survival outcomes moved | 0 | no corpus arm terminates abnormally or fails to close |
| net-new | 2 e2e looms + 1 unit cell + 2 record-grammar cells | additive |

The authored-fixture harness SYNTHESIZES a clean `reach-end` for any fixture that spells `reach`
records without one, exactly as it already did for `deriv-end`, so authoring alone never trips the
new gate and a case exercising it spells its own close.

### The behaviour pins: an A/B pair, one field apart

`pin28-reach-arm-survey-complete-spares` and `pin28-reach-arm-death-walls-total` are byte-identical
except for `body-rc` in one record, and the admin's `installfile` goes from elided to run. The pair
is two cases rather than one book because the refusal makes the wall total, and a total wall would
guard everything below it for a reason unrelated to what is being pinned. Non-vacuous BY
CONSTRUCTION rather than by argument: the A half's elision is real and committed, and it is exactly
what the B half's refusal costs.

The unit twin `pin_reach_arm_atomicity_refuses_the_whole_footprint` runs all four cells (complete /
dead body / cut stream / never closed) through the PRODUCTION deframer over one stream builder, so
the independent gates are asserted over identical bytes.

### `an-kind-reach` is now status B, not B-with-an-open

The row's "widens claims only — the safe direction" is kept and RE-SITED: it describes the operator
WHEN COMPLETE. The row now carries the atomicity clause and drops its `(dynamic-arm atomicity OPEN)`
status. The exit-0-truncation residue is NOT closed and is not this item's — a body that truncates
its survey and exits 0 stays invisible to both lanes, and that is
`ANALYZER-NEEDS:an-atmost-completion-signal`, human-owned. Nothing here builds toward it.

## wrapped-vouch-gate — the sixth seat joins the regime (LANDED)

`28P:adj-wrapped-vouch-positional-gate-to-builder-4`. `build_wrapped_vouches` now filters its
`live_source` answer through `live.answers_at(node, …)`, the bitem0 shape, through bitem3's one
custody crossing. Withhold-only; no re-resolution; no new mechanism.

### dec-the-wrapped-seat-takes-the-drivers-sets

The part that was NOT in the brief and had to be done anyway. The seat re-lifted `VerdictSet`s from
raw source text, which is `28P:fnd-build-vouches-relifted-the-verdict-sets` one seat further out —
and adding the gate ON TOP of a re-lift would have MANUFACTURED that bug rather than merely left it:
the re-lifted population still contains the contested and never-live definitions every other seat
dropped, so `live_source` could pick a file the run had withdrawn and the new gate would then refuse
the vouch — a silent wall nothing else in the run agreed with. So the seat takes the driver's
withdrawn sets, `oracle_srcs` leaves its signature, and all six seats now resolve over one
population and narrow on one rule. `oracle/CLAUDE.md live-source-is-the-only-resolution-seat` is
updated to say so.

### fnd-the-wrapped-gate-is-unreachable-today

Honest, and the same shape bitem1 recorded for hash-munge. A wrapped site above its definition never
reaches this seat at all: `resolve_inner_check` is already positional (bitem0), so it answers `None`,
the site becomes `WrappedProbe::Degrade`, and the vouch builder skips Degrade sites before the gate
can speak. The gate therefore cannot change an outcome on any world reachable today.

It is built anyway for bitem1's reason: the seat was safe by a SIBLING's agreement rather than by
its own answer, which is the "emergent, not typed" shape `28M` §8 complains about, and the two seats
are one widening apart from disagreeing. +SURE it cannot mis-bind on the current corpus.

### fnd-the-wrapped-lane-cannot-lift-a-case-bodied-in-book-verdict

Found while building the fixture, not by reasoning, and reported rather than chased. An in-book
`hork__is_converged` DOES reach the wrapped lane and ship — but only in the plain-delegation body
shape. The same function written as `case "$1" in install) hork query "$2" ;; *) return 2 ;; esac`
resolves to nothing at all at a wrapped site (measured through the built binary: both sites
`unresolvable-no-probe`, with the dial widened, with and without an oracle loaded), while the
byte-identical body in an ORACLE FILE ships fine. So the divergence is book-vs-oracle for the
CASE-bodied shape specifically. ~SUSPECT it is the same `res-why-world-lifts-no-book-definitions`
family (a seat reading an oracle-only vector) rather than anything positional; NOT diagnosed
further, because it is outside this item and the fixture had a working shape available. Worth a
named lane: the case-bodied verdict is THE canonical authored shape, so a book-sited one silently
answering nothing under a wrapper is a real coverage hole.

### The behaviour pins

`pin28-wrapped-vouch-answers-at-a-live-site` (definition first ⇒ the sudo-wrapped site elides, empty
run-set) and `pin28-wrapped-vouch-withholds-above-its-definition` (the SAME two statements swapped ⇒
`site:0 unresolvable-no-probe`, the line runs verbatim, `role-defined-below-its-sites` fires). Two
cases for the same reason the reach pair is two: the un-licensed site runs, and a running site is a
wall.

### Golden churn

ZERO. No existing `expected.out`, `expected.ran`, or loom transcript moved a byte across either
change — the expected result for a corpus whose wrapped verdicts all live in oracle files, where
positional and ambient are the same answer.

## bitem8 — the differential load-order battery (LANDED; the model held everywhere)

Ruled stage H. Six sentinel manifests, and the headline is that every predicted answer was
CONFIRMED by both floor binaries: `dash 0.5.12` ∩ `posh 0.14.1`, measured under WSL where both are
installed. Nothing in the load-order model needed correcting.

### The lane, and why it is a gate rather than an instrument

`mise run test:floor` / `DORC_E2E_FLOOR_SHELLS=dash,posh`, the SECOND opt-in real-invocation lane
after `real-tools-lane-opt-in`, with the same discipline: default UNSET ⇒ zero invocations,
listed-but-absent ⇒ loud refusal. A case opts in by carrying an `expected.emitted` section — no new
frontmatter key, because the flat-tree law already classifies by SHAPE and a section's presence is
the existing idiom. gate-9 strips the book (`276:rul-spec-two-binary-floor`'s own prescription:
strip-then-run-under-both IS the executable off-ramp test), runs it under each named binary, and
requires them to agree with each other AND with the committed bytes. Disagreement BETWEEN the
binaries is its own verdict — the construct is outside the base dialect — which is what made the
`||`-operand case safe to measure at all.

Resolution goes through a new `internal_tooling::Posix::floor`, beside `find` rather than in the
runner (`one-shell-answer`: a second copy is how the first rotted). It differs from `find` in
exactly one way and deliberately: `find` wants any POSIX shell and will take `sh`, while this wants
a NAMED binary and refuses rather than substituting — a differential answered by the wrong shell is
worse than one not run.

### fnd-printf-is-not-a-builtin-in-posh

The measurement that nearly made the whole battery read as a floor disagreement, and the finding
with reach beyond this item. `printf` is a BUILTIN in dash 0.5.12 and an EXTERNAL COMMAND in
posh 0.14.1 (measured: `env -i PATH=/nonexistent posh` ⇒ `printf: not found`, rc 127; dash ⇒ fine).
Under the corpus's ordinary `PATH=mocks-only` rail, a posh body therefore emits NOTHING AT ALL.

Two consequences. Local: this lane alone joins the floor binary's own userland to the mocks, scoped
to gate-9, rail otherwise intact. Standing, and flagged below: the corpus's inert-mocks rail is
DASH-SHAPED, so no oracle body's `printf` emissions — which is every `disturbs`, `reaches` and
`resolve` body we ship — have ever been executed under posh. The `printf`-doctrine
(`dialect-quality-law`) is sound and this does not dent it; what it dents is the belief that the
existing corpus exercises the floor.

### tbl-the-six-shapes-measured

| case | shape | dash 0.5.12 | posh 0.14.1 | model |
|---|---|---|---|---|
| `floor28-load-order-last-definition-wins` | both orders, isolated in subshells | `b` / `a` | same | CONFIRMED (`28K` §1 rul-sh-loads-dorc-reads) |
| `floor28-unset-f-and-redefinition` | removal · post-unset redefinition · unset-of-absent | `gone` / `b` / ok | same | CONFIRMED (the blessing is behaviourally what `28K` §1 says) |
| `floor28-subshell-scoped-re-source` | preference dies at the `)` | `a` / `b` / `a` | same | CONFIRMED (`28K` §1 rul-scope-by-subshell-resource) |
| `floor28-define-if-absent-polyfill` | P1 and P2 textual orders | `real` / `real` | same | CONFIRMED — the guarded incoming definition cannot override in EITHER order ("exempt as a consequence, not a blessing") |
| `floor28-command-v-reads-fn-definedness` | one spelling, three species | `fn: yes` / `path: yes` / `absent: no` | same | the DIVERGENCE, pinned (below) |
| `floor28-funcdef-as-or-operand` | a funcdef as the RHS of `\|\|` | parses AND behaves | same | measured, documents only (below) |

### res-the-command-v-divergence-is-pinned-with-its-consistency

The contract's sole divergence cell (`28M:rul-command-v-reads-fn-definedness`; the
function-only-spelling scout came back negative, so `command -v` has to carry a question narrower
than it answers). Measured: one spelling reaches a defined FUNCTION and a PATH EXECUTABLE alike, and
`dec-decidable-set-v0` reads only the first — a name the analysed unit DEFINES.

Pinned as the brief required: the divergence AND its consistency, never its absence. The divergence
runs in the safe direction (an undecided condition loses precision and can never mask an arm a shell
would take), and it is tolerable rather than a bug because the analysis's answer is PINNED into the
artifact (`28K` §4) rather than re-derived by the landing shell — `28K` §5
`pattern-carry-the-answer`, wrong-but-consistent. The case states all of that in its own prose,
because the next reader of that cell will be reading the case and not this ledger.

### res-the-or-operand-form-parses-across-the-floor

Measured, both binaries agreeing, and BOTH halves: the terse guard form parses, and it also behaves
as the polyfill idiom intends (the definition lands only in a free slot; an existing one is
untouched). Dorc's own parser accepts it too — the case runs the full round-trip with no diagnostic.

THE RULING IS UNCHANGED BY THE ANSWER, and the case says so: the `if` form is canonical regardless.
This documents; it never licenses. Worth recording that the answer was not the expected one — a
function definition is a compound command and the natural guess is that it may not be an operand
there, which is exactly why the question was on the list to MEASURE rather than to reason about.

### Where it actually executed, honestly

Both legs run gate-9, and they measure different things:

- WSL (`dash 0.5.12` + `posh 0.14.1`): the REAL floor. The full 133-case e2e corpus is green with
  the lane on, and all six differential cases pass under both binaries.
- Windows (`mise run test:floor`): a HALF floor. `posh` is not in git's userland, so the lane
  resolves `dash` alone and says so in the task description. Not a substitute for the WSL run.

The default gates on both platforms leave the lane OFF, so the committed `expected.emitted` bytes
are proven by the opt-in run only — the same standing as the real-tools lane's assertions, and
disclosed here rather than implied.

## bitem9 — value-flow source targets (LANDED; the item was in a different crate than its brief)

Ruled stage C2, last. `28K` §1 `rul-unloadable-is-unlicensed`'s richness half is delivered:
`LIB=./oracles; . "$LIB/yum.sh"` now loads exactly as the literal spelling does. The headline is
that it took ZERO lines of `funcenv`: the domain was never the blocker.

### fnd-the-refusal-was-in-the-parser-not-the-domain

The measurement that reframed the item, taken before any code was written, and it inverts the
brief. `funcenv`'s `load_sites`/`command_transfer` already read their target through
`SourceLiteralPlane`, and the value plane already resolves `"$LIB/lib.sh"` to a concatenated
literal — so the domain would have answered all along. It never got the chance: `dorc_syntax`'s
`check_simple_triggers` ⊤-rejected `.`/`source` of any non-literal target as
`DynamicExecution`, so the statement became an `Unsupported` node, lowered to `CfgNodeKind::Top`,
and HAVOC'd the environment. Measured through the built engine: the site's argv was `[]` and the
node was `Top`, so `unresolvable_loads` did not even record it. +SURE.

That is why the brief's "reuse the existing machinery, never a second resolver" was satisfiable
literally rather than approximately — the machinery was already correct and merely starved. After
the parser cut the same book reads `argv=[".", "./oracles/lib.sh"]` and binds `Defined(d)`, with
no diff in `analysis/` outside its test module.

### dec-the-trigger-asks-for-value-existence-not-literalness

The cut, and it is a deliberate shrink of a FIXED `syntactic-top-triggers` entry
(`syntax/CLAUDE.md`), which `inv-top-reject` says is never an accident. The warrant is `28K` §1
itself — human-ACKED at `28K` §9 rat-four-rules-wording, and its text names this exact spelling as
one that must load — so the design act was already taken; this is its implementation.

The trigger now asks whether ANY value-flow could ever hold the target, not whether the word is a
literal: a command substitution or arithmetic expansion answers no under every possible flow and
stays a parse-tier ⊤; a parameter expansion is ordinary value-flow and belongs to the analyzer.
The predicate is not new — it is `word_has_expansion_effect`, reused verbatim from the FOR-LIST
word trigger, which is the same question (a word whose VALUE the analyzer must know to model
control flow) in the same structural position. So the two-tier answer is a convention the parser
already keeps, not the confusingly-similar middle `271:rul-net-quality-u-curve` warns against; and
`semantic-top-not-here` ("the dynamic-word/expansion surface is the analyzer's ⊤, not the
parser's") already said expansions in argument position are not the parser's business. The
source-target trigger was the exception to its own crate's rule, minted before the
function-environment domain existed to take the question.

### fnd-parity-is-the-deliverable-and-the-e2e-tier-can-only-show-it-negatively

Measured, and it changed the shape of the commissioned pin. `. "$PKG.oracle.sh"` and
`. foobar.oracle.sh` in the same book render the SAME PLAN — `sites=0`, the same three sites
`unresolvable-no-probe` — identical apart from the book's own bytes and the digest over them.
Parity IS the ruling ("exactly as a literal path does"), so a case showing the variable spelling
doing something SPECIAL would be pinning a bug.

But both spellings ship nothing, and the reason is orthogonal to this item: a top-level `.` is an
unmodeled command in book position, so it walls (`opaque-poison-is-the-product`) and nothing below
it resolves — for the literal spelling equally, today and before bitem9. That is
`28O:res-book-sourcing-walls-at-the-site`, whose blessing question
(`.`-of-a-proven-load-inert-file) is ALREADY routed to the human beside `command -v`
(`adj-command-v-blessing-routed-to-human`). Standing consequence worth the conductor's attention:
until that question is answered, a book that sources an oracle walls its own remainder by every
spelling, so bitem9's admin-visible payoff is gated behind a decision this lane did not own.
bitem9 widened what RESOLVES; the wall is a different question.

`pin28-variable-resolved-source-loads` is therefore non-vacuous through GATE-3 rather than through
its transcript: before the cut this book carried two undeclared error-severity diagnostics
(`syntax-unsupported` + `cfg-top-node`) and the case would have failed outright. Clean stderr is
the whole difference, and the transcript carries the parity claim in prose.

### fnd-an-executing-corpus-case-cannot-carry-a-top-level-source

Found by the harness, not by reasoning, and it explains an absence nobody had named: NO executing
corpus case has ever had a book-level `.`. `.` is a SPECIAL builtin, so a non-interactive shell
that cannot find what it sources EXITS — and the exec rail's cwd is an empty throwaway sandbox, so
the rendered apply exits rc 2 and `ap-2-exec` fails the case. (gate-5 fails too: the BARE book dies
at the same line, so the site below never runs.) The floor lane already copies the case dir into
its sandbox for exactly this reason, scoped to gate-9. This is `28K` §8
`res-book-ships-its-load-closure` — named there, unbuilt — wearing harness clothes; the new pin
carries no `mocks/` and says so.

### res-the-load-set-lookup-is-exact-string (disclosed)

`definitions_of_path` is a `BTreeMap<String, _>` keyed on the path AS THE CONTROLLER SPELLED IT, so
`. "$LIB/x.sh"` with `LIB=./oracles` resolves only when the driver was handed `./oracles/x.sh`
spelled identically; `oracles/x.sh` misses and walls. Conservative (a miss is an unresolvable load
⇒ ⊤ ⇒ withhold), never a wrong license. NOT normalized here on purpose: lexical `./`-collapsing is
guessing at a filesystem question from a pure kernel, and `28K` §8 already sites
relative-path-vs-cwd robustness as a build concern. ~SUSPECT it bites the first real admin who
tries this; the honest repair is at the input surface, not in the domain.

### res-unresolvable-loads-is-computed-and-never-consumed (pre-existing, now load-bearing)

`FuncEnv::unresolvable_loads` documents itself as "reported by the caller" and NO caller reports
it — the accessor has one reference in the workspace, the field's own getter. Pre-existing and
reachable before this item (`. never-given.sh` with a literal target hits it), but bitem9 widens
the population that lands there, so it is worth naming. NOT a license bug: an unresolvable load
havocs the environment, `unprovable` names every family, and the driver withholds — ⊤ licenses
nothing, exactly as `rul-unloadable-is-unlicensed` requires. What is missing is the NARRATION
(`two-plane-aid-law`: the aid plane should be telling the admin which families went quiet and
why), which is a code + defining case + ceiling bump, i.e. its own small item. Named, not built.

### The aid surface (no new code, no ceiling bump)

The reason variant and its slug renamed in place, no alias (`rul-strawman-formats-no-compat`):
`SourceOfNonLiteralTarget` → `SourceOfDynamicTarget`, `syntax-unsupported-source-of-non-literal-
target` → `syntax-unsupported-source-of-dynamic-target`, with the defining case renamed and its
book moved to `. "$(hork profile)"` — a target that still fires the narrowed trigger, so the case
proves the trigger rather than the old one's memory.

The WORDS were deliberately NOT rewritten. `Words::Unwritten` was tried first and is the right
builder posture (`error-authorship-tier`: builders author ZERO prose), but
`every_migrated_reason_renders_words_not_a_placeholder` refuses a placeholder for this family — it
is a WIRING gate whose failure signal is `[unwritten:`, so an intentionally-unwritten row trips it
for the wrong reason. Landed instead: the pre-bitem9 migrated text stands verbatim (authoring
nothing), and the row's `when_used` METADATA now records that the words are still true of every
firing but wider than the trigger, and are owed a sharpening. Flagged below.

### Golden churn, predicted vs actual

Predicted before building: the parser cut moves any corpus case whose book carries a `.` with a
non-literal target — a survey found exactly one, the renamed defining case — and nothing else,
since no corpus book sources through a variable.

Actual: exactly that.

| cause | cases | delta |
|---|---|---|
| the defining case's rename + new book | 1 | slug, filename, book, transcript; the diagnostic still fires |
| every other loom transcript | 0 | 231 unchanged, byte-identical |
| `expected.out` / `expected.ran` | 0 | no e2e golden moved a byte |
| net-new | 1 e2e loom + 6 unit cells | additive |

Windows 1908 passed / WSL 1904 passed, both legs, full gate. Comment budget: 10 net-new non-doc
`//` lines against a cap of 12.

### The behaviour pins

Unit tier is where this item actually lives (`analysis::funcenv` TABLE 5), and the load-bearing
one is `a_variable_resolved_source_target_binds_what_the_literal_spelling_binds`, which loops the
SAME assertions over BOTH spellings — so it fails the day a second resolver appears beside the
first and lets them disagree. Beside it: the two ⊤ cells (an unresolved variable, and a resolved
path the controller never read — resolving a PATH is not learning what lives at it), the shadow
refusal reading a variable-resolved load exactly as it reads a literal one (widening the
resolvable set must not widen the SILENT set), and the fence cell below.

`a_variable_spelled_file_test_decides_what_the_literal_one_decides` is the fence, and it is
MEASURED rather than argued: run against the OLD trigger it still passes, while the two
source-target cells fail. So the file test's operand already resolved through the plane and
`28M:dec-decidable-set-v0` was untouched by this item — the decidable-CONDITION set did not move,
only the RESOLVABLE-target set. That distinction is the brief's own fence and it is now checkable
by anyone who doubts it.

## Flagged upward (bitem9)

- **`tc-source-target-trigger-shrank-a-fixed-syntactic-top`** — the item could not be done inside
  `analysis/` at all; it required shrinking a FIXED entry of `syntax/CLAUDE.md`'s
  `syntactic-top-triggers`, which `inv-top-reject` reserves as a deliberate design act. My read is
  that `28K` §1 (human-ACKED) already took that act and this is merely its implementation, and the
  shrink is the narrowest one available (the for-list word's own predicate, reused). But it is a
  parser-tier licensure-adjacent widening in the crate its own file calls "the engine's
  highest-risk surface", it landed from a single-crate brief that did not anticipate it, and the
  conductor should decide whether `syntax/CLAUDE.md`'s trigger list wants updating in the same
  breath (this lane did NOT edit it — the entry now reads wider than the code).
- **`tc-migrated-words-now-overshoot-their-trigger`** — `syntax-unsupported-source-of-dynamic-
  target` renders "`.`/`source` of a non-literal target", which is true of every firing but
  describes a trigger three times wider than the live one. A builder may not author the
  replacement and the placeholder route is gate-refused (above), so the sharpening is a
  conductor/human prose act with the precise description already sitting in the row's `when_used`.
- **`res-book-sourcing-wall-gates-this-item's-payoff`** — not new, and not mine to rule: the
  `.`-of-a-proven-load-inert-file blessing question (routed at
  `adj-command-v-blessing-routed-to-human`) is what stands between bitem9's delivered capability
  and an admin ever seeing value from it. Worth the human knowing that a second item now waits on
  that same answer.

## Flagged upward (builder-4)

- **`tc-inert-mocks-rail-is-dash-shaped`** — `fnd-printf-is-not-a-builtin-in-posh`. Every shipped
  emitting body (`disturbs`, `reaches`, `resolve`) is a `printf` emitter, and under the corpus's
  `PATH=mocks-only` rail posh cannot run one. So the two-binary floor has never been exercised over
  the corpus's own oracle bodies — only over the six manifests this item added, which are given the
  floor's userland deliberately. Cross-cutting (it touches the determinism rail every executing gate
  shares), so flagged rather than taken. The cheap first move is probably a `printf` shim in the
  standard mocks set, which would make the whole corpus posh-runnable; whether that is a FAITHFUL
  measurement is the judgment call, since a shimmed `printf` is not the floor's `printf`.
- **`tc-wrapped-lane-drops-a-case-bodied-in-book-verdict`** —
  `fnd-the-wrapped-lane-cannot-lift-a-case-bodied-in-book-verdict`. The canonical authored verdict
  shape, sited in a book, answers nothing at a wrapped site while the same bytes in an oracle file
  ship. Measured, not diagnosed. It costs coverage, never a license (the site runs), which is why it
  is flagged rather than fixed inside an item that owns neither seat.
- **`res-strip-leaves-a-bare-colon-for-a-standalone-mark`** (observation, LOW confidence) — the
  shipped verdict body for a standalone `safe-across` mark carries a bare `:` where the mark was,
  visible in `context-entry-babby-elides` and both new wrapped pins. `strip-is-pure-erasure` says a
  bare-mark statement is an annotation LINE that erases to NOTHING, never a null command; the
  competing reading is that the author wrote a POSIX `:` carrying a TRAILING mark, under which the
  command correctly survives. Genuinely ambiguous by construction, inert where it appears (never the
  last status-affecting statement in any corpus body), pre-existing, and NOT this lane's — recorded
  only because three cases now display it.
