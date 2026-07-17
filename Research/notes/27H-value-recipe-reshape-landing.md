# 27H — block-rebuild stage-4 (value-recipe-reshape) landing + residue

AI-authored (Opus builder, r27 stage-4 session). Records what landed for
`270:block-rebuild` stage 4 (the fragment-preserving, cause-tagged ValueOf/Recipe reshape +
the value-prediction species' derived provenance) against `notes/275` + `plans/271`. Companion
to `27D` (the conductor ledger), `27E`/`27F`/`27G` (stages 2/2b/3). Authority: root docs +
`spike/CLAUDE.md` rulings + `271`/`275`/`277` outrank this. Branch: `ai/r27-value-recipe` off
`ai/spike3-r27` (base `0387c08`).

## What landed (all green: full unit suite + 128/128 e2e; four gates clean; ZERO golden churn)

The reshape is REPRESENTATION + DERIVATION only. The consumers (folds, transport, capture) arrive
at block-context; nothing here mints a license (`value-predictions`: provenance/backing are
DERIVED, never declared — the authored surface stays THE EMPTY SET).

- **`core::TopCause`** — every ⊤ names its CAUSE (`219` q-2): `UnmodeledExpansion` /
  `UnresolvablePositional` / `DynamicParameter` / `DynamicValue` / `SplitOrGlob` / `NonConvergent` /
  `WalledRead` (reserved — no producer at this stage; the capture lane's walled read). `Copy`,
  payload-free (a category, not a span); `TopCause::describe()` supplies the why-lane phrase.
- **`core::ValueGrade`** — the value-prediction provenance grade (`275` §2): the four species
  grades `⊤ < AuthorComposed < WorldSpoken < Register` plus `ProgramText` (the top — NOT a
  prediction; the `seam-literal-provenance` distinction). `ValueGrade::weakest` is the
  weakest-fragment meet (min); ordered so `ProgramText` is the identity and `⊤` the absorbing
  bottom, matching "delegation-read + composed-decoration grades composed" (`275` §2).
- **`ValueOf::Top(TopCause)`** (`analysis::value`) — cause-tagged. Every consumer pattern-matches
  `Top(_)` (`inv-top-reject`: all ⊤ run, the cause is why-lane content, never a branch on which ⊤).
- **`Recipe` is fragment-preserving** — a ⊤ part becomes `Frag::Top(cause)`, RETAINING its
  neighbours (the pre-reshape `Recipe::Top` collapse that erased fragments + cause is gone). So the
  weakest-fragment provenance is computable over a MIXED word (`275` §2). `Recipe::Opaque(cause)` is
  the not-a-word-node defensive case only.
- **The derivation** — `provenance_of_recipe` (weakest-fragment over frags) + `word_grade`
  (env-aware: an unset `Frag::Var` resolves ⊤, so grade follows the VALUE's ⊤-ness, then the
  recipe supplies the sub-⊤ grade). Exposed as `ValueFlow::argv_word_grades(node)` — one grade per
  SOURCE word (not per split-field). This is the `read-value-slice` seam block-context consumes
  directly (report ask #2): it tells a probe-captured value (a prediction) from a source literal
  (program text), which bare `ValueOf` cannot.
- **`OutClaim` → `OutBytes`** (`275` care-outclaim-rename) — the newtype is channel CONTENT, not a
  claim.
- **`Relation::Same` → `Relation::Overlaps`** (`27D` disposition-relation-same-misnomer) — the
  overlap-honest name; survival-collide, NOT cell-identity. Consumer-map doc: transport-grade
  sameness is `selector_identifies`-gated, NEVER the overlap variant. The synthetic
  cross-generator pin (which encoded the exact misnomer, `transport_licensed = Overlaps`) is
  corrected.
- **`219` q-2 cause-naming** — the cmdsub-⊤ disclosure now names the specific cause (subst vs
  unresolvable-positional vs dynamic-var …) instead of the generic "$(…) or runtime-dynamic value".
  No new diagnostics — enriches the existing per-⊤-site disclosure; correlated-cascade suppression
  intact. Exempt-plane (display only, distinct from the attribution `ProvId` cause).
- **brace-verdict-loud-reject** (`27D` disposition-brace-verdict-silent) — a `#{a,b}` on a
  verdict/observe mark now emits a LOUD Warning (`mark-brace-verdict-single-cell`), surfaced via
  `oracle::lift`. `derive_predict` gained a diagnostics channel; it walks only verdict/observe
  bodies, so the rejection is role-aware (the parser accepts the brace shape role-agnostically).

## The foreclosure walk (`279f` §5 rider — `219`'s six-step capture chain × carrying element)

For each step of the capture chain, WHICH reshape element carries it, and confirmation nothing
frozen forecloses it. +SURE on all six: the reshape opens the representation without committing a
route.

- **q-3.a — license the inner command into the probe (the vouched-Query gate).** Carried by the
  EXISTING Observe (`:?`) / `CommandEffect::Queries` machinery, untouched. The reshape adds
  `TopCause::WalledRead` (reserved) for a capture whose producing read is walled — the ⊤ side of
  the gate. NOT foreclosed: the gate is orthogonal to the value representation.
- **q-3.b — the record-grammar wire (multi-line stdout).** Carried by `OutBytes` (renamed, still an
  interned `Symbol`) + the reserved `stdout=` record key. NOT foreclosed: the rename is
  content-tier only; the single-token wire floor (`275` §10) and the base64/refuse-non-text fork
  (`219` q-3.b) stay open.
- **q-3.c — `inv-probe-sourced-values` fit + provenance.** Carried by `ValueGrade`: a captured
  value's `WorldSpoken` grade IS the probe-provenance the fold will require. The weakest-meet fold
  already consumes it; only a producer (a capture fragment carrying `WorldSpoken`) is owed. NOT
  foreclosed — the grade lattice has the slot.
- **q-3.d — kFAIL-withhold fit.** Carried by `ValueOf::Top(WalledRead)`: an unvouched inner command
  keeps the site ⊤ ⇒ runs. NOT foreclosed: the reshape only makes ⊤ NAME its cause, never removes a
  ⊤.
- **q-3.e — the value-plane ← probe-record back-edge (post-probe re-bind).** NAMED as
  `seam-pipeline-order` (below); the fragment-preserving recipe is exactly what a second value-flow
  pass (or a fold-time substitution channel) re-reads. NOT foreclosed: the recipe representation
  neither assumes nor forbids a re-bind pass — a captured fragment folds back as a new `Frag`
  carrying its grade + backing.
- **q-3.f — the apply render of a folded `v=$(cmd)`.** Carried by the value plane's existing
  assignment/argv resolution (unchanged) + the fragment recipe (which records the capture fragment
  for the render to substitute). NOT foreclosed: no render decision is baked; the binding-site
  elision hazard (`275` §5) stays a block-context care.

## The two reserved seams (NAMED + representation-open; build NOTHING — `271:rider-value-recipe-reshape-capture-seams`)

- **`seam-pipeline-order`** (the post-probe value re-bind). NAMED in the `Recipe` / `TopCause`
  doc-comments and `provenance_of_recipe`'s doc: the value plane runs strictly BEFORE the probe, so
  folding a captured literal back requires a second value-flow pass OR a fold-time substitution
  channel. The fragment-preserving recipe does NOT foreclose either route (it holds the fragments a
  re-bind would re-resolve). **What would build it:** a post-probe pass that binds a capture site's
  variable to the probe-record's `OutBytes` (a new `Frag` variant carrying the captured value + its
  `WorldSpoken` grade + backing coords), then re-runs value propagation — OR a fold-time channel
  that substitutes at the leaf without re-flowing.
- **`seam-literal-provenance`** (source-literal vs probe-captured bytes). DELIVERED as
  representation: `ValueGrade::ProgramText` (a source literal — NOT a prediction) is distinct from
  the four prediction grades. `frag_grade` currently maps every non-⊤ fragment to `ProgramText`;
  the probe-captured fragment (graded `WorldSpoken`) is the open slot (`219` q-4.c leans the
  site-keyed-record route; the grade slot exists either way). **What would build it:** the capture
  re-bind (above) attaching a non-`ProgramText` grade to the captured fragment.
- **`seam-per-channel-backing`** — the value-plane counterpart (a value-prediction's backing SET,
  per-channel through recipe dataflow — `275` §2). REPRESENTATION-EMPTY at this stage: no current
  fragment carries a backing coord (captures are ⊤ ⇒ `Frag::Top`; the value plane runs before the
  probe). The recipe is the structure a backing derivation folds over once a capture fragment lands.

## Riders — statuses

- **rider-relation-overlap-rename** — DONE (mechanical rename + consumer-map doc + corrected
  synthetic pin; all stage-3 pins green).
- **rider-brace-verdict-loud-reject** — DONE (loud Warning + role-aware + test; DiagCode registered
  on the legacy allow-list).
- **`219` q-2 cause-named ⊤** — DONE (label enrichment; kWARN-rich, cascade-suppression intact).
- **rider-positional-modeling-hardening** — SPLIT. The BOOK-side (value plane) is DELIVERED by the
  reshape: bare `$@`/`$*` and quoted `"$@"`/`"$*"` route to `ValueOf::Top(UnresolvablePositional)`
  (`inv-top-reject`) via `top_cause_of_part`. The ORACLE-side (the predict parser's wrong-concrete
  `Word::Literal("$@")`, `parser.rs:1170`) is FLAGGED, NOT changed — see the finding below.

## Findings (flagged UP; NOT resolved locally)

### finding-positional-oracle-side-couples-founding-pin (the `24C:fd-headline-oneliner-gap`)
Routing the oracle predict parser's `$@`/`$*`/`"$@"` from `Word::Literal("$@")` to the existing
`Word::Unmodeled` (⊤ in every position — the round-20 `${x:-y}` precedent) BREAKS
`typeless-floor-oneliner` (verified: probe record goes empty ⇒ gate-1 + ap-2-exec fail). CAUSE
(`verdict::run_command`, `verdict.rs:367`): a reached authored CHECK command requires EVERY word to
resolve concretely or the whole verdict traces ⊤ ⇒ Declined ⇒ no probe. The founding
`mycmd__is_converged() { mycmd --dry-run "$@" ;}` vouches only because `"$@"` currently resolves
to the wrong-concrete literal `"$@"` (Ok) — the disposition-positional-literal-model's "wrong model
caught by rc-authority" (`27D`). The CORRECT model (`"$@"` → concrete positionals) needs a
position-aware `Word::PositionalArgs`:
- in COMMAND position (`run_command` in `verdict.rs` AND the predict `Evaluator` in `eval.rs`) the
  arg-vector is concrete-by-construction (the traced positionals) ⇒ it must NOT ⊤ the check;
- in VALUE position (annotation RHS, `[ ]` operand, `case` scrutinee) it is genuinely ⊤ (multi-value
  can't be one value) ⇒ Err.
TWO obstacles make this block-context-scale, not an in-stage rider: (1) the change spans BOTH
evaluators + every exhaustive `Word` match + the parser, with the founding pin as a hard tripwire;
(2) the rider's quoted-`"$@"`-vs-bare-`$@` DISTINCTION is not representable — the predict parser
collapses quoting (`"$1"` and `$1` both → `Word::Positional(1)`), so `"$@"` and `$@` arrive
indistinguishable at `parse_word`. A uniform `PositionalArgs` (family → vouch-concrete in command,
⊤ in value) is SOUND (the shipped probe runs authored bytes with real runtime expansion; the
probe's real rc is authoritative) but drops the quoted/unquoted precision the rider names. HEAD is
green with the acknowledged-wrong-but-sound-via-rc-authority model; recommend this rides
block-context's wrapper/passthrough surface (`273`) where `"$@"` re-expansion is native.

### finding-observe-backing-widening-production-is-effect-plane
The value-plane reshape delivers the SPECIES representation (provenance grade; the recipe the
backing derivation folds over). The EFFECT-plane backing SET — a `:?` observe inside a verdict body
WIDENING the enclosing FACT's backing (`277` §5 `seam-backing-sets` / `271` observe-backing-widening,
"now producing") — is a SEPARATE deliverable that touches `plan::survival` (the design's ONE
naked-trust cell) and requires a NEW per-body derivation: `derive_predict` currently emits
independent rows (a `:` verdict → Establish, a `:?` observe → Observe); linking the observe coord to
the verdict's fact-backing within one body is unbuilt. The corpus HAS `:?` observe marks (door1
cases, exec-shimmed-query-fold), so wiring production is NOT dormant — it must be proven byte-
identical (safe direction: widening only ADDS coords ⇒ toward-collide; no survival fires without
`--risk-faultless-skips` + selector-bearing footprints, so byte-identity is expected but must be
e2e-verified). The universal-meet law + `pin-set-meet-order-independence` /
`pin-no-outcome-as-generator` are ALREADY landed (`27G`, synthetic). The MINTING-family threading
(`27D` disposition-backing-family-recovery: carry the fact's true minting family instead of the
`sole_family` reverse-lookup) needs a `ProviderId` slot on the fact/backing path — stage-3 deferred
it (`tc-backing-family-via-dialect-reverse-lookup`), and it rides the same effect-plane change.
RECOMMEND: a dedicated backing-SET stage (or block-context) owns `Backing`→SET + observe-widening
production + family-threading + the survival universal-meet consumer migration, with e2e byte-
identity as the gate. NOT rushed into the naked-trust cell late in stage-4's budget.

## tc-* carried forward (from `27G`, still open — NONE resolved here)
tc-context-slot-on-coord-not-factkey · tc-resolutions-stays-in-plan ·
tc-backing-family-via-dialect-reverse-lookup (subsumed by the backing-SET stage above) ·
tc-same-is-overlap-not-identity (RESOLVED by rider-relation-overlap-rename — the variant is renamed
and the transport gate documented) · tc-brace-verdict-silent-skip (RESOLVED by
rider-brace-verdict-loud-reject).

## Commits (on `ai/r27-value-recipe`)
1. rename `Relation::Same` → `Overlaps` (+ consumer-map doc + corrected synthetic pin).
2. `OutClaim` → `OutBytes` (content-tier).
3. the value-recipe-reshape (cause-tagged fragment-preserving ValueOf/Recipe + provenance
   derivation + `ValueFlow::argv_word_grades` + tests).
4. `219` q-2 cause-naming in the cmdsub-⊤ disclosure.
5. brace-alternation-on-verdict loud reject (+ test).
6. register the new DiagCode on the legacy allow-list.
