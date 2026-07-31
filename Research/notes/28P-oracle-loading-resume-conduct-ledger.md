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
  pin-by-definition-bytes (NOT STARTED — see its section for why, and for the handoff state).

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

## bitem1 — pin-by-definition-bytes (NOT STARTED)

Deliberately not begun rather than half-begun. It is the only item in this brief that MOVES
ARTIFACT BYTES (hoisted definitions, hash-munged names, provenance blocks), so it re-blesses a
large slice of the corpus, and the brief's stop-condition — artifact bytes moving or a license
widening — is exactly the surface it operates on. Starting it without the budget to inspect that
churn case by case would have produced the one outcome the instruction forbids. Handoff state: the
seats it builds on are all landed and the corpus is byte-stable at this tip, so it starts from a
clean baseline.
