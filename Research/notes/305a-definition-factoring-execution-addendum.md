# 305a — Definition-factoring execution addendum (rulings + the staged chunk)

> Tier: conductor-authored work order (Fable, 2026-08-16), SUBORDINATE to
> `notes/305` (which stands verbatim except where amended here) and `plans/28Q`.
> Purpose: `notes/305`'s lane ran its checkpoint + first half under one executor
> (branch `ai/r30-lane-definition-factoring`, 7 commits, tip `c41b95f8`, tree
> clean, green: check-quiet · fresh build · whole test:e2e zero-drift ·
> gate:quick-quiet 1251/1251); the second half is an atomic ~2.5h chunk the
> executor correctly declined to open at deep context. This doc makes the
> accumulated conductor rulings + the staged plan durable so a FRESH executor
> opens with zero re-derivation. The prior executor's checkpoint findings are
> recorded in `notes/307` §2 and land fully in `307c` at fold.

## §1 — Rulings banked (all conductor-adjudicated at checkpoints; binding)

- `305` §2 item 8 (the stage-0 retroactive audit) is STRUCK — stage-0 was never
  built (`307:fnd-stage-zero-is-not-built`; `28Q` §8 corrected); it is its own
  queued lane. This lane builds NOTHING toward it; the byte-identity gate stands
  as specified over as-built lane preference.
- The type contract is LANDED in `core::definition` — consume, don't redesign:
  `DefinitionId{file, span}` private-field, one mint `at()`, custody DERIVED not
  stored; `DefinitionProvenance{Unkeyed, Keyed, Ambiguous}`;
  `LiveDefinition{Live, Withheld, NoOpinion}`; `answering_file(live, count,
  provenance_of)` is THE resolution seat. Frame lookup:
  `LiveDefinitions::definition_before` (+ `provenance_of`,
  `DefinitionTable::{identity_of, provenance_of}`, `source_file_of_index`).
- The authored/munged join-miss maps to `Unkeyed` (the ruled permissive arm,
  typed; `28P:dec-the-gate-applies-only-to-names-the-unit-knows` preserved).
  The both-sides-munge hardening is BANKED, never built here.
- **Dialect: `opt-dialect-keeps-a-whole-unit-winner` RULED.** `build_dialect`
  keeps a whole-unit fold named `dialect_minting_source` — vocabulary-aggregation
  ONLY, never resolution vocabulary in its name — doc-cited to `28Q` §9
  `pin-two-position-sparing` + the ruling's ground (mint-from-every-row is a
  latent sparing-LIBERALIZING widening: more both-in-dialect-distinct pairs =
  more sparing = the naked-trust tier's dangerous direction). It preserves the
  minting SET, not a resolution. The seat-law edit (below) carries the
  parenthetical naming it so "the frame-lookup is the only resolution seat"
  reads true.
- **The plurality census: ALLOW-LIST-SHAPED.** Asserts every REACHABLE plural
  family (load-set-modeled: parse each case's replay `-o`/positionals; exclude
  contested-withdrawn families) sits in an explicitly enumerated plural-idiom
  list — empty at the census's landing commit, growing to exactly this lane's
  six new cells when they land; anything unlisted reddens naming itself.
  Fallback if the parsing balloons: prose-only in `307c`, disclosed.
  Background: `307:fnd-corpus-carries-twelve-plural-families` — twelve textual
  plural cases; seven never load the second file; FIVE (`contest28-*` ×4,
  `guard23-reingest-collision-verbatim`) are held byte-stable ONLY by the
  contested-withdrawal (tripwire note landed on `ContestedFamilies`' test,
  commit `c41b95f8`).
- The SIX new ordinary golden cells are CONFIRMED (the pre-authorized new-cell
  class; named individually in the report); `floor30-*` committed bytes are
  untouchable; `pin30`'s flip per `305` §2 item 6 unchanged.
- Seat 9's fix (`307:fnd-survival-footprint-lane-scans-forward` — first-file-wins
  + no withdrawals in the disturbs lane, a live wrong-elision route) gets PINNING
  coverage for the two-file `__disturbs` shape; unit/differential tier acceptable
  if a golden is disproportionate, disclosed.
- The funcenv certifier floor (`folded_edges = ∅`) must survive `never_live`'s
  deletion on its own merits — add a seat test if none pins it independently.
- The whyworld world's frame-unification is IN scope (`28Q` §8 stage-i); if it
  genuinely cannot land, STOP with evidence rather than silently deferring.
- Law edits owed at the law step: `oracle/CLAUDE.md
  live-source-is-the-only-resolution-seat` → "the frame-lookup is the only
  resolution seat (the one surviving non-resolution fold is
  `dialect_minting_source`, vocabulary-aggregation only)"; the
  `analysis/CLAUDE.md` + `plan/CLAUDE.md` "built at stage-0" bullets → ruled at
  stage-0, NOT YET BUILT, citing `307:fnd-stage-zero-is-not-built`; the
  winner-shifting rider (`305` §3) into doc-comments at every seat.

## §2 — The staged atomic chunk (steps 3–4; verbatim from the prior executor)

Re-key `KindIndex.effects`/`widenings` to `(file, provider, verb)` and delete
`sources`/`set_source`/`source_of` · drop `lift_from_sets`' `live_source` filter
(every file's rows survive) · add `dialect_minting_source` per §1 · re-key
`VerdictIndex.by_provider` to `(file, provider)` and drop its `sources` · convert
seats 3–4 in `effect.rs` via `answering_file`, deleting `live_predict_source`'s
third condition (`idx.source_of == live`) as structurally unnecessary and
reducing `VisibleRole` to its family-munge · fix the ~10 in-crate call sites +
ripples (`coverage`/`sweep`/`lint`/`plan` tests) · then `build` + `test:e2e` +
drift check + `gate:quick-quiet` before ONE commit. Addressing shape (endorsed):
rows addressed by file index, selected by the frame's `DefinitionId` via
`answering_file`; `Ambiguous` blocks within-file plurality so same-file
addressing gives same-definition by construction; no stored row provenance, no
seat signature changes.

## §3 — Then unchanged (order preserved)

Seat 9 (the survival/footprint re-lift: frame resolution + the withdrawal edge +
the pinning coverage) → the `never_live` retirement end-to-end (funcenv + cli +
tests) → the law edits (§1) → the allow-list plurality census → the differential
test (`crates/cli/tests/definition_frames.rs` per the checkpoint-approved
mechanism: sourced-files-as-inputs under exact path strings;
file-of-answering-definition vs the committed `expected.emitted` lines; the two
helper cells as WITHHOLD cells asserting the closure floor) + the six new cells →
`task-verify-definition-vector-walls` (per `notes/28R` §snapshot) → the whyworld
unification → `mise run both gate:full-quiet` + `bless:dry` FOREGROUND →
`Research/notes/307c-definition-factoring-lane-report.md`.

`307c` must carry: the seat table (before/after) · `fnd-stage-zero-is-not-built` ·
`fnd-reserved-name-error-does-not-refuse` (+SURE lint-only trace; the ~SUSPECT
can-answer-at-sites gap disclosed, unproven by design) ·
`fnd-survival-footprint-lane-scans-forward` · `fnd-ship-predict-stage-is-not-in-
world` (discharged: seat 6 routes through the shared seat) ·
`fnd-corpus-carries-twelve-plural-families` with the five-case withdrawal
dependency · the sanctioned goldens named individually · gate evidence per leg ·
the decomposed comment counts (≤10% plain `//` among added non-test lines;
doc-comments separate).

## §4 — Out of scope (unchanged from `305` §5, plus this arc's additions)

Snapshot-transplant emission · closure-custody · world-scopes · stage-0 (its own
lane) · `Reach` facade eviction · minispec content · the both-sides-munge
hardening · any dialect behavior change.
