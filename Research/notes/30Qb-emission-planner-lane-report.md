# 30Qb — the emission-planner lane

> Tier: **LLM-authored, builder (Opus-class)**, lane `ai/r30-lane-planner` from `ai/main@aabcc2d9`.
> Charter: `plans/30P` (all; `the-emission-planner`, `the-stream-forms`, `review-adjudication-inputs`)
> · `notes/30Ng` §7 (the front-lift ladder, human-typed) · `notes/30Nh` (the as-built this extends) ·
> `28Q:pin-emission-planner-universal`. Schedule: `30O:lane-emission-planner`.
> Grades: +SURE / ~SUSPECT / -GUESS / --WONDER.
>
> Written in two halves. **§map** is this builder's proposal and the red cells it committed; a
> fresh EXECUTE builder implements it after the conductor rules. **§execute** is that builder's,
> appended later — this note is append-only and §map is never rewritten.

# §map

## what-landed-before-the-map — three commits

1. `72895f60` — the `30Pc` repair (`bug-assignment-bearing-dot-is-inlineable`). `cli::artifact`'s
   `book_loads` ignored `NodeKind::Simple`'s `assigns`, so `MODE=prod . ./entry.oracle.sh` was
   ABSORBABLE and `ImportEdit::Inline` — which replaces the whole command NODE — substituted the
   bundle's bytes for it and dropped the assignment. The absorbed top level then read an environment
   the authored `.` never had. `absorbable` now demands `assigns.is_empty()` beside the redirect and
   whole-line conditions; `Repoint` moves only the operand and stays eligible, so multipart is
   unaffected. Native cell `a_load_carrying_a_leading_assignment_is_not_absorbable` (both ends: the
   single stream falls back, the multipart still re-points), FALSIFIED once by removing the gate and
   watching it redden. Whole-product cell `emit30-assigned-load-is-not-absorbable.loom` — `auto` on a
   terminal render falls back to the preserved tree with `artifact-form-fallback`, and its apply block
   still carries `MODE=prod . ./wombat.dorc.sh` verbatim.
2. `b29524a5` — pin TEXT refresh only, horizons untouched (`30P:scheduling-truth`).
   `p-x-placement-tuning-pair`'s trigger now says `hoist`/`sink` rather than the retired
   top-lift/in-paren-colocation; `p-x-intra-compound-plurality`'s trigger and its test's doc-comment
   said alpha-rename was a live candidate spelling, which `30P:rul-planner-apply-side-first` closed —
   both now say ONE placement value (a per-segment environment), never a mechanism, with
   `d-alpha-rename-equivalence` named as RESERVED.
3. `daae914d` / `20966a75` — the two red/evidence cells, §7 below.

## fnd-the-question-is-asked-in-four-places-not-three

+SURE, measured. The charter says three seats; there are four, and the fourth is why two of the
cells below are red.

| seat | file:line | what it decides | placement values it has |
|---|---|---|---|
| the preamble hoist | `plan/src/lib.rs:5181-5251` (`pin_definitions`) | which bodies + closure declarations the artifact places, and under what name | `hoist`, hardwired |
| the bundle inliner | `cli/src/artifact.rs:610-625` (`inline_imports`) / `:473-541` (`bundle_files`) | where a book-sited dorc-lang root's bytes go | `in-place`, hardwired (`30Ng` §7's T3) |
| the naming switch | `cli/src/main.rs:1989-1997`, mirrored `cli/src/world.rs:502-510` | whole-artifact `defensive_emission` | naming only |
| **the interaction of the first two** | nowhere | — | — |

The fourth row is the finding. `pin_definitions` takes `(emitting, invoking, defensive_emission,
src, ast)`: it is told the BOOK's bytes and nothing about what the artifact will ALSO carry. Since
`30Nh`'s bundling, the artifact carries a book-sited root's whole stripped text at the `.` — so the
ALREADY-IN-PLACE rule (`plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding`), which asks
`book_already_defines(src, ast, …)` about the book's own top-level bytes, no longer sees everything
the artifact places. Measured consequence in `emit30-single-stream-carries-the-closure-twice`: the
apply artifact holds TWO `WOMBAT_ROOT=` assignments, TWO `_wombat_dest()` definitions and TWO
`wombat__is_converged()` definitions. "The EMITTED preamble never carries two same-named funcdefs"
holds against the route it was written for and not against the one the bundling added.

## 1 — component-shape

### where-it-lives

**`plan::placement`**, a new module in `dorc-plan`. Grounds: `plan` already depends on `analysis`,
`syntax`, `oracle` and `core`, so every legality input is reachable; `cli` depends on `plan`, so
`cli::artifact` can call it; and `PinnedDefinitions` — the thing that must gain a placement — already
lives there. Putting it in `cli` would invert that edge.

### rul-the-planner-answers-questions-it-does-not-schedule

+SURE this is forced, and it is the shape the charter's "ONE component" has to take. The two
consumers cannot run at one moment:

- `cli::artifact::Selection` settles BEFORE the plan exists (authored-before-contact,
  `artifact-forms-derive-from-one-structure`), because `ImportEdit` is an INPUT to `Plan::decided`.
- `pin_definitions` runs INSIDE `Plan::decided`, over the settled dispositions — it cannot know which
  guards emit until they are decided.

So the planner is a **decision procedure**, not a pass: one component that answers *"may bytes
binding name-set N be placed at position P, and under what name?"* purely from
authored-before-contact inputs, called by each consumer at its own moment with its own candidate
set. `the-render-decides-nothing` is preserved because the ANSWERS derive from frozen inputs and
`Plan::decided` remains the one seat that records them; nothing is re-derived at render and nothing
feeds back into analysis (`30Ng:attn-render-refusal-feeds-the-spine` is NOT a prerequisite — a hoist
under a proven closed set changes no resolution, per `30P`).

### inputs, as existing APIs

| the predicate needs | today's API | status |
|---|---|---|
| which definition is live for `name` immediately before a CFG node | `funcenv::LiveDefinitions::definition_before` (`analysis/src/funcenv.rs:767-784`) → `Live/Withheld/NoOpinion` | EXISTS, position-aware; answers BINDING only |
| which file that definition came from | `funcenv::LiveDefinitions::source_before` (same file, just below) | EXISTS |
| whole-unit dynamism openers | `region::CensusOpeners::of(universe, unresolvable_loads, definition_vectors, string_execution)` (`plan/src/region.rs:255-299`) | EXISTS; whole-unit, not position-scoped |
| the book's top-level funcdef names | `plan::book_defines_at_top_level` (`lib.rs:1417`) | EXISTS; name-only, position-blind |
| variable READS | none | **must be built** |
| book OBSERVATION or MUTATION of name N above position P | none | **must be built** |

**`need-a-name-observation-census`** — the seat proposal, and the one genuinely new analysis this
lane needs. `30Pb:fnd-emission-legality-covers-all-shell-state` sharpened tier one from "no book
CALL above the `.`" to "no book OBSERVATION OR MUTATION". Propose
`analysis::nameuse::NameUseCensus`, built once beside `funcenv::analyze` in the cli edge's frozen
set and threaded to `plan::placement`:

```
NameUseCensus::of(&cfg, &ast, &value) -> NameUseCensus
NameUseCensus::first_use_of(name) -> Option<CfgNodeId>     // earliest observation OR mutation
NameUseCensus::uses_above(name, node) -> bool              // the predicate's actual question
```

What counts as a use, enumerated closed (silence never licenses — `silence-licenses-nothing`):
- a command word equal to `name` (a call);
- `command -v NAME` / `command -V NAME` / `type NAME` — argv-literal only, ⊤ on a dynamic word;
- `unset NAME` / `unset -f NAME` / `unset -v NAME`;
- `alias NAME=…` / `unalias NAME`;
- any `WordPart` parameter expansion naming `NAME` (the variable read — this is the half no API
  answers today; the value plane walks words already, so this is a second visitor over the same
  tree, not a new lattice);
- an assignment binding `NAME` (`NodeKind::Simple`'s `assigns`, and `export`/`readonly` with a
  literal operand).

Anything the census cannot decide is a use (⊤-biased), and a dynamic command word or a
`ParamComplex` whose name the lexer discarded (`28O:res-load-inert-conservatism`) is a use of EVERY
name. -GUESS M-sized; it is one AST walk with no fixpoint.

### output type

```rust
/// Where a definition the artifact places may stand, and under what name.
pub enum Placement { Hoist, InPlace(AstId), Sink(AstId) }

pub enum EmittedName { Authored, Munged { authored: String, emitted: String } }

pub struct PlacementDecision { pub placement: Placement, pub naming: EmittedName, pub why: PlacementReason }
```

`PinnedDefinition` gains a non-`Option` `placement: Placement`, and `PinnedDefinitions::hoisted()`
becomes `PinnedDefinitions::typeset(Placement::Hoist)`-shaped — a form asks for the material AT a
placement. See §9 for what that makes unrepresentable.

### how each seat becomes a consumer

- **`pin_definitions`** stops assuming `hoist`. For each body/declaration it asks
  `plan::placement::decide(...)`, and it is handed the artifact's OTHER placements (the settled
  `Selection`'s import edits) so ALREADY-IN-PLACE can see them. This is the fix for
  `fnd-the-question-is-asked-in-four-places-not-three`: **a definition the artifact already carries
  at its authored position is not hoisted a second time**, which is the existing ALREADY-IN-PLACE
  rule widened from "the book's own text" to "anything the artifact places in the book's own order".
- **`bundle_files` / `inline_imports`** stop hardwiring `in-place`: they ask the same component for
  the bundle's placement, and the answer is the tier (§2).
- **the `defensive_emission` switch** stays a whole-artifact naming input, unchanged in meaning, and
  becomes one clause of `EmittedName`'s derivation rather than a bool `pin_definitions` reads
  directly. Do NOT make it per-body in this lane: `rul-happy-path-is-a-closed-set` says every tier
  above the defensive floor is licensed by PROVEN enumeration, and narrowing the trigger is a
  separate licence act.

### names-argued

- **`placement`** — an OVERLOAD in this corpus, and taken deliberately.
  `30P:rul-emission-is-the-umbrella-name` RESERVES `placement` for "SEMANTIC arrangement, the
  sh-parity engine concerns (where a definition may go and under what name)". That is precisely this
  component, so it is the reserved word being spent as reserved, not squatted. `emission` stays the
  umbrella (the lane, the forms, the planner's own title); `layout` stays weft's textual-emission
  word, which is why `hoisted()`/`typeset()` is layout and `Placement` is not.
- **`lift` is not used anywhere in the new code.** `30P` is explicit: "the lift" is the static lift
  of oracle text into the engine. `30Ng` §7's tier names still say lift-as-is / lift-and-munge; the
  ladder rows below use `tier-hoist-as-is` / `tier-hoist-munged` per `30P`'s own re-say, and the
  code should too.
- **`NameUseCensus`** over `NameCensus` — "census" alone is already spent on
  `region::CensusOpeners` and the occupancy-1 census; the discriminator is that this one is about
  USES of a name, not populations of a region.
- `Hoist`/`InPlace`/`Sink` are the charter's own three words and are not overloads.
- `PlacementReason` is a reason ENUM, not sibling codes (`28L:rul-reason-enums-not-sibling-codes`).

## 2 — the-tier-predicate-table

Every condition is computable from the definition table plus the census openers plus the new
name-use census; none needs settlement re-modelling. Read top to bottom; the first row whose
conditions all hold wins.

| tier | conditions (all must hold) | computation seat | fallback when it fails |
|---|---|---|---|
| **`tier-hoist-as-is`** (T1) | (a) the `.` is ABSORBABLE — top-level `Simple`, alone on its line, no redirect, **no leading assignment** (`30Pc`, landed); (b) for every name the bundle BINDS, `NameUseCensus::uses_above(name, dot_node)` is false — no book observation OR mutation above the `.`; (c) the bundle's own top level READS no variable the book binds above the `.` (the converse direction: hoisting makes it read a different value); (d) `CensusOpeners::opens_every_population()` is false ABOVE the `.` — no unresolvable load, definition vector or string-execution site earlier in the unit | `artifact::book_loads` (a) · `NameUseCensus` (b,c) · `CensusOpeners` + node ordering (d) | T2 if (b) failed on collision grounds ALONE and the unit is dynamism-free; else T3 |
| **`tier-hoist-munged`** (T2) | (a) as T1; (d′) the unit carries NO dynamism opener ANYWHERE (knowledge total — stronger than (d), because every reference must be enumerable, not just the ones above the `.`); (b′) T1's (b) failed, and **every colliding name's every reference is engine-emitted** | as above, plus a reference census over the bundle's own bodies | **T3** — see `tc-t2-is-narrower-than-the-ladder-says` |
| **`tier-in-place-rewritten`** (T3, today's as-built) | the bundle is buildable and the import is re-sayable | `bundle_files` / `inline_imports`, unchanged | Decline |
| **Decline** | the existing refusal classes: dorc-lang load dynamism and kin; plus a single stream whose `.` is outside the absorbable shape (`FormFallback::InliningUnproven`) | `artifact::select` | the named refusal / `auto`'s fallback, unchanged |

**Conductor default for T2 "willing", as briefed**: munge whenever T1 fails only on collision
grounds. Adopted in the table above, with the narrowing that follows.

**`tc-t2-is-narrower-than-the-ladder-says`** — FLAGGED, not resolved; winner-shifting and
licence-adjacent. `30Ng` §7 (human-typed) says T2 "lift[s] under MUNGED names and rewrite[s] every
reference consistently in the GENERATED plan". Rewriting a reference INSIDE an authored body is
alpha-rename, which is RESERVED (`d-alpha-rename-equivalence`), and `28R:rul-munge-oracle-names-only`
scopes the header-only munge to definitions whose EVERY reference is engine-emitted. For a bundle
that is essentially the role functions alone: helpers are called from the oracle's own authored role
bodies, and file-level constants are read from them. So T2 splits:

- **T2a** — the colliding name is a role function (only guard/probe scaffolding invokes it):
  header-only munge, legal today, this is the `pin_definitions` munge already shipping.
- **T2b** — the colliding name is a helper or a file-level constant: alpha-rename would be needed,
  it is reserved, so **T2b falls to T3**.

Product-surface strawman for the human — the book collides with a package's private helper:

```sh
#!/bin/sh
_wombat_dest() { printf '%s\n' "$1" ;}   # the admin's own helper, same private name
. ./wombat.dorc.sh                        # the package binds _wombat_dest too
wombat sync a.conf
```

T1 refuses (the book binds a bundle-bound name above the `.`). T2 cannot rescue it, because the
package's role body calls `_wombat_dest` by that authored name. So it is T3: positional, correct,
and the front-lift attention win is lost for this book. My lean: **accept the narrowing and say so
in the tier's disclosure**, because the alternative is unreserving alpha-rename, which is a much
larger act than this lane. The disclosure is the gradual-enhancement hook — the reason arm should
name the colliding name so the admin can rename their own helper and get the lift back.

**Tier-one's sharpening is load-bearing and cheap** (`30Pb:fnd-emission-legality-covers-all-shell-state`):
`command -v`, `type`, `unset`, `alias` and variable reads all count. The decidable-set machinery
already sees `command -v <unit-defined-fn>`; the variable-read half is the new census.

## 3 — the-oracle-constant-hole

**CONFIRMED, and it is live end-to-end, not latent.** `30P:review-adjudication-inputs` left this
OPEN for this lane; the answer is yes, with two directions.

`pin_definitions`'s snapshot track (`lib.rs:5191-5211`) inserts every `ClosureDecl` into a
`BTreeMap` keyed by `(file, offset)` and pushes each into `definitions` with `disambiguated: None`.
`ClosureDecl` (`oracle/src/closure.rs`) carries only that key and its bytes — **it does not carry a
NAME at all**, so nothing here could check a collision even if it wanted to. The only
collision machinery in the function, `book_defines_at_top_level` / `book_already_defines`, runs on
the SEPARATE role-body track. A `ClosureDecl` is documented as "a helper funcdef **or a file-level
constant**", so `WOMBAT_ROOT=/etc/wombat` hoists verbatim above the whole book, unchecked.

Measured, by removing the XFAIL lens from `emit30-hoisted-closure-outruns-its-load` and reading the
failure:

```
FAIL [ap-2-exec: apply ran the wrong commands or wrong order]
      want: ran: hork stage unset          got: ran: hork stage /etc/wombat
FAIL [gate-6: apply/bare run-set delta not covered by the license ledger (cm-1 dual-rail)]
      apply-only (ran in apply, not in bare): hork stage /etc/wombat
      unattributable bare-only (elided with no replace/omit license): hork stage unset
```

The engine's own counterfactual rail calls it an unattributable delta. That is the correctness
statement: the artifact binds a name at a line the authored program does not, and a book line above
the `.` reads it.

### the two directions, and the one that is currently masked

- **book OBSERVES above the `.`** — the cell above. Red, XFAIL.
- **book BINDS above the `.`** — the artifact's hoisted copy runs first, the book's own assignment
  then wins, and the ORACLE's check reads the ADMIN's value. In multipart this bites. In the SINGLE
  STREAM it is currently masked, because the absorbed bundle re-binds everything at the authored `.`
  position (last-wins) — which is the double-carry `emit30-single-stream-carries-the-closure-twice`
  pins. Masked-by-duplication is not a defence; it disappears the moment the preamble munges a body
  the absorbed copy still spells with the authored name.

### the-proposal — `rul-a-loaded-definitions-placement-is-its-load-position`

A definition's default placement is decided by HOW its source entered the unit:

1. **A `--pre-source` root is AMBIENT** (`only-invocation-roots-are-ambient`,
   `30Mc:required-root-occurrence-identity`): the analysis already models its bindings as live before
   the book's first line, so hoisting is FAITHFUL and needs no predicate. **No repair is owed here,
   and this is most of the corpus.** +SURE.
2. **A source reached by a book `.` at position P** binds at P in the model. Its default placement is
   therefore **`InPlace(P)`**, not `Hoist` — and in the single-stream and multipart forms the
   artifact ALREADY carries its bytes there, so the correct emission is to *stop hoisting a second
   copy*, not to add machinery. This simultaneously fixes the double-carry and both hole directions.
3. **`Hoist` for a book-`.`-loaded source is then an OPTIMISATION**, gated by §2's T1/T2 predicate —
   which is exactly what the front-lift ladder is. The ladder and the hole repair are one mechanism.
4. **Fallback when the predicate fails and there is no authored position to fall back to** — a
   pre-source root cannot arise here (case 1), so the residual is a body whose source the artifact
   does NOT carry. Then **withhold**: no hoist, no guard, no elide, the site runs. This reuses
   `the-pinned-unit-includes-the-closure`'s existing shape ("a CONTESTED closure withholds the
   VOUCH") rather than minting a second withholding mechanism, and it lands on `kFAIL-perform`'s
   safe side.

Munging a VARIABLE is not on this list, deliberately: it needs alpha-rename inside the bodies that
read it, which is reserved. `option-strict-defensive-emission` [TYPED lean] is where a future
strict mode could close the residue by emitting per-site; keep the machinery general.

## 4 — emitted-name-injectivity

Two gaps, both real, both cheap.

- **The BARE-IF-SINGLETON census is over FUNCTIONS only.** `lib.rs:5219-5227` reads
  `book_defines_at_top_level`, whose walk matches `NodeKind::FuncDef` and nothing else
  (`lib.rs:1417-1424`). Variables are outside it entirely — which is the same root as §3. The census
  must range over emitted ∪ book top-level names for **functions AND variables**; the new
  `NameUseCensus` is the natural supplier of the book half.
- **Detect-and-lengthen on digest collision is NOT implemented** (+SURE by absence; grepped zero).
  `short_digest` (`lib.rs:1401-1406`) takes 8 hex of SHA-256 over the definition bytes.
  `emitted_names` is keyed `(name, body)`, so a same-NAME collision cannot arise; what is unguarded
  is the emitted string `<name>_h<digest>` colliding with **an authored name already in the unit**.
  That is not a hash accident — it is squattable in plain sight, and it is a different cell from the
  human's narrow `rul-munged-name-lifts-over-opaque-load` ruling (which is scoped to "Dorc-munged
  name, lifted across an OPAQUE load" and no further).

**Seat proposal**: one `EmittedNames` allocator inside `plan::placement`, holding the reserved set
(book top-level funcdefs + book top-level assignments + every already-emitted name) and minting
`<name>_h<digest>`, lengthening the digest one hex at a time until the name is fresh. Deterministic,
no runtime source, and it makes injectivity a property of the mint rather than of luck.

**`ask-injectivity-cell-needs-a-witness-roster-entry`** — for the conductor, NOT taken. The cell for
this is a book that top-level-defines the exact emitted `<role>_h<digest>` name; the preamble emits
that name, the book's own definition wins by last-wins, and the guard invokes the ADMIN's body —
pope-sin tier (`271:rul-sin-ordering`). I BUILT it, confirmed the shape, and then DELETED it,
because it necessarily puts a munged name into the corpus and
`sh_parity.rs`'s `the_happy_path_corpus_emits_no_munged_names` ratchet (`30A`'s zero-munge ratchet)
refuses any case not enumerated in `MUNGE_WITNESS_CASES`, whose own comment says "Adding to
`MUNGE_WITNESS_CASES` is a reviewed act, never a fix." Recipe for EXECUTE, once the conductor
approves the roster entry: book with `alias ll='ls -l'` (forces `defensive_emission`), a
pre-sourced `wombat.oracle.sh` with an ordinary verdict body, `site 2 effect=holds`; bless once to
read the emitted `wombat__is_converged_h<digest>` out of the transcript, then add a book top-level
`wombat__is_converged_h<digest>() { wombat squat "$@" ;}` and assert `expected.ran` shows the
ORACLE's `wombat cmp`, not `wombat squat`. Red today; greens with the `EmittedNames` allocator.

## 5 — disclosure

`ImportEdit` (`plan/src/lib.rs:2304-2350`) is `Repoint { ast, path } | Inline { ast, sh }` and
carries no reason. `PlanImportRewritten` (`aid/src/diag.rs:734-742`) carries `verb: &'static str`
(closed `repointed`/`inlined`) and `names: String`. Neither says WHICH TIER fired or WHY.

Per `28L:rul-reason-enums-not-sibling-codes` — one code, reason-components, never sibling codes:

```rust
pub enum PlacementReason {
    HoistedAsIs,                              // T1
    HoistedMunged { collided: String },       // T2a
    KeptInPlaceNameCollides { name: String }, // T2b — alpha-rename reserved
    KeptInPlaceDynamismOpener,                // T3
    KeptInPlaceShapeUnmeasured,               // the `.` is outside the absorbable cell
}
```

It rides a new field on BOTH `ImportEdit` variants (the edit and its reason mint together, like
`acts-and-dispositions-mint-together`), and a matching field on the `PlanImportRewritten` payload.
`dorc why` gets one arm per variant. **Every new code and register mints `message: None`** →
`[unwritten: <slug>]`; the prose is the conductor's/human's
(`error-authorship-tier` · `error-prose-conductor-flow`). The defining case is
`crates/aid/tests/plan-import-rewritten.loom`, which already exists and gains the new hole; publish
with `dorc-loom publish plan-import-rewritten`, then rebuild, then the scoped e2e bless — in that
order (`two-bless-paths-split-by-directory`).

**`WhyWorld` and the `30Nh` residue `dev-why-world-carries-no-import-edits`.**
`cli/src/world.rs:516-517` hard-codes `imports: &[]` with the comment "a why world settles no
artifact form, so it has no bundle for an import to name". That was true when a why world could not
settle a form; under this lane it becomes a real narrowing, because the placement decisions are part
of what a why report must explain. Proposal: `WhyWorld::analyze` takes the same `Selection` the
binary settled (it is authored-before-contact and needs no host), and passes
`selection.imports()`. One-definition-table-two-drivers' argument applies verbatim: a why report
explaining an artifact with different PLACEMENTS than the run's is a decoration. ~SUSPECT this is a
small change; the seam is that `WhyWorld` currently has no `StreamPosture` to settle a form under —
give it `StreamPosture::TerminalRender`, which is what a why render is.

## 6 — the-sink-value

`p-x-placement-tuning-pair` (`plan/src/lib.rs:6937-6967`,
`a_once_used_colliding_body_is_colocated_rather_than_lifted`). Control (must keep passing): two
sites sharing one colliding body still hoist+munge. Target (xfail): a ONCE-USED colliding body is
colocated inside its own guard's `( … )` subshell rather than hoisted and munged — asserted as
`guards == 1 && !hoisted.contains(name)`.

**What "once-used" is computed from**: the count of `Disposition::Guard` edits in the `emitting`
iterator whose `insert().fn_name()` and `insert().body()` are this exact `(name, body)` pair — i.e.
the length of the multiset `pin_definitions` already builds while filling `bodies` and
`emitted_names`. It is NOT a route count, NOT a census population, and NOT `ElisionRegion`
cardinality: those answer how many INVOCATIONS share an authored edit, and this asks how many
EMISSION SITES would carry the body. One extra counter in the existing loop; no new input.

`Sink(ast)` names the guard site whose subshell hosts the body. Legality is free here and worth
stating so EXECUTE does not over-engineer it: the guard's own `( … )` is a subshell, a definition
made inside it dies at the `)` (`death at the paren`, one of the three sh truths
`28Q:pin-emission-planner-universal` names), and it is the ONLY consumer by construction — so a
sunk body cannot collide with anything, and its naming is always `Authored`. That is the whole
attraction: `sink` trades one hoisted munged name for zero namespace footprint.

Guard-shape caution: `guard-shape` is `( <verdict-fn> <site argv> ) || <original bytes>` and
`guard_shape_violations` screams even under XFAIL. A sunk body goes INSIDE the parens ahead of the
call — `( <body>; <verdict-fn> <argv> ) || <original>`. EXECUTE must check that against the shape
floor before committing, because that gate is not xfail-blind.

## 7 — red-cells, committed

| cell | commit | tier | red? | what fails, and why that is the right reason |
|---|---|---|---|---|
| `emit30-hoisted-closure-outruns-its-load` (dir) | `daae914d` | the §3 hole, direction 1 | **XFAIL, verified** | `ap-2-exec` run-set (`hork stage unset` wanted, `hork stage /etc/wombat` got) AND gate-6's dual rail as an unattributable delta. Carries `head-expected.ran` so today's wrong behaviour is a two-sided pin: any drift before the fix screams instead of hiding. Multipart form (`ARTIFACT_SET`), so exec runs the published plan from its own generation. |
| `emit30-single-stream-carries-the-closure-twice` (dir) | `20966a75` | `fnd-the-question-is-asked-in-four-places-not-three` | **GREEN, deliberately** | Its committed transcript IS the evidence: four `WOMBAT_ROOT=` and two `_wombat_dest() {` in one apply artifact. The run-set is still right, by luck — the absorbed copy sits at the authored position and last-wins for the guard below it. The header says so, and says where the luck runs out. |
| `emit30-assigned-load-is-not-absorbable` (loom) | `72895f60` | Decline | GREEN | The decline cell of the ladder, minted by the `30Pc` repair: `auto` falls back, `artifact-form-fallback` fires, `MODE=prod . ./wombat.dorc.sh` survives verbatim. |

### dev-tier-cells-cannot-be-xfail-e2e-cases — the harness finding, and my deviation

`30Ng` §7 asks for "one round-trip case per tier plus the decline cell, single-stream form, each
asserting both the layout and `expected.ran` unmoved". **Those two clauses are in conflict under
this harness**, and I did not build T1/T2 as XFAIL e2e cases because of it:

- `e2e.rs:1870` skips the content golden when `xfail_active`, so an XFAIL case cannot assert LAYOUT.
- The ladder's own guarantee is that the run-set is UNMOVED, so the exec gate cannot fail either.
- A tier cell whose only observable is layout therefore has NO structural gate to fail, and would
  **XPASS on day one** — which the runner reports as an error. It is also exactly the rider
  "a case minted to demonstrate a capability must OBSERVE that capability" refusing the shape.

Three ways out, for the conductor to pick (my lean: **(c)**, then (a) as the fast pin):

- **(a)** Unit-tier `xfail_until` pins over `Selection::imports()` / `PinnedDefinitions`, asserting
  the `Placement` value directly. Cheapest and precise — but unwritable TODAY, because the assertion
  needs the `Placement` TYPE to exist. EXECUTE lands the type and the pins in one commit.
- **(b)** A harness capability: an `expected.layout` section, or an XFAIL variant that still
  compares a named golden. Real work, and it widens a governed runner.
- **(c)** Mint the §5 disclosure code FIRST, then each tier's e2e case declares
  `expect-diagnostic:`-style needles for its own reason arm — `needles-are-structural` makes a
  declared-but-unfired code RED, so the cell is red for a product-surface reason rather than a
  layout one, and it stays meaningful after promotion. This is the shape the ladder's own
  user-facing disclosure obligation wants anyway.

Note for whichever is chosen: a slug that does not exist in the catalog is REFUSED by the runner, so
(c) is strictly ordered — code first, cases second.

### the kept-stream expected-empty-stdout cell (the `30Nh` residue)

`30Nh` §1 and `cli/CLAUDE.md an-artifact-set-runs-from-its-own-generation` both name this as a
harness gap: the kept-stream cell's two behaviours are pre-network refusals with EMPTY stdout, and
the round-trip battery hard-fails empty output before any lens (the crash/empty guard). An ad-hoc
`#[test]` already does the job at `cli/tests/e2e.rs:3574-3591` (`output.stdout.is_empty()` inside
the bundle-incoherence refusal test).

**Recommendation: use `.rs` tests; do NOT build a declarative lane.** Grounds: the cell is a
pre-network refusal with no artifact, no run-set and no transcript — every axis the declarative
battery exists to compare is absent, so a lane for it would be a marker plus a negation and nothing
else, on a runner whose key vocabulary is closed and governed (`loom-form-is-the-same-battery`: a
new key joins the vocabulary in the same commit or the corpus goes red). The three behaviours worth
pinning are already native (`which_stream_carries_the_artifact_is_a_closed_table`,
`a_kept_stream_refuses_where_a_terminal_render_falls_back`,
`naming_the_preserved_tree_does_not_override_a_kept_stream`); what this lane owes is one MORE native
test beside them, asserting empty stdout for the `IncompleteSingleStream` refusal that a
placement-declined book now produces. If the conductor would rather close the gap properly, it is a
harness lane of its own and not this one's.

## 8 — main.rs-touch-plan

`cli/src/main.rs` is shared with `lane-load-plane-precision`. My touches stay inside the scout's
spans; the list, with what each needs:

| span | what this lane does there |
|---|---|
| `~1060-1063` (`funcenv::analyze`) | **READ ONLY.** The load lane owns this construction. This lane needs its result; it must not change the call. |
| `1106-1131` (`select_artifact_form`) | **EDIT.** Thread the definition table + `NameUseCensus` + census openers into the selector, so the tier predicate can run pre-network. |
| `1137-1163` (`HelperIndex::build`, `LiveDefinitions::new`, `shadows`, `unprovable`) | **EDIT, minimal.** Build `NameUseCensus` here, beside the other frozen inputs. Coordinate: the load lane also touches this region. |
| `1989-1997` (`defensive_emission` mint) | **EDIT.** Unchanged in meaning; becomes one input to `EmittedName` rather than a bool read directly downstream. |
| `2009-2016` (`project_plan`) | **EDIT, signature only.** Placement decisions travel beside `form_selection.imports()`. |
| `2261-2265` (`render_apply` / artifact assembly) | **EDIT.** `in-place` and `sink` need render entry points that `hoisted()` alone does not provide. |
| `2614-2639` (`select_artifact_form` helper) | **EDIT.** Self-contained; lowest blast radius. |

`cli/src/world.rs:498-524` gets the mirrored change (§5), and the mirror is not optional —
`one-definition-table-two-drivers`.

## 9 — types: what becomes unrepresentable, product-wide

Stated at the whole-product level, per the standing rider.

**Made unrepresentable** by `PinnedDefinition.placement: Placement` (non-`Option`) plus retiring
`hoisted()` in favour of a placement-indexed accessor:

- *No emitted definition can reach the artifact without a placement decision.* Today `hoisted()` is
  the single typesetting and "hoist" is an assumption no value records; afterwards a definition with
  no placement cannot be constructed, and a form that ignores a placement value fails to compile
  rather than silently flattening it — which is the `30Nd` §6.2 split (`invoked` is the DECISION,
  `definitions()` the material) carried one step further.
- *No placement decision can be minted at render time.* `PlacementDecision` is an input to
  `Plan::decided` on `ImportEdit`'s exact footing, so `the-render-decides-nothing` becomes a type
  property for placement, not a convention.
- *No emitted name can be minted outside the allocator* (§4), so injectivity over the emitted ∪ book
  namespace stops being a property of the call order.
- *A tier cannot fire without its reason* — `PlacementReason` is a field of the edit, minted in the
  same act (`acts-and-dispositions-mint-together`, `pin-no-outcome-as-generator`: the reason is the
  other half of the conclusion, never re-derived from the outcome).

**Still admitted, and worth the conductor knowing:**

- A placement whose legality was computed against a DIFFERENT unit than the one being emitted. The
  types carry no proof that the `NameUseCensus` and the `Ast` came from one analysis. `30K`'s
  frozen-set discipline is the only thing holding this, exactly as it holds for the definition table
  today.
- `Sink(ast)`/`InPlace(ast)` naming an `AstId` that is not the guard/load site they claim. An
  `AstId` is not newtyped per role, so a swap type-checks. Mitigating this properly means a
  `PlacementSite` newtype pair; -GUESS cheap, and I recommend EXECUTE take it, because a wrong site
  here relocates a definition and that is the pope-sin neighbourhood.
- Two placements for one definition. The allocator makes names unique but nothing makes the
  definition→placement map injective; a duplicate would re-emit. This is precisely the double-carry
  bug in type form, so a `BTreeMap` keyed by definition identity is the cheap fence.
- The pre-source/book-`.` distinction of §3 is DATA (the load account), not a type. Nothing stops a
  future seat from hoisting a book-loaded definition without asking. The fence is that
  `pin_definitions` no longer has a `hoist` default to fall back to.

## execution-order — for EXECUTE, one commit per row

1. `Placement` / `EmittedName` / `PlacementDecision` / `PlacementReason` types + the placement-indexed
   accessor; `hoisted()` becomes `typeset(Hoist)`. Corpus stays byte-identical (every decision is
   `Hoist`). Land the (a)-tier `xfail_until` pins here — they are writable the moment the type is.
2. `NameUseCensus` in `analysis`, built in the cli edge's frozen set and mirrored into `WhyWorld`.
3. `rul-a-loaded-definitions-placement-is-its-load-position` (§3): stop hoisting a definition the
   artifact already carries at its authored position. This is the correctness repair — it greens
   `emit30-hoisted-closure-outruns-its-load` and MOVES
   `emit30-single-stream-carries-the-closure-twice`'s golden (expected; that case exists to record
   the move). Re-bless BOTH, scoped, and check the porcelain.
4. The `EmittedNames` allocator (§4) + the injectivity cell, IF the conductor granted the
   `MUNGE_WITNESS_CASES` entry.
5. The §5 disclosure (reason enum on `ImportEdit` + `PlanImportRewritten`, `message: None`), then
   `dorc-loom publish`, then rebuild, then the tier e2e cases under option (c).
6. T1/T2 in `artifact::select` (§2), and only then the `sink` value (§6) for
   `p-x-placement-tuning-pair`.

Rung-0 pin on every commit: with no oracles loaded the whole thing is invisible and the output is
byte-identical (`empty-world-byte-identical`).

## deviations — every one OPEN, none self-endorsed

1. **`dev-no-xfail-e2e-per-tier`** — the brief asked for one XFAIL round-trip case per tier; the
   harness cannot express a layout-only target under the XFAIL lens (§7). I built the hole cells and
   the decline cell instead and specified three routes for the tier cells. Conductor rules which.
2. **`dev-injectivity-cell-not-committed`** — built, confirmed, deleted: it needs a governed
   `MUNGE_WITNESS_CASES` entry and that roster's own comment says adding one is a reviewed act. The
   full recipe is in §4 so nothing is lost.
3. **`dev-second-hole-cell-is-green-not-red`** — `emit30-single-stream-carries-the-closure-twice`
   pins the double-carry as a GREEN transcript rather than an XFAIL, because its run-set is
   accidentally correct today and there is no structural gate for "the artifact says this twice".
   Its golden is expected to move in step 3.
4. **`dev-name-use-census-is-new-analysis`** — §1 proposes a new `analysis` module rather than
   composing existing APIs. I could find no API answering variable READS by name
   (grepped `analysis/src/value.rs`; only resolution helpers, no reverse read-site index), and the
   tier-one condition cannot be stated without one.
5. **`dev-why-world-gains-a-selection`** — closing `dev-why-world-carries-no-import-edits` means
   `WhyWorld` settles a form it previously did not. Argued in §5 as forced by
   `one-definition-table-two-drivers`; it is nonetheless a widening of the why driver.

## tc-flags

- **`tc-t2-is-narrower-than-the-ladder-says`** — §2. `30Ng` §7's T2 is human-typed and, as written,
  needs alpha-rename for any bundle-bound name referenced from an authored body. Strawman and lean
  in §2. Winner-shifting: it decides whether a whole class of books gets the front-lift.
- **`tc-placement-spends-the-reserved-word`** — §1. `rul-emission-is-the-umbrella-name` RESERVES
  `placement`; this lane spends it on the component. I believe that is what reserving it was for,
  but it is a vocabulary act and cannot be taken back cheaply.
- **`tc-sink-changes-the-guard-shape`** — §6. Putting a body inside the guard's own parens changes
  the emitted guard shape, and `guard_shape_violations` is a floor that screams even under XFAIL.
  Whether `( <body>; <fn> <argv> ) || <original>` is inside `rul-ternary-verdict`'s "no
  engine-synthesized sh" is a licence question, not a layout one.

## proposed-steering-edits — conductor's to place, NOT edited by this lane

**`spike/crates/plan/CLAUDE.md`, appending to `pinned-definitions-are-the-artifact's-binding`:**

> ALREADY-IN-PLACE reads the ARTIFACT, not the book. `book_already_defines` answers about the book's
> own top-level bytes, and since the bundling (`30Ng:rul-bundle-at-dorc-lang-boundaries`) the
> artifact also carries a book-sited root's whole stripped text at the `.` — so a preamble that
> consults only the book emits a second copy of every definition that root declares
> (`30Qb:fnd-the-question-is-asked-in-four-places-not-three`, measured;
> `emit30-single-stream-carries-the-closure-twice` is the transcript). A definition's default
> placement follows how its source entered the unit: a `--pre-source` root is AMBIENT and hoisting
> is faithful, a source reached by a book `.` binds at that `.` and its placement is `in-place`
> unless the front-lift predicate licenses a hoist. Hoisting a book-loaded definition changes which
> binding is live at every book line it moved past — for VARIABLES as much as funcdefs, and the
> closure track carries file-level constants.

**`spike/crates/cli/CLAUDE.md`, appending to `artifact-forms-derive-from-one-structure`:**

> A book `.` carrying a LEADING ASSIGNMENT is not absorbable: `ImportEdit::Inline` replaces the whole
> command node, so `MODE=prod . ./entry.oracle.sh` would lose the assignment and the absorbed top
> level would read an environment the authored `.` never had. `Repoint` moves only the operand and
> stays eligible, so multipart is unaffected — the assignment costs the FORM, not the bundling.

**`FORFEITS.md`** — nothing proposed. Every gap here is emission/placement, which the header's
human-typed scope sharpening (2026-08-21) explicitly excludes: "emission/placement/artifact
limitations, harness gaps, and prose debt are never rows here."

**`ANALYZER-NEEDS.md`** — propose one row for `need-a-name-observation-census` (§1): the engine must
answer "does the book observe or mutate name N above position P", covering calls, `command -v`,
`type`, `unset`, `alias` and variable reads, and no existing API does.

## context-other-lanes-must-maintain

- **`lane-load-plane-precision`** shares `cli/src/main.rs` (§8) and owns `funcenv::analyze`'s
  construction at `~1060-1063`; this lane reads it and must not change it. It also owns
  `mech-acquire-and-ship-plain-sh`, whose `Selection` touch lands in `cli/src/artifact.rs` —
  `30O` predicts it is minimal. Where it is not, the collision is in `select`/`bundle_files`, and
  the placement lane's edits there are additive (a new input, a new tier branch), so a rebase should
  be mechanical.
- **Point-havoc changes the tier predicate's inputs.** `p-x-unknown-source-is-a-point-havoc` makes an
  unresolvable `.` havoc AT ITS LINE rather than poisoning the unit. Today's T1 condition (d) reads
  `CensusOpeners`, which is whole-unit; once havoc is positional, condition (d) should be
  position-scoped too — and that STRENGTHENS the ladder (a dynamism opener BELOW the `.` stops
  blocking a hoist above it). EXECUTE should not hardcode the whole-unit reading in a way that makes
  the positional version a rewrite.
- **`lane-influence-carriage`** touches every constructor of a stable object, `plan/src/lib.rs` and
  `cli/src/artifact.rs` included, and runs SERIAL over the merged tip. `PlacementDecision` is a new
  stable object and will need an influence account at its mint; minting it as a plain struct now is
  correct, and that lane converts it.

## stale-trigger-noticed-outside-this-lane

`p-x-load-operand-dirname-of-dollar-zero` and `p-x-load-operand-cd-pwd-of-dollar-zero` still cite the
open ruling `ask-dollar-zero-command-substitution-path` and say predicting `dirname`'s output "is the
tool-modelling `identity-declared-never-inferred` forbids". `30P:rul-static-predict-sites-loads`
[ACKED 2026-08-22] unparked exactly that in its static form. Text-only drift, in
`lane-load-plane-precision`'s pins — reported, not touched.

# §execute

> Tier: **LLM-authored, builder (Opus-class)**, lane `ai/r30-lane-planner-exec` from
> `ai/r30-lane-planner@0c38045d`. Implements §map as the conductor ruled it, plus the mid-lane rider
> `30P:rul-rewrite-permission-is-derived`. Grades: +SURE / ~SUSPECT / -GUESS / --WONDER.

## what-landed — nine commits

| commit | what |
|---|---|
| `8e9ab850` | `plan::placement`: the vocabulary (`Placement` · `EmittedName` · `PlacementReason` · `PlacementDecision` · `PlacedSources` · the `LoadSite`/`GuardSite` newtype pair · `DefinitionKey`). No consumer; corpus untouched. |
| `3f60b87b` | `mise run bless:case` — the scoped re-bless (see `fnd-bless-gates-on-a-green-suite`). |
| `33695701` | THE CORRECTNESS REPAIR: a definition inherits its SOURCE's placement, so nothing the artifact already carries at the author's `.` is hoisted a second time. Carries the rewrite-permission rider. Four goldens moved (`dev-four-goldens-moved-not-one`). |
| `715526ff` | `analysis::nameuse::NameUseCensus` — where the book first observes or mutates each name. |
| `f8c8cc9b` | `placement::EmittedNames`, the one mint for an emitted name, plus the squat witness case and its roster entry. |
| `35cb32b0` | `PlacementReason` on both `ImportEdit` variants and on the `PlanImportRewritten` payload. |
| `0941ed0a` | the regenerated catalogue lock (metadata only; `message` stays `None`). |
| `a8a58e57` | the two front-hoist tier pins, RED, plus the kept-in-place reason assertions. |
| `f9c5110e` | the kept-stream refusal, driven natively, asserting empty stdout (the `30Nh` harness gap). |

## as-built — the shape, in four sentences

**A definition inherits the placement of the FILE it was authored in.** `cli::artifact::select`
settles a `PlacedSources` beside its import edits — for every source a book `.` reaches, whether the
chosen form carries its bytes and where — and `Selection::emission()` hands the pair to
`Plan::decided` as ONE value (`ArtifactEmission`), so a producer cannot hold a carriage account from
one form beside an import list from another. `pin_definitions` files each closure declaration by its
`ClosureDecl` key's source and each role body by its vouch's `defining_span` file; a source no book
`.` reaches is AMBIENT and hoists as before, one a book `.` reaches is already carried and the
preamble adds nothing, and one nothing carries places nothing. `PinnedDefinitions::hoisted()` is
retired for `typeset(&Placement)`, so a form asks for the material AT a placement rather than
assuming one.

## deviations — every one OPEN

1. **`dev-four-goldens-moved-not-one`** — the brief pre-authorised ONE existing golden to move
   (`emit30-single-stream-carries-the-closure-twice`). Four did, all in the emit30 family, all the
   same twelve bytes: the duplicated guard preamble leaving the artifact. Enumerated:
   - `emit30-hoisted-closure-outruns-its-load` — XPASSED. Promoted: `XFAIL` and `head-expected.ran`
     deleted, `expected.out` blessed, header re-said from target-tense to landed. `expected.ran`
     unmoved (`hork stage unset`, which was the target all along).
   - `emit30-single-stream-carries-the-closure-twice` → RENAMED `…-carries-the-closure-once`, header
     rewritten. Its name asserted the defect; keeping it while the transcript shows one copy would
     have made the case's own identity a lie. Run-set unmoved.
   - `emit30-multipart-publishes-its-dependency` — the SAME repair, second witness: the bundle is
     published beside the plan, so the preamble's copy was the second one. Twelve lines gone, nothing
     else. Header's "puts the vouching body on the apply surface" re-said.
   - `emit30-assigned-load-is-not-absorbable` — the residual cell: `PreservedBookTree` carries no
     dependency, so nothing places the package's bytes and the preamble is empty. The GUARD survives
     and resolves through the book's own `.`. Header re-said; `MODE=prod . ./wombat.dorc.sh` still
     survives verbatim, which is what the case was minted for.
2. **`dev-residual-withholds-placement-not-the-vouch`** — ruling 1 says the residual (a source the
   artifact does not carry) WITHHOLDS "no hoist, no guard, no elide". Built as no PLACEMENT; the
   vouch is untouched. Reasoning, and it is a collision between two of the conductor's own rulings:
   ruling 9 mirrors the `Selection` into `WhyWorld` under `StreamPosture::TerminalRender`, which is
   the only total posture — but a run with `--artifact-dir` settles `Materializable`, so for a book
   whose `.` is outside the absorbable cell the two drivers settle DIFFERENT forms (multipart vs
   preserved-book-tree) and therefore different carriage. If carriage fed the vouch, the why driver
   would withhold guards the run granted — a decoration, which is exactly what
   `one-definition-table-two-drivers` forbids — and the only repair would be putting the run's
   posture in the durable, which ruling 9 forbids and `rul-durable-contents-reviewed-before-design`
   STOPS for. Placement not feeding licensing dissolves it. Two further reasons it is also right on
   its own: the PROBE ships oracle bytes whatever the apply form is, so an elision rests on a
   measurement that was actually made; and a guard over an uncarried source resolves through the
   author's own `.`, which is `rul-guard-resolves-like-its-mutation` exactly. ~SUSPECT the conductor
   re-rules this as built; +SURE the two rulings as written cannot both hold.
3. **`dev-hoist-action-not-built`** — step 6's T1/T2a are NOT built; the two pins are red and this is
   the sentence the brief asked for. The predicate's inputs all exist now (`NameUseCensus` ·
   `CensusOpeners` · the absorbable/explicit shape), and the missing half is not the decision but the
   ACTION: hoisting a bundle means the authored `.` line's own bytes must become something, and
   `30Ng` §7 says only "rewritten to its lifted resolution". In a single stream it cannot stay
   verbatim (`$0` is `sh`, and the target is not beside the plan), so the engine would substitute an
   authored line — a product-surface act bearing on `rul-attention-honesty`, the `two-surfaces` byte
   floor, and `KNOBS:kBACKFLIPS`'s enumerated edit classes. Strawman, single stream, T1:

   ```sh
   # the artifact, with the bundle lifted to the front and the author's own load line …
   :                      # … replaced by a null command? or
   . ./wombat.dorc.sh     # … left verbatim, and fatal, because nothing is beside the plan?
   ```

   Second half, cheap but real: a hoisted bundle re-opens the double-carry from the other side (its
   definitions would be pinned into the preamble AND stand in the lifted bundle text), so
   `PlacedSources::of_definition` has to answer "carried, wherever" rather than "carried in place" —
   a small reshape that belongs with the thing that needs it.
4. **`dev-a-tier-pins-not-in-the-type-commit`** — ruling 5 asked for the (a)-tier pins in the same
   commit as the type. They landed four commits later (`a8a58e57`), because they assert over
   `Selection`'s placement account and that account is what commit `33695701` builds. Nothing was
   lost; the ordering was the brief's, not a law.
5. **`dev-tier-needles-are-not-expressible`** — ruling 5's option (c) has the tier e2e cases declare
   `expect-diagnostic` needles for their own reason arm. `needles-are-structural` validates code
   SLUGS, and `28L:rul-reason-enums-not-sibling-codes` forbids a code per tier, so a reason arm is
   needle-able only once it RENDERS — and `plan-import-rewritten`'s register is `message: None`,
   which renders `[unwritten: plan-import-rewritten]` with no reason in it. Built the observable half
   natively instead (`a_kept_in_place_bundle_says_which_condition_kept_it`). The needle half unblocks
   when a human writes the prose; the hole is declared in the case's `when-fires`.
6. **`dev-two-mise-tasks-added`** — `bless:case` and `loom` (see `fnd-bless-gates-on-a-green-suite`
   and `fnd-loom-cli-resolves-the-wrong-rustc`). Both are tooling, not product.

## findings

- **`fnd-bless-gates-on-a-green-suite`** (+SURE, measured) — `mise run bless` opens with
  `gate:full-quiet` and refuses to write while ANYTHING is red. That is right for a conductor's close
  and a deadlock for the one job a builder needs it for: a change that legitimately moves a case's
  golden cannot bless that case, because that case IS the red one. Worse, the deadlock is
  self-widening — an unblessed golden makes `dorc-aid`'s `rendered_corpus_carries_no_minted_non_ascii`
  fail too, since a fixture line that no longer appears in the case's own inputs reads as
  engine-minted. `bless:case` is `BLESS=1` over the ordinary trial filter, with the scope check left
  to the caller's own porcelain.
- **`fnd-loom-cli-resolves-the-wrong-rustc`** (+SURE, measured) — a bare `cargo run -p dorc-loom`
  resolves rustc 1.95 and dies naming all fifteen crates before it reads a case; `dorc-loom publish`
  is the sanctioned prose-edit path and had no task. `mise run loom -- …` is that task.
- **`fnd-a-funcdef-is-not-a-plan-leaf`** (+SURE, measured while authoring the squat cell) — a
  top-level `NodeKind::FuncDef` takes no `LeafId`, so site numbering skips it. Recorded because two
  fixtures in this lane were authored against the wrong number first.
- **`fnd-book-set-roots-resolve-and-must-stay-explicit`** (+SURE, measured) — the rewrite-permission
  rider says "today every resolved operand is literal … so this is vacuous now". It is not:
  `pin28-variable-resolved-source-loads` re-points `. "./$PKG.oracle.sh"` with `PKG=foobar`, and
  `load30-rooted-shared-dependency` and two siblings are the same shape. A strict literal-only
  predicate reddened four corpus cases. `30P:rul-rewrite-permission-is-derived`'s own definition
  admits them ("a literal word, **or a literal-assigned book-set root**") while its per-form gloss
  says "re-points LITERAL dorc-lang imports"; the definition is what is built. ~SUSPECT the gloss
  wants the same words as the definition.

## tc-flags

- **`tc-uncarried-source-still-guards`** — `dev-residual-withholds-placement-not-the-vouch`, restated
  as the judgment call it is. Product surface: a book whose `.` is outside the absorbable cell, run
  without `--artifact-dir`, so nothing is placed beside the plan.

  ```sh
  false || . ./wombat.dorc.sh      # the artifact carries no copy of this package
  hork provision                   # unmodeled: the site below guards rather than elides
  wombat sync a.conf               # ( wombat__is_converged sync a.conf ) || wombat sync a.conf
  ```

  As built, that guard ships and resolves through the author's own `.` on the target. Under ruling
  1's letter it would not ship and the mutator would run. My lean: as built — the guard is a live
  re-check in the environment its own mutation runs in, and the alternative makes the why driver and
  the run disagree.
- **`tc-sink-not-attempted`** — `p-x-placement-tuning-pair` stays red. `tc-sink-changes-the-guard-shape`
  was ruled admissible and step 6 was ordered last; the budget went to the ladder question above it,
  which turned out to be the blocking one. Untouched, unprejudiced.

## proposed-steering-edits — conductor's to place

**`spike/crates/plan/CLAUDE.md`, appending to `pinned-definitions-are-the-artifact's-binding`:**

> A definition's placement is INHERITED from the source it was authored in, never assumed
> (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`). `Plan::decided` demands an
> `ArtifactEmission` — the settled form's carriage account plus its import edits, as ONE value — and
> `pin_definitions` files each declaration by its `ClosureDecl` key's source and each body by its
> vouch's defining file. A `--pre-source` root is AMBIENT and hoisting it is faithful; a source a
> book `.` reaches is already carried at that `.` since the bundling
> (`30Ng:rul-bundle-at-dorc-lang-boundaries`), so the preamble adds nothing and a hoist would bind
> names at lines the authored program does not — for VARIABLES as much as funcdefs, since the
> closure track carries file-level constants. A source the form carries NOWHERE places nothing; the
> guard, if any, resolves through the author's own `.`, which is
> `30P:rul-guard-resolves-like-its-mutation` exactly. Every emitted name is minted through
> `placement::EmittedNames`, which holds every name the book names and lengthens the digest until
> the candidate is free — `<name>_h<digest>` is computable from the artifact, so it is squattable in
> plain sight rather than colliding by accident.

**`spike/crates/cli/CLAUDE.md`, appending to `artifact-forms-derive-from-one-structure`:**

> A book `.` carrying a LEADING ASSIGNMENT is not absorbable: `ImportEdit::Inline` replaces the whole
> command node, so `MODE=prod . ./entry.oracle.sh` would lose the assignment and the absorbed top
> level would read an environment the authored `.` never had. `Repoint` moves only the operand and
> stays eligible, so multipart is unaffected — the assignment costs the FORM, not the bundling.
> Separately, permission to rewrite a `.` line at all is DERIVED from the operand naming its target
> EXPLICITLY (`30P:rul-rewrite-permission-is-derived`): `artifact::operand_is_explicit` is the one
> seat, no tier re-points, inlines or hoists a line it answers `false` for, and multipart mirrors
> such a target at its authored relative path instead so the author's own operand finds it. EXACT is
> a different axis and grants nothing here — it governs AUTHORITY, explicitness governs REWRITING.
> `Selection::emission()` is what reaches the plan: the carriage account and the import edits as one
> value, because they are one answer about one form.

**`spike/CLAUDE.md`, the task list:** `mise run bless:case -- <case>` (the scoped re-bless; `bless`
verifies the whole suite first and cannot write while the case it would fix is the red one) and
`mise run loom -- <args>` (the loom CLI on this workspace's toolchain).

**`ANALYZER-NEEDS.md`**, one row:

> | an-name-observation-census | for a NAME and a POSITION: does the book observe or mutate that name above that line — calls, `command -v`/`type`, `unset`/`unalias`, `alias`, `export`/`readonly`, funcdefs, loop variables, and every parameter-expansion read. ⊤-biased: a non-literal command word, an operator-bearing expansion whose name the lexer discarded, and an `eval` are each a use of EVERY name. Built as `analysis::nameuse::NameUseCensus` (one AST walk, no fixpoint); the ⊤ arm narrows mechanically when the load lane decodes `ParamComplex` | emission planner (hoist legality); emitted-name freshness | an-static-load-occurrence-account | 30P, 30Qb | B |

**`FORFEITS.md`** — nothing proposed; §map's reading holds (emission/placement limitations are
excluded by the header's human-typed scope sharpening).

## context-other-lanes-must-maintain

- **`lane-load-plane-precision`** — `cli/src/artifact.rs`'s `operand_is_explicit` is the seat the
  typed `Explicitness` marker replaces at the fold; its doc-comment says so in one line. It reads
  the AST word today and refuses `ParamComplex`, `CommandSubst`, `Arithmetic` and `$0`, while
  ACCEPTING a simple `$name` (see `fnd-book-set-roots-resolve-and-must-stay-explicit`). Also:
  `NameUseCensus`'s ⊤ arm has one seat (`uses_everything`), and the `ParamComplex` decoding narrows
  it there and nowhere else. `main.rs:~1060-1063` untouched, as briefed.
- **`lane-influence-carriage`** — `PlacementDecision` and `PlacedSources` are new stable objects,
  minted plain, ready to convert. `ArtifactEmission` is a borrow-only carrier of the two and is not
  itself an object.
- **Point-havoc** — T1's condition (d) is unbuilt, so nothing hardcodes the whole-unit reading of
  `CensusOpeners`. When the hoist action lands it must be written position-scoped from the start
  (conductor ruling 11).
- **The four moved goldens** are all the same twelve lines. Any lane that re-blesses an emit30 case
  should confirm the preamble is absent by design rather than by drift.
